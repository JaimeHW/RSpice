#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_232(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign62260_e96659, assign62260_e96659_d_n0, assign62260_e96659_d_n2, assign62260_e96659_d_n4, assign62260_e96659_d_n5, assign62260_e96659_d_n6, assign62260_e96659_d_n7, assign62260_e96659_d_n8, assign62260_e96659_d_n9, assign62260_e96659_d_n10, assign62260_e96659_d_n11, assign62260_e96659_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62260_e96657: f64 = (1.0 / locals.var_t4);
        (assign62260_e96657, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign62260_e96659;
        locals.var_t5_dn0 = assign62260_e96659_d_n0;
        locals.var_t5_dn2 = assign62260_e96659_d_n2;
        locals.var_t5_dn4 = assign62260_e96659_d_n4;
        locals.var_t5_dn5 = assign62260_e96659_d_n5;
        locals.var_t5_dn6 = assign62260_e96659_d_n6;
        locals.var_t5_dn7 = assign62260_e96659_d_n7;
        locals.var_t5_dn8 = assign62260_e96659_d_n8;
        locals.var_t5_dn9 = assign62260_e96659_d_n9;
        locals.var_t5_dn10 = assign62260_e96659_d_n10;
        locals.var_t5_dn11 = assign62260_e96659_d_n11;
        locals.var_t5_dn14 = assign62260_e96659_d_n14;
        locals.var_t5_rv = 0.0;

        let assign62270_e96663: f64 = (10.0 * 2.220446049250313e-16);
        let assign62270_e96664: f64 = (2.0 - assign62270_e96663);
        let assign62270_e96671: f64 = (10.0 * 2.220446049250313e-16);
        let assign62270_e96672: f64 = (2.0 + assign62270_e96671);
        let assign62270_e96674: f64 = if ((assign62270_e96664 <= p.p178) && (p.p178 <= assign62270_e96672)) { 1.0 } else { 0.0 };
        locals.var_guard1498 = assign62270_e96674;
        locals.var_guard1498_rv = 0.0;

        let (assign62280_e96689, assign62280_e96689_d_n0, assign62280_e96689_d_n2, assign62280_e96689_d_n4, assign62280_e96689_d_n5, assign62280_e96689_d_n6, assign62280_e96689_d_n7, assign62280_e96689_d_n8, assign62280_e96689_d_n9, assign62280_e96689_d_n10, assign62280_e96689_d_n11, assign62280_e96689_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1497 == 0.0)) && (locals.var_guard1498 != 0.0)) {
        let assign62280_e96686: f64 = (locals.var_t4).sqrt();
        let assign62280_e96687: f64 = (1.0 / assign62280_e96686);
        (assign62280_e96687, (-((locals.var_t4_dn0 / (2.0 * assign62280_e96686)) / (assign62280_e96686 * assign62280_e96686))), (-((locals.var_t4_dn2 / (2.0 * assign62280_e96686)) / (assign62280_e96686 * assign62280_e96686))), (-((locals.var_t4_dn4 / (2.0 * assign62280_e96686)) / (assign62280_e96686 * assign62280_e96686))), (-((locals.var_t4_dn5 / (2.0 * assign62280_e96686)) / (assign62280_e96686 * assign62280_e96686))), (-((locals.var_t4_dn6 / (2.0 * assign62280_e96686)) / (assign62280_e96686 * assign62280_e96686))), (-((locals.var_t4_dn7 / (2.0 * assign62280_e96686)) / (assign62280_e96686 * assign62280_e96686))), (-((locals.var_t4_dn8 / (2.0 * assign62280_e96686)) / (assign62280_e96686 * assign62280_e96686))), (-((locals.var_t4_dn9 / (2.0 * assign62280_e96686)) / (assign62280_e96686 * assign62280_e96686))), (-((locals.var_t4_dn10 / (2.0 * assign62280_e96686)) / (assign62280_e96686 * assign62280_e96686))), (-((locals.var_t4_dn11 / (2.0 * assign62280_e96686)) / (assign62280_e96686 * assign62280_e96686))), (-((locals.var_t4_dn14 / (2.0 * assign62280_e96686)) / (assign62280_e96686 * assign62280_e96686))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign62280_e96689;
        locals.var_t5_dn0 = assign62280_e96689_d_n0;
        locals.var_t5_dn2 = assign62280_e96689_d_n2;
        locals.var_t5_dn4 = assign62280_e96689_d_n4;
        locals.var_t5_dn5 = assign62280_e96689_d_n5;
        locals.var_t5_dn6 = assign62280_e96689_d_n6;
        locals.var_t5_dn7 = assign62280_e96689_d_n7;
        locals.var_t5_dn8 = assign62280_e96689_d_n8;
        locals.var_t5_dn9 = assign62280_e96689_d_n9;
        locals.var_t5_dn10 = assign62280_e96689_d_n10;
        locals.var_t5_dn11 = assign62280_e96689_d_n11;
        locals.var_t5_dn14 = assign62280_e96689_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign62290_e96714, assign62290_e96714_d_n0, assign62290_e96714_d_n2, assign62290_e96714_d_n4, assign62290_e96714_d_n5, assign62290_e96714_d_n6, assign62290_e96714_d_n7, assign62290_e96714_d_n8, assign62290_e96714_d_n9, assign62290_e96714_d_n10, assign62290_e96714_d_n11, assign62290_e96714_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1497 == 0.0)) && (locals.var_guard1498 == 0.0)) {
        let (assign62290_e96712, assign62290_e96712_d_n0, assign62290_e96712_d_n2, assign62290_e96712_d_n4, assign62290_e96712_d_n5, assign62290_e96712_d_n6, assign62290_e96712_d_n7, assign62290_e96712_d_n8, assign62290_e96712_d_n9, assign62290_e96712_d_n10, assign62290_e96712_d_n11, assign62290_e96712_d_n14,) = {
            if (locals.var_t4 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign62290_e96706: f64 = (-1.0);
                let assign62290_e96708: f64 = (assign62290_e96706 / p.p178);
                let assign62290_e96710: f64 = (assign62290_e96708 - 1.0);
                let assign62290_e96711: f64 = (locals.var_t4).powf(assign62290_e96710);
                (assign62290_e96711, if 0.0 == 0.0 && ((assign62290_e96710) as f64).is_finite() && ((assign62290_e96710) as f64).fract() == 0.0 { if assign62290_e96710 == 0.0 { 0.0 } else { (assign62290_e96710 * ((locals.var_t4).powf(assign62290_e96710 - 1.0) * locals.var_t4_dn0)) } } else { (assign62290_e96711 * (assign62290_e96710 * (locals.var_t4_dn0 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62290_e96710) as f64).is_finite() && ((assign62290_e96710) as f64).fract() == 0.0 { if assign62290_e96710 == 0.0 { 0.0 } else { (assign62290_e96710 * ((locals.var_t4).powf(assign62290_e96710 - 1.0) * locals.var_t4_dn2)) } } else { (assign62290_e96711 * (assign62290_e96710 * (locals.var_t4_dn2 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62290_e96710) as f64).is_finite() && ((assign62290_e96710) as f64).fract() == 0.0 { if assign62290_e96710 == 0.0 { 0.0 } else { (assign62290_e96710 * ((locals.var_t4).powf(assign62290_e96710 - 1.0) * locals.var_t4_dn4)) } } else { (assign62290_e96711 * (assign62290_e96710 * (locals.var_t4_dn4 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62290_e96710) as f64).is_finite() && ((assign62290_e96710) as f64).fract() == 0.0 { if assign62290_e96710 == 0.0 { 0.0 } else { (assign62290_e96710 * ((locals.var_t4).powf(assign62290_e96710 - 1.0) * locals.var_t4_dn5)) } } else { (assign62290_e96711 * (assign62290_e96710 * (locals.var_t4_dn5 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62290_e96710) as f64).is_finite() && ((assign62290_e96710) as f64).fract() == 0.0 { if assign62290_e96710 == 0.0 { 0.0 } else { (assign62290_e96710 * ((locals.var_t4).powf(assign62290_e96710 - 1.0) * locals.var_t4_dn6)) } } else { (assign62290_e96711 * (assign62290_e96710 * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62290_e96710) as f64).is_finite() && ((assign62290_e96710) as f64).fract() == 0.0 { if assign62290_e96710 == 0.0 { 0.0 } else { (assign62290_e96710 * ((locals.var_t4).powf(assign62290_e96710 - 1.0) * locals.var_t4_dn7)) } } else { (assign62290_e96711 * (assign62290_e96710 * (locals.var_t4_dn7 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62290_e96710) as f64).is_finite() && ((assign62290_e96710) as f64).fract() == 0.0 { if assign62290_e96710 == 0.0 { 0.0 } else { (assign62290_e96710 * ((locals.var_t4).powf(assign62290_e96710 - 1.0) * locals.var_t4_dn8)) } } else { (assign62290_e96711 * (assign62290_e96710 * (locals.var_t4_dn8 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62290_e96710) as f64).is_finite() && ((assign62290_e96710) as f64).fract() == 0.0 { if assign62290_e96710 == 0.0 { 0.0 } else { (assign62290_e96710 * ((locals.var_t4).powf(assign62290_e96710 - 1.0) * locals.var_t4_dn9)) } } else { (assign62290_e96711 * (assign62290_e96710 * (locals.var_t4_dn9 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62290_e96710) as f64).is_finite() && ((assign62290_e96710) as f64).fract() == 0.0 { if assign62290_e96710 == 0.0 { 0.0 } else { (assign62290_e96710 * ((locals.var_t4).powf(assign62290_e96710 - 1.0) * locals.var_t4_dn10)) } } else { (assign62290_e96711 * (assign62290_e96710 * (locals.var_t4_dn10 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62290_e96710) as f64).is_finite() && ((assign62290_e96710) as f64).fract() == 0.0 { if assign62290_e96710 == 0.0 { 0.0 } else { (assign62290_e96710 * ((locals.var_t4).powf(assign62290_e96710 - 1.0) * locals.var_t4_dn11)) } } else { (assign62290_e96711 * (assign62290_e96710 * (locals.var_t4_dn11 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62290_e96710) as f64).is_finite() && ((assign62290_e96710) as f64).fract() == 0.0 { if assign62290_e96710 == 0.0 { 0.0 } else { (assign62290_e96710 * ((locals.var_t4).powf(assign62290_e96710 - 1.0) * locals.var_t4_dn14)) } } else { (assign62290_e96711 * (assign62290_e96710 * (locals.var_t4_dn14 / locals.var_t4))) },)
            }
        };
        (assign62290_e96712, assign62290_e96712_d_n0, assign62290_e96712_d_n2, assign62290_e96712_d_n4, assign62290_e96712_d_n5, assign62290_e96712_d_n6, assign62290_e96712_d_n7, assign62290_e96712_d_n8, assign62290_e96712_d_n9, assign62290_e96712_d_n10, assign62290_e96712_d_n11, assign62290_e96712_d_n14,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign62290_e96714;
        locals.var_t6_dn0 = assign62290_e96714_d_n0;
        locals.var_t6_dn2 = assign62290_e96714_d_n2;
        locals.var_t6_dn4 = assign62290_e96714_d_n4;
        locals.var_t6_dn5 = assign62290_e96714_d_n5;
        locals.var_t6_dn6 = assign62290_e96714_d_n6;
        locals.var_t6_dn7 = assign62290_e96714_d_n7;
        locals.var_t6_dn8 = assign62290_e96714_d_n8;
        locals.var_t6_dn9 = assign62290_e96714_d_n9;
        locals.var_t6_dn10 = assign62290_e96714_d_n10;
        locals.var_t6_dn11 = assign62290_e96714_d_n11;
        locals.var_t6_dn14 = assign62290_e96714_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign62300_e96729, assign62300_e96729_d_n0, assign62300_e96729_d_n2, assign62300_e96729_d_n4, assign62300_e96729_d_n5, assign62300_e96729_d_n6, assign62300_e96729_d_n7, assign62300_e96729_d_n8, assign62300_e96729_d_n9, assign62300_e96729_d_n10, assign62300_e96729_d_n11, assign62300_e96729_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1497 == 0.0)) && (locals.var_guard1498 == 0.0)) {
        let assign62300_e96727: f64 = (locals.var_t4 * locals.var_t6);
        (assign62300_e96727, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn4 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn4)), ((locals.var_t4_dn5 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn5)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn7 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn7)), ((locals.var_t4_dn8 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn8)), ((locals.var_t4_dn9 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn9)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn11 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn11)), ((locals.var_t4_dn14 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign62300_e96729;
        locals.var_t5_dn0 = assign62300_e96729_d_n0;
        locals.var_t5_dn2 = assign62300_e96729_d_n2;
        locals.var_t5_dn4 = assign62300_e96729_d_n4;
        locals.var_t5_dn5 = assign62300_e96729_d_n5;
        locals.var_t5_dn6 = assign62300_e96729_d_n6;
        locals.var_t5_dn7 = assign62300_e96729_d_n7;
        locals.var_t5_dn8 = assign62300_e96729_d_n8;
        locals.var_t5_dn9 = assign62300_e96729_d_n9;
        locals.var_t5_dn10 = assign62300_e96729_d_n10;
        locals.var_t5_dn11 = assign62300_e96729_d_n11;
        locals.var_t5_dn14 = assign62300_e96729_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign62310_e96738, assign62310_e96738_d_n0, assign62310_e96738_d_n2, assign62310_e96738_d_n4, assign62310_e96738_d_n5, assign62310_e96738_d_n6, assign62310_e96738_d_n7, assign62310_e96738_d_n8, assign62310_e96738_d_n9, assign62310_e96738_d_n10, assign62310_e96738_d_n11, assign62310_e96738_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign62310_e96736: f64 = (locals.var_muun * locals.var_t5);
        (assign62310_e96736, ((locals.var_muun_dn0 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn0)), ((locals.var_muun_dn2 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn2)), ((locals.var_muun_dn4 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn4)), ((locals.var_muun_dn5 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn5)), ((locals.var_muun_dn6 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn6)), ((locals.var_muun_dn7 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn7)), ((locals.var_muun_dn8 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn8)), ((locals.var_muun_dn9 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn9)), ((locals.var_muun_dn10 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn10)), ((locals.var_muun_dn11 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn11)), ((locals.var_muun_dn14 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn14)),)
    } else {
        (locals.var_mu, locals.var_mu_dn0, locals.var_mu_dn2, locals.var_mu_dn4, locals.var_mu_dn5, locals.var_mu_dn6, locals.var_mu_dn7, locals.var_mu_dn8, locals.var_mu_dn9, locals.var_mu_dn10, locals.var_mu_dn11, locals.var_mu_dn14,)
    }
};
        locals.var_mu = assign62310_e96738;
        locals.var_mu_dn0 = assign62310_e96738_d_n0;
        locals.var_mu_dn2 = assign62310_e96738_d_n2;
        locals.var_mu_dn4 = assign62310_e96738_d_n4;
        locals.var_mu_dn5 = assign62310_e96738_d_n5;
        locals.var_mu_dn6 = assign62310_e96738_d_n6;
        locals.var_mu_dn7 = assign62310_e96738_d_n7;
        locals.var_mu_dn8 = assign62310_e96738_d_n8;
        locals.var_mu_dn9 = assign62310_e96738_d_n9;
        locals.var_mu_dn10 = assign62310_e96738_d_n10;
        locals.var_mu_dn11 = assign62310_e96738_d_n11;
        locals.var_mu_dn14 = assign62310_e96738_d_n14;
        locals.var_mu_rv = 0.0;

        let (assign62320_e96749, assign62320_e96749_d_n0, assign62320_e96749_d_n2, assign62320_e96749_d_n4, assign62320_e96749_d_n5, assign62320_e96749_d_n6, assign62320_e96749_d_n7, assign62320_e96749_d_n8, assign62320_e96749_d_n9, assign62320_e96749_d_n10, assign62320_e96749_d_n11, assign62320_e96749_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign62320_e96745: f64 = (locals.var_weff_nf * locals.var_beta_inv);
        let assign62320_e96747: f64 = (assign62320_e96745 / locals.var_lch);
        (assign62320_e96747, ((((locals.var_weff_nf * locals.var_beta_inv_dn0) * locals.var_lch) - (assign62320_e96745 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn2) * locals.var_lch) - (assign62320_e96745 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn4) * locals.var_lch) - (assign62320_e96745 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn5) * locals.var_lch) - (assign62320_e96745 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn6) * locals.var_lch) - (assign62320_e96745 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn7) * locals.var_lch) - (assign62320_e96745 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn8) * locals.var_lch) - (assign62320_e96745 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn9) * locals.var_lch) - (assign62320_e96745 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn10) * locals.var_lch) - (assign62320_e96745 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn11) * locals.var_lch) - (assign62320_e96745 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn14) * locals.var_lch) - (assign62320_e96745 * locals.var_lch_dn14)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_betawl, locals.var_betawl_dn0, locals.var_betawl_dn2, locals.var_betawl_dn4, locals.var_betawl_dn5, locals.var_betawl_dn6, locals.var_betawl_dn7, locals.var_betawl_dn8, locals.var_betawl_dn9, locals.var_betawl_dn10, locals.var_betawl_dn11, locals.var_betawl_dn14,)
    }
};
        locals.var_betawl = assign62320_e96749;
        locals.var_betawl_dn0 = assign62320_e96749_d_n0;
        locals.var_betawl_dn2 = assign62320_e96749_d_n2;
        locals.var_betawl_dn4 = assign62320_e96749_d_n4;
        locals.var_betawl_dn5 = assign62320_e96749_d_n5;
        locals.var_betawl_dn6 = assign62320_e96749_d_n6;
        locals.var_betawl_dn7 = assign62320_e96749_d_n7;
        locals.var_betawl_dn8 = assign62320_e96749_d_n8;
        locals.var_betawl_dn9 = assign62320_e96749_d_n9;
        locals.var_betawl_dn10 = assign62320_e96749_d_n10;
        locals.var_betawl_dn11 = assign62320_e96749_d_n11;
        locals.var_betawl_dn14 = assign62320_e96749_d_n14;
        locals.var_betawl_rv = 0.0;

        let (assign62330_e96759, assign62330_e96759_d_n0, assign62330_e96759_d_n2, assign62330_e96759_d_n4, assign62330_e96759_d_n5, assign62330_e96759_d_n6, assign62330_e96759_d_n7, assign62330_e96759_d_n8, assign62330_e96759_d_n9, assign62330_e96759_d_n10, assign62330_e96759_d_n11, assign62330_e96759_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign62330_e96755: f64 = (-locals.var_betawl);
        let assign62330_e96757: f64 = (assign62330_e96755 / locals.var_lch);
        (assign62330_e96757, ((((-locals.var_betawl_dn0) * locals.var_lch) - (assign62330_e96755 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn2) * locals.var_lch) - (assign62330_e96755 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn4) * locals.var_lch) - (assign62330_e96755 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn5) * locals.var_lch) - (assign62330_e96755 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn6) * locals.var_lch) - (assign62330_e96755 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn7) * locals.var_lch) - (assign62330_e96755 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn8) * locals.var_lch) - (assign62330_e96755 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn9) * locals.var_lch) - (assign62330_e96755 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn10) * locals.var_lch) - (assign62330_e96755 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn11) * locals.var_lch) - (assign62330_e96755 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn14) * locals.var_lch) - (assign62330_e96755 * locals.var_lch_dn14)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign62330_e96759;
        locals.var_t1_dn0 = assign62330_e96759_d_n0;
        locals.var_t1_dn2 = assign62330_e96759_d_n2;
        locals.var_t1_dn4 = assign62330_e96759_d_n4;
        locals.var_t1_dn5 = assign62330_e96759_d_n5;
        locals.var_t1_dn6 = assign62330_e96759_d_n6;
        locals.var_t1_dn7 = assign62330_e96759_d_n7;
        locals.var_t1_dn8 = assign62330_e96759_d_n8;
        locals.var_t1_dn9 = assign62330_e96759_d_n9;
        locals.var_t1_dn10 = assign62330_e96759_d_n10;
        locals.var_t1_dn11 = assign62330_e96759_d_n11;
        locals.var_t1_dn14 = assign62330_e96759_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign62340_e96770, assign62340_e96770_d_n0, assign62340_e96770_d_n2, assign62340_e96770_d_n4, assign62340_e96770_d_n5, assign62340_e96770_d_n6, assign62340_e96770_d_n7, assign62340_e96770_d_n8, assign62340_e96770_d_n9, assign62340_e96770_d_n10, assign62340_e96770_d_n11, assign62340_e96770_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign62340_e96766: f64 = (locals.var_betawl * locals.var_idd);
        let assign62340_e96768: f64 = (assign62340_e96766 * locals.var_mu);
        (assign62340_e96768, ((((locals.var_betawl_dn0 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn0)) * locals.var_mu) + (assign62340_e96766 * locals.var_mu_dn0)), ((((locals.var_betawl_dn2 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn2)) * locals.var_mu) + (assign62340_e96766 * locals.var_mu_dn2)), ((((locals.var_betawl_dn4 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn4)) * locals.var_mu) + (assign62340_e96766 * locals.var_mu_dn4)), ((((locals.var_betawl_dn5 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn5)) * locals.var_mu) + (assign62340_e96766 * locals.var_mu_dn5)), ((((locals.var_betawl_dn6 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn6)) * locals.var_mu) + (assign62340_e96766 * locals.var_mu_dn6)), ((((locals.var_betawl_dn7 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn7)) * locals.var_mu) + (assign62340_e96766 * locals.var_mu_dn7)), ((((locals.var_betawl_dn8 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn8)) * locals.var_mu) + (assign62340_e96766 * locals.var_mu_dn8)), ((((locals.var_betawl_dn9 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn9)) * locals.var_mu) + (assign62340_e96766 * locals.var_mu_dn9)), ((((locals.var_betawl_dn10 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn10)) * locals.var_mu) + (assign62340_e96766 * locals.var_mu_dn10)), ((((locals.var_betawl_dn11 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn11)) * locals.var_mu) + (assign62340_e96766 * locals.var_mu_dn11)), ((((locals.var_betawl_dn14 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn14)) * locals.var_mu) + (assign62340_e96766 * locals.var_mu_dn14)),)
    } else {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn4, locals.var_ids0_dn5, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn8, locals.var_ids0_dn9, locals.var_ids0_dn10, locals.var_ids0_dn11, locals.var_ids0_dn14,)
    }
};
        locals.var_ids0 = assign62340_e96770;
        locals.var_ids0_dn0 = assign62340_e96770_d_n0;
        locals.var_ids0_dn2 = assign62340_e96770_d_n2;
        locals.var_ids0_dn4 = assign62340_e96770_d_n4;
        locals.var_ids0_dn5 = assign62340_e96770_d_n5;
        locals.var_ids0_dn6 = assign62340_e96770_d_n6;
        locals.var_ids0_dn7 = assign62340_e96770_d_n7;
        locals.var_ids0_dn8 = assign62340_e96770_d_n8;
        locals.var_ids0_dn9 = assign62340_e96770_d_n9;
        locals.var_ids0_dn10 = assign62340_e96770_d_n10;
        locals.var_ids0_dn11 = assign62340_e96770_d_n11;
        locals.var_ids0_dn14 = assign62340_e96770_d_n14;
        locals.var_ids0_rv = 0.0;

        let assign62350_e96773: f64 = if p.p283 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1499 = assign62350_e96773;
        locals.var_guard1499_rv = 0.0;

        let (assign62360_e96786, assign62360_e96786_d_n0, assign62360_e96786_d_n2, assign62360_e96786_d_n4, assign62360_e96786_d_n5, assign62360_e96786_d_n6, assign62360_e96786_d_n7, assign62360_e96786_d_n8, assign62360_e96786_d_n9, assign62360_e96786_d_n10, assign62360_e96786_d_n11, assign62360_e96786_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62360_e96783: f64 = (locals.var_vds - locals.var_pds);
        let assign62360_e96784: f64 = (0.5 * assign62360_e96783);
        (assign62360_e96784, (0.5 * (locals.var_vds_dn0 - locals.var_pds_dn0)), (0.5 * (locals.var_vds_dn2 - locals.var_pds_dn2)), (0.5 * (locals.var_vds_dn4 - locals.var_pds_dn4)), (0.5 * (locals.var_vds_dn5 - locals.var_pds_dn5)), (0.5 * (locals.var_vds_dn6 - locals.var_pds_dn6)), (0.5 * (locals.var_vds_dn7 - locals.var_pds_dn7)), (0.5 * (locals.var_vds_dn8 - locals.var_pds_dn8)), (0.5 * (locals.var_vds_dn9 - locals.var_pds_dn9)), (0.5 * (locals.var_vds_dn10 - locals.var_pds_dn10)), (0.5 * (locals.var_vds_dn11 - locals.var_pds_dn11)), (0.5 * (locals.var_vds_dn14 - locals.var_pds_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign62360_e96786;
        locals.var_t1_dn0 = assign62360_e96786_d_n0;
        locals.var_t1_dn2 = assign62360_e96786_d_n2;
        locals.var_t1_dn4 = assign62360_e96786_d_n4;
        locals.var_t1_dn5 = assign62360_e96786_d_n5;
        locals.var_t1_dn6 = assign62360_e96786_d_n6;
        locals.var_t1_dn7 = assign62360_e96786_d_n7;
        locals.var_t1_dn8 = assign62360_e96786_d_n8;
        locals.var_t1_dn9 = assign62360_e96786_d_n9;
        locals.var_t1_dn10 = assign62360_e96786_d_n10;
        locals.var_t1_dn11 = assign62360_e96786_d_n11;
        locals.var_t1_dn14 = assign62360_e96786_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign62370_e96799, assign62370_e96799_d_n0, assign62370_e96799_d_n2, assign62370_e96799_d_n4, assign62370_e96799_d_n5, assign62370_e96799_d_n6, assign62370_e96799_d_n7, assign62370_e96799_d_n8, assign62370_e96799_d_n9, assign62370_e96799_d_n10, assign62370_e96799_d_n11, assign62370_e96799_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62370_e96795: f64 = (2.0 * locals.var_t1);
        let assign62370_e96797: f64 = (assign62370_e96795 / 0.01);
        (assign62370_e96797, ((2.0 * locals.var_t1_dn0) / 0.01), ((2.0 * locals.var_t1_dn2) / 0.01), ((2.0 * locals.var_t1_dn4) / 0.01), ((2.0 * locals.var_t1_dn5) / 0.01), ((2.0 * locals.var_t1_dn6) / 0.01), ((2.0 * locals.var_t1_dn7) / 0.01), ((2.0 * locals.var_t1_dn8) / 0.01), ((2.0 * locals.var_t1_dn9) / 0.01), ((2.0 * locals.var_t1_dn10) / 0.01), ((2.0 * locals.var_t1_dn11) / 0.01), ((2.0 * locals.var_t1_dn14) / 0.01),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign62370_e96799;
        locals.var_tmf1_dn0 = assign62370_e96799_d_n0;
        locals.var_tmf1_dn2 = assign62370_e96799_d_n2;
        locals.var_tmf1_dn4 = assign62370_e96799_d_n4;
        locals.var_tmf1_dn5 = assign62370_e96799_d_n5;
        locals.var_tmf1_dn6 = assign62370_e96799_d_n6;
        locals.var_tmf1_dn7 = assign62370_e96799_d_n7;
        locals.var_tmf1_dn8 = assign62370_e96799_d_n8;
        locals.var_tmf1_dn9 = assign62370_e96799_d_n9;
        locals.var_tmf1_dn10 = assign62370_e96799_d_n10;
        locals.var_tmf1_dn11 = assign62370_e96799_d_n11;
        locals.var_tmf1_dn14 = assign62370_e96799_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign62380_e96844, assign62380_e96844_d_n0, assign62380_e96844_d_n2, assign62380_e96844_d_n4, assign62380_e96844_d_n5, assign62380_e96844_d_n6, assign62380_e96844_d_n7, assign62380_e96844_d_n8, assign62380_e96844_d_n9, assign62380_e96844_d_n10, assign62380_e96844_d_n11, assign62380_e96844_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62380_e96810: f64 = (1.0 / 2.0);
        let assign62380_e96814: f64 = (1.0 / 6.0);
        let assign62380_e96818: f64 = (1.0 / 24.0);
        let assign62380_e96822: f64 = (1.0 / 120.0);
        let assign62380_e96826: f64 = (1.0 / 720.0);
        let assign62380_e96830: f64 = (1.0 / 5040.0);
        let assign62380_e96831: f64 = (locals.var_tmf1 * assign62380_e96830);
        let assign62380_e96832: f64 = (assign62380_e96826 + assign62380_e96831);
        let assign62380_e96833: f64 = (locals.var_tmf1 * assign62380_e96832);
        let assign62380_e96834: f64 = (assign62380_e96822 + assign62380_e96833);
        let assign62380_e96835: f64 = (locals.var_tmf1 * assign62380_e96834);
        let assign62380_e96836: f64 = (assign62380_e96818 + assign62380_e96835);
        let assign62380_e96837: f64 = (locals.var_tmf1 * assign62380_e96836);
        let assign62380_e96838: f64 = (assign62380_e96814 + assign62380_e96837);
        let assign62380_e96839: f64 = (locals.var_tmf1 * assign62380_e96838);
        let assign62380_e96840: f64 = (assign62380_e96810 + assign62380_e96839);
        let assign62380_e96841: f64 = (locals.var_tmf1 * assign62380_e96840);
        let assign62380_e96842: f64 = (1.0 + assign62380_e96841);
        (assign62380_e96842, ((locals.var_tmf1_dn0 * assign62380_e96840) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign62380_e96838) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign62380_e96836) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign62380_e96834) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign62380_e96832) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign62380_e96830))))))))))), ((locals.var_tmf1_dn2 * assign62380_e96840) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign62380_e96838) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign62380_e96836) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign62380_e96834) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign62380_e96832) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign62380_e96830))))))))))), ((locals.var_tmf1_dn4 * assign62380_e96840) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign62380_e96838) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign62380_e96836) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign62380_e96834) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign62380_e96832) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign62380_e96830))))))))))), ((locals.var_tmf1_dn5 * assign62380_e96840) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign62380_e96838) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign62380_e96836) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign62380_e96834) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign62380_e96832) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign62380_e96830))))))))))), ((locals.var_tmf1_dn6 * assign62380_e96840) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign62380_e96838) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign62380_e96836) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign62380_e96834) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign62380_e96832) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign62380_e96830))))))))))), ((locals.var_tmf1_dn7 * assign62380_e96840) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign62380_e96838) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign62380_e96836) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign62380_e96834) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign62380_e96832) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign62380_e96830))))))))))), ((locals.var_tmf1_dn8 * assign62380_e96840) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign62380_e96838) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign62380_e96836) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign62380_e96834) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign62380_e96832) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign62380_e96830))))))))))), ((locals.var_tmf1_dn9 * assign62380_e96840) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign62380_e96838) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign62380_e96836) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign62380_e96834) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign62380_e96832) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign62380_e96830))))))))))), ((locals.var_tmf1_dn10 * assign62380_e96840) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign62380_e96838) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign62380_e96836) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign62380_e96834) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign62380_e96832) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign62380_e96830))))))))))), ((locals.var_tmf1_dn11 * assign62380_e96840) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign62380_e96838) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign62380_e96836) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign62380_e96834) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign62380_e96832) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign62380_e96830))))))))))), ((locals.var_tmf1_dn14 * assign62380_e96840) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign62380_e96838) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign62380_e96836) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign62380_e96834) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign62380_e96832) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign62380_e96830))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign62380_e96844;
        locals.var_tmf2_dn0 = assign62380_e96844_d_n0;
        locals.var_tmf2_dn2 = assign62380_e96844_d_n2;
        locals.var_tmf2_dn4 = assign62380_e96844_d_n4;
        locals.var_tmf2_dn5 = assign62380_e96844_d_n5;
        locals.var_tmf2_dn6 = assign62380_e96844_d_n6;
        locals.var_tmf2_dn7 = assign62380_e96844_d_n7;
        locals.var_tmf2_dn8 = assign62380_e96844_d_n8;
        locals.var_tmf2_dn9 = assign62380_e96844_d_n9;
        locals.var_tmf2_dn10 = assign62380_e96844_d_n10;
        locals.var_tmf2_dn11 = assign62380_e96844_d_n11;
        locals.var_tmf2_dn14 = assign62380_e96844_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign62390_e96885, assign62390_e96885_d_n0, assign62390_e96885_d_n2, assign62390_e96885_d_n4, assign62390_e96885_d_n5, assign62390_e96885_d_n6, assign62390_e96885_d_n7, assign62390_e96885_d_n8, assign62390_e96885_d_n9, assign62390_e96885_d_n10, assign62390_e96885_d_n11, assign62390_e96885_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62390_e96853: f64 = (1.0 / 2.0);
        let assign62390_e96857: f64 = (1.0 / 3.0);
        let assign62390_e96861: f64 = (1.0 / 8.0);
        let assign62390_e96865: f64 = (1.0 / 30.0);
        let assign62390_e96869: f64 = (1.0 / 144.0);
        let assign62390_e96873: f64 = (1.0 / 840.0);
        let assign62390_e96874: f64 = (locals.var_tmf1 * assign62390_e96873);
        let assign62390_e96875: f64 = (assign62390_e96869 + assign62390_e96874);
        let assign62390_e96876: f64 = (locals.var_tmf1 * assign62390_e96875);
        let assign62390_e96877: f64 = (assign62390_e96865 + assign62390_e96876);
        let assign62390_e96878: f64 = (locals.var_tmf1 * assign62390_e96877);
        let assign62390_e96879: f64 = (assign62390_e96861 + assign62390_e96878);
        let assign62390_e96880: f64 = (locals.var_tmf1 * assign62390_e96879);
        let assign62390_e96881: f64 = (assign62390_e96857 + assign62390_e96880);
        let assign62390_e96882: f64 = (locals.var_tmf1 * assign62390_e96881);
        let assign62390_e96883: f64 = (assign62390_e96853 + assign62390_e96882);
        (assign62390_e96883, ((locals.var_tmf1_dn0 * assign62390_e96881) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign62390_e96879) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign62390_e96877) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign62390_e96875) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign62390_e96873))))))))), ((locals.var_tmf1_dn2 * assign62390_e96881) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign62390_e96879) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign62390_e96877) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign62390_e96875) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign62390_e96873))))))))), ((locals.var_tmf1_dn4 * assign62390_e96881) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign62390_e96879) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign62390_e96877) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign62390_e96875) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign62390_e96873))))))))), ((locals.var_tmf1_dn5 * assign62390_e96881) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign62390_e96879) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign62390_e96877) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign62390_e96875) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign62390_e96873))))))))), ((locals.var_tmf1_dn6 * assign62390_e96881) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign62390_e96879) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign62390_e96877) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign62390_e96875) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign62390_e96873))))))))), ((locals.var_tmf1_dn7 * assign62390_e96881) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign62390_e96879) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign62390_e96877) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign62390_e96875) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign62390_e96873))))))))), ((locals.var_tmf1_dn8 * assign62390_e96881) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign62390_e96879) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign62390_e96877) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign62390_e96875) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign62390_e96873))))))))), ((locals.var_tmf1_dn9 * assign62390_e96881) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign62390_e96879) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign62390_e96877) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign62390_e96875) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign62390_e96873))))))))), ((locals.var_tmf1_dn10 * assign62390_e96881) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign62390_e96879) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign62390_e96877) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign62390_e96875) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign62390_e96873))))))))), ((locals.var_tmf1_dn11 * assign62390_e96881) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign62390_e96879) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign62390_e96877) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign62390_e96875) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign62390_e96873))))))))), ((locals.var_tmf1_dn14 * assign62390_e96881) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign62390_e96879) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign62390_e96877) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign62390_e96875) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign62390_e96873))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign62390_e96885;
        locals.var_tmf3_dn0 = assign62390_e96885_d_n0;
        locals.var_tmf3_dn2 = assign62390_e96885_d_n2;
        locals.var_tmf3_dn4 = assign62390_e96885_d_n4;
        locals.var_tmf3_dn5 = assign62390_e96885_d_n5;
        locals.var_tmf3_dn6 = assign62390_e96885_d_n6;
        locals.var_tmf3_dn7 = assign62390_e96885_d_n7;
        locals.var_tmf3_dn8 = assign62390_e96885_d_n8;
        locals.var_tmf3_dn9 = assign62390_e96885_d_n9;
        locals.var_tmf3_dn10 = assign62390_e96885_d_n10;
        locals.var_tmf3_dn11 = assign62390_e96885_d_n11;
        locals.var_tmf3_dn14 = assign62390_e96885_d_n14;
        locals.var_tmf3_rv = 0.0;

        let (assign62400_e96896, assign62400_e96896_d_n0, assign62400_e96896_d_n2, assign62400_e96896_d_n4, assign62400_e96896_d_n5, assign62400_e96896_d_n6, assign62400_e96896_d_n7, assign62400_e96896_d_n8, assign62400_e96896_d_n9, assign62400_e96896_d_n10, assign62400_e96896_d_n11, assign62400_e96896_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62400_e96894: f64 = (0.01 / locals.var_tmf2);
        (assign62400_e96894, (-((0.01 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign62400_e96896;
        locals.var_t6_dn0 = assign62400_e96896_d_n0;
        locals.var_t6_dn2 = assign62400_e96896_d_n2;
        locals.var_t6_dn4 = assign62400_e96896_d_n4;
        locals.var_t6_dn5 = assign62400_e96896_d_n5;
        locals.var_t6_dn6 = assign62400_e96896_d_n6;
        locals.var_t6_dn7 = assign62400_e96896_d_n7;
        locals.var_t6_dn8 = assign62400_e96896_d_n8;
        locals.var_t6_dn9 = assign62400_e96896_d_n9;
        locals.var_t6_dn10 = assign62400_e96896_d_n10;
        locals.var_t6_dn11 = assign62400_e96896_d_n11;
        locals.var_t6_dn14 = assign62400_e96896_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign62410_e96912, assign62410_e96912_d_n0, assign62410_e96912_d_n2, assign62410_e96912_d_n4, assign62410_e96912_d_n5, assign62410_e96912_d_n6, assign62410_e96912_d_n7, assign62410_e96912_d_n8, assign62410_e96912_d_n9, assign62410_e96912_d_n10, assign62410_e96912_d_n11, assign62410_e96912_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62410_e96904: f64 = (-2.0);
        let assign62410_e96906: f64 = (assign62410_e96904 * locals.var_tmf3);
        let assign62410_e96909: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign62410_e96910: f64 = (assign62410_e96906 / assign62410_e96909);
        (assign62410_e96910, ((((assign62410_e96904 * locals.var_tmf3_dn0) * assign62410_e96909) - (assign62410_e96906 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign62410_e96909 * assign62410_e96909)), ((((assign62410_e96904 * locals.var_tmf3_dn2) * assign62410_e96909) - (assign62410_e96906 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign62410_e96909 * assign62410_e96909)), ((((assign62410_e96904 * locals.var_tmf3_dn4) * assign62410_e96909) - (assign62410_e96906 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign62410_e96909 * assign62410_e96909)), ((((assign62410_e96904 * locals.var_tmf3_dn5) * assign62410_e96909) - (assign62410_e96906 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign62410_e96909 * assign62410_e96909)), ((((assign62410_e96904 * locals.var_tmf3_dn6) * assign62410_e96909) - (assign62410_e96906 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign62410_e96909 * assign62410_e96909)), ((((assign62410_e96904 * locals.var_tmf3_dn7) * assign62410_e96909) - (assign62410_e96906 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign62410_e96909 * assign62410_e96909)), ((((assign62410_e96904 * locals.var_tmf3_dn8) * assign62410_e96909) - (assign62410_e96906 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign62410_e96909 * assign62410_e96909)), ((((assign62410_e96904 * locals.var_tmf3_dn9) * assign62410_e96909) - (assign62410_e96906 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign62410_e96909 * assign62410_e96909)), ((((assign62410_e96904 * locals.var_tmf3_dn10) * assign62410_e96909) - (assign62410_e96906 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign62410_e96909 * assign62410_e96909)), ((((assign62410_e96904 * locals.var_tmf3_dn11) * assign62410_e96909) - (assign62410_e96906 * ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)))) / (assign62410_e96909 * assign62410_e96909)), ((((assign62410_e96904 * locals.var_tmf3_dn14) * assign62410_e96909) - (assign62410_e96906 * ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)))) / (assign62410_e96909 * assign62410_e96909)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign62410_e96912;
        locals.var_t2_dn0 = assign62410_e96912_d_n0;
        locals.var_t2_dn2 = assign62410_e96912_d_n2;
        locals.var_t2_dn4 = assign62410_e96912_d_n4;
        locals.var_t2_dn5 = assign62410_e96912_d_n5;
        locals.var_t2_dn6 = assign62410_e96912_d_n6;
        locals.var_t2_dn7 = assign62410_e96912_d_n7;
        locals.var_t2_dn8 = assign62410_e96912_d_n8;
        locals.var_t2_dn9 = assign62410_e96912_d_n9;
        locals.var_t2_dn10 = assign62410_e96912_d_n10;
        locals.var_t2_dn11 = assign62410_e96912_d_n11;
        locals.var_t2_dn14 = assign62410_e96912_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign62420_e96923, assign62420_e96923_d_n0, assign62420_e96923_d_n2, assign62420_e96923_d_n4, assign62420_e96923_d_n5, assign62420_e96923_d_n6, assign62420_e96923_d_n7, assign62420_e96923_d_n8, assign62420_e96923_d_n9, assign62420_e96923_d_n10, assign62420_e96923_d_n11, assign62420_e96923_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62420_e96921: f64 = (locals.var_t2 * 0.5);
        (assign62420_e96921, (locals.var_t2_dn0 * 0.5), (locals.var_t2_dn2 * 0.5), (locals.var_t2_dn4 * 0.5), (locals.var_t2_dn5 * 0.5), (locals.var_t2_dn6 * 0.5), (locals.var_t2_dn7 * 0.5), (locals.var_t2_dn8 * 0.5), (locals.var_t2_dn9 * 0.5), (locals.var_t2_dn10 * 0.5), (locals.var_t2_dn11 * 0.5), (locals.var_t2_dn14 * 0.5),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign62420_e96923;
        locals.var_t2_dn0 = assign62420_e96923_d_n0;
        locals.var_t2_dn2 = assign62420_e96923_d_n2;
        locals.var_t2_dn4 = assign62420_e96923_d_n4;
        locals.var_t2_dn5 = assign62420_e96923_d_n5;
        locals.var_t2_dn6 = assign62420_e96923_d_n6;
        locals.var_t2_dn7 = assign62420_e96923_d_n7;
        locals.var_t2_dn8 = assign62420_e96923_d_n8;
        locals.var_t2_dn9 = assign62420_e96923_d_n9;
        locals.var_t2_dn10 = assign62420_e96923_d_n10;
        locals.var_t2_dn11 = assign62420_e96923_d_n11;
        locals.var_t2_dn14 = assign62420_e96923_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign62430_e96936, assign62430_e96936_d_n0, assign62430_e96936_d_n2, assign62430_e96936_d_n4, assign62430_e96936_d_n5, assign62430_e96936_d_n6, assign62430_e96936_d_n7, assign62430_e96936_d_n8, assign62430_e96936_d_n9, assign62430_e96936_d_n10, assign62430_e96936_d_n11, assign62430_e96936_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62430_e96933: f64 = (locals.var_ps0 + locals.var_t6);
        let assign62430_e96934: f64 = (1.1 - assign62430_e96933);
        (assign62430_e96934, (-(locals.var_ps0_dn0 + locals.var_t6_dn0)), (-(locals.var_ps0_dn2 + locals.var_t6_dn2)), (-(locals.var_ps0_dn4 + locals.var_t6_dn4)), (-(locals.var_ps0_dn5 + locals.var_t6_dn5)), (-(locals.var_ps0_dn6 + locals.var_t6_dn6)), (-(locals.var_ps0_dn7 + locals.var_t6_dn7)), (-(locals.var_ps0_dn8 + locals.var_t6_dn8)), (-(locals.var_ps0_dn9 + locals.var_t6_dn9)), (-(locals.var_ps0_dn10 + locals.var_t6_dn10)), (-(locals.var_ps0_dn11 + locals.var_t6_dn11)), (-(locals.var_ps0_dn14 + locals.var_t6_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign62430_e96936;
        locals.var_t1_dn0 = assign62430_e96936_d_n0;
        locals.var_t1_dn2 = assign62430_e96936_d_n2;
        locals.var_t1_dn4 = assign62430_e96936_d_n4;
        locals.var_t1_dn5 = assign62430_e96936_d_n5;
        locals.var_t1_dn6 = assign62430_e96936_d_n6;
        locals.var_t1_dn7 = assign62430_e96936_d_n7;
        locals.var_t1_dn8 = assign62430_e96936_d_n8;
        locals.var_t1_dn9 = assign62430_e96936_d_n9;
        locals.var_t1_dn10 = assign62430_e96936_d_n10;
        locals.var_t1_dn11 = assign62430_e96936_d_n11;
        locals.var_t1_dn14 = assign62430_e96936_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign62440_e96954, assign62440_e96954_d_n0, assign62440_e96954_d_n2, assign62440_e96954_d_n4, assign62440_e96954_d_n5, assign62440_e96954_d_n6, assign62440_e96954_d_n7, assign62440_e96954_d_n8, assign62440_e96954_d_n9, assign62440_e96954_d_n10, assign62440_e96954_d_n11, assign62440_e96954_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62440_e96945: f64 = (locals.var_t1 * locals.var_t1);
        let assign62440_e96948: f64 = (4.0 * 0.05);
        let assign62440_e96950: f64 = (assign62440_e96948 * 0.05);
        let assign62440_e96951: f64 = (assign62440_e96945 + assign62440_e96950);
        let assign62440_e96952: f64 = (assign62440_e96951).sqrt();
        (assign62440_e96952, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign62440_e96952)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign62440_e96952)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign62440_e96952)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign62440_e96952)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign62440_e96952)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign62440_e96952)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign62440_e96952)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign62440_e96952)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign62440_e96952)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign62440_e96952)), (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign62440_e96952)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign62440_e96954;
        locals.var_tmf2_dn0 = assign62440_e96954_d_n0;
        locals.var_tmf2_dn2 = assign62440_e96954_d_n2;
        locals.var_tmf2_dn4 = assign62440_e96954_d_n4;
        locals.var_tmf2_dn5 = assign62440_e96954_d_n5;
        locals.var_tmf2_dn6 = assign62440_e96954_d_n6;
        locals.var_tmf2_dn7 = assign62440_e96954_d_n7;
        locals.var_tmf2_dn8 = assign62440_e96954_d_n8;
        locals.var_tmf2_dn9 = assign62440_e96954_d_n9;
        locals.var_tmf2_dn10 = assign62440_e96954_d_n10;
        locals.var_tmf2_dn11 = assign62440_e96954_d_n11;
        locals.var_tmf2_dn14 = assign62440_e96954_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign62450_e96969, assign62450_e96969_d_n0, assign62450_e96969_d_n2, assign62450_e96969_d_n4, assign62450_e96969_d_n5, assign62450_e96969_d_n6, assign62450_e96969_d_n7, assign62450_e96969_d_n8, assign62450_e96969_d_n9, assign62450_e96969_d_n10, assign62450_e96969_d_n11, assign62450_e96969_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62450_e96965: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign62450_e96966: f64 = (1.0 + assign62450_e96965);
        let assign62450_e96967: f64 = (0.5 * assign62450_e96966);
        (assign62450_e96967, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn14 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign62450_e96969;
        locals.var_t0_dn0 = assign62450_e96969_d_n0;
        locals.var_t0_dn2 = assign62450_e96969_d_n2;
        locals.var_t0_dn4 = assign62450_e96969_d_n4;
        locals.var_t0_dn5 = assign62450_e96969_d_n5;
        locals.var_t0_dn6 = assign62450_e96969_d_n6;
        locals.var_t0_dn7 = assign62450_e96969_d_n7;
        locals.var_t0_dn8 = assign62450_e96969_d_n8;
        locals.var_t0_dn9 = assign62450_e96969_d_n9;
        locals.var_t0_dn10 = assign62450_e96969_d_n10;
        locals.var_t0_dn11 = assign62450_e96969_d_n11;
        locals.var_t0_dn14 = assign62450_e96969_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign62460_e96982, assign62460_e96982_d_n0, assign62460_e96982_d_n2, assign62460_e96982_d_n4, assign62460_e96982_d_n5, assign62460_e96982_d_n6, assign62460_e96982_d_n7, assign62460_e96982_d_n8, assign62460_e96982_d_n9, assign62460_e96982_d_n10, assign62460_e96982_d_n11, assign62460_e96982_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62460_e96979: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign62460_e96980: f64 = (0.5 * assign62460_e96979);
        (assign62460_e96980, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign62460_e96982;
        locals.var_t2_dn0 = assign62460_e96982_d_n0;
        locals.var_t2_dn2 = assign62460_e96982_d_n2;
        locals.var_t2_dn4 = assign62460_e96982_d_n4;
        locals.var_t2_dn5 = assign62460_e96982_d_n5;
        locals.var_t2_dn6 = assign62460_e96982_d_n6;
        locals.var_t2_dn7 = assign62460_e96982_d_n7;
        locals.var_t2_dn8 = assign62460_e96982_d_n8;
        locals.var_t2_dn9 = assign62460_e96982_d_n9;
        locals.var_t2_dn10 = assign62460_e96982_d_n10;
        locals.var_t2_dn11 = assign62460_e96982_d_n11;
        locals.var_t2_dn14 = assign62460_e96982_d_n14;
        locals.var_t2_rv = 0.0;

        let assign62470_e96985: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1500 = assign62470_e96985;
        locals.var_guard1500_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_233(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign62480_e96996, assign62480_e96996_d_n0, assign62480_e96996_d_n2, assign62480_e96996_d_n4, assign62480_e96996_d_n5, assign62480_e96996_d_n6, assign62480_e96996_d_n7, assign62480_e96996_d_n8, assign62480_e96996_d_n9, assign62480_e96996_d_n10, assign62480_e96996_d_n11, assign62480_e96996_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) && (locals.var_guard1500 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign62480_e96996;
        locals.var_t2_dn0 = assign62480_e96996_d_n0;
        locals.var_t2_dn2 = assign62480_e96996_d_n2;
        locals.var_t2_dn4 = assign62480_e96996_d_n4;
        locals.var_t2_dn5 = assign62480_e96996_d_n5;
        locals.var_t2_dn6 = assign62480_e96996_d_n6;
        locals.var_t2_dn7 = assign62480_e96996_d_n7;
        locals.var_t2_dn8 = assign62480_e96996_d_n8;
        locals.var_t2_dn9 = assign62480_e96996_d_n9;
        locals.var_t2_dn10 = assign62480_e96996_d_n10;
        locals.var_t2_dn11 = assign62480_e96996_d_n11;
        locals.var_t2_dn14 = assign62480_e96996_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign62490_e97007, assign62490_e97007_d_n0, assign62490_e97007_d_n2, assign62490_e97007_d_n4, assign62490_e97007_d_n5, assign62490_e97007_d_n6, assign62490_e97007_d_n7, assign62490_e97007_d_n8, assign62490_e97007_d_n9, assign62490_e97007_d_n10, assign62490_e97007_d_n11, assign62490_e97007_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) && (locals.var_guard1500 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign62490_e97007;
        locals.var_t0_dn0 = assign62490_e97007_d_n0;
        locals.var_t0_dn2 = assign62490_e97007_d_n2;
        locals.var_t0_dn4 = assign62490_e97007_d_n4;
        locals.var_t0_dn5 = assign62490_e97007_d_n5;
        locals.var_t0_dn6 = assign62490_e97007_d_n6;
        locals.var_t0_dn7 = assign62490_e97007_d_n7;
        locals.var_t0_dn8 = assign62490_e97007_d_n8;
        locals.var_t0_dn9 = assign62490_e97007_d_n9;
        locals.var_t0_dn10 = assign62490_e97007_d_n10;
        locals.var_t0_dn11 = assign62490_e97007_d_n11;
        locals.var_t0_dn14 = assign62490_e97007_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign62500_e97018, assign62500_e97018_d_n0, assign62500_e97018_d_n2, assign62500_e97018_d_n4, assign62500_e97018_d_n5, assign62500_e97018_d_n6, assign62500_e97018_d_n7, assign62500_e97018_d_n8, assign62500_e97018_d_n9, assign62500_e97018_d_n10, assign62500_e97018_d_n11, assign62500_e97018_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62500_e97016: f64 = (locals.var_t2 + 1e-25);
        (assign62500_e97016, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign62500_e97018;
        locals.var_t2_dn0 = assign62500_e97018_d_n0;
        locals.var_t2_dn2 = assign62500_e97018_d_n2;
        locals.var_t2_dn4 = assign62500_e97018_d_n4;
        locals.var_t2_dn5 = assign62500_e97018_d_n5;
        locals.var_t2_dn6 = assign62500_e97018_d_n6;
        locals.var_t2_dn7 = assign62500_e97018_d_n7;
        locals.var_t2_dn8 = assign62500_e97018_d_n8;
        locals.var_t2_dn9 = assign62500_e97018_d_n9;
        locals.var_t2_dn10 = assign62500_e97018_d_n10;
        locals.var_t2_dn11 = assign62500_e97018_d_n11;
        locals.var_t2_dn14 = assign62500_e97018_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign62510_e97029, assign62510_e97029_d_n0, assign62510_e97029_d_n2, assign62510_e97029_d_n4, assign62510_e97029_d_n5, assign62510_e97029_d_n6, assign62510_e97029_d_n7, assign62510_e97029_d_n8, assign62510_e97029_d_n9, assign62510_e97029_d_n10, assign62510_e97029_d_n11, assign62510_e97029_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62510_e97027: f64 = (locals.var_beta * locals.var_ptl0);
        (assign62510_e97027, (locals.var_beta_dn0 * locals.var_ptl0), (locals.var_beta_dn2 * locals.var_ptl0), (locals.var_beta_dn4 * locals.var_ptl0), (locals.var_beta_dn5 * locals.var_ptl0), (locals.var_beta_dn6 * locals.var_ptl0), (locals.var_beta_dn7 * locals.var_ptl0), (locals.var_beta_dn8 * locals.var_ptl0), (locals.var_beta_dn9 * locals.var_ptl0), (locals.var_beta_dn10 * locals.var_ptl0), (locals.var_beta_dn11 * locals.var_ptl0), (locals.var_beta_dn14 * locals.var_ptl0),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign62510_e97029;
        locals.var_t0_dn0 = assign62510_e97029_d_n0;
        locals.var_t0_dn2 = assign62510_e97029_d_n2;
        locals.var_t0_dn4 = assign62510_e97029_d_n4;
        locals.var_t0_dn5 = assign62510_e97029_d_n5;
        locals.var_t0_dn6 = assign62510_e97029_d_n6;
        locals.var_t0_dn7 = assign62510_e97029_d_n7;
        locals.var_t0_dn8 = assign62510_e97029_d_n8;
        locals.var_t0_dn9 = assign62510_e97029_d_n9;
        locals.var_t0_dn10 = assign62510_e97029_d_n10;
        locals.var_t0_dn11 = assign62510_e97029_d_n11;
        locals.var_t0_dn14 = assign62510_e97029_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign62520_e97040, assign62520_e97040_d_n0, assign62520_e97040_d_n2, assign62520_e97040_d_n4, assign62520_e97040_d_n5, assign62520_e97040_d_n6, assign62520_e97040_d_n7, assign62520_e97040_d_n8, assign62520_e97040_d_n9, assign62520_e97040_d_n10, assign62520_e97040_d_n11, assign62520_e97040_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62520_e97038: f64 = (locals.var_cox * locals.var_t0);
        (assign62520_e97038, ((locals.var_cox_dn0 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn0)), ((locals.var_cox_dn2 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn2)), ((locals.var_cox_dn4 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn4)), ((locals.var_cox_dn5 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn5)), ((locals.var_cox_dn6 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn6)), ((locals.var_cox_dn7 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn7)), ((locals.var_cox_dn8 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn8)), ((locals.var_cox_dn9 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn9)), ((locals.var_cox_dn10 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn10)), ((locals.var_cox_dn11 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn11)), ((locals.var_cox_dn14 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign62520_e97040;
        locals.var_t3_dn0 = assign62520_e97040_d_n0;
        locals.var_t3_dn2 = assign62520_e97040_d_n2;
        locals.var_t3_dn4 = assign62520_e97040_d_n4;
        locals.var_t3_dn5 = assign62520_e97040_d_n5;
        locals.var_t3_dn6 = assign62520_e97040_d_n6;
        locals.var_t3_dn7 = assign62520_e97040_d_n7;
        locals.var_t3_dn8 = assign62520_e97040_d_n8;
        locals.var_t3_dn9 = assign62520_e97040_d_n9;
        locals.var_t3_dn10 = assign62520_e97040_d_n10;
        locals.var_t3_dn11 = assign62520_e97040_d_n11;
        locals.var_t3_dn14 = assign62520_e97040_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign62530_e97051, assign62530_e97051_d_n0, assign62530_e97051_d_n2, assign62530_e97051_d_n4, assign62530_e97051_d_n5, assign62530_e97051_d_n6, assign62530_e97051_d_n7, assign62530_e97051_d_n8, assign62530_e97051_d_n9, assign62530_e97051_d_n10, assign62530_e97051_d_n11, assign62530_e97051_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62530_e97049: f64 = (locals.var_t2).powf(p.p284);
        (assign62530_e97049, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn0)) } } else { (assign62530_e97049 * (p.p284 * (locals.var_t2_dn0 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn2)) } } else { (assign62530_e97049 * (p.p284 * (locals.var_t2_dn2 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn4)) } } else { (assign62530_e97049 * (p.p284 * (locals.var_t2_dn4 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn5)) } } else { (assign62530_e97049 * (p.p284 * (locals.var_t2_dn5 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn6)) } } else { (assign62530_e97049 * (p.p284 * (locals.var_t2_dn6 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn7)) } } else { (assign62530_e97049 * (p.p284 * (locals.var_t2_dn7 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn8)) } } else { (assign62530_e97049 * (p.p284 * (locals.var_t2_dn8 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn9)) } } else { (assign62530_e97049 * (p.p284 * (locals.var_t2_dn9 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn10)) } } else { (assign62530_e97049 * (p.p284 * (locals.var_t2_dn10 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn11)) } } else { (assign62530_e97049 * (p.p284 * (locals.var_t2_dn11 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn14)) } } else { (assign62530_e97049 * (p.p284 * (locals.var_t2_dn14 / locals.var_t2))) },)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign62530_e97051;
        locals.var_t0_dn0 = assign62530_e97051_d_n0;
        locals.var_t0_dn2 = assign62530_e97051_d_n2;
        locals.var_t0_dn4 = assign62530_e97051_d_n4;
        locals.var_t0_dn5 = assign62530_e97051_d_n5;
        locals.var_t0_dn6 = assign62530_e97051_d_n6;
        locals.var_t0_dn7 = assign62530_e97051_d_n7;
        locals.var_t0_dn8 = assign62530_e97051_d_n8;
        locals.var_t0_dn9 = assign62530_e97051_d_n9;
        locals.var_t0_dn10 = assign62530_e97051_d_n10;
        locals.var_t0_dn11 = assign62530_e97051_d_n11;
        locals.var_t0_dn14 = assign62530_e97051_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign62540_e97062, assign62540_e97062_d_n0, assign62540_e97062_d_n2, assign62540_e97062_d_n4, assign62540_e97062_d_n5, assign62540_e97062_d_n6, assign62540_e97062_d_n7, assign62540_e97062_d_n8, assign62540_e97062_d_n9, assign62540_e97062_d_n10, assign62540_e97062_d_n11, assign62540_e97062_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62540_e97060: f64 = (locals.var_t3 * locals.var_t0);
        (assign62540_e97060, ((locals.var_t3_dn0 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn0)), ((locals.var_t3_dn2 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn2)), ((locals.var_t3_dn4 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn4)), ((locals.var_t3_dn5 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn5)), ((locals.var_t3_dn6 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn6)), ((locals.var_t3_dn7 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn7)), ((locals.var_t3_dn8 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn8)), ((locals.var_t3_dn9 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn9)), ((locals.var_t3_dn10 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn10)), ((locals.var_t3_dn11 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn11)), ((locals.var_t3_dn14 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign62540_e97062;
        locals.var_t9_dn0 = assign62540_e97062_d_n0;
        locals.var_t9_dn2 = assign62540_e97062_d_n2;
        locals.var_t9_dn4 = assign62540_e97062_d_n4;
        locals.var_t9_dn5 = assign62540_e97062_d_n5;
        locals.var_t9_dn6 = assign62540_e97062_d_n6;
        locals.var_t9_dn7 = assign62540_e97062_d_n7;
        locals.var_t9_dn8 = assign62540_e97062_d_n8;
        locals.var_t9_dn9 = assign62540_e97062_d_n9;
        locals.var_t9_dn10 = assign62540_e97062_d_n10;
        locals.var_t9_dn11 = assign62540_e97062_d_n11;
        locals.var_t9_dn14 = assign62540_e97062_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign62550_e97075, assign62550_e97075_d_n0, assign62550_e97075_d_n2, assign62550_e97075_d_n4, assign62550_e97075_d_n5, assign62550_e97075_d_n6, assign62550_e97075_d_n7, assign62550_e97075_d_n8, assign62550_e97075_d_n9, assign62550_e97075_d_n10, assign62550_e97075_d_n11, assign62550_e97075_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62550_e97072: f64 = (locals.var_vdsz__blk443 * p.p285);
        let assign62550_e97073: f64 = (1.0 + assign62550_e97072);
        (assign62550_e97073, (locals.var_vdsz__blk443_dn0 * p.p285), (locals.var_vdsz__blk443_dn2 * p.p285), (locals.var_vdsz__blk443_dn4 * p.p285), (locals.var_vdsz__blk443_dn5 * p.p285), (locals.var_vdsz__blk443_dn6 * p.p285), (locals.var_vdsz__blk443_dn7 * p.p285), (locals.var_vdsz__blk443_dn8 * p.p285), (locals.var_vdsz__blk443_dn9 * p.p285), (locals.var_vdsz__blk443_dn10 * p.p285), (locals.var_vdsz__blk443_dn11 * p.p285), (locals.var_vdsz__blk443_dn14 * p.p285),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign62550_e97075;
        locals.var_t4_dn0 = assign62550_e97075_d_n0;
        locals.var_t4_dn2 = assign62550_e97075_d_n2;
        locals.var_t4_dn4 = assign62550_e97075_d_n4;
        locals.var_t4_dn5 = assign62550_e97075_d_n5;
        locals.var_t4_dn6 = assign62550_e97075_d_n6;
        locals.var_t4_dn7 = assign62550_e97075_d_n7;
        locals.var_t4_dn8 = assign62550_e97075_d_n8;
        locals.var_t4_dn9 = assign62550_e97075_d_n9;
        locals.var_t4_dn10 = assign62550_e97075_d_n10;
        locals.var_t4_dn11 = assign62550_e97075_d_n11;
        locals.var_t4_dn14 = assign62550_e97075_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign62560_e97084, assign62560_e97084_d_n0, assign62560_e97084_d_n2, assign62560_e97084_d_n4, assign62560_e97084_d_n5, assign62560_e97084_d_n6, assign62560_e97084_d_n7, assign62560_e97084_d_n8, assign62560_e97084_d_n9, assign62560_e97084_d_n10, assign62560_e97084_d_n11, assign62560_e97084_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        (locals.var_pt40, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign62560_e97084;
        locals.var_t0_dn0 = assign62560_e97084_d_n0;
        locals.var_t0_dn2 = assign62560_e97084_d_n2;
        locals.var_t0_dn4 = assign62560_e97084_d_n4;
        locals.var_t0_dn5 = assign62560_e97084_d_n5;
        locals.var_t0_dn6 = assign62560_e97084_d_n6;
        locals.var_t0_dn7 = assign62560_e97084_d_n7;
        locals.var_t0_dn8 = assign62560_e97084_d_n8;
        locals.var_t0_dn9 = assign62560_e97084_d_n9;
        locals.var_t0_dn10 = assign62560_e97084_d_n10;
        locals.var_t0_dn11 = assign62560_e97084_d_n11;
        locals.var_t0_dn14 = assign62560_e97084_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign62570_e97097, assign62570_e97097_d_n0, assign62570_e97097_d_n2, assign62570_e97097_d_n4, assign62570_e97097_d_n5, assign62570_e97097_d_n6, assign62570_e97097_d_n7, assign62570_e97097_d_n8, assign62570_e97097_d_n9, assign62570_e97097_d_n10, assign62570_e97097_d_n11, assign62570_e97097_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62570_e97093: f64 = (locals.var_ps0 + locals.var_t6);
        let assign62570_e97095: f64 = (assign62570_e97093 - locals.var_vbsz__blk442);
        (assign62570_e97095, ((locals.var_ps0_dn0 + locals.var_t6_dn0) - locals.var_vbsz__blk442_dn0), ((locals.var_ps0_dn2 + locals.var_t6_dn2) - locals.var_vbsz__blk442_dn2), ((locals.var_ps0_dn4 + locals.var_t6_dn4) - locals.var_vbsz__blk442_dn4), ((locals.var_ps0_dn5 + locals.var_t6_dn5) - locals.var_vbsz__blk442_dn5), ((locals.var_ps0_dn6 + locals.var_t6_dn6) - locals.var_vbsz__blk442_dn6), ((locals.var_ps0_dn7 + locals.var_t6_dn7) - locals.var_vbsz__blk442_dn7), ((locals.var_ps0_dn8 + locals.var_t6_dn8) - locals.var_vbsz__blk442_dn8), ((locals.var_ps0_dn9 + locals.var_t6_dn9) - locals.var_vbsz__blk442_dn9), ((locals.var_ps0_dn10 + locals.var_t6_dn10) - locals.var_vbsz__blk442_dn10), ((locals.var_ps0_dn11 + locals.var_t6_dn11) - locals.var_vbsz__blk442_dn11), ((locals.var_ps0_dn14 + locals.var_t6_dn14) - locals.var_vbsz__blk442_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign62570_e97097;
        locals.var_t5_dn0 = assign62570_e97097_d_n0;
        locals.var_t5_dn2 = assign62570_e97097_d_n2;
        locals.var_t5_dn4 = assign62570_e97097_d_n4;
        locals.var_t5_dn5 = assign62570_e97097_d_n5;
        locals.var_t5_dn6 = assign62570_e97097_d_n6;
        locals.var_t5_dn7 = assign62570_e97097_d_n7;
        locals.var_t5_dn8 = assign62570_e97097_d_n8;
        locals.var_t5_dn9 = assign62570_e97097_d_n9;
        locals.var_t5_dn10 = assign62570_e97097_d_n10;
        locals.var_t5_dn11 = assign62570_e97097_d_n11;
        locals.var_t5_dn14 = assign62570_e97097_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign62580_e97112, assign62580_e97112_d_n0, assign62580_e97112_d_n2, assign62580_e97112_d_n4, assign62580_e97112_d_n5, assign62580_e97112_d_n6, assign62580_e97112_d_n7, assign62580_e97112_d_n8, assign62580_e97112_d_n9, assign62580_e97112_d_n10, assign62580_e97112_d_n11, assign62580_e97112_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62580_e97107: f64 = (locals.var_vdsz__blk443 * locals.var_t0);
        let assign62580_e97109: f64 = (assign62580_e97107 * locals.var_t5);
        let assign62580_e97110: f64 = (locals.var_t4 + assign62580_e97109);
        (assign62580_e97110, (locals.var_t4_dn0 + ((((locals.var_vdsz__blk443_dn0 * locals.var_t0) + (locals.var_vdsz__blk443 * locals.var_t0_dn0)) * locals.var_t5) + (assign62580_e97107 * locals.var_t5_dn0))), (locals.var_t4_dn2 + ((((locals.var_vdsz__blk443_dn2 * locals.var_t0) + (locals.var_vdsz__blk443 * locals.var_t0_dn2)) * locals.var_t5) + (assign62580_e97107 * locals.var_t5_dn2))), (locals.var_t4_dn4 + ((((locals.var_vdsz__blk443_dn4 * locals.var_t0) + (locals.var_vdsz__blk443 * locals.var_t0_dn4)) * locals.var_t5) + (assign62580_e97107 * locals.var_t5_dn4))), (locals.var_t4_dn5 + ((((locals.var_vdsz__blk443_dn5 * locals.var_t0) + (locals.var_vdsz__blk443 * locals.var_t0_dn5)) * locals.var_t5) + (assign62580_e97107 * locals.var_t5_dn5))), (locals.var_t4_dn6 + ((((locals.var_vdsz__blk443_dn6 * locals.var_t0) + (locals.var_vdsz__blk443 * locals.var_t0_dn6)) * locals.var_t5) + (assign62580_e97107 * locals.var_t5_dn6))), (locals.var_t4_dn7 + ((((locals.var_vdsz__blk443_dn7 * locals.var_t0) + (locals.var_vdsz__blk443 * locals.var_t0_dn7)) * locals.var_t5) + (assign62580_e97107 * locals.var_t5_dn7))), (locals.var_t4_dn8 + ((((locals.var_vdsz__blk443_dn8 * locals.var_t0) + (locals.var_vdsz__blk443 * locals.var_t0_dn8)) * locals.var_t5) + (assign62580_e97107 * locals.var_t5_dn8))), (locals.var_t4_dn9 + ((((locals.var_vdsz__blk443_dn9 * locals.var_t0) + (locals.var_vdsz__blk443 * locals.var_t0_dn9)) * locals.var_t5) + (assign62580_e97107 * locals.var_t5_dn9))), (locals.var_t4_dn10 + ((((locals.var_vdsz__blk443_dn10 * locals.var_t0) + (locals.var_vdsz__blk443 * locals.var_t0_dn10)) * locals.var_t5) + (assign62580_e97107 * locals.var_t5_dn10))), (locals.var_t4_dn11 + ((((locals.var_vdsz__blk443_dn11 * locals.var_t0) + (locals.var_vdsz__blk443 * locals.var_t0_dn11)) * locals.var_t5) + (assign62580_e97107 * locals.var_t5_dn11))), (locals.var_t4_dn14 + ((((locals.var_vdsz__blk443_dn14 * locals.var_t0) + (locals.var_vdsz__blk443 * locals.var_t0_dn14)) * locals.var_t5) + (assign62580_e97107 * locals.var_t5_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign62580_e97112;
        locals.var_t4_dn0 = assign62580_e97112_d_n0;
        locals.var_t4_dn2 = assign62580_e97112_d_n2;
        locals.var_t4_dn4 = assign62580_e97112_d_n4;
        locals.var_t4_dn5 = assign62580_e97112_d_n5;
        locals.var_t4_dn6 = assign62580_e97112_d_n6;
        locals.var_t4_dn7 = assign62580_e97112_d_n7;
        locals.var_t4_dn8 = assign62580_e97112_d_n8;
        locals.var_t4_dn9 = assign62580_e97112_d_n9;
        locals.var_t4_dn10 = assign62580_e97112_d_n10;
        locals.var_t4_dn11 = assign62580_e97112_d_n11;
        locals.var_t4_dn14 = assign62580_e97112_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign62590_e97123, assign62590_e97123_d_n0, assign62590_e97123_d_n2, assign62590_e97123_d_n4, assign62590_e97123_d_n5, assign62590_e97123_d_n6, assign62590_e97123_d_n7, assign62590_e97123_d_n8, assign62590_e97123_d_n9, assign62590_e97123_d_n10, assign62590_e97123_d_n11, assign62590_e97123_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62590_e97121: f64 = (locals.var_t9 * locals.var_t4);
        (assign62590_e97121, ((locals.var_t9_dn0 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn0)), ((locals.var_t9_dn2 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn2)), ((locals.var_t9_dn4 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn4)), ((locals.var_t9_dn5 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn5)), ((locals.var_t9_dn6 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn6)), ((locals.var_t9_dn7 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn7)), ((locals.var_t9_dn8 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn8)), ((locals.var_t9_dn9 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn9)), ((locals.var_t9_dn10 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn10)), ((locals.var_t9_dn11 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn11)), ((locals.var_t9_dn14 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign62590_e97123;
        locals.var_t6_dn0 = assign62590_e97123_d_n0;
        locals.var_t6_dn2 = assign62590_e97123_d_n2;
        locals.var_t6_dn4 = assign62590_e97123_d_n4;
        locals.var_t6_dn5 = assign62590_e97123_d_n5;
        locals.var_t6_dn6 = assign62590_e97123_d_n6;
        locals.var_t6_dn7 = assign62590_e97123_d_n7;
        locals.var_t6_dn8 = assign62590_e97123_d_n8;
        locals.var_t6_dn9 = assign62590_e97123_d_n9;
        locals.var_t6_dn10 = assign62590_e97123_d_n10;
        locals.var_t6_dn11 = assign62590_e97123_d_n11;
        locals.var_t6_dn14 = assign62590_e97123_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign62600_e97132, assign62600_e97132_d_n0, assign62600_e97132_d_n2, assign62600_e97132_d_n4, assign62600_e97132_d_n5, assign62600_e97132_d_n6, assign62600_e97132_d_n7, assign62600_e97132_d_n8, assign62600_e97132_d_n9, assign62600_e97132_d_n10, assign62600_e97132_d_n11, assign62600_e97132_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign62600_e97132;
        locals.var_t9_dn0 = assign62600_e97132_d_n0;
        locals.var_t9_dn2 = assign62600_e97132_d_n2;
        locals.var_t9_dn4 = assign62600_e97132_d_n4;
        locals.var_t9_dn5 = assign62600_e97132_d_n5;
        locals.var_t9_dn6 = assign62600_e97132_d_n6;
        locals.var_t9_dn7 = assign62600_e97132_d_n7;
        locals.var_t9_dn8 = assign62600_e97132_d_n8;
        locals.var_t9_dn9 = assign62600_e97132_d_n9;
        locals.var_t9_dn10 = assign62600_e97132_d_n10;
        locals.var_t9_dn11 = assign62600_e97132_d_n11;
        locals.var_t9_dn14 = assign62600_e97132_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign62610_e97142, assign62610_e97142_d_n0, assign62610_e97142_d_n2, assign62610_e97142_d_n4, assign62610_e97142_d_n5, assign62610_e97142_d_n6, assign62610_e97142_d_n7, assign62610_e97142_d_n8, assign62610_e97142_d_n9, assign62610_e97142_d_n10, assign62610_e97142_d_n11, assign62610_e97142_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign62610_e97142;
        locals.var_t9_dn0 = assign62610_e97142_d_n0;
        locals.var_t9_dn2 = assign62610_e97142_d_n2;
        locals.var_t9_dn4 = assign62610_e97142_d_n4;
        locals.var_t9_dn5 = assign62610_e97142_d_n5;
        locals.var_t9_dn6 = assign62610_e97142_d_n6;
        locals.var_t9_dn7 = assign62610_e97142_d_n7;
        locals.var_t9_dn8 = assign62610_e97142_d_n8;
        locals.var_t9_dn9 = assign62610_e97142_d_n9;
        locals.var_t9_dn10 = assign62610_e97142_d_n10;
        locals.var_t9_dn11 = assign62610_e97142_d_n11;
        locals.var_t9_dn14 = assign62610_e97142_d_n14;
        locals.var_t9_rv = 0.0;

        let assign62620_e97145: f64 = if p.p287 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1501 = assign62620_e97145;
        locals.var_guard1501_rv = 0.0;

        let (assign62630_e97156, assign62630_e97156_d_n0, assign62630_e97156_d_n2, assign62630_e97156_d_n4, assign62630_e97156_d_n5, assign62630_e97156_d_n6, assign62630_e97156_d_n7, assign62630_e97156_d_n8, assign62630_e97156_d_n9, assign62630_e97156_d_n10, assign62630_e97156_d_n11, assign62630_e97156_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1501 != 0.0)) {
        let assign62630_e97154: f64 = (locals.var_beta * locals.var_gdl0);
        (assign62630_e97154, (locals.var_beta_dn0 * locals.var_gdl0), (locals.var_beta_dn2 * locals.var_gdl0), (locals.var_beta_dn4 * locals.var_gdl0), (locals.var_beta_dn5 * locals.var_gdl0), (locals.var_beta_dn6 * locals.var_gdl0), (locals.var_beta_dn7 * locals.var_gdl0), (locals.var_beta_dn8 * locals.var_gdl0), (locals.var_beta_dn9 * locals.var_gdl0), (locals.var_beta_dn10 * locals.var_gdl0), (locals.var_beta_dn11 * locals.var_gdl0), (locals.var_beta_dn14 * locals.var_gdl0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign62630_e97156;
        locals.var_t1_dn0 = assign62630_e97156_d_n0;
        locals.var_t1_dn2 = assign62630_e97156_d_n2;
        locals.var_t1_dn4 = assign62630_e97156_d_n4;
        locals.var_t1_dn5 = assign62630_e97156_d_n5;
        locals.var_t1_dn6 = assign62630_e97156_d_n6;
        locals.var_t1_dn7 = assign62630_e97156_d_n7;
        locals.var_t1_dn8 = assign62630_e97156_d_n8;
        locals.var_t1_dn9 = assign62630_e97156_d_n9;
        locals.var_t1_dn10 = assign62630_e97156_d_n10;
        locals.var_t1_dn11 = assign62630_e97156_d_n11;
        locals.var_t1_dn14 = assign62630_e97156_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign62640_e97167, assign62640_e97167_d_n0, assign62640_e97167_d_n2, assign62640_e97167_d_n4, assign62640_e97167_d_n5, assign62640_e97167_d_n6, assign62640_e97167_d_n7, assign62640_e97167_d_n8, assign62640_e97167_d_n9, assign62640_e97167_d_n10, assign62640_e97167_d_n11, assign62640_e97167_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1501 != 0.0)) {
        let assign62640_e97165: f64 = (locals.var_cox * locals.var_t1);
        (assign62640_e97165, ((locals.var_cox_dn0 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn0)), ((locals.var_cox_dn2 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn2)), ((locals.var_cox_dn4 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn4)), ((locals.var_cox_dn5 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn5)), ((locals.var_cox_dn6 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn6)), ((locals.var_cox_dn7 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn7)), ((locals.var_cox_dn8 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn8)), ((locals.var_cox_dn9 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn9)), ((locals.var_cox_dn10 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn10)), ((locals.var_cox_dn11 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn11)), ((locals.var_cox_dn14 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign62640_e97167;
        locals.var_t2_dn0 = assign62640_e97167_d_n0;
        locals.var_t2_dn2 = assign62640_e97167_d_n2;
        locals.var_t2_dn4 = assign62640_e97167_d_n4;
        locals.var_t2_dn5 = assign62640_e97167_d_n5;
        locals.var_t2_dn6 = assign62640_e97167_d_n6;
        locals.var_t2_dn7 = assign62640_e97167_d_n7;
        locals.var_t2_dn8 = assign62640_e97167_d_n8;
        locals.var_t2_dn9 = assign62640_e97167_d_n9;
        locals.var_t2_dn10 = assign62640_e97167_d_n10;
        locals.var_t2_dn11 = assign62640_e97167_d_n11;
        locals.var_t2_dn14 = assign62640_e97167_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign62650_e97178, assign62650_e97178_d_n0, assign62650_e97178_d_n2, assign62650_e97178_d_n4, assign62650_e97178_d_n5, assign62650_e97178_d_n6, assign62650_e97178_d_n7, assign62650_e97178_d_n8, assign62650_e97178_d_n9, assign62650_e97178_d_n10, assign62650_e97178_d_n11, assign62650_e97178_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1501 != 0.0)) {
        let assign62650_e97176: f64 = (locals.var_t2 * locals.var_vdsz__blk443);
        (assign62650_e97176, ((locals.var_t2_dn0 * locals.var_vdsz__blk443) + (locals.var_t2 * locals.var_vdsz__blk443_dn0)), ((locals.var_t2_dn2 * locals.var_vdsz__blk443) + (locals.var_t2 * locals.var_vdsz__blk443_dn2)), ((locals.var_t2_dn4 * locals.var_vdsz__blk443) + (locals.var_t2 * locals.var_vdsz__blk443_dn4)), ((locals.var_t2_dn5 * locals.var_vdsz__blk443) + (locals.var_t2 * locals.var_vdsz__blk443_dn5)), ((locals.var_t2_dn6 * locals.var_vdsz__blk443) + (locals.var_t2 * locals.var_vdsz__blk443_dn6)), ((locals.var_t2_dn7 * locals.var_vdsz__blk443) + (locals.var_t2 * locals.var_vdsz__blk443_dn7)), ((locals.var_t2_dn8 * locals.var_vdsz__blk443) + (locals.var_t2 * locals.var_vdsz__blk443_dn8)), ((locals.var_t2_dn9 * locals.var_vdsz__blk443) + (locals.var_t2 * locals.var_vdsz__blk443_dn9)), ((locals.var_t2_dn10 * locals.var_vdsz__blk443) + (locals.var_t2 * locals.var_vdsz__blk443_dn10)), ((locals.var_t2_dn11 * locals.var_vdsz__blk443) + (locals.var_t2 * locals.var_vdsz__blk443_dn11)), ((locals.var_t2_dn14 * locals.var_vdsz__blk443) + (locals.var_t2 * locals.var_vdsz__blk443_dn14)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign62650_e97178;
        locals.var_t8_dn0 = assign62650_e97178_d_n0;
        locals.var_t8_dn2 = assign62650_e97178_d_n2;
        locals.var_t8_dn4 = assign62650_e97178_d_n4;
        locals.var_t8_dn5 = assign62650_e97178_d_n5;
        locals.var_t8_dn6 = assign62650_e97178_d_n6;
        locals.var_t8_dn7 = assign62650_e97178_d_n7;
        locals.var_t8_dn8 = assign62650_e97178_d_n8;
        locals.var_t8_dn9 = assign62650_e97178_d_n9;
        locals.var_t8_dn10 = assign62650_e97178_d_n10;
        locals.var_t8_dn11 = assign62650_e97178_d_n11;
        locals.var_t8_dn14 = assign62650_e97178_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign62660_e97188, assign62660_e97188_d_n0, assign62660_e97188_d_n2, assign62660_e97188_d_n4, assign62660_e97188_d_n5, assign62660_e97188_d_n6, assign62660_e97188_d_n7, assign62660_e97188_d_n8, assign62660_e97188_d_n9, assign62660_e97188_d_n10, assign62660_e97188_d_n11, assign62660_e97188_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1501 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign62660_e97188;
        locals.var_t8_dn0 = assign62660_e97188_d_n0;
        locals.var_t8_dn2 = assign62660_e97188_d_n2;
        locals.var_t8_dn4 = assign62660_e97188_d_n4;
        locals.var_t8_dn5 = assign62660_e97188_d_n5;
        locals.var_t8_dn6 = assign62660_e97188_d_n6;
        locals.var_t8_dn7 = assign62660_e97188_d_n7;
        locals.var_t8_dn8 = assign62660_e97188_d_n8;
        locals.var_t8_dn9 = assign62660_e97188_d_n9;
        locals.var_t8_dn10 = assign62660_e97188_d_n10;
        locals.var_t8_dn11 = assign62660_e97188_d_n11;
        locals.var_t8_dn14 = assign62660_e97188_d_n14;
        locals.var_t8_rv = 0.0;

        let assign62670_e97191: f64 = (locals.var_t9 + locals.var_t8);
        let assign62670_e97193: f64 = if assign62670_e97191 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1502 = assign62670_e97193;
        locals.var_guard1502_rv = 0.0;

        let (assign62680_e97206, assign62680_e97206_d_n0, assign62680_e97206_d_n2, assign62680_e97206_d_n4, assign62680_e97206_d_n5, assign62680_e97206_d_n6, assign62680_e97206_d_n7, assign62680_e97206_d_n8, assign62680_e97206_d_n9, assign62680_e97206_d_n10, assign62680_e97206_d_n11, assign62680_e97206_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign62680_e97203: f64 = (locals.var_t9 + locals.var_t8);
        let assign62680_e97204: f64 = (locals.var_pds * assign62680_e97203);
        (assign62680_e97204, ((locals.var_pds_dn0 * assign62680_e97203) + (locals.var_pds * (locals.var_t9_dn0 + locals.var_t8_dn0))), ((locals.var_pds_dn2 * assign62680_e97203) + (locals.var_pds * (locals.var_t9_dn2 + locals.var_t8_dn2))), ((locals.var_pds_dn4 * assign62680_e97203) + (locals.var_pds * (locals.var_t9_dn4 + locals.var_t8_dn4))), ((locals.var_pds_dn5 * assign62680_e97203) + (locals.var_pds * (locals.var_t9_dn5 + locals.var_t8_dn5))), ((locals.var_pds_dn6 * assign62680_e97203) + (locals.var_pds * (locals.var_t9_dn6 + locals.var_t8_dn6))), ((locals.var_pds_dn7 * assign62680_e97203) + (locals.var_pds * (locals.var_t9_dn7 + locals.var_t8_dn7))), ((locals.var_pds_dn8 * assign62680_e97203) + (locals.var_pds * (locals.var_t9_dn8 + locals.var_t8_dn8))), ((locals.var_pds_dn9 * assign62680_e97203) + (locals.var_pds * (locals.var_t9_dn9 + locals.var_t8_dn9))), ((locals.var_pds_dn10 * assign62680_e97203) + (locals.var_pds * (locals.var_t9_dn10 + locals.var_t8_dn10))), ((locals.var_pds_dn11 * assign62680_e97203) + (locals.var_pds * (locals.var_t9_dn11 + locals.var_t8_dn11))), ((locals.var_pds_dn14 * assign62680_e97203) + (locals.var_pds * (locals.var_t9_dn14 + locals.var_t8_dn14))),)
    } else {
        (locals.var_idd1, locals.var_idd1_dn0, locals.var_idd1_dn2, locals.var_idd1_dn4, locals.var_idd1_dn5, locals.var_idd1_dn6, locals.var_idd1_dn7, locals.var_idd1_dn8, locals.var_idd1_dn9, locals.var_idd1_dn10, locals.var_idd1_dn11, locals.var_idd1_dn14,)
    }
};
        locals.var_idd1 = assign62680_e97206;
        locals.var_idd1_dn0 = assign62680_e97206_d_n0;
        locals.var_idd1_dn2 = assign62680_e97206_d_n2;
        locals.var_idd1_dn4 = assign62680_e97206_d_n4;
        locals.var_idd1_dn5 = assign62680_e97206_d_n5;
        locals.var_idd1_dn6 = assign62680_e97206_d_n6;
        locals.var_idd1_dn7 = assign62680_e97206_d_n7;
        locals.var_idd1_dn8 = assign62680_e97206_d_n8;
        locals.var_idd1_dn9 = assign62680_e97206_d_n9;
        locals.var_idd1_dn10 = assign62680_e97206_d_n10;
        locals.var_idd1_dn11 = assign62680_e97206_d_n11;
        locals.var_idd1_dn14 = assign62680_e97206_d_n14;
        locals.var_idd1_rv = 0.0;

        let (assign62690_e97219, assign62690_e97219_d_n0, assign62690_e97219_d_n2, assign62690_e97219_d_n4, assign62690_e97219_d_n5, assign62690_e97219_d_n6, assign62690_e97219_d_n7, assign62690_e97219_d_n8, assign62690_e97219_d_n9, assign62690_e97219_d_n10, assign62690_e97219_d_n11, assign62690_e97219_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign62690_e97215: f64 = (locals.var_betawl * locals.var_idd1);
        let assign62690_e97217: f64 = (assign62690_e97215 * locals.var_mu);
        (assign62690_e97217, ((((locals.var_betawl_dn0 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn0)) * locals.var_mu) + (assign62690_e97215 * locals.var_mu_dn0)), ((((locals.var_betawl_dn2 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn2)) * locals.var_mu) + (assign62690_e97215 * locals.var_mu_dn2)), ((((locals.var_betawl_dn4 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn4)) * locals.var_mu) + (assign62690_e97215 * locals.var_mu_dn4)), ((((locals.var_betawl_dn5 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn5)) * locals.var_mu) + (assign62690_e97215 * locals.var_mu_dn5)), ((((locals.var_betawl_dn6 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn6)) * locals.var_mu) + (assign62690_e97215 * locals.var_mu_dn6)), ((((locals.var_betawl_dn7 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn7)) * locals.var_mu) + (assign62690_e97215 * locals.var_mu_dn7)), ((((locals.var_betawl_dn8 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn8)) * locals.var_mu) + (assign62690_e97215 * locals.var_mu_dn8)), ((((locals.var_betawl_dn9 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn9)) * locals.var_mu) + (assign62690_e97215 * locals.var_mu_dn9)), ((((locals.var_betawl_dn10 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn10)) * locals.var_mu) + (assign62690_e97215 * locals.var_mu_dn10)), ((((locals.var_betawl_dn11 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn11)) * locals.var_mu) + (assign62690_e97215 * locals.var_mu_dn11)), ((((locals.var_betawl_dn14 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn14)) * locals.var_mu) + (assign62690_e97215 * locals.var_mu_dn14)),)
    } else {
        (locals.var_idspt0, locals.var_idspt0_dn0, locals.var_idspt0_dn2, locals.var_idspt0_dn4, locals.var_idspt0_dn5, locals.var_idspt0_dn6, locals.var_idspt0_dn7, locals.var_idspt0_dn8, locals.var_idspt0_dn9, locals.var_idspt0_dn10, locals.var_idspt0_dn11, locals.var_idspt0_dn14,)
    }
};
        locals.var_idspt0 = assign62690_e97219;
        locals.var_idspt0_dn0 = assign62690_e97219_d_n0;
        locals.var_idspt0_dn2 = assign62690_e97219_d_n2;
        locals.var_idspt0_dn4 = assign62690_e97219_d_n4;
        locals.var_idspt0_dn5 = assign62690_e97219_d_n5;
        locals.var_idspt0_dn6 = assign62690_e97219_d_n6;
        locals.var_idspt0_dn7 = assign62690_e97219_d_n7;
        locals.var_idspt0_dn8 = assign62690_e97219_d_n8;
        locals.var_idspt0_dn9 = assign62690_e97219_d_n9;
        locals.var_idspt0_dn10 = assign62690_e97219_d_n10;
        locals.var_idspt0_dn11 = assign62690_e97219_d_n11;
        locals.var_idspt0_dn14 = assign62690_e97219_d_n14;
        locals.var_idspt0_rv = 0.0;

        let (assign62700_e97230, assign62700_e97230_d_n0, assign62700_e97230_d_n2, assign62700_e97230_d_n4, assign62700_e97230_d_n5, assign62700_e97230_d_n6, assign62700_e97230_d_n7, assign62700_e97230_d_n8, assign62700_e97230_d_n9, assign62700_e97230_d_n10, assign62700_e97230_d_n11, assign62700_e97230_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign62700_e97228: f64 = (locals.var_ids0 + locals.var_idspt0);
        (assign62700_e97228, (locals.var_ids0_dn0 + locals.var_idspt0_dn0), (locals.var_ids0_dn2 + locals.var_idspt0_dn2), (locals.var_ids0_dn4 + locals.var_idspt0_dn4), (locals.var_ids0_dn5 + locals.var_idspt0_dn5), (locals.var_ids0_dn6 + locals.var_idspt0_dn6), (locals.var_ids0_dn7 + locals.var_idspt0_dn7), (locals.var_ids0_dn8 + locals.var_idspt0_dn8), (locals.var_ids0_dn9 + locals.var_idspt0_dn9), (locals.var_ids0_dn10 + locals.var_idspt0_dn10), (locals.var_ids0_dn11 + locals.var_idspt0_dn11), (locals.var_ids0_dn14 + locals.var_idspt0_dn14),)
    } else {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn4, locals.var_ids0_dn5, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn8, locals.var_ids0_dn9, locals.var_ids0_dn10, locals.var_ids0_dn11, locals.var_ids0_dn14,)
    }
};
        locals.var_ids0 = assign62700_e97230;
        locals.var_ids0_dn0 = assign62700_e97230_d_n0;
        locals.var_ids0_dn2 = assign62700_e97230_d_n2;
        locals.var_ids0_dn4 = assign62700_e97230_d_n4;
        locals.var_ids0_dn5 = assign62700_e97230_d_n5;
        locals.var_ids0_dn6 = assign62700_e97230_d_n6;
        locals.var_ids0_dn7 = assign62700_e97230_d_n7;
        locals.var_ids0_dn8 = assign62700_e97230_d_n8;
        locals.var_ids0_dn9 = assign62700_e97230_d_n9;
        locals.var_ids0_dn10 = assign62700_e97230_d_n10;
        locals.var_ids0_dn11 = assign62700_e97230_d_n11;
        locals.var_ids0_dn14 = assign62700_e97230_d_n14;
        locals.var_ids0_rv = 0.0;

        let (assign62710_e97240, assign62710_e97240_d_n0, assign62710_e97240_d_n2, assign62710_e97240_d_n4, assign62710_e97240_d_n5, assign62710_e97240_d_n6, assign62710_e97240_d_n7, assign62710_e97240_d_n8, assign62710_e97240_d_n9, assign62710_e97240_d_n10, assign62710_e97240_d_n11, assign62710_e97240_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1502 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_idspt0, locals.var_idspt0_dn0, locals.var_idspt0_dn2, locals.var_idspt0_dn4, locals.var_idspt0_dn5, locals.var_idspt0_dn6, locals.var_idspt0_dn7, locals.var_idspt0_dn8, locals.var_idspt0_dn9, locals.var_idspt0_dn10, locals.var_idspt0_dn11, locals.var_idspt0_dn14,)
    }
};
        locals.var_idspt0 = assign62710_e97240;
        locals.var_idspt0_dn0 = assign62710_e97240_d_n0;
        locals.var_idspt0_dn2 = assign62710_e97240_d_n2;
        locals.var_idspt0_dn4 = assign62710_e97240_d_n4;
        locals.var_idspt0_dn5 = assign62710_e97240_d_n5;
        locals.var_idspt0_dn6 = assign62710_e97240_d_n6;
        locals.var_idspt0_dn7 = assign62710_e97240_d_n7;
        locals.var_idspt0_dn8 = assign62710_e97240_d_n8;
        locals.var_idspt0_dn9 = assign62710_e97240_d_n9;
        locals.var_idspt0_dn10 = assign62710_e97240_d_n10;
        locals.var_idspt0_dn11 = assign62710_e97240_d_n11;
        locals.var_idspt0_dn14 = assign62710_e97240_d_n14;
        locals.var_idspt0_rv = 0.0;

        let assign62720_e97247: f64 = if ((locals.var_flg_rsrd == 2.0) || (locals.var_flg_rsrd == 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard1503 = assign62720_e97247;
        locals.var_guard1503_rv = 0.0;

        let assign62730_e97250: f64 = if p.p296 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1504 = assign62730_e97250;
        locals.var_guard1504_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_234(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign62740_e97261, assign62740_e97261_d_n0, assign62740_e97261_d_n2, assign62740_e97261_d_n4, assign62740_e97261_d_n5, assign62740_e97261_d_n6, assign62740_e97261_d_n7, assign62740_e97261_d_n8, assign62740_e97261_d_n9, assign62740_e97261_d_n10, assign62740_e97261_d_n11, assign62740_e97261_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1504 != 0.0)) {
        (locals.var_rd23e, locals.var_rd23e_dn0, locals.var_rd23e_dn2, locals.var_rd23e_dn4, locals.var_rd23e_dn5, locals.var_rd23e_dn6, locals.var_rd23e_dn7, locals.var_rd23e_dn8, locals.var_rd23e_dn9, locals.var_rd23e_dn10, locals.var_rd23e_dn11, locals.var_rd23e_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign62740_e97261;
        locals.var_t4_dn0 = assign62740_e97261_d_n0;
        locals.var_t4_dn2 = assign62740_e97261_d_n2;
        locals.var_t4_dn4 = assign62740_e97261_d_n4;
        locals.var_t4_dn5 = assign62740_e97261_d_n5;
        locals.var_t4_dn6 = assign62740_e97261_d_n6;
        locals.var_t4_dn7 = assign62740_e97261_d_n7;
        locals.var_t4_dn8 = assign62740_e97261_d_n8;
        locals.var_t4_dn9 = assign62740_e97261_d_n9;
        locals.var_t4_dn10 = assign62740_e97261_d_n10;
        locals.var_t4_dn11 = assign62740_e97261_d_n11;
        locals.var_t4_dn14 = assign62740_e97261_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign62750_e97276, assign62750_e97276_d_n0, assign62750_e97276_d_n2, assign62750_e97276_d_n4, assign62750_e97276_d_n5, assign62750_e97276_d_n6, assign62750_e97276_d_n7, assign62750_e97276_d_n8, assign62750_e97276_d_n9, assign62750_e97276_d_n10, assign62750_e97276_d_n11, assign62750_e97276_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1504 != 0.0)) {
        let assign62750_e97273: f64 = (locals.var_vgse - p.p300);
        let assign62750_e97274: f64 = (locals.var_uc_rd24 * assign62750_e97273);
        (assign62750_e97274, (locals.var_uc_rd24 * locals.var_vgse_dn0), (locals.var_uc_rd24 * locals.var_vgse_dn2), 0.0, 0.0, 0.0, (locals.var_uc_rd24 * locals.var_vgse_dn7), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign62750_e97276;
        locals.var_t1_dn0 = assign62750_e97276_d_n0;
        locals.var_t1_dn2 = assign62750_e97276_d_n2;
        locals.var_t1_dn4 = assign62750_e97276_d_n4;
        locals.var_t1_dn5 = assign62750_e97276_d_n5;
        locals.var_t1_dn6 = assign62750_e97276_d_n6;
        locals.var_t1_dn7 = assign62750_e97276_d_n7;
        locals.var_t1_dn8 = assign62750_e97276_d_n8;
        locals.var_t1_dn9 = assign62750_e97276_d_n9;
        locals.var_t1_dn10 = assign62750_e97276_d_n10;
        locals.var_t1_dn11 = assign62750_e97276_d_n11;
        locals.var_t1_dn14 = assign62750_e97276_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign62760_e97293, assign62760_e97293_d_n0, assign62760_e97293_d_n2, assign62760_e97293_d_n4, assign62760_e97293_d_n5, assign62760_e97293_d_n6, assign62760_e97293_d_n7, assign62760_e97293_d_n8, assign62760_e97293_d_n9, assign62760_e97293_d_n10, assign62760_e97293_d_n11, assign62760_e97293_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1504 != 0.0)) {
        let assign62760_e97287: f64 = (locals.var_t1 - locals.var_t4);
        let assign62760_e97290: f64 = (0.01 * 0.01);
        let assign62760_e97291: f64 = (assign62760_e97287 - assign62760_e97290);
        (assign62760_e97291, (locals.var_t1_dn0 - locals.var_t4_dn0), (locals.var_t1_dn2 - locals.var_t4_dn2), (locals.var_t1_dn4 - locals.var_t4_dn4), (locals.var_t1_dn5 - locals.var_t4_dn5), (locals.var_t1_dn6 - locals.var_t4_dn6), (locals.var_t1_dn7 - locals.var_t4_dn7), (locals.var_t1_dn8 - locals.var_t4_dn8), (locals.var_t1_dn9 - locals.var_t4_dn9), (locals.var_t1_dn10 - locals.var_t4_dn10), (locals.var_t1_dn11 - locals.var_t4_dn11), (locals.var_t1_dn14 - locals.var_t4_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign62760_e97293;
        locals.var_tmf1_dn0 = assign62760_e97293_d_n0;
        locals.var_tmf1_dn2 = assign62760_e97293_d_n2;
        locals.var_tmf1_dn4 = assign62760_e97293_d_n4;
        locals.var_tmf1_dn5 = assign62760_e97293_d_n5;
        locals.var_tmf1_dn6 = assign62760_e97293_d_n6;
        locals.var_tmf1_dn7 = assign62760_e97293_d_n7;
        locals.var_tmf1_dn8 = assign62760_e97293_d_n8;
        locals.var_tmf1_dn9 = assign62760_e97293_d_n9;
        locals.var_tmf1_dn10 = assign62760_e97293_d_n10;
        locals.var_tmf1_dn11 = assign62760_e97293_d_n11;
        locals.var_tmf1_dn14 = assign62760_e97293_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign62770_e97310, assign62770_e97310_d_n0, assign62770_e97310_d_n2, assign62770_e97310_d_n4, assign62770_e97310_d_n5, assign62770_e97310_d_n6, assign62770_e97310_d_n7, assign62770_e97310_d_n8, assign62770_e97310_d_n9, assign62770_e97310_d_n10, assign62770_e97310_d_n11, assign62770_e97310_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1504 != 0.0)) {
        let assign62770_e97304: f64 = (4.0 * locals.var_t4);
        let assign62770_e97307: f64 = (0.01 * 0.01);
        let assign62770_e97308: f64 = (assign62770_e97304 * assign62770_e97307);
        (assign62770_e97308, ((4.0 * locals.var_t4_dn0) * assign62770_e97307), ((4.0 * locals.var_t4_dn2) * assign62770_e97307), ((4.0 * locals.var_t4_dn4) * assign62770_e97307), ((4.0 * locals.var_t4_dn5) * assign62770_e97307), ((4.0 * locals.var_t4_dn6) * assign62770_e97307), ((4.0 * locals.var_t4_dn7) * assign62770_e97307), ((4.0 * locals.var_t4_dn8) * assign62770_e97307), ((4.0 * locals.var_t4_dn9) * assign62770_e97307), ((4.0 * locals.var_t4_dn10) * assign62770_e97307), ((4.0 * locals.var_t4_dn11) * assign62770_e97307), ((4.0 * locals.var_t4_dn14) * assign62770_e97307),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign62770_e97310;
        locals.var_tmf2_dn0 = assign62770_e97310_d_n0;
        locals.var_tmf2_dn2 = assign62770_e97310_d_n2;
        locals.var_tmf2_dn4 = assign62770_e97310_d_n4;
        locals.var_tmf2_dn5 = assign62770_e97310_d_n5;
        locals.var_tmf2_dn6 = assign62770_e97310_d_n6;
        locals.var_tmf2_dn7 = assign62770_e97310_d_n7;
        locals.var_tmf2_dn8 = assign62770_e97310_d_n8;
        locals.var_tmf2_dn9 = assign62770_e97310_d_n9;
        locals.var_tmf2_dn10 = assign62770_e97310_d_n10;
        locals.var_tmf2_dn11 = assign62770_e97310_d_n11;
        locals.var_tmf2_dn14 = assign62770_e97310_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign62780_e97327, assign62780_e97327_d_n0, assign62780_e97327_d_n2, assign62780_e97327_d_n4, assign62780_e97327_d_n5, assign62780_e97327_d_n6, assign62780_e97327_d_n7, assign62780_e97327_d_n8, assign62780_e97327_d_n9, assign62780_e97327_d_n10, assign62780_e97327_d_n11, assign62780_e97327_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1504 != 0.0)) {
        let (assign62780_e97325, assign62780_e97325_d_n0, assign62780_e97325_d_n2, assign62780_e97325_d_n4, assign62780_e97325_d_n5, assign62780_e97325_d_n6, assign62780_e97325_d_n7, assign62780_e97325_d_n8, assign62780_e97325_d_n9, assign62780_e97325_d_n10, assign62780_e97325_d_n11, assign62780_e97325_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign62780_e97324: f64 = (-locals.var_tmf2);
                (assign62780_e97324, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign62780_e97325, assign62780_e97325_d_n0, assign62780_e97325_d_n2, assign62780_e97325_d_n4, assign62780_e97325_d_n5, assign62780_e97325_d_n6, assign62780_e97325_d_n7, assign62780_e97325_d_n8, assign62780_e97325_d_n9, assign62780_e97325_d_n10, assign62780_e97325_d_n11, assign62780_e97325_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign62780_e97327;
        locals.var_tmf2_dn0 = assign62780_e97327_d_n0;
        locals.var_tmf2_dn2 = assign62780_e97327_d_n2;
        locals.var_tmf2_dn4 = assign62780_e97327_d_n4;
        locals.var_tmf2_dn5 = assign62780_e97327_d_n5;
        locals.var_tmf2_dn6 = assign62780_e97327_d_n6;
        locals.var_tmf2_dn7 = assign62780_e97327_d_n7;
        locals.var_tmf2_dn8 = assign62780_e97327_d_n8;
        locals.var_tmf2_dn9 = assign62780_e97327_d_n9;
        locals.var_tmf2_dn10 = assign62780_e97327_d_n10;
        locals.var_tmf2_dn11 = assign62780_e97327_d_n11;
        locals.var_tmf2_dn14 = assign62780_e97327_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign62790_e97343, assign62790_e97343_d_n0, assign62790_e97343_d_n2, assign62790_e97343_d_n4, assign62790_e97343_d_n5, assign62790_e97343_d_n6, assign62790_e97343_d_n7, assign62790_e97343_d_n8, assign62790_e97343_d_n9, assign62790_e97343_d_n10, assign62790_e97343_d_n11, assign62790_e97343_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1504 != 0.0)) {
        let assign62790_e97338: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign62790_e97340: f64 = (assign62790_e97338 + locals.var_tmf2);
        let assign62790_e97341: f64 = (assign62790_e97340).sqrt();
        (assign62790_e97341, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign62790_e97341)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign62790_e97341)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign62790_e97341)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign62790_e97341)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign62790_e97341)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign62790_e97341)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign62790_e97341)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign62790_e97341)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign62790_e97341)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign62790_e97341)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign62790_e97341)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign62790_e97343;
        locals.var_tmf2_dn0 = assign62790_e97343_d_n0;
        locals.var_tmf2_dn2 = assign62790_e97343_d_n2;
        locals.var_tmf2_dn4 = assign62790_e97343_d_n4;
        locals.var_tmf2_dn5 = assign62790_e97343_d_n5;
        locals.var_tmf2_dn6 = assign62790_e97343_d_n6;
        locals.var_tmf2_dn7 = assign62790_e97343_d_n7;
        locals.var_tmf2_dn8 = assign62790_e97343_d_n8;
        locals.var_tmf2_dn9 = assign62790_e97343_d_n9;
        locals.var_tmf2_dn10 = assign62790_e97343_d_n10;
        locals.var_tmf2_dn11 = assign62790_e97343_d_n11;
        locals.var_tmf2_dn14 = assign62790_e97343_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign62800_e97360, assign62800_e97360_d_n0, assign62800_e97360_d_n2, assign62800_e97360_d_n4, assign62800_e97360_d_n5, assign62800_e97360_d_n6, assign62800_e97360_d_n7, assign62800_e97360_d_n8, assign62800_e97360_d_n9, assign62800_e97360_d_n10, assign62800_e97360_d_n11, assign62800_e97360_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1504 != 0.0)) {
        let assign62800_e97356: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign62800_e97357: f64 = (1.0 + assign62800_e97356);
        let assign62800_e97358: f64 = (0.5 * assign62800_e97357);
        (assign62800_e97358, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign62800_e97360;
        locals.var_t0_dn0 = assign62800_e97360_d_n0;
        locals.var_t0_dn2 = assign62800_e97360_d_n2;
        locals.var_t0_dn4 = assign62800_e97360_d_n4;
        locals.var_t0_dn5 = assign62800_e97360_d_n5;
        locals.var_t0_dn6 = assign62800_e97360_d_n6;
        locals.var_t0_dn7 = assign62800_e97360_d_n7;
        locals.var_t0_dn8 = assign62800_e97360_d_n8;
        locals.var_t0_dn9 = assign62800_e97360_d_n9;
        locals.var_t0_dn10 = assign62800_e97360_d_n10;
        locals.var_t0_dn11 = assign62800_e97360_d_n11;
        locals.var_t0_dn14 = assign62800_e97360_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign62810_e97377, assign62810_e97377_d_n0, assign62810_e97377_d_n2, assign62810_e97377_d_n4, assign62810_e97377_d_n5, assign62810_e97377_d_n6, assign62810_e97377_d_n7, assign62810_e97377_d_n8, assign62810_e97377_d_n9, assign62810_e97377_d_n10, assign62810_e97377_d_n11, assign62810_e97377_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1504 != 0.0)) {
        let assign62810_e97373: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign62810_e97374: f64 = (0.5 * assign62810_e97373);
        let assign62810_e97375: f64 = (locals.var_t4 + assign62810_e97374);
        (assign62810_e97375, (locals.var_t4_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t4_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t4_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t4_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t4_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t4_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t4_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t4_dn9 + (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t4_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t4_dn11 + (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t4_dn14 + (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign62810_e97377;
        locals.var_t2_dn0 = assign62810_e97377_d_n0;
        locals.var_t2_dn2 = assign62810_e97377_d_n2;
        locals.var_t2_dn4 = assign62810_e97377_d_n4;
        locals.var_t2_dn5 = assign62810_e97377_d_n5;
        locals.var_t2_dn6 = assign62810_e97377_d_n6;
        locals.var_t2_dn7 = assign62810_e97377_d_n7;
        locals.var_t2_dn8 = assign62810_e97377_d_n8;
        locals.var_t2_dn9 = assign62810_e97377_d_n9;
        locals.var_t2_dn10 = assign62810_e97377_d_n10;
        locals.var_t2_dn11 = assign62810_e97377_d_n11;
        locals.var_t2_dn14 = assign62810_e97377_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign62820_e97392, assign62820_e97392_d_n0, assign62820_e97392_d_n2, assign62820_e97392_d_n4, assign62820_e97392_d_n5, assign62820_e97392_d_n6, assign62820_e97392_d_n7, assign62820_e97392_d_n8, assign62820_e97392_d_n9, assign62820_e97392_d_n10, assign62820_e97392_d_n11, assign62820_e97392_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1504 != 0.0)) {
        let assign62820_e97389: f64 = (p.p296 + 1.0);
        let assign62820_e97390: f64 = (locals.var_t4 * assign62820_e97389);
        (assign62820_e97390, (locals.var_t4_dn0 * assign62820_e97389), (locals.var_t4_dn2 * assign62820_e97389), (locals.var_t4_dn4 * assign62820_e97389), (locals.var_t4_dn5 * assign62820_e97389), (locals.var_t4_dn6 * assign62820_e97389), (locals.var_t4_dn7 * assign62820_e97389), (locals.var_t4_dn8 * assign62820_e97389), (locals.var_t4_dn9 * assign62820_e97389), (locals.var_t4_dn10 * assign62820_e97389), (locals.var_t4_dn11 * assign62820_e97389), (locals.var_t4_dn14 * assign62820_e97389),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign62820_e97392;
        locals.var_t3_dn0 = assign62820_e97392_d_n0;
        locals.var_t3_dn2 = assign62820_e97392_d_n2;
        locals.var_t3_dn4 = assign62820_e97392_d_n4;
        locals.var_t3_dn5 = assign62820_e97392_d_n5;
        locals.var_t3_dn6 = assign62820_e97392_d_n6;
        locals.var_t3_dn7 = assign62820_e97392_d_n7;
        locals.var_t3_dn8 = assign62820_e97392_d_n8;
        locals.var_t3_dn9 = assign62820_e97392_d_n9;
        locals.var_t3_dn10 = assign62820_e97392_d_n10;
        locals.var_t3_dn11 = assign62820_e97392_d_n11;
        locals.var_t3_dn14 = assign62820_e97392_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign62830_e97409, assign62830_e97409_d_n0, assign62830_e97409_d_n2, assign62830_e97409_d_n4, assign62830_e97409_d_n5, assign62830_e97409_d_n6, assign62830_e97409_d_n7, assign62830_e97409_d_n8, assign62830_e97409_d_n9, assign62830_e97409_d_n10, assign62830_e97409_d_n11, assign62830_e97409_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1504 != 0.0)) {
        let assign62830_e97403: f64 = (locals.var_t3 - locals.var_t2);
        let assign62830_e97406: f64 = (0.01 * 0.01);
        let assign62830_e97407: f64 = (assign62830_e97403 - assign62830_e97406);
        (assign62830_e97407, (locals.var_t3_dn0 - locals.var_t2_dn0), (locals.var_t3_dn2 - locals.var_t2_dn2), (locals.var_t3_dn4 - locals.var_t2_dn4), (locals.var_t3_dn5 - locals.var_t2_dn5), (locals.var_t3_dn6 - locals.var_t2_dn6), (locals.var_t3_dn7 - locals.var_t2_dn7), (locals.var_t3_dn8 - locals.var_t2_dn8), (locals.var_t3_dn9 - locals.var_t2_dn9), (locals.var_t3_dn10 - locals.var_t2_dn10), (locals.var_t3_dn11 - locals.var_t2_dn11), (locals.var_t3_dn14 - locals.var_t2_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign62830_e97409;
        locals.var_tmf1_dn0 = assign62830_e97409_d_n0;
        locals.var_tmf1_dn2 = assign62830_e97409_d_n2;
        locals.var_tmf1_dn4 = assign62830_e97409_d_n4;
        locals.var_tmf1_dn5 = assign62830_e97409_d_n5;
        locals.var_tmf1_dn6 = assign62830_e97409_d_n6;
        locals.var_tmf1_dn7 = assign62830_e97409_d_n7;
        locals.var_tmf1_dn8 = assign62830_e97409_d_n8;
        locals.var_tmf1_dn9 = assign62830_e97409_d_n9;
        locals.var_tmf1_dn10 = assign62830_e97409_d_n10;
        locals.var_tmf1_dn11 = assign62830_e97409_d_n11;
        locals.var_tmf1_dn14 = assign62830_e97409_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign62840_e97426, assign62840_e97426_d_n0, assign62840_e97426_d_n2, assign62840_e97426_d_n4, assign62840_e97426_d_n5, assign62840_e97426_d_n6, assign62840_e97426_d_n7, assign62840_e97426_d_n8, assign62840_e97426_d_n9, assign62840_e97426_d_n10, assign62840_e97426_d_n11, assign62840_e97426_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1504 != 0.0)) {
        let assign62840_e97420: f64 = (4.0 * locals.var_t3);
        let assign62840_e97423: f64 = (0.01 * 0.01);
        let assign62840_e97424: f64 = (assign62840_e97420 * assign62840_e97423);
        (assign62840_e97424, ((4.0 * locals.var_t3_dn0) * assign62840_e97423), ((4.0 * locals.var_t3_dn2) * assign62840_e97423), ((4.0 * locals.var_t3_dn4) * assign62840_e97423), ((4.0 * locals.var_t3_dn5) * assign62840_e97423), ((4.0 * locals.var_t3_dn6) * assign62840_e97423), ((4.0 * locals.var_t3_dn7) * assign62840_e97423), ((4.0 * locals.var_t3_dn8) * assign62840_e97423), ((4.0 * locals.var_t3_dn9) * assign62840_e97423), ((4.0 * locals.var_t3_dn10) * assign62840_e97423), ((4.0 * locals.var_t3_dn11) * assign62840_e97423), ((4.0 * locals.var_t3_dn14) * assign62840_e97423),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign62840_e97426;
        locals.var_tmf2_dn0 = assign62840_e97426_d_n0;
        locals.var_tmf2_dn2 = assign62840_e97426_d_n2;
        locals.var_tmf2_dn4 = assign62840_e97426_d_n4;
        locals.var_tmf2_dn5 = assign62840_e97426_d_n5;
        locals.var_tmf2_dn6 = assign62840_e97426_d_n6;
        locals.var_tmf2_dn7 = assign62840_e97426_d_n7;
        locals.var_tmf2_dn8 = assign62840_e97426_d_n8;
        locals.var_tmf2_dn9 = assign62840_e97426_d_n9;
        locals.var_tmf2_dn10 = assign62840_e97426_d_n10;
        locals.var_tmf2_dn11 = assign62840_e97426_d_n11;
        locals.var_tmf2_dn14 = assign62840_e97426_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign62850_e97443, assign62850_e97443_d_n0, assign62850_e97443_d_n2, assign62850_e97443_d_n4, assign62850_e97443_d_n5, assign62850_e97443_d_n6, assign62850_e97443_d_n7, assign62850_e97443_d_n8, assign62850_e97443_d_n9, assign62850_e97443_d_n10, assign62850_e97443_d_n11, assign62850_e97443_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1504 != 0.0)) {
        let (assign62850_e97441, assign62850_e97441_d_n0, assign62850_e97441_d_n2, assign62850_e97441_d_n4, assign62850_e97441_d_n5, assign62850_e97441_d_n6, assign62850_e97441_d_n7, assign62850_e97441_d_n8, assign62850_e97441_d_n9, assign62850_e97441_d_n10, assign62850_e97441_d_n11, assign62850_e97441_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign62850_e97440: f64 = (-locals.var_tmf2);
                (assign62850_e97440, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign62850_e97441, assign62850_e97441_d_n0, assign62850_e97441_d_n2, assign62850_e97441_d_n4, assign62850_e97441_d_n5, assign62850_e97441_d_n6, assign62850_e97441_d_n7, assign62850_e97441_d_n8, assign62850_e97441_d_n9, assign62850_e97441_d_n10, assign62850_e97441_d_n11, assign62850_e97441_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign62850_e97443;
        locals.var_tmf2_dn0 = assign62850_e97443_d_n0;
        locals.var_tmf2_dn2 = assign62850_e97443_d_n2;
        locals.var_tmf2_dn4 = assign62850_e97443_d_n4;
        locals.var_tmf2_dn5 = assign62850_e97443_d_n5;
        locals.var_tmf2_dn6 = assign62850_e97443_d_n6;
        locals.var_tmf2_dn7 = assign62850_e97443_d_n7;
        locals.var_tmf2_dn8 = assign62850_e97443_d_n8;
        locals.var_tmf2_dn9 = assign62850_e97443_d_n9;
        locals.var_tmf2_dn10 = assign62850_e97443_d_n10;
        locals.var_tmf2_dn11 = assign62850_e97443_d_n11;
        locals.var_tmf2_dn14 = assign62850_e97443_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign62860_e97459, assign62860_e97459_d_n0, assign62860_e97459_d_n2, assign62860_e97459_d_n4, assign62860_e97459_d_n5, assign62860_e97459_d_n6, assign62860_e97459_d_n7, assign62860_e97459_d_n8, assign62860_e97459_d_n9, assign62860_e97459_d_n10, assign62860_e97459_d_n11, assign62860_e97459_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1504 != 0.0)) {
        let assign62860_e97454: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign62860_e97456: f64 = (assign62860_e97454 + locals.var_tmf2);
        let assign62860_e97457: f64 = (assign62860_e97456).sqrt();
        (assign62860_e97457, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign62860_e97457)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign62860_e97457)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign62860_e97457)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign62860_e97457)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign62860_e97457)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign62860_e97457)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign62860_e97457)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign62860_e97457)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign62860_e97457)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign62860_e97457)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign62860_e97457)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign62860_e97459;
        locals.var_tmf2_dn0 = assign62860_e97459_d_n0;
        locals.var_tmf2_dn2 = assign62860_e97459_d_n2;
        locals.var_tmf2_dn4 = assign62860_e97459_d_n4;
        locals.var_tmf2_dn5 = assign62860_e97459_d_n5;
        locals.var_tmf2_dn6 = assign62860_e97459_d_n6;
        locals.var_tmf2_dn7 = assign62860_e97459_d_n7;
        locals.var_tmf2_dn8 = assign62860_e97459_d_n8;
        locals.var_tmf2_dn9 = assign62860_e97459_d_n9;
        locals.var_tmf2_dn10 = assign62860_e97459_d_n10;
        locals.var_tmf2_dn11 = assign62860_e97459_d_n11;
        locals.var_tmf2_dn14 = assign62860_e97459_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign62870_e97476, assign62870_e97476_d_n0, assign62870_e97476_d_n2, assign62870_e97476_d_n4, assign62870_e97476_d_n5, assign62870_e97476_d_n6, assign62870_e97476_d_n7, assign62870_e97476_d_n8, assign62870_e97476_d_n9, assign62870_e97476_d_n10, assign62870_e97476_d_n11, assign62870_e97476_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1504 != 0.0)) {
        let assign62870_e97472: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign62870_e97473: f64 = (1.0 + assign62870_e97472);
        let assign62870_e97474: f64 = (0.5 * assign62870_e97473);
        (assign62870_e97474, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign62870_e97476;
        locals.var_t0_dn0 = assign62870_e97476_d_n0;
        locals.var_t0_dn2 = assign62870_e97476_d_n2;
        locals.var_t0_dn4 = assign62870_e97476_d_n4;
        locals.var_t0_dn5 = assign62870_e97476_d_n5;
        locals.var_t0_dn6 = assign62870_e97476_d_n6;
        locals.var_t0_dn7 = assign62870_e97476_d_n7;
        locals.var_t0_dn8 = assign62870_e97476_d_n8;
        locals.var_t0_dn9 = assign62870_e97476_d_n9;
        locals.var_t0_dn10 = assign62870_e97476_d_n10;
        locals.var_t0_dn11 = assign62870_e97476_d_n11;
        locals.var_t0_dn14 = assign62870_e97476_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign62880_e97493, assign62880_e97493_d_n0, assign62880_e97493_d_n2, assign62880_e97493_d_n4, assign62880_e97493_d_n5, assign62880_e97493_d_n6, assign62880_e97493_d_n7, assign62880_e97493_d_n8, assign62880_e97493_d_n9, assign62880_e97493_d_n10, assign62880_e97493_d_n11, assign62880_e97493_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1504 != 0.0)) {
        let assign62880_e97489: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign62880_e97490: f64 = (0.5 * assign62880_e97489);
        let assign62880_e97491: f64 = (locals.var_t3 - assign62880_e97490);
        (assign62880_e97491, (locals.var_t3_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t3_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t3_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t3_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t3_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t3_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t3_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t3_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t3_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t3_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t3_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign62880_e97493;
        locals.var_t7_dn0 = assign62880_e97493_d_n0;
        locals.var_t7_dn2 = assign62880_e97493_d_n2;
        locals.var_t7_dn4 = assign62880_e97493_d_n4;
        locals.var_t7_dn5 = assign62880_e97493_d_n5;
        locals.var_t7_dn6 = assign62880_e97493_d_n6;
        locals.var_t7_dn7 = assign62880_e97493_d_n7;
        locals.var_t7_dn8 = assign62880_e97493_d_n8;
        locals.var_t7_dn9 = assign62880_e97493_d_n9;
        locals.var_t7_dn10 = assign62880_e97493_d_n10;
        locals.var_t7_dn11 = assign62880_e97493_d_n11;
        locals.var_t7_dn14 = assign62880_e97493_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign62890_e97505, assign62890_e97505_d_n0, assign62890_e97505_d_n2, assign62890_e97505_d_n4, assign62890_e97505_d_n5, assign62890_e97505_d_n6, assign62890_e97505_d_n7, assign62890_e97505_d_n8, assign62890_e97505_d_n9, assign62890_e97505_d_n10, assign62890_e97505_d_n11, assign62890_e97505_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1504 == 0.0)) {
        (locals.var_rd23e, locals.var_rd23e_dn0, locals.var_rd23e_dn2, locals.var_rd23e_dn4, locals.var_rd23e_dn5, locals.var_rd23e_dn6, locals.var_rd23e_dn7, locals.var_rd23e_dn8, locals.var_rd23e_dn9, locals.var_rd23e_dn10, locals.var_rd23e_dn11, locals.var_rd23e_dn14,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign62890_e97505;
        locals.var_t7_dn0 = assign62890_e97505_d_n0;
        locals.var_t7_dn2 = assign62890_e97505_d_n2;
        locals.var_t7_dn4 = assign62890_e97505_d_n4;
        locals.var_t7_dn5 = assign62890_e97505_d_n5;
        locals.var_t7_dn6 = assign62890_e97505_d_n6;
        locals.var_t7_dn7 = assign62890_e97505_d_n7;
        locals.var_t7_dn8 = assign62890_e97505_d_n8;
        locals.var_t7_dn9 = assign62890_e97505_d_n9;
        locals.var_t7_dn10 = assign62890_e97505_d_n10;
        locals.var_t7_dn11 = assign62890_e97505_d_n11;
        locals.var_t7_dn14 = assign62890_e97505_d_n14;
        locals.var_t7_rv = 0.0;

        let assign62900_e97508: f64 = if locals.var_vdse >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1505 = assign62900_e97508;
        locals.var_guard1505_rv = 0.0;

        let (assign62910_e97519, assign62910_e97519_d_n0, assign62910_e97519_d_n2,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        (locals.var_vdse, locals.var_vdse_dn0, locals.var_vdse_dn2,)
    } else {
        (locals.var_vdse_eff, locals.var_vdse_eff_dn0, locals.var_vdse_eff_dn2,)
    }
};
        locals.var_vdse_eff = assign62910_e97519;
        locals.var_vdse_eff_dn0 = assign62910_e97519_d_n0;
        locals.var_vdse_eff_dn2 = assign62910_e97519_d_n2;
        locals.var_vdse_eff_rv = 0.0;

        let (assign62920_e97531, assign62920_e97531_d_n0, assign62920_e97531_d_n2,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1505 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_vdse_eff, locals.var_vdse_eff_dn0, locals.var_vdse_eff_dn2,)
    }
};
        locals.var_vdse_eff = assign62920_e97531;
        locals.var_vdse_eff_dn0 = assign62920_e97531_d_n0;
        locals.var_vdse_eff_dn2 = assign62920_e97531_d_n2;
        locals.var_vdse_eff_rv = 0.0;

        let assign62930_e97535: f64 = (20.0 * 1e-12);
        let assign62930_e97536: f64 = if locals.var_vdse_eff < assign62930_e97535 { 1.0 } else { 0.0 };
        locals.var_guard1506 = assign62930_e97536;
        locals.var_guard1506_rv = 0.0;

        let (assign62940_e97567,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1506 != 0.0)) {
        let assign62940_e97547: f64 = (20.0 + 1.0);
        let assign62940_e97550: f64 = (p.p297 - 1.0);
        let assign62940_e97551: f64 = (assign62940_e97547).powf(assign62940_e97550);
        let assign62940_e97554: f64 = (20.0 + 1.0);
        let assign62940_e97557: f64 = (0.5 * p.p297);
        let assign62940_e97559: f64 = (assign62940_e97557 * 20.0);
        let assign62940_e97560: f64 = (assign62940_e97554 - assign62940_e97559);
        let assign62940_e97561: f64 = (assign62940_e97551 * assign62940_e97560);
        let assign62940_e97564: f64 = (1e-12_f64).powf(p.p297);
        let assign62940_e97565: f64 = (assign62940_e97561 * assign62940_e97564);
        (assign62940_e97565,)
    } else {
        (locals.var_ra_alpha,)
    }
};
        locals.var_ra_alpha = assign62940_e97567;
        locals.var_ra_alpha_rv = 0.0;

        let (assign62950_e97596,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1506 != 0.0)) {
        let assign62950_e97578: f64 = (0.5 * p.p297);
        let assign62950_e97581: f64 = (20.0 + 1.0);
        let assign62950_e97584: f64 = (p.p297 - 1.0);
        let assign62950_e97585: f64 = (assign62950_e97581).powf(assign62950_e97584);
        let assign62950_e97586: f64 = (assign62950_e97578 * assign62950_e97585);
        let assign62950_e97588: f64 = (assign62950_e97586 / 20.0);
        let assign62950_e97592: f64 = (p.p297 - 2.0);
        let assign62950_e97593: f64 = (1e-12_f64).powf(assign62950_e97592);
        let assign62950_e97594: f64 = (assign62950_e97588 * assign62950_e97593);
        (assign62950_e97594,)
    } else {
        (locals.var_ra_beta,)
    }
};
        locals.var_ra_beta = assign62950_e97596;
        locals.var_ra_beta_rv = 0.0;

        let (assign62960_e97613, assign62960_e97613_d_n0, assign62960_e97613_d_n2, assign62960_e97613_d_n4, assign62960_e97613_d_n5, assign62960_e97613_d_n6, assign62960_e97613_d_n7, assign62960_e97613_d_n8, assign62960_e97613_d_n9, assign62960_e97613_d_n10, assign62960_e97613_d_n11, assign62960_e97613_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1506 != 0.0)) {
        let assign62960_e97608: f64 = (locals.var_ra_beta * locals.var_vdse_eff);
        let assign62960_e97610: f64 = (assign62960_e97608 * locals.var_vdse_eff);
        let assign62960_e97611: f64 = (locals.var_ra_alpha + assign62960_e97610);
        (assign62960_e97611, (((locals.var_ra_beta * locals.var_vdse_eff_dn0) * locals.var_vdse_eff) + (assign62960_e97608 * locals.var_vdse_eff_dn0)), (((locals.var_ra_beta * locals.var_vdse_eff_dn2) * locals.var_vdse_eff) + (assign62960_e97608 * locals.var_vdse_eff_dn2)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign62960_e97613;
        locals.var_t1_dn0 = assign62960_e97613_d_n0;
        locals.var_t1_dn2 = assign62960_e97613_d_n2;
        locals.var_t1_dn4 = assign62960_e97613_d_n4;
        locals.var_t1_dn5 = assign62960_e97613_d_n5;
        locals.var_t1_dn6 = assign62960_e97613_d_n6;
        locals.var_t1_dn7 = assign62960_e97613_d_n7;
        locals.var_t1_dn8 = assign62960_e97613_d_n8;
        locals.var_t1_dn9 = assign62960_e97613_d_n9;
        locals.var_t1_dn10 = assign62960_e97613_d_n10;
        locals.var_t1_dn11 = assign62960_e97613_d_n11;
        locals.var_t1_dn14 = assign62960_e97613_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign62970_e97629, assign62970_e97629_d_n0, assign62970_e97629_d_n2, assign62970_e97629_d_n4, assign62970_e97629_d_n5, assign62970_e97629_d_n6, assign62970_e97629_d_n7, assign62970_e97629_d_n8, assign62970_e97629_d_n9, assign62970_e97629_d_n10, assign62970_e97629_d_n11, assign62970_e97629_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1506 == 0.0)) {
        let assign62970_e97625: f64 = (locals.var_vdse_eff + 1e-12);
        let assign62970_e97627: f64 = (assign62970_e97625).powf(p.p297);
        (assign62970_e97627, if 0.0 == 0.0 && ((p.p297) as f64).is_finite() && ((p.p297) as f64).fract() == 0.0 { if p.p297 == 0.0 { 0.0 } else { (p.p297 * ((assign62970_e97625).powf(p.p297 - 1.0) * locals.var_vdse_eff_dn0)) } } else { (assign62970_e97627 * (p.p297 * (locals.var_vdse_eff_dn0 / assign62970_e97625))) }, if 0.0 == 0.0 && ((p.p297) as f64).is_finite() && ((p.p297) as f64).fract() == 0.0 { if p.p297 == 0.0 { 0.0 } else { (p.p297 * ((assign62970_e97625).powf(p.p297 - 1.0) * locals.var_vdse_eff_dn2)) } } else { (assign62970_e97627 * (p.p297 * (locals.var_vdse_eff_dn2 / assign62970_e97625))) }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign62970_e97629;
        locals.var_t1_dn0 = assign62970_e97629_d_n0;
        locals.var_t1_dn2 = assign62970_e97629_d_n2;
        locals.var_t1_dn4 = assign62970_e97629_d_n4;
        locals.var_t1_dn5 = assign62970_e97629_d_n5;
        locals.var_t1_dn6 = assign62970_e97629_d_n6;
        locals.var_t1_dn7 = assign62970_e97629_d_n7;
        locals.var_t1_dn8 = assign62970_e97629_d_n8;
        locals.var_t1_dn9 = assign62970_e97629_d_n9;
        locals.var_t1_dn10 = assign62970_e97629_d_n10;
        locals.var_t1_dn11 = assign62970_e97629_d_n11;
        locals.var_t1_dn14 = assign62970_e97629_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_235(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign62980_e97642, assign62980_e97642_d_n0, assign62980_e97642_d_n2, assign62980_e97642_d_n4, assign62980_e97642_d_n5, assign62980_e97642_d_n6, assign62980_e97642_d_n7, assign62980_e97642_d_n8, assign62980_e97642_d_n9, assign62980_e97642_d_n10, assign62980_e97642_d_n11, assign62980_e97642_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign62980_e97638: f64 = (locals.var_vdse_eff + 1e-12);
        let assign62980_e97640: f64 = (assign62980_e97638).powf(p.p299);
        (assign62980_e97640, if 0.0 == 0.0 && ((p.p299) as f64).is_finite() && ((p.p299) as f64).fract() == 0.0 { if p.p299 == 0.0 { 0.0 } else { (p.p299 * ((assign62980_e97638).powf(p.p299 - 1.0) * locals.var_vdse_eff_dn0)) } } else { (assign62980_e97640 * (p.p299 * (locals.var_vdse_eff_dn0 / assign62980_e97638))) }, if 0.0 == 0.0 && ((p.p299) as f64).is_finite() && ((p.p299) as f64).fract() == 0.0 { if p.p299 == 0.0 { 0.0 } else { (p.p299 * ((assign62980_e97638).powf(p.p299 - 1.0) * locals.var_vdse_eff_dn2)) } } else { (assign62980_e97640 * (p.p299 * (locals.var_vdse_eff_dn2 / assign62980_e97638))) }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign62980_e97642;
        locals.var_t9_dn0 = assign62980_e97642_d_n0;
        locals.var_t9_dn2 = assign62980_e97642_d_n2;
        locals.var_t9_dn4 = assign62980_e97642_d_n4;
        locals.var_t9_dn5 = assign62980_e97642_d_n5;
        locals.var_t9_dn6 = assign62980_e97642_d_n6;
        locals.var_t9_dn7 = assign62980_e97642_d_n7;
        locals.var_t9_dn8 = assign62980_e97642_d_n8;
        locals.var_t9_dn9 = assign62980_e97642_d_n9;
        locals.var_t9_dn10 = assign62980_e97642_d_n10;
        locals.var_t9_dn11 = assign62980_e97642_d_n11;
        locals.var_t9_dn14 = assign62980_e97642_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign62990_e97661, assign62990_e97661_d_n0, assign62990_e97661_d_n2, assign62990_e97661_d_n4, assign62990_e97661_d_n5, assign62990_e97661_d_n6, assign62990_e97661_d_n7, assign62990_e97661_d_n8, assign62990_e97661_d_n9, assign62990_e97661_d_n10, assign62990_e97661_d_n11, assign62990_e97661_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign62990_e97651: f64 = (locals.var_t7 * locals.var_t1);
        let assign62990_e97654: f64 = (locals.var_vbse * locals.var_uc_rd22);
        let assign62990_e97656: f64 = (assign62990_e97654 * locals.var_t9);
        let assign62990_e97657: f64 = (assign62990_e97651 + assign62990_e97656);
        let assign62990_e97659: f64 = (assign62990_e97657 / locals.var_weff_nf);
        (assign62990_e97659, ((((locals.var_t7_dn0 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn0)) + (((locals.var_vbse_dn0 * locals.var_uc_rd22) * locals.var_t9) + (assign62990_e97654 * locals.var_t9_dn0))) / locals.var_weff_nf), ((((locals.var_t7_dn2 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn2)) + (((locals.var_vbse_dn2 * locals.var_uc_rd22) * locals.var_t9) + (assign62990_e97654 * locals.var_t9_dn2))) / locals.var_weff_nf), ((((locals.var_t7_dn4 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn4)) + (assign62990_e97654 * locals.var_t9_dn4)) / locals.var_weff_nf), ((((locals.var_t7_dn5 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn5)) + (assign62990_e97654 * locals.var_t9_dn5)) / locals.var_weff_nf), ((((locals.var_t7_dn6 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn6)) + (assign62990_e97654 * locals.var_t9_dn6)) / locals.var_weff_nf), ((((locals.var_t7_dn7 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn7)) + (assign62990_e97654 * locals.var_t9_dn7)) / locals.var_weff_nf), ((((locals.var_t7_dn8 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn8)) + (assign62990_e97654 * locals.var_t9_dn8)) / locals.var_weff_nf), ((((locals.var_t7_dn9 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn9)) + (((locals.var_vbse_dn9 * locals.var_uc_rd22) * locals.var_t9) + (assign62990_e97654 * locals.var_t9_dn9))) / locals.var_weff_nf), ((((locals.var_t7_dn10 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn10)) + (assign62990_e97654 * locals.var_t9_dn10)) / locals.var_weff_nf), ((((locals.var_t7_dn11 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn11)) + (assign62990_e97654 * locals.var_t9_dn11)) / locals.var_weff_nf), ((((locals.var_t7_dn14 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn14)) + (assign62990_e97654 * locals.var_t9_dn14)) / locals.var_weff_nf),)
    } else {
        (locals.var_ra, locals.var_ra_dn0, locals.var_ra_dn2, locals.var_ra_dn4, locals.var_ra_dn5, locals.var_ra_dn6, locals.var_ra_dn7, locals.var_ra_dn8, locals.var_ra_dn9, locals.var_ra_dn10, locals.var_ra_dn11, locals.var_ra_dn14,)
    }
};
        locals.var_ra = assign62990_e97661;
        locals.var_ra_dn0 = assign62990_e97661_d_n0;
        locals.var_ra_dn2 = assign62990_e97661_d_n2;
        locals.var_ra_dn4 = assign62990_e97661_d_n4;
        locals.var_ra_dn5 = assign62990_e97661_d_n5;
        locals.var_ra_dn6 = assign62990_e97661_d_n6;
        locals.var_ra_dn7 = assign62990_e97661_d_n7;
        locals.var_ra_dn8 = assign62990_e97661_d_n8;
        locals.var_ra_dn9 = assign62990_e97661_d_n9;
        locals.var_ra_dn10 = assign62990_e97661_d_n10;
        locals.var_ra_dn11 = assign62990_e97661_d_n11;
        locals.var_ra_dn14 = assign62990_e97661_d_n14;
        locals.var_ra_rv = 0.0;

        let (assign63000_e97672, assign63000_e97672_d_n0, assign63000_e97672_d_n2, assign63000_e97672_d_n4, assign63000_e97672_d_n5, assign63000_e97672_d_n6, assign63000_e97672_d_n7, assign63000_e97672_d_n8, assign63000_e97672_d_n9, assign63000_e97672_d_n10, assign63000_e97672_d_n11, assign63000_e97672_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63000_e97670: f64 = (locals.var_ra * locals.var_ids0);
        (assign63000_e97670, ((locals.var_ra_dn0 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn0)), ((locals.var_ra_dn2 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn2)), ((locals.var_ra_dn4 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn4)), ((locals.var_ra_dn5 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn5)), ((locals.var_ra_dn6 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn6)), ((locals.var_ra_dn7 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn7)), ((locals.var_ra_dn8 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn8)), ((locals.var_ra_dn9 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn9)), ((locals.var_ra_dn10 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn10)), ((locals.var_ra_dn11 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn11)), ((locals.var_ra_dn14 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign63000_e97672;
        locals.var_t0_dn0 = assign63000_e97672_d_n0;
        locals.var_t0_dn2 = assign63000_e97672_d_n2;
        locals.var_t0_dn4 = assign63000_e97672_d_n4;
        locals.var_t0_dn5 = assign63000_e97672_d_n5;
        locals.var_t0_dn6 = assign63000_e97672_d_n6;
        locals.var_t0_dn7 = assign63000_e97672_d_n7;
        locals.var_t0_dn8 = assign63000_e97672_d_n8;
        locals.var_t0_dn9 = assign63000_e97672_d_n9;
        locals.var_t0_dn10 = assign63000_e97672_d_n10;
        locals.var_t0_dn11 = assign63000_e97672_d_n11;
        locals.var_t0_dn14 = assign63000_e97672_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign63010_e97683, assign63010_e97683_d_n0, assign63010_e97683_d_n2, assign63010_e97683_d_n4, assign63010_e97683_d_n5, assign63010_e97683_d_n6, assign63010_e97683_d_n7, assign63010_e97683_d_n8, assign63010_e97683_d_n9, assign63010_e97683_d_n10, assign63010_e97683_d_n11, assign63010_e97683_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63010_e97681: f64 = (locals.var_vds + 1e-12);
        (assign63010_e97681, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63010_e97683;
        locals.var_t1_dn0 = assign63010_e97683_d_n0;
        locals.var_t1_dn2 = assign63010_e97683_d_n2;
        locals.var_t1_dn4 = assign63010_e97683_d_n4;
        locals.var_t1_dn5 = assign63010_e97683_d_n5;
        locals.var_t1_dn6 = assign63010_e97683_d_n6;
        locals.var_t1_dn7 = assign63010_e97683_d_n7;
        locals.var_t1_dn8 = assign63010_e97683_d_n8;
        locals.var_t1_dn9 = assign63010_e97683_d_n9;
        locals.var_t1_dn10 = assign63010_e97683_d_n10;
        locals.var_t1_dn11 = assign63010_e97683_d_n11;
        locals.var_t1_dn14 = assign63010_e97683_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign63020_e97694, assign63020_e97694_d_n0, assign63020_e97694_d_n2, assign63020_e97694_d_n4, assign63020_e97694_d_n5, assign63020_e97694_d_n6, assign63020_e97694_d_n7, assign63020_e97694_d_n8, assign63020_e97694_d_n9, assign63020_e97694_d_n10, assign63020_e97694_d_n11, assign63020_e97694_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63020_e97692: f64 = (1.0 / locals.var_t1);
        (assign63020_e97692, (-(locals.var_t1_dn0 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn2 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn11 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn14 / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign63020_e97694;
        locals.var_t2_dn0 = assign63020_e97694_d_n0;
        locals.var_t2_dn2 = assign63020_e97694_d_n2;
        locals.var_t2_dn4 = assign63020_e97694_d_n4;
        locals.var_t2_dn5 = assign63020_e97694_d_n5;
        locals.var_t2_dn6 = assign63020_e97694_d_n6;
        locals.var_t2_dn7 = assign63020_e97694_d_n7;
        locals.var_t2_dn8 = assign63020_e97694_d_n8;
        locals.var_t2_dn9 = assign63020_e97694_d_n9;
        locals.var_t2_dn10 = assign63020_e97694_d_n10;
        locals.var_t2_dn11 = assign63020_e97694_d_n11;
        locals.var_t2_dn14 = assign63020_e97694_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign63030_e97707, assign63030_e97707_d_n0, assign63030_e97707_d_n2, assign63030_e97707_d_n4, assign63030_e97707_d_n5, assign63030_e97707_d_n6, assign63030_e97707_d_n7, assign63030_e97707_d_n8, assign63030_e97707_d_n9, assign63030_e97707_d_n10, assign63030_e97707_d_n11, assign63030_e97707_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63030_e97704: f64 = (locals.var_t0 * locals.var_t2);
        let assign63030_e97705: f64 = (1.0 + assign63030_e97704);
        (assign63030_e97705, ((locals.var_t0_dn0 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn0)), ((locals.var_t0_dn2 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn2)), ((locals.var_t0_dn4 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn4)), ((locals.var_t0_dn5 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn5)), ((locals.var_t0_dn6 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn6)), ((locals.var_t0_dn7 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn7)), ((locals.var_t0_dn8 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn8)), ((locals.var_t0_dn9 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn9)), ((locals.var_t0_dn10 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn10)), ((locals.var_t0_dn11 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn11)), ((locals.var_t0_dn14 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign63030_e97707;
        locals.var_t3_dn0 = assign63030_e97707_d_n0;
        locals.var_t3_dn2 = assign63030_e97707_d_n2;
        locals.var_t3_dn4 = assign63030_e97707_d_n4;
        locals.var_t3_dn5 = assign63030_e97707_d_n5;
        locals.var_t3_dn6 = assign63030_e97707_d_n6;
        locals.var_t3_dn7 = assign63030_e97707_d_n7;
        locals.var_t3_dn8 = assign63030_e97707_d_n8;
        locals.var_t3_dn9 = assign63030_e97707_d_n9;
        locals.var_t3_dn10 = assign63030_e97707_d_n10;
        locals.var_t3_dn11 = assign63030_e97707_d_n11;
        locals.var_t3_dn14 = assign63030_e97707_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign63040_e97718, assign63040_e97718_d_n0, assign63040_e97718_d_n2, assign63040_e97718_d_n4, assign63040_e97718_d_n5, assign63040_e97718_d_n6, assign63040_e97718_d_n7, assign63040_e97718_d_n8, assign63040_e97718_d_n9, assign63040_e97718_d_n10, assign63040_e97718_d_n11, assign63040_e97718_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63040_e97716: f64 = (1.0 / locals.var_t3);
        (assign63040_e97716, (-(locals.var_t3_dn0 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn2 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn14 / (locals.var_t3 * locals.var_t3))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign63040_e97718;
        locals.var_t4_dn0 = assign63040_e97718_d_n0;
        locals.var_t4_dn2 = assign63040_e97718_d_n2;
        locals.var_t4_dn4 = assign63040_e97718_d_n4;
        locals.var_t4_dn5 = assign63040_e97718_d_n5;
        locals.var_t4_dn6 = assign63040_e97718_d_n6;
        locals.var_t4_dn7 = assign63040_e97718_d_n7;
        locals.var_t4_dn8 = assign63040_e97718_d_n8;
        locals.var_t4_dn9 = assign63040_e97718_d_n9;
        locals.var_t4_dn10 = assign63040_e97718_d_n10;
        locals.var_t4_dn11 = assign63040_e97718_d_n11;
        locals.var_t4_dn14 = assign63040_e97718_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign63050_e97729, assign63050_e97729_d_n0, assign63050_e97729_d_n2, assign63050_e97729_d_n4, assign63050_e97729_d_n5, assign63050_e97729_d_n6, assign63050_e97729_d_n7, assign63050_e97729_d_n8, assign63050_e97729_d_n9, assign63050_e97729_d_n10, assign63050_e97729_d_n11, assign63050_e97729_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        let assign63050_e97727: f64 = (locals.var_ids0 * locals.var_t4);
        (assign63050_e97727, ((locals.var_ids0_dn0 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn0)), ((locals.var_ids0_dn2 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn2)), ((locals.var_ids0_dn4 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn4)), ((locals.var_ids0_dn5 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn5)), ((locals.var_ids0_dn6 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn6)), ((locals.var_ids0_dn7 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn7)), ((locals.var_ids0_dn8 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn8)), ((locals.var_ids0_dn9 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn9)), ((locals.var_ids0_dn10 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn10)), ((locals.var_ids0_dn11 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn11)), ((locals.var_ids0_dn14 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn14)),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn14,)
    }
};
        locals.var_ids = assign63050_e97729;
        locals.var_ids_dn0 = assign63050_e97729_d_n0;
        locals.var_ids_dn2 = assign63050_e97729_d_n2;
        locals.var_ids_dn4 = assign63050_e97729_d_n4;
        locals.var_ids_dn5 = assign63050_e97729_d_n5;
        locals.var_ids_dn6 = assign63050_e97729_d_n6;
        locals.var_ids_dn7 = assign63050_e97729_d_n7;
        locals.var_ids_dn8 = assign63050_e97729_d_n8;
        locals.var_ids_dn9 = assign63050_e97729_d_n9;
        locals.var_ids_dn10 = assign63050_e97729_d_n10;
        locals.var_ids_dn11 = assign63050_e97729_d_n11;
        locals.var_ids_dn14 = assign63050_e97729_d_n14;
        locals.var_ids_rv = 0.0;

        let (assign63060_e97739, assign63060_e97739_d_n0, assign63060_e97739_d_n2, assign63060_e97739_d_n4, assign63060_e97739_d_n5, assign63060_e97739_d_n6, assign63060_e97739_d_n7, assign63060_e97739_d_n8, assign63060_e97739_d_n9, assign63060_e97739_d_n10, assign63060_e97739_d_n11, assign63060_e97739_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1503 == 0.0)) {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn4, locals.var_ids0_dn5, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn8, locals.var_ids0_dn9, locals.var_ids0_dn10, locals.var_ids0_dn11, locals.var_ids0_dn14,)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn14,)
    }
};
        locals.var_ids = assign63060_e97739;
        locals.var_ids_dn0 = assign63060_e97739_d_n0;
        locals.var_ids_dn2 = assign63060_e97739_d_n2;
        locals.var_ids_dn4 = assign63060_e97739_d_n4;
        locals.var_ids_dn5 = assign63060_e97739_d_n5;
        locals.var_ids_dn6 = assign63060_e97739_d_n6;
        locals.var_ids_dn7 = assign63060_e97739_d_n7;
        locals.var_ids_dn8 = assign63060_e97739_d_n8;
        locals.var_ids_dn9 = assign63060_e97739_d_n9;
        locals.var_ids_dn10 = assign63060_e97739_d_n10;
        locals.var_ids_dn11 = assign63060_e97739_d_n11;
        locals.var_ids_dn14 = assign63060_e97739_d_n14;
        locals.var_ids_rv = 0.0;

        let (assign63070_e97749, assign63070_e97749_d_n0, assign63070_e97749_d_n2, assign63070_e97749_d_n4, assign63070_e97749_d_n5, assign63070_e97749_d_n6, assign63070_e97749_d_n7, assign63070_e97749_d_n8, assign63070_e97749_d_n9, assign63070_e97749_d_n10, assign63070_e97749_d_n11, assign63070_e97749_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1503 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ra, locals.var_ra_dn0, locals.var_ra_dn2, locals.var_ra_dn4, locals.var_ra_dn5, locals.var_ra_dn6, locals.var_ra_dn7, locals.var_ra_dn8, locals.var_ra_dn9, locals.var_ra_dn10, locals.var_ra_dn11, locals.var_ra_dn14,)
    }
};
        locals.var_ra = assign63070_e97749;
        locals.var_ra_dn0 = assign63070_e97749_d_n0;
        locals.var_ra_dn2 = assign63070_e97749_d_n2;
        locals.var_ra_dn4 = assign63070_e97749_d_n4;
        locals.var_ra_dn5 = assign63070_e97749_d_n5;
        locals.var_ra_dn6 = assign63070_e97749_d_n6;
        locals.var_ra_dn7 = assign63070_e97749_d_n7;
        locals.var_ra_dn8 = assign63070_e97749_d_n8;
        locals.var_ra_dn9 = assign63070_e97749_d_n9;
        locals.var_ra_dn10 = assign63070_e97749_d_n10;
        locals.var_ra_dn11 = assign63070_e97749_d_n11;
        locals.var_ra_dn14 = assign63070_e97749_d_n14;
        locals.var_ra_rv = 0.0;

        let assign63080_e97752: f64 = if p.p27 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1507 = assign63080_e97752;
        locals.var_guard1507_rv = 0.0;

        let (assign63090_e97763, assign63090_e97763_d_n0, assign63090_e97763_d_n2, assign63090_e97763_d_n4, assign63090_e97763_d_n5, assign63090_e97763_d_n6, assign63090_e97763_d_n7, assign63090_e97763_d_n8, assign63090_e97763_d_n9, assign63090_e97763_d_n10, assign63090_e97763_d_n11, assign63090_e97763_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63090_e97761: f64 = (1.034943e-10 * locals.var_cox_inv);
        (assign63090_e97761, (1.034943e-10 * locals.var_cox_inv_dn0), (1.034943e-10 * locals.var_cox_inv_dn2), (1.034943e-10 * locals.var_cox_inv_dn4), (1.034943e-10 * locals.var_cox_inv_dn5), (1.034943e-10 * locals.var_cox_inv_dn6), (1.034943e-10 * locals.var_cox_inv_dn7), (1.034943e-10 * locals.var_cox_inv_dn8), (1.034943e-10 * locals.var_cox_inv_dn9), (1.034943e-10 * locals.var_cox_inv_dn10), (1.034943e-10 * locals.var_cox_inv_dn11), (1.034943e-10 * locals.var_cox_inv_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63090_e97763;
        locals.var_t1_dn0 = assign63090_e97763_d_n0;
        locals.var_t1_dn2 = assign63090_e97763_d_n2;
        locals.var_t1_dn4 = assign63090_e97763_d_n4;
        locals.var_t1_dn5 = assign63090_e97763_d_n5;
        locals.var_t1_dn6 = assign63090_e97763_d_n6;
        locals.var_t1_dn7 = assign63090_e97763_d_n7;
        locals.var_t1_dn8 = assign63090_e97763_d_n8;
        locals.var_t1_dn9 = assign63090_e97763_d_n9;
        locals.var_t1_dn10 = assign63090_e97763_d_n10;
        locals.var_t1_dn11 = assign63090_e97763_d_n11;
        locals.var_t1_dn14 = assign63090_e97763_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign63100_e97772, assign63100_e97772_d_n0, assign63100_e97772_d_n2, assign63100_e97772_d_n4, assign63100_e97772_d_n5, assign63100_e97772_d_n6, assign63100_e97772_d_n7, assign63100_e97772_d_n8, assign63100_e97772_d_n9, assign63100_e97772_d_n10, assign63100_e97772_d_n11, assign63100_e97772_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        (locals.var_wdpl, locals.var_wdpl_dn0, locals.var_wdpl_dn2, locals.var_wdpl_dn4, locals.var_wdpl_dn5, locals.var_wdpl_dn6, locals.var_wdpl_dn7, locals.var_wdpl_dn8, locals.var_wdpl_dn9, locals.var_wdpl_dn10, locals.var_wdpl_dn11, locals.var_wdpl_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign63100_e97772;
        locals.var_t2_dn0 = assign63100_e97772_d_n0;
        locals.var_t2_dn2 = assign63100_e97772_d_n2;
        locals.var_t2_dn4 = assign63100_e97772_d_n4;
        locals.var_t2_dn5 = assign63100_e97772_d_n5;
        locals.var_t2_dn6 = assign63100_e97772_d_n6;
        locals.var_t2_dn7 = assign63100_e97772_d_n7;
        locals.var_t2_dn8 = assign63100_e97772_d_n8;
        locals.var_t2_dn9 = assign63100_e97772_d_n9;
        locals.var_t2_dn10 = assign63100_e97772_d_n10;
        locals.var_t2_dn11 = assign63100_e97772_d_n11;
        locals.var_t2_dn14 = assign63100_e97772_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign63110_e97783, assign63110_e97783_d_n0, assign63110_e97783_d_n2, assign63110_e97783_d_n4, assign63110_e97783_d_n5, assign63110_e97783_d_n6, assign63110_e97783_d_n7, assign63110_e97783_d_n8, assign63110_e97783_d_n9, assign63110_e97783_d_n10, assign63110_e97783_d_n11, assign63110_e97783_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63110_e97781: f64 = (locals.var_lgatesm - p.p139);
        (assign63110_e97781, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign63110_e97783;
        locals.var_t3_dn0 = assign63110_e97783_d_n0;
        locals.var_t3_dn2 = assign63110_e97783_d_n2;
        locals.var_t3_dn4 = assign63110_e97783_d_n4;
        locals.var_t3_dn5 = assign63110_e97783_d_n5;
        locals.var_t3_dn6 = assign63110_e97783_d_n6;
        locals.var_t3_dn7 = assign63110_e97783_d_n7;
        locals.var_t3_dn8 = assign63110_e97783_d_n8;
        locals.var_t3_dn9 = assign63110_e97783_d_n9;
        locals.var_t3_dn10 = assign63110_e97783_d_n10;
        locals.var_t3_dn11 = assign63110_e97783_d_n11;
        locals.var_t3_dn14 = assign63110_e97783_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign63120_e97796, assign63120_e97796_d_n0, assign63120_e97796_d_n2, assign63120_e97796_d_n4, assign63120_e97796_d_n5, assign63120_e97796_d_n6, assign63120_e97796_d_n7, assign63120_e97796_d_n8, assign63120_e97796_d_n9, assign63120_e97796_d_n10, assign63120_e97796_d_n11, assign63120_e97796_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63120_e97793: f64 = (locals.var_t3 * locals.var_t3);
        let assign63120_e97794: f64 = (1.0 / assign63120_e97793);
        (assign63120_e97794, (-(((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0)) / (assign63120_e97793 * assign63120_e97793))), (-(((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2)) / (assign63120_e97793 * assign63120_e97793))), (-(((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4)) / (assign63120_e97793 * assign63120_e97793))), (-(((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5)) / (assign63120_e97793 * assign63120_e97793))), (-(((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6)) / (assign63120_e97793 * assign63120_e97793))), (-(((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7)) / (assign63120_e97793 * assign63120_e97793))), (-(((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8)) / (assign63120_e97793 * assign63120_e97793))), (-(((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9)) / (assign63120_e97793 * assign63120_e97793))), (-(((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10)) / (assign63120_e97793 * assign63120_e97793))), (-(((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11)) / (assign63120_e97793 * assign63120_e97793))), (-(((locals.var_t3_dn14 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn14)) / (assign63120_e97793 * assign63120_e97793))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign63120_e97796;
        locals.var_t4_dn0 = assign63120_e97796_d_n0;
        locals.var_t4_dn2 = assign63120_e97796_d_n2;
        locals.var_t4_dn4 = assign63120_e97796_d_n4;
        locals.var_t4_dn5 = assign63120_e97796_d_n5;
        locals.var_t4_dn6 = assign63120_e97796_d_n6;
        locals.var_t4_dn7 = assign63120_e97796_d_n7;
        locals.var_t4_dn8 = assign63120_e97796_d_n8;
        locals.var_t4_dn9 = assign63120_e97796_d_n9;
        locals.var_t4_dn10 = assign63120_e97796_d_n10;
        locals.var_t4_dn11 = assign63120_e97796_d_n11;
        locals.var_t4_dn14 = assign63120_e97796_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign63130_e97815, assign63130_e97815_d_n0, assign63130_e97815_d_n2, assign63130_e97815_d_n4, assign63130_e97815_d_n5, assign63130_e97815_d_n6, assign63130_e97815_d_n7, assign63130_e97815_d_n8, assign63130_e97815_d_n9, assign63130_e97815_d_n10, assign63130_e97815_d_n11, assign63130_e97815_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63130_e97806: f64 = (p.p137 - locals.var_pb20b);
        let assign63130_e97807: f64 = (2.0 * assign63130_e97806);
        let assign63130_e97809: f64 = (assign63130_e97807 * locals.var_t1);
        let assign63130_e97811: f64 = (assign63130_e97809 * locals.var_t2);
        let assign63130_e97813: f64 = (assign63130_e97811 * locals.var_t4);
        (assign63130_e97813, (((((((2.0 * (-locals.var_pb20b_dn0)) * locals.var_t1) + (assign63130_e97807 * locals.var_t1_dn0)) * locals.var_t2) + (assign63130_e97809 * locals.var_t2_dn0)) * locals.var_t4) + (assign63130_e97811 * locals.var_t4_dn0)), (((((((2.0 * (-locals.var_pb20b_dn2)) * locals.var_t1) + (assign63130_e97807 * locals.var_t1_dn2)) * locals.var_t2) + (assign63130_e97809 * locals.var_t2_dn2)) * locals.var_t4) + (assign63130_e97811 * locals.var_t4_dn2)), (((((((2.0 * (-locals.var_pb20b_dn4)) * locals.var_t1) + (assign63130_e97807 * locals.var_t1_dn4)) * locals.var_t2) + (assign63130_e97809 * locals.var_t2_dn4)) * locals.var_t4) + (assign63130_e97811 * locals.var_t4_dn4)), (((((((2.0 * (-locals.var_pb20b_dn5)) * locals.var_t1) + (assign63130_e97807 * locals.var_t1_dn5)) * locals.var_t2) + (assign63130_e97809 * locals.var_t2_dn5)) * locals.var_t4) + (assign63130_e97811 * locals.var_t4_dn5)), (((((((2.0 * (-locals.var_pb20b_dn6)) * locals.var_t1) + (assign63130_e97807 * locals.var_t1_dn6)) * locals.var_t2) + (assign63130_e97809 * locals.var_t2_dn6)) * locals.var_t4) + (assign63130_e97811 * locals.var_t4_dn6)), (((((((2.0 * (-locals.var_pb20b_dn7)) * locals.var_t1) + (assign63130_e97807 * locals.var_t1_dn7)) * locals.var_t2) + (assign63130_e97809 * locals.var_t2_dn7)) * locals.var_t4) + (assign63130_e97811 * locals.var_t4_dn7)), (((((((2.0 * (-locals.var_pb20b_dn8)) * locals.var_t1) + (assign63130_e97807 * locals.var_t1_dn8)) * locals.var_t2) + (assign63130_e97809 * locals.var_t2_dn8)) * locals.var_t4) + (assign63130_e97811 * locals.var_t4_dn8)), (((((((2.0 * (-locals.var_pb20b_dn9)) * locals.var_t1) + (assign63130_e97807 * locals.var_t1_dn9)) * locals.var_t2) + (assign63130_e97809 * locals.var_t2_dn9)) * locals.var_t4) + (assign63130_e97811 * locals.var_t4_dn9)), (((((((2.0 * (-locals.var_pb20b_dn10)) * locals.var_t1) + (assign63130_e97807 * locals.var_t1_dn10)) * locals.var_t2) + (assign63130_e97809 * locals.var_t2_dn10)) * locals.var_t4) + (assign63130_e97811 * locals.var_t4_dn10)), (((((((2.0 * (-locals.var_pb20b_dn11)) * locals.var_t1) + (assign63130_e97807 * locals.var_t1_dn11)) * locals.var_t2) + (assign63130_e97809 * locals.var_t2_dn11)) * locals.var_t4) + (assign63130_e97811 * locals.var_t4_dn11)), (((((((2.0 * (-locals.var_pb20b_dn14)) * locals.var_t1) + (assign63130_e97807 * locals.var_t1_dn14)) * locals.var_t2) + (assign63130_e97809 * locals.var_t2_dn14)) * locals.var_t4) + (assign63130_e97811 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign63130_e97815;
        locals.var_t5_dn0 = assign63130_e97815_d_n0;
        locals.var_t5_dn2 = assign63130_e97815_d_n2;
        locals.var_t5_dn4 = assign63130_e97815_d_n4;
        locals.var_t5_dn5 = assign63130_e97815_d_n5;
        locals.var_t5_dn6 = assign63130_e97815_d_n6;
        locals.var_t5_dn7 = assign63130_e97815_d_n7;
        locals.var_t5_dn8 = assign63130_e97815_d_n8;
        locals.var_t5_dn9 = assign63130_e97815_d_n9;
        locals.var_t5_dn10 = assign63130_e97815_d_n10;
        locals.var_t5_dn11 = assign63130_e97815_d_n11;
        locals.var_t5_dn14 = assign63130_e97815_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign63140_e97826, assign63140_e97826_d_n0, assign63140_e97826_d_n2, assign63140_e97826_d_n4, assign63140_e97826_d_n5, assign63140_e97826_d_n6, assign63140_e97826_d_n7, assign63140_e97826_d_n8, assign63140_e97826_d_n9, assign63140_e97826_d_n10, assign63140_e97826_d_n11, assign63140_e97826_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63140_e97824: f64 = (locals.var_t5 * locals.var_sqrt_pbsum);
        (assign63140_e97824, ((locals.var_t5_dn0 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn0)), ((locals.var_t5_dn2 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn2)), ((locals.var_t5_dn4 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn4)), ((locals.var_t5_dn5 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn5)), ((locals.var_t5_dn6 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn6)), ((locals.var_t5_dn7 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn7)), ((locals.var_t5_dn8 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn8)), ((locals.var_t5_dn9 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn9)), ((locals.var_t5_dn10 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn10)), ((locals.var_t5_dn11 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn11)), ((locals.var_t5_dn14 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn14)),)
    } else {
        (locals.var_dvth0, locals.var_dvth0_dn0, locals.var_dvth0_dn2, locals.var_dvth0_dn4, locals.var_dvth0_dn5, locals.var_dvth0_dn6, locals.var_dvth0_dn7, locals.var_dvth0_dn8, locals.var_dvth0_dn9, locals.var_dvth0_dn10, locals.var_dvth0_dn11, locals.var_dvth0_dn14,)
    }
};
        locals.var_dvth0 = assign63140_e97826;
        locals.var_dvth0_dn0 = assign63140_e97826_d_n0;
        locals.var_dvth0_dn2 = assign63140_e97826_d_n2;
        locals.var_dvth0_dn4 = assign63140_e97826_d_n4;
        locals.var_dvth0_dn5 = assign63140_e97826_d_n5;
        locals.var_dvth0_dn6 = assign63140_e97826_d_n6;
        locals.var_dvth0_dn7 = assign63140_e97826_d_n7;
        locals.var_dvth0_dn8 = assign63140_e97826_d_n8;
        locals.var_dvth0_dn9 = assign63140_e97826_d_n9;
        locals.var_dvth0_dn10 = assign63140_e97826_d_n10;
        locals.var_dvth0_dn11 = assign63140_e97826_d_n11;
        locals.var_dvth0_dn14 = assign63140_e97826_d_n14;
        locals.var_dvth0_rv = 0.0;

        let (assign63150_e97839, assign63150_e97839_d_n0, assign63150_e97839_d_n2, assign63150_e97839_d_n4, assign63150_e97839_d_n5, assign63150_e97839_d_n6, assign63150_e97839_d_n7, assign63150_e97839_d_n8, assign63150_e97839_d_n9, assign63150_e97839_d_n10, assign63150_e97839_d_n11, assign63150_e97839_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63150_e97835: f64 = (locals.var_t5 * 0.5);
        let assign63150_e97837: f64 = (assign63150_e97835 / locals.var_sqrt_pbsum);
        (assign63150_e97837, ((((locals.var_t5_dn0 * 0.5) * locals.var_sqrt_pbsum) - (assign63150_e97835 * locals.var_sqrt_pbsum_dn0)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn2 * 0.5) * locals.var_sqrt_pbsum) - (assign63150_e97835 * locals.var_sqrt_pbsum_dn2)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn4 * 0.5) * locals.var_sqrt_pbsum) - (assign63150_e97835 * locals.var_sqrt_pbsum_dn4)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn5 * 0.5) * locals.var_sqrt_pbsum) - (assign63150_e97835 * locals.var_sqrt_pbsum_dn5)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn6 * 0.5) * locals.var_sqrt_pbsum) - (assign63150_e97835 * locals.var_sqrt_pbsum_dn6)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn7 * 0.5) * locals.var_sqrt_pbsum) - (assign63150_e97835 * locals.var_sqrt_pbsum_dn7)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn8 * 0.5) * locals.var_sqrt_pbsum) - (assign63150_e97835 * locals.var_sqrt_pbsum_dn8)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn9 * 0.5) * locals.var_sqrt_pbsum) - (assign63150_e97835 * locals.var_sqrt_pbsum_dn9)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn10 * 0.5) * locals.var_sqrt_pbsum) - (assign63150_e97835 * locals.var_sqrt_pbsum_dn10)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn11 * 0.5) * locals.var_sqrt_pbsum) - (assign63150_e97835 * locals.var_sqrt_pbsum_dn11)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn14 * 0.5) * locals.var_sqrt_pbsum) - (assign63150_e97835 * locals.var_sqrt_pbsum_dn14)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign63150_e97839;
        locals.var_t6_dn0 = assign63150_e97839_d_n0;
        locals.var_t6_dn2 = assign63150_e97839_d_n2;
        locals.var_t6_dn4 = assign63150_e97839_d_n4;
        locals.var_t6_dn5 = assign63150_e97839_d_n5;
        locals.var_t6_dn6 = assign63150_e97839_d_n6;
        locals.var_t6_dn7 = assign63150_e97839_d_n7;
        locals.var_t6_dn8 = assign63150_e97839_d_n8;
        locals.var_t6_dn9 = assign63150_e97839_d_n9;
        locals.var_t6_dn10 = assign63150_e97839_d_n10;
        locals.var_t6_dn11 = assign63150_e97839_d_n11;
        locals.var_t6_dn14 = assign63150_e97839_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign63160_e97860, assign63160_e97860_d_n0, assign63160_e97860_d_n2, assign63160_e97860_d_n4, assign63160_e97860_d_n5, assign63160_e97860_d_n6, assign63160_e97860_d_n7, assign63160_e97860_d_n8, assign63160_e97860_d_n9, assign63160_e97860_d_n10, assign63160_e97860_d_n11, assign63160_e97860_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63160_e97849: f64 = (p.p137 - locals.var_pb20b);
        let assign63160_e97850: f64 = (2.0 * assign63160_e97849);
        let assign63160_e97852: f64 = (assign63160_e97850 * 1.034943e-10);
        let assign63160_e97854: f64 = (assign63160_e97852 * locals.var_t2);
        let assign63160_e97856: f64 = (assign63160_e97854 * locals.var_t4);
        let assign63160_e97858: f64 = (assign63160_e97856 * locals.var_sqrt_pbsum);
        (assign63160_e97858, ((((((((2.0 * (-locals.var_pb20b_dn0)) * 1.034943e-10) * locals.var_t2) + (assign63160_e97852 * locals.var_t2_dn0)) * locals.var_t4) + (assign63160_e97854 * locals.var_t4_dn0)) * locals.var_sqrt_pbsum) + (assign63160_e97856 * locals.var_sqrt_pbsum_dn0)), ((((((((2.0 * (-locals.var_pb20b_dn2)) * 1.034943e-10) * locals.var_t2) + (assign63160_e97852 * locals.var_t2_dn2)) * locals.var_t4) + (assign63160_e97854 * locals.var_t4_dn2)) * locals.var_sqrt_pbsum) + (assign63160_e97856 * locals.var_sqrt_pbsum_dn2)), ((((((((2.0 * (-locals.var_pb20b_dn4)) * 1.034943e-10) * locals.var_t2) + (assign63160_e97852 * locals.var_t2_dn4)) * locals.var_t4) + (assign63160_e97854 * locals.var_t4_dn4)) * locals.var_sqrt_pbsum) + (assign63160_e97856 * locals.var_sqrt_pbsum_dn4)), ((((((((2.0 * (-locals.var_pb20b_dn5)) * 1.034943e-10) * locals.var_t2) + (assign63160_e97852 * locals.var_t2_dn5)) * locals.var_t4) + (assign63160_e97854 * locals.var_t4_dn5)) * locals.var_sqrt_pbsum) + (assign63160_e97856 * locals.var_sqrt_pbsum_dn5)), ((((((((2.0 * (-locals.var_pb20b_dn6)) * 1.034943e-10) * locals.var_t2) + (assign63160_e97852 * locals.var_t2_dn6)) * locals.var_t4) + (assign63160_e97854 * locals.var_t4_dn6)) * locals.var_sqrt_pbsum) + (assign63160_e97856 * locals.var_sqrt_pbsum_dn6)), ((((((((2.0 * (-locals.var_pb20b_dn7)) * 1.034943e-10) * locals.var_t2) + (assign63160_e97852 * locals.var_t2_dn7)) * locals.var_t4) + (assign63160_e97854 * locals.var_t4_dn7)) * locals.var_sqrt_pbsum) + (assign63160_e97856 * locals.var_sqrt_pbsum_dn7)), ((((((((2.0 * (-locals.var_pb20b_dn8)) * 1.034943e-10) * locals.var_t2) + (assign63160_e97852 * locals.var_t2_dn8)) * locals.var_t4) + (assign63160_e97854 * locals.var_t4_dn8)) * locals.var_sqrt_pbsum) + (assign63160_e97856 * locals.var_sqrt_pbsum_dn8)), ((((((((2.0 * (-locals.var_pb20b_dn9)) * 1.034943e-10) * locals.var_t2) + (assign63160_e97852 * locals.var_t2_dn9)) * locals.var_t4) + (assign63160_e97854 * locals.var_t4_dn9)) * locals.var_sqrt_pbsum) + (assign63160_e97856 * locals.var_sqrt_pbsum_dn9)), ((((((((2.0 * (-locals.var_pb20b_dn10)) * 1.034943e-10) * locals.var_t2) + (assign63160_e97852 * locals.var_t2_dn10)) * locals.var_t4) + (assign63160_e97854 * locals.var_t4_dn10)) * locals.var_sqrt_pbsum) + (assign63160_e97856 * locals.var_sqrt_pbsum_dn10)), ((((((((2.0 * (-locals.var_pb20b_dn11)) * 1.034943e-10) * locals.var_t2) + (assign63160_e97852 * locals.var_t2_dn11)) * locals.var_t4) + (assign63160_e97854 * locals.var_t4_dn11)) * locals.var_sqrt_pbsum) + (assign63160_e97856 * locals.var_sqrt_pbsum_dn11)), ((((((((2.0 * (-locals.var_pb20b_dn14)) * 1.034943e-10) * locals.var_t2) + (assign63160_e97852 * locals.var_t2_dn14)) * locals.var_t4) + (assign63160_e97854 * locals.var_t4_dn14)) * locals.var_sqrt_pbsum) + (assign63160_e97856 * locals.var_sqrt_pbsum_dn14)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign63160_e97860;
        locals.var_t7_dn0 = assign63160_e97860_d_n0;
        locals.var_t7_dn2 = assign63160_e97860_d_n2;
        locals.var_t7_dn4 = assign63160_e97860_d_n4;
        locals.var_t7_dn5 = assign63160_e97860_d_n5;
        locals.var_t7_dn6 = assign63160_e97860_d_n6;
        locals.var_t7_dn7 = assign63160_e97860_d_n7;
        locals.var_t7_dn8 = assign63160_e97860_d_n8;
        locals.var_t7_dn9 = assign63160_e97860_d_n9;
        locals.var_t7_dn10 = assign63160_e97860_d_n10;
        locals.var_t7_dn11 = assign63160_e97860_d_n11;
        locals.var_t7_dn14 = assign63160_e97860_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign63170_e97878, assign63170_e97878_d_n0, assign63170_e97878_d_n2, assign63170_e97878_d_n4, assign63170_e97878_d_n5, assign63170_e97878_d_n6, assign63170_e97878_d_n7, assign63170_e97878_d_n8, assign63170_e97878_d_n9, assign63170_e97878_d_n10, assign63170_e97878_d_n11, assign63170_e97878_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63170_e97868: f64 = (-2.0);
        let assign63170_e97870: f64 = (assign63170_e97868 * locals.var_t1);
        let assign63170_e97872: f64 = (assign63170_e97870 * locals.var_t2);
        let assign63170_e97874: f64 = (assign63170_e97872 * locals.var_t4);
        let assign63170_e97876: f64 = (assign63170_e97874 * locals.var_sqrt_pbsum);
        (assign63170_e97876, (((((((assign63170_e97868 * locals.var_t1_dn0) * locals.var_t2) + (assign63170_e97870 * locals.var_t2_dn0)) * locals.var_t4) + (assign63170_e97872 * locals.var_t4_dn0)) * locals.var_sqrt_pbsum) + (assign63170_e97874 * locals.var_sqrt_pbsum_dn0)), (((((((assign63170_e97868 * locals.var_t1_dn2) * locals.var_t2) + (assign63170_e97870 * locals.var_t2_dn2)) * locals.var_t4) + (assign63170_e97872 * locals.var_t4_dn2)) * locals.var_sqrt_pbsum) + (assign63170_e97874 * locals.var_sqrt_pbsum_dn2)), (((((((assign63170_e97868 * locals.var_t1_dn4) * locals.var_t2) + (assign63170_e97870 * locals.var_t2_dn4)) * locals.var_t4) + (assign63170_e97872 * locals.var_t4_dn4)) * locals.var_sqrt_pbsum) + (assign63170_e97874 * locals.var_sqrt_pbsum_dn4)), (((((((assign63170_e97868 * locals.var_t1_dn5) * locals.var_t2) + (assign63170_e97870 * locals.var_t2_dn5)) * locals.var_t4) + (assign63170_e97872 * locals.var_t4_dn5)) * locals.var_sqrt_pbsum) + (assign63170_e97874 * locals.var_sqrt_pbsum_dn5)), (((((((assign63170_e97868 * locals.var_t1_dn6) * locals.var_t2) + (assign63170_e97870 * locals.var_t2_dn6)) * locals.var_t4) + (assign63170_e97872 * locals.var_t4_dn6)) * locals.var_sqrt_pbsum) + (assign63170_e97874 * locals.var_sqrt_pbsum_dn6)), (((((((assign63170_e97868 * locals.var_t1_dn7) * locals.var_t2) + (assign63170_e97870 * locals.var_t2_dn7)) * locals.var_t4) + (assign63170_e97872 * locals.var_t4_dn7)) * locals.var_sqrt_pbsum) + (assign63170_e97874 * locals.var_sqrt_pbsum_dn7)), (((((((assign63170_e97868 * locals.var_t1_dn8) * locals.var_t2) + (assign63170_e97870 * locals.var_t2_dn8)) * locals.var_t4) + (assign63170_e97872 * locals.var_t4_dn8)) * locals.var_sqrt_pbsum) + (assign63170_e97874 * locals.var_sqrt_pbsum_dn8)), (((((((assign63170_e97868 * locals.var_t1_dn9) * locals.var_t2) + (assign63170_e97870 * locals.var_t2_dn9)) * locals.var_t4) + (assign63170_e97872 * locals.var_t4_dn9)) * locals.var_sqrt_pbsum) + (assign63170_e97874 * locals.var_sqrt_pbsum_dn9)), (((((((assign63170_e97868 * locals.var_t1_dn10) * locals.var_t2) + (assign63170_e97870 * locals.var_t2_dn10)) * locals.var_t4) + (assign63170_e97872 * locals.var_t4_dn10)) * locals.var_sqrt_pbsum) + (assign63170_e97874 * locals.var_sqrt_pbsum_dn10)), (((((((assign63170_e97868 * locals.var_t1_dn11) * locals.var_t2) + (assign63170_e97870 * locals.var_t2_dn11)) * locals.var_t4) + (assign63170_e97872 * locals.var_t4_dn11)) * locals.var_sqrt_pbsum) + (assign63170_e97874 * locals.var_sqrt_pbsum_dn11)), (((((((assign63170_e97868 * locals.var_t1_dn14) * locals.var_t2) + (assign63170_e97870 * locals.var_t2_dn14)) * locals.var_t4) + (assign63170_e97872 * locals.var_t4_dn14)) * locals.var_sqrt_pbsum) + (assign63170_e97874 * locals.var_sqrt_pbsum_dn14)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign63170_e97878;
        locals.var_t8_dn0 = assign63170_e97878_d_n0;
        locals.var_t8_dn2 = assign63170_e97878_d_n2;
        locals.var_t8_dn4 = assign63170_e97878_d_n4;
        locals.var_t8_dn5 = assign63170_e97878_d_n5;
        locals.var_t8_dn6 = assign63170_e97878_d_n6;
        locals.var_t8_dn7 = assign63170_e97878_d_n7;
        locals.var_t8_dn8 = assign63170_e97878_d_n8;
        locals.var_t8_dn9 = assign63170_e97878_d_n9;
        locals.var_t8_dn10 = assign63170_e97878_d_n10;
        locals.var_t8_dn11 = assign63170_e97878_d_n11;
        locals.var_t8_dn14 = assign63170_e97878_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign63180_e97887, assign63180_e97887_d_n0, assign63180_e97887_d_n2, assign63180_e97887_d_n4, assign63180_e97887_d_n5, assign63180_e97887_d_n6, assign63180_e97887_d_n7, assign63180_e97887_d_n8, assign63180_e97887_d_n9, assign63180_e97887_d_n10, assign63180_e97887_d_n11, assign63180_e97887_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        (locals.var_uc_scsti1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign63180_e97887;
        locals.var_t4_dn0 = assign63180_e97887_d_n0;
        locals.var_t4_dn2 = assign63180_e97887_d_n2;
        locals.var_t4_dn4 = assign63180_e97887_d_n4;
        locals.var_t4_dn5 = assign63180_e97887_d_n5;
        locals.var_t4_dn6 = assign63180_e97887_d_n6;
        locals.var_t4_dn7 = assign63180_e97887_d_n7;
        locals.var_t4_dn8 = assign63180_e97887_d_n8;
        locals.var_t4_dn9 = assign63180_e97887_d_n9;
        locals.var_t4_dn10 = assign63180_e97887_d_n10;
        locals.var_t4_dn11 = assign63180_e97887_d_n11;
        locals.var_t4_dn14 = assign63180_e97887_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign63190_e97896, assign63190_e97896_d_n0, assign63190_e97896_d_n2, assign63190_e97896_d_n4, assign63190_e97896_d_n5, assign63190_e97896_d_n6, assign63190_e97896_d_n7, assign63190_e97896_d_n8, assign63190_e97896_d_n9, assign63190_e97896_d_n10, assign63190_e97896_d_n11, assign63190_e97896_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        (locals.var_uc_scsti2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign63190_e97896;
        locals.var_t6_dn0 = assign63190_e97896_d_n0;
        locals.var_t6_dn2 = assign63190_e97896_d_n2;
        locals.var_t6_dn4 = assign63190_e97896_d_n4;
        locals.var_t6_dn5 = assign63190_e97896_d_n5;
        locals.var_t6_dn6 = assign63190_e97896_d_n6;
        locals.var_t6_dn7 = assign63190_e97896_d_n7;
        locals.var_t6_dn8 = assign63190_e97896_d_n8;
        locals.var_t6_dn9 = assign63190_e97896_d_n9;
        locals.var_t6_dn10 = assign63190_e97896_d_n10;
        locals.var_t6_dn11 = assign63190_e97896_d_n11;
        locals.var_t6_dn14 = assign63190_e97896_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign63200_e97909, assign63200_e97909_d_n0, assign63200_e97909_d_n2, assign63200_e97909_d_n4, assign63200_e97909_d_n5, assign63200_e97909_d_n6, assign63200_e97909_d_n7, assign63200_e97909_d_n8, assign63200_e97909_d_n9, assign63200_e97909_d_n10, assign63200_e97909_d_n11, assign63200_e97909_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63200_e97906: f64 = (locals.var_t6 * locals.var_vdsz__blk443);
        let assign63200_e97907: f64 = (locals.var_t4 + assign63200_e97906);
        (assign63200_e97907, (locals.var_t4_dn0 + ((locals.var_t6_dn0 * locals.var_vdsz__blk443) + (locals.var_t6 * locals.var_vdsz__blk443_dn0))), (locals.var_t4_dn2 + ((locals.var_t6_dn2 * locals.var_vdsz__blk443) + (locals.var_t6 * locals.var_vdsz__blk443_dn2))), (locals.var_t4_dn4 + ((locals.var_t6_dn4 * locals.var_vdsz__blk443) + (locals.var_t6 * locals.var_vdsz__blk443_dn4))), (locals.var_t4_dn5 + ((locals.var_t6_dn5 * locals.var_vdsz__blk443) + (locals.var_t6 * locals.var_vdsz__blk443_dn5))), (locals.var_t4_dn6 + ((locals.var_t6_dn6 * locals.var_vdsz__blk443) + (locals.var_t6 * locals.var_vdsz__blk443_dn6))), (locals.var_t4_dn7 + ((locals.var_t6_dn7 * locals.var_vdsz__blk443) + (locals.var_t6 * locals.var_vdsz__blk443_dn7))), (locals.var_t4_dn8 + ((locals.var_t6_dn8 * locals.var_vdsz__blk443) + (locals.var_t6 * locals.var_vdsz__blk443_dn8))), (locals.var_t4_dn9 + ((locals.var_t6_dn9 * locals.var_vdsz__blk443) + (locals.var_t6 * locals.var_vdsz__blk443_dn9))), (locals.var_t4_dn10 + ((locals.var_t6_dn10 * locals.var_vdsz__blk443) + (locals.var_t6 * locals.var_vdsz__blk443_dn10))), (locals.var_t4_dn11 + ((locals.var_t6_dn11 * locals.var_vdsz__blk443) + (locals.var_t6 * locals.var_vdsz__blk443_dn11))), (locals.var_t4_dn14 + ((locals.var_t6_dn14 * locals.var_vdsz__blk443) + (locals.var_t6 * locals.var_vdsz__blk443_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63200_e97909;
        locals.var_t1_dn0 = assign63200_e97909_d_n0;
        locals.var_t1_dn2 = assign63200_e97909_d_n2;
        locals.var_t1_dn4 = assign63200_e97909_d_n4;
        locals.var_t1_dn5 = assign63200_e97909_d_n5;
        locals.var_t1_dn6 = assign63200_e97909_d_n6;
        locals.var_t1_dn7 = assign63200_e97909_d_n7;
        locals.var_t1_dn8 = assign63200_e97909_d_n8;
        locals.var_t1_dn9 = assign63200_e97909_d_n9;
        locals.var_t1_dn10 = assign63200_e97909_d_n10;
        locals.var_t1_dn11 = assign63200_e97909_d_n11;
        locals.var_t1_dn14 = assign63200_e97909_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_236(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign63210_e97920, assign63210_e97920_d_n0, assign63210_e97920_d_n2, assign63210_e97920_d_n4, assign63210_e97920_d_n5, assign63210_e97920_d_n6, assign63210_e97920_d_n7, assign63210_e97920_d_n8, assign63210_e97920_d_n9, assign63210_e97920_d_n10, assign63210_e97920_d_n11, assign63210_e97920_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63210_e97918: f64 = (locals.var_dvth0 * locals.var_t1);
        (assign63210_e97918, ((locals.var_dvth0_dn0 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn0)), ((locals.var_dvth0_dn2 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn2)), ((locals.var_dvth0_dn4 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn4)), ((locals.var_dvth0_dn5 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn5)), ((locals.var_dvth0_dn6 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn6)), ((locals.var_dvth0_dn7 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn7)), ((locals.var_dvth0_dn8 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn8)), ((locals.var_dvth0_dn9 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn9)), ((locals.var_dvth0_dn10 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn10)), ((locals.var_dvth0_dn11 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn11)), ((locals.var_dvth0_dn14 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn14)),)
    } else {
        (locals.var_dvthscsti, locals.var_dvthscsti_dn0, locals.var_dvthscsti_dn2, locals.var_dvthscsti_dn4, locals.var_dvthscsti_dn5, locals.var_dvthscsti_dn6, locals.var_dvthscsti_dn7, locals.var_dvthscsti_dn8, locals.var_dvthscsti_dn9, locals.var_dvthscsti_dn10, locals.var_dvthscsti_dn11, locals.var_dvthscsti_dn14,)
    }
};
        locals.var_dvthscsti = assign63210_e97920;
        locals.var_dvthscsti_dn0 = assign63210_e97920_d_n0;
        locals.var_dvthscsti_dn2 = assign63210_e97920_d_n2;
        locals.var_dvthscsti_dn4 = assign63210_e97920_d_n4;
        locals.var_dvthscsti_dn5 = assign63210_e97920_d_n5;
        locals.var_dvthscsti_dn6 = assign63210_e97920_d_n6;
        locals.var_dvthscsti_dn7 = assign63210_e97920_d_n7;
        locals.var_dvthscsti_dn8 = assign63210_e97920_d_n8;
        locals.var_dvthscsti_dn9 = assign63210_e97920_d_n9;
        locals.var_dvthscsti_dn10 = assign63210_e97920_d_n10;
        locals.var_dvthscsti_dn11 = assign63210_e97920_d_n11;
        locals.var_dvthscsti_dn14 = assign63210_e97920_d_n14;
        locals.var_dvthscsti_rv = 0.0;

        let (assign63220_e97933, assign63220_e97933_d_n0, assign63220_e97933_d_n2, assign63220_e97933_d_n4, assign63220_e97933_d_n5, assign63220_e97933_d_n6, assign63220_e97933_d_n7, assign63220_e97933_d_n8, assign63220_e97933_d_n9, assign63220_e97933_d_n10, assign63220_e97933_d_n11, assign63220_e97933_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63220_e97930: f64 = (p.p213 * locals.var_vds);
        let assign63220_e97931: f64 = (locals.var_uc_vthsti - assign63220_e97930);
        (assign63220_e97931, (-(p.p213 * locals.var_vds_dn0)), (-(p.p213 * locals.var_vds_dn2)), (-(p.p213 * locals.var_vds_dn4)), (-(p.p213 * locals.var_vds_dn5)), (-(p.p213 * locals.var_vds_dn6)), (-(p.p213 * locals.var_vds_dn7)), (-(p.p213 * locals.var_vds_dn8)), (-(p.p213 * locals.var_vds_dn9)), (-(p.p213 * locals.var_vds_dn10)), (-(p.p213 * locals.var_vds_dn11)), (-(p.p213 * locals.var_vds_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63220_e97933;
        locals.var_t1_dn0 = assign63220_e97933_d_n0;
        locals.var_t1_dn2 = assign63220_e97933_d_n2;
        locals.var_t1_dn4 = assign63220_e97933_d_n4;
        locals.var_t1_dn5 = assign63220_e97933_d_n5;
        locals.var_t1_dn6 = assign63220_e97933_d_n6;
        locals.var_t1_dn7 = assign63220_e97933_d_n7;
        locals.var_t1_dn8 = assign63220_e97933_d_n8;
        locals.var_t1_dn9 = assign63220_e97933_d_n9;
        locals.var_t1_dn10 = assign63220_e97933_d_n10;
        locals.var_t1_dn11 = assign63220_e97933_d_n11;
        locals.var_t1_dn14 = assign63220_e97933_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign63230_e97948, assign63230_e97948_d_n0, assign63230_e97948_d_n2, assign63230_e97948_d_n4, assign63230_e97948_d_n5, assign63230_e97948_d_n6, assign63230_e97948_d_n7, assign63230_e97948_d_n8, assign63230_e97948_d_n9, assign63230_e97948_d_n10, assign63230_e97948_d_n11, assign63230_e97948_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63230_e97942: f64 = (locals.var_vgsz__blk444 - locals.var_vfb);
        let assign63230_e97944: f64 = (assign63230_e97942 + locals.var_t1);
        let assign63230_e97946: f64 = (assign63230_e97944 + locals.var_dvthscsti);
        (assign63230_e97946, ((locals.var_vgsz__blk444_dn0 + locals.var_t1_dn0) + locals.var_dvthscsti_dn0), ((locals.var_vgsz__blk444_dn2 + locals.var_t1_dn2) + locals.var_dvthscsti_dn2), ((locals.var_vgsz__blk444_dn4 + locals.var_t1_dn4) + locals.var_dvthscsti_dn4), ((locals.var_vgsz__blk444_dn5 + locals.var_t1_dn5) + locals.var_dvthscsti_dn5), ((locals.var_vgsz__blk444_dn6 + locals.var_t1_dn6) + locals.var_dvthscsti_dn6), ((locals.var_vgsz__blk444_dn7 + locals.var_t1_dn7) + locals.var_dvthscsti_dn7), ((locals.var_vgsz__blk444_dn8 + locals.var_t1_dn8) + locals.var_dvthscsti_dn8), ((locals.var_vgsz__blk444_dn9 + locals.var_t1_dn9) + locals.var_dvthscsti_dn9), ((locals.var_vgsz__blk444_dn10 + locals.var_t1_dn10) + locals.var_dvthscsti_dn10), ((locals.var_vgsz__blk444_dn11 + locals.var_t1_dn11) + locals.var_dvthscsti_dn11), ((locals.var_vgsz__blk444_dn14 + locals.var_t1_dn14) + locals.var_dvthscsti_dn14),)
    } else {
        (locals.var_vgssti, locals.var_vgssti_dn0, locals.var_vgssti_dn2, locals.var_vgssti_dn4, locals.var_vgssti_dn5, locals.var_vgssti_dn6, locals.var_vgssti_dn7, locals.var_vgssti_dn8, locals.var_vgssti_dn9, locals.var_vgssti_dn10, locals.var_vgssti_dn11, locals.var_vgssti_dn14,)
    }
};
        locals.var_vgssti = assign63230_e97948;
        locals.var_vgssti_dn0 = assign63230_e97948_d_n0;
        locals.var_vgssti_dn2 = assign63230_e97948_d_n2;
        locals.var_vgssti_dn4 = assign63230_e97948_d_n4;
        locals.var_vgssti_dn5 = assign63230_e97948_d_n5;
        locals.var_vgssti_dn6 = assign63230_e97948_d_n6;
        locals.var_vgssti_dn7 = assign63230_e97948_d_n7;
        locals.var_vgssti_dn8 = assign63230_e97948_d_n8;
        locals.var_vgssti_dn9 = assign63230_e97948_d_n9;
        locals.var_vgssti_dn10 = assign63230_e97948_d_n10;
        locals.var_vgssti_dn11 = assign63230_e97948_d_n11;
        locals.var_vgssti_dn14 = assign63230_e97948_d_n14;
        locals.var_vgssti_rv = 0.0;

        let (assign63240_e97961, assign63240_e97961_d_n0, assign63240_e97961_d_n2, assign63240_e97961_d_n4, assign63240_e97961_d_n5, assign63240_e97961_d_n6, assign63240_e97961_d_n7, assign63240_e97961_d_n8, assign63240_e97961_d_n9, assign63240_e97961_d_n10, assign63240_e97961_d_n11, assign63240_e97961_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63240_e97957: f64 = (locals.var_costi0_p2 * locals.var_cox_inv);
        let assign63240_e97959: f64 = (assign63240_e97957 * locals.var_cox_inv);
        (assign63240_e97959, ((((locals.var_costi0_p2_dn0 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn0)) * locals.var_cox_inv) + (assign63240_e97957 * locals.var_cox_inv_dn0)), ((((locals.var_costi0_p2_dn2 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn2)) * locals.var_cox_inv) + (assign63240_e97957 * locals.var_cox_inv_dn2)), ((((locals.var_costi0_p2_dn4 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn4)) * locals.var_cox_inv) + (assign63240_e97957 * locals.var_cox_inv_dn4)), ((((locals.var_costi0_p2_dn5 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn5)) * locals.var_cox_inv) + (assign63240_e97957 * locals.var_cox_inv_dn5)), ((((locals.var_costi0_p2_dn6 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn6)) * locals.var_cox_inv) + (assign63240_e97957 * locals.var_cox_inv_dn6)), ((((locals.var_costi0_p2_dn7 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn7)) * locals.var_cox_inv) + (assign63240_e97957 * locals.var_cox_inv_dn7)), ((((locals.var_costi0_p2_dn8 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn8)) * locals.var_cox_inv) + (assign63240_e97957 * locals.var_cox_inv_dn8)), ((((locals.var_costi0_p2_dn9 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn9)) * locals.var_cox_inv) + (assign63240_e97957 * locals.var_cox_inv_dn9)), ((((locals.var_costi0_p2_dn10 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn10)) * locals.var_cox_inv) + (assign63240_e97957 * locals.var_cox_inv_dn10)), ((((locals.var_costi0_p2_dn11 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn11)) * locals.var_cox_inv) + (assign63240_e97957 * locals.var_cox_inv_dn11)), ((((locals.var_costi0_p2_dn14 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn14)) * locals.var_cox_inv) + (assign63240_e97957 * locals.var_cox_inv_dn14)),)
    } else {
        (locals.var_costi3, locals.var_costi3_dn0, locals.var_costi3_dn2, locals.var_costi3_dn4, locals.var_costi3_dn5, locals.var_costi3_dn6, locals.var_costi3_dn7, locals.var_costi3_dn8, locals.var_costi3_dn9, locals.var_costi3_dn10, locals.var_costi3_dn11, locals.var_costi3_dn14,)
    }
};
        locals.var_costi3 = assign63240_e97961;
        locals.var_costi3_dn0 = assign63240_e97961_d_n0;
        locals.var_costi3_dn2 = assign63240_e97961_d_n2;
        locals.var_costi3_dn4 = assign63240_e97961_d_n4;
        locals.var_costi3_dn5 = assign63240_e97961_d_n5;
        locals.var_costi3_dn6 = assign63240_e97961_d_n6;
        locals.var_costi3_dn7 = assign63240_e97961_d_n7;
        locals.var_costi3_dn8 = assign63240_e97961_d_n8;
        locals.var_costi3_dn9 = assign63240_e97961_d_n9;
        locals.var_costi3_dn10 = assign63240_e97961_d_n10;
        locals.var_costi3_dn11 = assign63240_e97961_d_n11;
        locals.var_costi3_dn14 = assign63240_e97961_d_n14;
        locals.var_costi3_rv = 0.0;

        let (assign63250_e97974, assign63250_e97974_d_n0, assign63250_e97974_d_n2, assign63250_e97974_d_n4, assign63250_e97974_d_n5, assign63250_e97974_d_n6, assign63250_e97974_d_n7, assign63250_e97974_d_n8, assign63250_e97974_d_n9, assign63250_e97974_d_n10, assign63250_e97974_d_n11, assign63250_e97974_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63250_e97970: f64 = (locals.var_costi3 * locals.var_beta);
        let assign63250_e97972: f64 = (assign63250_e97970 * 0.5);
        (assign63250_e97972, (((locals.var_costi3_dn0 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn0)) * 0.5), (((locals.var_costi3_dn2 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn2)) * 0.5), (((locals.var_costi3_dn4 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn4)) * 0.5), (((locals.var_costi3_dn5 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn5)) * 0.5), (((locals.var_costi3_dn6 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn6)) * 0.5), (((locals.var_costi3_dn7 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn7)) * 0.5), (((locals.var_costi3_dn8 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn8)) * 0.5), (((locals.var_costi3_dn9 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn9)) * 0.5), (((locals.var_costi3_dn10 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn10)) * 0.5), (((locals.var_costi3_dn11 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn11)) * 0.5), (((locals.var_costi3_dn14 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn14)) * 0.5),)
    } else {
        (locals.var_costi4, locals.var_costi4_dn0, locals.var_costi4_dn2, locals.var_costi4_dn4, locals.var_costi4_dn5, locals.var_costi4_dn6, locals.var_costi4_dn7, locals.var_costi4_dn8, locals.var_costi4_dn9, locals.var_costi4_dn10, locals.var_costi4_dn11, locals.var_costi4_dn14,)
    }
};
        locals.var_costi4 = assign63250_e97974;
        locals.var_costi4_dn0 = assign63250_e97974_d_n0;
        locals.var_costi4_dn2 = assign63250_e97974_d_n2;
        locals.var_costi4_dn4 = assign63250_e97974_d_n4;
        locals.var_costi4_dn5 = assign63250_e97974_d_n5;
        locals.var_costi4_dn6 = assign63250_e97974_d_n6;
        locals.var_costi4_dn7 = assign63250_e97974_d_n7;
        locals.var_costi4_dn8 = assign63250_e97974_d_n8;
        locals.var_costi4_dn9 = assign63250_e97974_d_n9;
        locals.var_costi4_dn10 = assign63250_e97974_d_n10;
        locals.var_costi4_dn11 = assign63250_e97974_d_n11;
        locals.var_costi4_dn14 = assign63250_e97974_d_n14;
        locals.var_costi4_rv = 0.0;

        let (assign63260_e97987, assign63260_e97987_d_n0, assign63260_e97987_d_n2, assign63260_e97987_d_n4, assign63260_e97987_d_n5, assign63260_e97987_d_n6, assign63260_e97987_d_n7, assign63260_e97987_d_n8, assign63260_e97987_d_n9, assign63260_e97987_d_n10, assign63260_e97987_d_n11, assign63260_e97987_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63260_e97983: f64 = (locals.var_costi4 * locals.var_beta);
        let assign63260_e97985: f64 = (assign63260_e97983 * 2.0);
        (assign63260_e97985, (((locals.var_costi4_dn0 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn0)) * 2.0), (((locals.var_costi4_dn2 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn2)) * 2.0), (((locals.var_costi4_dn4 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn4)) * 2.0), (((locals.var_costi4_dn5 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn5)) * 2.0), (((locals.var_costi4_dn6 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn6)) * 2.0), (((locals.var_costi4_dn7 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn7)) * 2.0), (((locals.var_costi4_dn8 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn8)) * 2.0), (((locals.var_costi4_dn9 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn9)) * 2.0), (((locals.var_costi4_dn10 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn10)) * 2.0), (((locals.var_costi4_dn11 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn11)) * 2.0), (((locals.var_costi4_dn14 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn14)) * 2.0),)
    } else {
        (locals.var_costi5, locals.var_costi5_dn0, locals.var_costi5_dn2, locals.var_costi5_dn4, locals.var_costi5_dn5, locals.var_costi5_dn6, locals.var_costi5_dn7, locals.var_costi5_dn8, locals.var_costi5_dn9, locals.var_costi5_dn10, locals.var_costi5_dn11, locals.var_costi5_dn14,)
    }
};
        locals.var_costi5 = assign63260_e97987;
        locals.var_costi5_dn0 = assign63260_e97987_d_n0;
        locals.var_costi5_dn2 = assign63260_e97987_d_n2;
        locals.var_costi5_dn4 = assign63260_e97987_d_n4;
        locals.var_costi5_dn5 = assign63260_e97987_d_n5;
        locals.var_costi5_dn6 = assign63260_e97987_d_n6;
        locals.var_costi5_dn7 = assign63260_e97987_d_n7;
        locals.var_costi5_dn8 = assign63260_e97987_d_n8;
        locals.var_costi5_dn9 = assign63260_e97987_d_n9;
        locals.var_costi5_dn10 = assign63260_e97987_d_n10;
        locals.var_costi5_dn11 = assign63260_e97987_d_n11;
        locals.var_costi5_dn14 = assign63260_e97987_d_n14;
        locals.var_costi5_rv = 0.0;

        let (assign63270_e97998, assign63270_e97998_d_n0, assign63270_e97998_d_n2, assign63270_e97998_d_n4, assign63270_e97998_d_n5, assign63270_e97998_d_n6, assign63270_e97998_d_n7, assign63270_e97998_d_n8, assign63270_e97998_d_n9, assign63270_e97998_d_n10, assign63270_e97998_d_n11, assign63270_e97998_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63270_e97996: f64 = (locals.var_beta * 0.25);
        (assign63270_e97996, (locals.var_beta_dn0 * 0.25), (locals.var_beta_dn2 * 0.25), (locals.var_beta_dn4 * 0.25), (locals.var_beta_dn5 * 0.25), (locals.var_beta_dn6 * 0.25), (locals.var_beta_dn7 * 0.25), (locals.var_beta_dn8 * 0.25), (locals.var_beta_dn9 * 0.25), (locals.var_beta_dn10 * 0.25), (locals.var_beta_dn11 * 0.25), (locals.var_beta_dn14 * 0.25),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign63270_e97998;
        locals.var_t11_dn0 = assign63270_e97998_d_n0;
        locals.var_t11_dn2 = assign63270_e97998_d_n2;
        locals.var_t11_dn4 = assign63270_e97998_d_n4;
        locals.var_t11_dn5 = assign63270_e97998_d_n5;
        locals.var_t11_dn6 = assign63270_e97998_d_n6;
        locals.var_t11_dn7 = assign63270_e97998_d_n7;
        locals.var_t11_dn8 = assign63270_e97998_d_n8;
        locals.var_t11_dn9 = assign63270_e97998_d_n9;
        locals.var_t11_dn10 = assign63270_e97998_d_n10;
        locals.var_t11_dn11 = assign63270_e97998_d_n11;
        locals.var_t11_dn14 = assign63270_e97998_d_n14;
        locals.var_t11_rv = 0.0;

        let (assign63280_e98019, assign63280_e98019_d_n0, assign63280_e98019_d_n2, assign63280_e98019_d_n4, assign63280_e98019_d_n5, assign63280_e98019_d_n6, assign63280_e98019_d_n7, assign63280_e98019_d_n8, assign63280_e98019_d_n9, assign63280_e98019_d_n10, assign63280_e98019_d_n11, assign63280_e98019_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63280_e98008: f64 = (locals.var_costi3 * locals.var_t11);
        let assign63280_e98009: f64 = (locals.var_beta_inv - assign63280_e98008);
        let assign63280_e98011: f64 = (assign63280_e98009 + locals.var_vfb);
        let assign63280_e98013: f64 = (assign63280_e98011 - locals.var_uc_vthsti);
        let assign63280_e98015: f64 = (assign63280_e98013 - locals.var_dvthscsti);
        let assign63280_e98017: f64 = (assign63280_e98015 + 1e-25);
        (assign63280_e98017, ((locals.var_beta_inv_dn0 - ((locals.var_costi3_dn0 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn0))) - locals.var_dvthscsti_dn0), ((locals.var_beta_inv_dn2 - ((locals.var_costi3_dn2 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn2))) - locals.var_dvthscsti_dn2), ((locals.var_beta_inv_dn4 - ((locals.var_costi3_dn4 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn4))) - locals.var_dvthscsti_dn4), ((locals.var_beta_inv_dn5 - ((locals.var_costi3_dn5 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn5))) - locals.var_dvthscsti_dn5), ((locals.var_beta_inv_dn6 - ((locals.var_costi3_dn6 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn6))) - locals.var_dvthscsti_dn6), ((locals.var_beta_inv_dn7 - ((locals.var_costi3_dn7 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn7))) - locals.var_dvthscsti_dn7), ((locals.var_beta_inv_dn8 - ((locals.var_costi3_dn8 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn8))) - locals.var_dvthscsti_dn8), ((locals.var_beta_inv_dn9 - ((locals.var_costi3_dn9 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn9))) - locals.var_dvthscsti_dn9), ((locals.var_beta_inv_dn10 - ((locals.var_costi3_dn10 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn10))) - locals.var_dvthscsti_dn10), ((locals.var_beta_inv_dn11 - ((locals.var_costi3_dn11 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn11))) - locals.var_dvthscsti_dn11), ((locals.var_beta_inv_dn14 - ((locals.var_costi3_dn14 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn14))) - locals.var_dvthscsti_dn14),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign63280_e98019;
        locals.var_t10_dn0 = assign63280_e98019_d_n0;
        locals.var_t10_dn2 = assign63280_e98019_d_n2;
        locals.var_t10_dn4 = assign63280_e98019_d_n4;
        locals.var_t10_dn5 = assign63280_e98019_d_n5;
        locals.var_t10_dn6 = assign63280_e98019_d_n6;
        locals.var_t10_dn7 = assign63280_e98019_d_n7;
        locals.var_t10_dn8 = assign63280_e98019_d_n8;
        locals.var_t10_dn9 = assign63280_e98019_d_n9;
        locals.var_t10_dn10 = assign63280_e98019_d_n10;
        locals.var_t10_dn11 = assign63280_e98019_d_n11;
        locals.var_t10_dn14 = assign63280_e98019_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign63290_e98032, assign63290_e98032_d_n0, assign63290_e98032_d_n2, assign63290_e98032_d_n4, assign63290_e98032_d_n5, assign63290_e98032_d_n6, assign63290_e98032_d_n7, assign63290_e98032_d_n8, assign63290_e98032_d_n9, assign63290_e98032_d_n10, assign63290_e98032_d_n11, assign63290_e98032_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63290_e98028: f64 = (locals.var_vgsz__blk444 - locals.var_t10);
        let assign63290_e98030: f64 = (assign63290_e98028 - 0.005);
        (assign63290_e98030, (locals.var_vgsz__blk444_dn0 - locals.var_t10_dn0), (locals.var_vgsz__blk444_dn2 - locals.var_t10_dn2), (locals.var_vgsz__blk444_dn4 - locals.var_t10_dn4), (locals.var_vgsz__blk444_dn5 - locals.var_t10_dn5), (locals.var_vgsz__blk444_dn6 - locals.var_t10_dn6), (locals.var_vgsz__blk444_dn7 - locals.var_t10_dn7), (locals.var_vgsz__blk444_dn8 - locals.var_t10_dn8), (locals.var_vgsz__blk444_dn9 - locals.var_t10_dn9), (locals.var_vgsz__blk444_dn10 - locals.var_t10_dn10), (locals.var_vgsz__blk444_dn11 - locals.var_t10_dn11), (locals.var_vgsz__blk444_dn14 - locals.var_t10_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63290_e98032;
        locals.var_t1_dn0 = assign63290_e98032_d_n0;
        locals.var_t1_dn2 = assign63290_e98032_d_n2;
        locals.var_t1_dn4 = assign63290_e98032_d_n4;
        locals.var_t1_dn5 = assign63290_e98032_d_n5;
        locals.var_t1_dn6 = assign63290_e98032_d_n6;
        locals.var_t1_dn7 = assign63290_e98032_d_n7;
        locals.var_t1_dn8 = assign63290_e98032_d_n8;
        locals.var_t1_dn9 = assign63290_e98032_d_n9;
        locals.var_t1_dn10 = assign63290_e98032_d_n10;
        locals.var_t1_dn11 = assign63290_e98032_d_n11;
        locals.var_t1_dn14 = assign63290_e98032_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign63300_e98047, assign63300_e98047_d_n0, assign63300_e98047_d_n2, assign63300_e98047_d_n4, assign63300_e98047_d_n5, assign63300_e98047_d_n6, assign63300_e98047_d_n7, assign63300_e98047_d_n8, assign63300_e98047_d_n9, assign63300_e98047_d_n10, assign63300_e98047_d_n11, assign63300_e98047_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let (assign63300_e98045,) = {
            if (locals.var_t10 >= 0.0) {
                (1.0,)
            } else {
                let assign63300_e98044: f64 = (-1.0);
                (assign63300_e98044,)
            }
        };
        (assign63300_e98045, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign63300_e98047;
        locals.var_t0_dn0 = assign63300_e98047_d_n0;
        locals.var_t0_dn2 = assign63300_e98047_d_n2;
        locals.var_t0_dn4 = assign63300_e98047_d_n4;
        locals.var_t0_dn5 = assign63300_e98047_d_n5;
        locals.var_t0_dn6 = assign63300_e98047_d_n6;
        locals.var_t0_dn7 = assign63300_e98047_d_n7;
        locals.var_t0_dn8 = assign63300_e98047_d_n8;
        locals.var_t0_dn9 = assign63300_e98047_d_n9;
        locals.var_t0_dn10 = assign63300_e98047_d_n10;
        locals.var_t0_dn11 = assign63300_e98047_d_n11;
        locals.var_t0_dn14 = assign63300_e98047_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign63310_e98067, assign63310_e98067_d_n0, assign63310_e98067_d_n2, assign63310_e98067_d_n4, assign63310_e98067_d_n5, assign63310_e98067_d_n6, assign63310_e98067_d_n7, assign63310_e98067_d_n8, assign63310_e98067_d_n9, assign63310_e98067_d_n10, assign63310_e98067_d_n11, assign63310_e98067_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63310_e98056: f64 = (locals.var_t1 * locals.var_t1);
        let assign63310_e98059: f64 = (locals.var_t0 * 4.0);
        let assign63310_e98061: f64 = (assign63310_e98059 * locals.var_t10);
        let assign63310_e98063: f64 = (assign63310_e98061 * 0.005);
        let assign63310_e98064: f64 = (assign63310_e98056 + assign63310_e98063);
        let assign63310_e98065: f64 = (assign63310_e98064).sqrt();
        (assign63310_e98065, ((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) + ((((locals.var_t0_dn0 * 4.0) * locals.var_t10) + (assign63310_e98059 * locals.var_t10_dn0)) * 0.005)) / (2.0 * assign63310_e98065)), ((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) + ((((locals.var_t0_dn2 * 4.0) * locals.var_t10) + (assign63310_e98059 * locals.var_t10_dn2)) * 0.005)) / (2.0 * assign63310_e98065)), ((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) + ((((locals.var_t0_dn4 * 4.0) * locals.var_t10) + (assign63310_e98059 * locals.var_t10_dn4)) * 0.005)) / (2.0 * assign63310_e98065)), ((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) + ((((locals.var_t0_dn5 * 4.0) * locals.var_t10) + (assign63310_e98059 * locals.var_t10_dn5)) * 0.005)) / (2.0 * assign63310_e98065)), ((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + ((((locals.var_t0_dn6 * 4.0) * locals.var_t10) + (assign63310_e98059 * locals.var_t10_dn6)) * 0.005)) / (2.0 * assign63310_e98065)), ((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) + ((((locals.var_t0_dn7 * 4.0) * locals.var_t10) + (assign63310_e98059 * locals.var_t10_dn7)) * 0.005)) / (2.0 * assign63310_e98065)), ((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) + ((((locals.var_t0_dn8 * 4.0) * locals.var_t10) + (assign63310_e98059 * locals.var_t10_dn8)) * 0.005)) / (2.0 * assign63310_e98065)), ((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) + ((((locals.var_t0_dn9 * 4.0) * locals.var_t10) + (assign63310_e98059 * locals.var_t10_dn9)) * 0.005)) / (2.0 * assign63310_e98065)), ((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + ((((locals.var_t0_dn10 * 4.0) * locals.var_t10) + (assign63310_e98059 * locals.var_t10_dn10)) * 0.005)) / (2.0 * assign63310_e98065)), ((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) + ((((locals.var_t0_dn11 * 4.0) * locals.var_t10) + (assign63310_e98059 * locals.var_t10_dn11)) * 0.005)) / (2.0 * assign63310_e98065)), ((((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) + ((((locals.var_t0_dn14 * 4.0) * locals.var_t10) + (assign63310_e98059 * locals.var_t10_dn14)) * 0.005)) / (2.0 * assign63310_e98065)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign63310_e98067;
        locals.var_t2_dn0 = assign63310_e98067_d_n0;
        locals.var_t2_dn2 = assign63310_e98067_d_n2;
        locals.var_t2_dn4 = assign63310_e98067_d_n4;
        locals.var_t2_dn5 = assign63310_e98067_d_n5;
        locals.var_t2_dn6 = assign63310_e98067_d_n6;
        locals.var_t2_dn7 = assign63310_e98067_d_n7;
        locals.var_t2_dn8 = assign63310_e98067_d_n8;
        locals.var_t2_dn9 = assign63310_e98067_d_n9;
        locals.var_t2_dn10 = assign63310_e98067_d_n10;
        locals.var_t2_dn11 = assign63310_e98067_d_n11;
        locals.var_t2_dn14 = assign63310_e98067_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign63320_e98090, assign63320_e98090_d_n0, assign63320_e98090_d_n2, assign63320_e98090_d_n4, assign63320_e98090_d_n5, assign63320_e98090_d_n6, assign63320_e98090_d_n7, assign63320_e98090_d_n8, assign63320_e98090_d_n9, assign63320_e98090_d_n10, assign63320_e98090_d_n11, assign63320_e98090_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63320_e98078: f64 = (locals.var_t1 + locals.var_t2);
        let assign63320_e98079: f64 = (0.5 * assign63320_e98078);
        let assign63320_e98080: f64 = (locals.var_t10 + assign63320_e98079);
        let assign63320_e98082: f64 = (assign63320_e98080 - locals.var_vfb);
        let assign63320_e98084: f64 = (assign63320_e98082 + locals.var_uc_vthsti);
        let assign63320_e98086: f64 = (assign63320_e98084 + locals.var_dvthscsti);
        let assign63320_e98088: f64 = (assign63320_e98086 - locals.var_vbsz__blk442);
        (assign63320_e98088, (((locals.var_t10_dn0 + (0.5 * (locals.var_t1_dn0 + locals.var_t2_dn0))) + locals.var_dvthscsti_dn0) - locals.var_vbsz__blk442_dn0), (((locals.var_t10_dn2 + (0.5 * (locals.var_t1_dn2 + locals.var_t2_dn2))) + locals.var_dvthscsti_dn2) - locals.var_vbsz__blk442_dn2), (((locals.var_t10_dn4 + (0.5 * (locals.var_t1_dn4 + locals.var_t2_dn4))) + locals.var_dvthscsti_dn4) - locals.var_vbsz__blk442_dn4), (((locals.var_t10_dn5 + (0.5 * (locals.var_t1_dn5 + locals.var_t2_dn5))) + locals.var_dvthscsti_dn5) - locals.var_vbsz__blk442_dn5), (((locals.var_t10_dn6 + (0.5 * (locals.var_t1_dn6 + locals.var_t2_dn6))) + locals.var_dvthscsti_dn6) - locals.var_vbsz__blk442_dn6), (((locals.var_t10_dn7 + (0.5 * (locals.var_t1_dn7 + locals.var_t2_dn7))) + locals.var_dvthscsti_dn7) - locals.var_vbsz__blk442_dn7), (((locals.var_t10_dn8 + (0.5 * (locals.var_t1_dn8 + locals.var_t2_dn8))) + locals.var_dvthscsti_dn8) - locals.var_vbsz__blk442_dn8), (((locals.var_t10_dn9 + (0.5 * (locals.var_t1_dn9 + locals.var_t2_dn9))) + locals.var_dvthscsti_dn9) - locals.var_vbsz__blk442_dn9), (((locals.var_t10_dn10 + (0.5 * (locals.var_t1_dn10 + locals.var_t2_dn10))) + locals.var_dvthscsti_dn10) - locals.var_vbsz__blk442_dn10), (((locals.var_t10_dn11 + (0.5 * (locals.var_t1_dn11 + locals.var_t2_dn11))) + locals.var_dvthscsti_dn11) - locals.var_vbsz__blk442_dn11), (((locals.var_t10_dn14 + (0.5 * (locals.var_t1_dn14 + locals.var_t2_dn14))) + locals.var_dvthscsti_dn14) - locals.var_vbsz__blk442_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign63320_e98090;
        locals.var_t3_dn0 = assign63320_e98090_d_n0;
        locals.var_t3_dn2 = assign63320_e98090_d_n2;
        locals.var_t3_dn4 = assign63320_e98090_d_n4;
        locals.var_t3_dn5 = assign63320_e98090_d_n5;
        locals.var_t3_dn6 = assign63320_e98090_d_n6;
        locals.var_t3_dn7 = assign63320_e98090_d_n7;
        locals.var_t3_dn8 = assign63320_e98090_d_n8;
        locals.var_t3_dn9 = assign63320_e98090_d_n9;
        locals.var_t3_dn10 = assign63320_e98090_d_n10;
        locals.var_t3_dn11 = assign63320_e98090_d_n11;
        locals.var_t3_dn14 = assign63320_e98090_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign63330_e98103, assign63330_e98103_d_n0, assign63330_e98103_d_n2, assign63330_e98103_d_n4, assign63330_e98103_d_n5, assign63330_e98103_d_n6, assign63330_e98103_d_n7, assign63330_e98103_d_n8, assign63330_e98103_d_n9, assign63330_e98103_d_n10, assign63330_e98103_d_n11, assign63330_e98103_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63330_e98099: f64 = (locals.var_beta * locals.var_t3);
        let assign63330_e98101: f64 = (assign63330_e98099 - 1.0);
        (assign63330_e98101, ((locals.var_beta_dn0 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn0)), ((locals.var_beta_dn2 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn2)), ((locals.var_beta_dn4 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn4)), ((locals.var_beta_dn5 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn5)), ((locals.var_beta_dn6 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn6)), ((locals.var_beta_dn7 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn7)), ((locals.var_beta_dn8 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn8)), ((locals.var_beta_dn9 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn9)), ((locals.var_beta_dn10 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn10)), ((locals.var_beta_dn11 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn11)), ((locals.var_beta_dn14 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign63330_e98103;
        locals.var_t4_dn0 = assign63330_e98103_d_n0;
        locals.var_t4_dn2 = assign63330_e98103_d_n2;
        locals.var_t4_dn4 = assign63330_e98103_d_n4;
        locals.var_t4_dn5 = assign63330_e98103_d_n5;
        locals.var_t4_dn6 = assign63330_e98103_d_n6;
        locals.var_t4_dn7 = assign63330_e98103_d_n7;
        locals.var_t4_dn8 = assign63330_e98103_d_n8;
        locals.var_t4_dn9 = assign63330_e98103_d_n9;
        locals.var_t4_dn10 = assign63330_e98103_d_n10;
        locals.var_t4_dn11 = assign63330_e98103_d_n11;
        locals.var_t4_dn14 = assign63330_e98103_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign63340_e98114, assign63340_e98114_d_n0, assign63340_e98114_d_n2, assign63340_e98114_d_n4, assign63340_e98114_d_n5, assign63340_e98114_d_n6, assign63340_e98114_d_n7, assign63340_e98114_d_n8, assign63340_e98114_d_n9, assign63340_e98114_d_n10, assign63340_e98114_d_n11, assign63340_e98114_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63340_e98112: f64 = (4.0 / locals.var_costi5);
        (assign63340_e98112, (-((4.0 * locals.var_costi5_dn0) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn2) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn4) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn5) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn6) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn7) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn8) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn9) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn10) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn11) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn14) / (locals.var_costi5 * locals.var_costi5))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign63340_e98114;
        locals.var_t5_dn0 = assign63340_e98114_d_n0;
        locals.var_t5_dn2 = assign63340_e98114_d_n2;
        locals.var_t5_dn4 = assign63340_e98114_d_n4;
        locals.var_t5_dn5 = assign63340_e98114_d_n5;
        locals.var_t5_dn6 = assign63340_e98114_d_n6;
        locals.var_t5_dn7 = assign63340_e98114_d_n7;
        locals.var_t5_dn8 = assign63340_e98114_d_n8;
        locals.var_t5_dn9 = assign63340_e98114_d_n9;
        locals.var_t5_dn10 = assign63340_e98114_d_n10;
        locals.var_t5_dn11 = assign63340_e98114_d_n11;
        locals.var_t5_dn14 = assign63340_e98114_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign63350_e98127, assign63350_e98127_d_n0, assign63350_e98127_d_n2, assign63350_e98127_d_n4, assign63350_e98127_d_n5, assign63350_e98127_d_n6, assign63350_e98127_d_n7, assign63350_e98127_d_n8, assign63350_e98127_d_n9, assign63350_e98127_d_n10, assign63350_e98127_d_n11, assign63350_e98127_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63350_e98124: f64 = (locals.var_t4 * locals.var_t5);
        let assign63350_e98125: f64 = (1.0 + assign63350_e98124);
        (assign63350_e98125, ((locals.var_t4_dn0 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn0)), ((locals.var_t4_dn2 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn2)), ((locals.var_t4_dn4 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn4)), ((locals.var_t4_dn5 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn5)), ((locals.var_t4_dn6 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn6)), ((locals.var_t4_dn7 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn7)), ((locals.var_t4_dn8 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn8)), ((locals.var_t4_dn9 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn9)), ((locals.var_t4_dn10 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn10)), ((locals.var_t4_dn11 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn11)), ((locals.var_t4_dn14 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63350_e98127;
        locals.var_t1_dn0 = assign63350_e98127_d_n0;
        locals.var_t1_dn2 = assign63350_e98127_d_n2;
        locals.var_t1_dn4 = assign63350_e98127_d_n4;
        locals.var_t1_dn5 = assign63350_e98127_d_n5;
        locals.var_t1_dn6 = assign63350_e98127_d_n6;
        locals.var_t1_dn7 = assign63350_e98127_d_n7;
        locals.var_t1_dn8 = assign63350_e98127_d_n8;
        locals.var_t1_dn9 = assign63350_e98127_d_n9;
        locals.var_t1_dn10 = assign63350_e98127_d_n10;
        locals.var_t1_dn11 = assign63350_e98127_d_n11;
        locals.var_t1_dn14 = assign63350_e98127_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign63360_e98138, assign63360_e98138_d_n0, assign63360_e98138_d_n2, assign63360_e98138_d_n4, assign63360_e98138_d_n5, assign63360_e98138_d_n6, assign63360_e98138_d_n7, assign63360_e98138_d_n8, assign63360_e98138_d_n9, assign63360_e98138_d_n10, assign63360_e98138_d_n11, assign63360_e98138_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63360_e98136: f64 = (locals.var_beta * locals.var_t5);
        (assign63360_e98136, ((locals.var_beta_dn0 * locals.var_t5) + (locals.var_beta * locals.var_t5_dn0)), ((locals.var_beta_dn2 * locals.var_t5) + (locals.var_beta * locals.var_t5_dn2)), ((locals.var_beta_dn4 * locals.var_t5) + (locals.var_beta * locals.var_t5_dn4)), ((locals.var_beta_dn5 * locals.var_t5) + (locals.var_beta * locals.var_t5_dn5)), ((locals.var_beta_dn6 * locals.var_t5) + (locals.var_beta * locals.var_t5_dn6)), ((locals.var_beta_dn7 * locals.var_t5) + (locals.var_beta * locals.var_t5_dn7)), ((locals.var_beta_dn8 * locals.var_t5) + (locals.var_beta * locals.var_t5_dn8)), ((locals.var_beta_dn9 * locals.var_t5) + (locals.var_beta * locals.var_t5_dn9)), ((locals.var_beta_dn10 * locals.var_t5) + (locals.var_beta * locals.var_t5_dn10)), ((locals.var_beta_dn11 * locals.var_t5) + (locals.var_beta * locals.var_t5_dn11)), ((locals.var_beta_dn14 * locals.var_t5) + (locals.var_beta * locals.var_t5_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign63360_e98138;
        locals.var_t6_dn0 = assign63360_e98138_d_n0;
        locals.var_t6_dn2 = assign63360_e98138_d_n2;
        locals.var_t6_dn4 = assign63360_e98138_d_n4;
        locals.var_t6_dn5 = assign63360_e98138_d_n5;
        locals.var_t6_dn6 = assign63360_e98138_d_n6;
        locals.var_t6_dn7 = assign63360_e98138_d_n7;
        locals.var_t6_dn8 = assign63360_e98138_d_n8;
        locals.var_t6_dn9 = assign63360_e98138_d_n9;
        locals.var_t6_dn10 = assign63360_e98138_d_n10;
        locals.var_t6_dn11 = assign63360_e98138_d_n11;
        locals.var_t6_dn14 = assign63360_e98138_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign63370_e98149, assign63370_e98149_d_n0, assign63370_e98149_d_n2, assign63370_e98149_d_n4, assign63370_e98149_d_n5, assign63370_e98149_d_n6, assign63370_e98149_d_n7, assign63370_e98149_d_n8, assign63370_e98149_d_n9, assign63370_e98149_d_n10, assign63370_e98149_d_n11, assign63370_e98149_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63370_e98147: f64 = (locals.var_t4 * locals.var_t5);
        (assign63370_e98147, ((locals.var_t4_dn0 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn0)), ((locals.var_t4_dn2 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn2)), ((locals.var_t4_dn4 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn4)), ((locals.var_t4_dn5 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn5)), ((locals.var_t4_dn6 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn6)), ((locals.var_t4_dn7 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn7)), ((locals.var_t4_dn8 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn8)), ((locals.var_t4_dn9 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn9)), ((locals.var_t4_dn10 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn10)), ((locals.var_t4_dn11 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn11)), ((locals.var_t4_dn14 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn14)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign63370_e98149;
        locals.var_t7_dn0 = assign63370_e98149_d_n0;
        locals.var_t7_dn2 = assign63370_e98149_d_n2;
        locals.var_t7_dn4 = assign63370_e98149_d_n4;
        locals.var_t7_dn5 = assign63370_e98149_d_n5;
        locals.var_t7_dn6 = assign63370_e98149_d_n6;
        locals.var_t7_dn7 = assign63370_e98149_d_n7;
        locals.var_t7_dn8 = assign63370_e98149_d_n8;
        locals.var_t7_dn9 = assign63370_e98149_d_n9;
        locals.var_t7_dn10 = assign63370_e98149_d_n10;
        locals.var_t7_dn11 = assign63370_e98149_d_n11;
        locals.var_t7_dn14 = assign63370_e98149_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign63380_e98167, assign63380_e98167_d_n0, assign63380_e98167_d_n2, assign63380_e98167_d_n4, assign63380_e98167_d_n5, assign63380_e98167_d_n6, assign63380_e98167_d_n7, assign63380_e98167_d_n8, assign63380_e98167_d_n9, assign63380_e98167_d_n10, assign63380_e98167_d_n11, assign63380_e98167_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63380_e98158: f64 = (locals.var_t1 * locals.var_t1);
        let assign63380_e98161: f64 = (4.0 * 0.01);
        let assign63380_e98163: f64 = (assign63380_e98161 * 0.01);
        let assign63380_e98164: f64 = (assign63380_e98158 + assign63380_e98163);
        let assign63380_e98165: f64 = (assign63380_e98164).sqrt();
        (assign63380_e98165, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign63380_e98165)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign63380_e98165)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign63380_e98165)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign63380_e98165)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign63380_e98165)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign63380_e98165)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign63380_e98165)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign63380_e98165)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign63380_e98165)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign63380_e98165)), (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign63380_e98165)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign63380_e98167;
        locals.var_tmf2_dn0 = assign63380_e98167_d_n0;
        locals.var_tmf2_dn2 = assign63380_e98167_d_n2;
        locals.var_tmf2_dn4 = assign63380_e98167_d_n4;
        locals.var_tmf2_dn5 = assign63380_e98167_d_n5;
        locals.var_tmf2_dn6 = assign63380_e98167_d_n6;
        locals.var_tmf2_dn7 = assign63380_e98167_d_n7;
        locals.var_tmf2_dn8 = assign63380_e98167_d_n8;
        locals.var_tmf2_dn9 = assign63380_e98167_d_n9;
        locals.var_tmf2_dn10 = assign63380_e98167_d_n10;
        locals.var_tmf2_dn11 = assign63380_e98167_d_n11;
        locals.var_tmf2_dn14 = assign63380_e98167_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign63390_e98182, assign63390_e98182_d_n0, assign63390_e98182_d_n2, assign63390_e98182_d_n4, assign63390_e98182_d_n5, assign63390_e98182_d_n6, assign63390_e98182_d_n7, assign63390_e98182_d_n8, assign63390_e98182_d_n9, assign63390_e98182_d_n10, assign63390_e98182_d_n11, assign63390_e98182_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63390_e98178: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign63390_e98179: f64 = (1.0 + assign63390_e98178);
        let assign63390_e98180: f64 = (0.5 * assign63390_e98179);
        (assign63390_e98180, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn14 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign63390_e98182;
        locals.var_t2_dn0 = assign63390_e98182_d_n0;
        locals.var_t2_dn2 = assign63390_e98182_d_n2;
        locals.var_t2_dn4 = assign63390_e98182_d_n4;
        locals.var_t2_dn5 = assign63390_e98182_d_n5;
        locals.var_t2_dn6 = assign63390_e98182_d_n6;
        locals.var_t2_dn7 = assign63390_e98182_d_n7;
        locals.var_t2_dn8 = assign63390_e98182_d_n8;
        locals.var_t2_dn9 = assign63390_e98182_d_n9;
        locals.var_t2_dn10 = assign63390_e98182_d_n10;
        locals.var_t2_dn11 = assign63390_e98182_d_n11;
        locals.var_t2_dn14 = assign63390_e98182_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign63400_e98195, assign63400_e98195_d_n0, assign63400_e98195_d_n2, assign63400_e98195_d_n4, assign63400_e98195_d_n5, assign63400_e98195_d_n6, assign63400_e98195_d_n7, assign63400_e98195_d_n8, assign63400_e98195_d_n9, assign63400_e98195_d_n10, assign63400_e98195_d_n11, assign63400_e98195_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63400_e98192: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign63400_e98193: f64 = (0.5 * assign63400_e98192);
        (assign63400_e98193, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63400_e98195;
        locals.var_t1_dn0 = assign63400_e98195_d_n0;
        locals.var_t1_dn2 = assign63400_e98195_d_n2;
        locals.var_t1_dn4 = assign63400_e98195_d_n4;
        locals.var_t1_dn5 = assign63400_e98195_d_n5;
        locals.var_t1_dn6 = assign63400_e98195_d_n6;
        locals.var_t1_dn7 = assign63400_e98195_d_n7;
        locals.var_t1_dn8 = assign63400_e98195_d_n8;
        locals.var_t1_dn9 = assign63400_e98195_d_n9;
        locals.var_t1_dn10 = assign63400_e98195_d_n10;
        locals.var_t1_dn11 = assign63400_e98195_d_n11;
        locals.var_t1_dn14 = assign63400_e98195_d_n14;
        locals.var_t1_rv = 0.0;

        let assign63410_e98198: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1508 = assign63410_e98198;
        locals.var_guard1508_rv = 0.0;

        let (assign63420_e98209, assign63420_e98209_d_n0, assign63420_e98209_d_n2, assign63420_e98209_d_n4, assign63420_e98209_d_n5, assign63420_e98209_d_n6, assign63420_e98209_d_n7, assign63420_e98209_d_n8, assign63420_e98209_d_n9, assign63420_e98209_d_n10, assign63420_e98209_d_n11, assign63420_e98209_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) && (locals.var_guard1508 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63420_e98209;
        locals.var_t1_dn0 = assign63420_e98209_d_n0;
        locals.var_t1_dn2 = assign63420_e98209_d_n2;
        locals.var_t1_dn4 = assign63420_e98209_d_n4;
        locals.var_t1_dn5 = assign63420_e98209_d_n5;
        locals.var_t1_dn6 = assign63420_e98209_d_n6;
        locals.var_t1_dn7 = assign63420_e98209_d_n7;
        locals.var_t1_dn8 = assign63420_e98209_d_n8;
        locals.var_t1_dn9 = assign63420_e98209_d_n9;
        locals.var_t1_dn10 = assign63420_e98209_d_n10;
        locals.var_t1_dn11 = assign63420_e98209_d_n11;
        locals.var_t1_dn14 = assign63420_e98209_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_237(
        locals: &mut StampLocals,
    ) {
        let (assign63430_e98220, assign63430_e98220_d_n0, assign63430_e98220_d_n2, assign63430_e98220_d_n4, assign63430_e98220_d_n5, assign63430_e98220_d_n6, assign63430_e98220_d_n7, assign63430_e98220_d_n8, assign63430_e98220_d_n9, assign63430_e98220_d_n10, assign63430_e98220_d_n11, assign63430_e98220_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) && (locals.var_guard1508 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign63430_e98220;
        locals.var_t2_dn0 = assign63430_e98220_d_n0;
        locals.var_t2_dn2 = assign63430_e98220_d_n2;
        locals.var_t2_dn4 = assign63430_e98220_d_n4;
        locals.var_t2_dn5 = assign63430_e98220_d_n5;
        locals.var_t2_dn6 = assign63430_e98220_d_n6;
        locals.var_t2_dn7 = assign63430_e98220_d_n7;
        locals.var_t2_dn8 = assign63430_e98220_d_n8;
        locals.var_t2_dn9 = assign63430_e98220_d_n9;
        locals.var_t2_dn10 = assign63430_e98220_d_n10;
        locals.var_t2_dn11 = assign63430_e98220_d_n11;
        locals.var_t2_dn14 = assign63430_e98220_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign63440_e98231, assign63440_e98231_d_n0, assign63440_e98231_d_n2, assign63440_e98231_d_n4, assign63440_e98231_d_n5, assign63440_e98231_d_n6, assign63440_e98231_d_n7, assign63440_e98231_d_n8, assign63440_e98231_d_n9, assign63440_e98231_d_n10, assign63440_e98231_d_n11, assign63440_e98231_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63440_e98229: f64 = (locals.var_t1 + 1e-25);
        (assign63440_e98229, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63440_e98231;
        locals.var_t1_dn0 = assign63440_e98231_d_n0;
        locals.var_t1_dn2 = assign63440_e98231_d_n2;
        locals.var_t1_dn4 = assign63440_e98231_d_n4;
        locals.var_t1_dn5 = assign63440_e98231_d_n5;
        locals.var_t1_dn6 = assign63440_e98231_d_n6;
        locals.var_t1_dn7 = assign63440_e98231_d_n7;
        locals.var_t1_dn8 = assign63440_e98231_d_n8;
        locals.var_t1_dn9 = assign63440_e98231_d_n9;
        locals.var_t1_dn10 = assign63440_e98231_d_n10;
        locals.var_t1_dn11 = assign63440_e98231_d_n11;
        locals.var_t1_dn14 = assign63440_e98231_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign63450_e98241, assign63450_e98241_d_n0, assign63450_e98241_d_n2, assign63450_e98241_d_n4, assign63450_e98241_d_n5, assign63450_e98241_d_n6, assign63450_e98241_d_n7, assign63450_e98241_d_n8, assign63450_e98241_d_n9, assign63450_e98241_d_n10, assign63450_e98241_d_n11, assign63450_e98241_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63450_e98239: f64 = (locals.var_t1).sqrt();
        (assign63450_e98239, (locals.var_t1_dn0 / (2.0 * assign63450_e98239)), (locals.var_t1_dn2 / (2.0 * assign63450_e98239)), (locals.var_t1_dn4 / (2.0 * assign63450_e98239)), (locals.var_t1_dn5 / (2.0 * assign63450_e98239)), (locals.var_t1_dn6 / (2.0 * assign63450_e98239)), (locals.var_t1_dn7 / (2.0 * assign63450_e98239)), (locals.var_t1_dn8 / (2.0 * assign63450_e98239)), (locals.var_t1_dn9 / (2.0 * assign63450_e98239)), (locals.var_t1_dn10 / (2.0 * assign63450_e98239)), (locals.var_t1_dn11 / (2.0 * assign63450_e98239)), (locals.var_t1_dn14 / (2.0 * assign63450_e98239)),)
    } else {
        (locals.var_costi6, locals.var_costi6_dn0, locals.var_costi6_dn2, locals.var_costi6_dn4, locals.var_costi6_dn5, locals.var_costi6_dn6, locals.var_costi6_dn7, locals.var_costi6_dn8, locals.var_costi6_dn9, locals.var_costi6_dn10, locals.var_costi6_dn11, locals.var_costi6_dn14,)
    }
};
        locals.var_costi6 = assign63450_e98241;
        locals.var_costi6_dn0 = assign63450_e98241_d_n0;
        locals.var_costi6_dn2 = assign63450_e98241_d_n2;
        locals.var_costi6_dn4 = assign63450_e98241_d_n4;
        locals.var_costi6_dn5 = assign63450_e98241_d_n5;
        locals.var_costi6_dn6 = assign63450_e98241_d_n6;
        locals.var_costi6_dn7 = assign63450_e98241_d_n7;
        locals.var_costi6_dn8 = assign63450_e98241_d_n8;
        locals.var_costi6_dn9 = assign63450_e98241_d_n9;
        locals.var_costi6_dn10 = assign63450_e98241_d_n10;
        locals.var_costi6_dn11 = assign63450_e98241_d_n11;
        locals.var_costi6_dn14 = assign63450_e98241_d_n14;
        locals.var_costi6_rv = 0.0;

        let (assign63460_e98254, assign63460_e98254_d_n0, assign63460_e98254_d_n2, assign63460_e98254_d_n4, assign63460_e98254_d_n5, assign63460_e98254_d_n6, assign63460_e98254_d_n7, assign63460_e98254_d_n8, assign63460_e98254_d_n9, assign63460_e98254_d_n10, assign63460_e98254_d_n11, assign63460_e98254_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63460_e98251: f64 = (1.0 - locals.var_costi6);
        let assign63460_e98252: f64 = (locals.var_costi4 * assign63460_e98251);
        (assign63460_e98252, ((locals.var_costi4_dn0 * assign63460_e98251) + (locals.var_costi4 * (-locals.var_costi6_dn0))), ((locals.var_costi4_dn2 * assign63460_e98251) + (locals.var_costi4 * (-locals.var_costi6_dn2))), ((locals.var_costi4_dn4 * assign63460_e98251) + (locals.var_costi4 * (-locals.var_costi6_dn4))), ((locals.var_costi4_dn5 * assign63460_e98251) + (locals.var_costi4 * (-locals.var_costi6_dn5))), ((locals.var_costi4_dn6 * assign63460_e98251) + (locals.var_costi4 * (-locals.var_costi6_dn6))), ((locals.var_costi4_dn7 * assign63460_e98251) + (locals.var_costi4 * (-locals.var_costi6_dn7))), ((locals.var_costi4_dn8 * assign63460_e98251) + (locals.var_costi4 * (-locals.var_costi6_dn8))), ((locals.var_costi4_dn9 * assign63460_e98251) + (locals.var_costi4 * (-locals.var_costi6_dn9))), ((locals.var_costi4_dn10 * assign63460_e98251) + (locals.var_costi4 * (-locals.var_costi6_dn10))), ((locals.var_costi4_dn11 * assign63460_e98251) + (locals.var_costi4 * (-locals.var_costi6_dn11))), ((locals.var_costi4_dn14 * assign63460_e98251) + (locals.var_costi4 * (-locals.var_costi6_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign63460_e98254;
        locals.var_t0_dn0 = assign63460_e98254_d_n0;
        locals.var_t0_dn2 = assign63460_e98254_d_n2;
        locals.var_t0_dn4 = assign63460_e98254_d_n4;
        locals.var_t0_dn5 = assign63460_e98254_d_n5;
        locals.var_t0_dn6 = assign63460_e98254_d_n6;
        locals.var_t0_dn7 = assign63460_e98254_d_n7;
        locals.var_t0_dn8 = assign63460_e98254_d_n8;
        locals.var_t0_dn9 = assign63460_e98254_d_n9;
        locals.var_t0_dn10 = assign63460_e98254_d_n10;
        locals.var_t0_dn11 = assign63460_e98254_d_n11;
        locals.var_t0_dn14 = assign63460_e98254_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign63470_e98265, assign63470_e98265_d_n0, assign63470_e98265_d_n2, assign63470_e98265_d_n4, assign63470_e98265_d_n5, assign63470_e98265_d_n6, assign63470_e98265_d_n7, assign63470_e98265_d_n8, assign63470_e98265_d_n9, assign63470_e98265_d_n10, assign63470_e98265_d_n11, assign63470_e98265_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63470_e98263: f64 = (locals.var_vgssti + locals.var_t0);
        (assign63470_e98263, (locals.var_vgssti_dn0 + locals.var_t0_dn0), (locals.var_vgssti_dn2 + locals.var_t0_dn2), (locals.var_vgssti_dn4 + locals.var_t0_dn4), (locals.var_vgssti_dn5 + locals.var_t0_dn5), (locals.var_vgssti_dn6 + locals.var_t0_dn6), (locals.var_vgssti_dn7 + locals.var_t0_dn7), (locals.var_vgssti_dn8 + locals.var_t0_dn8), (locals.var_vgssti_dn9 + locals.var_t0_dn9), (locals.var_vgssti_dn10 + locals.var_t0_dn10), (locals.var_vgssti_dn11 + locals.var_t0_dn11), (locals.var_vgssti_dn14 + locals.var_t0_dn14),)
    } else {
        (locals.var_psasti, locals.var_psasti_dn0, locals.var_psasti_dn2, locals.var_psasti_dn4, locals.var_psasti_dn5, locals.var_psasti_dn6, locals.var_psasti_dn7, locals.var_psasti_dn8, locals.var_psasti_dn9, locals.var_psasti_dn10, locals.var_psasti_dn11, locals.var_psasti_dn14,)
    }
};
        locals.var_psasti = assign63470_e98265;
        locals.var_psasti_dn0 = assign63470_e98265_d_n0;
        locals.var_psasti_dn2 = assign63470_e98265_d_n2;
        locals.var_psasti_dn4 = assign63470_e98265_d_n4;
        locals.var_psasti_dn5 = assign63470_e98265_d_n5;
        locals.var_psasti_dn6 = assign63470_e98265_d_n6;
        locals.var_psasti_dn7 = assign63470_e98265_d_n7;
        locals.var_psasti_dn8 = assign63470_e98265_d_n8;
        locals.var_psasti_dn9 = assign63470_e98265_d_n9;
        locals.var_psasti_dn10 = assign63470_e98265_d_n10;
        locals.var_psasti_dn11 = assign63470_e98265_d_n11;
        locals.var_psasti_dn14 = assign63470_e98265_d_n14;
        locals.var_psasti_rv = 0.0;

        let (assign63480_e98282, assign63480_e98282_d_n0, assign63480_e98282_d_n2, assign63480_e98282_d_n4, assign63480_e98282_d_n5, assign63480_e98282_d_n6, assign63480_e98282_d_n7, assign63480_e98282_d_n8, assign63480_e98282_d_n9, assign63480_e98282_d_n10, assign63480_e98282_d_n11, assign63480_e98282_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63480_e98277: f64 = (locals.var_vgssti + 1e-25);
        let assign63480_e98278: f64 = (2.0 / assign63480_e98277);
        let assign63480_e98279: f64 = (locals.var_beta + assign63480_e98278);
        let assign63480_e98280: f64 = (1.0 / assign63480_e98279);
        (assign63480_e98280, (-((locals.var_beta_dn0 + (-((2.0 * locals.var_vgssti_dn0) / (assign63480_e98277 * assign63480_e98277)))) / (assign63480_e98279 * assign63480_e98279))), (-((locals.var_beta_dn2 + (-((2.0 * locals.var_vgssti_dn2) / (assign63480_e98277 * assign63480_e98277)))) / (assign63480_e98279 * assign63480_e98279))), (-((locals.var_beta_dn4 + (-((2.0 * locals.var_vgssti_dn4) / (assign63480_e98277 * assign63480_e98277)))) / (assign63480_e98279 * assign63480_e98279))), (-((locals.var_beta_dn5 + (-((2.0 * locals.var_vgssti_dn5) / (assign63480_e98277 * assign63480_e98277)))) / (assign63480_e98279 * assign63480_e98279))), (-((locals.var_beta_dn6 + (-((2.0 * locals.var_vgssti_dn6) / (assign63480_e98277 * assign63480_e98277)))) / (assign63480_e98279 * assign63480_e98279))), (-((locals.var_beta_dn7 + (-((2.0 * locals.var_vgssti_dn7) / (assign63480_e98277 * assign63480_e98277)))) / (assign63480_e98279 * assign63480_e98279))), (-((locals.var_beta_dn8 + (-((2.0 * locals.var_vgssti_dn8) / (assign63480_e98277 * assign63480_e98277)))) / (assign63480_e98279 * assign63480_e98279))), (-((locals.var_beta_dn9 + (-((2.0 * locals.var_vgssti_dn9) / (assign63480_e98277 * assign63480_e98277)))) / (assign63480_e98279 * assign63480_e98279))), (-((locals.var_beta_dn10 + (-((2.0 * locals.var_vgssti_dn10) / (assign63480_e98277 * assign63480_e98277)))) / (assign63480_e98279 * assign63480_e98279))), (-((locals.var_beta_dn11 + (-((2.0 * locals.var_vgssti_dn11) / (assign63480_e98277 * assign63480_e98277)))) / (assign63480_e98279 * assign63480_e98279))), (-((locals.var_beta_dn14 + (-((2.0 * locals.var_vgssti_dn14) / (assign63480_e98277 * assign63480_e98277)))) / (assign63480_e98279 * assign63480_e98279))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign63480_e98282;
        locals.var_t0_dn0 = assign63480_e98282_d_n0;
        locals.var_t0_dn2 = assign63480_e98282_d_n2;
        locals.var_t0_dn4 = assign63480_e98282_d_n4;
        locals.var_t0_dn5 = assign63480_e98282_d_n5;
        locals.var_t0_dn6 = assign63480_e98282_d_n6;
        locals.var_t0_dn7 = assign63480_e98282_d_n7;
        locals.var_t0_dn8 = assign63480_e98282_d_n8;
        locals.var_t0_dn9 = assign63480_e98282_d_n9;
        locals.var_t0_dn10 = assign63480_e98282_d_n10;
        locals.var_t0_dn11 = assign63480_e98282_d_n11;
        locals.var_t0_dn14 = assign63480_e98282_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign63490_e98302, assign63490_e98302_d_n0, assign63490_e98302_d_n2, assign63490_e98302_d_n4, assign63490_e98302_d_n5, assign63490_e98302_d_n6, assign63490_e98302_d_n7, assign63490_e98302_d_n8, assign63490_e98302_d_n9, assign63490_e98302_d_n10, assign63490_e98302_d_n11, assign63490_e98302_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63490_e98291: f64 = (1.0 / locals.var_costi1);
        let assign63490_e98293: f64 = (assign63490_e98291 / locals.var_costi3);
        let assign63490_e98296: f64 = (locals.var_vgssti * locals.var_vgssti);
        let assign63490_e98297: f64 = (assign63490_e98293 * assign63490_e98296);
        let assign63490_e98298: f64 = (assign63490_e98297).ln();
        let assign63490_e98300: f64 = (assign63490_e98298 * locals.var_t0);
        (assign63490_e98300, (((((((((-(locals.var_costi1_dn0 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign63490_e98291 * locals.var_costi3_dn0)) / (locals.var_costi3 * locals.var_costi3)) * assign63490_e98296) + (assign63490_e98293 * ((locals.var_vgssti_dn0 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn0)))) / assign63490_e98297) * locals.var_t0) + (assign63490_e98298 * locals.var_t0_dn0)), (((((((((-(locals.var_costi1_dn2 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign63490_e98291 * locals.var_costi3_dn2)) / (locals.var_costi3 * locals.var_costi3)) * assign63490_e98296) + (assign63490_e98293 * ((locals.var_vgssti_dn2 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn2)))) / assign63490_e98297) * locals.var_t0) + (assign63490_e98298 * locals.var_t0_dn2)), (((((((((-(locals.var_costi1_dn4 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign63490_e98291 * locals.var_costi3_dn4)) / (locals.var_costi3 * locals.var_costi3)) * assign63490_e98296) + (assign63490_e98293 * ((locals.var_vgssti_dn4 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn4)))) / assign63490_e98297) * locals.var_t0) + (assign63490_e98298 * locals.var_t0_dn4)), (((((((((-(locals.var_costi1_dn5 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign63490_e98291 * locals.var_costi3_dn5)) / (locals.var_costi3 * locals.var_costi3)) * assign63490_e98296) + (assign63490_e98293 * ((locals.var_vgssti_dn5 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn5)))) / assign63490_e98297) * locals.var_t0) + (assign63490_e98298 * locals.var_t0_dn5)), (((((((((-(locals.var_costi1_dn6 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign63490_e98291 * locals.var_costi3_dn6)) / (locals.var_costi3 * locals.var_costi3)) * assign63490_e98296) + (assign63490_e98293 * ((locals.var_vgssti_dn6 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn6)))) / assign63490_e98297) * locals.var_t0) + (assign63490_e98298 * locals.var_t0_dn6)), (((((((((-(locals.var_costi1_dn7 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign63490_e98291 * locals.var_costi3_dn7)) / (locals.var_costi3 * locals.var_costi3)) * assign63490_e98296) + (assign63490_e98293 * ((locals.var_vgssti_dn7 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn7)))) / assign63490_e98297) * locals.var_t0) + (assign63490_e98298 * locals.var_t0_dn7)), (((((((((-(locals.var_costi1_dn8 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign63490_e98291 * locals.var_costi3_dn8)) / (locals.var_costi3 * locals.var_costi3)) * assign63490_e98296) + (assign63490_e98293 * ((locals.var_vgssti_dn8 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn8)))) / assign63490_e98297) * locals.var_t0) + (assign63490_e98298 * locals.var_t0_dn8)), (((((((((-(locals.var_costi1_dn9 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign63490_e98291 * locals.var_costi3_dn9)) / (locals.var_costi3 * locals.var_costi3)) * assign63490_e98296) + (assign63490_e98293 * ((locals.var_vgssti_dn9 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn9)))) / assign63490_e98297) * locals.var_t0) + (assign63490_e98298 * locals.var_t0_dn9)), (((((((((-(locals.var_costi1_dn10 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign63490_e98291 * locals.var_costi3_dn10)) / (locals.var_costi3 * locals.var_costi3)) * assign63490_e98296) + (assign63490_e98293 * ((locals.var_vgssti_dn10 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn10)))) / assign63490_e98297) * locals.var_t0) + (assign63490_e98298 * locals.var_t0_dn10)), (((((((((-(locals.var_costi1_dn11 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign63490_e98291 * locals.var_costi3_dn11)) / (locals.var_costi3 * locals.var_costi3)) * assign63490_e98296) + (assign63490_e98293 * ((locals.var_vgssti_dn11 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn11)))) / assign63490_e98297) * locals.var_t0) + (assign63490_e98298 * locals.var_t0_dn11)), (((((((((-(locals.var_costi1_dn14 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign63490_e98291 * locals.var_costi3_dn14)) / (locals.var_costi3 * locals.var_costi3)) * assign63490_e98296) + (assign63490_e98293 * ((locals.var_vgssti_dn14 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn14)))) / assign63490_e98297) * locals.var_t0) + (assign63490_e98298 * locals.var_t0_dn14)),)
    } else {
        (locals.var_psbsti, locals.var_psbsti_dn0, locals.var_psbsti_dn2, locals.var_psbsti_dn4, locals.var_psbsti_dn5, locals.var_psbsti_dn6, locals.var_psbsti_dn7, locals.var_psbsti_dn8, locals.var_psbsti_dn9, locals.var_psbsti_dn10, locals.var_psbsti_dn11, locals.var_psbsti_dn14,)
    }
};
        locals.var_psbsti = assign63490_e98302;
        locals.var_psbsti_dn0 = assign63490_e98302_d_n0;
        locals.var_psbsti_dn2 = assign63490_e98302_d_n2;
        locals.var_psbsti_dn4 = assign63490_e98302_d_n4;
        locals.var_psbsti_dn5 = assign63490_e98302_d_n5;
        locals.var_psbsti_dn6 = assign63490_e98302_d_n6;
        locals.var_psbsti_dn7 = assign63490_e98302_d_n7;
        locals.var_psbsti_dn8 = assign63490_e98302_d_n8;
        locals.var_psbsti_dn9 = assign63490_e98302_d_n9;
        locals.var_psbsti_dn10 = assign63490_e98302_d_n10;
        locals.var_psbsti_dn11 = assign63490_e98302_d_n11;
        locals.var_psbsti_dn14 = assign63490_e98302_d_n14;
        locals.var_psbsti_rv = 0.0;

        let (assign63500_e98315, assign63500_e98315_d_n0, assign63500_e98315_d_n2, assign63500_e98315_d_n4, assign63500_e98315_d_n5, assign63500_e98315_d_n6, assign63500_e98315_d_n7, assign63500_e98315_d_n8, assign63500_e98315_d_n9, assign63500_e98315_d_n10, assign63500_e98315_d_n11, assign63500_e98315_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63500_e98311: f64 = (locals.var_psbsti - locals.var_psasti);
        let assign63500_e98313: f64 = (assign63500_e98311 - 0.002);
        (assign63500_e98313, (locals.var_psbsti_dn0 - locals.var_psasti_dn0), (locals.var_psbsti_dn2 - locals.var_psasti_dn2), (locals.var_psbsti_dn4 - locals.var_psasti_dn4), (locals.var_psbsti_dn5 - locals.var_psasti_dn5), (locals.var_psbsti_dn6 - locals.var_psasti_dn6), (locals.var_psbsti_dn7 - locals.var_psasti_dn7), (locals.var_psbsti_dn8 - locals.var_psasti_dn8), (locals.var_psbsti_dn9 - locals.var_psasti_dn9), (locals.var_psbsti_dn10 - locals.var_psasti_dn10), (locals.var_psbsti_dn11 - locals.var_psasti_dn11), (locals.var_psbsti_dn14 - locals.var_psasti_dn14),)
    } else {
        (locals.var_psab, locals.var_psab_dn0, locals.var_psab_dn2, locals.var_psab_dn4, locals.var_psab_dn5, locals.var_psab_dn6, locals.var_psab_dn7, locals.var_psab_dn8, locals.var_psab_dn9, locals.var_psab_dn10, locals.var_psab_dn11, locals.var_psab_dn14,)
    }
};
        locals.var_psab = assign63500_e98315;
        locals.var_psab_dn0 = assign63500_e98315_d_n0;
        locals.var_psab_dn2 = assign63500_e98315_d_n2;
        locals.var_psab_dn4 = assign63500_e98315_d_n4;
        locals.var_psab_dn5 = assign63500_e98315_d_n5;
        locals.var_psab_dn6 = assign63500_e98315_d_n6;
        locals.var_psab_dn7 = assign63500_e98315_d_n7;
        locals.var_psab_dn8 = assign63500_e98315_d_n8;
        locals.var_psab_dn9 = assign63500_e98315_d_n9;
        locals.var_psab_dn10 = assign63500_e98315_d_n10;
        locals.var_psab_dn11 = assign63500_e98315_d_n11;
        locals.var_psab_dn14 = assign63500_e98315_d_n14;
        locals.var_psab_rv = 0.0;

        let (assign63510_e98333, assign63510_e98333_d_n0, assign63510_e98333_d_n2, assign63510_e98333_d_n4, assign63510_e98333_d_n5, assign63510_e98333_d_n6, assign63510_e98333_d_n7, assign63510_e98333_d_n8, assign63510_e98333_d_n9, assign63510_e98333_d_n10, assign63510_e98333_d_n11, assign63510_e98333_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63510_e98324: f64 = (locals.var_psab * locals.var_psab);
        let assign63510_e98327: f64 = (4.0 * 0.002);
        let assign63510_e98329: f64 = (assign63510_e98327 * locals.var_psbsti);
        let assign63510_e98330: f64 = (assign63510_e98324 + assign63510_e98329);
        let assign63510_e98331: f64 = (assign63510_e98330).sqrt();
        (assign63510_e98331, ((((locals.var_psab_dn0 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn0)) + (assign63510_e98327 * locals.var_psbsti_dn0)) / (2.0 * assign63510_e98331)), ((((locals.var_psab_dn2 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn2)) + (assign63510_e98327 * locals.var_psbsti_dn2)) / (2.0 * assign63510_e98331)), ((((locals.var_psab_dn4 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn4)) + (assign63510_e98327 * locals.var_psbsti_dn4)) / (2.0 * assign63510_e98331)), ((((locals.var_psab_dn5 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn5)) + (assign63510_e98327 * locals.var_psbsti_dn5)) / (2.0 * assign63510_e98331)), ((((locals.var_psab_dn6 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn6)) + (assign63510_e98327 * locals.var_psbsti_dn6)) / (2.0 * assign63510_e98331)), ((((locals.var_psab_dn7 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn7)) + (assign63510_e98327 * locals.var_psbsti_dn7)) / (2.0 * assign63510_e98331)), ((((locals.var_psab_dn8 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn8)) + (assign63510_e98327 * locals.var_psbsti_dn8)) / (2.0 * assign63510_e98331)), ((((locals.var_psab_dn9 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn9)) + (assign63510_e98327 * locals.var_psbsti_dn9)) / (2.0 * assign63510_e98331)), ((((locals.var_psab_dn10 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn10)) + (assign63510_e98327 * locals.var_psbsti_dn10)) / (2.0 * assign63510_e98331)), ((((locals.var_psab_dn11 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn11)) + (assign63510_e98327 * locals.var_psbsti_dn11)) / (2.0 * assign63510_e98331)), ((((locals.var_psab_dn14 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn14)) + (assign63510_e98327 * locals.var_psbsti_dn14)) / (2.0 * assign63510_e98331)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign63510_e98333;
        locals.var_t0_dn0 = assign63510_e98333_d_n0;
        locals.var_t0_dn2 = assign63510_e98333_d_n2;
        locals.var_t0_dn4 = assign63510_e98333_d_n4;
        locals.var_t0_dn5 = assign63510_e98333_d_n5;
        locals.var_t0_dn6 = assign63510_e98333_d_n6;
        locals.var_t0_dn7 = assign63510_e98333_d_n7;
        locals.var_t0_dn8 = assign63510_e98333_d_n8;
        locals.var_t0_dn9 = assign63510_e98333_d_n9;
        locals.var_t0_dn10 = assign63510_e98333_d_n10;
        locals.var_t0_dn11 = assign63510_e98333_d_n11;
        locals.var_t0_dn14 = assign63510_e98333_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign63520_e98348, assign63520_e98348_d_n0, assign63520_e98348_d_n2, assign63520_e98348_d_n4, assign63520_e98348_d_n5, assign63520_e98348_d_n6, assign63520_e98348_d_n7, assign63520_e98348_d_n8, assign63520_e98348_d_n9, assign63520_e98348_d_n10, assign63520_e98348_d_n11, assign63520_e98348_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63520_e98344: f64 = (locals.var_psab + locals.var_t0);
        let assign63520_e98345: f64 = (0.5 * assign63520_e98344);
        let assign63520_e98346: f64 = (locals.var_psbsti - assign63520_e98345);
        (assign63520_e98346, (locals.var_psbsti_dn0 - (0.5 * (locals.var_psab_dn0 + locals.var_t0_dn0))), (locals.var_psbsti_dn2 - (0.5 * (locals.var_psab_dn2 + locals.var_t0_dn2))), (locals.var_psbsti_dn4 - (0.5 * (locals.var_psab_dn4 + locals.var_t0_dn4))), (locals.var_psbsti_dn5 - (0.5 * (locals.var_psab_dn5 + locals.var_t0_dn5))), (locals.var_psbsti_dn6 - (0.5 * (locals.var_psab_dn6 + locals.var_t0_dn6))), (locals.var_psbsti_dn7 - (0.5 * (locals.var_psab_dn7 + locals.var_t0_dn7))), (locals.var_psbsti_dn8 - (0.5 * (locals.var_psab_dn8 + locals.var_t0_dn8))), (locals.var_psbsti_dn9 - (0.5 * (locals.var_psab_dn9 + locals.var_t0_dn9))), (locals.var_psbsti_dn10 - (0.5 * (locals.var_psab_dn10 + locals.var_t0_dn10))), (locals.var_psbsti_dn11 - (0.5 * (locals.var_psab_dn11 + locals.var_t0_dn11))), (locals.var_psbsti_dn14 - (0.5 * (locals.var_psab_dn14 + locals.var_t0_dn14))),)
    } else {
        (locals.var_psti, locals.var_psti_dn0, locals.var_psti_dn2, locals.var_psti_dn4, locals.var_psti_dn5, locals.var_psti_dn6, locals.var_psti_dn7, locals.var_psti_dn8, locals.var_psti_dn9, locals.var_psti_dn10, locals.var_psti_dn11, locals.var_psti_dn14,)
    }
};
        locals.var_psti = assign63520_e98348;
        locals.var_psti_dn0 = assign63520_e98348_d_n0;
        locals.var_psti_dn2 = assign63520_e98348_d_n2;
        locals.var_psti_dn4 = assign63520_e98348_d_n4;
        locals.var_psti_dn5 = assign63520_e98348_d_n5;
        locals.var_psti_dn6 = assign63520_e98348_d_n6;
        locals.var_psti_dn7 = assign63520_e98348_d_n7;
        locals.var_psti_dn8 = assign63520_e98348_d_n8;
        locals.var_psti_dn9 = assign63520_e98348_d_n9;
        locals.var_psti_dn10 = assign63520_e98348_d_n10;
        locals.var_psti_dn11 = assign63520_e98348_d_n11;
        locals.var_psti_dn14 = assign63520_e98348_d_n14;
        locals.var_psti_rv = 0.0;

        let (assign63530_e98362, assign63530_e98362_d_n0, assign63530_e98362_d_n2, assign63530_e98362_d_n4, assign63530_e98362_d_n5, assign63530_e98362_d_n6, assign63530_e98362_d_n7, assign63530_e98362_d_n8, assign63530_e98362_d_n9, assign63530_e98362_d_n10, assign63530_e98362_d_n11, assign63530_e98362_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63530_e98358: f64 = (locals.var_beta * locals.var_psti);
        let assign63530_e98359: f64 = (assign63530_e98358).exp();
        let assign63530_e98360: f64 = (locals.var_costi1 * assign63530_e98359);
        (assign63530_e98360, ((locals.var_costi1_dn0 * assign63530_e98359) + (locals.var_costi1 * (assign63530_e98359 * ((locals.var_beta_dn0 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn0))))), ((locals.var_costi1_dn2 * assign63530_e98359) + (locals.var_costi1 * (assign63530_e98359 * ((locals.var_beta_dn2 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn2))))), ((locals.var_costi1_dn4 * assign63530_e98359) + (locals.var_costi1 * (assign63530_e98359 * ((locals.var_beta_dn4 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn4))))), ((locals.var_costi1_dn5 * assign63530_e98359) + (locals.var_costi1 * (assign63530_e98359 * ((locals.var_beta_dn5 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn5))))), ((locals.var_costi1_dn6 * assign63530_e98359) + (locals.var_costi1 * (assign63530_e98359 * ((locals.var_beta_dn6 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn6))))), ((locals.var_costi1_dn7 * assign63530_e98359) + (locals.var_costi1 * (assign63530_e98359 * ((locals.var_beta_dn7 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn7))))), ((locals.var_costi1_dn8 * assign63530_e98359) + (locals.var_costi1 * (assign63530_e98359 * ((locals.var_beta_dn8 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn8))))), ((locals.var_costi1_dn9 * assign63530_e98359) + (locals.var_costi1 * (assign63530_e98359 * ((locals.var_beta_dn9 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn9))))), ((locals.var_costi1_dn10 * assign63530_e98359) + (locals.var_costi1 * (assign63530_e98359 * ((locals.var_beta_dn10 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn10))))), ((locals.var_costi1_dn11 * assign63530_e98359) + (locals.var_costi1 * (assign63530_e98359 * ((locals.var_beta_dn11 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn11))))), ((locals.var_costi1_dn14 * assign63530_e98359) + (locals.var_costi1 * (assign63530_e98359 * ((locals.var_beta_dn14 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn14))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign63530_e98362;
        locals.var_t0_dn0 = assign63530_e98362_d_n0;
        locals.var_t0_dn2 = assign63530_e98362_d_n2;
        locals.var_t0_dn4 = assign63530_e98362_d_n4;
        locals.var_t0_dn5 = assign63530_e98362_d_n5;
        locals.var_t0_dn6 = assign63530_e98362_d_n6;
        locals.var_t0_dn7 = assign63530_e98362_d_n7;
        locals.var_t0_dn8 = assign63530_e98362_d_n8;
        locals.var_t0_dn9 = assign63530_e98362_d_n9;
        locals.var_t0_dn10 = assign63530_e98362_d_n10;
        locals.var_t0_dn11 = assign63530_e98362_d_n11;
        locals.var_t0_dn14 = assign63530_e98362_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign63540_e98379, assign63540_e98379_d_n0, assign63540_e98379_d_n2, assign63540_e98379_d_n4, assign63540_e98379_d_n5, assign63540_e98379_d_n6, assign63540_e98379_d_n7, assign63540_e98379_d_n8, assign63540_e98379_d_n9, assign63540_e98379_d_n10, assign63540_e98379_d_n11, assign63540_e98379_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63540_e98372: f64 = (locals.var_psti - locals.var_vbsz__blk442);
        let assign63540_e98373: f64 = (locals.var_beta * assign63540_e98372);
        let assign63540_e98375: f64 = (assign63540_e98373 - 1.0);
        let assign63540_e98377: f64 = (assign63540_e98375 + locals.var_t0);
        (assign63540_e98377, (((locals.var_beta_dn0 * assign63540_e98372) + (locals.var_beta * (locals.var_psti_dn0 - locals.var_vbsz__blk442_dn0))) + locals.var_t0_dn0), (((locals.var_beta_dn2 * assign63540_e98372) + (locals.var_beta * (locals.var_psti_dn2 - locals.var_vbsz__blk442_dn2))) + locals.var_t0_dn2), (((locals.var_beta_dn4 * assign63540_e98372) + (locals.var_beta * (locals.var_psti_dn4 - locals.var_vbsz__blk442_dn4))) + locals.var_t0_dn4), (((locals.var_beta_dn5 * assign63540_e98372) + (locals.var_beta * (locals.var_psti_dn5 - locals.var_vbsz__blk442_dn5))) + locals.var_t0_dn5), (((locals.var_beta_dn6 * assign63540_e98372) + (locals.var_beta * (locals.var_psti_dn6 - locals.var_vbsz__blk442_dn6))) + locals.var_t0_dn6), (((locals.var_beta_dn7 * assign63540_e98372) + (locals.var_beta * (locals.var_psti_dn7 - locals.var_vbsz__blk442_dn7))) + locals.var_t0_dn7), (((locals.var_beta_dn8 * assign63540_e98372) + (locals.var_beta * (locals.var_psti_dn8 - locals.var_vbsz__blk442_dn8))) + locals.var_t0_dn8), (((locals.var_beta_dn9 * assign63540_e98372) + (locals.var_beta * (locals.var_psti_dn9 - locals.var_vbsz__blk442_dn9))) + locals.var_t0_dn9), (((locals.var_beta_dn10 * assign63540_e98372) + (locals.var_beta * (locals.var_psti_dn10 - locals.var_vbsz__blk442_dn10))) + locals.var_t0_dn10), (((locals.var_beta_dn11 * assign63540_e98372) + (locals.var_beta * (locals.var_psti_dn11 - locals.var_vbsz__blk442_dn11))) + locals.var_t0_dn11), (((locals.var_beta_dn14 * assign63540_e98372) + (locals.var_beta * (locals.var_psti_dn14 - locals.var_vbsz__blk442_dn14))) + locals.var_t0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63540_e98379;
        locals.var_t1_dn0 = assign63540_e98379_d_n0;
        locals.var_t1_dn2 = assign63540_e98379_d_n2;
        locals.var_t1_dn4 = assign63540_e98379_d_n4;
        locals.var_t1_dn5 = assign63540_e98379_d_n5;
        locals.var_t1_dn6 = assign63540_e98379_d_n6;
        locals.var_t1_dn7 = assign63540_e98379_d_n7;
        locals.var_t1_dn8 = assign63540_e98379_d_n8;
        locals.var_t1_dn9 = assign63540_e98379_d_n9;
        locals.var_t1_dn10 = assign63540_e98379_d_n10;
        locals.var_t1_dn11 = assign63540_e98379_d_n11;
        locals.var_t1_dn14 = assign63540_e98379_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign63550_e98397, assign63550_e98397_d_n0, assign63550_e98397_d_n2, assign63550_e98397_d_n4, assign63550_e98397_d_n5, assign63550_e98397_d_n6, assign63550_e98397_d_n7, assign63550_e98397_d_n8, assign63550_e98397_d_n9, assign63550_e98397_d_n10, assign63550_e98397_d_n11, assign63550_e98397_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63550_e98388: f64 = (locals.var_t1 * locals.var_t1);
        let assign63550_e98391: f64 = (4.0 * 0.01);
        let assign63550_e98393: f64 = (assign63550_e98391 * 0.01);
        let assign63550_e98394: f64 = (assign63550_e98388 + assign63550_e98393);
        let assign63550_e98395: f64 = (assign63550_e98394).sqrt();
        (assign63550_e98395, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign63550_e98395)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign63550_e98395)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign63550_e98395)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign63550_e98395)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign63550_e98395)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign63550_e98395)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign63550_e98395)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign63550_e98395)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign63550_e98395)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign63550_e98395)), (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign63550_e98395)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign63550_e98397;
        locals.var_tmf2_dn0 = assign63550_e98397_d_n0;
        locals.var_tmf2_dn2 = assign63550_e98397_d_n2;
        locals.var_tmf2_dn4 = assign63550_e98397_d_n4;
        locals.var_tmf2_dn5 = assign63550_e98397_d_n5;
        locals.var_tmf2_dn6 = assign63550_e98397_d_n6;
        locals.var_tmf2_dn7 = assign63550_e98397_d_n7;
        locals.var_tmf2_dn8 = assign63550_e98397_d_n8;
        locals.var_tmf2_dn9 = assign63550_e98397_d_n9;
        locals.var_tmf2_dn10 = assign63550_e98397_d_n10;
        locals.var_tmf2_dn11 = assign63550_e98397_d_n11;
        locals.var_tmf2_dn14 = assign63550_e98397_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign63560_e98412, assign63560_e98412_d_n0, assign63560_e98412_d_n2, assign63560_e98412_d_n4, assign63560_e98412_d_n5, assign63560_e98412_d_n6, assign63560_e98412_d_n7, assign63560_e98412_d_n8, assign63560_e98412_d_n9, assign63560_e98412_d_n10, assign63560_e98412_d_n11, assign63560_e98412_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63560_e98408: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign63560_e98409: f64 = (1.0 + assign63560_e98408);
        let assign63560_e98410: f64 = (0.5 * assign63560_e98409);
        (assign63560_e98410, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn14 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign63560_e98412;
        locals.var_t0_dn0 = assign63560_e98412_d_n0;
        locals.var_t0_dn2 = assign63560_e98412_d_n2;
        locals.var_t0_dn4 = assign63560_e98412_d_n4;
        locals.var_t0_dn5 = assign63560_e98412_d_n5;
        locals.var_t0_dn6 = assign63560_e98412_d_n6;
        locals.var_t0_dn7 = assign63560_e98412_d_n7;
        locals.var_t0_dn8 = assign63560_e98412_d_n8;
        locals.var_t0_dn9 = assign63560_e98412_d_n9;
        locals.var_t0_dn10 = assign63560_e98412_d_n10;
        locals.var_t0_dn11 = assign63560_e98412_d_n11;
        locals.var_t0_dn14 = assign63560_e98412_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign63570_e98425, assign63570_e98425_d_n0, assign63570_e98425_d_n2, assign63570_e98425_d_n4, assign63570_e98425_d_n5, assign63570_e98425_d_n6, assign63570_e98425_d_n7, assign63570_e98425_d_n8, assign63570_e98425_d_n9, assign63570_e98425_d_n10, assign63570_e98425_d_n11, assign63570_e98425_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63570_e98422: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign63570_e98423: f64 = (0.5 * assign63570_e98422);
        (assign63570_e98423, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63570_e98425;
        locals.var_t1_dn0 = assign63570_e98425_d_n0;
        locals.var_t1_dn2 = assign63570_e98425_d_n2;
        locals.var_t1_dn4 = assign63570_e98425_d_n4;
        locals.var_t1_dn5 = assign63570_e98425_d_n5;
        locals.var_t1_dn6 = assign63570_e98425_d_n6;
        locals.var_t1_dn7 = assign63570_e98425_d_n7;
        locals.var_t1_dn8 = assign63570_e98425_d_n8;
        locals.var_t1_dn9 = assign63570_e98425_d_n9;
        locals.var_t1_dn10 = assign63570_e98425_d_n10;
        locals.var_t1_dn11 = assign63570_e98425_d_n11;
        locals.var_t1_dn14 = assign63570_e98425_d_n14;
        locals.var_t1_rv = 0.0;

        let assign63580_e98428: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1509 = assign63580_e98428;
        locals.var_guard1509_rv = 0.0;

        let (assign63590_e98439, assign63590_e98439_d_n0, assign63590_e98439_d_n2, assign63590_e98439_d_n4, assign63590_e98439_d_n5, assign63590_e98439_d_n6, assign63590_e98439_d_n7, assign63590_e98439_d_n8, assign63590_e98439_d_n9, assign63590_e98439_d_n10, assign63590_e98439_d_n11, assign63590_e98439_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) && (locals.var_guard1509 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63590_e98439;
        locals.var_t1_dn0 = assign63590_e98439_d_n0;
        locals.var_t1_dn2 = assign63590_e98439_d_n2;
        locals.var_t1_dn4 = assign63590_e98439_d_n4;
        locals.var_t1_dn5 = assign63590_e98439_d_n5;
        locals.var_t1_dn6 = assign63590_e98439_d_n6;
        locals.var_t1_dn7 = assign63590_e98439_d_n7;
        locals.var_t1_dn8 = assign63590_e98439_d_n8;
        locals.var_t1_dn9 = assign63590_e98439_d_n9;
        locals.var_t1_dn10 = assign63590_e98439_d_n10;
        locals.var_t1_dn11 = assign63590_e98439_d_n11;
        locals.var_t1_dn14 = assign63590_e98439_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign63600_e98450, assign63600_e98450_d_n0, assign63600_e98450_d_n2, assign63600_e98450_d_n4, assign63600_e98450_d_n5, assign63600_e98450_d_n6, assign63600_e98450_d_n7, assign63600_e98450_d_n8, assign63600_e98450_d_n9, assign63600_e98450_d_n10, assign63600_e98450_d_n11, assign63600_e98450_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) && (locals.var_guard1509 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign63600_e98450;
        locals.var_t0_dn0 = assign63600_e98450_d_n0;
        locals.var_t0_dn2 = assign63600_e98450_d_n2;
        locals.var_t0_dn4 = assign63600_e98450_d_n4;
        locals.var_t0_dn5 = assign63600_e98450_d_n5;
        locals.var_t0_dn6 = assign63600_e98450_d_n6;
        locals.var_t0_dn7 = assign63600_e98450_d_n7;
        locals.var_t0_dn8 = assign63600_e98450_d_n8;
        locals.var_t0_dn9 = assign63600_e98450_d_n9;
        locals.var_t0_dn10 = assign63600_e98450_d_n10;
        locals.var_t0_dn11 = assign63600_e98450_d_n11;
        locals.var_t0_dn14 = assign63600_e98450_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign63610_e98461, assign63610_e98461_d_n0, assign63610_e98461_d_n2, assign63610_e98461_d_n4, assign63610_e98461_d_n5, assign63610_e98461_d_n6, assign63610_e98461_d_n7, assign63610_e98461_d_n8, assign63610_e98461_d_n9, assign63610_e98461_d_n10, assign63610_e98461_d_n11, assign63610_e98461_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63610_e98459: f64 = (locals.var_t1 + 1e-25);
        (assign63610_e98459, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63610_e98461;
        locals.var_t1_dn0 = assign63610_e98461_d_n0;
        locals.var_t1_dn2 = assign63610_e98461_d_n2;
        locals.var_t1_dn4 = assign63610_e98461_d_n4;
        locals.var_t1_dn5 = assign63610_e98461_d_n5;
        locals.var_t1_dn6 = assign63610_e98461_d_n6;
        locals.var_t1_dn7 = assign63610_e98461_d_n7;
        locals.var_t1_dn8 = assign63610_e98461_d_n8;
        locals.var_t1_dn9 = assign63610_e98461_d_n9;
        locals.var_t1_dn10 = assign63610_e98461_d_n10;
        locals.var_t1_dn11 = assign63610_e98461_d_n11;
        locals.var_t1_dn14 = assign63610_e98461_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign63620_e98471, assign63620_e98471_d_n0, assign63620_e98471_d_n2, assign63620_e98471_d_n4, assign63620_e98471_d_n5, assign63620_e98471_d_n6, assign63620_e98471_d_n7, assign63620_e98471_d_n8, assign63620_e98471_d_n9, assign63620_e98471_d_n10, assign63620_e98471_d_n11, assign63620_e98471_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63620_e98469: f64 = (locals.var_t1).sqrt();
        (assign63620_e98469, (locals.var_t1_dn0 / (2.0 * assign63620_e98469)), (locals.var_t1_dn2 / (2.0 * assign63620_e98469)), (locals.var_t1_dn4 / (2.0 * assign63620_e98469)), (locals.var_t1_dn5 / (2.0 * assign63620_e98469)), (locals.var_t1_dn6 / (2.0 * assign63620_e98469)), (locals.var_t1_dn7 / (2.0 * assign63620_e98469)), (locals.var_t1_dn8 / (2.0 * assign63620_e98469)), (locals.var_t1_dn9 / (2.0 * assign63620_e98469)), (locals.var_t1_dn10 / (2.0 * assign63620_e98469)), (locals.var_t1_dn11 / (2.0 * assign63620_e98469)), (locals.var_t1_dn14 / (2.0 * assign63620_e98469)),)
    } else {
        (locals.var_sq1sti, locals.var_sq1sti_dn0, locals.var_sq1sti_dn2, locals.var_sq1sti_dn4, locals.var_sq1sti_dn5, locals.var_sq1sti_dn6, locals.var_sq1sti_dn7, locals.var_sq1sti_dn8, locals.var_sq1sti_dn9, locals.var_sq1sti_dn10, locals.var_sq1sti_dn11, locals.var_sq1sti_dn14,)
    }
};
        locals.var_sq1sti = assign63620_e98471;
        locals.var_sq1sti_dn0 = assign63620_e98471_d_n0;
        locals.var_sq1sti_dn2 = assign63620_e98471_d_n2;
        locals.var_sq1sti_dn4 = assign63620_e98471_d_n4;
        locals.var_sq1sti_dn5 = assign63620_e98471_d_n5;
        locals.var_sq1sti_dn6 = assign63620_e98471_d_n6;
        locals.var_sq1sti_dn7 = assign63620_e98471_d_n7;
        locals.var_sq1sti_dn8 = assign63620_e98471_d_n8;
        locals.var_sq1sti_dn9 = assign63620_e98471_d_n9;
        locals.var_sq1sti_dn10 = assign63620_e98471_d_n10;
        locals.var_sq1sti_dn11 = assign63620_e98471_d_n11;
        locals.var_sq1sti_dn14 = assign63620_e98471_d_n14;
        locals.var_sq1sti_rv = 0.0;

        let (assign63630_e98486, assign63630_e98486_d_n0, assign63630_e98486_d_n2, assign63630_e98486_d_n4, assign63630_e98486_d_n5, assign63630_e98486_d_n6, assign63630_e98486_d_n7, assign63630_e98486_d_n8, assign63630_e98486_d_n9, assign63630_e98486_d_n10, assign63630_e98486_d_n11, assign63630_e98486_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63630_e98481: f64 = (locals.var_psti - locals.var_vbsz__blk442);
        let assign63630_e98482: f64 = (locals.var_beta * assign63630_e98481);
        let assign63630_e98484: f64 = (assign63630_e98482 - 1.0);
        (assign63630_e98484, ((locals.var_beta_dn0 * assign63630_e98481) + (locals.var_beta * (locals.var_psti_dn0 - locals.var_vbsz__blk442_dn0))), ((locals.var_beta_dn2 * assign63630_e98481) + (locals.var_beta * (locals.var_psti_dn2 - locals.var_vbsz__blk442_dn2))), ((locals.var_beta_dn4 * assign63630_e98481) + (locals.var_beta * (locals.var_psti_dn4 - locals.var_vbsz__blk442_dn4))), ((locals.var_beta_dn5 * assign63630_e98481) + (locals.var_beta * (locals.var_psti_dn5 - locals.var_vbsz__blk442_dn5))), ((locals.var_beta_dn6 * assign63630_e98481) + (locals.var_beta * (locals.var_psti_dn6 - locals.var_vbsz__blk442_dn6))), ((locals.var_beta_dn7 * assign63630_e98481) + (locals.var_beta * (locals.var_psti_dn7 - locals.var_vbsz__blk442_dn7))), ((locals.var_beta_dn8 * assign63630_e98481) + (locals.var_beta * (locals.var_psti_dn8 - locals.var_vbsz__blk442_dn8))), ((locals.var_beta_dn9 * assign63630_e98481) + (locals.var_beta * (locals.var_psti_dn9 - locals.var_vbsz__blk442_dn9))), ((locals.var_beta_dn10 * assign63630_e98481) + (locals.var_beta * (locals.var_psti_dn10 - locals.var_vbsz__blk442_dn10))), ((locals.var_beta_dn11 * assign63630_e98481) + (locals.var_beta * (locals.var_psti_dn11 - locals.var_vbsz__blk442_dn11))), ((locals.var_beta_dn14 * assign63630_e98481) + (locals.var_beta * (locals.var_psti_dn14 - locals.var_vbsz__blk442_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63630_e98486;
        locals.var_t1_dn0 = assign63630_e98486_d_n0;
        locals.var_t1_dn2 = assign63630_e98486_d_n2;
        locals.var_t1_dn4 = assign63630_e98486_d_n4;
        locals.var_t1_dn5 = assign63630_e98486_d_n5;
        locals.var_t1_dn6 = assign63630_e98486_d_n6;
        locals.var_t1_dn7 = assign63630_e98486_d_n7;
        locals.var_t1_dn8 = assign63630_e98486_d_n8;
        locals.var_t1_dn9 = assign63630_e98486_d_n9;
        locals.var_t1_dn10 = assign63630_e98486_d_n10;
        locals.var_t1_dn11 = assign63630_e98486_d_n11;
        locals.var_t1_dn14 = assign63630_e98486_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign63640_e98504, assign63640_e98504_d_n0, assign63640_e98504_d_n2, assign63640_e98504_d_n4, assign63640_e98504_d_n5, assign63640_e98504_d_n6, assign63640_e98504_d_n7, assign63640_e98504_d_n8, assign63640_e98504_d_n9, assign63640_e98504_d_n10, assign63640_e98504_d_n11, assign63640_e98504_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63640_e98495: f64 = (locals.var_t1 * locals.var_t1);
        let assign63640_e98498: f64 = (4.0 * 0.01);
        let assign63640_e98500: f64 = (assign63640_e98498 * 0.01);
        let assign63640_e98501: f64 = (assign63640_e98495 + assign63640_e98500);
        let assign63640_e98502: f64 = (assign63640_e98501).sqrt();
        (assign63640_e98502, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign63640_e98502)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign63640_e98502)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign63640_e98502)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign63640_e98502)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign63640_e98502)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign63640_e98502)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign63640_e98502)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign63640_e98502)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign63640_e98502)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign63640_e98502)), (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign63640_e98502)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign63640_e98504;
        locals.var_tmf2_dn0 = assign63640_e98504_d_n0;
        locals.var_tmf2_dn2 = assign63640_e98504_d_n2;
        locals.var_tmf2_dn4 = assign63640_e98504_d_n4;
        locals.var_tmf2_dn5 = assign63640_e98504_d_n5;
        locals.var_tmf2_dn6 = assign63640_e98504_d_n6;
        locals.var_tmf2_dn7 = assign63640_e98504_d_n7;
        locals.var_tmf2_dn8 = assign63640_e98504_d_n8;
        locals.var_tmf2_dn9 = assign63640_e98504_d_n9;
        locals.var_tmf2_dn10 = assign63640_e98504_d_n10;
        locals.var_tmf2_dn11 = assign63640_e98504_d_n11;
        locals.var_tmf2_dn14 = assign63640_e98504_d_n14;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_238(
        locals: &mut StampLocals,
    ) {
        let (assign63650_e98519, assign63650_e98519_d_n0, assign63650_e98519_d_n2, assign63650_e98519_d_n4, assign63650_e98519_d_n5, assign63650_e98519_d_n6, assign63650_e98519_d_n7, assign63650_e98519_d_n8, assign63650_e98519_d_n9, assign63650_e98519_d_n10, assign63650_e98519_d_n11, assign63650_e98519_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63650_e98515: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign63650_e98516: f64 = (1.0 + assign63650_e98515);
        let assign63650_e98517: f64 = (0.5 * assign63650_e98516);
        (assign63650_e98517, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn14 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign63650_e98519;
        locals.var_t0_dn0 = assign63650_e98519_d_n0;
        locals.var_t0_dn2 = assign63650_e98519_d_n2;
        locals.var_t0_dn4 = assign63650_e98519_d_n4;
        locals.var_t0_dn5 = assign63650_e98519_d_n5;
        locals.var_t0_dn6 = assign63650_e98519_d_n6;
        locals.var_t0_dn7 = assign63650_e98519_d_n7;
        locals.var_t0_dn8 = assign63650_e98519_d_n8;
        locals.var_t0_dn9 = assign63650_e98519_d_n9;
        locals.var_t0_dn10 = assign63650_e98519_d_n10;
        locals.var_t0_dn11 = assign63650_e98519_d_n11;
        locals.var_t0_dn14 = assign63650_e98519_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign63660_e98532, assign63660_e98532_d_n0, assign63660_e98532_d_n2, assign63660_e98532_d_n4, assign63660_e98532_d_n5, assign63660_e98532_d_n6, assign63660_e98532_d_n7, assign63660_e98532_d_n8, assign63660_e98532_d_n9, assign63660_e98532_d_n10, assign63660_e98532_d_n11, assign63660_e98532_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63660_e98529: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign63660_e98530: f64 = (0.5 * assign63660_e98529);
        (assign63660_e98530, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63660_e98532;
        locals.var_t1_dn0 = assign63660_e98532_d_n0;
        locals.var_t1_dn2 = assign63660_e98532_d_n2;
        locals.var_t1_dn4 = assign63660_e98532_d_n4;
        locals.var_t1_dn5 = assign63660_e98532_d_n5;
        locals.var_t1_dn6 = assign63660_e98532_d_n6;
        locals.var_t1_dn7 = assign63660_e98532_d_n7;
        locals.var_t1_dn8 = assign63660_e98532_d_n8;
        locals.var_t1_dn9 = assign63660_e98532_d_n9;
        locals.var_t1_dn10 = assign63660_e98532_d_n10;
        locals.var_t1_dn11 = assign63660_e98532_d_n11;
        locals.var_t1_dn14 = assign63660_e98532_d_n14;
        locals.var_t1_rv = 0.0;

        let assign63670_e98535: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1510 = assign63670_e98535;
        locals.var_guard1510_rv = 0.0;

        let (assign63680_e98546, assign63680_e98546_d_n0, assign63680_e98546_d_n2, assign63680_e98546_d_n4, assign63680_e98546_d_n5, assign63680_e98546_d_n6, assign63680_e98546_d_n7, assign63680_e98546_d_n8, assign63680_e98546_d_n9, assign63680_e98546_d_n10, assign63680_e98546_d_n11, assign63680_e98546_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) && (locals.var_guard1510 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63680_e98546;
        locals.var_t1_dn0 = assign63680_e98546_d_n0;
        locals.var_t1_dn2 = assign63680_e98546_d_n2;
        locals.var_t1_dn4 = assign63680_e98546_d_n4;
        locals.var_t1_dn5 = assign63680_e98546_d_n5;
        locals.var_t1_dn6 = assign63680_e98546_d_n6;
        locals.var_t1_dn7 = assign63680_e98546_d_n7;
        locals.var_t1_dn8 = assign63680_e98546_d_n8;
        locals.var_t1_dn9 = assign63680_e98546_d_n9;
        locals.var_t1_dn10 = assign63680_e98546_d_n10;
        locals.var_t1_dn11 = assign63680_e98546_d_n11;
        locals.var_t1_dn14 = assign63680_e98546_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign63690_e98557, assign63690_e98557_d_n0, assign63690_e98557_d_n2, assign63690_e98557_d_n4, assign63690_e98557_d_n5, assign63690_e98557_d_n6, assign63690_e98557_d_n7, assign63690_e98557_d_n8, assign63690_e98557_d_n9, assign63690_e98557_d_n10, assign63690_e98557_d_n11, assign63690_e98557_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) && (locals.var_guard1510 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign63690_e98557;
        locals.var_t0_dn0 = assign63690_e98557_d_n0;
        locals.var_t0_dn2 = assign63690_e98557_d_n2;
        locals.var_t0_dn4 = assign63690_e98557_d_n4;
        locals.var_t0_dn5 = assign63690_e98557_d_n5;
        locals.var_t0_dn6 = assign63690_e98557_d_n6;
        locals.var_t0_dn7 = assign63690_e98557_d_n7;
        locals.var_t0_dn8 = assign63690_e98557_d_n8;
        locals.var_t0_dn9 = assign63690_e98557_d_n9;
        locals.var_t0_dn10 = assign63690_e98557_d_n10;
        locals.var_t0_dn11 = assign63690_e98557_d_n11;
        locals.var_t0_dn14 = assign63690_e98557_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign63700_e98568, assign63700_e98568_d_n0, assign63700_e98568_d_n2, assign63700_e98568_d_n4, assign63700_e98568_d_n5, assign63700_e98568_d_n6, assign63700_e98568_d_n7, assign63700_e98568_d_n8, assign63700_e98568_d_n9, assign63700_e98568_d_n10, assign63700_e98568_d_n11, assign63700_e98568_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63700_e98566: f64 = (locals.var_t1 + 1e-25);
        (assign63700_e98566, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63700_e98568;
        locals.var_t1_dn0 = assign63700_e98568_d_n0;
        locals.var_t1_dn2 = assign63700_e98568_d_n2;
        locals.var_t1_dn4 = assign63700_e98568_d_n4;
        locals.var_t1_dn5 = assign63700_e98568_d_n5;
        locals.var_t1_dn6 = assign63700_e98568_d_n6;
        locals.var_t1_dn7 = assign63700_e98568_d_n7;
        locals.var_t1_dn8 = assign63700_e98568_d_n8;
        locals.var_t1_dn9 = assign63700_e98568_d_n9;
        locals.var_t1_dn10 = assign63700_e98568_d_n10;
        locals.var_t1_dn11 = assign63700_e98568_d_n11;
        locals.var_t1_dn14 = assign63700_e98568_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign63710_e98578, assign63710_e98578_d_n0, assign63710_e98578_d_n2, assign63710_e98578_d_n4, assign63710_e98578_d_n5, assign63710_e98578_d_n6, assign63710_e98578_d_n7, assign63710_e98578_d_n8, assign63710_e98578_d_n9, assign63710_e98578_d_n10, assign63710_e98578_d_n11, assign63710_e98578_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63710_e98576: f64 = (locals.var_t1).sqrt();
        (assign63710_e98576, (locals.var_t1_dn0 / (2.0 * assign63710_e98576)), (locals.var_t1_dn2 / (2.0 * assign63710_e98576)), (locals.var_t1_dn4 / (2.0 * assign63710_e98576)), (locals.var_t1_dn5 / (2.0 * assign63710_e98576)), (locals.var_t1_dn6 / (2.0 * assign63710_e98576)), (locals.var_t1_dn7 / (2.0 * assign63710_e98576)), (locals.var_t1_dn8 / (2.0 * assign63710_e98576)), (locals.var_t1_dn9 / (2.0 * assign63710_e98576)), (locals.var_t1_dn10 / (2.0 * assign63710_e98576)), (locals.var_t1_dn11 / (2.0 * assign63710_e98576)), (locals.var_t1_dn14 / (2.0 * assign63710_e98576)),)
    } else {
        (locals.var_sq2sti, locals.var_sq2sti_dn0, locals.var_sq2sti_dn2, locals.var_sq2sti_dn4, locals.var_sq2sti_dn5, locals.var_sq2sti_dn6, locals.var_sq2sti_dn7, locals.var_sq2sti_dn8, locals.var_sq2sti_dn9, locals.var_sq2sti_dn10, locals.var_sq2sti_dn11, locals.var_sq2sti_dn14,)
    }
};
        locals.var_sq2sti = assign63710_e98578;
        locals.var_sq2sti_dn0 = assign63710_e98578_d_n0;
        locals.var_sq2sti_dn2 = assign63710_e98578_d_n2;
        locals.var_sq2sti_dn4 = assign63710_e98578_d_n4;
        locals.var_sq2sti_dn5 = assign63710_e98578_d_n5;
        locals.var_sq2sti_dn6 = assign63710_e98578_d_n6;
        locals.var_sq2sti_dn7 = assign63710_e98578_d_n7;
        locals.var_sq2sti_dn8 = assign63710_e98578_d_n8;
        locals.var_sq2sti_dn9 = assign63710_e98578_d_n9;
        locals.var_sq2sti_dn10 = assign63710_e98578_d_n10;
        locals.var_sq2sti_dn11 = assign63710_e98578_d_n11;
        locals.var_sq2sti_dn14 = assign63710_e98578_d_n14;
        locals.var_sq2sti_rv = 0.0;

        let (assign63720_e98589, assign63720_e98589_d_n0, assign63720_e98589_d_n2, assign63720_e98589_d_n4, assign63720_e98589_d_n5, assign63720_e98589_d_n6, assign63720_e98589_d_n7, assign63720_e98589_d_n8, assign63720_e98589_d_n9, assign63720_e98589_d_n10, assign63720_e98589_d_n11, assign63720_e98589_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63720_e98587: f64 = (0.5 / locals.var_sq2sti);
        (assign63720_e98587, (-((0.5 * locals.var_sq2sti_dn0) / (locals.var_sq2sti * locals.var_sq2sti))), (-((0.5 * locals.var_sq2sti_dn2) / (locals.var_sq2sti * locals.var_sq2sti))), (-((0.5 * locals.var_sq2sti_dn4) / (locals.var_sq2sti * locals.var_sq2sti))), (-((0.5 * locals.var_sq2sti_dn5) / (locals.var_sq2sti * locals.var_sq2sti))), (-((0.5 * locals.var_sq2sti_dn6) / (locals.var_sq2sti * locals.var_sq2sti))), (-((0.5 * locals.var_sq2sti_dn7) / (locals.var_sq2sti * locals.var_sq2sti))), (-((0.5 * locals.var_sq2sti_dn8) / (locals.var_sq2sti * locals.var_sq2sti))), (-((0.5 * locals.var_sq2sti_dn9) / (locals.var_sq2sti * locals.var_sq2sti))), (-((0.5 * locals.var_sq2sti_dn10) / (locals.var_sq2sti * locals.var_sq2sti))), (-((0.5 * locals.var_sq2sti_dn11) / (locals.var_sq2sti * locals.var_sq2sti))), (-((0.5 * locals.var_sq2sti_dn14) / (locals.var_sq2sti * locals.var_sq2sti))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign63720_e98589;
        locals.var_t2_dn0 = assign63720_e98589_d_n0;
        locals.var_t2_dn2 = assign63720_e98589_d_n2;
        locals.var_t2_dn4 = assign63720_e98589_d_n4;
        locals.var_t2_dn5 = assign63720_e98589_d_n5;
        locals.var_t2_dn6 = assign63720_e98589_d_n6;
        locals.var_t2_dn7 = assign63720_e98589_d_n7;
        locals.var_t2_dn8 = assign63720_e98589_d_n8;
        locals.var_t2_dn9 = assign63720_e98589_d_n9;
        locals.var_t2_dn10 = assign63720_e98589_d_n10;
        locals.var_t2_dn11 = assign63720_e98589_d_n11;
        locals.var_t2_dn14 = assign63720_e98589_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign63730_e98602, assign63730_e98602_d_n0, assign63730_e98602_d_n2, assign63730_e98602_d_n4, assign63730_e98602_d_n5, assign63730_e98602_d_n6, assign63730_e98602_d_n7, assign63730_e98602_d_n8, assign63730_e98602_d_n9, assign63730_e98602_d_n10, assign63730_e98602_d_n11, assign63730_e98602_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63730_e98599: f64 = (locals.var_sq1sti - locals.var_sq2sti);
        let assign63730_e98600: f64 = (locals.var_costi0 * assign63730_e98599);
        (assign63730_e98600, ((locals.var_costi0_dn0 * assign63730_e98599) + (locals.var_costi0 * (locals.var_sq1sti_dn0 - locals.var_sq2sti_dn0))), ((locals.var_costi0_dn2 * assign63730_e98599) + (locals.var_costi0 * (locals.var_sq1sti_dn2 - locals.var_sq2sti_dn2))), ((locals.var_costi0_dn4 * assign63730_e98599) + (locals.var_costi0 * (locals.var_sq1sti_dn4 - locals.var_sq2sti_dn4))), ((locals.var_costi0_dn5 * assign63730_e98599) + (locals.var_costi0 * (locals.var_sq1sti_dn5 - locals.var_sq2sti_dn5))), ((locals.var_costi0_dn6 * assign63730_e98599) + (locals.var_costi0 * (locals.var_sq1sti_dn6 - locals.var_sq2sti_dn6))), ((locals.var_costi0_dn7 * assign63730_e98599) + (locals.var_costi0 * (locals.var_sq1sti_dn7 - locals.var_sq2sti_dn7))), ((locals.var_costi0_dn8 * assign63730_e98599) + (locals.var_costi0 * (locals.var_sq1sti_dn8 - locals.var_sq2sti_dn8))), ((locals.var_costi0_dn9 * assign63730_e98599) + (locals.var_costi0 * (locals.var_sq1sti_dn9 - locals.var_sq2sti_dn9))), ((locals.var_costi0_dn10 * assign63730_e98599) + (locals.var_costi0 * (locals.var_sq1sti_dn10 - locals.var_sq2sti_dn10))), ((locals.var_costi0_dn11 * assign63730_e98599) + (locals.var_costi0 * (locals.var_sq1sti_dn11 - locals.var_sq2sti_dn11))), ((locals.var_costi0_dn14 * assign63730_e98599) + (locals.var_costi0 * (locals.var_sq1sti_dn14 - locals.var_sq2sti_dn14))),)
    } else {
        (locals.var_qn0sti, locals.var_qn0sti_dn0, locals.var_qn0sti_dn2, locals.var_qn0sti_dn4, locals.var_qn0sti_dn5, locals.var_qn0sti_dn6, locals.var_qn0sti_dn7, locals.var_qn0sti_dn8, locals.var_qn0sti_dn9, locals.var_qn0sti_dn10, locals.var_qn0sti_dn11, locals.var_qn0sti_dn14,)
    }
};
        locals.var_qn0sti = assign63730_e98602;
        locals.var_qn0sti_dn0 = assign63730_e98602_d_n0;
        locals.var_qn0sti_dn2 = assign63730_e98602_d_n2;
        locals.var_qn0sti_dn4 = assign63730_e98602_d_n4;
        locals.var_qn0sti_dn5 = assign63730_e98602_d_n5;
        locals.var_qn0sti_dn6 = assign63730_e98602_d_n6;
        locals.var_qn0sti_dn7 = assign63730_e98602_d_n7;
        locals.var_qn0sti_dn8 = assign63730_e98602_d_n8;
        locals.var_qn0sti_dn9 = assign63730_e98602_d_n9;
        locals.var_qn0sti_dn10 = assign63730_e98602_d_n10;
        locals.var_qn0sti_dn11 = assign63730_e98602_d_n11;
        locals.var_qn0sti_dn14 = assign63730_e98602_d_n14;
        locals.var_qn0sti_rv = 0.0;

        let (assign63740_e98613, assign63740_e98613_d_n0, assign63740_e98613_d_n2, assign63740_e98613_d_n4, assign63740_e98613_d_n5, assign63740_e98613_d_n6, assign63740_e98613_d_n7, assign63740_e98613_d_n8, assign63740_e98613_d_n9, assign63740_e98613_d_n10, assign63740_e98613_d_n11, assign63740_e98613_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63740_e98611: f64 = (locals.var_psasti - locals.var_psti);
        (assign63740_e98611, (locals.var_psasti_dn0 - locals.var_psti_dn0), (locals.var_psasti_dn2 - locals.var_psti_dn2), (locals.var_psasti_dn4 - locals.var_psti_dn4), (locals.var_psasti_dn5 - locals.var_psti_dn5), (locals.var_psasti_dn6 - locals.var_psti_dn6), (locals.var_psasti_dn7 - locals.var_psti_dn7), (locals.var_psasti_dn8 - locals.var_psti_dn8), (locals.var_psasti_dn9 - locals.var_psti_dn9), (locals.var_psasti_dn10 - locals.var_psti_dn10), (locals.var_psasti_dn11 - locals.var_psti_dn11), (locals.var_psasti_dn14 - locals.var_psti_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63740_e98613;
        locals.var_t1_dn0 = assign63740_e98613_d_n0;
        locals.var_t1_dn2 = assign63740_e98613_d_n2;
        locals.var_t1_dn4 = assign63740_e98613_d_n4;
        locals.var_t1_dn5 = assign63740_e98613_d_n5;
        locals.var_t1_dn6 = assign63740_e98613_d_n6;
        locals.var_t1_dn7 = assign63740_e98613_d_n7;
        locals.var_t1_dn8 = assign63740_e98613_d_n8;
        locals.var_t1_dn9 = assign63740_e98613_d_n9;
        locals.var_t1_dn10 = assign63740_e98613_d_n10;
        locals.var_t1_dn11 = assign63740_e98613_d_n11;
        locals.var_t1_dn14 = assign63740_e98613_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign63750_e98631, assign63750_e98631_d_n0, assign63750_e98631_d_n2, assign63750_e98631_d_n4, assign63750_e98631_d_n5, assign63750_e98631_d_n6, assign63750_e98631_d_n7, assign63750_e98631_d_n8, assign63750_e98631_d_n9, assign63750_e98631_d_n10, assign63750_e98631_d_n11, assign63750_e98631_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63750_e98622: f64 = (locals.var_t1 * locals.var_t1);
        let assign63750_e98625: f64 = (4.0 * 0.1);
        let assign63750_e98627: f64 = (assign63750_e98625 * 0.1);
        let assign63750_e98628: f64 = (assign63750_e98622 + assign63750_e98627);
        let assign63750_e98629: f64 = (assign63750_e98628).sqrt();
        (assign63750_e98629, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign63750_e98629)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign63750_e98629)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign63750_e98629)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign63750_e98629)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign63750_e98629)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign63750_e98629)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign63750_e98629)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign63750_e98629)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign63750_e98629)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign63750_e98629)), (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign63750_e98629)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign63750_e98631;
        locals.var_tmf2_dn0 = assign63750_e98631_d_n0;
        locals.var_tmf2_dn2 = assign63750_e98631_d_n2;
        locals.var_tmf2_dn4 = assign63750_e98631_d_n4;
        locals.var_tmf2_dn5 = assign63750_e98631_d_n5;
        locals.var_tmf2_dn6 = assign63750_e98631_d_n6;
        locals.var_tmf2_dn7 = assign63750_e98631_d_n7;
        locals.var_tmf2_dn8 = assign63750_e98631_d_n8;
        locals.var_tmf2_dn9 = assign63750_e98631_d_n9;
        locals.var_tmf2_dn10 = assign63750_e98631_d_n10;
        locals.var_tmf2_dn11 = assign63750_e98631_d_n11;
        locals.var_tmf2_dn14 = assign63750_e98631_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign63760_e98646, assign63760_e98646_d_n0, assign63760_e98646_d_n2, assign63760_e98646_d_n4, assign63760_e98646_d_n5, assign63760_e98646_d_n6, assign63760_e98646_d_n7, assign63760_e98646_d_n8, assign63760_e98646_d_n9, assign63760_e98646_d_n10, assign63760_e98646_d_n11, assign63760_e98646_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63760_e98642: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign63760_e98643: f64 = (1.0 + assign63760_e98642);
        let assign63760_e98644: f64 = (0.5 * assign63760_e98643);
        (assign63760_e98644, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn14 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign63760_e98646;
        locals.var_t2_dn0 = assign63760_e98646_d_n0;
        locals.var_t2_dn2 = assign63760_e98646_d_n2;
        locals.var_t2_dn4 = assign63760_e98646_d_n4;
        locals.var_t2_dn5 = assign63760_e98646_d_n5;
        locals.var_t2_dn6 = assign63760_e98646_d_n6;
        locals.var_t2_dn7 = assign63760_e98646_d_n7;
        locals.var_t2_dn8 = assign63760_e98646_d_n8;
        locals.var_t2_dn9 = assign63760_e98646_d_n9;
        locals.var_t2_dn10 = assign63760_e98646_d_n10;
        locals.var_t2_dn11 = assign63760_e98646_d_n11;
        locals.var_t2_dn14 = assign63760_e98646_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign63770_e98659, assign63770_e98659_d_n0, assign63770_e98659_d_n2, assign63770_e98659_d_n4, assign63770_e98659_d_n5, assign63770_e98659_d_n6, assign63770_e98659_d_n7, assign63770_e98659_d_n8, assign63770_e98659_d_n9, assign63770_e98659_d_n10, assign63770_e98659_d_n11, assign63770_e98659_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63770_e98656: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign63770_e98657: f64 = (0.5 * assign63770_e98656);
        (assign63770_e98657, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63770_e98659;
        locals.var_t1_dn0 = assign63770_e98659_d_n0;
        locals.var_t1_dn2 = assign63770_e98659_d_n2;
        locals.var_t1_dn4 = assign63770_e98659_d_n4;
        locals.var_t1_dn5 = assign63770_e98659_d_n5;
        locals.var_t1_dn6 = assign63770_e98659_d_n6;
        locals.var_t1_dn7 = assign63770_e98659_d_n7;
        locals.var_t1_dn8 = assign63770_e98659_d_n8;
        locals.var_t1_dn9 = assign63770_e98659_d_n9;
        locals.var_t1_dn10 = assign63770_e98659_d_n10;
        locals.var_t1_dn11 = assign63770_e98659_d_n11;
        locals.var_t1_dn14 = assign63770_e98659_d_n14;
        locals.var_t1_rv = 0.0;

        let assign63780_e98662: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1511 = assign63780_e98662;
        locals.var_guard1511_rv = 0.0;

        let (assign63790_e98673, assign63790_e98673_d_n0, assign63790_e98673_d_n2, assign63790_e98673_d_n4, assign63790_e98673_d_n5, assign63790_e98673_d_n6, assign63790_e98673_d_n7, assign63790_e98673_d_n8, assign63790_e98673_d_n9, assign63790_e98673_d_n10, assign63790_e98673_d_n11, assign63790_e98673_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) && (locals.var_guard1511 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63790_e98673;
        locals.var_t1_dn0 = assign63790_e98673_d_n0;
        locals.var_t1_dn2 = assign63790_e98673_d_n2;
        locals.var_t1_dn4 = assign63790_e98673_d_n4;
        locals.var_t1_dn5 = assign63790_e98673_d_n5;
        locals.var_t1_dn6 = assign63790_e98673_d_n6;
        locals.var_t1_dn7 = assign63790_e98673_d_n7;
        locals.var_t1_dn8 = assign63790_e98673_d_n8;
        locals.var_t1_dn9 = assign63790_e98673_d_n9;
        locals.var_t1_dn10 = assign63790_e98673_d_n10;
        locals.var_t1_dn11 = assign63790_e98673_d_n11;
        locals.var_t1_dn14 = assign63790_e98673_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign63800_e98684, assign63800_e98684_d_n0, assign63800_e98684_d_n2, assign63800_e98684_d_n4, assign63800_e98684_d_n5, assign63800_e98684_d_n6, assign63800_e98684_d_n7, assign63800_e98684_d_n8, assign63800_e98684_d_n9, assign63800_e98684_d_n10, assign63800_e98684_d_n11, assign63800_e98684_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) && (locals.var_guard1511 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign63800_e98684;
        locals.var_t2_dn0 = assign63800_e98684_d_n0;
        locals.var_t2_dn2 = assign63800_e98684_d_n2;
        locals.var_t2_dn4 = assign63800_e98684_d_n4;
        locals.var_t2_dn5 = assign63800_e98684_d_n5;
        locals.var_t2_dn6 = assign63800_e98684_d_n6;
        locals.var_t2_dn7 = assign63800_e98684_d_n7;
        locals.var_t2_dn8 = assign63800_e98684_d_n8;
        locals.var_t2_dn9 = assign63800_e98684_d_n9;
        locals.var_t2_dn10 = assign63800_e98684_d_n10;
        locals.var_t2_dn11 = assign63800_e98684_d_n11;
        locals.var_t2_dn14 = assign63800_e98684_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign63810_e98695, assign63810_e98695_d_n0, assign63810_e98695_d_n2, assign63810_e98695_d_n4, assign63810_e98695_d_n5, assign63810_e98695_d_n6, assign63810_e98695_d_n7, assign63810_e98695_d_n8, assign63810_e98695_d_n9, assign63810_e98695_d_n10, assign63810_e98695_d_n11, assign63810_e98695_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63810_e98693: f64 = (locals.var_t1 + 1e-25);
        (assign63810_e98693, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63810_e98695;
        locals.var_t1_dn0 = assign63810_e98695_d_n0;
        locals.var_t1_dn2 = assign63810_e98695_d_n2;
        locals.var_t1_dn4 = assign63810_e98695_d_n4;
        locals.var_t1_dn5 = assign63810_e98695_d_n5;
        locals.var_t1_dn6 = assign63810_e98695_d_n6;
        locals.var_t1_dn7 = assign63810_e98695_d_n7;
        locals.var_t1_dn8 = assign63810_e98695_d_n8;
        locals.var_t1_dn9 = assign63810_e98695_d_n9;
        locals.var_t1_dn10 = assign63810_e98695_d_n10;
        locals.var_t1_dn11 = assign63810_e98695_d_n11;
        locals.var_t1_dn14 = assign63810_e98695_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign63820_e98706, assign63820_e98706_d_n0, assign63820_e98706_d_n2, assign63820_e98706_d_n4, assign63820_e98706_d_n5, assign63820_e98706_d_n6, assign63820_e98706_d_n7, assign63820_e98706_d_n8, assign63820_e98706_d_n9, assign63820_e98706_d_n10, assign63820_e98706_d_n11, assign63820_e98706_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63820_e98704: f64 = (locals.var_vds / locals.var_t1);
        (assign63820_e98704, (((locals.var_vds_dn0 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn0)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn2 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn2)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn4 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn5 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn6 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn7 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn8 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn9 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn10 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn11 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn14 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn14)) / (locals.var_t1 * locals.var_t1)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign63820_e98706;
        locals.var_tx_dn0 = assign63820_e98706_d_n0;
        locals.var_tx_dn2 = assign63820_e98706_d_n2;
        locals.var_tx_dn4 = assign63820_e98706_d_n4;
        locals.var_tx_dn5 = assign63820_e98706_d_n5;
        locals.var_tx_dn6 = assign63820_e98706_d_n6;
        locals.var_tx_dn7 = assign63820_e98706_d_n7;
        locals.var_tx_dn8 = assign63820_e98706_d_n8;
        locals.var_tx_dn9 = assign63820_e98706_d_n9;
        locals.var_tx_dn10 = assign63820_e98706_d_n10;
        locals.var_tx_dn11 = assign63820_e98706_d_n11;
        locals.var_tx_dn14 = assign63820_e98706_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign63830_e98719, assign63830_e98719_d_n0, assign63830_e98719_d_n2, assign63830_e98719_d_n4, assign63830_e98719_d_n5, assign63830_e98719_d_n6, assign63830_e98719_d_n7, assign63830_e98719_d_n8, assign63830_e98719_d_n9, assign63830_e98719_d_n10, assign63830_e98719_d_n11, assign63830_e98719_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63830_e98716: f64 = (locals.var_t1 * locals.var_t1);
        let assign63830_e98717: f64 = (1.0 / assign63830_e98716);
        (assign63830_e98717, (-(((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (assign63830_e98716 * assign63830_e98716))), (-(((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (assign63830_e98716 * assign63830_e98716))), (-(((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (assign63830_e98716 * assign63830_e98716))), (-(((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (assign63830_e98716 * assign63830_e98716))), (-(((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (assign63830_e98716 * assign63830_e98716))), (-(((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (assign63830_e98716 * assign63830_e98716))), (-(((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (assign63830_e98716 * assign63830_e98716))), (-(((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (assign63830_e98716 * assign63830_e98716))), (-(((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (assign63830_e98716 * assign63830_e98716))), (-(((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (assign63830_e98716 * assign63830_e98716))), (-(((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (assign63830_e98716 * assign63830_e98716))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign63830_e98719;
        locals.var_t2_dn0 = assign63830_e98719_d_n0;
        locals.var_t2_dn2 = assign63830_e98719_d_n2;
        locals.var_t2_dn4 = assign63830_e98719_d_n4;
        locals.var_t2_dn5 = assign63830_e98719_d_n5;
        locals.var_t2_dn6 = assign63830_e98719_d_n6;
        locals.var_t2_dn7 = assign63830_e98719_d_n7;
        locals.var_t2_dn8 = assign63830_e98719_d_n8;
        locals.var_t2_dn9 = assign63830_e98719_d_n9;
        locals.var_t2_dn10 = assign63830_e98719_d_n10;
        locals.var_t2_dn11 = assign63830_e98719_d_n11;
        locals.var_t2_dn14 = assign63830_e98719_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign63840_e98730, assign63840_e98730_d_n0, assign63840_e98730_d_n2, assign63840_e98730_d_n4, assign63840_e98730_d_n5, assign63840_e98730_d_n6, assign63840_e98730_d_n7, assign63840_e98730_d_n8, assign63840_e98730_d_n9, assign63840_e98730_d_n10, assign63840_e98730_d_n11, assign63840_e98730_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63840_e98728: f64 = (locals.var_tx * locals.var_tx);
        (assign63840_e98728, ((locals.var_tx_dn0 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn0)), ((locals.var_tx_dn2 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn2)), ((locals.var_tx_dn4 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn4)), ((locals.var_tx_dn5 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn5)), ((locals.var_tx_dn6 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn6)), ((locals.var_tx_dn7 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn7)), ((locals.var_tx_dn8 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn8)), ((locals.var_tx_dn9 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn9)), ((locals.var_tx_dn10 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn10)), ((locals.var_tx_dn11 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn11)), ((locals.var_tx_dn14 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign63840_e98730;
        locals.var_x2_dn0 = assign63840_e98730_d_n0;
        locals.var_x2_dn2 = assign63840_e98730_d_n2;
        locals.var_x2_dn4 = assign63840_e98730_d_n4;
        locals.var_x2_dn5 = assign63840_e98730_d_n5;
        locals.var_x2_dn6 = assign63840_e98730_d_n6;
        locals.var_x2_dn7 = assign63840_e98730_d_n7;
        locals.var_x2_dn8 = assign63840_e98730_d_n8;
        locals.var_x2_dn9 = assign63840_e98730_d_n9;
        locals.var_x2_dn10 = assign63840_e98730_d_n10;
        locals.var_x2_dn11 = assign63840_e98730_d_n11;
        locals.var_x2_dn14 = assign63840_e98730_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign63850_e98741, assign63850_e98741_d_n0, assign63850_e98741_d_n2, assign63850_e98741_d_n4, assign63850_e98741_d_n5, assign63850_e98741_d_n6, assign63850_e98741_d_n7, assign63850_e98741_d_n8, assign63850_e98741_d_n9, assign63850_e98741_d_n10, assign63850_e98741_d_n11, assign63850_e98741_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63850_e98739: f64 = 1.0;
        (assign63850_e98739, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign63850_e98741;
        locals.var_xmax2_dn0 = assign63850_e98741_d_n0;
        locals.var_xmax2_dn2 = assign63850_e98741_d_n2;
        locals.var_xmax2_dn4 = assign63850_e98741_d_n4;
        locals.var_xmax2_dn5 = assign63850_e98741_d_n5;
        locals.var_xmax2_dn6 = assign63850_e98741_d_n6;
        locals.var_xmax2_dn7 = assign63850_e98741_d_n7;
        locals.var_xmax2_dn8 = assign63850_e98741_d_n8;
        locals.var_xmax2_dn9 = assign63850_e98741_d_n9;
        locals.var_xmax2_dn10 = assign63850_e98741_d_n10;
        locals.var_xmax2_dn11 = assign63850_e98741_d_n11;
        locals.var_xmax2_dn14 = assign63850_e98741_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign63860_e98750, assign63860_e98750_d_n0, assign63860_e98750_d_n2, assign63860_e98750_d_n4, assign63860_e98750_d_n5, assign63860_e98750_d_n6, assign63860_e98750_d_n7, assign63860_e98750_d_n8, assign63860_e98750_d_n9, assign63860_e98750_d_n10, assign63860_e98750_d_n11, assign63860_e98750_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign63860_e98750;
        locals.var_xp_dn0 = assign63860_e98750_d_n0;
        locals.var_xp_dn2 = assign63860_e98750_d_n2;
        locals.var_xp_dn4 = assign63860_e98750_d_n4;
        locals.var_xp_dn5 = assign63860_e98750_d_n5;
        locals.var_xp_dn6 = assign63860_e98750_d_n6;
        locals.var_xp_dn7 = assign63860_e98750_d_n7;
        locals.var_xp_dn8 = assign63860_e98750_d_n8;
        locals.var_xp_dn9 = assign63860_e98750_d_n9;
        locals.var_xp_dn10 = assign63860_e98750_d_n10;
        locals.var_xp_dn11 = assign63860_e98750_d_n11;
        locals.var_xp_dn14 = assign63860_e98750_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign63870_e98759, assign63870_e98759_d_n0, assign63870_e98759_d_n2, assign63870_e98759_d_n4, assign63870_e98759_d_n5, assign63870_e98759_d_n6, assign63870_e98759_d_n7, assign63870_e98759_d_n8, assign63870_e98759_d_n9, assign63870_e98759_d_n10, assign63870_e98759_d_n11, assign63870_e98759_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign63870_e98759;
        locals.var_xmp_dn0 = assign63870_e98759_d_n0;
        locals.var_xmp_dn2 = assign63870_e98759_d_n2;
        locals.var_xmp_dn4 = assign63870_e98759_d_n4;
        locals.var_xmp_dn5 = assign63870_e98759_d_n5;
        locals.var_xmp_dn6 = assign63870_e98759_d_n6;
        locals.var_xmp_dn7 = assign63870_e98759_d_n7;
        locals.var_xmp_dn8 = assign63870_e98759_d_n8;
        locals.var_xmp_dn9 = assign63870_e98759_d_n9;
        locals.var_xmp_dn10 = assign63870_e98759_d_n10;
        locals.var_xmp_dn11 = assign63870_e98759_d_n11;
        locals.var_xmp_dn14 = assign63870_e98759_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign63880_e98768,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign63880_e98768;
        locals.var_m0_rv = 0.0;

        let (assign63890_e98777,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign63890_e98777;
        locals.var_mm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_239(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign63900_e98786, assign63900_e98786_d_n0, assign63900_e98786_d_n2, assign63900_e98786_d_n4, assign63900_e98786_d_n5, assign63900_e98786_d_n6, assign63900_e98786_d_n7, assign63900_e98786_d_n8, assign63900_e98786_d_n9, assign63900_e98786_d_n10, assign63900_e98786_d_n11, assign63900_e98786_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign63900_e98786;
        locals.var_arg_dn0 = assign63900_e98786_d_n0;
        locals.var_arg_dn2 = assign63900_e98786_d_n2;
        locals.var_arg_dn4 = assign63900_e98786_d_n4;
        locals.var_arg_dn5 = assign63900_e98786_d_n5;
        locals.var_arg_dn6 = assign63900_e98786_d_n6;
        locals.var_arg_dn7 = assign63900_e98786_d_n7;
        locals.var_arg_dn8 = assign63900_e98786_d_n8;
        locals.var_arg_dn9 = assign63900_e98786_d_n9;
        locals.var_arg_dn10 = assign63900_e98786_d_n10;
        locals.var_arg_dn11 = assign63900_e98786_d_n11;
        locals.var_arg_dn14 = assign63900_e98786_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign63910_e98795, assign63910_e98795_d_n0, assign63910_e98795_d_n2, assign63910_e98795_d_n4, assign63910_e98795_d_n5, assign63910_e98795_d_n6, assign63910_e98795_d_n7, assign63910_e98795_d_n8, assign63910_e98795_d_n9, assign63910_e98795_d_n10, assign63910_e98795_d_n11, assign63910_e98795_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign63910_e98795;
        locals.var_dnm_dn0 = assign63910_e98795_d_n0;
        locals.var_dnm_dn2 = assign63910_e98795_d_n2;
        locals.var_dnm_dn4 = assign63910_e98795_d_n4;
        locals.var_dnm_dn5 = assign63910_e98795_d_n5;
        locals.var_dnm_dn6 = assign63910_e98795_d_n6;
        locals.var_dnm_dn7 = assign63910_e98795_d_n7;
        locals.var_dnm_dn8 = assign63910_e98795_d_n8;
        locals.var_dnm_dn9 = assign63910_e98795_d_n9;
        locals.var_dnm_dn10 = assign63910_e98795_d_n10;
        locals.var_dnm_dn11 = assign63910_e98795_d_n11;
        locals.var_dnm_dn14 = assign63910_e98795_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign63920_e98806, assign63920_e98806_d_n0, assign63920_e98806_d_n2, assign63920_e98806_d_n4, assign63920_e98806_d_n5, assign63920_e98806_d_n6, assign63920_e98806_d_n7, assign63920_e98806_d_n8, assign63920_e98806_d_n9, assign63920_e98806_d_n10, assign63920_e98806_d_n11, assign63920_e98806_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63920_e98804: f64 = (locals.var_xp * locals.var_x2);
        (assign63920_e98804, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign63920_e98806;
        locals.var_xp_dn0 = assign63920_e98806_d_n0;
        locals.var_xp_dn2 = assign63920_e98806_d_n2;
        locals.var_xp_dn4 = assign63920_e98806_d_n4;
        locals.var_xp_dn5 = assign63920_e98806_d_n5;
        locals.var_xp_dn6 = assign63920_e98806_d_n6;
        locals.var_xp_dn7 = assign63920_e98806_d_n7;
        locals.var_xp_dn8 = assign63920_e98806_d_n8;
        locals.var_xp_dn9 = assign63920_e98806_d_n9;
        locals.var_xp_dn10 = assign63920_e98806_d_n10;
        locals.var_xp_dn11 = assign63920_e98806_d_n11;
        locals.var_xp_dn14 = assign63920_e98806_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign63930_e98817, assign63930_e98817_d_n0, assign63930_e98817_d_n2, assign63930_e98817_d_n4, assign63930_e98817_d_n5, assign63930_e98817_d_n6, assign63930_e98817_d_n7, assign63930_e98817_d_n8, assign63930_e98817_d_n9, assign63930_e98817_d_n10, assign63930_e98817_d_n11, assign63930_e98817_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63930_e98815: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign63930_e98815, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign63930_e98817;
        locals.var_xmp_dn0 = assign63930_e98817_d_n0;
        locals.var_xmp_dn2 = assign63930_e98817_d_n2;
        locals.var_xmp_dn4 = assign63930_e98817_d_n4;
        locals.var_xmp_dn5 = assign63930_e98817_d_n5;
        locals.var_xmp_dn6 = assign63930_e98817_d_n6;
        locals.var_xmp_dn7 = assign63930_e98817_d_n7;
        locals.var_xmp_dn8 = assign63930_e98817_d_n8;
        locals.var_xmp_dn9 = assign63930_e98817_d_n9;
        locals.var_xmp_dn10 = assign63930_e98817_d_n10;
        locals.var_xmp_dn11 = assign63930_e98817_d_n11;
        locals.var_xmp_dn14 = assign63930_e98817_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign63940_e98828, assign63940_e98828_d_n0, assign63940_e98828_d_n2, assign63940_e98828_d_n4, assign63940_e98828_d_n5, assign63940_e98828_d_n6, assign63940_e98828_d_n7, assign63940_e98828_d_n8, assign63940_e98828_d_n9, assign63940_e98828_d_n10, assign63940_e98828_d_n11, assign63940_e98828_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63940_e98826: f64 = (locals.var_xp * locals.var_x2);
        (assign63940_e98826, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign63940_e98828;
        locals.var_xp_dn0 = assign63940_e98828_d_n0;
        locals.var_xp_dn2 = assign63940_e98828_d_n2;
        locals.var_xp_dn4 = assign63940_e98828_d_n4;
        locals.var_xp_dn5 = assign63940_e98828_d_n5;
        locals.var_xp_dn6 = assign63940_e98828_d_n6;
        locals.var_xp_dn7 = assign63940_e98828_d_n7;
        locals.var_xp_dn8 = assign63940_e98828_d_n8;
        locals.var_xp_dn9 = assign63940_e98828_d_n9;
        locals.var_xp_dn10 = assign63940_e98828_d_n10;
        locals.var_xp_dn11 = assign63940_e98828_d_n11;
        locals.var_xp_dn14 = assign63940_e98828_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign63950_e98839, assign63950_e98839_d_n0, assign63950_e98839_d_n2, assign63950_e98839_d_n4, assign63950_e98839_d_n5, assign63950_e98839_d_n6, assign63950_e98839_d_n7, assign63950_e98839_d_n8, assign63950_e98839_d_n9, assign63950_e98839_d_n10, assign63950_e98839_d_n11, assign63950_e98839_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63950_e98837: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign63950_e98837, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign63950_e98839;
        locals.var_xmp_dn0 = assign63950_e98839_d_n0;
        locals.var_xmp_dn2 = assign63950_e98839_d_n2;
        locals.var_xmp_dn4 = assign63950_e98839_d_n4;
        locals.var_xmp_dn5 = assign63950_e98839_d_n5;
        locals.var_xmp_dn6 = assign63950_e98839_d_n6;
        locals.var_xmp_dn7 = assign63950_e98839_d_n7;
        locals.var_xmp_dn8 = assign63950_e98839_d_n8;
        locals.var_xmp_dn9 = assign63950_e98839_d_n9;
        locals.var_xmp_dn10 = assign63950_e98839_d_n10;
        locals.var_xmp_dn11 = assign63950_e98839_d_n11;
        locals.var_xmp_dn14 = assign63950_e98839_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign63960_e98850, assign63960_e98850_d_n0, assign63960_e98850_d_n2, assign63960_e98850_d_n4, assign63960_e98850_d_n5, assign63960_e98850_d_n6, assign63960_e98850_d_n7, assign63960_e98850_d_n8, assign63960_e98850_d_n9, assign63960_e98850_d_n10, assign63960_e98850_d_n11, assign63960_e98850_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63960_e98848: f64 = (locals.var_xp * locals.var_x2);
        (assign63960_e98848, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign63960_e98850;
        locals.var_xp_dn0 = assign63960_e98850_d_n0;
        locals.var_xp_dn2 = assign63960_e98850_d_n2;
        locals.var_xp_dn4 = assign63960_e98850_d_n4;
        locals.var_xp_dn5 = assign63960_e98850_d_n5;
        locals.var_xp_dn6 = assign63960_e98850_d_n6;
        locals.var_xp_dn7 = assign63960_e98850_d_n7;
        locals.var_xp_dn8 = assign63960_e98850_d_n8;
        locals.var_xp_dn9 = assign63960_e98850_d_n9;
        locals.var_xp_dn10 = assign63960_e98850_d_n10;
        locals.var_xp_dn11 = assign63960_e98850_d_n11;
        locals.var_xp_dn14 = assign63960_e98850_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign63970_e98861, assign63970_e98861_d_n0, assign63970_e98861_d_n2, assign63970_e98861_d_n4, assign63970_e98861_d_n5, assign63970_e98861_d_n6, assign63970_e98861_d_n7, assign63970_e98861_d_n8, assign63970_e98861_d_n9, assign63970_e98861_d_n10, assign63970_e98861_d_n11, assign63970_e98861_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63970_e98859: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign63970_e98859, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign63970_e98861;
        locals.var_xmp_dn0 = assign63970_e98861_d_n0;
        locals.var_xmp_dn2 = assign63970_e98861_d_n2;
        locals.var_xmp_dn4 = assign63970_e98861_d_n4;
        locals.var_xmp_dn5 = assign63970_e98861_d_n5;
        locals.var_xmp_dn6 = assign63970_e98861_d_n6;
        locals.var_xmp_dn7 = assign63970_e98861_d_n7;
        locals.var_xmp_dn8 = assign63970_e98861_d_n8;
        locals.var_xmp_dn9 = assign63970_e98861_d_n9;
        locals.var_xmp_dn10 = assign63970_e98861_d_n10;
        locals.var_xmp_dn11 = assign63970_e98861_d_n11;
        locals.var_xmp_dn14 = assign63970_e98861_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign63980_e98872, assign63980_e98872_d_n0, assign63980_e98872_d_n2, assign63980_e98872_d_n4, assign63980_e98872_d_n5, assign63980_e98872_d_n6, assign63980_e98872_d_n7, assign63980_e98872_d_n8, assign63980_e98872_d_n9, assign63980_e98872_d_n10, assign63980_e98872_d_n11, assign63980_e98872_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63980_e98870: f64 = (locals.var_xp * locals.var_x2);
        (assign63980_e98870, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign63980_e98872;
        locals.var_xp_dn0 = assign63980_e98872_d_n0;
        locals.var_xp_dn2 = assign63980_e98872_d_n2;
        locals.var_xp_dn4 = assign63980_e98872_d_n4;
        locals.var_xp_dn5 = assign63980_e98872_d_n5;
        locals.var_xp_dn6 = assign63980_e98872_d_n6;
        locals.var_xp_dn7 = assign63980_e98872_d_n7;
        locals.var_xp_dn8 = assign63980_e98872_d_n8;
        locals.var_xp_dn9 = assign63980_e98872_d_n9;
        locals.var_xp_dn10 = assign63980_e98872_d_n10;
        locals.var_xp_dn11 = assign63980_e98872_d_n11;
        locals.var_xp_dn14 = assign63980_e98872_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign63990_e98883, assign63990_e98883_d_n0, assign63990_e98883_d_n2, assign63990_e98883_d_n4, assign63990_e98883_d_n5, assign63990_e98883_d_n6, assign63990_e98883_d_n7, assign63990_e98883_d_n8, assign63990_e98883_d_n9, assign63990_e98883_d_n10, assign63990_e98883_d_n11, assign63990_e98883_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign63990_e98881: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign63990_e98881, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign63990_e98883;
        locals.var_xmp_dn0 = assign63990_e98883_d_n0;
        locals.var_xmp_dn2 = assign63990_e98883_d_n2;
        locals.var_xmp_dn4 = assign63990_e98883_d_n4;
        locals.var_xmp_dn5 = assign63990_e98883_d_n5;
        locals.var_xmp_dn6 = assign63990_e98883_d_n6;
        locals.var_xmp_dn7 = assign63990_e98883_d_n7;
        locals.var_xmp_dn8 = assign63990_e98883_d_n8;
        locals.var_xmp_dn9 = assign63990_e98883_d_n9;
        locals.var_xmp_dn10 = assign63990_e98883_d_n10;
        locals.var_xmp_dn11 = assign63990_e98883_d_n11;
        locals.var_xmp_dn14 = assign63990_e98883_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign64000_e98894, assign64000_e98894_d_n0, assign64000_e98894_d_n2, assign64000_e98894_d_n4, assign64000_e98894_d_n5, assign64000_e98894_d_n6, assign64000_e98894_d_n7, assign64000_e98894_d_n8, assign64000_e98894_d_n9, assign64000_e98894_d_n10, assign64000_e98894_d_n11, assign64000_e98894_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign64000_e98892: f64 = (locals.var_xp + locals.var_xmp);
        (assign64000_e98892, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign64000_e98894;
        locals.var_arg_dn0 = assign64000_e98894_d_n0;
        locals.var_arg_dn2 = assign64000_e98894_d_n2;
        locals.var_arg_dn4 = assign64000_e98894_d_n4;
        locals.var_arg_dn5 = assign64000_e98894_d_n5;
        locals.var_arg_dn6 = assign64000_e98894_d_n6;
        locals.var_arg_dn7 = assign64000_e98894_d_n7;
        locals.var_arg_dn8 = assign64000_e98894_d_n8;
        locals.var_arg_dn9 = assign64000_e98894_d_n9;
        locals.var_arg_dn10 = assign64000_e98894_d_n10;
        locals.var_arg_dn11 = assign64000_e98894_d_n11;
        locals.var_arg_dn14 = assign64000_e98894_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign64010_e98903, assign64010_e98903_d_n0, assign64010_e98903_d_n2, assign64010_e98903_d_n4, assign64010_e98903_d_n5, assign64010_e98903_d_n6, assign64010_e98903_d_n7, assign64010_e98903_d_n8, assign64010_e98903_d_n9, assign64010_e98903_d_n10, assign64010_e98903_d_n11, assign64010_e98903_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign64010_e98903;
        locals.var_dnm_dn0 = assign64010_e98903_d_n0;
        locals.var_dnm_dn2 = assign64010_e98903_d_n2;
        locals.var_dnm_dn4 = assign64010_e98903_d_n4;
        locals.var_dnm_dn5 = assign64010_e98903_d_n5;
        locals.var_dnm_dn6 = assign64010_e98903_d_n6;
        locals.var_dnm_dn7 = assign64010_e98903_d_n7;
        locals.var_dnm_dn8 = assign64010_e98903_d_n8;
        locals.var_dnm_dn9 = assign64010_e98903_d_n9;
        locals.var_dnm_dn10 = assign64010_e98903_d_n10;
        locals.var_dnm_dn11 = assign64010_e98903_d_n11;
        locals.var_dnm_dn14 = assign64010_e98903_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign64020_e98918: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1512 = assign64020_e98918;
        locals.var_guard1512_rv = 0.0;

        let assign64030_e98921: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1513 = assign64030_e98921;
        locals.var_guard1513_rv = 0.0;

        let (assign64040_e98934,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) && (locals.var_guard1512 != 0.0)) && (locals.var_guard1513 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign64040_e98934;
        locals.var_mm_rv = 0.0;

        let assign64050_e98937: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1514 = assign64050_e98937;
        locals.var_guard1514_rv = 0.0;

        let (assign64060_e98953,) = {
    if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) && (locals.var_guard1512 != 0.0)) && (locals.var_guard1513 == 0.0)) && (locals.var_guard1514 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign64060_e98953;
        locals.var_mm_rv = 0.0;

        let assign64070_e98956: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1515 = assign64070_e98956;
        locals.var_guard1515_rv = 0.0;

        let (assign64080_e98975,) = {
    if (((((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) && (locals.var_guard1512 != 0.0)) && (locals.var_guard1513 == 0.0)) && (locals.var_guard1514 == 0.0)) && (locals.var_guard1515 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign64080_e98975;
        locals.var_mm_rv = 0.0;

        let assign64090_e98978: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1516 = assign64090_e98978;
        locals.var_guard1516_rv = 0.0;

        let (assign64100_e99000,) = {
    if ((((((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) && (locals.var_guard1512 != 0.0)) && (locals.var_guard1513 == 0.0)) && (locals.var_guard1514 == 0.0)) && (locals.var_guard1515 == 0.0)) && (locals.var_guard1516 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign64100_e99000;
        locals.var_mm_rv = 0.0;

        let (assign64110_e99011,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) && (locals.var_guard1512 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign64110_e99011;
        locals.var_m0_rv = 0.0;

        let mut assign64120_loop_guard: usize = 0;
        while {
            let assign64120_cond_e99023: f64 = if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) && (locals.var_guard1512 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign64120_cond_e99023 != 0.0
        } {
            assign64120_loop_guard += 1;
            assert!(assign64120_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign64120_body0_e99035, assign64120_body0_e99035_d_n0, assign64120_body0_e99035_d_n2, assign64120_body0_e99035_d_n4, assign64120_body0_e99035_d_n5, assign64120_body0_e99035_d_n6, assign64120_body0_e99035_d_n7, assign64120_body0_e99035_d_n8, assign64120_body0_e99035_d_n9, assign64120_body0_e99035_d_n10, assign64120_body0_e99035_d_n11, assign64120_body0_e99035_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) && (locals.var_guard1512 != 0.0)) {
        let assign64120_body0_e99033: f64 = (locals.var_dnm).sqrt();
        (assign64120_body0_e99033, (locals.var_dnm_dn0 / (2.0 * assign64120_body0_e99033)), (locals.var_dnm_dn2 / (2.0 * assign64120_body0_e99033)), (locals.var_dnm_dn4 / (2.0 * assign64120_body0_e99033)), (locals.var_dnm_dn5 / (2.0 * assign64120_body0_e99033)), (locals.var_dnm_dn6 / (2.0 * assign64120_body0_e99033)), (locals.var_dnm_dn7 / (2.0 * assign64120_body0_e99033)), (locals.var_dnm_dn8 / (2.0 * assign64120_body0_e99033)), (locals.var_dnm_dn9 / (2.0 * assign64120_body0_e99033)), (locals.var_dnm_dn10 / (2.0 * assign64120_body0_e99033)), (locals.var_dnm_dn11 / (2.0 * assign64120_body0_e99033)), (locals.var_dnm_dn14 / (2.0 * assign64120_body0_e99033)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign64120_body0_e99035;
            locals.var_dnm_dn0 = assign64120_body0_e99035_d_n0;
            locals.var_dnm_dn2 = assign64120_body0_e99035_d_n2;
            locals.var_dnm_dn4 = assign64120_body0_e99035_d_n4;
            locals.var_dnm_dn5 = assign64120_body0_e99035_d_n5;
            locals.var_dnm_dn6 = assign64120_body0_e99035_d_n6;
            locals.var_dnm_dn7 = assign64120_body0_e99035_d_n7;
            locals.var_dnm_dn8 = assign64120_body0_e99035_d_n8;
            locals.var_dnm_dn9 = assign64120_body0_e99035_d_n9;
            locals.var_dnm_dn10 = assign64120_body0_e99035_d_n10;
            locals.var_dnm_dn11 = assign64120_body0_e99035_d_n11;
            locals.var_dnm_dn14 = assign64120_body0_e99035_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign64120_body1_e99048,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) && (locals.var_guard1512 != 0.0)) {
        let assign64120_body1_e99046: f64 = (locals.var_m0 + 1.0);
        (assign64120_body1_e99046,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign64120_body1_e99048;
            locals.var_m0_rv = 0.0;
        }

        let (assign64130_e99071, assign64130_e99071_d_n0, assign64130_e99071_d_n2, assign64130_e99071_d_n4, assign64130_e99071_d_n5, assign64130_e99071_d_n6, assign64130_e99071_d_n7, assign64130_e99071_d_n8, assign64130_e99071_d_n9, assign64130_e99071_d_n10, assign64130_e99071_d_n11, assign64130_e99071_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) && (locals.var_guard1512 == 0.0)) {
        let (assign64130_e99069, assign64130_e99069_d_n0, assign64130_e99069_d_n2, assign64130_e99069_d_n4, assign64130_e99069_d_n5, assign64130_e99069_d_n6, assign64130_e99069_d_n7, assign64130_e99069_d_n8, assign64130_e99069_d_n9, assign64130_e99069_d_n10, assign64130_e99069_d_n11, assign64130_e99069_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign64130_e99066: f64 = (2.0 * 4.0);
                let assign64130_e99067: f64 = (1.0 / assign64130_e99066);
                let assign64130_e99068: f64 = (locals.var_dnm).powf(assign64130_e99067);
                (assign64130_e99068, if 0.0 == 0.0 && ((assign64130_e99067) as f64).is_finite() && ((assign64130_e99067) as f64).fract() == 0.0 { if assign64130_e99067 == 0.0 { 0.0 } else { (assign64130_e99067 * ((locals.var_dnm).powf(assign64130_e99067 - 1.0) * locals.var_dnm_dn0)) } } else { (assign64130_e99068 * (assign64130_e99067 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64130_e99067) as f64).is_finite() && ((assign64130_e99067) as f64).fract() == 0.0 { if assign64130_e99067 == 0.0 { 0.0 } else { (assign64130_e99067 * ((locals.var_dnm).powf(assign64130_e99067 - 1.0) * locals.var_dnm_dn2)) } } else { (assign64130_e99068 * (assign64130_e99067 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64130_e99067) as f64).is_finite() && ((assign64130_e99067) as f64).fract() == 0.0 { if assign64130_e99067 == 0.0 { 0.0 } else { (assign64130_e99067 * ((locals.var_dnm).powf(assign64130_e99067 - 1.0) * locals.var_dnm_dn4)) } } else { (assign64130_e99068 * (assign64130_e99067 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64130_e99067) as f64).is_finite() && ((assign64130_e99067) as f64).fract() == 0.0 { if assign64130_e99067 == 0.0 { 0.0 } else { (assign64130_e99067 * ((locals.var_dnm).powf(assign64130_e99067 - 1.0) * locals.var_dnm_dn5)) } } else { (assign64130_e99068 * (assign64130_e99067 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64130_e99067) as f64).is_finite() && ((assign64130_e99067) as f64).fract() == 0.0 { if assign64130_e99067 == 0.0 { 0.0 } else { (assign64130_e99067 * ((locals.var_dnm).powf(assign64130_e99067 - 1.0) * locals.var_dnm_dn6)) } } else { (assign64130_e99068 * (assign64130_e99067 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64130_e99067) as f64).is_finite() && ((assign64130_e99067) as f64).fract() == 0.0 { if assign64130_e99067 == 0.0 { 0.0 } else { (assign64130_e99067 * ((locals.var_dnm).powf(assign64130_e99067 - 1.0) * locals.var_dnm_dn7)) } } else { (assign64130_e99068 * (assign64130_e99067 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64130_e99067) as f64).is_finite() && ((assign64130_e99067) as f64).fract() == 0.0 { if assign64130_e99067 == 0.0 { 0.0 } else { (assign64130_e99067 * ((locals.var_dnm).powf(assign64130_e99067 - 1.0) * locals.var_dnm_dn8)) } } else { (assign64130_e99068 * (assign64130_e99067 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64130_e99067) as f64).is_finite() && ((assign64130_e99067) as f64).fract() == 0.0 { if assign64130_e99067 == 0.0 { 0.0 } else { (assign64130_e99067 * ((locals.var_dnm).powf(assign64130_e99067 - 1.0) * locals.var_dnm_dn9)) } } else { (assign64130_e99068 * (assign64130_e99067 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64130_e99067) as f64).is_finite() && ((assign64130_e99067) as f64).fract() == 0.0 { if assign64130_e99067 == 0.0 { 0.0 } else { (assign64130_e99067 * ((locals.var_dnm).powf(assign64130_e99067 - 1.0) * locals.var_dnm_dn10)) } } else { (assign64130_e99068 * (assign64130_e99067 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64130_e99067) as f64).is_finite() && ((assign64130_e99067) as f64).fract() == 0.0 { if assign64130_e99067 == 0.0 { 0.0 } else { (assign64130_e99067 * ((locals.var_dnm).powf(assign64130_e99067 - 1.0) * locals.var_dnm_dn11)) } } else { (assign64130_e99068 * (assign64130_e99067 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64130_e99067) as f64).is_finite() && ((assign64130_e99067) as f64).fract() == 0.0 { if assign64130_e99067 == 0.0 { 0.0 } else { (assign64130_e99067 * ((locals.var_dnm).powf(assign64130_e99067 - 1.0) * locals.var_dnm_dn14)) } } else { (assign64130_e99068 * (assign64130_e99067 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign64130_e99069, assign64130_e99069_d_n0, assign64130_e99069_d_n2, assign64130_e99069_d_n4, assign64130_e99069_d_n5, assign64130_e99069_d_n6, assign64130_e99069_d_n7, assign64130_e99069_d_n8, assign64130_e99069_d_n9, assign64130_e99069_d_n10, assign64130_e99069_d_n11, assign64130_e99069_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign64130_e99071;
        locals.var_dnm_dn0 = assign64130_e99071_d_n0;
        locals.var_dnm_dn2 = assign64130_e99071_d_n2;
        locals.var_dnm_dn4 = assign64130_e99071_d_n4;
        locals.var_dnm_dn5 = assign64130_e99071_d_n5;
        locals.var_dnm_dn6 = assign64130_e99071_d_n6;
        locals.var_dnm_dn7 = assign64130_e99071_d_n7;
        locals.var_dnm_dn8 = assign64130_e99071_d_n8;
        locals.var_dnm_dn9 = assign64130_e99071_d_n9;
        locals.var_dnm_dn10 = assign64130_e99071_d_n10;
        locals.var_dnm_dn11 = assign64130_e99071_d_n11;
        locals.var_dnm_dn14 = assign64130_e99071_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign64140_e99082, assign64140_e99082_d_n0, assign64140_e99082_d_n2, assign64140_e99082_d_n4, assign64140_e99082_d_n5, assign64140_e99082_d_n6, assign64140_e99082_d_n7, assign64140_e99082_d_n8, assign64140_e99082_d_n9, assign64140_e99082_d_n10, assign64140_e99082_d_n11, assign64140_e99082_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign64140_e99080: f64 = (1.0 / locals.var_dnm);
        (assign64140_e99080, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign64140_e99082;
        locals.var_dnm_dn0 = assign64140_e99082_d_n0;
        locals.var_dnm_dn2 = assign64140_e99082_d_n2;
        locals.var_dnm_dn4 = assign64140_e99082_d_n4;
        locals.var_dnm_dn5 = assign64140_e99082_d_n5;
        locals.var_dnm_dn6 = assign64140_e99082_d_n6;
        locals.var_dnm_dn7 = assign64140_e99082_d_n7;
        locals.var_dnm_dn8 = assign64140_e99082_d_n8;
        locals.var_dnm_dn9 = assign64140_e99082_d_n9;
        locals.var_dnm_dn10 = assign64140_e99082_d_n10;
        locals.var_dnm_dn11 = assign64140_e99082_d_n11;
        locals.var_dnm_dn14 = assign64140_e99082_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign64150_e99095, assign64150_e99095_d_n0, assign64150_e99095_d_n2, assign64150_e99095_d_n4, assign64150_e99095_d_n5, assign64150_e99095_d_n6, assign64150_e99095_d_n7, assign64150_e99095_d_n8, assign64150_e99095_d_n9, assign64150_e99095_d_n10, assign64150_e99095_d_n11, assign64150_e99095_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign64150_e99091: f64 = locals.var_tx;
        let assign64150_e99093: f64 = (assign64150_e99091 * locals.var_dnm);
        (assign64150_e99093, ((locals.var_tx_dn0 * locals.var_dnm) + (assign64150_e99091 * locals.var_dnm_dn0)), ((locals.var_tx_dn2 * locals.var_dnm) + (assign64150_e99091 * locals.var_dnm_dn2)), ((locals.var_tx_dn4 * locals.var_dnm) + (assign64150_e99091 * locals.var_dnm_dn4)), ((locals.var_tx_dn5 * locals.var_dnm) + (assign64150_e99091 * locals.var_dnm_dn5)), ((locals.var_tx_dn6 * locals.var_dnm) + (assign64150_e99091 * locals.var_dnm_dn6)), ((locals.var_tx_dn7 * locals.var_dnm) + (assign64150_e99091 * locals.var_dnm_dn7)), ((locals.var_tx_dn8 * locals.var_dnm) + (assign64150_e99091 * locals.var_dnm_dn8)), ((locals.var_tx_dn9 * locals.var_dnm) + (assign64150_e99091 * locals.var_dnm_dn9)), ((locals.var_tx_dn10 * locals.var_dnm) + (assign64150_e99091 * locals.var_dnm_dn10)), ((locals.var_tx_dn11 * locals.var_dnm) + (assign64150_e99091 * locals.var_dnm_dn11)), ((locals.var_tx_dn14 * locals.var_dnm) + (assign64150_e99091 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign64150_e99095;
        locals.var_ty_dn0 = assign64150_e99095_d_n0;
        locals.var_ty_dn2 = assign64150_e99095_d_n2;
        locals.var_ty_dn4 = assign64150_e99095_d_n4;
        locals.var_ty_dn5 = assign64150_e99095_d_n5;
        locals.var_ty_dn6 = assign64150_e99095_d_n6;
        locals.var_ty_dn7 = assign64150_e99095_d_n7;
        locals.var_ty_dn8 = assign64150_e99095_d_n8;
        locals.var_ty_dn9 = assign64150_e99095_d_n9;
        locals.var_ty_dn10 = assign64150_e99095_d_n10;
        locals.var_ty_dn11 = assign64150_e99095_d_n11;
        locals.var_ty_dn14 = assign64150_e99095_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign64160_e99110, assign64160_e99110_d_n0, assign64160_e99110_d_n2, assign64160_e99110_d_n4, assign64160_e99110_d_n5, assign64160_e99110_d_n6, assign64160_e99110_d_n7, assign64160_e99110_d_n8, assign64160_e99110_d_n9, assign64160_e99110_d_n10, assign64160_e99110_d_n11, assign64160_e99110_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign64160_e99104: f64 = locals.var_xmp;
        let assign64160_e99106: f64 = (assign64160_e99104 * locals.var_dnm);
        let assign64160_e99108: f64 = (assign64160_e99106 / locals.var_arg);
        (assign64160_e99108, (((((locals.var_xmp_dn0 * locals.var_dnm) + (assign64160_e99104 * locals.var_dnm_dn0)) * locals.var_arg) - (assign64160_e99106 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn2 * locals.var_dnm) + (assign64160_e99104 * locals.var_dnm_dn2)) * locals.var_arg) - (assign64160_e99106 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn4 * locals.var_dnm) + (assign64160_e99104 * locals.var_dnm_dn4)) * locals.var_arg) - (assign64160_e99106 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn5 * locals.var_dnm) + (assign64160_e99104 * locals.var_dnm_dn5)) * locals.var_arg) - (assign64160_e99106 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn6 * locals.var_dnm) + (assign64160_e99104 * locals.var_dnm_dn6)) * locals.var_arg) - (assign64160_e99106 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn7 * locals.var_dnm) + (assign64160_e99104 * locals.var_dnm_dn7)) * locals.var_arg) - (assign64160_e99106 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn8 * locals.var_dnm) + (assign64160_e99104 * locals.var_dnm_dn8)) * locals.var_arg) - (assign64160_e99106 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn9 * locals.var_dnm) + (assign64160_e99104 * locals.var_dnm_dn9)) * locals.var_arg) - (assign64160_e99106 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn10 * locals.var_dnm) + (assign64160_e99104 * locals.var_dnm_dn10)) * locals.var_arg) - (assign64160_e99106 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn11 * locals.var_dnm) + (assign64160_e99104 * locals.var_dnm_dn11)) * locals.var_arg) - (assign64160_e99106 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn14 * locals.var_dnm) + (assign64160_e99104 * locals.var_dnm_dn14)) * locals.var_arg) - (assign64160_e99106 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign64160_e99110;
        locals.var_t2_dn0 = assign64160_e99110_d_n0;
        locals.var_t2_dn2 = assign64160_e99110_d_n2;
        locals.var_t2_dn4 = assign64160_e99110_d_n4;
        locals.var_t2_dn5 = assign64160_e99110_d_n5;
        locals.var_t2_dn6 = assign64160_e99110_d_n6;
        locals.var_t2_dn7 = assign64160_e99110_d_n7;
        locals.var_t2_dn8 = assign64160_e99110_d_n8;
        locals.var_t2_dn9 = assign64160_e99110_d_n9;
        locals.var_t2_dn10 = assign64160_e99110_d_n10;
        locals.var_t2_dn11 = assign64160_e99110_d_n11;
        locals.var_t2_dn14 = assign64160_e99110_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign64170_e99125, assign64170_e99125_d_n0, assign64170_e99125_d_n2, assign64170_e99125_d_n4, assign64170_e99125_d_n5, assign64170_e99125_d_n6, assign64170_e99125_d_n7, assign64170_e99125_d_n8, assign64170_e99125_d_n9, assign64170_e99125_d_n10, assign64170_e99125_d_n11, assign64170_e99125_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign64170_e99119: f64 = (2.0 * locals.var_uc_wsti);
        let assign64170_e99121: f64 = (assign64170_e99119 * p.p7);
        let assign64170_e99123: f64 = (assign64170_e99121 * locals.var_beta_inv);
        (assign64170_e99123, ((((2.0 * locals.var_uc_wsti_dn0) * p.p7) * locals.var_beta_inv) + (assign64170_e99121 * locals.var_beta_inv_dn0)), ((((2.0 * locals.var_uc_wsti_dn2) * p.p7) * locals.var_beta_inv) + (assign64170_e99121 * locals.var_beta_inv_dn2)), ((((2.0 * locals.var_uc_wsti_dn4) * p.p7) * locals.var_beta_inv) + (assign64170_e99121 * locals.var_beta_inv_dn4)), ((((2.0 * locals.var_uc_wsti_dn5) * p.p7) * locals.var_beta_inv) + (assign64170_e99121 * locals.var_beta_inv_dn5)), ((((2.0 * locals.var_uc_wsti_dn6) * p.p7) * locals.var_beta_inv) + (assign64170_e99121 * locals.var_beta_inv_dn6)), ((((2.0 * locals.var_uc_wsti_dn7) * p.p7) * locals.var_beta_inv) + (assign64170_e99121 * locals.var_beta_inv_dn7)), ((((2.0 * locals.var_uc_wsti_dn8) * p.p7) * locals.var_beta_inv) + (assign64170_e99121 * locals.var_beta_inv_dn8)), ((((2.0 * locals.var_uc_wsti_dn9) * p.p7) * locals.var_beta_inv) + (assign64170_e99121 * locals.var_beta_inv_dn9)), ((((2.0 * locals.var_uc_wsti_dn10) * p.p7) * locals.var_beta_inv) + (assign64170_e99121 * locals.var_beta_inv_dn10)), ((((2.0 * locals.var_uc_wsti_dn11) * p.p7) * locals.var_beta_inv) + (assign64170_e99121 * locals.var_beta_inv_dn11)), ((((2.0 * locals.var_uc_wsti_dn14) * p.p7) * locals.var_beta_inv) + (assign64170_e99121 * locals.var_beta_inv_dn14)),)
    } else {
        (locals.var_costi7, locals.var_costi7_dn0, locals.var_costi7_dn2, locals.var_costi7_dn4, locals.var_costi7_dn5, locals.var_costi7_dn6, locals.var_costi7_dn7, locals.var_costi7_dn8, locals.var_costi7_dn9, locals.var_costi7_dn10, locals.var_costi7_dn11, locals.var_costi7_dn14,)
    }
};
        locals.var_costi7 = assign64170_e99125;
        locals.var_costi7_dn0 = assign64170_e99125_d_n0;
        locals.var_costi7_dn2 = assign64170_e99125_d_n2;
        locals.var_costi7_dn4 = assign64170_e99125_d_n4;
        locals.var_costi7_dn5 = assign64170_e99125_d_n5;
        locals.var_costi7_dn6 = assign64170_e99125_d_n6;
        locals.var_costi7_dn7 = assign64170_e99125_d_n7;
        locals.var_costi7_dn8 = assign64170_e99125_d_n8;
        locals.var_costi7_dn9 = assign64170_e99125_d_n9;
        locals.var_costi7_dn10 = assign64170_e99125_d_n10;
        locals.var_costi7_dn11 = assign64170_e99125_d_n11;
        locals.var_costi7_dn14 = assign64170_e99125_d_n14;
        locals.var_costi7_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_240(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign64180_e99134, assign64180_e99134_d_n0, assign64180_e99134_d_n2, assign64180_e99134_d_n4, assign64180_e99134_d_n5, assign64180_e99134_d_n6, assign64180_e99134_d_n7, assign64180_e99134_d_n8, assign64180_e99134_d_n9, assign64180_e99134_d_n10, assign64180_e99134_d_n11, assign64180_e99134_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn4, locals.var_lch_dn5, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn8, locals.var_lch_dn9, locals.var_lch_dn10, locals.var_lch_dn11, locals.var_lch_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign64180_e99134;
        locals.var_t1_dn0 = assign64180_e99134_d_n0;
        locals.var_t1_dn2 = assign64180_e99134_d_n2;
        locals.var_t1_dn4 = assign64180_e99134_d_n4;
        locals.var_t1_dn5 = assign64180_e99134_d_n5;
        locals.var_t1_dn6 = assign64180_e99134_d_n6;
        locals.var_t1_dn7 = assign64180_e99134_d_n7;
        locals.var_t1_dn8 = assign64180_e99134_d_n8;
        locals.var_t1_dn9 = assign64180_e99134_d_n9;
        locals.var_t1_dn10 = assign64180_e99134_d_n10;
        locals.var_t1_dn11 = assign64180_e99134_d_n11;
        locals.var_t1_dn14 = assign64180_e99134_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign64190_e99151, assign64190_e99151_d_n0, assign64190_e99151_d_n2, assign64190_e99151_d_n4, assign64190_e99151_d_n5, assign64190_e99151_d_n6, assign64190_e99151_d_n7, assign64190_e99151_d_n8, assign64190_e99151_d_n9, assign64190_e99151_d_n10, assign64190_e99151_d_n11, assign64190_e99151_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign64190_e99143: f64 = (locals.var_costi7 * locals.var_mu);
        let assign64190_e99145: f64 = (assign64190_e99143 * locals.var_qn0sti);
        let assign64190_e99147: f64 = (assign64190_e99145 * locals.var_ty);
        let assign64190_e99149: f64 = (assign64190_e99147 / locals.var_t1);
        (assign64190_e99149, (((((((((locals.var_costi7_dn0 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn0)) * locals.var_qn0sti) + (assign64190_e99143 * locals.var_qn0sti_dn0)) * locals.var_ty) + (assign64190_e99145 * locals.var_ty_dn0)) * locals.var_t1) - (assign64190_e99147 * locals.var_t1_dn0)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn2 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn2)) * locals.var_qn0sti) + (assign64190_e99143 * locals.var_qn0sti_dn2)) * locals.var_ty) + (assign64190_e99145 * locals.var_ty_dn2)) * locals.var_t1) - (assign64190_e99147 * locals.var_t1_dn2)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn4 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn4)) * locals.var_qn0sti) + (assign64190_e99143 * locals.var_qn0sti_dn4)) * locals.var_ty) + (assign64190_e99145 * locals.var_ty_dn4)) * locals.var_t1) - (assign64190_e99147 * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn5 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn5)) * locals.var_qn0sti) + (assign64190_e99143 * locals.var_qn0sti_dn5)) * locals.var_ty) + (assign64190_e99145 * locals.var_ty_dn5)) * locals.var_t1) - (assign64190_e99147 * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn6 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn6)) * locals.var_qn0sti) + (assign64190_e99143 * locals.var_qn0sti_dn6)) * locals.var_ty) + (assign64190_e99145 * locals.var_ty_dn6)) * locals.var_t1) - (assign64190_e99147 * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn7 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn7)) * locals.var_qn0sti) + (assign64190_e99143 * locals.var_qn0sti_dn7)) * locals.var_ty) + (assign64190_e99145 * locals.var_ty_dn7)) * locals.var_t1) - (assign64190_e99147 * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn8 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn8)) * locals.var_qn0sti) + (assign64190_e99143 * locals.var_qn0sti_dn8)) * locals.var_ty) + (assign64190_e99145 * locals.var_ty_dn8)) * locals.var_t1) - (assign64190_e99147 * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn9 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn9)) * locals.var_qn0sti) + (assign64190_e99143 * locals.var_qn0sti_dn9)) * locals.var_ty) + (assign64190_e99145 * locals.var_ty_dn9)) * locals.var_t1) - (assign64190_e99147 * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn10 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn10)) * locals.var_qn0sti) + (assign64190_e99143 * locals.var_qn0sti_dn10)) * locals.var_ty) + (assign64190_e99145 * locals.var_ty_dn10)) * locals.var_t1) - (assign64190_e99147 * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn11 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn11)) * locals.var_qn0sti) + (assign64190_e99143 * locals.var_qn0sti_dn11)) * locals.var_ty) + (assign64190_e99145 * locals.var_ty_dn11)) * locals.var_t1) - (assign64190_e99147 * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn14 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn14)) * locals.var_qn0sti) + (assign64190_e99143 * locals.var_qn0sti_dn14)) * locals.var_ty) + (assign64190_e99145 * locals.var_ty_dn14)) * locals.var_t1) - (assign64190_e99147 * locals.var_t1_dn14)) / (locals.var_t1 * locals.var_t1)),)
    } else {
        (locals.var_idssti, locals.var_idssti_dn0, locals.var_idssti_dn2, locals.var_idssti_dn4, locals.var_idssti_dn5, locals.var_idssti_dn6, locals.var_idssti_dn7, locals.var_idssti_dn8, locals.var_idssti_dn9, locals.var_idssti_dn10, locals.var_idssti_dn11, locals.var_idssti_dn14,)
    }
};
        locals.var_idssti = assign64190_e99151;
        locals.var_idssti_dn0 = assign64190_e99151_d_n0;
        locals.var_idssti_dn2 = assign64190_e99151_d_n2;
        locals.var_idssti_dn4 = assign64190_e99151_d_n4;
        locals.var_idssti_dn5 = assign64190_e99151_d_n5;
        locals.var_idssti_dn6 = assign64190_e99151_d_n6;
        locals.var_idssti_dn7 = assign64190_e99151_d_n7;
        locals.var_idssti_dn8 = assign64190_e99151_d_n8;
        locals.var_idssti_dn9 = assign64190_e99151_d_n9;
        locals.var_idssti_dn10 = assign64190_e99151_d_n10;
        locals.var_idssti_dn11 = assign64190_e99151_d_n11;
        locals.var_idssti_dn14 = assign64190_e99151_d_n14;
        locals.var_idssti_rv = 0.0;

        let (assign64200_e99162, assign64200_e99162_d_n0, assign64200_e99162_d_n2, assign64200_e99162_d_n4, assign64200_e99162_d_n5, assign64200_e99162_d_n6, assign64200_e99162_d_n7, assign64200_e99162_d_n8, assign64200_e99162_d_n9, assign64200_e99162_d_n10, assign64200_e99162_d_n11, assign64200_e99162_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        let assign64200_e99160: f64 = (locals.var_ids + locals.var_idssti);
        (assign64200_e99160, (locals.var_ids_dn0 + locals.var_idssti_dn0), (locals.var_ids_dn2 + locals.var_idssti_dn2), (locals.var_ids_dn4 + locals.var_idssti_dn4), (locals.var_ids_dn5 + locals.var_idssti_dn5), (locals.var_ids_dn6 + locals.var_idssti_dn6), (locals.var_ids_dn7 + locals.var_idssti_dn7), (locals.var_ids_dn8 + locals.var_idssti_dn8), (locals.var_ids_dn9 + locals.var_idssti_dn9), (locals.var_ids_dn10 + locals.var_idssti_dn10), (locals.var_ids_dn11 + locals.var_idssti_dn11), (locals.var_ids_dn14 + locals.var_idssti_dn14),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn14,)
    }
};
        locals.var_ids = assign64200_e99162;
        locals.var_ids_dn0 = assign64200_e99162_d_n0;
        locals.var_ids_dn2 = assign64200_e99162_d_n2;
        locals.var_ids_dn4 = assign64200_e99162_d_n4;
        locals.var_ids_dn5 = assign64200_e99162_d_n5;
        locals.var_ids_dn6 = assign64200_e99162_d_n6;
        locals.var_ids_dn7 = assign64200_e99162_d_n7;
        locals.var_ids_dn8 = assign64200_e99162_d_n8;
        locals.var_ids_dn9 = assign64200_e99162_d_n9;
        locals.var_ids_dn10 = assign64200_e99162_d_n10;
        locals.var_ids_dn11 = assign64200_e99162_d_n11;
        locals.var_ids_dn14 = assign64200_e99162_d_n14;
        locals.var_ids_rv = 0.0;

        let assign64210_e99173: f64 = if (((p.p31 != 0.0) && (p.p30 != 0.0)) && (locals.var_uc_codep == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1517 = assign64210_e99173;
        locals.var_guard1517_rv = 0.0;

        let (assign64220_e99184, assign64220_e99184_d_n0, assign64220_e99184_d_n2, assign64220_e99184_d_n4, assign64220_e99184_d_n5, assign64220_e99184_d_n6, assign64220_e99184_d_n7, assign64220_e99184_d_n8, assign64220_e99184_d_n9, assign64220_e99184_d_n10, assign64220_e99184_d_n11, assign64220_e99184_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1517 != 0.0)) {
        let assign64220_e99182: f64 = (locals.var_vgvt * locals.var_vgvt);
        (assign64220_e99182, ((locals.var_vgvt_dn0 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn0)), ((locals.var_vgvt_dn2 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn2)), ((locals.var_vgvt_dn4 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn4)), ((locals.var_vgvt_dn5 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn5)), ((locals.var_vgvt_dn6 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn6)), ((locals.var_vgvt_dn7 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn7)), ((locals.var_vgvt_dn8 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn8)), ((locals.var_vgvt_dn9 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn9)), ((locals.var_vgvt_dn10 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn10)), ((locals.var_vgvt_dn11 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn11)), ((locals.var_vgvt_dn14 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn14)),)
    } else {
        (locals.var_kusai00, locals.var_kusai00_dn0, locals.var_kusai00_dn2, locals.var_kusai00_dn4, locals.var_kusai00_dn5, locals.var_kusai00_dn6, locals.var_kusai00_dn7, locals.var_kusai00_dn8, locals.var_kusai00_dn9, locals.var_kusai00_dn10, locals.var_kusai00_dn11, locals.var_kusai00_dn14,)
    }
};
        locals.var_kusai00 = assign64220_e99184;
        locals.var_kusai00_dn0 = assign64220_e99184_d_n0;
        locals.var_kusai00_dn2 = assign64220_e99184_d_n2;
        locals.var_kusai00_dn4 = assign64220_e99184_d_n4;
        locals.var_kusai00_dn5 = assign64220_e99184_d_n5;
        locals.var_kusai00_dn6 = assign64220_e99184_d_n6;
        locals.var_kusai00_dn7 = assign64220_e99184_d_n7;
        locals.var_kusai00_dn8 = assign64220_e99184_d_n8;
        locals.var_kusai00_dn9 = assign64220_e99184_d_n9;
        locals.var_kusai00_dn10 = assign64220_e99184_d_n10;
        locals.var_kusai00_dn11 = assign64220_e99184_d_n11;
        locals.var_kusai00_dn14 = assign64220_e99184_d_n14;
        locals.var_kusai00_rv = 0.0;

        let (assign64230_e99199, assign64230_e99199_d_n0, assign64230_e99199_d_n2, assign64230_e99199_d_n4, assign64230_e99199_d_n5, assign64230_e99199_d_n6, assign64230_e99199_d_n7, assign64230_e99199_d_n8, assign64230_e99199_d_n9, assign64230_e99199_d_n10, assign64230_e99199_d_n11, assign64230_e99199_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1517 != 0.0)) {
        let assign64230_e99193: f64 = (2.0 * locals.var_beta_inv);
        let assign64230_e99195: f64 = (assign64230_e99193 * locals.var_cox_inv);
        let assign64230_e99197: f64 = (assign64230_e99195 * locals.var_idd);
        (assign64230_e99197, (((((2.0 * locals.var_beta_inv_dn0) * locals.var_cox_inv) + (assign64230_e99193 * locals.var_cox_inv_dn0)) * locals.var_idd) + (assign64230_e99195 * locals.var_idd_dn0)), (((((2.0 * locals.var_beta_inv_dn2) * locals.var_cox_inv) + (assign64230_e99193 * locals.var_cox_inv_dn2)) * locals.var_idd) + (assign64230_e99195 * locals.var_idd_dn2)), (((((2.0 * locals.var_beta_inv_dn4) * locals.var_cox_inv) + (assign64230_e99193 * locals.var_cox_inv_dn4)) * locals.var_idd) + (assign64230_e99195 * locals.var_idd_dn4)), (((((2.0 * locals.var_beta_inv_dn5) * locals.var_cox_inv) + (assign64230_e99193 * locals.var_cox_inv_dn5)) * locals.var_idd) + (assign64230_e99195 * locals.var_idd_dn5)), (((((2.0 * locals.var_beta_inv_dn6) * locals.var_cox_inv) + (assign64230_e99193 * locals.var_cox_inv_dn6)) * locals.var_idd) + (assign64230_e99195 * locals.var_idd_dn6)), (((((2.0 * locals.var_beta_inv_dn7) * locals.var_cox_inv) + (assign64230_e99193 * locals.var_cox_inv_dn7)) * locals.var_idd) + (assign64230_e99195 * locals.var_idd_dn7)), (((((2.0 * locals.var_beta_inv_dn8) * locals.var_cox_inv) + (assign64230_e99193 * locals.var_cox_inv_dn8)) * locals.var_idd) + (assign64230_e99195 * locals.var_idd_dn8)), (((((2.0 * locals.var_beta_inv_dn9) * locals.var_cox_inv) + (assign64230_e99193 * locals.var_cox_inv_dn9)) * locals.var_idd) + (assign64230_e99195 * locals.var_idd_dn9)), (((((2.0 * locals.var_beta_inv_dn10) * locals.var_cox_inv) + (assign64230_e99193 * locals.var_cox_inv_dn10)) * locals.var_idd) + (assign64230_e99195 * locals.var_idd_dn10)), (((((2.0 * locals.var_beta_inv_dn11) * locals.var_cox_inv) + (assign64230_e99193 * locals.var_cox_inv_dn11)) * locals.var_idd) + (assign64230_e99195 * locals.var_idd_dn11)), (((((2.0 * locals.var_beta_inv_dn14) * locals.var_cox_inv) + (assign64230_e99193 * locals.var_cox_inv_dn14)) * locals.var_idd) + (assign64230_e99195 * locals.var_idd_dn14)),)
    } else {
        (locals.var_kusaidd, locals.var_kusaidd_dn0, locals.var_kusaidd_dn2, locals.var_kusaidd_dn4, locals.var_kusaidd_dn5, locals.var_kusaidd_dn6, locals.var_kusaidd_dn7, locals.var_kusaidd_dn8, locals.var_kusaidd_dn9, locals.var_kusaidd_dn10, locals.var_kusaidd_dn11, locals.var_kusaidd_dn14,)
    }
};
        locals.var_kusaidd = assign64230_e99199;
        locals.var_kusaidd_dn0 = assign64230_e99199_d_n0;
        locals.var_kusaidd_dn2 = assign64230_e99199_d_n2;
        locals.var_kusaidd_dn4 = assign64230_e99199_d_n4;
        locals.var_kusaidd_dn5 = assign64230_e99199_d_n5;
        locals.var_kusaidd_dn6 = assign64230_e99199_d_n6;
        locals.var_kusaidd_dn7 = assign64230_e99199_d_n7;
        locals.var_kusaidd_dn8 = assign64230_e99199_d_n8;
        locals.var_kusaidd_dn9 = assign64230_e99199_d_n9;
        locals.var_kusaidd_dn10 = assign64230_e99199_d_n10;
        locals.var_kusaidd_dn11 = assign64230_e99199_d_n11;
        locals.var_kusaidd_dn14 = assign64230_e99199_d_n14;
        locals.var_kusaidd_rv = 0.0;

        let (assign64240_e99210, assign64240_e99210_d_n0, assign64240_e99210_d_n2, assign64240_e99210_d_n4, assign64240_e99210_d_n5, assign64240_e99210_d_n6, assign64240_e99210_d_n7, assign64240_e99210_d_n8, assign64240_e99210_d_n9, assign64240_e99210_d_n10, assign64240_e99210_d_n11, assign64240_e99210_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1517 != 0.0)) {
        let assign64240_e99208: f64 = (locals.var_kusai00 - locals.var_kusaidd);
        (assign64240_e99208, (locals.var_kusai00_dn0 - locals.var_kusaidd_dn0), (locals.var_kusai00_dn2 - locals.var_kusaidd_dn2), (locals.var_kusai00_dn4 - locals.var_kusaidd_dn4), (locals.var_kusai00_dn5 - locals.var_kusaidd_dn5), (locals.var_kusai00_dn6 - locals.var_kusaidd_dn6), (locals.var_kusai00_dn7 - locals.var_kusaidd_dn7), (locals.var_kusai00_dn8 - locals.var_kusaidd_dn8), (locals.var_kusai00_dn9 - locals.var_kusaidd_dn9), (locals.var_kusai00_dn10 - locals.var_kusaidd_dn10), (locals.var_kusai00_dn11 - locals.var_kusaidd_dn11), (locals.var_kusai00_dn14 - locals.var_kusaidd_dn14),)
    } else {
        (locals.var_kusail, locals.var_kusail_dn0, locals.var_kusail_dn2, locals.var_kusail_dn4, locals.var_kusail_dn5, locals.var_kusail_dn6, locals.var_kusail_dn7, locals.var_kusail_dn8, locals.var_kusail_dn9, locals.var_kusail_dn10, locals.var_kusail_dn11, locals.var_kusail_dn14,)
    }
};
        locals.var_kusail = assign64240_e99210;
        locals.var_kusail_dn0 = assign64240_e99210_d_n0;
        locals.var_kusail_dn2 = assign64240_e99210_d_n2;
        locals.var_kusail_dn4 = assign64240_e99210_d_n4;
        locals.var_kusail_dn5 = assign64240_e99210_d_n5;
        locals.var_kusail_dn6 = assign64240_e99210_d_n6;
        locals.var_kusail_dn7 = assign64240_e99210_d_n7;
        locals.var_kusail_dn8 = assign64240_e99210_d_n8;
        locals.var_kusail_dn9 = assign64240_e99210_d_n9;
        locals.var_kusail_dn10 = assign64240_e99210_d_n10;
        locals.var_kusail_dn11 = assign64240_e99210_d_n11;
        locals.var_kusail_dn14 = assign64240_e99210_d_n14;
        locals.var_kusail_rv = 0.0;

        let (assign64250_e99228, assign64250_e99228_d_n0, assign64250_e99228_d_n2, assign64250_e99228_d_n4, assign64250_e99228_d_n5, assign64250_e99228_d_n6, assign64250_e99228_d_n7, assign64250_e99228_d_n8, assign64250_e99228_d_n9, assign64250_e99228_d_n10, assign64250_e99228_d_n11, assign64250_e99228_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1517 != 0.0)) {
        let assign64250_e99219: f64 = (locals.var_kusai00 * locals.var_kusai00);
        let assign64250_e99222: f64 = (4.0 * 0.001);
        let assign64250_e99224: f64 = (assign64250_e99222 * 0.001);
        let assign64250_e99225: f64 = (assign64250_e99219 + assign64250_e99224);
        let assign64250_e99226: f64 = (assign64250_e99225).sqrt();
        (assign64250_e99226, (((locals.var_kusai00_dn0 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn0)) / (2.0 * assign64250_e99226)), (((locals.var_kusai00_dn2 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn2)) / (2.0 * assign64250_e99226)), (((locals.var_kusai00_dn4 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn4)) / (2.0 * assign64250_e99226)), (((locals.var_kusai00_dn5 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn5)) / (2.0 * assign64250_e99226)), (((locals.var_kusai00_dn6 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn6)) / (2.0 * assign64250_e99226)), (((locals.var_kusai00_dn7 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn7)) / (2.0 * assign64250_e99226)), (((locals.var_kusai00_dn8 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn8)) / (2.0 * assign64250_e99226)), (((locals.var_kusai00_dn9 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn9)) / (2.0 * assign64250_e99226)), (((locals.var_kusai00_dn10 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn10)) / (2.0 * assign64250_e99226)), (((locals.var_kusai00_dn11 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn11)) / (2.0 * assign64250_e99226)), (((locals.var_kusai00_dn14 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn14)) / (2.0 * assign64250_e99226)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign64250_e99228;
        locals.var_tmf2_dn0 = assign64250_e99228_d_n0;
        locals.var_tmf2_dn2 = assign64250_e99228_d_n2;
        locals.var_tmf2_dn4 = assign64250_e99228_d_n4;
        locals.var_tmf2_dn5 = assign64250_e99228_d_n5;
        locals.var_tmf2_dn6 = assign64250_e99228_d_n6;
        locals.var_tmf2_dn7 = assign64250_e99228_d_n7;
        locals.var_tmf2_dn8 = assign64250_e99228_d_n8;
        locals.var_tmf2_dn9 = assign64250_e99228_d_n9;
        locals.var_tmf2_dn10 = assign64250_e99228_d_n10;
        locals.var_tmf2_dn11 = assign64250_e99228_d_n11;
        locals.var_tmf2_dn14 = assign64250_e99228_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign64260_e99243, assign64260_e99243_d_n0, assign64260_e99243_d_n2, assign64260_e99243_d_n4, assign64260_e99243_d_n5, assign64260_e99243_d_n6, assign64260_e99243_d_n7, assign64260_e99243_d_n8, assign64260_e99243_d_n9, assign64260_e99243_d_n10, assign64260_e99243_d_n11, assign64260_e99243_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1517 != 0.0)) {
        let assign64260_e99239: f64 = (locals.var_kusai00 / locals.var_tmf2);
        let assign64260_e99240: f64 = (1.0 + assign64260_e99239);
        let assign64260_e99241: f64 = (0.5 * assign64260_e99240);
        (assign64260_e99241, (0.5 * (((locals.var_kusai00_dn0 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn2 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn4 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn5 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn6 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn7 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn8 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn9 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn10 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn11 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn14 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign64260_e99243;
        locals.var_t0_dn0 = assign64260_e99243_d_n0;
        locals.var_t0_dn2 = assign64260_e99243_d_n2;
        locals.var_t0_dn4 = assign64260_e99243_d_n4;
        locals.var_t0_dn5 = assign64260_e99243_d_n5;
        locals.var_t0_dn6 = assign64260_e99243_d_n6;
        locals.var_t0_dn7 = assign64260_e99243_d_n7;
        locals.var_t0_dn8 = assign64260_e99243_d_n8;
        locals.var_t0_dn9 = assign64260_e99243_d_n9;
        locals.var_t0_dn10 = assign64260_e99243_d_n10;
        locals.var_t0_dn11 = assign64260_e99243_d_n11;
        locals.var_t0_dn14 = assign64260_e99243_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign64270_e99256, assign64270_e99256_d_n0, assign64270_e99256_d_n2, assign64270_e99256_d_n4, assign64270_e99256_d_n5, assign64270_e99256_d_n6, assign64270_e99256_d_n7, assign64270_e99256_d_n8, assign64270_e99256_d_n9, assign64270_e99256_d_n10, assign64270_e99256_d_n11, assign64270_e99256_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1517 != 0.0)) {
        let assign64270_e99253: f64 = (locals.var_kusai00 + locals.var_tmf2);
        let assign64270_e99254: f64 = (0.5 * assign64270_e99253);
        (assign64270_e99254, (0.5 * (locals.var_kusai00_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_kusai00_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_kusai00_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_kusai00_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_kusai00_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_kusai00_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_kusai00_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_kusai00_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_kusai00_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_kusai00_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_kusai00_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_kusai00, locals.var_kusai00_dn0, locals.var_kusai00_dn2, locals.var_kusai00_dn4, locals.var_kusai00_dn5, locals.var_kusai00_dn6, locals.var_kusai00_dn7, locals.var_kusai00_dn8, locals.var_kusai00_dn9, locals.var_kusai00_dn10, locals.var_kusai00_dn11, locals.var_kusai00_dn14,)
    }
};
        locals.var_kusai00 = assign64270_e99256;
        locals.var_kusai00_dn0 = assign64270_e99256_d_n0;
        locals.var_kusai00_dn2 = assign64270_e99256_d_n2;
        locals.var_kusai00_dn4 = assign64270_e99256_d_n4;
        locals.var_kusai00_dn5 = assign64270_e99256_d_n5;
        locals.var_kusai00_dn6 = assign64270_e99256_d_n6;
        locals.var_kusai00_dn7 = assign64270_e99256_d_n7;
        locals.var_kusai00_dn8 = assign64270_e99256_d_n8;
        locals.var_kusai00_dn9 = assign64270_e99256_d_n9;
        locals.var_kusai00_dn10 = assign64270_e99256_d_n10;
        locals.var_kusai00_dn11 = assign64270_e99256_d_n11;
        locals.var_kusai00_dn14 = assign64270_e99256_d_n14;
        locals.var_kusai00_rv = 0.0;

        let assign64280_e99259: f64 = if locals.var_kusai00 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1518 = assign64280_e99259;
        locals.var_guard1518_rv = 0.0;

        let (assign64290_e99270, assign64290_e99270_d_n0, assign64290_e99270_d_n2, assign64290_e99270_d_n4, assign64290_e99270_d_n5, assign64290_e99270_d_n6, assign64290_e99270_d_n7, assign64290_e99270_d_n8, assign64290_e99270_d_n9, assign64290_e99270_d_n10, assign64290_e99270_d_n11, assign64290_e99270_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1517 != 0.0)) && (locals.var_guard1518 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_kusai00, locals.var_kusai00_dn0, locals.var_kusai00_dn2, locals.var_kusai00_dn4, locals.var_kusai00_dn5, locals.var_kusai00_dn6, locals.var_kusai00_dn7, locals.var_kusai00_dn8, locals.var_kusai00_dn9, locals.var_kusai00_dn10, locals.var_kusai00_dn11, locals.var_kusai00_dn14,)
    }
};
        locals.var_kusai00 = assign64290_e99270;
        locals.var_kusai00_dn0 = assign64290_e99270_d_n0;
        locals.var_kusai00_dn2 = assign64290_e99270_d_n2;
        locals.var_kusai00_dn4 = assign64290_e99270_d_n4;
        locals.var_kusai00_dn5 = assign64290_e99270_d_n5;
        locals.var_kusai00_dn6 = assign64290_e99270_d_n6;
        locals.var_kusai00_dn7 = assign64290_e99270_d_n7;
        locals.var_kusai00_dn8 = assign64290_e99270_d_n8;
        locals.var_kusai00_dn9 = assign64290_e99270_d_n9;
        locals.var_kusai00_dn10 = assign64290_e99270_d_n10;
        locals.var_kusai00_dn11 = assign64290_e99270_d_n11;
        locals.var_kusai00_dn14 = assign64290_e99270_d_n14;
        locals.var_kusai00_rv = 0.0;

        let (assign64300_e99281, assign64300_e99281_d_n0, assign64300_e99281_d_n2, assign64300_e99281_d_n4, assign64300_e99281_d_n5, assign64300_e99281_d_n6, assign64300_e99281_d_n7, assign64300_e99281_d_n8, assign64300_e99281_d_n9, assign64300_e99281_d_n10, assign64300_e99281_d_n11, assign64300_e99281_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1517 != 0.0)) && (locals.var_guard1518 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign64300_e99281;
        locals.var_t0_dn0 = assign64300_e99281_d_n0;
        locals.var_t0_dn2 = assign64300_e99281_d_n2;
        locals.var_t0_dn4 = assign64300_e99281_d_n4;
        locals.var_t0_dn5 = assign64300_e99281_d_n5;
        locals.var_t0_dn6 = assign64300_e99281_d_n6;
        locals.var_t0_dn7 = assign64300_e99281_d_n7;
        locals.var_t0_dn8 = assign64300_e99281_d_n8;
        locals.var_t0_dn9 = assign64300_e99281_d_n9;
        locals.var_t0_dn10 = assign64300_e99281_d_n10;
        locals.var_t0_dn11 = assign64300_e99281_d_n11;
        locals.var_t0_dn14 = assign64300_e99281_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign64310_e99299, assign64310_e99299_d_n0, assign64310_e99299_d_n2, assign64310_e99299_d_n4, assign64310_e99299_d_n5, assign64310_e99299_d_n6, assign64310_e99299_d_n7, assign64310_e99299_d_n8, assign64310_e99299_d_n9, assign64310_e99299_d_n10, assign64310_e99299_d_n11, assign64310_e99299_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1517 != 0.0)) {
        let assign64310_e99290: f64 = (locals.var_kusail * locals.var_kusail);
        let assign64310_e99293: f64 = (4.0 * 0.001);
        let assign64310_e99295: f64 = (assign64310_e99293 * 0.001);
        let assign64310_e99296: f64 = (assign64310_e99290 + assign64310_e99295);
        let assign64310_e99297: f64 = (assign64310_e99296).sqrt();
        (assign64310_e99297, (((locals.var_kusail_dn0 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn0)) / (2.0 * assign64310_e99297)), (((locals.var_kusail_dn2 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn2)) / (2.0 * assign64310_e99297)), (((locals.var_kusail_dn4 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn4)) / (2.0 * assign64310_e99297)), (((locals.var_kusail_dn5 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn5)) / (2.0 * assign64310_e99297)), (((locals.var_kusail_dn6 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn6)) / (2.0 * assign64310_e99297)), (((locals.var_kusail_dn7 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn7)) / (2.0 * assign64310_e99297)), (((locals.var_kusail_dn8 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn8)) / (2.0 * assign64310_e99297)), (((locals.var_kusail_dn9 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn9)) / (2.0 * assign64310_e99297)), (((locals.var_kusail_dn10 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn10)) / (2.0 * assign64310_e99297)), (((locals.var_kusail_dn11 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn11)) / (2.0 * assign64310_e99297)), (((locals.var_kusail_dn14 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn14)) / (2.0 * assign64310_e99297)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign64310_e99299;
        locals.var_tmf2_dn0 = assign64310_e99299_d_n0;
        locals.var_tmf2_dn2 = assign64310_e99299_d_n2;
        locals.var_tmf2_dn4 = assign64310_e99299_d_n4;
        locals.var_tmf2_dn5 = assign64310_e99299_d_n5;
        locals.var_tmf2_dn6 = assign64310_e99299_d_n6;
        locals.var_tmf2_dn7 = assign64310_e99299_d_n7;
        locals.var_tmf2_dn8 = assign64310_e99299_d_n8;
        locals.var_tmf2_dn9 = assign64310_e99299_d_n9;
        locals.var_tmf2_dn10 = assign64310_e99299_d_n10;
        locals.var_tmf2_dn11 = assign64310_e99299_d_n11;
        locals.var_tmf2_dn14 = assign64310_e99299_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign64320_e99314, assign64320_e99314_d_n0, assign64320_e99314_d_n2, assign64320_e99314_d_n4, assign64320_e99314_d_n5, assign64320_e99314_d_n6, assign64320_e99314_d_n7, assign64320_e99314_d_n8, assign64320_e99314_d_n9, assign64320_e99314_d_n10, assign64320_e99314_d_n11, assign64320_e99314_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1517 != 0.0)) {
        let assign64320_e99310: f64 = (locals.var_kusail / locals.var_tmf2);
        let assign64320_e99311: f64 = (1.0 + assign64320_e99310);
        let assign64320_e99312: f64 = (0.5 * assign64320_e99311);
        (assign64320_e99312, (0.5 * (((locals.var_kusail_dn0 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn2 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn4 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn5 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn6 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn7 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn8 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn9 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn10 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn11 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn14 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign64320_e99314;
        locals.var_t0_dn0 = assign64320_e99314_d_n0;
        locals.var_t0_dn2 = assign64320_e99314_d_n2;
        locals.var_t0_dn4 = assign64320_e99314_d_n4;
        locals.var_t0_dn5 = assign64320_e99314_d_n5;
        locals.var_t0_dn6 = assign64320_e99314_d_n6;
        locals.var_t0_dn7 = assign64320_e99314_d_n7;
        locals.var_t0_dn8 = assign64320_e99314_d_n8;
        locals.var_t0_dn9 = assign64320_e99314_d_n9;
        locals.var_t0_dn10 = assign64320_e99314_d_n10;
        locals.var_t0_dn11 = assign64320_e99314_d_n11;
        locals.var_t0_dn14 = assign64320_e99314_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign64330_e99327, assign64330_e99327_d_n0, assign64330_e99327_d_n2, assign64330_e99327_d_n4, assign64330_e99327_d_n5, assign64330_e99327_d_n6, assign64330_e99327_d_n7, assign64330_e99327_d_n8, assign64330_e99327_d_n9, assign64330_e99327_d_n10, assign64330_e99327_d_n11, assign64330_e99327_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1517 != 0.0)) {
        let assign64330_e99324: f64 = (locals.var_kusail + locals.var_tmf2);
        let assign64330_e99325: f64 = (0.5 * assign64330_e99324);
        (assign64330_e99325, (0.5 * (locals.var_kusail_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_kusail_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_kusail_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_kusail_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_kusail_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_kusail_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_kusail_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_kusail_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_kusail_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_kusail_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_kusail_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_kusail, locals.var_kusail_dn0, locals.var_kusail_dn2, locals.var_kusail_dn4, locals.var_kusail_dn5, locals.var_kusail_dn6, locals.var_kusail_dn7, locals.var_kusail_dn8, locals.var_kusail_dn9, locals.var_kusail_dn10, locals.var_kusail_dn11, locals.var_kusail_dn14,)
    }
};
        locals.var_kusail = assign64330_e99327;
        locals.var_kusail_dn0 = assign64330_e99327_d_n0;
        locals.var_kusail_dn2 = assign64330_e99327_d_n2;
        locals.var_kusail_dn4 = assign64330_e99327_d_n4;
        locals.var_kusail_dn5 = assign64330_e99327_d_n5;
        locals.var_kusail_dn6 = assign64330_e99327_d_n6;
        locals.var_kusail_dn7 = assign64330_e99327_d_n7;
        locals.var_kusail_dn8 = assign64330_e99327_d_n8;
        locals.var_kusail_dn9 = assign64330_e99327_d_n9;
        locals.var_kusail_dn10 = assign64330_e99327_d_n10;
        locals.var_kusail_dn11 = assign64330_e99327_d_n11;
        locals.var_kusail_dn14 = assign64330_e99327_d_n14;
        locals.var_kusail_rv = 0.0;

        let assign64340_e99330: f64 = if locals.var_kusail < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1519 = assign64340_e99330;
        locals.var_guard1519_rv = 0.0;

        let (assign64350_e99341, assign64350_e99341_d_n0, assign64350_e99341_d_n2, assign64350_e99341_d_n4, assign64350_e99341_d_n5, assign64350_e99341_d_n6, assign64350_e99341_d_n7, assign64350_e99341_d_n8, assign64350_e99341_d_n9, assign64350_e99341_d_n10, assign64350_e99341_d_n11, assign64350_e99341_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1517 != 0.0)) && (locals.var_guard1519 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_kusail, locals.var_kusail_dn0, locals.var_kusail_dn2, locals.var_kusail_dn4, locals.var_kusail_dn5, locals.var_kusail_dn6, locals.var_kusail_dn7, locals.var_kusail_dn8, locals.var_kusail_dn9, locals.var_kusail_dn10, locals.var_kusail_dn11, locals.var_kusail_dn14,)
    }
};
        locals.var_kusail = assign64350_e99341;
        locals.var_kusail_dn0 = assign64350_e99341_d_n0;
        locals.var_kusail_dn2 = assign64350_e99341_d_n2;
        locals.var_kusail_dn4 = assign64350_e99341_d_n4;
        locals.var_kusail_dn5 = assign64350_e99341_d_n5;
        locals.var_kusail_dn6 = assign64350_e99341_d_n6;
        locals.var_kusail_dn7 = assign64350_e99341_d_n7;
        locals.var_kusail_dn8 = assign64350_e99341_d_n8;
        locals.var_kusail_dn9 = assign64350_e99341_d_n9;
        locals.var_kusail_dn10 = assign64350_e99341_d_n10;
        locals.var_kusail_dn11 = assign64350_e99341_d_n11;
        locals.var_kusail_dn14 = assign64350_e99341_d_n14;
        locals.var_kusail_rv = 0.0;

        let (assign64360_e99352, assign64360_e99352_d_n0, assign64360_e99352_d_n2, assign64360_e99352_d_n4, assign64360_e99352_d_n5, assign64360_e99352_d_n6, assign64360_e99352_d_n7, assign64360_e99352_d_n8, assign64360_e99352_d_n9, assign64360_e99352_d_n10, assign64360_e99352_d_n11, assign64360_e99352_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1517 != 0.0)) && (locals.var_guard1519 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign64360_e99352;
        locals.var_t0_dn0 = assign64360_e99352_d_n0;
        locals.var_t0_dn2 = assign64360_e99352_d_n2;
        locals.var_t0_dn4 = assign64360_e99352_d_n4;
        locals.var_t0_dn5 = assign64360_e99352_d_n5;
        locals.var_t0_dn6 = assign64360_e99352_d_n6;
        locals.var_t0_dn7 = assign64360_e99352_d_n7;
        locals.var_t0_dn8 = assign64360_e99352_d_n8;
        locals.var_t0_dn9 = assign64360_e99352_d_n9;
        locals.var_t0_dn10 = assign64360_e99352_d_n10;
        locals.var_t0_dn11 = assign64360_e99352_d_n11;
        locals.var_t0_dn14 = assign64360_e99352_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign64370_e99363, assign64370_e99363_d_n0, assign64370_e99363_d_n2, assign64370_e99363_d_n4, assign64370_e99363_d_n5, assign64370_e99363_d_n6, assign64370_e99363_d_n7, assign64370_e99363_d_n8, assign64370_e99363_d_n9, assign64370_e99363_d_n10, assign64370_e99363_d_n11, assign64370_e99363_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1517 != 0.0)) {
        let assign64370_e99361: f64 = (locals.var_kusai00 - locals.var_kusail);
        (assign64370_e99361, (locals.var_kusai00_dn0 - locals.var_kusail_dn0), (locals.var_kusai00_dn2 - locals.var_kusail_dn2), (locals.var_kusai00_dn4 - locals.var_kusail_dn4), (locals.var_kusai00_dn5 - locals.var_kusail_dn5), (locals.var_kusai00_dn6 - locals.var_kusail_dn6), (locals.var_kusai00_dn7 - locals.var_kusail_dn7), (locals.var_kusai00_dn8 - locals.var_kusail_dn8), (locals.var_kusai00_dn9 - locals.var_kusail_dn9), (locals.var_kusai00_dn10 - locals.var_kusail_dn10), (locals.var_kusai00_dn11 - locals.var_kusail_dn11), (locals.var_kusai00_dn14 - locals.var_kusail_dn14),)
    } else {
        (locals.var_kusai00l, locals.var_kusai00l_dn0, locals.var_kusai00l_dn2, locals.var_kusai00l_dn4, locals.var_kusai00l_dn5, locals.var_kusai00l_dn6, locals.var_kusai00l_dn7, locals.var_kusai00l_dn8, locals.var_kusai00l_dn9, locals.var_kusai00l_dn10, locals.var_kusai00l_dn11, locals.var_kusai00l_dn14,)
    }
};
        locals.var_kusai00l = assign64370_e99363;
        locals.var_kusai00l_dn0 = assign64370_e99363_d_n0;
        locals.var_kusai00l_dn2 = assign64370_e99363_d_n2;
        locals.var_kusai00l_dn4 = assign64370_e99363_d_n4;
        locals.var_kusai00l_dn5 = assign64370_e99363_d_n5;
        locals.var_kusai00l_dn6 = assign64370_e99363_d_n6;
        locals.var_kusai00l_dn7 = assign64370_e99363_d_n7;
        locals.var_kusai00l_dn8 = assign64370_e99363_d_n8;
        locals.var_kusai00l_dn9 = assign64370_e99363_d_n9;
        locals.var_kusai00l_dn10 = assign64370_e99363_d_n10;
        locals.var_kusai00l_dn11 = assign64370_e99363_d_n11;
        locals.var_kusai00l_dn14 = assign64370_e99363_d_n14;
        locals.var_kusai00l_rv = 0.0;

        let assign64380_e99367: f64 = (10.0 * 2.220446049250313e-16);
        let assign64380_e99372: f64 = (10.0 * 2.220446049250313e-16);
        let assign64380_e99374: f64 = if ((locals.var_qn0 < assign64380_e99367) || (locals.var_kusai00l < assign64380_e99372)) { 1.0 } else { 0.0 };
        locals.var_guard1520 = assign64380_e99374;
        locals.var_guard1520_rv = 0.0;

        let (assign64390_e99385,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1517 != 0.0)) && (locals.var_guard1520 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_ign,)
    }
};
        locals.var_flg_ign = assign64390_e99385;
        locals.var_flg_ign_rv = 0.0;

        let (assign64400_e99397,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1517 != 0.0)) && (locals.var_guard1520 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_ign,)
    }
};
        locals.var_flg_ign = assign64400_e99397;
        locals.var_flg_ign_rv = 0.0;

        let (assign64410_e99404,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_end_of_part_1 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_end_of_part_1,)
    }
};
        locals.var_end_of_part_1 = assign64410_e99404;
        locals.var_end_of_part_1_rv = 0.0;

        let assign64420_e99411: f64 = if ((locals.var_flg_noqi == 0.0) && (locals.var_vgvt > 1e-12)) { 1.0 } else { 0.0 };
        locals.var_guard1521 = assign64420_e99411;
        locals.var_guard1521_rv = 0.0;

        let (assign64430_e99424, assign64430_e99424_d_n0, assign64430_e99424_d_n2, assign64430_e99424_d_n4, assign64430_e99424_d_n5, assign64430_e99424_d_n6, assign64430_e99424_d_n7, assign64430_e99424_d_n8, assign64430_e99424_d_n9, assign64430_e99424_d_n10, assign64430_e99424_d_n11, assign64430_e99424_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1521 != 0.0)) {
        let assign64430_e99418: f64 = (locals.var_fac1 * locals.var_beta);
        let assign64430_e99421: f64 = (2.0 * locals.var_xi0p12);
        let assign64430_e99422: f64 = (assign64430_e99418 / assign64430_e99421);
        (assign64430_e99422, (((((locals.var_fac1_dn0 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn0)) * assign64430_e99421) - (assign64430_e99418 * (2.0 * locals.var_xi0p12_dn0))) / (assign64430_e99421 * assign64430_e99421)), (((((locals.var_fac1_dn2 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn2)) * assign64430_e99421) - (assign64430_e99418 * (2.0 * locals.var_xi0p12_dn2))) / (assign64430_e99421 * assign64430_e99421)), (((((locals.var_fac1_dn4 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn4)) * assign64430_e99421) - (assign64430_e99418 * (2.0 * locals.var_xi0p12_dn4))) / (assign64430_e99421 * assign64430_e99421)), (((((locals.var_fac1_dn5 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn5)) * assign64430_e99421) - (assign64430_e99418 * (2.0 * locals.var_xi0p12_dn5))) / (assign64430_e99421 * assign64430_e99421)), (((((locals.var_fac1_dn6 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn6)) * assign64430_e99421) - (assign64430_e99418 * (2.0 * locals.var_xi0p12_dn6))) / (assign64430_e99421 * assign64430_e99421)), (((((locals.var_fac1_dn7 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn7)) * assign64430_e99421) - (assign64430_e99418 * (2.0 * locals.var_xi0p12_dn7))) / (assign64430_e99421 * assign64430_e99421)), (((((locals.var_fac1_dn8 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn8)) * assign64430_e99421) - (assign64430_e99418 * (2.0 * locals.var_xi0p12_dn8))) / (assign64430_e99421 * assign64430_e99421)), (((((locals.var_fac1_dn9 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn9)) * assign64430_e99421) - (assign64430_e99418 * (2.0 * locals.var_xi0p12_dn9))) / (assign64430_e99421 * assign64430_e99421)), (((((locals.var_fac1_dn10 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn10)) * assign64430_e99421) - (assign64430_e99418 * (2.0 * locals.var_xi0p12_dn10))) / (assign64430_e99421 * assign64430_e99421)), (((((locals.var_fac1_dn11 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn11)) * assign64430_e99421) - (assign64430_e99418 * (2.0 * locals.var_xi0p12_dn11))) / (assign64430_e99421 * assign64430_e99421)), (((((locals.var_fac1_dn14 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn14)) * assign64430_e99421) - (assign64430_e99418 * (2.0 * locals.var_xi0p12_dn14))) / (assign64430_e99421 * assign64430_e99421)),)
    } else {
        (locals.var_delta, locals.var_delta_dn0, locals.var_delta_dn2, locals.var_delta_dn4, locals.var_delta_dn5, locals.var_delta_dn6, locals.var_delta_dn7, locals.var_delta_dn8, locals.var_delta_dn9, locals.var_delta_dn10, locals.var_delta_dn11, locals.var_delta_dn14,)
    }
};
        locals.var_delta = assign64430_e99424;
        locals.var_delta_dn0 = assign64430_e99424_d_n0;
        locals.var_delta_dn2 = assign64430_e99424_d_n2;
        locals.var_delta_dn4 = assign64430_e99424_d_n4;
        locals.var_delta_dn5 = assign64430_e99424_d_n5;
        locals.var_delta_dn6 = assign64430_e99424_d_n6;
        locals.var_delta_dn7 = assign64430_e99424_d_n7;
        locals.var_delta_dn8 = assign64430_e99424_d_n8;
        locals.var_delta_dn9 = assign64430_e99424_d_n9;
        locals.var_delta_dn10 = assign64430_e99424_d_n10;
        locals.var_delta_dn11 = assign64430_e99424_d_n11;
        locals.var_delta_dn14 = assign64430_e99424_d_n14;
        locals.var_delta_rv = 0.0;

        let (assign64440_e99437, assign64440_e99437_d_n0, assign64440_e99437_d_n2, assign64440_e99437_d_n4, assign64440_e99437_d_n5, assign64440_e99437_d_n6, assign64440_e99437_d_n7, assign64440_e99437_d_n8, assign64440_e99437_d_n9, assign64440_e99437_d_n10, assign64440_e99437_d_n11, assign64440_e99437_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1521 != 0.0)) {
        let assign64440_e99432: f64 = (1.0 + locals.var_delta);
        let assign64440_e99433: f64 = (locals.var_vgvt / assign64440_e99432);
        let assign64440_e99435: f64 = (assign64440_e99433 + locals.var_ps0);
        (assign64440_e99435, ((((locals.var_vgvt_dn0 * assign64440_e99432) - (locals.var_vgvt * locals.var_delta_dn0)) / (assign64440_e99432 * assign64440_e99432)) + locals.var_ps0_dn0), ((((locals.var_vgvt_dn2 * assign64440_e99432) - (locals.var_vgvt * locals.var_delta_dn2)) / (assign64440_e99432 * assign64440_e99432)) + locals.var_ps0_dn2), ((((locals.var_vgvt_dn4 * assign64440_e99432) - (locals.var_vgvt * locals.var_delta_dn4)) / (assign64440_e99432 * assign64440_e99432)) + locals.var_ps0_dn4), ((((locals.var_vgvt_dn5 * assign64440_e99432) - (locals.var_vgvt * locals.var_delta_dn5)) / (assign64440_e99432 * assign64440_e99432)) + locals.var_ps0_dn5), ((((locals.var_vgvt_dn6 * assign64440_e99432) - (locals.var_vgvt * locals.var_delta_dn6)) / (assign64440_e99432 * assign64440_e99432)) + locals.var_ps0_dn6), ((((locals.var_vgvt_dn7 * assign64440_e99432) - (locals.var_vgvt * locals.var_delta_dn7)) / (assign64440_e99432 * assign64440_e99432)) + locals.var_ps0_dn7), ((((locals.var_vgvt_dn8 * assign64440_e99432) - (locals.var_vgvt * locals.var_delta_dn8)) / (assign64440_e99432 * assign64440_e99432)) + locals.var_ps0_dn8), ((((locals.var_vgvt_dn9 * assign64440_e99432) - (locals.var_vgvt * locals.var_delta_dn9)) / (assign64440_e99432 * assign64440_e99432)) + locals.var_ps0_dn9), ((((locals.var_vgvt_dn10 * assign64440_e99432) - (locals.var_vgvt * locals.var_delta_dn10)) / (assign64440_e99432 * assign64440_e99432)) + locals.var_ps0_dn10), ((((locals.var_vgvt_dn11 * assign64440_e99432) - (locals.var_vgvt * locals.var_delta_dn11)) / (assign64440_e99432 * assign64440_e99432)) + locals.var_ps0_dn11), ((((locals.var_vgvt_dn14 * assign64440_e99432) - (locals.var_vgvt * locals.var_delta_dn14)) / (assign64440_e99432 * assign64440_e99432)) + locals.var_ps0_dn14),)
    } else {
        (locals.var_pslsat, locals.var_pslsat_dn0, locals.var_pslsat_dn2, locals.var_pslsat_dn4, locals.var_pslsat_dn5, locals.var_pslsat_dn6, locals.var_pslsat_dn7, locals.var_pslsat_dn8, locals.var_pslsat_dn9, locals.var_pslsat_dn10, locals.var_pslsat_dn11, locals.var_pslsat_dn14,)
    }
};
        locals.var_pslsat = assign64440_e99437;
        locals.var_pslsat_dn0 = assign64440_e99437_d_n0;
        locals.var_pslsat_dn2 = assign64440_e99437_d_n2;
        locals.var_pslsat_dn4 = assign64440_e99437_d_n4;
        locals.var_pslsat_dn5 = assign64440_e99437_d_n5;
        locals.var_pslsat_dn6 = assign64440_e99437_d_n6;
        locals.var_pslsat_dn7 = assign64440_e99437_d_n7;
        locals.var_pslsat_dn8 = assign64440_e99437_d_n8;
        locals.var_pslsat_dn9 = assign64440_e99437_d_n9;
        locals.var_pslsat_dn10 = assign64440_e99437_d_n10;
        locals.var_pslsat_dn11 = assign64440_e99437_d_n11;
        locals.var_pslsat_dn14 = assign64440_e99437_d_n14;
        locals.var_pslsat_rv = 0.0;

        let (assign64450_e99445, assign64450_e99445_d_n0, assign64450_e99445_d_n2, assign64450_e99445_d_n4, assign64450_e99445_d_n5, assign64450_e99445_d_n6, assign64450_e99445_d_n7, assign64450_e99445_d_n8, assign64450_e99445_d_n9, assign64450_e99445_d_n10, assign64450_e99445_d_n11, assign64450_e99445_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1521 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pslsat, locals.var_pslsat_dn0, locals.var_pslsat_dn2, locals.var_pslsat_dn4, locals.var_pslsat_dn5, locals.var_pslsat_dn6, locals.var_pslsat_dn7, locals.var_pslsat_dn8, locals.var_pslsat_dn9, locals.var_pslsat_dn10, locals.var_pslsat_dn11, locals.var_pslsat_dn14,)
    }
};
        locals.var_pslsat = assign64450_e99445;
        locals.var_pslsat_dn0 = assign64450_e99445_d_n0;
        locals.var_pslsat_dn2 = assign64450_e99445_d_n2;
        locals.var_pslsat_dn4 = assign64450_e99445_d_n4;
        locals.var_pslsat_dn5 = assign64450_e99445_d_n5;
        locals.var_pslsat_dn6 = assign64450_e99445_d_n6;
        locals.var_pslsat_dn7 = assign64450_e99445_d_n7;
        locals.var_pslsat_dn8 = assign64450_e99445_d_n8;
        locals.var_pslsat_dn9 = assign64450_e99445_d_n9;
        locals.var_pslsat_dn10 = assign64450_e99445_d_n10;
        locals.var_pslsat_dn11 = assign64450_e99445_d_n11;
        locals.var_pslsat_dn14 = assign64450_e99445_d_n14;
        locals.var_pslsat_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_241(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign64490_e99467, assign64490_e99467_d_n0, assign64490_e99467_d_n2, assign64490_e99467_d_n4, assign64490_e99467_d_n5, assign64490_e99467_d_n6, assign64490_e99467_d_n7, assign64490_e99467_d_n8, assign64490_e99467_d_n9, assign64490_e99467_d_n10, assign64490_e99467_d_n11, assign64490_e99467_d_n14,) = {
    if (locals.var_guard447 == 0.0) {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn14,)
    } else {
        (locals.var_idsorg, locals.var_idsorg_dn0, locals.var_idsorg_dn2, locals.var_idsorg_dn4, locals.var_idsorg_dn5, locals.var_idsorg_dn6, locals.var_idsorg_dn7, locals.var_idsorg_dn8, locals.var_idsorg_dn9, locals.var_idsorg_dn10, locals.var_idsorg_dn11, locals.var_idsorg_dn14,)
    }
};
        locals.var_idsorg = assign64490_e99467;
        locals.var_idsorg_dn0 = assign64490_e99467_d_n0;
        locals.var_idsorg_dn2 = assign64490_e99467_d_n2;
        locals.var_idsorg_dn4 = assign64490_e99467_d_n4;
        locals.var_idsorg_dn5 = assign64490_e99467_d_n5;
        locals.var_idsorg_dn6 = assign64490_e99467_d_n6;
        locals.var_idsorg_dn7 = assign64490_e99467_d_n7;
        locals.var_idsorg_dn8 = assign64490_e99467_d_n8;
        locals.var_idsorg_dn9 = assign64490_e99467_d_n9;
        locals.var_idsorg_dn10 = assign64490_e99467_d_n10;
        locals.var_idsorg_dn11 = assign64490_e99467_d_n11;
        locals.var_idsorg_dn14 = assign64490_e99467_d_n14;
        locals.var_idsorg_rv = 0.0;

        let (assign64500_e99472, assign64500_e99472_d_n0, assign64500_e99472_d_n2, assign64500_e99472_d_n4, assign64500_e99472_d_n5, assign64500_e99472_d_n6, assign64500_e99472_d_n7, assign64500_e99472_d_n8, assign64500_e99472_d_n9, assign64500_e99472_d_n10, assign64500_e99472_d_n11, assign64500_e99472_d_n14,) = {
    if (locals.var_guard447 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_idspt1, locals.var_idspt1_dn0, locals.var_idspt1_dn2, locals.var_idspt1_dn4, locals.var_idspt1_dn5, locals.var_idspt1_dn6, locals.var_idspt1_dn7, locals.var_idspt1_dn8, locals.var_idspt1_dn9, locals.var_idspt1_dn10, locals.var_idspt1_dn11, locals.var_idspt1_dn14,)
    }
};
        locals.var_idspt1 = assign64500_e99472;
        locals.var_idspt1_dn0 = assign64500_e99472_d_n0;
        locals.var_idspt1_dn2 = assign64500_e99472_d_n2;
        locals.var_idspt1_dn4 = assign64500_e99472_d_n4;
        locals.var_idspt1_dn5 = assign64500_e99472_d_n5;
        locals.var_idspt1_dn6 = assign64500_e99472_d_n6;
        locals.var_idspt1_dn7 = assign64500_e99472_d_n7;
        locals.var_idspt1_dn8 = assign64500_e99472_d_n8;
        locals.var_idspt1_dn9 = assign64500_e99472_d_n9;
        locals.var_idspt1_dn10 = assign64500_e99472_d_n10;
        locals.var_idspt1_dn11 = assign64500_e99472_d_n11;
        locals.var_idspt1_dn14 = assign64500_e99472_d_n14;
        locals.var_idspt1_rv = 0.0;

        let assign64510_e99479: f64 = if ((p.p450 > 0.0) && (p.p454 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1523 = assign64510_e99479;
        locals.var_guard1523_rv = 0.0;

        let (assign64520_e99486,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) {
        (1e-5,)
    } else {
        (locals.var_t_sub,)
    }
};
        locals.var_t_sub = assign64520_e99486;
        locals.var_t_sub_rv = 0.0;

        let (assign64530_e99501, assign64530_e99501_d_n0, assign64530_e99501_d_n2, assign64530_e99501_d_n4, assign64530_e99501_d_n5, assign64530_e99501_d_n6, assign64530_e99501_d_n7, assign64530_e99501_d_n8, assign64530_e99501_d_n9, assign64530_e99501_d_n10, assign64530_e99501_d_n11, assign64530_e99501_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) {
        let assign64530_e99493: f64 = (locals.var_vgs - locals.var_vfb);
        let assign64530_e99495: f64 = (assign64530_e99493 + locals.var_dvth);
        let assign64530_e99497: f64 = (assign64530_e99495 - locals.var_dppg);
        let assign64530_e99499: f64 = (assign64530_e99497 - p.p455);
        (assign64530_e99499, (locals.var_dvth_dn0 - locals.var_dppg_dn0), (locals.var_dvth_dn2 - locals.var_dppg_dn2), (locals.var_dvth_dn4 - locals.var_dppg_dn4), (locals.var_dvth_dn5 - locals.var_dppg_dn5), ((locals.var_vgs_dn6 + locals.var_dvth_dn6) - locals.var_dppg_dn6), ((locals.var_vgs_dn7 + locals.var_dvth_dn7) - locals.var_dppg_dn7), ((locals.var_vgs_dn8 + locals.var_dvth_dn8) - locals.var_dppg_dn8), (locals.var_dvth_dn9 - locals.var_dppg_dn9), (locals.var_dvth_dn10 - locals.var_dppg_dn10), (locals.var_dvth_dn11 - locals.var_dppg_dn11), (locals.var_dvth_dn14 - locals.var_dppg_dn14),)
    } else {
        (locals.var_vgp__blk1529, locals.var_vgp__blk1529_dn0, locals.var_vgp__blk1529_dn2, locals.var_vgp__blk1529_dn4, locals.var_vgp__blk1529_dn5, locals.var_vgp__blk1529_dn6, locals.var_vgp__blk1529_dn7, locals.var_vgp__blk1529_dn8, locals.var_vgp__blk1529_dn9, locals.var_vgp__blk1529_dn10, locals.var_vgp__blk1529_dn11, locals.var_vgp__blk1529_dn14,)
    }
};
        locals.var_vgp__blk1529 = assign64530_e99501;
        locals.var_vgp__blk1529_dn0 = assign64530_e99501_d_n0;
        locals.var_vgp__blk1529_dn2 = assign64530_e99501_d_n2;
        locals.var_vgp__blk1529_dn4 = assign64530_e99501_d_n4;
        locals.var_vgp__blk1529_dn5 = assign64530_e99501_d_n5;
        locals.var_vgp__blk1529_dn6 = assign64530_e99501_d_n6;
        locals.var_vgp__blk1529_dn7 = assign64530_e99501_d_n7;
        locals.var_vgp__blk1529_dn8 = assign64530_e99501_d_n8;
        locals.var_vgp__blk1529_dn9 = assign64530_e99501_d_n9;
        locals.var_vgp__blk1529_dn10 = assign64530_e99501_d_n10;
        locals.var_vgp__blk1529_dn11 = assign64530_e99501_d_n11;
        locals.var_vgp__blk1529_dn14 = assign64530_e99501_d_n14;
        locals.var_vgp__blk1529_rv = 0.0;

        let (assign64540_e99510, assign64540_e99510_d_n0, assign64540_e99510_d_n2, assign64540_e99510_d_n4, assign64540_e99510_d_n5, assign64540_e99510_d_n6, assign64540_e99510_d_n7, assign64540_e99510_d_n8, assign64540_e99510_d_n9, assign64540_e99510_d_n10, assign64540_e99510_d_n11, assign64540_e99510_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) {
        let assign64540_e99508: f64 = (locals.var_vth + p.p455);
        (assign64540_e99508, locals.var_vth_dn0, locals.var_vth_dn2, locals.var_vth_dn4, locals.var_vth_dn5, locals.var_vth_dn6, locals.var_vth_dn7, locals.var_vth_dn8, locals.var_vth_dn9, locals.var_vth_dn10, locals.var_vth_dn11, locals.var_vth_dn14,)
    } else {
        (locals.var_wk_vth, locals.var_wk_vth_dn0, locals.var_wk_vth_dn2, locals.var_wk_vth_dn4, locals.var_wk_vth_dn5, locals.var_wk_vth_dn6, locals.var_wk_vth_dn7, locals.var_wk_vth_dn8, locals.var_wk_vth_dn9, locals.var_wk_vth_dn10, locals.var_wk_vth_dn11, locals.var_wk_vth_dn14,)
    }
};
        locals.var_wk_vth = assign64540_e99510;
        locals.var_wk_vth_dn0 = assign64540_e99510_d_n0;
        locals.var_wk_vth_dn2 = assign64540_e99510_d_n2;
        locals.var_wk_vth_dn4 = assign64540_e99510_d_n4;
        locals.var_wk_vth_dn5 = assign64540_e99510_d_n5;
        locals.var_wk_vth_dn6 = assign64540_e99510_d_n6;
        locals.var_wk_vth_dn7 = assign64540_e99510_d_n7;
        locals.var_wk_vth_dn8 = assign64540_e99510_d_n8;
        locals.var_wk_vth_dn9 = assign64540_e99510_d_n9;
        locals.var_wk_vth_dn10 = assign64540_e99510_d_n10;
        locals.var_wk_vth_dn11 = assign64540_e99510_d_n11;
        locals.var_wk_vth_dn14 = assign64540_e99510_d_n14;
        locals.var_wk_vth_rv = 0.0;

        let (assign64550_e99530, assign64550_e99530_d_n0, assign64550_e99530_d_n2, assign64550_e99530_d_n4, assign64550_e99530_d_n5, assign64550_e99530_d_n6, assign64550_e99530_d_n7, assign64550_e99530_d_n8, assign64550_e99530_d_n9, assign64550_e99530_d_n10, assign64550_e99530_d_n11, assign64550_e99530_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) {
        let assign64550_e99517: f64 = (locals.var_vbipn - locals.var_vbscl__blk439);
        let assign64550_e99520: f64 = (locals.var_vbipn - locals.var_vbscl__blk439);
        let assign64550_e99521: f64 = (assign64550_e99517 * assign64550_e99520);
        let assign64550_e99524: f64 = (4.0 * 0.01);
        let assign64550_e99526: f64 = (assign64550_e99524 * 0.01);
        let assign64550_e99527: f64 = (assign64550_e99521 + assign64550_e99526);
        let assign64550_e99528: f64 = (assign64550_e99527).sqrt();
        (assign64550_e99528, ((((locals.var_vbipn_dn0 - locals.var_vbscl__blk439_dn0) * assign64550_e99520) + (assign64550_e99517 * (locals.var_vbipn_dn0 - locals.var_vbscl__blk439_dn0))) / (2.0 * assign64550_e99528)), ((((locals.var_vbipn_dn2 - locals.var_vbscl__blk439_dn2) * assign64550_e99520) + (assign64550_e99517 * (locals.var_vbipn_dn2 - locals.var_vbscl__blk439_dn2))) / (2.0 * assign64550_e99528)), ((((locals.var_vbipn_dn4 - locals.var_vbscl__blk439_dn4) * assign64550_e99520) + (assign64550_e99517 * (locals.var_vbipn_dn4 - locals.var_vbscl__blk439_dn4))) / (2.0 * assign64550_e99528)), ((((locals.var_vbipn_dn5 - locals.var_vbscl__blk439_dn5) * assign64550_e99520) + (assign64550_e99517 * (locals.var_vbipn_dn5 - locals.var_vbscl__blk439_dn5))) / (2.0 * assign64550_e99528)), ((((locals.var_vbipn_dn6 - locals.var_vbscl__blk439_dn6) * assign64550_e99520) + (assign64550_e99517 * (locals.var_vbipn_dn6 - locals.var_vbscl__blk439_dn6))) / (2.0 * assign64550_e99528)), ((((locals.var_vbipn_dn7 - locals.var_vbscl__blk439_dn7) * assign64550_e99520) + (assign64550_e99517 * (locals.var_vbipn_dn7 - locals.var_vbscl__blk439_dn7))) / (2.0 * assign64550_e99528)), ((((locals.var_vbipn_dn8 - locals.var_vbscl__blk439_dn8) * assign64550_e99520) + (assign64550_e99517 * (locals.var_vbipn_dn8 - locals.var_vbscl__blk439_dn8))) / (2.0 * assign64550_e99528)), ((((locals.var_vbipn_dn9 - locals.var_vbscl__blk439_dn9) * assign64550_e99520) + (assign64550_e99517 * (locals.var_vbipn_dn9 - locals.var_vbscl__blk439_dn9))) / (2.0 * assign64550_e99528)), ((((locals.var_vbipn_dn10 - locals.var_vbscl__blk439_dn10) * assign64550_e99520) + (assign64550_e99517 * (locals.var_vbipn_dn10 - locals.var_vbscl__blk439_dn10))) / (2.0 * assign64550_e99528)), ((((locals.var_vbipn_dn11 - locals.var_vbscl__blk439_dn11) * assign64550_e99520) + (assign64550_e99517 * (locals.var_vbipn_dn11 - locals.var_vbscl__blk439_dn11))) / (2.0 * assign64550_e99528)), ((((locals.var_vbipn_dn14 - locals.var_vbscl__blk439_dn14) * assign64550_e99520) + (assign64550_e99517 * (locals.var_vbipn_dn14 - locals.var_vbscl__blk439_dn14))) / (2.0 * assign64550_e99528)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign64550_e99530;
        locals.var_tmf1_dn0 = assign64550_e99530_d_n0;
        locals.var_tmf1_dn2 = assign64550_e99530_d_n2;
        locals.var_tmf1_dn4 = assign64550_e99530_d_n4;
        locals.var_tmf1_dn5 = assign64550_e99530_d_n5;
        locals.var_tmf1_dn6 = assign64550_e99530_d_n6;
        locals.var_tmf1_dn7 = assign64550_e99530_d_n7;
        locals.var_tmf1_dn8 = assign64550_e99530_d_n8;
        locals.var_tmf1_dn9 = assign64550_e99530_d_n9;
        locals.var_tmf1_dn10 = assign64550_e99530_d_n10;
        locals.var_tmf1_dn11 = assign64550_e99530_d_n11;
        locals.var_tmf1_dn14 = assign64550_e99530_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign64560_e99543, assign64560_e99543_d_n0, assign64560_e99543_d_n2, assign64560_e99543_d_n4, assign64560_e99543_d_n5, assign64560_e99543_d_n6, assign64560_e99543_d_n7, assign64560_e99543_d_n8, assign64560_e99543_d_n9, assign64560_e99543_d_n10, assign64560_e99543_d_n11, assign64560_e99543_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) {
        let assign64560_e99538: f64 = (locals.var_vbipn - locals.var_vbscl__blk439);
        let assign64560_e99540: f64 = (assign64560_e99538 + locals.var_tmf1);
        let assign64560_e99541: f64 = (0.5 * assign64560_e99540);
        (assign64560_e99541, (0.5 * ((locals.var_vbipn_dn0 - locals.var_vbscl__blk439_dn0) + locals.var_tmf1_dn0)), (0.5 * ((locals.var_vbipn_dn2 - locals.var_vbscl__blk439_dn2) + locals.var_tmf1_dn2)), (0.5 * ((locals.var_vbipn_dn4 - locals.var_vbscl__blk439_dn4) + locals.var_tmf1_dn4)), (0.5 * ((locals.var_vbipn_dn5 - locals.var_vbscl__blk439_dn5) + locals.var_tmf1_dn5)), (0.5 * ((locals.var_vbipn_dn6 - locals.var_vbscl__blk439_dn6) + locals.var_tmf1_dn6)), (0.5 * ((locals.var_vbipn_dn7 - locals.var_vbscl__blk439_dn7) + locals.var_tmf1_dn7)), (0.5 * ((locals.var_vbipn_dn8 - locals.var_vbscl__blk439_dn8) + locals.var_tmf1_dn8)), (0.5 * ((locals.var_vbipn_dn9 - locals.var_vbscl__blk439_dn9) + locals.var_tmf1_dn9)), (0.5 * ((locals.var_vbipn_dn10 - locals.var_vbscl__blk439_dn10) + locals.var_tmf1_dn10)), (0.5 * ((locals.var_vbipn_dn11 - locals.var_vbscl__blk439_dn11) + locals.var_tmf1_dn11)), (0.5 * ((locals.var_vbipn_dn14 - locals.var_vbscl__blk439_dn14) + locals.var_tmf1_dn14)),)
    } else {
        (locals.var_vpositive, locals.var_vpositive_dn0, locals.var_vpositive_dn2, locals.var_vpositive_dn4, locals.var_vpositive_dn5, locals.var_vpositive_dn6, locals.var_vpositive_dn7, locals.var_vpositive_dn8, locals.var_vpositive_dn9, locals.var_vpositive_dn10, locals.var_vpositive_dn11, locals.var_vpositive_dn14,)
    }
};
        locals.var_vpositive = assign64560_e99543;
        locals.var_vpositive_dn0 = assign64560_e99543_d_n0;
        locals.var_vpositive_dn2 = assign64560_e99543_d_n2;
        locals.var_vpositive_dn4 = assign64560_e99543_d_n4;
        locals.var_vpositive_dn5 = assign64560_e99543_d_n5;
        locals.var_vpositive_dn6 = assign64560_e99543_d_n6;
        locals.var_vpositive_dn7 = assign64560_e99543_d_n7;
        locals.var_vpositive_dn8 = assign64560_e99543_d_n8;
        locals.var_vpositive_dn9 = assign64560_e99543_d_n9;
        locals.var_vpositive_dn10 = assign64560_e99543_d_n10;
        locals.var_vpositive_dn11 = assign64560_e99543_d_n11;
        locals.var_vpositive_dn14 = assign64560_e99543_d_n14;
        locals.var_vpositive_rv = 0.0;

        let (assign64570_e99565, assign64570_e99565_d_n0, assign64570_e99565_d_n2, assign64570_e99565_d_n4, assign64570_e99565_d_n5, assign64570_e99565_d_n6, assign64570_e99565_d_n7, assign64570_e99565_d_n8, assign64570_e99565_d_n9, assign64570_e99565_d_n10, assign64570_e99565_d_n11, assign64570_e99565_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) {
        let assign64570_e99550: f64 = (2.0 * 1.6021918e-19);
        let assign64570_e99552: f64 = (assign64570_e99550 * locals.var_vpositive);
        let assign64570_e99554: f64 = (assign64570_e99552 / 1.034943e-10);
        let assign64570_e99556: f64 = (assign64570_e99554 * locals.var_nsub);
        let assign64570_e99558: f64 = (assign64570_e99556 * locals.var_uc_njunc);
        let assign64570_e99561: f64 = (locals.var_nsub + locals.var_uc_njunc);
        let assign64570_e99562: f64 = (assign64570_e99558 / assign64570_e99561);
        let assign64570_e99563: f64 = (assign64570_e99562).sqrt();
        (assign64570_e99563, (((((((((assign64570_e99550 * locals.var_vpositive_dn0) / 1.034943e-10) * locals.var_nsub) + (assign64570_e99554 * locals.var_nsub_dn0)) * locals.var_uc_njunc) * assign64570_e99561) - (assign64570_e99558 * locals.var_nsub_dn0)) / (assign64570_e99561 * assign64570_e99561)) / (2.0 * assign64570_e99563)), (((((((((assign64570_e99550 * locals.var_vpositive_dn2) / 1.034943e-10) * locals.var_nsub) + (assign64570_e99554 * locals.var_nsub_dn2)) * locals.var_uc_njunc) * assign64570_e99561) - (assign64570_e99558 * locals.var_nsub_dn2)) / (assign64570_e99561 * assign64570_e99561)) / (2.0 * assign64570_e99563)), (((((((((assign64570_e99550 * locals.var_vpositive_dn4) / 1.034943e-10) * locals.var_nsub) + (assign64570_e99554 * locals.var_nsub_dn4)) * locals.var_uc_njunc) * assign64570_e99561) - (assign64570_e99558 * locals.var_nsub_dn4)) / (assign64570_e99561 * assign64570_e99561)) / (2.0 * assign64570_e99563)), (((((((((assign64570_e99550 * locals.var_vpositive_dn5) / 1.034943e-10) * locals.var_nsub) + (assign64570_e99554 * locals.var_nsub_dn5)) * locals.var_uc_njunc) * assign64570_e99561) - (assign64570_e99558 * locals.var_nsub_dn5)) / (assign64570_e99561 * assign64570_e99561)) / (2.0 * assign64570_e99563)), (((((((((assign64570_e99550 * locals.var_vpositive_dn6) / 1.034943e-10) * locals.var_nsub) + (assign64570_e99554 * locals.var_nsub_dn6)) * locals.var_uc_njunc) * assign64570_e99561) - (assign64570_e99558 * locals.var_nsub_dn6)) / (assign64570_e99561 * assign64570_e99561)) / (2.0 * assign64570_e99563)), (((((((((assign64570_e99550 * locals.var_vpositive_dn7) / 1.034943e-10) * locals.var_nsub) + (assign64570_e99554 * locals.var_nsub_dn7)) * locals.var_uc_njunc) * assign64570_e99561) - (assign64570_e99558 * locals.var_nsub_dn7)) / (assign64570_e99561 * assign64570_e99561)) / (2.0 * assign64570_e99563)), (((((((((assign64570_e99550 * locals.var_vpositive_dn8) / 1.034943e-10) * locals.var_nsub) + (assign64570_e99554 * locals.var_nsub_dn8)) * locals.var_uc_njunc) * assign64570_e99561) - (assign64570_e99558 * locals.var_nsub_dn8)) / (assign64570_e99561 * assign64570_e99561)) / (2.0 * assign64570_e99563)), (((((((((assign64570_e99550 * locals.var_vpositive_dn9) / 1.034943e-10) * locals.var_nsub) + (assign64570_e99554 * locals.var_nsub_dn9)) * locals.var_uc_njunc) * assign64570_e99561) - (assign64570_e99558 * locals.var_nsub_dn9)) / (assign64570_e99561 * assign64570_e99561)) / (2.0 * assign64570_e99563)), (((((((((assign64570_e99550 * locals.var_vpositive_dn10) / 1.034943e-10) * locals.var_nsub) + (assign64570_e99554 * locals.var_nsub_dn10)) * locals.var_uc_njunc) * assign64570_e99561) - (assign64570_e99558 * locals.var_nsub_dn10)) / (assign64570_e99561 * assign64570_e99561)) / (2.0 * assign64570_e99563)), (((((((((assign64570_e99550 * locals.var_vpositive_dn11) / 1.034943e-10) * locals.var_nsub) + (assign64570_e99554 * locals.var_nsub_dn11)) * locals.var_uc_njunc) * assign64570_e99561) - (assign64570_e99558 * locals.var_nsub_dn11)) / (assign64570_e99561 * assign64570_e99561)) / (2.0 * assign64570_e99563)), (((((((((assign64570_e99550 * locals.var_vpositive_dn14) / 1.034943e-10) * locals.var_nsub) + (assign64570_e99554 * locals.var_nsub_dn14)) * locals.var_uc_njunc) * assign64570_e99561) - (assign64570_e99558 * locals.var_nsub_dn14)) / (assign64570_e99561 * assign64570_e99561)) / (2.0 * assign64570_e99563)),)
    } else {
        (locals.var_ec__blk1524, locals.var_ec__blk1524_dn0, locals.var_ec__blk1524_dn2, locals.var_ec__blk1524_dn4, locals.var_ec__blk1524_dn5, locals.var_ec__blk1524_dn6, locals.var_ec__blk1524_dn7, locals.var_ec__blk1524_dn8, locals.var_ec__blk1524_dn9, locals.var_ec__blk1524_dn10, locals.var_ec__blk1524_dn11, locals.var_ec__blk1524_dn14,)
    }
};
        locals.var_ec__blk1524 = assign64570_e99565;
        locals.var_ec__blk1524_dn0 = assign64570_e99565_d_n0;
        locals.var_ec__blk1524_dn2 = assign64570_e99565_d_n2;
        locals.var_ec__blk1524_dn4 = assign64570_e99565_d_n4;
        locals.var_ec__blk1524_dn5 = assign64570_e99565_d_n5;
        locals.var_ec__blk1524_dn6 = assign64570_e99565_d_n6;
        locals.var_ec__blk1524_dn7 = assign64570_e99565_d_n7;
        locals.var_ec__blk1524_dn8 = assign64570_e99565_d_n8;
        locals.var_ec__blk1524_dn9 = assign64570_e99565_d_n9;
        locals.var_ec__blk1524_dn10 = assign64570_e99565_d_n10;
        locals.var_ec__blk1524_dn11 = assign64570_e99565_d_n11;
        locals.var_ec__blk1524_dn14 = assign64570_e99565_d_n14;
        locals.var_ec__blk1524_rv = 0.0;

        let (assign64580_e99574, assign64580_e99574_d_n0, assign64580_e99574_d_n2, assign64580_e99574_d_n4, assign64580_e99574_d_n5, assign64580_e99574_d_n6, assign64580_e99574_d_n7, assign64580_e99574_d_n8, assign64580_e99574_d_n9, assign64580_e99574_d_n10, assign64580_e99574_d_n11, assign64580_e99574_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) {
        let assign64580_e99572: f64 = (locals.var_ec__blk1524 * locals.var_leff);
        (assign64580_e99572, (locals.var_ec__blk1524_dn0 * locals.var_leff), (locals.var_ec__blk1524_dn2 * locals.var_leff), (locals.var_ec__blk1524_dn4 * locals.var_leff), (locals.var_ec__blk1524_dn5 * locals.var_leff), (locals.var_ec__blk1524_dn6 * locals.var_leff), (locals.var_ec__blk1524_dn7 * locals.var_leff), (locals.var_ec__blk1524_dn8 * locals.var_leff), (locals.var_ec__blk1524_dn9 * locals.var_leff), (locals.var_ec__blk1524_dn10 * locals.var_leff), (locals.var_ec__blk1524_dn11 * locals.var_leff), (locals.var_ec__blk1524_dn14 * locals.var_leff),)
    } else {
        (locals.var_wk, locals.var_wk_dn0, locals.var_wk_dn2, locals.var_wk_dn4, locals.var_wk_dn5, locals.var_wk_dn6, locals.var_wk_dn7, locals.var_wk_dn8, locals.var_wk_dn9, locals.var_wk_dn10, locals.var_wk_dn11, locals.var_wk_dn14,)
    }
};
        locals.var_wk = assign64580_e99574;
        locals.var_wk_dn0 = assign64580_e99574_d_n0;
        locals.var_wk_dn2 = assign64580_e99574_d_n2;
        locals.var_wk_dn4 = assign64580_e99574_d_n4;
        locals.var_wk_dn5 = assign64580_e99574_d_n5;
        locals.var_wk_dn6 = assign64580_e99574_d_n6;
        locals.var_wk_dn7 = assign64580_e99574_d_n7;
        locals.var_wk_dn8 = assign64580_e99574_d_n8;
        locals.var_wk_dn9 = assign64580_e99574_d_n9;
        locals.var_wk_dn10 = assign64580_e99574_d_n10;
        locals.var_wk_dn11 = assign64580_e99574_d_n11;
        locals.var_wk_dn14 = assign64580_e99574_d_n14;
        locals.var_wk_rv = 0.0;

        let (assign64590_e99590, assign64590_e99590_d_n0, assign64590_e99590_d_n2, assign64590_e99590_d_n4, assign64590_e99590_d_n5, assign64590_e99590_d_n6, assign64590_e99590_d_n7, assign64590_e99590_d_n8, assign64590_e99590_d_n9, assign64590_e99590_d_n10, assign64590_e99590_d_n11, assign64590_e99590_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) {
        let assign64590_e99580: f64 = (-0.25);
        let assign64590_e99582: f64 = (assign64590_e99580 * locals.var_wk);
        let assign64590_e99584: f64 = (assign64590_e99582 * locals.var_wk);
        let assign64590_e99587: f64 = (locals.var_vds + locals.var_wk);
        let assign64590_e99588: f64 = (assign64590_e99584 / assign64590_e99587);
        (assign64590_e99588, ((((((assign64590_e99580 * locals.var_wk_dn0) * locals.var_wk) + (assign64590_e99582 * locals.var_wk_dn0)) * assign64590_e99587) - (assign64590_e99584 * (locals.var_vds_dn0 + locals.var_wk_dn0))) / (assign64590_e99587 * assign64590_e99587)), ((((((assign64590_e99580 * locals.var_wk_dn2) * locals.var_wk) + (assign64590_e99582 * locals.var_wk_dn2)) * assign64590_e99587) - (assign64590_e99584 * (locals.var_vds_dn2 + locals.var_wk_dn2))) / (assign64590_e99587 * assign64590_e99587)), ((((((assign64590_e99580 * locals.var_wk_dn4) * locals.var_wk) + (assign64590_e99582 * locals.var_wk_dn4)) * assign64590_e99587) - (assign64590_e99584 * (locals.var_vds_dn4 + locals.var_wk_dn4))) / (assign64590_e99587 * assign64590_e99587)), ((((((assign64590_e99580 * locals.var_wk_dn5) * locals.var_wk) + (assign64590_e99582 * locals.var_wk_dn5)) * assign64590_e99587) - (assign64590_e99584 * (locals.var_vds_dn5 + locals.var_wk_dn5))) / (assign64590_e99587 * assign64590_e99587)), ((((((assign64590_e99580 * locals.var_wk_dn6) * locals.var_wk) + (assign64590_e99582 * locals.var_wk_dn6)) * assign64590_e99587) - (assign64590_e99584 * (locals.var_vds_dn6 + locals.var_wk_dn6))) / (assign64590_e99587 * assign64590_e99587)), ((((((assign64590_e99580 * locals.var_wk_dn7) * locals.var_wk) + (assign64590_e99582 * locals.var_wk_dn7)) * assign64590_e99587) - (assign64590_e99584 * (locals.var_vds_dn7 + locals.var_wk_dn7))) / (assign64590_e99587 * assign64590_e99587)), ((((((assign64590_e99580 * locals.var_wk_dn8) * locals.var_wk) + (assign64590_e99582 * locals.var_wk_dn8)) * assign64590_e99587) - (assign64590_e99584 * (locals.var_vds_dn8 + locals.var_wk_dn8))) / (assign64590_e99587 * assign64590_e99587)), ((((((assign64590_e99580 * locals.var_wk_dn9) * locals.var_wk) + (assign64590_e99582 * locals.var_wk_dn9)) * assign64590_e99587) - (assign64590_e99584 * (locals.var_vds_dn9 + locals.var_wk_dn9))) / (assign64590_e99587 * assign64590_e99587)), ((((((assign64590_e99580 * locals.var_wk_dn10) * locals.var_wk) + (assign64590_e99582 * locals.var_wk_dn10)) * assign64590_e99587) - (assign64590_e99584 * (locals.var_vds_dn10 + locals.var_wk_dn10))) / (assign64590_e99587 * assign64590_e99587)), ((((((assign64590_e99580 * locals.var_wk_dn11) * locals.var_wk) + (assign64590_e99582 * locals.var_wk_dn11)) * assign64590_e99587) - (assign64590_e99584 * (locals.var_vds_dn11 + locals.var_wk_dn11))) / (assign64590_e99587 * assign64590_e99587)), ((((((assign64590_e99580 * locals.var_wk_dn14) * locals.var_wk) + (assign64590_e99582 * locals.var_wk_dn14)) * assign64590_e99587) - (assign64590_e99584 * (locals.var_vds_dn14 + locals.var_wk_dn14))) / (assign64590_e99587 * assign64590_e99587)),)
    } else {
        (locals.var_dphi_vds, locals.var_dphi_vds_dn0, locals.var_dphi_vds_dn2, locals.var_dphi_vds_dn4, locals.var_dphi_vds_dn5, locals.var_dphi_vds_dn6, locals.var_dphi_vds_dn7, locals.var_dphi_vds_dn8, locals.var_dphi_vds_dn9, locals.var_dphi_vds_dn10, locals.var_dphi_vds_dn11, locals.var_dphi_vds_dn14,)
    }
};
        locals.var_dphi_vds = assign64590_e99590;
        locals.var_dphi_vds_dn0 = assign64590_e99590_d_n0;
        locals.var_dphi_vds_dn2 = assign64590_e99590_d_n2;
        locals.var_dphi_vds_dn4 = assign64590_e99590_d_n4;
        locals.var_dphi_vds_dn5 = assign64590_e99590_d_n5;
        locals.var_dphi_vds_dn6 = assign64590_e99590_d_n6;
        locals.var_dphi_vds_dn7 = assign64590_e99590_d_n7;
        locals.var_dphi_vds_dn8 = assign64590_e99590_d_n8;
        locals.var_dphi_vds_dn9 = assign64590_e99590_d_n9;
        locals.var_dphi_vds_dn10 = assign64590_e99590_d_n10;
        locals.var_dphi_vds_dn11 = assign64590_e99590_d_n11;
        locals.var_dphi_vds_dn14 = assign64590_e99590_d_n14;
        locals.var_dphi_vds_rv = 0.0;

        let assign64600_e99593: f64 = if p.p457 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1542 = assign64600_e99593;
        locals.var_guard1542_rv = 0.0;

        let (assign64610_e99602, assign64610_e99602_d_n0, assign64610_e99602_d_n2, assign64610_e99602_d_n4, assign64610_e99602_d_n5, assign64610_e99602_d_n6, assign64610_e99602_d_n7, assign64610_e99602_d_n8, assign64610_e99602_d_n9, assign64610_e99602_d_n10, assign64610_e99602_d_n11, assign64610_e99602_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 != 0.0)) {
        (p.p457, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps0__blk1527, locals.var_ps0__blk1527_dn0, locals.var_ps0__blk1527_dn2, locals.var_ps0__blk1527_dn4, locals.var_ps0__blk1527_dn5, locals.var_ps0__blk1527_dn6, locals.var_ps0__blk1527_dn7, locals.var_ps0__blk1527_dn8, locals.var_ps0__blk1527_dn9, locals.var_ps0__blk1527_dn10, locals.var_ps0__blk1527_dn11, locals.var_ps0__blk1527_dn14,)
    }
};
        locals.var_ps0__blk1527 = assign64610_e99602;
        locals.var_ps0__blk1527_dn0 = assign64610_e99602_d_n0;
        locals.var_ps0__blk1527_dn2 = assign64610_e99602_d_n2;
        locals.var_ps0__blk1527_dn4 = assign64610_e99602_d_n4;
        locals.var_ps0__blk1527_dn5 = assign64610_e99602_d_n5;
        locals.var_ps0__blk1527_dn6 = assign64610_e99602_d_n6;
        locals.var_ps0__blk1527_dn7 = assign64610_e99602_d_n7;
        locals.var_ps0__blk1527_dn8 = assign64610_e99602_d_n8;
        locals.var_ps0__blk1527_dn9 = assign64610_e99602_d_n9;
        locals.var_ps0__blk1527_dn10 = assign64610_e99602_d_n10;
        locals.var_ps0__blk1527_dn11 = assign64610_e99602_d_n11;
        locals.var_ps0__blk1527_dn14 = assign64610_e99602_d_n14;
        locals.var_ps0__blk1527_rv = 0.0;

        let (assign64620_e99612, assign64620_e99612_d_n0, assign64620_e99612_d_n2, assign64620_e99612_d_n4, assign64620_e99612_d_n5, assign64620_e99612_d_n6, assign64620_e99612_d_n7, assign64620_e99612_d_n8, assign64620_e99612_d_n9, assign64620_e99612_d_n10, assign64620_e99612_d_n11, assign64620_e99612_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) {
        (locals.var_dphi_vds, locals.var_dphi_vds_dn0, locals.var_dphi_vds_dn2, locals.var_dphi_vds_dn4, locals.var_dphi_vds_dn5, locals.var_dphi_vds_dn6, locals.var_dphi_vds_dn7, locals.var_dphi_vds_dn8, locals.var_dphi_vds_dn9, locals.var_dphi_vds_dn10, locals.var_dphi_vds_dn11, locals.var_dphi_vds_dn14,)
    } else {
        (locals.var_vbscl__blk1543, locals.var_vbscl__blk1543_dn0, locals.var_vbscl__blk1543_dn2, locals.var_vbscl__blk1543_dn4, locals.var_vbscl__blk1543_dn5, locals.var_vbscl__blk1543_dn6, locals.var_vbscl__blk1543_dn7, locals.var_vbscl__blk1543_dn8, locals.var_vbscl__blk1543_dn9, locals.var_vbscl__blk1543_dn10, locals.var_vbscl__blk1543_dn11, locals.var_vbscl__blk1543_dn14,)
    }
};
        locals.var_vbscl__blk1543 = assign64620_e99612;
        locals.var_vbscl__blk1543_dn0 = assign64620_e99612_d_n0;
        locals.var_vbscl__blk1543_dn2 = assign64620_e99612_d_n2;
        locals.var_vbscl__blk1543_dn4 = assign64620_e99612_d_n4;
        locals.var_vbscl__blk1543_dn5 = assign64620_e99612_d_n5;
        locals.var_vbscl__blk1543_dn6 = assign64620_e99612_d_n6;
        locals.var_vbscl__blk1543_dn7 = assign64620_e99612_d_n7;
        locals.var_vbscl__blk1543_dn8 = assign64620_e99612_d_n8;
        locals.var_vbscl__blk1543_dn9 = assign64620_e99612_d_n9;
        locals.var_vbscl__blk1543_dn10 = assign64620_e99612_d_n10;
        locals.var_vbscl__blk1543_dn11 = assign64620_e99612_d_n11;
        locals.var_vbscl__blk1543_dn14 = assign64620_e99612_d_n14;
        locals.var_vbscl__blk1543_rv = 0.0;

        let (assign64630_e99622, assign64630_e99622_d_n0, assign64630_e99622_d_n2, assign64630_e99622_d_n4, assign64630_e99622_d_n5, assign64630_e99622_d_n6, assign64630_e99622_d_n7, assign64630_e99622_d_n8, assign64630_e99622_d_n9, assign64630_e99622_d_n10, assign64630_e99622_d_n11, assign64630_e99622_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) {
        (locals.var_wk_vth, locals.var_wk_vth_dn0, locals.var_wk_vth_dn2, locals.var_wk_vth_dn4, locals.var_wk_vth_dn5, locals.var_wk_vth_dn6, locals.var_wk_vth_dn7, locals.var_wk_vth_dn8, locals.var_wk_vth_dn9, locals.var_wk_vth_dn10, locals.var_wk_vth_dn11, locals.var_wk_vth_dn14,)
    } else {
        (locals.var_vth__blk1544, locals.var_vth__blk1544_dn0, locals.var_vth__blk1544_dn2, locals.var_vth__blk1544_dn4, locals.var_vth__blk1544_dn5, locals.var_vth__blk1544_dn6, locals.var_vth__blk1544_dn7, locals.var_vth__blk1544_dn8, locals.var_vth__blk1544_dn9, locals.var_vth__blk1544_dn10, locals.var_vth__blk1544_dn11, locals.var_vth__blk1544_dn14,)
    }
};
        locals.var_vth__blk1544 = assign64630_e99622;
        locals.var_vth__blk1544_dn0 = assign64630_e99622_d_n0;
        locals.var_vth__blk1544_dn2 = assign64630_e99622_d_n2;
        locals.var_vth__blk1544_dn4 = assign64630_e99622_d_n4;
        locals.var_vth__blk1544_dn5 = assign64630_e99622_d_n5;
        locals.var_vth__blk1544_dn6 = assign64630_e99622_d_n6;
        locals.var_vth__blk1544_dn7 = assign64630_e99622_d_n7;
        locals.var_vth__blk1544_dn8 = assign64630_e99622_d_n8;
        locals.var_vth__blk1544_dn9 = assign64630_e99622_d_n9;
        locals.var_vth__blk1544_dn10 = assign64630_e99622_d_n10;
        locals.var_vth__blk1544_dn11 = assign64630_e99622_d_n11;
        locals.var_vth__blk1544_dn14 = assign64630_e99622_d_n14;
        locals.var_vth__blk1544_rv = 0.0;

        let (assign64640_e99646, assign64640_e99646_d_n0, assign64640_e99646_d_n2, assign64640_e99646_d_n4, assign64640_e99646_d_n5, assign64640_e99646_d_n6, assign64640_e99646_d_n7, assign64640_e99646_d_n8, assign64640_e99646_d_n9, assign64640_e99646_d_n10, assign64640_e99646_d_n11, assign64640_e99646_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) {
        let assign64640_e99635: f64 = (locals.var_vgp__blk1529 - locals.var_vbscl__blk1543);
        let assign64640_e99636: f64 = (locals.var_beta * assign64640_e99635);
        let assign64640_e99638: f64 = (assign64640_e99636 - 1.0);
        let assign64640_e99639: f64 = (4.0 * assign64640_e99638);
        let assign64640_e99642: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign64640_e99643: f64 = (assign64640_e99639 / assign64640_e99642);
        let assign64640_e99644: f64 = (1.0 + assign64640_e99643);
        (assign64640_e99644, ((((4.0 * ((locals.var_beta_dn0 * assign64640_e99635) + (locals.var_beta * (locals.var_vgp__blk1529_dn0 - locals.var_vbscl__blk1543_dn0)))) * assign64640_e99642) - (assign64640_e99639 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign64640_e99642 * assign64640_e99642)), ((((4.0 * ((locals.var_beta_dn2 * assign64640_e99635) + (locals.var_beta * (locals.var_vgp__blk1529_dn2 - locals.var_vbscl__blk1543_dn2)))) * assign64640_e99642) - (assign64640_e99639 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign64640_e99642 * assign64640_e99642)), ((((4.0 * ((locals.var_beta_dn4 * assign64640_e99635) + (locals.var_beta * (locals.var_vgp__blk1529_dn4 - locals.var_vbscl__blk1543_dn4)))) * assign64640_e99642) - (assign64640_e99639 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign64640_e99642 * assign64640_e99642)), ((((4.0 * ((locals.var_beta_dn5 * assign64640_e99635) + (locals.var_beta * (locals.var_vgp__blk1529_dn5 - locals.var_vbscl__blk1543_dn5)))) * assign64640_e99642) - (assign64640_e99639 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign64640_e99642 * assign64640_e99642)), ((((4.0 * ((locals.var_beta_dn6 * assign64640_e99635) + (locals.var_beta * (locals.var_vgp__blk1529_dn6 - locals.var_vbscl__blk1543_dn6)))) * assign64640_e99642) - (assign64640_e99639 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign64640_e99642 * assign64640_e99642)), ((((4.0 * ((locals.var_beta_dn7 * assign64640_e99635) + (locals.var_beta * (locals.var_vgp__blk1529_dn7 - locals.var_vbscl__blk1543_dn7)))) * assign64640_e99642) - (assign64640_e99639 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign64640_e99642 * assign64640_e99642)), ((((4.0 * ((locals.var_beta_dn8 * assign64640_e99635) + (locals.var_beta * (locals.var_vgp__blk1529_dn8 - locals.var_vbscl__blk1543_dn8)))) * assign64640_e99642) - (assign64640_e99639 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign64640_e99642 * assign64640_e99642)), ((((4.0 * ((locals.var_beta_dn9 * assign64640_e99635) + (locals.var_beta * (locals.var_vgp__blk1529_dn9 - locals.var_vbscl__blk1543_dn9)))) * assign64640_e99642) - (assign64640_e99639 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign64640_e99642 * assign64640_e99642)), ((((4.0 * ((locals.var_beta_dn10 * assign64640_e99635) + (locals.var_beta * (locals.var_vgp__blk1529_dn10 - locals.var_vbscl__blk1543_dn10)))) * assign64640_e99642) - (assign64640_e99639 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign64640_e99642 * assign64640_e99642)), ((((4.0 * ((locals.var_beta_dn11 * assign64640_e99635) + (locals.var_beta * (locals.var_vgp__blk1529_dn11 - locals.var_vbscl__blk1543_dn11)))) * assign64640_e99642) - (assign64640_e99639 * ((locals.var_fac1p2_dn11 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn11)))) / (assign64640_e99642 * assign64640_e99642)), ((((4.0 * ((locals.var_beta_dn14 * assign64640_e99635) + (locals.var_beta * (locals.var_vgp__blk1529_dn14 - locals.var_vbscl__blk1543_dn14)))) * assign64640_e99642) - (assign64640_e99639 * ((locals.var_fac1p2_dn14 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn14)))) / (assign64640_e99642 * assign64640_e99642)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign64640_e99646;
        locals.var_tx_dn0 = assign64640_e99646_d_n0;
        locals.var_tx_dn2 = assign64640_e99646_d_n2;
        locals.var_tx_dn4 = assign64640_e99646_d_n4;
        locals.var_tx_dn5 = assign64640_e99646_d_n5;
        locals.var_tx_dn6 = assign64640_e99646_d_n6;
        locals.var_tx_dn7 = assign64640_e99646_d_n7;
        locals.var_tx_dn8 = assign64640_e99646_d_n8;
        locals.var_tx_dn9 = assign64640_e99646_d_n9;
        locals.var_tx_dn10 = assign64640_e99646_d_n10;
        locals.var_tx_dn11 = assign64640_e99646_d_n11;
        locals.var_tx_dn14 = assign64640_e99646_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign64650_e99665, assign64650_e99665_d_n0, assign64650_e99665_d_n2, assign64650_e99665_d_n4, assign64650_e99665_d_n5, assign64650_e99665_d_n6, assign64650_e99665_d_n7, assign64650_e99665_d_n8, assign64650_e99665_d_n9, assign64650_e99665_d_n10, assign64650_e99665_d_n11, assign64650_e99665_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) {
        let assign64650_e99657: f64 = (10.0 * 2.220446049250313e-16);
        let (assign64650_e99663, assign64650_e99663_d_n0, assign64650_e99663_d_n2, assign64650_e99663_d_n4, assign64650_e99663_d_n5, assign64650_e99663_d_n6, assign64650_e99663_d_n7, assign64650_e99663_d_n8, assign64650_e99663_d_n9, assign64650_e99663_d_n10, assign64650_e99663_d_n11, assign64650_e99663_d_n14,) = {
            if (locals.var_tx >= assign64650_e99657) {
                (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
            } else {
                let assign64650_e99662: f64 = (10.0 * 2.220446049250313e-16);
                (assign64650_e99662, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign64650_e99663, assign64650_e99663_d_n0, assign64650_e99663_d_n2, assign64650_e99663_d_n4, assign64650_e99663_d_n5, assign64650_e99663_d_n6, assign64650_e99663_d_n7, assign64650_e99663_d_n8, assign64650_e99663_d_n9, assign64650_e99663_d_n10, assign64650_e99663_d_n11, assign64650_e99663_d_n14,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign64650_e99665;
        locals.var_tx_dn0 = assign64650_e99665_d_n0;
        locals.var_tx_dn2 = assign64650_e99665_d_n2;
        locals.var_tx_dn4 = assign64650_e99665_d_n4;
        locals.var_tx_dn5 = assign64650_e99665_d_n5;
        locals.var_tx_dn6 = assign64650_e99665_d_n6;
        locals.var_tx_dn7 = assign64650_e99665_d_n7;
        locals.var_tx_dn8 = assign64650_e99665_d_n8;
        locals.var_tx_dn9 = assign64650_e99665_d_n9;
        locals.var_tx_dn10 = assign64650_e99665_d_n10;
        locals.var_tx_dn11 = assign64650_e99665_d_n11;
        locals.var_tx_dn14 = assign64650_e99665_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign64660_e99686, assign64660_e99686_d_n0, assign64660_e99686_d_n2, assign64660_e99686_d_n4, assign64660_e99686_d_n5, assign64660_e99686_d_n6, assign64660_e99686_d_n7, assign64660_e99686_d_n8, assign64660_e99686_d_n9, assign64660_e99686_d_n10, assign64660_e99686_d_n11, assign64660_e99686_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) {
        let assign64660_e99676: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign64660_e99678: f64 = (assign64660_e99676 * 0.5);
        let assign64660_e99681: f64 = (locals.var_tx).sqrt();
        let assign64660_e99682: f64 = (1.0 - assign64660_e99681);
        let assign64660_e99683: f64 = (assign64660_e99678 * assign64660_e99682);
        let assign64660_e99684: f64 = (locals.var_vgp__blk1529 + assign64660_e99683);
        (assign64660_e99684, (locals.var_vgp__blk1529_dn0 + (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) * 0.5) * assign64660_e99682) + (assign64660_e99678 * (-(locals.var_tx_dn0 / (2.0 * assign64660_e99681)))))), (locals.var_vgp__blk1529_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) * 0.5) * assign64660_e99682) + (assign64660_e99678 * (-(locals.var_tx_dn2 / (2.0 * assign64660_e99681)))))), (locals.var_vgp__blk1529_dn4 + (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) * 0.5) * assign64660_e99682) + (assign64660_e99678 * (-(locals.var_tx_dn4 / (2.0 * assign64660_e99681)))))), (locals.var_vgp__blk1529_dn5 + (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) * 0.5) * assign64660_e99682) + (assign64660_e99678 * (-(locals.var_tx_dn5 / (2.0 * assign64660_e99681)))))), (locals.var_vgp__blk1529_dn6 + (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) * 0.5) * assign64660_e99682) + (assign64660_e99678 * (-(locals.var_tx_dn6 / (2.0 * assign64660_e99681)))))), (locals.var_vgp__blk1529_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) * 0.5) * assign64660_e99682) + (assign64660_e99678 * (-(locals.var_tx_dn7 / (2.0 * assign64660_e99681)))))), (locals.var_vgp__blk1529_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) * 0.5) * assign64660_e99682) + (assign64660_e99678 * (-(locals.var_tx_dn8 / (2.0 * assign64660_e99681)))))), (locals.var_vgp__blk1529_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) * 0.5) * assign64660_e99682) + (assign64660_e99678 * (-(locals.var_tx_dn9 / (2.0 * assign64660_e99681)))))), (locals.var_vgp__blk1529_dn10 + (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) * 0.5) * assign64660_e99682) + (assign64660_e99678 * (-(locals.var_tx_dn10 / (2.0 * assign64660_e99681)))))), (locals.var_vgp__blk1529_dn11 + (((((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) * 0.5) * assign64660_e99682) + (assign64660_e99678 * (-(locals.var_tx_dn11 / (2.0 * assign64660_e99681)))))), (locals.var_vgp__blk1529_dn14 + (((((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) * 0.5) * assign64660_e99682) + (assign64660_e99678 * (-(locals.var_tx_dn14 / (2.0 * assign64660_e99681)))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign64660_e99686;
        locals.var_ps0_inia_dn0 = assign64660_e99686_d_n0;
        locals.var_ps0_inia_dn2 = assign64660_e99686_d_n2;
        locals.var_ps0_inia_dn4 = assign64660_e99686_d_n4;
        locals.var_ps0_inia_dn5 = assign64660_e99686_d_n5;
        locals.var_ps0_inia_dn6 = assign64660_e99686_d_n6;
        locals.var_ps0_inia_dn7 = assign64660_e99686_d_n7;
        locals.var_ps0_inia_dn8 = assign64660_e99686_d_n8;
        locals.var_ps0_inia_dn9 = assign64660_e99686_d_n9;
        locals.var_ps0_inia_dn10 = assign64660_e99686_d_n10;
        locals.var_ps0_inia_dn11 = assign64660_e99686_d_n11;
        locals.var_ps0_inia_dn14 = assign64660_e99686_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let (assign64670_e99700, assign64670_e99700_d_n0, assign64670_e99700_d_n2, assign64670_e99700_d_n4, assign64670_e99700_d_n5, assign64670_e99700_d_n6, assign64670_e99700_d_n7, assign64670_e99700_d_n8, assign64670_e99700_d_n9, assign64670_e99700_d_n10, assign64670_e99700_d_n11, assign64670_e99700_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) {
        let assign64670_e99697: f64 = (locals.var_ps0_inia - locals.var_vbscl__blk1543);
        let assign64670_e99698: f64 = (locals.var_beta * assign64670_e99697);
        (assign64670_e99698, ((locals.var_beta_dn0 * assign64670_e99697) + (locals.var_beta * (locals.var_ps0_inia_dn0 - locals.var_vbscl__blk1543_dn0))), ((locals.var_beta_dn2 * assign64670_e99697) + (locals.var_beta * (locals.var_ps0_inia_dn2 - locals.var_vbscl__blk1543_dn2))), ((locals.var_beta_dn4 * assign64670_e99697) + (locals.var_beta * (locals.var_ps0_inia_dn4 - locals.var_vbscl__blk1543_dn4))), ((locals.var_beta_dn5 * assign64670_e99697) + (locals.var_beta * (locals.var_ps0_inia_dn5 - locals.var_vbscl__blk1543_dn5))), ((locals.var_beta_dn6 * assign64670_e99697) + (locals.var_beta * (locals.var_ps0_inia_dn6 - locals.var_vbscl__blk1543_dn6))), ((locals.var_beta_dn7 * assign64670_e99697) + (locals.var_beta * (locals.var_ps0_inia_dn7 - locals.var_vbscl__blk1543_dn7))), ((locals.var_beta_dn8 * assign64670_e99697) + (locals.var_beta * (locals.var_ps0_inia_dn8 - locals.var_vbscl__blk1543_dn8))), ((locals.var_beta_dn9 * assign64670_e99697) + (locals.var_beta * (locals.var_ps0_inia_dn9 - locals.var_vbscl__blk1543_dn9))), ((locals.var_beta_dn10 * assign64670_e99697) + (locals.var_beta * (locals.var_ps0_inia_dn10 - locals.var_vbscl__blk1543_dn10))), ((locals.var_beta_dn11 * assign64670_e99697) + (locals.var_beta * (locals.var_ps0_inia_dn11 - locals.var_vbscl__blk1543_dn11))), ((locals.var_beta_dn14 * assign64670_e99697) + (locals.var_beta * (locals.var_ps0_inia_dn14 - locals.var_vbscl__blk1543_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign64670_e99700;
        locals.var_chi_dn0 = assign64670_e99700_d_n0;
        locals.var_chi_dn2 = assign64670_e99700_d_n2;
        locals.var_chi_dn4 = assign64670_e99700_d_n4;
        locals.var_chi_dn5 = assign64670_e99700_d_n5;
        locals.var_chi_dn6 = assign64670_e99700_d_n6;
        locals.var_chi_dn7 = assign64670_e99700_d_n7;
        locals.var_chi_dn8 = assign64670_e99700_d_n8;
        locals.var_chi_dn9 = assign64670_e99700_d_n9;
        locals.var_chi_dn10 = assign64670_e99700_d_n10;
        locals.var_chi_dn11 = assign64670_e99700_d_n11;
        locals.var_chi_dn14 = assign64670_e99700_d_n14;
        locals.var_chi_rv = 0.0;

        let assign64680_e99703: f64 = if locals.var_chi < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1545 = assign64680_e99703;
        locals.var_guard1545_rv = 0.0;

        let (assign64690_e99719, assign64690_e99719_d_n0, assign64690_e99719_d_n2, assign64690_e99719_d_n4, assign64690_e99719_d_n5, assign64690_e99719_d_n6, assign64690_e99719_d_n7, assign64690_e99719_d_n8, assign64690_e99719_d_n9, assign64690_e99719_d_n10, assign64690_e99719_d_n11, assign64690_e99719_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1545 != 0.0)) {
        let assign64690_e99716: f64 = (locals.var_vgp__blk1529 - locals.var_vbscl__blk1543);
        let assign64690_e99717: f64 = (locals.var_beta * assign64690_e99716);
        (assign64690_e99717, ((locals.var_beta_dn0 * assign64690_e99716) + (locals.var_beta * (locals.var_vgp__blk1529_dn0 - locals.var_vbscl__blk1543_dn0))), ((locals.var_beta_dn2 * assign64690_e99716) + (locals.var_beta * (locals.var_vgp__blk1529_dn2 - locals.var_vbscl__blk1543_dn2))), ((locals.var_beta_dn4 * assign64690_e99716) + (locals.var_beta * (locals.var_vgp__blk1529_dn4 - locals.var_vbscl__blk1543_dn4))), ((locals.var_beta_dn5 * assign64690_e99716) + (locals.var_beta * (locals.var_vgp__blk1529_dn5 - locals.var_vbscl__blk1543_dn5))), ((locals.var_beta_dn6 * assign64690_e99716) + (locals.var_beta * (locals.var_vgp__blk1529_dn6 - locals.var_vbscl__blk1543_dn6))), ((locals.var_beta_dn7 * assign64690_e99716) + (locals.var_beta * (locals.var_vgp__blk1529_dn7 - locals.var_vbscl__blk1543_dn7))), ((locals.var_beta_dn8 * assign64690_e99716) + (locals.var_beta * (locals.var_vgp__blk1529_dn8 - locals.var_vbscl__blk1543_dn8))), ((locals.var_beta_dn9 * assign64690_e99716) + (locals.var_beta * (locals.var_vgp__blk1529_dn9 - locals.var_vbscl__blk1543_dn9))), ((locals.var_beta_dn10 * assign64690_e99716) + (locals.var_beta * (locals.var_vgp__blk1529_dn10 - locals.var_vbscl__blk1543_dn10))), ((locals.var_beta_dn11 * assign64690_e99716) + (locals.var_beta * (locals.var_vgp__blk1529_dn11 - locals.var_vbscl__blk1543_dn11))), ((locals.var_beta_dn14 * assign64690_e99716) + (locals.var_beta * (locals.var_vgp__blk1529_dn14 - locals.var_vbscl__blk1543_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign64690_e99719;
        locals.var_ty_dn0 = assign64690_e99719_d_n0;
        locals.var_ty_dn2 = assign64690_e99719_d_n2;
        locals.var_ty_dn4 = assign64690_e99719_d_n4;
        locals.var_ty_dn5 = assign64690_e99719_d_n5;
        locals.var_ty_dn6 = assign64690_e99719_d_n6;
        locals.var_ty_dn7 = assign64690_e99719_d_n7;
        locals.var_ty_dn8 = assign64690_e99719_d_n8;
        locals.var_ty_dn9 = assign64690_e99719_d_n9;
        locals.var_ty_dn10 = assign64690_e99719_d_n10;
        locals.var_ty_dn11 = assign64690_e99719_d_n11;
        locals.var_ty_dn14 = assign64690_e99719_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign64700_e99739, assign64700_e99739_d_n0, assign64700_e99739_d_n2, assign64700_e99739_d_n4, assign64700_e99739_d_n5, assign64700_e99739_d_n6, assign64700_e99739_d_n7, assign64700_e99739_d_n8, assign64700_e99739_d_n9, assign64700_e99739_d_n10, assign64700_e99739_d_n11, assign64700_e99739_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1545 != 0.0)) {
        let assign64700_e99732: f64 = (1.414213562373095 / 108.0);
        let assign64700_e99734: f64 = (assign64700_e99732 * locals.var_beta);
        let assign64700_e99736: f64 = (assign64700_e99734 * locals.var_fac1);
        let assign64700_e99737: f64 = (1.0 / assign64700_e99736);
        (assign64700_e99737, (-((((assign64700_e99732 * locals.var_beta_dn0) * locals.var_fac1) + (assign64700_e99734 * locals.var_fac1_dn0)) / (assign64700_e99736 * assign64700_e99736))), (-((((assign64700_e99732 * locals.var_beta_dn2) * locals.var_fac1) + (assign64700_e99734 * locals.var_fac1_dn2)) / (assign64700_e99736 * assign64700_e99736))), (-((((assign64700_e99732 * locals.var_beta_dn4) * locals.var_fac1) + (assign64700_e99734 * locals.var_fac1_dn4)) / (assign64700_e99736 * assign64700_e99736))), (-((((assign64700_e99732 * locals.var_beta_dn5) * locals.var_fac1) + (assign64700_e99734 * locals.var_fac1_dn5)) / (assign64700_e99736 * assign64700_e99736))), (-((((assign64700_e99732 * locals.var_beta_dn6) * locals.var_fac1) + (assign64700_e99734 * locals.var_fac1_dn6)) / (assign64700_e99736 * assign64700_e99736))), (-((((assign64700_e99732 * locals.var_beta_dn7) * locals.var_fac1) + (assign64700_e99734 * locals.var_fac1_dn7)) / (assign64700_e99736 * assign64700_e99736))), (-((((assign64700_e99732 * locals.var_beta_dn8) * locals.var_fac1) + (assign64700_e99734 * locals.var_fac1_dn8)) / (assign64700_e99736 * assign64700_e99736))), (-((((assign64700_e99732 * locals.var_beta_dn9) * locals.var_fac1) + (assign64700_e99734 * locals.var_fac1_dn9)) / (assign64700_e99736 * assign64700_e99736))), (-((((assign64700_e99732 * locals.var_beta_dn10) * locals.var_fac1) + (assign64700_e99734 * locals.var_fac1_dn10)) / (assign64700_e99736 * assign64700_e99736))), (-((((assign64700_e99732 * locals.var_beta_dn11) * locals.var_fac1) + (assign64700_e99734 * locals.var_fac1_dn11)) / (assign64700_e99736 * assign64700_e99736))), (-((((assign64700_e99732 * locals.var_beta_dn14) * locals.var_fac1) + (assign64700_e99734 * locals.var_fac1_dn14)) / (assign64700_e99736 * assign64700_e99736))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign64700_e99739;
        locals.var_t1_dn0 = assign64700_e99739_d_n0;
        locals.var_t1_dn2 = assign64700_e99739_d_n2;
        locals.var_t1_dn4 = assign64700_e99739_d_n4;
        locals.var_t1_dn5 = assign64700_e99739_d_n5;
        locals.var_t1_dn6 = assign64700_e99739_d_n6;
        locals.var_t1_dn7 = assign64700_e99739_d_n7;
        locals.var_t1_dn8 = assign64700_e99739_d_n8;
        locals.var_t1_dn9 = assign64700_e99739_d_n9;
        locals.var_t1_dn10 = assign64700_e99739_d_n10;
        locals.var_t1_dn11 = assign64700_e99739_d_n11;
        locals.var_t1_dn14 = assign64700_e99739_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign64710_e99755, assign64710_e99755_d_n0, assign64710_e99755_d_n2, assign64710_e99755_d_n4, assign64710_e99755_d_n5, assign64710_e99755_d_n6, assign64710_e99755_d_n7, assign64710_e99755_d_n8, assign64710_e99755_d_n9, assign64710_e99755_d_n10, assign64710_e99755_d_n11, assign64710_e99755_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1545 != 0.0)) {
        let assign64710_e99752: f64 = (3.0 * locals.var_t1);
        let assign64710_e99753: f64 = (81.0 + assign64710_e99752);
        (assign64710_e99753, (3.0 * locals.var_t1_dn0), (3.0 * locals.var_t1_dn2), (3.0 * locals.var_t1_dn4), (3.0 * locals.var_t1_dn5), (3.0 * locals.var_t1_dn6), (3.0 * locals.var_t1_dn7), (3.0 * locals.var_t1_dn8), (3.0 * locals.var_t1_dn9), (3.0 * locals.var_t1_dn10), (3.0 * locals.var_t1_dn11), (3.0 * locals.var_t1_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign64710_e99755;
        locals.var_t2_dn0 = assign64710_e99755_d_n0;
        locals.var_t2_dn2 = assign64710_e99755_d_n2;
        locals.var_t2_dn4 = assign64710_e99755_d_n4;
        locals.var_t2_dn5 = assign64710_e99755_d_n5;
        locals.var_t2_dn6 = assign64710_e99755_d_n6;
        locals.var_t2_dn7 = assign64710_e99755_d_n7;
        locals.var_t2_dn8 = assign64710_e99755_d_n8;
        locals.var_t2_dn9 = assign64710_e99755_d_n9;
        locals.var_t2_dn10 = assign64710_e99755_d_n10;
        locals.var_t2_dn11 = assign64710_e99755_d_n11;
        locals.var_t2_dn14 = assign64710_e99755_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign64720_e99778, assign64720_e99778_d_n0, assign64720_e99778_d_n2, assign64720_e99778_d_n4, assign64720_e99778_d_n5, assign64720_e99778_d_n6, assign64720_e99778_d_n7, assign64720_e99778_d_n8, assign64720_e99778_d_n9, assign64720_e99778_d_n10, assign64720_e99778_d_n11, assign64720_e99778_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1545 != 0.0)) {
        let assign64720_e99766: f64 = (-2916.0);
        let assign64720_e99769: f64 = (81.0 * locals.var_t1);
        let assign64720_e99770: f64 = (assign64720_e99766 - assign64720_e99769);
        let assign64720_e99773: f64 = (27.0 * locals.var_t1);
        let assign64720_e99775: f64 = (assign64720_e99773 * locals.var_ty);
        let assign64720_e99776: f64 = (assign64720_e99770 + assign64720_e99775);
        (assign64720_e99776, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign64720_e99773 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign64720_e99773 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn4)) + (((27.0 * locals.var_t1_dn4) * locals.var_ty) + (assign64720_e99773 * locals.var_ty_dn4))), ((-(81.0 * locals.var_t1_dn5)) + (((27.0 * locals.var_t1_dn5) * locals.var_ty) + (assign64720_e99773 * locals.var_ty_dn5))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign64720_e99773 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign64720_e99773 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn8)) + (((27.0 * locals.var_t1_dn8) * locals.var_ty) + (assign64720_e99773 * locals.var_ty_dn8))), ((-(81.0 * locals.var_t1_dn9)) + (((27.0 * locals.var_t1_dn9) * locals.var_ty) + (assign64720_e99773 * locals.var_ty_dn9))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign64720_e99773 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn11)) + (((27.0 * locals.var_t1_dn11) * locals.var_ty) + (assign64720_e99773 * locals.var_ty_dn11))), ((-(81.0 * locals.var_t1_dn14)) + (((27.0 * locals.var_t1_dn14) * locals.var_ty) + (assign64720_e99773 * locals.var_ty_dn14))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign64720_e99778;
        locals.var_t3_dn0 = assign64720_e99778_d_n0;
        locals.var_t3_dn2 = assign64720_e99778_d_n2;
        locals.var_t3_dn4 = assign64720_e99778_d_n4;
        locals.var_t3_dn5 = assign64720_e99778_d_n5;
        locals.var_t3_dn6 = assign64720_e99778_d_n6;
        locals.var_t3_dn7 = assign64720_e99778_d_n7;
        locals.var_t3_dn8 = assign64720_e99778_d_n8;
        locals.var_t3_dn9 = assign64720_e99778_d_n9;
        locals.var_t3_dn10 = assign64720_e99778_d_n10;
        locals.var_t3_dn11 = assign64720_e99778_d_n11;
        locals.var_t3_dn14 = assign64720_e99778_d_n14;
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_242(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign64730_e99802, assign64730_e99802_d_n0, assign64730_e99802_d_n2, assign64730_e99802_d_n4, assign64730_e99802_d_n5, assign64730_e99802_d_n6, assign64730_e99802_d_n7, assign64730_e99802_d_n8, assign64730_e99802_d_n9, assign64730_e99802_d_n10, assign64730_e99802_d_n11, assign64730_e99802_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1545 != 0.0)) {
        let assign64730_e99792: f64 = (54.0 + locals.var_t1);
        let assign64730_e99793: f64 = (81.0 * assign64730_e99792);
        let assign64730_e99794: f64 = (1458.0 - assign64730_e99793);
        let assign64730_e99797: f64 = (27.0 * locals.var_t1);
        let assign64730_e99799: f64 = (assign64730_e99797 * locals.var_ty);
        let assign64730_e99800: f64 = (assign64730_e99794 + assign64730_e99799);
        (assign64730_e99800, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign64730_e99797 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign64730_e99797 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn4)) + (((27.0 * locals.var_t1_dn4) * locals.var_ty) + (assign64730_e99797 * locals.var_ty_dn4))), ((-(81.0 * locals.var_t1_dn5)) + (((27.0 * locals.var_t1_dn5) * locals.var_ty) + (assign64730_e99797 * locals.var_ty_dn5))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign64730_e99797 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign64730_e99797 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn8)) + (((27.0 * locals.var_t1_dn8) * locals.var_ty) + (assign64730_e99797 * locals.var_ty_dn8))), ((-(81.0 * locals.var_t1_dn9)) + (((27.0 * locals.var_t1_dn9) * locals.var_ty) + (assign64730_e99797 * locals.var_ty_dn9))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign64730_e99797 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn11)) + (((27.0 * locals.var_t1_dn11) * locals.var_ty) + (assign64730_e99797 * locals.var_ty_dn11))), ((-(81.0 * locals.var_t1_dn14)) + (((27.0 * locals.var_t1_dn14) * locals.var_ty) + (assign64730_e99797 * locals.var_ty_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign64730_e99802;
        locals.var_t4_dn0 = assign64730_e99802_d_n0;
        locals.var_t4_dn2 = assign64730_e99802_d_n2;
        locals.var_t4_dn4 = assign64730_e99802_d_n4;
        locals.var_t4_dn5 = assign64730_e99802_d_n5;
        locals.var_t4_dn6 = assign64730_e99802_d_n6;
        locals.var_t4_dn7 = assign64730_e99802_d_n7;
        locals.var_t4_dn8 = assign64730_e99802_d_n8;
        locals.var_t4_dn9 = assign64730_e99802_d_n9;
        locals.var_t4_dn10 = assign64730_e99802_d_n10;
        locals.var_t4_dn11 = assign64730_e99802_d_n11;
        locals.var_t4_dn14 = assign64730_e99802_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign64740_e99816, assign64740_e99816_d_n0, assign64740_e99816_d_n2, assign64740_e99816_d_n4, assign64740_e99816_d_n5, assign64740_e99816_d_n6, assign64740_e99816_d_n7, assign64740_e99816_d_n8, assign64740_e99816_d_n9, assign64740_e99816_d_n10, assign64740_e99816_d_n11, assign64740_e99816_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1545 != 0.0)) {
        let assign64740_e99814: f64 = (locals.var_t4 * locals.var_t4);
        (assign64740_e99814, ((locals.var_t4_dn0 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn0)), ((locals.var_t4_dn2 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn2)), ((locals.var_t4_dn4 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn4)), ((locals.var_t4_dn5 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn5)), ((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)), ((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)), ((locals.var_t4_dn8 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn8)), ((locals.var_t4_dn9 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn9)), ((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)), ((locals.var_t4_dn11 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn11)), ((locals.var_t4_dn14 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign64740_e99816;
        locals.var_t4_dn0 = assign64740_e99816_d_n0;
        locals.var_t4_dn2 = assign64740_e99816_d_n2;
        locals.var_t4_dn4 = assign64740_e99816_d_n4;
        locals.var_t4_dn5 = assign64740_e99816_d_n5;
        locals.var_t4_dn6 = assign64740_e99816_d_n6;
        locals.var_t4_dn7 = assign64740_e99816_d_n7;
        locals.var_t4_dn8 = assign64740_e99816_d_n8;
        locals.var_t4_dn9 = assign64740_e99816_d_n9;
        locals.var_t4_dn10 = assign64740_e99816_d_n10;
        locals.var_t4_dn11 = assign64740_e99816_d_n11;
        locals.var_t4_dn14 = assign64740_e99816_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign64750_e99857, assign64750_e99857_d_n0, assign64750_e99857_d_n2, assign64750_e99857_d_n4, assign64750_e99857_d_n5, assign64750_e99857_d_n6, assign64750_e99857_d_n7, assign64750_e99857_d_n8, assign64750_e99857_d_n9, assign64750_e99857_d_n10, assign64750_e99857_d_n11, assign64750_e99857_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1545 != 0.0)) {
        let assign64750_e99829: f64 = (4.0 * locals.var_t2);
        let assign64750_e99831: f64 = (assign64750_e99829 * locals.var_t2);
        let assign64750_e99833: f64 = (assign64750_e99831 * locals.var_t2);
        let assign64750_e99835: f64 = (assign64750_e99833 + locals.var_t4);
        let assign64750_e99836: f64 = (assign64750_e99835).sqrt();
        let assign64750_e99837: f64 = (locals.var_t3 + assign64750_e99836);
        let (assign64750_e99855, assign64750_e99855_d_n0, assign64750_e99855_d_n2, assign64750_e99855_d_n4, assign64750_e99855_d_n5, assign64750_e99855_d_n6, assign64750_e99855_d_n7, assign64750_e99855_d_n8, assign64750_e99855_d_n9, assign64750_e99855_d_n10, assign64750_e99855_d_n11, assign64750_e99855_d_n14,) = {
            if (assign64750_e99837 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign64750_e99844: f64 = (4.0 * locals.var_t2);
                let assign64750_e99846: f64 = (assign64750_e99844 * locals.var_t2);
                let assign64750_e99848: f64 = (assign64750_e99846 * locals.var_t2);
                let assign64750_e99850: f64 = (assign64750_e99848 + locals.var_t4);
                let assign64750_e99851: f64 = (assign64750_e99850).sqrt();
                let assign64750_e99852: f64 = (locals.var_t3 + assign64750_e99851);
                let assign64750_e99854: f64 = (assign64750_e99852).powf(0.3333333333333333);
                (assign64750_e99854, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64750_e99852).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign64750_e99844 * locals.var_t2_dn0)) * locals.var_t2) + (assign64750_e99846 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign64750_e99851))))) } } else { (assign64750_e99854 * (0.3333333333333333 * ((locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign64750_e99844 * locals.var_t2_dn0)) * locals.var_t2) + (assign64750_e99846 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign64750_e99851))) / assign64750_e99852))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64750_e99852).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign64750_e99844 * locals.var_t2_dn2)) * locals.var_t2) + (assign64750_e99846 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign64750_e99851))))) } } else { (assign64750_e99854 * (0.3333333333333333 * ((locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign64750_e99844 * locals.var_t2_dn2)) * locals.var_t2) + (assign64750_e99846 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign64750_e99851))) / assign64750_e99852))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64750_e99852).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn4 + (((((((4.0 * locals.var_t2_dn4) * locals.var_t2) + (assign64750_e99844 * locals.var_t2_dn4)) * locals.var_t2) + (assign64750_e99846 * locals.var_t2_dn4)) + locals.var_t4_dn4) / (2.0 * assign64750_e99851))))) } } else { (assign64750_e99854 * (0.3333333333333333 * ((locals.var_t3_dn4 + (((((((4.0 * locals.var_t2_dn4) * locals.var_t2) + (assign64750_e99844 * locals.var_t2_dn4)) * locals.var_t2) + (assign64750_e99846 * locals.var_t2_dn4)) + locals.var_t4_dn4) / (2.0 * assign64750_e99851))) / assign64750_e99852))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64750_e99852).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn5 + (((((((4.0 * locals.var_t2_dn5) * locals.var_t2) + (assign64750_e99844 * locals.var_t2_dn5)) * locals.var_t2) + (assign64750_e99846 * locals.var_t2_dn5)) + locals.var_t4_dn5) / (2.0 * assign64750_e99851))))) } } else { (assign64750_e99854 * (0.3333333333333333 * ((locals.var_t3_dn5 + (((((((4.0 * locals.var_t2_dn5) * locals.var_t2) + (assign64750_e99844 * locals.var_t2_dn5)) * locals.var_t2) + (assign64750_e99846 * locals.var_t2_dn5)) + locals.var_t4_dn5) / (2.0 * assign64750_e99851))) / assign64750_e99852))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64750_e99852).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign64750_e99844 * locals.var_t2_dn6)) * locals.var_t2) + (assign64750_e99846 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign64750_e99851))))) } } else { (assign64750_e99854 * (0.3333333333333333 * ((locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign64750_e99844 * locals.var_t2_dn6)) * locals.var_t2) + (assign64750_e99846 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign64750_e99851))) / assign64750_e99852))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64750_e99852).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign64750_e99844 * locals.var_t2_dn7)) * locals.var_t2) + (assign64750_e99846 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign64750_e99851))))) } } else { (assign64750_e99854 * (0.3333333333333333 * ((locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign64750_e99844 * locals.var_t2_dn7)) * locals.var_t2) + (assign64750_e99846 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign64750_e99851))) / assign64750_e99852))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64750_e99852).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn8 + (((((((4.0 * locals.var_t2_dn8) * locals.var_t2) + (assign64750_e99844 * locals.var_t2_dn8)) * locals.var_t2) + (assign64750_e99846 * locals.var_t2_dn8)) + locals.var_t4_dn8) / (2.0 * assign64750_e99851))))) } } else { (assign64750_e99854 * (0.3333333333333333 * ((locals.var_t3_dn8 + (((((((4.0 * locals.var_t2_dn8) * locals.var_t2) + (assign64750_e99844 * locals.var_t2_dn8)) * locals.var_t2) + (assign64750_e99846 * locals.var_t2_dn8)) + locals.var_t4_dn8) / (2.0 * assign64750_e99851))) / assign64750_e99852))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64750_e99852).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn9 + (((((((4.0 * locals.var_t2_dn9) * locals.var_t2) + (assign64750_e99844 * locals.var_t2_dn9)) * locals.var_t2) + (assign64750_e99846 * locals.var_t2_dn9)) + locals.var_t4_dn9) / (2.0 * assign64750_e99851))))) } } else { (assign64750_e99854 * (0.3333333333333333 * ((locals.var_t3_dn9 + (((((((4.0 * locals.var_t2_dn9) * locals.var_t2) + (assign64750_e99844 * locals.var_t2_dn9)) * locals.var_t2) + (assign64750_e99846 * locals.var_t2_dn9)) + locals.var_t4_dn9) / (2.0 * assign64750_e99851))) / assign64750_e99852))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64750_e99852).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign64750_e99844 * locals.var_t2_dn10)) * locals.var_t2) + (assign64750_e99846 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign64750_e99851))))) } } else { (assign64750_e99854 * (0.3333333333333333 * ((locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign64750_e99844 * locals.var_t2_dn10)) * locals.var_t2) + (assign64750_e99846 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign64750_e99851))) / assign64750_e99852))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64750_e99852).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn11 + (((((((4.0 * locals.var_t2_dn11) * locals.var_t2) + (assign64750_e99844 * locals.var_t2_dn11)) * locals.var_t2) + (assign64750_e99846 * locals.var_t2_dn11)) + locals.var_t4_dn11) / (2.0 * assign64750_e99851))))) } } else { (assign64750_e99854 * (0.3333333333333333 * ((locals.var_t3_dn11 + (((((((4.0 * locals.var_t2_dn11) * locals.var_t2) + (assign64750_e99844 * locals.var_t2_dn11)) * locals.var_t2) + (assign64750_e99846 * locals.var_t2_dn11)) + locals.var_t4_dn11) / (2.0 * assign64750_e99851))) / assign64750_e99852))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64750_e99852).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn14 + (((((((4.0 * locals.var_t2_dn14) * locals.var_t2) + (assign64750_e99844 * locals.var_t2_dn14)) * locals.var_t2) + (assign64750_e99846 * locals.var_t2_dn14)) + locals.var_t4_dn14) / (2.0 * assign64750_e99851))))) } } else { (assign64750_e99854 * (0.3333333333333333 * ((locals.var_t3_dn14 + (((((((4.0 * locals.var_t2_dn14) * locals.var_t2) + (assign64750_e99844 * locals.var_t2_dn14)) * locals.var_t2) + (assign64750_e99846 * locals.var_t2_dn14)) + locals.var_t4_dn14) / (2.0 * assign64750_e99851))) / assign64750_e99852))) },)
            }
        };
        (assign64750_e99855, assign64750_e99855_d_n0, assign64750_e99855_d_n2, assign64750_e99855_d_n4, assign64750_e99855_d_n5, assign64750_e99855_d_n6, assign64750_e99855_d_n7, assign64750_e99855_d_n8, assign64750_e99855_d_n9, assign64750_e99855_d_n10, assign64750_e99855_d_n11, assign64750_e99855_d_n14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign64750_e99857;
        locals.var_t5_dn0 = assign64750_e99857_d_n0;
        locals.var_t5_dn2 = assign64750_e99857_d_n2;
        locals.var_t5_dn4 = assign64750_e99857_d_n4;
        locals.var_t5_dn5 = assign64750_e99857_d_n5;
        locals.var_t5_dn6 = assign64750_e99857_d_n6;
        locals.var_t5_dn7 = assign64750_e99857_d_n7;
        locals.var_t5_dn8 = assign64750_e99857_d_n8;
        locals.var_t5_dn9 = assign64750_e99857_d_n9;
        locals.var_t5_dn10 = assign64750_e99857_d_n10;
        locals.var_t5_dn11 = assign64750_e99857_d_n11;
        locals.var_t5_dn14 = assign64750_e99857_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign64760_e99885, assign64760_e99885_d_n0, assign64760_e99885_d_n2, assign64760_e99885_d_n4, assign64760_e99885_d_n5, assign64760_e99885_d_n6, assign64760_e99885_d_n7, assign64760_e99885_d_n8, assign64760_e99885_d_n9, assign64760_e99885_d_n10, assign64760_e99885_d_n11, assign64760_e99885_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1545 != 0.0)) {
        let assign64760_e99870: f64 = (1.259921049894873 * locals.var_t2);
        let assign64760_e99873: f64 = (3.0 * locals.var_t5);
        let assign64760_e99874: f64 = (assign64760_e99870 / assign64760_e99873);
        let assign64760_e99875: f64 = (3.0 - assign64760_e99874);
        let assign64760_e99879: f64 = (3.0 * 1.259921049894873);
        let assign64760_e99880: f64 = (1.0 / assign64760_e99879);
        let assign64760_e99882: f64 = (assign64760_e99880 * locals.var_t5);
        let assign64760_e99883: f64 = (assign64760_e99875 + assign64760_e99882);
        (assign64760_e99883, ((-((((1.259921049894873 * locals.var_t2_dn0) * assign64760_e99873) - (assign64760_e99870 * (3.0 * locals.var_t5_dn0))) / (assign64760_e99873 * assign64760_e99873))) + (assign64760_e99880 * locals.var_t5_dn0)), ((-((((1.259921049894873 * locals.var_t2_dn2) * assign64760_e99873) - (assign64760_e99870 * (3.0 * locals.var_t5_dn2))) / (assign64760_e99873 * assign64760_e99873))) + (assign64760_e99880 * locals.var_t5_dn2)), ((-((((1.259921049894873 * locals.var_t2_dn4) * assign64760_e99873) - (assign64760_e99870 * (3.0 * locals.var_t5_dn4))) / (assign64760_e99873 * assign64760_e99873))) + (assign64760_e99880 * locals.var_t5_dn4)), ((-((((1.259921049894873 * locals.var_t2_dn5) * assign64760_e99873) - (assign64760_e99870 * (3.0 * locals.var_t5_dn5))) / (assign64760_e99873 * assign64760_e99873))) + (assign64760_e99880 * locals.var_t5_dn5)), ((-((((1.259921049894873 * locals.var_t2_dn6) * assign64760_e99873) - (assign64760_e99870 * (3.0 * locals.var_t5_dn6))) / (assign64760_e99873 * assign64760_e99873))) + (assign64760_e99880 * locals.var_t5_dn6)), ((-((((1.259921049894873 * locals.var_t2_dn7) * assign64760_e99873) - (assign64760_e99870 * (3.0 * locals.var_t5_dn7))) / (assign64760_e99873 * assign64760_e99873))) + (assign64760_e99880 * locals.var_t5_dn7)), ((-((((1.259921049894873 * locals.var_t2_dn8) * assign64760_e99873) - (assign64760_e99870 * (3.0 * locals.var_t5_dn8))) / (assign64760_e99873 * assign64760_e99873))) + (assign64760_e99880 * locals.var_t5_dn8)), ((-((((1.259921049894873 * locals.var_t2_dn9) * assign64760_e99873) - (assign64760_e99870 * (3.0 * locals.var_t5_dn9))) / (assign64760_e99873 * assign64760_e99873))) + (assign64760_e99880 * locals.var_t5_dn9)), ((-((((1.259921049894873 * locals.var_t2_dn10) * assign64760_e99873) - (assign64760_e99870 * (3.0 * locals.var_t5_dn10))) / (assign64760_e99873 * assign64760_e99873))) + (assign64760_e99880 * locals.var_t5_dn10)), ((-((((1.259921049894873 * locals.var_t2_dn11) * assign64760_e99873) - (assign64760_e99870 * (3.0 * locals.var_t5_dn11))) / (assign64760_e99873 * assign64760_e99873))) + (assign64760_e99880 * locals.var_t5_dn11)), ((-((((1.259921049894873 * locals.var_t2_dn14) * assign64760_e99873) - (assign64760_e99870 * (3.0 * locals.var_t5_dn14))) / (assign64760_e99873 * assign64760_e99873))) + (assign64760_e99880 * locals.var_t5_dn14)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign64760_e99885;
        locals.var_tx_dn0 = assign64760_e99885_d_n0;
        locals.var_tx_dn2 = assign64760_e99885_d_n2;
        locals.var_tx_dn4 = assign64760_e99885_d_n4;
        locals.var_tx_dn5 = assign64760_e99885_d_n5;
        locals.var_tx_dn6 = assign64760_e99885_d_n6;
        locals.var_tx_dn7 = assign64760_e99885_d_n7;
        locals.var_tx_dn8 = assign64760_e99885_d_n8;
        locals.var_tx_dn9 = assign64760_e99885_d_n9;
        locals.var_tx_dn10 = assign64760_e99885_d_n10;
        locals.var_tx_dn11 = assign64760_e99885_d_n11;
        locals.var_tx_dn14 = assign64760_e99885_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign64770_e99901, assign64770_e99901_d_n0, assign64770_e99901_d_n2, assign64770_e99901_d_n4, assign64770_e99901_d_n5, assign64770_e99901_d_n6, assign64770_e99901_d_n7, assign64770_e99901_d_n8, assign64770_e99901_d_n9, assign64770_e99901_d_n10, assign64770_e99901_d_n11, assign64770_e99901_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1545 != 0.0)) {
        let assign64770_e99897: f64 = (locals.var_tx * locals.var_beta_inv);
        let assign64770_e99899: f64 = (assign64770_e99897 + locals.var_vbscl__blk1543);
        (assign64770_e99899, (((locals.var_tx_dn0 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn0)) + locals.var_vbscl__blk1543_dn0), (((locals.var_tx_dn2 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn2)) + locals.var_vbscl__blk1543_dn2), (((locals.var_tx_dn4 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn4)) + locals.var_vbscl__blk1543_dn4), (((locals.var_tx_dn5 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn5)) + locals.var_vbscl__blk1543_dn5), (((locals.var_tx_dn6 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn6)) + locals.var_vbscl__blk1543_dn6), (((locals.var_tx_dn7 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn7)) + locals.var_vbscl__blk1543_dn7), (((locals.var_tx_dn8 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn8)) + locals.var_vbscl__blk1543_dn8), (((locals.var_tx_dn9 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn9)) + locals.var_vbscl__blk1543_dn9), (((locals.var_tx_dn10 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn10)) + locals.var_vbscl__blk1543_dn10), (((locals.var_tx_dn11 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn11)) + locals.var_vbscl__blk1543_dn11), (((locals.var_tx_dn14 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn14)) + locals.var_vbscl__blk1543_dn14),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign64770_e99901;
        locals.var_ps0_inia_dn0 = assign64770_e99901_d_n0;
        locals.var_ps0_inia_dn2 = assign64770_e99901_d_n2;
        locals.var_ps0_inia_dn4 = assign64770_e99901_d_n4;
        locals.var_ps0_inia_dn5 = assign64770_e99901_d_n5;
        locals.var_ps0_inia_dn6 = assign64770_e99901_d_n6;
        locals.var_ps0_inia_dn7 = assign64770_e99901_d_n7;
        locals.var_ps0_inia_dn8 = assign64770_e99901_d_n8;
        locals.var_ps0_inia_dn9 = assign64770_e99901_d_n9;
        locals.var_ps0_inia_dn10 = assign64770_e99901_d_n10;
        locals.var_ps0_inia_dn11 = assign64770_e99901_d_n11;
        locals.var_ps0_inia_dn14 = assign64770_e99901_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let (assign64780_e99913, assign64780_e99913_d_n0, assign64780_e99913_d_n2, assign64780_e99913_d_n4, assign64780_e99913_d_n5, assign64780_e99913_d_n6, assign64780_e99913_d_n7, assign64780_e99913_d_n8, assign64780_e99913_d_n9, assign64780_e99913_d_n10, assign64780_e99913_d_n11, assign64780_e99913_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1545 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn14,)
    }
};
        locals.var_ps0_ini = assign64780_e99913;
        locals.var_ps0_ini_dn0 = assign64780_e99913_d_n0;
        locals.var_ps0_ini_dn2 = assign64780_e99913_d_n2;
        locals.var_ps0_ini_dn4 = assign64780_e99913_d_n4;
        locals.var_ps0_ini_dn5 = assign64780_e99913_d_n5;
        locals.var_ps0_ini_dn6 = assign64780_e99913_d_n6;
        locals.var_ps0_ini_dn7 = assign64780_e99913_d_n7;
        locals.var_ps0_ini_dn8 = assign64780_e99913_d_n8;
        locals.var_ps0_ini_dn9 = assign64780_e99913_d_n9;
        locals.var_ps0_ini_dn10 = assign64780_e99913_d_n10;
        locals.var_ps0_ini_dn11 = assign64780_e99913_d_n11;
        locals.var_ps0_ini_dn14 = assign64780_e99913_d_n14;
        locals.var_ps0_ini_rv = 0.0;

        let assign64790_e99916: f64 = if locals.var_vgs <= locals.var_vth__blk1544 { 1.0 } else { 0.0 };
        locals.var_guard1546 = assign64790_e99916;
        locals.var_guard1546_rv = 0.0;

        let (assign64800_e99931, assign64800_e99931_d_n0, assign64800_e99931_d_n2, assign64800_e99931_d_n4, assign64800_e99931_d_n5, assign64800_e99931_d_n6, assign64800_e99931_d_n7, assign64800_e99931_d_n8, assign64800_e99931_d_n9, assign64800_e99931_d_n10, assign64800_e99931_d_n11, assign64800_e99931_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1545 == 0.0)) && (locals.var_guard1546 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn14,)
    }
};
        locals.var_ps0_ini = assign64800_e99931;
        locals.var_ps0_ini_dn0 = assign64800_e99931_d_n0;
        locals.var_ps0_ini_dn2 = assign64800_e99931_d_n2;
        locals.var_ps0_ini_dn4 = assign64800_e99931_d_n4;
        locals.var_ps0_ini_dn5 = assign64800_e99931_d_n5;
        locals.var_ps0_ini_dn6 = assign64800_e99931_d_n6;
        locals.var_ps0_ini_dn7 = assign64800_e99931_d_n7;
        locals.var_ps0_ini_dn8 = assign64800_e99931_d_n8;
        locals.var_ps0_ini_dn9 = assign64800_e99931_d_n9;
        locals.var_ps0_ini_dn10 = assign64800_e99931_d_n10;
        locals.var_ps0_ini_dn11 = assign64800_e99931_d_n11;
        locals.var_ps0_ini_dn14 = assign64800_e99931_d_n14;
        locals.var_ps0_ini_rv = 0.0;

        let (assign64810_e99951, assign64810_e99951_d_n0, assign64810_e99951_d_n2, assign64810_e99951_d_n4, assign64810_e99951_d_n5, assign64810_e99951_d_n6, assign64810_e99951_d_n7, assign64810_e99951_d_n8, assign64810_e99951_d_n9, assign64810_e99951_d_n10, assign64810_e99951_d_n11, assign64810_e99951_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1545 == 0.0)) && (locals.var_guard1546 == 0.0)) {
        let assign64810_e99947: f64 = (1.0 / locals.var_cnst1);
        let assign64810_e99949: f64 = (assign64810_e99947 / locals.var_cnstcoxi);
        (assign64810_e99949, ((((-(locals.var_cnst1_dn0 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64810_e99947 * locals.var_cnstcoxi_dn0)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn2 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64810_e99947 * locals.var_cnstcoxi_dn2)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn4 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64810_e99947 * locals.var_cnstcoxi_dn4)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn5 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64810_e99947 * locals.var_cnstcoxi_dn5)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn6 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64810_e99947 * locals.var_cnstcoxi_dn6)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn7 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64810_e99947 * locals.var_cnstcoxi_dn7)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn8 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64810_e99947 * locals.var_cnstcoxi_dn8)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn9 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64810_e99947 * locals.var_cnstcoxi_dn9)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn10 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64810_e99947 * locals.var_cnstcoxi_dn10)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn11 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64810_e99947 * locals.var_cnstcoxi_dn11)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn14 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64810_e99947 * locals.var_cnstcoxi_dn14)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign64810_e99951;
        locals.var_t1_dn0 = assign64810_e99951_d_n0;
        locals.var_t1_dn2 = assign64810_e99951_d_n2;
        locals.var_t1_dn4 = assign64810_e99951_d_n4;
        locals.var_t1_dn5 = assign64810_e99951_d_n5;
        locals.var_t1_dn6 = assign64810_e99951_d_n6;
        locals.var_t1_dn7 = assign64810_e99951_d_n7;
        locals.var_t1_dn8 = assign64810_e99951_d_n8;
        locals.var_t1_dn9 = assign64810_e99951_d_n9;
        locals.var_t1_dn10 = assign64810_e99951_d_n10;
        locals.var_t1_dn11 = assign64810_e99951_d_n11;
        locals.var_t1_dn14 = assign64810_e99951_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign64820_e99971, assign64820_e99971_d_n0, assign64820_e99971_d_n2, assign64820_e99971_d_n4, assign64820_e99971_d_n5, assign64820_e99971_d_n6, assign64820_e99971_d_n7, assign64820_e99971_d_n8, assign64820_e99971_d_n9, assign64820_e99971_d_n10, assign64820_e99971_d_n11, assign64820_e99971_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1545 == 0.0)) && (locals.var_guard1546 == 0.0)) {
        let assign64820_e99967: f64 = (locals.var_t1 * locals.var_vgp__blk1529);
        let assign64820_e99969: f64 = (assign64820_e99967 * locals.var_vgp__blk1529);
        (assign64820_e99969, ((((locals.var_t1_dn0 * locals.var_vgp__blk1529) + (locals.var_t1 * locals.var_vgp__blk1529_dn0)) * locals.var_vgp__blk1529) + (assign64820_e99967 * locals.var_vgp__blk1529_dn0)), ((((locals.var_t1_dn2 * locals.var_vgp__blk1529) + (locals.var_t1 * locals.var_vgp__blk1529_dn2)) * locals.var_vgp__blk1529) + (assign64820_e99967 * locals.var_vgp__blk1529_dn2)), ((((locals.var_t1_dn4 * locals.var_vgp__blk1529) + (locals.var_t1 * locals.var_vgp__blk1529_dn4)) * locals.var_vgp__blk1529) + (assign64820_e99967 * locals.var_vgp__blk1529_dn4)), ((((locals.var_t1_dn5 * locals.var_vgp__blk1529) + (locals.var_t1 * locals.var_vgp__blk1529_dn5)) * locals.var_vgp__blk1529) + (assign64820_e99967 * locals.var_vgp__blk1529_dn5)), ((((locals.var_t1_dn6 * locals.var_vgp__blk1529) + (locals.var_t1 * locals.var_vgp__blk1529_dn6)) * locals.var_vgp__blk1529) + (assign64820_e99967 * locals.var_vgp__blk1529_dn6)), ((((locals.var_t1_dn7 * locals.var_vgp__blk1529) + (locals.var_t1 * locals.var_vgp__blk1529_dn7)) * locals.var_vgp__blk1529) + (assign64820_e99967 * locals.var_vgp__blk1529_dn7)), ((((locals.var_t1_dn8 * locals.var_vgp__blk1529) + (locals.var_t1 * locals.var_vgp__blk1529_dn8)) * locals.var_vgp__blk1529) + (assign64820_e99967 * locals.var_vgp__blk1529_dn8)), ((((locals.var_t1_dn9 * locals.var_vgp__blk1529) + (locals.var_t1 * locals.var_vgp__blk1529_dn9)) * locals.var_vgp__blk1529) + (assign64820_e99967 * locals.var_vgp__blk1529_dn9)), ((((locals.var_t1_dn10 * locals.var_vgp__blk1529) + (locals.var_t1 * locals.var_vgp__blk1529_dn10)) * locals.var_vgp__blk1529) + (assign64820_e99967 * locals.var_vgp__blk1529_dn10)), ((((locals.var_t1_dn11 * locals.var_vgp__blk1529) + (locals.var_t1 * locals.var_vgp__blk1529_dn11)) * locals.var_vgp__blk1529) + (assign64820_e99967 * locals.var_vgp__blk1529_dn11)), ((((locals.var_t1_dn14 * locals.var_vgp__blk1529) + (locals.var_t1 * locals.var_vgp__blk1529_dn14)) * locals.var_vgp__blk1529) + (assign64820_e99967 * locals.var_vgp__blk1529_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign64820_e99971;
        locals.var_t2_dn0 = assign64820_e99971_d_n0;
        locals.var_t2_dn2 = assign64820_e99971_d_n2;
        locals.var_t2_dn4 = assign64820_e99971_d_n4;
        locals.var_t2_dn5 = assign64820_e99971_d_n5;
        locals.var_t2_dn6 = assign64820_e99971_d_n6;
        locals.var_t2_dn7 = assign64820_e99971_d_n7;
        locals.var_t2_dn8 = assign64820_e99971_d_n8;
        locals.var_t2_dn9 = assign64820_e99971_d_n9;
        locals.var_t2_dn10 = assign64820_e99971_d_n10;
        locals.var_t2_dn11 = assign64820_e99971_d_n11;
        locals.var_t2_dn14 = assign64820_e99971_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign64830_e99991, assign64830_e99991_d_n0, assign64830_e99991_d_n2, assign64830_e99991_d_n4, assign64830_e99991_d_n5, assign64830_e99991_d_n6, assign64830_e99991_d_n7, assign64830_e99991_d_n8, assign64830_e99991_d_n9, assign64830_e99991_d_n10, assign64830_e99991_d_n11, assign64830_e99991_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1545 == 0.0)) && (locals.var_guard1546 == 0.0)) {
        let assign64830_e99988: f64 = (2.0 / locals.var_vgp__blk1529);
        let assign64830_e99989: f64 = (locals.var_beta + assign64830_e99988);
        (assign64830_e99989, (locals.var_beta_dn0 + (-((2.0 * locals.var_vgp__blk1529_dn0) / (locals.var_vgp__blk1529 * locals.var_vgp__blk1529)))), (locals.var_beta_dn2 + (-((2.0 * locals.var_vgp__blk1529_dn2) / (locals.var_vgp__blk1529 * locals.var_vgp__blk1529)))), (locals.var_beta_dn4 + (-((2.0 * locals.var_vgp__blk1529_dn4) / (locals.var_vgp__blk1529 * locals.var_vgp__blk1529)))), (locals.var_beta_dn5 + (-((2.0 * locals.var_vgp__blk1529_dn5) / (locals.var_vgp__blk1529 * locals.var_vgp__blk1529)))), (locals.var_beta_dn6 + (-((2.0 * locals.var_vgp__blk1529_dn6) / (locals.var_vgp__blk1529 * locals.var_vgp__blk1529)))), (locals.var_beta_dn7 + (-((2.0 * locals.var_vgp__blk1529_dn7) / (locals.var_vgp__blk1529 * locals.var_vgp__blk1529)))), (locals.var_beta_dn8 + (-((2.0 * locals.var_vgp__blk1529_dn8) / (locals.var_vgp__blk1529 * locals.var_vgp__blk1529)))), (locals.var_beta_dn9 + (-((2.0 * locals.var_vgp__blk1529_dn9) / (locals.var_vgp__blk1529 * locals.var_vgp__blk1529)))), (locals.var_beta_dn10 + (-((2.0 * locals.var_vgp__blk1529_dn10) / (locals.var_vgp__blk1529 * locals.var_vgp__blk1529)))), (locals.var_beta_dn11 + (-((2.0 * locals.var_vgp__blk1529_dn11) / (locals.var_vgp__blk1529 * locals.var_vgp__blk1529)))), (locals.var_beta_dn14 + (-((2.0 * locals.var_vgp__blk1529_dn14) / (locals.var_vgp__blk1529 * locals.var_vgp__blk1529)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign64830_e99991;
        locals.var_t3_dn0 = assign64830_e99991_d_n0;
        locals.var_t3_dn2 = assign64830_e99991_d_n2;
        locals.var_t3_dn4 = assign64830_e99991_d_n4;
        locals.var_t3_dn5 = assign64830_e99991_d_n5;
        locals.var_t3_dn6 = assign64830_e99991_d_n6;
        locals.var_t3_dn7 = assign64830_e99991_d_n7;
        locals.var_t3_dn8 = assign64830_e99991_d_n8;
        locals.var_t3_dn9 = assign64830_e99991_d_n9;
        locals.var_t3_dn10 = assign64830_e99991_d_n10;
        locals.var_t3_dn11 = assign64830_e99991_d_n11;
        locals.var_t3_dn14 = assign64830_e99991_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign64840_e100012, assign64840_e100012_d_n0, assign64840_e100012_d_n2, assign64840_e100012_d_n4, assign64840_e100012_d_n5, assign64840_e100012_d_n6, assign64840_e100012_d_n7, assign64840_e100012_d_n8, assign64840_e100012_d_n9, assign64840_e100012_d_n10, assign64840_e100012_d_n11, assign64840_e100012_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1545 == 0.0)) && (locals.var_guard1546 == 0.0)) {
        let assign64840_e100006: f64 = (locals.var_t2).ln();
        let assign64840_e100008: f64 = (assign64840_e100006 / locals.var_t3);
        let assign64840_e100010: f64 = (assign64840_e100008 + p.p456);
        (assign64840_e100010, ((((locals.var_t2_dn0 / locals.var_t2) * locals.var_t3) - (assign64840_e100006 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn2 / locals.var_t2) * locals.var_t3) - (assign64840_e100006 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn4 / locals.var_t2) * locals.var_t3) - (assign64840_e100006 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn5 / locals.var_t2) * locals.var_t3) - (assign64840_e100006 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn6 / locals.var_t2) * locals.var_t3) - (assign64840_e100006 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn7 / locals.var_t2) * locals.var_t3) - (assign64840_e100006 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn8 / locals.var_t2) * locals.var_t3) - (assign64840_e100006 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn9 / locals.var_t2) * locals.var_t3) - (assign64840_e100006 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn10 / locals.var_t2) * locals.var_t3) - (assign64840_e100006 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn11 / locals.var_t2) * locals.var_t3) - (assign64840_e100006 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn14 / locals.var_t2) * locals.var_t3) - (assign64840_e100006 * locals.var_t3_dn14)) / (locals.var_t3 * locals.var_t3)),)
    } else {
        (locals.var_ps0_inib, locals.var_ps0_inib_dn0, locals.var_ps0_inib_dn2, locals.var_ps0_inib_dn4, locals.var_ps0_inib_dn5, locals.var_ps0_inib_dn6, locals.var_ps0_inib_dn7, locals.var_ps0_inib_dn8, locals.var_ps0_inib_dn9, locals.var_ps0_inib_dn10, locals.var_ps0_inib_dn11, locals.var_ps0_inib_dn14,)
    }
};
        locals.var_ps0_inib = assign64840_e100012;
        locals.var_ps0_inib_dn0 = assign64840_e100012_d_n0;
        locals.var_ps0_inib_dn2 = assign64840_e100012_d_n2;
        locals.var_ps0_inib_dn4 = assign64840_e100012_d_n4;
        locals.var_ps0_inib_dn5 = assign64840_e100012_d_n5;
        locals.var_ps0_inib_dn6 = assign64840_e100012_d_n6;
        locals.var_ps0_inib_dn7 = assign64840_e100012_d_n7;
        locals.var_ps0_inib_dn8 = assign64840_e100012_d_n8;
        locals.var_ps0_inib_dn9 = assign64840_e100012_d_n9;
        locals.var_ps0_inib_dn10 = assign64840_e100012_d_n10;
        locals.var_ps0_inib_dn11 = assign64840_e100012_d_n11;
        locals.var_ps0_inib_dn14 = assign64840_e100012_d_n14;
        locals.var_ps0_inib_rv = 0.0;

        let (assign64850_e100032, assign64850_e100032_d_n0, assign64850_e100032_d_n2, assign64850_e100032_d_n4, assign64850_e100032_d_n5, assign64850_e100032_d_n6, assign64850_e100032_d_n7, assign64850_e100032_d_n8, assign64850_e100032_d_n9, assign64850_e100032_d_n10, assign64850_e100032_d_n11, assign64850_e100032_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1545 == 0.0)) && (locals.var_guard1546 == 0.0)) {
        let assign64850_e100028: f64 = (locals.var_ps0_inib - locals.var_ps0_inia);
        let assign64850_e100030: f64 = (assign64850_e100028 - 0.0008);
        (assign64850_e100030, (locals.var_ps0_inib_dn0 - locals.var_ps0_inia_dn0), (locals.var_ps0_inib_dn2 - locals.var_ps0_inia_dn2), (locals.var_ps0_inib_dn4 - locals.var_ps0_inia_dn4), (locals.var_ps0_inib_dn5 - locals.var_ps0_inia_dn5), (locals.var_ps0_inib_dn6 - locals.var_ps0_inia_dn6), (locals.var_ps0_inib_dn7 - locals.var_ps0_inia_dn7), (locals.var_ps0_inib_dn8 - locals.var_ps0_inia_dn8), (locals.var_ps0_inib_dn9 - locals.var_ps0_inia_dn9), (locals.var_ps0_inib_dn10 - locals.var_ps0_inia_dn10), (locals.var_ps0_inib_dn11 - locals.var_ps0_inia_dn11), (locals.var_ps0_inib_dn14 - locals.var_ps0_inia_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign64850_e100032;
        locals.var_tmf1_dn0 = assign64850_e100032_d_n0;
        locals.var_tmf1_dn2 = assign64850_e100032_d_n2;
        locals.var_tmf1_dn4 = assign64850_e100032_d_n4;
        locals.var_tmf1_dn5 = assign64850_e100032_d_n5;
        locals.var_tmf1_dn6 = assign64850_e100032_d_n6;
        locals.var_tmf1_dn7 = assign64850_e100032_d_n7;
        locals.var_tmf1_dn8 = assign64850_e100032_d_n8;
        locals.var_tmf1_dn9 = assign64850_e100032_d_n9;
        locals.var_tmf1_dn10 = assign64850_e100032_d_n10;
        locals.var_tmf1_dn11 = assign64850_e100032_d_n11;
        locals.var_tmf1_dn14 = assign64850_e100032_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign64860_e100052, assign64860_e100052_d_n0, assign64860_e100052_d_n2, assign64860_e100052_d_n4, assign64860_e100052_d_n5, assign64860_e100052_d_n6, assign64860_e100052_d_n7, assign64860_e100052_d_n8, assign64860_e100052_d_n9, assign64860_e100052_d_n10, assign64860_e100052_d_n11, assign64860_e100052_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1545 == 0.0)) && (locals.var_guard1546 == 0.0)) {
        let assign64860_e100048: f64 = (4.0 * locals.var_ps0_inib);
        let assign64860_e100050: f64 = (assign64860_e100048 * 0.0008);
        (assign64860_e100050, ((4.0 * locals.var_ps0_inib_dn0) * 0.0008), ((4.0 * locals.var_ps0_inib_dn2) * 0.0008), ((4.0 * locals.var_ps0_inib_dn4) * 0.0008), ((4.0 * locals.var_ps0_inib_dn5) * 0.0008), ((4.0 * locals.var_ps0_inib_dn6) * 0.0008), ((4.0 * locals.var_ps0_inib_dn7) * 0.0008), ((4.0 * locals.var_ps0_inib_dn8) * 0.0008), ((4.0 * locals.var_ps0_inib_dn9) * 0.0008), ((4.0 * locals.var_ps0_inib_dn10) * 0.0008), ((4.0 * locals.var_ps0_inib_dn11) * 0.0008), ((4.0 * locals.var_ps0_inib_dn14) * 0.0008),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign64860_e100052;
        locals.var_tmf2_dn0 = assign64860_e100052_d_n0;
        locals.var_tmf2_dn2 = assign64860_e100052_d_n2;
        locals.var_tmf2_dn4 = assign64860_e100052_d_n4;
        locals.var_tmf2_dn5 = assign64860_e100052_d_n5;
        locals.var_tmf2_dn6 = assign64860_e100052_d_n6;
        locals.var_tmf2_dn7 = assign64860_e100052_d_n7;
        locals.var_tmf2_dn8 = assign64860_e100052_d_n8;
        locals.var_tmf2_dn9 = assign64860_e100052_d_n9;
        locals.var_tmf2_dn10 = assign64860_e100052_d_n10;
        locals.var_tmf2_dn11 = assign64860_e100052_d_n11;
        locals.var_tmf2_dn14 = assign64860_e100052_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign64870_e100074, assign64870_e100074_d_n0, assign64870_e100074_d_n2, assign64870_e100074_d_n4, assign64870_e100074_d_n5, assign64870_e100074_d_n6, assign64870_e100074_d_n7, assign64870_e100074_d_n8, assign64870_e100074_d_n9, assign64870_e100074_d_n10, assign64870_e100074_d_n11, assign64870_e100074_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1545 == 0.0)) && (locals.var_guard1546 == 0.0)) {
        let (assign64870_e100072, assign64870_e100072_d_n0, assign64870_e100072_d_n2, assign64870_e100072_d_n4, assign64870_e100072_d_n5, assign64870_e100072_d_n6, assign64870_e100072_d_n7, assign64870_e100072_d_n8, assign64870_e100072_d_n9, assign64870_e100072_d_n10, assign64870_e100072_d_n11, assign64870_e100072_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign64870_e100071: f64 = (-locals.var_tmf2);
                (assign64870_e100071, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign64870_e100072, assign64870_e100072_d_n0, assign64870_e100072_d_n2, assign64870_e100072_d_n4, assign64870_e100072_d_n5, assign64870_e100072_d_n6, assign64870_e100072_d_n7, assign64870_e100072_d_n8, assign64870_e100072_d_n9, assign64870_e100072_d_n10, assign64870_e100072_d_n11, assign64870_e100072_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign64870_e100074;
        locals.var_tmf2_dn0 = assign64870_e100074_d_n0;
        locals.var_tmf2_dn2 = assign64870_e100074_d_n2;
        locals.var_tmf2_dn4 = assign64870_e100074_d_n4;
        locals.var_tmf2_dn5 = assign64870_e100074_d_n5;
        locals.var_tmf2_dn6 = assign64870_e100074_d_n6;
        locals.var_tmf2_dn7 = assign64870_e100074_d_n7;
        locals.var_tmf2_dn8 = assign64870_e100074_d_n8;
        locals.var_tmf2_dn9 = assign64870_e100074_d_n9;
        locals.var_tmf2_dn10 = assign64870_e100074_d_n10;
        locals.var_tmf2_dn11 = assign64870_e100074_d_n11;
        locals.var_tmf2_dn14 = assign64870_e100074_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign64880_e100095, assign64880_e100095_d_n0, assign64880_e100095_d_n2, assign64880_e100095_d_n4, assign64880_e100095_d_n5, assign64880_e100095_d_n6, assign64880_e100095_d_n7, assign64880_e100095_d_n8, assign64880_e100095_d_n9, assign64880_e100095_d_n10, assign64880_e100095_d_n11, assign64880_e100095_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1545 == 0.0)) && (locals.var_guard1546 == 0.0)) {
        let assign64880_e100090: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign64880_e100092: f64 = (assign64880_e100090 + locals.var_tmf2);
        let assign64880_e100093: f64 = (assign64880_e100092).sqrt();
        (assign64880_e100093, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign64880_e100093)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign64880_e100093)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign64880_e100093)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign64880_e100093)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign64880_e100093)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign64880_e100093)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign64880_e100093)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign64880_e100093)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign64880_e100093)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign64880_e100093)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign64880_e100093)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign64880_e100095;
        locals.var_tmf2_dn0 = assign64880_e100095_d_n0;
        locals.var_tmf2_dn2 = assign64880_e100095_d_n2;
        locals.var_tmf2_dn4 = assign64880_e100095_d_n4;
        locals.var_tmf2_dn5 = assign64880_e100095_d_n5;
        locals.var_tmf2_dn6 = assign64880_e100095_d_n6;
        locals.var_tmf2_dn7 = assign64880_e100095_d_n7;
        locals.var_tmf2_dn8 = assign64880_e100095_d_n8;
        locals.var_tmf2_dn9 = assign64880_e100095_d_n9;
        locals.var_tmf2_dn10 = assign64880_e100095_d_n10;
        locals.var_tmf2_dn11 = assign64880_e100095_d_n11;
        locals.var_tmf2_dn14 = assign64880_e100095_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign64890_e100117, assign64890_e100117_d_n0, assign64890_e100117_d_n2, assign64890_e100117_d_n4, assign64890_e100117_d_n5, assign64890_e100117_d_n6, assign64890_e100117_d_n7, assign64890_e100117_d_n8, assign64890_e100117_d_n9, assign64890_e100117_d_n10, assign64890_e100117_d_n11, assign64890_e100117_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1545 == 0.0)) && (locals.var_guard1546 == 0.0)) {
        let assign64890_e100113: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign64890_e100114: f64 = (0.5 * assign64890_e100113);
        let assign64890_e100115: f64 = (locals.var_ps0_inib - assign64890_e100114);
        (assign64890_e100115, (locals.var_ps0_inib_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_ps0_inib_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_ps0_inib_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_ps0_inib_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_ps0_inib_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_ps0_inib_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_ps0_inib_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_ps0_inib_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_ps0_inib_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_ps0_inib_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_ps0_inib_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn14,)
    }
};
        locals.var_ps0_ini = assign64890_e100117;
        locals.var_ps0_ini_dn0 = assign64890_e100117_d_n0;
        locals.var_ps0_ini_dn2 = assign64890_e100117_d_n2;
        locals.var_ps0_ini_dn4 = assign64890_e100117_d_n4;
        locals.var_ps0_ini_dn5 = assign64890_e100117_d_n5;
        locals.var_ps0_ini_dn6 = assign64890_e100117_d_n6;
        locals.var_ps0_ini_dn7 = assign64890_e100117_d_n7;
        locals.var_ps0_ini_dn8 = assign64890_e100117_d_n8;
        locals.var_ps0_ini_dn9 = assign64890_e100117_d_n9;
        locals.var_ps0_ini_dn10 = assign64890_e100117_d_n10;
        locals.var_ps0_ini_dn11 = assign64890_e100117_d_n11;
        locals.var_ps0_ini_dn14 = assign64890_e100117_d_n14;
        locals.var_ps0_ini_rv = 0.0;

        let (assign64900_e100131, assign64900_e100131_d_n0, assign64900_e100131_d_n2, assign64900_e100131_d_n4, assign64900_e100131_d_n5, assign64900_e100131_d_n6, assign64900_e100131_d_n7, assign64900_e100131_d_n8, assign64900_e100131_d_n9, assign64900_e100131_d_n10, assign64900_e100131_d_n11, assign64900_e100131_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) {
        let assign64900_e100128: f64 = (1e-12 / 2.0);
        let assign64900_e100129: f64 = (locals.var_vbscl__blk1543 + assign64900_e100128);
        (assign64900_e100129, locals.var_vbscl__blk1543_dn0, locals.var_vbscl__blk1543_dn2, locals.var_vbscl__blk1543_dn4, locals.var_vbscl__blk1543_dn5, locals.var_vbscl__blk1543_dn6, locals.var_vbscl__blk1543_dn7, locals.var_vbscl__blk1543_dn8, locals.var_vbscl__blk1543_dn9, locals.var_vbscl__blk1543_dn10, locals.var_vbscl__blk1543_dn11, locals.var_vbscl__blk1543_dn14,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign64900_e100131;
        locals.var_tx_dn0 = assign64900_e100131_d_n0;
        locals.var_tx_dn2 = assign64900_e100131_d_n2;
        locals.var_tx_dn4 = assign64900_e100131_d_n4;
        locals.var_tx_dn5 = assign64900_e100131_d_n5;
        locals.var_tx_dn6 = assign64900_e100131_d_n6;
        locals.var_tx_dn7 = assign64900_e100131_d_n7;
        locals.var_tx_dn8 = assign64900_e100131_d_n8;
        locals.var_tx_dn9 = assign64900_e100131_d_n9;
        locals.var_tx_dn10 = assign64900_e100131_d_n10;
        locals.var_tx_dn11 = assign64900_e100131_d_n11;
        locals.var_tx_dn14 = assign64900_e100131_d_n14;
        locals.var_tx_rv = 0.0;

        let assign64910_e100134: f64 = if locals.var_ps0_ini < locals.var_tx { 1.0 } else { 0.0 };
        locals.var_guard1547 = assign64910_e100134;
        locals.var_guard1547_rv = 0.0;

        let (assign64920_e100146, assign64920_e100146_d_n0, assign64920_e100146_d_n2, assign64920_e100146_d_n4, assign64920_e100146_d_n5, assign64920_e100146_d_n6, assign64920_e100146_d_n7, assign64920_e100146_d_n8, assign64920_e100146_d_n9, assign64920_e100146_d_n10, assign64920_e100146_d_n11, assign64920_e100146_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1547 != 0.0)) {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn14,)
    }
};
        locals.var_ps0_ini = assign64920_e100146;
        locals.var_ps0_ini_dn0 = assign64920_e100146_d_n0;
        locals.var_ps0_ini_dn2 = assign64920_e100146_d_n2;
        locals.var_ps0_ini_dn4 = assign64920_e100146_d_n4;
        locals.var_ps0_ini_dn5 = assign64920_e100146_d_n5;
        locals.var_ps0_ini_dn6 = assign64920_e100146_d_n6;
        locals.var_ps0_ini_dn7 = assign64920_e100146_d_n7;
        locals.var_ps0_ini_dn8 = assign64920_e100146_d_n8;
        locals.var_ps0_ini_dn9 = assign64920_e100146_d_n9;
        locals.var_ps0_ini_dn10 = assign64920_e100146_d_n10;
        locals.var_ps0_ini_dn11 = assign64920_e100146_d_n11;
        locals.var_ps0_ini_dn14 = assign64920_e100146_d_n14;
        locals.var_ps0_ini_rv = 0.0;

        let (assign64930_e100156, assign64930_e100156_d_n0, assign64930_e100156_d_n2, assign64930_e100156_d_n4, assign64930_e100156_d_n5, assign64930_e100156_d_n6, assign64930_e100156_d_n7, assign64930_e100156_d_n8, assign64930_e100156_d_n9, assign64930_e100156_d_n10, assign64930_e100156_d_n11, assign64930_e100156_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn14,)
    } else {
        (locals.var_ps0__blk1527, locals.var_ps0__blk1527_dn0, locals.var_ps0__blk1527_dn2, locals.var_ps0__blk1527_dn4, locals.var_ps0__blk1527_dn5, locals.var_ps0__blk1527_dn6, locals.var_ps0__blk1527_dn7, locals.var_ps0__blk1527_dn8, locals.var_ps0__blk1527_dn9, locals.var_ps0__blk1527_dn10, locals.var_ps0__blk1527_dn11, locals.var_ps0__blk1527_dn14,)
    }
};
        locals.var_ps0__blk1527 = assign64930_e100156;
        locals.var_ps0__blk1527_dn0 = assign64930_e100156_d_n0;
        locals.var_ps0__blk1527_dn2 = assign64930_e100156_d_n2;
        locals.var_ps0__blk1527_dn4 = assign64930_e100156_d_n4;
        locals.var_ps0__blk1527_dn5 = assign64930_e100156_d_n5;
        locals.var_ps0__blk1527_dn6 = assign64930_e100156_d_n6;
        locals.var_ps0__blk1527_dn7 = assign64930_e100156_d_n7;
        locals.var_ps0__blk1527_dn8 = assign64930_e100156_d_n8;
        locals.var_ps0__blk1527_dn9 = assign64930_e100156_d_n9;
        locals.var_ps0__blk1527_dn10 = assign64930_e100156_d_n10;
        locals.var_ps0__blk1527_dn11 = assign64930_e100156_d_n11;
        locals.var_ps0__blk1527_dn14 = assign64930_e100156_d_n14;
        locals.var_ps0__blk1527_rv = 0.0;

        let assign64940_e100159: f64 = if p.p451 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1548 = assign64940_e100159;
        locals.var_guard1548_rv = 0.0;

        let (assign64950_e100171, assign64950_e100171_d_n0, assign64950_e100171_d_n2, assign64950_e100171_d_n4, assign64950_e100171_d_n5, assign64950_e100171_d_n6, assign64950_e100171_d_n7, assign64950_e100171_d_n8, assign64950_e100171_d_n9, assign64950_e100171_d_n10, assign64950_e100171_d_n11, assign64950_e100171_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) {
        (locals.var_ps0__blk1527, locals.var_ps0__blk1527_dn0, locals.var_ps0__blk1527_dn2, locals.var_ps0__blk1527_dn4, locals.var_ps0__blk1527_dn5, locals.var_ps0__blk1527_dn6, locals.var_ps0__blk1527_dn7, locals.var_ps0__blk1527_dn8, locals.var_ps0__blk1527_dn9, locals.var_ps0__blk1527_dn10, locals.var_ps0__blk1527_dn11, locals.var_ps0__blk1527_dn14,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn14,)
    }
};
        locals.var_ps0_ini = assign64950_e100171;
        locals.var_ps0_ini_dn0 = assign64950_e100171_d_n0;
        locals.var_ps0_ini_dn2 = assign64950_e100171_d_n2;
        locals.var_ps0_ini_dn4 = assign64950_e100171_d_n4;
        locals.var_ps0_ini_dn5 = assign64950_e100171_d_n5;
        locals.var_ps0_ini_dn6 = assign64950_e100171_d_n6;
        locals.var_ps0_ini_dn7 = assign64950_e100171_d_n7;
        locals.var_ps0_ini_dn8 = assign64950_e100171_d_n8;
        locals.var_ps0_ini_dn9 = assign64950_e100171_d_n9;
        locals.var_ps0_ini_dn10 = assign64950_e100171_d_n10;
        locals.var_ps0_ini_dn11 = assign64950_e100171_d_n11;
        locals.var_ps0_ini_dn14 = assign64950_e100171_d_n14;
        locals.var_ps0_ini_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_243(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign64960_e100183, assign64960_e100183_d_n0, assign64960_e100183_d_n2, assign64960_e100183_d_n4, assign64960_e100183_d_n5, assign64960_e100183_d_n6, assign64960_e100183_d_n7, assign64960_e100183_d_n8, assign64960_e100183_d_n9, assign64960_e100183_d_n10, assign64960_e100183_d_n11, assign64960_e100183_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) {
        (locals.var_dphi_vds, locals.var_dphi_vds_dn0, locals.var_dphi_vds_dn2, locals.var_dphi_vds_dn4, locals.var_dphi_vds_dn5, locals.var_dphi_vds_dn6, locals.var_dphi_vds_dn7, locals.var_dphi_vds_dn8, locals.var_dphi_vds_dn9, locals.var_dphi_vds_dn10, locals.var_dphi_vds_dn11, locals.var_dphi_vds_dn14,)
    } else {
        (locals.var_vbscl__blk1549, locals.var_vbscl__blk1549_dn0, locals.var_vbscl__blk1549_dn2, locals.var_vbscl__blk1549_dn4, locals.var_vbscl__blk1549_dn5, locals.var_vbscl__blk1549_dn6, locals.var_vbscl__blk1549_dn7, locals.var_vbscl__blk1549_dn8, locals.var_vbscl__blk1549_dn9, locals.var_vbscl__blk1549_dn10, locals.var_vbscl__blk1549_dn11, locals.var_vbscl__blk1549_dn14,)
    }
};
        locals.var_vbscl__blk1549 = assign64960_e100183;
        locals.var_vbscl__blk1549_dn0 = assign64960_e100183_d_n0;
        locals.var_vbscl__blk1549_dn2 = assign64960_e100183_d_n2;
        locals.var_vbscl__blk1549_dn4 = assign64960_e100183_d_n4;
        locals.var_vbscl__blk1549_dn5 = assign64960_e100183_d_n5;
        locals.var_vbscl__blk1549_dn6 = assign64960_e100183_d_n6;
        locals.var_vbscl__blk1549_dn7 = assign64960_e100183_d_n7;
        locals.var_vbscl__blk1549_dn8 = assign64960_e100183_d_n8;
        locals.var_vbscl__blk1549_dn9 = assign64960_e100183_d_n9;
        locals.var_vbscl__blk1549_dn10 = assign64960_e100183_d_n10;
        locals.var_vbscl__blk1549_dn11 = assign64960_e100183_d_n11;
        locals.var_vbscl__blk1549_dn14 = assign64960_e100183_d_n14;
        locals.var_vbscl__blk1549_rv = 0.0;

        let (assign64970_e100203, assign64970_e100203_d_n0, assign64970_e100203_d_n2, assign64970_e100203_d_n4, assign64970_e100203_d_n5, assign64970_e100203_d_n6, assign64970_e100203_d_n7, assign64970_e100203_d_n8, assign64970_e100203_d_n9, assign64970_e100203_d_n10, assign64970_e100203_d_n11, assign64970_e100203_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) {
        let assign64970_e100195: f64 = (locals.var_vfb - locals.var_dvth);
        let assign64970_e100197: f64 = (assign64970_e100195 + locals.var_dppg);
        let assign64970_e100199: f64 = (assign64970_e100197 + locals.var_vbscl__blk1549);
        let assign64970_e100201: f64 = (assign64970_e100199 + p.p455);
        (assign64970_e100201, (((-locals.var_dvth_dn0) + locals.var_dppg_dn0) + locals.var_vbscl__blk1549_dn0), (((-locals.var_dvth_dn2) + locals.var_dppg_dn2) + locals.var_vbscl__blk1549_dn2), (((-locals.var_dvth_dn4) + locals.var_dppg_dn4) + locals.var_vbscl__blk1549_dn4), (((-locals.var_dvth_dn5) + locals.var_dppg_dn5) + locals.var_vbscl__blk1549_dn5), (((-locals.var_dvth_dn6) + locals.var_dppg_dn6) + locals.var_vbscl__blk1549_dn6), (((-locals.var_dvth_dn7) + locals.var_dppg_dn7) + locals.var_vbscl__blk1549_dn7), (((-locals.var_dvth_dn8) + locals.var_dppg_dn8) + locals.var_vbscl__blk1549_dn8), (((-locals.var_dvth_dn9) + locals.var_dppg_dn9) + locals.var_vbscl__blk1549_dn9), (((-locals.var_dvth_dn10) + locals.var_dppg_dn10) + locals.var_vbscl__blk1549_dn10), (((-locals.var_dvth_dn11) + locals.var_dppg_dn11) + locals.var_vbscl__blk1549_dn11), (((-locals.var_dvth_dn14) + locals.var_dppg_dn14) + locals.var_vbscl__blk1549_dn14),)
    } else {
        (locals.var_vgs_fb, locals.var_vgs_fb_dn0, locals.var_vgs_fb_dn2, locals.var_vgs_fb_dn4, locals.var_vgs_fb_dn5, locals.var_vgs_fb_dn6, locals.var_vgs_fb_dn7, locals.var_vgs_fb_dn8, locals.var_vgs_fb_dn9, locals.var_vgs_fb_dn10, locals.var_vgs_fb_dn11, locals.var_vgs_fb_dn14,)
    }
};
        locals.var_vgs_fb = assign64970_e100203;
        locals.var_vgs_fb_dn0 = assign64970_e100203_d_n0;
        locals.var_vgs_fb_dn2 = assign64970_e100203_d_n2;
        locals.var_vgs_fb_dn4 = assign64970_e100203_d_n4;
        locals.var_vgs_fb_dn5 = assign64970_e100203_d_n5;
        locals.var_vgs_fb_dn6 = assign64970_e100203_d_n6;
        locals.var_vgs_fb_dn7 = assign64970_e100203_d_n7;
        locals.var_vgs_fb_dn8 = assign64970_e100203_d_n8;
        locals.var_vgs_fb_dn9 = assign64970_e100203_d_n9;
        locals.var_vgs_fb_dn10 = assign64970_e100203_d_n10;
        locals.var_vgs_fb_dn11 = assign64970_e100203_d_n11;
        locals.var_vgs_fb_dn14 = assign64970_e100203_d_n14;
        locals.var_vgs_fb_rv = 0.0;

        let assign64980_e100206: f64 = if locals.var_vgs < locals.var_vgs_fb { 1.0 } else { 0.0 };
        locals.var_guard1558 = assign64980_e100206;
        locals.var_guard1558_rv = 0.0;

        let (assign64990_e100221,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 != 0.0)) {
        let assign64990_e100219: f64 = (-1.0);
        (assign64990_e100219,)
    } else {
        (locals.var_flg_zone,)
    }
};
        locals.var_flg_zone = assign64990_e100221;
        locals.var_flg_zone_rv = 0.0;

        let (assign65000_e100243, assign65000_e100243_d_n0, assign65000_e100243_d_n2, assign65000_e100243_d_n4, assign65000_e100243_d_n5, assign65000_e100243_d_n6, assign65000_e100243_d_n7, assign65000_e100243_d_n8, assign65000_e100243_d_n9, assign65000_e100243_d_n10, assign65000_e100243_d_n11, assign65000_e100243_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 != 0.0)) {
        let assign65000_e100235: f64 = (2.0 * locals.var_beta_inv);
        let assign65000_e100237: f64 = (-locals.var_vgs_min);
        let assign65000_e100239: f64 = (assign65000_e100237 / locals.var_fac1);
        let assign65000_e100240: f64 = (assign65000_e100239).ln();
        let assign65000_e100241: f64 = (assign65000_e100235 * assign65000_e100240);
        (assign65000_e100241, (((2.0 * locals.var_beta_inv_dn0) * assign65000_e100240) + (assign65000_e100235 * ((-((assign65000_e100237 * locals.var_fac1_dn0) / (locals.var_fac1 * locals.var_fac1))) / assign65000_e100239))), (((2.0 * locals.var_beta_inv_dn2) * assign65000_e100240) + (assign65000_e100235 * ((-((assign65000_e100237 * locals.var_fac1_dn2) / (locals.var_fac1 * locals.var_fac1))) / assign65000_e100239))), (((2.0 * locals.var_beta_inv_dn4) * assign65000_e100240) + (assign65000_e100235 * ((-((assign65000_e100237 * locals.var_fac1_dn4) / (locals.var_fac1 * locals.var_fac1))) / assign65000_e100239))), (((2.0 * locals.var_beta_inv_dn5) * assign65000_e100240) + (assign65000_e100235 * ((-((assign65000_e100237 * locals.var_fac1_dn5) / (locals.var_fac1 * locals.var_fac1))) / assign65000_e100239))), (((2.0 * locals.var_beta_inv_dn6) * assign65000_e100240) + (assign65000_e100235 * ((-((assign65000_e100237 * locals.var_fac1_dn6) / (locals.var_fac1 * locals.var_fac1))) / assign65000_e100239))), (((2.0 * locals.var_beta_inv_dn7) * assign65000_e100240) + (assign65000_e100235 * ((-((assign65000_e100237 * locals.var_fac1_dn7) / (locals.var_fac1 * locals.var_fac1))) / assign65000_e100239))), (((2.0 * locals.var_beta_inv_dn8) * assign65000_e100240) + (assign65000_e100235 * ((-((assign65000_e100237 * locals.var_fac1_dn8) / (locals.var_fac1 * locals.var_fac1))) / assign65000_e100239))), (((2.0 * locals.var_beta_inv_dn9) * assign65000_e100240) + (assign65000_e100235 * ((-((assign65000_e100237 * locals.var_fac1_dn9) / (locals.var_fac1 * locals.var_fac1))) / assign65000_e100239))), (((2.0 * locals.var_beta_inv_dn10) * assign65000_e100240) + (assign65000_e100235 * ((-((assign65000_e100237 * locals.var_fac1_dn10) / (locals.var_fac1 * locals.var_fac1))) / assign65000_e100239))), (((2.0 * locals.var_beta_inv_dn11) * assign65000_e100240) + (assign65000_e100235 * ((-((assign65000_e100237 * locals.var_fac1_dn11) / (locals.var_fac1 * locals.var_fac1))) / assign65000_e100239))), (((2.0 * locals.var_beta_inv_dn14) * assign65000_e100240) + (assign65000_e100235 * ((-((assign65000_e100237 * locals.var_fac1_dn14) / (locals.var_fac1 * locals.var_fac1))) / assign65000_e100239))),)
    } else {
        (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn4, locals.var_ps0_min_dn5, locals.var_ps0_min_dn6, locals.var_ps0_min_dn7, locals.var_ps0_min_dn8, locals.var_ps0_min_dn9, locals.var_ps0_min_dn10, locals.var_ps0_min_dn11, locals.var_ps0_min_dn14,)
    }
};
        locals.var_ps0_min = assign65000_e100243;
        locals.var_ps0_min_dn0 = assign65000_e100243_d_n0;
        locals.var_ps0_min_dn2 = assign65000_e100243_d_n2;
        locals.var_ps0_min_dn4 = assign65000_e100243_d_n4;
        locals.var_ps0_min_dn5 = assign65000_e100243_d_n5;
        locals.var_ps0_min_dn6 = assign65000_e100243_d_n6;
        locals.var_ps0_min_dn7 = assign65000_e100243_d_n7;
        locals.var_ps0_min_dn8 = assign65000_e100243_d_n8;
        locals.var_ps0_min_dn9 = assign65000_e100243_d_n9;
        locals.var_ps0_min_dn10 = assign65000_e100243_d_n10;
        locals.var_ps0_min_dn11 = assign65000_e100243_d_n11;
        locals.var_ps0_min_dn14 = assign65000_e100243_d_n14;
        locals.var_ps0_min_rv = 0.0;

        let (assign65010_e100261, assign65010_e100261_d_n0, assign65010_e100261_d_n2, assign65010_e100261_d_n4, assign65010_e100261_d_n5, assign65010_e100261_d_n6, assign65010_e100261_d_n7, assign65010_e100261_d_n8, assign65010_e100261_d_n9, assign65010_e100261_d_n10, assign65010_e100261_d_n11, assign65010_e100261_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 != 0.0)) {
        let assign65010_e100258: f64 = (locals.var_vgp__blk1529 - locals.var_vbscl__blk1549);
        let assign65010_e100259: f64 = (locals.var_beta * assign65010_e100258);
        (assign65010_e100259, ((locals.var_beta_dn0 * assign65010_e100258) + (locals.var_beta * (locals.var_vgp__blk1529_dn0 - locals.var_vbscl__blk1549_dn0))), ((locals.var_beta_dn2 * assign65010_e100258) + (locals.var_beta * (locals.var_vgp__blk1529_dn2 - locals.var_vbscl__blk1549_dn2))), ((locals.var_beta_dn4 * assign65010_e100258) + (locals.var_beta * (locals.var_vgp__blk1529_dn4 - locals.var_vbscl__blk1549_dn4))), ((locals.var_beta_dn5 * assign65010_e100258) + (locals.var_beta * (locals.var_vgp__blk1529_dn5 - locals.var_vbscl__blk1549_dn5))), ((locals.var_beta_dn6 * assign65010_e100258) + (locals.var_beta * (locals.var_vgp__blk1529_dn6 - locals.var_vbscl__blk1549_dn6))), ((locals.var_beta_dn7 * assign65010_e100258) + (locals.var_beta * (locals.var_vgp__blk1529_dn7 - locals.var_vbscl__blk1549_dn7))), ((locals.var_beta_dn8 * assign65010_e100258) + (locals.var_beta * (locals.var_vgp__blk1529_dn8 - locals.var_vbscl__blk1549_dn8))), ((locals.var_beta_dn9 * assign65010_e100258) + (locals.var_beta * (locals.var_vgp__blk1529_dn9 - locals.var_vbscl__blk1549_dn9))), ((locals.var_beta_dn10 * assign65010_e100258) + (locals.var_beta * (locals.var_vgp__blk1529_dn10 - locals.var_vbscl__blk1549_dn10))), ((locals.var_beta_dn11 * assign65010_e100258) + (locals.var_beta * (locals.var_vgp__blk1529_dn11 - locals.var_vbscl__blk1549_dn11))), ((locals.var_beta_dn14 * assign65010_e100258) + (locals.var_beta * (locals.var_vgp__blk1529_dn14 - locals.var_vbscl__blk1549_dn14))),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign65010_e100261;
        locals.var_tx_dn0 = assign65010_e100261_d_n0;
        locals.var_tx_dn2 = assign65010_e100261_d_n2;
        locals.var_tx_dn4 = assign65010_e100261_d_n4;
        locals.var_tx_dn5 = assign65010_e100261_d_n5;
        locals.var_tx_dn6 = assign65010_e100261_d_n6;
        locals.var_tx_dn7 = assign65010_e100261_d_n7;
        locals.var_tx_dn8 = assign65010_e100261_d_n8;
        locals.var_tx_dn9 = assign65010_e100261_d_n9;
        locals.var_tx_dn10 = assign65010_e100261_d_n10;
        locals.var_tx_dn11 = assign65010_e100261_d_n11;
        locals.var_tx_dn14 = assign65010_e100261_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign65020_e100279, assign65020_e100279_d_n0, assign65020_e100279_d_n2, assign65020_e100279_d_n4, assign65020_e100279_d_n5, assign65020_e100279_d_n6, assign65020_e100279_d_n7, assign65020_e100279_d_n8, assign65020_e100279_d_n9, assign65020_e100279_d_n10, assign65020_e100279_d_n11, assign65020_e100279_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 != 0.0)) {
        let assign65020_e100276: f64 = (locals.var_beta * locals.var_cnst0);
        let assign65020_e100277: f64 = (1.0 / assign65020_e100276);
        (assign65020_e100277, (-(((locals.var_beta_dn0 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn0)) / (assign65020_e100276 * assign65020_e100276))), (-(((locals.var_beta_dn2 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn2)) / (assign65020_e100276 * assign65020_e100276))), (-(((locals.var_beta_dn4 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn4)) / (assign65020_e100276 * assign65020_e100276))), (-(((locals.var_beta_dn5 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn5)) / (assign65020_e100276 * assign65020_e100276))), (-(((locals.var_beta_dn6 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn6)) / (assign65020_e100276 * assign65020_e100276))), (-(((locals.var_beta_dn7 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn7)) / (assign65020_e100276 * assign65020_e100276))), (-(((locals.var_beta_dn8 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn8)) / (assign65020_e100276 * assign65020_e100276))), (-(((locals.var_beta_dn9 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn9)) / (assign65020_e100276 * assign65020_e100276))), (-(((locals.var_beta_dn10 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn10)) / (assign65020_e100276 * assign65020_e100276))), (-(((locals.var_beta_dn11 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn11)) / (assign65020_e100276 * assign65020_e100276))), (-(((locals.var_beta_dn14 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn14)) / (assign65020_e100276 * assign65020_e100276))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign65020_e100279;
        locals.var_t1_dn0 = assign65020_e100279_d_n0;
        locals.var_t1_dn2 = assign65020_e100279_d_n2;
        locals.var_t1_dn4 = assign65020_e100279_d_n4;
        locals.var_t1_dn5 = assign65020_e100279_d_n5;
        locals.var_t1_dn6 = assign65020_e100279_d_n6;
        locals.var_t1_dn7 = assign65020_e100279_d_n7;
        locals.var_t1_dn8 = assign65020_e100279_d_n8;
        locals.var_t1_dn9 = assign65020_e100279_d_n9;
        locals.var_t1_dn10 = assign65020_e100279_d_n10;
        locals.var_t1_dn11 = assign65020_e100279_d_n11;
        locals.var_t1_dn14 = assign65020_e100279_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign65030_e100295, assign65030_e100295_d_n0, assign65030_e100295_d_n2, assign65030_e100295_d_n4, assign65030_e100295_d_n5, assign65030_e100295_d_n6, assign65030_e100295_d_n7, assign65030_e100295_d_n8, assign65030_e100295_d_n9, assign65030_e100295_d_n10, assign65030_e100295_d_n11, assign65030_e100295_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 != 0.0)) {
        let assign65030_e100293: f64 = (locals.var_t1 * locals.var_cox);
        (assign65030_e100293, ((locals.var_t1_dn0 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn0)), ((locals.var_t1_dn2 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn2)), ((locals.var_t1_dn4 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn4)), ((locals.var_t1_dn5 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn5)), ((locals.var_t1_dn6 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn6)), ((locals.var_t1_dn7 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn7)), ((locals.var_t1_dn8 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn8)), ((locals.var_t1_dn9 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn9)), ((locals.var_t1_dn10 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn10)), ((locals.var_t1_dn11 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn11)), ((locals.var_t1_dn14 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign65030_e100295;
        locals.var_ty_dn0 = assign65030_e100295_d_n0;
        locals.var_ty_dn2 = assign65030_e100295_d_n2;
        locals.var_ty_dn4 = assign65030_e100295_d_n4;
        locals.var_ty_dn5 = assign65030_e100295_d_n5;
        locals.var_ty_dn6 = assign65030_e100295_d_n6;
        locals.var_ty_dn7 = assign65030_e100295_d_n7;
        locals.var_ty_dn8 = assign65030_e100295_d_n8;
        locals.var_ty_dn9 = assign65030_e100295_d_n9;
        locals.var_ty_dn10 = assign65030_e100295_d_n10;
        locals.var_ty_dn11 = assign65030_e100295_d_n11;
        locals.var_ty_dn14 = assign65030_e100295_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign65040_e100315, assign65040_e100315_d_n0, assign65040_e100315_d_n2, assign65040_e100315_d_n4, assign65040_e100315_d_n5, assign65040_e100315_d_n6, assign65040_e100315_d_n7, assign65040_e100315_d_n8, assign65040_e100315_d_n9, assign65040_e100315_d_n10, assign65040_e100315_d_n11, assign65040_e100315_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 != 0.0)) {
        let assign65040_e100310: f64 = (3.0 * 1.414213562373095);
        let assign65040_e100312: f64 = (assign65040_e100310 * locals.var_ty);
        let assign65040_e100313: f64 = (2.0 + assign65040_e100312);
        (assign65040_e100313, (assign65040_e100310 * locals.var_ty_dn0), (assign65040_e100310 * locals.var_ty_dn2), (assign65040_e100310 * locals.var_ty_dn4), (assign65040_e100310 * locals.var_ty_dn5), (assign65040_e100310 * locals.var_ty_dn6), (assign65040_e100310 * locals.var_ty_dn7), (assign65040_e100310 * locals.var_ty_dn8), (assign65040_e100310 * locals.var_ty_dn9), (assign65040_e100310 * locals.var_ty_dn10), (assign65040_e100310 * locals.var_ty_dn11), (assign65040_e100310 * locals.var_ty_dn14),)
    } else {
        (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn4, locals.var_ac41_dn5, locals.var_ac41_dn6, locals.var_ac41_dn7, locals.var_ac41_dn8, locals.var_ac41_dn9, locals.var_ac41_dn10, locals.var_ac41_dn11, locals.var_ac41_dn14,)
    }
};
        locals.var_ac41 = assign65040_e100315;
        locals.var_ac41_dn0 = assign65040_e100315_d_n0;
        locals.var_ac41_dn2 = assign65040_e100315_d_n2;
        locals.var_ac41_dn4 = assign65040_e100315_d_n4;
        locals.var_ac41_dn5 = assign65040_e100315_d_n5;
        locals.var_ac41_dn6 = assign65040_e100315_d_n6;
        locals.var_ac41_dn7 = assign65040_e100315_d_n7;
        locals.var_ac41_dn8 = assign65040_e100315_d_n8;
        locals.var_ac41_dn9 = assign65040_e100315_d_n9;
        locals.var_ac41_dn10 = assign65040_e100315_d_n10;
        locals.var_ac41_dn11 = assign65040_e100315_d_n11;
        locals.var_ac41_dn14 = assign65040_e100315_d_n14;
        locals.var_ac41_rv = 0.0;

        let (assign65050_e100335, assign65050_e100335_d_n0, assign65050_e100335_d_n2, assign65050_e100335_d_n4, assign65050_e100335_d_n5, assign65050_e100335_d_n6, assign65050_e100335_d_n7, assign65050_e100335_d_n8, assign65050_e100335_d_n9, assign65050_e100335_d_n10, assign65050_e100335_d_n11, assign65050_e100335_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 != 0.0)) {
        let assign65050_e100329: f64 = (8.0 * locals.var_ac41);
        let assign65050_e100331: f64 = (assign65050_e100329 * locals.var_ac41);
        let assign65050_e100333: f64 = (assign65050_e100331 * locals.var_ac41);
        (assign65050_e100333, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign65050_e100329 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign65050_e100331 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign65050_e100329 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign65050_e100331 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn4) * locals.var_ac41) + (assign65050_e100329 * locals.var_ac41_dn4)) * locals.var_ac41) + (assign65050_e100331 * locals.var_ac41_dn4)), (((((8.0 * locals.var_ac41_dn5) * locals.var_ac41) + (assign65050_e100329 * locals.var_ac41_dn5)) * locals.var_ac41) + (assign65050_e100331 * locals.var_ac41_dn5)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign65050_e100329 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign65050_e100331 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn7) * locals.var_ac41) + (assign65050_e100329 * locals.var_ac41_dn7)) * locals.var_ac41) + (assign65050_e100331 * locals.var_ac41_dn7)), (((((8.0 * locals.var_ac41_dn8) * locals.var_ac41) + (assign65050_e100329 * locals.var_ac41_dn8)) * locals.var_ac41) + (assign65050_e100331 * locals.var_ac41_dn8)), (((((8.0 * locals.var_ac41_dn9) * locals.var_ac41) + (assign65050_e100329 * locals.var_ac41_dn9)) * locals.var_ac41) + (assign65050_e100331 * locals.var_ac41_dn9)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign65050_e100329 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign65050_e100331 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn11) * locals.var_ac41) + (assign65050_e100329 * locals.var_ac41_dn11)) * locals.var_ac41) + (assign65050_e100331 * locals.var_ac41_dn11)), (((((8.0 * locals.var_ac41_dn14) * locals.var_ac41) + (assign65050_e100329 * locals.var_ac41_dn14)) * locals.var_ac41) + (assign65050_e100331 * locals.var_ac41_dn14)),)
    } else {
        (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn4, locals.var_ac4_dn5, locals.var_ac4_dn6, locals.var_ac4_dn7, locals.var_ac4_dn8, locals.var_ac4_dn9, locals.var_ac4_dn10, locals.var_ac4_dn11, locals.var_ac4_dn14,)
    }
};
        locals.var_ac4 = assign65050_e100335;
        locals.var_ac4_dn0 = assign65050_e100335_d_n0;
        locals.var_ac4_dn2 = assign65050_e100335_d_n2;
        locals.var_ac4_dn4 = assign65050_e100335_d_n4;
        locals.var_ac4_dn5 = assign65050_e100335_d_n5;
        locals.var_ac4_dn6 = assign65050_e100335_d_n6;
        locals.var_ac4_dn7 = assign65050_e100335_d_n7;
        locals.var_ac4_dn8 = assign65050_e100335_d_n8;
        locals.var_ac4_dn9 = assign65050_e100335_d_n9;
        locals.var_ac4_dn10 = assign65050_e100335_d_n10;
        locals.var_ac4_dn11 = assign65050_e100335_d_n11;
        locals.var_ac4_dn14 = assign65050_e100335_d_n14;
        locals.var_ac4_rv = 0.0;

        let (assign65060_e100351, assign65060_e100351_d_n0, assign65060_e100351_d_n2, assign65060_e100351_d_n4, assign65060_e100351_d_n5, assign65060_e100351_d_n6, assign65060_e100351_d_n7, assign65060_e100351_d_n8, assign65060_e100351_d_n9, assign65060_e100351_d_n10, assign65060_e100351_d_n11, assign65060_e100351_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 != 0.0)) {
        let assign65060_e100349: f64 = (locals.var_tx - 2.0);
        (assign65060_e100349, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign65060_e100351;
        locals.var_t4_dn0 = assign65060_e100351_d_n0;
        locals.var_t4_dn2 = assign65060_e100351_d_n2;
        locals.var_t4_dn4 = assign65060_e100351_d_n4;
        locals.var_t4_dn5 = assign65060_e100351_d_n5;
        locals.var_t4_dn6 = assign65060_e100351_d_n6;
        locals.var_t4_dn7 = assign65060_e100351_d_n7;
        locals.var_t4_dn8 = assign65060_e100351_d_n8;
        locals.var_t4_dn9 = assign65060_e100351_d_n9;
        locals.var_t4_dn10 = assign65060_e100351_d_n10;
        locals.var_t4_dn11 = assign65060_e100351_d_n11;
        locals.var_t4_dn14 = assign65060_e100351_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign65070_e100369, assign65070_e100369_d_n0, assign65070_e100369_d_n2, assign65070_e100369_d_n4, assign65070_e100369_d_n5, assign65070_e100369_d_n6, assign65070_e100369_d_n7, assign65070_e100369_d_n8, assign65070_e100369_d_n9, assign65070_e100369_d_n10, assign65070_e100369_d_n11, assign65070_e100369_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 != 0.0)) {
        let assign65070_e100365: f64 = (9.0 * locals.var_ty);
        let assign65070_e100367: f64 = (assign65070_e100365 * locals.var_t4);
        (assign65070_e100367, (((9.0 * locals.var_ty_dn0) * locals.var_t4) + (assign65070_e100365 * locals.var_t4_dn0)), (((9.0 * locals.var_ty_dn2) * locals.var_t4) + (assign65070_e100365 * locals.var_t4_dn2)), (((9.0 * locals.var_ty_dn4) * locals.var_t4) + (assign65070_e100365 * locals.var_t4_dn4)), (((9.0 * locals.var_ty_dn5) * locals.var_t4) + (assign65070_e100365 * locals.var_t4_dn5)), (((9.0 * locals.var_ty_dn6) * locals.var_t4) + (assign65070_e100365 * locals.var_t4_dn6)), (((9.0 * locals.var_ty_dn7) * locals.var_t4) + (assign65070_e100365 * locals.var_t4_dn7)), (((9.0 * locals.var_ty_dn8) * locals.var_t4) + (assign65070_e100365 * locals.var_t4_dn8)), (((9.0 * locals.var_ty_dn9) * locals.var_t4) + (assign65070_e100365 * locals.var_t4_dn9)), (((9.0 * locals.var_ty_dn10) * locals.var_t4) + (assign65070_e100365 * locals.var_t4_dn10)), (((9.0 * locals.var_ty_dn11) * locals.var_t4) + (assign65070_e100365 * locals.var_t4_dn11)), (((9.0 * locals.var_ty_dn14) * locals.var_t4) + (assign65070_e100365 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign65070_e100369;
        locals.var_t5_dn0 = assign65070_e100369_d_n0;
        locals.var_t5_dn2 = assign65070_e100369_d_n2;
        locals.var_t5_dn4 = assign65070_e100369_d_n4;
        locals.var_t5_dn5 = assign65070_e100369_d_n5;
        locals.var_t5_dn6 = assign65070_e100369_d_n6;
        locals.var_t5_dn7 = assign65070_e100369_d_n7;
        locals.var_t5_dn8 = assign65070_e100369_d_n8;
        locals.var_t5_dn9 = assign65070_e100369_d_n9;
        locals.var_t5_dn10 = assign65070_e100369_d_n10;
        locals.var_t5_dn11 = assign65070_e100369_d_n11;
        locals.var_t5_dn14 = assign65070_e100369_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign65080_e100387, assign65080_e100387_d_n0, assign65080_e100387_d_n2, assign65080_e100387_d_n4, assign65080_e100387_d_n5, assign65080_e100387_d_n6, assign65080_e100387_d_n7, assign65080_e100387_d_n8, assign65080_e100387_d_n9, assign65080_e100387_d_n10, assign65080_e100387_d_n11, assign65080_e100387_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 != 0.0)) {
        let assign65080_e100383: f64 = (7.0 * 1.414213562373095);
        let assign65080_e100385: f64 = (assign65080_e100383 - locals.var_t5);
        (assign65080_e100385, (-locals.var_t5_dn0), (-locals.var_t5_dn2), (-locals.var_t5_dn4), (-locals.var_t5_dn5), (-locals.var_t5_dn6), (-locals.var_t5_dn7), (-locals.var_t5_dn8), (-locals.var_t5_dn9), (-locals.var_t5_dn10), (-locals.var_t5_dn11), (-locals.var_t5_dn14),)
    } else {
        (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn4, locals.var_ac31_dn5, locals.var_ac31_dn6, locals.var_ac31_dn7, locals.var_ac31_dn8, locals.var_ac31_dn9, locals.var_ac31_dn10, locals.var_ac31_dn11, locals.var_ac31_dn14,)
    }
};
        locals.var_ac31 = assign65080_e100387;
        locals.var_ac31_dn0 = assign65080_e100387_d_n0;
        locals.var_ac31_dn2 = assign65080_e100387_d_n2;
        locals.var_ac31_dn4 = assign65080_e100387_d_n4;
        locals.var_ac31_dn5 = assign65080_e100387_d_n5;
        locals.var_ac31_dn6 = assign65080_e100387_d_n6;
        locals.var_ac31_dn7 = assign65080_e100387_d_n7;
        locals.var_ac31_dn8 = assign65080_e100387_d_n8;
        locals.var_ac31_dn9 = assign65080_e100387_d_n9;
        locals.var_ac31_dn10 = assign65080_e100387_d_n10;
        locals.var_ac31_dn11 = assign65080_e100387_d_n11;
        locals.var_ac31_dn14 = assign65080_e100387_d_n14;
        locals.var_ac31_rv = 0.0;

        let (assign65090_e100403, assign65090_e100403_d_n0, assign65090_e100403_d_n2, assign65090_e100403_d_n4, assign65090_e100403_d_n5, assign65090_e100403_d_n6, assign65090_e100403_d_n7, assign65090_e100403_d_n8, assign65090_e100403_d_n9, assign65090_e100403_d_n10, assign65090_e100403_d_n11, assign65090_e100403_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 != 0.0)) {
        let assign65090_e100401: f64 = (locals.var_ac31 * locals.var_ac31);
        (assign65090_e100401, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn4 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn4)), ((locals.var_ac31_dn5 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn5)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn7 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn7)), ((locals.var_ac31_dn8 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn8)), ((locals.var_ac31_dn9 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn9)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn11 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn11)), ((locals.var_ac31_dn14 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn14)),)
    } else {
        (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn4, locals.var_ac3_dn5, locals.var_ac3_dn6, locals.var_ac3_dn7, locals.var_ac3_dn8, locals.var_ac3_dn9, locals.var_ac3_dn10, locals.var_ac3_dn11, locals.var_ac3_dn14,)
    }
};
        locals.var_ac3 = assign65090_e100403;
        locals.var_ac3_dn0 = assign65090_e100403_d_n0;
        locals.var_ac3_dn2 = assign65090_e100403_d_n2;
        locals.var_ac3_dn4 = assign65090_e100403_d_n4;
        locals.var_ac3_dn5 = assign65090_e100403_d_n5;
        locals.var_ac3_dn6 = assign65090_e100403_d_n6;
        locals.var_ac3_dn7 = assign65090_e100403_d_n7;
        locals.var_ac3_dn8 = assign65090_e100403_d_n8;
        locals.var_ac3_dn9 = assign65090_e100403_d_n9;
        locals.var_ac3_dn10 = assign65090_e100403_d_n10;
        locals.var_ac3_dn11 = assign65090_e100403_d_n11;
        locals.var_ac3_dn14 = assign65090_e100403_d_n14;
        locals.var_ac3_rv = 0.0;

        let assign65100_e100407: f64 = (locals.var_ac3 * 1e-8);
        let assign65100_e100408: f64 = if locals.var_ac4 < assign65100_e100407 { 1.0 } else { 0.0 };
        locals.var_guard1559 = assign65100_e100408;
        locals.var_guard1559_rv = 0.0;

        let (assign65110_e100437, assign65110_e100437_d_n0, assign65110_e100437_d_n2, assign65110_e100437_d_n4, assign65110_e100437_d_n5, assign65110_e100437_d_n6, assign65110_e100437_d_n7, assign65110_e100437_d_n8, assign65110_e100437_d_n9, assign65110_e100437_d_n10, assign65110_e100437_d_n11, assign65110_e100437_d_n14,) = {
    if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 != 0.0)) && (locals.var_guard1559 != 0.0)) {
        let assign65110_e100423: f64 = (-7.0);
        let assign65110_e100425: f64 = (assign65110_e100423 * 1.414213562373095);
        let assign65110_e100427: f64 = (assign65110_e100425 + locals.var_ac31);
        let assign65110_e100430: f64 = (0.5 * locals.var_ac4);
        let assign65110_e100432: f64 = (assign65110_e100430 / locals.var_ac31);
        let assign65110_e100433: f64 = (assign65110_e100427 + assign65110_e100432);
        let assign65110_e100435: f64 = (assign65110_e100433 + locals.var_t5);
        (assign65110_e100435, ((locals.var_ac31_dn0 + ((((0.5 * locals.var_ac4_dn0) * locals.var_ac31) - (assign65110_e100430 * locals.var_ac31_dn0)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn0), ((locals.var_ac31_dn2 + ((((0.5 * locals.var_ac4_dn2) * locals.var_ac31) - (assign65110_e100430 * locals.var_ac31_dn2)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn2), ((locals.var_ac31_dn4 + ((((0.5 * locals.var_ac4_dn4) * locals.var_ac31) - (assign65110_e100430 * locals.var_ac31_dn4)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn4), ((locals.var_ac31_dn5 + ((((0.5 * locals.var_ac4_dn5) * locals.var_ac31) - (assign65110_e100430 * locals.var_ac31_dn5)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn5), ((locals.var_ac31_dn6 + ((((0.5 * locals.var_ac4_dn6) * locals.var_ac31) - (assign65110_e100430 * locals.var_ac31_dn6)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn6), ((locals.var_ac31_dn7 + ((((0.5 * locals.var_ac4_dn7) * locals.var_ac31) - (assign65110_e100430 * locals.var_ac31_dn7)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn7), ((locals.var_ac31_dn8 + ((((0.5 * locals.var_ac4_dn8) * locals.var_ac31) - (assign65110_e100430 * locals.var_ac31_dn8)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn8), ((locals.var_ac31_dn9 + ((((0.5 * locals.var_ac4_dn9) * locals.var_ac31) - (assign65110_e100430 * locals.var_ac31_dn9)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn9), ((locals.var_ac31_dn10 + ((((0.5 * locals.var_ac4_dn10) * locals.var_ac31) - (assign65110_e100430 * locals.var_ac31_dn10)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn10), ((locals.var_ac31_dn11 + ((((0.5 * locals.var_ac4_dn11) * locals.var_ac31) - (assign65110_e100430 * locals.var_ac31_dn11)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn11), ((locals.var_ac31_dn14 + ((((0.5 * locals.var_ac4_dn14) * locals.var_ac31) - (assign65110_e100430 * locals.var_ac31_dn14)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn14),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn14,)
    }
};
        locals.var_ac1 = assign65110_e100437;
        locals.var_ac1_dn0 = assign65110_e100437_d_n0;
        locals.var_ac1_dn2 = assign65110_e100437_d_n2;
        locals.var_ac1_dn4 = assign65110_e100437_d_n4;
        locals.var_ac1_dn5 = assign65110_e100437_d_n5;
        locals.var_ac1_dn6 = assign65110_e100437_d_n6;
        locals.var_ac1_dn7 = assign65110_e100437_d_n7;
        locals.var_ac1_dn8 = assign65110_e100437_d_n8;
        locals.var_ac1_dn9 = assign65110_e100437_d_n9;
        locals.var_ac1_dn10 = assign65110_e100437_d_n10;
        locals.var_ac1_dn11 = assign65110_e100437_d_n11;
        locals.var_ac1_dn14 = assign65110_e100437_d_n14;
        locals.var_ac1_rv = 0.0;

        let (assign65120_e100457, assign65120_e100457_d_n0, assign65120_e100457_d_n2, assign65120_e100457_d_n4, assign65120_e100457_d_n5, assign65120_e100457_d_n6, assign65120_e100457_d_n7, assign65120_e100457_d_n8, assign65120_e100457_d_n9, assign65120_e100457_d_n10, assign65120_e100457_d_n11, assign65120_e100457_d_n14,) = {
    if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 != 0.0)) && (locals.var_guard1559 == 0.0)) {
        let assign65120_e100454: f64 = (locals.var_ac4 + locals.var_ac3);
        let assign65120_e100455: f64 = (assign65120_e100454).sqrt();
        (assign65120_e100455, ((locals.var_ac4_dn0 + locals.var_ac3_dn0) / (2.0 * assign65120_e100455)), ((locals.var_ac4_dn2 + locals.var_ac3_dn2) / (2.0 * assign65120_e100455)), ((locals.var_ac4_dn4 + locals.var_ac3_dn4) / (2.0 * assign65120_e100455)), ((locals.var_ac4_dn5 + locals.var_ac3_dn5) / (2.0 * assign65120_e100455)), ((locals.var_ac4_dn6 + locals.var_ac3_dn6) / (2.0 * assign65120_e100455)), ((locals.var_ac4_dn7 + locals.var_ac3_dn7) / (2.0 * assign65120_e100455)), ((locals.var_ac4_dn8 + locals.var_ac3_dn8) / (2.0 * assign65120_e100455)), ((locals.var_ac4_dn9 + locals.var_ac3_dn9) / (2.0 * assign65120_e100455)), ((locals.var_ac4_dn10 + locals.var_ac3_dn10) / (2.0 * assign65120_e100455)), ((locals.var_ac4_dn11 + locals.var_ac3_dn11) / (2.0 * assign65120_e100455)), ((locals.var_ac4_dn14 + locals.var_ac3_dn14) / (2.0 * assign65120_e100455)),)
    } else {
        (locals.var_ac2, locals.var_ac2_dn0, locals.var_ac2_dn2, locals.var_ac2_dn4, locals.var_ac2_dn5, locals.var_ac2_dn6, locals.var_ac2_dn7, locals.var_ac2_dn8, locals.var_ac2_dn9, locals.var_ac2_dn10, locals.var_ac2_dn11, locals.var_ac2_dn14,)
    }
};
        locals.var_ac2 = assign65120_e100457;
        locals.var_ac2_dn0 = assign65120_e100457_d_n0;
        locals.var_ac2_dn2 = assign65120_e100457_d_n2;
        locals.var_ac2_dn4 = assign65120_e100457_d_n4;
        locals.var_ac2_dn5 = assign65120_e100457_d_n5;
        locals.var_ac2_dn6 = assign65120_e100457_d_n6;
        locals.var_ac2_dn7 = assign65120_e100457_d_n7;
        locals.var_ac2_dn8 = assign65120_e100457_d_n8;
        locals.var_ac2_dn9 = assign65120_e100457_d_n9;
        locals.var_ac2_dn10 = assign65120_e100457_d_n10;
        locals.var_ac2_dn11 = assign65120_e100457_d_n11;
        locals.var_ac2_dn14 = assign65120_e100457_d_n14;
        locals.var_ac2_rv = 0.0;

        let (assign65130_e100481, assign65130_e100481_d_n0, assign65130_e100481_d_n2, assign65130_e100481_d_n4, assign65130_e100481_d_n5, assign65130_e100481_d_n6, assign65130_e100481_d_n7, assign65130_e100481_d_n8, assign65130_e100481_d_n9, assign65130_e100481_d_n10, assign65130_e100481_d_n11, assign65130_e100481_d_n14,) = {
    if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 != 0.0)) && (locals.var_guard1559 == 0.0)) {
        let assign65130_e100473: f64 = (-7.0);
        let assign65130_e100475: f64 = (assign65130_e100473 * 1.414213562373095);
        let assign65130_e100477: f64 = (assign65130_e100475 + locals.var_ac2);
        let assign65130_e100479: f64 = (assign65130_e100477 + locals.var_t5);
        (assign65130_e100479, (locals.var_ac2_dn0 + locals.var_t5_dn0), (locals.var_ac2_dn2 + locals.var_t5_dn2), (locals.var_ac2_dn4 + locals.var_t5_dn4), (locals.var_ac2_dn5 + locals.var_t5_dn5), (locals.var_ac2_dn6 + locals.var_t5_dn6), (locals.var_ac2_dn7 + locals.var_t5_dn7), (locals.var_ac2_dn8 + locals.var_t5_dn8), (locals.var_ac2_dn9 + locals.var_t5_dn9), (locals.var_ac2_dn10 + locals.var_t5_dn10), (locals.var_ac2_dn11 + locals.var_t5_dn11), (locals.var_ac2_dn14 + locals.var_t5_dn14),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn14,)
    }
};
        locals.var_ac1 = assign65130_e100481;
        locals.var_ac1_dn0 = assign65130_e100481_d_n0;
        locals.var_ac1_dn2 = assign65130_e100481_d_n2;
        locals.var_ac1_dn4 = assign65130_e100481_d_n4;
        locals.var_ac1_dn5 = assign65130_e100481_d_n5;
        locals.var_ac1_dn6 = assign65130_e100481_d_n6;
        locals.var_ac1_dn7 = assign65130_e100481_d_n7;
        locals.var_ac1_dn8 = assign65130_e100481_d_n8;
        locals.var_ac1_dn9 = assign65130_e100481_d_n9;
        locals.var_ac1_dn10 = assign65130_e100481_d_n10;
        locals.var_ac1_dn11 = assign65130_e100481_d_n11;
        locals.var_ac1_dn14 = assign65130_e100481_d_n14;
        locals.var_ac1_rv = 0.0;

        let (assign65140_e100502, assign65140_e100502_d_n0, assign65140_e100502_d_n2, assign65140_e100502_d_n4, assign65140_e100502_d_n5, assign65140_e100502_d_n6, assign65140_e100502_d_n7, assign65140_e100502_d_n8, assign65140_e100502_d_n9, assign65140_e100502_d_n10, assign65140_e100502_d_n11, assign65140_e100502_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 != 0.0)) {
        let (assign65140_e100500, assign65140_e100500_d_n0, assign65140_e100500_d_n2, assign65140_e100500_d_n4, assign65140_e100500_d_n5, assign65140_e100500_d_n6, assign65140_e100500_d_n7, assign65140_e100500_d_n8, assign65140_e100500_d_n9, assign65140_e100500_d_n10, assign65140_e100500_d_n11, assign65140_e100500_d_n14,) = {
            if (locals.var_ac1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign65140_e100499: f64 = (locals.var_ac1).powf(0.3333333333333333);
                (assign65140_e100499, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn0)) } } else { (assign65140_e100499 * (0.3333333333333333 * (locals.var_ac1_dn0 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn2)) } } else { (assign65140_e100499 * (0.3333333333333333 * (locals.var_ac1_dn2 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn4)) } } else { (assign65140_e100499 * (0.3333333333333333 * (locals.var_ac1_dn4 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn5)) } } else { (assign65140_e100499 * (0.3333333333333333 * (locals.var_ac1_dn5 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn6)) } } else { (assign65140_e100499 * (0.3333333333333333 * (locals.var_ac1_dn6 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn7)) } } else { (assign65140_e100499 * (0.3333333333333333 * (locals.var_ac1_dn7 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn8)) } } else { (assign65140_e100499 * (0.3333333333333333 * (locals.var_ac1_dn8 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn9)) } } else { (assign65140_e100499 * (0.3333333333333333 * (locals.var_ac1_dn9 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn10)) } } else { (assign65140_e100499 * (0.3333333333333333 * (locals.var_ac1_dn10 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn11)) } } else { (assign65140_e100499 * (0.3333333333333333 * (locals.var_ac1_dn11 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn14)) } } else { (assign65140_e100499 * (0.3333333333333333 * (locals.var_ac1_dn14 / locals.var_ac1))) },)
            }
        };
        (assign65140_e100500, assign65140_e100500_d_n0, assign65140_e100500_d_n2, assign65140_e100500_d_n4, assign65140_e100500_d_n5, assign65140_e100500_d_n6, assign65140_e100500_d_n7, assign65140_e100500_d_n8, assign65140_e100500_d_n9, assign65140_e100500_d_n10, assign65140_e100500_d_n11, assign65140_e100500_d_n14,)
    } else {
        (locals.var_acd, locals.var_acd_dn0, locals.var_acd_dn2, locals.var_acd_dn4, locals.var_acd_dn5, locals.var_acd_dn6, locals.var_acd_dn7, locals.var_acd_dn8, locals.var_acd_dn9, locals.var_acd_dn10, locals.var_acd_dn11, locals.var_acd_dn14,)
    }
};
        locals.var_acd = assign65140_e100502;
        locals.var_acd_dn0 = assign65140_e100502_d_n0;
        locals.var_acd_dn2 = assign65140_e100502_d_n2;
        locals.var_acd_dn4 = assign65140_e100502_d_n4;
        locals.var_acd_dn5 = assign65140_e100502_d_n5;
        locals.var_acd_dn6 = assign65140_e100502_d_n6;
        locals.var_acd_dn7 = assign65140_e100502_d_n7;
        locals.var_acd_dn8 = assign65140_e100502_d_n8;
        locals.var_acd_dn9 = assign65140_e100502_d_n9;
        locals.var_acd_dn10 = assign65140_e100502_d_n10;
        locals.var_acd_dn11 = assign65140_e100502_d_n11;
        locals.var_acd_dn14 = assign65140_e100502_d_n14;
        locals.var_acd_rv = 0.0;

        let (assign65150_e100533, assign65150_e100533_d_n0, assign65150_e100533_d_n2, assign65150_e100533_d_n4, assign65150_e100533_d_n5, assign65150_e100533_d_n6, assign65150_e100533_d_n7, assign65150_e100533_d_n8, assign65150_e100533_d_n9, assign65150_e100533_d_n10, assign65150_e100533_d_n11, assign65150_e100533_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 != 0.0)) {
        let assign65150_e100515: f64 = (-4.0);
        let assign65150_e100517: f64 = (assign65150_e100515 * 1.414213562373095);
        let assign65150_e100520: f64 = (12.0 * locals.var_ty);
        let assign65150_e100521: f64 = (assign65150_e100517 - assign65150_e100520);
        let assign65150_e100524: f64 = (2.0 * locals.var_acd);
        let assign65150_e100525: f64 = (assign65150_e100521 + assign65150_e100524);
        let assign65150_e100528: f64 = (1.414213562373095 * locals.var_acd);
        let assign65150_e100530: f64 = (assign65150_e100528 * locals.var_acd);
        let assign65150_e100531: f64 = (assign65150_e100525 + assign65150_e100530);
        (assign65150_e100531, (((-(12.0 * locals.var_ty_dn0)) + (2.0 * locals.var_acd_dn0)) + (((1.414213562373095 * locals.var_acd_dn0) * locals.var_acd) + (assign65150_e100528 * locals.var_acd_dn0))), (((-(12.0 * locals.var_ty_dn2)) + (2.0 * locals.var_acd_dn2)) + (((1.414213562373095 * locals.var_acd_dn2) * locals.var_acd) + (assign65150_e100528 * locals.var_acd_dn2))), (((-(12.0 * locals.var_ty_dn4)) + (2.0 * locals.var_acd_dn4)) + (((1.414213562373095 * locals.var_acd_dn4) * locals.var_acd) + (assign65150_e100528 * locals.var_acd_dn4))), (((-(12.0 * locals.var_ty_dn5)) + (2.0 * locals.var_acd_dn5)) + (((1.414213562373095 * locals.var_acd_dn5) * locals.var_acd) + (assign65150_e100528 * locals.var_acd_dn5))), (((-(12.0 * locals.var_ty_dn6)) + (2.0 * locals.var_acd_dn6)) + (((1.414213562373095 * locals.var_acd_dn6) * locals.var_acd) + (assign65150_e100528 * locals.var_acd_dn6))), (((-(12.0 * locals.var_ty_dn7)) + (2.0 * locals.var_acd_dn7)) + (((1.414213562373095 * locals.var_acd_dn7) * locals.var_acd) + (assign65150_e100528 * locals.var_acd_dn7))), (((-(12.0 * locals.var_ty_dn8)) + (2.0 * locals.var_acd_dn8)) + (((1.414213562373095 * locals.var_acd_dn8) * locals.var_acd) + (assign65150_e100528 * locals.var_acd_dn8))), (((-(12.0 * locals.var_ty_dn9)) + (2.0 * locals.var_acd_dn9)) + (((1.414213562373095 * locals.var_acd_dn9) * locals.var_acd) + (assign65150_e100528 * locals.var_acd_dn9))), (((-(12.0 * locals.var_ty_dn10)) + (2.0 * locals.var_acd_dn10)) + (((1.414213562373095 * locals.var_acd_dn10) * locals.var_acd) + (assign65150_e100528 * locals.var_acd_dn10))), (((-(12.0 * locals.var_ty_dn11)) + (2.0 * locals.var_acd_dn11)) + (((1.414213562373095 * locals.var_acd_dn11) * locals.var_acd) + (assign65150_e100528 * locals.var_acd_dn11))), (((-(12.0 * locals.var_ty_dn14)) + (2.0 * locals.var_acd_dn14)) + (((1.414213562373095 * locals.var_acd_dn14) * locals.var_acd) + (assign65150_e100528 * locals.var_acd_dn14))),)
    } else {
        (locals.var_acn, locals.var_acn_dn0, locals.var_acn_dn2, locals.var_acn_dn4, locals.var_acn_dn5, locals.var_acn_dn6, locals.var_acn_dn7, locals.var_acn_dn8, locals.var_acn_dn9, locals.var_acn_dn10, locals.var_acn_dn11, locals.var_acn_dn14,)
    }
};
        locals.var_acn = assign65150_e100533;
        locals.var_acn_dn0 = assign65150_e100533_d_n0;
        locals.var_acn_dn2 = assign65150_e100533_d_n2;
        locals.var_acn_dn4 = assign65150_e100533_d_n4;
        locals.var_acn_dn5 = assign65150_e100533_d_n5;
        locals.var_acn_dn6 = assign65150_e100533_d_n6;
        locals.var_acn_dn7 = assign65150_e100533_d_n7;
        locals.var_acn_dn8 = assign65150_e100533_d_n8;
        locals.var_acn_dn9 = assign65150_e100533_d_n9;
        locals.var_acn_dn10 = assign65150_e100533_d_n10;
        locals.var_acn_dn11 = assign65150_e100533_d_n11;
        locals.var_acn_dn14 = assign65150_e100533_d_n14;
        locals.var_acn_rv = 0.0;

        let (assign65160_e100549, assign65160_e100549_d_n0, assign65160_e100549_d_n2, assign65160_e100549_d_n4, assign65160_e100549_d_n5, assign65160_e100549_d_n6, assign65160_e100549_d_n7, assign65160_e100549_d_n8, assign65160_e100549_d_n9, assign65160_e100549_d_n10, assign65160_e100549_d_n11, assign65160_e100549_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 != 0.0)) {
        let assign65160_e100547: f64 = (1.0 / locals.var_acd);
        (assign65160_e100547, (-(locals.var_acd_dn0 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn2 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn4 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn5 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn6 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn7 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn8 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn9 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn10 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn11 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn14 / (locals.var_acd * locals.var_acd))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign65160_e100549;
        locals.var_t1_dn0 = assign65160_e100549_d_n0;
        locals.var_t1_dn2 = assign65160_e100549_d_n2;
        locals.var_t1_dn4 = assign65160_e100549_d_n4;
        locals.var_t1_dn5 = assign65160_e100549_d_n5;
        locals.var_t1_dn6 = assign65160_e100549_d_n6;
        locals.var_t1_dn7 = assign65160_e100549_d_n7;
        locals.var_t1_dn8 = assign65160_e100549_d_n8;
        locals.var_t1_dn9 = assign65160_e100549_d_n9;
        locals.var_t1_dn10 = assign65160_e100549_d_n10;
        locals.var_t1_dn11 = assign65160_e100549_d_n11;
        locals.var_t1_dn14 = assign65160_e100549_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign65170_e100565, assign65170_e100565_d_n0, assign65170_e100565_d_n2, assign65170_e100565_d_n4, assign65170_e100565_d_n5, assign65170_e100565_d_n6, assign65170_e100565_d_n7, assign65170_e100565_d_n8, assign65170_e100565_d_n9, assign65170_e100565_d_n10, assign65170_e100565_d_n11, assign65170_e100565_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 != 0.0)) {
        let assign65170_e100563: f64 = (locals.var_acn * locals.var_t1);
        (assign65170_e100563, ((locals.var_acn_dn0 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn0)), ((locals.var_acn_dn2 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn2)), ((locals.var_acn_dn4 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn4)), ((locals.var_acn_dn5 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn5)), ((locals.var_acn_dn6 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn6)), ((locals.var_acn_dn7 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn7)), ((locals.var_acn_dn8 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn8)), ((locals.var_acn_dn9 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn9)), ((locals.var_acn_dn10 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn10)), ((locals.var_acn_dn11 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn11)), ((locals.var_acn_dn14 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn14)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign65170_e100565;
        locals.var_chi_dn0 = assign65170_e100565_d_n0;
        locals.var_chi_dn2 = assign65170_e100565_d_n2;
        locals.var_chi_dn4 = assign65170_e100565_d_n4;
        locals.var_chi_dn5 = assign65170_e100565_d_n5;
        locals.var_chi_dn6 = assign65170_e100565_d_n6;
        locals.var_chi_dn7 = assign65170_e100565_d_n7;
        locals.var_chi_dn8 = assign65170_e100565_d_n8;
        locals.var_chi_dn9 = assign65170_e100565_d_n9;
        locals.var_chi_dn10 = assign65170_e100565_d_n10;
        locals.var_chi_dn11 = assign65170_e100565_d_n11;
        locals.var_chi_dn14 = assign65170_e100565_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign65180_e100583, assign65180_e100583_d_n0, assign65180_e100583_d_n2, assign65180_e100583_d_n4, assign65180_e100583_d_n5, assign65180_e100583_d_n6, assign65180_e100583_d_n7, assign65180_e100583_d_n8, assign65180_e100583_d_n9, assign65180_e100583_d_n10, assign65180_e100583_d_n11, assign65180_e100583_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 != 0.0)) {
        let assign65180_e100579: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign65180_e100581: f64 = (assign65180_e100579 + locals.var_vbscl__blk1549);
        (assign65180_e100581, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) + locals.var_vbscl__blk1549_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) + locals.var_vbscl__blk1549_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) + locals.var_vbscl__blk1549_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) + locals.var_vbscl__blk1549_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) + locals.var_vbscl__blk1549_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) + locals.var_vbscl__blk1549_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) + locals.var_vbscl__blk1549_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) + locals.var_vbscl__blk1549_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) + locals.var_vbscl__blk1549_dn10), (((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)) + locals.var_vbscl__blk1549_dn11), (((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)) + locals.var_vbscl__blk1549_dn14),)
    } else {
        (locals.var_psa, locals.var_psa_dn0, locals.var_psa_dn2, locals.var_psa_dn4, locals.var_psa_dn5, locals.var_psa_dn6, locals.var_psa_dn7, locals.var_psa_dn8, locals.var_psa_dn9, locals.var_psa_dn10, locals.var_psa_dn11, locals.var_psa_dn14,)
    }
};
        locals.var_psa = assign65180_e100583;
        locals.var_psa_dn0 = assign65180_e100583_d_n0;
        locals.var_psa_dn2 = assign65180_e100583_d_n2;
        locals.var_psa_dn4 = assign65180_e100583_d_n4;
        locals.var_psa_dn5 = assign65180_e100583_d_n5;
        locals.var_psa_dn6 = assign65180_e100583_d_n6;
        locals.var_psa_dn7 = assign65180_e100583_d_n7;
        locals.var_psa_dn8 = assign65180_e100583_d_n8;
        locals.var_psa_dn9 = assign65180_e100583_d_n9;
        locals.var_psa_dn10 = assign65180_e100583_d_n10;
        locals.var_psa_dn11 = assign65180_e100583_d_n11;
        locals.var_psa_dn14 = assign65180_e100583_d_n14;
        locals.var_psa_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_244(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign65190_e100599, assign65190_e100599_d_n0, assign65190_e100599_d_n2, assign65190_e100599_d_n4, assign65190_e100599_d_n5, assign65190_e100599_d_n6, assign65190_e100599_d_n7, assign65190_e100599_d_n8, assign65190_e100599_d_n9, assign65190_e100599_d_n10, assign65190_e100599_d_n11, assign65190_e100599_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 != 0.0)) {
        let assign65190_e100597: f64 = (locals.var_psa - locals.var_vbscl__blk1549);
        (assign65190_e100597, (locals.var_psa_dn0 - locals.var_vbscl__blk1549_dn0), (locals.var_psa_dn2 - locals.var_vbscl__blk1549_dn2), (locals.var_psa_dn4 - locals.var_vbscl__blk1549_dn4), (locals.var_psa_dn5 - locals.var_vbscl__blk1549_dn5), (locals.var_psa_dn6 - locals.var_vbscl__blk1549_dn6), (locals.var_psa_dn7 - locals.var_vbscl__blk1549_dn7), (locals.var_psa_dn8 - locals.var_vbscl__blk1549_dn8), (locals.var_psa_dn9 - locals.var_vbscl__blk1549_dn9), (locals.var_psa_dn10 - locals.var_vbscl__blk1549_dn10), (locals.var_psa_dn11 - locals.var_vbscl__blk1549_dn11), (locals.var_psa_dn14 - locals.var_vbscl__blk1549_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign65190_e100599;
        locals.var_t1_dn0 = assign65190_e100599_d_n0;
        locals.var_t1_dn2 = assign65190_e100599_d_n2;
        locals.var_t1_dn4 = assign65190_e100599_d_n4;
        locals.var_t1_dn5 = assign65190_e100599_d_n5;
        locals.var_t1_dn6 = assign65190_e100599_d_n6;
        locals.var_t1_dn7 = assign65190_e100599_d_n7;
        locals.var_t1_dn8 = assign65190_e100599_d_n8;
        locals.var_t1_dn9 = assign65190_e100599_d_n9;
        locals.var_t1_dn10 = assign65190_e100599_d_n10;
        locals.var_t1_dn11 = assign65190_e100599_d_n11;
        locals.var_t1_dn14 = assign65190_e100599_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign65200_e100615, assign65200_e100615_d_n0, assign65200_e100615_d_n2, assign65200_e100615_d_n4, assign65200_e100615_d_n5, assign65200_e100615_d_n6, assign65200_e100615_d_n7, assign65200_e100615_d_n8, assign65200_e100615_d_n9, assign65200_e100615_d_n10, assign65200_e100615_d_n11, assign65200_e100615_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 != 0.0)) {
        let assign65200_e100613: f64 = (locals.var_t1 / locals.var_ps0_min);
        (assign65200_e100613, (((locals.var_t1_dn0 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn0)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn2 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn2)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn4 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn4)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn5 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn5)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn6 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn6)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn7 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn7)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn8 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn8)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn9 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn9)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn10 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn10)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn11 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn11)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn14 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn14)) / (locals.var_ps0_min * locals.var_ps0_min)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign65200_e100615;
        locals.var_t2_dn0 = assign65200_e100615_d_n0;
        locals.var_t2_dn2 = assign65200_e100615_d_n2;
        locals.var_t2_dn4 = assign65200_e100615_d_n4;
        locals.var_t2_dn5 = assign65200_e100615_d_n5;
        locals.var_t2_dn6 = assign65200_e100615_d_n6;
        locals.var_t2_dn7 = assign65200_e100615_d_n7;
        locals.var_t2_dn8 = assign65200_e100615_d_n8;
        locals.var_t2_dn9 = assign65200_e100615_d_n9;
        locals.var_t2_dn10 = assign65200_e100615_d_n10;
        locals.var_t2_dn11 = assign65200_e100615_d_n11;
        locals.var_t2_dn14 = assign65200_e100615_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign65210_e100634, assign65210_e100634_d_n0, assign65210_e100634_d_n2, assign65210_e100634_d_n4, assign65210_e100634_d_n5, assign65210_e100634_d_n6, assign65210_e100634_d_n7, assign65210_e100634_d_n8, assign65210_e100634_d_n9, assign65210_e100634_d_n10, assign65210_e100634_d_n11, assign65210_e100634_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 != 0.0)) {
        let assign65210_e100630: f64 = (locals.var_t2 * locals.var_t2);
        let assign65210_e100631: f64 = (1.0 + assign65210_e100630);
        let assign65210_e100632: f64 = (assign65210_e100631).sqrt();
        (assign65210_e100632, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign65210_e100632)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign65210_e100632)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign65210_e100632)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign65210_e100632)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign65210_e100632)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign65210_e100632)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign65210_e100632)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign65210_e100632)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign65210_e100632)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign65210_e100632)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign65210_e100632)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign65210_e100634;
        locals.var_t3_dn0 = assign65210_e100634_d_n0;
        locals.var_t3_dn2 = assign65210_e100634_d_n2;
        locals.var_t3_dn4 = assign65210_e100634_d_n4;
        locals.var_t3_dn5 = assign65210_e100634_d_n5;
        locals.var_t3_dn6 = assign65210_e100634_d_n6;
        locals.var_t3_dn7 = assign65210_e100634_d_n7;
        locals.var_t3_dn8 = assign65210_e100634_d_n8;
        locals.var_t3_dn9 = assign65210_e100634_d_n9;
        locals.var_t3_dn10 = assign65210_e100634_d_n10;
        locals.var_t3_dn11 = assign65210_e100634_d_n11;
        locals.var_t3_dn14 = assign65210_e100634_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign65220_e100652, assign65220_e100652_d_n0, assign65220_e100652_d_n2, assign65220_e100652_d_n4, assign65220_e100652_d_n5, assign65220_e100652_d_n6, assign65220_e100652_d_n7, assign65220_e100652_d_n8, assign65220_e100652_d_n9, assign65220_e100652_d_n10, assign65220_e100652_d_n11, assign65220_e100652_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 != 0.0)) {
        let assign65220_e100648: f64 = (locals.var_t1 / locals.var_t3);
        let assign65220_e100650: f64 = (assign65220_e100648 + locals.var_vbscl__blk1549);
        (assign65220_e100650, ((((locals.var_t1_dn0 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1549_dn0), ((((locals.var_t1_dn2 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1549_dn2), ((((locals.var_t1_dn4 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1549_dn4), ((((locals.var_t1_dn5 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1549_dn5), ((((locals.var_t1_dn6 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1549_dn6), ((((locals.var_t1_dn7 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1549_dn7), ((((locals.var_t1_dn8 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1549_dn8), ((((locals.var_t1_dn9 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1549_dn9), ((((locals.var_t1_dn10 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1549_dn10), ((((locals.var_t1_dn11 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1549_dn11), ((((locals.var_t1_dn14 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn14)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1549_dn14),)
    } else {
        (locals.var_ps0__blk1527, locals.var_ps0__blk1527_dn0, locals.var_ps0__blk1527_dn2, locals.var_ps0__blk1527_dn4, locals.var_ps0__blk1527_dn5, locals.var_ps0__blk1527_dn6, locals.var_ps0__blk1527_dn7, locals.var_ps0__blk1527_dn8, locals.var_ps0__blk1527_dn9, locals.var_ps0__blk1527_dn10, locals.var_ps0__blk1527_dn11, locals.var_ps0__blk1527_dn14,)
    }
};
        locals.var_ps0__blk1527 = assign65220_e100652;
        locals.var_ps0__blk1527_dn0 = assign65220_e100652_d_n0;
        locals.var_ps0__blk1527_dn2 = assign65220_e100652_d_n2;
        locals.var_ps0__blk1527_dn4 = assign65220_e100652_d_n4;
        locals.var_ps0__blk1527_dn5 = assign65220_e100652_d_n5;
        locals.var_ps0__blk1527_dn6 = assign65220_e100652_d_n6;
        locals.var_ps0__blk1527_dn7 = assign65220_e100652_d_n7;
        locals.var_ps0__blk1527_dn8 = assign65220_e100652_d_n8;
        locals.var_ps0__blk1527_dn9 = assign65220_e100652_d_n9;
        locals.var_ps0__blk1527_dn10 = assign65220_e100652_d_n10;
        locals.var_ps0__blk1527_dn11 = assign65220_e100652_d_n11;
        locals.var_ps0__blk1527_dn14 = assign65220_e100652_d_n14;
        locals.var_ps0__blk1527_rv = 0.0;

        let (assign65230_e100672, assign65230_e100672_d_n0, assign65230_e100672_d_n2, assign65230_e100672_d_n4, assign65230_e100672_d_n5, assign65230_e100672_d_n6, assign65230_e100672_d_n7, assign65230_e100672_d_n8, assign65230_e100672_d_n9, assign65230_e100672_d_n10, assign65230_e100672_d_n11, assign65230_e100672_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) {
        let assign65230_e100668: f64 = (locals.var_vbscl__blk1549 - p.p456);
        let assign65230_e100669: f64 = (locals.var_beta * assign65230_e100668);
        let assign65230_e100670: f64 = (assign65230_e100669).exp();
        (assign65230_e100670, (assign65230_e100670 * ((locals.var_beta_dn0 * assign65230_e100668) + (locals.var_beta * locals.var_vbscl__blk1549_dn0))), (assign65230_e100670 * ((locals.var_beta_dn2 * assign65230_e100668) + (locals.var_beta * locals.var_vbscl__blk1549_dn2))), (assign65230_e100670 * ((locals.var_beta_dn4 * assign65230_e100668) + (locals.var_beta * locals.var_vbscl__blk1549_dn4))), (assign65230_e100670 * ((locals.var_beta_dn5 * assign65230_e100668) + (locals.var_beta * locals.var_vbscl__blk1549_dn5))), (assign65230_e100670 * ((locals.var_beta_dn6 * assign65230_e100668) + (locals.var_beta * locals.var_vbscl__blk1549_dn6))), (assign65230_e100670 * ((locals.var_beta_dn7 * assign65230_e100668) + (locals.var_beta * locals.var_vbscl__blk1549_dn7))), (assign65230_e100670 * ((locals.var_beta_dn8 * assign65230_e100668) + (locals.var_beta * locals.var_vbscl__blk1549_dn8))), (assign65230_e100670 * ((locals.var_beta_dn9 * assign65230_e100668) + (locals.var_beta * locals.var_vbscl__blk1549_dn9))), (assign65230_e100670 * ((locals.var_beta_dn10 * assign65230_e100668) + (locals.var_beta * locals.var_vbscl__blk1549_dn10))), (assign65230_e100670 * ((locals.var_beta_dn11 * assign65230_e100668) + (locals.var_beta * locals.var_vbscl__blk1549_dn11))), (assign65230_e100670 * ((locals.var_beta_dn14 * assign65230_e100668) + (locals.var_beta * locals.var_vbscl__blk1549_dn14))),)
    } else {
        (locals.var_exp_bvbsvds, locals.var_exp_bvbsvds_dn0, locals.var_exp_bvbsvds_dn2, locals.var_exp_bvbsvds_dn4, locals.var_exp_bvbsvds_dn5, locals.var_exp_bvbsvds_dn6, locals.var_exp_bvbsvds_dn7, locals.var_exp_bvbsvds_dn8, locals.var_exp_bvbsvds_dn9, locals.var_exp_bvbsvds_dn10, locals.var_exp_bvbsvds_dn11, locals.var_exp_bvbsvds_dn14,)
    }
};
        locals.var_exp_bvbsvds = assign65230_e100672;
        locals.var_exp_bvbsvds_dn0 = assign65230_e100672_d_n0;
        locals.var_exp_bvbsvds_dn2 = assign65230_e100672_d_n2;
        locals.var_exp_bvbsvds_dn4 = assign65230_e100672_d_n4;
        locals.var_exp_bvbsvds_dn5 = assign65230_e100672_d_n5;
        locals.var_exp_bvbsvds_dn6 = assign65230_e100672_d_n6;
        locals.var_exp_bvbsvds_dn7 = assign65230_e100672_d_n7;
        locals.var_exp_bvbsvds_dn8 = assign65230_e100672_d_n8;
        locals.var_exp_bvbsvds_dn9 = assign65230_e100672_d_n9;
        locals.var_exp_bvbsvds_dn10 = assign65230_e100672_d_n10;
        locals.var_exp_bvbsvds_dn11 = assign65230_e100672_d_n11;
        locals.var_exp_bvbsvds_dn14 = assign65230_e100672_d_n14;
        locals.var_exp_bvbsvds_rv = 0.0;

        let (assign65240_e100687,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign65240_e100687;
        locals.var_flg_conv_rv = 0.0;

        let (assign65250_e100702, assign65250_e100702_d_n0, assign65250_e100702_d_n2, assign65250_e100702_d_n4, assign65250_e100702_d_n5, assign65250_e100702_d_n6, assign65250_e100702_d_n7, assign65250_e100702_d_n8, assign65250_e100702_d_n9, assign65250_e100702_d_n10, assign65250_e100702_d_n11, assign65250_e100702_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn14,)
    } else {
        (locals.var_phi_s0, locals.var_phi_s0_dn0, locals.var_phi_s0_dn2, locals.var_phi_s0_dn4, locals.var_phi_s0_dn5, locals.var_phi_s0_dn6, locals.var_phi_s0_dn7, locals.var_phi_s0_dn8, locals.var_phi_s0_dn9, locals.var_phi_s0_dn10, locals.var_phi_s0_dn11, locals.var_phi_s0_dn14,)
    }
};
        locals.var_phi_s0 = assign65250_e100702;
        locals.var_phi_s0_dn0 = assign65250_e100702_d_n0;
        locals.var_phi_s0_dn2 = assign65250_e100702_d_n2;
        locals.var_phi_s0_dn4 = assign65250_e100702_d_n4;
        locals.var_phi_s0_dn5 = assign65250_e100702_d_n5;
        locals.var_phi_s0_dn6 = assign65250_e100702_d_n6;
        locals.var_phi_s0_dn7 = assign65250_e100702_d_n7;
        locals.var_phi_s0_dn8 = assign65250_e100702_d_n8;
        locals.var_phi_s0_dn9 = assign65250_e100702_d_n9;
        locals.var_phi_s0_dn10 = assign65250_e100702_d_n10;
        locals.var_phi_s0_dn11 = assign65250_e100702_d_n11;
        locals.var_phi_s0_dn14 = assign65250_e100702_d_n14;
        locals.var_phi_s0_rv = 0.0;

        let (assign65260_e100725, assign65260_e100725_d_n0, assign65260_e100725_d_n2, assign65260_e100725_d_n4, assign65260_e100725_d_n5, assign65260_e100725_d_n6, assign65260_e100725_d_n7, assign65260_e100725_d_n8, assign65260_e100725_d_n9, assign65260_e100725_d_n10, assign65260_e100725_d_n11, assign65260_e100725_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) {
        let assign65260_e100717: f64 = (locals.var_q_nsub * locals.var_t_sub);
        let assign65260_e100719: f64 = (assign65260_e100717 * locals.var_t_sub);
        let assign65260_e100721: f64 = (assign65260_e100719 / 2.0);
        let assign65260_e100723: f64 = (assign65260_e100721 / 1.034943e-10);
        (assign65260_e100723, ((((locals.var_q_nsub_dn0 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn2 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn4 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn5 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn6 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn7 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn8 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn9 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn10 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn11 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn14 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb__blk1551, locals.var_dphi_sb__blk1551_dn0, locals.var_dphi_sb__blk1551_dn2, locals.var_dphi_sb__blk1551_dn4, locals.var_dphi_sb__blk1551_dn5, locals.var_dphi_sb__blk1551_dn6, locals.var_dphi_sb__blk1551_dn7, locals.var_dphi_sb__blk1551_dn8, locals.var_dphi_sb__blk1551_dn9, locals.var_dphi_sb__blk1551_dn10, locals.var_dphi_sb__blk1551_dn11, locals.var_dphi_sb__blk1551_dn14,)
    }
};
        locals.var_dphi_sb__blk1551 = assign65260_e100725;
        locals.var_dphi_sb__blk1551_dn0 = assign65260_e100725_d_n0;
        locals.var_dphi_sb__blk1551_dn2 = assign65260_e100725_d_n2;
        locals.var_dphi_sb__blk1551_dn4 = assign65260_e100725_d_n4;
        locals.var_dphi_sb__blk1551_dn5 = assign65260_e100725_d_n5;
        locals.var_dphi_sb__blk1551_dn6 = assign65260_e100725_d_n6;
        locals.var_dphi_sb__blk1551_dn7 = assign65260_e100725_d_n7;
        locals.var_dphi_sb__blk1551_dn8 = assign65260_e100725_d_n8;
        locals.var_dphi_sb__blk1551_dn9 = assign65260_e100725_d_n9;
        locals.var_dphi_sb__blk1551_dn10 = assign65260_e100725_d_n10;
        locals.var_dphi_sb__blk1551_dn11 = assign65260_e100725_d_n11;
        locals.var_dphi_sb__blk1551_dn14 = assign65260_e100725_d_n14;
        locals.var_dphi_sb__blk1551_rv = 0.0;

        let (assign65270_e100745, assign65270_e100745_d_n0, assign65270_e100745_d_n2, assign65270_e100745_d_n4, assign65270_e100745_d_n5, assign65270_e100745_d_n6, assign65270_e100745_d_n7, assign65270_e100745_d_n8, assign65270_e100745_d_n9, assign65270_e100745_d_n10, assign65270_e100745_d_n11, assign65270_e100745_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) {
        let assign65270_e100740: f64 = (2.0 * locals.var_beta);
        let assign65270_e100742: f64 = (assign65270_e100740 * locals.var_dphi_sb__blk1551);
        let assign65270_e100743: f64 = (assign65270_e100742).sqrt();
        (assign65270_e100743, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb__blk1551) + (assign65270_e100740 * locals.var_dphi_sb__blk1551_dn0)) / (2.0 * assign65270_e100743)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb__blk1551) + (assign65270_e100740 * locals.var_dphi_sb__blk1551_dn2)) / (2.0 * assign65270_e100743)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb__blk1551) + (assign65270_e100740 * locals.var_dphi_sb__blk1551_dn4)) / (2.0 * assign65270_e100743)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb__blk1551) + (assign65270_e100740 * locals.var_dphi_sb__blk1551_dn5)) / (2.0 * assign65270_e100743)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb__blk1551) + (assign65270_e100740 * locals.var_dphi_sb__blk1551_dn6)) / (2.0 * assign65270_e100743)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb__blk1551) + (assign65270_e100740 * locals.var_dphi_sb__blk1551_dn7)) / (2.0 * assign65270_e100743)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb__blk1551) + (assign65270_e100740 * locals.var_dphi_sb__blk1551_dn8)) / (2.0 * assign65270_e100743)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb__blk1551) + (assign65270_e100740 * locals.var_dphi_sb__blk1551_dn9)) / (2.0 * assign65270_e100743)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb__blk1551) + (assign65270_e100740 * locals.var_dphi_sb__blk1551_dn10)) / (2.0 * assign65270_e100743)), ((((2.0 * locals.var_beta_dn11) * locals.var_dphi_sb__blk1551) + (assign65270_e100740 * locals.var_dphi_sb__blk1551_dn11)) / (2.0 * assign65270_e100743)), ((((2.0 * locals.var_beta_dn14) * locals.var_dphi_sb__blk1551) + (assign65270_e100740 * locals.var_dphi_sb__blk1551_dn14)) / (2.0 * assign65270_e100743)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign65270_e100745;
        locals.var_t0_dn0 = assign65270_e100745_d_n0;
        locals.var_t0_dn2 = assign65270_e100745_d_n2;
        locals.var_t0_dn4 = assign65270_e100745_d_n4;
        locals.var_t0_dn5 = assign65270_e100745_d_n5;
        locals.var_t0_dn6 = assign65270_e100745_d_n6;
        locals.var_t0_dn7 = assign65270_e100745_d_n7;
        locals.var_t0_dn8 = assign65270_e100745_d_n8;
        locals.var_t0_dn9 = assign65270_e100745_d_n9;
        locals.var_t0_dn10 = assign65270_e100745_d_n10;
        locals.var_t0_dn11 = assign65270_e100745_d_n11;
        locals.var_t0_dn14 = assign65270_e100745_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign65280_e100767, assign65280_e100767_d_n0, assign65280_e100767_d_n2, assign65280_e100767_d_n4, assign65280_e100767_d_n5, assign65280_e100767_d_n6, assign65280_e100767_d_n7, assign65280_e100767_d_n8, assign65280_e100767_d_n9, assign65280_e100767_d_n10, assign65280_e100767_d_n11, assign65280_e100767_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) {
        let assign65280_e100759: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign65280_e100761: f64 = (-locals.var_t0);
        let assign65280_e100762: f64 = { let limited_exp_arg = assign65280_e100761; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign65280_e100763: f64 = (assign65280_e100759 + assign65280_e100762);
        let assign65280_e100765: f64 = (assign65280_e100763 / 2.0);
        (assign65280_e100765, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign65280_e100761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign65280_e100761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign65280_e100761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign65280_e100761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign65280_e100761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign65280_e100761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign65280_e100761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign65280_e100761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign65280_e100761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11) + ({ let limited_exp_arg = assign65280_e100761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn11))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn14) + ({ let limited_exp_arg = assign65280_e100761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn14))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign65280_e100767;
        locals.var_t1_dn0 = assign65280_e100767_d_n0;
        locals.var_t1_dn2 = assign65280_e100767_d_n2;
        locals.var_t1_dn4 = assign65280_e100767_d_n4;
        locals.var_t1_dn5 = assign65280_e100767_d_n5;
        locals.var_t1_dn6 = assign65280_e100767_d_n6;
        locals.var_t1_dn7 = assign65280_e100767_d_n7;
        locals.var_t1_dn8 = assign65280_e100767_d_n8;
        locals.var_t1_dn9 = assign65280_e100767_d_n9;
        locals.var_t1_dn10 = assign65280_e100767_d_n10;
        locals.var_t1_dn11 = assign65280_e100767_d_n11;
        locals.var_t1_dn14 = assign65280_e100767_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign65290_e100785, assign65290_e100785_d_n0, assign65290_e100785_d_n2, assign65290_e100785_d_n4, assign65290_e100785_d_n5, assign65290_e100785_d_n6, assign65290_e100785_d_n7, assign65290_e100785_d_n8, assign65290_e100785_d_n9, assign65290_e100785_d_n10, assign65290_e100785_d_n11, assign65290_e100785_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) {
        let assign65290_e100781: f64 = (locals.var_t1).ln();
        let assign65290_e100783: f64 = (assign65290_e100781 / locals.var_dphi_sb__blk1551);
        (assign65290_e100783, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb__blk1551) - (assign65290_e100781 * locals.var_dphi_sb__blk1551_dn0)) / (locals.var_dphi_sb__blk1551 * locals.var_dphi_sb__blk1551)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb__blk1551) - (assign65290_e100781 * locals.var_dphi_sb__blk1551_dn2)) / (locals.var_dphi_sb__blk1551 * locals.var_dphi_sb__blk1551)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb__blk1551) - (assign65290_e100781 * locals.var_dphi_sb__blk1551_dn4)) / (locals.var_dphi_sb__blk1551 * locals.var_dphi_sb__blk1551)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb__blk1551) - (assign65290_e100781 * locals.var_dphi_sb__blk1551_dn5)) / (locals.var_dphi_sb__blk1551 * locals.var_dphi_sb__blk1551)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb__blk1551) - (assign65290_e100781 * locals.var_dphi_sb__blk1551_dn6)) / (locals.var_dphi_sb__blk1551 * locals.var_dphi_sb__blk1551)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb__blk1551) - (assign65290_e100781 * locals.var_dphi_sb__blk1551_dn7)) / (locals.var_dphi_sb__blk1551 * locals.var_dphi_sb__blk1551)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb__blk1551) - (assign65290_e100781 * locals.var_dphi_sb__blk1551_dn8)) / (locals.var_dphi_sb__blk1551 * locals.var_dphi_sb__blk1551)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb__blk1551) - (assign65290_e100781 * locals.var_dphi_sb__blk1551_dn9)) / (locals.var_dphi_sb__blk1551 * locals.var_dphi_sb__blk1551)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb__blk1551) - (assign65290_e100781 * locals.var_dphi_sb__blk1551_dn10)) / (locals.var_dphi_sb__blk1551 * locals.var_dphi_sb__blk1551)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb__blk1551) - (assign65290_e100781 * locals.var_dphi_sb__blk1551_dn11)) / (locals.var_dphi_sb__blk1551 * locals.var_dphi_sb__blk1551)), ((((locals.var_t1_dn14 / locals.var_t1) * locals.var_dphi_sb__blk1551) - (assign65290_e100781 * locals.var_dphi_sb__blk1551_dn14)) / (locals.var_dphi_sb__blk1551 * locals.var_dphi_sb__blk1551)),)
    } else {
        (locals.var_c_sb__blk1552, locals.var_c_sb__blk1552_dn0, locals.var_c_sb__blk1552_dn2, locals.var_c_sb__blk1552_dn4, locals.var_c_sb__blk1552_dn5, locals.var_c_sb__blk1552_dn6, locals.var_c_sb__blk1552_dn7, locals.var_c_sb__blk1552_dn8, locals.var_c_sb__blk1552_dn9, locals.var_c_sb__blk1552_dn10, locals.var_c_sb__blk1552_dn11, locals.var_c_sb__blk1552_dn14,)
    }
};
        locals.var_c_sb__blk1552 = assign65290_e100785;
        locals.var_c_sb__blk1552_dn0 = assign65290_e100785_d_n0;
        locals.var_c_sb__blk1552_dn2 = assign65290_e100785_d_n2;
        locals.var_c_sb__blk1552_dn4 = assign65290_e100785_d_n4;
        locals.var_c_sb__blk1552_dn5 = assign65290_e100785_d_n5;
        locals.var_c_sb__blk1552_dn6 = assign65290_e100785_d_n6;
        locals.var_c_sb__blk1552_dn7 = assign65290_e100785_d_n7;
        locals.var_c_sb__blk1552_dn8 = assign65290_e100785_d_n8;
        locals.var_c_sb__blk1552_dn9 = assign65290_e100785_d_n9;
        locals.var_c_sb__blk1552_dn10 = assign65290_e100785_d_n10;
        locals.var_c_sb__blk1552_dn11 = assign65290_e100785_d_n11;
        locals.var_c_sb__blk1552_dn14 = assign65290_e100785_d_n14;
        locals.var_c_sb__blk1552_rv = 0.0;

        let (assign65300_e100800,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign65300_e100800;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_245(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign65310_loop_guard: usize = 0;
        while {
            let assign65310_cond_e100816: f64 = (locals.var_lp_s0_max + 1.0);
            let assign65310_cond_e100818: f64 = if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_lp_s0 <= assign65310_cond_e100816)) { 1.0 } else { 0.0 };
            assign65310_cond_e100818 != 0.0
        } {
            assign65310_loop_guard += 1;
            assert!(assign65310_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign65310_body0_e100835, assign65310_body0_e100835_d_n0, assign65310_body0_e100835_d_n2, assign65310_body0_e100835_d_n4, assign65310_body0_e100835_d_n5, assign65310_body0_e100835_d_n6, assign65310_body0_e100835_d_n7, assign65310_body0_e100835_d_n8, assign65310_body0_e100835_d_n9, assign65310_body0_e100835_d_n10, assign65310_body0_e100835_d_n11, assign65310_body0_e100835_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) {
        let assign65310_body0_e100833: f64 = (locals.var_phi_s0 - locals.var_vbscl__blk1549);
        (assign65310_body0_e100833, (locals.var_phi_s0_dn0 - locals.var_vbscl__blk1549_dn0), (locals.var_phi_s0_dn2 - locals.var_vbscl__blk1549_dn2), (locals.var_phi_s0_dn4 - locals.var_vbscl__blk1549_dn4), (locals.var_phi_s0_dn5 - locals.var_vbscl__blk1549_dn5), (locals.var_phi_s0_dn6 - locals.var_vbscl__blk1549_dn6), (locals.var_phi_s0_dn7 - locals.var_vbscl__blk1549_dn7), (locals.var_phi_s0_dn8 - locals.var_vbscl__blk1549_dn8), (locals.var_phi_s0_dn9 - locals.var_vbscl__blk1549_dn9), (locals.var_phi_s0_dn10 - locals.var_vbscl__blk1549_dn10), (locals.var_phi_s0_dn11 - locals.var_vbscl__blk1549_dn11), (locals.var_phi_s0_dn14 - locals.var_vbscl__blk1549_dn14),)
    } else {
        (locals.var_phi_0, locals.var_phi_0_dn0, locals.var_phi_0_dn2, locals.var_phi_0_dn4, locals.var_phi_0_dn5, locals.var_phi_0_dn6, locals.var_phi_0_dn7, locals.var_phi_0_dn8, locals.var_phi_0_dn9, locals.var_phi_0_dn10, locals.var_phi_0_dn11, locals.var_phi_0_dn14,)
    }
};
            locals.var_phi_0 = assign65310_body0_e100835;
            locals.var_phi_0_dn0 = assign65310_body0_e100835_d_n0;
            locals.var_phi_0_dn2 = assign65310_body0_e100835_d_n2;
            locals.var_phi_0_dn4 = assign65310_body0_e100835_d_n4;
            locals.var_phi_0_dn5 = assign65310_body0_e100835_d_n5;
            locals.var_phi_0_dn6 = assign65310_body0_e100835_d_n6;
            locals.var_phi_0_dn7 = assign65310_body0_e100835_d_n7;
            locals.var_phi_0_dn8 = assign65310_body0_e100835_d_n8;
            locals.var_phi_0_dn9 = assign65310_body0_e100835_d_n9;
            locals.var_phi_0_dn10 = assign65310_body0_e100835_d_n10;
            locals.var_phi_0_dn11 = assign65310_body0_e100835_d_n11;
            locals.var_phi_0_dn14 = assign65310_body0_e100835_d_n14;
            locals.var_phi_0_rv = 0.0;
            let (assign65310_body1_e100852, assign65310_body1_e100852_d_n0, assign65310_body1_e100852_d_n2, assign65310_body1_e100852_d_n4, assign65310_body1_e100852_d_n5, assign65310_body1_e100852_d_n6, assign65310_body1_e100852_d_n7, assign65310_body1_e100852_d_n8, assign65310_body1_e100852_d_n9, assign65310_body1_e100852_d_n10, assign65310_body1_e100852_d_n11, assign65310_body1_e100852_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) {
        let assign65310_body1_e100850: f64 = (locals.var_beta * locals.var_phi_0);
        (assign65310_body1_e100850, ((locals.var_beta_dn0 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn0)), ((locals.var_beta_dn2 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn2)), ((locals.var_beta_dn4 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn4)), ((locals.var_beta_dn5 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn5)), ((locals.var_beta_dn6 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn6)), ((locals.var_beta_dn7 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn7)), ((locals.var_beta_dn8 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn8)), ((locals.var_beta_dn9 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn9)), ((locals.var_beta_dn10 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn10)), ((locals.var_beta_dn11 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn11)), ((locals.var_beta_dn14 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn14)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
            locals.var_chi = assign65310_body1_e100852;
            locals.var_chi_dn0 = assign65310_body1_e100852_d_n0;
            locals.var_chi_dn2 = assign65310_body1_e100852_d_n2;
            locals.var_chi_dn4 = assign65310_body1_e100852_d_n4;
            locals.var_chi_dn5 = assign65310_body1_e100852_d_n5;
            locals.var_chi_dn6 = assign65310_body1_e100852_d_n6;
            locals.var_chi_dn7 = assign65310_body1_e100852_d_n7;
            locals.var_chi_dn8 = assign65310_body1_e100852_d_n8;
            locals.var_chi_dn9 = assign65310_body1_e100852_d_n9;
            locals.var_chi_dn10 = assign65310_body1_e100852_d_n10;
            locals.var_chi_dn11 = assign65310_body1_e100852_d_n11;
            locals.var_chi_dn14 = assign65310_body1_e100852_d_n14;
            locals.var_chi_rv = 0.0;
            let (assign65310_body2_e100871, assign65310_body2_e100871_d_n0, assign65310_body2_e100871_d_n2, assign65310_body2_e100871_d_n4, assign65310_body2_e100871_d_n5, assign65310_body2_e100871_d_n6, assign65310_body2_e100871_d_n7, assign65310_body2_e100871_d_n8, assign65310_body2_e100871_d_n9, assign65310_body2_e100871_d_n10, assign65310_body2_e100871_d_n11, assign65310_body2_e100871_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) {
        let assign65310_body2_e100868: f64 = (locals.var_phi_0 - locals.var_dphi_sb__blk1551);
        let assign65310_body2_e100869: f64 = (locals.var_c_sb__blk1552 * assign65310_body2_e100868);
        (assign65310_body2_e100869, ((locals.var_c_sb__blk1552_dn0 * assign65310_body2_e100868) + (locals.var_c_sb__blk1552 * (locals.var_phi_0_dn0 - locals.var_dphi_sb__blk1551_dn0))), ((locals.var_c_sb__blk1552_dn2 * assign65310_body2_e100868) + (locals.var_c_sb__blk1552 * (locals.var_phi_0_dn2 - locals.var_dphi_sb__blk1551_dn2))), ((locals.var_c_sb__blk1552_dn4 * assign65310_body2_e100868) + (locals.var_c_sb__blk1552 * (locals.var_phi_0_dn4 - locals.var_dphi_sb__blk1551_dn4))), ((locals.var_c_sb__blk1552_dn5 * assign65310_body2_e100868) + (locals.var_c_sb__blk1552 * (locals.var_phi_0_dn5 - locals.var_dphi_sb__blk1551_dn5))), ((locals.var_c_sb__blk1552_dn6 * assign65310_body2_e100868) + (locals.var_c_sb__blk1552 * (locals.var_phi_0_dn6 - locals.var_dphi_sb__blk1551_dn6))), ((locals.var_c_sb__blk1552_dn7 * assign65310_body2_e100868) + (locals.var_c_sb__blk1552 * (locals.var_phi_0_dn7 - locals.var_dphi_sb__blk1551_dn7))), ((locals.var_c_sb__blk1552_dn8 * assign65310_body2_e100868) + (locals.var_c_sb__blk1552 * (locals.var_phi_0_dn8 - locals.var_dphi_sb__blk1551_dn8))), ((locals.var_c_sb__blk1552_dn9 * assign65310_body2_e100868) + (locals.var_c_sb__blk1552 * (locals.var_phi_0_dn9 - locals.var_dphi_sb__blk1551_dn9))), ((locals.var_c_sb__blk1552_dn10 * assign65310_body2_e100868) + (locals.var_c_sb__blk1552 * (locals.var_phi_0_dn10 - locals.var_dphi_sb__blk1551_dn10))), ((locals.var_c_sb__blk1552_dn11 * assign65310_body2_e100868) + (locals.var_c_sb__blk1552 * (locals.var_phi_0_dn11 - locals.var_dphi_sb__blk1551_dn11))), ((locals.var_c_sb__blk1552_dn14 * assign65310_body2_e100868) + (locals.var_c_sb__blk1552 * (locals.var_phi_0_dn14 - locals.var_dphi_sb__blk1551_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
            locals.var_ty = assign65310_body2_e100871;
            locals.var_ty_dn0 = assign65310_body2_e100871_d_n0;
            locals.var_ty_dn2 = assign65310_body2_e100871_d_n2;
            locals.var_ty_dn4 = assign65310_body2_e100871_d_n4;
            locals.var_ty_dn5 = assign65310_body2_e100871_d_n5;
            locals.var_ty_dn6 = assign65310_body2_e100871_d_n6;
            locals.var_ty_dn7 = assign65310_body2_e100871_d_n7;
            locals.var_ty_dn8 = assign65310_body2_e100871_d_n8;
            locals.var_ty_dn9 = assign65310_body2_e100871_d_n9;
            locals.var_ty_dn10 = assign65310_body2_e100871_d_n10;
            locals.var_ty_dn11 = assign65310_body2_e100871_d_n11;
            locals.var_ty_dn14 = assign65310_body2_e100871_d_n14;
            locals.var_ty_rv = 0.0;
            let assign65310_body3_e100874: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard1560 = assign65310_body3_e100874;
            locals.var_guard1560_rv = 0.0;
            let (assign65310_body4_e100892, assign65310_body4_e100892_d_n0, assign65310_body4_e100892_d_n2, assign65310_body4_e100892_d_n4, assign65310_body4_e100892_d_n5, assign65310_body4_e100892_d_n6, assign65310_body4_e100892_d_n7, assign65310_body4_e100892_d_n8, assign65310_body4_e100892_d_n9, assign65310_body4_e100892_d_n10, assign65310_body4_e100892_d_n11, assign65310_body4_e100892_d_n14,) = {
    if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1560 != 0.0)) {
        let assign65310_body4_e100890: f64 = (locals.var_ty).exp();
        (assign65310_body4_e100890, (assign65310_body4_e100890 * locals.var_ty_dn0), (assign65310_body4_e100890 * locals.var_ty_dn2), (assign65310_body4_e100890 * locals.var_ty_dn4), (assign65310_body4_e100890 * locals.var_ty_dn5), (assign65310_body4_e100890 * locals.var_ty_dn6), (assign65310_body4_e100890 * locals.var_ty_dn7), (assign65310_body4_e100890 * locals.var_ty_dn8), (assign65310_body4_e100890 * locals.var_ty_dn9), (assign65310_body4_e100890 * locals.var_ty_dn10), (assign65310_body4_e100890 * locals.var_ty_dn11), (assign65310_body4_e100890 * locals.var_ty_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign65310_body4_e100892;
            locals.var_t1_dn0 = assign65310_body4_e100892_d_n0;
            locals.var_t1_dn2 = assign65310_body4_e100892_d_n2;
            locals.var_t1_dn4 = assign65310_body4_e100892_d_n4;
            locals.var_t1_dn5 = assign65310_body4_e100892_d_n5;
            locals.var_t1_dn6 = assign65310_body4_e100892_d_n6;
            locals.var_t1_dn7 = assign65310_body4_e100892_d_n7;
            locals.var_t1_dn8 = assign65310_body4_e100892_d_n8;
            locals.var_t1_dn9 = assign65310_body4_e100892_d_n9;
            locals.var_t1_dn10 = assign65310_body4_e100892_d_n10;
            locals.var_t1_dn11 = assign65310_body4_e100892_d_n11;
            locals.var_t1_dn14 = assign65310_body4_e100892_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign65310_body5_e100913, assign65310_body5_e100913_d_n0, assign65310_body5_e100913_d_n2, assign65310_body5_e100913_d_n4, assign65310_body5_e100913_d_n5, assign65310_body5_e100913_d_n6, assign65310_body5_e100913_d_n7, assign65310_body5_e100913_d_n8, assign65310_body5_e100913_d_n9, assign65310_body5_e100913_d_n10, assign65310_body5_e100913_d_n11, assign65310_body5_e100913_d_n14,) = {
    if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1560 != 0.0)) {
        let assign65310_body5_e100908: f64 = (-locals.var_c_sb__blk1552);
        let assign65310_body5_e100910: f64 = (assign65310_body5_e100908 * locals.var_dphi_sb__blk1551);
        let assign65310_body5_e100911: f64 = (assign65310_body5_e100910).exp();
        (assign65310_body5_e100911, (assign65310_body5_e100911 * (((-locals.var_c_sb__blk1552_dn0) * locals.var_dphi_sb__blk1551) + (assign65310_body5_e100908 * locals.var_dphi_sb__blk1551_dn0))), (assign65310_body5_e100911 * (((-locals.var_c_sb__blk1552_dn2) * locals.var_dphi_sb__blk1551) + (assign65310_body5_e100908 * locals.var_dphi_sb__blk1551_dn2))), (assign65310_body5_e100911 * (((-locals.var_c_sb__blk1552_dn4) * locals.var_dphi_sb__blk1551) + (assign65310_body5_e100908 * locals.var_dphi_sb__blk1551_dn4))), (assign65310_body5_e100911 * (((-locals.var_c_sb__blk1552_dn5) * locals.var_dphi_sb__blk1551) + (assign65310_body5_e100908 * locals.var_dphi_sb__blk1551_dn5))), (assign65310_body5_e100911 * (((-locals.var_c_sb__blk1552_dn6) * locals.var_dphi_sb__blk1551) + (assign65310_body5_e100908 * locals.var_dphi_sb__blk1551_dn6))), (assign65310_body5_e100911 * (((-locals.var_c_sb__blk1552_dn7) * locals.var_dphi_sb__blk1551) + (assign65310_body5_e100908 * locals.var_dphi_sb__blk1551_dn7))), (assign65310_body5_e100911 * (((-locals.var_c_sb__blk1552_dn8) * locals.var_dphi_sb__blk1551) + (assign65310_body5_e100908 * locals.var_dphi_sb__blk1551_dn8))), (assign65310_body5_e100911 * (((-locals.var_c_sb__blk1552_dn9) * locals.var_dphi_sb__blk1551) + (assign65310_body5_e100908 * locals.var_dphi_sb__blk1551_dn9))), (assign65310_body5_e100911 * (((-locals.var_c_sb__blk1552_dn10) * locals.var_dphi_sb__blk1551) + (assign65310_body5_e100908 * locals.var_dphi_sb__blk1551_dn10))), (assign65310_body5_e100911 * (((-locals.var_c_sb__blk1552_dn11) * locals.var_dphi_sb__blk1551) + (assign65310_body5_e100908 * locals.var_dphi_sb__blk1551_dn11))), (assign65310_body5_e100911 * (((-locals.var_c_sb__blk1552_dn14) * locals.var_dphi_sb__blk1551) + (assign65310_body5_e100908 * locals.var_dphi_sb__blk1551_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign65310_body5_e100913;
            locals.var_t0_dn0 = assign65310_body5_e100913_d_n0;
            locals.var_t0_dn2 = assign65310_body5_e100913_d_n2;
            locals.var_t0_dn4 = assign65310_body5_e100913_d_n4;
            locals.var_t0_dn5 = assign65310_body5_e100913_d_n5;
            locals.var_t0_dn6 = assign65310_body5_e100913_d_n6;
            locals.var_t0_dn7 = assign65310_body5_e100913_d_n7;
            locals.var_t0_dn8 = assign65310_body5_e100913_d_n8;
            locals.var_t0_dn9 = assign65310_body5_e100913_d_n9;
            locals.var_t0_dn10 = assign65310_body5_e100913_d_n10;
            locals.var_t0_dn11 = assign65310_body5_e100913_d_n11;
            locals.var_t0_dn14 = assign65310_body5_e100913_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign65310_body6_e100932, assign65310_body6_e100932_d_n0, assign65310_body6_e100932_d_n2, assign65310_body6_e100932_d_n4, assign65310_body6_e100932_d_n5, assign65310_body6_e100932_d_n6, assign65310_body6_e100932_d_n7, assign65310_body6_e100932_d_n8, assign65310_body6_e100932_d_n9, assign65310_body6_e100932_d_n10, assign65310_body6_e100932_d_n11, assign65310_body6_e100932_d_n14,) = {
    if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1560 != 0.0)) {
        let assign65310_body6_e100930: f64 = (locals.var_t1 - locals.var_t0);
        (assign65310_body6_e100930, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn14 - locals.var_t0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign65310_body6_e100932;
            locals.var_t2_dn0 = assign65310_body6_e100932_d_n0;
            locals.var_t2_dn2 = assign65310_body6_e100932_d_n2;
            locals.var_t2_dn4 = assign65310_body6_e100932_d_n4;
            locals.var_t2_dn5 = assign65310_body6_e100932_d_n5;
            locals.var_t2_dn6 = assign65310_body6_e100932_d_n6;
            locals.var_t2_dn7 = assign65310_body6_e100932_d_n7;
            locals.var_t2_dn8 = assign65310_body6_e100932_d_n8;
            locals.var_t2_dn9 = assign65310_body6_e100932_d_n9;
            locals.var_t2_dn10 = assign65310_body6_e100932_d_n10;
            locals.var_t2_dn11 = assign65310_body6_e100932_d_n11;
            locals.var_t2_dn14 = assign65310_body6_e100932_d_n14;
            locals.var_t2_rv = 0.0;
            let (assign65310_body7_e100954, assign65310_body7_e100954_d_n0, assign65310_body7_e100954_d_n2, assign65310_body7_e100954_d_n4, assign65310_body7_e100954_d_n5, assign65310_body7_e100954_d_n6, assign65310_body7_e100954_d_n7, assign65310_body7_e100954_d_n8, assign65310_body7_e100954_d_n9, assign65310_body7_e100954_d_n10, assign65310_body7_e100954_d_n11, assign65310_body7_e100954_d_n14,) = {
    if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1560 != 0.0)) {
        let assign65310_body7_e100949: f64 = (1.0 + locals.var_t2);
        let assign65310_body7_e100950: f64 = (assign65310_body7_e100949).ln();
        let assign65310_body7_e100952: f64 = (assign65310_body7_e100950 / locals.var_c_sb__blk1552);
        (assign65310_body7_e100952, ((((locals.var_t2_dn0 / assign65310_body7_e100949) * locals.var_c_sb__blk1552) - (assign65310_body7_e100950 * locals.var_c_sb__blk1552_dn0)) / (locals.var_c_sb__blk1552 * locals.var_c_sb__blk1552)), ((((locals.var_t2_dn2 / assign65310_body7_e100949) * locals.var_c_sb__blk1552) - (assign65310_body7_e100950 * locals.var_c_sb__blk1552_dn2)) / (locals.var_c_sb__blk1552 * locals.var_c_sb__blk1552)), ((((locals.var_t2_dn4 / assign65310_body7_e100949) * locals.var_c_sb__blk1552) - (assign65310_body7_e100950 * locals.var_c_sb__blk1552_dn4)) / (locals.var_c_sb__blk1552 * locals.var_c_sb__blk1552)), ((((locals.var_t2_dn5 / assign65310_body7_e100949) * locals.var_c_sb__blk1552) - (assign65310_body7_e100950 * locals.var_c_sb__blk1552_dn5)) / (locals.var_c_sb__blk1552 * locals.var_c_sb__blk1552)), ((((locals.var_t2_dn6 / assign65310_body7_e100949) * locals.var_c_sb__blk1552) - (assign65310_body7_e100950 * locals.var_c_sb__blk1552_dn6)) / (locals.var_c_sb__blk1552 * locals.var_c_sb__blk1552)), ((((locals.var_t2_dn7 / assign65310_body7_e100949) * locals.var_c_sb__blk1552) - (assign65310_body7_e100950 * locals.var_c_sb__blk1552_dn7)) / (locals.var_c_sb__blk1552 * locals.var_c_sb__blk1552)), ((((locals.var_t2_dn8 / assign65310_body7_e100949) * locals.var_c_sb__blk1552) - (assign65310_body7_e100950 * locals.var_c_sb__blk1552_dn8)) / (locals.var_c_sb__blk1552 * locals.var_c_sb__blk1552)), ((((locals.var_t2_dn9 / assign65310_body7_e100949) * locals.var_c_sb__blk1552) - (assign65310_body7_e100950 * locals.var_c_sb__blk1552_dn9)) / (locals.var_c_sb__blk1552 * locals.var_c_sb__blk1552)), ((((locals.var_t2_dn10 / assign65310_body7_e100949) * locals.var_c_sb__blk1552) - (assign65310_body7_e100950 * locals.var_c_sb__blk1552_dn10)) / (locals.var_c_sb__blk1552 * locals.var_c_sb__blk1552)), ((((locals.var_t2_dn11 / assign65310_body7_e100949) * locals.var_c_sb__blk1552) - (assign65310_body7_e100950 * locals.var_c_sb__blk1552_dn11)) / (locals.var_c_sb__blk1552 * locals.var_c_sb__blk1552)), ((((locals.var_t2_dn14 / assign65310_body7_e100949) * locals.var_c_sb__blk1552) - (assign65310_body7_e100950 * locals.var_c_sb__blk1552_dn14)) / (locals.var_c_sb__blk1552 * locals.var_c_sb__blk1552)),)
    } else {
        (locals.var_phi_b__blk1555, locals.var_phi_b__blk1555_dn0, locals.var_phi_b__blk1555_dn2, locals.var_phi_b__blk1555_dn4, locals.var_phi_b__blk1555_dn5, locals.var_phi_b__blk1555_dn6, locals.var_phi_b__blk1555_dn7, locals.var_phi_b__blk1555_dn8, locals.var_phi_b__blk1555_dn9, locals.var_phi_b__blk1555_dn10, locals.var_phi_b__blk1555_dn11, locals.var_phi_b__blk1555_dn14,)
    }
};
            locals.var_phi_b__blk1555 = assign65310_body7_e100954;
            locals.var_phi_b__blk1555_dn0 = assign65310_body7_e100954_d_n0;
            locals.var_phi_b__blk1555_dn2 = assign65310_body7_e100954_d_n2;
            locals.var_phi_b__blk1555_dn4 = assign65310_body7_e100954_d_n4;
            locals.var_phi_b__blk1555_dn5 = assign65310_body7_e100954_d_n5;
            locals.var_phi_b__blk1555_dn6 = assign65310_body7_e100954_d_n6;
            locals.var_phi_b__blk1555_dn7 = assign65310_body7_e100954_d_n7;
            locals.var_phi_b__blk1555_dn8 = assign65310_body7_e100954_d_n8;
            locals.var_phi_b__blk1555_dn9 = assign65310_body7_e100954_d_n9;
            locals.var_phi_b__blk1555_dn10 = assign65310_body7_e100954_d_n10;
            locals.var_phi_b__blk1555_dn11 = assign65310_body7_e100954_d_n11;
            locals.var_phi_b__blk1555_dn14 = assign65310_body7_e100954_d_n14;
            locals.var_phi_b__blk1555_rv = 0.0;
            let (assign65310_body8_e100975, assign65310_body8_e100975_d_n0, assign65310_body8_e100975_d_n2, assign65310_body8_e100975_d_n4, assign65310_body8_e100975_d_n5, assign65310_body8_e100975_d_n6, assign65310_body8_e100975_d_n7, assign65310_body8_e100975_d_n8, assign65310_body8_e100975_d_n9, assign65310_body8_e100975_d_n10, assign65310_body8_e100975_d_n11, assign65310_body8_e100975_d_n14,) = {
    if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1560 != 0.0)) {
        let assign65310_body8_e100972: f64 = (1.0 + locals.var_t2);
        let assign65310_body8_e100973: f64 = (locals.var_t1 / assign65310_body8_e100972);
        (assign65310_body8_e100973, (((locals.var_t1_dn0 * assign65310_body8_e100972) - (locals.var_t1 * locals.var_t2_dn0)) / (assign65310_body8_e100972 * assign65310_body8_e100972)), (((locals.var_t1_dn2 * assign65310_body8_e100972) - (locals.var_t1 * locals.var_t2_dn2)) / (assign65310_body8_e100972 * assign65310_body8_e100972)), (((locals.var_t1_dn4 * assign65310_body8_e100972) - (locals.var_t1 * locals.var_t2_dn4)) / (assign65310_body8_e100972 * assign65310_body8_e100972)), (((locals.var_t1_dn5 * assign65310_body8_e100972) - (locals.var_t1 * locals.var_t2_dn5)) / (assign65310_body8_e100972 * assign65310_body8_e100972)), (((locals.var_t1_dn6 * assign65310_body8_e100972) - (locals.var_t1 * locals.var_t2_dn6)) / (assign65310_body8_e100972 * assign65310_body8_e100972)), (((locals.var_t1_dn7 * assign65310_body8_e100972) - (locals.var_t1 * locals.var_t2_dn7)) / (assign65310_body8_e100972 * assign65310_body8_e100972)), (((locals.var_t1_dn8 * assign65310_body8_e100972) - (locals.var_t1 * locals.var_t2_dn8)) / (assign65310_body8_e100972 * assign65310_body8_e100972)), (((locals.var_t1_dn9 * assign65310_body8_e100972) - (locals.var_t1 * locals.var_t2_dn9)) / (assign65310_body8_e100972 * assign65310_body8_e100972)), (((locals.var_t1_dn10 * assign65310_body8_e100972) - (locals.var_t1 * locals.var_t2_dn10)) / (assign65310_body8_e100972 * assign65310_body8_e100972)), (((locals.var_t1_dn11 * assign65310_body8_e100972) - (locals.var_t1 * locals.var_t2_dn11)) / (assign65310_body8_e100972 * assign65310_body8_e100972)), (((locals.var_t1_dn14 * assign65310_body8_e100972) - (locals.var_t1 * locals.var_t2_dn14)) / (assign65310_body8_e100972 * assign65310_body8_e100972)),)
    } else {
        (locals.var_phi_b_dpss__blk1556, locals.var_phi_b_dpss__blk1556_dn0, locals.var_phi_b_dpss__blk1556_dn2, locals.var_phi_b_dpss__blk1556_dn4, locals.var_phi_b_dpss__blk1556_dn5, locals.var_phi_b_dpss__blk1556_dn6, locals.var_phi_b_dpss__blk1556_dn7, locals.var_phi_b_dpss__blk1556_dn8, locals.var_phi_b_dpss__blk1556_dn9, locals.var_phi_b_dpss__blk1556_dn10, locals.var_phi_b_dpss__blk1556_dn11, locals.var_phi_b_dpss__blk1556_dn14,)
    }
};
            locals.var_phi_b_dpss__blk1556 = assign65310_body8_e100975;
            locals.var_phi_b_dpss__blk1556_dn0 = assign65310_body8_e100975_d_n0;
            locals.var_phi_b_dpss__blk1556_dn2 = assign65310_body8_e100975_d_n2;
            locals.var_phi_b_dpss__blk1556_dn4 = assign65310_body8_e100975_d_n4;
            locals.var_phi_b_dpss__blk1556_dn5 = assign65310_body8_e100975_d_n5;
            locals.var_phi_b_dpss__blk1556_dn6 = assign65310_body8_e100975_d_n6;
            locals.var_phi_b_dpss__blk1556_dn7 = assign65310_body8_e100975_d_n7;
            locals.var_phi_b_dpss__blk1556_dn8 = assign65310_body8_e100975_d_n8;
            locals.var_phi_b_dpss__blk1556_dn9 = assign65310_body8_e100975_d_n9;
            locals.var_phi_b_dpss__blk1556_dn10 = assign65310_body8_e100975_d_n10;
            locals.var_phi_b_dpss__blk1556_dn11 = assign65310_body8_e100975_d_n11;
            locals.var_phi_b_dpss__blk1556_dn14 = assign65310_body8_e100975_d_n14;
            locals.var_phi_b_dpss__blk1556_rv = 0.0;
            let (assign65310_body9_e100995, assign65310_body9_e100995_d_n0, assign65310_body9_e100995_d_n2, assign65310_body9_e100995_d_n4, assign65310_body9_e100995_d_n5, assign65310_body9_e100995_d_n6, assign65310_body9_e100995_d_n7, assign65310_body9_e100995_d_n8, assign65310_body9_e100995_d_n9, assign65310_body9_e100995_d_n10, assign65310_body9_e100995_d_n11, assign65310_body9_e100995_d_n14,) = {
    if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1560 == 0.0)) {
        let assign65310_body9_e100993: f64 = (locals.var_phi_0 - locals.var_dphi_sb__blk1551);
        (assign65310_body9_e100993, (locals.var_phi_0_dn0 - locals.var_dphi_sb__blk1551_dn0), (locals.var_phi_0_dn2 - locals.var_dphi_sb__blk1551_dn2), (locals.var_phi_0_dn4 - locals.var_dphi_sb__blk1551_dn4), (locals.var_phi_0_dn5 - locals.var_dphi_sb__blk1551_dn5), (locals.var_phi_0_dn6 - locals.var_dphi_sb__blk1551_dn6), (locals.var_phi_0_dn7 - locals.var_dphi_sb__blk1551_dn7), (locals.var_phi_0_dn8 - locals.var_dphi_sb__blk1551_dn8), (locals.var_phi_0_dn9 - locals.var_dphi_sb__blk1551_dn9), (locals.var_phi_0_dn10 - locals.var_dphi_sb__blk1551_dn10), (locals.var_phi_0_dn11 - locals.var_dphi_sb__blk1551_dn11), (locals.var_phi_0_dn14 - locals.var_dphi_sb__blk1551_dn14),)
    } else {
        (locals.var_phi_b__blk1555, locals.var_phi_b__blk1555_dn0, locals.var_phi_b__blk1555_dn2, locals.var_phi_b__blk1555_dn4, locals.var_phi_b__blk1555_dn5, locals.var_phi_b__blk1555_dn6, locals.var_phi_b__blk1555_dn7, locals.var_phi_b__blk1555_dn8, locals.var_phi_b__blk1555_dn9, locals.var_phi_b__blk1555_dn10, locals.var_phi_b__blk1555_dn11, locals.var_phi_b__blk1555_dn14,)
    }
};
            locals.var_phi_b__blk1555 = assign65310_body9_e100995;
            locals.var_phi_b__blk1555_dn0 = assign65310_body9_e100995_d_n0;
            locals.var_phi_b__blk1555_dn2 = assign65310_body9_e100995_d_n2;
            locals.var_phi_b__blk1555_dn4 = assign65310_body9_e100995_d_n4;
            locals.var_phi_b__blk1555_dn5 = assign65310_body9_e100995_d_n5;
            locals.var_phi_b__blk1555_dn6 = assign65310_body9_e100995_d_n6;
            locals.var_phi_b__blk1555_dn7 = assign65310_body9_e100995_d_n7;
            locals.var_phi_b__blk1555_dn8 = assign65310_body9_e100995_d_n8;
            locals.var_phi_b__blk1555_dn9 = assign65310_body9_e100995_d_n9;
            locals.var_phi_b__blk1555_dn10 = assign65310_body9_e100995_d_n10;
            locals.var_phi_b__blk1555_dn11 = assign65310_body9_e100995_d_n11;
            locals.var_phi_b__blk1555_dn14 = assign65310_body9_e100995_d_n14;
            locals.var_phi_b__blk1555_rv = 0.0;
            let (assign65310_body10_e101013, assign65310_body10_e101013_d_n0, assign65310_body10_e101013_d_n2, assign65310_body10_e101013_d_n4, assign65310_body10_e101013_d_n5, assign65310_body10_e101013_d_n6, assign65310_body10_e101013_d_n7, assign65310_body10_e101013_d_n8, assign65310_body10_e101013_d_n9, assign65310_body10_e101013_d_n10, assign65310_body10_e101013_d_n11, assign65310_body10_e101013_d_n14,) = {
    if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1560 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b_dpss__blk1556, locals.var_phi_b_dpss__blk1556_dn0, locals.var_phi_b_dpss__blk1556_dn2, locals.var_phi_b_dpss__blk1556_dn4, locals.var_phi_b_dpss__blk1556_dn5, locals.var_phi_b_dpss__blk1556_dn6, locals.var_phi_b_dpss__blk1556_dn7, locals.var_phi_b_dpss__blk1556_dn8, locals.var_phi_b_dpss__blk1556_dn9, locals.var_phi_b_dpss__blk1556_dn10, locals.var_phi_b_dpss__blk1556_dn11, locals.var_phi_b_dpss__blk1556_dn14,)
    }
};
            locals.var_phi_b_dpss__blk1556 = assign65310_body10_e101013;
            locals.var_phi_b_dpss__blk1556_dn0 = assign65310_body10_e101013_d_n0;
            locals.var_phi_b_dpss__blk1556_dn2 = assign65310_body10_e101013_d_n2;
            locals.var_phi_b_dpss__blk1556_dn4 = assign65310_body10_e101013_d_n4;
            locals.var_phi_b_dpss__blk1556_dn5 = assign65310_body10_e101013_d_n5;
            locals.var_phi_b_dpss__blk1556_dn6 = assign65310_body10_e101013_d_n6;
            locals.var_phi_b_dpss__blk1556_dn7 = assign65310_body10_e101013_d_n7;
            locals.var_phi_b_dpss__blk1556_dn8 = assign65310_body10_e101013_d_n8;
            locals.var_phi_b_dpss__blk1556_dn9 = assign65310_body10_e101013_d_n9;
            locals.var_phi_b_dpss__blk1556_dn10 = assign65310_body10_e101013_d_n10;
            locals.var_phi_b_dpss__blk1556_dn11 = assign65310_body10_e101013_d_n11;
            locals.var_phi_b_dpss__blk1556_dn14 = assign65310_body10_e101013_d_n14;
            locals.var_phi_b_dpss__blk1556_rv = 0.0;
            let (assign65310_body11_e101030, assign65310_body11_e101030_d_n0, assign65310_body11_e101030_d_n2, assign65310_body11_e101030_d_n4, assign65310_body11_e101030_d_n5, assign65310_body11_e101030_d_n6, assign65310_body11_e101030_d_n7, assign65310_body11_e101030_d_n8, assign65310_body11_e101030_d_n9, assign65310_body11_e101030_d_n10, assign65310_body11_e101030_d_n11, assign65310_body11_e101030_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) {
        let assign65310_body11_e101028: f64 = (locals.var_beta * locals.var_phi_b__blk1555);
        (assign65310_body11_e101028, ((locals.var_beta_dn0 * locals.var_phi_b__blk1555) + (locals.var_beta * locals.var_phi_b__blk1555_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b__blk1555) + (locals.var_beta * locals.var_phi_b__blk1555_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b__blk1555) + (locals.var_beta * locals.var_phi_b__blk1555_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b__blk1555) + (locals.var_beta * locals.var_phi_b__blk1555_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b__blk1555) + (locals.var_beta * locals.var_phi_b__blk1555_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b__blk1555) + (locals.var_beta * locals.var_phi_b__blk1555_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b__blk1555) + (locals.var_beta * locals.var_phi_b__blk1555_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b__blk1555) + (locals.var_beta * locals.var_phi_b__blk1555_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b__blk1555) + (locals.var_beta * locals.var_phi_b__blk1555_dn10)), ((locals.var_beta_dn11 * locals.var_phi_b__blk1555) + (locals.var_beta * locals.var_phi_b__blk1555_dn11)), ((locals.var_beta_dn14 * locals.var_phi_b__blk1555) + (locals.var_beta * locals.var_phi_b__blk1555_dn14)),)
    } else {
        (locals.var_chib__blk1554, locals.var_chib__blk1554_dn0, locals.var_chib__blk1554_dn2, locals.var_chib__blk1554_dn4, locals.var_chib__blk1554_dn5, locals.var_chib__blk1554_dn6, locals.var_chib__blk1554_dn7, locals.var_chib__blk1554_dn8, locals.var_chib__blk1554_dn9, locals.var_chib__blk1554_dn10, locals.var_chib__blk1554_dn11, locals.var_chib__blk1554_dn14,)
    }
};
            locals.var_chib__blk1554 = assign65310_body11_e101030;
            locals.var_chib__blk1554_dn0 = assign65310_body11_e101030_d_n0;
            locals.var_chib__blk1554_dn2 = assign65310_body11_e101030_d_n2;
            locals.var_chib__blk1554_dn4 = assign65310_body11_e101030_d_n4;
            locals.var_chib__blk1554_dn5 = assign65310_body11_e101030_d_n5;
            locals.var_chib__blk1554_dn6 = assign65310_body11_e101030_d_n6;
            locals.var_chib__blk1554_dn7 = assign65310_body11_e101030_d_n7;
            locals.var_chib__blk1554_dn8 = assign65310_body11_e101030_d_n8;
            locals.var_chib__blk1554_dn9 = assign65310_body11_e101030_d_n9;
            locals.var_chib__blk1554_dn10 = assign65310_body11_e101030_d_n10;
            locals.var_chib__blk1554_dn11 = assign65310_body11_e101030_d_n11;
            locals.var_chib__blk1554_dn14 = assign65310_body11_e101030_d_n14;
            locals.var_chib__blk1554_rv = 0.0;
            let assign65310_body12_e101032: f64 = (locals.var_chi).abs();
            let assign65310_body12_e101034: f64 = if assign65310_body12_e101032 < 1e-16 { 1.0 } else { 0.0 };
            locals.var_guard1561 = assign65310_body12_e101034;
            locals.var_guard1561_rv = 0.0;
            let (assign65310_body13_e101058, assign65310_body13_e101058_d_n0, assign65310_body13_e101058_d_n2, assign65310_body13_e101058_d_n4, assign65310_body13_e101058_d_n5, assign65310_body13_e101058_d_n6, assign65310_body13_e101058_d_n7, assign65310_body13_e101058_d_n8, assign65310_body13_e101058_d_n9, assign65310_body13_e101058_d_n10, assign65310_body13_e101058_d_n11, assign65310_body13_e101058_d_n14,) = {
    if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1561 != 0.0)) {
        let assign65310_body13_e101052: f64 = (locals.var_phi_b_dpss__blk1556 * locals.var_phi_b_dpss__blk1556);
        let assign65310_body13_e101053: f64 = (1.0 - assign65310_body13_e101052);
        let assign65310_body13_e101055: f64 = (assign65310_body13_e101053 / 2.0);
        let assign65310_body13_e101056: f64 = (assign65310_body13_e101055).sqrt();
        (assign65310_body13_e101056, (((-((locals.var_phi_b_dpss__blk1556_dn0 * locals.var_phi_b_dpss__blk1556) + (locals.var_phi_b_dpss__blk1556 * locals.var_phi_b_dpss__blk1556_dn0))) / 2.0) / (2.0 * assign65310_body13_e101056)), (((-((locals.var_phi_b_dpss__blk1556_dn2 * locals.var_phi_b_dpss__blk1556) + (locals.var_phi_b_dpss__blk1556 * locals.var_phi_b_dpss__blk1556_dn2))) / 2.0) / (2.0 * assign65310_body13_e101056)), (((-((locals.var_phi_b_dpss__blk1556_dn4 * locals.var_phi_b_dpss__blk1556) + (locals.var_phi_b_dpss__blk1556 * locals.var_phi_b_dpss__blk1556_dn4))) / 2.0) / (2.0 * assign65310_body13_e101056)), (((-((locals.var_phi_b_dpss__blk1556_dn5 * locals.var_phi_b_dpss__blk1556) + (locals.var_phi_b_dpss__blk1556 * locals.var_phi_b_dpss__blk1556_dn5))) / 2.0) / (2.0 * assign65310_body13_e101056)), (((-((locals.var_phi_b_dpss__blk1556_dn6 * locals.var_phi_b_dpss__blk1556) + (locals.var_phi_b_dpss__blk1556 * locals.var_phi_b_dpss__blk1556_dn6))) / 2.0) / (2.0 * assign65310_body13_e101056)), (((-((locals.var_phi_b_dpss__blk1556_dn7 * locals.var_phi_b_dpss__blk1556) + (locals.var_phi_b_dpss__blk1556 * locals.var_phi_b_dpss__blk1556_dn7))) / 2.0) / (2.0 * assign65310_body13_e101056)), (((-((locals.var_phi_b_dpss__blk1556_dn8 * locals.var_phi_b_dpss__blk1556) + (locals.var_phi_b_dpss__blk1556 * locals.var_phi_b_dpss__blk1556_dn8))) / 2.0) / (2.0 * assign65310_body13_e101056)), (((-((locals.var_phi_b_dpss__blk1556_dn9 * locals.var_phi_b_dpss__blk1556) + (locals.var_phi_b_dpss__blk1556 * locals.var_phi_b_dpss__blk1556_dn9))) / 2.0) / (2.0 * assign65310_body13_e101056)), (((-((locals.var_phi_b_dpss__blk1556_dn10 * locals.var_phi_b_dpss__blk1556) + (locals.var_phi_b_dpss__blk1556 * locals.var_phi_b_dpss__blk1556_dn10))) / 2.0) / (2.0 * assign65310_body13_e101056)), (((-((locals.var_phi_b_dpss__blk1556_dn11 * locals.var_phi_b_dpss__blk1556) + (locals.var_phi_b_dpss__blk1556 * locals.var_phi_b_dpss__blk1556_dn11))) / 2.0) / (2.0 * assign65310_body13_e101056)), (((-((locals.var_phi_b_dpss__blk1556_dn14 * locals.var_phi_b_dpss__blk1556) + (locals.var_phi_b_dpss__blk1556 * locals.var_phi_b_dpss__blk1556_dn14))) / 2.0) / (2.0 * assign65310_body13_e101056)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign65310_body13_e101058;
            locals.var_t0_dn0 = assign65310_body13_e101058_d_n0;
            locals.var_t0_dn2 = assign65310_body13_e101058_d_n2;
            locals.var_t0_dn4 = assign65310_body13_e101058_d_n4;
            locals.var_t0_dn5 = assign65310_body13_e101058_d_n5;
            locals.var_t0_dn6 = assign65310_body13_e101058_d_n6;
            locals.var_t0_dn7 = assign65310_body13_e101058_d_n7;
            locals.var_t0_dn8 = assign65310_body13_e101058_d_n8;
            locals.var_t0_dn9 = assign65310_body13_e101058_d_n9;
            locals.var_t0_dn10 = assign65310_body13_e101058_d_n10;
            locals.var_t0_dn11 = assign65310_body13_e101058_d_n11;
            locals.var_t0_dn14 = assign65310_body13_e101058_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign65310_body14_e101077, assign65310_body14_e101077_d_n0, assign65310_body14_e101077_d_n2, assign65310_body14_e101077_d_n4, assign65310_body14_e101077_d_n5, assign65310_body14_e101077_d_n6, assign65310_body14_e101077_d_n7, assign65310_body14_e101077_d_n8, assign65310_body14_e101077_d_n9, assign65310_body14_e101077_d_n10, assign65310_body14_e101077_d_n11, assign65310_body14_e101077_d_n14,) = {
    if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1561 != 0.0)) {
        let assign65310_body14_e101075: f64 = (locals.var_chi * locals.var_t0);
        (assign65310_body14_e101075, ((locals.var_chi_dn0 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn0)), ((locals.var_chi_dn2 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn2)), ((locals.var_chi_dn4 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn4)), ((locals.var_chi_dn5 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn5)), ((locals.var_chi_dn6 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn6)), ((locals.var_chi_dn7 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn7)), ((locals.var_chi_dn8 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn8)), ((locals.var_chi_dn9 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn9)), ((locals.var_chi_dn10 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn10)), ((locals.var_chi_dn11 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn11)), ((locals.var_chi_dn14 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn14)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign65310_body14_e101077;
            locals.var_fb_dn0 = assign65310_body14_e101077_d_n0;
            locals.var_fb_dn2 = assign65310_body14_e101077_d_n2;
            locals.var_fb_dn4 = assign65310_body14_e101077_d_n4;
            locals.var_fb_dn5 = assign65310_body14_e101077_d_n5;
            locals.var_fb_dn6 = assign65310_body14_e101077_d_n6;
            locals.var_fb_dn7 = assign65310_body14_e101077_d_n7;
            locals.var_fb_dn8 = assign65310_body14_e101077_d_n8;
            locals.var_fb_dn9 = assign65310_body14_e101077_d_n9;
            locals.var_fb_dn10 = assign65310_body14_e101077_d_n10;
            locals.var_fb_dn11 = assign65310_body14_e101077_d_n11;
            locals.var_fb_dn14 = assign65310_body14_e101077_d_n14;
            locals.var_fb_rv = 0.0;
            let (assign65310_body15_e101096, assign65310_body15_e101096_d_n0, assign65310_body15_e101096_d_n2, assign65310_body15_e101096_d_n4, assign65310_body15_e101096_d_n5, assign65310_body15_e101096_d_n6, assign65310_body15_e101096_d_n7, assign65310_body15_e101096_d_n8, assign65310_body15_e101096_d_n9, assign65310_body15_e101096_d_n10, assign65310_body15_e101096_d_n11, assign65310_body15_e101096_d_n14,) = {
    if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1561 != 0.0)) {
        let assign65310_body15_e101094: f64 = (locals.var_beta * locals.var_t0);
        (assign65310_body15_e101094, ((locals.var_beta_dn0 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn0)), ((locals.var_beta_dn2 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn2)), ((locals.var_beta_dn4 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn4)), ((locals.var_beta_dn5 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn5)), ((locals.var_beta_dn6 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn6)), ((locals.var_beta_dn7 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn7)), ((locals.var_beta_dn8 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn8)), ((locals.var_beta_dn9 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn9)), ((locals.var_beta_dn10 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn10)), ((locals.var_beta_dn11 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn11)), ((locals.var_beta_dn14 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn14)),)
    } else {
        (locals.var_fb_dpss__blk1557, locals.var_fb_dpss__blk1557_dn0, locals.var_fb_dpss__blk1557_dn2, locals.var_fb_dpss__blk1557_dn4, locals.var_fb_dpss__blk1557_dn5, locals.var_fb_dpss__blk1557_dn6, locals.var_fb_dpss__blk1557_dn7, locals.var_fb_dpss__blk1557_dn8, locals.var_fb_dpss__blk1557_dn9, locals.var_fb_dpss__blk1557_dn10, locals.var_fb_dpss__blk1557_dn11, locals.var_fb_dpss__blk1557_dn14,)
    }
};
            locals.var_fb_dpss__blk1557 = assign65310_body15_e101096;
            locals.var_fb_dpss__blk1557_dn0 = assign65310_body15_e101096_d_n0;
            locals.var_fb_dpss__blk1557_dn2 = assign65310_body15_e101096_d_n2;
            locals.var_fb_dpss__blk1557_dn4 = assign65310_body15_e101096_d_n4;
            locals.var_fb_dpss__blk1557_dn5 = assign65310_body15_e101096_d_n5;
            locals.var_fb_dpss__blk1557_dn6 = assign65310_body15_e101096_d_n6;
            locals.var_fb_dpss__blk1557_dn7 = assign65310_body15_e101096_d_n7;
            locals.var_fb_dpss__blk1557_dn8 = assign65310_body15_e101096_d_n8;
            locals.var_fb_dpss__blk1557_dn9 = assign65310_body15_e101096_d_n9;
            locals.var_fb_dpss__blk1557_dn10 = assign65310_body15_e101096_d_n10;
            locals.var_fb_dpss__blk1557_dn11 = assign65310_body15_e101096_d_n11;
            locals.var_fb_dpss__blk1557_dn14 = assign65310_body15_e101096_d_n14;
            locals.var_fb_dpss__blk1557_rv = 0.0;
            let assign65310_body16_e101099: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1562 = assign65310_body16_e101099;
            locals.var_guard1562_rv = 0.0;
            let (assign65310_body17_e101119, assign65310_body17_e101119_d_n0, assign65310_body17_e101119_d_n2, assign65310_body17_e101119_d_n4, assign65310_body17_e101119_d_n5, assign65310_body17_e101119_d_n6, assign65310_body17_e101119_d_n7, assign65310_body17_e101119_d_n8, assign65310_body17_e101119_d_n9, assign65310_body17_e101119_d_n10, assign65310_body17_e101119_d_n11, assign65310_body17_e101119_d_n14,) = {
    if (((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1561 != 0.0)) && (locals.var_guard1562 != 0.0)) {
        let assign65310_body17_e101117: f64 = (-locals.var_fb);
        (assign65310_body17_e101117, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn4), (-locals.var_fb_dn5), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn8), (-locals.var_fb_dn9), (-locals.var_fb_dn10), (-locals.var_fb_dn11), (-locals.var_fb_dn14),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign65310_body17_e101119;
            locals.var_fb_dn0 = assign65310_body17_e101119_d_n0;
            locals.var_fb_dn2 = assign65310_body17_e101119_d_n2;
            locals.var_fb_dn4 = assign65310_body17_e101119_d_n4;
            locals.var_fb_dn5 = assign65310_body17_e101119_d_n5;
            locals.var_fb_dn6 = assign65310_body17_e101119_d_n6;
            locals.var_fb_dn7 = assign65310_body17_e101119_d_n7;
            locals.var_fb_dn8 = assign65310_body17_e101119_d_n8;
            locals.var_fb_dn9 = assign65310_body17_e101119_d_n9;
            locals.var_fb_dn10 = assign65310_body17_e101119_d_n10;
            locals.var_fb_dn11 = assign65310_body17_e101119_d_n11;
            locals.var_fb_dn14 = assign65310_body17_e101119_d_n14;
            locals.var_fb_rv = 0.0;
            let (assign65310_body18_e101139, assign65310_body18_e101139_d_n0, assign65310_body18_e101139_d_n2, assign65310_body18_e101139_d_n4, assign65310_body18_e101139_d_n5, assign65310_body18_e101139_d_n6, assign65310_body18_e101139_d_n7, assign65310_body18_e101139_d_n8, assign65310_body18_e101139_d_n9, assign65310_body18_e101139_d_n10, assign65310_body18_e101139_d_n11, assign65310_body18_e101139_d_n14,) = {
    if (((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1561 != 0.0)) && (locals.var_guard1562 != 0.0)) {
        let assign65310_body18_e101137: f64 = (-locals.var_fb_dpss__blk1557);
        (assign65310_body18_e101137, (-locals.var_fb_dpss__blk1557_dn0), (-locals.var_fb_dpss__blk1557_dn2), (-locals.var_fb_dpss__blk1557_dn4), (-locals.var_fb_dpss__blk1557_dn5), (-locals.var_fb_dpss__blk1557_dn6), (-locals.var_fb_dpss__blk1557_dn7), (-locals.var_fb_dpss__blk1557_dn8), (-locals.var_fb_dpss__blk1557_dn9), (-locals.var_fb_dpss__blk1557_dn10), (-locals.var_fb_dpss__blk1557_dn11), (-locals.var_fb_dpss__blk1557_dn14),)
    } else {
        (locals.var_fb_dpss__blk1557, locals.var_fb_dpss__blk1557_dn0, locals.var_fb_dpss__blk1557_dn2, locals.var_fb_dpss__blk1557_dn4, locals.var_fb_dpss__blk1557_dn5, locals.var_fb_dpss__blk1557_dn6, locals.var_fb_dpss__blk1557_dn7, locals.var_fb_dpss__blk1557_dn8, locals.var_fb_dpss__blk1557_dn9, locals.var_fb_dpss__blk1557_dn10, locals.var_fb_dpss__blk1557_dn11, locals.var_fb_dpss__blk1557_dn14,)
    }
};
            locals.var_fb_dpss__blk1557 = assign65310_body18_e101139;
            locals.var_fb_dpss__blk1557_dn0 = assign65310_body18_e101139_d_n0;
            locals.var_fb_dpss__blk1557_dn2 = assign65310_body18_e101139_d_n2;
            locals.var_fb_dpss__blk1557_dn4 = assign65310_body18_e101139_d_n4;
            locals.var_fb_dpss__blk1557_dn5 = assign65310_body18_e101139_d_n5;
            locals.var_fb_dpss__blk1557_dn6 = assign65310_body18_e101139_d_n6;
            locals.var_fb_dpss__blk1557_dn7 = assign65310_body18_e101139_d_n7;
            locals.var_fb_dpss__blk1557_dn8 = assign65310_body18_e101139_d_n8;
            locals.var_fb_dpss__blk1557_dn9 = assign65310_body18_e101139_d_n9;
            locals.var_fb_dpss__blk1557_dn10 = assign65310_body18_e101139_d_n10;
            locals.var_fb_dpss__blk1557_dn11 = assign65310_body18_e101139_d_n11;
            locals.var_fb_dpss__blk1557_dn14 = assign65310_body18_e101139_d_n14;
            locals.var_fb_dpss__blk1557_rv = 0.0;
            let assign65310_body19_e101141: f64 = (locals.var_chi).abs();
            let assign65310_body19_e101143: f64 = if assign65310_body19_e101141 < 0.005 { 1.0 } else { 0.0 };
            locals.var_guard1563 = assign65310_body19_e101143;
            locals.var_guard1563_rv = 0.0;
            let (assign65310_body20_e101185, assign65310_body20_e101185_d_n0, assign65310_body20_e101185_d_n2, assign65310_body20_e101185_d_n4, assign65310_body20_e101185_d_n5, assign65310_body20_e101185_d_n6, assign65310_body20_e101185_d_n7, assign65310_body20_e101185_d_n8, assign65310_body20_e101185_d_n9, assign65310_body20_e101185_d_n10, assign65310_body20_e101185_d_n11, assign65310_body20_e101185_d_n14,) = {
    if (((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1561 == 0.0)) && (locals.var_guard1563 != 0.0)) {
        let assign65310_body20_e101163: f64 = (locals.var_chi * locals.var_chi);
        let assign65310_body20_e101165: f64 = (assign65310_body20_e101163 / 2.0);
        let assign65310_body20_e101169: f64 = (locals.var_chi / 3.0);
        let assign65310_body20_e101173: f64 = (locals.var_chi / 4.0);
        let assign65310_body20_e101177: f64 = (locals.var_chi / 5.0);
        let assign65310_body20_e101178: f64 = (1.0 - assign65310_body20_e101177);
        let assign65310_body20_e101179: f64 = (assign65310_body20_e101173 * assign65310_body20_e101178);
        let assign65310_body20_e101180: f64 = (1.0 - assign65310_body20_e101179);
        let assign65310_body20_e101181: f64 = (assign65310_body20_e101169 * assign65310_body20_e101180);
        let assign65310_body20_e101182: f64 = (1.0 - assign65310_body20_e101181);
        let assign65310_body20_e101183: f64 = (assign65310_body20_e101165 * assign65310_body20_e101182);
        (assign65310_body20_e101183, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign65310_body20_e101182) + (assign65310_body20_e101165 * (-(((locals.var_chi_dn0 / 3.0) * assign65310_body20_e101180) + (assign65310_body20_e101169 * (-(((locals.var_chi_dn0 / 4.0) * assign65310_body20_e101178) + (assign65310_body20_e101173 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign65310_body20_e101182) + (assign65310_body20_e101165 * (-(((locals.var_chi_dn2 / 3.0) * assign65310_body20_e101180) + (assign65310_body20_e101169 * (-(((locals.var_chi_dn2 / 4.0) * assign65310_body20_e101178) + (assign65310_body20_e101173 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign65310_body20_e101182) + (assign65310_body20_e101165 * (-(((locals.var_chi_dn4 / 3.0) * assign65310_body20_e101180) + (assign65310_body20_e101169 * (-(((locals.var_chi_dn4 / 4.0) * assign65310_body20_e101178) + (assign65310_body20_e101173 * (-(locals.var_chi_dn4 / 5.0)))))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign65310_body20_e101182) + (assign65310_body20_e101165 * (-(((locals.var_chi_dn5 / 3.0) * assign65310_body20_e101180) + (assign65310_body20_e101169 * (-(((locals.var_chi_dn5 / 4.0) * assign65310_body20_e101178) + (assign65310_body20_e101173 * (-(locals.var_chi_dn5 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign65310_body20_e101182) + (assign65310_body20_e101165 * (-(((locals.var_chi_dn6 / 3.0) * assign65310_body20_e101180) + (assign65310_body20_e101169 * (-(((locals.var_chi_dn6 / 4.0) * assign65310_body20_e101178) + (assign65310_body20_e101173 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign65310_body20_e101182) + (assign65310_body20_e101165 * (-(((locals.var_chi_dn7 / 3.0) * assign65310_body20_e101180) + (assign65310_body20_e101169 * (-(((locals.var_chi_dn7 / 4.0) * assign65310_body20_e101178) + (assign65310_body20_e101173 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign65310_body20_e101182) + (assign65310_body20_e101165 * (-(((locals.var_chi_dn8 / 3.0) * assign65310_body20_e101180) + (assign65310_body20_e101169 * (-(((locals.var_chi_dn8 / 4.0) * assign65310_body20_e101178) + (assign65310_body20_e101173 * (-(locals.var_chi_dn8 / 5.0)))))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign65310_body20_e101182) + (assign65310_body20_e101165 * (-(((locals.var_chi_dn9 / 3.0) * assign65310_body20_e101180) + (assign65310_body20_e101169 * (-(((locals.var_chi_dn9 / 4.0) * assign65310_body20_e101178) + (assign65310_body20_e101173 * (-(locals.var_chi_dn9 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign65310_body20_e101182) + (assign65310_body20_e101165 * (-(((locals.var_chi_dn10 / 3.0) * assign65310_body20_e101180) + (assign65310_body20_e101169 * (-(((locals.var_chi_dn10 / 4.0) * assign65310_body20_e101178) + (assign65310_body20_e101173 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) / 2.0) * assign65310_body20_e101182) + (assign65310_body20_e101165 * (-(((locals.var_chi_dn11 / 3.0) * assign65310_body20_e101180) + (assign65310_body20_e101169 * (-(((locals.var_chi_dn11 / 4.0) * assign65310_body20_e101178) + (assign65310_body20_e101173 * (-(locals.var_chi_dn11 / 5.0)))))))))), (((((locals.var_chi_dn14 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn14)) / 2.0) * assign65310_body20_e101182) + (assign65310_body20_e101165 * (-(((locals.var_chi_dn14 / 3.0) * assign65310_body20_e101180) + (assign65310_body20_e101169 * (-(((locals.var_chi_dn14 / 4.0) * assign65310_body20_e101178) + (assign65310_body20_e101173 * (-(locals.var_chi_dn14 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign65310_body20_e101185;
            locals.var_t0_dn0 = assign65310_body20_e101185_d_n0;
            locals.var_t0_dn2 = assign65310_body20_e101185_d_n2;
            locals.var_t0_dn4 = assign65310_body20_e101185_d_n4;
            locals.var_t0_dn5 = assign65310_body20_e101185_d_n5;
            locals.var_t0_dn6 = assign65310_body20_e101185_d_n6;
            locals.var_t0_dn7 = assign65310_body20_e101185_d_n7;
            locals.var_t0_dn8 = assign65310_body20_e101185_d_n8;
            locals.var_t0_dn9 = assign65310_body20_e101185_d_n9;
            locals.var_t0_dn10 = assign65310_body20_e101185_d_n10;
            locals.var_t0_dn11 = assign65310_body20_e101185_d_n11;
            locals.var_t0_dn14 = assign65310_body20_e101185_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign65310_body21_e101223, assign65310_body21_e101223_d_n0, assign65310_body21_e101223_d_n2, assign65310_body21_e101223_d_n4, assign65310_body21_e101223_d_n5, assign65310_body21_e101223_d_n6, assign65310_body21_e101223_d_n7, assign65310_body21_e101223_d_n8, assign65310_body21_e101223_d_n9, assign65310_body21_e101223_d_n10, assign65310_body21_e101223_d_n11, assign65310_body21_e101223_d_n14,) = {
    if (((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1561 == 0.0)) && (locals.var_guard1563 != 0.0)) {
        let assign65310_body21_e101207: f64 = (locals.var_chi / 2.0);
        let assign65310_body21_e101211: f64 = (locals.var_chi / 3.0);
        let assign65310_body21_e101215: f64 = (locals.var_chi / 4.0);
        let assign65310_body21_e101216: f64 = (1.0 - assign65310_body21_e101215);
        let assign65310_body21_e101217: f64 = (assign65310_body21_e101211 * assign65310_body21_e101216);
        let assign65310_body21_e101218: f64 = (1.0 - assign65310_body21_e101217);
        let assign65310_body21_e101219: f64 = (assign65310_body21_e101207 * assign65310_body21_e101218);
        let assign65310_body21_e101220: f64 = (1.0 - assign65310_body21_e101219);
        let assign65310_body21_e101221: f64 = (locals.var_chi * assign65310_body21_e101220);
        (assign65310_body21_e101221, ((locals.var_chi_dn0 * assign65310_body21_e101220) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign65310_body21_e101218) + (assign65310_body21_e101207 * (-(((locals.var_chi_dn0 / 3.0) * assign65310_body21_e101216) + (assign65310_body21_e101211 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign65310_body21_e101220) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign65310_body21_e101218) + (assign65310_body21_e101207 * (-(((locals.var_chi_dn2 / 3.0) * assign65310_body21_e101216) + (assign65310_body21_e101211 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn4 * assign65310_body21_e101220) + (locals.var_chi * (-(((locals.var_chi_dn4 / 2.0) * assign65310_body21_e101218) + (assign65310_body21_e101207 * (-(((locals.var_chi_dn4 / 3.0) * assign65310_body21_e101216) + (assign65310_body21_e101211 * (-(locals.var_chi_dn4 / 4.0)))))))))), ((locals.var_chi_dn5 * assign65310_body21_e101220) + (locals.var_chi * (-(((locals.var_chi_dn5 / 2.0) * assign65310_body21_e101218) + (assign65310_body21_e101207 * (-(((locals.var_chi_dn5 / 3.0) * assign65310_body21_e101216) + (assign65310_body21_e101211 * (-(locals.var_chi_dn5 / 4.0)))))))))), ((locals.var_chi_dn6 * assign65310_body21_e101220) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign65310_body21_e101218) + (assign65310_body21_e101207 * (-(((locals.var_chi_dn6 / 3.0) * assign65310_body21_e101216) + (assign65310_body21_e101211 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign65310_body21_e101220) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign65310_body21_e101218) + (assign65310_body21_e101207 * (-(((locals.var_chi_dn7 / 3.0) * assign65310_body21_e101216) + (assign65310_body21_e101211 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn8 * assign65310_body21_e101220) + (locals.var_chi * (-(((locals.var_chi_dn8 / 2.0) * assign65310_body21_e101218) + (assign65310_body21_e101207 * (-(((locals.var_chi_dn8 / 3.0) * assign65310_body21_e101216) + (assign65310_body21_e101211 * (-(locals.var_chi_dn8 / 4.0)))))))))), ((locals.var_chi_dn9 * assign65310_body21_e101220) + (locals.var_chi * (-(((locals.var_chi_dn9 / 2.0) * assign65310_body21_e101218) + (assign65310_body21_e101207 * (-(((locals.var_chi_dn9 / 3.0) * assign65310_body21_e101216) + (assign65310_body21_e101211 * (-(locals.var_chi_dn9 / 4.0)))))))))), ((locals.var_chi_dn10 * assign65310_body21_e101220) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign65310_body21_e101218) + (assign65310_body21_e101207 * (-(((locals.var_chi_dn10 / 3.0) * assign65310_body21_e101216) + (assign65310_body21_e101211 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn11 * assign65310_body21_e101220) + (locals.var_chi * (-(((locals.var_chi_dn11 / 2.0) * assign65310_body21_e101218) + (assign65310_body21_e101207 * (-(((locals.var_chi_dn11 / 3.0) * assign65310_body21_e101216) + (assign65310_body21_e101211 * (-(locals.var_chi_dn11 / 4.0)))))))))), ((locals.var_chi_dn14 * assign65310_body21_e101220) + (locals.var_chi * (-(((locals.var_chi_dn14 / 2.0) * assign65310_body21_e101218) + (assign65310_body21_e101207 * (-(((locals.var_chi_dn14 / 3.0) * assign65310_body21_e101216) + (assign65310_body21_e101211 * (-(locals.var_chi_dn14 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign65310_body21_e101223;
            locals.var_t1_dn0 = assign65310_body21_e101223_d_n0;
            locals.var_t1_dn2 = assign65310_body21_e101223_d_n2;
            locals.var_t1_dn4 = assign65310_body21_e101223_d_n4;
            locals.var_t1_dn5 = assign65310_body21_e101223_d_n5;
            locals.var_t1_dn6 = assign65310_body21_e101223_d_n6;
            locals.var_t1_dn7 = assign65310_body21_e101223_d_n7;
            locals.var_t1_dn8 = assign65310_body21_e101223_d_n8;
            locals.var_t1_dn9 = assign65310_body21_e101223_d_n9;
            locals.var_t1_dn10 = assign65310_body21_e101223_d_n10;
            locals.var_t1_dn11 = assign65310_body21_e101223_d_n11;
            locals.var_t1_dn14 = assign65310_body21_e101223_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign65310_body22_e101265, assign65310_body22_e101265_d_n0, assign65310_body22_e101265_d_n2, assign65310_body22_e101265_d_n4, assign65310_body22_e101265_d_n5, assign65310_body22_e101265_d_n6, assign65310_body22_e101265_d_n7, assign65310_body22_e101265_d_n8, assign65310_body22_e101265_d_n9, assign65310_body22_e101265_d_n10, assign65310_body22_e101265_d_n11, assign65310_body22_e101265_d_n14,) = {
    if (((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1561 == 0.0)) && (locals.var_guard1563 != 0.0)) {
        let assign65310_body22_e101243: f64 = (locals.var_chib__blk1554 * locals.var_chib__blk1554);
        let assign65310_body22_e101245: f64 = (assign65310_body22_e101243 / 2.0);
        let assign65310_body22_e101249: f64 = (locals.var_chib__blk1554 / 3.0);
        let assign65310_body22_e101253: f64 = (locals.var_chib__blk1554 / 4.0);
        let assign65310_body22_e101257: f64 = (locals.var_chib__blk1554 / 5.0);
        let assign65310_body22_e101258: f64 = (1.0 - assign65310_body22_e101257);
        let assign65310_body22_e101259: f64 = (assign65310_body22_e101253 * assign65310_body22_e101258);
        let assign65310_body22_e101260: f64 = (1.0 - assign65310_body22_e101259);
        let assign65310_body22_e101261: f64 = (assign65310_body22_e101249 * assign65310_body22_e101260);
        let assign65310_body22_e101262: f64 = (1.0 - assign65310_body22_e101261);
        let assign65310_body22_e101263: f64 = (assign65310_body22_e101245 * assign65310_body22_e101262);
        (assign65310_body22_e101263, (((((locals.var_chib__blk1554_dn0 * locals.var_chib__blk1554) + (locals.var_chib__blk1554 * locals.var_chib__blk1554_dn0)) / 2.0) * assign65310_body22_e101262) + (assign65310_body22_e101245 * (-(((locals.var_chib__blk1554_dn0 / 3.0) * assign65310_body22_e101260) + (assign65310_body22_e101249 * (-(((locals.var_chib__blk1554_dn0 / 4.0) * assign65310_body22_e101258) + (assign65310_body22_e101253 * (-(locals.var_chib__blk1554_dn0 / 5.0)))))))))), (((((locals.var_chib__blk1554_dn2 * locals.var_chib__blk1554) + (locals.var_chib__blk1554 * locals.var_chib__blk1554_dn2)) / 2.0) * assign65310_body22_e101262) + (assign65310_body22_e101245 * (-(((locals.var_chib__blk1554_dn2 / 3.0) * assign65310_body22_e101260) + (assign65310_body22_e101249 * (-(((locals.var_chib__blk1554_dn2 / 4.0) * assign65310_body22_e101258) + (assign65310_body22_e101253 * (-(locals.var_chib__blk1554_dn2 / 5.0)))))))))), (((((locals.var_chib__blk1554_dn4 * locals.var_chib__blk1554) + (locals.var_chib__blk1554 * locals.var_chib__blk1554_dn4)) / 2.0) * assign65310_body22_e101262) + (assign65310_body22_e101245 * (-(((locals.var_chib__blk1554_dn4 / 3.0) * assign65310_body22_e101260) + (assign65310_body22_e101249 * (-(((locals.var_chib__blk1554_dn4 / 4.0) * assign65310_body22_e101258) + (assign65310_body22_e101253 * (-(locals.var_chib__blk1554_dn4 / 5.0)))))))))), (((((locals.var_chib__blk1554_dn5 * locals.var_chib__blk1554) + (locals.var_chib__blk1554 * locals.var_chib__blk1554_dn5)) / 2.0) * assign65310_body22_e101262) + (assign65310_body22_e101245 * (-(((locals.var_chib__blk1554_dn5 / 3.0) * assign65310_body22_e101260) + (assign65310_body22_e101249 * (-(((locals.var_chib__blk1554_dn5 / 4.0) * assign65310_body22_e101258) + (assign65310_body22_e101253 * (-(locals.var_chib__blk1554_dn5 / 5.0)))))))))), (((((locals.var_chib__blk1554_dn6 * locals.var_chib__blk1554) + (locals.var_chib__blk1554 * locals.var_chib__blk1554_dn6)) / 2.0) * assign65310_body22_e101262) + (assign65310_body22_e101245 * (-(((locals.var_chib__blk1554_dn6 / 3.0) * assign65310_body22_e101260) + (assign65310_body22_e101249 * (-(((locals.var_chib__blk1554_dn6 / 4.0) * assign65310_body22_e101258) + (assign65310_body22_e101253 * (-(locals.var_chib__blk1554_dn6 / 5.0)))))))))), (((((locals.var_chib__blk1554_dn7 * locals.var_chib__blk1554) + (locals.var_chib__blk1554 * locals.var_chib__blk1554_dn7)) / 2.0) * assign65310_body22_e101262) + (assign65310_body22_e101245 * (-(((locals.var_chib__blk1554_dn7 / 3.0) * assign65310_body22_e101260) + (assign65310_body22_e101249 * (-(((locals.var_chib__blk1554_dn7 / 4.0) * assign65310_body22_e101258) + (assign65310_body22_e101253 * (-(locals.var_chib__blk1554_dn7 / 5.0)))))))))), (((((locals.var_chib__blk1554_dn8 * locals.var_chib__blk1554) + (locals.var_chib__blk1554 * locals.var_chib__blk1554_dn8)) / 2.0) * assign65310_body22_e101262) + (assign65310_body22_e101245 * (-(((locals.var_chib__blk1554_dn8 / 3.0) * assign65310_body22_e101260) + (assign65310_body22_e101249 * (-(((locals.var_chib__blk1554_dn8 / 4.0) * assign65310_body22_e101258) + (assign65310_body22_e101253 * (-(locals.var_chib__blk1554_dn8 / 5.0)))))))))), (((((locals.var_chib__blk1554_dn9 * locals.var_chib__blk1554) + (locals.var_chib__blk1554 * locals.var_chib__blk1554_dn9)) / 2.0) * assign65310_body22_e101262) + (assign65310_body22_e101245 * (-(((locals.var_chib__blk1554_dn9 / 3.0) * assign65310_body22_e101260) + (assign65310_body22_e101249 * (-(((locals.var_chib__blk1554_dn9 / 4.0) * assign65310_body22_e101258) + (assign65310_body22_e101253 * (-(locals.var_chib__blk1554_dn9 / 5.0)))))))))), (((((locals.var_chib__blk1554_dn10 * locals.var_chib__blk1554) + (locals.var_chib__blk1554 * locals.var_chib__blk1554_dn10)) / 2.0) * assign65310_body22_e101262) + (assign65310_body22_e101245 * (-(((locals.var_chib__blk1554_dn10 / 3.0) * assign65310_body22_e101260) + (assign65310_body22_e101249 * (-(((locals.var_chib__blk1554_dn10 / 4.0) * assign65310_body22_e101258) + (assign65310_body22_e101253 * (-(locals.var_chib__blk1554_dn10 / 5.0)))))))))), (((((locals.var_chib__blk1554_dn11 * locals.var_chib__blk1554) + (locals.var_chib__blk1554 * locals.var_chib__blk1554_dn11)) / 2.0) * assign65310_body22_e101262) + (assign65310_body22_e101245 * (-(((locals.var_chib__blk1554_dn11 / 3.0) * assign65310_body22_e101260) + (assign65310_body22_e101249 * (-(((locals.var_chib__blk1554_dn11 / 4.0) * assign65310_body22_e101258) + (assign65310_body22_e101253 * (-(locals.var_chib__blk1554_dn11 / 5.0)))))))))), (((((locals.var_chib__blk1554_dn14 * locals.var_chib__blk1554) + (locals.var_chib__blk1554 * locals.var_chib__blk1554_dn14)) / 2.0) * assign65310_body22_e101262) + (assign65310_body22_e101245 * (-(((locals.var_chib__blk1554_dn14 / 3.0) * assign65310_body22_e101260) + (assign65310_body22_e101249 * (-(((locals.var_chib__blk1554_dn14 / 4.0) * assign65310_body22_e101258) + (assign65310_body22_e101253 * (-(locals.var_chib__blk1554_dn14 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign65310_body22_e101265;
            locals.var_t2_dn0 = assign65310_body22_e101265_d_n0;
            locals.var_t2_dn2 = assign65310_body22_e101265_d_n2;
            locals.var_t2_dn4 = assign65310_body22_e101265_d_n4;
            locals.var_t2_dn5 = assign65310_body22_e101265_d_n5;
            locals.var_t2_dn6 = assign65310_body22_e101265_d_n6;
            locals.var_t2_dn7 = assign65310_body22_e101265_d_n7;
            locals.var_t2_dn8 = assign65310_body22_e101265_d_n8;
            locals.var_t2_dn9 = assign65310_body22_e101265_d_n9;
            locals.var_t2_dn10 = assign65310_body22_e101265_d_n10;
            locals.var_t2_dn11 = assign65310_body22_e101265_d_n11;
            locals.var_t2_dn14 = assign65310_body22_e101265_d_n14;
            locals.var_t2_rv = 0.0;
            let (assign65310_body23_e101303, assign65310_body23_e101303_d_n0, assign65310_body23_e101303_d_n2, assign65310_body23_e101303_d_n4, assign65310_body23_e101303_d_n5, assign65310_body23_e101303_d_n6, assign65310_body23_e101303_d_n7, assign65310_body23_e101303_d_n8, assign65310_body23_e101303_d_n9, assign65310_body23_e101303_d_n10, assign65310_body23_e101303_d_n11, assign65310_body23_e101303_d_n14,) = {
    if (((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1561 == 0.0)) && (locals.var_guard1563 != 0.0)) {
        let assign65310_body23_e101287: f64 = (locals.var_chib__blk1554 / 2.0);
        let assign65310_body23_e101291: f64 = (locals.var_chib__blk1554 / 3.0);
        let assign65310_body23_e101295: f64 = (locals.var_chib__blk1554 / 4.0);
        let assign65310_body23_e101296: f64 = (1.0 - assign65310_body23_e101295);
        let assign65310_body23_e101297: f64 = (assign65310_body23_e101291 * assign65310_body23_e101296);
        let assign65310_body23_e101298: f64 = (1.0 - assign65310_body23_e101297);
        let assign65310_body23_e101299: f64 = (assign65310_body23_e101287 * assign65310_body23_e101298);
        let assign65310_body23_e101300: f64 = (1.0 - assign65310_body23_e101299);
        let assign65310_body23_e101301: f64 = (locals.var_chib__blk1554 * assign65310_body23_e101300);
        (assign65310_body23_e101301, ((locals.var_chib__blk1554_dn0 * assign65310_body23_e101300) + (locals.var_chib__blk1554 * (-(((locals.var_chib__blk1554_dn0 / 2.0) * assign65310_body23_e101298) + (assign65310_body23_e101287 * (-(((locals.var_chib__blk1554_dn0 / 3.0) * assign65310_body23_e101296) + (assign65310_body23_e101291 * (-(locals.var_chib__blk1554_dn0 / 4.0)))))))))), ((locals.var_chib__blk1554_dn2 * assign65310_body23_e101300) + (locals.var_chib__blk1554 * (-(((locals.var_chib__blk1554_dn2 / 2.0) * assign65310_body23_e101298) + (assign65310_body23_e101287 * (-(((locals.var_chib__blk1554_dn2 / 3.0) * assign65310_body23_e101296) + (assign65310_body23_e101291 * (-(locals.var_chib__blk1554_dn2 / 4.0)))))))))), ((locals.var_chib__blk1554_dn4 * assign65310_body23_e101300) + (locals.var_chib__blk1554 * (-(((locals.var_chib__blk1554_dn4 / 2.0) * assign65310_body23_e101298) + (assign65310_body23_e101287 * (-(((locals.var_chib__blk1554_dn4 / 3.0) * assign65310_body23_e101296) + (assign65310_body23_e101291 * (-(locals.var_chib__blk1554_dn4 / 4.0)))))))))), ((locals.var_chib__blk1554_dn5 * assign65310_body23_e101300) + (locals.var_chib__blk1554 * (-(((locals.var_chib__blk1554_dn5 / 2.0) * assign65310_body23_e101298) + (assign65310_body23_e101287 * (-(((locals.var_chib__blk1554_dn5 / 3.0) * assign65310_body23_e101296) + (assign65310_body23_e101291 * (-(locals.var_chib__blk1554_dn5 / 4.0)))))))))), ((locals.var_chib__blk1554_dn6 * assign65310_body23_e101300) + (locals.var_chib__blk1554 * (-(((locals.var_chib__blk1554_dn6 / 2.0) * assign65310_body23_e101298) + (assign65310_body23_e101287 * (-(((locals.var_chib__blk1554_dn6 / 3.0) * assign65310_body23_e101296) + (assign65310_body23_e101291 * (-(locals.var_chib__blk1554_dn6 / 4.0)))))))))), ((locals.var_chib__blk1554_dn7 * assign65310_body23_e101300) + (locals.var_chib__blk1554 * (-(((locals.var_chib__blk1554_dn7 / 2.0) * assign65310_body23_e101298) + (assign65310_body23_e101287 * (-(((locals.var_chib__blk1554_dn7 / 3.0) * assign65310_body23_e101296) + (assign65310_body23_e101291 * (-(locals.var_chib__blk1554_dn7 / 4.0)))))))))), ((locals.var_chib__blk1554_dn8 * assign65310_body23_e101300) + (locals.var_chib__blk1554 * (-(((locals.var_chib__blk1554_dn8 / 2.0) * assign65310_body23_e101298) + (assign65310_body23_e101287 * (-(((locals.var_chib__blk1554_dn8 / 3.0) * assign65310_body23_e101296) + (assign65310_body23_e101291 * (-(locals.var_chib__blk1554_dn8 / 4.0)))))))))), ((locals.var_chib__blk1554_dn9 * assign65310_body23_e101300) + (locals.var_chib__blk1554 * (-(((locals.var_chib__blk1554_dn9 / 2.0) * assign65310_body23_e101298) + (assign65310_body23_e101287 * (-(((locals.var_chib__blk1554_dn9 / 3.0) * assign65310_body23_e101296) + (assign65310_body23_e101291 * (-(locals.var_chib__blk1554_dn9 / 4.0)))))))))), ((locals.var_chib__blk1554_dn10 * assign65310_body23_e101300) + (locals.var_chib__blk1554 * (-(((locals.var_chib__blk1554_dn10 / 2.0) * assign65310_body23_e101298) + (assign65310_body23_e101287 * (-(((locals.var_chib__blk1554_dn10 / 3.0) * assign65310_body23_e101296) + (assign65310_body23_e101291 * (-(locals.var_chib__blk1554_dn10 / 4.0)))))))))), ((locals.var_chib__blk1554_dn11 * assign65310_body23_e101300) + (locals.var_chib__blk1554 * (-(((locals.var_chib__blk1554_dn11 / 2.0) * assign65310_body23_e101298) + (assign65310_body23_e101287 * (-(((locals.var_chib__blk1554_dn11 / 3.0) * assign65310_body23_e101296) + (assign65310_body23_e101291 * (-(locals.var_chib__blk1554_dn11 / 4.0)))))))))), ((locals.var_chib__blk1554_dn14 * assign65310_body23_e101300) + (locals.var_chib__blk1554 * (-(((locals.var_chib__blk1554_dn14 / 2.0) * assign65310_body23_e101298) + (assign65310_body23_e101287 * (-(((locals.var_chib__blk1554_dn14 / 3.0) * assign65310_body23_e101296) + (assign65310_body23_e101291 * (-(locals.var_chib__blk1554_dn14 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
            locals.var_t3 = assign65310_body23_e101303;
            locals.var_t3_dn0 = assign65310_body23_e101303_d_n0;
            locals.var_t3_dn2 = assign65310_body23_e101303_d_n2;
            locals.var_t3_dn4 = assign65310_body23_e101303_d_n4;
            locals.var_t3_dn5 = assign65310_body23_e101303_d_n5;
            locals.var_t3_dn6 = assign65310_body23_e101303_d_n6;
            locals.var_t3_dn7 = assign65310_body23_e101303_d_n7;
            locals.var_t3_dn8 = assign65310_body23_e101303_d_n8;
            locals.var_t3_dn9 = assign65310_body23_e101303_d_n9;
            locals.var_t3_dn10 = assign65310_body23_e101303_d_n10;
            locals.var_t3_dn11 = assign65310_body23_e101303_d_n11;
            locals.var_t3_dn14 = assign65310_body23_e101303_d_n14;
            locals.var_t3_rv = 0.0;
            let (assign65310_body24_e101326, assign65310_body24_e101326_d_n0, assign65310_body24_e101326_d_n2, assign65310_body24_e101326_d_n4, assign65310_body24_e101326_d_n5, assign65310_body24_e101326_d_n6, assign65310_body24_e101326_d_n7, assign65310_body24_e101326_d_n8, assign65310_body24_e101326_d_n9, assign65310_body24_e101326_d_n10, assign65310_body24_e101326_d_n11, assign65310_body24_e101326_d_n14,) = {
    if (((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1561 == 0.0)) && (locals.var_guard1563 != 0.0)) {
        let assign65310_body24_e101323: f64 = (locals.var_t0 - locals.var_t2);
        let assign65310_body24_e101324: f64 = (assign65310_body24_e101323).sqrt();
        (assign65310_body24_e101324, ((locals.var_t0_dn0 - locals.var_t2_dn0) / (2.0 * assign65310_body24_e101324)), ((locals.var_t0_dn2 - locals.var_t2_dn2) / (2.0 * assign65310_body24_e101324)), ((locals.var_t0_dn4 - locals.var_t2_dn4) / (2.0 * assign65310_body24_e101324)), ((locals.var_t0_dn5 - locals.var_t2_dn5) / (2.0 * assign65310_body24_e101324)), ((locals.var_t0_dn6 - locals.var_t2_dn6) / (2.0 * assign65310_body24_e101324)), ((locals.var_t0_dn7 - locals.var_t2_dn7) / (2.0 * assign65310_body24_e101324)), ((locals.var_t0_dn8 - locals.var_t2_dn8) / (2.0 * assign65310_body24_e101324)), ((locals.var_t0_dn9 - locals.var_t2_dn9) / (2.0 * assign65310_body24_e101324)), ((locals.var_t0_dn10 - locals.var_t2_dn10) / (2.0 * assign65310_body24_e101324)), ((locals.var_t0_dn11 - locals.var_t2_dn11) / (2.0 * assign65310_body24_e101324)), ((locals.var_t0_dn14 - locals.var_t2_dn14) / (2.0 * assign65310_body24_e101324)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign65310_body24_e101326;
            locals.var_fb_dn0 = assign65310_body24_e101326_d_n0;
            locals.var_fb_dn2 = assign65310_body24_e101326_d_n2;
            locals.var_fb_dn4 = assign65310_body24_e101326_d_n4;
            locals.var_fb_dn5 = assign65310_body24_e101326_d_n5;
            locals.var_fb_dn6 = assign65310_body24_e101326_d_n6;
            locals.var_fb_dn7 = assign65310_body24_e101326_d_n7;
            locals.var_fb_dn8 = assign65310_body24_e101326_d_n8;
            locals.var_fb_dn9 = assign65310_body24_e101326_d_n9;
            locals.var_fb_dn10 = assign65310_body24_e101326_d_n10;
            locals.var_fb_dn11 = assign65310_body24_e101326_d_n11;
            locals.var_fb_dn14 = assign65310_body24_e101326_d_n14;
            locals.var_fb_rv = 0.0;
            let (assign65310_body25_e101356, assign65310_body25_e101356_d_n0, assign65310_body25_e101356_d_n2, assign65310_body25_e101356_d_n4, assign65310_body25_e101356_d_n5, assign65310_body25_e101356_d_n6, assign65310_body25_e101356_d_n7, assign65310_body25_e101356_d_n8, assign65310_body25_e101356_d_n9, assign65310_body25_e101356_d_n10, assign65310_body25_e101356_d_n11, assign65310_body25_e101356_d_n14,) = {
    if (((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1561 == 0.0)) && (locals.var_guard1563 != 0.0)) {
        let assign65310_body25_e101346: f64 = (locals.var_beta * 0.5);
        let assign65310_body25_e101350: f64 = (locals.var_phi_b_dpss__blk1556 * locals.var_t3);
        let assign65310_body25_e101351: f64 = (locals.var_t1 - assign65310_body25_e101350);
        let assign65310_body25_e101352: f64 = (assign65310_body25_e101346 * assign65310_body25_e101351);
        let assign65310_body25_e101354: f64 = (assign65310_body25_e101352 / locals.var_fb);
        (assign65310_body25_e101354, ((((((locals.var_beta_dn0 * 0.5) * assign65310_body25_e101351) + (assign65310_body25_e101346 * (locals.var_t1_dn0 - ((locals.var_phi_b_dpss__blk1556_dn0 * locals.var_t3) + (locals.var_phi_b_dpss__blk1556 * locals.var_t3_dn0))))) * locals.var_fb) - (assign65310_body25_e101352 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign65310_body25_e101351) + (assign65310_body25_e101346 * (locals.var_t1_dn2 - ((locals.var_phi_b_dpss__blk1556_dn2 * locals.var_t3) + (locals.var_phi_b_dpss__blk1556 * locals.var_t3_dn2))))) * locals.var_fb) - (assign65310_body25_e101352 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign65310_body25_e101351) + (assign65310_body25_e101346 * (locals.var_t1_dn4 - ((locals.var_phi_b_dpss__blk1556_dn4 * locals.var_t3) + (locals.var_phi_b_dpss__blk1556 * locals.var_t3_dn4))))) * locals.var_fb) - (assign65310_body25_e101352 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign65310_body25_e101351) + (assign65310_body25_e101346 * (locals.var_t1_dn5 - ((locals.var_phi_b_dpss__blk1556_dn5 * locals.var_t3) + (locals.var_phi_b_dpss__blk1556 * locals.var_t3_dn5))))) * locals.var_fb) - (assign65310_body25_e101352 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign65310_body25_e101351) + (assign65310_body25_e101346 * (locals.var_t1_dn6 - ((locals.var_phi_b_dpss__blk1556_dn6 * locals.var_t3) + (locals.var_phi_b_dpss__blk1556 * locals.var_t3_dn6))))) * locals.var_fb) - (assign65310_body25_e101352 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign65310_body25_e101351) + (assign65310_body25_e101346 * (locals.var_t1_dn7 - ((locals.var_phi_b_dpss__blk1556_dn7 * locals.var_t3) + (locals.var_phi_b_dpss__blk1556 * locals.var_t3_dn7))))) * locals.var_fb) - (assign65310_body25_e101352 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign65310_body25_e101351) + (assign65310_body25_e101346 * (locals.var_t1_dn8 - ((locals.var_phi_b_dpss__blk1556_dn8 * locals.var_t3) + (locals.var_phi_b_dpss__blk1556 * locals.var_t3_dn8))))) * locals.var_fb) - (assign65310_body25_e101352 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign65310_body25_e101351) + (assign65310_body25_e101346 * (locals.var_t1_dn9 - ((locals.var_phi_b_dpss__blk1556_dn9 * locals.var_t3) + (locals.var_phi_b_dpss__blk1556 * locals.var_t3_dn9))))) * locals.var_fb) - (assign65310_body25_e101352 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign65310_body25_e101351) + (assign65310_body25_e101346 * (locals.var_t1_dn10 - ((locals.var_phi_b_dpss__blk1556_dn10 * locals.var_t3) + (locals.var_phi_b_dpss__blk1556 * locals.var_t3_dn10))))) * locals.var_fb) - (assign65310_body25_e101352 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn11 * 0.5) * assign65310_body25_e101351) + (assign65310_body25_e101346 * (locals.var_t1_dn11 - ((locals.var_phi_b_dpss__blk1556_dn11 * locals.var_t3) + (locals.var_phi_b_dpss__blk1556 * locals.var_t3_dn11))))) * locals.var_fb) - (assign65310_body25_e101352 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn14 * 0.5) * assign65310_body25_e101351) + (assign65310_body25_e101346 * (locals.var_t1_dn14 - ((locals.var_phi_b_dpss__blk1556_dn14 * locals.var_t3) + (locals.var_phi_b_dpss__blk1556 * locals.var_t3_dn14))))) * locals.var_fb) - (assign65310_body25_e101352 * locals.var_fb_dn14)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss__blk1557, locals.var_fb_dpss__blk1557_dn0, locals.var_fb_dpss__blk1557_dn2, locals.var_fb_dpss__blk1557_dn4, locals.var_fb_dpss__blk1557_dn5, locals.var_fb_dpss__blk1557_dn6, locals.var_fb_dpss__blk1557_dn7, locals.var_fb_dpss__blk1557_dn8, locals.var_fb_dpss__blk1557_dn9, locals.var_fb_dpss__blk1557_dn10, locals.var_fb_dpss__blk1557_dn11, locals.var_fb_dpss__blk1557_dn14,)
    }
};
            locals.var_fb_dpss__blk1557 = assign65310_body25_e101356;
            locals.var_fb_dpss__blk1557_dn0 = assign65310_body25_e101356_d_n0;
            locals.var_fb_dpss__blk1557_dn2 = assign65310_body25_e101356_d_n2;
            locals.var_fb_dpss__blk1557_dn4 = assign65310_body25_e101356_d_n4;
            locals.var_fb_dpss__blk1557_dn5 = assign65310_body25_e101356_d_n5;
            locals.var_fb_dpss__blk1557_dn6 = assign65310_body25_e101356_d_n6;
            locals.var_fb_dpss__blk1557_dn7 = assign65310_body25_e101356_d_n7;
            locals.var_fb_dpss__blk1557_dn8 = assign65310_body25_e101356_d_n8;
            locals.var_fb_dpss__blk1557_dn9 = assign65310_body25_e101356_d_n9;
            locals.var_fb_dpss__blk1557_dn10 = assign65310_body25_e101356_d_n10;
            locals.var_fb_dpss__blk1557_dn11 = assign65310_body25_e101356_d_n11;
            locals.var_fb_dpss__blk1557_dn14 = assign65310_body25_e101356_d_n14;
            locals.var_fb_dpss__blk1557_rv = 0.0;
            let (assign65310_body26_e101379, assign65310_body26_e101379_d_n0, assign65310_body26_e101379_d_n2, assign65310_body26_e101379_d_n4, assign65310_body26_e101379_d_n5, assign65310_body26_e101379_d_n6, assign65310_body26_e101379_d_n7, assign65310_body26_e101379_d_n8, assign65310_body26_e101379_d_n9, assign65310_body26_e101379_d_n10, assign65310_body26_e101379_d_n11, assign65310_body26_e101379_d_n14,) = {
    if (((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1561 == 0.0)) && (locals.var_guard1563 == 0.0)) {
        let assign65310_body26_e101376: f64 = (-locals.var_chi);
        let assign65310_body26_e101377: f64 = (assign65310_body26_e101376).exp();
        (assign65310_body26_e101377, (assign65310_body26_e101377 * (-locals.var_chi_dn0)), (assign65310_body26_e101377 * (-locals.var_chi_dn2)), (assign65310_body26_e101377 * (-locals.var_chi_dn4)), (assign65310_body26_e101377 * (-locals.var_chi_dn5)), (assign65310_body26_e101377 * (-locals.var_chi_dn6)), (assign65310_body26_e101377 * (-locals.var_chi_dn7)), (assign65310_body26_e101377 * (-locals.var_chi_dn8)), (assign65310_body26_e101377 * (-locals.var_chi_dn9)), (assign65310_body26_e101377 * (-locals.var_chi_dn10)), (assign65310_body26_e101377 * (-locals.var_chi_dn11)), (assign65310_body26_e101377 * (-locals.var_chi_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign65310_body26_e101379;
            locals.var_t0_dn0 = assign65310_body26_e101379_d_n0;
            locals.var_t0_dn2 = assign65310_body26_e101379_d_n2;
            locals.var_t0_dn4 = assign65310_body26_e101379_d_n4;
            locals.var_t0_dn5 = assign65310_body26_e101379_d_n5;
            locals.var_t0_dn6 = assign65310_body26_e101379_d_n6;
            locals.var_t0_dn7 = assign65310_body26_e101379_d_n7;
            locals.var_t0_dn8 = assign65310_body26_e101379_d_n8;
            locals.var_t0_dn9 = assign65310_body26_e101379_d_n9;
            locals.var_t0_dn10 = assign65310_body26_e101379_d_n10;
            locals.var_t0_dn11 = assign65310_body26_e101379_d_n11;
            locals.var_t0_dn14 = assign65310_body26_e101379_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign65310_body27_e101402, assign65310_body27_e101402_d_n0, assign65310_body27_e101402_d_n2, assign65310_body27_e101402_d_n4, assign65310_body27_e101402_d_n5, assign65310_body27_e101402_d_n6, assign65310_body27_e101402_d_n7, assign65310_body27_e101402_d_n8, assign65310_body27_e101402_d_n9, assign65310_body27_e101402_d_n10, assign65310_body27_e101402_d_n11, assign65310_body27_e101402_d_n14,) = {
    if (((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1561 == 0.0)) && (locals.var_guard1563 == 0.0)) {
        let assign65310_body27_e101399: f64 = (-locals.var_chib__blk1554);
        let assign65310_body27_e101400: f64 = (assign65310_body27_e101399).exp();
        (assign65310_body27_e101400, (assign65310_body27_e101400 * (-locals.var_chib__blk1554_dn0)), (assign65310_body27_e101400 * (-locals.var_chib__blk1554_dn2)), (assign65310_body27_e101400 * (-locals.var_chib__blk1554_dn4)), (assign65310_body27_e101400 * (-locals.var_chib__blk1554_dn5)), (assign65310_body27_e101400 * (-locals.var_chib__blk1554_dn6)), (assign65310_body27_e101400 * (-locals.var_chib__blk1554_dn7)), (assign65310_body27_e101400 * (-locals.var_chib__blk1554_dn8)), (assign65310_body27_e101400 * (-locals.var_chib__blk1554_dn9)), (assign65310_body27_e101400 * (-locals.var_chib__blk1554_dn10)), (assign65310_body27_e101400 * (-locals.var_chib__blk1554_dn11)), (assign65310_body27_e101400 * (-locals.var_chib__blk1554_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign65310_body27_e101402;
            locals.var_t1_dn0 = assign65310_body27_e101402_d_n0;
            locals.var_t1_dn2 = assign65310_body27_e101402_d_n2;
            locals.var_t1_dn4 = assign65310_body27_e101402_d_n4;
            locals.var_t1_dn5 = assign65310_body27_e101402_d_n5;
            locals.var_t1_dn6 = assign65310_body27_e101402_d_n6;
            locals.var_t1_dn7 = assign65310_body27_e101402_d_n7;
            locals.var_t1_dn8 = assign65310_body27_e101402_d_n8;
            locals.var_t1_dn9 = assign65310_body27_e101402_d_n9;
            locals.var_t1_dn10 = assign65310_body27_e101402_d_n10;
            locals.var_t1_dn11 = assign65310_body27_e101402_d_n11;
            locals.var_t1_dn14 = assign65310_body27_e101402_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign65310_body28_e101430, assign65310_body28_e101430_d_n0, assign65310_body28_e101430_d_n2, assign65310_body28_e101430_d_n4, assign65310_body28_e101430_d_n5, assign65310_body28_e101430_d_n6, assign65310_body28_e101430_d_n7, assign65310_body28_e101430_d_n8, assign65310_body28_e101430_d_n9, assign65310_body28_e101430_d_n10, assign65310_body28_e101430_d_n11, assign65310_body28_e101430_d_n14,) = {
    if (((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1561 == 0.0)) && (locals.var_guard1563 == 0.0)) {
        let assign65310_body28_e101423: f64 = (locals.var_chi - locals.var_chib__blk1554);
        let assign65310_body28_e101426: f64 = (locals.var_t0 - locals.var_t1);
        let assign65310_body28_e101427: f64 = (assign65310_body28_e101423 + assign65310_body28_e101426);
        let assign65310_body28_e101428: f64 = (assign65310_body28_e101427).sqrt();
        (assign65310_body28_e101428, (((locals.var_chi_dn0 - locals.var_chib__blk1554_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)) / (2.0 * assign65310_body28_e101428)), (((locals.var_chi_dn2 - locals.var_chib__blk1554_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)) / (2.0 * assign65310_body28_e101428)), (((locals.var_chi_dn4 - locals.var_chib__blk1554_dn4) + (locals.var_t0_dn4 - locals.var_t1_dn4)) / (2.0 * assign65310_body28_e101428)), (((locals.var_chi_dn5 - locals.var_chib__blk1554_dn5) + (locals.var_t0_dn5 - locals.var_t1_dn5)) / (2.0 * assign65310_body28_e101428)), (((locals.var_chi_dn6 - locals.var_chib__blk1554_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)) / (2.0 * assign65310_body28_e101428)), (((locals.var_chi_dn7 - locals.var_chib__blk1554_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)) / (2.0 * assign65310_body28_e101428)), (((locals.var_chi_dn8 - locals.var_chib__blk1554_dn8) + (locals.var_t0_dn8 - locals.var_t1_dn8)) / (2.0 * assign65310_body28_e101428)), (((locals.var_chi_dn9 - locals.var_chib__blk1554_dn9) + (locals.var_t0_dn9 - locals.var_t1_dn9)) / (2.0 * assign65310_body28_e101428)), (((locals.var_chi_dn10 - locals.var_chib__blk1554_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)) / (2.0 * assign65310_body28_e101428)), (((locals.var_chi_dn11 - locals.var_chib__blk1554_dn11) + (locals.var_t0_dn11 - locals.var_t1_dn11)) / (2.0 * assign65310_body28_e101428)), (((locals.var_chi_dn14 - locals.var_chib__blk1554_dn14) + (locals.var_t0_dn14 - locals.var_t1_dn14)) / (2.0 * assign65310_body28_e101428)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign65310_body28_e101430;
            locals.var_fb_dn0 = assign65310_body28_e101430_d_n0;
            locals.var_fb_dn2 = assign65310_body28_e101430_d_n2;
            locals.var_fb_dn4 = assign65310_body28_e101430_d_n4;
            locals.var_fb_dn5 = assign65310_body28_e101430_d_n5;
            locals.var_fb_dn6 = assign65310_body28_e101430_d_n6;
            locals.var_fb_dn7 = assign65310_body28_e101430_d_n7;
            locals.var_fb_dn8 = assign65310_body28_e101430_d_n8;
            locals.var_fb_dn9 = assign65310_body28_e101430_d_n9;
            locals.var_fb_dn10 = assign65310_body28_e101430_d_n10;
            locals.var_fb_dn11 = assign65310_body28_e101430_d_n11;
            locals.var_fb_dn14 = assign65310_body28_e101430_d_n14;
            locals.var_fb_rv = 0.0;
            let (assign65310_body29_e101465, assign65310_body29_e101465_d_n0, assign65310_body29_e101465_d_n2, assign65310_body29_e101465_d_n4, assign65310_body29_e101465_d_n5, assign65310_body29_e101465_d_n6, assign65310_body29_e101465_d_n7, assign65310_body29_e101465_d_n8, assign65310_body29_e101465_d_n9, assign65310_body29_e101465_d_n10, assign65310_body29_e101465_d_n11, assign65310_body29_e101465_d_n14,) = {
    if (((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1561 == 0.0)) && (locals.var_guard1563 == 0.0)) {
        let assign65310_body29_e101451: f64 = (locals.var_beta * 0.5);
        let assign65310_body29_e101454: f64 = (1.0 - locals.var_t0);
        let assign65310_body29_e101458: f64 = (1.0 - locals.var_t1);
        let assign65310_body29_e101459: f64 = (locals.var_phi_b_dpss__blk1556 * assign65310_body29_e101458);
        let assign65310_body29_e101460: f64 = (assign65310_body29_e101454 - assign65310_body29_e101459);
        let assign65310_body29_e101461: f64 = (assign65310_body29_e101451 * assign65310_body29_e101460);
        let assign65310_body29_e101463: f64 = (assign65310_body29_e101461 / locals.var_fb);
        (assign65310_body29_e101463, ((((((locals.var_beta_dn0 * 0.5) * assign65310_body29_e101460) + (assign65310_body29_e101451 * ((-locals.var_t0_dn0) - ((locals.var_phi_b_dpss__blk1556_dn0 * assign65310_body29_e101458) + (locals.var_phi_b_dpss__blk1556 * (-locals.var_t1_dn0)))))) * locals.var_fb) - (assign65310_body29_e101461 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign65310_body29_e101460) + (assign65310_body29_e101451 * ((-locals.var_t0_dn2) - ((locals.var_phi_b_dpss__blk1556_dn2 * assign65310_body29_e101458) + (locals.var_phi_b_dpss__blk1556 * (-locals.var_t1_dn2)))))) * locals.var_fb) - (assign65310_body29_e101461 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign65310_body29_e101460) + (assign65310_body29_e101451 * ((-locals.var_t0_dn4) - ((locals.var_phi_b_dpss__blk1556_dn4 * assign65310_body29_e101458) + (locals.var_phi_b_dpss__blk1556 * (-locals.var_t1_dn4)))))) * locals.var_fb) - (assign65310_body29_e101461 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign65310_body29_e101460) + (assign65310_body29_e101451 * ((-locals.var_t0_dn5) - ((locals.var_phi_b_dpss__blk1556_dn5 * assign65310_body29_e101458) + (locals.var_phi_b_dpss__blk1556 * (-locals.var_t1_dn5)))))) * locals.var_fb) - (assign65310_body29_e101461 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign65310_body29_e101460) + (assign65310_body29_e101451 * ((-locals.var_t0_dn6) - ((locals.var_phi_b_dpss__blk1556_dn6 * assign65310_body29_e101458) + (locals.var_phi_b_dpss__blk1556 * (-locals.var_t1_dn6)))))) * locals.var_fb) - (assign65310_body29_e101461 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign65310_body29_e101460) + (assign65310_body29_e101451 * ((-locals.var_t0_dn7) - ((locals.var_phi_b_dpss__blk1556_dn7 * assign65310_body29_e101458) + (locals.var_phi_b_dpss__blk1556 * (-locals.var_t1_dn7)))))) * locals.var_fb) - (assign65310_body29_e101461 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign65310_body29_e101460) + (assign65310_body29_e101451 * ((-locals.var_t0_dn8) - ((locals.var_phi_b_dpss__blk1556_dn8 * assign65310_body29_e101458) + (locals.var_phi_b_dpss__blk1556 * (-locals.var_t1_dn8)))))) * locals.var_fb) - (assign65310_body29_e101461 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign65310_body29_e101460) + (assign65310_body29_e101451 * ((-locals.var_t0_dn9) - ((locals.var_phi_b_dpss__blk1556_dn9 * assign65310_body29_e101458) + (locals.var_phi_b_dpss__blk1556 * (-locals.var_t1_dn9)))))) * locals.var_fb) - (assign65310_body29_e101461 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign65310_body29_e101460) + (assign65310_body29_e101451 * ((-locals.var_t0_dn10) - ((locals.var_phi_b_dpss__blk1556_dn10 * assign65310_body29_e101458) + (locals.var_phi_b_dpss__blk1556 * (-locals.var_t1_dn10)))))) * locals.var_fb) - (assign65310_body29_e101461 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn11 * 0.5) * assign65310_body29_e101460) + (assign65310_body29_e101451 * ((-locals.var_t0_dn11) - ((locals.var_phi_b_dpss__blk1556_dn11 * assign65310_body29_e101458) + (locals.var_phi_b_dpss__blk1556 * (-locals.var_t1_dn11)))))) * locals.var_fb) - (assign65310_body29_e101461 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn14 * 0.5) * assign65310_body29_e101460) + (assign65310_body29_e101451 * ((-locals.var_t0_dn14) - ((locals.var_phi_b_dpss__blk1556_dn14 * assign65310_body29_e101458) + (locals.var_phi_b_dpss__blk1556 * (-locals.var_t1_dn14)))))) * locals.var_fb) - (assign65310_body29_e101461 * locals.var_fb_dn14)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss__blk1557, locals.var_fb_dpss__blk1557_dn0, locals.var_fb_dpss__blk1557_dn2, locals.var_fb_dpss__blk1557_dn4, locals.var_fb_dpss__blk1557_dn5, locals.var_fb_dpss__blk1557_dn6, locals.var_fb_dpss__blk1557_dn7, locals.var_fb_dpss__blk1557_dn8, locals.var_fb_dpss__blk1557_dn9, locals.var_fb_dpss__blk1557_dn10, locals.var_fb_dpss__blk1557_dn11, locals.var_fb_dpss__blk1557_dn14,)
    }
};
            locals.var_fb_dpss__blk1557 = assign65310_body29_e101465;
            locals.var_fb_dpss__blk1557_dn0 = assign65310_body29_e101465_d_n0;
            locals.var_fb_dpss__blk1557_dn2 = assign65310_body29_e101465_d_n2;
            locals.var_fb_dpss__blk1557_dn4 = assign65310_body29_e101465_d_n4;
            locals.var_fb_dpss__blk1557_dn5 = assign65310_body29_e101465_d_n5;
            locals.var_fb_dpss__blk1557_dn6 = assign65310_body29_e101465_d_n6;
            locals.var_fb_dpss__blk1557_dn7 = assign65310_body29_e101465_d_n7;
            locals.var_fb_dpss__blk1557_dn8 = assign65310_body29_e101465_d_n8;
            locals.var_fb_dpss__blk1557_dn9 = assign65310_body29_e101465_d_n9;
            locals.var_fb_dpss__blk1557_dn10 = assign65310_body29_e101465_d_n10;
            locals.var_fb_dpss__blk1557_dn11 = assign65310_body29_e101465_d_n11;
            locals.var_fb_dpss__blk1557_dn14 = assign65310_body29_e101465_d_n14;
            locals.var_fb_dpss__blk1557_rv = 0.0;
            let assign65310_body30_e101472: f64 = if ((locals.var_flg_conv == 1.0) && (locals.var_chi < 0.0)) { 1.0 } else { 0.0 };
            locals.var_guard1564 = assign65310_body30_e101472;
            locals.var_guard1564_rv = 0.0;
            let (assign65310_body31_e101490,) = {
    if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1564 != 0.0)) {
        let assign65310_body31_e101488: f64 = (-1.0);
        (assign65310_body31_e101488,)
    } else {
        (locals.var_flg_zone,)
    }
};
            locals.var_flg_zone = assign65310_body31_e101490;
            locals.var_flg_zone_rv = 0.0;
            let assign65310_body32_e101493: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1565 = assign65310_body32_e101493;
            locals.var_guard1565_rv = 0.0;
            let (assign65310_body33_e101511, assign65310_body33_e101511_d_n0, assign65310_body33_e101511_d_n2, assign65310_body33_e101511_d_n4, assign65310_body33_e101511_d_n5, assign65310_body33_e101511_d_n6, assign65310_body33_e101511_d_n7, assign65310_body33_e101511_d_n8, assign65310_body33_e101511_d_n9, assign65310_body33_e101511_d_n10, assign65310_body33_e101511_d_n11, assign65310_body33_e101511_d_n14,) = {
    if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1565 != 0.0)) {
        let assign65310_body33_e101509: f64 = (-locals.var_fb);
        (assign65310_body33_e101509, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn4), (-locals.var_fb_dn5), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn8), (-locals.var_fb_dn9), (-locals.var_fb_dn10), (-locals.var_fb_dn11), (-locals.var_fb_dn14),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign65310_body33_e101511;
            locals.var_fs02_dn0 = assign65310_body33_e101511_d_n0;
            locals.var_fs02_dn2 = assign65310_body33_e101511_d_n2;
            locals.var_fs02_dn4 = assign65310_body33_e101511_d_n4;
            locals.var_fs02_dn5 = assign65310_body33_e101511_d_n5;
            locals.var_fs02_dn6 = assign65310_body33_e101511_d_n6;
            locals.var_fs02_dn7 = assign65310_body33_e101511_d_n7;
            locals.var_fs02_dn8 = assign65310_body33_e101511_d_n8;
            locals.var_fs02_dn9 = assign65310_body33_e101511_d_n9;
            locals.var_fs02_dn10 = assign65310_body33_e101511_d_n10;
            locals.var_fs02_dn11 = assign65310_body33_e101511_d_n11;
            locals.var_fs02_dn14 = assign65310_body33_e101511_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign65310_body34_e101529, assign65310_body34_e101529_d_n0, assign65310_body34_e101529_d_n2, assign65310_body34_e101529_d_n4, assign65310_body34_e101529_d_n5, assign65310_body34_e101529_d_n6, assign65310_body34_e101529_d_n7, assign65310_body34_e101529_d_n8, assign65310_body34_e101529_d_n9, assign65310_body34_e101529_d_n10, assign65310_body34_e101529_d_n11, assign65310_body34_e101529_d_n14,) = {
    if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1565 != 0.0)) {
        let assign65310_body34_e101527: f64 = (-locals.var_fb_dpss__blk1557);
        (assign65310_body34_e101527, (-locals.var_fb_dpss__blk1557_dn0), (-locals.var_fb_dpss__blk1557_dn2), (-locals.var_fb_dpss__blk1557_dn4), (-locals.var_fb_dpss__blk1557_dn5), (-locals.var_fb_dpss__blk1557_dn6), (-locals.var_fb_dpss__blk1557_dn7), (-locals.var_fb_dpss__blk1557_dn8), (-locals.var_fb_dpss__blk1557_dn9), (-locals.var_fb_dpss__blk1557_dn10), (-locals.var_fb_dpss__blk1557_dn11), (-locals.var_fb_dpss__blk1557_dn14),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign65310_body34_e101529;
            locals.var_fs02_dps0_dn0 = assign65310_body34_e101529_d_n0;
            locals.var_fs02_dps0_dn2 = assign65310_body34_e101529_d_n2;
            locals.var_fs02_dps0_dn4 = assign65310_body34_e101529_d_n4;
            locals.var_fs02_dps0_dn5 = assign65310_body34_e101529_d_n5;
            locals.var_fs02_dps0_dn6 = assign65310_body34_e101529_d_n6;
            locals.var_fs02_dps0_dn7 = assign65310_body34_e101529_d_n7;
            locals.var_fs02_dps0_dn8 = assign65310_body34_e101529_d_n8;
            locals.var_fs02_dps0_dn9 = assign65310_body34_e101529_d_n9;
            locals.var_fs02_dps0_dn10 = assign65310_body34_e101529_d_n10;
            locals.var_fs02_dps0_dn11 = assign65310_body34_e101529_d_n11;
            locals.var_fs02_dps0_dn14 = assign65310_body34_e101529_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let assign65310_body35_e101532: f64 = if locals.var_chi < 1e-7 { 1.0 } else { 0.0 };
            locals.var_guard1566 = assign65310_body35_e101532;
            locals.var_guard1566_rv = 0.0;
            let (assign65310_body36_e101552, assign65310_body36_e101552_d_n0, assign65310_body36_e101552_d_n2, assign65310_body36_e101552_d_n4, assign65310_body36_e101552_d_n5, assign65310_body36_e101552_d_n6, assign65310_body36_e101552_d_n7, assign65310_body36_e101552_d_n8, assign65310_body36_e101552_d_n9, assign65310_body36_e101552_d_n10, assign65310_body36_e101552_d_n11, assign65310_body36_e101552_d_n14,) = {
    if (((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1565 == 0.0)) && (locals.var_guard1566 != 0.0)) {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign65310_body36_e101552;
            locals.var_fs02_dn0 = assign65310_body36_e101552_d_n0;
            locals.var_fs02_dn2 = assign65310_body36_e101552_d_n2;
            locals.var_fs02_dn4 = assign65310_body36_e101552_d_n4;
            locals.var_fs02_dn5 = assign65310_body36_e101552_d_n5;
            locals.var_fs02_dn6 = assign65310_body36_e101552_d_n6;
            locals.var_fs02_dn7 = assign65310_body36_e101552_d_n7;
            locals.var_fs02_dn8 = assign65310_body36_e101552_d_n8;
            locals.var_fs02_dn9 = assign65310_body36_e101552_d_n9;
            locals.var_fs02_dn10 = assign65310_body36_e101552_d_n10;
            locals.var_fs02_dn11 = assign65310_body36_e101552_d_n11;
            locals.var_fs02_dn14 = assign65310_body36_e101552_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign65310_body37_e101572, assign65310_body37_e101572_d_n0, assign65310_body37_e101572_d_n2, assign65310_body37_e101572_d_n4, assign65310_body37_e101572_d_n5, assign65310_body37_e101572_d_n6, assign65310_body37_e101572_d_n7, assign65310_body37_e101572_d_n8, assign65310_body37_e101572_d_n9, assign65310_body37_e101572_d_n10, assign65310_body37_e101572_d_n11, assign65310_body37_e101572_d_n14,) = {
    if (((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1565 == 0.0)) && (locals.var_guard1566 != 0.0)) {
        (locals.var_fb_dpss__blk1557, locals.var_fb_dpss__blk1557_dn0, locals.var_fb_dpss__blk1557_dn2, locals.var_fb_dpss__blk1557_dn4, locals.var_fb_dpss__blk1557_dn5, locals.var_fb_dpss__blk1557_dn6, locals.var_fb_dpss__blk1557_dn7, locals.var_fb_dpss__blk1557_dn8, locals.var_fb_dpss__blk1557_dn9, locals.var_fb_dpss__blk1557_dn10, locals.var_fb_dpss__blk1557_dn11, locals.var_fb_dpss__blk1557_dn14,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign65310_body37_e101572;
            locals.var_fs02_dps0_dn0 = assign65310_body37_e101572_d_n0;
            locals.var_fs02_dps0_dn2 = assign65310_body37_e101572_d_n2;
            locals.var_fs02_dps0_dn4 = assign65310_body37_e101572_d_n4;
            locals.var_fs02_dps0_dn5 = assign65310_body37_e101572_d_n5;
            locals.var_fs02_dps0_dn6 = assign65310_body37_e101572_d_n6;
            locals.var_fs02_dps0_dn7 = assign65310_body37_e101572_d_n7;
            locals.var_fs02_dps0_dn8 = assign65310_body37_e101572_d_n8;
            locals.var_fs02_dps0_dn9 = assign65310_body37_e101572_d_n9;
            locals.var_fs02_dps0_dn10 = assign65310_body37_e101572_d_n10;
            locals.var_fs02_dps0_dn11 = assign65310_body37_e101572_d_n11;
            locals.var_fs02_dps0_dn14 = assign65310_body37_e101572_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign65310_body38_e101597, assign65310_body38_e101597_d_n0, assign65310_body38_e101597_d_n2, assign65310_body38_e101597_d_n4, assign65310_body38_e101597_d_n5, assign65310_body38_e101597_d_n6, assign65310_body38_e101597_d_n7, assign65310_body38_e101597_d_n8, assign65310_body38_e101597_d_n9, assign65310_body38_e101597_d_n10, assign65310_body38_e101597_d_n11, assign65310_body38_e101597_d_n14,) = {
    if (((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1565 == 0.0)) && (locals.var_guard1566 == 0.0)) {
        let assign65310_body38_e101594: f64 = (locals.var_phi_s0 - p.p456);
        let assign65310_body38_e101595: f64 = (locals.var_beta * assign65310_body38_e101594);
        (assign65310_body38_e101595, ((locals.var_beta_dn0 * assign65310_body38_e101594) + (locals.var_beta * locals.var_phi_s0_dn0)), ((locals.var_beta_dn2 * assign65310_body38_e101594) + (locals.var_beta * locals.var_phi_s0_dn2)), ((locals.var_beta_dn4 * assign65310_body38_e101594) + (locals.var_beta * locals.var_phi_s0_dn4)), ((locals.var_beta_dn5 * assign65310_body38_e101594) + (locals.var_beta * locals.var_phi_s0_dn5)), ((locals.var_beta_dn6 * assign65310_body38_e101594) + (locals.var_beta * locals.var_phi_s0_dn6)), ((locals.var_beta_dn7 * assign65310_body38_e101594) + (locals.var_beta * locals.var_phi_s0_dn7)), ((locals.var_beta_dn8 * assign65310_body38_e101594) + (locals.var_beta * locals.var_phi_s0_dn8)), ((locals.var_beta_dn9 * assign65310_body38_e101594) + (locals.var_beta * locals.var_phi_s0_dn9)), ((locals.var_beta_dn10 * assign65310_body38_e101594) + (locals.var_beta * locals.var_phi_s0_dn10)), ((locals.var_beta_dn11 * assign65310_body38_e101594) + (locals.var_beta * locals.var_phi_s0_dn11)), ((locals.var_beta_dn14 * assign65310_body38_e101594) + (locals.var_beta * locals.var_phi_s0_dn14)),)
    } else {
        (locals.var_rho, locals.var_rho_dn0, locals.var_rho_dn2, locals.var_rho_dn4, locals.var_rho_dn5, locals.var_rho_dn6, locals.var_rho_dn7, locals.var_rho_dn8, locals.var_rho_dn9, locals.var_rho_dn10, locals.var_rho_dn11, locals.var_rho_dn14,)
    }
};
            locals.var_rho = assign65310_body38_e101597;
            locals.var_rho_dn0 = assign65310_body38_e101597_d_n0;
            locals.var_rho_dn2 = assign65310_body38_e101597_d_n2;
            locals.var_rho_dn4 = assign65310_body38_e101597_d_n4;
            locals.var_rho_dn5 = assign65310_body38_e101597_d_n5;
            locals.var_rho_dn6 = assign65310_body38_e101597_d_n6;
            locals.var_rho_dn7 = assign65310_body38_e101597_d_n7;
            locals.var_rho_dn8 = assign65310_body38_e101597_d_n8;
            locals.var_rho_dn9 = assign65310_body38_e101597_d_n9;
            locals.var_rho_dn10 = assign65310_body38_e101597_d_n10;
            locals.var_rho_dn11 = assign65310_body38_e101597_d_n11;
            locals.var_rho_dn14 = assign65310_body38_e101597_d_n14;
            locals.var_rho_rv = 0.0;
            let (assign65310_body39_e101619, assign65310_body39_e101619_d_n0, assign65310_body39_e101619_d_n2, assign65310_body39_e101619_d_n4, assign65310_body39_e101619_d_n5, assign65310_body39_e101619_d_n6, assign65310_body39_e101619_d_n7, assign65310_body39_e101619_d_n8, assign65310_body39_e101619_d_n9, assign65310_body39_e101619_d_n10, assign65310_body39_e101619_d_n11, assign65310_body39_e101619_d_n14,) = {
    if (((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1565 == 0.0)) && (locals.var_guard1566 == 0.0)) {
        let assign65310_body39_e101617: f64 = (locals.var_rho).exp();
        (assign65310_body39_e101617, (assign65310_body39_e101617 * locals.var_rho_dn0), (assign65310_body39_e101617 * locals.var_rho_dn2), (assign65310_body39_e101617 * locals.var_rho_dn4), (assign65310_body39_e101617 * locals.var_rho_dn5), (assign65310_body39_e101617 * locals.var_rho_dn6), (assign65310_body39_e101617 * locals.var_rho_dn7), (assign65310_body39_e101617 * locals.var_rho_dn8), (assign65310_body39_e101617 * locals.var_rho_dn9), (assign65310_body39_e101617 * locals.var_rho_dn10), (assign65310_body39_e101617 * locals.var_rho_dn11), (assign65310_body39_e101617 * locals.var_rho_dn14),)
    } else {
        (locals.var_exp_rho, locals.var_exp_rho_dn0, locals.var_exp_rho_dn2, locals.var_exp_rho_dn4, locals.var_exp_rho_dn5, locals.var_exp_rho_dn6, locals.var_exp_rho_dn7, locals.var_exp_rho_dn8, locals.var_exp_rho_dn9, locals.var_exp_rho_dn10, locals.var_exp_rho_dn11, locals.var_exp_rho_dn14,)
    }
};
            locals.var_exp_rho = assign65310_body39_e101619;
            locals.var_exp_rho_dn0 = assign65310_body39_e101619_d_n0;
            locals.var_exp_rho_dn2 = assign65310_body39_e101619_d_n2;
            locals.var_exp_rho_dn4 = assign65310_body39_e101619_d_n4;
            locals.var_exp_rho_dn5 = assign65310_body39_e101619_d_n5;
            locals.var_exp_rho_dn6 = assign65310_body39_e101619_d_n6;
            locals.var_exp_rho_dn7 = assign65310_body39_e101619_d_n7;
            locals.var_exp_rho_dn8 = assign65310_body39_e101619_d_n8;
            locals.var_exp_rho_dn9 = assign65310_body39_e101619_d_n9;
            locals.var_exp_rho_dn10 = assign65310_body39_e101619_d_n10;
            locals.var_exp_rho_dn11 = assign65310_body39_e101619_d_n11;
            locals.var_exp_rho_dn14 = assign65310_body39_e101619_d_n14;
            locals.var_exp_rho_rv = 0.0;
            let (assign65310_body40_e101648, assign65310_body40_e101648_d_n0, assign65310_body40_e101648_d_n2, assign65310_body40_e101648_d_n4, assign65310_body40_e101648_d_n5, assign65310_body40_e101648_d_n6, assign65310_body40_e101648_d_n7, assign65310_body40_e101648_d_n8, assign65310_body40_e101648_d_n9, assign65310_body40_e101648_d_n10, assign65310_body40_e101648_d_n11, assign65310_body40_e101648_d_n14,) = {
    if (((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1565 == 0.0)) && (locals.var_guard1566 == 0.0)) {
        let assign65310_body40_e101643: f64 = (locals.var_chi + 1.0);
        let assign65310_body40_e101644: f64 = (locals.var_exp_bvbsvds * assign65310_body40_e101643);
        let assign65310_body40_e101645: f64 = (locals.var_exp_rho - assign65310_body40_e101644);
        let assign65310_body40_e101646: f64 = (locals.var_cnst1 * assign65310_body40_e101645);
        (assign65310_body40_e101646, ((locals.var_cnst1_dn0 * assign65310_body40_e101645) + (locals.var_cnst1 * (locals.var_exp_rho_dn0 - ((locals.var_exp_bvbsvds_dn0 * assign65310_body40_e101643) + (locals.var_exp_bvbsvds * locals.var_chi_dn0))))), ((locals.var_cnst1_dn2 * assign65310_body40_e101645) + (locals.var_cnst1 * (locals.var_exp_rho_dn2 - ((locals.var_exp_bvbsvds_dn2 * assign65310_body40_e101643) + (locals.var_exp_bvbsvds * locals.var_chi_dn2))))), ((locals.var_cnst1_dn4 * assign65310_body40_e101645) + (locals.var_cnst1 * (locals.var_exp_rho_dn4 - ((locals.var_exp_bvbsvds_dn4 * assign65310_body40_e101643) + (locals.var_exp_bvbsvds * locals.var_chi_dn4))))), ((locals.var_cnst1_dn5 * assign65310_body40_e101645) + (locals.var_cnst1 * (locals.var_exp_rho_dn5 - ((locals.var_exp_bvbsvds_dn5 * assign65310_body40_e101643) + (locals.var_exp_bvbsvds * locals.var_chi_dn5))))), ((locals.var_cnst1_dn6 * assign65310_body40_e101645) + (locals.var_cnst1 * (locals.var_exp_rho_dn6 - ((locals.var_exp_bvbsvds_dn6 * assign65310_body40_e101643) + (locals.var_exp_bvbsvds * locals.var_chi_dn6))))), ((locals.var_cnst1_dn7 * assign65310_body40_e101645) + (locals.var_cnst1 * (locals.var_exp_rho_dn7 - ((locals.var_exp_bvbsvds_dn7 * assign65310_body40_e101643) + (locals.var_exp_bvbsvds * locals.var_chi_dn7))))), ((locals.var_cnst1_dn8 * assign65310_body40_e101645) + (locals.var_cnst1 * (locals.var_exp_rho_dn8 - ((locals.var_exp_bvbsvds_dn8 * assign65310_body40_e101643) + (locals.var_exp_bvbsvds * locals.var_chi_dn8))))), ((locals.var_cnst1_dn9 * assign65310_body40_e101645) + (locals.var_cnst1 * (locals.var_exp_rho_dn9 - ((locals.var_exp_bvbsvds_dn9 * assign65310_body40_e101643) + (locals.var_exp_bvbsvds * locals.var_chi_dn9))))), ((locals.var_cnst1_dn10 * assign65310_body40_e101645) + (locals.var_cnst1 * (locals.var_exp_rho_dn10 - ((locals.var_exp_bvbsvds_dn10 * assign65310_body40_e101643) + (locals.var_exp_bvbsvds * locals.var_chi_dn10))))), ((locals.var_cnst1_dn11 * assign65310_body40_e101645) + (locals.var_cnst1 * (locals.var_exp_rho_dn11 - ((locals.var_exp_bvbsvds_dn11 * assign65310_body40_e101643) + (locals.var_exp_bvbsvds * locals.var_chi_dn11))))), ((locals.var_cnst1_dn14 * assign65310_body40_e101645) + (locals.var_cnst1 * (locals.var_exp_rho_dn14 - ((locals.var_exp_bvbsvds_dn14 * assign65310_body40_e101643) + (locals.var_exp_bvbsvds * locals.var_chi_dn14))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign65310_body40_e101648;
            locals.var_fs01_dn0 = assign65310_body40_e101648_d_n0;
            locals.var_fs01_dn2 = assign65310_body40_e101648_d_n2;
            locals.var_fs01_dn4 = assign65310_body40_e101648_d_n4;
            locals.var_fs01_dn5 = assign65310_body40_e101648_d_n5;
            locals.var_fs01_dn6 = assign65310_body40_e101648_d_n6;
            locals.var_fs01_dn7 = assign65310_body40_e101648_d_n7;
            locals.var_fs01_dn8 = assign65310_body40_e101648_d_n8;
            locals.var_fs01_dn9 = assign65310_body40_e101648_d_n9;
            locals.var_fs01_dn10 = assign65310_body40_e101648_d_n10;
            locals.var_fs01_dn11 = assign65310_body40_e101648_d_n11;
            locals.var_fs01_dn14 = assign65310_body40_e101648_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign65310_body41_e101675, assign65310_body41_e101675_d_n0, assign65310_body41_e101675_d_n2, assign65310_body41_e101675_d_n4, assign65310_body41_e101675_d_n5, assign65310_body41_e101675_d_n6, assign65310_body41_e101675_d_n7, assign65310_body41_e101675_d_n8, assign65310_body41_e101675_d_n9, assign65310_body41_e101675_d_n10, assign65310_body41_e101675_d_n11, assign65310_body41_e101675_d_n14,) = {
    if (((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1565 == 0.0)) && (locals.var_guard1566 == 0.0)) {
        let assign65310_body41_e101669: f64 = (locals.var_cnst1 * locals.var_beta);
        let assign65310_body41_e101672: f64 = (locals.var_exp_rho - locals.var_exp_bvbsvds);
        let assign65310_body41_e101673: f64 = (assign65310_body41_e101669 * assign65310_body41_e101672);
        (assign65310_body41_e101673, ((((locals.var_cnst1_dn0 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn0)) * assign65310_body41_e101672) + (assign65310_body41_e101669 * (locals.var_exp_rho_dn0 - locals.var_exp_bvbsvds_dn0))), ((((locals.var_cnst1_dn2 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn2)) * assign65310_body41_e101672) + (assign65310_body41_e101669 * (locals.var_exp_rho_dn2 - locals.var_exp_bvbsvds_dn2))), ((((locals.var_cnst1_dn4 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn4)) * assign65310_body41_e101672) + (assign65310_body41_e101669 * (locals.var_exp_rho_dn4 - locals.var_exp_bvbsvds_dn4))), ((((locals.var_cnst1_dn5 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn5)) * assign65310_body41_e101672) + (assign65310_body41_e101669 * (locals.var_exp_rho_dn5 - locals.var_exp_bvbsvds_dn5))), ((((locals.var_cnst1_dn6 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn6)) * assign65310_body41_e101672) + (assign65310_body41_e101669 * (locals.var_exp_rho_dn6 - locals.var_exp_bvbsvds_dn6))), ((((locals.var_cnst1_dn7 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn7)) * assign65310_body41_e101672) + (assign65310_body41_e101669 * (locals.var_exp_rho_dn7 - locals.var_exp_bvbsvds_dn7))), ((((locals.var_cnst1_dn8 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn8)) * assign65310_body41_e101672) + (assign65310_body41_e101669 * (locals.var_exp_rho_dn8 - locals.var_exp_bvbsvds_dn8))), ((((locals.var_cnst1_dn9 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn9)) * assign65310_body41_e101672) + (assign65310_body41_e101669 * (locals.var_exp_rho_dn9 - locals.var_exp_bvbsvds_dn9))), ((((locals.var_cnst1_dn10 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn10)) * assign65310_body41_e101672) + (assign65310_body41_e101669 * (locals.var_exp_rho_dn10 - locals.var_exp_bvbsvds_dn10))), ((((locals.var_cnst1_dn11 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn11)) * assign65310_body41_e101672) + (assign65310_body41_e101669 * (locals.var_exp_rho_dn11 - locals.var_exp_bvbsvds_dn11))), ((((locals.var_cnst1_dn14 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn14)) * assign65310_body41_e101672) + (assign65310_body41_e101669 * (locals.var_exp_rho_dn14 - locals.var_exp_bvbsvds_dn14))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign65310_body41_e101675;
            locals.var_fs01_dps0_dn0 = assign65310_body41_e101675_d_n0;
            locals.var_fs01_dps0_dn2 = assign65310_body41_e101675_d_n2;
            locals.var_fs01_dps0_dn4 = assign65310_body41_e101675_d_n4;
            locals.var_fs01_dps0_dn5 = assign65310_body41_e101675_d_n5;
            locals.var_fs01_dps0_dn6 = assign65310_body41_e101675_d_n6;
            locals.var_fs01_dps0_dn7 = assign65310_body41_e101675_d_n7;
            locals.var_fs01_dps0_dn8 = assign65310_body41_e101675_d_n8;
            locals.var_fs01_dps0_dn9 = assign65310_body41_e101675_d_n9;
            locals.var_fs01_dps0_dn10 = assign65310_body41_e101675_d_n10;
            locals.var_fs01_dps0_dn11 = assign65310_body41_e101675_d_n11;
            locals.var_fs01_dps0_dn14 = assign65310_body41_e101675_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign65310_body42_e101701, assign65310_body42_e101701_d_n0, assign65310_body42_e101701_d_n2, assign65310_body42_e101701_d_n4, assign65310_body42_e101701_d_n5, assign65310_body42_e101701_d_n6, assign65310_body42_e101701_d_n7, assign65310_body42_e101701_d_n8, assign65310_body42_e101701_d_n9, assign65310_body42_e101701_d_n10, assign65310_body42_e101701_d_n11, assign65310_body42_e101701_d_n14,) = {
    if (((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1565 == 0.0)) && (locals.var_guard1566 == 0.0)) {
        let assign65310_body42_e101696: f64 = (locals.var_fb * locals.var_fb);
        let assign65310_body42_e101698: f64 = (assign65310_body42_e101696 + locals.var_fs01);
        let assign65310_body42_e101699: f64 = (assign65310_body42_e101698).sqrt();
        (assign65310_body42_e101699, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fs01_dn0) / (2.0 * assign65310_body42_e101699)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fs01_dn2) / (2.0 * assign65310_body42_e101699)), ((((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)) + locals.var_fs01_dn4) / (2.0 * assign65310_body42_e101699)), ((((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)) + locals.var_fs01_dn5) / (2.0 * assign65310_body42_e101699)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fs01_dn6) / (2.0 * assign65310_body42_e101699)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fs01_dn7) / (2.0 * assign65310_body42_e101699)), ((((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)) + locals.var_fs01_dn8) / (2.0 * assign65310_body42_e101699)), ((((locals.var_fb_dn9 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn9)) + locals.var_fs01_dn9) / (2.0 * assign65310_body42_e101699)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fs01_dn10) / (2.0 * assign65310_body42_e101699)), ((((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)) + locals.var_fs01_dn11) / (2.0 * assign65310_body42_e101699)), ((((locals.var_fb_dn14 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn14)) + locals.var_fs01_dn14) / (2.0 * assign65310_body42_e101699)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign65310_body42_e101701;
            locals.var_fs02_dn0 = assign65310_body42_e101701_d_n0;
            locals.var_fs02_dn2 = assign65310_body42_e101701_d_n2;
            locals.var_fs02_dn4 = assign65310_body42_e101701_d_n4;
            locals.var_fs02_dn5 = assign65310_body42_e101701_d_n5;
            locals.var_fs02_dn6 = assign65310_body42_e101701_d_n6;
            locals.var_fs02_dn7 = assign65310_body42_e101701_d_n7;
            locals.var_fs02_dn8 = assign65310_body42_e101701_d_n8;
            locals.var_fs02_dn9 = assign65310_body42_e101701_d_n9;
            locals.var_fs02_dn10 = assign65310_body42_e101701_d_n10;
            locals.var_fs02_dn11 = assign65310_body42_e101701_d_n11;
            locals.var_fs02_dn14 = assign65310_body42_e101701_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign65310_body43_e101732, assign65310_body43_e101732_d_n0, assign65310_body43_e101732_d_n2, assign65310_body43_e101732_d_n4, assign65310_body43_e101732_d_n5, assign65310_body43_e101732_d_n6, assign65310_body43_e101732_d_n7, assign65310_body43_e101732_d_n8, assign65310_body43_e101732_d_n9, assign65310_body43_e101732_d_n10, assign65310_body43_e101732_d_n11, assign65310_body43_e101732_d_n14,) = {
    if (((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1565 == 0.0)) && (locals.var_guard1566 == 0.0)) {
        let assign65310_body43_e101723: f64 = (2.0 * locals.var_fb_dpss__blk1557);
        let assign65310_body43_e101725: f64 = (assign65310_body43_e101723 * locals.var_fb);
        let assign65310_body43_e101727: f64 = (assign65310_body43_e101725 + locals.var_fs01_dps0);
        let assign65310_body43_e101728: f64 = (0.5 * assign65310_body43_e101727);
        let assign65310_body43_e101730: f64 = (assign65310_body43_e101728 / locals.var_fs02);
        (assign65310_body43_e101730, ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1557_dn0) * locals.var_fb) + (assign65310_body43_e101723 * locals.var_fb_dn0)) + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign65310_body43_e101728 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1557_dn2) * locals.var_fb) + (assign65310_body43_e101723 * locals.var_fb_dn2)) + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign65310_body43_e101728 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1557_dn4) * locals.var_fb) + (assign65310_body43_e101723 * locals.var_fb_dn4)) + locals.var_fs01_dps0_dn4)) * locals.var_fs02) - (assign65310_body43_e101728 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1557_dn5) * locals.var_fb) + (assign65310_body43_e101723 * locals.var_fb_dn5)) + locals.var_fs01_dps0_dn5)) * locals.var_fs02) - (assign65310_body43_e101728 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1557_dn6) * locals.var_fb) + (assign65310_body43_e101723 * locals.var_fb_dn6)) + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign65310_body43_e101728 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1557_dn7) * locals.var_fb) + (assign65310_body43_e101723 * locals.var_fb_dn7)) + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign65310_body43_e101728 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1557_dn8) * locals.var_fb) + (assign65310_body43_e101723 * locals.var_fb_dn8)) + locals.var_fs01_dps0_dn8)) * locals.var_fs02) - (assign65310_body43_e101728 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1557_dn9) * locals.var_fb) + (assign65310_body43_e101723 * locals.var_fb_dn9)) + locals.var_fs01_dps0_dn9)) * locals.var_fs02) - (assign65310_body43_e101728 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1557_dn10) * locals.var_fb) + (assign65310_body43_e101723 * locals.var_fb_dn10)) + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign65310_body43_e101728 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1557_dn11) * locals.var_fb) + (assign65310_body43_e101723 * locals.var_fb_dn11)) + locals.var_fs01_dps0_dn11)) * locals.var_fs02) - (assign65310_body43_e101728 * locals.var_fs02_dn11)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1557_dn14) * locals.var_fb) + (assign65310_body43_e101723 * locals.var_fb_dn14)) + locals.var_fs01_dps0_dn14)) * locals.var_fs02) - (assign65310_body43_e101728 * locals.var_fs02_dn14)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign65310_body43_e101732;
            locals.var_fs02_dps0_dn0 = assign65310_body43_e101732_d_n0;
            locals.var_fs02_dps0_dn2 = assign65310_body43_e101732_d_n2;
            locals.var_fs02_dps0_dn4 = assign65310_body43_e101732_d_n4;
            locals.var_fs02_dps0_dn5 = assign65310_body43_e101732_d_n5;
            locals.var_fs02_dps0_dn6 = assign65310_body43_e101732_d_n6;
            locals.var_fs02_dps0_dn7 = assign65310_body43_e101732_d_n7;
            locals.var_fs02_dps0_dn8 = assign65310_body43_e101732_d_n8;
            locals.var_fs02_dps0_dn9 = assign65310_body43_e101732_d_n9;
            locals.var_fs02_dps0_dn10 = assign65310_body43_e101732_d_n10;
            locals.var_fs02_dps0_dn11 = assign65310_body43_e101732_d_n11;
            locals.var_fs02_dps0_dn14 = assign65310_body43_e101732_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign65310_body44_e101754, assign65310_body44_e101754_d_n0, assign65310_body44_e101754_d_n2, assign65310_body44_e101754_d_n4, assign65310_body44_e101754_d_n5, assign65310_body44_e101754_d_n6, assign65310_body44_e101754_d_n7, assign65310_body44_e101754_d_n8, assign65310_body44_e101754_d_n9, assign65310_body44_e101754_d_n10, assign65310_body44_e101754_d_n11, assign65310_body44_e101754_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) {
        let assign65310_body44_e101746: f64 = (-locals.var_vgp__blk1529);
        let assign65310_body44_e101748: f64 = (assign65310_body44_e101746 + locals.var_phi_s0);
        let assign65310_body44_e101751: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign65310_body44_e101752: f64 = (assign65310_body44_e101748 + assign65310_body44_e101751);
        (assign65310_body44_e101752, (((-locals.var_vgp__blk1529_dn0) + locals.var_phi_s0_dn0) + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgp__blk1529_dn2) + locals.var_phi_s0_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (((-locals.var_vgp__blk1529_dn4) + locals.var_phi_s0_dn4) + ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), (((-locals.var_vgp__blk1529_dn5) + locals.var_phi_s0_dn5) + ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), (((-locals.var_vgp__blk1529_dn6) + locals.var_phi_s0_dn6) + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgp__blk1529_dn7) + locals.var_phi_s0_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgp__blk1529_dn8) + locals.var_phi_s0_dn8) + ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), (((-locals.var_vgp__blk1529_dn9) + locals.var_phi_s0_dn9) + ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), (((-locals.var_vgp__blk1529_dn10) + locals.var_phi_s0_dn10) + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (((-locals.var_vgp__blk1529_dn11) + locals.var_phi_s0_dn11) + ((locals.var_fac1_dn11 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn11))), (((-locals.var_vgp__blk1529_dn14) + locals.var_phi_s0_dn14) + ((locals.var_fac1_dn14 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn14))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn11, locals.var_fs0_dn14,)
    }
};
            locals.var_fs0 = assign65310_body44_e101754;
            locals.var_fs0_dn0 = assign65310_body44_e101754_d_n0;
            locals.var_fs0_dn2 = assign65310_body44_e101754_d_n2;
            locals.var_fs0_dn4 = assign65310_body44_e101754_d_n4;
            locals.var_fs0_dn5 = assign65310_body44_e101754_d_n5;
            locals.var_fs0_dn6 = assign65310_body44_e101754_d_n6;
            locals.var_fs0_dn7 = assign65310_body44_e101754_d_n7;
            locals.var_fs0_dn8 = assign65310_body44_e101754_d_n8;
            locals.var_fs0_dn9 = assign65310_body44_e101754_d_n9;
            locals.var_fs0_dn10 = assign65310_body44_e101754_d_n10;
            locals.var_fs0_dn11 = assign65310_body44_e101754_d_n11;
            locals.var_fs0_dn14 = assign65310_body44_e101754_d_n14;
            locals.var_fs0_rv = 0.0;
            let (assign65310_body45_e101773, assign65310_body45_e101773_d_n0, assign65310_body45_e101773_d_n2, assign65310_body45_e101773_d_n4, assign65310_body45_e101773_d_n5, assign65310_body45_e101773_d_n6, assign65310_body45_e101773_d_n7, assign65310_body45_e101773_d_n8, assign65310_body45_e101773_d_n9, assign65310_body45_e101773_d_n10, assign65310_body45_e101773_d_n11, assign65310_body45_e101773_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) {
        let assign65310_body45_e101770: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign65310_body45_e101771: f64 = (1.0 + assign65310_body45_e101770);
        (assign65310_body45_e101771, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4)), ((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8)), ((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn11 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn11)), ((locals.var_fac1_dn14 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn14)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn11, locals.var_fs0_dps0_dn14,)
    }
};
            locals.var_fs0_dps0 = assign65310_body45_e101773;
            locals.var_fs0_dps0_dn0 = assign65310_body45_e101773_d_n0;
            locals.var_fs0_dps0_dn2 = assign65310_body45_e101773_d_n2;
            locals.var_fs0_dps0_dn4 = assign65310_body45_e101773_d_n4;
            locals.var_fs0_dps0_dn5 = assign65310_body45_e101773_d_n5;
            locals.var_fs0_dps0_dn6 = assign65310_body45_e101773_d_n6;
            locals.var_fs0_dps0_dn7 = assign65310_body45_e101773_d_n7;
            locals.var_fs0_dps0_dn8 = assign65310_body45_e101773_d_n8;
            locals.var_fs0_dps0_dn9 = assign65310_body45_e101773_d_n9;
            locals.var_fs0_dps0_dn10 = assign65310_body45_e101773_d_n10;
            locals.var_fs0_dps0_dn11 = assign65310_body45_e101773_d_n11;
            locals.var_fs0_dps0_dn14 = assign65310_body45_e101773_d_n14;
            locals.var_fs0_dps0_rv = 0.0;
            let assign65310_body46_e101776: f64 = if locals.var_flg_conv == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard1567 = assign65310_body46_e101776;
            locals.var_guard1567_rv = 0.0;
            let (assign65310_body47_e101795,) = {
    if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1567 != 0.0)) {
        let assign65310_body47_e101793: f64 = (locals.var_lp_s0_max + 1.0);
        (assign65310_body47_e101793,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign65310_body47_e101795;
            locals.var_lp_s0_rv = 0.0;
            let (assign65310_body48_e101816, assign65310_body48_e101816_d_n0, assign65310_body48_e101816_d_n2, assign65310_body48_e101816_d_n4, assign65310_body48_e101816_d_n5, assign65310_body48_e101816_d_n6, assign65310_body48_e101816_d_n7, assign65310_body48_e101816_d_n8, assign65310_body48_e101816_d_n9, assign65310_body48_e101816_d_n10, assign65310_body48_e101816_d_n11, assign65310_body48_e101816_d_n14,) = {
    if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1567 == 0.0)) {
        let assign65310_body48_e101812: f64 = (-locals.var_fs0);
        let assign65310_body48_e101814: f64 = (assign65310_body48_e101812 / locals.var_fs0_dps0);
        (assign65310_body48_e101814, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign65310_body48_e101812 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign65310_body48_e101812 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign65310_body48_e101812 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign65310_body48_e101812 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign65310_body48_e101812 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign65310_body48_e101812 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign65310_body48_e101812 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign65310_body48_e101812 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign65310_body48_e101812 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn11) * locals.var_fs0_dps0) - (assign65310_body48_e101812 * locals.var_fs0_dps0_dn11)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn14) * locals.var_fs0_dps0) - (assign65310_body48_e101812 * locals.var_fs0_dps0_dn14)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
            locals.var_dps0 = assign65310_body48_e101816;
            locals.var_dps0_dn0 = assign65310_body48_e101816_d_n0;
            locals.var_dps0_dn2 = assign65310_body48_e101816_d_n2;
            locals.var_dps0_dn4 = assign65310_body48_e101816_d_n4;
            locals.var_dps0_dn5 = assign65310_body48_e101816_d_n5;
            locals.var_dps0_dn6 = assign65310_body48_e101816_d_n6;
            locals.var_dps0_dn7 = assign65310_body48_e101816_d_n7;
            locals.var_dps0_dn8 = assign65310_body48_e101816_d_n8;
            locals.var_dps0_dn9 = assign65310_body48_e101816_d_n9;
            locals.var_dps0_dn10 = assign65310_body48_e101816_d_n10;
            locals.var_dps0_dn11 = assign65310_body48_e101816_d_n11;
            locals.var_dps0_dn14 = assign65310_body48_e101816_d_n14;
            locals.var_dps0_rv = 0.0;
            let (assign65310_body49_e101847, assign65310_body49_e101847_d_n0, assign65310_body49_e101847_d_n2, assign65310_body49_e101847_d_n4, assign65310_body49_e101847_d_n5, assign65310_body49_e101847_d_n6, assign65310_body49_e101847_d_n7, assign65310_body49_e101847_d_n8, assign65310_body49_e101847_d_n9, assign65310_body49_e101847_d_n10, assign65310_body49_e101847_d_n11, assign65310_body49_e101847_d_n14,) = {
    if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1567 == 0.0)) {
        let assign65310_body49_e101834: f64 = (0.5 * 0.1);
        let assign65310_body49_e101838: f64 = (locals.var_phi_s0).abs();
        let (assign65310_body49_e101843, assign65310_body49_e101843_d_n0, assign65310_body49_e101843_d_n2, assign65310_body49_e101843_d_n4, assign65310_body49_e101843_d_n5, assign65310_body49_e101843_d_n6, assign65310_body49_e101843_d_n7, assign65310_body49_e101843_d_n8, assign65310_body49_e101843_d_n9, assign65310_body49_e101843_d_n10, assign65310_body49_e101843_d_n11, assign65310_body49_e101843_d_n14,) = {
            if (1.0 >= assign65310_body49_e101838) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign65310_body49_e101842: f64 = (locals.var_phi_s0).abs();
                (assign65310_body49_e101842, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn0 } else { (-locals.var_phi_s0_dn0) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn2 } else { (-locals.var_phi_s0_dn2) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn4 } else { (-locals.var_phi_s0_dn4) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn5 } else { (-locals.var_phi_s0_dn5) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn6 } else { (-locals.var_phi_s0_dn6) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn7 } else { (-locals.var_phi_s0_dn7) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn8 } else { (-locals.var_phi_s0_dn8) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn9 } else { (-locals.var_phi_s0_dn9) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn10 } else { (-locals.var_phi_s0_dn10) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn11 } else { (-locals.var_phi_s0_dn11) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn14 } else { (-locals.var_phi_s0_dn14) },)
            }
        };
        let assign65310_body49_e101844: f64 = (1.0 + assign65310_body49_e101843);
        let assign65310_body49_e101845: f64 = (assign65310_body49_e101834 * assign65310_body49_e101844);
        (assign65310_body49_e101845, (assign65310_body49_e101834 * assign65310_body49_e101843_d_n0), (assign65310_body49_e101834 * assign65310_body49_e101843_d_n2), (assign65310_body49_e101834 * assign65310_body49_e101843_d_n4), (assign65310_body49_e101834 * assign65310_body49_e101843_d_n5), (assign65310_body49_e101834 * assign65310_body49_e101843_d_n6), (assign65310_body49_e101834 * assign65310_body49_e101843_d_n7), (assign65310_body49_e101834 * assign65310_body49_e101843_d_n8), (assign65310_body49_e101834 * assign65310_body49_e101843_d_n9), (assign65310_body49_e101834 * assign65310_body49_e101843_d_n10), (assign65310_body49_e101834 * assign65310_body49_e101843_d_n11), (assign65310_body49_e101834 * assign65310_body49_e101843_d_n14),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn11, locals.var_dplim_dn14,)
    }
};
            locals.var_dplim = assign65310_body49_e101847;
            locals.var_dplim_dn0 = assign65310_body49_e101847_d_n0;
            locals.var_dplim_dn2 = assign65310_body49_e101847_d_n2;
            locals.var_dplim_dn4 = assign65310_body49_e101847_d_n4;
            locals.var_dplim_dn5 = assign65310_body49_e101847_d_n5;
            locals.var_dplim_dn6 = assign65310_body49_e101847_d_n6;
            locals.var_dplim_dn7 = assign65310_body49_e101847_d_n7;
            locals.var_dplim_dn8 = assign65310_body49_e101847_d_n8;
            locals.var_dplim_dn9 = assign65310_body49_e101847_d_n9;
            locals.var_dplim_dn10 = assign65310_body49_e101847_d_n10;
            locals.var_dplim_dn11 = assign65310_body49_e101847_d_n11;
            locals.var_dplim_dn14 = assign65310_body49_e101847_d_n14;
            locals.var_dplim_rv = 0.0;
            let assign65310_body50_e101849: f64 = (locals.var_dps0).abs();
            let assign65310_body50_e101851: f64 = if assign65310_body50_e101849 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard1568 = assign65310_body50_e101851;
            locals.var_guard1568_rv = 0.0;
            let (assign65310_body51_e101879, assign65310_body51_e101879_d_n0, assign65310_body51_e101879_d_n2, assign65310_body51_e101879_d_n4, assign65310_body51_e101879_d_n5, assign65310_body51_e101879_d_n6, assign65310_body51_e101879_d_n7, assign65310_body51_e101879_d_n8, assign65310_body51_e101879_d_n9, assign65310_body51_e101879_d_n10, assign65310_body51_e101879_d_n11, assign65310_body51_e101879_d_n14,) = {
    if (((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1567 == 0.0)) && (locals.var_guard1568 != 0.0)) {
        let (assign65310_body51_e101876,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign65310_body51_e101875: f64 = (-1.0);
                (assign65310_body51_e101875,)
            }
        };
        let assign65310_body51_e101877: f64 = (locals.var_dplim * assign65310_body51_e101876);
        (assign65310_body51_e101877, (locals.var_dplim_dn0 * assign65310_body51_e101876), (locals.var_dplim_dn2 * assign65310_body51_e101876), (locals.var_dplim_dn4 * assign65310_body51_e101876), (locals.var_dplim_dn5 * assign65310_body51_e101876), (locals.var_dplim_dn6 * assign65310_body51_e101876), (locals.var_dplim_dn7 * assign65310_body51_e101876), (locals.var_dplim_dn8 * assign65310_body51_e101876), (locals.var_dplim_dn9 * assign65310_body51_e101876), (locals.var_dplim_dn10 * assign65310_body51_e101876), (locals.var_dplim_dn11 * assign65310_body51_e101876), (locals.var_dplim_dn14 * assign65310_body51_e101876),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
            locals.var_dps0 = assign65310_body51_e101879;
            locals.var_dps0_dn0 = assign65310_body51_e101879_d_n0;
            locals.var_dps0_dn2 = assign65310_body51_e101879_d_n2;
            locals.var_dps0_dn4 = assign65310_body51_e101879_d_n4;
            locals.var_dps0_dn5 = assign65310_body51_e101879_d_n5;
            locals.var_dps0_dn6 = assign65310_body51_e101879_d_n6;
            locals.var_dps0_dn7 = assign65310_body51_e101879_d_n7;
            locals.var_dps0_dn8 = assign65310_body51_e101879_d_n8;
            locals.var_dps0_dn9 = assign65310_body51_e101879_d_n9;
            locals.var_dps0_dn10 = assign65310_body51_e101879_d_n10;
            locals.var_dps0_dn11 = assign65310_body51_e101879_d_n11;
            locals.var_dps0_dn14 = assign65310_body51_e101879_d_n14;
            locals.var_dps0_rv = 0.0;
            let (assign65310_body52_e101899, assign65310_body52_e101899_d_n0, assign65310_body52_e101899_d_n2, assign65310_body52_e101899_d_n4, assign65310_body52_e101899_d_n5, assign65310_body52_e101899_d_n6, assign65310_body52_e101899_d_n7, assign65310_body52_e101899_d_n8, assign65310_body52_e101899_d_n9, assign65310_body52_e101899_d_n10, assign65310_body52_e101899_d_n11, assign65310_body52_e101899_d_n14,) = {
    if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1567 == 0.0)) {
        let assign65310_body52_e101897: f64 = (locals.var_phi_s0 + locals.var_dps0);
        (assign65310_body52_e101897, (locals.var_phi_s0_dn0 + locals.var_dps0_dn0), (locals.var_phi_s0_dn2 + locals.var_dps0_dn2), (locals.var_phi_s0_dn4 + locals.var_dps0_dn4), (locals.var_phi_s0_dn5 + locals.var_dps0_dn5), (locals.var_phi_s0_dn6 + locals.var_dps0_dn6), (locals.var_phi_s0_dn7 + locals.var_dps0_dn7), (locals.var_phi_s0_dn8 + locals.var_dps0_dn8), (locals.var_phi_s0_dn9 + locals.var_dps0_dn9), (locals.var_phi_s0_dn10 + locals.var_dps0_dn10), (locals.var_phi_s0_dn11 + locals.var_dps0_dn11), (locals.var_phi_s0_dn14 + locals.var_dps0_dn14),)
    } else {
        (locals.var_phi_s0, locals.var_phi_s0_dn0, locals.var_phi_s0_dn2, locals.var_phi_s0_dn4, locals.var_phi_s0_dn5, locals.var_phi_s0_dn6, locals.var_phi_s0_dn7, locals.var_phi_s0_dn8, locals.var_phi_s0_dn9, locals.var_phi_s0_dn10, locals.var_phi_s0_dn11, locals.var_phi_s0_dn14,)
    }
};
            locals.var_phi_s0 = assign65310_body52_e101899;
            locals.var_phi_s0_dn0 = assign65310_body52_e101899_d_n0;
            locals.var_phi_s0_dn2 = assign65310_body52_e101899_d_n2;
            locals.var_phi_s0_dn4 = assign65310_body52_e101899_d_n4;
            locals.var_phi_s0_dn5 = assign65310_body52_e101899_d_n5;
            locals.var_phi_s0_dn6 = assign65310_body52_e101899_d_n6;
            locals.var_phi_s0_dn7 = assign65310_body52_e101899_d_n7;
            locals.var_phi_s0_dn8 = assign65310_body52_e101899_d_n8;
            locals.var_phi_s0_dn9 = assign65310_body52_e101899_d_n9;
            locals.var_phi_s0_dn10 = assign65310_body52_e101899_d_n10;
            locals.var_phi_s0_dn11 = assign65310_body52_e101899_d_n11;
            locals.var_phi_s0_dn14 = assign65310_body52_e101899_d_n14;
            locals.var_phi_s0_rv = 0.0;
            let assign65310_body53_e101901: f64 = (locals.var_dps0).abs();
            let assign65310_body53_e101905: f64 = (locals.var_fs0).abs();
            let assign65310_body53_e101908: f64 = if ((assign65310_body53_e101901 <= 1e-12) && (assign65310_body53_e101905 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard1569 = assign65310_body53_e101908;
            locals.var_guard1569_rv = 0.0;
            let (assign65310_body54_e101928,) = {
    if (((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) && (locals.var_guard1567 == 0.0)) && (locals.var_guard1569 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign65310_body54_e101928;
            locals.var_flg_conv_rv = 0.0;
            let (assign65310_body55_e101945,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) {
        let assign65310_body55_e101943: f64 = (locals.var_lp_s0 + 1.0);
        (assign65310_body55_e101943,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign65310_body55_e101945;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_246(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign65320_e101960, assign65320_e101960_d_n0, assign65320_e101960_d_n2, assign65320_e101960_d_n4, assign65320_e101960_d_n5, assign65320_e101960_d_n6, assign65320_e101960_d_n7, assign65320_e101960_d_n8, assign65320_e101960_d_n9, assign65320_e101960_d_n10, assign65320_e101960_d_n11, assign65320_e101960_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1542 == 0.0)) && (locals.var_guard1548 != 0.0)) && (locals.var_guard1558 == 0.0)) {
        (locals.var_phi_s0, locals.var_phi_s0_dn0, locals.var_phi_s0_dn2, locals.var_phi_s0_dn4, locals.var_phi_s0_dn5, locals.var_phi_s0_dn6, locals.var_phi_s0_dn7, locals.var_phi_s0_dn8, locals.var_phi_s0_dn9, locals.var_phi_s0_dn10, locals.var_phi_s0_dn11, locals.var_phi_s0_dn14,)
    } else {
        (locals.var_ps0__blk1527, locals.var_ps0__blk1527_dn0, locals.var_ps0__blk1527_dn2, locals.var_ps0__blk1527_dn4, locals.var_ps0__blk1527_dn5, locals.var_ps0__blk1527_dn6, locals.var_ps0__blk1527_dn7, locals.var_ps0__blk1527_dn8, locals.var_ps0__blk1527_dn9, locals.var_ps0__blk1527_dn10, locals.var_ps0__blk1527_dn11, locals.var_ps0__blk1527_dn14,)
    }
};
        locals.var_ps0__blk1527 = assign65320_e101960;
        locals.var_ps0__blk1527_dn0 = assign65320_e101960_d_n0;
        locals.var_ps0__blk1527_dn2 = assign65320_e101960_d_n2;
        locals.var_ps0__blk1527_dn4 = assign65320_e101960_d_n4;
        locals.var_ps0__blk1527_dn5 = assign65320_e101960_d_n5;
        locals.var_ps0__blk1527_dn6 = assign65320_e101960_d_n6;
        locals.var_ps0__blk1527_dn7 = assign65320_e101960_d_n7;
        locals.var_ps0__blk1527_dn8 = assign65320_e101960_d_n8;
        locals.var_ps0__blk1527_dn9 = assign65320_e101960_d_n9;
        locals.var_ps0__blk1527_dn10 = assign65320_e101960_d_n10;
        locals.var_ps0__blk1527_dn11 = assign65320_e101960_d_n11;
        locals.var_ps0__blk1527_dn14 = assign65320_e101960_d_n14;
        locals.var_ps0__blk1527_rv = 0.0;

        let (assign65330_e101972, assign65330_e101972_d_n0, assign65330_e101972_d_n2, assign65330_e101972_d_n4, assign65330_e101972_d_n5, assign65330_e101972_d_n6, assign65330_e101972_d_n7, assign65330_e101972_d_n8, assign65330_e101972_d_n9, assign65330_e101972_d_n10, assign65330_e101972_d_n11, assign65330_e101972_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) {
        let assign65330_e101966: f64 = (-locals.var_beta);
        let assign65330_e101969: f64 = (locals.var_ps0__blk1527 - locals.var_dphi_vds);
        let assign65330_e101970: f64 = (assign65330_e101966 * assign65330_e101969);
        (assign65330_e101970, (((-locals.var_beta_dn0) * assign65330_e101969) + (assign65330_e101966 * (locals.var_ps0__blk1527_dn0 - locals.var_dphi_vds_dn0))), (((-locals.var_beta_dn2) * assign65330_e101969) + (assign65330_e101966 * (locals.var_ps0__blk1527_dn2 - locals.var_dphi_vds_dn2))), (((-locals.var_beta_dn4) * assign65330_e101969) + (assign65330_e101966 * (locals.var_ps0__blk1527_dn4 - locals.var_dphi_vds_dn4))), (((-locals.var_beta_dn5) * assign65330_e101969) + (assign65330_e101966 * (locals.var_ps0__blk1527_dn5 - locals.var_dphi_vds_dn5))), (((-locals.var_beta_dn6) * assign65330_e101969) + (assign65330_e101966 * (locals.var_ps0__blk1527_dn6 - locals.var_dphi_vds_dn6))), (((-locals.var_beta_dn7) * assign65330_e101969) + (assign65330_e101966 * (locals.var_ps0__blk1527_dn7 - locals.var_dphi_vds_dn7))), (((-locals.var_beta_dn8) * assign65330_e101969) + (assign65330_e101966 * (locals.var_ps0__blk1527_dn8 - locals.var_dphi_vds_dn8))), (((-locals.var_beta_dn9) * assign65330_e101969) + (assign65330_e101966 * (locals.var_ps0__blk1527_dn9 - locals.var_dphi_vds_dn9))), (((-locals.var_beta_dn10) * assign65330_e101969) + (assign65330_e101966 * (locals.var_ps0__blk1527_dn10 - locals.var_dphi_vds_dn10))), (((-locals.var_beta_dn11) * assign65330_e101969) + (assign65330_e101966 * (locals.var_ps0__blk1527_dn11 - locals.var_dphi_vds_dn11))), (((-locals.var_beta_dn14) * assign65330_e101969) + (assign65330_e101966 * (locals.var_ps0__blk1527_dn14 - locals.var_dphi_vds_dn14))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign65330_e101972;
        locals.var_t5_dn0 = assign65330_e101972_d_n0;
        locals.var_t5_dn2 = assign65330_e101972_d_n2;
        locals.var_t5_dn4 = assign65330_e101972_d_n4;
        locals.var_t5_dn5 = assign65330_e101972_d_n5;
        locals.var_t5_dn6 = assign65330_e101972_d_n6;
        locals.var_t5_dn7 = assign65330_e101972_d_n7;
        locals.var_t5_dn8 = assign65330_e101972_d_n8;
        locals.var_t5_dn9 = assign65330_e101972_d_n9;
        locals.var_t5_dn10 = assign65330_e101972_d_n10;
        locals.var_t5_dn11 = assign65330_e101972_d_n11;
        locals.var_t5_dn14 = assign65330_e101972_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign65340_e101980, assign65340_e101980_d_n0, assign65340_e101980_d_n2, assign65340_e101980_d_n4, assign65340_e101980_d_n5, assign65340_e101980_d_n6, assign65340_e101980_d_n7, assign65340_e101980_d_n8, assign65340_e101980_d_n9, assign65340_e101980_d_n10, assign65340_e101980_d_n11, assign65340_e101980_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) {
        let assign65340_e101978: f64 = (locals.var_t5).abs();
        (assign65340_e101978, if locals.var_t5 >= 0.0 { locals.var_t5_dn0 } else { (-locals.var_t5_dn0) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn2 } else { (-locals.var_t5_dn2) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn4 } else { (-locals.var_t5_dn4) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn5 } else { (-locals.var_t5_dn5) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn6 } else { (-locals.var_t5_dn6) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn7 } else { (-locals.var_t5_dn7) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn8 } else { (-locals.var_t5_dn8) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn9 } else { (-locals.var_t5_dn9) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn10 } else { (-locals.var_t5_dn10) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn11 } else { (-locals.var_t5_dn11) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn14 } else { (-locals.var_t5_dn14) },)
    } else {
        (locals.var_t5abs, locals.var_t5abs_dn0, locals.var_t5abs_dn2, locals.var_t5abs_dn4, locals.var_t5abs_dn5, locals.var_t5abs_dn6, locals.var_t5abs_dn7, locals.var_t5abs_dn8, locals.var_t5abs_dn9, locals.var_t5abs_dn10, locals.var_t5abs_dn11, locals.var_t5abs_dn14,)
    }
};
        locals.var_t5abs = assign65340_e101980;
        locals.var_t5abs_dn0 = assign65340_e101980_d_n0;
        locals.var_t5abs_dn2 = assign65340_e101980_d_n2;
        locals.var_t5abs_dn4 = assign65340_e101980_d_n4;
        locals.var_t5abs_dn5 = assign65340_e101980_d_n5;
        locals.var_t5abs_dn6 = assign65340_e101980_d_n6;
        locals.var_t5abs_dn7 = assign65340_e101980_d_n7;
        locals.var_t5abs_dn8 = assign65340_e101980_d_n8;
        locals.var_t5abs_dn9 = assign65340_e101980_d_n9;
        locals.var_t5abs_dn10 = assign65340_e101980_d_n10;
        locals.var_t5abs_dn11 = assign65340_e101980_d_n11;
        locals.var_t5abs_dn14 = assign65340_e101980_d_n14;
        locals.var_t5abs_rv = 0.0;

        let (assign65350_e101988, assign65350_e101988_d_n0, assign65350_e101988_d_n2, assign65350_e101988_d_n4, assign65350_e101988_d_n5, assign65350_e101988_d_n6, assign65350_e101988_d_n7, assign65350_e101988_d_n8, assign65350_e101988_d_n9, assign65350_e101988_d_n10, assign65350_e101988_d_n11, assign65350_e101988_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) {
        let assign65350_e101986: f64 = (locals.var_t5).exp();
        (assign65350_e101986, (assign65350_e101986 * locals.var_t5_dn0), (assign65350_e101986 * locals.var_t5_dn2), (assign65350_e101986 * locals.var_t5_dn4), (assign65350_e101986 * locals.var_t5_dn5), (assign65350_e101986 * locals.var_t5_dn6), (assign65350_e101986 * locals.var_t5_dn7), (assign65350_e101986 * locals.var_t5_dn8), (assign65350_e101986 * locals.var_t5_dn9), (assign65350_e101986 * locals.var_t5_dn10), (assign65350_e101986 * locals.var_t5_dn11), (assign65350_e101986 * locals.var_t5_dn14),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign65350_e101988;
        locals.var_t6_dn0 = assign65350_e101988_d_n0;
        locals.var_t6_dn2 = assign65350_e101988_d_n2;
        locals.var_t6_dn4 = assign65350_e101988_d_n4;
        locals.var_t6_dn5 = assign65350_e101988_d_n5;
        locals.var_t6_dn6 = assign65350_e101988_d_n6;
        locals.var_t6_dn7 = assign65350_e101988_d_n7;
        locals.var_t6_dn8 = assign65350_e101988_d_n8;
        locals.var_t6_dn9 = assign65350_e101988_d_n9;
        locals.var_t6_dn10 = assign65350_e101988_d_n10;
        locals.var_t6_dn11 = assign65350_e101988_d_n11;
        locals.var_t6_dn14 = assign65350_e101988_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign65360_e101999, assign65360_e101999_d_n0, assign65360_e101999_d_n2, assign65360_e101999_d_n4, assign65360_e101999_d_n5, assign65360_e101999_d_n6, assign65360_e101999_d_n7, assign65360_e101999_d_n8, assign65360_e101999_d_n9, assign65360_e101999_d_n10, assign65360_e101999_d_n11, assign65360_e101999_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) {
        let assign65360_e101995: f64 = (locals.var_t6 - 1.0);
        let assign65360_e101997: f64 = (assign65360_e101995 - locals.var_t5);
        (assign65360_e101997, (locals.var_t6_dn0 - locals.var_t5_dn0), (locals.var_t6_dn2 - locals.var_t5_dn2), (locals.var_t6_dn4 - locals.var_t5_dn4), (locals.var_t6_dn5 - locals.var_t5_dn5), (locals.var_t6_dn6 - locals.var_t5_dn6), (locals.var_t6_dn7 - locals.var_t5_dn7), (locals.var_t6_dn8 - locals.var_t5_dn8), (locals.var_t6_dn9 - locals.var_t5_dn9), (locals.var_t6_dn10 - locals.var_t5_dn10), (locals.var_t6_dn11 - locals.var_t5_dn11), (locals.var_t6_dn14 - locals.var_t5_dn14),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign65360_e101999;
        locals.var_t7_dn0 = assign65360_e101999_d_n0;
        locals.var_t7_dn2 = assign65360_e101999_d_n2;
        locals.var_t7_dn4 = assign65360_e101999_d_n4;
        locals.var_t7_dn5 = assign65360_e101999_d_n5;
        locals.var_t7_dn6 = assign65360_e101999_d_n6;
        locals.var_t7_dn7 = assign65360_e101999_d_n7;
        locals.var_t7_dn8 = assign65360_e101999_d_n8;
        locals.var_t7_dn9 = assign65360_e101999_d_n9;
        locals.var_t7_dn10 = assign65360_e101999_d_n10;
        locals.var_t7_dn11 = assign65360_e101999_d_n11;
        locals.var_t7_dn14 = assign65360_e101999_d_n14;
        locals.var_t7_rv = 0.0;

        let assign65370_e102002: f64 = if locals.var_t5 > 1e-7 { 1.0 } else { 0.0 };
        locals.var_guard1570 = assign65370_e102002;
        locals.var_guard1570_rv = 0.0;

        let (assign65380_e102015, assign65380_e102015_d_n0, assign65380_e102015_d_n2, assign65380_e102015_d_n4, assign65380_e102015_d_n5, assign65380_e102015_d_n6, assign65380_e102015_d_n7, assign65380_e102015_d_n8, assign65380_e102015_d_n9, assign65380_e102015_d_n10, assign65380_e102015_d_n11, assign65380_e102015_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1570 != 0.0)) {
        let assign65380_e102010: f64 = (-locals.var_cnst0);
        let assign65380_e102012: f64 = (locals.var_t7).sqrt();
        let assign65380_e102013: f64 = (assign65380_e102010 * assign65380_e102012);
        (assign65380_e102013, (((-locals.var_cnst0_dn0) * assign65380_e102012) + (assign65380_e102010 * (locals.var_t7_dn0 / (2.0 * assign65380_e102012)))), (((-locals.var_cnst0_dn2) * assign65380_e102012) + (assign65380_e102010 * (locals.var_t7_dn2 / (2.0 * assign65380_e102012)))), (((-locals.var_cnst0_dn4) * assign65380_e102012) + (assign65380_e102010 * (locals.var_t7_dn4 / (2.0 * assign65380_e102012)))), (((-locals.var_cnst0_dn5) * assign65380_e102012) + (assign65380_e102010 * (locals.var_t7_dn5 / (2.0 * assign65380_e102012)))), (((-locals.var_cnst0_dn6) * assign65380_e102012) + (assign65380_e102010 * (locals.var_t7_dn6 / (2.0 * assign65380_e102012)))), (((-locals.var_cnst0_dn7) * assign65380_e102012) + (assign65380_e102010 * (locals.var_t7_dn7 / (2.0 * assign65380_e102012)))), (((-locals.var_cnst0_dn8) * assign65380_e102012) + (assign65380_e102010 * (locals.var_t7_dn8 / (2.0 * assign65380_e102012)))), (((-locals.var_cnst0_dn9) * assign65380_e102012) + (assign65380_e102010 * (locals.var_t7_dn9 / (2.0 * assign65380_e102012)))), (((-locals.var_cnst0_dn10) * assign65380_e102012) + (assign65380_e102010 * (locals.var_t7_dn10 / (2.0 * assign65380_e102012)))), (((-locals.var_cnst0_dn11) * assign65380_e102012) + (assign65380_e102010 * (locals.var_t7_dn11 / (2.0 * assign65380_e102012)))), (((-locals.var_cnst0_dn14) * assign65380_e102012) + (assign65380_e102010 * (locals.var_t7_dn14 / (2.0 * assign65380_e102012)))),)
    } else {
        (locals.var_qbu__blk1541, locals.var_qbu__blk1541_dn0, locals.var_qbu__blk1541_dn2, locals.var_qbu__blk1541_dn4, locals.var_qbu__blk1541_dn5, locals.var_qbu__blk1541_dn6, locals.var_qbu__blk1541_dn7, locals.var_qbu__blk1541_dn8, locals.var_qbu__blk1541_dn9, locals.var_qbu__blk1541_dn10, locals.var_qbu__blk1541_dn11, locals.var_qbu__blk1541_dn14,)
    }
};
        locals.var_qbu__blk1541 = assign65380_e102015;
        locals.var_qbu__blk1541_dn0 = assign65380_e102015_d_n0;
        locals.var_qbu__blk1541_dn2 = assign65380_e102015_d_n2;
        locals.var_qbu__blk1541_dn4 = assign65380_e102015_d_n4;
        locals.var_qbu__blk1541_dn5 = assign65380_e102015_d_n5;
        locals.var_qbu__blk1541_dn6 = assign65380_e102015_d_n6;
        locals.var_qbu__blk1541_dn7 = assign65380_e102015_d_n7;
        locals.var_qbu__blk1541_dn8 = assign65380_e102015_d_n8;
        locals.var_qbu__blk1541_dn9 = assign65380_e102015_d_n9;
        locals.var_qbu__blk1541_dn10 = assign65380_e102015_d_n10;
        locals.var_qbu__blk1541_dn11 = assign65380_e102015_d_n11;
        locals.var_qbu__blk1541_dn14 = assign65380_e102015_d_n14;
        locals.var_qbu__blk1541_rv = 0.0;

        let assign65390_e102018: f64 = if locals.var_t5abs > 1e-7 { 1.0 } else { 0.0 };
        locals.var_guard1571 = assign65390_e102018;
        locals.var_guard1571_rv = 0.0;

        let (assign65400_e102033, assign65400_e102033_d_n0, assign65400_e102033_d_n2, assign65400_e102033_d_n4, assign65400_e102033_d_n5, assign65400_e102033_d_n6, assign65400_e102033_d_n7, assign65400_e102033_d_n8, assign65400_e102033_d_n9, assign65400_e102033_d_n10, assign65400_e102033_d_n11, assign65400_e102033_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1570 == 0.0)) && (locals.var_guard1571 != 0.0)) {
        let assign65400_e102030: f64 = (locals.var_t7).sqrt();
        let assign65400_e102031: f64 = (locals.var_cnst0 * assign65400_e102030);
        (assign65400_e102031, ((locals.var_cnst0_dn0 * assign65400_e102030) + (locals.var_cnst0 * (locals.var_t7_dn0 / (2.0 * assign65400_e102030)))), ((locals.var_cnst0_dn2 * assign65400_e102030) + (locals.var_cnst0 * (locals.var_t7_dn2 / (2.0 * assign65400_e102030)))), ((locals.var_cnst0_dn4 * assign65400_e102030) + (locals.var_cnst0 * (locals.var_t7_dn4 / (2.0 * assign65400_e102030)))), ((locals.var_cnst0_dn5 * assign65400_e102030) + (locals.var_cnst0 * (locals.var_t7_dn5 / (2.0 * assign65400_e102030)))), ((locals.var_cnst0_dn6 * assign65400_e102030) + (locals.var_cnst0 * (locals.var_t7_dn6 / (2.0 * assign65400_e102030)))), ((locals.var_cnst0_dn7 * assign65400_e102030) + (locals.var_cnst0 * (locals.var_t7_dn7 / (2.0 * assign65400_e102030)))), ((locals.var_cnst0_dn8 * assign65400_e102030) + (locals.var_cnst0 * (locals.var_t7_dn8 / (2.0 * assign65400_e102030)))), ((locals.var_cnst0_dn9 * assign65400_e102030) + (locals.var_cnst0 * (locals.var_t7_dn9 / (2.0 * assign65400_e102030)))), ((locals.var_cnst0_dn10 * assign65400_e102030) + (locals.var_cnst0 * (locals.var_t7_dn10 / (2.0 * assign65400_e102030)))), ((locals.var_cnst0_dn11 * assign65400_e102030) + (locals.var_cnst0 * (locals.var_t7_dn11 / (2.0 * assign65400_e102030)))), ((locals.var_cnst0_dn14 * assign65400_e102030) + (locals.var_cnst0 * (locals.var_t7_dn14 / (2.0 * assign65400_e102030)))),)
    } else {
        (locals.var_qbu__blk1541, locals.var_qbu__blk1541_dn0, locals.var_qbu__blk1541_dn2, locals.var_qbu__blk1541_dn4, locals.var_qbu__blk1541_dn5, locals.var_qbu__blk1541_dn6, locals.var_qbu__blk1541_dn7, locals.var_qbu__blk1541_dn8, locals.var_qbu__blk1541_dn9, locals.var_qbu__blk1541_dn10, locals.var_qbu__blk1541_dn11, locals.var_qbu__blk1541_dn14,)
    }
};
        locals.var_qbu__blk1541 = assign65400_e102033;
        locals.var_qbu__blk1541_dn0 = assign65400_e102033_d_n0;
        locals.var_qbu__blk1541_dn2 = assign65400_e102033_d_n2;
        locals.var_qbu__blk1541_dn4 = assign65400_e102033_d_n4;
        locals.var_qbu__blk1541_dn5 = assign65400_e102033_d_n5;
        locals.var_qbu__blk1541_dn6 = assign65400_e102033_d_n6;
        locals.var_qbu__blk1541_dn7 = assign65400_e102033_d_n7;
        locals.var_qbu__blk1541_dn8 = assign65400_e102033_d_n8;
        locals.var_qbu__blk1541_dn9 = assign65400_e102033_d_n9;
        locals.var_qbu__blk1541_dn10 = assign65400_e102033_d_n10;
        locals.var_qbu__blk1541_dn11 = assign65400_e102033_d_n11;
        locals.var_qbu__blk1541_dn14 = assign65400_e102033_d_n14;
        locals.var_qbu__blk1541_rv = 0.0;

        let (assign65410_e102062, assign65410_e102062_d_n0, assign65410_e102062_d_n2, assign65410_e102062_d_n4, assign65410_e102062_d_n5, assign65410_e102062_d_n6, assign65410_e102062_d_n7, assign65410_e102062_d_n8, assign65410_e102062_d_n9, assign65410_e102062_d_n10, assign65410_e102062_d_n11, assign65410_e102062_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1570 == 0.0)) && (locals.var_guard1571 == 0.0)) {
        let assign65410_e102045: f64 = (-locals.var_t5);
        let assign65410_e102047: f64 = (assign65410_e102045 * 0.7071067811865475);
        let assign65410_e102051: f64 = (locals.var_t5abs * 0.3333333333333333);
        let assign65410_e102055: f64 = (0.25 * locals.var_t5abs);
        let assign65410_e102056: f64 = (1.0 + assign65410_e102055);
        let assign65410_e102057: f64 = (assign65410_e102051 * assign65410_e102056);
        let assign65410_e102058: f64 = (1.0 + assign65410_e102057);
        let assign65410_e102059: f64 = (assign65410_e102058).sqrt();
        let assign65410_e102060: f64 = (assign65410_e102047 * assign65410_e102059);
        (assign65410_e102060, ((((-locals.var_t5_dn0) * 0.7071067811865475) * assign65410_e102059) + (assign65410_e102047 * ((((locals.var_t5abs_dn0 * 0.3333333333333333) * assign65410_e102056) + (assign65410_e102051 * (0.25 * locals.var_t5abs_dn0))) / (2.0 * assign65410_e102059)))), ((((-locals.var_t5_dn2) * 0.7071067811865475) * assign65410_e102059) + (assign65410_e102047 * ((((locals.var_t5abs_dn2 * 0.3333333333333333) * assign65410_e102056) + (assign65410_e102051 * (0.25 * locals.var_t5abs_dn2))) / (2.0 * assign65410_e102059)))), ((((-locals.var_t5_dn4) * 0.7071067811865475) * assign65410_e102059) + (assign65410_e102047 * ((((locals.var_t5abs_dn4 * 0.3333333333333333) * assign65410_e102056) + (assign65410_e102051 * (0.25 * locals.var_t5abs_dn4))) / (2.0 * assign65410_e102059)))), ((((-locals.var_t5_dn5) * 0.7071067811865475) * assign65410_e102059) + (assign65410_e102047 * ((((locals.var_t5abs_dn5 * 0.3333333333333333) * assign65410_e102056) + (assign65410_e102051 * (0.25 * locals.var_t5abs_dn5))) / (2.0 * assign65410_e102059)))), ((((-locals.var_t5_dn6) * 0.7071067811865475) * assign65410_e102059) + (assign65410_e102047 * ((((locals.var_t5abs_dn6 * 0.3333333333333333) * assign65410_e102056) + (assign65410_e102051 * (0.25 * locals.var_t5abs_dn6))) / (2.0 * assign65410_e102059)))), ((((-locals.var_t5_dn7) * 0.7071067811865475) * assign65410_e102059) + (assign65410_e102047 * ((((locals.var_t5abs_dn7 * 0.3333333333333333) * assign65410_e102056) + (assign65410_e102051 * (0.25 * locals.var_t5abs_dn7))) / (2.0 * assign65410_e102059)))), ((((-locals.var_t5_dn8) * 0.7071067811865475) * assign65410_e102059) + (assign65410_e102047 * ((((locals.var_t5abs_dn8 * 0.3333333333333333) * assign65410_e102056) + (assign65410_e102051 * (0.25 * locals.var_t5abs_dn8))) / (2.0 * assign65410_e102059)))), ((((-locals.var_t5_dn9) * 0.7071067811865475) * assign65410_e102059) + (assign65410_e102047 * ((((locals.var_t5abs_dn9 * 0.3333333333333333) * assign65410_e102056) + (assign65410_e102051 * (0.25 * locals.var_t5abs_dn9))) / (2.0 * assign65410_e102059)))), ((((-locals.var_t5_dn10) * 0.7071067811865475) * assign65410_e102059) + (assign65410_e102047 * ((((locals.var_t5abs_dn10 * 0.3333333333333333) * assign65410_e102056) + (assign65410_e102051 * (0.25 * locals.var_t5abs_dn10))) / (2.0 * assign65410_e102059)))), ((((-locals.var_t5_dn11) * 0.7071067811865475) * assign65410_e102059) + (assign65410_e102047 * ((((locals.var_t5abs_dn11 * 0.3333333333333333) * assign65410_e102056) + (assign65410_e102051 * (0.25 * locals.var_t5abs_dn11))) / (2.0 * assign65410_e102059)))), ((((-locals.var_t5_dn14) * 0.7071067811865475) * assign65410_e102059) + (assign65410_e102047 * ((((locals.var_t5abs_dn14 * 0.3333333333333333) * assign65410_e102056) + (assign65410_e102051 * (0.25 * locals.var_t5abs_dn14))) / (2.0 * assign65410_e102059)))),)
    } else {
        (locals.var_qbu__blk1541, locals.var_qbu__blk1541_dn0, locals.var_qbu__blk1541_dn2, locals.var_qbu__blk1541_dn4, locals.var_qbu__blk1541_dn5, locals.var_qbu__blk1541_dn6, locals.var_qbu__blk1541_dn7, locals.var_qbu__blk1541_dn8, locals.var_qbu__blk1541_dn9, locals.var_qbu__blk1541_dn10, locals.var_qbu__blk1541_dn11, locals.var_qbu__blk1541_dn14,)
    }
};
        locals.var_qbu__blk1541 = assign65410_e102062;
        locals.var_qbu__blk1541_dn0 = assign65410_e102062_d_n0;
        locals.var_qbu__blk1541_dn2 = assign65410_e102062_d_n2;
        locals.var_qbu__blk1541_dn4 = assign65410_e102062_d_n4;
        locals.var_qbu__blk1541_dn5 = assign65410_e102062_d_n5;
        locals.var_qbu__blk1541_dn6 = assign65410_e102062_d_n6;
        locals.var_qbu__blk1541_dn7 = assign65410_e102062_d_n7;
        locals.var_qbu__blk1541_dn8 = assign65410_e102062_d_n8;
        locals.var_qbu__blk1541_dn9 = assign65410_e102062_d_n9;
        locals.var_qbu__blk1541_dn10 = assign65410_e102062_d_n10;
        locals.var_qbu__blk1541_dn11 = assign65410_e102062_d_n11;
        locals.var_qbu__blk1541_dn14 = assign65410_e102062_d_n14;
        locals.var_qbu__blk1541_rv = 0.0;

        let (assign65420_e102078, assign65420_e102078_d_n0, assign65420_e102078_d_n2, assign65420_e102078_d_n4, assign65420_e102078_d_n5, assign65420_e102078_d_n6, assign65420_e102078_d_n7, assign65420_e102078_d_n8, assign65420_e102078_d_n9, assign65420_e102078_d_n10, assign65420_e102078_d_n11, assign65420_e102078_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) {
        let assign65420_e102069: f64 = (locals.var_qbu__blk1541 * locals.var_qbu__blk1541);
        let assign65420_e102072: f64 = (4.0 * 1e-6);
        let assign65420_e102074: f64 = (assign65420_e102072 * 1e-6);
        let assign65420_e102075: f64 = (assign65420_e102069 + assign65420_e102074);
        let assign65420_e102076: f64 = (assign65420_e102075).sqrt();
        (assign65420_e102076, (((locals.var_qbu__blk1541_dn0 * locals.var_qbu__blk1541) + (locals.var_qbu__blk1541 * locals.var_qbu__blk1541_dn0)) / (2.0 * assign65420_e102076)), (((locals.var_qbu__blk1541_dn2 * locals.var_qbu__blk1541) + (locals.var_qbu__blk1541 * locals.var_qbu__blk1541_dn2)) / (2.0 * assign65420_e102076)), (((locals.var_qbu__blk1541_dn4 * locals.var_qbu__blk1541) + (locals.var_qbu__blk1541 * locals.var_qbu__blk1541_dn4)) / (2.0 * assign65420_e102076)), (((locals.var_qbu__blk1541_dn5 * locals.var_qbu__blk1541) + (locals.var_qbu__blk1541 * locals.var_qbu__blk1541_dn5)) / (2.0 * assign65420_e102076)), (((locals.var_qbu__blk1541_dn6 * locals.var_qbu__blk1541) + (locals.var_qbu__blk1541 * locals.var_qbu__blk1541_dn6)) / (2.0 * assign65420_e102076)), (((locals.var_qbu__blk1541_dn7 * locals.var_qbu__blk1541) + (locals.var_qbu__blk1541 * locals.var_qbu__blk1541_dn7)) / (2.0 * assign65420_e102076)), (((locals.var_qbu__blk1541_dn8 * locals.var_qbu__blk1541) + (locals.var_qbu__blk1541 * locals.var_qbu__blk1541_dn8)) / (2.0 * assign65420_e102076)), (((locals.var_qbu__blk1541_dn9 * locals.var_qbu__blk1541) + (locals.var_qbu__blk1541 * locals.var_qbu__blk1541_dn9)) / (2.0 * assign65420_e102076)), (((locals.var_qbu__blk1541_dn10 * locals.var_qbu__blk1541) + (locals.var_qbu__blk1541 * locals.var_qbu__blk1541_dn10)) / (2.0 * assign65420_e102076)), (((locals.var_qbu__blk1541_dn11 * locals.var_qbu__blk1541) + (locals.var_qbu__blk1541 * locals.var_qbu__blk1541_dn11)) / (2.0 * assign65420_e102076)), (((locals.var_qbu__blk1541_dn14 * locals.var_qbu__blk1541) + (locals.var_qbu__blk1541 * locals.var_qbu__blk1541_dn14)) / (2.0 * assign65420_e102076)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign65420_e102078;
        locals.var_tmf1_dn0 = assign65420_e102078_d_n0;
        locals.var_tmf1_dn2 = assign65420_e102078_d_n2;
        locals.var_tmf1_dn4 = assign65420_e102078_d_n4;
        locals.var_tmf1_dn5 = assign65420_e102078_d_n5;
        locals.var_tmf1_dn6 = assign65420_e102078_d_n6;
        locals.var_tmf1_dn7 = assign65420_e102078_d_n7;
        locals.var_tmf1_dn8 = assign65420_e102078_d_n8;
        locals.var_tmf1_dn9 = assign65420_e102078_d_n9;
        locals.var_tmf1_dn10 = assign65420_e102078_d_n10;
        locals.var_tmf1_dn11 = assign65420_e102078_d_n11;
        locals.var_tmf1_dn14 = assign65420_e102078_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign65430_e102089, assign65430_e102089_d_n0, assign65430_e102089_d_n2, assign65430_e102089_d_n4, assign65430_e102089_d_n5, assign65430_e102089_d_n6, assign65430_e102089_d_n7, assign65430_e102089_d_n8, assign65430_e102089_d_n9, assign65430_e102089_d_n10, assign65430_e102089_d_n11, assign65430_e102089_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) {
        let assign65430_e102086: f64 = (locals.var_qbu__blk1541 + locals.var_tmf1);
        let assign65430_e102087: f64 = (0.5 * assign65430_e102086);
        (assign65430_e102087, (0.5 * (locals.var_qbu__blk1541_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_qbu__blk1541_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_qbu__blk1541_dn4 + locals.var_tmf1_dn4)), (0.5 * (locals.var_qbu__blk1541_dn5 + locals.var_tmf1_dn5)), (0.5 * (locals.var_qbu__blk1541_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_qbu__blk1541_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_qbu__blk1541_dn8 + locals.var_tmf1_dn8)), (0.5 * (locals.var_qbu__blk1541_dn9 + locals.var_tmf1_dn9)), (0.5 * (locals.var_qbu__blk1541_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_qbu__blk1541_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_qbu__blk1541_dn14 + locals.var_tmf1_dn14)),)
    } else {
        (locals.var_wqbu, locals.var_wqbu_dn0, locals.var_wqbu_dn2, locals.var_wqbu_dn4, locals.var_wqbu_dn5, locals.var_wqbu_dn6, locals.var_wqbu_dn7, locals.var_wqbu_dn8, locals.var_wqbu_dn9, locals.var_wqbu_dn10, locals.var_wqbu_dn11, locals.var_wqbu_dn14,)
    }
};
        locals.var_wqbu = assign65430_e102089;
        locals.var_wqbu_dn0 = assign65430_e102089_d_n0;
        locals.var_wqbu_dn2 = assign65430_e102089_d_n2;
        locals.var_wqbu_dn4 = assign65430_e102089_d_n4;
        locals.var_wqbu_dn5 = assign65430_e102089_d_n5;
        locals.var_wqbu_dn6 = assign65430_e102089_d_n6;
        locals.var_wqbu_dn7 = assign65430_e102089_d_n7;
        locals.var_wqbu_dn8 = assign65430_e102089_d_n8;
        locals.var_wqbu_dn9 = assign65430_e102089_d_n9;
        locals.var_wqbu_dn10 = assign65430_e102089_d_n10;
        locals.var_wqbu_dn11 = assign65430_e102089_d_n11;
        locals.var_wqbu_dn14 = assign65430_e102089_d_n14;
        locals.var_wqbu_rv = 0.0;

        let (assign65440_e102100, assign65440_e102100_d_n0, assign65440_e102100_d_n2, assign65440_e102100_d_n4, assign65440_e102100_d_n5, assign65440_e102100_d_n6, assign65440_e102100_d_n7, assign65440_e102100_d_n8, assign65440_e102100_d_n9, assign65440_e102100_d_n10, assign65440_e102100_d_n11, assign65440_e102100_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) {
        let assign65440_e102097: f64 = (1.6021918e-19 * locals.var_nsub);
        let assign65440_e102098: f64 = (locals.var_wqbu / assign65440_e102097);
        (assign65440_e102098, (((locals.var_wqbu_dn0 * assign65440_e102097) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn0))) / (assign65440_e102097 * assign65440_e102097)), (((locals.var_wqbu_dn2 * assign65440_e102097) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn2))) / (assign65440_e102097 * assign65440_e102097)), (((locals.var_wqbu_dn4 * assign65440_e102097) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn4))) / (assign65440_e102097 * assign65440_e102097)), (((locals.var_wqbu_dn5 * assign65440_e102097) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn5))) / (assign65440_e102097 * assign65440_e102097)), (((locals.var_wqbu_dn6 * assign65440_e102097) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn6))) / (assign65440_e102097 * assign65440_e102097)), (((locals.var_wqbu_dn7 * assign65440_e102097) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn7))) / (assign65440_e102097 * assign65440_e102097)), (((locals.var_wqbu_dn8 * assign65440_e102097) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn8))) / (assign65440_e102097 * assign65440_e102097)), (((locals.var_wqbu_dn9 * assign65440_e102097) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn9))) / (assign65440_e102097 * assign65440_e102097)), (((locals.var_wqbu_dn10 * assign65440_e102097) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn10))) / (assign65440_e102097 * assign65440_e102097)), (((locals.var_wqbu_dn11 * assign65440_e102097) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn11))) / (assign65440_e102097 * assign65440_e102097)), (((locals.var_wqbu_dn14 * assign65440_e102097) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn14))) / (assign65440_e102097 * assign65440_e102097)),)
    } else {
        (locals.var_wdep__blk1537, locals.var_wdep__blk1537_dn0, locals.var_wdep__blk1537_dn2, locals.var_wdep__blk1537_dn4, locals.var_wdep__blk1537_dn5, locals.var_wdep__blk1537_dn6, locals.var_wdep__blk1537_dn7, locals.var_wdep__blk1537_dn8, locals.var_wdep__blk1537_dn9, locals.var_wdep__blk1537_dn10, locals.var_wdep__blk1537_dn11, locals.var_wdep__blk1537_dn14,)
    }
};
        locals.var_wdep__blk1537 = assign65440_e102100;
        locals.var_wdep__blk1537_dn0 = assign65440_e102100_d_n0;
        locals.var_wdep__blk1537_dn2 = assign65440_e102100_d_n2;
        locals.var_wdep__blk1537_dn4 = assign65440_e102100_d_n4;
        locals.var_wdep__blk1537_dn5 = assign65440_e102100_d_n5;
        locals.var_wdep__blk1537_dn6 = assign65440_e102100_d_n6;
        locals.var_wdep__blk1537_dn7 = assign65440_e102100_d_n7;
        locals.var_wdep__blk1537_dn8 = assign65440_e102100_d_n8;
        locals.var_wdep__blk1537_dn9 = assign65440_e102100_d_n9;
        locals.var_wdep__blk1537_dn10 = assign65440_e102100_d_n10;
        locals.var_wdep__blk1537_dn11 = assign65440_e102100_d_n11;
        locals.var_wdep__blk1537_dn14 = assign65440_e102100_d_n14;
        locals.var_wdep__blk1537_rv = 0.0;

        let (assign65450_e102109, assign65450_e102109_d_n0, assign65450_e102109_d_n2, assign65450_e102109_d_n4, assign65450_e102109_d_n5, assign65450_e102109_d_n6, assign65450_e102109_d_n7, assign65450_e102109_d_n8, assign65450_e102109_d_n9, assign65450_e102109_d_n10, assign65450_e102109_d_n11, assign65450_e102109_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) {
        let assign65450_e102107: f64 = (locals.var_wdep__blk1537 - p.p452);
        (assign65450_e102107, locals.var_wdep__blk1537_dn0, locals.var_wdep__blk1537_dn2, locals.var_wdep__blk1537_dn4, locals.var_wdep__blk1537_dn5, locals.var_wdep__blk1537_dn6, locals.var_wdep__blk1537_dn7, locals.var_wdep__blk1537_dn8, locals.var_wdep__blk1537_dn9, locals.var_wdep__blk1537_dn10, locals.var_wdep__blk1537_dn11, locals.var_wdep__blk1537_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign65450_e102109;
        locals.var_t1_dn0 = assign65450_e102109_d_n0;
        locals.var_t1_dn2 = assign65450_e102109_d_n2;
        locals.var_t1_dn4 = assign65450_e102109_d_n4;
        locals.var_t1_dn5 = assign65450_e102109_d_n5;
        locals.var_t1_dn6 = assign65450_e102109_d_n6;
        locals.var_t1_dn7 = assign65450_e102109_d_n7;
        locals.var_t1_dn8 = assign65450_e102109_d_n8;
        locals.var_t1_dn9 = assign65450_e102109_d_n9;
        locals.var_t1_dn10 = assign65450_e102109_d_n10;
        locals.var_t1_dn11 = assign65450_e102109_d_n11;
        locals.var_t1_dn14 = assign65450_e102109_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign65460_e102118, assign65460_e102118_d_n0, assign65460_e102118_d_n2, assign65460_e102118_d_n4, assign65460_e102118_d_n5, assign65460_e102118_d_n6, assign65460_e102118_d_n7, assign65460_e102118_d_n8, assign65460_e102118_d_n9, assign65460_e102118_d_n10, assign65460_e102118_d_n11, assign65460_e102118_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) {
        let assign65460_e102116: f64 = (locals.var_wdep__blk1537 * 0.01);
        (assign65460_e102116, (locals.var_wdep__blk1537_dn0 * 0.01), (locals.var_wdep__blk1537_dn2 * 0.01), (locals.var_wdep__blk1537_dn4 * 0.01), (locals.var_wdep__blk1537_dn5 * 0.01), (locals.var_wdep__blk1537_dn6 * 0.01), (locals.var_wdep__blk1537_dn7 * 0.01), (locals.var_wdep__blk1537_dn8 * 0.01), (locals.var_wdep__blk1537_dn9 * 0.01), (locals.var_wdep__blk1537_dn10 * 0.01), (locals.var_wdep__blk1537_dn11 * 0.01), (locals.var_wdep__blk1537_dn14 * 0.01),)
    } else {
        (locals.var_delta_1, locals.var_delta_1_dn0, locals.var_delta_1_dn2, locals.var_delta_1_dn4, locals.var_delta_1_dn5, locals.var_delta_1_dn6, locals.var_delta_1_dn7, locals.var_delta_1_dn8, locals.var_delta_1_dn9, locals.var_delta_1_dn10, locals.var_delta_1_dn11, locals.var_delta_1_dn14,)
    }
};
        locals.var_delta_1 = assign65460_e102118;
        locals.var_delta_1_dn0 = assign65460_e102118_d_n0;
        locals.var_delta_1_dn2 = assign65460_e102118_d_n2;
        locals.var_delta_1_dn4 = assign65460_e102118_d_n4;
        locals.var_delta_1_dn5 = assign65460_e102118_d_n5;
        locals.var_delta_1_dn6 = assign65460_e102118_d_n6;
        locals.var_delta_1_dn7 = assign65460_e102118_d_n7;
        locals.var_delta_1_dn8 = assign65460_e102118_d_n8;
        locals.var_delta_1_dn9 = assign65460_e102118_d_n9;
        locals.var_delta_1_dn10 = assign65460_e102118_d_n10;
        locals.var_delta_1_dn11 = assign65460_e102118_d_n11;
        locals.var_delta_1_dn14 = assign65460_e102118_d_n14;
        locals.var_delta_1_rv = 0.0;

        let (assign65470_e102134, assign65470_e102134_d_n0, assign65470_e102134_d_n2, assign65470_e102134_d_n4, assign65470_e102134_d_n5, assign65470_e102134_d_n6, assign65470_e102134_d_n7, assign65470_e102134_d_n8, assign65470_e102134_d_n9, assign65470_e102134_d_n10, assign65470_e102134_d_n11, assign65470_e102134_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) {
        let assign65470_e102125: f64 = (locals.var_t1 * locals.var_t1);
        let assign65470_e102128: f64 = (4.0 * locals.var_delta_1);
        let assign65470_e102130: f64 = (assign65470_e102128 * locals.var_delta_1);
        let assign65470_e102131: f64 = (assign65470_e102125 + assign65470_e102130);
        let assign65470_e102132: f64 = (assign65470_e102131).sqrt();
        (assign65470_e102132, ((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) + (((4.0 * locals.var_delta_1_dn0) * locals.var_delta_1) + (assign65470_e102128 * locals.var_delta_1_dn0))) / (2.0 * assign65470_e102132)), ((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) + (((4.0 * locals.var_delta_1_dn2) * locals.var_delta_1) + (assign65470_e102128 * locals.var_delta_1_dn2))) / (2.0 * assign65470_e102132)), ((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) + (((4.0 * locals.var_delta_1_dn4) * locals.var_delta_1) + (assign65470_e102128 * locals.var_delta_1_dn4))) / (2.0 * assign65470_e102132)), ((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) + (((4.0 * locals.var_delta_1_dn5) * locals.var_delta_1) + (assign65470_e102128 * locals.var_delta_1_dn5))) / (2.0 * assign65470_e102132)), ((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + (((4.0 * locals.var_delta_1_dn6) * locals.var_delta_1) + (assign65470_e102128 * locals.var_delta_1_dn6))) / (2.0 * assign65470_e102132)), ((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) + (((4.0 * locals.var_delta_1_dn7) * locals.var_delta_1) + (assign65470_e102128 * locals.var_delta_1_dn7))) / (2.0 * assign65470_e102132)), ((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) + (((4.0 * locals.var_delta_1_dn8) * locals.var_delta_1) + (assign65470_e102128 * locals.var_delta_1_dn8))) / (2.0 * assign65470_e102132)), ((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) + (((4.0 * locals.var_delta_1_dn9) * locals.var_delta_1) + (assign65470_e102128 * locals.var_delta_1_dn9))) / (2.0 * assign65470_e102132)), ((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + (((4.0 * locals.var_delta_1_dn10) * locals.var_delta_1) + (assign65470_e102128 * locals.var_delta_1_dn10))) / (2.0 * assign65470_e102132)), ((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) + (((4.0 * locals.var_delta_1_dn11) * locals.var_delta_1) + (assign65470_e102128 * locals.var_delta_1_dn11))) / (2.0 * assign65470_e102132)), ((((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) + (((4.0 * locals.var_delta_1_dn14) * locals.var_delta_1) + (assign65470_e102128 * locals.var_delta_1_dn14))) / (2.0 * assign65470_e102132)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign65470_e102134;
        locals.var_tmf1_dn0 = assign65470_e102134_d_n0;
        locals.var_tmf1_dn2 = assign65470_e102134_d_n2;
        locals.var_tmf1_dn4 = assign65470_e102134_d_n4;
        locals.var_tmf1_dn5 = assign65470_e102134_d_n5;
        locals.var_tmf1_dn6 = assign65470_e102134_d_n6;
        locals.var_tmf1_dn7 = assign65470_e102134_d_n7;
        locals.var_tmf1_dn8 = assign65470_e102134_d_n8;
        locals.var_tmf1_dn9 = assign65470_e102134_d_n9;
        locals.var_tmf1_dn10 = assign65470_e102134_d_n10;
        locals.var_tmf1_dn11 = assign65470_e102134_d_n11;
        locals.var_tmf1_dn14 = assign65470_e102134_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign65480_e102145, assign65480_e102145_d_n0, assign65480_e102145_d_n2, assign65480_e102145_d_n4, assign65480_e102145_d_n5, assign65480_e102145_d_n6, assign65480_e102145_d_n7, assign65480_e102145_d_n8, assign65480_e102145_d_n9, assign65480_e102145_d_n10, assign65480_e102145_d_n11, assign65480_e102145_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) {
        let assign65480_e102142: f64 = (locals.var_t1 + locals.var_tmf1);
        let assign65480_e102143: f64 = (0.5 * assign65480_e102142);
        (assign65480_e102143, (0.5 * (locals.var_t1_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf1_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf1_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf1_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf1_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t1_dn14 + locals.var_tmf1_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign65480_e102145;
        locals.var_t2_dn0 = assign65480_e102145_d_n0;
        locals.var_t2_dn2 = assign65480_e102145_d_n2;
        locals.var_t2_dn4 = assign65480_e102145_d_n4;
        locals.var_t2_dn5 = assign65480_e102145_d_n5;
        locals.var_t2_dn6 = assign65480_e102145_d_n6;
        locals.var_t2_dn7 = assign65480_e102145_d_n7;
        locals.var_t2_dn8 = assign65480_e102145_d_n8;
        locals.var_t2_dn9 = assign65480_e102145_d_n9;
        locals.var_t2_dn10 = assign65480_e102145_d_n10;
        locals.var_t2_dn11 = assign65480_e102145_d_n11;
        locals.var_t2_dn14 = assign65480_e102145_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign65490_e102158, assign65490_e102158_d_n0, assign65490_e102158_d_n2, assign65490_e102158_d_n4, assign65490_e102158_d_n5, assign65490_e102158_d_n6, assign65490_e102158_d_n7, assign65490_e102158_d_n8, assign65490_e102158_d_n9, assign65490_e102158_d_n10, assign65490_e102158_d_n11, assign65490_e102158_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) {
        let assign65490_e102152: f64 = (locals.var_t2 / locals.var_wdep__blk1537);
        let assign65490_e102154: f64 = (assign65490_e102152 * locals.var_t2);
        let assign65490_e102156: f64 = (assign65490_e102154 / locals.var_wdep__blk1537);
        (assign65490_e102156, ((((((((locals.var_t2_dn0 * locals.var_wdep__blk1537) - (locals.var_t2 * locals.var_wdep__blk1537_dn0)) / (locals.var_wdep__blk1537 * locals.var_wdep__blk1537)) * locals.var_t2) + (assign65490_e102152 * locals.var_t2_dn0)) * locals.var_wdep__blk1537) - (assign65490_e102154 * locals.var_wdep__blk1537_dn0)) / (locals.var_wdep__blk1537 * locals.var_wdep__blk1537)), ((((((((locals.var_t2_dn2 * locals.var_wdep__blk1537) - (locals.var_t2 * locals.var_wdep__blk1537_dn2)) / (locals.var_wdep__blk1537 * locals.var_wdep__blk1537)) * locals.var_t2) + (assign65490_e102152 * locals.var_t2_dn2)) * locals.var_wdep__blk1537) - (assign65490_e102154 * locals.var_wdep__blk1537_dn2)) / (locals.var_wdep__blk1537 * locals.var_wdep__blk1537)), ((((((((locals.var_t2_dn4 * locals.var_wdep__blk1537) - (locals.var_t2 * locals.var_wdep__blk1537_dn4)) / (locals.var_wdep__blk1537 * locals.var_wdep__blk1537)) * locals.var_t2) + (assign65490_e102152 * locals.var_t2_dn4)) * locals.var_wdep__blk1537) - (assign65490_e102154 * locals.var_wdep__blk1537_dn4)) / (locals.var_wdep__blk1537 * locals.var_wdep__blk1537)), ((((((((locals.var_t2_dn5 * locals.var_wdep__blk1537) - (locals.var_t2 * locals.var_wdep__blk1537_dn5)) / (locals.var_wdep__blk1537 * locals.var_wdep__blk1537)) * locals.var_t2) + (assign65490_e102152 * locals.var_t2_dn5)) * locals.var_wdep__blk1537) - (assign65490_e102154 * locals.var_wdep__blk1537_dn5)) / (locals.var_wdep__blk1537 * locals.var_wdep__blk1537)), ((((((((locals.var_t2_dn6 * locals.var_wdep__blk1537) - (locals.var_t2 * locals.var_wdep__blk1537_dn6)) / (locals.var_wdep__blk1537 * locals.var_wdep__blk1537)) * locals.var_t2) + (assign65490_e102152 * locals.var_t2_dn6)) * locals.var_wdep__blk1537) - (assign65490_e102154 * locals.var_wdep__blk1537_dn6)) / (locals.var_wdep__blk1537 * locals.var_wdep__blk1537)), ((((((((locals.var_t2_dn7 * locals.var_wdep__blk1537) - (locals.var_t2 * locals.var_wdep__blk1537_dn7)) / (locals.var_wdep__blk1537 * locals.var_wdep__blk1537)) * locals.var_t2) + (assign65490_e102152 * locals.var_t2_dn7)) * locals.var_wdep__blk1537) - (assign65490_e102154 * locals.var_wdep__blk1537_dn7)) / (locals.var_wdep__blk1537 * locals.var_wdep__blk1537)), ((((((((locals.var_t2_dn8 * locals.var_wdep__blk1537) - (locals.var_t2 * locals.var_wdep__blk1537_dn8)) / (locals.var_wdep__blk1537 * locals.var_wdep__blk1537)) * locals.var_t2) + (assign65490_e102152 * locals.var_t2_dn8)) * locals.var_wdep__blk1537) - (assign65490_e102154 * locals.var_wdep__blk1537_dn8)) / (locals.var_wdep__blk1537 * locals.var_wdep__blk1537)), ((((((((locals.var_t2_dn9 * locals.var_wdep__blk1537) - (locals.var_t2 * locals.var_wdep__blk1537_dn9)) / (locals.var_wdep__blk1537 * locals.var_wdep__blk1537)) * locals.var_t2) + (assign65490_e102152 * locals.var_t2_dn9)) * locals.var_wdep__blk1537) - (assign65490_e102154 * locals.var_wdep__blk1537_dn9)) / (locals.var_wdep__blk1537 * locals.var_wdep__blk1537)), ((((((((locals.var_t2_dn10 * locals.var_wdep__blk1537) - (locals.var_t2 * locals.var_wdep__blk1537_dn10)) / (locals.var_wdep__blk1537 * locals.var_wdep__blk1537)) * locals.var_t2) + (assign65490_e102152 * locals.var_t2_dn10)) * locals.var_wdep__blk1537) - (assign65490_e102154 * locals.var_wdep__blk1537_dn10)) / (locals.var_wdep__blk1537 * locals.var_wdep__blk1537)), ((((((((locals.var_t2_dn11 * locals.var_wdep__blk1537) - (locals.var_t2 * locals.var_wdep__blk1537_dn11)) / (locals.var_wdep__blk1537 * locals.var_wdep__blk1537)) * locals.var_t2) + (assign65490_e102152 * locals.var_t2_dn11)) * locals.var_wdep__blk1537) - (assign65490_e102154 * locals.var_wdep__blk1537_dn11)) / (locals.var_wdep__blk1537 * locals.var_wdep__blk1537)), ((((((((locals.var_t2_dn14 * locals.var_wdep__blk1537) - (locals.var_t2 * locals.var_wdep__blk1537_dn14)) / (locals.var_wdep__blk1537 * locals.var_wdep__blk1537)) * locals.var_t2) + (assign65490_e102152 * locals.var_t2_dn14)) * locals.var_wdep__blk1537) - (assign65490_e102154 * locals.var_wdep__blk1537_dn14)) / (locals.var_wdep__blk1537 * locals.var_wdep__blk1537)),)
    } else {
        (locals.var_wfactor, locals.var_wfactor_dn0, locals.var_wfactor_dn2, locals.var_wfactor_dn4, locals.var_wfactor_dn5, locals.var_wfactor_dn6, locals.var_wfactor_dn7, locals.var_wfactor_dn8, locals.var_wfactor_dn9, locals.var_wfactor_dn10, locals.var_wfactor_dn11, locals.var_wfactor_dn14,)
    }
};
        locals.var_wfactor = assign65490_e102158;
        locals.var_wfactor_dn0 = assign65490_e102158_d_n0;
        locals.var_wfactor_dn2 = assign65490_e102158_d_n2;
        locals.var_wfactor_dn4 = assign65490_e102158_d_n4;
        locals.var_wfactor_dn5 = assign65490_e102158_d_n5;
        locals.var_wfactor_dn6 = assign65490_e102158_d_n6;
        locals.var_wfactor_dn7 = assign65490_e102158_d_n7;
        locals.var_wfactor_dn8 = assign65490_e102158_d_n8;
        locals.var_wfactor_dn9 = assign65490_e102158_d_n9;
        locals.var_wfactor_dn10 = assign65490_e102158_d_n10;
        locals.var_wfactor_dn11 = assign65490_e102158_d_n11;
        locals.var_wfactor_dn14 = assign65490_e102158_d_n14;
        locals.var_wfactor_rv = 0.0;

        let (assign65500_e102171, assign65500_e102171_d_n0, assign65500_e102171_d_n2, assign65500_e102171_d_n4, assign65500_e102171_d_n5, assign65500_e102171_d_n6, assign65500_e102171_d_n7, assign65500_e102171_d_n8, assign65500_e102171_d_n9, assign65500_e102171_d_n10, assign65500_e102171_d_n11, assign65500_e102171_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) {
        let assign65500_e102165: f64 = (locals.var_ps0__blk1527 - locals.var_dphi_vds);
        let assign65500_e102167: f64 = (assign65500_e102165 * locals.var_wfactor);
        let assign65500_e102169: f64 = (assign65500_e102167 + locals.var_dphi_vds);
        (assign65500_e102169, ((((locals.var_ps0__blk1527_dn0 - locals.var_dphi_vds_dn0) * locals.var_wfactor) + (assign65500_e102165 * locals.var_wfactor_dn0)) + locals.var_dphi_vds_dn0), ((((locals.var_ps0__blk1527_dn2 - locals.var_dphi_vds_dn2) * locals.var_wfactor) + (assign65500_e102165 * locals.var_wfactor_dn2)) + locals.var_dphi_vds_dn2), ((((locals.var_ps0__blk1527_dn4 - locals.var_dphi_vds_dn4) * locals.var_wfactor) + (assign65500_e102165 * locals.var_wfactor_dn4)) + locals.var_dphi_vds_dn4), ((((locals.var_ps0__blk1527_dn5 - locals.var_dphi_vds_dn5) * locals.var_wfactor) + (assign65500_e102165 * locals.var_wfactor_dn5)) + locals.var_dphi_vds_dn5), ((((locals.var_ps0__blk1527_dn6 - locals.var_dphi_vds_dn6) * locals.var_wfactor) + (assign65500_e102165 * locals.var_wfactor_dn6)) + locals.var_dphi_vds_dn6), ((((locals.var_ps0__blk1527_dn7 - locals.var_dphi_vds_dn7) * locals.var_wfactor) + (assign65500_e102165 * locals.var_wfactor_dn7)) + locals.var_dphi_vds_dn7), ((((locals.var_ps0__blk1527_dn8 - locals.var_dphi_vds_dn8) * locals.var_wfactor) + (assign65500_e102165 * locals.var_wfactor_dn8)) + locals.var_dphi_vds_dn8), ((((locals.var_ps0__blk1527_dn9 - locals.var_dphi_vds_dn9) * locals.var_wfactor) + (assign65500_e102165 * locals.var_wfactor_dn9)) + locals.var_dphi_vds_dn9), ((((locals.var_ps0__blk1527_dn10 - locals.var_dphi_vds_dn10) * locals.var_wfactor) + (assign65500_e102165 * locals.var_wfactor_dn10)) + locals.var_dphi_vds_dn10), ((((locals.var_ps0__blk1527_dn11 - locals.var_dphi_vds_dn11) * locals.var_wfactor) + (assign65500_e102165 * locals.var_wfactor_dn11)) + locals.var_dphi_vds_dn11), ((((locals.var_ps0__blk1527_dn14 - locals.var_dphi_vds_dn14) * locals.var_wfactor) + (assign65500_e102165 * locals.var_wfactor_dn14)) + locals.var_dphi_vds_dn14),)
    } else {
        (locals.var_phim, locals.var_phim_dn0, locals.var_phim_dn2, locals.var_phim_dn4, locals.var_phim_dn5, locals.var_phim_dn6, locals.var_phim_dn7, locals.var_phim_dn8, locals.var_phim_dn9, locals.var_phim_dn10, locals.var_phim_dn11, locals.var_phim_dn14,)
    }
};
        locals.var_phim = assign65500_e102171;
        locals.var_phim_dn0 = assign65500_e102171_d_n0;
        locals.var_phim_dn2 = assign65500_e102171_d_n2;
        locals.var_phim_dn4 = assign65500_e102171_d_n4;
        locals.var_phim_dn5 = assign65500_e102171_d_n5;
        locals.var_phim_dn6 = assign65500_e102171_d_n6;
        locals.var_phim_dn7 = assign65500_e102171_d_n7;
        locals.var_phim_dn8 = assign65500_e102171_d_n8;
        locals.var_phim_dn9 = assign65500_e102171_d_n9;
        locals.var_phim_dn10 = assign65500_e102171_d_n10;
        locals.var_phim_dn11 = assign65500_e102171_d_n11;
        locals.var_phim_dn14 = assign65500_e102171_d_n14;
        locals.var_phim_rv = 0.0;

        let (assign65510_e102193, assign65510_e102193_d_n0, assign65510_e102193_d_n2, assign65510_e102193_d_n4, assign65510_e102193_d_n5, assign65510_e102193_d_n6, assign65510_e102193_d_n7, assign65510_e102193_d_n8, assign65510_e102193_d_n9, assign65510_e102193_d_n10, assign65510_e102193_d_n11, assign65510_e102193_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) {
        let assign65510_e102180: f64 = (locals.var_vbipn - locals.var_vbscl__blk439);
        let assign65510_e102181: f64 = (locals.var_phim - assign65510_e102180);
        let assign65510_e102182: f64 = (locals.var_beta * assign65510_e102181);
        let assign65510_e102183: f64 = (assign65510_e102182).exp();
        let assign65510_e102186: f64 = (-locals.var_beta);
        let assign65510_e102188: f64 = (assign65510_e102186 * locals.var_vds);
        let assign65510_e102189: f64 = (assign65510_e102188).exp();
        let assign65510_e102190: f64 = (1.0 - assign65510_e102189);
        let assign65510_e102191: f64 = (assign65510_e102183 * assign65510_e102190);
        (assign65510_e102191, (((assign65510_e102183 * ((locals.var_beta_dn0 * assign65510_e102181) + (locals.var_beta * (locals.var_phim_dn0 - (locals.var_vbipn_dn0 - locals.var_vbscl__blk439_dn0))))) * assign65510_e102190) + (assign65510_e102183 * (-(assign65510_e102189 * (((-locals.var_beta_dn0) * locals.var_vds) + (assign65510_e102186 * locals.var_vds_dn0)))))), (((assign65510_e102183 * ((locals.var_beta_dn2 * assign65510_e102181) + (locals.var_beta * (locals.var_phim_dn2 - (locals.var_vbipn_dn2 - locals.var_vbscl__blk439_dn2))))) * assign65510_e102190) + (assign65510_e102183 * (-(assign65510_e102189 * (((-locals.var_beta_dn2) * locals.var_vds) + (assign65510_e102186 * locals.var_vds_dn2)))))), (((assign65510_e102183 * ((locals.var_beta_dn4 * assign65510_e102181) + (locals.var_beta * (locals.var_phim_dn4 - (locals.var_vbipn_dn4 - locals.var_vbscl__blk439_dn4))))) * assign65510_e102190) + (assign65510_e102183 * (-(assign65510_e102189 * (((-locals.var_beta_dn4) * locals.var_vds) + (assign65510_e102186 * locals.var_vds_dn4)))))), (((assign65510_e102183 * ((locals.var_beta_dn5 * assign65510_e102181) + (locals.var_beta * (locals.var_phim_dn5 - (locals.var_vbipn_dn5 - locals.var_vbscl__blk439_dn5))))) * assign65510_e102190) + (assign65510_e102183 * (-(assign65510_e102189 * (((-locals.var_beta_dn5) * locals.var_vds) + (assign65510_e102186 * locals.var_vds_dn5)))))), (((assign65510_e102183 * ((locals.var_beta_dn6 * assign65510_e102181) + (locals.var_beta * (locals.var_phim_dn6 - (locals.var_vbipn_dn6 - locals.var_vbscl__blk439_dn6))))) * assign65510_e102190) + (assign65510_e102183 * (-(assign65510_e102189 * (((-locals.var_beta_dn6) * locals.var_vds) + (assign65510_e102186 * locals.var_vds_dn6)))))), (((assign65510_e102183 * ((locals.var_beta_dn7 * assign65510_e102181) + (locals.var_beta * (locals.var_phim_dn7 - (locals.var_vbipn_dn7 - locals.var_vbscl__blk439_dn7))))) * assign65510_e102190) + (assign65510_e102183 * (-(assign65510_e102189 * (((-locals.var_beta_dn7) * locals.var_vds) + (assign65510_e102186 * locals.var_vds_dn7)))))), (((assign65510_e102183 * ((locals.var_beta_dn8 * assign65510_e102181) + (locals.var_beta * (locals.var_phim_dn8 - (locals.var_vbipn_dn8 - locals.var_vbscl__blk439_dn8))))) * assign65510_e102190) + (assign65510_e102183 * (-(assign65510_e102189 * (((-locals.var_beta_dn8) * locals.var_vds) + (assign65510_e102186 * locals.var_vds_dn8)))))), (((assign65510_e102183 * ((locals.var_beta_dn9 * assign65510_e102181) + (locals.var_beta * (locals.var_phim_dn9 - (locals.var_vbipn_dn9 - locals.var_vbscl__blk439_dn9))))) * assign65510_e102190) + (assign65510_e102183 * (-(assign65510_e102189 * (((-locals.var_beta_dn9) * locals.var_vds) + (assign65510_e102186 * locals.var_vds_dn9)))))), (((assign65510_e102183 * ((locals.var_beta_dn10 * assign65510_e102181) + (locals.var_beta * (locals.var_phim_dn10 - (locals.var_vbipn_dn10 - locals.var_vbscl__blk439_dn10))))) * assign65510_e102190) + (assign65510_e102183 * (-(assign65510_e102189 * (((-locals.var_beta_dn10) * locals.var_vds) + (assign65510_e102186 * locals.var_vds_dn10)))))), (((assign65510_e102183 * ((locals.var_beta_dn11 * assign65510_e102181) + (locals.var_beta * (locals.var_phim_dn11 - (locals.var_vbipn_dn11 - locals.var_vbscl__blk439_dn11))))) * assign65510_e102190) + (assign65510_e102183 * (-(assign65510_e102189 * (((-locals.var_beta_dn11) * locals.var_vds) + (assign65510_e102186 * locals.var_vds_dn11)))))), (((assign65510_e102183 * ((locals.var_beta_dn14 * assign65510_e102181) + (locals.var_beta * (locals.var_phim_dn14 - (locals.var_vbipn_dn14 - locals.var_vbscl__blk439_dn14))))) * assign65510_e102190) + (assign65510_e102183 * (-(assign65510_e102189 * (((-locals.var_beta_dn14) * locals.var_vds) + (assign65510_e102186 * locals.var_vds_dn14)))))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign65510_e102193;
        locals.var_ty_dn0 = assign65510_e102193_d_n0;
        locals.var_ty_dn2 = assign65510_e102193_d_n2;
        locals.var_ty_dn4 = assign65510_e102193_d_n4;
        locals.var_ty_dn5 = assign65510_e102193_d_n5;
        locals.var_ty_dn6 = assign65510_e102193_d_n6;
        locals.var_ty_dn7 = assign65510_e102193_d_n7;
        locals.var_ty_dn8 = assign65510_e102193_d_n8;
        locals.var_ty_dn9 = assign65510_e102193_d_n9;
        locals.var_ty_dn10 = assign65510_e102193_d_n10;
        locals.var_ty_dn11 = assign65510_e102193_d_n11;
        locals.var_ty_dn14 = assign65510_e102193_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign65520_e102207,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) {
        let assign65520_e102200: f64 = (2.0 * 1.6021918e-19);
        let assign65520_e102202: f64 = (assign65520_e102200 * locals.var_uc_njunc);
        let assign65520_e102204: f64 = (assign65520_e102202 * 1.034943e-10);
        let assign65520_e102205: f64 = (assign65520_e102204).sqrt();
        (assign65520_e102205,)
    } else {
        (locals.var_conpt00,)
    }
};
        locals.var_conpt00 = assign65520_e102207;
        locals.var_conpt00_rv = 0.0;

        let (assign65530_e102217, assign65530_e102217_d_n0, assign65530_e102217_d_n2, assign65530_e102217_d_n4, assign65530_e102217_d_n5, assign65530_e102217_d_n6, assign65530_e102217_d_n7, assign65530_e102217_d_n8, assign65530_e102217_d_n9, assign65530_e102217_d_n10, assign65530_e102217_d_n11, assign65530_e102217_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) {
        let assign65530_e102214: f64 = (locals.var_beta_inv).sqrt();
        let assign65530_e102215: f64 = (locals.var_conpt00 * assign65530_e102214);
        (assign65530_e102215, (locals.var_conpt00 * (locals.var_beta_inv_dn0 / (2.0 * assign65530_e102214))), (locals.var_conpt00 * (locals.var_beta_inv_dn2 / (2.0 * assign65530_e102214))), (locals.var_conpt00 * (locals.var_beta_inv_dn4 / (2.0 * assign65530_e102214))), (locals.var_conpt00 * (locals.var_beta_inv_dn5 / (2.0 * assign65530_e102214))), (locals.var_conpt00 * (locals.var_beta_inv_dn6 / (2.0 * assign65530_e102214))), (locals.var_conpt00 * (locals.var_beta_inv_dn7 / (2.0 * assign65530_e102214))), (locals.var_conpt00 * (locals.var_beta_inv_dn8 / (2.0 * assign65530_e102214))), (locals.var_conpt00 * (locals.var_beta_inv_dn9 / (2.0 * assign65530_e102214))), (locals.var_conpt00 * (locals.var_beta_inv_dn10 / (2.0 * assign65530_e102214))), (locals.var_conpt00 * (locals.var_beta_inv_dn11 / (2.0 * assign65530_e102214))), (locals.var_conpt00 * (locals.var_beta_inv_dn14 / (2.0 * assign65530_e102214))),)
    } else {
        (locals.var_conpt0, locals.var_conpt0_dn0, locals.var_conpt0_dn2, locals.var_conpt0_dn4, locals.var_conpt0_dn5, locals.var_conpt0_dn6, locals.var_conpt0_dn7, locals.var_conpt0_dn8, locals.var_conpt0_dn9, locals.var_conpt0_dn10, locals.var_conpt0_dn11, locals.var_conpt0_dn14,)
    }
};
        locals.var_conpt0 = assign65530_e102217;
        locals.var_conpt0_dn0 = assign65530_e102217_d_n0;
        locals.var_conpt0_dn2 = assign65530_e102217_d_n2;
        locals.var_conpt0_dn4 = assign65530_e102217_d_n4;
        locals.var_conpt0_dn5 = assign65530_e102217_d_n5;
        locals.var_conpt0_dn6 = assign65530_e102217_d_n6;
        locals.var_conpt0_dn7 = assign65530_e102217_d_n7;
        locals.var_conpt0_dn8 = assign65530_e102217_d_n8;
        locals.var_conpt0_dn9 = assign65530_e102217_d_n9;
        locals.var_conpt0_dn10 = assign65530_e102217_d_n10;
        locals.var_conpt0_dn11 = assign65530_e102217_d_n11;
        locals.var_conpt0_dn14 = assign65530_e102217_d_n14;
        locals.var_conpt0_rv = 0.0;

        let (assign65540_e102228, assign65540_e102228_d_n0, assign65540_e102228_d_n2, assign65540_e102228_d_n4, assign65540_e102228_d_n5, assign65540_e102228_d_n6, assign65540_e102228_d_n7, assign65540_e102228_d_n8, assign65540_e102228_d_n9, assign65540_e102228_d_n10, assign65540_e102228_d_n11, assign65540_e102228_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) {
        let assign65540_e102225: f64 = (locals.var_phim - locals.var_dphi_vds);
        let assign65540_e102226: f64 = (locals.var_beta * assign65540_e102225);
        (assign65540_e102226, ((locals.var_beta_dn0 * assign65540_e102225) + (locals.var_beta * (locals.var_phim_dn0 - locals.var_dphi_vds_dn0))), ((locals.var_beta_dn2 * assign65540_e102225) + (locals.var_beta * (locals.var_phim_dn2 - locals.var_dphi_vds_dn2))), ((locals.var_beta_dn4 * assign65540_e102225) + (locals.var_beta * (locals.var_phim_dn4 - locals.var_dphi_vds_dn4))), ((locals.var_beta_dn5 * assign65540_e102225) + (locals.var_beta * (locals.var_phim_dn5 - locals.var_dphi_vds_dn5))), ((locals.var_beta_dn6 * assign65540_e102225) + (locals.var_beta * (locals.var_phim_dn6 - locals.var_dphi_vds_dn6))), ((locals.var_beta_dn7 * assign65540_e102225) + (locals.var_beta * (locals.var_phim_dn7 - locals.var_dphi_vds_dn7))), ((locals.var_beta_dn8 * assign65540_e102225) + (locals.var_beta * (locals.var_phim_dn8 - locals.var_dphi_vds_dn8))), ((locals.var_beta_dn9 * assign65540_e102225) + (locals.var_beta * (locals.var_phim_dn9 - locals.var_dphi_vds_dn9))), ((locals.var_beta_dn10 * assign65540_e102225) + (locals.var_beta * (locals.var_phim_dn10 - locals.var_dphi_vds_dn10))), ((locals.var_beta_dn11 * assign65540_e102225) + (locals.var_beta * (locals.var_phim_dn11 - locals.var_dphi_vds_dn11))), ((locals.var_beta_dn14 * assign65540_e102225) + (locals.var_beta * (locals.var_phim_dn14 - locals.var_dphi_vds_dn14))),)
    } else {
        (locals.var_t1w, locals.var_t1w_dn0, locals.var_t1w_dn2, locals.var_t1w_dn4, locals.var_t1w_dn5, locals.var_t1w_dn6, locals.var_t1w_dn7, locals.var_t1w_dn8, locals.var_t1w_dn9, locals.var_t1w_dn10, locals.var_t1w_dn11, locals.var_t1w_dn14,)
    }
};
        locals.var_t1w = assign65540_e102228;
        locals.var_t1w_dn0 = assign65540_e102228_d_n0;
        locals.var_t1w_dn2 = assign65540_e102228_d_n2;
        locals.var_t1w_dn4 = assign65540_e102228_d_n4;
        locals.var_t1w_dn5 = assign65540_e102228_d_n5;
        locals.var_t1w_dn6 = assign65540_e102228_d_n6;
        locals.var_t1w_dn7 = assign65540_e102228_d_n7;
        locals.var_t1w_dn8 = assign65540_e102228_d_n8;
        locals.var_t1w_dn9 = assign65540_e102228_d_n9;
        locals.var_t1w_dn10 = assign65540_e102228_d_n10;
        locals.var_t1w_dn11 = assign65540_e102228_d_n11;
        locals.var_t1w_dn14 = assign65540_e102228_d_n14;
        locals.var_t1w_rv = 0.0;

        let assign65550_e102233: f64 = (0.2 * locals.var_beta);
        let assign65550_e102234: f64 = assign65550_e102233;
        let assign65550_e102238: f64 = (0.2 * locals.var_beta);
        let assign65550_e102241: f64 = if ((locals.var_t1w < assign65550_e102234) && (assign65550_e102238 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1572 = assign65550_e102241;
        locals.var_guard1572_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_247(
        locals: &mut StampLocals,
    ) {
        let (assign65560_e102256, assign65560_e102256_d_n0, assign65560_e102256_d_n2, assign65560_e102256_d_n4, assign65560_e102256_d_n5, assign65560_e102256_d_n6, assign65560_e102256_d_n7, assign65560_e102256_d_n8, assign65560_e102256_d_n9, assign65560_e102256_d_n10, assign65560_e102256_d_n11, assign65560_e102256_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1572 != 0.0)) {
        let assign65560_e102251: f64 = (0.2 * locals.var_beta);
        let assign65560_e102252: f64 = assign65560_e102251;
        let assign65560_e102254: f64 = (assign65560_e102252 - locals.var_t1w);
        (assign65560_e102254, ((0.2 * locals.var_beta_dn0) - locals.var_t1w_dn0), ((0.2 * locals.var_beta_dn2) - locals.var_t1w_dn2), ((0.2 * locals.var_beta_dn4) - locals.var_t1w_dn4), ((0.2 * locals.var_beta_dn5) - locals.var_t1w_dn5), ((0.2 * locals.var_beta_dn6) - locals.var_t1w_dn6), ((0.2 * locals.var_beta_dn7) - locals.var_t1w_dn7), ((0.2 * locals.var_beta_dn8) - locals.var_t1w_dn8), ((0.2 * locals.var_beta_dn9) - locals.var_t1w_dn9), ((0.2 * locals.var_beta_dn10) - locals.var_t1w_dn10), ((0.2 * locals.var_beta_dn11) - locals.var_t1w_dn11), ((0.2 * locals.var_beta_dn14) - locals.var_t1w_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign65560_e102256;
        locals.var_tmf1_dn0 = assign65560_e102256_d_n0;
        locals.var_tmf1_dn2 = assign65560_e102256_d_n2;
        locals.var_tmf1_dn4 = assign65560_e102256_d_n4;
        locals.var_tmf1_dn5 = assign65560_e102256_d_n5;
        locals.var_tmf1_dn6 = assign65560_e102256_d_n6;
        locals.var_tmf1_dn7 = assign65560_e102256_d_n7;
        locals.var_tmf1_dn8 = assign65560_e102256_d_n8;
        locals.var_tmf1_dn9 = assign65560_e102256_d_n9;
        locals.var_tmf1_dn10 = assign65560_e102256_d_n10;
        locals.var_tmf1_dn11 = assign65560_e102256_d_n11;
        locals.var_tmf1_dn14 = assign65560_e102256_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign65570_e102267, assign65570_e102267_d_n0, assign65570_e102267_d_n2, assign65570_e102267_d_n4, assign65570_e102267_d_n5, assign65570_e102267_d_n6, assign65570_e102267_d_n7, assign65570_e102267_d_n8, assign65570_e102267_d_n9, assign65570_e102267_d_n10, assign65570_e102267_d_n11, assign65570_e102267_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1572 != 0.0)) {
        let assign65570_e102265: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign65570_e102265, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign65570_e102267;
        locals.var_x2_dn0 = assign65570_e102267_d_n0;
        locals.var_x2_dn2 = assign65570_e102267_d_n2;
        locals.var_x2_dn4 = assign65570_e102267_d_n4;
        locals.var_x2_dn5 = assign65570_e102267_d_n5;
        locals.var_x2_dn6 = assign65570_e102267_d_n6;
        locals.var_x2_dn7 = assign65570_e102267_d_n7;
        locals.var_x2_dn8 = assign65570_e102267_d_n8;
        locals.var_x2_dn9 = assign65570_e102267_d_n9;
        locals.var_x2_dn10 = assign65570_e102267_d_n10;
        locals.var_x2_dn11 = assign65570_e102267_d_n11;
        locals.var_x2_dn14 = assign65570_e102267_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign65580_e102282, assign65580_e102282_d_n0, assign65580_e102282_d_n2, assign65580_e102282_d_n4, assign65580_e102282_d_n5, assign65580_e102282_d_n6, assign65580_e102282_d_n7, assign65580_e102282_d_n8, assign65580_e102282_d_n9, assign65580_e102282_d_n10, assign65580_e102282_d_n11, assign65580_e102282_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1572 != 0.0)) {
        let assign65580_e102276: f64 = (0.2 * locals.var_beta);
        let assign65580_e102279: f64 = (0.2 * locals.var_beta);
        let assign65580_e102280: f64 = (assign65580_e102276 * assign65580_e102279);
        (assign65580_e102280, (((0.2 * locals.var_beta_dn0) * assign65580_e102279) + (assign65580_e102276 * (0.2 * locals.var_beta_dn0))), (((0.2 * locals.var_beta_dn2) * assign65580_e102279) + (assign65580_e102276 * (0.2 * locals.var_beta_dn2))), (((0.2 * locals.var_beta_dn4) * assign65580_e102279) + (assign65580_e102276 * (0.2 * locals.var_beta_dn4))), (((0.2 * locals.var_beta_dn5) * assign65580_e102279) + (assign65580_e102276 * (0.2 * locals.var_beta_dn5))), (((0.2 * locals.var_beta_dn6) * assign65580_e102279) + (assign65580_e102276 * (0.2 * locals.var_beta_dn6))), (((0.2 * locals.var_beta_dn7) * assign65580_e102279) + (assign65580_e102276 * (0.2 * locals.var_beta_dn7))), (((0.2 * locals.var_beta_dn8) * assign65580_e102279) + (assign65580_e102276 * (0.2 * locals.var_beta_dn8))), (((0.2 * locals.var_beta_dn9) * assign65580_e102279) + (assign65580_e102276 * (0.2 * locals.var_beta_dn9))), (((0.2 * locals.var_beta_dn10) * assign65580_e102279) + (assign65580_e102276 * (0.2 * locals.var_beta_dn10))), (((0.2 * locals.var_beta_dn11) * assign65580_e102279) + (assign65580_e102276 * (0.2 * locals.var_beta_dn11))), (((0.2 * locals.var_beta_dn14) * assign65580_e102279) + (assign65580_e102276 * (0.2 * locals.var_beta_dn14))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign65580_e102282;
        locals.var_xmax2_dn0 = assign65580_e102282_d_n0;
        locals.var_xmax2_dn2 = assign65580_e102282_d_n2;
        locals.var_xmax2_dn4 = assign65580_e102282_d_n4;
        locals.var_xmax2_dn5 = assign65580_e102282_d_n5;
        locals.var_xmax2_dn6 = assign65580_e102282_d_n6;
        locals.var_xmax2_dn7 = assign65580_e102282_d_n7;
        locals.var_xmax2_dn8 = assign65580_e102282_d_n8;
        locals.var_xmax2_dn9 = assign65580_e102282_d_n9;
        locals.var_xmax2_dn10 = assign65580_e102282_d_n10;
        locals.var_xmax2_dn11 = assign65580_e102282_d_n11;
        locals.var_xmax2_dn14 = assign65580_e102282_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign65590_e102291, assign65590_e102291_d_n0, assign65590_e102291_d_n2, assign65590_e102291_d_n4, assign65590_e102291_d_n5, assign65590_e102291_d_n6, assign65590_e102291_d_n7, assign65590_e102291_d_n8, assign65590_e102291_d_n9, assign65590_e102291_d_n10, assign65590_e102291_d_n11, assign65590_e102291_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1572 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign65590_e102291;
        locals.var_xp_dn0 = assign65590_e102291_d_n0;
        locals.var_xp_dn2 = assign65590_e102291_d_n2;
        locals.var_xp_dn4 = assign65590_e102291_d_n4;
        locals.var_xp_dn5 = assign65590_e102291_d_n5;
        locals.var_xp_dn6 = assign65590_e102291_d_n6;
        locals.var_xp_dn7 = assign65590_e102291_d_n7;
        locals.var_xp_dn8 = assign65590_e102291_d_n8;
        locals.var_xp_dn9 = assign65590_e102291_d_n9;
        locals.var_xp_dn10 = assign65590_e102291_d_n10;
        locals.var_xp_dn11 = assign65590_e102291_d_n11;
        locals.var_xp_dn14 = assign65590_e102291_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign65600_e102300, assign65600_e102300_d_n0, assign65600_e102300_d_n2, assign65600_e102300_d_n4, assign65600_e102300_d_n5, assign65600_e102300_d_n6, assign65600_e102300_d_n7, assign65600_e102300_d_n8, assign65600_e102300_d_n9, assign65600_e102300_d_n10, assign65600_e102300_d_n11, assign65600_e102300_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1572 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign65600_e102300;
        locals.var_xmp_dn0 = assign65600_e102300_d_n0;
        locals.var_xmp_dn2 = assign65600_e102300_d_n2;
        locals.var_xmp_dn4 = assign65600_e102300_d_n4;
        locals.var_xmp_dn5 = assign65600_e102300_d_n5;
        locals.var_xmp_dn6 = assign65600_e102300_d_n6;
        locals.var_xmp_dn7 = assign65600_e102300_d_n7;
        locals.var_xmp_dn8 = assign65600_e102300_d_n8;
        locals.var_xmp_dn9 = assign65600_e102300_d_n9;
        locals.var_xmp_dn10 = assign65600_e102300_d_n10;
        locals.var_xmp_dn11 = assign65600_e102300_d_n11;
        locals.var_xmp_dn14 = assign65600_e102300_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign65610_e102309,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1572 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign65610_e102309;
        locals.var_m0_rv = 0.0;

        let (assign65620_e102318,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1572 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign65620_e102318;
        locals.var_mm_rv = 0.0;

        let (assign65630_e102327, assign65630_e102327_d_n0, assign65630_e102327_d_n2, assign65630_e102327_d_n4, assign65630_e102327_d_n5, assign65630_e102327_d_n6, assign65630_e102327_d_n7, assign65630_e102327_d_n8, assign65630_e102327_d_n9, assign65630_e102327_d_n10, assign65630_e102327_d_n11, assign65630_e102327_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1572 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign65630_e102327;
        locals.var_arg_dn0 = assign65630_e102327_d_n0;
        locals.var_arg_dn2 = assign65630_e102327_d_n2;
        locals.var_arg_dn4 = assign65630_e102327_d_n4;
        locals.var_arg_dn5 = assign65630_e102327_d_n5;
        locals.var_arg_dn6 = assign65630_e102327_d_n6;
        locals.var_arg_dn7 = assign65630_e102327_d_n7;
        locals.var_arg_dn8 = assign65630_e102327_d_n8;
        locals.var_arg_dn9 = assign65630_e102327_d_n9;
        locals.var_arg_dn10 = assign65630_e102327_d_n10;
        locals.var_arg_dn11 = assign65630_e102327_d_n11;
        locals.var_arg_dn14 = assign65630_e102327_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign65640_e102336, assign65640_e102336_d_n0, assign65640_e102336_d_n2, assign65640_e102336_d_n4, assign65640_e102336_d_n5, assign65640_e102336_d_n6, assign65640_e102336_d_n7, assign65640_e102336_d_n8, assign65640_e102336_d_n9, assign65640_e102336_d_n10, assign65640_e102336_d_n11, assign65640_e102336_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1572 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign65640_e102336;
        locals.var_dnm_dn0 = assign65640_e102336_d_n0;
        locals.var_dnm_dn2 = assign65640_e102336_d_n2;
        locals.var_dnm_dn4 = assign65640_e102336_d_n4;
        locals.var_dnm_dn5 = assign65640_e102336_d_n5;
        locals.var_dnm_dn6 = assign65640_e102336_d_n6;
        locals.var_dnm_dn7 = assign65640_e102336_d_n7;
        locals.var_dnm_dn8 = assign65640_e102336_d_n8;
        locals.var_dnm_dn9 = assign65640_e102336_d_n9;
        locals.var_dnm_dn10 = assign65640_e102336_d_n10;
        locals.var_dnm_dn11 = assign65640_e102336_d_n11;
        locals.var_dnm_dn14 = assign65640_e102336_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign65650_e102347, assign65650_e102347_d_n0, assign65650_e102347_d_n2, assign65650_e102347_d_n4, assign65650_e102347_d_n5, assign65650_e102347_d_n6, assign65650_e102347_d_n7, assign65650_e102347_d_n8, assign65650_e102347_d_n9, assign65650_e102347_d_n10, assign65650_e102347_d_n11, assign65650_e102347_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1572 != 0.0)) {
        let assign65650_e102345: f64 = (locals.var_xp * locals.var_x2);
        (assign65650_e102345, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign65650_e102347;
        locals.var_xp_dn0 = assign65650_e102347_d_n0;
        locals.var_xp_dn2 = assign65650_e102347_d_n2;
        locals.var_xp_dn4 = assign65650_e102347_d_n4;
        locals.var_xp_dn5 = assign65650_e102347_d_n5;
        locals.var_xp_dn6 = assign65650_e102347_d_n6;
        locals.var_xp_dn7 = assign65650_e102347_d_n7;
        locals.var_xp_dn8 = assign65650_e102347_d_n8;
        locals.var_xp_dn9 = assign65650_e102347_d_n9;
        locals.var_xp_dn10 = assign65650_e102347_d_n10;
        locals.var_xp_dn11 = assign65650_e102347_d_n11;
        locals.var_xp_dn14 = assign65650_e102347_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign65660_e102358, assign65660_e102358_d_n0, assign65660_e102358_d_n2, assign65660_e102358_d_n4, assign65660_e102358_d_n5, assign65660_e102358_d_n6, assign65660_e102358_d_n7, assign65660_e102358_d_n8, assign65660_e102358_d_n9, assign65660_e102358_d_n10, assign65660_e102358_d_n11, assign65660_e102358_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1572 != 0.0)) {
        let assign65660_e102356: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign65660_e102356, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign65660_e102358;
        locals.var_xmp_dn0 = assign65660_e102358_d_n0;
        locals.var_xmp_dn2 = assign65660_e102358_d_n2;
        locals.var_xmp_dn4 = assign65660_e102358_d_n4;
        locals.var_xmp_dn5 = assign65660_e102358_d_n5;
        locals.var_xmp_dn6 = assign65660_e102358_d_n6;
        locals.var_xmp_dn7 = assign65660_e102358_d_n7;
        locals.var_xmp_dn8 = assign65660_e102358_d_n8;
        locals.var_xmp_dn9 = assign65660_e102358_d_n9;
        locals.var_xmp_dn10 = assign65660_e102358_d_n10;
        locals.var_xmp_dn11 = assign65660_e102358_d_n11;
        locals.var_xmp_dn14 = assign65660_e102358_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign65670_e102369, assign65670_e102369_d_n0, assign65670_e102369_d_n2, assign65670_e102369_d_n4, assign65670_e102369_d_n5, assign65670_e102369_d_n6, assign65670_e102369_d_n7, assign65670_e102369_d_n8, assign65670_e102369_d_n9, assign65670_e102369_d_n10, assign65670_e102369_d_n11, assign65670_e102369_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1572 != 0.0)) {
        let assign65670_e102367: f64 = (locals.var_xp + locals.var_xmp);
        (assign65670_e102367, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign65670_e102369;
        locals.var_arg_dn0 = assign65670_e102369_d_n0;
        locals.var_arg_dn2 = assign65670_e102369_d_n2;
        locals.var_arg_dn4 = assign65670_e102369_d_n4;
        locals.var_arg_dn5 = assign65670_e102369_d_n5;
        locals.var_arg_dn6 = assign65670_e102369_d_n6;
        locals.var_arg_dn7 = assign65670_e102369_d_n7;
        locals.var_arg_dn8 = assign65670_e102369_d_n8;
        locals.var_arg_dn9 = assign65670_e102369_d_n9;
        locals.var_arg_dn10 = assign65670_e102369_d_n10;
        locals.var_arg_dn11 = assign65670_e102369_d_n11;
        locals.var_arg_dn14 = assign65670_e102369_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign65680_e102378, assign65680_e102378_d_n0, assign65680_e102378_d_n2, assign65680_e102378_d_n4, assign65680_e102378_d_n5, assign65680_e102378_d_n6, assign65680_e102378_d_n7, assign65680_e102378_d_n8, assign65680_e102378_d_n9, assign65680_e102378_d_n10, assign65680_e102378_d_n11, assign65680_e102378_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1572 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign65680_e102378;
        locals.var_dnm_dn0 = assign65680_e102378_d_n0;
        locals.var_dnm_dn2 = assign65680_e102378_d_n2;
        locals.var_dnm_dn4 = assign65680_e102378_d_n4;
        locals.var_dnm_dn5 = assign65680_e102378_d_n5;
        locals.var_dnm_dn6 = assign65680_e102378_d_n6;
        locals.var_dnm_dn7 = assign65680_e102378_d_n7;
        locals.var_dnm_dn8 = assign65680_e102378_d_n8;
        locals.var_dnm_dn9 = assign65680_e102378_d_n9;
        locals.var_dnm_dn10 = assign65680_e102378_d_n10;
        locals.var_dnm_dn11 = assign65680_e102378_d_n11;
        locals.var_dnm_dn14 = assign65680_e102378_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign65690_e102393: f64 = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1573 = assign65690_e102393;
        locals.var_guard1573_rv = 0.0;

        let assign65700_e102396: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1574 = assign65700_e102396;
        locals.var_guard1574_rv = 0.0;

        let (assign65710_e102409,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1572 != 0.0)) && (locals.var_guard1573 != 0.0)) && (locals.var_guard1574 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign65710_e102409;
        locals.var_mm_rv = 0.0;

        let assign65720_e102412: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1575 = assign65720_e102412;
        locals.var_guard1575_rv = 0.0;

        let (assign65730_e102428,) = {
    if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1572 != 0.0)) && (locals.var_guard1573 != 0.0)) && (locals.var_guard1574 == 0.0)) && (locals.var_guard1575 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign65730_e102428;
        locals.var_mm_rv = 0.0;

        let assign65740_e102431: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1576 = assign65740_e102431;
        locals.var_guard1576_rv = 0.0;

        let (assign65750_e102450,) = {
    if (((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1572 != 0.0)) && (locals.var_guard1573 != 0.0)) && (locals.var_guard1574 == 0.0)) && (locals.var_guard1575 == 0.0)) && (locals.var_guard1576 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign65750_e102450;
        locals.var_mm_rv = 0.0;

        let assign65760_e102453: f64 = if 1.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1577 = assign65760_e102453;
        locals.var_guard1577_rv = 0.0;

        let (assign65770_e102475,) = {
    if ((((((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1572 != 0.0)) && (locals.var_guard1573 != 0.0)) && (locals.var_guard1574 == 0.0)) && (locals.var_guard1575 == 0.0)) && (locals.var_guard1576 == 0.0)) && (locals.var_guard1577 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign65770_e102475;
        locals.var_mm_rv = 0.0;

        let (assign65780_e102486,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1572 != 0.0)) && (locals.var_guard1573 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign65780_e102486;
        locals.var_m0_rv = 0.0;

        let mut assign65790_loop_guard: usize = 0;
        while {
            let assign65790_cond_e102498: f64 = if (((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1572 != 0.0)) && (locals.var_guard1573 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign65790_cond_e102498 != 0.0
        } {
            assign65790_loop_guard += 1;
            assert!(assign65790_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign65790_body0_e102510, assign65790_body0_e102510_d_n0, assign65790_body0_e102510_d_n2, assign65790_body0_e102510_d_n4, assign65790_body0_e102510_d_n5, assign65790_body0_e102510_d_n6, assign65790_body0_e102510_d_n7, assign65790_body0_e102510_d_n8, assign65790_body0_e102510_d_n9, assign65790_body0_e102510_d_n10, assign65790_body0_e102510_d_n11, assign65790_body0_e102510_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1572 != 0.0)) && (locals.var_guard1573 != 0.0)) {
        let assign65790_body0_e102508: f64 = (locals.var_dnm).sqrt();
        (assign65790_body0_e102508, (locals.var_dnm_dn0 / (2.0 * assign65790_body0_e102508)), (locals.var_dnm_dn2 / (2.0 * assign65790_body0_e102508)), (locals.var_dnm_dn4 / (2.0 * assign65790_body0_e102508)), (locals.var_dnm_dn5 / (2.0 * assign65790_body0_e102508)), (locals.var_dnm_dn6 / (2.0 * assign65790_body0_e102508)), (locals.var_dnm_dn7 / (2.0 * assign65790_body0_e102508)), (locals.var_dnm_dn8 / (2.0 * assign65790_body0_e102508)), (locals.var_dnm_dn9 / (2.0 * assign65790_body0_e102508)), (locals.var_dnm_dn10 / (2.0 * assign65790_body0_e102508)), (locals.var_dnm_dn11 / (2.0 * assign65790_body0_e102508)), (locals.var_dnm_dn14 / (2.0 * assign65790_body0_e102508)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign65790_body0_e102510;
            locals.var_dnm_dn0 = assign65790_body0_e102510_d_n0;
            locals.var_dnm_dn2 = assign65790_body0_e102510_d_n2;
            locals.var_dnm_dn4 = assign65790_body0_e102510_d_n4;
            locals.var_dnm_dn5 = assign65790_body0_e102510_d_n5;
            locals.var_dnm_dn6 = assign65790_body0_e102510_d_n6;
            locals.var_dnm_dn7 = assign65790_body0_e102510_d_n7;
            locals.var_dnm_dn8 = assign65790_body0_e102510_d_n8;
            locals.var_dnm_dn9 = assign65790_body0_e102510_d_n9;
            locals.var_dnm_dn10 = assign65790_body0_e102510_d_n10;
            locals.var_dnm_dn11 = assign65790_body0_e102510_d_n11;
            locals.var_dnm_dn14 = assign65790_body0_e102510_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign65790_body1_e102523,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1572 != 0.0)) && (locals.var_guard1573 != 0.0)) {
        let assign65790_body1_e102521: f64 = (locals.var_m0 + 1.0);
        (assign65790_body1_e102521,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign65790_body1_e102523;
            locals.var_m0_rv = 0.0;
        }

        let (assign65800_e102546, assign65800_e102546_d_n0, assign65800_e102546_d_n2, assign65800_e102546_d_n4, assign65800_e102546_d_n5, assign65800_e102546_d_n6, assign65800_e102546_d_n7, assign65800_e102546_d_n8, assign65800_e102546_d_n9, assign65800_e102546_d_n10, assign65800_e102546_d_n11, assign65800_e102546_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1572 != 0.0)) && (locals.var_guard1573 == 0.0)) {
        let (assign65800_e102544, assign65800_e102544_d_n0, assign65800_e102544_d_n2, assign65800_e102544_d_n4, assign65800_e102544_d_n5, assign65800_e102544_d_n6, assign65800_e102544_d_n7, assign65800_e102544_d_n8, assign65800_e102544_d_n9, assign65800_e102544_d_n10, assign65800_e102544_d_n11, assign65800_e102544_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign65800_e102541: f64 = 2.0;
                let assign65800_e102542: f64 = (1.0 / assign65800_e102541);
                let assign65800_e102543: f64 = (locals.var_dnm).powf(assign65800_e102542);
                (assign65800_e102543, if 0.0 == 0.0 && ((assign65800_e102542) as f64).is_finite() && ((assign65800_e102542) as f64).fract() == 0.0 { if assign65800_e102542 == 0.0 { 0.0 } else { (assign65800_e102542 * ((locals.var_dnm).powf(assign65800_e102542 - 1.0) * locals.var_dnm_dn0)) } } else { (assign65800_e102543 * (assign65800_e102542 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65800_e102542) as f64).is_finite() && ((assign65800_e102542) as f64).fract() == 0.0 { if assign65800_e102542 == 0.0 { 0.0 } else { (assign65800_e102542 * ((locals.var_dnm).powf(assign65800_e102542 - 1.0) * locals.var_dnm_dn2)) } } else { (assign65800_e102543 * (assign65800_e102542 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65800_e102542) as f64).is_finite() && ((assign65800_e102542) as f64).fract() == 0.0 { if assign65800_e102542 == 0.0 { 0.0 } else { (assign65800_e102542 * ((locals.var_dnm).powf(assign65800_e102542 - 1.0) * locals.var_dnm_dn4)) } } else { (assign65800_e102543 * (assign65800_e102542 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65800_e102542) as f64).is_finite() && ((assign65800_e102542) as f64).fract() == 0.0 { if assign65800_e102542 == 0.0 { 0.0 } else { (assign65800_e102542 * ((locals.var_dnm).powf(assign65800_e102542 - 1.0) * locals.var_dnm_dn5)) } } else { (assign65800_e102543 * (assign65800_e102542 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65800_e102542) as f64).is_finite() && ((assign65800_e102542) as f64).fract() == 0.0 { if assign65800_e102542 == 0.0 { 0.0 } else { (assign65800_e102542 * ((locals.var_dnm).powf(assign65800_e102542 - 1.0) * locals.var_dnm_dn6)) } } else { (assign65800_e102543 * (assign65800_e102542 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65800_e102542) as f64).is_finite() && ((assign65800_e102542) as f64).fract() == 0.0 { if assign65800_e102542 == 0.0 { 0.0 } else { (assign65800_e102542 * ((locals.var_dnm).powf(assign65800_e102542 - 1.0) * locals.var_dnm_dn7)) } } else { (assign65800_e102543 * (assign65800_e102542 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65800_e102542) as f64).is_finite() && ((assign65800_e102542) as f64).fract() == 0.0 { if assign65800_e102542 == 0.0 { 0.0 } else { (assign65800_e102542 * ((locals.var_dnm).powf(assign65800_e102542 - 1.0) * locals.var_dnm_dn8)) } } else { (assign65800_e102543 * (assign65800_e102542 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65800_e102542) as f64).is_finite() && ((assign65800_e102542) as f64).fract() == 0.0 { if assign65800_e102542 == 0.0 { 0.0 } else { (assign65800_e102542 * ((locals.var_dnm).powf(assign65800_e102542 - 1.0) * locals.var_dnm_dn9)) } } else { (assign65800_e102543 * (assign65800_e102542 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65800_e102542) as f64).is_finite() && ((assign65800_e102542) as f64).fract() == 0.0 { if assign65800_e102542 == 0.0 { 0.0 } else { (assign65800_e102542 * ((locals.var_dnm).powf(assign65800_e102542 - 1.0) * locals.var_dnm_dn10)) } } else { (assign65800_e102543 * (assign65800_e102542 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65800_e102542) as f64).is_finite() && ((assign65800_e102542) as f64).fract() == 0.0 { if assign65800_e102542 == 0.0 { 0.0 } else { (assign65800_e102542 * ((locals.var_dnm).powf(assign65800_e102542 - 1.0) * locals.var_dnm_dn11)) } } else { (assign65800_e102543 * (assign65800_e102542 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65800_e102542) as f64).is_finite() && ((assign65800_e102542) as f64).fract() == 0.0 { if assign65800_e102542 == 0.0 { 0.0 } else { (assign65800_e102542 * ((locals.var_dnm).powf(assign65800_e102542 - 1.0) * locals.var_dnm_dn14)) } } else { (assign65800_e102543 * (assign65800_e102542 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign65800_e102544, assign65800_e102544_d_n0, assign65800_e102544_d_n2, assign65800_e102544_d_n4, assign65800_e102544_d_n5, assign65800_e102544_d_n6, assign65800_e102544_d_n7, assign65800_e102544_d_n8, assign65800_e102544_d_n9, assign65800_e102544_d_n10, assign65800_e102544_d_n11, assign65800_e102544_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign65800_e102546;
        locals.var_dnm_dn0 = assign65800_e102546_d_n0;
        locals.var_dnm_dn2 = assign65800_e102546_d_n2;
        locals.var_dnm_dn4 = assign65800_e102546_d_n4;
        locals.var_dnm_dn5 = assign65800_e102546_d_n5;
        locals.var_dnm_dn6 = assign65800_e102546_d_n6;
        locals.var_dnm_dn7 = assign65800_e102546_d_n7;
        locals.var_dnm_dn8 = assign65800_e102546_d_n8;
        locals.var_dnm_dn9 = assign65800_e102546_d_n9;
        locals.var_dnm_dn10 = assign65800_e102546_d_n10;
        locals.var_dnm_dn11 = assign65800_e102546_d_n11;
        locals.var_dnm_dn14 = assign65800_e102546_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign65810_e102557, assign65810_e102557_d_n0, assign65810_e102557_d_n2, assign65810_e102557_d_n4, assign65810_e102557_d_n5, assign65810_e102557_d_n6, assign65810_e102557_d_n7, assign65810_e102557_d_n8, assign65810_e102557_d_n9, assign65810_e102557_d_n10, assign65810_e102557_d_n11, assign65810_e102557_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1572 != 0.0)) {
        let assign65810_e102555: f64 = (1.0 / locals.var_dnm);
        (assign65810_e102555, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign65810_e102557;
        locals.var_dnm_dn0 = assign65810_e102557_d_n0;
        locals.var_dnm_dn2 = assign65810_e102557_d_n2;
        locals.var_dnm_dn4 = assign65810_e102557_d_n4;
        locals.var_dnm_dn5 = assign65810_e102557_d_n5;
        locals.var_dnm_dn6 = assign65810_e102557_d_n6;
        locals.var_dnm_dn7 = assign65810_e102557_d_n7;
        locals.var_dnm_dn8 = assign65810_e102557_d_n8;
        locals.var_dnm_dn9 = assign65810_e102557_d_n9;
        locals.var_dnm_dn10 = assign65810_e102557_d_n10;
        locals.var_dnm_dn11 = assign65810_e102557_d_n11;
        locals.var_dnm_dn14 = assign65810_e102557_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign65820_e102572, assign65820_e102572_d_n0, assign65820_e102572_d_n2, assign65820_e102572_d_n4, assign65820_e102572_d_n5, assign65820_e102572_d_n6, assign65820_e102572_d_n7, assign65820_e102572_d_n8, assign65820_e102572_d_n9, assign65820_e102572_d_n10, assign65820_e102572_d_n11, assign65820_e102572_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1572 != 0.0)) {
        let assign65820_e102567: f64 = (0.2 * locals.var_beta);
        let assign65820_e102568: f64 = (locals.var_tmf1 * assign65820_e102567);
        let assign65820_e102570: f64 = (assign65820_e102568 * locals.var_dnm);
        (assign65820_e102570, ((((locals.var_tmf1_dn0 * assign65820_e102567) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn0))) * locals.var_dnm) + (assign65820_e102568 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign65820_e102567) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn2))) * locals.var_dnm) + (assign65820_e102568 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign65820_e102567) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn4))) * locals.var_dnm) + (assign65820_e102568 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign65820_e102567) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn5))) * locals.var_dnm) + (assign65820_e102568 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign65820_e102567) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn6))) * locals.var_dnm) + (assign65820_e102568 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign65820_e102567) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn7))) * locals.var_dnm) + (assign65820_e102568 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign65820_e102567) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn8))) * locals.var_dnm) + (assign65820_e102568 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign65820_e102567) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn9))) * locals.var_dnm) + (assign65820_e102568 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign65820_e102567) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn10))) * locals.var_dnm) + (assign65820_e102568 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * assign65820_e102567) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn11))) * locals.var_dnm) + (assign65820_e102568 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * assign65820_e102567) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn14))) * locals.var_dnm) + (assign65820_e102568 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign65820_e102572;
        locals.var_tmf0_dn0 = assign65820_e102572_d_n0;
        locals.var_tmf0_dn2 = assign65820_e102572_d_n2;
        locals.var_tmf0_dn4 = assign65820_e102572_d_n4;
        locals.var_tmf0_dn5 = assign65820_e102572_d_n5;
        locals.var_tmf0_dn6 = assign65820_e102572_d_n6;
        locals.var_tmf0_dn7 = assign65820_e102572_d_n7;
        locals.var_tmf0_dn8 = assign65820_e102572_d_n8;
        locals.var_tmf0_dn9 = assign65820_e102572_d_n9;
        locals.var_tmf0_dn10 = assign65820_e102572_d_n10;
        locals.var_tmf0_dn11 = assign65820_e102572_d_n11;
        locals.var_tmf0_dn14 = assign65820_e102572_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign65830_e102589, assign65830_e102589_d_n0, assign65830_e102589_d_n2, assign65830_e102589_d_n4, assign65830_e102589_d_n5, assign65830_e102589_d_n6, assign65830_e102589_d_n7, assign65830_e102589_d_n8, assign65830_e102589_d_n9, assign65830_e102589_d_n10, assign65830_e102589_d_n11, assign65830_e102589_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1572 != 0.0)) {
        let assign65830_e102581: f64 = (0.2 * locals.var_beta);
        let assign65830_e102583: f64 = (assign65830_e102581 * locals.var_xmp);
        let assign65830_e102585: f64 = (assign65830_e102583 * locals.var_dnm);
        let assign65830_e102587: f64 = (assign65830_e102585 / locals.var_arg);
        (assign65830_e102587, ((((((((0.2 * locals.var_beta_dn0) * locals.var_xmp) + (assign65830_e102581 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign65830_e102583 * locals.var_dnm_dn0)) * locals.var_arg) - (assign65830_e102585 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn2) * locals.var_xmp) + (assign65830_e102581 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign65830_e102583 * locals.var_dnm_dn2)) * locals.var_arg) - (assign65830_e102585 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn4) * locals.var_xmp) + (assign65830_e102581 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign65830_e102583 * locals.var_dnm_dn4)) * locals.var_arg) - (assign65830_e102585 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn5) * locals.var_xmp) + (assign65830_e102581 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign65830_e102583 * locals.var_dnm_dn5)) * locals.var_arg) - (assign65830_e102585 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn6) * locals.var_xmp) + (assign65830_e102581 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign65830_e102583 * locals.var_dnm_dn6)) * locals.var_arg) - (assign65830_e102585 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn7) * locals.var_xmp) + (assign65830_e102581 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign65830_e102583 * locals.var_dnm_dn7)) * locals.var_arg) - (assign65830_e102585 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn8) * locals.var_xmp) + (assign65830_e102581 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign65830_e102583 * locals.var_dnm_dn8)) * locals.var_arg) - (assign65830_e102585 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn9) * locals.var_xmp) + (assign65830_e102581 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign65830_e102583 * locals.var_dnm_dn9)) * locals.var_arg) - (assign65830_e102585 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn10) * locals.var_xmp) + (assign65830_e102581 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign65830_e102583 * locals.var_dnm_dn10)) * locals.var_arg) - (assign65830_e102585 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn11) * locals.var_xmp) + (assign65830_e102581 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign65830_e102583 * locals.var_dnm_dn11)) * locals.var_arg) - (assign65830_e102585 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn14) * locals.var_xmp) + (assign65830_e102581 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign65830_e102583 * locals.var_dnm_dn14)) * locals.var_arg) - (assign65830_e102585 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign65830_e102589;
        locals.var_t0_dn0 = assign65830_e102589_d_n0;
        locals.var_t0_dn2 = assign65830_e102589_d_n2;
        locals.var_t0_dn4 = assign65830_e102589_d_n4;
        locals.var_t0_dn5 = assign65830_e102589_d_n5;
        locals.var_t0_dn6 = assign65830_e102589_d_n6;
        locals.var_t0_dn7 = assign65830_e102589_d_n7;
        locals.var_t0_dn8 = assign65830_e102589_d_n8;
        locals.var_t0_dn9 = assign65830_e102589_d_n9;
        locals.var_t0_dn10 = assign65830_e102589_d_n10;
        locals.var_t0_dn11 = assign65830_e102589_d_n11;
        locals.var_t0_dn14 = assign65830_e102589_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign65840_e102604, assign65840_e102604_d_n0, assign65840_e102604_d_n2, assign65840_e102604_d_n4, assign65840_e102604_d_n5, assign65840_e102604_d_n6, assign65840_e102604_d_n7, assign65840_e102604_d_n8, assign65840_e102604_d_n9, assign65840_e102604_d_n10, assign65840_e102604_d_n11, assign65840_e102604_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1523 != 0.0)) && (locals.var_guard1572 != 0.0)) {
        let assign65840_e102599: f64 = (0.2 * locals.var_beta);
        let assign65840_e102600: f64 = assign65840_e102599;
        let assign65840_e102602: f64 = (assign65840_e102600 - locals.var_tmf0);
        (assign65840_e102602, ((0.2 * locals.var_beta_dn0) - locals.var_tmf0_dn0), ((0.2 * locals.var_beta_dn2) - locals.var_tmf0_dn2), ((0.2 * locals.var_beta_dn4) - locals.var_tmf0_dn4), ((0.2 * locals.var_beta_dn5) - locals.var_tmf0_dn5), ((0.2 * locals.var_beta_dn6) - locals.var_tmf0_dn6), ((0.2 * locals.var_beta_dn7) - locals.var_tmf0_dn7), ((0.2 * locals.var_beta_dn8) - locals.var_tmf0_dn8), ((0.2 * locals.var_beta_dn9) - locals.var_tmf0_dn9), ((0.2 * locals.var_beta_dn10) - locals.var_tmf0_dn10), ((0.2 * locals.var_beta_dn11) - locals.var_tmf0_dn11), ((0.2 * locals.var_beta_dn14) - locals.var_tmf0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign65840_e102604;
        locals.var_t1_dn0 = assign65840_e102604_d_n0;
        locals.var_t1_dn2 = assign65840_e102604_d_n2;
        locals.var_t1_dn4 = assign65840_e102604_d_n4;
        locals.var_t1_dn5 = assign65840_e102604_d_n5;
        locals.var_t1_dn6 = assign65840_e102604_d_n6;
        locals.var_t1_dn7 = assign65840_e102604_d_n7;
        locals.var_t1_dn8 = assign65840_e102604_d_n8;
        locals.var_t1_dn9 = assign65840_e102604_d_n9;
        locals.var_t1_dn10 = assign65840_e102604_d_n10;
        locals.var_t1_dn11 = assign65840_e102604_d_n11;
        locals.var_t1_dn14 = assign65840_e102604_d_n14;
        locals.var_t1_rv = 0.0;

    }
}

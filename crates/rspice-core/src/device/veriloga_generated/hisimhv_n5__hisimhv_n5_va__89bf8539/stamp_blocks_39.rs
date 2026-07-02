#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_233(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign62460_e96989, assign62460_e96989_d_n0, assign62460_e96989_d_n2, assign62460_e96989_d_n4, assign62460_e96989_d_n5, assign62460_e96989_d_n6, assign62460_e96989_d_n7, assign62460_e96989_d_n8, assign62460_e96989_d_n9, assign62460_e96989_d_n10, assign62460_e96989_d_n11, assign62460_e96989_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) && (locals.var_guard1498 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign62460_e96989;
        locals.var_t2_dn0 = assign62460_e96989_d_n0;
        locals.var_t2_dn2 = assign62460_e96989_d_n2;
        locals.var_t2_dn4 = assign62460_e96989_d_n4;
        locals.var_t2_dn5 = assign62460_e96989_d_n5;
        locals.var_t2_dn6 = assign62460_e96989_d_n6;
        locals.var_t2_dn7 = assign62460_e96989_d_n7;
        locals.var_t2_dn8 = assign62460_e96989_d_n8;
        locals.var_t2_dn9 = assign62460_e96989_d_n9;
        locals.var_t2_dn10 = assign62460_e96989_d_n10;
        locals.var_t2_dn11 = assign62460_e96989_d_n11;
        locals.var_t2_dn14 = assign62460_e96989_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign62470_e97000, assign62470_e97000_d_n0, assign62470_e97000_d_n2, assign62470_e97000_d_n4, assign62470_e97000_d_n5, assign62470_e97000_d_n6, assign62470_e97000_d_n7, assign62470_e97000_d_n8, assign62470_e97000_d_n9, assign62470_e97000_d_n10, assign62470_e97000_d_n11, assign62470_e97000_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) && (locals.var_guard1498 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign62470_e97000;
        locals.var_t0_dn0 = assign62470_e97000_d_n0;
        locals.var_t0_dn2 = assign62470_e97000_d_n2;
        locals.var_t0_dn4 = assign62470_e97000_d_n4;
        locals.var_t0_dn5 = assign62470_e97000_d_n5;
        locals.var_t0_dn6 = assign62470_e97000_d_n6;
        locals.var_t0_dn7 = assign62470_e97000_d_n7;
        locals.var_t0_dn8 = assign62470_e97000_d_n8;
        locals.var_t0_dn9 = assign62470_e97000_d_n9;
        locals.var_t0_dn10 = assign62470_e97000_d_n10;
        locals.var_t0_dn11 = assign62470_e97000_d_n11;
        locals.var_t0_dn14 = assign62470_e97000_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign62480_e97011, assign62480_e97011_d_n0, assign62480_e97011_d_n2, assign62480_e97011_d_n4, assign62480_e97011_d_n5, assign62480_e97011_d_n6, assign62480_e97011_d_n7, assign62480_e97011_d_n8, assign62480_e97011_d_n9, assign62480_e97011_d_n10, assign62480_e97011_d_n11, assign62480_e97011_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62480_e97009: f64 = (locals.var_t2 + 1e-25);
        (assign62480_e97009, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign62480_e97011;
        locals.var_t2_dn0 = assign62480_e97011_d_n0;
        locals.var_t2_dn2 = assign62480_e97011_d_n2;
        locals.var_t2_dn4 = assign62480_e97011_d_n4;
        locals.var_t2_dn5 = assign62480_e97011_d_n5;
        locals.var_t2_dn6 = assign62480_e97011_d_n6;
        locals.var_t2_dn7 = assign62480_e97011_d_n7;
        locals.var_t2_dn8 = assign62480_e97011_d_n8;
        locals.var_t2_dn9 = assign62480_e97011_d_n9;
        locals.var_t2_dn10 = assign62480_e97011_d_n10;
        locals.var_t2_dn11 = assign62480_e97011_d_n11;
        locals.var_t2_dn14 = assign62480_e97011_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign62490_e97022, assign62490_e97022_d_n0, assign62490_e97022_d_n2, assign62490_e97022_d_n4, assign62490_e97022_d_n5, assign62490_e97022_d_n6, assign62490_e97022_d_n7, assign62490_e97022_d_n8, assign62490_e97022_d_n9, assign62490_e97022_d_n10, assign62490_e97022_d_n11, assign62490_e97022_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62490_e97020: f64 = (locals.var_beta * locals.var_ptl0);
        (assign62490_e97020, (locals.var_beta_dn0 * locals.var_ptl0), (locals.var_beta_dn2 * locals.var_ptl0), (locals.var_beta_dn4 * locals.var_ptl0), (locals.var_beta_dn5 * locals.var_ptl0), (locals.var_beta_dn6 * locals.var_ptl0), (locals.var_beta_dn7 * locals.var_ptl0), (locals.var_beta_dn8 * locals.var_ptl0), (locals.var_beta_dn9 * locals.var_ptl0), (locals.var_beta_dn10 * locals.var_ptl0), (locals.var_beta_dn11 * locals.var_ptl0), (locals.var_beta_dn14 * locals.var_ptl0),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign62490_e97022;
        locals.var_t0_dn0 = assign62490_e97022_d_n0;
        locals.var_t0_dn2 = assign62490_e97022_d_n2;
        locals.var_t0_dn4 = assign62490_e97022_d_n4;
        locals.var_t0_dn5 = assign62490_e97022_d_n5;
        locals.var_t0_dn6 = assign62490_e97022_d_n6;
        locals.var_t0_dn7 = assign62490_e97022_d_n7;
        locals.var_t0_dn8 = assign62490_e97022_d_n8;
        locals.var_t0_dn9 = assign62490_e97022_d_n9;
        locals.var_t0_dn10 = assign62490_e97022_d_n10;
        locals.var_t0_dn11 = assign62490_e97022_d_n11;
        locals.var_t0_dn14 = assign62490_e97022_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign62500_e97033, assign62500_e97033_d_n0, assign62500_e97033_d_n2, assign62500_e97033_d_n4, assign62500_e97033_d_n5, assign62500_e97033_d_n6, assign62500_e97033_d_n7, assign62500_e97033_d_n8, assign62500_e97033_d_n9, assign62500_e97033_d_n10, assign62500_e97033_d_n11, assign62500_e97033_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62500_e97031: f64 = (locals.var_cox * locals.var_t0);
        (assign62500_e97031, ((locals.var_cox_dn0 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn0)), ((locals.var_cox_dn2 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn2)), ((locals.var_cox_dn4 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn4)), ((locals.var_cox_dn5 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn5)), ((locals.var_cox_dn6 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn6)), ((locals.var_cox_dn7 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn7)), ((locals.var_cox_dn8 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn8)), ((locals.var_cox_dn9 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn9)), ((locals.var_cox_dn10 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn10)), ((locals.var_cox_dn11 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn11)), ((locals.var_cox_dn14 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign62500_e97033;
        locals.var_t3_dn0 = assign62500_e97033_d_n0;
        locals.var_t3_dn2 = assign62500_e97033_d_n2;
        locals.var_t3_dn4 = assign62500_e97033_d_n4;
        locals.var_t3_dn5 = assign62500_e97033_d_n5;
        locals.var_t3_dn6 = assign62500_e97033_d_n6;
        locals.var_t3_dn7 = assign62500_e97033_d_n7;
        locals.var_t3_dn8 = assign62500_e97033_d_n8;
        locals.var_t3_dn9 = assign62500_e97033_d_n9;
        locals.var_t3_dn10 = assign62500_e97033_d_n10;
        locals.var_t3_dn11 = assign62500_e97033_d_n11;
        locals.var_t3_dn14 = assign62500_e97033_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign62510_e97044, assign62510_e97044_d_n0, assign62510_e97044_d_n2, assign62510_e97044_d_n4, assign62510_e97044_d_n5, assign62510_e97044_d_n6, assign62510_e97044_d_n7, assign62510_e97044_d_n8, assign62510_e97044_d_n9, assign62510_e97044_d_n10, assign62510_e97044_d_n11, assign62510_e97044_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62510_e97042: f64 = (locals.var_t2).powf(p.p284);
        (assign62510_e97042, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn0)) } } else { (assign62510_e97042 * (p.p284 * (locals.var_t2_dn0 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn2)) } } else { (assign62510_e97042 * (p.p284 * (locals.var_t2_dn2 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn4)) } } else { (assign62510_e97042 * (p.p284 * (locals.var_t2_dn4 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn5)) } } else { (assign62510_e97042 * (p.p284 * (locals.var_t2_dn5 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn6)) } } else { (assign62510_e97042 * (p.p284 * (locals.var_t2_dn6 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn7)) } } else { (assign62510_e97042 * (p.p284 * (locals.var_t2_dn7 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn8)) } } else { (assign62510_e97042 * (p.p284 * (locals.var_t2_dn8 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn9)) } } else { (assign62510_e97042 * (p.p284 * (locals.var_t2_dn9 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn10)) } } else { (assign62510_e97042 * (p.p284 * (locals.var_t2_dn10 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn11)) } } else { (assign62510_e97042 * (p.p284 * (locals.var_t2_dn11 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn14)) } } else { (assign62510_e97042 * (p.p284 * (locals.var_t2_dn14 / locals.var_t2))) },)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign62510_e97044;
        locals.var_t0_dn0 = assign62510_e97044_d_n0;
        locals.var_t0_dn2 = assign62510_e97044_d_n2;
        locals.var_t0_dn4 = assign62510_e97044_d_n4;
        locals.var_t0_dn5 = assign62510_e97044_d_n5;
        locals.var_t0_dn6 = assign62510_e97044_d_n6;
        locals.var_t0_dn7 = assign62510_e97044_d_n7;
        locals.var_t0_dn8 = assign62510_e97044_d_n8;
        locals.var_t0_dn9 = assign62510_e97044_d_n9;
        locals.var_t0_dn10 = assign62510_e97044_d_n10;
        locals.var_t0_dn11 = assign62510_e97044_d_n11;
        locals.var_t0_dn14 = assign62510_e97044_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign62520_e97055, assign62520_e97055_d_n0, assign62520_e97055_d_n2, assign62520_e97055_d_n4, assign62520_e97055_d_n5, assign62520_e97055_d_n6, assign62520_e97055_d_n7, assign62520_e97055_d_n8, assign62520_e97055_d_n9, assign62520_e97055_d_n10, assign62520_e97055_d_n11, assign62520_e97055_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62520_e97053: f64 = (locals.var_t3 * locals.var_t0);
        (assign62520_e97053, ((locals.var_t3_dn0 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn0)), ((locals.var_t3_dn2 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn2)), ((locals.var_t3_dn4 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn4)), ((locals.var_t3_dn5 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn5)), ((locals.var_t3_dn6 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn6)), ((locals.var_t3_dn7 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn7)), ((locals.var_t3_dn8 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn8)), ((locals.var_t3_dn9 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn9)), ((locals.var_t3_dn10 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn10)), ((locals.var_t3_dn11 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn11)), ((locals.var_t3_dn14 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign62520_e97055;
        locals.var_t9_dn0 = assign62520_e97055_d_n0;
        locals.var_t9_dn2 = assign62520_e97055_d_n2;
        locals.var_t9_dn4 = assign62520_e97055_d_n4;
        locals.var_t9_dn5 = assign62520_e97055_d_n5;
        locals.var_t9_dn6 = assign62520_e97055_d_n6;
        locals.var_t9_dn7 = assign62520_e97055_d_n7;
        locals.var_t9_dn8 = assign62520_e97055_d_n8;
        locals.var_t9_dn9 = assign62520_e97055_d_n9;
        locals.var_t9_dn10 = assign62520_e97055_d_n10;
        locals.var_t9_dn11 = assign62520_e97055_d_n11;
        locals.var_t9_dn14 = assign62520_e97055_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign62530_e97068, assign62530_e97068_d_n0, assign62530_e97068_d_n2, assign62530_e97068_d_n4, assign62530_e97068_d_n5, assign62530_e97068_d_n6, assign62530_e97068_d_n7, assign62530_e97068_d_n8, assign62530_e97068_d_n9, assign62530_e97068_d_n10, assign62530_e97068_d_n11, assign62530_e97068_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62530_e97065: f64 = (locals.var_vdsz__blk441 * p.p285);
        let assign62530_e97066: f64 = (1.0 + assign62530_e97065);
        (assign62530_e97066, (locals.var_vdsz__blk441_dn0 * p.p285), (locals.var_vdsz__blk441_dn2 * p.p285), (locals.var_vdsz__blk441_dn4 * p.p285), (locals.var_vdsz__blk441_dn5 * p.p285), (locals.var_vdsz__blk441_dn6 * p.p285), (locals.var_vdsz__blk441_dn7 * p.p285), (locals.var_vdsz__blk441_dn8 * p.p285), (locals.var_vdsz__blk441_dn9 * p.p285), (locals.var_vdsz__blk441_dn10 * p.p285), (locals.var_vdsz__blk441_dn11 * p.p285), (locals.var_vdsz__blk441_dn14 * p.p285),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign62530_e97068;
        locals.var_t4_dn0 = assign62530_e97068_d_n0;
        locals.var_t4_dn2 = assign62530_e97068_d_n2;
        locals.var_t4_dn4 = assign62530_e97068_d_n4;
        locals.var_t4_dn5 = assign62530_e97068_d_n5;
        locals.var_t4_dn6 = assign62530_e97068_d_n6;
        locals.var_t4_dn7 = assign62530_e97068_d_n7;
        locals.var_t4_dn8 = assign62530_e97068_d_n8;
        locals.var_t4_dn9 = assign62530_e97068_d_n9;
        locals.var_t4_dn10 = assign62530_e97068_d_n10;
        locals.var_t4_dn11 = assign62530_e97068_d_n11;
        locals.var_t4_dn14 = assign62530_e97068_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign62540_e97077, assign62540_e97077_d_n0, assign62540_e97077_d_n2, assign62540_e97077_d_n4, assign62540_e97077_d_n5, assign62540_e97077_d_n6, assign62540_e97077_d_n7, assign62540_e97077_d_n8, assign62540_e97077_d_n9, assign62540_e97077_d_n10, assign62540_e97077_d_n11, assign62540_e97077_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        (locals.var_pt40, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign62540_e97077;
        locals.var_t0_dn0 = assign62540_e97077_d_n0;
        locals.var_t0_dn2 = assign62540_e97077_d_n2;
        locals.var_t0_dn4 = assign62540_e97077_d_n4;
        locals.var_t0_dn5 = assign62540_e97077_d_n5;
        locals.var_t0_dn6 = assign62540_e97077_d_n6;
        locals.var_t0_dn7 = assign62540_e97077_d_n7;
        locals.var_t0_dn8 = assign62540_e97077_d_n8;
        locals.var_t0_dn9 = assign62540_e97077_d_n9;
        locals.var_t0_dn10 = assign62540_e97077_d_n10;
        locals.var_t0_dn11 = assign62540_e97077_d_n11;
        locals.var_t0_dn14 = assign62540_e97077_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign62550_e97090, assign62550_e97090_d_n0, assign62550_e97090_d_n2, assign62550_e97090_d_n4, assign62550_e97090_d_n5, assign62550_e97090_d_n6, assign62550_e97090_d_n7, assign62550_e97090_d_n8, assign62550_e97090_d_n9, assign62550_e97090_d_n10, assign62550_e97090_d_n11, assign62550_e97090_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62550_e97086: f64 = (locals.var_ps0 + locals.var_t6);
        let assign62550_e97088: f64 = (assign62550_e97086 - locals.var_vbsz__blk440);
        (assign62550_e97088, ((locals.var_ps0_dn0 + locals.var_t6_dn0) - locals.var_vbsz__blk440_dn0), ((locals.var_ps0_dn2 + locals.var_t6_dn2) - locals.var_vbsz__blk440_dn2), ((locals.var_ps0_dn4 + locals.var_t6_dn4) - locals.var_vbsz__blk440_dn4), ((locals.var_ps0_dn5 + locals.var_t6_dn5) - locals.var_vbsz__blk440_dn5), ((locals.var_ps0_dn6 + locals.var_t6_dn6) - locals.var_vbsz__blk440_dn6), ((locals.var_ps0_dn7 + locals.var_t6_dn7) - locals.var_vbsz__blk440_dn7), ((locals.var_ps0_dn8 + locals.var_t6_dn8) - locals.var_vbsz__blk440_dn8), ((locals.var_ps0_dn9 + locals.var_t6_dn9) - locals.var_vbsz__blk440_dn9), ((locals.var_ps0_dn10 + locals.var_t6_dn10) - locals.var_vbsz__blk440_dn10), ((locals.var_ps0_dn11 + locals.var_t6_dn11) - locals.var_vbsz__blk440_dn11), ((locals.var_ps0_dn14 + locals.var_t6_dn14) - locals.var_vbsz__blk440_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign62550_e97090;
        locals.var_t5_dn0 = assign62550_e97090_d_n0;
        locals.var_t5_dn2 = assign62550_e97090_d_n2;
        locals.var_t5_dn4 = assign62550_e97090_d_n4;
        locals.var_t5_dn5 = assign62550_e97090_d_n5;
        locals.var_t5_dn6 = assign62550_e97090_d_n6;
        locals.var_t5_dn7 = assign62550_e97090_d_n7;
        locals.var_t5_dn8 = assign62550_e97090_d_n8;
        locals.var_t5_dn9 = assign62550_e97090_d_n9;
        locals.var_t5_dn10 = assign62550_e97090_d_n10;
        locals.var_t5_dn11 = assign62550_e97090_d_n11;
        locals.var_t5_dn14 = assign62550_e97090_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign62560_e97105, assign62560_e97105_d_n0, assign62560_e97105_d_n2, assign62560_e97105_d_n4, assign62560_e97105_d_n5, assign62560_e97105_d_n6, assign62560_e97105_d_n7, assign62560_e97105_d_n8, assign62560_e97105_d_n9, assign62560_e97105_d_n10, assign62560_e97105_d_n11, assign62560_e97105_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62560_e97100: f64 = (locals.var_vdsz__blk441 * locals.var_t0);
        let assign62560_e97102: f64 = (assign62560_e97100 * locals.var_t5);
        let assign62560_e97103: f64 = (locals.var_t4 + assign62560_e97102);
        (assign62560_e97103, (locals.var_t4_dn0 + ((((locals.var_vdsz__blk441_dn0 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn0)) * locals.var_t5) + (assign62560_e97100 * locals.var_t5_dn0))), (locals.var_t4_dn2 + ((((locals.var_vdsz__blk441_dn2 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn2)) * locals.var_t5) + (assign62560_e97100 * locals.var_t5_dn2))), (locals.var_t4_dn4 + ((((locals.var_vdsz__blk441_dn4 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn4)) * locals.var_t5) + (assign62560_e97100 * locals.var_t5_dn4))), (locals.var_t4_dn5 + ((((locals.var_vdsz__blk441_dn5 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn5)) * locals.var_t5) + (assign62560_e97100 * locals.var_t5_dn5))), (locals.var_t4_dn6 + ((((locals.var_vdsz__blk441_dn6 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn6)) * locals.var_t5) + (assign62560_e97100 * locals.var_t5_dn6))), (locals.var_t4_dn7 + ((((locals.var_vdsz__blk441_dn7 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn7)) * locals.var_t5) + (assign62560_e97100 * locals.var_t5_dn7))), (locals.var_t4_dn8 + ((((locals.var_vdsz__blk441_dn8 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn8)) * locals.var_t5) + (assign62560_e97100 * locals.var_t5_dn8))), (locals.var_t4_dn9 + ((((locals.var_vdsz__blk441_dn9 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn9)) * locals.var_t5) + (assign62560_e97100 * locals.var_t5_dn9))), (locals.var_t4_dn10 + ((((locals.var_vdsz__blk441_dn10 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn10)) * locals.var_t5) + (assign62560_e97100 * locals.var_t5_dn10))), (locals.var_t4_dn11 + ((((locals.var_vdsz__blk441_dn11 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn11)) * locals.var_t5) + (assign62560_e97100 * locals.var_t5_dn11))), (locals.var_t4_dn14 + ((((locals.var_vdsz__blk441_dn14 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn14)) * locals.var_t5) + (assign62560_e97100 * locals.var_t5_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign62560_e97105;
        locals.var_t4_dn0 = assign62560_e97105_d_n0;
        locals.var_t4_dn2 = assign62560_e97105_d_n2;
        locals.var_t4_dn4 = assign62560_e97105_d_n4;
        locals.var_t4_dn5 = assign62560_e97105_d_n5;
        locals.var_t4_dn6 = assign62560_e97105_d_n6;
        locals.var_t4_dn7 = assign62560_e97105_d_n7;
        locals.var_t4_dn8 = assign62560_e97105_d_n8;
        locals.var_t4_dn9 = assign62560_e97105_d_n9;
        locals.var_t4_dn10 = assign62560_e97105_d_n10;
        locals.var_t4_dn11 = assign62560_e97105_d_n11;
        locals.var_t4_dn14 = assign62560_e97105_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign62570_e97116, assign62570_e97116_d_n0, assign62570_e97116_d_n2, assign62570_e97116_d_n4, assign62570_e97116_d_n5, assign62570_e97116_d_n6, assign62570_e97116_d_n7, assign62570_e97116_d_n8, assign62570_e97116_d_n9, assign62570_e97116_d_n10, assign62570_e97116_d_n11, assign62570_e97116_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62570_e97114: f64 = (locals.var_t9 * locals.var_t4);
        (assign62570_e97114, ((locals.var_t9_dn0 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn0)), ((locals.var_t9_dn2 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn2)), ((locals.var_t9_dn4 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn4)), ((locals.var_t9_dn5 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn5)), ((locals.var_t9_dn6 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn6)), ((locals.var_t9_dn7 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn7)), ((locals.var_t9_dn8 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn8)), ((locals.var_t9_dn9 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn9)), ((locals.var_t9_dn10 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn10)), ((locals.var_t9_dn11 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn11)), ((locals.var_t9_dn14 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign62570_e97116;
        locals.var_t6_dn0 = assign62570_e97116_d_n0;
        locals.var_t6_dn2 = assign62570_e97116_d_n2;
        locals.var_t6_dn4 = assign62570_e97116_d_n4;
        locals.var_t6_dn5 = assign62570_e97116_d_n5;
        locals.var_t6_dn6 = assign62570_e97116_d_n6;
        locals.var_t6_dn7 = assign62570_e97116_d_n7;
        locals.var_t6_dn8 = assign62570_e97116_d_n8;
        locals.var_t6_dn9 = assign62570_e97116_d_n9;
        locals.var_t6_dn10 = assign62570_e97116_d_n10;
        locals.var_t6_dn11 = assign62570_e97116_d_n11;
        locals.var_t6_dn14 = assign62570_e97116_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign62580_e97125, assign62580_e97125_d_n0, assign62580_e97125_d_n2, assign62580_e97125_d_n4, assign62580_e97125_d_n5, assign62580_e97125_d_n6, assign62580_e97125_d_n7, assign62580_e97125_d_n8, assign62580_e97125_d_n9, assign62580_e97125_d_n10, assign62580_e97125_d_n11, assign62580_e97125_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign62580_e97125;
        locals.var_t9_dn0 = assign62580_e97125_d_n0;
        locals.var_t9_dn2 = assign62580_e97125_d_n2;
        locals.var_t9_dn4 = assign62580_e97125_d_n4;
        locals.var_t9_dn5 = assign62580_e97125_d_n5;
        locals.var_t9_dn6 = assign62580_e97125_d_n6;
        locals.var_t9_dn7 = assign62580_e97125_d_n7;
        locals.var_t9_dn8 = assign62580_e97125_d_n8;
        locals.var_t9_dn9 = assign62580_e97125_d_n9;
        locals.var_t9_dn10 = assign62580_e97125_d_n10;
        locals.var_t9_dn11 = assign62580_e97125_d_n11;
        locals.var_t9_dn14 = assign62580_e97125_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign62590_e97135, assign62590_e97135_d_n0, assign62590_e97135_d_n2, assign62590_e97135_d_n4, assign62590_e97135_d_n5, assign62590_e97135_d_n6, assign62590_e97135_d_n7, assign62590_e97135_d_n8, assign62590_e97135_d_n9, assign62590_e97135_d_n10, assign62590_e97135_d_n11, assign62590_e97135_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1497 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign62590_e97135;
        locals.var_t9_dn0 = assign62590_e97135_d_n0;
        locals.var_t9_dn2 = assign62590_e97135_d_n2;
        locals.var_t9_dn4 = assign62590_e97135_d_n4;
        locals.var_t9_dn5 = assign62590_e97135_d_n5;
        locals.var_t9_dn6 = assign62590_e97135_d_n6;
        locals.var_t9_dn7 = assign62590_e97135_d_n7;
        locals.var_t9_dn8 = assign62590_e97135_d_n8;
        locals.var_t9_dn9 = assign62590_e97135_d_n9;
        locals.var_t9_dn10 = assign62590_e97135_d_n10;
        locals.var_t9_dn11 = assign62590_e97135_d_n11;
        locals.var_t9_dn14 = assign62590_e97135_d_n14;
        locals.var_t9_rv = 0.0;

        let assign62600_e97138: f64 = if p.p287 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1499 = assign62600_e97138;
        locals.var_guard1499_rv = 0.0;

        let (assign62610_e97149, assign62610_e97149_d_n0, assign62610_e97149_d_n2, assign62610_e97149_d_n4, assign62610_e97149_d_n5, assign62610_e97149_d_n6, assign62610_e97149_d_n7, assign62610_e97149_d_n8, assign62610_e97149_d_n9, assign62610_e97149_d_n10, assign62610_e97149_d_n11, assign62610_e97149_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62610_e97147: f64 = (locals.var_beta * locals.var_gdl0);
        (assign62610_e97147, (locals.var_beta_dn0 * locals.var_gdl0), (locals.var_beta_dn2 * locals.var_gdl0), (locals.var_beta_dn4 * locals.var_gdl0), (locals.var_beta_dn5 * locals.var_gdl0), (locals.var_beta_dn6 * locals.var_gdl0), (locals.var_beta_dn7 * locals.var_gdl0), (locals.var_beta_dn8 * locals.var_gdl0), (locals.var_beta_dn9 * locals.var_gdl0), (locals.var_beta_dn10 * locals.var_gdl0), (locals.var_beta_dn11 * locals.var_gdl0), (locals.var_beta_dn14 * locals.var_gdl0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign62610_e97149;
        locals.var_t1_dn0 = assign62610_e97149_d_n0;
        locals.var_t1_dn2 = assign62610_e97149_d_n2;
        locals.var_t1_dn4 = assign62610_e97149_d_n4;
        locals.var_t1_dn5 = assign62610_e97149_d_n5;
        locals.var_t1_dn6 = assign62610_e97149_d_n6;
        locals.var_t1_dn7 = assign62610_e97149_d_n7;
        locals.var_t1_dn8 = assign62610_e97149_d_n8;
        locals.var_t1_dn9 = assign62610_e97149_d_n9;
        locals.var_t1_dn10 = assign62610_e97149_d_n10;
        locals.var_t1_dn11 = assign62610_e97149_d_n11;
        locals.var_t1_dn14 = assign62610_e97149_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign62620_e97160, assign62620_e97160_d_n0, assign62620_e97160_d_n2, assign62620_e97160_d_n4, assign62620_e97160_d_n5, assign62620_e97160_d_n6, assign62620_e97160_d_n7, assign62620_e97160_d_n8, assign62620_e97160_d_n9, assign62620_e97160_d_n10, assign62620_e97160_d_n11, assign62620_e97160_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62620_e97158: f64 = (locals.var_cox * locals.var_t1);
        (assign62620_e97158, ((locals.var_cox_dn0 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn0)), ((locals.var_cox_dn2 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn2)), ((locals.var_cox_dn4 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn4)), ((locals.var_cox_dn5 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn5)), ((locals.var_cox_dn6 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn6)), ((locals.var_cox_dn7 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn7)), ((locals.var_cox_dn8 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn8)), ((locals.var_cox_dn9 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn9)), ((locals.var_cox_dn10 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn10)), ((locals.var_cox_dn11 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn11)), ((locals.var_cox_dn14 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign62620_e97160;
        locals.var_t2_dn0 = assign62620_e97160_d_n0;
        locals.var_t2_dn2 = assign62620_e97160_d_n2;
        locals.var_t2_dn4 = assign62620_e97160_d_n4;
        locals.var_t2_dn5 = assign62620_e97160_d_n5;
        locals.var_t2_dn6 = assign62620_e97160_d_n6;
        locals.var_t2_dn7 = assign62620_e97160_d_n7;
        locals.var_t2_dn8 = assign62620_e97160_d_n8;
        locals.var_t2_dn9 = assign62620_e97160_d_n9;
        locals.var_t2_dn10 = assign62620_e97160_d_n10;
        locals.var_t2_dn11 = assign62620_e97160_d_n11;
        locals.var_t2_dn14 = assign62620_e97160_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign62630_e97171, assign62630_e97171_d_n0, assign62630_e97171_d_n2, assign62630_e97171_d_n4, assign62630_e97171_d_n5, assign62630_e97171_d_n6, assign62630_e97171_d_n7, assign62630_e97171_d_n8, assign62630_e97171_d_n9, assign62630_e97171_d_n10, assign62630_e97171_d_n11, assign62630_e97171_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62630_e97169: f64 = (locals.var_t2 * locals.var_vdsz__blk441);
        (assign62630_e97169, ((locals.var_t2_dn0 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn0)), ((locals.var_t2_dn2 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn2)), ((locals.var_t2_dn4 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn4)), ((locals.var_t2_dn5 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn5)), ((locals.var_t2_dn6 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn6)), ((locals.var_t2_dn7 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn7)), ((locals.var_t2_dn8 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn8)), ((locals.var_t2_dn9 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn9)), ((locals.var_t2_dn10 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn10)), ((locals.var_t2_dn11 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn11)), ((locals.var_t2_dn14 * locals.var_vdsz__blk441) + (locals.var_t2 * locals.var_vdsz__blk441_dn14)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign62630_e97171;
        locals.var_t8_dn0 = assign62630_e97171_d_n0;
        locals.var_t8_dn2 = assign62630_e97171_d_n2;
        locals.var_t8_dn4 = assign62630_e97171_d_n4;
        locals.var_t8_dn5 = assign62630_e97171_d_n5;
        locals.var_t8_dn6 = assign62630_e97171_d_n6;
        locals.var_t8_dn7 = assign62630_e97171_d_n7;
        locals.var_t8_dn8 = assign62630_e97171_d_n8;
        locals.var_t8_dn9 = assign62630_e97171_d_n9;
        locals.var_t8_dn10 = assign62630_e97171_d_n10;
        locals.var_t8_dn11 = assign62630_e97171_d_n11;
        locals.var_t8_dn14 = assign62630_e97171_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign62640_e97181, assign62640_e97181_d_n0, assign62640_e97181_d_n2, assign62640_e97181_d_n4, assign62640_e97181_d_n5, assign62640_e97181_d_n6, assign62640_e97181_d_n7, assign62640_e97181_d_n8, assign62640_e97181_d_n9, assign62640_e97181_d_n10, assign62640_e97181_d_n11, assign62640_e97181_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1499 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign62640_e97181;
        locals.var_t8_dn0 = assign62640_e97181_d_n0;
        locals.var_t8_dn2 = assign62640_e97181_d_n2;
        locals.var_t8_dn4 = assign62640_e97181_d_n4;
        locals.var_t8_dn5 = assign62640_e97181_d_n5;
        locals.var_t8_dn6 = assign62640_e97181_d_n6;
        locals.var_t8_dn7 = assign62640_e97181_d_n7;
        locals.var_t8_dn8 = assign62640_e97181_d_n8;
        locals.var_t8_dn9 = assign62640_e97181_d_n9;
        locals.var_t8_dn10 = assign62640_e97181_d_n10;
        locals.var_t8_dn11 = assign62640_e97181_d_n11;
        locals.var_t8_dn14 = assign62640_e97181_d_n14;
        locals.var_t8_rv = 0.0;

        let assign62650_e97184: f64 = (locals.var_t9 + locals.var_t8);
        let assign62650_e97186: f64 = if assign62650_e97184 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1500 = assign62650_e97186;
        locals.var_guard1500_rv = 0.0;

        let (assign62660_e97199, assign62660_e97199_d_n0, assign62660_e97199_d_n2, assign62660_e97199_d_n4, assign62660_e97199_d_n5, assign62660_e97199_d_n6, assign62660_e97199_d_n7, assign62660_e97199_d_n8, assign62660_e97199_d_n9, assign62660_e97199_d_n10, assign62660_e97199_d_n11, assign62660_e97199_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1500 != 0.0)) {
        let assign62660_e97196: f64 = (locals.var_t9 + locals.var_t8);
        let assign62660_e97197: f64 = (locals.var_pds * assign62660_e97196);
        (assign62660_e97197, ((locals.var_pds_dn0 * assign62660_e97196) + (locals.var_pds * (locals.var_t9_dn0 + locals.var_t8_dn0))), ((locals.var_pds_dn2 * assign62660_e97196) + (locals.var_pds * (locals.var_t9_dn2 + locals.var_t8_dn2))), ((locals.var_pds_dn4 * assign62660_e97196) + (locals.var_pds * (locals.var_t9_dn4 + locals.var_t8_dn4))), ((locals.var_pds_dn5 * assign62660_e97196) + (locals.var_pds * (locals.var_t9_dn5 + locals.var_t8_dn5))), ((locals.var_pds_dn6 * assign62660_e97196) + (locals.var_pds * (locals.var_t9_dn6 + locals.var_t8_dn6))), ((locals.var_pds_dn7 * assign62660_e97196) + (locals.var_pds * (locals.var_t9_dn7 + locals.var_t8_dn7))), ((locals.var_pds_dn8 * assign62660_e97196) + (locals.var_pds * (locals.var_t9_dn8 + locals.var_t8_dn8))), ((locals.var_pds_dn9 * assign62660_e97196) + (locals.var_pds * (locals.var_t9_dn9 + locals.var_t8_dn9))), ((locals.var_pds_dn10 * assign62660_e97196) + (locals.var_pds * (locals.var_t9_dn10 + locals.var_t8_dn10))), ((locals.var_pds_dn11 * assign62660_e97196) + (locals.var_pds * (locals.var_t9_dn11 + locals.var_t8_dn11))), ((locals.var_pds_dn14 * assign62660_e97196) + (locals.var_pds * (locals.var_t9_dn14 + locals.var_t8_dn14))),)
    } else {
        (locals.var_idd1, locals.var_idd1_dn0, locals.var_idd1_dn2, locals.var_idd1_dn4, locals.var_idd1_dn5, locals.var_idd1_dn6, locals.var_idd1_dn7, locals.var_idd1_dn8, locals.var_idd1_dn9, locals.var_idd1_dn10, locals.var_idd1_dn11, locals.var_idd1_dn14,)
    }
};
        locals.var_idd1 = assign62660_e97199;
        locals.var_idd1_dn0 = assign62660_e97199_d_n0;
        locals.var_idd1_dn2 = assign62660_e97199_d_n2;
        locals.var_idd1_dn4 = assign62660_e97199_d_n4;
        locals.var_idd1_dn5 = assign62660_e97199_d_n5;
        locals.var_idd1_dn6 = assign62660_e97199_d_n6;
        locals.var_idd1_dn7 = assign62660_e97199_d_n7;
        locals.var_idd1_dn8 = assign62660_e97199_d_n8;
        locals.var_idd1_dn9 = assign62660_e97199_d_n9;
        locals.var_idd1_dn10 = assign62660_e97199_d_n10;
        locals.var_idd1_dn11 = assign62660_e97199_d_n11;
        locals.var_idd1_dn14 = assign62660_e97199_d_n14;
        locals.var_idd1_rv = 0.0;

        let (assign62670_e97212, assign62670_e97212_d_n0, assign62670_e97212_d_n2, assign62670_e97212_d_n4, assign62670_e97212_d_n5, assign62670_e97212_d_n6, assign62670_e97212_d_n7, assign62670_e97212_d_n8, assign62670_e97212_d_n9, assign62670_e97212_d_n10, assign62670_e97212_d_n11, assign62670_e97212_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1500 != 0.0)) {
        let assign62670_e97208: f64 = (locals.var_betawl * locals.var_idd1);
        let assign62670_e97210: f64 = (assign62670_e97208 * locals.var_mu);
        (assign62670_e97210, ((((locals.var_betawl_dn0 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn0)) * locals.var_mu) + (assign62670_e97208 * locals.var_mu_dn0)), ((((locals.var_betawl_dn2 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn2)) * locals.var_mu) + (assign62670_e97208 * locals.var_mu_dn2)), ((((locals.var_betawl_dn4 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn4)) * locals.var_mu) + (assign62670_e97208 * locals.var_mu_dn4)), ((((locals.var_betawl_dn5 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn5)) * locals.var_mu) + (assign62670_e97208 * locals.var_mu_dn5)), ((((locals.var_betawl_dn6 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn6)) * locals.var_mu) + (assign62670_e97208 * locals.var_mu_dn6)), ((((locals.var_betawl_dn7 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn7)) * locals.var_mu) + (assign62670_e97208 * locals.var_mu_dn7)), ((((locals.var_betawl_dn8 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn8)) * locals.var_mu) + (assign62670_e97208 * locals.var_mu_dn8)), ((((locals.var_betawl_dn9 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn9)) * locals.var_mu) + (assign62670_e97208 * locals.var_mu_dn9)), ((((locals.var_betawl_dn10 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn10)) * locals.var_mu) + (assign62670_e97208 * locals.var_mu_dn10)), ((((locals.var_betawl_dn11 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn11)) * locals.var_mu) + (assign62670_e97208 * locals.var_mu_dn11)), ((((locals.var_betawl_dn14 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn14)) * locals.var_mu) + (assign62670_e97208 * locals.var_mu_dn14)),)
    } else {
        (locals.var_idspt0, locals.var_idspt0_dn0, locals.var_idspt0_dn2, locals.var_idspt0_dn4, locals.var_idspt0_dn5, locals.var_idspt0_dn6, locals.var_idspt0_dn7, locals.var_idspt0_dn8, locals.var_idspt0_dn9, locals.var_idspt0_dn10, locals.var_idspt0_dn11, locals.var_idspt0_dn14,)
    }
};
        locals.var_idspt0 = assign62670_e97212;
        locals.var_idspt0_dn0 = assign62670_e97212_d_n0;
        locals.var_idspt0_dn2 = assign62670_e97212_d_n2;
        locals.var_idspt0_dn4 = assign62670_e97212_d_n4;
        locals.var_idspt0_dn5 = assign62670_e97212_d_n5;
        locals.var_idspt0_dn6 = assign62670_e97212_d_n6;
        locals.var_idspt0_dn7 = assign62670_e97212_d_n7;
        locals.var_idspt0_dn8 = assign62670_e97212_d_n8;
        locals.var_idspt0_dn9 = assign62670_e97212_d_n9;
        locals.var_idspt0_dn10 = assign62670_e97212_d_n10;
        locals.var_idspt0_dn11 = assign62670_e97212_d_n11;
        locals.var_idspt0_dn14 = assign62670_e97212_d_n14;
        locals.var_idspt0_rv = 0.0;

        let (assign62680_e97223, assign62680_e97223_d_n0, assign62680_e97223_d_n2, assign62680_e97223_d_n4, assign62680_e97223_d_n5, assign62680_e97223_d_n6, assign62680_e97223_d_n7, assign62680_e97223_d_n8, assign62680_e97223_d_n9, assign62680_e97223_d_n10, assign62680_e97223_d_n11, assign62680_e97223_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1500 != 0.0)) {
        let assign62680_e97221: f64 = (locals.var_ids0 + locals.var_idspt0);
        (assign62680_e97221, (locals.var_ids0_dn0 + locals.var_idspt0_dn0), (locals.var_ids0_dn2 + locals.var_idspt0_dn2), (locals.var_ids0_dn4 + locals.var_idspt0_dn4), (locals.var_ids0_dn5 + locals.var_idspt0_dn5), (locals.var_ids0_dn6 + locals.var_idspt0_dn6), (locals.var_ids0_dn7 + locals.var_idspt0_dn7), (locals.var_ids0_dn8 + locals.var_idspt0_dn8), (locals.var_ids0_dn9 + locals.var_idspt0_dn9), (locals.var_ids0_dn10 + locals.var_idspt0_dn10), (locals.var_ids0_dn11 + locals.var_idspt0_dn11), (locals.var_ids0_dn14 + locals.var_idspt0_dn14),)
    } else {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn4, locals.var_ids0_dn5, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn8, locals.var_ids0_dn9, locals.var_ids0_dn10, locals.var_ids0_dn11, locals.var_ids0_dn14,)
    }
};
        locals.var_ids0 = assign62680_e97223;
        locals.var_ids0_dn0 = assign62680_e97223_d_n0;
        locals.var_ids0_dn2 = assign62680_e97223_d_n2;
        locals.var_ids0_dn4 = assign62680_e97223_d_n4;
        locals.var_ids0_dn5 = assign62680_e97223_d_n5;
        locals.var_ids0_dn6 = assign62680_e97223_d_n6;
        locals.var_ids0_dn7 = assign62680_e97223_d_n7;
        locals.var_ids0_dn8 = assign62680_e97223_d_n8;
        locals.var_ids0_dn9 = assign62680_e97223_d_n9;
        locals.var_ids0_dn10 = assign62680_e97223_d_n10;
        locals.var_ids0_dn11 = assign62680_e97223_d_n11;
        locals.var_ids0_dn14 = assign62680_e97223_d_n14;
        locals.var_ids0_rv = 0.0;

        let (assign62690_e97233, assign62690_e97233_d_n0, assign62690_e97233_d_n2, assign62690_e97233_d_n4, assign62690_e97233_d_n5, assign62690_e97233_d_n6, assign62690_e97233_d_n7, assign62690_e97233_d_n8, assign62690_e97233_d_n9, assign62690_e97233_d_n10, assign62690_e97233_d_n11, assign62690_e97233_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1500 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_idspt0, locals.var_idspt0_dn0, locals.var_idspt0_dn2, locals.var_idspt0_dn4, locals.var_idspt0_dn5, locals.var_idspt0_dn6, locals.var_idspt0_dn7, locals.var_idspt0_dn8, locals.var_idspt0_dn9, locals.var_idspt0_dn10, locals.var_idspt0_dn11, locals.var_idspt0_dn14,)
    }
};
        locals.var_idspt0 = assign62690_e97233;
        locals.var_idspt0_dn0 = assign62690_e97233_d_n0;
        locals.var_idspt0_dn2 = assign62690_e97233_d_n2;
        locals.var_idspt0_dn4 = assign62690_e97233_d_n4;
        locals.var_idspt0_dn5 = assign62690_e97233_d_n5;
        locals.var_idspt0_dn6 = assign62690_e97233_d_n6;
        locals.var_idspt0_dn7 = assign62690_e97233_d_n7;
        locals.var_idspt0_dn8 = assign62690_e97233_d_n8;
        locals.var_idspt0_dn9 = assign62690_e97233_d_n9;
        locals.var_idspt0_dn10 = assign62690_e97233_d_n10;
        locals.var_idspt0_dn11 = assign62690_e97233_d_n11;
        locals.var_idspt0_dn14 = assign62690_e97233_d_n14;
        locals.var_idspt0_rv = 0.0;

        let assign62700_e97240: f64 = if ((locals.var_flg_rsrd == 2.0) || (locals.var_flg_rsrd == 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard1501 = assign62700_e97240;
        locals.var_guard1501_rv = 0.0;

        let assign62710_e97243: f64 = if p.p296 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1502 = assign62710_e97243;
        locals.var_guard1502_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_234(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign62720_e97254, assign62720_e97254_d_n0, assign62720_e97254_d_n2, assign62720_e97254_d_n4, assign62720_e97254_d_n5, assign62720_e97254_d_n6, assign62720_e97254_d_n7, assign62720_e97254_d_n8, assign62720_e97254_d_n9, assign62720_e97254_d_n10, assign62720_e97254_d_n11, assign62720_e97254_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        (locals.var_rd23e, locals.var_rd23e_dn0, locals.var_rd23e_dn2, locals.var_rd23e_dn4, locals.var_rd23e_dn5, locals.var_rd23e_dn6, locals.var_rd23e_dn7, locals.var_rd23e_dn8, locals.var_rd23e_dn9, locals.var_rd23e_dn10, locals.var_rd23e_dn11, locals.var_rd23e_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign62720_e97254;
        locals.var_t4_dn0 = assign62720_e97254_d_n0;
        locals.var_t4_dn2 = assign62720_e97254_d_n2;
        locals.var_t4_dn4 = assign62720_e97254_d_n4;
        locals.var_t4_dn5 = assign62720_e97254_d_n5;
        locals.var_t4_dn6 = assign62720_e97254_d_n6;
        locals.var_t4_dn7 = assign62720_e97254_d_n7;
        locals.var_t4_dn8 = assign62720_e97254_d_n8;
        locals.var_t4_dn9 = assign62720_e97254_d_n9;
        locals.var_t4_dn10 = assign62720_e97254_d_n10;
        locals.var_t4_dn11 = assign62720_e97254_d_n11;
        locals.var_t4_dn14 = assign62720_e97254_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign62730_e97269, assign62730_e97269_d_n0, assign62730_e97269_d_n2, assign62730_e97269_d_n4, assign62730_e97269_d_n5, assign62730_e97269_d_n6, assign62730_e97269_d_n7, assign62730_e97269_d_n8, assign62730_e97269_d_n9, assign62730_e97269_d_n10, assign62730_e97269_d_n11, assign62730_e97269_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign62730_e97266: f64 = (locals.var_vgse - p.p300);
        let assign62730_e97267: f64 = (locals.var_uc_rd24 * assign62730_e97266);
        (assign62730_e97267, (locals.var_uc_rd24 * locals.var_vgse_dn0), (locals.var_uc_rd24 * locals.var_vgse_dn2), 0.0, 0.0, 0.0, (locals.var_uc_rd24 * locals.var_vgse_dn7), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign62730_e97269;
        locals.var_t1_dn0 = assign62730_e97269_d_n0;
        locals.var_t1_dn2 = assign62730_e97269_d_n2;
        locals.var_t1_dn4 = assign62730_e97269_d_n4;
        locals.var_t1_dn5 = assign62730_e97269_d_n5;
        locals.var_t1_dn6 = assign62730_e97269_d_n6;
        locals.var_t1_dn7 = assign62730_e97269_d_n7;
        locals.var_t1_dn8 = assign62730_e97269_d_n8;
        locals.var_t1_dn9 = assign62730_e97269_d_n9;
        locals.var_t1_dn10 = assign62730_e97269_d_n10;
        locals.var_t1_dn11 = assign62730_e97269_d_n11;
        locals.var_t1_dn14 = assign62730_e97269_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign62740_e97286, assign62740_e97286_d_n0, assign62740_e97286_d_n2, assign62740_e97286_d_n4, assign62740_e97286_d_n5, assign62740_e97286_d_n6, assign62740_e97286_d_n7, assign62740_e97286_d_n8, assign62740_e97286_d_n9, assign62740_e97286_d_n10, assign62740_e97286_d_n11, assign62740_e97286_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign62740_e97280: f64 = (locals.var_t1 - locals.var_t4);
        let assign62740_e97283: f64 = (0.01 * 0.01);
        let assign62740_e97284: f64 = (assign62740_e97280 - assign62740_e97283);
        (assign62740_e97284, (locals.var_t1_dn0 - locals.var_t4_dn0), (locals.var_t1_dn2 - locals.var_t4_dn2), (locals.var_t1_dn4 - locals.var_t4_dn4), (locals.var_t1_dn5 - locals.var_t4_dn5), (locals.var_t1_dn6 - locals.var_t4_dn6), (locals.var_t1_dn7 - locals.var_t4_dn7), (locals.var_t1_dn8 - locals.var_t4_dn8), (locals.var_t1_dn9 - locals.var_t4_dn9), (locals.var_t1_dn10 - locals.var_t4_dn10), (locals.var_t1_dn11 - locals.var_t4_dn11), (locals.var_t1_dn14 - locals.var_t4_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign62740_e97286;
        locals.var_tmf1_dn0 = assign62740_e97286_d_n0;
        locals.var_tmf1_dn2 = assign62740_e97286_d_n2;
        locals.var_tmf1_dn4 = assign62740_e97286_d_n4;
        locals.var_tmf1_dn5 = assign62740_e97286_d_n5;
        locals.var_tmf1_dn6 = assign62740_e97286_d_n6;
        locals.var_tmf1_dn7 = assign62740_e97286_d_n7;
        locals.var_tmf1_dn8 = assign62740_e97286_d_n8;
        locals.var_tmf1_dn9 = assign62740_e97286_d_n9;
        locals.var_tmf1_dn10 = assign62740_e97286_d_n10;
        locals.var_tmf1_dn11 = assign62740_e97286_d_n11;
        locals.var_tmf1_dn14 = assign62740_e97286_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign62750_e97303, assign62750_e97303_d_n0, assign62750_e97303_d_n2, assign62750_e97303_d_n4, assign62750_e97303_d_n5, assign62750_e97303_d_n6, assign62750_e97303_d_n7, assign62750_e97303_d_n8, assign62750_e97303_d_n9, assign62750_e97303_d_n10, assign62750_e97303_d_n11, assign62750_e97303_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign62750_e97297: f64 = (4.0 * locals.var_t4);
        let assign62750_e97300: f64 = (0.01 * 0.01);
        let assign62750_e97301: f64 = (assign62750_e97297 * assign62750_e97300);
        (assign62750_e97301, ((4.0 * locals.var_t4_dn0) * assign62750_e97300), ((4.0 * locals.var_t4_dn2) * assign62750_e97300), ((4.0 * locals.var_t4_dn4) * assign62750_e97300), ((4.0 * locals.var_t4_dn5) * assign62750_e97300), ((4.0 * locals.var_t4_dn6) * assign62750_e97300), ((4.0 * locals.var_t4_dn7) * assign62750_e97300), ((4.0 * locals.var_t4_dn8) * assign62750_e97300), ((4.0 * locals.var_t4_dn9) * assign62750_e97300), ((4.0 * locals.var_t4_dn10) * assign62750_e97300), ((4.0 * locals.var_t4_dn11) * assign62750_e97300), ((4.0 * locals.var_t4_dn14) * assign62750_e97300),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign62750_e97303;
        locals.var_tmf2_dn0 = assign62750_e97303_d_n0;
        locals.var_tmf2_dn2 = assign62750_e97303_d_n2;
        locals.var_tmf2_dn4 = assign62750_e97303_d_n4;
        locals.var_tmf2_dn5 = assign62750_e97303_d_n5;
        locals.var_tmf2_dn6 = assign62750_e97303_d_n6;
        locals.var_tmf2_dn7 = assign62750_e97303_d_n7;
        locals.var_tmf2_dn8 = assign62750_e97303_d_n8;
        locals.var_tmf2_dn9 = assign62750_e97303_d_n9;
        locals.var_tmf2_dn10 = assign62750_e97303_d_n10;
        locals.var_tmf2_dn11 = assign62750_e97303_d_n11;
        locals.var_tmf2_dn14 = assign62750_e97303_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign62760_e97320, assign62760_e97320_d_n0, assign62760_e97320_d_n2, assign62760_e97320_d_n4, assign62760_e97320_d_n5, assign62760_e97320_d_n6, assign62760_e97320_d_n7, assign62760_e97320_d_n8, assign62760_e97320_d_n9, assign62760_e97320_d_n10, assign62760_e97320_d_n11, assign62760_e97320_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let (assign62760_e97318, assign62760_e97318_d_n0, assign62760_e97318_d_n2, assign62760_e97318_d_n4, assign62760_e97318_d_n5, assign62760_e97318_d_n6, assign62760_e97318_d_n7, assign62760_e97318_d_n8, assign62760_e97318_d_n9, assign62760_e97318_d_n10, assign62760_e97318_d_n11, assign62760_e97318_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign62760_e97317: f64 = (-locals.var_tmf2);
                (assign62760_e97317, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign62760_e97318, assign62760_e97318_d_n0, assign62760_e97318_d_n2, assign62760_e97318_d_n4, assign62760_e97318_d_n5, assign62760_e97318_d_n6, assign62760_e97318_d_n7, assign62760_e97318_d_n8, assign62760_e97318_d_n9, assign62760_e97318_d_n10, assign62760_e97318_d_n11, assign62760_e97318_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign62760_e97320;
        locals.var_tmf2_dn0 = assign62760_e97320_d_n0;
        locals.var_tmf2_dn2 = assign62760_e97320_d_n2;
        locals.var_tmf2_dn4 = assign62760_e97320_d_n4;
        locals.var_tmf2_dn5 = assign62760_e97320_d_n5;
        locals.var_tmf2_dn6 = assign62760_e97320_d_n6;
        locals.var_tmf2_dn7 = assign62760_e97320_d_n7;
        locals.var_tmf2_dn8 = assign62760_e97320_d_n8;
        locals.var_tmf2_dn9 = assign62760_e97320_d_n9;
        locals.var_tmf2_dn10 = assign62760_e97320_d_n10;
        locals.var_tmf2_dn11 = assign62760_e97320_d_n11;
        locals.var_tmf2_dn14 = assign62760_e97320_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign62770_e97336, assign62770_e97336_d_n0, assign62770_e97336_d_n2, assign62770_e97336_d_n4, assign62770_e97336_d_n5, assign62770_e97336_d_n6, assign62770_e97336_d_n7, assign62770_e97336_d_n8, assign62770_e97336_d_n9, assign62770_e97336_d_n10, assign62770_e97336_d_n11, assign62770_e97336_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign62770_e97331: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign62770_e97333: f64 = (assign62770_e97331 + locals.var_tmf2);
        let assign62770_e97334: f64 = (assign62770_e97333).sqrt();
        (assign62770_e97334, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign62770_e97334)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign62770_e97334)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign62770_e97334)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign62770_e97334)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign62770_e97334)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign62770_e97334)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign62770_e97334)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign62770_e97334)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign62770_e97334)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign62770_e97334)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign62770_e97334)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign62770_e97336;
        locals.var_tmf2_dn0 = assign62770_e97336_d_n0;
        locals.var_tmf2_dn2 = assign62770_e97336_d_n2;
        locals.var_tmf2_dn4 = assign62770_e97336_d_n4;
        locals.var_tmf2_dn5 = assign62770_e97336_d_n5;
        locals.var_tmf2_dn6 = assign62770_e97336_d_n6;
        locals.var_tmf2_dn7 = assign62770_e97336_d_n7;
        locals.var_tmf2_dn8 = assign62770_e97336_d_n8;
        locals.var_tmf2_dn9 = assign62770_e97336_d_n9;
        locals.var_tmf2_dn10 = assign62770_e97336_d_n10;
        locals.var_tmf2_dn11 = assign62770_e97336_d_n11;
        locals.var_tmf2_dn14 = assign62770_e97336_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign62780_e97353, assign62780_e97353_d_n0, assign62780_e97353_d_n2, assign62780_e97353_d_n4, assign62780_e97353_d_n5, assign62780_e97353_d_n6, assign62780_e97353_d_n7, assign62780_e97353_d_n8, assign62780_e97353_d_n9, assign62780_e97353_d_n10, assign62780_e97353_d_n11, assign62780_e97353_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign62780_e97349: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign62780_e97350: f64 = (1.0 + assign62780_e97349);
        let assign62780_e97351: f64 = (0.5 * assign62780_e97350);
        (assign62780_e97351, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign62780_e97353;
        locals.var_t0_dn0 = assign62780_e97353_d_n0;
        locals.var_t0_dn2 = assign62780_e97353_d_n2;
        locals.var_t0_dn4 = assign62780_e97353_d_n4;
        locals.var_t0_dn5 = assign62780_e97353_d_n5;
        locals.var_t0_dn6 = assign62780_e97353_d_n6;
        locals.var_t0_dn7 = assign62780_e97353_d_n7;
        locals.var_t0_dn8 = assign62780_e97353_d_n8;
        locals.var_t0_dn9 = assign62780_e97353_d_n9;
        locals.var_t0_dn10 = assign62780_e97353_d_n10;
        locals.var_t0_dn11 = assign62780_e97353_d_n11;
        locals.var_t0_dn14 = assign62780_e97353_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign62790_e97370, assign62790_e97370_d_n0, assign62790_e97370_d_n2, assign62790_e97370_d_n4, assign62790_e97370_d_n5, assign62790_e97370_d_n6, assign62790_e97370_d_n7, assign62790_e97370_d_n8, assign62790_e97370_d_n9, assign62790_e97370_d_n10, assign62790_e97370_d_n11, assign62790_e97370_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign62790_e97366: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign62790_e97367: f64 = (0.5 * assign62790_e97366);
        let assign62790_e97368: f64 = (locals.var_t4 + assign62790_e97367);
        (assign62790_e97368, (locals.var_t4_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t4_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t4_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t4_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t4_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t4_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t4_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t4_dn9 + (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t4_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t4_dn11 + (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t4_dn14 + (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign62790_e97370;
        locals.var_t2_dn0 = assign62790_e97370_d_n0;
        locals.var_t2_dn2 = assign62790_e97370_d_n2;
        locals.var_t2_dn4 = assign62790_e97370_d_n4;
        locals.var_t2_dn5 = assign62790_e97370_d_n5;
        locals.var_t2_dn6 = assign62790_e97370_d_n6;
        locals.var_t2_dn7 = assign62790_e97370_d_n7;
        locals.var_t2_dn8 = assign62790_e97370_d_n8;
        locals.var_t2_dn9 = assign62790_e97370_d_n9;
        locals.var_t2_dn10 = assign62790_e97370_d_n10;
        locals.var_t2_dn11 = assign62790_e97370_d_n11;
        locals.var_t2_dn14 = assign62790_e97370_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign62800_e97385, assign62800_e97385_d_n0, assign62800_e97385_d_n2, assign62800_e97385_d_n4, assign62800_e97385_d_n5, assign62800_e97385_d_n6, assign62800_e97385_d_n7, assign62800_e97385_d_n8, assign62800_e97385_d_n9, assign62800_e97385_d_n10, assign62800_e97385_d_n11, assign62800_e97385_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign62800_e97382: f64 = (p.p296 + 1.0);
        let assign62800_e97383: f64 = (locals.var_t4 * assign62800_e97382);
        (assign62800_e97383, (locals.var_t4_dn0 * assign62800_e97382), (locals.var_t4_dn2 * assign62800_e97382), (locals.var_t4_dn4 * assign62800_e97382), (locals.var_t4_dn5 * assign62800_e97382), (locals.var_t4_dn6 * assign62800_e97382), (locals.var_t4_dn7 * assign62800_e97382), (locals.var_t4_dn8 * assign62800_e97382), (locals.var_t4_dn9 * assign62800_e97382), (locals.var_t4_dn10 * assign62800_e97382), (locals.var_t4_dn11 * assign62800_e97382), (locals.var_t4_dn14 * assign62800_e97382),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign62800_e97385;
        locals.var_t3_dn0 = assign62800_e97385_d_n0;
        locals.var_t3_dn2 = assign62800_e97385_d_n2;
        locals.var_t3_dn4 = assign62800_e97385_d_n4;
        locals.var_t3_dn5 = assign62800_e97385_d_n5;
        locals.var_t3_dn6 = assign62800_e97385_d_n6;
        locals.var_t3_dn7 = assign62800_e97385_d_n7;
        locals.var_t3_dn8 = assign62800_e97385_d_n8;
        locals.var_t3_dn9 = assign62800_e97385_d_n9;
        locals.var_t3_dn10 = assign62800_e97385_d_n10;
        locals.var_t3_dn11 = assign62800_e97385_d_n11;
        locals.var_t3_dn14 = assign62800_e97385_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign62810_e97402, assign62810_e97402_d_n0, assign62810_e97402_d_n2, assign62810_e97402_d_n4, assign62810_e97402_d_n5, assign62810_e97402_d_n6, assign62810_e97402_d_n7, assign62810_e97402_d_n8, assign62810_e97402_d_n9, assign62810_e97402_d_n10, assign62810_e97402_d_n11, assign62810_e97402_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign62810_e97396: f64 = (locals.var_t3 - locals.var_t2);
        let assign62810_e97399: f64 = (0.01 * 0.01);
        let assign62810_e97400: f64 = (assign62810_e97396 - assign62810_e97399);
        (assign62810_e97400, (locals.var_t3_dn0 - locals.var_t2_dn0), (locals.var_t3_dn2 - locals.var_t2_dn2), (locals.var_t3_dn4 - locals.var_t2_dn4), (locals.var_t3_dn5 - locals.var_t2_dn5), (locals.var_t3_dn6 - locals.var_t2_dn6), (locals.var_t3_dn7 - locals.var_t2_dn7), (locals.var_t3_dn8 - locals.var_t2_dn8), (locals.var_t3_dn9 - locals.var_t2_dn9), (locals.var_t3_dn10 - locals.var_t2_dn10), (locals.var_t3_dn11 - locals.var_t2_dn11), (locals.var_t3_dn14 - locals.var_t2_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign62810_e97402;
        locals.var_tmf1_dn0 = assign62810_e97402_d_n0;
        locals.var_tmf1_dn2 = assign62810_e97402_d_n2;
        locals.var_tmf1_dn4 = assign62810_e97402_d_n4;
        locals.var_tmf1_dn5 = assign62810_e97402_d_n5;
        locals.var_tmf1_dn6 = assign62810_e97402_d_n6;
        locals.var_tmf1_dn7 = assign62810_e97402_d_n7;
        locals.var_tmf1_dn8 = assign62810_e97402_d_n8;
        locals.var_tmf1_dn9 = assign62810_e97402_d_n9;
        locals.var_tmf1_dn10 = assign62810_e97402_d_n10;
        locals.var_tmf1_dn11 = assign62810_e97402_d_n11;
        locals.var_tmf1_dn14 = assign62810_e97402_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign62820_e97419, assign62820_e97419_d_n0, assign62820_e97419_d_n2, assign62820_e97419_d_n4, assign62820_e97419_d_n5, assign62820_e97419_d_n6, assign62820_e97419_d_n7, assign62820_e97419_d_n8, assign62820_e97419_d_n9, assign62820_e97419_d_n10, assign62820_e97419_d_n11, assign62820_e97419_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign62820_e97413: f64 = (4.0 * locals.var_t3);
        let assign62820_e97416: f64 = (0.01 * 0.01);
        let assign62820_e97417: f64 = (assign62820_e97413 * assign62820_e97416);
        (assign62820_e97417, ((4.0 * locals.var_t3_dn0) * assign62820_e97416), ((4.0 * locals.var_t3_dn2) * assign62820_e97416), ((4.0 * locals.var_t3_dn4) * assign62820_e97416), ((4.0 * locals.var_t3_dn5) * assign62820_e97416), ((4.0 * locals.var_t3_dn6) * assign62820_e97416), ((4.0 * locals.var_t3_dn7) * assign62820_e97416), ((4.0 * locals.var_t3_dn8) * assign62820_e97416), ((4.0 * locals.var_t3_dn9) * assign62820_e97416), ((4.0 * locals.var_t3_dn10) * assign62820_e97416), ((4.0 * locals.var_t3_dn11) * assign62820_e97416), ((4.0 * locals.var_t3_dn14) * assign62820_e97416),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign62820_e97419;
        locals.var_tmf2_dn0 = assign62820_e97419_d_n0;
        locals.var_tmf2_dn2 = assign62820_e97419_d_n2;
        locals.var_tmf2_dn4 = assign62820_e97419_d_n4;
        locals.var_tmf2_dn5 = assign62820_e97419_d_n5;
        locals.var_tmf2_dn6 = assign62820_e97419_d_n6;
        locals.var_tmf2_dn7 = assign62820_e97419_d_n7;
        locals.var_tmf2_dn8 = assign62820_e97419_d_n8;
        locals.var_tmf2_dn9 = assign62820_e97419_d_n9;
        locals.var_tmf2_dn10 = assign62820_e97419_d_n10;
        locals.var_tmf2_dn11 = assign62820_e97419_d_n11;
        locals.var_tmf2_dn14 = assign62820_e97419_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign62830_e97436, assign62830_e97436_d_n0, assign62830_e97436_d_n2, assign62830_e97436_d_n4, assign62830_e97436_d_n5, assign62830_e97436_d_n6, assign62830_e97436_d_n7, assign62830_e97436_d_n8, assign62830_e97436_d_n9, assign62830_e97436_d_n10, assign62830_e97436_d_n11, assign62830_e97436_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let (assign62830_e97434, assign62830_e97434_d_n0, assign62830_e97434_d_n2, assign62830_e97434_d_n4, assign62830_e97434_d_n5, assign62830_e97434_d_n6, assign62830_e97434_d_n7, assign62830_e97434_d_n8, assign62830_e97434_d_n9, assign62830_e97434_d_n10, assign62830_e97434_d_n11, assign62830_e97434_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign62830_e97433: f64 = (-locals.var_tmf2);
                (assign62830_e97433, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign62830_e97434, assign62830_e97434_d_n0, assign62830_e97434_d_n2, assign62830_e97434_d_n4, assign62830_e97434_d_n5, assign62830_e97434_d_n6, assign62830_e97434_d_n7, assign62830_e97434_d_n8, assign62830_e97434_d_n9, assign62830_e97434_d_n10, assign62830_e97434_d_n11, assign62830_e97434_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign62830_e97436;
        locals.var_tmf2_dn0 = assign62830_e97436_d_n0;
        locals.var_tmf2_dn2 = assign62830_e97436_d_n2;
        locals.var_tmf2_dn4 = assign62830_e97436_d_n4;
        locals.var_tmf2_dn5 = assign62830_e97436_d_n5;
        locals.var_tmf2_dn6 = assign62830_e97436_d_n6;
        locals.var_tmf2_dn7 = assign62830_e97436_d_n7;
        locals.var_tmf2_dn8 = assign62830_e97436_d_n8;
        locals.var_tmf2_dn9 = assign62830_e97436_d_n9;
        locals.var_tmf2_dn10 = assign62830_e97436_d_n10;
        locals.var_tmf2_dn11 = assign62830_e97436_d_n11;
        locals.var_tmf2_dn14 = assign62830_e97436_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign62840_e97452, assign62840_e97452_d_n0, assign62840_e97452_d_n2, assign62840_e97452_d_n4, assign62840_e97452_d_n5, assign62840_e97452_d_n6, assign62840_e97452_d_n7, assign62840_e97452_d_n8, assign62840_e97452_d_n9, assign62840_e97452_d_n10, assign62840_e97452_d_n11, assign62840_e97452_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign62840_e97447: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign62840_e97449: f64 = (assign62840_e97447 + locals.var_tmf2);
        let assign62840_e97450: f64 = (assign62840_e97449).sqrt();
        (assign62840_e97450, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign62840_e97450)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign62840_e97450)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign62840_e97450)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign62840_e97450)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign62840_e97450)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign62840_e97450)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign62840_e97450)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign62840_e97450)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign62840_e97450)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign62840_e97450)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign62840_e97450)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign62840_e97452;
        locals.var_tmf2_dn0 = assign62840_e97452_d_n0;
        locals.var_tmf2_dn2 = assign62840_e97452_d_n2;
        locals.var_tmf2_dn4 = assign62840_e97452_d_n4;
        locals.var_tmf2_dn5 = assign62840_e97452_d_n5;
        locals.var_tmf2_dn6 = assign62840_e97452_d_n6;
        locals.var_tmf2_dn7 = assign62840_e97452_d_n7;
        locals.var_tmf2_dn8 = assign62840_e97452_d_n8;
        locals.var_tmf2_dn9 = assign62840_e97452_d_n9;
        locals.var_tmf2_dn10 = assign62840_e97452_d_n10;
        locals.var_tmf2_dn11 = assign62840_e97452_d_n11;
        locals.var_tmf2_dn14 = assign62840_e97452_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign62850_e97469, assign62850_e97469_d_n0, assign62850_e97469_d_n2, assign62850_e97469_d_n4, assign62850_e97469_d_n5, assign62850_e97469_d_n6, assign62850_e97469_d_n7, assign62850_e97469_d_n8, assign62850_e97469_d_n9, assign62850_e97469_d_n10, assign62850_e97469_d_n11, assign62850_e97469_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign62850_e97465: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign62850_e97466: f64 = (1.0 + assign62850_e97465);
        let assign62850_e97467: f64 = (0.5 * assign62850_e97466);
        (assign62850_e97467, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign62850_e97469;
        locals.var_t0_dn0 = assign62850_e97469_d_n0;
        locals.var_t0_dn2 = assign62850_e97469_d_n2;
        locals.var_t0_dn4 = assign62850_e97469_d_n4;
        locals.var_t0_dn5 = assign62850_e97469_d_n5;
        locals.var_t0_dn6 = assign62850_e97469_d_n6;
        locals.var_t0_dn7 = assign62850_e97469_d_n7;
        locals.var_t0_dn8 = assign62850_e97469_d_n8;
        locals.var_t0_dn9 = assign62850_e97469_d_n9;
        locals.var_t0_dn10 = assign62850_e97469_d_n10;
        locals.var_t0_dn11 = assign62850_e97469_d_n11;
        locals.var_t0_dn14 = assign62850_e97469_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign62860_e97486, assign62860_e97486_d_n0, assign62860_e97486_d_n2, assign62860_e97486_d_n4, assign62860_e97486_d_n5, assign62860_e97486_d_n6, assign62860_e97486_d_n7, assign62860_e97486_d_n8, assign62860_e97486_d_n9, assign62860_e97486_d_n10, assign62860_e97486_d_n11, assign62860_e97486_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
        let assign62860_e97482: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign62860_e97483: f64 = (0.5 * assign62860_e97482);
        let assign62860_e97484: f64 = (locals.var_t3 - assign62860_e97483);
        (assign62860_e97484, (locals.var_t3_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t3_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t3_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t3_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t3_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t3_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t3_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t3_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t3_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t3_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t3_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign62860_e97486;
        locals.var_t7_dn0 = assign62860_e97486_d_n0;
        locals.var_t7_dn2 = assign62860_e97486_d_n2;
        locals.var_t7_dn4 = assign62860_e97486_d_n4;
        locals.var_t7_dn5 = assign62860_e97486_d_n5;
        locals.var_t7_dn6 = assign62860_e97486_d_n6;
        locals.var_t7_dn7 = assign62860_e97486_d_n7;
        locals.var_t7_dn8 = assign62860_e97486_d_n8;
        locals.var_t7_dn9 = assign62860_e97486_d_n9;
        locals.var_t7_dn10 = assign62860_e97486_d_n10;
        locals.var_t7_dn11 = assign62860_e97486_d_n11;
        locals.var_t7_dn14 = assign62860_e97486_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign62870_e97498, assign62870_e97498_d_n0, assign62870_e97498_d_n2, assign62870_e97498_d_n4, assign62870_e97498_d_n5, assign62870_e97498_d_n6, assign62870_e97498_d_n7, assign62870_e97498_d_n8, assign62870_e97498_d_n9, assign62870_e97498_d_n10, assign62870_e97498_d_n11, assign62870_e97498_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 == 0.0)) {
        (locals.var_rd23e, locals.var_rd23e_dn0, locals.var_rd23e_dn2, locals.var_rd23e_dn4, locals.var_rd23e_dn5, locals.var_rd23e_dn6, locals.var_rd23e_dn7, locals.var_rd23e_dn8, locals.var_rd23e_dn9, locals.var_rd23e_dn10, locals.var_rd23e_dn11, locals.var_rd23e_dn14,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign62870_e97498;
        locals.var_t7_dn0 = assign62870_e97498_d_n0;
        locals.var_t7_dn2 = assign62870_e97498_d_n2;
        locals.var_t7_dn4 = assign62870_e97498_d_n4;
        locals.var_t7_dn5 = assign62870_e97498_d_n5;
        locals.var_t7_dn6 = assign62870_e97498_d_n6;
        locals.var_t7_dn7 = assign62870_e97498_d_n7;
        locals.var_t7_dn8 = assign62870_e97498_d_n8;
        locals.var_t7_dn9 = assign62870_e97498_d_n9;
        locals.var_t7_dn10 = assign62870_e97498_d_n10;
        locals.var_t7_dn11 = assign62870_e97498_d_n11;
        locals.var_t7_dn14 = assign62870_e97498_d_n14;
        locals.var_t7_rv = 0.0;

        let assign62880_e97501: f64 = if locals.var_vdse >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1503 = assign62880_e97501;
        locals.var_guard1503_rv = 0.0;

        let (assign62890_e97512, assign62890_e97512_d_n0, assign62890_e97512_d_n2,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1503 != 0.0)) {
        (locals.var_vdse, locals.var_vdse_dn0, locals.var_vdse_dn2,)
    } else {
        (locals.var_vdse_eff, locals.var_vdse_eff_dn0, locals.var_vdse_eff_dn2,)
    }
};
        locals.var_vdse_eff = assign62890_e97512;
        locals.var_vdse_eff_dn0 = assign62890_e97512_d_n0;
        locals.var_vdse_eff_dn2 = assign62890_e97512_d_n2;
        locals.var_vdse_eff_rv = 0.0;

        let (assign62900_e97524, assign62900_e97524_d_n0, assign62900_e97524_d_n2,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1503 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_vdse_eff, locals.var_vdse_eff_dn0, locals.var_vdse_eff_dn2,)
    }
};
        locals.var_vdse_eff = assign62900_e97524;
        locals.var_vdse_eff_dn0 = assign62900_e97524_d_n0;
        locals.var_vdse_eff_dn2 = assign62900_e97524_d_n2;
        locals.var_vdse_eff_rv = 0.0;

        let assign62910_e97528: f64 = (20.0 * 1e-12);
        let assign62910_e97529: f64 = if locals.var_vdse_eff < assign62910_e97528 { 1.0 } else { 0.0 };
        locals.var_guard1504 = assign62910_e97529;
        locals.var_guard1504_rv = 0.0;

        let (assign62920_e97560,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1504 != 0.0)) {
        let assign62920_e97540: f64 = (20.0 + 1.0);
        let assign62920_e97543: f64 = (p.p297 - 1.0);
        let assign62920_e97544: f64 = (assign62920_e97540).powf(assign62920_e97543);
        let assign62920_e97547: f64 = (20.0 + 1.0);
        let assign62920_e97550: f64 = (0.5 * p.p297);
        let assign62920_e97552: f64 = (assign62920_e97550 * 20.0);
        let assign62920_e97553: f64 = (assign62920_e97547 - assign62920_e97552);
        let assign62920_e97554: f64 = (assign62920_e97544 * assign62920_e97553);
        let assign62920_e97557: f64 = (1e-12_f64).powf(p.p297);
        let assign62920_e97558: f64 = (assign62920_e97554 * assign62920_e97557);
        (assign62920_e97558,)
    } else {
        (locals.var_ra_alpha,)
    }
};
        locals.var_ra_alpha = assign62920_e97560;
        locals.var_ra_alpha_rv = 0.0;

        let (assign62930_e97589,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1504 != 0.0)) {
        let assign62930_e97571: f64 = (0.5 * p.p297);
        let assign62930_e97574: f64 = (20.0 + 1.0);
        let assign62930_e97577: f64 = (p.p297 - 1.0);
        let assign62930_e97578: f64 = (assign62930_e97574).powf(assign62930_e97577);
        let assign62930_e97579: f64 = (assign62930_e97571 * assign62930_e97578);
        let assign62930_e97581: f64 = (assign62930_e97579 / 20.0);
        let assign62930_e97585: f64 = (p.p297 - 2.0);
        let assign62930_e97586: f64 = (1e-12_f64).powf(assign62930_e97585);
        let assign62930_e97587: f64 = (assign62930_e97581 * assign62930_e97586);
        (assign62930_e97587,)
    } else {
        (locals.var_ra_beta,)
    }
};
        locals.var_ra_beta = assign62930_e97589;
        locals.var_ra_beta_rv = 0.0;

        let (assign62940_e97606, assign62940_e97606_d_n0, assign62940_e97606_d_n2, assign62940_e97606_d_n4, assign62940_e97606_d_n5, assign62940_e97606_d_n6, assign62940_e97606_d_n7, assign62940_e97606_d_n8, assign62940_e97606_d_n9, assign62940_e97606_d_n10, assign62940_e97606_d_n11, assign62940_e97606_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1504 != 0.0)) {
        let assign62940_e97601: f64 = (locals.var_ra_beta * locals.var_vdse_eff);
        let assign62940_e97603: f64 = (assign62940_e97601 * locals.var_vdse_eff);
        let assign62940_e97604: f64 = (locals.var_ra_alpha + assign62940_e97603);
        (assign62940_e97604, (((locals.var_ra_beta * locals.var_vdse_eff_dn0) * locals.var_vdse_eff) + (assign62940_e97601 * locals.var_vdse_eff_dn0)), (((locals.var_ra_beta * locals.var_vdse_eff_dn2) * locals.var_vdse_eff) + (assign62940_e97601 * locals.var_vdse_eff_dn2)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign62940_e97606;
        locals.var_t1_dn0 = assign62940_e97606_d_n0;
        locals.var_t1_dn2 = assign62940_e97606_d_n2;
        locals.var_t1_dn4 = assign62940_e97606_d_n4;
        locals.var_t1_dn5 = assign62940_e97606_d_n5;
        locals.var_t1_dn6 = assign62940_e97606_d_n6;
        locals.var_t1_dn7 = assign62940_e97606_d_n7;
        locals.var_t1_dn8 = assign62940_e97606_d_n8;
        locals.var_t1_dn9 = assign62940_e97606_d_n9;
        locals.var_t1_dn10 = assign62940_e97606_d_n10;
        locals.var_t1_dn11 = assign62940_e97606_d_n11;
        locals.var_t1_dn14 = assign62940_e97606_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign62950_e97622, assign62950_e97622_d_n0, assign62950_e97622_d_n2, assign62950_e97622_d_n4, assign62950_e97622_d_n5, assign62950_e97622_d_n6, assign62950_e97622_d_n7, assign62950_e97622_d_n8, assign62950_e97622_d_n9, assign62950_e97622_d_n10, assign62950_e97622_d_n11, assign62950_e97622_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1504 == 0.0)) {
        let assign62950_e97618: f64 = (locals.var_vdse_eff + 1e-12);
        let assign62950_e97620: f64 = (assign62950_e97618).powf(p.p297);
        (assign62950_e97620, if 0.0 == 0.0 && ((p.p297) as f64).is_finite() && ((p.p297) as f64).fract() == 0.0 { if p.p297 == 0.0 { 0.0 } else { (p.p297 * ((assign62950_e97618).powf(p.p297 - 1.0) * locals.var_vdse_eff_dn0)) } } else { (assign62950_e97620 * (p.p297 * (locals.var_vdse_eff_dn0 / assign62950_e97618))) }, if 0.0 == 0.0 && ((p.p297) as f64).is_finite() && ((p.p297) as f64).fract() == 0.0 { if p.p297 == 0.0 { 0.0 } else { (p.p297 * ((assign62950_e97618).powf(p.p297 - 1.0) * locals.var_vdse_eff_dn2)) } } else { (assign62950_e97620 * (p.p297 * (locals.var_vdse_eff_dn2 / assign62950_e97618))) }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign62950_e97622;
        locals.var_t1_dn0 = assign62950_e97622_d_n0;
        locals.var_t1_dn2 = assign62950_e97622_d_n2;
        locals.var_t1_dn4 = assign62950_e97622_d_n4;
        locals.var_t1_dn5 = assign62950_e97622_d_n5;
        locals.var_t1_dn6 = assign62950_e97622_d_n6;
        locals.var_t1_dn7 = assign62950_e97622_d_n7;
        locals.var_t1_dn8 = assign62950_e97622_d_n8;
        locals.var_t1_dn9 = assign62950_e97622_d_n9;
        locals.var_t1_dn10 = assign62950_e97622_d_n10;
        locals.var_t1_dn11 = assign62950_e97622_d_n11;
        locals.var_t1_dn14 = assign62950_e97622_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_235(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign62960_e97635, assign62960_e97635_d_n0, assign62960_e97635_d_n2, assign62960_e97635_d_n4, assign62960_e97635_d_n5, assign62960_e97635_d_n6, assign62960_e97635_d_n7, assign62960_e97635_d_n8, assign62960_e97635_d_n9, assign62960_e97635_d_n10, assign62960_e97635_d_n11, assign62960_e97635_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) {
        let assign62960_e97631: f64 = (locals.var_vdse_eff + 1e-12);
        let assign62960_e97633: f64 = (assign62960_e97631).powf(p.p299);
        (assign62960_e97633, if 0.0 == 0.0 && ((p.p299) as f64).is_finite() && ((p.p299) as f64).fract() == 0.0 { if p.p299 == 0.0 { 0.0 } else { (p.p299 * ((assign62960_e97631).powf(p.p299 - 1.0) * locals.var_vdse_eff_dn0)) } } else { (assign62960_e97633 * (p.p299 * (locals.var_vdse_eff_dn0 / assign62960_e97631))) }, if 0.0 == 0.0 && ((p.p299) as f64).is_finite() && ((p.p299) as f64).fract() == 0.0 { if p.p299 == 0.0 { 0.0 } else { (p.p299 * ((assign62960_e97631).powf(p.p299 - 1.0) * locals.var_vdse_eff_dn2)) } } else { (assign62960_e97633 * (p.p299 * (locals.var_vdse_eff_dn2 / assign62960_e97631))) }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign62960_e97635;
        locals.var_t9_dn0 = assign62960_e97635_d_n0;
        locals.var_t9_dn2 = assign62960_e97635_d_n2;
        locals.var_t9_dn4 = assign62960_e97635_d_n4;
        locals.var_t9_dn5 = assign62960_e97635_d_n5;
        locals.var_t9_dn6 = assign62960_e97635_d_n6;
        locals.var_t9_dn7 = assign62960_e97635_d_n7;
        locals.var_t9_dn8 = assign62960_e97635_d_n8;
        locals.var_t9_dn9 = assign62960_e97635_d_n9;
        locals.var_t9_dn10 = assign62960_e97635_d_n10;
        locals.var_t9_dn11 = assign62960_e97635_d_n11;
        locals.var_t9_dn14 = assign62960_e97635_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign62970_e97654, assign62970_e97654_d_n0, assign62970_e97654_d_n2, assign62970_e97654_d_n4, assign62970_e97654_d_n5, assign62970_e97654_d_n6, assign62970_e97654_d_n7, assign62970_e97654_d_n8, assign62970_e97654_d_n9, assign62970_e97654_d_n10, assign62970_e97654_d_n11, assign62970_e97654_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) {
        let assign62970_e97644: f64 = (locals.var_t7 * locals.var_t1);
        let assign62970_e97647: f64 = (locals.var_vbse * locals.var_uc_rd22);
        let assign62970_e97649: f64 = (assign62970_e97647 * locals.var_t9);
        let assign62970_e97650: f64 = (assign62970_e97644 + assign62970_e97649);
        let assign62970_e97652: f64 = (assign62970_e97650 / locals.var_weff_nf);
        (assign62970_e97652, ((((locals.var_t7_dn0 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn0)) + (((locals.var_vbse_dn0 * locals.var_uc_rd22) * locals.var_t9) + (assign62970_e97647 * locals.var_t9_dn0))) / locals.var_weff_nf), ((((locals.var_t7_dn2 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn2)) + (((locals.var_vbse_dn2 * locals.var_uc_rd22) * locals.var_t9) + (assign62970_e97647 * locals.var_t9_dn2))) / locals.var_weff_nf), ((((locals.var_t7_dn4 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn4)) + (assign62970_e97647 * locals.var_t9_dn4)) / locals.var_weff_nf), ((((locals.var_t7_dn5 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn5)) + (assign62970_e97647 * locals.var_t9_dn5)) / locals.var_weff_nf), ((((locals.var_t7_dn6 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn6)) + (assign62970_e97647 * locals.var_t9_dn6)) / locals.var_weff_nf), ((((locals.var_t7_dn7 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn7)) + (assign62970_e97647 * locals.var_t9_dn7)) / locals.var_weff_nf), ((((locals.var_t7_dn8 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn8)) + (assign62970_e97647 * locals.var_t9_dn8)) / locals.var_weff_nf), ((((locals.var_t7_dn9 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn9)) + (((locals.var_vbse_dn9 * locals.var_uc_rd22) * locals.var_t9) + (assign62970_e97647 * locals.var_t9_dn9))) / locals.var_weff_nf), ((((locals.var_t7_dn10 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn10)) + (assign62970_e97647 * locals.var_t9_dn10)) / locals.var_weff_nf), ((((locals.var_t7_dn11 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn11)) + (assign62970_e97647 * locals.var_t9_dn11)) / locals.var_weff_nf), ((((locals.var_t7_dn14 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn14)) + (assign62970_e97647 * locals.var_t9_dn14)) / locals.var_weff_nf),)
    } else {
        (locals.var_ra, locals.var_ra_dn0, locals.var_ra_dn2, locals.var_ra_dn4, locals.var_ra_dn5, locals.var_ra_dn6, locals.var_ra_dn7, locals.var_ra_dn8, locals.var_ra_dn9, locals.var_ra_dn10, locals.var_ra_dn11, locals.var_ra_dn14,)
    }
};
        locals.var_ra = assign62970_e97654;
        locals.var_ra_dn0 = assign62970_e97654_d_n0;
        locals.var_ra_dn2 = assign62970_e97654_d_n2;
        locals.var_ra_dn4 = assign62970_e97654_d_n4;
        locals.var_ra_dn5 = assign62970_e97654_d_n5;
        locals.var_ra_dn6 = assign62970_e97654_d_n6;
        locals.var_ra_dn7 = assign62970_e97654_d_n7;
        locals.var_ra_dn8 = assign62970_e97654_d_n8;
        locals.var_ra_dn9 = assign62970_e97654_d_n9;
        locals.var_ra_dn10 = assign62970_e97654_d_n10;
        locals.var_ra_dn11 = assign62970_e97654_d_n11;
        locals.var_ra_dn14 = assign62970_e97654_d_n14;
        locals.var_ra_rv = 0.0;

        let (assign62980_e97665, assign62980_e97665_d_n0, assign62980_e97665_d_n2, assign62980_e97665_d_n4, assign62980_e97665_d_n5, assign62980_e97665_d_n6, assign62980_e97665_d_n7, assign62980_e97665_d_n8, assign62980_e97665_d_n9, assign62980_e97665_d_n10, assign62980_e97665_d_n11, assign62980_e97665_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) {
        let assign62980_e97663: f64 = (locals.var_ra * locals.var_ids0);
        (assign62980_e97663, ((locals.var_ra_dn0 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn0)), ((locals.var_ra_dn2 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn2)), ((locals.var_ra_dn4 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn4)), ((locals.var_ra_dn5 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn5)), ((locals.var_ra_dn6 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn6)), ((locals.var_ra_dn7 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn7)), ((locals.var_ra_dn8 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn8)), ((locals.var_ra_dn9 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn9)), ((locals.var_ra_dn10 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn10)), ((locals.var_ra_dn11 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn11)), ((locals.var_ra_dn14 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign62980_e97665;
        locals.var_t0_dn0 = assign62980_e97665_d_n0;
        locals.var_t0_dn2 = assign62980_e97665_d_n2;
        locals.var_t0_dn4 = assign62980_e97665_d_n4;
        locals.var_t0_dn5 = assign62980_e97665_d_n5;
        locals.var_t0_dn6 = assign62980_e97665_d_n6;
        locals.var_t0_dn7 = assign62980_e97665_d_n7;
        locals.var_t0_dn8 = assign62980_e97665_d_n8;
        locals.var_t0_dn9 = assign62980_e97665_d_n9;
        locals.var_t0_dn10 = assign62980_e97665_d_n10;
        locals.var_t0_dn11 = assign62980_e97665_d_n11;
        locals.var_t0_dn14 = assign62980_e97665_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign62990_e97676, assign62990_e97676_d_n0, assign62990_e97676_d_n2, assign62990_e97676_d_n4, assign62990_e97676_d_n5, assign62990_e97676_d_n6, assign62990_e97676_d_n7, assign62990_e97676_d_n8, assign62990_e97676_d_n9, assign62990_e97676_d_n10, assign62990_e97676_d_n11, assign62990_e97676_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) {
        let assign62990_e97674: f64 = (locals.var_vds + 1e-12);
        (assign62990_e97674, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign62990_e97676;
        locals.var_t1_dn0 = assign62990_e97676_d_n0;
        locals.var_t1_dn2 = assign62990_e97676_d_n2;
        locals.var_t1_dn4 = assign62990_e97676_d_n4;
        locals.var_t1_dn5 = assign62990_e97676_d_n5;
        locals.var_t1_dn6 = assign62990_e97676_d_n6;
        locals.var_t1_dn7 = assign62990_e97676_d_n7;
        locals.var_t1_dn8 = assign62990_e97676_d_n8;
        locals.var_t1_dn9 = assign62990_e97676_d_n9;
        locals.var_t1_dn10 = assign62990_e97676_d_n10;
        locals.var_t1_dn11 = assign62990_e97676_d_n11;
        locals.var_t1_dn14 = assign62990_e97676_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign63000_e97687, assign63000_e97687_d_n0, assign63000_e97687_d_n2, assign63000_e97687_d_n4, assign63000_e97687_d_n5, assign63000_e97687_d_n6, assign63000_e97687_d_n7, assign63000_e97687_d_n8, assign63000_e97687_d_n9, assign63000_e97687_d_n10, assign63000_e97687_d_n11, assign63000_e97687_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) {
        let assign63000_e97685: f64 = (1.0 / locals.var_t1);
        (assign63000_e97685, (-(locals.var_t1_dn0 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn2 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn11 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn14 / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign63000_e97687;
        locals.var_t2_dn0 = assign63000_e97687_d_n0;
        locals.var_t2_dn2 = assign63000_e97687_d_n2;
        locals.var_t2_dn4 = assign63000_e97687_d_n4;
        locals.var_t2_dn5 = assign63000_e97687_d_n5;
        locals.var_t2_dn6 = assign63000_e97687_d_n6;
        locals.var_t2_dn7 = assign63000_e97687_d_n7;
        locals.var_t2_dn8 = assign63000_e97687_d_n8;
        locals.var_t2_dn9 = assign63000_e97687_d_n9;
        locals.var_t2_dn10 = assign63000_e97687_d_n10;
        locals.var_t2_dn11 = assign63000_e97687_d_n11;
        locals.var_t2_dn14 = assign63000_e97687_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign63010_e97700, assign63010_e97700_d_n0, assign63010_e97700_d_n2, assign63010_e97700_d_n4, assign63010_e97700_d_n5, assign63010_e97700_d_n6, assign63010_e97700_d_n7, assign63010_e97700_d_n8, assign63010_e97700_d_n9, assign63010_e97700_d_n10, assign63010_e97700_d_n11, assign63010_e97700_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) {
        let assign63010_e97697: f64 = (locals.var_t0 * locals.var_t2);
        let assign63010_e97698: f64 = (1.0 + assign63010_e97697);
        (assign63010_e97698, ((locals.var_t0_dn0 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn0)), ((locals.var_t0_dn2 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn2)), ((locals.var_t0_dn4 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn4)), ((locals.var_t0_dn5 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn5)), ((locals.var_t0_dn6 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn6)), ((locals.var_t0_dn7 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn7)), ((locals.var_t0_dn8 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn8)), ((locals.var_t0_dn9 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn9)), ((locals.var_t0_dn10 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn10)), ((locals.var_t0_dn11 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn11)), ((locals.var_t0_dn14 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign63010_e97700;
        locals.var_t3_dn0 = assign63010_e97700_d_n0;
        locals.var_t3_dn2 = assign63010_e97700_d_n2;
        locals.var_t3_dn4 = assign63010_e97700_d_n4;
        locals.var_t3_dn5 = assign63010_e97700_d_n5;
        locals.var_t3_dn6 = assign63010_e97700_d_n6;
        locals.var_t3_dn7 = assign63010_e97700_d_n7;
        locals.var_t3_dn8 = assign63010_e97700_d_n8;
        locals.var_t3_dn9 = assign63010_e97700_d_n9;
        locals.var_t3_dn10 = assign63010_e97700_d_n10;
        locals.var_t3_dn11 = assign63010_e97700_d_n11;
        locals.var_t3_dn14 = assign63010_e97700_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign63020_e97711, assign63020_e97711_d_n0, assign63020_e97711_d_n2, assign63020_e97711_d_n4, assign63020_e97711_d_n5, assign63020_e97711_d_n6, assign63020_e97711_d_n7, assign63020_e97711_d_n8, assign63020_e97711_d_n9, assign63020_e97711_d_n10, assign63020_e97711_d_n11, assign63020_e97711_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) {
        let assign63020_e97709: f64 = (1.0 / locals.var_t3);
        (assign63020_e97709, (-(locals.var_t3_dn0 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn2 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn14 / (locals.var_t3 * locals.var_t3))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign63020_e97711;
        locals.var_t4_dn0 = assign63020_e97711_d_n0;
        locals.var_t4_dn2 = assign63020_e97711_d_n2;
        locals.var_t4_dn4 = assign63020_e97711_d_n4;
        locals.var_t4_dn5 = assign63020_e97711_d_n5;
        locals.var_t4_dn6 = assign63020_e97711_d_n6;
        locals.var_t4_dn7 = assign63020_e97711_d_n7;
        locals.var_t4_dn8 = assign63020_e97711_d_n8;
        locals.var_t4_dn9 = assign63020_e97711_d_n9;
        locals.var_t4_dn10 = assign63020_e97711_d_n10;
        locals.var_t4_dn11 = assign63020_e97711_d_n11;
        locals.var_t4_dn14 = assign63020_e97711_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign63030_e97722, assign63030_e97722_d_n0, assign63030_e97722_d_n2, assign63030_e97722_d_n4, assign63030_e97722_d_n5, assign63030_e97722_d_n6, assign63030_e97722_d_n7, assign63030_e97722_d_n8, assign63030_e97722_d_n9, assign63030_e97722_d_n10, assign63030_e97722_d_n11, assign63030_e97722_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 != 0.0)) {
        let assign63030_e97720: f64 = (locals.var_ids0 * locals.var_t4);
        (assign63030_e97720, ((locals.var_ids0_dn0 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn0)), ((locals.var_ids0_dn2 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn2)), ((locals.var_ids0_dn4 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn4)), ((locals.var_ids0_dn5 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn5)), ((locals.var_ids0_dn6 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn6)), ((locals.var_ids0_dn7 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn7)), ((locals.var_ids0_dn8 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn8)), ((locals.var_ids0_dn9 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn9)), ((locals.var_ids0_dn10 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn10)), ((locals.var_ids0_dn11 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn11)), ((locals.var_ids0_dn14 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn14)),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn14,)
    }
};
        locals.var_ids = assign63030_e97722;
        locals.var_ids_dn0 = assign63030_e97722_d_n0;
        locals.var_ids_dn2 = assign63030_e97722_d_n2;
        locals.var_ids_dn4 = assign63030_e97722_d_n4;
        locals.var_ids_dn5 = assign63030_e97722_d_n5;
        locals.var_ids_dn6 = assign63030_e97722_d_n6;
        locals.var_ids_dn7 = assign63030_e97722_d_n7;
        locals.var_ids_dn8 = assign63030_e97722_d_n8;
        locals.var_ids_dn9 = assign63030_e97722_d_n9;
        locals.var_ids_dn10 = assign63030_e97722_d_n10;
        locals.var_ids_dn11 = assign63030_e97722_d_n11;
        locals.var_ids_dn14 = assign63030_e97722_d_n14;
        locals.var_ids_rv = 0.0;

        let (assign63040_e97732, assign63040_e97732_d_n0, assign63040_e97732_d_n2, assign63040_e97732_d_n4, assign63040_e97732_d_n5, assign63040_e97732_d_n6, assign63040_e97732_d_n7, assign63040_e97732_d_n8, assign63040_e97732_d_n9, assign63040_e97732_d_n10, assign63040_e97732_d_n11, assign63040_e97732_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 == 0.0)) {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn4, locals.var_ids0_dn5, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn8, locals.var_ids0_dn9, locals.var_ids0_dn10, locals.var_ids0_dn11, locals.var_ids0_dn14,)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn14,)
    }
};
        locals.var_ids = assign63040_e97732;
        locals.var_ids_dn0 = assign63040_e97732_d_n0;
        locals.var_ids_dn2 = assign63040_e97732_d_n2;
        locals.var_ids_dn4 = assign63040_e97732_d_n4;
        locals.var_ids_dn5 = assign63040_e97732_d_n5;
        locals.var_ids_dn6 = assign63040_e97732_d_n6;
        locals.var_ids_dn7 = assign63040_e97732_d_n7;
        locals.var_ids_dn8 = assign63040_e97732_d_n8;
        locals.var_ids_dn9 = assign63040_e97732_d_n9;
        locals.var_ids_dn10 = assign63040_e97732_d_n10;
        locals.var_ids_dn11 = assign63040_e97732_d_n11;
        locals.var_ids_dn14 = assign63040_e97732_d_n14;
        locals.var_ids_rv = 0.0;

        let (assign63050_e97742, assign63050_e97742_d_n0, assign63050_e97742_d_n2, assign63050_e97742_d_n4, assign63050_e97742_d_n5, assign63050_e97742_d_n6, assign63050_e97742_d_n7, assign63050_e97742_d_n8, assign63050_e97742_d_n9, assign63050_e97742_d_n10, assign63050_e97742_d_n11, assign63050_e97742_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1501 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ra, locals.var_ra_dn0, locals.var_ra_dn2, locals.var_ra_dn4, locals.var_ra_dn5, locals.var_ra_dn6, locals.var_ra_dn7, locals.var_ra_dn8, locals.var_ra_dn9, locals.var_ra_dn10, locals.var_ra_dn11, locals.var_ra_dn14,)
    }
};
        locals.var_ra = assign63050_e97742;
        locals.var_ra_dn0 = assign63050_e97742_d_n0;
        locals.var_ra_dn2 = assign63050_e97742_d_n2;
        locals.var_ra_dn4 = assign63050_e97742_d_n4;
        locals.var_ra_dn5 = assign63050_e97742_d_n5;
        locals.var_ra_dn6 = assign63050_e97742_d_n6;
        locals.var_ra_dn7 = assign63050_e97742_d_n7;
        locals.var_ra_dn8 = assign63050_e97742_d_n8;
        locals.var_ra_dn9 = assign63050_e97742_d_n9;
        locals.var_ra_dn10 = assign63050_e97742_d_n10;
        locals.var_ra_dn11 = assign63050_e97742_d_n11;
        locals.var_ra_dn14 = assign63050_e97742_d_n14;
        locals.var_ra_rv = 0.0;

        let assign63060_e97745: f64 = if p.p27 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1505 = assign63060_e97745;
        locals.var_guard1505_rv = 0.0;

        let (assign63070_e97756, assign63070_e97756_d_n0, assign63070_e97756_d_n2, assign63070_e97756_d_n4, assign63070_e97756_d_n5, assign63070_e97756_d_n6, assign63070_e97756_d_n7, assign63070_e97756_d_n8, assign63070_e97756_d_n9, assign63070_e97756_d_n10, assign63070_e97756_d_n11, assign63070_e97756_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63070_e97754: f64 = (1.034943e-10 * locals.var_cox_inv);
        (assign63070_e97754, (1.034943e-10 * locals.var_cox_inv_dn0), (1.034943e-10 * locals.var_cox_inv_dn2), (1.034943e-10 * locals.var_cox_inv_dn4), (1.034943e-10 * locals.var_cox_inv_dn5), (1.034943e-10 * locals.var_cox_inv_dn6), (1.034943e-10 * locals.var_cox_inv_dn7), (1.034943e-10 * locals.var_cox_inv_dn8), (1.034943e-10 * locals.var_cox_inv_dn9), (1.034943e-10 * locals.var_cox_inv_dn10), (1.034943e-10 * locals.var_cox_inv_dn11), (1.034943e-10 * locals.var_cox_inv_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63070_e97756;
        locals.var_t1_dn0 = assign63070_e97756_d_n0;
        locals.var_t1_dn2 = assign63070_e97756_d_n2;
        locals.var_t1_dn4 = assign63070_e97756_d_n4;
        locals.var_t1_dn5 = assign63070_e97756_d_n5;
        locals.var_t1_dn6 = assign63070_e97756_d_n6;
        locals.var_t1_dn7 = assign63070_e97756_d_n7;
        locals.var_t1_dn8 = assign63070_e97756_d_n8;
        locals.var_t1_dn9 = assign63070_e97756_d_n9;
        locals.var_t1_dn10 = assign63070_e97756_d_n10;
        locals.var_t1_dn11 = assign63070_e97756_d_n11;
        locals.var_t1_dn14 = assign63070_e97756_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign63080_e97765, assign63080_e97765_d_n0, assign63080_e97765_d_n2, assign63080_e97765_d_n4, assign63080_e97765_d_n5, assign63080_e97765_d_n6, assign63080_e97765_d_n7, assign63080_e97765_d_n8, assign63080_e97765_d_n9, assign63080_e97765_d_n10, assign63080_e97765_d_n11, assign63080_e97765_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        (locals.var_wdpl, locals.var_wdpl_dn0, locals.var_wdpl_dn2, locals.var_wdpl_dn4, locals.var_wdpl_dn5, locals.var_wdpl_dn6, locals.var_wdpl_dn7, locals.var_wdpl_dn8, locals.var_wdpl_dn9, locals.var_wdpl_dn10, locals.var_wdpl_dn11, locals.var_wdpl_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign63080_e97765;
        locals.var_t2_dn0 = assign63080_e97765_d_n0;
        locals.var_t2_dn2 = assign63080_e97765_d_n2;
        locals.var_t2_dn4 = assign63080_e97765_d_n4;
        locals.var_t2_dn5 = assign63080_e97765_d_n5;
        locals.var_t2_dn6 = assign63080_e97765_d_n6;
        locals.var_t2_dn7 = assign63080_e97765_d_n7;
        locals.var_t2_dn8 = assign63080_e97765_d_n8;
        locals.var_t2_dn9 = assign63080_e97765_d_n9;
        locals.var_t2_dn10 = assign63080_e97765_d_n10;
        locals.var_t2_dn11 = assign63080_e97765_d_n11;
        locals.var_t2_dn14 = assign63080_e97765_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign63090_e97776, assign63090_e97776_d_n0, assign63090_e97776_d_n2, assign63090_e97776_d_n4, assign63090_e97776_d_n5, assign63090_e97776_d_n6, assign63090_e97776_d_n7, assign63090_e97776_d_n8, assign63090_e97776_d_n9, assign63090_e97776_d_n10, assign63090_e97776_d_n11, assign63090_e97776_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63090_e97774: f64 = (locals.var_lgatesm - p.p139);
        (assign63090_e97774, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign63090_e97776;
        locals.var_t3_dn0 = assign63090_e97776_d_n0;
        locals.var_t3_dn2 = assign63090_e97776_d_n2;
        locals.var_t3_dn4 = assign63090_e97776_d_n4;
        locals.var_t3_dn5 = assign63090_e97776_d_n5;
        locals.var_t3_dn6 = assign63090_e97776_d_n6;
        locals.var_t3_dn7 = assign63090_e97776_d_n7;
        locals.var_t3_dn8 = assign63090_e97776_d_n8;
        locals.var_t3_dn9 = assign63090_e97776_d_n9;
        locals.var_t3_dn10 = assign63090_e97776_d_n10;
        locals.var_t3_dn11 = assign63090_e97776_d_n11;
        locals.var_t3_dn14 = assign63090_e97776_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign63100_e97789, assign63100_e97789_d_n0, assign63100_e97789_d_n2, assign63100_e97789_d_n4, assign63100_e97789_d_n5, assign63100_e97789_d_n6, assign63100_e97789_d_n7, assign63100_e97789_d_n8, assign63100_e97789_d_n9, assign63100_e97789_d_n10, assign63100_e97789_d_n11, assign63100_e97789_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63100_e97786: f64 = (locals.var_t3 * locals.var_t3);
        let assign63100_e97787: f64 = (1.0 / assign63100_e97786);
        (assign63100_e97787, (-(((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0)) / (assign63100_e97786 * assign63100_e97786))), (-(((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2)) / (assign63100_e97786 * assign63100_e97786))), (-(((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4)) / (assign63100_e97786 * assign63100_e97786))), (-(((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5)) / (assign63100_e97786 * assign63100_e97786))), (-(((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6)) / (assign63100_e97786 * assign63100_e97786))), (-(((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7)) / (assign63100_e97786 * assign63100_e97786))), (-(((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8)) / (assign63100_e97786 * assign63100_e97786))), (-(((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9)) / (assign63100_e97786 * assign63100_e97786))), (-(((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10)) / (assign63100_e97786 * assign63100_e97786))), (-(((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11)) / (assign63100_e97786 * assign63100_e97786))), (-(((locals.var_t3_dn14 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn14)) / (assign63100_e97786 * assign63100_e97786))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign63100_e97789;
        locals.var_t4_dn0 = assign63100_e97789_d_n0;
        locals.var_t4_dn2 = assign63100_e97789_d_n2;
        locals.var_t4_dn4 = assign63100_e97789_d_n4;
        locals.var_t4_dn5 = assign63100_e97789_d_n5;
        locals.var_t4_dn6 = assign63100_e97789_d_n6;
        locals.var_t4_dn7 = assign63100_e97789_d_n7;
        locals.var_t4_dn8 = assign63100_e97789_d_n8;
        locals.var_t4_dn9 = assign63100_e97789_d_n9;
        locals.var_t4_dn10 = assign63100_e97789_d_n10;
        locals.var_t4_dn11 = assign63100_e97789_d_n11;
        locals.var_t4_dn14 = assign63100_e97789_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign63110_e97808, assign63110_e97808_d_n0, assign63110_e97808_d_n2, assign63110_e97808_d_n4, assign63110_e97808_d_n5, assign63110_e97808_d_n6, assign63110_e97808_d_n7, assign63110_e97808_d_n8, assign63110_e97808_d_n9, assign63110_e97808_d_n10, assign63110_e97808_d_n11, assign63110_e97808_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63110_e97799: f64 = (p.p137 - locals.var_pb20b);
        let assign63110_e97800: f64 = (2.0 * assign63110_e97799);
        let assign63110_e97802: f64 = (assign63110_e97800 * locals.var_t1);
        let assign63110_e97804: f64 = (assign63110_e97802 * locals.var_t2);
        let assign63110_e97806: f64 = (assign63110_e97804 * locals.var_t4);
        (assign63110_e97806, (((((((2.0 * (-locals.var_pb20b_dn0)) * locals.var_t1) + (assign63110_e97800 * locals.var_t1_dn0)) * locals.var_t2) + (assign63110_e97802 * locals.var_t2_dn0)) * locals.var_t4) + (assign63110_e97804 * locals.var_t4_dn0)), (((((((2.0 * (-locals.var_pb20b_dn2)) * locals.var_t1) + (assign63110_e97800 * locals.var_t1_dn2)) * locals.var_t2) + (assign63110_e97802 * locals.var_t2_dn2)) * locals.var_t4) + (assign63110_e97804 * locals.var_t4_dn2)), (((((((2.0 * (-locals.var_pb20b_dn4)) * locals.var_t1) + (assign63110_e97800 * locals.var_t1_dn4)) * locals.var_t2) + (assign63110_e97802 * locals.var_t2_dn4)) * locals.var_t4) + (assign63110_e97804 * locals.var_t4_dn4)), (((((((2.0 * (-locals.var_pb20b_dn5)) * locals.var_t1) + (assign63110_e97800 * locals.var_t1_dn5)) * locals.var_t2) + (assign63110_e97802 * locals.var_t2_dn5)) * locals.var_t4) + (assign63110_e97804 * locals.var_t4_dn5)), (((((((2.0 * (-locals.var_pb20b_dn6)) * locals.var_t1) + (assign63110_e97800 * locals.var_t1_dn6)) * locals.var_t2) + (assign63110_e97802 * locals.var_t2_dn6)) * locals.var_t4) + (assign63110_e97804 * locals.var_t4_dn6)), (((((((2.0 * (-locals.var_pb20b_dn7)) * locals.var_t1) + (assign63110_e97800 * locals.var_t1_dn7)) * locals.var_t2) + (assign63110_e97802 * locals.var_t2_dn7)) * locals.var_t4) + (assign63110_e97804 * locals.var_t4_dn7)), (((((((2.0 * (-locals.var_pb20b_dn8)) * locals.var_t1) + (assign63110_e97800 * locals.var_t1_dn8)) * locals.var_t2) + (assign63110_e97802 * locals.var_t2_dn8)) * locals.var_t4) + (assign63110_e97804 * locals.var_t4_dn8)), (((((((2.0 * (-locals.var_pb20b_dn9)) * locals.var_t1) + (assign63110_e97800 * locals.var_t1_dn9)) * locals.var_t2) + (assign63110_e97802 * locals.var_t2_dn9)) * locals.var_t4) + (assign63110_e97804 * locals.var_t4_dn9)), (((((((2.0 * (-locals.var_pb20b_dn10)) * locals.var_t1) + (assign63110_e97800 * locals.var_t1_dn10)) * locals.var_t2) + (assign63110_e97802 * locals.var_t2_dn10)) * locals.var_t4) + (assign63110_e97804 * locals.var_t4_dn10)), (((((((2.0 * (-locals.var_pb20b_dn11)) * locals.var_t1) + (assign63110_e97800 * locals.var_t1_dn11)) * locals.var_t2) + (assign63110_e97802 * locals.var_t2_dn11)) * locals.var_t4) + (assign63110_e97804 * locals.var_t4_dn11)), (((((((2.0 * (-locals.var_pb20b_dn14)) * locals.var_t1) + (assign63110_e97800 * locals.var_t1_dn14)) * locals.var_t2) + (assign63110_e97802 * locals.var_t2_dn14)) * locals.var_t4) + (assign63110_e97804 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign63110_e97808;
        locals.var_t5_dn0 = assign63110_e97808_d_n0;
        locals.var_t5_dn2 = assign63110_e97808_d_n2;
        locals.var_t5_dn4 = assign63110_e97808_d_n4;
        locals.var_t5_dn5 = assign63110_e97808_d_n5;
        locals.var_t5_dn6 = assign63110_e97808_d_n6;
        locals.var_t5_dn7 = assign63110_e97808_d_n7;
        locals.var_t5_dn8 = assign63110_e97808_d_n8;
        locals.var_t5_dn9 = assign63110_e97808_d_n9;
        locals.var_t5_dn10 = assign63110_e97808_d_n10;
        locals.var_t5_dn11 = assign63110_e97808_d_n11;
        locals.var_t5_dn14 = assign63110_e97808_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign63120_e97819, assign63120_e97819_d_n0, assign63120_e97819_d_n2, assign63120_e97819_d_n4, assign63120_e97819_d_n5, assign63120_e97819_d_n6, assign63120_e97819_d_n7, assign63120_e97819_d_n8, assign63120_e97819_d_n9, assign63120_e97819_d_n10, assign63120_e97819_d_n11, assign63120_e97819_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63120_e97817: f64 = (locals.var_t5 * locals.var_sqrt_pbsum);
        (assign63120_e97817, ((locals.var_t5_dn0 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn0)), ((locals.var_t5_dn2 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn2)), ((locals.var_t5_dn4 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn4)), ((locals.var_t5_dn5 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn5)), ((locals.var_t5_dn6 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn6)), ((locals.var_t5_dn7 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn7)), ((locals.var_t5_dn8 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn8)), ((locals.var_t5_dn9 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn9)), ((locals.var_t5_dn10 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn10)), ((locals.var_t5_dn11 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn11)), ((locals.var_t5_dn14 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn14)),)
    } else {
        (locals.var_dvth0, locals.var_dvth0_dn0, locals.var_dvth0_dn2, locals.var_dvth0_dn4, locals.var_dvth0_dn5, locals.var_dvth0_dn6, locals.var_dvth0_dn7, locals.var_dvth0_dn8, locals.var_dvth0_dn9, locals.var_dvth0_dn10, locals.var_dvth0_dn11, locals.var_dvth0_dn14,)
    }
};
        locals.var_dvth0 = assign63120_e97819;
        locals.var_dvth0_dn0 = assign63120_e97819_d_n0;
        locals.var_dvth0_dn2 = assign63120_e97819_d_n2;
        locals.var_dvth0_dn4 = assign63120_e97819_d_n4;
        locals.var_dvth0_dn5 = assign63120_e97819_d_n5;
        locals.var_dvth0_dn6 = assign63120_e97819_d_n6;
        locals.var_dvth0_dn7 = assign63120_e97819_d_n7;
        locals.var_dvth0_dn8 = assign63120_e97819_d_n8;
        locals.var_dvth0_dn9 = assign63120_e97819_d_n9;
        locals.var_dvth0_dn10 = assign63120_e97819_d_n10;
        locals.var_dvth0_dn11 = assign63120_e97819_d_n11;
        locals.var_dvth0_dn14 = assign63120_e97819_d_n14;
        locals.var_dvth0_rv = 0.0;

        let (assign63130_e97832, assign63130_e97832_d_n0, assign63130_e97832_d_n2, assign63130_e97832_d_n4, assign63130_e97832_d_n5, assign63130_e97832_d_n6, assign63130_e97832_d_n7, assign63130_e97832_d_n8, assign63130_e97832_d_n9, assign63130_e97832_d_n10, assign63130_e97832_d_n11, assign63130_e97832_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63130_e97828: f64 = (locals.var_t5 * 0.5);
        let assign63130_e97830: f64 = (assign63130_e97828 / locals.var_sqrt_pbsum);
        (assign63130_e97830, ((((locals.var_t5_dn0 * 0.5) * locals.var_sqrt_pbsum) - (assign63130_e97828 * locals.var_sqrt_pbsum_dn0)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn2 * 0.5) * locals.var_sqrt_pbsum) - (assign63130_e97828 * locals.var_sqrt_pbsum_dn2)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn4 * 0.5) * locals.var_sqrt_pbsum) - (assign63130_e97828 * locals.var_sqrt_pbsum_dn4)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn5 * 0.5) * locals.var_sqrt_pbsum) - (assign63130_e97828 * locals.var_sqrt_pbsum_dn5)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn6 * 0.5) * locals.var_sqrt_pbsum) - (assign63130_e97828 * locals.var_sqrt_pbsum_dn6)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn7 * 0.5) * locals.var_sqrt_pbsum) - (assign63130_e97828 * locals.var_sqrt_pbsum_dn7)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn8 * 0.5) * locals.var_sqrt_pbsum) - (assign63130_e97828 * locals.var_sqrt_pbsum_dn8)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn9 * 0.5) * locals.var_sqrt_pbsum) - (assign63130_e97828 * locals.var_sqrt_pbsum_dn9)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn10 * 0.5) * locals.var_sqrt_pbsum) - (assign63130_e97828 * locals.var_sqrt_pbsum_dn10)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn11 * 0.5) * locals.var_sqrt_pbsum) - (assign63130_e97828 * locals.var_sqrt_pbsum_dn11)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((locals.var_t5_dn14 * 0.5) * locals.var_sqrt_pbsum) - (assign63130_e97828 * locals.var_sqrt_pbsum_dn14)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign63130_e97832;
        locals.var_t6_dn0 = assign63130_e97832_d_n0;
        locals.var_t6_dn2 = assign63130_e97832_d_n2;
        locals.var_t6_dn4 = assign63130_e97832_d_n4;
        locals.var_t6_dn5 = assign63130_e97832_d_n5;
        locals.var_t6_dn6 = assign63130_e97832_d_n6;
        locals.var_t6_dn7 = assign63130_e97832_d_n7;
        locals.var_t6_dn8 = assign63130_e97832_d_n8;
        locals.var_t6_dn9 = assign63130_e97832_d_n9;
        locals.var_t6_dn10 = assign63130_e97832_d_n10;
        locals.var_t6_dn11 = assign63130_e97832_d_n11;
        locals.var_t6_dn14 = assign63130_e97832_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign63140_e97853, assign63140_e97853_d_n0, assign63140_e97853_d_n2, assign63140_e97853_d_n4, assign63140_e97853_d_n5, assign63140_e97853_d_n6, assign63140_e97853_d_n7, assign63140_e97853_d_n8, assign63140_e97853_d_n9, assign63140_e97853_d_n10, assign63140_e97853_d_n11, assign63140_e97853_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63140_e97842: f64 = (p.p137 - locals.var_pb20b);
        let assign63140_e97843: f64 = (2.0 * assign63140_e97842);
        let assign63140_e97845: f64 = (assign63140_e97843 * 1.034943e-10);
        let assign63140_e97847: f64 = (assign63140_e97845 * locals.var_t2);
        let assign63140_e97849: f64 = (assign63140_e97847 * locals.var_t4);
        let assign63140_e97851: f64 = (assign63140_e97849 * locals.var_sqrt_pbsum);
        (assign63140_e97851, ((((((((2.0 * (-locals.var_pb20b_dn0)) * 1.034943e-10) * locals.var_t2) + (assign63140_e97845 * locals.var_t2_dn0)) * locals.var_t4) + (assign63140_e97847 * locals.var_t4_dn0)) * locals.var_sqrt_pbsum) + (assign63140_e97849 * locals.var_sqrt_pbsum_dn0)), ((((((((2.0 * (-locals.var_pb20b_dn2)) * 1.034943e-10) * locals.var_t2) + (assign63140_e97845 * locals.var_t2_dn2)) * locals.var_t4) + (assign63140_e97847 * locals.var_t4_dn2)) * locals.var_sqrt_pbsum) + (assign63140_e97849 * locals.var_sqrt_pbsum_dn2)), ((((((((2.0 * (-locals.var_pb20b_dn4)) * 1.034943e-10) * locals.var_t2) + (assign63140_e97845 * locals.var_t2_dn4)) * locals.var_t4) + (assign63140_e97847 * locals.var_t4_dn4)) * locals.var_sqrt_pbsum) + (assign63140_e97849 * locals.var_sqrt_pbsum_dn4)), ((((((((2.0 * (-locals.var_pb20b_dn5)) * 1.034943e-10) * locals.var_t2) + (assign63140_e97845 * locals.var_t2_dn5)) * locals.var_t4) + (assign63140_e97847 * locals.var_t4_dn5)) * locals.var_sqrt_pbsum) + (assign63140_e97849 * locals.var_sqrt_pbsum_dn5)), ((((((((2.0 * (-locals.var_pb20b_dn6)) * 1.034943e-10) * locals.var_t2) + (assign63140_e97845 * locals.var_t2_dn6)) * locals.var_t4) + (assign63140_e97847 * locals.var_t4_dn6)) * locals.var_sqrt_pbsum) + (assign63140_e97849 * locals.var_sqrt_pbsum_dn6)), ((((((((2.0 * (-locals.var_pb20b_dn7)) * 1.034943e-10) * locals.var_t2) + (assign63140_e97845 * locals.var_t2_dn7)) * locals.var_t4) + (assign63140_e97847 * locals.var_t4_dn7)) * locals.var_sqrt_pbsum) + (assign63140_e97849 * locals.var_sqrt_pbsum_dn7)), ((((((((2.0 * (-locals.var_pb20b_dn8)) * 1.034943e-10) * locals.var_t2) + (assign63140_e97845 * locals.var_t2_dn8)) * locals.var_t4) + (assign63140_e97847 * locals.var_t4_dn8)) * locals.var_sqrt_pbsum) + (assign63140_e97849 * locals.var_sqrt_pbsum_dn8)), ((((((((2.0 * (-locals.var_pb20b_dn9)) * 1.034943e-10) * locals.var_t2) + (assign63140_e97845 * locals.var_t2_dn9)) * locals.var_t4) + (assign63140_e97847 * locals.var_t4_dn9)) * locals.var_sqrt_pbsum) + (assign63140_e97849 * locals.var_sqrt_pbsum_dn9)), ((((((((2.0 * (-locals.var_pb20b_dn10)) * 1.034943e-10) * locals.var_t2) + (assign63140_e97845 * locals.var_t2_dn10)) * locals.var_t4) + (assign63140_e97847 * locals.var_t4_dn10)) * locals.var_sqrt_pbsum) + (assign63140_e97849 * locals.var_sqrt_pbsum_dn10)), ((((((((2.0 * (-locals.var_pb20b_dn11)) * 1.034943e-10) * locals.var_t2) + (assign63140_e97845 * locals.var_t2_dn11)) * locals.var_t4) + (assign63140_e97847 * locals.var_t4_dn11)) * locals.var_sqrt_pbsum) + (assign63140_e97849 * locals.var_sqrt_pbsum_dn11)), ((((((((2.0 * (-locals.var_pb20b_dn14)) * 1.034943e-10) * locals.var_t2) + (assign63140_e97845 * locals.var_t2_dn14)) * locals.var_t4) + (assign63140_e97847 * locals.var_t4_dn14)) * locals.var_sqrt_pbsum) + (assign63140_e97849 * locals.var_sqrt_pbsum_dn14)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign63140_e97853;
        locals.var_t7_dn0 = assign63140_e97853_d_n0;
        locals.var_t7_dn2 = assign63140_e97853_d_n2;
        locals.var_t7_dn4 = assign63140_e97853_d_n4;
        locals.var_t7_dn5 = assign63140_e97853_d_n5;
        locals.var_t7_dn6 = assign63140_e97853_d_n6;
        locals.var_t7_dn7 = assign63140_e97853_d_n7;
        locals.var_t7_dn8 = assign63140_e97853_d_n8;
        locals.var_t7_dn9 = assign63140_e97853_d_n9;
        locals.var_t7_dn10 = assign63140_e97853_d_n10;
        locals.var_t7_dn11 = assign63140_e97853_d_n11;
        locals.var_t7_dn14 = assign63140_e97853_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign63150_e97871, assign63150_e97871_d_n0, assign63150_e97871_d_n2, assign63150_e97871_d_n4, assign63150_e97871_d_n5, assign63150_e97871_d_n6, assign63150_e97871_d_n7, assign63150_e97871_d_n8, assign63150_e97871_d_n9, assign63150_e97871_d_n10, assign63150_e97871_d_n11, assign63150_e97871_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63150_e97861: f64 = (-2.0);
        let assign63150_e97863: f64 = (assign63150_e97861 * locals.var_t1);
        let assign63150_e97865: f64 = (assign63150_e97863 * locals.var_t2);
        let assign63150_e97867: f64 = (assign63150_e97865 * locals.var_t4);
        let assign63150_e97869: f64 = (assign63150_e97867 * locals.var_sqrt_pbsum);
        (assign63150_e97869, (((((((assign63150_e97861 * locals.var_t1_dn0) * locals.var_t2) + (assign63150_e97863 * locals.var_t2_dn0)) * locals.var_t4) + (assign63150_e97865 * locals.var_t4_dn0)) * locals.var_sqrt_pbsum) + (assign63150_e97867 * locals.var_sqrt_pbsum_dn0)), (((((((assign63150_e97861 * locals.var_t1_dn2) * locals.var_t2) + (assign63150_e97863 * locals.var_t2_dn2)) * locals.var_t4) + (assign63150_e97865 * locals.var_t4_dn2)) * locals.var_sqrt_pbsum) + (assign63150_e97867 * locals.var_sqrt_pbsum_dn2)), (((((((assign63150_e97861 * locals.var_t1_dn4) * locals.var_t2) + (assign63150_e97863 * locals.var_t2_dn4)) * locals.var_t4) + (assign63150_e97865 * locals.var_t4_dn4)) * locals.var_sqrt_pbsum) + (assign63150_e97867 * locals.var_sqrt_pbsum_dn4)), (((((((assign63150_e97861 * locals.var_t1_dn5) * locals.var_t2) + (assign63150_e97863 * locals.var_t2_dn5)) * locals.var_t4) + (assign63150_e97865 * locals.var_t4_dn5)) * locals.var_sqrt_pbsum) + (assign63150_e97867 * locals.var_sqrt_pbsum_dn5)), (((((((assign63150_e97861 * locals.var_t1_dn6) * locals.var_t2) + (assign63150_e97863 * locals.var_t2_dn6)) * locals.var_t4) + (assign63150_e97865 * locals.var_t4_dn6)) * locals.var_sqrt_pbsum) + (assign63150_e97867 * locals.var_sqrt_pbsum_dn6)), (((((((assign63150_e97861 * locals.var_t1_dn7) * locals.var_t2) + (assign63150_e97863 * locals.var_t2_dn7)) * locals.var_t4) + (assign63150_e97865 * locals.var_t4_dn7)) * locals.var_sqrt_pbsum) + (assign63150_e97867 * locals.var_sqrt_pbsum_dn7)), (((((((assign63150_e97861 * locals.var_t1_dn8) * locals.var_t2) + (assign63150_e97863 * locals.var_t2_dn8)) * locals.var_t4) + (assign63150_e97865 * locals.var_t4_dn8)) * locals.var_sqrt_pbsum) + (assign63150_e97867 * locals.var_sqrt_pbsum_dn8)), (((((((assign63150_e97861 * locals.var_t1_dn9) * locals.var_t2) + (assign63150_e97863 * locals.var_t2_dn9)) * locals.var_t4) + (assign63150_e97865 * locals.var_t4_dn9)) * locals.var_sqrt_pbsum) + (assign63150_e97867 * locals.var_sqrt_pbsum_dn9)), (((((((assign63150_e97861 * locals.var_t1_dn10) * locals.var_t2) + (assign63150_e97863 * locals.var_t2_dn10)) * locals.var_t4) + (assign63150_e97865 * locals.var_t4_dn10)) * locals.var_sqrt_pbsum) + (assign63150_e97867 * locals.var_sqrt_pbsum_dn10)), (((((((assign63150_e97861 * locals.var_t1_dn11) * locals.var_t2) + (assign63150_e97863 * locals.var_t2_dn11)) * locals.var_t4) + (assign63150_e97865 * locals.var_t4_dn11)) * locals.var_sqrt_pbsum) + (assign63150_e97867 * locals.var_sqrt_pbsum_dn11)), (((((((assign63150_e97861 * locals.var_t1_dn14) * locals.var_t2) + (assign63150_e97863 * locals.var_t2_dn14)) * locals.var_t4) + (assign63150_e97865 * locals.var_t4_dn14)) * locals.var_sqrt_pbsum) + (assign63150_e97867 * locals.var_sqrt_pbsum_dn14)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign63150_e97871;
        locals.var_t8_dn0 = assign63150_e97871_d_n0;
        locals.var_t8_dn2 = assign63150_e97871_d_n2;
        locals.var_t8_dn4 = assign63150_e97871_d_n4;
        locals.var_t8_dn5 = assign63150_e97871_d_n5;
        locals.var_t8_dn6 = assign63150_e97871_d_n6;
        locals.var_t8_dn7 = assign63150_e97871_d_n7;
        locals.var_t8_dn8 = assign63150_e97871_d_n8;
        locals.var_t8_dn9 = assign63150_e97871_d_n9;
        locals.var_t8_dn10 = assign63150_e97871_d_n10;
        locals.var_t8_dn11 = assign63150_e97871_d_n11;
        locals.var_t8_dn14 = assign63150_e97871_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign63160_e97880, assign63160_e97880_d_n0, assign63160_e97880_d_n2, assign63160_e97880_d_n4, assign63160_e97880_d_n5, assign63160_e97880_d_n6, assign63160_e97880_d_n7, assign63160_e97880_d_n8, assign63160_e97880_d_n9, assign63160_e97880_d_n10, assign63160_e97880_d_n11, assign63160_e97880_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        (locals.var_uc_scsti1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign63160_e97880;
        locals.var_t4_dn0 = assign63160_e97880_d_n0;
        locals.var_t4_dn2 = assign63160_e97880_d_n2;
        locals.var_t4_dn4 = assign63160_e97880_d_n4;
        locals.var_t4_dn5 = assign63160_e97880_d_n5;
        locals.var_t4_dn6 = assign63160_e97880_d_n6;
        locals.var_t4_dn7 = assign63160_e97880_d_n7;
        locals.var_t4_dn8 = assign63160_e97880_d_n8;
        locals.var_t4_dn9 = assign63160_e97880_d_n9;
        locals.var_t4_dn10 = assign63160_e97880_d_n10;
        locals.var_t4_dn11 = assign63160_e97880_d_n11;
        locals.var_t4_dn14 = assign63160_e97880_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign63170_e97889, assign63170_e97889_d_n0, assign63170_e97889_d_n2, assign63170_e97889_d_n4, assign63170_e97889_d_n5, assign63170_e97889_d_n6, assign63170_e97889_d_n7, assign63170_e97889_d_n8, assign63170_e97889_d_n9, assign63170_e97889_d_n10, assign63170_e97889_d_n11, assign63170_e97889_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        (locals.var_uc_scsti2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign63170_e97889;
        locals.var_t6_dn0 = assign63170_e97889_d_n0;
        locals.var_t6_dn2 = assign63170_e97889_d_n2;
        locals.var_t6_dn4 = assign63170_e97889_d_n4;
        locals.var_t6_dn5 = assign63170_e97889_d_n5;
        locals.var_t6_dn6 = assign63170_e97889_d_n6;
        locals.var_t6_dn7 = assign63170_e97889_d_n7;
        locals.var_t6_dn8 = assign63170_e97889_d_n8;
        locals.var_t6_dn9 = assign63170_e97889_d_n9;
        locals.var_t6_dn10 = assign63170_e97889_d_n10;
        locals.var_t6_dn11 = assign63170_e97889_d_n11;
        locals.var_t6_dn14 = assign63170_e97889_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign63180_e97902, assign63180_e97902_d_n0, assign63180_e97902_d_n2, assign63180_e97902_d_n4, assign63180_e97902_d_n5, assign63180_e97902_d_n6, assign63180_e97902_d_n7, assign63180_e97902_d_n8, assign63180_e97902_d_n9, assign63180_e97902_d_n10, assign63180_e97902_d_n11, assign63180_e97902_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63180_e97899: f64 = (locals.var_t6 * locals.var_vdsz__blk441);
        let assign63180_e97900: f64 = (locals.var_t4 + assign63180_e97899);
        (assign63180_e97900, (locals.var_t4_dn0 + ((locals.var_t6_dn0 * locals.var_vdsz__blk441) + (locals.var_t6 * locals.var_vdsz__blk441_dn0))), (locals.var_t4_dn2 + ((locals.var_t6_dn2 * locals.var_vdsz__blk441) + (locals.var_t6 * locals.var_vdsz__blk441_dn2))), (locals.var_t4_dn4 + ((locals.var_t6_dn4 * locals.var_vdsz__blk441) + (locals.var_t6 * locals.var_vdsz__blk441_dn4))), (locals.var_t4_dn5 + ((locals.var_t6_dn5 * locals.var_vdsz__blk441) + (locals.var_t6 * locals.var_vdsz__blk441_dn5))), (locals.var_t4_dn6 + ((locals.var_t6_dn6 * locals.var_vdsz__blk441) + (locals.var_t6 * locals.var_vdsz__blk441_dn6))), (locals.var_t4_dn7 + ((locals.var_t6_dn7 * locals.var_vdsz__blk441) + (locals.var_t6 * locals.var_vdsz__blk441_dn7))), (locals.var_t4_dn8 + ((locals.var_t6_dn8 * locals.var_vdsz__blk441) + (locals.var_t6 * locals.var_vdsz__blk441_dn8))), (locals.var_t4_dn9 + ((locals.var_t6_dn9 * locals.var_vdsz__blk441) + (locals.var_t6 * locals.var_vdsz__blk441_dn9))), (locals.var_t4_dn10 + ((locals.var_t6_dn10 * locals.var_vdsz__blk441) + (locals.var_t6 * locals.var_vdsz__blk441_dn10))), (locals.var_t4_dn11 + ((locals.var_t6_dn11 * locals.var_vdsz__blk441) + (locals.var_t6 * locals.var_vdsz__blk441_dn11))), (locals.var_t4_dn14 + ((locals.var_t6_dn14 * locals.var_vdsz__blk441) + (locals.var_t6 * locals.var_vdsz__blk441_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63180_e97902;
        locals.var_t1_dn0 = assign63180_e97902_d_n0;
        locals.var_t1_dn2 = assign63180_e97902_d_n2;
        locals.var_t1_dn4 = assign63180_e97902_d_n4;
        locals.var_t1_dn5 = assign63180_e97902_d_n5;
        locals.var_t1_dn6 = assign63180_e97902_d_n6;
        locals.var_t1_dn7 = assign63180_e97902_d_n7;
        locals.var_t1_dn8 = assign63180_e97902_d_n8;
        locals.var_t1_dn9 = assign63180_e97902_d_n9;
        locals.var_t1_dn10 = assign63180_e97902_d_n10;
        locals.var_t1_dn11 = assign63180_e97902_d_n11;
        locals.var_t1_dn14 = assign63180_e97902_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_236(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign63190_e97913, assign63190_e97913_d_n0, assign63190_e97913_d_n2, assign63190_e97913_d_n4, assign63190_e97913_d_n5, assign63190_e97913_d_n6, assign63190_e97913_d_n7, assign63190_e97913_d_n8, assign63190_e97913_d_n9, assign63190_e97913_d_n10, assign63190_e97913_d_n11, assign63190_e97913_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63190_e97911: f64 = (locals.var_dvth0 * locals.var_t1);
        (assign63190_e97911, ((locals.var_dvth0_dn0 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn0)), ((locals.var_dvth0_dn2 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn2)), ((locals.var_dvth0_dn4 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn4)), ((locals.var_dvth0_dn5 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn5)), ((locals.var_dvth0_dn6 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn6)), ((locals.var_dvth0_dn7 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn7)), ((locals.var_dvth0_dn8 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn8)), ((locals.var_dvth0_dn9 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn9)), ((locals.var_dvth0_dn10 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn10)), ((locals.var_dvth0_dn11 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn11)), ((locals.var_dvth0_dn14 * locals.var_t1) + (locals.var_dvth0 * locals.var_t1_dn14)),)
    } else {
        (locals.var_dvthscsti, locals.var_dvthscsti_dn0, locals.var_dvthscsti_dn2, locals.var_dvthscsti_dn4, locals.var_dvthscsti_dn5, locals.var_dvthscsti_dn6, locals.var_dvthscsti_dn7, locals.var_dvthscsti_dn8, locals.var_dvthscsti_dn9, locals.var_dvthscsti_dn10, locals.var_dvthscsti_dn11, locals.var_dvthscsti_dn14,)
    }
};
        locals.var_dvthscsti = assign63190_e97913;
        locals.var_dvthscsti_dn0 = assign63190_e97913_d_n0;
        locals.var_dvthscsti_dn2 = assign63190_e97913_d_n2;
        locals.var_dvthscsti_dn4 = assign63190_e97913_d_n4;
        locals.var_dvthscsti_dn5 = assign63190_e97913_d_n5;
        locals.var_dvthscsti_dn6 = assign63190_e97913_d_n6;
        locals.var_dvthscsti_dn7 = assign63190_e97913_d_n7;
        locals.var_dvthscsti_dn8 = assign63190_e97913_d_n8;
        locals.var_dvthscsti_dn9 = assign63190_e97913_d_n9;
        locals.var_dvthscsti_dn10 = assign63190_e97913_d_n10;
        locals.var_dvthscsti_dn11 = assign63190_e97913_d_n11;
        locals.var_dvthscsti_dn14 = assign63190_e97913_d_n14;
        locals.var_dvthscsti_rv = 0.0;

        let (assign63200_e97926, assign63200_e97926_d_n0, assign63200_e97926_d_n2, assign63200_e97926_d_n4, assign63200_e97926_d_n5, assign63200_e97926_d_n6, assign63200_e97926_d_n7, assign63200_e97926_d_n8, assign63200_e97926_d_n9, assign63200_e97926_d_n10, assign63200_e97926_d_n11, assign63200_e97926_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63200_e97923: f64 = (p.p213 * locals.var_vds);
        let assign63200_e97924: f64 = (locals.var_uc_vthsti - assign63200_e97923);
        (assign63200_e97924, (-(p.p213 * locals.var_vds_dn0)), (-(p.p213 * locals.var_vds_dn2)), (-(p.p213 * locals.var_vds_dn4)), (-(p.p213 * locals.var_vds_dn5)), (-(p.p213 * locals.var_vds_dn6)), (-(p.p213 * locals.var_vds_dn7)), (-(p.p213 * locals.var_vds_dn8)), (-(p.p213 * locals.var_vds_dn9)), (-(p.p213 * locals.var_vds_dn10)), (-(p.p213 * locals.var_vds_dn11)), (-(p.p213 * locals.var_vds_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63200_e97926;
        locals.var_t1_dn0 = assign63200_e97926_d_n0;
        locals.var_t1_dn2 = assign63200_e97926_d_n2;
        locals.var_t1_dn4 = assign63200_e97926_d_n4;
        locals.var_t1_dn5 = assign63200_e97926_d_n5;
        locals.var_t1_dn6 = assign63200_e97926_d_n6;
        locals.var_t1_dn7 = assign63200_e97926_d_n7;
        locals.var_t1_dn8 = assign63200_e97926_d_n8;
        locals.var_t1_dn9 = assign63200_e97926_d_n9;
        locals.var_t1_dn10 = assign63200_e97926_d_n10;
        locals.var_t1_dn11 = assign63200_e97926_d_n11;
        locals.var_t1_dn14 = assign63200_e97926_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign63210_e97941, assign63210_e97941_d_n0, assign63210_e97941_d_n2, assign63210_e97941_d_n4, assign63210_e97941_d_n5, assign63210_e97941_d_n6, assign63210_e97941_d_n7, assign63210_e97941_d_n8, assign63210_e97941_d_n9, assign63210_e97941_d_n10, assign63210_e97941_d_n11, assign63210_e97941_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63210_e97935: f64 = (locals.var_vgsz__blk442 - locals.var_vfb);
        let assign63210_e97937: f64 = (assign63210_e97935 + locals.var_t1);
        let assign63210_e97939: f64 = (assign63210_e97937 + locals.var_dvthscsti);
        (assign63210_e97939, ((locals.var_vgsz__blk442_dn0 + locals.var_t1_dn0) + locals.var_dvthscsti_dn0), ((locals.var_vgsz__blk442_dn2 + locals.var_t1_dn2) + locals.var_dvthscsti_dn2), ((locals.var_vgsz__blk442_dn4 + locals.var_t1_dn4) + locals.var_dvthscsti_dn4), ((locals.var_vgsz__blk442_dn5 + locals.var_t1_dn5) + locals.var_dvthscsti_dn5), ((locals.var_vgsz__blk442_dn6 + locals.var_t1_dn6) + locals.var_dvthscsti_dn6), ((locals.var_vgsz__blk442_dn7 + locals.var_t1_dn7) + locals.var_dvthscsti_dn7), ((locals.var_vgsz__blk442_dn8 + locals.var_t1_dn8) + locals.var_dvthscsti_dn8), ((locals.var_vgsz__blk442_dn9 + locals.var_t1_dn9) + locals.var_dvthscsti_dn9), ((locals.var_vgsz__blk442_dn10 + locals.var_t1_dn10) + locals.var_dvthscsti_dn10), ((locals.var_vgsz__blk442_dn11 + locals.var_t1_dn11) + locals.var_dvthscsti_dn11), ((locals.var_vgsz__blk442_dn14 + locals.var_t1_dn14) + locals.var_dvthscsti_dn14),)
    } else {
        (locals.var_vgssti, locals.var_vgssti_dn0, locals.var_vgssti_dn2, locals.var_vgssti_dn4, locals.var_vgssti_dn5, locals.var_vgssti_dn6, locals.var_vgssti_dn7, locals.var_vgssti_dn8, locals.var_vgssti_dn9, locals.var_vgssti_dn10, locals.var_vgssti_dn11, locals.var_vgssti_dn14,)
    }
};
        locals.var_vgssti = assign63210_e97941;
        locals.var_vgssti_dn0 = assign63210_e97941_d_n0;
        locals.var_vgssti_dn2 = assign63210_e97941_d_n2;
        locals.var_vgssti_dn4 = assign63210_e97941_d_n4;
        locals.var_vgssti_dn5 = assign63210_e97941_d_n5;
        locals.var_vgssti_dn6 = assign63210_e97941_d_n6;
        locals.var_vgssti_dn7 = assign63210_e97941_d_n7;
        locals.var_vgssti_dn8 = assign63210_e97941_d_n8;
        locals.var_vgssti_dn9 = assign63210_e97941_d_n9;
        locals.var_vgssti_dn10 = assign63210_e97941_d_n10;
        locals.var_vgssti_dn11 = assign63210_e97941_d_n11;
        locals.var_vgssti_dn14 = assign63210_e97941_d_n14;
        locals.var_vgssti_rv = 0.0;

        let (assign63220_e97954, assign63220_e97954_d_n0, assign63220_e97954_d_n2, assign63220_e97954_d_n4, assign63220_e97954_d_n5, assign63220_e97954_d_n6, assign63220_e97954_d_n7, assign63220_e97954_d_n8, assign63220_e97954_d_n9, assign63220_e97954_d_n10, assign63220_e97954_d_n11, assign63220_e97954_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63220_e97950: f64 = (locals.var_costi0_p2 * locals.var_cox_inv);
        let assign63220_e97952: f64 = (assign63220_e97950 * locals.var_cox_inv);
        (assign63220_e97952, ((((locals.var_costi0_p2_dn0 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn0)) * locals.var_cox_inv) + (assign63220_e97950 * locals.var_cox_inv_dn0)), ((((locals.var_costi0_p2_dn2 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn2)) * locals.var_cox_inv) + (assign63220_e97950 * locals.var_cox_inv_dn2)), ((((locals.var_costi0_p2_dn4 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn4)) * locals.var_cox_inv) + (assign63220_e97950 * locals.var_cox_inv_dn4)), ((((locals.var_costi0_p2_dn5 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn5)) * locals.var_cox_inv) + (assign63220_e97950 * locals.var_cox_inv_dn5)), ((((locals.var_costi0_p2_dn6 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn6)) * locals.var_cox_inv) + (assign63220_e97950 * locals.var_cox_inv_dn6)), ((((locals.var_costi0_p2_dn7 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn7)) * locals.var_cox_inv) + (assign63220_e97950 * locals.var_cox_inv_dn7)), ((((locals.var_costi0_p2_dn8 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn8)) * locals.var_cox_inv) + (assign63220_e97950 * locals.var_cox_inv_dn8)), ((((locals.var_costi0_p2_dn9 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn9)) * locals.var_cox_inv) + (assign63220_e97950 * locals.var_cox_inv_dn9)), ((((locals.var_costi0_p2_dn10 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn10)) * locals.var_cox_inv) + (assign63220_e97950 * locals.var_cox_inv_dn10)), ((((locals.var_costi0_p2_dn11 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn11)) * locals.var_cox_inv) + (assign63220_e97950 * locals.var_cox_inv_dn11)), ((((locals.var_costi0_p2_dn14 * locals.var_cox_inv) + (locals.var_costi0_p2 * locals.var_cox_inv_dn14)) * locals.var_cox_inv) + (assign63220_e97950 * locals.var_cox_inv_dn14)),)
    } else {
        (locals.var_costi3, locals.var_costi3_dn0, locals.var_costi3_dn2, locals.var_costi3_dn4, locals.var_costi3_dn5, locals.var_costi3_dn6, locals.var_costi3_dn7, locals.var_costi3_dn8, locals.var_costi3_dn9, locals.var_costi3_dn10, locals.var_costi3_dn11, locals.var_costi3_dn14,)
    }
};
        locals.var_costi3 = assign63220_e97954;
        locals.var_costi3_dn0 = assign63220_e97954_d_n0;
        locals.var_costi3_dn2 = assign63220_e97954_d_n2;
        locals.var_costi3_dn4 = assign63220_e97954_d_n4;
        locals.var_costi3_dn5 = assign63220_e97954_d_n5;
        locals.var_costi3_dn6 = assign63220_e97954_d_n6;
        locals.var_costi3_dn7 = assign63220_e97954_d_n7;
        locals.var_costi3_dn8 = assign63220_e97954_d_n8;
        locals.var_costi3_dn9 = assign63220_e97954_d_n9;
        locals.var_costi3_dn10 = assign63220_e97954_d_n10;
        locals.var_costi3_dn11 = assign63220_e97954_d_n11;
        locals.var_costi3_dn14 = assign63220_e97954_d_n14;
        locals.var_costi3_rv = 0.0;

        let (assign63230_e97967, assign63230_e97967_d_n0, assign63230_e97967_d_n2, assign63230_e97967_d_n4, assign63230_e97967_d_n5, assign63230_e97967_d_n6, assign63230_e97967_d_n7, assign63230_e97967_d_n8, assign63230_e97967_d_n9, assign63230_e97967_d_n10, assign63230_e97967_d_n11, assign63230_e97967_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63230_e97963: f64 = (locals.var_costi3 * locals.var_beta);
        let assign63230_e97965: f64 = (assign63230_e97963 * 0.5);
        (assign63230_e97965, (((locals.var_costi3_dn0 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn0)) * 0.5), (((locals.var_costi3_dn2 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn2)) * 0.5), (((locals.var_costi3_dn4 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn4)) * 0.5), (((locals.var_costi3_dn5 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn5)) * 0.5), (((locals.var_costi3_dn6 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn6)) * 0.5), (((locals.var_costi3_dn7 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn7)) * 0.5), (((locals.var_costi3_dn8 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn8)) * 0.5), (((locals.var_costi3_dn9 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn9)) * 0.5), (((locals.var_costi3_dn10 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn10)) * 0.5), (((locals.var_costi3_dn11 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn11)) * 0.5), (((locals.var_costi3_dn14 * locals.var_beta) + (locals.var_costi3 * locals.var_beta_dn14)) * 0.5),)
    } else {
        (locals.var_costi4, locals.var_costi4_dn0, locals.var_costi4_dn2, locals.var_costi4_dn4, locals.var_costi4_dn5, locals.var_costi4_dn6, locals.var_costi4_dn7, locals.var_costi4_dn8, locals.var_costi4_dn9, locals.var_costi4_dn10, locals.var_costi4_dn11, locals.var_costi4_dn14,)
    }
};
        locals.var_costi4 = assign63230_e97967;
        locals.var_costi4_dn0 = assign63230_e97967_d_n0;
        locals.var_costi4_dn2 = assign63230_e97967_d_n2;
        locals.var_costi4_dn4 = assign63230_e97967_d_n4;
        locals.var_costi4_dn5 = assign63230_e97967_d_n5;
        locals.var_costi4_dn6 = assign63230_e97967_d_n6;
        locals.var_costi4_dn7 = assign63230_e97967_d_n7;
        locals.var_costi4_dn8 = assign63230_e97967_d_n8;
        locals.var_costi4_dn9 = assign63230_e97967_d_n9;
        locals.var_costi4_dn10 = assign63230_e97967_d_n10;
        locals.var_costi4_dn11 = assign63230_e97967_d_n11;
        locals.var_costi4_dn14 = assign63230_e97967_d_n14;
        locals.var_costi4_rv = 0.0;

        let (assign63240_e97980, assign63240_e97980_d_n0, assign63240_e97980_d_n2, assign63240_e97980_d_n4, assign63240_e97980_d_n5, assign63240_e97980_d_n6, assign63240_e97980_d_n7, assign63240_e97980_d_n8, assign63240_e97980_d_n9, assign63240_e97980_d_n10, assign63240_e97980_d_n11, assign63240_e97980_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63240_e97976: f64 = (locals.var_costi4 * locals.var_beta);
        let assign63240_e97978: f64 = (assign63240_e97976 * 2.0);
        (assign63240_e97978, (((locals.var_costi4_dn0 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn0)) * 2.0), (((locals.var_costi4_dn2 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn2)) * 2.0), (((locals.var_costi4_dn4 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn4)) * 2.0), (((locals.var_costi4_dn5 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn5)) * 2.0), (((locals.var_costi4_dn6 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn6)) * 2.0), (((locals.var_costi4_dn7 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn7)) * 2.0), (((locals.var_costi4_dn8 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn8)) * 2.0), (((locals.var_costi4_dn9 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn9)) * 2.0), (((locals.var_costi4_dn10 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn10)) * 2.0), (((locals.var_costi4_dn11 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn11)) * 2.0), (((locals.var_costi4_dn14 * locals.var_beta) + (locals.var_costi4 * locals.var_beta_dn14)) * 2.0),)
    } else {
        (locals.var_costi5, locals.var_costi5_dn0, locals.var_costi5_dn2, locals.var_costi5_dn4, locals.var_costi5_dn5, locals.var_costi5_dn6, locals.var_costi5_dn7, locals.var_costi5_dn8, locals.var_costi5_dn9, locals.var_costi5_dn10, locals.var_costi5_dn11, locals.var_costi5_dn14,)
    }
};
        locals.var_costi5 = assign63240_e97980;
        locals.var_costi5_dn0 = assign63240_e97980_d_n0;
        locals.var_costi5_dn2 = assign63240_e97980_d_n2;
        locals.var_costi5_dn4 = assign63240_e97980_d_n4;
        locals.var_costi5_dn5 = assign63240_e97980_d_n5;
        locals.var_costi5_dn6 = assign63240_e97980_d_n6;
        locals.var_costi5_dn7 = assign63240_e97980_d_n7;
        locals.var_costi5_dn8 = assign63240_e97980_d_n8;
        locals.var_costi5_dn9 = assign63240_e97980_d_n9;
        locals.var_costi5_dn10 = assign63240_e97980_d_n10;
        locals.var_costi5_dn11 = assign63240_e97980_d_n11;
        locals.var_costi5_dn14 = assign63240_e97980_d_n14;
        locals.var_costi5_rv = 0.0;

        let (assign63250_e97991, assign63250_e97991_d_n0, assign63250_e97991_d_n2, assign63250_e97991_d_n4, assign63250_e97991_d_n5, assign63250_e97991_d_n6, assign63250_e97991_d_n7, assign63250_e97991_d_n8, assign63250_e97991_d_n9, assign63250_e97991_d_n10, assign63250_e97991_d_n11, assign63250_e97991_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63250_e97989: f64 = (locals.var_beta * 0.25);
        (assign63250_e97989, (locals.var_beta_dn0 * 0.25), (locals.var_beta_dn2 * 0.25), (locals.var_beta_dn4 * 0.25), (locals.var_beta_dn5 * 0.25), (locals.var_beta_dn6 * 0.25), (locals.var_beta_dn7 * 0.25), (locals.var_beta_dn8 * 0.25), (locals.var_beta_dn9 * 0.25), (locals.var_beta_dn10 * 0.25), (locals.var_beta_dn11 * 0.25), (locals.var_beta_dn14 * 0.25),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign63250_e97991;
        locals.var_t11_dn0 = assign63250_e97991_d_n0;
        locals.var_t11_dn2 = assign63250_e97991_d_n2;
        locals.var_t11_dn4 = assign63250_e97991_d_n4;
        locals.var_t11_dn5 = assign63250_e97991_d_n5;
        locals.var_t11_dn6 = assign63250_e97991_d_n6;
        locals.var_t11_dn7 = assign63250_e97991_d_n7;
        locals.var_t11_dn8 = assign63250_e97991_d_n8;
        locals.var_t11_dn9 = assign63250_e97991_d_n9;
        locals.var_t11_dn10 = assign63250_e97991_d_n10;
        locals.var_t11_dn11 = assign63250_e97991_d_n11;
        locals.var_t11_dn14 = assign63250_e97991_d_n14;
        locals.var_t11_rv = 0.0;

        let (assign63260_e98012, assign63260_e98012_d_n0, assign63260_e98012_d_n2, assign63260_e98012_d_n4, assign63260_e98012_d_n5, assign63260_e98012_d_n6, assign63260_e98012_d_n7, assign63260_e98012_d_n8, assign63260_e98012_d_n9, assign63260_e98012_d_n10, assign63260_e98012_d_n11, assign63260_e98012_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63260_e98001: f64 = (locals.var_costi3 * locals.var_t11);
        let assign63260_e98002: f64 = (locals.var_beta_inv - assign63260_e98001);
        let assign63260_e98004: f64 = (assign63260_e98002 + locals.var_vfb);
        let assign63260_e98006: f64 = (assign63260_e98004 - locals.var_uc_vthsti);
        let assign63260_e98008: f64 = (assign63260_e98006 - locals.var_dvthscsti);
        let assign63260_e98010: f64 = (assign63260_e98008 + 1e-25);
        (assign63260_e98010, ((locals.var_beta_inv_dn0 - ((locals.var_costi3_dn0 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn0))) - locals.var_dvthscsti_dn0), ((locals.var_beta_inv_dn2 - ((locals.var_costi3_dn2 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn2))) - locals.var_dvthscsti_dn2), ((locals.var_beta_inv_dn4 - ((locals.var_costi3_dn4 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn4))) - locals.var_dvthscsti_dn4), ((locals.var_beta_inv_dn5 - ((locals.var_costi3_dn5 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn5))) - locals.var_dvthscsti_dn5), ((locals.var_beta_inv_dn6 - ((locals.var_costi3_dn6 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn6))) - locals.var_dvthscsti_dn6), ((locals.var_beta_inv_dn7 - ((locals.var_costi3_dn7 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn7))) - locals.var_dvthscsti_dn7), ((locals.var_beta_inv_dn8 - ((locals.var_costi3_dn8 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn8))) - locals.var_dvthscsti_dn8), ((locals.var_beta_inv_dn9 - ((locals.var_costi3_dn9 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn9))) - locals.var_dvthscsti_dn9), ((locals.var_beta_inv_dn10 - ((locals.var_costi3_dn10 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn10))) - locals.var_dvthscsti_dn10), ((locals.var_beta_inv_dn11 - ((locals.var_costi3_dn11 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn11))) - locals.var_dvthscsti_dn11), ((locals.var_beta_inv_dn14 - ((locals.var_costi3_dn14 * locals.var_t11) + (locals.var_costi3 * locals.var_t11_dn14))) - locals.var_dvthscsti_dn14),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign63260_e98012;
        locals.var_t10_dn0 = assign63260_e98012_d_n0;
        locals.var_t10_dn2 = assign63260_e98012_d_n2;
        locals.var_t10_dn4 = assign63260_e98012_d_n4;
        locals.var_t10_dn5 = assign63260_e98012_d_n5;
        locals.var_t10_dn6 = assign63260_e98012_d_n6;
        locals.var_t10_dn7 = assign63260_e98012_d_n7;
        locals.var_t10_dn8 = assign63260_e98012_d_n8;
        locals.var_t10_dn9 = assign63260_e98012_d_n9;
        locals.var_t10_dn10 = assign63260_e98012_d_n10;
        locals.var_t10_dn11 = assign63260_e98012_d_n11;
        locals.var_t10_dn14 = assign63260_e98012_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign63270_e98025, assign63270_e98025_d_n0, assign63270_e98025_d_n2, assign63270_e98025_d_n4, assign63270_e98025_d_n5, assign63270_e98025_d_n6, assign63270_e98025_d_n7, assign63270_e98025_d_n8, assign63270_e98025_d_n9, assign63270_e98025_d_n10, assign63270_e98025_d_n11, assign63270_e98025_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63270_e98021: f64 = (locals.var_vgsz__blk442 - locals.var_t10);
        let assign63270_e98023: f64 = (assign63270_e98021 - 0.005);
        (assign63270_e98023, (locals.var_vgsz__blk442_dn0 - locals.var_t10_dn0), (locals.var_vgsz__blk442_dn2 - locals.var_t10_dn2), (locals.var_vgsz__blk442_dn4 - locals.var_t10_dn4), (locals.var_vgsz__blk442_dn5 - locals.var_t10_dn5), (locals.var_vgsz__blk442_dn6 - locals.var_t10_dn6), (locals.var_vgsz__blk442_dn7 - locals.var_t10_dn7), (locals.var_vgsz__blk442_dn8 - locals.var_t10_dn8), (locals.var_vgsz__blk442_dn9 - locals.var_t10_dn9), (locals.var_vgsz__blk442_dn10 - locals.var_t10_dn10), (locals.var_vgsz__blk442_dn11 - locals.var_t10_dn11), (locals.var_vgsz__blk442_dn14 - locals.var_t10_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63270_e98025;
        locals.var_t1_dn0 = assign63270_e98025_d_n0;
        locals.var_t1_dn2 = assign63270_e98025_d_n2;
        locals.var_t1_dn4 = assign63270_e98025_d_n4;
        locals.var_t1_dn5 = assign63270_e98025_d_n5;
        locals.var_t1_dn6 = assign63270_e98025_d_n6;
        locals.var_t1_dn7 = assign63270_e98025_d_n7;
        locals.var_t1_dn8 = assign63270_e98025_d_n8;
        locals.var_t1_dn9 = assign63270_e98025_d_n9;
        locals.var_t1_dn10 = assign63270_e98025_d_n10;
        locals.var_t1_dn11 = assign63270_e98025_d_n11;
        locals.var_t1_dn14 = assign63270_e98025_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign63280_e98040, assign63280_e98040_d_n0, assign63280_e98040_d_n2, assign63280_e98040_d_n4, assign63280_e98040_d_n5, assign63280_e98040_d_n6, assign63280_e98040_d_n7, assign63280_e98040_d_n8, assign63280_e98040_d_n9, assign63280_e98040_d_n10, assign63280_e98040_d_n11, assign63280_e98040_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let (assign63280_e98038,) = {
            if (locals.var_t10 >= 0.0) {
                (1.0,)
            } else {
                let assign63280_e98037: f64 = (-1.0);
                (assign63280_e98037,)
            }
        };
        (assign63280_e98038, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign63280_e98040;
        locals.var_t0_dn0 = assign63280_e98040_d_n0;
        locals.var_t0_dn2 = assign63280_e98040_d_n2;
        locals.var_t0_dn4 = assign63280_e98040_d_n4;
        locals.var_t0_dn5 = assign63280_e98040_d_n5;
        locals.var_t0_dn6 = assign63280_e98040_d_n6;
        locals.var_t0_dn7 = assign63280_e98040_d_n7;
        locals.var_t0_dn8 = assign63280_e98040_d_n8;
        locals.var_t0_dn9 = assign63280_e98040_d_n9;
        locals.var_t0_dn10 = assign63280_e98040_d_n10;
        locals.var_t0_dn11 = assign63280_e98040_d_n11;
        locals.var_t0_dn14 = assign63280_e98040_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign63290_e98060, assign63290_e98060_d_n0, assign63290_e98060_d_n2, assign63290_e98060_d_n4, assign63290_e98060_d_n5, assign63290_e98060_d_n6, assign63290_e98060_d_n7, assign63290_e98060_d_n8, assign63290_e98060_d_n9, assign63290_e98060_d_n10, assign63290_e98060_d_n11, assign63290_e98060_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63290_e98049: f64 = (locals.var_t1 * locals.var_t1);
        let assign63290_e98052: f64 = (locals.var_t0 * 4.0);
        let assign63290_e98054: f64 = (assign63290_e98052 * locals.var_t10);
        let assign63290_e98056: f64 = (assign63290_e98054 * 0.005);
        let assign63290_e98057: f64 = (assign63290_e98049 + assign63290_e98056);
        let assign63290_e98058: f64 = (assign63290_e98057).sqrt();
        (assign63290_e98058, ((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) + ((((locals.var_t0_dn0 * 4.0) * locals.var_t10) + (assign63290_e98052 * locals.var_t10_dn0)) * 0.005)) / (2.0 * assign63290_e98058)), ((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) + ((((locals.var_t0_dn2 * 4.0) * locals.var_t10) + (assign63290_e98052 * locals.var_t10_dn2)) * 0.005)) / (2.0 * assign63290_e98058)), ((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) + ((((locals.var_t0_dn4 * 4.0) * locals.var_t10) + (assign63290_e98052 * locals.var_t10_dn4)) * 0.005)) / (2.0 * assign63290_e98058)), ((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) + ((((locals.var_t0_dn5 * 4.0) * locals.var_t10) + (assign63290_e98052 * locals.var_t10_dn5)) * 0.005)) / (2.0 * assign63290_e98058)), ((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + ((((locals.var_t0_dn6 * 4.0) * locals.var_t10) + (assign63290_e98052 * locals.var_t10_dn6)) * 0.005)) / (2.0 * assign63290_e98058)), ((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) + ((((locals.var_t0_dn7 * 4.0) * locals.var_t10) + (assign63290_e98052 * locals.var_t10_dn7)) * 0.005)) / (2.0 * assign63290_e98058)), ((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) + ((((locals.var_t0_dn8 * 4.0) * locals.var_t10) + (assign63290_e98052 * locals.var_t10_dn8)) * 0.005)) / (2.0 * assign63290_e98058)), ((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) + ((((locals.var_t0_dn9 * 4.0) * locals.var_t10) + (assign63290_e98052 * locals.var_t10_dn9)) * 0.005)) / (2.0 * assign63290_e98058)), ((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + ((((locals.var_t0_dn10 * 4.0) * locals.var_t10) + (assign63290_e98052 * locals.var_t10_dn10)) * 0.005)) / (2.0 * assign63290_e98058)), ((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) + ((((locals.var_t0_dn11 * 4.0) * locals.var_t10) + (assign63290_e98052 * locals.var_t10_dn11)) * 0.005)) / (2.0 * assign63290_e98058)), ((((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) + ((((locals.var_t0_dn14 * 4.0) * locals.var_t10) + (assign63290_e98052 * locals.var_t10_dn14)) * 0.005)) / (2.0 * assign63290_e98058)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign63290_e98060;
        locals.var_t2_dn0 = assign63290_e98060_d_n0;
        locals.var_t2_dn2 = assign63290_e98060_d_n2;
        locals.var_t2_dn4 = assign63290_e98060_d_n4;
        locals.var_t2_dn5 = assign63290_e98060_d_n5;
        locals.var_t2_dn6 = assign63290_e98060_d_n6;
        locals.var_t2_dn7 = assign63290_e98060_d_n7;
        locals.var_t2_dn8 = assign63290_e98060_d_n8;
        locals.var_t2_dn9 = assign63290_e98060_d_n9;
        locals.var_t2_dn10 = assign63290_e98060_d_n10;
        locals.var_t2_dn11 = assign63290_e98060_d_n11;
        locals.var_t2_dn14 = assign63290_e98060_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign63300_e98083, assign63300_e98083_d_n0, assign63300_e98083_d_n2, assign63300_e98083_d_n4, assign63300_e98083_d_n5, assign63300_e98083_d_n6, assign63300_e98083_d_n7, assign63300_e98083_d_n8, assign63300_e98083_d_n9, assign63300_e98083_d_n10, assign63300_e98083_d_n11, assign63300_e98083_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63300_e98071: f64 = (locals.var_t1 + locals.var_t2);
        let assign63300_e98072: f64 = (0.5 * assign63300_e98071);
        let assign63300_e98073: f64 = (locals.var_t10 + assign63300_e98072);
        let assign63300_e98075: f64 = (assign63300_e98073 - locals.var_vfb);
        let assign63300_e98077: f64 = (assign63300_e98075 + locals.var_uc_vthsti);
        let assign63300_e98079: f64 = (assign63300_e98077 + locals.var_dvthscsti);
        let assign63300_e98081: f64 = (assign63300_e98079 - locals.var_vbsz__blk440);
        (assign63300_e98081, (((locals.var_t10_dn0 + (0.5 * (locals.var_t1_dn0 + locals.var_t2_dn0))) + locals.var_dvthscsti_dn0) - locals.var_vbsz__blk440_dn0), (((locals.var_t10_dn2 + (0.5 * (locals.var_t1_dn2 + locals.var_t2_dn2))) + locals.var_dvthscsti_dn2) - locals.var_vbsz__blk440_dn2), (((locals.var_t10_dn4 + (0.5 * (locals.var_t1_dn4 + locals.var_t2_dn4))) + locals.var_dvthscsti_dn4) - locals.var_vbsz__blk440_dn4), (((locals.var_t10_dn5 + (0.5 * (locals.var_t1_dn5 + locals.var_t2_dn5))) + locals.var_dvthscsti_dn5) - locals.var_vbsz__blk440_dn5), (((locals.var_t10_dn6 + (0.5 * (locals.var_t1_dn6 + locals.var_t2_dn6))) + locals.var_dvthscsti_dn6) - locals.var_vbsz__blk440_dn6), (((locals.var_t10_dn7 + (0.5 * (locals.var_t1_dn7 + locals.var_t2_dn7))) + locals.var_dvthscsti_dn7) - locals.var_vbsz__blk440_dn7), (((locals.var_t10_dn8 + (0.5 * (locals.var_t1_dn8 + locals.var_t2_dn8))) + locals.var_dvthscsti_dn8) - locals.var_vbsz__blk440_dn8), (((locals.var_t10_dn9 + (0.5 * (locals.var_t1_dn9 + locals.var_t2_dn9))) + locals.var_dvthscsti_dn9) - locals.var_vbsz__blk440_dn9), (((locals.var_t10_dn10 + (0.5 * (locals.var_t1_dn10 + locals.var_t2_dn10))) + locals.var_dvthscsti_dn10) - locals.var_vbsz__blk440_dn10), (((locals.var_t10_dn11 + (0.5 * (locals.var_t1_dn11 + locals.var_t2_dn11))) + locals.var_dvthscsti_dn11) - locals.var_vbsz__blk440_dn11), (((locals.var_t10_dn14 + (0.5 * (locals.var_t1_dn14 + locals.var_t2_dn14))) + locals.var_dvthscsti_dn14) - locals.var_vbsz__blk440_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign63300_e98083;
        locals.var_t3_dn0 = assign63300_e98083_d_n0;
        locals.var_t3_dn2 = assign63300_e98083_d_n2;
        locals.var_t3_dn4 = assign63300_e98083_d_n4;
        locals.var_t3_dn5 = assign63300_e98083_d_n5;
        locals.var_t3_dn6 = assign63300_e98083_d_n6;
        locals.var_t3_dn7 = assign63300_e98083_d_n7;
        locals.var_t3_dn8 = assign63300_e98083_d_n8;
        locals.var_t3_dn9 = assign63300_e98083_d_n9;
        locals.var_t3_dn10 = assign63300_e98083_d_n10;
        locals.var_t3_dn11 = assign63300_e98083_d_n11;
        locals.var_t3_dn14 = assign63300_e98083_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign63310_e98096, assign63310_e98096_d_n0, assign63310_e98096_d_n2, assign63310_e98096_d_n4, assign63310_e98096_d_n5, assign63310_e98096_d_n6, assign63310_e98096_d_n7, assign63310_e98096_d_n8, assign63310_e98096_d_n9, assign63310_e98096_d_n10, assign63310_e98096_d_n11, assign63310_e98096_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63310_e98092: f64 = (locals.var_beta * locals.var_t3);
        let assign63310_e98094: f64 = (assign63310_e98092 - 1.0);
        (assign63310_e98094, ((locals.var_beta_dn0 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn0)), ((locals.var_beta_dn2 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn2)), ((locals.var_beta_dn4 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn4)), ((locals.var_beta_dn5 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn5)), ((locals.var_beta_dn6 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn6)), ((locals.var_beta_dn7 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn7)), ((locals.var_beta_dn8 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn8)), ((locals.var_beta_dn9 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn9)), ((locals.var_beta_dn10 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn10)), ((locals.var_beta_dn11 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn11)), ((locals.var_beta_dn14 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign63310_e98096;
        locals.var_t4_dn0 = assign63310_e98096_d_n0;
        locals.var_t4_dn2 = assign63310_e98096_d_n2;
        locals.var_t4_dn4 = assign63310_e98096_d_n4;
        locals.var_t4_dn5 = assign63310_e98096_d_n5;
        locals.var_t4_dn6 = assign63310_e98096_d_n6;
        locals.var_t4_dn7 = assign63310_e98096_d_n7;
        locals.var_t4_dn8 = assign63310_e98096_d_n8;
        locals.var_t4_dn9 = assign63310_e98096_d_n9;
        locals.var_t4_dn10 = assign63310_e98096_d_n10;
        locals.var_t4_dn11 = assign63310_e98096_d_n11;
        locals.var_t4_dn14 = assign63310_e98096_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign63320_e98107, assign63320_e98107_d_n0, assign63320_e98107_d_n2, assign63320_e98107_d_n4, assign63320_e98107_d_n5, assign63320_e98107_d_n6, assign63320_e98107_d_n7, assign63320_e98107_d_n8, assign63320_e98107_d_n9, assign63320_e98107_d_n10, assign63320_e98107_d_n11, assign63320_e98107_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63320_e98105: f64 = (4.0 / locals.var_costi5);
        (assign63320_e98105, (-((4.0 * locals.var_costi5_dn0) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn2) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn4) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn5) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn6) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn7) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn8) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn9) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn10) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn11) / (locals.var_costi5 * locals.var_costi5))), (-((4.0 * locals.var_costi5_dn14) / (locals.var_costi5 * locals.var_costi5))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign63320_e98107;
        locals.var_t5_dn0 = assign63320_e98107_d_n0;
        locals.var_t5_dn2 = assign63320_e98107_d_n2;
        locals.var_t5_dn4 = assign63320_e98107_d_n4;
        locals.var_t5_dn5 = assign63320_e98107_d_n5;
        locals.var_t5_dn6 = assign63320_e98107_d_n6;
        locals.var_t5_dn7 = assign63320_e98107_d_n7;
        locals.var_t5_dn8 = assign63320_e98107_d_n8;
        locals.var_t5_dn9 = assign63320_e98107_d_n9;
        locals.var_t5_dn10 = assign63320_e98107_d_n10;
        locals.var_t5_dn11 = assign63320_e98107_d_n11;
        locals.var_t5_dn14 = assign63320_e98107_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign63330_e98120, assign63330_e98120_d_n0, assign63330_e98120_d_n2, assign63330_e98120_d_n4, assign63330_e98120_d_n5, assign63330_e98120_d_n6, assign63330_e98120_d_n7, assign63330_e98120_d_n8, assign63330_e98120_d_n9, assign63330_e98120_d_n10, assign63330_e98120_d_n11, assign63330_e98120_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63330_e98117: f64 = (locals.var_t4 * locals.var_t5);
        let assign63330_e98118: f64 = (1.0 + assign63330_e98117);
        (assign63330_e98118, ((locals.var_t4_dn0 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn0)), ((locals.var_t4_dn2 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn2)), ((locals.var_t4_dn4 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn4)), ((locals.var_t4_dn5 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn5)), ((locals.var_t4_dn6 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn6)), ((locals.var_t4_dn7 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn7)), ((locals.var_t4_dn8 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn8)), ((locals.var_t4_dn9 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn9)), ((locals.var_t4_dn10 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn10)), ((locals.var_t4_dn11 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn11)), ((locals.var_t4_dn14 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63330_e98120;
        locals.var_t1_dn0 = assign63330_e98120_d_n0;
        locals.var_t1_dn2 = assign63330_e98120_d_n2;
        locals.var_t1_dn4 = assign63330_e98120_d_n4;
        locals.var_t1_dn5 = assign63330_e98120_d_n5;
        locals.var_t1_dn6 = assign63330_e98120_d_n6;
        locals.var_t1_dn7 = assign63330_e98120_d_n7;
        locals.var_t1_dn8 = assign63330_e98120_d_n8;
        locals.var_t1_dn9 = assign63330_e98120_d_n9;
        locals.var_t1_dn10 = assign63330_e98120_d_n10;
        locals.var_t1_dn11 = assign63330_e98120_d_n11;
        locals.var_t1_dn14 = assign63330_e98120_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign63340_e98131, assign63340_e98131_d_n0, assign63340_e98131_d_n2, assign63340_e98131_d_n4, assign63340_e98131_d_n5, assign63340_e98131_d_n6, assign63340_e98131_d_n7, assign63340_e98131_d_n8, assign63340_e98131_d_n9, assign63340_e98131_d_n10, assign63340_e98131_d_n11, assign63340_e98131_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63340_e98129: f64 = (locals.var_beta * locals.var_t5);
        (assign63340_e98129, ((locals.var_beta_dn0 * locals.var_t5) + (locals.var_beta * locals.var_t5_dn0)), ((locals.var_beta_dn2 * locals.var_t5) + (locals.var_beta * locals.var_t5_dn2)), ((locals.var_beta_dn4 * locals.var_t5) + (locals.var_beta * locals.var_t5_dn4)), ((locals.var_beta_dn5 * locals.var_t5) + (locals.var_beta * locals.var_t5_dn5)), ((locals.var_beta_dn6 * locals.var_t5) + (locals.var_beta * locals.var_t5_dn6)), ((locals.var_beta_dn7 * locals.var_t5) + (locals.var_beta * locals.var_t5_dn7)), ((locals.var_beta_dn8 * locals.var_t5) + (locals.var_beta * locals.var_t5_dn8)), ((locals.var_beta_dn9 * locals.var_t5) + (locals.var_beta * locals.var_t5_dn9)), ((locals.var_beta_dn10 * locals.var_t5) + (locals.var_beta * locals.var_t5_dn10)), ((locals.var_beta_dn11 * locals.var_t5) + (locals.var_beta * locals.var_t5_dn11)), ((locals.var_beta_dn14 * locals.var_t5) + (locals.var_beta * locals.var_t5_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign63340_e98131;
        locals.var_t6_dn0 = assign63340_e98131_d_n0;
        locals.var_t6_dn2 = assign63340_e98131_d_n2;
        locals.var_t6_dn4 = assign63340_e98131_d_n4;
        locals.var_t6_dn5 = assign63340_e98131_d_n5;
        locals.var_t6_dn6 = assign63340_e98131_d_n6;
        locals.var_t6_dn7 = assign63340_e98131_d_n7;
        locals.var_t6_dn8 = assign63340_e98131_d_n8;
        locals.var_t6_dn9 = assign63340_e98131_d_n9;
        locals.var_t6_dn10 = assign63340_e98131_d_n10;
        locals.var_t6_dn11 = assign63340_e98131_d_n11;
        locals.var_t6_dn14 = assign63340_e98131_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign63350_e98142, assign63350_e98142_d_n0, assign63350_e98142_d_n2, assign63350_e98142_d_n4, assign63350_e98142_d_n5, assign63350_e98142_d_n6, assign63350_e98142_d_n7, assign63350_e98142_d_n8, assign63350_e98142_d_n9, assign63350_e98142_d_n10, assign63350_e98142_d_n11, assign63350_e98142_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63350_e98140: f64 = (locals.var_t4 * locals.var_t5);
        (assign63350_e98140, ((locals.var_t4_dn0 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn0)), ((locals.var_t4_dn2 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn2)), ((locals.var_t4_dn4 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn4)), ((locals.var_t4_dn5 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn5)), ((locals.var_t4_dn6 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn6)), ((locals.var_t4_dn7 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn7)), ((locals.var_t4_dn8 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn8)), ((locals.var_t4_dn9 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn9)), ((locals.var_t4_dn10 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn10)), ((locals.var_t4_dn11 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn11)), ((locals.var_t4_dn14 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn14)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign63350_e98142;
        locals.var_t7_dn0 = assign63350_e98142_d_n0;
        locals.var_t7_dn2 = assign63350_e98142_d_n2;
        locals.var_t7_dn4 = assign63350_e98142_d_n4;
        locals.var_t7_dn5 = assign63350_e98142_d_n5;
        locals.var_t7_dn6 = assign63350_e98142_d_n6;
        locals.var_t7_dn7 = assign63350_e98142_d_n7;
        locals.var_t7_dn8 = assign63350_e98142_d_n8;
        locals.var_t7_dn9 = assign63350_e98142_d_n9;
        locals.var_t7_dn10 = assign63350_e98142_d_n10;
        locals.var_t7_dn11 = assign63350_e98142_d_n11;
        locals.var_t7_dn14 = assign63350_e98142_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign63360_e98160, assign63360_e98160_d_n0, assign63360_e98160_d_n2, assign63360_e98160_d_n4, assign63360_e98160_d_n5, assign63360_e98160_d_n6, assign63360_e98160_d_n7, assign63360_e98160_d_n8, assign63360_e98160_d_n9, assign63360_e98160_d_n10, assign63360_e98160_d_n11, assign63360_e98160_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63360_e98151: f64 = (locals.var_t1 * locals.var_t1);
        let assign63360_e98154: f64 = (4.0 * 0.01);
        let assign63360_e98156: f64 = (assign63360_e98154 * 0.01);
        let assign63360_e98157: f64 = (assign63360_e98151 + assign63360_e98156);
        let assign63360_e98158: f64 = (assign63360_e98157).sqrt();
        (assign63360_e98158, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign63360_e98158)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign63360_e98158)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign63360_e98158)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign63360_e98158)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign63360_e98158)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign63360_e98158)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign63360_e98158)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign63360_e98158)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign63360_e98158)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign63360_e98158)), (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign63360_e98158)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign63360_e98160;
        locals.var_tmf2_dn0 = assign63360_e98160_d_n0;
        locals.var_tmf2_dn2 = assign63360_e98160_d_n2;
        locals.var_tmf2_dn4 = assign63360_e98160_d_n4;
        locals.var_tmf2_dn5 = assign63360_e98160_d_n5;
        locals.var_tmf2_dn6 = assign63360_e98160_d_n6;
        locals.var_tmf2_dn7 = assign63360_e98160_d_n7;
        locals.var_tmf2_dn8 = assign63360_e98160_d_n8;
        locals.var_tmf2_dn9 = assign63360_e98160_d_n9;
        locals.var_tmf2_dn10 = assign63360_e98160_d_n10;
        locals.var_tmf2_dn11 = assign63360_e98160_d_n11;
        locals.var_tmf2_dn14 = assign63360_e98160_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign63370_e98175, assign63370_e98175_d_n0, assign63370_e98175_d_n2, assign63370_e98175_d_n4, assign63370_e98175_d_n5, assign63370_e98175_d_n6, assign63370_e98175_d_n7, assign63370_e98175_d_n8, assign63370_e98175_d_n9, assign63370_e98175_d_n10, assign63370_e98175_d_n11, assign63370_e98175_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63370_e98171: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign63370_e98172: f64 = (1.0 + assign63370_e98171);
        let assign63370_e98173: f64 = (0.5 * assign63370_e98172);
        (assign63370_e98173, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn14 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign63370_e98175;
        locals.var_t2_dn0 = assign63370_e98175_d_n0;
        locals.var_t2_dn2 = assign63370_e98175_d_n2;
        locals.var_t2_dn4 = assign63370_e98175_d_n4;
        locals.var_t2_dn5 = assign63370_e98175_d_n5;
        locals.var_t2_dn6 = assign63370_e98175_d_n6;
        locals.var_t2_dn7 = assign63370_e98175_d_n7;
        locals.var_t2_dn8 = assign63370_e98175_d_n8;
        locals.var_t2_dn9 = assign63370_e98175_d_n9;
        locals.var_t2_dn10 = assign63370_e98175_d_n10;
        locals.var_t2_dn11 = assign63370_e98175_d_n11;
        locals.var_t2_dn14 = assign63370_e98175_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign63380_e98188, assign63380_e98188_d_n0, assign63380_e98188_d_n2, assign63380_e98188_d_n4, assign63380_e98188_d_n5, assign63380_e98188_d_n6, assign63380_e98188_d_n7, assign63380_e98188_d_n8, assign63380_e98188_d_n9, assign63380_e98188_d_n10, assign63380_e98188_d_n11, assign63380_e98188_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63380_e98185: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign63380_e98186: f64 = (0.5 * assign63380_e98185);
        (assign63380_e98186, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63380_e98188;
        locals.var_t1_dn0 = assign63380_e98188_d_n0;
        locals.var_t1_dn2 = assign63380_e98188_d_n2;
        locals.var_t1_dn4 = assign63380_e98188_d_n4;
        locals.var_t1_dn5 = assign63380_e98188_d_n5;
        locals.var_t1_dn6 = assign63380_e98188_d_n6;
        locals.var_t1_dn7 = assign63380_e98188_d_n7;
        locals.var_t1_dn8 = assign63380_e98188_d_n8;
        locals.var_t1_dn9 = assign63380_e98188_d_n9;
        locals.var_t1_dn10 = assign63380_e98188_d_n10;
        locals.var_t1_dn11 = assign63380_e98188_d_n11;
        locals.var_t1_dn14 = assign63380_e98188_d_n14;
        locals.var_t1_rv = 0.0;

        let assign63390_e98191: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1506 = assign63390_e98191;
        locals.var_guard1506_rv = 0.0;

        let (assign63400_e98202, assign63400_e98202_d_n0, assign63400_e98202_d_n2, assign63400_e98202_d_n4, assign63400_e98202_d_n5, assign63400_e98202_d_n6, assign63400_e98202_d_n7, assign63400_e98202_d_n8, assign63400_e98202_d_n9, assign63400_e98202_d_n10, assign63400_e98202_d_n11, assign63400_e98202_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) && (locals.var_guard1506 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63400_e98202;
        locals.var_t1_dn0 = assign63400_e98202_d_n0;
        locals.var_t1_dn2 = assign63400_e98202_d_n2;
        locals.var_t1_dn4 = assign63400_e98202_d_n4;
        locals.var_t1_dn5 = assign63400_e98202_d_n5;
        locals.var_t1_dn6 = assign63400_e98202_d_n6;
        locals.var_t1_dn7 = assign63400_e98202_d_n7;
        locals.var_t1_dn8 = assign63400_e98202_d_n8;
        locals.var_t1_dn9 = assign63400_e98202_d_n9;
        locals.var_t1_dn10 = assign63400_e98202_d_n10;
        locals.var_t1_dn11 = assign63400_e98202_d_n11;
        locals.var_t1_dn14 = assign63400_e98202_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_237(
        locals: &mut StampLocals,
    ) {
        let (assign63410_e98213, assign63410_e98213_d_n0, assign63410_e98213_d_n2, assign63410_e98213_d_n4, assign63410_e98213_d_n5, assign63410_e98213_d_n6, assign63410_e98213_d_n7, assign63410_e98213_d_n8, assign63410_e98213_d_n9, assign63410_e98213_d_n10, assign63410_e98213_d_n11, assign63410_e98213_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) && (locals.var_guard1506 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign63410_e98213;
        locals.var_t2_dn0 = assign63410_e98213_d_n0;
        locals.var_t2_dn2 = assign63410_e98213_d_n2;
        locals.var_t2_dn4 = assign63410_e98213_d_n4;
        locals.var_t2_dn5 = assign63410_e98213_d_n5;
        locals.var_t2_dn6 = assign63410_e98213_d_n6;
        locals.var_t2_dn7 = assign63410_e98213_d_n7;
        locals.var_t2_dn8 = assign63410_e98213_d_n8;
        locals.var_t2_dn9 = assign63410_e98213_d_n9;
        locals.var_t2_dn10 = assign63410_e98213_d_n10;
        locals.var_t2_dn11 = assign63410_e98213_d_n11;
        locals.var_t2_dn14 = assign63410_e98213_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign63420_e98224, assign63420_e98224_d_n0, assign63420_e98224_d_n2, assign63420_e98224_d_n4, assign63420_e98224_d_n5, assign63420_e98224_d_n6, assign63420_e98224_d_n7, assign63420_e98224_d_n8, assign63420_e98224_d_n9, assign63420_e98224_d_n10, assign63420_e98224_d_n11, assign63420_e98224_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63420_e98222: f64 = (locals.var_t1 + 1e-25);
        (assign63420_e98222, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63420_e98224;
        locals.var_t1_dn0 = assign63420_e98224_d_n0;
        locals.var_t1_dn2 = assign63420_e98224_d_n2;
        locals.var_t1_dn4 = assign63420_e98224_d_n4;
        locals.var_t1_dn5 = assign63420_e98224_d_n5;
        locals.var_t1_dn6 = assign63420_e98224_d_n6;
        locals.var_t1_dn7 = assign63420_e98224_d_n7;
        locals.var_t1_dn8 = assign63420_e98224_d_n8;
        locals.var_t1_dn9 = assign63420_e98224_d_n9;
        locals.var_t1_dn10 = assign63420_e98224_d_n10;
        locals.var_t1_dn11 = assign63420_e98224_d_n11;
        locals.var_t1_dn14 = assign63420_e98224_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign63430_e98234, assign63430_e98234_d_n0, assign63430_e98234_d_n2, assign63430_e98234_d_n4, assign63430_e98234_d_n5, assign63430_e98234_d_n6, assign63430_e98234_d_n7, assign63430_e98234_d_n8, assign63430_e98234_d_n9, assign63430_e98234_d_n10, assign63430_e98234_d_n11, assign63430_e98234_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63430_e98232: f64 = (locals.var_t1).sqrt();
        (assign63430_e98232, (locals.var_t1_dn0 / (2.0 * assign63430_e98232)), (locals.var_t1_dn2 / (2.0 * assign63430_e98232)), (locals.var_t1_dn4 / (2.0 * assign63430_e98232)), (locals.var_t1_dn5 / (2.0 * assign63430_e98232)), (locals.var_t1_dn6 / (2.0 * assign63430_e98232)), (locals.var_t1_dn7 / (2.0 * assign63430_e98232)), (locals.var_t1_dn8 / (2.0 * assign63430_e98232)), (locals.var_t1_dn9 / (2.0 * assign63430_e98232)), (locals.var_t1_dn10 / (2.0 * assign63430_e98232)), (locals.var_t1_dn11 / (2.0 * assign63430_e98232)), (locals.var_t1_dn14 / (2.0 * assign63430_e98232)),)
    } else {
        (locals.var_costi6, locals.var_costi6_dn0, locals.var_costi6_dn2, locals.var_costi6_dn4, locals.var_costi6_dn5, locals.var_costi6_dn6, locals.var_costi6_dn7, locals.var_costi6_dn8, locals.var_costi6_dn9, locals.var_costi6_dn10, locals.var_costi6_dn11, locals.var_costi6_dn14,)
    }
};
        locals.var_costi6 = assign63430_e98234;
        locals.var_costi6_dn0 = assign63430_e98234_d_n0;
        locals.var_costi6_dn2 = assign63430_e98234_d_n2;
        locals.var_costi6_dn4 = assign63430_e98234_d_n4;
        locals.var_costi6_dn5 = assign63430_e98234_d_n5;
        locals.var_costi6_dn6 = assign63430_e98234_d_n6;
        locals.var_costi6_dn7 = assign63430_e98234_d_n7;
        locals.var_costi6_dn8 = assign63430_e98234_d_n8;
        locals.var_costi6_dn9 = assign63430_e98234_d_n9;
        locals.var_costi6_dn10 = assign63430_e98234_d_n10;
        locals.var_costi6_dn11 = assign63430_e98234_d_n11;
        locals.var_costi6_dn14 = assign63430_e98234_d_n14;
        locals.var_costi6_rv = 0.0;

        let (assign63440_e98247, assign63440_e98247_d_n0, assign63440_e98247_d_n2, assign63440_e98247_d_n4, assign63440_e98247_d_n5, assign63440_e98247_d_n6, assign63440_e98247_d_n7, assign63440_e98247_d_n8, assign63440_e98247_d_n9, assign63440_e98247_d_n10, assign63440_e98247_d_n11, assign63440_e98247_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63440_e98244: f64 = (1.0 - locals.var_costi6);
        let assign63440_e98245: f64 = (locals.var_costi4 * assign63440_e98244);
        (assign63440_e98245, ((locals.var_costi4_dn0 * assign63440_e98244) + (locals.var_costi4 * (-locals.var_costi6_dn0))), ((locals.var_costi4_dn2 * assign63440_e98244) + (locals.var_costi4 * (-locals.var_costi6_dn2))), ((locals.var_costi4_dn4 * assign63440_e98244) + (locals.var_costi4 * (-locals.var_costi6_dn4))), ((locals.var_costi4_dn5 * assign63440_e98244) + (locals.var_costi4 * (-locals.var_costi6_dn5))), ((locals.var_costi4_dn6 * assign63440_e98244) + (locals.var_costi4 * (-locals.var_costi6_dn6))), ((locals.var_costi4_dn7 * assign63440_e98244) + (locals.var_costi4 * (-locals.var_costi6_dn7))), ((locals.var_costi4_dn8 * assign63440_e98244) + (locals.var_costi4 * (-locals.var_costi6_dn8))), ((locals.var_costi4_dn9 * assign63440_e98244) + (locals.var_costi4 * (-locals.var_costi6_dn9))), ((locals.var_costi4_dn10 * assign63440_e98244) + (locals.var_costi4 * (-locals.var_costi6_dn10))), ((locals.var_costi4_dn11 * assign63440_e98244) + (locals.var_costi4 * (-locals.var_costi6_dn11))), ((locals.var_costi4_dn14 * assign63440_e98244) + (locals.var_costi4 * (-locals.var_costi6_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign63440_e98247;
        locals.var_t0_dn0 = assign63440_e98247_d_n0;
        locals.var_t0_dn2 = assign63440_e98247_d_n2;
        locals.var_t0_dn4 = assign63440_e98247_d_n4;
        locals.var_t0_dn5 = assign63440_e98247_d_n5;
        locals.var_t0_dn6 = assign63440_e98247_d_n6;
        locals.var_t0_dn7 = assign63440_e98247_d_n7;
        locals.var_t0_dn8 = assign63440_e98247_d_n8;
        locals.var_t0_dn9 = assign63440_e98247_d_n9;
        locals.var_t0_dn10 = assign63440_e98247_d_n10;
        locals.var_t0_dn11 = assign63440_e98247_d_n11;
        locals.var_t0_dn14 = assign63440_e98247_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign63450_e98258, assign63450_e98258_d_n0, assign63450_e98258_d_n2, assign63450_e98258_d_n4, assign63450_e98258_d_n5, assign63450_e98258_d_n6, assign63450_e98258_d_n7, assign63450_e98258_d_n8, assign63450_e98258_d_n9, assign63450_e98258_d_n10, assign63450_e98258_d_n11, assign63450_e98258_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63450_e98256: f64 = (locals.var_vgssti + locals.var_t0);
        (assign63450_e98256, (locals.var_vgssti_dn0 + locals.var_t0_dn0), (locals.var_vgssti_dn2 + locals.var_t0_dn2), (locals.var_vgssti_dn4 + locals.var_t0_dn4), (locals.var_vgssti_dn5 + locals.var_t0_dn5), (locals.var_vgssti_dn6 + locals.var_t0_dn6), (locals.var_vgssti_dn7 + locals.var_t0_dn7), (locals.var_vgssti_dn8 + locals.var_t0_dn8), (locals.var_vgssti_dn9 + locals.var_t0_dn9), (locals.var_vgssti_dn10 + locals.var_t0_dn10), (locals.var_vgssti_dn11 + locals.var_t0_dn11), (locals.var_vgssti_dn14 + locals.var_t0_dn14),)
    } else {
        (locals.var_psasti, locals.var_psasti_dn0, locals.var_psasti_dn2, locals.var_psasti_dn4, locals.var_psasti_dn5, locals.var_psasti_dn6, locals.var_psasti_dn7, locals.var_psasti_dn8, locals.var_psasti_dn9, locals.var_psasti_dn10, locals.var_psasti_dn11, locals.var_psasti_dn14,)
    }
};
        locals.var_psasti = assign63450_e98258;
        locals.var_psasti_dn0 = assign63450_e98258_d_n0;
        locals.var_psasti_dn2 = assign63450_e98258_d_n2;
        locals.var_psasti_dn4 = assign63450_e98258_d_n4;
        locals.var_psasti_dn5 = assign63450_e98258_d_n5;
        locals.var_psasti_dn6 = assign63450_e98258_d_n6;
        locals.var_psasti_dn7 = assign63450_e98258_d_n7;
        locals.var_psasti_dn8 = assign63450_e98258_d_n8;
        locals.var_psasti_dn9 = assign63450_e98258_d_n9;
        locals.var_psasti_dn10 = assign63450_e98258_d_n10;
        locals.var_psasti_dn11 = assign63450_e98258_d_n11;
        locals.var_psasti_dn14 = assign63450_e98258_d_n14;
        locals.var_psasti_rv = 0.0;

        let (assign63460_e98275, assign63460_e98275_d_n0, assign63460_e98275_d_n2, assign63460_e98275_d_n4, assign63460_e98275_d_n5, assign63460_e98275_d_n6, assign63460_e98275_d_n7, assign63460_e98275_d_n8, assign63460_e98275_d_n9, assign63460_e98275_d_n10, assign63460_e98275_d_n11, assign63460_e98275_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63460_e98270: f64 = (locals.var_vgssti + 1e-25);
        let assign63460_e98271: f64 = (2.0 / assign63460_e98270);
        let assign63460_e98272: f64 = (locals.var_beta + assign63460_e98271);
        let assign63460_e98273: f64 = (1.0 / assign63460_e98272);
        (assign63460_e98273, (-((locals.var_beta_dn0 + (-((2.0 * locals.var_vgssti_dn0) / (assign63460_e98270 * assign63460_e98270)))) / (assign63460_e98272 * assign63460_e98272))), (-((locals.var_beta_dn2 + (-((2.0 * locals.var_vgssti_dn2) / (assign63460_e98270 * assign63460_e98270)))) / (assign63460_e98272 * assign63460_e98272))), (-((locals.var_beta_dn4 + (-((2.0 * locals.var_vgssti_dn4) / (assign63460_e98270 * assign63460_e98270)))) / (assign63460_e98272 * assign63460_e98272))), (-((locals.var_beta_dn5 + (-((2.0 * locals.var_vgssti_dn5) / (assign63460_e98270 * assign63460_e98270)))) / (assign63460_e98272 * assign63460_e98272))), (-((locals.var_beta_dn6 + (-((2.0 * locals.var_vgssti_dn6) / (assign63460_e98270 * assign63460_e98270)))) / (assign63460_e98272 * assign63460_e98272))), (-((locals.var_beta_dn7 + (-((2.0 * locals.var_vgssti_dn7) / (assign63460_e98270 * assign63460_e98270)))) / (assign63460_e98272 * assign63460_e98272))), (-((locals.var_beta_dn8 + (-((2.0 * locals.var_vgssti_dn8) / (assign63460_e98270 * assign63460_e98270)))) / (assign63460_e98272 * assign63460_e98272))), (-((locals.var_beta_dn9 + (-((2.0 * locals.var_vgssti_dn9) / (assign63460_e98270 * assign63460_e98270)))) / (assign63460_e98272 * assign63460_e98272))), (-((locals.var_beta_dn10 + (-((2.0 * locals.var_vgssti_dn10) / (assign63460_e98270 * assign63460_e98270)))) / (assign63460_e98272 * assign63460_e98272))), (-((locals.var_beta_dn11 + (-((2.0 * locals.var_vgssti_dn11) / (assign63460_e98270 * assign63460_e98270)))) / (assign63460_e98272 * assign63460_e98272))), (-((locals.var_beta_dn14 + (-((2.0 * locals.var_vgssti_dn14) / (assign63460_e98270 * assign63460_e98270)))) / (assign63460_e98272 * assign63460_e98272))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign63460_e98275;
        locals.var_t0_dn0 = assign63460_e98275_d_n0;
        locals.var_t0_dn2 = assign63460_e98275_d_n2;
        locals.var_t0_dn4 = assign63460_e98275_d_n4;
        locals.var_t0_dn5 = assign63460_e98275_d_n5;
        locals.var_t0_dn6 = assign63460_e98275_d_n6;
        locals.var_t0_dn7 = assign63460_e98275_d_n7;
        locals.var_t0_dn8 = assign63460_e98275_d_n8;
        locals.var_t0_dn9 = assign63460_e98275_d_n9;
        locals.var_t0_dn10 = assign63460_e98275_d_n10;
        locals.var_t0_dn11 = assign63460_e98275_d_n11;
        locals.var_t0_dn14 = assign63460_e98275_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign63470_e98295, assign63470_e98295_d_n0, assign63470_e98295_d_n2, assign63470_e98295_d_n4, assign63470_e98295_d_n5, assign63470_e98295_d_n6, assign63470_e98295_d_n7, assign63470_e98295_d_n8, assign63470_e98295_d_n9, assign63470_e98295_d_n10, assign63470_e98295_d_n11, assign63470_e98295_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63470_e98284: f64 = (1.0 / locals.var_costi1);
        let assign63470_e98286: f64 = (assign63470_e98284 / locals.var_costi3);
        let assign63470_e98289: f64 = (locals.var_vgssti * locals.var_vgssti);
        let assign63470_e98290: f64 = (assign63470_e98286 * assign63470_e98289);
        let assign63470_e98291: f64 = (assign63470_e98290).ln();
        let assign63470_e98293: f64 = (assign63470_e98291 * locals.var_t0);
        (assign63470_e98293, (((((((((-(locals.var_costi1_dn0 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign63470_e98284 * locals.var_costi3_dn0)) / (locals.var_costi3 * locals.var_costi3)) * assign63470_e98289) + (assign63470_e98286 * ((locals.var_vgssti_dn0 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn0)))) / assign63470_e98290) * locals.var_t0) + (assign63470_e98291 * locals.var_t0_dn0)), (((((((((-(locals.var_costi1_dn2 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign63470_e98284 * locals.var_costi3_dn2)) / (locals.var_costi3 * locals.var_costi3)) * assign63470_e98289) + (assign63470_e98286 * ((locals.var_vgssti_dn2 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn2)))) / assign63470_e98290) * locals.var_t0) + (assign63470_e98291 * locals.var_t0_dn2)), (((((((((-(locals.var_costi1_dn4 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign63470_e98284 * locals.var_costi3_dn4)) / (locals.var_costi3 * locals.var_costi3)) * assign63470_e98289) + (assign63470_e98286 * ((locals.var_vgssti_dn4 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn4)))) / assign63470_e98290) * locals.var_t0) + (assign63470_e98291 * locals.var_t0_dn4)), (((((((((-(locals.var_costi1_dn5 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign63470_e98284 * locals.var_costi3_dn5)) / (locals.var_costi3 * locals.var_costi3)) * assign63470_e98289) + (assign63470_e98286 * ((locals.var_vgssti_dn5 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn5)))) / assign63470_e98290) * locals.var_t0) + (assign63470_e98291 * locals.var_t0_dn5)), (((((((((-(locals.var_costi1_dn6 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign63470_e98284 * locals.var_costi3_dn6)) / (locals.var_costi3 * locals.var_costi3)) * assign63470_e98289) + (assign63470_e98286 * ((locals.var_vgssti_dn6 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn6)))) / assign63470_e98290) * locals.var_t0) + (assign63470_e98291 * locals.var_t0_dn6)), (((((((((-(locals.var_costi1_dn7 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign63470_e98284 * locals.var_costi3_dn7)) / (locals.var_costi3 * locals.var_costi3)) * assign63470_e98289) + (assign63470_e98286 * ((locals.var_vgssti_dn7 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn7)))) / assign63470_e98290) * locals.var_t0) + (assign63470_e98291 * locals.var_t0_dn7)), (((((((((-(locals.var_costi1_dn8 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign63470_e98284 * locals.var_costi3_dn8)) / (locals.var_costi3 * locals.var_costi3)) * assign63470_e98289) + (assign63470_e98286 * ((locals.var_vgssti_dn8 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn8)))) / assign63470_e98290) * locals.var_t0) + (assign63470_e98291 * locals.var_t0_dn8)), (((((((((-(locals.var_costi1_dn9 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign63470_e98284 * locals.var_costi3_dn9)) / (locals.var_costi3 * locals.var_costi3)) * assign63470_e98289) + (assign63470_e98286 * ((locals.var_vgssti_dn9 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn9)))) / assign63470_e98290) * locals.var_t0) + (assign63470_e98291 * locals.var_t0_dn9)), (((((((((-(locals.var_costi1_dn10 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign63470_e98284 * locals.var_costi3_dn10)) / (locals.var_costi3 * locals.var_costi3)) * assign63470_e98289) + (assign63470_e98286 * ((locals.var_vgssti_dn10 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn10)))) / assign63470_e98290) * locals.var_t0) + (assign63470_e98291 * locals.var_t0_dn10)), (((((((((-(locals.var_costi1_dn11 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign63470_e98284 * locals.var_costi3_dn11)) / (locals.var_costi3 * locals.var_costi3)) * assign63470_e98289) + (assign63470_e98286 * ((locals.var_vgssti_dn11 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn11)))) / assign63470_e98290) * locals.var_t0) + (assign63470_e98291 * locals.var_t0_dn11)), (((((((((-(locals.var_costi1_dn14 / (locals.var_costi1 * locals.var_costi1))) * locals.var_costi3) - (assign63470_e98284 * locals.var_costi3_dn14)) / (locals.var_costi3 * locals.var_costi3)) * assign63470_e98289) + (assign63470_e98286 * ((locals.var_vgssti_dn14 * locals.var_vgssti) + (locals.var_vgssti * locals.var_vgssti_dn14)))) / assign63470_e98290) * locals.var_t0) + (assign63470_e98291 * locals.var_t0_dn14)),)
    } else {
        (locals.var_psbsti, locals.var_psbsti_dn0, locals.var_psbsti_dn2, locals.var_psbsti_dn4, locals.var_psbsti_dn5, locals.var_psbsti_dn6, locals.var_psbsti_dn7, locals.var_psbsti_dn8, locals.var_psbsti_dn9, locals.var_psbsti_dn10, locals.var_psbsti_dn11, locals.var_psbsti_dn14,)
    }
};
        locals.var_psbsti = assign63470_e98295;
        locals.var_psbsti_dn0 = assign63470_e98295_d_n0;
        locals.var_psbsti_dn2 = assign63470_e98295_d_n2;
        locals.var_psbsti_dn4 = assign63470_e98295_d_n4;
        locals.var_psbsti_dn5 = assign63470_e98295_d_n5;
        locals.var_psbsti_dn6 = assign63470_e98295_d_n6;
        locals.var_psbsti_dn7 = assign63470_e98295_d_n7;
        locals.var_psbsti_dn8 = assign63470_e98295_d_n8;
        locals.var_psbsti_dn9 = assign63470_e98295_d_n9;
        locals.var_psbsti_dn10 = assign63470_e98295_d_n10;
        locals.var_psbsti_dn11 = assign63470_e98295_d_n11;
        locals.var_psbsti_dn14 = assign63470_e98295_d_n14;
        locals.var_psbsti_rv = 0.0;

        let (assign63480_e98308, assign63480_e98308_d_n0, assign63480_e98308_d_n2, assign63480_e98308_d_n4, assign63480_e98308_d_n5, assign63480_e98308_d_n6, assign63480_e98308_d_n7, assign63480_e98308_d_n8, assign63480_e98308_d_n9, assign63480_e98308_d_n10, assign63480_e98308_d_n11, assign63480_e98308_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63480_e98304: f64 = (locals.var_psbsti - locals.var_psasti);
        let assign63480_e98306: f64 = (assign63480_e98304 - 0.002);
        (assign63480_e98306, (locals.var_psbsti_dn0 - locals.var_psasti_dn0), (locals.var_psbsti_dn2 - locals.var_psasti_dn2), (locals.var_psbsti_dn4 - locals.var_psasti_dn4), (locals.var_psbsti_dn5 - locals.var_psasti_dn5), (locals.var_psbsti_dn6 - locals.var_psasti_dn6), (locals.var_psbsti_dn7 - locals.var_psasti_dn7), (locals.var_psbsti_dn8 - locals.var_psasti_dn8), (locals.var_psbsti_dn9 - locals.var_psasti_dn9), (locals.var_psbsti_dn10 - locals.var_psasti_dn10), (locals.var_psbsti_dn11 - locals.var_psasti_dn11), (locals.var_psbsti_dn14 - locals.var_psasti_dn14),)
    } else {
        (locals.var_psab, locals.var_psab_dn0, locals.var_psab_dn2, locals.var_psab_dn4, locals.var_psab_dn5, locals.var_psab_dn6, locals.var_psab_dn7, locals.var_psab_dn8, locals.var_psab_dn9, locals.var_psab_dn10, locals.var_psab_dn11, locals.var_psab_dn14,)
    }
};
        locals.var_psab = assign63480_e98308;
        locals.var_psab_dn0 = assign63480_e98308_d_n0;
        locals.var_psab_dn2 = assign63480_e98308_d_n2;
        locals.var_psab_dn4 = assign63480_e98308_d_n4;
        locals.var_psab_dn5 = assign63480_e98308_d_n5;
        locals.var_psab_dn6 = assign63480_e98308_d_n6;
        locals.var_psab_dn7 = assign63480_e98308_d_n7;
        locals.var_psab_dn8 = assign63480_e98308_d_n8;
        locals.var_psab_dn9 = assign63480_e98308_d_n9;
        locals.var_psab_dn10 = assign63480_e98308_d_n10;
        locals.var_psab_dn11 = assign63480_e98308_d_n11;
        locals.var_psab_dn14 = assign63480_e98308_d_n14;
        locals.var_psab_rv = 0.0;

        let (assign63490_e98326, assign63490_e98326_d_n0, assign63490_e98326_d_n2, assign63490_e98326_d_n4, assign63490_e98326_d_n5, assign63490_e98326_d_n6, assign63490_e98326_d_n7, assign63490_e98326_d_n8, assign63490_e98326_d_n9, assign63490_e98326_d_n10, assign63490_e98326_d_n11, assign63490_e98326_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63490_e98317: f64 = (locals.var_psab * locals.var_psab);
        let assign63490_e98320: f64 = (4.0 * 0.002);
        let assign63490_e98322: f64 = (assign63490_e98320 * locals.var_psbsti);
        let assign63490_e98323: f64 = (assign63490_e98317 + assign63490_e98322);
        let assign63490_e98324: f64 = (assign63490_e98323).sqrt();
        (assign63490_e98324, ((((locals.var_psab_dn0 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn0)) + (assign63490_e98320 * locals.var_psbsti_dn0)) / (2.0 * assign63490_e98324)), ((((locals.var_psab_dn2 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn2)) + (assign63490_e98320 * locals.var_psbsti_dn2)) / (2.0 * assign63490_e98324)), ((((locals.var_psab_dn4 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn4)) + (assign63490_e98320 * locals.var_psbsti_dn4)) / (2.0 * assign63490_e98324)), ((((locals.var_psab_dn5 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn5)) + (assign63490_e98320 * locals.var_psbsti_dn5)) / (2.0 * assign63490_e98324)), ((((locals.var_psab_dn6 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn6)) + (assign63490_e98320 * locals.var_psbsti_dn6)) / (2.0 * assign63490_e98324)), ((((locals.var_psab_dn7 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn7)) + (assign63490_e98320 * locals.var_psbsti_dn7)) / (2.0 * assign63490_e98324)), ((((locals.var_psab_dn8 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn8)) + (assign63490_e98320 * locals.var_psbsti_dn8)) / (2.0 * assign63490_e98324)), ((((locals.var_psab_dn9 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn9)) + (assign63490_e98320 * locals.var_psbsti_dn9)) / (2.0 * assign63490_e98324)), ((((locals.var_psab_dn10 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn10)) + (assign63490_e98320 * locals.var_psbsti_dn10)) / (2.0 * assign63490_e98324)), ((((locals.var_psab_dn11 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn11)) + (assign63490_e98320 * locals.var_psbsti_dn11)) / (2.0 * assign63490_e98324)), ((((locals.var_psab_dn14 * locals.var_psab) + (locals.var_psab * locals.var_psab_dn14)) + (assign63490_e98320 * locals.var_psbsti_dn14)) / (2.0 * assign63490_e98324)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign63490_e98326;
        locals.var_t0_dn0 = assign63490_e98326_d_n0;
        locals.var_t0_dn2 = assign63490_e98326_d_n2;
        locals.var_t0_dn4 = assign63490_e98326_d_n4;
        locals.var_t0_dn5 = assign63490_e98326_d_n5;
        locals.var_t0_dn6 = assign63490_e98326_d_n6;
        locals.var_t0_dn7 = assign63490_e98326_d_n7;
        locals.var_t0_dn8 = assign63490_e98326_d_n8;
        locals.var_t0_dn9 = assign63490_e98326_d_n9;
        locals.var_t0_dn10 = assign63490_e98326_d_n10;
        locals.var_t0_dn11 = assign63490_e98326_d_n11;
        locals.var_t0_dn14 = assign63490_e98326_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign63500_e98341, assign63500_e98341_d_n0, assign63500_e98341_d_n2, assign63500_e98341_d_n4, assign63500_e98341_d_n5, assign63500_e98341_d_n6, assign63500_e98341_d_n7, assign63500_e98341_d_n8, assign63500_e98341_d_n9, assign63500_e98341_d_n10, assign63500_e98341_d_n11, assign63500_e98341_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63500_e98337: f64 = (locals.var_psab + locals.var_t0);
        let assign63500_e98338: f64 = (0.5 * assign63500_e98337);
        let assign63500_e98339: f64 = (locals.var_psbsti - assign63500_e98338);
        (assign63500_e98339, (locals.var_psbsti_dn0 - (0.5 * (locals.var_psab_dn0 + locals.var_t0_dn0))), (locals.var_psbsti_dn2 - (0.5 * (locals.var_psab_dn2 + locals.var_t0_dn2))), (locals.var_psbsti_dn4 - (0.5 * (locals.var_psab_dn4 + locals.var_t0_dn4))), (locals.var_psbsti_dn5 - (0.5 * (locals.var_psab_dn5 + locals.var_t0_dn5))), (locals.var_psbsti_dn6 - (0.5 * (locals.var_psab_dn6 + locals.var_t0_dn6))), (locals.var_psbsti_dn7 - (0.5 * (locals.var_psab_dn7 + locals.var_t0_dn7))), (locals.var_psbsti_dn8 - (0.5 * (locals.var_psab_dn8 + locals.var_t0_dn8))), (locals.var_psbsti_dn9 - (0.5 * (locals.var_psab_dn9 + locals.var_t0_dn9))), (locals.var_psbsti_dn10 - (0.5 * (locals.var_psab_dn10 + locals.var_t0_dn10))), (locals.var_psbsti_dn11 - (0.5 * (locals.var_psab_dn11 + locals.var_t0_dn11))), (locals.var_psbsti_dn14 - (0.5 * (locals.var_psab_dn14 + locals.var_t0_dn14))),)
    } else {
        (locals.var_psti, locals.var_psti_dn0, locals.var_psti_dn2, locals.var_psti_dn4, locals.var_psti_dn5, locals.var_psti_dn6, locals.var_psti_dn7, locals.var_psti_dn8, locals.var_psti_dn9, locals.var_psti_dn10, locals.var_psti_dn11, locals.var_psti_dn14,)
    }
};
        locals.var_psti = assign63500_e98341;
        locals.var_psti_dn0 = assign63500_e98341_d_n0;
        locals.var_psti_dn2 = assign63500_e98341_d_n2;
        locals.var_psti_dn4 = assign63500_e98341_d_n4;
        locals.var_psti_dn5 = assign63500_e98341_d_n5;
        locals.var_psti_dn6 = assign63500_e98341_d_n6;
        locals.var_psti_dn7 = assign63500_e98341_d_n7;
        locals.var_psti_dn8 = assign63500_e98341_d_n8;
        locals.var_psti_dn9 = assign63500_e98341_d_n9;
        locals.var_psti_dn10 = assign63500_e98341_d_n10;
        locals.var_psti_dn11 = assign63500_e98341_d_n11;
        locals.var_psti_dn14 = assign63500_e98341_d_n14;
        locals.var_psti_rv = 0.0;

        let (assign63510_e98355, assign63510_e98355_d_n0, assign63510_e98355_d_n2, assign63510_e98355_d_n4, assign63510_e98355_d_n5, assign63510_e98355_d_n6, assign63510_e98355_d_n7, assign63510_e98355_d_n8, assign63510_e98355_d_n9, assign63510_e98355_d_n10, assign63510_e98355_d_n11, assign63510_e98355_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63510_e98351: f64 = (locals.var_beta * locals.var_psti);
        let assign63510_e98352: f64 = (assign63510_e98351).exp();
        let assign63510_e98353: f64 = (locals.var_costi1 * assign63510_e98352);
        (assign63510_e98353, ((locals.var_costi1_dn0 * assign63510_e98352) + (locals.var_costi1 * (assign63510_e98352 * ((locals.var_beta_dn0 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn0))))), ((locals.var_costi1_dn2 * assign63510_e98352) + (locals.var_costi1 * (assign63510_e98352 * ((locals.var_beta_dn2 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn2))))), ((locals.var_costi1_dn4 * assign63510_e98352) + (locals.var_costi1 * (assign63510_e98352 * ((locals.var_beta_dn4 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn4))))), ((locals.var_costi1_dn5 * assign63510_e98352) + (locals.var_costi1 * (assign63510_e98352 * ((locals.var_beta_dn5 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn5))))), ((locals.var_costi1_dn6 * assign63510_e98352) + (locals.var_costi1 * (assign63510_e98352 * ((locals.var_beta_dn6 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn6))))), ((locals.var_costi1_dn7 * assign63510_e98352) + (locals.var_costi1 * (assign63510_e98352 * ((locals.var_beta_dn7 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn7))))), ((locals.var_costi1_dn8 * assign63510_e98352) + (locals.var_costi1 * (assign63510_e98352 * ((locals.var_beta_dn8 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn8))))), ((locals.var_costi1_dn9 * assign63510_e98352) + (locals.var_costi1 * (assign63510_e98352 * ((locals.var_beta_dn9 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn9))))), ((locals.var_costi1_dn10 * assign63510_e98352) + (locals.var_costi1 * (assign63510_e98352 * ((locals.var_beta_dn10 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn10))))), ((locals.var_costi1_dn11 * assign63510_e98352) + (locals.var_costi1 * (assign63510_e98352 * ((locals.var_beta_dn11 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn11))))), ((locals.var_costi1_dn14 * assign63510_e98352) + (locals.var_costi1 * (assign63510_e98352 * ((locals.var_beta_dn14 * locals.var_psti) + (locals.var_beta * locals.var_psti_dn14))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign63510_e98355;
        locals.var_t0_dn0 = assign63510_e98355_d_n0;
        locals.var_t0_dn2 = assign63510_e98355_d_n2;
        locals.var_t0_dn4 = assign63510_e98355_d_n4;
        locals.var_t0_dn5 = assign63510_e98355_d_n5;
        locals.var_t0_dn6 = assign63510_e98355_d_n6;
        locals.var_t0_dn7 = assign63510_e98355_d_n7;
        locals.var_t0_dn8 = assign63510_e98355_d_n8;
        locals.var_t0_dn9 = assign63510_e98355_d_n9;
        locals.var_t0_dn10 = assign63510_e98355_d_n10;
        locals.var_t0_dn11 = assign63510_e98355_d_n11;
        locals.var_t0_dn14 = assign63510_e98355_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign63520_e98372, assign63520_e98372_d_n0, assign63520_e98372_d_n2, assign63520_e98372_d_n4, assign63520_e98372_d_n5, assign63520_e98372_d_n6, assign63520_e98372_d_n7, assign63520_e98372_d_n8, assign63520_e98372_d_n9, assign63520_e98372_d_n10, assign63520_e98372_d_n11, assign63520_e98372_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63520_e98365: f64 = (locals.var_psti - locals.var_vbsz__blk440);
        let assign63520_e98366: f64 = (locals.var_beta * assign63520_e98365);
        let assign63520_e98368: f64 = (assign63520_e98366 - 1.0);
        let assign63520_e98370: f64 = (assign63520_e98368 + locals.var_t0);
        (assign63520_e98370, (((locals.var_beta_dn0 * assign63520_e98365) + (locals.var_beta * (locals.var_psti_dn0 - locals.var_vbsz__blk440_dn0))) + locals.var_t0_dn0), (((locals.var_beta_dn2 * assign63520_e98365) + (locals.var_beta * (locals.var_psti_dn2 - locals.var_vbsz__blk440_dn2))) + locals.var_t0_dn2), (((locals.var_beta_dn4 * assign63520_e98365) + (locals.var_beta * (locals.var_psti_dn4 - locals.var_vbsz__blk440_dn4))) + locals.var_t0_dn4), (((locals.var_beta_dn5 * assign63520_e98365) + (locals.var_beta * (locals.var_psti_dn5 - locals.var_vbsz__blk440_dn5))) + locals.var_t0_dn5), (((locals.var_beta_dn6 * assign63520_e98365) + (locals.var_beta * (locals.var_psti_dn6 - locals.var_vbsz__blk440_dn6))) + locals.var_t0_dn6), (((locals.var_beta_dn7 * assign63520_e98365) + (locals.var_beta * (locals.var_psti_dn7 - locals.var_vbsz__blk440_dn7))) + locals.var_t0_dn7), (((locals.var_beta_dn8 * assign63520_e98365) + (locals.var_beta * (locals.var_psti_dn8 - locals.var_vbsz__blk440_dn8))) + locals.var_t0_dn8), (((locals.var_beta_dn9 * assign63520_e98365) + (locals.var_beta * (locals.var_psti_dn9 - locals.var_vbsz__blk440_dn9))) + locals.var_t0_dn9), (((locals.var_beta_dn10 * assign63520_e98365) + (locals.var_beta * (locals.var_psti_dn10 - locals.var_vbsz__blk440_dn10))) + locals.var_t0_dn10), (((locals.var_beta_dn11 * assign63520_e98365) + (locals.var_beta * (locals.var_psti_dn11 - locals.var_vbsz__blk440_dn11))) + locals.var_t0_dn11), (((locals.var_beta_dn14 * assign63520_e98365) + (locals.var_beta * (locals.var_psti_dn14 - locals.var_vbsz__blk440_dn14))) + locals.var_t0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63520_e98372;
        locals.var_t1_dn0 = assign63520_e98372_d_n0;
        locals.var_t1_dn2 = assign63520_e98372_d_n2;
        locals.var_t1_dn4 = assign63520_e98372_d_n4;
        locals.var_t1_dn5 = assign63520_e98372_d_n5;
        locals.var_t1_dn6 = assign63520_e98372_d_n6;
        locals.var_t1_dn7 = assign63520_e98372_d_n7;
        locals.var_t1_dn8 = assign63520_e98372_d_n8;
        locals.var_t1_dn9 = assign63520_e98372_d_n9;
        locals.var_t1_dn10 = assign63520_e98372_d_n10;
        locals.var_t1_dn11 = assign63520_e98372_d_n11;
        locals.var_t1_dn14 = assign63520_e98372_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign63530_e98390, assign63530_e98390_d_n0, assign63530_e98390_d_n2, assign63530_e98390_d_n4, assign63530_e98390_d_n5, assign63530_e98390_d_n6, assign63530_e98390_d_n7, assign63530_e98390_d_n8, assign63530_e98390_d_n9, assign63530_e98390_d_n10, assign63530_e98390_d_n11, assign63530_e98390_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63530_e98381: f64 = (locals.var_t1 * locals.var_t1);
        let assign63530_e98384: f64 = (4.0 * 0.01);
        let assign63530_e98386: f64 = (assign63530_e98384 * 0.01);
        let assign63530_e98387: f64 = (assign63530_e98381 + assign63530_e98386);
        let assign63530_e98388: f64 = (assign63530_e98387).sqrt();
        (assign63530_e98388, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign63530_e98388)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign63530_e98388)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign63530_e98388)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign63530_e98388)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign63530_e98388)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign63530_e98388)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign63530_e98388)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign63530_e98388)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign63530_e98388)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign63530_e98388)), (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign63530_e98388)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign63530_e98390;
        locals.var_tmf2_dn0 = assign63530_e98390_d_n0;
        locals.var_tmf2_dn2 = assign63530_e98390_d_n2;
        locals.var_tmf2_dn4 = assign63530_e98390_d_n4;
        locals.var_tmf2_dn5 = assign63530_e98390_d_n5;
        locals.var_tmf2_dn6 = assign63530_e98390_d_n6;
        locals.var_tmf2_dn7 = assign63530_e98390_d_n7;
        locals.var_tmf2_dn8 = assign63530_e98390_d_n8;
        locals.var_tmf2_dn9 = assign63530_e98390_d_n9;
        locals.var_tmf2_dn10 = assign63530_e98390_d_n10;
        locals.var_tmf2_dn11 = assign63530_e98390_d_n11;
        locals.var_tmf2_dn14 = assign63530_e98390_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign63540_e98405, assign63540_e98405_d_n0, assign63540_e98405_d_n2, assign63540_e98405_d_n4, assign63540_e98405_d_n5, assign63540_e98405_d_n6, assign63540_e98405_d_n7, assign63540_e98405_d_n8, assign63540_e98405_d_n9, assign63540_e98405_d_n10, assign63540_e98405_d_n11, assign63540_e98405_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63540_e98401: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign63540_e98402: f64 = (1.0 + assign63540_e98401);
        let assign63540_e98403: f64 = (0.5 * assign63540_e98402);
        (assign63540_e98403, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn14 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign63540_e98405;
        locals.var_t0_dn0 = assign63540_e98405_d_n0;
        locals.var_t0_dn2 = assign63540_e98405_d_n2;
        locals.var_t0_dn4 = assign63540_e98405_d_n4;
        locals.var_t0_dn5 = assign63540_e98405_d_n5;
        locals.var_t0_dn6 = assign63540_e98405_d_n6;
        locals.var_t0_dn7 = assign63540_e98405_d_n7;
        locals.var_t0_dn8 = assign63540_e98405_d_n8;
        locals.var_t0_dn9 = assign63540_e98405_d_n9;
        locals.var_t0_dn10 = assign63540_e98405_d_n10;
        locals.var_t0_dn11 = assign63540_e98405_d_n11;
        locals.var_t0_dn14 = assign63540_e98405_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign63550_e98418, assign63550_e98418_d_n0, assign63550_e98418_d_n2, assign63550_e98418_d_n4, assign63550_e98418_d_n5, assign63550_e98418_d_n6, assign63550_e98418_d_n7, assign63550_e98418_d_n8, assign63550_e98418_d_n9, assign63550_e98418_d_n10, assign63550_e98418_d_n11, assign63550_e98418_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63550_e98415: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign63550_e98416: f64 = (0.5 * assign63550_e98415);
        (assign63550_e98416, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63550_e98418;
        locals.var_t1_dn0 = assign63550_e98418_d_n0;
        locals.var_t1_dn2 = assign63550_e98418_d_n2;
        locals.var_t1_dn4 = assign63550_e98418_d_n4;
        locals.var_t1_dn5 = assign63550_e98418_d_n5;
        locals.var_t1_dn6 = assign63550_e98418_d_n6;
        locals.var_t1_dn7 = assign63550_e98418_d_n7;
        locals.var_t1_dn8 = assign63550_e98418_d_n8;
        locals.var_t1_dn9 = assign63550_e98418_d_n9;
        locals.var_t1_dn10 = assign63550_e98418_d_n10;
        locals.var_t1_dn11 = assign63550_e98418_d_n11;
        locals.var_t1_dn14 = assign63550_e98418_d_n14;
        locals.var_t1_rv = 0.0;

        let assign63560_e98421: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1507 = assign63560_e98421;
        locals.var_guard1507_rv = 0.0;

        let (assign63570_e98432, assign63570_e98432_d_n0, assign63570_e98432_d_n2, assign63570_e98432_d_n4, assign63570_e98432_d_n5, assign63570_e98432_d_n6, assign63570_e98432_d_n7, assign63570_e98432_d_n8, assign63570_e98432_d_n9, assign63570_e98432_d_n10, assign63570_e98432_d_n11, assign63570_e98432_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63570_e98432;
        locals.var_t1_dn0 = assign63570_e98432_d_n0;
        locals.var_t1_dn2 = assign63570_e98432_d_n2;
        locals.var_t1_dn4 = assign63570_e98432_d_n4;
        locals.var_t1_dn5 = assign63570_e98432_d_n5;
        locals.var_t1_dn6 = assign63570_e98432_d_n6;
        locals.var_t1_dn7 = assign63570_e98432_d_n7;
        locals.var_t1_dn8 = assign63570_e98432_d_n8;
        locals.var_t1_dn9 = assign63570_e98432_d_n9;
        locals.var_t1_dn10 = assign63570_e98432_d_n10;
        locals.var_t1_dn11 = assign63570_e98432_d_n11;
        locals.var_t1_dn14 = assign63570_e98432_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign63580_e98443, assign63580_e98443_d_n0, assign63580_e98443_d_n2, assign63580_e98443_d_n4, assign63580_e98443_d_n5, assign63580_e98443_d_n6, assign63580_e98443_d_n7, assign63580_e98443_d_n8, assign63580_e98443_d_n9, assign63580_e98443_d_n10, assign63580_e98443_d_n11, assign63580_e98443_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) && (locals.var_guard1507 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign63580_e98443;
        locals.var_t0_dn0 = assign63580_e98443_d_n0;
        locals.var_t0_dn2 = assign63580_e98443_d_n2;
        locals.var_t0_dn4 = assign63580_e98443_d_n4;
        locals.var_t0_dn5 = assign63580_e98443_d_n5;
        locals.var_t0_dn6 = assign63580_e98443_d_n6;
        locals.var_t0_dn7 = assign63580_e98443_d_n7;
        locals.var_t0_dn8 = assign63580_e98443_d_n8;
        locals.var_t0_dn9 = assign63580_e98443_d_n9;
        locals.var_t0_dn10 = assign63580_e98443_d_n10;
        locals.var_t0_dn11 = assign63580_e98443_d_n11;
        locals.var_t0_dn14 = assign63580_e98443_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign63590_e98454, assign63590_e98454_d_n0, assign63590_e98454_d_n2, assign63590_e98454_d_n4, assign63590_e98454_d_n5, assign63590_e98454_d_n6, assign63590_e98454_d_n7, assign63590_e98454_d_n8, assign63590_e98454_d_n9, assign63590_e98454_d_n10, assign63590_e98454_d_n11, assign63590_e98454_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63590_e98452: f64 = (locals.var_t1 + 1e-25);
        (assign63590_e98452, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63590_e98454;
        locals.var_t1_dn0 = assign63590_e98454_d_n0;
        locals.var_t1_dn2 = assign63590_e98454_d_n2;
        locals.var_t1_dn4 = assign63590_e98454_d_n4;
        locals.var_t1_dn5 = assign63590_e98454_d_n5;
        locals.var_t1_dn6 = assign63590_e98454_d_n6;
        locals.var_t1_dn7 = assign63590_e98454_d_n7;
        locals.var_t1_dn8 = assign63590_e98454_d_n8;
        locals.var_t1_dn9 = assign63590_e98454_d_n9;
        locals.var_t1_dn10 = assign63590_e98454_d_n10;
        locals.var_t1_dn11 = assign63590_e98454_d_n11;
        locals.var_t1_dn14 = assign63590_e98454_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign63600_e98464, assign63600_e98464_d_n0, assign63600_e98464_d_n2, assign63600_e98464_d_n4, assign63600_e98464_d_n5, assign63600_e98464_d_n6, assign63600_e98464_d_n7, assign63600_e98464_d_n8, assign63600_e98464_d_n9, assign63600_e98464_d_n10, assign63600_e98464_d_n11, assign63600_e98464_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63600_e98462: f64 = (locals.var_t1).sqrt();
        (assign63600_e98462, (locals.var_t1_dn0 / (2.0 * assign63600_e98462)), (locals.var_t1_dn2 / (2.0 * assign63600_e98462)), (locals.var_t1_dn4 / (2.0 * assign63600_e98462)), (locals.var_t1_dn5 / (2.0 * assign63600_e98462)), (locals.var_t1_dn6 / (2.0 * assign63600_e98462)), (locals.var_t1_dn7 / (2.0 * assign63600_e98462)), (locals.var_t1_dn8 / (2.0 * assign63600_e98462)), (locals.var_t1_dn9 / (2.0 * assign63600_e98462)), (locals.var_t1_dn10 / (2.0 * assign63600_e98462)), (locals.var_t1_dn11 / (2.0 * assign63600_e98462)), (locals.var_t1_dn14 / (2.0 * assign63600_e98462)),)
    } else {
        (locals.var_sq1sti, locals.var_sq1sti_dn0, locals.var_sq1sti_dn2, locals.var_sq1sti_dn4, locals.var_sq1sti_dn5, locals.var_sq1sti_dn6, locals.var_sq1sti_dn7, locals.var_sq1sti_dn8, locals.var_sq1sti_dn9, locals.var_sq1sti_dn10, locals.var_sq1sti_dn11, locals.var_sq1sti_dn14,)
    }
};
        locals.var_sq1sti = assign63600_e98464;
        locals.var_sq1sti_dn0 = assign63600_e98464_d_n0;
        locals.var_sq1sti_dn2 = assign63600_e98464_d_n2;
        locals.var_sq1sti_dn4 = assign63600_e98464_d_n4;
        locals.var_sq1sti_dn5 = assign63600_e98464_d_n5;
        locals.var_sq1sti_dn6 = assign63600_e98464_d_n6;
        locals.var_sq1sti_dn7 = assign63600_e98464_d_n7;
        locals.var_sq1sti_dn8 = assign63600_e98464_d_n8;
        locals.var_sq1sti_dn9 = assign63600_e98464_d_n9;
        locals.var_sq1sti_dn10 = assign63600_e98464_d_n10;
        locals.var_sq1sti_dn11 = assign63600_e98464_d_n11;
        locals.var_sq1sti_dn14 = assign63600_e98464_d_n14;
        locals.var_sq1sti_rv = 0.0;

        let (assign63610_e98479, assign63610_e98479_d_n0, assign63610_e98479_d_n2, assign63610_e98479_d_n4, assign63610_e98479_d_n5, assign63610_e98479_d_n6, assign63610_e98479_d_n7, assign63610_e98479_d_n8, assign63610_e98479_d_n9, assign63610_e98479_d_n10, assign63610_e98479_d_n11, assign63610_e98479_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63610_e98474: f64 = (locals.var_psti - locals.var_vbsz__blk440);
        let assign63610_e98475: f64 = (locals.var_beta * assign63610_e98474);
        let assign63610_e98477: f64 = (assign63610_e98475 - 1.0);
        (assign63610_e98477, ((locals.var_beta_dn0 * assign63610_e98474) + (locals.var_beta * (locals.var_psti_dn0 - locals.var_vbsz__blk440_dn0))), ((locals.var_beta_dn2 * assign63610_e98474) + (locals.var_beta * (locals.var_psti_dn2 - locals.var_vbsz__blk440_dn2))), ((locals.var_beta_dn4 * assign63610_e98474) + (locals.var_beta * (locals.var_psti_dn4 - locals.var_vbsz__blk440_dn4))), ((locals.var_beta_dn5 * assign63610_e98474) + (locals.var_beta * (locals.var_psti_dn5 - locals.var_vbsz__blk440_dn5))), ((locals.var_beta_dn6 * assign63610_e98474) + (locals.var_beta * (locals.var_psti_dn6 - locals.var_vbsz__blk440_dn6))), ((locals.var_beta_dn7 * assign63610_e98474) + (locals.var_beta * (locals.var_psti_dn7 - locals.var_vbsz__blk440_dn7))), ((locals.var_beta_dn8 * assign63610_e98474) + (locals.var_beta * (locals.var_psti_dn8 - locals.var_vbsz__blk440_dn8))), ((locals.var_beta_dn9 * assign63610_e98474) + (locals.var_beta * (locals.var_psti_dn9 - locals.var_vbsz__blk440_dn9))), ((locals.var_beta_dn10 * assign63610_e98474) + (locals.var_beta * (locals.var_psti_dn10 - locals.var_vbsz__blk440_dn10))), ((locals.var_beta_dn11 * assign63610_e98474) + (locals.var_beta * (locals.var_psti_dn11 - locals.var_vbsz__blk440_dn11))), ((locals.var_beta_dn14 * assign63610_e98474) + (locals.var_beta * (locals.var_psti_dn14 - locals.var_vbsz__blk440_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63610_e98479;
        locals.var_t1_dn0 = assign63610_e98479_d_n0;
        locals.var_t1_dn2 = assign63610_e98479_d_n2;
        locals.var_t1_dn4 = assign63610_e98479_d_n4;
        locals.var_t1_dn5 = assign63610_e98479_d_n5;
        locals.var_t1_dn6 = assign63610_e98479_d_n6;
        locals.var_t1_dn7 = assign63610_e98479_d_n7;
        locals.var_t1_dn8 = assign63610_e98479_d_n8;
        locals.var_t1_dn9 = assign63610_e98479_d_n9;
        locals.var_t1_dn10 = assign63610_e98479_d_n10;
        locals.var_t1_dn11 = assign63610_e98479_d_n11;
        locals.var_t1_dn14 = assign63610_e98479_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign63620_e98497, assign63620_e98497_d_n0, assign63620_e98497_d_n2, assign63620_e98497_d_n4, assign63620_e98497_d_n5, assign63620_e98497_d_n6, assign63620_e98497_d_n7, assign63620_e98497_d_n8, assign63620_e98497_d_n9, assign63620_e98497_d_n10, assign63620_e98497_d_n11, assign63620_e98497_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63620_e98488: f64 = (locals.var_t1 * locals.var_t1);
        let assign63620_e98491: f64 = (4.0 * 0.01);
        let assign63620_e98493: f64 = (assign63620_e98491 * 0.01);
        let assign63620_e98494: f64 = (assign63620_e98488 + assign63620_e98493);
        let assign63620_e98495: f64 = (assign63620_e98494).sqrt();
        (assign63620_e98495, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign63620_e98495)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign63620_e98495)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign63620_e98495)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign63620_e98495)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign63620_e98495)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign63620_e98495)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign63620_e98495)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign63620_e98495)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign63620_e98495)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign63620_e98495)), (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign63620_e98495)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign63620_e98497;
        locals.var_tmf2_dn0 = assign63620_e98497_d_n0;
        locals.var_tmf2_dn2 = assign63620_e98497_d_n2;
        locals.var_tmf2_dn4 = assign63620_e98497_d_n4;
        locals.var_tmf2_dn5 = assign63620_e98497_d_n5;
        locals.var_tmf2_dn6 = assign63620_e98497_d_n6;
        locals.var_tmf2_dn7 = assign63620_e98497_d_n7;
        locals.var_tmf2_dn8 = assign63620_e98497_d_n8;
        locals.var_tmf2_dn9 = assign63620_e98497_d_n9;
        locals.var_tmf2_dn10 = assign63620_e98497_d_n10;
        locals.var_tmf2_dn11 = assign63620_e98497_d_n11;
        locals.var_tmf2_dn14 = assign63620_e98497_d_n14;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_238(
        locals: &mut StampLocals,
    ) {
        let (assign63630_e98512, assign63630_e98512_d_n0, assign63630_e98512_d_n2, assign63630_e98512_d_n4, assign63630_e98512_d_n5, assign63630_e98512_d_n6, assign63630_e98512_d_n7, assign63630_e98512_d_n8, assign63630_e98512_d_n9, assign63630_e98512_d_n10, assign63630_e98512_d_n11, assign63630_e98512_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63630_e98508: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign63630_e98509: f64 = (1.0 + assign63630_e98508);
        let assign63630_e98510: f64 = (0.5 * assign63630_e98509);
        (assign63630_e98510, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn14 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign63630_e98512;
        locals.var_t0_dn0 = assign63630_e98512_d_n0;
        locals.var_t0_dn2 = assign63630_e98512_d_n2;
        locals.var_t0_dn4 = assign63630_e98512_d_n4;
        locals.var_t0_dn5 = assign63630_e98512_d_n5;
        locals.var_t0_dn6 = assign63630_e98512_d_n6;
        locals.var_t0_dn7 = assign63630_e98512_d_n7;
        locals.var_t0_dn8 = assign63630_e98512_d_n8;
        locals.var_t0_dn9 = assign63630_e98512_d_n9;
        locals.var_t0_dn10 = assign63630_e98512_d_n10;
        locals.var_t0_dn11 = assign63630_e98512_d_n11;
        locals.var_t0_dn14 = assign63630_e98512_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign63640_e98525, assign63640_e98525_d_n0, assign63640_e98525_d_n2, assign63640_e98525_d_n4, assign63640_e98525_d_n5, assign63640_e98525_d_n6, assign63640_e98525_d_n7, assign63640_e98525_d_n8, assign63640_e98525_d_n9, assign63640_e98525_d_n10, assign63640_e98525_d_n11, assign63640_e98525_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63640_e98522: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign63640_e98523: f64 = (0.5 * assign63640_e98522);
        (assign63640_e98523, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63640_e98525;
        locals.var_t1_dn0 = assign63640_e98525_d_n0;
        locals.var_t1_dn2 = assign63640_e98525_d_n2;
        locals.var_t1_dn4 = assign63640_e98525_d_n4;
        locals.var_t1_dn5 = assign63640_e98525_d_n5;
        locals.var_t1_dn6 = assign63640_e98525_d_n6;
        locals.var_t1_dn7 = assign63640_e98525_d_n7;
        locals.var_t1_dn8 = assign63640_e98525_d_n8;
        locals.var_t1_dn9 = assign63640_e98525_d_n9;
        locals.var_t1_dn10 = assign63640_e98525_d_n10;
        locals.var_t1_dn11 = assign63640_e98525_d_n11;
        locals.var_t1_dn14 = assign63640_e98525_d_n14;
        locals.var_t1_rv = 0.0;

        let assign63650_e98528: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1508 = assign63650_e98528;
        locals.var_guard1508_rv = 0.0;

        let (assign63660_e98539, assign63660_e98539_d_n0, assign63660_e98539_d_n2, assign63660_e98539_d_n4, assign63660_e98539_d_n5, assign63660_e98539_d_n6, assign63660_e98539_d_n7, assign63660_e98539_d_n8, assign63660_e98539_d_n9, assign63660_e98539_d_n10, assign63660_e98539_d_n11, assign63660_e98539_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) && (locals.var_guard1508 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63660_e98539;
        locals.var_t1_dn0 = assign63660_e98539_d_n0;
        locals.var_t1_dn2 = assign63660_e98539_d_n2;
        locals.var_t1_dn4 = assign63660_e98539_d_n4;
        locals.var_t1_dn5 = assign63660_e98539_d_n5;
        locals.var_t1_dn6 = assign63660_e98539_d_n6;
        locals.var_t1_dn7 = assign63660_e98539_d_n7;
        locals.var_t1_dn8 = assign63660_e98539_d_n8;
        locals.var_t1_dn9 = assign63660_e98539_d_n9;
        locals.var_t1_dn10 = assign63660_e98539_d_n10;
        locals.var_t1_dn11 = assign63660_e98539_d_n11;
        locals.var_t1_dn14 = assign63660_e98539_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign63670_e98550, assign63670_e98550_d_n0, assign63670_e98550_d_n2, assign63670_e98550_d_n4, assign63670_e98550_d_n5, assign63670_e98550_d_n6, assign63670_e98550_d_n7, assign63670_e98550_d_n8, assign63670_e98550_d_n9, assign63670_e98550_d_n10, assign63670_e98550_d_n11, assign63670_e98550_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) && (locals.var_guard1508 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign63670_e98550;
        locals.var_t0_dn0 = assign63670_e98550_d_n0;
        locals.var_t0_dn2 = assign63670_e98550_d_n2;
        locals.var_t0_dn4 = assign63670_e98550_d_n4;
        locals.var_t0_dn5 = assign63670_e98550_d_n5;
        locals.var_t0_dn6 = assign63670_e98550_d_n6;
        locals.var_t0_dn7 = assign63670_e98550_d_n7;
        locals.var_t0_dn8 = assign63670_e98550_d_n8;
        locals.var_t0_dn9 = assign63670_e98550_d_n9;
        locals.var_t0_dn10 = assign63670_e98550_d_n10;
        locals.var_t0_dn11 = assign63670_e98550_d_n11;
        locals.var_t0_dn14 = assign63670_e98550_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign63680_e98561, assign63680_e98561_d_n0, assign63680_e98561_d_n2, assign63680_e98561_d_n4, assign63680_e98561_d_n5, assign63680_e98561_d_n6, assign63680_e98561_d_n7, assign63680_e98561_d_n8, assign63680_e98561_d_n9, assign63680_e98561_d_n10, assign63680_e98561_d_n11, assign63680_e98561_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63680_e98559: f64 = (locals.var_t1 + 1e-25);
        (assign63680_e98559, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63680_e98561;
        locals.var_t1_dn0 = assign63680_e98561_d_n0;
        locals.var_t1_dn2 = assign63680_e98561_d_n2;
        locals.var_t1_dn4 = assign63680_e98561_d_n4;
        locals.var_t1_dn5 = assign63680_e98561_d_n5;
        locals.var_t1_dn6 = assign63680_e98561_d_n6;
        locals.var_t1_dn7 = assign63680_e98561_d_n7;
        locals.var_t1_dn8 = assign63680_e98561_d_n8;
        locals.var_t1_dn9 = assign63680_e98561_d_n9;
        locals.var_t1_dn10 = assign63680_e98561_d_n10;
        locals.var_t1_dn11 = assign63680_e98561_d_n11;
        locals.var_t1_dn14 = assign63680_e98561_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign63690_e98571, assign63690_e98571_d_n0, assign63690_e98571_d_n2, assign63690_e98571_d_n4, assign63690_e98571_d_n5, assign63690_e98571_d_n6, assign63690_e98571_d_n7, assign63690_e98571_d_n8, assign63690_e98571_d_n9, assign63690_e98571_d_n10, assign63690_e98571_d_n11, assign63690_e98571_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63690_e98569: f64 = (locals.var_t1).sqrt();
        (assign63690_e98569, (locals.var_t1_dn0 / (2.0 * assign63690_e98569)), (locals.var_t1_dn2 / (2.0 * assign63690_e98569)), (locals.var_t1_dn4 / (2.0 * assign63690_e98569)), (locals.var_t1_dn5 / (2.0 * assign63690_e98569)), (locals.var_t1_dn6 / (2.0 * assign63690_e98569)), (locals.var_t1_dn7 / (2.0 * assign63690_e98569)), (locals.var_t1_dn8 / (2.0 * assign63690_e98569)), (locals.var_t1_dn9 / (2.0 * assign63690_e98569)), (locals.var_t1_dn10 / (2.0 * assign63690_e98569)), (locals.var_t1_dn11 / (2.0 * assign63690_e98569)), (locals.var_t1_dn14 / (2.0 * assign63690_e98569)),)
    } else {
        (locals.var_sq2sti, locals.var_sq2sti_dn0, locals.var_sq2sti_dn2, locals.var_sq2sti_dn4, locals.var_sq2sti_dn5, locals.var_sq2sti_dn6, locals.var_sq2sti_dn7, locals.var_sq2sti_dn8, locals.var_sq2sti_dn9, locals.var_sq2sti_dn10, locals.var_sq2sti_dn11, locals.var_sq2sti_dn14,)
    }
};
        locals.var_sq2sti = assign63690_e98571;
        locals.var_sq2sti_dn0 = assign63690_e98571_d_n0;
        locals.var_sq2sti_dn2 = assign63690_e98571_d_n2;
        locals.var_sq2sti_dn4 = assign63690_e98571_d_n4;
        locals.var_sq2sti_dn5 = assign63690_e98571_d_n5;
        locals.var_sq2sti_dn6 = assign63690_e98571_d_n6;
        locals.var_sq2sti_dn7 = assign63690_e98571_d_n7;
        locals.var_sq2sti_dn8 = assign63690_e98571_d_n8;
        locals.var_sq2sti_dn9 = assign63690_e98571_d_n9;
        locals.var_sq2sti_dn10 = assign63690_e98571_d_n10;
        locals.var_sq2sti_dn11 = assign63690_e98571_d_n11;
        locals.var_sq2sti_dn14 = assign63690_e98571_d_n14;
        locals.var_sq2sti_rv = 0.0;

        let (assign63700_e98582, assign63700_e98582_d_n0, assign63700_e98582_d_n2, assign63700_e98582_d_n4, assign63700_e98582_d_n5, assign63700_e98582_d_n6, assign63700_e98582_d_n7, assign63700_e98582_d_n8, assign63700_e98582_d_n9, assign63700_e98582_d_n10, assign63700_e98582_d_n11, assign63700_e98582_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63700_e98580: f64 = (0.5 / locals.var_sq2sti);
        (assign63700_e98580, (-((0.5 * locals.var_sq2sti_dn0) / (locals.var_sq2sti * locals.var_sq2sti))), (-((0.5 * locals.var_sq2sti_dn2) / (locals.var_sq2sti * locals.var_sq2sti))), (-((0.5 * locals.var_sq2sti_dn4) / (locals.var_sq2sti * locals.var_sq2sti))), (-((0.5 * locals.var_sq2sti_dn5) / (locals.var_sq2sti * locals.var_sq2sti))), (-((0.5 * locals.var_sq2sti_dn6) / (locals.var_sq2sti * locals.var_sq2sti))), (-((0.5 * locals.var_sq2sti_dn7) / (locals.var_sq2sti * locals.var_sq2sti))), (-((0.5 * locals.var_sq2sti_dn8) / (locals.var_sq2sti * locals.var_sq2sti))), (-((0.5 * locals.var_sq2sti_dn9) / (locals.var_sq2sti * locals.var_sq2sti))), (-((0.5 * locals.var_sq2sti_dn10) / (locals.var_sq2sti * locals.var_sq2sti))), (-((0.5 * locals.var_sq2sti_dn11) / (locals.var_sq2sti * locals.var_sq2sti))), (-((0.5 * locals.var_sq2sti_dn14) / (locals.var_sq2sti * locals.var_sq2sti))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign63700_e98582;
        locals.var_t2_dn0 = assign63700_e98582_d_n0;
        locals.var_t2_dn2 = assign63700_e98582_d_n2;
        locals.var_t2_dn4 = assign63700_e98582_d_n4;
        locals.var_t2_dn5 = assign63700_e98582_d_n5;
        locals.var_t2_dn6 = assign63700_e98582_d_n6;
        locals.var_t2_dn7 = assign63700_e98582_d_n7;
        locals.var_t2_dn8 = assign63700_e98582_d_n8;
        locals.var_t2_dn9 = assign63700_e98582_d_n9;
        locals.var_t2_dn10 = assign63700_e98582_d_n10;
        locals.var_t2_dn11 = assign63700_e98582_d_n11;
        locals.var_t2_dn14 = assign63700_e98582_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign63710_e98595, assign63710_e98595_d_n0, assign63710_e98595_d_n2, assign63710_e98595_d_n4, assign63710_e98595_d_n5, assign63710_e98595_d_n6, assign63710_e98595_d_n7, assign63710_e98595_d_n8, assign63710_e98595_d_n9, assign63710_e98595_d_n10, assign63710_e98595_d_n11, assign63710_e98595_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63710_e98592: f64 = (locals.var_sq1sti - locals.var_sq2sti);
        let assign63710_e98593: f64 = (locals.var_costi0 * assign63710_e98592);
        (assign63710_e98593, ((locals.var_costi0_dn0 * assign63710_e98592) + (locals.var_costi0 * (locals.var_sq1sti_dn0 - locals.var_sq2sti_dn0))), ((locals.var_costi0_dn2 * assign63710_e98592) + (locals.var_costi0 * (locals.var_sq1sti_dn2 - locals.var_sq2sti_dn2))), ((locals.var_costi0_dn4 * assign63710_e98592) + (locals.var_costi0 * (locals.var_sq1sti_dn4 - locals.var_sq2sti_dn4))), ((locals.var_costi0_dn5 * assign63710_e98592) + (locals.var_costi0 * (locals.var_sq1sti_dn5 - locals.var_sq2sti_dn5))), ((locals.var_costi0_dn6 * assign63710_e98592) + (locals.var_costi0 * (locals.var_sq1sti_dn6 - locals.var_sq2sti_dn6))), ((locals.var_costi0_dn7 * assign63710_e98592) + (locals.var_costi0 * (locals.var_sq1sti_dn7 - locals.var_sq2sti_dn7))), ((locals.var_costi0_dn8 * assign63710_e98592) + (locals.var_costi0 * (locals.var_sq1sti_dn8 - locals.var_sq2sti_dn8))), ((locals.var_costi0_dn9 * assign63710_e98592) + (locals.var_costi0 * (locals.var_sq1sti_dn9 - locals.var_sq2sti_dn9))), ((locals.var_costi0_dn10 * assign63710_e98592) + (locals.var_costi0 * (locals.var_sq1sti_dn10 - locals.var_sq2sti_dn10))), ((locals.var_costi0_dn11 * assign63710_e98592) + (locals.var_costi0 * (locals.var_sq1sti_dn11 - locals.var_sq2sti_dn11))), ((locals.var_costi0_dn14 * assign63710_e98592) + (locals.var_costi0 * (locals.var_sq1sti_dn14 - locals.var_sq2sti_dn14))),)
    } else {
        (locals.var_qn0sti, locals.var_qn0sti_dn0, locals.var_qn0sti_dn2, locals.var_qn0sti_dn4, locals.var_qn0sti_dn5, locals.var_qn0sti_dn6, locals.var_qn0sti_dn7, locals.var_qn0sti_dn8, locals.var_qn0sti_dn9, locals.var_qn0sti_dn10, locals.var_qn0sti_dn11, locals.var_qn0sti_dn14,)
    }
};
        locals.var_qn0sti = assign63710_e98595;
        locals.var_qn0sti_dn0 = assign63710_e98595_d_n0;
        locals.var_qn0sti_dn2 = assign63710_e98595_d_n2;
        locals.var_qn0sti_dn4 = assign63710_e98595_d_n4;
        locals.var_qn0sti_dn5 = assign63710_e98595_d_n5;
        locals.var_qn0sti_dn6 = assign63710_e98595_d_n6;
        locals.var_qn0sti_dn7 = assign63710_e98595_d_n7;
        locals.var_qn0sti_dn8 = assign63710_e98595_d_n8;
        locals.var_qn0sti_dn9 = assign63710_e98595_d_n9;
        locals.var_qn0sti_dn10 = assign63710_e98595_d_n10;
        locals.var_qn0sti_dn11 = assign63710_e98595_d_n11;
        locals.var_qn0sti_dn14 = assign63710_e98595_d_n14;
        locals.var_qn0sti_rv = 0.0;

        let (assign63720_e98606, assign63720_e98606_d_n0, assign63720_e98606_d_n2, assign63720_e98606_d_n4, assign63720_e98606_d_n5, assign63720_e98606_d_n6, assign63720_e98606_d_n7, assign63720_e98606_d_n8, assign63720_e98606_d_n9, assign63720_e98606_d_n10, assign63720_e98606_d_n11, assign63720_e98606_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63720_e98604: f64 = (locals.var_psasti - locals.var_psti);
        (assign63720_e98604, (locals.var_psasti_dn0 - locals.var_psti_dn0), (locals.var_psasti_dn2 - locals.var_psti_dn2), (locals.var_psasti_dn4 - locals.var_psti_dn4), (locals.var_psasti_dn5 - locals.var_psti_dn5), (locals.var_psasti_dn6 - locals.var_psti_dn6), (locals.var_psasti_dn7 - locals.var_psti_dn7), (locals.var_psasti_dn8 - locals.var_psti_dn8), (locals.var_psasti_dn9 - locals.var_psti_dn9), (locals.var_psasti_dn10 - locals.var_psti_dn10), (locals.var_psasti_dn11 - locals.var_psti_dn11), (locals.var_psasti_dn14 - locals.var_psti_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63720_e98606;
        locals.var_t1_dn0 = assign63720_e98606_d_n0;
        locals.var_t1_dn2 = assign63720_e98606_d_n2;
        locals.var_t1_dn4 = assign63720_e98606_d_n4;
        locals.var_t1_dn5 = assign63720_e98606_d_n5;
        locals.var_t1_dn6 = assign63720_e98606_d_n6;
        locals.var_t1_dn7 = assign63720_e98606_d_n7;
        locals.var_t1_dn8 = assign63720_e98606_d_n8;
        locals.var_t1_dn9 = assign63720_e98606_d_n9;
        locals.var_t1_dn10 = assign63720_e98606_d_n10;
        locals.var_t1_dn11 = assign63720_e98606_d_n11;
        locals.var_t1_dn14 = assign63720_e98606_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign63730_e98624, assign63730_e98624_d_n0, assign63730_e98624_d_n2, assign63730_e98624_d_n4, assign63730_e98624_d_n5, assign63730_e98624_d_n6, assign63730_e98624_d_n7, assign63730_e98624_d_n8, assign63730_e98624_d_n9, assign63730_e98624_d_n10, assign63730_e98624_d_n11, assign63730_e98624_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63730_e98615: f64 = (locals.var_t1 * locals.var_t1);
        let assign63730_e98618: f64 = (4.0 * 0.1);
        let assign63730_e98620: f64 = (assign63730_e98618 * 0.1);
        let assign63730_e98621: f64 = (assign63730_e98615 + assign63730_e98620);
        let assign63730_e98622: f64 = (assign63730_e98621).sqrt();
        (assign63730_e98622, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign63730_e98622)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign63730_e98622)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign63730_e98622)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign63730_e98622)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign63730_e98622)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign63730_e98622)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign63730_e98622)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign63730_e98622)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign63730_e98622)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign63730_e98622)), (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign63730_e98622)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign63730_e98624;
        locals.var_tmf2_dn0 = assign63730_e98624_d_n0;
        locals.var_tmf2_dn2 = assign63730_e98624_d_n2;
        locals.var_tmf2_dn4 = assign63730_e98624_d_n4;
        locals.var_tmf2_dn5 = assign63730_e98624_d_n5;
        locals.var_tmf2_dn6 = assign63730_e98624_d_n6;
        locals.var_tmf2_dn7 = assign63730_e98624_d_n7;
        locals.var_tmf2_dn8 = assign63730_e98624_d_n8;
        locals.var_tmf2_dn9 = assign63730_e98624_d_n9;
        locals.var_tmf2_dn10 = assign63730_e98624_d_n10;
        locals.var_tmf2_dn11 = assign63730_e98624_d_n11;
        locals.var_tmf2_dn14 = assign63730_e98624_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign63740_e98639, assign63740_e98639_d_n0, assign63740_e98639_d_n2, assign63740_e98639_d_n4, assign63740_e98639_d_n5, assign63740_e98639_d_n6, assign63740_e98639_d_n7, assign63740_e98639_d_n8, assign63740_e98639_d_n9, assign63740_e98639_d_n10, assign63740_e98639_d_n11, assign63740_e98639_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63740_e98635: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign63740_e98636: f64 = (1.0 + assign63740_e98635);
        let assign63740_e98637: f64 = (0.5 * assign63740_e98636);
        (assign63740_e98637, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn14 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign63740_e98639;
        locals.var_t2_dn0 = assign63740_e98639_d_n0;
        locals.var_t2_dn2 = assign63740_e98639_d_n2;
        locals.var_t2_dn4 = assign63740_e98639_d_n4;
        locals.var_t2_dn5 = assign63740_e98639_d_n5;
        locals.var_t2_dn6 = assign63740_e98639_d_n6;
        locals.var_t2_dn7 = assign63740_e98639_d_n7;
        locals.var_t2_dn8 = assign63740_e98639_d_n8;
        locals.var_t2_dn9 = assign63740_e98639_d_n9;
        locals.var_t2_dn10 = assign63740_e98639_d_n10;
        locals.var_t2_dn11 = assign63740_e98639_d_n11;
        locals.var_t2_dn14 = assign63740_e98639_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign63750_e98652, assign63750_e98652_d_n0, assign63750_e98652_d_n2, assign63750_e98652_d_n4, assign63750_e98652_d_n5, assign63750_e98652_d_n6, assign63750_e98652_d_n7, assign63750_e98652_d_n8, assign63750_e98652_d_n9, assign63750_e98652_d_n10, assign63750_e98652_d_n11, assign63750_e98652_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63750_e98649: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign63750_e98650: f64 = (0.5 * assign63750_e98649);
        (assign63750_e98650, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63750_e98652;
        locals.var_t1_dn0 = assign63750_e98652_d_n0;
        locals.var_t1_dn2 = assign63750_e98652_d_n2;
        locals.var_t1_dn4 = assign63750_e98652_d_n4;
        locals.var_t1_dn5 = assign63750_e98652_d_n5;
        locals.var_t1_dn6 = assign63750_e98652_d_n6;
        locals.var_t1_dn7 = assign63750_e98652_d_n7;
        locals.var_t1_dn8 = assign63750_e98652_d_n8;
        locals.var_t1_dn9 = assign63750_e98652_d_n9;
        locals.var_t1_dn10 = assign63750_e98652_d_n10;
        locals.var_t1_dn11 = assign63750_e98652_d_n11;
        locals.var_t1_dn14 = assign63750_e98652_d_n14;
        locals.var_t1_rv = 0.0;

        let assign63760_e98655: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1509 = assign63760_e98655;
        locals.var_guard1509_rv = 0.0;

        let (assign63770_e98666, assign63770_e98666_d_n0, assign63770_e98666_d_n2, assign63770_e98666_d_n4, assign63770_e98666_d_n5, assign63770_e98666_d_n6, assign63770_e98666_d_n7, assign63770_e98666_d_n8, assign63770_e98666_d_n9, assign63770_e98666_d_n10, assign63770_e98666_d_n11, assign63770_e98666_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) && (locals.var_guard1509 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63770_e98666;
        locals.var_t1_dn0 = assign63770_e98666_d_n0;
        locals.var_t1_dn2 = assign63770_e98666_d_n2;
        locals.var_t1_dn4 = assign63770_e98666_d_n4;
        locals.var_t1_dn5 = assign63770_e98666_d_n5;
        locals.var_t1_dn6 = assign63770_e98666_d_n6;
        locals.var_t1_dn7 = assign63770_e98666_d_n7;
        locals.var_t1_dn8 = assign63770_e98666_d_n8;
        locals.var_t1_dn9 = assign63770_e98666_d_n9;
        locals.var_t1_dn10 = assign63770_e98666_d_n10;
        locals.var_t1_dn11 = assign63770_e98666_d_n11;
        locals.var_t1_dn14 = assign63770_e98666_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign63780_e98677, assign63780_e98677_d_n0, assign63780_e98677_d_n2, assign63780_e98677_d_n4, assign63780_e98677_d_n5, assign63780_e98677_d_n6, assign63780_e98677_d_n7, assign63780_e98677_d_n8, assign63780_e98677_d_n9, assign63780_e98677_d_n10, assign63780_e98677_d_n11, assign63780_e98677_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) && (locals.var_guard1509 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign63780_e98677;
        locals.var_t2_dn0 = assign63780_e98677_d_n0;
        locals.var_t2_dn2 = assign63780_e98677_d_n2;
        locals.var_t2_dn4 = assign63780_e98677_d_n4;
        locals.var_t2_dn5 = assign63780_e98677_d_n5;
        locals.var_t2_dn6 = assign63780_e98677_d_n6;
        locals.var_t2_dn7 = assign63780_e98677_d_n7;
        locals.var_t2_dn8 = assign63780_e98677_d_n8;
        locals.var_t2_dn9 = assign63780_e98677_d_n9;
        locals.var_t2_dn10 = assign63780_e98677_d_n10;
        locals.var_t2_dn11 = assign63780_e98677_d_n11;
        locals.var_t2_dn14 = assign63780_e98677_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign63790_e98688, assign63790_e98688_d_n0, assign63790_e98688_d_n2, assign63790_e98688_d_n4, assign63790_e98688_d_n5, assign63790_e98688_d_n6, assign63790_e98688_d_n7, assign63790_e98688_d_n8, assign63790_e98688_d_n9, assign63790_e98688_d_n10, assign63790_e98688_d_n11, assign63790_e98688_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63790_e98686: f64 = (locals.var_t1 + 1e-25);
        (assign63790_e98686, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign63790_e98688;
        locals.var_t1_dn0 = assign63790_e98688_d_n0;
        locals.var_t1_dn2 = assign63790_e98688_d_n2;
        locals.var_t1_dn4 = assign63790_e98688_d_n4;
        locals.var_t1_dn5 = assign63790_e98688_d_n5;
        locals.var_t1_dn6 = assign63790_e98688_d_n6;
        locals.var_t1_dn7 = assign63790_e98688_d_n7;
        locals.var_t1_dn8 = assign63790_e98688_d_n8;
        locals.var_t1_dn9 = assign63790_e98688_d_n9;
        locals.var_t1_dn10 = assign63790_e98688_d_n10;
        locals.var_t1_dn11 = assign63790_e98688_d_n11;
        locals.var_t1_dn14 = assign63790_e98688_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign63800_e98699, assign63800_e98699_d_n0, assign63800_e98699_d_n2, assign63800_e98699_d_n4, assign63800_e98699_d_n5, assign63800_e98699_d_n6, assign63800_e98699_d_n7, assign63800_e98699_d_n8, assign63800_e98699_d_n9, assign63800_e98699_d_n10, assign63800_e98699_d_n11, assign63800_e98699_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63800_e98697: f64 = (locals.var_vds / locals.var_t1);
        (assign63800_e98697, (((locals.var_vds_dn0 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn0)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn2 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn2)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn4 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn5 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn6 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn7 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn8 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn9 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn10 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn11 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vds_dn14 * locals.var_t1) - (locals.var_vds * locals.var_t1_dn14)) / (locals.var_t1 * locals.var_t1)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign63800_e98699;
        locals.var_tx_dn0 = assign63800_e98699_d_n0;
        locals.var_tx_dn2 = assign63800_e98699_d_n2;
        locals.var_tx_dn4 = assign63800_e98699_d_n4;
        locals.var_tx_dn5 = assign63800_e98699_d_n5;
        locals.var_tx_dn6 = assign63800_e98699_d_n6;
        locals.var_tx_dn7 = assign63800_e98699_d_n7;
        locals.var_tx_dn8 = assign63800_e98699_d_n8;
        locals.var_tx_dn9 = assign63800_e98699_d_n9;
        locals.var_tx_dn10 = assign63800_e98699_d_n10;
        locals.var_tx_dn11 = assign63800_e98699_d_n11;
        locals.var_tx_dn14 = assign63800_e98699_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign63810_e98712, assign63810_e98712_d_n0, assign63810_e98712_d_n2, assign63810_e98712_d_n4, assign63810_e98712_d_n5, assign63810_e98712_d_n6, assign63810_e98712_d_n7, assign63810_e98712_d_n8, assign63810_e98712_d_n9, assign63810_e98712_d_n10, assign63810_e98712_d_n11, assign63810_e98712_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63810_e98709: f64 = (locals.var_t1 * locals.var_t1);
        let assign63810_e98710: f64 = (1.0 / assign63810_e98709);
        (assign63810_e98710, (-(((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (assign63810_e98709 * assign63810_e98709))), (-(((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (assign63810_e98709 * assign63810_e98709))), (-(((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (assign63810_e98709 * assign63810_e98709))), (-(((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (assign63810_e98709 * assign63810_e98709))), (-(((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (assign63810_e98709 * assign63810_e98709))), (-(((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (assign63810_e98709 * assign63810_e98709))), (-(((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (assign63810_e98709 * assign63810_e98709))), (-(((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (assign63810_e98709 * assign63810_e98709))), (-(((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (assign63810_e98709 * assign63810_e98709))), (-(((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (assign63810_e98709 * assign63810_e98709))), (-(((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (assign63810_e98709 * assign63810_e98709))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign63810_e98712;
        locals.var_t2_dn0 = assign63810_e98712_d_n0;
        locals.var_t2_dn2 = assign63810_e98712_d_n2;
        locals.var_t2_dn4 = assign63810_e98712_d_n4;
        locals.var_t2_dn5 = assign63810_e98712_d_n5;
        locals.var_t2_dn6 = assign63810_e98712_d_n6;
        locals.var_t2_dn7 = assign63810_e98712_d_n7;
        locals.var_t2_dn8 = assign63810_e98712_d_n8;
        locals.var_t2_dn9 = assign63810_e98712_d_n9;
        locals.var_t2_dn10 = assign63810_e98712_d_n10;
        locals.var_t2_dn11 = assign63810_e98712_d_n11;
        locals.var_t2_dn14 = assign63810_e98712_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign63820_e98723, assign63820_e98723_d_n0, assign63820_e98723_d_n2, assign63820_e98723_d_n4, assign63820_e98723_d_n5, assign63820_e98723_d_n6, assign63820_e98723_d_n7, assign63820_e98723_d_n8, assign63820_e98723_d_n9, assign63820_e98723_d_n10, assign63820_e98723_d_n11, assign63820_e98723_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63820_e98721: f64 = (locals.var_tx * locals.var_tx);
        (assign63820_e98721, ((locals.var_tx_dn0 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn0)), ((locals.var_tx_dn2 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn2)), ((locals.var_tx_dn4 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn4)), ((locals.var_tx_dn5 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn5)), ((locals.var_tx_dn6 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn6)), ((locals.var_tx_dn7 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn7)), ((locals.var_tx_dn8 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn8)), ((locals.var_tx_dn9 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn9)), ((locals.var_tx_dn10 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn10)), ((locals.var_tx_dn11 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn11)), ((locals.var_tx_dn14 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign63820_e98723;
        locals.var_x2_dn0 = assign63820_e98723_d_n0;
        locals.var_x2_dn2 = assign63820_e98723_d_n2;
        locals.var_x2_dn4 = assign63820_e98723_d_n4;
        locals.var_x2_dn5 = assign63820_e98723_d_n5;
        locals.var_x2_dn6 = assign63820_e98723_d_n6;
        locals.var_x2_dn7 = assign63820_e98723_d_n7;
        locals.var_x2_dn8 = assign63820_e98723_d_n8;
        locals.var_x2_dn9 = assign63820_e98723_d_n9;
        locals.var_x2_dn10 = assign63820_e98723_d_n10;
        locals.var_x2_dn11 = assign63820_e98723_d_n11;
        locals.var_x2_dn14 = assign63820_e98723_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign63830_e98734, assign63830_e98734_d_n0, assign63830_e98734_d_n2, assign63830_e98734_d_n4, assign63830_e98734_d_n5, assign63830_e98734_d_n6, assign63830_e98734_d_n7, assign63830_e98734_d_n8, assign63830_e98734_d_n9, assign63830_e98734_d_n10, assign63830_e98734_d_n11, assign63830_e98734_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63830_e98732: f64 = 1.0;
        (assign63830_e98732, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign63830_e98734;
        locals.var_xmax2_dn0 = assign63830_e98734_d_n0;
        locals.var_xmax2_dn2 = assign63830_e98734_d_n2;
        locals.var_xmax2_dn4 = assign63830_e98734_d_n4;
        locals.var_xmax2_dn5 = assign63830_e98734_d_n5;
        locals.var_xmax2_dn6 = assign63830_e98734_d_n6;
        locals.var_xmax2_dn7 = assign63830_e98734_d_n7;
        locals.var_xmax2_dn8 = assign63830_e98734_d_n8;
        locals.var_xmax2_dn9 = assign63830_e98734_d_n9;
        locals.var_xmax2_dn10 = assign63830_e98734_d_n10;
        locals.var_xmax2_dn11 = assign63830_e98734_d_n11;
        locals.var_xmax2_dn14 = assign63830_e98734_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign63840_e98743, assign63840_e98743_d_n0, assign63840_e98743_d_n2, assign63840_e98743_d_n4, assign63840_e98743_d_n5, assign63840_e98743_d_n6, assign63840_e98743_d_n7, assign63840_e98743_d_n8, assign63840_e98743_d_n9, assign63840_e98743_d_n10, assign63840_e98743_d_n11, assign63840_e98743_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign63840_e98743;
        locals.var_xp_dn0 = assign63840_e98743_d_n0;
        locals.var_xp_dn2 = assign63840_e98743_d_n2;
        locals.var_xp_dn4 = assign63840_e98743_d_n4;
        locals.var_xp_dn5 = assign63840_e98743_d_n5;
        locals.var_xp_dn6 = assign63840_e98743_d_n6;
        locals.var_xp_dn7 = assign63840_e98743_d_n7;
        locals.var_xp_dn8 = assign63840_e98743_d_n8;
        locals.var_xp_dn9 = assign63840_e98743_d_n9;
        locals.var_xp_dn10 = assign63840_e98743_d_n10;
        locals.var_xp_dn11 = assign63840_e98743_d_n11;
        locals.var_xp_dn14 = assign63840_e98743_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign63850_e98752, assign63850_e98752_d_n0, assign63850_e98752_d_n2, assign63850_e98752_d_n4, assign63850_e98752_d_n5, assign63850_e98752_d_n6, assign63850_e98752_d_n7, assign63850_e98752_d_n8, assign63850_e98752_d_n9, assign63850_e98752_d_n10, assign63850_e98752_d_n11, assign63850_e98752_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign63850_e98752;
        locals.var_xmp_dn0 = assign63850_e98752_d_n0;
        locals.var_xmp_dn2 = assign63850_e98752_d_n2;
        locals.var_xmp_dn4 = assign63850_e98752_d_n4;
        locals.var_xmp_dn5 = assign63850_e98752_d_n5;
        locals.var_xmp_dn6 = assign63850_e98752_d_n6;
        locals.var_xmp_dn7 = assign63850_e98752_d_n7;
        locals.var_xmp_dn8 = assign63850_e98752_d_n8;
        locals.var_xmp_dn9 = assign63850_e98752_d_n9;
        locals.var_xmp_dn10 = assign63850_e98752_d_n10;
        locals.var_xmp_dn11 = assign63850_e98752_d_n11;
        locals.var_xmp_dn14 = assign63850_e98752_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign63860_e98761,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign63860_e98761;
        locals.var_m0_rv = 0.0;

        let (assign63870_e98770,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign63870_e98770;
        locals.var_mm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_239(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign63880_e98779, assign63880_e98779_d_n0, assign63880_e98779_d_n2, assign63880_e98779_d_n4, assign63880_e98779_d_n5, assign63880_e98779_d_n6, assign63880_e98779_d_n7, assign63880_e98779_d_n8, assign63880_e98779_d_n9, assign63880_e98779_d_n10, assign63880_e98779_d_n11, assign63880_e98779_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign63880_e98779;
        locals.var_arg_dn0 = assign63880_e98779_d_n0;
        locals.var_arg_dn2 = assign63880_e98779_d_n2;
        locals.var_arg_dn4 = assign63880_e98779_d_n4;
        locals.var_arg_dn5 = assign63880_e98779_d_n5;
        locals.var_arg_dn6 = assign63880_e98779_d_n6;
        locals.var_arg_dn7 = assign63880_e98779_d_n7;
        locals.var_arg_dn8 = assign63880_e98779_d_n8;
        locals.var_arg_dn9 = assign63880_e98779_d_n9;
        locals.var_arg_dn10 = assign63880_e98779_d_n10;
        locals.var_arg_dn11 = assign63880_e98779_d_n11;
        locals.var_arg_dn14 = assign63880_e98779_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign63890_e98788, assign63890_e98788_d_n0, assign63890_e98788_d_n2, assign63890_e98788_d_n4, assign63890_e98788_d_n5, assign63890_e98788_d_n6, assign63890_e98788_d_n7, assign63890_e98788_d_n8, assign63890_e98788_d_n9, assign63890_e98788_d_n10, assign63890_e98788_d_n11, assign63890_e98788_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign63890_e98788;
        locals.var_dnm_dn0 = assign63890_e98788_d_n0;
        locals.var_dnm_dn2 = assign63890_e98788_d_n2;
        locals.var_dnm_dn4 = assign63890_e98788_d_n4;
        locals.var_dnm_dn5 = assign63890_e98788_d_n5;
        locals.var_dnm_dn6 = assign63890_e98788_d_n6;
        locals.var_dnm_dn7 = assign63890_e98788_d_n7;
        locals.var_dnm_dn8 = assign63890_e98788_d_n8;
        locals.var_dnm_dn9 = assign63890_e98788_d_n9;
        locals.var_dnm_dn10 = assign63890_e98788_d_n10;
        locals.var_dnm_dn11 = assign63890_e98788_d_n11;
        locals.var_dnm_dn14 = assign63890_e98788_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign63900_e98799, assign63900_e98799_d_n0, assign63900_e98799_d_n2, assign63900_e98799_d_n4, assign63900_e98799_d_n5, assign63900_e98799_d_n6, assign63900_e98799_d_n7, assign63900_e98799_d_n8, assign63900_e98799_d_n9, assign63900_e98799_d_n10, assign63900_e98799_d_n11, assign63900_e98799_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63900_e98797: f64 = (locals.var_xp * locals.var_x2);
        (assign63900_e98797, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign63900_e98799;
        locals.var_xp_dn0 = assign63900_e98799_d_n0;
        locals.var_xp_dn2 = assign63900_e98799_d_n2;
        locals.var_xp_dn4 = assign63900_e98799_d_n4;
        locals.var_xp_dn5 = assign63900_e98799_d_n5;
        locals.var_xp_dn6 = assign63900_e98799_d_n6;
        locals.var_xp_dn7 = assign63900_e98799_d_n7;
        locals.var_xp_dn8 = assign63900_e98799_d_n8;
        locals.var_xp_dn9 = assign63900_e98799_d_n9;
        locals.var_xp_dn10 = assign63900_e98799_d_n10;
        locals.var_xp_dn11 = assign63900_e98799_d_n11;
        locals.var_xp_dn14 = assign63900_e98799_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign63910_e98810, assign63910_e98810_d_n0, assign63910_e98810_d_n2, assign63910_e98810_d_n4, assign63910_e98810_d_n5, assign63910_e98810_d_n6, assign63910_e98810_d_n7, assign63910_e98810_d_n8, assign63910_e98810_d_n9, assign63910_e98810_d_n10, assign63910_e98810_d_n11, assign63910_e98810_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63910_e98808: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign63910_e98808, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign63910_e98810;
        locals.var_xmp_dn0 = assign63910_e98810_d_n0;
        locals.var_xmp_dn2 = assign63910_e98810_d_n2;
        locals.var_xmp_dn4 = assign63910_e98810_d_n4;
        locals.var_xmp_dn5 = assign63910_e98810_d_n5;
        locals.var_xmp_dn6 = assign63910_e98810_d_n6;
        locals.var_xmp_dn7 = assign63910_e98810_d_n7;
        locals.var_xmp_dn8 = assign63910_e98810_d_n8;
        locals.var_xmp_dn9 = assign63910_e98810_d_n9;
        locals.var_xmp_dn10 = assign63910_e98810_d_n10;
        locals.var_xmp_dn11 = assign63910_e98810_d_n11;
        locals.var_xmp_dn14 = assign63910_e98810_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign63920_e98821, assign63920_e98821_d_n0, assign63920_e98821_d_n2, assign63920_e98821_d_n4, assign63920_e98821_d_n5, assign63920_e98821_d_n6, assign63920_e98821_d_n7, assign63920_e98821_d_n8, assign63920_e98821_d_n9, assign63920_e98821_d_n10, assign63920_e98821_d_n11, assign63920_e98821_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63920_e98819: f64 = (locals.var_xp * locals.var_x2);
        (assign63920_e98819, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign63920_e98821;
        locals.var_xp_dn0 = assign63920_e98821_d_n0;
        locals.var_xp_dn2 = assign63920_e98821_d_n2;
        locals.var_xp_dn4 = assign63920_e98821_d_n4;
        locals.var_xp_dn5 = assign63920_e98821_d_n5;
        locals.var_xp_dn6 = assign63920_e98821_d_n6;
        locals.var_xp_dn7 = assign63920_e98821_d_n7;
        locals.var_xp_dn8 = assign63920_e98821_d_n8;
        locals.var_xp_dn9 = assign63920_e98821_d_n9;
        locals.var_xp_dn10 = assign63920_e98821_d_n10;
        locals.var_xp_dn11 = assign63920_e98821_d_n11;
        locals.var_xp_dn14 = assign63920_e98821_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign63930_e98832, assign63930_e98832_d_n0, assign63930_e98832_d_n2, assign63930_e98832_d_n4, assign63930_e98832_d_n5, assign63930_e98832_d_n6, assign63930_e98832_d_n7, assign63930_e98832_d_n8, assign63930_e98832_d_n9, assign63930_e98832_d_n10, assign63930_e98832_d_n11, assign63930_e98832_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63930_e98830: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign63930_e98830, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign63930_e98832;
        locals.var_xmp_dn0 = assign63930_e98832_d_n0;
        locals.var_xmp_dn2 = assign63930_e98832_d_n2;
        locals.var_xmp_dn4 = assign63930_e98832_d_n4;
        locals.var_xmp_dn5 = assign63930_e98832_d_n5;
        locals.var_xmp_dn6 = assign63930_e98832_d_n6;
        locals.var_xmp_dn7 = assign63930_e98832_d_n7;
        locals.var_xmp_dn8 = assign63930_e98832_d_n8;
        locals.var_xmp_dn9 = assign63930_e98832_d_n9;
        locals.var_xmp_dn10 = assign63930_e98832_d_n10;
        locals.var_xmp_dn11 = assign63930_e98832_d_n11;
        locals.var_xmp_dn14 = assign63930_e98832_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign63940_e98843, assign63940_e98843_d_n0, assign63940_e98843_d_n2, assign63940_e98843_d_n4, assign63940_e98843_d_n5, assign63940_e98843_d_n6, assign63940_e98843_d_n7, assign63940_e98843_d_n8, assign63940_e98843_d_n9, assign63940_e98843_d_n10, assign63940_e98843_d_n11, assign63940_e98843_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63940_e98841: f64 = (locals.var_xp * locals.var_x2);
        (assign63940_e98841, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign63940_e98843;
        locals.var_xp_dn0 = assign63940_e98843_d_n0;
        locals.var_xp_dn2 = assign63940_e98843_d_n2;
        locals.var_xp_dn4 = assign63940_e98843_d_n4;
        locals.var_xp_dn5 = assign63940_e98843_d_n5;
        locals.var_xp_dn6 = assign63940_e98843_d_n6;
        locals.var_xp_dn7 = assign63940_e98843_d_n7;
        locals.var_xp_dn8 = assign63940_e98843_d_n8;
        locals.var_xp_dn9 = assign63940_e98843_d_n9;
        locals.var_xp_dn10 = assign63940_e98843_d_n10;
        locals.var_xp_dn11 = assign63940_e98843_d_n11;
        locals.var_xp_dn14 = assign63940_e98843_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign63950_e98854, assign63950_e98854_d_n0, assign63950_e98854_d_n2, assign63950_e98854_d_n4, assign63950_e98854_d_n5, assign63950_e98854_d_n6, assign63950_e98854_d_n7, assign63950_e98854_d_n8, assign63950_e98854_d_n9, assign63950_e98854_d_n10, assign63950_e98854_d_n11, assign63950_e98854_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63950_e98852: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign63950_e98852, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign63950_e98854;
        locals.var_xmp_dn0 = assign63950_e98854_d_n0;
        locals.var_xmp_dn2 = assign63950_e98854_d_n2;
        locals.var_xmp_dn4 = assign63950_e98854_d_n4;
        locals.var_xmp_dn5 = assign63950_e98854_d_n5;
        locals.var_xmp_dn6 = assign63950_e98854_d_n6;
        locals.var_xmp_dn7 = assign63950_e98854_d_n7;
        locals.var_xmp_dn8 = assign63950_e98854_d_n8;
        locals.var_xmp_dn9 = assign63950_e98854_d_n9;
        locals.var_xmp_dn10 = assign63950_e98854_d_n10;
        locals.var_xmp_dn11 = assign63950_e98854_d_n11;
        locals.var_xmp_dn14 = assign63950_e98854_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign63960_e98865, assign63960_e98865_d_n0, assign63960_e98865_d_n2, assign63960_e98865_d_n4, assign63960_e98865_d_n5, assign63960_e98865_d_n6, assign63960_e98865_d_n7, assign63960_e98865_d_n8, assign63960_e98865_d_n9, assign63960_e98865_d_n10, assign63960_e98865_d_n11, assign63960_e98865_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63960_e98863: f64 = (locals.var_xp * locals.var_x2);
        (assign63960_e98863, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign63960_e98865;
        locals.var_xp_dn0 = assign63960_e98865_d_n0;
        locals.var_xp_dn2 = assign63960_e98865_d_n2;
        locals.var_xp_dn4 = assign63960_e98865_d_n4;
        locals.var_xp_dn5 = assign63960_e98865_d_n5;
        locals.var_xp_dn6 = assign63960_e98865_d_n6;
        locals.var_xp_dn7 = assign63960_e98865_d_n7;
        locals.var_xp_dn8 = assign63960_e98865_d_n8;
        locals.var_xp_dn9 = assign63960_e98865_d_n9;
        locals.var_xp_dn10 = assign63960_e98865_d_n10;
        locals.var_xp_dn11 = assign63960_e98865_d_n11;
        locals.var_xp_dn14 = assign63960_e98865_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign63970_e98876, assign63970_e98876_d_n0, assign63970_e98876_d_n2, assign63970_e98876_d_n4, assign63970_e98876_d_n5, assign63970_e98876_d_n6, assign63970_e98876_d_n7, assign63970_e98876_d_n8, assign63970_e98876_d_n9, assign63970_e98876_d_n10, assign63970_e98876_d_n11, assign63970_e98876_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63970_e98874: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign63970_e98874, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign63970_e98876;
        locals.var_xmp_dn0 = assign63970_e98876_d_n0;
        locals.var_xmp_dn2 = assign63970_e98876_d_n2;
        locals.var_xmp_dn4 = assign63970_e98876_d_n4;
        locals.var_xmp_dn5 = assign63970_e98876_d_n5;
        locals.var_xmp_dn6 = assign63970_e98876_d_n6;
        locals.var_xmp_dn7 = assign63970_e98876_d_n7;
        locals.var_xmp_dn8 = assign63970_e98876_d_n8;
        locals.var_xmp_dn9 = assign63970_e98876_d_n9;
        locals.var_xmp_dn10 = assign63970_e98876_d_n10;
        locals.var_xmp_dn11 = assign63970_e98876_d_n11;
        locals.var_xmp_dn14 = assign63970_e98876_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign63980_e98887, assign63980_e98887_d_n0, assign63980_e98887_d_n2, assign63980_e98887_d_n4, assign63980_e98887_d_n5, assign63980_e98887_d_n6, assign63980_e98887_d_n7, assign63980_e98887_d_n8, assign63980_e98887_d_n9, assign63980_e98887_d_n10, assign63980_e98887_d_n11, assign63980_e98887_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign63980_e98885: f64 = (locals.var_xp + locals.var_xmp);
        (assign63980_e98885, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign63980_e98887;
        locals.var_arg_dn0 = assign63980_e98887_d_n0;
        locals.var_arg_dn2 = assign63980_e98887_d_n2;
        locals.var_arg_dn4 = assign63980_e98887_d_n4;
        locals.var_arg_dn5 = assign63980_e98887_d_n5;
        locals.var_arg_dn6 = assign63980_e98887_d_n6;
        locals.var_arg_dn7 = assign63980_e98887_d_n7;
        locals.var_arg_dn8 = assign63980_e98887_d_n8;
        locals.var_arg_dn9 = assign63980_e98887_d_n9;
        locals.var_arg_dn10 = assign63980_e98887_d_n10;
        locals.var_arg_dn11 = assign63980_e98887_d_n11;
        locals.var_arg_dn14 = assign63980_e98887_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign63990_e98896, assign63990_e98896_d_n0, assign63990_e98896_d_n2, assign63990_e98896_d_n4, assign63990_e98896_d_n5, assign63990_e98896_d_n6, assign63990_e98896_d_n7, assign63990_e98896_d_n8, assign63990_e98896_d_n9, assign63990_e98896_d_n10, assign63990_e98896_d_n11, assign63990_e98896_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign63990_e98896;
        locals.var_dnm_dn0 = assign63990_e98896_d_n0;
        locals.var_dnm_dn2 = assign63990_e98896_d_n2;
        locals.var_dnm_dn4 = assign63990_e98896_d_n4;
        locals.var_dnm_dn5 = assign63990_e98896_d_n5;
        locals.var_dnm_dn6 = assign63990_e98896_d_n6;
        locals.var_dnm_dn7 = assign63990_e98896_d_n7;
        locals.var_dnm_dn8 = assign63990_e98896_d_n8;
        locals.var_dnm_dn9 = assign63990_e98896_d_n9;
        locals.var_dnm_dn10 = assign63990_e98896_d_n10;
        locals.var_dnm_dn11 = assign63990_e98896_d_n11;
        locals.var_dnm_dn14 = assign63990_e98896_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign64000_e98911: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1510 = assign64000_e98911;
        locals.var_guard1510_rv = 0.0;

        let assign64010_e98914: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1511 = assign64010_e98914;
        locals.var_guard1511_rv = 0.0;

        let (assign64020_e98927,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) && (locals.var_guard1510 != 0.0)) && (locals.var_guard1511 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign64020_e98927;
        locals.var_mm_rv = 0.0;

        let assign64030_e98930: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1512 = assign64030_e98930;
        locals.var_guard1512_rv = 0.0;

        let (assign64040_e98946,) = {
    if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) && (locals.var_guard1510 != 0.0)) && (locals.var_guard1511 == 0.0)) && (locals.var_guard1512 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign64040_e98946;
        locals.var_mm_rv = 0.0;

        let assign64050_e98949: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1513 = assign64050_e98949;
        locals.var_guard1513_rv = 0.0;

        let (assign64060_e98968,) = {
    if (((((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) && (locals.var_guard1510 != 0.0)) && (locals.var_guard1511 == 0.0)) && (locals.var_guard1512 == 0.0)) && (locals.var_guard1513 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign64060_e98968;
        locals.var_mm_rv = 0.0;

        let assign64070_e98971: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1514 = assign64070_e98971;
        locals.var_guard1514_rv = 0.0;

        let (assign64080_e98993,) = {
    if ((((((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) && (locals.var_guard1510 != 0.0)) && (locals.var_guard1511 == 0.0)) && (locals.var_guard1512 == 0.0)) && (locals.var_guard1513 == 0.0)) && (locals.var_guard1514 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign64080_e98993;
        locals.var_mm_rv = 0.0;

        let (assign64090_e99004,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) && (locals.var_guard1510 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign64090_e99004;
        locals.var_m0_rv = 0.0;

        let mut assign64100_loop_guard: usize = 0;
        while {
            let assign64100_cond_e99016: f64 = if (((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) && (locals.var_guard1510 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign64100_cond_e99016 != 0.0
        } {
            assign64100_loop_guard += 1;
            assert!(assign64100_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign64100_body0_e99028, assign64100_body0_e99028_d_n0, assign64100_body0_e99028_d_n2, assign64100_body0_e99028_d_n4, assign64100_body0_e99028_d_n5, assign64100_body0_e99028_d_n6, assign64100_body0_e99028_d_n7, assign64100_body0_e99028_d_n8, assign64100_body0_e99028_d_n9, assign64100_body0_e99028_d_n10, assign64100_body0_e99028_d_n11, assign64100_body0_e99028_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) && (locals.var_guard1510 != 0.0)) {
        let assign64100_body0_e99026: f64 = (locals.var_dnm).sqrt();
        (assign64100_body0_e99026, (locals.var_dnm_dn0 / (2.0 * assign64100_body0_e99026)), (locals.var_dnm_dn2 / (2.0 * assign64100_body0_e99026)), (locals.var_dnm_dn4 / (2.0 * assign64100_body0_e99026)), (locals.var_dnm_dn5 / (2.0 * assign64100_body0_e99026)), (locals.var_dnm_dn6 / (2.0 * assign64100_body0_e99026)), (locals.var_dnm_dn7 / (2.0 * assign64100_body0_e99026)), (locals.var_dnm_dn8 / (2.0 * assign64100_body0_e99026)), (locals.var_dnm_dn9 / (2.0 * assign64100_body0_e99026)), (locals.var_dnm_dn10 / (2.0 * assign64100_body0_e99026)), (locals.var_dnm_dn11 / (2.0 * assign64100_body0_e99026)), (locals.var_dnm_dn14 / (2.0 * assign64100_body0_e99026)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign64100_body0_e99028;
            locals.var_dnm_dn0 = assign64100_body0_e99028_d_n0;
            locals.var_dnm_dn2 = assign64100_body0_e99028_d_n2;
            locals.var_dnm_dn4 = assign64100_body0_e99028_d_n4;
            locals.var_dnm_dn5 = assign64100_body0_e99028_d_n5;
            locals.var_dnm_dn6 = assign64100_body0_e99028_d_n6;
            locals.var_dnm_dn7 = assign64100_body0_e99028_d_n7;
            locals.var_dnm_dn8 = assign64100_body0_e99028_d_n8;
            locals.var_dnm_dn9 = assign64100_body0_e99028_d_n9;
            locals.var_dnm_dn10 = assign64100_body0_e99028_d_n10;
            locals.var_dnm_dn11 = assign64100_body0_e99028_d_n11;
            locals.var_dnm_dn14 = assign64100_body0_e99028_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign64100_body1_e99041,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) && (locals.var_guard1510 != 0.0)) {
        let assign64100_body1_e99039: f64 = (locals.var_m0 + 1.0);
        (assign64100_body1_e99039,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign64100_body1_e99041;
            locals.var_m0_rv = 0.0;
        }

        let (assign64110_e99064, assign64110_e99064_d_n0, assign64110_e99064_d_n2, assign64110_e99064_d_n4, assign64110_e99064_d_n5, assign64110_e99064_d_n6, assign64110_e99064_d_n7, assign64110_e99064_d_n8, assign64110_e99064_d_n9, assign64110_e99064_d_n10, assign64110_e99064_d_n11, assign64110_e99064_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) && (locals.var_guard1510 == 0.0)) {
        let (assign64110_e99062, assign64110_e99062_d_n0, assign64110_e99062_d_n2, assign64110_e99062_d_n4, assign64110_e99062_d_n5, assign64110_e99062_d_n6, assign64110_e99062_d_n7, assign64110_e99062_d_n8, assign64110_e99062_d_n9, assign64110_e99062_d_n10, assign64110_e99062_d_n11, assign64110_e99062_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign64110_e99059: f64 = (2.0 * 4.0);
                let assign64110_e99060: f64 = (1.0 / assign64110_e99059);
                let assign64110_e99061: f64 = (locals.var_dnm).powf(assign64110_e99060);
                (assign64110_e99061, if 0.0 == 0.0 && ((assign64110_e99060) as f64).is_finite() && ((assign64110_e99060) as f64).fract() == 0.0 { if assign64110_e99060 == 0.0 { 0.0 } else { (assign64110_e99060 * ((locals.var_dnm).powf(assign64110_e99060 - 1.0) * locals.var_dnm_dn0)) } } else { (assign64110_e99061 * (assign64110_e99060 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64110_e99060) as f64).is_finite() && ((assign64110_e99060) as f64).fract() == 0.0 { if assign64110_e99060 == 0.0 { 0.0 } else { (assign64110_e99060 * ((locals.var_dnm).powf(assign64110_e99060 - 1.0) * locals.var_dnm_dn2)) } } else { (assign64110_e99061 * (assign64110_e99060 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64110_e99060) as f64).is_finite() && ((assign64110_e99060) as f64).fract() == 0.0 { if assign64110_e99060 == 0.0 { 0.0 } else { (assign64110_e99060 * ((locals.var_dnm).powf(assign64110_e99060 - 1.0) * locals.var_dnm_dn4)) } } else { (assign64110_e99061 * (assign64110_e99060 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64110_e99060) as f64).is_finite() && ((assign64110_e99060) as f64).fract() == 0.0 { if assign64110_e99060 == 0.0 { 0.0 } else { (assign64110_e99060 * ((locals.var_dnm).powf(assign64110_e99060 - 1.0) * locals.var_dnm_dn5)) } } else { (assign64110_e99061 * (assign64110_e99060 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64110_e99060) as f64).is_finite() && ((assign64110_e99060) as f64).fract() == 0.0 { if assign64110_e99060 == 0.0 { 0.0 } else { (assign64110_e99060 * ((locals.var_dnm).powf(assign64110_e99060 - 1.0) * locals.var_dnm_dn6)) } } else { (assign64110_e99061 * (assign64110_e99060 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64110_e99060) as f64).is_finite() && ((assign64110_e99060) as f64).fract() == 0.0 { if assign64110_e99060 == 0.0 { 0.0 } else { (assign64110_e99060 * ((locals.var_dnm).powf(assign64110_e99060 - 1.0) * locals.var_dnm_dn7)) } } else { (assign64110_e99061 * (assign64110_e99060 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64110_e99060) as f64).is_finite() && ((assign64110_e99060) as f64).fract() == 0.0 { if assign64110_e99060 == 0.0 { 0.0 } else { (assign64110_e99060 * ((locals.var_dnm).powf(assign64110_e99060 - 1.0) * locals.var_dnm_dn8)) } } else { (assign64110_e99061 * (assign64110_e99060 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64110_e99060) as f64).is_finite() && ((assign64110_e99060) as f64).fract() == 0.0 { if assign64110_e99060 == 0.0 { 0.0 } else { (assign64110_e99060 * ((locals.var_dnm).powf(assign64110_e99060 - 1.0) * locals.var_dnm_dn9)) } } else { (assign64110_e99061 * (assign64110_e99060 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64110_e99060) as f64).is_finite() && ((assign64110_e99060) as f64).fract() == 0.0 { if assign64110_e99060 == 0.0 { 0.0 } else { (assign64110_e99060 * ((locals.var_dnm).powf(assign64110_e99060 - 1.0) * locals.var_dnm_dn10)) } } else { (assign64110_e99061 * (assign64110_e99060 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64110_e99060) as f64).is_finite() && ((assign64110_e99060) as f64).fract() == 0.0 { if assign64110_e99060 == 0.0 { 0.0 } else { (assign64110_e99060 * ((locals.var_dnm).powf(assign64110_e99060 - 1.0) * locals.var_dnm_dn11)) } } else { (assign64110_e99061 * (assign64110_e99060 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign64110_e99060) as f64).is_finite() && ((assign64110_e99060) as f64).fract() == 0.0 { if assign64110_e99060 == 0.0 { 0.0 } else { (assign64110_e99060 * ((locals.var_dnm).powf(assign64110_e99060 - 1.0) * locals.var_dnm_dn14)) } } else { (assign64110_e99061 * (assign64110_e99060 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign64110_e99062, assign64110_e99062_d_n0, assign64110_e99062_d_n2, assign64110_e99062_d_n4, assign64110_e99062_d_n5, assign64110_e99062_d_n6, assign64110_e99062_d_n7, assign64110_e99062_d_n8, assign64110_e99062_d_n9, assign64110_e99062_d_n10, assign64110_e99062_d_n11, assign64110_e99062_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign64110_e99064;
        locals.var_dnm_dn0 = assign64110_e99064_d_n0;
        locals.var_dnm_dn2 = assign64110_e99064_d_n2;
        locals.var_dnm_dn4 = assign64110_e99064_d_n4;
        locals.var_dnm_dn5 = assign64110_e99064_d_n5;
        locals.var_dnm_dn6 = assign64110_e99064_d_n6;
        locals.var_dnm_dn7 = assign64110_e99064_d_n7;
        locals.var_dnm_dn8 = assign64110_e99064_d_n8;
        locals.var_dnm_dn9 = assign64110_e99064_d_n9;
        locals.var_dnm_dn10 = assign64110_e99064_d_n10;
        locals.var_dnm_dn11 = assign64110_e99064_d_n11;
        locals.var_dnm_dn14 = assign64110_e99064_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign64120_e99075, assign64120_e99075_d_n0, assign64120_e99075_d_n2, assign64120_e99075_d_n4, assign64120_e99075_d_n5, assign64120_e99075_d_n6, assign64120_e99075_d_n7, assign64120_e99075_d_n8, assign64120_e99075_d_n9, assign64120_e99075_d_n10, assign64120_e99075_d_n11, assign64120_e99075_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign64120_e99073: f64 = (1.0 / locals.var_dnm);
        (assign64120_e99073, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign64120_e99075;
        locals.var_dnm_dn0 = assign64120_e99075_d_n0;
        locals.var_dnm_dn2 = assign64120_e99075_d_n2;
        locals.var_dnm_dn4 = assign64120_e99075_d_n4;
        locals.var_dnm_dn5 = assign64120_e99075_d_n5;
        locals.var_dnm_dn6 = assign64120_e99075_d_n6;
        locals.var_dnm_dn7 = assign64120_e99075_d_n7;
        locals.var_dnm_dn8 = assign64120_e99075_d_n8;
        locals.var_dnm_dn9 = assign64120_e99075_d_n9;
        locals.var_dnm_dn10 = assign64120_e99075_d_n10;
        locals.var_dnm_dn11 = assign64120_e99075_d_n11;
        locals.var_dnm_dn14 = assign64120_e99075_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign64130_e99088, assign64130_e99088_d_n0, assign64130_e99088_d_n2, assign64130_e99088_d_n4, assign64130_e99088_d_n5, assign64130_e99088_d_n6, assign64130_e99088_d_n7, assign64130_e99088_d_n8, assign64130_e99088_d_n9, assign64130_e99088_d_n10, assign64130_e99088_d_n11, assign64130_e99088_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign64130_e99084: f64 = locals.var_tx;
        let assign64130_e99086: f64 = (assign64130_e99084 * locals.var_dnm);
        (assign64130_e99086, ((locals.var_tx_dn0 * locals.var_dnm) + (assign64130_e99084 * locals.var_dnm_dn0)), ((locals.var_tx_dn2 * locals.var_dnm) + (assign64130_e99084 * locals.var_dnm_dn2)), ((locals.var_tx_dn4 * locals.var_dnm) + (assign64130_e99084 * locals.var_dnm_dn4)), ((locals.var_tx_dn5 * locals.var_dnm) + (assign64130_e99084 * locals.var_dnm_dn5)), ((locals.var_tx_dn6 * locals.var_dnm) + (assign64130_e99084 * locals.var_dnm_dn6)), ((locals.var_tx_dn7 * locals.var_dnm) + (assign64130_e99084 * locals.var_dnm_dn7)), ((locals.var_tx_dn8 * locals.var_dnm) + (assign64130_e99084 * locals.var_dnm_dn8)), ((locals.var_tx_dn9 * locals.var_dnm) + (assign64130_e99084 * locals.var_dnm_dn9)), ((locals.var_tx_dn10 * locals.var_dnm) + (assign64130_e99084 * locals.var_dnm_dn10)), ((locals.var_tx_dn11 * locals.var_dnm) + (assign64130_e99084 * locals.var_dnm_dn11)), ((locals.var_tx_dn14 * locals.var_dnm) + (assign64130_e99084 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign64130_e99088;
        locals.var_ty_dn0 = assign64130_e99088_d_n0;
        locals.var_ty_dn2 = assign64130_e99088_d_n2;
        locals.var_ty_dn4 = assign64130_e99088_d_n4;
        locals.var_ty_dn5 = assign64130_e99088_d_n5;
        locals.var_ty_dn6 = assign64130_e99088_d_n6;
        locals.var_ty_dn7 = assign64130_e99088_d_n7;
        locals.var_ty_dn8 = assign64130_e99088_d_n8;
        locals.var_ty_dn9 = assign64130_e99088_d_n9;
        locals.var_ty_dn10 = assign64130_e99088_d_n10;
        locals.var_ty_dn11 = assign64130_e99088_d_n11;
        locals.var_ty_dn14 = assign64130_e99088_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign64140_e99103, assign64140_e99103_d_n0, assign64140_e99103_d_n2, assign64140_e99103_d_n4, assign64140_e99103_d_n5, assign64140_e99103_d_n6, assign64140_e99103_d_n7, assign64140_e99103_d_n8, assign64140_e99103_d_n9, assign64140_e99103_d_n10, assign64140_e99103_d_n11, assign64140_e99103_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign64140_e99097: f64 = locals.var_xmp;
        let assign64140_e99099: f64 = (assign64140_e99097 * locals.var_dnm);
        let assign64140_e99101: f64 = (assign64140_e99099 / locals.var_arg);
        (assign64140_e99101, (((((locals.var_xmp_dn0 * locals.var_dnm) + (assign64140_e99097 * locals.var_dnm_dn0)) * locals.var_arg) - (assign64140_e99099 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn2 * locals.var_dnm) + (assign64140_e99097 * locals.var_dnm_dn2)) * locals.var_arg) - (assign64140_e99099 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn4 * locals.var_dnm) + (assign64140_e99097 * locals.var_dnm_dn4)) * locals.var_arg) - (assign64140_e99099 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn5 * locals.var_dnm) + (assign64140_e99097 * locals.var_dnm_dn5)) * locals.var_arg) - (assign64140_e99099 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn6 * locals.var_dnm) + (assign64140_e99097 * locals.var_dnm_dn6)) * locals.var_arg) - (assign64140_e99099 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn7 * locals.var_dnm) + (assign64140_e99097 * locals.var_dnm_dn7)) * locals.var_arg) - (assign64140_e99099 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn8 * locals.var_dnm) + (assign64140_e99097 * locals.var_dnm_dn8)) * locals.var_arg) - (assign64140_e99099 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn9 * locals.var_dnm) + (assign64140_e99097 * locals.var_dnm_dn9)) * locals.var_arg) - (assign64140_e99099 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn10 * locals.var_dnm) + (assign64140_e99097 * locals.var_dnm_dn10)) * locals.var_arg) - (assign64140_e99099 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn11 * locals.var_dnm) + (assign64140_e99097 * locals.var_dnm_dn11)) * locals.var_arg) - (assign64140_e99099 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn14 * locals.var_dnm) + (assign64140_e99097 * locals.var_dnm_dn14)) * locals.var_arg) - (assign64140_e99099 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign64140_e99103;
        locals.var_t2_dn0 = assign64140_e99103_d_n0;
        locals.var_t2_dn2 = assign64140_e99103_d_n2;
        locals.var_t2_dn4 = assign64140_e99103_d_n4;
        locals.var_t2_dn5 = assign64140_e99103_d_n5;
        locals.var_t2_dn6 = assign64140_e99103_d_n6;
        locals.var_t2_dn7 = assign64140_e99103_d_n7;
        locals.var_t2_dn8 = assign64140_e99103_d_n8;
        locals.var_t2_dn9 = assign64140_e99103_d_n9;
        locals.var_t2_dn10 = assign64140_e99103_d_n10;
        locals.var_t2_dn11 = assign64140_e99103_d_n11;
        locals.var_t2_dn14 = assign64140_e99103_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign64150_e99118, assign64150_e99118_d_n0, assign64150_e99118_d_n2, assign64150_e99118_d_n4, assign64150_e99118_d_n5, assign64150_e99118_d_n6, assign64150_e99118_d_n7, assign64150_e99118_d_n8, assign64150_e99118_d_n9, assign64150_e99118_d_n10, assign64150_e99118_d_n11, assign64150_e99118_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign64150_e99112: f64 = (2.0 * locals.var_uc_wsti);
        let assign64150_e99114: f64 = (assign64150_e99112 * p.p7);
        let assign64150_e99116: f64 = (assign64150_e99114 * locals.var_beta_inv);
        (assign64150_e99116, ((((2.0 * locals.var_uc_wsti_dn0) * p.p7) * locals.var_beta_inv) + (assign64150_e99114 * locals.var_beta_inv_dn0)), ((((2.0 * locals.var_uc_wsti_dn2) * p.p7) * locals.var_beta_inv) + (assign64150_e99114 * locals.var_beta_inv_dn2)), ((((2.0 * locals.var_uc_wsti_dn4) * p.p7) * locals.var_beta_inv) + (assign64150_e99114 * locals.var_beta_inv_dn4)), ((((2.0 * locals.var_uc_wsti_dn5) * p.p7) * locals.var_beta_inv) + (assign64150_e99114 * locals.var_beta_inv_dn5)), ((((2.0 * locals.var_uc_wsti_dn6) * p.p7) * locals.var_beta_inv) + (assign64150_e99114 * locals.var_beta_inv_dn6)), ((((2.0 * locals.var_uc_wsti_dn7) * p.p7) * locals.var_beta_inv) + (assign64150_e99114 * locals.var_beta_inv_dn7)), ((((2.0 * locals.var_uc_wsti_dn8) * p.p7) * locals.var_beta_inv) + (assign64150_e99114 * locals.var_beta_inv_dn8)), ((((2.0 * locals.var_uc_wsti_dn9) * p.p7) * locals.var_beta_inv) + (assign64150_e99114 * locals.var_beta_inv_dn9)), ((((2.0 * locals.var_uc_wsti_dn10) * p.p7) * locals.var_beta_inv) + (assign64150_e99114 * locals.var_beta_inv_dn10)), ((((2.0 * locals.var_uc_wsti_dn11) * p.p7) * locals.var_beta_inv) + (assign64150_e99114 * locals.var_beta_inv_dn11)), ((((2.0 * locals.var_uc_wsti_dn14) * p.p7) * locals.var_beta_inv) + (assign64150_e99114 * locals.var_beta_inv_dn14)),)
    } else {
        (locals.var_costi7, locals.var_costi7_dn0, locals.var_costi7_dn2, locals.var_costi7_dn4, locals.var_costi7_dn5, locals.var_costi7_dn6, locals.var_costi7_dn7, locals.var_costi7_dn8, locals.var_costi7_dn9, locals.var_costi7_dn10, locals.var_costi7_dn11, locals.var_costi7_dn14,)
    }
};
        locals.var_costi7 = assign64150_e99118;
        locals.var_costi7_dn0 = assign64150_e99118_d_n0;
        locals.var_costi7_dn2 = assign64150_e99118_d_n2;
        locals.var_costi7_dn4 = assign64150_e99118_d_n4;
        locals.var_costi7_dn5 = assign64150_e99118_d_n5;
        locals.var_costi7_dn6 = assign64150_e99118_d_n6;
        locals.var_costi7_dn7 = assign64150_e99118_d_n7;
        locals.var_costi7_dn8 = assign64150_e99118_d_n8;
        locals.var_costi7_dn9 = assign64150_e99118_d_n9;
        locals.var_costi7_dn10 = assign64150_e99118_d_n10;
        locals.var_costi7_dn11 = assign64150_e99118_d_n11;
        locals.var_costi7_dn14 = assign64150_e99118_d_n14;
        locals.var_costi7_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_240(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign64160_e99127, assign64160_e99127_d_n0, assign64160_e99127_d_n2, assign64160_e99127_d_n4, assign64160_e99127_d_n5, assign64160_e99127_d_n6, assign64160_e99127_d_n7, assign64160_e99127_d_n8, assign64160_e99127_d_n9, assign64160_e99127_d_n10, assign64160_e99127_d_n11, assign64160_e99127_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn4, locals.var_lch_dn5, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn8, locals.var_lch_dn9, locals.var_lch_dn10, locals.var_lch_dn11, locals.var_lch_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign64160_e99127;
        locals.var_t1_dn0 = assign64160_e99127_d_n0;
        locals.var_t1_dn2 = assign64160_e99127_d_n2;
        locals.var_t1_dn4 = assign64160_e99127_d_n4;
        locals.var_t1_dn5 = assign64160_e99127_d_n5;
        locals.var_t1_dn6 = assign64160_e99127_d_n6;
        locals.var_t1_dn7 = assign64160_e99127_d_n7;
        locals.var_t1_dn8 = assign64160_e99127_d_n8;
        locals.var_t1_dn9 = assign64160_e99127_d_n9;
        locals.var_t1_dn10 = assign64160_e99127_d_n10;
        locals.var_t1_dn11 = assign64160_e99127_d_n11;
        locals.var_t1_dn14 = assign64160_e99127_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign64170_e99144, assign64170_e99144_d_n0, assign64170_e99144_d_n2, assign64170_e99144_d_n4, assign64170_e99144_d_n5, assign64170_e99144_d_n6, assign64170_e99144_d_n7, assign64170_e99144_d_n8, assign64170_e99144_d_n9, assign64170_e99144_d_n10, assign64170_e99144_d_n11, assign64170_e99144_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign64170_e99136: f64 = (locals.var_costi7 * locals.var_mu);
        let assign64170_e99138: f64 = (assign64170_e99136 * locals.var_qn0sti);
        let assign64170_e99140: f64 = (assign64170_e99138 * locals.var_ty);
        let assign64170_e99142: f64 = (assign64170_e99140 / locals.var_t1);
        (assign64170_e99142, (((((((((locals.var_costi7_dn0 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn0)) * locals.var_qn0sti) + (assign64170_e99136 * locals.var_qn0sti_dn0)) * locals.var_ty) + (assign64170_e99138 * locals.var_ty_dn0)) * locals.var_t1) - (assign64170_e99140 * locals.var_t1_dn0)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn2 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn2)) * locals.var_qn0sti) + (assign64170_e99136 * locals.var_qn0sti_dn2)) * locals.var_ty) + (assign64170_e99138 * locals.var_ty_dn2)) * locals.var_t1) - (assign64170_e99140 * locals.var_t1_dn2)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn4 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn4)) * locals.var_qn0sti) + (assign64170_e99136 * locals.var_qn0sti_dn4)) * locals.var_ty) + (assign64170_e99138 * locals.var_ty_dn4)) * locals.var_t1) - (assign64170_e99140 * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn5 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn5)) * locals.var_qn0sti) + (assign64170_e99136 * locals.var_qn0sti_dn5)) * locals.var_ty) + (assign64170_e99138 * locals.var_ty_dn5)) * locals.var_t1) - (assign64170_e99140 * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn6 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn6)) * locals.var_qn0sti) + (assign64170_e99136 * locals.var_qn0sti_dn6)) * locals.var_ty) + (assign64170_e99138 * locals.var_ty_dn6)) * locals.var_t1) - (assign64170_e99140 * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn7 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn7)) * locals.var_qn0sti) + (assign64170_e99136 * locals.var_qn0sti_dn7)) * locals.var_ty) + (assign64170_e99138 * locals.var_ty_dn7)) * locals.var_t1) - (assign64170_e99140 * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn8 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn8)) * locals.var_qn0sti) + (assign64170_e99136 * locals.var_qn0sti_dn8)) * locals.var_ty) + (assign64170_e99138 * locals.var_ty_dn8)) * locals.var_t1) - (assign64170_e99140 * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn9 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn9)) * locals.var_qn0sti) + (assign64170_e99136 * locals.var_qn0sti_dn9)) * locals.var_ty) + (assign64170_e99138 * locals.var_ty_dn9)) * locals.var_t1) - (assign64170_e99140 * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn10 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn10)) * locals.var_qn0sti) + (assign64170_e99136 * locals.var_qn0sti_dn10)) * locals.var_ty) + (assign64170_e99138 * locals.var_ty_dn10)) * locals.var_t1) - (assign64170_e99140 * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn11 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn11)) * locals.var_qn0sti) + (assign64170_e99136 * locals.var_qn0sti_dn11)) * locals.var_ty) + (assign64170_e99138 * locals.var_ty_dn11)) * locals.var_t1) - (assign64170_e99140 * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)), (((((((((locals.var_costi7_dn14 * locals.var_mu) + (locals.var_costi7 * locals.var_mu_dn14)) * locals.var_qn0sti) + (assign64170_e99136 * locals.var_qn0sti_dn14)) * locals.var_ty) + (assign64170_e99138 * locals.var_ty_dn14)) * locals.var_t1) - (assign64170_e99140 * locals.var_t1_dn14)) / (locals.var_t1 * locals.var_t1)),)
    } else {
        (locals.var_idssti, locals.var_idssti_dn0, locals.var_idssti_dn2, locals.var_idssti_dn4, locals.var_idssti_dn5, locals.var_idssti_dn6, locals.var_idssti_dn7, locals.var_idssti_dn8, locals.var_idssti_dn9, locals.var_idssti_dn10, locals.var_idssti_dn11, locals.var_idssti_dn14,)
    }
};
        locals.var_idssti = assign64170_e99144;
        locals.var_idssti_dn0 = assign64170_e99144_d_n0;
        locals.var_idssti_dn2 = assign64170_e99144_d_n2;
        locals.var_idssti_dn4 = assign64170_e99144_d_n4;
        locals.var_idssti_dn5 = assign64170_e99144_d_n5;
        locals.var_idssti_dn6 = assign64170_e99144_d_n6;
        locals.var_idssti_dn7 = assign64170_e99144_d_n7;
        locals.var_idssti_dn8 = assign64170_e99144_d_n8;
        locals.var_idssti_dn9 = assign64170_e99144_d_n9;
        locals.var_idssti_dn10 = assign64170_e99144_d_n10;
        locals.var_idssti_dn11 = assign64170_e99144_d_n11;
        locals.var_idssti_dn14 = assign64170_e99144_d_n14;
        locals.var_idssti_rv = 0.0;

        let (assign64180_e99155, assign64180_e99155_d_n0, assign64180_e99155_d_n2, assign64180_e99155_d_n4, assign64180_e99155_d_n5, assign64180_e99155_d_n6, assign64180_e99155_d_n7, assign64180_e99155_d_n8, assign64180_e99155_d_n9, assign64180_e99155_d_n10, assign64180_e99155_d_n11, assign64180_e99155_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1505 != 0.0)) {
        let assign64180_e99153: f64 = (locals.var_ids + locals.var_idssti);
        (assign64180_e99153, (locals.var_ids_dn0 + locals.var_idssti_dn0), (locals.var_ids_dn2 + locals.var_idssti_dn2), (locals.var_ids_dn4 + locals.var_idssti_dn4), (locals.var_ids_dn5 + locals.var_idssti_dn5), (locals.var_ids_dn6 + locals.var_idssti_dn6), (locals.var_ids_dn7 + locals.var_idssti_dn7), (locals.var_ids_dn8 + locals.var_idssti_dn8), (locals.var_ids_dn9 + locals.var_idssti_dn9), (locals.var_ids_dn10 + locals.var_idssti_dn10), (locals.var_ids_dn11 + locals.var_idssti_dn11), (locals.var_ids_dn14 + locals.var_idssti_dn14),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn14,)
    }
};
        locals.var_ids = assign64180_e99155;
        locals.var_ids_dn0 = assign64180_e99155_d_n0;
        locals.var_ids_dn2 = assign64180_e99155_d_n2;
        locals.var_ids_dn4 = assign64180_e99155_d_n4;
        locals.var_ids_dn5 = assign64180_e99155_d_n5;
        locals.var_ids_dn6 = assign64180_e99155_d_n6;
        locals.var_ids_dn7 = assign64180_e99155_d_n7;
        locals.var_ids_dn8 = assign64180_e99155_d_n8;
        locals.var_ids_dn9 = assign64180_e99155_d_n9;
        locals.var_ids_dn10 = assign64180_e99155_d_n10;
        locals.var_ids_dn11 = assign64180_e99155_d_n11;
        locals.var_ids_dn14 = assign64180_e99155_d_n14;
        locals.var_ids_rv = 0.0;

        let assign64190_e99166: f64 = if (((p.p31 != 0.0) && (p.p30 != 0.0)) && (locals.var_uc_codep == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1515 = assign64190_e99166;
        locals.var_guard1515_rv = 0.0;

        let (assign64200_e99177, assign64200_e99177_d_n0, assign64200_e99177_d_n2, assign64200_e99177_d_n4, assign64200_e99177_d_n5, assign64200_e99177_d_n6, assign64200_e99177_d_n7, assign64200_e99177_d_n8, assign64200_e99177_d_n9, assign64200_e99177_d_n10, assign64200_e99177_d_n11, assign64200_e99177_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1515 != 0.0)) {
        let assign64200_e99175: f64 = (locals.var_vgvt * locals.var_vgvt);
        (assign64200_e99175, ((locals.var_vgvt_dn0 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn0)), ((locals.var_vgvt_dn2 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn2)), ((locals.var_vgvt_dn4 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn4)), ((locals.var_vgvt_dn5 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn5)), ((locals.var_vgvt_dn6 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn6)), ((locals.var_vgvt_dn7 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn7)), ((locals.var_vgvt_dn8 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn8)), ((locals.var_vgvt_dn9 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn9)), ((locals.var_vgvt_dn10 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn10)), ((locals.var_vgvt_dn11 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn11)), ((locals.var_vgvt_dn14 * locals.var_vgvt) + (locals.var_vgvt * locals.var_vgvt_dn14)),)
    } else {
        (locals.var_kusai00, locals.var_kusai00_dn0, locals.var_kusai00_dn2, locals.var_kusai00_dn4, locals.var_kusai00_dn5, locals.var_kusai00_dn6, locals.var_kusai00_dn7, locals.var_kusai00_dn8, locals.var_kusai00_dn9, locals.var_kusai00_dn10, locals.var_kusai00_dn11, locals.var_kusai00_dn14,)
    }
};
        locals.var_kusai00 = assign64200_e99177;
        locals.var_kusai00_dn0 = assign64200_e99177_d_n0;
        locals.var_kusai00_dn2 = assign64200_e99177_d_n2;
        locals.var_kusai00_dn4 = assign64200_e99177_d_n4;
        locals.var_kusai00_dn5 = assign64200_e99177_d_n5;
        locals.var_kusai00_dn6 = assign64200_e99177_d_n6;
        locals.var_kusai00_dn7 = assign64200_e99177_d_n7;
        locals.var_kusai00_dn8 = assign64200_e99177_d_n8;
        locals.var_kusai00_dn9 = assign64200_e99177_d_n9;
        locals.var_kusai00_dn10 = assign64200_e99177_d_n10;
        locals.var_kusai00_dn11 = assign64200_e99177_d_n11;
        locals.var_kusai00_dn14 = assign64200_e99177_d_n14;
        locals.var_kusai00_rv = 0.0;

        let (assign64210_e99192, assign64210_e99192_d_n0, assign64210_e99192_d_n2, assign64210_e99192_d_n4, assign64210_e99192_d_n5, assign64210_e99192_d_n6, assign64210_e99192_d_n7, assign64210_e99192_d_n8, assign64210_e99192_d_n9, assign64210_e99192_d_n10, assign64210_e99192_d_n11, assign64210_e99192_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1515 != 0.0)) {
        let assign64210_e99186: f64 = (2.0 * locals.var_beta_inv);
        let assign64210_e99188: f64 = (assign64210_e99186 * locals.var_cox_inv);
        let assign64210_e99190: f64 = (assign64210_e99188 * locals.var_idd);
        (assign64210_e99190, (((((2.0 * locals.var_beta_inv_dn0) * locals.var_cox_inv) + (assign64210_e99186 * locals.var_cox_inv_dn0)) * locals.var_idd) + (assign64210_e99188 * locals.var_idd_dn0)), (((((2.0 * locals.var_beta_inv_dn2) * locals.var_cox_inv) + (assign64210_e99186 * locals.var_cox_inv_dn2)) * locals.var_idd) + (assign64210_e99188 * locals.var_idd_dn2)), (((((2.0 * locals.var_beta_inv_dn4) * locals.var_cox_inv) + (assign64210_e99186 * locals.var_cox_inv_dn4)) * locals.var_idd) + (assign64210_e99188 * locals.var_idd_dn4)), (((((2.0 * locals.var_beta_inv_dn5) * locals.var_cox_inv) + (assign64210_e99186 * locals.var_cox_inv_dn5)) * locals.var_idd) + (assign64210_e99188 * locals.var_idd_dn5)), (((((2.0 * locals.var_beta_inv_dn6) * locals.var_cox_inv) + (assign64210_e99186 * locals.var_cox_inv_dn6)) * locals.var_idd) + (assign64210_e99188 * locals.var_idd_dn6)), (((((2.0 * locals.var_beta_inv_dn7) * locals.var_cox_inv) + (assign64210_e99186 * locals.var_cox_inv_dn7)) * locals.var_idd) + (assign64210_e99188 * locals.var_idd_dn7)), (((((2.0 * locals.var_beta_inv_dn8) * locals.var_cox_inv) + (assign64210_e99186 * locals.var_cox_inv_dn8)) * locals.var_idd) + (assign64210_e99188 * locals.var_idd_dn8)), (((((2.0 * locals.var_beta_inv_dn9) * locals.var_cox_inv) + (assign64210_e99186 * locals.var_cox_inv_dn9)) * locals.var_idd) + (assign64210_e99188 * locals.var_idd_dn9)), (((((2.0 * locals.var_beta_inv_dn10) * locals.var_cox_inv) + (assign64210_e99186 * locals.var_cox_inv_dn10)) * locals.var_idd) + (assign64210_e99188 * locals.var_idd_dn10)), (((((2.0 * locals.var_beta_inv_dn11) * locals.var_cox_inv) + (assign64210_e99186 * locals.var_cox_inv_dn11)) * locals.var_idd) + (assign64210_e99188 * locals.var_idd_dn11)), (((((2.0 * locals.var_beta_inv_dn14) * locals.var_cox_inv) + (assign64210_e99186 * locals.var_cox_inv_dn14)) * locals.var_idd) + (assign64210_e99188 * locals.var_idd_dn14)),)
    } else {
        (locals.var_kusaidd, locals.var_kusaidd_dn0, locals.var_kusaidd_dn2, locals.var_kusaidd_dn4, locals.var_kusaidd_dn5, locals.var_kusaidd_dn6, locals.var_kusaidd_dn7, locals.var_kusaidd_dn8, locals.var_kusaidd_dn9, locals.var_kusaidd_dn10, locals.var_kusaidd_dn11, locals.var_kusaidd_dn14,)
    }
};
        locals.var_kusaidd = assign64210_e99192;
        locals.var_kusaidd_dn0 = assign64210_e99192_d_n0;
        locals.var_kusaidd_dn2 = assign64210_e99192_d_n2;
        locals.var_kusaidd_dn4 = assign64210_e99192_d_n4;
        locals.var_kusaidd_dn5 = assign64210_e99192_d_n5;
        locals.var_kusaidd_dn6 = assign64210_e99192_d_n6;
        locals.var_kusaidd_dn7 = assign64210_e99192_d_n7;
        locals.var_kusaidd_dn8 = assign64210_e99192_d_n8;
        locals.var_kusaidd_dn9 = assign64210_e99192_d_n9;
        locals.var_kusaidd_dn10 = assign64210_e99192_d_n10;
        locals.var_kusaidd_dn11 = assign64210_e99192_d_n11;
        locals.var_kusaidd_dn14 = assign64210_e99192_d_n14;
        locals.var_kusaidd_rv = 0.0;

        let (assign64220_e99203, assign64220_e99203_d_n0, assign64220_e99203_d_n2, assign64220_e99203_d_n4, assign64220_e99203_d_n5, assign64220_e99203_d_n6, assign64220_e99203_d_n7, assign64220_e99203_d_n8, assign64220_e99203_d_n9, assign64220_e99203_d_n10, assign64220_e99203_d_n11, assign64220_e99203_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1515 != 0.0)) {
        let assign64220_e99201: f64 = (locals.var_kusai00 - locals.var_kusaidd);
        (assign64220_e99201, (locals.var_kusai00_dn0 - locals.var_kusaidd_dn0), (locals.var_kusai00_dn2 - locals.var_kusaidd_dn2), (locals.var_kusai00_dn4 - locals.var_kusaidd_dn4), (locals.var_kusai00_dn5 - locals.var_kusaidd_dn5), (locals.var_kusai00_dn6 - locals.var_kusaidd_dn6), (locals.var_kusai00_dn7 - locals.var_kusaidd_dn7), (locals.var_kusai00_dn8 - locals.var_kusaidd_dn8), (locals.var_kusai00_dn9 - locals.var_kusaidd_dn9), (locals.var_kusai00_dn10 - locals.var_kusaidd_dn10), (locals.var_kusai00_dn11 - locals.var_kusaidd_dn11), (locals.var_kusai00_dn14 - locals.var_kusaidd_dn14),)
    } else {
        (locals.var_kusail, locals.var_kusail_dn0, locals.var_kusail_dn2, locals.var_kusail_dn4, locals.var_kusail_dn5, locals.var_kusail_dn6, locals.var_kusail_dn7, locals.var_kusail_dn8, locals.var_kusail_dn9, locals.var_kusail_dn10, locals.var_kusail_dn11, locals.var_kusail_dn14,)
    }
};
        locals.var_kusail = assign64220_e99203;
        locals.var_kusail_dn0 = assign64220_e99203_d_n0;
        locals.var_kusail_dn2 = assign64220_e99203_d_n2;
        locals.var_kusail_dn4 = assign64220_e99203_d_n4;
        locals.var_kusail_dn5 = assign64220_e99203_d_n5;
        locals.var_kusail_dn6 = assign64220_e99203_d_n6;
        locals.var_kusail_dn7 = assign64220_e99203_d_n7;
        locals.var_kusail_dn8 = assign64220_e99203_d_n8;
        locals.var_kusail_dn9 = assign64220_e99203_d_n9;
        locals.var_kusail_dn10 = assign64220_e99203_d_n10;
        locals.var_kusail_dn11 = assign64220_e99203_d_n11;
        locals.var_kusail_dn14 = assign64220_e99203_d_n14;
        locals.var_kusail_rv = 0.0;

        let (assign64230_e99221, assign64230_e99221_d_n0, assign64230_e99221_d_n2, assign64230_e99221_d_n4, assign64230_e99221_d_n5, assign64230_e99221_d_n6, assign64230_e99221_d_n7, assign64230_e99221_d_n8, assign64230_e99221_d_n9, assign64230_e99221_d_n10, assign64230_e99221_d_n11, assign64230_e99221_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1515 != 0.0)) {
        let assign64230_e99212: f64 = (locals.var_kusai00 * locals.var_kusai00);
        let assign64230_e99215: f64 = (4.0 * 0.001);
        let assign64230_e99217: f64 = (assign64230_e99215 * 0.001);
        let assign64230_e99218: f64 = (assign64230_e99212 + assign64230_e99217);
        let assign64230_e99219: f64 = (assign64230_e99218).sqrt();
        (assign64230_e99219, (((locals.var_kusai00_dn0 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn0)) / (2.0 * assign64230_e99219)), (((locals.var_kusai00_dn2 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn2)) / (2.0 * assign64230_e99219)), (((locals.var_kusai00_dn4 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn4)) / (2.0 * assign64230_e99219)), (((locals.var_kusai00_dn5 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn5)) / (2.0 * assign64230_e99219)), (((locals.var_kusai00_dn6 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn6)) / (2.0 * assign64230_e99219)), (((locals.var_kusai00_dn7 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn7)) / (2.0 * assign64230_e99219)), (((locals.var_kusai00_dn8 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn8)) / (2.0 * assign64230_e99219)), (((locals.var_kusai00_dn9 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn9)) / (2.0 * assign64230_e99219)), (((locals.var_kusai00_dn10 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn10)) / (2.0 * assign64230_e99219)), (((locals.var_kusai00_dn11 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn11)) / (2.0 * assign64230_e99219)), (((locals.var_kusai00_dn14 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn14)) / (2.0 * assign64230_e99219)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign64230_e99221;
        locals.var_tmf2_dn0 = assign64230_e99221_d_n0;
        locals.var_tmf2_dn2 = assign64230_e99221_d_n2;
        locals.var_tmf2_dn4 = assign64230_e99221_d_n4;
        locals.var_tmf2_dn5 = assign64230_e99221_d_n5;
        locals.var_tmf2_dn6 = assign64230_e99221_d_n6;
        locals.var_tmf2_dn7 = assign64230_e99221_d_n7;
        locals.var_tmf2_dn8 = assign64230_e99221_d_n8;
        locals.var_tmf2_dn9 = assign64230_e99221_d_n9;
        locals.var_tmf2_dn10 = assign64230_e99221_d_n10;
        locals.var_tmf2_dn11 = assign64230_e99221_d_n11;
        locals.var_tmf2_dn14 = assign64230_e99221_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign64240_e99236, assign64240_e99236_d_n0, assign64240_e99236_d_n2, assign64240_e99236_d_n4, assign64240_e99236_d_n5, assign64240_e99236_d_n6, assign64240_e99236_d_n7, assign64240_e99236_d_n8, assign64240_e99236_d_n9, assign64240_e99236_d_n10, assign64240_e99236_d_n11, assign64240_e99236_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1515 != 0.0)) {
        let assign64240_e99232: f64 = (locals.var_kusai00 / locals.var_tmf2);
        let assign64240_e99233: f64 = (1.0 + assign64240_e99232);
        let assign64240_e99234: f64 = (0.5 * assign64240_e99233);
        (assign64240_e99234, (0.5 * (((locals.var_kusai00_dn0 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn2 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn4 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn5 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn6 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn7 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn8 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn9 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn10 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn11 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusai00_dn14 * locals.var_tmf2) - (locals.var_kusai00 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign64240_e99236;
        locals.var_t0_dn0 = assign64240_e99236_d_n0;
        locals.var_t0_dn2 = assign64240_e99236_d_n2;
        locals.var_t0_dn4 = assign64240_e99236_d_n4;
        locals.var_t0_dn5 = assign64240_e99236_d_n5;
        locals.var_t0_dn6 = assign64240_e99236_d_n6;
        locals.var_t0_dn7 = assign64240_e99236_d_n7;
        locals.var_t0_dn8 = assign64240_e99236_d_n8;
        locals.var_t0_dn9 = assign64240_e99236_d_n9;
        locals.var_t0_dn10 = assign64240_e99236_d_n10;
        locals.var_t0_dn11 = assign64240_e99236_d_n11;
        locals.var_t0_dn14 = assign64240_e99236_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign64250_e99249, assign64250_e99249_d_n0, assign64250_e99249_d_n2, assign64250_e99249_d_n4, assign64250_e99249_d_n5, assign64250_e99249_d_n6, assign64250_e99249_d_n7, assign64250_e99249_d_n8, assign64250_e99249_d_n9, assign64250_e99249_d_n10, assign64250_e99249_d_n11, assign64250_e99249_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1515 != 0.0)) {
        let assign64250_e99246: f64 = (locals.var_kusai00 + locals.var_tmf2);
        let assign64250_e99247: f64 = (0.5 * assign64250_e99246);
        (assign64250_e99247, (0.5 * (locals.var_kusai00_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_kusai00_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_kusai00_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_kusai00_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_kusai00_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_kusai00_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_kusai00_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_kusai00_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_kusai00_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_kusai00_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_kusai00_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_kusai00, locals.var_kusai00_dn0, locals.var_kusai00_dn2, locals.var_kusai00_dn4, locals.var_kusai00_dn5, locals.var_kusai00_dn6, locals.var_kusai00_dn7, locals.var_kusai00_dn8, locals.var_kusai00_dn9, locals.var_kusai00_dn10, locals.var_kusai00_dn11, locals.var_kusai00_dn14,)
    }
};
        locals.var_kusai00 = assign64250_e99249;
        locals.var_kusai00_dn0 = assign64250_e99249_d_n0;
        locals.var_kusai00_dn2 = assign64250_e99249_d_n2;
        locals.var_kusai00_dn4 = assign64250_e99249_d_n4;
        locals.var_kusai00_dn5 = assign64250_e99249_d_n5;
        locals.var_kusai00_dn6 = assign64250_e99249_d_n6;
        locals.var_kusai00_dn7 = assign64250_e99249_d_n7;
        locals.var_kusai00_dn8 = assign64250_e99249_d_n8;
        locals.var_kusai00_dn9 = assign64250_e99249_d_n9;
        locals.var_kusai00_dn10 = assign64250_e99249_d_n10;
        locals.var_kusai00_dn11 = assign64250_e99249_d_n11;
        locals.var_kusai00_dn14 = assign64250_e99249_d_n14;
        locals.var_kusai00_rv = 0.0;

        let assign64260_e99252: f64 = if locals.var_kusai00 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1516 = assign64260_e99252;
        locals.var_guard1516_rv = 0.0;

        let (assign64270_e99263, assign64270_e99263_d_n0, assign64270_e99263_d_n2, assign64270_e99263_d_n4, assign64270_e99263_d_n5, assign64270_e99263_d_n6, assign64270_e99263_d_n7, assign64270_e99263_d_n8, assign64270_e99263_d_n9, assign64270_e99263_d_n10, assign64270_e99263_d_n11, assign64270_e99263_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1515 != 0.0)) && (locals.var_guard1516 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_kusai00, locals.var_kusai00_dn0, locals.var_kusai00_dn2, locals.var_kusai00_dn4, locals.var_kusai00_dn5, locals.var_kusai00_dn6, locals.var_kusai00_dn7, locals.var_kusai00_dn8, locals.var_kusai00_dn9, locals.var_kusai00_dn10, locals.var_kusai00_dn11, locals.var_kusai00_dn14,)
    }
};
        locals.var_kusai00 = assign64270_e99263;
        locals.var_kusai00_dn0 = assign64270_e99263_d_n0;
        locals.var_kusai00_dn2 = assign64270_e99263_d_n2;
        locals.var_kusai00_dn4 = assign64270_e99263_d_n4;
        locals.var_kusai00_dn5 = assign64270_e99263_d_n5;
        locals.var_kusai00_dn6 = assign64270_e99263_d_n6;
        locals.var_kusai00_dn7 = assign64270_e99263_d_n7;
        locals.var_kusai00_dn8 = assign64270_e99263_d_n8;
        locals.var_kusai00_dn9 = assign64270_e99263_d_n9;
        locals.var_kusai00_dn10 = assign64270_e99263_d_n10;
        locals.var_kusai00_dn11 = assign64270_e99263_d_n11;
        locals.var_kusai00_dn14 = assign64270_e99263_d_n14;
        locals.var_kusai00_rv = 0.0;

        let (assign64280_e99274, assign64280_e99274_d_n0, assign64280_e99274_d_n2, assign64280_e99274_d_n4, assign64280_e99274_d_n5, assign64280_e99274_d_n6, assign64280_e99274_d_n7, assign64280_e99274_d_n8, assign64280_e99274_d_n9, assign64280_e99274_d_n10, assign64280_e99274_d_n11, assign64280_e99274_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1515 != 0.0)) && (locals.var_guard1516 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign64280_e99274;
        locals.var_t0_dn0 = assign64280_e99274_d_n0;
        locals.var_t0_dn2 = assign64280_e99274_d_n2;
        locals.var_t0_dn4 = assign64280_e99274_d_n4;
        locals.var_t0_dn5 = assign64280_e99274_d_n5;
        locals.var_t0_dn6 = assign64280_e99274_d_n6;
        locals.var_t0_dn7 = assign64280_e99274_d_n7;
        locals.var_t0_dn8 = assign64280_e99274_d_n8;
        locals.var_t0_dn9 = assign64280_e99274_d_n9;
        locals.var_t0_dn10 = assign64280_e99274_d_n10;
        locals.var_t0_dn11 = assign64280_e99274_d_n11;
        locals.var_t0_dn14 = assign64280_e99274_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign64290_e99292, assign64290_e99292_d_n0, assign64290_e99292_d_n2, assign64290_e99292_d_n4, assign64290_e99292_d_n5, assign64290_e99292_d_n6, assign64290_e99292_d_n7, assign64290_e99292_d_n8, assign64290_e99292_d_n9, assign64290_e99292_d_n10, assign64290_e99292_d_n11, assign64290_e99292_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1515 != 0.0)) {
        let assign64290_e99283: f64 = (locals.var_kusail * locals.var_kusail);
        let assign64290_e99286: f64 = (4.0 * 0.001);
        let assign64290_e99288: f64 = (assign64290_e99286 * 0.001);
        let assign64290_e99289: f64 = (assign64290_e99283 + assign64290_e99288);
        let assign64290_e99290: f64 = (assign64290_e99289).sqrt();
        (assign64290_e99290, (((locals.var_kusail_dn0 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn0)) / (2.0 * assign64290_e99290)), (((locals.var_kusail_dn2 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn2)) / (2.0 * assign64290_e99290)), (((locals.var_kusail_dn4 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn4)) / (2.0 * assign64290_e99290)), (((locals.var_kusail_dn5 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn5)) / (2.0 * assign64290_e99290)), (((locals.var_kusail_dn6 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn6)) / (2.0 * assign64290_e99290)), (((locals.var_kusail_dn7 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn7)) / (2.0 * assign64290_e99290)), (((locals.var_kusail_dn8 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn8)) / (2.0 * assign64290_e99290)), (((locals.var_kusail_dn9 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn9)) / (2.0 * assign64290_e99290)), (((locals.var_kusail_dn10 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn10)) / (2.0 * assign64290_e99290)), (((locals.var_kusail_dn11 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn11)) / (2.0 * assign64290_e99290)), (((locals.var_kusail_dn14 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn14)) / (2.0 * assign64290_e99290)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign64290_e99292;
        locals.var_tmf2_dn0 = assign64290_e99292_d_n0;
        locals.var_tmf2_dn2 = assign64290_e99292_d_n2;
        locals.var_tmf2_dn4 = assign64290_e99292_d_n4;
        locals.var_tmf2_dn5 = assign64290_e99292_d_n5;
        locals.var_tmf2_dn6 = assign64290_e99292_d_n6;
        locals.var_tmf2_dn7 = assign64290_e99292_d_n7;
        locals.var_tmf2_dn8 = assign64290_e99292_d_n8;
        locals.var_tmf2_dn9 = assign64290_e99292_d_n9;
        locals.var_tmf2_dn10 = assign64290_e99292_d_n10;
        locals.var_tmf2_dn11 = assign64290_e99292_d_n11;
        locals.var_tmf2_dn14 = assign64290_e99292_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign64300_e99307, assign64300_e99307_d_n0, assign64300_e99307_d_n2, assign64300_e99307_d_n4, assign64300_e99307_d_n5, assign64300_e99307_d_n6, assign64300_e99307_d_n7, assign64300_e99307_d_n8, assign64300_e99307_d_n9, assign64300_e99307_d_n10, assign64300_e99307_d_n11, assign64300_e99307_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1515 != 0.0)) {
        let assign64300_e99303: f64 = (locals.var_kusail / locals.var_tmf2);
        let assign64300_e99304: f64 = (1.0 + assign64300_e99303);
        let assign64300_e99305: f64 = (0.5 * assign64300_e99304);
        (assign64300_e99305, (0.5 * (((locals.var_kusail_dn0 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn2 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn4 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn5 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn6 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn7 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn8 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn9 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn10 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn11 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_kusail_dn14 * locals.var_tmf2) - (locals.var_kusail * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign64300_e99307;
        locals.var_t0_dn0 = assign64300_e99307_d_n0;
        locals.var_t0_dn2 = assign64300_e99307_d_n2;
        locals.var_t0_dn4 = assign64300_e99307_d_n4;
        locals.var_t0_dn5 = assign64300_e99307_d_n5;
        locals.var_t0_dn6 = assign64300_e99307_d_n6;
        locals.var_t0_dn7 = assign64300_e99307_d_n7;
        locals.var_t0_dn8 = assign64300_e99307_d_n8;
        locals.var_t0_dn9 = assign64300_e99307_d_n9;
        locals.var_t0_dn10 = assign64300_e99307_d_n10;
        locals.var_t0_dn11 = assign64300_e99307_d_n11;
        locals.var_t0_dn14 = assign64300_e99307_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign64310_e99320, assign64310_e99320_d_n0, assign64310_e99320_d_n2, assign64310_e99320_d_n4, assign64310_e99320_d_n5, assign64310_e99320_d_n6, assign64310_e99320_d_n7, assign64310_e99320_d_n8, assign64310_e99320_d_n9, assign64310_e99320_d_n10, assign64310_e99320_d_n11, assign64310_e99320_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1515 != 0.0)) {
        let assign64310_e99317: f64 = (locals.var_kusail + locals.var_tmf2);
        let assign64310_e99318: f64 = (0.5 * assign64310_e99317);
        (assign64310_e99318, (0.5 * (locals.var_kusail_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_kusail_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_kusail_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_kusail_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_kusail_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_kusail_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_kusail_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_kusail_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_kusail_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_kusail_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_kusail_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_kusail, locals.var_kusail_dn0, locals.var_kusail_dn2, locals.var_kusail_dn4, locals.var_kusail_dn5, locals.var_kusail_dn6, locals.var_kusail_dn7, locals.var_kusail_dn8, locals.var_kusail_dn9, locals.var_kusail_dn10, locals.var_kusail_dn11, locals.var_kusail_dn14,)
    }
};
        locals.var_kusail = assign64310_e99320;
        locals.var_kusail_dn0 = assign64310_e99320_d_n0;
        locals.var_kusail_dn2 = assign64310_e99320_d_n2;
        locals.var_kusail_dn4 = assign64310_e99320_d_n4;
        locals.var_kusail_dn5 = assign64310_e99320_d_n5;
        locals.var_kusail_dn6 = assign64310_e99320_d_n6;
        locals.var_kusail_dn7 = assign64310_e99320_d_n7;
        locals.var_kusail_dn8 = assign64310_e99320_d_n8;
        locals.var_kusail_dn9 = assign64310_e99320_d_n9;
        locals.var_kusail_dn10 = assign64310_e99320_d_n10;
        locals.var_kusail_dn11 = assign64310_e99320_d_n11;
        locals.var_kusail_dn14 = assign64310_e99320_d_n14;
        locals.var_kusail_rv = 0.0;

        let assign64320_e99323: f64 = if locals.var_kusail < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1517 = assign64320_e99323;
        locals.var_guard1517_rv = 0.0;

        let (assign64330_e99334, assign64330_e99334_d_n0, assign64330_e99334_d_n2, assign64330_e99334_d_n4, assign64330_e99334_d_n5, assign64330_e99334_d_n6, assign64330_e99334_d_n7, assign64330_e99334_d_n8, assign64330_e99334_d_n9, assign64330_e99334_d_n10, assign64330_e99334_d_n11, assign64330_e99334_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1515 != 0.0)) && (locals.var_guard1517 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_kusail, locals.var_kusail_dn0, locals.var_kusail_dn2, locals.var_kusail_dn4, locals.var_kusail_dn5, locals.var_kusail_dn6, locals.var_kusail_dn7, locals.var_kusail_dn8, locals.var_kusail_dn9, locals.var_kusail_dn10, locals.var_kusail_dn11, locals.var_kusail_dn14,)
    }
};
        locals.var_kusail = assign64330_e99334;
        locals.var_kusail_dn0 = assign64330_e99334_d_n0;
        locals.var_kusail_dn2 = assign64330_e99334_d_n2;
        locals.var_kusail_dn4 = assign64330_e99334_d_n4;
        locals.var_kusail_dn5 = assign64330_e99334_d_n5;
        locals.var_kusail_dn6 = assign64330_e99334_d_n6;
        locals.var_kusail_dn7 = assign64330_e99334_d_n7;
        locals.var_kusail_dn8 = assign64330_e99334_d_n8;
        locals.var_kusail_dn9 = assign64330_e99334_d_n9;
        locals.var_kusail_dn10 = assign64330_e99334_d_n10;
        locals.var_kusail_dn11 = assign64330_e99334_d_n11;
        locals.var_kusail_dn14 = assign64330_e99334_d_n14;
        locals.var_kusail_rv = 0.0;

        let (assign64340_e99345, assign64340_e99345_d_n0, assign64340_e99345_d_n2, assign64340_e99345_d_n4, assign64340_e99345_d_n5, assign64340_e99345_d_n6, assign64340_e99345_d_n7, assign64340_e99345_d_n8, assign64340_e99345_d_n9, assign64340_e99345_d_n10, assign64340_e99345_d_n11, assign64340_e99345_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1515 != 0.0)) && (locals.var_guard1517 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign64340_e99345;
        locals.var_t0_dn0 = assign64340_e99345_d_n0;
        locals.var_t0_dn2 = assign64340_e99345_d_n2;
        locals.var_t0_dn4 = assign64340_e99345_d_n4;
        locals.var_t0_dn5 = assign64340_e99345_d_n5;
        locals.var_t0_dn6 = assign64340_e99345_d_n6;
        locals.var_t0_dn7 = assign64340_e99345_d_n7;
        locals.var_t0_dn8 = assign64340_e99345_d_n8;
        locals.var_t0_dn9 = assign64340_e99345_d_n9;
        locals.var_t0_dn10 = assign64340_e99345_d_n10;
        locals.var_t0_dn11 = assign64340_e99345_d_n11;
        locals.var_t0_dn14 = assign64340_e99345_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign64350_e99356, assign64350_e99356_d_n0, assign64350_e99356_d_n2, assign64350_e99356_d_n4, assign64350_e99356_d_n5, assign64350_e99356_d_n6, assign64350_e99356_d_n7, assign64350_e99356_d_n8, assign64350_e99356_d_n9, assign64350_e99356_d_n10, assign64350_e99356_d_n11, assign64350_e99356_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1515 != 0.0)) {
        let assign64350_e99354: f64 = (locals.var_kusai00 - locals.var_kusail);
        (assign64350_e99354, (locals.var_kusai00_dn0 - locals.var_kusail_dn0), (locals.var_kusai00_dn2 - locals.var_kusail_dn2), (locals.var_kusai00_dn4 - locals.var_kusail_dn4), (locals.var_kusai00_dn5 - locals.var_kusail_dn5), (locals.var_kusai00_dn6 - locals.var_kusail_dn6), (locals.var_kusai00_dn7 - locals.var_kusail_dn7), (locals.var_kusai00_dn8 - locals.var_kusail_dn8), (locals.var_kusai00_dn9 - locals.var_kusail_dn9), (locals.var_kusai00_dn10 - locals.var_kusail_dn10), (locals.var_kusai00_dn11 - locals.var_kusail_dn11), (locals.var_kusai00_dn14 - locals.var_kusail_dn14),)
    } else {
        (locals.var_kusai00l, locals.var_kusai00l_dn0, locals.var_kusai00l_dn2, locals.var_kusai00l_dn4, locals.var_kusai00l_dn5, locals.var_kusai00l_dn6, locals.var_kusai00l_dn7, locals.var_kusai00l_dn8, locals.var_kusai00l_dn9, locals.var_kusai00l_dn10, locals.var_kusai00l_dn11, locals.var_kusai00l_dn14,)
    }
};
        locals.var_kusai00l = assign64350_e99356;
        locals.var_kusai00l_dn0 = assign64350_e99356_d_n0;
        locals.var_kusai00l_dn2 = assign64350_e99356_d_n2;
        locals.var_kusai00l_dn4 = assign64350_e99356_d_n4;
        locals.var_kusai00l_dn5 = assign64350_e99356_d_n5;
        locals.var_kusai00l_dn6 = assign64350_e99356_d_n6;
        locals.var_kusai00l_dn7 = assign64350_e99356_d_n7;
        locals.var_kusai00l_dn8 = assign64350_e99356_d_n8;
        locals.var_kusai00l_dn9 = assign64350_e99356_d_n9;
        locals.var_kusai00l_dn10 = assign64350_e99356_d_n10;
        locals.var_kusai00l_dn11 = assign64350_e99356_d_n11;
        locals.var_kusai00l_dn14 = assign64350_e99356_d_n14;
        locals.var_kusai00l_rv = 0.0;

        let assign64360_e99360: f64 = (10.0 * 2.220446049250313e-16);
        let assign64360_e99365: f64 = (10.0 * 2.220446049250313e-16);
        let assign64360_e99367: f64 = if ((locals.var_qn0 < assign64360_e99360) || (locals.var_kusai00l < assign64360_e99365)) { 1.0 } else { 0.0 };
        locals.var_guard1518 = assign64360_e99367;
        locals.var_guard1518_rv = 0.0;

        let (assign64370_e99378,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1515 != 0.0)) && (locals.var_guard1518 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_ign,)
    }
};
        locals.var_flg_ign = assign64370_e99378;
        locals.var_flg_ign_rv = 0.0;

        let (assign64380_e99390,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1515 != 0.0)) && (locals.var_guard1518 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_ign,)
    }
};
        locals.var_flg_ign = assign64380_e99390;
        locals.var_flg_ign_rv = 0.0;

        let (assign64390_e99397,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_end_of_part_1 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_end_of_part_1,)
    }
};
        locals.var_end_of_part_1 = assign64390_e99397;
        locals.var_end_of_part_1_rv = 0.0;

        let assign64400_e99404: f64 = if ((locals.var_flg_noqi == 0.0) && (locals.var_vgvt > 1e-12)) { 1.0 } else { 0.0 };
        locals.var_guard1519 = assign64400_e99404;
        locals.var_guard1519_rv = 0.0;

        let (assign64410_e99417, assign64410_e99417_d_n0, assign64410_e99417_d_n2, assign64410_e99417_d_n4, assign64410_e99417_d_n5, assign64410_e99417_d_n6, assign64410_e99417_d_n7, assign64410_e99417_d_n8, assign64410_e99417_d_n9, assign64410_e99417_d_n10, assign64410_e99417_d_n11, assign64410_e99417_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign64410_e99411: f64 = (locals.var_fac1 * locals.var_beta);
        let assign64410_e99414: f64 = (2.0 * locals.var_xi0p12);
        let assign64410_e99415: f64 = (assign64410_e99411 / assign64410_e99414);
        (assign64410_e99415, (((((locals.var_fac1_dn0 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn0)) * assign64410_e99414) - (assign64410_e99411 * (2.0 * locals.var_xi0p12_dn0))) / (assign64410_e99414 * assign64410_e99414)), (((((locals.var_fac1_dn2 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn2)) * assign64410_e99414) - (assign64410_e99411 * (2.0 * locals.var_xi0p12_dn2))) / (assign64410_e99414 * assign64410_e99414)), (((((locals.var_fac1_dn4 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn4)) * assign64410_e99414) - (assign64410_e99411 * (2.0 * locals.var_xi0p12_dn4))) / (assign64410_e99414 * assign64410_e99414)), (((((locals.var_fac1_dn5 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn5)) * assign64410_e99414) - (assign64410_e99411 * (2.0 * locals.var_xi0p12_dn5))) / (assign64410_e99414 * assign64410_e99414)), (((((locals.var_fac1_dn6 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn6)) * assign64410_e99414) - (assign64410_e99411 * (2.0 * locals.var_xi0p12_dn6))) / (assign64410_e99414 * assign64410_e99414)), (((((locals.var_fac1_dn7 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn7)) * assign64410_e99414) - (assign64410_e99411 * (2.0 * locals.var_xi0p12_dn7))) / (assign64410_e99414 * assign64410_e99414)), (((((locals.var_fac1_dn8 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn8)) * assign64410_e99414) - (assign64410_e99411 * (2.0 * locals.var_xi0p12_dn8))) / (assign64410_e99414 * assign64410_e99414)), (((((locals.var_fac1_dn9 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn9)) * assign64410_e99414) - (assign64410_e99411 * (2.0 * locals.var_xi0p12_dn9))) / (assign64410_e99414 * assign64410_e99414)), (((((locals.var_fac1_dn10 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn10)) * assign64410_e99414) - (assign64410_e99411 * (2.0 * locals.var_xi0p12_dn10))) / (assign64410_e99414 * assign64410_e99414)), (((((locals.var_fac1_dn11 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn11)) * assign64410_e99414) - (assign64410_e99411 * (2.0 * locals.var_xi0p12_dn11))) / (assign64410_e99414 * assign64410_e99414)), (((((locals.var_fac1_dn14 * locals.var_beta) + (locals.var_fac1 * locals.var_beta_dn14)) * assign64410_e99414) - (assign64410_e99411 * (2.0 * locals.var_xi0p12_dn14))) / (assign64410_e99414 * assign64410_e99414)),)
    } else {
        (locals.var_delta, locals.var_delta_dn0, locals.var_delta_dn2, locals.var_delta_dn4, locals.var_delta_dn5, locals.var_delta_dn6, locals.var_delta_dn7, locals.var_delta_dn8, locals.var_delta_dn9, locals.var_delta_dn10, locals.var_delta_dn11, locals.var_delta_dn14,)
    }
};
        locals.var_delta = assign64410_e99417;
        locals.var_delta_dn0 = assign64410_e99417_d_n0;
        locals.var_delta_dn2 = assign64410_e99417_d_n2;
        locals.var_delta_dn4 = assign64410_e99417_d_n4;
        locals.var_delta_dn5 = assign64410_e99417_d_n5;
        locals.var_delta_dn6 = assign64410_e99417_d_n6;
        locals.var_delta_dn7 = assign64410_e99417_d_n7;
        locals.var_delta_dn8 = assign64410_e99417_d_n8;
        locals.var_delta_dn9 = assign64410_e99417_d_n9;
        locals.var_delta_dn10 = assign64410_e99417_d_n10;
        locals.var_delta_dn11 = assign64410_e99417_d_n11;
        locals.var_delta_dn14 = assign64410_e99417_d_n14;
        locals.var_delta_rv = 0.0;

        let (assign64420_e99430, assign64420_e99430_d_n0, assign64420_e99430_d_n2, assign64420_e99430_d_n4, assign64420_e99430_d_n5, assign64420_e99430_d_n6, assign64420_e99430_d_n7, assign64420_e99430_d_n8, assign64420_e99430_d_n9, assign64420_e99430_d_n10, assign64420_e99430_d_n11, assign64420_e99430_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1519 != 0.0)) {
        let assign64420_e99425: f64 = (1.0 + locals.var_delta);
        let assign64420_e99426: f64 = (locals.var_vgvt / assign64420_e99425);
        let assign64420_e99428: f64 = (assign64420_e99426 + locals.var_ps0);
        (assign64420_e99428, ((((locals.var_vgvt_dn0 * assign64420_e99425) - (locals.var_vgvt * locals.var_delta_dn0)) / (assign64420_e99425 * assign64420_e99425)) + locals.var_ps0_dn0), ((((locals.var_vgvt_dn2 * assign64420_e99425) - (locals.var_vgvt * locals.var_delta_dn2)) / (assign64420_e99425 * assign64420_e99425)) + locals.var_ps0_dn2), ((((locals.var_vgvt_dn4 * assign64420_e99425) - (locals.var_vgvt * locals.var_delta_dn4)) / (assign64420_e99425 * assign64420_e99425)) + locals.var_ps0_dn4), ((((locals.var_vgvt_dn5 * assign64420_e99425) - (locals.var_vgvt * locals.var_delta_dn5)) / (assign64420_e99425 * assign64420_e99425)) + locals.var_ps0_dn5), ((((locals.var_vgvt_dn6 * assign64420_e99425) - (locals.var_vgvt * locals.var_delta_dn6)) / (assign64420_e99425 * assign64420_e99425)) + locals.var_ps0_dn6), ((((locals.var_vgvt_dn7 * assign64420_e99425) - (locals.var_vgvt * locals.var_delta_dn7)) / (assign64420_e99425 * assign64420_e99425)) + locals.var_ps0_dn7), ((((locals.var_vgvt_dn8 * assign64420_e99425) - (locals.var_vgvt * locals.var_delta_dn8)) / (assign64420_e99425 * assign64420_e99425)) + locals.var_ps0_dn8), ((((locals.var_vgvt_dn9 * assign64420_e99425) - (locals.var_vgvt * locals.var_delta_dn9)) / (assign64420_e99425 * assign64420_e99425)) + locals.var_ps0_dn9), ((((locals.var_vgvt_dn10 * assign64420_e99425) - (locals.var_vgvt * locals.var_delta_dn10)) / (assign64420_e99425 * assign64420_e99425)) + locals.var_ps0_dn10), ((((locals.var_vgvt_dn11 * assign64420_e99425) - (locals.var_vgvt * locals.var_delta_dn11)) / (assign64420_e99425 * assign64420_e99425)) + locals.var_ps0_dn11), ((((locals.var_vgvt_dn14 * assign64420_e99425) - (locals.var_vgvt * locals.var_delta_dn14)) / (assign64420_e99425 * assign64420_e99425)) + locals.var_ps0_dn14),)
    } else {
        (locals.var_pslsat, locals.var_pslsat_dn0, locals.var_pslsat_dn2, locals.var_pslsat_dn4, locals.var_pslsat_dn5, locals.var_pslsat_dn6, locals.var_pslsat_dn7, locals.var_pslsat_dn8, locals.var_pslsat_dn9, locals.var_pslsat_dn10, locals.var_pslsat_dn11, locals.var_pslsat_dn14,)
    }
};
        locals.var_pslsat = assign64420_e99430;
        locals.var_pslsat_dn0 = assign64420_e99430_d_n0;
        locals.var_pslsat_dn2 = assign64420_e99430_d_n2;
        locals.var_pslsat_dn4 = assign64420_e99430_d_n4;
        locals.var_pslsat_dn5 = assign64420_e99430_d_n5;
        locals.var_pslsat_dn6 = assign64420_e99430_d_n6;
        locals.var_pslsat_dn7 = assign64420_e99430_d_n7;
        locals.var_pslsat_dn8 = assign64420_e99430_d_n8;
        locals.var_pslsat_dn9 = assign64420_e99430_d_n9;
        locals.var_pslsat_dn10 = assign64420_e99430_d_n10;
        locals.var_pslsat_dn11 = assign64420_e99430_d_n11;
        locals.var_pslsat_dn14 = assign64420_e99430_d_n14;
        locals.var_pslsat_rv = 0.0;

        let (assign64430_e99438, assign64430_e99438_d_n0, assign64430_e99438_d_n2, assign64430_e99438_d_n4, assign64430_e99438_d_n5, assign64430_e99438_d_n6, assign64430_e99438_d_n7, assign64430_e99438_d_n8, assign64430_e99438_d_n9, assign64430_e99438_d_n10, assign64430_e99438_d_n11, assign64430_e99438_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1519 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pslsat, locals.var_pslsat_dn0, locals.var_pslsat_dn2, locals.var_pslsat_dn4, locals.var_pslsat_dn5, locals.var_pslsat_dn6, locals.var_pslsat_dn7, locals.var_pslsat_dn8, locals.var_pslsat_dn9, locals.var_pslsat_dn10, locals.var_pslsat_dn11, locals.var_pslsat_dn14,)
    }
};
        locals.var_pslsat = assign64430_e99438;
        locals.var_pslsat_dn0 = assign64430_e99438_d_n0;
        locals.var_pslsat_dn2 = assign64430_e99438_d_n2;
        locals.var_pslsat_dn4 = assign64430_e99438_d_n4;
        locals.var_pslsat_dn5 = assign64430_e99438_d_n5;
        locals.var_pslsat_dn6 = assign64430_e99438_d_n6;
        locals.var_pslsat_dn7 = assign64430_e99438_d_n7;
        locals.var_pslsat_dn8 = assign64430_e99438_d_n8;
        locals.var_pslsat_dn9 = assign64430_e99438_d_n9;
        locals.var_pslsat_dn10 = assign64430_e99438_d_n10;
        locals.var_pslsat_dn11 = assign64430_e99438_d_n11;
        locals.var_pslsat_dn14 = assign64430_e99438_d_n14;
        locals.var_pslsat_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_241(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign64470_e99460, assign64470_e99460_d_n0, assign64470_e99460_d_n2, assign64470_e99460_d_n4, assign64470_e99460_d_n5, assign64470_e99460_d_n6, assign64470_e99460_d_n7, assign64470_e99460_d_n8, assign64470_e99460_d_n9, assign64470_e99460_d_n10, assign64470_e99460_d_n11, assign64470_e99460_d_n14,) = {
    if (locals.var_guard445 == 0.0) {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn14,)
    } else {
        (locals.var_idsorg, locals.var_idsorg_dn0, locals.var_idsorg_dn2, locals.var_idsorg_dn4, locals.var_idsorg_dn5, locals.var_idsorg_dn6, locals.var_idsorg_dn7, locals.var_idsorg_dn8, locals.var_idsorg_dn9, locals.var_idsorg_dn10, locals.var_idsorg_dn11, locals.var_idsorg_dn14,)
    }
};
        locals.var_idsorg = assign64470_e99460;
        locals.var_idsorg_dn0 = assign64470_e99460_d_n0;
        locals.var_idsorg_dn2 = assign64470_e99460_d_n2;
        locals.var_idsorg_dn4 = assign64470_e99460_d_n4;
        locals.var_idsorg_dn5 = assign64470_e99460_d_n5;
        locals.var_idsorg_dn6 = assign64470_e99460_d_n6;
        locals.var_idsorg_dn7 = assign64470_e99460_d_n7;
        locals.var_idsorg_dn8 = assign64470_e99460_d_n8;
        locals.var_idsorg_dn9 = assign64470_e99460_d_n9;
        locals.var_idsorg_dn10 = assign64470_e99460_d_n10;
        locals.var_idsorg_dn11 = assign64470_e99460_d_n11;
        locals.var_idsorg_dn14 = assign64470_e99460_d_n14;
        locals.var_idsorg_rv = 0.0;

        let (assign64480_e99465, assign64480_e99465_d_n0, assign64480_e99465_d_n2, assign64480_e99465_d_n4, assign64480_e99465_d_n5, assign64480_e99465_d_n6, assign64480_e99465_d_n7, assign64480_e99465_d_n8, assign64480_e99465_d_n9, assign64480_e99465_d_n10, assign64480_e99465_d_n11, assign64480_e99465_d_n14,) = {
    if (locals.var_guard445 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_idspt1, locals.var_idspt1_dn0, locals.var_idspt1_dn2, locals.var_idspt1_dn4, locals.var_idspt1_dn5, locals.var_idspt1_dn6, locals.var_idspt1_dn7, locals.var_idspt1_dn8, locals.var_idspt1_dn9, locals.var_idspt1_dn10, locals.var_idspt1_dn11, locals.var_idspt1_dn14,)
    }
};
        locals.var_idspt1 = assign64480_e99465;
        locals.var_idspt1_dn0 = assign64480_e99465_d_n0;
        locals.var_idspt1_dn2 = assign64480_e99465_d_n2;
        locals.var_idspt1_dn4 = assign64480_e99465_d_n4;
        locals.var_idspt1_dn5 = assign64480_e99465_d_n5;
        locals.var_idspt1_dn6 = assign64480_e99465_d_n6;
        locals.var_idspt1_dn7 = assign64480_e99465_d_n7;
        locals.var_idspt1_dn8 = assign64480_e99465_d_n8;
        locals.var_idspt1_dn9 = assign64480_e99465_d_n9;
        locals.var_idspt1_dn10 = assign64480_e99465_d_n10;
        locals.var_idspt1_dn11 = assign64480_e99465_d_n11;
        locals.var_idspt1_dn14 = assign64480_e99465_d_n14;
        locals.var_idspt1_rv = 0.0;

        let assign64490_e99472: f64 = if ((p.p450 > 0.0) && (p.p454 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1521 = assign64490_e99472;
        locals.var_guard1521_rv = 0.0;

        let (assign64500_e99479,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) {
        (1e-5,)
    } else {
        (locals.var_t_sub,)
    }
};
        locals.var_t_sub = assign64500_e99479;
        locals.var_t_sub_rv = 0.0;

        let (assign64510_e99494, assign64510_e99494_d_n0, assign64510_e99494_d_n2, assign64510_e99494_d_n4, assign64510_e99494_d_n5, assign64510_e99494_d_n6, assign64510_e99494_d_n7, assign64510_e99494_d_n8, assign64510_e99494_d_n9, assign64510_e99494_d_n10, assign64510_e99494_d_n11, assign64510_e99494_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) {
        let assign64510_e99486: f64 = (locals.var_vgs - locals.var_vfb);
        let assign64510_e99488: f64 = (assign64510_e99486 + locals.var_dvth);
        let assign64510_e99490: f64 = (assign64510_e99488 - locals.var_dppg);
        let assign64510_e99492: f64 = (assign64510_e99490 - p.p455);
        (assign64510_e99492, (locals.var_dvth_dn0 - locals.var_dppg_dn0), (locals.var_dvth_dn2 - locals.var_dppg_dn2), (locals.var_dvth_dn4 - locals.var_dppg_dn4), (locals.var_dvth_dn5 - locals.var_dppg_dn5), ((locals.var_vgs_dn6 + locals.var_dvth_dn6) - locals.var_dppg_dn6), ((locals.var_vgs_dn7 + locals.var_dvth_dn7) - locals.var_dppg_dn7), ((locals.var_vgs_dn8 + locals.var_dvth_dn8) - locals.var_dppg_dn8), (locals.var_dvth_dn9 - locals.var_dppg_dn9), (locals.var_dvth_dn10 - locals.var_dppg_dn10), (locals.var_dvth_dn11 - locals.var_dppg_dn11), (locals.var_dvth_dn14 - locals.var_dppg_dn14),)
    } else {
        (locals.var_vgp__blk1527, locals.var_vgp__blk1527_dn0, locals.var_vgp__blk1527_dn2, locals.var_vgp__blk1527_dn4, locals.var_vgp__blk1527_dn5, locals.var_vgp__blk1527_dn6, locals.var_vgp__blk1527_dn7, locals.var_vgp__blk1527_dn8, locals.var_vgp__blk1527_dn9, locals.var_vgp__blk1527_dn10, locals.var_vgp__blk1527_dn11, locals.var_vgp__blk1527_dn14,)
    }
};
        locals.var_vgp__blk1527 = assign64510_e99494;
        locals.var_vgp__blk1527_dn0 = assign64510_e99494_d_n0;
        locals.var_vgp__blk1527_dn2 = assign64510_e99494_d_n2;
        locals.var_vgp__blk1527_dn4 = assign64510_e99494_d_n4;
        locals.var_vgp__blk1527_dn5 = assign64510_e99494_d_n5;
        locals.var_vgp__blk1527_dn6 = assign64510_e99494_d_n6;
        locals.var_vgp__blk1527_dn7 = assign64510_e99494_d_n7;
        locals.var_vgp__blk1527_dn8 = assign64510_e99494_d_n8;
        locals.var_vgp__blk1527_dn9 = assign64510_e99494_d_n9;
        locals.var_vgp__blk1527_dn10 = assign64510_e99494_d_n10;
        locals.var_vgp__blk1527_dn11 = assign64510_e99494_d_n11;
        locals.var_vgp__blk1527_dn14 = assign64510_e99494_d_n14;
        locals.var_vgp__blk1527_rv = 0.0;

        let (assign64520_e99503, assign64520_e99503_d_n0, assign64520_e99503_d_n2, assign64520_e99503_d_n4, assign64520_e99503_d_n5, assign64520_e99503_d_n6, assign64520_e99503_d_n7, assign64520_e99503_d_n8, assign64520_e99503_d_n9, assign64520_e99503_d_n10, assign64520_e99503_d_n11, assign64520_e99503_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) {
        let assign64520_e99501: f64 = (locals.var_vth + p.p455);
        (assign64520_e99501, locals.var_vth_dn0, locals.var_vth_dn2, locals.var_vth_dn4, locals.var_vth_dn5, locals.var_vth_dn6, locals.var_vth_dn7, locals.var_vth_dn8, locals.var_vth_dn9, locals.var_vth_dn10, locals.var_vth_dn11, locals.var_vth_dn14,)
    } else {
        (locals.var_wk_vth, locals.var_wk_vth_dn0, locals.var_wk_vth_dn2, locals.var_wk_vth_dn4, locals.var_wk_vth_dn5, locals.var_wk_vth_dn6, locals.var_wk_vth_dn7, locals.var_wk_vth_dn8, locals.var_wk_vth_dn9, locals.var_wk_vth_dn10, locals.var_wk_vth_dn11, locals.var_wk_vth_dn14,)
    }
};
        locals.var_wk_vth = assign64520_e99503;
        locals.var_wk_vth_dn0 = assign64520_e99503_d_n0;
        locals.var_wk_vth_dn2 = assign64520_e99503_d_n2;
        locals.var_wk_vth_dn4 = assign64520_e99503_d_n4;
        locals.var_wk_vth_dn5 = assign64520_e99503_d_n5;
        locals.var_wk_vth_dn6 = assign64520_e99503_d_n6;
        locals.var_wk_vth_dn7 = assign64520_e99503_d_n7;
        locals.var_wk_vth_dn8 = assign64520_e99503_d_n8;
        locals.var_wk_vth_dn9 = assign64520_e99503_d_n9;
        locals.var_wk_vth_dn10 = assign64520_e99503_d_n10;
        locals.var_wk_vth_dn11 = assign64520_e99503_d_n11;
        locals.var_wk_vth_dn14 = assign64520_e99503_d_n14;
        locals.var_wk_vth_rv = 0.0;

        let (assign64530_e99523, assign64530_e99523_d_n0, assign64530_e99523_d_n2, assign64530_e99523_d_n4, assign64530_e99523_d_n5, assign64530_e99523_d_n6, assign64530_e99523_d_n7, assign64530_e99523_d_n8, assign64530_e99523_d_n9, assign64530_e99523_d_n10, assign64530_e99523_d_n11, assign64530_e99523_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) {
        let assign64530_e99510: f64 = (locals.var_vbipn - locals.var_vbscl__blk437);
        let assign64530_e99513: f64 = (locals.var_vbipn - locals.var_vbscl__blk437);
        let assign64530_e99514: f64 = (assign64530_e99510 * assign64530_e99513);
        let assign64530_e99517: f64 = (4.0 * 0.01);
        let assign64530_e99519: f64 = (assign64530_e99517 * 0.01);
        let assign64530_e99520: f64 = (assign64530_e99514 + assign64530_e99519);
        let assign64530_e99521: f64 = (assign64530_e99520).sqrt();
        (assign64530_e99521, ((((locals.var_vbipn_dn0 - locals.var_vbscl__blk437_dn0) * assign64530_e99513) + (assign64530_e99510 * (locals.var_vbipn_dn0 - locals.var_vbscl__blk437_dn0))) / (2.0 * assign64530_e99521)), ((((locals.var_vbipn_dn2 - locals.var_vbscl__blk437_dn2) * assign64530_e99513) + (assign64530_e99510 * (locals.var_vbipn_dn2 - locals.var_vbscl__blk437_dn2))) / (2.0 * assign64530_e99521)), ((((locals.var_vbipn_dn4 - locals.var_vbscl__blk437_dn4) * assign64530_e99513) + (assign64530_e99510 * (locals.var_vbipn_dn4 - locals.var_vbscl__blk437_dn4))) / (2.0 * assign64530_e99521)), ((((locals.var_vbipn_dn5 - locals.var_vbscl__blk437_dn5) * assign64530_e99513) + (assign64530_e99510 * (locals.var_vbipn_dn5 - locals.var_vbscl__blk437_dn5))) / (2.0 * assign64530_e99521)), ((((locals.var_vbipn_dn6 - locals.var_vbscl__blk437_dn6) * assign64530_e99513) + (assign64530_e99510 * (locals.var_vbipn_dn6 - locals.var_vbscl__blk437_dn6))) / (2.0 * assign64530_e99521)), ((((locals.var_vbipn_dn7 - locals.var_vbscl__blk437_dn7) * assign64530_e99513) + (assign64530_e99510 * (locals.var_vbipn_dn7 - locals.var_vbscl__blk437_dn7))) / (2.0 * assign64530_e99521)), ((((locals.var_vbipn_dn8 - locals.var_vbscl__blk437_dn8) * assign64530_e99513) + (assign64530_e99510 * (locals.var_vbipn_dn8 - locals.var_vbscl__blk437_dn8))) / (2.0 * assign64530_e99521)), ((((locals.var_vbipn_dn9 - locals.var_vbscl__blk437_dn9) * assign64530_e99513) + (assign64530_e99510 * (locals.var_vbipn_dn9 - locals.var_vbscl__blk437_dn9))) / (2.0 * assign64530_e99521)), ((((locals.var_vbipn_dn10 - locals.var_vbscl__blk437_dn10) * assign64530_e99513) + (assign64530_e99510 * (locals.var_vbipn_dn10 - locals.var_vbscl__blk437_dn10))) / (2.0 * assign64530_e99521)), ((((locals.var_vbipn_dn11 - locals.var_vbscl__blk437_dn11) * assign64530_e99513) + (assign64530_e99510 * (locals.var_vbipn_dn11 - locals.var_vbscl__blk437_dn11))) / (2.0 * assign64530_e99521)), ((((locals.var_vbipn_dn14 - locals.var_vbscl__blk437_dn14) * assign64530_e99513) + (assign64530_e99510 * (locals.var_vbipn_dn14 - locals.var_vbscl__blk437_dn14))) / (2.0 * assign64530_e99521)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign64530_e99523;
        locals.var_tmf1_dn0 = assign64530_e99523_d_n0;
        locals.var_tmf1_dn2 = assign64530_e99523_d_n2;
        locals.var_tmf1_dn4 = assign64530_e99523_d_n4;
        locals.var_tmf1_dn5 = assign64530_e99523_d_n5;
        locals.var_tmf1_dn6 = assign64530_e99523_d_n6;
        locals.var_tmf1_dn7 = assign64530_e99523_d_n7;
        locals.var_tmf1_dn8 = assign64530_e99523_d_n8;
        locals.var_tmf1_dn9 = assign64530_e99523_d_n9;
        locals.var_tmf1_dn10 = assign64530_e99523_d_n10;
        locals.var_tmf1_dn11 = assign64530_e99523_d_n11;
        locals.var_tmf1_dn14 = assign64530_e99523_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign64540_e99536, assign64540_e99536_d_n0, assign64540_e99536_d_n2, assign64540_e99536_d_n4, assign64540_e99536_d_n5, assign64540_e99536_d_n6, assign64540_e99536_d_n7, assign64540_e99536_d_n8, assign64540_e99536_d_n9, assign64540_e99536_d_n10, assign64540_e99536_d_n11, assign64540_e99536_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) {
        let assign64540_e99531: f64 = (locals.var_vbipn - locals.var_vbscl__blk437);
        let assign64540_e99533: f64 = (assign64540_e99531 + locals.var_tmf1);
        let assign64540_e99534: f64 = (0.5 * assign64540_e99533);
        (assign64540_e99534, (0.5 * ((locals.var_vbipn_dn0 - locals.var_vbscl__blk437_dn0) + locals.var_tmf1_dn0)), (0.5 * ((locals.var_vbipn_dn2 - locals.var_vbscl__blk437_dn2) + locals.var_tmf1_dn2)), (0.5 * ((locals.var_vbipn_dn4 - locals.var_vbscl__blk437_dn4) + locals.var_tmf1_dn4)), (0.5 * ((locals.var_vbipn_dn5 - locals.var_vbscl__blk437_dn5) + locals.var_tmf1_dn5)), (0.5 * ((locals.var_vbipn_dn6 - locals.var_vbscl__blk437_dn6) + locals.var_tmf1_dn6)), (0.5 * ((locals.var_vbipn_dn7 - locals.var_vbscl__blk437_dn7) + locals.var_tmf1_dn7)), (0.5 * ((locals.var_vbipn_dn8 - locals.var_vbscl__blk437_dn8) + locals.var_tmf1_dn8)), (0.5 * ((locals.var_vbipn_dn9 - locals.var_vbscl__blk437_dn9) + locals.var_tmf1_dn9)), (0.5 * ((locals.var_vbipn_dn10 - locals.var_vbscl__blk437_dn10) + locals.var_tmf1_dn10)), (0.5 * ((locals.var_vbipn_dn11 - locals.var_vbscl__blk437_dn11) + locals.var_tmf1_dn11)), (0.5 * ((locals.var_vbipn_dn14 - locals.var_vbscl__blk437_dn14) + locals.var_tmf1_dn14)),)
    } else {
        (locals.var_vpositive, locals.var_vpositive_dn0, locals.var_vpositive_dn2, locals.var_vpositive_dn4, locals.var_vpositive_dn5, locals.var_vpositive_dn6, locals.var_vpositive_dn7, locals.var_vpositive_dn8, locals.var_vpositive_dn9, locals.var_vpositive_dn10, locals.var_vpositive_dn11, locals.var_vpositive_dn14,)
    }
};
        locals.var_vpositive = assign64540_e99536;
        locals.var_vpositive_dn0 = assign64540_e99536_d_n0;
        locals.var_vpositive_dn2 = assign64540_e99536_d_n2;
        locals.var_vpositive_dn4 = assign64540_e99536_d_n4;
        locals.var_vpositive_dn5 = assign64540_e99536_d_n5;
        locals.var_vpositive_dn6 = assign64540_e99536_d_n6;
        locals.var_vpositive_dn7 = assign64540_e99536_d_n7;
        locals.var_vpositive_dn8 = assign64540_e99536_d_n8;
        locals.var_vpositive_dn9 = assign64540_e99536_d_n9;
        locals.var_vpositive_dn10 = assign64540_e99536_d_n10;
        locals.var_vpositive_dn11 = assign64540_e99536_d_n11;
        locals.var_vpositive_dn14 = assign64540_e99536_d_n14;
        locals.var_vpositive_rv = 0.0;

        let (assign64550_e99558, assign64550_e99558_d_n0, assign64550_e99558_d_n2, assign64550_e99558_d_n4, assign64550_e99558_d_n5, assign64550_e99558_d_n6, assign64550_e99558_d_n7, assign64550_e99558_d_n8, assign64550_e99558_d_n9, assign64550_e99558_d_n10, assign64550_e99558_d_n11, assign64550_e99558_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) {
        let assign64550_e99543: f64 = (2.0 * 1.6021918e-19);
        let assign64550_e99545: f64 = (assign64550_e99543 * locals.var_vpositive);
        let assign64550_e99547: f64 = (assign64550_e99545 / 1.034943e-10);
        let assign64550_e99549: f64 = (assign64550_e99547 * locals.var_nsub);
        let assign64550_e99551: f64 = (assign64550_e99549 * locals.var_uc_njunc);
        let assign64550_e99554: f64 = (locals.var_nsub + locals.var_uc_njunc);
        let assign64550_e99555: f64 = (assign64550_e99551 / assign64550_e99554);
        let assign64550_e99556: f64 = (assign64550_e99555).sqrt();
        (assign64550_e99556, (((((((((assign64550_e99543 * locals.var_vpositive_dn0) / 1.034943e-10) * locals.var_nsub) + (assign64550_e99547 * locals.var_nsub_dn0)) * locals.var_uc_njunc) * assign64550_e99554) - (assign64550_e99551 * locals.var_nsub_dn0)) / (assign64550_e99554 * assign64550_e99554)) / (2.0 * assign64550_e99556)), (((((((((assign64550_e99543 * locals.var_vpositive_dn2) / 1.034943e-10) * locals.var_nsub) + (assign64550_e99547 * locals.var_nsub_dn2)) * locals.var_uc_njunc) * assign64550_e99554) - (assign64550_e99551 * locals.var_nsub_dn2)) / (assign64550_e99554 * assign64550_e99554)) / (2.0 * assign64550_e99556)), (((((((((assign64550_e99543 * locals.var_vpositive_dn4) / 1.034943e-10) * locals.var_nsub) + (assign64550_e99547 * locals.var_nsub_dn4)) * locals.var_uc_njunc) * assign64550_e99554) - (assign64550_e99551 * locals.var_nsub_dn4)) / (assign64550_e99554 * assign64550_e99554)) / (2.0 * assign64550_e99556)), (((((((((assign64550_e99543 * locals.var_vpositive_dn5) / 1.034943e-10) * locals.var_nsub) + (assign64550_e99547 * locals.var_nsub_dn5)) * locals.var_uc_njunc) * assign64550_e99554) - (assign64550_e99551 * locals.var_nsub_dn5)) / (assign64550_e99554 * assign64550_e99554)) / (2.0 * assign64550_e99556)), (((((((((assign64550_e99543 * locals.var_vpositive_dn6) / 1.034943e-10) * locals.var_nsub) + (assign64550_e99547 * locals.var_nsub_dn6)) * locals.var_uc_njunc) * assign64550_e99554) - (assign64550_e99551 * locals.var_nsub_dn6)) / (assign64550_e99554 * assign64550_e99554)) / (2.0 * assign64550_e99556)), (((((((((assign64550_e99543 * locals.var_vpositive_dn7) / 1.034943e-10) * locals.var_nsub) + (assign64550_e99547 * locals.var_nsub_dn7)) * locals.var_uc_njunc) * assign64550_e99554) - (assign64550_e99551 * locals.var_nsub_dn7)) / (assign64550_e99554 * assign64550_e99554)) / (2.0 * assign64550_e99556)), (((((((((assign64550_e99543 * locals.var_vpositive_dn8) / 1.034943e-10) * locals.var_nsub) + (assign64550_e99547 * locals.var_nsub_dn8)) * locals.var_uc_njunc) * assign64550_e99554) - (assign64550_e99551 * locals.var_nsub_dn8)) / (assign64550_e99554 * assign64550_e99554)) / (2.0 * assign64550_e99556)), (((((((((assign64550_e99543 * locals.var_vpositive_dn9) / 1.034943e-10) * locals.var_nsub) + (assign64550_e99547 * locals.var_nsub_dn9)) * locals.var_uc_njunc) * assign64550_e99554) - (assign64550_e99551 * locals.var_nsub_dn9)) / (assign64550_e99554 * assign64550_e99554)) / (2.0 * assign64550_e99556)), (((((((((assign64550_e99543 * locals.var_vpositive_dn10) / 1.034943e-10) * locals.var_nsub) + (assign64550_e99547 * locals.var_nsub_dn10)) * locals.var_uc_njunc) * assign64550_e99554) - (assign64550_e99551 * locals.var_nsub_dn10)) / (assign64550_e99554 * assign64550_e99554)) / (2.0 * assign64550_e99556)), (((((((((assign64550_e99543 * locals.var_vpositive_dn11) / 1.034943e-10) * locals.var_nsub) + (assign64550_e99547 * locals.var_nsub_dn11)) * locals.var_uc_njunc) * assign64550_e99554) - (assign64550_e99551 * locals.var_nsub_dn11)) / (assign64550_e99554 * assign64550_e99554)) / (2.0 * assign64550_e99556)), (((((((((assign64550_e99543 * locals.var_vpositive_dn14) / 1.034943e-10) * locals.var_nsub) + (assign64550_e99547 * locals.var_nsub_dn14)) * locals.var_uc_njunc) * assign64550_e99554) - (assign64550_e99551 * locals.var_nsub_dn14)) / (assign64550_e99554 * assign64550_e99554)) / (2.0 * assign64550_e99556)),)
    } else {
        (locals.var_ec__blk1522, locals.var_ec__blk1522_dn0, locals.var_ec__blk1522_dn2, locals.var_ec__blk1522_dn4, locals.var_ec__blk1522_dn5, locals.var_ec__blk1522_dn6, locals.var_ec__blk1522_dn7, locals.var_ec__blk1522_dn8, locals.var_ec__blk1522_dn9, locals.var_ec__blk1522_dn10, locals.var_ec__blk1522_dn11, locals.var_ec__blk1522_dn14,)
    }
};
        locals.var_ec__blk1522 = assign64550_e99558;
        locals.var_ec__blk1522_dn0 = assign64550_e99558_d_n0;
        locals.var_ec__blk1522_dn2 = assign64550_e99558_d_n2;
        locals.var_ec__blk1522_dn4 = assign64550_e99558_d_n4;
        locals.var_ec__blk1522_dn5 = assign64550_e99558_d_n5;
        locals.var_ec__blk1522_dn6 = assign64550_e99558_d_n6;
        locals.var_ec__blk1522_dn7 = assign64550_e99558_d_n7;
        locals.var_ec__blk1522_dn8 = assign64550_e99558_d_n8;
        locals.var_ec__blk1522_dn9 = assign64550_e99558_d_n9;
        locals.var_ec__blk1522_dn10 = assign64550_e99558_d_n10;
        locals.var_ec__blk1522_dn11 = assign64550_e99558_d_n11;
        locals.var_ec__blk1522_dn14 = assign64550_e99558_d_n14;
        locals.var_ec__blk1522_rv = 0.0;

        let (assign64560_e99567, assign64560_e99567_d_n0, assign64560_e99567_d_n2, assign64560_e99567_d_n4, assign64560_e99567_d_n5, assign64560_e99567_d_n6, assign64560_e99567_d_n7, assign64560_e99567_d_n8, assign64560_e99567_d_n9, assign64560_e99567_d_n10, assign64560_e99567_d_n11, assign64560_e99567_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) {
        let assign64560_e99565: f64 = (locals.var_ec__blk1522 * locals.var_leff);
        (assign64560_e99565, (locals.var_ec__blk1522_dn0 * locals.var_leff), (locals.var_ec__blk1522_dn2 * locals.var_leff), (locals.var_ec__blk1522_dn4 * locals.var_leff), (locals.var_ec__blk1522_dn5 * locals.var_leff), (locals.var_ec__blk1522_dn6 * locals.var_leff), (locals.var_ec__blk1522_dn7 * locals.var_leff), (locals.var_ec__blk1522_dn8 * locals.var_leff), (locals.var_ec__blk1522_dn9 * locals.var_leff), (locals.var_ec__blk1522_dn10 * locals.var_leff), (locals.var_ec__blk1522_dn11 * locals.var_leff), (locals.var_ec__blk1522_dn14 * locals.var_leff),)
    } else {
        (locals.var_wk, locals.var_wk_dn0, locals.var_wk_dn2, locals.var_wk_dn4, locals.var_wk_dn5, locals.var_wk_dn6, locals.var_wk_dn7, locals.var_wk_dn8, locals.var_wk_dn9, locals.var_wk_dn10, locals.var_wk_dn11, locals.var_wk_dn14,)
    }
};
        locals.var_wk = assign64560_e99567;
        locals.var_wk_dn0 = assign64560_e99567_d_n0;
        locals.var_wk_dn2 = assign64560_e99567_d_n2;
        locals.var_wk_dn4 = assign64560_e99567_d_n4;
        locals.var_wk_dn5 = assign64560_e99567_d_n5;
        locals.var_wk_dn6 = assign64560_e99567_d_n6;
        locals.var_wk_dn7 = assign64560_e99567_d_n7;
        locals.var_wk_dn8 = assign64560_e99567_d_n8;
        locals.var_wk_dn9 = assign64560_e99567_d_n9;
        locals.var_wk_dn10 = assign64560_e99567_d_n10;
        locals.var_wk_dn11 = assign64560_e99567_d_n11;
        locals.var_wk_dn14 = assign64560_e99567_d_n14;
        locals.var_wk_rv = 0.0;

        let (assign64570_e99583, assign64570_e99583_d_n0, assign64570_e99583_d_n2, assign64570_e99583_d_n4, assign64570_e99583_d_n5, assign64570_e99583_d_n6, assign64570_e99583_d_n7, assign64570_e99583_d_n8, assign64570_e99583_d_n9, assign64570_e99583_d_n10, assign64570_e99583_d_n11, assign64570_e99583_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) {
        let assign64570_e99573: f64 = (-0.25);
        let assign64570_e99575: f64 = (assign64570_e99573 * locals.var_wk);
        let assign64570_e99577: f64 = (assign64570_e99575 * locals.var_wk);
        let assign64570_e99580: f64 = (locals.var_vds + locals.var_wk);
        let assign64570_e99581: f64 = (assign64570_e99577 / assign64570_e99580);
        (assign64570_e99581, ((((((assign64570_e99573 * locals.var_wk_dn0) * locals.var_wk) + (assign64570_e99575 * locals.var_wk_dn0)) * assign64570_e99580) - (assign64570_e99577 * (locals.var_vds_dn0 + locals.var_wk_dn0))) / (assign64570_e99580 * assign64570_e99580)), ((((((assign64570_e99573 * locals.var_wk_dn2) * locals.var_wk) + (assign64570_e99575 * locals.var_wk_dn2)) * assign64570_e99580) - (assign64570_e99577 * (locals.var_vds_dn2 + locals.var_wk_dn2))) / (assign64570_e99580 * assign64570_e99580)), ((((((assign64570_e99573 * locals.var_wk_dn4) * locals.var_wk) + (assign64570_e99575 * locals.var_wk_dn4)) * assign64570_e99580) - (assign64570_e99577 * (locals.var_vds_dn4 + locals.var_wk_dn4))) / (assign64570_e99580 * assign64570_e99580)), ((((((assign64570_e99573 * locals.var_wk_dn5) * locals.var_wk) + (assign64570_e99575 * locals.var_wk_dn5)) * assign64570_e99580) - (assign64570_e99577 * (locals.var_vds_dn5 + locals.var_wk_dn5))) / (assign64570_e99580 * assign64570_e99580)), ((((((assign64570_e99573 * locals.var_wk_dn6) * locals.var_wk) + (assign64570_e99575 * locals.var_wk_dn6)) * assign64570_e99580) - (assign64570_e99577 * (locals.var_vds_dn6 + locals.var_wk_dn6))) / (assign64570_e99580 * assign64570_e99580)), ((((((assign64570_e99573 * locals.var_wk_dn7) * locals.var_wk) + (assign64570_e99575 * locals.var_wk_dn7)) * assign64570_e99580) - (assign64570_e99577 * (locals.var_vds_dn7 + locals.var_wk_dn7))) / (assign64570_e99580 * assign64570_e99580)), ((((((assign64570_e99573 * locals.var_wk_dn8) * locals.var_wk) + (assign64570_e99575 * locals.var_wk_dn8)) * assign64570_e99580) - (assign64570_e99577 * (locals.var_vds_dn8 + locals.var_wk_dn8))) / (assign64570_e99580 * assign64570_e99580)), ((((((assign64570_e99573 * locals.var_wk_dn9) * locals.var_wk) + (assign64570_e99575 * locals.var_wk_dn9)) * assign64570_e99580) - (assign64570_e99577 * (locals.var_vds_dn9 + locals.var_wk_dn9))) / (assign64570_e99580 * assign64570_e99580)), ((((((assign64570_e99573 * locals.var_wk_dn10) * locals.var_wk) + (assign64570_e99575 * locals.var_wk_dn10)) * assign64570_e99580) - (assign64570_e99577 * (locals.var_vds_dn10 + locals.var_wk_dn10))) / (assign64570_e99580 * assign64570_e99580)), ((((((assign64570_e99573 * locals.var_wk_dn11) * locals.var_wk) + (assign64570_e99575 * locals.var_wk_dn11)) * assign64570_e99580) - (assign64570_e99577 * (locals.var_vds_dn11 + locals.var_wk_dn11))) / (assign64570_e99580 * assign64570_e99580)), ((((((assign64570_e99573 * locals.var_wk_dn14) * locals.var_wk) + (assign64570_e99575 * locals.var_wk_dn14)) * assign64570_e99580) - (assign64570_e99577 * (locals.var_vds_dn14 + locals.var_wk_dn14))) / (assign64570_e99580 * assign64570_e99580)),)
    } else {
        (locals.var_dphi_vds, locals.var_dphi_vds_dn0, locals.var_dphi_vds_dn2, locals.var_dphi_vds_dn4, locals.var_dphi_vds_dn5, locals.var_dphi_vds_dn6, locals.var_dphi_vds_dn7, locals.var_dphi_vds_dn8, locals.var_dphi_vds_dn9, locals.var_dphi_vds_dn10, locals.var_dphi_vds_dn11, locals.var_dphi_vds_dn14,)
    }
};
        locals.var_dphi_vds = assign64570_e99583;
        locals.var_dphi_vds_dn0 = assign64570_e99583_d_n0;
        locals.var_dphi_vds_dn2 = assign64570_e99583_d_n2;
        locals.var_dphi_vds_dn4 = assign64570_e99583_d_n4;
        locals.var_dphi_vds_dn5 = assign64570_e99583_d_n5;
        locals.var_dphi_vds_dn6 = assign64570_e99583_d_n6;
        locals.var_dphi_vds_dn7 = assign64570_e99583_d_n7;
        locals.var_dphi_vds_dn8 = assign64570_e99583_d_n8;
        locals.var_dphi_vds_dn9 = assign64570_e99583_d_n9;
        locals.var_dphi_vds_dn10 = assign64570_e99583_d_n10;
        locals.var_dphi_vds_dn11 = assign64570_e99583_d_n11;
        locals.var_dphi_vds_dn14 = assign64570_e99583_d_n14;
        locals.var_dphi_vds_rv = 0.0;

        let assign64580_e99586: f64 = if p.p457 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1540 = assign64580_e99586;
        locals.var_guard1540_rv = 0.0;

        let (assign64590_e99595, assign64590_e99595_d_n0, assign64590_e99595_d_n2, assign64590_e99595_d_n4, assign64590_e99595_d_n5, assign64590_e99595_d_n6, assign64590_e99595_d_n7, assign64590_e99595_d_n8, assign64590_e99595_d_n9, assign64590_e99595_d_n10, assign64590_e99595_d_n11, assign64590_e99595_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 != 0.0)) {
        (p.p457, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps0__blk1525, locals.var_ps0__blk1525_dn0, locals.var_ps0__blk1525_dn2, locals.var_ps0__blk1525_dn4, locals.var_ps0__blk1525_dn5, locals.var_ps0__blk1525_dn6, locals.var_ps0__blk1525_dn7, locals.var_ps0__blk1525_dn8, locals.var_ps0__blk1525_dn9, locals.var_ps0__blk1525_dn10, locals.var_ps0__blk1525_dn11, locals.var_ps0__blk1525_dn14,)
    }
};
        locals.var_ps0__blk1525 = assign64590_e99595;
        locals.var_ps0__blk1525_dn0 = assign64590_e99595_d_n0;
        locals.var_ps0__blk1525_dn2 = assign64590_e99595_d_n2;
        locals.var_ps0__blk1525_dn4 = assign64590_e99595_d_n4;
        locals.var_ps0__blk1525_dn5 = assign64590_e99595_d_n5;
        locals.var_ps0__blk1525_dn6 = assign64590_e99595_d_n6;
        locals.var_ps0__blk1525_dn7 = assign64590_e99595_d_n7;
        locals.var_ps0__blk1525_dn8 = assign64590_e99595_d_n8;
        locals.var_ps0__blk1525_dn9 = assign64590_e99595_d_n9;
        locals.var_ps0__blk1525_dn10 = assign64590_e99595_d_n10;
        locals.var_ps0__blk1525_dn11 = assign64590_e99595_d_n11;
        locals.var_ps0__blk1525_dn14 = assign64590_e99595_d_n14;
        locals.var_ps0__blk1525_rv = 0.0;

        let (assign64600_e99605, assign64600_e99605_d_n0, assign64600_e99605_d_n2, assign64600_e99605_d_n4, assign64600_e99605_d_n5, assign64600_e99605_d_n6, assign64600_e99605_d_n7, assign64600_e99605_d_n8, assign64600_e99605_d_n9, assign64600_e99605_d_n10, assign64600_e99605_d_n11, assign64600_e99605_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) {
        (locals.var_dphi_vds, locals.var_dphi_vds_dn0, locals.var_dphi_vds_dn2, locals.var_dphi_vds_dn4, locals.var_dphi_vds_dn5, locals.var_dphi_vds_dn6, locals.var_dphi_vds_dn7, locals.var_dphi_vds_dn8, locals.var_dphi_vds_dn9, locals.var_dphi_vds_dn10, locals.var_dphi_vds_dn11, locals.var_dphi_vds_dn14,)
    } else {
        (locals.var_vbscl__blk1541, locals.var_vbscl__blk1541_dn0, locals.var_vbscl__blk1541_dn2, locals.var_vbscl__blk1541_dn4, locals.var_vbscl__blk1541_dn5, locals.var_vbscl__blk1541_dn6, locals.var_vbscl__blk1541_dn7, locals.var_vbscl__blk1541_dn8, locals.var_vbscl__blk1541_dn9, locals.var_vbscl__blk1541_dn10, locals.var_vbscl__blk1541_dn11, locals.var_vbscl__blk1541_dn14,)
    }
};
        locals.var_vbscl__blk1541 = assign64600_e99605;
        locals.var_vbscl__blk1541_dn0 = assign64600_e99605_d_n0;
        locals.var_vbscl__blk1541_dn2 = assign64600_e99605_d_n2;
        locals.var_vbscl__blk1541_dn4 = assign64600_e99605_d_n4;
        locals.var_vbscl__blk1541_dn5 = assign64600_e99605_d_n5;
        locals.var_vbscl__blk1541_dn6 = assign64600_e99605_d_n6;
        locals.var_vbscl__blk1541_dn7 = assign64600_e99605_d_n7;
        locals.var_vbscl__blk1541_dn8 = assign64600_e99605_d_n8;
        locals.var_vbscl__blk1541_dn9 = assign64600_e99605_d_n9;
        locals.var_vbscl__blk1541_dn10 = assign64600_e99605_d_n10;
        locals.var_vbscl__blk1541_dn11 = assign64600_e99605_d_n11;
        locals.var_vbscl__blk1541_dn14 = assign64600_e99605_d_n14;
        locals.var_vbscl__blk1541_rv = 0.0;

        let (assign64610_e99615, assign64610_e99615_d_n0, assign64610_e99615_d_n2, assign64610_e99615_d_n4, assign64610_e99615_d_n5, assign64610_e99615_d_n6, assign64610_e99615_d_n7, assign64610_e99615_d_n8, assign64610_e99615_d_n9, assign64610_e99615_d_n10, assign64610_e99615_d_n11, assign64610_e99615_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) {
        (locals.var_wk_vth, locals.var_wk_vth_dn0, locals.var_wk_vth_dn2, locals.var_wk_vth_dn4, locals.var_wk_vth_dn5, locals.var_wk_vth_dn6, locals.var_wk_vth_dn7, locals.var_wk_vth_dn8, locals.var_wk_vth_dn9, locals.var_wk_vth_dn10, locals.var_wk_vth_dn11, locals.var_wk_vth_dn14,)
    } else {
        (locals.var_vth__blk1542, locals.var_vth__blk1542_dn0, locals.var_vth__blk1542_dn2, locals.var_vth__blk1542_dn4, locals.var_vth__blk1542_dn5, locals.var_vth__blk1542_dn6, locals.var_vth__blk1542_dn7, locals.var_vth__blk1542_dn8, locals.var_vth__blk1542_dn9, locals.var_vth__blk1542_dn10, locals.var_vth__blk1542_dn11, locals.var_vth__blk1542_dn14,)
    }
};
        locals.var_vth__blk1542 = assign64610_e99615;
        locals.var_vth__blk1542_dn0 = assign64610_e99615_d_n0;
        locals.var_vth__blk1542_dn2 = assign64610_e99615_d_n2;
        locals.var_vth__blk1542_dn4 = assign64610_e99615_d_n4;
        locals.var_vth__blk1542_dn5 = assign64610_e99615_d_n5;
        locals.var_vth__blk1542_dn6 = assign64610_e99615_d_n6;
        locals.var_vth__blk1542_dn7 = assign64610_e99615_d_n7;
        locals.var_vth__blk1542_dn8 = assign64610_e99615_d_n8;
        locals.var_vth__blk1542_dn9 = assign64610_e99615_d_n9;
        locals.var_vth__blk1542_dn10 = assign64610_e99615_d_n10;
        locals.var_vth__blk1542_dn11 = assign64610_e99615_d_n11;
        locals.var_vth__blk1542_dn14 = assign64610_e99615_d_n14;
        locals.var_vth__blk1542_rv = 0.0;

        let (assign64620_e99639, assign64620_e99639_d_n0, assign64620_e99639_d_n2, assign64620_e99639_d_n4, assign64620_e99639_d_n5, assign64620_e99639_d_n6, assign64620_e99639_d_n7, assign64620_e99639_d_n8, assign64620_e99639_d_n9, assign64620_e99639_d_n10, assign64620_e99639_d_n11, assign64620_e99639_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) {
        let assign64620_e99628: f64 = (locals.var_vgp__blk1527 - locals.var_vbscl__blk1541);
        let assign64620_e99629: f64 = (locals.var_beta * assign64620_e99628);
        let assign64620_e99631: f64 = (assign64620_e99629 - 1.0);
        let assign64620_e99632: f64 = (4.0 * assign64620_e99631);
        let assign64620_e99635: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign64620_e99636: f64 = (assign64620_e99632 / assign64620_e99635);
        let assign64620_e99637: f64 = (1.0 + assign64620_e99636);
        (assign64620_e99637, ((((4.0 * ((locals.var_beta_dn0 * assign64620_e99628) + (locals.var_beta * (locals.var_vgp__blk1527_dn0 - locals.var_vbscl__blk1541_dn0)))) * assign64620_e99635) - (assign64620_e99632 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign64620_e99635 * assign64620_e99635)), ((((4.0 * ((locals.var_beta_dn2 * assign64620_e99628) + (locals.var_beta * (locals.var_vgp__blk1527_dn2 - locals.var_vbscl__blk1541_dn2)))) * assign64620_e99635) - (assign64620_e99632 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign64620_e99635 * assign64620_e99635)), ((((4.0 * ((locals.var_beta_dn4 * assign64620_e99628) + (locals.var_beta * (locals.var_vgp__blk1527_dn4 - locals.var_vbscl__blk1541_dn4)))) * assign64620_e99635) - (assign64620_e99632 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign64620_e99635 * assign64620_e99635)), ((((4.0 * ((locals.var_beta_dn5 * assign64620_e99628) + (locals.var_beta * (locals.var_vgp__blk1527_dn5 - locals.var_vbscl__blk1541_dn5)))) * assign64620_e99635) - (assign64620_e99632 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign64620_e99635 * assign64620_e99635)), ((((4.0 * ((locals.var_beta_dn6 * assign64620_e99628) + (locals.var_beta * (locals.var_vgp__blk1527_dn6 - locals.var_vbscl__blk1541_dn6)))) * assign64620_e99635) - (assign64620_e99632 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign64620_e99635 * assign64620_e99635)), ((((4.0 * ((locals.var_beta_dn7 * assign64620_e99628) + (locals.var_beta * (locals.var_vgp__blk1527_dn7 - locals.var_vbscl__blk1541_dn7)))) * assign64620_e99635) - (assign64620_e99632 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign64620_e99635 * assign64620_e99635)), ((((4.0 * ((locals.var_beta_dn8 * assign64620_e99628) + (locals.var_beta * (locals.var_vgp__blk1527_dn8 - locals.var_vbscl__blk1541_dn8)))) * assign64620_e99635) - (assign64620_e99632 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign64620_e99635 * assign64620_e99635)), ((((4.0 * ((locals.var_beta_dn9 * assign64620_e99628) + (locals.var_beta * (locals.var_vgp__blk1527_dn9 - locals.var_vbscl__blk1541_dn9)))) * assign64620_e99635) - (assign64620_e99632 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign64620_e99635 * assign64620_e99635)), ((((4.0 * ((locals.var_beta_dn10 * assign64620_e99628) + (locals.var_beta * (locals.var_vgp__blk1527_dn10 - locals.var_vbscl__blk1541_dn10)))) * assign64620_e99635) - (assign64620_e99632 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign64620_e99635 * assign64620_e99635)), ((((4.0 * ((locals.var_beta_dn11 * assign64620_e99628) + (locals.var_beta * (locals.var_vgp__blk1527_dn11 - locals.var_vbscl__blk1541_dn11)))) * assign64620_e99635) - (assign64620_e99632 * ((locals.var_fac1p2_dn11 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn11)))) / (assign64620_e99635 * assign64620_e99635)), ((((4.0 * ((locals.var_beta_dn14 * assign64620_e99628) + (locals.var_beta * (locals.var_vgp__blk1527_dn14 - locals.var_vbscl__blk1541_dn14)))) * assign64620_e99635) - (assign64620_e99632 * ((locals.var_fac1p2_dn14 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn14)))) / (assign64620_e99635 * assign64620_e99635)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign64620_e99639;
        locals.var_tx_dn0 = assign64620_e99639_d_n0;
        locals.var_tx_dn2 = assign64620_e99639_d_n2;
        locals.var_tx_dn4 = assign64620_e99639_d_n4;
        locals.var_tx_dn5 = assign64620_e99639_d_n5;
        locals.var_tx_dn6 = assign64620_e99639_d_n6;
        locals.var_tx_dn7 = assign64620_e99639_d_n7;
        locals.var_tx_dn8 = assign64620_e99639_d_n8;
        locals.var_tx_dn9 = assign64620_e99639_d_n9;
        locals.var_tx_dn10 = assign64620_e99639_d_n10;
        locals.var_tx_dn11 = assign64620_e99639_d_n11;
        locals.var_tx_dn14 = assign64620_e99639_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign64630_e99658, assign64630_e99658_d_n0, assign64630_e99658_d_n2, assign64630_e99658_d_n4, assign64630_e99658_d_n5, assign64630_e99658_d_n6, assign64630_e99658_d_n7, assign64630_e99658_d_n8, assign64630_e99658_d_n9, assign64630_e99658_d_n10, assign64630_e99658_d_n11, assign64630_e99658_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) {
        let assign64630_e99650: f64 = (10.0 * 2.220446049250313e-16);
        let (assign64630_e99656, assign64630_e99656_d_n0, assign64630_e99656_d_n2, assign64630_e99656_d_n4, assign64630_e99656_d_n5, assign64630_e99656_d_n6, assign64630_e99656_d_n7, assign64630_e99656_d_n8, assign64630_e99656_d_n9, assign64630_e99656_d_n10, assign64630_e99656_d_n11, assign64630_e99656_d_n14,) = {
            if (locals.var_tx >= assign64630_e99650) {
                (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
            } else {
                let assign64630_e99655: f64 = (10.0 * 2.220446049250313e-16);
                (assign64630_e99655, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign64630_e99656, assign64630_e99656_d_n0, assign64630_e99656_d_n2, assign64630_e99656_d_n4, assign64630_e99656_d_n5, assign64630_e99656_d_n6, assign64630_e99656_d_n7, assign64630_e99656_d_n8, assign64630_e99656_d_n9, assign64630_e99656_d_n10, assign64630_e99656_d_n11, assign64630_e99656_d_n14,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign64630_e99658;
        locals.var_tx_dn0 = assign64630_e99658_d_n0;
        locals.var_tx_dn2 = assign64630_e99658_d_n2;
        locals.var_tx_dn4 = assign64630_e99658_d_n4;
        locals.var_tx_dn5 = assign64630_e99658_d_n5;
        locals.var_tx_dn6 = assign64630_e99658_d_n6;
        locals.var_tx_dn7 = assign64630_e99658_d_n7;
        locals.var_tx_dn8 = assign64630_e99658_d_n8;
        locals.var_tx_dn9 = assign64630_e99658_d_n9;
        locals.var_tx_dn10 = assign64630_e99658_d_n10;
        locals.var_tx_dn11 = assign64630_e99658_d_n11;
        locals.var_tx_dn14 = assign64630_e99658_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign64640_e99679, assign64640_e99679_d_n0, assign64640_e99679_d_n2, assign64640_e99679_d_n4, assign64640_e99679_d_n5, assign64640_e99679_d_n6, assign64640_e99679_d_n7, assign64640_e99679_d_n8, assign64640_e99679_d_n9, assign64640_e99679_d_n10, assign64640_e99679_d_n11, assign64640_e99679_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) {
        let assign64640_e99669: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign64640_e99671: f64 = (assign64640_e99669 * 0.5);
        let assign64640_e99674: f64 = (locals.var_tx).sqrt();
        let assign64640_e99675: f64 = (1.0 - assign64640_e99674);
        let assign64640_e99676: f64 = (assign64640_e99671 * assign64640_e99675);
        let assign64640_e99677: f64 = (locals.var_vgp__blk1527 + assign64640_e99676);
        (assign64640_e99677, (locals.var_vgp__blk1527_dn0 + (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) * 0.5) * assign64640_e99675) + (assign64640_e99671 * (-(locals.var_tx_dn0 / (2.0 * assign64640_e99674)))))), (locals.var_vgp__blk1527_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) * 0.5) * assign64640_e99675) + (assign64640_e99671 * (-(locals.var_tx_dn2 / (2.0 * assign64640_e99674)))))), (locals.var_vgp__blk1527_dn4 + (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) * 0.5) * assign64640_e99675) + (assign64640_e99671 * (-(locals.var_tx_dn4 / (2.0 * assign64640_e99674)))))), (locals.var_vgp__blk1527_dn5 + (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) * 0.5) * assign64640_e99675) + (assign64640_e99671 * (-(locals.var_tx_dn5 / (2.0 * assign64640_e99674)))))), (locals.var_vgp__blk1527_dn6 + (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) * 0.5) * assign64640_e99675) + (assign64640_e99671 * (-(locals.var_tx_dn6 / (2.0 * assign64640_e99674)))))), (locals.var_vgp__blk1527_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) * 0.5) * assign64640_e99675) + (assign64640_e99671 * (-(locals.var_tx_dn7 / (2.0 * assign64640_e99674)))))), (locals.var_vgp__blk1527_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) * 0.5) * assign64640_e99675) + (assign64640_e99671 * (-(locals.var_tx_dn8 / (2.0 * assign64640_e99674)))))), (locals.var_vgp__blk1527_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) * 0.5) * assign64640_e99675) + (assign64640_e99671 * (-(locals.var_tx_dn9 / (2.0 * assign64640_e99674)))))), (locals.var_vgp__blk1527_dn10 + (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) * 0.5) * assign64640_e99675) + (assign64640_e99671 * (-(locals.var_tx_dn10 / (2.0 * assign64640_e99674)))))), (locals.var_vgp__blk1527_dn11 + (((((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) * 0.5) * assign64640_e99675) + (assign64640_e99671 * (-(locals.var_tx_dn11 / (2.0 * assign64640_e99674)))))), (locals.var_vgp__blk1527_dn14 + (((((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) * 0.5) * assign64640_e99675) + (assign64640_e99671 * (-(locals.var_tx_dn14 / (2.0 * assign64640_e99674)))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign64640_e99679;
        locals.var_ps0_inia_dn0 = assign64640_e99679_d_n0;
        locals.var_ps0_inia_dn2 = assign64640_e99679_d_n2;
        locals.var_ps0_inia_dn4 = assign64640_e99679_d_n4;
        locals.var_ps0_inia_dn5 = assign64640_e99679_d_n5;
        locals.var_ps0_inia_dn6 = assign64640_e99679_d_n6;
        locals.var_ps0_inia_dn7 = assign64640_e99679_d_n7;
        locals.var_ps0_inia_dn8 = assign64640_e99679_d_n8;
        locals.var_ps0_inia_dn9 = assign64640_e99679_d_n9;
        locals.var_ps0_inia_dn10 = assign64640_e99679_d_n10;
        locals.var_ps0_inia_dn11 = assign64640_e99679_d_n11;
        locals.var_ps0_inia_dn14 = assign64640_e99679_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let (assign64650_e99693, assign64650_e99693_d_n0, assign64650_e99693_d_n2, assign64650_e99693_d_n4, assign64650_e99693_d_n5, assign64650_e99693_d_n6, assign64650_e99693_d_n7, assign64650_e99693_d_n8, assign64650_e99693_d_n9, assign64650_e99693_d_n10, assign64650_e99693_d_n11, assign64650_e99693_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) {
        let assign64650_e99690: f64 = (locals.var_ps0_inia - locals.var_vbscl__blk1541);
        let assign64650_e99691: f64 = (locals.var_beta * assign64650_e99690);
        (assign64650_e99691, ((locals.var_beta_dn0 * assign64650_e99690) + (locals.var_beta * (locals.var_ps0_inia_dn0 - locals.var_vbscl__blk1541_dn0))), ((locals.var_beta_dn2 * assign64650_e99690) + (locals.var_beta * (locals.var_ps0_inia_dn2 - locals.var_vbscl__blk1541_dn2))), ((locals.var_beta_dn4 * assign64650_e99690) + (locals.var_beta * (locals.var_ps0_inia_dn4 - locals.var_vbscl__blk1541_dn4))), ((locals.var_beta_dn5 * assign64650_e99690) + (locals.var_beta * (locals.var_ps0_inia_dn5 - locals.var_vbscl__blk1541_dn5))), ((locals.var_beta_dn6 * assign64650_e99690) + (locals.var_beta * (locals.var_ps0_inia_dn6 - locals.var_vbscl__blk1541_dn6))), ((locals.var_beta_dn7 * assign64650_e99690) + (locals.var_beta * (locals.var_ps0_inia_dn7 - locals.var_vbscl__blk1541_dn7))), ((locals.var_beta_dn8 * assign64650_e99690) + (locals.var_beta * (locals.var_ps0_inia_dn8 - locals.var_vbscl__blk1541_dn8))), ((locals.var_beta_dn9 * assign64650_e99690) + (locals.var_beta * (locals.var_ps0_inia_dn9 - locals.var_vbscl__blk1541_dn9))), ((locals.var_beta_dn10 * assign64650_e99690) + (locals.var_beta * (locals.var_ps0_inia_dn10 - locals.var_vbscl__blk1541_dn10))), ((locals.var_beta_dn11 * assign64650_e99690) + (locals.var_beta * (locals.var_ps0_inia_dn11 - locals.var_vbscl__blk1541_dn11))), ((locals.var_beta_dn14 * assign64650_e99690) + (locals.var_beta * (locals.var_ps0_inia_dn14 - locals.var_vbscl__blk1541_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign64650_e99693;
        locals.var_chi_dn0 = assign64650_e99693_d_n0;
        locals.var_chi_dn2 = assign64650_e99693_d_n2;
        locals.var_chi_dn4 = assign64650_e99693_d_n4;
        locals.var_chi_dn5 = assign64650_e99693_d_n5;
        locals.var_chi_dn6 = assign64650_e99693_d_n6;
        locals.var_chi_dn7 = assign64650_e99693_d_n7;
        locals.var_chi_dn8 = assign64650_e99693_d_n8;
        locals.var_chi_dn9 = assign64650_e99693_d_n9;
        locals.var_chi_dn10 = assign64650_e99693_d_n10;
        locals.var_chi_dn11 = assign64650_e99693_d_n11;
        locals.var_chi_dn14 = assign64650_e99693_d_n14;
        locals.var_chi_rv = 0.0;

        let assign64660_e99696: f64 = if locals.var_chi < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1543 = assign64660_e99696;
        locals.var_guard1543_rv = 0.0;

        let (assign64670_e99712, assign64670_e99712_d_n0, assign64670_e99712_d_n2, assign64670_e99712_d_n4, assign64670_e99712_d_n5, assign64670_e99712_d_n6, assign64670_e99712_d_n7, assign64670_e99712_d_n8, assign64670_e99712_d_n9, assign64670_e99712_d_n10, assign64670_e99712_d_n11, assign64670_e99712_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1543 != 0.0)) {
        let assign64670_e99709: f64 = (locals.var_vgp__blk1527 - locals.var_vbscl__blk1541);
        let assign64670_e99710: f64 = (locals.var_beta * assign64670_e99709);
        (assign64670_e99710, ((locals.var_beta_dn0 * assign64670_e99709) + (locals.var_beta * (locals.var_vgp__blk1527_dn0 - locals.var_vbscl__blk1541_dn0))), ((locals.var_beta_dn2 * assign64670_e99709) + (locals.var_beta * (locals.var_vgp__blk1527_dn2 - locals.var_vbscl__blk1541_dn2))), ((locals.var_beta_dn4 * assign64670_e99709) + (locals.var_beta * (locals.var_vgp__blk1527_dn4 - locals.var_vbscl__blk1541_dn4))), ((locals.var_beta_dn5 * assign64670_e99709) + (locals.var_beta * (locals.var_vgp__blk1527_dn5 - locals.var_vbscl__blk1541_dn5))), ((locals.var_beta_dn6 * assign64670_e99709) + (locals.var_beta * (locals.var_vgp__blk1527_dn6 - locals.var_vbscl__blk1541_dn6))), ((locals.var_beta_dn7 * assign64670_e99709) + (locals.var_beta * (locals.var_vgp__blk1527_dn7 - locals.var_vbscl__blk1541_dn7))), ((locals.var_beta_dn8 * assign64670_e99709) + (locals.var_beta * (locals.var_vgp__blk1527_dn8 - locals.var_vbscl__blk1541_dn8))), ((locals.var_beta_dn9 * assign64670_e99709) + (locals.var_beta * (locals.var_vgp__blk1527_dn9 - locals.var_vbscl__blk1541_dn9))), ((locals.var_beta_dn10 * assign64670_e99709) + (locals.var_beta * (locals.var_vgp__blk1527_dn10 - locals.var_vbscl__blk1541_dn10))), ((locals.var_beta_dn11 * assign64670_e99709) + (locals.var_beta * (locals.var_vgp__blk1527_dn11 - locals.var_vbscl__blk1541_dn11))), ((locals.var_beta_dn14 * assign64670_e99709) + (locals.var_beta * (locals.var_vgp__blk1527_dn14 - locals.var_vbscl__blk1541_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign64670_e99712;
        locals.var_ty_dn0 = assign64670_e99712_d_n0;
        locals.var_ty_dn2 = assign64670_e99712_d_n2;
        locals.var_ty_dn4 = assign64670_e99712_d_n4;
        locals.var_ty_dn5 = assign64670_e99712_d_n5;
        locals.var_ty_dn6 = assign64670_e99712_d_n6;
        locals.var_ty_dn7 = assign64670_e99712_d_n7;
        locals.var_ty_dn8 = assign64670_e99712_d_n8;
        locals.var_ty_dn9 = assign64670_e99712_d_n9;
        locals.var_ty_dn10 = assign64670_e99712_d_n10;
        locals.var_ty_dn11 = assign64670_e99712_d_n11;
        locals.var_ty_dn14 = assign64670_e99712_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign64680_e99732, assign64680_e99732_d_n0, assign64680_e99732_d_n2, assign64680_e99732_d_n4, assign64680_e99732_d_n5, assign64680_e99732_d_n6, assign64680_e99732_d_n7, assign64680_e99732_d_n8, assign64680_e99732_d_n9, assign64680_e99732_d_n10, assign64680_e99732_d_n11, assign64680_e99732_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1543 != 0.0)) {
        let assign64680_e99725: f64 = (1.414213562373095 / 108.0);
        let assign64680_e99727: f64 = (assign64680_e99725 * locals.var_beta);
        let assign64680_e99729: f64 = (assign64680_e99727 * locals.var_fac1);
        let assign64680_e99730: f64 = (1.0 / assign64680_e99729);
        (assign64680_e99730, (-((((assign64680_e99725 * locals.var_beta_dn0) * locals.var_fac1) + (assign64680_e99727 * locals.var_fac1_dn0)) / (assign64680_e99729 * assign64680_e99729))), (-((((assign64680_e99725 * locals.var_beta_dn2) * locals.var_fac1) + (assign64680_e99727 * locals.var_fac1_dn2)) / (assign64680_e99729 * assign64680_e99729))), (-((((assign64680_e99725 * locals.var_beta_dn4) * locals.var_fac1) + (assign64680_e99727 * locals.var_fac1_dn4)) / (assign64680_e99729 * assign64680_e99729))), (-((((assign64680_e99725 * locals.var_beta_dn5) * locals.var_fac1) + (assign64680_e99727 * locals.var_fac1_dn5)) / (assign64680_e99729 * assign64680_e99729))), (-((((assign64680_e99725 * locals.var_beta_dn6) * locals.var_fac1) + (assign64680_e99727 * locals.var_fac1_dn6)) / (assign64680_e99729 * assign64680_e99729))), (-((((assign64680_e99725 * locals.var_beta_dn7) * locals.var_fac1) + (assign64680_e99727 * locals.var_fac1_dn7)) / (assign64680_e99729 * assign64680_e99729))), (-((((assign64680_e99725 * locals.var_beta_dn8) * locals.var_fac1) + (assign64680_e99727 * locals.var_fac1_dn8)) / (assign64680_e99729 * assign64680_e99729))), (-((((assign64680_e99725 * locals.var_beta_dn9) * locals.var_fac1) + (assign64680_e99727 * locals.var_fac1_dn9)) / (assign64680_e99729 * assign64680_e99729))), (-((((assign64680_e99725 * locals.var_beta_dn10) * locals.var_fac1) + (assign64680_e99727 * locals.var_fac1_dn10)) / (assign64680_e99729 * assign64680_e99729))), (-((((assign64680_e99725 * locals.var_beta_dn11) * locals.var_fac1) + (assign64680_e99727 * locals.var_fac1_dn11)) / (assign64680_e99729 * assign64680_e99729))), (-((((assign64680_e99725 * locals.var_beta_dn14) * locals.var_fac1) + (assign64680_e99727 * locals.var_fac1_dn14)) / (assign64680_e99729 * assign64680_e99729))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign64680_e99732;
        locals.var_t1_dn0 = assign64680_e99732_d_n0;
        locals.var_t1_dn2 = assign64680_e99732_d_n2;
        locals.var_t1_dn4 = assign64680_e99732_d_n4;
        locals.var_t1_dn5 = assign64680_e99732_d_n5;
        locals.var_t1_dn6 = assign64680_e99732_d_n6;
        locals.var_t1_dn7 = assign64680_e99732_d_n7;
        locals.var_t1_dn8 = assign64680_e99732_d_n8;
        locals.var_t1_dn9 = assign64680_e99732_d_n9;
        locals.var_t1_dn10 = assign64680_e99732_d_n10;
        locals.var_t1_dn11 = assign64680_e99732_d_n11;
        locals.var_t1_dn14 = assign64680_e99732_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign64690_e99748, assign64690_e99748_d_n0, assign64690_e99748_d_n2, assign64690_e99748_d_n4, assign64690_e99748_d_n5, assign64690_e99748_d_n6, assign64690_e99748_d_n7, assign64690_e99748_d_n8, assign64690_e99748_d_n9, assign64690_e99748_d_n10, assign64690_e99748_d_n11, assign64690_e99748_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1543 != 0.0)) {
        let assign64690_e99745: f64 = (3.0 * locals.var_t1);
        let assign64690_e99746: f64 = (81.0 + assign64690_e99745);
        (assign64690_e99746, (3.0 * locals.var_t1_dn0), (3.0 * locals.var_t1_dn2), (3.0 * locals.var_t1_dn4), (3.0 * locals.var_t1_dn5), (3.0 * locals.var_t1_dn6), (3.0 * locals.var_t1_dn7), (3.0 * locals.var_t1_dn8), (3.0 * locals.var_t1_dn9), (3.0 * locals.var_t1_dn10), (3.0 * locals.var_t1_dn11), (3.0 * locals.var_t1_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign64690_e99748;
        locals.var_t2_dn0 = assign64690_e99748_d_n0;
        locals.var_t2_dn2 = assign64690_e99748_d_n2;
        locals.var_t2_dn4 = assign64690_e99748_d_n4;
        locals.var_t2_dn5 = assign64690_e99748_d_n5;
        locals.var_t2_dn6 = assign64690_e99748_d_n6;
        locals.var_t2_dn7 = assign64690_e99748_d_n7;
        locals.var_t2_dn8 = assign64690_e99748_d_n8;
        locals.var_t2_dn9 = assign64690_e99748_d_n9;
        locals.var_t2_dn10 = assign64690_e99748_d_n10;
        locals.var_t2_dn11 = assign64690_e99748_d_n11;
        locals.var_t2_dn14 = assign64690_e99748_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign64700_e99771, assign64700_e99771_d_n0, assign64700_e99771_d_n2, assign64700_e99771_d_n4, assign64700_e99771_d_n5, assign64700_e99771_d_n6, assign64700_e99771_d_n7, assign64700_e99771_d_n8, assign64700_e99771_d_n9, assign64700_e99771_d_n10, assign64700_e99771_d_n11, assign64700_e99771_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1543 != 0.0)) {
        let assign64700_e99759: f64 = (-2916.0);
        let assign64700_e99762: f64 = (81.0 * locals.var_t1);
        let assign64700_e99763: f64 = (assign64700_e99759 - assign64700_e99762);
        let assign64700_e99766: f64 = (27.0 * locals.var_t1);
        let assign64700_e99768: f64 = (assign64700_e99766 * locals.var_ty);
        let assign64700_e99769: f64 = (assign64700_e99763 + assign64700_e99768);
        (assign64700_e99769, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign64700_e99766 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign64700_e99766 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn4)) + (((27.0 * locals.var_t1_dn4) * locals.var_ty) + (assign64700_e99766 * locals.var_ty_dn4))), ((-(81.0 * locals.var_t1_dn5)) + (((27.0 * locals.var_t1_dn5) * locals.var_ty) + (assign64700_e99766 * locals.var_ty_dn5))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign64700_e99766 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign64700_e99766 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn8)) + (((27.0 * locals.var_t1_dn8) * locals.var_ty) + (assign64700_e99766 * locals.var_ty_dn8))), ((-(81.0 * locals.var_t1_dn9)) + (((27.0 * locals.var_t1_dn9) * locals.var_ty) + (assign64700_e99766 * locals.var_ty_dn9))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign64700_e99766 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn11)) + (((27.0 * locals.var_t1_dn11) * locals.var_ty) + (assign64700_e99766 * locals.var_ty_dn11))), ((-(81.0 * locals.var_t1_dn14)) + (((27.0 * locals.var_t1_dn14) * locals.var_ty) + (assign64700_e99766 * locals.var_ty_dn14))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign64700_e99771;
        locals.var_t3_dn0 = assign64700_e99771_d_n0;
        locals.var_t3_dn2 = assign64700_e99771_d_n2;
        locals.var_t3_dn4 = assign64700_e99771_d_n4;
        locals.var_t3_dn5 = assign64700_e99771_d_n5;
        locals.var_t3_dn6 = assign64700_e99771_d_n6;
        locals.var_t3_dn7 = assign64700_e99771_d_n7;
        locals.var_t3_dn8 = assign64700_e99771_d_n8;
        locals.var_t3_dn9 = assign64700_e99771_d_n9;
        locals.var_t3_dn10 = assign64700_e99771_d_n10;
        locals.var_t3_dn11 = assign64700_e99771_d_n11;
        locals.var_t3_dn14 = assign64700_e99771_d_n14;
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_242(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign64710_e99795, assign64710_e99795_d_n0, assign64710_e99795_d_n2, assign64710_e99795_d_n4, assign64710_e99795_d_n5, assign64710_e99795_d_n6, assign64710_e99795_d_n7, assign64710_e99795_d_n8, assign64710_e99795_d_n9, assign64710_e99795_d_n10, assign64710_e99795_d_n11, assign64710_e99795_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1543 != 0.0)) {
        let assign64710_e99785: f64 = (54.0 + locals.var_t1);
        let assign64710_e99786: f64 = (81.0 * assign64710_e99785);
        let assign64710_e99787: f64 = (1458.0 - assign64710_e99786);
        let assign64710_e99790: f64 = (27.0 * locals.var_t1);
        let assign64710_e99792: f64 = (assign64710_e99790 * locals.var_ty);
        let assign64710_e99793: f64 = (assign64710_e99787 + assign64710_e99792);
        (assign64710_e99793, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign64710_e99790 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign64710_e99790 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn4)) + (((27.0 * locals.var_t1_dn4) * locals.var_ty) + (assign64710_e99790 * locals.var_ty_dn4))), ((-(81.0 * locals.var_t1_dn5)) + (((27.0 * locals.var_t1_dn5) * locals.var_ty) + (assign64710_e99790 * locals.var_ty_dn5))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign64710_e99790 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign64710_e99790 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn8)) + (((27.0 * locals.var_t1_dn8) * locals.var_ty) + (assign64710_e99790 * locals.var_ty_dn8))), ((-(81.0 * locals.var_t1_dn9)) + (((27.0 * locals.var_t1_dn9) * locals.var_ty) + (assign64710_e99790 * locals.var_ty_dn9))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign64710_e99790 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn11)) + (((27.0 * locals.var_t1_dn11) * locals.var_ty) + (assign64710_e99790 * locals.var_ty_dn11))), ((-(81.0 * locals.var_t1_dn14)) + (((27.0 * locals.var_t1_dn14) * locals.var_ty) + (assign64710_e99790 * locals.var_ty_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign64710_e99795;
        locals.var_t4_dn0 = assign64710_e99795_d_n0;
        locals.var_t4_dn2 = assign64710_e99795_d_n2;
        locals.var_t4_dn4 = assign64710_e99795_d_n4;
        locals.var_t4_dn5 = assign64710_e99795_d_n5;
        locals.var_t4_dn6 = assign64710_e99795_d_n6;
        locals.var_t4_dn7 = assign64710_e99795_d_n7;
        locals.var_t4_dn8 = assign64710_e99795_d_n8;
        locals.var_t4_dn9 = assign64710_e99795_d_n9;
        locals.var_t4_dn10 = assign64710_e99795_d_n10;
        locals.var_t4_dn11 = assign64710_e99795_d_n11;
        locals.var_t4_dn14 = assign64710_e99795_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign64720_e99809, assign64720_e99809_d_n0, assign64720_e99809_d_n2, assign64720_e99809_d_n4, assign64720_e99809_d_n5, assign64720_e99809_d_n6, assign64720_e99809_d_n7, assign64720_e99809_d_n8, assign64720_e99809_d_n9, assign64720_e99809_d_n10, assign64720_e99809_d_n11, assign64720_e99809_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1543 != 0.0)) {
        let assign64720_e99807: f64 = (locals.var_t4 * locals.var_t4);
        (assign64720_e99807, ((locals.var_t4_dn0 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn0)), ((locals.var_t4_dn2 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn2)), ((locals.var_t4_dn4 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn4)), ((locals.var_t4_dn5 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn5)), ((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)), ((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)), ((locals.var_t4_dn8 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn8)), ((locals.var_t4_dn9 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn9)), ((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)), ((locals.var_t4_dn11 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn11)), ((locals.var_t4_dn14 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign64720_e99809;
        locals.var_t4_dn0 = assign64720_e99809_d_n0;
        locals.var_t4_dn2 = assign64720_e99809_d_n2;
        locals.var_t4_dn4 = assign64720_e99809_d_n4;
        locals.var_t4_dn5 = assign64720_e99809_d_n5;
        locals.var_t4_dn6 = assign64720_e99809_d_n6;
        locals.var_t4_dn7 = assign64720_e99809_d_n7;
        locals.var_t4_dn8 = assign64720_e99809_d_n8;
        locals.var_t4_dn9 = assign64720_e99809_d_n9;
        locals.var_t4_dn10 = assign64720_e99809_d_n10;
        locals.var_t4_dn11 = assign64720_e99809_d_n11;
        locals.var_t4_dn14 = assign64720_e99809_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign64730_e99850, assign64730_e99850_d_n0, assign64730_e99850_d_n2, assign64730_e99850_d_n4, assign64730_e99850_d_n5, assign64730_e99850_d_n6, assign64730_e99850_d_n7, assign64730_e99850_d_n8, assign64730_e99850_d_n9, assign64730_e99850_d_n10, assign64730_e99850_d_n11, assign64730_e99850_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1543 != 0.0)) {
        let assign64730_e99822: f64 = (4.0 * locals.var_t2);
        let assign64730_e99824: f64 = (assign64730_e99822 * locals.var_t2);
        let assign64730_e99826: f64 = (assign64730_e99824 * locals.var_t2);
        let assign64730_e99828: f64 = (assign64730_e99826 + locals.var_t4);
        let assign64730_e99829: f64 = (assign64730_e99828).sqrt();
        let assign64730_e99830: f64 = (locals.var_t3 + assign64730_e99829);
        let (assign64730_e99848, assign64730_e99848_d_n0, assign64730_e99848_d_n2, assign64730_e99848_d_n4, assign64730_e99848_d_n5, assign64730_e99848_d_n6, assign64730_e99848_d_n7, assign64730_e99848_d_n8, assign64730_e99848_d_n9, assign64730_e99848_d_n10, assign64730_e99848_d_n11, assign64730_e99848_d_n14,) = {
            if (assign64730_e99830 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign64730_e99837: f64 = (4.0 * locals.var_t2);
                let assign64730_e99839: f64 = (assign64730_e99837 * locals.var_t2);
                let assign64730_e99841: f64 = (assign64730_e99839 * locals.var_t2);
                let assign64730_e99843: f64 = (assign64730_e99841 + locals.var_t4);
                let assign64730_e99844: f64 = (assign64730_e99843).sqrt();
                let assign64730_e99845: f64 = (locals.var_t3 + assign64730_e99844);
                let assign64730_e99847: f64 = (assign64730_e99845).powf(0.3333333333333333);
                (assign64730_e99847, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64730_e99845).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign64730_e99837 * locals.var_t2_dn0)) * locals.var_t2) + (assign64730_e99839 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign64730_e99844))))) } } else { (assign64730_e99847 * (0.3333333333333333 * ((locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign64730_e99837 * locals.var_t2_dn0)) * locals.var_t2) + (assign64730_e99839 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign64730_e99844))) / assign64730_e99845))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64730_e99845).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign64730_e99837 * locals.var_t2_dn2)) * locals.var_t2) + (assign64730_e99839 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign64730_e99844))))) } } else { (assign64730_e99847 * (0.3333333333333333 * ((locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign64730_e99837 * locals.var_t2_dn2)) * locals.var_t2) + (assign64730_e99839 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign64730_e99844))) / assign64730_e99845))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64730_e99845).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn4 + (((((((4.0 * locals.var_t2_dn4) * locals.var_t2) + (assign64730_e99837 * locals.var_t2_dn4)) * locals.var_t2) + (assign64730_e99839 * locals.var_t2_dn4)) + locals.var_t4_dn4) / (2.0 * assign64730_e99844))))) } } else { (assign64730_e99847 * (0.3333333333333333 * ((locals.var_t3_dn4 + (((((((4.0 * locals.var_t2_dn4) * locals.var_t2) + (assign64730_e99837 * locals.var_t2_dn4)) * locals.var_t2) + (assign64730_e99839 * locals.var_t2_dn4)) + locals.var_t4_dn4) / (2.0 * assign64730_e99844))) / assign64730_e99845))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64730_e99845).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn5 + (((((((4.0 * locals.var_t2_dn5) * locals.var_t2) + (assign64730_e99837 * locals.var_t2_dn5)) * locals.var_t2) + (assign64730_e99839 * locals.var_t2_dn5)) + locals.var_t4_dn5) / (2.0 * assign64730_e99844))))) } } else { (assign64730_e99847 * (0.3333333333333333 * ((locals.var_t3_dn5 + (((((((4.0 * locals.var_t2_dn5) * locals.var_t2) + (assign64730_e99837 * locals.var_t2_dn5)) * locals.var_t2) + (assign64730_e99839 * locals.var_t2_dn5)) + locals.var_t4_dn5) / (2.0 * assign64730_e99844))) / assign64730_e99845))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64730_e99845).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign64730_e99837 * locals.var_t2_dn6)) * locals.var_t2) + (assign64730_e99839 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign64730_e99844))))) } } else { (assign64730_e99847 * (0.3333333333333333 * ((locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign64730_e99837 * locals.var_t2_dn6)) * locals.var_t2) + (assign64730_e99839 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign64730_e99844))) / assign64730_e99845))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64730_e99845).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign64730_e99837 * locals.var_t2_dn7)) * locals.var_t2) + (assign64730_e99839 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign64730_e99844))))) } } else { (assign64730_e99847 * (0.3333333333333333 * ((locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign64730_e99837 * locals.var_t2_dn7)) * locals.var_t2) + (assign64730_e99839 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign64730_e99844))) / assign64730_e99845))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64730_e99845).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn8 + (((((((4.0 * locals.var_t2_dn8) * locals.var_t2) + (assign64730_e99837 * locals.var_t2_dn8)) * locals.var_t2) + (assign64730_e99839 * locals.var_t2_dn8)) + locals.var_t4_dn8) / (2.0 * assign64730_e99844))))) } } else { (assign64730_e99847 * (0.3333333333333333 * ((locals.var_t3_dn8 + (((((((4.0 * locals.var_t2_dn8) * locals.var_t2) + (assign64730_e99837 * locals.var_t2_dn8)) * locals.var_t2) + (assign64730_e99839 * locals.var_t2_dn8)) + locals.var_t4_dn8) / (2.0 * assign64730_e99844))) / assign64730_e99845))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64730_e99845).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn9 + (((((((4.0 * locals.var_t2_dn9) * locals.var_t2) + (assign64730_e99837 * locals.var_t2_dn9)) * locals.var_t2) + (assign64730_e99839 * locals.var_t2_dn9)) + locals.var_t4_dn9) / (2.0 * assign64730_e99844))))) } } else { (assign64730_e99847 * (0.3333333333333333 * ((locals.var_t3_dn9 + (((((((4.0 * locals.var_t2_dn9) * locals.var_t2) + (assign64730_e99837 * locals.var_t2_dn9)) * locals.var_t2) + (assign64730_e99839 * locals.var_t2_dn9)) + locals.var_t4_dn9) / (2.0 * assign64730_e99844))) / assign64730_e99845))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64730_e99845).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign64730_e99837 * locals.var_t2_dn10)) * locals.var_t2) + (assign64730_e99839 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign64730_e99844))))) } } else { (assign64730_e99847 * (0.3333333333333333 * ((locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign64730_e99837 * locals.var_t2_dn10)) * locals.var_t2) + (assign64730_e99839 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign64730_e99844))) / assign64730_e99845))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64730_e99845).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn11 + (((((((4.0 * locals.var_t2_dn11) * locals.var_t2) + (assign64730_e99837 * locals.var_t2_dn11)) * locals.var_t2) + (assign64730_e99839 * locals.var_t2_dn11)) + locals.var_t4_dn11) / (2.0 * assign64730_e99844))))) } } else { (assign64730_e99847 * (0.3333333333333333 * ((locals.var_t3_dn11 + (((((((4.0 * locals.var_t2_dn11) * locals.var_t2) + (assign64730_e99837 * locals.var_t2_dn11)) * locals.var_t2) + (assign64730_e99839 * locals.var_t2_dn11)) + locals.var_t4_dn11) / (2.0 * assign64730_e99844))) / assign64730_e99845))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign64730_e99845).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn14 + (((((((4.0 * locals.var_t2_dn14) * locals.var_t2) + (assign64730_e99837 * locals.var_t2_dn14)) * locals.var_t2) + (assign64730_e99839 * locals.var_t2_dn14)) + locals.var_t4_dn14) / (2.0 * assign64730_e99844))))) } } else { (assign64730_e99847 * (0.3333333333333333 * ((locals.var_t3_dn14 + (((((((4.0 * locals.var_t2_dn14) * locals.var_t2) + (assign64730_e99837 * locals.var_t2_dn14)) * locals.var_t2) + (assign64730_e99839 * locals.var_t2_dn14)) + locals.var_t4_dn14) / (2.0 * assign64730_e99844))) / assign64730_e99845))) },)
            }
        };
        (assign64730_e99848, assign64730_e99848_d_n0, assign64730_e99848_d_n2, assign64730_e99848_d_n4, assign64730_e99848_d_n5, assign64730_e99848_d_n6, assign64730_e99848_d_n7, assign64730_e99848_d_n8, assign64730_e99848_d_n9, assign64730_e99848_d_n10, assign64730_e99848_d_n11, assign64730_e99848_d_n14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign64730_e99850;
        locals.var_t5_dn0 = assign64730_e99850_d_n0;
        locals.var_t5_dn2 = assign64730_e99850_d_n2;
        locals.var_t5_dn4 = assign64730_e99850_d_n4;
        locals.var_t5_dn5 = assign64730_e99850_d_n5;
        locals.var_t5_dn6 = assign64730_e99850_d_n6;
        locals.var_t5_dn7 = assign64730_e99850_d_n7;
        locals.var_t5_dn8 = assign64730_e99850_d_n8;
        locals.var_t5_dn9 = assign64730_e99850_d_n9;
        locals.var_t5_dn10 = assign64730_e99850_d_n10;
        locals.var_t5_dn11 = assign64730_e99850_d_n11;
        locals.var_t5_dn14 = assign64730_e99850_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign64740_e99878, assign64740_e99878_d_n0, assign64740_e99878_d_n2, assign64740_e99878_d_n4, assign64740_e99878_d_n5, assign64740_e99878_d_n6, assign64740_e99878_d_n7, assign64740_e99878_d_n8, assign64740_e99878_d_n9, assign64740_e99878_d_n10, assign64740_e99878_d_n11, assign64740_e99878_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1543 != 0.0)) {
        let assign64740_e99863: f64 = (1.259921049894873 * locals.var_t2);
        let assign64740_e99866: f64 = (3.0 * locals.var_t5);
        let assign64740_e99867: f64 = (assign64740_e99863 / assign64740_e99866);
        let assign64740_e99868: f64 = (3.0 - assign64740_e99867);
        let assign64740_e99872: f64 = (3.0 * 1.259921049894873);
        let assign64740_e99873: f64 = (1.0 / assign64740_e99872);
        let assign64740_e99875: f64 = (assign64740_e99873 * locals.var_t5);
        let assign64740_e99876: f64 = (assign64740_e99868 + assign64740_e99875);
        (assign64740_e99876, ((-((((1.259921049894873 * locals.var_t2_dn0) * assign64740_e99866) - (assign64740_e99863 * (3.0 * locals.var_t5_dn0))) / (assign64740_e99866 * assign64740_e99866))) + (assign64740_e99873 * locals.var_t5_dn0)), ((-((((1.259921049894873 * locals.var_t2_dn2) * assign64740_e99866) - (assign64740_e99863 * (3.0 * locals.var_t5_dn2))) / (assign64740_e99866 * assign64740_e99866))) + (assign64740_e99873 * locals.var_t5_dn2)), ((-((((1.259921049894873 * locals.var_t2_dn4) * assign64740_e99866) - (assign64740_e99863 * (3.0 * locals.var_t5_dn4))) / (assign64740_e99866 * assign64740_e99866))) + (assign64740_e99873 * locals.var_t5_dn4)), ((-((((1.259921049894873 * locals.var_t2_dn5) * assign64740_e99866) - (assign64740_e99863 * (3.0 * locals.var_t5_dn5))) / (assign64740_e99866 * assign64740_e99866))) + (assign64740_e99873 * locals.var_t5_dn5)), ((-((((1.259921049894873 * locals.var_t2_dn6) * assign64740_e99866) - (assign64740_e99863 * (3.0 * locals.var_t5_dn6))) / (assign64740_e99866 * assign64740_e99866))) + (assign64740_e99873 * locals.var_t5_dn6)), ((-((((1.259921049894873 * locals.var_t2_dn7) * assign64740_e99866) - (assign64740_e99863 * (3.0 * locals.var_t5_dn7))) / (assign64740_e99866 * assign64740_e99866))) + (assign64740_e99873 * locals.var_t5_dn7)), ((-((((1.259921049894873 * locals.var_t2_dn8) * assign64740_e99866) - (assign64740_e99863 * (3.0 * locals.var_t5_dn8))) / (assign64740_e99866 * assign64740_e99866))) + (assign64740_e99873 * locals.var_t5_dn8)), ((-((((1.259921049894873 * locals.var_t2_dn9) * assign64740_e99866) - (assign64740_e99863 * (3.0 * locals.var_t5_dn9))) / (assign64740_e99866 * assign64740_e99866))) + (assign64740_e99873 * locals.var_t5_dn9)), ((-((((1.259921049894873 * locals.var_t2_dn10) * assign64740_e99866) - (assign64740_e99863 * (3.0 * locals.var_t5_dn10))) / (assign64740_e99866 * assign64740_e99866))) + (assign64740_e99873 * locals.var_t5_dn10)), ((-((((1.259921049894873 * locals.var_t2_dn11) * assign64740_e99866) - (assign64740_e99863 * (3.0 * locals.var_t5_dn11))) / (assign64740_e99866 * assign64740_e99866))) + (assign64740_e99873 * locals.var_t5_dn11)), ((-((((1.259921049894873 * locals.var_t2_dn14) * assign64740_e99866) - (assign64740_e99863 * (3.0 * locals.var_t5_dn14))) / (assign64740_e99866 * assign64740_e99866))) + (assign64740_e99873 * locals.var_t5_dn14)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign64740_e99878;
        locals.var_tx_dn0 = assign64740_e99878_d_n0;
        locals.var_tx_dn2 = assign64740_e99878_d_n2;
        locals.var_tx_dn4 = assign64740_e99878_d_n4;
        locals.var_tx_dn5 = assign64740_e99878_d_n5;
        locals.var_tx_dn6 = assign64740_e99878_d_n6;
        locals.var_tx_dn7 = assign64740_e99878_d_n7;
        locals.var_tx_dn8 = assign64740_e99878_d_n8;
        locals.var_tx_dn9 = assign64740_e99878_d_n9;
        locals.var_tx_dn10 = assign64740_e99878_d_n10;
        locals.var_tx_dn11 = assign64740_e99878_d_n11;
        locals.var_tx_dn14 = assign64740_e99878_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign64750_e99894, assign64750_e99894_d_n0, assign64750_e99894_d_n2, assign64750_e99894_d_n4, assign64750_e99894_d_n5, assign64750_e99894_d_n6, assign64750_e99894_d_n7, assign64750_e99894_d_n8, assign64750_e99894_d_n9, assign64750_e99894_d_n10, assign64750_e99894_d_n11, assign64750_e99894_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1543 != 0.0)) {
        let assign64750_e99890: f64 = (locals.var_tx * locals.var_beta_inv);
        let assign64750_e99892: f64 = (assign64750_e99890 + locals.var_vbscl__blk1541);
        (assign64750_e99892, (((locals.var_tx_dn0 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn0)) + locals.var_vbscl__blk1541_dn0), (((locals.var_tx_dn2 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn2)) + locals.var_vbscl__blk1541_dn2), (((locals.var_tx_dn4 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn4)) + locals.var_vbscl__blk1541_dn4), (((locals.var_tx_dn5 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn5)) + locals.var_vbscl__blk1541_dn5), (((locals.var_tx_dn6 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn6)) + locals.var_vbscl__blk1541_dn6), (((locals.var_tx_dn7 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn7)) + locals.var_vbscl__blk1541_dn7), (((locals.var_tx_dn8 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn8)) + locals.var_vbscl__blk1541_dn8), (((locals.var_tx_dn9 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn9)) + locals.var_vbscl__blk1541_dn9), (((locals.var_tx_dn10 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn10)) + locals.var_vbscl__blk1541_dn10), (((locals.var_tx_dn11 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn11)) + locals.var_vbscl__blk1541_dn11), (((locals.var_tx_dn14 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn14)) + locals.var_vbscl__blk1541_dn14),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign64750_e99894;
        locals.var_ps0_inia_dn0 = assign64750_e99894_d_n0;
        locals.var_ps0_inia_dn2 = assign64750_e99894_d_n2;
        locals.var_ps0_inia_dn4 = assign64750_e99894_d_n4;
        locals.var_ps0_inia_dn5 = assign64750_e99894_d_n5;
        locals.var_ps0_inia_dn6 = assign64750_e99894_d_n6;
        locals.var_ps0_inia_dn7 = assign64750_e99894_d_n7;
        locals.var_ps0_inia_dn8 = assign64750_e99894_d_n8;
        locals.var_ps0_inia_dn9 = assign64750_e99894_d_n9;
        locals.var_ps0_inia_dn10 = assign64750_e99894_d_n10;
        locals.var_ps0_inia_dn11 = assign64750_e99894_d_n11;
        locals.var_ps0_inia_dn14 = assign64750_e99894_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let (assign64760_e99906, assign64760_e99906_d_n0, assign64760_e99906_d_n2, assign64760_e99906_d_n4, assign64760_e99906_d_n5, assign64760_e99906_d_n6, assign64760_e99906_d_n7, assign64760_e99906_d_n8, assign64760_e99906_d_n9, assign64760_e99906_d_n10, assign64760_e99906_d_n11, assign64760_e99906_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1543 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn14,)
    }
};
        locals.var_ps0_ini = assign64760_e99906;
        locals.var_ps0_ini_dn0 = assign64760_e99906_d_n0;
        locals.var_ps0_ini_dn2 = assign64760_e99906_d_n2;
        locals.var_ps0_ini_dn4 = assign64760_e99906_d_n4;
        locals.var_ps0_ini_dn5 = assign64760_e99906_d_n5;
        locals.var_ps0_ini_dn6 = assign64760_e99906_d_n6;
        locals.var_ps0_ini_dn7 = assign64760_e99906_d_n7;
        locals.var_ps0_ini_dn8 = assign64760_e99906_d_n8;
        locals.var_ps0_ini_dn9 = assign64760_e99906_d_n9;
        locals.var_ps0_ini_dn10 = assign64760_e99906_d_n10;
        locals.var_ps0_ini_dn11 = assign64760_e99906_d_n11;
        locals.var_ps0_ini_dn14 = assign64760_e99906_d_n14;
        locals.var_ps0_ini_rv = 0.0;

        let assign64770_e99909: f64 = if locals.var_vgs <= locals.var_vth__blk1542 { 1.0 } else { 0.0 };
        locals.var_guard1544 = assign64770_e99909;
        locals.var_guard1544_rv = 0.0;

        let (assign64780_e99924, assign64780_e99924_d_n0, assign64780_e99924_d_n2, assign64780_e99924_d_n4, assign64780_e99924_d_n5, assign64780_e99924_d_n6, assign64780_e99924_d_n7, assign64780_e99924_d_n8, assign64780_e99924_d_n9, assign64780_e99924_d_n10, assign64780_e99924_d_n11, assign64780_e99924_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1543 == 0.0)) && (locals.var_guard1544 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn14,)
    }
};
        locals.var_ps0_ini = assign64780_e99924;
        locals.var_ps0_ini_dn0 = assign64780_e99924_d_n0;
        locals.var_ps0_ini_dn2 = assign64780_e99924_d_n2;
        locals.var_ps0_ini_dn4 = assign64780_e99924_d_n4;
        locals.var_ps0_ini_dn5 = assign64780_e99924_d_n5;
        locals.var_ps0_ini_dn6 = assign64780_e99924_d_n6;
        locals.var_ps0_ini_dn7 = assign64780_e99924_d_n7;
        locals.var_ps0_ini_dn8 = assign64780_e99924_d_n8;
        locals.var_ps0_ini_dn9 = assign64780_e99924_d_n9;
        locals.var_ps0_ini_dn10 = assign64780_e99924_d_n10;
        locals.var_ps0_ini_dn11 = assign64780_e99924_d_n11;
        locals.var_ps0_ini_dn14 = assign64780_e99924_d_n14;
        locals.var_ps0_ini_rv = 0.0;

        let (assign64790_e99944, assign64790_e99944_d_n0, assign64790_e99944_d_n2, assign64790_e99944_d_n4, assign64790_e99944_d_n5, assign64790_e99944_d_n6, assign64790_e99944_d_n7, assign64790_e99944_d_n8, assign64790_e99944_d_n9, assign64790_e99944_d_n10, assign64790_e99944_d_n11, assign64790_e99944_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1543 == 0.0)) && (locals.var_guard1544 == 0.0)) {
        let assign64790_e99940: f64 = (1.0 / locals.var_cnst1);
        let assign64790_e99942: f64 = (assign64790_e99940 / locals.var_cnstcoxi);
        (assign64790_e99942, ((((-(locals.var_cnst1_dn0 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64790_e99940 * locals.var_cnstcoxi_dn0)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn2 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64790_e99940 * locals.var_cnstcoxi_dn2)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn4 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64790_e99940 * locals.var_cnstcoxi_dn4)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn5 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64790_e99940 * locals.var_cnstcoxi_dn5)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn6 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64790_e99940 * locals.var_cnstcoxi_dn6)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn7 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64790_e99940 * locals.var_cnstcoxi_dn7)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn8 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64790_e99940 * locals.var_cnstcoxi_dn8)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn9 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64790_e99940 * locals.var_cnstcoxi_dn9)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn10 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64790_e99940 * locals.var_cnstcoxi_dn10)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn11 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64790_e99940 * locals.var_cnstcoxi_dn11)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)), ((((-(locals.var_cnst1_dn14 / (locals.var_cnst1 * locals.var_cnst1))) * locals.var_cnstcoxi) - (assign64790_e99940 * locals.var_cnstcoxi_dn14)) / (locals.var_cnstcoxi * locals.var_cnstcoxi)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign64790_e99944;
        locals.var_t1_dn0 = assign64790_e99944_d_n0;
        locals.var_t1_dn2 = assign64790_e99944_d_n2;
        locals.var_t1_dn4 = assign64790_e99944_d_n4;
        locals.var_t1_dn5 = assign64790_e99944_d_n5;
        locals.var_t1_dn6 = assign64790_e99944_d_n6;
        locals.var_t1_dn7 = assign64790_e99944_d_n7;
        locals.var_t1_dn8 = assign64790_e99944_d_n8;
        locals.var_t1_dn9 = assign64790_e99944_d_n9;
        locals.var_t1_dn10 = assign64790_e99944_d_n10;
        locals.var_t1_dn11 = assign64790_e99944_d_n11;
        locals.var_t1_dn14 = assign64790_e99944_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign64800_e99964, assign64800_e99964_d_n0, assign64800_e99964_d_n2, assign64800_e99964_d_n4, assign64800_e99964_d_n5, assign64800_e99964_d_n6, assign64800_e99964_d_n7, assign64800_e99964_d_n8, assign64800_e99964_d_n9, assign64800_e99964_d_n10, assign64800_e99964_d_n11, assign64800_e99964_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1543 == 0.0)) && (locals.var_guard1544 == 0.0)) {
        let assign64800_e99960: f64 = (locals.var_t1 * locals.var_vgp__blk1527);
        let assign64800_e99962: f64 = (assign64800_e99960 * locals.var_vgp__blk1527);
        (assign64800_e99962, ((((locals.var_t1_dn0 * locals.var_vgp__blk1527) + (locals.var_t1 * locals.var_vgp__blk1527_dn0)) * locals.var_vgp__blk1527) + (assign64800_e99960 * locals.var_vgp__blk1527_dn0)), ((((locals.var_t1_dn2 * locals.var_vgp__blk1527) + (locals.var_t1 * locals.var_vgp__blk1527_dn2)) * locals.var_vgp__blk1527) + (assign64800_e99960 * locals.var_vgp__blk1527_dn2)), ((((locals.var_t1_dn4 * locals.var_vgp__blk1527) + (locals.var_t1 * locals.var_vgp__blk1527_dn4)) * locals.var_vgp__blk1527) + (assign64800_e99960 * locals.var_vgp__blk1527_dn4)), ((((locals.var_t1_dn5 * locals.var_vgp__blk1527) + (locals.var_t1 * locals.var_vgp__blk1527_dn5)) * locals.var_vgp__blk1527) + (assign64800_e99960 * locals.var_vgp__blk1527_dn5)), ((((locals.var_t1_dn6 * locals.var_vgp__blk1527) + (locals.var_t1 * locals.var_vgp__blk1527_dn6)) * locals.var_vgp__blk1527) + (assign64800_e99960 * locals.var_vgp__blk1527_dn6)), ((((locals.var_t1_dn7 * locals.var_vgp__blk1527) + (locals.var_t1 * locals.var_vgp__blk1527_dn7)) * locals.var_vgp__blk1527) + (assign64800_e99960 * locals.var_vgp__blk1527_dn7)), ((((locals.var_t1_dn8 * locals.var_vgp__blk1527) + (locals.var_t1 * locals.var_vgp__blk1527_dn8)) * locals.var_vgp__blk1527) + (assign64800_e99960 * locals.var_vgp__blk1527_dn8)), ((((locals.var_t1_dn9 * locals.var_vgp__blk1527) + (locals.var_t1 * locals.var_vgp__blk1527_dn9)) * locals.var_vgp__blk1527) + (assign64800_e99960 * locals.var_vgp__blk1527_dn9)), ((((locals.var_t1_dn10 * locals.var_vgp__blk1527) + (locals.var_t1 * locals.var_vgp__blk1527_dn10)) * locals.var_vgp__blk1527) + (assign64800_e99960 * locals.var_vgp__blk1527_dn10)), ((((locals.var_t1_dn11 * locals.var_vgp__blk1527) + (locals.var_t1 * locals.var_vgp__blk1527_dn11)) * locals.var_vgp__blk1527) + (assign64800_e99960 * locals.var_vgp__blk1527_dn11)), ((((locals.var_t1_dn14 * locals.var_vgp__blk1527) + (locals.var_t1 * locals.var_vgp__blk1527_dn14)) * locals.var_vgp__blk1527) + (assign64800_e99960 * locals.var_vgp__blk1527_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign64800_e99964;
        locals.var_t2_dn0 = assign64800_e99964_d_n0;
        locals.var_t2_dn2 = assign64800_e99964_d_n2;
        locals.var_t2_dn4 = assign64800_e99964_d_n4;
        locals.var_t2_dn5 = assign64800_e99964_d_n5;
        locals.var_t2_dn6 = assign64800_e99964_d_n6;
        locals.var_t2_dn7 = assign64800_e99964_d_n7;
        locals.var_t2_dn8 = assign64800_e99964_d_n8;
        locals.var_t2_dn9 = assign64800_e99964_d_n9;
        locals.var_t2_dn10 = assign64800_e99964_d_n10;
        locals.var_t2_dn11 = assign64800_e99964_d_n11;
        locals.var_t2_dn14 = assign64800_e99964_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign64810_e99984, assign64810_e99984_d_n0, assign64810_e99984_d_n2, assign64810_e99984_d_n4, assign64810_e99984_d_n5, assign64810_e99984_d_n6, assign64810_e99984_d_n7, assign64810_e99984_d_n8, assign64810_e99984_d_n9, assign64810_e99984_d_n10, assign64810_e99984_d_n11, assign64810_e99984_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1543 == 0.0)) && (locals.var_guard1544 == 0.0)) {
        let assign64810_e99981: f64 = (2.0 / locals.var_vgp__blk1527);
        let assign64810_e99982: f64 = (locals.var_beta + assign64810_e99981);
        (assign64810_e99982, (locals.var_beta_dn0 + (-((2.0 * locals.var_vgp__blk1527_dn0) / (locals.var_vgp__blk1527 * locals.var_vgp__blk1527)))), (locals.var_beta_dn2 + (-((2.0 * locals.var_vgp__blk1527_dn2) / (locals.var_vgp__blk1527 * locals.var_vgp__blk1527)))), (locals.var_beta_dn4 + (-((2.0 * locals.var_vgp__blk1527_dn4) / (locals.var_vgp__blk1527 * locals.var_vgp__blk1527)))), (locals.var_beta_dn5 + (-((2.0 * locals.var_vgp__blk1527_dn5) / (locals.var_vgp__blk1527 * locals.var_vgp__blk1527)))), (locals.var_beta_dn6 + (-((2.0 * locals.var_vgp__blk1527_dn6) / (locals.var_vgp__blk1527 * locals.var_vgp__blk1527)))), (locals.var_beta_dn7 + (-((2.0 * locals.var_vgp__blk1527_dn7) / (locals.var_vgp__blk1527 * locals.var_vgp__blk1527)))), (locals.var_beta_dn8 + (-((2.0 * locals.var_vgp__blk1527_dn8) / (locals.var_vgp__blk1527 * locals.var_vgp__blk1527)))), (locals.var_beta_dn9 + (-((2.0 * locals.var_vgp__blk1527_dn9) / (locals.var_vgp__blk1527 * locals.var_vgp__blk1527)))), (locals.var_beta_dn10 + (-((2.0 * locals.var_vgp__blk1527_dn10) / (locals.var_vgp__blk1527 * locals.var_vgp__blk1527)))), (locals.var_beta_dn11 + (-((2.0 * locals.var_vgp__blk1527_dn11) / (locals.var_vgp__blk1527 * locals.var_vgp__blk1527)))), (locals.var_beta_dn14 + (-((2.0 * locals.var_vgp__blk1527_dn14) / (locals.var_vgp__blk1527 * locals.var_vgp__blk1527)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign64810_e99984;
        locals.var_t3_dn0 = assign64810_e99984_d_n0;
        locals.var_t3_dn2 = assign64810_e99984_d_n2;
        locals.var_t3_dn4 = assign64810_e99984_d_n4;
        locals.var_t3_dn5 = assign64810_e99984_d_n5;
        locals.var_t3_dn6 = assign64810_e99984_d_n6;
        locals.var_t3_dn7 = assign64810_e99984_d_n7;
        locals.var_t3_dn8 = assign64810_e99984_d_n8;
        locals.var_t3_dn9 = assign64810_e99984_d_n9;
        locals.var_t3_dn10 = assign64810_e99984_d_n10;
        locals.var_t3_dn11 = assign64810_e99984_d_n11;
        locals.var_t3_dn14 = assign64810_e99984_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign64820_e100005, assign64820_e100005_d_n0, assign64820_e100005_d_n2, assign64820_e100005_d_n4, assign64820_e100005_d_n5, assign64820_e100005_d_n6, assign64820_e100005_d_n7, assign64820_e100005_d_n8, assign64820_e100005_d_n9, assign64820_e100005_d_n10, assign64820_e100005_d_n11, assign64820_e100005_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1543 == 0.0)) && (locals.var_guard1544 == 0.0)) {
        let assign64820_e99999: f64 = (locals.var_t2).ln();
        let assign64820_e100001: f64 = (assign64820_e99999 / locals.var_t3);
        let assign64820_e100003: f64 = (assign64820_e100001 + p.p456);
        (assign64820_e100003, ((((locals.var_t2_dn0 / locals.var_t2) * locals.var_t3) - (assign64820_e99999 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn2 / locals.var_t2) * locals.var_t3) - (assign64820_e99999 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn4 / locals.var_t2) * locals.var_t3) - (assign64820_e99999 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn5 / locals.var_t2) * locals.var_t3) - (assign64820_e99999 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn6 / locals.var_t2) * locals.var_t3) - (assign64820_e99999 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn7 / locals.var_t2) * locals.var_t3) - (assign64820_e99999 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn8 / locals.var_t2) * locals.var_t3) - (assign64820_e99999 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn9 / locals.var_t2) * locals.var_t3) - (assign64820_e99999 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn10 / locals.var_t2) * locals.var_t3) - (assign64820_e99999 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn11 / locals.var_t2) * locals.var_t3) - (assign64820_e99999 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn14 / locals.var_t2) * locals.var_t3) - (assign64820_e99999 * locals.var_t3_dn14)) / (locals.var_t3 * locals.var_t3)),)
    } else {
        (locals.var_ps0_inib, locals.var_ps0_inib_dn0, locals.var_ps0_inib_dn2, locals.var_ps0_inib_dn4, locals.var_ps0_inib_dn5, locals.var_ps0_inib_dn6, locals.var_ps0_inib_dn7, locals.var_ps0_inib_dn8, locals.var_ps0_inib_dn9, locals.var_ps0_inib_dn10, locals.var_ps0_inib_dn11, locals.var_ps0_inib_dn14,)
    }
};
        locals.var_ps0_inib = assign64820_e100005;
        locals.var_ps0_inib_dn0 = assign64820_e100005_d_n0;
        locals.var_ps0_inib_dn2 = assign64820_e100005_d_n2;
        locals.var_ps0_inib_dn4 = assign64820_e100005_d_n4;
        locals.var_ps0_inib_dn5 = assign64820_e100005_d_n5;
        locals.var_ps0_inib_dn6 = assign64820_e100005_d_n6;
        locals.var_ps0_inib_dn7 = assign64820_e100005_d_n7;
        locals.var_ps0_inib_dn8 = assign64820_e100005_d_n8;
        locals.var_ps0_inib_dn9 = assign64820_e100005_d_n9;
        locals.var_ps0_inib_dn10 = assign64820_e100005_d_n10;
        locals.var_ps0_inib_dn11 = assign64820_e100005_d_n11;
        locals.var_ps0_inib_dn14 = assign64820_e100005_d_n14;
        locals.var_ps0_inib_rv = 0.0;

        let (assign64830_e100025, assign64830_e100025_d_n0, assign64830_e100025_d_n2, assign64830_e100025_d_n4, assign64830_e100025_d_n5, assign64830_e100025_d_n6, assign64830_e100025_d_n7, assign64830_e100025_d_n8, assign64830_e100025_d_n9, assign64830_e100025_d_n10, assign64830_e100025_d_n11, assign64830_e100025_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1543 == 0.0)) && (locals.var_guard1544 == 0.0)) {
        let assign64830_e100021: f64 = (locals.var_ps0_inib - locals.var_ps0_inia);
        let assign64830_e100023: f64 = (assign64830_e100021 - 0.0008);
        (assign64830_e100023, (locals.var_ps0_inib_dn0 - locals.var_ps0_inia_dn0), (locals.var_ps0_inib_dn2 - locals.var_ps0_inia_dn2), (locals.var_ps0_inib_dn4 - locals.var_ps0_inia_dn4), (locals.var_ps0_inib_dn5 - locals.var_ps0_inia_dn5), (locals.var_ps0_inib_dn6 - locals.var_ps0_inia_dn6), (locals.var_ps0_inib_dn7 - locals.var_ps0_inia_dn7), (locals.var_ps0_inib_dn8 - locals.var_ps0_inia_dn8), (locals.var_ps0_inib_dn9 - locals.var_ps0_inia_dn9), (locals.var_ps0_inib_dn10 - locals.var_ps0_inia_dn10), (locals.var_ps0_inib_dn11 - locals.var_ps0_inia_dn11), (locals.var_ps0_inib_dn14 - locals.var_ps0_inia_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign64830_e100025;
        locals.var_tmf1_dn0 = assign64830_e100025_d_n0;
        locals.var_tmf1_dn2 = assign64830_e100025_d_n2;
        locals.var_tmf1_dn4 = assign64830_e100025_d_n4;
        locals.var_tmf1_dn5 = assign64830_e100025_d_n5;
        locals.var_tmf1_dn6 = assign64830_e100025_d_n6;
        locals.var_tmf1_dn7 = assign64830_e100025_d_n7;
        locals.var_tmf1_dn8 = assign64830_e100025_d_n8;
        locals.var_tmf1_dn9 = assign64830_e100025_d_n9;
        locals.var_tmf1_dn10 = assign64830_e100025_d_n10;
        locals.var_tmf1_dn11 = assign64830_e100025_d_n11;
        locals.var_tmf1_dn14 = assign64830_e100025_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign64840_e100045, assign64840_e100045_d_n0, assign64840_e100045_d_n2, assign64840_e100045_d_n4, assign64840_e100045_d_n5, assign64840_e100045_d_n6, assign64840_e100045_d_n7, assign64840_e100045_d_n8, assign64840_e100045_d_n9, assign64840_e100045_d_n10, assign64840_e100045_d_n11, assign64840_e100045_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1543 == 0.0)) && (locals.var_guard1544 == 0.0)) {
        let assign64840_e100041: f64 = (4.0 * locals.var_ps0_inib);
        let assign64840_e100043: f64 = (assign64840_e100041 * 0.0008);
        (assign64840_e100043, ((4.0 * locals.var_ps0_inib_dn0) * 0.0008), ((4.0 * locals.var_ps0_inib_dn2) * 0.0008), ((4.0 * locals.var_ps0_inib_dn4) * 0.0008), ((4.0 * locals.var_ps0_inib_dn5) * 0.0008), ((4.0 * locals.var_ps0_inib_dn6) * 0.0008), ((4.0 * locals.var_ps0_inib_dn7) * 0.0008), ((4.0 * locals.var_ps0_inib_dn8) * 0.0008), ((4.0 * locals.var_ps0_inib_dn9) * 0.0008), ((4.0 * locals.var_ps0_inib_dn10) * 0.0008), ((4.0 * locals.var_ps0_inib_dn11) * 0.0008), ((4.0 * locals.var_ps0_inib_dn14) * 0.0008),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign64840_e100045;
        locals.var_tmf2_dn0 = assign64840_e100045_d_n0;
        locals.var_tmf2_dn2 = assign64840_e100045_d_n2;
        locals.var_tmf2_dn4 = assign64840_e100045_d_n4;
        locals.var_tmf2_dn5 = assign64840_e100045_d_n5;
        locals.var_tmf2_dn6 = assign64840_e100045_d_n6;
        locals.var_tmf2_dn7 = assign64840_e100045_d_n7;
        locals.var_tmf2_dn8 = assign64840_e100045_d_n8;
        locals.var_tmf2_dn9 = assign64840_e100045_d_n9;
        locals.var_tmf2_dn10 = assign64840_e100045_d_n10;
        locals.var_tmf2_dn11 = assign64840_e100045_d_n11;
        locals.var_tmf2_dn14 = assign64840_e100045_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign64850_e100067, assign64850_e100067_d_n0, assign64850_e100067_d_n2, assign64850_e100067_d_n4, assign64850_e100067_d_n5, assign64850_e100067_d_n6, assign64850_e100067_d_n7, assign64850_e100067_d_n8, assign64850_e100067_d_n9, assign64850_e100067_d_n10, assign64850_e100067_d_n11, assign64850_e100067_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1543 == 0.0)) && (locals.var_guard1544 == 0.0)) {
        let (assign64850_e100065, assign64850_e100065_d_n0, assign64850_e100065_d_n2, assign64850_e100065_d_n4, assign64850_e100065_d_n5, assign64850_e100065_d_n6, assign64850_e100065_d_n7, assign64850_e100065_d_n8, assign64850_e100065_d_n9, assign64850_e100065_d_n10, assign64850_e100065_d_n11, assign64850_e100065_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign64850_e100064: f64 = (-locals.var_tmf2);
                (assign64850_e100064, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign64850_e100065, assign64850_e100065_d_n0, assign64850_e100065_d_n2, assign64850_e100065_d_n4, assign64850_e100065_d_n5, assign64850_e100065_d_n6, assign64850_e100065_d_n7, assign64850_e100065_d_n8, assign64850_e100065_d_n9, assign64850_e100065_d_n10, assign64850_e100065_d_n11, assign64850_e100065_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign64850_e100067;
        locals.var_tmf2_dn0 = assign64850_e100067_d_n0;
        locals.var_tmf2_dn2 = assign64850_e100067_d_n2;
        locals.var_tmf2_dn4 = assign64850_e100067_d_n4;
        locals.var_tmf2_dn5 = assign64850_e100067_d_n5;
        locals.var_tmf2_dn6 = assign64850_e100067_d_n6;
        locals.var_tmf2_dn7 = assign64850_e100067_d_n7;
        locals.var_tmf2_dn8 = assign64850_e100067_d_n8;
        locals.var_tmf2_dn9 = assign64850_e100067_d_n9;
        locals.var_tmf2_dn10 = assign64850_e100067_d_n10;
        locals.var_tmf2_dn11 = assign64850_e100067_d_n11;
        locals.var_tmf2_dn14 = assign64850_e100067_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign64860_e100088, assign64860_e100088_d_n0, assign64860_e100088_d_n2, assign64860_e100088_d_n4, assign64860_e100088_d_n5, assign64860_e100088_d_n6, assign64860_e100088_d_n7, assign64860_e100088_d_n8, assign64860_e100088_d_n9, assign64860_e100088_d_n10, assign64860_e100088_d_n11, assign64860_e100088_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1543 == 0.0)) && (locals.var_guard1544 == 0.0)) {
        let assign64860_e100083: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign64860_e100085: f64 = (assign64860_e100083 + locals.var_tmf2);
        let assign64860_e100086: f64 = (assign64860_e100085).sqrt();
        (assign64860_e100086, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign64860_e100086)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign64860_e100086)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign64860_e100086)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign64860_e100086)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign64860_e100086)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign64860_e100086)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign64860_e100086)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign64860_e100086)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign64860_e100086)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign64860_e100086)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign64860_e100086)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign64860_e100088;
        locals.var_tmf2_dn0 = assign64860_e100088_d_n0;
        locals.var_tmf2_dn2 = assign64860_e100088_d_n2;
        locals.var_tmf2_dn4 = assign64860_e100088_d_n4;
        locals.var_tmf2_dn5 = assign64860_e100088_d_n5;
        locals.var_tmf2_dn6 = assign64860_e100088_d_n6;
        locals.var_tmf2_dn7 = assign64860_e100088_d_n7;
        locals.var_tmf2_dn8 = assign64860_e100088_d_n8;
        locals.var_tmf2_dn9 = assign64860_e100088_d_n9;
        locals.var_tmf2_dn10 = assign64860_e100088_d_n10;
        locals.var_tmf2_dn11 = assign64860_e100088_d_n11;
        locals.var_tmf2_dn14 = assign64860_e100088_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign64870_e100110, assign64870_e100110_d_n0, assign64870_e100110_d_n2, assign64870_e100110_d_n4, assign64870_e100110_d_n5, assign64870_e100110_d_n6, assign64870_e100110_d_n7, assign64870_e100110_d_n8, assign64870_e100110_d_n9, assign64870_e100110_d_n10, assign64870_e100110_d_n11, assign64870_e100110_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1543 == 0.0)) && (locals.var_guard1544 == 0.0)) {
        let assign64870_e100106: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign64870_e100107: f64 = (0.5 * assign64870_e100106);
        let assign64870_e100108: f64 = (locals.var_ps0_inib - assign64870_e100107);
        (assign64870_e100108, (locals.var_ps0_inib_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_ps0_inib_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_ps0_inib_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_ps0_inib_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_ps0_inib_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_ps0_inib_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_ps0_inib_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_ps0_inib_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_ps0_inib_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_ps0_inib_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_ps0_inib_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn14,)
    }
};
        locals.var_ps0_ini = assign64870_e100110;
        locals.var_ps0_ini_dn0 = assign64870_e100110_d_n0;
        locals.var_ps0_ini_dn2 = assign64870_e100110_d_n2;
        locals.var_ps0_ini_dn4 = assign64870_e100110_d_n4;
        locals.var_ps0_ini_dn5 = assign64870_e100110_d_n5;
        locals.var_ps0_ini_dn6 = assign64870_e100110_d_n6;
        locals.var_ps0_ini_dn7 = assign64870_e100110_d_n7;
        locals.var_ps0_ini_dn8 = assign64870_e100110_d_n8;
        locals.var_ps0_ini_dn9 = assign64870_e100110_d_n9;
        locals.var_ps0_ini_dn10 = assign64870_e100110_d_n10;
        locals.var_ps0_ini_dn11 = assign64870_e100110_d_n11;
        locals.var_ps0_ini_dn14 = assign64870_e100110_d_n14;
        locals.var_ps0_ini_rv = 0.0;

        let (assign64880_e100124, assign64880_e100124_d_n0, assign64880_e100124_d_n2, assign64880_e100124_d_n4, assign64880_e100124_d_n5, assign64880_e100124_d_n6, assign64880_e100124_d_n7, assign64880_e100124_d_n8, assign64880_e100124_d_n9, assign64880_e100124_d_n10, assign64880_e100124_d_n11, assign64880_e100124_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) {
        let assign64880_e100121: f64 = (1e-12 / 2.0);
        let assign64880_e100122: f64 = (locals.var_vbscl__blk1541 + assign64880_e100121);
        (assign64880_e100122, locals.var_vbscl__blk1541_dn0, locals.var_vbscl__blk1541_dn2, locals.var_vbscl__blk1541_dn4, locals.var_vbscl__blk1541_dn5, locals.var_vbscl__blk1541_dn6, locals.var_vbscl__blk1541_dn7, locals.var_vbscl__blk1541_dn8, locals.var_vbscl__blk1541_dn9, locals.var_vbscl__blk1541_dn10, locals.var_vbscl__blk1541_dn11, locals.var_vbscl__blk1541_dn14,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign64880_e100124;
        locals.var_tx_dn0 = assign64880_e100124_d_n0;
        locals.var_tx_dn2 = assign64880_e100124_d_n2;
        locals.var_tx_dn4 = assign64880_e100124_d_n4;
        locals.var_tx_dn5 = assign64880_e100124_d_n5;
        locals.var_tx_dn6 = assign64880_e100124_d_n6;
        locals.var_tx_dn7 = assign64880_e100124_d_n7;
        locals.var_tx_dn8 = assign64880_e100124_d_n8;
        locals.var_tx_dn9 = assign64880_e100124_d_n9;
        locals.var_tx_dn10 = assign64880_e100124_d_n10;
        locals.var_tx_dn11 = assign64880_e100124_d_n11;
        locals.var_tx_dn14 = assign64880_e100124_d_n14;
        locals.var_tx_rv = 0.0;

        let assign64890_e100127: f64 = if locals.var_ps0_ini < locals.var_tx { 1.0 } else { 0.0 };
        locals.var_guard1545 = assign64890_e100127;
        locals.var_guard1545_rv = 0.0;

        let (assign64900_e100139, assign64900_e100139_d_n0, assign64900_e100139_d_n2, assign64900_e100139_d_n4, assign64900_e100139_d_n5, assign64900_e100139_d_n6, assign64900_e100139_d_n7, assign64900_e100139_d_n8, assign64900_e100139_d_n9, assign64900_e100139_d_n10, assign64900_e100139_d_n11, assign64900_e100139_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1545 != 0.0)) {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn14,)
    }
};
        locals.var_ps0_ini = assign64900_e100139;
        locals.var_ps0_ini_dn0 = assign64900_e100139_d_n0;
        locals.var_ps0_ini_dn2 = assign64900_e100139_d_n2;
        locals.var_ps0_ini_dn4 = assign64900_e100139_d_n4;
        locals.var_ps0_ini_dn5 = assign64900_e100139_d_n5;
        locals.var_ps0_ini_dn6 = assign64900_e100139_d_n6;
        locals.var_ps0_ini_dn7 = assign64900_e100139_d_n7;
        locals.var_ps0_ini_dn8 = assign64900_e100139_d_n8;
        locals.var_ps0_ini_dn9 = assign64900_e100139_d_n9;
        locals.var_ps0_ini_dn10 = assign64900_e100139_d_n10;
        locals.var_ps0_ini_dn11 = assign64900_e100139_d_n11;
        locals.var_ps0_ini_dn14 = assign64900_e100139_d_n14;
        locals.var_ps0_ini_rv = 0.0;

        let (assign64910_e100149, assign64910_e100149_d_n0, assign64910_e100149_d_n2, assign64910_e100149_d_n4, assign64910_e100149_d_n5, assign64910_e100149_d_n6, assign64910_e100149_d_n7, assign64910_e100149_d_n8, assign64910_e100149_d_n9, assign64910_e100149_d_n10, assign64910_e100149_d_n11, assign64910_e100149_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn14,)
    } else {
        (locals.var_ps0__blk1525, locals.var_ps0__blk1525_dn0, locals.var_ps0__blk1525_dn2, locals.var_ps0__blk1525_dn4, locals.var_ps0__blk1525_dn5, locals.var_ps0__blk1525_dn6, locals.var_ps0__blk1525_dn7, locals.var_ps0__blk1525_dn8, locals.var_ps0__blk1525_dn9, locals.var_ps0__blk1525_dn10, locals.var_ps0__blk1525_dn11, locals.var_ps0__blk1525_dn14,)
    }
};
        locals.var_ps0__blk1525 = assign64910_e100149;
        locals.var_ps0__blk1525_dn0 = assign64910_e100149_d_n0;
        locals.var_ps0__blk1525_dn2 = assign64910_e100149_d_n2;
        locals.var_ps0__blk1525_dn4 = assign64910_e100149_d_n4;
        locals.var_ps0__blk1525_dn5 = assign64910_e100149_d_n5;
        locals.var_ps0__blk1525_dn6 = assign64910_e100149_d_n6;
        locals.var_ps0__blk1525_dn7 = assign64910_e100149_d_n7;
        locals.var_ps0__blk1525_dn8 = assign64910_e100149_d_n8;
        locals.var_ps0__blk1525_dn9 = assign64910_e100149_d_n9;
        locals.var_ps0__blk1525_dn10 = assign64910_e100149_d_n10;
        locals.var_ps0__blk1525_dn11 = assign64910_e100149_d_n11;
        locals.var_ps0__blk1525_dn14 = assign64910_e100149_d_n14;
        locals.var_ps0__blk1525_rv = 0.0;

        let assign64920_e100152: f64 = if p.p451 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1546 = assign64920_e100152;
        locals.var_guard1546_rv = 0.0;

        let (assign64930_e100164, assign64930_e100164_d_n0, assign64930_e100164_d_n2, assign64930_e100164_d_n4, assign64930_e100164_d_n5, assign64930_e100164_d_n6, assign64930_e100164_d_n7, assign64930_e100164_d_n8, assign64930_e100164_d_n9, assign64930_e100164_d_n10, assign64930_e100164_d_n11, assign64930_e100164_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) {
        (locals.var_ps0__blk1525, locals.var_ps0__blk1525_dn0, locals.var_ps0__blk1525_dn2, locals.var_ps0__blk1525_dn4, locals.var_ps0__blk1525_dn5, locals.var_ps0__blk1525_dn6, locals.var_ps0__blk1525_dn7, locals.var_ps0__blk1525_dn8, locals.var_ps0__blk1525_dn9, locals.var_ps0__blk1525_dn10, locals.var_ps0__blk1525_dn11, locals.var_ps0__blk1525_dn14,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn14,)
    }
};
        locals.var_ps0_ini = assign64930_e100164;
        locals.var_ps0_ini_dn0 = assign64930_e100164_d_n0;
        locals.var_ps0_ini_dn2 = assign64930_e100164_d_n2;
        locals.var_ps0_ini_dn4 = assign64930_e100164_d_n4;
        locals.var_ps0_ini_dn5 = assign64930_e100164_d_n5;
        locals.var_ps0_ini_dn6 = assign64930_e100164_d_n6;
        locals.var_ps0_ini_dn7 = assign64930_e100164_d_n7;
        locals.var_ps0_ini_dn8 = assign64930_e100164_d_n8;
        locals.var_ps0_ini_dn9 = assign64930_e100164_d_n9;
        locals.var_ps0_ini_dn10 = assign64930_e100164_d_n10;
        locals.var_ps0_ini_dn11 = assign64930_e100164_d_n11;
        locals.var_ps0_ini_dn14 = assign64930_e100164_d_n14;
        locals.var_ps0_ini_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_243(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign64940_e100176, assign64940_e100176_d_n0, assign64940_e100176_d_n2, assign64940_e100176_d_n4, assign64940_e100176_d_n5, assign64940_e100176_d_n6, assign64940_e100176_d_n7, assign64940_e100176_d_n8, assign64940_e100176_d_n9, assign64940_e100176_d_n10, assign64940_e100176_d_n11, assign64940_e100176_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) {
        (locals.var_dphi_vds, locals.var_dphi_vds_dn0, locals.var_dphi_vds_dn2, locals.var_dphi_vds_dn4, locals.var_dphi_vds_dn5, locals.var_dphi_vds_dn6, locals.var_dphi_vds_dn7, locals.var_dphi_vds_dn8, locals.var_dphi_vds_dn9, locals.var_dphi_vds_dn10, locals.var_dphi_vds_dn11, locals.var_dphi_vds_dn14,)
    } else {
        (locals.var_vbscl__blk1547, locals.var_vbscl__blk1547_dn0, locals.var_vbscl__blk1547_dn2, locals.var_vbscl__blk1547_dn4, locals.var_vbscl__blk1547_dn5, locals.var_vbscl__blk1547_dn6, locals.var_vbscl__blk1547_dn7, locals.var_vbscl__blk1547_dn8, locals.var_vbscl__blk1547_dn9, locals.var_vbscl__blk1547_dn10, locals.var_vbscl__blk1547_dn11, locals.var_vbscl__blk1547_dn14,)
    }
};
        locals.var_vbscl__blk1547 = assign64940_e100176;
        locals.var_vbscl__blk1547_dn0 = assign64940_e100176_d_n0;
        locals.var_vbscl__blk1547_dn2 = assign64940_e100176_d_n2;
        locals.var_vbscl__blk1547_dn4 = assign64940_e100176_d_n4;
        locals.var_vbscl__blk1547_dn5 = assign64940_e100176_d_n5;
        locals.var_vbscl__blk1547_dn6 = assign64940_e100176_d_n6;
        locals.var_vbscl__blk1547_dn7 = assign64940_e100176_d_n7;
        locals.var_vbscl__blk1547_dn8 = assign64940_e100176_d_n8;
        locals.var_vbscl__blk1547_dn9 = assign64940_e100176_d_n9;
        locals.var_vbscl__blk1547_dn10 = assign64940_e100176_d_n10;
        locals.var_vbscl__blk1547_dn11 = assign64940_e100176_d_n11;
        locals.var_vbscl__blk1547_dn14 = assign64940_e100176_d_n14;
        locals.var_vbscl__blk1547_rv = 0.0;

        let (assign64950_e100196, assign64950_e100196_d_n0, assign64950_e100196_d_n2, assign64950_e100196_d_n4, assign64950_e100196_d_n5, assign64950_e100196_d_n6, assign64950_e100196_d_n7, assign64950_e100196_d_n8, assign64950_e100196_d_n9, assign64950_e100196_d_n10, assign64950_e100196_d_n11, assign64950_e100196_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) {
        let assign64950_e100188: f64 = (locals.var_vfb - locals.var_dvth);
        let assign64950_e100190: f64 = (assign64950_e100188 + locals.var_dppg);
        let assign64950_e100192: f64 = (assign64950_e100190 + locals.var_vbscl__blk1547);
        let assign64950_e100194: f64 = (assign64950_e100192 + p.p455);
        (assign64950_e100194, (((-locals.var_dvth_dn0) + locals.var_dppg_dn0) + locals.var_vbscl__blk1547_dn0), (((-locals.var_dvth_dn2) + locals.var_dppg_dn2) + locals.var_vbscl__blk1547_dn2), (((-locals.var_dvth_dn4) + locals.var_dppg_dn4) + locals.var_vbscl__blk1547_dn4), (((-locals.var_dvth_dn5) + locals.var_dppg_dn5) + locals.var_vbscl__blk1547_dn5), (((-locals.var_dvth_dn6) + locals.var_dppg_dn6) + locals.var_vbscl__blk1547_dn6), (((-locals.var_dvth_dn7) + locals.var_dppg_dn7) + locals.var_vbscl__blk1547_dn7), (((-locals.var_dvth_dn8) + locals.var_dppg_dn8) + locals.var_vbscl__blk1547_dn8), (((-locals.var_dvth_dn9) + locals.var_dppg_dn9) + locals.var_vbscl__blk1547_dn9), (((-locals.var_dvth_dn10) + locals.var_dppg_dn10) + locals.var_vbscl__blk1547_dn10), (((-locals.var_dvth_dn11) + locals.var_dppg_dn11) + locals.var_vbscl__blk1547_dn11), (((-locals.var_dvth_dn14) + locals.var_dppg_dn14) + locals.var_vbscl__blk1547_dn14),)
    } else {
        (locals.var_vgs_fb, locals.var_vgs_fb_dn0, locals.var_vgs_fb_dn2, locals.var_vgs_fb_dn4, locals.var_vgs_fb_dn5, locals.var_vgs_fb_dn6, locals.var_vgs_fb_dn7, locals.var_vgs_fb_dn8, locals.var_vgs_fb_dn9, locals.var_vgs_fb_dn10, locals.var_vgs_fb_dn11, locals.var_vgs_fb_dn14,)
    }
};
        locals.var_vgs_fb = assign64950_e100196;
        locals.var_vgs_fb_dn0 = assign64950_e100196_d_n0;
        locals.var_vgs_fb_dn2 = assign64950_e100196_d_n2;
        locals.var_vgs_fb_dn4 = assign64950_e100196_d_n4;
        locals.var_vgs_fb_dn5 = assign64950_e100196_d_n5;
        locals.var_vgs_fb_dn6 = assign64950_e100196_d_n6;
        locals.var_vgs_fb_dn7 = assign64950_e100196_d_n7;
        locals.var_vgs_fb_dn8 = assign64950_e100196_d_n8;
        locals.var_vgs_fb_dn9 = assign64950_e100196_d_n9;
        locals.var_vgs_fb_dn10 = assign64950_e100196_d_n10;
        locals.var_vgs_fb_dn11 = assign64950_e100196_d_n11;
        locals.var_vgs_fb_dn14 = assign64950_e100196_d_n14;
        locals.var_vgs_fb_rv = 0.0;

        let assign64960_e100199: f64 = if locals.var_vgs < locals.var_vgs_fb { 1.0 } else { 0.0 };
        locals.var_guard1556 = assign64960_e100199;
        locals.var_guard1556_rv = 0.0;

        let (assign64970_e100214,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 != 0.0)) {
        let assign64970_e100212: f64 = (-1.0);
        (assign64970_e100212,)
    } else {
        (locals.var_flg_zone,)
    }
};
        locals.var_flg_zone = assign64970_e100214;
        locals.var_flg_zone_rv = 0.0;

        let (assign64980_e100236, assign64980_e100236_d_n0, assign64980_e100236_d_n2, assign64980_e100236_d_n4, assign64980_e100236_d_n5, assign64980_e100236_d_n6, assign64980_e100236_d_n7, assign64980_e100236_d_n8, assign64980_e100236_d_n9, assign64980_e100236_d_n10, assign64980_e100236_d_n11, assign64980_e100236_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 != 0.0)) {
        let assign64980_e100228: f64 = (2.0 * locals.var_beta_inv);
        let assign64980_e100230: f64 = (-locals.var_vgs_min);
        let assign64980_e100232: f64 = (assign64980_e100230 / locals.var_fac1);
        let assign64980_e100233: f64 = (assign64980_e100232).ln();
        let assign64980_e100234: f64 = (assign64980_e100228 * assign64980_e100233);
        (assign64980_e100234, (((2.0 * locals.var_beta_inv_dn0) * assign64980_e100233) + (assign64980_e100228 * ((-((assign64980_e100230 * locals.var_fac1_dn0) / (locals.var_fac1 * locals.var_fac1))) / assign64980_e100232))), (((2.0 * locals.var_beta_inv_dn2) * assign64980_e100233) + (assign64980_e100228 * ((-((assign64980_e100230 * locals.var_fac1_dn2) / (locals.var_fac1 * locals.var_fac1))) / assign64980_e100232))), (((2.0 * locals.var_beta_inv_dn4) * assign64980_e100233) + (assign64980_e100228 * ((-((assign64980_e100230 * locals.var_fac1_dn4) / (locals.var_fac1 * locals.var_fac1))) / assign64980_e100232))), (((2.0 * locals.var_beta_inv_dn5) * assign64980_e100233) + (assign64980_e100228 * ((-((assign64980_e100230 * locals.var_fac1_dn5) / (locals.var_fac1 * locals.var_fac1))) / assign64980_e100232))), (((2.0 * locals.var_beta_inv_dn6) * assign64980_e100233) + (assign64980_e100228 * ((-((assign64980_e100230 * locals.var_fac1_dn6) / (locals.var_fac1 * locals.var_fac1))) / assign64980_e100232))), (((2.0 * locals.var_beta_inv_dn7) * assign64980_e100233) + (assign64980_e100228 * ((-((assign64980_e100230 * locals.var_fac1_dn7) / (locals.var_fac1 * locals.var_fac1))) / assign64980_e100232))), (((2.0 * locals.var_beta_inv_dn8) * assign64980_e100233) + (assign64980_e100228 * ((-((assign64980_e100230 * locals.var_fac1_dn8) / (locals.var_fac1 * locals.var_fac1))) / assign64980_e100232))), (((2.0 * locals.var_beta_inv_dn9) * assign64980_e100233) + (assign64980_e100228 * ((-((assign64980_e100230 * locals.var_fac1_dn9) / (locals.var_fac1 * locals.var_fac1))) / assign64980_e100232))), (((2.0 * locals.var_beta_inv_dn10) * assign64980_e100233) + (assign64980_e100228 * ((-((assign64980_e100230 * locals.var_fac1_dn10) / (locals.var_fac1 * locals.var_fac1))) / assign64980_e100232))), (((2.0 * locals.var_beta_inv_dn11) * assign64980_e100233) + (assign64980_e100228 * ((-((assign64980_e100230 * locals.var_fac1_dn11) / (locals.var_fac1 * locals.var_fac1))) / assign64980_e100232))), (((2.0 * locals.var_beta_inv_dn14) * assign64980_e100233) + (assign64980_e100228 * ((-((assign64980_e100230 * locals.var_fac1_dn14) / (locals.var_fac1 * locals.var_fac1))) / assign64980_e100232))),)
    } else {
        (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn4, locals.var_ps0_min_dn5, locals.var_ps0_min_dn6, locals.var_ps0_min_dn7, locals.var_ps0_min_dn8, locals.var_ps0_min_dn9, locals.var_ps0_min_dn10, locals.var_ps0_min_dn11, locals.var_ps0_min_dn14,)
    }
};
        locals.var_ps0_min = assign64980_e100236;
        locals.var_ps0_min_dn0 = assign64980_e100236_d_n0;
        locals.var_ps0_min_dn2 = assign64980_e100236_d_n2;
        locals.var_ps0_min_dn4 = assign64980_e100236_d_n4;
        locals.var_ps0_min_dn5 = assign64980_e100236_d_n5;
        locals.var_ps0_min_dn6 = assign64980_e100236_d_n6;
        locals.var_ps0_min_dn7 = assign64980_e100236_d_n7;
        locals.var_ps0_min_dn8 = assign64980_e100236_d_n8;
        locals.var_ps0_min_dn9 = assign64980_e100236_d_n9;
        locals.var_ps0_min_dn10 = assign64980_e100236_d_n10;
        locals.var_ps0_min_dn11 = assign64980_e100236_d_n11;
        locals.var_ps0_min_dn14 = assign64980_e100236_d_n14;
        locals.var_ps0_min_rv = 0.0;

        let (assign64990_e100254, assign64990_e100254_d_n0, assign64990_e100254_d_n2, assign64990_e100254_d_n4, assign64990_e100254_d_n5, assign64990_e100254_d_n6, assign64990_e100254_d_n7, assign64990_e100254_d_n8, assign64990_e100254_d_n9, assign64990_e100254_d_n10, assign64990_e100254_d_n11, assign64990_e100254_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 != 0.0)) {
        let assign64990_e100251: f64 = (locals.var_vgp__blk1527 - locals.var_vbscl__blk1547);
        let assign64990_e100252: f64 = (locals.var_beta * assign64990_e100251);
        (assign64990_e100252, ((locals.var_beta_dn0 * assign64990_e100251) + (locals.var_beta * (locals.var_vgp__blk1527_dn0 - locals.var_vbscl__blk1547_dn0))), ((locals.var_beta_dn2 * assign64990_e100251) + (locals.var_beta * (locals.var_vgp__blk1527_dn2 - locals.var_vbscl__blk1547_dn2))), ((locals.var_beta_dn4 * assign64990_e100251) + (locals.var_beta * (locals.var_vgp__blk1527_dn4 - locals.var_vbscl__blk1547_dn4))), ((locals.var_beta_dn5 * assign64990_e100251) + (locals.var_beta * (locals.var_vgp__blk1527_dn5 - locals.var_vbscl__blk1547_dn5))), ((locals.var_beta_dn6 * assign64990_e100251) + (locals.var_beta * (locals.var_vgp__blk1527_dn6 - locals.var_vbscl__blk1547_dn6))), ((locals.var_beta_dn7 * assign64990_e100251) + (locals.var_beta * (locals.var_vgp__blk1527_dn7 - locals.var_vbscl__blk1547_dn7))), ((locals.var_beta_dn8 * assign64990_e100251) + (locals.var_beta * (locals.var_vgp__blk1527_dn8 - locals.var_vbscl__blk1547_dn8))), ((locals.var_beta_dn9 * assign64990_e100251) + (locals.var_beta * (locals.var_vgp__blk1527_dn9 - locals.var_vbscl__blk1547_dn9))), ((locals.var_beta_dn10 * assign64990_e100251) + (locals.var_beta * (locals.var_vgp__blk1527_dn10 - locals.var_vbscl__blk1547_dn10))), ((locals.var_beta_dn11 * assign64990_e100251) + (locals.var_beta * (locals.var_vgp__blk1527_dn11 - locals.var_vbscl__blk1547_dn11))), ((locals.var_beta_dn14 * assign64990_e100251) + (locals.var_beta * (locals.var_vgp__blk1527_dn14 - locals.var_vbscl__blk1547_dn14))),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign64990_e100254;
        locals.var_tx_dn0 = assign64990_e100254_d_n0;
        locals.var_tx_dn2 = assign64990_e100254_d_n2;
        locals.var_tx_dn4 = assign64990_e100254_d_n4;
        locals.var_tx_dn5 = assign64990_e100254_d_n5;
        locals.var_tx_dn6 = assign64990_e100254_d_n6;
        locals.var_tx_dn7 = assign64990_e100254_d_n7;
        locals.var_tx_dn8 = assign64990_e100254_d_n8;
        locals.var_tx_dn9 = assign64990_e100254_d_n9;
        locals.var_tx_dn10 = assign64990_e100254_d_n10;
        locals.var_tx_dn11 = assign64990_e100254_d_n11;
        locals.var_tx_dn14 = assign64990_e100254_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign65000_e100272, assign65000_e100272_d_n0, assign65000_e100272_d_n2, assign65000_e100272_d_n4, assign65000_e100272_d_n5, assign65000_e100272_d_n6, assign65000_e100272_d_n7, assign65000_e100272_d_n8, assign65000_e100272_d_n9, assign65000_e100272_d_n10, assign65000_e100272_d_n11, assign65000_e100272_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 != 0.0)) {
        let assign65000_e100269: f64 = (locals.var_beta * locals.var_cnst0);
        let assign65000_e100270: f64 = (1.0 / assign65000_e100269);
        (assign65000_e100270, (-(((locals.var_beta_dn0 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn0)) / (assign65000_e100269 * assign65000_e100269))), (-(((locals.var_beta_dn2 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn2)) / (assign65000_e100269 * assign65000_e100269))), (-(((locals.var_beta_dn4 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn4)) / (assign65000_e100269 * assign65000_e100269))), (-(((locals.var_beta_dn5 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn5)) / (assign65000_e100269 * assign65000_e100269))), (-(((locals.var_beta_dn6 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn6)) / (assign65000_e100269 * assign65000_e100269))), (-(((locals.var_beta_dn7 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn7)) / (assign65000_e100269 * assign65000_e100269))), (-(((locals.var_beta_dn8 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn8)) / (assign65000_e100269 * assign65000_e100269))), (-(((locals.var_beta_dn9 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn9)) / (assign65000_e100269 * assign65000_e100269))), (-(((locals.var_beta_dn10 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn10)) / (assign65000_e100269 * assign65000_e100269))), (-(((locals.var_beta_dn11 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn11)) / (assign65000_e100269 * assign65000_e100269))), (-(((locals.var_beta_dn14 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn14)) / (assign65000_e100269 * assign65000_e100269))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign65000_e100272;
        locals.var_t1_dn0 = assign65000_e100272_d_n0;
        locals.var_t1_dn2 = assign65000_e100272_d_n2;
        locals.var_t1_dn4 = assign65000_e100272_d_n4;
        locals.var_t1_dn5 = assign65000_e100272_d_n5;
        locals.var_t1_dn6 = assign65000_e100272_d_n6;
        locals.var_t1_dn7 = assign65000_e100272_d_n7;
        locals.var_t1_dn8 = assign65000_e100272_d_n8;
        locals.var_t1_dn9 = assign65000_e100272_d_n9;
        locals.var_t1_dn10 = assign65000_e100272_d_n10;
        locals.var_t1_dn11 = assign65000_e100272_d_n11;
        locals.var_t1_dn14 = assign65000_e100272_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign65010_e100288, assign65010_e100288_d_n0, assign65010_e100288_d_n2, assign65010_e100288_d_n4, assign65010_e100288_d_n5, assign65010_e100288_d_n6, assign65010_e100288_d_n7, assign65010_e100288_d_n8, assign65010_e100288_d_n9, assign65010_e100288_d_n10, assign65010_e100288_d_n11, assign65010_e100288_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 != 0.0)) {
        let assign65010_e100286: f64 = (locals.var_t1 * locals.var_cox);
        (assign65010_e100286, ((locals.var_t1_dn0 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn0)), ((locals.var_t1_dn2 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn2)), ((locals.var_t1_dn4 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn4)), ((locals.var_t1_dn5 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn5)), ((locals.var_t1_dn6 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn6)), ((locals.var_t1_dn7 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn7)), ((locals.var_t1_dn8 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn8)), ((locals.var_t1_dn9 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn9)), ((locals.var_t1_dn10 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn10)), ((locals.var_t1_dn11 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn11)), ((locals.var_t1_dn14 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign65010_e100288;
        locals.var_ty_dn0 = assign65010_e100288_d_n0;
        locals.var_ty_dn2 = assign65010_e100288_d_n2;
        locals.var_ty_dn4 = assign65010_e100288_d_n4;
        locals.var_ty_dn5 = assign65010_e100288_d_n5;
        locals.var_ty_dn6 = assign65010_e100288_d_n6;
        locals.var_ty_dn7 = assign65010_e100288_d_n7;
        locals.var_ty_dn8 = assign65010_e100288_d_n8;
        locals.var_ty_dn9 = assign65010_e100288_d_n9;
        locals.var_ty_dn10 = assign65010_e100288_d_n10;
        locals.var_ty_dn11 = assign65010_e100288_d_n11;
        locals.var_ty_dn14 = assign65010_e100288_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign65020_e100308, assign65020_e100308_d_n0, assign65020_e100308_d_n2, assign65020_e100308_d_n4, assign65020_e100308_d_n5, assign65020_e100308_d_n6, assign65020_e100308_d_n7, assign65020_e100308_d_n8, assign65020_e100308_d_n9, assign65020_e100308_d_n10, assign65020_e100308_d_n11, assign65020_e100308_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 != 0.0)) {
        let assign65020_e100303: f64 = (3.0 * 1.414213562373095);
        let assign65020_e100305: f64 = (assign65020_e100303 * locals.var_ty);
        let assign65020_e100306: f64 = (2.0 + assign65020_e100305);
        (assign65020_e100306, (assign65020_e100303 * locals.var_ty_dn0), (assign65020_e100303 * locals.var_ty_dn2), (assign65020_e100303 * locals.var_ty_dn4), (assign65020_e100303 * locals.var_ty_dn5), (assign65020_e100303 * locals.var_ty_dn6), (assign65020_e100303 * locals.var_ty_dn7), (assign65020_e100303 * locals.var_ty_dn8), (assign65020_e100303 * locals.var_ty_dn9), (assign65020_e100303 * locals.var_ty_dn10), (assign65020_e100303 * locals.var_ty_dn11), (assign65020_e100303 * locals.var_ty_dn14),)
    } else {
        (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn4, locals.var_ac41_dn5, locals.var_ac41_dn6, locals.var_ac41_dn7, locals.var_ac41_dn8, locals.var_ac41_dn9, locals.var_ac41_dn10, locals.var_ac41_dn11, locals.var_ac41_dn14,)
    }
};
        locals.var_ac41 = assign65020_e100308;
        locals.var_ac41_dn0 = assign65020_e100308_d_n0;
        locals.var_ac41_dn2 = assign65020_e100308_d_n2;
        locals.var_ac41_dn4 = assign65020_e100308_d_n4;
        locals.var_ac41_dn5 = assign65020_e100308_d_n5;
        locals.var_ac41_dn6 = assign65020_e100308_d_n6;
        locals.var_ac41_dn7 = assign65020_e100308_d_n7;
        locals.var_ac41_dn8 = assign65020_e100308_d_n8;
        locals.var_ac41_dn9 = assign65020_e100308_d_n9;
        locals.var_ac41_dn10 = assign65020_e100308_d_n10;
        locals.var_ac41_dn11 = assign65020_e100308_d_n11;
        locals.var_ac41_dn14 = assign65020_e100308_d_n14;
        locals.var_ac41_rv = 0.0;

        let (assign65030_e100328, assign65030_e100328_d_n0, assign65030_e100328_d_n2, assign65030_e100328_d_n4, assign65030_e100328_d_n5, assign65030_e100328_d_n6, assign65030_e100328_d_n7, assign65030_e100328_d_n8, assign65030_e100328_d_n9, assign65030_e100328_d_n10, assign65030_e100328_d_n11, assign65030_e100328_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 != 0.0)) {
        let assign65030_e100322: f64 = (8.0 * locals.var_ac41);
        let assign65030_e100324: f64 = (assign65030_e100322 * locals.var_ac41);
        let assign65030_e100326: f64 = (assign65030_e100324 * locals.var_ac41);
        (assign65030_e100326, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign65030_e100322 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign65030_e100324 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign65030_e100322 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign65030_e100324 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn4) * locals.var_ac41) + (assign65030_e100322 * locals.var_ac41_dn4)) * locals.var_ac41) + (assign65030_e100324 * locals.var_ac41_dn4)), (((((8.0 * locals.var_ac41_dn5) * locals.var_ac41) + (assign65030_e100322 * locals.var_ac41_dn5)) * locals.var_ac41) + (assign65030_e100324 * locals.var_ac41_dn5)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign65030_e100322 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign65030_e100324 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn7) * locals.var_ac41) + (assign65030_e100322 * locals.var_ac41_dn7)) * locals.var_ac41) + (assign65030_e100324 * locals.var_ac41_dn7)), (((((8.0 * locals.var_ac41_dn8) * locals.var_ac41) + (assign65030_e100322 * locals.var_ac41_dn8)) * locals.var_ac41) + (assign65030_e100324 * locals.var_ac41_dn8)), (((((8.0 * locals.var_ac41_dn9) * locals.var_ac41) + (assign65030_e100322 * locals.var_ac41_dn9)) * locals.var_ac41) + (assign65030_e100324 * locals.var_ac41_dn9)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign65030_e100322 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign65030_e100324 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn11) * locals.var_ac41) + (assign65030_e100322 * locals.var_ac41_dn11)) * locals.var_ac41) + (assign65030_e100324 * locals.var_ac41_dn11)), (((((8.0 * locals.var_ac41_dn14) * locals.var_ac41) + (assign65030_e100322 * locals.var_ac41_dn14)) * locals.var_ac41) + (assign65030_e100324 * locals.var_ac41_dn14)),)
    } else {
        (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn4, locals.var_ac4_dn5, locals.var_ac4_dn6, locals.var_ac4_dn7, locals.var_ac4_dn8, locals.var_ac4_dn9, locals.var_ac4_dn10, locals.var_ac4_dn11, locals.var_ac4_dn14,)
    }
};
        locals.var_ac4 = assign65030_e100328;
        locals.var_ac4_dn0 = assign65030_e100328_d_n0;
        locals.var_ac4_dn2 = assign65030_e100328_d_n2;
        locals.var_ac4_dn4 = assign65030_e100328_d_n4;
        locals.var_ac4_dn5 = assign65030_e100328_d_n5;
        locals.var_ac4_dn6 = assign65030_e100328_d_n6;
        locals.var_ac4_dn7 = assign65030_e100328_d_n7;
        locals.var_ac4_dn8 = assign65030_e100328_d_n8;
        locals.var_ac4_dn9 = assign65030_e100328_d_n9;
        locals.var_ac4_dn10 = assign65030_e100328_d_n10;
        locals.var_ac4_dn11 = assign65030_e100328_d_n11;
        locals.var_ac4_dn14 = assign65030_e100328_d_n14;
        locals.var_ac4_rv = 0.0;

        let (assign65040_e100344, assign65040_e100344_d_n0, assign65040_e100344_d_n2, assign65040_e100344_d_n4, assign65040_e100344_d_n5, assign65040_e100344_d_n6, assign65040_e100344_d_n7, assign65040_e100344_d_n8, assign65040_e100344_d_n9, assign65040_e100344_d_n10, assign65040_e100344_d_n11, assign65040_e100344_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 != 0.0)) {
        let assign65040_e100342: f64 = (locals.var_tx - 2.0);
        (assign65040_e100342, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign65040_e100344;
        locals.var_t4_dn0 = assign65040_e100344_d_n0;
        locals.var_t4_dn2 = assign65040_e100344_d_n2;
        locals.var_t4_dn4 = assign65040_e100344_d_n4;
        locals.var_t4_dn5 = assign65040_e100344_d_n5;
        locals.var_t4_dn6 = assign65040_e100344_d_n6;
        locals.var_t4_dn7 = assign65040_e100344_d_n7;
        locals.var_t4_dn8 = assign65040_e100344_d_n8;
        locals.var_t4_dn9 = assign65040_e100344_d_n9;
        locals.var_t4_dn10 = assign65040_e100344_d_n10;
        locals.var_t4_dn11 = assign65040_e100344_d_n11;
        locals.var_t4_dn14 = assign65040_e100344_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign65050_e100362, assign65050_e100362_d_n0, assign65050_e100362_d_n2, assign65050_e100362_d_n4, assign65050_e100362_d_n5, assign65050_e100362_d_n6, assign65050_e100362_d_n7, assign65050_e100362_d_n8, assign65050_e100362_d_n9, assign65050_e100362_d_n10, assign65050_e100362_d_n11, assign65050_e100362_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 != 0.0)) {
        let assign65050_e100358: f64 = (9.0 * locals.var_ty);
        let assign65050_e100360: f64 = (assign65050_e100358 * locals.var_t4);
        (assign65050_e100360, (((9.0 * locals.var_ty_dn0) * locals.var_t4) + (assign65050_e100358 * locals.var_t4_dn0)), (((9.0 * locals.var_ty_dn2) * locals.var_t4) + (assign65050_e100358 * locals.var_t4_dn2)), (((9.0 * locals.var_ty_dn4) * locals.var_t4) + (assign65050_e100358 * locals.var_t4_dn4)), (((9.0 * locals.var_ty_dn5) * locals.var_t4) + (assign65050_e100358 * locals.var_t4_dn5)), (((9.0 * locals.var_ty_dn6) * locals.var_t4) + (assign65050_e100358 * locals.var_t4_dn6)), (((9.0 * locals.var_ty_dn7) * locals.var_t4) + (assign65050_e100358 * locals.var_t4_dn7)), (((9.0 * locals.var_ty_dn8) * locals.var_t4) + (assign65050_e100358 * locals.var_t4_dn8)), (((9.0 * locals.var_ty_dn9) * locals.var_t4) + (assign65050_e100358 * locals.var_t4_dn9)), (((9.0 * locals.var_ty_dn10) * locals.var_t4) + (assign65050_e100358 * locals.var_t4_dn10)), (((9.0 * locals.var_ty_dn11) * locals.var_t4) + (assign65050_e100358 * locals.var_t4_dn11)), (((9.0 * locals.var_ty_dn14) * locals.var_t4) + (assign65050_e100358 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign65050_e100362;
        locals.var_t5_dn0 = assign65050_e100362_d_n0;
        locals.var_t5_dn2 = assign65050_e100362_d_n2;
        locals.var_t5_dn4 = assign65050_e100362_d_n4;
        locals.var_t5_dn5 = assign65050_e100362_d_n5;
        locals.var_t5_dn6 = assign65050_e100362_d_n6;
        locals.var_t5_dn7 = assign65050_e100362_d_n7;
        locals.var_t5_dn8 = assign65050_e100362_d_n8;
        locals.var_t5_dn9 = assign65050_e100362_d_n9;
        locals.var_t5_dn10 = assign65050_e100362_d_n10;
        locals.var_t5_dn11 = assign65050_e100362_d_n11;
        locals.var_t5_dn14 = assign65050_e100362_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign65060_e100380, assign65060_e100380_d_n0, assign65060_e100380_d_n2, assign65060_e100380_d_n4, assign65060_e100380_d_n5, assign65060_e100380_d_n6, assign65060_e100380_d_n7, assign65060_e100380_d_n8, assign65060_e100380_d_n9, assign65060_e100380_d_n10, assign65060_e100380_d_n11, assign65060_e100380_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 != 0.0)) {
        let assign65060_e100376: f64 = (7.0 * 1.414213562373095);
        let assign65060_e100378: f64 = (assign65060_e100376 - locals.var_t5);
        (assign65060_e100378, (-locals.var_t5_dn0), (-locals.var_t5_dn2), (-locals.var_t5_dn4), (-locals.var_t5_dn5), (-locals.var_t5_dn6), (-locals.var_t5_dn7), (-locals.var_t5_dn8), (-locals.var_t5_dn9), (-locals.var_t5_dn10), (-locals.var_t5_dn11), (-locals.var_t5_dn14),)
    } else {
        (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn4, locals.var_ac31_dn5, locals.var_ac31_dn6, locals.var_ac31_dn7, locals.var_ac31_dn8, locals.var_ac31_dn9, locals.var_ac31_dn10, locals.var_ac31_dn11, locals.var_ac31_dn14,)
    }
};
        locals.var_ac31 = assign65060_e100380;
        locals.var_ac31_dn0 = assign65060_e100380_d_n0;
        locals.var_ac31_dn2 = assign65060_e100380_d_n2;
        locals.var_ac31_dn4 = assign65060_e100380_d_n4;
        locals.var_ac31_dn5 = assign65060_e100380_d_n5;
        locals.var_ac31_dn6 = assign65060_e100380_d_n6;
        locals.var_ac31_dn7 = assign65060_e100380_d_n7;
        locals.var_ac31_dn8 = assign65060_e100380_d_n8;
        locals.var_ac31_dn9 = assign65060_e100380_d_n9;
        locals.var_ac31_dn10 = assign65060_e100380_d_n10;
        locals.var_ac31_dn11 = assign65060_e100380_d_n11;
        locals.var_ac31_dn14 = assign65060_e100380_d_n14;
        locals.var_ac31_rv = 0.0;

        let (assign65070_e100396, assign65070_e100396_d_n0, assign65070_e100396_d_n2, assign65070_e100396_d_n4, assign65070_e100396_d_n5, assign65070_e100396_d_n6, assign65070_e100396_d_n7, assign65070_e100396_d_n8, assign65070_e100396_d_n9, assign65070_e100396_d_n10, assign65070_e100396_d_n11, assign65070_e100396_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 != 0.0)) {
        let assign65070_e100394: f64 = (locals.var_ac31 * locals.var_ac31);
        (assign65070_e100394, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn4 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn4)), ((locals.var_ac31_dn5 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn5)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn7 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn7)), ((locals.var_ac31_dn8 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn8)), ((locals.var_ac31_dn9 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn9)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn11 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn11)), ((locals.var_ac31_dn14 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn14)),)
    } else {
        (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn4, locals.var_ac3_dn5, locals.var_ac3_dn6, locals.var_ac3_dn7, locals.var_ac3_dn8, locals.var_ac3_dn9, locals.var_ac3_dn10, locals.var_ac3_dn11, locals.var_ac3_dn14,)
    }
};
        locals.var_ac3 = assign65070_e100396;
        locals.var_ac3_dn0 = assign65070_e100396_d_n0;
        locals.var_ac3_dn2 = assign65070_e100396_d_n2;
        locals.var_ac3_dn4 = assign65070_e100396_d_n4;
        locals.var_ac3_dn5 = assign65070_e100396_d_n5;
        locals.var_ac3_dn6 = assign65070_e100396_d_n6;
        locals.var_ac3_dn7 = assign65070_e100396_d_n7;
        locals.var_ac3_dn8 = assign65070_e100396_d_n8;
        locals.var_ac3_dn9 = assign65070_e100396_d_n9;
        locals.var_ac3_dn10 = assign65070_e100396_d_n10;
        locals.var_ac3_dn11 = assign65070_e100396_d_n11;
        locals.var_ac3_dn14 = assign65070_e100396_d_n14;
        locals.var_ac3_rv = 0.0;

        let assign65080_e100400: f64 = (locals.var_ac3 * 1e-8);
        let assign65080_e100401: f64 = if locals.var_ac4 < assign65080_e100400 { 1.0 } else { 0.0 };
        locals.var_guard1557 = assign65080_e100401;
        locals.var_guard1557_rv = 0.0;

        let (assign65090_e100430, assign65090_e100430_d_n0, assign65090_e100430_d_n2, assign65090_e100430_d_n4, assign65090_e100430_d_n5, assign65090_e100430_d_n6, assign65090_e100430_d_n7, assign65090_e100430_d_n8, assign65090_e100430_d_n9, assign65090_e100430_d_n10, assign65090_e100430_d_n11, assign65090_e100430_d_n14,) = {
    if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 != 0.0)) && (locals.var_guard1557 != 0.0)) {
        let assign65090_e100416: f64 = (-7.0);
        let assign65090_e100418: f64 = (assign65090_e100416 * 1.414213562373095);
        let assign65090_e100420: f64 = (assign65090_e100418 + locals.var_ac31);
        let assign65090_e100423: f64 = (0.5 * locals.var_ac4);
        let assign65090_e100425: f64 = (assign65090_e100423 / locals.var_ac31);
        let assign65090_e100426: f64 = (assign65090_e100420 + assign65090_e100425);
        let assign65090_e100428: f64 = (assign65090_e100426 + locals.var_t5);
        (assign65090_e100428, ((locals.var_ac31_dn0 + ((((0.5 * locals.var_ac4_dn0) * locals.var_ac31) - (assign65090_e100423 * locals.var_ac31_dn0)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn0), ((locals.var_ac31_dn2 + ((((0.5 * locals.var_ac4_dn2) * locals.var_ac31) - (assign65090_e100423 * locals.var_ac31_dn2)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn2), ((locals.var_ac31_dn4 + ((((0.5 * locals.var_ac4_dn4) * locals.var_ac31) - (assign65090_e100423 * locals.var_ac31_dn4)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn4), ((locals.var_ac31_dn5 + ((((0.5 * locals.var_ac4_dn5) * locals.var_ac31) - (assign65090_e100423 * locals.var_ac31_dn5)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn5), ((locals.var_ac31_dn6 + ((((0.5 * locals.var_ac4_dn6) * locals.var_ac31) - (assign65090_e100423 * locals.var_ac31_dn6)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn6), ((locals.var_ac31_dn7 + ((((0.5 * locals.var_ac4_dn7) * locals.var_ac31) - (assign65090_e100423 * locals.var_ac31_dn7)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn7), ((locals.var_ac31_dn8 + ((((0.5 * locals.var_ac4_dn8) * locals.var_ac31) - (assign65090_e100423 * locals.var_ac31_dn8)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn8), ((locals.var_ac31_dn9 + ((((0.5 * locals.var_ac4_dn9) * locals.var_ac31) - (assign65090_e100423 * locals.var_ac31_dn9)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn9), ((locals.var_ac31_dn10 + ((((0.5 * locals.var_ac4_dn10) * locals.var_ac31) - (assign65090_e100423 * locals.var_ac31_dn10)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn10), ((locals.var_ac31_dn11 + ((((0.5 * locals.var_ac4_dn11) * locals.var_ac31) - (assign65090_e100423 * locals.var_ac31_dn11)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn11), ((locals.var_ac31_dn14 + ((((0.5 * locals.var_ac4_dn14) * locals.var_ac31) - (assign65090_e100423 * locals.var_ac31_dn14)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn14),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn14,)
    }
};
        locals.var_ac1 = assign65090_e100430;
        locals.var_ac1_dn0 = assign65090_e100430_d_n0;
        locals.var_ac1_dn2 = assign65090_e100430_d_n2;
        locals.var_ac1_dn4 = assign65090_e100430_d_n4;
        locals.var_ac1_dn5 = assign65090_e100430_d_n5;
        locals.var_ac1_dn6 = assign65090_e100430_d_n6;
        locals.var_ac1_dn7 = assign65090_e100430_d_n7;
        locals.var_ac1_dn8 = assign65090_e100430_d_n8;
        locals.var_ac1_dn9 = assign65090_e100430_d_n9;
        locals.var_ac1_dn10 = assign65090_e100430_d_n10;
        locals.var_ac1_dn11 = assign65090_e100430_d_n11;
        locals.var_ac1_dn14 = assign65090_e100430_d_n14;
        locals.var_ac1_rv = 0.0;

        let (assign65100_e100450, assign65100_e100450_d_n0, assign65100_e100450_d_n2, assign65100_e100450_d_n4, assign65100_e100450_d_n5, assign65100_e100450_d_n6, assign65100_e100450_d_n7, assign65100_e100450_d_n8, assign65100_e100450_d_n9, assign65100_e100450_d_n10, assign65100_e100450_d_n11, assign65100_e100450_d_n14,) = {
    if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 != 0.0)) && (locals.var_guard1557 == 0.0)) {
        let assign65100_e100447: f64 = (locals.var_ac4 + locals.var_ac3);
        let assign65100_e100448: f64 = (assign65100_e100447).sqrt();
        (assign65100_e100448, ((locals.var_ac4_dn0 + locals.var_ac3_dn0) / (2.0 * assign65100_e100448)), ((locals.var_ac4_dn2 + locals.var_ac3_dn2) / (2.0 * assign65100_e100448)), ((locals.var_ac4_dn4 + locals.var_ac3_dn4) / (2.0 * assign65100_e100448)), ((locals.var_ac4_dn5 + locals.var_ac3_dn5) / (2.0 * assign65100_e100448)), ((locals.var_ac4_dn6 + locals.var_ac3_dn6) / (2.0 * assign65100_e100448)), ((locals.var_ac4_dn7 + locals.var_ac3_dn7) / (2.0 * assign65100_e100448)), ((locals.var_ac4_dn8 + locals.var_ac3_dn8) / (2.0 * assign65100_e100448)), ((locals.var_ac4_dn9 + locals.var_ac3_dn9) / (2.0 * assign65100_e100448)), ((locals.var_ac4_dn10 + locals.var_ac3_dn10) / (2.0 * assign65100_e100448)), ((locals.var_ac4_dn11 + locals.var_ac3_dn11) / (2.0 * assign65100_e100448)), ((locals.var_ac4_dn14 + locals.var_ac3_dn14) / (2.0 * assign65100_e100448)),)
    } else {
        (locals.var_ac2, locals.var_ac2_dn0, locals.var_ac2_dn2, locals.var_ac2_dn4, locals.var_ac2_dn5, locals.var_ac2_dn6, locals.var_ac2_dn7, locals.var_ac2_dn8, locals.var_ac2_dn9, locals.var_ac2_dn10, locals.var_ac2_dn11, locals.var_ac2_dn14,)
    }
};
        locals.var_ac2 = assign65100_e100450;
        locals.var_ac2_dn0 = assign65100_e100450_d_n0;
        locals.var_ac2_dn2 = assign65100_e100450_d_n2;
        locals.var_ac2_dn4 = assign65100_e100450_d_n4;
        locals.var_ac2_dn5 = assign65100_e100450_d_n5;
        locals.var_ac2_dn6 = assign65100_e100450_d_n6;
        locals.var_ac2_dn7 = assign65100_e100450_d_n7;
        locals.var_ac2_dn8 = assign65100_e100450_d_n8;
        locals.var_ac2_dn9 = assign65100_e100450_d_n9;
        locals.var_ac2_dn10 = assign65100_e100450_d_n10;
        locals.var_ac2_dn11 = assign65100_e100450_d_n11;
        locals.var_ac2_dn14 = assign65100_e100450_d_n14;
        locals.var_ac2_rv = 0.0;

        let (assign65110_e100474, assign65110_e100474_d_n0, assign65110_e100474_d_n2, assign65110_e100474_d_n4, assign65110_e100474_d_n5, assign65110_e100474_d_n6, assign65110_e100474_d_n7, assign65110_e100474_d_n8, assign65110_e100474_d_n9, assign65110_e100474_d_n10, assign65110_e100474_d_n11, assign65110_e100474_d_n14,) = {
    if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 != 0.0)) && (locals.var_guard1557 == 0.0)) {
        let assign65110_e100466: f64 = (-7.0);
        let assign65110_e100468: f64 = (assign65110_e100466 * 1.414213562373095);
        let assign65110_e100470: f64 = (assign65110_e100468 + locals.var_ac2);
        let assign65110_e100472: f64 = (assign65110_e100470 + locals.var_t5);
        (assign65110_e100472, (locals.var_ac2_dn0 + locals.var_t5_dn0), (locals.var_ac2_dn2 + locals.var_t5_dn2), (locals.var_ac2_dn4 + locals.var_t5_dn4), (locals.var_ac2_dn5 + locals.var_t5_dn5), (locals.var_ac2_dn6 + locals.var_t5_dn6), (locals.var_ac2_dn7 + locals.var_t5_dn7), (locals.var_ac2_dn8 + locals.var_t5_dn8), (locals.var_ac2_dn9 + locals.var_t5_dn9), (locals.var_ac2_dn10 + locals.var_t5_dn10), (locals.var_ac2_dn11 + locals.var_t5_dn11), (locals.var_ac2_dn14 + locals.var_t5_dn14),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn14,)
    }
};
        locals.var_ac1 = assign65110_e100474;
        locals.var_ac1_dn0 = assign65110_e100474_d_n0;
        locals.var_ac1_dn2 = assign65110_e100474_d_n2;
        locals.var_ac1_dn4 = assign65110_e100474_d_n4;
        locals.var_ac1_dn5 = assign65110_e100474_d_n5;
        locals.var_ac1_dn6 = assign65110_e100474_d_n6;
        locals.var_ac1_dn7 = assign65110_e100474_d_n7;
        locals.var_ac1_dn8 = assign65110_e100474_d_n8;
        locals.var_ac1_dn9 = assign65110_e100474_d_n9;
        locals.var_ac1_dn10 = assign65110_e100474_d_n10;
        locals.var_ac1_dn11 = assign65110_e100474_d_n11;
        locals.var_ac1_dn14 = assign65110_e100474_d_n14;
        locals.var_ac1_rv = 0.0;

        let (assign65120_e100495, assign65120_e100495_d_n0, assign65120_e100495_d_n2, assign65120_e100495_d_n4, assign65120_e100495_d_n5, assign65120_e100495_d_n6, assign65120_e100495_d_n7, assign65120_e100495_d_n8, assign65120_e100495_d_n9, assign65120_e100495_d_n10, assign65120_e100495_d_n11, assign65120_e100495_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 != 0.0)) {
        let (assign65120_e100493, assign65120_e100493_d_n0, assign65120_e100493_d_n2, assign65120_e100493_d_n4, assign65120_e100493_d_n5, assign65120_e100493_d_n6, assign65120_e100493_d_n7, assign65120_e100493_d_n8, assign65120_e100493_d_n9, assign65120_e100493_d_n10, assign65120_e100493_d_n11, assign65120_e100493_d_n14,) = {
            if (locals.var_ac1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign65120_e100492: f64 = (locals.var_ac1).powf(0.3333333333333333);
                (assign65120_e100492, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn0)) } } else { (assign65120_e100492 * (0.3333333333333333 * (locals.var_ac1_dn0 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn2)) } } else { (assign65120_e100492 * (0.3333333333333333 * (locals.var_ac1_dn2 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn4)) } } else { (assign65120_e100492 * (0.3333333333333333 * (locals.var_ac1_dn4 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn5)) } } else { (assign65120_e100492 * (0.3333333333333333 * (locals.var_ac1_dn5 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn6)) } } else { (assign65120_e100492 * (0.3333333333333333 * (locals.var_ac1_dn6 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn7)) } } else { (assign65120_e100492 * (0.3333333333333333 * (locals.var_ac1_dn7 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn8)) } } else { (assign65120_e100492 * (0.3333333333333333 * (locals.var_ac1_dn8 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn9)) } } else { (assign65120_e100492 * (0.3333333333333333 * (locals.var_ac1_dn9 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn10)) } } else { (assign65120_e100492 * (0.3333333333333333 * (locals.var_ac1_dn10 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn11)) } } else { (assign65120_e100492 * (0.3333333333333333 * (locals.var_ac1_dn11 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn14)) } } else { (assign65120_e100492 * (0.3333333333333333 * (locals.var_ac1_dn14 / locals.var_ac1))) },)
            }
        };
        (assign65120_e100493, assign65120_e100493_d_n0, assign65120_e100493_d_n2, assign65120_e100493_d_n4, assign65120_e100493_d_n5, assign65120_e100493_d_n6, assign65120_e100493_d_n7, assign65120_e100493_d_n8, assign65120_e100493_d_n9, assign65120_e100493_d_n10, assign65120_e100493_d_n11, assign65120_e100493_d_n14,)
    } else {
        (locals.var_acd, locals.var_acd_dn0, locals.var_acd_dn2, locals.var_acd_dn4, locals.var_acd_dn5, locals.var_acd_dn6, locals.var_acd_dn7, locals.var_acd_dn8, locals.var_acd_dn9, locals.var_acd_dn10, locals.var_acd_dn11, locals.var_acd_dn14,)
    }
};
        locals.var_acd = assign65120_e100495;
        locals.var_acd_dn0 = assign65120_e100495_d_n0;
        locals.var_acd_dn2 = assign65120_e100495_d_n2;
        locals.var_acd_dn4 = assign65120_e100495_d_n4;
        locals.var_acd_dn5 = assign65120_e100495_d_n5;
        locals.var_acd_dn6 = assign65120_e100495_d_n6;
        locals.var_acd_dn7 = assign65120_e100495_d_n7;
        locals.var_acd_dn8 = assign65120_e100495_d_n8;
        locals.var_acd_dn9 = assign65120_e100495_d_n9;
        locals.var_acd_dn10 = assign65120_e100495_d_n10;
        locals.var_acd_dn11 = assign65120_e100495_d_n11;
        locals.var_acd_dn14 = assign65120_e100495_d_n14;
        locals.var_acd_rv = 0.0;

        let (assign65130_e100526, assign65130_e100526_d_n0, assign65130_e100526_d_n2, assign65130_e100526_d_n4, assign65130_e100526_d_n5, assign65130_e100526_d_n6, assign65130_e100526_d_n7, assign65130_e100526_d_n8, assign65130_e100526_d_n9, assign65130_e100526_d_n10, assign65130_e100526_d_n11, assign65130_e100526_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 != 0.0)) {
        let assign65130_e100508: f64 = (-4.0);
        let assign65130_e100510: f64 = (assign65130_e100508 * 1.414213562373095);
        let assign65130_e100513: f64 = (12.0 * locals.var_ty);
        let assign65130_e100514: f64 = (assign65130_e100510 - assign65130_e100513);
        let assign65130_e100517: f64 = (2.0 * locals.var_acd);
        let assign65130_e100518: f64 = (assign65130_e100514 + assign65130_e100517);
        let assign65130_e100521: f64 = (1.414213562373095 * locals.var_acd);
        let assign65130_e100523: f64 = (assign65130_e100521 * locals.var_acd);
        let assign65130_e100524: f64 = (assign65130_e100518 + assign65130_e100523);
        (assign65130_e100524, (((-(12.0 * locals.var_ty_dn0)) + (2.0 * locals.var_acd_dn0)) + (((1.414213562373095 * locals.var_acd_dn0) * locals.var_acd) + (assign65130_e100521 * locals.var_acd_dn0))), (((-(12.0 * locals.var_ty_dn2)) + (2.0 * locals.var_acd_dn2)) + (((1.414213562373095 * locals.var_acd_dn2) * locals.var_acd) + (assign65130_e100521 * locals.var_acd_dn2))), (((-(12.0 * locals.var_ty_dn4)) + (2.0 * locals.var_acd_dn4)) + (((1.414213562373095 * locals.var_acd_dn4) * locals.var_acd) + (assign65130_e100521 * locals.var_acd_dn4))), (((-(12.0 * locals.var_ty_dn5)) + (2.0 * locals.var_acd_dn5)) + (((1.414213562373095 * locals.var_acd_dn5) * locals.var_acd) + (assign65130_e100521 * locals.var_acd_dn5))), (((-(12.0 * locals.var_ty_dn6)) + (2.0 * locals.var_acd_dn6)) + (((1.414213562373095 * locals.var_acd_dn6) * locals.var_acd) + (assign65130_e100521 * locals.var_acd_dn6))), (((-(12.0 * locals.var_ty_dn7)) + (2.0 * locals.var_acd_dn7)) + (((1.414213562373095 * locals.var_acd_dn7) * locals.var_acd) + (assign65130_e100521 * locals.var_acd_dn7))), (((-(12.0 * locals.var_ty_dn8)) + (2.0 * locals.var_acd_dn8)) + (((1.414213562373095 * locals.var_acd_dn8) * locals.var_acd) + (assign65130_e100521 * locals.var_acd_dn8))), (((-(12.0 * locals.var_ty_dn9)) + (2.0 * locals.var_acd_dn9)) + (((1.414213562373095 * locals.var_acd_dn9) * locals.var_acd) + (assign65130_e100521 * locals.var_acd_dn9))), (((-(12.0 * locals.var_ty_dn10)) + (2.0 * locals.var_acd_dn10)) + (((1.414213562373095 * locals.var_acd_dn10) * locals.var_acd) + (assign65130_e100521 * locals.var_acd_dn10))), (((-(12.0 * locals.var_ty_dn11)) + (2.0 * locals.var_acd_dn11)) + (((1.414213562373095 * locals.var_acd_dn11) * locals.var_acd) + (assign65130_e100521 * locals.var_acd_dn11))), (((-(12.0 * locals.var_ty_dn14)) + (2.0 * locals.var_acd_dn14)) + (((1.414213562373095 * locals.var_acd_dn14) * locals.var_acd) + (assign65130_e100521 * locals.var_acd_dn14))),)
    } else {
        (locals.var_acn, locals.var_acn_dn0, locals.var_acn_dn2, locals.var_acn_dn4, locals.var_acn_dn5, locals.var_acn_dn6, locals.var_acn_dn7, locals.var_acn_dn8, locals.var_acn_dn9, locals.var_acn_dn10, locals.var_acn_dn11, locals.var_acn_dn14,)
    }
};
        locals.var_acn = assign65130_e100526;
        locals.var_acn_dn0 = assign65130_e100526_d_n0;
        locals.var_acn_dn2 = assign65130_e100526_d_n2;
        locals.var_acn_dn4 = assign65130_e100526_d_n4;
        locals.var_acn_dn5 = assign65130_e100526_d_n5;
        locals.var_acn_dn6 = assign65130_e100526_d_n6;
        locals.var_acn_dn7 = assign65130_e100526_d_n7;
        locals.var_acn_dn8 = assign65130_e100526_d_n8;
        locals.var_acn_dn9 = assign65130_e100526_d_n9;
        locals.var_acn_dn10 = assign65130_e100526_d_n10;
        locals.var_acn_dn11 = assign65130_e100526_d_n11;
        locals.var_acn_dn14 = assign65130_e100526_d_n14;
        locals.var_acn_rv = 0.0;

        let (assign65140_e100542, assign65140_e100542_d_n0, assign65140_e100542_d_n2, assign65140_e100542_d_n4, assign65140_e100542_d_n5, assign65140_e100542_d_n6, assign65140_e100542_d_n7, assign65140_e100542_d_n8, assign65140_e100542_d_n9, assign65140_e100542_d_n10, assign65140_e100542_d_n11, assign65140_e100542_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 != 0.0)) {
        let assign65140_e100540: f64 = (1.0 / locals.var_acd);
        (assign65140_e100540, (-(locals.var_acd_dn0 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn2 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn4 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn5 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn6 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn7 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn8 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn9 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn10 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn11 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn14 / (locals.var_acd * locals.var_acd))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign65140_e100542;
        locals.var_t1_dn0 = assign65140_e100542_d_n0;
        locals.var_t1_dn2 = assign65140_e100542_d_n2;
        locals.var_t1_dn4 = assign65140_e100542_d_n4;
        locals.var_t1_dn5 = assign65140_e100542_d_n5;
        locals.var_t1_dn6 = assign65140_e100542_d_n6;
        locals.var_t1_dn7 = assign65140_e100542_d_n7;
        locals.var_t1_dn8 = assign65140_e100542_d_n8;
        locals.var_t1_dn9 = assign65140_e100542_d_n9;
        locals.var_t1_dn10 = assign65140_e100542_d_n10;
        locals.var_t1_dn11 = assign65140_e100542_d_n11;
        locals.var_t1_dn14 = assign65140_e100542_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign65150_e100558, assign65150_e100558_d_n0, assign65150_e100558_d_n2, assign65150_e100558_d_n4, assign65150_e100558_d_n5, assign65150_e100558_d_n6, assign65150_e100558_d_n7, assign65150_e100558_d_n8, assign65150_e100558_d_n9, assign65150_e100558_d_n10, assign65150_e100558_d_n11, assign65150_e100558_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 != 0.0)) {
        let assign65150_e100556: f64 = (locals.var_acn * locals.var_t1);
        (assign65150_e100556, ((locals.var_acn_dn0 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn0)), ((locals.var_acn_dn2 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn2)), ((locals.var_acn_dn4 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn4)), ((locals.var_acn_dn5 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn5)), ((locals.var_acn_dn6 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn6)), ((locals.var_acn_dn7 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn7)), ((locals.var_acn_dn8 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn8)), ((locals.var_acn_dn9 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn9)), ((locals.var_acn_dn10 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn10)), ((locals.var_acn_dn11 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn11)), ((locals.var_acn_dn14 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn14)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign65150_e100558;
        locals.var_chi_dn0 = assign65150_e100558_d_n0;
        locals.var_chi_dn2 = assign65150_e100558_d_n2;
        locals.var_chi_dn4 = assign65150_e100558_d_n4;
        locals.var_chi_dn5 = assign65150_e100558_d_n5;
        locals.var_chi_dn6 = assign65150_e100558_d_n6;
        locals.var_chi_dn7 = assign65150_e100558_d_n7;
        locals.var_chi_dn8 = assign65150_e100558_d_n8;
        locals.var_chi_dn9 = assign65150_e100558_d_n9;
        locals.var_chi_dn10 = assign65150_e100558_d_n10;
        locals.var_chi_dn11 = assign65150_e100558_d_n11;
        locals.var_chi_dn14 = assign65150_e100558_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign65160_e100576, assign65160_e100576_d_n0, assign65160_e100576_d_n2, assign65160_e100576_d_n4, assign65160_e100576_d_n5, assign65160_e100576_d_n6, assign65160_e100576_d_n7, assign65160_e100576_d_n8, assign65160_e100576_d_n9, assign65160_e100576_d_n10, assign65160_e100576_d_n11, assign65160_e100576_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 != 0.0)) {
        let assign65160_e100572: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign65160_e100574: f64 = (assign65160_e100572 + locals.var_vbscl__blk1547);
        (assign65160_e100574, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) + locals.var_vbscl__blk1547_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) + locals.var_vbscl__blk1547_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) + locals.var_vbscl__blk1547_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) + locals.var_vbscl__blk1547_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) + locals.var_vbscl__blk1547_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) + locals.var_vbscl__blk1547_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) + locals.var_vbscl__blk1547_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) + locals.var_vbscl__blk1547_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) + locals.var_vbscl__blk1547_dn10), (((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)) + locals.var_vbscl__blk1547_dn11), (((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)) + locals.var_vbscl__blk1547_dn14),)
    } else {
        (locals.var_psa, locals.var_psa_dn0, locals.var_psa_dn2, locals.var_psa_dn4, locals.var_psa_dn5, locals.var_psa_dn6, locals.var_psa_dn7, locals.var_psa_dn8, locals.var_psa_dn9, locals.var_psa_dn10, locals.var_psa_dn11, locals.var_psa_dn14,)
    }
};
        locals.var_psa = assign65160_e100576;
        locals.var_psa_dn0 = assign65160_e100576_d_n0;
        locals.var_psa_dn2 = assign65160_e100576_d_n2;
        locals.var_psa_dn4 = assign65160_e100576_d_n4;
        locals.var_psa_dn5 = assign65160_e100576_d_n5;
        locals.var_psa_dn6 = assign65160_e100576_d_n6;
        locals.var_psa_dn7 = assign65160_e100576_d_n7;
        locals.var_psa_dn8 = assign65160_e100576_d_n8;
        locals.var_psa_dn9 = assign65160_e100576_d_n9;
        locals.var_psa_dn10 = assign65160_e100576_d_n10;
        locals.var_psa_dn11 = assign65160_e100576_d_n11;
        locals.var_psa_dn14 = assign65160_e100576_d_n14;
        locals.var_psa_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_244(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign65170_e100592, assign65170_e100592_d_n0, assign65170_e100592_d_n2, assign65170_e100592_d_n4, assign65170_e100592_d_n5, assign65170_e100592_d_n6, assign65170_e100592_d_n7, assign65170_e100592_d_n8, assign65170_e100592_d_n9, assign65170_e100592_d_n10, assign65170_e100592_d_n11, assign65170_e100592_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 != 0.0)) {
        let assign65170_e100590: f64 = (locals.var_psa - locals.var_vbscl__blk1547);
        (assign65170_e100590, (locals.var_psa_dn0 - locals.var_vbscl__blk1547_dn0), (locals.var_psa_dn2 - locals.var_vbscl__blk1547_dn2), (locals.var_psa_dn4 - locals.var_vbscl__blk1547_dn4), (locals.var_psa_dn5 - locals.var_vbscl__blk1547_dn5), (locals.var_psa_dn6 - locals.var_vbscl__blk1547_dn6), (locals.var_psa_dn7 - locals.var_vbscl__blk1547_dn7), (locals.var_psa_dn8 - locals.var_vbscl__blk1547_dn8), (locals.var_psa_dn9 - locals.var_vbscl__blk1547_dn9), (locals.var_psa_dn10 - locals.var_vbscl__blk1547_dn10), (locals.var_psa_dn11 - locals.var_vbscl__blk1547_dn11), (locals.var_psa_dn14 - locals.var_vbscl__blk1547_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign65170_e100592;
        locals.var_t1_dn0 = assign65170_e100592_d_n0;
        locals.var_t1_dn2 = assign65170_e100592_d_n2;
        locals.var_t1_dn4 = assign65170_e100592_d_n4;
        locals.var_t1_dn5 = assign65170_e100592_d_n5;
        locals.var_t1_dn6 = assign65170_e100592_d_n6;
        locals.var_t1_dn7 = assign65170_e100592_d_n7;
        locals.var_t1_dn8 = assign65170_e100592_d_n8;
        locals.var_t1_dn9 = assign65170_e100592_d_n9;
        locals.var_t1_dn10 = assign65170_e100592_d_n10;
        locals.var_t1_dn11 = assign65170_e100592_d_n11;
        locals.var_t1_dn14 = assign65170_e100592_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign65180_e100608, assign65180_e100608_d_n0, assign65180_e100608_d_n2, assign65180_e100608_d_n4, assign65180_e100608_d_n5, assign65180_e100608_d_n6, assign65180_e100608_d_n7, assign65180_e100608_d_n8, assign65180_e100608_d_n9, assign65180_e100608_d_n10, assign65180_e100608_d_n11, assign65180_e100608_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 != 0.0)) {
        let assign65180_e100606: f64 = (locals.var_t1 / locals.var_ps0_min);
        (assign65180_e100606, (((locals.var_t1_dn0 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn0)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn2 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn2)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn4 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn4)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn5 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn5)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn6 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn6)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn7 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn7)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn8 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn8)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn9 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn9)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn10 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn10)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn11 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn11)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn14 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn14)) / (locals.var_ps0_min * locals.var_ps0_min)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign65180_e100608;
        locals.var_t2_dn0 = assign65180_e100608_d_n0;
        locals.var_t2_dn2 = assign65180_e100608_d_n2;
        locals.var_t2_dn4 = assign65180_e100608_d_n4;
        locals.var_t2_dn5 = assign65180_e100608_d_n5;
        locals.var_t2_dn6 = assign65180_e100608_d_n6;
        locals.var_t2_dn7 = assign65180_e100608_d_n7;
        locals.var_t2_dn8 = assign65180_e100608_d_n8;
        locals.var_t2_dn9 = assign65180_e100608_d_n9;
        locals.var_t2_dn10 = assign65180_e100608_d_n10;
        locals.var_t2_dn11 = assign65180_e100608_d_n11;
        locals.var_t2_dn14 = assign65180_e100608_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign65190_e100627, assign65190_e100627_d_n0, assign65190_e100627_d_n2, assign65190_e100627_d_n4, assign65190_e100627_d_n5, assign65190_e100627_d_n6, assign65190_e100627_d_n7, assign65190_e100627_d_n8, assign65190_e100627_d_n9, assign65190_e100627_d_n10, assign65190_e100627_d_n11, assign65190_e100627_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 != 0.0)) {
        let assign65190_e100623: f64 = (locals.var_t2 * locals.var_t2);
        let assign65190_e100624: f64 = (1.0 + assign65190_e100623);
        let assign65190_e100625: f64 = (assign65190_e100624).sqrt();
        (assign65190_e100625, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign65190_e100625)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign65190_e100625)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign65190_e100625)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign65190_e100625)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign65190_e100625)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign65190_e100625)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign65190_e100625)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign65190_e100625)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign65190_e100625)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign65190_e100625)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign65190_e100625)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign65190_e100627;
        locals.var_t3_dn0 = assign65190_e100627_d_n0;
        locals.var_t3_dn2 = assign65190_e100627_d_n2;
        locals.var_t3_dn4 = assign65190_e100627_d_n4;
        locals.var_t3_dn5 = assign65190_e100627_d_n5;
        locals.var_t3_dn6 = assign65190_e100627_d_n6;
        locals.var_t3_dn7 = assign65190_e100627_d_n7;
        locals.var_t3_dn8 = assign65190_e100627_d_n8;
        locals.var_t3_dn9 = assign65190_e100627_d_n9;
        locals.var_t3_dn10 = assign65190_e100627_d_n10;
        locals.var_t3_dn11 = assign65190_e100627_d_n11;
        locals.var_t3_dn14 = assign65190_e100627_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign65200_e100645, assign65200_e100645_d_n0, assign65200_e100645_d_n2, assign65200_e100645_d_n4, assign65200_e100645_d_n5, assign65200_e100645_d_n6, assign65200_e100645_d_n7, assign65200_e100645_d_n8, assign65200_e100645_d_n9, assign65200_e100645_d_n10, assign65200_e100645_d_n11, assign65200_e100645_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 != 0.0)) {
        let assign65200_e100641: f64 = (locals.var_t1 / locals.var_t3);
        let assign65200_e100643: f64 = (assign65200_e100641 + locals.var_vbscl__blk1547);
        (assign65200_e100643, ((((locals.var_t1_dn0 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1547_dn0), ((((locals.var_t1_dn2 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1547_dn2), ((((locals.var_t1_dn4 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1547_dn4), ((((locals.var_t1_dn5 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1547_dn5), ((((locals.var_t1_dn6 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1547_dn6), ((((locals.var_t1_dn7 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1547_dn7), ((((locals.var_t1_dn8 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1547_dn8), ((((locals.var_t1_dn9 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1547_dn9), ((((locals.var_t1_dn10 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1547_dn10), ((((locals.var_t1_dn11 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1547_dn11), ((((locals.var_t1_dn14 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn14)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbscl__blk1547_dn14),)
    } else {
        (locals.var_ps0__blk1525, locals.var_ps0__blk1525_dn0, locals.var_ps0__blk1525_dn2, locals.var_ps0__blk1525_dn4, locals.var_ps0__blk1525_dn5, locals.var_ps0__blk1525_dn6, locals.var_ps0__blk1525_dn7, locals.var_ps0__blk1525_dn8, locals.var_ps0__blk1525_dn9, locals.var_ps0__blk1525_dn10, locals.var_ps0__blk1525_dn11, locals.var_ps0__blk1525_dn14,)
    }
};
        locals.var_ps0__blk1525 = assign65200_e100645;
        locals.var_ps0__blk1525_dn0 = assign65200_e100645_d_n0;
        locals.var_ps0__blk1525_dn2 = assign65200_e100645_d_n2;
        locals.var_ps0__blk1525_dn4 = assign65200_e100645_d_n4;
        locals.var_ps0__blk1525_dn5 = assign65200_e100645_d_n5;
        locals.var_ps0__blk1525_dn6 = assign65200_e100645_d_n6;
        locals.var_ps0__blk1525_dn7 = assign65200_e100645_d_n7;
        locals.var_ps0__blk1525_dn8 = assign65200_e100645_d_n8;
        locals.var_ps0__blk1525_dn9 = assign65200_e100645_d_n9;
        locals.var_ps0__blk1525_dn10 = assign65200_e100645_d_n10;
        locals.var_ps0__blk1525_dn11 = assign65200_e100645_d_n11;
        locals.var_ps0__blk1525_dn14 = assign65200_e100645_d_n14;
        locals.var_ps0__blk1525_rv = 0.0;

        let (assign65210_e100665, assign65210_e100665_d_n0, assign65210_e100665_d_n2, assign65210_e100665_d_n4, assign65210_e100665_d_n5, assign65210_e100665_d_n6, assign65210_e100665_d_n7, assign65210_e100665_d_n8, assign65210_e100665_d_n9, assign65210_e100665_d_n10, assign65210_e100665_d_n11, assign65210_e100665_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) {
        let assign65210_e100661: f64 = (locals.var_vbscl__blk1547 - p.p456);
        let assign65210_e100662: f64 = (locals.var_beta * assign65210_e100661);
        let assign65210_e100663: f64 = (assign65210_e100662).exp();
        (assign65210_e100663, (assign65210_e100663 * ((locals.var_beta_dn0 * assign65210_e100661) + (locals.var_beta * locals.var_vbscl__blk1547_dn0))), (assign65210_e100663 * ((locals.var_beta_dn2 * assign65210_e100661) + (locals.var_beta * locals.var_vbscl__blk1547_dn2))), (assign65210_e100663 * ((locals.var_beta_dn4 * assign65210_e100661) + (locals.var_beta * locals.var_vbscl__blk1547_dn4))), (assign65210_e100663 * ((locals.var_beta_dn5 * assign65210_e100661) + (locals.var_beta * locals.var_vbscl__blk1547_dn5))), (assign65210_e100663 * ((locals.var_beta_dn6 * assign65210_e100661) + (locals.var_beta * locals.var_vbscl__blk1547_dn6))), (assign65210_e100663 * ((locals.var_beta_dn7 * assign65210_e100661) + (locals.var_beta * locals.var_vbscl__blk1547_dn7))), (assign65210_e100663 * ((locals.var_beta_dn8 * assign65210_e100661) + (locals.var_beta * locals.var_vbscl__blk1547_dn8))), (assign65210_e100663 * ((locals.var_beta_dn9 * assign65210_e100661) + (locals.var_beta * locals.var_vbscl__blk1547_dn9))), (assign65210_e100663 * ((locals.var_beta_dn10 * assign65210_e100661) + (locals.var_beta * locals.var_vbscl__blk1547_dn10))), (assign65210_e100663 * ((locals.var_beta_dn11 * assign65210_e100661) + (locals.var_beta * locals.var_vbscl__blk1547_dn11))), (assign65210_e100663 * ((locals.var_beta_dn14 * assign65210_e100661) + (locals.var_beta * locals.var_vbscl__blk1547_dn14))),)
    } else {
        (locals.var_exp_bvbsvds, locals.var_exp_bvbsvds_dn0, locals.var_exp_bvbsvds_dn2, locals.var_exp_bvbsvds_dn4, locals.var_exp_bvbsvds_dn5, locals.var_exp_bvbsvds_dn6, locals.var_exp_bvbsvds_dn7, locals.var_exp_bvbsvds_dn8, locals.var_exp_bvbsvds_dn9, locals.var_exp_bvbsvds_dn10, locals.var_exp_bvbsvds_dn11, locals.var_exp_bvbsvds_dn14,)
    }
};
        locals.var_exp_bvbsvds = assign65210_e100665;
        locals.var_exp_bvbsvds_dn0 = assign65210_e100665_d_n0;
        locals.var_exp_bvbsvds_dn2 = assign65210_e100665_d_n2;
        locals.var_exp_bvbsvds_dn4 = assign65210_e100665_d_n4;
        locals.var_exp_bvbsvds_dn5 = assign65210_e100665_d_n5;
        locals.var_exp_bvbsvds_dn6 = assign65210_e100665_d_n6;
        locals.var_exp_bvbsvds_dn7 = assign65210_e100665_d_n7;
        locals.var_exp_bvbsvds_dn8 = assign65210_e100665_d_n8;
        locals.var_exp_bvbsvds_dn9 = assign65210_e100665_d_n9;
        locals.var_exp_bvbsvds_dn10 = assign65210_e100665_d_n10;
        locals.var_exp_bvbsvds_dn11 = assign65210_e100665_d_n11;
        locals.var_exp_bvbsvds_dn14 = assign65210_e100665_d_n14;
        locals.var_exp_bvbsvds_rv = 0.0;

        let (assign65220_e100680,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign65220_e100680;
        locals.var_flg_conv_rv = 0.0;

        let (assign65230_e100695, assign65230_e100695_d_n0, assign65230_e100695_d_n2, assign65230_e100695_d_n4, assign65230_e100695_d_n5, assign65230_e100695_d_n6, assign65230_e100695_d_n7, assign65230_e100695_d_n8, assign65230_e100695_d_n9, assign65230_e100695_d_n10, assign65230_e100695_d_n11, assign65230_e100695_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn4, locals.var_ps0_ini_dn5, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn8, locals.var_ps0_ini_dn9, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn14,)
    } else {
        (locals.var_phi_s0, locals.var_phi_s0_dn0, locals.var_phi_s0_dn2, locals.var_phi_s0_dn4, locals.var_phi_s0_dn5, locals.var_phi_s0_dn6, locals.var_phi_s0_dn7, locals.var_phi_s0_dn8, locals.var_phi_s0_dn9, locals.var_phi_s0_dn10, locals.var_phi_s0_dn11, locals.var_phi_s0_dn14,)
    }
};
        locals.var_phi_s0 = assign65230_e100695;
        locals.var_phi_s0_dn0 = assign65230_e100695_d_n0;
        locals.var_phi_s0_dn2 = assign65230_e100695_d_n2;
        locals.var_phi_s0_dn4 = assign65230_e100695_d_n4;
        locals.var_phi_s0_dn5 = assign65230_e100695_d_n5;
        locals.var_phi_s0_dn6 = assign65230_e100695_d_n6;
        locals.var_phi_s0_dn7 = assign65230_e100695_d_n7;
        locals.var_phi_s0_dn8 = assign65230_e100695_d_n8;
        locals.var_phi_s0_dn9 = assign65230_e100695_d_n9;
        locals.var_phi_s0_dn10 = assign65230_e100695_d_n10;
        locals.var_phi_s0_dn11 = assign65230_e100695_d_n11;
        locals.var_phi_s0_dn14 = assign65230_e100695_d_n14;
        locals.var_phi_s0_rv = 0.0;

        let (assign65240_e100718, assign65240_e100718_d_n0, assign65240_e100718_d_n2, assign65240_e100718_d_n4, assign65240_e100718_d_n5, assign65240_e100718_d_n6, assign65240_e100718_d_n7, assign65240_e100718_d_n8, assign65240_e100718_d_n9, assign65240_e100718_d_n10, assign65240_e100718_d_n11, assign65240_e100718_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) {
        let assign65240_e100710: f64 = (locals.var_q_nsub * locals.var_t_sub);
        let assign65240_e100712: f64 = (assign65240_e100710 * locals.var_t_sub);
        let assign65240_e100714: f64 = (assign65240_e100712 / 2.0);
        let assign65240_e100716: f64 = (assign65240_e100714 / 1.034943e-10);
        (assign65240_e100716, ((((locals.var_q_nsub_dn0 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn2 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn4 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn5 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn6 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn7 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn8 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn9 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn10 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn11 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10), ((((locals.var_q_nsub_dn14 * locals.var_t_sub) * locals.var_t_sub) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb__blk1549, locals.var_dphi_sb__blk1549_dn0, locals.var_dphi_sb__blk1549_dn2, locals.var_dphi_sb__blk1549_dn4, locals.var_dphi_sb__blk1549_dn5, locals.var_dphi_sb__blk1549_dn6, locals.var_dphi_sb__blk1549_dn7, locals.var_dphi_sb__blk1549_dn8, locals.var_dphi_sb__blk1549_dn9, locals.var_dphi_sb__blk1549_dn10, locals.var_dphi_sb__blk1549_dn11, locals.var_dphi_sb__blk1549_dn14,)
    }
};
        locals.var_dphi_sb__blk1549 = assign65240_e100718;
        locals.var_dphi_sb__blk1549_dn0 = assign65240_e100718_d_n0;
        locals.var_dphi_sb__blk1549_dn2 = assign65240_e100718_d_n2;
        locals.var_dphi_sb__blk1549_dn4 = assign65240_e100718_d_n4;
        locals.var_dphi_sb__blk1549_dn5 = assign65240_e100718_d_n5;
        locals.var_dphi_sb__blk1549_dn6 = assign65240_e100718_d_n6;
        locals.var_dphi_sb__blk1549_dn7 = assign65240_e100718_d_n7;
        locals.var_dphi_sb__blk1549_dn8 = assign65240_e100718_d_n8;
        locals.var_dphi_sb__blk1549_dn9 = assign65240_e100718_d_n9;
        locals.var_dphi_sb__blk1549_dn10 = assign65240_e100718_d_n10;
        locals.var_dphi_sb__blk1549_dn11 = assign65240_e100718_d_n11;
        locals.var_dphi_sb__blk1549_dn14 = assign65240_e100718_d_n14;
        locals.var_dphi_sb__blk1549_rv = 0.0;

        let (assign65250_e100738, assign65250_e100738_d_n0, assign65250_e100738_d_n2, assign65250_e100738_d_n4, assign65250_e100738_d_n5, assign65250_e100738_d_n6, assign65250_e100738_d_n7, assign65250_e100738_d_n8, assign65250_e100738_d_n9, assign65250_e100738_d_n10, assign65250_e100738_d_n11, assign65250_e100738_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) {
        let assign65250_e100733: f64 = (2.0 * locals.var_beta);
        let assign65250_e100735: f64 = (assign65250_e100733 * locals.var_dphi_sb__blk1549);
        let assign65250_e100736: f64 = (assign65250_e100735).sqrt();
        (assign65250_e100736, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb__blk1549) + (assign65250_e100733 * locals.var_dphi_sb__blk1549_dn0)) / (2.0 * assign65250_e100736)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb__blk1549) + (assign65250_e100733 * locals.var_dphi_sb__blk1549_dn2)) / (2.0 * assign65250_e100736)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb__blk1549) + (assign65250_e100733 * locals.var_dphi_sb__blk1549_dn4)) / (2.0 * assign65250_e100736)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb__blk1549) + (assign65250_e100733 * locals.var_dphi_sb__blk1549_dn5)) / (2.0 * assign65250_e100736)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb__blk1549) + (assign65250_e100733 * locals.var_dphi_sb__blk1549_dn6)) / (2.0 * assign65250_e100736)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb__blk1549) + (assign65250_e100733 * locals.var_dphi_sb__blk1549_dn7)) / (2.0 * assign65250_e100736)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb__blk1549) + (assign65250_e100733 * locals.var_dphi_sb__blk1549_dn8)) / (2.0 * assign65250_e100736)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb__blk1549) + (assign65250_e100733 * locals.var_dphi_sb__blk1549_dn9)) / (2.0 * assign65250_e100736)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb__blk1549) + (assign65250_e100733 * locals.var_dphi_sb__blk1549_dn10)) / (2.0 * assign65250_e100736)), ((((2.0 * locals.var_beta_dn11) * locals.var_dphi_sb__blk1549) + (assign65250_e100733 * locals.var_dphi_sb__blk1549_dn11)) / (2.0 * assign65250_e100736)), ((((2.0 * locals.var_beta_dn14) * locals.var_dphi_sb__blk1549) + (assign65250_e100733 * locals.var_dphi_sb__blk1549_dn14)) / (2.0 * assign65250_e100736)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign65250_e100738;
        locals.var_t0_dn0 = assign65250_e100738_d_n0;
        locals.var_t0_dn2 = assign65250_e100738_d_n2;
        locals.var_t0_dn4 = assign65250_e100738_d_n4;
        locals.var_t0_dn5 = assign65250_e100738_d_n5;
        locals.var_t0_dn6 = assign65250_e100738_d_n6;
        locals.var_t0_dn7 = assign65250_e100738_d_n7;
        locals.var_t0_dn8 = assign65250_e100738_d_n8;
        locals.var_t0_dn9 = assign65250_e100738_d_n9;
        locals.var_t0_dn10 = assign65250_e100738_d_n10;
        locals.var_t0_dn11 = assign65250_e100738_d_n11;
        locals.var_t0_dn14 = assign65250_e100738_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign65260_e100760, assign65260_e100760_d_n0, assign65260_e100760_d_n2, assign65260_e100760_d_n4, assign65260_e100760_d_n5, assign65260_e100760_d_n6, assign65260_e100760_d_n7, assign65260_e100760_d_n8, assign65260_e100760_d_n9, assign65260_e100760_d_n10, assign65260_e100760_d_n11, assign65260_e100760_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) {
        let assign65260_e100752: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign65260_e100754: f64 = (-locals.var_t0);
        let assign65260_e100755: f64 = { let limited_exp_arg = assign65260_e100754; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign65260_e100756: f64 = (assign65260_e100752 + assign65260_e100755);
        let assign65260_e100758: f64 = (assign65260_e100756 / 2.0);
        (assign65260_e100758, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign65260_e100754; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign65260_e100754; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign65260_e100754; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign65260_e100754; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign65260_e100754; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign65260_e100754; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign65260_e100754; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign65260_e100754; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign65260_e100754; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11) + ({ let limited_exp_arg = assign65260_e100754; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn11))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn14) + ({ let limited_exp_arg = assign65260_e100754; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn14))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign65260_e100760;
        locals.var_t1_dn0 = assign65260_e100760_d_n0;
        locals.var_t1_dn2 = assign65260_e100760_d_n2;
        locals.var_t1_dn4 = assign65260_e100760_d_n4;
        locals.var_t1_dn5 = assign65260_e100760_d_n5;
        locals.var_t1_dn6 = assign65260_e100760_d_n6;
        locals.var_t1_dn7 = assign65260_e100760_d_n7;
        locals.var_t1_dn8 = assign65260_e100760_d_n8;
        locals.var_t1_dn9 = assign65260_e100760_d_n9;
        locals.var_t1_dn10 = assign65260_e100760_d_n10;
        locals.var_t1_dn11 = assign65260_e100760_d_n11;
        locals.var_t1_dn14 = assign65260_e100760_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign65270_e100778, assign65270_e100778_d_n0, assign65270_e100778_d_n2, assign65270_e100778_d_n4, assign65270_e100778_d_n5, assign65270_e100778_d_n6, assign65270_e100778_d_n7, assign65270_e100778_d_n8, assign65270_e100778_d_n9, assign65270_e100778_d_n10, assign65270_e100778_d_n11, assign65270_e100778_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) {
        let assign65270_e100774: f64 = (locals.var_t1).ln();
        let assign65270_e100776: f64 = (assign65270_e100774 / locals.var_dphi_sb__blk1549);
        (assign65270_e100776, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb__blk1549) - (assign65270_e100774 * locals.var_dphi_sb__blk1549_dn0)) / (locals.var_dphi_sb__blk1549 * locals.var_dphi_sb__blk1549)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb__blk1549) - (assign65270_e100774 * locals.var_dphi_sb__blk1549_dn2)) / (locals.var_dphi_sb__blk1549 * locals.var_dphi_sb__blk1549)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb__blk1549) - (assign65270_e100774 * locals.var_dphi_sb__blk1549_dn4)) / (locals.var_dphi_sb__blk1549 * locals.var_dphi_sb__blk1549)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb__blk1549) - (assign65270_e100774 * locals.var_dphi_sb__blk1549_dn5)) / (locals.var_dphi_sb__blk1549 * locals.var_dphi_sb__blk1549)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb__blk1549) - (assign65270_e100774 * locals.var_dphi_sb__blk1549_dn6)) / (locals.var_dphi_sb__blk1549 * locals.var_dphi_sb__blk1549)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb__blk1549) - (assign65270_e100774 * locals.var_dphi_sb__blk1549_dn7)) / (locals.var_dphi_sb__blk1549 * locals.var_dphi_sb__blk1549)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb__blk1549) - (assign65270_e100774 * locals.var_dphi_sb__blk1549_dn8)) / (locals.var_dphi_sb__blk1549 * locals.var_dphi_sb__blk1549)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb__blk1549) - (assign65270_e100774 * locals.var_dphi_sb__blk1549_dn9)) / (locals.var_dphi_sb__blk1549 * locals.var_dphi_sb__blk1549)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb__blk1549) - (assign65270_e100774 * locals.var_dphi_sb__blk1549_dn10)) / (locals.var_dphi_sb__blk1549 * locals.var_dphi_sb__blk1549)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb__blk1549) - (assign65270_e100774 * locals.var_dphi_sb__blk1549_dn11)) / (locals.var_dphi_sb__blk1549 * locals.var_dphi_sb__blk1549)), ((((locals.var_t1_dn14 / locals.var_t1) * locals.var_dphi_sb__blk1549) - (assign65270_e100774 * locals.var_dphi_sb__blk1549_dn14)) / (locals.var_dphi_sb__blk1549 * locals.var_dphi_sb__blk1549)),)
    } else {
        (locals.var_c_sb__blk1550, locals.var_c_sb__blk1550_dn0, locals.var_c_sb__blk1550_dn2, locals.var_c_sb__blk1550_dn4, locals.var_c_sb__blk1550_dn5, locals.var_c_sb__blk1550_dn6, locals.var_c_sb__blk1550_dn7, locals.var_c_sb__blk1550_dn8, locals.var_c_sb__blk1550_dn9, locals.var_c_sb__blk1550_dn10, locals.var_c_sb__blk1550_dn11, locals.var_c_sb__blk1550_dn14,)
    }
};
        locals.var_c_sb__blk1550 = assign65270_e100778;
        locals.var_c_sb__blk1550_dn0 = assign65270_e100778_d_n0;
        locals.var_c_sb__blk1550_dn2 = assign65270_e100778_d_n2;
        locals.var_c_sb__blk1550_dn4 = assign65270_e100778_d_n4;
        locals.var_c_sb__blk1550_dn5 = assign65270_e100778_d_n5;
        locals.var_c_sb__blk1550_dn6 = assign65270_e100778_d_n6;
        locals.var_c_sb__blk1550_dn7 = assign65270_e100778_d_n7;
        locals.var_c_sb__blk1550_dn8 = assign65270_e100778_d_n8;
        locals.var_c_sb__blk1550_dn9 = assign65270_e100778_d_n9;
        locals.var_c_sb__blk1550_dn10 = assign65270_e100778_d_n10;
        locals.var_c_sb__blk1550_dn11 = assign65270_e100778_d_n11;
        locals.var_c_sb__blk1550_dn14 = assign65270_e100778_d_n14;
        locals.var_c_sb__blk1550_rv = 0.0;

        let (assign65280_e100793,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign65280_e100793;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_245(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign65290_loop_guard: usize = 0;
        while {
            let assign65290_cond_e100809: f64 = (locals.var_lp_s0_max + 1.0);
            let assign65290_cond_e100811: f64 = if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_lp_s0 <= assign65290_cond_e100809)) { 1.0 } else { 0.0 };
            assign65290_cond_e100811 != 0.0
        } {
            assign65290_loop_guard += 1;
            assert!(assign65290_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign65290_body0_e100828, assign65290_body0_e100828_d_n0, assign65290_body0_e100828_d_n2, assign65290_body0_e100828_d_n4, assign65290_body0_e100828_d_n5, assign65290_body0_e100828_d_n6, assign65290_body0_e100828_d_n7, assign65290_body0_e100828_d_n8, assign65290_body0_e100828_d_n9, assign65290_body0_e100828_d_n10, assign65290_body0_e100828_d_n11, assign65290_body0_e100828_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) {
        let assign65290_body0_e100826: f64 = (locals.var_phi_s0 - locals.var_vbscl__blk1547);
        (assign65290_body0_e100826, (locals.var_phi_s0_dn0 - locals.var_vbscl__blk1547_dn0), (locals.var_phi_s0_dn2 - locals.var_vbscl__blk1547_dn2), (locals.var_phi_s0_dn4 - locals.var_vbscl__blk1547_dn4), (locals.var_phi_s0_dn5 - locals.var_vbscl__blk1547_dn5), (locals.var_phi_s0_dn6 - locals.var_vbscl__blk1547_dn6), (locals.var_phi_s0_dn7 - locals.var_vbscl__blk1547_dn7), (locals.var_phi_s0_dn8 - locals.var_vbscl__blk1547_dn8), (locals.var_phi_s0_dn9 - locals.var_vbscl__blk1547_dn9), (locals.var_phi_s0_dn10 - locals.var_vbscl__blk1547_dn10), (locals.var_phi_s0_dn11 - locals.var_vbscl__blk1547_dn11), (locals.var_phi_s0_dn14 - locals.var_vbscl__blk1547_dn14),)
    } else {
        (locals.var_phi_0, locals.var_phi_0_dn0, locals.var_phi_0_dn2, locals.var_phi_0_dn4, locals.var_phi_0_dn5, locals.var_phi_0_dn6, locals.var_phi_0_dn7, locals.var_phi_0_dn8, locals.var_phi_0_dn9, locals.var_phi_0_dn10, locals.var_phi_0_dn11, locals.var_phi_0_dn14,)
    }
};
            locals.var_phi_0 = assign65290_body0_e100828;
            locals.var_phi_0_dn0 = assign65290_body0_e100828_d_n0;
            locals.var_phi_0_dn2 = assign65290_body0_e100828_d_n2;
            locals.var_phi_0_dn4 = assign65290_body0_e100828_d_n4;
            locals.var_phi_0_dn5 = assign65290_body0_e100828_d_n5;
            locals.var_phi_0_dn6 = assign65290_body0_e100828_d_n6;
            locals.var_phi_0_dn7 = assign65290_body0_e100828_d_n7;
            locals.var_phi_0_dn8 = assign65290_body0_e100828_d_n8;
            locals.var_phi_0_dn9 = assign65290_body0_e100828_d_n9;
            locals.var_phi_0_dn10 = assign65290_body0_e100828_d_n10;
            locals.var_phi_0_dn11 = assign65290_body0_e100828_d_n11;
            locals.var_phi_0_dn14 = assign65290_body0_e100828_d_n14;
            locals.var_phi_0_rv = 0.0;
            let (assign65290_body1_e100845, assign65290_body1_e100845_d_n0, assign65290_body1_e100845_d_n2, assign65290_body1_e100845_d_n4, assign65290_body1_e100845_d_n5, assign65290_body1_e100845_d_n6, assign65290_body1_e100845_d_n7, assign65290_body1_e100845_d_n8, assign65290_body1_e100845_d_n9, assign65290_body1_e100845_d_n10, assign65290_body1_e100845_d_n11, assign65290_body1_e100845_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) {
        let assign65290_body1_e100843: f64 = (locals.var_beta * locals.var_phi_0);
        (assign65290_body1_e100843, ((locals.var_beta_dn0 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn0)), ((locals.var_beta_dn2 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn2)), ((locals.var_beta_dn4 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn4)), ((locals.var_beta_dn5 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn5)), ((locals.var_beta_dn6 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn6)), ((locals.var_beta_dn7 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn7)), ((locals.var_beta_dn8 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn8)), ((locals.var_beta_dn9 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn9)), ((locals.var_beta_dn10 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn10)), ((locals.var_beta_dn11 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn11)), ((locals.var_beta_dn14 * locals.var_phi_0) + (locals.var_beta * locals.var_phi_0_dn14)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
            locals.var_chi = assign65290_body1_e100845;
            locals.var_chi_dn0 = assign65290_body1_e100845_d_n0;
            locals.var_chi_dn2 = assign65290_body1_e100845_d_n2;
            locals.var_chi_dn4 = assign65290_body1_e100845_d_n4;
            locals.var_chi_dn5 = assign65290_body1_e100845_d_n5;
            locals.var_chi_dn6 = assign65290_body1_e100845_d_n6;
            locals.var_chi_dn7 = assign65290_body1_e100845_d_n7;
            locals.var_chi_dn8 = assign65290_body1_e100845_d_n8;
            locals.var_chi_dn9 = assign65290_body1_e100845_d_n9;
            locals.var_chi_dn10 = assign65290_body1_e100845_d_n10;
            locals.var_chi_dn11 = assign65290_body1_e100845_d_n11;
            locals.var_chi_dn14 = assign65290_body1_e100845_d_n14;
            locals.var_chi_rv = 0.0;
            let (assign65290_body2_e100864, assign65290_body2_e100864_d_n0, assign65290_body2_e100864_d_n2, assign65290_body2_e100864_d_n4, assign65290_body2_e100864_d_n5, assign65290_body2_e100864_d_n6, assign65290_body2_e100864_d_n7, assign65290_body2_e100864_d_n8, assign65290_body2_e100864_d_n9, assign65290_body2_e100864_d_n10, assign65290_body2_e100864_d_n11, assign65290_body2_e100864_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) {
        let assign65290_body2_e100861: f64 = (locals.var_phi_0 - locals.var_dphi_sb__blk1549);
        let assign65290_body2_e100862: f64 = (locals.var_c_sb__blk1550 * assign65290_body2_e100861);
        (assign65290_body2_e100862, ((locals.var_c_sb__blk1550_dn0 * assign65290_body2_e100861) + (locals.var_c_sb__blk1550 * (locals.var_phi_0_dn0 - locals.var_dphi_sb__blk1549_dn0))), ((locals.var_c_sb__blk1550_dn2 * assign65290_body2_e100861) + (locals.var_c_sb__blk1550 * (locals.var_phi_0_dn2 - locals.var_dphi_sb__blk1549_dn2))), ((locals.var_c_sb__blk1550_dn4 * assign65290_body2_e100861) + (locals.var_c_sb__blk1550 * (locals.var_phi_0_dn4 - locals.var_dphi_sb__blk1549_dn4))), ((locals.var_c_sb__blk1550_dn5 * assign65290_body2_e100861) + (locals.var_c_sb__blk1550 * (locals.var_phi_0_dn5 - locals.var_dphi_sb__blk1549_dn5))), ((locals.var_c_sb__blk1550_dn6 * assign65290_body2_e100861) + (locals.var_c_sb__blk1550 * (locals.var_phi_0_dn6 - locals.var_dphi_sb__blk1549_dn6))), ((locals.var_c_sb__blk1550_dn7 * assign65290_body2_e100861) + (locals.var_c_sb__blk1550 * (locals.var_phi_0_dn7 - locals.var_dphi_sb__blk1549_dn7))), ((locals.var_c_sb__blk1550_dn8 * assign65290_body2_e100861) + (locals.var_c_sb__blk1550 * (locals.var_phi_0_dn8 - locals.var_dphi_sb__blk1549_dn8))), ((locals.var_c_sb__blk1550_dn9 * assign65290_body2_e100861) + (locals.var_c_sb__blk1550 * (locals.var_phi_0_dn9 - locals.var_dphi_sb__blk1549_dn9))), ((locals.var_c_sb__blk1550_dn10 * assign65290_body2_e100861) + (locals.var_c_sb__blk1550 * (locals.var_phi_0_dn10 - locals.var_dphi_sb__blk1549_dn10))), ((locals.var_c_sb__blk1550_dn11 * assign65290_body2_e100861) + (locals.var_c_sb__blk1550 * (locals.var_phi_0_dn11 - locals.var_dphi_sb__blk1549_dn11))), ((locals.var_c_sb__blk1550_dn14 * assign65290_body2_e100861) + (locals.var_c_sb__blk1550 * (locals.var_phi_0_dn14 - locals.var_dphi_sb__blk1549_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
            locals.var_ty = assign65290_body2_e100864;
            locals.var_ty_dn0 = assign65290_body2_e100864_d_n0;
            locals.var_ty_dn2 = assign65290_body2_e100864_d_n2;
            locals.var_ty_dn4 = assign65290_body2_e100864_d_n4;
            locals.var_ty_dn5 = assign65290_body2_e100864_d_n5;
            locals.var_ty_dn6 = assign65290_body2_e100864_d_n6;
            locals.var_ty_dn7 = assign65290_body2_e100864_d_n7;
            locals.var_ty_dn8 = assign65290_body2_e100864_d_n8;
            locals.var_ty_dn9 = assign65290_body2_e100864_d_n9;
            locals.var_ty_dn10 = assign65290_body2_e100864_d_n10;
            locals.var_ty_dn11 = assign65290_body2_e100864_d_n11;
            locals.var_ty_dn14 = assign65290_body2_e100864_d_n14;
            locals.var_ty_rv = 0.0;
            let assign65290_body3_e100867: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard1558 = assign65290_body3_e100867;
            locals.var_guard1558_rv = 0.0;
            let (assign65290_body4_e100885, assign65290_body4_e100885_d_n0, assign65290_body4_e100885_d_n2, assign65290_body4_e100885_d_n4, assign65290_body4_e100885_d_n5, assign65290_body4_e100885_d_n6, assign65290_body4_e100885_d_n7, assign65290_body4_e100885_d_n8, assign65290_body4_e100885_d_n9, assign65290_body4_e100885_d_n10, assign65290_body4_e100885_d_n11, assign65290_body4_e100885_d_n14,) = {
    if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1558 != 0.0)) {
        let assign65290_body4_e100883: f64 = (locals.var_ty).exp();
        (assign65290_body4_e100883, (assign65290_body4_e100883 * locals.var_ty_dn0), (assign65290_body4_e100883 * locals.var_ty_dn2), (assign65290_body4_e100883 * locals.var_ty_dn4), (assign65290_body4_e100883 * locals.var_ty_dn5), (assign65290_body4_e100883 * locals.var_ty_dn6), (assign65290_body4_e100883 * locals.var_ty_dn7), (assign65290_body4_e100883 * locals.var_ty_dn8), (assign65290_body4_e100883 * locals.var_ty_dn9), (assign65290_body4_e100883 * locals.var_ty_dn10), (assign65290_body4_e100883 * locals.var_ty_dn11), (assign65290_body4_e100883 * locals.var_ty_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign65290_body4_e100885;
            locals.var_t1_dn0 = assign65290_body4_e100885_d_n0;
            locals.var_t1_dn2 = assign65290_body4_e100885_d_n2;
            locals.var_t1_dn4 = assign65290_body4_e100885_d_n4;
            locals.var_t1_dn5 = assign65290_body4_e100885_d_n5;
            locals.var_t1_dn6 = assign65290_body4_e100885_d_n6;
            locals.var_t1_dn7 = assign65290_body4_e100885_d_n7;
            locals.var_t1_dn8 = assign65290_body4_e100885_d_n8;
            locals.var_t1_dn9 = assign65290_body4_e100885_d_n9;
            locals.var_t1_dn10 = assign65290_body4_e100885_d_n10;
            locals.var_t1_dn11 = assign65290_body4_e100885_d_n11;
            locals.var_t1_dn14 = assign65290_body4_e100885_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign65290_body5_e100906, assign65290_body5_e100906_d_n0, assign65290_body5_e100906_d_n2, assign65290_body5_e100906_d_n4, assign65290_body5_e100906_d_n5, assign65290_body5_e100906_d_n6, assign65290_body5_e100906_d_n7, assign65290_body5_e100906_d_n8, assign65290_body5_e100906_d_n9, assign65290_body5_e100906_d_n10, assign65290_body5_e100906_d_n11, assign65290_body5_e100906_d_n14,) = {
    if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1558 != 0.0)) {
        let assign65290_body5_e100901: f64 = (-locals.var_c_sb__blk1550);
        let assign65290_body5_e100903: f64 = (assign65290_body5_e100901 * locals.var_dphi_sb__blk1549);
        let assign65290_body5_e100904: f64 = (assign65290_body5_e100903).exp();
        (assign65290_body5_e100904, (assign65290_body5_e100904 * (((-locals.var_c_sb__blk1550_dn0) * locals.var_dphi_sb__blk1549) + (assign65290_body5_e100901 * locals.var_dphi_sb__blk1549_dn0))), (assign65290_body5_e100904 * (((-locals.var_c_sb__blk1550_dn2) * locals.var_dphi_sb__blk1549) + (assign65290_body5_e100901 * locals.var_dphi_sb__blk1549_dn2))), (assign65290_body5_e100904 * (((-locals.var_c_sb__blk1550_dn4) * locals.var_dphi_sb__blk1549) + (assign65290_body5_e100901 * locals.var_dphi_sb__blk1549_dn4))), (assign65290_body5_e100904 * (((-locals.var_c_sb__blk1550_dn5) * locals.var_dphi_sb__blk1549) + (assign65290_body5_e100901 * locals.var_dphi_sb__blk1549_dn5))), (assign65290_body5_e100904 * (((-locals.var_c_sb__blk1550_dn6) * locals.var_dphi_sb__blk1549) + (assign65290_body5_e100901 * locals.var_dphi_sb__blk1549_dn6))), (assign65290_body5_e100904 * (((-locals.var_c_sb__blk1550_dn7) * locals.var_dphi_sb__blk1549) + (assign65290_body5_e100901 * locals.var_dphi_sb__blk1549_dn7))), (assign65290_body5_e100904 * (((-locals.var_c_sb__blk1550_dn8) * locals.var_dphi_sb__blk1549) + (assign65290_body5_e100901 * locals.var_dphi_sb__blk1549_dn8))), (assign65290_body5_e100904 * (((-locals.var_c_sb__blk1550_dn9) * locals.var_dphi_sb__blk1549) + (assign65290_body5_e100901 * locals.var_dphi_sb__blk1549_dn9))), (assign65290_body5_e100904 * (((-locals.var_c_sb__blk1550_dn10) * locals.var_dphi_sb__blk1549) + (assign65290_body5_e100901 * locals.var_dphi_sb__blk1549_dn10))), (assign65290_body5_e100904 * (((-locals.var_c_sb__blk1550_dn11) * locals.var_dphi_sb__blk1549) + (assign65290_body5_e100901 * locals.var_dphi_sb__blk1549_dn11))), (assign65290_body5_e100904 * (((-locals.var_c_sb__blk1550_dn14) * locals.var_dphi_sb__blk1549) + (assign65290_body5_e100901 * locals.var_dphi_sb__blk1549_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign65290_body5_e100906;
            locals.var_t0_dn0 = assign65290_body5_e100906_d_n0;
            locals.var_t0_dn2 = assign65290_body5_e100906_d_n2;
            locals.var_t0_dn4 = assign65290_body5_e100906_d_n4;
            locals.var_t0_dn5 = assign65290_body5_e100906_d_n5;
            locals.var_t0_dn6 = assign65290_body5_e100906_d_n6;
            locals.var_t0_dn7 = assign65290_body5_e100906_d_n7;
            locals.var_t0_dn8 = assign65290_body5_e100906_d_n8;
            locals.var_t0_dn9 = assign65290_body5_e100906_d_n9;
            locals.var_t0_dn10 = assign65290_body5_e100906_d_n10;
            locals.var_t0_dn11 = assign65290_body5_e100906_d_n11;
            locals.var_t0_dn14 = assign65290_body5_e100906_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign65290_body6_e100925, assign65290_body6_e100925_d_n0, assign65290_body6_e100925_d_n2, assign65290_body6_e100925_d_n4, assign65290_body6_e100925_d_n5, assign65290_body6_e100925_d_n6, assign65290_body6_e100925_d_n7, assign65290_body6_e100925_d_n8, assign65290_body6_e100925_d_n9, assign65290_body6_e100925_d_n10, assign65290_body6_e100925_d_n11, assign65290_body6_e100925_d_n14,) = {
    if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1558 != 0.0)) {
        let assign65290_body6_e100923: f64 = (locals.var_t1 - locals.var_t0);
        (assign65290_body6_e100923, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn14 - locals.var_t0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign65290_body6_e100925;
            locals.var_t2_dn0 = assign65290_body6_e100925_d_n0;
            locals.var_t2_dn2 = assign65290_body6_e100925_d_n2;
            locals.var_t2_dn4 = assign65290_body6_e100925_d_n4;
            locals.var_t2_dn5 = assign65290_body6_e100925_d_n5;
            locals.var_t2_dn6 = assign65290_body6_e100925_d_n6;
            locals.var_t2_dn7 = assign65290_body6_e100925_d_n7;
            locals.var_t2_dn8 = assign65290_body6_e100925_d_n8;
            locals.var_t2_dn9 = assign65290_body6_e100925_d_n9;
            locals.var_t2_dn10 = assign65290_body6_e100925_d_n10;
            locals.var_t2_dn11 = assign65290_body6_e100925_d_n11;
            locals.var_t2_dn14 = assign65290_body6_e100925_d_n14;
            locals.var_t2_rv = 0.0;
            let (assign65290_body7_e100947, assign65290_body7_e100947_d_n0, assign65290_body7_e100947_d_n2, assign65290_body7_e100947_d_n4, assign65290_body7_e100947_d_n5, assign65290_body7_e100947_d_n6, assign65290_body7_e100947_d_n7, assign65290_body7_e100947_d_n8, assign65290_body7_e100947_d_n9, assign65290_body7_e100947_d_n10, assign65290_body7_e100947_d_n11, assign65290_body7_e100947_d_n14,) = {
    if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1558 != 0.0)) {
        let assign65290_body7_e100942: f64 = (1.0 + locals.var_t2);
        let assign65290_body7_e100943: f64 = (assign65290_body7_e100942).ln();
        let assign65290_body7_e100945: f64 = (assign65290_body7_e100943 / locals.var_c_sb__blk1550);
        (assign65290_body7_e100945, ((((locals.var_t2_dn0 / assign65290_body7_e100942) * locals.var_c_sb__blk1550) - (assign65290_body7_e100943 * locals.var_c_sb__blk1550_dn0)) / (locals.var_c_sb__blk1550 * locals.var_c_sb__blk1550)), ((((locals.var_t2_dn2 / assign65290_body7_e100942) * locals.var_c_sb__blk1550) - (assign65290_body7_e100943 * locals.var_c_sb__blk1550_dn2)) / (locals.var_c_sb__blk1550 * locals.var_c_sb__blk1550)), ((((locals.var_t2_dn4 / assign65290_body7_e100942) * locals.var_c_sb__blk1550) - (assign65290_body7_e100943 * locals.var_c_sb__blk1550_dn4)) / (locals.var_c_sb__blk1550 * locals.var_c_sb__blk1550)), ((((locals.var_t2_dn5 / assign65290_body7_e100942) * locals.var_c_sb__blk1550) - (assign65290_body7_e100943 * locals.var_c_sb__blk1550_dn5)) / (locals.var_c_sb__blk1550 * locals.var_c_sb__blk1550)), ((((locals.var_t2_dn6 / assign65290_body7_e100942) * locals.var_c_sb__blk1550) - (assign65290_body7_e100943 * locals.var_c_sb__blk1550_dn6)) / (locals.var_c_sb__blk1550 * locals.var_c_sb__blk1550)), ((((locals.var_t2_dn7 / assign65290_body7_e100942) * locals.var_c_sb__blk1550) - (assign65290_body7_e100943 * locals.var_c_sb__blk1550_dn7)) / (locals.var_c_sb__blk1550 * locals.var_c_sb__blk1550)), ((((locals.var_t2_dn8 / assign65290_body7_e100942) * locals.var_c_sb__blk1550) - (assign65290_body7_e100943 * locals.var_c_sb__blk1550_dn8)) / (locals.var_c_sb__blk1550 * locals.var_c_sb__blk1550)), ((((locals.var_t2_dn9 / assign65290_body7_e100942) * locals.var_c_sb__blk1550) - (assign65290_body7_e100943 * locals.var_c_sb__blk1550_dn9)) / (locals.var_c_sb__blk1550 * locals.var_c_sb__blk1550)), ((((locals.var_t2_dn10 / assign65290_body7_e100942) * locals.var_c_sb__blk1550) - (assign65290_body7_e100943 * locals.var_c_sb__blk1550_dn10)) / (locals.var_c_sb__blk1550 * locals.var_c_sb__blk1550)), ((((locals.var_t2_dn11 / assign65290_body7_e100942) * locals.var_c_sb__blk1550) - (assign65290_body7_e100943 * locals.var_c_sb__blk1550_dn11)) / (locals.var_c_sb__blk1550 * locals.var_c_sb__blk1550)), ((((locals.var_t2_dn14 / assign65290_body7_e100942) * locals.var_c_sb__blk1550) - (assign65290_body7_e100943 * locals.var_c_sb__blk1550_dn14)) / (locals.var_c_sb__blk1550 * locals.var_c_sb__blk1550)),)
    } else {
        (locals.var_phi_b__blk1553, locals.var_phi_b__blk1553_dn0, locals.var_phi_b__blk1553_dn2, locals.var_phi_b__blk1553_dn4, locals.var_phi_b__blk1553_dn5, locals.var_phi_b__blk1553_dn6, locals.var_phi_b__blk1553_dn7, locals.var_phi_b__blk1553_dn8, locals.var_phi_b__blk1553_dn9, locals.var_phi_b__blk1553_dn10, locals.var_phi_b__blk1553_dn11, locals.var_phi_b__blk1553_dn14,)
    }
};
            locals.var_phi_b__blk1553 = assign65290_body7_e100947;
            locals.var_phi_b__blk1553_dn0 = assign65290_body7_e100947_d_n0;
            locals.var_phi_b__blk1553_dn2 = assign65290_body7_e100947_d_n2;
            locals.var_phi_b__blk1553_dn4 = assign65290_body7_e100947_d_n4;
            locals.var_phi_b__blk1553_dn5 = assign65290_body7_e100947_d_n5;
            locals.var_phi_b__blk1553_dn6 = assign65290_body7_e100947_d_n6;
            locals.var_phi_b__blk1553_dn7 = assign65290_body7_e100947_d_n7;
            locals.var_phi_b__blk1553_dn8 = assign65290_body7_e100947_d_n8;
            locals.var_phi_b__blk1553_dn9 = assign65290_body7_e100947_d_n9;
            locals.var_phi_b__blk1553_dn10 = assign65290_body7_e100947_d_n10;
            locals.var_phi_b__blk1553_dn11 = assign65290_body7_e100947_d_n11;
            locals.var_phi_b__blk1553_dn14 = assign65290_body7_e100947_d_n14;
            locals.var_phi_b__blk1553_rv = 0.0;
            let (assign65290_body8_e100968, assign65290_body8_e100968_d_n0, assign65290_body8_e100968_d_n2, assign65290_body8_e100968_d_n4, assign65290_body8_e100968_d_n5, assign65290_body8_e100968_d_n6, assign65290_body8_e100968_d_n7, assign65290_body8_e100968_d_n8, assign65290_body8_e100968_d_n9, assign65290_body8_e100968_d_n10, assign65290_body8_e100968_d_n11, assign65290_body8_e100968_d_n14,) = {
    if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1558 != 0.0)) {
        let assign65290_body8_e100965: f64 = (1.0 + locals.var_t2);
        let assign65290_body8_e100966: f64 = (locals.var_t1 / assign65290_body8_e100965);
        (assign65290_body8_e100966, (((locals.var_t1_dn0 * assign65290_body8_e100965) - (locals.var_t1 * locals.var_t2_dn0)) / (assign65290_body8_e100965 * assign65290_body8_e100965)), (((locals.var_t1_dn2 * assign65290_body8_e100965) - (locals.var_t1 * locals.var_t2_dn2)) / (assign65290_body8_e100965 * assign65290_body8_e100965)), (((locals.var_t1_dn4 * assign65290_body8_e100965) - (locals.var_t1 * locals.var_t2_dn4)) / (assign65290_body8_e100965 * assign65290_body8_e100965)), (((locals.var_t1_dn5 * assign65290_body8_e100965) - (locals.var_t1 * locals.var_t2_dn5)) / (assign65290_body8_e100965 * assign65290_body8_e100965)), (((locals.var_t1_dn6 * assign65290_body8_e100965) - (locals.var_t1 * locals.var_t2_dn6)) / (assign65290_body8_e100965 * assign65290_body8_e100965)), (((locals.var_t1_dn7 * assign65290_body8_e100965) - (locals.var_t1 * locals.var_t2_dn7)) / (assign65290_body8_e100965 * assign65290_body8_e100965)), (((locals.var_t1_dn8 * assign65290_body8_e100965) - (locals.var_t1 * locals.var_t2_dn8)) / (assign65290_body8_e100965 * assign65290_body8_e100965)), (((locals.var_t1_dn9 * assign65290_body8_e100965) - (locals.var_t1 * locals.var_t2_dn9)) / (assign65290_body8_e100965 * assign65290_body8_e100965)), (((locals.var_t1_dn10 * assign65290_body8_e100965) - (locals.var_t1 * locals.var_t2_dn10)) / (assign65290_body8_e100965 * assign65290_body8_e100965)), (((locals.var_t1_dn11 * assign65290_body8_e100965) - (locals.var_t1 * locals.var_t2_dn11)) / (assign65290_body8_e100965 * assign65290_body8_e100965)), (((locals.var_t1_dn14 * assign65290_body8_e100965) - (locals.var_t1 * locals.var_t2_dn14)) / (assign65290_body8_e100965 * assign65290_body8_e100965)),)
    } else {
        (locals.var_phi_b_dpss__blk1554, locals.var_phi_b_dpss__blk1554_dn0, locals.var_phi_b_dpss__blk1554_dn2, locals.var_phi_b_dpss__blk1554_dn4, locals.var_phi_b_dpss__blk1554_dn5, locals.var_phi_b_dpss__blk1554_dn6, locals.var_phi_b_dpss__blk1554_dn7, locals.var_phi_b_dpss__blk1554_dn8, locals.var_phi_b_dpss__blk1554_dn9, locals.var_phi_b_dpss__blk1554_dn10, locals.var_phi_b_dpss__blk1554_dn11, locals.var_phi_b_dpss__blk1554_dn14,)
    }
};
            locals.var_phi_b_dpss__blk1554 = assign65290_body8_e100968;
            locals.var_phi_b_dpss__blk1554_dn0 = assign65290_body8_e100968_d_n0;
            locals.var_phi_b_dpss__blk1554_dn2 = assign65290_body8_e100968_d_n2;
            locals.var_phi_b_dpss__blk1554_dn4 = assign65290_body8_e100968_d_n4;
            locals.var_phi_b_dpss__blk1554_dn5 = assign65290_body8_e100968_d_n5;
            locals.var_phi_b_dpss__blk1554_dn6 = assign65290_body8_e100968_d_n6;
            locals.var_phi_b_dpss__blk1554_dn7 = assign65290_body8_e100968_d_n7;
            locals.var_phi_b_dpss__blk1554_dn8 = assign65290_body8_e100968_d_n8;
            locals.var_phi_b_dpss__blk1554_dn9 = assign65290_body8_e100968_d_n9;
            locals.var_phi_b_dpss__blk1554_dn10 = assign65290_body8_e100968_d_n10;
            locals.var_phi_b_dpss__blk1554_dn11 = assign65290_body8_e100968_d_n11;
            locals.var_phi_b_dpss__blk1554_dn14 = assign65290_body8_e100968_d_n14;
            locals.var_phi_b_dpss__blk1554_rv = 0.0;
            let (assign65290_body9_e100988, assign65290_body9_e100988_d_n0, assign65290_body9_e100988_d_n2, assign65290_body9_e100988_d_n4, assign65290_body9_e100988_d_n5, assign65290_body9_e100988_d_n6, assign65290_body9_e100988_d_n7, assign65290_body9_e100988_d_n8, assign65290_body9_e100988_d_n9, assign65290_body9_e100988_d_n10, assign65290_body9_e100988_d_n11, assign65290_body9_e100988_d_n14,) = {
    if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1558 == 0.0)) {
        let assign65290_body9_e100986: f64 = (locals.var_phi_0 - locals.var_dphi_sb__blk1549);
        (assign65290_body9_e100986, (locals.var_phi_0_dn0 - locals.var_dphi_sb__blk1549_dn0), (locals.var_phi_0_dn2 - locals.var_dphi_sb__blk1549_dn2), (locals.var_phi_0_dn4 - locals.var_dphi_sb__blk1549_dn4), (locals.var_phi_0_dn5 - locals.var_dphi_sb__blk1549_dn5), (locals.var_phi_0_dn6 - locals.var_dphi_sb__blk1549_dn6), (locals.var_phi_0_dn7 - locals.var_dphi_sb__blk1549_dn7), (locals.var_phi_0_dn8 - locals.var_dphi_sb__blk1549_dn8), (locals.var_phi_0_dn9 - locals.var_dphi_sb__blk1549_dn9), (locals.var_phi_0_dn10 - locals.var_dphi_sb__blk1549_dn10), (locals.var_phi_0_dn11 - locals.var_dphi_sb__blk1549_dn11), (locals.var_phi_0_dn14 - locals.var_dphi_sb__blk1549_dn14),)
    } else {
        (locals.var_phi_b__blk1553, locals.var_phi_b__blk1553_dn0, locals.var_phi_b__blk1553_dn2, locals.var_phi_b__blk1553_dn4, locals.var_phi_b__blk1553_dn5, locals.var_phi_b__blk1553_dn6, locals.var_phi_b__blk1553_dn7, locals.var_phi_b__blk1553_dn8, locals.var_phi_b__blk1553_dn9, locals.var_phi_b__blk1553_dn10, locals.var_phi_b__blk1553_dn11, locals.var_phi_b__blk1553_dn14,)
    }
};
            locals.var_phi_b__blk1553 = assign65290_body9_e100988;
            locals.var_phi_b__blk1553_dn0 = assign65290_body9_e100988_d_n0;
            locals.var_phi_b__blk1553_dn2 = assign65290_body9_e100988_d_n2;
            locals.var_phi_b__blk1553_dn4 = assign65290_body9_e100988_d_n4;
            locals.var_phi_b__blk1553_dn5 = assign65290_body9_e100988_d_n5;
            locals.var_phi_b__blk1553_dn6 = assign65290_body9_e100988_d_n6;
            locals.var_phi_b__blk1553_dn7 = assign65290_body9_e100988_d_n7;
            locals.var_phi_b__blk1553_dn8 = assign65290_body9_e100988_d_n8;
            locals.var_phi_b__blk1553_dn9 = assign65290_body9_e100988_d_n9;
            locals.var_phi_b__blk1553_dn10 = assign65290_body9_e100988_d_n10;
            locals.var_phi_b__blk1553_dn11 = assign65290_body9_e100988_d_n11;
            locals.var_phi_b__blk1553_dn14 = assign65290_body9_e100988_d_n14;
            locals.var_phi_b__blk1553_rv = 0.0;
            let (assign65290_body10_e101006, assign65290_body10_e101006_d_n0, assign65290_body10_e101006_d_n2, assign65290_body10_e101006_d_n4, assign65290_body10_e101006_d_n5, assign65290_body10_e101006_d_n6, assign65290_body10_e101006_d_n7, assign65290_body10_e101006_d_n8, assign65290_body10_e101006_d_n9, assign65290_body10_e101006_d_n10, assign65290_body10_e101006_d_n11, assign65290_body10_e101006_d_n14,) = {
    if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1558 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b_dpss__blk1554, locals.var_phi_b_dpss__blk1554_dn0, locals.var_phi_b_dpss__blk1554_dn2, locals.var_phi_b_dpss__blk1554_dn4, locals.var_phi_b_dpss__blk1554_dn5, locals.var_phi_b_dpss__blk1554_dn6, locals.var_phi_b_dpss__blk1554_dn7, locals.var_phi_b_dpss__blk1554_dn8, locals.var_phi_b_dpss__blk1554_dn9, locals.var_phi_b_dpss__blk1554_dn10, locals.var_phi_b_dpss__blk1554_dn11, locals.var_phi_b_dpss__blk1554_dn14,)
    }
};
            locals.var_phi_b_dpss__blk1554 = assign65290_body10_e101006;
            locals.var_phi_b_dpss__blk1554_dn0 = assign65290_body10_e101006_d_n0;
            locals.var_phi_b_dpss__blk1554_dn2 = assign65290_body10_e101006_d_n2;
            locals.var_phi_b_dpss__blk1554_dn4 = assign65290_body10_e101006_d_n4;
            locals.var_phi_b_dpss__blk1554_dn5 = assign65290_body10_e101006_d_n5;
            locals.var_phi_b_dpss__blk1554_dn6 = assign65290_body10_e101006_d_n6;
            locals.var_phi_b_dpss__blk1554_dn7 = assign65290_body10_e101006_d_n7;
            locals.var_phi_b_dpss__blk1554_dn8 = assign65290_body10_e101006_d_n8;
            locals.var_phi_b_dpss__blk1554_dn9 = assign65290_body10_e101006_d_n9;
            locals.var_phi_b_dpss__blk1554_dn10 = assign65290_body10_e101006_d_n10;
            locals.var_phi_b_dpss__blk1554_dn11 = assign65290_body10_e101006_d_n11;
            locals.var_phi_b_dpss__blk1554_dn14 = assign65290_body10_e101006_d_n14;
            locals.var_phi_b_dpss__blk1554_rv = 0.0;
            let (assign65290_body11_e101023, assign65290_body11_e101023_d_n0, assign65290_body11_e101023_d_n2, assign65290_body11_e101023_d_n4, assign65290_body11_e101023_d_n5, assign65290_body11_e101023_d_n6, assign65290_body11_e101023_d_n7, assign65290_body11_e101023_d_n8, assign65290_body11_e101023_d_n9, assign65290_body11_e101023_d_n10, assign65290_body11_e101023_d_n11, assign65290_body11_e101023_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) {
        let assign65290_body11_e101021: f64 = (locals.var_beta * locals.var_phi_b__blk1553);
        (assign65290_body11_e101021, ((locals.var_beta_dn0 * locals.var_phi_b__blk1553) + (locals.var_beta * locals.var_phi_b__blk1553_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b__blk1553) + (locals.var_beta * locals.var_phi_b__blk1553_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b__blk1553) + (locals.var_beta * locals.var_phi_b__blk1553_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b__blk1553) + (locals.var_beta * locals.var_phi_b__blk1553_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b__blk1553) + (locals.var_beta * locals.var_phi_b__blk1553_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b__blk1553) + (locals.var_beta * locals.var_phi_b__blk1553_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b__blk1553) + (locals.var_beta * locals.var_phi_b__blk1553_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b__blk1553) + (locals.var_beta * locals.var_phi_b__blk1553_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b__blk1553) + (locals.var_beta * locals.var_phi_b__blk1553_dn10)), ((locals.var_beta_dn11 * locals.var_phi_b__blk1553) + (locals.var_beta * locals.var_phi_b__blk1553_dn11)), ((locals.var_beta_dn14 * locals.var_phi_b__blk1553) + (locals.var_beta * locals.var_phi_b__blk1553_dn14)),)
    } else {
        (locals.var_chib__blk1552, locals.var_chib__blk1552_dn0, locals.var_chib__blk1552_dn2, locals.var_chib__blk1552_dn4, locals.var_chib__blk1552_dn5, locals.var_chib__blk1552_dn6, locals.var_chib__blk1552_dn7, locals.var_chib__blk1552_dn8, locals.var_chib__blk1552_dn9, locals.var_chib__blk1552_dn10, locals.var_chib__blk1552_dn11, locals.var_chib__blk1552_dn14,)
    }
};
            locals.var_chib__blk1552 = assign65290_body11_e101023;
            locals.var_chib__blk1552_dn0 = assign65290_body11_e101023_d_n0;
            locals.var_chib__blk1552_dn2 = assign65290_body11_e101023_d_n2;
            locals.var_chib__blk1552_dn4 = assign65290_body11_e101023_d_n4;
            locals.var_chib__blk1552_dn5 = assign65290_body11_e101023_d_n5;
            locals.var_chib__blk1552_dn6 = assign65290_body11_e101023_d_n6;
            locals.var_chib__blk1552_dn7 = assign65290_body11_e101023_d_n7;
            locals.var_chib__blk1552_dn8 = assign65290_body11_e101023_d_n8;
            locals.var_chib__blk1552_dn9 = assign65290_body11_e101023_d_n9;
            locals.var_chib__blk1552_dn10 = assign65290_body11_e101023_d_n10;
            locals.var_chib__blk1552_dn11 = assign65290_body11_e101023_d_n11;
            locals.var_chib__blk1552_dn14 = assign65290_body11_e101023_d_n14;
            locals.var_chib__blk1552_rv = 0.0;
            let assign65290_body12_e101025: f64 = (locals.var_chi).abs();
            let assign65290_body12_e101027: f64 = if assign65290_body12_e101025 < 1e-16 { 1.0 } else { 0.0 };
            locals.var_guard1559 = assign65290_body12_e101027;
            locals.var_guard1559_rv = 0.0;
            let (assign65290_body13_e101051, assign65290_body13_e101051_d_n0, assign65290_body13_e101051_d_n2, assign65290_body13_e101051_d_n4, assign65290_body13_e101051_d_n5, assign65290_body13_e101051_d_n6, assign65290_body13_e101051_d_n7, assign65290_body13_e101051_d_n8, assign65290_body13_e101051_d_n9, assign65290_body13_e101051_d_n10, assign65290_body13_e101051_d_n11, assign65290_body13_e101051_d_n14,) = {
    if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1559 != 0.0)) {
        let assign65290_body13_e101045: f64 = (locals.var_phi_b_dpss__blk1554 * locals.var_phi_b_dpss__blk1554);
        let assign65290_body13_e101046: f64 = (1.0 - assign65290_body13_e101045);
        let assign65290_body13_e101048: f64 = (assign65290_body13_e101046 / 2.0);
        let assign65290_body13_e101049: f64 = (assign65290_body13_e101048).sqrt();
        (assign65290_body13_e101049, (((-((locals.var_phi_b_dpss__blk1554_dn0 * locals.var_phi_b_dpss__blk1554) + (locals.var_phi_b_dpss__blk1554 * locals.var_phi_b_dpss__blk1554_dn0))) / 2.0) / (2.0 * assign65290_body13_e101049)), (((-((locals.var_phi_b_dpss__blk1554_dn2 * locals.var_phi_b_dpss__blk1554) + (locals.var_phi_b_dpss__blk1554 * locals.var_phi_b_dpss__blk1554_dn2))) / 2.0) / (2.0 * assign65290_body13_e101049)), (((-((locals.var_phi_b_dpss__blk1554_dn4 * locals.var_phi_b_dpss__blk1554) + (locals.var_phi_b_dpss__blk1554 * locals.var_phi_b_dpss__blk1554_dn4))) / 2.0) / (2.0 * assign65290_body13_e101049)), (((-((locals.var_phi_b_dpss__blk1554_dn5 * locals.var_phi_b_dpss__blk1554) + (locals.var_phi_b_dpss__blk1554 * locals.var_phi_b_dpss__blk1554_dn5))) / 2.0) / (2.0 * assign65290_body13_e101049)), (((-((locals.var_phi_b_dpss__blk1554_dn6 * locals.var_phi_b_dpss__blk1554) + (locals.var_phi_b_dpss__blk1554 * locals.var_phi_b_dpss__blk1554_dn6))) / 2.0) / (2.0 * assign65290_body13_e101049)), (((-((locals.var_phi_b_dpss__blk1554_dn7 * locals.var_phi_b_dpss__blk1554) + (locals.var_phi_b_dpss__blk1554 * locals.var_phi_b_dpss__blk1554_dn7))) / 2.0) / (2.0 * assign65290_body13_e101049)), (((-((locals.var_phi_b_dpss__blk1554_dn8 * locals.var_phi_b_dpss__blk1554) + (locals.var_phi_b_dpss__blk1554 * locals.var_phi_b_dpss__blk1554_dn8))) / 2.0) / (2.0 * assign65290_body13_e101049)), (((-((locals.var_phi_b_dpss__blk1554_dn9 * locals.var_phi_b_dpss__blk1554) + (locals.var_phi_b_dpss__blk1554 * locals.var_phi_b_dpss__blk1554_dn9))) / 2.0) / (2.0 * assign65290_body13_e101049)), (((-((locals.var_phi_b_dpss__blk1554_dn10 * locals.var_phi_b_dpss__blk1554) + (locals.var_phi_b_dpss__blk1554 * locals.var_phi_b_dpss__blk1554_dn10))) / 2.0) / (2.0 * assign65290_body13_e101049)), (((-((locals.var_phi_b_dpss__blk1554_dn11 * locals.var_phi_b_dpss__blk1554) + (locals.var_phi_b_dpss__blk1554 * locals.var_phi_b_dpss__blk1554_dn11))) / 2.0) / (2.0 * assign65290_body13_e101049)), (((-((locals.var_phi_b_dpss__blk1554_dn14 * locals.var_phi_b_dpss__blk1554) + (locals.var_phi_b_dpss__blk1554 * locals.var_phi_b_dpss__blk1554_dn14))) / 2.0) / (2.0 * assign65290_body13_e101049)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign65290_body13_e101051;
            locals.var_t0_dn0 = assign65290_body13_e101051_d_n0;
            locals.var_t0_dn2 = assign65290_body13_e101051_d_n2;
            locals.var_t0_dn4 = assign65290_body13_e101051_d_n4;
            locals.var_t0_dn5 = assign65290_body13_e101051_d_n5;
            locals.var_t0_dn6 = assign65290_body13_e101051_d_n6;
            locals.var_t0_dn7 = assign65290_body13_e101051_d_n7;
            locals.var_t0_dn8 = assign65290_body13_e101051_d_n8;
            locals.var_t0_dn9 = assign65290_body13_e101051_d_n9;
            locals.var_t0_dn10 = assign65290_body13_e101051_d_n10;
            locals.var_t0_dn11 = assign65290_body13_e101051_d_n11;
            locals.var_t0_dn14 = assign65290_body13_e101051_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign65290_body14_e101070, assign65290_body14_e101070_d_n0, assign65290_body14_e101070_d_n2, assign65290_body14_e101070_d_n4, assign65290_body14_e101070_d_n5, assign65290_body14_e101070_d_n6, assign65290_body14_e101070_d_n7, assign65290_body14_e101070_d_n8, assign65290_body14_e101070_d_n9, assign65290_body14_e101070_d_n10, assign65290_body14_e101070_d_n11, assign65290_body14_e101070_d_n14,) = {
    if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1559 != 0.0)) {
        let assign65290_body14_e101068: f64 = (locals.var_chi * locals.var_t0);
        (assign65290_body14_e101068, ((locals.var_chi_dn0 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn0)), ((locals.var_chi_dn2 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn2)), ((locals.var_chi_dn4 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn4)), ((locals.var_chi_dn5 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn5)), ((locals.var_chi_dn6 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn6)), ((locals.var_chi_dn7 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn7)), ((locals.var_chi_dn8 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn8)), ((locals.var_chi_dn9 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn9)), ((locals.var_chi_dn10 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn10)), ((locals.var_chi_dn11 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn11)), ((locals.var_chi_dn14 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn14)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign65290_body14_e101070;
            locals.var_fb_dn0 = assign65290_body14_e101070_d_n0;
            locals.var_fb_dn2 = assign65290_body14_e101070_d_n2;
            locals.var_fb_dn4 = assign65290_body14_e101070_d_n4;
            locals.var_fb_dn5 = assign65290_body14_e101070_d_n5;
            locals.var_fb_dn6 = assign65290_body14_e101070_d_n6;
            locals.var_fb_dn7 = assign65290_body14_e101070_d_n7;
            locals.var_fb_dn8 = assign65290_body14_e101070_d_n8;
            locals.var_fb_dn9 = assign65290_body14_e101070_d_n9;
            locals.var_fb_dn10 = assign65290_body14_e101070_d_n10;
            locals.var_fb_dn11 = assign65290_body14_e101070_d_n11;
            locals.var_fb_dn14 = assign65290_body14_e101070_d_n14;
            locals.var_fb_rv = 0.0;
            let (assign65290_body15_e101089, assign65290_body15_e101089_d_n0, assign65290_body15_e101089_d_n2, assign65290_body15_e101089_d_n4, assign65290_body15_e101089_d_n5, assign65290_body15_e101089_d_n6, assign65290_body15_e101089_d_n7, assign65290_body15_e101089_d_n8, assign65290_body15_e101089_d_n9, assign65290_body15_e101089_d_n10, assign65290_body15_e101089_d_n11, assign65290_body15_e101089_d_n14,) = {
    if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1559 != 0.0)) {
        let assign65290_body15_e101087: f64 = (locals.var_beta * locals.var_t0);
        (assign65290_body15_e101087, ((locals.var_beta_dn0 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn0)), ((locals.var_beta_dn2 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn2)), ((locals.var_beta_dn4 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn4)), ((locals.var_beta_dn5 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn5)), ((locals.var_beta_dn6 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn6)), ((locals.var_beta_dn7 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn7)), ((locals.var_beta_dn8 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn8)), ((locals.var_beta_dn9 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn9)), ((locals.var_beta_dn10 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn10)), ((locals.var_beta_dn11 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn11)), ((locals.var_beta_dn14 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn14)),)
    } else {
        (locals.var_fb_dpss__blk1555, locals.var_fb_dpss__blk1555_dn0, locals.var_fb_dpss__blk1555_dn2, locals.var_fb_dpss__blk1555_dn4, locals.var_fb_dpss__blk1555_dn5, locals.var_fb_dpss__blk1555_dn6, locals.var_fb_dpss__blk1555_dn7, locals.var_fb_dpss__blk1555_dn8, locals.var_fb_dpss__blk1555_dn9, locals.var_fb_dpss__blk1555_dn10, locals.var_fb_dpss__blk1555_dn11, locals.var_fb_dpss__blk1555_dn14,)
    }
};
            locals.var_fb_dpss__blk1555 = assign65290_body15_e101089;
            locals.var_fb_dpss__blk1555_dn0 = assign65290_body15_e101089_d_n0;
            locals.var_fb_dpss__blk1555_dn2 = assign65290_body15_e101089_d_n2;
            locals.var_fb_dpss__blk1555_dn4 = assign65290_body15_e101089_d_n4;
            locals.var_fb_dpss__blk1555_dn5 = assign65290_body15_e101089_d_n5;
            locals.var_fb_dpss__blk1555_dn6 = assign65290_body15_e101089_d_n6;
            locals.var_fb_dpss__blk1555_dn7 = assign65290_body15_e101089_d_n7;
            locals.var_fb_dpss__blk1555_dn8 = assign65290_body15_e101089_d_n8;
            locals.var_fb_dpss__blk1555_dn9 = assign65290_body15_e101089_d_n9;
            locals.var_fb_dpss__blk1555_dn10 = assign65290_body15_e101089_d_n10;
            locals.var_fb_dpss__blk1555_dn11 = assign65290_body15_e101089_d_n11;
            locals.var_fb_dpss__blk1555_dn14 = assign65290_body15_e101089_d_n14;
            locals.var_fb_dpss__blk1555_rv = 0.0;
            let assign65290_body16_e101092: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1560 = assign65290_body16_e101092;
            locals.var_guard1560_rv = 0.0;
            let (assign65290_body17_e101112, assign65290_body17_e101112_d_n0, assign65290_body17_e101112_d_n2, assign65290_body17_e101112_d_n4, assign65290_body17_e101112_d_n5, assign65290_body17_e101112_d_n6, assign65290_body17_e101112_d_n7, assign65290_body17_e101112_d_n8, assign65290_body17_e101112_d_n9, assign65290_body17_e101112_d_n10, assign65290_body17_e101112_d_n11, assign65290_body17_e101112_d_n14,) = {
    if (((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1559 != 0.0)) && (locals.var_guard1560 != 0.0)) {
        let assign65290_body17_e101110: f64 = (-locals.var_fb);
        (assign65290_body17_e101110, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn4), (-locals.var_fb_dn5), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn8), (-locals.var_fb_dn9), (-locals.var_fb_dn10), (-locals.var_fb_dn11), (-locals.var_fb_dn14),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign65290_body17_e101112;
            locals.var_fb_dn0 = assign65290_body17_e101112_d_n0;
            locals.var_fb_dn2 = assign65290_body17_e101112_d_n2;
            locals.var_fb_dn4 = assign65290_body17_e101112_d_n4;
            locals.var_fb_dn5 = assign65290_body17_e101112_d_n5;
            locals.var_fb_dn6 = assign65290_body17_e101112_d_n6;
            locals.var_fb_dn7 = assign65290_body17_e101112_d_n7;
            locals.var_fb_dn8 = assign65290_body17_e101112_d_n8;
            locals.var_fb_dn9 = assign65290_body17_e101112_d_n9;
            locals.var_fb_dn10 = assign65290_body17_e101112_d_n10;
            locals.var_fb_dn11 = assign65290_body17_e101112_d_n11;
            locals.var_fb_dn14 = assign65290_body17_e101112_d_n14;
            locals.var_fb_rv = 0.0;
            let (assign65290_body18_e101132, assign65290_body18_e101132_d_n0, assign65290_body18_e101132_d_n2, assign65290_body18_e101132_d_n4, assign65290_body18_e101132_d_n5, assign65290_body18_e101132_d_n6, assign65290_body18_e101132_d_n7, assign65290_body18_e101132_d_n8, assign65290_body18_e101132_d_n9, assign65290_body18_e101132_d_n10, assign65290_body18_e101132_d_n11, assign65290_body18_e101132_d_n14,) = {
    if (((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1559 != 0.0)) && (locals.var_guard1560 != 0.0)) {
        let assign65290_body18_e101130: f64 = (-locals.var_fb_dpss__blk1555);
        (assign65290_body18_e101130, (-locals.var_fb_dpss__blk1555_dn0), (-locals.var_fb_dpss__blk1555_dn2), (-locals.var_fb_dpss__blk1555_dn4), (-locals.var_fb_dpss__blk1555_dn5), (-locals.var_fb_dpss__blk1555_dn6), (-locals.var_fb_dpss__blk1555_dn7), (-locals.var_fb_dpss__blk1555_dn8), (-locals.var_fb_dpss__blk1555_dn9), (-locals.var_fb_dpss__blk1555_dn10), (-locals.var_fb_dpss__blk1555_dn11), (-locals.var_fb_dpss__blk1555_dn14),)
    } else {
        (locals.var_fb_dpss__blk1555, locals.var_fb_dpss__blk1555_dn0, locals.var_fb_dpss__blk1555_dn2, locals.var_fb_dpss__blk1555_dn4, locals.var_fb_dpss__blk1555_dn5, locals.var_fb_dpss__blk1555_dn6, locals.var_fb_dpss__blk1555_dn7, locals.var_fb_dpss__blk1555_dn8, locals.var_fb_dpss__blk1555_dn9, locals.var_fb_dpss__blk1555_dn10, locals.var_fb_dpss__blk1555_dn11, locals.var_fb_dpss__blk1555_dn14,)
    }
};
            locals.var_fb_dpss__blk1555 = assign65290_body18_e101132;
            locals.var_fb_dpss__blk1555_dn0 = assign65290_body18_e101132_d_n0;
            locals.var_fb_dpss__blk1555_dn2 = assign65290_body18_e101132_d_n2;
            locals.var_fb_dpss__blk1555_dn4 = assign65290_body18_e101132_d_n4;
            locals.var_fb_dpss__blk1555_dn5 = assign65290_body18_e101132_d_n5;
            locals.var_fb_dpss__blk1555_dn6 = assign65290_body18_e101132_d_n6;
            locals.var_fb_dpss__blk1555_dn7 = assign65290_body18_e101132_d_n7;
            locals.var_fb_dpss__blk1555_dn8 = assign65290_body18_e101132_d_n8;
            locals.var_fb_dpss__blk1555_dn9 = assign65290_body18_e101132_d_n9;
            locals.var_fb_dpss__blk1555_dn10 = assign65290_body18_e101132_d_n10;
            locals.var_fb_dpss__blk1555_dn11 = assign65290_body18_e101132_d_n11;
            locals.var_fb_dpss__blk1555_dn14 = assign65290_body18_e101132_d_n14;
            locals.var_fb_dpss__blk1555_rv = 0.0;
            let assign65290_body19_e101134: f64 = (locals.var_chi).abs();
            let assign65290_body19_e101136: f64 = if assign65290_body19_e101134 < 0.005 { 1.0 } else { 0.0 };
            locals.var_guard1561 = assign65290_body19_e101136;
            locals.var_guard1561_rv = 0.0;
            let (assign65290_body20_e101178, assign65290_body20_e101178_d_n0, assign65290_body20_e101178_d_n2, assign65290_body20_e101178_d_n4, assign65290_body20_e101178_d_n5, assign65290_body20_e101178_d_n6, assign65290_body20_e101178_d_n7, assign65290_body20_e101178_d_n8, assign65290_body20_e101178_d_n9, assign65290_body20_e101178_d_n10, assign65290_body20_e101178_d_n11, assign65290_body20_e101178_d_n14,) = {
    if (((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1559 == 0.0)) && (locals.var_guard1561 != 0.0)) {
        let assign65290_body20_e101156: f64 = (locals.var_chi * locals.var_chi);
        let assign65290_body20_e101158: f64 = (assign65290_body20_e101156 / 2.0);
        let assign65290_body20_e101162: f64 = (locals.var_chi / 3.0);
        let assign65290_body20_e101166: f64 = (locals.var_chi / 4.0);
        let assign65290_body20_e101170: f64 = (locals.var_chi / 5.0);
        let assign65290_body20_e101171: f64 = (1.0 - assign65290_body20_e101170);
        let assign65290_body20_e101172: f64 = (assign65290_body20_e101166 * assign65290_body20_e101171);
        let assign65290_body20_e101173: f64 = (1.0 - assign65290_body20_e101172);
        let assign65290_body20_e101174: f64 = (assign65290_body20_e101162 * assign65290_body20_e101173);
        let assign65290_body20_e101175: f64 = (1.0 - assign65290_body20_e101174);
        let assign65290_body20_e101176: f64 = (assign65290_body20_e101158 * assign65290_body20_e101175);
        (assign65290_body20_e101176, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign65290_body20_e101175) + (assign65290_body20_e101158 * (-(((locals.var_chi_dn0 / 3.0) * assign65290_body20_e101173) + (assign65290_body20_e101162 * (-(((locals.var_chi_dn0 / 4.0) * assign65290_body20_e101171) + (assign65290_body20_e101166 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign65290_body20_e101175) + (assign65290_body20_e101158 * (-(((locals.var_chi_dn2 / 3.0) * assign65290_body20_e101173) + (assign65290_body20_e101162 * (-(((locals.var_chi_dn2 / 4.0) * assign65290_body20_e101171) + (assign65290_body20_e101166 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign65290_body20_e101175) + (assign65290_body20_e101158 * (-(((locals.var_chi_dn4 / 3.0) * assign65290_body20_e101173) + (assign65290_body20_e101162 * (-(((locals.var_chi_dn4 / 4.0) * assign65290_body20_e101171) + (assign65290_body20_e101166 * (-(locals.var_chi_dn4 / 5.0)))))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign65290_body20_e101175) + (assign65290_body20_e101158 * (-(((locals.var_chi_dn5 / 3.0) * assign65290_body20_e101173) + (assign65290_body20_e101162 * (-(((locals.var_chi_dn5 / 4.0) * assign65290_body20_e101171) + (assign65290_body20_e101166 * (-(locals.var_chi_dn5 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign65290_body20_e101175) + (assign65290_body20_e101158 * (-(((locals.var_chi_dn6 / 3.0) * assign65290_body20_e101173) + (assign65290_body20_e101162 * (-(((locals.var_chi_dn6 / 4.0) * assign65290_body20_e101171) + (assign65290_body20_e101166 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign65290_body20_e101175) + (assign65290_body20_e101158 * (-(((locals.var_chi_dn7 / 3.0) * assign65290_body20_e101173) + (assign65290_body20_e101162 * (-(((locals.var_chi_dn7 / 4.0) * assign65290_body20_e101171) + (assign65290_body20_e101166 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign65290_body20_e101175) + (assign65290_body20_e101158 * (-(((locals.var_chi_dn8 / 3.0) * assign65290_body20_e101173) + (assign65290_body20_e101162 * (-(((locals.var_chi_dn8 / 4.0) * assign65290_body20_e101171) + (assign65290_body20_e101166 * (-(locals.var_chi_dn8 / 5.0)))))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign65290_body20_e101175) + (assign65290_body20_e101158 * (-(((locals.var_chi_dn9 / 3.0) * assign65290_body20_e101173) + (assign65290_body20_e101162 * (-(((locals.var_chi_dn9 / 4.0) * assign65290_body20_e101171) + (assign65290_body20_e101166 * (-(locals.var_chi_dn9 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign65290_body20_e101175) + (assign65290_body20_e101158 * (-(((locals.var_chi_dn10 / 3.0) * assign65290_body20_e101173) + (assign65290_body20_e101162 * (-(((locals.var_chi_dn10 / 4.0) * assign65290_body20_e101171) + (assign65290_body20_e101166 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) / 2.0) * assign65290_body20_e101175) + (assign65290_body20_e101158 * (-(((locals.var_chi_dn11 / 3.0) * assign65290_body20_e101173) + (assign65290_body20_e101162 * (-(((locals.var_chi_dn11 / 4.0) * assign65290_body20_e101171) + (assign65290_body20_e101166 * (-(locals.var_chi_dn11 / 5.0)))))))))), (((((locals.var_chi_dn14 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn14)) / 2.0) * assign65290_body20_e101175) + (assign65290_body20_e101158 * (-(((locals.var_chi_dn14 / 3.0) * assign65290_body20_e101173) + (assign65290_body20_e101162 * (-(((locals.var_chi_dn14 / 4.0) * assign65290_body20_e101171) + (assign65290_body20_e101166 * (-(locals.var_chi_dn14 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign65290_body20_e101178;
            locals.var_t0_dn0 = assign65290_body20_e101178_d_n0;
            locals.var_t0_dn2 = assign65290_body20_e101178_d_n2;
            locals.var_t0_dn4 = assign65290_body20_e101178_d_n4;
            locals.var_t0_dn5 = assign65290_body20_e101178_d_n5;
            locals.var_t0_dn6 = assign65290_body20_e101178_d_n6;
            locals.var_t0_dn7 = assign65290_body20_e101178_d_n7;
            locals.var_t0_dn8 = assign65290_body20_e101178_d_n8;
            locals.var_t0_dn9 = assign65290_body20_e101178_d_n9;
            locals.var_t0_dn10 = assign65290_body20_e101178_d_n10;
            locals.var_t0_dn11 = assign65290_body20_e101178_d_n11;
            locals.var_t0_dn14 = assign65290_body20_e101178_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign65290_body21_e101216, assign65290_body21_e101216_d_n0, assign65290_body21_e101216_d_n2, assign65290_body21_e101216_d_n4, assign65290_body21_e101216_d_n5, assign65290_body21_e101216_d_n6, assign65290_body21_e101216_d_n7, assign65290_body21_e101216_d_n8, assign65290_body21_e101216_d_n9, assign65290_body21_e101216_d_n10, assign65290_body21_e101216_d_n11, assign65290_body21_e101216_d_n14,) = {
    if (((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1559 == 0.0)) && (locals.var_guard1561 != 0.0)) {
        let assign65290_body21_e101200: f64 = (locals.var_chi / 2.0);
        let assign65290_body21_e101204: f64 = (locals.var_chi / 3.0);
        let assign65290_body21_e101208: f64 = (locals.var_chi / 4.0);
        let assign65290_body21_e101209: f64 = (1.0 - assign65290_body21_e101208);
        let assign65290_body21_e101210: f64 = (assign65290_body21_e101204 * assign65290_body21_e101209);
        let assign65290_body21_e101211: f64 = (1.0 - assign65290_body21_e101210);
        let assign65290_body21_e101212: f64 = (assign65290_body21_e101200 * assign65290_body21_e101211);
        let assign65290_body21_e101213: f64 = (1.0 - assign65290_body21_e101212);
        let assign65290_body21_e101214: f64 = (locals.var_chi * assign65290_body21_e101213);
        (assign65290_body21_e101214, ((locals.var_chi_dn0 * assign65290_body21_e101213) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign65290_body21_e101211) + (assign65290_body21_e101200 * (-(((locals.var_chi_dn0 / 3.0) * assign65290_body21_e101209) + (assign65290_body21_e101204 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign65290_body21_e101213) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign65290_body21_e101211) + (assign65290_body21_e101200 * (-(((locals.var_chi_dn2 / 3.0) * assign65290_body21_e101209) + (assign65290_body21_e101204 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn4 * assign65290_body21_e101213) + (locals.var_chi * (-(((locals.var_chi_dn4 / 2.0) * assign65290_body21_e101211) + (assign65290_body21_e101200 * (-(((locals.var_chi_dn4 / 3.0) * assign65290_body21_e101209) + (assign65290_body21_e101204 * (-(locals.var_chi_dn4 / 4.0)))))))))), ((locals.var_chi_dn5 * assign65290_body21_e101213) + (locals.var_chi * (-(((locals.var_chi_dn5 / 2.0) * assign65290_body21_e101211) + (assign65290_body21_e101200 * (-(((locals.var_chi_dn5 / 3.0) * assign65290_body21_e101209) + (assign65290_body21_e101204 * (-(locals.var_chi_dn5 / 4.0)))))))))), ((locals.var_chi_dn6 * assign65290_body21_e101213) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign65290_body21_e101211) + (assign65290_body21_e101200 * (-(((locals.var_chi_dn6 / 3.0) * assign65290_body21_e101209) + (assign65290_body21_e101204 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign65290_body21_e101213) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign65290_body21_e101211) + (assign65290_body21_e101200 * (-(((locals.var_chi_dn7 / 3.0) * assign65290_body21_e101209) + (assign65290_body21_e101204 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn8 * assign65290_body21_e101213) + (locals.var_chi * (-(((locals.var_chi_dn8 / 2.0) * assign65290_body21_e101211) + (assign65290_body21_e101200 * (-(((locals.var_chi_dn8 / 3.0) * assign65290_body21_e101209) + (assign65290_body21_e101204 * (-(locals.var_chi_dn8 / 4.0)))))))))), ((locals.var_chi_dn9 * assign65290_body21_e101213) + (locals.var_chi * (-(((locals.var_chi_dn9 / 2.0) * assign65290_body21_e101211) + (assign65290_body21_e101200 * (-(((locals.var_chi_dn9 / 3.0) * assign65290_body21_e101209) + (assign65290_body21_e101204 * (-(locals.var_chi_dn9 / 4.0)))))))))), ((locals.var_chi_dn10 * assign65290_body21_e101213) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign65290_body21_e101211) + (assign65290_body21_e101200 * (-(((locals.var_chi_dn10 / 3.0) * assign65290_body21_e101209) + (assign65290_body21_e101204 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn11 * assign65290_body21_e101213) + (locals.var_chi * (-(((locals.var_chi_dn11 / 2.0) * assign65290_body21_e101211) + (assign65290_body21_e101200 * (-(((locals.var_chi_dn11 / 3.0) * assign65290_body21_e101209) + (assign65290_body21_e101204 * (-(locals.var_chi_dn11 / 4.0)))))))))), ((locals.var_chi_dn14 * assign65290_body21_e101213) + (locals.var_chi * (-(((locals.var_chi_dn14 / 2.0) * assign65290_body21_e101211) + (assign65290_body21_e101200 * (-(((locals.var_chi_dn14 / 3.0) * assign65290_body21_e101209) + (assign65290_body21_e101204 * (-(locals.var_chi_dn14 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign65290_body21_e101216;
            locals.var_t1_dn0 = assign65290_body21_e101216_d_n0;
            locals.var_t1_dn2 = assign65290_body21_e101216_d_n2;
            locals.var_t1_dn4 = assign65290_body21_e101216_d_n4;
            locals.var_t1_dn5 = assign65290_body21_e101216_d_n5;
            locals.var_t1_dn6 = assign65290_body21_e101216_d_n6;
            locals.var_t1_dn7 = assign65290_body21_e101216_d_n7;
            locals.var_t1_dn8 = assign65290_body21_e101216_d_n8;
            locals.var_t1_dn9 = assign65290_body21_e101216_d_n9;
            locals.var_t1_dn10 = assign65290_body21_e101216_d_n10;
            locals.var_t1_dn11 = assign65290_body21_e101216_d_n11;
            locals.var_t1_dn14 = assign65290_body21_e101216_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign65290_body22_e101258, assign65290_body22_e101258_d_n0, assign65290_body22_e101258_d_n2, assign65290_body22_e101258_d_n4, assign65290_body22_e101258_d_n5, assign65290_body22_e101258_d_n6, assign65290_body22_e101258_d_n7, assign65290_body22_e101258_d_n8, assign65290_body22_e101258_d_n9, assign65290_body22_e101258_d_n10, assign65290_body22_e101258_d_n11, assign65290_body22_e101258_d_n14,) = {
    if (((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1559 == 0.0)) && (locals.var_guard1561 != 0.0)) {
        let assign65290_body22_e101236: f64 = (locals.var_chib__blk1552 * locals.var_chib__blk1552);
        let assign65290_body22_e101238: f64 = (assign65290_body22_e101236 / 2.0);
        let assign65290_body22_e101242: f64 = (locals.var_chib__blk1552 / 3.0);
        let assign65290_body22_e101246: f64 = (locals.var_chib__blk1552 / 4.0);
        let assign65290_body22_e101250: f64 = (locals.var_chib__blk1552 / 5.0);
        let assign65290_body22_e101251: f64 = (1.0 - assign65290_body22_e101250);
        let assign65290_body22_e101252: f64 = (assign65290_body22_e101246 * assign65290_body22_e101251);
        let assign65290_body22_e101253: f64 = (1.0 - assign65290_body22_e101252);
        let assign65290_body22_e101254: f64 = (assign65290_body22_e101242 * assign65290_body22_e101253);
        let assign65290_body22_e101255: f64 = (1.0 - assign65290_body22_e101254);
        let assign65290_body22_e101256: f64 = (assign65290_body22_e101238 * assign65290_body22_e101255);
        (assign65290_body22_e101256, (((((locals.var_chib__blk1552_dn0 * locals.var_chib__blk1552) + (locals.var_chib__blk1552 * locals.var_chib__blk1552_dn0)) / 2.0) * assign65290_body22_e101255) + (assign65290_body22_e101238 * (-(((locals.var_chib__blk1552_dn0 / 3.0) * assign65290_body22_e101253) + (assign65290_body22_e101242 * (-(((locals.var_chib__blk1552_dn0 / 4.0) * assign65290_body22_e101251) + (assign65290_body22_e101246 * (-(locals.var_chib__blk1552_dn0 / 5.0)))))))))), (((((locals.var_chib__blk1552_dn2 * locals.var_chib__blk1552) + (locals.var_chib__blk1552 * locals.var_chib__blk1552_dn2)) / 2.0) * assign65290_body22_e101255) + (assign65290_body22_e101238 * (-(((locals.var_chib__blk1552_dn2 / 3.0) * assign65290_body22_e101253) + (assign65290_body22_e101242 * (-(((locals.var_chib__blk1552_dn2 / 4.0) * assign65290_body22_e101251) + (assign65290_body22_e101246 * (-(locals.var_chib__blk1552_dn2 / 5.0)))))))))), (((((locals.var_chib__blk1552_dn4 * locals.var_chib__blk1552) + (locals.var_chib__blk1552 * locals.var_chib__blk1552_dn4)) / 2.0) * assign65290_body22_e101255) + (assign65290_body22_e101238 * (-(((locals.var_chib__blk1552_dn4 / 3.0) * assign65290_body22_e101253) + (assign65290_body22_e101242 * (-(((locals.var_chib__blk1552_dn4 / 4.0) * assign65290_body22_e101251) + (assign65290_body22_e101246 * (-(locals.var_chib__blk1552_dn4 / 5.0)))))))))), (((((locals.var_chib__blk1552_dn5 * locals.var_chib__blk1552) + (locals.var_chib__blk1552 * locals.var_chib__blk1552_dn5)) / 2.0) * assign65290_body22_e101255) + (assign65290_body22_e101238 * (-(((locals.var_chib__blk1552_dn5 / 3.0) * assign65290_body22_e101253) + (assign65290_body22_e101242 * (-(((locals.var_chib__blk1552_dn5 / 4.0) * assign65290_body22_e101251) + (assign65290_body22_e101246 * (-(locals.var_chib__blk1552_dn5 / 5.0)))))))))), (((((locals.var_chib__blk1552_dn6 * locals.var_chib__blk1552) + (locals.var_chib__blk1552 * locals.var_chib__blk1552_dn6)) / 2.0) * assign65290_body22_e101255) + (assign65290_body22_e101238 * (-(((locals.var_chib__blk1552_dn6 / 3.0) * assign65290_body22_e101253) + (assign65290_body22_e101242 * (-(((locals.var_chib__blk1552_dn6 / 4.0) * assign65290_body22_e101251) + (assign65290_body22_e101246 * (-(locals.var_chib__blk1552_dn6 / 5.0)))))))))), (((((locals.var_chib__blk1552_dn7 * locals.var_chib__blk1552) + (locals.var_chib__blk1552 * locals.var_chib__blk1552_dn7)) / 2.0) * assign65290_body22_e101255) + (assign65290_body22_e101238 * (-(((locals.var_chib__blk1552_dn7 / 3.0) * assign65290_body22_e101253) + (assign65290_body22_e101242 * (-(((locals.var_chib__blk1552_dn7 / 4.0) * assign65290_body22_e101251) + (assign65290_body22_e101246 * (-(locals.var_chib__blk1552_dn7 / 5.0)))))))))), (((((locals.var_chib__blk1552_dn8 * locals.var_chib__blk1552) + (locals.var_chib__blk1552 * locals.var_chib__blk1552_dn8)) / 2.0) * assign65290_body22_e101255) + (assign65290_body22_e101238 * (-(((locals.var_chib__blk1552_dn8 / 3.0) * assign65290_body22_e101253) + (assign65290_body22_e101242 * (-(((locals.var_chib__blk1552_dn8 / 4.0) * assign65290_body22_e101251) + (assign65290_body22_e101246 * (-(locals.var_chib__blk1552_dn8 / 5.0)))))))))), (((((locals.var_chib__blk1552_dn9 * locals.var_chib__blk1552) + (locals.var_chib__blk1552 * locals.var_chib__blk1552_dn9)) / 2.0) * assign65290_body22_e101255) + (assign65290_body22_e101238 * (-(((locals.var_chib__blk1552_dn9 / 3.0) * assign65290_body22_e101253) + (assign65290_body22_e101242 * (-(((locals.var_chib__blk1552_dn9 / 4.0) * assign65290_body22_e101251) + (assign65290_body22_e101246 * (-(locals.var_chib__blk1552_dn9 / 5.0)))))))))), (((((locals.var_chib__blk1552_dn10 * locals.var_chib__blk1552) + (locals.var_chib__blk1552 * locals.var_chib__blk1552_dn10)) / 2.0) * assign65290_body22_e101255) + (assign65290_body22_e101238 * (-(((locals.var_chib__blk1552_dn10 / 3.0) * assign65290_body22_e101253) + (assign65290_body22_e101242 * (-(((locals.var_chib__blk1552_dn10 / 4.0) * assign65290_body22_e101251) + (assign65290_body22_e101246 * (-(locals.var_chib__blk1552_dn10 / 5.0)))))))))), (((((locals.var_chib__blk1552_dn11 * locals.var_chib__blk1552) + (locals.var_chib__blk1552 * locals.var_chib__blk1552_dn11)) / 2.0) * assign65290_body22_e101255) + (assign65290_body22_e101238 * (-(((locals.var_chib__blk1552_dn11 / 3.0) * assign65290_body22_e101253) + (assign65290_body22_e101242 * (-(((locals.var_chib__blk1552_dn11 / 4.0) * assign65290_body22_e101251) + (assign65290_body22_e101246 * (-(locals.var_chib__blk1552_dn11 / 5.0)))))))))), (((((locals.var_chib__blk1552_dn14 * locals.var_chib__blk1552) + (locals.var_chib__blk1552 * locals.var_chib__blk1552_dn14)) / 2.0) * assign65290_body22_e101255) + (assign65290_body22_e101238 * (-(((locals.var_chib__blk1552_dn14 / 3.0) * assign65290_body22_e101253) + (assign65290_body22_e101242 * (-(((locals.var_chib__blk1552_dn14 / 4.0) * assign65290_body22_e101251) + (assign65290_body22_e101246 * (-(locals.var_chib__blk1552_dn14 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign65290_body22_e101258;
            locals.var_t2_dn0 = assign65290_body22_e101258_d_n0;
            locals.var_t2_dn2 = assign65290_body22_e101258_d_n2;
            locals.var_t2_dn4 = assign65290_body22_e101258_d_n4;
            locals.var_t2_dn5 = assign65290_body22_e101258_d_n5;
            locals.var_t2_dn6 = assign65290_body22_e101258_d_n6;
            locals.var_t2_dn7 = assign65290_body22_e101258_d_n7;
            locals.var_t2_dn8 = assign65290_body22_e101258_d_n8;
            locals.var_t2_dn9 = assign65290_body22_e101258_d_n9;
            locals.var_t2_dn10 = assign65290_body22_e101258_d_n10;
            locals.var_t2_dn11 = assign65290_body22_e101258_d_n11;
            locals.var_t2_dn14 = assign65290_body22_e101258_d_n14;
            locals.var_t2_rv = 0.0;
            let (assign65290_body23_e101296, assign65290_body23_e101296_d_n0, assign65290_body23_e101296_d_n2, assign65290_body23_e101296_d_n4, assign65290_body23_e101296_d_n5, assign65290_body23_e101296_d_n6, assign65290_body23_e101296_d_n7, assign65290_body23_e101296_d_n8, assign65290_body23_e101296_d_n9, assign65290_body23_e101296_d_n10, assign65290_body23_e101296_d_n11, assign65290_body23_e101296_d_n14,) = {
    if (((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1559 == 0.0)) && (locals.var_guard1561 != 0.0)) {
        let assign65290_body23_e101280: f64 = (locals.var_chib__blk1552 / 2.0);
        let assign65290_body23_e101284: f64 = (locals.var_chib__blk1552 / 3.0);
        let assign65290_body23_e101288: f64 = (locals.var_chib__blk1552 / 4.0);
        let assign65290_body23_e101289: f64 = (1.0 - assign65290_body23_e101288);
        let assign65290_body23_e101290: f64 = (assign65290_body23_e101284 * assign65290_body23_e101289);
        let assign65290_body23_e101291: f64 = (1.0 - assign65290_body23_e101290);
        let assign65290_body23_e101292: f64 = (assign65290_body23_e101280 * assign65290_body23_e101291);
        let assign65290_body23_e101293: f64 = (1.0 - assign65290_body23_e101292);
        let assign65290_body23_e101294: f64 = (locals.var_chib__blk1552 * assign65290_body23_e101293);
        (assign65290_body23_e101294, ((locals.var_chib__blk1552_dn0 * assign65290_body23_e101293) + (locals.var_chib__blk1552 * (-(((locals.var_chib__blk1552_dn0 / 2.0) * assign65290_body23_e101291) + (assign65290_body23_e101280 * (-(((locals.var_chib__blk1552_dn0 / 3.0) * assign65290_body23_e101289) + (assign65290_body23_e101284 * (-(locals.var_chib__blk1552_dn0 / 4.0)))))))))), ((locals.var_chib__blk1552_dn2 * assign65290_body23_e101293) + (locals.var_chib__blk1552 * (-(((locals.var_chib__blk1552_dn2 / 2.0) * assign65290_body23_e101291) + (assign65290_body23_e101280 * (-(((locals.var_chib__blk1552_dn2 / 3.0) * assign65290_body23_e101289) + (assign65290_body23_e101284 * (-(locals.var_chib__blk1552_dn2 / 4.0)))))))))), ((locals.var_chib__blk1552_dn4 * assign65290_body23_e101293) + (locals.var_chib__blk1552 * (-(((locals.var_chib__blk1552_dn4 / 2.0) * assign65290_body23_e101291) + (assign65290_body23_e101280 * (-(((locals.var_chib__blk1552_dn4 / 3.0) * assign65290_body23_e101289) + (assign65290_body23_e101284 * (-(locals.var_chib__blk1552_dn4 / 4.0)))))))))), ((locals.var_chib__blk1552_dn5 * assign65290_body23_e101293) + (locals.var_chib__blk1552 * (-(((locals.var_chib__blk1552_dn5 / 2.0) * assign65290_body23_e101291) + (assign65290_body23_e101280 * (-(((locals.var_chib__blk1552_dn5 / 3.0) * assign65290_body23_e101289) + (assign65290_body23_e101284 * (-(locals.var_chib__blk1552_dn5 / 4.0)))))))))), ((locals.var_chib__blk1552_dn6 * assign65290_body23_e101293) + (locals.var_chib__blk1552 * (-(((locals.var_chib__blk1552_dn6 / 2.0) * assign65290_body23_e101291) + (assign65290_body23_e101280 * (-(((locals.var_chib__blk1552_dn6 / 3.0) * assign65290_body23_e101289) + (assign65290_body23_e101284 * (-(locals.var_chib__blk1552_dn6 / 4.0)))))))))), ((locals.var_chib__blk1552_dn7 * assign65290_body23_e101293) + (locals.var_chib__blk1552 * (-(((locals.var_chib__blk1552_dn7 / 2.0) * assign65290_body23_e101291) + (assign65290_body23_e101280 * (-(((locals.var_chib__blk1552_dn7 / 3.0) * assign65290_body23_e101289) + (assign65290_body23_e101284 * (-(locals.var_chib__blk1552_dn7 / 4.0)))))))))), ((locals.var_chib__blk1552_dn8 * assign65290_body23_e101293) + (locals.var_chib__blk1552 * (-(((locals.var_chib__blk1552_dn8 / 2.0) * assign65290_body23_e101291) + (assign65290_body23_e101280 * (-(((locals.var_chib__blk1552_dn8 / 3.0) * assign65290_body23_e101289) + (assign65290_body23_e101284 * (-(locals.var_chib__blk1552_dn8 / 4.0)))))))))), ((locals.var_chib__blk1552_dn9 * assign65290_body23_e101293) + (locals.var_chib__blk1552 * (-(((locals.var_chib__blk1552_dn9 / 2.0) * assign65290_body23_e101291) + (assign65290_body23_e101280 * (-(((locals.var_chib__blk1552_dn9 / 3.0) * assign65290_body23_e101289) + (assign65290_body23_e101284 * (-(locals.var_chib__blk1552_dn9 / 4.0)))))))))), ((locals.var_chib__blk1552_dn10 * assign65290_body23_e101293) + (locals.var_chib__blk1552 * (-(((locals.var_chib__blk1552_dn10 / 2.0) * assign65290_body23_e101291) + (assign65290_body23_e101280 * (-(((locals.var_chib__blk1552_dn10 / 3.0) * assign65290_body23_e101289) + (assign65290_body23_e101284 * (-(locals.var_chib__blk1552_dn10 / 4.0)))))))))), ((locals.var_chib__blk1552_dn11 * assign65290_body23_e101293) + (locals.var_chib__blk1552 * (-(((locals.var_chib__blk1552_dn11 / 2.0) * assign65290_body23_e101291) + (assign65290_body23_e101280 * (-(((locals.var_chib__blk1552_dn11 / 3.0) * assign65290_body23_e101289) + (assign65290_body23_e101284 * (-(locals.var_chib__blk1552_dn11 / 4.0)))))))))), ((locals.var_chib__blk1552_dn14 * assign65290_body23_e101293) + (locals.var_chib__blk1552 * (-(((locals.var_chib__blk1552_dn14 / 2.0) * assign65290_body23_e101291) + (assign65290_body23_e101280 * (-(((locals.var_chib__blk1552_dn14 / 3.0) * assign65290_body23_e101289) + (assign65290_body23_e101284 * (-(locals.var_chib__blk1552_dn14 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
            locals.var_t3 = assign65290_body23_e101296;
            locals.var_t3_dn0 = assign65290_body23_e101296_d_n0;
            locals.var_t3_dn2 = assign65290_body23_e101296_d_n2;
            locals.var_t3_dn4 = assign65290_body23_e101296_d_n4;
            locals.var_t3_dn5 = assign65290_body23_e101296_d_n5;
            locals.var_t3_dn6 = assign65290_body23_e101296_d_n6;
            locals.var_t3_dn7 = assign65290_body23_e101296_d_n7;
            locals.var_t3_dn8 = assign65290_body23_e101296_d_n8;
            locals.var_t3_dn9 = assign65290_body23_e101296_d_n9;
            locals.var_t3_dn10 = assign65290_body23_e101296_d_n10;
            locals.var_t3_dn11 = assign65290_body23_e101296_d_n11;
            locals.var_t3_dn14 = assign65290_body23_e101296_d_n14;
            locals.var_t3_rv = 0.0;
            let (assign65290_body24_e101319, assign65290_body24_e101319_d_n0, assign65290_body24_e101319_d_n2, assign65290_body24_e101319_d_n4, assign65290_body24_e101319_d_n5, assign65290_body24_e101319_d_n6, assign65290_body24_e101319_d_n7, assign65290_body24_e101319_d_n8, assign65290_body24_e101319_d_n9, assign65290_body24_e101319_d_n10, assign65290_body24_e101319_d_n11, assign65290_body24_e101319_d_n14,) = {
    if (((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1559 == 0.0)) && (locals.var_guard1561 != 0.0)) {
        let assign65290_body24_e101316: f64 = (locals.var_t0 - locals.var_t2);
        let assign65290_body24_e101317: f64 = (assign65290_body24_e101316).sqrt();
        (assign65290_body24_e101317, ((locals.var_t0_dn0 - locals.var_t2_dn0) / (2.0 * assign65290_body24_e101317)), ((locals.var_t0_dn2 - locals.var_t2_dn2) / (2.0 * assign65290_body24_e101317)), ((locals.var_t0_dn4 - locals.var_t2_dn4) / (2.0 * assign65290_body24_e101317)), ((locals.var_t0_dn5 - locals.var_t2_dn5) / (2.0 * assign65290_body24_e101317)), ((locals.var_t0_dn6 - locals.var_t2_dn6) / (2.0 * assign65290_body24_e101317)), ((locals.var_t0_dn7 - locals.var_t2_dn7) / (2.0 * assign65290_body24_e101317)), ((locals.var_t0_dn8 - locals.var_t2_dn8) / (2.0 * assign65290_body24_e101317)), ((locals.var_t0_dn9 - locals.var_t2_dn9) / (2.0 * assign65290_body24_e101317)), ((locals.var_t0_dn10 - locals.var_t2_dn10) / (2.0 * assign65290_body24_e101317)), ((locals.var_t0_dn11 - locals.var_t2_dn11) / (2.0 * assign65290_body24_e101317)), ((locals.var_t0_dn14 - locals.var_t2_dn14) / (2.0 * assign65290_body24_e101317)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign65290_body24_e101319;
            locals.var_fb_dn0 = assign65290_body24_e101319_d_n0;
            locals.var_fb_dn2 = assign65290_body24_e101319_d_n2;
            locals.var_fb_dn4 = assign65290_body24_e101319_d_n4;
            locals.var_fb_dn5 = assign65290_body24_e101319_d_n5;
            locals.var_fb_dn6 = assign65290_body24_e101319_d_n6;
            locals.var_fb_dn7 = assign65290_body24_e101319_d_n7;
            locals.var_fb_dn8 = assign65290_body24_e101319_d_n8;
            locals.var_fb_dn9 = assign65290_body24_e101319_d_n9;
            locals.var_fb_dn10 = assign65290_body24_e101319_d_n10;
            locals.var_fb_dn11 = assign65290_body24_e101319_d_n11;
            locals.var_fb_dn14 = assign65290_body24_e101319_d_n14;
            locals.var_fb_rv = 0.0;
            let (assign65290_body25_e101349, assign65290_body25_e101349_d_n0, assign65290_body25_e101349_d_n2, assign65290_body25_e101349_d_n4, assign65290_body25_e101349_d_n5, assign65290_body25_e101349_d_n6, assign65290_body25_e101349_d_n7, assign65290_body25_e101349_d_n8, assign65290_body25_e101349_d_n9, assign65290_body25_e101349_d_n10, assign65290_body25_e101349_d_n11, assign65290_body25_e101349_d_n14,) = {
    if (((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1559 == 0.0)) && (locals.var_guard1561 != 0.0)) {
        let assign65290_body25_e101339: f64 = (locals.var_beta * 0.5);
        let assign65290_body25_e101343: f64 = (locals.var_phi_b_dpss__blk1554 * locals.var_t3);
        let assign65290_body25_e101344: f64 = (locals.var_t1 - assign65290_body25_e101343);
        let assign65290_body25_e101345: f64 = (assign65290_body25_e101339 * assign65290_body25_e101344);
        let assign65290_body25_e101347: f64 = (assign65290_body25_e101345 / locals.var_fb);
        (assign65290_body25_e101347, ((((((locals.var_beta_dn0 * 0.5) * assign65290_body25_e101344) + (assign65290_body25_e101339 * (locals.var_t1_dn0 - ((locals.var_phi_b_dpss__blk1554_dn0 * locals.var_t3) + (locals.var_phi_b_dpss__blk1554 * locals.var_t3_dn0))))) * locals.var_fb) - (assign65290_body25_e101345 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign65290_body25_e101344) + (assign65290_body25_e101339 * (locals.var_t1_dn2 - ((locals.var_phi_b_dpss__blk1554_dn2 * locals.var_t3) + (locals.var_phi_b_dpss__blk1554 * locals.var_t3_dn2))))) * locals.var_fb) - (assign65290_body25_e101345 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign65290_body25_e101344) + (assign65290_body25_e101339 * (locals.var_t1_dn4 - ((locals.var_phi_b_dpss__blk1554_dn4 * locals.var_t3) + (locals.var_phi_b_dpss__blk1554 * locals.var_t3_dn4))))) * locals.var_fb) - (assign65290_body25_e101345 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign65290_body25_e101344) + (assign65290_body25_e101339 * (locals.var_t1_dn5 - ((locals.var_phi_b_dpss__blk1554_dn5 * locals.var_t3) + (locals.var_phi_b_dpss__blk1554 * locals.var_t3_dn5))))) * locals.var_fb) - (assign65290_body25_e101345 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign65290_body25_e101344) + (assign65290_body25_e101339 * (locals.var_t1_dn6 - ((locals.var_phi_b_dpss__blk1554_dn6 * locals.var_t3) + (locals.var_phi_b_dpss__blk1554 * locals.var_t3_dn6))))) * locals.var_fb) - (assign65290_body25_e101345 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign65290_body25_e101344) + (assign65290_body25_e101339 * (locals.var_t1_dn7 - ((locals.var_phi_b_dpss__blk1554_dn7 * locals.var_t3) + (locals.var_phi_b_dpss__blk1554 * locals.var_t3_dn7))))) * locals.var_fb) - (assign65290_body25_e101345 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign65290_body25_e101344) + (assign65290_body25_e101339 * (locals.var_t1_dn8 - ((locals.var_phi_b_dpss__blk1554_dn8 * locals.var_t3) + (locals.var_phi_b_dpss__blk1554 * locals.var_t3_dn8))))) * locals.var_fb) - (assign65290_body25_e101345 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign65290_body25_e101344) + (assign65290_body25_e101339 * (locals.var_t1_dn9 - ((locals.var_phi_b_dpss__blk1554_dn9 * locals.var_t3) + (locals.var_phi_b_dpss__blk1554 * locals.var_t3_dn9))))) * locals.var_fb) - (assign65290_body25_e101345 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign65290_body25_e101344) + (assign65290_body25_e101339 * (locals.var_t1_dn10 - ((locals.var_phi_b_dpss__blk1554_dn10 * locals.var_t3) + (locals.var_phi_b_dpss__blk1554 * locals.var_t3_dn10))))) * locals.var_fb) - (assign65290_body25_e101345 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn11 * 0.5) * assign65290_body25_e101344) + (assign65290_body25_e101339 * (locals.var_t1_dn11 - ((locals.var_phi_b_dpss__blk1554_dn11 * locals.var_t3) + (locals.var_phi_b_dpss__blk1554 * locals.var_t3_dn11))))) * locals.var_fb) - (assign65290_body25_e101345 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn14 * 0.5) * assign65290_body25_e101344) + (assign65290_body25_e101339 * (locals.var_t1_dn14 - ((locals.var_phi_b_dpss__blk1554_dn14 * locals.var_t3) + (locals.var_phi_b_dpss__blk1554 * locals.var_t3_dn14))))) * locals.var_fb) - (assign65290_body25_e101345 * locals.var_fb_dn14)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss__blk1555, locals.var_fb_dpss__blk1555_dn0, locals.var_fb_dpss__blk1555_dn2, locals.var_fb_dpss__blk1555_dn4, locals.var_fb_dpss__blk1555_dn5, locals.var_fb_dpss__blk1555_dn6, locals.var_fb_dpss__blk1555_dn7, locals.var_fb_dpss__blk1555_dn8, locals.var_fb_dpss__blk1555_dn9, locals.var_fb_dpss__blk1555_dn10, locals.var_fb_dpss__blk1555_dn11, locals.var_fb_dpss__blk1555_dn14,)
    }
};
            locals.var_fb_dpss__blk1555 = assign65290_body25_e101349;
            locals.var_fb_dpss__blk1555_dn0 = assign65290_body25_e101349_d_n0;
            locals.var_fb_dpss__blk1555_dn2 = assign65290_body25_e101349_d_n2;
            locals.var_fb_dpss__blk1555_dn4 = assign65290_body25_e101349_d_n4;
            locals.var_fb_dpss__blk1555_dn5 = assign65290_body25_e101349_d_n5;
            locals.var_fb_dpss__blk1555_dn6 = assign65290_body25_e101349_d_n6;
            locals.var_fb_dpss__blk1555_dn7 = assign65290_body25_e101349_d_n7;
            locals.var_fb_dpss__blk1555_dn8 = assign65290_body25_e101349_d_n8;
            locals.var_fb_dpss__blk1555_dn9 = assign65290_body25_e101349_d_n9;
            locals.var_fb_dpss__blk1555_dn10 = assign65290_body25_e101349_d_n10;
            locals.var_fb_dpss__blk1555_dn11 = assign65290_body25_e101349_d_n11;
            locals.var_fb_dpss__blk1555_dn14 = assign65290_body25_e101349_d_n14;
            locals.var_fb_dpss__blk1555_rv = 0.0;
            let (assign65290_body26_e101372, assign65290_body26_e101372_d_n0, assign65290_body26_e101372_d_n2, assign65290_body26_e101372_d_n4, assign65290_body26_e101372_d_n5, assign65290_body26_e101372_d_n6, assign65290_body26_e101372_d_n7, assign65290_body26_e101372_d_n8, assign65290_body26_e101372_d_n9, assign65290_body26_e101372_d_n10, assign65290_body26_e101372_d_n11, assign65290_body26_e101372_d_n14,) = {
    if (((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1559 == 0.0)) && (locals.var_guard1561 == 0.0)) {
        let assign65290_body26_e101369: f64 = (-locals.var_chi);
        let assign65290_body26_e101370: f64 = (assign65290_body26_e101369).exp();
        (assign65290_body26_e101370, (assign65290_body26_e101370 * (-locals.var_chi_dn0)), (assign65290_body26_e101370 * (-locals.var_chi_dn2)), (assign65290_body26_e101370 * (-locals.var_chi_dn4)), (assign65290_body26_e101370 * (-locals.var_chi_dn5)), (assign65290_body26_e101370 * (-locals.var_chi_dn6)), (assign65290_body26_e101370 * (-locals.var_chi_dn7)), (assign65290_body26_e101370 * (-locals.var_chi_dn8)), (assign65290_body26_e101370 * (-locals.var_chi_dn9)), (assign65290_body26_e101370 * (-locals.var_chi_dn10)), (assign65290_body26_e101370 * (-locals.var_chi_dn11)), (assign65290_body26_e101370 * (-locals.var_chi_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign65290_body26_e101372;
            locals.var_t0_dn0 = assign65290_body26_e101372_d_n0;
            locals.var_t0_dn2 = assign65290_body26_e101372_d_n2;
            locals.var_t0_dn4 = assign65290_body26_e101372_d_n4;
            locals.var_t0_dn5 = assign65290_body26_e101372_d_n5;
            locals.var_t0_dn6 = assign65290_body26_e101372_d_n6;
            locals.var_t0_dn7 = assign65290_body26_e101372_d_n7;
            locals.var_t0_dn8 = assign65290_body26_e101372_d_n8;
            locals.var_t0_dn9 = assign65290_body26_e101372_d_n9;
            locals.var_t0_dn10 = assign65290_body26_e101372_d_n10;
            locals.var_t0_dn11 = assign65290_body26_e101372_d_n11;
            locals.var_t0_dn14 = assign65290_body26_e101372_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign65290_body27_e101395, assign65290_body27_e101395_d_n0, assign65290_body27_e101395_d_n2, assign65290_body27_e101395_d_n4, assign65290_body27_e101395_d_n5, assign65290_body27_e101395_d_n6, assign65290_body27_e101395_d_n7, assign65290_body27_e101395_d_n8, assign65290_body27_e101395_d_n9, assign65290_body27_e101395_d_n10, assign65290_body27_e101395_d_n11, assign65290_body27_e101395_d_n14,) = {
    if (((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1559 == 0.0)) && (locals.var_guard1561 == 0.0)) {
        let assign65290_body27_e101392: f64 = (-locals.var_chib__blk1552);
        let assign65290_body27_e101393: f64 = (assign65290_body27_e101392).exp();
        (assign65290_body27_e101393, (assign65290_body27_e101393 * (-locals.var_chib__blk1552_dn0)), (assign65290_body27_e101393 * (-locals.var_chib__blk1552_dn2)), (assign65290_body27_e101393 * (-locals.var_chib__blk1552_dn4)), (assign65290_body27_e101393 * (-locals.var_chib__blk1552_dn5)), (assign65290_body27_e101393 * (-locals.var_chib__blk1552_dn6)), (assign65290_body27_e101393 * (-locals.var_chib__blk1552_dn7)), (assign65290_body27_e101393 * (-locals.var_chib__blk1552_dn8)), (assign65290_body27_e101393 * (-locals.var_chib__blk1552_dn9)), (assign65290_body27_e101393 * (-locals.var_chib__blk1552_dn10)), (assign65290_body27_e101393 * (-locals.var_chib__blk1552_dn11)), (assign65290_body27_e101393 * (-locals.var_chib__blk1552_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign65290_body27_e101395;
            locals.var_t1_dn0 = assign65290_body27_e101395_d_n0;
            locals.var_t1_dn2 = assign65290_body27_e101395_d_n2;
            locals.var_t1_dn4 = assign65290_body27_e101395_d_n4;
            locals.var_t1_dn5 = assign65290_body27_e101395_d_n5;
            locals.var_t1_dn6 = assign65290_body27_e101395_d_n6;
            locals.var_t1_dn7 = assign65290_body27_e101395_d_n7;
            locals.var_t1_dn8 = assign65290_body27_e101395_d_n8;
            locals.var_t1_dn9 = assign65290_body27_e101395_d_n9;
            locals.var_t1_dn10 = assign65290_body27_e101395_d_n10;
            locals.var_t1_dn11 = assign65290_body27_e101395_d_n11;
            locals.var_t1_dn14 = assign65290_body27_e101395_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign65290_body28_e101423, assign65290_body28_e101423_d_n0, assign65290_body28_e101423_d_n2, assign65290_body28_e101423_d_n4, assign65290_body28_e101423_d_n5, assign65290_body28_e101423_d_n6, assign65290_body28_e101423_d_n7, assign65290_body28_e101423_d_n8, assign65290_body28_e101423_d_n9, assign65290_body28_e101423_d_n10, assign65290_body28_e101423_d_n11, assign65290_body28_e101423_d_n14,) = {
    if (((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1559 == 0.0)) && (locals.var_guard1561 == 0.0)) {
        let assign65290_body28_e101416: f64 = (locals.var_chi - locals.var_chib__blk1552);
        let assign65290_body28_e101419: f64 = (locals.var_t0 - locals.var_t1);
        let assign65290_body28_e101420: f64 = (assign65290_body28_e101416 + assign65290_body28_e101419);
        let assign65290_body28_e101421: f64 = (assign65290_body28_e101420).sqrt();
        (assign65290_body28_e101421, (((locals.var_chi_dn0 - locals.var_chib__blk1552_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)) / (2.0 * assign65290_body28_e101421)), (((locals.var_chi_dn2 - locals.var_chib__blk1552_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)) / (2.0 * assign65290_body28_e101421)), (((locals.var_chi_dn4 - locals.var_chib__blk1552_dn4) + (locals.var_t0_dn4 - locals.var_t1_dn4)) / (2.0 * assign65290_body28_e101421)), (((locals.var_chi_dn5 - locals.var_chib__blk1552_dn5) + (locals.var_t0_dn5 - locals.var_t1_dn5)) / (2.0 * assign65290_body28_e101421)), (((locals.var_chi_dn6 - locals.var_chib__blk1552_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)) / (2.0 * assign65290_body28_e101421)), (((locals.var_chi_dn7 - locals.var_chib__blk1552_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)) / (2.0 * assign65290_body28_e101421)), (((locals.var_chi_dn8 - locals.var_chib__blk1552_dn8) + (locals.var_t0_dn8 - locals.var_t1_dn8)) / (2.0 * assign65290_body28_e101421)), (((locals.var_chi_dn9 - locals.var_chib__blk1552_dn9) + (locals.var_t0_dn9 - locals.var_t1_dn9)) / (2.0 * assign65290_body28_e101421)), (((locals.var_chi_dn10 - locals.var_chib__blk1552_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)) / (2.0 * assign65290_body28_e101421)), (((locals.var_chi_dn11 - locals.var_chib__blk1552_dn11) + (locals.var_t0_dn11 - locals.var_t1_dn11)) / (2.0 * assign65290_body28_e101421)), (((locals.var_chi_dn14 - locals.var_chib__blk1552_dn14) + (locals.var_t0_dn14 - locals.var_t1_dn14)) / (2.0 * assign65290_body28_e101421)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign65290_body28_e101423;
            locals.var_fb_dn0 = assign65290_body28_e101423_d_n0;
            locals.var_fb_dn2 = assign65290_body28_e101423_d_n2;
            locals.var_fb_dn4 = assign65290_body28_e101423_d_n4;
            locals.var_fb_dn5 = assign65290_body28_e101423_d_n5;
            locals.var_fb_dn6 = assign65290_body28_e101423_d_n6;
            locals.var_fb_dn7 = assign65290_body28_e101423_d_n7;
            locals.var_fb_dn8 = assign65290_body28_e101423_d_n8;
            locals.var_fb_dn9 = assign65290_body28_e101423_d_n9;
            locals.var_fb_dn10 = assign65290_body28_e101423_d_n10;
            locals.var_fb_dn11 = assign65290_body28_e101423_d_n11;
            locals.var_fb_dn14 = assign65290_body28_e101423_d_n14;
            locals.var_fb_rv = 0.0;
            let (assign65290_body29_e101458, assign65290_body29_e101458_d_n0, assign65290_body29_e101458_d_n2, assign65290_body29_e101458_d_n4, assign65290_body29_e101458_d_n5, assign65290_body29_e101458_d_n6, assign65290_body29_e101458_d_n7, assign65290_body29_e101458_d_n8, assign65290_body29_e101458_d_n9, assign65290_body29_e101458_d_n10, assign65290_body29_e101458_d_n11, assign65290_body29_e101458_d_n14,) = {
    if (((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1559 == 0.0)) && (locals.var_guard1561 == 0.0)) {
        let assign65290_body29_e101444: f64 = (locals.var_beta * 0.5);
        let assign65290_body29_e101447: f64 = (1.0 - locals.var_t0);
        let assign65290_body29_e101451: f64 = (1.0 - locals.var_t1);
        let assign65290_body29_e101452: f64 = (locals.var_phi_b_dpss__blk1554 * assign65290_body29_e101451);
        let assign65290_body29_e101453: f64 = (assign65290_body29_e101447 - assign65290_body29_e101452);
        let assign65290_body29_e101454: f64 = (assign65290_body29_e101444 * assign65290_body29_e101453);
        let assign65290_body29_e101456: f64 = (assign65290_body29_e101454 / locals.var_fb);
        (assign65290_body29_e101456, ((((((locals.var_beta_dn0 * 0.5) * assign65290_body29_e101453) + (assign65290_body29_e101444 * ((-locals.var_t0_dn0) - ((locals.var_phi_b_dpss__blk1554_dn0 * assign65290_body29_e101451) + (locals.var_phi_b_dpss__blk1554 * (-locals.var_t1_dn0)))))) * locals.var_fb) - (assign65290_body29_e101454 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign65290_body29_e101453) + (assign65290_body29_e101444 * ((-locals.var_t0_dn2) - ((locals.var_phi_b_dpss__blk1554_dn2 * assign65290_body29_e101451) + (locals.var_phi_b_dpss__blk1554 * (-locals.var_t1_dn2)))))) * locals.var_fb) - (assign65290_body29_e101454 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign65290_body29_e101453) + (assign65290_body29_e101444 * ((-locals.var_t0_dn4) - ((locals.var_phi_b_dpss__blk1554_dn4 * assign65290_body29_e101451) + (locals.var_phi_b_dpss__blk1554 * (-locals.var_t1_dn4)))))) * locals.var_fb) - (assign65290_body29_e101454 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign65290_body29_e101453) + (assign65290_body29_e101444 * ((-locals.var_t0_dn5) - ((locals.var_phi_b_dpss__blk1554_dn5 * assign65290_body29_e101451) + (locals.var_phi_b_dpss__blk1554 * (-locals.var_t1_dn5)))))) * locals.var_fb) - (assign65290_body29_e101454 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign65290_body29_e101453) + (assign65290_body29_e101444 * ((-locals.var_t0_dn6) - ((locals.var_phi_b_dpss__blk1554_dn6 * assign65290_body29_e101451) + (locals.var_phi_b_dpss__blk1554 * (-locals.var_t1_dn6)))))) * locals.var_fb) - (assign65290_body29_e101454 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign65290_body29_e101453) + (assign65290_body29_e101444 * ((-locals.var_t0_dn7) - ((locals.var_phi_b_dpss__blk1554_dn7 * assign65290_body29_e101451) + (locals.var_phi_b_dpss__blk1554 * (-locals.var_t1_dn7)))))) * locals.var_fb) - (assign65290_body29_e101454 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign65290_body29_e101453) + (assign65290_body29_e101444 * ((-locals.var_t0_dn8) - ((locals.var_phi_b_dpss__blk1554_dn8 * assign65290_body29_e101451) + (locals.var_phi_b_dpss__blk1554 * (-locals.var_t1_dn8)))))) * locals.var_fb) - (assign65290_body29_e101454 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign65290_body29_e101453) + (assign65290_body29_e101444 * ((-locals.var_t0_dn9) - ((locals.var_phi_b_dpss__blk1554_dn9 * assign65290_body29_e101451) + (locals.var_phi_b_dpss__blk1554 * (-locals.var_t1_dn9)))))) * locals.var_fb) - (assign65290_body29_e101454 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign65290_body29_e101453) + (assign65290_body29_e101444 * ((-locals.var_t0_dn10) - ((locals.var_phi_b_dpss__blk1554_dn10 * assign65290_body29_e101451) + (locals.var_phi_b_dpss__blk1554 * (-locals.var_t1_dn10)))))) * locals.var_fb) - (assign65290_body29_e101454 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn11 * 0.5) * assign65290_body29_e101453) + (assign65290_body29_e101444 * ((-locals.var_t0_dn11) - ((locals.var_phi_b_dpss__blk1554_dn11 * assign65290_body29_e101451) + (locals.var_phi_b_dpss__blk1554 * (-locals.var_t1_dn11)))))) * locals.var_fb) - (assign65290_body29_e101454 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn14 * 0.5) * assign65290_body29_e101453) + (assign65290_body29_e101444 * ((-locals.var_t0_dn14) - ((locals.var_phi_b_dpss__blk1554_dn14 * assign65290_body29_e101451) + (locals.var_phi_b_dpss__blk1554 * (-locals.var_t1_dn14)))))) * locals.var_fb) - (assign65290_body29_e101454 * locals.var_fb_dn14)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss__blk1555, locals.var_fb_dpss__blk1555_dn0, locals.var_fb_dpss__blk1555_dn2, locals.var_fb_dpss__blk1555_dn4, locals.var_fb_dpss__blk1555_dn5, locals.var_fb_dpss__blk1555_dn6, locals.var_fb_dpss__blk1555_dn7, locals.var_fb_dpss__blk1555_dn8, locals.var_fb_dpss__blk1555_dn9, locals.var_fb_dpss__blk1555_dn10, locals.var_fb_dpss__blk1555_dn11, locals.var_fb_dpss__blk1555_dn14,)
    }
};
            locals.var_fb_dpss__blk1555 = assign65290_body29_e101458;
            locals.var_fb_dpss__blk1555_dn0 = assign65290_body29_e101458_d_n0;
            locals.var_fb_dpss__blk1555_dn2 = assign65290_body29_e101458_d_n2;
            locals.var_fb_dpss__blk1555_dn4 = assign65290_body29_e101458_d_n4;
            locals.var_fb_dpss__blk1555_dn5 = assign65290_body29_e101458_d_n5;
            locals.var_fb_dpss__blk1555_dn6 = assign65290_body29_e101458_d_n6;
            locals.var_fb_dpss__blk1555_dn7 = assign65290_body29_e101458_d_n7;
            locals.var_fb_dpss__blk1555_dn8 = assign65290_body29_e101458_d_n8;
            locals.var_fb_dpss__blk1555_dn9 = assign65290_body29_e101458_d_n9;
            locals.var_fb_dpss__blk1555_dn10 = assign65290_body29_e101458_d_n10;
            locals.var_fb_dpss__blk1555_dn11 = assign65290_body29_e101458_d_n11;
            locals.var_fb_dpss__blk1555_dn14 = assign65290_body29_e101458_d_n14;
            locals.var_fb_dpss__blk1555_rv = 0.0;
            let assign65290_body30_e101465: f64 = if ((locals.var_flg_conv == 1.0) && (locals.var_chi < 0.0)) { 1.0 } else { 0.0 };
            locals.var_guard1562 = assign65290_body30_e101465;
            locals.var_guard1562_rv = 0.0;
            let (assign65290_body31_e101483,) = {
    if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1562 != 0.0)) {
        let assign65290_body31_e101481: f64 = (-1.0);
        (assign65290_body31_e101481,)
    } else {
        (locals.var_flg_zone,)
    }
};
            locals.var_flg_zone = assign65290_body31_e101483;
            locals.var_flg_zone_rv = 0.0;
            let assign65290_body32_e101486: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1563 = assign65290_body32_e101486;
            locals.var_guard1563_rv = 0.0;
            let (assign65290_body33_e101504, assign65290_body33_e101504_d_n0, assign65290_body33_e101504_d_n2, assign65290_body33_e101504_d_n4, assign65290_body33_e101504_d_n5, assign65290_body33_e101504_d_n6, assign65290_body33_e101504_d_n7, assign65290_body33_e101504_d_n8, assign65290_body33_e101504_d_n9, assign65290_body33_e101504_d_n10, assign65290_body33_e101504_d_n11, assign65290_body33_e101504_d_n14,) = {
    if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1563 != 0.0)) {
        let assign65290_body33_e101502: f64 = (-locals.var_fb);
        (assign65290_body33_e101502, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn4), (-locals.var_fb_dn5), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn8), (-locals.var_fb_dn9), (-locals.var_fb_dn10), (-locals.var_fb_dn11), (-locals.var_fb_dn14),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign65290_body33_e101504;
            locals.var_fs02_dn0 = assign65290_body33_e101504_d_n0;
            locals.var_fs02_dn2 = assign65290_body33_e101504_d_n2;
            locals.var_fs02_dn4 = assign65290_body33_e101504_d_n4;
            locals.var_fs02_dn5 = assign65290_body33_e101504_d_n5;
            locals.var_fs02_dn6 = assign65290_body33_e101504_d_n6;
            locals.var_fs02_dn7 = assign65290_body33_e101504_d_n7;
            locals.var_fs02_dn8 = assign65290_body33_e101504_d_n8;
            locals.var_fs02_dn9 = assign65290_body33_e101504_d_n9;
            locals.var_fs02_dn10 = assign65290_body33_e101504_d_n10;
            locals.var_fs02_dn11 = assign65290_body33_e101504_d_n11;
            locals.var_fs02_dn14 = assign65290_body33_e101504_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign65290_body34_e101522, assign65290_body34_e101522_d_n0, assign65290_body34_e101522_d_n2, assign65290_body34_e101522_d_n4, assign65290_body34_e101522_d_n5, assign65290_body34_e101522_d_n6, assign65290_body34_e101522_d_n7, assign65290_body34_e101522_d_n8, assign65290_body34_e101522_d_n9, assign65290_body34_e101522_d_n10, assign65290_body34_e101522_d_n11, assign65290_body34_e101522_d_n14,) = {
    if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1563 != 0.0)) {
        let assign65290_body34_e101520: f64 = (-locals.var_fb_dpss__blk1555);
        (assign65290_body34_e101520, (-locals.var_fb_dpss__blk1555_dn0), (-locals.var_fb_dpss__blk1555_dn2), (-locals.var_fb_dpss__blk1555_dn4), (-locals.var_fb_dpss__blk1555_dn5), (-locals.var_fb_dpss__blk1555_dn6), (-locals.var_fb_dpss__blk1555_dn7), (-locals.var_fb_dpss__blk1555_dn8), (-locals.var_fb_dpss__blk1555_dn9), (-locals.var_fb_dpss__blk1555_dn10), (-locals.var_fb_dpss__blk1555_dn11), (-locals.var_fb_dpss__blk1555_dn14),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign65290_body34_e101522;
            locals.var_fs02_dps0_dn0 = assign65290_body34_e101522_d_n0;
            locals.var_fs02_dps0_dn2 = assign65290_body34_e101522_d_n2;
            locals.var_fs02_dps0_dn4 = assign65290_body34_e101522_d_n4;
            locals.var_fs02_dps0_dn5 = assign65290_body34_e101522_d_n5;
            locals.var_fs02_dps0_dn6 = assign65290_body34_e101522_d_n6;
            locals.var_fs02_dps0_dn7 = assign65290_body34_e101522_d_n7;
            locals.var_fs02_dps0_dn8 = assign65290_body34_e101522_d_n8;
            locals.var_fs02_dps0_dn9 = assign65290_body34_e101522_d_n9;
            locals.var_fs02_dps0_dn10 = assign65290_body34_e101522_d_n10;
            locals.var_fs02_dps0_dn11 = assign65290_body34_e101522_d_n11;
            locals.var_fs02_dps0_dn14 = assign65290_body34_e101522_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let assign65290_body35_e101525: f64 = if locals.var_chi < 1e-7 { 1.0 } else { 0.0 };
            locals.var_guard1564 = assign65290_body35_e101525;
            locals.var_guard1564_rv = 0.0;
            let (assign65290_body36_e101545, assign65290_body36_e101545_d_n0, assign65290_body36_e101545_d_n2, assign65290_body36_e101545_d_n4, assign65290_body36_e101545_d_n5, assign65290_body36_e101545_d_n6, assign65290_body36_e101545_d_n7, assign65290_body36_e101545_d_n8, assign65290_body36_e101545_d_n9, assign65290_body36_e101545_d_n10, assign65290_body36_e101545_d_n11, assign65290_body36_e101545_d_n14,) = {
    if (((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1563 == 0.0)) && (locals.var_guard1564 != 0.0)) {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign65290_body36_e101545;
            locals.var_fs02_dn0 = assign65290_body36_e101545_d_n0;
            locals.var_fs02_dn2 = assign65290_body36_e101545_d_n2;
            locals.var_fs02_dn4 = assign65290_body36_e101545_d_n4;
            locals.var_fs02_dn5 = assign65290_body36_e101545_d_n5;
            locals.var_fs02_dn6 = assign65290_body36_e101545_d_n6;
            locals.var_fs02_dn7 = assign65290_body36_e101545_d_n7;
            locals.var_fs02_dn8 = assign65290_body36_e101545_d_n8;
            locals.var_fs02_dn9 = assign65290_body36_e101545_d_n9;
            locals.var_fs02_dn10 = assign65290_body36_e101545_d_n10;
            locals.var_fs02_dn11 = assign65290_body36_e101545_d_n11;
            locals.var_fs02_dn14 = assign65290_body36_e101545_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign65290_body37_e101565, assign65290_body37_e101565_d_n0, assign65290_body37_e101565_d_n2, assign65290_body37_e101565_d_n4, assign65290_body37_e101565_d_n5, assign65290_body37_e101565_d_n6, assign65290_body37_e101565_d_n7, assign65290_body37_e101565_d_n8, assign65290_body37_e101565_d_n9, assign65290_body37_e101565_d_n10, assign65290_body37_e101565_d_n11, assign65290_body37_e101565_d_n14,) = {
    if (((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1563 == 0.0)) && (locals.var_guard1564 != 0.0)) {
        (locals.var_fb_dpss__blk1555, locals.var_fb_dpss__blk1555_dn0, locals.var_fb_dpss__blk1555_dn2, locals.var_fb_dpss__blk1555_dn4, locals.var_fb_dpss__blk1555_dn5, locals.var_fb_dpss__blk1555_dn6, locals.var_fb_dpss__blk1555_dn7, locals.var_fb_dpss__blk1555_dn8, locals.var_fb_dpss__blk1555_dn9, locals.var_fb_dpss__blk1555_dn10, locals.var_fb_dpss__blk1555_dn11, locals.var_fb_dpss__blk1555_dn14,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign65290_body37_e101565;
            locals.var_fs02_dps0_dn0 = assign65290_body37_e101565_d_n0;
            locals.var_fs02_dps0_dn2 = assign65290_body37_e101565_d_n2;
            locals.var_fs02_dps0_dn4 = assign65290_body37_e101565_d_n4;
            locals.var_fs02_dps0_dn5 = assign65290_body37_e101565_d_n5;
            locals.var_fs02_dps0_dn6 = assign65290_body37_e101565_d_n6;
            locals.var_fs02_dps0_dn7 = assign65290_body37_e101565_d_n7;
            locals.var_fs02_dps0_dn8 = assign65290_body37_e101565_d_n8;
            locals.var_fs02_dps0_dn9 = assign65290_body37_e101565_d_n9;
            locals.var_fs02_dps0_dn10 = assign65290_body37_e101565_d_n10;
            locals.var_fs02_dps0_dn11 = assign65290_body37_e101565_d_n11;
            locals.var_fs02_dps0_dn14 = assign65290_body37_e101565_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign65290_body38_e101590, assign65290_body38_e101590_d_n0, assign65290_body38_e101590_d_n2, assign65290_body38_e101590_d_n4, assign65290_body38_e101590_d_n5, assign65290_body38_e101590_d_n6, assign65290_body38_e101590_d_n7, assign65290_body38_e101590_d_n8, assign65290_body38_e101590_d_n9, assign65290_body38_e101590_d_n10, assign65290_body38_e101590_d_n11, assign65290_body38_e101590_d_n14,) = {
    if (((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1563 == 0.0)) && (locals.var_guard1564 == 0.0)) {
        let assign65290_body38_e101587: f64 = (locals.var_phi_s0 - p.p456);
        let assign65290_body38_e101588: f64 = (locals.var_beta * assign65290_body38_e101587);
        (assign65290_body38_e101588, ((locals.var_beta_dn0 * assign65290_body38_e101587) + (locals.var_beta * locals.var_phi_s0_dn0)), ((locals.var_beta_dn2 * assign65290_body38_e101587) + (locals.var_beta * locals.var_phi_s0_dn2)), ((locals.var_beta_dn4 * assign65290_body38_e101587) + (locals.var_beta * locals.var_phi_s0_dn4)), ((locals.var_beta_dn5 * assign65290_body38_e101587) + (locals.var_beta * locals.var_phi_s0_dn5)), ((locals.var_beta_dn6 * assign65290_body38_e101587) + (locals.var_beta * locals.var_phi_s0_dn6)), ((locals.var_beta_dn7 * assign65290_body38_e101587) + (locals.var_beta * locals.var_phi_s0_dn7)), ((locals.var_beta_dn8 * assign65290_body38_e101587) + (locals.var_beta * locals.var_phi_s0_dn8)), ((locals.var_beta_dn9 * assign65290_body38_e101587) + (locals.var_beta * locals.var_phi_s0_dn9)), ((locals.var_beta_dn10 * assign65290_body38_e101587) + (locals.var_beta * locals.var_phi_s0_dn10)), ((locals.var_beta_dn11 * assign65290_body38_e101587) + (locals.var_beta * locals.var_phi_s0_dn11)), ((locals.var_beta_dn14 * assign65290_body38_e101587) + (locals.var_beta * locals.var_phi_s0_dn14)),)
    } else {
        (locals.var_rho, locals.var_rho_dn0, locals.var_rho_dn2, locals.var_rho_dn4, locals.var_rho_dn5, locals.var_rho_dn6, locals.var_rho_dn7, locals.var_rho_dn8, locals.var_rho_dn9, locals.var_rho_dn10, locals.var_rho_dn11, locals.var_rho_dn14,)
    }
};
            locals.var_rho = assign65290_body38_e101590;
            locals.var_rho_dn0 = assign65290_body38_e101590_d_n0;
            locals.var_rho_dn2 = assign65290_body38_e101590_d_n2;
            locals.var_rho_dn4 = assign65290_body38_e101590_d_n4;
            locals.var_rho_dn5 = assign65290_body38_e101590_d_n5;
            locals.var_rho_dn6 = assign65290_body38_e101590_d_n6;
            locals.var_rho_dn7 = assign65290_body38_e101590_d_n7;
            locals.var_rho_dn8 = assign65290_body38_e101590_d_n8;
            locals.var_rho_dn9 = assign65290_body38_e101590_d_n9;
            locals.var_rho_dn10 = assign65290_body38_e101590_d_n10;
            locals.var_rho_dn11 = assign65290_body38_e101590_d_n11;
            locals.var_rho_dn14 = assign65290_body38_e101590_d_n14;
            locals.var_rho_rv = 0.0;
            let (assign65290_body39_e101612, assign65290_body39_e101612_d_n0, assign65290_body39_e101612_d_n2, assign65290_body39_e101612_d_n4, assign65290_body39_e101612_d_n5, assign65290_body39_e101612_d_n6, assign65290_body39_e101612_d_n7, assign65290_body39_e101612_d_n8, assign65290_body39_e101612_d_n9, assign65290_body39_e101612_d_n10, assign65290_body39_e101612_d_n11, assign65290_body39_e101612_d_n14,) = {
    if (((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1563 == 0.0)) && (locals.var_guard1564 == 0.0)) {
        let assign65290_body39_e101610: f64 = (locals.var_rho).exp();
        (assign65290_body39_e101610, (assign65290_body39_e101610 * locals.var_rho_dn0), (assign65290_body39_e101610 * locals.var_rho_dn2), (assign65290_body39_e101610 * locals.var_rho_dn4), (assign65290_body39_e101610 * locals.var_rho_dn5), (assign65290_body39_e101610 * locals.var_rho_dn6), (assign65290_body39_e101610 * locals.var_rho_dn7), (assign65290_body39_e101610 * locals.var_rho_dn8), (assign65290_body39_e101610 * locals.var_rho_dn9), (assign65290_body39_e101610 * locals.var_rho_dn10), (assign65290_body39_e101610 * locals.var_rho_dn11), (assign65290_body39_e101610 * locals.var_rho_dn14),)
    } else {
        (locals.var_exp_rho, locals.var_exp_rho_dn0, locals.var_exp_rho_dn2, locals.var_exp_rho_dn4, locals.var_exp_rho_dn5, locals.var_exp_rho_dn6, locals.var_exp_rho_dn7, locals.var_exp_rho_dn8, locals.var_exp_rho_dn9, locals.var_exp_rho_dn10, locals.var_exp_rho_dn11, locals.var_exp_rho_dn14,)
    }
};
            locals.var_exp_rho = assign65290_body39_e101612;
            locals.var_exp_rho_dn0 = assign65290_body39_e101612_d_n0;
            locals.var_exp_rho_dn2 = assign65290_body39_e101612_d_n2;
            locals.var_exp_rho_dn4 = assign65290_body39_e101612_d_n4;
            locals.var_exp_rho_dn5 = assign65290_body39_e101612_d_n5;
            locals.var_exp_rho_dn6 = assign65290_body39_e101612_d_n6;
            locals.var_exp_rho_dn7 = assign65290_body39_e101612_d_n7;
            locals.var_exp_rho_dn8 = assign65290_body39_e101612_d_n8;
            locals.var_exp_rho_dn9 = assign65290_body39_e101612_d_n9;
            locals.var_exp_rho_dn10 = assign65290_body39_e101612_d_n10;
            locals.var_exp_rho_dn11 = assign65290_body39_e101612_d_n11;
            locals.var_exp_rho_dn14 = assign65290_body39_e101612_d_n14;
            locals.var_exp_rho_rv = 0.0;
            let (assign65290_body40_e101641, assign65290_body40_e101641_d_n0, assign65290_body40_e101641_d_n2, assign65290_body40_e101641_d_n4, assign65290_body40_e101641_d_n5, assign65290_body40_e101641_d_n6, assign65290_body40_e101641_d_n7, assign65290_body40_e101641_d_n8, assign65290_body40_e101641_d_n9, assign65290_body40_e101641_d_n10, assign65290_body40_e101641_d_n11, assign65290_body40_e101641_d_n14,) = {
    if (((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1563 == 0.0)) && (locals.var_guard1564 == 0.0)) {
        let assign65290_body40_e101636: f64 = (locals.var_chi + 1.0);
        let assign65290_body40_e101637: f64 = (locals.var_exp_bvbsvds * assign65290_body40_e101636);
        let assign65290_body40_e101638: f64 = (locals.var_exp_rho - assign65290_body40_e101637);
        let assign65290_body40_e101639: f64 = (locals.var_cnst1 * assign65290_body40_e101638);
        (assign65290_body40_e101639, ((locals.var_cnst1_dn0 * assign65290_body40_e101638) + (locals.var_cnst1 * (locals.var_exp_rho_dn0 - ((locals.var_exp_bvbsvds_dn0 * assign65290_body40_e101636) + (locals.var_exp_bvbsvds * locals.var_chi_dn0))))), ((locals.var_cnst1_dn2 * assign65290_body40_e101638) + (locals.var_cnst1 * (locals.var_exp_rho_dn2 - ((locals.var_exp_bvbsvds_dn2 * assign65290_body40_e101636) + (locals.var_exp_bvbsvds * locals.var_chi_dn2))))), ((locals.var_cnst1_dn4 * assign65290_body40_e101638) + (locals.var_cnst1 * (locals.var_exp_rho_dn4 - ((locals.var_exp_bvbsvds_dn4 * assign65290_body40_e101636) + (locals.var_exp_bvbsvds * locals.var_chi_dn4))))), ((locals.var_cnst1_dn5 * assign65290_body40_e101638) + (locals.var_cnst1 * (locals.var_exp_rho_dn5 - ((locals.var_exp_bvbsvds_dn5 * assign65290_body40_e101636) + (locals.var_exp_bvbsvds * locals.var_chi_dn5))))), ((locals.var_cnst1_dn6 * assign65290_body40_e101638) + (locals.var_cnst1 * (locals.var_exp_rho_dn6 - ((locals.var_exp_bvbsvds_dn6 * assign65290_body40_e101636) + (locals.var_exp_bvbsvds * locals.var_chi_dn6))))), ((locals.var_cnst1_dn7 * assign65290_body40_e101638) + (locals.var_cnst1 * (locals.var_exp_rho_dn7 - ((locals.var_exp_bvbsvds_dn7 * assign65290_body40_e101636) + (locals.var_exp_bvbsvds * locals.var_chi_dn7))))), ((locals.var_cnst1_dn8 * assign65290_body40_e101638) + (locals.var_cnst1 * (locals.var_exp_rho_dn8 - ((locals.var_exp_bvbsvds_dn8 * assign65290_body40_e101636) + (locals.var_exp_bvbsvds * locals.var_chi_dn8))))), ((locals.var_cnst1_dn9 * assign65290_body40_e101638) + (locals.var_cnst1 * (locals.var_exp_rho_dn9 - ((locals.var_exp_bvbsvds_dn9 * assign65290_body40_e101636) + (locals.var_exp_bvbsvds * locals.var_chi_dn9))))), ((locals.var_cnst1_dn10 * assign65290_body40_e101638) + (locals.var_cnst1 * (locals.var_exp_rho_dn10 - ((locals.var_exp_bvbsvds_dn10 * assign65290_body40_e101636) + (locals.var_exp_bvbsvds * locals.var_chi_dn10))))), ((locals.var_cnst1_dn11 * assign65290_body40_e101638) + (locals.var_cnst1 * (locals.var_exp_rho_dn11 - ((locals.var_exp_bvbsvds_dn11 * assign65290_body40_e101636) + (locals.var_exp_bvbsvds * locals.var_chi_dn11))))), ((locals.var_cnst1_dn14 * assign65290_body40_e101638) + (locals.var_cnst1 * (locals.var_exp_rho_dn14 - ((locals.var_exp_bvbsvds_dn14 * assign65290_body40_e101636) + (locals.var_exp_bvbsvds * locals.var_chi_dn14))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign65290_body40_e101641;
            locals.var_fs01_dn0 = assign65290_body40_e101641_d_n0;
            locals.var_fs01_dn2 = assign65290_body40_e101641_d_n2;
            locals.var_fs01_dn4 = assign65290_body40_e101641_d_n4;
            locals.var_fs01_dn5 = assign65290_body40_e101641_d_n5;
            locals.var_fs01_dn6 = assign65290_body40_e101641_d_n6;
            locals.var_fs01_dn7 = assign65290_body40_e101641_d_n7;
            locals.var_fs01_dn8 = assign65290_body40_e101641_d_n8;
            locals.var_fs01_dn9 = assign65290_body40_e101641_d_n9;
            locals.var_fs01_dn10 = assign65290_body40_e101641_d_n10;
            locals.var_fs01_dn11 = assign65290_body40_e101641_d_n11;
            locals.var_fs01_dn14 = assign65290_body40_e101641_d_n14;
            locals.var_fs01_rv = 0.0;
            let (assign65290_body41_e101668, assign65290_body41_e101668_d_n0, assign65290_body41_e101668_d_n2, assign65290_body41_e101668_d_n4, assign65290_body41_e101668_d_n5, assign65290_body41_e101668_d_n6, assign65290_body41_e101668_d_n7, assign65290_body41_e101668_d_n8, assign65290_body41_e101668_d_n9, assign65290_body41_e101668_d_n10, assign65290_body41_e101668_d_n11, assign65290_body41_e101668_d_n14,) = {
    if (((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1563 == 0.0)) && (locals.var_guard1564 == 0.0)) {
        let assign65290_body41_e101662: f64 = (locals.var_cnst1 * locals.var_beta);
        let assign65290_body41_e101665: f64 = (locals.var_exp_rho - locals.var_exp_bvbsvds);
        let assign65290_body41_e101666: f64 = (assign65290_body41_e101662 * assign65290_body41_e101665);
        (assign65290_body41_e101666, ((((locals.var_cnst1_dn0 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn0)) * assign65290_body41_e101665) + (assign65290_body41_e101662 * (locals.var_exp_rho_dn0 - locals.var_exp_bvbsvds_dn0))), ((((locals.var_cnst1_dn2 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn2)) * assign65290_body41_e101665) + (assign65290_body41_e101662 * (locals.var_exp_rho_dn2 - locals.var_exp_bvbsvds_dn2))), ((((locals.var_cnst1_dn4 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn4)) * assign65290_body41_e101665) + (assign65290_body41_e101662 * (locals.var_exp_rho_dn4 - locals.var_exp_bvbsvds_dn4))), ((((locals.var_cnst1_dn5 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn5)) * assign65290_body41_e101665) + (assign65290_body41_e101662 * (locals.var_exp_rho_dn5 - locals.var_exp_bvbsvds_dn5))), ((((locals.var_cnst1_dn6 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn6)) * assign65290_body41_e101665) + (assign65290_body41_e101662 * (locals.var_exp_rho_dn6 - locals.var_exp_bvbsvds_dn6))), ((((locals.var_cnst1_dn7 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn7)) * assign65290_body41_e101665) + (assign65290_body41_e101662 * (locals.var_exp_rho_dn7 - locals.var_exp_bvbsvds_dn7))), ((((locals.var_cnst1_dn8 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn8)) * assign65290_body41_e101665) + (assign65290_body41_e101662 * (locals.var_exp_rho_dn8 - locals.var_exp_bvbsvds_dn8))), ((((locals.var_cnst1_dn9 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn9)) * assign65290_body41_e101665) + (assign65290_body41_e101662 * (locals.var_exp_rho_dn9 - locals.var_exp_bvbsvds_dn9))), ((((locals.var_cnst1_dn10 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn10)) * assign65290_body41_e101665) + (assign65290_body41_e101662 * (locals.var_exp_rho_dn10 - locals.var_exp_bvbsvds_dn10))), ((((locals.var_cnst1_dn11 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn11)) * assign65290_body41_e101665) + (assign65290_body41_e101662 * (locals.var_exp_rho_dn11 - locals.var_exp_bvbsvds_dn11))), ((((locals.var_cnst1_dn14 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn14)) * assign65290_body41_e101665) + (assign65290_body41_e101662 * (locals.var_exp_rho_dn14 - locals.var_exp_bvbsvds_dn14))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign65290_body41_e101668;
            locals.var_fs01_dps0_dn0 = assign65290_body41_e101668_d_n0;
            locals.var_fs01_dps0_dn2 = assign65290_body41_e101668_d_n2;
            locals.var_fs01_dps0_dn4 = assign65290_body41_e101668_d_n4;
            locals.var_fs01_dps0_dn5 = assign65290_body41_e101668_d_n5;
            locals.var_fs01_dps0_dn6 = assign65290_body41_e101668_d_n6;
            locals.var_fs01_dps0_dn7 = assign65290_body41_e101668_d_n7;
            locals.var_fs01_dps0_dn8 = assign65290_body41_e101668_d_n8;
            locals.var_fs01_dps0_dn9 = assign65290_body41_e101668_d_n9;
            locals.var_fs01_dps0_dn10 = assign65290_body41_e101668_d_n10;
            locals.var_fs01_dps0_dn11 = assign65290_body41_e101668_d_n11;
            locals.var_fs01_dps0_dn14 = assign65290_body41_e101668_d_n14;
            locals.var_fs01_dps0_rv = 0.0;
            let (assign65290_body42_e101694, assign65290_body42_e101694_d_n0, assign65290_body42_e101694_d_n2, assign65290_body42_e101694_d_n4, assign65290_body42_e101694_d_n5, assign65290_body42_e101694_d_n6, assign65290_body42_e101694_d_n7, assign65290_body42_e101694_d_n8, assign65290_body42_e101694_d_n9, assign65290_body42_e101694_d_n10, assign65290_body42_e101694_d_n11, assign65290_body42_e101694_d_n14,) = {
    if (((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1563 == 0.0)) && (locals.var_guard1564 == 0.0)) {
        let assign65290_body42_e101689: f64 = (locals.var_fb * locals.var_fb);
        let assign65290_body42_e101691: f64 = (assign65290_body42_e101689 + locals.var_fs01);
        let assign65290_body42_e101692: f64 = (assign65290_body42_e101691).sqrt();
        (assign65290_body42_e101692, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fs01_dn0) / (2.0 * assign65290_body42_e101692)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fs01_dn2) / (2.0 * assign65290_body42_e101692)), ((((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)) + locals.var_fs01_dn4) / (2.0 * assign65290_body42_e101692)), ((((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)) + locals.var_fs01_dn5) / (2.0 * assign65290_body42_e101692)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fs01_dn6) / (2.0 * assign65290_body42_e101692)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fs01_dn7) / (2.0 * assign65290_body42_e101692)), ((((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)) + locals.var_fs01_dn8) / (2.0 * assign65290_body42_e101692)), ((((locals.var_fb_dn9 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn9)) + locals.var_fs01_dn9) / (2.0 * assign65290_body42_e101692)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fs01_dn10) / (2.0 * assign65290_body42_e101692)), ((((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)) + locals.var_fs01_dn11) / (2.0 * assign65290_body42_e101692)), ((((locals.var_fb_dn14 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn14)) + locals.var_fs01_dn14) / (2.0 * assign65290_body42_e101692)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign65290_body42_e101694;
            locals.var_fs02_dn0 = assign65290_body42_e101694_d_n0;
            locals.var_fs02_dn2 = assign65290_body42_e101694_d_n2;
            locals.var_fs02_dn4 = assign65290_body42_e101694_d_n4;
            locals.var_fs02_dn5 = assign65290_body42_e101694_d_n5;
            locals.var_fs02_dn6 = assign65290_body42_e101694_d_n6;
            locals.var_fs02_dn7 = assign65290_body42_e101694_d_n7;
            locals.var_fs02_dn8 = assign65290_body42_e101694_d_n8;
            locals.var_fs02_dn9 = assign65290_body42_e101694_d_n9;
            locals.var_fs02_dn10 = assign65290_body42_e101694_d_n10;
            locals.var_fs02_dn11 = assign65290_body42_e101694_d_n11;
            locals.var_fs02_dn14 = assign65290_body42_e101694_d_n14;
            locals.var_fs02_rv = 0.0;
            let (assign65290_body43_e101725, assign65290_body43_e101725_d_n0, assign65290_body43_e101725_d_n2, assign65290_body43_e101725_d_n4, assign65290_body43_e101725_d_n5, assign65290_body43_e101725_d_n6, assign65290_body43_e101725_d_n7, assign65290_body43_e101725_d_n8, assign65290_body43_e101725_d_n9, assign65290_body43_e101725_d_n10, assign65290_body43_e101725_d_n11, assign65290_body43_e101725_d_n14,) = {
    if (((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1563 == 0.0)) && (locals.var_guard1564 == 0.0)) {
        let assign65290_body43_e101716: f64 = (2.0 * locals.var_fb_dpss__blk1555);
        let assign65290_body43_e101718: f64 = (assign65290_body43_e101716 * locals.var_fb);
        let assign65290_body43_e101720: f64 = (assign65290_body43_e101718 + locals.var_fs01_dps0);
        let assign65290_body43_e101721: f64 = (0.5 * assign65290_body43_e101720);
        let assign65290_body43_e101723: f64 = (assign65290_body43_e101721 / locals.var_fs02);
        (assign65290_body43_e101723, ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1555_dn0) * locals.var_fb) + (assign65290_body43_e101716 * locals.var_fb_dn0)) + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign65290_body43_e101721 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1555_dn2) * locals.var_fb) + (assign65290_body43_e101716 * locals.var_fb_dn2)) + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign65290_body43_e101721 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1555_dn4) * locals.var_fb) + (assign65290_body43_e101716 * locals.var_fb_dn4)) + locals.var_fs01_dps0_dn4)) * locals.var_fs02) - (assign65290_body43_e101721 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1555_dn5) * locals.var_fb) + (assign65290_body43_e101716 * locals.var_fb_dn5)) + locals.var_fs01_dps0_dn5)) * locals.var_fs02) - (assign65290_body43_e101721 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1555_dn6) * locals.var_fb) + (assign65290_body43_e101716 * locals.var_fb_dn6)) + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign65290_body43_e101721 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1555_dn7) * locals.var_fb) + (assign65290_body43_e101716 * locals.var_fb_dn7)) + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign65290_body43_e101721 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1555_dn8) * locals.var_fb) + (assign65290_body43_e101716 * locals.var_fb_dn8)) + locals.var_fs01_dps0_dn8)) * locals.var_fs02) - (assign65290_body43_e101721 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1555_dn9) * locals.var_fb) + (assign65290_body43_e101716 * locals.var_fb_dn9)) + locals.var_fs01_dps0_dn9)) * locals.var_fs02) - (assign65290_body43_e101721 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1555_dn10) * locals.var_fb) + (assign65290_body43_e101716 * locals.var_fb_dn10)) + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign65290_body43_e101721 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1555_dn11) * locals.var_fb) + (assign65290_body43_e101716 * locals.var_fb_dn11)) + locals.var_fs01_dps0_dn11)) * locals.var_fs02) - (assign65290_body43_e101721 * locals.var_fs02_dn11)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss__blk1555_dn14) * locals.var_fb) + (assign65290_body43_e101716 * locals.var_fb_dn14)) + locals.var_fs01_dps0_dn14)) * locals.var_fs02) - (assign65290_body43_e101721 * locals.var_fs02_dn14)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign65290_body43_e101725;
            locals.var_fs02_dps0_dn0 = assign65290_body43_e101725_d_n0;
            locals.var_fs02_dps0_dn2 = assign65290_body43_e101725_d_n2;
            locals.var_fs02_dps0_dn4 = assign65290_body43_e101725_d_n4;
            locals.var_fs02_dps0_dn5 = assign65290_body43_e101725_d_n5;
            locals.var_fs02_dps0_dn6 = assign65290_body43_e101725_d_n6;
            locals.var_fs02_dps0_dn7 = assign65290_body43_e101725_d_n7;
            locals.var_fs02_dps0_dn8 = assign65290_body43_e101725_d_n8;
            locals.var_fs02_dps0_dn9 = assign65290_body43_e101725_d_n9;
            locals.var_fs02_dps0_dn10 = assign65290_body43_e101725_d_n10;
            locals.var_fs02_dps0_dn11 = assign65290_body43_e101725_d_n11;
            locals.var_fs02_dps0_dn14 = assign65290_body43_e101725_d_n14;
            locals.var_fs02_dps0_rv = 0.0;
            let (assign65290_body44_e101747, assign65290_body44_e101747_d_n0, assign65290_body44_e101747_d_n2, assign65290_body44_e101747_d_n4, assign65290_body44_e101747_d_n5, assign65290_body44_e101747_d_n6, assign65290_body44_e101747_d_n7, assign65290_body44_e101747_d_n8, assign65290_body44_e101747_d_n9, assign65290_body44_e101747_d_n10, assign65290_body44_e101747_d_n11, assign65290_body44_e101747_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) {
        let assign65290_body44_e101739: f64 = (-locals.var_vgp__blk1527);
        let assign65290_body44_e101741: f64 = (assign65290_body44_e101739 + locals.var_phi_s0);
        let assign65290_body44_e101744: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign65290_body44_e101745: f64 = (assign65290_body44_e101741 + assign65290_body44_e101744);
        (assign65290_body44_e101745, (((-locals.var_vgp__blk1527_dn0) + locals.var_phi_s0_dn0) + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgp__blk1527_dn2) + locals.var_phi_s0_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (((-locals.var_vgp__blk1527_dn4) + locals.var_phi_s0_dn4) + ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), (((-locals.var_vgp__blk1527_dn5) + locals.var_phi_s0_dn5) + ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), (((-locals.var_vgp__blk1527_dn6) + locals.var_phi_s0_dn6) + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgp__blk1527_dn7) + locals.var_phi_s0_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgp__blk1527_dn8) + locals.var_phi_s0_dn8) + ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), (((-locals.var_vgp__blk1527_dn9) + locals.var_phi_s0_dn9) + ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), (((-locals.var_vgp__blk1527_dn10) + locals.var_phi_s0_dn10) + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (((-locals.var_vgp__blk1527_dn11) + locals.var_phi_s0_dn11) + ((locals.var_fac1_dn11 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn11))), (((-locals.var_vgp__blk1527_dn14) + locals.var_phi_s0_dn14) + ((locals.var_fac1_dn14 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn14))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn11, locals.var_fs0_dn14,)
    }
};
            locals.var_fs0 = assign65290_body44_e101747;
            locals.var_fs0_dn0 = assign65290_body44_e101747_d_n0;
            locals.var_fs0_dn2 = assign65290_body44_e101747_d_n2;
            locals.var_fs0_dn4 = assign65290_body44_e101747_d_n4;
            locals.var_fs0_dn5 = assign65290_body44_e101747_d_n5;
            locals.var_fs0_dn6 = assign65290_body44_e101747_d_n6;
            locals.var_fs0_dn7 = assign65290_body44_e101747_d_n7;
            locals.var_fs0_dn8 = assign65290_body44_e101747_d_n8;
            locals.var_fs0_dn9 = assign65290_body44_e101747_d_n9;
            locals.var_fs0_dn10 = assign65290_body44_e101747_d_n10;
            locals.var_fs0_dn11 = assign65290_body44_e101747_d_n11;
            locals.var_fs0_dn14 = assign65290_body44_e101747_d_n14;
            locals.var_fs0_rv = 0.0;
            let (assign65290_body45_e101766, assign65290_body45_e101766_d_n0, assign65290_body45_e101766_d_n2, assign65290_body45_e101766_d_n4, assign65290_body45_e101766_d_n5, assign65290_body45_e101766_d_n6, assign65290_body45_e101766_d_n7, assign65290_body45_e101766_d_n8, assign65290_body45_e101766_d_n9, assign65290_body45_e101766_d_n10, assign65290_body45_e101766_d_n11, assign65290_body45_e101766_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) {
        let assign65290_body45_e101763: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign65290_body45_e101764: f64 = (1.0 + assign65290_body45_e101763);
        (assign65290_body45_e101764, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4)), ((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8)), ((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn11 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn11)), ((locals.var_fac1_dn14 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn14)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn11, locals.var_fs0_dps0_dn14,)
    }
};
            locals.var_fs0_dps0 = assign65290_body45_e101766;
            locals.var_fs0_dps0_dn0 = assign65290_body45_e101766_d_n0;
            locals.var_fs0_dps0_dn2 = assign65290_body45_e101766_d_n2;
            locals.var_fs0_dps0_dn4 = assign65290_body45_e101766_d_n4;
            locals.var_fs0_dps0_dn5 = assign65290_body45_e101766_d_n5;
            locals.var_fs0_dps0_dn6 = assign65290_body45_e101766_d_n6;
            locals.var_fs0_dps0_dn7 = assign65290_body45_e101766_d_n7;
            locals.var_fs0_dps0_dn8 = assign65290_body45_e101766_d_n8;
            locals.var_fs0_dps0_dn9 = assign65290_body45_e101766_d_n9;
            locals.var_fs0_dps0_dn10 = assign65290_body45_e101766_d_n10;
            locals.var_fs0_dps0_dn11 = assign65290_body45_e101766_d_n11;
            locals.var_fs0_dps0_dn14 = assign65290_body45_e101766_d_n14;
            locals.var_fs0_dps0_rv = 0.0;
            let assign65290_body46_e101769: f64 = if locals.var_flg_conv == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard1565 = assign65290_body46_e101769;
            locals.var_guard1565_rv = 0.0;
            let (assign65290_body47_e101788,) = {
    if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1565 != 0.0)) {
        let assign65290_body47_e101786: f64 = (locals.var_lp_s0_max + 1.0);
        (assign65290_body47_e101786,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign65290_body47_e101788;
            locals.var_lp_s0_rv = 0.0;
            let (assign65290_body48_e101809, assign65290_body48_e101809_d_n0, assign65290_body48_e101809_d_n2, assign65290_body48_e101809_d_n4, assign65290_body48_e101809_d_n5, assign65290_body48_e101809_d_n6, assign65290_body48_e101809_d_n7, assign65290_body48_e101809_d_n8, assign65290_body48_e101809_d_n9, assign65290_body48_e101809_d_n10, assign65290_body48_e101809_d_n11, assign65290_body48_e101809_d_n14,) = {
    if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1565 == 0.0)) {
        let assign65290_body48_e101805: f64 = (-locals.var_fs0);
        let assign65290_body48_e101807: f64 = (assign65290_body48_e101805 / locals.var_fs0_dps0);
        (assign65290_body48_e101807, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign65290_body48_e101805 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign65290_body48_e101805 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign65290_body48_e101805 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign65290_body48_e101805 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign65290_body48_e101805 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign65290_body48_e101805 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign65290_body48_e101805 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign65290_body48_e101805 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign65290_body48_e101805 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn11) * locals.var_fs0_dps0) - (assign65290_body48_e101805 * locals.var_fs0_dps0_dn11)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn14) * locals.var_fs0_dps0) - (assign65290_body48_e101805 * locals.var_fs0_dps0_dn14)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
            locals.var_dps0 = assign65290_body48_e101809;
            locals.var_dps0_dn0 = assign65290_body48_e101809_d_n0;
            locals.var_dps0_dn2 = assign65290_body48_e101809_d_n2;
            locals.var_dps0_dn4 = assign65290_body48_e101809_d_n4;
            locals.var_dps0_dn5 = assign65290_body48_e101809_d_n5;
            locals.var_dps0_dn6 = assign65290_body48_e101809_d_n6;
            locals.var_dps0_dn7 = assign65290_body48_e101809_d_n7;
            locals.var_dps0_dn8 = assign65290_body48_e101809_d_n8;
            locals.var_dps0_dn9 = assign65290_body48_e101809_d_n9;
            locals.var_dps0_dn10 = assign65290_body48_e101809_d_n10;
            locals.var_dps0_dn11 = assign65290_body48_e101809_d_n11;
            locals.var_dps0_dn14 = assign65290_body48_e101809_d_n14;
            locals.var_dps0_rv = 0.0;
            let (assign65290_body49_e101840, assign65290_body49_e101840_d_n0, assign65290_body49_e101840_d_n2, assign65290_body49_e101840_d_n4, assign65290_body49_e101840_d_n5, assign65290_body49_e101840_d_n6, assign65290_body49_e101840_d_n7, assign65290_body49_e101840_d_n8, assign65290_body49_e101840_d_n9, assign65290_body49_e101840_d_n10, assign65290_body49_e101840_d_n11, assign65290_body49_e101840_d_n14,) = {
    if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1565 == 0.0)) {
        let assign65290_body49_e101827: f64 = (0.5 * 0.1);
        let assign65290_body49_e101831: f64 = (locals.var_phi_s0).abs();
        let (assign65290_body49_e101836, assign65290_body49_e101836_d_n0, assign65290_body49_e101836_d_n2, assign65290_body49_e101836_d_n4, assign65290_body49_e101836_d_n5, assign65290_body49_e101836_d_n6, assign65290_body49_e101836_d_n7, assign65290_body49_e101836_d_n8, assign65290_body49_e101836_d_n9, assign65290_body49_e101836_d_n10, assign65290_body49_e101836_d_n11, assign65290_body49_e101836_d_n14,) = {
            if (1.0 >= assign65290_body49_e101831) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign65290_body49_e101835: f64 = (locals.var_phi_s0).abs();
                (assign65290_body49_e101835, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn0 } else { (-locals.var_phi_s0_dn0) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn2 } else { (-locals.var_phi_s0_dn2) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn4 } else { (-locals.var_phi_s0_dn4) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn5 } else { (-locals.var_phi_s0_dn5) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn6 } else { (-locals.var_phi_s0_dn6) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn7 } else { (-locals.var_phi_s0_dn7) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn8 } else { (-locals.var_phi_s0_dn8) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn9 } else { (-locals.var_phi_s0_dn9) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn10 } else { (-locals.var_phi_s0_dn10) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn11 } else { (-locals.var_phi_s0_dn11) }, if locals.var_phi_s0 >= 0.0 { locals.var_phi_s0_dn14 } else { (-locals.var_phi_s0_dn14) },)
            }
        };
        let assign65290_body49_e101837: f64 = (1.0 + assign65290_body49_e101836);
        let assign65290_body49_e101838: f64 = (assign65290_body49_e101827 * assign65290_body49_e101837);
        (assign65290_body49_e101838, (assign65290_body49_e101827 * assign65290_body49_e101836_d_n0), (assign65290_body49_e101827 * assign65290_body49_e101836_d_n2), (assign65290_body49_e101827 * assign65290_body49_e101836_d_n4), (assign65290_body49_e101827 * assign65290_body49_e101836_d_n5), (assign65290_body49_e101827 * assign65290_body49_e101836_d_n6), (assign65290_body49_e101827 * assign65290_body49_e101836_d_n7), (assign65290_body49_e101827 * assign65290_body49_e101836_d_n8), (assign65290_body49_e101827 * assign65290_body49_e101836_d_n9), (assign65290_body49_e101827 * assign65290_body49_e101836_d_n10), (assign65290_body49_e101827 * assign65290_body49_e101836_d_n11), (assign65290_body49_e101827 * assign65290_body49_e101836_d_n14),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn11, locals.var_dplim_dn14,)
    }
};
            locals.var_dplim = assign65290_body49_e101840;
            locals.var_dplim_dn0 = assign65290_body49_e101840_d_n0;
            locals.var_dplim_dn2 = assign65290_body49_e101840_d_n2;
            locals.var_dplim_dn4 = assign65290_body49_e101840_d_n4;
            locals.var_dplim_dn5 = assign65290_body49_e101840_d_n5;
            locals.var_dplim_dn6 = assign65290_body49_e101840_d_n6;
            locals.var_dplim_dn7 = assign65290_body49_e101840_d_n7;
            locals.var_dplim_dn8 = assign65290_body49_e101840_d_n8;
            locals.var_dplim_dn9 = assign65290_body49_e101840_d_n9;
            locals.var_dplim_dn10 = assign65290_body49_e101840_d_n10;
            locals.var_dplim_dn11 = assign65290_body49_e101840_d_n11;
            locals.var_dplim_dn14 = assign65290_body49_e101840_d_n14;
            locals.var_dplim_rv = 0.0;
            let assign65290_body50_e101842: f64 = (locals.var_dps0).abs();
            let assign65290_body50_e101844: f64 = if assign65290_body50_e101842 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard1566 = assign65290_body50_e101844;
            locals.var_guard1566_rv = 0.0;
            let (assign65290_body51_e101872, assign65290_body51_e101872_d_n0, assign65290_body51_e101872_d_n2, assign65290_body51_e101872_d_n4, assign65290_body51_e101872_d_n5, assign65290_body51_e101872_d_n6, assign65290_body51_e101872_d_n7, assign65290_body51_e101872_d_n8, assign65290_body51_e101872_d_n9, assign65290_body51_e101872_d_n10, assign65290_body51_e101872_d_n11, assign65290_body51_e101872_d_n14,) = {
    if (((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1565 == 0.0)) && (locals.var_guard1566 != 0.0)) {
        let (assign65290_body51_e101869,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign65290_body51_e101868: f64 = (-1.0);
                (assign65290_body51_e101868,)
            }
        };
        let assign65290_body51_e101870: f64 = (locals.var_dplim * assign65290_body51_e101869);
        (assign65290_body51_e101870, (locals.var_dplim_dn0 * assign65290_body51_e101869), (locals.var_dplim_dn2 * assign65290_body51_e101869), (locals.var_dplim_dn4 * assign65290_body51_e101869), (locals.var_dplim_dn5 * assign65290_body51_e101869), (locals.var_dplim_dn6 * assign65290_body51_e101869), (locals.var_dplim_dn7 * assign65290_body51_e101869), (locals.var_dplim_dn8 * assign65290_body51_e101869), (locals.var_dplim_dn9 * assign65290_body51_e101869), (locals.var_dplim_dn10 * assign65290_body51_e101869), (locals.var_dplim_dn11 * assign65290_body51_e101869), (locals.var_dplim_dn14 * assign65290_body51_e101869),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
            locals.var_dps0 = assign65290_body51_e101872;
            locals.var_dps0_dn0 = assign65290_body51_e101872_d_n0;
            locals.var_dps0_dn2 = assign65290_body51_e101872_d_n2;
            locals.var_dps0_dn4 = assign65290_body51_e101872_d_n4;
            locals.var_dps0_dn5 = assign65290_body51_e101872_d_n5;
            locals.var_dps0_dn6 = assign65290_body51_e101872_d_n6;
            locals.var_dps0_dn7 = assign65290_body51_e101872_d_n7;
            locals.var_dps0_dn8 = assign65290_body51_e101872_d_n8;
            locals.var_dps0_dn9 = assign65290_body51_e101872_d_n9;
            locals.var_dps0_dn10 = assign65290_body51_e101872_d_n10;
            locals.var_dps0_dn11 = assign65290_body51_e101872_d_n11;
            locals.var_dps0_dn14 = assign65290_body51_e101872_d_n14;
            locals.var_dps0_rv = 0.0;
            let (assign65290_body52_e101892, assign65290_body52_e101892_d_n0, assign65290_body52_e101892_d_n2, assign65290_body52_e101892_d_n4, assign65290_body52_e101892_d_n5, assign65290_body52_e101892_d_n6, assign65290_body52_e101892_d_n7, assign65290_body52_e101892_d_n8, assign65290_body52_e101892_d_n9, assign65290_body52_e101892_d_n10, assign65290_body52_e101892_d_n11, assign65290_body52_e101892_d_n14,) = {
    if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1565 == 0.0)) {
        let assign65290_body52_e101890: f64 = (locals.var_phi_s0 + locals.var_dps0);
        (assign65290_body52_e101890, (locals.var_phi_s0_dn0 + locals.var_dps0_dn0), (locals.var_phi_s0_dn2 + locals.var_dps0_dn2), (locals.var_phi_s0_dn4 + locals.var_dps0_dn4), (locals.var_phi_s0_dn5 + locals.var_dps0_dn5), (locals.var_phi_s0_dn6 + locals.var_dps0_dn6), (locals.var_phi_s0_dn7 + locals.var_dps0_dn7), (locals.var_phi_s0_dn8 + locals.var_dps0_dn8), (locals.var_phi_s0_dn9 + locals.var_dps0_dn9), (locals.var_phi_s0_dn10 + locals.var_dps0_dn10), (locals.var_phi_s0_dn11 + locals.var_dps0_dn11), (locals.var_phi_s0_dn14 + locals.var_dps0_dn14),)
    } else {
        (locals.var_phi_s0, locals.var_phi_s0_dn0, locals.var_phi_s0_dn2, locals.var_phi_s0_dn4, locals.var_phi_s0_dn5, locals.var_phi_s0_dn6, locals.var_phi_s0_dn7, locals.var_phi_s0_dn8, locals.var_phi_s0_dn9, locals.var_phi_s0_dn10, locals.var_phi_s0_dn11, locals.var_phi_s0_dn14,)
    }
};
            locals.var_phi_s0 = assign65290_body52_e101892;
            locals.var_phi_s0_dn0 = assign65290_body52_e101892_d_n0;
            locals.var_phi_s0_dn2 = assign65290_body52_e101892_d_n2;
            locals.var_phi_s0_dn4 = assign65290_body52_e101892_d_n4;
            locals.var_phi_s0_dn5 = assign65290_body52_e101892_d_n5;
            locals.var_phi_s0_dn6 = assign65290_body52_e101892_d_n6;
            locals.var_phi_s0_dn7 = assign65290_body52_e101892_d_n7;
            locals.var_phi_s0_dn8 = assign65290_body52_e101892_d_n8;
            locals.var_phi_s0_dn9 = assign65290_body52_e101892_d_n9;
            locals.var_phi_s0_dn10 = assign65290_body52_e101892_d_n10;
            locals.var_phi_s0_dn11 = assign65290_body52_e101892_d_n11;
            locals.var_phi_s0_dn14 = assign65290_body52_e101892_d_n14;
            locals.var_phi_s0_rv = 0.0;
            let assign65290_body53_e101894: f64 = (locals.var_dps0).abs();
            let assign65290_body53_e101898: f64 = (locals.var_fs0).abs();
            let assign65290_body53_e101901: f64 = if ((assign65290_body53_e101894 <= 1e-12) && (assign65290_body53_e101898 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard1567 = assign65290_body53_e101901;
            locals.var_guard1567_rv = 0.0;
            let (assign65290_body54_e101921,) = {
    if (((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) && (locals.var_guard1565 == 0.0)) && (locals.var_guard1567 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign65290_body54_e101921;
            locals.var_flg_conv_rv = 0.0;
            let (assign65290_body55_e101938,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) {
        let assign65290_body55_e101936: f64 = (locals.var_lp_s0 + 1.0);
        (assign65290_body55_e101936,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign65290_body55_e101938;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_246(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign65300_e101953, assign65300_e101953_d_n0, assign65300_e101953_d_n2, assign65300_e101953_d_n4, assign65300_e101953_d_n5, assign65300_e101953_d_n6, assign65300_e101953_d_n7, assign65300_e101953_d_n8, assign65300_e101953_d_n9, assign65300_e101953_d_n10, assign65300_e101953_d_n11, assign65300_e101953_d_n14,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1540 == 0.0)) && (locals.var_guard1546 != 0.0)) && (locals.var_guard1556 == 0.0)) {
        (locals.var_phi_s0, locals.var_phi_s0_dn0, locals.var_phi_s0_dn2, locals.var_phi_s0_dn4, locals.var_phi_s0_dn5, locals.var_phi_s0_dn6, locals.var_phi_s0_dn7, locals.var_phi_s0_dn8, locals.var_phi_s0_dn9, locals.var_phi_s0_dn10, locals.var_phi_s0_dn11, locals.var_phi_s0_dn14,)
    } else {
        (locals.var_ps0__blk1525, locals.var_ps0__blk1525_dn0, locals.var_ps0__blk1525_dn2, locals.var_ps0__blk1525_dn4, locals.var_ps0__blk1525_dn5, locals.var_ps0__blk1525_dn6, locals.var_ps0__blk1525_dn7, locals.var_ps0__blk1525_dn8, locals.var_ps0__blk1525_dn9, locals.var_ps0__blk1525_dn10, locals.var_ps0__blk1525_dn11, locals.var_ps0__blk1525_dn14,)
    }
};
        locals.var_ps0__blk1525 = assign65300_e101953;
        locals.var_ps0__blk1525_dn0 = assign65300_e101953_d_n0;
        locals.var_ps0__blk1525_dn2 = assign65300_e101953_d_n2;
        locals.var_ps0__blk1525_dn4 = assign65300_e101953_d_n4;
        locals.var_ps0__blk1525_dn5 = assign65300_e101953_d_n5;
        locals.var_ps0__blk1525_dn6 = assign65300_e101953_d_n6;
        locals.var_ps0__blk1525_dn7 = assign65300_e101953_d_n7;
        locals.var_ps0__blk1525_dn8 = assign65300_e101953_d_n8;
        locals.var_ps0__blk1525_dn9 = assign65300_e101953_d_n9;
        locals.var_ps0__blk1525_dn10 = assign65300_e101953_d_n10;
        locals.var_ps0__blk1525_dn11 = assign65300_e101953_d_n11;
        locals.var_ps0__blk1525_dn14 = assign65300_e101953_d_n14;
        locals.var_ps0__blk1525_rv = 0.0;

        let (assign65310_e101965, assign65310_e101965_d_n0, assign65310_e101965_d_n2, assign65310_e101965_d_n4, assign65310_e101965_d_n5, assign65310_e101965_d_n6, assign65310_e101965_d_n7, assign65310_e101965_d_n8, assign65310_e101965_d_n9, assign65310_e101965_d_n10, assign65310_e101965_d_n11, assign65310_e101965_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) {
        let assign65310_e101959: f64 = (-locals.var_beta);
        let assign65310_e101962: f64 = (locals.var_ps0__blk1525 - locals.var_dphi_vds);
        let assign65310_e101963: f64 = (assign65310_e101959 * assign65310_e101962);
        (assign65310_e101963, (((-locals.var_beta_dn0) * assign65310_e101962) + (assign65310_e101959 * (locals.var_ps0__blk1525_dn0 - locals.var_dphi_vds_dn0))), (((-locals.var_beta_dn2) * assign65310_e101962) + (assign65310_e101959 * (locals.var_ps0__blk1525_dn2 - locals.var_dphi_vds_dn2))), (((-locals.var_beta_dn4) * assign65310_e101962) + (assign65310_e101959 * (locals.var_ps0__blk1525_dn4 - locals.var_dphi_vds_dn4))), (((-locals.var_beta_dn5) * assign65310_e101962) + (assign65310_e101959 * (locals.var_ps0__blk1525_dn5 - locals.var_dphi_vds_dn5))), (((-locals.var_beta_dn6) * assign65310_e101962) + (assign65310_e101959 * (locals.var_ps0__blk1525_dn6 - locals.var_dphi_vds_dn6))), (((-locals.var_beta_dn7) * assign65310_e101962) + (assign65310_e101959 * (locals.var_ps0__blk1525_dn7 - locals.var_dphi_vds_dn7))), (((-locals.var_beta_dn8) * assign65310_e101962) + (assign65310_e101959 * (locals.var_ps0__blk1525_dn8 - locals.var_dphi_vds_dn8))), (((-locals.var_beta_dn9) * assign65310_e101962) + (assign65310_e101959 * (locals.var_ps0__blk1525_dn9 - locals.var_dphi_vds_dn9))), (((-locals.var_beta_dn10) * assign65310_e101962) + (assign65310_e101959 * (locals.var_ps0__blk1525_dn10 - locals.var_dphi_vds_dn10))), (((-locals.var_beta_dn11) * assign65310_e101962) + (assign65310_e101959 * (locals.var_ps0__blk1525_dn11 - locals.var_dphi_vds_dn11))), (((-locals.var_beta_dn14) * assign65310_e101962) + (assign65310_e101959 * (locals.var_ps0__blk1525_dn14 - locals.var_dphi_vds_dn14))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign65310_e101965;
        locals.var_t5_dn0 = assign65310_e101965_d_n0;
        locals.var_t5_dn2 = assign65310_e101965_d_n2;
        locals.var_t5_dn4 = assign65310_e101965_d_n4;
        locals.var_t5_dn5 = assign65310_e101965_d_n5;
        locals.var_t5_dn6 = assign65310_e101965_d_n6;
        locals.var_t5_dn7 = assign65310_e101965_d_n7;
        locals.var_t5_dn8 = assign65310_e101965_d_n8;
        locals.var_t5_dn9 = assign65310_e101965_d_n9;
        locals.var_t5_dn10 = assign65310_e101965_d_n10;
        locals.var_t5_dn11 = assign65310_e101965_d_n11;
        locals.var_t5_dn14 = assign65310_e101965_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign65320_e101973, assign65320_e101973_d_n0, assign65320_e101973_d_n2, assign65320_e101973_d_n4, assign65320_e101973_d_n5, assign65320_e101973_d_n6, assign65320_e101973_d_n7, assign65320_e101973_d_n8, assign65320_e101973_d_n9, assign65320_e101973_d_n10, assign65320_e101973_d_n11, assign65320_e101973_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) {
        let assign65320_e101971: f64 = (locals.var_t5).abs();
        (assign65320_e101971, if locals.var_t5 >= 0.0 { locals.var_t5_dn0 } else { (-locals.var_t5_dn0) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn2 } else { (-locals.var_t5_dn2) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn4 } else { (-locals.var_t5_dn4) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn5 } else { (-locals.var_t5_dn5) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn6 } else { (-locals.var_t5_dn6) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn7 } else { (-locals.var_t5_dn7) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn8 } else { (-locals.var_t5_dn8) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn9 } else { (-locals.var_t5_dn9) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn10 } else { (-locals.var_t5_dn10) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn11 } else { (-locals.var_t5_dn11) }, if locals.var_t5 >= 0.0 { locals.var_t5_dn14 } else { (-locals.var_t5_dn14) },)
    } else {
        (locals.var_t5abs, locals.var_t5abs_dn0, locals.var_t5abs_dn2, locals.var_t5abs_dn4, locals.var_t5abs_dn5, locals.var_t5abs_dn6, locals.var_t5abs_dn7, locals.var_t5abs_dn8, locals.var_t5abs_dn9, locals.var_t5abs_dn10, locals.var_t5abs_dn11, locals.var_t5abs_dn14,)
    }
};
        locals.var_t5abs = assign65320_e101973;
        locals.var_t5abs_dn0 = assign65320_e101973_d_n0;
        locals.var_t5abs_dn2 = assign65320_e101973_d_n2;
        locals.var_t5abs_dn4 = assign65320_e101973_d_n4;
        locals.var_t5abs_dn5 = assign65320_e101973_d_n5;
        locals.var_t5abs_dn6 = assign65320_e101973_d_n6;
        locals.var_t5abs_dn7 = assign65320_e101973_d_n7;
        locals.var_t5abs_dn8 = assign65320_e101973_d_n8;
        locals.var_t5abs_dn9 = assign65320_e101973_d_n9;
        locals.var_t5abs_dn10 = assign65320_e101973_d_n10;
        locals.var_t5abs_dn11 = assign65320_e101973_d_n11;
        locals.var_t5abs_dn14 = assign65320_e101973_d_n14;
        locals.var_t5abs_rv = 0.0;

        let (assign65330_e101981, assign65330_e101981_d_n0, assign65330_e101981_d_n2, assign65330_e101981_d_n4, assign65330_e101981_d_n5, assign65330_e101981_d_n6, assign65330_e101981_d_n7, assign65330_e101981_d_n8, assign65330_e101981_d_n9, assign65330_e101981_d_n10, assign65330_e101981_d_n11, assign65330_e101981_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) {
        let assign65330_e101979: f64 = (locals.var_t5).exp();
        (assign65330_e101979, (assign65330_e101979 * locals.var_t5_dn0), (assign65330_e101979 * locals.var_t5_dn2), (assign65330_e101979 * locals.var_t5_dn4), (assign65330_e101979 * locals.var_t5_dn5), (assign65330_e101979 * locals.var_t5_dn6), (assign65330_e101979 * locals.var_t5_dn7), (assign65330_e101979 * locals.var_t5_dn8), (assign65330_e101979 * locals.var_t5_dn9), (assign65330_e101979 * locals.var_t5_dn10), (assign65330_e101979 * locals.var_t5_dn11), (assign65330_e101979 * locals.var_t5_dn14),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign65330_e101981;
        locals.var_t6_dn0 = assign65330_e101981_d_n0;
        locals.var_t6_dn2 = assign65330_e101981_d_n2;
        locals.var_t6_dn4 = assign65330_e101981_d_n4;
        locals.var_t6_dn5 = assign65330_e101981_d_n5;
        locals.var_t6_dn6 = assign65330_e101981_d_n6;
        locals.var_t6_dn7 = assign65330_e101981_d_n7;
        locals.var_t6_dn8 = assign65330_e101981_d_n8;
        locals.var_t6_dn9 = assign65330_e101981_d_n9;
        locals.var_t6_dn10 = assign65330_e101981_d_n10;
        locals.var_t6_dn11 = assign65330_e101981_d_n11;
        locals.var_t6_dn14 = assign65330_e101981_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign65340_e101992, assign65340_e101992_d_n0, assign65340_e101992_d_n2, assign65340_e101992_d_n4, assign65340_e101992_d_n5, assign65340_e101992_d_n6, assign65340_e101992_d_n7, assign65340_e101992_d_n8, assign65340_e101992_d_n9, assign65340_e101992_d_n10, assign65340_e101992_d_n11, assign65340_e101992_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) {
        let assign65340_e101988: f64 = (locals.var_t6 - 1.0);
        let assign65340_e101990: f64 = (assign65340_e101988 - locals.var_t5);
        (assign65340_e101990, (locals.var_t6_dn0 - locals.var_t5_dn0), (locals.var_t6_dn2 - locals.var_t5_dn2), (locals.var_t6_dn4 - locals.var_t5_dn4), (locals.var_t6_dn5 - locals.var_t5_dn5), (locals.var_t6_dn6 - locals.var_t5_dn6), (locals.var_t6_dn7 - locals.var_t5_dn7), (locals.var_t6_dn8 - locals.var_t5_dn8), (locals.var_t6_dn9 - locals.var_t5_dn9), (locals.var_t6_dn10 - locals.var_t5_dn10), (locals.var_t6_dn11 - locals.var_t5_dn11), (locals.var_t6_dn14 - locals.var_t5_dn14),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign65340_e101992;
        locals.var_t7_dn0 = assign65340_e101992_d_n0;
        locals.var_t7_dn2 = assign65340_e101992_d_n2;
        locals.var_t7_dn4 = assign65340_e101992_d_n4;
        locals.var_t7_dn5 = assign65340_e101992_d_n5;
        locals.var_t7_dn6 = assign65340_e101992_d_n6;
        locals.var_t7_dn7 = assign65340_e101992_d_n7;
        locals.var_t7_dn8 = assign65340_e101992_d_n8;
        locals.var_t7_dn9 = assign65340_e101992_d_n9;
        locals.var_t7_dn10 = assign65340_e101992_d_n10;
        locals.var_t7_dn11 = assign65340_e101992_d_n11;
        locals.var_t7_dn14 = assign65340_e101992_d_n14;
        locals.var_t7_rv = 0.0;

        let assign65350_e101995: f64 = if locals.var_t5 > 1e-7 { 1.0 } else { 0.0 };
        locals.var_guard1568 = assign65350_e101995;
        locals.var_guard1568_rv = 0.0;

        let (assign65360_e102008, assign65360_e102008_d_n0, assign65360_e102008_d_n2, assign65360_e102008_d_n4, assign65360_e102008_d_n5, assign65360_e102008_d_n6, assign65360_e102008_d_n7, assign65360_e102008_d_n8, assign65360_e102008_d_n9, assign65360_e102008_d_n10, assign65360_e102008_d_n11, assign65360_e102008_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1568 != 0.0)) {
        let assign65360_e102003: f64 = (-locals.var_cnst0);
        let assign65360_e102005: f64 = (locals.var_t7).sqrt();
        let assign65360_e102006: f64 = (assign65360_e102003 * assign65360_e102005);
        (assign65360_e102006, (((-locals.var_cnst0_dn0) * assign65360_e102005) + (assign65360_e102003 * (locals.var_t7_dn0 / (2.0 * assign65360_e102005)))), (((-locals.var_cnst0_dn2) * assign65360_e102005) + (assign65360_e102003 * (locals.var_t7_dn2 / (2.0 * assign65360_e102005)))), (((-locals.var_cnst0_dn4) * assign65360_e102005) + (assign65360_e102003 * (locals.var_t7_dn4 / (2.0 * assign65360_e102005)))), (((-locals.var_cnst0_dn5) * assign65360_e102005) + (assign65360_e102003 * (locals.var_t7_dn5 / (2.0 * assign65360_e102005)))), (((-locals.var_cnst0_dn6) * assign65360_e102005) + (assign65360_e102003 * (locals.var_t7_dn6 / (2.0 * assign65360_e102005)))), (((-locals.var_cnst0_dn7) * assign65360_e102005) + (assign65360_e102003 * (locals.var_t7_dn7 / (2.0 * assign65360_e102005)))), (((-locals.var_cnst0_dn8) * assign65360_e102005) + (assign65360_e102003 * (locals.var_t7_dn8 / (2.0 * assign65360_e102005)))), (((-locals.var_cnst0_dn9) * assign65360_e102005) + (assign65360_e102003 * (locals.var_t7_dn9 / (2.0 * assign65360_e102005)))), (((-locals.var_cnst0_dn10) * assign65360_e102005) + (assign65360_e102003 * (locals.var_t7_dn10 / (2.0 * assign65360_e102005)))), (((-locals.var_cnst0_dn11) * assign65360_e102005) + (assign65360_e102003 * (locals.var_t7_dn11 / (2.0 * assign65360_e102005)))), (((-locals.var_cnst0_dn14) * assign65360_e102005) + (assign65360_e102003 * (locals.var_t7_dn14 / (2.0 * assign65360_e102005)))),)
    } else {
        (locals.var_qbu__blk1539, locals.var_qbu__blk1539_dn0, locals.var_qbu__blk1539_dn2, locals.var_qbu__blk1539_dn4, locals.var_qbu__blk1539_dn5, locals.var_qbu__blk1539_dn6, locals.var_qbu__blk1539_dn7, locals.var_qbu__blk1539_dn8, locals.var_qbu__blk1539_dn9, locals.var_qbu__blk1539_dn10, locals.var_qbu__blk1539_dn11, locals.var_qbu__blk1539_dn14,)
    }
};
        locals.var_qbu__blk1539 = assign65360_e102008;
        locals.var_qbu__blk1539_dn0 = assign65360_e102008_d_n0;
        locals.var_qbu__blk1539_dn2 = assign65360_e102008_d_n2;
        locals.var_qbu__blk1539_dn4 = assign65360_e102008_d_n4;
        locals.var_qbu__blk1539_dn5 = assign65360_e102008_d_n5;
        locals.var_qbu__blk1539_dn6 = assign65360_e102008_d_n6;
        locals.var_qbu__blk1539_dn7 = assign65360_e102008_d_n7;
        locals.var_qbu__blk1539_dn8 = assign65360_e102008_d_n8;
        locals.var_qbu__blk1539_dn9 = assign65360_e102008_d_n9;
        locals.var_qbu__blk1539_dn10 = assign65360_e102008_d_n10;
        locals.var_qbu__blk1539_dn11 = assign65360_e102008_d_n11;
        locals.var_qbu__blk1539_dn14 = assign65360_e102008_d_n14;
        locals.var_qbu__blk1539_rv = 0.0;

        let assign65370_e102011: f64 = if locals.var_t5abs > 1e-7 { 1.0 } else { 0.0 };
        locals.var_guard1569 = assign65370_e102011;
        locals.var_guard1569_rv = 0.0;

        let (assign65380_e102026, assign65380_e102026_d_n0, assign65380_e102026_d_n2, assign65380_e102026_d_n4, assign65380_e102026_d_n5, assign65380_e102026_d_n6, assign65380_e102026_d_n7, assign65380_e102026_d_n8, assign65380_e102026_d_n9, assign65380_e102026_d_n10, assign65380_e102026_d_n11, assign65380_e102026_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1568 == 0.0)) && (locals.var_guard1569 != 0.0)) {
        let assign65380_e102023: f64 = (locals.var_t7).sqrt();
        let assign65380_e102024: f64 = (locals.var_cnst0 * assign65380_e102023);
        (assign65380_e102024, ((locals.var_cnst0_dn0 * assign65380_e102023) + (locals.var_cnst0 * (locals.var_t7_dn0 / (2.0 * assign65380_e102023)))), ((locals.var_cnst0_dn2 * assign65380_e102023) + (locals.var_cnst0 * (locals.var_t7_dn2 / (2.0 * assign65380_e102023)))), ((locals.var_cnst0_dn4 * assign65380_e102023) + (locals.var_cnst0 * (locals.var_t7_dn4 / (2.0 * assign65380_e102023)))), ((locals.var_cnst0_dn5 * assign65380_e102023) + (locals.var_cnst0 * (locals.var_t7_dn5 / (2.0 * assign65380_e102023)))), ((locals.var_cnst0_dn6 * assign65380_e102023) + (locals.var_cnst0 * (locals.var_t7_dn6 / (2.0 * assign65380_e102023)))), ((locals.var_cnst0_dn7 * assign65380_e102023) + (locals.var_cnst0 * (locals.var_t7_dn7 / (2.0 * assign65380_e102023)))), ((locals.var_cnst0_dn8 * assign65380_e102023) + (locals.var_cnst0 * (locals.var_t7_dn8 / (2.0 * assign65380_e102023)))), ((locals.var_cnst0_dn9 * assign65380_e102023) + (locals.var_cnst0 * (locals.var_t7_dn9 / (2.0 * assign65380_e102023)))), ((locals.var_cnst0_dn10 * assign65380_e102023) + (locals.var_cnst0 * (locals.var_t7_dn10 / (2.0 * assign65380_e102023)))), ((locals.var_cnst0_dn11 * assign65380_e102023) + (locals.var_cnst0 * (locals.var_t7_dn11 / (2.0 * assign65380_e102023)))), ((locals.var_cnst0_dn14 * assign65380_e102023) + (locals.var_cnst0 * (locals.var_t7_dn14 / (2.0 * assign65380_e102023)))),)
    } else {
        (locals.var_qbu__blk1539, locals.var_qbu__blk1539_dn0, locals.var_qbu__blk1539_dn2, locals.var_qbu__blk1539_dn4, locals.var_qbu__blk1539_dn5, locals.var_qbu__blk1539_dn6, locals.var_qbu__blk1539_dn7, locals.var_qbu__blk1539_dn8, locals.var_qbu__blk1539_dn9, locals.var_qbu__blk1539_dn10, locals.var_qbu__blk1539_dn11, locals.var_qbu__blk1539_dn14,)
    }
};
        locals.var_qbu__blk1539 = assign65380_e102026;
        locals.var_qbu__blk1539_dn0 = assign65380_e102026_d_n0;
        locals.var_qbu__blk1539_dn2 = assign65380_e102026_d_n2;
        locals.var_qbu__blk1539_dn4 = assign65380_e102026_d_n4;
        locals.var_qbu__blk1539_dn5 = assign65380_e102026_d_n5;
        locals.var_qbu__blk1539_dn6 = assign65380_e102026_d_n6;
        locals.var_qbu__blk1539_dn7 = assign65380_e102026_d_n7;
        locals.var_qbu__blk1539_dn8 = assign65380_e102026_d_n8;
        locals.var_qbu__blk1539_dn9 = assign65380_e102026_d_n9;
        locals.var_qbu__blk1539_dn10 = assign65380_e102026_d_n10;
        locals.var_qbu__blk1539_dn11 = assign65380_e102026_d_n11;
        locals.var_qbu__blk1539_dn14 = assign65380_e102026_d_n14;
        locals.var_qbu__blk1539_rv = 0.0;

        let (assign65390_e102055, assign65390_e102055_d_n0, assign65390_e102055_d_n2, assign65390_e102055_d_n4, assign65390_e102055_d_n5, assign65390_e102055_d_n6, assign65390_e102055_d_n7, assign65390_e102055_d_n8, assign65390_e102055_d_n9, assign65390_e102055_d_n10, assign65390_e102055_d_n11, assign65390_e102055_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1568 == 0.0)) && (locals.var_guard1569 == 0.0)) {
        let assign65390_e102038: f64 = (-locals.var_t5);
        let assign65390_e102040: f64 = (assign65390_e102038 * 0.7071067811865475);
        let assign65390_e102044: f64 = (locals.var_t5abs * 0.3333333333333333);
        let assign65390_e102048: f64 = (0.25 * locals.var_t5abs);
        let assign65390_e102049: f64 = (1.0 + assign65390_e102048);
        let assign65390_e102050: f64 = (assign65390_e102044 * assign65390_e102049);
        let assign65390_e102051: f64 = (1.0 + assign65390_e102050);
        let assign65390_e102052: f64 = (assign65390_e102051).sqrt();
        let assign65390_e102053: f64 = (assign65390_e102040 * assign65390_e102052);
        (assign65390_e102053, ((((-locals.var_t5_dn0) * 0.7071067811865475) * assign65390_e102052) + (assign65390_e102040 * ((((locals.var_t5abs_dn0 * 0.3333333333333333) * assign65390_e102049) + (assign65390_e102044 * (0.25 * locals.var_t5abs_dn0))) / (2.0 * assign65390_e102052)))), ((((-locals.var_t5_dn2) * 0.7071067811865475) * assign65390_e102052) + (assign65390_e102040 * ((((locals.var_t5abs_dn2 * 0.3333333333333333) * assign65390_e102049) + (assign65390_e102044 * (0.25 * locals.var_t5abs_dn2))) / (2.0 * assign65390_e102052)))), ((((-locals.var_t5_dn4) * 0.7071067811865475) * assign65390_e102052) + (assign65390_e102040 * ((((locals.var_t5abs_dn4 * 0.3333333333333333) * assign65390_e102049) + (assign65390_e102044 * (0.25 * locals.var_t5abs_dn4))) / (2.0 * assign65390_e102052)))), ((((-locals.var_t5_dn5) * 0.7071067811865475) * assign65390_e102052) + (assign65390_e102040 * ((((locals.var_t5abs_dn5 * 0.3333333333333333) * assign65390_e102049) + (assign65390_e102044 * (0.25 * locals.var_t5abs_dn5))) / (2.0 * assign65390_e102052)))), ((((-locals.var_t5_dn6) * 0.7071067811865475) * assign65390_e102052) + (assign65390_e102040 * ((((locals.var_t5abs_dn6 * 0.3333333333333333) * assign65390_e102049) + (assign65390_e102044 * (0.25 * locals.var_t5abs_dn6))) / (2.0 * assign65390_e102052)))), ((((-locals.var_t5_dn7) * 0.7071067811865475) * assign65390_e102052) + (assign65390_e102040 * ((((locals.var_t5abs_dn7 * 0.3333333333333333) * assign65390_e102049) + (assign65390_e102044 * (0.25 * locals.var_t5abs_dn7))) / (2.0 * assign65390_e102052)))), ((((-locals.var_t5_dn8) * 0.7071067811865475) * assign65390_e102052) + (assign65390_e102040 * ((((locals.var_t5abs_dn8 * 0.3333333333333333) * assign65390_e102049) + (assign65390_e102044 * (0.25 * locals.var_t5abs_dn8))) / (2.0 * assign65390_e102052)))), ((((-locals.var_t5_dn9) * 0.7071067811865475) * assign65390_e102052) + (assign65390_e102040 * ((((locals.var_t5abs_dn9 * 0.3333333333333333) * assign65390_e102049) + (assign65390_e102044 * (0.25 * locals.var_t5abs_dn9))) / (2.0 * assign65390_e102052)))), ((((-locals.var_t5_dn10) * 0.7071067811865475) * assign65390_e102052) + (assign65390_e102040 * ((((locals.var_t5abs_dn10 * 0.3333333333333333) * assign65390_e102049) + (assign65390_e102044 * (0.25 * locals.var_t5abs_dn10))) / (2.0 * assign65390_e102052)))), ((((-locals.var_t5_dn11) * 0.7071067811865475) * assign65390_e102052) + (assign65390_e102040 * ((((locals.var_t5abs_dn11 * 0.3333333333333333) * assign65390_e102049) + (assign65390_e102044 * (0.25 * locals.var_t5abs_dn11))) / (2.0 * assign65390_e102052)))), ((((-locals.var_t5_dn14) * 0.7071067811865475) * assign65390_e102052) + (assign65390_e102040 * ((((locals.var_t5abs_dn14 * 0.3333333333333333) * assign65390_e102049) + (assign65390_e102044 * (0.25 * locals.var_t5abs_dn14))) / (2.0 * assign65390_e102052)))),)
    } else {
        (locals.var_qbu__blk1539, locals.var_qbu__blk1539_dn0, locals.var_qbu__blk1539_dn2, locals.var_qbu__blk1539_dn4, locals.var_qbu__blk1539_dn5, locals.var_qbu__blk1539_dn6, locals.var_qbu__blk1539_dn7, locals.var_qbu__blk1539_dn8, locals.var_qbu__blk1539_dn9, locals.var_qbu__blk1539_dn10, locals.var_qbu__blk1539_dn11, locals.var_qbu__blk1539_dn14,)
    }
};
        locals.var_qbu__blk1539 = assign65390_e102055;
        locals.var_qbu__blk1539_dn0 = assign65390_e102055_d_n0;
        locals.var_qbu__blk1539_dn2 = assign65390_e102055_d_n2;
        locals.var_qbu__blk1539_dn4 = assign65390_e102055_d_n4;
        locals.var_qbu__blk1539_dn5 = assign65390_e102055_d_n5;
        locals.var_qbu__blk1539_dn6 = assign65390_e102055_d_n6;
        locals.var_qbu__blk1539_dn7 = assign65390_e102055_d_n7;
        locals.var_qbu__blk1539_dn8 = assign65390_e102055_d_n8;
        locals.var_qbu__blk1539_dn9 = assign65390_e102055_d_n9;
        locals.var_qbu__blk1539_dn10 = assign65390_e102055_d_n10;
        locals.var_qbu__blk1539_dn11 = assign65390_e102055_d_n11;
        locals.var_qbu__blk1539_dn14 = assign65390_e102055_d_n14;
        locals.var_qbu__blk1539_rv = 0.0;

        let (assign65400_e102071, assign65400_e102071_d_n0, assign65400_e102071_d_n2, assign65400_e102071_d_n4, assign65400_e102071_d_n5, assign65400_e102071_d_n6, assign65400_e102071_d_n7, assign65400_e102071_d_n8, assign65400_e102071_d_n9, assign65400_e102071_d_n10, assign65400_e102071_d_n11, assign65400_e102071_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) {
        let assign65400_e102062: f64 = (locals.var_qbu__blk1539 * locals.var_qbu__blk1539);
        let assign65400_e102065: f64 = (4.0 * 1e-6);
        let assign65400_e102067: f64 = (assign65400_e102065 * 1e-6);
        let assign65400_e102068: f64 = (assign65400_e102062 + assign65400_e102067);
        let assign65400_e102069: f64 = (assign65400_e102068).sqrt();
        (assign65400_e102069, (((locals.var_qbu__blk1539_dn0 * locals.var_qbu__blk1539) + (locals.var_qbu__blk1539 * locals.var_qbu__blk1539_dn0)) / (2.0 * assign65400_e102069)), (((locals.var_qbu__blk1539_dn2 * locals.var_qbu__blk1539) + (locals.var_qbu__blk1539 * locals.var_qbu__blk1539_dn2)) / (2.0 * assign65400_e102069)), (((locals.var_qbu__blk1539_dn4 * locals.var_qbu__blk1539) + (locals.var_qbu__blk1539 * locals.var_qbu__blk1539_dn4)) / (2.0 * assign65400_e102069)), (((locals.var_qbu__blk1539_dn5 * locals.var_qbu__blk1539) + (locals.var_qbu__blk1539 * locals.var_qbu__blk1539_dn5)) / (2.0 * assign65400_e102069)), (((locals.var_qbu__blk1539_dn6 * locals.var_qbu__blk1539) + (locals.var_qbu__blk1539 * locals.var_qbu__blk1539_dn6)) / (2.0 * assign65400_e102069)), (((locals.var_qbu__blk1539_dn7 * locals.var_qbu__blk1539) + (locals.var_qbu__blk1539 * locals.var_qbu__blk1539_dn7)) / (2.0 * assign65400_e102069)), (((locals.var_qbu__blk1539_dn8 * locals.var_qbu__blk1539) + (locals.var_qbu__blk1539 * locals.var_qbu__blk1539_dn8)) / (2.0 * assign65400_e102069)), (((locals.var_qbu__blk1539_dn9 * locals.var_qbu__blk1539) + (locals.var_qbu__blk1539 * locals.var_qbu__blk1539_dn9)) / (2.0 * assign65400_e102069)), (((locals.var_qbu__blk1539_dn10 * locals.var_qbu__blk1539) + (locals.var_qbu__blk1539 * locals.var_qbu__blk1539_dn10)) / (2.0 * assign65400_e102069)), (((locals.var_qbu__blk1539_dn11 * locals.var_qbu__blk1539) + (locals.var_qbu__blk1539 * locals.var_qbu__blk1539_dn11)) / (2.0 * assign65400_e102069)), (((locals.var_qbu__blk1539_dn14 * locals.var_qbu__blk1539) + (locals.var_qbu__blk1539 * locals.var_qbu__blk1539_dn14)) / (2.0 * assign65400_e102069)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign65400_e102071;
        locals.var_tmf1_dn0 = assign65400_e102071_d_n0;
        locals.var_tmf1_dn2 = assign65400_e102071_d_n2;
        locals.var_tmf1_dn4 = assign65400_e102071_d_n4;
        locals.var_tmf1_dn5 = assign65400_e102071_d_n5;
        locals.var_tmf1_dn6 = assign65400_e102071_d_n6;
        locals.var_tmf1_dn7 = assign65400_e102071_d_n7;
        locals.var_tmf1_dn8 = assign65400_e102071_d_n8;
        locals.var_tmf1_dn9 = assign65400_e102071_d_n9;
        locals.var_tmf1_dn10 = assign65400_e102071_d_n10;
        locals.var_tmf1_dn11 = assign65400_e102071_d_n11;
        locals.var_tmf1_dn14 = assign65400_e102071_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign65410_e102082, assign65410_e102082_d_n0, assign65410_e102082_d_n2, assign65410_e102082_d_n4, assign65410_e102082_d_n5, assign65410_e102082_d_n6, assign65410_e102082_d_n7, assign65410_e102082_d_n8, assign65410_e102082_d_n9, assign65410_e102082_d_n10, assign65410_e102082_d_n11, assign65410_e102082_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) {
        let assign65410_e102079: f64 = (locals.var_qbu__blk1539 + locals.var_tmf1);
        let assign65410_e102080: f64 = (0.5 * assign65410_e102079);
        (assign65410_e102080, (0.5 * (locals.var_qbu__blk1539_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_qbu__blk1539_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_qbu__blk1539_dn4 + locals.var_tmf1_dn4)), (0.5 * (locals.var_qbu__blk1539_dn5 + locals.var_tmf1_dn5)), (0.5 * (locals.var_qbu__blk1539_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_qbu__blk1539_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_qbu__blk1539_dn8 + locals.var_tmf1_dn8)), (0.5 * (locals.var_qbu__blk1539_dn9 + locals.var_tmf1_dn9)), (0.5 * (locals.var_qbu__blk1539_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_qbu__blk1539_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_qbu__blk1539_dn14 + locals.var_tmf1_dn14)),)
    } else {
        (locals.var_wqbu, locals.var_wqbu_dn0, locals.var_wqbu_dn2, locals.var_wqbu_dn4, locals.var_wqbu_dn5, locals.var_wqbu_dn6, locals.var_wqbu_dn7, locals.var_wqbu_dn8, locals.var_wqbu_dn9, locals.var_wqbu_dn10, locals.var_wqbu_dn11, locals.var_wqbu_dn14,)
    }
};
        locals.var_wqbu = assign65410_e102082;
        locals.var_wqbu_dn0 = assign65410_e102082_d_n0;
        locals.var_wqbu_dn2 = assign65410_e102082_d_n2;
        locals.var_wqbu_dn4 = assign65410_e102082_d_n4;
        locals.var_wqbu_dn5 = assign65410_e102082_d_n5;
        locals.var_wqbu_dn6 = assign65410_e102082_d_n6;
        locals.var_wqbu_dn7 = assign65410_e102082_d_n7;
        locals.var_wqbu_dn8 = assign65410_e102082_d_n8;
        locals.var_wqbu_dn9 = assign65410_e102082_d_n9;
        locals.var_wqbu_dn10 = assign65410_e102082_d_n10;
        locals.var_wqbu_dn11 = assign65410_e102082_d_n11;
        locals.var_wqbu_dn14 = assign65410_e102082_d_n14;
        locals.var_wqbu_rv = 0.0;

        let (assign65420_e102093, assign65420_e102093_d_n0, assign65420_e102093_d_n2, assign65420_e102093_d_n4, assign65420_e102093_d_n5, assign65420_e102093_d_n6, assign65420_e102093_d_n7, assign65420_e102093_d_n8, assign65420_e102093_d_n9, assign65420_e102093_d_n10, assign65420_e102093_d_n11, assign65420_e102093_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) {
        let assign65420_e102090: f64 = (1.6021918e-19 * locals.var_nsub);
        let assign65420_e102091: f64 = (locals.var_wqbu / assign65420_e102090);
        (assign65420_e102091, (((locals.var_wqbu_dn0 * assign65420_e102090) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn0))) / (assign65420_e102090 * assign65420_e102090)), (((locals.var_wqbu_dn2 * assign65420_e102090) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn2))) / (assign65420_e102090 * assign65420_e102090)), (((locals.var_wqbu_dn4 * assign65420_e102090) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn4))) / (assign65420_e102090 * assign65420_e102090)), (((locals.var_wqbu_dn5 * assign65420_e102090) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn5))) / (assign65420_e102090 * assign65420_e102090)), (((locals.var_wqbu_dn6 * assign65420_e102090) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn6))) / (assign65420_e102090 * assign65420_e102090)), (((locals.var_wqbu_dn7 * assign65420_e102090) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn7))) / (assign65420_e102090 * assign65420_e102090)), (((locals.var_wqbu_dn8 * assign65420_e102090) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn8))) / (assign65420_e102090 * assign65420_e102090)), (((locals.var_wqbu_dn9 * assign65420_e102090) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn9))) / (assign65420_e102090 * assign65420_e102090)), (((locals.var_wqbu_dn10 * assign65420_e102090) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn10))) / (assign65420_e102090 * assign65420_e102090)), (((locals.var_wqbu_dn11 * assign65420_e102090) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn11))) / (assign65420_e102090 * assign65420_e102090)), (((locals.var_wqbu_dn14 * assign65420_e102090) - (locals.var_wqbu * (1.6021918e-19 * locals.var_nsub_dn14))) / (assign65420_e102090 * assign65420_e102090)),)
    } else {
        (locals.var_wdep__blk1535, locals.var_wdep__blk1535_dn0, locals.var_wdep__blk1535_dn2, locals.var_wdep__blk1535_dn4, locals.var_wdep__blk1535_dn5, locals.var_wdep__blk1535_dn6, locals.var_wdep__blk1535_dn7, locals.var_wdep__blk1535_dn8, locals.var_wdep__blk1535_dn9, locals.var_wdep__blk1535_dn10, locals.var_wdep__blk1535_dn11, locals.var_wdep__blk1535_dn14,)
    }
};
        locals.var_wdep__blk1535 = assign65420_e102093;
        locals.var_wdep__blk1535_dn0 = assign65420_e102093_d_n0;
        locals.var_wdep__blk1535_dn2 = assign65420_e102093_d_n2;
        locals.var_wdep__blk1535_dn4 = assign65420_e102093_d_n4;
        locals.var_wdep__blk1535_dn5 = assign65420_e102093_d_n5;
        locals.var_wdep__blk1535_dn6 = assign65420_e102093_d_n6;
        locals.var_wdep__blk1535_dn7 = assign65420_e102093_d_n7;
        locals.var_wdep__blk1535_dn8 = assign65420_e102093_d_n8;
        locals.var_wdep__blk1535_dn9 = assign65420_e102093_d_n9;
        locals.var_wdep__blk1535_dn10 = assign65420_e102093_d_n10;
        locals.var_wdep__blk1535_dn11 = assign65420_e102093_d_n11;
        locals.var_wdep__blk1535_dn14 = assign65420_e102093_d_n14;
        locals.var_wdep__blk1535_rv = 0.0;

        let (assign65430_e102102, assign65430_e102102_d_n0, assign65430_e102102_d_n2, assign65430_e102102_d_n4, assign65430_e102102_d_n5, assign65430_e102102_d_n6, assign65430_e102102_d_n7, assign65430_e102102_d_n8, assign65430_e102102_d_n9, assign65430_e102102_d_n10, assign65430_e102102_d_n11, assign65430_e102102_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) {
        let assign65430_e102100: f64 = (locals.var_wdep__blk1535 - p.p452);
        (assign65430_e102100, locals.var_wdep__blk1535_dn0, locals.var_wdep__blk1535_dn2, locals.var_wdep__blk1535_dn4, locals.var_wdep__blk1535_dn5, locals.var_wdep__blk1535_dn6, locals.var_wdep__blk1535_dn7, locals.var_wdep__blk1535_dn8, locals.var_wdep__blk1535_dn9, locals.var_wdep__blk1535_dn10, locals.var_wdep__blk1535_dn11, locals.var_wdep__blk1535_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign65430_e102102;
        locals.var_t1_dn0 = assign65430_e102102_d_n0;
        locals.var_t1_dn2 = assign65430_e102102_d_n2;
        locals.var_t1_dn4 = assign65430_e102102_d_n4;
        locals.var_t1_dn5 = assign65430_e102102_d_n5;
        locals.var_t1_dn6 = assign65430_e102102_d_n6;
        locals.var_t1_dn7 = assign65430_e102102_d_n7;
        locals.var_t1_dn8 = assign65430_e102102_d_n8;
        locals.var_t1_dn9 = assign65430_e102102_d_n9;
        locals.var_t1_dn10 = assign65430_e102102_d_n10;
        locals.var_t1_dn11 = assign65430_e102102_d_n11;
        locals.var_t1_dn14 = assign65430_e102102_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign65440_e102111, assign65440_e102111_d_n0, assign65440_e102111_d_n2, assign65440_e102111_d_n4, assign65440_e102111_d_n5, assign65440_e102111_d_n6, assign65440_e102111_d_n7, assign65440_e102111_d_n8, assign65440_e102111_d_n9, assign65440_e102111_d_n10, assign65440_e102111_d_n11, assign65440_e102111_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) {
        let assign65440_e102109: f64 = (locals.var_wdep__blk1535 * 0.01);
        (assign65440_e102109, (locals.var_wdep__blk1535_dn0 * 0.01), (locals.var_wdep__blk1535_dn2 * 0.01), (locals.var_wdep__blk1535_dn4 * 0.01), (locals.var_wdep__blk1535_dn5 * 0.01), (locals.var_wdep__blk1535_dn6 * 0.01), (locals.var_wdep__blk1535_dn7 * 0.01), (locals.var_wdep__blk1535_dn8 * 0.01), (locals.var_wdep__blk1535_dn9 * 0.01), (locals.var_wdep__blk1535_dn10 * 0.01), (locals.var_wdep__blk1535_dn11 * 0.01), (locals.var_wdep__blk1535_dn14 * 0.01),)
    } else {
        (locals.var_delta_1, locals.var_delta_1_dn0, locals.var_delta_1_dn2, locals.var_delta_1_dn4, locals.var_delta_1_dn5, locals.var_delta_1_dn6, locals.var_delta_1_dn7, locals.var_delta_1_dn8, locals.var_delta_1_dn9, locals.var_delta_1_dn10, locals.var_delta_1_dn11, locals.var_delta_1_dn14,)
    }
};
        locals.var_delta_1 = assign65440_e102111;
        locals.var_delta_1_dn0 = assign65440_e102111_d_n0;
        locals.var_delta_1_dn2 = assign65440_e102111_d_n2;
        locals.var_delta_1_dn4 = assign65440_e102111_d_n4;
        locals.var_delta_1_dn5 = assign65440_e102111_d_n5;
        locals.var_delta_1_dn6 = assign65440_e102111_d_n6;
        locals.var_delta_1_dn7 = assign65440_e102111_d_n7;
        locals.var_delta_1_dn8 = assign65440_e102111_d_n8;
        locals.var_delta_1_dn9 = assign65440_e102111_d_n9;
        locals.var_delta_1_dn10 = assign65440_e102111_d_n10;
        locals.var_delta_1_dn11 = assign65440_e102111_d_n11;
        locals.var_delta_1_dn14 = assign65440_e102111_d_n14;
        locals.var_delta_1_rv = 0.0;

        let (assign65450_e102127, assign65450_e102127_d_n0, assign65450_e102127_d_n2, assign65450_e102127_d_n4, assign65450_e102127_d_n5, assign65450_e102127_d_n6, assign65450_e102127_d_n7, assign65450_e102127_d_n8, assign65450_e102127_d_n9, assign65450_e102127_d_n10, assign65450_e102127_d_n11, assign65450_e102127_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) {
        let assign65450_e102118: f64 = (locals.var_t1 * locals.var_t1);
        let assign65450_e102121: f64 = (4.0 * locals.var_delta_1);
        let assign65450_e102123: f64 = (assign65450_e102121 * locals.var_delta_1);
        let assign65450_e102124: f64 = (assign65450_e102118 + assign65450_e102123);
        let assign65450_e102125: f64 = (assign65450_e102124).sqrt();
        (assign65450_e102125, ((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) + (((4.0 * locals.var_delta_1_dn0) * locals.var_delta_1) + (assign65450_e102121 * locals.var_delta_1_dn0))) / (2.0 * assign65450_e102125)), ((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) + (((4.0 * locals.var_delta_1_dn2) * locals.var_delta_1) + (assign65450_e102121 * locals.var_delta_1_dn2))) / (2.0 * assign65450_e102125)), ((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) + (((4.0 * locals.var_delta_1_dn4) * locals.var_delta_1) + (assign65450_e102121 * locals.var_delta_1_dn4))) / (2.0 * assign65450_e102125)), ((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) + (((4.0 * locals.var_delta_1_dn5) * locals.var_delta_1) + (assign65450_e102121 * locals.var_delta_1_dn5))) / (2.0 * assign65450_e102125)), ((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + (((4.0 * locals.var_delta_1_dn6) * locals.var_delta_1) + (assign65450_e102121 * locals.var_delta_1_dn6))) / (2.0 * assign65450_e102125)), ((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) + (((4.0 * locals.var_delta_1_dn7) * locals.var_delta_1) + (assign65450_e102121 * locals.var_delta_1_dn7))) / (2.0 * assign65450_e102125)), ((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) + (((4.0 * locals.var_delta_1_dn8) * locals.var_delta_1) + (assign65450_e102121 * locals.var_delta_1_dn8))) / (2.0 * assign65450_e102125)), ((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) + (((4.0 * locals.var_delta_1_dn9) * locals.var_delta_1) + (assign65450_e102121 * locals.var_delta_1_dn9))) / (2.0 * assign65450_e102125)), ((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + (((4.0 * locals.var_delta_1_dn10) * locals.var_delta_1) + (assign65450_e102121 * locals.var_delta_1_dn10))) / (2.0 * assign65450_e102125)), ((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) + (((4.0 * locals.var_delta_1_dn11) * locals.var_delta_1) + (assign65450_e102121 * locals.var_delta_1_dn11))) / (2.0 * assign65450_e102125)), ((((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) + (((4.0 * locals.var_delta_1_dn14) * locals.var_delta_1) + (assign65450_e102121 * locals.var_delta_1_dn14))) / (2.0 * assign65450_e102125)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign65450_e102127;
        locals.var_tmf1_dn0 = assign65450_e102127_d_n0;
        locals.var_tmf1_dn2 = assign65450_e102127_d_n2;
        locals.var_tmf1_dn4 = assign65450_e102127_d_n4;
        locals.var_tmf1_dn5 = assign65450_e102127_d_n5;
        locals.var_tmf1_dn6 = assign65450_e102127_d_n6;
        locals.var_tmf1_dn7 = assign65450_e102127_d_n7;
        locals.var_tmf1_dn8 = assign65450_e102127_d_n8;
        locals.var_tmf1_dn9 = assign65450_e102127_d_n9;
        locals.var_tmf1_dn10 = assign65450_e102127_d_n10;
        locals.var_tmf1_dn11 = assign65450_e102127_d_n11;
        locals.var_tmf1_dn14 = assign65450_e102127_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign65460_e102138, assign65460_e102138_d_n0, assign65460_e102138_d_n2, assign65460_e102138_d_n4, assign65460_e102138_d_n5, assign65460_e102138_d_n6, assign65460_e102138_d_n7, assign65460_e102138_d_n8, assign65460_e102138_d_n9, assign65460_e102138_d_n10, assign65460_e102138_d_n11, assign65460_e102138_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) {
        let assign65460_e102135: f64 = (locals.var_t1 + locals.var_tmf1);
        let assign65460_e102136: f64 = (0.5 * assign65460_e102135);
        (assign65460_e102136, (0.5 * (locals.var_t1_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf1_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf1_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf1_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf1_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t1_dn14 + locals.var_tmf1_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign65460_e102138;
        locals.var_t2_dn0 = assign65460_e102138_d_n0;
        locals.var_t2_dn2 = assign65460_e102138_d_n2;
        locals.var_t2_dn4 = assign65460_e102138_d_n4;
        locals.var_t2_dn5 = assign65460_e102138_d_n5;
        locals.var_t2_dn6 = assign65460_e102138_d_n6;
        locals.var_t2_dn7 = assign65460_e102138_d_n7;
        locals.var_t2_dn8 = assign65460_e102138_d_n8;
        locals.var_t2_dn9 = assign65460_e102138_d_n9;
        locals.var_t2_dn10 = assign65460_e102138_d_n10;
        locals.var_t2_dn11 = assign65460_e102138_d_n11;
        locals.var_t2_dn14 = assign65460_e102138_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign65470_e102151, assign65470_e102151_d_n0, assign65470_e102151_d_n2, assign65470_e102151_d_n4, assign65470_e102151_d_n5, assign65470_e102151_d_n6, assign65470_e102151_d_n7, assign65470_e102151_d_n8, assign65470_e102151_d_n9, assign65470_e102151_d_n10, assign65470_e102151_d_n11, assign65470_e102151_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) {
        let assign65470_e102145: f64 = (locals.var_t2 / locals.var_wdep__blk1535);
        let assign65470_e102147: f64 = (assign65470_e102145 * locals.var_t2);
        let assign65470_e102149: f64 = (assign65470_e102147 / locals.var_wdep__blk1535);
        (assign65470_e102149, ((((((((locals.var_t2_dn0 * locals.var_wdep__blk1535) - (locals.var_t2 * locals.var_wdep__blk1535_dn0)) / (locals.var_wdep__blk1535 * locals.var_wdep__blk1535)) * locals.var_t2) + (assign65470_e102145 * locals.var_t2_dn0)) * locals.var_wdep__blk1535) - (assign65470_e102147 * locals.var_wdep__blk1535_dn0)) / (locals.var_wdep__blk1535 * locals.var_wdep__blk1535)), ((((((((locals.var_t2_dn2 * locals.var_wdep__blk1535) - (locals.var_t2 * locals.var_wdep__blk1535_dn2)) / (locals.var_wdep__blk1535 * locals.var_wdep__blk1535)) * locals.var_t2) + (assign65470_e102145 * locals.var_t2_dn2)) * locals.var_wdep__blk1535) - (assign65470_e102147 * locals.var_wdep__blk1535_dn2)) / (locals.var_wdep__blk1535 * locals.var_wdep__blk1535)), ((((((((locals.var_t2_dn4 * locals.var_wdep__blk1535) - (locals.var_t2 * locals.var_wdep__blk1535_dn4)) / (locals.var_wdep__blk1535 * locals.var_wdep__blk1535)) * locals.var_t2) + (assign65470_e102145 * locals.var_t2_dn4)) * locals.var_wdep__blk1535) - (assign65470_e102147 * locals.var_wdep__blk1535_dn4)) / (locals.var_wdep__blk1535 * locals.var_wdep__blk1535)), ((((((((locals.var_t2_dn5 * locals.var_wdep__blk1535) - (locals.var_t2 * locals.var_wdep__blk1535_dn5)) / (locals.var_wdep__blk1535 * locals.var_wdep__blk1535)) * locals.var_t2) + (assign65470_e102145 * locals.var_t2_dn5)) * locals.var_wdep__blk1535) - (assign65470_e102147 * locals.var_wdep__blk1535_dn5)) / (locals.var_wdep__blk1535 * locals.var_wdep__blk1535)), ((((((((locals.var_t2_dn6 * locals.var_wdep__blk1535) - (locals.var_t2 * locals.var_wdep__blk1535_dn6)) / (locals.var_wdep__blk1535 * locals.var_wdep__blk1535)) * locals.var_t2) + (assign65470_e102145 * locals.var_t2_dn6)) * locals.var_wdep__blk1535) - (assign65470_e102147 * locals.var_wdep__blk1535_dn6)) / (locals.var_wdep__blk1535 * locals.var_wdep__blk1535)), ((((((((locals.var_t2_dn7 * locals.var_wdep__blk1535) - (locals.var_t2 * locals.var_wdep__blk1535_dn7)) / (locals.var_wdep__blk1535 * locals.var_wdep__blk1535)) * locals.var_t2) + (assign65470_e102145 * locals.var_t2_dn7)) * locals.var_wdep__blk1535) - (assign65470_e102147 * locals.var_wdep__blk1535_dn7)) / (locals.var_wdep__blk1535 * locals.var_wdep__blk1535)), ((((((((locals.var_t2_dn8 * locals.var_wdep__blk1535) - (locals.var_t2 * locals.var_wdep__blk1535_dn8)) / (locals.var_wdep__blk1535 * locals.var_wdep__blk1535)) * locals.var_t2) + (assign65470_e102145 * locals.var_t2_dn8)) * locals.var_wdep__blk1535) - (assign65470_e102147 * locals.var_wdep__blk1535_dn8)) / (locals.var_wdep__blk1535 * locals.var_wdep__blk1535)), ((((((((locals.var_t2_dn9 * locals.var_wdep__blk1535) - (locals.var_t2 * locals.var_wdep__blk1535_dn9)) / (locals.var_wdep__blk1535 * locals.var_wdep__blk1535)) * locals.var_t2) + (assign65470_e102145 * locals.var_t2_dn9)) * locals.var_wdep__blk1535) - (assign65470_e102147 * locals.var_wdep__blk1535_dn9)) / (locals.var_wdep__blk1535 * locals.var_wdep__blk1535)), ((((((((locals.var_t2_dn10 * locals.var_wdep__blk1535) - (locals.var_t2 * locals.var_wdep__blk1535_dn10)) / (locals.var_wdep__blk1535 * locals.var_wdep__blk1535)) * locals.var_t2) + (assign65470_e102145 * locals.var_t2_dn10)) * locals.var_wdep__blk1535) - (assign65470_e102147 * locals.var_wdep__blk1535_dn10)) / (locals.var_wdep__blk1535 * locals.var_wdep__blk1535)), ((((((((locals.var_t2_dn11 * locals.var_wdep__blk1535) - (locals.var_t2 * locals.var_wdep__blk1535_dn11)) / (locals.var_wdep__blk1535 * locals.var_wdep__blk1535)) * locals.var_t2) + (assign65470_e102145 * locals.var_t2_dn11)) * locals.var_wdep__blk1535) - (assign65470_e102147 * locals.var_wdep__blk1535_dn11)) / (locals.var_wdep__blk1535 * locals.var_wdep__blk1535)), ((((((((locals.var_t2_dn14 * locals.var_wdep__blk1535) - (locals.var_t2 * locals.var_wdep__blk1535_dn14)) / (locals.var_wdep__blk1535 * locals.var_wdep__blk1535)) * locals.var_t2) + (assign65470_e102145 * locals.var_t2_dn14)) * locals.var_wdep__blk1535) - (assign65470_e102147 * locals.var_wdep__blk1535_dn14)) / (locals.var_wdep__blk1535 * locals.var_wdep__blk1535)),)
    } else {
        (locals.var_wfactor, locals.var_wfactor_dn0, locals.var_wfactor_dn2, locals.var_wfactor_dn4, locals.var_wfactor_dn5, locals.var_wfactor_dn6, locals.var_wfactor_dn7, locals.var_wfactor_dn8, locals.var_wfactor_dn9, locals.var_wfactor_dn10, locals.var_wfactor_dn11, locals.var_wfactor_dn14,)
    }
};
        locals.var_wfactor = assign65470_e102151;
        locals.var_wfactor_dn0 = assign65470_e102151_d_n0;
        locals.var_wfactor_dn2 = assign65470_e102151_d_n2;
        locals.var_wfactor_dn4 = assign65470_e102151_d_n4;
        locals.var_wfactor_dn5 = assign65470_e102151_d_n5;
        locals.var_wfactor_dn6 = assign65470_e102151_d_n6;
        locals.var_wfactor_dn7 = assign65470_e102151_d_n7;
        locals.var_wfactor_dn8 = assign65470_e102151_d_n8;
        locals.var_wfactor_dn9 = assign65470_e102151_d_n9;
        locals.var_wfactor_dn10 = assign65470_e102151_d_n10;
        locals.var_wfactor_dn11 = assign65470_e102151_d_n11;
        locals.var_wfactor_dn14 = assign65470_e102151_d_n14;
        locals.var_wfactor_rv = 0.0;

        let (assign65480_e102164, assign65480_e102164_d_n0, assign65480_e102164_d_n2, assign65480_e102164_d_n4, assign65480_e102164_d_n5, assign65480_e102164_d_n6, assign65480_e102164_d_n7, assign65480_e102164_d_n8, assign65480_e102164_d_n9, assign65480_e102164_d_n10, assign65480_e102164_d_n11, assign65480_e102164_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) {
        let assign65480_e102158: f64 = (locals.var_ps0__blk1525 - locals.var_dphi_vds);
        let assign65480_e102160: f64 = (assign65480_e102158 * locals.var_wfactor);
        let assign65480_e102162: f64 = (assign65480_e102160 + locals.var_dphi_vds);
        (assign65480_e102162, ((((locals.var_ps0__blk1525_dn0 - locals.var_dphi_vds_dn0) * locals.var_wfactor) + (assign65480_e102158 * locals.var_wfactor_dn0)) + locals.var_dphi_vds_dn0), ((((locals.var_ps0__blk1525_dn2 - locals.var_dphi_vds_dn2) * locals.var_wfactor) + (assign65480_e102158 * locals.var_wfactor_dn2)) + locals.var_dphi_vds_dn2), ((((locals.var_ps0__blk1525_dn4 - locals.var_dphi_vds_dn4) * locals.var_wfactor) + (assign65480_e102158 * locals.var_wfactor_dn4)) + locals.var_dphi_vds_dn4), ((((locals.var_ps0__blk1525_dn5 - locals.var_dphi_vds_dn5) * locals.var_wfactor) + (assign65480_e102158 * locals.var_wfactor_dn5)) + locals.var_dphi_vds_dn5), ((((locals.var_ps0__blk1525_dn6 - locals.var_dphi_vds_dn6) * locals.var_wfactor) + (assign65480_e102158 * locals.var_wfactor_dn6)) + locals.var_dphi_vds_dn6), ((((locals.var_ps0__blk1525_dn7 - locals.var_dphi_vds_dn7) * locals.var_wfactor) + (assign65480_e102158 * locals.var_wfactor_dn7)) + locals.var_dphi_vds_dn7), ((((locals.var_ps0__blk1525_dn8 - locals.var_dphi_vds_dn8) * locals.var_wfactor) + (assign65480_e102158 * locals.var_wfactor_dn8)) + locals.var_dphi_vds_dn8), ((((locals.var_ps0__blk1525_dn9 - locals.var_dphi_vds_dn9) * locals.var_wfactor) + (assign65480_e102158 * locals.var_wfactor_dn9)) + locals.var_dphi_vds_dn9), ((((locals.var_ps0__blk1525_dn10 - locals.var_dphi_vds_dn10) * locals.var_wfactor) + (assign65480_e102158 * locals.var_wfactor_dn10)) + locals.var_dphi_vds_dn10), ((((locals.var_ps0__blk1525_dn11 - locals.var_dphi_vds_dn11) * locals.var_wfactor) + (assign65480_e102158 * locals.var_wfactor_dn11)) + locals.var_dphi_vds_dn11), ((((locals.var_ps0__blk1525_dn14 - locals.var_dphi_vds_dn14) * locals.var_wfactor) + (assign65480_e102158 * locals.var_wfactor_dn14)) + locals.var_dphi_vds_dn14),)
    } else {
        (locals.var_phim, locals.var_phim_dn0, locals.var_phim_dn2, locals.var_phim_dn4, locals.var_phim_dn5, locals.var_phim_dn6, locals.var_phim_dn7, locals.var_phim_dn8, locals.var_phim_dn9, locals.var_phim_dn10, locals.var_phim_dn11, locals.var_phim_dn14,)
    }
};
        locals.var_phim = assign65480_e102164;
        locals.var_phim_dn0 = assign65480_e102164_d_n0;
        locals.var_phim_dn2 = assign65480_e102164_d_n2;
        locals.var_phim_dn4 = assign65480_e102164_d_n4;
        locals.var_phim_dn5 = assign65480_e102164_d_n5;
        locals.var_phim_dn6 = assign65480_e102164_d_n6;
        locals.var_phim_dn7 = assign65480_e102164_d_n7;
        locals.var_phim_dn8 = assign65480_e102164_d_n8;
        locals.var_phim_dn9 = assign65480_e102164_d_n9;
        locals.var_phim_dn10 = assign65480_e102164_d_n10;
        locals.var_phim_dn11 = assign65480_e102164_d_n11;
        locals.var_phim_dn14 = assign65480_e102164_d_n14;
        locals.var_phim_rv = 0.0;

        let (assign65490_e102186, assign65490_e102186_d_n0, assign65490_e102186_d_n2, assign65490_e102186_d_n4, assign65490_e102186_d_n5, assign65490_e102186_d_n6, assign65490_e102186_d_n7, assign65490_e102186_d_n8, assign65490_e102186_d_n9, assign65490_e102186_d_n10, assign65490_e102186_d_n11, assign65490_e102186_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) {
        let assign65490_e102173: f64 = (locals.var_vbipn - locals.var_vbscl__blk437);
        let assign65490_e102174: f64 = (locals.var_phim - assign65490_e102173);
        let assign65490_e102175: f64 = (locals.var_beta * assign65490_e102174);
        let assign65490_e102176: f64 = (assign65490_e102175).exp();
        let assign65490_e102179: f64 = (-locals.var_beta);
        let assign65490_e102181: f64 = (assign65490_e102179 * locals.var_vds);
        let assign65490_e102182: f64 = (assign65490_e102181).exp();
        let assign65490_e102183: f64 = (1.0 - assign65490_e102182);
        let assign65490_e102184: f64 = (assign65490_e102176 * assign65490_e102183);
        (assign65490_e102184, (((assign65490_e102176 * ((locals.var_beta_dn0 * assign65490_e102174) + (locals.var_beta * (locals.var_phim_dn0 - (locals.var_vbipn_dn0 - locals.var_vbscl__blk437_dn0))))) * assign65490_e102183) + (assign65490_e102176 * (-(assign65490_e102182 * (((-locals.var_beta_dn0) * locals.var_vds) + (assign65490_e102179 * locals.var_vds_dn0)))))), (((assign65490_e102176 * ((locals.var_beta_dn2 * assign65490_e102174) + (locals.var_beta * (locals.var_phim_dn2 - (locals.var_vbipn_dn2 - locals.var_vbscl__blk437_dn2))))) * assign65490_e102183) + (assign65490_e102176 * (-(assign65490_e102182 * (((-locals.var_beta_dn2) * locals.var_vds) + (assign65490_e102179 * locals.var_vds_dn2)))))), (((assign65490_e102176 * ((locals.var_beta_dn4 * assign65490_e102174) + (locals.var_beta * (locals.var_phim_dn4 - (locals.var_vbipn_dn4 - locals.var_vbscl__blk437_dn4))))) * assign65490_e102183) + (assign65490_e102176 * (-(assign65490_e102182 * (((-locals.var_beta_dn4) * locals.var_vds) + (assign65490_e102179 * locals.var_vds_dn4)))))), (((assign65490_e102176 * ((locals.var_beta_dn5 * assign65490_e102174) + (locals.var_beta * (locals.var_phim_dn5 - (locals.var_vbipn_dn5 - locals.var_vbscl__blk437_dn5))))) * assign65490_e102183) + (assign65490_e102176 * (-(assign65490_e102182 * (((-locals.var_beta_dn5) * locals.var_vds) + (assign65490_e102179 * locals.var_vds_dn5)))))), (((assign65490_e102176 * ((locals.var_beta_dn6 * assign65490_e102174) + (locals.var_beta * (locals.var_phim_dn6 - (locals.var_vbipn_dn6 - locals.var_vbscl__blk437_dn6))))) * assign65490_e102183) + (assign65490_e102176 * (-(assign65490_e102182 * (((-locals.var_beta_dn6) * locals.var_vds) + (assign65490_e102179 * locals.var_vds_dn6)))))), (((assign65490_e102176 * ((locals.var_beta_dn7 * assign65490_e102174) + (locals.var_beta * (locals.var_phim_dn7 - (locals.var_vbipn_dn7 - locals.var_vbscl__blk437_dn7))))) * assign65490_e102183) + (assign65490_e102176 * (-(assign65490_e102182 * (((-locals.var_beta_dn7) * locals.var_vds) + (assign65490_e102179 * locals.var_vds_dn7)))))), (((assign65490_e102176 * ((locals.var_beta_dn8 * assign65490_e102174) + (locals.var_beta * (locals.var_phim_dn8 - (locals.var_vbipn_dn8 - locals.var_vbscl__blk437_dn8))))) * assign65490_e102183) + (assign65490_e102176 * (-(assign65490_e102182 * (((-locals.var_beta_dn8) * locals.var_vds) + (assign65490_e102179 * locals.var_vds_dn8)))))), (((assign65490_e102176 * ((locals.var_beta_dn9 * assign65490_e102174) + (locals.var_beta * (locals.var_phim_dn9 - (locals.var_vbipn_dn9 - locals.var_vbscl__blk437_dn9))))) * assign65490_e102183) + (assign65490_e102176 * (-(assign65490_e102182 * (((-locals.var_beta_dn9) * locals.var_vds) + (assign65490_e102179 * locals.var_vds_dn9)))))), (((assign65490_e102176 * ((locals.var_beta_dn10 * assign65490_e102174) + (locals.var_beta * (locals.var_phim_dn10 - (locals.var_vbipn_dn10 - locals.var_vbscl__blk437_dn10))))) * assign65490_e102183) + (assign65490_e102176 * (-(assign65490_e102182 * (((-locals.var_beta_dn10) * locals.var_vds) + (assign65490_e102179 * locals.var_vds_dn10)))))), (((assign65490_e102176 * ((locals.var_beta_dn11 * assign65490_e102174) + (locals.var_beta * (locals.var_phim_dn11 - (locals.var_vbipn_dn11 - locals.var_vbscl__blk437_dn11))))) * assign65490_e102183) + (assign65490_e102176 * (-(assign65490_e102182 * (((-locals.var_beta_dn11) * locals.var_vds) + (assign65490_e102179 * locals.var_vds_dn11)))))), (((assign65490_e102176 * ((locals.var_beta_dn14 * assign65490_e102174) + (locals.var_beta * (locals.var_phim_dn14 - (locals.var_vbipn_dn14 - locals.var_vbscl__blk437_dn14))))) * assign65490_e102183) + (assign65490_e102176 * (-(assign65490_e102182 * (((-locals.var_beta_dn14) * locals.var_vds) + (assign65490_e102179 * locals.var_vds_dn14)))))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign65490_e102186;
        locals.var_ty_dn0 = assign65490_e102186_d_n0;
        locals.var_ty_dn2 = assign65490_e102186_d_n2;
        locals.var_ty_dn4 = assign65490_e102186_d_n4;
        locals.var_ty_dn5 = assign65490_e102186_d_n5;
        locals.var_ty_dn6 = assign65490_e102186_d_n6;
        locals.var_ty_dn7 = assign65490_e102186_d_n7;
        locals.var_ty_dn8 = assign65490_e102186_d_n8;
        locals.var_ty_dn9 = assign65490_e102186_d_n9;
        locals.var_ty_dn10 = assign65490_e102186_d_n10;
        locals.var_ty_dn11 = assign65490_e102186_d_n11;
        locals.var_ty_dn14 = assign65490_e102186_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign65500_e102200,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) {
        let assign65500_e102193: f64 = (2.0 * 1.6021918e-19);
        let assign65500_e102195: f64 = (assign65500_e102193 * locals.var_uc_njunc);
        let assign65500_e102197: f64 = (assign65500_e102195 * 1.034943e-10);
        let assign65500_e102198: f64 = (assign65500_e102197).sqrt();
        (assign65500_e102198,)
    } else {
        (locals.var_conpt00,)
    }
};
        locals.var_conpt00 = assign65500_e102200;
        locals.var_conpt00_rv = 0.0;

        let (assign65510_e102210, assign65510_e102210_d_n0, assign65510_e102210_d_n2, assign65510_e102210_d_n4, assign65510_e102210_d_n5, assign65510_e102210_d_n6, assign65510_e102210_d_n7, assign65510_e102210_d_n8, assign65510_e102210_d_n9, assign65510_e102210_d_n10, assign65510_e102210_d_n11, assign65510_e102210_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) {
        let assign65510_e102207: f64 = (locals.var_beta_inv).sqrt();
        let assign65510_e102208: f64 = (locals.var_conpt00 * assign65510_e102207);
        (assign65510_e102208, (locals.var_conpt00 * (locals.var_beta_inv_dn0 / (2.0 * assign65510_e102207))), (locals.var_conpt00 * (locals.var_beta_inv_dn2 / (2.0 * assign65510_e102207))), (locals.var_conpt00 * (locals.var_beta_inv_dn4 / (2.0 * assign65510_e102207))), (locals.var_conpt00 * (locals.var_beta_inv_dn5 / (2.0 * assign65510_e102207))), (locals.var_conpt00 * (locals.var_beta_inv_dn6 / (2.0 * assign65510_e102207))), (locals.var_conpt00 * (locals.var_beta_inv_dn7 / (2.0 * assign65510_e102207))), (locals.var_conpt00 * (locals.var_beta_inv_dn8 / (2.0 * assign65510_e102207))), (locals.var_conpt00 * (locals.var_beta_inv_dn9 / (2.0 * assign65510_e102207))), (locals.var_conpt00 * (locals.var_beta_inv_dn10 / (2.0 * assign65510_e102207))), (locals.var_conpt00 * (locals.var_beta_inv_dn11 / (2.0 * assign65510_e102207))), (locals.var_conpt00 * (locals.var_beta_inv_dn14 / (2.0 * assign65510_e102207))),)
    } else {
        (locals.var_conpt0, locals.var_conpt0_dn0, locals.var_conpt0_dn2, locals.var_conpt0_dn4, locals.var_conpt0_dn5, locals.var_conpt0_dn6, locals.var_conpt0_dn7, locals.var_conpt0_dn8, locals.var_conpt0_dn9, locals.var_conpt0_dn10, locals.var_conpt0_dn11, locals.var_conpt0_dn14,)
    }
};
        locals.var_conpt0 = assign65510_e102210;
        locals.var_conpt0_dn0 = assign65510_e102210_d_n0;
        locals.var_conpt0_dn2 = assign65510_e102210_d_n2;
        locals.var_conpt0_dn4 = assign65510_e102210_d_n4;
        locals.var_conpt0_dn5 = assign65510_e102210_d_n5;
        locals.var_conpt0_dn6 = assign65510_e102210_d_n6;
        locals.var_conpt0_dn7 = assign65510_e102210_d_n7;
        locals.var_conpt0_dn8 = assign65510_e102210_d_n8;
        locals.var_conpt0_dn9 = assign65510_e102210_d_n9;
        locals.var_conpt0_dn10 = assign65510_e102210_d_n10;
        locals.var_conpt0_dn11 = assign65510_e102210_d_n11;
        locals.var_conpt0_dn14 = assign65510_e102210_d_n14;
        locals.var_conpt0_rv = 0.0;

        let (assign65520_e102221, assign65520_e102221_d_n0, assign65520_e102221_d_n2, assign65520_e102221_d_n4, assign65520_e102221_d_n5, assign65520_e102221_d_n6, assign65520_e102221_d_n7, assign65520_e102221_d_n8, assign65520_e102221_d_n9, assign65520_e102221_d_n10, assign65520_e102221_d_n11, assign65520_e102221_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) {
        let assign65520_e102218: f64 = (locals.var_phim - locals.var_dphi_vds);
        let assign65520_e102219: f64 = (locals.var_beta * assign65520_e102218);
        (assign65520_e102219, ((locals.var_beta_dn0 * assign65520_e102218) + (locals.var_beta * (locals.var_phim_dn0 - locals.var_dphi_vds_dn0))), ((locals.var_beta_dn2 * assign65520_e102218) + (locals.var_beta * (locals.var_phim_dn2 - locals.var_dphi_vds_dn2))), ((locals.var_beta_dn4 * assign65520_e102218) + (locals.var_beta * (locals.var_phim_dn4 - locals.var_dphi_vds_dn4))), ((locals.var_beta_dn5 * assign65520_e102218) + (locals.var_beta * (locals.var_phim_dn5 - locals.var_dphi_vds_dn5))), ((locals.var_beta_dn6 * assign65520_e102218) + (locals.var_beta * (locals.var_phim_dn6 - locals.var_dphi_vds_dn6))), ((locals.var_beta_dn7 * assign65520_e102218) + (locals.var_beta * (locals.var_phim_dn7 - locals.var_dphi_vds_dn7))), ((locals.var_beta_dn8 * assign65520_e102218) + (locals.var_beta * (locals.var_phim_dn8 - locals.var_dphi_vds_dn8))), ((locals.var_beta_dn9 * assign65520_e102218) + (locals.var_beta * (locals.var_phim_dn9 - locals.var_dphi_vds_dn9))), ((locals.var_beta_dn10 * assign65520_e102218) + (locals.var_beta * (locals.var_phim_dn10 - locals.var_dphi_vds_dn10))), ((locals.var_beta_dn11 * assign65520_e102218) + (locals.var_beta * (locals.var_phim_dn11 - locals.var_dphi_vds_dn11))), ((locals.var_beta_dn14 * assign65520_e102218) + (locals.var_beta * (locals.var_phim_dn14 - locals.var_dphi_vds_dn14))),)
    } else {
        (locals.var_t1w, locals.var_t1w_dn0, locals.var_t1w_dn2, locals.var_t1w_dn4, locals.var_t1w_dn5, locals.var_t1w_dn6, locals.var_t1w_dn7, locals.var_t1w_dn8, locals.var_t1w_dn9, locals.var_t1w_dn10, locals.var_t1w_dn11, locals.var_t1w_dn14,)
    }
};
        locals.var_t1w = assign65520_e102221;
        locals.var_t1w_dn0 = assign65520_e102221_d_n0;
        locals.var_t1w_dn2 = assign65520_e102221_d_n2;
        locals.var_t1w_dn4 = assign65520_e102221_d_n4;
        locals.var_t1w_dn5 = assign65520_e102221_d_n5;
        locals.var_t1w_dn6 = assign65520_e102221_d_n6;
        locals.var_t1w_dn7 = assign65520_e102221_d_n7;
        locals.var_t1w_dn8 = assign65520_e102221_d_n8;
        locals.var_t1w_dn9 = assign65520_e102221_d_n9;
        locals.var_t1w_dn10 = assign65520_e102221_d_n10;
        locals.var_t1w_dn11 = assign65520_e102221_d_n11;
        locals.var_t1w_dn14 = assign65520_e102221_d_n14;
        locals.var_t1w_rv = 0.0;

        let assign65530_e102226: f64 = (0.2 * locals.var_beta);
        let assign65530_e102227: f64 = assign65530_e102226;
        let assign65530_e102231: f64 = (0.2 * locals.var_beta);
        let assign65530_e102234: f64 = if ((locals.var_t1w < assign65530_e102227) && (assign65530_e102231 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1570 = assign65530_e102234;
        locals.var_guard1570_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_247(
        locals: &mut StampLocals,
    ) {
        let (assign65540_e102249, assign65540_e102249_d_n0, assign65540_e102249_d_n2, assign65540_e102249_d_n4, assign65540_e102249_d_n5, assign65540_e102249_d_n6, assign65540_e102249_d_n7, assign65540_e102249_d_n8, assign65540_e102249_d_n9, assign65540_e102249_d_n10, assign65540_e102249_d_n11, assign65540_e102249_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1570 != 0.0)) {
        let assign65540_e102244: f64 = (0.2 * locals.var_beta);
        let assign65540_e102245: f64 = assign65540_e102244;
        let assign65540_e102247: f64 = (assign65540_e102245 - locals.var_t1w);
        (assign65540_e102247, ((0.2 * locals.var_beta_dn0) - locals.var_t1w_dn0), ((0.2 * locals.var_beta_dn2) - locals.var_t1w_dn2), ((0.2 * locals.var_beta_dn4) - locals.var_t1w_dn4), ((0.2 * locals.var_beta_dn5) - locals.var_t1w_dn5), ((0.2 * locals.var_beta_dn6) - locals.var_t1w_dn6), ((0.2 * locals.var_beta_dn7) - locals.var_t1w_dn7), ((0.2 * locals.var_beta_dn8) - locals.var_t1w_dn8), ((0.2 * locals.var_beta_dn9) - locals.var_t1w_dn9), ((0.2 * locals.var_beta_dn10) - locals.var_t1w_dn10), ((0.2 * locals.var_beta_dn11) - locals.var_t1w_dn11), ((0.2 * locals.var_beta_dn14) - locals.var_t1w_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign65540_e102249;
        locals.var_tmf1_dn0 = assign65540_e102249_d_n0;
        locals.var_tmf1_dn2 = assign65540_e102249_d_n2;
        locals.var_tmf1_dn4 = assign65540_e102249_d_n4;
        locals.var_tmf1_dn5 = assign65540_e102249_d_n5;
        locals.var_tmf1_dn6 = assign65540_e102249_d_n6;
        locals.var_tmf1_dn7 = assign65540_e102249_d_n7;
        locals.var_tmf1_dn8 = assign65540_e102249_d_n8;
        locals.var_tmf1_dn9 = assign65540_e102249_d_n9;
        locals.var_tmf1_dn10 = assign65540_e102249_d_n10;
        locals.var_tmf1_dn11 = assign65540_e102249_d_n11;
        locals.var_tmf1_dn14 = assign65540_e102249_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign65550_e102260, assign65550_e102260_d_n0, assign65550_e102260_d_n2, assign65550_e102260_d_n4, assign65550_e102260_d_n5, assign65550_e102260_d_n6, assign65550_e102260_d_n7, assign65550_e102260_d_n8, assign65550_e102260_d_n9, assign65550_e102260_d_n10, assign65550_e102260_d_n11, assign65550_e102260_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1570 != 0.0)) {
        let assign65550_e102258: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign65550_e102258, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign65550_e102260;
        locals.var_x2_dn0 = assign65550_e102260_d_n0;
        locals.var_x2_dn2 = assign65550_e102260_d_n2;
        locals.var_x2_dn4 = assign65550_e102260_d_n4;
        locals.var_x2_dn5 = assign65550_e102260_d_n5;
        locals.var_x2_dn6 = assign65550_e102260_d_n6;
        locals.var_x2_dn7 = assign65550_e102260_d_n7;
        locals.var_x2_dn8 = assign65550_e102260_d_n8;
        locals.var_x2_dn9 = assign65550_e102260_d_n9;
        locals.var_x2_dn10 = assign65550_e102260_d_n10;
        locals.var_x2_dn11 = assign65550_e102260_d_n11;
        locals.var_x2_dn14 = assign65550_e102260_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign65560_e102275, assign65560_e102275_d_n0, assign65560_e102275_d_n2, assign65560_e102275_d_n4, assign65560_e102275_d_n5, assign65560_e102275_d_n6, assign65560_e102275_d_n7, assign65560_e102275_d_n8, assign65560_e102275_d_n9, assign65560_e102275_d_n10, assign65560_e102275_d_n11, assign65560_e102275_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1570 != 0.0)) {
        let assign65560_e102269: f64 = (0.2 * locals.var_beta);
        let assign65560_e102272: f64 = (0.2 * locals.var_beta);
        let assign65560_e102273: f64 = (assign65560_e102269 * assign65560_e102272);
        (assign65560_e102273, (((0.2 * locals.var_beta_dn0) * assign65560_e102272) + (assign65560_e102269 * (0.2 * locals.var_beta_dn0))), (((0.2 * locals.var_beta_dn2) * assign65560_e102272) + (assign65560_e102269 * (0.2 * locals.var_beta_dn2))), (((0.2 * locals.var_beta_dn4) * assign65560_e102272) + (assign65560_e102269 * (0.2 * locals.var_beta_dn4))), (((0.2 * locals.var_beta_dn5) * assign65560_e102272) + (assign65560_e102269 * (0.2 * locals.var_beta_dn5))), (((0.2 * locals.var_beta_dn6) * assign65560_e102272) + (assign65560_e102269 * (0.2 * locals.var_beta_dn6))), (((0.2 * locals.var_beta_dn7) * assign65560_e102272) + (assign65560_e102269 * (0.2 * locals.var_beta_dn7))), (((0.2 * locals.var_beta_dn8) * assign65560_e102272) + (assign65560_e102269 * (0.2 * locals.var_beta_dn8))), (((0.2 * locals.var_beta_dn9) * assign65560_e102272) + (assign65560_e102269 * (0.2 * locals.var_beta_dn9))), (((0.2 * locals.var_beta_dn10) * assign65560_e102272) + (assign65560_e102269 * (0.2 * locals.var_beta_dn10))), (((0.2 * locals.var_beta_dn11) * assign65560_e102272) + (assign65560_e102269 * (0.2 * locals.var_beta_dn11))), (((0.2 * locals.var_beta_dn14) * assign65560_e102272) + (assign65560_e102269 * (0.2 * locals.var_beta_dn14))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign65560_e102275;
        locals.var_xmax2_dn0 = assign65560_e102275_d_n0;
        locals.var_xmax2_dn2 = assign65560_e102275_d_n2;
        locals.var_xmax2_dn4 = assign65560_e102275_d_n4;
        locals.var_xmax2_dn5 = assign65560_e102275_d_n5;
        locals.var_xmax2_dn6 = assign65560_e102275_d_n6;
        locals.var_xmax2_dn7 = assign65560_e102275_d_n7;
        locals.var_xmax2_dn8 = assign65560_e102275_d_n8;
        locals.var_xmax2_dn9 = assign65560_e102275_d_n9;
        locals.var_xmax2_dn10 = assign65560_e102275_d_n10;
        locals.var_xmax2_dn11 = assign65560_e102275_d_n11;
        locals.var_xmax2_dn14 = assign65560_e102275_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign65570_e102284, assign65570_e102284_d_n0, assign65570_e102284_d_n2, assign65570_e102284_d_n4, assign65570_e102284_d_n5, assign65570_e102284_d_n6, assign65570_e102284_d_n7, assign65570_e102284_d_n8, assign65570_e102284_d_n9, assign65570_e102284_d_n10, assign65570_e102284_d_n11, assign65570_e102284_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1570 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign65570_e102284;
        locals.var_xp_dn0 = assign65570_e102284_d_n0;
        locals.var_xp_dn2 = assign65570_e102284_d_n2;
        locals.var_xp_dn4 = assign65570_e102284_d_n4;
        locals.var_xp_dn5 = assign65570_e102284_d_n5;
        locals.var_xp_dn6 = assign65570_e102284_d_n6;
        locals.var_xp_dn7 = assign65570_e102284_d_n7;
        locals.var_xp_dn8 = assign65570_e102284_d_n8;
        locals.var_xp_dn9 = assign65570_e102284_d_n9;
        locals.var_xp_dn10 = assign65570_e102284_d_n10;
        locals.var_xp_dn11 = assign65570_e102284_d_n11;
        locals.var_xp_dn14 = assign65570_e102284_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign65580_e102293, assign65580_e102293_d_n0, assign65580_e102293_d_n2, assign65580_e102293_d_n4, assign65580_e102293_d_n5, assign65580_e102293_d_n6, assign65580_e102293_d_n7, assign65580_e102293_d_n8, assign65580_e102293_d_n9, assign65580_e102293_d_n10, assign65580_e102293_d_n11, assign65580_e102293_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1570 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign65580_e102293;
        locals.var_xmp_dn0 = assign65580_e102293_d_n0;
        locals.var_xmp_dn2 = assign65580_e102293_d_n2;
        locals.var_xmp_dn4 = assign65580_e102293_d_n4;
        locals.var_xmp_dn5 = assign65580_e102293_d_n5;
        locals.var_xmp_dn6 = assign65580_e102293_d_n6;
        locals.var_xmp_dn7 = assign65580_e102293_d_n7;
        locals.var_xmp_dn8 = assign65580_e102293_d_n8;
        locals.var_xmp_dn9 = assign65580_e102293_d_n9;
        locals.var_xmp_dn10 = assign65580_e102293_d_n10;
        locals.var_xmp_dn11 = assign65580_e102293_d_n11;
        locals.var_xmp_dn14 = assign65580_e102293_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign65590_e102302,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1570 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign65590_e102302;
        locals.var_m0_rv = 0.0;

        let (assign65600_e102311,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1570 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign65600_e102311;
        locals.var_mm_rv = 0.0;

        let (assign65610_e102320, assign65610_e102320_d_n0, assign65610_e102320_d_n2, assign65610_e102320_d_n4, assign65610_e102320_d_n5, assign65610_e102320_d_n6, assign65610_e102320_d_n7, assign65610_e102320_d_n8, assign65610_e102320_d_n9, assign65610_e102320_d_n10, assign65610_e102320_d_n11, assign65610_e102320_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1570 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign65610_e102320;
        locals.var_arg_dn0 = assign65610_e102320_d_n0;
        locals.var_arg_dn2 = assign65610_e102320_d_n2;
        locals.var_arg_dn4 = assign65610_e102320_d_n4;
        locals.var_arg_dn5 = assign65610_e102320_d_n5;
        locals.var_arg_dn6 = assign65610_e102320_d_n6;
        locals.var_arg_dn7 = assign65610_e102320_d_n7;
        locals.var_arg_dn8 = assign65610_e102320_d_n8;
        locals.var_arg_dn9 = assign65610_e102320_d_n9;
        locals.var_arg_dn10 = assign65610_e102320_d_n10;
        locals.var_arg_dn11 = assign65610_e102320_d_n11;
        locals.var_arg_dn14 = assign65610_e102320_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign65620_e102329, assign65620_e102329_d_n0, assign65620_e102329_d_n2, assign65620_e102329_d_n4, assign65620_e102329_d_n5, assign65620_e102329_d_n6, assign65620_e102329_d_n7, assign65620_e102329_d_n8, assign65620_e102329_d_n9, assign65620_e102329_d_n10, assign65620_e102329_d_n11, assign65620_e102329_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1570 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign65620_e102329;
        locals.var_dnm_dn0 = assign65620_e102329_d_n0;
        locals.var_dnm_dn2 = assign65620_e102329_d_n2;
        locals.var_dnm_dn4 = assign65620_e102329_d_n4;
        locals.var_dnm_dn5 = assign65620_e102329_d_n5;
        locals.var_dnm_dn6 = assign65620_e102329_d_n6;
        locals.var_dnm_dn7 = assign65620_e102329_d_n7;
        locals.var_dnm_dn8 = assign65620_e102329_d_n8;
        locals.var_dnm_dn9 = assign65620_e102329_d_n9;
        locals.var_dnm_dn10 = assign65620_e102329_d_n10;
        locals.var_dnm_dn11 = assign65620_e102329_d_n11;
        locals.var_dnm_dn14 = assign65620_e102329_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign65630_e102340, assign65630_e102340_d_n0, assign65630_e102340_d_n2, assign65630_e102340_d_n4, assign65630_e102340_d_n5, assign65630_e102340_d_n6, assign65630_e102340_d_n7, assign65630_e102340_d_n8, assign65630_e102340_d_n9, assign65630_e102340_d_n10, assign65630_e102340_d_n11, assign65630_e102340_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1570 != 0.0)) {
        let assign65630_e102338: f64 = (locals.var_xp * locals.var_x2);
        (assign65630_e102338, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign65630_e102340;
        locals.var_xp_dn0 = assign65630_e102340_d_n0;
        locals.var_xp_dn2 = assign65630_e102340_d_n2;
        locals.var_xp_dn4 = assign65630_e102340_d_n4;
        locals.var_xp_dn5 = assign65630_e102340_d_n5;
        locals.var_xp_dn6 = assign65630_e102340_d_n6;
        locals.var_xp_dn7 = assign65630_e102340_d_n7;
        locals.var_xp_dn8 = assign65630_e102340_d_n8;
        locals.var_xp_dn9 = assign65630_e102340_d_n9;
        locals.var_xp_dn10 = assign65630_e102340_d_n10;
        locals.var_xp_dn11 = assign65630_e102340_d_n11;
        locals.var_xp_dn14 = assign65630_e102340_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign65640_e102351, assign65640_e102351_d_n0, assign65640_e102351_d_n2, assign65640_e102351_d_n4, assign65640_e102351_d_n5, assign65640_e102351_d_n6, assign65640_e102351_d_n7, assign65640_e102351_d_n8, assign65640_e102351_d_n9, assign65640_e102351_d_n10, assign65640_e102351_d_n11, assign65640_e102351_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1570 != 0.0)) {
        let assign65640_e102349: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign65640_e102349, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign65640_e102351;
        locals.var_xmp_dn0 = assign65640_e102351_d_n0;
        locals.var_xmp_dn2 = assign65640_e102351_d_n2;
        locals.var_xmp_dn4 = assign65640_e102351_d_n4;
        locals.var_xmp_dn5 = assign65640_e102351_d_n5;
        locals.var_xmp_dn6 = assign65640_e102351_d_n6;
        locals.var_xmp_dn7 = assign65640_e102351_d_n7;
        locals.var_xmp_dn8 = assign65640_e102351_d_n8;
        locals.var_xmp_dn9 = assign65640_e102351_d_n9;
        locals.var_xmp_dn10 = assign65640_e102351_d_n10;
        locals.var_xmp_dn11 = assign65640_e102351_d_n11;
        locals.var_xmp_dn14 = assign65640_e102351_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign65650_e102362, assign65650_e102362_d_n0, assign65650_e102362_d_n2, assign65650_e102362_d_n4, assign65650_e102362_d_n5, assign65650_e102362_d_n6, assign65650_e102362_d_n7, assign65650_e102362_d_n8, assign65650_e102362_d_n9, assign65650_e102362_d_n10, assign65650_e102362_d_n11, assign65650_e102362_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1570 != 0.0)) {
        let assign65650_e102360: f64 = (locals.var_xp + locals.var_xmp);
        (assign65650_e102360, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign65650_e102362;
        locals.var_arg_dn0 = assign65650_e102362_d_n0;
        locals.var_arg_dn2 = assign65650_e102362_d_n2;
        locals.var_arg_dn4 = assign65650_e102362_d_n4;
        locals.var_arg_dn5 = assign65650_e102362_d_n5;
        locals.var_arg_dn6 = assign65650_e102362_d_n6;
        locals.var_arg_dn7 = assign65650_e102362_d_n7;
        locals.var_arg_dn8 = assign65650_e102362_d_n8;
        locals.var_arg_dn9 = assign65650_e102362_d_n9;
        locals.var_arg_dn10 = assign65650_e102362_d_n10;
        locals.var_arg_dn11 = assign65650_e102362_d_n11;
        locals.var_arg_dn14 = assign65650_e102362_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign65660_e102371, assign65660_e102371_d_n0, assign65660_e102371_d_n2, assign65660_e102371_d_n4, assign65660_e102371_d_n5, assign65660_e102371_d_n6, assign65660_e102371_d_n7, assign65660_e102371_d_n8, assign65660_e102371_d_n9, assign65660_e102371_d_n10, assign65660_e102371_d_n11, assign65660_e102371_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1570 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign65660_e102371;
        locals.var_dnm_dn0 = assign65660_e102371_d_n0;
        locals.var_dnm_dn2 = assign65660_e102371_d_n2;
        locals.var_dnm_dn4 = assign65660_e102371_d_n4;
        locals.var_dnm_dn5 = assign65660_e102371_d_n5;
        locals.var_dnm_dn6 = assign65660_e102371_d_n6;
        locals.var_dnm_dn7 = assign65660_e102371_d_n7;
        locals.var_dnm_dn8 = assign65660_e102371_d_n8;
        locals.var_dnm_dn9 = assign65660_e102371_d_n9;
        locals.var_dnm_dn10 = assign65660_e102371_d_n10;
        locals.var_dnm_dn11 = assign65660_e102371_d_n11;
        locals.var_dnm_dn14 = assign65660_e102371_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign65670_e102386: f64 = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1571 = assign65670_e102386;
        locals.var_guard1571_rv = 0.0;

        let assign65680_e102389: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1572 = assign65680_e102389;
        locals.var_guard1572_rv = 0.0;

        let (assign65690_e102402,) = {
    if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1570 != 0.0)) && (locals.var_guard1571 != 0.0)) && (locals.var_guard1572 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign65690_e102402;
        locals.var_mm_rv = 0.0;

        let assign65700_e102405: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1573 = assign65700_e102405;
        locals.var_guard1573_rv = 0.0;

        let (assign65710_e102421,) = {
    if ((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1570 != 0.0)) && (locals.var_guard1571 != 0.0)) && (locals.var_guard1572 == 0.0)) && (locals.var_guard1573 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign65710_e102421;
        locals.var_mm_rv = 0.0;

        let assign65720_e102424: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1574 = assign65720_e102424;
        locals.var_guard1574_rv = 0.0;

        let (assign65730_e102443,) = {
    if (((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1570 != 0.0)) && (locals.var_guard1571 != 0.0)) && (locals.var_guard1572 == 0.0)) && (locals.var_guard1573 == 0.0)) && (locals.var_guard1574 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign65730_e102443;
        locals.var_mm_rv = 0.0;

        let assign65740_e102446: f64 = if 1.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1575 = assign65740_e102446;
        locals.var_guard1575_rv = 0.0;

        let (assign65750_e102468,) = {
    if ((((((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1570 != 0.0)) && (locals.var_guard1571 != 0.0)) && (locals.var_guard1572 == 0.0)) && (locals.var_guard1573 == 0.0)) && (locals.var_guard1574 == 0.0)) && (locals.var_guard1575 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign65750_e102468;
        locals.var_mm_rv = 0.0;

        let (assign65760_e102479,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1570 != 0.0)) && (locals.var_guard1571 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign65760_e102479;
        locals.var_m0_rv = 0.0;

        let mut assign65770_loop_guard: usize = 0;
        while {
            let assign65770_cond_e102491: f64 = if (((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1570 != 0.0)) && (locals.var_guard1571 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign65770_cond_e102491 != 0.0
        } {
            assign65770_loop_guard += 1;
            assert!(assign65770_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign65770_body0_e102503, assign65770_body0_e102503_d_n0, assign65770_body0_e102503_d_n2, assign65770_body0_e102503_d_n4, assign65770_body0_e102503_d_n5, assign65770_body0_e102503_d_n6, assign65770_body0_e102503_d_n7, assign65770_body0_e102503_d_n8, assign65770_body0_e102503_d_n9, assign65770_body0_e102503_d_n10, assign65770_body0_e102503_d_n11, assign65770_body0_e102503_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1570 != 0.0)) && (locals.var_guard1571 != 0.0)) {
        let assign65770_body0_e102501: f64 = (locals.var_dnm).sqrt();
        (assign65770_body0_e102501, (locals.var_dnm_dn0 / (2.0 * assign65770_body0_e102501)), (locals.var_dnm_dn2 / (2.0 * assign65770_body0_e102501)), (locals.var_dnm_dn4 / (2.0 * assign65770_body0_e102501)), (locals.var_dnm_dn5 / (2.0 * assign65770_body0_e102501)), (locals.var_dnm_dn6 / (2.0 * assign65770_body0_e102501)), (locals.var_dnm_dn7 / (2.0 * assign65770_body0_e102501)), (locals.var_dnm_dn8 / (2.0 * assign65770_body0_e102501)), (locals.var_dnm_dn9 / (2.0 * assign65770_body0_e102501)), (locals.var_dnm_dn10 / (2.0 * assign65770_body0_e102501)), (locals.var_dnm_dn11 / (2.0 * assign65770_body0_e102501)), (locals.var_dnm_dn14 / (2.0 * assign65770_body0_e102501)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign65770_body0_e102503;
            locals.var_dnm_dn0 = assign65770_body0_e102503_d_n0;
            locals.var_dnm_dn2 = assign65770_body0_e102503_d_n2;
            locals.var_dnm_dn4 = assign65770_body0_e102503_d_n4;
            locals.var_dnm_dn5 = assign65770_body0_e102503_d_n5;
            locals.var_dnm_dn6 = assign65770_body0_e102503_d_n6;
            locals.var_dnm_dn7 = assign65770_body0_e102503_d_n7;
            locals.var_dnm_dn8 = assign65770_body0_e102503_d_n8;
            locals.var_dnm_dn9 = assign65770_body0_e102503_d_n9;
            locals.var_dnm_dn10 = assign65770_body0_e102503_d_n10;
            locals.var_dnm_dn11 = assign65770_body0_e102503_d_n11;
            locals.var_dnm_dn14 = assign65770_body0_e102503_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign65770_body1_e102516,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1570 != 0.0)) && (locals.var_guard1571 != 0.0)) {
        let assign65770_body1_e102514: f64 = (locals.var_m0 + 1.0);
        (assign65770_body1_e102514,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign65770_body1_e102516;
            locals.var_m0_rv = 0.0;
        }

        let (assign65780_e102539, assign65780_e102539_d_n0, assign65780_e102539_d_n2, assign65780_e102539_d_n4, assign65780_e102539_d_n5, assign65780_e102539_d_n6, assign65780_e102539_d_n7, assign65780_e102539_d_n8, assign65780_e102539_d_n9, assign65780_e102539_d_n10, assign65780_e102539_d_n11, assign65780_e102539_d_n14,) = {
    if ((((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1570 != 0.0)) && (locals.var_guard1571 == 0.0)) {
        let (assign65780_e102537, assign65780_e102537_d_n0, assign65780_e102537_d_n2, assign65780_e102537_d_n4, assign65780_e102537_d_n5, assign65780_e102537_d_n6, assign65780_e102537_d_n7, assign65780_e102537_d_n8, assign65780_e102537_d_n9, assign65780_e102537_d_n10, assign65780_e102537_d_n11, assign65780_e102537_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign65780_e102534: f64 = 2.0;
                let assign65780_e102535: f64 = (1.0 / assign65780_e102534);
                let assign65780_e102536: f64 = (locals.var_dnm).powf(assign65780_e102535);
                (assign65780_e102536, if 0.0 == 0.0 && ((assign65780_e102535) as f64).is_finite() && ((assign65780_e102535) as f64).fract() == 0.0 { if assign65780_e102535 == 0.0 { 0.0 } else { (assign65780_e102535 * ((locals.var_dnm).powf(assign65780_e102535 - 1.0) * locals.var_dnm_dn0)) } } else { (assign65780_e102536 * (assign65780_e102535 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65780_e102535) as f64).is_finite() && ((assign65780_e102535) as f64).fract() == 0.0 { if assign65780_e102535 == 0.0 { 0.0 } else { (assign65780_e102535 * ((locals.var_dnm).powf(assign65780_e102535 - 1.0) * locals.var_dnm_dn2)) } } else { (assign65780_e102536 * (assign65780_e102535 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65780_e102535) as f64).is_finite() && ((assign65780_e102535) as f64).fract() == 0.0 { if assign65780_e102535 == 0.0 { 0.0 } else { (assign65780_e102535 * ((locals.var_dnm).powf(assign65780_e102535 - 1.0) * locals.var_dnm_dn4)) } } else { (assign65780_e102536 * (assign65780_e102535 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65780_e102535) as f64).is_finite() && ((assign65780_e102535) as f64).fract() == 0.0 { if assign65780_e102535 == 0.0 { 0.0 } else { (assign65780_e102535 * ((locals.var_dnm).powf(assign65780_e102535 - 1.0) * locals.var_dnm_dn5)) } } else { (assign65780_e102536 * (assign65780_e102535 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65780_e102535) as f64).is_finite() && ((assign65780_e102535) as f64).fract() == 0.0 { if assign65780_e102535 == 0.0 { 0.0 } else { (assign65780_e102535 * ((locals.var_dnm).powf(assign65780_e102535 - 1.0) * locals.var_dnm_dn6)) } } else { (assign65780_e102536 * (assign65780_e102535 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65780_e102535) as f64).is_finite() && ((assign65780_e102535) as f64).fract() == 0.0 { if assign65780_e102535 == 0.0 { 0.0 } else { (assign65780_e102535 * ((locals.var_dnm).powf(assign65780_e102535 - 1.0) * locals.var_dnm_dn7)) } } else { (assign65780_e102536 * (assign65780_e102535 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65780_e102535) as f64).is_finite() && ((assign65780_e102535) as f64).fract() == 0.0 { if assign65780_e102535 == 0.0 { 0.0 } else { (assign65780_e102535 * ((locals.var_dnm).powf(assign65780_e102535 - 1.0) * locals.var_dnm_dn8)) } } else { (assign65780_e102536 * (assign65780_e102535 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65780_e102535) as f64).is_finite() && ((assign65780_e102535) as f64).fract() == 0.0 { if assign65780_e102535 == 0.0 { 0.0 } else { (assign65780_e102535 * ((locals.var_dnm).powf(assign65780_e102535 - 1.0) * locals.var_dnm_dn9)) } } else { (assign65780_e102536 * (assign65780_e102535 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65780_e102535) as f64).is_finite() && ((assign65780_e102535) as f64).fract() == 0.0 { if assign65780_e102535 == 0.0 { 0.0 } else { (assign65780_e102535 * ((locals.var_dnm).powf(assign65780_e102535 - 1.0) * locals.var_dnm_dn10)) } } else { (assign65780_e102536 * (assign65780_e102535 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65780_e102535) as f64).is_finite() && ((assign65780_e102535) as f64).fract() == 0.0 { if assign65780_e102535 == 0.0 { 0.0 } else { (assign65780_e102535 * ((locals.var_dnm).powf(assign65780_e102535 - 1.0) * locals.var_dnm_dn11)) } } else { (assign65780_e102536 * (assign65780_e102535 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign65780_e102535) as f64).is_finite() && ((assign65780_e102535) as f64).fract() == 0.0 { if assign65780_e102535 == 0.0 { 0.0 } else { (assign65780_e102535 * ((locals.var_dnm).powf(assign65780_e102535 - 1.0) * locals.var_dnm_dn14)) } } else { (assign65780_e102536 * (assign65780_e102535 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign65780_e102537, assign65780_e102537_d_n0, assign65780_e102537_d_n2, assign65780_e102537_d_n4, assign65780_e102537_d_n5, assign65780_e102537_d_n6, assign65780_e102537_d_n7, assign65780_e102537_d_n8, assign65780_e102537_d_n9, assign65780_e102537_d_n10, assign65780_e102537_d_n11, assign65780_e102537_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign65780_e102539;
        locals.var_dnm_dn0 = assign65780_e102539_d_n0;
        locals.var_dnm_dn2 = assign65780_e102539_d_n2;
        locals.var_dnm_dn4 = assign65780_e102539_d_n4;
        locals.var_dnm_dn5 = assign65780_e102539_d_n5;
        locals.var_dnm_dn6 = assign65780_e102539_d_n6;
        locals.var_dnm_dn7 = assign65780_e102539_d_n7;
        locals.var_dnm_dn8 = assign65780_e102539_d_n8;
        locals.var_dnm_dn9 = assign65780_e102539_d_n9;
        locals.var_dnm_dn10 = assign65780_e102539_d_n10;
        locals.var_dnm_dn11 = assign65780_e102539_d_n11;
        locals.var_dnm_dn14 = assign65780_e102539_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign65790_e102550, assign65790_e102550_d_n0, assign65790_e102550_d_n2, assign65790_e102550_d_n4, assign65790_e102550_d_n5, assign65790_e102550_d_n6, assign65790_e102550_d_n7, assign65790_e102550_d_n8, assign65790_e102550_d_n9, assign65790_e102550_d_n10, assign65790_e102550_d_n11, assign65790_e102550_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1570 != 0.0)) {
        let assign65790_e102548: f64 = (1.0 / locals.var_dnm);
        (assign65790_e102548, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign65790_e102550;
        locals.var_dnm_dn0 = assign65790_e102550_d_n0;
        locals.var_dnm_dn2 = assign65790_e102550_d_n2;
        locals.var_dnm_dn4 = assign65790_e102550_d_n4;
        locals.var_dnm_dn5 = assign65790_e102550_d_n5;
        locals.var_dnm_dn6 = assign65790_e102550_d_n6;
        locals.var_dnm_dn7 = assign65790_e102550_d_n7;
        locals.var_dnm_dn8 = assign65790_e102550_d_n8;
        locals.var_dnm_dn9 = assign65790_e102550_d_n9;
        locals.var_dnm_dn10 = assign65790_e102550_d_n10;
        locals.var_dnm_dn11 = assign65790_e102550_d_n11;
        locals.var_dnm_dn14 = assign65790_e102550_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign65800_e102565, assign65800_e102565_d_n0, assign65800_e102565_d_n2, assign65800_e102565_d_n4, assign65800_e102565_d_n5, assign65800_e102565_d_n6, assign65800_e102565_d_n7, assign65800_e102565_d_n8, assign65800_e102565_d_n9, assign65800_e102565_d_n10, assign65800_e102565_d_n11, assign65800_e102565_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1570 != 0.0)) {
        let assign65800_e102560: f64 = (0.2 * locals.var_beta);
        let assign65800_e102561: f64 = (locals.var_tmf1 * assign65800_e102560);
        let assign65800_e102563: f64 = (assign65800_e102561 * locals.var_dnm);
        (assign65800_e102563, ((((locals.var_tmf1_dn0 * assign65800_e102560) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn0))) * locals.var_dnm) + (assign65800_e102561 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign65800_e102560) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn2))) * locals.var_dnm) + (assign65800_e102561 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign65800_e102560) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn4))) * locals.var_dnm) + (assign65800_e102561 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign65800_e102560) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn5))) * locals.var_dnm) + (assign65800_e102561 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign65800_e102560) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn6))) * locals.var_dnm) + (assign65800_e102561 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign65800_e102560) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn7))) * locals.var_dnm) + (assign65800_e102561 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign65800_e102560) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn8))) * locals.var_dnm) + (assign65800_e102561 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign65800_e102560) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn9))) * locals.var_dnm) + (assign65800_e102561 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign65800_e102560) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn10))) * locals.var_dnm) + (assign65800_e102561 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * assign65800_e102560) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn11))) * locals.var_dnm) + (assign65800_e102561 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * assign65800_e102560) + (locals.var_tmf1 * (0.2 * locals.var_beta_dn14))) * locals.var_dnm) + (assign65800_e102561 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign65800_e102565;
        locals.var_tmf0_dn0 = assign65800_e102565_d_n0;
        locals.var_tmf0_dn2 = assign65800_e102565_d_n2;
        locals.var_tmf0_dn4 = assign65800_e102565_d_n4;
        locals.var_tmf0_dn5 = assign65800_e102565_d_n5;
        locals.var_tmf0_dn6 = assign65800_e102565_d_n6;
        locals.var_tmf0_dn7 = assign65800_e102565_d_n7;
        locals.var_tmf0_dn8 = assign65800_e102565_d_n8;
        locals.var_tmf0_dn9 = assign65800_e102565_d_n9;
        locals.var_tmf0_dn10 = assign65800_e102565_d_n10;
        locals.var_tmf0_dn11 = assign65800_e102565_d_n11;
        locals.var_tmf0_dn14 = assign65800_e102565_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign65810_e102582, assign65810_e102582_d_n0, assign65810_e102582_d_n2, assign65810_e102582_d_n4, assign65810_e102582_d_n5, assign65810_e102582_d_n6, assign65810_e102582_d_n7, assign65810_e102582_d_n8, assign65810_e102582_d_n9, assign65810_e102582_d_n10, assign65810_e102582_d_n11, assign65810_e102582_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1570 != 0.0)) {
        let assign65810_e102574: f64 = (0.2 * locals.var_beta);
        let assign65810_e102576: f64 = (assign65810_e102574 * locals.var_xmp);
        let assign65810_e102578: f64 = (assign65810_e102576 * locals.var_dnm);
        let assign65810_e102580: f64 = (assign65810_e102578 / locals.var_arg);
        (assign65810_e102580, ((((((((0.2 * locals.var_beta_dn0) * locals.var_xmp) + (assign65810_e102574 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign65810_e102576 * locals.var_dnm_dn0)) * locals.var_arg) - (assign65810_e102578 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn2) * locals.var_xmp) + (assign65810_e102574 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign65810_e102576 * locals.var_dnm_dn2)) * locals.var_arg) - (assign65810_e102578 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn4) * locals.var_xmp) + (assign65810_e102574 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign65810_e102576 * locals.var_dnm_dn4)) * locals.var_arg) - (assign65810_e102578 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn5) * locals.var_xmp) + (assign65810_e102574 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign65810_e102576 * locals.var_dnm_dn5)) * locals.var_arg) - (assign65810_e102578 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn6) * locals.var_xmp) + (assign65810_e102574 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign65810_e102576 * locals.var_dnm_dn6)) * locals.var_arg) - (assign65810_e102578 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn7) * locals.var_xmp) + (assign65810_e102574 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign65810_e102576 * locals.var_dnm_dn7)) * locals.var_arg) - (assign65810_e102578 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn8) * locals.var_xmp) + (assign65810_e102574 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign65810_e102576 * locals.var_dnm_dn8)) * locals.var_arg) - (assign65810_e102578 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn9) * locals.var_xmp) + (assign65810_e102574 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign65810_e102576 * locals.var_dnm_dn9)) * locals.var_arg) - (assign65810_e102578 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn10) * locals.var_xmp) + (assign65810_e102574 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign65810_e102576 * locals.var_dnm_dn10)) * locals.var_arg) - (assign65810_e102578 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn11) * locals.var_xmp) + (assign65810_e102574 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign65810_e102576 * locals.var_dnm_dn11)) * locals.var_arg) - (assign65810_e102578 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_beta_dn14) * locals.var_xmp) + (assign65810_e102574 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign65810_e102576 * locals.var_dnm_dn14)) * locals.var_arg) - (assign65810_e102578 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign65810_e102582;
        locals.var_t0_dn0 = assign65810_e102582_d_n0;
        locals.var_t0_dn2 = assign65810_e102582_d_n2;
        locals.var_t0_dn4 = assign65810_e102582_d_n4;
        locals.var_t0_dn5 = assign65810_e102582_d_n5;
        locals.var_t0_dn6 = assign65810_e102582_d_n6;
        locals.var_t0_dn7 = assign65810_e102582_d_n7;
        locals.var_t0_dn8 = assign65810_e102582_d_n8;
        locals.var_t0_dn9 = assign65810_e102582_d_n9;
        locals.var_t0_dn10 = assign65810_e102582_d_n10;
        locals.var_t0_dn11 = assign65810_e102582_d_n11;
        locals.var_t0_dn14 = assign65810_e102582_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign65820_e102597, assign65820_e102597_d_n0, assign65820_e102597_d_n2, assign65820_e102597_d_n4, assign65820_e102597_d_n5, assign65820_e102597_d_n6, assign65820_e102597_d_n7, assign65820_e102597_d_n8, assign65820_e102597_d_n9, assign65820_e102597_d_n10, assign65820_e102597_d_n11, assign65820_e102597_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1570 != 0.0)) {
        let assign65820_e102592: f64 = (0.2 * locals.var_beta);
        let assign65820_e102593: f64 = assign65820_e102592;
        let assign65820_e102595: f64 = (assign65820_e102593 - locals.var_tmf0);
        (assign65820_e102595, ((0.2 * locals.var_beta_dn0) - locals.var_tmf0_dn0), ((0.2 * locals.var_beta_dn2) - locals.var_tmf0_dn2), ((0.2 * locals.var_beta_dn4) - locals.var_tmf0_dn4), ((0.2 * locals.var_beta_dn5) - locals.var_tmf0_dn5), ((0.2 * locals.var_beta_dn6) - locals.var_tmf0_dn6), ((0.2 * locals.var_beta_dn7) - locals.var_tmf0_dn7), ((0.2 * locals.var_beta_dn8) - locals.var_tmf0_dn8), ((0.2 * locals.var_beta_dn9) - locals.var_tmf0_dn9), ((0.2 * locals.var_beta_dn10) - locals.var_tmf0_dn10), ((0.2 * locals.var_beta_dn11) - locals.var_tmf0_dn11), ((0.2 * locals.var_beta_dn14) - locals.var_tmf0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign65820_e102597;
        locals.var_t1_dn0 = assign65820_e102597_d_n0;
        locals.var_t1_dn2 = assign65820_e102597_d_n2;
        locals.var_t1_dn4 = assign65820_e102597_d_n4;
        locals.var_t1_dn5 = assign65820_e102597_d_n5;
        locals.var_t1_dn6 = assign65820_e102597_d_n6;
        locals.var_t1_dn7 = assign65820_e102597_d_n7;
        locals.var_t1_dn8 = assign65820_e102597_d_n8;
        locals.var_t1_dn9 = assign65820_e102597_d_n9;
        locals.var_t1_dn10 = assign65820_e102597_d_n10;
        locals.var_t1_dn11 = assign65820_e102597_d_n11;
        locals.var_t1_dn14 = assign65820_e102597_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_248(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign65830_e102606, assign65830_e102606_d_n0, assign65830_e102606_d_n2, assign65830_e102606_d_n4, assign65830_e102606_d_n5, assign65830_e102606_d_n6, assign65830_e102606_d_n7, assign65830_e102606_d_n8, assign65830_e102606_d_n9, assign65830_e102606_d_n10, assign65830_e102606_d_n11, assign65830_e102606_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1570 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign65830_e102606;
        locals.var_t0_dn0 = assign65830_e102606_d_n0;
        locals.var_t0_dn2 = assign65830_e102606_d_n2;
        locals.var_t0_dn4 = assign65830_e102606_d_n4;
        locals.var_t0_dn5 = assign65830_e102606_d_n5;
        locals.var_t0_dn6 = assign65830_e102606_d_n6;
        locals.var_t0_dn7 = assign65830_e102606_d_n7;
        locals.var_t0_dn8 = assign65830_e102606_d_n8;
        locals.var_t0_dn9 = assign65830_e102606_d_n9;
        locals.var_t0_dn10 = assign65830_e102606_d_n10;
        locals.var_t0_dn11 = assign65830_e102606_d_n11;
        locals.var_t0_dn14 = assign65830_e102606_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign65840_e102616, assign65840_e102616_d_n0, assign65840_e102616_d_n2, assign65840_e102616_d_n4, assign65840_e102616_d_n5, assign65840_e102616_d_n6, assign65840_e102616_d_n7, assign65840_e102616_d_n8, assign65840_e102616_d_n9, assign65840_e102616_d_n10, assign65840_e102616_d_n11, assign65840_e102616_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1570 == 0.0)) {
        (locals.var_t1w, locals.var_t1w_dn0, locals.var_t1w_dn2, locals.var_t1w_dn4, locals.var_t1w_dn5, locals.var_t1w_dn6, locals.var_t1w_dn7, locals.var_t1w_dn8, locals.var_t1w_dn9, locals.var_t1w_dn10, locals.var_t1w_dn11, locals.var_t1w_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign65840_e102616;
        locals.var_t1_dn0 = assign65840_e102616_d_n0;
        locals.var_t1_dn2 = assign65840_e102616_d_n2;
        locals.var_t1_dn4 = assign65840_e102616_d_n4;
        locals.var_t1_dn5 = assign65840_e102616_d_n5;
        locals.var_t1_dn6 = assign65840_e102616_d_n6;
        locals.var_t1_dn7 = assign65840_e102616_d_n7;
        locals.var_t1_dn8 = assign65840_e102616_d_n8;
        locals.var_t1_dn9 = assign65840_e102616_d_n9;
        locals.var_t1_dn10 = assign65840_e102616_d_n10;
        locals.var_t1_dn11 = assign65840_e102616_d_n11;
        locals.var_t1_dn14 = assign65840_e102616_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign65850_e102626, assign65850_e102626_d_n0, assign65850_e102626_d_n2, assign65850_e102626_d_n4, assign65850_e102626_d_n5, assign65850_e102626_d_n6, assign65850_e102626_d_n7, assign65850_e102626_d_n8, assign65850_e102626_d_n9, assign65850_e102626_d_n10, assign65850_e102626_d_n11, assign65850_e102626_d_n14,) = {
    if (((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) && (locals.var_guard1570 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign65850_e102626;
        locals.var_t0_dn0 = assign65850_e102626_d_n0;
        locals.var_t0_dn2 = assign65850_e102626_d_n2;
        locals.var_t0_dn4 = assign65850_e102626_d_n4;
        locals.var_t0_dn5 = assign65850_e102626_d_n5;
        locals.var_t0_dn6 = assign65850_e102626_d_n6;
        locals.var_t0_dn7 = assign65850_e102626_d_n7;
        locals.var_t0_dn8 = assign65850_e102626_d_n8;
        locals.var_t0_dn9 = assign65850_e102626_d_n9;
        locals.var_t0_dn10 = assign65850_e102626_d_n10;
        locals.var_t0_dn11 = assign65850_e102626_d_n11;
        locals.var_t0_dn14 = assign65850_e102626_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign65860_e102638, assign65860_e102638_d_n0, assign65860_e102638_d_n2, assign65860_e102638_d_n4, assign65860_e102638_d_n5, assign65860_e102638_d_n6, assign65860_e102638_d_n7, assign65860_e102638_d_n8, assign65860_e102638_d_n9, assign65860_e102638_d_n10, assign65860_e102638_d_n11, assign65860_e102638_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) {
        let assign65860_e102634: f64 = (10.0 * 2.220446049250313e-16);
        let assign65860_e102635: f64 = (locals.var_t1 + assign65860_e102634);
        let assign65860_e102636: f64 = (assign65860_e102635).sqrt();
        (assign65860_e102636, (locals.var_t1_dn0 / (2.0 * assign65860_e102636)), (locals.var_t1_dn2 / (2.0 * assign65860_e102636)), (locals.var_t1_dn4 / (2.0 * assign65860_e102636)), (locals.var_t1_dn5 / (2.0 * assign65860_e102636)), (locals.var_t1_dn6 / (2.0 * assign65860_e102636)), (locals.var_t1_dn7 / (2.0 * assign65860_e102636)), (locals.var_t1_dn8 / (2.0 * assign65860_e102636)), (locals.var_t1_dn9 / (2.0 * assign65860_e102636)), (locals.var_t1_dn10 / (2.0 * assign65860_e102636)), (locals.var_t1_dn11 / (2.0 * assign65860_e102636)), (locals.var_t1_dn14 / (2.0 * assign65860_e102636)),)
    } else {
        (locals.var_sq1npt, locals.var_sq1npt_dn0, locals.var_sq1npt_dn2, locals.var_sq1npt_dn4, locals.var_sq1npt_dn5, locals.var_sq1npt_dn6, locals.var_sq1npt_dn7, locals.var_sq1npt_dn8, locals.var_sq1npt_dn9, locals.var_sq1npt_dn10, locals.var_sq1npt_dn11, locals.var_sq1npt_dn14,)
    }
};
        locals.var_sq1npt = assign65860_e102638;
        locals.var_sq1npt_dn0 = assign65860_e102638_d_n0;
        locals.var_sq1npt_dn2 = assign65860_e102638_d_n2;
        locals.var_sq1npt_dn4 = assign65860_e102638_d_n4;
        locals.var_sq1npt_dn5 = assign65860_e102638_d_n5;
        locals.var_sq1npt_dn6 = assign65860_e102638_d_n6;
        locals.var_sq1npt_dn7 = assign65860_e102638_d_n7;
        locals.var_sq1npt_dn8 = assign65860_e102638_d_n8;
        locals.var_sq1npt_dn9 = assign65860_e102638_d_n9;
        locals.var_sq1npt_dn10 = assign65860_e102638_d_n10;
        locals.var_sq1npt_dn11 = assign65860_e102638_d_n11;
        locals.var_sq1npt_dn14 = assign65860_e102638_d_n14;
        locals.var_sq1npt_rv = 0.0;

        let (assign65870_e102647, assign65870_e102647_d_n0, assign65870_e102647_d_n2, assign65870_e102647_d_n4, assign65870_e102647_d_n5, assign65870_e102647_d_n6, assign65870_e102647_d_n7, assign65870_e102647_d_n8, assign65870_e102647_d_n9, assign65870_e102647_d_n10, assign65870_e102647_d_n11, assign65870_e102647_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) {
        let assign65870_e102645: f64 = (locals.var_conpt0 * locals.var_sq1npt);
        (assign65870_e102645, ((locals.var_conpt0_dn0 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn0)), ((locals.var_conpt0_dn2 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn2)), ((locals.var_conpt0_dn4 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn4)), ((locals.var_conpt0_dn5 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn5)), ((locals.var_conpt0_dn6 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn6)), ((locals.var_conpt0_dn7 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn7)), ((locals.var_conpt0_dn8 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn8)), ((locals.var_conpt0_dn9 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn9)), ((locals.var_conpt0_dn10 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn10)), ((locals.var_conpt0_dn11 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn11)), ((locals.var_conpt0_dn14 * locals.var_sq1npt) + (locals.var_conpt0 * locals.var_sq1npt_dn14)),)
    } else {
        (locals.var_qn0npt, locals.var_qn0npt_dn0, locals.var_qn0npt_dn2, locals.var_qn0npt_dn4, locals.var_qn0npt_dn5, locals.var_qn0npt_dn6, locals.var_qn0npt_dn7, locals.var_qn0npt_dn8, locals.var_qn0npt_dn9, locals.var_qn0npt_dn10, locals.var_qn0npt_dn11, locals.var_qn0npt_dn14,)
    }
};
        locals.var_qn0npt = assign65870_e102647;
        locals.var_qn0npt_dn0 = assign65870_e102647_d_n0;
        locals.var_qn0npt_dn2 = assign65870_e102647_d_n2;
        locals.var_qn0npt_dn4 = assign65870_e102647_d_n4;
        locals.var_qn0npt_dn5 = assign65870_e102647_d_n5;
        locals.var_qn0npt_dn6 = assign65870_e102647_d_n6;
        locals.var_qn0npt_dn7 = assign65870_e102647_d_n7;
        locals.var_qn0npt_dn8 = assign65870_e102647_d_n8;
        locals.var_qn0npt_dn9 = assign65870_e102647_d_n9;
        locals.var_qn0npt_dn10 = assign65870_e102647_d_n10;
        locals.var_qn0npt_dn11 = assign65870_e102647_d_n11;
        locals.var_qn0npt_dn14 = assign65870_e102647_d_n14;
        locals.var_qn0npt_rv = 0.0;

        let (assign65880_e102662, assign65880_e102662_d_n0, assign65880_e102662_d_n2, assign65880_e102662_d_n4, assign65880_e102662_d_n5, assign65880_e102662_d_n6, assign65880_e102662_d_n7, assign65880_e102662_d_n8, assign65880_e102662_d_n9, assign65880_e102662_d_n10, assign65880_e102662_d_n11, assign65880_e102662_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) {
        let assign65880_e102654: f64 = (2.0 * locals.var_beta_inv);
        let assign65880_e102656: f64 = (assign65880_e102654 / locals.var_leff);
        let assign65880_e102658: f64 = (assign65880_e102656 * locals.var_qn0npt);
        let assign65880_e102660: f64 = (assign65880_e102658 * p.p454);
        (assign65880_e102660, (((((2.0 * locals.var_beta_inv_dn0) / locals.var_leff) * locals.var_qn0npt) + (assign65880_e102656 * locals.var_qn0npt_dn0)) * p.p454), (((((2.0 * locals.var_beta_inv_dn2) / locals.var_leff) * locals.var_qn0npt) + (assign65880_e102656 * locals.var_qn0npt_dn2)) * p.p454), (((((2.0 * locals.var_beta_inv_dn4) / locals.var_leff) * locals.var_qn0npt) + (assign65880_e102656 * locals.var_qn0npt_dn4)) * p.p454), (((((2.0 * locals.var_beta_inv_dn5) / locals.var_leff) * locals.var_qn0npt) + (assign65880_e102656 * locals.var_qn0npt_dn5)) * p.p454), (((((2.0 * locals.var_beta_inv_dn6) / locals.var_leff) * locals.var_qn0npt) + (assign65880_e102656 * locals.var_qn0npt_dn6)) * p.p454), (((((2.0 * locals.var_beta_inv_dn7) / locals.var_leff) * locals.var_qn0npt) + (assign65880_e102656 * locals.var_qn0npt_dn7)) * p.p454), (((((2.0 * locals.var_beta_inv_dn8) / locals.var_leff) * locals.var_qn0npt) + (assign65880_e102656 * locals.var_qn0npt_dn8)) * p.p454), (((((2.0 * locals.var_beta_inv_dn9) / locals.var_leff) * locals.var_qn0npt) + (assign65880_e102656 * locals.var_qn0npt_dn9)) * p.p454), (((((2.0 * locals.var_beta_inv_dn10) / locals.var_leff) * locals.var_qn0npt) + (assign65880_e102656 * locals.var_qn0npt_dn10)) * p.p454), (((((2.0 * locals.var_beta_inv_dn11) / locals.var_leff) * locals.var_qn0npt) + (assign65880_e102656 * locals.var_qn0npt_dn11)) * p.p454), (((((2.0 * locals.var_beta_inv_dn14) / locals.var_leff) * locals.var_qn0npt) + (assign65880_e102656 * locals.var_qn0npt_dn14)) * p.p454),)
    } else {
        (locals.var_wk_jnpt_a, locals.var_wk_jnpt_a_dn0, locals.var_wk_jnpt_a_dn2, locals.var_wk_jnpt_a_dn4, locals.var_wk_jnpt_a_dn5, locals.var_wk_jnpt_a_dn6, locals.var_wk_jnpt_a_dn7, locals.var_wk_jnpt_a_dn8, locals.var_wk_jnpt_a_dn9, locals.var_wk_jnpt_a_dn10, locals.var_wk_jnpt_a_dn11, locals.var_wk_jnpt_a_dn14,)
    }
};
        locals.var_wk_jnpt_a = assign65880_e102662;
        locals.var_wk_jnpt_a_dn0 = assign65880_e102662_d_n0;
        locals.var_wk_jnpt_a_dn2 = assign65880_e102662_d_n2;
        locals.var_wk_jnpt_a_dn4 = assign65880_e102662_d_n4;
        locals.var_wk_jnpt_a_dn5 = assign65880_e102662_d_n5;
        locals.var_wk_jnpt_a_dn6 = assign65880_e102662_d_n6;
        locals.var_wk_jnpt_a_dn7 = assign65880_e102662_d_n7;
        locals.var_wk_jnpt_a_dn8 = assign65880_e102662_d_n8;
        locals.var_wk_jnpt_a_dn9 = assign65880_e102662_d_n9;
        locals.var_wk_jnpt_a_dn10 = assign65880_e102662_d_n10;
        locals.var_wk_jnpt_a_dn11 = assign65880_e102662_d_n11;
        locals.var_wk_jnpt_a_dn14 = assign65880_e102662_d_n14;
        locals.var_wk_jnpt_a_rv = 0.0;

        let (assign65890_e102673, assign65890_e102673_d_n0, assign65890_e102673_d_n2, assign65890_e102673_d_n4, assign65890_e102673_d_n5, assign65890_e102673_d_n6, assign65890_e102673_d_n7, assign65890_e102673_d_n8, assign65890_e102673_d_n9, assign65890_e102673_d_n10, assign65890_e102673_d_n11, assign65890_e102673_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) {
        let assign65890_e102669: f64 = (locals.var_wk_jnpt_a * locals.var_weff_nf);
        let assign65890_e102671: f64 = (assign65890_e102669 * locals.var_ty);
        (assign65890_e102671, (((locals.var_wk_jnpt_a_dn0 * locals.var_weff_nf) * locals.var_ty) + (assign65890_e102669 * locals.var_ty_dn0)), (((locals.var_wk_jnpt_a_dn2 * locals.var_weff_nf) * locals.var_ty) + (assign65890_e102669 * locals.var_ty_dn2)), (((locals.var_wk_jnpt_a_dn4 * locals.var_weff_nf) * locals.var_ty) + (assign65890_e102669 * locals.var_ty_dn4)), (((locals.var_wk_jnpt_a_dn5 * locals.var_weff_nf) * locals.var_ty) + (assign65890_e102669 * locals.var_ty_dn5)), (((locals.var_wk_jnpt_a_dn6 * locals.var_weff_nf) * locals.var_ty) + (assign65890_e102669 * locals.var_ty_dn6)), (((locals.var_wk_jnpt_a_dn7 * locals.var_weff_nf) * locals.var_ty) + (assign65890_e102669 * locals.var_ty_dn7)), (((locals.var_wk_jnpt_a_dn8 * locals.var_weff_nf) * locals.var_ty) + (assign65890_e102669 * locals.var_ty_dn8)), (((locals.var_wk_jnpt_a_dn9 * locals.var_weff_nf) * locals.var_ty) + (assign65890_e102669 * locals.var_ty_dn9)), (((locals.var_wk_jnpt_a_dn10 * locals.var_weff_nf) * locals.var_ty) + (assign65890_e102669 * locals.var_ty_dn10)), (((locals.var_wk_jnpt_a_dn11 * locals.var_weff_nf) * locals.var_ty) + (assign65890_e102669 * locals.var_ty_dn11)), (((locals.var_wk_jnpt_a_dn14 * locals.var_weff_nf) * locals.var_ty) + (assign65890_e102669 * locals.var_ty_dn14)),)
    } else {
        (locals.var_idspt1, locals.var_idspt1_dn0, locals.var_idspt1_dn2, locals.var_idspt1_dn4, locals.var_idspt1_dn5, locals.var_idspt1_dn6, locals.var_idspt1_dn7, locals.var_idspt1_dn8, locals.var_idspt1_dn9, locals.var_idspt1_dn10, locals.var_idspt1_dn11, locals.var_idspt1_dn14,)
    }
};
        locals.var_idspt1 = assign65890_e102673;
        locals.var_idspt1_dn0 = assign65890_e102673_d_n0;
        locals.var_idspt1_dn2 = assign65890_e102673_d_n2;
        locals.var_idspt1_dn4 = assign65890_e102673_d_n4;
        locals.var_idspt1_dn5 = assign65890_e102673_d_n5;
        locals.var_idspt1_dn6 = assign65890_e102673_d_n6;
        locals.var_idspt1_dn7 = assign65890_e102673_d_n7;
        locals.var_idspt1_dn8 = assign65890_e102673_d_n8;
        locals.var_idspt1_dn9 = assign65890_e102673_d_n9;
        locals.var_idspt1_dn10 = assign65890_e102673_d_n10;
        locals.var_idspt1_dn11 = assign65890_e102673_d_n11;
        locals.var_idspt1_dn14 = assign65890_e102673_d_n14;
        locals.var_idspt1_rv = 0.0;

        let (assign65900_e102682, assign65900_e102682_d_n0, assign65900_e102682_d_n2, assign65900_e102682_d_n4, assign65900_e102682_d_n5, assign65900_e102682_d_n6, assign65900_e102682_d_n7, assign65900_e102682_d_n8, assign65900_e102682_d_n9, assign65900_e102682_d_n10, assign65900_e102682_d_n11, assign65900_e102682_d_n14,) = {
    if ((locals.var_guard445 == 0.0) && (locals.var_guard1521 != 0.0)) {
        let assign65900_e102680: f64 = (locals.var_idsorg + locals.var_idspt1);
        (assign65900_e102680, (locals.var_idsorg_dn0 + locals.var_idspt1_dn0), (locals.var_idsorg_dn2 + locals.var_idspt1_dn2), (locals.var_idsorg_dn4 + locals.var_idspt1_dn4), (locals.var_idsorg_dn5 + locals.var_idspt1_dn5), (locals.var_idsorg_dn6 + locals.var_idspt1_dn6), (locals.var_idsorg_dn7 + locals.var_idspt1_dn7), (locals.var_idsorg_dn8 + locals.var_idspt1_dn8), (locals.var_idsorg_dn9 + locals.var_idspt1_dn9), (locals.var_idsorg_dn10 + locals.var_idspt1_dn10), (locals.var_idsorg_dn11 + locals.var_idspt1_dn11), (locals.var_idsorg_dn14 + locals.var_idspt1_dn14),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn14,)
    }
};
        locals.var_ids = assign65900_e102682;
        locals.var_ids_dn0 = assign65900_e102682_d_n0;
        locals.var_ids_dn2 = assign65900_e102682_d_n2;
        locals.var_ids_dn4 = assign65900_e102682_d_n4;
        locals.var_ids_dn5 = assign65900_e102682_d_n5;
        locals.var_ids_dn6 = assign65900_e102682_d_n6;
        locals.var_ids_dn7 = assign65900_e102682_d_n7;
        locals.var_ids_dn8 = assign65900_e102682_d_n8;
        locals.var_ids_dn9 = assign65900_e102682_d_n9;
        locals.var_ids_dn10 = assign65900_e102682_d_n10;
        locals.var_ids_dn11 = assign65900_e102682_d_n11;
        locals.var_ids_dn14 = assign65900_e102682_d_n14;
        locals.var_ids_rv = 0.0;

        let (assign65910_e102689, assign65910_e102689_d_n0, assign65910_e102689_d_n2, assign65910_e102689_d_n4, assign65910_e102689_d_n5, assign65910_e102689_d_n6, assign65910_e102689_d_n7, assign65910_e102689_d_n8, assign65910_e102689_d_n9, assign65910_e102689_d_n10, assign65910_e102689_d_n11, assign65910_e102689_d_n14,) = {
    if (locals.var_guard445 == 0.0) {
        let assign65910_e102687: f64 = (locals.var_idsorg + locals.var_idspt1);
        (assign65910_e102687, (locals.var_idsorg_dn0 + locals.var_idspt1_dn0), (locals.var_idsorg_dn2 + locals.var_idspt1_dn2), (locals.var_idsorg_dn4 + locals.var_idspt1_dn4), (locals.var_idsorg_dn5 + locals.var_idspt1_dn5), (locals.var_idsorg_dn6 + locals.var_idspt1_dn6), (locals.var_idsorg_dn7 + locals.var_idspt1_dn7), (locals.var_idsorg_dn8 + locals.var_idspt1_dn8), (locals.var_idsorg_dn9 + locals.var_idspt1_dn9), (locals.var_idsorg_dn10 + locals.var_idspt1_dn10), (locals.var_idsorg_dn11 + locals.var_idspt1_dn11), (locals.var_idsorg_dn14 + locals.var_idspt1_dn14),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn14,)
    }
};
        locals.var_ids = assign65910_e102689;
        locals.var_ids_dn0 = assign65910_e102689_d_n0;
        locals.var_ids_dn2 = assign65910_e102689_d_n2;
        locals.var_ids_dn4 = assign65910_e102689_d_n4;
        locals.var_ids_dn5 = assign65910_e102689_d_n5;
        locals.var_ids_dn6 = assign65910_e102689_d_n6;
        locals.var_ids_dn7 = assign65910_e102689_d_n7;
        locals.var_ids_dn8 = assign65910_e102689_d_n8;
        locals.var_ids_dn9 = assign65910_e102689_d_n9;
        locals.var_ids_dn10 = assign65910_e102689_d_n10;
        locals.var_ids_dn11 = assign65910_e102689_d_n11;
        locals.var_ids_dn14 = assign65910_e102689_d_n14;
        locals.var_ids_rv = 0.0;

        let (assign65930_e102701, assign65930_e102701_d_n0, assign65930_e102701_d_n2, assign65930_e102701_d_n4, assign65930_e102701_d_n5, assign65930_e102701_d_n6, assign65930_e102701_d_n7, assign65930_e102701_d_n8, assign65930_e102701_d_n9, assign65930_e102701_d_n10, assign65930_e102701_d_n11, assign65930_e102701_d_n14,) = {
    if (locals.var_guard445 == 0.0) {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn14,)
    } else {
        (locals.var_qiu_noi, locals.var_qiu_noi_dn0, locals.var_qiu_noi_dn2, locals.var_qiu_noi_dn4, locals.var_qiu_noi_dn5, locals.var_qiu_noi_dn6, locals.var_qiu_noi_dn7, locals.var_qiu_noi_dn8, locals.var_qiu_noi_dn9, locals.var_qiu_noi_dn10, locals.var_qiu_noi_dn11, locals.var_qiu_noi_dn14,)
    }
};
        locals.var_qiu_noi = assign65930_e102701;
        locals.var_qiu_noi_dn0 = assign65930_e102701_d_n0;
        locals.var_qiu_noi_dn2 = assign65930_e102701_d_n2;
        locals.var_qiu_noi_dn4 = assign65930_e102701_d_n4;
        locals.var_qiu_noi_dn5 = assign65930_e102701_d_n5;
        locals.var_qiu_noi_dn6 = assign65930_e102701_d_n6;
        locals.var_qiu_noi_dn7 = assign65930_e102701_d_n7;
        locals.var_qiu_noi_dn8 = assign65930_e102701_d_n8;
        locals.var_qiu_noi_dn9 = assign65930_e102701_d_n9;
        locals.var_qiu_noi_dn10 = assign65930_e102701_d_n10;
        locals.var_qiu_noi_dn11 = assign65930_e102701_d_n11;
        locals.var_qiu_noi_dn14 = assign65930_e102701_d_n14;
        locals.var_qiu_noi_rv = 0.0;

        let assign65940_e102703: f64 = (-locals.var_weffcv_nf);
        let assign65940_e102705: f64 = (assign65940_e102703 * locals.var_leff);
        locals.var_t1 = assign65940_e102705;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign65950_e102708: f64 = (locals.var_t1 * locals.var_qbu);
        locals.var_qb = assign65950_e102708;
        locals.var_qb_dn0 = ((locals.var_t1_dn0 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn0));
        locals.var_qb_dn2 = ((locals.var_t1_dn2 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn2));
        locals.var_qb_dn4 = ((locals.var_t1_dn4 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn4));
        locals.var_qb_dn5 = ((locals.var_t1_dn5 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn5));
        locals.var_qb_dn6 = ((locals.var_t1_dn6 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn6));
        locals.var_qb_dn7 = ((locals.var_t1_dn7 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn7));
        locals.var_qb_dn8 = ((locals.var_t1_dn8 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn8));
        locals.var_qb_dn9 = ((locals.var_t1_dn9 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn9));
        locals.var_qb_dn10 = ((locals.var_t1_dn10 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn10));
        locals.var_qb_dn11 = ((locals.var_t1_dn11 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn11));
        locals.var_qb_dn14 = ((locals.var_t1_dn14 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn14));
        locals.var_qb_rv = 0.0;

        let assign65960_e102711: f64 = (locals.var_t1 * locals.var_qiu);
        locals.var_qi = assign65960_e102711;
        locals.var_qi_dn0 = ((locals.var_t1_dn0 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn0));
        locals.var_qi_dn2 = ((locals.var_t1_dn2 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn2));
        locals.var_qi_dn4 = ((locals.var_t1_dn4 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn4));
        locals.var_qi_dn5 = ((locals.var_t1_dn5 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn5));
        locals.var_qi_dn6 = ((locals.var_t1_dn6 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn6));
        locals.var_qi_dn7 = ((locals.var_t1_dn7 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn7));
        locals.var_qi_dn8 = ((locals.var_t1_dn8 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn8));
        locals.var_qi_dn9 = ((locals.var_t1_dn9 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn9));
        locals.var_qi_dn10 = ((locals.var_t1_dn10 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn10));
        locals.var_qi_dn11 = ((locals.var_t1_dn11 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn11));
        locals.var_qi_dn14 = ((locals.var_t1_dn14 * locals.var_qiu) + (locals.var_t1 * locals.var_qiu_dn14));
        locals.var_qi_rv = 0.0;

        let assign65970_e102714: f64 = (locals.var_qi * locals.var_qdrat);
        locals.var_qd = assign65970_e102714;
        locals.var_qd_dn0 = ((locals.var_qi_dn0 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn0));
        locals.var_qd_dn2 = ((locals.var_qi_dn2 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn2));
        locals.var_qd_dn4 = ((locals.var_qi_dn4 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn4));
        locals.var_qd_dn5 = ((locals.var_qi_dn5 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn5));
        locals.var_qd_dn6 = ((locals.var_qi_dn6 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn6));
        locals.var_qd_dn7 = ((locals.var_qi_dn7 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn7));
        locals.var_qd_dn8 = ((locals.var_qi_dn8 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn8));
        locals.var_qd_dn9 = ((locals.var_qi_dn9 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn9));
        locals.var_qd_dn10 = ((locals.var_qi_dn10 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn10));
        locals.var_qd_dn11 = ((locals.var_qi_dn11 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn11));
        locals.var_qd_dn14 = ((locals.var_qi_dn14 * locals.var_qdrat) + (locals.var_qi * locals.var_qdrat_dn14));
        locals.var_qd_rv = 0.0;

        let assign65980_e102717: f64 = (locals.var_t1 * locals.var_qiu_noi);
        locals.var_qi_noi = assign65980_e102717;
        locals.var_qi_noi_dn0 = ((locals.var_t1_dn0 * locals.var_qiu_noi) + (locals.var_t1 * locals.var_qiu_noi_dn0));
        locals.var_qi_noi_dn2 = ((locals.var_t1_dn2 * locals.var_qiu_noi) + (locals.var_t1 * locals.var_qiu_noi_dn2));
        locals.var_qi_noi_dn4 = ((locals.var_t1_dn4 * locals.var_qiu_noi) + (locals.var_t1 * locals.var_qiu_noi_dn4));
        locals.var_qi_noi_dn5 = ((locals.var_t1_dn5 * locals.var_qiu_noi) + (locals.var_t1 * locals.var_qiu_noi_dn5));
        locals.var_qi_noi_dn6 = ((locals.var_t1_dn6 * locals.var_qiu_noi) + (locals.var_t1 * locals.var_qiu_noi_dn6));
        locals.var_qi_noi_dn7 = ((locals.var_t1_dn7 * locals.var_qiu_noi) + (locals.var_t1 * locals.var_qiu_noi_dn7));
        locals.var_qi_noi_dn8 = ((locals.var_t1_dn8 * locals.var_qiu_noi) + (locals.var_t1 * locals.var_qiu_noi_dn8));
        locals.var_qi_noi_dn9 = ((locals.var_t1_dn9 * locals.var_qiu_noi) + (locals.var_t1 * locals.var_qiu_noi_dn9));
        locals.var_qi_noi_dn10 = ((locals.var_t1_dn10 * locals.var_qiu_noi) + (locals.var_t1 * locals.var_qiu_noi_dn10));
        locals.var_qi_noi_dn11 = ((locals.var_t1_dn11 * locals.var_qiu_noi) + (locals.var_t1 * locals.var_qiu_noi_dn11));
        locals.var_qi_noi_dn14 = ((locals.var_t1_dn14 * locals.var_qiu_noi) + (locals.var_t1 * locals.var_qiu_noi_dn14));
        locals.var_qi_noi_rv = 0.0;

        let assign65990_e102720: f64 = (locals.var_vds - locals.var_pds);
        let assign65990_e102722: f64 = (assign65990_e102720 / 2.0);
        locals.var_t1 = assign65990_e102722;
        locals.var_t1_dn0 = ((locals.var_vds_dn0 - locals.var_pds_dn0) / 2.0);
        locals.var_t1_dn2 = ((locals.var_vds_dn2 - locals.var_pds_dn2) / 2.0);
        locals.var_t1_dn4 = ((locals.var_vds_dn4 - locals.var_pds_dn4) / 2.0);
        locals.var_t1_dn5 = ((locals.var_vds_dn5 - locals.var_pds_dn5) / 2.0);
        locals.var_t1_dn6 = ((locals.var_vds_dn6 - locals.var_pds_dn6) / 2.0);
        locals.var_t1_dn7 = ((locals.var_vds_dn7 - locals.var_pds_dn7) / 2.0);
        locals.var_t1_dn8 = ((locals.var_vds_dn8 - locals.var_pds_dn8) / 2.0);
        locals.var_t1_dn9 = ((locals.var_vds_dn9 - locals.var_pds_dn9) / 2.0);
        locals.var_t1_dn10 = ((locals.var_vds_dn10 - locals.var_pds_dn10) / 2.0);
        locals.var_t1_dn11 = ((locals.var_vds_dn11 - locals.var_pds_dn11) / 2.0);
        locals.var_t1_dn14 = ((locals.var_vds_dn14 - locals.var_pds_dn14) / 2.0);
        locals.var_t1_rv = 0.0;

        let assign66000_e102725: f64 = (2.0 * locals.var_t1);
        let assign66000_e102727: f64 = (assign66000_e102725 / p.p263);
        locals.var_tmf1 = assign66000_e102727;
        locals.var_tmf1_dn0 = ((2.0 * locals.var_t1_dn0) / p.p263);
        locals.var_tmf1_dn2 = ((2.0 * locals.var_t1_dn2) / p.p263);
        locals.var_tmf1_dn4 = ((2.0 * locals.var_t1_dn4) / p.p263);
        locals.var_tmf1_dn5 = ((2.0 * locals.var_t1_dn5) / p.p263);
        locals.var_tmf1_dn6 = ((2.0 * locals.var_t1_dn6) / p.p263);
        locals.var_tmf1_dn7 = ((2.0 * locals.var_t1_dn7) / p.p263);
        locals.var_tmf1_dn8 = ((2.0 * locals.var_t1_dn8) / p.p263);
        locals.var_tmf1_dn9 = ((2.0 * locals.var_t1_dn9) / p.p263);
        locals.var_tmf1_dn10 = ((2.0 * locals.var_t1_dn10) / p.p263);
        locals.var_tmf1_dn11 = ((2.0 * locals.var_t1_dn11) / p.p263);
        locals.var_tmf1_dn14 = ((2.0 * locals.var_t1_dn14) / p.p263);
        locals.var_tmf1_rv = 0.0;

        let assign66010_e102732: f64 = (1.0 / 2.0);
        let assign66010_e102736: f64 = (1.0 / 6.0);
        let assign66010_e102740: f64 = (1.0 / 24.0);
        let assign66010_e102744: f64 = (1.0 / 120.0);
        let assign66010_e102748: f64 = (1.0 / 720.0);
        let assign66010_e102752: f64 = (1.0 / 5040.0);
        let assign66010_e102753: f64 = (locals.var_tmf1 * assign66010_e102752);
        let assign66010_e102754: f64 = (assign66010_e102748 + assign66010_e102753);
        let assign66010_e102755: f64 = (locals.var_tmf1 * assign66010_e102754);
        let assign66010_e102756: f64 = (assign66010_e102744 + assign66010_e102755);
        let assign66010_e102757: f64 = (locals.var_tmf1 * assign66010_e102756);
        let assign66010_e102758: f64 = (assign66010_e102740 + assign66010_e102757);
        let assign66010_e102759: f64 = (locals.var_tmf1 * assign66010_e102758);
        let assign66010_e102760: f64 = (assign66010_e102736 + assign66010_e102759);
        let assign66010_e102761: f64 = (locals.var_tmf1 * assign66010_e102760);
        let assign66010_e102762: f64 = (assign66010_e102732 + assign66010_e102761);
        let assign66010_e102763: f64 = (locals.var_tmf1 * assign66010_e102762);
        let assign66010_e102764: f64 = (1.0 + assign66010_e102763);
        locals.var_tmf2 = assign66010_e102764;
        locals.var_tmf2_dn0 = ((locals.var_tmf1_dn0 * assign66010_e102762) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign66010_e102760) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign66010_e102758) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign66010_e102756) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign66010_e102754) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign66010_e102752)))))))))));
        locals.var_tmf2_dn2 = ((locals.var_tmf1_dn2 * assign66010_e102762) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign66010_e102760) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign66010_e102758) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign66010_e102756) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign66010_e102754) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign66010_e102752)))))))))));
        locals.var_tmf2_dn4 = ((locals.var_tmf1_dn4 * assign66010_e102762) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign66010_e102760) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign66010_e102758) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign66010_e102756) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign66010_e102754) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign66010_e102752)))))))))));
        locals.var_tmf2_dn5 = ((locals.var_tmf1_dn5 * assign66010_e102762) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign66010_e102760) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign66010_e102758) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign66010_e102756) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign66010_e102754) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign66010_e102752)))))))))));
        locals.var_tmf2_dn6 = ((locals.var_tmf1_dn6 * assign66010_e102762) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign66010_e102760) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign66010_e102758) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign66010_e102756) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign66010_e102754) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign66010_e102752)))))))))));
        locals.var_tmf2_dn7 = ((locals.var_tmf1_dn7 * assign66010_e102762) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign66010_e102760) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign66010_e102758) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign66010_e102756) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign66010_e102754) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign66010_e102752)))))))))));
        locals.var_tmf2_dn8 = ((locals.var_tmf1_dn8 * assign66010_e102762) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign66010_e102760) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign66010_e102758) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign66010_e102756) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign66010_e102754) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign66010_e102752)))))))))));
        locals.var_tmf2_dn9 = ((locals.var_tmf1_dn9 * assign66010_e102762) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign66010_e102760) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign66010_e102758) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign66010_e102756) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign66010_e102754) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign66010_e102752)))))))))));
        locals.var_tmf2_dn10 = ((locals.var_tmf1_dn10 * assign66010_e102762) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign66010_e102760) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign66010_e102758) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign66010_e102756) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign66010_e102754) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign66010_e102752)))))))))));
        locals.var_tmf2_dn11 = ((locals.var_tmf1_dn11 * assign66010_e102762) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign66010_e102760) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign66010_e102758) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign66010_e102756) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign66010_e102754) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign66010_e102752)))))))))));
        locals.var_tmf2_dn14 = ((locals.var_tmf1_dn14 * assign66010_e102762) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign66010_e102760) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign66010_e102758) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign66010_e102756) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign66010_e102754) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign66010_e102752)))))))))));
        locals.var_tmf2_rv = 0.0;

        let assign66020_e102767: f64 = (1.0 / 2.0);
        let assign66020_e102771: f64 = (1.0 / 3.0);
        let assign66020_e102775: f64 = (1.0 / 8.0);
        let assign66020_e102779: f64 = (1.0 / 30.0);
        let assign66020_e102783: f64 = (1.0 / 144.0);
        let assign66020_e102787: f64 = (1.0 / 840.0);
        let assign66020_e102788: f64 = (locals.var_tmf1 * assign66020_e102787);
        let assign66020_e102789: f64 = (assign66020_e102783 + assign66020_e102788);
        let assign66020_e102790: f64 = (locals.var_tmf1 * assign66020_e102789);
        let assign66020_e102791: f64 = (assign66020_e102779 + assign66020_e102790);
        let assign66020_e102792: f64 = (locals.var_tmf1 * assign66020_e102791);
        let assign66020_e102793: f64 = (assign66020_e102775 + assign66020_e102792);
        let assign66020_e102794: f64 = (locals.var_tmf1 * assign66020_e102793);
        let assign66020_e102795: f64 = (assign66020_e102771 + assign66020_e102794);
        let assign66020_e102796: f64 = (locals.var_tmf1 * assign66020_e102795);
        let assign66020_e102797: f64 = (assign66020_e102767 + assign66020_e102796);
        locals.var_tmf3 = assign66020_e102797;
        locals.var_tmf3_dn0 = ((locals.var_tmf1_dn0 * assign66020_e102795) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign66020_e102793) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign66020_e102791) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign66020_e102789) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign66020_e102787)))))))));
        locals.var_tmf3_dn2 = ((locals.var_tmf1_dn2 * assign66020_e102795) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign66020_e102793) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign66020_e102791) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign66020_e102789) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign66020_e102787)))))))));
        locals.var_tmf3_dn4 = ((locals.var_tmf1_dn4 * assign66020_e102795) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign66020_e102793) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign66020_e102791) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign66020_e102789) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign66020_e102787)))))))));
        locals.var_tmf3_dn5 = ((locals.var_tmf1_dn5 * assign66020_e102795) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign66020_e102793) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign66020_e102791) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign66020_e102789) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign66020_e102787)))))))));
        locals.var_tmf3_dn6 = ((locals.var_tmf1_dn6 * assign66020_e102795) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign66020_e102793) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign66020_e102791) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign66020_e102789) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign66020_e102787)))))))));
        locals.var_tmf3_dn7 = ((locals.var_tmf1_dn7 * assign66020_e102795) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign66020_e102793) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign66020_e102791) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign66020_e102789) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign66020_e102787)))))))));
        locals.var_tmf3_dn8 = ((locals.var_tmf1_dn8 * assign66020_e102795) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign66020_e102793) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign66020_e102791) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign66020_e102789) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign66020_e102787)))))))));
        locals.var_tmf3_dn9 = ((locals.var_tmf1_dn9 * assign66020_e102795) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign66020_e102793) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign66020_e102791) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign66020_e102789) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign66020_e102787)))))))));
        locals.var_tmf3_dn10 = ((locals.var_tmf1_dn10 * assign66020_e102795) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign66020_e102793) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign66020_e102791) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign66020_e102789) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign66020_e102787)))))))));
        locals.var_tmf3_dn11 = ((locals.var_tmf1_dn11 * assign66020_e102795) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign66020_e102793) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign66020_e102791) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign66020_e102789) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign66020_e102787)))))))));
        locals.var_tmf3_dn14 = ((locals.var_tmf1_dn14 * assign66020_e102795) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign66020_e102793) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign66020_e102791) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign66020_e102789) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign66020_e102787)))))))));
        locals.var_tmf3_rv = 0.0;

        let assign66030_e102800: f64 = (p.p263 / locals.var_tmf2);
        locals.var_pzadd = assign66030_e102800;
        locals.var_pzadd_dn0 = (-((p.p263 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn2 = (-((p.p263 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn4 = (-((p.p263 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn5 = (-((p.p263 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn6 = (-((p.p263 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn7 = (-((p.p263 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn8 = (-((p.p263 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn9 = (-((p.p263 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn10 = (-((p.p263 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn11 = (-((p.p263 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_dn14 = (-((p.p263 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_pzadd_rv = 0.0;

        let assign66040_e102802: f64 = (-2.0);
        let assign66040_e102804: f64 = (assign66040_e102802 * locals.var_tmf3);
        let assign66040_e102807: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign66040_e102808: f64 = (assign66040_e102804 / assign66040_e102807);
        locals.var_t2 = assign66040_e102808;
        locals.var_t2_dn0 = ((((assign66040_e102802 * locals.var_tmf3_dn0) * assign66040_e102807) - (assign66040_e102804 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign66040_e102807 * assign66040_e102807));
        locals.var_t2_dn2 = ((((assign66040_e102802 * locals.var_tmf3_dn2) * assign66040_e102807) - (assign66040_e102804 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign66040_e102807 * assign66040_e102807));
        locals.var_t2_dn4 = ((((assign66040_e102802 * locals.var_tmf3_dn4) * assign66040_e102807) - (assign66040_e102804 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign66040_e102807 * assign66040_e102807));
        locals.var_t2_dn5 = ((((assign66040_e102802 * locals.var_tmf3_dn5) * assign66040_e102807) - (assign66040_e102804 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign66040_e102807 * assign66040_e102807));
        locals.var_t2_dn6 = ((((assign66040_e102802 * locals.var_tmf3_dn6) * assign66040_e102807) - (assign66040_e102804 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign66040_e102807 * assign66040_e102807));
        locals.var_t2_dn7 = ((((assign66040_e102802 * locals.var_tmf3_dn7) * assign66040_e102807) - (assign66040_e102804 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign66040_e102807 * assign66040_e102807));
        locals.var_t2_dn8 = ((((assign66040_e102802 * locals.var_tmf3_dn8) * assign66040_e102807) - (assign66040_e102804 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign66040_e102807 * assign66040_e102807));
        locals.var_t2_dn9 = ((((assign66040_e102802 * locals.var_tmf3_dn9) * assign66040_e102807) - (assign66040_e102804 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign66040_e102807 * assign66040_e102807));
        locals.var_t2_dn10 = ((((assign66040_e102802 * locals.var_tmf3_dn10) * assign66040_e102807) - (assign66040_e102804 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign66040_e102807 * assign66040_e102807));
        locals.var_t2_dn11 = ((((assign66040_e102802 * locals.var_tmf3_dn11) * assign66040_e102807) - (assign66040_e102804 * ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)))) / (assign66040_e102807 * assign66040_e102807));
        locals.var_t2_dn14 = ((((assign66040_e102802 * locals.var_tmf3_dn14) * assign66040_e102807) - (assign66040_e102804 * ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)))) / (assign66040_e102807 * assign66040_e102807));
        locals.var_t2_rv = 0.0;

        let assign66050_e102812: f64 = (10.0 * 2.220446049250313e-16);
        let assign66050_e102815: f64 = (10.0 * 2.220446049250313e-16);
        let assign66050_e102816: f64 = (assign66050_e102812 + assign66050_e102815);
        let assign66050_e102820: f64 = (10.0 * 2.220446049250313e-16);
        let assign66050_e102823: f64 = if ((locals.var_pzadd < assign66050_e102816) && (assign66050_e102820 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1576 = assign66050_e102823;
        locals.var_guard1576_rv = 0.0;

        let (assign66060_e102835, assign66060_e102835_d_n0, assign66060_e102835_d_n2, assign66060_e102835_d_n4, assign66060_e102835_d_n5, assign66060_e102835_d_n6, assign66060_e102835_d_n7, assign66060_e102835_d_n8, assign66060_e102835_d_n9, assign66060_e102835_d_n10, assign66060_e102835_d_n11, assign66060_e102835_d_n14,) = {
    if (locals.var_guard1576 != 0.0) {
        let assign66060_e102827: f64 = (10.0 * 2.220446049250313e-16);
        let assign66060_e102830: f64 = (10.0 * 2.220446049250313e-16);
        let assign66060_e102831: f64 = (assign66060_e102827 + assign66060_e102830);
        let assign66060_e102833: f64 = (assign66060_e102831 - locals.var_pzadd);
        (assign66060_e102833, (-locals.var_pzadd_dn0), (-locals.var_pzadd_dn2), (-locals.var_pzadd_dn4), (-locals.var_pzadd_dn5), (-locals.var_pzadd_dn6), (-locals.var_pzadd_dn7), (-locals.var_pzadd_dn8), (-locals.var_pzadd_dn9), (-locals.var_pzadd_dn10), (-locals.var_pzadd_dn11), (-locals.var_pzadd_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign66060_e102835;
        locals.var_tmf1_dn0 = assign66060_e102835_d_n0;
        locals.var_tmf1_dn2 = assign66060_e102835_d_n2;
        locals.var_tmf1_dn4 = assign66060_e102835_d_n4;
        locals.var_tmf1_dn5 = assign66060_e102835_d_n5;
        locals.var_tmf1_dn6 = assign66060_e102835_d_n6;
        locals.var_tmf1_dn7 = assign66060_e102835_d_n7;
        locals.var_tmf1_dn8 = assign66060_e102835_d_n8;
        locals.var_tmf1_dn9 = assign66060_e102835_d_n9;
        locals.var_tmf1_dn10 = assign66060_e102835_d_n10;
        locals.var_tmf1_dn11 = assign66060_e102835_d_n11;
        locals.var_tmf1_dn14 = assign66060_e102835_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign66070_e102841, assign66070_e102841_d_n0, assign66070_e102841_d_n2, assign66070_e102841_d_n4, assign66070_e102841_d_n5, assign66070_e102841_d_n6, assign66070_e102841_d_n7, assign66070_e102841_d_n8, assign66070_e102841_d_n9, assign66070_e102841_d_n10, assign66070_e102841_d_n11, assign66070_e102841_d_n14,) = {
    if (locals.var_guard1576 != 0.0) {
        let assign66070_e102839: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign66070_e102839, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign66070_e102841;
        locals.var_x2_dn0 = assign66070_e102841_d_n0;
        locals.var_x2_dn2 = assign66070_e102841_d_n2;
        locals.var_x2_dn4 = assign66070_e102841_d_n4;
        locals.var_x2_dn5 = assign66070_e102841_d_n5;
        locals.var_x2_dn6 = assign66070_e102841_d_n6;
        locals.var_x2_dn7 = assign66070_e102841_d_n7;
        locals.var_x2_dn8 = assign66070_e102841_d_n8;
        locals.var_x2_dn9 = assign66070_e102841_d_n9;
        locals.var_x2_dn10 = assign66070_e102841_d_n10;
        locals.var_x2_dn11 = assign66070_e102841_d_n11;
        locals.var_x2_dn14 = assign66070_e102841_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign66080_e102851, assign66080_e102851_d_n0, assign66080_e102851_d_n2, assign66080_e102851_d_n4, assign66080_e102851_d_n5, assign66080_e102851_d_n6, assign66080_e102851_d_n7, assign66080_e102851_d_n8, assign66080_e102851_d_n9, assign66080_e102851_d_n10, assign66080_e102851_d_n11, assign66080_e102851_d_n14,) = {
    if (locals.var_guard1576 != 0.0) {
        let assign66080_e102845: f64 = (10.0 * 2.220446049250313e-16);
        let assign66080_e102848: f64 = (10.0 * 2.220446049250313e-16);
        let assign66080_e102849: f64 = (assign66080_e102845 * assign66080_e102848);
        (assign66080_e102849, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign66080_e102851;
        locals.var_xmax2_dn0 = assign66080_e102851_d_n0;
        locals.var_xmax2_dn2 = assign66080_e102851_d_n2;
        locals.var_xmax2_dn4 = assign66080_e102851_d_n4;
        locals.var_xmax2_dn5 = assign66080_e102851_d_n5;
        locals.var_xmax2_dn6 = assign66080_e102851_d_n6;
        locals.var_xmax2_dn7 = assign66080_e102851_d_n7;
        locals.var_xmax2_dn8 = assign66080_e102851_d_n8;
        locals.var_xmax2_dn9 = assign66080_e102851_d_n9;
        locals.var_xmax2_dn10 = assign66080_e102851_d_n10;
        locals.var_xmax2_dn11 = assign66080_e102851_d_n11;
        locals.var_xmax2_dn14 = assign66080_e102851_d_n14;
        locals.var_xmax2_rv = 0.0;

    }
}

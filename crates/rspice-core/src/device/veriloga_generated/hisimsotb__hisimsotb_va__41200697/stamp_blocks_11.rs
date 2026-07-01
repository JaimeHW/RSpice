#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_86(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20950_e26464, assign20950_e26464_d_n0, assign20950_e26464_d_n2, assign20950_e26464_d_n4, assign20950_e26464_d_n5, assign20950_e26464_d_n6, assign20950_e26464_d_n8, assign20950_e26464_d_n10, assign20950_e26464_d_n11, assign20950_e26464_d_n12,) = {
    if ((locals.var_guard327 == 0.0) && (locals.var_guard362 == 0.0)) {
        let assign20950_e26458: f64 = (locals.var_modervs * p.p174);
        let assign20950_e26461: f64 = (locals.var_modenml * p.p173);
        let assign20950_e26462: f64 = (assign20950_e26458 + assign20950_e26461);
        (assign20950_e26462, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn4, locals.var_cgdoe_dn5, locals.var_cgdoe_dn6, locals.var_cgdoe_dn8, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12,)
    }
};
        locals.var_cgdoe = assign20950_e26464;
        locals.var_cgdoe_dn0 = assign20950_e26464_d_n0;
        locals.var_cgdoe_dn2 = assign20950_e26464_d_n2;
        locals.var_cgdoe_dn4 = assign20950_e26464_d_n4;
        locals.var_cgdoe_dn5 = assign20950_e26464_d_n5;
        locals.var_cgdoe_dn6 = assign20950_e26464_d_n6;
        locals.var_cgdoe_dn8 = assign20950_e26464_d_n8;
        locals.var_cgdoe_dn10 = assign20950_e26464_d_n10;
        locals.var_cgdoe_dn11 = assign20950_e26464_d_n11;
        locals.var_cgdoe_dn12 = assign20950_e26464_d_n12;
        locals.var_cgdoe_rv = 0.0;

        let (assign20960_e26475, assign20960_e26475_d_n0, assign20960_e26475_d_n2, assign20960_e26475_d_n4, assign20960_e26475_d_n5, assign20960_e26475_d_n6, assign20960_e26475_d_n8, assign20960_e26475_d_n10, assign20960_e26475_d_n11, assign20960_e26475_d_n12,) = {
    if ((locals.var_guard327 == 0.0) && (locals.var_guard362 == 0.0)) {
        let assign20960_e26472: f64 = (-locals.var_weffcv_nf);
        let assign20960_e26473: f64 = (locals.var_cgdoe * assign20960_e26472);
        (assign20960_e26473, ((locals.var_cgdoe_dn0 * assign20960_e26472) + (locals.var_cgdoe * (-locals.var_weffcv_nf_dn0))), ((locals.var_cgdoe_dn2 * assign20960_e26472) + (locals.var_cgdoe * (-locals.var_weffcv_nf_dn2))), ((locals.var_cgdoe_dn4 * assign20960_e26472) + (locals.var_cgdoe * (-locals.var_weffcv_nf_dn4))), ((locals.var_cgdoe_dn5 * assign20960_e26472) + (locals.var_cgdoe * (-locals.var_weffcv_nf_dn5))), ((locals.var_cgdoe_dn6 * assign20960_e26472) + (locals.var_cgdoe * (-locals.var_weffcv_nf_dn6))), ((locals.var_cgdoe_dn8 * assign20960_e26472) + (locals.var_cgdoe * (-locals.var_weffcv_nf_dn8))), ((locals.var_cgdoe_dn10 * assign20960_e26472) + (locals.var_cgdoe * (-locals.var_weffcv_nf_dn10))), ((locals.var_cgdoe_dn11 * assign20960_e26472) + (locals.var_cgdoe * (-locals.var_weffcv_nf_dn11))), ((locals.var_cgdoe_dn12 * assign20960_e26472) + (locals.var_cgdoe * (-locals.var_weffcv_nf_dn12))),)
    } else {
        (locals.var_cgdoe, locals.var_cgdoe_dn0, locals.var_cgdoe_dn2, locals.var_cgdoe_dn4, locals.var_cgdoe_dn5, locals.var_cgdoe_dn6, locals.var_cgdoe_dn8, locals.var_cgdoe_dn10, locals.var_cgdoe_dn11, locals.var_cgdoe_dn12,)
    }
};
        locals.var_cgdoe = assign20960_e26475;
        locals.var_cgdoe_dn0 = assign20960_e26475_d_n0;
        locals.var_cgdoe_dn2 = assign20960_e26475_d_n2;
        locals.var_cgdoe_dn4 = assign20960_e26475_d_n4;
        locals.var_cgdoe_dn5 = assign20960_e26475_d_n5;
        locals.var_cgdoe_dn6 = assign20960_e26475_d_n6;
        locals.var_cgdoe_dn8 = assign20960_e26475_d_n8;
        locals.var_cgdoe_dn10 = assign20960_e26475_d_n10;
        locals.var_cgdoe_dn11 = assign20960_e26475_d_n11;
        locals.var_cgdoe_dn12 = assign20960_e26475_d_n12;
        locals.var_cgdoe_rv = 0.0;

        let (assign20970_e26485, assign20970_e26485_d_n0, assign20970_e26485_d_n2, assign20970_e26485_d_n4, assign20970_e26485_d_n5, assign20970_e26485_d_n6, assign20970_e26485_d_n8, assign20970_e26485_d_n10, assign20970_e26485_d_n11, assign20970_e26485_d_n12,) = {
    if (locals.var_guard327 == 0.0) {
        let assign20970_e26479: f64 = (-locals.var_cgdoe);
        let assign20970_e26482: f64 = (locals.var_vgs - locals.var_vds);
        let assign20970_e26483: f64 = (assign20970_e26479 * assign20970_e26482);
        (assign20970_e26483, (((-locals.var_cgdoe_dn0) * assign20970_e26482) + (assign20970_e26479 * (-locals.var_vds_dn0))), (((-locals.var_cgdoe_dn2) * assign20970_e26482) + (assign20970_e26479 * (-locals.var_vds_dn2))), (((-locals.var_cgdoe_dn4) * assign20970_e26482) + (assign20970_e26479 * (-locals.var_vds_dn4))), (((-locals.var_cgdoe_dn5) * assign20970_e26482) + (assign20970_e26479 * (locals.var_vgs_dn5 - locals.var_vds_dn5))), (((-locals.var_cgdoe_dn6) * assign20970_e26482) + (assign20970_e26479 * (-locals.var_vds_dn6))), (((-locals.var_cgdoe_dn8) * assign20970_e26482) + (assign20970_e26479 * (-locals.var_vds_dn8))), (((-locals.var_cgdoe_dn10) * assign20970_e26482) + (assign20970_e26479 * (-locals.var_vds_dn10))), (((-locals.var_cgdoe_dn11) * assign20970_e26482) + (assign20970_e26479 * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (((-locals.var_cgdoe_dn12) * assign20970_e26482) + (assign20970_e26479 * (locals.var_vgs_dn12 - locals.var_vds_dn12))),)
    } else {
        (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn4, locals.var_qgod_dn5, locals.var_qgod_dn6, locals.var_qgod_dn8, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12,)
    }
};
        locals.var_qgod = assign20970_e26485;
        locals.var_qgod_dn0 = assign20970_e26485_d_n0;
        locals.var_qgod_dn2 = assign20970_e26485_d_n2;
        locals.var_qgod_dn4 = assign20970_e26485_d_n4;
        locals.var_qgod_dn5 = assign20970_e26485_d_n5;
        locals.var_qgod_dn6 = assign20970_e26485_d_n6;
        locals.var_qgod_dn8 = assign20970_e26485_d_n8;
        locals.var_qgod_dn10 = assign20970_e26485_d_n10;
        locals.var_qgod_dn11 = assign20970_e26485_d_n11;
        locals.var_qgod_dn12 = assign20970_e26485_d_n12;
        locals.var_qgod_rv = 0.0;

        let assign20980_e26498: f64 = if (((locals.var_mode == 1.0) && (locals.var_cgso_given == 0.0)) || ((locals.var_mode != 1.0) && (locals.var_cgdo_given == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard364 = assign20980_e26498;
        locals.var_guard364_rv = 0.0;

        let (assign20990_e26510, assign20990_e26510_d_n0, assign20990_e26510_d_n2, assign20990_e26510_d_n4, assign20990_e26510_d_n5, assign20990_e26510_d_n6, assign20990_e26510_d_n8, assign20990_e26510_d_n10, assign20990_e26510_d_n11, assign20990_e26510_d_n12,) = {
    if ((locals.var_guard327 == 0.0) && (locals.var_guard364 != 0.0)) {
        let assign20990_e26504: f64 = (-locals.var_cox0);
        let assign20990_e26506: f64 = (assign20990_e26504 * p.p175);
        let assign20990_e26508: f64 = (assign20990_e26506 * locals.var_weffcv_nf);
        (assign20990_e26508, (assign20990_e26506 * locals.var_weffcv_nf_dn0), (assign20990_e26506 * locals.var_weffcv_nf_dn2), (assign20990_e26506 * locals.var_weffcv_nf_dn4), (assign20990_e26506 * locals.var_weffcv_nf_dn5), (assign20990_e26506 * locals.var_weffcv_nf_dn6), (assign20990_e26506 * locals.var_weffcv_nf_dn8), (assign20990_e26506 * locals.var_weffcv_nf_dn10), (assign20990_e26506 * locals.var_weffcv_nf_dn11), (assign20990_e26506 * locals.var_weffcv_nf_dn12),)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn4, locals.var_cgsoe_dn5, locals.var_cgsoe_dn6, locals.var_cgsoe_dn8, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12,)
    }
};
        locals.var_cgsoe = assign20990_e26510;
        locals.var_cgsoe_dn0 = assign20990_e26510_d_n0;
        locals.var_cgsoe_dn2 = assign20990_e26510_d_n2;
        locals.var_cgsoe_dn4 = assign20990_e26510_d_n4;
        locals.var_cgsoe_dn5 = assign20990_e26510_d_n5;
        locals.var_cgsoe_dn6 = assign20990_e26510_d_n6;
        locals.var_cgsoe_dn8 = assign20990_e26510_d_n8;
        locals.var_cgsoe_dn10 = assign20990_e26510_d_n10;
        locals.var_cgsoe_dn11 = assign20990_e26510_d_n11;
        locals.var_cgsoe_dn12 = assign20990_e26510_d_n12;
        locals.var_cgsoe_rv = 0.0;

        let (assign21000_e26524, assign21000_e26524_d_n0, assign21000_e26524_d_n2, assign21000_e26524_d_n4, assign21000_e26524_d_n5, assign21000_e26524_d_n6, assign21000_e26524_d_n8, assign21000_e26524_d_n10, assign21000_e26524_d_n11, assign21000_e26524_d_n12,) = {
    if ((locals.var_guard327 == 0.0) && (locals.var_guard364 == 0.0)) {
        let assign21000_e26518: f64 = (locals.var_modenml * p.p174);
        let assign21000_e26521: f64 = (locals.var_modervs * p.p173);
        let assign21000_e26522: f64 = (assign21000_e26518 + assign21000_e26521);
        (assign21000_e26522, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn4, locals.var_cgsoe_dn5, locals.var_cgsoe_dn6, locals.var_cgsoe_dn8, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12,)
    }
};
        locals.var_cgsoe = assign21000_e26524;
        locals.var_cgsoe_dn0 = assign21000_e26524_d_n0;
        locals.var_cgsoe_dn2 = assign21000_e26524_d_n2;
        locals.var_cgsoe_dn4 = assign21000_e26524_d_n4;
        locals.var_cgsoe_dn5 = assign21000_e26524_d_n5;
        locals.var_cgsoe_dn6 = assign21000_e26524_d_n6;
        locals.var_cgsoe_dn8 = assign21000_e26524_d_n8;
        locals.var_cgsoe_dn10 = assign21000_e26524_d_n10;
        locals.var_cgsoe_dn11 = assign21000_e26524_d_n11;
        locals.var_cgsoe_dn12 = assign21000_e26524_d_n12;
        locals.var_cgsoe_rv = 0.0;

        let (assign21010_e26535, assign21010_e26535_d_n0, assign21010_e26535_d_n2, assign21010_e26535_d_n4, assign21010_e26535_d_n5, assign21010_e26535_d_n6, assign21010_e26535_d_n8, assign21010_e26535_d_n10, assign21010_e26535_d_n11, assign21010_e26535_d_n12,) = {
    if ((locals.var_guard327 == 0.0) && (locals.var_guard364 == 0.0)) {
        let assign21010_e26532: f64 = (-locals.var_weffcv_nf);
        let assign21010_e26533: f64 = (locals.var_cgsoe * assign21010_e26532);
        (assign21010_e26533, ((locals.var_cgsoe_dn0 * assign21010_e26532) + (locals.var_cgsoe * (-locals.var_weffcv_nf_dn0))), ((locals.var_cgsoe_dn2 * assign21010_e26532) + (locals.var_cgsoe * (-locals.var_weffcv_nf_dn2))), ((locals.var_cgsoe_dn4 * assign21010_e26532) + (locals.var_cgsoe * (-locals.var_weffcv_nf_dn4))), ((locals.var_cgsoe_dn5 * assign21010_e26532) + (locals.var_cgsoe * (-locals.var_weffcv_nf_dn5))), ((locals.var_cgsoe_dn6 * assign21010_e26532) + (locals.var_cgsoe * (-locals.var_weffcv_nf_dn6))), ((locals.var_cgsoe_dn8 * assign21010_e26532) + (locals.var_cgsoe * (-locals.var_weffcv_nf_dn8))), ((locals.var_cgsoe_dn10 * assign21010_e26532) + (locals.var_cgsoe * (-locals.var_weffcv_nf_dn10))), ((locals.var_cgsoe_dn11 * assign21010_e26532) + (locals.var_cgsoe * (-locals.var_weffcv_nf_dn11))), ((locals.var_cgsoe_dn12 * assign21010_e26532) + (locals.var_cgsoe * (-locals.var_weffcv_nf_dn12))),)
    } else {
        (locals.var_cgsoe, locals.var_cgsoe_dn0, locals.var_cgsoe_dn2, locals.var_cgsoe_dn4, locals.var_cgsoe_dn5, locals.var_cgsoe_dn6, locals.var_cgsoe_dn8, locals.var_cgsoe_dn10, locals.var_cgsoe_dn11, locals.var_cgsoe_dn12,)
    }
};
        locals.var_cgsoe = assign21010_e26535;
        locals.var_cgsoe_dn0 = assign21010_e26535_d_n0;
        locals.var_cgsoe_dn2 = assign21010_e26535_d_n2;
        locals.var_cgsoe_dn4 = assign21010_e26535_d_n4;
        locals.var_cgsoe_dn5 = assign21010_e26535_d_n5;
        locals.var_cgsoe_dn6 = assign21010_e26535_d_n6;
        locals.var_cgsoe_dn8 = assign21010_e26535_d_n8;
        locals.var_cgsoe_dn10 = assign21010_e26535_d_n10;
        locals.var_cgsoe_dn11 = assign21010_e26535_d_n11;
        locals.var_cgsoe_dn12 = assign21010_e26535_d_n12;
        locals.var_cgsoe_rv = 0.0;

        let (assign21020_e26543, assign21020_e26543_d_n0, assign21020_e26543_d_n2, assign21020_e26543_d_n4, assign21020_e26543_d_n5, assign21020_e26543_d_n6, assign21020_e26543_d_n8, assign21020_e26543_d_n10, assign21020_e26543_d_n11, assign21020_e26543_d_n12,) = {
    if (locals.var_guard327 == 0.0) {
        let assign21020_e26539: f64 = (-locals.var_cgsoe);
        let assign21020_e26541: f64 = (assign21020_e26539 * locals.var_vgs);
        (assign21020_e26541, ((-locals.var_cgsoe_dn0) * locals.var_vgs), ((-locals.var_cgsoe_dn2) * locals.var_vgs), ((-locals.var_cgsoe_dn4) * locals.var_vgs), (((-locals.var_cgsoe_dn5) * locals.var_vgs) + (assign21020_e26539 * locals.var_vgs_dn5)), ((-locals.var_cgsoe_dn6) * locals.var_vgs), ((-locals.var_cgsoe_dn8) * locals.var_vgs), ((-locals.var_cgsoe_dn10) * locals.var_vgs), (((-locals.var_cgsoe_dn11) * locals.var_vgs) + (assign21020_e26539 * locals.var_vgs_dn11)), (((-locals.var_cgsoe_dn12) * locals.var_vgs) + (assign21020_e26539 * locals.var_vgs_dn12)),)
    } else {
        (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn4, locals.var_qgos_dn5, locals.var_qgos_dn6, locals.var_qgos_dn8, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12,)
    }
};
        locals.var_qgos = assign21020_e26543;
        locals.var_qgos_dn0 = assign21020_e26543_d_n0;
        locals.var_qgos_dn2 = assign21020_e26543_d_n2;
        locals.var_qgos_dn4 = assign21020_e26543_d_n4;
        locals.var_qgos_dn5 = assign21020_e26543_d_n5;
        locals.var_qgos_dn6 = assign21020_e26543_d_n6;
        locals.var_qgos_dn8 = assign21020_e26543_d_n8;
        locals.var_qgos_dn10 = assign21020_e26543_d_n10;
        locals.var_qgos_dn11 = assign21020_e26543_d_n11;
        locals.var_qgos_dn12 = assign21020_e26543_d_n12;
        locals.var_qgos_rv = 0.0;

        let assign21030_e26546: f64 = if locals.var_flg_noqi == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard365 = assign21030_e26546;
        locals.var_guard365_rv = 0.0;

        let (assign21040_e26558, assign21040_e26558_d_n0, assign21040_e26558_d_n2, assign21040_e26558_d_n4, assign21040_e26558_d_n5, assign21040_e26558_d_n6, assign21040_e26558_d_n8, assign21040_e26558_d_n10, assign21040_e26558_d_n11, assign21040_e26558_d_n12,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard365 != 0.0)) {
        let assign21040_e26552: f64 = (p.p223 * p.p224);
        let assign21040_e26554: f64 = (assign21040_e26552 * locals.var_lch);
        let assign21040_e26556: f64 = (assign21040_e26554 * locals.var_lch);
        (assign21040_e26556, (((assign21040_e26552 * locals.var_lch_dn0) * locals.var_lch) + (assign21040_e26554 * locals.var_lch_dn0)), (((assign21040_e26552 * locals.var_lch_dn2) * locals.var_lch) + (assign21040_e26554 * locals.var_lch_dn2)), (((assign21040_e26552 * locals.var_lch_dn4) * locals.var_lch) + (assign21040_e26554 * locals.var_lch_dn4)), (((assign21040_e26552 * locals.var_lch_dn5) * locals.var_lch) + (assign21040_e26554 * locals.var_lch_dn5)), (((assign21040_e26552 * locals.var_lch_dn6) * locals.var_lch) + (assign21040_e26554 * locals.var_lch_dn6)), (((assign21040_e26552 * locals.var_lch_dn8) * locals.var_lch) + (assign21040_e26554 * locals.var_lch_dn8)), (((assign21040_e26552 * locals.var_lch_dn10) * locals.var_lch) + (assign21040_e26554 * locals.var_lch_dn10)), (((assign21040_e26552 * locals.var_lch_dn11) * locals.var_lch) + (assign21040_e26554 * locals.var_lch_dn11)), (((assign21040_e26552 * locals.var_lch_dn12) * locals.var_lch) + (assign21040_e26554 * locals.var_lch_dn12)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign21040_e26558;
        locals.var_t1_dn0 = assign21040_e26558_d_n0;
        locals.var_t1_dn2 = assign21040_e26558_d_n2;
        locals.var_t1_dn4 = assign21040_e26558_d_n4;
        locals.var_t1_dn5 = assign21040_e26558_d_n5;
        locals.var_t1_dn6 = assign21040_e26558_d_n6;
        locals.var_t1_dn8 = assign21040_e26558_d_n8;
        locals.var_t1_dn10 = assign21040_e26558_d_n10;
        locals.var_t1_dn11 = assign21040_e26558_d_n11;
        locals.var_t1_dn12 = assign21040_e26558_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign21050_e26576, assign21050_e26576_d_n0, assign21050_e26576_d_n2, assign21050_e26576_d_n4, assign21050_e26576_d_n5, assign21050_e26576_d_n6, assign21050_e26576_d_n8, assign21050_e26576_d_n10, assign21050_e26576_d_n11, assign21050_e26576_d_n12,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard365 != 0.0)) {
        let assign21050_e26564: f64 = (locals.var_mu * locals.var_vgvt);
        let assign21050_e26566: f64 = (assign21050_e26564 * p.p223);
        let assign21050_e26569: f64 = (p.p224 * locals.var_lch);
        let assign21050_e26571: f64 = (assign21050_e26569 * locals.var_lch);
        let assign21050_e26572: f64 = (assign21050_e26566 + assign21050_e26571);
        let assign21050_e26574: f64 = (assign21050_e26572 + 1e-50);
        (assign21050_e26574, ((((locals.var_mu_dn0 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn0)) * p.p223) + (((p.p224 * locals.var_lch_dn0) * locals.var_lch) + (assign21050_e26569 * locals.var_lch_dn0))), ((((locals.var_mu_dn2 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn2)) * p.p223) + (((p.p224 * locals.var_lch_dn2) * locals.var_lch) + (assign21050_e26569 * locals.var_lch_dn2))), ((((locals.var_mu_dn4 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn4)) * p.p223) + (((p.p224 * locals.var_lch_dn4) * locals.var_lch) + (assign21050_e26569 * locals.var_lch_dn4))), ((((locals.var_mu_dn5 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn5)) * p.p223) + (((p.p224 * locals.var_lch_dn5) * locals.var_lch) + (assign21050_e26569 * locals.var_lch_dn5))), ((((locals.var_mu_dn6 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn6)) * p.p223) + (((p.p224 * locals.var_lch_dn6) * locals.var_lch) + (assign21050_e26569 * locals.var_lch_dn6))), ((((locals.var_mu_dn8 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn8)) * p.p223) + (((p.p224 * locals.var_lch_dn8) * locals.var_lch) + (assign21050_e26569 * locals.var_lch_dn8))), ((((locals.var_mu_dn10 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn10)) * p.p223) + (((p.p224 * locals.var_lch_dn10) * locals.var_lch) + (assign21050_e26569 * locals.var_lch_dn10))), ((((locals.var_mu_dn11 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn11)) * p.p223) + (((p.p224 * locals.var_lch_dn11) * locals.var_lch) + (assign21050_e26569 * locals.var_lch_dn11))), ((((locals.var_mu_dn12 * locals.var_vgvt) + (locals.var_mu * locals.var_vgvt_dn12)) * p.p223) + (((p.p224 * locals.var_lch_dn12) * locals.var_lch) + (assign21050_e26569 * locals.var_lch_dn12))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign21050_e26576;
        locals.var_t2_dn0 = assign21050_e26576_d_n0;
        locals.var_t2_dn2 = assign21050_e26576_d_n2;
        locals.var_t2_dn4 = assign21050_e26576_d_n4;
        locals.var_t2_dn5 = assign21050_e26576_d_n5;
        locals.var_t2_dn6 = assign21050_e26576_d_n6;
        locals.var_t2_dn8 = assign21050_e26576_d_n8;
        locals.var_t2_dn10 = assign21050_e26576_d_n10;
        locals.var_t2_dn11 = assign21050_e26576_d_n11;
        locals.var_t2_dn12 = assign21050_e26576_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign21060_e26584, assign21060_e26584_d_n0, assign21060_e26584_d_n2, assign21060_e26584_d_n4, assign21060_e26584_d_n5, assign21060_e26584_d_n6, assign21060_e26584_d_n8, assign21060_e26584_d_n10, assign21060_e26584_d_n11, assign21060_e26584_d_n12,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard365 != 0.0)) {
        let assign21060_e26582: f64 = (locals.var_t1 / locals.var_t2);
        (assign21060_e26582, (((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn11 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn12 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn12)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_tau, locals.var_tau_dn0, locals.var_tau_dn2, locals.var_tau_dn4, locals.var_tau_dn5, locals.var_tau_dn6, locals.var_tau_dn8, locals.var_tau_dn10, locals.var_tau_dn11, locals.var_tau_dn12,)
    }
};
        locals.var_tau = assign21060_e26584;
        locals.var_tau_dn0 = assign21060_e26584_d_n0;
        locals.var_tau_dn2 = assign21060_e26584_d_n2;
        locals.var_tau_dn4 = assign21060_e26584_d_n4;
        locals.var_tau_dn5 = assign21060_e26584_d_n5;
        locals.var_tau_dn6 = assign21060_e26584_d_n6;
        locals.var_tau_dn8 = assign21060_e26584_d_n8;
        locals.var_tau_dn10 = assign21060_e26584_d_n10;
        locals.var_tau_dn11 = assign21060_e26584_d_n11;
        locals.var_tau_dn12 = assign21060_e26584_d_n12;
        locals.var_tau_rv = 0.0;

        let (assign21070_e26593, assign21070_e26593_d_n0, assign21070_e26593_d_n2, assign21070_e26593_d_n4, assign21070_e26593_d_n5, assign21070_e26593_d_n6, assign21070_e26593_d_n8, assign21070_e26593_d_n10, assign21070_e26593_d_n11, assign21070_e26593_d_n12,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard365 == 0.0)) {
        let assign21070_e26591: f64 = (p.p223 + 1e-50);
        (assign21070_e26591, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tau, locals.var_tau_dn0, locals.var_tau_dn2, locals.var_tau_dn4, locals.var_tau_dn5, locals.var_tau_dn6, locals.var_tau_dn8, locals.var_tau_dn10, locals.var_tau_dn11, locals.var_tau_dn12,)
    }
};
        locals.var_tau = assign21070_e26593;
        locals.var_tau_dn0 = assign21070_e26593_d_n0;
        locals.var_tau_dn2 = assign21070_e26593_d_n2;
        locals.var_tau_dn4 = assign21070_e26593_d_n4;
        locals.var_tau_dn5 = assign21070_e26593_d_n5;
        locals.var_tau_dn6 = assign21070_e26593_d_n6;
        locals.var_tau_dn8 = assign21070_e26593_d_n8;
        locals.var_tau_dn10 = assign21070_e26593_d_n10;
        locals.var_tau_dn11 = assign21070_e26593_d_n11;
        locals.var_tau_dn12 = assign21070_e26593_d_n12;
        locals.var_tau_rv = 0.0;

        let (assign21080_e26601, assign21080_e26601_d_n0, assign21080_e26601_d_n2, assign21080_e26601_d_n4, assign21080_e26601_d_n5, assign21080_e26601_d_n6, assign21080_e26601_d_n8, assign21080_e26601_d_n10, assign21080_e26601_d_n11, assign21080_e26601_d_n12,) = {
    if (locals.var_flg_nqs != 0.0) {
        let assign21080_e26597: f64 = (p.p225 * locals.var_c_fox);
        let assign21080_e26599: f64 = (assign21080_e26597 / 10000.0);
        (assign21080_e26599, ((p.p225 * locals.var_c_fox_dn0) / 10000.0), ((p.p225 * locals.var_c_fox_dn2) / 10000.0), ((p.p225 * locals.var_c_fox_dn4) / 10000.0), ((p.p225 * locals.var_c_fox_dn5) / 10000.0), ((p.p225 * locals.var_c_fox_dn6) / 10000.0), ((p.p225 * locals.var_c_fox_dn8) / 10000.0), ((p.p225 * locals.var_c_fox_dn10) / 10000.0), ((p.p225 * locals.var_c_fox_dn11) / 10000.0), ((p.p225 * locals.var_c_fox_dn12) / 10000.0),)
    } else {
        (locals.var_taub, locals.var_taub_dn0, locals.var_taub_dn2, locals.var_taub_dn4, locals.var_taub_dn5, locals.var_taub_dn6, locals.var_taub_dn8, locals.var_taub_dn10, locals.var_taub_dn11, locals.var_taub_dn12,)
    }
};
        locals.var_taub = assign21080_e26601;
        locals.var_taub_dn0 = assign21080_e26601_d_n0;
        locals.var_taub_dn2 = assign21080_e26601_d_n2;
        locals.var_taub_dn4 = assign21080_e26601_d_n4;
        locals.var_taub_dn5 = assign21080_e26601_d_n5;
        locals.var_taub_dn6 = assign21080_e26601_d_n6;
        locals.var_taub_dn8 = assign21080_e26601_d_n8;
        locals.var_taub_dn10 = assign21080_e26601_d_n10;
        locals.var_taub_dn11 = assign21080_e26601_d_n11;
        locals.var_taub_dn12 = assign21080_e26601_d_n12;
        locals.var_taub_rv = 0.0;

        let assign21090_e26607: f64 = if ((p.p21 != 0.0) && (locals.var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard366 = assign21090_e26607;
        locals.var_guard366_rv = 0.0;

        let (assign21100_e26611,) = {
    if (locals.var_guard366 != 0.0) {
        (locals.var_mks_nfalp,)
    } else {
        (locals.var_nfalpe,)
    }
};
        locals.var_nfalpe = assign21100_e26611;
        locals.var_nfalpe_rv = 0.0;

        let (assign21120_e26619,) = {
    if (locals.var_guard366 != 0.0) {
        (locals.var_mks_cit,)
    } else {
        (locals.var_cite,)
    }
};
        locals.var_cite = assign21120_e26619;
        locals.var_cite_rv = 0.0;

        let (assign21130_e26625, assign21130_e26625_d_n0, assign21130_e26625_d_n2, assign21130_e26625_d_n4, assign21130_e26625_d_n5, assign21130_e26625_d_n6, assign21130_e26625_d_n8, assign21130_e26625_d_n10, assign21130_e26625_d_n11, assign21130_e26625_d_n12,) = {
    if (locals.var_guard366 != 0.0) {
        let assign21130_e26623: f64 = (locals.var_qn0 / 1.6021918e-19);
        (assign21130_e26623, (locals.var_qn0_dn0 / 1.6021918e-19), (locals.var_qn0_dn2 / 1.6021918e-19), (locals.var_qn0_dn4 / 1.6021918e-19), (locals.var_qn0_dn5 / 1.6021918e-19), (locals.var_qn0_dn6 / 1.6021918e-19), (locals.var_qn0_dn8 / 1.6021918e-19), (locals.var_qn0_dn10 / 1.6021918e-19), (locals.var_qn0_dn11 / 1.6021918e-19), (locals.var_qn0_dn12 / 1.6021918e-19),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign21130_e26625;
        locals.var_t1_dn0 = assign21130_e26625_d_n0;
        locals.var_t1_dn2 = assign21130_e26625_d_n2;
        locals.var_t1_dn4 = assign21130_e26625_d_n4;
        locals.var_t1_dn5 = assign21130_e26625_d_n5;
        locals.var_t1_dn6 = assign21130_e26625_d_n6;
        locals.var_t1_dn8 = assign21130_e26625_d_n8;
        locals.var_t1_dn10 = assign21130_e26625_d_n10;
        locals.var_t1_dn11 = assign21130_e26625_d_n11;
        locals.var_t1_dn12 = assign21130_e26625_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign21140_e26641, assign21140_e26641_d_n0, assign21140_e26641_d_n2, assign21140_e26641_d_n4, assign21140_e26641_d_n5, assign21140_e26641_d_n6, assign21140_e26641_d_n8, assign21140_e26641_d_n10, assign21140_e26641_d_n11, assign21140_e26641_d_n12,) = {
    if (locals.var_guard366 != 0.0) {
        let assign21140_e26631: f64 = (locals.var_ps0 - locals.var_vbs);
        let assign21140_e26632: f64 = (locals.var_qn0 / assign21140_e26631);
        let assign21140_e26633: f64 = (locals.var_c_fox + assign21140_e26632);
        let assign21140_e26635: f64 = (assign21140_e26633 + locals.var_cite);
        let assign21140_e26637: f64 = (assign21140_e26635 * locals.var_beta_inv);
        let assign21140_e26639: f64 = (assign21140_e26637 / 1.6021918e-19);
        (assign21140_e26639, (((locals.var_c_fox_dn0 + (((locals.var_qn0_dn0 * assign21140_e26631) - (locals.var_qn0 * (locals.var_ps0_dn0 - locals.var_vbs_dn0))) / (assign21140_e26631 * assign21140_e26631))) * locals.var_beta_inv) / 1.6021918e-19), (((locals.var_c_fox_dn2 + (((locals.var_qn0_dn2 * assign21140_e26631) - (locals.var_qn0 * (locals.var_ps0_dn2 - locals.var_vbs_dn2))) / (assign21140_e26631 * assign21140_e26631))) * locals.var_beta_inv) / 1.6021918e-19), ((((locals.var_c_fox_dn4 + (((locals.var_qn0_dn4 * assign21140_e26631) - (locals.var_qn0 * (locals.var_ps0_dn4 - locals.var_vbs_dn4))) / (assign21140_e26631 * assign21140_e26631))) * locals.var_beta_inv) + (assign21140_e26635 * locals.var_beta_inv_dn4)) / 1.6021918e-19), (((locals.var_c_fox_dn5 + (((locals.var_qn0_dn5 * assign21140_e26631) - (locals.var_qn0 * (locals.var_ps0_dn5 - locals.var_vbs_dn5))) / (assign21140_e26631 * assign21140_e26631))) * locals.var_beta_inv) / 1.6021918e-19), (((locals.var_c_fox_dn6 + (((locals.var_qn0_dn6 * assign21140_e26631) - (locals.var_qn0 * (locals.var_ps0_dn6 - locals.var_vbs_dn6))) / (assign21140_e26631 * assign21140_e26631))) * locals.var_beta_inv) / 1.6021918e-19), (((locals.var_c_fox_dn8 + (((locals.var_qn0_dn8 * assign21140_e26631) - (locals.var_qn0 * (locals.var_ps0_dn8 - locals.var_vbs_dn8))) / (assign21140_e26631 * assign21140_e26631))) * locals.var_beta_inv) / 1.6021918e-19), (((locals.var_c_fox_dn10 + (((locals.var_qn0_dn10 * assign21140_e26631) - (locals.var_qn0 * (locals.var_ps0_dn10 - locals.var_vbs_dn10))) / (assign21140_e26631 * assign21140_e26631))) * locals.var_beta_inv) / 1.6021918e-19), (((locals.var_c_fox_dn11 + (((locals.var_qn0_dn11 * assign21140_e26631) - (locals.var_qn0 * (locals.var_ps0_dn11 - locals.var_vbs_dn11))) / (assign21140_e26631 * assign21140_e26631))) * locals.var_beta_inv) / 1.6021918e-19), (((locals.var_c_fox_dn12 + (((locals.var_qn0_dn12 * assign21140_e26631) - (locals.var_qn0 * (locals.var_ps0_dn12 - locals.var_vbs_dn12))) / (assign21140_e26631 * assign21140_e26631))) * locals.var_beta_inv) / 1.6021918e-19),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign21140_e26641;
        locals.var_t2_dn0 = assign21140_e26641_d_n0;
        locals.var_t2_dn2 = assign21140_e26641_d_n2;
        locals.var_t2_dn4 = assign21140_e26641_d_n4;
        locals.var_t2_dn5 = assign21140_e26641_d_n5;
        locals.var_t2_dn6 = assign21140_e26641_d_n6;
        locals.var_t2_dn8 = assign21140_e26641_d_n8;
        locals.var_t2_dn10 = assign21140_e26641_d_n10;
        locals.var_t2_dn11 = assign21140_e26641_d_n11;
        locals.var_t2_dn12 = assign21140_e26641_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign21150_e26656, assign21150_e26656_d_n0, assign21150_e26656_d_n2, assign21150_e26656_d_n4, assign21150_e26656_d_n5, assign21150_e26656_d_n6, assign21150_e26656_d_n8, assign21150_e26656_d_n10, assign21150_e26656_d_n11, assign21150_e26656_d_n12,) = {
    if (locals.var_guard366 != 0.0) {
        let assign21150_e26644: f64 = (-2.0);
        let assign21150_e26646: f64 = (assign21150_e26644 * locals.var_qi);
        let assign21150_e26648: f64 = (assign21150_e26646 / 1.6021918e-19);
        let assign21150_e26650: f64 = (assign21150_e26648 / locals.var_lch);
        let assign21150_e26652: f64 = (assign21150_e26650 / locals.var_weffcv_nf);
        let assign21150_e26654: f64 = (assign21150_e26652 - locals.var_t1);
        (assign21150_e26654, (((((((((assign21150_e26644 * locals.var_qi_dn0) / 1.6021918e-19) * locals.var_lch) - (assign21150_e26648 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)) * locals.var_weffcv_nf) - (assign21150_e26650 * locals.var_weffcv_nf_dn0)) / (locals.var_weffcv_nf * locals.var_weffcv_nf)) - locals.var_t1_dn0), (((((((((assign21150_e26644 * locals.var_qi_dn2) / 1.6021918e-19) * locals.var_lch) - (assign21150_e26648 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)) * locals.var_weffcv_nf) - (assign21150_e26650 * locals.var_weffcv_nf_dn2)) / (locals.var_weffcv_nf * locals.var_weffcv_nf)) - locals.var_t1_dn2), (((((((((assign21150_e26644 * locals.var_qi_dn4) / 1.6021918e-19) * locals.var_lch) - (assign21150_e26648 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)) * locals.var_weffcv_nf) - (assign21150_e26650 * locals.var_weffcv_nf_dn4)) / (locals.var_weffcv_nf * locals.var_weffcv_nf)) - locals.var_t1_dn4), (((((((((assign21150_e26644 * locals.var_qi_dn5) / 1.6021918e-19) * locals.var_lch) - (assign21150_e26648 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)) * locals.var_weffcv_nf) - (assign21150_e26650 * locals.var_weffcv_nf_dn5)) / (locals.var_weffcv_nf * locals.var_weffcv_nf)) - locals.var_t1_dn5), (((((((((assign21150_e26644 * locals.var_qi_dn6) / 1.6021918e-19) * locals.var_lch) - (assign21150_e26648 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)) * locals.var_weffcv_nf) - (assign21150_e26650 * locals.var_weffcv_nf_dn6)) / (locals.var_weffcv_nf * locals.var_weffcv_nf)) - locals.var_t1_dn6), (((((((((assign21150_e26644 * locals.var_qi_dn8) / 1.6021918e-19) * locals.var_lch) - (assign21150_e26648 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)) * locals.var_weffcv_nf) - (assign21150_e26650 * locals.var_weffcv_nf_dn8)) / (locals.var_weffcv_nf * locals.var_weffcv_nf)) - locals.var_t1_dn8), (((((((((assign21150_e26644 * locals.var_qi_dn10) / 1.6021918e-19) * locals.var_lch) - (assign21150_e26648 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)) * locals.var_weffcv_nf) - (assign21150_e26650 * locals.var_weffcv_nf_dn10)) / (locals.var_weffcv_nf * locals.var_weffcv_nf)) - locals.var_t1_dn10), (((((((((assign21150_e26644 * locals.var_qi_dn11) / 1.6021918e-19) * locals.var_lch) - (assign21150_e26648 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)) * locals.var_weffcv_nf) - (assign21150_e26650 * locals.var_weffcv_nf_dn11)) / (locals.var_weffcv_nf * locals.var_weffcv_nf)) - locals.var_t1_dn11), (((((((((assign21150_e26644 * locals.var_qi_dn12) / 1.6021918e-19) * locals.var_lch) - (assign21150_e26648 * locals.var_lch_dn12)) / (locals.var_lch * locals.var_lch)) * locals.var_weffcv_nf) - (assign21150_e26650 * locals.var_weffcv_nf_dn12)) / (locals.var_weffcv_nf * locals.var_weffcv_nf)) - locals.var_t1_dn12),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign21150_e26656;
        locals.var_t3_dn0 = assign21150_e26656_d_n0;
        locals.var_t3_dn2 = assign21150_e26656_d_n2;
        locals.var_t3_dn4 = assign21150_e26656_d_n4;
        locals.var_t3_dn5 = assign21150_e26656_d_n5;
        locals.var_t3_dn6 = assign21150_e26656_d_n6;
        locals.var_t3_dn8 = assign21150_e26656_d_n8;
        locals.var_t3_dn10 = assign21150_e26656_d_n10;
        locals.var_t3_dn11 = assign21150_e26656_d_n11;
        locals.var_t3_dn12 = assign21150_e26656_d_n12;
        locals.var_t3_rv = 0.0;

        let assign21160_e26659: f64 = (locals.var_t3 - locals.var_t1);
        let assign21160_e26660: f64 = (assign21160_e26659).abs();
        let assign21160_e26663: f64 = (10.0 * 2.220446049250313e-16);
        let assign21160_e26664: f64 = if assign21160_e26660 > assign21160_e26663 { 1.0 } else { 0.0 };
        locals.var_guard367 = assign21160_e26664;
        locals.var_guard367_rv = 0.0;

        let (assign21170_e26711, assign21170_e26711_d_n0, assign21170_e26711_d_n2, assign21170_e26711_d_n4, assign21170_e26711_d_n5, assign21170_e26711_d_n6, assign21170_e26711_d_n8, assign21170_e26711_d_n10, assign21170_e26711_d_n11, assign21170_e26711_d_n12,) = {
    if ((locals.var_guard366 != 0.0) && (locals.var_guard367 != 0.0)) {
        let assign21170_e26671: f64 = (locals.var_t1 + locals.var_t2);
        let assign21170_e26672: f64 = (1.0 / assign21170_e26671);
        let assign21170_e26675: f64 = (locals.var_t3 + locals.var_t2);
        let assign21170_e26676: f64 = (assign21170_e26672 / assign21170_e26675);
        let assign21170_e26679: f64 = (2.0 * locals.var_nfalpe);
        let assign21170_e26681: f64 = (assign21170_e26679 * locals.var_ey);
        let assign21170_e26683: f64 = (assign21170_e26681 * locals.var_mu);
        let assign21170_e26686: f64 = (locals.var_t3 - locals.var_t1);
        let assign21170_e26687: f64 = (assign21170_e26683 / assign21170_e26686);
        let assign21170_e26690: f64 = (locals.var_t3 + locals.var_t2);
        let assign21170_e26693: f64 = (locals.var_t1 + locals.var_t2);
        let assign21170_e26694: f64 = (assign21170_e26690 / assign21170_e26693);
        let assign21170_e26695: f64 = (assign21170_e26694).ln();
        let assign21170_e26696: f64 = (assign21170_e26687 * assign21170_e26695);
        let assign21170_e26697: f64 = (assign21170_e26676 + assign21170_e26696);
        let assign21170_e26700: f64 = (locals.var_nfalpe * locals.var_ey);
        let assign21170_e26702: f64 = (assign21170_e26700 * locals.var_mu);
        let assign21170_e26704: f64 = (assign21170_e26702 * locals.var_nfalpe);
        let assign21170_e26706: f64 = (assign21170_e26704 * locals.var_ey);
        let assign21170_e26708: f64 = (assign21170_e26706 * locals.var_mu);
        let assign21170_e26709: f64 = (assign21170_e26697 + assign21170_e26708);
        (assign21170_e26709, ((((((-((locals.var_t1_dn0 + locals.var_t2_dn0) / (assign21170_e26671 * assign21170_e26671))) * assign21170_e26675) - (assign21170_e26672 * (locals.var_t3_dn0 + locals.var_t2_dn0))) / (assign21170_e26675 * assign21170_e26675)) + ((((((((assign21170_e26679 * locals.var_ey_dn0) * locals.var_mu) + (assign21170_e26681 * locals.var_mu_dn0)) * assign21170_e26686) - (assign21170_e26683 * (locals.var_t3_dn0 - locals.var_t1_dn0))) / (assign21170_e26686 * assign21170_e26686)) * assign21170_e26695) + (assign21170_e26687 * (((((locals.var_t3_dn0 + locals.var_t2_dn0) * assign21170_e26693) - (assign21170_e26690 * (locals.var_t1_dn0 + locals.var_t2_dn0))) / (assign21170_e26693 * assign21170_e26693)) / assign21170_e26694)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn0) * locals.var_mu) + (assign21170_e26700 * locals.var_mu_dn0)) * locals.var_nfalpe) * locals.var_ey) + (assign21170_e26704 * locals.var_ey_dn0)) * locals.var_mu) + (assign21170_e26706 * locals.var_mu_dn0))), ((((((-((locals.var_t1_dn2 + locals.var_t2_dn2) / (assign21170_e26671 * assign21170_e26671))) * assign21170_e26675) - (assign21170_e26672 * (locals.var_t3_dn2 + locals.var_t2_dn2))) / (assign21170_e26675 * assign21170_e26675)) + ((((((((assign21170_e26679 * locals.var_ey_dn2) * locals.var_mu) + (assign21170_e26681 * locals.var_mu_dn2)) * assign21170_e26686) - (assign21170_e26683 * (locals.var_t3_dn2 - locals.var_t1_dn2))) / (assign21170_e26686 * assign21170_e26686)) * assign21170_e26695) + (assign21170_e26687 * (((((locals.var_t3_dn2 + locals.var_t2_dn2) * assign21170_e26693) - (assign21170_e26690 * (locals.var_t1_dn2 + locals.var_t2_dn2))) / (assign21170_e26693 * assign21170_e26693)) / assign21170_e26694)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn2) * locals.var_mu) + (assign21170_e26700 * locals.var_mu_dn2)) * locals.var_nfalpe) * locals.var_ey) + (assign21170_e26704 * locals.var_ey_dn2)) * locals.var_mu) + (assign21170_e26706 * locals.var_mu_dn2))), ((((((-((locals.var_t1_dn4 + locals.var_t2_dn4) / (assign21170_e26671 * assign21170_e26671))) * assign21170_e26675) - (assign21170_e26672 * (locals.var_t3_dn4 + locals.var_t2_dn4))) / (assign21170_e26675 * assign21170_e26675)) + ((((((((assign21170_e26679 * locals.var_ey_dn4) * locals.var_mu) + (assign21170_e26681 * locals.var_mu_dn4)) * assign21170_e26686) - (assign21170_e26683 * (locals.var_t3_dn4 - locals.var_t1_dn4))) / (assign21170_e26686 * assign21170_e26686)) * assign21170_e26695) + (assign21170_e26687 * (((((locals.var_t3_dn4 + locals.var_t2_dn4) * assign21170_e26693) - (assign21170_e26690 * (locals.var_t1_dn4 + locals.var_t2_dn4))) / (assign21170_e26693 * assign21170_e26693)) / assign21170_e26694)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn4) * locals.var_mu) + (assign21170_e26700 * locals.var_mu_dn4)) * locals.var_nfalpe) * locals.var_ey) + (assign21170_e26704 * locals.var_ey_dn4)) * locals.var_mu) + (assign21170_e26706 * locals.var_mu_dn4))), ((((((-((locals.var_t1_dn5 + locals.var_t2_dn5) / (assign21170_e26671 * assign21170_e26671))) * assign21170_e26675) - (assign21170_e26672 * (locals.var_t3_dn5 + locals.var_t2_dn5))) / (assign21170_e26675 * assign21170_e26675)) + ((((((((assign21170_e26679 * locals.var_ey_dn5) * locals.var_mu) + (assign21170_e26681 * locals.var_mu_dn5)) * assign21170_e26686) - (assign21170_e26683 * (locals.var_t3_dn5 - locals.var_t1_dn5))) / (assign21170_e26686 * assign21170_e26686)) * assign21170_e26695) + (assign21170_e26687 * (((((locals.var_t3_dn5 + locals.var_t2_dn5) * assign21170_e26693) - (assign21170_e26690 * (locals.var_t1_dn5 + locals.var_t2_dn5))) / (assign21170_e26693 * assign21170_e26693)) / assign21170_e26694)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn5) * locals.var_mu) + (assign21170_e26700 * locals.var_mu_dn5)) * locals.var_nfalpe) * locals.var_ey) + (assign21170_e26704 * locals.var_ey_dn5)) * locals.var_mu) + (assign21170_e26706 * locals.var_mu_dn5))), ((((((-((locals.var_t1_dn6 + locals.var_t2_dn6) / (assign21170_e26671 * assign21170_e26671))) * assign21170_e26675) - (assign21170_e26672 * (locals.var_t3_dn6 + locals.var_t2_dn6))) / (assign21170_e26675 * assign21170_e26675)) + ((((((((assign21170_e26679 * locals.var_ey_dn6) * locals.var_mu) + (assign21170_e26681 * locals.var_mu_dn6)) * assign21170_e26686) - (assign21170_e26683 * (locals.var_t3_dn6 - locals.var_t1_dn6))) / (assign21170_e26686 * assign21170_e26686)) * assign21170_e26695) + (assign21170_e26687 * (((((locals.var_t3_dn6 + locals.var_t2_dn6) * assign21170_e26693) - (assign21170_e26690 * (locals.var_t1_dn6 + locals.var_t2_dn6))) / (assign21170_e26693 * assign21170_e26693)) / assign21170_e26694)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn6) * locals.var_mu) + (assign21170_e26700 * locals.var_mu_dn6)) * locals.var_nfalpe) * locals.var_ey) + (assign21170_e26704 * locals.var_ey_dn6)) * locals.var_mu) + (assign21170_e26706 * locals.var_mu_dn6))), ((((((-((locals.var_t1_dn8 + locals.var_t2_dn8) / (assign21170_e26671 * assign21170_e26671))) * assign21170_e26675) - (assign21170_e26672 * (locals.var_t3_dn8 + locals.var_t2_dn8))) / (assign21170_e26675 * assign21170_e26675)) + ((((((((assign21170_e26679 * locals.var_ey_dn8) * locals.var_mu) + (assign21170_e26681 * locals.var_mu_dn8)) * assign21170_e26686) - (assign21170_e26683 * (locals.var_t3_dn8 - locals.var_t1_dn8))) / (assign21170_e26686 * assign21170_e26686)) * assign21170_e26695) + (assign21170_e26687 * (((((locals.var_t3_dn8 + locals.var_t2_dn8) * assign21170_e26693) - (assign21170_e26690 * (locals.var_t1_dn8 + locals.var_t2_dn8))) / (assign21170_e26693 * assign21170_e26693)) / assign21170_e26694)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn8) * locals.var_mu) + (assign21170_e26700 * locals.var_mu_dn8)) * locals.var_nfalpe) * locals.var_ey) + (assign21170_e26704 * locals.var_ey_dn8)) * locals.var_mu) + (assign21170_e26706 * locals.var_mu_dn8))), ((((((-((locals.var_t1_dn10 + locals.var_t2_dn10) / (assign21170_e26671 * assign21170_e26671))) * assign21170_e26675) - (assign21170_e26672 * (locals.var_t3_dn10 + locals.var_t2_dn10))) / (assign21170_e26675 * assign21170_e26675)) + ((((((((assign21170_e26679 * locals.var_ey_dn10) * locals.var_mu) + (assign21170_e26681 * locals.var_mu_dn10)) * assign21170_e26686) - (assign21170_e26683 * (locals.var_t3_dn10 - locals.var_t1_dn10))) / (assign21170_e26686 * assign21170_e26686)) * assign21170_e26695) + (assign21170_e26687 * (((((locals.var_t3_dn10 + locals.var_t2_dn10) * assign21170_e26693) - (assign21170_e26690 * (locals.var_t1_dn10 + locals.var_t2_dn10))) / (assign21170_e26693 * assign21170_e26693)) / assign21170_e26694)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn10) * locals.var_mu) + (assign21170_e26700 * locals.var_mu_dn10)) * locals.var_nfalpe) * locals.var_ey) + (assign21170_e26704 * locals.var_ey_dn10)) * locals.var_mu) + (assign21170_e26706 * locals.var_mu_dn10))), ((((((-((locals.var_t1_dn11 + locals.var_t2_dn11) / (assign21170_e26671 * assign21170_e26671))) * assign21170_e26675) - (assign21170_e26672 * (locals.var_t3_dn11 + locals.var_t2_dn11))) / (assign21170_e26675 * assign21170_e26675)) + ((((((((assign21170_e26679 * locals.var_ey_dn11) * locals.var_mu) + (assign21170_e26681 * locals.var_mu_dn11)) * assign21170_e26686) - (assign21170_e26683 * (locals.var_t3_dn11 - locals.var_t1_dn11))) / (assign21170_e26686 * assign21170_e26686)) * assign21170_e26695) + (assign21170_e26687 * (((((locals.var_t3_dn11 + locals.var_t2_dn11) * assign21170_e26693) - (assign21170_e26690 * (locals.var_t1_dn11 + locals.var_t2_dn11))) / (assign21170_e26693 * assign21170_e26693)) / assign21170_e26694)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn11) * locals.var_mu) + (assign21170_e26700 * locals.var_mu_dn11)) * locals.var_nfalpe) * locals.var_ey) + (assign21170_e26704 * locals.var_ey_dn11)) * locals.var_mu) + (assign21170_e26706 * locals.var_mu_dn11))), ((((((-((locals.var_t1_dn12 + locals.var_t2_dn12) / (assign21170_e26671 * assign21170_e26671))) * assign21170_e26675) - (assign21170_e26672 * (locals.var_t3_dn12 + locals.var_t2_dn12))) / (assign21170_e26675 * assign21170_e26675)) + ((((((((assign21170_e26679 * locals.var_ey_dn12) * locals.var_mu) + (assign21170_e26681 * locals.var_mu_dn12)) * assign21170_e26686) - (assign21170_e26683 * (locals.var_t3_dn12 - locals.var_t1_dn12))) / (assign21170_e26686 * assign21170_e26686)) * assign21170_e26695) + (assign21170_e26687 * (((((locals.var_t3_dn12 + locals.var_t2_dn12) * assign21170_e26693) - (assign21170_e26690 * (locals.var_t1_dn12 + locals.var_t2_dn12))) / (assign21170_e26693 * assign21170_e26693)) / assign21170_e26694)))) + ((((((((locals.var_nfalpe * locals.var_ey_dn12) * locals.var_mu) + (assign21170_e26700 * locals.var_mu_dn12)) * locals.var_nfalpe) * locals.var_ey) + (assign21170_e26704 * locals.var_ey_dn12)) * locals.var_mu) + (assign21170_e26706 * locals.var_mu_dn12))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn8, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign21170_e26711;
        locals.var_t4_dn0 = assign21170_e26711_d_n0;
        locals.var_t4_dn2 = assign21170_e26711_d_n2;
        locals.var_t4_dn4 = assign21170_e26711_d_n4;
        locals.var_t4_dn5 = assign21170_e26711_d_n5;
        locals.var_t4_dn6 = assign21170_e26711_d_n6;
        locals.var_t4_dn8 = assign21170_e26711_d_n8;
        locals.var_t4_dn10 = assign21170_e26711_d_n10;
        locals.var_t4_dn11 = assign21170_e26711_d_n11;
        locals.var_t4_dn12 = assign21170_e26711_d_n12;
        locals.var_t4_rv = 0.0;

        let (assign21180_e26750, assign21180_e26750_d_n0, assign21180_e26750_d_n2, assign21180_e26750_d_n4, assign21180_e26750_d_n5, assign21180_e26750_d_n6, assign21180_e26750_d_n8, assign21180_e26750_d_n10, assign21180_e26750_d_n11, assign21180_e26750_d_n12,) = {
    if ((locals.var_guard366 != 0.0) && (locals.var_guard367 == 0.0)) {
        let assign21180_e26719: f64 = (locals.var_t1 + locals.var_t2);
        let assign21180_e26720: f64 = (1.0 / assign21180_e26719);
        let assign21180_e26723: f64 = (locals.var_t3 + locals.var_t2);
        let assign21180_e26724: f64 = (assign21180_e26720 / assign21180_e26723);
        let assign21180_e26727: f64 = (2.0 * locals.var_nfalpe);
        let assign21180_e26729: f64 = (assign21180_e26727 * locals.var_ey);
        let assign21180_e26731: f64 = (assign21180_e26729 * locals.var_mu);
        let assign21180_e26734: f64 = (locals.var_t1 + locals.var_t2);
        let assign21180_e26735: f64 = (assign21180_e26731 / assign21180_e26734);
        let assign21180_e26736: f64 = (assign21180_e26724 + assign21180_e26735);
        let assign21180_e26739: f64 = (locals.var_nfalpe * locals.var_ey);
        let assign21180_e26741: f64 = (assign21180_e26739 * locals.var_mu);
        let assign21180_e26743: f64 = (assign21180_e26741 * locals.var_nfalpe);
        let assign21180_e26745: f64 = (assign21180_e26743 * locals.var_ey);
        let assign21180_e26747: f64 = (assign21180_e26745 * locals.var_mu);
        let assign21180_e26748: f64 = (assign21180_e26736 + assign21180_e26747);
        (assign21180_e26748, ((((((-((locals.var_t1_dn0 + locals.var_t2_dn0) / (assign21180_e26719 * assign21180_e26719))) * assign21180_e26723) - (assign21180_e26720 * (locals.var_t3_dn0 + locals.var_t2_dn0))) / (assign21180_e26723 * assign21180_e26723)) + ((((((assign21180_e26727 * locals.var_ey_dn0) * locals.var_mu) + (assign21180_e26729 * locals.var_mu_dn0)) * assign21180_e26734) - (assign21180_e26731 * (locals.var_t1_dn0 + locals.var_t2_dn0))) / (assign21180_e26734 * assign21180_e26734))) + ((((((((locals.var_nfalpe * locals.var_ey_dn0) * locals.var_mu) + (assign21180_e26739 * locals.var_mu_dn0)) * locals.var_nfalpe) * locals.var_ey) + (assign21180_e26743 * locals.var_ey_dn0)) * locals.var_mu) + (assign21180_e26745 * locals.var_mu_dn0))), ((((((-((locals.var_t1_dn2 + locals.var_t2_dn2) / (assign21180_e26719 * assign21180_e26719))) * assign21180_e26723) - (assign21180_e26720 * (locals.var_t3_dn2 + locals.var_t2_dn2))) / (assign21180_e26723 * assign21180_e26723)) + ((((((assign21180_e26727 * locals.var_ey_dn2) * locals.var_mu) + (assign21180_e26729 * locals.var_mu_dn2)) * assign21180_e26734) - (assign21180_e26731 * (locals.var_t1_dn2 + locals.var_t2_dn2))) / (assign21180_e26734 * assign21180_e26734))) + ((((((((locals.var_nfalpe * locals.var_ey_dn2) * locals.var_mu) + (assign21180_e26739 * locals.var_mu_dn2)) * locals.var_nfalpe) * locals.var_ey) + (assign21180_e26743 * locals.var_ey_dn2)) * locals.var_mu) + (assign21180_e26745 * locals.var_mu_dn2))), ((((((-((locals.var_t1_dn4 + locals.var_t2_dn4) / (assign21180_e26719 * assign21180_e26719))) * assign21180_e26723) - (assign21180_e26720 * (locals.var_t3_dn4 + locals.var_t2_dn4))) / (assign21180_e26723 * assign21180_e26723)) + ((((((assign21180_e26727 * locals.var_ey_dn4) * locals.var_mu) + (assign21180_e26729 * locals.var_mu_dn4)) * assign21180_e26734) - (assign21180_e26731 * (locals.var_t1_dn4 + locals.var_t2_dn4))) / (assign21180_e26734 * assign21180_e26734))) + ((((((((locals.var_nfalpe * locals.var_ey_dn4) * locals.var_mu) + (assign21180_e26739 * locals.var_mu_dn4)) * locals.var_nfalpe) * locals.var_ey) + (assign21180_e26743 * locals.var_ey_dn4)) * locals.var_mu) + (assign21180_e26745 * locals.var_mu_dn4))), ((((((-((locals.var_t1_dn5 + locals.var_t2_dn5) / (assign21180_e26719 * assign21180_e26719))) * assign21180_e26723) - (assign21180_e26720 * (locals.var_t3_dn5 + locals.var_t2_dn5))) / (assign21180_e26723 * assign21180_e26723)) + ((((((assign21180_e26727 * locals.var_ey_dn5) * locals.var_mu) + (assign21180_e26729 * locals.var_mu_dn5)) * assign21180_e26734) - (assign21180_e26731 * (locals.var_t1_dn5 + locals.var_t2_dn5))) / (assign21180_e26734 * assign21180_e26734))) + ((((((((locals.var_nfalpe * locals.var_ey_dn5) * locals.var_mu) + (assign21180_e26739 * locals.var_mu_dn5)) * locals.var_nfalpe) * locals.var_ey) + (assign21180_e26743 * locals.var_ey_dn5)) * locals.var_mu) + (assign21180_e26745 * locals.var_mu_dn5))), ((((((-((locals.var_t1_dn6 + locals.var_t2_dn6) / (assign21180_e26719 * assign21180_e26719))) * assign21180_e26723) - (assign21180_e26720 * (locals.var_t3_dn6 + locals.var_t2_dn6))) / (assign21180_e26723 * assign21180_e26723)) + ((((((assign21180_e26727 * locals.var_ey_dn6) * locals.var_mu) + (assign21180_e26729 * locals.var_mu_dn6)) * assign21180_e26734) - (assign21180_e26731 * (locals.var_t1_dn6 + locals.var_t2_dn6))) / (assign21180_e26734 * assign21180_e26734))) + ((((((((locals.var_nfalpe * locals.var_ey_dn6) * locals.var_mu) + (assign21180_e26739 * locals.var_mu_dn6)) * locals.var_nfalpe) * locals.var_ey) + (assign21180_e26743 * locals.var_ey_dn6)) * locals.var_mu) + (assign21180_e26745 * locals.var_mu_dn6))), ((((((-((locals.var_t1_dn8 + locals.var_t2_dn8) / (assign21180_e26719 * assign21180_e26719))) * assign21180_e26723) - (assign21180_e26720 * (locals.var_t3_dn8 + locals.var_t2_dn8))) / (assign21180_e26723 * assign21180_e26723)) + ((((((assign21180_e26727 * locals.var_ey_dn8) * locals.var_mu) + (assign21180_e26729 * locals.var_mu_dn8)) * assign21180_e26734) - (assign21180_e26731 * (locals.var_t1_dn8 + locals.var_t2_dn8))) / (assign21180_e26734 * assign21180_e26734))) + ((((((((locals.var_nfalpe * locals.var_ey_dn8) * locals.var_mu) + (assign21180_e26739 * locals.var_mu_dn8)) * locals.var_nfalpe) * locals.var_ey) + (assign21180_e26743 * locals.var_ey_dn8)) * locals.var_mu) + (assign21180_e26745 * locals.var_mu_dn8))), ((((((-((locals.var_t1_dn10 + locals.var_t2_dn10) / (assign21180_e26719 * assign21180_e26719))) * assign21180_e26723) - (assign21180_e26720 * (locals.var_t3_dn10 + locals.var_t2_dn10))) / (assign21180_e26723 * assign21180_e26723)) + ((((((assign21180_e26727 * locals.var_ey_dn10) * locals.var_mu) + (assign21180_e26729 * locals.var_mu_dn10)) * assign21180_e26734) - (assign21180_e26731 * (locals.var_t1_dn10 + locals.var_t2_dn10))) / (assign21180_e26734 * assign21180_e26734))) + ((((((((locals.var_nfalpe * locals.var_ey_dn10) * locals.var_mu) + (assign21180_e26739 * locals.var_mu_dn10)) * locals.var_nfalpe) * locals.var_ey) + (assign21180_e26743 * locals.var_ey_dn10)) * locals.var_mu) + (assign21180_e26745 * locals.var_mu_dn10))), ((((((-((locals.var_t1_dn11 + locals.var_t2_dn11) / (assign21180_e26719 * assign21180_e26719))) * assign21180_e26723) - (assign21180_e26720 * (locals.var_t3_dn11 + locals.var_t2_dn11))) / (assign21180_e26723 * assign21180_e26723)) + ((((((assign21180_e26727 * locals.var_ey_dn11) * locals.var_mu) + (assign21180_e26729 * locals.var_mu_dn11)) * assign21180_e26734) - (assign21180_e26731 * (locals.var_t1_dn11 + locals.var_t2_dn11))) / (assign21180_e26734 * assign21180_e26734))) + ((((((((locals.var_nfalpe * locals.var_ey_dn11) * locals.var_mu) + (assign21180_e26739 * locals.var_mu_dn11)) * locals.var_nfalpe) * locals.var_ey) + (assign21180_e26743 * locals.var_ey_dn11)) * locals.var_mu) + (assign21180_e26745 * locals.var_mu_dn11))), ((((((-((locals.var_t1_dn12 + locals.var_t2_dn12) / (assign21180_e26719 * assign21180_e26719))) * assign21180_e26723) - (assign21180_e26720 * (locals.var_t3_dn12 + locals.var_t2_dn12))) / (assign21180_e26723 * assign21180_e26723)) + ((((((assign21180_e26727 * locals.var_ey_dn12) * locals.var_mu) + (assign21180_e26729 * locals.var_mu_dn12)) * assign21180_e26734) - (assign21180_e26731 * (locals.var_t1_dn12 + locals.var_t2_dn12))) / (assign21180_e26734 * assign21180_e26734))) + ((((((((locals.var_nfalpe * locals.var_ey_dn12) * locals.var_mu) + (assign21180_e26739 * locals.var_mu_dn12)) * locals.var_nfalpe) * locals.var_ey) + (assign21180_e26743 * locals.var_ey_dn12)) * locals.var_mu) + (assign21180_e26745 * locals.var_mu_dn12))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn8, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign21180_e26750;
        locals.var_t4_dn0 = assign21180_e26750_d_n0;
        locals.var_t4_dn2 = assign21180_e26750_d_n2;
        locals.var_t4_dn4 = assign21180_e26750_d_n4;
        locals.var_t4_dn5 = assign21180_e26750_d_n5;
        locals.var_t4_dn6 = assign21180_e26750_d_n6;
        locals.var_t4_dn8 = assign21180_e26750_d_n8;
        locals.var_t4_dn10 = assign21180_e26750_d_n10;
        locals.var_t4_dn11 = assign21180_e26750_d_n11;
        locals.var_t4_dn12 = assign21180_e26750_d_n12;
        locals.var_t4_rv = 0.0;

        let assign21210_e26777: f64 = if ((p.p23 != 0.0) && (locals.var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard368 = assign21210_e26777;
        locals.var_guard368_rv = 0.0;

        let (assign21220_e26785, assign21220_e26785_d_n0, assign21220_e26785_d_n2, assign21220_e26785_d_n4, assign21220_e26785_d_n5, assign21220_e26785_d_n6, assign21220_e26785_d_n8, assign21220_e26785_d_n10, assign21220_e26785_d_n11, assign21220_e26785_d_n12,) = {
    if (locals.var_guard368 != 0.0) {
        let assign21220_e26781: f64 = (locals.var_psdl - locals.var_ps0);
        let assign21220_e26783: f64 = (assign21220_e26781 / locals.var_lch);
        (assign21220_e26783, ((((locals.var_psdl_dn0 - locals.var_ps0_dn0) * locals.var_lch) - (assign21220_e26781 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn2 - locals.var_ps0_dn2) * locals.var_lch) - (assign21220_e26781 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn4 - locals.var_ps0_dn4) * locals.var_lch) - (assign21220_e26781 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn5 - locals.var_ps0_dn5) * locals.var_lch) - (assign21220_e26781 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn6 - locals.var_ps0_dn6) * locals.var_lch) - (assign21220_e26781 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn8 - locals.var_ps0_dn8) * locals.var_lch) - (assign21220_e26781 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn10 - locals.var_ps0_dn10) * locals.var_lch) - (assign21220_e26781 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn11 - locals.var_ps0_dn11) * locals.var_lch) - (assign21220_e26781 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)), ((((locals.var_psdl_dn12 - locals.var_ps0_dn12) * locals.var_lch) - (assign21220_e26781 * locals.var_lch_dn12)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_eyd, locals.var_eyd_dn0, locals.var_eyd_dn2, locals.var_eyd_dn4, locals.var_eyd_dn5, locals.var_eyd_dn6, locals.var_eyd_dn8, locals.var_eyd_dn10, locals.var_eyd_dn11, locals.var_eyd_dn12,)
    }
};
        locals.var_eyd = assign21220_e26785;
        locals.var_eyd_dn0 = assign21220_e26785_d_n0;
        locals.var_eyd_dn2 = assign21220_e26785_d_n2;
        locals.var_eyd_dn4 = assign21220_e26785_d_n4;
        locals.var_eyd_dn5 = assign21220_e26785_d_n5;
        locals.var_eyd_dn6 = assign21220_e26785_d_n6;
        locals.var_eyd_dn8 = assign21220_e26785_d_n8;
        locals.var_eyd_dn10 = assign21220_e26785_d_n10;
        locals.var_eyd_dn11 = assign21220_e26785_d_n11;
        locals.var_eyd_dn12 = assign21220_e26785_d_n12;
        locals.var_eyd_rv = 0.0;

        let (assign21230_e26795, assign21230_e26795_d_n0, assign21230_e26795_d_n2, assign21230_e26795_d_n4, assign21230_e26795_d_n5, assign21230_e26795_d_n6, assign21230_e26795_d_n8, assign21230_e26795_d_n10, assign21230_e26795_d_n11, assign21230_e26795_d_n12,) = {
    if (locals.var_guard368 != 0.0) {
        let assign21230_e26789: f64 = (locals.var_muun * locals.var_eyd);
        let assign21230_e26792: f64 = (10000000.0 * 0.01);
        let assign21230_e26793: f64 = (assign21230_e26789 / assign21230_e26792);
        (assign21230_e26793, (((locals.var_muun_dn0 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn0)) / assign21230_e26792), (((locals.var_muun_dn2 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn2)) / assign21230_e26792), (((locals.var_muun_dn4 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn4)) / assign21230_e26792), (((locals.var_muun_dn5 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn5)) / assign21230_e26792), (((locals.var_muun_dn6 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn6)) / assign21230_e26792), (((locals.var_muun_dn8 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn8)) / assign21230_e26792), (((locals.var_muun_dn10 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn10)) / assign21230_e26792), (((locals.var_muun_dn11 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn11)) / assign21230_e26792), (((locals.var_muun_dn12 * locals.var_eyd) + (locals.var_muun * locals.var_eyd_dn12)) / assign21230_e26792),)
    } else {
        (locals.var_t12, locals.var_t12_dn0, locals.var_t12_dn2, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn8, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn12,)
    }
};
        locals.var_t12 = assign21230_e26795;
        locals.var_t12_dn0 = assign21230_e26795_d_n0;
        locals.var_t12_dn2 = assign21230_e26795_d_n2;
        locals.var_t12_dn4 = assign21230_e26795_d_n4;
        locals.var_t12_dn5 = assign21230_e26795_d_n5;
        locals.var_t12_dn6 = assign21230_e26795_d_n6;
        locals.var_t12_dn8 = assign21230_e26795_d_n8;
        locals.var_t12_dn10 = assign21230_e26795_d_n10;
        locals.var_t12_dn11 = assign21230_e26795_d_n11;
        locals.var_t12_dn12 = assign21230_e26795_d_n12;
        locals.var_t12_rv = 0.0;

        let assign21240_e26799: f64 = (10.0 * 2.220446049250313e-16);
        let assign21240_e26800: f64 = (1.0 - assign21240_e26799);
        let assign21240_e26807: f64 = (10.0 * 2.220446049250313e-16);
        let assign21240_e26808: f64 = (1.0 + assign21240_e26807);
        let assign21240_e26810: f64 = if ((assign21240_e26800 <= p.p114) && (p.p114 <= assign21240_e26808)) { 1.0 } else { 0.0 };
        locals.var_guard369 = assign21240_e26810;
        locals.var_guard369_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_87(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21250_e26816, assign21250_e26816_d_n0, assign21250_e26816_d_n2, assign21250_e26816_d_n4, assign21250_e26816_d_n5, assign21250_e26816_d_n6, assign21250_e26816_d_n8, assign21250_e26816_d_n10, assign21250_e26816_d_n11, assign21250_e26816_d_n12,) = {
    if ((locals.var_guard368 != 0.0) && (locals.var_guard369 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn8, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12,)
    }
};
        locals.var_t7 = assign21250_e26816;
        locals.var_t7_dn0 = assign21250_e26816_d_n0;
        locals.var_t7_dn2 = assign21250_e26816_d_n2;
        locals.var_t7_dn4 = assign21250_e26816_d_n4;
        locals.var_t7_dn5 = assign21250_e26816_d_n5;
        locals.var_t7_dn6 = assign21250_e26816_d_n6;
        locals.var_t7_dn8 = assign21250_e26816_d_n8;
        locals.var_t7_dn10 = assign21250_e26816_d_n10;
        locals.var_t7_dn11 = assign21250_e26816_d_n11;
        locals.var_t7_dn12 = assign21250_e26816_d_n12;
        locals.var_t7_rv = 0.0;

        let assign21260_e26820: f64 = (10.0 * 2.220446049250313e-16);
        let assign21260_e26821: f64 = (2.0 - assign21260_e26820);
        let assign21260_e26828: f64 = (10.0 * 2.220446049250313e-16);
        let assign21260_e26829: f64 = (2.0 + assign21260_e26828);
        let assign21260_e26831: f64 = if ((assign21260_e26821 <= p.p114) && (p.p114 <= assign21260_e26829)) { 1.0 } else { 0.0 };
        locals.var_guard370 = assign21260_e26831;
        locals.var_guard370_rv = 0.0;

        let (assign21270_e26840, assign21270_e26840_d_n0, assign21270_e26840_d_n2, assign21270_e26840_d_n4, assign21270_e26840_d_n5, assign21270_e26840_d_n6, assign21270_e26840_d_n8, assign21270_e26840_d_n10, assign21270_e26840_d_n11, assign21270_e26840_d_n12,) = {
    if (((locals.var_guard368 != 0.0) && (locals.var_guard369 == 0.0)) && (locals.var_guard370 != 0.0)) {
        (locals.var_t12, locals.var_t12_dn0, locals.var_t12_dn2, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn8, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn12,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn8, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12,)
    }
};
        locals.var_t7 = assign21270_e26840;
        locals.var_t7_dn0 = assign21270_e26840_d_n0;
        locals.var_t7_dn2 = assign21270_e26840_d_n2;
        locals.var_t7_dn4 = assign21270_e26840_d_n4;
        locals.var_t7_dn5 = assign21270_e26840_d_n5;
        locals.var_t7_dn6 = assign21270_e26840_d_n6;
        locals.var_t7_dn8 = assign21270_e26840_d_n8;
        locals.var_t7_dn10 = assign21270_e26840_d_n10;
        locals.var_t7_dn11 = assign21270_e26840_d_n11;
        locals.var_t7_dn12 = assign21270_e26840_d_n12;
        locals.var_t7_rv = 0.0;

        let (assign21280_e26854, assign21280_e26854_d_n0, assign21280_e26854_d_n2, assign21280_e26854_d_n4, assign21280_e26854_d_n5, assign21280_e26854_d_n6, assign21280_e26854_d_n8, assign21280_e26854_d_n10, assign21280_e26854_d_n11, assign21280_e26854_d_n12,) = {
    if (((locals.var_guard368 != 0.0) && (locals.var_guard369 == 0.0)) && (locals.var_guard370 == 0.0)) {
        let assign21280_e26851: f64 = (p.p114 - 1.0);
        let assign21280_e26852: f64 = (locals.var_t12).powf(assign21280_e26851);
        (assign21280_e26852, if 0.0 == 0.0 && ((assign21280_e26851) as f64).is_finite() && ((assign21280_e26851) as f64).fract() == 0.0 { if assign21280_e26851 == 0.0 { 0.0 } else { (assign21280_e26851 * ((locals.var_t12).powf(assign21280_e26851 - 1.0) * locals.var_t12_dn0)) } } else { (assign21280_e26852 * (assign21280_e26851 * (locals.var_t12_dn0 / locals.var_t12))) }, if 0.0 == 0.0 && ((assign21280_e26851) as f64).is_finite() && ((assign21280_e26851) as f64).fract() == 0.0 { if assign21280_e26851 == 0.0 { 0.0 } else { (assign21280_e26851 * ((locals.var_t12).powf(assign21280_e26851 - 1.0) * locals.var_t12_dn2)) } } else { (assign21280_e26852 * (assign21280_e26851 * (locals.var_t12_dn2 / locals.var_t12))) }, if 0.0 == 0.0 && ((assign21280_e26851) as f64).is_finite() && ((assign21280_e26851) as f64).fract() == 0.0 { if assign21280_e26851 == 0.0 { 0.0 } else { (assign21280_e26851 * ((locals.var_t12).powf(assign21280_e26851 - 1.0) * locals.var_t12_dn4)) } } else { (assign21280_e26852 * (assign21280_e26851 * (locals.var_t12_dn4 / locals.var_t12))) }, if 0.0 == 0.0 && ((assign21280_e26851) as f64).is_finite() && ((assign21280_e26851) as f64).fract() == 0.0 { if assign21280_e26851 == 0.0 { 0.0 } else { (assign21280_e26851 * ((locals.var_t12).powf(assign21280_e26851 - 1.0) * locals.var_t12_dn5)) } } else { (assign21280_e26852 * (assign21280_e26851 * (locals.var_t12_dn5 / locals.var_t12))) }, if 0.0 == 0.0 && ((assign21280_e26851) as f64).is_finite() && ((assign21280_e26851) as f64).fract() == 0.0 { if assign21280_e26851 == 0.0 { 0.0 } else { (assign21280_e26851 * ((locals.var_t12).powf(assign21280_e26851 - 1.0) * locals.var_t12_dn6)) } } else { (assign21280_e26852 * (assign21280_e26851 * (locals.var_t12_dn6 / locals.var_t12))) }, if 0.0 == 0.0 && ((assign21280_e26851) as f64).is_finite() && ((assign21280_e26851) as f64).fract() == 0.0 { if assign21280_e26851 == 0.0 { 0.0 } else { (assign21280_e26851 * ((locals.var_t12).powf(assign21280_e26851 - 1.0) * locals.var_t12_dn8)) } } else { (assign21280_e26852 * (assign21280_e26851 * (locals.var_t12_dn8 / locals.var_t12))) }, if 0.0 == 0.0 && ((assign21280_e26851) as f64).is_finite() && ((assign21280_e26851) as f64).fract() == 0.0 { if assign21280_e26851 == 0.0 { 0.0 } else { (assign21280_e26851 * ((locals.var_t12).powf(assign21280_e26851 - 1.0) * locals.var_t12_dn10)) } } else { (assign21280_e26852 * (assign21280_e26851 * (locals.var_t12_dn10 / locals.var_t12))) }, if 0.0 == 0.0 && ((assign21280_e26851) as f64).is_finite() && ((assign21280_e26851) as f64).fract() == 0.0 { if assign21280_e26851 == 0.0 { 0.0 } else { (assign21280_e26851 * ((locals.var_t12).powf(assign21280_e26851 - 1.0) * locals.var_t12_dn11)) } } else { (assign21280_e26852 * (assign21280_e26851 * (locals.var_t12_dn11 / locals.var_t12))) }, if 0.0 == 0.0 && ((assign21280_e26851) as f64).is_finite() && ((assign21280_e26851) as f64).fract() == 0.0 { if assign21280_e26851 == 0.0 { 0.0 } else { (assign21280_e26851 * ((locals.var_t12).powf(assign21280_e26851 - 1.0) * locals.var_t12_dn12)) } } else { (assign21280_e26852 * (assign21280_e26851 * (locals.var_t12_dn12 / locals.var_t12))) },)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn8, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12,)
    }
};
        locals.var_t7 = assign21280_e26854;
        locals.var_t7_dn0 = assign21280_e26854_d_n0;
        locals.var_t7_dn2 = assign21280_e26854_d_n2;
        locals.var_t7_dn4 = assign21280_e26854_d_n4;
        locals.var_t7_dn5 = assign21280_e26854_d_n5;
        locals.var_t7_dn6 = assign21280_e26854_d_n6;
        locals.var_t7_dn8 = assign21280_e26854_d_n8;
        locals.var_t7_dn10 = assign21280_e26854_d_n10;
        locals.var_t7_dn11 = assign21280_e26854_d_n11;
        locals.var_t7_dn12 = assign21280_e26854_d_n12;
        locals.var_t7_rv = 0.0;

        let (assign21290_e26862, assign21290_e26862_d_n0, assign21290_e26862_d_n2, assign21290_e26862_d_n4, assign21290_e26862_d_n5, assign21290_e26862_d_n6, assign21290_e26862_d_n8, assign21290_e26862_d_n10, assign21290_e26862_d_n11, assign21290_e26862_d_n12,) = {
    if (locals.var_guard368 != 0.0) {
        let assign21290_e26859: f64 = (locals.var_t12 * locals.var_t7);
        let assign21290_e26860: f64 = (1.0 + assign21290_e26859);
        (assign21290_e26860, ((locals.var_t12_dn0 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn0)), ((locals.var_t12_dn2 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn2)), ((locals.var_t12_dn4 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn4)), ((locals.var_t12_dn5 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn5)), ((locals.var_t12_dn6 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn6)), ((locals.var_t12_dn8 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn8)), ((locals.var_t12_dn10 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn10)), ((locals.var_t12_dn11 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn11)), ((locals.var_t12_dn12 * locals.var_t7) + (locals.var_t12 * locals.var_t7_dn12)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn8, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    }
};
        locals.var_t9 = assign21290_e26862;
        locals.var_t9_dn0 = assign21290_e26862_d_n0;
        locals.var_t9_dn2 = assign21290_e26862_d_n2;
        locals.var_t9_dn4 = assign21290_e26862_d_n4;
        locals.var_t9_dn5 = assign21290_e26862_d_n5;
        locals.var_t9_dn6 = assign21290_e26862_d_n6;
        locals.var_t9_dn8 = assign21290_e26862_d_n8;
        locals.var_t9_dn10 = assign21290_e26862_d_n10;
        locals.var_t9_dn11 = assign21290_e26862_d_n11;
        locals.var_t9_dn12 = assign21290_e26862_d_n12;
        locals.var_t9_rv = 0.0;

        let (assign21300_e26873, assign21300_e26873_d_n0, assign21300_e26873_d_n2, assign21300_e26873_d_n4, assign21300_e26873_d_n5, assign21300_e26873_d_n6, assign21300_e26873_d_n8, assign21300_e26873_d_n10, assign21300_e26873_d_n11, assign21300_e26873_d_n12,) = {
    if (locals.var_guard368 != 0.0) {
        let assign21300_e26866: f64 = (-1.0);
        let assign21300_e26868: f64 = (assign21300_e26866 / p.p114);
        let assign21300_e26870: f64 = (assign21300_e26868 - 1.0);
        let assign21300_e26871: f64 = (locals.var_t9).powf(assign21300_e26870);
        (assign21300_e26871, if 0.0 == 0.0 && ((assign21300_e26870) as f64).is_finite() && ((assign21300_e26870) as f64).fract() == 0.0 { if assign21300_e26870 == 0.0 { 0.0 } else { (assign21300_e26870 * ((locals.var_t9).powf(assign21300_e26870 - 1.0) * locals.var_t9_dn0)) } } else { (assign21300_e26871 * (assign21300_e26870 * (locals.var_t9_dn0 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign21300_e26870) as f64).is_finite() && ((assign21300_e26870) as f64).fract() == 0.0 { if assign21300_e26870 == 0.0 { 0.0 } else { (assign21300_e26870 * ((locals.var_t9).powf(assign21300_e26870 - 1.0) * locals.var_t9_dn2)) } } else { (assign21300_e26871 * (assign21300_e26870 * (locals.var_t9_dn2 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign21300_e26870) as f64).is_finite() && ((assign21300_e26870) as f64).fract() == 0.0 { if assign21300_e26870 == 0.0 { 0.0 } else { (assign21300_e26870 * ((locals.var_t9).powf(assign21300_e26870 - 1.0) * locals.var_t9_dn4)) } } else { (assign21300_e26871 * (assign21300_e26870 * (locals.var_t9_dn4 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign21300_e26870) as f64).is_finite() && ((assign21300_e26870) as f64).fract() == 0.0 { if assign21300_e26870 == 0.0 { 0.0 } else { (assign21300_e26870 * ((locals.var_t9).powf(assign21300_e26870 - 1.0) * locals.var_t9_dn5)) } } else { (assign21300_e26871 * (assign21300_e26870 * (locals.var_t9_dn5 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign21300_e26870) as f64).is_finite() && ((assign21300_e26870) as f64).fract() == 0.0 { if assign21300_e26870 == 0.0 { 0.0 } else { (assign21300_e26870 * ((locals.var_t9).powf(assign21300_e26870 - 1.0) * locals.var_t9_dn6)) } } else { (assign21300_e26871 * (assign21300_e26870 * (locals.var_t9_dn6 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign21300_e26870) as f64).is_finite() && ((assign21300_e26870) as f64).fract() == 0.0 { if assign21300_e26870 == 0.0 { 0.0 } else { (assign21300_e26870 * ((locals.var_t9).powf(assign21300_e26870 - 1.0) * locals.var_t9_dn8)) } } else { (assign21300_e26871 * (assign21300_e26870 * (locals.var_t9_dn8 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign21300_e26870) as f64).is_finite() && ((assign21300_e26870) as f64).fract() == 0.0 { if assign21300_e26870 == 0.0 { 0.0 } else { (assign21300_e26870 * ((locals.var_t9).powf(assign21300_e26870 - 1.0) * locals.var_t9_dn10)) } } else { (assign21300_e26871 * (assign21300_e26870 * (locals.var_t9_dn10 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign21300_e26870) as f64).is_finite() && ((assign21300_e26870) as f64).fract() == 0.0 { if assign21300_e26870 == 0.0 { 0.0 } else { (assign21300_e26870 * ((locals.var_t9).powf(assign21300_e26870 - 1.0) * locals.var_t9_dn11)) } } else { (assign21300_e26871 * (assign21300_e26870 * (locals.var_t9_dn11 / locals.var_t9))) }, if 0.0 == 0.0 && ((assign21300_e26870) as f64).is_finite() && ((assign21300_e26870) as f64).fract() == 0.0 { if assign21300_e26870 == 0.0 { 0.0 } else { (assign21300_e26870 * ((locals.var_t9).powf(assign21300_e26870 - 1.0) * locals.var_t9_dn12)) } } else { (assign21300_e26871 * (assign21300_e26870 * (locals.var_t9_dn12 / locals.var_t9))) },)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn8, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12,)
    }
};
        locals.var_t10 = assign21300_e26873;
        locals.var_t10_dn0 = assign21300_e26873_d_n0;
        locals.var_t10_dn2 = assign21300_e26873_d_n2;
        locals.var_t10_dn4 = assign21300_e26873_d_n4;
        locals.var_t10_dn5 = assign21300_e26873_d_n5;
        locals.var_t10_dn6 = assign21300_e26873_d_n6;
        locals.var_t10_dn8 = assign21300_e26873_d_n8;
        locals.var_t10_dn10 = assign21300_e26873_d_n10;
        locals.var_t10_dn11 = assign21300_e26873_d_n11;
        locals.var_t10_dn12 = assign21300_e26873_d_n12;
        locals.var_t10_rv = 0.0;

        let (assign21310_e26881, assign21310_e26881_d_n0, assign21310_e26881_d_n2, assign21310_e26881_d_n4, assign21310_e26881_d_n5, assign21310_e26881_d_n6, assign21310_e26881_d_n8, assign21310_e26881_d_n10, assign21310_e26881_d_n11, assign21310_e26881_d_n12,) = {
    if (locals.var_guard368 != 0.0) {
        let assign21310_e26877: f64 = (locals.var_muun * locals.var_t9);
        let assign21310_e26879: f64 = (assign21310_e26877 * locals.var_t10);
        (assign21310_e26879, ((((locals.var_muun_dn0 * locals.var_t9) + (locals.var_muun * locals.var_t9_dn0)) * locals.var_t10) + (assign21310_e26877 * locals.var_t10_dn0)), ((((locals.var_muun_dn2 * locals.var_t9) + (locals.var_muun * locals.var_t9_dn2)) * locals.var_t10) + (assign21310_e26877 * locals.var_t10_dn2)), ((((locals.var_muun_dn4 * locals.var_t9) + (locals.var_muun * locals.var_t9_dn4)) * locals.var_t10) + (assign21310_e26877 * locals.var_t10_dn4)), ((((locals.var_muun_dn5 * locals.var_t9) + (locals.var_muun * locals.var_t9_dn5)) * locals.var_t10) + (assign21310_e26877 * locals.var_t10_dn5)), ((((locals.var_muun_dn6 * locals.var_t9) + (locals.var_muun * locals.var_t9_dn6)) * locals.var_t10) + (assign21310_e26877 * locals.var_t10_dn6)), ((((locals.var_muun_dn8 * locals.var_t9) + (locals.var_muun * locals.var_t9_dn8)) * locals.var_t10) + (assign21310_e26877 * locals.var_t10_dn8)), ((((locals.var_muun_dn10 * locals.var_t9) + (locals.var_muun * locals.var_t9_dn10)) * locals.var_t10) + (assign21310_e26877 * locals.var_t10_dn10)), ((((locals.var_muun_dn11 * locals.var_t9) + (locals.var_muun * locals.var_t9_dn11)) * locals.var_t10) + (assign21310_e26877 * locals.var_t10_dn11)), ((((locals.var_muun_dn12 * locals.var_t9) + (locals.var_muun * locals.var_t9_dn12)) * locals.var_t10) + (assign21310_e26877 * locals.var_t10_dn12)),)
    } else {
        (locals.var_mud_hoso, locals.var_mud_hoso_dn0, locals.var_mud_hoso_dn2, locals.var_mud_hoso_dn4, locals.var_mud_hoso_dn5, locals.var_mud_hoso_dn6, locals.var_mud_hoso_dn8, locals.var_mud_hoso_dn10, locals.var_mud_hoso_dn11, locals.var_mud_hoso_dn12,)
    }
};
        locals.var_mud_hoso = assign21310_e26881;
        locals.var_mud_hoso_dn0 = assign21310_e26881_d_n0;
        locals.var_mud_hoso_dn2 = assign21310_e26881_d_n2;
        locals.var_mud_hoso_dn4 = assign21310_e26881_d_n4;
        locals.var_mud_hoso_dn5 = assign21310_e26881_d_n5;
        locals.var_mud_hoso_dn6 = assign21310_e26881_d_n6;
        locals.var_mud_hoso_dn8 = assign21310_e26881_d_n8;
        locals.var_mud_hoso_dn10 = assign21310_e26881_d_n10;
        locals.var_mud_hoso_dn11 = assign21310_e26881_d_n11;
        locals.var_mud_hoso_dn12 = assign21310_e26881_d_n12;
        locals.var_mud_hoso_rv = 0.0;

        let (assign21320_e26889, assign21320_e26889_d_n0, assign21320_e26889_d_n2, assign21320_e26889_d_n4, assign21320_e26889_d_n5, assign21320_e26889_d_n6, assign21320_e26889_d_n8, assign21320_e26889_d_n10, assign21320_e26889_d_n11, assign21320_e26889_d_n12,) = {
    if (locals.var_guard368 != 0.0) {
        let assign21320_e26885: f64 = (locals.var_mu + locals.var_mud_hoso);
        let assign21320_e26887: f64 = (assign21320_e26885 / 2.0);
        (assign21320_e26887, ((locals.var_mu_dn0 + locals.var_mud_hoso_dn0) / 2.0), ((locals.var_mu_dn2 + locals.var_mud_hoso_dn2) / 2.0), ((locals.var_mu_dn4 + locals.var_mud_hoso_dn4) / 2.0), ((locals.var_mu_dn5 + locals.var_mud_hoso_dn5) / 2.0), ((locals.var_mu_dn6 + locals.var_mud_hoso_dn6) / 2.0), ((locals.var_mu_dn8 + locals.var_mud_hoso_dn8) / 2.0), ((locals.var_mu_dn10 + locals.var_mud_hoso_dn10) / 2.0), ((locals.var_mu_dn11 + locals.var_mud_hoso_dn11) / 2.0), ((locals.var_mu_dn12 + locals.var_mud_hoso_dn12) / 2.0),)
    } else {
        (locals.var_mu_ave, locals.var_mu_ave_dn0, locals.var_mu_ave_dn2, locals.var_mu_ave_dn4, locals.var_mu_ave_dn5, locals.var_mu_ave_dn6, locals.var_mu_ave_dn8, locals.var_mu_ave_dn10, locals.var_mu_ave_dn11, locals.var_mu_ave_dn12,)
    }
};
        locals.var_mu_ave = assign21320_e26889;
        locals.var_mu_ave_dn0 = assign21320_e26889_d_n0;
        locals.var_mu_ave_dn2 = assign21320_e26889_d_n2;
        locals.var_mu_ave_dn4 = assign21320_e26889_d_n4;
        locals.var_mu_ave_dn5 = assign21320_e26889_d_n5;
        locals.var_mu_ave_dn6 = assign21320_e26889_d_n6;
        locals.var_mu_ave_dn8 = assign21320_e26889_d_n8;
        locals.var_mu_ave_dn10 = assign21320_e26889_d_n10;
        locals.var_mu_ave_dn11 = assign21320_e26889_d_n11;
        locals.var_mu_ave_dn12 = assign21320_e26889_d_n12;
        locals.var_mu_ave_rv = 0.0;

        let (assign21330_e26895, assign21330_e26895_d_n0, assign21330_e26895_d_n2, assign21330_e26895_d_n4, assign21330_e26895_d_n5, assign21330_e26895_d_n6, assign21330_e26895_d_n8, assign21330_e26895_d_n10, assign21330_e26895_d_n11, assign21330_e26895_d_n12,) = {
    if (locals.var_guard368 != 0.0) {
        let assign21330_e26893: f64 = (locals.var_alpha * locals.var_alpha);
        (assign21330_e26893, ((locals.var_alpha_dn0 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn0)), ((locals.var_alpha_dn2 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn2)), ((locals.var_alpha_dn4 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn4)), ((locals.var_alpha_dn5 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn5)), ((locals.var_alpha_dn6 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn6)), ((locals.var_alpha_dn8 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn8)), ((locals.var_alpha_dn10 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn10)), ((locals.var_alpha_dn11 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn11)), ((locals.var_alpha_dn12 * locals.var_alpha) + (locals.var_alpha * locals.var_alpha_dn12)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign21330_e26895;
        locals.var_t0_dn0 = assign21330_e26895_d_n0;
        locals.var_t0_dn2 = assign21330_e26895_d_n2;
        locals.var_t0_dn4 = assign21330_e26895_d_n4;
        locals.var_t0_dn5 = assign21330_e26895_d_n5;
        locals.var_t0_dn6 = assign21330_e26895_d_n6;
        locals.var_t0_dn8 = assign21330_e26895_d_n8;
        locals.var_t0_dn10 = assign21330_e26895_d_n10;
        locals.var_t0_dn11 = assign21330_e26895_d_n11;
        locals.var_t0_dn12 = assign21330_e26895_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign21340_e26957, assign21340_e26957_d_n0, assign21340_e26957_d_n2, assign21340_e26957_d_n4, assign21340_e26957_d_n5, assign21340_e26957_d_n6, assign21340_e26957_d_n8, assign21340_e26957_d_n10, assign21340_e26957_d_n11, assign21340_e26957_d_n12,) = {
    if (locals.var_guard368 != 0.0) {
        let assign21340_e26899: f64 = (locals.var_weff_nf * locals.var_c_fox);
        let assign21340_e26901: f64 = (assign21340_e26899 * locals.var_vgvt);
        let assign21340_e26903: f64 = (assign21340_e26901 * locals.var_mu);
        let assign21340_e26907: f64 = (3.0 * locals.var_alpha);
        let assign21340_e26908: f64 = (1.0 + assign21340_e26907);
        let assign21340_e26911: f64 = (6.0 * locals.var_t0);
        let assign21340_e26912: f64 = (assign21340_e26908 + assign21340_e26911);
        let assign21340_e26914: f64 = (assign21340_e26912 * locals.var_mud_hoso);
        let assign21340_e26916: f64 = (assign21340_e26914 * locals.var_mud_hoso);
        let assign21340_e26920: f64 = (4.0 * locals.var_alpha);
        let assign21340_e26921: f64 = (3.0 + assign21340_e26920);
        let assign21340_e26924: f64 = (3.0 * locals.var_t0);
        let assign21340_e26925: f64 = (assign21340_e26921 + assign21340_e26924);
        let assign21340_e26927: f64 = (assign21340_e26925 * locals.var_mud_hoso);
        let assign21340_e26929: f64 = (assign21340_e26927 * locals.var_mu);
        let assign21340_e26930: f64 = (assign21340_e26916 + assign21340_e26929);
        let assign21340_e26934: f64 = (3.0 * locals.var_alpha);
        let assign21340_e26935: f64 = (6.0 + assign21340_e26934);
        let assign21340_e26937: f64 = (assign21340_e26935 + locals.var_t0);
        let assign21340_e26939: f64 = (assign21340_e26937 * locals.var_mu);
        let assign21340_e26941: f64 = (assign21340_e26939 * locals.var_mu);
        let assign21340_e26942: f64 = (assign21340_e26930 + assign21340_e26941);
        let assign21340_e26943: f64 = (assign21340_e26903 * assign21340_e26942);
        let assign21340_e26946: f64 = (15.0 * locals.var_lch);
        let assign21340_e26949: f64 = (1.0 + locals.var_alpha);
        let assign21340_e26950: f64 = (assign21340_e26946 * assign21340_e26949);
        let assign21340_e26952: f64 = (assign21340_e26950 * locals.var_mu_ave);
        let assign21340_e26954: f64 = (assign21340_e26952 * locals.var_mu_ave);
        let assign21340_e26955: f64 = (assign21340_e26943 / assign21340_e26954);
        (assign21340_e26955, (((((((((((locals.var_weff_nf_dn0 * locals.var_c_fox) + (locals.var_weff_nf * locals.var_c_fox_dn0)) * locals.var_vgvt) + (assign21340_e26899 * locals.var_vgvt_dn0)) * locals.var_mu) + (assign21340_e26901 * locals.var_mu_dn0)) * assign21340_e26942) + (assign21340_e26903 * ((((((((3.0 * locals.var_alpha_dn0) + (6.0 * locals.var_t0_dn0)) * locals.var_mud_hoso) + (assign21340_e26912 * locals.var_mud_hoso_dn0)) * locals.var_mud_hoso) + (assign21340_e26914 * locals.var_mud_hoso_dn0)) + ((((((4.0 * locals.var_alpha_dn0) + (3.0 * locals.var_t0_dn0)) * locals.var_mud_hoso) + (assign21340_e26925 * locals.var_mud_hoso_dn0)) * locals.var_mu) + (assign21340_e26927 * locals.var_mu_dn0))) + ((((((3.0 * locals.var_alpha_dn0) + locals.var_t0_dn0) * locals.var_mu) + (assign21340_e26937 * locals.var_mu_dn0)) * locals.var_mu) + (assign21340_e26939 * locals.var_mu_dn0))))) * assign21340_e26954) - (assign21340_e26943 * (((((((15.0 * locals.var_lch_dn0) * assign21340_e26949) + (assign21340_e26946 * locals.var_alpha_dn0)) * locals.var_mu_ave) + (assign21340_e26950 * locals.var_mu_ave_dn0)) * locals.var_mu_ave) + (assign21340_e26952 * locals.var_mu_ave_dn0)))) / (assign21340_e26954 * assign21340_e26954)), (((((((((((locals.var_weff_nf_dn2 * locals.var_c_fox) + (locals.var_weff_nf * locals.var_c_fox_dn2)) * locals.var_vgvt) + (assign21340_e26899 * locals.var_vgvt_dn2)) * locals.var_mu) + (assign21340_e26901 * locals.var_mu_dn2)) * assign21340_e26942) + (assign21340_e26903 * ((((((((3.0 * locals.var_alpha_dn2) + (6.0 * locals.var_t0_dn2)) * locals.var_mud_hoso) + (assign21340_e26912 * locals.var_mud_hoso_dn2)) * locals.var_mud_hoso) + (assign21340_e26914 * locals.var_mud_hoso_dn2)) + ((((((4.0 * locals.var_alpha_dn2) + (3.0 * locals.var_t0_dn2)) * locals.var_mud_hoso) + (assign21340_e26925 * locals.var_mud_hoso_dn2)) * locals.var_mu) + (assign21340_e26927 * locals.var_mu_dn2))) + ((((((3.0 * locals.var_alpha_dn2) + locals.var_t0_dn2) * locals.var_mu) + (assign21340_e26937 * locals.var_mu_dn2)) * locals.var_mu) + (assign21340_e26939 * locals.var_mu_dn2))))) * assign21340_e26954) - (assign21340_e26943 * (((((((15.0 * locals.var_lch_dn2) * assign21340_e26949) + (assign21340_e26946 * locals.var_alpha_dn2)) * locals.var_mu_ave) + (assign21340_e26950 * locals.var_mu_ave_dn2)) * locals.var_mu_ave) + (assign21340_e26952 * locals.var_mu_ave_dn2)))) / (assign21340_e26954 * assign21340_e26954)), (((((((((((locals.var_weff_nf_dn4 * locals.var_c_fox) + (locals.var_weff_nf * locals.var_c_fox_dn4)) * locals.var_vgvt) + (assign21340_e26899 * locals.var_vgvt_dn4)) * locals.var_mu) + (assign21340_e26901 * locals.var_mu_dn4)) * assign21340_e26942) + (assign21340_e26903 * ((((((((3.0 * locals.var_alpha_dn4) + (6.0 * locals.var_t0_dn4)) * locals.var_mud_hoso) + (assign21340_e26912 * locals.var_mud_hoso_dn4)) * locals.var_mud_hoso) + (assign21340_e26914 * locals.var_mud_hoso_dn4)) + ((((((4.0 * locals.var_alpha_dn4) + (3.0 * locals.var_t0_dn4)) * locals.var_mud_hoso) + (assign21340_e26925 * locals.var_mud_hoso_dn4)) * locals.var_mu) + (assign21340_e26927 * locals.var_mu_dn4))) + ((((((3.0 * locals.var_alpha_dn4) + locals.var_t0_dn4) * locals.var_mu) + (assign21340_e26937 * locals.var_mu_dn4)) * locals.var_mu) + (assign21340_e26939 * locals.var_mu_dn4))))) * assign21340_e26954) - (assign21340_e26943 * (((((((15.0 * locals.var_lch_dn4) * assign21340_e26949) + (assign21340_e26946 * locals.var_alpha_dn4)) * locals.var_mu_ave) + (assign21340_e26950 * locals.var_mu_ave_dn4)) * locals.var_mu_ave) + (assign21340_e26952 * locals.var_mu_ave_dn4)))) / (assign21340_e26954 * assign21340_e26954)), (((((((((((locals.var_weff_nf_dn5 * locals.var_c_fox) + (locals.var_weff_nf * locals.var_c_fox_dn5)) * locals.var_vgvt) + (assign21340_e26899 * locals.var_vgvt_dn5)) * locals.var_mu) + (assign21340_e26901 * locals.var_mu_dn5)) * assign21340_e26942) + (assign21340_e26903 * ((((((((3.0 * locals.var_alpha_dn5) + (6.0 * locals.var_t0_dn5)) * locals.var_mud_hoso) + (assign21340_e26912 * locals.var_mud_hoso_dn5)) * locals.var_mud_hoso) + (assign21340_e26914 * locals.var_mud_hoso_dn5)) + ((((((4.0 * locals.var_alpha_dn5) + (3.0 * locals.var_t0_dn5)) * locals.var_mud_hoso) + (assign21340_e26925 * locals.var_mud_hoso_dn5)) * locals.var_mu) + (assign21340_e26927 * locals.var_mu_dn5))) + ((((((3.0 * locals.var_alpha_dn5) + locals.var_t0_dn5) * locals.var_mu) + (assign21340_e26937 * locals.var_mu_dn5)) * locals.var_mu) + (assign21340_e26939 * locals.var_mu_dn5))))) * assign21340_e26954) - (assign21340_e26943 * (((((((15.0 * locals.var_lch_dn5) * assign21340_e26949) + (assign21340_e26946 * locals.var_alpha_dn5)) * locals.var_mu_ave) + (assign21340_e26950 * locals.var_mu_ave_dn5)) * locals.var_mu_ave) + (assign21340_e26952 * locals.var_mu_ave_dn5)))) / (assign21340_e26954 * assign21340_e26954)), (((((((((((locals.var_weff_nf_dn6 * locals.var_c_fox) + (locals.var_weff_nf * locals.var_c_fox_dn6)) * locals.var_vgvt) + (assign21340_e26899 * locals.var_vgvt_dn6)) * locals.var_mu) + (assign21340_e26901 * locals.var_mu_dn6)) * assign21340_e26942) + (assign21340_e26903 * ((((((((3.0 * locals.var_alpha_dn6) + (6.0 * locals.var_t0_dn6)) * locals.var_mud_hoso) + (assign21340_e26912 * locals.var_mud_hoso_dn6)) * locals.var_mud_hoso) + (assign21340_e26914 * locals.var_mud_hoso_dn6)) + ((((((4.0 * locals.var_alpha_dn6) + (3.0 * locals.var_t0_dn6)) * locals.var_mud_hoso) + (assign21340_e26925 * locals.var_mud_hoso_dn6)) * locals.var_mu) + (assign21340_e26927 * locals.var_mu_dn6))) + ((((((3.0 * locals.var_alpha_dn6) + locals.var_t0_dn6) * locals.var_mu) + (assign21340_e26937 * locals.var_mu_dn6)) * locals.var_mu) + (assign21340_e26939 * locals.var_mu_dn6))))) * assign21340_e26954) - (assign21340_e26943 * (((((((15.0 * locals.var_lch_dn6) * assign21340_e26949) + (assign21340_e26946 * locals.var_alpha_dn6)) * locals.var_mu_ave) + (assign21340_e26950 * locals.var_mu_ave_dn6)) * locals.var_mu_ave) + (assign21340_e26952 * locals.var_mu_ave_dn6)))) / (assign21340_e26954 * assign21340_e26954)), (((((((((((locals.var_weff_nf_dn8 * locals.var_c_fox) + (locals.var_weff_nf * locals.var_c_fox_dn8)) * locals.var_vgvt) + (assign21340_e26899 * locals.var_vgvt_dn8)) * locals.var_mu) + (assign21340_e26901 * locals.var_mu_dn8)) * assign21340_e26942) + (assign21340_e26903 * ((((((((3.0 * locals.var_alpha_dn8) + (6.0 * locals.var_t0_dn8)) * locals.var_mud_hoso) + (assign21340_e26912 * locals.var_mud_hoso_dn8)) * locals.var_mud_hoso) + (assign21340_e26914 * locals.var_mud_hoso_dn8)) + ((((((4.0 * locals.var_alpha_dn8) + (3.0 * locals.var_t0_dn8)) * locals.var_mud_hoso) + (assign21340_e26925 * locals.var_mud_hoso_dn8)) * locals.var_mu) + (assign21340_e26927 * locals.var_mu_dn8))) + ((((((3.0 * locals.var_alpha_dn8) + locals.var_t0_dn8) * locals.var_mu) + (assign21340_e26937 * locals.var_mu_dn8)) * locals.var_mu) + (assign21340_e26939 * locals.var_mu_dn8))))) * assign21340_e26954) - (assign21340_e26943 * (((((((15.0 * locals.var_lch_dn8) * assign21340_e26949) + (assign21340_e26946 * locals.var_alpha_dn8)) * locals.var_mu_ave) + (assign21340_e26950 * locals.var_mu_ave_dn8)) * locals.var_mu_ave) + (assign21340_e26952 * locals.var_mu_ave_dn8)))) / (assign21340_e26954 * assign21340_e26954)), (((((((((((locals.var_weff_nf_dn10 * locals.var_c_fox) + (locals.var_weff_nf * locals.var_c_fox_dn10)) * locals.var_vgvt) + (assign21340_e26899 * locals.var_vgvt_dn10)) * locals.var_mu) + (assign21340_e26901 * locals.var_mu_dn10)) * assign21340_e26942) + (assign21340_e26903 * ((((((((3.0 * locals.var_alpha_dn10) + (6.0 * locals.var_t0_dn10)) * locals.var_mud_hoso) + (assign21340_e26912 * locals.var_mud_hoso_dn10)) * locals.var_mud_hoso) + (assign21340_e26914 * locals.var_mud_hoso_dn10)) + ((((((4.0 * locals.var_alpha_dn10) + (3.0 * locals.var_t0_dn10)) * locals.var_mud_hoso) + (assign21340_e26925 * locals.var_mud_hoso_dn10)) * locals.var_mu) + (assign21340_e26927 * locals.var_mu_dn10))) + ((((((3.0 * locals.var_alpha_dn10) + locals.var_t0_dn10) * locals.var_mu) + (assign21340_e26937 * locals.var_mu_dn10)) * locals.var_mu) + (assign21340_e26939 * locals.var_mu_dn10))))) * assign21340_e26954) - (assign21340_e26943 * (((((((15.0 * locals.var_lch_dn10) * assign21340_e26949) + (assign21340_e26946 * locals.var_alpha_dn10)) * locals.var_mu_ave) + (assign21340_e26950 * locals.var_mu_ave_dn10)) * locals.var_mu_ave) + (assign21340_e26952 * locals.var_mu_ave_dn10)))) / (assign21340_e26954 * assign21340_e26954)), (((((((((((locals.var_weff_nf_dn11 * locals.var_c_fox) + (locals.var_weff_nf * locals.var_c_fox_dn11)) * locals.var_vgvt) + (assign21340_e26899 * locals.var_vgvt_dn11)) * locals.var_mu) + (assign21340_e26901 * locals.var_mu_dn11)) * assign21340_e26942) + (assign21340_e26903 * ((((((((3.0 * locals.var_alpha_dn11) + (6.0 * locals.var_t0_dn11)) * locals.var_mud_hoso) + (assign21340_e26912 * locals.var_mud_hoso_dn11)) * locals.var_mud_hoso) + (assign21340_e26914 * locals.var_mud_hoso_dn11)) + ((((((4.0 * locals.var_alpha_dn11) + (3.0 * locals.var_t0_dn11)) * locals.var_mud_hoso) + (assign21340_e26925 * locals.var_mud_hoso_dn11)) * locals.var_mu) + (assign21340_e26927 * locals.var_mu_dn11))) + ((((((3.0 * locals.var_alpha_dn11) + locals.var_t0_dn11) * locals.var_mu) + (assign21340_e26937 * locals.var_mu_dn11)) * locals.var_mu) + (assign21340_e26939 * locals.var_mu_dn11))))) * assign21340_e26954) - (assign21340_e26943 * (((((((15.0 * locals.var_lch_dn11) * assign21340_e26949) + (assign21340_e26946 * locals.var_alpha_dn11)) * locals.var_mu_ave) + (assign21340_e26950 * locals.var_mu_ave_dn11)) * locals.var_mu_ave) + (assign21340_e26952 * locals.var_mu_ave_dn11)))) / (assign21340_e26954 * assign21340_e26954)), (((((((((((locals.var_weff_nf_dn12 * locals.var_c_fox) + (locals.var_weff_nf * locals.var_c_fox_dn12)) * locals.var_vgvt) + (assign21340_e26899 * locals.var_vgvt_dn12)) * locals.var_mu) + (assign21340_e26901 * locals.var_mu_dn12)) * assign21340_e26942) + (assign21340_e26903 * ((((((((3.0 * locals.var_alpha_dn12) + (6.0 * locals.var_t0_dn12)) * locals.var_mud_hoso) + (assign21340_e26912 * locals.var_mud_hoso_dn12)) * locals.var_mud_hoso) + (assign21340_e26914 * locals.var_mud_hoso_dn12)) + ((((((4.0 * locals.var_alpha_dn12) + (3.0 * locals.var_t0_dn12)) * locals.var_mud_hoso) + (assign21340_e26925 * locals.var_mud_hoso_dn12)) * locals.var_mu) + (assign21340_e26927 * locals.var_mu_dn12))) + ((((((3.0 * locals.var_alpha_dn12) + locals.var_t0_dn12) * locals.var_mu) + (assign21340_e26937 * locals.var_mu_dn12)) * locals.var_mu) + (assign21340_e26939 * locals.var_mu_dn12))))) * assign21340_e26954) - (assign21340_e26943 * (((((((15.0 * locals.var_lch_dn12) * assign21340_e26949) + (assign21340_e26946 * locals.var_alpha_dn12)) * locals.var_mu_ave) + (assign21340_e26950 * locals.var_mu_ave_dn12)) * locals.var_mu_ave) + (assign21340_e26952 * locals.var_mu_ave_dn12)))) / (assign21340_e26954 * assign21340_e26954)),)
    } else {
        (locals.var_nthrml, locals.var_nthrml_dn0, locals.var_nthrml_dn2, locals.var_nthrml_dn4, locals.var_nthrml_dn5, locals.var_nthrml_dn6, locals.var_nthrml_dn8, locals.var_nthrml_dn10, locals.var_nthrml_dn11, locals.var_nthrml_dn12,)
    }
};
        locals.var_nthrml = assign21340_e26957;
        locals.var_nthrml_dn0 = assign21340_e26957_d_n0;
        locals.var_nthrml_dn2 = assign21340_e26957_d_n2;
        locals.var_nthrml_dn4 = assign21340_e26957_d_n4;
        locals.var_nthrml_dn5 = assign21340_e26957_d_n5;
        locals.var_nthrml_dn6 = assign21340_e26957_d_n6;
        locals.var_nthrml_dn8 = assign21340_e26957_d_n8;
        locals.var_nthrml_dn10 = assign21340_e26957_d_n10;
        locals.var_nthrml_dn11 = assign21340_e26957_d_n11;
        locals.var_nthrml_dn12 = assign21340_e26957_d_n12;
        locals.var_nthrml_rv = 0.0;

        let (assign21350_e26962, assign21350_e26962_d_n0, assign21350_e26962_d_n2, assign21350_e26962_d_n4, assign21350_e26962_d_n5, assign21350_e26962_d_n6, assign21350_e26962_d_n8, assign21350_e26962_d_n10, assign21350_e26962_d_n11, assign21350_e26962_d_n12,) = {
    if (locals.var_guard368 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_nthrml, locals.var_nthrml_dn0, locals.var_nthrml_dn2, locals.var_nthrml_dn4, locals.var_nthrml_dn5, locals.var_nthrml_dn6, locals.var_nthrml_dn8, locals.var_nthrml_dn10, locals.var_nthrml_dn11, locals.var_nthrml_dn12,)
    }
};
        locals.var_nthrml = assign21350_e26962;
        locals.var_nthrml_dn0 = assign21350_e26962_d_n0;
        locals.var_nthrml_dn2 = assign21350_e26962_d_n2;
        locals.var_nthrml_dn4 = assign21350_e26962_d_n4;
        locals.var_nthrml_dn5 = assign21350_e26962_d_n5;
        locals.var_nthrml_dn6 = assign21350_e26962_d_n6;
        locals.var_nthrml_dn8 = assign21350_e26962_d_n8;
        locals.var_nthrml_dn10 = assign21350_e26962_d_n10;
        locals.var_nthrml_dn11 = assign21350_e26962_d_n11;
        locals.var_nthrml_dn12 = assign21350_e26962_d_n12;
        locals.var_nthrml_rv = 0.0;

        let assign21360_e26976: f64 = if ((((p.p20 != 0.0) && (p.p23 != 0.0)) && (locals.var_flg_ign == 1.0)) && (locals.var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard371 = assign21360_e26976;
        locals.var_guard371_rv = 0.0;

        let (assign21370_e26981, assign21370_e26981_d_n0, assign21370_e26981_d_n2, assign21370_e26981_d_n4, assign21370_e26981_d_n5, assign21370_e26981_d_n6, assign21370_e26981_d_n8, assign21370_e26981_d_n10, assign21370_e26981_d_n11, assign21370_e26981_d_n12,) = {
    if (locals.var_guard371 != 0.0) {
        let assign21370_e26979: f64 = (locals.var_kusail).sqrt();
        (assign21370_e26979, (locals.var_kusail_dn0 / (2.0 * assign21370_e26979)), (locals.var_kusail_dn2 / (2.0 * assign21370_e26979)), (locals.var_kusail_dn4 / (2.0 * assign21370_e26979)), (locals.var_kusail_dn5 / (2.0 * assign21370_e26979)), (locals.var_kusail_dn6 / (2.0 * assign21370_e26979)), (locals.var_kusail_dn8 / (2.0 * assign21370_e26979)), (locals.var_kusail_dn10 / (2.0 * assign21370_e26979)), (locals.var_kusail_dn11 / (2.0 * assign21370_e26979)), (locals.var_kusail_dn12 / (2.0 * assign21370_e26979)),)
    } else {
        (locals.var_sqrtkusail, locals.var_sqrtkusail_dn0, locals.var_sqrtkusail_dn2, locals.var_sqrtkusail_dn4, locals.var_sqrtkusail_dn5, locals.var_sqrtkusail_dn6, locals.var_sqrtkusail_dn8, locals.var_sqrtkusail_dn10, locals.var_sqrtkusail_dn11, locals.var_sqrtkusail_dn12,)
    }
};
        locals.var_sqrtkusail = assign21370_e26981;
        locals.var_sqrtkusail_dn0 = assign21370_e26981_d_n0;
        locals.var_sqrtkusail_dn2 = assign21370_e26981_d_n2;
        locals.var_sqrtkusail_dn4 = assign21370_e26981_d_n4;
        locals.var_sqrtkusail_dn5 = assign21370_e26981_d_n5;
        locals.var_sqrtkusail_dn6 = assign21370_e26981_d_n6;
        locals.var_sqrtkusail_dn8 = assign21370_e26981_d_n8;
        locals.var_sqrtkusail_dn10 = assign21370_e26981_d_n10;
        locals.var_sqrtkusail_dn11 = assign21370_e26981_d_n11;
        locals.var_sqrtkusail_dn12 = assign21370_e26981_d_n12;
        locals.var_sqrtkusail_rv = 0.0;

        let (assign21380_e26987, assign21380_e26987_d_n0, assign21380_e26987_d_n2, assign21380_e26987_d_n4, assign21380_e26987_d_n5, assign21380_e26987_d_n6, assign21380_e26987_d_n8, assign21380_e26987_d_n10, assign21380_e26987_d_n11, assign21380_e26987_d_n12,) = {
    if (locals.var_guard371 != 0.0) {
        let assign21380_e26985: f64 = (locals.var_vgvt + locals.var_sqrtkusail);
        (assign21380_e26985, (locals.var_vgvt_dn0 + locals.var_sqrtkusail_dn0), (locals.var_vgvt_dn2 + locals.var_sqrtkusail_dn2), (locals.var_vgvt_dn4 + locals.var_sqrtkusail_dn4), (locals.var_vgvt_dn5 + locals.var_sqrtkusail_dn5), (locals.var_vgvt_dn6 + locals.var_sqrtkusail_dn6), (locals.var_vgvt_dn8 + locals.var_sqrtkusail_dn8), (locals.var_vgvt_dn10 + locals.var_sqrtkusail_dn10), (locals.var_vgvt_dn11 + locals.var_sqrtkusail_dn11), (locals.var_vgvt_dn12 + locals.var_sqrtkusail_dn12),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign21380_e26987;
        locals.var_t2_dn0 = assign21380_e26987_d_n0;
        locals.var_t2_dn2 = assign21380_e26987_d_n2;
        locals.var_t2_dn4 = assign21380_e26987_d_n4;
        locals.var_t2_dn5 = assign21380_e26987_d_n5;
        locals.var_t2_dn6 = assign21380_e26987_d_n6;
        locals.var_t2_dn8 = assign21380_e26987_d_n8;
        locals.var_t2_dn10 = assign21380_e26987_d_n10;
        locals.var_t2_dn11 = assign21380_e26987_d_n11;
        locals.var_t2_dn12 = assign21380_e26987_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign21390_e26993, assign21390_e26993_d_n0, assign21390_e26993_d_n2, assign21390_e26993_d_n4, assign21390_e26993_d_n5, assign21390_e26993_d_n6, assign21390_e26993_d_n8, assign21390_e26993_d_n10, assign21390_e26993_d_n11, assign21390_e26993_d_n12,) = {
    if (locals.var_guard371 != 0.0) {
        let assign21390_e26991: f64 = (locals.var_kusai00 * locals.var_kusai00);
        (assign21390_e26991, ((locals.var_kusai00_dn0 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn0)), ((locals.var_kusai00_dn2 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn2)), ((locals.var_kusai00_dn4 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn4)), ((locals.var_kusai00_dn5 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn5)), ((locals.var_kusai00_dn6 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn6)), ((locals.var_kusai00_dn8 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn8)), ((locals.var_kusai00_dn10 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn10)), ((locals.var_kusai00_dn11 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn11)), ((locals.var_kusai00_dn12 * locals.var_kusai00) + (locals.var_kusai00 * locals.var_kusai00_dn12)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign21390_e26993;
        locals.var_t3_dn0 = assign21390_e26993_d_n0;
        locals.var_t3_dn2 = assign21390_e26993_d_n2;
        locals.var_t3_dn4 = assign21390_e26993_d_n4;
        locals.var_t3_dn5 = assign21390_e26993_d_n5;
        locals.var_t3_dn6 = assign21390_e26993_d_n6;
        locals.var_t3_dn8 = assign21390_e26993_d_n8;
        locals.var_t3_dn10 = assign21390_e26993_d_n10;
        locals.var_t3_dn11 = assign21390_e26993_d_n11;
        locals.var_t3_dn12 = assign21390_e26993_d_n12;
        locals.var_t3_rv = 0.0;

        let (assign21400_e26999, assign21400_e26999_d_n0, assign21400_e26999_d_n2, assign21400_e26999_d_n4, assign21400_e26999_d_n5, assign21400_e26999_d_n6, assign21400_e26999_d_n8, assign21400_e26999_d_n10, assign21400_e26999_d_n11, assign21400_e26999_d_n12,) = {
    if (locals.var_guard371 != 0.0) {
        let assign21400_e26997: f64 = (locals.var_kusail * locals.var_kusail);
        (assign21400_e26997, ((locals.var_kusail_dn0 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn0)), ((locals.var_kusail_dn2 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn2)), ((locals.var_kusail_dn4 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn4)), ((locals.var_kusail_dn5 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn5)), ((locals.var_kusail_dn6 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn6)), ((locals.var_kusail_dn8 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn8)), ((locals.var_kusail_dn10 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn10)), ((locals.var_kusail_dn11 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn11)), ((locals.var_kusail_dn12 * locals.var_kusail) + (locals.var_kusail * locals.var_kusail_dn12)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn8, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign21400_e26999;
        locals.var_t4_dn0 = assign21400_e26999_d_n0;
        locals.var_t4_dn2 = assign21400_e26999_d_n2;
        locals.var_t4_dn4 = assign21400_e26999_d_n4;
        locals.var_t4_dn5 = assign21400_e26999_d_n5;
        locals.var_t4_dn6 = assign21400_e26999_d_n6;
        locals.var_t4_dn8 = assign21400_e26999_d_n8;
        locals.var_t4_dn10 = assign21400_e26999_d_n10;
        locals.var_t4_dn11 = assign21400_e26999_d_n11;
        locals.var_t4_dn12 = assign21400_e26999_d_n12;
        locals.var_t4_rv = 0.0;

        let (assign21410_e27007, assign21410_e27007_d_n0, assign21410_e27007_d_n2, assign21410_e27007_d_n4, assign21410_e27007_d_n5, assign21410_e27007_d_n6, assign21410_e27007_d_n8, assign21410_e27007_d_n10, assign21410_e27007_d_n11, assign21410_e27007_d_n12,) = {
    if (locals.var_guard371 != 0.0) {
        let assign21410_e27003: f64 = (42.0 * locals.var_kusai00);
        let assign21410_e27005: f64 = (assign21410_e27003 * locals.var_kusail);
        (assign21410_e27005, (((42.0 * locals.var_kusai00_dn0) * locals.var_kusail) + (assign21410_e27003 * locals.var_kusail_dn0)), (((42.0 * locals.var_kusai00_dn2) * locals.var_kusail) + (assign21410_e27003 * locals.var_kusail_dn2)), (((42.0 * locals.var_kusai00_dn4) * locals.var_kusail) + (assign21410_e27003 * locals.var_kusail_dn4)), (((42.0 * locals.var_kusai00_dn5) * locals.var_kusail) + (assign21410_e27003 * locals.var_kusail_dn5)), (((42.0 * locals.var_kusai00_dn6) * locals.var_kusail) + (assign21410_e27003 * locals.var_kusail_dn6)), (((42.0 * locals.var_kusai00_dn8) * locals.var_kusail) + (assign21410_e27003 * locals.var_kusail_dn8)), (((42.0 * locals.var_kusai00_dn10) * locals.var_kusail) + (assign21410_e27003 * locals.var_kusail_dn10)), (((42.0 * locals.var_kusai00_dn11) * locals.var_kusail) + (assign21410_e27003 * locals.var_kusail_dn11)), (((42.0 * locals.var_kusai00_dn12) * locals.var_kusail) + (assign21410_e27003 * locals.var_kusail_dn12)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
        locals.var_t5 = assign21410_e27007;
        locals.var_t5_dn0 = assign21410_e27007_d_n0;
        locals.var_t5_dn2 = assign21410_e27007_d_n2;
        locals.var_t5_dn4 = assign21410_e27007_d_n4;
        locals.var_t5_dn5 = assign21410_e27007_d_n5;
        locals.var_t5_dn6 = assign21410_e27007_d_n6;
        locals.var_t5_dn8 = assign21410_e27007_d_n8;
        locals.var_t5_dn10 = assign21410_e27007_d_n10;
        locals.var_t5_dn11 = assign21410_e27007_d_n11;
        locals.var_t5_dn12 = assign21410_e27007_d_n12;
        locals.var_t5_rv = 0.0;

        let (assign21420_e27017, assign21420_e27017_d_n0, assign21420_e27017_d_n2, assign21420_e27017_d_n4, assign21420_e27017_d_n5, assign21420_e27017_d_n6, assign21420_e27017_d_n8, assign21420_e27017_d_n10, assign21420_e27017_d_n11, assign21420_e27017_d_n12,) = {
    if (locals.var_guard371 != 0.0) {
        let assign21420_e27013: f64 = (locals.var_t3 + locals.var_t4);
        let assign21420_e27014: f64 = (4.0 * assign21420_e27013);
        let assign21420_e27015: f64 = (locals.var_t5 + assign21420_e27014);
        (assign21420_e27015, (locals.var_t5_dn0 + (4.0 * (locals.var_t3_dn0 + locals.var_t4_dn0))), (locals.var_t5_dn2 + (4.0 * (locals.var_t3_dn2 + locals.var_t4_dn2))), (locals.var_t5_dn4 + (4.0 * (locals.var_t3_dn4 + locals.var_t4_dn4))), (locals.var_t5_dn5 + (4.0 * (locals.var_t3_dn5 + locals.var_t4_dn5))), (locals.var_t5_dn6 + (4.0 * (locals.var_t3_dn6 + locals.var_t4_dn6))), (locals.var_t5_dn8 + (4.0 * (locals.var_t3_dn8 + locals.var_t4_dn8))), (locals.var_t5_dn10 + (4.0 * (locals.var_t3_dn10 + locals.var_t4_dn10))), (locals.var_t5_dn11 + (4.0 * (locals.var_t3_dn11 + locals.var_t4_dn11))), (locals.var_t5_dn12 + (4.0 * (locals.var_t3_dn12 + locals.var_t4_dn12))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
        locals.var_t5 = assign21420_e27017;
        locals.var_t5_dn0 = assign21420_e27017_d_n0;
        locals.var_t5_dn2 = assign21420_e27017_d_n2;
        locals.var_t5_dn4 = assign21420_e27017_d_n4;
        locals.var_t5_dn5 = assign21420_e27017_d_n5;
        locals.var_t5_dn6 = assign21420_e27017_d_n6;
        locals.var_t5_dn8 = assign21420_e27017_d_n8;
        locals.var_t5_dn10 = assign21420_e27017_d_n10;
        locals.var_t5_dn11 = assign21420_e27017_d_n11;
        locals.var_t5_dn12 = assign21420_e27017_d_n12;
        locals.var_t5_rv = 0.0;

        let (assign21430_e27031, assign21430_e27031_d_n0, assign21430_e27031_d_n2, assign21430_e27031_d_n4, assign21430_e27031_d_n5, assign21430_e27031_d_n6, assign21430_e27031_d_n8, assign21430_e27031_d_n10, assign21430_e27031_d_n11, assign21430_e27031_d_n12,) = {
    if (locals.var_guard371 != 0.0) {
        let assign21430_e27022: f64 = (20.0 * locals.var_sqrtkusail);
        let assign21430_e27024: f64 = (assign21430_e27022 * locals.var_vgvt);
        let assign21430_e27027: f64 = (locals.var_kusai00 + locals.var_kusail);
        let assign21430_e27028: f64 = (assign21430_e27024 * assign21430_e27027);
        let assign21430_e27029: f64 = (locals.var_t5 + assign21430_e27028);
        (assign21430_e27029, (locals.var_t5_dn0 + (((((20.0 * locals.var_sqrtkusail_dn0) * locals.var_vgvt) + (assign21430_e27022 * locals.var_vgvt_dn0)) * assign21430_e27027) + (assign21430_e27024 * (locals.var_kusai00_dn0 + locals.var_kusail_dn0)))), (locals.var_t5_dn2 + (((((20.0 * locals.var_sqrtkusail_dn2) * locals.var_vgvt) + (assign21430_e27022 * locals.var_vgvt_dn2)) * assign21430_e27027) + (assign21430_e27024 * (locals.var_kusai00_dn2 + locals.var_kusail_dn2)))), (locals.var_t5_dn4 + (((((20.0 * locals.var_sqrtkusail_dn4) * locals.var_vgvt) + (assign21430_e27022 * locals.var_vgvt_dn4)) * assign21430_e27027) + (assign21430_e27024 * (locals.var_kusai00_dn4 + locals.var_kusail_dn4)))), (locals.var_t5_dn5 + (((((20.0 * locals.var_sqrtkusail_dn5) * locals.var_vgvt) + (assign21430_e27022 * locals.var_vgvt_dn5)) * assign21430_e27027) + (assign21430_e27024 * (locals.var_kusai00_dn5 + locals.var_kusail_dn5)))), (locals.var_t5_dn6 + (((((20.0 * locals.var_sqrtkusail_dn6) * locals.var_vgvt) + (assign21430_e27022 * locals.var_vgvt_dn6)) * assign21430_e27027) + (assign21430_e27024 * (locals.var_kusai00_dn6 + locals.var_kusail_dn6)))), (locals.var_t5_dn8 + (((((20.0 * locals.var_sqrtkusail_dn8) * locals.var_vgvt) + (assign21430_e27022 * locals.var_vgvt_dn8)) * assign21430_e27027) + (assign21430_e27024 * (locals.var_kusai00_dn8 + locals.var_kusail_dn8)))), (locals.var_t5_dn10 + (((((20.0 * locals.var_sqrtkusail_dn10) * locals.var_vgvt) + (assign21430_e27022 * locals.var_vgvt_dn10)) * assign21430_e27027) + (assign21430_e27024 * (locals.var_kusai00_dn10 + locals.var_kusail_dn10)))), (locals.var_t5_dn11 + (((((20.0 * locals.var_sqrtkusail_dn11) * locals.var_vgvt) + (assign21430_e27022 * locals.var_vgvt_dn11)) * assign21430_e27027) + (assign21430_e27024 * (locals.var_kusai00_dn11 + locals.var_kusail_dn11)))), (locals.var_t5_dn12 + (((((20.0 * locals.var_sqrtkusail_dn12) * locals.var_vgvt) + (assign21430_e27022 * locals.var_vgvt_dn12)) * assign21430_e27027) + (assign21430_e27024 * (locals.var_kusai00_dn12 + locals.var_kusail_dn12)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
        locals.var_t5 = assign21430_e27031;
        locals.var_t5_dn0 = assign21430_e27031_d_n0;
        locals.var_t5_dn2 = assign21430_e27031_d_n2;
        locals.var_t5_dn4 = assign21430_e27031_d_n4;
        locals.var_t5_dn5 = assign21430_e27031_d_n5;
        locals.var_t5_dn6 = assign21430_e27031_d_n6;
        locals.var_t5_dn8 = assign21430_e27031_d_n8;
        locals.var_t5_dn10 = assign21430_e27031_d_n10;
        locals.var_t5_dn11 = assign21430_e27031_d_n11;
        locals.var_t5_dn12 = assign21430_e27031_d_n12;
        locals.var_t5_rv = 0.0;

        let (assign21440_e27037, assign21440_e27037_d_n0, assign21440_e27037_d_n2, assign21440_e27037_d_n4, assign21440_e27037_d_n5, assign21440_e27037_d_n6, assign21440_e27037_d_n8, assign21440_e27037_d_n10, assign21440_e27037_d_n11, assign21440_e27037_d_n12,) = {
    if (locals.var_guard371 != 0.0) {
        let assign21440_e27035: f64 = (locals.var_t2 * locals.var_t2);
        (assign21440_e27035, ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)), ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)), ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)), ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)), ((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn8, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12,)
    }
};
        locals.var_t10 = assign21440_e27037;
        locals.var_t10_dn0 = assign21440_e27037_d_n0;
        locals.var_t10_dn2 = assign21440_e27037_d_n2;
        locals.var_t10_dn4 = assign21440_e27037_d_n4;
        locals.var_t10_dn5 = assign21440_e27037_d_n5;
        locals.var_t10_dn6 = assign21440_e27037_d_n6;
        locals.var_t10_dn8 = assign21440_e27037_d_n8;
        locals.var_t10_dn10 = assign21440_e27037_d_n10;
        locals.var_t10_dn11 = assign21440_e27037_d_n11;
        locals.var_t10_dn12 = assign21440_e27037_d_n12;
        locals.var_t10_rv = 0.0;

        let (assign21450_e27047, assign21450_e27047_d_n0, assign21450_e27047_d_n2, assign21450_e27047_d_n4, assign21450_e27047_d_n5, assign21450_e27047_d_n6, assign21450_e27047_d_n8, assign21450_e27047_d_n10, assign21450_e27047_d_n11, assign21450_e27047_d_n12,) = {
    if (locals.var_guard371 != 0.0) {
        let assign21450_e27042: f64 = (locals.var_t10 * locals.var_t10);
        let assign21450_e27044: f64 = (assign21450_e27042 * locals.var_t2);
        let assign21450_e27045: f64 = (locals.var_t5 / assign21450_e27044);
        (assign21450_e27045, (((locals.var_t5_dn0 * assign21450_e27044) - (locals.var_t5 * ((((locals.var_t10_dn0 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn0)) * locals.var_t2) + (assign21450_e27042 * locals.var_t2_dn0)))) / (assign21450_e27044 * assign21450_e27044)), (((locals.var_t5_dn2 * assign21450_e27044) - (locals.var_t5 * ((((locals.var_t10_dn2 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn2)) * locals.var_t2) + (assign21450_e27042 * locals.var_t2_dn2)))) / (assign21450_e27044 * assign21450_e27044)), (((locals.var_t5_dn4 * assign21450_e27044) - (locals.var_t5 * ((((locals.var_t10_dn4 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn4)) * locals.var_t2) + (assign21450_e27042 * locals.var_t2_dn4)))) / (assign21450_e27044 * assign21450_e27044)), (((locals.var_t5_dn5 * assign21450_e27044) - (locals.var_t5 * ((((locals.var_t10_dn5 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn5)) * locals.var_t2) + (assign21450_e27042 * locals.var_t2_dn5)))) / (assign21450_e27044 * assign21450_e27044)), (((locals.var_t5_dn6 * assign21450_e27044) - (locals.var_t5 * ((((locals.var_t10_dn6 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn6)) * locals.var_t2) + (assign21450_e27042 * locals.var_t2_dn6)))) / (assign21450_e27044 * assign21450_e27044)), (((locals.var_t5_dn8 * assign21450_e27044) - (locals.var_t5 * ((((locals.var_t10_dn8 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn8)) * locals.var_t2) + (assign21450_e27042 * locals.var_t2_dn8)))) / (assign21450_e27044 * assign21450_e27044)), (((locals.var_t5_dn10 * assign21450_e27044) - (locals.var_t5 * ((((locals.var_t10_dn10 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn10)) * locals.var_t2) + (assign21450_e27042 * locals.var_t2_dn10)))) / (assign21450_e27044 * assign21450_e27044)), (((locals.var_t5_dn11 * assign21450_e27044) - (locals.var_t5 * ((((locals.var_t10_dn11 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn11)) * locals.var_t2) + (assign21450_e27042 * locals.var_t2_dn11)))) / (assign21450_e27044 * assign21450_e27044)), (((locals.var_t5_dn12 * assign21450_e27044) - (locals.var_t5 * ((((locals.var_t10_dn12 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn12)) * locals.var_t2) + (assign21450_e27042 * locals.var_t2_dn12)))) / (assign21450_e27044 * assign21450_e27044)),)
    } else {
        (locals.var_kusai_ig, locals.var_kusai_ig_dn0, locals.var_kusai_ig_dn2, locals.var_kusai_ig_dn4, locals.var_kusai_ig_dn5, locals.var_kusai_ig_dn6, locals.var_kusai_ig_dn8, locals.var_kusai_ig_dn10, locals.var_kusai_ig_dn11, locals.var_kusai_ig_dn12,)
    }
};
        locals.var_kusai_ig = assign21450_e27047;
        locals.var_kusai_ig_dn0 = assign21450_e27047_d_n0;
        locals.var_kusai_ig_dn2 = assign21450_e27047_d_n2;
        locals.var_kusai_ig_dn4 = assign21450_e27047_d_n4;
        locals.var_kusai_ig_dn5 = assign21450_e27047_d_n5;
        locals.var_kusai_ig_dn6 = assign21450_e27047_d_n6;
        locals.var_kusai_ig_dn8 = assign21450_e27047_d_n8;
        locals.var_kusai_ig_dn10 = assign21450_e27047_d_n10;
        locals.var_kusai_ig_dn11 = assign21450_e27047_d_n11;
        locals.var_kusai_ig_dn12 = assign21450_e27047_d_n12;
        locals.var_kusai_ig_rv = 0.0;

        let (assign21460_e27057, assign21460_e27057_d_n0, assign21460_e27057_d_n2, assign21460_e27057_d_n4, assign21460_e27057_d_n5, assign21460_e27057_d_n6, assign21460_e27057_d_n8, assign21460_e27057_d_n10, assign21460_e27057_d_n11, assign21460_e27057_d_n12,) = {
    if (locals.var_guard371 != 0.0) {
        let assign21460_e27051: f64 = (locals.var_weff_nf / locals.var_lch);
        let assign21460_e27053: f64 = (assign21460_e27051 * locals.var_mu);
        let assign21460_e27055: f64 = (assign21460_e27053 * locals.var_c_fox);
        (assign21460_e27055, (((((((locals.var_weff_nf_dn0 * locals.var_lch) - (locals.var_weff_nf * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)) * locals.var_mu) + (assign21460_e27051 * locals.var_mu_dn0)) * locals.var_c_fox) + (assign21460_e27053 * locals.var_c_fox_dn0)), (((((((locals.var_weff_nf_dn2 * locals.var_lch) - (locals.var_weff_nf * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)) * locals.var_mu) + (assign21460_e27051 * locals.var_mu_dn2)) * locals.var_c_fox) + (assign21460_e27053 * locals.var_c_fox_dn2)), (((((((locals.var_weff_nf_dn4 * locals.var_lch) - (locals.var_weff_nf * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)) * locals.var_mu) + (assign21460_e27051 * locals.var_mu_dn4)) * locals.var_c_fox) + (assign21460_e27053 * locals.var_c_fox_dn4)), (((((((locals.var_weff_nf_dn5 * locals.var_lch) - (locals.var_weff_nf * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)) * locals.var_mu) + (assign21460_e27051 * locals.var_mu_dn5)) * locals.var_c_fox) + (assign21460_e27053 * locals.var_c_fox_dn5)), (((((((locals.var_weff_nf_dn6 * locals.var_lch) - (locals.var_weff_nf * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)) * locals.var_mu) + (assign21460_e27051 * locals.var_mu_dn6)) * locals.var_c_fox) + (assign21460_e27053 * locals.var_c_fox_dn6)), (((((((locals.var_weff_nf_dn8 * locals.var_lch) - (locals.var_weff_nf * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)) * locals.var_mu) + (assign21460_e27051 * locals.var_mu_dn8)) * locals.var_c_fox) + (assign21460_e27053 * locals.var_c_fox_dn8)), (((((((locals.var_weff_nf_dn10 * locals.var_lch) - (locals.var_weff_nf * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)) * locals.var_mu) + (assign21460_e27051 * locals.var_mu_dn10)) * locals.var_c_fox) + (assign21460_e27053 * locals.var_c_fox_dn10)), (((((((locals.var_weff_nf_dn11 * locals.var_lch) - (locals.var_weff_nf * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)) * locals.var_mu) + (assign21460_e27051 * locals.var_mu_dn11)) * locals.var_c_fox) + (assign21460_e27053 * locals.var_c_fox_dn11)), (((((((locals.var_weff_nf_dn12 * locals.var_lch) - (locals.var_weff_nf * locals.var_lch_dn12)) / (locals.var_lch * locals.var_lch)) * locals.var_mu) + (assign21460_e27051 * locals.var_mu_dn12)) * locals.var_c_fox) + (assign21460_e27053 * locals.var_c_fox_dn12)),)
    } else {
        (locals.var_gds0_ign, locals.var_gds0_ign_dn0, locals.var_gds0_ign_dn2, locals.var_gds0_ign_dn4, locals.var_gds0_ign_dn5, locals.var_gds0_ign_dn6, locals.var_gds0_ign_dn8, locals.var_gds0_ign_dn10, locals.var_gds0_ign_dn11, locals.var_gds0_ign_dn12,)
    }
};
        locals.var_gds0_ign = assign21460_e27057;
        locals.var_gds0_ign_dn0 = assign21460_e27057_d_n0;
        locals.var_gds0_ign_dn2 = assign21460_e27057_d_n2;
        locals.var_gds0_ign_dn4 = assign21460_e27057_d_n4;
        locals.var_gds0_ign_dn5 = assign21460_e27057_d_n5;
        locals.var_gds0_ign_dn6 = assign21460_e27057_d_n6;
        locals.var_gds0_ign_dn8 = assign21460_e27057_d_n8;
        locals.var_gds0_ign_dn10 = assign21460_e27057_d_n10;
        locals.var_gds0_ign_dn11 = assign21460_e27057_d_n11;
        locals.var_gds0_ign_dn12 = assign21460_e27057_d_n12;
        locals.var_gds0_ign_rv = 0.0;

        let (assign21490_e27081, assign21490_e27081_d_n0, assign21490_e27081_d_n2, assign21490_e27081_d_n4, assign21490_e27081_d_n5, assign21490_e27081_d_n6, assign21490_e27081_d_n8, assign21490_e27081_d_n10, assign21490_e27081_d_n11, assign21490_e27081_d_n12,) = {
    if (locals.var_guard371 != 0.0) {
        let assign21490_e27074: f64 = (4.0 * locals.var_vgvt);
        let assign21490_e27076: f64 = (assign21490_e27074 * locals.var_sqrtkusail);
        let assign21490_e27077: f64 = (locals.var_kusai00 + assign21490_e27076);
        let assign21490_e27079: f64 = (assign21490_e27077 + locals.var_kusail);
        (assign21490_e27079, ((locals.var_kusai00_dn0 + (((4.0 * locals.var_vgvt_dn0) * locals.var_sqrtkusail) + (assign21490_e27074 * locals.var_sqrtkusail_dn0))) + locals.var_kusail_dn0), ((locals.var_kusai00_dn2 + (((4.0 * locals.var_vgvt_dn2) * locals.var_sqrtkusail) + (assign21490_e27074 * locals.var_sqrtkusail_dn2))) + locals.var_kusail_dn2), ((locals.var_kusai00_dn4 + (((4.0 * locals.var_vgvt_dn4) * locals.var_sqrtkusail) + (assign21490_e27074 * locals.var_sqrtkusail_dn4))) + locals.var_kusail_dn4), ((locals.var_kusai00_dn5 + (((4.0 * locals.var_vgvt_dn5) * locals.var_sqrtkusail) + (assign21490_e27074 * locals.var_sqrtkusail_dn5))) + locals.var_kusail_dn5), ((locals.var_kusai00_dn6 + (((4.0 * locals.var_vgvt_dn6) * locals.var_sqrtkusail) + (assign21490_e27074 * locals.var_sqrtkusail_dn6))) + locals.var_kusail_dn6), ((locals.var_kusai00_dn8 + (((4.0 * locals.var_vgvt_dn8) * locals.var_sqrtkusail) + (assign21490_e27074 * locals.var_sqrtkusail_dn8))) + locals.var_kusail_dn8), ((locals.var_kusai00_dn10 + (((4.0 * locals.var_vgvt_dn10) * locals.var_sqrtkusail) + (assign21490_e27074 * locals.var_sqrtkusail_dn10))) + locals.var_kusail_dn10), ((locals.var_kusai00_dn11 + (((4.0 * locals.var_vgvt_dn11) * locals.var_sqrtkusail) + (assign21490_e27074 * locals.var_sqrtkusail_dn11))) + locals.var_kusail_dn11), ((locals.var_kusai00_dn12 + (((4.0 * locals.var_vgvt_dn12) * locals.var_sqrtkusail) + (assign21490_e27074 * locals.var_sqrtkusail_dn12))) + locals.var_kusail_dn12),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn8, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12,)
    }
};
        locals.var_t7 = assign21490_e27081;
        locals.var_t7_dn0 = assign21490_e27081_d_n0;
        locals.var_t7_dn2 = assign21490_e27081_d_n2;
        locals.var_t7_dn4 = assign21490_e27081_d_n4;
        locals.var_t7_dn5 = assign21490_e27081_d_n5;
        locals.var_t7_dn6 = assign21490_e27081_d_n6;
        locals.var_t7_dn8 = assign21490_e27081_d_n8;
        locals.var_t7_dn10 = assign21490_e27081_d_n10;
        locals.var_t7_dn11 = assign21490_e27081_d_n11;
        locals.var_t7_dn12 = assign21490_e27081_d_n12;
        locals.var_t7_rv = 0.0;

        let assign21510_e27105: f64 = (locals.var_ids + locals.var_idsibpc);
        locals.var_ids = assign21510_e27105;
        locals.var_ids_dn0 = (locals.var_ids_dn0 + locals.var_idsibpc_dn0);
        locals.var_ids_dn2 = (locals.var_ids_dn2 + locals.var_idsibpc_dn2);
        locals.var_ids_dn4 = (locals.var_ids_dn4 + locals.var_idsibpc_dn4);
        locals.var_ids_dn5 = (locals.var_ids_dn5 + locals.var_idsibpc_dn5);
        locals.var_ids_dn6 = (locals.var_ids_dn6 + locals.var_idsibpc_dn6);
        locals.var_ids_dn8 = (locals.var_ids_dn8 + locals.var_idsibpc_dn8);
        locals.var_ids_dn10 = (locals.var_ids_dn10 + locals.var_idsibpc_dn10);
        locals.var_ids_dn11 = (locals.var_ids_dn11 + locals.var_idsibpc_dn11);
        locals.var_ids_dn12 = (locals.var_ids_dn12 + locals.var_idsibpc_dn12);
        locals.var_ids_rv = 0.0;

        let (assign21520_e27112,) = {
    if (locals.var_cgbo_given != 0.0) {
        let assign21520_e27108: f64 = (-p.p172);
        let assign21520_e27110: f64 = (assign21520_e27108 * locals.var_lgate);
        (assign21520_e27110,)
    } else {
        (locals.var_cgbe,)
    }
};
        locals.var_cgbe = assign21520_e27112;
        locals.var_cgbe_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_88(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21530_e27120, assign21530_e27120_d_n0, assign21530_e27120_d_n2, assign21530_e27120_d_n5, assign21530_e27120_d_n6,) = {
    if (locals.var_cgbo_given != 0.0) {
        let assign21530_e27117: f64 = (locals.var_vgse - locals.var_vbse);
        let assign21530_e27118: f64 = (locals.var_cgbe * assign21530_e27117);
        (assign21530_e27118, (locals.var_cgbe * (locals.var_vgse_dn0 - locals.var_vbse_dn0)), (locals.var_cgbe * (locals.var_vgse_dn2 - locals.var_vbse_dn2)), (locals.var_cgbe * locals.var_vgse_dn5), (locals.var_cgbe * (-locals.var_vbse_dn6)),)
    } else {
        (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn5, locals.var_qgob_dn6,)
    }
};
        locals.var_qgob = assign21530_e27120;
        locals.var_qgob_dn0 = assign21530_e27120_d_n0;
        locals.var_qgob_dn2 = assign21530_e27120_d_n2;
        locals.var_qgob_dn5 = assign21530_e27120_d_n5;
        locals.var_qgob_dn6 = assign21530_e27120_d_n6;
        locals.var_qgob_rv = 0.0;

        let (assign21540_e27125,) = {
    if (locals.var_cgbo_given == 0.0) {
        (0.0,)
    } else {
        (locals.var_cgbe,)
    }
};
        locals.var_cgbe = assign21540_e27125;
        locals.var_cgbe_rv = 0.0;

        let (assign21550_e27130, assign21550_e27130_d_n0, assign21550_e27130_d_n2, assign21550_e27130_d_n5, assign21550_e27130_d_n6,) = {
    if (locals.var_cgbo_given == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qgob, locals.var_qgob_dn0, locals.var_qgob_dn2, locals.var_qgob_dn5, locals.var_qgob_dn6,)
    }
};
        locals.var_qgob = assign21550_e27130;
        locals.var_qgob_dn0 = assign21550_e27130_d_n0;
        locals.var_qgob_dn2 = assign21550_e27130_d_n2;
        locals.var_qgob_dn5 = assign21550_e27130_d_n5;
        locals.var_qgob_dn6 = assign21550_e27130_d_n6;
        locals.var_qgob_rv = 0.0;

        locals.var_cf = 0.0;
        locals.var_cf_dn0 = 0.0;
        locals.var_cf_dn2 = 0.0;
        locals.var_cf_dn4 = 0.0;
        locals.var_cf_dn5 = 0.0;
        locals.var_cf_dn6 = 0.0;
        locals.var_cf_dn8 = 0.0;
        locals.var_cf_dn10 = 0.0;
        locals.var_cf_dn11 = 0.0;
        locals.var_cf_dn12 = 0.0;
        locals.var_cf_rv = 0.0;

        let assign21570_e27143: f64 = (locals.var_vgse - locals.var_vdse);
        let assign21570_e27144: f64 = (locals.var_cf * assign21570_e27143);
        locals.var_qfd = assign21570_e27144;
        locals.var_qfd_dn0 = ((locals.var_cf_dn0 * assign21570_e27143) + (locals.var_cf * (locals.var_vgse_dn0 - locals.var_vdse_dn0)));
        locals.var_qfd_dn2 = ((locals.var_cf_dn2 * assign21570_e27143) + (locals.var_cf * (locals.var_vgse_dn2 - locals.var_vdse_dn2)));
        locals.var_qfd_dn4 = (locals.var_cf_dn4 * assign21570_e27143);
        locals.var_qfd_dn5 = ((locals.var_cf_dn5 * assign21570_e27143) + (locals.var_cf * locals.var_vgse_dn5));
        locals.var_qfd_dn6 = (locals.var_cf_dn6 * assign21570_e27143);
        locals.var_qfd_dn8 = (locals.var_cf_dn8 * assign21570_e27143);
        locals.var_qfd_dn10 = (locals.var_cf_dn10 * assign21570_e27143);
        locals.var_qfd_dn11 = (locals.var_cf_dn11 * assign21570_e27143);
        locals.var_qfd_dn12 = (locals.var_cf_dn12 * assign21570_e27143);
        locals.var_qfd_rv = 0.0;

        let assign21580_e27147: f64 = (locals.var_cf * locals.var_vgse);
        locals.var_qfs = assign21580_e27147;
        locals.var_qfs_dn0 = ((locals.var_cf_dn0 * locals.var_vgse) + (locals.var_cf * locals.var_vgse_dn0));
        locals.var_qfs_dn2 = ((locals.var_cf_dn2 * locals.var_vgse) + (locals.var_cf * locals.var_vgse_dn2));
        locals.var_qfs_dn4 = (locals.var_cf_dn4 * locals.var_vgse);
        locals.var_qfs_dn5 = ((locals.var_cf_dn5 * locals.var_vgse) + (locals.var_cf * locals.var_vgse_dn5));
        locals.var_qfs_dn6 = (locals.var_cf_dn6 * locals.var_vgse);
        locals.var_qfs_dn8 = (locals.var_cf_dn8 * locals.var_vgse);
        locals.var_qfs_dn10 = (locals.var_cf_dn10 * locals.var_vgse);
        locals.var_qfs_dn11 = (locals.var_cf_dn11 * locals.var_vgse);
        locals.var_qfs_dn12 = (locals.var_cf_dn12 * locals.var_vgse);
        locals.var_qfs_rv = 0.0;

        let assign21590_e27150: f64 = (locals.var_qgod + locals.var_qfd);
        locals.var_qgod = assign21590_e27150;
        locals.var_qgod_dn0 = (locals.var_qgod_dn0 + locals.var_qfd_dn0);
        locals.var_qgod_dn2 = (locals.var_qgod_dn2 + locals.var_qfd_dn2);
        locals.var_qgod_dn4 = (locals.var_qgod_dn4 + locals.var_qfd_dn4);
        locals.var_qgod_dn5 = (locals.var_qgod_dn5 + locals.var_qfd_dn5);
        locals.var_qgod_dn6 = (locals.var_qgod_dn6 + locals.var_qfd_dn6);
        locals.var_qgod_dn8 = (locals.var_qgod_dn8 + locals.var_qfd_dn8);
        locals.var_qgod_dn10 = (locals.var_qgod_dn10 + locals.var_qfd_dn10);
        locals.var_qgod_dn11 = (locals.var_qgod_dn11 + locals.var_qfd_dn11);
        locals.var_qgod_dn12 = (locals.var_qgod_dn12 + locals.var_qfd_dn12);
        locals.var_qgod_rv = 0.0;

        let assign21600_e27153: f64 = (locals.var_qgos + locals.var_qfs);
        locals.var_qgos = assign21600_e27153;
        locals.var_qgos_dn0 = (locals.var_qgos_dn0 + locals.var_qfs_dn0);
        locals.var_qgos_dn2 = (locals.var_qgos_dn2 + locals.var_qfs_dn2);
        locals.var_qgos_dn4 = (locals.var_qgos_dn4 + locals.var_qfs_dn4);
        locals.var_qgos_dn5 = (locals.var_qgos_dn5 + locals.var_qfs_dn5);
        locals.var_qgos_dn6 = (locals.var_qgos_dn6 + locals.var_qfs_dn6);
        locals.var_qgos_dn8 = (locals.var_qgos_dn8 + locals.var_qfs_dn8);
        locals.var_qgos_dn10 = (locals.var_qgos_dn10 + locals.var_qfs_dn10);
        locals.var_qgos_dn11 = (locals.var_qgos_dn11 + locals.var_qfs_dn11);
        locals.var_qgos_dn12 = (locals.var_qgos_dn12 + locals.var_qfs_dn12);
        locals.var_qgos_rv = 0.0;

        let assign21610_e27156: f64 = (locals.var_mfactor * locals.var_ids);
        locals.var_idse = assign21610_e27156;
        locals.var_idse_dn0 = (locals.var_mfactor * locals.var_ids_dn0);
        locals.var_idse_dn2 = (locals.var_mfactor * locals.var_ids_dn2);
        locals.var_idse_dn4 = (locals.var_mfactor * locals.var_ids_dn4);
        locals.var_idse_dn5 = (locals.var_mfactor * locals.var_ids_dn5);
        locals.var_idse_dn6 = (locals.var_mfactor * locals.var_ids_dn6);
        locals.var_idse_dn8 = (locals.var_mfactor * locals.var_ids_dn8);
        locals.var_idse_dn10 = (locals.var_mfactor * locals.var_ids_dn10);
        locals.var_idse_dn11 = (locals.var_mfactor * locals.var_ids_dn11);
        locals.var_idse_dn12 = (locals.var_mfactor * locals.var_ids_dn12);
        locals.var_idse_rv = 0.0;

        let assign21620_e27158: f64 = (-locals.var_weffcv_nf);
        let assign21620_e27160: f64 = (assign21620_e27158 * locals.var_leff);
        locals.var_t1 = assign21620_e27160;
        locals.var_t1_dn0 = (((-locals.var_weffcv_nf_dn0) * locals.var_leff) + (assign21620_e27158 * locals.var_leff_dn0));
        locals.var_t1_dn2 = (((-locals.var_weffcv_nf_dn2) * locals.var_leff) + (assign21620_e27158 * locals.var_leff_dn2));
        locals.var_t1_dn4 = (((-locals.var_weffcv_nf_dn4) * locals.var_leff) + (assign21620_e27158 * locals.var_leff_dn4));
        locals.var_t1_dn5 = (((-locals.var_weffcv_nf_dn5) * locals.var_leff) + (assign21620_e27158 * locals.var_leff_dn5));
        locals.var_t1_dn6 = (((-locals.var_weffcv_nf_dn6) * locals.var_leff) + (assign21620_e27158 * locals.var_leff_dn6));
        locals.var_t1_dn8 = (((-locals.var_weffcv_nf_dn8) * locals.var_leff) + (assign21620_e27158 * locals.var_leff_dn8));
        locals.var_t1_dn10 = (((-locals.var_weffcv_nf_dn10) * locals.var_leff) + (assign21620_e27158 * locals.var_leff_dn10));
        locals.var_t1_dn11 = (((-locals.var_weffcv_nf_dn11) * locals.var_leff) + (assign21620_e27158 * locals.var_leff_dn11));
        locals.var_t1_dn12 = (((-locals.var_weffcv_nf_dn12) * locals.var_leff) + (assign21620_e27158 * locals.var_leff_dn12));
        locals.var_t1_rv = 0.0;

        let assign21630_e27162: f64 = (-0.5);
        let assign21630_e27165: f64 = (locals.var_q_s0_dep + locals.var_q_sl_dep);
        let assign21630_e27166: f64 = (assign21630_e27162 * assign21630_e27165);
        locals.var_t2 = assign21630_e27166;
        locals.var_t2_dn0 = (assign21630_e27162 * (locals.var_q_s0_dep_dn0 + locals.var_q_sl_dep_dn0));
        locals.var_t2_dn2 = (assign21630_e27162 * (locals.var_q_s0_dep_dn2 + locals.var_q_sl_dep_dn2));
        locals.var_t2_dn4 = (assign21630_e27162 * (locals.var_q_s0_dep_dn4 + locals.var_q_sl_dep_dn4));
        locals.var_t2_dn5 = (assign21630_e27162 * (locals.var_q_s0_dep_dn5 + locals.var_q_sl_dep_dn5));
        locals.var_t2_dn6 = (assign21630_e27162 * (locals.var_q_s0_dep_dn6 + locals.var_q_sl_dep_dn6));
        locals.var_t2_dn8 = (assign21630_e27162 * (locals.var_q_s0_dep_dn8 + locals.var_q_sl_dep_dn8));
        locals.var_t2_dn10 = (assign21630_e27162 * (locals.var_q_s0_dep_dn10 + locals.var_q_sl_dep_dn10));
        locals.var_t2_dn11 = (assign21630_e27162 * (locals.var_q_s0_dep_dn11 + locals.var_q_sl_dep_dn11));
        locals.var_t2_dn12 = (assign21630_e27162 * (locals.var_q_s0_dep_dn12 + locals.var_q_sl_dep_dn12));
        locals.var_t2_rv = 0.0;

        let assign21640_e27168: f64 = (-0.5);
        let assign21640_e27171: f64 = (locals.var_q_b0_dep + locals.var_q_bl_dep);
        let assign21640_e27172: f64 = (assign21640_e27168 * assign21640_e27171);
        locals.var_t3 = assign21640_e27172;
        locals.var_t3_dn0 = (assign21640_e27168 * (locals.var_q_b0_dep_dn0 + locals.var_q_bl_dep_dn0));
        locals.var_t3_dn2 = (assign21640_e27168 * (locals.var_q_b0_dep_dn2 + locals.var_q_bl_dep_dn2));
        locals.var_t3_dn4 = (assign21640_e27168 * (locals.var_q_b0_dep_dn4 + locals.var_q_bl_dep_dn4));
        locals.var_t3_dn5 = (assign21640_e27168 * (locals.var_q_b0_dep_dn5 + locals.var_q_bl_dep_dn5));
        locals.var_t3_dn6 = (assign21640_e27168 * (locals.var_q_b0_dep_dn6 + locals.var_q_bl_dep_dn6));
        locals.var_t3_dn8 = (assign21640_e27168 * (locals.var_q_b0_dep_dn8 + locals.var_q_bl_dep_dn8));
        locals.var_t3_dn10 = (assign21640_e27168 * (locals.var_q_b0_dep_dn10 + locals.var_q_bl_dep_dn10));
        locals.var_t3_dn11 = (assign21640_e27168 * (locals.var_q_b0_dep_dn11 + locals.var_q_bl_dep_dn11));
        locals.var_t3_dn12 = (assign21640_e27168 * (locals.var_q_b0_dep_dn12 + locals.var_q_bl_dep_dn12));
        locals.var_t3_rv = 0.0;

        let assign21650_e27176: f64 = (0.1 * locals.var_c_box);
        let assign21650_e27177: f64 = (locals.var_t1 * assign21650_e27176);
        let assign21650_e27179: f64 = (assign21650_e27177 * locals.var_vbse);
        locals.var_qfs_box = assign21650_e27179;
        locals.var_qfs_box_dn0 = (((locals.var_t1_dn0 * assign21650_e27176) * locals.var_vbse) + (assign21650_e27177 * locals.var_vbse_dn0));
        locals.var_qfs_box_dn2 = (((locals.var_t1_dn2 * assign21650_e27176) * locals.var_vbse) + (assign21650_e27177 * locals.var_vbse_dn2));
        locals.var_qfs_box_dn4 = ((locals.var_t1_dn4 * assign21650_e27176) * locals.var_vbse);
        locals.var_qfs_box_dn5 = ((locals.var_t1_dn5 * assign21650_e27176) * locals.var_vbse);
        locals.var_qfs_box_dn6 = (((locals.var_t1_dn6 * assign21650_e27176) * locals.var_vbse) + (assign21650_e27177 * locals.var_vbse_dn6));
        locals.var_qfs_box_dn8 = ((locals.var_t1_dn8 * assign21650_e27176) * locals.var_vbse);
        locals.var_qfs_box_dn10 = ((locals.var_t1_dn10 * assign21650_e27176) * locals.var_vbse);
        locals.var_qfs_box_dn11 = ((locals.var_t1_dn11 * assign21650_e27176) * locals.var_vbse);
        locals.var_qfs_box_dn12 = ((locals.var_t1_dn12 * assign21650_e27176) * locals.var_vbse);
        locals.var_qfs_box_rv = 0.0;

        let assign21660_e27183: f64 = (0.1 * locals.var_c_box);
        let assign21660_e27184: f64 = (locals.var_t1 * assign21660_e27183);
        let assign21660_e27187: f64 = (locals.var_vbse - locals.var_vdse);
        let assign21660_e27188: f64 = (assign21660_e27184 * assign21660_e27187);
        locals.var_qfd_box = assign21660_e27188;
        locals.var_qfd_box_dn0 = (((locals.var_t1_dn0 * assign21660_e27183) * assign21660_e27187) + (assign21660_e27184 * (locals.var_vbse_dn0 - locals.var_vdse_dn0)));
        locals.var_qfd_box_dn2 = (((locals.var_t1_dn2 * assign21660_e27183) * assign21660_e27187) + (assign21660_e27184 * (locals.var_vbse_dn2 - locals.var_vdse_dn2)));
        locals.var_qfd_box_dn4 = ((locals.var_t1_dn4 * assign21660_e27183) * assign21660_e27187);
        locals.var_qfd_box_dn5 = ((locals.var_t1_dn5 * assign21660_e27183) * assign21660_e27187);
        locals.var_qfd_box_dn6 = (((locals.var_t1_dn6 * assign21660_e27183) * assign21660_e27187) + (assign21660_e27184 * locals.var_vbse_dn6));
        locals.var_qfd_box_dn8 = ((locals.var_t1_dn8 * assign21660_e27183) * assign21660_e27187);
        locals.var_qfd_box_dn10 = ((locals.var_t1_dn10 * assign21660_e27183) * assign21660_e27187);
        locals.var_qfd_box_dn11 = ((locals.var_t1_dn11 * assign21660_e27183) * assign21660_e27187);
        locals.var_qfd_box_dn12 = ((locals.var_t1_dn12 * assign21660_e27183) * assign21660_e27187);
        locals.var_qfd_box_rv = 0.0;

        let assign21670_e27191: f64 = (locals.var_t1 * locals.var_t2);
        locals.var_qs_dep = assign21670_e27191;
        locals.var_qs_dep_dn0 = ((locals.var_t1_dn0 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn0));
        locals.var_qs_dep_dn2 = ((locals.var_t1_dn2 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn2));
        locals.var_qs_dep_dn4 = ((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4));
        locals.var_qs_dep_dn5 = ((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5));
        locals.var_qs_dep_dn6 = ((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6));
        locals.var_qs_dep_dn8 = ((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8));
        locals.var_qs_dep_dn10 = ((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10));
        locals.var_qs_dep_dn11 = ((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11));
        locals.var_qs_dep_dn12 = ((locals.var_t1_dn12 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn12));
        locals.var_qs_dep_rv = 0.0;

        let assign21680_e27194: f64 = (locals.var_t1 * locals.var_t3);
        locals.var_qb_dep = assign21680_e27194;
        locals.var_qb_dep_dn0 = ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0));
        locals.var_qb_dep_dn2 = ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2));
        locals.var_qb_dep_dn4 = ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4));
        locals.var_qb_dep_dn5 = ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5));
        locals.var_qb_dep_dn6 = ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6));
        locals.var_qb_dep_dn8 = ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8));
        locals.var_qb_dep_dn10 = ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10));
        locals.var_qb_dep_dn11 = ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11));
        locals.var_qb_dep_dn12 = ((locals.var_t1_dn12 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn12));
        locals.var_qb_dep_rv = 0.0;

        let (assign21690_e27198, assign21690_e27198_d_n0, assign21690_e27198_d_n2, assign21690_e27198_d_n4, assign21690_e27198_d_n5, assign21690_e27198_d_n6, assign21690_e27198_d_n8, assign21690_e27198_d_n10, assign21690_e27198_d_n11, assign21690_e27198_d_n12,) = {
    if (p.p303 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qsub, locals.var_qsub_dn0, locals.var_qsub_dn2, locals.var_qsub_dn4, locals.var_qsub_dn5, locals.var_qsub_dn6, locals.var_qsub_dn8, locals.var_qsub_dn10, locals.var_qsub_dn11, locals.var_qsub_dn12,)
    }
};
        locals.var_qsub = assign21690_e27198;
        locals.var_qsub_dn0 = assign21690_e27198_d_n0;
        locals.var_qsub_dn2 = assign21690_e27198_d_n2;
        locals.var_qsub_dn4 = assign21690_e27198_d_n4;
        locals.var_qsub_dn5 = assign21690_e27198_d_n5;
        locals.var_qsub_dn6 = assign21690_e27198_d_n6;
        locals.var_qsub_dn8 = assign21690_e27198_d_n8;
        locals.var_qsub_dn10 = assign21690_e27198_d_n10;
        locals.var_qsub_dn11 = assign21690_e27198_d_n11;
        locals.var_qsub_dn12 = assign21690_e27198_d_n12;
        locals.var_qsub_rv = 0.0;

        let (assign21700_e27202, assign21700_e27202_d_n0, assign21700_e27202_d_n2, assign21700_e27202_d_n4, assign21700_e27202_d_n5, assign21700_e27202_d_n6, assign21700_e27202_d_n8, assign21700_e27202_d_n10, assign21700_e27202_d_n11, assign21700_e27202_d_n12,) = {
    if (p.p303 != 0.0) {
        (locals.var_qi, locals.var_qi_dn0, locals.var_qi_dn2, locals.var_qi_dn4, locals.var_qi_dn5, locals.var_qi_dn6, locals.var_qi_dn8, locals.var_qi_dn10, locals.var_qi_dn11, locals.var_qi_dn12,)
    } else {
        (locals.var_qidep, locals.var_qidep_dn0, locals.var_qidep_dn2, locals.var_qidep_dn4, locals.var_qidep_dn5, locals.var_qidep_dn6, locals.var_qidep_dn8, locals.var_qidep_dn10, locals.var_qidep_dn11, locals.var_qidep_dn12,)
    }
};
        locals.var_qidep = assign21700_e27202;
        locals.var_qidep_dn0 = assign21700_e27202_d_n0;
        locals.var_qidep_dn2 = assign21700_e27202_d_n2;
        locals.var_qidep_dn4 = assign21700_e27202_d_n4;
        locals.var_qidep_dn5 = assign21700_e27202_d_n5;
        locals.var_qidep_dn6 = assign21700_e27202_d_n6;
        locals.var_qidep_dn8 = assign21700_e27202_d_n8;
        locals.var_qidep_dn10 = assign21700_e27202_d_n10;
        locals.var_qidep_dn11 = assign21700_e27202_d_n11;
        locals.var_qidep_dn12 = assign21700_e27202_d_n12;
        locals.var_qidep_rv = 0.0;

        let (assign21710_e27211, assign21710_e27211_d_n0, assign21710_e27211_d_n2, assign21710_e27211_d_n4, assign21710_e27211_d_n5, assign21710_e27211_d_n6, assign21710_e27211_d_n8, assign21710_e27211_d_n10, assign21710_e27211_d_n11, assign21710_e27211_d_n12,) = {
    if (p.p303 == 0.0) {
        let assign21710_e27207: f64 = (locals.var_qi + locals.var_qs_dep);
        let assign21710_e27209: f64 = (assign21710_e27207 + locals.var_qb_dep);
        (assign21710_e27209, ((locals.var_qi_dn0 + locals.var_qs_dep_dn0) + locals.var_qb_dep_dn0), ((locals.var_qi_dn2 + locals.var_qs_dep_dn2) + locals.var_qb_dep_dn2), ((locals.var_qi_dn4 + locals.var_qs_dep_dn4) + locals.var_qb_dep_dn4), ((locals.var_qi_dn5 + locals.var_qs_dep_dn5) + locals.var_qb_dep_dn5), ((locals.var_qi_dn6 + locals.var_qs_dep_dn6) + locals.var_qb_dep_dn6), ((locals.var_qi_dn8 + locals.var_qs_dep_dn8) + locals.var_qb_dep_dn8), ((locals.var_qi_dn10 + locals.var_qs_dep_dn10) + locals.var_qb_dep_dn10), ((locals.var_qi_dn11 + locals.var_qs_dep_dn11) + locals.var_qb_dep_dn11), ((locals.var_qi_dn12 + locals.var_qs_dep_dn12) + locals.var_qb_dep_dn12),)
    } else {
        (locals.var_qidep, locals.var_qidep_dn0, locals.var_qidep_dn2, locals.var_qidep_dn4, locals.var_qidep_dn5, locals.var_qidep_dn6, locals.var_qidep_dn8, locals.var_qidep_dn10, locals.var_qidep_dn11, locals.var_qidep_dn12,)
    }
};
        locals.var_qidep = assign21710_e27211;
        locals.var_qidep_dn0 = assign21710_e27211_d_n0;
        locals.var_qidep_dn2 = assign21710_e27211_d_n2;
        locals.var_qidep_dn4 = assign21710_e27211_d_n4;
        locals.var_qidep_dn5 = assign21710_e27211_d_n5;
        locals.var_qidep_dn6 = assign21710_e27211_d_n6;
        locals.var_qidep_dn8 = assign21710_e27211_d_n8;
        locals.var_qidep_dn10 = assign21710_e27211_d_n10;
        locals.var_qidep_dn11 = assign21710_e27211_d_n11;
        locals.var_qidep_dn12 = assign21710_e27211_d_n12;
        locals.var_qidep_rv = 0.0;

        let assign21720_e27214: f64 = (locals.var_qidep * locals.var_qdrat);
        locals.var_qd = assign21720_e27214;
        locals.var_qd_dn0 = (locals.var_qidep_dn0 * locals.var_qdrat);
        locals.var_qd_dn2 = (locals.var_qidep_dn2 * locals.var_qdrat);
        locals.var_qd_dn4 = (locals.var_qidep_dn4 * locals.var_qdrat);
        locals.var_qd_dn5 = (locals.var_qidep_dn5 * locals.var_qdrat);
        locals.var_qd_dn6 = (locals.var_qidep_dn6 * locals.var_qdrat);
        locals.var_qd_dn8 = (locals.var_qidep_dn8 * locals.var_qdrat);
        locals.var_qd_dn10 = (locals.var_qidep_dn10 * locals.var_qdrat);
        locals.var_qd_dn11 = (locals.var_qidep_dn11 * locals.var_qdrat);
        locals.var_qd_dn12 = (locals.var_qidep_dn12 * locals.var_qdrat);
        locals.var_qd_rv = 0.0;

        let (assign21730_e27218, assign21730_e27218_d_n0, assign21730_e27218_d_n2, assign21730_e27218_d_n4, assign21730_e27218_d_n5, assign21730_e27218_d_n6, assign21730_e27218_d_n8, assign21730_e27218_d_n10, assign21730_e27218_d_n11, assign21730_e27218_d_n12,) = {
    if (locals.var_flg_nqs != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn8, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12,)
    }
};
        locals.var_qde = assign21730_e27218;
        locals.var_qde_dn0 = assign21730_e27218_d_n0;
        locals.var_qde_dn2 = assign21730_e27218_d_n2;
        locals.var_qde_dn4 = assign21730_e27218_d_n4;
        locals.var_qde_dn5 = assign21730_e27218_d_n5;
        locals.var_qde_dn6 = assign21730_e27218_d_n6;
        locals.var_qde_dn8 = assign21730_e27218_d_n8;
        locals.var_qde_dn10 = assign21730_e27218_d_n10;
        locals.var_qde_dn11 = assign21730_e27218_d_n11;
        locals.var_qde_dn12 = assign21730_e27218_d_n12;
        locals.var_qde_rv = 0.0;

        let (assign21740_e27222, assign21740_e27222_d_n0, assign21740_e27222_d_n2, assign21740_e27222_d_n4, assign21740_e27222_d_n5, assign21740_e27222_d_n6, assign21740_e27222_d_n8, assign21740_e27222_d_n10, assign21740_e27222_d_n11, assign21740_e27222_d_n12,) = {
    if (locals.var_flg_nqs != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn8, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12,)
    }
};
        locals.var_qge = assign21740_e27222;
        locals.var_qge_dn0 = assign21740_e27222_d_n0;
        locals.var_qge_dn2 = assign21740_e27222_d_n2;
        locals.var_qge_dn4 = assign21740_e27222_d_n4;
        locals.var_qge_dn5 = assign21740_e27222_d_n5;
        locals.var_qge_dn6 = assign21740_e27222_d_n6;
        locals.var_qge_dn8 = assign21740_e27222_d_n8;
        locals.var_qge_dn10 = assign21740_e27222_d_n10;
        locals.var_qge_dn11 = assign21740_e27222_d_n11;
        locals.var_qge_dn12 = assign21740_e27222_d_n12;
        locals.var_qge_rv = 0.0;

        let (assign21750_e27226, assign21750_e27226_d_n0, assign21750_e27226_d_n2, assign21750_e27226_d_n4, assign21750_e27226_d_n5, assign21750_e27226_d_n6, assign21750_e27226_d_n8, assign21750_e27226_d_n10, assign21750_e27226_d_n11, assign21750_e27226_d_n12,) = {
    if (locals.var_flg_nqs != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn4, locals.var_qbe_dn5, locals.var_qbe_dn6, locals.var_qbe_dn8, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12,)
    }
};
        locals.var_qbe = assign21750_e27226;
        locals.var_qbe_dn0 = assign21750_e27226_d_n0;
        locals.var_qbe_dn2 = assign21750_e27226_d_n2;
        locals.var_qbe_dn4 = assign21750_e27226_d_n4;
        locals.var_qbe_dn5 = assign21750_e27226_d_n5;
        locals.var_qbe_dn6 = assign21750_e27226_d_n6;
        locals.var_qbe_dn8 = assign21750_e27226_d_n8;
        locals.var_qbe_dn10 = assign21750_e27226_d_n10;
        locals.var_qbe_dn11 = assign21750_e27226_d_n11;
        locals.var_qbe_dn12 = assign21750_e27226_d_n12;
        locals.var_qbe_rv = 0.0;

        let (assign21760_e27232, assign21760_e27232_d_n0, assign21760_e27232_d_n2, assign21760_e27232_d_n4, assign21760_e27232_d_n5, assign21760_e27232_d_n6, assign21760_e27232_d_n8, assign21760_e27232_d_n10, assign21760_e27232_d_n11, assign21760_e27232_d_n12,) = {
    if (locals.var_flg_nqs != 0.0) {
        let assign21760_e27230: f64 = (locals.var_mfactor * locals.var_qsub);
        (assign21760_e27230, (locals.var_mfactor * locals.var_qsub_dn0), (locals.var_mfactor * locals.var_qsub_dn2), (locals.var_mfactor * locals.var_qsub_dn4), (locals.var_mfactor * locals.var_qsub_dn5), (locals.var_mfactor * locals.var_qsub_dn6), (locals.var_mfactor * locals.var_qsub_dn8), (locals.var_mfactor * locals.var_qsub_dn10), (locals.var_mfactor * locals.var_qsub_dn11), (locals.var_mfactor * locals.var_qsub_dn12),)
    } else {
        (locals.var_qb_qs, locals.var_qb_qs_dn0, locals.var_qb_qs_dn2, locals.var_qb_qs_dn4, locals.var_qb_qs_dn5, locals.var_qb_qs_dn6, locals.var_qb_qs_dn8, locals.var_qb_qs_dn10, locals.var_qb_qs_dn11, locals.var_qb_qs_dn12,)
    }
};
        locals.var_qb_qs = assign21760_e27232;
        locals.var_qb_qs_dn0 = assign21760_e27232_d_n0;
        locals.var_qb_qs_dn2 = assign21760_e27232_d_n2;
        locals.var_qb_qs_dn4 = assign21760_e27232_d_n4;
        locals.var_qb_qs_dn5 = assign21760_e27232_d_n5;
        locals.var_qb_qs_dn6 = assign21760_e27232_d_n6;
        locals.var_qb_qs_dn8 = assign21760_e27232_d_n8;
        locals.var_qb_qs_dn10 = assign21760_e27232_d_n10;
        locals.var_qb_qs_dn11 = assign21760_e27232_d_n11;
        locals.var_qb_qs_dn12 = assign21760_e27232_d_n12;
        locals.var_qb_qs_rv = 0.0;

        let (assign21770_e27238, assign21770_e27238_d_n0, assign21770_e27238_d_n2, assign21770_e27238_d_n4, assign21770_e27238_d_n5, assign21770_e27238_d_n6, assign21770_e27238_d_n8, assign21770_e27238_d_n10, assign21770_e27238_d_n11, assign21770_e27238_d_n12,) = {
    if (locals.var_flg_nqs != 0.0) {
        let assign21770_e27236: f64 = (locals.var_mfactor * locals.var_qidep);
        (assign21770_e27236, (locals.var_mfactor * locals.var_qidep_dn0), (locals.var_mfactor * locals.var_qidep_dn2), (locals.var_mfactor * locals.var_qidep_dn4), (locals.var_mfactor * locals.var_qidep_dn5), (locals.var_mfactor * locals.var_qidep_dn6), (locals.var_mfactor * locals.var_qidep_dn8), (locals.var_mfactor * locals.var_qidep_dn10), (locals.var_mfactor * locals.var_qidep_dn11), (locals.var_mfactor * locals.var_qidep_dn12),)
    } else {
        (locals.var_qi_qs, locals.var_qi_qs_dn0, locals.var_qi_qs_dn2, locals.var_qi_qs_dn4, locals.var_qi_qs_dn5, locals.var_qi_qs_dn6, locals.var_qi_qs_dn8, locals.var_qi_qs_dn10, locals.var_qi_qs_dn11, locals.var_qi_qs_dn12,)
    }
};
        locals.var_qi_qs = assign21770_e27238;
        locals.var_qi_qs_dn0 = assign21770_e27238_d_n0;
        locals.var_qi_qs_dn2 = assign21770_e27238_d_n2;
        locals.var_qi_qs_dn4 = assign21770_e27238_d_n4;
        locals.var_qi_qs_dn5 = assign21770_e27238_d_n5;
        locals.var_qi_qs_dn6 = assign21770_e27238_d_n6;
        locals.var_qi_qs_dn8 = assign21770_e27238_d_n8;
        locals.var_qi_qs_dn10 = assign21770_e27238_d_n10;
        locals.var_qi_qs_dn11 = assign21770_e27238_d_n11;
        locals.var_qi_qs_dn12 = assign21770_e27238_d_n12;
        locals.var_qi_qs_rv = 0.0;

        let (assign21780_e27248, assign21780_e27248_d_n0, assign21780_e27248_d_n2, assign21780_e27248_d_n4, assign21780_e27248_d_n5, assign21780_e27248_d_n6, assign21780_e27248_d_n8, assign21780_e27248_d_n10, assign21780_e27248_d_n11, assign21780_e27248_d_n12,) = {
    if (locals.var_flg_nqs == 0.0) {
        let assign21780_e27243: f64 = (-locals.var_qsub);
        let assign21780_e27245: f64 = (assign21780_e27243 - locals.var_qidep);
        let assign21780_e27246: f64 = (locals.var_mfactor * assign21780_e27245);
        (assign21780_e27246, (locals.var_mfactor * ((-locals.var_qsub_dn0) - locals.var_qidep_dn0)), (locals.var_mfactor * ((-locals.var_qsub_dn2) - locals.var_qidep_dn2)), (locals.var_mfactor * ((-locals.var_qsub_dn4) - locals.var_qidep_dn4)), (locals.var_mfactor * ((-locals.var_qsub_dn5) - locals.var_qidep_dn5)), (locals.var_mfactor * ((-locals.var_qsub_dn6) - locals.var_qidep_dn6)), (locals.var_mfactor * ((-locals.var_qsub_dn8) - locals.var_qidep_dn8)), (locals.var_mfactor * ((-locals.var_qsub_dn10) - locals.var_qidep_dn10)), (locals.var_mfactor * ((-locals.var_qsub_dn11) - locals.var_qidep_dn11)), (locals.var_mfactor * ((-locals.var_qsub_dn12) - locals.var_qidep_dn12)),)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn8, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12,)
    }
};
        locals.var_qge = assign21780_e27248;
        locals.var_qge_dn0 = assign21780_e27248_d_n0;
        locals.var_qge_dn2 = assign21780_e27248_d_n2;
        locals.var_qge_dn4 = assign21780_e27248_d_n4;
        locals.var_qge_dn5 = assign21780_e27248_d_n5;
        locals.var_qge_dn6 = assign21780_e27248_d_n6;
        locals.var_qge_dn8 = assign21780_e27248_d_n8;
        locals.var_qge_dn10 = assign21780_e27248_d_n10;
        locals.var_qge_dn11 = assign21780_e27248_d_n11;
        locals.var_qge_dn12 = assign21780_e27248_d_n12;
        locals.var_qge_rv = 0.0;

        let (assign21790_e27257, assign21790_e27257_d_n0, assign21790_e27257_d_n2, assign21790_e27257_d_n4, assign21790_e27257_d_n5, assign21790_e27257_d_n6, assign21790_e27257_d_n8, assign21790_e27257_d_n10, assign21790_e27257_d_n11, assign21790_e27257_d_n12,) = {
    if (locals.var_flg_nqs == 0.0) {
        let assign21790_e27254: f64 = (locals.var_qd + locals.var_qfd_box);
        let assign21790_e27255: f64 = (locals.var_mfactor * assign21790_e27254);
        (assign21790_e27255, (locals.var_mfactor * (locals.var_qd_dn0 + locals.var_qfd_box_dn0)), (locals.var_mfactor * (locals.var_qd_dn2 + locals.var_qfd_box_dn2)), (locals.var_mfactor * (locals.var_qd_dn4 + locals.var_qfd_box_dn4)), (locals.var_mfactor * (locals.var_qd_dn5 + locals.var_qfd_box_dn5)), (locals.var_mfactor * (locals.var_qd_dn6 + locals.var_qfd_box_dn6)), (locals.var_mfactor * (locals.var_qd_dn8 + locals.var_qfd_box_dn8)), (locals.var_mfactor * (locals.var_qd_dn10 + locals.var_qfd_box_dn10)), (locals.var_mfactor * (locals.var_qd_dn11 + locals.var_qfd_box_dn11)), (locals.var_mfactor * (locals.var_qd_dn12 + locals.var_qfd_box_dn12)),)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn8, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12,)
    }
};
        locals.var_qde = assign21790_e27257;
        locals.var_qde_dn0 = assign21790_e27257_d_n0;
        locals.var_qde_dn2 = assign21790_e27257_d_n2;
        locals.var_qde_dn4 = assign21790_e27257_d_n4;
        locals.var_qde_dn5 = assign21790_e27257_d_n5;
        locals.var_qde_dn6 = assign21790_e27257_d_n6;
        locals.var_qde_dn8 = assign21790_e27257_d_n8;
        locals.var_qde_dn10 = assign21790_e27257_d_n10;
        locals.var_qde_dn11 = assign21790_e27257_d_n11;
        locals.var_qde_dn12 = assign21790_e27257_d_n12;
        locals.var_qde_rv = 0.0;

        let (assign21800_e27268, assign21800_e27268_d_n0, assign21800_e27268_d_n2, assign21800_e27268_d_n4, assign21800_e27268_d_n5, assign21800_e27268_d_n6, assign21800_e27268_d_n8, assign21800_e27268_d_n10, assign21800_e27268_d_n11, assign21800_e27268_d_n12,) = {
    if (locals.var_flg_nqs == 0.0) {
        let assign21800_e27263: f64 = (locals.var_qidep - locals.var_qd);
        let assign21800_e27265: f64 = (assign21800_e27263 + locals.var_qfs_box);
        let assign21800_e27266: f64 = (locals.var_mfactor * assign21800_e27265);
        (assign21800_e27266, (locals.var_mfactor * ((locals.var_qidep_dn0 - locals.var_qd_dn0) + locals.var_qfs_box_dn0)), (locals.var_mfactor * ((locals.var_qidep_dn2 - locals.var_qd_dn2) + locals.var_qfs_box_dn2)), (locals.var_mfactor * ((locals.var_qidep_dn4 - locals.var_qd_dn4) + locals.var_qfs_box_dn4)), (locals.var_mfactor * ((locals.var_qidep_dn5 - locals.var_qd_dn5) + locals.var_qfs_box_dn5)), (locals.var_mfactor * ((locals.var_qidep_dn6 - locals.var_qd_dn6) + locals.var_qfs_box_dn6)), (locals.var_mfactor * ((locals.var_qidep_dn8 - locals.var_qd_dn8) + locals.var_qfs_box_dn8)), (locals.var_mfactor * ((locals.var_qidep_dn10 - locals.var_qd_dn10) + locals.var_qfs_box_dn10)), (locals.var_mfactor * ((locals.var_qidep_dn11 - locals.var_qd_dn11) + locals.var_qfs_box_dn11)), (locals.var_mfactor * ((locals.var_qidep_dn12 - locals.var_qd_dn12) + locals.var_qfs_box_dn12)),)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn8, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12,)
    }
};
        locals.var_qse = assign21800_e27268;
        locals.var_qse_dn0 = assign21800_e27268_d_n0;
        locals.var_qse_dn2 = assign21800_e27268_d_n2;
        locals.var_qse_dn4 = assign21800_e27268_d_n4;
        locals.var_qse_dn5 = assign21800_e27268_d_n5;
        locals.var_qse_dn6 = assign21800_e27268_d_n6;
        locals.var_qse_dn8 = assign21800_e27268_d_n8;
        locals.var_qse_dn10 = assign21800_e27268_d_n10;
        locals.var_qse_dn11 = assign21800_e27268_d_n11;
        locals.var_qse_dn12 = assign21800_e27268_d_n12;
        locals.var_qse_rv = 0.0;

        let assign21810_e27271: f64 = if p.p45 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard372 = assign21810_e27271;
        locals.var_guard372_rv = 0.0;

        let (assign21820_e27275, assign21820_e27275_d_n0, assign21820_e27275_d_n2, assign21820_e27275_d_n4, assign21820_e27275_d_n5, assign21820_e27275_d_n6, assign21820_e27275_d_n8, assign21820_e27275_d_n10, assign21820_e27275_d_n11, assign21820_e27275_d_n12,) = {
    if (locals.var_guard372 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, locals.var_qy_dn5, locals.var_qy_dn6, locals.var_qy_dn8, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn12,)
    }
};
        locals.var_qy = assign21820_e27275;
        locals.var_qy_dn0 = assign21820_e27275_d_n0;
        locals.var_qy_dn2 = assign21820_e27275_d_n2;
        locals.var_qy_dn4 = assign21820_e27275_d_n4;
        locals.var_qy_dn5 = assign21820_e27275_d_n5;
        locals.var_qy_dn6 = assign21820_e27275_d_n6;
        locals.var_qy_dn8 = assign21820_e27275_d_n8;
        locals.var_qy_dn10 = assign21820_e27275_d_n10;
        locals.var_qy_dn11 = assign21820_e27275_d_n11;
        locals.var_qy_dn12 = assign21820_e27275_d_n12;
        locals.var_qy_rv = 0.0;

        let (assign21830_e27284, assign21830_e27284_d_n0, assign21830_e27284_d_n2, assign21830_e27284_d_n4, assign21830_e27284_d_n5, assign21830_e27284_d_n6, assign21830_e27284_d_n8, assign21830_e27284_d_n10, assign21830_e27284_d_n11, assign21830_e27284_d_n12,) = {
    if (locals.var_guard372 == 0.0) {
        let assign21830_e27280: f64 = (locals.var_ec * locals.var_leff);
        let assign21830_e27282: f64 = (assign21830_e27280 + locals.var_ps0);
        (assign21830_e27282, (((locals.var_ec_dn0 * locals.var_leff) + (locals.var_ec * locals.var_leff_dn0)) + locals.var_ps0_dn0), (((locals.var_ec_dn2 * locals.var_leff) + (locals.var_ec * locals.var_leff_dn2)) + locals.var_ps0_dn2), (((locals.var_ec_dn4 * locals.var_leff) + (locals.var_ec * locals.var_leff_dn4)) + locals.var_ps0_dn4), (((locals.var_ec_dn5 * locals.var_leff) + (locals.var_ec * locals.var_leff_dn5)) + locals.var_ps0_dn5), (((locals.var_ec_dn6 * locals.var_leff) + (locals.var_ec * locals.var_leff_dn6)) + locals.var_ps0_dn6), (((locals.var_ec_dn8 * locals.var_leff) + (locals.var_ec * locals.var_leff_dn8)) + locals.var_ps0_dn8), (((locals.var_ec_dn10 * locals.var_leff) + (locals.var_ec * locals.var_leff_dn10)) + locals.var_ps0_dn10), (((locals.var_ec_dn11 * locals.var_leff) + (locals.var_ec * locals.var_leff_dn11)) + locals.var_ps0_dn11), (((locals.var_ec_dn12 * locals.var_leff) + (locals.var_ec * locals.var_leff_dn12)) + locals.var_ps0_dn12),)
    } else {
        (locals.var_pslk, locals.var_pslk_dn0, locals.var_pslk_dn2, locals.var_pslk_dn4, locals.var_pslk_dn5, locals.var_pslk_dn6, locals.var_pslk_dn8, locals.var_pslk_dn10, locals.var_pslk_dn11, locals.var_pslk_dn12,)
    }
};
        locals.var_pslk = assign21830_e27284;
        locals.var_pslk_dn0 = assign21830_e27284_d_n0;
        locals.var_pslk_dn2 = assign21830_e27284_d_n2;
        locals.var_pslk_dn4 = assign21830_e27284_d_n4;
        locals.var_pslk_dn5 = assign21830_e27284_d_n5;
        locals.var_pslk_dn6 = assign21830_e27284_d_n6;
        locals.var_pslk_dn8 = assign21830_e27284_d_n8;
        locals.var_pslk_dn10 = assign21830_e27284_d_n10;
        locals.var_pslk_dn11 = assign21830_e27284_d_n11;
        locals.var_pslk_dn12 = assign21830_e27284_d_n12;
        locals.var_pslk_rv = 0.0;

        let assign21840_e27287: f64 = if locals.var_pslk > locals.var_psdl { 1.0 } else { 0.0 };
        locals.var_guard373 = assign21840_e27287;
        locals.var_guard373_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_89(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21850_e27294, assign21850_e27294_d_n0, assign21850_e27294_d_n2, assign21850_e27294_d_n4, assign21850_e27294_d_n5, assign21850_e27294_d_n6, assign21850_e27294_d_n8, assign21850_e27294_d_n10, assign21850_e27294_d_n11, assign21850_e27294_d_n12,) = {
    if ((locals.var_guard372 == 0.0) && (locals.var_guard373 != 0.0)) {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn8, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12,)
    } else {
        (locals.var_pslk, locals.var_pslk_dn0, locals.var_pslk_dn2, locals.var_pslk_dn4, locals.var_pslk_dn5, locals.var_pslk_dn6, locals.var_pslk_dn8, locals.var_pslk_dn10, locals.var_pslk_dn11, locals.var_pslk_dn12,)
    }
};
        locals.var_pslk = assign21850_e27294;
        locals.var_pslk_dn0 = assign21850_e27294_d_n0;
        locals.var_pslk_dn2 = assign21850_e27294_d_n2;
        locals.var_pslk_dn4 = assign21850_e27294_d_n4;
        locals.var_pslk_dn5 = assign21850_e27294_d_n5;
        locals.var_pslk_dn6 = assign21850_e27294_d_n6;
        locals.var_pslk_dn8 = assign21850_e27294_d_n8;
        locals.var_pslk_dn10 = assign21850_e27294_d_n10;
        locals.var_pslk_dn11 = assign21850_e27294_d_n11;
        locals.var_pslk_dn12 = assign21850_e27294_d_n12;
        locals.var_pslk_rv = 0.0;

        let (assign21860_e27309, assign21860_e27309_d_n0, assign21860_e27309_d_n2, assign21860_e27309_d_n4, assign21860_e27309_d_n5, assign21860_e27309_d_n6, assign21860_e27309_d_n8, assign21860_e27309_d_n10, assign21860_e27309_d_n11, assign21860_e27309_d_n12,) = {
    if (locals.var_guard372 == 0.0) {
        let assign21860_e27300: f64 = (locals.var_vds + locals.var_ps0);
        let assign21860_e27301: f64 = (locals.var_aclm * assign21860_e27300);
        let assign21860_e27304: f64 = (1.0 - locals.var_aclm);
        let assign21860_e27306: f64 = (assign21860_e27304 * locals.var_pslk);
        let assign21860_e27307: f64 = (assign21860_e27301 + assign21860_e27306);
        (assign21860_e27307, ((locals.var_aclm * (locals.var_vds_dn0 + locals.var_ps0_dn0)) + (assign21860_e27304 * locals.var_pslk_dn0)), ((locals.var_aclm * (locals.var_vds_dn2 + locals.var_ps0_dn2)) + (assign21860_e27304 * locals.var_pslk_dn2)), ((locals.var_aclm * (locals.var_vds_dn4 + locals.var_ps0_dn4)) + (assign21860_e27304 * locals.var_pslk_dn4)), ((locals.var_aclm * (locals.var_vds_dn5 + locals.var_ps0_dn5)) + (assign21860_e27304 * locals.var_pslk_dn5)), ((locals.var_aclm * (locals.var_vds_dn6 + locals.var_ps0_dn6)) + (assign21860_e27304 * locals.var_pslk_dn6)), ((locals.var_aclm * (locals.var_vds_dn8 + locals.var_ps0_dn8)) + (assign21860_e27304 * locals.var_pslk_dn8)), ((locals.var_aclm * (locals.var_vds_dn10 + locals.var_ps0_dn10)) + (assign21860_e27304 * locals.var_pslk_dn10)), ((locals.var_aclm * (locals.var_vds_dn11 + locals.var_ps0_dn11)) + (assign21860_e27304 * locals.var_pslk_dn11)), ((locals.var_aclm * (locals.var_vds_dn12 + locals.var_ps0_dn12)) + (assign21860_e27304 * locals.var_pslk_dn12)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign21860_e27309;
        locals.var_t1_dn0 = assign21860_e27309_d_n0;
        locals.var_t1_dn2 = assign21860_e27309_d_n2;
        locals.var_t1_dn4 = assign21860_e27309_d_n4;
        locals.var_t1_dn5 = assign21860_e27309_d_n5;
        locals.var_t1_dn6 = assign21860_e27309_d_n6;
        locals.var_t1_dn8 = assign21860_e27309_d_n8;
        locals.var_t1_dn10 = assign21860_e27309_d_n10;
        locals.var_t1_dn11 = assign21860_e27309_d_n11;
        locals.var_t1_dn12 = assign21860_e27309_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign21870_e27319, assign21870_e27319_d_n0, assign21870_e27319_d_n2, assign21870_e27319_d_n4, assign21870_e27319_d_n5, assign21870_e27319_d_n6, assign21870_e27319_d_n8, assign21870_e27319_d_n10, assign21870_e27319_d_n11, assign21870_e27319_d_n12,) = {
    if (locals.var_guard372 == 0.0) {
        let assign21870_e27314: f64 = (2.0 * 1.034943e-10);
        let assign21870_e27316: f64 = (assign21870_e27314 / locals.var_q_nsub);
        let assign21870_e27317: f64 = (assign21870_e27316).sqrt();
        (assign21870_e27317, ((-((assign21870_e27314 * locals.var_q_nsub_dn0) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign21870_e27317)), ((-((assign21870_e27314 * locals.var_q_nsub_dn2) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign21870_e27317)), ((-((assign21870_e27314 * locals.var_q_nsub_dn4) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign21870_e27317)), ((-((assign21870_e27314 * locals.var_q_nsub_dn5) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign21870_e27317)), ((-((assign21870_e27314 * locals.var_q_nsub_dn6) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign21870_e27317)), ((-((assign21870_e27314 * locals.var_q_nsub_dn8) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign21870_e27317)), ((-((assign21870_e27314 * locals.var_q_nsub_dn10) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign21870_e27317)), ((-((assign21870_e27314 * locals.var_q_nsub_dn11) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign21870_e27317)), ((-((assign21870_e27314 * locals.var_q_nsub_dn12) / (locals.var_q_nsub * locals.var_q_nsub))) / (2.0 * assign21870_e27317)),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn8, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12,)
    }
};
        locals.var_t10 = assign21870_e27319;
        locals.var_t10_dn0 = assign21870_e27319_d_n0;
        locals.var_t10_dn2 = assign21870_e27319_d_n2;
        locals.var_t10_dn4 = assign21870_e27319_d_n4;
        locals.var_t10_dn5 = assign21870_e27319_d_n5;
        locals.var_t10_dn6 = assign21870_e27319_d_n6;
        locals.var_t10_dn8 = assign21870_e27319_d_n8;
        locals.var_t10_dn10 = assign21870_e27319_d_n10;
        locals.var_t10_dn11 = assign21870_e27319_d_n11;
        locals.var_t10_dn12 = assign21870_e27319_d_n12;
        locals.var_t10_rv = 0.0;

        let (assign21880_e27326, assign21880_e27326_d_n0, assign21880_e27326_d_n2, assign21880_e27326_d_n4, assign21880_e27326_d_n5, assign21880_e27326_d_n6, assign21880_e27326_d_n8, assign21880_e27326_d_n10, assign21880_e27326_d_n11, assign21880_e27326_d_n12,) = {
    if (locals.var_guard372 == 0.0) {
        let assign21880_e27324: f64 = (locals.var_t10 * 1.3);
        (assign21880_e27324, (locals.var_t10_dn0 * 1.3), (locals.var_t10_dn2 * 1.3), (locals.var_t10_dn4 * 1.3), (locals.var_t10_dn5 * 1.3), (locals.var_t10_dn6 * 1.3), (locals.var_t10_dn8 * 1.3), (locals.var_t10_dn10 * 1.3), (locals.var_t10_dn11 * 1.3), (locals.var_t10_dn12 * 1.3),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign21880_e27326;
        locals.var_t3_dn0 = assign21880_e27326_d_n0;
        locals.var_t3_dn2 = assign21880_e27326_d_n2;
        locals.var_t3_dn4 = assign21880_e27326_d_n4;
        locals.var_t3_dn5 = assign21880_e27326_d_n5;
        locals.var_t3_dn6 = assign21880_e27326_d_n6;
        locals.var_t3_dn8 = assign21880_e27326_d_n8;
        locals.var_t3_dn10 = assign21880_e27326_d_n10;
        locals.var_t3_dn11 = assign21880_e27326_d_n11;
        locals.var_t3_dn12 = assign21880_e27326_d_n12;
        locals.var_t3_rv = 0.0;

        let (assign21890_e27335, assign21890_e27335_d_n0, assign21890_e27335_d_n2, assign21890_e27335_d_n4, assign21890_e27335_d_n5, assign21890_e27335_d_n6, assign21890_e27335_d_n8, assign21890_e27335_d_n10, assign21890_e27335_d_n11, assign21890_e27335_d_n12,) = {
    if (locals.var_guard372 == 0.0) {
        let assign21890_e27331: f64 = (1.034943e-10 * locals.var_weffcv_nf);
        let assign21890_e27333: f64 = (assign21890_e27331 * locals.var_t3);
        (assign21890_e27333, (((1.034943e-10 * locals.var_weffcv_nf_dn0) * locals.var_t3) + (assign21890_e27331 * locals.var_t3_dn0)), (((1.034943e-10 * locals.var_weffcv_nf_dn2) * locals.var_t3) + (assign21890_e27331 * locals.var_t3_dn2)), (((1.034943e-10 * locals.var_weffcv_nf_dn4) * locals.var_t3) + (assign21890_e27331 * locals.var_t3_dn4)), (((1.034943e-10 * locals.var_weffcv_nf_dn5) * locals.var_t3) + (assign21890_e27331 * locals.var_t3_dn5)), (((1.034943e-10 * locals.var_weffcv_nf_dn6) * locals.var_t3) + (assign21890_e27331 * locals.var_t3_dn6)), (((1.034943e-10 * locals.var_weffcv_nf_dn8) * locals.var_t3) + (assign21890_e27331 * locals.var_t3_dn8)), (((1.034943e-10 * locals.var_weffcv_nf_dn10) * locals.var_t3) + (assign21890_e27331 * locals.var_t3_dn10)), (((1.034943e-10 * locals.var_weffcv_nf_dn11) * locals.var_t3) + (assign21890_e27331 * locals.var_t3_dn11)), (((1.034943e-10 * locals.var_weffcv_nf_dn12) * locals.var_t3) + (assign21890_e27331 * locals.var_t3_dn12)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign21890_e27335;
        locals.var_t2_dn0 = assign21890_e27335_d_n0;
        locals.var_t2_dn2 = assign21890_e27335_d_n2;
        locals.var_t2_dn4 = assign21890_e27335_d_n4;
        locals.var_t2_dn5 = assign21890_e27335_d_n5;
        locals.var_t2_dn6 = assign21890_e27335_d_n6;
        locals.var_t2_dn8 = assign21890_e27335_d_n8;
        locals.var_t2_dn10 = assign21890_e27335_d_n10;
        locals.var_t2_dn11 = assign21890_e27335_d_n11;
        locals.var_t2_dn12 = assign21890_e27335_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign21900_e27350, assign21900_e27350_d_n0, assign21900_e27350_d_n2, assign21900_e27350_d_n4, assign21900_e27350_d_n5, assign21900_e27350_d_n6, assign21900_e27350_d_n8, assign21900_e27350_d_n10, assign21900_e27350_d_n11, assign21900_e27350_d_n12,) = {
    if (locals.var_guard372 == 0.0) {
        let assign21900_e27340: f64 = (locals.var_ps0 + locals.var_vds);
        let assign21900_e27342: f64 = (assign21900_e27340 - locals.var_t1);
        let assign21900_e27344: f64 = (assign21900_e27342 / p.p45);
        let assign21900_e27346: f64 = (assign21900_e27344 - locals.var_ec);
        let assign21900_e27348: f64 = (assign21900_e27346 * locals.var_t2);
        (assign21900_e27348, ((((((locals.var_ps0_dn0 + locals.var_vds_dn0) - locals.var_t1_dn0) / p.p45) - locals.var_ec_dn0) * locals.var_t2) + (assign21900_e27346 * locals.var_t2_dn0)), ((((((locals.var_ps0_dn2 + locals.var_vds_dn2) - locals.var_t1_dn2) / p.p45) - locals.var_ec_dn2) * locals.var_t2) + (assign21900_e27346 * locals.var_t2_dn2)), ((((((locals.var_ps0_dn4 + locals.var_vds_dn4) - locals.var_t1_dn4) / p.p45) - locals.var_ec_dn4) * locals.var_t2) + (assign21900_e27346 * locals.var_t2_dn4)), ((((((locals.var_ps0_dn5 + locals.var_vds_dn5) - locals.var_t1_dn5) / p.p45) - locals.var_ec_dn5) * locals.var_t2) + (assign21900_e27346 * locals.var_t2_dn5)), ((((((locals.var_ps0_dn6 + locals.var_vds_dn6) - locals.var_t1_dn6) / p.p45) - locals.var_ec_dn6) * locals.var_t2) + (assign21900_e27346 * locals.var_t2_dn6)), ((((((locals.var_ps0_dn8 + locals.var_vds_dn8) - locals.var_t1_dn8) / p.p45) - locals.var_ec_dn8) * locals.var_t2) + (assign21900_e27346 * locals.var_t2_dn8)), ((((((locals.var_ps0_dn10 + locals.var_vds_dn10) - locals.var_t1_dn10) / p.p45) - locals.var_ec_dn10) * locals.var_t2) + (assign21900_e27346 * locals.var_t2_dn10)), ((((((locals.var_ps0_dn11 + locals.var_vds_dn11) - locals.var_t1_dn11) / p.p45) - locals.var_ec_dn11) * locals.var_t2) + (assign21900_e27346 * locals.var_t2_dn11)), ((((((locals.var_ps0_dn12 + locals.var_vds_dn12) - locals.var_t1_dn12) / p.p45) - locals.var_ec_dn12) * locals.var_t2) + (assign21900_e27346 * locals.var_t2_dn12)),)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, locals.var_qy_dn5, locals.var_qy_dn6, locals.var_qy_dn8, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn12,)
    }
};
        locals.var_qy = assign21900_e27350;
        locals.var_qy_dn0 = assign21900_e27350_d_n0;
        locals.var_qy_dn2 = assign21900_e27350_d_n2;
        locals.var_qy_dn4 = assign21900_e27350_d_n4;
        locals.var_qy_dn5 = assign21900_e27350_d_n5;
        locals.var_qy_dn6 = assign21900_e27350_d_n6;
        locals.var_qy_dn8 = assign21900_e27350_d_n8;
        locals.var_qy_dn10 = assign21900_e27350_d_n10;
        locals.var_qy_dn11 = assign21900_e27350_d_n11;
        locals.var_qy_dn12 = assign21900_e27350_d_n12;
        locals.var_qy_rv = 0.0;

        let assign21910_e27353: f64 = if p.p46 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard374 = assign21910_e27353;
        locals.var_guard374_rv = 0.0;

        let (assign21920_e27361, assign21920_e27361_d_n0, assign21920_e27361_d_n2, assign21920_e27361_d_n4, assign21920_e27361_d_n5, assign21920_e27361_d_n6, assign21920_e27361_d_n8, assign21920_e27361_d_n10, assign21920_e27361_d_n11, assign21920_e27361_d_n12,) = {
    if (locals.var_guard374 != 0.0) {
        let assign21920_e27358: f64 = (locals.var_cqyb0 * locals.var_vbs);
        let assign21920_e27359: f64 = (locals.var_qy + assign21920_e27358);
        (assign21920_e27359, (locals.var_qy_dn0 + ((locals.var_cqyb0_dn0 * locals.var_vbs) + (locals.var_cqyb0 * locals.var_vbs_dn0))), (locals.var_qy_dn2 + ((locals.var_cqyb0_dn2 * locals.var_vbs) + (locals.var_cqyb0 * locals.var_vbs_dn2))), (locals.var_qy_dn4 + ((locals.var_cqyb0_dn4 * locals.var_vbs) + (locals.var_cqyb0 * locals.var_vbs_dn4))), (locals.var_qy_dn5 + ((locals.var_cqyb0_dn5 * locals.var_vbs) + (locals.var_cqyb0 * locals.var_vbs_dn5))), (locals.var_qy_dn6 + ((locals.var_cqyb0_dn6 * locals.var_vbs) + (locals.var_cqyb0 * locals.var_vbs_dn6))), (locals.var_qy_dn8 + ((locals.var_cqyb0_dn8 * locals.var_vbs) + (locals.var_cqyb0 * locals.var_vbs_dn8))), (locals.var_qy_dn10 + ((locals.var_cqyb0_dn10 * locals.var_vbs) + (locals.var_cqyb0 * locals.var_vbs_dn10))), (locals.var_qy_dn11 + ((locals.var_cqyb0_dn11 * locals.var_vbs) + (locals.var_cqyb0 * locals.var_vbs_dn11))), (locals.var_qy_dn12 + ((locals.var_cqyb0_dn12 * locals.var_vbs) + (locals.var_cqyb0 * locals.var_vbs_dn12))),)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, locals.var_qy_dn5, locals.var_qy_dn6, locals.var_qy_dn8, locals.var_qy_dn10, locals.var_qy_dn11, locals.var_qy_dn12,)
    }
};
        locals.var_qy = assign21920_e27361;
        locals.var_qy_dn0 = assign21920_e27361_d_n0;
        locals.var_qy_dn2 = assign21920_e27361_d_n2;
        locals.var_qy_dn4 = assign21920_e27361_d_n4;
        locals.var_qy_dn5 = assign21920_e27361_d_n5;
        locals.var_qy_dn6 = assign21920_e27361_d_n6;
        locals.var_qy_dn8 = assign21920_e27361_d_n8;
        locals.var_qy_dn10 = assign21920_e27361_d_n10;
        locals.var_qy_dn11 = assign21920_e27361_d_n11;
        locals.var_qy_dn12 = assign21920_e27361_d_n12;
        locals.var_qy_rv = 0.0;

        let assign21930_e27364: f64 = if p.p14 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard375 = assign21930_e27364;
        locals.var_guard375_rv = 0.0;

        let (assign21940_e27382, assign21940_e27382_d_n0, assign21940_e27382_d_n2, assign21940_e27382_d_n4, assign21940_e27382_d_n5, assign21940_e27382_d_n6, assign21940_e27382_d_n8, assign21940_e27382_d_n10, assign21940_e27382_d_n11, assign21940_e27382_d_n12,) = {
    if (locals.var_guard375 != 0.0) {
        let assign21940_e27370: f64 = (locals.var_qgod + locals.var_qgos);
        let assign21940_e27372: f64 = (assign21940_e27370 - locals.var_qgob);
        let assign21940_e27374: f64 = (assign21940_e27372 - locals.var_qy);
        let assign21940_e27376: f64 = (assign21940_e27374 - locals.var_qovs);
        let assign21940_e27378: f64 = (assign21940_e27376 - locals.var_qovd);
        let assign21940_e27379: f64 = (locals.var_mfactor * assign21940_e27378);
        let assign21940_e27380: f64 = (locals.var_qge + assign21940_e27379);
        (assign21940_e27380, (locals.var_qge_dn0 + (locals.var_mfactor * (((((locals.var_qgod_dn0 + locals.var_qgos_dn0) - locals.var_qgob_dn0) - locals.var_qy_dn0) - locals.var_qovs_dn0) - locals.var_qovd_dn0))), (locals.var_qge_dn2 + (locals.var_mfactor * (((((locals.var_qgod_dn2 + locals.var_qgos_dn2) - locals.var_qgob_dn2) - locals.var_qy_dn2) - locals.var_qovs_dn2) - locals.var_qovd_dn2))), (locals.var_qge_dn4 + (locals.var_mfactor * ((((locals.var_qgod_dn4 + locals.var_qgos_dn4) - locals.var_qy_dn4) - locals.var_qovs_dn4) - locals.var_qovd_dn4))), (locals.var_qge_dn5 + (locals.var_mfactor * (((((locals.var_qgod_dn5 + locals.var_qgos_dn5) - locals.var_qgob_dn5) - locals.var_qy_dn5) - locals.var_qovs_dn5) - locals.var_qovd_dn5))), (locals.var_qge_dn6 + (locals.var_mfactor * (((((locals.var_qgod_dn6 + locals.var_qgos_dn6) - locals.var_qgob_dn6) - locals.var_qy_dn6) - locals.var_qovs_dn6) - locals.var_qovd_dn6))), (locals.var_qge_dn8 + (locals.var_mfactor * ((((locals.var_qgod_dn8 + locals.var_qgos_dn8) - locals.var_qy_dn8) - locals.var_qovs_dn8) - locals.var_qovd_dn8))), (locals.var_qge_dn10 + (locals.var_mfactor * ((((locals.var_qgod_dn10 + locals.var_qgos_dn10) - locals.var_qy_dn10) - locals.var_qovs_dn10) - locals.var_qovd_dn10))), (locals.var_qge_dn11 + (locals.var_mfactor * ((((locals.var_qgod_dn11 + locals.var_qgos_dn11) - locals.var_qy_dn11) - locals.var_qovs_dn11) - locals.var_qovd_dn11))), (locals.var_qge_dn12 + (locals.var_mfactor * ((((locals.var_qgod_dn12 + locals.var_qgos_dn12) - locals.var_qy_dn12) - locals.var_qovs_dn12) - locals.var_qovd_dn12))),)
    } else {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn8, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12,)
    }
};
        locals.var_qge = assign21940_e27382;
        locals.var_qge_dn0 = assign21940_e27382_d_n0;
        locals.var_qge_dn2 = assign21940_e27382_d_n2;
        locals.var_qge_dn4 = assign21940_e27382_d_n4;
        locals.var_qge_dn5 = assign21940_e27382_d_n5;
        locals.var_qge_dn6 = assign21940_e27382_d_n6;
        locals.var_qge_dn8 = assign21940_e27382_d_n8;
        locals.var_qge_dn10 = assign21940_e27382_d_n10;
        locals.var_qge_dn11 = assign21940_e27382_d_n11;
        locals.var_qge_dn12 = assign21940_e27382_d_n12;
        locals.var_qge_rv = 0.0;

        let (assign21950_e27395, assign21950_e27395_d_n0, assign21950_e27395_d_n2, assign21950_e27395_d_n4, assign21950_e27395_d_n5, assign21950_e27395_d_n6, assign21950_e27395_d_n8, assign21950_e27395_d_n10, assign21950_e27395_d_n11, assign21950_e27395_d_n12,) = {
    if (locals.var_guard375 != 0.0) {
        let assign21950_e27387: f64 = (-locals.var_qgod);
        let assign21950_e27389: f64 = (assign21950_e27387 + locals.var_qy);
        let assign21950_e27391: f64 = (assign21950_e27389 + locals.var_qbdld);
        let assign21950_e27392: f64 = (locals.var_mfactor * assign21950_e27391);
        let assign21950_e27393: f64 = (locals.var_qde + assign21950_e27392);
        (assign21950_e27393, (locals.var_qde_dn0 + (locals.var_mfactor * (((-locals.var_qgod_dn0) + locals.var_qy_dn0) + locals.var_qbdld_dn0))), (locals.var_qde_dn2 + (locals.var_mfactor * (((-locals.var_qgod_dn2) + locals.var_qy_dn2) + locals.var_qbdld_dn2))), (locals.var_qde_dn4 + (locals.var_mfactor * (((-locals.var_qgod_dn4) + locals.var_qy_dn4) + locals.var_qbdld_dn4))), (locals.var_qde_dn5 + (locals.var_mfactor * (((-locals.var_qgod_dn5) + locals.var_qy_dn5) + locals.var_qbdld_dn5))), (locals.var_qde_dn6 + (locals.var_mfactor * (((-locals.var_qgod_dn6) + locals.var_qy_dn6) + locals.var_qbdld_dn6))), (locals.var_qde_dn8 + (locals.var_mfactor * (((-locals.var_qgod_dn8) + locals.var_qy_dn8) + locals.var_qbdld_dn8))), (locals.var_qde_dn10 + (locals.var_mfactor * (((-locals.var_qgod_dn10) + locals.var_qy_dn10) + locals.var_qbdld_dn10))), (locals.var_qde_dn11 + (locals.var_mfactor * (((-locals.var_qgod_dn11) + locals.var_qy_dn11) + locals.var_qbdld_dn11))), (locals.var_qde_dn12 + (locals.var_mfactor * (((-locals.var_qgod_dn12) + locals.var_qy_dn12) + locals.var_qbdld_dn12))),)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn8, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12,)
    }
};
        locals.var_qde = assign21950_e27395;
        locals.var_qde_dn0 = assign21950_e27395_d_n0;
        locals.var_qde_dn2 = assign21950_e27395_d_n2;
        locals.var_qde_dn4 = assign21950_e27395_d_n4;
        locals.var_qde_dn5 = assign21950_e27395_d_n5;
        locals.var_qde_dn6 = assign21950_e27395_d_n6;
        locals.var_qde_dn8 = assign21950_e27395_d_n8;
        locals.var_qde_dn10 = assign21950_e27395_d_n10;
        locals.var_qde_dn11 = assign21950_e27395_d_n11;
        locals.var_qde_dn12 = assign21950_e27395_d_n12;
        locals.var_qde_rv = 0.0;

        let (assign21960_e27406, assign21960_e27406_d_n0, assign21960_e27406_d_n2, assign21960_e27406_d_n4, assign21960_e27406_d_n5, assign21960_e27406_d_n6, assign21960_e27406_d_n8, assign21960_e27406_d_n10, assign21960_e27406_d_n11, assign21960_e27406_d_n12,) = {
    if (locals.var_guard375 != 0.0) {
        let assign21960_e27400: f64 = (-locals.var_qgos);
        let assign21960_e27402: f64 = (assign21960_e27400 + locals.var_qbsld);
        let assign21960_e27403: f64 = (locals.var_mfactor * assign21960_e27402);
        let assign21960_e27404: f64 = (locals.var_qse + assign21960_e27403);
        (assign21960_e27404, (locals.var_qse_dn0 + (locals.var_mfactor * ((-locals.var_qgos_dn0) + locals.var_qbsld_dn0))), (locals.var_qse_dn2 + (locals.var_mfactor * ((-locals.var_qgos_dn2) + locals.var_qbsld_dn2))), (locals.var_qse_dn4 + (locals.var_mfactor * ((-locals.var_qgos_dn4) + locals.var_qbsld_dn4))), (locals.var_qse_dn5 + (locals.var_mfactor * ((-locals.var_qgos_dn5) + locals.var_qbsld_dn5))), (locals.var_qse_dn6 + (locals.var_mfactor * ((-locals.var_qgos_dn6) + locals.var_qbsld_dn6))), (locals.var_qse_dn8 + (locals.var_mfactor * ((-locals.var_qgos_dn8) + locals.var_qbsld_dn8))), (locals.var_qse_dn10 + (locals.var_mfactor * ((-locals.var_qgos_dn10) + locals.var_qbsld_dn10))), (locals.var_qse_dn11 + (locals.var_mfactor * ((-locals.var_qgos_dn11) + locals.var_qbsld_dn11))), (locals.var_qse_dn12 + (locals.var_mfactor * ((-locals.var_qgos_dn12) + locals.var_qbsld_dn12))),)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn8, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12,)
    }
};
        locals.var_qse = assign21960_e27406;
        locals.var_qse_dn0 = assign21960_e27406_d_n0;
        locals.var_qse_dn2 = assign21960_e27406_d_n2;
        locals.var_qse_dn4 = assign21960_e27406_d_n4;
        locals.var_qse_dn5 = assign21960_e27406_d_n5;
        locals.var_qse_dn6 = assign21960_e27406_d_n6;
        locals.var_qse_dn8 = assign21960_e27406_d_n8;
        locals.var_qse_dn10 = assign21960_e27406_d_n10;
        locals.var_qse_dn11 = assign21960_e27406_d_n11;
        locals.var_qse_dn12 = assign21960_e27406_d_n12;
        locals.var_qse_rv = 0.0;

        let assign21970_e27409: f64 = (locals.var_mfactor * locals.var_isub);
        locals.var_isube = assign21970_e27409;
        locals.var_isube_dn0 = (locals.var_mfactor * locals.var_isub_dn0);
        locals.var_isube_dn2 = (locals.var_mfactor * locals.var_isub_dn2);
        locals.var_isube_dn4 = (locals.var_mfactor * locals.var_isub_dn4);
        locals.var_isube_dn5 = (locals.var_mfactor * locals.var_isub_dn5);
        locals.var_isube_dn6 = (locals.var_mfactor * locals.var_isub_dn6);
        locals.var_isube_dn8 = (locals.var_mfactor * locals.var_isub_dn8);
        locals.var_isube_dn10 = (locals.var_mfactor * locals.var_isub_dn10);
        locals.var_isube_dn11 = (locals.var_mfactor * locals.var_isub_dn11);
        locals.var_isube_dn12 = (locals.var_mfactor * locals.var_isub_dn12);
        locals.var_isube_rv = 0.0;

        let assign22010_e27418: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard376 = assign22010_e27418;
        locals.var_guard376_rv = 0.0;

        let (assign22030_e27435, assign22030_e27435_d_n0, assign22030_e27435_d_n2, assign22030_e27435_d_n4, assign22030_e27435_d_n5, assign22030_e27435_d_n6, assign22030_e27435_d_n8, assign22030_e27435_d_n10, assign22030_e27435_d_n11, assign22030_e27435_d_n12,) = {
    if (locals.var_guard376 == 0.0) {
        let assign22030_e27433: f64 = (1.0 - locals.var_glpart1);
        (assign22030_e27433, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign22030_e27435;
        locals.var_t1_dn0 = assign22030_e27435_d_n0;
        locals.var_t1_dn2 = assign22030_e27435_d_n2;
        locals.var_t1_dn4 = assign22030_e27435_d_n4;
        locals.var_t1_dn5 = assign22030_e27435_d_n5;
        locals.var_t1_dn6 = assign22030_e27435_d_n6;
        locals.var_t1_dn8 = assign22030_e27435_d_n8;
        locals.var_t1_dn10 = assign22030_e27435_d_n10;
        locals.var_t1_dn11 = assign22030_e27435_d_n11;
        locals.var_t1_dn12 = assign22030_e27435_d_n12;
        locals.var_t1_rv = 0.0;

        let assign22050_e27449: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard377 = assign22050_e27449;
        locals.var_guard377_rv = 0.0;

        let (assign22060_e27455, assign22060_e27455_d_n0, assign22060_e27455_d_n2, assign22060_e27455_d_n4, assign22060_e27455_d_n5, assign22060_e27455_d_n6, assign22060_e27455_d_n8, assign22060_e27455_d_n10, assign22060_e27455_d_n11, assign22060_e27455_d_n12,) = {
    if (locals.var_guard377 != 0.0) {
        let assign22060_e27453: f64 = (1.0 - locals.var_glpart1);
        (assign22060_e27453, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign22060_e27455;
        locals.var_t1_dn0 = assign22060_e27455_d_n0;
        locals.var_t1_dn2 = assign22060_e27455_d_n2;
        locals.var_t1_dn4 = assign22060_e27455_d_n4;
        locals.var_t1_dn5 = assign22060_e27455_d_n5;
        locals.var_t1_dn6 = assign22060_e27455_d_n6;
        locals.var_t1_dn8 = assign22060_e27455_d_n8;
        locals.var_t1_dn10 = assign22060_e27455_d_n10;
        locals.var_t1_dn11 = assign22060_e27455_d_n11;
        locals.var_t1_dn12 = assign22060_e27455_d_n12;
        locals.var_t1_rv = 0.0;

        let assign22110_e27499: f64 = (4.0 * 1.3806226e-23);
        let assign22110_e27501: f64 = (assign22110_e27499 * locals.var_ttemp);
        let assign22110_e27503: f64 = assign22110_e27501;
        locals.var_whi_noise = assign22110_e27503;
        locals.var_whi_noise_dn4 = (assign22110_e27499 * locals.var_ttemp_dn4);
        locals.var_whi_noise_rv = 0.0;

        let assign22130_e27509: f64 = (locals.var_mfactor * locals.var_nthrml);
        locals.var_noithrml = assign22130_e27509;
        locals.var_noithrml_dn0 = (locals.var_mfactor * locals.var_nthrml_dn0);
        locals.var_noithrml_dn2 = (locals.var_mfactor * locals.var_nthrml_dn2);
        locals.var_noithrml_dn4 = (locals.var_mfactor * locals.var_nthrml_dn4);
        locals.var_noithrml_dn5 = (locals.var_mfactor * locals.var_nthrml_dn5);
        locals.var_noithrml_dn6 = (locals.var_mfactor * locals.var_nthrml_dn6);
        locals.var_noithrml_dn8 = (locals.var_mfactor * locals.var_nthrml_dn8);
        locals.var_noithrml_dn10 = (locals.var_mfactor * locals.var_nthrml_dn10);
        locals.var_noithrml_dn11 = (locals.var_mfactor * locals.var_nthrml_dn11);
        locals.var_noithrml_dn12 = (locals.var_mfactor * locals.var_nthrml_dn12);
        locals.var_noithrml_rv = 0.0;

        let assign22140_e27512: f64 = locals.var_qge_dn11;
        locals.var_cgdbd = assign22140_e27512;
        locals.var_cgdbd_dn0 = 0.0;
        locals.var_cgdbd_dn2 = 0.0;
        locals.var_cgdbd_dn4 = 0.0;
        locals.var_cgdbd_dn5 = 0.0;
        locals.var_cgdbd_dn6 = 0.0;
        locals.var_cgdbd_dn8 = 0.0;
        locals.var_cgdbd_dn10 = 0.0;
        locals.var_cgdbd_dn11 = 0.0;
        locals.var_cgdbd_dn12 = 0.0;
        locals.var_cgdbd_rv = 0.0;

        let assign22150_e27515: f64 = (p.p33 * locals.var_cgdbd);
        locals.var_cgdbd = assign22150_e27515;
        locals.var_cgdbd_dn0 = (p.p33 * locals.var_cgdbd_dn0);
        locals.var_cgdbd_dn2 = (p.p33 * locals.var_cgdbd_dn2);
        locals.var_cgdbd_dn4 = (p.p33 * locals.var_cgdbd_dn4);
        locals.var_cgdbd_dn5 = (p.p33 * locals.var_cgdbd_dn5);
        locals.var_cgdbd_dn6 = (p.p33 * locals.var_cgdbd_dn6);
        locals.var_cgdbd_dn8 = (p.p33 * locals.var_cgdbd_dn8);
        locals.var_cgdbd_dn10 = (p.p33 * locals.var_cgdbd_dn10);
        locals.var_cgdbd_dn11 = (p.p33 * locals.var_cgdbd_dn11);
        locals.var_cgdbd_dn12 = (p.p33 * locals.var_cgdbd_dn12);
        locals.var_cgdbd_rv = 0.0;

        let assign22160_e27518: f64 = locals.var_qge_dn12;
        locals.var_cgsbd = assign22160_e27518;
        locals.var_cgsbd_dn0 = 0.0;
        locals.var_cgsbd_dn2 = 0.0;
        locals.var_cgsbd_dn4 = 0.0;
        locals.var_cgsbd_dn5 = 0.0;
        locals.var_cgsbd_dn6 = 0.0;
        locals.var_cgsbd_dn8 = 0.0;
        locals.var_cgsbd_dn10 = 0.0;
        locals.var_cgsbd_dn11 = 0.0;
        locals.var_cgsbd_dn12 = 0.0;
        locals.var_cgsbd_rv = 0.0;

        let assign22170_e27521: f64 = (p.p33 * locals.var_cgsbd);
        locals.var_cgsbd = assign22170_e27521;
        locals.var_cgsbd_dn0 = (p.p33 * locals.var_cgsbd_dn0);
        locals.var_cgsbd_dn2 = (p.p33 * locals.var_cgsbd_dn2);
        locals.var_cgsbd_dn4 = (p.p33 * locals.var_cgsbd_dn4);
        locals.var_cgsbd_dn5 = (p.p33 * locals.var_cgsbd_dn5);
        locals.var_cgsbd_dn6 = (p.p33 * locals.var_cgsbd_dn6);
        locals.var_cgsbd_dn8 = (p.p33 * locals.var_cgsbd_dn8);
        locals.var_cgsbd_dn10 = (p.p33 * locals.var_cgsbd_dn10);
        locals.var_cgsbd_dn11 = (p.p33 * locals.var_cgsbd_dn11);
        locals.var_cgsbd_dn12 = (p.p33 * locals.var_cgsbd_dn12);
        locals.var_cgsbd_rv = 0.0;

        let (assign22180_e27527, assign22180_e27527_d_n0, assign22180_e27527_d_n2, assign22180_e27527_d_n4, assign22180_e27527_d_n5, assign22180_e27527_d_n6, assign22180_e27527_d_n8, assign22180_e27527_d_n10, assign22180_e27527_d_n11, assign22180_e27527_d_n12,) = {
    if (locals.var_mode > 0.0) {
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn4, locals.var_cgsbd_dn5, locals.var_cgsbd_dn6, locals.var_cgsbd_dn8, locals.var_cgsbd_dn10, locals.var_cgsbd_dn11, locals.var_cgsbd_dn12,)
    } else {
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn4, locals.var_cgdbd_dn5, locals.var_cgdbd_dn6, locals.var_cgdbd_dn8, locals.var_cgdbd_dn10, locals.var_cgdbd_dn11, locals.var_cgdbd_dn12,)
    }
};
        locals.var_cgsb = assign22180_e27527;
        locals.var_cgsb_dn0 = assign22180_e27527_d_n0;
        locals.var_cgsb_dn2 = assign22180_e27527_d_n2;
        locals.var_cgsb_dn4 = assign22180_e27527_d_n4;
        locals.var_cgsb_dn5 = assign22180_e27527_d_n5;
        locals.var_cgsb_dn6 = assign22180_e27527_d_n6;
        locals.var_cgsb_dn8 = assign22180_e27527_d_n8;
        locals.var_cgsb_dn10 = assign22180_e27527_d_n10;
        locals.var_cgsb_dn11 = assign22180_e27527_d_n11;
        locals.var_cgsb_dn12 = assign22180_e27527_d_n12;
        locals.var_cgsb_rv = 0.0;

        let assign22190_e27541: f64 = if ((((p.p20 != 0.0) && (p.p23 != 0.0)) && (locals.var_flg_ign == 1.0)) && (locals.var_flg_noqi == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard378 = assign22190_e27541;
        locals.var_guard378_rv = 0.0;

        let (assign22200_e27551, assign22200_e27551_d_n0, assign22200_e27551_d_n2, assign22200_e27551_d_n4, assign22200_e27551_d_n5, assign22200_e27551_d_n6, assign22200_e27551_d_n8, assign22200_e27551_d_n10, assign22200_e27551_d_n11, assign22200_e27551_d_n12,) = {
    if (locals.var_guard378 != 0.0) {
        let assign22200_e27545: f64 = (1e-6 * locals.var_c_fox);
        let assign22200_e27547: f64 = (assign22200_e27545 * locals.var_weffcv_nf);
        let assign22200_e27549: f64 = (assign22200_e27547 * locals.var_leff);
        (assign22200_e27549, (((((1e-6 * locals.var_c_fox_dn0) * locals.var_weffcv_nf) + (assign22200_e27545 * locals.var_weffcv_nf_dn0)) * locals.var_leff) + (assign22200_e27547 * locals.var_leff_dn0)), (((((1e-6 * locals.var_c_fox_dn2) * locals.var_weffcv_nf) + (assign22200_e27545 * locals.var_weffcv_nf_dn2)) * locals.var_leff) + (assign22200_e27547 * locals.var_leff_dn2)), (((((1e-6 * locals.var_c_fox_dn4) * locals.var_weffcv_nf) + (assign22200_e27545 * locals.var_weffcv_nf_dn4)) * locals.var_leff) + (assign22200_e27547 * locals.var_leff_dn4)), (((((1e-6 * locals.var_c_fox_dn5) * locals.var_weffcv_nf) + (assign22200_e27545 * locals.var_weffcv_nf_dn5)) * locals.var_leff) + (assign22200_e27547 * locals.var_leff_dn5)), (((((1e-6 * locals.var_c_fox_dn6) * locals.var_weffcv_nf) + (assign22200_e27545 * locals.var_weffcv_nf_dn6)) * locals.var_leff) + (assign22200_e27547 * locals.var_leff_dn6)), (((((1e-6 * locals.var_c_fox_dn8) * locals.var_weffcv_nf) + (assign22200_e27545 * locals.var_weffcv_nf_dn8)) * locals.var_leff) + (assign22200_e27547 * locals.var_leff_dn8)), (((((1e-6 * locals.var_c_fox_dn10) * locals.var_weffcv_nf) + (assign22200_e27545 * locals.var_weffcv_nf_dn10)) * locals.var_leff) + (assign22200_e27547 * locals.var_leff_dn10)), (((((1e-6 * locals.var_c_fox_dn11) * locals.var_weffcv_nf) + (assign22200_e27545 * locals.var_weffcv_nf_dn11)) * locals.var_leff) + (assign22200_e27547 * locals.var_leff_dn11)), (((((1e-6 * locals.var_c_fox_dn12) * locals.var_weffcv_nf) + (assign22200_e27545 * locals.var_weffcv_nf_dn12)) * locals.var_leff) + (assign22200_e27547 * locals.var_leff_dn12)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign22200_e27551;
        locals.var_t0_dn0 = assign22200_e27551_d_n0;
        locals.var_t0_dn2 = assign22200_e27551_d_n2;
        locals.var_t0_dn4 = assign22200_e27551_d_n4;
        locals.var_t0_dn5 = assign22200_e27551_d_n5;
        locals.var_t0_dn6 = assign22200_e27551_d_n6;
        locals.var_t0_dn8 = assign22200_e27551_d_n8;
        locals.var_t0_dn10 = assign22200_e27551_d_n10;
        locals.var_t0_dn11 = assign22200_e27551_d_n11;
        locals.var_t0_dn12 = assign22200_e27551_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign22210_e27557, assign22210_e27557_d_n0, assign22210_e27557_d_n2, assign22210_e27557_d_n4, assign22210_e27557_d_n5, assign22210_e27557_d_n6, assign22210_e27557_d_n8, assign22210_e27557_d_n10, assign22210_e27557_d_n11, assign22210_e27557_d_n12,) = {
    if (locals.var_guard378 != 0.0) {
        let assign22210_e27555: f64 = (locals.var_cgsb / locals.var_mfactor);
        (assign22210_e27555, (locals.var_cgsb_dn0 / locals.var_mfactor), (locals.var_cgsb_dn2 / locals.var_mfactor), (locals.var_cgsb_dn4 / locals.var_mfactor), (locals.var_cgsb_dn5 / locals.var_mfactor), (locals.var_cgsb_dn6 / locals.var_mfactor), (locals.var_cgsb_dn8 / locals.var_mfactor), (locals.var_cgsb_dn10 / locals.var_mfactor), (locals.var_cgsb_dn11 / locals.var_mfactor), (locals.var_cgsb_dn12 / locals.var_mfactor),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn8, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12,)
    }
};
        locals.var_t10 = assign22210_e27557;
        locals.var_t10_dn0 = assign22210_e27557_d_n0;
        locals.var_t10_dn2 = assign22210_e27557_d_n2;
        locals.var_t10_dn4 = assign22210_e27557_d_n4;
        locals.var_t10_dn5 = assign22210_e27557_d_n5;
        locals.var_t10_dn6 = assign22210_e27557_d_n6;
        locals.var_t10_dn8 = assign22210_e27557_d_n8;
        locals.var_t10_dn10 = assign22210_e27557_d_n10;
        locals.var_t10_dn11 = assign22210_e27557_d_n11;
        locals.var_t10_dn12 = assign22210_e27557_d_n12;
        locals.var_t10_rv = 0.0;

        let (assign22220_e27571, assign22220_e27571_d_n0, assign22220_e27571_d_n2, assign22220_e27571_d_n4, assign22220_e27571_d_n5, assign22220_e27571_d_n6, assign22220_e27571_d_n8, assign22220_e27571_d_n10, assign22220_e27571_d_n11, assign22220_e27571_d_n12,) = {
    if (locals.var_guard378 != 0.0) {
        let assign22220_e27561: f64 = (0.1185185185185185 * 1.6021918e-19);
        let assign22220_e27563: f64 = (assign22220_e27561 * locals.var_beta_inv);
        let assign22220_e27565: f64 = (assign22220_e27563 * locals.var_t10);
        let assign22220_e27567: f64 = (assign22220_e27565 * locals.var_t10);
        let assign22220_e27569: f64 = (assign22220_e27567 / locals.var_gds0_ign);
        (assign22220_e27569, ((((((assign22220_e27563 * locals.var_t10_dn0) * locals.var_t10) + (assign22220_e27565 * locals.var_t10_dn0)) * locals.var_gds0_ign) - (assign22220_e27567 * locals.var_gds0_ign_dn0)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign22220_e27563 * locals.var_t10_dn2) * locals.var_t10) + (assign22220_e27565 * locals.var_t10_dn2)) * locals.var_gds0_ign) - (assign22220_e27567 * locals.var_gds0_ign_dn2)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((((assign22220_e27561 * locals.var_beta_inv_dn4) * locals.var_t10) + (assign22220_e27563 * locals.var_t10_dn4)) * locals.var_t10) + (assign22220_e27565 * locals.var_t10_dn4)) * locals.var_gds0_ign) - (assign22220_e27567 * locals.var_gds0_ign_dn4)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign22220_e27563 * locals.var_t10_dn5) * locals.var_t10) + (assign22220_e27565 * locals.var_t10_dn5)) * locals.var_gds0_ign) - (assign22220_e27567 * locals.var_gds0_ign_dn5)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign22220_e27563 * locals.var_t10_dn6) * locals.var_t10) + (assign22220_e27565 * locals.var_t10_dn6)) * locals.var_gds0_ign) - (assign22220_e27567 * locals.var_gds0_ign_dn6)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign22220_e27563 * locals.var_t10_dn8) * locals.var_t10) + (assign22220_e27565 * locals.var_t10_dn8)) * locals.var_gds0_ign) - (assign22220_e27567 * locals.var_gds0_ign_dn8)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign22220_e27563 * locals.var_t10_dn10) * locals.var_t10) + (assign22220_e27565 * locals.var_t10_dn10)) * locals.var_gds0_ign) - (assign22220_e27567 * locals.var_gds0_ign_dn10)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign22220_e27563 * locals.var_t10_dn11) * locals.var_t10) + (assign22220_e27565 * locals.var_t10_dn11)) * locals.var_gds0_ign) - (assign22220_e27567 * locals.var_gds0_ign_dn11)) / (locals.var_gds0_ign * locals.var_gds0_ign)), ((((((assign22220_e27563 * locals.var_t10_dn12) * locals.var_t10) + (assign22220_e27565 * locals.var_t10_dn12)) * locals.var_gds0_ign) - (assign22220_e27567 * locals.var_gds0_ign_dn12)) / (locals.var_gds0_ign * locals.var_gds0_ign)),)
    } else {
        (locals.var_nign0, locals.var_nign0_dn0, locals.var_nign0_dn2, locals.var_nign0_dn4, locals.var_nign0_dn5, locals.var_nign0_dn6, locals.var_nign0_dn8, locals.var_nign0_dn10, locals.var_nign0_dn11, locals.var_nign0_dn12,)
    }
};
        locals.var_nign0 = assign22220_e27571;
        locals.var_nign0_dn0 = assign22220_e27571_d_n0;
        locals.var_nign0_dn2 = assign22220_e27571_d_n2;
        locals.var_nign0_dn4 = assign22220_e27571_d_n4;
        locals.var_nign0_dn5 = assign22220_e27571_d_n5;
        locals.var_nign0_dn6 = assign22220_e27571_d_n6;
        locals.var_nign0_dn8 = assign22220_e27571_d_n8;
        locals.var_nign0_dn10 = assign22220_e27571_d_n10;
        locals.var_nign0_dn11 = assign22220_e27571_d_n11;
        locals.var_nign0_dn12 = assign22220_e27571_d_n12;
        locals.var_nign0_rv = 0.0;

        let assign22230_e27575: f64 = (10.0 * 2.220446049250313e-16);
        let assign22230_e27580: f64 = (10.0 * 2.220446049250313e-16);
        let assign22230_e27582: f64 = if ((locals.var_kusai00l > assign22230_e27575) && (locals.var_vds > assign22230_e27580)) { 1.0 } else { 0.0 };
        locals.var_guard379 = assign22230_e27582;
        locals.var_guard379_rv = 0.0;

        let (assign22240_e27590, assign22240_e27590_d_n0, assign22240_e27590_d_n2, assign22240_e27590_d_n4, assign22240_e27590_d_n5, assign22240_e27590_d_n6, assign22240_e27590_d_n8, assign22240_e27590_d_n10, assign22240_e27590_d_n11, assign22240_e27590_d_n12,) = {
    if ((locals.var_guard378 != 0.0) && (locals.var_guard379 != 0.0)) {
        let assign22240_e27588: f64 = (locals.var_muun / locals.var_mu);
        (assign22240_e27588, (((locals.var_muun_dn0 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn0)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn2 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn2)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn4 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn4)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn5 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn5)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn6 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn6)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn8 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn8)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn10 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn10)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn11 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn11)) / (locals.var_mu * locals.var_mu)), (((locals.var_muun_dn12 * locals.var_mu) - (locals.var_muun * locals.var_mu_dn12)) / (locals.var_mu * locals.var_mu)),)
    } else {
        (locals.var_mumoda, locals.var_mumoda_dn0, locals.var_mumoda_dn2, locals.var_mumoda_dn4, locals.var_mumoda_dn5, locals.var_mumoda_dn6, locals.var_mumoda_dn8, locals.var_mumoda_dn10, locals.var_mumoda_dn11, locals.var_mumoda_dn12,)
    }
};
        locals.var_mumoda = assign22240_e27590;
        locals.var_mumoda_dn0 = assign22240_e27590_d_n0;
        locals.var_mumoda_dn2 = assign22240_e27590_d_n2;
        locals.var_mumoda_dn4 = assign22240_e27590_d_n4;
        locals.var_mumoda_dn5 = assign22240_e27590_d_n5;
        locals.var_mumoda_dn6 = assign22240_e27590_d_n6;
        locals.var_mumoda_dn8 = assign22240_e27590_d_n8;
        locals.var_mumoda_dn10 = assign22240_e27590_d_n10;
        locals.var_mumoda_dn11 = assign22240_e27590_d_n11;
        locals.var_mumoda_dn12 = assign22240_e27590_d_n12;
        locals.var_mumoda_rv = 0.0;

        let (assign22250_e27602, assign22250_e27602_d_n0, assign22250_e27602_d_n2, assign22250_e27602_d_n4, assign22250_e27602_d_n5, assign22250_e27602_d_n6, assign22250_e27602_d_n8, assign22250_e27602_d_n10, assign22250_e27602_d_n11, assign22250_e27602_d_n12,) = {
    if ((locals.var_guard378 != 0.0) && (locals.var_guard379 != 0.0)) {
        let assign22250_e27596: f64 = (locals.var_muun / locals.var_mud_hoso);
        let assign22250_e27598: f64 = (assign22250_e27596 - locals.var_mumoda);
        let assign22250_e27600: f64 = (assign22250_e27598 / locals.var_vds);
        (assign22250_e27600, (((((((locals.var_muun_dn0 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn0)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn0) * locals.var_vds) - (assign22250_e27598 * locals.var_vds_dn0)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn2 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn2)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn2) * locals.var_vds) - (assign22250_e27598 * locals.var_vds_dn2)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn4 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn4)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn4) * locals.var_vds) - (assign22250_e27598 * locals.var_vds_dn4)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn5 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn5)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn5) * locals.var_vds) - (assign22250_e27598 * locals.var_vds_dn5)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn6 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn6)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn6) * locals.var_vds) - (assign22250_e27598 * locals.var_vds_dn6)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn8 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn8)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn8) * locals.var_vds) - (assign22250_e27598 * locals.var_vds_dn8)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn10 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn10)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn10) * locals.var_vds) - (assign22250_e27598 * locals.var_vds_dn10)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn11 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn11)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn11) * locals.var_vds) - (assign22250_e27598 * locals.var_vds_dn11)) / (locals.var_vds * locals.var_vds)), (((((((locals.var_muun_dn12 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn12)) / (locals.var_mud_hoso * locals.var_mud_hoso)) - locals.var_mumoda_dn12) * locals.var_vds) - (assign22250_e27598 * locals.var_vds_dn12)) / (locals.var_vds * locals.var_vds)),)
    } else {
        (locals.var_mumodb, locals.var_mumodb_dn0, locals.var_mumodb_dn2, locals.var_mumodb_dn4, locals.var_mumodb_dn5, locals.var_mumodb_dn6, locals.var_mumodb_dn8, locals.var_mumodb_dn10, locals.var_mumodb_dn11, locals.var_mumodb_dn12,)
    }
};
        locals.var_mumodb = assign22250_e27602;
        locals.var_mumodb_dn0 = assign22250_e27602_d_n0;
        locals.var_mumodb_dn2 = assign22250_e27602_d_n2;
        locals.var_mumodb_dn4 = assign22250_e27602_d_n4;
        locals.var_mumodb_dn5 = assign22250_e27602_d_n5;
        locals.var_mumodb_dn6 = assign22250_e27602_d_n6;
        locals.var_mumodb_dn8 = assign22250_e27602_d_n8;
        locals.var_mumodb_dn10 = assign22250_e27602_d_n10;
        locals.var_mumodb_dn11 = assign22250_e27602_d_n11;
        locals.var_mumodb_dn12 = assign22250_e27602_d_n12;
        locals.var_mumodb_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_90(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let (assign22260_e27624, assign22260_e27624_d_n0, assign22260_e27624_d_n2, assign22260_e27624_d_n4, assign22260_e27624_d_n5, assign22260_e27624_d_n6, assign22260_e27624_d_n8, assign22260_e27624_d_n10, assign22260_e27624_d_n11, assign22260_e27624_d_n12,) = {
    if ((locals.var_guard378 != 0.0) && (locals.var_guard379 != 0.0)) {
        let assign22260_e27609: f64 = (0.6666666666666667 * locals.var_mumodb);
        let assign22260_e27613: f64 = (locals.var_vgvt * locals.var_sqrtkusail);
        let assign22260_e27614: f64 = (locals.var_kusai00 + assign22260_e27613);
        let assign22260_e27616: f64 = (assign22260_e27614 + locals.var_kusail);
        let assign22260_e27617: f64 = (assign22260_e27609 * assign22260_e27616);
        let assign22260_e27620: f64 = (locals.var_vgvt + locals.var_sqrtkusail);
        let assign22260_e27621: f64 = (assign22260_e27617 / assign22260_e27620);
        let assign22260_e27622: f64 = (locals.var_mumoda + assign22260_e27621);
        (assign22260_e27622, (locals.var_mumoda_dn0 + ((((((0.6666666666666667 * locals.var_mumodb_dn0) * assign22260_e27616) + (assign22260_e27609 * ((locals.var_kusai00_dn0 + ((locals.var_vgvt_dn0 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn0))) + locals.var_kusail_dn0))) * assign22260_e27620) - (assign22260_e27617 * (locals.var_vgvt_dn0 + locals.var_sqrtkusail_dn0))) / (assign22260_e27620 * assign22260_e27620))), (locals.var_mumoda_dn2 + ((((((0.6666666666666667 * locals.var_mumodb_dn2) * assign22260_e27616) + (assign22260_e27609 * ((locals.var_kusai00_dn2 + ((locals.var_vgvt_dn2 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn2))) + locals.var_kusail_dn2))) * assign22260_e27620) - (assign22260_e27617 * (locals.var_vgvt_dn2 + locals.var_sqrtkusail_dn2))) / (assign22260_e27620 * assign22260_e27620))), (locals.var_mumoda_dn4 + ((((((0.6666666666666667 * locals.var_mumodb_dn4) * assign22260_e27616) + (assign22260_e27609 * ((locals.var_kusai00_dn4 + ((locals.var_vgvt_dn4 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn4))) + locals.var_kusail_dn4))) * assign22260_e27620) - (assign22260_e27617 * (locals.var_vgvt_dn4 + locals.var_sqrtkusail_dn4))) / (assign22260_e27620 * assign22260_e27620))), (locals.var_mumoda_dn5 + ((((((0.6666666666666667 * locals.var_mumodb_dn5) * assign22260_e27616) + (assign22260_e27609 * ((locals.var_kusai00_dn5 + ((locals.var_vgvt_dn5 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn5))) + locals.var_kusail_dn5))) * assign22260_e27620) - (assign22260_e27617 * (locals.var_vgvt_dn5 + locals.var_sqrtkusail_dn5))) / (assign22260_e27620 * assign22260_e27620))), (locals.var_mumoda_dn6 + ((((((0.6666666666666667 * locals.var_mumodb_dn6) * assign22260_e27616) + (assign22260_e27609 * ((locals.var_kusai00_dn6 + ((locals.var_vgvt_dn6 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn6))) + locals.var_kusail_dn6))) * assign22260_e27620) - (assign22260_e27617 * (locals.var_vgvt_dn6 + locals.var_sqrtkusail_dn6))) / (assign22260_e27620 * assign22260_e27620))), (locals.var_mumoda_dn8 + ((((((0.6666666666666667 * locals.var_mumodb_dn8) * assign22260_e27616) + (assign22260_e27609 * ((locals.var_kusai00_dn8 + ((locals.var_vgvt_dn8 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn8))) + locals.var_kusail_dn8))) * assign22260_e27620) - (assign22260_e27617 * (locals.var_vgvt_dn8 + locals.var_sqrtkusail_dn8))) / (assign22260_e27620 * assign22260_e27620))), (locals.var_mumoda_dn10 + ((((((0.6666666666666667 * locals.var_mumodb_dn10) * assign22260_e27616) + (assign22260_e27609 * ((locals.var_kusai00_dn10 + ((locals.var_vgvt_dn10 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn10))) + locals.var_kusail_dn10))) * assign22260_e27620) - (assign22260_e27617 * (locals.var_vgvt_dn10 + locals.var_sqrtkusail_dn10))) / (assign22260_e27620 * assign22260_e27620))), (locals.var_mumoda_dn11 + ((((((0.6666666666666667 * locals.var_mumodb_dn11) * assign22260_e27616) + (assign22260_e27609 * ((locals.var_kusai00_dn11 + ((locals.var_vgvt_dn11 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn11))) + locals.var_kusail_dn11))) * assign22260_e27620) - (assign22260_e27617 * (locals.var_vgvt_dn11 + locals.var_sqrtkusail_dn11))) / (assign22260_e27620 * assign22260_e27620))), (locals.var_mumoda_dn12 + ((((((0.6666666666666667 * locals.var_mumodb_dn12) * assign22260_e27616) + (assign22260_e27609 * ((locals.var_kusai00_dn12 + ((locals.var_vgvt_dn12 * locals.var_sqrtkusail) + (locals.var_vgvt * locals.var_sqrtkusail_dn12))) + locals.var_kusail_dn12))) * assign22260_e27620) - (assign22260_e27617 * (locals.var_vgvt_dn12 + locals.var_sqrtkusail_dn12))) / (assign22260_e27620 * assign22260_e27620))),)
    } else {
        (locals.var_correct_w1, locals.var_correct_w1_dn0, locals.var_correct_w1_dn2, locals.var_correct_w1_dn4, locals.var_correct_w1_dn5, locals.var_correct_w1_dn6, locals.var_correct_w1_dn8, locals.var_correct_w1_dn10, locals.var_correct_w1_dn11, locals.var_correct_w1_dn12,)
    }
};
        locals.var_correct_w1 = assign22260_e27624;
        locals.var_correct_w1_dn0 = assign22260_e27624_d_n0;
        locals.var_correct_w1_dn2 = assign22260_e27624_d_n2;
        locals.var_correct_w1_dn4 = assign22260_e27624_d_n4;
        locals.var_correct_w1_dn5 = assign22260_e27624_d_n5;
        locals.var_correct_w1_dn6 = assign22260_e27624_d_n6;
        locals.var_correct_w1_dn8 = assign22260_e27624_d_n8;
        locals.var_correct_w1_dn10 = assign22260_e27624_d_n10;
        locals.var_correct_w1_dn11 = assign22260_e27624_d_n11;
        locals.var_correct_w1_dn12 = assign22260_e27624_d_n12;
        locals.var_correct_w1_rv = 0.0;

        let (assign22270_e27633, assign22270_e27633_d_n0, assign22270_e27633_d_n2, assign22270_e27633_d_n4, assign22270_e27633_d_n5, assign22270_e27633_d_n6, assign22270_e27633_d_n8, assign22270_e27633_d_n10, assign22270_e27633_d_n11, assign22270_e27633_d_n12,) = {
    if ((locals.var_guard378 != 0.0) && (locals.var_guard379 == 0.0)) {
        let assign22270_e27631: f64 = (locals.var_muun / locals.var_mud_hoso);
        (assign22270_e27631, (((locals.var_muun_dn0 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn0)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn2 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn2)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn4 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn4)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn5 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn5)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn6 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn6)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn8 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn8)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn10 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn10)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn11 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn11)) / (locals.var_mud_hoso * locals.var_mud_hoso)), (((locals.var_muun_dn12 * locals.var_mud_hoso) - (locals.var_muun * locals.var_mud_hoso_dn12)) / (locals.var_mud_hoso * locals.var_mud_hoso)),)
    } else {
        (locals.var_correct_w1, locals.var_correct_w1_dn0, locals.var_correct_w1_dn2, locals.var_correct_w1_dn4, locals.var_correct_w1_dn5, locals.var_correct_w1_dn6, locals.var_correct_w1_dn8, locals.var_correct_w1_dn10, locals.var_correct_w1_dn11, locals.var_correct_w1_dn12,)
    }
};
        locals.var_correct_w1 = assign22270_e27633;
        locals.var_correct_w1_dn0 = assign22270_e27633_d_n0;
        locals.var_correct_w1_dn2 = assign22270_e27633_d_n2;
        locals.var_correct_w1_dn4 = assign22270_e27633_d_n4;
        locals.var_correct_w1_dn5 = assign22270_e27633_d_n5;
        locals.var_correct_w1_dn6 = assign22270_e27633_d_n6;
        locals.var_correct_w1_dn8 = assign22270_e27633_d_n8;
        locals.var_correct_w1_dn10 = assign22270_e27633_d_n10;
        locals.var_correct_w1_dn11 = assign22270_e27633_d_n11;
        locals.var_correct_w1_dn12 = assign22270_e27633_d_n12;
        locals.var_correct_w1_rv = 0.0;

        let (assign22280_e27643, assign22280_e27643_d_n0, assign22280_e27643_d_n2, assign22280_e27643_d_n4, assign22280_e27643_d_n5, assign22280_e27643_d_n6, assign22280_e27643_d_n8, assign22280_e27643_d_n10, assign22280_e27643_d_n11, assign22280_e27643_d_n12,) = {
    if (locals.var_guard378 != 0.0) {
        let assign22280_e27637: f64 = (locals.var_mfactor * locals.var_nign0);
        let assign22280_e27639: f64 = (assign22280_e27637 * locals.var_kusai_ig);
        let assign22280_e27641: f64 = (assign22280_e27639 * locals.var_correct_w1);
        (assign22280_e27641, (((((locals.var_mfactor * locals.var_nign0_dn0) * locals.var_kusai_ig) + (assign22280_e27637 * locals.var_kusai_ig_dn0)) * locals.var_correct_w1) + (assign22280_e27639 * locals.var_correct_w1_dn0)), (((((locals.var_mfactor * locals.var_nign0_dn2) * locals.var_kusai_ig) + (assign22280_e27637 * locals.var_kusai_ig_dn2)) * locals.var_correct_w1) + (assign22280_e27639 * locals.var_correct_w1_dn2)), (((((locals.var_mfactor * locals.var_nign0_dn4) * locals.var_kusai_ig) + (assign22280_e27637 * locals.var_kusai_ig_dn4)) * locals.var_correct_w1) + (assign22280_e27639 * locals.var_correct_w1_dn4)), (((((locals.var_mfactor * locals.var_nign0_dn5) * locals.var_kusai_ig) + (assign22280_e27637 * locals.var_kusai_ig_dn5)) * locals.var_correct_w1) + (assign22280_e27639 * locals.var_correct_w1_dn5)), (((((locals.var_mfactor * locals.var_nign0_dn6) * locals.var_kusai_ig) + (assign22280_e27637 * locals.var_kusai_ig_dn6)) * locals.var_correct_w1) + (assign22280_e27639 * locals.var_correct_w1_dn6)), (((((locals.var_mfactor * locals.var_nign0_dn8) * locals.var_kusai_ig) + (assign22280_e27637 * locals.var_kusai_ig_dn8)) * locals.var_correct_w1) + (assign22280_e27639 * locals.var_correct_w1_dn8)), (((((locals.var_mfactor * locals.var_nign0_dn10) * locals.var_kusai_ig) + (assign22280_e27637 * locals.var_kusai_ig_dn10)) * locals.var_correct_w1) + (assign22280_e27639 * locals.var_correct_w1_dn10)), (((((locals.var_mfactor * locals.var_nign0_dn11) * locals.var_kusai_ig) + (assign22280_e27637 * locals.var_kusai_ig_dn11)) * locals.var_correct_w1) + (assign22280_e27639 * locals.var_correct_w1_dn11)), (((((locals.var_mfactor * locals.var_nign0_dn12) * locals.var_kusai_ig) + (assign22280_e27637 * locals.var_kusai_ig_dn12)) * locals.var_correct_w1) + (assign22280_e27639 * locals.var_correct_w1_dn12)),)
    } else {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn8, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12,)
    }
};
        locals.var_noiigate = assign22280_e27643;
        locals.var_noiigate_dn0 = assign22280_e27643_d_n0;
        locals.var_noiigate_dn2 = assign22280_e27643_d_n2;
        locals.var_noiigate_dn4 = assign22280_e27643_d_n4;
        locals.var_noiigate_dn5 = assign22280_e27643_d_n5;
        locals.var_noiigate_dn6 = assign22280_e27643_d_n6;
        locals.var_noiigate_dn8 = assign22280_e27643_d_n8;
        locals.var_noiigate_dn10 = assign22280_e27643_d_n10;
        locals.var_noiigate_dn11 = assign22280_e27643_d_n11;
        locals.var_noiigate_dn12 = assign22280_e27643_d_n12;
        locals.var_noiigate_rv = 0.0;

        let (assign22300_e27656, assign22300_e27656_d_n0, assign22300_e27656_d_n2, assign22300_e27656_d_n4, assign22300_e27656_d_n5, assign22300_e27656_d_n6, assign22300_e27656_d_n8, assign22300_e27656_d_n10, assign22300_e27656_d_n11, assign22300_e27656_d_n12,) = {
    if (locals.var_guard378 != 0.0) {
        let (assign22300_e27654, assign22300_e27654_d_n0, assign22300_e27654_d_n2, assign22300_e27654_d_n4, assign22300_e27654_d_n5, assign22300_e27654_d_n6, assign22300_e27654_d_n8, assign22300_e27654_d_n10, assign22300_e27654_d_n11, assign22300_e27654_d_n12,) = {
            if (locals.var_noiigate < 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn8, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12,)
            }
        };
        (assign22300_e27654, assign22300_e27654_d_n0, assign22300_e27654_d_n2, assign22300_e27654_d_n4, assign22300_e27654_d_n5, assign22300_e27654_d_n6, assign22300_e27654_d_n8, assign22300_e27654_d_n10, assign22300_e27654_d_n11, assign22300_e27654_d_n12,)
    } else {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn8, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12,)
    }
};
        locals.var_noiigate = assign22300_e27656;
        locals.var_noiigate_dn0 = assign22300_e27656_d_n0;
        locals.var_noiigate_dn2 = assign22300_e27656_d_n2;
        locals.var_noiigate_dn4 = assign22300_e27656_d_n4;
        locals.var_noiigate_dn5 = assign22300_e27656_d_n5;
        locals.var_noiigate_dn6 = assign22300_e27656_d_n6;
        locals.var_noiigate_dn8 = assign22300_e27656_d_n8;
        locals.var_noiigate_dn10 = assign22300_e27656_d_n10;
        locals.var_noiigate_dn11 = assign22300_e27656_d_n11;
        locals.var_noiigate_dn12 = assign22300_e27656_d_n12;
        locals.var_noiigate_rv = 0.0;

        let (assign22310_e27666, assign22310_e27666_d_n0, assign22310_e27666_d_n2, assign22310_e27666_d_n4, assign22310_e27666_d_n5, assign22310_e27666_d_n6, assign22310_e27666_d_n8, assign22310_e27666_d_n10, assign22310_e27666_d_n11, assign22310_e27666_d_n12,) = {
    if (locals.var_guard378 != 0.0) {
        let assign22310_e27659: f64 = (-locals.var_t10);
        let (assign22310_e27664, assign22310_e27664_d_n0, assign22310_e27664_d_n2, assign22310_e27664_d_n4, assign22310_e27664_d_n5, assign22310_e27664_d_n6, assign22310_e27664_d_n8, assign22310_e27664_d_n10, assign22310_e27664_d_n11, assign22310_e27664_d_n12,) = {
            if (assign22310_e27659 > locals.var_t0) {
                (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn8, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign22310_e27664, assign22310_e27664_d_n0, assign22310_e27664_d_n2, assign22310_e27664_d_n4, assign22310_e27664_d_n5, assign22310_e27664_d_n6, assign22310_e27664_d_n8, assign22310_e27664_d_n10, assign22310_e27664_d_n11, assign22310_e27664_d_n12,)
    } else {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn8, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12,)
    }
};
        locals.var_noiigate = assign22310_e27666;
        locals.var_noiigate_dn0 = assign22310_e27666_d_n0;
        locals.var_noiigate_dn2 = assign22310_e27666_d_n2;
        locals.var_noiigate_dn4 = assign22310_e27666_d_n4;
        locals.var_noiigate_dn5 = assign22310_e27666_d_n5;
        locals.var_noiigate_dn6 = assign22310_e27666_d_n6;
        locals.var_noiigate_dn8 = assign22310_e27666_d_n8;
        locals.var_noiigate_dn10 = assign22310_e27666_d_n10;
        locals.var_noiigate_dn11 = assign22310_e27666_d_n11;
        locals.var_noiigate_dn12 = assign22310_e27666_d_n12;
        locals.var_noiigate_rv = 0.0;

        let (assign22330_e27681, assign22330_e27681_d_n0, assign22330_e27681_d_n2, assign22330_e27681_d_n4, assign22330_e27681_d_n5, assign22330_e27681_d_n6, assign22330_e27681_d_n8, assign22330_e27681_d_n10, assign22330_e27681_d_n11, assign22330_e27681_d_n12,) = {
    if (locals.var_guard378 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_noiigate, locals.var_noiigate_dn0, locals.var_noiigate_dn2, locals.var_noiigate_dn4, locals.var_noiigate_dn5, locals.var_noiigate_dn6, locals.var_noiigate_dn8, locals.var_noiigate_dn10, locals.var_noiigate_dn11, locals.var_noiigate_dn12,)
    }
};
        locals.var_noiigate = assign22330_e27681;
        locals.var_noiigate_dn0 = assign22330_e27681_d_n0;
        locals.var_noiigate_dn2 = assign22330_e27681_d_n2;
        locals.var_noiigate_dn4 = assign22330_e27681_d_n4;
        locals.var_noiigate_dn5 = assign22330_e27681_d_n5;
        locals.var_noiigate_dn6 = assign22330_e27681_d_n6;
        locals.var_noiigate_dn8 = assign22330_e27681_d_n8;
        locals.var_noiigate_dn10 = assign22330_e27681_d_n10;
        locals.var_noiigate_dn11 = assign22330_e27681_d_n11;
        locals.var_noiigate_dn12 = assign22330_e27681_d_n12;
        locals.var_noiigate_rv = 0.0;

        let assign22350_e27689: f64 = (locals.var_whi_noise * locals.var_noithrml);
        locals.var_sid = assign22350_e27689;
        locals.var_sid_dn0 = (locals.var_whi_noise * locals.var_noithrml_dn0);
        locals.var_sid_dn2 = (locals.var_whi_noise * locals.var_noithrml_dn2);
        locals.var_sid_dn4 = ((locals.var_whi_noise_dn4 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn4));
        locals.var_sid_dn5 = (locals.var_whi_noise * locals.var_noithrml_dn5);
        locals.var_sid_dn6 = (locals.var_whi_noise * locals.var_noithrml_dn6);
        locals.var_sid_dn8 = (locals.var_whi_noise * locals.var_noithrml_dn8);
        locals.var_sid_dn10 = (locals.var_whi_noise * locals.var_noithrml_dn10);
        locals.var_sid_dn11 = (locals.var_whi_noise * locals.var_noithrml_dn11);
        locals.var_sid_dn12 = (locals.var_whi_noise * locals.var_noithrml_dn12);
        locals.var_sid_rv = 0.0;

        let (assign22370_e27703, assign22370_e27703_d_n0, assign22370_e27703_d_n2, assign22370_e27703_d_n4, assign22370_e27703_d_n5, assign22370_e27703_d_n6, assign22370_e27703_d_n8, assign22370_e27703_d_n10, assign22370_e27703_d_n11, assign22370_e27703_d_n12,) = {
    if ((locals.var_sid > 0.0) && (locals.var_noiigate > 0.0)) {
        let assign22370_e27700: f64 = (locals.var_noiigate / locals.var_sid);
        let assign22370_e27701: f64 = (assign22370_e27700).sqrt();
        (assign22370_e27701, ((((locals.var_noiigate_dn0 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn0)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign22370_e27701)), ((((locals.var_noiigate_dn2 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn2)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign22370_e27701)), ((((locals.var_noiigate_dn4 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn4)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign22370_e27701)), ((((locals.var_noiigate_dn5 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn5)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign22370_e27701)), ((((locals.var_noiigate_dn6 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn6)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign22370_e27701)), ((((locals.var_noiigate_dn8 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn8)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign22370_e27701)), ((((locals.var_noiigate_dn10 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn10)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign22370_e27701)), ((((locals.var_noiigate_dn11 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn11)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign22370_e27701)), ((((locals.var_noiigate_dn12 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn12)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign22370_e27701)),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        locals.var_sigrat = assign22370_e27703;
        locals.var_sigrat_dn0 = assign22370_e27703_d_n0;
        locals.var_sigrat_dn2 = assign22370_e27703_d_n2;
        locals.var_sigrat_dn4 = assign22370_e27703_d_n4;
        locals.var_sigrat_dn5 = assign22370_e27703_d_n5;
        locals.var_sigrat_dn6 = assign22370_e27703_d_n6;
        locals.var_sigrat_dn8 = assign22370_e27703_d_n8;
        locals.var_sigrat_dn10 = assign22370_e27703_d_n10;
        locals.var_sigrat_dn11 = assign22370_e27703_d_n11;
        locals.var_sigrat_dn12 = assign22370_e27703_d_n12;
        locals.var_sigrat_rv = 0.0;

        let (assign22380_e27715, assign22380_e27715_d_n0, assign22380_e27715_d_n2, assign22380_e27715_d_n4, assign22380_e27715_d_n5, assign22380_e27715_d_n6, assign22380_e27715_d_n8, assign22380_e27715_d_n10, assign22380_e27715_d_n11, assign22380_e27715_d_n12,) = {
    if (locals.var_mode > 0.0) {
        let assign22380_e27710: f64 = (1.0 - locals.var_qdrat);
        let assign22380_e27711: f64 = (locals.var_sigrat * assign22380_e27710);
        (assign22380_e27711, (locals.var_sigrat_dn0 * assign22380_e27710), (locals.var_sigrat_dn2 * assign22380_e27710), (locals.var_sigrat_dn4 * assign22380_e27710), (locals.var_sigrat_dn5 * assign22380_e27710), (locals.var_sigrat_dn6 * assign22380_e27710), (locals.var_sigrat_dn8 * assign22380_e27710), (locals.var_sigrat_dn10 * assign22380_e27710), (locals.var_sigrat_dn11 * assign22380_e27710), (locals.var_sigrat_dn12 * assign22380_e27710),)
    } else {
        let assign22380_e27714: f64 = (locals.var_sigrat * locals.var_qdrat);
        (assign22380_e27714, (locals.var_sigrat_dn0 * locals.var_qdrat), (locals.var_sigrat_dn2 * locals.var_qdrat), (locals.var_sigrat_dn4 * locals.var_qdrat), (locals.var_sigrat_dn5 * locals.var_qdrat), (locals.var_sigrat_dn6 * locals.var_qdrat), (locals.var_sigrat_dn8 * locals.var_qdrat), (locals.var_sigrat_dn10 * locals.var_qdrat), (locals.var_sigrat_dn11 * locals.var_qdrat), (locals.var_sigrat_dn12 * locals.var_qdrat),)
    }
};
        locals.var_sigrat_s = assign22380_e27715;
        locals.var_sigrat_s_dn0 = assign22380_e27715_d_n0;
        locals.var_sigrat_s_dn2 = assign22380_e27715_d_n2;
        locals.var_sigrat_s_dn4 = assign22380_e27715_d_n4;
        locals.var_sigrat_s_dn5 = assign22380_e27715_d_n5;
        locals.var_sigrat_s_dn6 = assign22380_e27715_d_n6;
        locals.var_sigrat_s_dn8 = assign22380_e27715_d_n8;
        locals.var_sigrat_s_dn10 = assign22380_e27715_d_n10;
        locals.var_sigrat_s_dn11 = assign22380_e27715_d_n11;
        locals.var_sigrat_s_dn12 = assign22380_e27715_d_n12;
        locals.var_sigrat_s_rv = 0.0;

        let (assign22390_e27727, assign22390_e27727_d_n0, assign22390_e27727_d_n2, assign22390_e27727_d_n4, assign22390_e27727_d_n5, assign22390_e27727_d_n6, assign22390_e27727_d_n8, assign22390_e27727_d_n10, assign22390_e27727_d_n11, assign22390_e27727_d_n12,) = {
    if (locals.var_mode > 0.0) {
        let assign22390_e27721: f64 = (locals.var_sigrat * locals.var_qdrat);
        (assign22390_e27721, (locals.var_sigrat_dn0 * locals.var_qdrat), (locals.var_sigrat_dn2 * locals.var_qdrat), (locals.var_sigrat_dn4 * locals.var_qdrat), (locals.var_sigrat_dn5 * locals.var_qdrat), (locals.var_sigrat_dn6 * locals.var_qdrat), (locals.var_sigrat_dn8 * locals.var_qdrat), (locals.var_sigrat_dn10 * locals.var_qdrat), (locals.var_sigrat_dn11 * locals.var_qdrat), (locals.var_sigrat_dn12 * locals.var_qdrat),)
    } else {
        let assign22390_e27725: f64 = (1.0 - locals.var_qdrat);
        let assign22390_e27726: f64 = (locals.var_sigrat * assign22390_e27725);
        (assign22390_e27726, (locals.var_sigrat_dn0 * assign22390_e27725), (locals.var_sigrat_dn2 * assign22390_e27725), (locals.var_sigrat_dn4 * assign22390_e27725), (locals.var_sigrat_dn5 * assign22390_e27725), (locals.var_sigrat_dn6 * assign22390_e27725), (locals.var_sigrat_dn8 * assign22390_e27725), (locals.var_sigrat_dn10 * assign22390_e27725), (locals.var_sigrat_dn11 * assign22390_e27725), (locals.var_sigrat_dn12 * assign22390_e27725),)
    }
};
        locals.var_sigrat_d = assign22390_e27727;
        locals.var_sigrat_d_dn0 = assign22390_e27727_d_n0;
        locals.var_sigrat_d_dn2 = assign22390_e27727_d_n2;
        locals.var_sigrat_d_dn4 = assign22390_e27727_d_n4;
        locals.var_sigrat_d_dn5 = assign22390_e27727_d_n5;
        locals.var_sigrat_d_dn6 = assign22390_e27727_d_n6;
        locals.var_sigrat_d_dn8 = assign22390_e27727_d_n8;
        locals.var_sigrat_d_dn10 = assign22390_e27727_d_n10;
        locals.var_sigrat_d_dn11 = assign22390_e27727_d_n11;
        locals.var_sigrat_d_dn12 = assign22390_e27727_d_n12;
        locals.var_sigrat_d_rv = 0.0;

        let assign22440_e27734: f64 = if p.p312 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard380 = assign22440_e27734;
        locals.var_guard380_rv = 0.0;

        let (assign22460_e27744,) = {
    if (locals.var_guard380 != 0.0) {
        (p.p317,)
    } else {
        (locals.var_mks_rdrmue,)
    }
};
        locals.var_mks_rdrmue = assign22460_e27744;
        locals.var_mks_rdrmue_rv = 0.0;

        let (assign22470_e27748,) = {
    if (locals.var_guard380 != 0.0) {
        (p.p319,)
    } else {
        (locals.var_mks_rdrvmax,)
    }
};
        locals.var_mks_rdrvmax = assign22470_e27748;
        locals.var_mks_rdrvmax_rv = 0.0;

        let (assign22480_e27752, assign22480_e27752_d_n4,) = {
    if (locals.var_guard380 != 0.0) {
        (p.p324, 0.0,)
    } else {
        (locals.var_rrdrbb, locals.var_rrdrbb_dn4,)
    }
};
        locals.var_rrdrbb = assign22480_e27752;
        locals.var_rrdrbb_dn4 = assign22480_e27752_d_n4;
        locals.var_rrdrbb_rv = 0.0;

        let (assign22500_e27767,) = {
    if (locals.var_guard380 != 0.0) {
        (p.p311,)
    } else {
        (locals.var_ldrifte,)
    }
};
        locals.var_ldrifte = assign22500_e27767;
        locals.var_ldrifte_rv = 0.0;

        let (assign22510_e27773, assign22510_e27773_d_n2, assign22510_e27773_d_n12,) = {
    if (locals.var_guard380 != 0.0) {
        let assign22510_e27771: f64 = (p.p33 * (nv12 - nv2));
        (assign22510_e27771, (-p.p33), p.p33,)
    } else {
        (locals.var_vrdr, locals.var_vrdr_dn2, locals.var_vrdr_dn12,)
    }
};
        locals.var_vrdr = assign22510_e27773;
        locals.var_vrdr_dn2 = assign22510_e27773_d_n2;
        locals.var_vrdr_dn12 = assign22510_e27773_d_n12;
        locals.var_vrdr_rv = 0.0;

        let (assign22540_e27796,) = {
    if (locals.var_guard380 != 0.0) {
        let assign22540_e27794: f64 = (locals.var_mks_rdrmue / 10000.0);
        (assign22540_e27794,)
    } else {
        (locals.var_mks_rdrmue,)
    }
};
        locals.var_mks_rdrmue = assign22540_e27796;
        locals.var_mks_rdrmue_rv = 0.0;

        let (assign22550_e27802,) = {
    if (locals.var_guard380 != 0.0) {
        let assign22550_e27800: f64 = (locals.var_mks_rdrvmax / 100.0);
        (assign22550_e27800,)
    } else {
        (locals.var_mks_rdrvmax,)
    }
};
        locals.var_mks_rdrvmax = assign22550_e27802;
        locals.var_mks_rdrvmax_rv = 0.0;

        let (assign22560_e27808, assign22560_e27808_d_n4,) = {
    if (locals.var_guard380 != 0.0) {
        let assign22560_e27806: f64 = (locals.var_ttemp / locals.var_uc_tnom);
        (assign22560_e27806, (locals.var_ttemp_dn4 / locals.var_uc_tnom),)
    } else {
        (locals.var_tratio, locals.var_tratio_dn4,)
    }
};
        locals.var_tratio = assign22560_e27808;
        locals.var_tratio_dn4 = assign22560_e27808_d_n4;
        locals.var_tratio_rv = 0.0;

        let (assign22570_e27814, assign22570_e27814_d_n0, assign22570_e27814_d_n2, assign22570_e27814_d_n4, assign22570_e27814_d_n5, assign22570_e27814_d_n6, assign22570_e27814_d_n8, assign22570_e27814_d_n10, assign22570_e27814_d_n11, assign22570_e27814_d_n12,) = {
    if (locals.var_guard380 != 0.0) {
        let assign22570_e27812: f64 = (locals.var_tratio).powf(p.p320);
        (assign22570_e27812, 0.0, 0.0, if 0.0 == 0.0 && ((p.p320) as f64).is_finite() && ((p.p320) as f64).fract() == 0.0 { if p.p320 == 0.0 { 0.0 } else { (p.p320 * ((locals.var_tratio).powf(p.p320 - 1.0) * locals.var_tratio_dn4)) } } else { (assign22570_e27812 * (p.p320 * (locals.var_tratio_dn4 / locals.var_tratio))) }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign22570_e27814;
        locals.var_t1_dn0 = assign22570_e27814_d_n0;
        locals.var_t1_dn2 = assign22570_e27814_d_n2;
        locals.var_t1_dn4 = assign22570_e27814_d_n4;
        locals.var_t1_dn5 = assign22570_e27814_d_n5;
        locals.var_t1_dn6 = assign22570_e27814_d_n6;
        locals.var_t1_dn8 = assign22570_e27814_d_n8;
        locals.var_t1_dn10 = assign22570_e27814_d_n10;
        locals.var_t1_dn11 = assign22570_e27814_d_n11;
        locals.var_t1_dn12 = assign22570_e27814_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign22580_e27820, assign22580_e27820_d_n0, assign22580_e27820_d_n2, assign22580_e27820_d_n4, assign22580_e27820_d_n5, assign22580_e27820_d_n6, assign22580_e27820_d_n8, assign22580_e27820_d_n10, assign22580_e27820_d_n11, assign22580_e27820_d_n12,) = {
    if (locals.var_guard380 != 0.0) {
        let assign22580_e27818: f64 = (locals.var_mks_rdrmue / locals.var_t1);
        (assign22580_e27818, (-((locals.var_mks_rdrmue * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_mu0, locals.var_mu0_dn0, locals.var_mu0_dn2, locals.var_mu0_dn4, locals.var_mu0_dn5, locals.var_mu0_dn6, locals.var_mu0_dn8, locals.var_mu0_dn10, locals.var_mu0_dn11, locals.var_mu0_dn12,)
    }
};
        locals.var_mu0 = assign22580_e27820;
        locals.var_mu0_dn0 = assign22580_e27820_d_n0;
        locals.var_mu0_dn2 = assign22580_e27820_d_n2;
        locals.var_mu0_dn4 = assign22580_e27820_d_n4;
        locals.var_mu0_dn5 = assign22580_e27820_d_n5;
        locals.var_mu0_dn6 = assign22580_e27820_d_n6;
        locals.var_mu0_dn8 = assign22580_e27820_d_n8;
        locals.var_mu0_dn10 = assign22580_e27820_d_n10;
        locals.var_mu0_dn11 = assign22580_e27820_d_n11;
        locals.var_mu0_dn12 = assign22580_e27820_d_n12;
        locals.var_mu0_rv = 0.0;

        let (assign22590_e27840, assign22590_e27840_d_n0, assign22590_e27840_d_n2, assign22590_e27840_d_n4, assign22590_e27840_d_n5, assign22590_e27840_d_n6, assign22590_e27840_d_n8, assign22590_e27840_d_n10, assign22590_e27840_d_n11, assign22590_e27840_d_n12,) = {
    if (locals.var_guard380 != 0.0) {
        let assign22590_e27825: f64 = (0.4 * locals.var_tratio);
        let assign22590_e27826: f64 = (1.8 + assign22590_e27825);
        let assign22590_e27829: f64 = (0.1 * locals.var_tratio);
        let assign22590_e27831: f64 = (assign22590_e27829 * locals.var_tratio);
        let assign22590_e27832: f64 = (assign22590_e27826 + assign22590_e27831);
        let assign22590_e27836: f64 = (1.0 - locals.var_tratio);
        let assign22590_e27837: f64 = (p.p321 * assign22590_e27836);
        let assign22590_e27838: f64 = (assign22590_e27832 - assign22590_e27837);
        (assign22590_e27838, 0.0, 0.0, (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign22590_e27829 * locals.var_tratio_dn4))) - (p.p321 * (-locals.var_tratio_dn4))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign22590_e27840;
        locals.var_t0_dn0 = assign22590_e27840_d_n0;
        locals.var_t0_dn2 = assign22590_e27840_d_n2;
        locals.var_t0_dn4 = assign22590_e27840_d_n4;
        locals.var_t0_dn5 = assign22590_e27840_d_n5;
        locals.var_t0_dn6 = assign22590_e27840_d_n6;
        locals.var_t0_dn8 = assign22590_e27840_d_n8;
        locals.var_t0_dn10 = assign22590_e27840_d_n10;
        locals.var_t0_dn11 = assign22590_e27840_d_n11;
        locals.var_t0_dn12 = assign22590_e27840_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign22600_e27846, assign22600_e27846_d_n0, assign22600_e27846_d_n2, assign22600_e27846_d_n4, assign22600_e27846_d_n5, assign22600_e27846_d_n6, assign22600_e27846_d_n8, assign22600_e27846_d_n10, assign22600_e27846_d_n11, assign22600_e27846_d_n12,) = {
    if (locals.var_guard380 != 0.0) {
        let assign22600_e27844: f64 = (locals.var_mks_rdrvmax / locals.var_t0);
        (assign22600_e27844, (-((locals.var_mks_rdrvmax * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_vmaxe__blk393, locals.var_vmaxe__blk393_dn0, locals.var_vmaxe__blk393_dn2, locals.var_vmaxe__blk393_dn4, locals.var_vmaxe__blk393_dn5, locals.var_vmaxe__blk393_dn6, locals.var_vmaxe__blk393_dn8, locals.var_vmaxe__blk393_dn10, locals.var_vmaxe__blk393_dn11, locals.var_vmaxe__blk393_dn12,)
    }
};
        locals.var_vmaxe__blk393 = assign22600_e27846;
        locals.var_vmaxe__blk393_dn0 = assign22600_e27846_d_n0;
        locals.var_vmaxe__blk393_dn2 = assign22600_e27846_d_n2;
        locals.var_vmaxe__blk393_dn4 = assign22600_e27846_d_n4;
        locals.var_vmaxe__blk393_dn5 = assign22600_e27846_d_n5;
        locals.var_vmaxe__blk393_dn6 = assign22600_e27846_d_n6;
        locals.var_vmaxe__blk393_dn8 = assign22600_e27846_d_n8;
        locals.var_vmaxe__blk393_dn10 = assign22600_e27846_d_n10;
        locals.var_vmaxe__blk393_dn11 = assign22600_e27846_d_n11;
        locals.var_vmaxe__blk393_dn12 = assign22600_e27846_d_n12;
        locals.var_vmaxe__blk393_rv = 0.0;

        let (assign22610_e27856, assign22610_e27856_d_n4,) = {
    if (locals.var_guard380 != 0.0) {
        let assign22610_e27852: f64 = (locals.var_ttemp - locals.var_uc_tnom);
        let assign22610_e27853: f64 = (p.p325 * assign22610_e27852);
        let assign22610_e27854: f64 = (locals.var_rrdrbb + assign22610_e27853);
        (assign22610_e27854, (locals.var_rrdrbb_dn4 + (p.p325 * locals.var_ttemp_dn4)),)
    } else {
        (locals.var_rrdrbb, locals.var_rrdrbb_dn4,)
    }
};
        locals.var_rrdrbb = assign22610_e27856;
        locals.var_rrdrbb_dn4 = assign22610_e27856_d_n4;
        locals.var_rrdrbb_rv = 0.0;

        let (assign22620_e27866,) = {
    if (locals.var_guard380 != 0.0) {
        let assign22620_e27862: f64 = (locals.var_lg).powf(p.p331);
        let assign22620_e27863: f64 = (p.p330 / assign22620_e27862);
        let assign22620_e27864: f64 = (1.0 + assign22620_e27863);
        (assign22620_e27864,)
    } else {
        (locals.var_rdrmuele,)
    }
};
        locals.var_rdrmuele = assign22620_e27866;
        locals.var_rdrmuele_rv = 0.0;

        let (assign22630_e27876,) = {
    if (locals.var_guard380 != 0.0) {
        let assign22630_e27872: f64 = (locals.var_lg).powf(p.p329);
        let assign22630_e27873: f64 = (p.p328 / assign22630_e27872);
        let assign22630_e27874: f64 = (1.0 + assign22630_e27873);
        (assign22630_e27874,)
    } else {
        (locals.var_rdrvmaxle,)
    }
};
        locals.var_rdrvmaxle = assign22630_e27876;
        locals.var_rdrvmaxle_rv = 0.0;

        let (assign22640_e27886,) = {
    if (locals.var_guard380 != 0.0) {
        let assign22640_e27882: f64 = (locals.var_wg).powf(p.p327);
        let assign22640_e27883: f64 = (p.p326 / assign22640_e27882);
        let assign22640_e27884: f64 = (1.0 + assign22640_e27883);
        (assign22640_e27884,)
    } else {
        (locals.var_rdrvmaxwe,)
    }
};
        locals.var_rdrvmaxwe = assign22640_e27886;
        locals.var_rdrvmaxwe_rv = 0.0;

        let (assign22650_e27892, assign22650_e27892_d_n0, assign22650_e27892_d_n2, assign22650_e27892_d_n4, assign22650_e27892_d_n5, assign22650_e27892_d_n6, assign22650_e27892_d_n8, assign22650_e27892_d_n10, assign22650_e27892_d_n11, assign22650_e27892_d_n12,) = {
    if (locals.var_guard380 != 0.0) {
        let assign22650_e27890: f64 = (locals.var_mu0 * locals.var_rdrmuele);
        (assign22650_e27890, (locals.var_mu0_dn0 * locals.var_rdrmuele), (locals.var_mu0_dn2 * locals.var_rdrmuele), (locals.var_mu0_dn4 * locals.var_rdrmuele), (locals.var_mu0_dn5 * locals.var_rdrmuele), (locals.var_mu0_dn6 * locals.var_rdrmuele), (locals.var_mu0_dn8 * locals.var_rdrmuele), (locals.var_mu0_dn10 * locals.var_rdrmuele), (locals.var_mu0_dn11 * locals.var_rdrmuele), (locals.var_mu0_dn12 * locals.var_rdrmuele),)
    } else {
        (locals.var_mu0, locals.var_mu0_dn0, locals.var_mu0_dn2, locals.var_mu0_dn4, locals.var_mu0_dn5, locals.var_mu0_dn6, locals.var_mu0_dn8, locals.var_mu0_dn10, locals.var_mu0_dn11, locals.var_mu0_dn12,)
    }
};
        locals.var_mu0 = assign22650_e27892;
        locals.var_mu0_dn0 = assign22650_e27892_d_n0;
        locals.var_mu0_dn2 = assign22650_e27892_d_n2;
        locals.var_mu0_dn4 = assign22650_e27892_d_n4;
        locals.var_mu0_dn5 = assign22650_e27892_d_n5;
        locals.var_mu0_dn6 = assign22650_e27892_d_n6;
        locals.var_mu0_dn8 = assign22650_e27892_d_n8;
        locals.var_mu0_dn10 = assign22650_e27892_d_n10;
        locals.var_mu0_dn11 = assign22650_e27892_d_n11;
        locals.var_mu0_dn12 = assign22650_e27892_d_n12;
        locals.var_mu0_rv = 0.0;

        let (assign22660_e27902, assign22660_e27902_d_n0, assign22660_e27902_d_n2, assign22660_e27902_d_n4, assign22660_e27902_d_n5, assign22660_e27902_d_n6, assign22660_e27902_d_n8, assign22660_e27902_d_n10, assign22660_e27902_d_n11, assign22660_e27902_d_n12,) = {
    if (locals.var_guard380 != 0.0) {
        let assign22660_e27896: f64 = (locals.var_vmaxe__blk393 * locals.var_rdrvmaxwe);
        let assign22660_e27898: f64 = (assign22660_e27896 * locals.var_rdrvmaxle);
        let assign22660_e27900: f64 = (assign22660_e27898 + 1e-50);
        (assign22660_e27900, ((locals.var_vmaxe__blk393_dn0 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk393_dn2 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk393_dn4 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk393_dn5 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk393_dn6 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk393_dn8 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk393_dn10 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk393_dn11 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle), ((locals.var_vmaxe__blk393_dn12 * locals.var_rdrvmaxwe) * locals.var_rdrvmaxle),)
    } else {
        (locals.var_vmaxe__blk393, locals.var_vmaxe__blk393_dn0, locals.var_vmaxe__blk393_dn2, locals.var_vmaxe__blk393_dn4, locals.var_vmaxe__blk393_dn5, locals.var_vmaxe__blk393_dn6, locals.var_vmaxe__blk393_dn8, locals.var_vmaxe__blk393_dn10, locals.var_vmaxe__blk393_dn11, locals.var_vmaxe__blk393_dn12,)
    }
};
        locals.var_vmaxe__blk393 = assign22660_e27902;
        locals.var_vmaxe__blk393_dn0 = assign22660_e27902_d_n0;
        locals.var_vmaxe__blk393_dn2 = assign22660_e27902_d_n2;
        locals.var_vmaxe__blk393_dn4 = assign22660_e27902_d_n4;
        locals.var_vmaxe__blk393_dn5 = assign22660_e27902_d_n5;
        locals.var_vmaxe__blk393_dn6 = assign22660_e27902_d_n6;
        locals.var_vmaxe__blk393_dn8 = assign22660_e27902_d_n8;
        locals.var_vmaxe__blk393_dn10 = assign22660_e27902_d_n10;
        locals.var_vmaxe__blk393_dn11 = assign22660_e27902_d_n11;
        locals.var_vmaxe__blk393_dn12 = assign22660_e27902_d_n12;
        locals.var_vmaxe__blk393_rv = 0.0;

        let (assign22670_e27908, assign22670_e27908_d_n2, assign22670_e27908_d_n12,) = {
    if (locals.var_guard380 != 0.0) {
        let assign22670_e27906: f64 = (locals.var_vrdr / locals.var_ldrifte);
        (assign22670_e27906, (locals.var_vrdr_dn2 / locals.var_ldrifte), (locals.var_vrdr_dn12 / locals.var_ldrifte),)
    } else {
        (locals.var_edri, locals.var_edri_dn2, locals.var_edri_dn12,)
    }
};
        locals.var_edri = assign22670_e27908;
        locals.var_edri_dn2 = assign22670_e27908_d_n2;
        locals.var_edri_dn12 = assign22670_e27908_d_n12;
        locals.var_edri_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_91(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let (assign22680_e27914, assign22680_e27914_d_n0, assign22680_e27914_d_n2, assign22680_e27914_d_n4, assign22680_e27914_d_n5, assign22680_e27914_d_n6, assign22680_e27914_d_n8, assign22680_e27914_d_n10, assign22680_e27914_d_n11, assign22680_e27914_d_n12,) = {
    if (locals.var_guard380 != 0.0) {
        let assign22680_e27912: f64 = (locals.var_mu0 * locals.var_edri);
        (assign22680_e27912, (locals.var_mu0_dn0 * locals.var_edri), ((locals.var_mu0_dn2 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn2)), (locals.var_mu0_dn4 * locals.var_edri), (locals.var_mu0_dn5 * locals.var_edri), (locals.var_mu0_dn6 * locals.var_edri), (locals.var_mu0_dn8 * locals.var_edri), (locals.var_mu0_dn10 * locals.var_edri), (locals.var_mu0_dn11 * locals.var_edri), ((locals.var_mu0_dn12 * locals.var_edri) + (locals.var_mu0 * locals.var_edri_dn12)),)
    } else {
        (locals.var_vdri, locals.var_vdri_dn0, locals.var_vdri_dn2, locals.var_vdri_dn4, locals.var_vdri_dn5, locals.var_vdri_dn6, locals.var_vdri_dn8, locals.var_vdri_dn10, locals.var_vdri_dn11, locals.var_vdri_dn12,)
    }
};
        locals.var_vdri = assign22680_e27914;
        locals.var_vdri_dn0 = assign22680_e27914_d_n0;
        locals.var_vdri_dn2 = assign22680_e27914_d_n2;
        locals.var_vdri_dn4 = assign22680_e27914_d_n4;
        locals.var_vdri_dn5 = assign22680_e27914_d_n5;
        locals.var_vdri_dn6 = assign22680_e27914_d_n6;
        locals.var_vdri_dn8 = assign22680_e27914_d_n8;
        locals.var_vdri_dn10 = assign22680_e27914_d_n10;
        locals.var_vdri_dn11 = assign22680_e27914_d_n11;
        locals.var_vdri_dn12 = assign22680_e27914_d_n12;
        locals.var_vdri_rv = 0.0;

        let assign22690_e27917: f64 = if locals.var_vrdr >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard400 = assign22690_e27917;
        locals.var_guard400_rv = 0.0;

        let (assign22700_e27925, assign22700_e27925_d_n0, assign22700_e27925_d_n2, assign22700_e27925_d_n4, assign22700_e27925_d_n5, assign22700_e27925_d_n6, assign22700_e27925_d_n8, assign22700_e27925_d_n10, assign22700_e27925_d_n11, assign22700_e27925_d_n12,) = {
    if ((locals.var_guard380 != 0.0) && (locals.var_guard400 != 0.0)) {
        let assign22700_e27923: f64 = (locals.var_vdri / locals.var_vmaxe__blk393);
        (assign22700_e27923, (((locals.var_vdri_dn0 * locals.var_vmaxe__blk393) - (locals.var_vdri * locals.var_vmaxe__blk393_dn0)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), (((locals.var_vdri_dn2 * locals.var_vmaxe__blk393) - (locals.var_vdri * locals.var_vmaxe__blk393_dn2)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), (((locals.var_vdri_dn4 * locals.var_vmaxe__blk393) - (locals.var_vdri * locals.var_vmaxe__blk393_dn4)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), (((locals.var_vdri_dn5 * locals.var_vmaxe__blk393) - (locals.var_vdri * locals.var_vmaxe__blk393_dn5)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), (((locals.var_vdri_dn6 * locals.var_vmaxe__blk393) - (locals.var_vdri * locals.var_vmaxe__blk393_dn6)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), (((locals.var_vdri_dn8 * locals.var_vmaxe__blk393) - (locals.var_vdri * locals.var_vmaxe__blk393_dn8)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), (((locals.var_vdri_dn10 * locals.var_vmaxe__blk393) - (locals.var_vdri * locals.var_vmaxe__blk393_dn10)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), (((locals.var_vdri_dn11 * locals.var_vmaxe__blk393) - (locals.var_vdri * locals.var_vmaxe__blk393_dn11)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), (((locals.var_vdri_dn12 * locals.var_vmaxe__blk393) - (locals.var_vdri * locals.var_vmaxe__blk393_dn12)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign22700_e27925;
        locals.var_t1_dn0 = assign22700_e27925_d_n0;
        locals.var_t1_dn2 = assign22700_e27925_d_n2;
        locals.var_t1_dn4 = assign22700_e27925_d_n4;
        locals.var_t1_dn5 = assign22700_e27925_d_n5;
        locals.var_t1_dn6 = assign22700_e27925_d_n6;
        locals.var_t1_dn8 = assign22700_e27925_d_n8;
        locals.var_t1_dn10 = assign22700_e27925_d_n10;
        locals.var_t1_dn11 = assign22700_e27925_d_n11;
        locals.var_t1_dn12 = assign22700_e27925_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign22710_e27935, assign22710_e27935_d_n0, assign22710_e27935_d_n2, assign22710_e27935_d_n4, assign22710_e27935_d_n5, assign22710_e27935_d_n6, assign22710_e27935_d_n8, assign22710_e27935_d_n10, assign22710_e27935_d_n11, assign22710_e27935_d_n12,) = {
    if ((locals.var_guard380 != 0.0) && (locals.var_guard400 == 0.0)) {
        let assign22710_e27931: f64 = (-locals.var_vdri);
        let assign22710_e27933: f64 = (assign22710_e27931 / locals.var_vmaxe__blk393);
        (assign22710_e27933, ((((-locals.var_vdri_dn0) * locals.var_vmaxe__blk393) - (assign22710_e27931 * locals.var_vmaxe__blk393_dn0)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), ((((-locals.var_vdri_dn2) * locals.var_vmaxe__blk393) - (assign22710_e27931 * locals.var_vmaxe__blk393_dn2)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), ((((-locals.var_vdri_dn4) * locals.var_vmaxe__blk393) - (assign22710_e27931 * locals.var_vmaxe__blk393_dn4)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), ((((-locals.var_vdri_dn5) * locals.var_vmaxe__blk393) - (assign22710_e27931 * locals.var_vmaxe__blk393_dn5)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), ((((-locals.var_vdri_dn6) * locals.var_vmaxe__blk393) - (assign22710_e27931 * locals.var_vmaxe__blk393_dn6)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), ((((-locals.var_vdri_dn8) * locals.var_vmaxe__blk393) - (assign22710_e27931 * locals.var_vmaxe__blk393_dn8)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), ((((-locals.var_vdri_dn10) * locals.var_vmaxe__blk393) - (assign22710_e27931 * locals.var_vmaxe__blk393_dn10)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), ((((-locals.var_vdri_dn11) * locals.var_vmaxe__blk393) - (assign22710_e27931 * locals.var_vmaxe__blk393_dn11)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)), ((((-locals.var_vdri_dn12) * locals.var_vmaxe__blk393) - (assign22710_e27931 * locals.var_vmaxe__blk393_dn12)) / (locals.var_vmaxe__blk393 * locals.var_vmaxe__blk393)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign22710_e27935;
        locals.var_t1_dn0 = assign22710_e27935_d_n0;
        locals.var_t1_dn2 = assign22710_e27935_d_n2;
        locals.var_t1_dn4 = assign22710_e27935_d_n4;
        locals.var_t1_dn5 = assign22710_e27935_d_n5;
        locals.var_t1_dn6 = assign22710_e27935_d_n6;
        locals.var_t1_dn8 = assign22710_e27935_d_n8;
        locals.var_t1_dn10 = assign22710_e27935_d_n10;
        locals.var_t1_dn11 = assign22710_e27935_d_n11;
        locals.var_t1_dn12 = assign22710_e27935_d_n12;
        locals.var_t1_rv = 0.0;

        let assign22720_e27939: f64 = (10.0 * 2.220446049250313e-16);
        let assign22720_e27940: f64 = (1.0 - assign22720_e27939);
        let assign22720_e27947: f64 = (10.0 * 2.220446049250313e-16);
        let assign22720_e27948: f64 = (1.0 + assign22720_e27947);
        let assign22720_e27950: f64 = if ((assign22720_e27940 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign22720_e27948)) { 1.0 } else { 0.0 };
        locals.var_guard401 = assign22720_e27950;
        locals.var_guard401_rv = 0.0;

        let (assign22730_e27956, assign22730_e27956_d_n0, assign22730_e27956_d_n2, assign22730_e27956_d_n4, assign22730_e27956_d_n5, assign22730_e27956_d_n6, assign22730_e27956_d_n8, assign22730_e27956_d_n10, assign22730_e27956_d_n11, assign22730_e27956_d_n12,) = {
    if ((locals.var_guard380 != 0.0) && (locals.var_guard401 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign22730_e27956;
        locals.var_t3_dn0 = assign22730_e27956_d_n0;
        locals.var_t3_dn2 = assign22730_e27956_d_n2;
        locals.var_t3_dn4 = assign22730_e27956_d_n4;
        locals.var_t3_dn5 = assign22730_e27956_d_n5;
        locals.var_t3_dn6 = assign22730_e27956_d_n6;
        locals.var_t3_dn8 = assign22730_e27956_d_n8;
        locals.var_t3_dn10 = assign22730_e27956_d_n10;
        locals.var_t3_dn11 = assign22730_e27956_d_n11;
        locals.var_t3_dn12 = assign22730_e27956_d_n12;
        locals.var_t3_rv = 0.0;

        let assign22740_e27960: f64 = (10.0 * 2.220446049250313e-16);
        let assign22740_e27961: f64 = (2.0 - assign22740_e27960);
        let assign22740_e27968: f64 = (10.0 * 2.220446049250313e-16);
        let assign22740_e27969: f64 = (2.0 + assign22740_e27968);
        let assign22740_e27971: f64 = if ((assign22740_e27961 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign22740_e27969)) { 1.0 } else { 0.0 };
        locals.var_guard402 = assign22740_e27971;
        locals.var_guard402_rv = 0.0;

        let (assign22750_e27980, assign22750_e27980_d_n0, assign22750_e27980_d_n2, assign22750_e27980_d_n4, assign22750_e27980_d_n5, assign22750_e27980_d_n6, assign22750_e27980_d_n8, assign22750_e27980_d_n10, assign22750_e27980_d_n11, assign22750_e27980_d_n12,) = {
    if (((locals.var_guard380 != 0.0) && (locals.var_guard401 == 0.0)) && (locals.var_guard402 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign22750_e27980;
        locals.var_t3_dn0 = assign22750_e27980_d_n0;
        locals.var_t3_dn2 = assign22750_e27980_d_n2;
        locals.var_t3_dn4 = assign22750_e27980_d_n4;
        locals.var_t3_dn5 = assign22750_e27980_d_n5;
        locals.var_t3_dn6 = assign22750_e27980_d_n6;
        locals.var_t3_dn8 = assign22750_e27980_d_n8;
        locals.var_t3_dn10 = assign22750_e27980_d_n10;
        locals.var_t3_dn11 = assign22750_e27980_d_n11;
        locals.var_t3_dn12 = assign22750_e27980_d_n12;
        locals.var_t3_rv = 0.0;

        let (assign22760_e27994, assign22760_e27994_d_n0, assign22760_e27994_d_n2, assign22760_e27994_d_n4, assign22760_e27994_d_n5, assign22760_e27994_d_n6, assign22760_e27994_d_n8, assign22760_e27994_d_n10, assign22760_e27994_d_n11, assign22760_e27994_d_n12,) = {
    if (((locals.var_guard380 != 0.0) && (locals.var_guard401 == 0.0)) && (locals.var_guard402 == 0.0)) {
        let assign22760_e27991: f64 = (locals.var_rrdrbb - 1.0);
        let assign22760_e27992: f64 = (locals.var_t1).powf(assign22760_e27991);
        (assign22760_e27992, if 0.0 == 0.0 && ((assign22760_e27991) as f64).is_finite() && ((assign22760_e27991) as f64).fract() == 0.0 { if assign22760_e27991 == 0.0 { 0.0 } else { (assign22760_e27991 * ((locals.var_t1).powf(assign22760_e27991 - 1.0) * locals.var_t1_dn0)) } } else { (assign22760_e27992 * (assign22760_e27991 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign22760_e27991) as f64).is_finite() && ((assign22760_e27991) as f64).fract() == 0.0 { if assign22760_e27991 == 0.0 { 0.0 } else { (assign22760_e27991 * ((locals.var_t1).powf(assign22760_e27991 - 1.0) * locals.var_t1_dn2)) } } else { (assign22760_e27992 * (assign22760_e27991 * (locals.var_t1_dn2 / locals.var_t1))) }, if locals.var_rrdrbb_dn4 == 0.0 && ((assign22760_e27991) as f64).is_finite() && ((assign22760_e27991) as f64).fract() == 0.0 { if assign22760_e27991 == 0.0 { 0.0 } else { (assign22760_e27991 * ((locals.var_t1).powf(assign22760_e27991 - 1.0) * locals.var_t1_dn4)) } } else { (assign22760_e27992 * ((locals.var_rrdrbb_dn4 * (locals.var_t1).ln()) + (assign22760_e27991 * (locals.var_t1_dn4 / locals.var_t1)))) }, if 0.0 == 0.0 && ((assign22760_e27991) as f64).is_finite() && ((assign22760_e27991) as f64).fract() == 0.0 { if assign22760_e27991 == 0.0 { 0.0 } else { (assign22760_e27991 * ((locals.var_t1).powf(assign22760_e27991 - 1.0) * locals.var_t1_dn5)) } } else { (assign22760_e27992 * (assign22760_e27991 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign22760_e27991) as f64).is_finite() && ((assign22760_e27991) as f64).fract() == 0.0 { if assign22760_e27991 == 0.0 { 0.0 } else { (assign22760_e27991 * ((locals.var_t1).powf(assign22760_e27991 - 1.0) * locals.var_t1_dn6)) } } else { (assign22760_e27992 * (assign22760_e27991 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign22760_e27991) as f64).is_finite() && ((assign22760_e27991) as f64).fract() == 0.0 { if assign22760_e27991 == 0.0 { 0.0 } else { (assign22760_e27991 * ((locals.var_t1).powf(assign22760_e27991 - 1.0) * locals.var_t1_dn8)) } } else { (assign22760_e27992 * (assign22760_e27991 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign22760_e27991) as f64).is_finite() && ((assign22760_e27991) as f64).fract() == 0.0 { if assign22760_e27991 == 0.0 { 0.0 } else { (assign22760_e27991 * ((locals.var_t1).powf(assign22760_e27991 - 1.0) * locals.var_t1_dn10)) } } else { (assign22760_e27992 * (assign22760_e27991 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign22760_e27991) as f64).is_finite() && ((assign22760_e27991) as f64).fract() == 0.0 { if assign22760_e27991 == 0.0 { 0.0 } else { (assign22760_e27991 * ((locals.var_t1).powf(assign22760_e27991 - 1.0) * locals.var_t1_dn11)) } } else { (assign22760_e27992 * (assign22760_e27991 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign22760_e27991) as f64).is_finite() && ((assign22760_e27991) as f64).fract() == 0.0 { if assign22760_e27991 == 0.0 { 0.0 } else { (assign22760_e27991 * ((locals.var_t1).powf(assign22760_e27991 - 1.0) * locals.var_t1_dn12)) } } else { (assign22760_e27992 * (assign22760_e27991 * (locals.var_t1_dn12 / locals.var_t1))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign22760_e27994;
        locals.var_t3_dn0 = assign22760_e27994_d_n0;
        locals.var_t3_dn2 = assign22760_e27994_d_n2;
        locals.var_t3_dn4 = assign22760_e27994_d_n4;
        locals.var_t3_dn5 = assign22760_e27994_d_n5;
        locals.var_t3_dn6 = assign22760_e27994_d_n6;
        locals.var_t3_dn8 = assign22760_e27994_d_n8;
        locals.var_t3_dn10 = assign22760_e27994_d_n10;
        locals.var_t3_dn11 = assign22760_e27994_d_n11;
        locals.var_t3_dn12 = assign22760_e27994_d_n12;
        locals.var_t3_rv = 0.0;

        let (assign22770_e28000, assign22770_e28000_d_n0, assign22770_e28000_d_n2, assign22770_e28000_d_n4, assign22770_e28000_d_n5, assign22770_e28000_d_n6, assign22770_e28000_d_n8, assign22770_e28000_d_n10, assign22770_e28000_d_n11, assign22770_e28000_d_n12,) = {
    if (locals.var_guard380 != 0.0) {
        let assign22770_e27998: f64 = (locals.var_t1 * locals.var_t3);
        (assign22770_e27998, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn12 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn12)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign22770_e28000;
        locals.var_t2_dn0 = assign22770_e28000_d_n0;
        locals.var_t2_dn2 = assign22770_e28000_d_n2;
        locals.var_t2_dn4 = assign22770_e28000_d_n4;
        locals.var_t2_dn5 = assign22770_e28000_d_n5;
        locals.var_t2_dn6 = assign22770_e28000_d_n6;
        locals.var_t2_dn8 = assign22770_e28000_d_n8;
        locals.var_t2_dn10 = assign22770_e28000_d_n10;
        locals.var_t2_dn11 = assign22770_e28000_d_n11;
        locals.var_t2_dn12 = assign22770_e28000_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign22780_e28006, assign22780_e28006_d_n0, assign22780_e28006_d_n2, assign22780_e28006_d_n4, assign22780_e28006_d_n5, assign22780_e28006_d_n6, assign22780_e28006_d_n8, assign22780_e28006_d_n10, assign22780_e28006_d_n11, assign22780_e28006_d_n12,) = {
    if (locals.var_guard380 != 0.0) {
        let assign22780_e28004: f64 = (1.0 + locals.var_t2);
        (assign22780_e28004, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn8, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign22780_e28006;
        locals.var_t4_dn0 = assign22780_e28006_d_n0;
        locals.var_t4_dn2 = assign22780_e28006_d_n2;
        locals.var_t4_dn4 = assign22780_e28006_d_n4;
        locals.var_t4_dn5 = assign22780_e28006_d_n5;
        locals.var_t4_dn6 = assign22780_e28006_d_n6;
        locals.var_t4_dn8 = assign22780_e28006_d_n8;
        locals.var_t4_dn10 = assign22780_e28006_d_n10;
        locals.var_t4_dn11 = assign22780_e28006_d_n11;
        locals.var_t4_dn12 = assign22780_e28006_d_n12;
        locals.var_t4_rv = 0.0;

        let assign22790_e28010: f64 = (10.0 * 2.220446049250313e-16);
        let assign22790_e28011: f64 = (1.0 - assign22790_e28010);
        let assign22790_e28018: f64 = (10.0 * 2.220446049250313e-16);
        let assign22790_e28019: f64 = (1.0 + assign22790_e28018);
        let assign22790_e28021: f64 = if ((assign22790_e28011 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign22790_e28019)) { 1.0 } else { 0.0 };
        locals.var_guard403 = assign22790_e28021;
        locals.var_guard403_rv = 0.0;

        let (assign22800_e28029, assign22800_e28029_d_n0, assign22800_e28029_d_n2, assign22800_e28029_d_n4, assign22800_e28029_d_n5, assign22800_e28029_d_n6, assign22800_e28029_d_n8, assign22800_e28029_d_n10, assign22800_e28029_d_n11, assign22800_e28029_d_n12,) = {
    if ((locals.var_guard380 != 0.0) && (locals.var_guard403 != 0.0)) {
        let assign22800_e28027: f64 = (1.0 / locals.var_t4);
        (assign22800_e28027, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn12 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
        locals.var_t5 = assign22800_e28029;
        locals.var_t5_dn0 = assign22800_e28029_d_n0;
        locals.var_t5_dn2 = assign22800_e28029_d_n2;
        locals.var_t5_dn4 = assign22800_e28029_d_n4;
        locals.var_t5_dn5 = assign22800_e28029_d_n5;
        locals.var_t5_dn6 = assign22800_e28029_d_n6;
        locals.var_t5_dn8 = assign22800_e28029_d_n8;
        locals.var_t5_dn10 = assign22800_e28029_d_n10;
        locals.var_t5_dn11 = assign22800_e28029_d_n11;
        locals.var_t5_dn12 = assign22800_e28029_d_n12;
        locals.var_t5_rv = 0.0;

        let assign22810_e28033: f64 = (10.0 * 2.220446049250313e-16);
        let assign22810_e28034: f64 = (2.0 - assign22810_e28033);
        let assign22810_e28041: f64 = (10.0 * 2.220446049250313e-16);
        let assign22810_e28042: f64 = (2.0 + assign22810_e28041);
        let assign22810_e28044: f64 = if ((assign22810_e28034 <= locals.var_rrdrbb) && (locals.var_rrdrbb <= assign22810_e28042)) { 1.0 } else { 0.0 };
        locals.var_guard404 = assign22810_e28044;
        locals.var_guard404_rv = 0.0;

        let (assign22820_e28056, assign22820_e28056_d_n0, assign22820_e28056_d_n2, assign22820_e28056_d_n4, assign22820_e28056_d_n5, assign22820_e28056_d_n6, assign22820_e28056_d_n8, assign22820_e28056_d_n10, assign22820_e28056_d_n11, assign22820_e28056_d_n12,) = {
    if (((locals.var_guard380 != 0.0) && (locals.var_guard403 == 0.0)) && (locals.var_guard404 != 0.0)) {
        let assign22820_e28053: f64 = (locals.var_t4).sqrt();
        let assign22820_e28054: f64 = (1.0 / assign22820_e28053);
        (assign22820_e28054, (-((locals.var_t4_dn0 / (2.0 * assign22820_e28053)) / (assign22820_e28053 * assign22820_e28053))), (-((locals.var_t4_dn2 / (2.0 * assign22820_e28053)) / (assign22820_e28053 * assign22820_e28053))), (-((locals.var_t4_dn4 / (2.0 * assign22820_e28053)) / (assign22820_e28053 * assign22820_e28053))), (-((locals.var_t4_dn5 / (2.0 * assign22820_e28053)) / (assign22820_e28053 * assign22820_e28053))), (-((locals.var_t4_dn6 / (2.0 * assign22820_e28053)) / (assign22820_e28053 * assign22820_e28053))), (-((locals.var_t4_dn8 / (2.0 * assign22820_e28053)) / (assign22820_e28053 * assign22820_e28053))), (-((locals.var_t4_dn10 / (2.0 * assign22820_e28053)) / (assign22820_e28053 * assign22820_e28053))), (-((locals.var_t4_dn11 / (2.0 * assign22820_e28053)) / (assign22820_e28053 * assign22820_e28053))), (-((locals.var_t4_dn12 / (2.0 * assign22820_e28053)) / (assign22820_e28053 * assign22820_e28053))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
        locals.var_t5 = assign22820_e28056;
        locals.var_t5_dn0 = assign22820_e28056_d_n0;
        locals.var_t5_dn2 = assign22820_e28056_d_n2;
        locals.var_t5_dn4 = assign22820_e28056_d_n4;
        locals.var_t5_dn5 = assign22820_e28056_d_n5;
        locals.var_t5_dn6 = assign22820_e28056_d_n6;
        locals.var_t5_dn8 = assign22820_e28056_d_n8;
        locals.var_t5_dn10 = assign22820_e28056_d_n10;
        locals.var_t5_dn11 = assign22820_e28056_d_n11;
        locals.var_t5_dn12 = assign22820_e28056_d_n12;
        locals.var_t5_rv = 0.0;

        let (assign22830_e28073, assign22830_e28073_d_n0, assign22830_e28073_d_n2, assign22830_e28073_d_n4, assign22830_e28073_d_n5, assign22830_e28073_d_n6, assign22830_e28073_d_n8, assign22830_e28073_d_n10, assign22830_e28073_d_n11, assign22830_e28073_d_n12,) = {
    if (((locals.var_guard380 != 0.0) && (locals.var_guard403 == 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign22830_e28066: f64 = (-1.0);
        let assign22830_e28068: f64 = (assign22830_e28066 / locals.var_rrdrbb);
        let assign22830_e28070: f64 = (assign22830_e28068 - 1.0);
        let assign22830_e28071: f64 = (locals.var_t4).powf(assign22830_e28070);
        (assign22830_e28071, if 0.0 == 0.0 && ((assign22830_e28070) as f64).is_finite() && ((assign22830_e28070) as f64).fract() == 0.0 { if assign22830_e28070 == 0.0 { 0.0 } else { (assign22830_e28070 * ((locals.var_t4).powf(assign22830_e28070 - 1.0) * locals.var_t4_dn0)) } } else { (assign22830_e28071 * (assign22830_e28070 * (locals.var_t4_dn0 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign22830_e28070) as f64).is_finite() && ((assign22830_e28070) as f64).fract() == 0.0 { if assign22830_e28070 == 0.0 { 0.0 } else { (assign22830_e28070 * ((locals.var_t4).powf(assign22830_e28070 - 1.0) * locals.var_t4_dn2)) } } else { (assign22830_e28071 * (assign22830_e28070 * (locals.var_t4_dn2 / locals.var_t4))) }, if (-((assign22830_e28066 * locals.var_rrdrbb_dn4) / (locals.var_rrdrbb * locals.var_rrdrbb))) == 0.0 && ((assign22830_e28070) as f64).is_finite() && ((assign22830_e28070) as f64).fract() == 0.0 { if assign22830_e28070 == 0.0 { 0.0 } else { (assign22830_e28070 * ((locals.var_t4).powf(assign22830_e28070 - 1.0) * locals.var_t4_dn4)) } } else { (assign22830_e28071 * (((-((assign22830_e28066 * locals.var_rrdrbb_dn4) / (locals.var_rrdrbb * locals.var_rrdrbb))) * (locals.var_t4).ln()) + (assign22830_e28070 * (locals.var_t4_dn4 / locals.var_t4)))) }, if 0.0 == 0.0 && ((assign22830_e28070) as f64).is_finite() && ((assign22830_e28070) as f64).fract() == 0.0 { if assign22830_e28070 == 0.0 { 0.0 } else { (assign22830_e28070 * ((locals.var_t4).powf(assign22830_e28070 - 1.0) * locals.var_t4_dn5)) } } else { (assign22830_e28071 * (assign22830_e28070 * (locals.var_t4_dn5 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign22830_e28070) as f64).is_finite() && ((assign22830_e28070) as f64).fract() == 0.0 { if assign22830_e28070 == 0.0 { 0.0 } else { (assign22830_e28070 * ((locals.var_t4).powf(assign22830_e28070 - 1.0) * locals.var_t4_dn6)) } } else { (assign22830_e28071 * (assign22830_e28070 * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign22830_e28070) as f64).is_finite() && ((assign22830_e28070) as f64).fract() == 0.0 { if assign22830_e28070 == 0.0 { 0.0 } else { (assign22830_e28070 * ((locals.var_t4).powf(assign22830_e28070 - 1.0) * locals.var_t4_dn8)) } } else { (assign22830_e28071 * (assign22830_e28070 * (locals.var_t4_dn8 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign22830_e28070) as f64).is_finite() && ((assign22830_e28070) as f64).fract() == 0.0 { if assign22830_e28070 == 0.0 { 0.0 } else { (assign22830_e28070 * ((locals.var_t4).powf(assign22830_e28070 - 1.0) * locals.var_t4_dn10)) } } else { (assign22830_e28071 * (assign22830_e28070 * (locals.var_t4_dn10 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign22830_e28070) as f64).is_finite() && ((assign22830_e28070) as f64).fract() == 0.0 { if assign22830_e28070 == 0.0 { 0.0 } else { (assign22830_e28070 * ((locals.var_t4).powf(assign22830_e28070 - 1.0) * locals.var_t4_dn11)) } } else { (assign22830_e28071 * (assign22830_e28070 * (locals.var_t4_dn11 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign22830_e28070) as f64).is_finite() && ((assign22830_e28070) as f64).fract() == 0.0 { if assign22830_e28070 == 0.0 { 0.0 } else { (assign22830_e28070 * ((locals.var_t4).powf(assign22830_e28070 - 1.0) * locals.var_t4_dn12)) } } else { (assign22830_e28071 * (assign22830_e28070 * (locals.var_t4_dn12 / locals.var_t4))) },)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn8, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12,)
    }
};
        locals.var_t6 = assign22830_e28073;
        locals.var_t6_dn0 = assign22830_e28073_d_n0;
        locals.var_t6_dn2 = assign22830_e28073_d_n2;
        locals.var_t6_dn4 = assign22830_e28073_d_n4;
        locals.var_t6_dn5 = assign22830_e28073_d_n5;
        locals.var_t6_dn6 = assign22830_e28073_d_n6;
        locals.var_t6_dn8 = assign22830_e28073_d_n8;
        locals.var_t6_dn10 = assign22830_e28073_d_n10;
        locals.var_t6_dn11 = assign22830_e28073_d_n11;
        locals.var_t6_dn12 = assign22830_e28073_d_n12;
        locals.var_t6_rv = 0.0;

        let (assign22840_e28085, assign22840_e28085_d_n0, assign22840_e28085_d_n2, assign22840_e28085_d_n4, assign22840_e28085_d_n5, assign22840_e28085_d_n6, assign22840_e28085_d_n8, assign22840_e28085_d_n10, assign22840_e28085_d_n11, assign22840_e28085_d_n12,) = {
    if (((locals.var_guard380 != 0.0) && (locals.var_guard403 == 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign22840_e28083: f64 = (locals.var_t4 * locals.var_t6);
        (assign22840_e28083, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn4 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn4)), ((locals.var_t4_dn5 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn5)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn8 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn8)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn11 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn11)), ((locals.var_t4_dn12 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn12)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
        locals.var_t5 = assign22840_e28085;
        locals.var_t5_dn0 = assign22840_e28085_d_n0;
        locals.var_t5_dn2 = assign22840_e28085_d_n2;
        locals.var_t5_dn4 = assign22840_e28085_d_n4;
        locals.var_t5_dn5 = assign22840_e28085_d_n5;
        locals.var_t5_dn6 = assign22840_e28085_d_n6;
        locals.var_t5_dn8 = assign22840_e28085_d_n8;
        locals.var_t5_dn10 = assign22840_e28085_d_n10;
        locals.var_t5_dn11 = assign22840_e28085_d_n11;
        locals.var_t5_dn12 = assign22840_e28085_d_n12;
        locals.var_t5_rv = 0.0;

        let (assign22860_e28097, assign22860_e28097_d_n0, assign22860_e28097_d_n2, assign22860_e28097_d_n4, assign22860_e28097_d_n5, assign22860_e28097_d_n6, assign22860_e28097_d_n8, assign22860_e28097_d_n10, assign22860_e28097_d_n11, assign22860_e28097_d_n12,) = {
    if (locals.var_guard380 != 0.0) {
        let assign22860_e28095: f64 = (1.6021918e-19 / locals.var_ldrifte);
        (assign22860_e28095, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign22860_e28097;
        locals.var_t1_dn0 = assign22860_e28097_d_n0;
        locals.var_t1_dn2 = assign22860_e28097_d_n2;
        locals.var_t1_dn4 = assign22860_e28097_d_n4;
        locals.var_t1_dn5 = assign22860_e28097_d_n5;
        locals.var_t1_dn6 = assign22860_e28097_d_n6;
        locals.var_t1_dn8 = assign22860_e28097_d_n8;
        locals.var_t1_dn10 = assign22860_e28097_d_n10;
        locals.var_t1_dn11 = assign22860_e28097_d_n11;
        locals.var_t1_dn12 = assign22860_e28097_d_n12;
        locals.var_t1_rv = 0.0;

        let assign22980_e28171: f64 = if p.p313 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard407 = assign22980_e28171;
        locals.var_guard407_rv = 0.0;

        let (assign23000_e28181,) = {
    if (locals.var_guard407 != 0.0) {
        (p.p316,)
    } else {
        (locals.var_mks_rdrmue__blk411,)
    }
};
        locals.var_mks_rdrmue__blk411 = assign23000_e28181;
        locals.var_mks_rdrmue__blk411_rv = 0.0;

        let (assign23010_e28185,) = {
    if (locals.var_guard407 != 0.0) {
        (p.p318,)
    } else {
        (locals.var_mks_rdrvmax__blk412,)
    }
};
        locals.var_mks_rdrvmax__blk412 = assign23010_e28185;
        locals.var_mks_rdrvmax__blk412_rv = 0.0;

        let (assign23020_e28189, assign23020_e28189_d_n4,) = {
    if (locals.var_guard407 != 0.0) {
        (p.p323, 0.0,)
    } else {
        (locals.var_rrdrbb__blk413, locals.var_rrdrbb__blk413_dn4,)
    }
};
        locals.var_rrdrbb__blk413 = assign23020_e28189;
        locals.var_rrdrbb__blk413_dn4 = assign23020_e28189_d_n4;
        locals.var_rrdrbb__blk413_rv = 0.0;

        let (assign23040_e28204,) = {
    if (locals.var_guard407 != 0.0) {
        (p.p310,)
    } else {
        (locals.var_ldrifte__blk417,)
    }
};
        locals.var_ldrifte__blk417 = assign23040_e28204;
        locals.var_ldrifte__blk417_rv = 0.0;

        let (assign23050_e28210, assign23050_e28210_d_n0, assign23050_e28210_d_n11,) = {
    if (locals.var_guard407 != 0.0) {
        let assign23050_e28208: f64 = (p.p33 * (nv0 - nv11));
        (assign23050_e28208, p.p33, (-p.p33),)
    } else {
        (locals.var_vrdr__blk415, locals.var_vrdr__blk415_dn0, locals.var_vrdr__blk415_dn11,)
    }
};
        locals.var_vrdr__blk415 = assign23050_e28210;
        locals.var_vrdr__blk415_dn0 = assign23050_e28210_d_n0;
        locals.var_vrdr__blk415_dn11 = assign23050_e28210_d_n11;
        locals.var_vrdr__blk415_rv = 0.0;

        let (assign23080_e28233,) = {
    if (locals.var_guard407 != 0.0) {
        let assign23080_e28231: f64 = (locals.var_mks_rdrmue__blk411 / 10000.0);
        (assign23080_e28231,)
    } else {
        (locals.var_mks_rdrmue__blk411,)
    }
};
        locals.var_mks_rdrmue__blk411 = assign23080_e28233;
        locals.var_mks_rdrmue__blk411_rv = 0.0;

        let (assign23090_e28239,) = {
    if (locals.var_guard407 != 0.0) {
        let assign23090_e28237: f64 = (locals.var_mks_rdrvmax__blk412 / 100.0);
        (assign23090_e28237,)
    } else {
        (locals.var_mks_rdrvmax__blk412,)
    }
};
        locals.var_mks_rdrvmax__blk412 = assign23090_e28239;
        locals.var_mks_rdrvmax__blk412_rv = 0.0;

        let (assign23100_e28245, assign23100_e28245_d_n4,) = {
    if (locals.var_guard407 != 0.0) {
        let assign23100_e28243: f64 = (locals.var_ttemp / locals.var_uc_tnom);
        (assign23100_e28243, (locals.var_ttemp_dn4 / locals.var_uc_tnom),)
    } else {
        (locals.var_tratio__blk416, locals.var_tratio__blk416_dn4,)
    }
};
        locals.var_tratio__blk416 = assign23100_e28245;
        locals.var_tratio__blk416_dn4 = assign23100_e28245_d_n4;
        locals.var_tratio__blk416_rv = 0.0;

        let (assign23110_e28251, assign23110_e28251_d_n0, assign23110_e28251_d_n2, assign23110_e28251_d_n4, assign23110_e28251_d_n5, assign23110_e28251_d_n6, assign23110_e28251_d_n8, assign23110_e28251_d_n10, assign23110_e28251_d_n11, assign23110_e28251_d_n12,) = {
    if (locals.var_guard407 != 0.0) {
        let assign23110_e28249: f64 = (locals.var_tratio__blk416).powf(p.p320);
        (assign23110_e28249, 0.0, 0.0, if 0.0 == 0.0 && ((p.p320) as f64).is_finite() && ((p.p320) as f64).fract() == 0.0 { if p.p320 == 0.0 { 0.0 } else { (p.p320 * ((locals.var_tratio__blk416).powf(p.p320 - 1.0) * locals.var_tratio__blk416_dn4)) } } else { (assign23110_e28249 * (p.p320 * (locals.var_tratio__blk416_dn4 / locals.var_tratio__blk416))) }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign23110_e28251;
        locals.var_t1_dn0 = assign23110_e28251_d_n0;
        locals.var_t1_dn2 = assign23110_e28251_d_n2;
        locals.var_t1_dn4 = assign23110_e28251_d_n4;
        locals.var_t1_dn5 = assign23110_e28251_d_n5;
        locals.var_t1_dn6 = assign23110_e28251_d_n6;
        locals.var_t1_dn8 = assign23110_e28251_d_n8;
        locals.var_t1_dn10 = assign23110_e28251_d_n10;
        locals.var_t1_dn11 = assign23110_e28251_d_n11;
        locals.var_t1_dn12 = assign23110_e28251_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign23120_e28257, assign23120_e28257_d_n0, assign23120_e28257_d_n2, assign23120_e28257_d_n4, assign23120_e28257_d_n5, assign23120_e28257_d_n6, assign23120_e28257_d_n8, assign23120_e28257_d_n10, assign23120_e28257_d_n11, assign23120_e28257_d_n12,) = {
    if (locals.var_guard407 != 0.0) {
        let assign23120_e28255: f64 = (locals.var_mks_rdrmue__blk411 / locals.var_t1);
        (assign23120_e28255, (-((locals.var_mks_rdrmue__blk411 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk411 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk411 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk411 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk411 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk411 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk411 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk411 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue__blk411 * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_mu0__blk419, locals.var_mu0__blk419_dn0, locals.var_mu0__blk419_dn2, locals.var_mu0__blk419_dn4, locals.var_mu0__blk419_dn5, locals.var_mu0__blk419_dn6, locals.var_mu0__blk419_dn8, locals.var_mu0__blk419_dn10, locals.var_mu0__blk419_dn11, locals.var_mu0__blk419_dn12,)
    }
};
        locals.var_mu0__blk419 = assign23120_e28257;
        locals.var_mu0__blk419_dn0 = assign23120_e28257_d_n0;
        locals.var_mu0__blk419_dn2 = assign23120_e28257_d_n2;
        locals.var_mu0__blk419_dn4 = assign23120_e28257_d_n4;
        locals.var_mu0__blk419_dn5 = assign23120_e28257_d_n5;
        locals.var_mu0__blk419_dn6 = assign23120_e28257_d_n6;
        locals.var_mu0__blk419_dn8 = assign23120_e28257_d_n8;
        locals.var_mu0__blk419_dn10 = assign23120_e28257_d_n10;
        locals.var_mu0__blk419_dn11 = assign23120_e28257_d_n11;
        locals.var_mu0__blk419_dn12 = assign23120_e28257_d_n12;
        locals.var_mu0__blk419_rv = 0.0;

        let (assign23130_e28277, assign23130_e28277_d_n0, assign23130_e28277_d_n2, assign23130_e28277_d_n4, assign23130_e28277_d_n5, assign23130_e28277_d_n6, assign23130_e28277_d_n8, assign23130_e28277_d_n10, assign23130_e28277_d_n11, assign23130_e28277_d_n12,) = {
    if (locals.var_guard407 != 0.0) {
        let assign23130_e28262: f64 = (0.4 * locals.var_tratio__blk416);
        let assign23130_e28263: f64 = (1.8 + assign23130_e28262);
        let assign23130_e28266: f64 = (0.1 * locals.var_tratio__blk416);
        let assign23130_e28268: f64 = (assign23130_e28266 * locals.var_tratio__blk416);
        let assign23130_e28269: f64 = (assign23130_e28263 + assign23130_e28268);
        let assign23130_e28273: f64 = (1.0 - locals.var_tratio__blk416);
        let assign23130_e28274: f64 = (p.p321 * assign23130_e28273);
        let assign23130_e28275: f64 = (assign23130_e28269 - assign23130_e28274);
        (assign23130_e28275, 0.0, 0.0, (((0.4 * locals.var_tratio__blk416_dn4) + (((0.1 * locals.var_tratio__blk416_dn4) * locals.var_tratio__blk416) + (assign23130_e28266 * locals.var_tratio__blk416_dn4))) - (p.p321 * (-locals.var_tratio__blk416_dn4))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign23130_e28277;
        locals.var_t0_dn0 = assign23130_e28277_d_n0;
        locals.var_t0_dn2 = assign23130_e28277_d_n2;
        locals.var_t0_dn4 = assign23130_e28277_d_n4;
        locals.var_t0_dn5 = assign23130_e28277_d_n5;
        locals.var_t0_dn6 = assign23130_e28277_d_n6;
        locals.var_t0_dn8 = assign23130_e28277_d_n8;
        locals.var_t0_dn10 = assign23130_e28277_d_n10;
        locals.var_t0_dn11 = assign23130_e28277_d_n11;
        locals.var_t0_dn12 = assign23130_e28277_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign23140_e28283, assign23140_e28283_d_n0, assign23140_e28283_d_n2, assign23140_e28283_d_n4, assign23140_e28283_d_n5, assign23140_e28283_d_n6, assign23140_e28283_d_n8, assign23140_e28283_d_n10, assign23140_e28283_d_n11, assign23140_e28283_d_n12,) = {
    if (locals.var_guard407 != 0.0) {
        let assign23140_e28281: f64 = (locals.var_mks_rdrvmax__blk412 / locals.var_t0);
        (assign23140_e28281, (-((locals.var_mks_rdrvmax__blk412 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk412 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk412 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk412 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk412 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk412 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk412 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk412 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax__blk412 * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_vmaxe__blk420, locals.var_vmaxe__blk420_dn0, locals.var_vmaxe__blk420_dn2, locals.var_vmaxe__blk420_dn4, locals.var_vmaxe__blk420_dn5, locals.var_vmaxe__blk420_dn6, locals.var_vmaxe__blk420_dn8, locals.var_vmaxe__blk420_dn10, locals.var_vmaxe__blk420_dn11, locals.var_vmaxe__blk420_dn12,)
    }
};
        locals.var_vmaxe__blk420 = assign23140_e28283;
        locals.var_vmaxe__blk420_dn0 = assign23140_e28283_d_n0;
        locals.var_vmaxe__blk420_dn2 = assign23140_e28283_d_n2;
        locals.var_vmaxe__blk420_dn4 = assign23140_e28283_d_n4;
        locals.var_vmaxe__blk420_dn5 = assign23140_e28283_d_n5;
        locals.var_vmaxe__blk420_dn6 = assign23140_e28283_d_n6;
        locals.var_vmaxe__blk420_dn8 = assign23140_e28283_d_n8;
        locals.var_vmaxe__blk420_dn10 = assign23140_e28283_d_n10;
        locals.var_vmaxe__blk420_dn11 = assign23140_e28283_d_n11;
        locals.var_vmaxe__blk420_dn12 = assign23140_e28283_d_n12;
        locals.var_vmaxe__blk420_rv = 0.0;

        let (assign23150_e28293, assign23150_e28293_d_n4,) = {
    if (locals.var_guard407 != 0.0) {
        let assign23150_e28289: f64 = (locals.var_ttemp - locals.var_uc_tnom);
        let assign23150_e28290: f64 = (p.p325 * assign23150_e28289);
        let assign23150_e28291: f64 = (locals.var_rrdrbb__blk413 + assign23150_e28290);
        (assign23150_e28291, (locals.var_rrdrbb__blk413_dn4 + (p.p325 * locals.var_ttemp_dn4)),)
    } else {
        (locals.var_rrdrbb__blk413, locals.var_rrdrbb__blk413_dn4,)
    }
};
        locals.var_rrdrbb__blk413 = assign23150_e28293;
        locals.var_rrdrbb__blk413_dn4 = assign23150_e28293_d_n4;
        locals.var_rrdrbb__blk413_rv = 0.0;

        let (assign23160_e28303,) = {
    if (locals.var_guard407 != 0.0) {
        let assign23160_e28299: f64 = (locals.var_lg).powf(p.p331);
        let assign23160_e28300: f64 = (p.p330 / assign23160_e28299);
        let assign23160_e28301: f64 = (1.0 + assign23160_e28300);
        (assign23160_e28301,)
    } else {
        (locals.var_rdrmuele__blk408,)
    }
};
        locals.var_rdrmuele__blk408 = assign23160_e28303;
        locals.var_rdrmuele__blk408_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_92(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23170_e28313,) = {
    if (locals.var_guard407 != 0.0) {
        let assign23170_e28309: f64 = (locals.var_lg).powf(p.p329);
        let assign23170_e28310: f64 = (p.p328 / assign23170_e28309);
        let assign23170_e28311: f64 = (1.0 + assign23170_e28310);
        (assign23170_e28311,)
    } else {
        (locals.var_rdrvmaxle__blk410,)
    }
};
        locals.var_rdrvmaxle__blk410 = assign23170_e28313;
        locals.var_rdrvmaxle__blk410_rv = 0.0;

        let (assign23180_e28323,) = {
    if (locals.var_guard407 != 0.0) {
        let assign23180_e28319: f64 = (locals.var_wg).powf(p.p327);
        let assign23180_e28320: f64 = (p.p326 / assign23180_e28319);
        let assign23180_e28321: f64 = (1.0 + assign23180_e28320);
        (assign23180_e28321,)
    } else {
        (locals.var_rdrvmaxwe__blk409,)
    }
};
        locals.var_rdrvmaxwe__blk409 = assign23180_e28323;
        locals.var_rdrvmaxwe__blk409_rv = 0.0;

        let (assign23190_e28329, assign23190_e28329_d_n0, assign23190_e28329_d_n2, assign23190_e28329_d_n4, assign23190_e28329_d_n5, assign23190_e28329_d_n6, assign23190_e28329_d_n8, assign23190_e28329_d_n10, assign23190_e28329_d_n11, assign23190_e28329_d_n12,) = {
    if (locals.var_guard407 != 0.0) {
        let assign23190_e28327: f64 = (locals.var_mu0__blk419 * locals.var_rdrmuele__blk408);
        (assign23190_e28327, (locals.var_mu0__blk419_dn0 * locals.var_rdrmuele__blk408), (locals.var_mu0__blk419_dn2 * locals.var_rdrmuele__blk408), (locals.var_mu0__blk419_dn4 * locals.var_rdrmuele__blk408), (locals.var_mu0__blk419_dn5 * locals.var_rdrmuele__blk408), (locals.var_mu0__blk419_dn6 * locals.var_rdrmuele__blk408), (locals.var_mu0__blk419_dn8 * locals.var_rdrmuele__blk408), (locals.var_mu0__blk419_dn10 * locals.var_rdrmuele__blk408), (locals.var_mu0__blk419_dn11 * locals.var_rdrmuele__blk408), (locals.var_mu0__blk419_dn12 * locals.var_rdrmuele__blk408),)
    } else {
        (locals.var_mu0__blk419, locals.var_mu0__blk419_dn0, locals.var_mu0__blk419_dn2, locals.var_mu0__blk419_dn4, locals.var_mu0__blk419_dn5, locals.var_mu0__blk419_dn6, locals.var_mu0__blk419_dn8, locals.var_mu0__blk419_dn10, locals.var_mu0__blk419_dn11, locals.var_mu0__blk419_dn12,)
    }
};
        locals.var_mu0__blk419 = assign23190_e28329;
        locals.var_mu0__blk419_dn0 = assign23190_e28329_d_n0;
        locals.var_mu0__blk419_dn2 = assign23190_e28329_d_n2;
        locals.var_mu0__blk419_dn4 = assign23190_e28329_d_n4;
        locals.var_mu0__blk419_dn5 = assign23190_e28329_d_n5;
        locals.var_mu0__blk419_dn6 = assign23190_e28329_d_n6;
        locals.var_mu0__blk419_dn8 = assign23190_e28329_d_n8;
        locals.var_mu0__blk419_dn10 = assign23190_e28329_d_n10;
        locals.var_mu0__blk419_dn11 = assign23190_e28329_d_n11;
        locals.var_mu0__blk419_dn12 = assign23190_e28329_d_n12;
        locals.var_mu0__blk419_rv = 0.0;

        let (assign23200_e28339, assign23200_e28339_d_n0, assign23200_e28339_d_n2, assign23200_e28339_d_n4, assign23200_e28339_d_n5, assign23200_e28339_d_n6, assign23200_e28339_d_n8, assign23200_e28339_d_n10, assign23200_e28339_d_n11, assign23200_e28339_d_n12,) = {
    if (locals.var_guard407 != 0.0) {
        let assign23200_e28333: f64 = (locals.var_vmaxe__blk420 * locals.var_rdrvmaxwe__blk409);
        let assign23200_e28335: f64 = (assign23200_e28333 * locals.var_rdrvmaxle__blk410);
        let assign23200_e28337: f64 = (assign23200_e28335 + 1e-50);
        (assign23200_e28337, ((locals.var_vmaxe__blk420_dn0 * locals.var_rdrvmaxwe__blk409) * locals.var_rdrvmaxle__blk410), ((locals.var_vmaxe__blk420_dn2 * locals.var_rdrvmaxwe__blk409) * locals.var_rdrvmaxle__blk410), ((locals.var_vmaxe__blk420_dn4 * locals.var_rdrvmaxwe__blk409) * locals.var_rdrvmaxle__blk410), ((locals.var_vmaxe__blk420_dn5 * locals.var_rdrvmaxwe__blk409) * locals.var_rdrvmaxle__blk410), ((locals.var_vmaxe__blk420_dn6 * locals.var_rdrvmaxwe__blk409) * locals.var_rdrvmaxle__blk410), ((locals.var_vmaxe__blk420_dn8 * locals.var_rdrvmaxwe__blk409) * locals.var_rdrvmaxle__blk410), ((locals.var_vmaxe__blk420_dn10 * locals.var_rdrvmaxwe__blk409) * locals.var_rdrvmaxle__blk410), ((locals.var_vmaxe__blk420_dn11 * locals.var_rdrvmaxwe__blk409) * locals.var_rdrvmaxle__blk410), ((locals.var_vmaxe__blk420_dn12 * locals.var_rdrvmaxwe__blk409) * locals.var_rdrvmaxle__blk410),)
    } else {
        (locals.var_vmaxe__blk420, locals.var_vmaxe__blk420_dn0, locals.var_vmaxe__blk420_dn2, locals.var_vmaxe__blk420_dn4, locals.var_vmaxe__blk420_dn5, locals.var_vmaxe__blk420_dn6, locals.var_vmaxe__blk420_dn8, locals.var_vmaxe__blk420_dn10, locals.var_vmaxe__blk420_dn11, locals.var_vmaxe__blk420_dn12,)
    }
};
        locals.var_vmaxe__blk420 = assign23200_e28339;
        locals.var_vmaxe__blk420_dn0 = assign23200_e28339_d_n0;
        locals.var_vmaxe__blk420_dn2 = assign23200_e28339_d_n2;
        locals.var_vmaxe__blk420_dn4 = assign23200_e28339_d_n4;
        locals.var_vmaxe__blk420_dn5 = assign23200_e28339_d_n5;
        locals.var_vmaxe__blk420_dn6 = assign23200_e28339_d_n6;
        locals.var_vmaxe__blk420_dn8 = assign23200_e28339_d_n8;
        locals.var_vmaxe__blk420_dn10 = assign23200_e28339_d_n10;
        locals.var_vmaxe__blk420_dn11 = assign23200_e28339_d_n11;
        locals.var_vmaxe__blk420_dn12 = assign23200_e28339_d_n12;
        locals.var_vmaxe__blk420_rv = 0.0;

        let (assign23210_e28345, assign23210_e28345_d_n0, assign23210_e28345_d_n11,) = {
    if (locals.var_guard407 != 0.0) {
        let assign23210_e28343: f64 = (locals.var_vrdr__blk415 / locals.var_ldrifte__blk417);
        (assign23210_e28343, (locals.var_vrdr__blk415_dn0 / locals.var_ldrifte__blk417), (locals.var_vrdr__blk415_dn11 / locals.var_ldrifte__blk417),)
    } else {
        (locals.var_edri__blk421, locals.var_edri__blk421_dn0, locals.var_edri__blk421_dn11,)
    }
};
        locals.var_edri__blk421 = assign23210_e28345;
        locals.var_edri__blk421_dn0 = assign23210_e28345_d_n0;
        locals.var_edri__blk421_dn11 = assign23210_e28345_d_n11;
        locals.var_edri__blk421_rv = 0.0;

        let (assign23220_e28351, assign23220_e28351_d_n0, assign23220_e28351_d_n2, assign23220_e28351_d_n4, assign23220_e28351_d_n5, assign23220_e28351_d_n6, assign23220_e28351_d_n8, assign23220_e28351_d_n10, assign23220_e28351_d_n11, assign23220_e28351_d_n12,) = {
    if (locals.var_guard407 != 0.0) {
        let assign23220_e28349: f64 = (locals.var_mu0__blk419 * locals.var_edri__blk421);
        (assign23220_e28349, ((locals.var_mu0__blk419_dn0 * locals.var_edri__blk421) + (locals.var_mu0__blk419 * locals.var_edri__blk421_dn0)), (locals.var_mu0__blk419_dn2 * locals.var_edri__blk421), (locals.var_mu0__blk419_dn4 * locals.var_edri__blk421), (locals.var_mu0__blk419_dn5 * locals.var_edri__blk421), (locals.var_mu0__blk419_dn6 * locals.var_edri__blk421), (locals.var_mu0__blk419_dn8 * locals.var_edri__blk421), (locals.var_mu0__blk419_dn10 * locals.var_edri__blk421), ((locals.var_mu0__blk419_dn11 * locals.var_edri__blk421) + (locals.var_mu0__blk419 * locals.var_edri__blk421_dn11)), (locals.var_mu0__blk419_dn12 * locals.var_edri__blk421),)
    } else {
        (locals.var_vdri__blk422, locals.var_vdri__blk422_dn0, locals.var_vdri__blk422_dn2, locals.var_vdri__blk422_dn4, locals.var_vdri__blk422_dn5, locals.var_vdri__blk422_dn6, locals.var_vdri__blk422_dn8, locals.var_vdri__blk422_dn10, locals.var_vdri__blk422_dn11, locals.var_vdri__blk422_dn12,)
    }
};
        locals.var_vdri__blk422 = assign23220_e28351;
        locals.var_vdri__blk422_dn0 = assign23220_e28351_d_n0;
        locals.var_vdri__blk422_dn2 = assign23220_e28351_d_n2;
        locals.var_vdri__blk422_dn4 = assign23220_e28351_d_n4;
        locals.var_vdri__blk422_dn5 = assign23220_e28351_d_n5;
        locals.var_vdri__blk422_dn6 = assign23220_e28351_d_n6;
        locals.var_vdri__blk422_dn8 = assign23220_e28351_d_n8;
        locals.var_vdri__blk422_dn10 = assign23220_e28351_d_n10;
        locals.var_vdri__blk422_dn11 = assign23220_e28351_d_n11;
        locals.var_vdri__blk422_dn12 = assign23220_e28351_d_n12;
        locals.var_vdri__blk422_rv = 0.0;

        let assign23230_e28354: f64 = if locals.var_vrdr__blk415 >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard427 = assign23230_e28354;
        locals.var_guard427_rv = 0.0;

        let (assign23240_e28362, assign23240_e28362_d_n0, assign23240_e28362_d_n2, assign23240_e28362_d_n4, assign23240_e28362_d_n5, assign23240_e28362_d_n6, assign23240_e28362_d_n8, assign23240_e28362_d_n10, assign23240_e28362_d_n11, assign23240_e28362_d_n12,) = {
    if ((locals.var_guard407 != 0.0) && (locals.var_guard427 != 0.0)) {
        let assign23240_e28360: f64 = (locals.var_vdri__blk422 / locals.var_vmaxe__blk420);
        (assign23240_e28360, (((locals.var_vdri__blk422_dn0 * locals.var_vmaxe__blk420) - (locals.var_vdri__blk422 * locals.var_vmaxe__blk420_dn0)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), (((locals.var_vdri__blk422_dn2 * locals.var_vmaxe__blk420) - (locals.var_vdri__blk422 * locals.var_vmaxe__blk420_dn2)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), (((locals.var_vdri__blk422_dn4 * locals.var_vmaxe__blk420) - (locals.var_vdri__blk422 * locals.var_vmaxe__blk420_dn4)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), (((locals.var_vdri__blk422_dn5 * locals.var_vmaxe__blk420) - (locals.var_vdri__blk422 * locals.var_vmaxe__blk420_dn5)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), (((locals.var_vdri__blk422_dn6 * locals.var_vmaxe__blk420) - (locals.var_vdri__blk422 * locals.var_vmaxe__blk420_dn6)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), (((locals.var_vdri__blk422_dn8 * locals.var_vmaxe__blk420) - (locals.var_vdri__blk422 * locals.var_vmaxe__blk420_dn8)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), (((locals.var_vdri__blk422_dn10 * locals.var_vmaxe__blk420) - (locals.var_vdri__blk422 * locals.var_vmaxe__blk420_dn10)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), (((locals.var_vdri__blk422_dn11 * locals.var_vmaxe__blk420) - (locals.var_vdri__blk422 * locals.var_vmaxe__blk420_dn11)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), (((locals.var_vdri__blk422_dn12 * locals.var_vmaxe__blk420) - (locals.var_vdri__blk422 * locals.var_vmaxe__blk420_dn12)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign23240_e28362;
        locals.var_t1_dn0 = assign23240_e28362_d_n0;
        locals.var_t1_dn2 = assign23240_e28362_d_n2;
        locals.var_t1_dn4 = assign23240_e28362_d_n4;
        locals.var_t1_dn5 = assign23240_e28362_d_n5;
        locals.var_t1_dn6 = assign23240_e28362_d_n6;
        locals.var_t1_dn8 = assign23240_e28362_d_n8;
        locals.var_t1_dn10 = assign23240_e28362_d_n10;
        locals.var_t1_dn11 = assign23240_e28362_d_n11;
        locals.var_t1_dn12 = assign23240_e28362_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign23250_e28372, assign23250_e28372_d_n0, assign23250_e28372_d_n2, assign23250_e28372_d_n4, assign23250_e28372_d_n5, assign23250_e28372_d_n6, assign23250_e28372_d_n8, assign23250_e28372_d_n10, assign23250_e28372_d_n11, assign23250_e28372_d_n12,) = {
    if ((locals.var_guard407 != 0.0) && (locals.var_guard427 == 0.0)) {
        let assign23250_e28368: f64 = (-locals.var_vdri__blk422);
        let assign23250_e28370: f64 = (assign23250_e28368 / locals.var_vmaxe__blk420);
        (assign23250_e28370, ((((-locals.var_vdri__blk422_dn0) * locals.var_vmaxe__blk420) - (assign23250_e28368 * locals.var_vmaxe__blk420_dn0)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), ((((-locals.var_vdri__blk422_dn2) * locals.var_vmaxe__blk420) - (assign23250_e28368 * locals.var_vmaxe__blk420_dn2)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), ((((-locals.var_vdri__blk422_dn4) * locals.var_vmaxe__blk420) - (assign23250_e28368 * locals.var_vmaxe__blk420_dn4)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), ((((-locals.var_vdri__blk422_dn5) * locals.var_vmaxe__blk420) - (assign23250_e28368 * locals.var_vmaxe__blk420_dn5)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), ((((-locals.var_vdri__blk422_dn6) * locals.var_vmaxe__blk420) - (assign23250_e28368 * locals.var_vmaxe__blk420_dn6)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), ((((-locals.var_vdri__blk422_dn8) * locals.var_vmaxe__blk420) - (assign23250_e28368 * locals.var_vmaxe__blk420_dn8)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), ((((-locals.var_vdri__blk422_dn10) * locals.var_vmaxe__blk420) - (assign23250_e28368 * locals.var_vmaxe__blk420_dn10)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), ((((-locals.var_vdri__blk422_dn11) * locals.var_vmaxe__blk420) - (assign23250_e28368 * locals.var_vmaxe__blk420_dn11)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)), ((((-locals.var_vdri__blk422_dn12) * locals.var_vmaxe__blk420) - (assign23250_e28368 * locals.var_vmaxe__blk420_dn12)) / (locals.var_vmaxe__blk420 * locals.var_vmaxe__blk420)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign23250_e28372;
        locals.var_t1_dn0 = assign23250_e28372_d_n0;
        locals.var_t1_dn2 = assign23250_e28372_d_n2;
        locals.var_t1_dn4 = assign23250_e28372_d_n4;
        locals.var_t1_dn5 = assign23250_e28372_d_n5;
        locals.var_t1_dn6 = assign23250_e28372_d_n6;
        locals.var_t1_dn8 = assign23250_e28372_d_n8;
        locals.var_t1_dn10 = assign23250_e28372_d_n10;
        locals.var_t1_dn11 = assign23250_e28372_d_n11;
        locals.var_t1_dn12 = assign23250_e28372_d_n12;
        locals.var_t1_rv = 0.0;

        let assign23260_e28376: f64 = (10.0 * 2.220446049250313e-16);
        let assign23260_e28377: f64 = (1.0 - assign23260_e28376);
        let assign23260_e28384: f64 = (10.0 * 2.220446049250313e-16);
        let assign23260_e28385: f64 = (1.0 + assign23260_e28384);
        let assign23260_e28387: f64 = if ((assign23260_e28377 <= locals.var_rrdrbb__blk413) && (locals.var_rrdrbb__blk413 <= assign23260_e28385)) { 1.0 } else { 0.0 };
        locals.var_guard428 = assign23260_e28387;
        locals.var_guard428_rv = 0.0;

        let (assign23270_e28393, assign23270_e28393_d_n0, assign23270_e28393_d_n2, assign23270_e28393_d_n4, assign23270_e28393_d_n5, assign23270_e28393_d_n6, assign23270_e28393_d_n8, assign23270_e28393_d_n10, assign23270_e28393_d_n11, assign23270_e28393_d_n12,) = {
    if ((locals.var_guard407 != 0.0) && (locals.var_guard428 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign23270_e28393;
        locals.var_t3_dn0 = assign23270_e28393_d_n0;
        locals.var_t3_dn2 = assign23270_e28393_d_n2;
        locals.var_t3_dn4 = assign23270_e28393_d_n4;
        locals.var_t3_dn5 = assign23270_e28393_d_n5;
        locals.var_t3_dn6 = assign23270_e28393_d_n6;
        locals.var_t3_dn8 = assign23270_e28393_d_n8;
        locals.var_t3_dn10 = assign23270_e28393_d_n10;
        locals.var_t3_dn11 = assign23270_e28393_d_n11;
        locals.var_t3_dn12 = assign23270_e28393_d_n12;
        locals.var_t3_rv = 0.0;

        let assign23280_e28397: f64 = (10.0 * 2.220446049250313e-16);
        let assign23280_e28398: f64 = (2.0 - assign23280_e28397);
        let assign23280_e28405: f64 = (10.0 * 2.220446049250313e-16);
        let assign23280_e28406: f64 = (2.0 + assign23280_e28405);
        let assign23280_e28408: f64 = if ((assign23280_e28398 <= locals.var_rrdrbb__blk413) && (locals.var_rrdrbb__blk413 <= assign23280_e28406)) { 1.0 } else { 0.0 };
        locals.var_guard429 = assign23280_e28408;
        locals.var_guard429_rv = 0.0;

        let (assign23290_e28417, assign23290_e28417_d_n0, assign23290_e28417_d_n2, assign23290_e28417_d_n4, assign23290_e28417_d_n5, assign23290_e28417_d_n6, assign23290_e28417_d_n8, assign23290_e28417_d_n10, assign23290_e28417_d_n11, assign23290_e28417_d_n12,) = {
    if (((locals.var_guard407 != 0.0) && (locals.var_guard428 == 0.0)) && (locals.var_guard429 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign23290_e28417;
        locals.var_t3_dn0 = assign23290_e28417_d_n0;
        locals.var_t3_dn2 = assign23290_e28417_d_n2;
        locals.var_t3_dn4 = assign23290_e28417_d_n4;
        locals.var_t3_dn5 = assign23290_e28417_d_n5;
        locals.var_t3_dn6 = assign23290_e28417_d_n6;
        locals.var_t3_dn8 = assign23290_e28417_d_n8;
        locals.var_t3_dn10 = assign23290_e28417_d_n10;
        locals.var_t3_dn11 = assign23290_e28417_d_n11;
        locals.var_t3_dn12 = assign23290_e28417_d_n12;
        locals.var_t3_rv = 0.0;

        let (assign23300_e28431, assign23300_e28431_d_n0, assign23300_e28431_d_n2, assign23300_e28431_d_n4, assign23300_e28431_d_n5, assign23300_e28431_d_n6, assign23300_e28431_d_n8, assign23300_e28431_d_n10, assign23300_e28431_d_n11, assign23300_e28431_d_n12,) = {
    if (((locals.var_guard407 != 0.0) && (locals.var_guard428 == 0.0)) && (locals.var_guard429 == 0.0)) {
        let assign23300_e28428: f64 = (locals.var_rrdrbb__blk413 - 1.0);
        let assign23300_e28429: f64 = (locals.var_t1).powf(assign23300_e28428);
        (assign23300_e28429, if 0.0 == 0.0 && ((assign23300_e28428) as f64).is_finite() && ((assign23300_e28428) as f64).fract() == 0.0 { if assign23300_e28428 == 0.0 { 0.0 } else { (assign23300_e28428 * ((locals.var_t1).powf(assign23300_e28428 - 1.0) * locals.var_t1_dn0)) } } else { (assign23300_e28429 * (assign23300_e28428 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign23300_e28428) as f64).is_finite() && ((assign23300_e28428) as f64).fract() == 0.0 { if assign23300_e28428 == 0.0 { 0.0 } else { (assign23300_e28428 * ((locals.var_t1).powf(assign23300_e28428 - 1.0) * locals.var_t1_dn2)) } } else { (assign23300_e28429 * (assign23300_e28428 * (locals.var_t1_dn2 / locals.var_t1))) }, if locals.var_rrdrbb__blk413_dn4 == 0.0 && ((assign23300_e28428) as f64).is_finite() && ((assign23300_e28428) as f64).fract() == 0.0 { if assign23300_e28428 == 0.0 { 0.0 } else { (assign23300_e28428 * ((locals.var_t1).powf(assign23300_e28428 - 1.0) * locals.var_t1_dn4)) } } else { (assign23300_e28429 * ((locals.var_rrdrbb__blk413_dn4 * (locals.var_t1).ln()) + (assign23300_e28428 * (locals.var_t1_dn4 / locals.var_t1)))) }, if 0.0 == 0.0 && ((assign23300_e28428) as f64).is_finite() && ((assign23300_e28428) as f64).fract() == 0.0 { if assign23300_e28428 == 0.0 { 0.0 } else { (assign23300_e28428 * ((locals.var_t1).powf(assign23300_e28428 - 1.0) * locals.var_t1_dn5)) } } else { (assign23300_e28429 * (assign23300_e28428 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign23300_e28428) as f64).is_finite() && ((assign23300_e28428) as f64).fract() == 0.0 { if assign23300_e28428 == 0.0 { 0.0 } else { (assign23300_e28428 * ((locals.var_t1).powf(assign23300_e28428 - 1.0) * locals.var_t1_dn6)) } } else { (assign23300_e28429 * (assign23300_e28428 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign23300_e28428) as f64).is_finite() && ((assign23300_e28428) as f64).fract() == 0.0 { if assign23300_e28428 == 0.0 { 0.0 } else { (assign23300_e28428 * ((locals.var_t1).powf(assign23300_e28428 - 1.0) * locals.var_t1_dn8)) } } else { (assign23300_e28429 * (assign23300_e28428 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign23300_e28428) as f64).is_finite() && ((assign23300_e28428) as f64).fract() == 0.0 { if assign23300_e28428 == 0.0 { 0.0 } else { (assign23300_e28428 * ((locals.var_t1).powf(assign23300_e28428 - 1.0) * locals.var_t1_dn10)) } } else { (assign23300_e28429 * (assign23300_e28428 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign23300_e28428) as f64).is_finite() && ((assign23300_e28428) as f64).fract() == 0.0 { if assign23300_e28428 == 0.0 { 0.0 } else { (assign23300_e28428 * ((locals.var_t1).powf(assign23300_e28428 - 1.0) * locals.var_t1_dn11)) } } else { (assign23300_e28429 * (assign23300_e28428 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign23300_e28428) as f64).is_finite() && ((assign23300_e28428) as f64).fract() == 0.0 { if assign23300_e28428 == 0.0 { 0.0 } else { (assign23300_e28428 * ((locals.var_t1).powf(assign23300_e28428 - 1.0) * locals.var_t1_dn12)) } } else { (assign23300_e28429 * (assign23300_e28428 * (locals.var_t1_dn12 / locals.var_t1))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign23300_e28431;
        locals.var_t3_dn0 = assign23300_e28431_d_n0;
        locals.var_t3_dn2 = assign23300_e28431_d_n2;
        locals.var_t3_dn4 = assign23300_e28431_d_n4;
        locals.var_t3_dn5 = assign23300_e28431_d_n5;
        locals.var_t3_dn6 = assign23300_e28431_d_n6;
        locals.var_t3_dn8 = assign23300_e28431_d_n8;
        locals.var_t3_dn10 = assign23300_e28431_d_n10;
        locals.var_t3_dn11 = assign23300_e28431_d_n11;
        locals.var_t3_dn12 = assign23300_e28431_d_n12;
        locals.var_t3_rv = 0.0;

        let (assign23310_e28437, assign23310_e28437_d_n0, assign23310_e28437_d_n2, assign23310_e28437_d_n4, assign23310_e28437_d_n5, assign23310_e28437_d_n6, assign23310_e28437_d_n8, assign23310_e28437_d_n10, assign23310_e28437_d_n11, assign23310_e28437_d_n12,) = {
    if (locals.var_guard407 != 0.0) {
        let assign23310_e28435: f64 = (locals.var_t1 * locals.var_t3);
        (assign23310_e28435, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn12 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn12)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign23310_e28437;
        locals.var_t2_dn0 = assign23310_e28437_d_n0;
        locals.var_t2_dn2 = assign23310_e28437_d_n2;
        locals.var_t2_dn4 = assign23310_e28437_d_n4;
        locals.var_t2_dn5 = assign23310_e28437_d_n5;
        locals.var_t2_dn6 = assign23310_e28437_d_n6;
        locals.var_t2_dn8 = assign23310_e28437_d_n8;
        locals.var_t2_dn10 = assign23310_e28437_d_n10;
        locals.var_t2_dn11 = assign23310_e28437_d_n11;
        locals.var_t2_dn12 = assign23310_e28437_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign23320_e28443, assign23320_e28443_d_n0, assign23320_e28443_d_n2, assign23320_e28443_d_n4, assign23320_e28443_d_n5, assign23320_e28443_d_n6, assign23320_e28443_d_n8, assign23320_e28443_d_n10, assign23320_e28443_d_n11, assign23320_e28443_d_n12,) = {
    if (locals.var_guard407 != 0.0) {
        let assign23320_e28441: f64 = (1.0 + locals.var_t2);
        (assign23320_e28441, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn8, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign23320_e28443;
        locals.var_t4_dn0 = assign23320_e28443_d_n0;
        locals.var_t4_dn2 = assign23320_e28443_d_n2;
        locals.var_t4_dn4 = assign23320_e28443_d_n4;
        locals.var_t4_dn5 = assign23320_e28443_d_n5;
        locals.var_t4_dn6 = assign23320_e28443_d_n6;
        locals.var_t4_dn8 = assign23320_e28443_d_n8;
        locals.var_t4_dn10 = assign23320_e28443_d_n10;
        locals.var_t4_dn11 = assign23320_e28443_d_n11;
        locals.var_t4_dn12 = assign23320_e28443_d_n12;
        locals.var_t4_rv = 0.0;

        let assign23330_e28447: f64 = (10.0 * 2.220446049250313e-16);
        let assign23330_e28448: f64 = (1.0 - assign23330_e28447);
        let assign23330_e28455: f64 = (10.0 * 2.220446049250313e-16);
        let assign23330_e28456: f64 = (1.0 + assign23330_e28455);
        let assign23330_e28458: f64 = if ((assign23330_e28448 <= locals.var_rrdrbb__blk413) && (locals.var_rrdrbb__blk413 <= assign23330_e28456)) { 1.0 } else { 0.0 };
        locals.var_guard430 = assign23330_e28458;
        locals.var_guard430_rv = 0.0;

        let (assign23340_e28466, assign23340_e28466_d_n0, assign23340_e28466_d_n2, assign23340_e28466_d_n4, assign23340_e28466_d_n5, assign23340_e28466_d_n6, assign23340_e28466_d_n8, assign23340_e28466_d_n10, assign23340_e28466_d_n11, assign23340_e28466_d_n12,) = {
    if ((locals.var_guard407 != 0.0) && (locals.var_guard430 != 0.0)) {
        let assign23340_e28464: f64 = (1.0 / locals.var_t4);
        (assign23340_e28464, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn12 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
        locals.var_t5 = assign23340_e28466;
        locals.var_t5_dn0 = assign23340_e28466_d_n0;
        locals.var_t5_dn2 = assign23340_e28466_d_n2;
        locals.var_t5_dn4 = assign23340_e28466_d_n4;
        locals.var_t5_dn5 = assign23340_e28466_d_n5;
        locals.var_t5_dn6 = assign23340_e28466_d_n6;
        locals.var_t5_dn8 = assign23340_e28466_d_n8;
        locals.var_t5_dn10 = assign23340_e28466_d_n10;
        locals.var_t5_dn11 = assign23340_e28466_d_n11;
        locals.var_t5_dn12 = assign23340_e28466_d_n12;
        locals.var_t5_rv = 0.0;

        let assign23350_e28470: f64 = (10.0 * 2.220446049250313e-16);
        let assign23350_e28471: f64 = (2.0 - assign23350_e28470);
        let assign23350_e28478: f64 = (10.0 * 2.220446049250313e-16);
        let assign23350_e28479: f64 = (2.0 + assign23350_e28478);
        let assign23350_e28481: f64 = if ((assign23350_e28471 <= locals.var_rrdrbb__blk413) && (locals.var_rrdrbb__blk413 <= assign23350_e28479)) { 1.0 } else { 0.0 };
        locals.var_guard431 = assign23350_e28481;
        locals.var_guard431_rv = 0.0;

        let (assign23360_e28493, assign23360_e28493_d_n0, assign23360_e28493_d_n2, assign23360_e28493_d_n4, assign23360_e28493_d_n5, assign23360_e28493_d_n6, assign23360_e28493_d_n8, assign23360_e28493_d_n10, assign23360_e28493_d_n11, assign23360_e28493_d_n12,) = {
    if (((locals.var_guard407 != 0.0) && (locals.var_guard430 == 0.0)) && (locals.var_guard431 != 0.0)) {
        let assign23360_e28490: f64 = (locals.var_t4).sqrt();
        let assign23360_e28491: f64 = (1.0 / assign23360_e28490);
        (assign23360_e28491, (-((locals.var_t4_dn0 / (2.0 * assign23360_e28490)) / (assign23360_e28490 * assign23360_e28490))), (-((locals.var_t4_dn2 / (2.0 * assign23360_e28490)) / (assign23360_e28490 * assign23360_e28490))), (-((locals.var_t4_dn4 / (2.0 * assign23360_e28490)) / (assign23360_e28490 * assign23360_e28490))), (-((locals.var_t4_dn5 / (2.0 * assign23360_e28490)) / (assign23360_e28490 * assign23360_e28490))), (-((locals.var_t4_dn6 / (2.0 * assign23360_e28490)) / (assign23360_e28490 * assign23360_e28490))), (-((locals.var_t4_dn8 / (2.0 * assign23360_e28490)) / (assign23360_e28490 * assign23360_e28490))), (-((locals.var_t4_dn10 / (2.0 * assign23360_e28490)) / (assign23360_e28490 * assign23360_e28490))), (-((locals.var_t4_dn11 / (2.0 * assign23360_e28490)) / (assign23360_e28490 * assign23360_e28490))), (-((locals.var_t4_dn12 / (2.0 * assign23360_e28490)) / (assign23360_e28490 * assign23360_e28490))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
        locals.var_t5 = assign23360_e28493;
        locals.var_t5_dn0 = assign23360_e28493_d_n0;
        locals.var_t5_dn2 = assign23360_e28493_d_n2;
        locals.var_t5_dn4 = assign23360_e28493_d_n4;
        locals.var_t5_dn5 = assign23360_e28493_d_n5;
        locals.var_t5_dn6 = assign23360_e28493_d_n6;
        locals.var_t5_dn8 = assign23360_e28493_d_n8;
        locals.var_t5_dn10 = assign23360_e28493_d_n10;
        locals.var_t5_dn11 = assign23360_e28493_d_n11;
        locals.var_t5_dn12 = assign23360_e28493_d_n12;
        locals.var_t5_rv = 0.0;

        let (assign23370_e28510, assign23370_e28510_d_n0, assign23370_e28510_d_n2, assign23370_e28510_d_n4, assign23370_e28510_d_n5, assign23370_e28510_d_n6, assign23370_e28510_d_n8, assign23370_e28510_d_n10, assign23370_e28510_d_n11, assign23370_e28510_d_n12,) = {
    if (((locals.var_guard407 != 0.0) && (locals.var_guard430 == 0.0)) && (locals.var_guard431 == 0.0)) {
        let assign23370_e28503: f64 = (-1.0);
        let assign23370_e28505: f64 = (assign23370_e28503 / locals.var_rrdrbb__blk413);
        let assign23370_e28507: f64 = (assign23370_e28505 - 1.0);
        let assign23370_e28508: f64 = (locals.var_t4).powf(assign23370_e28507);
        (assign23370_e28508, if 0.0 == 0.0 && ((assign23370_e28507) as f64).is_finite() && ((assign23370_e28507) as f64).fract() == 0.0 { if assign23370_e28507 == 0.0 { 0.0 } else { (assign23370_e28507 * ((locals.var_t4).powf(assign23370_e28507 - 1.0) * locals.var_t4_dn0)) } } else { (assign23370_e28508 * (assign23370_e28507 * (locals.var_t4_dn0 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign23370_e28507) as f64).is_finite() && ((assign23370_e28507) as f64).fract() == 0.0 { if assign23370_e28507 == 0.0 { 0.0 } else { (assign23370_e28507 * ((locals.var_t4).powf(assign23370_e28507 - 1.0) * locals.var_t4_dn2)) } } else { (assign23370_e28508 * (assign23370_e28507 * (locals.var_t4_dn2 / locals.var_t4))) }, if (-((assign23370_e28503 * locals.var_rrdrbb__blk413_dn4) / (locals.var_rrdrbb__blk413 * locals.var_rrdrbb__blk413))) == 0.0 && ((assign23370_e28507) as f64).is_finite() && ((assign23370_e28507) as f64).fract() == 0.0 { if assign23370_e28507 == 0.0 { 0.0 } else { (assign23370_e28507 * ((locals.var_t4).powf(assign23370_e28507 - 1.0) * locals.var_t4_dn4)) } } else { (assign23370_e28508 * (((-((assign23370_e28503 * locals.var_rrdrbb__blk413_dn4) / (locals.var_rrdrbb__blk413 * locals.var_rrdrbb__blk413))) * (locals.var_t4).ln()) + (assign23370_e28507 * (locals.var_t4_dn4 / locals.var_t4)))) }, if 0.0 == 0.0 && ((assign23370_e28507) as f64).is_finite() && ((assign23370_e28507) as f64).fract() == 0.0 { if assign23370_e28507 == 0.0 { 0.0 } else { (assign23370_e28507 * ((locals.var_t4).powf(assign23370_e28507 - 1.0) * locals.var_t4_dn5)) } } else { (assign23370_e28508 * (assign23370_e28507 * (locals.var_t4_dn5 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign23370_e28507) as f64).is_finite() && ((assign23370_e28507) as f64).fract() == 0.0 { if assign23370_e28507 == 0.0 { 0.0 } else { (assign23370_e28507 * ((locals.var_t4).powf(assign23370_e28507 - 1.0) * locals.var_t4_dn6)) } } else { (assign23370_e28508 * (assign23370_e28507 * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign23370_e28507) as f64).is_finite() && ((assign23370_e28507) as f64).fract() == 0.0 { if assign23370_e28507 == 0.0 { 0.0 } else { (assign23370_e28507 * ((locals.var_t4).powf(assign23370_e28507 - 1.0) * locals.var_t4_dn8)) } } else { (assign23370_e28508 * (assign23370_e28507 * (locals.var_t4_dn8 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign23370_e28507) as f64).is_finite() && ((assign23370_e28507) as f64).fract() == 0.0 { if assign23370_e28507 == 0.0 { 0.0 } else { (assign23370_e28507 * ((locals.var_t4).powf(assign23370_e28507 - 1.0) * locals.var_t4_dn10)) } } else { (assign23370_e28508 * (assign23370_e28507 * (locals.var_t4_dn10 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign23370_e28507) as f64).is_finite() && ((assign23370_e28507) as f64).fract() == 0.0 { if assign23370_e28507 == 0.0 { 0.0 } else { (assign23370_e28507 * ((locals.var_t4).powf(assign23370_e28507 - 1.0) * locals.var_t4_dn11)) } } else { (assign23370_e28508 * (assign23370_e28507 * (locals.var_t4_dn11 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign23370_e28507) as f64).is_finite() && ((assign23370_e28507) as f64).fract() == 0.0 { if assign23370_e28507 == 0.0 { 0.0 } else { (assign23370_e28507 * ((locals.var_t4).powf(assign23370_e28507 - 1.0) * locals.var_t4_dn12)) } } else { (assign23370_e28508 * (assign23370_e28507 * (locals.var_t4_dn12 / locals.var_t4))) },)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn8, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12,)
    }
};
        locals.var_t6 = assign23370_e28510;
        locals.var_t6_dn0 = assign23370_e28510_d_n0;
        locals.var_t6_dn2 = assign23370_e28510_d_n2;
        locals.var_t6_dn4 = assign23370_e28510_d_n4;
        locals.var_t6_dn5 = assign23370_e28510_d_n5;
        locals.var_t6_dn6 = assign23370_e28510_d_n6;
        locals.var_t6_dn8 = assign23370_e28510_d_n8;
        locals.var_t6_dn10 = assign23370_e28510_d_n10;
        locals.var_t6_dn11 = assign23370_e28510_d_n11;
        locals.var_t6_dn12 = assign23370_e28510_d_n12;
        locals.var_t6_rv = 0.0;

        let (assign23380_e28522, assign23380_e28522_d_n0, assign23380_e28522_d_n2, assign23380_e28522_d_n4, assign23380_e28522_d_n5, assign23380_e28522_d_n6, assign23380_e28522_d_n8, assign23380_e28522_d_n10, assign23380_e28522_d_n11, assign23380_e28522_d_n12,) = {
    if (((locals.var_guard407 != 0.0) && (locals.var_guard430 == 0.0)) && (locals.var_guard431 == 0.0)) {
        let assign23380_e28520: f64 = (locals.var_t4 * locals.var_t6);
        (assign23380_e28520, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn4 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn4)), ((locals.var_t4_dn5 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn5)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn8 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn8)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn11 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn11)), ((locals.var_t4_dn12 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn12)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
        locals.var_t5 = assign23380_e28522;
        locals.var_t5_dn0 = assign23380_e28522_d_n0;
        locals.var_t5_dn2 = assign23380_e28522_d_n2;
        locals.var_t5_dn4 = assign23380_e28522_d_n4;
        locals.var_t5_dn5 = assign23380_e28522_d_n5;
        locals.var_t5_dn6 = assign23380_e28522_d_n6;
        locals.var_t5_dn8 = assign23380_e28522_d_n8;
        locals.var_t5_dn10 = assign23380_e28522_d_n10;
        locals.var_t5_dn11 = assign23380_e28522_d_n11;
        locals.var_t5_dn12 = assign23380_e28522_d_n12;
        locals.var_t5_rv = 0.0;

        let (assign23400_e28534, assign23400_e28534_d_n0, assign23400_e28534_d_n2, assign23400_e28534_d_n4, assign23400_e28534_d_n5, assign23400_e28534_d_n6, assign23400_e28534_d_n8, assign23400_e28534_d_n10, assign23400_e28534_d_n11, assign23400_e28534_d_n12,) = {
    if (locals.var_guard407 != 0.0) {
        let assign23400_e28532: f64 = (1.6021918e-19 / locals.var_ldrifte__blk417);
        (assign23400_e28532, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign23400_e28534;
        locals.var_t1_dn0 = assign23400_e28534_d_n0;
        locals.var_t1_dn2 = assign23400_e28534_d_n2;
        locals.var_t1_dn4 = assign23400_e28534_d_n4;
        locals.var_t1_dn5 = assign23400_e28534_d_n5;
        locals.var_t1_dn6 = assign23400_e28534_d_n6;
        locals.var_t1_dn8 = assign23400_e28534_d_n8;
        locals.var_t1_dn10 = assign23400_e28534_d_n10;
        locals.var_t1_dn11 = assign23400_e28534_d_n11;
        locals.var_t1_dn12 = assign23400_e28534_d_n12;
        locals.var_t1_rv = 0.0;

        let assign23520_e28608: f64 = if locals.var_tau < 1e-18 { 1.0 } else { 0.0 };
        locals.var_guard434 = assign23520_e28608;
        locals.var_guard434_rv = 0.0;

        let (assign23530_e28614, assign23530_e28614_d_n0, assign23530_e28614_d_n2, assign23530_e28614_d_n4, assign23530_e28614_d_n5, assign23530_e28614_d_n6, assign23530_e28614_d_n8, assign23530_e28614_d_n10, assign23530_e28614_d_n11, assign23530_e28614_d_n12,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard434 != 0.0)) {
        (1e-18, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tau, locals.var_tau_dn0, locals.var_tau_dn2, locals.var_tau_dn4, locals.var_tau_dn5, locals.var_tau_dn6, locals.var_tau_dn8, locals.var_tau_dn10, locals.var_tau_dn11, locals.var_tau_dn12,)
    }
};
        locals.var_tau = assign23530_e28614;
        locals.var_tau_dn0 = assign23530_e28614_d_n0;
        locals.var_tau_dn2 = assign23530_e28614_d_n2;
        locals.var_tau_dn4 = assign23530_e28614_d_n4;
        locals.var_tau_dn5 = assign23530_e28614_d_n5;
        locals.var_tau_dn6 = assign23530_e28614_d_n6;
        locals.var_tau_dn8 = assign23530_e28614_d_n8;
        locals.var_tau_dn10 = assign23530_e28614_d_n10;
        locals.var_tau_dn11 = assign23530_e28614_d_n11;
        locals.var_tau_dn12 = assign23530_e28614_d_n12;
        locals.var_tau_rv = 0.0;

        let assign23540_e28617: f64 = if locals.var_taub < 1e-18 { 1.0 } else { 0.0 };
        locals.var_guard435 = assign23540_e28617;
        locals.var_guard435_rv = 0.0;

        let (assign23550_e28623, assign23550_e28623_d_n0, assign23550_e28623_d_n2, assign23550_e28623_d_n4, assign23550_e28623_d_n5, assign23550_e28623_d_n6, assign23550_e28623_d_n8, assign23550_e28623_d_n10, assign23550_e28623_d_n11, assign23550_e28623_d_n12,) = {
    if ((locals.var_flg_nqs != 0.0) && (locals.var_guard435 != 0.0)) {
        (1e-18, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_taub, locals.var_taub_dn0, locals.var_taub_dn2, locals.var_taub_dn4, locals.var_taub_dn5, locals.var_taub_dn6, locals.var_taub_dn8, locals.var_taub_dn10, locals.var_taub_dn11, locals.var_taub_dn12,)
    }
};
        locals.var_taub = assign23550_e28623;
        locals.var_taub_dn0 = assign23550_e28623_d_n0;
        locals.var_taub_dn2 = assign23550_e28623_d_n2;
        locals.var_taub_dn4 = assign23550_e28623_d_n4;
        locals.var_taub_dn5 = assign23550_e28623_d_n5;
        locals.var_taub_dn6 = assign23550_e28623_d_n6;
        locals.var_taub_dn8 = assign23550_e28623_d_n8;
        locals.var_taub_dn10 = assign23550_e28623_d_n10;
        locals.var_taub_dn11 = assign23550_e28623_d_n11;
        locals.var_taub_dn12 = assign23550_e28623_d_n12;
        locals.var_taub_rv = 0.0;

        let (assign23560_e28631, assign23560_e28631_d_n0, assign23560_e28631_d_n2, assign23560_e28631_d_n4, assign23560_e28631_d_n5, assign23560_e28631_d_n6, assign23560_e28631_d_n8, assign23560_e28631_d_n10, assign23560_e28631_d_n11, assign23560_e28631_d_n12,) = {
    if (locals.var_flg_nqs != 0.0) {
        let assign23560_e28627: f64 = (locals.var_qi_nqs - locals.var_qi_qs);
        let assign23560_e28629: f64 = (assign23560_e28627 / locals.var_tau);
        (assign23560_e28629, ((((-locals.var_qi_qs_dn0) * locals.var_tau) - (assign23560_e28627 * locals.var_tau_dn0)) / (locals.var_tau * locals.var_tau)), ((((-locals.var_qi_qs_dn2) * locals.var_tau) - (assign23560_e28627 * locals.var_tau_dn2)) / (locals.var_tau * locals.var_tau)), ((((-locals.var_qi_qs_dn4) * locals.var_tau) - (assign23560_e28627 * locals.var_tau_dn4)) / (locals.var_tau * locals.var_tau)), ((((-locals.var_qi_qs_dn5) * locals.var_tau) - (assign23560_e28627 * locals.var_tau_dn5)) / (locals.var_tau * locals.var_tau)), ((((-locals.var_qi_qs_dn6) * locals.var_tau) - (assign23560_e28627 * locals.var_tau_dn6)) / (locals.var_tau * locals.var_tau)), ((((locals.var_qi_nqs_dn8 - locals.var_qi_qs_dn8) * locals.var_tau) - (assign23560_e28627 * locals.var_tau_dn8)) / (locals.var_tau * locals.var_tau)), ((((-locals.var_qi_qs_dn10) * locals.var_tau) - (assign23560_e28627 * locals.var_tau_dn10)) / (locals.var_tau * locals.var_tau)), ((((-locals.var_qi_qs_dn11) * locals.var_tau) - (assign23560_e28627 * locals.var_tau_dn11)) / (locals.var_tau * locals.var_tau)), ((((-locals.var_qi_qs_dn12) * locals.var_tau) - (assign23560_e28627 * locals.var_tau_dn12)) / (locals.var_tau * locals.var_tau)),)
    } else {
        (locals.var_iqi_nqs, locals.var_iqi_nqs_dn0, locals.var_iqi_nqs_dn2, locals.var_iqi_nqs_dn4, locals.var_iqi_nqs_dn5, locals.var_iqi_nqs_dn6, locals.var_iqi_nqs_dn8, locals.var_iqi_nqs_dn10, locals.var_iqi_nqs_dn11, locals.var_iqi_nqs_dn12,)
    }
};
        locals.var_iqi_nqs = assign23560_e28631;
        locals.var_iqi_nqs_dn0 = assign23560_e28631_d_n0;
        locals.var_iqi_nqs_dn2 = assign23560_e28631_d_n2;
        locals.var_iqi_nqs_dn4 = assign23560_e28631_d_n4;
        locals.var_iqi_nqs_dn5 = assign23560_e28631_d_n5;
        locals.var_iqi_nqs_dn6 = assign23560_e28631_d_n6;
        locals.var_iqi_nqs_dn8 = assign23560_e28631_d_n8;
        locals.var_iqi_nqs_dn10 = assign23560_e28631_d_n10;
        locals.var_iqi_nqs_dn11 = assign23560_e28631_d_n11;
        locals.var_iqi_nqs_dn12 = assign23560_e28631_d_n12;
        locals.var_iqi_nqs_rv = 0.0;

        let (assign23570_e28639, assign23570_e28639_d_n0, assign23570_e28639_d_n2, assign23570_e28639_d_n4, assign23570_e28639_d_n5, assign23570_e28639_d_n6, assign23570_e28639_d_n8, assign23570_e28639_d_n9, assign23570_e28639_d_n10, assign23570_e28639_d_n11, assign23570_e28639_d_n12,) = {
    if (locals.var_flg_nqs != 0.0) {
        let assign23570_e28635: f64 = (locals.var_qb_nqs - locals.var_qb_qs);
        let assign23570_e28637: f64 = (assign23570_e28635 / locals.var_taub);
        (assign23570_e28637, ((((-locals.var_qb_qs_dn0) * locals.var_taub) - (assign23570_e28635 * locals.var_taub_dn0)) / (locals.var_taub * locals.var_taub)), ((((-locals.var_qb_qs_dn2) * locals.var_taub) - (assign23570_e28635 * locals.var_taub_dn2)) / (locals.var_taub * locals.var_taub)), ((((-locals.var_qb_qs_dn4) * locals.var_taub) - (assign23570_e28635 * locals.var_taub_dn4)) / (locals.var_taub * locals.var_taub)), ((((-locals.var_qb_qs_dn5) * locals.var_taub) - (assign23570_e28635 * locals.var_taub_dn5)) / (locals.var_taub * locals.var_taub)), ((((-locals.var_qb_qs_dn6) * locals.var_taub) - (assign23570_e28635 * locals.var_taub_dn6)) / (locals.var_taub * locals.var_taub)), ((((-locals.var_qb_qs_dn8) * locals.var_taub) - (assign23570_e28635 * locals.var_taub_dn8)) / (locals.var_taub * locals.var_taub)), (locals.var_qb_nqs_dn9 / locals.var_taub), ((((-locals.var_qb_qs_dn10) * locals.var_taub) - (assign23570_e28635 * locals.var_taub_dn10)) / (locals.var_taub * locals.var_taub)), ((((-locals.var_qb_qs_dn11) * locals.var_taub) - (assign23570_e28635 * locals.var_taub_dn11)) / (locals.var_taub * locals.var_taub)), ((((-locals.var_qb_qs_dn12) * locals.var_taub) - (assign23570_e28635 * locals.var_taub_dn12)) / (locals.var_taub * locals.var_taub)),)
    } else {
        (locals.var_iqb_nqs, locals.var_iqb_nqs_dn0, locals.var_iqb_nqs_dn2, locals.var_iqb_nqs_dn4, locals.var_iqb_nqs_dn5, locals.var_iqb_nqs_dn6, locals.var_iqb_nqs_dn8, locals.var_iqb_nqs_dn9, locals.var_iqb_nqs_dn10, locals.var_iqb_nqs_dn11, locals.var_iqb_nqs_dn12,)
    }
};
        locals.var_iqb_nqs = assign23570_e28639;
        locals.var_iqb_nqs_dn0 = assign23570_e28639_d_n0;
        locals.var_iqb_nqs_dn2 = assign23570_e28639_d_n2;
        locals.var_iqb_nqs_dn4 = assign23570_e28639_d_n4;
        locals.var_iqb_nqs_dn5 = assign23570_e28639_d_n5;
        locals.var_iqb_nqs_dn6 = assign23570_e28639_d_n6;
        locals.var_iqb_nqs_dn8 = assign23570_e28639_d_n8;
        locals.var_iqb_nqs_dn9 = assign23570_e28639_d_n9;
        locals.var_iqb_nqs_dn10 = assign23570_e28639_d_n10;
        locals.var_iqb_nqs_dn11 = assign23570_e28639_d_n11;
        locals.var_iqb_nqs_dn12 = assign23570_e28639_d_n12;
        locals.var_iqb_nqs_rv = 0.0;

        let (assign23580_e28646, assign23580_e28646_d_n8, assign23580_e28646_d_n9,) = {
    if (locals.var_flg_nqs != 0.0) {
        let assign23580_e28642: f64 = (-locals.var_qi_nqs);
        let assign23580_e28644: f64 = (assign23580_e28642 - locals.var_qb_nqs);
        (assign23580_e28644, (-locals.var_qi_nqs_dn8), (-locals.var_qb_nqs_dn9),)
    } else {
        (locals.var_qg_nqs, locals.var_qg_nqs_dn8, locals.var_qg_nqs_dn9,)
    }
};
        locals.var_qg_nqs = assign23580_e28646;
        locals.var_qg_nqs_dn8 = assign23580_e28646_d_n8;
        locals.var_qg_nqs_dn9 = assign23580_e28646_d_n9;
        locals.var_qg_nqs_rv = 0.0;

        let (assign23590_e28652, assign23590_e28652_d_n0, assign23590_e28652_d_n2, assign23590_e28652_d_n4, assign23590_e28652_d_n5, assign23590_e28652_d_n6, assign23590_e28652_d_n8, assign23590_e28652_d_n10, assign23590_e28652_d_n11, assign23590_e28652_d_n12,) = {
    if (locals.var_flg_nqs != 0.0) {
        let assign23590_e28650: f64 = (locals.var_qi_nqs * locals.var_qdrat);
        (assign23590_e28650, 0.0, 0.0, 0.0, 0.0, 0.0, (locals.var_qi_nqs_dn8 * locals.var_qdrat), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn4, locals.var_qd_nqs_dn5, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn8, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12,)
    }
};
        locals.var_qd_nqs = assign23590_e28652;
        locals.var_qd_nqs_dn0 = assign23590_e28652_d_n0;
        locals.var_qd_nqs_dn2 = assign23590_e28652_d_n2;
        locals.var_qd_nqs_dn4 = assign23590_e28652_d_n4;
        locals.var_qd_nqs_dn5 = assign23590_e28652_d_n5;
        locals.var_qd_nqs_dn6 = assign23590_e28652_d_n6;
        locals.var_qd_nqs_dn8 = assign23590_e28652_d_n8;
        locals.var_qd_nqs_dn10 = assign23590_e28652_d_n10;
        locals.var_qd_nqs_dn11 = assign23590_e28652_d_n11;
        locals.var_qd_nqs_dn12 = assign23590_e28652_d_n12;
        locals.var_qd_nqs_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_93(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23600_e28660, assign23600_e28660_d_n0, assign23600_e28660_d_n2, assign23600_e28660_d_n4, assign23600_e28660_d_n5, assign23600_e28660_d_n6, assign23600_e28660_d_n8, assign23600_e28660_d_n10, assign23600_e28660_d_n11, assign23600_e28660_d_n12,) = {
    if (locals.var_flg_nqs != 0.0) {
        let assign23600_e28657: f64 = (1.0 - locals.var_qdrat);
        let assign23600_e28658: f64 = (locals.var_qi_nqs * assign23600_e28657);
        (assign23600_e28658, 0.0, 0.0, 0.0, 0.0, 0.0, (locals.var_qi_nqs_dn8 * assign23600_e28657), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn4, locals.var_qs_nqs_dn5, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn8, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12,)
    }
};
        locals.var_qs_nqs = assign23600_e28660;
        locals.var_qs_nqs_dn0 = assign23600_e28660_d_n0;
        locals.var_qs_nqs_dn2 = assign23600_e28660_d_n2;
        locals.var_qs_nqs_dn4 = assign23600_e28660_d_n4;
        locals.var_qs_nqs_dn5 = assign23600_e28660_d_n5;
        locals.var_qs_nqs_dn6 = assign23600_e28660_d_n6;
        locals.var_qs_nqs_dn8 = assign23600_e28660_d_n8;
        locals.var_qs_nqs_dn10 = assign23600_e28660_d_n10;
        locals.var_qs_nqs_dn11 = assign23600_e28660_d_n11;
        locals.var_qs_nqs_dn12 = assign23600_e28660_d_n12;
        locals.var_qs_nqs_rv = 0.0;

        let (assign23610_e28665, assign23610_e28665_d_n0, assign23610_e28665_d_n2, assign23610_e28665_d_n4, assign23610_e28665_d_n5, assign23610_e28665_d_n6, assign23610_e28665_d_n8, assign23610_e28665_d_n10, assign23610_e28665_d_n11, assign23610_e28665_d_n12,) = {
    if (locals.var_flg_nqs == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_iqi_nqs, locals.var_iqi_nqs_dn0, locals.var_iqi_nqs_dn2, locals.var_iqi_nqs_dn4, locals.var_iqi_nqs_dn5, locals.var_iqi_nqs_dn6, locals.var_iqi_nqs_dn8, locals.var_iqi_nqs_dn10, locals.var_iqi_nqs_dn11, locals.var_iqi_nqs_dn12,)
    }
};
        locals.var_iqi_nqs = assign23610_e28665;
        locals.var_iqi_nqs_dn0 = assign23610_e28665_d_n0;
        locals.var_iqi_nqs_dn2 = assign23610_e28665_d_n2;
        locals.var_iqi_nqs_dn4 = assign23610_e28665_d_n4;
        locals.var_iqi_nqs_dn5 = assign23610_e28665_d_n5;
        locals.var_iqi_nqs_dn6 = assign23610_e28665_d_n6;
        locals.var_iqi_nqs_dn8 = assign23610_e28665_d_n8;
        locals.var_iqi_nqs_dn10 = assign23610_e28665_d_n10;
        locals.var_iqi_nqs_dn11 = assign23610_e28665_d_n11;
        locals.var_iqi_nqs_dn12 = assign23610_e28665_d_n12;
        locals.var_iqi_nqs_rv = 0.0;

        let (assign23620_e28670, assign23620_e28670_d_n0, assign23620_e28670_d_n2, assign23620_e28670_d_n4, assign23620_e28670_d_n5, assign23620_e28670_d_n6, assign23620_e28670_d_n8, assign23620_e28670_d_n9, assign23620_e28670_d_n10, assign23620_e28670_d_n11, assign23620_e28670_d_n12,) = {
    if (locals.var_flg_nqs == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_iqb_nqs, locals.var_iqb_nqs_dn0, locals.var_iqb_nqs_dn2, locals.var_iqb_nqs_dn4, locals.var_iqb_nqs_dn5, locals.var_iqb_nqs_dn6, locals.var_iqb_nqs_dn8, locals.var_iqb_nqs_dn9, locals.var_iqb_nqs_dn10, locals.var_iqb_nqs_dn11, locals.var_iqb_nqs_dn12,)
    }
};
        locals.var_iqb_nqs = assign23620_e28670;
        locals.var_iqb_nqs_dn0 = assign23620_e28670_d_n0;
        locals.var_iqb_nqs_dn2 = assign23620_e28670_d_n2;
        locals.var_iqb_nqs_dn4 = assign23620_e28670_d_n4;
        locals.var_iqb_nqs_dn5 = assign23620_e28670_d_n5;
        locals.var_iqb_nqs_dn6 = assign23620_e28670_d_n6;
        locals.var_iqb_nqs_dn8 = assign23620_e28670_d_n8;
        locals.var_iqb_nqs_dn9 = assign23620_e28670_d_n9;
        locals.var_iqb_nqs_dn10 = assign23620_e28670_d_n10;
        locals.var_iqb_nqs_dn11 = assign23620_e28670_d_n11;
        locals.var_iqb_nqs_dn12 = assign23620_e28670_d_n12;
        locals.var_iqb_nqs_rv = 0.0;

        let (assign23630_e28675, assign23630_e28675_d_n0, assign23630_e28675_d_n2, assign23630_e28675_d_n4, assign23630_e28675_d_n5, assign23630_e28675_d_n6, assign23630_e28675_d_n8, assign23630_e28675_d_n10, assign23630_e28675_d_n11, assign23630_e28675_d_n12,) = {
    if (locals.var_flg_nqs == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn4, locals.var_qd_nqs_dn5, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn8, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12,)
    }
};
        locals.var_qd_nqs = assign23630_e28675;
        locals.var_qd_nqs_dn0 = assign23630_e28675_d_n0;
        locals.var_qd_nqs_dn2 = assign23630_e28675_d_n2;
        locals.var_qd_nqs_dn4 = assign23630_e28675_d_n4;
        locals.var_qd_nqs_dn5 = assign23630_e28675_d_n5;
        locals.var_qd_nqs_dn6 = assign23630_e28675_d_n6;
        locals.var_qd_nqs_dn8 = assign23630_e28675_d_n8;
        locals.var_qd_nqs_dn10 = assign23630_e28675_d_n10;
        locals.var_qd_nqs_dn11 = assign23630_e28675_d_n11;
        locals.var_qd_nqs_dn12 = assign23630_e28675_d_n12;
        locals.var_qd_nqs_rv = 0.0;

        let (assign23640_e28680, assign23640_e28680_d_n0, assign23640_e28680_d_n2, assign23640_e28680_d_n4, assign23640_e28680_d_n5, assign23640_e28680_d_n6, assign23640_e28680_d_n8, assign23640_e28680_d_n10, assign23640_e28680_d_n11, assign23640_e28680_d_n12,) = {
    if (locals.var_flg_nqs == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn4, locals.var_qs_nqs_dn5, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn8, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12,)
    }
};
        locals.var_qs_nqs = assign23640_e28680;
        locals.var_qs_nqs_dn0 = assign23640_e28680_d_n0;
        locals.var_qs_nqs_dn2 = assign23640_e28680_d_n2;
        locals.var_qs_nqs_dn4 = assign23640_e28680_d_n4;
        locals.var_qs_nqs_dn5 = assign23640_e28680_d_n5;
        locals.var_qs_nqs_dn6 = assign23640_e28680_d_n6;
        locals.var_qs_nqs_dn8 = assign23640_e28680_d_n8;
        locals.var_qs_nqs_dn10 = assign23640_e28680_d_n10;
        locals.var_qs_nqs_dn11 = assign23640_e28680_d_n11;
        locals.var_qs_nqs_dn12 = assign23640_e28680_d_n12;
        locals.var_qs_nqs_rv = 0.0;

        let (assign23650_e28685, assign23650_e28685_d_n8, assign23650_e28685_d_n9,) = {
    if (locals.var_flg_nqs == 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_qg_nqs, locals.var_qg_nqs_dn8, locals.var_qg_nqs_dn9,)
    }
};
        locals.var_qg_nqs = assign23650_e28685;
        locals.var_qg_nqs_dn8 = assign23650_e28685_d_n8;
        locals.var_qg_nqs_dn9 = assign23650_e28685_d_n9;
        locals.var_qg_nqs_rv = 0.0;

        let (assign23660_e28690, assign23660_e28690_d_n9,) = {
    if (locals.var_flg_nqs == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_qb_nqs, locals.var_qb_nqs_dn9,)
    }
};
        locals.var_qb_nqs = assign23660_e28690;
        locals.var_qb_nqs_dn9 = assign23660_e28690_d_n9;
        locals.var_qb_nqs_rv = 0.0;

        let assign23690_e28695: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard436 = assign23690_e28695;
        locals.var_guard436_rv = 0.0;

        let (assign23700_e28699, assign23700_e28699_d_n0, assign23700_e28699_d_n2, assign23700_e28699_d_n4, assign23700_e28699_d_n5, assign23700_e28699_d_n6, assign23700_e28699_d_n8, assign23700_e28699_d_n10, assign23700_e28699_d_n11, assign23700_e28699_d_n12,) = {
    if (locals.var_guard436 != 0.0) {
        (locals.var_idse, locals.var_idse_dn0, locals.var_idse_dn2, locals.var_idse_dn4, locals.var_idse_dn5, locals.var_idse_dn6, locals.var_idse_dn8, locals.var_idse_dn10, locals.var_idse_dn11, locals.var_idse_dn12,)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn8, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12,)
    }
};
        locals.var_ids = assign23700_e28699;
        locals.var_ids_dn0 = assign23700_e28699_d_n0;
        locals.var_ids_dn2 = assign23700_e28699_d_n2;
        locals.var_ids_dn4 = assign23700_e28699_d_n4;
        locals.var_ids_dn5 = assign23700_e28699_d_n5;
        locals.var_ids_dn6 = assign23700_e28699_d_n6;
        locals.var_ids_dn8 = assign23700_e28699_d_n8;
        locals.var_ids_dn10 = assign23700_e28699_d_n10;
        locals.var_ids_dn11 = assign23700_e28699_d_n11;
        locals.var_ids_dn12 = assign23700_e28699_d_n12;
        locals.var_ids_rv = 0.0;

        let (assign23710_e28703, assign23710_e28703_d_n0, assign23710_e28703_d_n2, assign23710_e28703_d_n4, assign23710_e28703_d_n5, assign23710_e28703_d_n6, assign23710_e28703_d_n8, assign23710_e28703_d_n10, assign23710_e28703_d_n11, assign23710_e28703_d_n12,) = {
    if (locals.var_guard436 != 0.0) {
        (locals.var_isube, locals.var_isube_dn0, locals.var_isube_dn2, locals.var_isube_dn4, locals.var_isube_dn5, locals.var_isube_dn6, locals.var_isube_dn8, locals.var_isube_dn10, locals.var_isube_dn11, locals.var_isube_dn12,)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn4, locals.var_isub_dn5, locals.var_isub_dn6, locals.var_isub_dn8, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12,)
    }
};
        locals.var_isub = assign23710_e28703;
        locals.var_isub_dn0 = assign23710_e28703_d_n0;
        locals.var_isub_dn2 = assign23710_e28703_d_n2;
        locals.var_isub_dn4 = assign23710_e28703_d_n4;
        locals.var_isub_dn5 = assign23710_e28703_d_n5;
        locals.var_isub_dn6 = assign23710_e28703_d_n6;
        locals.var_isub_dn8 = assign23710_e28703_d_n8;
        locals.var_isub_dn10 = assign23710_e28703_d_n10;
        locals.var_isub_dn11 = assign23710_e28703_d_n11;
        locals.var_isub_dn12 = assign23710_e28703_d_n12;
        locals.var_isub_rv = 0.0;

        let (assign23730_e28711, assign23730_e28711_d_n0, assign23730_e28711_d_n2, assign23730_e28711_d_n4, assign23730_e28711_d_n5, assign23730_e28711_d_n6, assign23730_e28711_d_n8, assign23730_e28711_d_n10, assign23730_e28711_d_n11, assign23730_e28711_d_n12,) = {
    if (locals.var_guard436 != 0.0) {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn8, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12,)
    } else {
        (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn4, locals.var_qg_dn5, locals.var_qg_dn6, locals.var_qg_dn8, locals.var_qg_dn10, locals.var_qg_dn11, locals.var_qg_dn12,)
    }
};
        locals.var_qg = assign23730_e28711;
        locals.var_qg_dn0 = assign23730_e28711_d_n0;
        locals.var_qg_dn2 = assign23730_e28711_d_n2;
        locals.var_qg_dn4 = assign23730_e28711_d_n4;
        locals.var_qg_dn5 = assign23730_e28711_d_n5;
        locals.var_qg_dn6 = assign23730_e28711_d_n6;
        locals.var_qg_dn8 = assign23730_e28711_d_n8;
        locals.var_qg_dn10 = assign23730_e28711_d_n10;
        locals.var_qg_dn11 = assign23730_e28711_d_n11;
        locals.var_qg_dn12 = assign23730_e28711_d_n12;
        locals.var_qg_rv = 0.0;

        let (assign23740_e28715, assign23740_e28715_d_n0, assign23740_e28715_d_n2, assign23740_e28715_d_n4, assign23740_e28715_d_n5, assign23740_e28715_d_n6, assign23740_e28715_d_n8, assign23740_e28715_d_n10, assign23740_e28715_d_n11, assign23740_e28715_d_n12,) = {
    if (locals.var_guard436 != 0.0) {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn8, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12,)
    } else {
        (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn4, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn8, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12,)
    }
};
        locals.var_qd = assign23740_e28715;
        locals.var_qd_dn0 = assign23740_e28715_d_n0;
        locals.var_qd_dn2 = assign23740_e28715_d_n2;
        locals.var_qd_dn4 = assign23740_e28715_d_n4;
        locals.var_qd_dn5 = assign23740_e28715_d_n5;
        locals.var_qd_dn6 = assign23740_e28715_d_n6;
        locals.var_qd_dn8 = assign23740_e28715_d_n8;
        locals.var_qd_dn10 = assign23740_e28715_d_n10;
        locals.var_qd_dn11 = assign23740_e28715_d_n11;
        locals.var_qd_dn12 = assign23740_e28715_d_n12;
        locals.var_qd_rv = 0.0;

        let (assign23750_e28724, assign23750_e28724_d_n0, assign23750_e28724_d_n2, assign23750_e28724_d_n4, assign23750_e28724_d_n5, assign23750_e28724_d_n6, assign23750_e28724_d_n8, assign23750_e28724_d_n10, assign23750_e28724_d_n11, assign23750_e28724_d_n12,) = {
    if (locals.var_guard436 != 0.0) {
        let assign23750_e28719: f64 = (locals.var_qge + locals.var_qde);
        let assign23750_e28721: f64 = (assign23750_e28719 + locals.var_qse);
        let assign23750_e28722: f64 = (-assign23750_e28721);
        (assign23750_e28722, (-((locals.var_qge_dn0 + locals.var_qde_dn0) + locals.var_qse_dn0)), (-((locals.var_qge_dn2 + locals.var_qde_dn2) + locals.var_qse_dn2)), (-((locals.var_qge_dn4 + locals.var_qde_dn4) + locals.var_qse_dn4)), (-((locals.var_qge_dn5 + locals.var_qde_dn5) + locals.var_qse_dn5)), (-((locals.var_qge_dn6 + locals.var_qde_dn6) + locals.var_qse_dn6)), (-((locals.var_qge_dn8 + locals.var_qde_dn8) + locals.var_qse_dn8)), (-((locals.var_qge_dn10 + locals.var_qde_dn10) + locals.var_qse_dn10)), (-((locals.var_qge_dn11 + locals.var_qde_dn11) + locals.var_qse_dn11)), (-((locals.var_qge_dn12 + locals.var_qde_dn12) + locals.var_qse_dn12)),)
    } else {
        (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn4, locals.var_qbe_dn5, locals.var_qbe_dn6, locals.var_qbe_dn8, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12,)
    }
};
        locals.var_qbe = assign23750_e28724;
        locals.var_qbe_dn0 = assign23750_e28724_d_n0;
        locals.var_qbe_dn2 = assign23750_e28724_d_n2;
        locals.var_qbe_dn4 = assign23750_e28724_d_n4;
        locals.var_qbe_dn5 = assign23750_e28724_d_n5;
        locals.var_qbe_dn6 = assign23750_e28724_d_n6;
        locals.var_qbe_dn8 = assign23750_e28724_d_n8;
        locals.var_qbe_dn10 = assign23750_e28724_d_n10;
        locals.var_qbe_dn11 = assign23750_e28724_d_n11;
        locals.var_qbe_dn12 = assign23750_e28724_d_n12;
        locals.var_qbe_rv = 0.0;

        let (assign23760_e28728, assign23760_e28728_d_n0, assign23760_e28728_d_n2, assign23760_e28728_d_n4, assign23760_e28728_d_n5, assign23760_e28728_d_n6, assign23760_e28728_d_n8, assign23760_e28728_d_n10, assign23760_e28728_d_n11, assign23760_e28728_d_n12,) = {
    if (locals.var_guard436 != 0.0) {
        (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn4, locals.var_qbe_dn5, locals.var_qbe_dn6, locals.var_qbe_dn8, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12,)
    } else {
        (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn4, locals.var_qb_dn5, locals.var_qb_dn6, locals.var_qb_dn8, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn12,)
    }
};
        locals.var_qb = assign23760_e28728;
        locals.var_qb_dn0 = assign23760_e28728_d_n0;
        locals.var_qb_dn2 = assign23760_e28728_d_n2;
        locals.var_qb_dn4 = assign23760_e28728_d_n4;
        locals.var_qb_dn5 = assign23760_e28728_d_n5;
        locals.var_qb_dn6 = assign23760_e28728_d_n6;
        locals.var_qb_dn8 = assign23760_e28728_d_n8;
        locals.var_qb_dn10 = assign23760_e28728_d_n10;
        locals.var_qb_dn11 = assign23760_e28728_d_n11;
        locals.var_qb_dn12 = assign23760_e28728_d_n12;
        locals.var_qb_rv = 0.0;

        let (assign23770_e28734, assign23770_e28734_d_n0, assign23770_e28734_d_n2, assign23770_e28734_d_n4, assign23770_e28734_d_n5, assign23770_e28734_d_n6, assign23770_e28734_d_n8, assign23770_e28734_d_n10, assign23770_e28734_d_n11, assign23770_e28734_d_n12,) = {
    if (locals.var_guard436 == 0.0) {
        let assign23770_e28732: f64 = (-locals.var_idse);
        (assign23770_e28732, (-locals.var_idse_dn0), (-locals.var_idse_dn2), (-locals.var_idse_dn4), (-locals.var_idse_dn5), (-locals.var_idse_dn6), (-locals.var_idse_dn8), (-locals.var_idse_dn10), (-locals.var_idse_dn11), (-locals.var_idse_dn12),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn8, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12,)
    }
};
        locals.var_ids = assign23770_e28734;
        locals.var_ids_dn0 = assign23770_e28734_d_n0;
        locals.var_ids_dn2 = assign23770_e28734_d_n2;
        locals.var_ids_dn4 = assign23770_e28734_d_n4;
        locals.var_ids_dn5 = assign23770_e28734_d_n5;
        locals.var_ids_dn6 = assign23770_e28734_d_n6;
        locals.var_ids_dn8 = assign23770_e28734_d_n8;
        locals.var_ids_dn10 = assign23770_e28734_d_n10;
        locals.var_ids_dn11 = assign23770_e28734_d_n11;
        locals.var_ids_dn12 = assign23770_e28734_d_n12;
        locals.var_ids_rv = 0.0;

        let (assign23790_e28744, assign23790_e28744_d_n0, assign23790_e28744_d_n2, assign23790_e28744_d_n4, assign23790_e28744_d_n5, assign23790_e28744_d_n6, assign23790_e28744_d_n8, assign23790_e28744_d_n10, assign23790_e28744_d_n11, assign23790_e28744_d_n12,) = {
    if (locals.var_guard436 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn4, locals.var_isub_dn5, locals.var_isub_dn6, locals.var_isub_dn8, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12,)
    }
};
        locals.var_isub = assign23790_e28744;
        locals.var_isub_dn0 = assign23790_e28744_d_n0;
        locals.var_isub_dn2 = assign23790_e28744_d_n2;
        locals.var_isub_dn4 = assign23790_e28744_d_n4;
        locals.var_isub_dn5 = assign23790_e28744_d_n5;
        locals.var_isub_dn6 = assign23790_e28744_d_n6;
        locals.var_isub_dn8 = assign23790_e28744_d_n8;
        locals.var_isub_dn10 = assign23790_e28744_d_n10;
        locals.var_isub_dn11 = assign23790_e28744_d_n11;
        locals.var_isub_dn12 = assign23790_e28744_d_n12;
        locals.var_isub_rv = 0.0;

        let (assign23800_e28749, assign23800_e28749_d_n0, assign23800_e28749_d_n2, assign23800_e28749_d_n4, assign23800_e28749_d_n5, assign23800_e28749_d_n6, assign23800_e28749_d_n8, assign23800_e28749_d_n10, assign23800_e28749_d_n11, assign23800_e28749_d_n12,) = {
    if (locals.var_guard436 == 0.0) {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn8, locals.var_qge_dn10, locals.var_qge_dn11, locals.var_qge_dn12,)
    } else {
        (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn4, locals.var_qg_dn5, locals.var_qg_dn6, locals.var_qg_dn8, locals.var_qg_dn10, locals.var_qg_dn11, locals.var_qg_dn12,)
    }
};
        locals.var_qg = assign23800_e28749;
        locals.var_qg_dn0 = assign23800_e28749_d_n0;
        locals.var_qg_dn2 = assign23800_e28749_d_n2;
        locals.var_qg_dn4 = assign23800_e28749_d_n4;
        locals.var_qg_dn5 = assign23800_e28749_d_n5;
        locals.var_qg_dn6 = assign23800_e28749_d_n6;
        locals.var_qg_dn8 = assign23800_e28749_d_n8;
        locals.var_qg_dn10 = assign23800_e28749_d_n10;
        locals.var_qg_dn11 = assign23800_e28749_d_n11;
        locals.var_qg_dn12 = assign23800_e28749_d_n12;
        locals.var_qg_rv = 0.0;

        let (assign23810_e28754, assign23810_e28754_d_n0, assign23810_e28754_d_n2, assign23810_e28754_d_n4, assign23810_e28754_d_n5, assign23810_e28754_d_n6, assign23810_e28754_d_n8, assign23810_e28754_d_n10, assign23810_e28754_d_n11, assign23810_e28754_d_n12,) = {
    if (locals.var_guard436 == 0.0) {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn8, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12,)
    } else {
        (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn4, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn8, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12,)
    }
};
        locals.var_qd = assign23810_e28754;
        locals.var_qd_dn0 = assign23810_e28754_d_n0;
        locals.var_qd_dn2 = assign23810_e28754_d_n2;
        locals.var_qd_dn4 = assign23810_e28754_d_n4;
        locals.var_qd_dn5 = assign23810_e28754_d_n5;
        locals.var_qd_dn6 = assign23810_e28754_d_n6;
        locals.var_qd_dn8 = assign23810_e28754_d_n8;
        locals.var_qd_dn10 = assign23810_e28754_d_n10;
        locals.var_qd_dn11 = assign23810_e28754_d_n11;
        locals.var_qd_dn12 = assign23810_e28754_d_n12;
        locals.var_qd_rv = 0.0;

        let (assign23820_e28764, assign23820_e28764_d_n0, assign23820_e28764_d_n2, assign23820_e28764_d_n4, assign23820_e28764_d_n5, assign23820_e28764_d_n6, assign23820_e28764_d_n8, assign23820_e28764_d_n10, assign23820_e28764_d_n11, assign23820_e28764_d_n12,) = {
    if (locals.var_guard436 == 0.0) {
        let assign23820_e28759: f64 = (locals.var_qge + locals.var_qde);
        let assign23820_e28761: f64 = (assign23820_e28759 + locals.var_qse);
        let assign23820_e28762: f64 = (-assign23820_e28761);
        (assign23820_e28762, (-((locals.var_qge_dn0 + locals.var_qde_dn0) + locals.var_qse_dn0)), (-((locals.var_qge_dn2 + locals.var_qde_dn2) + locals.var_qse_dn2)), (-((locals.var_qge_dn4 + locals.var_qde_dn4) + locals.var_qse_dn4)), (-((locals.var_qge_dn5 + locals.var_qde_dn5) + locals.var_qse_dn5)), (-((locals.var_qge_dn6 + locals.var_qde_dn6) + locals.var_qse_dn6)), (-((locals.var_qge_dn8 + locals.var_qde_dn8) + locals.var_qse_dn8)), (-((locals.var_qge_dn10 + locals.var_qde_dn10) + locals.var_qse_dn10)), (-((locals.var_qge_dn11 + locals.var_qde_dn11) + locals.var_qse_dn11)), (-((locals.var_qge_dn12 + locals.var_qde_dn12) + locals.var_qse_dn12)),)
    } else {
        (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn4, locals.var_qbe_dn5, locals.var_qbe_dn6, locals.var_qbe_dn8, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12,)
    }
};
        locals.var_qbe = assign23820_e28764;
        locals.var_qbe_dn0 = assign23820_e28764_d_n0;
        locals.var_qbe_dn2 = assign23820_e28764_d_n2;
        locals.var_qbe_dn4 = assign23820_e28764_d_n4;
        locals.var_qbe_dn5 = assign23820_e28764_d_n5;
        locals.var_qbe_dn6 = assign23820_e28764_d_n6;
        locals.var_qbe_dn8 = assign23820_e28764_d_n8;
        locals.var_qbe_dn10 = assign23820_e28764_d_n10;
        locals.var_qbe_dn11 = assign23820_e28764_d_n11;
        locals.var_qbe_dn12 = assign23820_e28764_d_n12;
        locals.var_qbe_rv = 0.0;

        let (assign23830_e28769, assign23830_e28769_d_n0, assign23830_e28769_d_n2, assign23830_e28769_d_n4, assign23830_e28769_d_n5, assign23830_e28769_d_n6, assign23830_e28769_d_n8, assign23830_e28769_d_n10, assign23830_e28769_d_n11, assign23830_e28769_d_n12,) = {
    if (locals.var_guard436 == 0.0) {
        (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn4, locals.var_qbe_dn5, locals.var_qbe_dn6, locals.var_qbe_dn8, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12,)
    } else {
        (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn4, locals.var_qb_dn5, locals.var_qb_dn6, locals.var_qb_dn8, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn12,)
    }
};
        locals.var_qb = assign23830_e28769;
        locals.var_qb_dn0 = assign23830_e28769_d_n0;
        locals.var_qb_dn2 = assign23830_e28769_d_n2;
        locals.var_qb_dn4 = assign23830_e28769_d_n4;
        locals.var_qb_dn5 = assign23830_e28769_d_n5;
        locals.var_qb_dn6 = assign23830_e28769_d_n6;
        locals.var_qb_dn8 = assign23830_e28769_d_n8;
        locals.var_qb_dn10 = assign23830_e28769_d_n10;
        locals.var_qb_dn11 = assign23830_e28769_d_n11;
        locals.var_qb_dn12 = assign23830_e28769_d_n12;
        locals.var_qb_rv = 0.0;

        let (assign23840_e28774, assign23840_e28774_d_n0, assign23840_e28774_d_n2, assign23840_e28774_d_n4, assign23840_e28774_d_n5, assign23840_e28774_d_n6, assign23840_e28774_d_n8, assign23840_e28774_d_n10, assign23840_e28774_d_n11, assign23840_e28774_d_n12,) = {
    if (locals.var_guard436 == 0.0) {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn8, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12,)
    } else {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn8, locals.var_qse_dn10, locals.var_qse_dn11, locals.var_qse_dn12,)
    }
};
        locals.var_qse = assign23840_e28774;
        locals.var_qse_dn0 = assign23840_e28774_d_n0;
        locals.var_qse_dn2 = assign23840_e28774_d_n2;
        locals.var_qse_dn4 = assign23840_e28774_d_n4;
        locals.var_qse_dn5 = assign23840_e28774_d_n5;
        locals.var_qse_dn6 = assign23840_e28774_d_n6;
        locals.var_qse_dn8 = assign23840_e28774_d_n8;
        locals.var_qse_dn10 = assign23840_e28774_d_n10;
        locals.var_qse_dn11 = assign23840_e28774_d_n11;
        locals.var_qse_dn12 = assign23840_e28774_d_n12;
        locals.var_qse_rv = 0.0;

        let (assign23850_e28779, assign23850_e28779_d_n0, assign23850_e28779_d_n2, assign23850_e28779_d_n4, assign23850_e28779_d_n5, assign23850_e28779_d_n6, assign23850_e28779_d_n8, assign23850_e28779_d_n10, assign23850_e28779_d_n11, assign23850_e28779_d_n12,) = {
    if (locals.var_guard436 == 0.0) {
        (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn4, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn8, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12,)
    } else {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn8, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn12,)
    }
};
        locals.var_qde = assign23850_e28779;
        locals.var_qde_dn0 = assign23850_e28779_d_n0;
        locals.var_qde_dn2 = assign23850_e28779_d_n2;
        locals.var_qde_dn4 = assign23850_e28779_d_n4;
        locals.var_qde_dn5 = assign23850_e28779_d_n5;
        locals.var_qde_dn6 = assign23850_e28779_d_n6;
        locals.var_qde_dn8 = assign23850_e28779_d_n8;
        locals.var_qde_dn10 = assign23850_e28779_d_n10;
        locals.var_qde_dn11 = assign23850_e28779_d_n11;
        locals.var_qde_dn12 = assign23850_e28779_d_n12;
        locals.var_qde_rv = 0.0;

        let (assign23860_e28786, assign23860_e28786_d_n0, assign23860_e28786_d_n2, assign23860_e28786_d_n4, assign23860_e28786_d_n5, assign23860_e28786_d_n6, assign23860_e28786_d_n8, assign23860_e28786_d_n10, assign23860_e28786_d_n11, assign23860_e28786_d_n12,) = {
    if ((locals.var_guard436 == 0.0) && (locals.var_flg_nqs != 0.0)) {
        (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn4, locals.var_qd_nqs_dn5, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn8, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign23860_e28786;
        locals.var_t1_dn0 = assign23860_e28786_d_n0;
        locals.var_t1_dn2 = assign23860_e28786_d_n2;
        locals.var_t1_dn4 = assign23860_e28786_d_n4;
        locals.var_t1_dn5 = assign23860_e28786_d_n5;
        locals.var_t1_dn6 = assign23860_e28786_d_n6;
        locals.var_t1_dn8 = assign23860_e28786_d_n8;
        locals.var_t1_dn10 = assign23860_e28786_d_n10;
        locals.var_t1_dn11 = assign23860_e28786_d_n11;
        locals.var_t1_dn12 = assign23860_e28786_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign23870_e28793, assign23870_e28793_d_n0, assign23870_e28793_d_n2, assign23870_e28793_d_n4, assign23870_e28793_d_n5, assign23870_e28793_d_n6, assign23870_e28793_d_n8, assign23870_e28793_d_n10, assign23870_e28793_d_n11, assign23870_e28793_d_n12,) = {
    if ((locals.var_guard436 == 0.0) && (locals.var_flg_nqs != 0.0)) {
        (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn4, locals.var_qs_nqs_dn5, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn8, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12,)
    } else {
        (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn4, locals.var_qd_nqs_dn5, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn8, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12,)
    }
};
        locals.var_qd_nqs = assign23870_e28793;
        locals.var_qd_nqs_dn0 = assign23870_e28793_d_n0;
        locals.var_qd_nqs_dn2 = assign23870_e28793_d_n2;
        locals.var_qd_nqs_dn4 = assign23870_e28793_d_n4;
        locals.var_qd_nqs_dn5 = assign23870_e28793_d_n5;
        locals.var_qd_nqs_dn6 = assign23870_e28793_d_n6;
        locals.var_qd_nqs_dn8 = assign23870_e28793_d_n8;
        locals.var_qd_nqs_dn10 = assign23870_e28793_d_n10;
        locals.var_qd_nqs_dn11 = assign23870_e28793_d_n11;
        locals.var_qd_nqs_dn12 = assign23870_e28793_d_n12;
        locals.var_qd_nqs_rv = 0.0;

        let (assign23880_e28800, assign23880_e28800_d_n0, assign23880_e28800_d_n2, assign23880_e28800_d_n4, assign23880_e28800_d_n5, assign23880_e28800_d_n6, assign23880_e28800_d_n8, assign23880_e28800_d_n10, assign23880_e28800_d_n11, assign23880_e28800_d_n12,) = {
    if ((locals.var_guard436 == 0.0) && (locals.var_flg_nqs != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    } else {
        (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn4, locals.var_qs_nqs_dn5, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn8, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12,)
    }
};
        locals.var_qs_nqs = assign23880_e28800;
        locals.var_qs_nqs_dn0 = assign23880_e28800_d_n0;
        locals.var_qs_nqs_dn2 = assign23880_e28800_d_n2;
        locals.var_qs_nqs_dn4 = assign23880_e28800_d_n4;
        locals.var_qs_nqs_dn5 = assign23880_e28800_d_n5;
        locals.var_qs_nqs_dn6 = assign23880_e28800_d_n6;
        locals.var_qs_nqs_dn8 = assign23880_e28800_d_n8;
        locals.var_qs_nqs_dn10 = assign23880_e28800_d_n10;
        locals.var_qs_nqs_dn11 = assign23880_e28800_d_n11;
        locals.var_qs_nqs_dn12 = assign23880_e28800_d_n12;
        locals.var_qs_nqs_rv = 0.0;

        let assign23890_e28805: f64 = if ((p.p28 != 0.0) && (p.p237 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard437 = assign23890_e28805;
        locals.var_guard437_rv = 0.0;

        let (assign23900_e28811, assign23900_e28811_d_n0, assign23900_e28811_d_n2, assign23900_e28811_d_n4, assign23900_e28811_d_n5, assign23900_e28811_d_n6, assign23900_e28811_d_n8, assign23900_e28811_d_n10, assign23900_e28811_d_n11, assign23900_e28811_d_n12,) = {
    if (locals.var_guard437 != 0.0) {
        let assign23900_e28809: f64 = (locals.var_idse * locals.var_vds);
        (assign23900_e28809, ((locals.var_idse_dn0 * locals.var_vds) + (locals.var_idse * locals.var_vds_dn0)), ((locals.var_idse_dn2 * locals.var_vds) + (locals.var_idse * locals.var_vds_dn2)), ((locals.var_idse_dn4 * locals.var_vds) + (locals.var_idse * locals.var_vds_dn4)), ((locals.var_idse_dn5 * locals.var_vds) + (locals.var_idse * locals.var_vds_dn5)), ((locals.var_idse_dn6 * locals.var_vds) + (locals.var_idse * locals.var_vds_dn6)), ((locals.var_idse_dn8 * locals.var_vds) + (locals.var_idse * locals.var_vds_dn8)), ((locals.var_idse_dn10 * locals.var_vds) + (locals.var_idse * locals.var_vds_dn10)), ((locals.var_idse_dn11 * locals.var_vds) + (locals.var_idse * locals.var_vds_dn11)), ((locals.var_idse_dn12 * locals.var_vds) + (locals.var_idse * locals.var_vds_dn12)),)
    } else {
        (locals.var_rpower, locals.var_rpower_dn0, locals.var_rpower_dn2, locals.var_rpower_dn4, locals.var_rpower_dn5, locals.var_rpower_dn6, locals.var_rpower_dn8, locals.var_rpower_dn10, locals.var_rpower_dn11, locals.var_rpower_dn12,)
    }
};
        locals.var_rpower = assign23900_e28811;
        locals.var_rpower_dn0 = assign23900_e28811_d_n0;
        locals.var_rpower_dn2 = assign23900_e28811_d_n2;
        locals.var_rpower_dn4 = assign23900_e28811_d_n4;
        locals.var_rpower_dn5 = assign23900_e28811_d_n5;
        locals.var_rpower_dn6 = assign23900_e28811_d_n6;
        locals.var_rpower_dn8 = assign23900_e28811_d_n8;
        locals.var_rpower_dn10 = assign23900_e28811_d_n10;
        locals.var_rpower_dn11 = assign23900_e28811_d_n11;
        locals.var_rpower_dn12 = assign23900_e28811_d_n12;
        locals.var_rpower_rv = 0.0;

        let (assign23910_e28815, assign23910_e28815_d_n0, assign23910_e28815_d_n2, assign23910_e28815_d_n4, assign23910_e28815_d_n5, assign23910_e28815_d_n6, assign23910_e28815_d_n8, assign23910_e28815_d_n10, assign23910_e28815_d_n11, assign23910_e28815_d_n12,) = {
    if (locals.var_guard437 != 0.0) {
        (locals.var_cth, locals.var_cth_dn0, locals.var_cth_dn2, locals.var_cth_dn4, locals.var_cth_dn5, locals.var_cth_dn6, locals.var_cth_dn8, locals.var_cth_dn10, locals.var_cth_dn11, locals.var_cth_dn12,)
    } else {
        (locals.var_cthe, locals.var_cthe_dn0, locals.var_cthe_dn2, locals.var_cthe_dn4, locals.var_cthe_dn5, locals.var_cthe_dn6, locals.var_cthe_dn8, locals.var_cthe_dn10, locals.var_cthe_dn11, locals.var_cthe_dn12,)
    }
};
        locals.var_cthe = assign23910_e28815;
        locals.var_cthe_dn0 = assign23910_e28815_d_n0;
        locals.var_cthe_dn2 = assign23910_e28815_d_n2;
        locals.var_cthe_dn4 = assign23910_e28815_d_n4;
        locals.var_cthe_dn5 = assign23910_e28815_d_n5;
        locals.var_cthe_dn6 = assign23910_e28815_d_n6;
        locals.var_cthe_dn8 = assign23910_e28815_d_n8;
        locals.var_cthe_dn10 = assign23910_e28815_d_n10;
        locals.var_cthe_dn11 = assign23910_e28815_d_n11;
        locals.var_cthe_dn12 = assign23910_e28815_d_n12;
        locals.var_cthe_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_94(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23920_e28821, assign23920_e28821_d_n0, assign23920_e28821_d_n2, assign23920_e28821_d_n4, assign23920_e28821_d_n5, assign23920_e28821_d_n6, assign23920_e28821_d_n8, assign23920_e28821_d_n10, assign23920_e28821_d_n11, assign23920_e28821_d_n12,) = {
    if (locals.var_guard437 != 0.0) {
        let assign23920_e28819: f64 = (1.0 / locals.var_rth);
        (assign23920_e28819, (-(locals.var_rth_dn0 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn2 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn4 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn5 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn6 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn8 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn10 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn11 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn12 / (locals.var_rth * locals.var_rth))),)
    } else {
        (locals.var_gth, locals.var_gth_dn0, locals.var_gth_dn2, locals.var_gth_dn4, locals.var_gth_dn5, locals.var_gth_dn6, locals.var_gth_dn8, locals.var_gth_dn10, locals.var_gth_dn11, locals.var_gth_dn12,)
    }
};
        locals.var_gth = assign23920_e28821;
        locals.var_gth_dn0 = assign23920_e28821_d_n0;
        locals.var_gth_dn2 = assign23920_e28821_d_n2;
        locals.var_gth_dn4 = assign23920_e28821_d_n4;
        locals.var_gth_dn5 = assign23920_e28821_d_n5;
        locals.var_gth_dn6 = assign23920_e28821_d_n6;
        locals.var_gth_dn8 = assign23920_e28821_d_n8;
        locals.var_gth_dn10 = assign23920_e28821_d_n10;
        locals.var_gth_dn11 = assign23920_e28821_d_n11;
        locals.var_gth_dn12 = assign23920_e28821_d_n12;
        locals.var_gth_rv = 0.0;

        let (assign23930_e28826, assign23930_e28826_d_n0, assign23930_e28826_d_n2, assign23930_e28826_d_n4, assign23930_e28826_d_n5, assign23930_e28826_d_n6, assign23930_e28826_d_n8, assign23930_e28826_d_n10, assign23930_e28826_d_n11, assign23930_e28826_d_n12,) = {
    if (locals.var_guard437 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rpower, locals.var_rpower_dn0, locals.var_rpower_dn2, locals.var_rpower_dn4, locals.var_rpower_dn5, locals.var_rpower_dn6, locals.var_rpower_dn8, locals.var_rpower_dn10, locals.var_rpower_dn11, locals.var_rpower_dn12,)
    }
};
        locals.var_rpower = assign23930_e28826;
        locals.var_rpower_dn0 = assign23930_e28826_d_n0;
        locals.var_rpower_dn2 = assign23930_e28826_d_n2;
        locals.var_rpower_dn4 = assign23930_e28826_d_n4;
        locals.var_rpower_dn5 = assign23930_e28826_d_n5;
        locals.var_rpower_dn6 = assign23930_e28826_d_n6;
        locals.var_rpower_dn8 = assign23930_e28826_d_n8;
        locals.var_rpower_dn10 = assign23930_e28826_d_n10;
        locals.var_rpower_dn11 = assign23930_e28826_d_n11;
        locals.var_rpower_dn12 = assign23930_e28826_d_n12;
        locals.var_rpower_rv = 0.0;

        let (assign23940_e28831, assign23940_e28831_d_n0, assign23940_e28831_d_n2, assign23940_e28831_d_n4, assign23940_e28831_d_n5, assign23940_e28831_d_n6, assign23940_e28831_d_n8, assign23940_e28831_d_n10, assign23940_e28831_d_n11, assign23940_e28831_d_n12,) = {
    if (locals.var_guard437 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cthe, locals.var_cthe_dn0, locals.var_cthe_dn2, locals.var_cthe_dn4, locals.var_cthe_dn5, locals.var_cthe_dn6, locals.var_cthe_dn8, locals.var_cthe_dn10, locals.var_cthe_dn11, locals.var_cthe_dn12,)
    }
};
        locals.var_cthe = assign23940_e28831;
        locals.var_cthe_dn0 = assign23940_e28831_d_n0;
        locals.var_cthe_dn2 = assign23940_e28831_d_n2;
        locals.var_cthe_dn4 = assign23940_e28831_d_n4;
        locals.var_cthe_dn5 = assign23940_e28831_d_n5;
        locals.var_cthe_dn6 = assign23940_e28831_d_n6;
        locals.var_cthe_dn8 = assign23940_e28831_d_n8;
        locals.var_cthe_dn10 = assign23940_e28831_d_n10;
        locals.var_cthe_dn11 = assign23940_e28831_d_n11;
        locals.var_cthe_dn12 = assign23940_e28831_d_n12;
        locals.var_cthe_rv = 0.0;

        let (assign23950_e28836, assign23950_e28836_d_n0, assign23950_e28836_d_n2, assign23950_e28836_d_n4, assign23950_e28836_d_n5, assign23950_e28836_d_n6, assign23950_e28836_d_n8, assign23950_e28836_d_n10, assign23950_e28836_d_n11, assign23950_e28836_d_n12,) = {
    if (locals.var_guard437 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_gth, locals.var_gth_dn0, locals.var_gth_dn2, locals.var_gth_dn4, locals.var_gth_dn5, locals.var_gth_dn6, locals.var_gth_dn8, locals.var_gth_dn10, locals.var_gth_dn11, locals.var_gth_dn12,)
    }
};
        locals.var_gth = assign23950_e28836;
        locals.var_gth_dn0 = assign23950_e28836_d_n0;
        locals.var_gth_dn2 = assign23950_e28836_d_n2;
        locals.var_gth_dn4 = assign23950_e28836_d_n4;
        locals.var_gth_dn5 = assign23950_e28836_d_n5;
        locals.var_gth_dn6 = assign23950_e28836_d_n6;
        locals.var_gth_dn8 = assign23950_e28836_d_n8;
        locals.var_gth_dn10 = assign23950_e28836_d_n10;
        locals.var_gth_dn11 = assign23950_e28836_d_n11;
        locals.var_gth_dn12 = assign23950_e28836_d_n12;
        locals.var_gth_rv = 0.0;

        locals.var_idse = locals.var_ids;
        locals.var_idse_dn0 = locals.var_ids_dn0;
        locals.var_idse_dn2 = locals.var_ids_dn2;
        locals.var_idse_dn4 = locals.var_ids_dn4;
        locals.var_idse_dn5 = locals.var_ids_dn5;
        locals.var_idse_dn6 = locals.var_ids_dn6;
        locals.var_idse_dn8 = locals.var_ids_dn8;
        locals.var_idse_dn10 = locals.var_ids_dn10;
        locals.var_idse_dn11 = locals.var_ids_dn11;
        locals.var_idse_dn12 = locals.var_ids_dn12;
        locals.var_idse_rv = 0.0;

        let assign24160_e28890: f64 = locals.var_qge_dn11;
        locals.var_cgdbd = assign24160_e28890;
        locals.var_cgdbd_dn0 = 0.0;
        locals.var_cgdbd_dn2 = 0.0;
        locals.var_cgdbd_dn4 = 0.0;
        locals.var_cgdbd_dn5 = 0.0;
        locals.var_cgdbd_dn6 = 0.0;
        locals.var_cgdbd_dn8 = 0.0;
        locals.var_cgdbd_dn10 = 0.0;
        locals.var_cgdbd_dn11 = 0.0;
        locals.var_cgdbd_dn12 = 0.0;
        locals.var_cgdbd_rv = 0.0;

        let assign24170_e28893: f64 = (p.p33 * locals.var_cgdbd);
        locals.var_cgdbd = assign24170_e28893;
        locals.var_cgdbd_dn0 = (p.p33 * locals.var_cgdbd_dn0);
        locals.var_cgdbd_dn2 = (p.p33 * locals.var_cgdbd_dn2);
        locals.var_cgdbd_dn4 = (p.p33 * locals.var_cgdbd_dn4);
        locals.var_cgdbd_dn5 = (p.p33 * locals.var_cgdbd_dn5);
        locals.var_cgdbd_dn6 = (p.p33 * locals.var_cgdbd_dn6);
        locals.var_cgdbd_dn8 = (p.p33 * locals.var_cgdbd_dn8);
        locals.var_cgdbd_dn10 = (p.p33 * locals.var_cgdbd_dn10);
        locals.var_cgdbd_dn11 = (p.p33 * locals.var_cgdbd_dn11);
        locals.var_cgdbd_dn12 = (p.p33 * locals.var_cgdbd_dn12);
        locals.var_cgdbd_rv = 0.0;

        let assign24180_e28896: f64 = locals.var_qge_dn12;
        locals.var_cgsbd = assign24180_e28896;
        locals.var_cgsbd_dn0 = 0.0;
        locals.var_cgsbd_dn2 = 0.0;
        locals.var_cgsbd_dn4 = 0.0;
        locals.var_cgsbd_dn5 = 0.0;
        locals.var_cgsbd_dn6 = 0.0;
        locals.var_cgsbd_dn8 = 0.0;
        locals.var_cgsbd_dn10 = 0.0;
        locals.var_cgsbd_dn11 = 0.0;
        locals.var_cgsbd_dn12 = 0.0;
        locals.var_cgsbd_rv = 0.0;

        let assign24190_e28899: f64 = (p.p33 * locals.var_cgsbd);
        locals.var_cgsbd = assign24190_e28899;
        locals.var_cgsbd_dn0 = (p.p33 * locals.var_cgsbd_dn0);
        locals.var_cgsbd_dn2 = (p.p33 * locals.var_cgsbd_dn2);
        locals.var_cgsbd_dn4 = (p.p33 * locals.var_cgsbd_dn4);
        locals.var_cgsbd_dn5 = (p.p33 * locals.var_cgsbd_dn5);
        locals.var_cgsbd_dn6 = (p.p33 * locals.var_cgsbd_dn6);
        locals.var_cgsbd_dn8 = (p.p33 * locals.var_cgsbd_dn8);
        locals.var_cgsbd_dn10 = (p.p33 * locals.var_cgsbd_dn10);
        locals.var_cgsbd_dn11 = (p.p33 * locals.var_cgsbd_dn11);
        locals.var_cgsbd_dn12 = (p.p33 * locals.var_cgsbd_dn12);
        locals.var_cgsbd_rv = 0.0;

        let assign24500_e28994: f64 = if ((p.p28 != 0.0) && (p.p237 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard443 = assign24500_e28994;
        locals.var_guard443_rv = 0.0;

        let assign24510_e28999: f64 = if (((p.p27 != 0.0) && (p.p15 != 0.0)) && (p.p16 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard444 = assign24510_e28999;
        locals.var_guard444_rv = 0.0;

    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let eq1_e346: f64 = (locals.var_igidl + locals.var_isub);
        let eq1_e346_d_n0: f64 = (locals.var_igidl_dn0 + locals.var_isub_dn0);
        let eq1_e346_d_n2: f64 = (locals.var_igidl_dn2 + locals.var_isub_dn2);
        let eq1_e346_d_n4: f64 = (locals.var_igidl_dn4 + locals.var_isub_dn4);
        let eq1_e346_d_n5: f64 = (locals.var_igidl_dn5 + locals.var_isub_dn5);
        let eq1_e346_d_n6: f64 = (locals.var_igidl_dn6 + locals.var_isub_dn6);
        let eq1_e346_d_n8: f64 = (locals.var_igidl_dn8 + locals.var_isub_dn8);
        let eq1_e346_d_n10: f64 = (locals.var_igidl_dn10 + locals.var_isub_dn10);
        let eq1_e346_d_n11: f64 = (locals.var_igidl_dn11 + locals.var_isub_dn11);
        let eq1_e346_d_n12: f64 = (locals.var_igidl_dn12 + locals.var_isub_dn12);
        let eq1_e347: f64 = (p.p33 * eq1_e346);
        let eq1_e347_d_n0: f64 = (p.p33 * eq1_e346_d_n0);
        let eq1_e347_d_n2: f64 = (p.p33 * eq1_e346_d_n2);
        let eq1_e347_d_n4: f64 = (p.p33 * eq1_e346_d_n4);
        let eq1_e347_d_n5: f64 = (p.p33 * eq1_e346_d_n5);
        let eq1_e347_d_n6: f64 = (p.p33 * eq1_e346_d_n6);
        let eq1_e347_d_n8: f64 = (p.p33 * eq1_e346_d_n8);
        let eq1_e347_d_n10: f64 = (p.p33 * eq1_e346_d_n10);
        let eq1_e347_d_n11: f64 = (p.p33 * eq1_e346_d_n11);
        let eq1_e347_d_n12: f64 = (p.p33 * eq1_e346_d_n12);
        let eq1_value: f64 = eq1_e347;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(11),
            Some(12),
            multiplicity * (eq1_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq1_e347_d_n0), multiplicity * (eq1_e347_d_n2), multiplicity * (eq1_e347_d_n4), multiplicity * (eq1_e347_d_n5), multiplicity * (eq1_e347_d_n6), multiplicity * (eq1_e347_d_n8), multiplicity * (eq1_e347_d_n10), multiplicity * (eq1_e347_d_n11), multiplicity * (eq1_e347_d_n12)],
            [],
            [],
            1.0,
        );
        let eq2_e351: f64 = (locals.var_igisl + locals.var_isubs);
        let eq2_e351_d_n0: f64 = (locals.var_igisl_dn0 + locals.var_isubs_dn0);
        let eq2_e351_d_n2: f64 = (locals.var_igisl_dn2 + locals.var_isubs_dn2);
        let eq2_e351_d_n4: f64 = (locals.var_igisl_dn4 + locals.var_isubs_dn4);
        let eq2_e351_d_n5: f64 = (locals.var_igisl_dn5 + locals.var_isubs_dn5);
        let eq2_e351_d_n6: f64 = (locals.var_igisl_dn6 + locals.var_isubs_dn6);
        let eq2_e351_d_n8: f64 = (locals.var_igisl_dn8 + locals.var_isubs_dn8);
        let eq2_e351_d_n10: f64 = (locals.var_igisl_dn10 + locals.var_isubs_dn10);
        let eq2_e351_d_n11: f64 = (locals.var_igisl_dn11 + locals.var_isubs_dn11);
        let eq2_e351_d_n12: f64 = (locals.var_igisl_dn12 + locals.var_isubs_dn12);
        let eq2_e352: f64 = (p.p33 * eq2_e351);
        let eq2_e352_d_n0: f64 = (p.p33 * eq2_e351_d_n0);
        let eq2_e352_d_n2: f64 = (p.p33 * eq2_e351_d_n2);
        let eq2_e352_d_n4: f64 = (p.p33 * eq2_e351_d_n4);
        let eq2_e352_d_n5: f64 = (p.p33 * eq2_e351_d_n5);
        let eq2_e352_d_n6: f64 = (p.p33 * eq2_e351_d_n6);
        let eq2_e352_d_n8: f64 = (p.p33 * eq2_e351_d_n8);
        let eq2_e352_d_n10: f64 = (p.p33 * eq2_e351_d_n10);
        let eq2_e352_d_n11: f64 = (p.p33 * eq2_e351_d_n11);
        let eq2_e352_d_n12: f64 = (p.p33 * eq2_e351_d_n12);
        let eq2_value: f64 = eq2_e352;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(12),
            Some(11),
            multiplicity * (eq2_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq2_e352_d_n0), multiplicity * (eq2_e352_d_n2), multiplicity * (eq2_e352_d_n4), multiplicity * (eq2_e352_d_n5), multiplicity * (eq2_e352_d_n6), multiplicity * (eq2_e352_d_n8), multiplicity * (eq2_e352_d_n10), multiplicity * (eq2_e352_d_n11), multiplicity * (eq2_e352_d_n12)],
            [],
            [],
            1.0,
        );
        let eq3_e355: f64 = (p.p33 * locals.var_igs);
        let eq3_e355_d_n0: f64 = (p.p33 * locals.var_igs_dn0);
        let eq3_e355_d_n2: f64 = (p.p33 * locals.var_igs_dn2);
        let eq3_e355_d_n4: f64 = (p.p33 * locals.var_igs_dn4);
        let eq3_e355_d_n5: f64 = (p.p33 * locals.var_igs_dn5);
        let eq3_e355_d_n6: f64 = (p.p33 * locals.var_igs_dn6);
        let eq3_e355_d_n8: f64 = (p.p33 * locals.var_igs_dn8);
        let eq3_e355_d_n10: f64 = (p.p33 * locals.var_igs_dn10);
        let eq3_e355_d_n11: f64 = (p.p33 * locals.var_igs_dn11);
        let eq3_e355_d_n12: f64 = (p.p33 * locals.var_igs_dn12);
        let eq3_value: f64 = eq3_e355;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(5),
            Some(12),
            multiplicity * (eq3_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq3_e355_d_n0), multiplicity * (eq3_e355_d_n2), multiplicity * (eq3_e355_d_n4), multiplicity * (eq3_e355_d_n5), multiplicity * (eq3_e355_d_n6), multiplicity * (eq3_e355_d_n8), multiplicity * (eq3_e355_d_n10), multiplicity * (eq3_e355_d_n11), multiplicity * (eq3_e355_d_n12)],
            [],
            [],
            1.0,
        );
        let eq4_e358: f64 = (p.p33 * locals.var_igd);
        let eq4_e358_d_n0: f64 = (p.p33 * locals.var_igd_dn0);
        let eq4_e358_d_n2: f64 = (p.p33 * locals.var_igd_dn2);
        let eq4_e358_d_n4: f64 = (p.p33 * locals.var_igd_dn4);
        let eq4_e358_d_n5: f64 = (p.p33 * locals.var_igd_dn5);
        let eq4_e358_d_n6: f64 = (p.p33 * locals.var_igd_dn6);
        let eq4_e358_d_n8: f64 = (p.p33 * locals.var_igd_dn8);
        let eq4_e358_d_n10: f64 = (p.p33 * locals.var_igd_dn10);
        let eq4_e358_d_n11: f64 = (p.p33 * locals.var_igd_dn11);
        let eq4_e358_d_n12: f64 = (p.p33 * locals.var_igd_dn12);
        let eq4_value: f64 = eq4_e358;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(5),
            Some(11),
            multiplicity * (eq4_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq4_e358_d_n0), multiplicity * (eq4_e358_d_n2), multiplicity * (eq4_e358_d_n4), multiplicity * (eq4_e358_d_n5), multiplicity * (eq4_e358_d_n6), multiplicity * (eq4_e358_d_n8), multiplicity * (eq4_e358_d_n10), multiplicity * (eq4_e358_d_n11), multiplicity * (eq4_e358_d_n12)],
            [],
            [],
            1.0,
        );
        let eq5_e361: f64 = (p.p33 * locals.var_igb);
        let eq5_e361_d_n0: f64 = (p.p33 * locals.var_igb_dn0);
        let eq5_e361_d_n2: f64 = (p.p33 * locals.var_igb_dn2);
        let eq5_e361_d_n4: f64 = (p.p33 * locals.var_igb_dn4);
        let eq5_e361_d_n5: f64 = (p.p33 * locals.var_igb_dn5);
        let eq5_e361_d_n6: f64 = (p.p33 * locals.var_igb_dn6);
        let eq5_e361_d_n8: f64 = (p.p33 * locals.var_igb_dn8);
        let eq5_e361_d_n10: f64 = (p.p33 * locals.var_igb_dn10);
        let eq5_e361_d_n11: f64 = (p.p33 * locals.var_igb_dn11);
        let eq5_e361_d_n12: f64 = (p.p33 * locals.var_igb_dn12);
        let eq5_value: f64 = eq5_e361;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq5_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq5_e361_d_n0), multiplicity * (eq5_e361_d_n2), multiplicity * (eq5_e361_d_n4), multiplicity * (eq5_e361_d_n5), multiplicity * (eq5_e361_d_n6), multiplicity * (eq5_e361_d_n8), multiplicity * (eq5_e361_d_n10), multiplicity * (eq5_e361_d_n11), multiplicity * (eq5_e361_d_n12)],
            [],
            [],
            1.0,
        );
        let eq10_e387: f64 = (locals.var_qg + locals.var_qg_nqs);
        let eq10_e387_d_n8: f64 = (locals.var_qg_dn8 + locals.var_qg_nqs_dn8);
        let eq10_e388: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq10_e387);
        let eq10_e389: f64 = (p.p33 * eq10_e388);
        let eq10_e389_d_n0: f64 = (p.p33 * (locals.var_qg_dn0 * ddt_scale));
        let eq10_e389_d_n2: f64 = (p.p33 * (locals.var_qg_dn2 * ddt_scale));
        let eq10_e389_d_n4: f64 = (p.p33 * (locals.var_qg_dn4 * ddt_scale));
        let eq10_e389_d_n5: f64 = (p.p33 * (locals.var_qg_dn5 * ddt_scale));
        let eq10_e389_d_n6: f64 = (p.p33 * (locals.var_qg_dn6 * ddt_scale));
        let eq10_e389_d_n8: f64 = (p.p33 * (eq10_e387_d_n8 * ddt_scale));
        let eq10_e389_d_n9: f64 = (p.p33 * (locals.var_qg_nqs_dn9 * ddt_scale));
        let eq10_e389_d_n10: f64 = (p.p33 * (locals.var_qg_dn10 * ddt_scale));
        let eq10_e389_d_n11: f64 = (p.p33 * (locals.var_qg_dn11 * ddt_scale));
        let eq10_e389_d_n12: f64 = (p.p33 * (locals.var_qg_dn12 * ddt_scale));
        let eq10_value: f64 = eq10_e389;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(12),
            multiplicity * (eq10_value),
            [0, 2, 4, 5, 6, 8, 9, 10, 11, 12],
            [multiplicity * (eq10_e389_d_n0), multiplicity * (eq10_e389_d_n2), multiplicity * (eq10_e389_d_n4), multiplicity * (eq10_e389_d_n5), multiplicity * (eq10_e389_d_n6), multiplicity * (eq10_e389_d_n8), multiplicity * (eq10_e389_d_n9), multiplicity * (eq10_e389_d_n10), multiplicity * (eq10_e389_d_n11), multiplicity * (eq10_e389_d_n12)],
            [],
            [],
            1.0,
        );
        let eq11_e393: f64 = (locals.var_qd + locals.var_qd_nqs);
        let eq11_e393_d_n0: f64 = (locals.var_qd_dn0 + locals.var_qd_nqs_dn0);
        let eq11_e393_d_n2: f64 = (locals.var_qd_dn2 + locals.var_qd_nqs_dn2);
        let eq11_e393_d_n4: f64 = (locals.var_qd_dn4 + locals.var_qd_nqs_dn4);
        let eq11_e393_d_n5: f64 = (locals.var_qd_dn5 + locals.var_qd_nqs_dn5);
        let eq11_e393_d_n6: f64 = (locals.var_qd_dn6 + locals.var_qd_nqs_dn6);
        let eq11_e393_d_n8: f64 = (locals.var_qd_dn8 + locals.var_qd_nqs_dn8);
        let eq11_e393_d_n10: f64 = (locals.var_qd_dn10 + locals.var_qd_nqs_dn10);
        let eq11_e393_d_n11: f64 = (locals.var_qd_dn11 + locals.var_qd_nqs_dn11);
        let eq11_e393_d_n12: f64 = (locals.var_qd_dn12 + locals.var_qd_nqs_dn12);
        let eq11_e394: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq11_e393);
        let eq11_e395: f64 = (p.p33 * eq11_e394);
        let eq11_e395_d_n0: f64 = (p.p33 * (eq11_e393_d_n0 * ddt_scale));
        let eq11_e395_d_n2: f64 = (p.p33 * (eq11_e393_d_n2 * ddt_scale));
        let eq11_e395_d_n4: f64 = (p.p33 * (eq11_e393_d_n4 * ddt_scale));
        let eq11_e395_d_n5: f64 = (p.p33 * (eq11_e393_d_n5 * ddt_scale));
        let eq11_e395_d_n6: f64 = (p.p33 * (eq11_e393_d_n6 * ddt_scale));
        let eq11_e395_d_n8: f64 = (p.p33 * (eq11_e393_d_n8 * ddt_scale));
        let eq11_e395_d_n10: f64 = (p.p33 * (eq11_e393_d_n10 * ddt_scale));
        let eq11_e395_d_n11: f64 = (p.p33 * (eq11_e393_d_n11 * ddt_scale));
        let eq11_e395_d_n12: f64 = (p.p33 * (eq11_e393_d_n12 * ddt_scale));
        let eq11_value: f64 = eq11_e395;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(11),
            Some(12),
            multiplicity * (eq11_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq11_e395_d_n0), multiplicity * (eq11_e395_d_n2), multiplicity * (eq11_e395_d_n4), multiplicity * (eq11_e395_d_n5), multiplicity * (eq11_e395_d_n6), multiplicity * (eq11_e395_d_n8), multiplicity * (eq11_e395_d_n10), multiplicity * (eq11_e395_d_n11), multiplicity * (eq11_e395_d_n12)],
            [],
            [],
            1.0,
        );
        let eq12_e399: f64 = (locals.var_qb + locals.var_qb_nqs);
        let eq12_e400: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq12_e399);
        let eq12_e401: f64 = (p.p33 * eq12_e400);
        let eq12_e401_d_n0: f64 = (p.p33 * (locals.var_qb_dn0 * ddt_scale));
        let eq12_e401_d_n2: f64 = (p.p33 * (locals.var_qb_dn2 * ddt_scale));
        let eq12_e401_d_n4: f64 = (p.p33 * (locals.var_qb_dn4 * ddt_scale));
        let eq12_e401_d_n5: f64 = (p.p33 * (locals.var_qb_dn5 * ddt_scale));
        let eq12_e401_d_n6: f64 = (p.p33 * (locals.var_qb_dn6 * ddt_scale));
        let eq12_e401_d_n8: f64 = (p.p33 * (locals.var_qb_dn8 * ddt_scale));
        let eq12_e401_d_n9: f64 = (p.p33 * (locals.var_qb_nqs_dn9 * ddt_scale));
        let eq12_e401_d_n10: f64 = (p.p33 * (locals.var_qb_dn10 * ddt_scale));
        let eq12_e401_d_n11: f64 = (p.p33 * (locals.var_qb_dn11 * ddt_scale));
        let eq12_e401_d_n12: f64 = (p.p33 * (locals.var_qb_dn12 * ddt_scale));
        let eq12_value: f64 = eq12_e401;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(12),
            multiplicity * (eq12_value),
            [0, 2, 4, 5, 6, 8, 9, 10, 11, 12],
            [multiplicity * (eq12_e401_d_n0), multiplicity * (eq12_e401_d_n2), multiplicity * (eq12_e401_d_n4), multiplicity * (eq12_e401_d_n5), multiplicity * (eq12_e401_d_n6), multiplicity * (eq12_e401_d_n8), multiplicity * (eq12_e401_d_n9), multiplicity * (eq12_e401_d_n10), multiplicity * (eq12_e401_d_n11), multiplicity * (eq12_e401_d_n12)],
            [],
            [],
            1.0,
        );
        let eq17_e427: f64 = (locals.var_ci * (nv7 - 0.0));
        let eq17_e427_d_n0: f64 = (locals.var_ci_dn0 * (nv7 - 0.0));
        let eq17_e427_d_n2: f64 = (locals.var_ci_dn2 * (nv7 - 0.0));
        let eq17_e427_d_n4: f64 = (locals.var_ci_dn4 * (nv7 - 0.0));
        let eq17_e427_d_n5: f64 = (locals.var_ci_dn5 * (nv7 - 0.0));
        let eq17_e427_d_n6: f64 = (locals.var_ci_dn6 * (nv7 - 0.0));
        let eq17_e427_d_n8: f64 = (locals.var_ci_dn8 * (nv7 - 0.0));
        let eq17_e427_d_n10: f64 = (locals.var_ci_dn10 * (nv7 - 0.0));
        let eq17_e427_d_n11: f64 = (locals.var_ci_dn11 * (nv7 - 0.0));
        let eq17_e427_d_n12: f64 = (locals.var_ci_dn12 * (nv7 - 0.0));
        let eq17_value: f64 = eq17_e427;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(11),
            Some(12),
            multiplicity * (eq17_value),
            [0, 2, 4, 5, 6, 7, 8, 10, 11, 12],
            [multiplicity * (eq17_e427_d_n0), multiplicity * (eq17_e427_d_n2), multiplicity * (eq17_e427_d_n4), multiplicity * (eq17_e427_d_n5), multiplicity * (eq17_e427_d_n6), multiplicity * (locals.var_ci), multiplicity * (eq17_e427_d_n8), multiplicity * (eq17_e427_d_n10), multiplicity * (eq17_e427_d_n11), multiplicity * (eq17_e427_d_n12)],
            [],
            [],
            1.0,
        );
        let eq18_e430: f64 = ((nv7 - 0.0) * locals.var_sigrat_s);
        let eq18_e430_d_n0: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn0);
        let eq18_e430_d_n2: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn2);
        let eq18_e430_d_n4: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn4);
        let eq18_e430_d_n5: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn5);
        let eq18_e430_d_n6: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn6);
        let eq18_e430_d_n8: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn8);
        let eq18_e430_d_n10: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn10);
        let eq18_e430_d_n11: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn11);
        let eq18_e430_d_n12: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn12);
        let eq18_e431: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq18_e430);
        let eq18_value: f64 = eq18_e431;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(12),
            multiplicity * (eq18_value),
            [0, 2, 4, 5, 6, 7, 8, 10, 11, 12],
            [multiplicity * ((eq18_e430_d_n0 * ddt_scale)), multiplicity * ((eq18_e430_d_n2 * ddt_scale)), multiplicity * ((eq18_e430_d_n4 * ddt_scale)), multiplicity * ((eq18_e430_d_n5 * ddt_scale)), multiplicity * ((eq18_e430_d_n6 * ddt_scale)), multiplicity * ((locals.var_sigrat_s * ddt_scale)), multiplicity * ((eq18_e430_d_n8 * ddt_scale)), multiplicity * ((eq18_e430_d_n10 * ddt_scale)), multiplicity * ((eq18_e430_d_n11 * ddt_scale)), multiplicity * ((eq18_e430_d_n12 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq19_e434: f64 = ((nv7 - 0.0) * locals.var_sigrat_d);
        let eq19_e434_d_n0: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn0);
        let eq19_e434_d_n2: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn2);
        let eq19_e434_d_n4: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn4);
        let eq19_e434_d_n5: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn5);
        let eq19_e434_d_n6: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn6);
        let eq19_e434_d_n8: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn8);
        let eq19_e434_d_n10: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn10);
        let eq19_e434_d_n11: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn11);
        let eq19_e434_d_n12: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn12);
        let eq19_e435: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq19_e434);
        let eq19_value: f64 = eq19_e435;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(11),
            multiplicity * (eq19_value),
            [0, 2, 4, 5, 6, 7, 8, 10, 11, 12],
            [multiplicity * ((eq19_e434_d_n0 * ddt_scale)), multiplicity * ((eq19_e434_d_n2 * ddt_scale)), multiplicity * ((eq19_e434_d_n4 * ddt_scale)), multiplicity * ((eq19_e434_d_n5 * ddt_scale)), multiplicity * ((eq19_e434_d_n6 * ddt_scale)), multiplicity * ((locals.var_sigrat_d * ddt_scale)), multiplicity * ((eq19_e434_d_n8 * ddt_scale)), multiplicity * ((eq19_e434_d_n10 * ddt_scale)), multiplicity * ((eq19_e434_d_n11 * ddt_scale)), multiplicity * ((eq19_e434_d_n12 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let (eq28_e498, eq28_e498_d_n0, eq28_e498_d_n2, eq28_e498_d_n4, eq28_e498_d_n5, eq28_e498_d_n6, eq28_e498_d_n8, eq28_e498_d_n10, eq28_e498_d_n11, eq28_e498_d_n12,) = {
    if (locals.var_guard443 != 0.0) {
        let eq28_e487: f64 = (-locals.var_rpower);
        let eq28_e490: f64 = (locals.var_cthe * (nv4 - 0.0));
        let eq28_e490_d_n0: f64 = (locals.var_cthe_dn0 * (nv4 - 0.0));
        let eq28_e490_d_n2: f64 = (locals.var_cthe_dn2 * (nv4 - 0.0));
        let eq28_e490_d_n4: f64 = ((locals.var_cthe_dn4 * (nv4 - 0.0)) + locals.var_cthe);
        let eq28_e490_d_n5: f64 = (locals.var_cthe_dn5 * (nv4 - 0.0));
        let eq28_e490_d_n6: f64 = (locals.var_cthe_dn6 * (nv4 - 0.0));
        let eq28_e490_d_n8: f64 = (locals.var_cthe_dn8 * (nv4 - 0.0));
        let eq28_e490_d_n10: f64 = (locals.var_cthe_dn10 * (nv4 - 0.0));
        let eq28_e490_d_n11: f64 = (locals.var_cthe_dn11 * (nv4 - 0.0));
        let eq28_e490_d_n12: f64 = (locals.var_cthe_dn12 * (nv4 - 0.0));
        let eq28_e491: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq28_e490);
        let eq28_e492: f64 = (eq28_e487 + eq28_e491);
        let eq28_e492_d_n0: f64 = ((-locals.var_rpower_dn0) + (eq28_e490_d_n0 * ddt_scale));
        let eq28_e492_d_n2: f64 = ((-locals.var_rpower_dn2) + (eq28_e490_d_n2 * ddt_scale));
        let eq28_e492_d_n4: f64 = ((-locals.var_rpower_dn4) + (eq28_e490_d_n4 * ddt_scale));
        let eq28_e492_d_n5: f64 = ((-locals.var_rpower_dn5) + (eq28_e490_d_n5 * ddt_scale));
        let eq28_e492_d_n6: f64 = ((-locals.var_rpower_dn6) + (eq28_e490_d_n6 * ddt_scale));
        let eq28_e492_d_n8: f64 = ((-locals.var_rpower_dn8) + (eq28_e490_d_n8 * ddt_scale));
        let eq28_e492_d_n10: f64 = ((-locals.var_rpower_dn10) + (eq28_e490_d_n10 * ddt_scale));
        let eq28_e492_d_n11: f64 = ((-locals.var_rpower_dn11) + (eq28_e490_d_n11 * ddt_scale));
        let eq28_e492_d_n12: f64 = ((-locals.var_rpower_dn12) + (eq28_e490_d_n12 * ddt_scale));
        let eq28_e495: f64 = ((nv4 - 0.0) * locals.var_gth);
        let eq28_e495_d_n0: f64 = ((nv4 - 0.0) * locals.var_gth_dn0);
        let eq28_e495_d_n2: f64 = ((nv4 - 0.0) * locals.var_gth_dn2);
        let eq28_e495_d_n4: f64 = (locals.var_gth + ((nv4 - 0.0) * locals.var_gth_dn4));
        let eq28_e495_d_n5: f64 = ((nv4 - 0.0) * locals.var_gth_dn5);
        let eq28_e495_d_n6: f64 = ((nv4 - 0.0) * locals.var_gth_dn6);
        let eq28_e495_d_n8: f64 = ((nv4 - 0.0) * locals.var_gth_dn8);
        let eq28_e495_d_n10: f64 = ((nv4 - 0.0) * locals.var_gth_dn10);
        let eq28_e495_d_n11: f64 = ((nv4 - 0.0) * locals.var_gth_dn11);
        let eq28_e495_d_n12: f64 = ((nv4 - 0.0) * locals.var_gth_dn12);
        let eq28_e496: f64 = (eq28_e492 + eq28_e495);
        let eq28_e496_d_n0: f64 = (eq28_e492_d_n0 + eq28_e495_d_n0);
        let eq28_e496_d_n2: f64 = (eq28_e492_d_n2 + eq28_e495_d_n2);
        let eq28_e496_d_n4: f64 = (eq28_e492_d_n4 + eq28_e495_d_n4);
        let eq28_e496_d_n5: f64 = (eq28_e492_d_n5 + eq28_e495_d_n5);
        let eq28_e496_d_n6: f64 = (eq28_e492_d_n6 + eq28_e495_d_n6);
        let eq28_e496_d_n8: f64 = (eq28_e492_d_n8 + eq28_e495_d_n8);
        let eq28_e496_d_n10: f64 = (eq28_e492_d_n10 + eq28_e495_d_n10);
        let eq28_e496_d_n11: f64 = (eq28_e492_d_n11 + eq28_e495_d_n11);
        let eq28_e496_d_n12: f64 = (eq28_e492_d_n12 + eq28_e495_d_n12);
        (eq28_e496, eq28_e496_d_n0, eq28_e496_d_n2, eq28_e496_d_n4, eq28_e496_d_n5, eq28_e496_d_n6, eq28_e496_d_n8, eq28_e496_d_n10, eq28_e496_d_n11, eq28_e496_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e498;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(4),
            None,
            multiplicity * (eq28_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq28_e498_d_n0), multiplicity * (eq28_e498_d_n2), multiplicity * (eq28_e498_d_n4), multiplicity * (eq28_e498_d_n5), multiplicity * (eq28_e498_d_n6), multiplicity * (eq28_e498_d_n8), multiplicity * (eq28_e498_d_n10), multiplicity * (eq28_e498_d_n11), multiplicity * (eq28_e498_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq30_e512, eq30_e512_d_n0, eq30_e512_d_n2, eq30_e512_d_n4, eq30_e512_d_n5, eq30_e512_d_n6, eq30_e512_d_n8, eq30_e512_d_n10, eq30_e512_d_n11, eq30_e512_d_n12,) = {
    if (locals.var_guard444 != 0.0) {
        let eq30_e508: f64 = (1e-9 * (nv10 - 0.0));
        let eq30_e509: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq30_e508);
        let eq30_e510: f64 = (locals.var_iqh_nqs + eq30_e509);
        let eq30_e510_d_n10: f64 = (locals.var_iqh_nqs_dn10 + (1e-9 * ddt_scale));
        (eq30_e510, locals.var_iqh_nqs_dn0, locals.var_iqh_nqs_dn2, locals.var_iqh_nqs_dn4, locals.var_iqh_nqs_dn5, locals.var_iqh_nqs_dn6, locals.var_iqh_nqs_dn8, eq30_e510_d_n10, locals.var_iqh_nqs_dn11, locals.var_iqh_nqs_dn12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e512;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(10),
            None,
            multiplicity * (eq30_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq30_e512_d_n0), multiplicity * (eq30_e512_d_n2), multiplicity * (eq30_e512_d_n4), multiplicity * (eq30_e512_d_n5), multiplicity * (eq30_e512_d_n6), multiplicity * (eq30_e512_d_n8), multiplicity * (eq30_e512_d_n10), multiplicity * (eq30_e512_d_n11), multiplicity * (eq30_e512_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq32_e526, eq32_e526_d_n0, eq32_e526_d_n2, eq32_e526_d_n4, eq32_e526_d_n5, eq32_e526_d_n6, eq32_e526_d_n8, eq32_e526_d_n10, eq32_e526_d_n11, eq32_e526_d_n12,) = {
    if (p.p24 != 0.0) {
        let eq32_e522: f64 = (1e-9 * (nv8 - 0.0));
        let eq32_e523: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq32_e522);
        let eq32_e524: f64 = (locals.var_iqi_nqs + eq32_e523);
        let eq32_e524_d_n8: f64 = (locals.var_iqi_nqs_dn8 + (1e-9 * ddt_scale));
        (eq32_e524, locals.var_iqi_nqs_dn0, locals.var_iqi_nqs_dn2, locals.var_iqi_nqs_dn4, locals.var_iqi_nqs_dn5, locals.var_iqi_nqs_dn6, eq32_e524_d_n8, locals.var_iqi_nqs_dn10, locals.var_iqi_nqs_dn11, locals.var_iqi_nqs_dn12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq32_value: f64 = eq32_e526;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(8),
            None,
            multiplicity * (eq32_value),
            [0, 2, 4, 5, 6, 8, 10, 11, 12],
            [multiplicity * (eq32_e526_d_n0), multiplicity * (eq32_e526_d_n2), multiplicity * (eq32_e526_d_n4), multiplicity * (eq32_e526_d_n5), multiplicity * (eq32_e526_d_n6), multiplicity * (eq32_e526_d_n8), multiplicity * (eq32_e526_d_n10), multiplicity * (eq32_e526_d_n11), multiplicity * (eq32_e526_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq33_e535, eq33_e535_d_n0, eq33_e535_d_n2, eq33_e535_d_n4, eq33_e535_d_n5, eq33_e535_d_n6, eq33_e535_d_n8, eq33_e535_d_n9, eq33_e535_d_n10, eq33_e535_d_n11, eq33_e535_d_n12,) = {
    if (p.p24 != 0.0) {
        let eq33_e531: f64 = (1e-9 * (nv9 - 0.0));
        let eq33_e532: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq33_e531);
        let eq33_e533: f64 = (locals.var_iqb_nqs + eq33_e532);
        let eq33_e533_d_n9: f64 = (locals.var_iqb_nqs_dn9 + (1e-9 * ddt_scale));
        (eq33_e533, locals.var_iqb_nqs_dn0, locals.var_iqb_nqs_dn2, locals.var_iqb_nqs_dn4, locals.var_iqb_nqs_dn5, locals.var_iqb_nqs_dn6, locals.var_iqb_nqs_dn8, eq33_e533_d_n9, locals.var_iqb_nqs_dn10, locals.var_iqb_nqs_dn11, locals.var_iqb_nqs_dn12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e535;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(9),
            None,
            multiplicity * (eq33_value),
            [0, 2, 4, 5, 6, 8, 9, 10, 11, 12],
            [multiplicity * (eq33_e535_d_n0), multiplicity * (eq33_e535_d_n2), multiplicity * (eq33_e535_d_n4), multiplicity * (eq33_e535_d_n5), multiplicity * (eq33_e535_d_n6), multiplicity * (eq33_e535_d_n8), multiplicity * (eq33_e535_d_n9), multiplicity * (eq33_e535_d_n10), multiplicity * (eq33_e535_d_n11), multiplicity * (eq33_e535_d_n12)],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let eq10_e387: f64 = (locals.var_qg + locals.var_qg_nqs);
        let eq10_e387_d_n8: f64 = (locals.var_qg_dn8 + locals.var_qg_nqs_dn8);
        let eq10_e388_q: f64 = eq10_e387;
        let eq10_e389: f64 = (p.p33 * eq10_e387);
        let eq10_e389_d_n0: f64 = (p.p33 * locals.var_qg_dn0);
        let eq10_e389_d_n2: f64 = (p.p33 * locals.var_qg_dn2);
        let eq10_e389_d_n4: f64 = (p.p33 * locals.var_qg_dn4);
        let eq10_e389_d_n5: f64 = (p.p33 * locals.var_qg_dn5);
        let eq10_e389_d_n6: f64 = (p.p33 * locals.var_qg_dn6);
        let eq10_e389_d_n8: f64 = (p.p33 * eq10_e387_d_n8);
        let eq10_e389_d_n9: f64 = (p.p33 * locals.var_qg_nqs_dn9);
        let eq10_e389_d_n10: f64 = (p.p33 * locals.var_qg_dn10);
        let eq10_e389_d_n11: f64 = (p.p33 * locals.var_qg_dn11);
        let eq10_e389_d_n12: f64 = (p.p33 * locals.var_qg_dn12);
        let eq10_e389_q: f64 = (p.p33 * eq10_e388_q);
        let eq10_reactive_node_derivatives: [f64; 13] = [eq10_e389_d_n0, 0.0, eq10_e389_d_n2, 0.0, eq10_e389_d_n4, eq10_e389_d_n5, eq10_e389_d_n6, 0.0, eq10_e389_d_n8, eq10_e389_d_n9, eq10_e389_d_n10, eq10_e389_d_n11, eq10_e389_d_n12];
        let eq10_reactive_branch_derivatives: [f64; 8] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[12]),
            nodes,
            &eq10_reactive_node_derivatives,
            branches,
            &eq10_reactive_branch_derivatives,
            multiplicity,
        );
        let eq11_e393: f64 = (locals.var_qd + locals.var_qd_nqs);
        let eq11_e393_d_n0: f64 = (locals.var_qd_dn0 + locals.var_qd_nqs_dn0);
        let eq11_e393_d_n2: f64 = (locals.var_qd_dn2 + locals.var_qd_nqs_dn2);
        let eq11_e393_d_n4: f64 = (locals.var_qd_dn4 + locals.var_qd_nqs_dn4);
        let eq11_e393_d_n5: f64 = (locals.var_qd_dn5 + locals.var_qd_nqs_dn5);
        let eq11_e393_d_n6: f64 = (locals.var_qd_dn6 + locals.var_qd_nqs_dn6);
        let eq11_e393_d_n8: f64 = (locals.var_qd_dn8 + locals.var_qd_nqs_dn8);
        let eq11_e393_d_n10: f64 = (locals.var_qd_dn10 + locals.var_qd_nqs_dn10);
        let eq11_e393_d_n11: f64 = (locals.var_qd_dn11 + locals.var_qd_nqs_dn11);
        let eq11_e393_d_n12: f64 = (locals.var_qd_dn12 + locals.var_qd_nqs_dn12);
        let eq11_e394_q: f64 = eq11_e393;
        let eq11_e395: f64 = (p.p33 * eq11_e393);
        let eq11_e395_d_n0: f64 = (p.p33 * eq11_e393_d_n0);
        let eq11_e395_d_n2: f64 = (p.p33 * eq11_e393_d_n2);
        let eq11_e395_d_n4: f64 = (p.p33 * eq11_e393_d_n4);
        let eq11_e395_d_n5: f64 = (p.p33 * eq11_e393_d_n5);
        let eq11_e395_d_n6: f64 = (p.p33 * eq11_e393_d_n6);
        let eq11_e395_d_n8: f64 = (p.p33 * eq11_e393_d_n8);
        let eq11_e395_d_n10: f64 = (p.p33 * eq11_e393_d_n10);
        let eq11_e395_d_n11: f64 = (p.p33 * eq11_e393_d_n11);
        let eq11_e395_d_n12: f64 = (p.p33 * eq11_e393_d_n12);
        let eq11_e395_q: f64 = (p.p33 * eq11_e394_q);
        let eq11_reactive_node_derivatives: [f64; 13] = [eq11_e395_d_n0, 0.0, eq11_e395_d_n2, 0.0, eq11_e395_d_n4, eq11_e395_d_n5, eq11_e395_d_n6, 0.0, eq11_e395_d_n8, 0.0, eq11_e395_d_n10, eq11_e395_d_n11, eq11_e395_d_n12];
        let eq11_reactive_branch_derivatives: [f64; 8] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[12]),
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
        let eq12_e399: f64 = (locals.var_qb + locals.var_qb_nqs);
        let eq12_e400_q: f64 = eq12_e399;
        let eq12_e401: f64 = (p.p33 * eq12_e399);
        let eq12_e401_d_n0: f64 = (p.p33 * locals.var_qb_dn0);
        let eq12_e401_d_n2: f64 = (p.p33 * locals.var_qb_dn2);
        let eq12_e401_d_n4: f64 = (p.p33 * locals.var_qb_dn4);
        let eq12_e401_d_n5: f64 = (p.p33 * locals.var_qb_dn5);
        let eq12_e401_d_n6: f64 = (p.p33 * locals.var_qb_dn6);
        let eq12_e401_d_n8: f64 = (p.p33 * locals.var_qb_dn8);
        let eq12_e401_d_n9: f64 = (p.p33 * locals.var_qb_nqs_dn9);
        let eq12_e401_d_n10: f64 = (p.p33 * locals.var_qb_dn10);
        let eq12_e401_d_n11: f64 = (p.p33 * locals.var_qb_dn11);
        let eq12_e401_d_n12: f64 = (p.p33 * locals.var_qb_dn12);
        let eq12_e401_q: f64 = (p.p33 * eq12_e400_q);
        let eq12_reactive_node_derivatives: [f64; 13] = [eq12_e401_d_n0, 0.0, eq12_e401_d_n2, 0.0, eq12_e401_d_n4, eq12_e401_d_n5, eq12_e401_d_n6, 0.0, eq12_e401_d_n8, eq12_e401_d_n9, eq12_e401_d_n10, eq12_e401_d_n11, eq12_e401_d_n12];
        let eq12_reactive_branch_derivatives: [f64; 8] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[12]),
            nodes,
            &eq12_reactive_node_derivatives,
            branches,
            &eq12_reactive_branch_derivatives,
            multiplicity,
        );
        let eq18_e430: f64 = ((nv7 - 0.0) * locals.var_sigrat_s);
        let eq18_e430_d_n0: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn0);
        let eq18_e430_d_n2: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn2);
        let eq18_e430_d_n4: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn4);
        let eq18_e430_d_n5: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn5);
        let eq18_e430_d_n6: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn6);
        let eq18_e430_d_n8: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn8);
        let eq18_e430_d_n10: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn10);
        let eq18_e430_d_n11: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn11);
        let eq18_e430_d_n12: f64 = ((nv7 - 0.0) * locals.var_sigrat_s_dn12);
        let eq18_e431_q: f64 = eq18_e430;
        let eq18_reactive_node_derivatives: [f64; 13] = [eq18_e430_d_n0, 0.0, eq18_e430_d_n2, 0.0, eq18_e430_d_n4, eq18_e430_d_n5, eq18_e430_d_n6, locals.var_sigrat_s, eq18_e430_d_n8, 0.0, eq18_e430_d_n10, eq18_e430_d_n11, eq18_e430_d_n12];
        let eq18_reactive_branch_derivatives: [f64; 8] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[12]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let eq19_e434: f64 = ((nv7 - 0.0) * locals.var_sigrat_d);
        let eq19_e434_d_n0: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn0);
        let eq19_e434_d_n2: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn2);
        let eq19_e434_d_n4: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn4);
        let eq19_e434_d_n5: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn5);
        let eq19_e434_d_n6: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn6);
        let eq19_e434_d_n8: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn8);
        let eq19_e434_d_n10: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn10);
        let eq19_e434_d_n11: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn11);
        let eq19_e434_d_n12: f64 = ((nv7 - 0.0) * locals.var_sigrat_d_dn12);
        let eq19_e435_q: f64 = eq19_e434;
        let eq19_reactive_node_derivatives: [f64; 13] = [eq19_e434_d_n0, 0.0, eq19_e434_d_n2, 0.0, eq19_e434_d_n4, eq19_e434_d_n5, eq19_e434_d_n6, locals.var_sigrat_d, eq19_e434_d_n8, 0.0, eq19_e434_d_n10, eq19_e434_d_n11, eq19_e434_d_n12];
        let eq19_reactive_branch_derivatives: [f64; 8] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[11]),
            nodes,
            &eq19_reactive_node_derivatives,
            branches,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq28_e498, eq28_e498_d_n0, eq28_e498_d_n2, eq28_e498_d_n4, eq28_e498_d_n5, eq28_e498_d_n6, eq28_e498_d_n8, eq28_e498_d_n10, eq28_e498_d_n11, eq28_e498_d_n12, eq28_e498_q, eq28_e498_q_d_n0, eq28_e498_q_d_n2, eq28_e498_q_d_n4, eq28_e498_q_d_n5, eq28_e498_q_d_n6, eq28_e498_q_d_n8, eq28_e498_q_d_n10, eq28_e498_q_d_n11, eq28_e498_q_d_n12,) = {
    if (locals.var_guard443 != 0.0) {
        let eq28_e487: f64 = (-locals.var_rpower);
        let eq28_e490: f64 = (locals.var_cthe * (nv4 - 0.0));
        let eq28_e490_d_n0: f64 = (locals.var_cthe_dn0 * (nv4 - 0.0));
        let eq28_e490_d_n2: f64 = (locals.var_cthe_dn2 * (nv4 - 0.0));
        let eq28_e490_d_n4: f64 = ((locals.var_cthe_dn4 * (nv4 - 0.0)) + locals.var_cthe);
        let eq28_e490_d_n5: f64 = (locals.var_cthe_dn5 * (nv4 - 0.0));
        let eq28_e490_d_n6: f64 = (locals.var_cthe_dn6 * (nv4 - 0.0));
        let eq28_e490_d_n8: f64 = (locals.var_cthe_dn8 * (nv4 - 0.0));
        let eq28_e490_d_n10: f64 = (locals.var_cthe_dn10 * (nv4 - 0.0));
        let eq28_e490_d_n11: f64 = (locals.var_cthe_dn11 * (nv4 - 0.0));
        let eq28_e490_d_n12: f64 = (locals.var_cthe_dn12 * (nv4 - 0.0));
        let eq28_e491_q: f64 = eq28_e490;
        let eq28_e492: f64 = (eq28_e487 + eq28_e490);
        let eq28_e492_d_n0: f64 = ((-locals.var_rpower_dn0) + eq28_e490_d_n0);
        let eq28_e492_d_n2: f64 = ((-locals.var_rpower_dn2) + eq28_e490_d_n2);
        let eq28_e492_d_n4: f64 = ((-locals.var_rpower_dn4) + eq28_e490_d_n4);
        let eq28_e492_d_n5: f64 = ((-locals.var_rpower_dn5) + eq28_e490_d_n5);
        let eq28_e492_d_n6: f64 = ((-locals.var_rpower_dn6) + eq28_e490_d_n6);
        let eq28_e492_d_n8: f64 = ((-locals.var_rpower_dn8) + eq28_e490_d_n8);
        let eq28_e492_d_n10: f64 = ((-locals.var_rpower_dn10) + eq28_e490_d_n10);
        let eq28_e492_d_n11: f64 = ((-locals.var_rpower_dn11) + eq28_e490_d_n11);
        let eq28_e492_d_n12: f64 = ((-locals.var_rpower_dn12) + eq28_e490_d_n12);
        let eq28_e492_q: f64 = eq28_e491_q;
        let eq28_e495: f64 = ((nv4 - 0.0) * locals.var_gth);
        let eq28_e495_d_n0: f64 = ((nv4 - 0.0) * locals.var_gth_dn0);
        let eq28_e495_d_n2: f64 = ((nv4 - 0.0) * locals.var_gth_dn2);
        let eq28_e495_d_n4: f64 = (locals.var_gth + ((nv4 - 0.0) * locals.var_gth_dn4));
        let eq28_e495_d_n5: f64 = ((nv4 - 0.0) * locals.var_gth_dn5);
        let eq28_e495_d_n6: f64 = ((nv4 - 0.0) * locals.var_gth_dn6);
        let eq28_e495_d_n8: f64 = ((nv4 - 0.0) * locals.var_gth_dn8);
        let eq28_e495_d_n10: f64 = ((nv4 - 0.0) * locals.var_gth_dn10);
        let eq28_e495_d_n11: f64 = ((nv4 - 0.0) * locals.var_gth_dn11);
        let eq28_e495_d_n12: f64 = ((nv4 - 0.0) * locals.var_gth_dn12);
        let eq28_e496: f64 = (eq28_e492 + eq28_e495);
        let eq28_e496_d_n0: f64 = (eq28_e492_d_n0 + eq28_e495_d_n0);
        let eq28_e496_d_n2: f64 = (eq28_e492_d_n2 + eq28_e495_d_n2);
        let eq28_e496_d_n4: f64 = (eq28_e492_d_n4 + eq28_e495_d_n4);
        let eq28_e496_d_n5: f64 = (eq28_e492_d_n5 + eq28_e495_d_n5);
        let eq28_e496_d_n6: f64 = (eq28_e492_d_n6 + eq28_e495_d_n6);
        let eq28_e496_d_n8: f64 = (eq28_e492_d_n8 + eq28_e495_d_n8);
        let eq28_e496_d_n10: f64 = (eq28_e492_d_n10 + eq28_e495_d_n10);
        let eq28_e496_d_n11: f64 = (eq28_e492_d_n11 + eq28_e495_d_n11);
        let eq28_e496_d_n12: f64 = (eq28_e492_d_n12 + eq28_e495_d_n12);
        let eq28_e496_q: f64 = eq28_e492_q;
        (eq28_e496, eq28_e496_d_n0, eq28_e496_d_n2, eq28_e496_d_n4, eq28_e496_d_n5, eq28_e496_d_n6, eq28_e496_d_n8, eq28_e496_d_n10, eq28_e496_d_n11, eq28_e496_d_n12, eq28_e496_q, eq28_e490_d_n0, eq28_e490_d_n2, eq28_e490_d_n4, eq28_e490_d_n5, eq28_e490_d_n6, eq28_e490_d_n8, eq28_e490_d_n10, eq28_e490_d_n11, eq28_e490_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_reactive_node_derivatives: [f64; 13] = [eq28_e498_q_d_n0, 0.0, eq28_e498_q_d_n2, 0.0, eq28_e498_q_d_n4, eq28_e498_q_d_n5, eq28_e498_q_d_n6, 0.0, eq28_e498_q_d_n8, 0.0, eq28_e498_q_d_n10, eq28_e498_q_d_n11, eq28_e498_q_d_n12];
        let eq28_reactive_branch_derivatives: [f64; 8] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq28_reactive_node_derivatives,
            branches,
            &eq28_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq30_e512, eq30_e512_d_n0, eq30_e512_d_n2, eq30_e512_d_n4, eq30_e512_d_n5, eq30_e512_d_n6, eq30_e512_d_n8, eq30_e512_d_n10, eq30_e512_d_n11, eq30_e512_d_n12, eq30_e512_q, eq30_e512_q_d_n10,) = {
    if (locals.var_guard444 != 0.0) {
        let eq30_e508: f64 = (1e-9 * (nv10 - 0.0));
        let eq30_e509_q: f64 = eq30_e508;
        let eq30_e510: f64 = (locals.var_iqh_nqs + eq30_e508);
        let eq30_e510_d_n10: f64 = (locals.var_iqh_nqs_dn10 + 1e-9);
        let eq30_e510_q: f64 = eq30_e509_q;
        (eq30_e510, locals.var_iqh_nqs_dn0, locals.var_iqh_nqs_dn2, locals.var_iqh_nqs_dn4, locals.var_iqh_nqs_dn5, locals.var_iqh_nqs_dn6, locals.var_iqh_nqs_dn8, eq30_e510_d_n10, locals.var_iqh_nqs_dn11, locals.var_iqh_nqs_dn12, eq30_e510_q, 1e-9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[10]),
            None,
            nodes[10],
            multiplicity * (eq30_e512_q_d_n10),
        );
        let (eq32_e526, eq32_e526_d_n0, eq32_e526_d_n2, eq32_e526_d_n4, eq32_e526_d_n5, eq32_e526_d_n6, eq32_e526_d_n8, eq32_e526_d_n10, eq32_e526_d_n11, eq32_e526_d_n12, eq32_e526_q, eq32_e526_q_d_n8,) = {
    if (p.p24 != 0.0) {
        let eq32_e522: f64 = (1e-9 * (nv8 - 0.0));
        let eq32_e523_q: f64 = eq32_e522;
        let eq32_e524: f64 = (locals.var_iqi_nqs + eq32_e522);
        let eq32_e524_d_n8: f64 = (locals.var_iqi_nqs_dn8 + 1e-9);
        let eq32_e524_q: f64 = eq32_e523_q;
        (eq32_e524, locals.var_iqi_nqs_dn0, locals.var_iqi_nqs_dn2, locals.var_iqi_nqs_dn4, locals.var_iqi_nqs_dn5, locals.var_iqi_nqs_dn6, eq32_e524_d_n8, locals.var_iqi_nqs_dn10, locals.var_iqi_nqs_dn11, locals.var_iqi_nqs_dn12, eq32_e524_q, 1e-9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[8]),
            None,
            nodes[8],
            multiplicity * (eq32_e526_q_d_n8),
        );
        let (eq33_e535, eq33_e535_d_n0, eq33_e535_d_n2, eq33_e535_d_n4, eq33_e535_d_n5, eq33_e535_d_n6, eq33_e535_d_n8, eq33_e535_d_n9, eq33_e535_d_n10, eq33_e535_d_n11, eq33_e535_d_n12, eq33_e535_q, eq33_e535_q_d_n9,) = {
    if (p.p24 != 0.0) {
        let eq33_e531: f64 = (1e-9 * (nv9 - 0.0));
        let eq33_e532_q: f64 = eq33_e531;
        let eq33_e533: f64 = (locals.var_iqb_nqs + eq33_e531);
        let eq33_e533_d_n9: f64 = (locals.var_iqb_nqs_dn9 + 1e-9);
        let eq33_e533_q: f64 = eq33_e532_q;
        (eq33_e533, locals.var_iqb_nqs_dn0, locals.var_iqb_nqs_dn2, locals.var_iqb_nqs_dn4, locals.var_iqb_nqs_dn5, locals.var_iqb_nqs_dn6, locals.var_iqb_nqs_dn8, eq33_e533_d_n9, locals.var_iqb_nqs_dn10, locals.var_iqb_nqs_dn11, locals.var_iqb_nqs_dn12, eq33_e533_q, 1e-9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[9]),
            None,
            nodes[9],
            multiplicity * (eq33_e535_q_d_n9),
        );
    }
}

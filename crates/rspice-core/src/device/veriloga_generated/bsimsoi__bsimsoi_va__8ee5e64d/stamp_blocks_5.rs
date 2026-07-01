#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_80(
        locals: &mut StampLocals,
    ) {
        let (assign29790_e23507, assign29790_e23507_d_n3, assign29790_e23507_d_n4, assign29790_e23507_d_n5, assign29790_e23507_d_n6, assign29790_e23507_d_n7, assign29790_e23507_d_n8, assign29790_e23507_d_n9, assign29790_e23507_d_n10, assign29790_e23507_d_n11, assign29790_e23507_d_n12,) = {
    if (((locals.var_guard1698 != 0.0) && (locals.var_guard1699 == 0.0)) && (locals.var_guard1705 != 0.0)) {
        let assign29790_e23501: f64 = (locals.var_vgs_eff2 - locals.var_vfbeff2);
        let assign29790_e23503: f64 = (assign29790_e23501 - locals.var_vbseff);
        let assign29790_e23505: f64 = (assign29790_e23503 - locals.var_vgsteff2);
        (assign29790_e23505, (((-locals.var_vfbeff2_dn3) - locals.var_vbseff_dn3) - locals.var_vgsteff2_dn3), (((-locals.var_vfbeff2_dn4) - locals.var_vbseff_dn4) - locals.var_vgsteff2_dn4), (((-locals.var_vfbeff2_dn5) - locals.var_vbseff_dn5) - locals.var_vgsteff2_dn5), (((-locals.var_vfbeff2_dn6) - locals.var_vbseff_dn6) - locals.var_vgsteff2_dn6), (((locals.var_vgs_eff2_dn7 - locals.var_vfbeff2_dn7) - locals.var_vbseff_dn7) - locals.var_vgsteff2_dn7), (((locals.var_vgs_eff2_dn8 - locals.var_vfbeff2_dn8) - locals.var_vbseff_dn8) - locals.var_vgsteff2_dn8), (((locals.var_vgs_eff2_dn9 - locals.var_vfbeff2_dn9) - locals.var_vbseff_dn9) - locals.var_vgsteff2_dn9), (((-locals.var_vfbeff2_dn10) - locals.var_vbseff_dn10) - locals.var_vgsteff2_dn10), (((-locals.var_vfbeff2_dn11) - locals.var_vbseff_dn11) - locals.var_vgsteff2_dn11), (((-locals.var_vfbeff2_dn12) - locals.var_vbseff_dn12) - locals.var_vgsteff2_dn12),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign29790_e23507;
        locals.var_t3__blk1147_dn3 = assign29790_e23507_d_n3;
        locals.var_t3__blk1147_dn4 = assign29790_e23507_d_n4;
        locals.var_t3__blk1147_dn5 = assign29790_e23507_d_n5;
        locals.var_t3__blk1147_dn6 = assign29790_e23507_d_n6;
        locals.var_t3__blk1147_dn7 = assign29790_e23507_d_n7;
        locals.var_t3__blk1147_dn8 = assign29790_e23507_d_n8;
        locals.var_t3__blk1147_dn9 = assign29790_e23507_d_n9;
        locals.var_t3__blk1147_dn10 = assign29790_e23507_d_n10;
        locals.var_t3__blk1147_dn11 = assign29790_e23507_d_n11;
        locals.var_t3__blk1147_dn12 = assign29790_e23507_d_n12;

        let assign29800_e23510: f64 = if locals.var_t3__blk1147 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1706 = assign29800_e23510;

        let (assign29810_e23525, assign29810_e23525_d_n3, assign29810_e23525_d_n4, assign29810_e23525_d_n5, assign29810_e23525_d_n6, assign29810_e23525_d_n7, assign29810_e23525_d_n8, assign29810_e23525_d_n9, assign29810_e23525_d_n10, assign29810_e23525_d_n11, assign29810_e23525_d_n12,) = {
    if ((((locals.var_guard1698 != 0.0) && (locals.var_guard1699 == 0.0)) && (locals.var_guard1705 != 0.0)) && (locals.var_guard1706 != 0.0)) {
        let assign29810_e23522: f64 = (locals.var_t3__blk1147 / locals.var_pparam_b4soik1ox);
        let assign29810_e23523: f64 = (locals.var_t0__blk1144 + assign29810_e23522);
        (assign29810_e23523, (locals.var_t0__blk1144_dn3 + (((locals.var_t3__blk1147_dn3 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn3)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn4 + (((locals.var_t3__blk1147_dn4 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn4)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn5 + (((locals.var_t3__blk1147_dn5 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn5)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn6 + (((locals.var_t3__blk1147_dn6 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn6)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn7 + (((locals.var_t3__blk1147_dn7 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn7)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn8 + (((locals.var_t3__blk1147_dn8 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn8)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn9 + (((locals.var_t3__blk1147_dn9 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn9)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn10 + (((locals.var_t3__blk1147_dn10 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn10)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn11 + (((locals.var_t3__blk1147_dn11 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn11)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn12 + (((locals.var_t3__blk1147_dn12 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn12)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign29810_e23525;
        locals.var_t1__blk1145_dn3 = assign29810_e23525_d_n3;
        locals.var_t1__blk1145_dn4 = assign29810_e23525_d_n4;
        locals.var_t1__blk1145_dn5 = assign29810_e23525_d_n5;
        locals.var_t1__blk1145_dn6 = assign29810_e23525_d_n6;
        locals.var_t1__blk1145_dn7 = assign29810_e23525_d_n7;
        locals.var_t1__blk1145_dn8 = assign29810_e23525_d_n8;
        locals.var_t1__blk1145_dn9 = assign29810_e23525_d_n9;
        locals.var_t1__blk1145_dn10 = assign29810_e23525_d_n10;
        locals.var_t1__blk1145_dn11 = assign29810_e23525_d_n11;
        locals.var_t1__blk1145_dn12 = assign29810_e23525_d_n12;

        let (assign29820_e23542, assign29820_e23542_d_n3, assign29820_e23542_d_n4, assign29820_e23542_d_n5, assign29820_e23542_d_n6, assign29820_e23542_d_n7, assign29820_e23542_d_n8, assign29820_e23542_d_n9, assign29820_e23542_d_n10, assign29820_e23542_d_n11, assign29820_e23542_d_n12,) = {
    if ((((locals.var_guard1698 != 0.0) && (locals.var_guard1699 == 0.0)) && (locals.var_guard1705 != 0.0)) && (locals.var_guard1706 == 0.0)) {
        let assign29820_e23537: f64 = (locals.var_t0__blk1144 * locals.var_t0__blk1144);
        let assign29820_e23539: f64 = (assign29820_e23537 + locals.var_t3__blk1147);
        let assign29820_e23540: f64 = (assign29820_e23539).sqrt();
        (assign29820_e23540, ((((locals.var_t0__blk1144_dn3 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn3)) + locals.var_t3__blk1147_dn3) / (2.0 * assign29820_e23540)), ((((locals.var_t0__blk1144_dn4 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn4)) + locals.var_t3__blk1147_dn4) / (2.0 * assign29820_e23540)), ((((locals.var_t0__blk1144_dn5 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn5)) + locals.var_t3__blk1147_dn5) / (2.0 * assign29820_e23540)), ((((locals.var_t0__blk1144_dn6 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn6)) + locals.var_t3__blk1147_dn6) / (2.0 * assign29820_e23540)), ((((locals.var_t0__blk1144_dn7 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn7)) + locals.var_t3__blk1147_dn7) / (2.0 * assign29820_e23540)), ((((locals.var_t0__blk1144_dn8 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn8)) + locals.var_t3__blk1147_dn8) / (2.0 * assign29820_e23540)), ((((locals.var_t0__blk1144_dn9 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn9)) + locals.var_t3__blk1147_dn9) / (2.0 * assign29820_e23540)), ((((locals.var_t0__blk1144_dn10 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn10)) + locals.var_t3__blk1147_dn10) / (2.0 * assign29820_e23540)), ((((locals.var_t0__blk1144_dn11 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn11)) + locals.var_t3__blk1147_dn11) / (2.0 * assign29820_e23540)), ((((locals.var_t0__blk1144_dn12 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn12)) + locals.var_t3__blk1147_dn12) / (2.0 * assign29820_e23540)),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign29820_e23542;
        locals.var_t1__blk1145_dn3 = assign29820_e23542_d_n3;
        locals.var_t1__blk1145_dn4 = assign29820_e23542_d_n4;
        locals.var_t1__blk1145_dn5 = assign29820_e23542_d_n5;
        locals.var_t1__blk1145_dn6 = assign29820_e23542_d_n6;
        locals.var_t1__blk1145_dn7 = assign29820_e23542_d_n7;
        locals.var_t1__blk1145_dn8 = assign29820_e23542_d_n8;
        locals.var_t1__blk1145_dn9 = assign29820_e23542_d_n9;
        locals.var_t1__blk1145_dn10 = assign29820_e23542_d_n10;
        locals.var_t1__blk1145_dn11 = assign29820_e23542_d_n11;
        locals.var_t1__blk1145_dn12 = assign29820_e23542_d_n12;

        let (assign29830_e23559, assign29830_e23559_d_n3, assign29830_e23559_d_n4, assign29830_e23559_d_n5, assign29830_e23559_d_n6, assign29830_e23559_d_n7, assign29830_e23559_d_n8, assign29830_e23559_d_n9, assign29830_e23559_d_n10, assign29830_e23559_d_n11, assign29830_e23559_d_n12,) = {
    if (((locals.var_guard1698 != 0.0) && (locals.var_guard1699 == 0.0)) && (locals.var_guard1705 != 0.0)) {
        let assign29830_e23552: f64 = (locals.var_coxwlb2 * locals.var_pparam_b4soik1ox);
        let assign29830_e23555: f64 = (locals.var_t1__blk1145 - locals.var_t0__blk1144);
        let assign29830_e23556: f64 = (assign29830_e23552 * assign29830_e23555);
        let assign29830_e23557: f64 = (locals.var_qsub0 + assign29830_e23556);
        (assign29830_e23557, (locals.var_qsub0_dn3 + ((((locals.var_coxwlb2_dn3 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlb2 * locals.var_pparam_b4soik1ox_dn3)) * assign29830_e23555) + (assign29830_e23552 * (locals.var_t1__blk1145_dn3 - locals.var_t0__blk1144_dn3)))), (locals.var_qsub0_dn4 + ((((locals.var_coxwlb2_dn4 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlb2 * locals.var_pparam_b4soik1ox_dn4)) * assign29830_e23555) + (assign29830_e23552 * (locals.var_t1__blk1145_dn4 - locals.var_t0__blk1144_dn4)))), (locals.var_qsub0_dn5 + ((((locals.var_coxwlb2_dn5 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlb2 * locals.var_pparam_b4soik1ox_dn5)) * assign29830_e23555) + (assign29830_e23552 * (locals.var_t1__blk1145_dn5 - locals.var_t0__blk1144_dn5)))), (locals.var_qsub0_dn6 + ((((locals.var_coxwlb2_dn6 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlb2 * locals.var_pparam_b4soik1ox_dn6)) * assign29830_e23555) + (assign29830_e23552 * (locals.var_t1__blk1145_dn6 - locals.var_t0__blk1144_dn6)))), (locals.var_qsub0_dn7 + ((((locals.var_coxwlb2_dn7 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlb2 * locals.var_pparam_b4soik1ox_dn7)) * assign29830_e23555) + (assign29830_e23552 * (locals.var_t1__blk1145_dn7 - locals.var_t0__blk1144_dn7)))), (locals.var_qsub0_dn8 + ((((locals.var_coxwlb2_dn8 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlb2 * locals.var_pparam_b4soik1ox_dn8)) * assign29830_e23555) + (assign29830_e23552 * (locals.var_t1__blk1145_dn8 - locals.var_t0__blk1144_dn8)))), (locals.var_qsub0_dn9 + ((((locals.var_coxwlb2_dn9 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlb2 * locals.var_pparam_b4soik1ox_dn9)) * assign29830_e23555) + (assign29830_e23552 * (locals.var_t1__blk1145_dn9 - locals.var_t0__blk1144_dn9)))), (locals.var_qsub0_dn10 + ((((locals.var_coxwlb2_dn10 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlb2 * locals.var_pparam_b4soik1ox_dn10)) * assign29830_e23555) + (assign29830_e23552 * (locals.var_t1__blk1145_dn10 - locals.var_t0__blk1144_dn10)))), (locals.var_qsub0_dn11 + ((((locals.var_coxwlb2_dn11 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlb2 * locals.var_pparam_b4soik1ox_dn11)) * assign29830_e23555) + (assign29830_e23552 * (locals.var_t1__blk1145_dn11 - locals.var_t0__blk1144_dn11)))), (locals.var_qsub0_dn12 + ((((locals.var_coxwlb2_dn12 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlb2 * locals.var_pparam_b4soik1ox_dn12)) * assign29830_e23555) + (assign29830_e23552 * (locals.var_t1__blk1145_dn12 - locals.var_t0__blk1144_dn12)))),)
    } else {
        (locals.var_qsub0, locals.var_qsub0_dn3, locals.var_qsub0_dn4, locals.var_qsub0_dn5, locals.var_qsub0_dn6, locals.var_qsub0_dn7, locals.var_qsub0_dn8, locals.var_qsub0_dn9, locals.var_qsub0_dn10, locals.var_qsub0_dn11, locals.var_qsub0_dn12,)
    }
};
        locals.var_qsub0 = assign29830_e23559;
        locals.var_qsub0_dn3 = assign29830_e23559_d_n3;
        locals.var_qsub0_dn4 = assign29830_e23559_d_n4;
        locals.var_qsub0_dn5 = assign29830_e23559_d_n5;
        locals.var_qsub0_dn6 = assign29830_e23559_d_n6;
        locals.var_qsub0_dn7 = assign29830_e23559_d_n7;
        locals.var_qsub0_dn8 = assign29830_e23559_d_n8;
        locals.var_qsub0_dn9 = assign29830_e23559_d_n9;
        locals.var_qsub0_dn10 = assign29830_e23559_d_n10;
        locals.var_qsub0_dn11 = assign29830_e23559_d_n11;
        locals.var_qsub0_dn12 = assign29830_e23559_d_n12;

        let (assign29840_e23565, assign29840_e23565_d_n3, assign29840_e23565_d_n4, assign29840_e23565_d_n5, assign29840_e23565_d_n6, assign29840_e23565_d_n7, assign29840_e23565_d_n8, assign29840_e23565_d_n9, assign29840_e23565_d_n10, assign29840_e23565_d_n11, assign29840_e23565_d_n12,) = {
    if (locals.var_guard1698 != 0.0) {
        let assign29840_e23563: f64 = (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor);
        (assign29840_e23563, ((locals.var_abulk0_dn3 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn3)), ((locals.var_abulk0_dn4 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn4)), ((locals.var_abulk0_dn5 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn5)), ((locals.var_abulk0_dn6 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn6)), ((locals.var_abulk0_dn7 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn7)), ((locals.var_abulk0_dn8 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn8)), ((locals.var_abulk0_dn9 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn9)), ((locals.var_abulk0_dn10 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn10)), ((locals.var_abulk0_dn11 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn11)), ((locals.var_abulk0_dn12 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn12)),)
    } else {
        (locals.var_abulkcv, locals.var_abulkcv_dn3, locals.var_abulkcv_dn4, locals.var_abulkcv_dn5, locals.var_abulkcv_dn6, locals.var_abulkcv_dn7, locals.var_abulkcv_dn8, locals.var_abulkcv_dn9, locals.var_abulkcv_dn10, locals.var_abulkcv_dn11, locals.var_abulkcv_dn12,)
    }
};
        locals.var_abulkcv = assign29840_e23565;
        locals.var_abulkcv_dn3 = assign29840_e23565_d_n3;
        locals.var_abulkcv_dn4 = assign29840_e23565_d_n4;
        locals.var_abulkcv_dn5 = assign29840_e23565_d_n5;
        locals.var_abulkcv_dn6 = assign29840_e23565_d_n6;
        locals.var_abulkcv_dn7 = assign29840_e23565_d_n7;
        locals.var_abulkcv_dn8 = assign29840_e23565_d_n8;
        locals.var_abulkcv_dn9 = assign29840_e23565_d_n9;
        locals.var_abulkcv_dn10 = assign29840_e23565_d_n10;
        locals.var_abulkcv_dn11 = assign29840_e23565_d_n11;
        locals.var_abulkcv_dn12 = assign29840_e23565_d_n12;

        let (assign29850_e23571, assign29850_e23571_d_n3, assign29850_e23571_d_n4, assign29850_e23571_d_n5, assign29850_e23571_d_n6, assign29850_e23571_d_n7, assign29850_e23571_d_n8, assign29850_e23571_d_n9, assign29850_e23571_d_n10, assign29850_e23571_d_n11, assign29850_e23571_d_n12,) = {
    if (locals.var_guard1698 != 0.0) {
        let assign29850_e23569: f64 = (locals.var_vgsteff__blk1175 / locals.var_abulkcv);
        (assign29850_e23569, (((locals.var_vgsteff__blk1175_dn3 * locals.var_abulkcv) - (locals.var_vgsteff__blk1175 * locals.var_abulkcv_dn3)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff__blk1175_dn4 * locals.var_abulkcv) - (locals.var_vgsteff__blk1175 * locals.var_abulkcv_dn4)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff__blk1175_dn5 * locals.var_abulkcv) - (locals.var_vgsteff__blk1175 * locals.var_abulkcv_dn5)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff__blk1175_dn6 * locals.var_abulkcv) - (locals.var_vgsteff__blk1175 * locals.var_abulkcv_dn6)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff__blk1175_dn7 * locals.var_abulkcv) - (locals.var_vgsteff__blk1175 * locals.var_abulkcv_dn7)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff__blk1175_dn8 * locals.var_abulkcv) - (locals.var_vgsteff__blk1175 * locals.var_abulkcv_dn8)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff__blk1175_dn9 * locals.var_abulkcv) - (locals.var_vgsteff__blk1175 * locals.var_abulkcv_dn9)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff__blk1175_dn10 * locals.var_abulkcv) - (locals.var_vgsteff__blk1175 * locals.var_abulkcv_dn10)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff__blk1175_dn11 * locals.var_abulkcv) - (locals.var_vgsteff__blk1175 * locals.var_abulkcv_dn11)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff__blk1175_dn12 * locals.var_abulkcv) - (locals.var_vgsteff__blk1175 * locals.var_abulkcv_dn12)) / (locals.var_abulkcv * locals.var_abulkcv)),)
    } else {
        (locals.var_vdsatcv, locals.var_vdsatcv_dn3, locals.var_vdsatcv_dn4, locals.var_vdsatcv_dn5, locals.var_vdsatcv_dn6, locals.var_vdsatcv_dn7, locals.var_vdsatcv_dn8, locals.var_vdsatcv_dn9, locals.var_vdsatcv_dn10, locals.var_vdsatcv_dn11, locals.var_vdsatcv_dn12,)
    }
};
        locals.var_vdsatcv = assign29850_e23571;
        locals.var_vdsatcv_dn3 = assign29850_e23571_d_n3;
        locals.var_vdsatcv_dn4 = assign29850_e23571_d_n4;
        locals.var_vdsatcv_dn5 = assign29850_e23571_d_n5;
        locals.var_vdsatcv_dn6 = assign29850_e23571_d_n6;
        locals.var_vdsatcv_dn7 = assign29850_e23571_d_n7;
        locals.var_vdsatcv_dn8 = assign29850_e23571_d_n8;
        locals.var_vdsatcv_dn9 = assign29850_e23571_d_n9;
        locals.var_vdsatcv_dn10 = assign29850_e23571_d_n10;
        locals.var_vdsatcv_dn11 = assign29850_e23571_d_n11;
        locals.var_vdsatcv_dn12 = assign29850_e23571_d_n12;

        let (assign29860_e23579, assign29860_e23579_d_n3, assign29860_e23579_d_n4, assign29860_e23579_d_n5, assign29860_e23579_d_n6, assign29860_e23579_d_n7, assign29860_e23579_d_n8, assign29860_e23579_d_n9, assign29860_e23579_d_n10, assign29860_e23579_d_n11, assign29860_e23579_d_n12,) = {
    if (locals.var_guard1698 != 0.0) {
        let assign29860_e23575: f64 = (locals.var_vdsatcv - locals.var_vds_1);
        let assign29860_e23577: f64 = (assign29860_e23575 - 0.02);
        (assign29860_e23577, locals.var_vdsatcv_dn3, locals.var_vdsatcv_dn4, locals.var_vdsatcv_dn5, locals.var_vdsatcv_dn6, (locals.var_vdsatcv_dn7 - locals.var_vds_1_dn7), (locals.var_vdsatcv_dn8 - locals.var_vds_1_dn8), locals.var_vdsatcv_dn9, locals.var_vdsatcv_dn10, locals.var_vdsatcv_dn11, locals.var_vdsatcv_dn12,)
    } else {
        (locals.var_v4, locals.var_v4_dn3, locals.var_v4_dn4, locals.var_v4_dn5, locals.var_v4_dn6, locals.var_v4_dn7, locals.var_v4_dn8, locals.var_v4_dn9, locals.var_v4_dn10, locals.var_v4_dn11, locals.var_v4_dn12,)
    }
};
        locals.var_v4 = assign29860_e23579;
        locals.var_v4_dn3 = assign29860_e23579_d_n3;
        locals.var_v4_dn4 = assign29860_e23579_d_n4;
        locals.var_v4_dn5 = assign29860_e23579_d_n5;
        locals.var_v4_dn6 = assign29860_e23579_d_n6;
        locals.var_v4_dn7 = assign29860_e23579_d_n7;
        locals.var_v4_dn8 = assign29860_e23579_d_n8;
        locals.var_v4_dn9 = assign29860_e23579_d_n9;
        locals.var_v4_dn10 = assign29860_e23579_d_n10;
        locals.var_v4_dn11 = assign29860_e23579_d_n11;
        locals.var_v4_dn12 = assign29860_e23579_d_n12;

        let (assign29870_e23592, assign29870_e23592_d_n3, assign29870_e23592_d_n4, assign29870_e23592_d_n5, assign29870_e23592_d_n6, assign29870_e23592_d_n7, assign29870_e23592_d_n8, assign29870_e23592_d_n9, assign29870_e23592_d_n10, assign29870_e23592_d_n11, assign29870_e23592_d_n12,) = {
    if (locals.var_guard1698 != 0.0) {
        let assign29870_e23583: f64 = (locals.var_v4 * locals.var_v4);
        let assign29870_e23586: f64 = (4.0 * 0.02);
        let assign29870_e23588: f64 = (assign29870_e23586 * locals.var_vdsatcv);
        let assign29870_e23589: f64 = (assign29870_e23583 + assign29870_e23588);
        let assign29870_e23590: f64 = (assign29870_e23589).sqrt();
        (assign29870_e23590, ((((locals.var_v4_dn3 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn3)) + (assign29870_e23586 * locals.var_vdsatcv_dn3)) / (2.0 * assign29870_e23590)), ((((locals.var_v4_dn4 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn4)) + (assign29870_e23586 * locals.var_vdsatcv_dn4)) / (2.0 * assign29870_e23590)), ((((locals.var_v4_dn5 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn5)) + (assign29870_e23586 * locals.var_vdsatcv_dn5)) / (2.0 * assign29870_e23590)), ((((locals.var_v4_dn6 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn6)) + (assign29870_e23586 * locals.var_vdsatcv_dn6)) / (2.0 * assign29870_e23590)), ((((locals.var_v4_dn7 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn7)) + (assign29870_e23586 * locals.var_vdsatcv_dn7)) / (2.0 * assign29870_e23590)), ((((locals.var_v4_dn8 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn8)) + (assign29870_e23586 * locals.var_vdsatcv_dn8)) / (2.0 * assign29870_e23590)), ((((locals.var_v4_dn9 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn9)) + (assign29870_e23586 * locals.var_vdsatcv_dn9)) / (2.0 * assign29870_e23590)), ((((locals.var_v4_dn10 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn10)) + (assign29870_e23586 * locals.var_vdsatcv_dn10)) / (2.0 * assign29870_e23590)), ((((locals.var_v4_dn11 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn11)) + (assign29870_e23586 * locals.var_vdsatcv_dn11)) / (2.0 * assign29870_e23590)), ((((locals.var_v4_dn12 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn12)) + (assign29870_e23586 * locals.var_vdsatcv_dn12)) / (2.0 * assign29870_e23590)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign29870_e23592;
        locals.var_t0__blk1144_dn3 = assign29870_e23592_d_n3;
        locals.var_t0__blk1144_dn4 = assign29870_e23592_d_n4;
        locals.var_t0__blk1144_dn5 = assign29870_e23592_d_n5;
        locals.var_t0__blk1144_dn6 = assign29870_e23592_d_n6;
        locals.var_t0__blk1144_dn7 = assign29870_e23592_d_n7;
        locals.var_t0__blk1144_dn8 = assign29870_e23592_d_n8;
        locals.var_t0__blk1144_dn9 = assign29870_e23592_d_n9;
        locals.var_t0__blk1144_dn10 = assign29870_e23592_d_n10;
        locals.var_t0__blk1144_dn11 = assign29870_e23592_d_n11;
        locals.var_t0__blk1144_dn12 = assign29870_e23592_d_n12;

        let (assign29880_e23602, assign29880_e23602_d_n3, assign29880_e23602_d_n4, assign29880_e23602_d_n5, assign29880_e23602_d_n6, assign29880_e23602_d_n7, assign29880_e23602_d_n8, assign29880_e23602_d_n9, assign29880_e23602_d_n10, assign29880_e23602_d_n11, assign29880_e23602_d_n12,) = {
    if (locals.var_guard1698 != 0.0) {
        let assign29880_e23598: f64 = (locals.var_v4 + locals.var_t0__blk1144);
        let assign29880_e23599: f64 = (0.5 * assign29880_e23598);
        let assign29880_e23600: f64 = (locals.var_vdsatcv - assign29880_e23599);
        (assign29880_e23600, (locals.var_vdsatcv_dn3 - (0.5 * (locals.var_v4_dn3 + locals.var_t0__blk1144_dn3))), (locals.var_vdsatcv_dn4 - (0.5 * (locals.var_v4_dn4 + locals.var_t0__blk1144_dn4))), (locals.var_vdsatcv_dn5 - (0.5 * (locals.var_v4_dn5 + locals.var_t0__blk1144_dn5))), (locals.var_vdsatcv_dn6 - (0.5 * (locals.var_v4_dn6 + locals.var_t0__blk1144_dn6))), (locals.var_vdsatcv_dn7 - (0.5 * (locals.var_v4_dn7 + locals.var_t0__blk1144_dn7))), (locals.var_vdsatcv_dn8 - (0.5 * (locals.var_v4_dn8 + locals.var_t0__blk1144_dn8))), (locals.var_vdsatcv_dn9 - (0.5 * (locals.var_v4_dn9 + locals.var_t0__blk1144_dn9))), (locals.var_vdsatcv_dn10 - (0.5 * (locals.var_v4_dn10 + locals.var_t0__blk1144_dn10))), (locals.var_vdsatcv_dn11 - (0.5 * (locals.var_v4_dn11 + locals.var_t0__blk1144_dn11))), (locals.var_vdsatcv_dn12 - (0.5 * (locals.var_v4_dn12 + locals.var_t0__blk1144_dn12))),)
    } else {
        (locals.var_vdseffcv, locals.var_vdseffcv_dn3, locals.var_vdseffcv_dn4, locals.var_vdseffcv_dn5, locals.var_vdseffcv_dn6, locals.var_vdseffcv_dn7, locals.var_vdseffcv_dn8, locals.var_vdseffcv_dn9, locals.var_vdseffcv_dn10, locals.var_vdseffcv_dn11, locals.var_vdseffcv_dn12,)
    }
};
        locals.var_vdseffcv = assign29880_e23602;
        locals.var_vdseffcv_dn3 = assign29880_e23602_d_n3;
        locals.var_vdseffcv_dn4 = assign29880_e23602_d_n4;
        locals.var_vdseffcv_dn5 = assign29880_e23602_d_n5;
        locals.var_vdseffcv_dn6 = assign29880_e23602_d_n6;
        locals.var_vdseffcv_dn7 = assign29880_e23602_d_n7;
        locals.var_vdseffcv_dn8 = assign29880_e23602_d_n8;
        locals.var_vdseffcv_dn9 = assign29880_e23602_d_n9;
        locals.var_vdseffcv_dn10 = assign29880_e23602_d_n10;
        locals.var_vdseffcv_dn11 = assign29880_e23602_d_n11;
        locals.var_vdseffcv_dn12 = assign29880_e23602_d_n12;

        let assign29890_e23605: f64 = if locals.var_b4soiagbcp2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1707 = assign29890_e23605;

        let (assign29900_e23613, assign29900_e23613_d_n3, assign29900_e23613_d_n4, assign29900_e23613_d_n5, assign29900_e23613_d_n6, assign29900_e23613_d_n7, assign29900_e23613_d_n8, assign29900_e23613_d_n9, assign29900_e23613_d_n10, assign29900_e23613_d_n11, assign29900_e23613_d_n12,) = {
    if ((locals.var_guard1698 != 0.0) && (locals.var_guard1707 != 0.0)) {
        let assign29900_e23611: f64 = (locals.var_vgsteff2 / locals.var_abulkcv);
        (assign29900_e23611, (((locals.var_vgsteff2_dn3 * locals.var_abulkcv) - (locals.var_vgsteff2 * locals.var_abulkcv_dn3)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff2_dn4 * locals.var_abulkcv) - (locals.var_vgsteff2 * locals.var_abulkcv_dn4)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff2_dn5 * locals.var_abulkcv) - (locals.var_vgsteff2 * locals.var_abulkcv_dn5)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff2_dn6 * locals.var_abulkcv) - (locals.var_vgsteff2 * locals.var_abulkcv_dn6)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff2_dn7 * locals.var_abulkcv) - (locals.var_vgsteff2 * locals.var_abulkcv_dn7)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff2_dn8 * locals.var_abulkcv) - (locals.var_vgsteff2 * locals.var_abulkcv_dn8)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff2_dn9 * locals.var_abulkcv) - (locals.var_vgsteff2 * locals.var_abulkcv_dn9)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff2_dn10 * locals.var_abulkcv) - (locals.var_vgsteff2 * locals.var_abulkcv_dn10)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff2_dn11 * locals.var_abulkcv) - (locals.var_vgsteff2 * locals.var_abulkcv_dn11)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff2_dn12 * locals.var_abulkcv) - (locals.var_vgsteff2 * locals.var_abulkcv_dn12)) / (locals.var_abulkcv * locals.var_abulkcv)),)
    } else {
        (locals.var_vdsatcv2, locals.var_vdsatcv2_dn3, locals.var_vdsatcv2_dn4, locals.var_vdsatcv2_dn5, locals.var_vdsatcv2_dn6, locals.var_vdsatcv2_dn7, locals.var_vdsatcv2_dn8, locals.var_vdsatcv2_dn9, locals.var_vdsatcv2_dn10, locals.var_vdsatcv2_dn11, locals.var_vdsatcv2_dn12,)
    }
};
        locals.var_vdsatcv2 = assign29900_e23613;
        locals.var_vdsatcv2_dn3 = assign29900_e23613_d_n3;
        locals.var_vdsatcv2_dn4 = assign29900_e23613_d_n4;
        locals.var_vdsatcv2_dn5 = assign29900_e23613_d_n5;
        locals.var_vdsatcv2_dn6 = assign29900_e23613_d_n6;
        locals.var_vdsatcv2_dn7 = assign29900_e23613_d_n7;
        locals.var_vdsatcv2_dn8 = assign29900_e23613_d_n8;
        locals.var_vdsatcv2_dn9 = assign29900_e23613_d_n9;
        locals.var_vdsatcv2_dn10 = assign29900_e23613_d_n10;
        locals.var_vdsatcv2_dn11 = assign29900_e23613_d_n11;
        locals.var_vdsatcv2_dn12 = assign29900_e23613_d_n12;

        let (assign29910_e23623, assign29910_e23623_d_n3, assign29910_e23623_d_n4, assign29910_e23623_d_n5, assign29910_e23623_d_n6, assign29910_e23623_d_n7, assign29910_e23623_d_n8, assign29910_e23623_d_n9, assign29910_e23623_d_n10, assign29910_e23623_d_n11, assign29910_e23623_d_n12,) = {
    if ((locals.var_guard1698 != 0.0) && (locals.var_guard1707 != 0.0)) {
        let assign29910_e23619: f64 = (locals.var_vdsatcv2 - locals.var_vds_1);
        let assign29910_e23621: f64 = (assign29910_e23619 - 0.02);
        (assign29910_e23621, locals.var_vdsatcv2_dn3, locals.var_vdsatcv2_dn4, locals.var_vdsatcv2_dn5, locals.var_vdsatcv2_dn6, (locals.var_vdsatcv2_dn7 - locals.var_vds_1_dn7), (locals.var_vdsatcv2_dn8 - locals.var_vds_1_dn8), locals.var_vdsatcv2_dn9, locals.var_vdsatcv2_dn10, locals.var_vdsatcv2_dn11, locals.var_vdsatcv2_dn12,)
    } else {
        (locals.var_v4, locals.var_v4_dn3, locals.var_v4_dn4, locals.var_v4_dn5, locals.var_v4_dn6, locals.var_v4_dn7, locals.var_v4_dn8, locals.var_v4_dn9, locals.var_v4_dn10, locals.var_v4_dn11, locals.var_v4_dn12,)
    }
};
        locals.var_v4 = assign29910_e23623;
        locals.var_v4_dn3 = assign29910_e23623_d_n3;
        locals.var_v4_dn4 = assign29910_e23623_d_n4;
        locals.var_v4_dn5 = assign29910_e23623_d_n5;
        locals.var_v4_dn6 = assign29910_e23623_d_n6;
        locals.var_v4_dn7 = assign29910_e23623_d_n7;
        locals.var_v4_dn8 = assign29910_e23623_d_n8;
        locals.var_v4_dn9 = assign29910_e23623_d_n9;
        locals.var_v4_dn10 = assign29910_e23623_d_n10;
        locals.var_v4_dn11 = assign29910_e23623_d_n11;
        locals.var_v4_dn12 = assign29910_e23623_d_n12;

        let (assign29920_e23638, assign29920_e23638_d_n3, assign29920_e23638_d_n4, assign29920_e23638_d_n5, assign29920_e23638_d_n6, assign29920_e23638_d_n7, assign29920_e23638_d_n8, assign29920_e23638_d_n9, assign29920_e23638_d_n10, assign29920_e23638_d_n11, assign29920_e23638_d_n12,) = {
    if ((locals.var_guard1698 != 0.0) && (locals.var_guard1707 != 0.0)) {
        let assign29920_e23629: f64 = (locals.var_v4 * locals.var_v4);
        let assign29920_e23632: f64 = (4.0 * 0.02);
        let assign29920_e23634: f64 = (assign29920_e23632 * locals.var_vdsatcv2);
        let assign29920_e23635: f64 = (assign29920_e23629 + assign29920_e23634);
        let assign29920_e23636: f64 = (assign29920_e23635).sqrt();
        (assign29920_e23636, ((((locals.var_v4_dn3 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn3)) + (assign29920_e23632 * locals.var_vdsatcv2_dn3)) / (2.0 * assign29920_e23636)), ((((locals.var_v4_dn4 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn4)) + (assign29920_e23632 * locals.var_vdsatcv2_dn4)) / (2.0 * assign29920_e23636)), ((((locals.var_v4_dn5 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn5)) + (assign29920_e23632 * locals.var_vdsatcv2_dn5)) / (2.0 * assign29920_e23636)), ((((locals.var_v4_dn6 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn6)) + (assign29920_e23632 * locals.var_vdsatcv2_dn6)) / (2.0 * assign29920_e23636)), ((((locals.var_v4_dn7 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn7)) + (assign29920_e23632 * locals.var_vdsatcv2_dn7)) / (2.0 * assign29920_e23636)), ((((locals.var_v4_dn8 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn8)) + (assign29920_e23632 * locals.var_vdsatcv2_dn8)) / (2.0 * assign29920_e23636)), ((((locals.var_v4_dn9 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn9)) + (assign29920_e23632 * locals.var_vdsatcv2_dn9)) / (2.0 * assign29920_e23636)), ((((locals.var_v4_dn10 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn10)) + (assign29920_e23632 * locals.var_vdsatcv2_dn10)) / (2.0 * assign29920_e23636)), ((((locals.var_v4_dn11 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn11)) + (assign29920_e23632 * locals.var_vdsatcv2_dn11)) / (2.0 * assign29920_e23636)), ((((locals.var_v4_dn12 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn12)) + (assign29920_e23632 * locals.var_vdsatcv2_dn12)) / (2.0 * assign29920_e23636)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign29920_e23638;
        locals.var_t0__blk1144_dn3 = assign29920_e23638_d_n3;
        locals.var_t0__blk1144_dn4 = assign29920_e23638_d_n4;
        locals.var_t0__blk1144_dn5 = assign29920_e23638_d_n5;
        locals.var_t0__blk1144_dn6 = assign29920_e23638_d_n6;
        locals.var_t0__blk1144_dn7 = assign29920_e23638_d_n7;
        locals.var_t0__blk1144_dn8 = assign29920_e23638_d_n8;
        locals.var_t0__blk1144_dn9 = assign29920_e23638_d_n9;
        locals.var_t0__blk1144_dn10 = assign29920_e23638_d_n10;
        locals.var_t0__blk1144_dn11 = assign29920_e23638_d_n11;
        locals.var_t0__blk1144_dn12 = assign29920_e23638_d_n12;

        let (assign29930_e23650, assign29930_e23650_d_n3, assign29930_e23650_d_n4, assign29930_e23650_d_n5, assign29930_e23650_d_n6, assign29930_e23650_d_n7, assign29930_e23650_d_n8, assign29930_e23650_d_n9, assign29930_e23650_d_n10, assign29930_e23650_d_n11, assign29930_e23650_d_n12,) = {
    if ((locals.var_guard1698 != 0.0) && (locals.var_guard1707 != 0.0)) {
        let assign29930_e23646: f64 = (locals.var_v4 + locals.var_t0__blk1144);
        let assign29930_e23647: f64 = (0.5 * assign29930_e23646);
        let assign29930_e23648: f64 = (locals.var_vdsatcv2 - assign29930_e23647);
        (assign29930_e23648, (locals.var_vdsatcv2_dn3 - (0.5 * (locals.var_v4_dn3 + locals.var_t0__blk1144_dn3))), (locals.var_vdsatcv2_dn4 - (0.5 * (locals.var_v4_dn4 + locals.var_t0__blk1144_dn4))), (locals.var_vdsatcv2_dn5 - (0.5 * (locals.var_v4_dn5 + locals.var_t0__blk1144_dn5))), (locals.var_vdsatcv2_dn6 - (0.5 * (locals.var_v4_dn6 + locals.var_t0__blk1144_dn6))), (locals.var_vdsatcv2_dn7 - (0.5 * (locals.var_v4_dn7 + locals.var_t0__blk1144_dn7))), (locals.var_vdsatcv2_dn8 - (0.5 * (locals.var_v4_dn8 + locals.var_t0__blk1144_dn8))), (locals.var_vdsatcv2_dn9 - (0.5 * (locals.var_v4_dn9 + locals.var_t0__blk1144_dn9))), (locals.var_vdsatcv2_dn10 - (0.5 * (locals.var_v4_dn10 + locals.var_t0__blk1144_dn10))), (locals.var_vdsatcv2_dn11 - (0.5 * (locals.var_v4_dn11 + locals.var_t0__blk1144_dn11))), (locals.var_vdsatcv2_dn12 - (0.5 * (locals.var_v4_dn12 + locals.var_t0__blk1144_dn12))),)
    } else {
        (locals.var_vdseffcv2, locals.var_vdseffcv2_dn3, locals.var_vdseffcv2_dn4, locals.var_vdseffcv2_dn5, locals.var_vdseffcv2_dn6, locals.var_vdseffcv2_dn7, locals.var_vdseffcv2_dn8, locals.var_vdseffcv2_dn9, locals.var_vdseffcv2_dn10, locals.var_vdseffcv2_dn11, locals.var_vdseffcv2_dn12,)
    }
};
        locals.var_vdseffcv2 = assign29930_e23650;
        locals.var_vdseffcv2_dn3 = assign29930_e23650_d_n3;
        locals.var_vdseffcv2_dn4 = assign29930_e23650_d_n4;
        locals.var_vdseffcv2_dn5 = assign29930_e23650_d_n5;
        locals.var_vdseffcv2_dn6 = assign29930_e23650_d_n6;
        locals.var_vdseffcv2_dn7 = assign29930_e23650_d_n7;
        locals.var_vdseffcv2_dn8 = assign29930_e23650_d_n8;
        locals.var_vdseffcv2_dn9 = assign29930_e23650_d_n9;
        locals.var_vdseffcv2_dn10 = assign29930_e23650_d_n10;
        locals.var_vdseffcv2_dn11 = assign29930_e23650_d_n11;
        locals.var_vdseffcv2_dn12 = assign29930_e23650_d_n12;

        let assign29940_e23653: f64 = if locals.var_b4soisoimod == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1708 = assign29940_e23653;

        let (assign29950_e23659, assign29950_e23659_d_n3, assign29950_e23659_d_n4, assign29950_e23659_d_n5, assign29950_e23659_d_n6, assign29950_e23659_d_n7, assign29950_e23659_d_n8, assign29950_e23659_d_n9, assign29950_e23659_d_n10, assign29950_e23659_d_n11, assign29950_e23659_d_n12,) = {
    if ((locals.var_guard1698 != 0.0) && (locals.var_guard1708 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbulk, locals.var_qbulk_dn3, locals.var_qbulk_dn4, locals.var_qbulk_dn5, locals.var_qbulk_dn6, locals.var_qbulk_dn7, locals.var_qbulk_dn8, locals.var_qbulk_dn9, locals.var_qbulk_dn10, locals.var_qbulk_dn11, locals.var_qbulk_dn12,)
    }
};
        locals.var_qbulk = assign29950_e23659;
        locals.var_qbulk_dn3 = assign29950_e23659_d_n3;
        locals.var_qbulk_dn4 = assign29950_e23659_d_n4;
        locals.var_qbulk_dn5 = assign29950_e23659_d_n5;
        locals.var_qbulk_dn6 = assign29950_e23659_d_n6;
        locals.var_qbulk_dn7 = assign29950_e23659_d_n7;
        locals.var_qbulk_dn8 = assign29950_e23659_d_n8;
        locals.var_qbulk_dn9 = assign29950_e23659_d_n9;
        locals.var_qbulk_dn10 = assign29950_e23659_d_n10;
        locals.var_qbulk_dn11 = assign29950_e23659_d_n11;
        locals.var_qbulk_dn12 = assign29950_e23659_d_n12;

        let (assign29960_e23668, assign29960_e23668_d_n3, assign29960_e23668_d_n4, assign29960_e23668_d_n5, assign29960_e23668_d_n6, assign29960_e23668_d_n7, assign29960_e23668_d_n8, assign29960_e23668_d_n9, assign29960_e23668_d_n10, assign29960_e23668_d_n11, assign29960_e23668_d_n12,) = {
    if ((locals.var_guard1698 != 0.0) && (locals.var_guard1708 == 0.0)) {
        let assign29960_e23666: f64 = (locals.var_abulkcv * locals.var_vdseffcv);
        (assign29960_e23666, ((locals.var_abulkcv_dn3 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn3)), ((locals.var_abulkcv_dn4 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn4)), ((locals.var_abulkcv_dn5 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn5)), ((locals.var_abulkcv_dn6 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn6)), ((locals.var_abulkcv_dn7 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn7)), ((locals.var_abulkcv_dn8 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn8)), ((locals.var_abulkcv_dn9 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn9)), ((locals.var_abulkcv_dn10 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn10)), ((locals.var_abulkcv_dn11 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn11)), ((locals.var_abulkcv_dn12 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn12)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign29960_e23668;
        locals.var_t0__blk1144_dn3 = assign29960_e23668_d_n3;
        locals.var_t0__blk1144_dn4 = assign29960_e23668_d_n4;
        locals.var_t0__blk1144_dn5 = assign29960_e23668_d_n5;
        locals.var_t0__blk1144_dn6 = assign29960_e23668_d_n6;
        locals.var_t0__blk1144_dn7 = assign29960_e23668_d_n7;
        locals.var_t0__blk1144_dn8 = assign29960_e23668_d_n8;
        locals.var_t0__blk1144_dn9 = assign29960_e23668_d_n9;
        locals.var_t0__blk1144_dn10 = assign29960_e23668_d_n10;
        locals.var_t0__blk1144_dn11 = assign29960_e23668_d_n11;
        locals.var_t0__blk1144_dn12 = assign29960_e23668_d_n12;

        let (assign29970_e23683, assign29970_e23683_d_n3, assign29970_e23683_d_n4, assign29970_e23683_d_n5, assign29970_e23683_d_n6, assign29970_e23683_d_n7, assign29970_e23683_d_n8, assign29970_e23683_d_n9, assign29970_e23683_d_n10, assign29970_e23683_d_n11, assign29970_e23683_d_n12,) = {
    if ((locals.var_guard1698 != 0.0) && (locals.var_guard1708 == 0.0)) {
        let assign29970_e23677: f64 = (0.5 * locals.var_t0__blk1144);
        let assign29970_e23678: f64 = (locals.var_vgsteff__blk1175 - assign29970_e23677);
        let assign29970_e23680: f64 = (assign29970_e23678 + 1e-20);
        let assign29970_e23681: f64 = (12.0 * assign29970_e23680);
        (assign29970_e23681, (12.0 * (locals.var_vgsteff__blk1175_dn3 - (0.5 * locals.var_t0__blk1144_dn3))), (12.0 * (locals.var_vgsteff__blk1175_dn4 - (0.5 * locals.var_t0__blk1144_dn4))), (12.0 * (locals.var_vgsteff__blk1175_dn5 - (0.5 * locals.var_t0__blk1144_dn5))), (12.0 * (locals.var_vgsteff__blk1175_dn6 - (0.5 * locals.var_t0__blk1144_dn6))), (12.0 * (locals.var_vgsteff__blk1175_dn7 - (0.5 * locals.var_t0__blk1144_dn7))), (12.0 * (locals.var_vgsteff__blk1175_dn8 - (0.5 * locals.var_t0__blk1144_dn8))), (12.0 * (locals.var_vgsteff__blk1175_dn9 - (0.5 * locals.var_t0__blk1144_dn9))), (12.0 * (locals.var_vgsteff__blk1175_dn10 - (0.5 * locals.var_t0__blk1144_dn10))), (12.0 * (locals.var_vgsteff__blk1175_dn11 - (0.5 * locals.var_t0__blk1144_dn11))), (12.0 * (locals.var_vgsteff__blk1175_dn12 - (0.5 * locals.var_t0__blk1144_dn12))),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign29970_e23683;
        locals.var_t1__blk1145_dn3 = assign29970_e23683_d_n3;
        locals.var_t1__blk1145_dn4 = assign29970_e23683_d_n4;
        locals.var_t1__blk1145_dn5 = assign29970_e23683_d_n5;
        locals.var_t1__blk1145_dn6 = assign29970_e23683_d_n6;
        locals.var_t1__blk1145_dn7 = assign29970_e23683_d_n7;
        locals.var_t1__blk1145_dn8 = assign29970_e23683_d_n8;
        locals.var_t1__blk1145_dn9 = assign29970_e23683_d_n9;
        locals.var_t1__blk1145_dn10 = assign29970_e23683_d_n10;
        locals.var_t1__blk1145_dn11 = assign29970_e23683_d_n11;
        locals.var_t1__blk1145_dn12 = assign29970_e23683_d_n12;

        let (assign29980_e23692, assign29980_e23692_d_n3, assign29980_e23692_d_n4, assign29980_e23692_d_n5, assign29980_e23692_d_n6, assign29980_e23692_d_n7, assign29980_e23692_d_n8, assign29980_e23692_d_n9, assign29980_e23692_d_n10, assign29980_e23692_d_n11, assign29980_e23692_d_n12,) = {
    if ((locals.var_guard1698 != 0.0) && (locals.var_guard1708 == 0.0)) {
        let assign29980_e23690: f64 = (locals.var_vdseffcv / locals.var_t1__blk1145);
        (assign29980_e23690, (((locals.var_vdseffcv_dn3 * locals.var_t1__blk1145) - (locals.var_vdseffcv * locals.var_t1__blk1145_dn3)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145)), (((locals.var_vdseffcv_dn4 * locals.var_t1__blk1145) - (locals.var_vdseffcv * locals.var_t1__blk1145_dn4)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145)), (((locals.var_vdseffcv_dn5 * locals.var_t1__blk1145) - (locals.var_vdseffcv * locals.var_t1__blk1145_dn5)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145)), (((locals.var_vdseffcv_dn6 * locals.var_t1__blk1145) - (locals.var_vdseffcv * locals.var_t1__blk1145_dn6)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145)), (((locals.var_vdseffcv_dn7 * locals.var_t1__blk1145) - (locals.var_vdseffcv * locals.var_t1__blk1145_dn7)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145)), (((locals.var_vdseffcv_dn8 * locals.var_t1__blk1145) - (locals.var_vdseffcv * locals.var_t1__blk1145_dn8)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145)), (((locals.var_vdseffcv_dn9 * locals.var_t1__blk1145) - (locals.var_vdseffcv * locals.var_t1__blk1145_dn9)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145)), (((locals.var_vdseffcv_dn10 * locals.var_t1__blk1145) - (locals.var_vdseffcv * locals.var_t1__blk1145_dn10)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145)), (((locals.var_vdseffcv_dn11 * locals.var_t1__blk1145) - (locals.var_vdseffcv * locals.var_t1__blk1145_dn11)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145)), (((locals.var_vdseffcv_dn12 * locals.var_t1__blk1145) - (locals.var_vdseffcv * locals.var_t1__blk1145_dn12)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145)),)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign29980_e23692;
        locals.var_t2__blk1146_dn3 = assign29980_e23692_d_n3;
        locals.var_t2__blk1146_dn4 = assign29980_e23692_d_n4;
        locals.var_t2__blk1146_dn5 = assign29980_e23692_d_n5;
        locals.var_t2__blk1146_dn6 = assign29980_e23692_d_n6;
        locals.var_t2__blk1146_dn7 = assign29980_e23692_d_n7;
        locals.var_t2__blk1146_dn8 = assign29980_e23692_d_n8;
        locals.var_t2__blk1146_dn9 = assign29980_e23692_d_n9;
        locals.var_t2__blk1146_dn10 = assign29980_e23692_d_n10;
        locals.var_t2__blk1146_dn11 = assign29980_e23692_d_n11;
        locals.var_t2__blk1146_dn12 = assign29980_e23692_d_n12;

        let (assign29990_e23701, assign29990_e23701_d_n3, assign29990_e23701_d_n4, assign29990_e23701_d_n5, assign29990_e23701_d_n6, assign29990_e23701_d_n7, assign29990_e23701_d_n8, assign29990_e23701_d_n9, assign29990_e23701_d_n10, assign29990_e23701_d_n11, assign29990_e23701_d_n12,) = {
    if ((locals.var_guard1698 != 0.0) && (locals.var_guard1708 == 0.0)) {
        let assign29990_e23699: f64 = (locals.var_t0__blk1144 * locals.var_t2__blk1146);
        (assign29990_e23699, ((locals.var_t0__blk1144_dn3 * locals.var_t2__blk1146) + (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn3)), ((locals.var_t0__blk1144_dn4 * locals.var_t2__blk1146) + (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn4)), ((locals.var_t0__blk1144_dn5 * locals.var_t2__blk1146) + (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn5)), ((locals.var_t0__blk1144_dn6 * locals.var_t2__blk1146) + (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn6)), ((locals.var_t0__blk1144_dn7 * locals.var_t2__blk1146) + (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn7)), ((locals.var_t0__blk1144_dn8 * locals.var_t2__blk1146) + (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn8)), ((locals.var_t0__blk1144_dn9 * locals.var_t2__blk1146) + (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn9)), ((locals.var_t0__blk1144_dn10 * locals.var_t2__blk1146) + (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn10)), ((locals.var_t0__blk1144_dn11 * locals.var_t2__blk1146) + (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn11)), ((locals.var_t0__blk1144_dn12 * locals.var_t2__blk1146) + (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn12)),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign29990_e23701;
        locals.var_t3__blk1147_dn3 = assign29990_e23701_d_n3;
        locals.var_t3__blk1147_dn4 = assign29990_e23701_d_n4;
        locals.var_t3__blk1147_dn5 = assign29990_e23701_d_n5;
        locals.var_t3__blk1147_dn6 = assign29990_e23701_d_n6;
        locals.var_t3__blk1147_dn7 = assign29990_e23701_d_n7;
        locals.var_t3__blk1147_dn8 = assign29990_e23701_d_n8;
        locals.var_t3__blk1147_dn9 = assign29990_e23701_d_n9;
        locals.var_t3__blk1147_dn10 = assign29990_e23701_d_n10;
        locals.var_t3__blk1147_dn11 = assign29990_e23701_d_n11;
        locals.var_t3__blk1147_dn12 = assign29990_e23701_d_n12;

        let (assign30000_e23710, assign30000_e23710_d_n3, assign30000_e23710_d_n4, assign30000_e23710_d_n5, assign30000_e23710_d_n6, assign30000_e23710_d_n7, assign30000_e23710_d_n8, assign30000_e23710_d_n9, assign30000_e23710_d_n10, assign30000_e23710_d_n11, assign30000_e23710_d_n12,) = {
    if ((locals.var_guard1698 != 0.0) && (locals.var_guard1708 == 0.0)) {
        let assign30000_e23708: f64 = (1.0 - locals.var_abulkcv);
        (assign30000_e23708, (-locals.var_abulkcv_dn3), (-locals.var_abulkcv_dn4), (-locals.var_abulkcv_dn5), (-locals.var_abulkcv_dn6), (-locals.var_abulkcv_dn7), (-locals.var_abulkcv_dn8), (-locals.var_abulkcv_dn9), (-locals.var_abulkcv_dn10), (-locals.var_abulkcv_dn11), (-locals.var_abulkcv_dn12),)
    } else {
        (locals.var_t7__blk1151, locals.var_t7__blk1151_dn3, locals.var_t7__blk1151_dn4, locals.var_t7__blk1151_dn5, locals.var_t7__blk1151_dn6, locals.var_t7__blk1151_dn7, locals.var_t7__blk1151_dn8, locals.var_t7__blk1151_dn9, locals.var_t7__blk1151_dn10, locals.var_t7__blk1151_dn11, locals.var_t7__blk1151_dn12,)
    }
};
        locals.var_t7__blk1151 = assign30000_e23710;
        locals.var_t7__blk1151_dn3 = assign30000_e23710_d_n3;
        locals.var_t7__blk1151_dn4 = assign30000_e23710_d_n4;
        locals.var_t7__blk1151_dn5 = assign30000_e23710_d_n5;
        locals.var_t7__blk1151_dn6 = assign30000_e23710_d_n6;
        locals.var_t7__blk1151_dn7 = assign30000_e23710_d_n7;
        locals.var_t7__blk1151_dn8 = assign30000_e23710_d_n8;
        locals.var_t7__blk1151_dn9 = assign30000_e23710_d_n9;
        locals.var_t7__blk1151_dn10 = assign30000_e23710_d_n10;
        locals.var_t7__blk1151_dn11 = assign30000_e23710_d_n11;
        locals.var_t7__blk1151_dn12 = assign30000_e23710_d_n12;

        let (assign30010_e23725, assign30010_e23725_d_n3, assign30010_e23725_d_n4, assign30010_e23725_d_n5, assign30010_e23725_d_n6, assign30010_e23725_d_n7, assign30010_e23725_d_n8, assign30010_e23725_d_n9, assign30010_e23725_d_n10, assign30010_e23725_d_n11, assign30010_e23725_d_n12,) = {
    if ((locals.var_guard1698 != 0.0) && (locals.var_guard1708 == 0.0)) {
        let assign30010_e23717: f64 = (locals.var_coxwlb * locals.var_t7__blk1151);
        let assign30010_e23720: f64 = (0.5 * locals.var_vdseffcv);
        let assign30010_e23722: f64 = (assign30010_e23720 - locals.var_t3__blk1147);
        let assign30010_e23723: f64 = (assign30010_e23717 * assign30010_e23722);
        (assign30010_e23723, ((((locals.var_coxwlb_dn3 * locals.var_t7__blk1151) + (locals.var_coxwlb * locals.var_t7__blk1151_dn3)) * assign30010_e23722) + (assign30010_e23717 * ((0.5 * locals.var_vdseffcv_dn3) - locals.var_t3__blk1147_dn3))), ((((locals.var_coxwlb_dn4 * locals.var_t7__blk1151) + (locals.var_coxwlb * locals.var_t7__blk1151_dn4)) * assign30010_e23722) + (assign30010_e23717 * ((0.5 * locals.var_vdseffcv_dn4) - locals.var_t3__blk1147_dn4))), ((((locals.var_coxwlb_dn5 * locals.var_t7__blk1151) + (locals.var_coxwlb * locals.var_t7__blk1151_dn5)) * assign30010_e23722) + (assign30010_e23717 * ((0.5 * locals.var_vdseffcv_dn5) - locals.var_t3__blk1147_dn5))), ((((locals.var_coxwlb_dn6 * locals.var_t7__blk1151) + (locals.var_coxwlb * locals.var_t7__blk1151_dn6)) * assign30010_e23722) + (assign30010_e23717 * ((0.5 * locals.var_vdseffcv_dn6) - locals.var_t3__blk1147_dn6))), ((((locals.var_coxwlb_dn7 * locals.var_t7__blk1151) + (locals.var_coxwlb * locals.var_t7__blk1151_dn7)) * assign30010_e23722) + (assign30010_e23717 * ((0.5 * locals.var_vdseffcv_dn7) - locals.var_t3__blk1147_dn7))), ((((locals.var_coxwlb_dn8 * locals.var_t7__blk1151) + (locals.var_coxwlb * locals.var_t7__blk1151_dn8)) * assign30010_e23722) + (assign30010_e23717 * ((0.5 * locals.var_vdseffcv_dn8) - locals.var_t3__blk1147_dn8))), ((((locals.var_coxwlb_dn9 * locals.var_t7__blk1151) + (locals.var_coxwlb * locals.var_t7__blk1151_dn9)) * assign30010_e23722) + (assign30010_e23717 * ((0.5 * locals.var_vdseffcv_dn9) - locals.var_t3__blk1147_dn9))), ((((locals.var_coxwlb_dn10 * locals.var_t7__blk1151) + (locals.var_coxwlb * locals.var_t7__blk1151_dn10)) * assign30010_e23722) + (assign30010_e23717 * ((0.5 * locals.var_vdseffcv_dn10) - locals.var_t3__blk1147_dn10))), ((((locals.var_coxwlb_dn11 * locals.var_t7__blk1151) + (locals.var_coxwlb * locals.var_t7__blk1151_dn11)) * assign30010_e23722) + (assign30010_e23717 * ((0.5 * locals.var_vdseffcv_dn11) - locals.var_t3__blk1147_dn11))), ((((locals.var_coxwlb_dn12 * locals.var_t7__blk1151) + (locals.var_coxwlb * locals.var_t7__blk1151_dn12)) * assign30010_e23722) + (assign30010_e23717 * ((0.5 * locals.var_vdseffcv_dn12) - locals.var_t3__blk1147_dn12))),)
    } else {
        (locals.var_qbulk, locals.var_qbulk_dn3, locals.var_qbulk_dn4, locals.var_qbulk_dn5, locals.var_qbulk_dn6, locals.var_qbulk_dn7, locals.var_qbulk_dn8, locals.var_qbulk_dn9, locals.var_qbulk_dn10, locals.var_qbulk_dn11, locals.var_qbulk_dn12,)
    }
};
        locals.var_qbulk = assign30010_e23725;
        locals.var_qbulk_dn3 = assign30010_e23725_d_n3;
        locals.var_qbulk_dn4 = assign30010_e23725_d_n4;
        locals.var_qbulk_dn5 = assign30010_e23725_d_n5;
        locals.var_qbulk_dn6 = assign30010_e23725_d_n6;
        locals.var_qbulk_dn7 = assign30010_e23725_d_n7;
        locals.var_qbulk_dn8 = assign30010_e23725_d_n8;
        locals.var_qbulk_dn9 = assign30010_e23725_d_n9;
        locals.var_qbulk_dn10 = assign30010_e23725_d_n10;
        locals.var_qbulk_dn11 = assign30010_e23725_d_n11;
        locals.var_qbulk_dn12 = assign30010_e23725_d_n12;

        let assign30020_e23736: f64 = if (((locals.var_b4soisoimod != 2.0) && (locals.var_b4soibodymod != 0.0)) && (locals.var_b4soiagbcp2 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1709 = assign30020_e23736;

        let (assign30030_e23747, assign30030_e23747_d_n3, assign30030_e23747_d_n4, assign30030_e23747_d_n5, assign30030_e23747_d_n6, assign30030_e23747_d_n7, assign30030_e23747_d_n8, assign30030_e23747_d_n9, assign30030_e23747_d_n10, assign30030_e23747_d_n11, assign30030_e23747_d_n12,) = {
    if (((locals.var_guard1698 != 0.0) && (locals.var_guard1708 == 0.0)) && (locals.var_guard1709 != 0.0)) {
        let assign30030_e23745: f64 = (locals.var_abulkcv * locals.var_vdseffcv2);
        (assign30030_e23745, ((locals.var_abulkcv_dn3 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn3)), ((locals.var_abulkcv_dn4 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn4)), ((locals.var_abulkcv_dn5 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn5)), ((locals.var_abulkcv_dn6 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn6)), ((locals.var_abulkcv_dn7 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn7)), ((locals.var_abulkcv_dn8 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn8)), ((locals.var_abulkcv_dn9 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn9)), ((locals.var_abulkcv_dn10 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn10)), ((locals.var_abulkcv_dn11 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn11)), ((locals.var_abulkcv_dn12 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn12)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign30030_e23747;
        locals.var_t0__blk1144_dn3 = assign30030_e23747_d_n3;
        locals.var_t0__blk1144_dn4 = assign30030_e23747_d_n4;
        locals.var_t0__blk1144_dn5 = assign30030_e23747_d_n5;
        locals.var_t0__blk1144_dn6 = assign30030_e23747_d_n6;
        locals.var_t0__blk1144_dn7 = assign30030_e23747_d_n7;
        locals.var_t0__blk1144_dn8 = assign30030_e23747_d_n8;
        locals.var_t0__blk1144_dn9 = assign30030_e23747_d_n9;
        locals.var_t0__blk1144_dn10 = assign30030_e23747_d_n10;
        locals.var_t0__blk1144_dn11 = assign30030_e23747_d_n11;
        locals.var_t0__blk1144_dn12 = assign30030_e23747_d_n12;

        let (assign30040_e23764, assign30040_e23764_d_n3, assign30040_e23764_d_n4, assign30040_e23764_d_n5, assign30040_e23764_d_n6, assign30040_e23764_d_n7, assign30040_e23764_d_n8, assign30040_e23764_d_n9, assign30040_e23764_d_n10, assign30040_e23764_d_n11, assign30040_e23764_d_n12,) = {
    if (((locals.var_guard1698 != 0.0) && (locals.var_guard1708 == 0.0)) && (locals.var_guard1709 != 0.0)) {
        let assign30040_e23758: f64 = (0.5 * locals.var_t0__blk1144);
        let assign30040_e23759: f64 = (locals.var_vgsteff2 - assign30040_e23758);
        let assign30040_e23761: f64 = (assign30040_e23759 + 1e-20);
        let assign30040_e23762: f64 = (12.0 * assign30040_e23761);
        (assign30040_e23762, (12.0 * (locals.var_vgsteff2_dn3 - (0.5 * locals.var_t0__blk1144_dn3))), (12.0 * (locals.var_vgsteff2_dn4 - (0.5 * locals.var_t0__blk1144_dn4))), (12.0 * (locals.var_vgsteff2_dn5 - (0.5 * locals.var_t0__blk1144_dn5))), (12.0 * (locals.var_vgsteff2_dn6 - (0.5 * locals.var_t0__blk1144_dn6))), (12.0 * (locals.var_vgsteff2_dn7 - (0.5 * locals.var_t0__blk1144_dn7))), (12.0 * (locals.var_vgsteff2_dn8 - (0.5 * locals.var_t0__blk1144_dn8))), (12.0 * (locals.var_vgsteff2_dn9 - (0.5 * locals.var_t0__blk1144_dn9))), (12.0 * (locals.var_vgsteff2_dn10 - (0.5 * locals.var_t0__blk1144_dn10))), (12.0 * (locals.var_vgsteff2_dn11 - (0.5 * locals.var_t0__blk1144_dn11))), (12.0 * (locals.var_vgsteff2_dn12 - (0.5 * locals.var_t0__blk1144_dn12))),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign30040_e23764;
        locals.var_t1__blk1145_dn3 = assign30040_e23764_d_n3;
        locals.var_t1__blk1145_dn4 = assign30040_e23764_d_n4;
        locals.var_t1__blk1145_dn5 = assign30040_e23764_d_n5;
        locals.var_t1__blk1145_dn6 = assign30040_e23764_d_n6;
        locals.var_t1__blk1145_dn7 = assign30040_e23764_d_n7;
        locals.var_t1__blk1145_dn8 = assign30040_e23764_d_n8;
        locals.var_t1__blk1145_dn9 = assign30040_e23764_d_n9;
        locals.var_t1__blk1145_dn10 = assign30040_e23764_d_n10;
        locals.var_t1__blk1145_dn11 = assign30040_e23764_d_n11;
        locals.var_t1__blk1145_dn12 = assign30040_e23764_d_n12;

        let (assign30050_e23775, assign30050_e23775_d_n3, assign30050_e23775_d_n4, assign30050_e23775_d_n5, assign30050_e23775_d_n6, assign30050_e23775_d_n7, assign30050_e23775_d_n8, assign30050_e23775_d_n9, assign30050_e23775_d_n10, assign30050_e23775_d_n11, assign30050_e23775_d_n12,) = {
    if (((locals.var_guard1698 != 0.0) && (locals.var_guard1708 == 0.0)) && (locals.var_guard1709 != 0.0)) {
        let assign30050_e23773: f64 = (locals.var_vdseffcv2 / locals.var_t1__blk1145);
        (assign30050_e23773, (((locals.var_vdseffcv2_dn3 * locals.var_t1__blk1145) - (locals.var_vdseffcv2 * locals.var_t1__blk1145_dn3)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145)), (((locals.var_vdseffcv2_dn4 * locals.var_t1__blk1145) - (locals.var_vdseffcv2 * locals.var_t1__blk1145_dn4)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145)), (((locals.var_vdseffcv2_dn5 * locals.var_t1__blk1145) - (locals.var_vdseffcv2 * locals.var_t1__blk1145_dn5)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145)), (((locals.var_vdseffcv2_dn6 * locals.var_t1__blk1145) - (locals.var_vdseffcv2 * locals.var_t1__blk1145_dn6)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145)), (((locals.var_vdseffcv2_dn7 * locals.var_t1__blk1145) - (locals.var_vdseffcv2 * locals.var_t1__blk1145_dn7)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145)), (((locals.var_vdseffcv2_dn8 * locals.var_t1__blk1145) - (locals.var_vdseffcv2 * locals.var_t1__blk1145_dn8)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145)), (((locals.var_vdseffcv2_dn9 * locals.var_t1__blk1145) - (locals.var_vdseffcv2 * locals.var_t1__blk1145_dn9)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145)), (((locals.var_vdseffcv2_dn10 * locals.var_t1__blk1145) - (locals.var_vdseffcv2 * locals.var_t1__blk1145_dn10)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145)), (((locals.var_vdseffcv2_dn11 * locals.var_t1__blk1145) - (locals.var_vdseffcv2 * locals.var_t1__blk1145_dn11)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145)), (((locals.var_vdseffcv2_dn12 * locals.var_t1__blk1145) - (locals.var_vdseffcv2 * locals.var_t1__blk1145_dn12)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145)),)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign30050_e23775;
        locals.var_t2__blk1146_dn3 = assign30050_e23775_d_n3;
        locals.var_t2__blk1146_dn4 = assign30050_e23775_d_n4;
        locals.var_t2__blk1146_dn5 = assign30050_e23775_d_n5;
        locals.var_t2__blk1146_dn6 = assign30050_e23775_d_n6;
        locals.var_t2__blk1146_dn7 = assign30050_e23775_d_n7;
        locals.var_t2__blk1146_dn8 = assign30050_e23775_d_n8;
        locals.var_t2__blk1146_dn9 = assign30050_e23775_d_n9;
        locals.var_t2__blk1146_dn10 = assign30050_e23775_d_n10;
        locals.var_t2__blk1146_dn11 = assign30050_e23775_d_n11;
        locals.var_t2__blk1146_dn12 = assign30050_e23775_d_n12;

    }

    pub(super) fn stamp_transient_block_81(
        locals: &mut StampLocals,
    ) {
        let (assign30060_e23786, assign30060_e23786_d_n3, assign30060_e23786_d_n4, assign30060_e23786_d_n5, assign30060_e23786_d_n6, assign30060_e23786_d_n7, assign30060_e23786_d_n8, assign30060_e23786_d_n9, assign30060_e23786_d_n10, assign30060_e23786_d_n11, assign30060_e23786_d_n12,) = {
    if (((locals.var_guard1698 != 0.0) && (locals.var_guard1708 == 0.0)) && (locals.var_guard1709 != 0.0)) {
        let assign30060_e23784: f64 = (locals.var_t0__blk1144 * locals.var_t2__blk1146);
        (assign30060_e23784, ((locals.var_t0__blk1144_dn3 * locals.var_t2__blk1146) + (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn3)), ((locals.var_t0__blk1144_dn4 * locals.var_t2__blk1146) + (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn4)), ((locals.var_t0__blk1144_dn5 * locals.var_t2__blk1146) + (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn5)), ((locals.var_t0__blk1144_dn6 * locals.var_t2__blk1146) + (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn6)), ((locals.var_t0__blk1144_dn7 * locals.var_t2__blk1146) + (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn7)), ((locals.var_t0__blk1144_dn8 * locals.var_t2__blk1146) + (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn8)), ((locals.var_t0__blk1144_dn9 * locals.var_t2__blk1146) + (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn9)), ((locals.var_t0__blk1144_dn10 * locals.var_t2__blk1146) + (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn10)), ((locals.var_t0__blk1144_dn11 * locals.var_t2__blk1146) + (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn11)), ((locals.var_t0__blk1144_dn12 * locals.var_t2__blk1146) + (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn12)),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign30060_e23786;
        locals.var_t3__blk1147_dn3 = assign30060_e23786_d_n3;
        locals.var_t3__blk1147_dn4 = assign30060_e23786_d_n4;
        locals.var_t3__blk1147_dn5 = assign30060_e23786_d_n5;
        locals.var_t3__blk1147_dn6 = assign30060_e23786_d_n6;
        locals.var_t3__blk1147_dn7 = assign30060_e23786_d_n7;
        locals.var_t3__blk1147_dn8 = assign30060_e23786_d_n8;
        locals.var_t3__blk1147_dn9 = assign30060_e23786_d_n9;
        locals.var_t3__blk1147_dn10 = assign30060_e23786_d_n10;
        locals.var_t3__blk1147_dn11 = assign30060_e23786_d_n11;
        locals.var_t3__blk1147_dn12 = assign30060_e23786_d_n12;

        let (assign30070_e23797, assign30070_e23797_d_n3, assign30070_e23797_d_n4, assign30070_e23797_d_n5, assign30070_e23797_d_n6, assign30070_e23797_d_n7, assign30070_e23797_d_n8, assign30070_e23797_d_n9, assign30070_e23797_d_n10, assign30070_e23797_d_n11, assign30070_e23797_d_n12,) = {
    if (((locals.var_guard1698 != 0.0) && (locals.var_guard1708 == 0.0)) && (locals.var_guard1709 != 0.0)) {
        let assign30070_e23795: f64 = (1.0 - locals.var_abulkcv);
        (assign30070_e23795, (-locals.var_abulkcv_dn3), (-locals.var_abulkcv_dn4), (-locals.var_abulkcv_dn5), (-locals.var_abulkcv_dn6), (-locals.var_abulkcv_dn7), (-locals.var_abulkcv_dn8), (-locals.var_abulkcv_dn9), (-locals.var_abulkcv_dn10), (-locals.var_abulkcv_dn11), (-locals.var_abulkcv_dn12),)
    } else {
        (locals.var_t7__blk1151, locals.var_t7__blk1151_dn3, locals.var_t7__blk1151_dn4, locals.var_t7__blk1151_dn5, locals.var_t7__blk1151_dn6, locals.var_t7__blk1151_dn7, locals.var_t7__blk1151_dn8, locals.var_t7__blk1151_dn9, locals.var_t7__blk1151_dn10, locals.var_t7__blk1151_dn11, locals.var_t7__blk1151_dn12,)
    }
};
        locals.var_t7__blk1151 = assign30070_e23797;
        locals.var_t7__blk1151_dn3 = assign30070_e23797_d_n3;
        locals.var_t7__blk1151_dn4 = assign30070_e23797_d_n4;
        locals.var_t7__blk1151_dn5 = assign30070_e23797_d_n5;
        locals.var_t7__blk1151_dn6 = assign30070_e23797_d_n6;
        locals.var_t7__blk1151_dn7 = assign30070_e23797_d_n7;
        locals.var_t7__blk1151_dn8 = assign30070_e23797_d_n8;
        locals.var_t7__blk1151_dn9 = assign30070_e23797_d_n9;
        locals.var_t7__blk1151_dn10 = assign30070_e23797_d_n10;
        locals.var_t7__blk1151_dn11 = assign30070_e23797_d_n11;
        locals.var_t7__blk1151_dn12 = assign30070_e23797_d_n12;

        let (assign30080_e23816, assign30080_e23816_d_n3, assign30080_e23816_d_n4, assign30080_e23816_d_n5, assign30080_e23816_d_n6, assign30080_e23816_d_n7, assign30080_e23816_d_n8, assign30080_e23816_d_n9, assign30080_e23816_d_n10, assign30080_e23816_d_n11, assign30080_e23816_d_n12,) = {
    if (((locals.var_guard1698 != 0.0) && (locals.var_guard1708 == 0.0)) && (locals.var_guard1709 != 0.0)) {
        let assign30080_e23807: f64 = (locals.var_coxwlb2 * locals.var_t7__blk1151);
        let assign30080_e23810: f64 = (0.5 * locals.var_vdseffcv2);
        let assign30080_e23812: f64 = (assign30080_e23810 - locals.var_t3__blk1147);
        let assign30080_e23813: f64 = (assign30080_e23807 * assign30080_e23812);
        let assign30080_e23814: f64 = (locals.var_qbulk + assign30080_e23813);
        (assign30080_e23814, (locals.var_qbulk_dn3 + ((((locals.var_coxwlb2_dn3 * locals.var_t7__blk1151) + (locals.var_coxwlb2 * locals.var_t7__blk1151_dn3)) * assign30080_e23812) + (assign30080_e23807 * ((0.5 * locals.var_vdseffcv2_dn3) - locals.var_t3__blk1147_dn3)))), (locals.var_qbulk_dn4 + ((((locals.var_coxwlb2_dn4 * locals.var_t7__blk1151) + (locals.var_coxwlb2 * locals.var_t7__blk1151_dn4)) * assign30080_e23812) + (assign30080_e23807 * ((0.5 * locals.var_vdseffcv2_dn4) - locals.var_t3__blk1147_dn4)))), (locals.var_qbulk_dn5 + ((((locals.var_coxwlb2_dn5 * locals.var_t7__blk1151) + (locals.var_coxwlb2 * locals.var_t7__blk1151_dn5)) * assign30080_e23812) + (assign30080_e23807 * ((0.5 * locals.var_vdseffcv2_dn5) - locals.var_t3__blk1147_dn5)))), (locals.var_qbulk_dn6 + ((((locals.var_coxwlb2_dn6 * locals.var_t7__blk1151) + (locals.var_coxwlb2 * locals.var_t7__blk1151_dn6)) * assign30080_e23812) + (assign30080_e23807 * ((0.5 * locals.var_vdseffcv2_dn6) - locals.var_t3__blk1147_dn6)))), (locals.var_qbulk_dn7 + ((((locals.var_coxwlb2_dn7 * locals.var_t7__blk1151) + (locals.var_coxwlb2 * locals.var_t7__blk1151_dn7)) * assign30080_e23812) + (assign30080_e23807 * ((0.5 * locals.var_vdseffcv2_dn7) - locals.var_t3__blk1147_dn7)))), (locals.var_qbulk_dn8 + ((((locals.var_coxwlb2_dn8 * locals.var_t7__blk1151) + (locals.var_coxwlb2 * locals.var_t7__blk1151_dn8)) * assign30080_e23812) + (assign30080_e23807 * ((0.5 * locals.var_vdseffcv2_dn8) - locals.var_t3__blk1147_dn8)))), (locals.var_qbulk_dn9 + ((((locals.var_coxwlb2_dn9 * locals.var_t7__blk1151) + (locals.var_coxwlb2 * locals.var_t7__blk1151_dn9)) * assign30080_e23812) + (assign30080_e23807 * ((0.5 * locals.var_vdseffcv2_dn9) - locals.var_t3__blk1147_dn9)))), (locals.var_qbulk_dn10 + ((((locals.var_coxwlb2_dn10 * locals.var_t7__blk1151) + (locals.var_coxwlb2 * locals.var_t7__blk1151_dn10)) * assign30080_e23812) + (assign30080_e23807 * ((0.5 * locals.var_vdseffcv2_dn10) - locals.var_t3__blk1147_dn10)))), (locals.var_qbulk_dn11 + ((((locals.var_coxwlb2_dn11 * locals.var_t7__blk1151) + (locals.var_coxwlb2 * locals.var_t7__blk1151_dn11)) * assign30080_e23812) + (assign30080_e23807 * ((0.5 * locals.var_vdseffcv2_dn11) - locals.var_t3__blk1147_dn11)))), (locals.var_qbulk_dn12 + ((((locals.var_coxwlb2_dn12 * locals.var_t7__blk1151) + (locals.var_coxwlb2 * locals.var_t7__blk1151_dn12)) * assign30080_e23812) + (assign30080_e23807 * ((0.5 * locals.var_vdseffcv2_dn12) - locals.var_t3__blk1147_dn12)))),)
    } else {
        (locals.var_qbulk, locals.var_qbulk_dn3, locals.var_qbulk_dn4, locals.var_qbulk_dn5, locals.var_qbulk_dn6, locals.var_qbulk_dn7, locals.var_qbulk_dn8, locals.var_qbulk_dn9, locals.var_qbulk_dn10, locals.var_qbulk_dn11, locals.var_qbulk_dn12,)
    }
};
        locals.var_qbulk = assign30080_e23816;
        locals.var_qbulk_dn3 = assign30080_e23816_d_n3;
        locals.var_qbulk_dn4 = assign30080_e23816_d_n4;
        locals.var_qbulk_dn5 = assign30080_e23816_d_n5;
        locals.var_qbulk_dn6 = assign30080_e23816_d_n6;
        locals.var_qbulk_dn7 = assign30080_e23816_d_n7;
        locals.var_qbulk_dn8 = assign30080_e23816_d_n8;
        locals.var_qbulk_dn9 = assign30080_e23816_d_n9;
        locals.var_qbulk_dn10 = assign30080_e23816_d_n10;
        locals.var_qbulk_dn11 = assign30080_e23816_d_n11;
        locals.var_qbulk_dn12 = assign30080_e23816_d_n12;

        let (assign30090_e23822, assign30090_e23822_d_n3, assign30090_e23822_d_n4, assign30090_e23822_d_n5, assign30090_e23822_d_n6, assign30090_e23822_d_n7, assign30090_e23822_d_n8, assign30090_e23822_d_n9, assign30090_e23822_d_n10, assign30090_e23822_d_n11, assign30090_e23822_d_n12,) = {
    if (locals.var_guard1698 != 0.0) {
        let assign30090_e23820: f64 = (locals.var_abulkcv * locals.var_vdseffcv);
        (assign30090_e23820, ((locals.var_abulkcv_dn3 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn3)), ((locals.var_abulkcv_dn4 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn4)), ((locals.var_abulkcv_dn5 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn5)), ((locals.var_abulkcv_dn6 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn6)), ((locals.var_abulkcv_dn7 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn7)), ((locals.var_abulkcv_dn8 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn8)), ((locals.var_abulkcv_dn9 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn9)), ((locals.var_abulkcv_dn10 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn10)), ((locals.var_abulkcv_dn11 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn11)), ((locals.var_abulkcv_dn12 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn12)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign30090_e23822;
        locals.var_t0__blk1144_dn3 = assign30090_e23822_d_n3;
        locals.var_t0__blk1144_dn4 = assign30090_e23822_d_n4;
        locals.var_t0__blk1144_dn5 = assign30090_e23822_d_n5;
        locals.var_t0__blk1144_dn6 = assign30090_e23822_d_n6;
        locals.var_t0__blk1144_dn7 = assign30090_e23822_d_n7;
        locals.var_t0__blk1144_dn8 = assign30090_e23822_d_n8;
        locals.var_t0__blk1144_dn9 = assign30090_e23822_d_n9;
        locals.var_t0__blk1144_dn10 = assign30090_e23822_d_n10;
        locals.var_t0__blk1144_dn11 = assign30090_e23822_d_n11;
        locals.var_t0__blk1144_dn12 = assign30090_e23822_d_n12;

        let (assign30100_e23834, assign30100_e23834_d_n3, assign30100_e23834_d_n4, assign30100_e23834_d_n5, assign30100_e23834_d_n6, assign30100_e23834_d_n7, assign30100_e23834_d_n8, assign30100_e23834_d_n9, assign30100_e23834_d_n10, assign30100_e23834_d_n11, assign30100_e23834_d_n12,) = {
    if (locals.var_guard1698 != 0.0) {
        let assign30100_e23828: f64 = (0.5 * locals.var_t0__blk1144);
        let assign30100_e23829: f64 = (locals.var_vgsteff__blk1175 - assign30100_e23828);
        let assign30100_e23831: f64 = (assign30100_e23829 + 1e-20);
        let assign30100_e23832: f64 = (12.0 * assign30100_e23831);
        (assign30100_e23832, (12.0 * (locals.var_vgsteff__blk1175_dn3 - (0.5 * locals.var_t0__blk1144_dn3))), (12.0 * (locals.var_vgsteff__blk1175_dn4 - (0.5 * locals.var_t0__blk1144_dn4))), (12.0 * (locals.var_vgsteff__blk1175_dn5 - (0.5 * locals.var_t0__blk1144_dn5))), (12.0 * (locals.var_vgsteff__blk1175_dn6 - (0.5 * locals.var_t0__blk1144_dn6))), (12.0 * (locals.var_vgsteff__blk1175_dn7 - (0.5 * locals.var_t0__blk1144_dn7))), (12.0 * (locals.var_vgsteff__blk1175_dn8 - (0.5 * locals.var_t0__blk1144_dn8))), (12.0 * (locals.var_vgsteff__blk1175_dn9 - (0.5 * locals.var_t0__blk1144_dn9))), (12.0 * (locals.var_vgsteff__blk1175_dn10 - (0.5 * locals.var_t0__blk1144_dn10))), (12.0 * (locals.var_vgsteff__blk1175_dn11 - (0.5 * locals.var_t0__blk1144_dn11))), (12.0 * (locals.var_vgsteff__blk1175_dn12 - (0.5 * locals.var_t0__blk1144_dn12))),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign30100_e23834;
        locals.var_t1__blk1145_dn3 = assign30100_e23834_d_n3;
        locals.var_t1__blk1145_dn4 = assign30100_e23834_d_n4;
        locals.var_t1__blk1145_dn5 = assign30100_e23834_d_n5;
        locals.var_t1__blk1145_dn6 = assign30100_e23834_d_n6;
        locals.var_t1__blk1145_dn7 = assign30100_e23834_d_n7;
        locals.var_t1__blk1145_dn8 = assign30100_e23834_d_n8;
        locals.var_t1__blk1145_dn9 = assign30100_e23834_d_n9;
        locals.var_t1__blk1145_dn10 = assign30100_e23834_d_n10;
        locals.var_t1__blk1145_dn11 = assign30100_e23834_d_n11;
        locals.var_t1__blk1145_dn12 = assign30100_e23834_d_n12;

        let (assign30110_e23840, assign30110_e23840_d_n3, assign30110_e23840_d_n4, assign30110_e23840_d_n5, assign30110_e23840_d_n6, assign30110_e23840_d_n7, assign30110_e23840_d_n8, assign30110_e23840_d_n9, assign30110_e23840_d_n10, assign30110_e23840_d_n11, assign30110_e23840_d_n12,) = {
    if (locals.var_guard1698 != 0.0) {
        let assign30110_e23838: f64 = (locals.var_t0__blk1144 / locals.var_t1__blk1145);
        (assign30110_e23838, (((locals.var_t0__blk1144_dn3 * locals.var_t1__blk1145) - (locals.var_t0__blk1144 * locals.var_t1__blk1145_dn3)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145)), (((locals.var_t0__blk1144_dn4 * locals.var_t1__blk1145) - (locals.var_t0__blk1144 * locals.var_t1__blk1145_dn4)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145)), (((locals.var_t0__blk1144_dn5 * locals.var_t1__blk1145) - (locals.var_t0__blk1144 * locals.var_t1__blk1145_dn5)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145)), (((locals.var_t0__blk1144_dn6 * locals.var_t1__blk1145) - (locals.var_t0__blk1144 * locals.var_t1__blk1145_dn6)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145)), (((locals.var_t0__blk1144_dn7 * locals.var_t1__blk1145) - (locals.var_t0__blk1144 * locals.var_t1__blk1145_dn7)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145)), (((locals.var_t0__blk1144_dn8 * locals.var_t1__blk1145) - (locals.var_t0__blk1144 * locals.var_t1__blk1145_dn8)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145)), (((locals.var_t0__blk1144_dn9 * locals.var_t1__blk1145) - (locals.var_t0__blk1144 * locals.var_t1__blk1145_dn9)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145)), (((locals.var_t0__blk1144_dn10 * locals.var_t1__blk1145) - (locals.var_t0__blk1144 * locals.var_t1__blk1145_dn10)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145)), (((locals.var_t0__blk1144_dn11 * locals.var_t1__blk1145) - (locals.var_t0__blk1144 * locals.var_t1__blk1145_dn11)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145)), (((locals.var_t0__blk1144_dn12 * locals.var_t1__blk1145) - (locals.var_t0__blk1144 * locals.var_t1__blk1145_dn12)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145)),)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign30110_e23840;
        locals.var_t2__blk1146_dn3 = assign30110_e23840_d_n3;
        locals.var_t2__blk1146_dn4 = assign30110_e23840_d_n4;
        locals.var_t2__blk1146_dn5 = assign30110_e23840_d_n5;
        locals.var_t2__blk1146_dn6 = assign30110_e23840_d_n6;
        locals.var_t2__blk1146_dn7 = assign30110_e23840_d_n7;
        locals.var_t2__blk1146_dn8 = assign30110_e23840_d_n8;
        locals.var_t2__blk1146_dn9 = assign30110_e23840_d_n9;
        locals.var_t2__blk1146_dn10 = assign30110_e23840_d_n10;
        locals.var_t2__blk1146_dn11 = assign30110_e23840_d_n11;
        locals.var_t2__blk1146_dn12 = assign30110_e23840_d_n12;

        let (assign30120_e23846, assign30120_e23846_d_n3, assign30120_e23846_d_n4, assign30120_e23846_d_n5, assign30120_e23846_d_n6, assign30120_e23846_d_n7, assign30120_e23846_d_n8, assign30120_e23846_d_n9, assign30120_e23846_d_n10, assign30120_e23846_d_n11, assign30120_e23846_d_n12,) = {
    if (locals.var_guard1698 != 0.0) {
        let assign30120_e23844: f64 = (locals.var_t0__blk1144 * locals.var_t2__blk1146);
        (assign30120_e23844, ((locals.var_t0__blk1144_dn3 * locals.var_t2__blk1146) + (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn3)), ((locals.var_t0__blk1144_dn4 * locals.var_t2__blk1146) + (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn4)), ((locals.var_t0__blk1144_dn5 * locals.var_t2__blk1146) + (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn5)), ((locals.var_t0__blk1144_dn6 * locals.var_t2__blk1146) + (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn6)), ((locals.var_t0__blk1144_dn7 * locals.var_t2__blk1146) + (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn7)), ((locals.var_t0__blk1144_dn8 * locals.var_t2__blk1146) + (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn8)), ((locals.var_t0__blk1144_dn9 * locals.var_t2__blk1146) + (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn9)), ((locals.var_t0__blk1144_dn10 * locals.var_t2__blk1146) + (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn10)), ((locals.var_t0__blk1144_dn11 * locals.var_t2__blk1146) + (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn11)), ((locals.var_t0__blk1144_dn12 * locals.var_t2__blk1146) + (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn12)),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign30120_e23846;
        locals.var_t3__blk1147_dn3 = assign30120_e23846_d_n3;
        locals.var_t3__blk1147_dn4 = assign30120_e23846_d_n4;
        locals.var_t3__blk1147_dn5 = assign30120_e23846_d_n5;
        locals.var_t3__blk1147_dn6 = assign30120_e23846_d_n6;
        locals.var_t3__blk1147_dn7 = assign30120_e23846_d_n7;
        locals.var_t3__blk1147_dn8 = assign30120_e23846_d_n8;
        locals.var_t3__blk1147_dn9 = assign30120_e23846_d_n9;
        locals.var_t3__blk1147_dn10 = assign30120_e23846_d_n10;
        locals.var_t3__blk1147_dn11 = assign30120_e23846_d_n11;
        locals.var_t3__blk1147_dn12 = assign30120_e23846_d_n12;

        let (assign30130_e23858, assign30130_e23858_d_n3, assign30130_e23858_d_n4, assign30130_e23858_d_n5, assign30130_e23858_d_n6, assign30130_e23858_d_n7, assign30130_e23858_d_n8, assign30130_e23858_d_n9, assign30130_e23858_d_n10, assign30130_e23858_d_n11, assign30130_e23858_d_n12,) = {
    if (locals.var_guard1698 != 0.0) {
        let assign30130_e23852: f64 = (0.5 * locals.var_t0__blk1144);
        let assign30130_e23853: f64 = (locals.var_vgsteff__blk1175 - assign30130_e23852);
        let assign30130_e23855: f64 = (assign30130_e23853 + locals.var_t3__blk1147);
        let assign30130_e23856: f64 = (locals.var_coxwl * assign30130_e23855);
        (assign30130_e23856, ((locals.var_coxwl_dn3 * assign30130_e23855) + (locals.var_coxwl * ((locals.var_vgsteff__blk1175_dn3 - (0.5 * locals.var_t0__blk1144_dn3)) + locals.var_t3__blk1147_dn3))), ((locals.var_coxwl_dn4 * assign30130_e23855) + (locals.var_coxwl * ((locals.var_vgsteff__blk1175_dn4 - (0.5 * locals.var_t0__blk1144_dn4)) + locals.var_t3__blk1147_dn4))), ((locals.var_coxwl_dn5 * assign30130_e23855) + (locals.var_coxwl * ((locals.var_vgsteff__blk1175_dn5 - (0.5 * locals.var_t0__blk1144_dn5)) + locals.var_t3__blk1147_dn5))), ((locals.var_coxwl_dn6 * assign30130_e23855) + (locals.var_coxwl * ((locals.var_vgsteff__blk1175_dn6 - (0.5 * locals.var_t0__blk1144_dn6)) + locals.var_t3__blk1147_dn6))), ((locals.var_coxwl_dn7 * assign30130_e23855) + (locals.var_coxwl * ((locals.var_vgsteff__blk1175_dn7 - (0.5 * locals.var_t0__blk1144_dn7)) + locals.var_t3__blk1147_dn7))), ((locals.var_coxwl_dn8 * assign30130_e23855) + (locals.var_coxwl * ((locals.var_vgsteff__blk1175_dn8 - (0.5 * locals.var_t0__blk1144_dn8)) + locals.var_t3__blk1147_dn8))), ((locals.var_coxwl_dn9 * assign30130_e23855) + (locals.var_coxwl * ((locals.var_vgsteff__blk1175_dn9 - (0.5 * locals.var_t0__blk1144_dn9)) + locals.var_t3__blk1147_dn9))), ((locals.var_coxwl_dn10 * assign30130_e23855) + (locals.var_coxwl * ((locals.var_vgsteff__blk1175_dn10 - (0.5 * locals.var_t0__blk1144_dn10)) + locals.var_t3__blk1147_dn10))), ((locals.var_coxwl_dn11 * assign30130_e23855) + (locals.var_coxwl * ((locals.var_vgsteff__blk1175_dn11 - (0.5 * locals.var_t0__blk1144_dn11)) + locals.var_t3__blk1147_dn11))), ((locals.var_coxwl_dn12 * assign30130_e23855) + (locals.var_coxwl * ((locals.var_vgsteff__blk1175_dn12 - (0.5 * locals.var_t0__blk1144_dn12)) + locals.var_t3__blk1147_dn12))),)
    } else {
        (locals.var_qinv, locals.var_qinv_dn3, locals.var_qinv_dn4, locals.var_qinv_dn5, locals.var_qinv_dn6, locals.var_qinv_dn7, locals.var_qinv_dn8, locals.var_qinv_dn9, locals.var_qinv_dn10, locals.var_qinv_dn11, locals.var_qinv_dn12,)
    }
};
        locals.var_qinv = assign30130_e23858;
        locals.var_qinv_dn3 = assign30130_e23858_d_n3;
        locals.var_qinv_dn4 = assign30130_e23858_d_n4;
        locals.var_qinv_dn5 = assign30130_e23858_d_n5;
        locals.var_qinv_dn6 = assign30130_e23858_d_n6;
        locals.var_qinv_dn7 = assign30130_e23858_d_n7;
        locals.var_qinv_dn8 = assign30130_e23858_d_n8;
        locals.var_qinv_dn9 = assign30130_e23858_d_n9;
        locals.var_qinv_dn10 = assign30130_e23858_d_n10;
        locals.var_qinv_dn11 = assign30130_e23858_d_n11;
        locals.var_qinv_dn12 = assign30130_e23858_d_n12;

        let assign30150_e23874: f64 = if (((locals.var_b4soisoimod != 2.0) && (locals.var_b4soibodymod != 0.0)) && (locals.var_b4soiagbcp2 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1710 = assign30150_e23874;

        let (assign30160_e23882, assign30160_e23882_d_n3, assign30160_e23882_d_n4, assign30160_e23882_d_n5, assign30160_e23882_d_n6, assign30160_e23882_d_n7, assign30160_e23882_d_n8, assign30160_e23882_d_n9, assign30160_e23882_d_n10, assign30160_e23882_d_n11, assign30160_e23882_d_n12,) = {
    if ((locals.var_guard1698 != 0.0) && (locals.var_guard1710 != 0.0)) {
        let assign30160_e23880: f64 = (locals.var_abulkcv * locals.var_vdseffcv2);
        (assign30160_e23880, ((locals.var_abulkcv_dn3 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn3)), ((locals.var_abulkcv_dn4 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn4)), ((locals.var_abulkcv_dn5 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn5)), ((locals.var_abulkcv_dn6 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn6)), ((locals.var_abulkcv_dn7 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn7)), ((locals.var_abulkcv_dn8 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn8)), ((locals.var_abulkcv_dn9 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn9)), ((locals.var_abulkcv_dn10 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn10)), ((locals.var_abulkcv_dn11 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn11)), ((locals.var_abulkcv_dn12 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn12)),)
    } else {
        (locals.var_t02, locals.var_t02_dn3, locals.var_t02_dn4, locals.var_t02_dn5, locals.var_t02_dn6, locals.var_t02_dn7, locals.var_t02_dn8, locals.var_t02_dn9, locals.var_t02_dn10, locals.var_t02_dn11, locals.var_t02_dn12,)
    }
};
        locals.var_t02 = assign30160_e23882;
        locals.var_t02_dn3 = assign30160_e23882_d_n3;
        locals.var_t02_dn4 = assign30160_e23882_d_n4;
        locals.var_t02_dn5 = assign30160_e23882_d_n5;
        locals.var_t02_dn6 = assign30160_e23882_d_n6;
        locals.var_t02_dn7 = assign30160_e23882_d_n7;
        locals.var_t02_dn8 = assign30160_e23882_d_n8;
        locals.var_t02_dn9 = assign30160_e23882_d_n9;
        locals.var_t02_dn10 = assign30160_e23882_d_n10;
        locals.var_t02_dn11 = assign30160_e23882_d_n11;
        locals.var_t02_dn12 = assign30160_e23882_d_n12;

        let (assign30170_e23896, assign30170_e23896_d_n3, assign30170_e23896_d_n4, assign30170_e23896_d_n5, assign30170_e23896_d_n6, assign30170_e23896_d_n7, assign30170_e23896_d_n8, assign30170_e23896_d_n9, assign30170_e23896_d_n10, assign30170_e23896_d_n11, assign30170_e23896_d_n12,) = {
    if ((locals.var_guard1698 != 0.0) && (locals.var_guard1710 != 0.0)) {
        let assign30170_e23890: f64 = (0.5 * locals.var_t02);
        let assign30170_e23891: f64 = (locals.var_vgsteff2 - assign30170_e23890);
        let assign30170_e23893: f64 = (assign30170_e23891 + 1e-20);
        let assign30170_e23894: f64 = (12.0 * assign30170_e23893);
        (assign30170_e23894, (12.0 * (locals.var_vgsteff2_dn3 - (0.5 * locals.var_t02_dn3))), (12.0 * (locals.var_vgsteff2_dn4 - (0.5 * locals.var_t02_dn4))), (12.0 * (locals.var_vgsteff2_dn5 - (0.5 * locals.var_t02_dn5))), (12.0 * (locals.var_vgsteff2_dn6 - (0.5 * locals.var_t02_dn6))), (12.0 * (locals.var_vgsteff2_dn7 - (0.5 * locals.var_t02_dn7))), (12.0 * (locals.var_vgsteff2_dn8 - (0.5 * locals.var_t02_dn8))), (12.0 * (locals.var_vgsteff2_dn9 - (0.5 * locals.var_t02_dn9))), (12.0 * (locals.var_vgsteff2_dn10 - (0.5 * locals.var_t02_dn10))), (12.0 * (locals.var_vgsteff2_dn11 - (0.5 * locals.var_t02_dn11))), (12.0 * (locals.var_vgsteff2_dn12 - (0.5 * locals.var_t02_dn12))),)
    } else {
        (locals.var_t12, locals.var_t12_dn3, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn12,)
    }
};
        locals.var_t12 = assign30170_e23896;
        locals.var_t12_dn3 = assign30170_e23896_d_n3;
        locals.var_t12_dn4 = assign30170_e23896_d_n4;
        locals.var_t12_dn5 = assign30170_e23896_d_n5;
        locals.var_t12_dn6 = assign30170_e23896_d_n6;
        locals.var_t12_dn7 = assign30170_e23896_d_n7;
        locals.var_t12_dn8 = assign30170_e23896_d_n8;
        locals.var_t12_dn9 = assign30170_e23896_d_n9;
        locals.var_t12_dn10 = assign30170_e23896_d_n10;
        locals.var_t12_dn11 = assign30170_e23896_d_n11;
        locals.var_t12_dn12 = assign30170_e23896_d_n12;

        let (assign30180_e23904, assign30180_e23904_d_n3, assign30180_e23904_d_n4, assign30180_e23904_d_n5, assign30180_e23904_d_n6, assign30180_e23904_d_n7, assign30180_e23904_d_n8, assign30180_e23904_d_n9, assign30180_e23904_d_n10, assign30180_e23904_d_n11, assign30180_e23904_d_n12,) = {
    if ((locals.var_guard1698 != 0.0) && (locals.var_guard1710 != 0.0)) {
        let assign30180_e23902: f64 = (locals.var_t02 / locals.var_t12);
        (assign30180_e23902, (((locals.var_t02_dn3 * locals.var_t12) - (locals.var_t02 * locals.var_t12_dn3)) / (locals.var_t12 * locals.var_t12)), (((locals.var_t02_dn4 * locals.var_t12) - (locals.var_t02 * locals.var_t12_dn4)) / (locals.var_t12 * locals.var_t12)), (((locals.var_t02_dn5 * locals.var_t12) - (locals.var_t02 * locals.var_t12_dn5)) / (locals.var_t12 * locals.var_t12)), (((locals.var_t02_dn6 * locals.var_t12) - (locals.var_t02 * locals.var_t12_dn6)) / (locals.var_t12 * locals.var_t12)), (((locals.var_t02_dn7 * locals.var_t12) - (locals.var_t02 * locals.var_t12_dn7)) / (locals.var_t12 * locals.var_t12)), (((locals.var_t02_dn8 * locals.var_t12) - (locals.var_t02 * locals.var_t12_dn8)) / (locals.var_t12 * locals.var_t12)), (((locals.var_t02_dn9 * locals.var_t12) - (locals.var_t02 * locals.var_t12_dn9)) / (locals.var_t12 * locals.var_t12)), (((locals.var_t02_dn10 * locals.var_t12) - (locals.var_t02 * locals.var_t12_dn10)) / (locals.var_t12 * locals.var_t12)), (((locals.var_t02_dn11 * locals.var_t12) - (locals.var_t02 * locals.var_t12_dn11)) / (locals.var_t12 * locals.var_t12)), (((locals.var_t02_dn12 * locals.var_t12) - (locals.var_t02 * locals.var_t12_dn12)) / (locals.var_t12 * locals.var_t12)),)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign30180_e23904;
        locals.var_t2__blk1146_dn3 = assign30180_e23904_d_n3;
        locals.var_t2__blk1146_dn4 = assign30180_e23904_d_n4;
        locals.var_t2__blk1146_dn5 = assign30180_e23904_d_n5;
        locals.var_t2__blk1146_dn6 = assign30180_e23904_d_n6;
        locals.var_t2__blk1146_dn7 = assign30180_e23904_d_n7;
        locals.var_t2__blk1146_dn8 = assign30180_e23904_d_n8;
        locals.var_t2__blk1146_dn9 = assign30180_e23904_d_n9;
        locals.var_t2__blk1146_dn10 = assign30180_e23904_d_n10;
        locals.var_t2__blk1146_dn11 = assign30180_e23904_d_n11;
        locals.var_t2__blk1146_dn12 = assign30180_e23904_d_n12;

        let (assign30190_e23912, assign30190_e23912_d_n3, assign30190_e23912_d_n4, assign30190_e23912_d_n5, assign30190_e23912_d_n6, assign30190_e23912_d_n7, assign30190_e23912_d_n8, assign30190_e23912_d_n9, assign30190_e23912_d_n10, assign30190_e23912_d_n11, assign30190_e23912_d_n12,) = {
    if ((locals.var_guard1698 != 0.0) && (locals.var_guard1710 != 0.0)) {
        let assign30190_e23910: f64 = (locals.var_t02 * locals.var_t2__blk1146);
        (assign30190_e23910, ((locals.var_t02_dn3 * locals.var_t2__blk1146) + (locals.var_t02 * locals.var_t2__blk1146_dn3)), ((locals.var_t02_dn4 * locals.var_t2__blk1146) + (locals.var_t02 * locals.var_t2__blk1146_dn4)), ((locals.var_t02_dn5 * locals.var_t2__blk1146) + (locals.var_t02 * locals.var_t2__blk1146_dn5)), ((locals.var_t02_dn6 * locals.var_t2__blk1146) + (locals.var_t02 * locals.var_t2__blk1146_dn6)), ((locals.var_t02_dn7 * locals.var_t2__blk1146) + (locals.var_t02 * locals.var_t2__blk1146_dn7)), ((locals.var_t02_dn8 * locals.var_t2__blk1146) + (locals.var_t02 * locals.var_t2__blk1146_dn8)), ((locals.var_t02_dn9 * locals.var_t2__blk1146) + (locals.var_t02 * locals.var_t2__blk1146_dn9)), ((locals.var_t02_dn10 * locals.var_t2__blk1146) + (locals.var_t02 * locals.var_t2__blk1146_dn10)), ((locals.var_t02_dn11 * locals.var_t2__blk1146) + (locals.var_t02 * locals.var_t2__blk1146_dn11)), ((locals.var_t02_dn12 * locals.var_t2__blk1146) + (locals.var_t02 * locals.var_t2__blk1146_dn12)),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign30190_e23912;
        locals.var_t3__blk1147_dn3 = assign30190_e23912_d_n3;
        locals.var_t3__blk1147_dn4 = assign30190_e23912_d_n4;
        locals.var_t3__blk1147_dn5 = assign30190_e23912_d_n5;
        locals.var_t3__blk1147_dn6 = assign30190_e23912_d_n6;
        locals.var_t3__blk1147_dn7 = assign30190_e23912_d_n7;
        locals.var_t3__blk1147_dn8 = assign30190_e23912_d_n8;
        locals.var_t3__blk1147_dn9 = assign30190_e23912_d_n9;
        locals.var_t3__blk1147_dn10 = assign30190_e23912_d_n10;
        locals.var_t3__blk1147_dn11 = assign30190_e23912_d_n11;
        locals.var_t3__blk1147_dn12 = assign30190_e23912_d_n12;

        let (assign30200_e23928, assign30200_e23928_d_n3, assign30200_e23928_d_n4, assign30200_e23928_d_n5, assign30200_e23928_d_n6, assign30200_e23928_d_n7, assign30200_e23928_d_n8, assign30200_e23928_d_n9, assign30200_e23928_d_n10, assign30200_e23928_d_n11, assign30200_e23928_d_n12,) = {
    if ((locals.var_guard1698 != 0.0) && (locals.var_guard1710 != 0.0)) {
        let assign30200_e23921: f64 = (0.5 * locals.var_t02);
        let assign30200_e23922: f64 = (locals.var_vgsteff2 - assign30200_e23921);
        let assign30200_e23924: f64 = (assign30200_e23922 + locals.var_t3__blk1147);
        let assign30200_e23925: f64 = (locals.var_coxwl2 * assign30200_e23924);
        let assign30200_e23926: f64 = (locals.var_qinv + assign30200_e23925);
        (assign30200_e23926, (locals.var_qinv_dn3 + ((locals.var_coxwl2_dn3 * assign30200_e23924) + (locals.var_coxwl2 * ((locals.var_vgsteff2_dn3 - (0.5 * locals.var_t02_dn3)) + locals.var_t3__blk1147_dn3)))), (locals.var_qinv_dn4 + ((locals.var_coxwl2_dn4 * assign30200_e23924) + (locals.var_coxwl2 * ((locals.var_vgsteff2_dn4 - (0.5 * locals.var_t02_dn4)) + locals.var_t3__blk1147_dn4)))), (locals.var_qinv_dn5 + ((locals.var_coxwl2_dn5 * assign30200_e23924) + (locals.var_coxwl2 * ((locals.var_vgsteff2_dn5 - (0.5 * locals.var_t02_dn5)) + locals.var_t3__blk1147_dn5)))), (locals.var_qinv_dn6 + ((locals.var_coxwl2_dn6 * assign30200_e23924) + (locals.var_coxwl2 * ((locals.var_vgsteff2_dn6 - (0.5 * locals.var_t02_dn6)) + locals.var_t3__blk1147_dn6)))), (locals.var_qinv_dn7 + ((locals.var_coxwl2_dn7 * assign30200_e23924) + (locals.var_coxwl2 * ((locals.var_vgsteff2_dn7 - (0.5 * locals.var_t02_dn7)) + locals.var_t3__blk1147_dn7)))), (locals.var_qinv_dn8 + ((locals.var_coxwl2_dn8 * assign30200_e23924) + (locals.var_coxwl2 * ((locals.var_vgsteff2_dn8 - (0.5 * locals.var_t02_dn8)) + locals.var_t3__blk1147_dn8)))), (locals.var_qinv_dn9 + ((locals.var_coxwl2_dn9 * assign30200_e23924) + (locals.var_coxwl2 * ((locals.var_vgsteff2_dn9 - (0.5 * locals.var_t02_dn9)) + locals.var_t3__blk1147_dn9)))), (locals.var_qinv_dn10 + ((locals.var_coxwl2_dn10 * assign30200_e23924) + (locals.var_coxwl2 * ((locals.var_vgsteff2_dn10 - (0.5 * locals.var_t02_dn10)) + locals.var_t3__blk1147_dn10)))), (locals.var_qinv_dn11 + ((locals.var_coxwl2_dn11 * assign30200_e23924) + (locals.var_coxwl2 * ((locals.var_vgsteff2_dn11 - (0.5 * locals.var_t02_dn11)) + locals.var_t3__blk1147_dn11)))), (locals.var_qinv_dn12 + ((locals.var_coxwl2_dn12 * assign30200_e23924) + (locals.var_coxwl2 * ((locals.var_vgsteff2_dn12 - (0.5 * locals.var_t02_dn12)) + locals.var_t3__blk1147_dn12)))),)
    } else {
        (locals.var_qinv, locals.var_qinv_dn3, locals.var_qinv_dn4, locals.var_qinv_dn5, locals.var_qinv_dn6, locals.var_qinv_dn7, locals.var_qinv_dn8, locals.var_qinv_dn9, locals.var_qinv_dn10, locals.var_qinv_dn11, locals.var_qinv_dn12,)
    }
};
        locals.var_qinv = assign30200_e23928;
        locals.var_qinv_dn3 = assign30200_e23928_d_n3;
        locals.var_qinv_dn4 = assign30200_e23928_d_n4;
        locals.var_qinv_dn5 = assign30200_e23928_d_n5;
        locals.var_qinv_dn6 = assign30200_e23928_d_n6;
        locals.var_qinv_dn7 = assign30200_e23928_d_n7;
        locals.var_qinv_dn8 = assign30200_e23928_d_n8;
        locals.var_qinv_dn9 = assign30200_e23928_d_n9;
        locals.var_qinv_dn10 = assign30200_e23928_d_n10;
        locals.var_qinv_dn11 = assign30200_e23928_d_n11;
        locals.var_qinv_dn12 = assign30200_e23928_d_n12;

        let assign30220_e23938: f64 = if locals.var_b4soixpart > 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1711 = assign30220_e23938;

        let (assign30230_e23946, assign30230_e23946_d_n3, assign30230_e23946_d_n4, assign30230_e23946_d_n5, assign30230_e23946_d_n6, assign30230_e23946_d_n7, assign30230_e23946_d_n8, assign30230_e23946_d_n9, assign30230_e23946_d_n10, assign30230_e23946_d_n11, assign30230_e23946_d_n12,) = {
    if ((locals.var_guard1698 != 0.0) && (locals.var_guard1711 != 0.0)) {
        let assign30230_e23944: f64 = (locals.var_t1__blk1145 + locals.var_t1__blk1145);
        (assign30230_e23944, (locals.var_t1__blk1145_dn3 + locals.var_t1__blk1145_dn3), (locals.var_t1__blk1145_dn4 + locals.var_t1__blk1145_dn4), (locals.var_t1__blk1145_dn5 + locals.var_t1__blk1145_dn5), (locals.var_t1__blk1145_dn6 + locals.var_t1__blk1145_dn6), (locals.var_t1__blk1145_dn7 + locals.var_t1__blk1145_dn7), (locals.var_t1__blk1145_dn8 + locals.var_t1__blk1145_dn8), (locals.var_t1__blk1145_dn9 + locals.var_t1__blk1145_dn9), (locals.var_t1__blk1145_dn10 + locals.var_t1__blk1145_dn10), (locals.var_t1__blk1145_dn11 + locals.var_t1__blk1145_dn11), (locals.var_t1__blk1145_dn12 + locals.var_t1__blk1145_dn12),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign30230_e23946;
        locals.var_t1__blk1145_dn3 = assign30230_e23946_d_n3;
        locals.var_t1__blk1145_dn4 = assign30230_e23946_d_n4;
        locals.var_t1__blk1145_dn5 = assign30230_e23946_d_n5;
        locals.var_t1__blk1145_dn6 = assign30230_e23946_d_n6;
        locals.var_t1__blk1145_dn7 = assign30230_e23946_d_n7;
        locals.var_t1__blk1145_dn8 = assign30230_e23946_d_n8;
        locals.var_t1__blk1145_dn9 = assign30230_e23946_d_n9;
        locals.var_t1__blk1145_dn10 = assign30230_e23946_d_n10;
        locals.var_t1__blk1145_dn11 = assign30230_e23946_d_n11;
        locals.var_t1__blk1145_dn12 = assign30230_e23946_d_n12;

        let (assign30240_e23967, assign30240_e23967_d_n3, assign30240_e23967_d_n4, assign30240_e23967_d_n5, assign30240_e23967_d_n6, assign30240_e23967_d_n7, assign30240_e23967_d_n8, assign30240_e23967_d_n9, assign30240_e23967_d_n10, assign30240_e23967_d_n11, assign30240_e23967_d_n12,) = {
    if ((locals.var_guard1698 != 0.0) && (locals.var_guard1711 != 0.0)) {
        let assign30240_e23951: f64 = (-locals.var_coxwl);
        let assign30240_e23954: f64 = (0.5 * locals.var_vgsteff__blk1175);
        let assign30240_e23957: f64 = (0.25 * locals.var_t0__blk1144);
        let assign30240_e23958: f64 = (assign30240_e23954 + assign30240_e23957);
        let assign30240_e23961: f64 = (locals.var_t0__blk1144 * locals.var_t0__blk1144);
        let assign30240_e23963: f64 = (assign30240_e23961 / locals.var_t1__blk1145);
        let assign30240_e23964: f64 = (assign30240_e23958 - assign30240_e23963);
        let assign30240_e23965: f64 = (assign30240_e23951 * assign30240_e23964);
        (assign30240_e23965, (((-locals.var_coxwl_dn3) * assign30240_e23964) + (assign30240_e23951 * (((0.5 * locals.var_vgsteff__blk1175_dn3) + (0.25 * locals.var_t0__blk1144_dn3)) - (((((locals.var_t0__blk1144_dn3 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn3)) * locals.var_t1__blk1145) - (assign30240_e23961 * locals.var_t1__blk1145_dn3)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))))), (((-locals.var_coxwl_dn4) * assign30240_e23964) + (assign30240_e23951 * (((0.5 * locals.var_vgsteff__blk1175_dn4) + (0.25 * locals.var_t0__blk1144_dn4)) - (((((locals.var_t0__blk1144_dn4 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn4)) * locals.var_t1__blk1145) - (assign30240_e23961 * locals.var_t1__blk1145_dn4)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))))), (((-locals.var_coxwl_dn5) * assign30240_e23964) + (assign30240_e23951 * (((0.5 * locals.var_vgsteff__blk1175_dn5) + (0.25 * locals.var_t0__blk1144_dn5)) - (((((locals.var_t0__blk1144_dn5 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn5)) * locals.var_t1__blk1145) - (assign30240_e23961 * locals.var_t1__blk1145_dn5)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))))), (((-locals.var_coxwl_dn6) * assign30240_e23964) + (assign30240_e23951 * (((0.5 * locals.var_vgsteff__blk1175_dn6) + (0.25 * locals.var_t0__blk1144_dn6)) - (((((locals.var_t0__blk1144_dn6 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn6)) * locals.var_t1__blk1145) - (assign30240_e23961 * locals.var_t1__blk1145_dn6)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))))), (((-locals.var_coxwl_dn7) * assign30240_e23964) + (assign30240_e23951 * (((0.5 * locals.var_vgsteff__blk1175_dn7) + (0.25 * locals.var_t0__blk1144_dn7)) - (((((locals.var_t0__blk1144_dn7 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn7)) * locals.var_t1__blk1145) - (assign30240_e23961 * locals.var_t1__blk1145_dn7)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))))), (((-locals.var_coxwl_dn8) * assign30240_e23964) + (assign30240_e23951 * (((0.5 * locals.var_vgsteff__blk1175_dn8) + (0.25 * locals.var_t0__blk1144_dn8)) - (((((locals.var_t0__blk1144_dn8 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn8)) * locals.var_t1__blk1145) - (assign30240_e23961 * locals.var_t1__blk1145_dn8)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))))), (((-locals.var_coxwl_dn9) * assign30240_e23964) + (assign30240_e23951 * (((0.5 * locals.var_vgsteff__blk1175_dn9) + (0.25 * locals.var_t0__blk1144_dn9)) - (((((locals.var_t0__blk1144_dn9 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn9)) * locals.var_t1__blk1145) - (assign30240_e23961 * locals.var_t1__blk1145_dn9)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))))), (((-locals.var_coxwl_dn10) * assign30240_e23964) + (assign30240_e23951 * (((0.5 * locals.var_vgsteff__blk1175_dn10) + (0.25 * locals.var_t0__blk1144_dn10)) - (((((locals.var_t0__blk1144_dn10 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn10)) * locals.var_t1__blk1145) - (assign30240_e23961 * locals.var_t1__blk1145_dn10)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))))), (((-locals.var_coxwl_dn11) * assign30240_e23964) + (assign30240_e23951 * (((0.5 * locals.var_vgsteff__blk1175_dn11) + (0.25 * locals.var_t0__blk1144_dn11)) - (((((locals.var_t0__blk1144_dn11 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn11)) * locals.var_t1__blk1145) - (assign30240_e23961 * locals.var_t1__blk1145_dn11)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))))), (((-locals.var_coxwl_dn12) * assign30240_e23964) + (assign30240_e23951 * (((0.5 * locals.var_vgsteff__blk1175_dn12) + (0.25 * locals.var_t0__blk1144_dn12)) - (((((locals.var_t0__blk1144_dn12 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn12)) * locals.var_t1__blk1145) - (assign30240_e23961 * locals.var_t1__blk1145_dn12)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))))),)
    } else {
        (locals.var_qsrc, locals.var_qsrc_dn3, locals.var_qsrc_dn4, locals.var_qsrc_dn5, locals.var_qsrc_dn6, locals.var_qsrc_dn7, locals.var_qsrc_dn8, locals.var_qsrc_dn9, locals.var_qsrc_dn10, locals.var_qsrc_dn11, locals.var_qsrc_dn12,)
    }
};
        locals.var_qsrc = assign30240_e23967;
        locals.var_qsrc_dn3 = assign30240_e23967_d_n3;
        locals.var_qsrc_dn4 = assign30240_e23967_d_n4;
        locals.var_qsrc_dn5 = assign30240_e23967_d_n5;
        locals.var_qsrc_dn6 = assign30240_e23967_d_n6;
        locals.var_qsrc_dn7 = assign30240_e23967_d_n7;
        locals.var_qsrc_dn8 = assign30240_e23967_d_n8;
        locals.var_qsrc_dn9 = assign30240_e23967_d_n9;
        locals.var_qsrc_dn10 = assign30240_e23967_d_n10;
        locals.var_qsrc_dn11 = assign30240_e23967_d_n11;
        locals.var_qsrc_dn12 = assign30240_e23967_d_n12;

        let assign30250_e23978: f64 = if (((locals.var_b4soisoimod != 2.0) && (locals.var_b4soibodymod != 0.0)) && (locals.var_b4soiagbcp2 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1712 = assign30250_e23978;

        let (assign30260_e23988, assign30260_e23988_d_n3, assign30260_e23988_d_n4, assign30260_e23988_d_n5, assign30260_e23988_d_n6, assign30260_e23988_d_n7, assign30260_e23988_d_n8, assign30260_e23988_d_n9, assign30260_e23988_d_n10, assign30260_e23988_d_n11, assign30260_e23988_d_n12,) = {
    if (((locals.var_guard1698 != 0.0) && (locals.var_guard1711 != 0.0)) && (locals.var_guard1712 != 0.0)) {
        let assign30260_e23986: f64 = (locals.var_t12 + locals.var_t12);
        (assign30260_e23986, (locals.var_t12_dn3 + locals.var_t12_dn3), (locals.var_t12_dn4 + locals.var_t12_dn4), (locals.var_t12_dn5 + locals.var_t12_dn5), (locals.var_t12_dn6 + locals.var_t12_dn6), (locals.var_t12_dn7 + locals.var_t12_dn7), (locals.var_t12_dn8 + locals.var_t12_dn8), (locals.var_t12_dn9 + locals.var_t12_dn9), (locals.var_t12_dn10 + locals.var_t12_dn10), (locals.var_t12_dn11 + locals.var_t12_dn11), (locals.var_t12_dn12 + locals.var_t12_dn12),)
    } else {
        (locals.var_t12, locals.var_t12_dn3, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn12,)
    }
};
        locals.var_t12 = assign30260_e23988;
        locals.var_t12_dn3 = assign30260_e23988_d_n3;
        locals.var_t12_dn4 = assign30260_e23988_d_n4;
        locals.var_t12_dn5 = assign30260_e23988_d_n5;
        locals.var_t12_dn6 = assign30260_e23988_d_n6;
        locals.var_t12_dn7 = assign30260_e23988_d_n7;
        locals.var_t12_dn8 = assign30260_e23988_d_n8;
        locals.var_t12_dn9 = assign30260_e23988_d_n9;
        locals.var_t12_dn10 = assign30260_e23988_d_n10;
        locals.var_t12_dn11 = assign30260_e23988_d_n11;
        locals.var_t12_dn12 = assign30260_e23988_d_n12;

        let (assign30270_e24012, assign30270_e24012_d_n3, assign30270_e24012_d_n4, assign30270_e24012_d_n5, assign30270_e24012_d_n6, assign30270_e24012_d_n7, assign30270_e24012_d_n8, assign30270_e24012_d_n9, assign30270_e24012_d_n10, assign30270_e24012_d_n11, assign30270_e24012_d_n12,) = {
    if (((locals.var_guard1698 != 0.0) && (locals.var_guard1711 != 0.0)) && (locals.var_guard1712 != 0.0)) {
        let assign30270_e23998: f64 = (0.5 * locals.var_vgsteff2);
        let assign30270_e24001: f64 = (0.25 * locals.var_t02);
        let assign30270_e24002: f64 = (assign30270_e23998 + assign30270_e24001);
        let assign30270_e24005: f64 = (locals.var_t02 * locals.var_t02);
        let assign30270_e24007: f64 = (assign30270_e24005 / locals.var_t12);
        let assign30270_e24008: f64 = (assign30270_e24002 - assign30270_e24007);
        let assign30270_e24009: f64 = (locals.var_coxwl2 * assign30270_e24008);
        let assign30270_e24010: f64 = (locals.var_qsrc - assign30270_e24009);
        (assign30270_e24010, (locals.var_qsrc_dn3 - ((locals.var_coxwl2_dn3 * assign30270_e24008) + (locals.var_coxwl2 * (((0.5 * locals.var_vgsteff2_dn3) + (0.25 * locals.var_t02_dn3)) - (((((locals.var_t02_dn3 * locals.var_t02) + (locals.var_t02 * locals.var_t02_dn3)) * locals.var_t12) - (assign30270_e24005 * locals.var_t12_dn3)) / (locals.var_t12 * locals.var_t12)))))), (locals.var_qsrc_dn4 - ((locals.var_coxwl2_dn4 * assign30270_e24008) + (locals.var_coxwl2 * (((0.5 * locals.var_vgsteff2_dn4) + (0.25 * locals.var_t02_dn4)) - (((((locals.var_t02_dn4 * locals.var_t02) + (locals.var_t02 * locals.var_t02_dn4)) * locals.var_t12) - (assign30270_e24005 * locals.var_t12_dn4)) / (locals.var_t12 * locals.var_t12)))))), (locals.var_qsrc_dn5 - ((locals.var_coxwl2_dn5 * assign30270_e24008) + (locals.var_coxwl2 * (((0.5 * locals.var_vgsteff2_dn5) + (0.25 * locals.var_t02_dn5)) - (((((locals.var_t02_dn5 * locals.var_t02) + (locals.var_t02 * locals.var_t02_dn5)) * locals.var_t12) - (assign30270_e24005 * locals.var_t12_dn5)) / (locals.var_t12 * locals.var_t12)))))), (locals.var_qsrc_dn6 - ((locals.var_coxwl2_dn6 * assign30270_e24008) + (locals.var_coxwl2 * (((0.5 * locals.var_vgsteff2_dn6) + (0.25 * locals.var_t02_dn6)) - (((((locals.var_t02_dn6 * locals.var_t02) + (locals.var_t02 * locals.var_t02_dn6)) * locals.var_t12) - (assign30270_e24005 * locals.var_t12_dn6)) / (locals.var_t12 * locals.var_t12)))))), (locals.var_qsrc_dn7 - ((locals.var_coxwl2_dn7 * assign30270_e24008) + (locals.var_coxwl2 * (((0.5 * locals.var_vgsteff2_dn7) + (0.25 * locals.var_t02_dn7)) - (((((locals.var_t02_dn7 * locals.var_t02) + (locals.var_t02 * locals.var_t02_dn7)) * locals.var_t12) - (assign30270_e24005 * locals.var_t12_dn7)) / (locals.var_t12 * locals.var_t12)))))), (locals.var_qsrc_dn8 - ((locals.var_coxwl2_dn8 * assign30270_e24008) + (locals.var_coxwl2 * (((0.5 * locals.var_vgsteff2_dn8) + (0.25 * locals.var_t02_dn8)) - (((((locals.var_t02_dn8 * locals.var_t02) + (locals.var_t02 * locals.var_t02_dn8)) * locals.var_t12) - (assign30270_e24005 * locals.var_t12_dn8)) / (locals.var_t12 * locals.var_t12)))))), (locals.var_qsrc_dn9 - ((locals.var_coxwl2_dn9 * assign30270_e24008) + (locals.var_coxwl2 * (((0.5 * locals.var_vgsteff2_dn9) + (0.25 * locals.var_t02_dn9)) - (((((locals.var_t02_dn9 * locals.var_t02) + (locals.var_t02 * locals.var_t02_dn9)) * locals.var_t12) - (assign30270_e24005 * locals.var_t12_dn9)) / (locals.var_t12 * locals.var_t12)))))), (locals.var_qsrc_dn10 - ((locals.var_coxwl2_dn10 * assign30270_e24008) + (locals.var_coxwl2 * (((0.5 * locals.var_vgsteff2_dn10) + (0.25 * locals.var_t02_dn10)) - (((((locals.var_t02_dn10 * locals.var_t02) + (locals.var_t02 * locals.var_t02_dn10)) * locals.var_t12) - (assign30270_e24005 * locals.var_t12_dn10)) / (locals.var_t12 * locals.var_t12)))))), (locals.var_qsrc_dn11 - ((locals.var_coxwl2_dn11 * assign30270_e24008) + (locals.var_coxwl2 * (((0.5 * locals.var_vgsteff2_dn11) + (0.25 * locals.var_t02_dn11)) - (((((locals.var_t02_dn11 * locals.var_t02) + (locals.var_t02 * locals.var_t02_dn11)) * locals.var_t12) - (assign30270_e24005 * locals.var_t12_dn11)) / (locals.var_t12 * locals.var_t12)))))), (locals.var_qsrc_dn12 - ((locals.var_coxwl2_dn12 * assign30270_e24008) + (locals.var_coxwl2 * (((0.5 * locals.var_vgsteff2_dn12) + (0.25 * locals.var_t02_dn12)) - (((((locals.var_t02_dn12 * locals.var_t02) + (locals.var_t02 * locals.var_t02_dn12)) * locals.var_t12) - (assign30270_e24005 * locals.var_t12_dn12)) / (locals.var_t12 * locals.var_t12)))))),)
    } else {
        (locals.var_qsrc, locals.var_qsrc_dn3, locals.var_qsrc_dn4, locals.var_qsrc_dn5, locals.var_qsrc_dn6, locals.var_qsrc_dn7, locals.var_qsrc_dn8, locals.var_qsrc_dn9, locals.var_qsrc_dn10, locals.var_qsrc_dn11, locals.var_qsrc_dn12,)
    }
};
        locals.var_qsrc = assign30270_e24012;
        locals.var_qsrc_dn3 = assign30270_e24012_d_n3;
        locals.var_qsrc_dn4 = assign30270_e24012_d_n4;
        locals.var_qsrc_dn5 = assign30270_e24012_d_n5;
        locals.var_qsrc_dn6 = assign30270_e24012_d_n6;
        locals.var_qsrc_dn7 = assign30270_e24012_d_n7;
        locals.var_qsrc_dn8 = assign30270_e24012_d_n8;
        locals.var_qsrc_dn9 = assign30270_e24012_d_n9;
        locals.var_qsrc_dn10 = assign30270_e24012_d_n10;
        locals.var_qsrc_dn11 = assign30270_e24012_d_n11;
        locals.var_qsrc_dn12 = assign30270_e24012_d_n12;

        let assign30280_e24015: f64 = if locals.var_b4soixpart < 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1713 = assign30280_e24015;

        let (assign30290_e24026, assign30290_e24026_d_n3, assign30290_e24026_d_n4, assign30290_e24026_d_n5, assign30290_e24026_d_n6, assign30290_e24026_d_n7, assign30290_e24026_d_n8, assign30290_e24026_d_n9, assign30290_e24026_d_n10, assign30290_e24026_d_n11, assign30290_e24026_d_n12,) = {
    if (((locals.var_guard1698 != 0.0) && (locals.var_guard1711 == 0.0)) && (locals.var_guard1713 != 0.0)) {
        let assign30290_e24024: f64 = (locals.var_t1__blk1145 / 12.0);
        (assign30290_e24024, (locals.var_t1__blk1145_dn3 / 12.0), (locals.var_t1__blk1145_dn4 / 12.0), (locals.var_t1__blk1145_dn5 / 12.0), (locals.var_t1__blk1145_dn6 / 12.0), (locals.var_t1__blk1145_dn7 / 12.0), (locals.var_t1__blk1145_dn8 / 12.0), (locals.var_t1__blk1145_dn9 / 12.0), (locals.var_t1__blk1145_dn10 / 12.0), (locals.var_t1__blk1145_dn11 / 12.0), (locals.var_t1__blk1145_dn12 / 12.0),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign30290_e24026;
        locals.var_t1__blk1145_dn3 = assign30290_e24026_d_n3;
        locals.var_t1__blk1145_dn4 = assign30290_e24026_d_n4;
        locals.var_t1__blk1145_dn5 = assign30290_e24026_d_n5;
        locals.var_t1__blk1145_dn6 = assign30290_e24026_d_n6;
        locals.var_t1__blk1145_dn7 = assign30290_e24026_d_n7;
        locals.var_t1__blk1145_dn8 = assign30290_e24026_d_n8;
        locals.var_t1__blk1145_dn9 = assign30290_e24026_d_n9;
        locals.var_t1__blk1145_dn10 = assign30290_e24026_d_n10;
        locals.var_t1__blk1145_dn11 = assign30290_e24026_d_n11;
        locals.var_t1__blk1145_dn12 = assign30290_e24026_d_n12;

        let (assign30300_e24041, assign30300_e24041_d_n3, assign30300_e24041_d_n4, assign30300_e24041_d_n5, assign30300_e24041_d_n6, assign30300_e24041_d_n7, assign30300_e24041_d_n8, assign30300_e24041_d_n9, assign30300_e24041_d_n10, assign30300_e24041_d_n11, assign30300_e24041_d_n12,) = {
    if (((locals.var_guard1698 != 0.0) && (locals.var_guard1711 == 0.0)) && (locals.var_guard1713 != 0.0)) {
        let assign30300_e24035: f64 = (0.5 * locals.var_coxwl);
        let assign30300_e24038: f64 = (locals.var_t1__blk1145 * locals.var_t1__blk1145);
        let assign30300_e24039: f64 = (assign30300_e24035 / assign30300_e24038);
        (assign30300_e24039, ((((0.5 * locals.var_coxwl_dn3) * assign30300_e24038) - (assign30300_e24035 * ((locals.var_t1__blk1145_dn3 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn3)))) / (assign30300_e24038 * assign30300_e24038)), ((((0.5 * locals.var_coxwl_dn4) * assign30300_e24038) - (assign30300_e24035 * ((locals.var_t1__blk1145_dn4 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn4)))) / (assign30300_e24038 * assign30300_e24038)), ((((0.5 * locals.var_coxwl_dn5) * assign30300_e24038) - (assign30300_e24035 * ((locals.var_t1__blk1145_dn5 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn5)))) / (assign30300_e24038 * assign30300_e24038)), ((((0.5 * locals.var_coxwl_dn6) * assign30300_e24038) - (assign30300_e24035 * ((locals.var_t1__blk1145_dn6 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn6)))) / (assign30300_e24038 * assign30300_e24038)), ((((0.5 * locals.var_coxwl_dn7) * assign30300_e24038) - (assign30300_e24035 * ((locals.var_t1__blk1145_dn7 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn7)))) / (assign30300_e24038 * assign30300_e24038)), ((((0.5 * locals.var_coxwl_dn8) * assign30300_e24038) - (assign30300_e24035 * ((locals.var_t1__blk1145_dn8 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn8)))) / (assign30300_e24038 * assign30300_e24038)), ((((0.5 * locals.var_coxwl_dn9) * assign30300_e24038) - (assign30300_e24035 * ((locals.var_t1__blk1145_dn9 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn9)))) / (assign30300_e24038 * assign30300_e24038)), ((((0.5 * locals.var_coxwl_dn10) * assign30300_e24038) - (assign30300_e24035 * ((locals.var_t1__blk1145_dn10 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn10)))) / (assign30300_e24038 * assign30300_e24038)), ((((0.5 * locals.var_coxwl_dn11) * assign30300_e24038) - (assign30300_e24035 * ((locals.var_t1__blk1145_dn11 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn11)))) / (assign30300_e24038 * assign30300_e24038)), ((((0.5 * locals.var_coxwl_dn12) * assign30300_e24038) - (assign30300_e24035 * ((locals.var_t1__blk1145_dn12 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn12)))) / (assign30300_e24038 * assign30300_e24038)),)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign30300_e24041;
        locals.var_t2__blk1146_dn3 = assign30300_e24041_d_n3;
        locals.var_t2__blk1146_dn4 = assign30300_e24041_d_n4;
        locals.var_t2__blk1146_dn5 = assign30300_e24041_d_n5;
        locals.var_t2__blk1146_dn6 = assign30300_e24041_d_n6;
        locals.var_t2__blk1146_dn7 = assign30300_e24041_d_n7;
        locals.var_t2__blk1146_dn8 = assign30300_e24041_d_n8;
        locals.var_t2__blk1146_dn9 = assign30300_e24041_d_n9;
        locals.var_t2__blk1146_dn10 = assign30300_e24041_d_n10;
        locals.var_t2__blk1146_dn11 = assign30300_e24041_d_n11;
        locals.var_t2__blk1146_dn12 = assign30300_e24041_d_n12;

        let (assign30310_e24078, assign30310_e24078_d_n3, assign30310_e24078_d_n4, assign30310_e24078_d_n5, assign30310_e24078_d_n6, assign30310_e24078_d_n7, assign30310_e24078_d_n8, assign30310_e24078_d_n9, assign30310_e24078_d_n10, assign30310_e24078_d_n11, assign30310_e24078_d_n12,) = {
    if (((locals.var_guard1698 != 0.0) && (locals.var_guard1711 == 0.0)) && (locals.var_guard1713 != 0.0)) {
        let assign30310_e24051: f64 = (2.0 * locals.var_t0__blk1144);
        let assign30310_e24053: f64 = (assign30310_e24051 * locals.var_t0__blk1144);
        let assign30310_e24055: f64 = (assign30310_e24053 / 3.0);
        let assign30310_e24060: f64 = (4.0 * locals.var_t0__blk1144);
        let assign30310_e24062: f64 = (assign30310_e24060 / 3.0);
        let assign30310_e24063: f64 = (locals.var_vgsteff__blk1175 - assign30310_e24062);
        let assign30310_e24064: f64 = (locals.var_vgsteff__blk1175 * assign30310_e24063);
        let assign30310_e24065: f64 = (assign30310_e24055 + assign30310_e24064);
        let assign30310_e24066: f64 = (locals.var_vgsteff__blk1175 * assign30310_e24065);
        let assign30310_e24069: f64 = (2.0 * locals.var_t0__blk1144);
        let assign30310_e24071: f64 = (assign30310_e24069 * locals.var_t0__blk1144);
        let assign30310_e24073: f64 = (assign30310_e24071 * locals.var_t0__blk1144);
        let assign30310_e24075: f64 = (assign30310_e24073 / 15.0);
        let assign30310_e24076: f64 = (assign30310_e24066 - assign30310_e24075);
        (assign30310_e24076, (((locals.var_vgsteff__blk1175_dn3 * assign30310_e24065) + (locals.var_vgsteff__blk1175 * (((((2.0 * locals.var_t0__blk1144_dn3) * locals.var_t0__blk1144) + (assign30310_e24051 * locals.var_t0__blk1144_dn3)) / 3.0) + ((locals.var_vgsteff__blk1175_dn3 * assign30310_e24063) + (locals.var_vgsteff__blk1175 * (locals.var_vgsteff__blk1175_dn3 - ((4.0 * locals.var_t0__blk1144_dn3) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk1144_dn3) * locals.var_t0__blk1144) + (assign30310_e24069 * locals.var_t0__blk1144_dn3)) * locals.var_t0__blk1144) + (assign30310_e24071 * locals.var_t0__blk1144_dn3)) / 15.0)), (((locals.var_vgsteff__blk1175_dn4 * assign30310_e24065) + (locals.var_vgsteff__blk1175 * (((((2.0 * locals.var_t0__blk1144_dn4) * locals.var_t0__blk1144) + (assign30310_e24051 * locals.var_t0__blk1144_dn4)) / 3.0) + ((locals.var_vgsteff__blk1175_dn4 * assign30310_e24063) + (locals.var_vgsteff__blk1175 * (locals.var_vgsteff__blk1175_dn4 - ((4.0 * locals.var_t0__blk1144_dn4) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk1144_dn4) * locals.var_t0__blk1144) + (assign30310_e24069 * locals.var_t0__blk1144_dn4)) * locals.var_t0__blk1144) + (assign30310_e24071 * locals.var_t0__blk1144_dn4)) / 15.0)), (((locals.var_vgsteff__blk1175_dn5 * assign30310_e24065) + (locals.var_vgsteff__blk1175 * (((((2.0 * locals.var_t0__blk1144_dn5) * locals.var_t0__blk1144) + (assign30310_e24051 * locals.var_t0__blk1144_dn5)) / 3.0) + ((locals.var_vgsteff__blk1175_dn5 * assign30310_e24063) + (locals.var_vgsteff__blk1175 * (locals.var_vgsteff__blk1175_dn5 - ((4.0 * locals.var_t0__blk1144_dn5) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk1144_dn5) * locals.var_t0__blk1144) + (assign30310_e24069 * locals.var_t0__blk1144_dn5)) * locals.var_t0__blk1144) + (assign30310_e24071 * locals.var_t0__blk1144_dn5)) / 15.0)), (((locals.var_vgsteff__blk1175_dn6 * assign30310_e24065) + (locals.var_vgsteff__blk1175 * (((((2.0 * locals.var_t0__blk1144_dn6) * locals.var_t0__blk1144) + (assign30310_e24051 * locals.var_t0__blk1144_dn6)) / 3.0) + ((locals.var_vgsteff__blk1175_dn6 * assign30310_e24063) + (locals.var_vgsteff__blk1175 * (locals.var_vgsteff__blk1175_dn6 - ((4.0 * locals.var_t0__blk1144_dn6) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk1144_dn6) * locals.var_t0__blk1144) + (assign30310_e24069 * locals.var_t0__blk1144_dn6)) * locals.var_t0__blk1144) + (assign30310_e24071 * locals.var_t0__blk1144_dn6)) / 15.0)), (((locals.var_vgsteff__blk1175_dn7 * assign30310_e24065) + (locals.var_vgsteff__blk1175 * (((((2.0 * locals.var_t0__blk1144_dn7) * locals.var_t0__blk1144) + (assign30310_e24051 * locals.var_t0__blk1144_dn7)) / 3.0) + ((locals.var_vgsteff__blk1175_dn7 * assign30310_e24063) + (locals.var_vgsteff__blk1175 * (locals.var_vgsteff__blk1175_dn7 - ((4.0 * locals.var_t0__blk1144_dn7) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk1144_dn7) * locals.var_t0__blk1144) + (assign30310_e24069 * locals.var_t0__blk1144_dn7)) * locals.var_t0__blk1144) + (assign30310_e24071 * locals.var_t0__blk1144_dn7)) / 15.0)), (((locals.var_vgsteff__blk1175_dn8 * assign30310_e24065) + (locals.var_vgsteff__blk1175 * (((((2.0 * locals.var_t0__blk1144_dn8) * locals.var_t0__blk1144) + (assign30310_e24051 * locals.var_t0__blk1144_dn8)) / 3.0) + ((locals.var_vgsteff__blk1175_dn8 * assign30310_e24063) + (locals.var_vgsteff__blk1175 * (locals.var_vgsteff__blk1175_dn8 - ((4.0 * locals.var_t0__blk1144_dn8) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk1144_dn8) * locals.var_t0__blk1144) + (assign30310_e24069 * locals.var_t0__blk1144_dn8)) * locals.var_t0__blk1144) + (assign30310_e24071 * locals.var_t0__blk1144_dn8)) / 15.0)), (((locals.var_vgsteff__blk1175_dn9 * assign30310_e24065) + (locals.var_vgsteff__blk1175 * (((((2.0 * locals.var_t0__blk1144_dn9) * locals.var_t0__blk1144) + (assign30310_e24051 * locals.var_t0__blk1144_dn9)) / 3.0) + ((locals.var_vgsteff__blk1175_dn9 * assign30310_e24063) + (locals.var_vgsteff__blk1175 * (locals.var_vgsteff__blk1175_dn9 - ((4.0 * locals.var_t0__blk1144_dn9) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk1144_dn9) * locals.var_t0__blk1144) + (assign30310_e24069 * locals.var_t0__blk1144_dn9)) * locals.var_t0__blk1144) + (assign30310_e24071 * locals.var_t0__blk1144_dn9)) / 15.0)), (((locals.var_vgsteff__blk1175_dn10 * assign30310_e24065) + (locals.var_vgsteff__blk1175 * (((((2.0 * locals.var_t0__blk1144_dn10) * locals.var_t0__blk1144) + (assign30310_e24051 * locals.var_t0__blk1144_dn10)) / 3.0) + ((locals.var_vgsteff__blk1175_dn10 * assign30310_e24063) + (locals.var_vgsteff__blk1175 * (locals.var_vgsteff__blk1175_dn10 - ((4.0 * locals.var_t0__blk1144_dn10) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk1144_dn10) * locals.var_t0__blk1144) + (assign30310_e24069 * locals.var_t0__blk1144_dn10)) * locals.var_t0__blk1144) + (assign30310_e24071 * locals.var_t0__blk1144_dn10)) / 15.0)), (((locals.var_vgsteff__blk1175_dn11 * assign30310_e24065) + (locals.var_vgsteff__blk1175 * (((((2.0 * locals.var_t0__blk1144_dn11) * locals.var_t0__blk1144) + (assign30310_e24051 * locals.var_t0__blk1144_dn11)) / 3.0) + ((locals.var_vgsteff__blk1175_dn11 * assign30310_e24063) + (locals.var_vgsteff__blk1175 * (locals.var_vgsteff__blk1175_dn11 - ((4.0 * locals.var_t0__blk1144_dn11) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk1144_dn11) * locals.var_t0__blk1144) + (assign30310_e24069 * locals.var_t0__blk1144_dn11)) * locals.var_t0__blk1144) + (assign30310_e24071 * locals.var_t0__blk1144_dn11)) / 15.0)), (((locals.var_vgsteff__blk1175_dn12 * assign30310_e24065) + (locals.var_vgsteff__blk1175 * (((((2.0 * locals.var_t0__blk1144_dn12) * locals.var_t0__blk1144) + (assign30310_e24051 * locals.var_t0__blk1144_dn12)) / 3.0) + ((locals.var_vgsteff__blk1175_dn12 * assign30310_e24063) + (locals.var_vgsteff__blk1175 * (locals.var_vgsteff__blk1175_dn12 - ((4.0 * locals.var_t0__blk1144_dn12) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk1144_dn12) * locals.var_t0__blk1144) + (assign30310_e24069 * locals.var_t0__blk1144_dn12)) * locals.var_t0__blk1144) + (assign30310_e24071 * locals.var_t0__blk1144_dn12)) / 15.0)),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign30310_e24078;
        locals.var_t3__blk1147_dn3 = assign30310_e24078_d_n3;
        locals.var_t3__blk1147_dn4 = assign30310_e24078_d_n4;
        locals.var_t3__blk1147_dn5 = assign30310_e24078_d_n5;
        locals.var_t3__blk1147_dn6 = assign30310_e24078_d_n6;
        locals.var_t3__blk1147_dn7 = assign30310_e24078_d_n7;
        locals.var_t3__blk1147_dn8 = assign30310_e24078_d_n8;
        locals.var_t3__blk1147_dn9 = assign30310_e24078_d_n9;
        locals.var_t3__blk1147_dn10 = assign30310_e24078_d_n10;
        locals.var_t3__blk1147_dn11 = assign30310_e24078_d_n11;
        locals.var_t3__blk1147_dn12 = assign30310_e24078_d_n12;

        let (assign30320_e24090, assign30320_e24090_d_n3, assign30320_e24090_d_n4, assign30320_e24090_d_n5, assign30320_e24090_d_n6, assign30320_e24090_d_n7, assign30320_e24090_d_n8, assign30320_e24090_d_n9, assign30320_e24090_d_n10, assign30320_e24090_d_n11, assign30320_e24090_d_n12,) = {
    if (((locals.var_guard1698 != 0.0) && (locals.var_guard1711 == 0.0)) && (locals.var_guard1713 != 0.0)) {
        let assign30320_e24086: f64 = (-locals.var_t2__blk1146);
        let assign30320_e24088: f64 = (assign30320_e24086 * locals.var_t3__blk1147);
        (assign30320_e24088, (((-locals.var_t2__blk1146_dn3) * locals.var_t3__blk1147) + (assign30320_e24086 * locals.var_t3__blk1147_dn3)), (((-locals.var_t2__blk1146_dn4) * locals.var_t3__blk1147) + (assign30320_e24086 * locals.var_t3__blk1147_dn4)), (((-locals.var_t2__blk1146_dn5) * locals.var_t3__blk1147) + (assign30320_e24086 * locals.var_t3__blk1147_dn5)), (((-locals.var_t2__blk1146_dn6) * locals.var_t3__blk1147) + (assign30320_e24086 * locals.var_t3__blk1147_dn6)), (((-locals.var_t2__blk1146_dn7) * locals.var_t3__blk1147) + (assign30320_e24086 * locals.var_t3__blk1147_dn7)), (((-locals.var_t2__blk1146_dn8) * locals.var_t3__blk1147) + (assign30320_e24086 * locals.var_t3__blk1147_dn8)), (((-locals.var_t2__blk1146_dn9) * locals.var_t3__blk1147) + (assign30320_e24086 * locals.var_t3__blk1147_dn9)), (((-locals.var_t2__blk1146_dn10) * locals.var_t3__blk1147) + (assign30320_e24086 * locals.var_t3__blk1147_dn10)), (((-locals.var_t2__blk1146_dn11) * locals.var_t3__blk1147) + (assign30320_e24086 * locals.var_t3__blk1147_dn11)), (((-locals.var_t2__blk1146_dn12) * locals.var_t3__blk1147) + (assign30320_e24086 * locals.var_t3__blk1147_dn12)),)
    } else {
        (locals.var_qsrc, locals.var_qsrc_dn3, locals.var_qsrc_dn4, locals.var_qsrc_dn5, locals.var_qsrc_dn6, locals.var_qsrc_dn7, locals.var_qsrc_dn8, locals.var_qsrc_dn9, locals.var_qsrc_dn10, locals.var_qsrc_dn11, locals.var_qsrc_dn12,)
    }
};
        locals.var_qsrc = assign30320_e24090;
        locals.var_qsrc_dn3 = assign30320_e24090_d_n3;
        locals.var_qsrc_dn4 = assign30320_e24090_d_n4;
        locals.var_qsrc_dn5 = assign30320_e24090_d_n5;
        locals.var_qsrc_dn6 = assign30320_e24090_d_n6;
        locals.var_qsrc_dn7 = assign30320_e24090_d_n7;
        locals.var_qsrc_dn8 = assign30320_e24090_d_n8;
        locals.var_qsrc_dn9 = assign30320_e24090_d_n9;
        locals.var_qsrc_dn10 = assign30320_e24090_d_n10;
        locals.var_qsrc_dn11 = assign30320_e24090_d_n11;
        locals.var_qsrc_dn12 = assign30320_e24090_d_n12;

        let assign30330_e24101: f64 = if (((locals.var_b4soisoimod != 2.0) && (locals.var_b4soibodymod != 0.0)) && (locals.var_b4soiagbcp2 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1714 = assign30330_e24101;

        let (assign30340_e24114, assign30340_e24114_d_n3, assign30340_e24114_d_n4, assign30340_e24114_d_n5, assign30340_e24114_d_n6, assign30340_e24114_d_n7, assign30340_e24114_d_n8, assign30340_e24114_d_n9, assign30340_e24114_d_n10, assign30340_e24114_d_n11, assign30340_e24114_d_n12,) = {
    if ((((locals.var_guard1698 != 0.0) && (locals.var_guard1711 == 0.0)) && (locals.var_guard1713 != 0.0)) && (locals.var_guard1714 != 0.0)) {
        let assign30340_e24112: f64 = (locals.var_t12 / 12.0);
        (assign30340_e24112, (locals.var_t12_dn3 / 12.0), (locals.var_t12_dn4 / 12.0), (locals.var_t12_dn5 / 12.0), (locals.var_t12_dn6 / 12.0), (locals.var_t12_dn7 / 12.0), (locals.var_t12_dn8 / 12.0), (locals.var_t12_dn9 / 12.0), (locals.var_t12_dn10 / 12.0), (locals.var_t12_dn11 / 12.0), (locals.var_t12_dn12 / 12.0),)
    } else {
        (locals.var_t12, locals.var_t12_dn3, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn12,)
    }
};
        locals.var_t12 = assign30340_e24114;
        locals.var_t12_dn3 = assign30340_e24114_d_n3;
        locals.var_t12_dn4 = assign30340_e24114_d_n4;
        locals.var_t12_dn5 = assign30340_e24114_d_n5;
        locals.var_t12_dn6 = assign30340_e24114_d_n6;
        locals.var_t12_dn7 = assign30340_e24114_d_n7;
        locals.var_t12_dn8 = assign30340_e24114_d_n8;
        locals.var_t12_dn9 = assign30340_e24114_d_n9;
        locals.var_t12_dn10 = assign30340_e24114_d_n10;
        locals.var_t12_dn11 = assign30340_e24114_d_n11;
        locals.var_t12_dn12 = assign30340_e24114_d_n12;

    }

    pub(super) fn stamp_transient_block_82(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign30350_e24131, assign30350_e24131_d_n3, assign30350_e24131_d_n4, assign30350_e24131_d_n5, assign30350_e24131_d_n6, assign30350_e24131_d_n7, assign30350_e24131_d_n8, assign30350_e24131_d_n9, assign30350_e24131_d_n10, assign30350_e24131_d_n11, assign30350_e24131_d_n12,) = {
    if ((((locals.var_guard1698 != 0.0) && (locals.var_guard1711 == 0.0)) && (locals.var_guard1713 != 0.0)) && (locals.var_guard1714 != 0.0)) {
        let assign30350_e24125: f64 = (0.5 * locals.var_coxwl2);
        let assign30350_e24128: f64 = (locals.var_t12 * locals.var_t12);
        let assign30350_e24129: f64 = (assign30350_e24125 / assign30350_e24128);
        (assign30350_e24129, ((((0.5 * locals.var_coxwl2_dn3) * assign30350_e24128) - (assign30350_e24125 * ((locals.var_t12_dn3 * locals.var_t12) + (locals.var_t12 * locals.var_t12_dn3)))) / (assign30350_e24128 * assign30350_e24128)), ((((0.5 * locals.var_coxwl2_dn4) * assign30350_e24128) - (assign30350_e24125 * ((locals.var_t12_dn4 * locals.var_t12) + (locals.var_t12 * locals.var_t12_dn4)))) / (assign30350_e24128 * assign30350_e24128)), ((((0.5 * locals.var_coxwl2_dn5) * assign30350_e24128) - (assign30350_e24125 * ((locals.var_t12_dn5 * locals.var_t12) + (locals.var_t12 * locals.var_t12_dn5)))) / (assign30350_e24128 * assign30350_e24128)), ((((0.5 * locals.var_coxwl2_dn6) * assign30350_e24128) - (assign30350_e24125 * ((locals.var_t12_dn6 * locals.var_t12) + (locals.var_t12 * locals.var_t12_dn6)))) / (assign30350_e24128 * assign30350_e24128)), ((((0.5 * locals.var_coxwl2_dn7) * assign30350_e24128) - (assign30350_e24125 * ((locals.var_t12_dn7 * locals.var_t12) + (locals.var_t12 * locals.var_t12_dn7)))) / (assign30350_e24128 * assign30350_e24128)), ((((0.5 * locals.var_coxwl2_dn8) * assign30350_e24128) - (assign30350_e24125 * ((locals.var_t12_dn8 * locals.var_t12) + (locals.var_t12 * locals.var_t12_dn8)))) / (assign30350_e24128 * assign30350_e24128)), ((((0.5 * locals.var_coxwl2_dn9) * assign30350_e24128) - (assign30350_e24125 * ((locals.var_t12_dn9 * locals.var_t12) + (locals.var_t12 * locals.var_t12_dn9)))) / (assign30350_e24128 * assign30350_e24128)), ((((0.5 * locals.var_coxwl2_dn10) * assign30350_e24128) - (assign30350_e24125 * ((locals.var_t12_dn10 * locals.var_t12) + (locals.var_t12 * locals.var_t12_dn10)))) / (assign30350_e24128 * assign30350_e24128)), ((((0.5 * locals.var_coxwl2_dn11) * assign30350_e24128) - (assign30350_e24125 * ((locals.var_t12_dn11 * locals.var_t12) + (locals.var_t12 * locals.var_t12_dn11)))) / (assign30350_e24128 * assign30350_e24128)), ((((0.5 * locals.var_coxwl2_dn12) * assign30350_e24128) - (assign30350_e24125 * ((locals.var_t12_dn12 * locals.var_t12) + (locals.var_t12 * locals.var_t12_dn12)))) / (assign30350_e24128 * assign30350_e24128)),)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign30350_e24131;
        locals.var_t2__blk1146_dn3 = assign30350_e24131_d_n3;
        locals.var_t2__blk1146_dn4 = assign30350_e24131_d_n4;
        locals.var_t2__blk1146_dn5 = assign30350_e24131_d_n5;
        locals.var_t2__blk1146_dn6 = assign30350_e24131_d_n6;
        locals.var_t2__blk1146_dn7 = assign30350_e24131_d_n7;
        locals.var_t2__blk1146_dn8 = assign30350_e24131_d_n8;
        locals.var_t2__blk1146_dn9 = assign30350_e24131_d_n9;
        locals.var_t2__blk1146_dn10 = assign30350_e24131_d_n10;
        locals.var_t2__blk1146_dn11 = assign30350_e24131_d_n11;
        locals.var_t2__blk1146_dn12 = assign30350_e24131_d_n12;

        let (assign30360_e24170, assign30360_e24170_d_n3, assign30360_e24170_d_n4, assign30360_e24170_d_n5, assign30360_e24170_d_n6, assign30360_e24170_d_n7, assign30360_e24170_d_n8, assign30360_e24170_d_n9, assign30360_e24170_d_n10, assign30360_e24170_d_n11, assign30360_e24170_d_n12,) = {
    if ((((locals.var_guard1698 != 0.0) && (locals.var_guard1711 == 0.0)) && (locals.var_guard1713 != 0.0)) && (locals.var_guard1714 != 0.0)) {
        let assign30360_e24143: f64 = (2.0 * locals.var_t02);
        let assign30360_e24145: f64 = (assign30360_e24143 * locals.var_t02);
        let assign30360_e24147: f64 = (assign30360_e24145 / 3.0);
        let assign30360_e24152: f64 = (4.0 * locals.var_t02);
        let assign30360_e24154: f64 = (assign30360_e24152 / 3.0);
        let assign30360_e24155: f64 = (locals.var_vgsteff2 - assign30360_e24154);
        let assign30360_e24156: f64 = (locals.var_vgsteff2 * assign30360_e24155);
        let assign30360_e24157: f64 = (assign30360_e24147 + assign30360_e24156);
        let assign30360_e24158: f64 = (locals.var_vgsteff2 * assign30360_e24157);
        let assign30360_e24161: f64 = (2.0 * locals.var_t02);
        let assign30360_e24163: f64 = (assign30360_e24161 * locals.var_t02);
        let assign30360_e24165: f64 = (assign30360_e24163 * locals.var_t02);
        let assign30360_e24167: f64 = (assign30360_e24165 / 15.0);
        let assign30360_e24168: f64 = (assign30360_e24158 - assign30360_e24167);
        (assign30360_e24168, (((locals.var_vgsteff2_dn3 * assign30360_e24157) + (locals.var_vgsteff2 * (((((2.0 * locals.var_t02_dn3) * locals.var_t02) + (assign30360_e24143 * locals.var_t02_dn3)) / 3.0) + ((locals.var_vgsteff2_dn3 * assign30360_e24155) + (locals.var_vgsteff2 * (locals.var_vgsteff2_dn3 - ((4.0 * locals.var_t02_dn3) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn3) * locals.var_t02) + (assign30360_e24161 * locals.var_t02_dn3)) * locals.var_t02) + (assign30360_e24163 * locals.var_t02_dn3)) / 15.0)), (((locals.var_vgsteff2_dn4 * assign30360_e24157) + (locals.var_vgsteff2 * (((((2.0 * locals.var_t02_dn4) * locals.var_t02) + (assign30360_e24143 * locals.var_t02_dn4)) / 3.0) + ((locals.var_vgsteff2_dn4 * assign30360_e24155) + (locals.var_vgsteff2 * (locals.var_vgsteff2_dn4 - ((4.0 * locals.var_t02_dn4) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn4) * locals.var_t02) + (assign30360_e24161 * locals.var_t02_dn4)) * locals.var_t02) + (assign30360_e24163 * locals.var_t02_dn4)) / 15.0)), (((locals.var_vgsteff2_dn5 * assign30360_e24157) + (locals.var_vgsteff2 * (((((2.0 * locals.var_t02_dn5) * locals.var_t02) + (assign30360_e24143 * locals.var_t02_dn5)) / 3.0) + ((locals.var_vgsteff2_dn5 * assign30360_e24155) + (locals.var_vgsteff2 * (locals.var_vgsteff2_dn5 - ((4.0 * locals.var_t02_dn5) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn5) * locals.var_t02) + (assign30360_e24161 * locals.var_t02_dn5)) * locals.var_t02) + (assign30360_e24163 * locals.var_t02_dn5)) / 15.0)), (((locals.var_vgsteff2_dn6 * assign30360_e24157) + (locals.var_vgsteff2 * (((((2.0 * locals.var_t02_dn6) * locals.var_t02) + (assign30360_e24143 * locals.var_t02_dn6)) / 3.0) + ((locals.var_vgsteff2_dn6 * assign30360_e24155) + (locals.var_vgsteff2 * (locals.var_vgsteff2_dn6 - ((4.0 * locals.var_t02_dn6) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn6) * locals.var_t02) + (assign30360_e24161 * locals.var_t02_dn6)) * locals.var_t02) + (assign30360_e24163 * locals.var_t02_dn6)) / 15.0)), (((locals.var_vgsteff2_dn7 * assign30360_e24157) + (locals.var_vgsteff2 * (((((2.0 * locals.var_t02_dn7) * locals.var_t02) + (assign30360_e24143 * locals.var_t02_dn7)) / 3.0) + ((locals.var_vgsteff2_dn7 * assign30360_e24155) + (locals.var_vgsteff2 * (locals.var_vgsteff2_dn7 - ((4.0 * locals.var_t02_dn7) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn7) * locals.var_t02) + (assign30360_e24161 * locals.var_t02_dn7)) * locals.var_t02) + (assign30360_e24163 * locals.var_t02_dn7)) / 15.0)), (((locals.var_vgsteff2_dn8 * assign30360_e24157) + (locals.var_vgsteff2 * (((((2.0 * locals.var_t02_dn8) * locals.var_t02) + (assign30360_e24143 * locals.var_t02_dn8)) / 3.0) + ((locals.var_vgsteff2_dn8 * assign30360_e24155) + (locals.var_vgsteff2 * (locals.var_vgsteff2_dn8 - ((4.0 * locals.var_t02_dn8) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn8) * locals.var_t02) + (assign30360_e24161 * locals.var_t02_dn8)) * locals.var_t02) + (assign30360_e24163 * locals.var_t02_dn8)) / 15.0)), (((locals.var_vgsteff2_dn9 * assign30360_e24157) + (locals.var_vgsteff2 * (((((2.0 * locals.var_t02_dn9) * locals.var_t02) + (assign30360_e24143 * locals.var_t02_dn9)) / 3.0) + ((locals.var_vgsteff2_dn9 * assign30360_e24155) + (locals.var_vgsteff2 * (locals.var_vgsteff2_dn9 - ((4.0 * locals.var_t02_dn9) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn9) * locals.var_t02) + (assign30360_e24161 * locals.var_t02_dn9)) * locals.var_t02) + (assign30360_e24163 * locals.var_t02_dn9)) / 15.0)), (((locals.var_vgsteff2_dn10 * assign30360_e24157) + (locals.var_vgsteff2 * (((((2.0 * locals.var_t02_dn10) * locals.var_t02) + (assign30360_e24143 * locals.var_t02_dn10)) / 3.0) + ((locals.var_vgsteff2_dn10 * assign30360_e24155) + (locals.var_vgsteff2 * (locals.var_vgsteff2_dn10 - ((4.0 * locals.var_t02_dn10) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn10) * locals.var_t02) + (assign30360_e24161 * locals.var_t02_dn10)) * locals.var_t02) + (assign30360_e24163 * locals.var_t02_dn10)) / 15.0)), (((locals.var_vgsteff2_dn11 * assign30360_e24157) + (locals.var_vgsteff2 * (((((2.0 * locals.var_t02_dn11) * locals.var_t02) + (assign30360_e24143 * locals.var_t02_dn11)) / 3.0) + ((locals.var_vgsteff2_dn11 * assign30360_e24155) + (locals.var_vgsteff2 * (locals.var_vgsteff2_dn11 - ((4.0 * locals.var_t02_dn11) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn11) * locals.var_t02) + (assign30360_e24161 * locals.var_t02_dn11)) * locals.var_t02) + (assign30360_e24163 * locals.var_t02_dn11)) / 15.0)), (((locals.var_vgsteff2_dn12 * assign30360_e24157) + (locals.var_vgsteff2 * (((((2.0 * locals.var_t02_dn12) * locals.var_t02) + (assign30360_e24143 * locals.var_t02_dn12)) / 3.0) + ((locals.var_vgsteff2_dn12 * assign30360_e24155) + (locals.var_vgsteff2 * (locals.var_vgsteff2_dn12 - ((4.0 * locals.var_t02_dn12) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn12) * locals.var_t02) + (assign30360_e24161 * locals.var_t02_dn12)) * locals.var_t02) + (assign30360_e24163 * locals.var_t02_dn12)) / 15.0)),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign30360_e24170;
        locals.var_t3__blk1147_dn3 = assign30360_e24170_d_n3;
        locals.var_t3__blk1147_dn4 = assign30360_e24170_d_n4;
        locals.var_t3__blk1147_dn5 = assign30360_e24170_d_n5;
        locals.var_t3__blk1147_dn6 = assign30360_e24170_d_n6;
        locals.var_t3__blk1147_dn7 = assign30360_e24170_d_n7;
        locals.var_t3__blk1147_dn8 = assign30360_e24170_d_n8;
        locals.var_t3__blk1147_dn9 = assign30360_e24170_d_n9;
        locals.var_t3__blk1147_dn10 = assign30360_e24170_d_n10;
        locals.var_t3__blk1147_dn11 = assign30360_e24170_d_n11;
        locals.var_t3__blk1147_dn12 = assign30360_e24170_d_n12;

        let (assign30370_e24184, assign30370_e24184_d_n3, assign30370_e24184_d_n4, assign30370_e24184_d_n5, assign30370_e24184_d_n6, assign30370_e24184_d_n7, assign30370_e24184_d_n8, assign30370_e24184_d_n9, assign30370_e24184_d_n10, assign30370_e24184_d_n11, assign30370_e24184_d_n12,) = {
    if ((((locals.var_guard1698 != 0.0) && (locals.var_guard1711 == 0.0)) && (locals.var_guard1713 != 0.0)) && (locals.var_guard1714 != 0.0)) {
        let assign30370_e24180: f64 = (-locals.var_t2__blk1146);
        let assign30370_e24182: f64 = (assign30370_e24180 * locals.var_t3__blk1147);
        (assign30370_e24182, (((-locals.var_t2__blk1146_dn3) * locals.var_t3__blk1147) + (assign30370_e24180 * locals.var_t3__blk1147_dn3)), (((-locals.var_t2__blk1146_dn4) * locals.var_t3__blk1147) + (assign30370_e24180 * locals.var_t3__blk1147_dn4)), (((-locals.var_t2__blk1146_dn5) * locals.var_t3__blk1147) + (assign30370_e24180 * locals.var_t3__blk1147_dn5)), (((-locals.var_t2__blk1146_dn6) * locals.var_t3__blk1147) + (assign30370_e24180 * locals.var_t3__blk1147_dn6)), (((-locals.var_t2__blk1146_dn7) * locals.var_t3__blk1147) + (assign30370_e24180 * locals.var_t3__blk1147_dn7)), (((-locals.var_t2__blk1146_dn8) * locals.var_t3__blk1147) + (assign30370_e24180 * locals.var_t3__blk1147_dn8)), (((-locals.var_t2__blk1146_dn9) * locals.var_t3__blk1147) + (assign30370_e24180 * locals.var_t3__blk1147_dn9)), (((-locals.var_t2__blk1146_dn10) * locals.var_t3__blk1147) + (assign30370_e24180 * locals.var_t3__blk1147_dn10)), (((-locals.var_t2__blk1146_dn11) * locals.var_t3__blk1147) + (assign30370_e24180 * locals.var_t3__blk1147_dn11)), (((-locals.var_t2__blk1146_dn12) * locals.var_t3__blk1147) + (assign30370_e24180 * locals.var_t3__blk1147_dn12)),)
    } else {
        (locals.var_qsrc2, locals.var_qsrc2_dn3, locals.var_qsrc2_dn4, locals.var_qsrc2_dn5, locals.var_qsrc2_dn6, locals.var_qsrc2_dn7, locals.var_qsrc2_dn8, locals.var_qsrc2_dn9, locals.var_qsrc2_dn10, locals.var_qsrc2_dn11, locals.var_qsrc2_dn12,)
    }
};
        locals.var_qsrc2 = assign30370_e24184;
        locals.var_qsrc2_dn3 = assign30370_e24184_d_n3;
        locals.var_qsrc2_dn4 = assign30370_e24184_d_n4;
        locals.var_qsrc2_dn5 = assign30370_e24184_d_n5;
        locals.var_qsrc2_dn6 = assign30370_e24184_d_n6;
        locals.var_qsrc2_dn7 = assign30370_e24184_d_n7;
        locals.var_qsrc2_dn8 = assign30370_e24184_d_n8;
        locals.var_qsrc2_dn9 = assign30370_e24184_d_n9;
        locals.var_qsrc2_dn10 = assign30370_e24184_d_n10;
        locals.var_qsrc2_dn11 = assign30370_e24184_d_n11;
        locals.var_qsrc2_dn12 = assign30370_e24184_d_n12;

        let (assign30380_e24197, assign30380_e24197_d_n3, assign30380_e24197_d_n4, assign30380_e24197_d_n5, assign30380_e24197_d_n6, assign30380_e24197_d_n7, assign30380_e24197_d_n8, assign30380_e24197_d_n9, assign30380_e24197_d_n10, assign30380_e24197_d_n11, assign30380_e24197_d_n12,) = {
    if ((((locals.var_guard1698 != 0.0) && (locals.var_guard1711 == 0.0)) && (locals.var_guard1713 != 0.0)) && (locals.var_guard1714 != 0.0)) {
        let assign30380_e24195: f64 = (locals.var_qsrc + locals.var_qsrc2);
        (assign30380_e24195, (locals.var_qsrc_dn3 + locals.var_qsrc2_dn3), (locals.var_qsrc_dn4 + locals.var_qsrc2_dn4), (locals.var_qsrc_dn5 + locals.var_qsrc2_dn5), (locals.var_qsrc_dn6 + locals.var_qsrc2_dn6), (locals.var_qsrc_dn7 + locals.var_qsrc2_dn7), (locals.var_qsrc_dn8 + locals.var_qsrc2_dn8), (locals.var_qsrc_dn9 + locals.var_qsrc2_dn9), (locals.var_qsrc_dn10 + locals.var_qsrc2_dn10), (locals.var_qsrc_dn11 + locals.var_qsrc2_dn11), (locals.var_qsrc_dn12 + locals.var_qsrc2_dn12),)
    } else {
        (locals.var_qsrc, locals.var_qsrc_dn3, locals.var_qsrc_dn4, locals.var_qsrc_dn5, locals.var_qsrc_dn6, locals.var_qsrc_dn7, locals.var_qsrc_dn8, locals.var_qsrc_dn9, locals.var_qsrc_dn10, locals.var_qsrc_dn11, locals.var_qsrc_dn12,)
    }
};
        locals.var_qsrc = assign30380_e24197;
        locals.var_qsrc_dn3 = assign30380_e24197_d_n3;
        locals.var_qsrc_dn4 = assign30380_e24197_d_n4;
        locals.var_qsrc_dn5 = assign30380_e24197_d_n5;
        locals.var_qsrc_dn6 = assign30380_e24197_d_n6;
        locals.var_qsrc_dn7 = assign30380_e24197_d_n7;
        locals.var_qsrc_dn8 = assign30380_e24197_d_n8;
        locals.var_qsrc_dn9 = assign30380_e24197_d_n9;
        locals.var_qsrc_dn10 = assign30380_e24197_d_n10;
        locals.var_qsrc_dn11 = assign30380_e24197_d_n11;
        locals.var_qsrc_dn12 = assign30380_e24197_d_n12;

        let (assign30390_e24212, assign30390_e24212_d_n3, assign30390_e24212_d_n4, assign30390_e24212_d_n5, assign30390_e24212_d_n6, assign30390_e24212_d_n7, assign30390_e24212_d_n8, assign30390_e24212_d_n9, assign30390_e24212_d_n10, assign30390_e24212_d_n11, assign30390_e24212_d_n12,) = {
    if (((locals.var_guard1698 != 0.0) && (locals.var_guard1711 == 0.0)) && (locals.var_guard1713 == 0.0)) {
        let assign30390_e24206: f64 = (-0.5);
        let assign30390_e24209: f64 = (locals.var_qinv + locals.var_qbulk);
        let assign30390_e24210: f64 = (assign30390_e24206 * assign30390_e24209);
        (assign30390_e24210, (assign30390_e24206 * (locals.var_qinv_dn3 + locals.var_qbulk_dn3)), (assign30390_e24206 * (locals.var_qinv_dn4 + locals.var_qbulk_dn4)), (assign30390_e24206 * (locals.var_qinv_dn5 + locals.var_qbulk_dn5)), (assign30390_e24206 * (locals.var_qinv_dn6 + locals.var_qbulk_dn6)), (assign30390_e24206 * (locals.var_qinv_dn7 + locals.var_qbulk_dn7)), (assign30390_e24206 * (locals.var_qinv_dn8 + locals.var_qbulk_dn8)), (assign30390_e24206 * (locals.var_qinv_dn9 + locals.var_qbulk_dn9)), (assign30390_e24206 * (locals.var_qinv_dn10 + locals.var_qbulk_dn10)), (assign30390_e24206 * (locals.var_qinv_dn11 + locals.var_qbulk_dn11)), (assign30390_e24206 * (locals.var_qinv_dn12 + locals.var_qbulk_dn12)),)
    } else {
        (locals.var_qsrc, locals.var_qsrc_dn3, locals.var_qsrc_dn4, locals.var_qsrc_dn5, locals.var_qsrc_dn6, locals.var_qsrc_dn7, locals.var_qsrc_dn8, locals.var_qsrc_dn9, locals.var_qsrc_dn10, locals.var_qsrc_dn11, locals.var_qsrc_dn12,)
    }
};
        locals.var_qsrc = assign30390_e24212;
        locals.var_qsrc_dn3 = assign30390_e24212_d_n3;
        locals.var_qsrc_dn4 = assign30390_e24212_d_n4;
        locals.var_qsrc_dn5 = assign30390_e24212_d_n5;
        locals.var_qsrc_dn6 = assign30390_e24212_d_n6;
        locals.var_qsrc_dn7 = assign30390_e24212_d_n7;
        locals.var_qsrc_dn8 = assign30390_e24212_d_n8;
        locals.var_qsrc_dn9 = assign30390_e24212_d_n9;
        locals.var_qsrc_dn10 = assign30390_e24212_d_n10;
        locals.var_qsrc_dn11 = assign30390_e24212_d_n11;
        locals.var_qsrc_dn12 = assign30390_e24212_d_n12;

        let assign30400_e24215: f64 = if locals.var_b4soisoimod == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1715 = assign30400_e24215;

        let (assign30410_e24221, assign30410_e24221_d_n3, assign30410_e24221_d_n4, assign30410_e24221_d_n5, assign30410_e24221_d_n6, assign30410_e24221_d_n7, assign30410_e24221_d_n8, assign30410_e24221_d_n9, assign30410_e24221_d_n10, assign30410_e24221_d_n11, assign30410_e24221_d_n12,) = {
    if ((locals.var_guard1698 != 0.0) && (locals.var_guard1715 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qe1, locals.var_qe1_dn3, locals.var_qe1_dn4, locals.var_qe1_dn5, locals.var_qe1_dn6, locals.var_qe1_dn7, locals.var_qe1_dn8, locals.var_qe1_dn9, locals.var_qe1_dn10, locals.var_qe1_dn11, locals.var_qe1_dn12,)
    }
};
        locals.var_qe1 = assign30410_e24221;
        locals.var_qe1_dn3 = assign30410_e24221_d_n3;
        locals.var_qe1_dn4 = assign30410_e24221_d_n4;
        locals.var_qe1_dn5 = assign30410_e24221_d_n5;
        locals.var_qe1_dn6 = assign30410_e24221_d_n6;
        locals.var_qe1_dn7 = assign30410_e24221_d_n7;
        locals.var_qe1_dn8 = assign30410_e24221_d_n8;
        locals.var_qe1_dn9 = assign30410_e24221_d_n9;
        locals.var_qe1_dn10 = assign30410_e24221_d_n10;
        locals.var_qe1_dn11 = assign30410_e24221_d_n11;
        locals.var_qe1_dn12 = assign30410_e24221_d_n12;

        let (assign30420_e24242, assign30420_e24242_d_n3, assign30420_e24242_d_n4, assign30420_e24242_d_n5, assign30420_e24242_d_n6, assign30420_e24242_d_n7, assign30420_e24242_d_n8, assign30420_e24242_d_n9, assign30420_e24242_d_n10, assign30420_e24242_d_n11, assign30420_e24242_d_n12,) = {
    if ((locals.var_guard1698 != 0.0) && (locals.var_guard1715 == 0.0)) {
        let assign30420_e24228: f64 = (locals.var_pparam_b4soikb1 * locals.var_b4soifbody);
        let assign30420_e24230: f64 = (assign30420_e24228 * locals.var_cbox);
        let assign30420_e24233: f64 = (locals.var_pparam_b4soiweffcv / locals.var_b4soinseg);
        let assign30420_e24235: f64 = (assign30420_e24233 * locals.var_b4soinf);
        let assign30420_e24237: f64 = (assign30420_e24235 * locals.var_pparam_b4soileffcvbg);
        let assign30420_e24239: f64 = (assign30420_e24237 + locals.var_b4soiaebcp);
        let assign30420_e24240: f64 = (assign30420_e24230 * assign30420_e24239);
        (assign30420_e24240, ((((locals.var_pparam_b4soikb1_dn3 * locals.var_b4soifbody) * locals.var_cbox) * assign30420_e24239) + (assign30420_e24230 * ((((locals.var_pparam_b4soiweffcv_dn3 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcvbg) + (assign30420_e24235 * locals.var_pparam_b4soileffcvbg_dn3)))), ((((locals.var_pparam_b4soikb1_dn4 * locals.var_b4soifbody) * locals.var_cbox) * assign30420_e24239) + (assign30420_e24230 * ((((locals.var_pparam_b4soiweffcv_dn4 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcvbg) + (assign30420_e24235 * locals.var_pparam_b4soileffcvbg_dn4)))), ((((locals.var_pparam_b4soikb1_dn5 * locals.var_b4soifbody) * locals.var_cbox) * assign30420_e24239) + (assign30420_e24230 * ((((locals.var_pparam_b4soiweffcv_dn5 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcvbg) + (assign30420_e24235 * locals.var_pparam_b4soileffcvbg_dn5)))), ((((locals.var_pparam_b4soikb1_dn6 * locals.var_b4soifbody) * locals.var_cbox) * assign30420_e24239) + (assign30420_e24230 * ((((locals.var_pparam_b4soiweffcv_dn6 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcvbg) + (assign30420_e24235 * locals.var_pparam_b4soileffcvbg_dn6)))), ((((locals.var_pparam_b4soikb1_dn7 * locals.var_b4soifbody) * locals.var_cbox) * assign30420_e24239) + (assign30420_e24230 * ((((locals.var_pparam_b4soiweffcv_dn7 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcvbg) + (assign30420_e24235 * locals.var_pparam_b4soileffcvbg_dn7)))), ((((locals.var_pparam_b4soikb1_dn8 * locals.var_b4soifbody) * locals.var_cbox) * assign30420_e24239) + (assign30420_e24230 * ((((locals.var_pparam_b4soiweffcv_dn8 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcvbg) + (assign30420_e24235 * locals.var_pparam_b4soileffcvbg_dn8)))), ((((locals.var_pparam_b4soikb1_dn9 * locals.var_b4soifbody) * locals.var_cbox) * assign30420_e24239) + (assign30420_e24230 * ((((locals.var_pparam_b4soiweffcv_dn9 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcvbg) + (assign30420_e24235 * locals.var_pparam_b4soileffcvbg_dn9)))), ((((locals.var_pparam_b4soikb1_dn10 * locals.var_b4soifbody) * locals.var_cbox) * assign30420_e24239) + (assign30420_e24230 * ((((locals.var_pparam_b4soiweffcv_dn10 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcvbg) + (assign30420_e24235 * locals.var_pparam_b4soileffcvbg_dn10)))), ((((locals.var_pparam_b4soikb1_dn11 * locals.var_b4soifbody) * locals.var_cbox) * assign30420_e24239) + (assign30420_e24230 * ((((locals.var_pparam_b4soiweffcv_dn11 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcvbg) + (assign30420_e24235 * locals.var_pparam_b4soileffcvbg_dn11)))), ((((locals.var_pparam_b4soikb1_dn12 * locals.var_b4soifbody) * locals.var_cbox) * assign30420_e24239) + (assign30420_e24230 * ((((locals.var_pparam_b4soiweffcv_dn12 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcvbg) + (assign30420_e24235 * locals.var_pparam_b4soileffcvbg_dn12)))),)
    } else {
        (locals.var_cboxwl, locals.var_cboxwl_dn3, locals.var_cboxwl_dn4, locals.var_cboxwl_dn5, locals.var_cboxwl_dn6, locals.var_cboxwl_dn7, locals.var_cboxwl_dn8, locals.var_cboxwl_dn9, locals.var_cboxwl_dn10, locals.var_cboxwl_dn11, locals.var_cboxwl_dn12,)
    }
};
        locals.var_cboxwl = assign30420_e24242;
        locals.var_cboxwl_dn3 = assign30420_e24242_d_n3;
        locals.var_cboxwl_dn4 = assign30420_e24242_d_n4;
        locals.var_cboxwl_dn5 = assign30420_e24242_d_n5;
        locals.var_cboxwl_dn6 = assign30420_e24242_d_n6;
        locals.var_cboxwl_dn7 = assign30420_e24242_d_n7;
        locals.var_cboxwl_dn8 = assign30420_e24242_d_n8;
        locals.var_cboxwl_dn9 = assign30420_e24242_d_n9;
        locals.var_cboxwl_dn10 = assign30420_e24242_d_n10;
        locals.var_cboxwl_dn11 = assign30420_e24242_d_n11;
        locals.var_cboxwl_dn12 = assign30420_e24242_d_n12;

        let (assign30430_e24253, assign30430_e24253_d_n3, assign30430_e24253_d_n4, assign30430_e24253_d_n5, assign30430_e24253_d_n6, assign30430_e24253_d_n7, assign30430_e24253_d_n8, assign30430_e24253_d_n9, assign30430_e24253_d_n10, assign30430_e24253_d_n11, assign30430_e24253_d_n12,) = {
    if ((locals.var_guard1698 != 0.0) && (locals.var_guard1715 == 0.0)) {
        let assign30430_e24250: f64 = (locals.var_vesfb - locals.var_vbs_1);
        let assign30430_e24251: f64 = (locals.var_cboxwl * assign30430_e24250);
        (assign30430_e24251, ((locals.var_cboxwl_dn3 * assign30430_e24250) + (locals.var_cboxwl * (locals.var_vesfb_dn3 - locals.var_vbs_1_dn3))), ((locals.var_cboxwl_dn4 * assign30430_e24250) + (locals.var_cboxwl * (locals.var_vesfb_dn4 - locals.var_vbs_1_dn4))), ((locals.var_cboxwl_dn5 * assign30430_e24250) + (locals.var_cboxwl * (locals.var_vesfb_dn5 - locals.var_vbs_1_dn5))), ((locals.var_cboxwl_dn6 * assign30430_e24250) + (locals.var_cboxwl * (locals.var_vesfb_dn6 - locals.var_vbs_1_dn6))), ((locals.var_cboxwl_dn7 * assign30430_e24250) + (locals.var_cboxwl * (locals.var_vesfb_dn7 - locals.var_vbs_1_dn7))), ((locals.var_cboxwl_dn8 * assign30430_e24250) + (locals.var_cboxwl * (locals.var_vesfb_dn8 - locals.var_vbs_1_dn8))), ((locals.var_cboxwl_dn9 * assign30430_e24250) + (locals.var_cboxwl * (locals.var_vesfb_dn9 - locals.var_vbs_1_dn9))), ((locals.var_cboxwl_dn10 * assign30430_e24250) + (locals.var_cboxwl * (locals.var_vesfb_dn10 - locals.var_vbs_1_dn10))), ((locals.var_cboxwl_dn11 * assign30430_e24250) + (locals.var_cboxwl * (locals.var_vesfb_dn11 - locals.var_vbs_1_dn11))), ((locals.var_cboxwl_dn12 * assign30430_e24250) + (locals.var_cboxwl * (locals.var_vesfb_dn12 - locals.var_vbs_1_dn12))),)
    } else {
        (locals.var_qe1, locals.var_qe1_dn3, locals.var_qe1_dn4, locals.var_qe1_dn5, locals.var_qe1_dn6, locals.var_qe1_dn7, locals.var_qe1_dn8, locals.var_qe1_dn9, locals.var_qe1_dn10, locals.var_qe1_dn11, locals.var_qe1_dn12,)
    }
};
        locals.var_qe1 = assign30430_e24253;
        locals.var_qe1_dn3 = assign30430_e24253_d_n3;
        locals.var_qe1_dn4 = assign30430_e24253_d_n4;
        locals.var_qe1_dn5 = assign30430_e24253_d_n5;
        locals.var_qe1_dn6 = assign30430_e24253_d_n6;
        locals.var_qe1_dn7 = assign30430_e24253_d_n7;
        locals.var_qe1_dn8 = assign30430_e24253_d_n8;
        locals.var_qe1_dn9 = assign30430_e24253_d_n9;
        locals.var_qe1_dn10 = assign30430_e24253_d_n10;
        locals.var_qe1_dn11 = assign30430_e24253_d_n11;
        locals.var_qe1_dn12 = assign30430_e24253_d_n12;

        let (assign30440_e24261, assign30440_e24261_d_n3, assign30440_e24261_d_n4, assign30440_e24261_d_n5, assign30440_e24261_d_n6, assign30440_e24261_d_n7, assign30440_e24261_d_n8, assign30440_e24261_d_n9, assign30440_e24261_d_n10, assign30440_e24261_d_n11, assign30440_e24261_d_n12,) = {
    if (locals.var_guard1698 != 0.0) {
        let assign30440_e24257: f64 = (locals.var_qinv + locals.var_qac0);
        let assign30440_e24259: f64 = (assign30440_e24257 + locals.var_qsub0);
        (assign30440_e24259, ((locals.var_qinv_dn3 + locals.var_qac0_dn3) + locals.var_qsub0_dn3), ((locals.var_qinv_dn4 + locals.var_qac0_dn4) + locals.var_qsub0_dn4), ((locals.var_qinv_dn5 + locals.var_qac0_dn5) + locals.var_qsub0_dn5), ((locals.var_qinv_dn6 + locals.var_qac0_dn6) + locals.var_qsub0_dn6), ((locals.var_qinv_dn7 + locals.var_qac0_dn7) + locals.var_qsub0_dn7), ((locals.var_qinv_dn8 + locals.var_qac0_dn8) + locals.var_qsub0_dn8), ((locals.var_qinv_dn9 + locals.var_qac0_dn9) + locals.var_qsub0_dn9), ((locals.var_qinv_dn10 + locals.var_qac0_dn10) + locals.var_qsub0_dn10), ((locals.var_qinv_dn11 + locals.var_qac0_dn11) + locals.var_qsub0_dn11), ((locals.var_qinv_dn12 + locals.var_qac0_dn12) + locals.var_qsub0_dn12),)
    } else {
        (locals.var_qgate, locals.var_qgate_dn3, locals.var_qgate_dn4, locals.var_qgate_dn5, locals.var_qgate_dn6, locals.var_qgate_dn7, locals.var_qgate_dn8, locals.var_qgate_dn9, locals.var_qgate_dn10, locals.var_qgate_dn11, locals.var_qgate_dn12,)
    }
};
        locals.var_qgate = assign30440_e24261;
        locals.var_qgate_dn3 = assign30440_e24261_d_n3;
        locals.var_qgate_dn4 = assign30440_e24261_d_n4;
        locals.var_qgate_dn5 = assign30440_e24261_d_n5;
        locals.var_qgate_dn6 = assign30440_e24261_d_n6;
        locals.var_qgate_dn7 = assign30440_e24261_d_n7;
        locals.var_qgate_dn8 = assign30440_e24261_d_n8;
        locals.var_qgate_dn9 = assign30440_e24261_d_n9;
        locals.var_qgate_dn10 = assign30440_e24261_d_n10;
        locals.var_qgate_dn11 = assign30440_e24261_d_n11;
        locals.var_qgate_dn12 = assign30440_e24261_d_n12;

        let (assign30450_e24271, assign30450_e24271_d_n3, assign30450_e24271_d_n4, assign30450_e24271_d_n5, assign30450_e24271_d_n6, assign30450_e24271_d_n7, assign30450_e24271_d_n8, assign30450_e24271_d_n9, assign30450_e24271_d_n10, assign30450_e24271_d_n11, assign30450_e24271_d_n12,) = {
    if (locals.var_guard1698 != 0.0) {
        let assign30450_e24265: f64 = (locals.var_qbulk - locals.var_qac0);
        let assign30450_e24267: f64 = (assign30450_e24265 - locals.var_qsub0);
        let assign30450_e24269: f64 = (assign30450_e24267 - locals.var_qe1);
        (assign30450_e24269, (((locals.var_qbulk_dn3 - locals.var_qac0_dn3) - locals.var_qsub0_dn3) - locals.var_qe1_dn3), (((locals.var_qbulk_dn4 - locals.var_qac0_dn4) - locals.var_qsub0_dn4) - locals.var_qe1_dn4), (((locals.var_qbulk_dn5 - locals.var_qac0_dn5) - locals.var_qsub0_dn5) - locals.var_qe1_dn5), (((locals.var_qbulk_dn6 - locals.var_qac0_dn6) - locals.var_qsub0_dn6) - locals.var_qe1_dn6), (((locals.var_qbulk_dn7 - locals.var_qac0_dn7) - locals.var_qsub0_dn7) - locals.var_qe1_dn7), (((locals.var_qbulk_dn8 - locals.var_qac0_dn8) - locals.var_qsub0_dn8) - locals.var_qe1_dn8), (((locals.var_qbulk_dn9 - locals.var_qac0_dn9) - locals.var_qsub0_dn9) - locals.var_qe1_dn9), (((locals.var_qbulk_dn10 - locals.var_qac0_dn10) - locals.var_qsub0_dn10) - locals.var_qe1_dn10), (((locals.var_qbulk_dn11 - locals.var_qac0_dn11) - locals.var_qsub0_dn11) - locals.var_qe1_dn11), (((locals.var_qbulk_dn12 - locals.var_qac0_dn12) - locals.var_qsub0_dn12) - locals.var_qe1_dn12),)
    } else {
        (locals.var_qbody, locals.var_qbody_dn3, locals.var_qbody_dn4, locals.var_qbody_dn5, locals.var_qbody_dn6, locals.var_qbody_dn7, locals.var_qbody_dn8, locals.var_qbody_dn9, locals.var_qbody_dn10, locals.var_qbody_dn11, locals.var_qbody_dn12,)
    }
};
        locals.var_qbody = assign30450_e24271;
        locals.var_qbody_dn3 = assign30450_e24271_d_n3;
        locals.var_qbody_dn4 = assign30450_e24271_d_n4;
        locals.var_qbody_dn5 = assign30450_e24271_d_n5;
        locals.var_qbody_dn6 = assign30450_e24271_d_n6;
        locals.var_qbody_dn7 = assign30450_e24271_d_n7;
        locals.var_qbody_dn8 = assign30450_e24271_d_n8;
        locals.var_qbody_dn9 = assign30450_e24271_d_n9;
        locals.var_qbody_dn10 = assign30450_e24271_d_n10;
        locals.var_qbody_dn11 = assign30450_e24271_d_n11;
        locals.var_qbody_dn12 = assign30450_e24271_d_n12;

        let (assign30460_e24275, assign30460_e24275_d_n3, assign30460_e24275_d_n4, assign30460_e24275_d_n5, assign30460_e24275_d_n6, assign30460_e24275_d_n7, assign30460_e24275_d_n8, assign30460_e24275_d_n9, assign30460_e24275_d_n10, assign30460_e24275_d_n11, assign30460_e24275_d_n12,) = {
    if (locals.var_guard1698 != 0.0) {
        (locals.var_qe1, locals.var_qe1_dn3, locals.var_qe1_dn4, locals.var_qe1_dn5, locals.var_qe1_dn6, locals.var_qe1_dn7, locals.var_qe1_dn8, locals.var_qe1_dn9, locals.var_qe1_dn10, locals.var_qe1_dn11, locals.var_qe1_dn12,)
    } else {
        (locals.var_qsub, locals.var_qsub_dn3, locals.var_qsub_dn4, locals.var_qsub_dn5, locals.var_qsub_dn6, locals.var_qsub_dn7, locals.var_qsub_dn8, locals.var_qsub_dn9, locals.var_qsub_dn10, locals.var_qsub_dn11, locals.var_qsub_dn12,)
    }
};
        locals.var_qsub = assign30460_e24275;
        locals.var_qsub_dn3 = assign30460_e24275_d_n3;
        locals.var_qsub_dn4 = assign30460_e24275_d_n4;
        locals.var_qsub_dn5 = assign30460_e24275_d_n5;
        locals.var_qsub_dn6 = assign30460_e24275_d_n6;
        locals.var_qsub_dn7 = assign30460_e24275_d_n7;
        locals.var_qsub_dn8 = assign30460_e24275_d_n8;
        locals.var_qsub_dn9 = assign30460_e24275_d_n9;
        locals.var_qsub_dn10 = assign30460_e24275_d_n10;
        locals.var_qsub_dn11 = assign30460_e24275_d_n11;
        locals.var_qsub_dn12 = assign30460_e24275_d_n12;

        let (assign30470_e24286, assign30470_e24286_d_n3, assign30470_e24286_d_n4, assign30470_e24286_d_n5, assign30470_e24286_d_n6, assign30470_e24286_d_n7, assign30470_e24286_d_n8, assign30470_e24286_d_n9, assign30470_e24286_d_n10, assign30470_e24286_d_n11, assign30470_e24286_d_n12,) = {
    if (locals.var_guard1698 != 0.0) {
        let assign30470_e24279: f64 = (locals.var_qgate + locals.var_qsrc);
        let assign30470_e24281: f64 = (assign30470_e24279 + locals.var_qbody);
        let assign30470_e24283: f64 = (assign30470_e24281 + locals.var_qsub);
        let assign30470_e24284: f64 = (-assign30470_e24283);
        (assign30470_e24284, (-(((locals.var_qgate_dn3 + locals.var_qsrc_dn3) + locals.var_qbody_dn3) + locals.var_qsub_dn3)), (-(((locals.var_qgate_dn4 + locals.var_qsrc_dn4) + locals.var_qbody_dn4) + locals.var_qsub_dn4)), (-(((locals.var_qgate_dn5 + locals.var_qsrc_dn5) + locals.var_qbody_dn5) + locals.var_qsub_dn5)), (-(((locals.var_qgate_dn6 + locals.var_qsrc_dn6) + locals.var_qbody_dn6) + locals.var_qsub_dn6)), (-(((locals.var_qgate_dn7 + locals.var_qsrc_dn7) + locals.var_qbody_dn7) + locals.var_qsub_dn7)), (-(((locals.var_qgate_dn8 + locals.var_qsrc_dn8) + locals.var_qbody_dn8) + locals.var_qsub_dn8)), (-(((locals.var_qgate_dn9 + locals.var_qsrc_dn9) + locals.var_qbody_dn9) + locals.var_qsub_dn9)), (-(((locals.var_qgate_dn10 + locals.var_qsrc_dn10) + locals.var_qbody_dn10) + locals.var_qsub_dn10)), (-(((locals.var_qgate_dn11 + locals.var_qsrc_dn11) + locals.var_qbody_dn11) + locals.var_qsub_dn11)), (-(((locals.var_qgate_dn12 + locals.var_qsrc_dn12) + locals.var_qbody_dn12) + locals.var_qsub_dn12)),)
    } else {
        (locals.var_qdrn, locals.var_qdrn_dn3, locals.var_qdrn_dn4, locals.var_qdrn_dn5, locals.var_qdrn_dn6, locals.var_qdrn_dn7, locals.var_qdrn_dn8, locals.var_qdrn_dn9, locals.var_qdrn_dn10, locals.var_qdrn_dn11, locals.var_qdrn_dn12,)
    }
};
        locals.var_qdrn = assign30470_e24286;
        locals.var_qdrn_dn3 = assign30470_e24286_d_n3;
        locals.var_qdrn_dn4 = assign30470_e24286_d_n4;
        locals.var_qdrn_dn5 = assign30470_e24286_d_n5;
        locals.var_qdrn_dn6 = assign30470_e24286_d_n6;
        locals.var_qdrn_dn7 = assign30470_e24286_d_n7;
        locals.var_qdrn_dn8 = assign30470_e24286_d_n8;
        locals.var_qdrn_dn9 = assign30470_e24286_d_n9;
        locals.var_qdrn_dn10 = assign30470_e24286_d_n10;
        locals.var_qdrn_dn11 = assign30470_e24286_d_n11;
        locals.var_qdrn_dn12 = assign30470_e24286_d_n12;

        let assign30480_e24289: f64 = if locals.var_b4soicapmod == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1716 = assign30480_e24289;

        let assign30490_e24292: f64 = if locals.var_b4soimtrlmod == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1717 = assign30490_e24292;

        let (assign30500_e24303, assign30500_e24303_d_n3, assign30500_e24303_d_n4, assign30500_e24303_d_n5, assign30500_e24303_d_n6, assign30500_e24303_d_n7, assign30500_e24303_d_n8, assign30500_e24303_d_n9, assign30500_e24303_d_n10, assign30500_e24303_d_n11, assign30500_e24303_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1717 != 0.0)) {
        let assign30500_e24301: f64 = (3.453133e-11 / locals.var_b4soitoxp);
        (assign30500_e24301, (-((3.453133e-11 * locals.var_b4soitoxp_dn3) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((3.453133e-11 * locals.var_b4soitoxp_dn4) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((3.453133e-11 * locals.var_b4soitoxp_dn5) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((3.453133e-11 * locals.var_b4soitoxp_dn6) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((3.453133e-11 * locals.var_b4soitoxp_dn7) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((3.453133e-11 * locals.var_b4soitoxp_dn8) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((3.453133e-11 * locals.var_b4soitoxp_dn9) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((3.453133e-11 * locals.var_b4soitoxp_dn10) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((3.453133e-11 * locals.var_b4soitoxp_dn11) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((3.453133e-11 * locals.var_b4soitoxp_dn12) / (locals.var_b4soitoxp * locals.var_b4soitoxp))),)
    } else {
        (locals.var_cox, locals.var_cox_dn3, locals.var_cox_dn4, locals.var_cox_dn5, locals.var_cox_dn6, locals.var_cox_dn7, locals.var_cox_dn8, locals.var_cox_dn9, locals.var_cox_dn10, locals.var_cox_dn11, locals.var_cox_dn12,)
    }
};
        locals.var_cox = assign30500_e24303;
        locals.var_cox_dn3 = assign30500_e24303_d_n3;
        locals.var_cox_dn4 = assign30500_e24303_d_n4;
        locals.var_cox_dn5 = assign30500_e24303_d_n5;
        locals.var_cox_dn6 = assign30500_e24303_d_n6;
        locals.var_cox_dn7 = assign30500_e24303_d_n7;
        locals.var_cox_dn8 = assign30500_e24303_d_n8;
        locals.var_cox_dn9 = assign30500_e24303_d_n9;
        locals.var_cox_dn10 = assign30500_e24303_d_n10;
        locals.var_cox_dn11 = assign30500_e24303_d_n11;
        locals.var_cox_dn12 = assign30500_e24303_d_n12;

        let (assign30510_e24317, assign30510_e24317_d_n3, assign30510_e24317_d_n4, assign30510_e24317_d_n5, assign30510_e24317_d_n6, assign30510_e24317_d_n7, assign30510_e24317_d_n8, assign30510_e24317_d_n9, assign30510_e24317_d_n10, assign30510_e24317_d_n11, assign30510_e24317_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1717 == 0.0)) {
        let assign30510_e24313: f64 = (locals.var_epsrox * 8.85418e-12);
        let assign30510_e24315: f64 = (assign30510_e24313 / locals.var_b4soitoxp);
        (assign30510_e24315, (-((assign30510_e24313 * locals.var_b4soitoxp_dn3) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((assign30510_e24313 * locals.var_b4soitoxp_dn4) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((assign30510_e24313 * locals.var_b4soitoxp_dn5) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((assign30510_e24313 * locals.var_b4soitoxp_dn6) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((assign30510_e24313 * locals.var_b4soitoxp_dn7) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((assign30510_e24313 * locals.var_b4soitoxp_dn8) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((assign30510_e24313 * locals.var_b4soitoxp_dn9) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((assign30510_e24313 * locals.var_b4soitoxp_dn10) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((assign30510_e24313 * locals.var_b4soitoxp_dn11) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((assign30510_e24313 * locals.var_b4soitoxp_dn12) / (locals.var_b4soitoxp * locals.var_b4soitoxp))),)
    } else {
        (locals.var_cox, locals.var_cox_dn3, locals.var_cox_dn4, locals.var_cox_dn5, locals.var_cox_dn6, locals.var_cox_dn7, locals.var_cox_dn8, locals.var_cox_dn9, locals.var_cox_dn10, locals.var_cox_dn11, locals.var_cox_dn12,)
    }
};
        locals.var_cox = assign30510_e24317;
        locals.var_cox_dn3 = assign30510_e24317_d_n3;
        locals.var_cox_dn4 = assign30510_e24317_d_n4;
        locals.var_cox_dn5 = assign30510_e24317_d_n5;
        locals.var_cox_dn6 = assign30510_e24317_d_n6;
        locals.var_cox_dn7 = assign30510_e24317_d_n7;
        locals.var_cox_dn8 = assign30510_e24317_d_n8;
        locals.var_cox_dn9 = assign30510_e24317_d_n9;
        locals.var_cox_dn10 = assign30510_e24317_d_n10;
        locals.var_cox_dn11 = assign30510_e24317_d_n11;
        locals.var_cox_dn12 = assign30510_e24317_d_n12;

        let (assign30520_e24328, assign30520_e24328_d_n3, assign30520_e24328_d_n4, assign30520_e24328_d_n5, assign30520_e24328_d_n6, assign30520_e24328_d_n7, assign30520_e24328_d_n8, assign30520_e24328_d_n9, assign30520_e24328_d_n10, assign30520_e24328_d_n11, assign30520_e24328_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) {
        let assign30520_e24324: f64 = (locals.var_coxwl * locals.var_toxe);
        let assign30520_e24326: f64 = (assign30520_e24324 / locals.var_b4soitoxp);
        (assign30520_e24326, ((((locals.var_coxwl_dn3 * locals.var_toxe) * locals.var_b4soitoxp) - (assign30520_e24324 * locals.var_b4soitoxp_dn3)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl_dn4 * locals.var_toxe) * locals.var_b4soitoxp) - (assign30520_e24324 * locals.var_b4soitoxp_dn4)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl_dn5 * locals.var_toxe) * locals.var_b4soitoxp) - (assign30520_e24324 * locals.var_b4soitoxp_dn5)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl_dn6 * locals.var_toxe) * locals.var_b4soitoxp) - (assign30520_e24324 * locals.var_b4soitoxp_dn6)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl_dn7 * locals.var_toxe) * locals.var_b4soitoxp) - (assign30520_e24324 * locals.var_b4soitoxp_dn7)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl_dn8 * locals.var_toxe) * locals.var_b4soitoxp) - (assign30520_e24324 * locals.var_b4soitoxp_dn8)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl_dn9 * locals.var_toxe) * locals.var_b4soitoxp) - (assign30520_e24324 * locals.var_b4soitoxp_dn9)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl_dn10 * locals.var_toxe) * locals.var_b4soitoxp) - (assign30520_e24324 * locals.var_b4soitoxp_dn10)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl_dn11 * locals.var_toxe) * locals.var_b4soitoxp) - (assign30520_e24324 * locals.var_b4soitoxp_dn11)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl_dn12 * locals.var_toxe) * locals.var_b4soitoxp) - (assign30520_e24324 * locals.var_b4soitoxp_dn12)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)),)
    } else {
        (locals.var_coxwl, locals.var_coxwl_dn3, locals.var_coxwl_dn4, locals.var_coxwl_dn5, locals.var_coxwl_dn6, locals.var_coxwl_dn7, locals.var_coxwl_dn8, locals.var_coxwl_dn9, locals.var_coxwl_dn10, locals.var_coxwl_dn11, locals.var_coxwl_dn12,)
    }
};
        locals.var_coxwl = assign30520_e24328;
        locals.var_coxwl_dn3 = assign30520_e24328_d_n3;
        locals.var_coxwl_dn4 = assign30520_e24328_d_n4;
        locals.var_coxwl_dn5 = assign30520_e24328_d_n5;
        locals.var_coxwl_dn6 = assign30520_e24328_d_n6;
        locals.var_coxwl_dn7 = assign30520_e24328_d_n7;
        locals.var_coxwl_dn8 = assign30520_e24328_d_n8;
        locals.var_coxwl_dn9 = assign30520_e24328_d_n9;
        locals.var_coxwl_dn10 = assign30520_e24328_d_n10;
        locals.var_coxwl_dn11 = assign30520_e24328_d_n11;
        locals.var_coxwl_dn12 = assign30520_e24328_d_n12;

        let (assign30530_e24339, assign30530_e24339_d_n3, assign30530_e24339_d_n4, assign30530_e24339_d_n5, assign30530_e24339_d_n6, assign30530_e24339_d_n7, assign30530_e24339_d_n8, assign30530_e24339_d_n9, assign30530_e24339_d_n10, assign30530_e24339_d_n11, assign30530_e24339_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) {
        let assign30530_e24335: f64 = (locals.var_coxwlb * locals.var_b4soitox);
        let assign30530_e24337: f64 = (assign30530_e24335 / locals.var_b4soitoxp);
        (assign30530_e24337, ((((locals.var_coxwlb_dn3 * locals.var_b4soitox) * locals.var_b4soitoxp) - (assign30530_e24335 * locals.var_b4soitoxp_dn3)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb_dn4 * locals.var_b4soitox) * locals.var_b4soitoxp) - (assign30530_e24335 * locals.var_b4soitoxp_dn4)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb_dn5 * locals.var_b4soitox) * locals.var_b4soitoxp) - (assign30530_e24335 * locals.var_b4soitoxp_dn5)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb_dn6 * locals.var_b4soitox) * locals.var_b4soitoxp) - (assign30530_e24335 * locals.var_b4soitoxp_dn6)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb_dn7 * locals.var_b4soitox) * locals.var_b4soitoxp) - (assign30530_e24335 * locals.var_b4soitoxp_dn7)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb_dn8 * locals.var_b4soitox) * locals.var_b4soitoxp) - (assign30530_e24335 * locals.var_b4soitoxp_dn8)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb_dn9 * locals.var_b4soitox) * locals.var_b4soitoxp) - (assign30530_e24335 * locals.var_b4soitoxp_dn9)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb_dn10 * locals.var_b4soitox) * locals.var_b4soitoxp) - (assign30530_e24335 * locals.var_b4soitoxp_dn10)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb_dn11 * locals.var_b4soitox) * locals.var_b4soitoxp) - (assign30530_e24335 * locals.var_b4soitoxp_dn11)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb_dn12 * locals.var_b4soitox) * locals.var_b4soitoxp) - (assign30530_e24335 * locals.var_b4soitoxp_dn12)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)),)
    } else {
        (locals.var_coxwlb, locals.var_coxwlb_dn3, locals.var_coxwlb_dn4, locals.var_coxwlb_dn5, locals.var_coxwlb_dn6, locals.var_coxwlb_dn7, locals.var_coxwlb_dn8, locals.var_coxwlb_dn9, locals.var_coxwlb_dn10, locals.var_coxwlb_dn11, locals.var_coxwlb_dn12,)
    }
};
        locals.var_coxwlb = assign30530_e24339;
        locals.var_coxwlb_dn3 = assign30530_e24339_d_n3;
        locals.var_coxwlb_dn4 = assign30530_e24339_d_n4;
        locals.var_coxwlb_dn5 = assign30530_e24339_d_n5;
        locals.var_coxwlb_dn6 = assign30530_e24339_d_n6;
        locals.var_coxwlb_dn7 = assign30530_e24339_d_n7;
        locals.var_coxwlb_dn8 = assign30530_e24339_d_n8;
        locals.var_coxwlb_dn9 = assign30530_e24339_d_n9;
        locals.var_coxwlb_dn10 = assign30530_e24339_d_n10;
        locals.var_coxwlb_dn11 = assign30530_e24339_d_n11;
        locals.var_coxwlb_dn12 = assign30530_e24339_d_n12;

        let (assign30540_e24348, assign30540_e24348_d_n3, assign30540_e24348_d_n4, assign30540_e24348_d_n5, assign30540_e24348_d_n6, assign30540_e24348_d_n7, assign30540_e24348_d_n8, assign30540_e24348_d_n9, assign30540_e24348_d_n10, assign30540_e24348_d_n11, assign30540_e24348_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) {
        let assign30540_e24346: f64 = (100000000.0 * locals.var_b4soitoxp);
        (assign30540_e24346, (100000000.0 * locals.var_b4soitoxp_dn3), (100000000.0 * locals.var_b4soitoxp_dn4), (100000000.0 * locals.var_b4soitoxp_dn5), (100000000.0 * locals.var_b4soitoxp_dn6), (100000000.0 * locals.var_b4soitoxp_dn7), (100000000.0 * locals.var_b4soitoxp_dn8), (100000000.0 * locals.var_b4soitoxp_dn9), (100000000.0 * locals.var_b4soitoxp_dn10), (100000000.0 * locals.var_b4soitoxp_dn11), (100000000.0 * locals.var_b4soitoxp_dn12),)
    } else {
        (locals.var_tox, locals.var_tox_dn3, locals.var_tox_dn4, locals.var_tox_dn5, locals.var_tox_dn6, locals.var_tox_dn7, locals.var_tox_dn8, locals.var_tox_dn9, locals.var_tox_dn10, locals.var_tox_dn11, locals.var_tox_dn12,)
    }
};
        locals.var_tox = assign30540_e24348;
        locals.var_tox_dn3 = assign30540_e24348_d_n3;
        locals.var_tox_dn4 = assign30540_e24348_d_n4;
        locals.var_tox_dn5 = assign30540_e24348_d_n5;
        locals.var_tox_dn6 = assign30540_e24348_d_n6;
        locals.var_tox_dn7 = assign30540_e24348_d_n7;
        locals.var_tox_dn8 = assign30540_e24348_d_n8;
        locals.var_tox_dn9 = assign30540_e24348_d_n9;
        locals.var_tox_dn10 = assign30540_e24348_d_n10;
        locals.var_tox_dn11 = assign30540_e24348_d_n11;
        locals.var_tox_dn12 = assign30540_e24348_d_n12;

        let assign30550_e24351: f64 = if locals.var_b4soiagbcp2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1718 = assign30550_e24351;

        let (assign30560_e24364, assign30560_e24364_d_n3, assign30560_e24364_d_n4, assign30560_e24364_d_n5, assign30560_e24364_d_n6, assign30560_e24364_d_n7, assign30560_e24364_d_n8, assign30560_e24364_d_n9, assign30560_e24364_d_n10, assign30560_e24364_d_n11, assign30560_e24364_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1718 != 0.0)) {
        let assign30560_e24360: f64 = (locals.var_coxwl2 * locals.var_b4soitox);
        let assign30560_e24362: f64 = (assign30560_e24360 / locals.var_b4soitoxp);
        (assign30560_e24362, ((((locals.var_coxwl2_dn3 * locals.var_b4soitox) * locals.var_b4soitoxp) - (assign30560_e24360 * locals.var_b4soitoxp_dn3)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl2_dn4 * locals.var_b4soitox) * locals.var_b4soitoxp) - (assign30560_e24360 * locals.var_b4soitoxp_dn4)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl2_dn5 * locals.var_b4soitox) * locals.var_b4soitoxp) - (assign30560_e24360 * locals.var_b4soitoxp_dn5)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl2_dn6 * locals.var_b4soitox) * locals.var_b4soitoxp) - (assign30560_e24360 * locals.var_b4soitoxp_dn6)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl2_dn7 * locals.var_b4soitox) * locals.var_b4soitoxp) - (assign30560_e24360 * locals.var_b4soitoxp_dn7)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl2_dn8 * locals.var_b4soitox) * locals.var_b4soitoxp) - (assign30560_e24360 * locals.var_b4soitoxp_dn8)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl2_dn9 * locals.var_b4soitox) * locals.var_b4soitoxp) - (assign30560_e24360 * locals.var_b4soitoxp_dn9)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl2_dn10 * locals.var_b4soitox) * locals.var_b4soitoxp) - (assign30560_e24360 * locals.var_b4soitoxp_dn10)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl2_dn11 * locals.var_b4soitox) * locals.var_b4soitoxp) - (assign30560_e24360 * locals.var_b4soitoxp_dn11)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl2_dn12 * locals.var_b4soitox) * locals.var_b4soitoxp) - (assign30560_e24360 * locals.var_b4soitoxp_dn12)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)),)
    } else {
        (locals.var_coxwl2, locals.var_coxwl2_dn3, locals.var_coxwl2_dn4, locals.var_coxwl2_dn5, locals.var_coxwl2_dn6, locals.var_coxwl2_dn7, locals.var_coxwl2_dn8, locals.var_coxwl2_dn9, locals.var_coxwl2_dn10, locals.var_coxwl2_dn11, locals.var_coxwl2_dn12,)
    }
};
        locals.var_coxwl2 = assign30560_e24364;
        locals.var_coxwl2_dn3 = assign30560_e24364_d_n3;
        locals.var_coxwl2_dn4 = assign30560_e24364_d_n4;
        locals.var_coxwl2_dn5 = assign30560_e24364_d_n5;
        locals.var_coxwl2_dn6 = assign30560_e24364_d_n6;
        locals.var_coxwl2_dn7 = assign30560_e24364_d_n7;
        locals.var_coxwl2_dn8 = assign30560_e24364_d_n8;
        locals.var_coxwl2_dn9 = assign30560_e24364_d_n9;
        locals.var_coxwl2_dn10 = assign30560_e24364_d_n10;
        locals.var_coxwl2_dn11 = assign30560_e24364_d_n11;
        locals.var_coxwl2_dn12 = assign30560_e24364_d_n12;

        let (assign30570_e24377, assign30570_e24377_d_n3, assign30570_e24377_d_n4, assign30570_e24377_d_n5, assign30570_e24377_d_n6, assign30570_e24377_d_n7, assign30570_e24377_d_n8, assign30570_e24377_d_n9, assign30570_e24377_d_n10, assign30570_e24377_d_n11, assign30570_e24377_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1718 != 0.0)) {
        let assign30570_e24373: f64 = (locals.var_coxwlb2 * locals.var_b4soitox);
        let assign30570_e24375: f64 = (assign30570_e24373 / locals.var_b4soitoxp);
        (assign30570_e24375, ((((locals.var_coxwlb2_dn3 * locals.var_b4soitox) * locals.var_b4soitoxp) - (assign30570_e24373 * locals.var_b4soitoxp_dn3)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb2_dn4 * locals.var_b4soitox) * locals.var_b4soitoxp) - (assign30570_e24373 * locals.var_b4soitoxp_dn4)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb2_dn5 * locals.var_b4soitox) * locals.var_b4soitoxp) - (assign30570_e24373 * locals.var_b4soitoxp_dn5)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb2_dn6 * locals.var_b4soitox) * locals.var_b4soitoxp) - (assign30570_e24373 * locals.var_b4soitoxp_dn6)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb2_dn7 * locals.var_b4soitox) * locals.var_b4soitoxp) - (assign30570_e24373 * locals.var_b4soitoxp_dn7)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb2_dn8 * locals.var_b4soitox) * locals.var_b4soitoxp) - (assign30570_e24373 * locals.var_b4soitoxp_dn8)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb2_dn9 * locals.var_b4soitox) * locals.var_b4soitoxp) - (assign30570_e24373 * locals.var_b4soitoxp_dn9)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb2_dn10 * locals.var_b4soitox) * locals.var_b4soitoxp) - (assign30570_e24373 * locals.var_b4soitoxp_dn10)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb2_dn11 * locals.var_b4soitox) * locals.var_b4soitoxp) - (assign30570_e24373 * locals.var_b4soitoxp_dn11)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb2_dn12 * locals.var_b4soitox) * locals.var_b4soitoxp) - (assign30570_e24373 * locals.var_b4soitoxp_dn12)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)),)
    } else {
        (locals.var_coxwlb2, locals.var_coxwlb2_dn3, locals.var_coxwlb2_dn4, locals.var_coxwlb2_dn5, locals.var_coxwlb2_dn6, locals.var_coxwlb2_dn7, locals.var_coxwlb2_dn8, locals.var_coxwlb2_dn9, locals.var_coxwlb2_dn10, locals.var_coxwlb2_dn11, locals.var_coxwlb2_dn12,)
    }
};
        locals.var_coxwlb2 = assign30570_e24377;
        locals.var_coxwlb2_dn3 = assign30570_e24377_d_n3;
        locals.var_coxwlb2_dn4 = assign30570_e24377_d_n4;
        locals.var_coxwlb2_dn5 = assign30570_e24377_d_n5;
        locals.var_coxwlb2_dn6 = assign30570_e24377_d_n6;
        locals.var_coxwlb2_dn7 = assign30570_e24377_d_n7;
        locals.var_coxwlb2_dn8 = assign30570_e24377_d_n8;
        locals.var_coxwlb2_dn9 = assign30570_e24377_d_n9;
        locals.var_coxwlb2_dn10 = assign30570_e24377_d_n10;
        locals.var_coxwlb2_dn11 = assign30570_e24377_d_n11;
        locals.var_coxwlb2_dn12 = assign30570_e24377_d_n12;

        let assign30580_e24380: f64 = if locals.var_b4soisoimod == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1719 = assign30580_e24380;

        let (assign30590_e24389, assign30590_e24389_d_n3, assign30590_e24389_d_n4, assign30590_e24389_d_n5, assign30590_e24389_d_n6, assign30590_e24389_d_n7, assign30590_e24389_d_n8, assign30590_e24389_d_n9, assign30590_e24389_d_n10, assign30590_e24389_d_n11, assign30590_e24389_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qac0, locals.var_qac0_dn3, locals.var_qac0_dn4, locals.var_qac0_dn5, locals.var_qac0_dn6, locals.var_qac0_dn7, locals.var_qac0_dn8, locals.var_qac0_dn9, locals.var_qac0_dn10, locals.var_qac0_dn11, locals.var_qac0_dn12,)
    }
};
        locals.var_qac0 = assign30590_e24389;
        locals.var_qac0_dn3 = assign30590_e24389_d_n3;
        locals.var_qac0_dn4 = assign30590_e24389_d_n4;
        locals.var_qac0_dn5 = assign30590_e24389_d_n5;
        locals.var_qac0_dn6 = assign30590_e24389_d_n6;
        locals.var_qac0_dn7 = assign30590_e24389_d_n7;
        locals.var_qac0_dn8 = assign30590_e24389_d_n8;
        locals.var_qac0_dn9 = assign30590_e24389_d_n9;
        locals.var_qac0_dn10 = assign30590_e24389_d_n10;
        locals.var_qac0_dn11 = assign30590_e24389_d_n11;
        locals.var_qac0_dn12 = assign30590_e24389_d_n12;

        let (assign30600_e24398, assign30600_e24398_d_n3, assign30600_e24398_d_n4, assign30600_e24398_d_n5, assign30600_e24398_d_n6, assign30600_e24398_d_n7, assign30600_e24398_d_n8, assign30600_e24398_d_n9, assign30600_e24398_d_n10, assign30600_e24398_d_n11, assign30600_e24398_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qsub0, locals.var_qsub0_dn3, locals.var_qsub0_dn4, locals.var_qsub0_dn5, locals.var_qsub0_dn6, locals.var_qsub0_dn7, locals.var_qsub0_dn8, locals.var_qsub0_dn9, locals.var_qsub0_dn10, locals.var_qsub0_dn11, locals.var_qsub0_dn12,)
    }
};
        locals.var_qsub0 = assign30600_e24398;
        locals.var_qsub0_dn3 = assign30600_e24398_d_n3;
        locals.var_qsub0_dn4 = assign30600_e24398_d_n4;
        locals.var_qsub0_dn5 = assign30600_e24398_d_n5;
        locals.var_qsub0_dn6 = assign30600_e24398_d_n6;
        locals.var_qsub0_dn7 = assign30600_e24398_d_n7;
        locals.var_qsub0_dn8 = assign30600_e24398_d_n8;
        locals.var_qsub0_dn9 = assign30600_e24398_d_n9;
        locals.var_qsub0_dn10 = assign30600_e24398_d_n10;
        locals.var_qsub0_dn11 = assign30600_e24398_d_n11;
        locals.var_qsub0_dn12 = assign30600_e24398_d_n12;

        let (assign30610_e24407, assign30610_e24407_d_n3, assign30610_e24407_d_n4, assign30610_e24407_d_n5, assign30610_e24407_d_n6, assign30610_e24407_d_n7, assign30610_e24407_d_n8, assign30610_e24407_d_n9, assign30610_e24407_d_n10, assign30610_e24407_d_n11, assign30610_e24407_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vfbzb, locals.var_vfbzb_dn3, locals.var_vfbzb_dn4, locals.var_vfbzb_dn5, locals.var_vfbzb_dn6, locals.var_vfbzb_dn7, locals.var_vfbzb_dn8, locals.var_vfbzb_dn9, locals.var_vfbzb_dn10, locals.var_vfbzb_dn11, locals.var_vfbzb_dn12,)
    }
};
        locals.var_vfbzb = assign30610_e24407;
        locals.var_vfbzb_dn3 = assign30610_e24407_d_n3;
        locals.var_vfbzb_dn4 = assign30610_e24407_d_n4;
        locals.var_vfbzb_dn5 = assign30610_e24407_d_n5;
        locals.var_vfbzb_dn6 = assign30610_e24407_d_n6;
        locals.var_vfbzb_dn7 = assign30610_e24407_d_n7;
        locals.var_vfbzb_dn8 = assign30610_e24407_d_n8;
        locals.var_vfbzb_dn9 = assign30610_e24407_d_n9;
        locals.var_vfbzb_dn10 = assign30610_e24407_d_n10;
        locals.var_vfbzb_dn11 = assign30610_e24407_d_n11;
        locals.var_vfbzb_dn12 = assign30610_e24407_d_n12;

        let assign30620_e24414: f64 = if ((p.p33 == 1.0) && (p.p16 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1720 = assign30620_e24414;

        let (assign30630_e24434, assign30630_e24434_d_n3, assign30630_e24434_d_n4, assign30630_e24434_d_n5, assign30630_e24434_d_n6, assign30630_e24434_d_n7, assign30630_e24434_d_n8, assign30630_e24434_d_n9, assign30630_e24434_d_n10, assign30630_e24434_d_n11, assign30630_e24434_d_n12,) = {
    if ((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1720 != 0.0)) {
        let assign30630_e24426: f64 = (locals.var_vthzb - locals.var_phi);
        let assign30630_e24429: f64 = (locals.var_pparam_b4soik1eff * locals.var_sqrtphi);
        let assign30630_e24430: f64 = (assign30630_e24426 - assign30630_e24429);
        let assign30630_e24432: f64 = (assign30630_e24430 + locals.var_pparam_b4soidelvt);
        (assign30630_e24432, (((locals.var_vthzb_dn3 - locals.var_phi_dn3) - ((locals.var_pparam_b4soik1eff_dn3 * locals.var_sqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphi_dn3))) + locals.var_pparam_b4soidelvt_dn3), (((locals.var_vthzb_dn4 - locals.var_phi_dn4) - ((locals.var_pparam_b4soik1eff_dn4 * locals.var_sqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphi_dn4))) + locals.var_pparam_b4soidelvt_dn4), (((locals.var_vthzb_dn5 - locals.var_phi_dn5) - ((locals.var_pparam_b4soik1eff_dn5 * locals.var_sqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphi_dn5))) + locals.var_pparam_b4soidelvt_dn5), (((locals.var_vthzb_dn6 - locals.var_phi_dn6) - ((locals.var_pparam_b4soik1eff_dn6 * locals.var_sqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphi_dn6))) + locals.var_pparam_b4soidelvt_dn6), (((locals.var_vthzb_dn7 - locals.var_phi_dn7) - ((locals.var_pparam_b4soik1eff_dn7 * locals.var_sqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphi_dn7))) + locals.var_pparam_b4soidelvt_dn7), (((locals.var_vthzb_dn8 - locals.var_phi_dn8) - ((locals.var_pparam_b4soik1eff_dn8 * locals.var_sqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphi_dn8))) + locals.var_pparam_b4soidelvt_dn8), (((locals.var_vthzb_dn9 - locals.var_phi_dn9) - ((locals.var_pparam_b4soik1eff_dn9 * locals.var_sqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphi_dn9))) + locals.var_pparam_b4soidelvt_dn9), (((locals.var_vthzb_dn10 - locals.var_phi_dn10) - ((locals.var_pparam_b4soik1eff_dn10 * locals.var_sqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphi_dn10))) + locals.var_pparam_b4soidelvt_dn10), (((locals.var_vthzb_dn11 - locals.var_phi_dn11) - ((locals.var_pparam_b4soik1eff_dn11 * locals.var_sqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphi_dn11))) + locals.var_pparam_b4soidelvt_dn11), (((locals.var_vthzb_dn12 - locals.var_phi_dn12) - ((locals.var_pparam_b4soik1eff_dn12 * locals.var_sqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphi_dn12))) + locals.var_pparam_b4soidelvt_dn12),)
    } else {
        (locals.var_vfbzb, locals.var_vfbzb_dn3, locals.var_vfbzb_dn4, locals.var_vfbzb_dn5, locals.var_vfbzb_dn6, locals.var_vfbzb_dn7, locals.var_vfbzb_dn8, locals.var_vfbzb_dn9, locals.var_vfbzb_dn10, locals.var_vfbzb_dn11, locals.var_vfbzb_dn12,)
    }
};
        locals.var_vfbzb = assign30630_e24434;
        locals.var_vfbzb_dn3 = assign30630_e24434_d_n3;
        locals.var_vfbzb_dn4 = assign30630_e24434_d_n4;
        locals.var_vfbzb_dn5 = assign30630_e24434_d_n5;
        locals.var_vfbzb_dn6 = assign30630_e24434_d_n6;
        locals.var_vfbzb_dn7 = assign30630_e24434_d_n7;
        locals.var_vfbzb_dn8 = assign30630_e24434_d_n8;
        locals.var_vfbzb_dn9 = assign30630_e24434_d_n9;
        locals.var_vfbzb_dn10 = assign30630_e24434_d_n10;
        locals.var_vfbzb_dn11 = assign30630_e24434_d_n11;
        locals.var_vfbzb_dn12 = assign30630_e24434_d_n12;

    }

    pub(super) fn stamp_transient_block_83(
        locals: &mut StampLocals,
    ) {
        let (assign30640_e24449, assign30640_e24449_d_n3, assign30640_e24449_d_n4, assign30640_e24449_d_n5, assign30640_e24449_d_n6, assign30640_e24449_d_n7, assign30640_e24449_d_n8, assign30640_e24449_d_n9, assign30640_e24449_d_n10, assign30640_e24449_d_n11, assign30640_e24449_d_n12,) = {
    if ((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1720 == 0.0)) {
        let assign30640_e24447: f64 = (locals.var_b4soivfbzb + locals.var_pparam_b4soidelvt);
        (assign30640_e24447, (locals.var_b4soivfbzb_dn3 + locals.var_pparam_b4soidelvt_dn3), (locals.var_b4soivfbzb_dn4 + locals.var_pparam_b4soidelvt_dn4), (locals.var_b4soivfbzb_dn5 + locals.var_pparam_b4soidelvt_dn5), (locals.var_b4soivfbzb_dn6 + locals.var_pparam_b4soidelvt_dn6), (locals.var_b4soivfbzb_dn7 + locals.var_pparam_b4soidelvt_dn7), (locals.var_b4soivfbzb_dn8 + locals.var_pparam_b4soidelvt_dn8), (locals.var_b4soivfbzb_dn9 + locals.var_pparam_b4soidelvt_dn9), (locals.var_b4soivfbzb_dn10 + locals.var_pparam_b4soidelvt_dn10), (locals.var_b4soivfbzb_dn11 + locals.var_pparam_b4soidelvt_dn11), (locals.var_b4soivfbzb_dn12 + locals.var_pparam_b4soidelvt_dn12),)
    } else {
        (locals.var_vfbzb, locals.var_vfbzb_dn3, locals.var_vfbzb_dn4, locals.var_vfbzb_dn5, locals.var_vfbzb_dn6, locals.var_vfbzb_dn7, locals.var_vfbzb_dn8, locals.var_vfbzb_dn9, locals.var_vfbzb_dn10, locals.var_vfbzb_dn11, locals.var_vfbzb_dn12,)
    }
};
        locals.var_vfbzb = assign30640_e24449;
        locals.var_vfbzb_dn3 = assign30640_e24449_d_n3;
        locals.var_vfbzb_dn4 = assign30640_e24449_d_n4;
        locals.var_vfbzb_dn5 = assign30640_e24449_d_n5;
        locals.var_vfbzb_dn6 = assign30640_e24449_d_n6;
        locals.var_vfbzb_dn7 = assign30640_e24449_d_n7;
        locals.var_vfbzb_dn8 = assign30640_e24449_d_n8;
        locals.var_vfbzb_dn9 = assign30640_e24449_d_n9;
        locals.var_vfbzb_dn10 = assign30640_e24449_d_n10;
        locals.var_vfbzb_dn11 = assign30640_e24449_d_n11;
        locals.var_vfbzb_dn12 = assign30640_e24449_d_n12;

        let (assign30650_e24465, assign30650_e24465_d_n3, assign30650_e24465_d_n4, assign30650_e24465_d_n5, assign30650_e24465_d_n6, assign30650_e24465_d_n7, assign30650_e24465_d_n8, assign30650_e24465_d_n9, assign30650_e24465_d_n10, assign30650_e24465_d_n11, assign30650_e24465_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) {
        let assign30650_e24459: f64 = (locals.var_vfbzb - locals.var_vgs_eff__blk1126);
        let assign30650_e24461: f64 = (assign30650_e24459 + locals.var_vbseff);
        let assign30650_e24463: f64 = (assign30650_e24461 - 0.02);
        (assign30650_e24463, ((locals.var_vfbzb_dn3 - locals.var_vgs_eff__blk1126_dn3) + locals.var_vbseff_dn3), ((locals.var_vfbzb_dn4 - locals.var_vgs_eff__blk1126_dn4) + locals.var_vbseff_dn4), ((locals.var_vfbzb_dn5 - locals.var_vgs_eff__blk1126_dn5) + locals.var_vbseff_dn5), ((locals.var_vfbzb_dn6 - locals.var_vgs_eff__blk1126_dn6) + locals.var_vbseff_dn6), ((locals.var_vfbzb_dn7 - locals.var_vgs_eff__blk1126_dn7) + locals.var_vbseff_dn7), ((locals.var_vfbzb_dn8 - locals.var_vgs_eff__blk1126_dn8) + locals.var_vbseff_dn8), ((locals.var_vfbzb_dn9 - locals.var_vgs_eff__blk1126_dn9) + locals.var_vbseff_dn9), ((locals.var_vfbzb_dn10 - locals.var_vgs_eff__blk1126_dn10) + locals.var_vbseff_dn10), ((locals.var_vfbzb_dn11 - locals.var_vgs_eff__blk1126_dn11) + locals.var_vbseff_dn11), ((locals.var_vfbzb_dn12 - locals.var_vgs_eff__blk1126_dn12) + locals.var_vbseff_dn12),)
    } else {
        (locals.var_v3, locals.var_v3_dn3, locals.var_v3_dn4, locals.var_v3_dn5, locals.var_v3_dn6, locals.var_v3_dn7, locals.var_v3_dn8, locals.var_v3_dn9, locals.var_v3_dn10, locals.var_v3_dn11, locals.var_v3_dn12,)
    }
};
        locals.var_v3 = assign30650_e24465;
        locals.var_v3_dn3 = assign30650_e24465_d_n3;
        locals.var_v3_dn4 = assign30650_e24465_d_n4;
        locals.var_v3_dn5 = assign30650_e24465_d_n5;
        locals.var_v3_dn6 = assign30650_e24465_d_n6;
        locals.var_v3_dn7 = assign30650_e24465_d_n7;
        locals.var_v3_dn8 = assign30650_e24465_d_n8;
        locals.var_v3_dn9 = assign30650_e24465_d_n9;
        locals.var_v3_dn10 = assign30650_e24465_d_n10;
        locals.var_v3_dn11 = assign30650_e24465_d_n11;
        locals.var_v3_dn12 = assign30650_e24465_d_n12;

        let assign30660_e24468: f64 = if locals.var_vfbzb <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1721 = assign30660_e24468;

        let (assign30670_e24489, assign30670_e24489_d_n3, assign30670_e24489_d_n4, assign30670_e24489_d_n5, assign30670_e24489_d_n6, assign30670_e24489_d_n7, assign30670_e24489_d_n8, assign30670_e24489_d_n9, assign30670_e24489_d_n10, assign30670_e24489_d_n11, assign30670_e24489_d_n12,) = {
    if ((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1721 != 0.0)) {
        let assign30670_e24480: f64 = (locals.var_v3 * locals.var_v3);
        let assign30670_e24483: f64 = (4.0 * 0.02);
        let assign30670_e24485: f64 = (assign30670_e24483 * locals.var_vfbzb);
        let assign30670_e24486: f64 = (assign30670_e24480 - assign30670_e24485);
        let assign30670_e24487: f64 = (assign30670_e24486).sqrt();
        (assign30670_e24487, ((((locals.var_v3_dn3 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn3)) - (assign30670_e24483 * locals.var_vfbzb_dn3)) / (2.0 * assign30670_e24487)), ((((locals.var_v3_dn4 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn4)) - (assign30670_e24483 * locals.var_vfbzb_dn4)) / (2.0 * assign30670_e24487)), ((((locals.var_v3_dn5 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn5)) - (assign30670_e24483 * locals.var_vfbzb_dn5)) / (2.0 * assign30670_e24487)), ((((locals.var_v3_dn6 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn6)) - (assign30670_e24483 * locals.var_vfbzb_dn6)) / (2.0 * assign30670_e24487)), ((((locals.var_v3_dn7 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn7)) - (assign30670_e24483 * locals.var_vfbzb_dn7)) / (2.0 * assign30670_e24487)), ((((locals.var_v3_dn8 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn8)) - (assign30670_e24483 * locals.var_vfbzb_dn8)) / (2.0 * assign30670_e24487)), ((((locals.var_v3_dn9 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn9)) - (assign30670_e24483 * locals.var_vfbzb_dn9)) / (2.0 * assign30670_e24487)), ((((locals.var_v3_dn10 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn10)) - (assign30670_e24483 * locals.var_vfbzb_dn10)) / (2.0 * assign30670_e24487)), ((((locals.var_v3_dn11 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn11)) - (assign30670_e24483 * locals.var_vfbzb_dn11)) / (2.0 * assign30670_e24487)), ((((locals.var_v3_dn12 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn12)) - (assign30670_e24483 * locals.var_vfbzb_dn12)) / (2.0 * assign30670_e24487)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign30670_e24489;
        locals.var_t0__blk1144_dn3 = assign30670_e24489_d_n3;
        locals.var_t0__blk1144_dn4 = assign30670_e24489_d_n4;
        locals.var_t0__blk1144_dn5 = assign30670_e24489_d_n5;
        locals.var_t0__blk1144_dn6 = assign30670_e24489_d_n6;
        locals.var_t0__blk1144_dn7 = assign30670_e24489_d_n7;
        locals.var_t0__blk1144_dn8 = assign30670_e24489_d_n8;
        locals.var_t0__blk1144_dn9 = assign30670_e24489_d_n9;
        locals.var_t0__blk1144_dn10 = assign30670_e24489_d_n10;
        locals.var_t0__blk1144_dn11 = assign30670_e24489_d_n11;
        locals.var_t0__blk1144_dn12 = assign30670_e24489_d_n12;

        let (assign30680_e24511, assign30680_e24511_d_n3, assign30680_e24511_d_n4, assign30680_e24511_d_n5, assign30680_e24511_d_n6, assign30680_e24511_d_n7, assign30680_e24511_d_n8, assign30680_e24511_d_n9, assign30680_e24511_d_n10, assign30680_e24511_d_n11, assign30680_e24511_d_n12,) = {
    if ((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1721 == 0.0)) {
        let assign30680_e24502: f64 = (locals.var_v3 * locals.var_v3);
        let assign30680_e24505: f64 = (4.0 * 0.02);
        let assign30680_e24507: f64 = (assign30680_e24505 * locals.var_vfbzb);
        let assign30680_e24508: f64 = (assign30680_e24502 + assign30680_e24507);
        let assign30680_e24509: f64 = (assign30680_e24508).sqrt();
        (assign30680_e24509, ((((locals.var_v3_dn3 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn3)) + (assign30680_e24505 * locals.var_vfbzb_dn3)) / (2.0 * assign30680_e24509)), ((((locals.var_v3_dn4 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn4)) + (assign30680_e24505 * locals.var_vfbzb_dn4)) / (2.0 * assign30680_e24509)), ((((locals.var_v3_dn5 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn5)) + (assign30680_e24505 * locals.var_vfbzb_dn5)) / (2.0 * assign30680_e24509)), ((((locals.var_v3_dn6 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn6)) + (assign30680_e24505 * locals.var_vfbzb_dn6)) / (2.0 * assign30680_e24509)), ((((locals.var_v3_dn7 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn7)) + (assign30680_e24505 * locals.var_vfbzb_dn7)) / (2.0 * assign30680_e24509)), ((((locals.var_v3_dn8 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn8)) + (assign30680_e24505 * locals.var_vfbzb_dn8)) / (2.0 * assign30680_e24509)), ((((locals.var_v3_dn9 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn9)) + (assign30680_e24505 * locals.var_vfbzb_dn9)) / (2.0 * assign30680_e24509)), ((((locals.var_v3_dn10 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn10)) + (assign30680_e24505 * locals.var_vfbzb_dn10)) / (2.0 * assign30680_e24509)), ((((locals.var_v3_dn11 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn11)) + (assign30680_e24505 * locals.var_vfbzb_dn11)) / (2.0 * assign30680_e24509)), ((((locals.var_v3_dn12 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn12)) + (assign30680_e24505 * locals.var_vfbzb_dn12)) / (2.0 * assign30680_e24509)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign30680_e24511;
        locals.var_t0__blk1144_dn3 = assign30680_e24511_d_n3;
        locals.var_t0__blk1144_dn4 = assign30680_e24511_d_n4;
        locals.var_t0__blk1144_dn5 = assign30680_e24511_d_n5;
        locals.var_t0__blk1144_dn6 = assign30680_e24511_d_n6;
        locals.var_t0__blk1144_dn7 = assign30680_e24511_d_n7;
        locals.var_t0__blk1144_dn8 = assign30680_e24511_d_n8;
        locals.var_t0__blk1144_dn9 = assign30680_e24511_d_n9;
        locals.var_t0__blk1144_dn10 = assign30680_e24511_d_n10;
        locals.var_t0__blk1144_dn11 = assign30680_e24511_d_n11;
        locals.var_t0__blk1144_dn12 = assign30680_e24511_d_n12;

        let (assign30690_e24527, assign30690_e24527_d_n3, assign30690_e24527_d_n4, assign30690_e24527_d_n5, assign30690_e24527_d_n6, assign30690_e24527_d_n7, assign30690_e24527_d_n8, assign30690_e24527_d_n9, assign30690_e24527_d_n10, assign30690_e24527_d_n11, assign30690_e24527_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) {
        let assign30690_e24523: f64 = (locals.var_v3 + locals.var_t0__blk1144);
        let assign30690_e24524: f64 = (0.5 * assign30690_e24523);
        let assign30690_e24525: f64 = (locals.var_vfbzb - assign30690_e24524);
        (assign30690_e24525, (locals.var_vfbzb_dn3 - (0.5 * (locals.var_v3_dn3 + locals.var_t0__blk1144_dn3))), (locals.var_vfbzb_dn4 - (0.5 * (locals.var_v3_dn4 + locals.var_t0__blk1144_dn4))), (locals.var_vfbzb_dn5 - (0.5 * (locals.var_v3_dn5 + locals.var_t0__blk1144_dn5))), (locals.var_vfbzb_dn6 - (0.5 * (locals.var_v3_dn6 + locals.var_t0__blk1144_dn6))), (locals.var_vfbzb_dn7 - (0.5 * (locals.var_v3_dn7 + locals.var_t0__blk1144_dn7))), (locals.var_vfbzb_dn8 - (0.5 * (locals.var_v3_dn8 + locals.var_t0__blk1144_dn8))), (locals.var_vfbzb_dn9 - (0.5 * (locals.var_v3_dn9 + locals.var_t0__blk1144_dn9))), (locals.var_vfbzb_dn10 - (0.5 * (locals.var_v3_dn10 + locals.var_t0__blk1144_dn10))), (locals.var_vfbzb_dn11 - (0.5 * (locals.var_v3_dn11 + locals.var_t0__blk1144_dn11))), (locals.var_vfbzb_dn12 - (0.5 * (locals.var_v3_dn12 + locals.var_t0__blk1144_dn12))),)
    } else {
        (locals.var_vfbeff, locals.var_vfbeff_dn3, locals.var_vfbeff_dn4, locals.var_vfbeff_dn5, locals.var_vfbeff_dn6, locals.var_vfbeff_dn7, locals.var_vfbeff_dn8, locals.var_vfbeff_dn9, locals.var_vfbeff_dn10, locals.var_vfbeff_dn11, locals.var_vfbeff_dn12,)
    }
};
        locals.var_vfbeff = assign30690_e24527;
        locals.var_vfbeff_dn3 = assign30690_e24527_d_n3;
        locals.var_vfbeff_dn4 = assign30690_e24527_d_n4;
        locals.var_vfbeff_dn5 = assign30690_e24527_d_n5;
        locals.var_vfbeff_dn6 = assign30690_e24527_d_n6;
        locals.var_vfbeff_dn7 = assign30690_e24527_d_n7;
        locals.var_vfbeff_dn8 = assign30690_e24527_d_n8;
        locals.var_vfbeff_dn9 = assign30690_e24527_d_n9;
        locals.var_vfbeff_dn10 = assign30690_e24527_d_n10;
        locals.var_vfbeff_dn11 = assign30690_e24527_d_n11;
        locals.var_vfbeff_dn12 = assign30690_e24527_d_n12;

        let assign30700_e24530: f64 = if locals.var_b4soiagbcp2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1722 = assign30700_e24530;

        let (assign30710_e24544, assign30710_e24544_d_n3, assign30710_e24544_d_n4, assign30710_e24544_d_n5, assign30710_e24544_d_n6, assign30710_e24544_d_n7, assign30710_e24544_d_n8, assign30710_e24544_d_n9, assign30710_e24544_d_n10, assign30710_e24544_d_n11, assign30710_e24544_d_n12,) = {
    if ((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1722 != 0.0)) {
        let assign30710_e24542: f64 = (locals.var_vfbzb + locals.var_eggbcp2);
        (assign30710_e24542, locals.var_vfbzb_dn3, locals.var_vfbzb_dn4, locals.var_vfbzb_dn5, locals.var_vfbzb_dn6, locals.var_vfbzb_dn7, locals.var_vfbzb_dn8, locals.var_vfbzb_dn9, locals.var_vfbzb_dn10, locals.var_vfbzb_dn11, locals.var_vfbzb_dn12,)
    } else {
        (locals.var_vfbzb2, locals.var_vfbzb2_dn3, locals.var_vfbzb2_dn4, locals.var_vfbzb2_dn5, locals.var_vfbzb2_dn6, locals.var_vfbzb2_dn7, locals.var_vfbzb2_dn8, locals.var_vfbzb2_dn9, locals.var_vfbzb2_dn10, locals.var_vfbzb2_dn11, locals.var_vfbzb2_dn12,)
    }
};
        locals.var_vfbzb2 = assign30710_e24544;
        locals.var_vfbzb2_dn3 = assign30710_e24544_d_n3;
        locals.var_vfbzb2_dn4 = assign30710_e24544_d_n4;
        locals.var_vfbzb2_dn5 = assign30710_e24544_d_n5;
        locals.var_vfbzb2_dn6 = assign30710_e24544_d_n6;
        locals.var_vfbzb2_dn7 = assign30710_e24544_d_n7;
        locals.var_vfbzb2_dn8 = assign30710_e24544_d_n8;
        locals.var_vfbzb2_dn9 = assign30710_e24544_d_n9;
        locals.var_vfbzb2_dn10 = assign30710_e24544_d_n10;
        locals.var_vfbzb2_dn11 = assign30710_e24544_d_n11;
        locals.var_vfbzb2_dn12 = assign30710_e24544_d_n12;

        let (assign30720_e24562, assign30720_e24562_d_n3, assign30720_e24562_d_n4, assign30720_e24562_d_n5, assign30720_e24562_d_n6, assign30720_e24562_d_n7, assign30720_e24562_d_n8, assign30720_e24562_d_n9, assign30720_e24562_d_n10, assign30720_e24562_d_n11, assign30720_e24562_d_n12,) = {
    if ((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1722 != 0.0)) {
        let assign30720_e24556: f64 = (locals.var_vfbzb2 - locals.var_vgs_eff2);
        let assign30720_e24558: f64 = (assign30720_e24556 + locals.var_vbseff);
        let assign30720_e24560: f64 = (assign30720_e24558 - 0.02);
        (assign30720_e24560, (locals.var_vfbzb2_dn3 + locals.var_vbseff_dn3), (locals.var_vfbzb2_dn4 + locals.var_vbseff_dn4), (locals.var_vfbzb2_dn5 + locals.var_vbseff_dn5), (locals.var_vfbzb2_dn6 + locals.var_vbseff_dn6), ((locals.var_vfbzb2_dn7 - locals.var_vgs_eff2_dn7) + locals.var_vbseff_dn7), ((locals.var_vfbzb2_dn8 - locals.var_vgs_eff2_dn8) + locals.var_vbseff_dn8), ((locals.var_vfbzb2_dn9 - locals.var_vgs_eff2_dn9) + locals.var_vbseff_dn9), (locals.var_vfbzb2_dn10 + locals.var_vbseff_dn10), (locals.var_vfbzb2_dn11 + locals.var_vbseff_dn11), (locals.var_vfbzb2_dn12 + locals.var_vbseff_dn12),)
    } else {
        (locals.var_v3, locals.var_v3_dn3, locals.var_v3_dn4, locals.var_v3_dn5, locals.var_v3_dn6, locals.var_v3_dn7, locals.var_v3_dn8, locals.var_v3_dn9, locals.var_v3_dn10, locals.var_v3_dn11, locals.var_v3_dn12,)
    }
};
        locals.var_v3 = assign30720_e24562;
        locals.var_v3_dn3 = assign30720_e24562_d_n3;
        locals.var_v3_dn4 = assign30720_e24562_d_n4;
        locals.var_v3_dn5 = assign30720_e24562_d_n5;
        locals.var_v3_dn6 = assign30720_e24562_d_n6;
        locals.var_v3_dn7 = assign30720_e24562_d_n7;
        locals.var_v3_dn8 = assign30720_e24562_d_n8;
        locals.var_v3_dn9 = assign30720_e24562_d_n9;
        locals.var_v3_dn10 = assign30720_e24562_d_n10;
        locals.var_v3_dn11 = assign30720_e24562_d_n11;
        locals.var_v3_dn12 = assign30720_e24562_d_n12;

        let assign30730_e24565: f64 = if locals.var_vfbzb2 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1723 = assign30730_e24565;

        let (assign30740_e24588, assign30740_e24588_d_n3, assign30740_e24588_d_n4, assign30740_e24588_d_n5, assign30740_e24588_d_n6, assign30740_e24588_d_n7, assign30740_e24588_d_n8, assign30740_e24588_d_n9, assign30740_e24588_d_n10, assign30740_e24588_d_n11, assign30740_e24588_d_n12,) = {
    if (((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1723 != 0.0)) {
        let assign30740_e24579: f64 = (locals.var_v3 * locals.var_v3);
        let assign30740_e24582: f64 = (100.0 * 0.02);
        let assign30740_e24584: f64 = (assign30740_e24582 * locals.var_vfbzb2);
        let assign30740_e24585: f64 = (assign30740_e24579 - assign30740_e24584);
        let assign30740_e24586: f64 = (assign30740_e24585).sqrt();
        (assign30740_e24586, ((((locals.var_v3_dn3 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn3)) - (assign30740_e24582 * locals.var_vfbzb2_dn3)) / (2.0 * assign30740_e24586)), ((((locals.var_v3_dn4 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn4)) - (assign30740_e24582 * locals.var_vfbzb2_dn4)) / (2.0 * assign30740_e24586)), ((((locals.var_v3_dn5 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn5)) - (assign30740_e24582 * locals.var_vfbzb2_dn5)) / (2.0 * assign30740_e24586)), ((((locals.var_v3_dn6 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn6)) - (assign30740_e24582 * locals.var_vfbzb2_dn6)) / (2.0 * assign30740_e24586)), ((((locals.var_v3_dn7 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn7)) - (assign30740_e24582 * locals.var_vfbzb2_dn7)) / (2.0 * assign30740_e24586)), ((((locals.var_v3_dn8 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn8)) - (assign30740_e24582 * locals.var_vfbzb2_dn8)) / (2.0 * assign30740_e24586)), ((((locals.var_v3_dn9 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn9)) - (assign30740_e24582 * locals.var_vfbzb2_dn9)) / (2.0 * assign30740_e24586)), ((((locals.var_v3_dn10 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn10)) - (assign30740_e24582 * locals.var_vfbzb2_dn10)) / (2.0 * assign30740_e24586)), ((((locals.var_v3_dn11 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn11)) - (assign30740_e24582 * locals.var_vfbzb2_dn11)) / (2.0 * assign30740_e24586)), ((((locals.var_v3_dn12 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn12)) - (assign30740_e24582 * locals.var_vfbzb2_dn12)) / (2.0 * assign30740_e24586)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign30740_e24588;
        locals.var_t0__blk1144_dn3 = assign30740_e24588_d_n3;
        locals.var_t0__blk1144_dn4 = assign30740_e24588_d_n4;
        locals.var_t0__blk1144_dn5 = assign30740_e24588_d_n5;
        locals.var_t0__blk1144_dn6 = assign30740_e24588_d_n6;
        locals.var_t0__blk1144_dn7 = assign30740_e24588_d_n7;
        locals.var_t0__blk1144_dn8 = assign30740_e24588_d_n8;
        locals.var_t0__blk1144_dn9 = assign30740_e24588_d_n9;
        locals.var_t0__blk1144_dn10 = assign30740_e24588_d_n10;
        locals.var_t0__blk1144_dn11 = assign30740_e24588_d_n11;
        locals.var_t0__blk1144_dn12 = assign30740_e24588_d_n12;

        let (assign30750_e24612, assign30750_e24612_d_n3, assign30750_e24612_d_n4, assign30750_e24612_d_n5, assign30750_e24612_d_n6, assign30750_e24612_d_n7, assign30750_e24612_d_n8, assign30750_e24612_d_n9, assign30750_e24612_d_n10, assign30750_e24612_d_n11, assign30750_e24612_d_n12,) = {
    if (((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1722 != 0.0)) && (locals.var_guard1723 == 0.0)) {
        let assign30750_e24603: f64 = (locals.var_v3 * locals.var_v3);
        let assign30750_e24606: f64 = (100.0 * 0.02);
        let assign30750_e24608: f64 = (assign30750_e24606 * locals.var_vfbzb2);
        let assign30750_e24609: f64 = (assign30750_e24603 + assign30750_e24608);
        let assign30750_e24610: f64 = (assign30750_e24609).sqrt();
        (assign30750_e24610, ((((locals.var_v3_dn3 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn3)) + (assign30750_e24606 * locals.var_vfbzb2_dn3)) / (2.0 * assign30750_e24610)), ((((locals.var_v3_dn4 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn4)) + (assign30750_e24606 * locals.var_vfbzb2_dn4)) / (2.0 * assign30750_e24610)), ((((locals.var_v3_dn5 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn5)) + (assign30750_e24606 * locals.var_vfbzb2_dn5)) / (2.0 * assign30750_e24610)), ((((locals.var_v3_dn6 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn6)) + (assign30750_e24606 * locals.var_vfbzb2_dn6)) / (2.0 * assign30750_e24610)), ((((locals.var_v3_dn7 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn7)) + (assign30750_e24606 * locals.var_vfbzb2_dn7)) / (2.0 * assign30750_e24610)), ((((locals.var_v3_dn8 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn8)) + (assign30750_e24606 * locals.var_vfbzb2_dn8)) / (2.0 * assign30750_e24610)), ((((locals.var_v3_dn9 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn9)) + (assign30750_e24606 * locals.var_vfbzb2_dn9)) / (2.0 * assign30750_e24610)), ((((locals.var_v3_dn10 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn10)) + (assign30750_e24606 * locals.var_vfbzb2_dn10)) / (2.0 * assign30750_e24610)), ((((locals.var_v3_dn11 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn11)) + (assign30750_e24606 * locals.var_vfbzb2_dn11)) / (2.0 * assign30750_e24610)), ((((locals.var_v3_dn12 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn12)) + (assign30750_e24606 * locals.var_vfbzb2_dn12)) / (2.0 * assign30750_e24610)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign30750_e24612;
        locals.var_t0__blk1144_dn3 = assign30750_e24612_d_n3;
        locals.var_t0__blk1144_dn4 = assign30750_e24612_d_n4;
        locals.var_t0__blk1144_dn5 = assign30750_e24612_d_n5;
        locals.var_t0__blk1144_dn6 = assign30750_e24612_d_n6;
        locals.var_t0__blk1144_dn7 = assign30750_e24612_d_n7;
        locals.var_t0__blk1144_dn8 = assign30750_e24612_d_n8;
        locals.var_t0__blk1144_dn9 = assign30750_e24612_d_n9;
        locals.var_t0__blk1144_dn10 = assign30750_e24612_d_n10;
        locals.var_t0__blk1144_dn11 = assign30750_e24612_d_n11;
        locals.var_t0__blk1144_dn12 = assign30750_e24612_d_n12;

        let (assign30760_e24630, assign30760_e24630_d_n3, assign30760_e24630_d_n4, assign30760_e24630_d_n5, assign30760_e24630_d_n6, assign30760_e24630_d_n7, assign30760_e24630_d_n8, assign30760_e24630_d_n9, assign30760_e24630_d_n10, assign30760_e24630_d_n11, assign30760_e24630_d_n12,) = {
    if ((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1722 != 0.0)) {
        let assign30760_e24626: f64 = (locals.var_v3 + locals.var_t0__blk1144);
        let assign30760_e24627: f64 = (0.5 * assign30760_e24626);
        let assign30760_e24628: f64 = (locals.var_vfbzb2 - assign30760_e24627);
        (assign30760_e24628, (locals.var_vfbzb2_dn3 - (0.5 * (locals.var_v3_dn3 + locals.var_t0__blk1144_dn3))), (locals.var_vfbzb2_dn4 - (0.5 * (locals.var_v3_dn4 + locals.var_t0__blk1144_dn4))), (locals.var_vfbzb2_dn5 - (0.5 * (locals.var_v3_dn5 + locals.var_t0__blk1144_dn5))), (locals.var_vfbzb2_dn6 - (0.5 * (locals.var_v3_dn6 + locals.var_t0__blk1144_dn6))), (locals.var_vfbzb2_dn7 - (0.5 * (locals.var_v3_dn7 + locals.var_t0__blk1144_dn7))), (locals.var_vfbzb2_dn8 - (0.5 * (locals.var_v3_dn8 + locals.var_t0__blk1144_dn8))), (locals.var_vfbzb2_dn9 - (0.5 * (locals.var_v3_dn9 + locals.var_t0__blk1144_dn9))), (locals.var_vfbzb2_dn10 - (0.5 * (locals.var_v3_dn10 + locals.var_t0__blk1144_dn10))), (locals.var_vfbzb2_dn11 - (0.5 * (locals.var_v3_dn11 + locals.var_t0__blk1144_dn11))), (locals.var_vfbzb2_dn12 - (0.5 * (locals.var_v3_dn12 + locals.var_t0__blk1144_dn12))),)
    } else {
        (locals.var_vfbeff2, locals.var_vfbeff2_dn3, locals.var_vfbeff2_dn4, locals.var_vfbeff2_dn5, locals.var_vfbeff2_dn6, locals.var_vfbeff2_dn7, locals.var_vfbeff2_dn8, locals.var_vfbeff2_dn9, locals.var_vfbeff2_dn10, locals.var_vfbeff2_dn11, locals.var_vfbeff2_dn12,)
    }
};
        locals.var_vfbeff2 = assign30760_e24630;
        locals.var_vfbeff2_dn3 = assign30760_e24630_d_n3;
        locals.var_vfbeff2_dn4 = assign30760_e24630_d_n4;
        locals.var_vfbeff2_dn5 = assign30760_e24630_d_n5;
        locals.var_vfbeff2_dn6 = assign30760_e24630_d_n6;
        locals.var_vfbeff2_dn7 = assign30760_e24630_d_n7;
        locals.var_vfbeff2_dn8 = assign30760_e24630_d_n8;
        locals.var_vfbeff2_dn9 = assign30760_e24630_d_n9;
        locals.var_vfbeff2_dn10 = assign30760_e24630_d_n10;
        locals.var_vfbeff2_dn11 = assign30760_e24630_d_n11;
        locals.var_vfbeff2_dn12 = assign30760_e24630_d_n12;

        let (assign30770_e24646, assign30770_e24646_d_n3, assign30770_e24646_d_n4, assign30770_e24646_d_n5, assign30770_e24646_d_n6, assign30770_e24646_d_n7, assign30770_e24646_d_n8, assign30770_e24646_d_n9, assign30770_e24646_d_n10, assign30770_e24646_d_n11, assign30770_e24646_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) {
        let assign30770_e24640: f64 = (locals.var_vgs_eff__blk1126 - locals.var_vbseff);
        let assign30770_e24642: f64 = (assign30770_e24640 - locals.var_vfbzb);
        let assign30770_e24644: f64 = (assign30770_e24642 / locals.var_tox);
        (assign30770_e24644, (((((locals.var_vgs_eff__blk1126_dn3 - locals.var_vbseff_dn3) - locals.var_vfbzb_dn3) * locals.var_tox) - (assign30770_e24642 * locals.var_tox_dn3)) / (locals.var_tox * locals.var_tox)), (((((locals.var_vgs_eff__blk1126_dn4 - locals.var_vbseff_dn4) - locals.var_vfbzb_dn4) * locals.var_tox) - (assign30770_e24642 * locals.var_tox_dn4)) / (locals.var_tox * locals.var_tox)), (((((locals.var_vgs_eff__blk1126_dn5 - locals.var_vbseff_dn5) - locals.var_vfbzb_dn5) * locals.var_tox) - (assign30770_e24642 * locals.var_tox_dn5)) / (locals.var_tox * locals.var_tox)), (((((locals.var_vgs_eff__blk1126_dn6 - locals.var_vbseff_dn6) - locals.var_vfbzb_dn6) * locals.var_tox) - (assign30770_e24642 * locals.var_tox_dn6)) / (locals.var_tox * locals.var_tox)), (((((locals.var_vgs_eff__blk1126_dn7 - locals.var_vbseff_dn7) - locals.var_vfbzb_dn7) * locals.var_tox) - (assign30770_e24642 * locals.var_tox_dn7)) / (locals.var_tox * locals.var_tox)), (((((locals.var_vgs_eff__blk1126_dn8 - locals.var_vbseff_dn8) - locals.var_vfbzb_dn8) * locals.var_tox) - (assign30770_e24642 * locals.var_tox_dn8)) / (locals.var_tox * locals.var_tox)), (((((locals.var_vgs_eff__blk1126_dn9 - locals.var_vbseff_dn9) - locals.var_vfbzb_dn9) * locals.var_tox) - (assign30770_e24642 * locals.var_tox_dn9)) / (locals.var_tox * locals.var_tox)), (((((locals.var_vgs_eff__blk1126_dn10 - locals.var_vbseff_dn10) - locals.var_vfbzb_dn10) * locals.var_tox) - (assign30770_e24642 * locals.var_tox_dn10)) / (locals.var_tox * locals.var_tox)), (((((locals.var_vgs_eff__blk1126_dn11 - locals.var_vbseff_dn11) - locals.var_vfbzb_dn11) * locals.var_tox) - (assign30770_e24642 * locals.var_tox_dn11)) / (locals.var_tox * locals.var_tox)), (((((locals.var_vgs_eff__blk1126_dn12 - locals.var_vbseff_dn12) - locals.var_vfbzb_dn12) * locals.var_tox) - (assign30770_e24642 * locals.var_tox_dn12)) / (locals.var_tox * locals.var_tox)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign30770_e24646;
        locals.var_t0__blk1144_dn3 = assign30770_e24646_d_n3;
        locals.var_t0__blk1144_dn4 = assign30770_e24646_d_n4;
        locals.var_t0__blk1144_dn5 = assign30770_e24646_d_n5;
        locals.var_t0__blk1144_dn6 = assign30770_e24646_d_n6;
        locals.var_t0__blk1144_dn7 = assign30770_e24646_d_n7;
        locals.var_t0__blk1144_dn8 = assign30770_e24646_d_n8;
        locals.var_t0__blk1144_dn9 = assign30770_e24646_d_n9;
        locals.var_t0__blk1144_dn10 = assign30770_e24646_d_n10;
        locals.var_t0__blk1144_dn11 = assign30770_e24646_d_n11;
        locals.var_t0__blk1144_dn12 = assign30770_e24646_d_n12;

        let (assign30780_e24658, assign30780_e24658_d_n3, assign30780_e24658_d_n4, assign30780_e24658_d_n5, assign30780_e24658_d_n6, assign30780_e24658_d_n7, assign30780_e24658_d_n8, assign30780_e24658_d_n9, assign30780_e24658_d_n10, assign30780_e24658_d_n11, assign30780_e24658_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) {
        let assign30780_e24656: f64 = (locals.var_t0__blk1144 * locals.var_pparam_b4soiacde);
        (assign30780_e24656, ((locals.var_t0__blk1144_dn3 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk1144 * locals.var_pparam_b4soiacde_dn3)), ((locals.var_t0__blk1144_dn4 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk1144 * locals.var_pparam_b4soiacde_dn4)), ((locals.var_t0__blk1144_dn5 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk1144 * locals.var_pparam_b4soiacde_dn5)), ((locals.var_t0__blk1144_dn6 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk1144 * locals.var_pparam_b4soiacde_dn6)), ((locals.var_t0__blk1144_dn7 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk1144 * locals.var_pparam_b4soiacde_dn7)), ((locals.var_t0__blk1144_dn8 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk1144 * locals.var_pparam_b4soiacde_dn8)), ((locals.var_t0__blk1144_dn9 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk1144 * locals.var_pparam_b4soiacde_dn9)), ((locals.var_t0__blk1144_dn10 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk1144 * locals.var_pparam_b4soiacde_dn10)), ((locals.var_t0__blk1144_dn11 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk1144 * locals.var_pparam_b4soiacde_dn11)), ((locals.var_t0__blk1144_dn12 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk1144 * locals.var_pparam_b4soiacde_dn12)),)
    } else {
        (locals.var_tmp__blk1159, locals.var_tmp__blk1159_dn3, locals.var_tmp__blk1159_dn4, locals.var_tmp__blk1159_dn5, locals.var_tmp__blk1159_dn6, locals.var_tmp__blk1159_dn7, locals.var_tmp__blk1159_dn8, locals.var_tmp__blk1159_dn9, locals.var_tmp__blk1159_dn10, locals.var_tmp__blk1159_dn11, locals.var_tmp__blk1159_dn12,)
    }
};
        locals.var_tmp__blk1159 = assign30780_e24658;
        locals.var_tmp__blk1159_dn3 = assign30780_e24658_d_n3;
        locals.var_tmp__blk1159_dn4 = assign30780_e24658_d_n4;
        locals.var_tmp__blk1159_dn5 = assign30780_e24658_d_n5;
        locals.var_tmp__blk1159_dn6 = assign30780_e24658_d_n6;
        locals.var_tmp__blk1159_dn7 = assign30780_e24658_d_n7;
        locals.var_tmp__blk1159_dn8 = assign30780_e24658_d_n8;
        locals.var_tmp__blk1159_dn9 = assign30780_e24658_d_n9;
        locals.var_tmp__blk1159_dn10 = assign30780_e24658_d_n10;
        locals.var_tmp__blk1159_dn11 = assign30780_e24658_d_n11;
        locals.var_tmp__blk1159_dn12 = assign30780_e24658_d_n12;

        let assign30790_e24660: f64 = (-100.0);
        let assign30790_e24666: f64 = if ((assign30790_e24660 < locals.var_tmp__blk1159) && (locals.var_tmp__blk1159 < 100.0)) { 1.0 } else { 0.0 };
        locals.var_guard1724 = assign30790_e24666;

        let (assign30800_e24681, assign30800_e24681_d_n3, assign30800_e24681_d_n4, assign30800_e24681_d_n5, assign30800_e24681_d_n6, assign30800_e24681_d_n7, assign30800_e24681_d_n8, assign30800_e24681_d_n9, assign30800_e24681_d_n10, assign30800_e24681_d_n11, assign30800_e24681_d_n12,) = {
    if ((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1724 != 0.0)) {
        let assign30800_e24678: f64 = (locals.var_tmp__blk1159).exp();
        let assign30800_e24679: f64 = (locals.var_pparam_b4soildeb * assign30800_e24678);
        (assign30800_e24679, ((locals.var_pparam_b4soildeb_dn3 * assign30800_e24678) + (locals.var_pparam_b4soildeb * (assign30800_e24678 * locals.var_tmp__blk1159_dn3))), ((locals.var_pparam_b4soildeb_dn4 * assign30800_e24678) + (locals.var_pparam_b4soildeb * (assign30800_e24678 * locals.var_tmp__blk1159_dn4))), ((locals.var_pparam_b4soildeb_dn5 * assign30800_e24678) + (locals.var_pparam_b4soildeb * (assign30800_e24678 * locals.var_tmp__blk1159_dn5))), ((locals.var_pparam_b4soildeb_dn6 * assign30800_e24678) + (locals.var_pparam_b4soildeb * (assign30800_e24678 * locals.var_tmp__blk1159_dn6))), ((locals.var_pparam_b4soildeb_dn7 * assign30800_e24678) + (locals.var_pparam_b4soildeb * (assign30800_e24678 * locals.var_tmp__blk1159_dn7))), ((locals.var_pparam_b4soildeb_dn8 * assign30800_e24678) + (locals.var_pparam_b4soildeb * (assign30800_e24678 * locals.var_tmp__blk1159_dn8))), ((locals.var_pparam_b4soildeb_dn9 * assign30800_e24678) + (locals.var_pparam_b4soildeb * (assign30800_e24678 * locals.var_tmp__blk1159_dn9))), ((locals.var_pparam_b4soildeb_dn10 * assign30800_e24678) + (locals.var_pparam_b4soildeb * (assign30800_e24678 * locals.var_tmp__blk1159_dn10))), ((locals.var_pparam_b4soildeb_dn11 * assign30800_e24678) + (locals.var_pparam_b4soildeb * (assign30800_e24678 * locals.var_tmp__blk1159_dn11))), ((locals.var_pparam_b4soildeb_dn12 * assign30800_e24678) + (locals.var_pparam_b4soildeb * (assign30800_e24678 * locals.var_tmp__blk1159_dn12))),)
    } else {
        (locals.var_tcen__blk1299, locals.var_tcen__blk1299_dn3, locals.var_tcen__blk1299_dn4, locals.var_tcen__blk1299_dn5, locals.var_tcen__blk1299_dn6, locals.var_tcen__blk1299_dn7, locals.var_tcen__blk1299_dn8, locals.var_tcen__blk1299_dn9, locals.var_tcen__blk1299_dn10, locals.var_tcen__blk1299_dn11, locals.var_tcen__blk1299_dn12,)
    }
};
        locals.var_tcen__blk1299 = assign30800_e24681;
        locals.var_tcen__blk1299_dn3 = assign30800_e24681_d_n3;
        locals.var_tcen__blk1299_dn4 = assign30800_e24681_d_n4;
        locals.var_tcen__blk1299_dn5 = assign30800_e24681_d_n5;
        locals.var_tcen__blk1299_dn6 = assign30800_e24681_d_n6;
        locals.var_tcen__blk1299_dn7 = assign30800_e24681_d_n7;
        locals.var_tcen__blk1299_dn8 = assign30800_e24681_d_n8;
        locals.var_tcen__blk1299_dn9 = assign30800_e24681_d_n9;
        locals.var_tcen__blk1299_dn10 = assign30800_e24681_d_n10;
        locals.var_tcen__blk1299_dn11 = assign30800_e24681_d_n11;
        locals.var_tcen__blk1299_dn12 = assign30800_e24681_d_n12;

        let assign30810_e24684: f64 = (-100.0);
        let assign30810_e24685: f64 = if locals.var_tmp__blk1159 <= assign30810_e24684 { 1.0 } else { 0.0 };
        locals.var_guard1725 = assign30810_e24685;

        let (assign30820_e24702, assign30820_e24702_d_n3, assign30820_e24702_d_n4, assign30820_e24702_d_n5, assign30820_e24702_d_n6, assign30820_e24702_d_n7, assign30820_e24702_d_n8, assign30820_e24702_d_n9, assign30820_e24702_d_n10, assign30820_e24702_d_n11, assign30820_e24702_d_n12,) = {
    if (((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1724 == 0.0)) && (locals.var_guard1725 != 0.0)) {
        let assign30820_e24700: f64 = (locals.var_pparam_b4soildeb * 3.720075976e-44);
        (assign30820_e24700, (locals.var_pparam_b4soildeb_dn3 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn4 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn5 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn6 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn7 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn8 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn9 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn10 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn11 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn12 * 3.720075976e-44),)
    } else {
        (locals.var_tcen__blk1299, locals.var_tcen__blk1299_dn3, locals.var_tcen__blk1299_dn4, locals.var_tcen__blk1299_dn5, locals.var_tcen__blk1299_dn6, locals.var_tcen__blk1299_dn7, locals.var_tcen__blk1299_dn8, locals.var_tcen__blk1299_dn9, locals.var_tcen__blk1299_dn10, locals.var_tcen__blk1299_dn11, locals.var_tcen__blk1299_dn12,)
    }
};
        locals.var_tcen__blk1299 = assign30820_e24702;
        locals.var_tcen__blk1299_dn3 = assign30820_e24702_d_n3;
        locals.var_tcen__blk1299_dn4 = assign30820_e24702_d_n4;
        locals.var_tcen__blk1299_dn5 = assign30820_e24702_d_n5;
        locals.var_tcen__blk1299_dn6 = assign30820_e24702_d_n6;
        locals.var_tcen__blk1299_dn7 = assign30820_e24702_d_n7;
        locals.var_tcen__blk1299_dn8 = assign30820_e24702_d_n8;
        locals.var_tcen__blk1299_dn9 = assign30820_e24702_d_n9;
        locals.var_tcen__blk1299_dn10 = assign30820_e24702_d_n10;
        locals.var_tcen__blk1299_dn11 = assign30820_e24702_d_n11;
        locals.var_tcen__blk1299_dn12 = assign30820_e24702_d_n12;

        let (assign30830_e24720, assign30830_e24720_d_n3, assign30830_e24720_d_n4, assign30830_e24720_d_n5, assign30830_e24720_d_n6, assign30830_e24720_d_n7, assign30830_e24720_d_n8, assign30830_e24720_d_n9, assign30830_e24720_d_n10, assign30830_e24720_d_n11, assign30830_e24720_d_n12,) = {
    if (((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1724 == 0.0)) && (locals.var_guard1725 == 0.0)) {
        let assign30830_e24718: f64 = (locals.var_pparam_b4soildeb * 2.688117142e43);
        (assign30830_e24718, (locals.var_pparam_b4soildeb_dn3 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn4 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn5 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn6 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn7 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn8 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn9 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn10 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn11 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn12 * 2.688117142e43),)
    } else {
        (locals.var_tcen__blk1299, locals.var_tcen__blk1299_dn3, locals.var_tcen__blk1299_dn4, locals.var_tcen__blk1299_dn5, locals.var_tcen__blk1299_dn6, locals.var_tcen__blk1299_dn7, locals.var_tcen__blk1299_dn8, locals.var_tcen__blk1299_dn9, locals.var_tcen__blk1299_dn10, locals.var_tcen__blk1299_dn11, locals.var_tcen__blk1299_dn12,)
    }
};
        locals.var_tcen__blk1299 = assign30830_e24720;
        locals.var_tcen__blk1299_dn3 = assign30830_e24720_d_n3;
        locals.var_tcen__blk1299_dn4 = assign30830_e24720_d_n4;
        locals.var_tcen__blk1299_dn5 = assign30830_e24720_d_n5;
        locals.var_tcen__blk1299_dn6 = assign30830_e24720_d_n6;
        locals.var_tcen__blk1299_dn7 = assign30830_e24720_d_n7;
        locals.var_tcen__blk1299_dn8 = assign30830_e24720_d_n8;
        locals.var_tcen__blk1299_dn9 = assign30830_e24720_d_n9;
        locals.var_tcen__blk1299_dn10 = assign30830_e24720_d_n10;
        locals.var_tcen__blk1299_dn11 = assign30830_e24720_d_n11;
        locals.var_tcen__blk1299_dn12 = assign30830_e24720_d_n12;

        let (assign30840_e24732, assign30840_e24732_d_n3, assign30840_e24732_d_n4, assign30840_e24732_d_n5, assign30840_e24732_d_n6, assign30840_e24732_d_n7, assign30840_e24732_d_n8, assign30840_e24732_d_n9, assign30840_e24732_d_n10, assign30840_e24732_d_n11, assign30840_e24732_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) {
        let assign30840_e24730: f64 = (0.001 * locals.var_b4soitoxp);
        (assign30840_e24730, (0.001 * locals.var_b4soitoxp_dn3), (0.001 * locals.var_b4soitoxp_dn4), (0.001 * locals.var_b4soitoxp_dn5), (0.001 * locals.var_b4soitoxp_dn6), (0.001 * locals.var_b4soitoxp_dn7), (0.001 * locals.var_b4soitoxp_dn8), (0.001 * locals.var_b4soitoxp_dn9), (0.001 * locals.var_b4soitoxp_dn10), (0.001 * locals.var_b4soitoxp_dn11), (0.001 * locals.var_b4soitoxp_dn12),)
    } else {
        (locals.var_link, locals.var_link_dn3, locals.var_link_dn4, locals.var_link_dn5, locals.var_link_dn6, locals.var_link_dn7, locals.var_link_dn8, locals.var_link_dn9, locals.var_link_dn10, locals.var_link_dn11, locals.var_link_dn12,)
    }
};
        locals.var_link = assign30840_e24732;
        locals.var_link_dn3 = assign30840_e24732_d_n3;
        locals.var_link_dn4 = assign30840_e24732_d_n4;
        locals.var_link_dn5 = assign30840_e24732_d_n5;
        locals.var_link_dn6 = assign30840_e24732_d_n6;
        locals.var_link_dn7 = assign30840_e24732_d_n7;
        locals.var_link_dn8 = assign30840_e24732_d_n8;
        locals.var_link_dn9 = assign30840_e24732_d_n9;
        locals.var_link_dn10 = assign30840_e24732_d_n10;
        locals.var_link_dn11 = assign30840_e24732_d_n11;
        locals.var_link_dn12 = assign30840_e24732_d_n12;

        let (assign30850_e24746, assign30850_e24746_d_n3, assign30850_e24746_d_n4, assign30850_e24746_d_n5, assign30850_e24746_d_n6, assign30850_e24746_d_n7, assign30850_e24746_d_n8, assign30850_e24746_d_n9, assign30850_e24746_d_n10, assign30850_e24746_d_n11, assign30850_e24746_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) {
        let assign30850_e24742: f64 = (locals.var_pparam_b4soildeb - locals.var_tcen__blk1299);
        let assign30850_e24744: f64 = (assign30850_e24742 - locals.var_link);
        (assign30850_e24744, ((locals.var_pparam_b4soildeb_dn3 - locals.var_tcen__blk1299_dn3) - locals.var_link_dn3), ((locals.var_pparam_b4soildeb_dn4 - locals.var_tcen__blk1299_dn4) - locals.var_link_dn4), ((locals.var_pparam_b4soildeb_dn5 - locals.var_tcen__blk1299_dn5) - locals.var_link_dn5), ((locals.var_pparam_b4soildeb_dn6 - locals.var_tcen__blk1299_dn6) - locals.var_link_dn6), ((locals.var_pparam_b4soildeb_dn7 - locals.var_tcen__blk1299_dn7) - locals.var_link_dn7), ((locals.var_pparam_b4soildeb_dn8 - locals.var_tcen__blk1299_dn8) - locals.var_link_dn8), ((locals.var_pparam_b4soildeb_dn9 - locals.var_tcen__blk1299_dn9) - locals.var_link_dn9), ((locals.var_pparam_b4soildeb_dn10 - locals.var_tcen__blk1299_dn10) - locals.var_link_dn10), ((locals.var_pparam_b4soildeb_dn11 - locals.var_tcen__blk1299_dn11) - locals.var_link_dn11), ((locals.var_pparam_b4soildeb_dn12 - locals.var_tcen__blk1299_dn12) - locals.var_link_dn12),)
    } else {
        (locals.var_v3, locals.var_v3_dn3, locals.var_v3_dn4, locals.var_v3_dn5, locals.var_v3_dn6, locals.var_v3_dn7, locals.var_v3_dn8, locals.var_v3_dn9, locals.var_v3_dn10, locals.var_v3_dn11, locals.var_v3_dn12,)
    }
};
        locals.var_v3 = assign30850_e24746;
        locals.var_v3_dn3 = assign30850_e24746_d_n3;
        locals.var_v3_dn4 = assign30850_e24746_d_n4;
        locals.var_v3_dn5 = assign30850_e24746_d_n5;
        locals.var_v3_dn6 = assign30850_e24746_d_n6;
        locals.var_v3_dn7 = assign30850_e24746_d_n7;
        locals.var_v3_dn8 = assign30850_e24746_d_n8;
        locals.var_v3_dn9 = assign30850_e24746_d_n9;
        locals.var_v3_dn10 = assign30850_e24746_d_n10;
        locals.var_v3_dn11 = assign30850_e24746_d_n11;
        locals.var_v3_dn12 = assign30850_e24746_d_n12;

        let (assign30860_e24765, assign30860_e24765_d_n3, assign30860_e24765_d_n4, assign30860_e24765_d_n5, assign30860_e24765_d_n6, assign30860_e24765_d_n7, assign30860_e24765_d_n8, assign30860_e24765_d_n9, assign30860_e24765_d_n10, assign30860_e24765_d_n11, assign30860_e24765_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) {
        let assign30860_e24756: f64 = (locals.var_v3 * locals.var_v3);
        let assign30860_e24759: f64 = (4.0 * locals.var_link);
        let assign30860_e24761: f64 = (assign30860_e24759 * locals.var_pparam_b4soildeb);
        let assign30860_e24762: f64 = (assign30860_e24756 + assign30860_e24761);
        let assign30860_e24763: f64 = (assign30860_e24762).sqrt();
        (assign30860_e24763, ((((locals.var_v3_dn3 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn3)) + (((4.0 * locals.var_link_dn3) * locals.var_pparam_b4soildeb) + (assign30860_e24759 * locals.var_pparam_b4soildeb_dn3))) / (2.0 * assign30860_e24763)), ((((locals.var_v3_dn4 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn4)) + (((4.0 * locals.var_link_dn4) * locals.var_pparam_b4soildeb) + (assign30860_e24759 * locals.var_pparam_b4soildeb_dn4))) / (2.0 * assign30860_e24763)), ((((locals.var_v3_dn5 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn5)) + (((4.0 * locals.var_link_dn5) * locals.var_pparam_b4soildeb) + (assign30860_e24759 * locals.var_pparam_b4soildeb_dn5))) / (2.0 * assign30860_e24763)), ((((locals.var_v3_dn6 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn6)) + (((4.0 * locals.var_link_dn6) * locals.var_pparam_b4soildeb) + (assign30860_e24759 * locals.var_pparam_b4soildeb_dn6))) / (2.0 * assign30860_e24763)), ((((locals.var_v3_dn7 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn7)) + (((4.0 * locals.var_link_dn7) * locals.var_pparam_b4soildeb) + (assign30860_e24759 * locals.var_pparam_b4soildeb_dn7))) / (2.0 * assign30860_e24763)), ((((locals.var_v3_dn8 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn8)) + (((4.0 * locals.var_link_dn8) * locals.var_pparam_b4soildeb) + (assign30860_e24759 * locals.var_pparam_b4soildeb_dn8))) / (2.0 * assign30860_e24763)), ((((locals.var_v3_dn9 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn9)) + (((4.0 * locals.var_link_dn9) * locals.var_pparam_b4soildeb) + (assign30860_e24759 * locals.var_pparam_b4soildeb_dn9))) / (2.0 * assign30860_e24763)), ((((locals.var_v3_dn10 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn10)) + (((4.0 * locals.var_link_dn10) * locals.var_pparam_b4soildeb) + (assign30860_e24759 * locals.var_pparam_b4soildeb_dn10))) / (2.0 * assign30860_e24763)), ((((locals.var_v3_dn11 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn11)) + (((4.0 * locals.var_link_dn11) * locals.var_pparam_b4soildeb) + (assign30860_e24759 * locals.var_pparam_b4soildeb_dn11))) / (2.0 * assign30860_e24763)), ((((locals.var_v3_dn12 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn12)) + (((4.0 * locals.var_link_dn12) * locals.var_pparam_b4soildeb) + (assign30860_e24759 * locals.var_pparam_b4soildeb_dn12))) / (2.0 * assign30860_e24763)),)
    } else {
        (locals.var_v4, locals.var_v4_dn3, locals.var_v4_dn4, locals.var_v4_dn5, locals.var_v4_dn6, locals.var_v4_dn7, locals.var_v4_dn8, locals.var_v4_dn9, locals.var_v4_dn10, locals.var_v4_dn11, locals.var_v4_dn12,)
    }
};
        locals.var_v4 = assign30860_e24765;
        locals.var_v4_dn3 = assign30860_e24765_d_n3;
        locals.var_v4_dn4 = assign30860_e24765_d_n4;
        locals.var_v4_dn5 = assign30860_e24765_d_n5;
        locals.var_v4_dn6 = assign30860_e24765_d_n6;
        locals.var_v4_dn7 = assign30860_e24765_d_n7;
        locals.var_v4_dn8 = assign30860_e24765_d_n8;
        locals.var_v4_dn9 = assign30860_e24765_d_n9;
        locals.var_v4_dn10 = assign30860_e24765_d_n10;
        locals.var_v4_dn11 = assign30860_e24765_d_n11;
        locals.var_v4_dn12 = assign30860_e24765_d_n12;

        let (assign30870_e24781, assign30870_e24781_d_n3, assign30870_e24781_d_n4, assign30870_e24781_d_n5, assign30870_e24781_d_n6, assign30870_e24781_d_n7, assign30870_e24781_d_n8, assign30870_e24781_d_n9, assign30870_e24781_d_n10, assign30870_e24781_d_n11, assign30870_e24781_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) {
        let assign30870_e24777: f64 = (locals.var_v3 + locals.var_v4);
        let assign30870_e24778: f64 = (0.5 * assign30870_e24777);
        let assign30870_e24779: f64 = (locals.var_pparam_b4soildeb - assign30870_e24778);
        (assign30870_e24779, (locals.var_pparam_b4soildeb_dn3 - (0.5 * (locals.var_v3_dn3 + locals.var_v4_dn3))), (locals.var_pparam_b4soildeb_dn4 - (0.5 * (locals.var_v3_dn4 + locals.var_v4_dn4))), (locals.var_pparam_b4soildeb_dn5 - (0.5 * (locals.var_v3_dn5 + locals.var_v4_dn5))), (locals.var_pparam_b4soildeb_dn6 - (0.5 * (locals.var_v3_dn6 + locals.var_v4_dn6))), (locals.var_pparam_b4soildeb_dn7 - (0.5 * (locals.var_v3_dn7 + locals.var_v4_dn7))), (locals.var_pparam_b4soildeb_dn8 - (0.5 * (locals.var_v3_dn8 + locals.var_v4_dn8))), (locals.var_pparam_b4soildeb_dn9 - (0.5 * (locals.var_v3_dn9 + locals.var_v4_dn9))), (locals.var_pparam_b4soildeb_dn10 - (0.5 * (locals.var_v3_dn10 + locals.var_v4_dn10))), (locals.var_pparam_b4soildeb_dn11 - (0.5 * (locals.var_v3_dn11 + locals.var_v4_dn11))), (locals.var_pparam_b4soildeb_dn12 - (0.5 * (locals.var_v3_dn12 + locals.var_v4_dn12))),)
    } else {
        (locals.var_tcen__blk1299, locals.var_tcen__blk1299_dn3, locals.var_tcen__blk1299_dn4, locals.var_tcen__blk1299_dn5, locals.var_tcen__blk1299_dn6, locals.var_tcen__blk1299_dn7, locals.var_tcen__blk1299_dn8, locals.var_tcen__blk1299_dn9, locals.var_tcen__blk1299_dn10, locals.var_tcen__blk1299_dn11, locals.var_tcen__blk1299_dn12,)
    }
};
        locals.var_tcen__blk1299 = assign30870_e24781;
        locals.var_tcen__blk1299_dn3 = assign30870_e24781_d_n3;
        locals.var_tcen__blk1299_dn4 = assign30870_e24781_d_n4;
        locals.var_tcen__blk1299_dn5 = assign30870_e24781_d_n5;
        locals.var_tcen__blk1299_dn6 = assign30870_e24781_d_n6;
        locals.var_tcen__blk1299_dn7 = assign30870_e24781_d_n7;
        locals.var_tcen__blk1299_dn8 = assign30870_e24781_d_n8;
        locals.var_tcen__blk1299_dn9 = assign30870_e24781_d_n9;
        locals.var_tcen__blk1299_dn10 = assign30870_e24781_d_n10;
        locals.var_tcen__blk1299_dn11 = assign30870_e24781_d_n11;
        locals.var_tcen__blk1299_dn12 = assign30870_e24781_d_n12;

        let assign30880_e24784: f64 = if locals.var_tcen__blk1299 < 1e-15 { 1.0 } else { 0.0 };
        locals.var_guard1726 = assign30880_e24784;

        let (assign30890_e24796, assign30890_e24796_d_n3, assign30890_e24796_d_n4, assign30890_e24796_d_n5, assign30890_e24796_d_n6, assign30890_e24796_d_n7, assign30890_e24796_d_n8, assign30890_e24796_d_n9, assign30890_e24796_d_n10, assign30890_e24796_d_n11, assign30890_e24796_d_n12,) = {
    if ((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1726 != 0.0)) {
        (1e-15, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tcen__blk1299, locals.var_tcen__blk1299_dn3, locals.var_tcen__blk1299_dn4, locals.var_tcen__blk1299_dn5, locals.var_tcen__blk1299_dn6, locals.var_tcen__blk1299_dn7, locals.var_tcen__blk1299_dn8, locals.var_tcen__blk1299_dn9, locals.var_tcen__blk1299_dn10, locals.var_tcen__blk1299_dn11, locals.var_tcen__blk1299_dn12,)
    }
};
        locals.var_tcen__blk1299 = assign30890_e24796;
        locals.var_tcen__blk1299_dn3 = assign30890_e24796_d_n3;
        locals.var_tcen__blk1299_dn4 = assign30890_e24796_d_n4;
        locals.var_tcen__blk1299_dn5 = assign30890_e24796_d_n5;
        locals.var_tcen__blk1299_dn6 = assign30890_e24796_d_n6;
        locals.var_tcen__blk1299_dn7 = assign30890_e24796_d_n7;
        locals.var_tcen__blk1299_dn8 = assign30890_e24796_d_n8;
        locals.var_tcen__blk1299_dn9 = assign30890_e24796_d_n9;
        locals.var_tcen__blk1299_dn10 = assign30890_e24796_d_n10;
        locals.var_tcen__blk1299_dn11 = assign30890_e24796_d_n11;
        locals.var_tcen__blk1299_dn12 = assign30890_e24796_d_n12;

        let assign30900_e24799: f64 = if locals.var_b4soiagbcp2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1727 = assign30900_e24799;

        let (assign30910_e24817, assign30910_e24817_d_n3, assign30910_e24817_d_n4, assign30910_e24817_d_n5, assign30910_e24817_d_n6, assign30910_e24817_d_n7, assign30910_e24817_d_n8, assign30910_e24817_d_n9, assign30910_e24817_d_n10, assign30910_e24817_d_n11, assign30910_e24817_d_n12,) = {
    if ((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1727 != 0.0)) {
        let assign30910_e24811: f64 = (locals.var_vgs_eff2 - locals.var_vbseff);
        let assign30910_e24813: f64 = (assign30910_e24811 - locals.var_vfbzb2);
        let assign30910_e24815: f64 = (assign30910_e24813 / locals.var_tox);
        (assign30910_e24815, (((((-locals.var_vbseff_dn3) - locals.var_vfbzb2_dn3) * locals.var_tox) - (assign30910_e24813 * locals.var_tox_dn3)) / (locals.var_tox * locals.var_tox)), (((((-locals.var_vbseff_dn4) - locals.var_vfbzb2_dn4) * locals.var_tox) - (assign30910_e24813 * locals.var_tox_dn4)) / (locals.var_tox * locals.var_tox)), (((((-locals.var_vbseff_dn5) - locals.var_vfbzb2_dn5) * locals.var_tox) - (assign30910_e24813 * locals.var_tox_dn5)) / (locals.var_tox * locals.var_tox)), (((((-locals.var_vbseff_dn6) - locals.var_vfbzb2_dn6) * locals.var_tox) - (assign30910_e24813 * locals.var_tox_dn6)) / (locals.var_tox * locals.var_tox)), (((((locals.var_vgs_eff2_dn7 - locals.var_vbseff_dn7) - locals.var_vfbzb2_dn7) * locals.var_tox) - (assign30910_e24813 * locals.var_tox_dn7)) / (locals.var_tox * locals.var_tox)), (((((locals.var_vgs_eff2_dn8 - locals.var_vbseff_dn8) - locals.var_vfbzb2_dn8) * locals.var_tox) - (assign30910_e24813 * locals.var_tox_dn8)) / (locals.var_tox * locals.var_tox)), (((((locals.var_vgs_eff2_dn9 - locals.var_vbseff_dn9) - locals.var_vfbzb2_dn9) * locals.var_tox) - (assign30910_e24813 * locals.var_tox_dn9)) / (locals.var_tox * locals.var_tox)), (((((-locals.var_vbseff_dn10) - locals.var_vfbzb2_dn10) * locals.var_tox) - (assign30910_e24813 * locals.var_tox_dn10)) / (locals.var_tox * locals.var_tox)), (((((-locals.var_vbseff_dn11) - locals.var_vfbzb2_dn11) * locals.var_tox) - (assign30910_e24813 * locals.var_tox_dn11)) / (locals.var_tox * locals.var_tox)), (((((-locals.var_vbseff_dn12) - locals.var_vfbzb2_dn12) * locals.var_tox) - (assign30910_e24813 * locals.var_tox_dn12)) / (locals.var_tox * locals.var_tox)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign30910_e24817;
        locals.var_t0__blk1144_dn3 = assign30910_e24817_d_n3;
        locals.var_t0__blk1144_dn4 = assign30910_e24817_d_n4;
        locals.var_t0__blk1144_dn5 = assign30910_e24817_d_n5;
        locals.var_t0__blk1144_dn6 = assign30910_e24817_d_n6;
        locals.var_t0__blk1144_dn7 = assign30910_e24817_d_n7;
        locals.var_t0__blk1144_dn8 = assign30910_e24817_d_n8;
        locals.var_t0__blk1144_dn9 = assign30910_e24817_d_n9;
        locals.var_t0__blk1144_dn10 = assign30910_e24817_d_n10;
        locals.var_t0__blk1144_dn11 = assign30910_e24817_d_n11;
        locals.var_t0__blk1144_dn12 = assign30910_e24817_d_n12;

        let (assign30920_e24831, assign30920_e24831_d_n3, assign30920_e24831_d_n4, assign30920_e24831_d_n5, assign30920_e24831_d_n6, assign30920_e24831_d_n7, assign30920_e24831_d_n8, assign30920_e24831_d_n9, assign30920_e24831_d_n10, assign30920_e24831_d_n11, assign30920_e24831_d_n12,) = {
    if ((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1727 != 0.0)) {
        let assign30920_e24829: f64 = (locals.var_t0__blk1144 * locals.var_pparam_b4soiacde);
        (assign30920_e24829, ((locals.var_t0__blk1144_dn3 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk1144 * locals.var_pparam_b4soiacde_dn3)), ((locals.var_t0__blk1144_dn4 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk1144 * locals.var_pparam_b4soiacde_dn4)), ((locals.var_t0__blk1144_dn5 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk1144 * locals.var_pparam_b4soiacde_dn5)), ((locals.var_t0__blk1144_dn6 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk1144 * locals.var_pparam_b4soiacde_dn6)), ((locals.var_t0__blk1144_dn7 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk1144 * locals.var_pparam_b4soiacde_dn7)), ((locals.var_t0__blk1144_dn8 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk1144 * locals.var_pparam_b4soiacde_dn8)), ((locals.var_t0__blk1144_dn9 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk1144 * locals.var_pparam_b4soiacde_dn9)), ((locals.var_t0__blk1144_dn10 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk1144 * locals.var_pparam_b4soiacde_dn10)), ((locals.var_t0__blk1144_dn11 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk1144 * locals.var_pparam_b4soiacde_dn11)), ((locals.var_t0__blk1144_dn12 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk1144 * locals.var_pparam_b4soiacde_dn12)),)
    } else {
        (locals.var_tmp__blk1159, locals.var_tmp__blk1159_dn3, locals.var_tmp__blk1159_dn4, locals.var_tmp__blk1159_dn5, locals.var_tmp__blk1159_dn6, locals.var_tmp__blk1159_dn7, locals.var_tmp__blk1159_dn8, locals.var_tmp__blk1159_dn9, locals.var_tmp__blk1159_dn10, locals.var_tmp__blk1159_dn11, locals.var_tmp__blk1159_dn12,)
    }
};
        locals.var_tmp__blk1159 = assign30920_e24831;
        locals.var_tmp__blk1159_dn3 = assign30920_e24831_d_n3;
        locals.var_tmp__blk1159_dn4 = assign30920_e24831_d_n4;
        locals.var_tmp__blk1159_dn5 = assign30920_e24831_d_n5;
        locals.var_tmp__blk1159_dn6 = assign30920_e24831_d_n6;
        locals.var_tmp__blk1159_dn7 = assign30920_e24831_d_n7;
        locals.var_tmp__blk1159_dn8 = assign30920_e24831_d_n8;
        locals.var_tmp__blk1159_dn9 = assign30920_e24831_d_n9;
        locals.var_tmp__blk1159_dn10 = assign30920_e24831_d_n10;
        locals.var_tmp__blk1159_dn11 = assign30920_e24831_d_n11;
        locals.var_tmp__blk1159_dn12 = assign30920_e24831_d_n12;

        let assign30930_e24833: f64 = (-100.0);
        let assign30930_e24839: f64 = if ((assign30930_e24833 < locals.var_tmp__blk1159) && (locals.var_tmp__blk1159 < 100.0)) { 1.0 } else { 0.0 };
        locals.var_guard1728 = assign30930_e24839;

    }

    pub(super) fn stamp_transient_block_84(
        locals: &mut StampLocals,
    ) {
        let (assign30940_e24856, assign30940_e24856_d_n3, assign30940_e24856_d_n4, assign30940_e24856_d_n5, assign30940_e24856_d_n6, assign30940_e24856_d_n7, assign30940_e24856_d_n8, assign30940_e24856_d_n9, assign30940_e24856_d_n10, assign30940_e24856_d_n11, assign30940_e24856_d_n12,) = {
    if (((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1727 != 0.0)) && (locals.var_guard1728 != 0.0)) {
        let assign30940_e24853: f64 = (locals.var_tmp__blk1159).exp();
        let assign30940_e24854: f64 = (locals.var_pparam_b4soildeb * assign30940_e24853);
        (assign30940_e24854, ((locals.var_pparam_b4soildeb_dn3 * assign30940_e24853) + (locals.var_pparam_b4soildeb * (assign30940_e24853 * locals.var_tmp__blk1159_dn3))), ((locals.var_pparam_b4soildeb_dn4 * assign30940_e24853) + (locals.var_pparam_b4soildeb * (assign30940_e24853 * locals.var_tmp__blk1159_dn4))), ((locals.var_pparam_b4soildeb_dn5 * assign30940_e24853) + (locals.var_pparam_b4soildeb * (assign30940_e24853 * locals.var_tmp__blk1159_dn5))), ((locals.var_pparam_b4soildeb_dn6 * assign30940_e24853) + (locals.var_pparam_b4soildeb * (assign30940_e24853 * locals.var_tmp__blk1159_dn6))), ((locals.var_pparam_b4soildeb_dn7 * assign30940_e24853) + (locals.var_pparam_b4soildeb * (assign30940_e24853 * locals.var_tmp__blk1159_dn7))), ((locals.var_pparam_b4soildeb_dn8 * assign30940_e24853) + (locals.var_pparam_b4soildeb * (assign30940_e24853 * locals.var_tmp__blk1159_dn8))), ((locals.var_pparam_b4soildeb_dn9 * assign30940_e24853) + (locals.var_pparam_b4soildeb * (assign30940_e24853 * locals.var_tmp__blk1159_dn9))), ((locals.var_pparam_b4soildeb_dn10 * assign30940_e24853) + (locals.var_pparam_b4soildeb * (assign30940_e24853 * locals.var_tmp__blk1159_dn10))), ((locals.var_pparam_b4soildeb_dn11 * assign30940_e24853) + (locals.var_pparam_b4soildeb * (assign30940_e24853 * locals.var_tmp__blk1159_dn11))), ((locals.var_pparam_b4soildeb_dn12 * assign30940_e24853) + (locals.var_pparam_b4soildeb * (assign30940_e24853 * locals.var_tmp__blk1159_dn12))),)
    } else {
        (locals.var_tcen2, locals.var_tcen2_dn3, locals.var_tcen2_dn4, locals.var_tcen2_dn5, locals.var_tcen2_dn6, locals.var_tcen2_dn7, locals.var_tcen2_dn8, locals.var_tcen2_dn9, locals.var_tcen2_dn10, locals.var_tcen2_dn11, locals.var_tcen2_dn12,)
    }
};
        locals.var_tcen2 = assign30940_e24856;
        locals.var_tcen2_dn3 = assign30940_e24856_d_n3;
        locals.var_tcen2_dn4 = assign30940_e24856_d_n4;
        locals.var_tcen2_dn5 = assign30940_e24856_d_n5;
        locals.var_tcen2_dn6 = assign30940_e24856_d_n6;
        locals.var_tcen2_dn7 = assign30940_e24856_d_n7;
        locals.var_tcen2_dn8 = assign30940_e24856_d_n8;
        locals.var_tcen2_dn9 = assign30940_e24856_d_n9;
        locals.var_tcen2_dn10 = assign30940_e24856_d_n10;
        locals.var_tcen2_dn11 = assign30940_e24856_d_n11;
        locals.var_tcen2_dn12 = assign30940_e24856_d_n12;

        let assign30950_e24859: f64 = (-100.0);
        let assign30950_e24860: f64 = if locals.var_tmp__blk1159 <= assign30950_e24859 { 1.0 } else { 0.0 };
        locals.var_guard1729 = assign30950_e24860;

        let (assign30960_e24879, assign30960_e24879_d_n3, assign30960_e24879_d_n4, assign30960_e24879_d_n5, assign30960_e24879_d_n6, assign30960_e24879_d_n7, assign30960_e24879_d_n8, assign30960_e24879_d_n9, assign30960_e24879_d_n10, assign30960_e24879_d_n11, assign30960_e24879_d_n12,) = {
    if ((((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1727 != 0.0)) && (locals.var_guard1728 == 0.0)) && (locals.var_guard1729 != 0.0)) {
        let assign30960_e24877: f64 = (locals.var_pparam_b4soildeb * 3.720075976e-44);
        (assign30960_e24877, (locals.var_pparam_b4soildeb_dn3 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn4 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn5 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn6 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn7 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn8 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn9 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn10 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn11 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn12 * 3.720075976e-44),)
    } else {
        (locals.var_tcen2, locals.var_tcen2_dn3, locals.var_tcen2_dn4, locals.var_tcen2_dn5, locals.var_tcen2_dn6, locals.var_tcen2_dn7, locals.var_tcen2_dn8, locals.var_tcen2_dn9, locals.var_tcen2_dn10, locals.var_tcen2_dn11, locals.var_tcen2_dn12,)
    }
};
        locals.var_tcen2 = assign30960_e24879;
        locals.var_tcen2_dn3 = assign30960_e24879_d_n3;
        locals.var_tcen2_dn4 = assign30960_e24879_d_n4;
        locals.var_tcen2_dn5 = assign30960_e24879_d_n5;
        locals.var_tcen2_dn6 = assign30960_e24879_d_n6;
        locals.var_tcen2_dn7 = assign30960_e24879_d_n7;
        locals.var_tcen2_dn8 = assign30960_e24879_d_n8;
        locals.var_tcen2_dn9 = assign30960_e24879_d_n9;
        locals.var_tcen2_dn10 = assign30960_e24879_d_n10;
        locals.var_tcen2_dn11 = assign30960_e24879_d_n11;
        locals.var_tcen2_dn12 = assign30960_e24879_d_n12;

        let (assign30970_e24899, assign30970_e24899_d_n3, assign30970_e24899_d_n4, assign30970_e24899_d_n5, assign30970_e24899_d_n6, assign30970_e24899_d_n7, assign30970_e24899_d_n8, assign30970_e24899_d_n9, assign30970_e24899_d_n10, assign30970_e24899_d_n11, assign30970_e24899_d_n12,) = {
    if ((((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1727 != 0.0)) && (locals.var_guard1728 == 0.0)) && (locals.var_guard1729 == 0.0)) {
        let assign30970_e24897: f64 = (locals.var_pparam_b4soildeb * 2.688117142e43);
        (assign30970_e24897, (locals.var_pparam_b4soildeb_dn3 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn4 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn5 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn6 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn7 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn8 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn9 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn10 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn11 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn12 * 2.688117142e43),)
    } else {
        (locals.var_tcen2, locals.var_tcen2_dn3, locals.var_tcen2_dn4, locals.var_tcen2_dn5, locals.var_tcen2_dn6, locals.var_tcen2_dn7, locals.var_tcen2_dn8, locals.var_tcen2_dn9, locals.var_tcen2_dn10, locals.var_tcen2_dn11, locals.var_tcen2_dn12,)
    }
};
        locals.var_tcen2 = assign30970_e24899;
        locals.var_tcen2_dn3 = assign30970_e24899_d_n3;
        locals.var_tcen2_dn4 = assign30970_e24899_d_n4;
        locals.var_tcen2_dn5 = assign30970_e24899_d_n5;
        locals.var_tcen2_dn6 = assign30970_e24899_d_n6;
        locals.var_tcen2_dn7 = assign30970_e24899_d_n7;
        locals.var_tcen2_dn8 = assign30970_e24899_d_n8;
        locals.var_tcen2_dn9 = assign30970_e24899_d_n9;
        locals.var_tcen2_dn10 = assign30970_e24899_d_n10;
        locals.var_tcen2_dn11 = assign30970_e24899_d_n11;
        locals.var_tcen2_dn12 = assign30970_e24899_d_n12;

        let (assign30980_e24915, assign30980_e24915_d_n3, assign30980_e24915_d_n4, assign30980_e24915_d_n5, assign30980_e24915_d_n6, assign30980_e24915_d_n7, assign30980_e24915_d_n8, assign30980_e24915_d_n9, assign30980_e24915_d_n10, assign30980_e24915_d_n11, assign30980_e24915_d_n12,) = {
    if ((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1727 != 0.0)) {
        let assign30980_e24911: f64 = (locals.var_pparam_b4soildeb - locals.var_tcen2);
        let assign30980_e24913: f64 = (assign30980_e24911 - locals.var_link);
        (assign30980_e24913, ((locals.var_pparam_b4soildeb_dn3 - locals.var_tcen2_dn3) - locals.var_link_dn3), ((locals.var_pparam_b4soildeb_dn4 - locals.var_tcen2_dn4) - locals.var_link_dn4), ((locals.var_pparam_b4soildeb_dn5 - locals.var_tcen2_dn5) - locals.var_link_dn5), ((locals.var_pparam_b4soildeb_dn6 - locals.var_tcen2_dn6) - locals.var_link_dn6), ((locals.var_pparam_b4soildeb_dn7 - locals.var_tcen2_dn7) - locals.var_link_dn7), ((locals.var_pparam_b4soildeb_dn8 - locals.var_tcen2_dn8) - locals.var_link_dn8), ((locals.var_pparam_b4soildeb_dn9 - locals.var_tcen2_dn9) - locals.var_link_dn9), ((locals.var_pparam_b4soildeb_dn10 - locals.var_tcen2_dn10) - locals.var_link_dn10), ((locals.var_pparam_b4soildeb_dn11 - locals.var_tcen2_dn11) - locals.var_link_dn11), ((locals.var_pparam_b4soildeb_dn12 - locals.var_tcen2_dn12) - locals.var_link_dn12),)
    } else {
        (locals.var_v3, locals.var_v3_dn3, locals.var_v3_dn4, locals.var_v3_dn5, locals.var_v3_dn6, locals.var_v3_dn7, locals.var_v3_dn8, locals.var_v3_dn9, locals.var_v3_dn10, locals.var_v3_dn11, locals.var_v3_dn12,)
    }
};
        locals.var_v3 = assign30980_e24915;
        locals.var_v3_dn3 = assign30980_e24915_d_n3;
        locals.var_v3_dn4 = assign30980_e24915_d_n4;
        locals.var_v3_dn5 = assign30980_e24915_d_n5;
        locals.var_v3_dn6 = assign30980_e24915_d_n6;
        locals.var_v3_dn7 = assign30980_e24915_d_n7;
        locals.var_v3_dn8 = assign30980_e24915_d_n8;
        locals.var_v3_dn9 = assign30980_e24915_d_n9;
        locals.var_v3_dn10 = assign30980_e24915_d_n10;
        locals.var_v3_dn11 = assign30980_e24915_d_n11;
        locals.var_v3_dn12 = assign30980_e24915_d_n12;

        let (assign30990_e24936, assign30990_e24936_d_n3, assign30990_e24936_d_n4, assign30990_e24936_d_n5, assign30990_e24936_d_n6, assign30990_e24936_d_n7, assign30990_e24936_d_n8, assign30990_e24936_d_n9, assign30990_e24936_d_n10, assign30990_e24936_d_n11, assign30990_e24936_d_n12,) = {
    if ((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1727 != 0.0)) {
        let assign30990_e24927: f64 = (locals.var_v3 * locals.var_v3);
        let assign30990_e24930: f64 = (4.0 * locals.var_link);
        let assign30990_e24932: f64 = (assign30990_e24930 * locals.var_pparam_b4soildeb);
        let assign30990_e24933: f64 = (assign30990_e24927 + assign30990_e24932);
        let assign30990_e24934: f64 = (assign30990_e24933).sqrt();
        (assign30990_e24934, ((((locals.var_v3_dn3 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn3)) + (((4.0 * locals.var_link_dn3) * locals.var_pparam_b4soildeb) + (assign30990_e24930 * locals.var_pparam_b4soildeb_dn3))) / (2.0 * assign30990_e24934)), ((((locals.var_v3_dn4 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn4)) + (((4.0 * locals.var_link_dn4) * locals.var_pparam_b4soildeb) + (assign30990_e24930 * locals.var_pparam_b4soildeb_dn4))) / (2.0 * assign30990_e24934)), ((((locals.var_v3_dn5 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn5)) + (((4.0 * locals.var_link_dn5) * locals.var_pparam_b4soildeb) + (assign30990_e24930 * locals.var_pparam_b4soildeb_dn5))) / (2.0 * assign30990_e24934)), ((((locals.var_v3_dn6 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn6)) + (((4.0 * locals.var_link_dn6) * locals.var_pparam_b4soildeb) + (assign30990_e24930 * locals.var_pparam_b4soildeb_dn6))) / (2.0 * assign30990_e24934)), ((((locals.var_v3_dn7 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn7)) + (((4.0 * locals.var_link_dn7) * locals.var_pparam_b4soildeb) + (assign30990_e24930 * locals.var_pparam_b4soildeb_dn7))) / (2.0 * assign30990_e24934)), ((((locals.var_v3_dn8 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn8)) + (((4.0 * locals.var_link_dn8) * locals.var_pparam_b4soildeb) + (assign30990_e24930 * locals.var_pparam_b4soildeb_dn8))) / (2.0 * assign30990_e24934)), ((((locals.var_v3_dn9 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn9)) + (((4.0 * locals.var_link_dn9) * locals.var_pparam_b4soildeb) + (assign30990_e24930 * locals.var_pparam_b4soildeb_dn9))) / (2.0 * assign30990_e24934)), ((((locals.var_v3_dn10 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn10)) + (((4.0 * locals.var_link_dn10) * locals.var_pparam_b4soildeb) + (assign30990_e24930 * locals.var_pparam_b4soildeb_dn10))) / (2.0 * assign30990_e24934)), ((((locals.var_v3_dn11 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn11)) + (((4.0 * locals.var_link_dn11) * locals.var_pparam_b4soildeb) + (assign30990_e24930 * locals.var_pparam_b4soildeb_dn11))) / (2.0 * assign30990_e24934)), ((((locals.var_v3_dn12 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn12)) + (((4.0 * locals.var_link_dn12) * locals.var_pparam_b4soildeb) + (assign30990_e24930 * locals.var_pparam_b4soildeb_dn12))) / (2.0 * assign30990_e24934)),)
    } else {
        (locals.var_v4, locals.var_v4_dn3, locals.var_v4_dn4, locals.var_v4_dn5, locals.var_v4_dn6, locals.var_v4_dn7, locals.var_v4_dn8, locals.var_v4_dn9, locals.var_v4_dn10, locals.var_v4_dn11, locals.var_v4_dn12,)
    }
};
        locals.var_v4 = assign30990_e24936;
        locals.var_v4_dn3 = assign30990_e24936_d_n3;
        locals.var_v4_dn4 = assign30990_e24936_d_n4;
        locals.var_v4_dn5 = assign30990_e24936_d_n5;
        locals.var_v4_dn6 = assign30990_e24936_d_n6;
        locals.var_v4_dn7 = assign30990_e24936_d_n7;
        locals.var_v4_dn8 = assign30990_e24936_d_n8;
        locals.var_v4_dn9 = assign30990_e24936_d_n9;
        locals.var_v4_dn10 = assign30990_e24936_d_n10;
        locals.var_v4_dn11 = assign30990_e24936_d_n11;
        locals.var_v4_dn12 = assign30990_e24936_d_n12;

        let (assign31000_e24954, assign31000_e24954_d_n3, assign31000_e24954_d_n4, assign31000_e24954_d_n5, assign31000_e24954_d_n6, assign31000_e24954_d_n7, assign31000_e24954_d_n8, assign31000_e24954_d_n9, assign31000_e24954_d_n10, assign31000_e24954_d_n11, assign31000_e24954_d_n12,) = {
    if ((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1727 != 0.0)) {
        let assign31000_e24950: f64 = (locals.var_v3 + locals.var_v4);
        let assign31000_e24951: f64 = (0.5 * assign31000_e24950);
        let assign31000_e24952: f64 = (locals.var_pparam_b4soildeb - assign31000_e24951);
        (assign31000_e24952, (locals.var_pparam_b4soildeb_dn3 - (0.5 * (locals.var_v3_dn3 + locals.var_v4_dn3))), (locals.var_pparam_b4soildeb_dn4 - (0.5 * (locals.var_v3_dn4 + locals.var_v4_dn4))), (locals.var_pparam_b4soildeb_dn5 - (0.5 * (locals.var_v3_dn5 + locals.var_v4_dn5))), (locals.var_pparam_b4soildeb_dn6 - (0.5 * (locals.var_v3_dn6 + locals.var_v4_dn6))), (locals.var_pparam_b4soildeb_dn7 - (0.5 * (locals.var_v3_dn7 + locals.var_v4_dn7))), (locals.var_pparam_b4soildeb_dn8 - (0.5 * (locals.var_v3_dn8 + locals.var_v4_dn8))), (locals.var_pparam_b4soildeb_dn9 - (0.5 * (locals.var_v3_dn9 + locals.var_v4_dn9))), (locals.var_pparam_b4soildeb_dn10 - (0.5 * (locals.var_v3_dn10 + locals.var_v4_dn10))), (locals.var_pparam_b4soildeb_dn11 - (0.5 * (locals.var_v3_dn11 + locals.var_v4_dn11))), (locals.var_pparam_b4soildeb_dn12 - (0.5 * (locals.var_v3_dn12 + locals.var_v4_dn12))),)
    } else {
        (locals.var_tcen2, locals.var_tcen2_dn3, locals.var_tcen2_dn4, locals.var_tcen2_dn5, locals.var_tcen2_dn6, locals.var_tcen2_dn7, locals.var_tcen2_dn8, locals.var_tcen2_dn9, locals.var_tcen2_dn10, locals.var_tcen2_dn11, locals.var_tcen2_dn12,)
    }
};
        locals.var_tcen2 = assign31000_e24954;
        locals.var_tcen2_dn3 = assign31000_e24954_d_n3;
        locals.var_tcen2_dn4 = assign31000_e24954_d_n4;
        locals.var_tcen2_dn5 = assign31000_e24954_d_n5;
        locals.var_tcen2_dn6 = assign31000_e24954_d_n6;
        locals.var_tcen2_dn7 = assign31000_e24954_d_n7;
        locals.var_tcen2_dn8 = assign31000_e24954_d_n8;
        locals.var_tcen2_dn9 = assign31000_e24954_d_n9;
        locals.var_tcen2_dn10 = assign31000_e24954_d_n10;
        locals.var_tcen2_dn11 = assign31000_e24954_d_n11;
        locals.var_tcen2_dn12 = assign31000_e24954_d_n12;

        let assign31010_e24957: f64 = if locals.var_tcen2 < 1e-15 { 1.0 } else { 0.0 };
        locals.var_guard1730 = assign31010_e24957;

        let (assign31020_e24971, assign31020_e24971_d_n3, assign31020_e24971_d_n4, assign31020_e24971_d_n5, assign31020_e24971_d_n6, assign31020_e24971_d_n7, assign31020_e24971_d_n8, assign31020_e24971_d_n9, assign31020_e24971_d_n10, assign31020_e24971_d_n11, assign31020_e24971_d_n12,) = {
    if (((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1727 != 0.0)) && (locals.var_guard1730 != 0.0)) {
        (1e-15, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tcen2, locals.var_tcen2_dn3, locals.var_tcen2_dn4, locals.var_tcen2_dn5, locals.var_tcen2_dn6, locals.var_tcen2_dn7, locals.var_tcen2_dn8, locals.var_tcen2_dn9, locals.var_tcen2_dn10, locals.var_tcen2_dn11, locals.var_tcen2_dn12,)
    }
};
        locals.var_tcen2 = assign31020_e24971;
        locals.var_tcen2_dn3 = assign31020_e24971_d_n3;
        locals.var_tcen2_dn4 = assign31020_e24971_d_n4;
        locals.var_tcen2_dn5 = assign31020_e24971_d_n5;
        locals.var_tcen2_dn6 = assign31020_e24971_d_n6;
        locals.var_tcen2_dn7 = assign31020_e24971_d_n7;
        locals.var_tcen2_dn8 = assign31020_e24971_d_n8;
        locals.var_tcen2_dn9 = assign31020_e24971_d_n9;
        locals.var_tcen2_dn10 = assign31020_e24971_d_n10;
        locals.var_tcen2_dn11 = assign31020_e24971_d_n11;
        locals.var_tcen2_dn12 = assign31020_e24971_d_n12;

        let (assign31030_e24983, assign31030_e24983_d_n3, assign31030_e24983_d_n4, assign31030_e24983_d_n5, assign31030_e24983_d_n6, assign31030_e24983_d_n7, assign31030_e24983_d_n8, assign31030_e24983_d_n9, assign31030_e24983_d_n10, assign31030_e24983_d_n11, assign31030_e24983_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) {
        let assign31030_e24981: f64 = (locals.var_epssub / locals.var_tcen__blk1299);
        (assign31030_e24981, (-((locals.var_epssub * locals.var_tcen__blk1299_dn3) / (locals.var_tcen__blk1299 * locals.var_tcen__blk1299))), (-((locals.var_epssub * locals.var_tcen__blk1299_dn4) / (locals.var_tcen__blk1299 * locals.var_tcen__blk1299))), (-((locals.var_epssub * locals.var_tcen__blk1299_dn5) / (locals.var_tcen__blk1299 * locals.var_tcen__blk1299))), (-((locals.var_epssub * locals.var_tcen__blk1299_dn6) / (locals.var_tcen__blk1299 * locals.var_tcen__blk1299))), (-((locals.var_epssub * locals.var_tcen__blk1299_dn7) / (locals.var_tcen__blk1299 * locals.var_tcen__blk1299))), (-((locals.var_epssub * locals.var_tcen__blk1299_dn8) / (locals.var_tcen__blk1299 * locals.var_tcen__blk1299))), (-((locals.var_epssub * locals.var_tcen__blk1299_dn9) / (locals.var_tcen__blk1299 * locals.var_tcen__blk1299))), (-((locals.var_epssub * locals.var_tcen__blk1299_dn10) / (locals.var_tcen__blk1299 * locals.var_tcen__blk1299))), (-((locals.var_epssub * locals.var_tcen__blk1299_dn11) / (locals.var_tcen__blk1299 * locals.var_tcen__blk1299))), (-((locals.var_epssub * locals.var_tcen__blk1299_dn12) / (locals.var_tcen__blk1299 * locals.var_tcen__blk1299))),)
    } else {
        (locals.var_ccen, locals.var_ccen_dn3, locals.var_ccen_dn4, locals.var_ccen_dn5, locals.var_ccen_dn6, locals.var_ccen_dn7, locals.var_ccen_dn8, locals.var_ccen_dn9, locals.var_ccen_dn10, locals.var_ccen_dn11, locals.var_ccen_dn12,)
    }
};
        locals.var_ccen = assign31030_e24983;
        locals.var_ccen_dn3 = assign31030_e24983_d_n3;
        locals.var_ccen_dn4 = assign31030_e24983_d_n4;
        locals.var_ccen_dn5 = assign31030_e24983_d_n5;
        locals.var_ccen_dn6 = assign31030_e24983_d_n6;
        locals.var_ccen_dn7 = assign31030_e24983_d_n7;
        locals.var_ccen_dn8 = assign31030_e24983_d_n8;
        locals.var_ccen_dn9 = assign31030_e24983_d_n9;
        locals.var_ccen_dn10 = assign31030_e24983_d_n10;
        locals.var_ccen_dn11 = assign31030_e24983_d_n11;
        locals.var_ccen_dn12 = assign31030_e24983_d_n12;

        let (assign31040_e24997, assign31040_e24997_d_n3, assign31040_e24997_d_n4, assign31040_e24997_d_n5, assign31040_e24997_d_n6, assign31040_e24997_d_n7, assign31040_e24997_d_n8, assign31040_e24997_d_n9, assign31040_e24997_d_n10, assign31040_e24997_d_n11, assign31040_e24997_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) {
        let assign31040_e24994: f64 = (locals.var_cox + locals.var_ccen);
        let assign31040_e24995: f64 = (locals.var_cox / assign31040_e24994);
        (assign31040_e24995, (((locals.var_cox_dn3 * assign31040_e24994) - (locals.var_cox * (locals.var_cox_dn3 + locals.var_ccen_dn3))) / (assign31040_e24994 * assign31040_e24994)), (((locals.var_cox_dn4 * assign31040_e24994) - (locals.var_cox * (locals.var_cox_dn4 + locals.var_ccen_dn4))) / (assign31040_e24994 * assign31040_e24994)), (((locals.var_cox_dn5 * assign31040_e24994) - (locals.var_cox * (locals.var_cox_dn5 + locals.var_ccen_dn5))) / (assign31040_e24994 * assign31040_e24994)), (((locals.var_cox_dn6 * assign31040_e24994) - (locals.var_cox * (locals.var_cox_dn6 + locals.var_ccen_dn6))) / (assign31040_e24994 * assign31040_e24994)), (((locals.var_cox_dn7 * assign31040_e24994) - (locals.var_cox * (locals.var_cox_dn7 + locals.var_ccen_dn7))) / (assign31040_e24994 * assign31040_e24994)), (((locals.var_cox_dn8 * assign31040_e24994) - (locals.var_cox * (locals.var_cox_dn8 + locals.var_ccen_dn8))) / (assign31040_e24994 * assign31040_e24994)), (((locals.var_cox_dn9 * assign31040_e24994) - (locals.var_cox * (locals.var_cox_dn9 + locals.var_ccen_dn9))) / (assign31040_e24994 * assign31040_e24994)), (((locals.var_cox_dn10 * assign31040_e24994) - (locals.var_cox * (locals.var_cox_dn10 + locals.var_ccen_dn10))) / (assign31040_e24994 * assign31040_e24994)), (((locals.var_cox_dn11 * assign31040_e24994) - (locals.var_cox * (locals.var_cox_dn11 + locals.var_ccen_dn11))) / (assign31040_e24994 * assign31040_e24994)), (((locals.var_cox_dn12 * assign31040_e24994) - (locals.var_cox * (locals.var_cox_dn12 + locals.var_ccen_dn12))) / (assign31040_e24994 * assign31040_e24994)),)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign31040_e24997;
        locals.var_t2__blk1146_dn3 = assign31040_e24997_d_n3;
        locals.var_t2__blk1146_dn4 = assign31040_e24997_d_n4;
        locals.var_t2__blk1146_dn5 = assign31040_e24997_d_n5;
        locals.var_t2__blk1146_dn6 = assign31040_e24997_d_n6;
        locals.var_t2__blk1146_dn7 = assign31040_e24997_d_n7;
        locals.var_t2__blk1146_dn8 = assign31040_e24997_d_n8;
        locals.var_t2__blk1146_dn9 = assign31040_e24997_d_n9;
        locals.var_t2__blk1146_dn10 = assign31040_e24997_d_n10;
        locals.var_t2__blk1146_dn11 = assign31040_e24997_d_n11;
        locals.var_t2__blk1146_dn12 = assign31040_e24997_d_n12;

        let (assign31050_e25009, assign31050_e25009_d_n3, assign31050_e25009_d_n4, assign31050_e25009_d_n5, assign31050_e25009_d_n6, assign31050_e25009_d_n7, assign31050_e25009_d_n8, assign31050_e25009_d_n9, assign31050_e25009_d_n10, assign31050_e25009_d_n11, assign31050_e25009_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) {
        let assign31050_e25007: f64 = (locals.var_t2__blk1146 * locals.var_ccen);
        (assign31050_e25007, ((locals.var_t2__blk1146_dn3 * locals.var_ccen) + (locals.var_t2__blk1146 * locals.var_ccen_dn3)), ((locals.var_t2__blk1146_dn4 * locals.var_ccen) + (locals.var_t2__blk1146 * locals.var_ccen_dn4)), ((locals.var_t2__blk1146_dn5 * locals.var_ccen) + (locals.var_t2__blk1146 * locals.var_ccen_dn5)), ((locals.var_t2__blk1146_dn6 * locals.var_ccen) + (locals.var_t2__blk1146 * locals.var_ccen_dn6)), ((locals.var_t2__blk1146_dn7 * locals.var_ccen) + (locals.var_t2__blk1146 * locals.var_ccen_dn7)), ((locals.var_t2__blk1146_dn8 * locals.var_ccen) + (locals.var_t2__blk1146 * locals.var_ccen_dn8)), ((locals.var_t2__blk1146_dn9 * locals.var_ccen) + (locals.var_t2__blk1146 * locals.var_ccen_dn9)), ((locals.var_t2__blk1146_dn10 * locals.var_ccen) + (locals.var_t2__blk1146 * locals.var_ccen_dn10)), ((locals.var_t2__blk1146_dn11 * locals.var_ccen) + (locals.var_t2__blk1146 * locals.var_ccen_dn11)), ((locals.var_t2__blk1146_dn12 * locals.var_ccen) + (locals.var_t2__blk1146 * locals.var_ccen_dn12)),)
    } else {
        (locals.var_coxeff, locals.var_coxeff_dn3, locals.var_coxeff_dn4, locals.var_coxeff_dn5, locals.var_coxeff_dn6, locals.var_coxeff_dn7, locals.var_coxeff_dn8, locals.var_coxeff_dn9, locals.var_coxeff_dn10, locals.var_coxeff_dn11, locals.var_coxeff_dn12,)
    }
};
        locals.var_coxeff = assign31050_e25009;
        locals.var_coxeff_dn3 = assign31050_e25009_d_n3;
        locals.var_coxeff_dn4 = assign31050_e25009_d_n4;
        locals.var_coxeff_dn5 = assign31050_e25009_d_n5;
        locals.var_coxeff_dn6 = assign31050_e25009_d_n6;
        locals.var_coxeff_dn7 = assign31050_e25009_d_n7;
        locals.var_coxeff_dn8 = assign31050_e25009_d_n8;
        locals.var_coxeff_dn9 = assign31050_e25009_d_n9;
        locals.var_coxeff_dn10 = assign31050_e25009_d_n10;
        locals.var_coxeff_dn11 = assign31050_e25009_d_n11;
        locals.var_coxeff_dn12 = assign31050_e25009_d_n12;

        let assign31060_e25020: f64 = if (((locals.var_b4soisoimod != 2.0) && (locals.var_b4soibodymod != 0.0)) && (locals.var_b4soiagbcp2 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1731 = assign31060_e25020;

        let (assign31070_e25034, assign31070_e25034_d_n3, assign31070_e25034_d_n4, assign31070_e25034_d_n5, assign31070_e25034_d_n6, assign31070_e25034_d_n7, assign31070_e25034_d_n8, assign31070_e25034_d_n9, assign31070_e25034_d_n10, assign31070_e25034_d_n11, assign31070_e25034_d_n12,) = {
    if ((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1731 != 0.0)) {
        let assign31070_e25032: f64 = (locals.var_epssub / locals.var_tcen2);
        (assign31070_e25032, (-((locals.var_epssub * locals.var_tcen2_dn3) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn4) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn5) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn6) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn7) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn8) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn9) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn10) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn11) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn12) / (locals.var_tcen2 * locals.var_tcen2))),)
    } else {
        (locals.var_ccen2, locals.var_ccen2_dn3, locals.var_ccen2_dn4, locals.var_ccen2_dn5, locals.var_ccen2_dn6, locals.var_ccen2_dn7, locals.var_ccen2_dn8, locals.var_ccen2_dn9, locals.var_ccen2_dn10, locals.var_ccen2_dn11, locals.var_ccen2_dn12,)
    }
};
        locals.var_ccen2 = assign31070_e25034;
        locals.var_ccen2_dn3 = assign31070_e25034_d_n3;
        locals.var_ccen2_dn4 = assign31070_e25034_d_n4;
        locals.var_ccen2_dn5 = assign31070_e25034_d_n5;
        locals.var_ccen2_dn6 = assign31070_e25034_d_n6;
        locals.var_ccen2_dn7 = assign31070_e25034_d_n7;
        locals.var_ccen2_dn8 = assign31070_e25034_d_n8;
        locals.var_ccen2_dn9 = assign31070_e25034_d_n9;
        locals.var_ccen2_dn10 = assign31070_e25034_d_n10;
        locals.var_ccen2_dn11 = assign31070_e25034_d_n11;
        locals.var_ccen2_dn12 = assign31070_e25034_d_n12;

        let (assign31080_e25050, assign31080_e25050_d_n3, assign31080_e25050_d_n4, assign31080_e25050_d_n5, assign31080_e25050_d_n6, assign31080_e25050_d_n7, assign31080_e25050_d_n8, assign31080_e25050_d_n9, assign31080_e25050_d_n10, assign31080_e25050_d_n11, assign31080_e25050_d_n12,) = {
    if ((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1731 != 0.0)) {
        let assign31080_e25047: f64 = (locals.var_cox + locals.var_ccen2);
        let assign31080_e25048: f64 = (locals.var_cox / assign31080_e25047);
        (assign31080_e25048, (((locals.var_cox_dn3 * assign31080_e25047) - (locals.var_cox * (locals.var_cox_dn3 + locals.var_ccen2_dn3))) / (assign31080_e25047 * assign31080_e25047)), (((locals.var_cox_dn4 * assign31080_e25047) - (locals.var_cox * (locals.var_cox_dn4 + locals.var_ccen2_dn4))) / (assign31080_e25047 * assign31080_e25047)), (((locals.var_cox_dn5 * assign31080_e25047) - (locals.var_cox * (locals.var_cox_dn5 + locals.var_ccen2_dn5))) / (assign31080_e25047 * assign31080_e25047)), (((locals.var_cox_dn6 * assign31080_e25047) - (locals.var_cox * (locals.var_cox_dn6 + locals.var_ccen2_dn6))) / (assign31080_e25047 * assign31080_e25047)), (((locals.var_cox_dn7 * assign31080_e25047) - (locals.var_cox * (locals.var_cox_dn7 + locals.var_ccen2_dn7))) / (assign31080_e25047 * assign31080_e25047)), (((locals.var_cox_dn8 * assign31080_e25047) - (locals.var_cox * (locals.var_cox_dn8 + locals.var_ccen2_dn8))) / (assign31080_e25047 * assign31080_e25047)), (((locals.var_cox_dn9 * assign31080_e25047) - (locals.var_cox * (locals.var_cox_dn9 + locals.var_ccen2_dn9))) / (assign31080_e25047 * assign31080_e25047)), (((locals.var_cox_dn10 * assign31080_e25047) - (locals.var_cox * (locals.var_cox_dn10 + locals.var_ccen2_dn10))) / (assign31080_e25047 * assign31080_e25047)), (((locals.var_cox_dn11 * assign31080_e25047) - (locals.var_cox * (locals.var_cox_dn11 + locals.var_ccen2_dn11))) / (assign31080_e25047 * assign31080_e25047)), (((locals.var_cox_dn12 * assign31080_e25047) - (locals.var_cox * (locals.var_cox_dn12 + locals.var_ccen2_dn12))) / (assign31080_e25047 * assign31080_e25047)),)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign31080_e25050;
        locals.var_t2__blk1146_dn3 = assign31080_e25050_d_n3;
        locals.var_t2__blk1146_dn4 = assign31080_e25050_d_n4;
        locals.var_t2__blk1146_dn5 = assign31080_e25050_d_n5;
        locals.var_t2__blk1146_dn6 = assign31080_e25050_d_n6;
        locals.var_t2__blk1146_dn7 = assign31080_e25050_d_n7;
        locals.var_t2__blk1146_dn8 = assign31080_e25050_d_n8;
        locals.var_t2__blk1146_dn9 = assign31080_e25050_d_n9;
        locals.var_t2__blk1146_dn10 = assign31080_e25050_d_n10;
        locals.var_t2__blk1146_dn11 = assign31080_e25050_d_n11;
        locals.var_t2__blk1146_dn12 = assign31080_e25050_d_n12;

        let (assign31090_e25064, assign31090_e25064_d_n3, assign31090_e25064_d_n4, assign31090_e25064_d_n5, assign31090_e25064_d_n6, assign31090_e25064_d_n7, assign31090_e25064_d_n8, assign31090_e25064_d_n9, assign31090_e25064_d_n10, assign31090_e25064_d_n11, assign31090_e25064_d_n12,) = {
    if ((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1731 != 0.0)) {
        let assign31090_e25062: f64 = (locals.var_t2__blk1146 * locals.var_ccen2);
        (assign31090_e25062, ((locals.var_t2__blk1146_dn3 * locals.var_ccen2) + (locals.var_t2__blk1146 * locals.var_ccen2_dn3)), ((locals.var_t2__blk1146_dn4 * locals.var_ccen2) + (locals.var_t2__blk1146 * locals.var_ccen2_dn4)), ((locals.var_t2__blk1146_dn5 * locals.var_ccen2) + (locals.var_t2__blk1146 * locals.var_ccen2_dn5)), ((locals.var_t2__blk1146_dn6 * locals.var_ccen2) + (locals.var_t2__blk1146 * locals.var_ccen2_dn6)), ((locals.var_t2__blk1146_dn7 * locals.var_ccen2) + (locals.var_t2__blk1146 * locals.var_ccen2_dn7)), ((locals.var_t2__blk1146_dn8 * locals.var_ccen2) + (locals.var_t2__blk1146 * locals.var_ccen2_dn8)), ((locals.var_t2__blk1146_dn9 * locals.var_ccen2) + (locals.var_t2__blk1146 * locals.var_ccen2_dn9)), ((locals.var_t2__blk1146_dn10 * locals.var_ccen2) + (locals.var_t2__blk1146 * locals.var_ccen2_dn10)), ((locals.var_t2__blk1146_dn11 * locals.var_ccen2) + (locals.var_t2__blk1146 * locals.var_ccen2_dn11)), ((locals.var_t2__blk1146_dn12 * locals.var_ccen2) + (locals.var_t2__blk1146 * locals.var_ccen2_dn12)),)
    } else {
        (locals.var_coxeff2, locals.var_coxeff2_dn3, locals.var_coxeff2_dn4, locals.var_coxeff2_dn5, locals.var_coxeff2_dn6, locals.var_coxeff2_dn7, locals.var_coxeff2_dn8, locals.var_coxeff2_dn9, locals.var_coxeff2_dn10, locals.var_coxeff2_dn11, locals.var_coxeff2_dn12,)
    }
};
        locals.var_coxeff2 = assign31090_e25064;
        locals.var_coxeff2_dn3 = assign31090_e25064_d_n3;
        locals.var_coxeff2_dn4 = assign31090_e25064_d_n4;
        locals.var_coxeff2_dn5 = assign31090_e25064_d_n5;
        locals.var_coxeff2_dn6 = assign31090_e25064_d_n6;
        locals.var_coxeff2_dn7 = assign31090_e25064_d_n7;
        locals.var_coxeff2_dn8 = assign31090_e25064_d_n8;
        locals.var_coxeff2_dn9 = assign31090_e25064_d_n9;
        locals.var_coxeff2_dn10 = assign31090_e25064_d_n10;
        locals.var_coxeff2_dn11 = assign31090_e25064_d_n11;
        locals.var_coxeff2_dn12 = assign31090_e25064_d_n12;

        let (assign31100_e25078, assign31100_e25078_d_n3, assign31100_e25078_d_n4, assign31100_e25078_d_n5, assign31100_e25078_d_n6, assign31100_e25078_d_n7, assign31100_e25078_d_n8, assign31100_e25078_d_n9, assign31100_e25078_d_n10, assign31100_e25078_d_n11, assign31100_e25078_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) {
        let assign31100_e25074: f64 = (locals.var_coxwlb * locals.var_coxeff);
        let assign31100_e25076: f64 = (assign31100_e25074 / locals.var_cox);
        (assign31100_e25076, (((((locals.var_coxwlb_dn3 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn3)) * locals.var_cox) - (assign31100_e25074 * locals.var_cox_dn3)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn4 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn4)) * locals.var_cox) - (assign31100_e25074 * locals.var_cox_dn4)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn5 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn5)) * locals.var_cox) - (assign31100_e25074 * locals.var_cox_dn5)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn6 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn6)) * locals.var_cox) - (assign31100_e25074 * locals.var_cox_dn6)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn7 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn7)) * locals.var_cox) - (assign31100_e25074 * locals.var_cox_dn7)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn8 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn8)) * locals.var_cox) - (assign31100_e25074 * locals.var_cox_dn8)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn9 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn9)) * locals.var_cox) - (assign31100_e25074 * locals.var_cox_dn9)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn10 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn10)) * locals.var_cox) - (assign31100_e25074 * locals.var_cox_dn10)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn11 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn11)) * locals.var_cox) - (assign31100_e25074 * locals.var_cox_dn11)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn12 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn12)) * locals.var_cox) - (assign31100_e25074 * locals.var_cox_dn12)) / (locals.var_cox * locals.var_cox)),)
    } else {
        (locals.var_coxwlcenb, locals.var_coxwlcenb_dn3, locals.var_coxwlcenb_dn4, locals.var_coxwlcenb_dn5, locals.var_coxwlcenb_dn6, locals.var_coxwlcenb_dn7, locals.var_coxwlcenb_dn8, locals.var_coxwlcenb_dn9, locals.var_coxwlcenb_dn10, locals.var_coxwlcenb_dn11, locals.var_coxwlcenb_dn12,)
    }
};
        locals.var_coxwlcenb = assign31100_e25078;
        locals.var_coxwlcenb_dn3 = assign31100_e25078_d_n3;
        locals.var_coxwlcenb_dn4 = assign31100_e25078_d_n4;
        locals.var_coxwlcenb_dn5 = assign31100_e25078_d_n5;
        locals.var_coxwlcenb_dn6 = assign31100_e25078_d_n6;
        locals.var_coxwlcenb_dn7 = assign31100_e25078_d_n7;
        locals.var_coxwlcenb_dn8 = assign31100_e25078_d_n8;
        locals.var_coxwlcenb_dn9 = assign31100_e25078_d_n9;
        locals.var_coxwlcenb_dn10 = assign31100_e25078_d_n10;
        locals.var_coxwlcenb_dn11 = assign31100_e25078_d_n11;
        locals.var_coxwlcenb_dn12 = assign31100_e25078_d_n12;

        let assign31110_e25081: f64 = if locals.var_b4soiagbcp2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1732 = assign31110_e25081;

        let (assign31120_e25097, assign31120_e25097_d_n3, assign31120_e25097_d_n4, assign31120_e25097_d_n5, assign31120_e25097_d_n6, assign31120_e25097_d_n7, assign31120_e25097_d_n8, assign31120_e25097_d_n9, assign31120_e25097_d_n10, assign31120_e25097_d_n11, assign31120_e25097_d_n12,) = {
    if ((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1732 != 0.0)) {
        let assign31120_e25093: f64 = (locals.var_coxwlb2 * locals.var_coxeff2);
        let assign31120_e25095: f64 = (assign31120_e25093 / locals.var_cox);
        (assign31120_e25095, (((((locals.var_coxwlb2_dn3 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn3)) * locals.var_cox) - (assign31120_e25093 * locals.var_cox_dn3)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn4 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn4)) * locals.var_cox) - (assign31120_e25093 * locals.var_cox_dn4)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn5 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn5)) * locals.var_cox) - (assign31120_e25093 * locals.var_cox_dn5)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn6 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn6)) * locals.var_cox) - (assign31120_e25093 * locals.var_cox_dn6)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn7 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn7)) * locals.var_cox) - (assign31120_e25093 * locals.var_cox_dn7)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn8 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn8)) * locals.var_cox) - (assign31120_e25093 * locals.var_cox_dn8)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn9 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn9)) * locals.var_cox) - (assign31120_e25093 * locals.var_cox_dn9)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn10 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn10)) * locals.var_cox) - (assign31120_e25093 * locals.var_cox_dn10)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn11 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn11)) * locals.var_cox) - (assign31120_e25093 * locals.var_cox_dn11)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn12 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn12)) * locals.var_cox) - (assign31120_e25093 * locals.var_cox_dn12)) / (locals.var_cox * locals.var_cox)),)
    } else {
        (locals.var_coxwlcenb2, locals.var_coxwlcenb2_dn3, locals.var_coxwlcenb2_dn4, locals.var_coxwlcenb2_dn5, locals.var_coxwlcenb2_dn6, locals.var_coxwlcenb2_dn7, locals.var_coxwlcenb2_dn8, locals.var_coxwlcenb2_dn9, locals.var_coxwlcenb2_dn10, locals.var_coxwlcenb2_dn11, locals.var_coxwlcenb2_dn12,)
    }
};
        locals.var_coxwlcenb2 = assign31120_e25097;
        locals.var_coxwlcenb2_dn3 = assign31120_e25097_d_n3;
        locals.var_coxwlcenb2_dn4 = assign31120_e25097_d_n4;
        locals.var_coxwlcenb2_dn5 = assign31120_e25097_d_n5;
        locals.var_coxwlcenb2_dn6 = assign31120_e25097_d_n6;
        locals.var_coxwlcenb2_dn7 = assign31120_e25097_d_n7;
        locals.var_coxwlcenb2_dn8 = assign31120_e25097_d_n8;
        locals.var_coxwlcenb2_dn9 = assign31120_e25097_d_n9;
        locals.var_coxwlcenb2_dn10 = assign31120_e25097_d_n10;
        locals.var_coxwlcenb2_dn11 = assign31120_e25097_d_n11;
        locals.var_coxwlcenb2_dn12 = assign31120_e25097_d_n12;

        let (assign31130_e25111, assign31130_e25111_d_n3, assign31130_e25111_d_n4, assign31130_e25111_d_n5, assign31130_e25111_d_n6, assign31130_e25111_d_n7, assign31130_e25111_d_n8, assign31130_e25111_d_n9, assign31130_e25111_d_n10, assign31130_e25111_d_n11, assign31130_e25111_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) {
        let assign31130_e25108: f64 = (locals.var_vfbeff - locals.var_vfbzb);
        let assign31130_e25109: f64 = (locals.var_coxwlcenb * assign31130_e25108);
        (assign31130_e25109, ((locals.var_coxwlcenb_dn3 * assign31130_e25108) + (locals.var_coxwlcenb * (locals.var_vfbeff_dn3 - locals.var_vfbzb_dn3))), ((locals.var_coxwlcenb_dn4 * assign31130_e25108) + (locals.var_coxwlcenb * (locals.var_vfbeff_dn4 - locals.var_vfbzb_dn4))), ((locals.var_coxwlcenb_dn5 * assign31130_e25108) + (locals.var_coxwlcenb * (locals.var_vfbeff_dn5 - locals.var_vfbzb_dn5))), ((locals.var_coxwlcenb_dn6 * assign31130_e25108) + (locals.var_coxwlcenb * (locals.var_vfbeff_dn6 - locals.var_vfbzb_dn6))), ((locals.var_coxwlcenb_dn7 * assign31130_e25108) + (locals.var_coxwlcenb * (locals.var_vfbeff_dn7 - locals.var_vfbzb_dn7))), ((locals.var_coxwlcenb_dn8 * assign31130_e25108) + (locals.var_coxwlcenb * (locals.var_vfbeff_dn8 - locals.var_vfbzb_dn8))), ((locals.var_coxwlcenb_dn9 * assign31130_e25108) + (locals.var_coxwlcenb * (locals.var_vfbeff_dn9 - locals.var_vfbzb_dn9))), ((locals.var_coxwlcenb_dn10 * assign31130_e25108) + (locals.var_coxwlcenb * (locals.var_vfbeff_dn10 - locals.var_vfbzb_dn10))), ((locals.var_coxwlcenb_dn11 * assign31130_e25108) + (locals.var_coxwlcenb * (locals.var_vfbeff_dn11 - locals.var_vfbzb_dn11))), ((locals.var_coxwlcenb_dn12 * assign31130_e25108) + (locals.var_coxwlcenb * (locals.var_vfbeff_dn12 - locals.var_vfbzb_dn12))),)
    } else {
        (locals.var_qac0, locals.var_qac0_dn3, locals.var_qac0_dn4, locals.var_qac0_dn5, locals.var_qac0_dn6, locals.var_qac0_dn7, locals.var_qac0_dn8, locals.var_qac0_dn9, locals.var_qac0_dn10, locals.var_qac0_dn11, locals.var_qac0_dn12,)
    }
};
        locals.var_qac0 = assign31130_e25111;
        locals.var_qac0_dn3 = assign31130_e25111_d_n3;
        locals.var_qac0_dn4 = assign31130_e25111_d_n4;
        locals.var_qac0_dn5 = assign31130_e25111_d_n5;
        locals.var_qac0_dn6 = assign31130_e25111_d_n6;
        locals.var_qac0_dn7 = assign31130_e25111_d_n7;
        locals.var_qac0_dn8 = assign31130_e25111_d_n8;
        locals.var_qac0_dn9 = assign31130_e25111_d_n9;
        locals.var_qac0_dn10 = assign31130_e25111_d_n10;
        locals.var_qac0_dn11 = assign31130_e25111_d_n11;
        locals.var_qac0_dn12 = assign31130_e25111_d_n12;

        let assign31140_e25122: f64 = if (((locals.var_b4soisoimod != 2.0) && (locals.var_b4soibodymod != 0.0)) && (locals.var_b4soiagbcp2 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1733 = assign31140_e25122;

        let (assign31150_e25138, assign31150_e25138_d_n3, assign31150_e25138_d_n4, assign31150_e25138_d_n5, assign31150_e25138_d_n6, assign31150_e25138_d_n7, assign31150_e25138_d_n8, assign31150_e25138_d_n9, assign31150_e25138_d_n10, assign31150_e25138_d_n11, assign31150_e25138_d_n12,) = {
    if ((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1733 != 0.0)) {
        let assign31150_e25135: f64 = (locals.var_vfbeff2 - locals.var_vfbzb2);
        let assign31150_e25136: f64 = (locals.var_coxwlcenb2 * assign31150_e25135);
        (assign31150_e25136, ((locals.var_coxwlcenb2_dn3 * assign31150_e25135) + (locals.var_coxwlcenb2 * (locals.var_vfbeff2_dn3 - locals.var_vfbzb2_dn3))), ((locals.var_coxwlcenb2_dn4 * assign31150_e25135) + (locals.var_coxwlcenb2 * (locals.var_vfbeff2_dn4 - locals.var_vfbzb2_dn4))), ((locals.var_coxwlcenb2_dn5 * assign31150_e25135) + (locals.var_coxwlcenb2 * (locals.var_vfbeff2_dn5 - locals.var_vfbzb2_dn5))), ((locals.var_coxwlcenb2_dn6 * assign31150_e25135) + (locals.var_coxwlcenb2 * (locals.var_vfbeff2_dn6 - locals.var_vfbzb2_dn6))), ((locals.var_coxwlcenb2_dn7 * assign31150_e25135) + (locals.var_coxwlcenb2 * (locals.var_vfbeff2_dn7 - locals.var_vfbzb2_dn7))), ((locals.var_coxwlcenb2_dn8 * assign31150_e25135) + (locals.var_coxwlcenb2 * (locals.var_vfbeff2_dn8 - locals.var_vfbzb2_dn8))), ((locals.var_coxwlcenb2_dn9 * assign31150_e25135) + (locals.var_coxwlcenb2 * (locals.var_vfbeff2_dn9 - locals.var_vfbzb2_dn9))), ((locals.var_coxwlcenb2_dn10 * assign31150_e25135) + (locals.var_coxwlcenb2 * (locals.var_vfbeff2_dn10 - locals.var_vfbzb2_dn10))), ((locals.var_coxwlcenb2_dn11 * assign31150_e25135) + (locals.var_coxwlcenb2 * (locals.var_vfbeff2_dn11 - locals.var_vfbzb2_dn11))), ((locals.var_coxwlcenb2_dn12 * assign31150_e25135) + (locals.var_coxwlcenb2 * (locals.var_vfbeff2_dn12 - locals.var_vfbzb2_dn12))),)
    } else {
        (locals.var_qac02, locals.var_qac02_dn3, locals.var_qac02_dn4, locals.var_qac02_dn5, locals.var_qac02_dn6, locals.var_qac02_dn7, locals.var_qac02_dn8, locals.var_qac02_dn9, locals.var_qac02_dn10, locals.var_qac02_dn11, locals.var_qac02_dn12,)
    }
};
        locals.var_qac02 = assign31150_e25138;
        locals.var_qac02_dn3 = assign31150_e25138_d_n3;
        locals.var_qac02_dn4 = assign31150_e25138_d_n4;
        locals.var_qac02_dn5 = assign31150_e25138_d_n5;
        locals.var_qac02_dn6 = assign31150_e25138_d_n6;
        locals.var_qac02_dn7 = assign31150_e25138_d_n7;
        locals.var_qac02_dn8 = assign31150_e25138_d_n8;
        locals.var_qac02_dn9 = assign31150_e25138_d_n9;
        locals.var_qac02_dn10 = assign31150_e25138_d_n10;
        locals.var_qac02_dn11 = assign31150_e25138_d_n11;
        locals.var_qac02_dn12 = assign31150_e25138_d_n12;

        let (assign31160_e25152, assign31160_e25152_d_n3, assign31160_e25152_d_n4, assign31160_e25152_d_n5, assign31160_e25152_d_n6, assign31160_e25152_d_n7, assign31160_e25152_d_n8, assign31160_e25152_d_n9, assign31160_e25152_d_n10, assign31160_e25152_d_n11, assign31160_e25152_d_n12,) = {
    if ((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1733 != 0.0)) {
        let assign31160_e25150: f64 = (locals.var_qac0 + locals.var_qac02);
        (assign31160_e25150, (locals.var_qac0_dn3 + locals.var_qac02_dn3), (locals.var_qac0_dn4 + locals.var_qac02_dn4), (locals.var_qac0_dn5 + locals.var_qac02_dn5), (locals.var_qac0_dn6 + locals.var_qac02_dn6), (locals.var_qac0_dn7 + locals.var_qac02_dn7), (locals.var_qac0_dn8 + locals.var_qac02_dn8), (locals.var_qac0_dn9 + locals.var_qac02_dn9), (locals.var_qac0_dn10 + locals.var_qac02_dn10), (locals.var_qac0_dn11 + locals.var_qac02_dn11), (locals.var_qac0_dn12 + locals.var_qac02_dn12),)
    } else {
        (locals.var_qac0, locals.var_qac0_dn3, locals.var_qac0_dn4, locals.var_qac0_dn5, locals.var_qac0_dn6, locals.var_qac0_dn7, locals.var_qac0_dn8, locals.var_qac0_dn9, locals.var_qac0_dn10, locals.var_qac0_dn11, locals.var_qac0_dn12,)
    }
};
        locals.var_qac0 = assign31160_e25152;
        locals.var_qac0_dn3 = assign31160_e25152_d_n3;
        locals.var_qac0_dn4 = assign31160_e25152_d_n4;
        locals.var_qac0_dn5 = assign31160_e25152_d_n5;
        locals.var_qac0_dn6 = assign31160_e25152_d_n6;
        locals.var_qac0_dn7 = assign31160_e25152_d_n7;
        locals.var_qac0_dn8 = assign31160_e25152_d_n8;
        locals.var_qac0_dn9 = assign31160_e25152_d_n9;
        locals.var_qac0_dn10 = assign31160_e25152_d_n10;
        locals.var_qac0_dn11 = assign31160_e25152_d_n11;
        locals.var_qac0_dn12 = assign31160_e25152_d_n12;

        let (assign31170_e25164, assign31170_e25164_d_n3, assign31170_e25164_d_n4, assign31170_e25164_d_n5, assign31170_e25164_d_n6, assign31170_e25164_d_n7, assign31170_e25164_d_n8, assign31170_e25164_d_n9, assign31170_e25164_d_n10, assign31170_e25164_d_n11, assign31170_e25164_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) {
        let assign31170_e25162: f64 = (0.5 * locals.var_pparam_b4soik1ox);
        (assign31170_e25162, (0.5 * locals.var_pparam_b4soik1ox_dn3), (0.5 * locals.var_pparam_b4soik1ox_dn4), (0.5 * locals.var_pparam_b4soik1ox_dn5), (0.5 * locals.var_pparam_b4soik1ox_dn6), (0.5 * locals.var_pparam_b4soik1ox_dn7), (0.5 * locals.var_pparam_b4soik1ox_dn8), (0.5 * locals.var_pparam_b4soik1ox_dn9), (0.5 * locals.var_pparam_b4soik1ox_dn10), (0.5 * locals.var_pparam_b4soik1ox_dn11), (0.5 * locals.var_pparam_b4soik1ox_dn12),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign31170_e25164;
        locals.var_t0__blk1144_dn3 = assign31170_e25164_d_n3;
        locals.var_t0__blk1144_dn4 = assign31170_e25164_d_n4;
        locals.var_t0__blk1144_dn5 = assign31170_e25164_d_n5;
        locals.var_t0__blk1144_dn6 = assign31170_e25164_d_n6;
        locals.var_t0__blk1144_dn7 = assign31170_e25164_d_n7;
        locals.var_t0__blk1144_dn8 = assign31170_e25164_d_n8;
        locals.var_t0__blk1144_dn9 = assign31170_e25164_d_n9;
        locals.var_t0__blk1144_dn10 = assign31170_e25164_d_n10;
        locals.var_t0__blk1144_dn11 = assign31170_e25164_d_n11;
        locals.var_t0__blk1144_dn12 = assign31170_e25164_d_n12;

        let (assign31180_e25180, assign31180_e25180_d_n3, assign31180_e25180_d_n4, assign31180_e25180_d_n5, assign31180_e25180_d_n6, assign31180_e25180_d_n7, assign31180_e25180_d_n8, assign31180_e25180_d_n9, assign31180_e25180_d_n10, assign31180_e25180_d_n11, assign31180_e25180_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) {
        let assign31180_e25174: f64 = (locals.var_vgs_eff__blk1126 - locals.var_vfbeff);
        let assign31180_e25176: f64 = (assign31180_e25174 - locals.var_vbseff);
        let assign31180_e25178: f64 = (assign31180_e25176 - locals.var_vgsteff__blk1175);
        (assign31180_e25178, (((locals.var_vgs_eff__blk1126_dn3 - locals.var_vfbeff_dn3) - locals.var_vbseff_dn3) - locals.var_vgsteff__blk1175_dn3), (((locals.var_vgs_eff__blk1126_dn4 - locals.var_vfbeff_dn4) - locals.var_vbseff_dn4) - locals.var_vgsteff__blk1175_dn4), (((locals.var_vgs_eff__blk1126_dn5 - locals.var_vfbeff_dn5) - locals.var_vbseff_dn5) - locals.var_vgsteff__blk1175_dn5), (((locals.var_vgs_eff__blk1126_dn6 - locals.var_vfbeff_dn6) - locals.var_vbseff_dn6) - locals.var_vgsteff__blk1175_dn6), (((locals.var_vgs_eff__blk1126_dn7 - locals.var_vfbeff_dn7) - locals.var_vbseff_dn7) - locals.var_vgsteff__blk1175_dn7), (((locals.var_vgs_eff__blk1126_dn8 - locals.var_vfbeff_dn8) - locals.var_vbseff_dn8) - locals.var_vgsteff__blk1175_dn8), (((locals.var_vgs_eff__blk1126_dn9 - locals.var_vfbeff_dn9) - locals.var_vbseff_dn9) - locals.var_vgsteff__blk1175_dn9), (((locals.var_vgs_eff__blk1126_dn10 - locals.var_vfbeff_dn10) - locals.var_vbseff_dn10) - locals.var_vgsteff__blk1175_dn10), (((locals.var_vgs_eff__blk1126_dn11 - locals.var_vfbeff_dn11) - locals.var_vbseff_dn11) - locals.var_vgsteff__blk1175_dn11), (((locals.var_vgs_eff__blk1126_dn12 - locals.var_vfbeff_dn12) - locals.var_vbseff_dn12) - locals.var_vgsteff__blk1175_dn12),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign31180_e25180;
        locals.var_t3__blk1147_dn3 = assign31180_e25180_d_n3;
        locals.var_t3__blk1147_dn4 = assign31180_e25180_d_n4;
        locals.var_t3__blk1147_dn5 = assign31180_e25180_d_n5;
        locals.var_t3__blk1147_dn6 = assign31180_e25180_d_n6;
        locals.var_t3__blk1147_dn7 = assign31180_e25180_d_n7;
        locals.var_t3__blk1147_dn8 = assign31180_e25180_d_n8;
        locals.var_t3__blk1147_dn9 = assign31180_e25180_d_n9;
        locals.var_t3__blk1147_dn10 = assign31180_e25180_d_n10;
        locals.var_t3__blk1147_dn11 = assign31180_e25180_d_n11;
        locals.var_t3__blk1147_dn12 = assign31180_e25180_d_n12;

        let assign31190_e25183: f64 = if locals.var_pparam_b4soik1ox == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1734 = assign31190_e25183;

        let (assign31200_e25195, assign31200_e25195_d_n3, assign31200_e25195_d_n4, assign31200_e25195_d_n5, assign31200_e25195_d_n6, assign31200_e25195_d_n7, assign31200_e25195_d_n8, assign31200_e25195_d_n9, assign31200_e25195_d_n10, assign31200_e25195_d_n11, assign31200_e25195_d_n12,) = {
    if ((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1734 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign31200_e25195;
        locals.var_t1__blk1145_dn3 = assign31200_e25195_d_n3;
        locals.var_t1__blk1145_dn4 = assign31200_e25195_d_n4;
        locals.var_t1__blk1145_dn5 = assign31200_e25195_d_n5;
        locals.var_t1__blk1145_dn6 = assign31200_e25195_d_n6;
        locals.var_t1__blk1145_dn7 = assign31200_e25195_d_n7;
        locals.var_t1__blk1145_dn8 = assign31200_e25195_d_n8;
        locals.var_t1__blk1145_dn9 = assign31200_e25195_d_n9;
        locals.var_t1__blk1145_dn10 = assign31200_e25195_d_n10;
        locals.var_t1__blk1145_dn11 = assign31200_e25195_d_n11;
        locals.var_t1__blk1145_dn12 = assign31200_e25195_d_n12;

        let assign31210_e25198: f64 = if locals.var_t3__blk1147 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1735 = assign31210_e25198;

        let (assign31220_e25217, assign31220_e25217_d_n3, assign31220_e25217_d_n4, assign31220_e25217_d_n5, assign31220_e25217_d_n6, assign31220_e25217_d_n7, assign31220_e25217_d_n8, assign31220_e25217_d_n9, assign31220_e25217_d_n10, assign31220_e25217_d_n11, assign31220_e25217_d_n12,) = {
    if (((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1734 == 0.0)) && (locals.var_guard1735 != 0.0)) {
        let assign31220_e25214: f64 = (locals.var_t3__blk1147 / locals.var_pparam_b4soik1ox);
        let assign31220_e25215: f64 = (locals.var_t0__blk1144 + assign31220_e25214);
        (assign31220_e25215, (locals.var_t0__blk1144_dn3 + (((locals.var_t3__blk1147_dn3 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn3)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn4 + (((locals.var_t3__blk1147_dn4 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn4)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn5 + (((locals.var_t3__blk1147_dn5 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn5)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn6 + (((locals.var_t3__blk1147_dn6 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn6)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn7 + (((locals.var_t3__blk1147_dn7 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn7)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn8 + (((locals.var_t3__blk1147_dn8 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn8)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn9 + (((locals.var_t3__blk1147_dn9 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn9)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn10 + (((locals.var_t3__blk1147_dn10 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn10)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn11 + (((locals.var_t3__blk1147_dn11 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn11)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn12 + (((locals.var_t3__blk1147_dn12 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn12)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign31220_e25217;
        locals.var_t1__blk1145_dn3 = assign31220_e25217_d_n3;
        locals.var_t1__blk1145_dn4 = assign31220_e25217_d_n4;
        locals.var_t1__blk1145_dn5 = assign31220_e25217_d_n5;
        locals.var_t1__blk1145_dn6 = assign31220_e25217_d_n6;
        locals.var_t1__blk1145_dn7 = assign31220_e25217_d_n7;
        locals.var_t1__blk1145_dn8 = assign31220_e25217_d_n8;
        locals.var_t1__blk1145_dn9 = assign31220_e25217_d_n9;
        locals.var_t1__blk1145_dn10 = assign31220_e25217_d_n10;
        locals.var_t1__blk1145_dn11 = assign31220_e25217_d_n11;
        locals.var_t1__blk1145_dn12 = assign31220_e25217_d_n12;

        let (assign31230_e25238, assign31230_e25238_d_n3, assign31230_e25238_d_n4, assign31230_e25238_d_n5, assign31230_e25238_d_n6, assign31230_e25238_d_n7, assign31230_e25238_d_n8, assign31230_e25238_d_n9, assign31230_e25238_d_n10, assign31230_e25238_d_n11, assign31230_e25238_d_n12,) = {
    if (((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1734 == 0.0)) && (locals.var_guard1735 == 0.0)) {
        let assign31230_e25233: f64 = (locals.var_t0__blk1144 * locals.var_t0__blk1144);
        let assign31230_e25235: f64 = (assign31230_e25233 + locals.var_t3__blk1147);
        let assign31230_e25236: f64 = (assign31230_e25235).sqrt();
        (assign31230_e25236, ((((locals.var_t0__blk1144_dn3 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn3)) + locals.var_t3__blk1147_dn3) / (2.0 * assign31230_e25236)), ((((locals.var_t0__blk1144_dn4 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn4)) + locals.var_t3__blk1147_dn4) / (2.0 * assign31230_e25236)), ((((locals.var_t0__blk1144_dn5 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn5)) + locals.var_t3__blk1147_dn5) / (2.0 * assign31230_e25236)), ((((locals.var_t0__blk1144_dn6 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn6)) + locals.var_t3__blk1147_dn6) / (2.0 * assign31230_e25236)), ((((locals.var_t0__blk1144_dn7 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn7)) + locals.var_t3__blk1147_dn7) / (2.0 * assign31230_e25236)), ((((locals.var_t0__blk1144_dn8 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn8)) + locals.var_t3__blk1147_dn8) / (2.0 * assign31230_e25236)), ((((locals.var_t0__blk1144_dn9 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn9)) + locals.var_t3__blk1147_dn9) / (2.0 * assign31230_e25236)), ((((locals.var_t0__blk1144_dn10 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn10)) + locals.var_t3__blk1147_dn10) / (2.0 * assign31230_e25236)), ((((locals.var_t0__blk1144_dn11 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn11)) + locals.var_t3__blk1147_dn11) / (2.0 * assign31230_e25236)), ((((locals.var_t0__blk1144_dn12 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn12)) + locals.var_t3__blk1147_dn12) / (2.0 * assign31230_e25236)),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign31230_e25238;
        locals.var_t1__blk1145_dn3 = assign31230_e25238_d_n3;
        locals.var_t1__blk1145_dn4 = assign31230_e25238_d_n4;
        locals.var_t1__blk1145_dn5 = assign31230_e25238_d_n5;
        locals.var_t1__blk1145_dn6 = assign31230_e25238_d_n6;
        locals.var_t1__blk1145_dn7 = assign31230_e25238_d_n7;
        locals.var_t1__blk1145_dn8 = assign31230_e25238_d_n8;
        locals.var_t1__blk1145_dn9 = assign31230_e25238_d_n9;
        locals.var_t1__blk1145_dn10 = assign31230_e25238_d_n10;
        locals.var_t1__blk1145_dn11 = assign31230_e25238_d_n11;
        locals.var_t1__blk1145_dn12 = assign31230_e25238_d_n12;

    }

    pub(super) fn stamp_transient_block_85(
        locals: &mut StampLocals,
    ) {
        let (assign31240_e25254, assign31240_e25254_d_n3, assign31240_e25254_d_n4, assign31240_e25254_d_n5, assign31240_e25254_d_n6, assign31240_e25254_d_n7, assign31240_e25254_d_n8, assign31240_e25254_d_n9, assign31240_e25254_d_n10, assign31240_e25254_d_n11, assign31240_e25254_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) {
        let assign31240_e25248: f64 = (locals.var_coxwlcenb * locals.var_pparam_b4soik1ox);
        let assign31240_e25251: f64 = (locals.var_t1__blk1145 - locals.var_t0__blk1144);
        let assign31240_e25252: f64 = (assign31240_e25248 * assign31240_e25251);
        (assign31240_e25252, ((((locals.var_coxwlcenb_dn3 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlcenb * locals.var_pparam_b4soik1ox_dn3)) * assign31240_e25251) + (assign31240_e25248 * (locals.var_t1__blk1145_dn3 - locals.var_t0__blk1144_dn3))), ((((locals.var_coxwlcenb_dn4 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlcenb * locals.var_pparam_b4soik1ox_dn4)) * assign31240_e25251) + (assign31240_e25248 * (locals.var_t1__blk1145_dn4 - locals.var_t0__blk1144_dn4))), ((((locals.var_coxwlcenb_dn5 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlcenb * locals.var_pparam_b4soik1ox_dn5)) * assign31240_e25251) + (assign31240_e25248 * (locals.var_t1__blk1145_dn5 - locals.var_t0__blk1144_dn5))), ((((locals.var_coxwlcenb_dn6 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlcenb * locals.var_pparam_b4soik1ox_dn6)) * assign31240_e25251) + (assign31240_e25248 * (locals.var_t1__blk1145_dn6 - locals.var_t0__blk1144_dn6))), ((((locals.var_coxwlcenb_dn7 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlcenb * locals.var_pparam_b4soik1ox_dn7)) * assign31240_e25251) + (assign31240_e25248 * (locals.var_t1__blk1145_dn7 - locals.var_t0__blk1144_dn7))), ((((locals.var_coxwlcenb_dn8 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlcenb * locals.var_pparam_b4soik1ox_dn8)) * assign31240_e25251) + (assign31240_e25248 * (locals.var_t1__blk1145_dn8 - locals.var_t0__blk1144_dn8))), ((((locals.var_coxwlcenb_dn9 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlcenb * locals.var_pparam_b4soik1ox_dn9)) * assign31240_e25251) + (assign31240_e25248 * (locals.var_t1__blk1145_dn9 - locals.var_t0__blk1144_dn9))), ((((locals.var_coxwlcenb_dn10 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlcenb * locals.var_pparam_b4soik1ox_dn10)) * assign31240_e25251) + (assign31240_e25248 * (locals.var_t1__blk1145_dn10 - locals.var_t0__blk1144_dn10))), ((((locals.var_coxwlcenb_dn11 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlcenb * locals.var_pparam_b4soik1ox_dn11)) * assign31240_e25251) + (assign31240_e25248 * (locals.var_t1__blk1145_dn11 - locals.var_t0__blk1144_dn11))), ((((locals.var_coxwlcenb_dn12 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlcenb * locals.var_pparam_b4soik1ox_dn12)) * assign31240_e25251) + (assign31240_e25248 * (locals.var_t1__blk1145_dn12 - locals.var_t0__blk1144_dn12))),)
    } else {
        (locals.var_qsub0, locals.var_qsub0_dn3, locals.var_qsub0_dn4, locals.var_qsub0_dn5, locals.var_qsub0_dn6, locals.var_qsub0_dn7, locals.var_qsub0_dn8, locals.var_qsub0_dn9, locals.var_qsub0_dn10, locals.var_qsub0_dn11, locals.var_qsub0_dn12,)
    }
};
        locals.var_qsub0 = assign31240_e25254;
        locals.var_qsub0_dn3 = assign31240_e25254_d_n3;
        locals.var_qsub0_dn4 = assign31240_e25254_d_n4;
        locals.var_qsub0_dn5 = assign31240_e25254_d_n5;
        locals.var_qsub0_dn6 = assign31240_e25254_d_n6;
        locals.var_qsub0_dn7 = assign31240_e25254_d_n7;
        locals.var_qsub0_dn8 = assign31240_e25254_d_n8;
        locals.var_qsub0_dn9 = assign31240_e25254_d_n9;
        locals.var_qsub0_dn10 = assign31240_e25254_d_n10;
        locals.var_qsub0_dn11 = assign31240_e25254_d_n11;
        locals.var_qsub0_dn12 = assign31240_e25254_d_n12;

        let assign31250_e25265: f64 = if (((locals.var_b4soisoimod != 2.0) && (locals.var_b4soibodymod != 0.0)) && (locals.var_b4soiagbcp2 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1736 = assign31250_e25265;

        let (assign31260_e25283, assign31260_e25283_d_n3, assign31260_e25283_d_n4, assign31260_e25283_d_n5, assign31260_e25283_d_n6, assign31260_e25283_d_n7, assign31260_e25283_d_n8, assign31260_e25283_d_n9, assign31260_e25283_d_n10, assign31260_e25283_d_n11, assign31260_e25283_d_n12,) = {
    if ((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1736 != 0.0)) {
        let assign31260_e25277: f64 = (locals.var_vgs_eff2 - locals.var_vfbeff2);
        let assign31260_e25279: f64 = (assign31260_e25277 - locals.var_vbseff);
        let assign31260_e25281: f64 = (assign31260_e25279 - locals.var_vgsteff2);
        (assign31260_e25281, (((-locals.var_vfbeff2_dn3) - locals.var_vbseff_dn3) - locals.var_vgsteff2_dn3), (((-locals.var_vfbeff2_dn4) - locals.var_vbseff_dn4) - locals.var_vgsteff2_dn4), (((-locals.var_vfbeff2_dn5) - locals.var_vbseff_dn5) - locals.var_vgsteff2_dn5), (((-locals.var_vfbeff2_dn6) - locals.var_vbseff_dn6) - locals.var_vgsteff2_dn6), (((locals.var_vgs_eff2_dn7 - locals.var_vfbeff2_dn7) - locals.var_vbseff_dn7) - locals.var_vgsteff2_dn7), (((locals.var_vgs_eff2_dn8 - locals.var_vfbeff2_dn8) - locals.var_vbseff_dn8) - locals.var_vgsteff2_dn8), (((locals.var_vgs_eff2_dn9 - locals.var_vfbeff2_dn9) - locals.var_vbseff_dn9) - locals.var_vgsteff2_dn9), (((-locals.var_vfbeff2_dn10) - locals.var_vbseff_dn10) - locals.var_vgsteff2_dn10), (((-locals.var_vfbeff2_dn11) - locals.var_vbseff_dn11) - locals.var_vgsteff2_dn11), (((-locals.var_vfbeff2_dn12) - locals.var_vbseff_dn12) - locals.var_vgsteff2_dn12),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign31260_e25283;
        locals.var_t3__blk1147_dn3 = assign31260_e25283_d_n3;
        locals.var_t3__blk1147_dn4 = assign31260_e25283_d_n4;
        locals.var_t3__blk1147_dn5 = assign31260_e25283_d_n5;
        locals.var_t3__blk1147_dn6 = assign31260_e25283_d_n6;
        locals.var_t3__blk1147_dn7 = assign31260_e25283_d_n7;
        locals.var_t3__blk1147_dn8 = assign31260_e25283_d_n8;
        locals.var_t3__blk1147_dn9 = assign31260_e25283_d_n9;
        locals.var_t3__blk1147_dn10 = assign31260_e25283_d_n10;
        locals.var_t3__blk1147_dn11 = assign31260_e25283_d_n11;
        locals.var_t3__blk1147_dn12 = assign31260_e25283_d_n12;

        let assign31270_e25286: f64 = if locals.var_pparam_b4soik1ox == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1737 = assign31270_e25286;

        let (assign31280_e25300, assign31280_e25300_d_n3, assign31280_e25300_d_n4, assign31280_e25300_d_n5, assign31280_e25300_d_n6, assign31280_e25300_d_n7, assign31280_e25300_d_n8, assign31280_e25300_d_n9, assign31280_e25300_d_n10, assign31280_e25300_d_n11, assign31280_e25300_d_n12,) = {
    if (((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1736 != 0.0)) && (locals.var_guard1737 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign31280_e25300;
        locals.var_t1__blk1145_dn3 = assign31280_e25300_d_n3;
        locals.var_t1__blk1145_dn4 = assign31280_e25300_d_n4;
        locals.var_t1__blk1145_dn5 = assign31280_e25300_d_n5;
        locals.var_t1__blk1145_dn6 = assign31280_e25300_d_n6;
        locals.var_t1__blk1145_dn7 = assign31280_e25300_d_n7;
        locals.var_t1__blk1145_dn8 = assign31280_e25300_d_n8;
        locals.var_t1__blk1145_dn9 = assign31280_e25300_d_n9;
        locals.var_t1__blk1145_dn10 = assign31280_e25300_d_n10;
        locals.var_t1__blk1145_dn11 = assign31280_e25300_d_n11;
        locals.var_t1__blk1145_dn12 = assign31280_e25300_d_n12;

        let assign31290_e25303: f64 = if locals.var_t3__blk1147 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1738 = assign31290_e25303;

        let (assign31300_e25324, assign31300_e25324_d_n3, assign31300_e25324_d_n4, assign31300_e25324_d_n5, assign31300_e25324_d_n6, assign31300_e25324_d_n7, assign31300_e25324_d_n8, assign31300_e25324_d_n9, assign31300_e25324_d_n10, assign31300_e25324_d_n11, assign31300_e25324_d_n12,) = {
    if ((((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1736 != 0.0)) && (locals.var_guard1737 == 0.0)) && (locals.var_guard1738 != 0.0)) {
        let assign31300_e25321: f64 = (locals.var_t3__blk1147 / locals.var_pparam_b4soik1ox);
        let assign31300_e25322: f64 = (locals.var_t0__blk1144 + assign31300_e25321);
        (assign31300_e25322, (locals.var_t0__blk1144_dn3 + (((locals.var_t3__blk1147_dn3 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn3)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn4 + (((locals.var_t3__blk1147_dn4 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn4)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn5 + (((locals.var_t3__blk1147_dn5 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn5)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn6 + (((locals.var_t3__blk1147_dn6 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn6)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn7 + (((locals.var_t3__blk1147_dn7 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn7)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn8 + (((locals.var_t3__blk1147_dn8 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn8)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn9 + (((locals.var_t3__blk1147_dn9 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn9)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn10 + (((locals.var_t3__blk1147_dn10 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn10)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn11 + (((locals.var_t3__blk1147_dn11 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn11)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))), (locals.var_t0__blk1144_dn12 + (((locals.var_t3__blk1147_dn12 * locals.var_pparam_b4soik1ox) - (locals.var_t3__blk1147 * locals.var_pparam_b4soik1ox_dn12)) / (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soik1ox))),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign31300_e25324;
        locals.var_t1__blk1145_dn3 = assign31300_e25324_d_n3;
        locals.var_t1__blk1145_dn4 = assign31300_e25324_d_n4;
        locals.var_t1__blk1145_dn5 = assign31300_e25324_d_n5;
        locals.var_t1__blk1145_dn6 = assign31300_e25324_d_n6;
        locals.var_t1__blk1145_dn7 = assign31300_e25324_d_n7;
        locals.var_t1__blk1145_dn8 = assign31300_e25324_d_n8;
        locals.var_t1__blk1145_dn9 = assign31300_e25324_d_n9;
        locals.var_t1__blk1145_dn10 = assign31300_e25324_d_n10;
        locals.var_t1__blk1145_dn11 = assign31300_e25324_d_n11;
        locals.var_t1__blk1145_dn12 = assign31300_e25324_d_n12;

        let (assign31310_e25347, assign31310_e25347_d_n3, assign31310_e25347_d_n4, assign31310_e25347_d_n5, assign31310_e25347_d_n6, assign31310_e25347_d_n7, assign31310_e25347_d_n8, assign31310_e25347_d_n9, assign31310_e25347_d_n10, assign31310_e25347_d_n11, assign31310_e25347_d_n12,) = {
    if ((((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1736 != 0.0)) && (locals.var_guard1737 == 0.0)) && (locals.var_guard1738 == 0.0)) {
        let assign31310_e25342: f64 = (locals.var_t0__blk1144 * locals.var_t0__blk1144);
        let assign31310_e25344: f64 = (assign31310_e25342 + locals.var_t3__blk1147);
        let assign31310_e25345: f64 = (assign31310_e25344).sqrt();
        (assign31310_e25345, ((((locals.var_t0__blk1144_dn3 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn3)) + locals.var_t3__blk1147_dn3) / (2.0 * assign31310_e25345)), ((((locals.var_t0__blk1144_dn4 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn4)) + locals.var_t3__blk1147_dn4) / (2.0 * assign31310_e25345)), ((((locals.var_t0__blk1144_dn5 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn5)) + locals.var_t3__blk1147_dn5) / (2.0 * assign31310_e25345)), ((((locals.var_t0__blk1144_dn6 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn6)) + locals.var_t3__blk1147_dn6) / (2.0 * assign31310_e25345)), ((((locals.var_t0__blk1144_dn7 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn7)) + locals.var_t3__blk1147_dn7) / (2.0 * assign31310_e25345)), ((((locals.var_t0__blk1144_dn8 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn8)) + locals.var_t3__blk1147_dn8) / (2.0 * assign31310_e25345)), ((((locals.var_t0__blk1144_dn9 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn9)) + locals.var_t3__blk1147_dn9) / (2.0 * assign31310_e25345)), ((((locals.var_t0__blk1144_dn10 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn10)) + locals.var_t3__blk1147_dn10) / (2.0 * assign31310_e25345)), ((((locals.var_t0__blk1144_dn11 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn11)) + locals.var_t3__blk1147_dn11) / (2.0 * assign31310_e25345)), ((((locals.var_t0__blk1144_dn12 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn12)) + locals.var_t3__blk1147_dn12) / (2.0 * assign31310_e25345)),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign31310_e25347;
        locals.var_t1__blk1145_dn3 = assign31310_e25347_d_n3;
        locals.var_t1__blk1145_dn4 = assign31310_e25347_d_n4;
        locals.var_t1__blk1145_dn5 = assign31310_e25347_d_n5;
        locals.var_t1__blk1145_dn6 = assign31310_e25347_d_n6;
        locals.var_t1__blk1145_dn7 = assign31310_e25347_d_n7;
        locals.var_t1__blk1145_dn8 = assign31310_e25347_d_n8;
        locals.var_t1__blk1145_dn9 = assign31310_e25347_d_n9;
        locals.var_t1__blk1145_dn10 = assign31310_e25347_d_n10;
        locals.var_t1__blk1145_dn11 = assign31310_e25347_d_n11;
        locals.var_t1__blk1145_dn12 = assign31310_e25347_d_n12;

        let (assign31320_e25365, assign31320_e25365_d_n3, assign31320_e25365_d_n4, assign31320_e25365_d_n5, assign31320_e25365_d_n6, assign31320_e25365_d_n7, assign31320_e25365_d_n8, assign31320_e25365_d_n9, assign31320_e25365_d_n10, assign31320_e25365_d_n11, assign31320_e25365_d_n12,) = {
    if ((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1736 != 0.0)) {
        let assign31320_e25359: f64 = (locals.var_coxwlcenb2 * locals.var_pparam_b4soik1ox);
        let assign31320_e25362: f64 = (locals.var_t1__blk1145 - locals.var_t0__blk1144);
        let assign31320_e25363: f64 = (assign31320_e25359 * assign31320_e25362);
        (assign31320_e25363, ((((locals.var_coxwlcenb2_dn3 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlcenb2 * locals.var_pparam_b4soik1ox_dn3)) * assign31320_e25362) + (assign31320_e25359 * (locals.var_t1__blk1145_dn3 - locals.var_t0__blk1144_dn3))), ((((locals.var_coxwlcenb2_dn4 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlcenb2 * locals.var_pparam_b4soik1ox_dn4)) * assign31320_e25362) + (assign31320_e25359 * (locals.var_t1__blk1145_dn4 - locals.var_t0__blk1144_dn4))), ((((locals.var_coxwlcenb2_dn5 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlcenb2 * locals.var_pparam_b4soik1ox_dn5)) * assign31320_e25362) + (assign31320_e25359 * (locals.var_t1__blk1145_dn5 - locals.var_t0__blk1144_dn5))), ((((locals.var_coxwlcenb2_dn6 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlcenb2 * locals.var_pparam_b4soik1ox_dn6)) * assign31320_e25362) + (assign31320_e25359 * (locals.var_t1__blk1145_dn6 - locals.var_t0__blk1144_dn6))), ((((locals.var_coxwlcenb2_dn7 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlcenb2 * locals.var_pparam_b4soik1ox_dn7)) * assign31320_e25362) + (assign31320_e25359 * (locals.var_t1__blk1145_dn7 - locals.var_t0__blk1144_dn7))), ((((locals.var_coxwlcenb2_dn8 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlcenb2 * locals.var_pparam_b4soik1ox_dn8)) * assign31320_e25362) + (assign31320_e25359 * (locals.var_t1__blk1145_dn8 - locals.var_t0__blk1144_dn8))), ((((locals.var_coxwlcenb2_dn9 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlcenb2 * locals.var_pparam_b4soik1ox_dn9)) * assign31320_e25362) + (assign31320_e25359 * (locals.var_t1__blk1145_dn9 - locals.var_t0__blk1144_dn9))), ((((locals.var_coxwlcenb2_dn10 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlcenb2 * locals.var_pparam_b4soik1ox_dn10)) * assign31320_e25362) + (assign31320_e25359 * (locals.var_t1__blk1145_dn10 - locals.var_t0__blk1144_dn10))), ((((locals.var_coxwlcenb2_dn11 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlcenb2 * locals.var_pparam_b4soik1ox_dn11)) * assign31320_e25362) + (assign31320_e25359 * (locals.var_t1__blk1145_dn11 - locals.var_t0__blk1144_dn11))), ((((locals.var_coxwlcenb2_dn12 * locals.var_pparam_b4soik1ox) + (locals.var_coxwlcenb2 * locals.var_pparam_b4soik1ox_dn12)) * assign31320_e25362) + (assign31320_e25359 * (locals.var_t1__blk1145_dn12 - locals.var_t0__blk1144_dn12))),)
    } else {
        (locals.var_qsub02, locals.var_qsub02_dn3, locals.var_qsub02_dn4, locals.var_qsub02_dn5, locals.var_qsub02_dn6, locals.var_qsub02_dn7, locals.var_qsub02_dn8, locals.var_qsub02_dn9, locals.var_qsub02_dn10, locals.var_qsub02_dn11, locals.var_qsub02_dn12,)
    }
};
        locals.var_qsub02 = assign31320_e25365;
        locals.var_qsub02_dn3 = assign31320_e25365_d_n3;
        locals.var_qsub02_dn4 = assign31320_e25365_d_n4;
        locals.var_qsub02_dn5 = assign31320_e25365_d_n5;
        locals.var_qsub02_dn6 = assign31320_e25365_d_n6;
        locals.var_qsub02_dn7 = assign31320_e25365_d_n7;
        locals.var_qsub02_dn8 = assign31320_e25365_d_n8;
        locals.var_qsub02_dn9 = assign31320_e25365_d_n9;
        locals.var_qsub02_dn10 = assign31320_e25365_d_n10;
        locals.var_qsub02_dn11 = assign31320_e25365_d_n11;
        locals.var_qsub02_dn12 = assign31320_e25365_d_n12;

        let (assign31330_e25379, assign31330_e25379_d_n3, assign31330_e25379_d_n4, assign31330_e25379_d_n5, assign31330_e25379_d_n6, assign31330_e25379_d_n7, assign31330_e25379_d_n8, assign31330_e25379_d_n9, assign31330_e25379_d_n10, assign31330_e25379_d_n11, assign31330_e25379_d_n12,) = {
    if ((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1719 == 0.0)) && (locals.var_guard1736 != 0.0)) {
        let assign31330_e25377: f64 = (locals.var_qsub0 + locals.var_qsub02);
        (assign31330_e25377, (locals.var_qsub0_dn3 + locals.var_qsub02_dn3), (locals.var_qsub0_dn4 + locals.var_qsub02_dn4), (locals.var_qsub0_dn5 + locals.var_qsub02_dn5), (locals.var_qsub0_dn6 + locals.var_qsub02_dn6), (locals.var_qsub0_dn7 + locals.var_qsub02_dn7), (locals.var_qsub0_dn8 + locals.var_qsub02_dn8), (locals.var_qsub0_dn9 + locals.var_qsub02_dn9), (locals.var_qsub0_dn10 + locals.var_qsub02_dn10), (locals.var_qsub0_dn11 + locals.var_qsub02_dn11), (locals.var_qsub0_dn12 + locals.var_qsub02_dn12),)
    } else {
        (locals.var_qsub0, locals.var_qsub0_dn3, locals.var_qsub0_dn4, locals.var_qsub0_dn5, locals.var_qsub0_dn6, locals.var_qsub0_dn7, locals.var_qsub0_dn8, locals.var_qsub0_dn9, locals.var_qsub0_dn10, locals.var_qsub0_dn11, locals.var_qsub0_dn12,)
    }
};
        locals.var_qsub0 = assign31330_e25379;
        locals.var_qsub0_dn3 = assign31330_e25379_d_n3;
        locals.var_qsub0_dn4 = assign31330_e25379_d_n4;
        locals.var_qsub0_dn5 = assign31330_e25379_d_n5;
        locals.var_qsub0_dn6 = assign31330_e25379_d_n6;
        locals.var_qsub0_dn7 = assign31330_e25379_d_n7;
        locals.var_qsub0_dn8 = assign31330_e25379_d_n8;
        locals.var_qsub0_dn9 = assign31330_e25379_d_n9;
        locals.var_qsub0_dn10 = assign31330_e25379_d_n10;
        locals.var_qsub0_dn11 = assign31330_e25379_d_n11;
        locals.var_qsub0_dn12 = assign31330_e25379_d_n12;

        let assign31340_e25382: f64 = if locals.var_pparam_b4soik1ox <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1739 = assign31340_e25382;

        let (assign31350_e25395, assign31350_e25395_d_n3, assign31350_e25395_d_n4, assign31350_e25395_d_n5, assign31350_e25395_d_n6, assign31350_e25395_d_n7, assign31350_e25395_d_n8, assign31350_e25395_d_n9, assign31350_e25395_d_n10, assign31350_e25395_d_n11, assign31350_e25395_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1739 != 0.0)) {
        let assign31350_e25391: f64 = (0.25 * locals.var_pparam_b4soimoin);
        let assign31350_e25393: f64 = (assign31350_e25391 * locals.var_vtm);
        (assign31350_e25393, ((0.25 * locals.var_pparam_b4soimoin_dn3) * locals.var_vtm), ((0.25 * locals.var_pparam_b4soimoin_dn4) * locals.var_vtm), ((0.25 * locals.var_pparam_b4soimoin_dn5) * locals.var_vtm), (((0.25 * locals.var_pparam_b4soimoin_dn6) * locals.var_vtm) + (assign31350_e25391 * locals.var_vtm_dn6)), ((0.25 * locals.var_pparam_b4soimoin_dn7) * locals.var_vtm), ((0.25 * locals.var_pparam_b4soimoin_dn8) * locals.var_vtm), ((0.25 * locals.var_pparam_b4soimoin_dn9) * locals.var_vtm), ((0.25 * locals.var_pparam_b4soimoin_dn10) * locals.var_vtm), ((0.25 * locals.var_pparam_b4soimoin_dn11) * locals.var_vtm), ((0.25 * locals.var_pparam_b4soimoin_dn12) * locals.var_vtm),)
    } else {
        (locals.var_denomi, locals.var_denomi_dn3, locals.var_denomi_dn4, locals.var_denomi_dn5, locals.var_denomi_dn6, locals.var_denomi_dn7, locals.var_denomi_dn8, locals.var_denomi_dn9, locals.var_denomi_dn10, locals.var_denomi_dn11, locals.var_denomi_dn12,)
    }
};
        locals.var_denomi = assign31350_e25395;
        locals.var_denomi_dn3 = assign31350_e25395_d_n3;
        locals.var_denomi_dn4 = assign31350_e25395_d_n4;
        locals.var_denomi_dn5 = assign31350_e25395_d_n5;
        locals.var_denomi_dn6 = assign31350_e25395_d_n6;
        locals.var_denomi_dn7 = assign31350_e25395_d_n7;
        locals.var_denomi_dn8 = assign31350_e25395_d_n8;
        locals.var_denomi_dn9 = assign31350_e25395_d_n9;
        locals.var_denomi_dn10 = assign31350_e25395_d_n10;
        locals.var_denomi_dn11 = assign31350_e25395_d_n11;
        locals.var_denomi_dn12 = assign31350_e25395_d_n12;

        let (assign31360_e25406, assign31360_e25406_d_n3, assign31360_e25406_d_n4, assign31360_e25406_d_n5, assign31360_e25406_d_n6, assign31360_e25406_d_n7, assign31360_e25406_d_n8, assign31360_e25406_d_n9, assign31360_e25406_d_n10, assign31360_e25406_d_n11, assign31360_e25406_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1739 != 0.0)) {
        let assign31360_e25404: f64 = (0.5 * locals.var_pparam_b4soisqrtphi);
        (assign31360_e25404, (0.5 * locals.var_pparam_b4soisqrtphi_dn3), (0.5 * locals.var_pparam_b4soisqrtphi_dn4), (0.5 * locals.var_pparam_b4soisqrtphi_dn5), (0.5 * locals.var_pparam_b4soisqrtphi_dn6), (0.5 * locals.var_pparam_b4soisqrtphi_dn7), (0.5 * locals.var_pparam_b4soisqrtphi_dn8), (0.5 * locals.var_pparam_b4soisqrtphi_dn9), (0.5 * locals.var_pparam_b4soisqrtphi_dn10), (0.5 * locals.var_pparam_b4soisqrtphi_dn11), (0.5 * locals.var_pparam_b4soisqrtphi_dn12),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign31360_e25406;
        locals.var_t0__blk1144_dn3 = assign31360_e25406_d_n3;
        locals.var_t0__blk1144_dn4 = assign31360_e25406_d_n4;
        locals.var_t0__blk1144_dn5 = assign31360_e25406_d_n5;
        locals.var_t0__blk1144_dn6 = assign31360_e25406_d_n6;
        locals.var_t0__blk1144_dn7 = assign31360_e25406_d_n7;
        locals.var_t0__blk1144_dn8 = assign31360_e25406_d_n8;
        locals.var_t0__blk1144_dn9 = assign31360_e25406_d_n9;
        locals.var_t0__blk1144_dn10 = assign31360_e25406_d_n10;
        locals.var_t0__blk1144_dn11 = assign31360_e25406_d_n11;
        locals.var_t0__blk1144_dn12 = assign31360_e25406_d_n12;

        let (assign31370_e25422, assign31370_e25422_d_n3, assign31370_e25422_d_n4, assign31370_e25422_d_n5, assign31370_e25422_d_n6, assign31370_e25422_d_n7, assign31370_e25422_d_n8, assign31370_e25422_d_n9, assign31370_e25422_d_n10, assign31370_e25422_d_n11, assign31370_e25422_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1739 == 0.0)) {
        let assign31370_e25416: f64 = (locals.var_pparam_b4soimoin * locals.var_vtm);
        let assign31370_e25418: f64 = (assign31370_e25416 * locals.var_pparam_b4soik1ox);
        let assign31370_e25420: f64 = (assign31370_e25418 * locals.var_pparam_b4soik1ox);
        (assign31370_e25420, (((((locals.var_pparam_b4soimoin_dn3 * locals.var_vtm) * locals.var_pparam_b4soik1ox) + (assign31370_e25416 * locals.var_pparam_b4soik1ox_dn3)) * locals.var_pparam_b4soik1ox) + (assign31370_e25418 * locals.var_pparam_b4soik1ox_dn3)), (((((locals.var_pparam_b4soimoin_dn4 * locals.var_vtm) * locals.var_pparam_b4soik1ox) + (assign31370_e25416 * locals.var_pparam_b4soik1ox_dn4)) * locals.var_pparam_b4soik1ox) + (assign31370_e25418 * locals.var_pparam_b4soik1ox_dn4)), (((((locals.var_pparam_b4soimoin_dn5 * locals.var_vtm) * locals.var_pparam_b4soik1ox) + (assign31370_e25416 * locals.var_pparam_b4soik1ox_dn5)) * locals.var_pparam_b4soik1ox) + (assign31370_e25418 * locals.var_pparam_b4soik1ox_dn5)), ((((((locals.var_pparam_b4soimoin_dn6 * locals.var_vtm) + (locals.var_pparam_b4soimoin * locals.var_vtm_dn6)) * locals.var_pparam_b4soik1ox) + (assign31370_e25416 * locals.var_pparam_b4soik1ox_dn6)) * locals.var_pparam_b4soik1ox) + (assign31370_e25418 * locals.var_pparam_b4soik1ox_dn6)), (((((locals.var_pparam_b4soimoin_dn7 * locals.var_vtm) * locals.var_pparam_b4soik1ox) + (assign31370_e25416 * locals.var_pparam_b4soik1ox_dn7)) * locals.var_pparam_b4soik1ox) + (assign31370_e25418 * locals.var_pparam_b4soik1ox_dn7)), (((((locals.var_pparam_b4soimoin_dn8 * locals.var_vtm) * locals.var_pparam_b4soik1ox) + (assign31370_e25416 * locals.var_pparam_b4soik1ox_dn8)) * locals.var_pparam_b4soik1ox) + (assign31370_e25418 * locals.var_pparam_b4soik1ox_dn8)), (((((locals.var_pparam_b4soimoin_dn9 * locals.var_vtm) * locals.var_pparam_b4soik1ox) + (assign31370_e25416 * locals.var_pparam_b4soik1ox_dn9)) * locals.var_pparam_b4soik1ox) + (assign31370_e25418 * locals.var_pparam_b4soik1ox_dn9)), (((((locals.var_pparam_b4soimoin_dn10 * locals.var_vtm) * locals.var_pparam_b4soik1ox) + (assign31370_e25416 * locals.var_pparam_b4soik1ox_dn10)) * locals.var_pparam_b4soik1ox) + (assign31370_e25418 * locals.var_pparam_b4soik1ox_dn10)), (((((locals.var_pparam_b4soimoin_dn11 * locals.var_vtm) * locals.var_pparam_b4soik1ox) + (assign31370_e25416 * locals.var_pparam_b4soik1ox_dn11)) * locals.var_pparam_b4soik1ox) + (assign31370_e25418 * locals.var_pparam_b4soik1ox_dn11)), (((((locals.var_pparam_b4soimoin_dn12 * locals.var_vtm) * locals.var_pparam_b4soik1ox) + (assign31370_e25416 * locals.var_pparam_b4soik1ox_dn12)) * locals.var_pparam_b4soik1ox) + (assign31370_e25418 * locals.var_pparam_b4soik1ox_dn12)),)
    } else {
        (locals.var_denomi, locals.var_denomi_dn3, locals.var_denomi_dn4, locals.var_denomi_dn5, locals.var_denomi_dn6, locals.var_denomi_dn7, locals.var_denomi_dn8, locals.var_denomi_dn9, locals.var_denomi_dn10, locals.var_denomi_dn11, locals.var_denomi_dn12,)
    }
};
        locals.var_denomi = assign31370_e25422;
        locals.var_denomi_dn3 = assign31370_e25422_d_n3;
        locals.var_denomi_dn4 = assign31370_e25422_d_n4;
        locals.var_denomi_dn5 = assign31370_e25422_d_n5;
        locals.var_denomi_dn6 = assign31370_e25422_d_n6;
        locals.var_denomi_dn7 = assign31370_e25422_d_n7;
        locals.var_denomi_dn8 = assign31370_e25422_d_n8;
        locals.var_denomi_dn9 = assign31370_e25422_d_n9;
        locals.var_denomi_dn10 = assign31370_e25422_d_n10;
        locals.var_denomi_dn11 = assign31370_e25422_d_n11;
        locals.var_denomi_dn12 = assign31370_e25422_d_n12;

        let (assign31380_e25434, assign31380_e25434_d_n3, assign31380_e25434_d_n4, assign31380_e25434_d_n5, assign31380_e25434_d_n6, assign31380_e25434_d_n7, assign31380_e25434_d_n8, assign31380_e25434_d_n9, assign31380_e25434_d_n10, assign31380_e25434_d_n11, assign31380_e25434_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1739 == 0.0)) {
        let assign31380_e25432: f64 = (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soisqrtphi);
        (assign31380_e25432, ((locals.var_pparam_b4soik1ox_dn3 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soisqrtphi_dn3)), ((locals.var_pparam_b4soik1ox_dn4 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soisqrtphi_dn4)), ((locals.var_pparam_b4soik1ox_dn5 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soisqrtphi_dn5)), ((locals.var_pparam_b4soik1ox_dn6 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soisqrtphi_dn6)), ((locals.var_pparam_b4soik1ox_dn7 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soisqrtphi_dn7)), ((locals.var_pparam_b4soik1ox_dn8 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soisqrtphi_dn8)), ((locals.var_pparam_b4soik1ox_dn9 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soisqrtphi_dn9)), ((locals.var_pparam_b4soik1ox_dn10 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soisqrtphi_dn10)), ((locals.var_pparam_b4soik1ox_dn11 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soisqrtphi_dn11)), ((locals.var_pparam_b4soik1ox_dn12 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1ox * locals.var_pparam_b4soisqrtphi_dn12)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign31380_e25434;
        locals.var_t0__blk1144_dn3 = assign31380_e25434_d_n3;
        locals.var_t0__blk1144_dn4 = assign31380_e25434_d_n4;
        locals.var_t0__blk1144_dn5 = assign31380_e25434_d_n5;
        locals.var_t0__blk1144_dn6 = assign31380_e25434_d_n6;
        locals.var_t0__blk1144_dn7 = assign31380_e25434_d_n7;
        locals.var_t0__blk1144_dn8 = assign31380_e25434_d_n8;
        locals.var_t0__blk1144_dn9 = assign31380_e25434_d_n9;
        locals.var_t0__blk1144_dn10 = assign31380_e25434_d_n10;
        locals.var_t0__blk1144_dn11 = assign31380_e25434_d_n11;
        locals.var_t0__blk1144_dn12 = assign31380_e25434_d_n12;

        let (assign31390_e25445, assign31390_e25445_d_n3, assign31390_e25445_d_n4, assign31390_e25445_d_n5, assign31390_e25445_d_n6, assign31390_e25445_d_n7, assign31390_e25445_d_n8, assign31390_e25445_d_n9, assign31390_e25445_d_n10, assign31390_e25445_d_n11, assign31390_e25445_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) {
        let assign31390_e25441: f64 = (2.0 * locals.var_t0__blk1144);
        let assign31390_e25443: f64 = (assign31390_e25441 + locals.var_vgsteff__blk1175);
        (assign31390_e25443, ((2.0 * locals.var_t0__blk1144_dn3) + locals.var_vgsteff__blk1175_dn3), ((2.0 * locals.var_t0__blk1144_dn4) + locals.var_vgsteff__blk1175_dn4), ((2.0 * locals.var_t0__blk1144_dn5) + locals.var_vgsteff__blk1175_dn5), ((2.0 * locals.var_t0__blk1144_dn6) + locals.var_vgsteff__blk1175_dn6), ((2.0 * locals.var_t0__blk1144_dn7) + locals.var_vgsteff__blk1175_dn7), ((2.0 * locals.var_t0__blk1144_dn8) + locals.var_vgsteff__blk1175_dn8), ((2.0 * locals.var_t0__blk1144_dn9) + locals.var_vgsteff__blk1175_dn9), ((2.0 * locals.var_t0__blk1144_dn10) + locals.var_vgsteff__blk1175_dn10), ((2.0 * locals.var_t0__blk1144_dn11) + locals.var_vgsteff__blk1175_dn11), ((2.0 * locals.var_t0__blk1144_dn12) + locals.var_vgsteff__blk1175_dn12),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign31390_e25445;
        locals.var_t1__blk1145_dn3 = assign31390_e25445_d_n3;
        locals.var_t1__blk1145_dn4 = assign31390_e25445_d_n4;
        locals.var_t1__blk1145_dn5 = assign31390_e25445_d_n5;
        locals.var_t1__blk1145_dn6 = assign31390_e25445_d_n6;
        locals.var_t1__blk1145_dn7 = assign31390_e25445_d_n7;
        locals.var_t1__blk1145_dn8 = assign31390_e25445_d_n8;
        locals.var_t1__blk1145_dn9 = assign31390_e25445_d_n9;
        locals.var_t1__blk1145_dn10 = assign31390_e25445_d_n10;
        locals.var_t1__blk1145_dn11 = assign31390_e25445_d_n11;
        locals.var_t1__blk1145_dn12 = assign31390_e25445_d_n12;

        let (assign31400_e25473, assign31400_e25473_d_n3, assign31400_e25473_d_n4, assign31400_e25473_d_n5, assign31400_e25473_d_n6, assign31400_e25473_d_n7, assign31400_e25473_d_n8, assign31400_e25473_d_n9, assign31400_e25473_d_n10, assign31400_e25473_d_n11, assign31400_e25473_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) {
        let assign31400_e25454: f64 = (locals.var_t1__blk1145 * locals.var_vgsteff__blk1175);
        let assign31400_e25456: f64 = (assign31400_e25454 / locals.var_denomi);
        let assign31400_e25457: f64 = (1.0 + assign31400_e25456);
        let (assign31400_e25470, assign31400_e25470_d_n3, assign31400_e25470_d_n4, assign31400_e25470_d_n5, assign31400_e25470_d_n6, assign31400_e25470_d_n7, assign31400_e25470_d_n8, assign31400_e25470_d_n9, assign31400_e25470_d_n10, assign31400_e25470_d_n11, assign31400_e25470_d_n12,) = {
            if (assign31400_e25457 > 1e-38) {
                let assign31400_e25463: f64 = (locals.var_t1__blk1145 * locals.var_vgsteff__blk1175);
                let assign31400_e25465: f64 = (assign31400_e25463 / locals.var_denomi);
                let assign31400_e25466: f64 = (1.0 + assign31400_e25465);
                let assign31400_e25467: f64 = (assign31400_e25466).ln();
                (assign31400_e25467, ((((((locals.var_t1__blk1145_dn3 * locals.var_vgsteff__blk1175) + (locals.var_t1__blk1145 * locals.var_vgsteff__blk1175_dn3)) * locals.var_denomi) - (assign31400_e25463 * locals.var_denomi_dn3)) / (locals.var_denomi * locals.var_denomi)) / assign31400_e25466), ((((((locals.var_t1__blk1145_dn4 * locals.var_vgsteff__blk1175) + (locals.var_t1__blk1145 * locals.var_vgsteff__blk1175_dn4)) * locals.var_denomi) - (assign31400_e25463 * locals.var_denomi_dn4)) / (locals.var_denomi * locals.var_denomi)) / assign31400_e25466), ((((((locals.var_t1__blk1145_dn5 * locals.var_vgsteff__blk1175) + (locals.var_t1__blk1145 * locals.var_vgsteff__blk1175_dn5)) * locals.var_denomi) - (assign31400_e25463 * locals.var_denomi_dn5)) / (locals.var_denomi * locals.var_denomi)) / assign31400_e25466), ((((((locals.var_t1__blk1145_dn6 * locals.var_vgsteff__blk1175) + (locals.var_t1__blk1145 * locals.var_vgsteff__blk1175_dn6)) * locals.var_denomi) - (assign31400_e25463 * locals.var_denomi_dn6)) / (locals.var_denomi * locals.var_denomi)) / assign31400_e25466), ((((((locals.var_t1__blk1145_dn7 * locals.var_vgsteff__blk1175) + (locals.var_t1__blk1145 * locals.var_vgsteff__blk1175_dn7)) * locals.var_denomi) - (assign31400_e25463 * locals.var_denomi_dn7)) / (locals.var_denomi * locals.var_denomi)) / assign31400_e25466), ((((((locals.var_t1__blk1145_dn8 * locals.var_vgsteff__blk1175) + (locals.var_t1__blk1145 * locals.var_vgsteff__blk1175_dn8)) * locals.var_denomi) - (assign31400_e25463 * locals.var_denomi_dn8)) / (locals.var_denomi * locals.var_denomi)) / assign31400_e25466), ((((((locals.var_t1__blk1145_dn9 * locals.var_vgsteff__blk1175) + (locals.var_t1__blk1145 * locals.var_vgsteff__blk1175_dn9)) * locals.var_denomi) - (assign31400_e25463 * locals.var_denomi_dn9)) / (locals.var_denomi * locals.var_denomi)) / assign31400_e25466), ((((((locals.var_t1__blk1145_dn10 * locals.var_vgsteff__blk1175) + (locals.var_t1__blk1145 * locals.var_vgsteff__blk1175_dn10)) * locals.var_denomi) - (assign31400_e25463 * locals.var_denomi_dn10)) / (locals.var_denomi * locals.var_denomi)) / assign31400_e25466), ((((((locals.var_t1__blk1145_dn11 * locals.var_vgsteff__blk1175) + (locals.var_t1__blk1145 * locals.var_vgsteff__blk1175_dn11)) * locals.var_denomi) - (assign31400_e25463 * locals.var_denomi_dn11)) / (locals.var_denomi * locals.var_denomi)) / assign31400_e25466), ((((((locals.var_t1__blk1145_dn12 * locals.var_vgsteff__blk1175) + (locals.var_t1__blk1145 * locals.var_vgsteff__blk1175_dn12)) * locals.var_denomi) - (assign31400_e25463 * locals.var_denomi_dn12)) / (locals.var_denomi * locals.var_denomi)) / assign31400_e25466),)
            } else {
                let assign31400_e25469: f64 = (-87.49823353377374);
                (assign31400_e25469, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign31400_e25471: f64 = (locals.var_vtm * assign31400_e25470);
        (assign31400_e25471, (locals.var_vtm * assign31400_e25470_d_n3), (locals.var_vtm * assign31400_e25470_d_n4), (locals.var_vtm * assign31400_e25470_d_n5), ((locals.var_vtm_dn6 * assign31400_e25470) + (locals.var_vtm * assign31400_e25470_d_n6)), (locals.var_vtm * assign31400_e25470_d_n7), (locals.var_vtm * assign31400_e25470_d_n8), (locals.var_vtm * assign31400_e25470_d_n9), (locals.var_vtm * assign31400_e25470_d_n10), (locals.var_vtm * assign31400_e25470_d_n11), (locals.var_vtm * assign31400_e25470_d_n12),)
    } else {
        (locals.var_deltaphi, locals.var_deltaphi_dn3, locals.var_deltaphi_dn4, locals.var_deltaphi_dn5, locals.var_deltaphi_dn6, locals.var_deltaphi_dn7, locals.var_deltaphi_dn8, locals.var_deltaphi_dn9, locals.var_deltaphi_dn10, locals.var_deltaphi_dn11, locals.var_deltaphi_dn12,)
    }
};
        locals.var_deltaphi = assign31400_e25473;
        locals.var_deltaphi_dn3 = assign31400_e25473_d_n3;
        locals.var_deltaphi_dn4 = assign31400_e25473_d_n4;
        locals.var_deltaphi_dn5 = assign31400_e25473_d_n5;
        locals.var_deltaphi_dn6 = assign31400_e25473_d_n6;
        locals.var_deltaphi_dn7 = assign31400_e25473_d_n7;
        locals.var_deltaphi_dn8 = assign31400_e25473_d_n8;
        locals.var_deltaphi_dn9 = assign31400_e25473_d_n9;
        locals.var_deltaphi_dn10 = assign31400_e25473_d_n10;
        locals.var_deltaphi_dn11 = assign31400_e25473_d_n11;
        locals.var_deltaphi_dn12 = assign31400_e25473_d_n12;

        let assign31410_e25476: f64 = if locals.var_b4soiagbcp2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1740 = assign31410_e25476;

        let (assign31420_e25489, assign31420_e25489_d_n3, assign31420_e25489_d_n4, assign31420_e25489_d_n5, assign31420_e25489_d_n6, assign31420_e25489_d_n7, assign31420_e25489_d_n8, assign31420_e25489_d_n9, assign31420_e25489_d_n10, assign31420_e25489_d_n11, assign31420_e25489_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1740 != 0.0)) {
        let assign31420_e25485: f64 = (2.0 * locals.var_t0__blk1144);
        let assign31420_e25487: f64 = (assign31420_e25485 + locals.var_vgsteff2);
        (assign31420_e25487, ((2.0 * locals.var_t0__blk1144_dn3) + locals.var_vgsteff2_dn3), ((2.0 * locals.var_t0__blk1144_dn4) + locals.var_vgsteff2_dn4), ((2.0 * locals.var_t0__blk1144_dn5) + locals.var_vgsteff2_dn5), ((2.0 * locals.var_t0__blk1144_dn6) + locals.var_vgsteff2_dn6), ((2.0 * locals.var_t0__blk1144_dn7) + locals.var_vgsteff2_dn7), ((2.0 * locals.var_t0__blk1144_dn8) + locals.var_vgsteff2_dn8), ((2.0 * locals.var_t0__blk1144_dn9) + locals.var_vgsteff2_dn9), ((2.0 * locals.var_t0__blk1144_dn10) + locals.var_vgsteff2_dn10), ((2.0 * locals.var_t0__blk1144_dn11) + locals.var_vgsteff2_dn11), ((2.0 * locals.var_t0__blk1144_dn12) + locals.var_vgsteff2_dn12),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign31420_e25489;
        locals.var_t1__blk1145_dn3 = assign31420_e25489_d_n3;
        locals.var_t1__blk1145_dn4 = assign31420_e25489_d_n4;
        locals.var_t1__blk1145_dn5 = assign31420_e25489_d_n5;
        locals.var_t1__blk1145_dn6 = assign31420_e25489_d_n6;
        locals.var_t1__blk1145_dn7 = assign31420_e25489_d_n7;
        locals.var_t1__blk1145_dn8 = assign31420_e25489_d_n8;
        locals.var_t1__blk1145_dn9 = assign31420_e25489_d_n9;
        locals.var_t1__blk1145_dn10 = assign31420_e25489_d_n10;
        locals.var_t1__blk1145_dn11 = assign31420_e25489_d_n11;
        locals.var_t1__blk1145_dn12 = assign31420_e25489_d_n12;

        let (assign31430_e25519, assign31430_e25519_d_n3, assign31430_e25519_d_n4, assign31430_e25519_d_n5, assign31430_e25519_d_n6, assign31430_e25519_d_n7, assign31430_e25519_d_n8, assign31430_e25519_d_n9, assign31430_e25519_d_n10, assign31430_e25519_d_n11, assign31430_e25519_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1740 != 0.0)) {
        let assign31430_e25500: f64 = (locals.var_t1__blk1145 * locals.var_vgsteff2);
        let assign31430_e25502: f64 = (assign31430_e25500 / locals.var_denomi);
        let assign31430_e25503: f64 = (1.0 + assign31430_e25502);
        let (assign31430_e25516, assign31430_e25516_d_n3, assign31430_e25516_d_n4, assign31430_e25516_d_n5, assign31430_e25516_d_n6, assign31430_e25516_d_n7, assign31430_e25516_d_n8, assign31430_e25516_d_n9, assign31430_e25516_d_n10, assign31430_e25516_d_n11, assign31430_e25516_d_n12,) = {
            if (assign31430_e25503 > 1e-38) {
                let assign31430_e25509: f64 = (locals.var_t1__blk1145 * locals.var_vgsteff2);
                let assign31430_e25511: f64 = (assign31430_e25509 / locals.var_denomi);
                let assign31430_e25512: f64 = (1.0 + assign31430_e25511);
                let assign31430_e25513: f64 = (assign31430_e25512).ln();
                (assign31430_e25513, ((((((locals.var_t1__blk1145_dn3 * locals.var_vgsteff2) + (locals.var_t1__blk1145 * locals.var_vgsteff2_dn3)) * locals.var_denomi) - (assign31430_e25509 * locals.var_denomi_dn3)) / (locals.var_denomi * locals.var_denomi)) / assign31430_e25512), ((((((locals.var_t1__blk1145_dn4 * locals.var_vgsteff2) + (locals.var_t1__blk1145 * locals.var_vgsteff2_dn4)) * locals.var_denomi) - (assign31430_e25509 * locals.var_denomi_dn4)) / (locals.var_denomi * locals.var_denomi)) / assign31430_e25512), ((((((locals.var_t1__blk1145_dn5 * locals.var_vgsteff2) + (locals.var_t1__blk1145 * locals.var_vgsteff2_dn5)) * locals.var_denomi) - (assign31430_e25509 * locals.var_denomi_dn5)) / (locals.var_denomi * locals.var_denomi)) / assign31430_e25512), ((((((locals.var_t1__blk1145_dn6 * locals.var_vgsteff2) + (locals.var_t1__blk1145 * locals.var_vgsteff2_dn6)) * locals.var_denomi) - (assign31430_e25509 * locals.var_denomi_dn6)) / (locals.var_denomi * locals.var_denomi)) / assign31430_e25512), ((((((locals.var_t1__blk1145_dn7 * locals.var_vgsteff2) + (locals.var_t1__blk1145 * locals.var_vgsteff2_dn7)) * locals.var_denomi) - (assign31430_e25509 * locals.var_denomi_dn7)) / (locals.var_denomi * locals.var_denomi)) / assign31430_e25512), ((((((locals.var_t1__blk1145_dn8 * locals.var_vgsteff2) + (locals.var_t1__blk1145 * locals.var_vgsteff2_dn8)) * locals.var_denomi) - (assign31430_e25509 * locals.var_denomi_dn8)) / (locals.var_denomi * locals.var_denomi)) / assign31430_e25512), ((((((locals.var_t1__blk1145_dn9 * locals.var_vgsteff2) + (locals.var_t1__blk1145 * locals.var_vgsteff2_dn9)) * locals.var_denomi) - (assign31430_e25509 * locals.var_denomi_dn9)) / (locals.var_denomi * locals.var_denomi)) / assign31430_e25512), ((((((locals.var_t1__blk1145_dn10 * locals.var_vgsteff2) + (locals.var_t1__blk1145 * locals.var_vgsteff2_dn10)) * locals.var_denomi) - (assign31430_e25509 * locals.var_denomi_dn10)) / (locals.var_denomi * locals.var_denomi)) / assign31430_e25512), ((((((locals.var_t1__blk1145_dn11 * locals.var_vgsteff2) + (locals.var_t1__blk1145 * locals.var_vgsteff2_dn11)) * locals.var_denomi) - (assign31430_e25509 * locals.var_denomi_dn11)) / (locals.var_denomi * locals.var_denomi)) / assign31430_e25512), ((((((locals.var_t1__blk1145_dn12 * locals.var_vgsteff2) + (locals.var_t1__blk1145 * locals.var_vgsteff2_dn12)) * locals.var_denomi) - (assign31430_e25509 * locals.var_denomi_dn12)) / (locals.var_denomi * locals.var_denomi)) / assign31430_e25512),)
            } else {
                let assign31430_e25515: f64 = (-87.49823353377374);
                (assign31430_e25515, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign31430_e25517: f64 = (locals.var_vtm * assign31430_e25516);
        (assign31430_e25517, (locals.var_vtm * assign31430_e25516_d_n3), (locals.var_vtm * assign31430_e25516_d_n4), (locals.var_vtm * assign31430_e25516_d_n5), ((locals.var_vtm_dn6 * assign31430_e25516) + (locals.var_vtm * assign31430_e25516_d_n6)), (locals.var_vtm * assign31430_e25516_d_n7), (locals.var_vtm * assign31430_e25516_d_n8), (locals.var_vtm * assign31430_e25516_d_n9), (locals.var_vtm * assign31430_e25516_d_n10), (locals.var_vtm * assign31430_e25516_d_n11), (locals.var_vtm * assign31430_e25516_d_n12),)
    } else {
        (locals.var_deltaphi2, locals.var_deltaphi2_dn3, locals.var_deltaphi2_dn4, locals.var_deltaphi2_dn5, locals.var_deltaphi2_dn6, locals.var_deltaphi2_dn7, locals.var_deltaphi2_dn8, locals.var_deltaphi2_dn9, locals.var_deltaphi2_dn10, locals.var_deltaphi2_dn11, locals.var_deltaphi2_dn12,)
    }
};
        locals.var_deltaphi2 = assign31430_e25519;
        locals.var_deltaphi2_dn3 = assign31430_e25519_d_n3;
        locals.var_deltaphi2_dn4 = assign31430_e25519_d_n4;
        locals.var_deltaphi2_dn5 = assign31430_e25519_d_n5;
        locals.var_deltaphi2_dn6 = assign31430_e25519_d_n6;
        locals.var_deltaphi2_dn7 = assign31430_e25519_d_n7;
        locals.var_deltaphi2_dn8 = assign31430_e25519_d_n8;
        locals.var_deltaphi2_dn9 = assign31430_e25519_d_n9;
        locals.var_deltaphi2_dn10 = assign31430_e25519_d_n10;
        locals.var_deltaphi2_dn11 = assign31430_e25519_d_n11;
        locals.var_deltaphi2_dn12 = assign31430_e25519_d_n12;

        let (assign31440_e25532, assign31440_e25532_d_n3, assign31440_e25532_d_n4, assign31440_e25532_d_n5, assign31440_e25532_d_n6, assign31440_e25532_d_n7, assign31440_e25532_d_n8, assign31440_e25532_d_n9, assign31440_e25532_d_n10, assign31440_e25532_d_n11, assign31440_e25532_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) {
        let assign31440_e25527: f64 = (locals.var_vth__blk1130 - locals.var_vfbzb);
        let assign31440_e25529: f64 = (assign31440_e25527 - locals.var_phi);
        let assign31440_e25530: f64 = (4.0 * assign31440_e25529);
        (assign31440_e25530, (4.0 * ((locals.var_vth__blk1130_dn3 - locals.var_vfbzb_dn3) - locals.var_phi_dn3)), (4.0 * ((locals.var_vth__blk1130_dn4 - locals.var_vfbzb_dn4) - locals.var_phi_dn4)), (4.0 * ((locals.var_vth__blk1130_dn5 - locals.var_vfbzb_dn5) - locals.var_phi_dn5)), (4.0 * ((locals.var_vth__blk1130_dn6 - locals.var_vfbzb_dn6) - locals.var_phi_dn6)), (4.0 * ((locals.var_vth__blk1130_dn7 - locals.var_vfbzb_dn7) - locals.var_phi_dn7)), (4.0 * ((locals.var_vth__blk1130_dn8 - locals.var_vfbzb_dn8) - locals.var_phi_dn8)), (4.0 * ((locals.var_vth__blk1130_dn9 - locals.var_vfbzb_dn9) - locals.var_phi_dn9)), (4.0 * ((locals.var_vth__blk1130_dn10 - locals.var_vfbzb_dn10) - locals.var_phi_dn10)), (4.0 * ((locals.var_vth__blk1130_dn11 - locals.var_vfbzb_dn11) - locals.var_phi_dn11)), (4.0 * ((locals.var_vth__blk1130_dn12 - locals.var_vfbzb_dn12) - locals.var_phi_dn12)),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign31440_e25532;
        locals.var_t3__blk1147_dn3 = assign31440_e25532_d_n3;
        locals.var_t3__blk1147_dn4 = assign31440_e25532_d_n4;
        locals.var_t3__blk1147_dn5 = assign31440_e25532_d_n5;
        locals.var_t3__blk1147_dn6 = assign31440_e25532_d_n6;
        locals.var_t3__blk1147_dn7 = assign31440_e25532_d_n7;
        locals.var_t3__blk1147_dn8 = assign31440_e25532_d_n8;
        locals.var_t3__blk1147_dn9 = assign31440_e25532_d_n9;
        locals.var_t3__blk1147_dn10 = assign31440_e25532_d_n10;
        locals.var_t3__blk1147_dn11 = assign31440_e25532_d_n11;
        locals.var_t3__blk1147_dn12 = assign31440_e25532_d_n12;

        let (assign31450_e25544, assign31450_e25544_d_n3, assign31450_e25544_d_n4, assign31450_e25544_d_n5, assign31450_e25544_d_n6, assign31450_e25544_d_n7, assign31450_e25544_d_n8, assign31450_e25544_d_n9, assign31450_e25544_d_n10, assign31450_e25544_d_n11, assign31450_e25544_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) {
        let assign31450_e25539: f64 = (locals.var_t3__blk1147 * locals.var_t3__blk1147);
        let assign31450_e25541: f64 = (assign31450_e25539 + 0.0001);
        let assign31450_e25542: f64 = (assign31450_e25541).sqrt();
        (assign31450_e25542, (((locals.var_t3__blk1147_dn3 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn3)) / (2.0 * assign31450_e25542)), (((locals.var_t3__blk1147_dn4 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn4)) / (2.0 * assign31450_e25542)), (((locals.var_t3__blk1147_dn5 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn5)) / (2.0 * assign31450_e25542)), (((locals.var_t3__blk1147_dn6 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn6)) / (2.0 * assign31450_e25542)), (((locals.var_t3__blk1147_dn7 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn7)) / (2.0 * assign31450_e25542)), (((locals.var_t3__blk1147_dn8 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn8)) / (2.0 * assign31450_e25542)), (((locals.var_t3__blk1147_dn9 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn9)) / (2.0 * assign31450_e25542)), (((locals.var_t3__blk1147_dn10 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn10)) / (2.0 * assign31450_e25542)), (((locals.var_t3__blk1147_dn11 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn11)) / (2.0 * assign31450_e25542)), (((locals.var_t3__blk1147_dn12 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn12)) / (2.0 * assign31450_e25542)),)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign31450_e25544;
        locals.var_t2__blk1146_dn3 = assign31450_e25544_d_n3;
        locals.var_t2__blk1146_dn4 = assign31450_e25544_d_n4;
        locals.var_t2__blk1146_dn5 = assign31450_e25544_d_n5;
        locals.var_t2__blk1146_dn6 = assign31450_e25544_d_n6;
        locals.var_t2__blk1146_dn7 = assign31450_e25544_d_n7;
        locals.var_t2__blk1146_dn8 = assign31450_e25544_d_n8;
        locals.var_t2__blk1146_dn9 = assign31450_e25544_d_n9;
        locals.var_t2__blk1146_dn10 = assign31450_e25544_d_n10;
        locals.var_t2__blk1146_dn11 = assign31450_e25544_d_n11;
        locals.var_t2__blk1146_dn12 = assign31450_e25544_d_n12;

        let (assign31460_e25555, assign31460_e25555_d_n3, assign31460_e25555_d_n4, assign31460_e25555_d_n5, assign31460_e25555_d_n6, assign31460_e25555_d_n7, assign31460_e25555_d_n8, assign31460_e25555_d_n9, assign31460_e25555_d_n10, assign31460_e25555_d_n11, assign31460_e25555_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) {
        let assign31460_e25552: f64 = (locals.var_t3__blk1147 + locals.var_t2__blk1146);
        let assign31460_e25553: f64 = (0.5 * assign31460_e25552);
        (assign31460_e25553, (0.5 * (locals.var_t3__blk1147_dn3 + locals.var_t2__blk1146_dn3)), (0.5 * (locals.var_t3__blk1147_dn4 + locals.var_t2__blk1146_dn4)), (0.5 * (locals.var_t3__blk1147_dn5 + locals.var_t2__blk1146_dn5)), (0.5 * (locals.var_t3__blk1147_dn6 + locals.var_t2__blk1146_dn6)), (0.5 * (locals.var_t3__blk1147_dn7 + locals.var_t2__blk1146_dn7)), (0.5 * (locals.var_t3__blk1147_dn8 + locals.var_t2__blk1146_dn8)), (0.5 * (locals.var_t3__blk1147_dn9 + locals.var_t2__blk1146_dn9)), (0.5 * (locals.var_t3__blk1147_dn10 + locals.var_t2__blk1146_dn10)), (0.5 * (locals.var_t3__blk1147_dn11 + locals.var_t2__blk1146_dn11)), (0.5 * (locals.var_t3__blk1147_dn12 + locals.var_t2__blk1146_dn12)),)
    } else {
        (locals.var_t4__blk1148, locals.var_t4__blk1148_dn3, locals.var_t4__blk1148_dn4, locals.var_t4__blk1148_dn5, locals.var_t4__blk1148_dn6, locals.var_t4__blk1148_dn7, locals.var_t4__blk1148_dn8, locals.var_t4__blk1148_dn9, locals.var_t4__blk1148_dn10, locals.var_t4__blk1148_dn11, locals.var_t4__blk1148_dn12,)
    }
};
        locals.var_t4__blk1148 = assign31460_e25555;
        locals.var_t4__blk1148_dn3 = assign31460_e25555_d_n3;
        locals.var_t4__blk1148_dn4 = assign31460_e25555_d_n4;
        locals.var_t4__blk1148_dn5 = assign31460_e25555_d_n5;
        locals.var_t4__blk1148_dn6 = assign31460_e25555_d_n6;
        locals.var_t4__blk1148_dn7 = assign31460_e25555_d_n7;
        locals.var_t4__blk1148_dn8 = assign31460_e25555_d_n8;
        locals.var_t4__blk1148_dn9 = assign31460_e25555_d_n9;
        locals.var_t4__blk1148_dn10 = assign31460_e25555_d_n10;
        locals.var_t4__blk1148_dn11 = assign31460_e25555_d_n11;
        locals.var_t4__blk1148_dn12 = assign31460_e25555_d_n12;

        let (assign31470_e25564, assign31470_e25564_d_n3, assign31470_e25564_d_n4, assign31470_e25564_d_n5, assign31470_e25564_d_n6, assign31470_e25564_d_n7, assign31470_e25564_d_n8, assign31470_e25564_d_n9, assign31470_e25564_d_n10, assign31470_e25564_d_n11, assign31470_e25564_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) {
        let assign31470_e25562: f64 = (locals.var_tox + locals.var_tox);
        (assign31470_e25562, (locals.var_tox_dn3 + locals.var_tox_dn3), (locals.var_tox_dn4 + locals.var_tox_dn4), (locals.var_tox_dn5 + locals.var_tox_dn5), (locals.var_tox_dn6 + locals.var_tox_dn6), (locals.var_tox_dn7 + locals.var_tox_dn7), (locals.var_tox_dn8 + locals.var_tox_dn8), (locals.var_tox_dn9 + locals.var_tox_dn9), (locals.var_tox_dn10 + locals.var_tox_dn10), (locals.var_tox_dn11 + locals.var_tox_dn11), (locals.var_tox_dn12 + locals.var_tox_dn12),)
    } else {
        (locals.var_tox, locals.var_tox_dn3, locals.var_tox_dn4, locals.var_tox_dn5, locals.var_tox_dn6, locals.var_tox_dn7, locals.var_tox_dn8, locals.var_tox_dn9, locals.var_tox_dn10, locals.var_tox_dn11, locals.var_tox_dn12,)
    }
};
        locals.var_tox = assign31470_e25564;
        locals.var_tox_dn3 = assign31470_e25564_d_n3;
        locals.var_tox_dn4 = assign31470_e25564_d_n4;
        locals.var_tox_dn5 = assign31470_e25564_d_n5;
        locals.var_tox_dn6 = assign31470_e25564_d_n6;
        locals.var_tox_dn7 = assign31470_e25564_d_n7;
        locals.var_tox_dn8 = assign31470_e25564_d_n8;
        locals.var_tox_dn9 = assign31470_e25564_d_n9;
        locals.var_tox_dn10 = assign31470_e25564_d_n10;
        locals.var_tox_dn11 = assign31470_e25564_d_n11;
        locals.var_tox_dn12 = assign31470_e25564_d_n12;

        let (assign31480_e25575, assign31480_e25575_d_n3, assign31480_e25575_d_n4, assign31480_e25575_d_n5, assign31480_e25575_d_n6, assign31480_e25575_d_n7, assign31480_e25575_d_n8, assign31480_e25575_d_n9, assign31480_e25575_d_n10, assign31480_e25575_d_n11, assign31480_e25575_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) {
        let assign31480_e25571: f64 = (locals.var_vgsteff__blk1175 + locals.var_t4__blk1148);
        let assign31480_e25573: f64 = (assign31480_e25571 / locals.var_tox);
        (assign31480_e25573, ((((locals.var_vgsteff__blk1175_dn3 + locals.var_t4__blk1148_dn3) * locals.var_tox) - (assign31480_e25571 * locals.var_tox_dn3)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff__blk1175_dn4 + locals.var_t4__blk1148_dn4) * locals.var_tox) - (assign31480_e25571 * locals.var_tox_dn4)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff__blk1175_dn5 + locals.var_t4__blk1148_dn5) * locals.var_tox) - (assign31480_e25571 * locals.var_tox_dn5)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff__blk1175_dn6 + locals.var_t4__blk1148_dn6) * locals.var_tox) - (assign31480_e25571 * locals.var_tox_dn6)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff__blk1175_dn7 + locals.var_t4__blk1148_dn7) * locals.var_tox) - (assign31480_e25571 * locals.var_tox_dn7)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff__blk1175_dn8 + locals.var_t4__blk1148_dn8) * locals.var_tox) - (assign31480_e25571 * locals.var_tox_dn8)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff__blk1175_dn9 + locals.var_t4__blk1148_dn9) * locals.var_tox) - (assign31480_e25571 * locals.var_tox_dn9)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff__blk1175_dn10 + locals.var_t4__blk1148_dn10) * locals.var_tox) - (assign31480_e25571 * locals.var_tox_dn10)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff__blk1175_dn11 + locals.var_t4__blk1148_dn11) * locals.var_tox) - (assign31480_e25571 * locals.var_tox_dn11)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff__blk1175_dn12 + locals.var_t4__blk1148_dn12) * locals.var_tox) - (assign31480_e25571 * locals.var_tox_dn12)) / (locals.var_tox * locals.var_tox)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign31480_e25575;
        locals.var_t0__blk1144_dn3 = assign31480_e25575_d_n3;
        locals.var_t0__blk1144_dn4 = assign31480_e25575_d_n4;
        locals.var_t0__blk1144_dn5 = assign31480_e25575_d_n5;
        locals.var_t0__blk1144_dn6 = assign31480_e25575_d_n6;
        locals.var_t0__blk1144_dn7 = assign31480_e25575_d_n7;
        locals.var_t0__blk1144_dn8 = assign31480_e25575_d_n8;
        locals.var_t0__blk1144_dn9 = assign31480_e25575_d_n9;
        locals.var_t0__blk1144_dn10 = assign31480_e25575_d_n10;
        locals.var_t0__blk1144_dn11 = assign31480_e25575_d_n11;
        locals.var_t0__blk1144_dn12 = assign31480_e25575_d_n12;

        let (assign31490_e25594, assign31490_e25594_d_n3, assign31490_e25594_d_n4, assign31490_e25594_d_n5, assign31490_e25594_d_n6, assign31490_e25594_d_n7, assign31490_e25594_d_n8, assign31490_e25594_d_n9, assign31490_e25594_d_n10, assign31490_e25594_d_n11, assign31490_e25594_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) {
        let assign31490_e25582: f64 = (locals.var_b4soibdos * 0.7);
        let (assign31490_e25590, assign31490_e25590_d_n3, assign31490_e25590_d_n4, assign31490_e25590_d_n5, assign31490_e25590_d_n6, assign31490_e25590_d_n7, assign31490_e25590_d_n8, assign31490_e25590_d_n9, assign31490_e25590_d_n10, assign31490_e25590_d_n11, assign31490_e25590_d_n12,) = {
            if (locals.var_t0__blk1144 > 1e-38) {
                let assign31490_e25587: f64 = (locals.var_t0__blk1144).ln();
                (assign31490_e25587, (locals.var_t0__blk1144_dn3 / locals.var_t0__blk1144), (locals.var_t0__blk1144_dn4 / locals.var_t0__blk1144), (locals.var_t0__blk1144_dn5 / locals.var_t0__blk1144), (locals.var_t0__blk1144_dn6 / locals.var_t0__blk1144), (locals.var_t0__blk1144_dn7 / locals.var_t0__blk1144), (locals.var_t0__blk1144_dn8 / locals.var_t0__blk1144), (locals.var_t0__blk1144_dn9 / locals.var_t0__blk1144), (locals.var_t0__blk1144_dn10 / locals.var_t0__blk1144), (locals.var_t0__blk1144_dn11 / locals.var_t0__blk1144), (locals.var_t0__blk1144_dn12 / locals.var_t0__blk1144),)
            } else {
                let assign31490_e25589: f64 = (-87.49823353377374);
                (assign31490_e25589, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign31490_e25591: f64 = (assign31490_e25582 * assign31490_e25590);
        let assign31490_e25592: f64 = (assign31490_e25591).exp();
        (assign31490_e25592, (assign31490_e25592 * (assign31490_e25582 * assign31490_e25590_d_n3)), (assign31490_e25592 * (assign31490_e25582 * assign31490_e25590_d_n4)), (assign31490_e25592 * (assign31490_e25582 * assign31490_e25590_d_n5)), (assign31490_e25592 * (assign31490_e25582 * assign31490_e25590_d_n6)), (assign31490_e25592 * (assign31490_e25582 * assign31490_e25590_d_n7)), (assign31490_e25592 * (assign31490_e25582 * assign31490_e25590_d_n8)), (assign31490_e25592 * (assign31490_e25582 * assign31490_e25590_d_n9)), (assign31490_e25592 * (assign31490_e25582 * assign31490_e25590_d_n10)), (assign31490_e25592 * (assign31490_e25582 * assign31490_e25590_d_n11)), (assign31490_e25592 * (assign31490_e25582 * assign31490_e25590_d_n12)),)
    } else {
        (locals.var_tmp__blk1159, locals.var_tmp__blk1159_dn3, locals.var_tmp__blk1159_dn4, locals.var_tmp__blk1159_dn5, locals.var_tmp__blk1159_dn6, locals.var_tmp__blk1159_dn7, locals.var_tmp__blk1159_dn8, locals.var_tmp__blk1159_dn9, locals.var_tmp__blk1159_dn10, locals.var_tmp__blk1159_dn11, locals.var_tmp__blk1159_dn12,)
    }
};
        locals.var_tmp__blk1159 = assign31490_e25594;
        locals.var_tmp__blk1159_dn3 = assign31490_e25594_d_n3;
        locals.var_tmp__blk1159_dn4 = assign31490_e25594_d_n4;
        locals.var_tmp__blk1159_dn5 = assign31490_e25594_d_n5;
        locals.var_tmp__blk1159_dn6 = assign31490_e25594_d_n6;
        locals.var_tmp__blk1159_dn7 = assign31490_e25594_d_n7;
        locals.var_tmp__blk1159_dn8 = assign31490_e25594_d_n8;
        locals.var_tmp__blk1159_dn9 = assign31490_e25594_d_n9;
        locals.var_tmp__blk1159_dn10 = assign31490_e25594_d_n10;
        locals.var_tmp__blk1159_dn11 = assign31490_e25594_d_n11;
        locals.var_tmp__blk1159_dn12 = assign31490_e25594_d_n12;

    }

    pub(super) fn stamp_transient_block_86(
        locals: &mut StampLocals,
    ) {
        let (assign31500_e25603, assign31500_e25603_d_n3, assign31500_e25603_d_n4, assign31500_e25603_d_n5, assign31500_e25603_d_n6, assign31500_e25603_d_n7, assign31500_e25603_d_n8, assign31500_e25603_d_n9, assign31500_e25603_d_n10, assign31500_e25603_d_n11, assign31500_e25603_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) {
        let assign31500_e25601: f64 = (1.0 + locals.var_tmp__blk1159);
        (assign31500_e25601, locals.var_tmp__blk1159_dn3, locals.var_tmp__blk1159_dn4, locals.var_tmp__blk1159_dn5, locals.var_tmp__blk1159_dn6, locals.var_tmp__blk1159_dn7, locals.var_tmp__blk1159_dn8, locals.var_tmp__blk1159_dn9, locals.var_tmp__blk1159_dn10, locals.var_tmp__blk1159_dn11, locals.var_tmp__blk1159_dn12,)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign31500_e25603;
        locals.var_t1__blk1145_dn3 = assign31500_e25603_d_n3;
        locals.var_t1__blk1145_dn4 = assign31500_e25603_d_n4;
        locals.var_t1__blk1145_dn5 = assign31500_e25603_d_n5;
        locals.var_t1__blk1145_dn6 = assign31500_e25603_d_n6;
        locals.var_t1__blk1145_dn7 = assign31500_e25603_d_n7;
        locals.var_t1__blk1145_dn8 = assign31500_e25603_d_n8;
        locals.var_t1__blk1145_dn9 = assign31500_e25603_d_n9;
        locals.var_t1__blk1145_dn10 = assign31500_e25603_d_n10;
        locals.var_t1__blk1145_dn11 = assign31500_e25603_d_n11;
        locals.var_t1__blk1145_dn12 = assign31500_e25603_d_n12;

        let (assign31510_e25614, assign31510_e25614_d_n3, assign31510_e25614_d_n4, assign31510_e25614_d_n5, assign31510_e25614_d_n6, assign31510_e25614_d_n7, assign31510_e25614_d_n8, assign31510_e25614_d_n9, assign31510_e25614_d_n10, assign31510_e25614_d_n11, assign31510_e25614_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) {
        let assign31510_e25610: f64 = (locals.var_b4soiados * 1.9e-9);
        let assign31510_e25612: f64 = (assign31510_e25610 / locals.var_t1__blk1145);
        (assign31510_e25612, (-((assign31510_e25610 * locals.var_t1__blk1145_dn3) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (-((assign31510_e25610 * locals.var_t1__blk1145_dn4) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (-((assign31510_e25610 * locals.var_t1__blk1145_dn5) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (-((assign31510_e25610 * locals.var_t1__blk1145_dn6) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (-((assign31510_e25610 * locals.var_t1__blk1145_dn7) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (-((assign31510_e25610 * locals.var_t1__blk1145_dn8) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (-((assign31510_e25610 * locals.var_t1__blk1145_dn9) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (-((assign31510_e25610 * locals.var_t1__blk1145_dn10) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (-((assign31510_e25610 * locals.var_t1__blk1145_dn11) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (-((assign31510_e25610 * locals.var_t1__blk1145_dn12) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))),)
    } else {
        (locals.var_tcen__blk1299, locals.var_tcen__blk1299_dn3, locals.var_tcen__blk1299_dn4, locals.var_tcen__blk1299_dn5, locals.var_tcen__blk1299_dn6, locals.var_tcen__blk1299_dn7, locals.var_tcen__blk1299_dn8, locals.var_tcen__blk1299_dn9, locals.var_tcen__blk1299_dn10, locals.var_tcen__blk1299_dn11, locals.var_tcen__blk1299_dn12,)
    }
};
        locals.var_tcen__blk1299 = assign31510_e25614;
        locals.var_tcen__blk1299_dn3 = assign31510_e25614_d_n3;
        locals.var_tcen__blk1299_dn4 = assign31510_e25614_d_n4;
        locals.var_tcen__blk1299_dn5 = assign31510_e25614_d_n5;
        locals.var_tcen__blk1299_dn6 = assign31510_e25614_d_n6;
        locals.var_tcen__blk1299_dn7 = assign31510_e25614_d_n7;
        locals.var_tcen__blk1299_dn8 = assign31510_e25614_d_n8;
        locals.var_tcen__blk1299_dn9 = assign31510_e25614_d_n9;
        locals.var_tcen__blk1299_dn10 = assign31510_e25614_d_n10;
        locals.var_tcen__blk1299_dn11 = assign31510_e25614_d_n11;
        locals.var_tcen__blk1299_dn12 = assign31510_e25614_d_n12;

        let (assign31520_e25623, assign31520_e25623_d_n3, assign31520_e25623_d_n4, assign31520_e25623_d_n5, assign31520_e25623_d_n6, assign31520_e25623_d_n7, assign31520_e25623_d_n8, assign31520_e25623_d_n9, assign31520_e25623_d_n10, assign31520_e25623_d_n11, assign31520_e25623_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) {
        let assign31520_e25621: f64 = (locals.var_epssub / locals.var_tcen__blk1299);
        (assign31520_e25621, (-((locals.var_epssub * locals.var_tcen__blk1299_dn3) / (locals.var_tcen__blk1299 * locals.var_tcen__blk1299))), (-((locals.var_epssub * locals.var_tcen__blk1299_dn4) / (locals.var_tcen__blk1299 * locals.var_tcen__blk1299))), (-((locals.var_epssub * locals.var_tcen__blk1299_dn5) / (locals.var_tcen__blk1299 * locals.var_tcen__blk1299))), (-((locals.var_epssub * locals.var_tcen__blk1299_dn6) / (locals.var_tcen__blk1299 * locals.var_tcen__blk1299))), (-((locals.var_epssub * locals.var_tcen__blk1299_dn7) / (locals.var_tcen__blk1299 * locals.var_tcen__blk1299))), (-((locals.var_epssub * locals.var_tcen__blk1299_dn8) / (locals.var_tcen__blk1299 * locals.var_tcen__blk1299))), (-((locals.var_epssub * locals.var_tcen__blk1299_dn9) / (locals.var_tcen__blk1299 * locals.var_tcen__blk1299))), (-((locals.var_epssub * locals.var_tcen__blk1299_dn10) / (locals.var_tcen__blk1299 * locals.var_tcen__blk1299))), (-((locals.var_epssub * locals.var_tcen__blk1299_dn11) / (locals.var_tcen__blk1299 * locals.var_tcen__blk1299))), (-((locals.var_epssub * locals.var_tcen__blk1299_dn12) / (locals.var_tcen__blk1299 * locals.var_tcen__blk1299))),)
    } else {
        (locals.var_ccen, locals.var_ccen_dn3, locals.var_ccen_dn4, locals.var_ccen_dn5, locals.var_ccen_dn6, locals.var_ccen_dn7, locals.var_ccen_dn8, locals.var_ccen_dn9, locals.var_ccen_dn10, locals.var_ccen_dn11, locals.var_ccen_dn12,)
    }
};
        locals.var_ccen = assign31520_e25623;
        locals.var_ccen_dn3 = assign31520_e25623_d_n3;
        locals.var_ccen_dn4 = assign31520_e25623_d_n4;
        locals.var_ccen_dn5 = assign31520_e25623_d_n5;
        locals.var_ccen_dn6 = assign31520_e25623_d_n6;
        locals.var_ccen_dn7 = assign31520_e25623_d_n7;
        locals.var_ccen_dn8 = assign31520_e25623_d_n8;
        locals.var_ccen_dn9 = assign31520_e25623_d_n9;
        locals.var_ccen_dn10 = assign31520_e25623_d_n10;
        locals.var_ccen_dn11 = assign31520_e25623_d_n11;
        locals.var_ccen_dn12 = assign31520_e25623_d_n12;

        let (assign31530_e25634, assign31530_e25634_d_n3, assign31530_e25634_d_n4, assign31530_e25634_d_n5, assign31530_e25634_d_n6, assign31530_e25634_d_n7, assign31530_e25634_d_n8, assign31530_e25634_d_n9, assign31530_e25634_d_n10, assign31530_e25634_d_n11, assign31530_e25634_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) {
        let assign31530_e25631: f64 = (locals.var_cox + locals.var_ccen);
        let assign31530_e25632: f64 = (locals.var_cox / assign31530_e25631);
        (assign31530_e25632, (((locals.var_cox_dn3 * assign31530_e25631) - (locals.var_cox * (locals.var_cox_dn3 + locals.var_ccen_dn3))) / (assign31530_e25631 * assign31530_e25631)), (((locals.var_cox_dn4 * assign31530_e25631) - (locals.var_cox * (locals.var_cox_dn4 + locals.var_ccen_dn4))) / (assign31530_e25631 * assign31530_e25631)), (((locals.var_cox_dn5 * assign31530_e25631) - (locals.var_cox * (locals.var_cox_dn5 + locals.var_ccen_dn5))) / (assign31530_e25631 * assign31530_e25631)), (((locals.var_cox_dn6 * assign31530_e25631) - (locals.var_cox * (locals.var_cox_dn6 + locals.var_ccen_dn6))) / (assign31530_e25631 * assign31530_e25631)), (((locals.var_cox_dn7 * assign31530_e25631) - (locals.var_cox * (locals.var_cox_dn7 + locals.var_ccen_dn7))) / (assign31530_e25631 * assign31530_e25631)), (((locals.var_cox_dn8 * assign31530_e25631) - (locals.var_cox * (locals.var_cox_dn8 + locals.var_ccen_dn8))) / (assign31530_e25631 * assign31530_e25631)), (((locals.var_cox_dn9 * assign31530_e25631) - (locals.var_cox * (locals.var_cox_dn9 + locals.var_ccen_dn9))) / (assign31530_e25631 * assign31530_e25631)), (((locals.var_cox_dn10 * assign31530_e25631) - (locals.var_cox * (locals.var_cox_dn10 + locals.var_ccen_dn10))) / (assign31530_e25631 * assign31530_e25631)), (((locals.var_cox_dn11 * assign31530_e25631) - (locals.var_cox * (locals.var_cox_dn11 + locals.var_ccen_dn11))) / (assign31530_e25631 * assign31530_e25631)), (((locals.var_cox_dn12 * assign31530_e25631) - (locals.var_cox * (locals.var_cox_dn12 + locals.var_ccen_dn12))) / (assign31530_e25631 * assign31530_e25631)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign31530_e25634;
        locals.var_t0__blk1144_dn3 = assign31530_e25634_d_n3;
        locals.var_t0__blk1144_dn4 = assign31530_e25634_d_n4;
        locals.var_t0__blk1144_dn5 = assign31530_e25634_d_n5;
        locals.var_t0__blk1144_dn6 = assign31530_e25634_d_n6;
        locals.var_t0__blk1144_dn7 = assign31530_e25634_d_n7;
        locals.var_t0__blk1144_dn8 = assign31530_e25634_d_n8;
        locals.var_t0__blk1144_dn9 = assign31530_e25634_d_n9;
        locals.var_t0__blk1144_dn10 = assign31530_e25634_d_n10;
        locals.var_t0__blk1144_dn11 = assign31530_e25634_d_n11;
        locals.var_t0__blk1144_dn12 = assign31530_e25634_d_n12;

        let (assign31540_e25643, assign31540_e25643_d_n3, assign31540_e25643_d_n4, assign31540_e25643_d_n5, assign31540_e25643_d_n6, assign31540_e25643_d_n7, assign31540_e25643_d_n8, assign31540_e25643_d_n9, assign31540_e25643_d_n10, assign31540_e25643_d_n11, assign31540_e25643_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) {
        let assign31540_e25641: f64 = (locals.var_t0__blk1144 * locals.var_ccen);
        (assign31540_e25641, ((locals.var_t0__blk1144_dn3 * locals.var_ccen) + (locals.var_t0__blk1144 * locals.var_ccen_dn3)), ((locals.var_t0__blk1144_dn4 * locals.var_ccen) + (locals.var_t0__blk1144 * locals.var_ccen_dn4)), ((locals.var_t0__blk1144_dn5 * locals.var_ccen) + (locals.var_t0__blk1144 * locals.var_ccen_dn5)), ((locals.var_t0__blk1144_dn6 * locals.var_ccen) + (locals.var_t0__blk1144 * locals.var_ccen_dn6)), ((locals.var_t0__blk1144_dn7 * locals.var_ccen) + (locals.var_t0__blk1144 * locals.var_ccen_dn7)), ((locals.var_t0__blk1144_dn8 * locals.var_ccen) + (locals.var_t0__blk1144 * locals.var_ccen_dn8)), ((locals.var_t0__blk1144_dn9 * locals.var_ccen) + (locals.var_t0__blk1144 * locals.var_ccen_dn9)), ((locals.var_t0__blk1144_dn10 * locals.var_ccen) + (locals.var_t0__blk1144 * locals.var_ccen_dn10)), ((locals.var_t0__blk1144_dn11 * locals.var_ccen) + (locals.var_t0__blk1144 * locals.var_ccen_dn11)), ((locals.var_t0__blk1144_dn12 * locals.var_ccen) + (locals.var_t0__blk1144 * locals.var_ccen_dn12)),)
    } else {
        (locals.var_coxeff, locals.var_coxeff_dn3, locals.var_coxeff_dn4, locals.var_coxeff_dn5, locals.var_coxeff_dn6, locals.var_coxeff_dn7, locals.var_coxeff_dn8, locals.var_coxeff_dn9, locals.var_coxeff_dn10, locals.var_coxeff_dn11, locals.var_coxeff_dn12,)
    }
};
        locals.var_coxeff = assign31540_e25643;
        locals.var_coxeff_dn3 = assign31540_e25643_d_n3;
        locals.var_coxeff_dn4 = assign31540_e25643_d_n4;
        locals.var_coxeff_dn5 = assign31540_e25643_d_n5;
        locals.var_coxeff_dn6 = assign31540_e25643_d_n6;
        locals.var_coxeff_dn7 = assign31540_e25643_d_n7;
        locals.var_coxeff_dn8 = assign31540_e25643_d_n8;
        locals.var_coxeff_dn9 = assign31540_e25643_d_n9;
        locals.var_coxeff_dn10 = assign31540_e25643_d_n10;
        locals.var_coxeff_dn11 = assign31540_e25643_d_n11;
        locals.var_coxeff_dn12 = assign31540_e25643_d_n12;

        let (assign31550_e25654, assign31550_e25654_d_n3, assign31550_e25654_d_n4, assign31550_e25654_d_n5, assign31550_e25654_d_n6, assign31550_e25654_d_n7, assign31550_e25654_d_n8, assign31550_e25654_d_n9, assign31550_e25654_d_n10, assign31550_e25654_d_n11, assign31550_e25654_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) {
        let assign31550_e25650: f64 = (locals.var_coxwl * locals.var_coxeff);
        let assign31550_e25652: f64 = (assign31550_e25650 / locals.var_cox);
        (assign31550_e25652, (((((locals.var_coxwl_dn3 * locals.var_coxeff) + (locals.var_coxwl * locals.var_coxeff_dn3)) * locals.var_cox) - (assign31550_e25650 * locals.var_cox_dn3)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl_dn4 * locals.var_coxeff) + (locals.var_coxwl * locals.var_coxeff_dn4)) * locals.var_cox) - (assign31550_e25650 * locals.var_cox_dn4)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl_dn5 * locals.var_coxeff) + (locals.var_coxwl * locals.var_coxeff_dn5)) * locals.var_cox) - (assign31550_e25650 * locals.var_cox_dn5)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl_dn6 * locals.var_coxeff) + (locals.var_coxwl * locals.var_coxeff_dn6)) * locals.var_cox) - (assign31550_e25650 * locals.var_cox_dn6)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl_dn7 * locals.var_coxeff) + (locals.var_coxwl * locals.var_coxeff_dn7)) * locals.var_cox) - (assign31550_e25650 * locals.var_cox_dn7)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl_dn8 * locals.var_coxeff) + (locals.var_coxwl * locals.var_coxeff_dn8)) * locals.var_cox) - (assign31550_e25650 * locals.var_cox_dn8)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl_dn9 * locals.var_coxeff) + (locals.var_coxwl * locals.var_coxeff_dn9)) * locals.var_cox) - (assign31550_e25650 * locals.var_cox_dn9)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl_dn10 * locals.var_coxeff) + (locals.var_coxwl * locals.var_coxeff_dn10)) * locals.var_cox) - (assign31550_e25650 * locals.var_cox_dn10)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl_dn11 * locals.var_coxeff) + (locals.var_coxwl * locals.var_coxeff_dn11)) * locals.var_cox) - (assign31550_e25650 * locals.var_cox_dn11)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl_dn12 * locals.var_coxeff) + (locals.var_coxwl * locals.var_coxeff_dn12)) * locals.var_cox) - (assign31550_e25650 * locals.var_cox_dn12)) / (locals.var_cox * locals.var_cox)),)
    } else {
        (locals.var_coxwlcen, locals.var_coxwlcen_dn3, locals.var_coxwlcen_dn4, locals.var_coxwlcen_dn5, locals.var_coxwlcen_dn6, locals.var_coxwlcen_dn7, locals.var_coxwlcen_dn8, locals.var_coxwlcen_dn9, locals.var_coxwlcen_dn10, locals.var_coxwlcen_dn11, locals.var_coxwlcen_dn12,)
    }
};
        locals.var_coxwlcen = assign31550_e25654;
        locals.var_coxwlcen_dn3 = assign31550_e25654_d_n3;
        locals.var_coxwlcen_dn4 = assign31550_e25654_d_n4;
        locals.var_coxwlcen_dn5 = assign31550_e25654_d_n5;
        locals.var_coxwlcen_dn6 = assign31550_e25654_d_n6;
        locals.var_coxwlcen_dn7 = assign31550_e25654_d_n7;
        locals.var_coxwlcen_dn8 = assign31550_e25654_d_n8;
        locals.var_coxwlcen_dn9 = assign31550_e25654_d_n9;
        locals.var_coxwlcen_dn10 = assign31550_e25654_d_n10;
        locals.var_coxwlcen_dn11 = assign31550_e25654_d_n11;
        locals.var_coxwlcen_dn12 = assign31550_e25654_d_n12;

        let (assign31560_e25665, assign31560_e25665_d_n3, assign31560_e25665_d_n4, assign31560_e25665_d_n5, assign31560_e25665_d_n6, assign31560_e25665_d_n7, assign31560_e25665_d_n8, assign31560_e25665_d_n9, assign31560_e25665_d_n10, assign31560_e25665_d_n11, assign31560_e25665_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) {
        let assign31560_e25661: f64 = (locals.var_coxwlb * locals.var_coxeff);
        let assign31560_e25663: f64 = (assign31560_e25661 / locals.var_cox);
        (assign31560_e25663, (((((locals.var_coxwlb_dn3 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn3)) * locals.var_cox) - (assign31560_e25661 * locals.var_cox_dn3)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn4 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn4)) * locals.var_cox) - (assign31560_e25661 * locals.var_cox_dn4)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn5 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn5)) * locals.var_cox) - (assign31560_e25661 * locals.var_cox_dn5)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn6 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn6)) * locals.var_cox) - (assign31560_e25661 * locals.var_cox_dn6)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn7 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn7)) * locals.var_cox) - (assign31560_e25661 * locals.var_cox_dn7)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn8 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn8)) * locals.var_cox) - (assign31560_e25661 * locals.var_cox_dn8)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn9 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn9)) * locals.var_cox) - (assign31560_e25661 * locals.var_cox_dn9)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn10 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn10)) * locals.var_cox) - (assign31560_e25661 * locals.var_cox_dn10)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn11 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn11)) * locals.var_cox) - (assign31560_e25661 * locals.var_cox_dn11)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn12 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn12)) * locals.var_cox) - (assign31560_e25661 * locals.var_cox_dn12)) / (locals.var_cox * locals.var_cox)),)
    } else {
        (locals.var_coxwlcenb, locals.var_coxwlcenb_dn3, locals.var_coxwlcenb_dn4, locals.var_coxwlcenb_dn5, locals.var_coxwlcenb_dn6, locals.var_coxwlcenb_dn7, locals.var_coxwlcenb_dn8, locals.var_coxwlcenb_dn9, locals.var_coxwlcenb_dn10, locals.var_coxwlcenb_dn11, locals.var_coxwlcenb_dn12,)
    }
};
        locals.var_coxwlcenb = assign31560_e25665;
        locals.var_coxwlcenb_dn3 = assign31560_e25665_d_n3;
        locals.var_coxwlcenb_dn4 = assign31560_e25665_d_n4;
        locals.var_coxwlcenb_dn5 = assign31560_e25665_d_n5;
        locals.var_coxwlcenb_dn6 = assign31560_e25665_d_n6;
        locals.var_coxwlcenb_dn7 = assign31560_e25665_d_n7;
        locals.var_coxwlcenb_dn8 = assign31560_e25665_d_n8;
        locals.var_coxwlcenb_dn9 = assign31560_e25665_d_n9;
        locals.var_coxwlcenb_dn10 = assign31560_e25665_d_n10;
        locals.var_coxwlcenb_dn11 = assign31560_e25665_d_n11;
        locals.var_coxwlcenb_dn12 = assign31560_e25665_d_n12;

        let assign31570_e25676: f64 = if (((locals.var_b4soisoimod != 2.0) && (locals.var_b4soibodymod != 0.0)) && (locals.var_b4soiagbcp2 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1741 = assign31570_e25676;

        let (assign31580_e25693, assign31580_e25693_d_n3, assign31580_e25693_d_n4, assign31580_e25693_d_n5, assign31580_e25693_d_n6, assign31580_e25693_d_n7, assign31580_e25693_d_n8, assign31580_e25693_d_n9, assign31580_e25693_d_n10, assign31580_e25693_d_n11, assign31580_e25693_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1741 != 0.0)) {
        let assign31580_e25686: f64 = (locals.var_vth__blk1130 + locals.var_eggbcp2);
        let assign31580_e25688: f64 = (assign31580_e25686 - locals.var_vfbzb2);
        let assign31580_e25690: f64 = (assign31580_e25688 - locals.var_phi);
        let assign31580_e25691: f64 = (4.0 * assign31580_e25690);
        (assign31580_e25691, (4.0 * ((locals.var_vth__blk1130_dn3 - locals.var_vfbzb2_dn3) - locals.var_phi_dn3)), (4.0 * ((locals.var_vth__blk1130_dn4 - locals.var_vfbzb2_dn4) - locals.var_phi_dn4)), (4.0 * ((locals.var_vth__blk1130_dn5 - locals.var_vfbzb2_dn5) - locals.var_phi_dn5)), (4.0 * ((locals.var_vth__blk1130_dn6 - locals.var_vfbzb2_dn6) - locals.var_phi_dn6)), (4.0 * ((locals.var_vth__blk1130_dn7 - locals.var_vfbzb2_dn7) - locals.var_phi_dn7)), (4.0 * ((locals.var_vth__blk1130_dn8 - locals.var_vfbzb2_dn8) - locals.var_phi_dn8)), (4.0 * ((locals.var_vth__blk1130_dn9 - locals.var_vfbzb2_dn9) - locals.var_phi_dn9)), (4.0 * ((locals.var_vth__blk1130_dn10 - locals.var_vfbzb2_dn10) - locals.var_phi_dn10)), (4.0 * ((locals.var_vth__blk1130_dn11 - locals.var_vfbzb2_dn11) - locals.var_phi_dn11)), (4.0 * ((locals.var_vth__blk1130_dn12 - locals.var_vfbzb2_dn12) - locals.var_phi_dn12)),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign31580_e25693;
        locals.var_t3__blk1147_dn3 = assign31580_e25693_d_n3;
        locals.var_t3__blk1147_dn4 = assign31580_e25693_d_n4;
        locals.var_t3__blk1147_dn5 = assign31580_e25693_d_n5;
        locals.var_t3__blk1147_dn6 = assign31580_e25693_d_n6;
        locals.var_t3__blk1147_dn7 = assign31580_e25693_d_n7;
        locals.var_t3__blk1147_dn8 = assign31580_e25693_d_n8;
        locals.var_t3__blk1147_dn9 = assign31580_e25693_d_n9;
        locals.var_t3__blk1147_dn10 = assign31580_e25693_d_n10;
        locals.var_t3__blk1147_dn11 = assign31580_e25693_d_n11;
        locals.var_t3__blk1147_dn12 = assign31580_e25693_d_n12;

        let (assign31590_e25707, assign31590_e25707_d_n3, assign31590_e25707_d_n4, assign31590_e25707_d_n5, assign31590_e25707_d_n6, assign31590_e25707_d_n7, assign31590_e25707_d_n8, assign31590_e25707_d_n9, assign31590_e25707_d_n10, assign31590_e25707_d_n11, assign31590_e25707_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1741 != 0.0)) {
        let assign31590_e25702: f64 = (locals.var_t3__blk1147 * locals.var_t3__blk1147);
        let assign31590_e25704: f64 = (assign31590_e25702 + 0.0001);
        let assign31590_e25705: f64 = (assign31590_e25704).sqrt();
        (assign31590_e25705, (((locals.var_t3__blk1147_dn3 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn3)) / (2.0 * assign31590_e25705)), (((locals.var_t3__blk1147_dn4 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn4)) / (2.0 * assign31590_e25705)), (((locals.var_t3__blk1147_dn5 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn5)) / (2.0 * assign31590_e25705)), (((locals.var_t3__blk1147_dn6 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn6)) / (2.0 * assign31590_e25705)), (((locals.var_t3__blk1147_dn7 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn7)) / (2.0 * assign31590_e25705)), (((locals.var_t3__blk1147_dn8 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn8)) / (2.0 * assign31590_e25705)), (((locals.var_t3__blk1147_dn9 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn9)) / (2.0 * assign31590_e25705)), (((locals.var_t3__blk1147_dn10 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn10)) / (2.0 * assign31590_e25705)), (((locals.var_t3__blk1147_dn11 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn11)) / (2.0 * assign31590_e25705)), (((locals.var_t3__blk1147_dn12 * locals.var_t3__blk1147) + (locals.var_t3__blk1147 * locals.var_t3__blk1147_dn12)) / (2.0 * assign31590_e25705)),)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign31590_e25707;
        locals.var_t2__blk1146_dn3 = assign31590_e25707_d_n3;
        locals.var_t2__blk1146_dn4 = assign31590_e25707_d_n4;
        locals.var_t2__blk1146_dn5 = assign31590_e25707_d_n5;
        locals.var_t2__blk1146_dn6 = assign31590_e25707_d_n6;
        locals.var_t2__blk1146_dn7 = assign31590_e25707_d_n7;
        locals.var_t2__blk1146_dn8 = assign31590_e25707_d_n8;
        locals.var_t2__blk1146_dn9 = assign31590_e25707_d_n9;
        locals.var_t2__blk1146_dn10 = assign31590_e25707_d_n10;
        locals.var_t2__blk1146_dn11 = assign31590_e25707_d_n11;
        locals.var_t2__blk1146_dn12 = assign31590_e25707_d_n12;

        let (assign31600_e25720, assign31600_e25720_d_n3, assign31600_e25720_d_n4, assign31600_e25720_d_n5, assign31600_e25720_d_n6, assign31600_e25720_d_n7, assign31600_e25720_d_n8, assign31600_e25720_d_n9, assign31600_e25720_d_n10, assign31600_e25720_d_n11, assign31600_e25720_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1741 != 0.0)) {
        let assign31600_e25717: f64 = (locals.var_t3__blk1147 + locals.var_t2__blk1146);
        let assign31600_e25718: f64 = (0.5 * assign31600_e25717);
        (assign31600_e25718, (0.5 * (locals.var_t3__blk1147_dn3 + locals.var_t2__blk1146_dn3)), (0.5 * (locals.var_t3__blk1147_dn4 + locals.var_t2__blk1146_dn4)), (0.5 * (locals.var_t3__blk1147_dn5 + locals.var_t2__blk1146_dn5)), (0.5 * (locals.var_t3__blk1147_dn6 + locals.var_t2__blk1146_dn6)), (0.5 * (locals.var_t3__blk1147_dn7 + locals.var_t2__blk1146_dn7)), (0.5 * (locals.var_t3__blk1147_dn8 + locals.var_t2__blk1146_dn8)), (0.5 * (locals.var_t3__blk1147_dn9 + locals.var_t2__blk1146_dn9)), (0.5 * (locals.var_t3__blk1147_dn10 + locals.var_t2__blk1146_dn10)), (0.5 * (locals.var_t3__blk1147_dn11 + locals.var_t2__blk1146_dn11)), (0.5 * (locals.var_t3__blk1147_dn12 + locals.var_t2__blk1146_dn12)),)
    } else {
        (locals.var_t4__blk1148, locals.var_t4__blk1148_dn3, locals.var_t4__blk1148_dn4, locals.var_t4__blk1148_dn5, locals.var_t4__blk1148_dn6, locals.var_t4__blk1148_dn7, locals.var_t4__blk1148_dn8, locals.var_t4__blk1148_dn9, locals.var_t4__blk1148_dn10, locals.var_t4__blk1148_dn11, locals.var_t4__blk1148_dn12,)
    }
};
        locals.var_t4__blk1148 = assign31600_e25720;
        locals.var_t4__blk1148_dn3 = assign31600_e25720_d_n3;
        locals.var_t4__blk1148_dn4 = assign31600_e25720_d_n4;
        locals.var_t4__blk1148_dn5 = assign31600_e25720_d_n5;
        locals.var_t4__blk1148_dn6 = assign31600_e25720_d_n6;
        locals.var_t4__blk1148_dn7 = assign31600_e25720_d_n7;
        locals.var_t4__blk1148_dn8 = assign31600_e25720_d_n8;
        locals.var_t4__blk1148_dn9 = assign31600_e25720_d_n9;
        locals.var_t4__blk1148_dn10 = assign31600_e25720_d_n10;
        locals.var_t4__blk1148_dn11 = assign31600_e25720_d_n11;
        locals.var_t4__blk1148_dn12 = assign31600_e25720_d_n12;

        let (assign31610_e25733, assign31610_e25733_d_n3, assign31610_e25733_d_n4, assign31610_e25733_d_n5, assign31610_e25733_d_n6, assign31610_e25733_d_n7, assign31610_e25733_d_n8, assign31610_e25733_d_n9, assign31610_e25733_d_n10, assign31610_e25733_d_n11, assign31610_e25733_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1741 != 0.0)) {
        let assign31610_e25729: f64 = (locals.var_vgsteff2 + locals.var_t4__blk1148);
        let assign31610_e25731: f64 = (assign31610_e25729 / locals.var_tox);
        (assign31610_e25731, ((((locals.var_vgsteff2_dn3 + locals.var_t4__blk1148_dn3) * locals.var_tox) - (assign31610_e25729 * locals.var_tox_dn3)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff2_dn4 + locals.var_t4__blk1148_dn4) * locals.var_tox) - (assign31610_e25729 * locals.var_tox_dn4)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff2_dn5 + locals.var_t4__blk1148_dn5) * locals.var_tox) - (assign31610_e25729 * locals.var_tox_dn5)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff2_dn6 + locals.var_t4__blk1148_dn6) * locals.var_tox) - (assign31610_e25729 * locals.var_tox_dn6)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff2_dn7 + locals.var_t4__blk1148_dn7) * locals.var_tox) - (assign31610_e25729 * locals.var_tox_dn7)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff2_dn8 + locals.var_t4__blk1148_dn8) * locals.var_tox) - (assign31610_e25729 * locals.var_tox_dn8)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff2_dn9 + locals.var_t4__blk1148_dn9) * locals.var_tox) - (assign31610_e25729 * locals.var_tox_dn9)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff2_dn10 + locals.var_t4__blk1148_dn10) * locals.var_tox) - (assign31610_e25729 * locals.var_tox_dn10)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff2_dn11 + locals.var_t4__blk1148_dn11) * locals.var_tox) - (assign31610_e25729 * locals.var_tox_dn11)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff2_dn12 + locals.var_t4__blk1148_dn12) * locals.var_tox) - (assign31610_e25729 * locals.var_tox_dn12)) / (locals.var_tox * locals.var_tox)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign31610_e25733;
        locals.var_t0__blk1144_dn3 = assign31610_e25733_d_n3;
        locals.var_t0__blk1144_dn4 = assign31610_e25733_d_n4;
        locals.var_t0__blk1144_dn5 = assign31610_e25733_d_n5;
        locals.var_t0__blk1144_dn6 = assign31610_e25733_d_n6;
        locals.var_t0__blk1144_dn7 = assign31610_e25733_d_n7;
        locals.var_t0__blk1144_dn8 = assign31610_e25733_d_n8;
        locals.var_t0__blk1144_dn9 = assign31610_e25733_d_n9;
        locals.var_t0__blk1144_dn10 = assign31610_e25733_d_n10;
        locals.var_t0__blk1144_dn11 = assign31610_e25733_d_n11;
        locals.var_t0__blk1144_dn12 = assign31610_e25733_d_n12;

        let (assign31620_e25754, assign31620_e25754_d_n3, assign31620_e25754_d_n4, assign31620_e25754_d_n5, assign31620_e25754_d_n6, assign31620_e25754_d_n7, assign31620_e25754_d_n8, assign31620_e25754_d_n9, assign31620_e25754_d_n10, assign31620_e25754_d_n11, assign31620_e25754_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1741 != 0.0)) {
        let assign31620_e25742: f64 = (locals.var_b4soibdos * 0.7);
        let (assign31620_e25750, assign31620_e25750_d_n3, assign31620_e25750_d_n4, assign31620_e25750_d_n5, assign31620_e25750_d_n6, assign31620_e25750_d_n7, assign31620_e25750_d_n8, assign31620_e25750_d_n9, assign31620_e25750_d_n10, assign31620_e25750_d_n11, assign31620_e25750_d_n12,) = {
            if (locals.var_t0__blk1144 > 1e-38) {
                let assign31620_e25747: f64 = (locals.var_t0__blk1144).ln();
                (assign31620_e25747, (locals.var_t0__blk1144_dn3 / locals.var_t0__blk1144), (locals.var_t0__blk1144_dn4 / locals.var_t0__blk1144), (locals.var_t0__blk1144_dn5 / locals.var_t0__blk1144), (locals.var_t0__blk1144_dn6 / locals.var_t0__blk1144), (locals.var_t0__blk1144_dn7 / locals.var_t0__blk1144), (locals.var_t0__blk1144_dn8 / locals.var_t0__blk1144), (locals.var_t0__blk1144_dn9 / locals.var_t0__blk1144), (locals.var_t0__blk1144_dn10 / locals.var_t0__blk1144), (locals.var_t0__blk1144_dn11 / locals.var_t0__blk1144), (locals.var_t0__blk1144_dn12 / locals.var_t0__blk1144),)
            } else {
                let assign31620_e25749: f64 = (-87.49823353377374);
                (assign31620_e25749, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign31620_e25751: f64 = (assign31620_e25742 * assign31620_e25750);
        let assign31620_e25752: f64 = (assign31620_e25751).exp();
        (assign31620_e25752, (assign31620_e25752 * (assign31620_e25742 * assign31620_e25750_d_n3)), (assign31620_e25752 * (assign31620_e25742 * assign31620_e25750_d_n4)), (assign31620_e25752 * (assign31620_e25742 * assign31620_e25750_d_n5)), (assign31620_e25752 * (assign31620_e25742 * assign31620_e25750_d_n6)), (assign31620_e25752 * (assign31620_e25742 * assign31620_e25750_d_n7)), (assign31620_e25752 * (assign31620_e25742 * assign31620_e25750_d_n8)), (assign31620_e25752 * (assign31620_e25742 * assign31620_e25750_d_n9)), (assign31620_e25752 * (assign31620_e25742 * assign31620_e25750_d_n10)), (assign31620_e25752 * (assign31620_e25742 * assign31620_e25750_d_n11)), (assign31620_e25752 * (assign31620_e25742 * assign31620_e25750_d_n12)),)
    } else {
        (locals.var_tmp__blk1159, locals.var_tmp__blk1159_dn3, locals.var_tmp__blk1159_dn4, locals.var_tmp__blk1159_dn5, locals.var_tmp__blk1159_dn6, locals.var_tmp__blk1159_dn7, locals.var_tmp__blk1159_dn8, locals.var_tmp__blk1159_dn9, locals.var_tmp__blk1159_dn10, locals.var_tmp__blk1159_dn11, locals.var_tmp__blk1159_dn12,)
    }
};
        locals.var_tmp__blk1159 = assign31620_e25754;
        locals.var_tmp__blk1159_dn3 = assign31620_e25754_d_n3;
        locals.var_tmp__blk1159_dn4 = assign31620_e25754_d_n4;
        locals.var_tmp__blk1159_dn5 = assign31620_e25754_d_n5;
        locals.var_tmp__blk1159_dn6 = assign31620_e25754_d_n6;
        locals.var_tmp__blk1159_dn7 = assign31620_e25754_d_n7;
        locals.var_tmp__blk1159_dn8 = assign31620_e25754_d_n8;
        locals.var_tmp__blk1159_dn9 = assign31620_e25754_d_n9;
        locals.var_tmp__blk1159_dn10 = assign31620_e25754_d_n10;
        locals.var_tmp__blk1159_dn11 = assign31620_e25754_d_n11;
        locals.var_tmp__blk1159_dn12 = assign31620_e25754_d_n12;

        let (assign31630_e25765, assign31630_e25765_d_n3, assign31630_e25765_d_n4, assign31630_e25765_d_n5, assign31630_e25765_d_n6, assign31630_e25765_d_n7, assign31630_e25765_d_n8, assign31630_e25765_d_n9, assign31630_e25765_d_n10, assign31630_e25765_d_n11, assign31630_e25765_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1741 != 0.0)) {
        let assign31630_e25763: f64 = (1.0 + locals.var_tmp__blk1159);
        (assign31630_e25763, locals.var_tmp__blk1159_dn3, locals.var_tmp__blk1159_dn4, locals.var_tmp__blk1159_dn5, locals.var_tmp__blk1159_dn6, locals.var_tmp__blk1159_dn7, locals.var_tmp__blk1159_dn8, locals.var_tmp__blk1159_dn9, locals.var_tmp__blk1159_dn10, locals.var_tmp__blk1159_dn11, locals.var_tmp__blk1159_dn12,)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign31630_e25765;
        locals.var_t1__blk1145_dn3 = assign31630_e25765_d_n3;
        locals.var_t1__blk1145_dn4 = assign31630_e25765_d_n4;
        locals.var_t1__blk1145_dn5 = assign31630_e25765_d_n5;
        locals.var_t1__blk1145_dn6 = assign31630_e25765_d_n6;
        locals.var_t1__blk1145_dn7 = assign31630_e25765_d_n7;
        locals.var_t1__blk1145_dn8 = assign31630_e25765_d_n8;
        locals.var_t1__blk1145_dn9 = assign31630_e25765_d_n9;
        locals.var_t1__blk1145_dn10 = assign31630_e25765_d_n10;
        locals.var_t1__blk1145_dn11 = assign31630_e25765_d_n11;
        locals.var_t1__blk1145_dn12 = assign31630_e25765_d_n12;

        let (assign31640_e25778, assign31640_e25778_d_n3, assign31640_e25778_d_n4, assign31640_e25778_d_n5, assign31640_e25778_d_n6, assign31640_e25778_d_n7, assign31640_e25778_d_n8, assign31640_e25778_d_n9, assign31640_e25778_d_n10, assign31640_e25778_d_n11, assign31640_e25778_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1741 != 0.0)) {
        let assign31640_e25774: f64 = (locals.var_b4soiados * 1.9e-9);
        let assign31640_e25776: f64 = (assign31640_e25774 / locals.var_t1__blk1145);
        (assign31640_e25776, (-((assign31640_e25774 * locals.var_t1__blk1145_dn3) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (-((assign31640_e25774 * locals.var_t1__blk1145_dn4) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (-((assign31640_e25774 * locals.var_t1__blk1145_dn5) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (-((assign31640_e25774 * locals.var_t1__blk1145_dn6) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (-((assign31640_e25774 * locals.var_t1__blk1145_dn7) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (-((assign31640_e25774 * locals.var_t1__blk1145_dn8) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (-((assign31640_e25774 * locals.var_t1__blk1145_dn9) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (-((assign31640_e25774 * locals.var_t1__blk1145_dn10) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (-((assign31640_e25774 * locals.var_t1__blk1145_dn11) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (-((assign31640_e25774 * locals.var_t1__blk1145_dn12) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))),)
    } else {
        (locals.var_tcen2, locals.var_tcen2_dn3, locals.var_tcen2_dn4, locals.var_tcen2_dn5, locals.var_tcen2_dn6, locals.var_tcen2_dn7, locals.var_tcen2_dn8, locals.var_tcen2_dn9, locals.var_tcen2_dn10, locals.var_tcen2_dn11, locals.var_tcen2_dn12,)
    }
};
        locals.var_tcen2 = assign31640_e25778;
        locals.var_tcen2_dn3 = assign31640_e25778_d_n3;
        locals.var_tcen2_dn4 = assign31640_e25778_d_n4;
        locals.var_tcen2_dn5 = assign31640_e25778_d_n5;
        locals.var_tcen2_dn6 = assign31640_e25778_d_n6;
        locals.var_tcen2_dn7 = assign31640_e25778_d_n7;
        locals.var_tcen2_dn8 = assign31640_e25778_d_n8;
        locals.var_tcen2_dn9 = assign31640_e25778_d_n9;
        locals.var_tcen2_dn10 = assign31640_e25778_d_n10;
        locals.var_tcen2_dn11 = assign31640_e25778_d_n11;
        locals.var_tcen2_dn12 = assign31640_e25778_d_n12;

        let (assign31650_e25789, assign31650_e25789_d_n3, assign31650_e25789_d_n4, assign31650_e25789_d_n5, assign31650_e25789_d_n6, assign31650_e25789_d_n7, assign31650_e25789_d_n8, assign31650_e25789_d_n9, assign31650_e25789_d_n10, assign31650_e25789_d_n11, assign31650_e25789_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1741 != 0.0)) {
        let assign31650_e25787: f64 = (locals.var_epssub / locals.var_tcen2);
        (assign31650_e25787, (-((locals.var_epssub * locals.var_tcen2_dn3) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn4) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn5) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn6) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn7) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn8) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn9) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn10) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn11) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn12) / (locals.var_tcen2 * locals.var_tcen2))),)
    } else {
        (locals.var_ccen2, locals.var_ccen2_dn3, locals.var_ccen2_dn4, locals.var_ccen2_dn5, locals.var_ccen2_dn6, locals.var_ccen2_dn7, locals.var_ccen2_dn8, locals.var_ccen2_dn9, locals.var_ccen2_dn10, locals.var_ccen2_dn11, locals.var_ccen2_dn12,)
    }
};
        locals.var_ccen2 = assign31650_e25789;
        locals.var_ccen2_dn3 = assign31650_e25789_d_n3;
        locals.var_ccen2_dn4 = assign31650_e25789_d_n4;
        locals.var_ccen2_dn5 = assign31650_e25789_d_n5;
        locals.var_ccen2_dn6 = assign31650_e25789_d_n6;
        locals.var_ccen2_dn7 = assign31650_e25789_d_n7;
        locals.var_ccen2_dn8 = assign31650_e25789_d_n8;
        locals.var_ccen2_dn9 = assign31650_e25789_d_n9;
        locals.var_ccen2_dn10 = assign31650_e25789_d_n10;
        locals.var_ccen2_dn11 = assign31650_e25789_d_n11;
        locals.var_ccen2_dn12 = assign31650_e25789_d_n12;

        let (assign31660_e25802, assign31660_e25802_d_n3, assign31660_e25802_d_n4, assign31660_e25802_d_n5, assign31660_e25802_d_n6, assign31660_e25802_d_n7, assign31660_e25802_d_n8, assign31660_e25802_d_n9, assign31660_e25802_d_n10, assign31660_e25802_d_n11, assign31660_e25802_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1741 != 0.0)) {
        let assign31660_e25799: f64 = (locals.var_cox + locals.var_ccen2);
        let assign31660_e25800: f64 = (locals.var_cox / assign31660_e25799);
        (assign31660_e25800, (((locals.var_cox_dn3 * assign31660_e25799) - (locals.var_cox * (locals.var_cox_dn3 + locals.var_ccen2_dn3))) / (assign31660_e25799 * assign31660_e25799)), (((locals.var_cox_dn4 * assign31660_e25799) - (locals.var_cox * (locals.var_cox_dn4 + locals.var_ccen2_dn4))) / (assign31660_e25799 * assign31660_e25799)), (((locals.var_cox_dn5 * assign31660_e25799) - (locals.var_cox * (locals.var_cox_dn5 + locals.var_ccen2_dn5))) / (assign31660_e25799 * assign31660_e25799)), (((locals.var_cox_dn6 * assign31660_e25799) - (locals.var_cox * (locals.var_cox_dn6 + locals.var_ccen2_dn6))) / (assign31660_e25799 * assign31660_e25799)), (((locals.var_cox_dn7 * assign31660_e25799) - (locals.var_cox * (locals.var_cox_dn7 + locals.var_ccen2_dn7))) / (assign31660_e25799 * assign31660_e25799)), (((locals.var_cox_dn8 * assign31660_e25799) - (locals.var_cox * (locals.var_cox_dn8 + locals.var_ccen2_dn8))) / (assign31660_e25799 * assign31660_e25799)), (((locals.var_cox_dn9 * assign31660_e25799) - (locals.var_cox * (locals.var_cox_dn9 + locals.var_ccen2_dn9))) / (assign31660_e25799 * assign31660_e25799)), (((locals.var_cox_dn10 * assign31660_e25799) - (locals.var_cox * (locals.var_cox_dn10 + locals.var_ccen2_dn10))) / (assign31660_e25799 * assign31660_e25799)), (((locals.var_cox_dn11 * assign31660_e25799) - (locals.var_cox * (locals.var_cox_dn11 + locals.var_ccen2_dn11))) / (assign31660_e25799 * assign31660_e25799)), (((locals.var_cox_dn12 * assign31660_e25799) - (locals.var_cox * (locals.var_cox_dn12 + locals.var_ccen2_dn12))) / (assign31660_e25799 * assign31660_e25799)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign31660_e25802;
        locals.var_t0__blk1144_dn3 = assign31660_e25802_d_n3;
        locals.var_t0__blk1144_dn4 = assign31660_e25802_d_n4;
        locals.var_t0__blk1144_dn5 = assign31660_e25802_d_n5;
        locals.var_t0__blk1144_dn6 = assign31660_e25802_d_n6;
        locals.var_t0__blk1144_dn7 = assign31660_e25802_d_n7;
        locals.var_t0__blk1144_dn8 = assign31660_e25802_d_n8;
        locals.var_t0__blk1144_dn9 = assign31660_e25802_d_n9;
        locals.var_t0__blk1144_dn10 = assign31660_e25802_d_n10;
        locals.var_t0__blk1144_dn11 = assign31660_e25802_d_n11;
        locals.var_t0__blk1144_dn12 = assign31660_e25802_d_n12;

        let (assign31670_e25813, assign31670_e25813_d_n3, assign31670_e25813_d_n4, assign31670_e25813_d_n5, assign31670_e25813_d_n6, assign31670_e25813_d_n7, assign31670_e25813_d_n8, assign31670_e25813_d_n9, assign31670_e25813_d_n10, assign31670_e25813_d_n11, assign31670_e25813_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1741 != 0.0)) {
        let assign31670_e25811: f64 = (locals.var_t0__blk1144 * locals.var_ccen2);
        (assign31670_e25811, ((locals.var_t0__blk1144_dn3 * locals.var_ccen2) + (locals.var_t0__blk1144 * locals.var_ccen2_dn3)), ((locals.var_t0__blk1144_dn4 * locals.var_ccen2) + (locals.var_t0__blk1144 * locals.var_ccen2_dn4)), ((locals.var_t0__blk1144_dn5 * locals.var_ccen2) + (locals.var_t0__blk1144 * locals.var_ccen2_dn5)), ((locals.var_t0__blk1144_dn6 * locals.var_ccen2) + (locals.var_t0__blk1144 * locals.var_ccen2_dn6)), ((locals.var_t0__blk1144_dn7 * locals.var_ccen2) + (locals.var_t0__blk1144 * locals.var_ccen2_dn7)), ((locals.var_t0__blk1144_dn8 * locals.var_ccen2) + (locals.var_t0__blk1144 * locals.var_ccen2_dn8)), ((locals.var_t0__blk1144_dn9 * locals.var_ccen2) + (locals.var_t0__blk1144 * locals.var_ccen2_dn9)), ((locals.var_t0__blk1144_dn10 * locals.var_ccen2) + (locals.var_t0__blk1144 * locals.var_ccen2_dn10)), ((locals.var_t0__blk1144_dn11 * locals.var_ccen2) + (locals.var_t0__blk1144 * locals.var_ccen2_dn11)), ((locals.var_t0__blk1144_dn12 * locals.var_ccen2) + (locals.var_t0__blk1144 * locals.var_ccen2_dn12)),)
    } else {
        (locals.var_coxeff2, locals.var_coxeff2_dn3, locals.var_coxeff2_dn4, locals.var_coxeff2_dn5, locals.var_coxeff2_dn6, locals.var_coxeff2_dn7, locals.var_coxeff2_dn8, locals.var_coxeff2_dn9, locals.var_coxeff2_dn10, locals.var_coxeff2_dn11, locals.var_coxeff2_dn12,)
    }
};
        locals.var_coxeff2 = assign31670_e25813;
        locals.var_coxeff2_dn3 = assign31670_e25813_d_n3;
        locals.var_coxeff2_dn4 = assign31670_e25813_d_n4;
        locals.var_coxeff2_dn5 = assign31670_e25813_d_n5;
        locals.var_coxeff2_dn6 = assign31670_e25813_d_n6;
        locals.var_coxeff2_dn7 = assign31670_e25813_d_n7;
        locals.var_coxeff2_dn8 = assign31670_e25813_d_n8;
        locals.var_coxeff2_dn9 = assign31670_e25813_d_n9;
        locals.var_coxeff2_dn10 = assign31670_e25813_d_n10;
        locals.var_coxeff2_dn11 = assign31670_e25813_d_n11;
        locals.var_coxeff2_dn12 = assign31670_e25813_d_n12;

        let (assign31680_e25826, assign31680_e25826_d_n3, assign31680_e25826_d_n4, assign31680_e25826_d_n5, assign31680_e25826_d_n6, assign31680_e25826_d_n7, assign31680_e25826_d_n8, assign31680_e25826_d_n9, assign31680_e25826_d_n10, assign31680_e25826_d_n11, assign31680_e25826_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1741 != 0.0)) {
        let assign31680_e25822: f64 = (locals.var_coxwl2 * locals.var_coxeff2);
        let assign31680_e25824: f64 = (assign31680_e25822 / locals.var_cox);
        (assign31680_e25824, (((((locals.var_coxwl2_dn3 * locals.var_coxeff2) + (locals.var_coxwl2 * locals.var_coxeff2_dn3)) * locals.var_cox) - (assign31680_e25822 * locals.var_cox_dn3)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl2_dn4 * locals.var_coxeff2) + (locals.var_coxwl2 * locals.var_coxeff2_dn4)) * locals.var_cox) - (assign31680_e25822 * locals.var_cox_dn4)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl2_dn5 * locals.var_coxeff2) + (locals.var_coxwl2 * locals.var_coxeff2_dn5)) * locals.var_cox) - (assign31680_e25822 * locals.var_cox_dn5)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl2_dn6 * locals.var_coxeff2) + (locals.var_coxwl2 * locals.var_coxeff2_dn6)) * locals.var_cox) - (assign31680_e25822 * locals.var_cox_dn6)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl2_dn7 * locals.var_coxeff2) + (locals.var_coxwl2 * locals.var_coxeff2_dn7)) * locals.var_cox) - (assign31680_e25822 * locals.var_cox_dn7)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl2_dn8 * locals.var_coxeff2) + (locals.var_coxwl2 * locals.var_coxeff2_dn8)) * locals.var_cox) - (assign31680_e25822 * locals.var_cox_dn8)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl2_dn9 * locals.var_coxeff2) + (locals.var_coxwl2 * locals.var_coxeff2_dn9)) * locals.var_cox) - (assign31680_e25822 * locals.var_cox_dn9)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl2_dn10 * locals.var_coxeff2) + (locals.var_coxwl2 * locals.var_coxeff2_dn10)) * locals.var_cox) - (assign31680_e25822 * locals.var_cox_dn10)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl2_dn11 * locals.var_coxeff2) + (locals.var_coxwl2 * locals.var_coxeff2_dn11)) * locals.var_cox) - (assign31680_e25822 * locals.var_cox_dn11)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl2_dn12 * locals.var_coxeff2) + (locals.var_coxwl2 * locals.var_coxeff2_dn12)) * locals.var_cox) - (assign31680_e25822 * locals.var_cox_dn12)) / (locals.var_cox * locals.var_cox)),)
    } else {
        (locals.var_coxwlcen2, locals.var_coxwlcen2_dn3, locals.var_coxwlcen2_dn4, locals.var_coxwlcen2_dn5, locals.var_coxwlcen2_dn6, locals.var_coxwlcen2_dn7, locals.var_coxwlcen2_dn8, locals.var_coxwlcen2_dn9, locals.var_coxwlcen2_dn10, locals.var_coxwlcen2_dn11, locals.var_coxwlcen2_dn12,)
    }
};
        locals.var_coxwlcen2 = assign31680_e25826;
        locals.var_coxwlcen2_dn3 = assign31680_e25826_d_n3;
        locals.var_coxwlcen2_dn4 = assign31680_e25826_d_n4;
        locals.var_coxwlcen2_dn5 = assign31680_e25826_d_n5;
        locals.var_coxwlcen2_dn6 = assign31680_e25826_d_n6;
        locals.var_coxwlcen2_dn7 = assign31680_e25826_d_n7;
        locals.var_coxwlcen2_dn8 = assign31680_e25826_d_n8;
        locals.var_coxwlcen2_dn9 = assign31680_e25826_d_n9;
        locals.var_coxwlcen2_dn10 = assign31680_e25826_d_n10;
        locals.var_coxwlcen2_dn11 = assign31680_e25826_d_n11;
        locals.var_coxwlcen2_dn12 = assign31680_e25826_d_n12;

        let (assign31690_e25839, assign31690_e25839_d_n3, assign31690_e25839_d_n4, assign31690_e25839_d_n5, assign31690_e25839_d_n6, assign31690_e25839_d_n7, assign31690_e25839_d_n8, assign31690_e25839_d_n9, assign31690_e25839_d_n10, assign31690_e25839_d_n11, assign31690_e25839_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1741 != 0.0)) {
        let assign31690_e25835: f64 = (locals.var_coxwlb2 * locals.var_coxeff2);
        let assign31690_e25837: f64 = (assign31690_e25835 / locals.var_cox);
        (assign31690_e25837, (((((locals.var_coxwlb2_dn3 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn3)) * locals.var_cox) - (assign31690_e25835 * locals.var_cox_dn3)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn4 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn4)) * locals.var_cox) - (assign31690_e25835 * locals.var_cox_dn4)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn5 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn5)) * locals.var_cox) - (assign31690_e25835 * locals.var_cox_dn5)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn6 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn6)) * locals.var_cox) - (assign31690_e25835 * locals.var_cox_dn6)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn7 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn7)) * locals.var_cox) - (assign31690_e25835 * locals.var_cox_dn7)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn8 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn8)) * locals.var_cox) - (assign31690_e25835 * locals.var_cox_dn8)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn9 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn9)) * locals.var_cox) - (assign31690_e25835 * locals.var_cox_dn9)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn10 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn10)) * locals.var_cox) - (assign31690_e25835 * locals.var_cox_dn10)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn11 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn11)) * locals.var_cox) - (assign31690_e25835 * locals.var_cox_dn11)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn12 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn12)) * locals.var_cox) - (assign31690_e25835 * locals.var_cox_dn12)) / (locals.var_cox * locals.var_cox)),)
    } else {
        (locals.var_coxwlcenb2, locals.var_coxwlcenb2_dn3, locals.var_coxwlcenb2_dn4, locals.var_coxwlcenb2_dn5, locals.var_coxwlcenb2_dn6, locals.var_coxwlcenb2_dn7, locals.var_coxwlcenb2_dn8, locals.var_coxwlcenb2_dn9, locals.var_coxwlcenb2_dn10, locals.var_coxwlcenb2_dn11, locals.var_coxwlcenb2_dn12,)
    }
};
        locals.var_coxwlcenb2 = assign31690_e25839;
        locals.var_coxwlcenb2_dn3 = assign31690_e25839_d_n3;
        locals.var_coxwlcenb2_dn4 = assign31690_e25839_d_n4;
        locals.var_coxwlcenb2_dn5 = assign31690_e25839_d_n5;
        locals.var_coxwlcenb2_dn6 = assign31690_e25839_d_n6;
        locals.var_coxwlcenb2_dn7 = assign31690_e25839_d_n7;
        locals.var_coxwlcenb2_dn8 = assign31690_e25839_d_n8;
        locals.var_coxwlcenb2_dn9 = assign31690_e25839_d_n9;
        locals.var_coxwlcenb2_dn10 = assign31690_e25839_d_n10;
        locals.var_coxwlcenb2_dn11 = assign31690_e25839_d_n11;
        locals.var_coxwlcenb2_dn12 = assign31690_e25839_d_n12;

        let (assign31700_e25848, assign31700_e25848_d_n3, assign31700_e25848_d_n4, assign31700_e25848_d_n5, assign31700_e25848_d_n6, assign31700_e25848_d_n7, assign31700_e25848_d_n8, assign31700_e25848_d_n9, assign31700_e25848_d_n10, assign31700_e25848_d_n11, assign31700_e25848_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) {
        let assign31700_e25846: f64 = (locals.var_vgsteff__blk1175 - locals.var_deltaphi);
        (assign31700_e25846, (locals.var_vgsteff__blk1175_dn3 - locals.var_deltaphi_dn3), (locals.var_vgsteff__blk1175_dn4 - locals.var_deltaphi_dn4), (locals.var_vgsteff__blk1175_dn5 - locals.var_deltaphi_dn5), (locals.var_vgsteff__blk1175_dn6 - locals.var_deltaphi_dn6), (locals.var_vgsteff__blk1175_dn7 - locals.var_deltaphi_dn7), (locals.var_vgsteff__blk1175_dn8 - locals.var_deltaphi_dn8), (locals.var_vgsteff__blk1175_dn9 - locals.var_deltaphi_dn9), (locals.var_vgsteff__blk1175_dn10 - locals.var_deltaphi_dn10), (locals.var_vgsteff__blk1175_dn11 - locals.var_deltaphi_dn11), (locals.var_vgsteff__blk1175_dn12 - locals.var_deltaphi_dn12),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign31700_e25848;
        locals.var_t1__blk1145_dn3 = assign31700_e25848_d_n3;
        locals.var_t1__blk1145_dn4 = assign31700_e25848_d_n4;
        locals.var_t1__blk1145_dn5 = assign31700_e25848_d_n5;
        locals.var_t1__blk1145_dn6 = assign31700_e25848_d_n6;
        locals.var_t1__blk1145_dn7 = assign31700_e25848_d_n7;
        locals.var_t1__blk1145_dn8 = assign31700_e25848_d_n8;
        locals.var_t1__blk1145_dn9 = assign31700_e25848_d_n9;
        locals.var_t1__blk1145_dn10 = assign31700_e25848_d_n10;
        locals.var_t1__blk1145_dn11 = assign31700_e25848_d_n11;
        locals.var_t1__blk1145_dn12 = assign31700_e25848_d_n12;

        let (assign31710_e25857, assign31710_e25857_d_n3, assign31710_e25857_d_n4, assign31710_e25857_d_n5, assign31710_e25857_d_n6, assign31710_e25857_d_n7, assign31710_e25857_d_n8, assign31710_e25857_d_n9, assign31710_e25857_d_n10, assign31710_e25857_d_n11, assign31710_e25857_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) {
        let assign31710_e25855: f64 = (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor);
        (assign31710_e25855, ((locals.var_abulk0_dn3 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn3)), ((locals.var_abulk0_dn4 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn4)), ((locals.var_abulk0_dn5 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn5)), ((locals.var_abulk0_dn6 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn6)), ((locals.var_abulk0_dn7 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn7)), ((locals.var_abulk0_dn8 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn8)), ((locals.var_abulk0_dn9 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn9)), ((locals.var_abulk0_dn10 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn10)), ((locals.var_abulk0_dn11 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn11)), ((locals.var_abulk0_dn12 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn12)),)
    } else {
        (locals.var_abulkcv, locals.var_abulkcv_dn3, locals.var_abulkcv_dn4, locals.var_abulkcv_dn5, locals.var_abulkcv_dn6, locals.var_abulkcv_dn7, locals.var_abulkcv_dn8, locals.var_abulkcv_dn9, locals.var_abulkcv_dn10, locals.var_abulkcv_dn11, locals.var_abulkcv_dn12,)
    }
};
        locals.var_abulkcv = assign31710_e25857;
        locals.var_abulkcv_dn3 = assign31710_e25857_d_n3;
        locals.var_abulkcv_dn4 = assign31710_e25857_d_n4;
        locals.var_abulkcv_dn5 = assign31710_e25857_d_n5;
        locals.var_abulkcv_dn6 = assign31710_e25857_d_n6;
        locals.var_abulkcv_dn7 = assign31710_e25857_d_n7;
        locals.var_abulkcv_dn8 = assign31710_e25857_d_n8;
        locals.var_abulkcv_dn9 = assign31710_e25857_d_n9;
        locals.var_abulkcv_dn10 = assign31710_e25857_d_n10;
        locals.var_abulkcv_dn11 = assign31710_e25857_d_n11;
        locals.var_abulkcv_dn12 = assign31710_e25857_d_n12;

        let (assign31720_e25866, assign31720_e25866_d_n3, assign31720_e25866_d_n4, assign31720_e25866_d_n5, assign31720_e25866_d_n6, assign31720_e25866_d_n7, assign31720_e25866_d_n8, assign31720_e25866_d_n9, assign31720_e25866_d_n10, assign31720_e25866_d_n11, assign31720_e25866_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) {
        let assign31720_e25864: f64 = (locals.var_t1__blk1145 / locals.var_abulkcv);
        (assign31720_e25864, (((locals.var_t1__blk1145_dn3 * locals.var_abulkcv) - (locals.var_t1__blk1145 * locals.var_abulkcv_dn3)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t1__blk1145_dn4 * locals.var_abulkcv) - (locals.var_t1__blk1145 * locals.var_abulkcv_dn4)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t1__blk1145_dn5 * locals.var_abulkcv) - (locals.var_t1__blk1145 * locals.var_abulkcv_dn5)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t1__blk1145_dn6 * locals.var_abulkcv) - (locals.var_t1__blk1145 * locals.var_abulkcv_dn6)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t1__blk1145_dn7 * locals.var_abulkcv) - (locals.var_t1__blk1145 * locals.var_abulkcv_dn7)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t1__blk1145_dn8 * locals.var_abulkcv) - (locals.var_t1__blk1145 * locals.var_abulkcv_dn8)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t1__blk1145_dn9 * locals.var_abulkcv) - (locals.var_t1__blk1145 * locals.var_abulkcv_dn9)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t1__blk1145_dn10 * locals.var_abulkcv) - (locals.var_t1__blk1145 * locals.var_abulkcv_dn10)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t1__blk1145_dn11 * locals.var_abulkcv) - (locals.var_t1__blk1145 * locals.var_abulkcv_dn11)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t1__blk1145_dn12 * locals.var_abulkcv) - (locals.var_t1__blk1145 * locals.var_abulkcv_dn12)) / (locals.var_abulkcv * locals.var_abulkcv)),)
    } else {
        (locals.var_vdsatcv, locals.var_vdsatcv_dn3, locals.var_vdsatcv_dn4, locals.var_vdsatcv_dn5, locals.var_vdsatcv_dn6, locals.var_vdsatcv_dn7, locals.var_vdsatcv_dn8, locals.var_vdsatcv_dn9, locals.var_vdsatcv_dn10, locals.var_vdsatcv_dn11, locals.var_vdsatcv_dn12,)
    }
};
        locals.var_vdsatcv = assign31720_e25866;
        locals.var_vdsatcv_dn3 = assign31720_e25866_d_n3;
        locals.var_vdsatcv_dn4 = assign31720_e25866_d_n4;
        locals.var_vdsatcv_dn5 = assign31720_e25866_d_n5;
        locals.var_vdsatcv_dn6 = assign31720_e25866_d_n6;
        locals.var_vdsatcv_dn7 = assign31720_e25866_d_n7;
        locals.var_vdsatcv_dn8 = assign31720_e25866_d_n8;
        locals.var_vdsatcv_dn9 = assign31720_e25866_d_n9;
        locals.var_vdsatcv_dn10 = assign31720_e25866_d_n10;
        locals.var_vdsatcv_dn11 = assign31720_e25866_d_n11;
        locals.var_vdsatcv_dn12 = assign31720_e25866_d_n12;

        let (assign31730_e25877, assign31730_e25877_d_n3, assign31730_e25877_d_n4, assign31730_e25877_d_n5, assign31730_e25877_d_n6, assign31730_e25877_d_n7, assign31730_e25877_d_n8, assign31730_e25877_d_n9, assign31730_e25877_d_n10, assign31730_e25877_d_n11, assign31730_e25877_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) {
        let assign31730_e25873: f64 = (locals.var_vdsatcv - locals.var_vds_1);
        let assign31730_e25875: f64 = (assign31730_e25873 - 0.02);
        (assign31730_e25875, locals.var_vdsatcv_dn3, locals.var_vdsatcv_dn4, locals.var_vdsatcv_dn5, locals.var_vdsatcv_dn6, (locals.var_vdsatcv_dn7 - locals.var_vds_1_dn7), (locals.var_vdsatcv_dn8 - locals.var_vds_1_dn8), locals.var_vdsatcv_dn9, locals.var_vdsatcv_dn10, locals.var_vdsatcv_dn11, locals.var_vdsatcv_dn12,)
    } else {
        (locals.var_v4, locals.var_v4_dn3, locals.var_v4_dn4, locals.var_v4_dn5, locals.var_v4_dn6, locals.var_v4_dn7, locals.var_v4_dn8, locals.var_v4_dn9, locals.var_v4_dn10, locals.var_v4_dn11, locals.var_v4_dn12,)
    }
};
        locals.var_v4 = assign31730_e25877;
        locals.var_v4_dn3 = assign31730_e25877_d_n3;
        locals.var_v4_dn4 = assign31730_e25877_d_n4;
        locals.var_v4_dn5 = assign31730_e25877_d_n5;
        locals.var_v4_dn6 = assign31730_e25877_d_n6;
        locals.var_v4_dn7 = assign31730_e25877_d_n7;
        locals.var_v4_dn8 = assign31730_e25877_d_n8;
        locals.var_v4_dn9 = assign31730_e25877_d_n9;
        locals.var_v4_dn10 = assign31730_e25877_d_n10;
        locals.var_v4_dn11 = assign31730_e25877_d_n11;
        locals.var_v4_dn12 = assign31730_e25877_d_n12;

    }

    pub(super) fn stamp_transient_block_87(
        locals: &mut StampLocals,
    ) {
        let (assign31740_e25893, assign31740_e25893_d_n3, assign31740_e25893_d_n4, assign31740_e25893_d_n5, assign31740_e25893_d_n6, assign31740_e25893_d_n7, assign31740_e25893_d_n8, assign31740_e25893_d_n9, assign31740_e25893_d_n10, assign31740_e25893_d_n11, assign31740_e25893_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) {
        let assign31740_e25884: f64 = (locals.var_v4 * locals.var_v4);
        let assign31740_e25887: f64 = (4.0 * 0.02);
        let assign31740_e25889: f64 = (assign31740_e25887 * locals.var_vdsatcv);
        let assign31740_e25890: f64 = (assign31740_e25884 + assign31740_e25889);
        let assign31740_e25891: f64 = (assign31740_e25890).sqrt();
        (assign31740_e25891, ((((locals.var_v4_dn3 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn3)) + (assign31740_e25887 * locals.var_vdsatcv_dn3)) / (2.0 * assign31740_e25891)), ((((locals.var_v4_dn4 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn4)) + (assign31740_e25887 * locals.var_vdsatcv_dn4)) / (2.0 * assign31740_e25891)), ((((locals.var_v4_dn5 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn5)) + (assign31740_e25887 * locals.var_vdsatcv_dn5)) / (2.0 * assign31740_e25891)), ((((locals.var_v4_dn6 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn6)) + (assign31740_e25887 * locals.var_vdsatcv_dn6)) / (2.0 * assign31740_e25891)), ((((locals.var_v4_dn7 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn7)) + (assign31740_e25887 * locals.var_vdsatcv_dn7)) / (2.0 * assign31740_e25891)), ((((locals.var_v4_dn8 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn8)) + (assign31740_e25887 * locals.var_vdsatcv_dn8)) / (2.0 * assign31740_e25891)), ((((locals.var_v4_dn9 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn9)) + (assign31740_e25887 * locals.var_vdsatcv_dn9)) / (2.0 * assign31740_e25891)), ((((locals.var_v4_dn10 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn10)) + (assign31740_e25887 * locals.var_vdsatcv_dn10)) / (2.0 * assign31740_e25891)), ((((locals.var_v4_dn11 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn11)) + (assign31740_e25887 * locals.var_vdsatcv_dn11)) / (2.0 * assign31740_e25891)), ((((locals.var_v4_dn12 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn12)) + (assign31740_e25887 * locals.var_vdsatcv_dn12)) / (2.0 * assign31740_e25891)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign31740_e25893;
        locals.var_t0__blk1144_dn3 = assign31740_e25893_d_n3;
        locals.var_t0__blk1144_dn4 = assign31740_e25893_d_n4;
        locals.var_t0__blk1144_dn5 = assign31740_e25893_d_n5;
        locals.var_t0__blk1144_dn6 = assign31740_e25893_d_n6;
        locals.var_t0__blk1144_dn7 = assign31740_e25893_d_n7;
        locals.var_t0__blk1144_dn8 = assign31740_e25893_d_n8;
        locals.var_t0__blk1144_dn9 = assign31740_e25893_d_n9;
        locals.var_t0__blk1144_dn10 = assign31740_e25893_d_n10;
        locals.var_t0__blk1144_dn11 = assign31740_e25893_d_n11;
        locals.var_t0__blk1144_dn12 = assign31740_e25893_d_n12;

        let (assign31750_e25906, assign31750_e25906_d_n3, assign31750_e25906_d_n4, assign31750_e25906_d_n5, assign31750_e25906_d_n6, assign31750_e25906_d_n7, assign31750_e25906_d_n8, assign31750_e25906_d_n9, assign31750_e25906_d_n10, assign31750_e25906_d_n11, assign31750_e25906_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) {
        let assign31750_e25902: f64 = (locals.var_v4 + locals.var_t0__blk1144);
        let assign31750_e25903: f64 = (0.5 * assign31750_e25902);
        let assign31750_e25904: f64 = (locals.var_vdsatcv - assign31750_e25903);
        (assign31750_e25904, (locals.var_vdsatcv_dn3 - (0.5 * (locals.var_v4_dn3 + locals.var_t0__blk1144_dn3))), (locals.var_vdsatcv_dn4 - (0.5 * (locals.var_v4_dn4 + locals.var_t0__blk1144_dn4))), (locals.var_vdsatcv_dn5 - (0.5 * (locals.var_v4_dn5 + locals.var_t0__blk1144_dn5))), (locals.var_vdsatcv_dn6 - (0.5 * (locals.var_v4_dn6 + locals.var_t0__blk1144_dn6))), (locals.var_vdsatcv_dn7 - (0.5 * (locals.var_v4_dn7 + locals.var_t0__blk1144_dn7))), (locals.var_vdsatcv_dn8 - (0.5 * (locals.var_v4_dn8 + locals.var_t0__blk1144_dn8))), (locals.var_vdsatcv_dn9 - (0.5 * (locals.var_v4_dn9 + locals.var_t0__blk1144_dn9))), (locals.var_vdsatcv_dn10 - (0.5 * (locals.var_v4_dn10 + locals.var_t0__blk1144_dn10))), (locals.var_vdsatcv_dn11 - (0.5 * (locals.var_v4_dn11 + locals.var_t0__blk1144_dn11))), (locals.var_vdsatcv_dn12 - (0.5 * (locals.var_v4_dn12 + locals.var_t0__blk1144_dn12))),)
    } else {
        (locals.var_vdseffcv, locals.var_vdseffcv_dn3, locals.var_vdseffcv_dn4, locals.var_vdseffcv_dn5, locals.var_vdseffcv_dn6, locals.var_vdseffcv_dn7, locals.var_vdseffcv_dn8, locals.var_vdseffcv_dn9, locals.var_vdseffcv_dn10, locals.var_vdseffcv_dn11, locals.var_vdseffcv_dn12,)
    }
};
        locals.var_vdseffcv = assign31750_e25906;
        locals.var_vdseffcv_dn3 = assign31750_e25906_d_n3;
        locals.var_vdseffcv_dn4 = assign31750_e25906_d_n4;
        locals.var_vdseffcv_dn5 = assign31750_e25906_d_n5;
        locals.var_vdseffcv_dn6 = assign31750_e25906_d_n6;
        locals.var_vdseffcv_dn7 = assign31750_e25906_d_n7;
        locals.var_vdseffcv_dn8 = assign31750_e25906_d_n8;
        locals.var_vdseffcv_dn9 = assign31750_e25906_d_n9;
        locals.var_vdseffcv_dn10 = assign31750_e25906_d_n10;
        locals.var_vdseffcv_dn11 = assign31750_e25906_d_n11;
        locals.var_vdseffcv_dn12 = assign31750_e25906_d_n12;

        let (assign31760_e25915, assign31760_e25915_d_n3, assign31760_e25915_d_n4, assign31760_e25915_d_n5, assign31760_e25915_d_n6, assign31760_e25915_d_n7, assign31760_e25915_d_n8, assign31760_e25915_d_n9, assign31760_e25915_d_n10, assign31760_e25915_d_n11, assign31760_e25915_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) {
        let assign31760_e25913: f64 = (locals.var_abulkcv * locals.var_vdseffcv);
        (assign31760_e25913, ((locals.var_abulkcv_dn3 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn3)), ((locals.var_abulkcv_dn4 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn4)), ((locals.var_abulkcv_dn5 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn5)), ((locals.var_abulkcv_dn6 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn6)), ((locals.var_abulkcv_dn7 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn7)), ((locals.var_abulkcv_dn8 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn8)), ((locals.var_abulkcv_dn9 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn9)), ((locals.var_abulkcv_dn10 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn10)), ((locals.var_abulkcv_dn11 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn11)), ((locals.var_abulkcv_dn12 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn12)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign31760_e25915;
        locals.var_t0__blk1144_dn3 = assign31760_e25915_d_n3;
        locals.var_t0__blk1144_dn4 = assign31760_e25915_d_n4;
        locals.var_t0__blk1144_dn5 = assign31760_e25915_d_n5;
        locals.var_t0__blk1144_dn6 = assign31760_e25915_d_n6;
        locals.var_t0__blk1144_dn7 = assign31760_e25915_d_n7;
        locals.var_t0__blk1144_dn8 = assign31760_e25915_d_n8;
        locals.var_t0__blk1144_dn9 = assign31760_e25915_d_n9;
        locals.var_t0__blk1144_dn10 = assign31760_e25915_d_n10;
        locals.var_t0__blk1144_dn11 = assign31760_e25915_d_n11;
        locals.var_t0__blk1144_dn12 = assign31760_e25915_d_n12;

        let (assign31770_e25930, assign31770_e25930_d_n3, assign31770_e25930_d_n4, assign31770_e25930_d_n5, assign31770_e25930_d_n6, assign31770_e25930_d_n7, assign31770_e25930_d_n8, assign31770_e25930_d_n9, assign31770_e25930_d_n10, assign31770_e25930_d_n11, assign31770_e25930_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) {
        let assign31770_e25924: f64 = (0.5 * locals.var_t0__blk1144);
        let assign31770_e25925: f64 = (locals.var_t1__blk1145 - assign31770_e25924);
        let assign31770_e25927: f64 = (assign31770_e25925 + 1e-20);
        let assign31770_e25928: f64 = (12.0 * assign31770_e25927);
        (assign31770_e25928, (12.0 * (locals.var_t1__blk1145_dn3 - (0.5 * locals.var_t0__blk1144_dn3))), (12.0 * (locals.var_t1__blk1145_dn4 - (0.5 * locals.var_t0__blk1144_dn4))), (12.0 * (locals.var_t1__blk1145_dn5 - (0.5 * locals.var_t0__blk1144_dn5))), (12.0 * (locals.var_t1__blk1145_dn6 - (0.5 * locals.var_t0__blk1144_dn6))), (12.0 * (locals.var_t1__blk1145_dn7 - (0.5 * locals.var_t0__blk1144_dn7))), (12.0 * (locals.var_t1__blk1145_dn8 - (0.5 * locals.var_t0__blk1144_dn8))), (12.0 * (locals.var_t1__blk1145_dn9 - (0.5 * locals.var_t0__blk1144_dn9))), (12.0 * (locals.var_t1__blk1145_dn10 - (0.5 * locals.var_t0__blk1144_dn10))), (12.0 * (locals.var_t1__blk1145_dn11 - (0.5 * locals.var_t0__blk1144_dn11))), (12.0 * (locals.var_t1__blk1145_dn12 - (0.5 * locals.var_t0__blk1144_dn12))),)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign31770_e25930;
        locals.var_t2__blk1146_dn3 = assign31770_e25930_d_n3;
        locals.var_t2__blk1146_dn4 = assign31770_e25930_d_n4;
        locals.var_t2__blk1146_dn5 = assign31770_e25930_d_n5;
        locals.var_t2__blk1146_dn6 = assign31770_e25930_d_n6;
        locals.var_t2__blk1146_dn7 = assign31770_e25930_d_n7;
        locals.var_t2__blk1146_dn8 = assign31770_e25930_d_n8;
        locals.var_t2__blk1146_dn9 = assign31770_e25930_d_n9;
        locals.var_t2__blk1146_dn10 = assign31770_e25930_d_n10;
        locals.var_t2__blk1146_dn11 = assign31770_e25930_d_n11;
        locals.var_t2__blk1146_dn12 = assign31770_e25930_d_n12;

        let (assign31780_e25939, assign31780_e25939_d_n3, assign31780_e25939_d_n4, assign31780_e25939_d_n5, assign31780_e25939_d_n6, assign31780_e25939_d_n7, assign31780_e25939_d_n8, assign31780_e25939_d_n9, assign31780_e25939_d_n10, assign31780_e25939_d_n11, assign31780_e25939_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) {
        let assign31780_e25937: f64 = (locals.var_t0__blk1144 / locals.var_t2__blk1146);
        (assign31780_e25937, (((locals.var_t0__blk1144_dn3 * locals.var_t2__blk1146) - (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn3)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146)), (((locals.var_t0__blk1144_dn4 * locals.var_t2__blk1146) - (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn4)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146)), (((locals.var_t0__blk1144_dn5 * locals.var_t2__blk1146) - (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn5)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146)), (((locals.var_t0__blk1144_dn6 * locals.var_t2__blk1146) - (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn6)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146)), (((locals.var_t0__blk1144_dn7 * locals.var_t2__blk1146) - (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn7)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146)), (((locals.var_t0__blk1144_dn8 * locals.var_t2__blk1146) - (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn8)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146)), (((locals.var_t0__blk1144_dn9 * locals.var_t2__blk1146) - (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn9)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146)), (((locals.var_t0__blk1144_dn10 * locals.var_t2__blk1146) - (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn10)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146)), (((locals.var_t0__blk1144_dn11 * locals.var_t2__blk1146) - (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn11)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146)), (((locals.var_t0__blk1144_dn12 * locals.var_t2__blk1146) - (locals.var_t0__blk1144 * locals.var_t2__blk1146_dn12)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146)),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign31780_e25939;
        locals.var_t3__blk1147_dn3 = assign31780_e25939_d_n3;
        locals.var_t3__blk1147_dn4 = assign31780_e25939_d_n4;
        locals.var_t3__blk1147_dn5 = assign31780_e25939_d_n5;
        locals.var_t3__blk1147_dn6 = assign31780_e25939_d_n6;
        locals.var_t3__blk1147_dn7 = assign31780_e25939_d_n7;
        locals.var_t3__blk1147_dn8 = assign31780_e25939_d_n8;
        locals.var_t3__blk1147_dn9 = assign31780_e25939_d_n9;
        locals.var_t3__blk1147_dn10 = assign31780_e25939_d_n10;
        locals.var_t3__blk1147_dn11 = assign31780_e25939_d_n11;
        locals.var_t3__blk1147_dn12 = assign31780_e25939_d_n12;

        let (assign31790_e25954, assign31790_e25954_d_n3, assign31790_e25954_d_n4, assign31790_e25954_d_n5, assign31790_e25954_d_n6, assign31790_e25954_d_n7, assign31790_e25954_d_n8, assign31790_e25954_d_n9, assign31790_e25954_d_n10, assign31790_e25954_d_n11, assign31790_e25954_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) {
        let assign31790_e25949: f64 = (0.5 - locals.var_t3__blk1147);
        let assign31790_e25950: f64 = (locals.var_t0__blk1144 * assign31790_e25949);
        let assign31790_e25951: f64 = (locals.var_t1__blk1145 - assign31790_e25950);
        let assign31790_e25952: f64 = (locals.var_coxwlcen * assign31790_e25951);
        (assign31790_e25952, ((locals.var_coxwlcen_dn3 * assign31790_e25951) + (locals.var_coxwlcen * (locals.var_t1__blk1145_dn3 - ((locals.var_t0__blk1144_dn3 * assign31790_e25949) + (locals.var_t0__blk1144 * (-locals.var_t3__blk1147_dn3)))))), ((locals.var_coxwlcen_dn4 * assign31790_e25951) + (locals.var_coxwlcen * (locals.var_t1__blk1145_dn4 - ((locals.var_t0__blk1144_dn4 * assign31790_e25949) + (locals.var_t0__blk1144 * (-locals.var_t3__blk1147_dn4)))))), ((locals.var_coxwlcen_dn5 * assign31790_e25951) + (locals.var_coxwlcen * (locals.var_t1__blk1145_dn5 - ((locals.var_t0__blk1144_dn5 * assign31790_e25949) + (locals.var_t0__blk1144 * (-locals.var_t3__blk1147_dn5)))))), ((locals.var_coxwlcen_dn6 * assign31790_e25951) + (locals.var_coxwlcen * (locals.var_t1__blk1145_dn6 - ((locals.var_t0__blk1144_dn6 * assign31790_e25949) + (locals.var_t0__blk1144 * (-locals.var_t3__blk1147_dn6)))))), ((locals.var_coxwlcen_dn7 * assign31790_e25951) + (locals.var_coxwlcen * (locals.var_t1__blk1145_dn7 - ((locals.var_t0__blk1144_dn7 * assign31790_e25949) + (locals.var_t0__blk1144 * (-locals.var_t3__blk1147_dn7)))))), ((locals.var_coxwlcen_dn8 * assign31790_e25951) + (locals.var_coxwlcen * (locals.var_t1__blk1145_dn8 - ((locals.var_t0__blk1144_dn8 * assign31790_e25949) + (locals.var_t0__blk1144 * (-locals.var_t3__blk1147_dn8)))))), ((locals.var_coxwlcen_dn9 * assign31790_e25951) + (locals.var_coxwlcen * (locals.var_t1__blk1145_dn9 - ((locals.var_t0__blk1144_dn9 * assign31790_e25949) + (locals.var_t0__blk1144 * (-locals.var_t3__blk1147_dn9)))))), ((locals.var_coxwlcen_dn10 * assign31790_e25951) + (locals.var_coxwlcen * (locals.var_t1__blk1145_dn10 - ((locals.var_t0__blk1144_dn10 * assign31790_e25949) + (locals.var_t0__blk1144 * (-locals.var_t3__blk1147_dn10)))))), ((locals.var_coxwlcen_dn11 * assign31790_e25951) + (locals.var_coxwlcen * (locals.var_t1__blk1145_dn11 - ((locals.var_t0__blk1144_dn11 * assign31790_e25949) + (locals.var_t0__blk1144 * (-locals.var_t3__blk1147_dn11)))))), ((locals.var_coxwlcen_dn12 * assign31790_e25951) + (locals.var_coxwlcen * (locals.var_t1__blk1145_dn12 - ((locals.var_t0__blk1144_dn12 * assign31790_e25949) + (locals.var_t0__blk1144 * (-locals.var_t3__blk1147_dn12)))))),)
    } else {
        (locals.var_qinv, locals.var_qinv_dn3, locals.var_qinv_dn4, locals.var_qinv_dn5, locals.var_qinv_dn6, locals.var_qinv_dn7, locals.var_qinv_dn8, locals.var_qinv_dn9, locals.var_qinv_dn10, locals.var_qinv_dn11, locals.var_qinv_dn12,)
    }
};
        locals.var_qinv = assign31790_e25954;
        locals.var_qinv_dn3 = assign31790_e25954_d_n3;
        locals.var_qinv_dn4 = assign31790_e25954_d_n4;
        locals.var_qinv_dn5 = assign31790_e25954_d_n5;
        locals.var_qinv_dn6 = assign31790_e25954_d_n6;
        locals.var_qinv_dn7 = assign31790_e25954_d_n7;
        locals.var_qinv_dn8 = assign31790_e25954_d_n8;
        locals.var_qinv_dn9 = assign31790_e25954_d_n9;
        locals.var_qinv_dn10 = assign31790_e25954_d_n10;
        locals.var_qinv_dn11 = assign31790_e25954_d_n11;
        locals.var_qinv_dn12 = assign31790_e25954_d_n12;

        let (assign31810_e25968, assign31810_e25968_d_n3, assign31810_e25968_d_n4, assign31810_e25968_d_n5, assign31810_e25968_d_n6, assign31810_e25968_d_n7, assign31810_e25968_d_n8, assign31810_e25968_d_n9, assign31810_e25968_d_n10, assign31810_e25968_d_n11, assign31810_e25968_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) {
        (locals.var_qinv, locals.var_qinv_dn3, locals.var_qinv_dn4, locals.var_qinv_dn5, locals.var_qinv_dn6, locals.var_qinv_dn7, locals.var_qinv_dn8, locals.var_qinv_dn9, locals.var_qinv_dn10, locals.var_qinv_dn11, locals.var_qinv_dn12,)
    } else {
        (locals.var_qgate, locals.var_qgate_dn3, locals.var_qgate_dn4, locals.var_qgate_dn5, locals.var_qgate_dn6, locals.var_qgate_dn7, locals.var_qgate_dn8, locals.var_qgate_dn9, locals.var_qgate_dn10, locals.var_qgate_dn11, locals.var_qgate_dn12,)
    }
};
        locals.var_qgate = assign31810_e25968;
        locals.var_qgate_dn3 = assign31810_e25968_d_n3;
        locals.var_qgate_dn4 = assign31810_e25968_d_n4;
        locals.var_qgate_dn5 = assign31810_e25968_d_n5;
        locals.var_qgate_dn6 = assign31810_e25968_d_n6;
        locals.var_qgate_dn7 = assign31810_e25968_d_n7;
        locals.var_qgate_dn8 = assign31810_e25968_d_n8;
        locals.var_qgate_dn9 = assign31810_e25968_d_n9;
        locals.var_qgate_dn10 = assign31810_e25968_d_n10;
        locals.var_qgate_dn11 = assign31810_e25968_d_n11;
        locals.var_qgate_dn12 = assign31810_e25968_d_n12;

        let assign31820_e25979: f64 = if (((locals.var_b4soisoimod != 2.0) && (locals.var_b4soibodymod != 0.0)) && (locals.var_b4soiagbcp2 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1742 = assign31820_e25979;

        let (assign31830_e25990, assign31830_e25990_d_n3, assign31830_e25990_d_n4, assign31830_e25990_d_n5, assign31830_e25990_d_n6, assign31830_e25990_d_n7, assign31830_e25990_d_n8, assign31830_e25990_d_n9, assign31830_e25990_d_n10, assign31830_e25990_d_n11, assign31830_e25990_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1742 != 0.0)) {
        let assign31830_e25988: f64 = (locals.var_vgsteff2 - locals.var_deltaphi2);
        (assign31830_e25988, (locals.var_vgsteff2_dn3 - locals.var_deltaphi2_dn3), (locals.var_vgsteff2_dn4 - locals.var_deltaphi2_dn4), (locals.var_vgsteff2_dn5 - locals.var_deltaphi2_dn5), (locals.var_vgsteff2_dn6 - locals.var_deltaphi2_dn6), (locals.var_vgsteff2_dn7 - locals.var_deltaphi2_dn7), (locals.var_vgsteff2_dn8 - locals.var_deltaphi2_dn8), (locals.var_vgsteff2_dn9 - locals.var_deltaphi2_dn9), (locals.var_vgsteff2_dn10 - locals.var_deltaphi2_dn10), (locals.var_vgsteff2_dn11 - locals.var_deltaphi2_dn11), (locals.var_vgsteff2_dn12 - locals.var_deltaphi2_dn12),)
    } else {
        (locals.var_t12, locals.var_t12_dn3, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn12,)
    }
};
        locals.var_t12 = assign31830_e25990;
        locals.var_t12_dn3 = assign31830_e25990_d_n3;
        locals.var_t12_dn4 = assign31830_e25990_d_n4;
        locals.var_t12_dn5 = assign31830_e25990_d_n5;
        locals.var_t12_dn6 = assign31830_e25990_d_n6;
        locals.var_t12_dn7 = assign31830_e25990_d_n7;
        locals.var_t12_dn8 = assign31830_e25990_d_n8;
        locals.var_t12_dn9 = assign31830_e25990_d_n9;
        locals.var_t12_dn10 = assign31830_e25990_d_n10;
        locals.var_t12_dn11 = assign31830_e25990_d_n11;
        locals.var_t12_dn12 = assign31830_e25990_d_n12;

        let (assign31840_e26001, assign31840_e26001_d_n3, assign31840_e26001_d_n4, assign31840_e26001_d_n5, assign31840_e26001_d_n6, assign31840_e26001_d_n7, assign31840_e26001_d_n8, assign31840_e26001_d_n9, assign31840_e26001_d_n10, assign31840_e26001_d_n11, assign31840_e26001_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1742 != 0.0)) {
        let assign31840_e25999: f64 = (locals.var_t12 / locals.var_abulkcv);
        (assign31840_e25999, (((locals.var_t12_dn3 * locals.var_abulkcv) - (locals.var_t12 * locals.var_abulkcv_dn3)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t12_dn4 * locals.var_abulkcv) - (locals.var_t12 * locals.var_abulkcv_dn4)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t12_dn5 * locals.var_abulkcv) - (locals.var_t12 * locals.var_abulkcv_dn5)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t12_dn6 * locals.var_abulkcv) - (locals.var_t12 * locals.var_abulkcv_dn6)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t12_dn7 * locals.var_abulkcv) - (locals.var_t12 * locals.var_abulkcv_dn7)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t12_dn8 * locals.var_abulkcv) - (locals.var_t12 * locals.var_abulkcv_dn8)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t12_dn9 * locals.var_abulkcv) - (locals.var_t12 * locals.var_abulkcv_dn9)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t12_dn10 * locals.var_abulkcv) - (locals.var_t12 * locals.var_abulkcv_dn10)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t12_dn11 * locals.var_abulkcv) - (locals.var_t12 * locals.var_abulkcv_dn11)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t12_dn12 * locals.var_abulkcv) - (locals.var_t12 * locals.var_abulkcv_dn12)) / (locals.var_abulkcv * locals.var_abulkcv)),)
    } else {
        (locals.var_vdsatcv2, locals.var_vdsatcv2_dn3, locals.var_vdsatcv2_dn4, locals.var_vdsatcv2_dn5, locals.var_vdsatcv2_dn6, locals.var_vdsatcv2_dn7, locals.var_vdsatcv2_dn8, locals.var_vdsatcv2_dn9, locals.var_vdsatcv2_dn10, locals.var_vdsatcv2_dn11, locals.var_vdsatcv2_dn12,)
    }
};
        locals.var_vdsatcv2 = assign31840_e26001;
        locals.var_vdsatcv2_dn3 = assign31840_e26001_d_n3;
        locals.var_vdsatcv2_dn4 = assign31840_e26001_d_n4;
        locals.var_vdsatcv2_dn5 = assign31840_e26001_d_n5;
        locals.var_vdsatcv2_dn6 = assign31840_e26001_d_n6;
        locals.var_vdsatcv2_dn7 = assign31840_e26001_d_n7;
        locals.var_vdsatcv2_dn8 = assign31840_e26001_d_n8;
        locals.var_vdsatcv2_dn9 = assign31840_e26001_d_n9;
        locals.var_vdsatcv2_dn10 = assign31840_e26001_d_n10;
        locals.var_vdsatcv2_dn11 = assign31840_e26001_d_n11;
        locals.var_vdsatcv2_dn12 = assign31840_e26001_d_n12;

        let (assign31850_e26014, assign31850_e26014_d_n3, assign31850_e26014_d_n4, assign31850_e26014_d_n5, assign31850_e26014_d_n6, assign31850_e26014_d_n7, assign31850_e26014_d_n8, assign31850_e26014_d_n9, assign31850_e26014_d_n10, assign31850_e26014_d_n11, assign31850_e26014_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1742 != 0.0)) {
        let assign31850_e26010: f64 = (locals.var_vdsatcv2 - locals.var_vds_1);
        let assign31850_e26012: f64 = (assign31850_e26010 - 0.02);
        (assign31850_e26012, locals.var_vdsatcv2_dn3, locals.var_vdsatcv2_dn4, locals.var_vdsatcv2_dn5, locals.var_vdsatcv2_dn6, (locals.var_vdsatcv2_dn7 - locals.var_vds_1_dn7), (locals.var_vdsatcv2_dn8 - locals.var_vds_1_dn8), locals.var_vdsatcv2_dn9, locals.var_vdsatcv2_dn10, locals.var_vdsatcv2_dn11, locals.var_vdsatcv2_dn12,)
    } else {
        (locals.var_v4, locals.var_v4_dn3, locals.var_v4_dn4, locals.var_v4_dn5, locals.var_v4_dn6, locals.var_v4_dn7, locals.var_v4_dn8, locals.var_v4_dn9, locals.var_v4_dn10, locals.var_v4_dn11, locals.var_v4_dn12,)
    }
};
        locals.var_v4 = assign31850_e26014;
        locals.var_v4_dn3 = assign31850_e26014_d_n3;
        locals.var_v4_dn4 = assign31850_e26014_d_n4;
        locals.var_v4_dn5 = assign31850_e26014_d_n5;
        locals.var_v4_dn6 = assign31850_e26014_d_n6;
        locals.var_v4_dn7 = assign31850_e26014_d_n7;
        locals.var_v4_dn8 = assign31850_e26014_d_n8;
        locals.var_v4_dn9 = assign31850_e26014_d_n9;
        locals.var_v4_dn10 = assign31850_e26014_d_n10;
        locals.var_v4_dn11 = assign31850_e26014_d_n11;
        locals.var_v4_dn12 = assign31850_e26014_d_n12;

        let (assign31860_e26032, assign31860_e26032_d_n3, assign31860_e26032_d_n4, assign31860_e26032_d_n5, assign31860_e26032_d_n6, assign31860_e26032_d_n7, assign31860_e26032_d_n8, assign31860_e26032_d_n9, assign31860_e26032_d_n10, assign31860_e26032_d_n11, assign31860_e26032_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1742 != 0.0)) {
        let assign31860_e26023: f64 = (locals.var_v4 * locals.var_v4);
        let assign31860_e26026: f64 = (4.0 * 0.02);
        let assign31860_e26028: f64 = (assign31860_e26026 * locals.var_vdsatcv2);
        let assign31860_e26029: f64 = (assign31860_e26023 + assign31860_e26028);
        let assign31860_e26030: f64 = (assign31860_e26029).sqrt();
        (assign31860_e26030, ((((locals.var_v4_dn3 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn3)) + (assign31860_e26026 * locals.var_vdsatcv2_dn3)) / (2.0 * assign31860_e26030)), ((((locals.var_v4_dn4 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn4)) + (assign31860_e26026 * locals.var_vdsatcv2_dn4)) / (2.0 * assign31860_e26030)), ((((locals.var_v4_dn5 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn5)) + (assign31860_e26026 * locals.var_vdsatcv2_dn5)) / (2.0 * assign31860_e26030)), ((((locals.var_v4_dn6 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn6)) + (assign31860_e26026 * locals.var_vdsatcv2_dn6)) / (2.0 * assign31860_e26030)), ((((locals.var_v4_dn7 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn7)) + (assign31860_e26026 * locals.var_vdsatcv2_dn7)) / (2.0 * assign31860_e26030)), ((((locals.var_v4_dn8 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn8)) + (assign31860_e26026 * locals.var_vdsatcv2_dn8)) / (2.0 * assign31860_e26030)), ((((locals.var_v4_dn9 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn9)) + (assign31860_e26026 * locals.var_vdsatcv2_dn9)) / (2.0 * assign31860_e26030)), ((((locals.var_v4_dn10 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn10)) + (assign31860_e26026 * locals.var_vdsatcv2_dn10)) / (2.0 * assign31860_e26030)), ((((locals.var_v4_dn11 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn11)) + (assign31860_e26026 * locals.var_vdsatcv2_dn11)) / (2.0 * assign31860_e26030)), ((((locals.var_v4_dn12 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn12)) + (assign31860_e26026 * locals.var_vdsatcv2_dn12)) / (2.0 * assign31860_e26030)),)
    } else {
        (locals.var_t02, locals.var_t02_dn3, locals.var_t02_dn4, locals.var_t02_dn5, locals.var_t02_dn6, locals.var_t02_dn7, locals.var_t02_dn8, locals.var_t02_dn9, locals.var_t02_dn10, locals.var_t02_dn11, locals.var_t02_dn12,)
    }
};
        locals.var_t02 = assign31860_e26032;
        locals.var_t02_dn3 = assign31860_e26032_d_n3;
        locals.var_t02_dn4 = assign31860_e26032_d_n4;
        locals.var_t02_dn5 = assign31860_e26032_d_n5;
        locals.var_t02_dn6 = assign31860_e26032_d_n6;
        locals.var_t02_dn7 = assign31860_e26032_d_n7;
        locals.var_t02_dn8 = assign31860_e26032_d_n8;
        locals.var_t02_dn9 = assign31860_e26032_d_n9;
        locals.var_t02_dn10 = assign31860_e26032_d_n10;
        locals.var_t02_dn11 = assign31860_e26032_d_n11;
        locals.var_t02_dn12 = assign31860_e26032_d_n12;

        let (assign31870_e26047, assign31870_e26047_d_n3, assign31870_e26047_d_n4, assign31870_e26047_d_n5, assign31870_e26047_d_n6, assign31870_e26047_d_n7, assign31870_e26047_d_n8, assign31870_e26047_d_n9, assign31870_e26047_d_n10, assign31870_e26047_d_n11, assign31870_e26047_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1742 != 0.0)) {
        let assign31870_e26043: f64 = (locals.var_v4 + locals.var_t02);
        let assign31870_e26044: f64 = (0.5 * assign31870_e26043);
        let assign31870_e26045: f64 = (locals.var_vdsatcv2 - assign31870_e26044);
        (assign31870_e26045, (locals.var_vdsatcv2_dn3 - (0.5 * (locals.var_v4_dn3 + locals.var_t02_dn3))), (locals.var_vdsatcv2_dn4 - (0.5 * (locals.var_v4_dn4 + locals.var_t02_dn4))), (locals.var_vdsatcv2_dn5 - (0.5 * (locals.var_v4_dn5 + locals.var_t02_dn5))), (locals.var_vdsatcv2_dn6 - (0.5 * (locals.var_v4_dn6 + locals.var_t02_dn6))), (locals.var_vdsatcv2_dn7 - (0.5 * (locals.var_v4_dn7 + locals.var_t02_dn7))), (locals.var_vdsatcv2_dn8 - (0.5 * (locals.var_v4_dn8 + locals.var_t02_dn8))), (locals.var_vdsatcv2_dn9 - (0.5 * (locals.var_v4_dn9 + locals.var_t02_dn9))), (locals.var_vdsatcv2_dn10 - (0.5 * (locals.var_v4_dn10 + locals.var_t02_dn10))), (locals.var_vdsatcv2_dn11 - (0.5 * (locals.var_v4_dn11 + locals.var_t02_dn11))), (locals.var_vdsatcv2_dn12 - (0.5 * (locals.var_v4_dn12 + locals.var_t02_dn12))),)
    } else {
        (locals.var_vdseffcv2, locals.var_vdseffcv2_dn3, locals.var_vdseffcv2_dn4, locals.var_vdseffcv2_dn5, locals.var_vdseffcv2_dn6, locals.var_vdseffcv2_dn7, locals.var_vdseffcv2_dn8, locals.var_vdseffcv2_dn9, locals.var_vdseffcv2_dn10, locals.var_vdseffcv2_dn11, locals.var_vdseffcv2_dn12,)
    }
};
        locals.var_vdseffcv2 = assign31870_e26047;
        locals.var_vdseffcv2_dn3 = assign31870_e26047_d_n3;
        locals.var_vdseffcv2_dn4 = assign31870_e26047_d_n4;
        locals.var_vdseffcv2_dn5 = assign31870_e26047_d_n5;
        locals.var_vdseffcv2_dn6 = assign31870_e26047_d_n6;
        locals.var_vdseffcv2_dn7 = assign31870_e26047_d_n7;
        locals.var_vdseffcv2_dn8 = assign31870_e26047_d_n8;
        locals.var_vdseffcv2_dn9 = assign31870_e26047_d_n9;
        locals.var_vdseffcv2_dn10 = assign31870_e26047_d_n10;
        locals.var_vdseffcv2_dn11 = assign31870_e26047_d_n11;
        locals.var_vdseffcv2_dn12 = assign31870_e26047_d_n12;

        let (assign31880_e26058, assign31880_e26058_d_n3, assign31880_e26058_d_n4, assign31880_e26058_d_n5, assign31880_e26058_d_n6, assign31880_e26058_d_n7, assign31880_e26058_d_n8, assign31880_e26058_d_n9, assign31880_e26058_d_n10, assign31880_e26058_d_n11, assign31880_e26058_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1742 != 0.0)) {
        let assign31880_e26056: f64 = (locals.var_abulkcv * locals.var_vdseffcv2);
        (assign31880_e26056, ((locals.var_abulkcv_dn3 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn3)), ((locals.var_abulkcv_dn4 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn4)), ((locals.var_abulkcv_dn5 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn5)), ((locals.var_abulkcv_dn6 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn6)), ((locals.var_abulkcv_dn7 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn7)), ((locals.var_abulkcv_dn8 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn8)), ((locals.var_abulkcv_dn9 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn9)), ((locals.var_abulkcv_dn10 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn10)), ((locals.var_abulkcv_dn11 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn11)), ((locals.var_abulkcv_dn12 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn12)),)
    } else {
        (locals.var_t02, locals.var_t02_dn3, locals.var_t02_dn4, locals.var_t02_dn5, locals.var_t02_dn6, locals.var_t02_dn7, locals.var_t02_dn8, locals.var_t02_dn9, locals.var_t02_dn10, locals.var_t02_dn11, locals.var_t02_dn12,)
    }
};
        locals.var_t02 = assign31880_e26058;
        locals.var_t02_dn3 = assign31880_e26058_d_n3;
        locals.var_t02_dn4 = assign31880_e26058_d_n4;
        locals.var_t02_dn5 = assign31880_e26058_d_n5;
        locals.var_t02_dn6 = assign31880_e26058_d_n6;
        locals.var_t02_dn7 = assign31880_e26058_d_n7;
        locals.var_t02_dn8 = assign31880_e26058_d_n8;
        locals.var_t02_dn9 = assign31880_e26058_d_n9;
        locals.var_t02_dn10 = assign31880_e26058_d_n10;
        locals.var_t02_dn11 = assign31880_e26058_d_n11;
        locals.var_t02_dn12 = assign31880_e26058_d_n12;

        let (assign31890_e26075, assign31890_e26075_d_n3, assign31890_e26075_d_n4, assign31890_e26075_d_n5, assign31890_e26075_d_n6, assign31890_e26075_d_n7, assign31890_e26075_d_n8, assign31890_e26075_d_n9, assign31890_e26075_d_n10, assign31890_e26075_d_n11, assign31890_e26075_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1742 != 0.0)) {
        let assign31890_e26069: f64 = (0.5 * locals.var_t02);
        let assign31890_e26070: f64 = (locals.var_t12 - assign31890_e26069);
        let assign31890_e26072: f64 = (assign31890_e26070 + 1e-20);
        let assign31890_e26073: f64 = (12.0 * assign31890_e26072);
        (assign31890_e26073, (12.0 * (locals.var_t12_dn3 - (0.5 * locals.var_t02_dn3))), (12.0 * (locals.var_t12_dn4 - (0.5 * locals.var_t02_dn4))), (12.0 * (locals.var_t12_dn5 - (0.5 * locals.var_t02_dn5))), (12.0 * (locals.var_t12_dn6 - (0.5 * locals.var_t02_dn6))), (12.0 * (locals.var_t12_dn7 - (0.5 * locals.var_t02_dn7))), (12.0 * (locals.var_t12_dn8 - (0.5 * locals.var_t02_dn8))), (12.0 * (locals.var_t12_dn9 - (0.5 * locals.var_t02_dn9))), (12.0 * (locals.var_t12_dn10 - (0.5 * locals.var_t02_dn10))), (12.0 * (locals.var_t12_dn11 - (0.5 * locals.var_t02_dn11))), (12.0 * (locals.var_t12_dn12 - (0.5 * locals.var_t02_dn12))),)
    } else {
        (locals.var_t22, locals.var_t22_dn3, locals.var_t22_dn4, locals.var_t22_dn5, locals.var_t22_dn6, locals.var_t22_dn7, locals.var_t22_dn8, locals.var_t22_dn9, locals.var_t22_dn10, locals.var_t22_dn11, locals.var_t22_dn12,)
    }
};
        locals.var_t22 = assign31890_e26075;
        locals.var_t22_dn3 = assign31890_e26075_d_n3;
        locals.var_t22_dn4 = assign31890_e26075_d_n4;
        locals.var_t22_dn5 = assign31890_e26075_d_n5;
        locals.var_t22_dn6 = assign31890_e26075_d_n6;
        locals.var_t22_dn7 = assign31890_e26075_d_n7;
        locals.var_t22_dn8 = assign31890_e26075_d_n8;
        locals.var_t22_dn9 = assign31890_e26075_d_n9;
        locals.var_t22_dn10 = assign31890_e26075_d_n10;
        locals.var_t22_dn11 = assign31890_e26075_d_n11;
        locals.var_t22_dn12 = assign31890_e26075_d_n12;

        let (assign31900_e26086, assign31900_e26086_d_n3, assign31900_e26086_d_n4, assign31900_e26086_d_n5, assign31900_e26086_d_n6, assign31900_e26086_d_n7, assign31900_e26086_d_n8, assign31900_e26086_d_n9, assign31900_e26086_d_n10, assign31900_e26086_d_n11, assign31900_e26086_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1742 != 0.0)) {
        let assign31900_e26084: f64 = (locals.var_t02 / locals.var_t22);
        (assign31900_e26084, (((locals.var_t02_dn3 * locals.var_t22) - (locals.var_t02 * locals.var_t22_dn3)) / (locals.var_t22 * locals.var_t22)), (((locals.var_t02_dn4 * locals.var_t22) - (locals.var_t02 * locals.var_t22_dn4)) / (locals.var_t22 * locals.var_t22)), (((locals.var_t02_dn5 * locals.var_t22) - (locals.var_t02 * locals.var_t22_dn5)) / (locals.var_t22 * locals.var_t22)), (((locals.var_t02_dn6 * locals.var_t22) - (locals.var_t02 * locals.var_t22_dn6)) / (locals.var_t22 * locals.var_t22)), (((locals.var_t02_dn7 * locals.var_t22) - (locals.var_t02 * locals.var_t22_dn7)) / (locals.var_t22 * locals.var_t22)), (((locals.var_t02_dn8 * locals.var_t22) - (locals.var_t02 * locals.var_t22_dn8)) / (locals.var_t22 * locals.var_t22)), (((locals.var_t02_dn9 * locals.var_t22) - (locals.var_t02 * locals.var_t22_dn9)) / (locals.var_t22 * locals.var_t22)), (((locals.var_t02_dn10 * locals.var_t22) - (locals.var_t02 * locals.var_t22_dn10)) / (locals.var_t22 * locals.var_t22)), (((locals.var_t02_dn11 * locals.var_t22) - (locals.var_t02 * locals.var_t22_dn11)) / (locals.var_t22 * locals.var_t22)), (((locals.var_t02_dn12 * locals.var_t22) - (locals.var_t02 * locals.var_t22_dn12)) / (locals.var_t22 * locals.var_t22)),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign31900_e26086;
        locals.var_t3__blk1147_dn3 = assign31900_e26086_d_n3;
        locals.var_t3__blk1147_dn4 = assign31900_e26086_d_n4;
        locals.var_t3__blk1147_dn5 = assign31900_e26086_d_n5;
        locals.var_t3__blk1147_dn6 = assign31900_e26086_d_n6;
        locals.var_t3__blk1147_dn7 = assign31900_e26086_d_n7;
        locals.var_t3__blk1147_dn8 = assign31900_e26086_d_n8;
        locals.var_t3__blk1147_dn9 = assign31900_e26086_d_n9;
        locals.var_t3__blk1147_dn10 = assign31900_e26086_d_n10;
        locals.var_t3__blk1147_dn11 = assign31900_e26086_d_n11;
        locals.var_t3__blk1147_dn12 = assign31900_e26086_d_n12;

        let (assign31910_e26103, assign31910_e26103_d_n3, assign31910_e26103_d_n4, assign31910_e26103_d_n5, assign31910_e26103_d_n6, assign31910_e26103_d_n7, assign31910_e26103_d_n8, assign31910_e26103_d_n9, assign31910_e26103_d_n10, assign31910_e26103_d_n11, assign31910_e26103_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1742 != 0.0)) {
        let assign31910_e26098: f64 = (0.5 - locals.var_t3__blk1147);
        let assign31910_e26099: f64 = (locals.var_t02 * assign31910_e26098);
        let assign31910_e26100: f64 = (locals.var_t12 - assign31910_e26099);
        let assign31910_e26101: f64 = (locals.var_coxwlcen2 * assign31910_e26100);
        (assign31910_e26101, ((locals.var_coxwlcen2_dn3 * assign31910_e26100) + (locals.var_coxwlcen2 * (locals.var_t12_dn3 - ((locals.var_t02_dn3 * assign31910_e26098) + (locals.var_t02 * (-locals.var_t3__blk1147_dn3)))))), ((locals.var_coxwlcen2_dn4 * assign31910_e26100) + (locals.var_coxwlcen2 * (locals.var_t12_dn4 - ((locals.var_t02_dn4 * assign31910_e26098) + (locals.var_t02 * (-locals.var_t3__blk1147_dn4)))))), ((locals.var_coxwlcen2_dn5 * assign31910_e26100) + (locals.var_coxwlcen2 * (locals.var_t12_dn5 - ((locals.var_t02_dn5 * assign31910_e26098) + (locals.var_t02 * (-locals.var_t3__blk1147_dn5)))))), ((locals.var_coxwlcen2_dn6 * assign31910_e26100) + (locals.var_coxwlcen2 * (locals.var_t12_dn6 - ((locals.var_t02_dn6 * assign31910_e26098) + (locals.var_t02 * (-locals.var_t3__blk1147_dn6)))))), ((locals.var_coxwlcen2_dn7 * assign31910_e26100) + (locals.var_coxwlcen2 * (locals.var_t12_dn7 - ((locals.var_t02_dn7 * assign31910_e26098) + (locals.var_t02 * (-locals.var_t3__blk1147_dn7)))))), ((locals.var_coxwlcen2_dn8 * assign31910_e26100) + (locals.var_coxwlcen2 * (locals.var_t12_dn8 - ((locals.var_t02_dn8 * assign31910_e26098) + (locals.var_t02 * (-locals.var_t3__blk1147_dn8)))))), ((locals.var_coxwlcen2_dn9 * assign31910_e26100) + (locals.var_coxwlcen2 * (locals.var_t12_dn9 - ((locals.var_t02_dn9 * assign31910_e26098) + (locals.var_t02 * (-locals.var_t3__blk1147_dn9)))))), ((locals.var_coxwlcen2_dn10 * assign31910_e26100) + (locals.var_coxwlcen2 * (locals.var_t12_dn10 - ((locals.var_t02_dn10 * assign31910_e26098) + (locals.var_t02 * (-locals.var_t3__blk1147_dn10)))))), ((locals.var_coxwlcen2_dn11 * assign31910_e26100) + (locals.var_coxwlcen2 * (locals.var_t12_dn11 - ((locals.var_t02_dn11 * assign31910_e26098) + (locals.var_t02 * (-locals.var_t3__blk1147_dn11)))))), ((locals.var_coxwlcen2_dn12 * assign31910_e26100) + (locals.var_coxwlcen2 * (locals.var_t12_dn12 - ((locals.var_t02_dn12 * assign31910_e26098) + (locals.var_t02 * (-locals.var_t3__blk1147_dn12)))))),)
    } else {
        (locals.var_t7__blk1151, locals.var_t7__blk1151_dn3, locals.var_t7__blk1151_dn4, locals.var_t7__blk1151_dn5, locals.var_t7__blk1151_dn6, locals.var_t7__blk1151_dn7, locals.var_t7__blk1151_dn8, locals.var_t7__blk1151_dn9, locals.var_t7__blk1151_dn10, locals.var_t7__blk1151_dn11, locals.var_t7__blk1151_dn12,)
    }
};
        locals.var_t7__blk1151 = assign31910_e26103;
        locals.var_t7__blk1151_dn3 = assign31910_e26103_d_n3;
        locals.var_t7__blk1151_dn4 = assign31910_e26103_d_n4;
        locals.var_t7__blk1151_dn5 = assign31910_e26103_d_n5;
        locals.var_t7__blk1151_dn6 = assign31910_e26103_d_n6;
        locals.var_t7__blk1151_dn7 = assign31910_e26103_d_n7;
        locals.var_t7__blk1151_dn8 = assign31910_e26103_d_n8;
        locals.var_t7__blk1151_dn9 = assign31910_e26103_d_n9;
        locals.var_t7__blk1151_dn10 = assign31910_e26103_d_n10;
        locals.var_t7__blk1151_dn11 = assign31910_e26103_d_n11;
        locals.var_t7__blk1151_dn12 = assign31910_e26103_d_n12;

        let (assign31920_e26114, assign31920_e26114_d_n3, assign31920_e26114_d_n4, assign31920_e26114_d_n5, assign31920_e26114_d_n6, assign31920_e26114_d_n7, assign31920_e26114_d_n8, assign31920_e26114_d_n9, assign31920_e26114_d_n10, assign31920_e26114_d_n11, assign31920_e26114_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1742 != 0.0)) {
        let assign31920_e26112: f64 = (locals.var_qinv + locals.var_t7__blk1151);
        (assign31920_e26112, (locals.var_qinv_dn3 + locals.var_t7__blk1151_dn3), (locals.var_qinv_dn4 + locals.var_t7__blk1151_dn4), (locals.var_qinv_dn5 + locals.var_t7__blk1151_dn5), (locals.var_qinv_dn6 + locals.var_t7__blk1151_dn6), (locals.var_qinv_dn7 + locals.var_t7__blk1151_dn7), (locals.var_qinv_dn8 + locals.var_t7__blk1151_dn8), (locals.var_qinv_dn9 + locals.var_t7__blk1151_dn9), (locals.var_qinv_dn10 + locals.var_t7__blk1151_dn10), (locals.var_qinv_dn11 + locals.var_t7__blk1151_dn11), (locals.var_qinv_dn12 + locals.var_t7__blk1151_dn12),)
    } else {
        (locals.var_qinv, locals.var_qinv_dn3, locals.var_qinv_dn4, locals.var_qinv_dn5, locals.var_qinv_dn6, locals.var_qinv_dn7, locals.var_qinv_dn8, locals.var_qinv_dn9, locals.var_qinv_dn10, locals.var_qinv_dn11, locals.var_qinv_dn12,)
    }
};
        locals.var_qinv = assign31920_e26114;
        locals.var_qinv_dn3 = assign31920_e26114_d_n3;
        locals.var_qinv_dn4 = assign31920_e26114_d_n4;
        locals.var_qinv_dn5 = assign31920_e26114_d_n5;
        locals.var_qinv_dn6 = assign31920_e26114_d_n6;
        locals.var_qinv_dn7 = assign31920_e26114_d_n7;
        locals.var_qinv_dn8 = assign31920_e26114_d_n8;
        locals.var_qinv_dn9 = assign31920_e26114_d_n9;
        locals.var_qinv_dn10 = assign31920_e26114_d_n10;
        locals.var_qinv_dn11 = assign31920_e26114_d_n11;
        locals.var_qinv_dn12 = assign31920_e26114_d_n12;

        let (assign31940_e26132, assign31940_e26132_d_n3, assign31940_e26132_d_n4, assign31940_e26132_d_n5, assign31940_e26132_d_n6, assign31940_e26132_d_n7, assign31940_e26132_d_n8, assign31940_e26132_d_n9, assign31940_e26132_d_n10, assign31940_e26132_d_n11, assign31940_e26132_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1742 != 0.0)) {
        (locals.var_qinv, locals.var_qinv_dn3, locals.var_qinv_dn4, locals.var_qinv_dn5, locals.var_qinv_dn6, locals.var_qinv_dn7, locals.var_qinv_dn8, locals.var_qinv_dn9, locals.var_qinv_dn10, locals.var_qinv_dn11, locals.var_qinv_dn12,)
    } else {
        (locals.var_qgate, locals.var_qgate_dn3, locals.var_qgate_dn4, locals.var_qgate_dn5, locals.var_qgate_dn6, locals.var_qgate_dn7, locals.var_qgate_dn8, locals.var_qgate_dn9, locals.var_qgate_dn10, locals.var_qgate_dn11, locals.var_qgate_dn12,)
    }
};
        locals.var_qgate = assign31940_e26132;
        locals.var_qgate_dn3 = assign31940_e26132_d_n3;
        locals.var_qgate_dn4 = assign31940_e26132_d_n4;
        locals.var_qgate_dn5 = assign31940_e26132_d_n5;
        locals.var_qgate_dn6 = assign31940_e26132_d_n6;
        locals.var_qgate_dn7 = assign31940_e26132_d_n7;
        locals.var_qgate_dn8 = assign31940_e26132_d_n8;
        locals.var_qgate_dn9 = assign31940_e26132_d_n9;
        locals.var_qgate_dn10 = assign31940_e26132_d_n10;
        locals.var_qgate_dn11 = assign31940_e26132_d_n11;
        locals.var_qgate_dn12 = assign31940_e26132_d_n12;

        let assign31950_e26135: f64 = if locals.var_b4soisoimod == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1743 = assign31950_e26135;

        let (assign31960_e26144, assign31960_e26144_d_n3, assign31960_e26144_d_n4, assign31960_e26144_d_n5, assign31960_e26144_d_n6, assign31960_e26144_d_n7, assign31960_e26144_d_n8, assign31960_e26144_d_n9, assign31960_e26144_d_n10, assign31960_e26144_d_n11, assign31960_e26144_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1743 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbulk, locals.var_qbulk_dn3, locals.var_qbulk_dn4, locals.var_qbulk_dn5, locals.var_qbulk_dn6, locals.var_qbulk_dn7, locals.var_qbulk_dn8, locals.var_qbulk_dn9, locals.var_qbulk_dn10, locals.var_qbulk_dn11, locals.var_qbulk_dn12,)
    }
};
        locals.var_qbulk = assign31960_e26144;
        locals.var_qbulk_dn3 = assign31960_e26144_d_n3;
        locals.var_qbulk_dn4 = assign31960_e26144_d_n4;
        locals.var_qbulk_dn5 = assign31960_e26144_d_n5;
        locals.var_qbulk_dn6 = assign31960_e26144_d_n6;
        locals.var_qbulk_dn7 = assign31960_e26144_d_n7;
        locals.var_qbulk_dn8 = assign31960_e26144_d_n8;
        locals.var_qbulk_dn9 = assign31960_e26144_d_n9;
        locals.var_qbulk_dn10 = assign31960_e26144_d_n10;
        locals.var_qbulk_dn11 = assign31960_e26144_d_n11;
        locals.var_qbulk_dn12 = assign31960_e26144_d_n12;

        let (assign31970_e26156, assign31970_e26156_d_n3, assign31970_e26156_d_n4, assign31970_e26156_d_n5, assign31970_e26156_d_n6, assign31970_e26156_d_n7, assign31970_e26156_d_n8, assign31970_e26156_d_n9, assign31970_e26156_d_n10, assign31970_e26156_d_n11, assign31970_e26156_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1743 == 0.0)) {
        let assign31970_e26154: f64 = (1.0 - locals.var_abulkcv);
        (assign31970_e26154, (-locals.var_abulkcv_dn3), (-locals.var_abulkcv_dn4), (-locals.var_abulkcv_dn5), (-locals.var_abulkcv_dn6), (-locals.var_abulkcv_dn7), (-locals.var_abulkcv_dn8), (-locals.var_abulkcv_dn9), (-locals.var_abulkcv_dn10), (-locals.var_abulkcv_dn11), (-locals.var_abulkcv_dn12),)
    } else {
        (locals.var_t7__blk1151, locals.var_t7__blk1151_dn3, locals.var_t7__blk1151_dn4, locals.var_t7__blk1151_dn5, locals.var_t7__blk1151_dn6, locals.var_t7__blk1151_dn7, locals.var_t7__blk1151_dn8, locals.var_t7__blk1151_dn9, locals.var_t7__blk1151_dn10, locals.var_t7__blk1151_dn11, locals.var_t7__blk1151_dn12,)
    }
};
        locals.var_t7__blk1151 = assign31970_e26156;
        locals.var_t7__blk1151_dn3 = assign31970_e26156_d_n3;
        locals.var_t7__blk1151_dn4 = assign31970_e26156_d_n4;
        locals.var_t7__blk1151_dn5 = assign31970_e26156_d_n5;
        locals.var_t7__blk1151_dn6 = assign31970_e26156_d_n6;
        locals.var_t7__blk1151_dn7 = assign31970_e26156_d_n7;
        locals.var_t7__blk1151_dn8 = assign31970_e26156_d_n8;
        locals.var_t7__blk1151_dn9 = assign31970_e26156_d_n9;
        locals.var_t7__blk1151_dn10 = assign31970_e26156_d_n10;
        locals.var_t7__blk1151_dn11 = assign31970_e26156_d_n11;
        locals.var_t7__blk1151_dn12 = assign31970_e26156_d_n12;

        let (assign31980_e26178, assign31980_e26178_d_n3, assign31980_e26178_d_n4, assign31980_e26178_d_n5, assign31980_e26178_d_n6, assign31980_e26178_d_n7, assign31980_e26178_d_n8, assign31980_e26178_d_n9, assign31980_e26178_d_n10, assign31980_e26178_d_n11, assign31980_e26178_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1743 == 0.0)) {
        let assign31980_e26166: f64 = (locals.var_coxwlcenb * locals.var_t7__blk1151);
        let assign31980_e26169: f64 = (0.5 * locals.var_vdseffcv);
        let assign31980_e26172: f64 = (locals.var_t0__blk1144 * locals.var_vdseffcv);
        let assign31980_e26174: f64 = (assign31980_e26172 / locals.var_t2__blk1146);
        let assign31980_e26175: f64 = (assign31980_e26169 - assign31980_e26174);
        let assign31980_e26176: f64 = (assign31980_e26166 * assign31980_e26175);
        (assign31980_e26176, ((((locals.var_coxwlcenb_dn3 * locals.var_t7__blk1151) + (locals.var_coxwlcenb * locals.var_t7__blk1151_dn3)) * assign31980_e26175) + (assign31980_e26166 * ((0.5 * locals.var_vdseffcv_dn3) - (((((locals.var_t0__blk1144_dn3 * locals.var_vdseffcv) + (locals.var_t0__blk1144 * locals.var_vdseffcv_dn3)) * locals.var_t2__blk1146) - (assign31980_e26172 * locals.var_t2__blk1146_dn3)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146))))), ((((locals.var_coxwlcenb_dn4 * locals.var_t7__blk1151) + (locals.var_coxwlcenb * locals.var_t7__blk1151_dn4)) * assign31980_e26175) + (assign31980_e26166 * ((0.5 * locals.var_vdseffcv_dn4) - (((((locals.var_t0__blk1144_dn4 * locals.var_vdseffcv) + (locals.var_t0__blk1144 * locals.var_vdseffcv_dn4)) * locals.var_t2__blk1146) - (assign31980_e26172 * locals.var_t2__blk1146_dn4)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146))))), ((((locals.var_coxwlcenb_dn5 * locals.var_t7__blk1151) + (locals.var_coxwlcenb * locals.var_t7__blk1151_dn5)) * assign31980_e26175) + (assign31980_e26166 * ((0.5 * locals.var_vdseffcv_dn5) - (((((locals.var_t0__blk1144_dn5 * locals.var_vdseffcv) + (locals.var_t0__blk1144 * locals.var_vdseffcv_dn5)) * locals.var_t2__blk1146) - (assign31980_e26172 * locals.var_t2__blk1146_dn5)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146))))), ((((locals.var_coxwlcenb_dn6 * locals.var_t7__blk1151) + (locals.var_coxwlcenb * locals.var_t7__blk1151_dn6)) * assign31980_e26175) + (assign31980_e26166 * ((0.5 * locals.var_vdseffcv_dn6) - (((((locals.var_t0__blk1144_dn6 * locals.var_vdseffcv) + (locals.var_t0__blk1144 * locals.var_vdseffcv_dn6)) * locals.var_t2__blk1146) - (assign31980_e26172 * locals.var_t2__blk1146_dn6)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146))))), ((((locals.var_coxwlcenb_dn7 * locals.var_t7__blk1151) + (locals.var_coxwlcenb * locals.var_t7__blk1151_dn7)) * assign31980_e26175) + (assign31980_e26166 * ((0.5 * locals.var_vdseffcv_dn7) - (((((locals.var_t0__blk1144_dn7 * locals.var_vdseffcv) + (locals.var_t0__blk1144 * locals.var_vdseffcv_dn7)) * locals.var_t2__blk1146) - (assign31980_e26172 * locals.var_t2__blk1146_dn7)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146))))), ((((locals.var_coxwlcenb_dn8 * locals.var_t7__blk1151) + (locals.var_coxwlcenb * locals.var_t7__blk1151_dn8)) * assign31980_e26175) + (assign31980_e26166 * ((0.5 * locals.var_vdseffcv_dn8) - (((((locals.var_t0__blk1144_dn8 * locals.var_vdseffcv) + (locals.var_t0__blk1144 * locals.var_vdseffcv_dn8)) * locals.var_t2__blk1146) - (assign31980_e26172 * locals.var_t2__blk1146_dn8)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146))))), ((((locals.var_coxwlcenb_dn9 * locals.var_t7__blk1151) + (locals.var_coxwlcenb * locals.var_t7__blk1151_dn9)) * assign31980_e26175) + (assign31980_e26166 * ((0.5 * locals.var_vdseffcv_dn9) - (((((locals.var_t0__blk1144_dn9 * locals.var_vdseffcv) + (locals.var_t0__blk1144 * locals.var_vdseffcv_dn9)) * locals.var_t2__blk1146) - (assign31980_e26172 * locals.var_t2__blk1146_dn9)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146))))), ((((locals.var_coxwlcenb_dn10 * locals.var_t7__blk1151) + (locals.var_coxwlcenb * locals.var_t7__blk1151_dn10)) * assign31980_e26175) + (assign31980_e26166 * ((0.5 * locals.var_vdseffcv_dn10) - (((((locals.var_t0__blk1144_dn10 * locals.var_vdseffcv) + (locals.var_t0__blk1144 * locals.var_vdseffcv_dn10)) * locals.var_t2__blk1146) - (assign31980_e26172 * locals.var_t2__blk1146_dn10)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146))))), ((((locals.var_coxwlcenb_dn11 * locals.var_t7__blk1151) + (locals.var_coxwlcenb * locals.var_t7__blk1151_dn11)) * assign31980_e26175) + (assign31980_e26166 * ((0.5 * locals.var_vdseffcv_dn11) - (((((locals.var_t0__blk1144_dn11 * locals.var_vdseffcv) + (locals.var_t0__blk1144 * locals.var_vdseffcv_dn11)) * locals.var_t2__blk1146) - (assign31980_e26172 * locals.var_t2__blk1146_dn11)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146))))), ((((locals.var_coxwlcenb_dn12 * locals.var_t7__blk1151) + (locals.var_coxwlcenb * locals.var_t7__blk1151_dn12)) * assign31980_e26175) + (assign31980_e26166 * ((0.5 * locals.var_vdseffcv_dn12) - (((((locals.var_t0__blk1144_dn12 * locals.var_vdseffcv) + (locals.var_t0__blk1144 * locals.var_vdseffcv_dn12)) * locals.var_t2__blk1146) - (assign31980_e26172 * locals.var_t2__blk1146_dn12)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146))))),)
    } else {
        (locals.var_qbulk, locals.var_qbulk_dn3, locals.var_qbulk_dn4, locals.var_qbulk_dn5, locals.var_qbulk_dn6, locals.var_qbulk_dn7, locals.var_qbulk_dn8, locals.var_qbulk_dn9, locals.var_qbulk_dn10, locals.var_qbulk_dn11, locals.var_qbulk_dn12,)
    }
};
        locals.var_qbulk = assign31980_e26178;
        locals.var_qbulk_dn3 = assign31980_e26178_d_n3;
        locals.var_qbulk_dn4 = assign31980_e26178_d_n4;
        locals.var_qbulk_dn5 = assign31980_e26178_d_n5;
        locals.var_qbulk_dn6 = assign31980_e26178_d_n6;
        locals.var_qbulk_dn7 = assign31980_e26178_d_n7;
        locals.var_qbulk_dn8 = assign31980_e26178_d_n8;
        locals.var_qbulk_dn9 = assign31980_e26178_d_n9;
        locals.var_qbulk_dn10 = assign31980_e26178_d_n10;
        locals.var_qbulk_dn11 = assign31980_e26178_d_n11;
        locals.var_qbulk_dn12 = assign31980_e26178_d_n12;

        let assign31990_e26189: f64 = if (((locals.var_b4soisoimod != 2.0) && (locals.var_b4soibodymod != 0.0)) && (locals.var_b4soiagbcp2 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1744 = assign31990_e26189;

        let (assign32000_e26213, assign32000_e26213_d_n3, assign32000_e26213_d_n4, assign32000_e26213_d_n5, assign32000_e26213_d_n6, assign32000_e26213_d_n7, assign32000_e26213_d_n8, assign32000_e26213_d_n9, assign32000_e26213_d_n10, assign32000_e26213_d_n11, assign32000_e26213_d_n12,) = {
    if ((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1743 == 0.0)) && (locals.var_guard1744 != 0.0)) {
        let assign32000_e26201: f64 = (locals.var_coxwlcenb2 * locals.var_t7__blk1151);
        let assign32000_e26204: f64 = (0.5 * locals.var_vdseffcv2);
        let assign32000_e26207: f64 = (locals.var_t02 * locals.var_vdseffcv2);
        let assign32000_e26209: f64 = (assign32000_e26207 / locals.var_t22);
        let assign32000_e26210: f64 = (assign32000_e26204 - assign32000_e26209);
        let assign32000_e26211: f64 = (assign32000_e26201 * assign32000_e26210);
        (assign32000_e26211, ((((locals.var_coxwlcenb2_dn3 * locals.var_t7__blk1151) + (locals.var_coxwlcenb2 * locals.var_t7__blk1151_dn3)) * assign32000_e26210) + (assign32000_e26201 * ((0.5 * locals.var_vdseffcv2_dn3) - (((((locals.var_t02_dn3 * locals.var_vdseffcv2) + (locals.var_t02 * locals.var_vdseffcv2_dn3)) * locals.var_t22) - (assign32000_e26207 * locals.var_t22_dn3)) / (locals.var_t22 * locals.var_t22))))), ((((locals.var_coxwlcenb2_dn4 * locals.var_t7__blk1151) + (locals.var_coxwlcenb2 * locals.var_t7__blk1151_dn4)) * assign32000_e26210) + (assign32000_e26201 * ((0.5 * locals.var_vdseffcv2_dn4) - (((((locals.var_t02_dn4 * locals.var_vdseffcv2) + (locals.var_t02 * locals.var_vdseffcv2_dn4)) * locals.var_t22) - (assign32000_e26207 * locals.var_t22_dn4)) / (locals.var_t22 * locals.var_t22))))), ((((locals.var_coxwlcenb2_dn5 * locals.var_t7__blk1151) + (locals.var_coxwlcenb2 * locals.var_t7__blk1151_dn5)) * assign32000_e26210) + (assign32000_e26201 * ((0.5 * locals.var_vdseffcv2_dn5) - (((((locals.var_t02_dn5 * locals.var_vdseffcv2) + (locals.var_t02 * locals.var_vdseffcv2_dn5)) * locals.var_t22) - (assign32000_e26207 * locals.var_t22_dn5)) / (locals.var_t22 * locals.var_t22))))), ((((locals.var_coxwlcenb2_dn6 * locals.var_t7__blk1151) + (locals.var_coxwlcenb2 * locals.var_t7__blk1151_dn6)) * assign32000_e26210) + (assign32000_e26201 * ((0.5 * locals.var_vdseffcv2_dn6) - (((((locals.var_t02_dn6 * locals.var_vdseffcv2) + (locals.var_t02 * locals.var_vdseffcv2_dn6)) * locals.var_t22) - (assign32000_e26207 * locals.var_t22_dn6)) / (locals.var_t22 * locals.var_t22))))), ((((locals.var_coxwlcenb2_dn7 * locals.var_t7__blk1151) + (locals.var_coxwlcenb2 * locals.var_t7__blk1151_dn7)) * assign32000_e26210) + (assign32000_e26201 * ((0.5 * locals.var_vdseffcv2_dn7) - (((((locals.var_t02_dn7 * locals.var_vdseffcv2) + (locals.var_t02 * locals.var_vdseffcv2_dn7)) * locals.var_t22) - (assign32000_e26207 * locals.var_t22_dn7)) / (locals.var_t22 * locals.var_t22))))), ((((locals.var_coxwlcenb2_dn8 * locals.var_t7__blk1151) + (locals.var_coxwlcenb2 * locals.var_t7__blk1151_dn8)) * assign32000_e26210) + (assign32000_e26201 * ((0.5 * locals.var_vdseffcv2_dn8) - (((((locals.var_t02_dn8 * locals.var_vdseffcv2) + (locals.var_t02 * locals.var_vdseffcv2_dn8)) * locals.var_t22) - (assign32000_e26207 * locals.var_t22_dn8)) / (locals.var_t22 * locals.var_t22))))), ((((locals.var_coxwlcenb2_dn9 * locals.var_t7__blk1151) + (locals.var_coxwlcenb2 * locals.var_t7__blk1151_dn9)) * assign32000_e26210) + (assign32000_e26201 * ((0.5 * locals.var_vdseffcv2_dn9) - (((((locals.var_t02_dn9 * locals.var_vdseffcv2) + (locals.var_t02 * locals.var_vdseffcv2_dn9)) * locals.var_t22) - (assign32000_e26207 * locals.var_t22_dn9)) / (locals.var_t22 * locals.var_t22))))), ((((locals.var_coxwlcenb2_dn10 * locals.var_t7__blk1151) + (locals.var_coxwlcenb2 * locals.var_t7__blk1151_dn10)) * assign32000_e26210) + (assign32000_e26201 * ((0.5 * locals.var_vdseffcv2_dn10) - (((((locals.var_t02_dn10 * locals.var_vdseffcv2) + (locals.var_t02 * locals.var_vdseffcv2_dn10)) * locals.var_t22) - (assign32000_e26207 * locals.var_t22_dn10)) / (locals.var_t22 * locals.var_t22))))), ((((locals.var_coxwlcenb2_dn11 * locals.var_t7__blk1151) + (locals.var_coxwlcenb2 * locals.var_t7__blk1151_dn11)) * assign32000_e26210) + (assign32000_e26201 * ((0.5 * locals.var_vdseffcv2_dn11) - (((((locals.var_t02_dn11 * locals.var_vdseffcv2) + (locals.var_t02 * locals.var_vdseffcv2_dn11)) * locals.var_t22) - (assign32000_e26207 * locals.var_t22_dn11)) / (locals.var_t22 * locals.var_t22))))), ((((locals.var_coxwlcenb2_dn12 * locals.var_t7__blk1151) + (locals.var_coxwlcenb2 * locals.var_t7__blk1151_dn12)) * assign32000_e26210) + (assign32000_e26201 * ((0.5 * locals.var_vdseffcv2_dn12) - (((((locals.var_t02_dn12 * locals.var_vdseffcv2) + (locals.var_t02 * locals.var_vdseffcv2_dn12)) * locals.var_t22) - (assign32000_e26207 * locals.var_t22_dn12)) / (locals.var_t22 * locals.var_t22))))),)
    } else {
        (locals.var_qbulk2, locals.var_qbulk2_dn3, locals.var_qbulk2_dn4, locals.var_qbulk2_dn5, locals.var_qbulk2_dn6, locals.var_qbulk2_dn7, locals.var_qbulk2_dn8, locals.var_qbulk2_dn9, locals.var_qbulk2_dn10, locals.var_qbulk2_dn11, locals.var_qbulk2_dn12,)
    }
};
        locals.var_qbulk2 = assign32000_e26213;
        locals.var_qbulk2_dn3 = assign32000_e26213_d_n3;
        locals.var_qbulk2_dn4 = assign32000_e26213_d_n4;
        locals.var_qbulk2_dn5 = assign32000_e26213_d_n5;
        locals.var_qbulk2_dn6 = assign32000_e26213_d_n6;
        locals.var_qbulk2_dn7 = assign32000_e26213_d_n7;
        locals.var_qbulk2_dn8 = assign32000_e26213_d_n8;
        locals.var_qbulk2_dn9 = assign32000_e26213_d_n9;
        locals.var_qbulk2_dn10 = assign32000_e26213_d_n10;
        locals.var_qbulk2_dn11 = assign32000_e26213_d_n11;
        locals.var_qbulk2_dn12 = assign32000_e26213_d_n12;

        let (assign32010_e26227, assign32010_e26227_d_n3, assign32010_e26227_d_n4, assign32010_e26227_d_n5, assign32010_e26227_d_n6, assign32010_e26227_d_n7, assign32010_e26227_d_n8, assign32010_e26227_d_n9, assign32010_e26227_d_n10, assign32010_e26227_d_n11, assign32010_e26227_d_n12,) = {
    if ((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1743 == 0.0)) && (locals.var_guard1744 != 0.0)) {
        let assign32010_e26225: f64 = (locals.var_qbulk + locals.var_qbulk2);
        (assign32010_e26225, (locals.var_qbulk_dn3 + locals.var_qbulk2_dn3), (locals.var_qbulk_dn4 + locals.var_qbulk2_dn4), (locals.var_qbulk_dn5 + locals.var_qbulk2_dn5), (locals.var_qbulk_dn6 + locals.var_qbulk2_dn6), (locals.var_qbulk_dn7 + locals.var_qbulk2_dn7), (locals.var_qbulk_dn8 + locals.var_qbulk2_dn8), (locals.var_qbulk_dn9 + locals.var_qbulk2_dn9), (locals.var_qbulk_dn10 + locals.var_qbulk2_dn10), (locals.var_qbulk_dn11 + locals.var_qbulk2_dn11), (locals.var_qbulk_dn12 + locals.var_qbulk2_dn12),)
    } else {
        (locals.var_qbulk, locals.var_qbulk_dn3, locals.var_qbulk_dn4, locals.var_qbulk_dn5, locals.var_qbulk_dn6, locals.var_qbulk_dn7, locals.var_qbulk_dn8, locals.var_qbulk_dn9, locals.var_qbulk_dn10, locals.var_qbulk_dn11, locals.var_qbulk_dn12,)
    }
};
        locals.var_qbulk = assign32010_e26227;
        locals.var_qbulk_dn3 = assign32010_e26227_d_n3;
        locals.var_qbulk_dn4 = assign32010_e26227_d_n4;
        locals.var_qbulk_dn5 = assign32010_e26227_d_n5;
        locals.var_qbulk_dn6 = assign32010_e26227_d_n6;
        locals.var_qbulk_dn7 = assign32010_e26227_d_n7;
        locals.var_qbulk_dn8 = assign32010_e26227_d_n8;
        locals.var_qbulk_dn9 = assign32010_e26227_d_n9;
        locals.var_qbulk_dn10 = assign32010_e26227_d_n10;
        locals.var_qbulk_dn11 = assign32010_e26227_d_n11;
        locals.var_qbulk_dn12 = assign32010_e26227_d_n12;

        let assign32020_e26230: f64 = if locals.var_b4soixpart > 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1745 = assign32020_e26230;

    }

    pub(super) fn stamp_transient_block_88(
        locals: &mut StampLocals,
    ) {
        let (assign32030_e26256, assign32030_e26256_d_n3, assign32030_e26256_d_n4, assign32030_e26256_d_n5, assign32030_e26256_d_n6, assign32030_e26256_d_n7, assign32030_e26256_d_n8, assign32030_e26256_d_n9, assign32030_e26256_d_n10, assign32030_e26256_d_n11, assign32030_e26256_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1745 != 0.0)) {
        let assign32030_e26238: f64 = (-locals.var_coxwlcen);
        let assign32030_e26241: f64 = (locals.var_t1__blk1145 / 2.0);
        let assign32030_e26244: f64 = (locals.var_t0__blk1144 / 4.0);
        let assign32030_e26245: f64 = (assign32030_e26241 + assign32030_e26244);
        let assign32030_e26248: f64 = (0.5 * locals.var_t0__blk1144);
        let assign32030_e26250: f64 = (assign32030_e26248 * locals.var_t0__blk1144);
        let assign32030_e26252: f64 = (assign32030_e26250 / locals.var_t2__blk1146);
        let assign32030_e26253: f64 = (assign32030_e26245 - assign32030_e26252);
        let assign32030_e26254: f64 = (assign32030_e26238 * assign32030_e26253);
        (assign32030_e26254, (((-locals.var_coxwlcen_dn3) * assign32030_e26253) + (assign32030_e26238 * (((locals.var_t1__blk1145_dn3 / 2.0) + (locals.var_t0__blk1144_dn3 / 4.0)) - ((((((0.5 * locals.var_t0__blk1144_dn3) * locals.var_t0__blk1144) + (assign32030_e26248 * locals.var_t0__blk1144_dn3)) * locals.var_t2__blk1146) - (assign32030_e26250 * locals.var_t2__blk1146_dn3)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146))))), (((-locals.var_coxwlcen_dn4) * assign32030_e26253) + (assign32030_e26238 * (((locals.var_t1__blk1145_dn4 / 2.0) + (locals.var_t0__blk1144_dn4 / 4.0)) - ((((((0.5 * locals.var_t0__blk1144_dn4) * locals.var_t0__blk1144) + (assign32030_e26248 * locals.var_t0__blk1144_dn4)) * locals.var_t2__blk1146) - (assign32030_e26250 * locals.var_t2__blk1146_dn4)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146))))), (((-locals.var_coxwlcen_dn5) * assign32030_e26253) + (assign32030_e26238 * (((locals.var_t1__blk1145_dn5 / 2.0) + (locals.var_t0__blk1144_dn5 / 4.0)) - ((((((0.5 * locals.var_t0__blk1144_dn5) * locals.var_t0__blk1144) + (assign32030_e26248 * locals.var_t0__blk1144_dn5)) * locals.var_t2__blk1146) - (assign32030_e26250 * locals.var_t2__blk1146_dn5)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146))))), (((-locals.var_coxwlcen_dn6) * assign32030_e26253) + (assign32030_e26238 * (((locals.var_t1__blk1145_dn6 / 2.0) + (locals.var_t0__blk1144_dn6 / 4.0)) - ((((((0.5 * locals.var_t0__blk1144_dn6) * locals.var_t0__blk1144) + (assign32030_e26248 * locals.var_t0__blk1144_dn6)) * locals.var_t2__blk1146) - (assign32030_e26250 * locals.var_t2__blk1146_dn6)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146))))), (((-locals.var_coxwlcen_dn7) * assign32030_e26253) + (assign32030_e26238 * (((locals.var_t1__blk1145_dn7 / 2.0) + (locals.var_t0__blk1144_dn7 / 4.0)) - ((((((0.5 * locals.var_t0__blk1144_dn7) * locals.var_t0__blk1144) + (assign32030_e26248 * locals.var_t0__blk1144_dn7)) * locals.var_t2__blk1146) - (assign32030_e26250 * locals.var_t2__blk1146_dn7)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146))))), (((-locals.var_coxwlcen_dn8) * assign32030_e26253) + (assign32030_e26238 * (((locals.var_t1__blk1145_dn8 / 2.0) + (locals.var_t0__blk1144_dn8 / 4.0)) - ((((((0.5 * locals.var_t0__blk1144_dn8) * locals.var_t0__blk1144) + (assign32030_e26248 * locals.var_t0__blk1144_dn8)) * locals.var_t2__blk1146) - (assign32030_e26250 * locals.var_t2__blk1146_dn8)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146))))), (((-locals.var_coxwlcen_dn9) * assign32030_e26253) + (assign32030_e26238 * (((locals.var_t1__blk1145_dn9 / 2.0) + (locals.var_t0__blk1144_dn9 / 4.0)) - ((((((0.5 * locals.var_t0__blk1144_dn9) * locals.var_t0__blk1144) + (assign32030_e26248 * locals.var_t0__blk1144_dn9)) * locals.var_t2__blk1146) - (assign32030_e26250 * locals.var_t2__blk1146_dn9)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146))))), (((-locals.var_coxwlcen_dn10) * assign32030_e26253) + (assign32030_e26238 * (((locals.var_t1__blk1145_dn10 / 2.0) + (locals.var_t0__blk1144_dn10 / 4.0)) - ((((((0.5 * locals.var_t0__blk1144_dn10) * locals.var_t0__blk1144) + (assign32030_e26248 * locals.var_t0__blk1144_dn10)) * locals.var_t2__blk1146) - (assign32030_e26250 * locals.var_t2__blk1146_dn10)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146))))), (((-locals.var_coxwlcen_dn11) * assign32030_e26253) + (assign32030_e26238 * (((locals.var_t1__blk1145_dn11 / 2.0) + (locals.var_t0__blk1144_dn11 / 4.0)) - ((((((0.5 * locals.var_t0__blk1144_dn11) * locals.var_t0__blk1144) + (assign32030_e26248 * locals.var_t0__blk1144_dn11)) * locals.var_t2__blk1146) - (assign32030_e26250 * locals.var_t2__blk1146_dn11)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146))))), (((-locals.var_coxwlcen_dn12) * assign32030_e26253) + (assign32030_e26238 * (((locals.var_t1__blk1145_dn12 / 2.0) + (locals.var_t0__blk1144_dn12 / 4.0)) - ((((((0.5 * locals.var_t0__blk1144_dn12) * locals.var_t0__blk1144) + (assign32030_e26248 * locals.var_t0__blk1144_dn12)) * locals.var_t2__blk1146) - (assign32030_e26250 * locals.var_t2__blk1146_dn12)) / (locals.var_t2__blk1146 * locals.var_t2__blk1146))))),)
    } else {
        (locals.var_qsrc, locals.var_qsrc_dn3, locals.var_qsrc_dn4, locals.var_qsrc_dn5, locals.var_qsrc_dn6, locals.var_qsrc_dn7, locals.var_qsrc_dn8, locals.var_qsrc_dn9, locals.var_qsrc_dn10, locals.var_qsrc_dn11, locals.var_qsrc_dn12,)
    }
};
        locals.var_qsrc = assign32030_e26256;
        locals.var_qsrc_dn3 = assign32030_e26256_d_n3;
        locals.var_qsrc_dn4 = assign32030_e26256_d_n4;
        locals.var_qsrc_dn5 = assign32030_e26256_d_n5;
        locals.var_qsrc_dn6 = assign32030_e26256_d_n6;
        locals.var_qsrc_dn7 = assign32030_e26256_d_n7;
        locals.var_qsrc_dn8 = assign32030_e26256_d_n8;
        locals.var_qsrc_dn9 = assign32030_e26256_d_n9;
        locals.var_qsrc_dn10 = assign32030_e26256_d_n10;
        locals.var_qsrc_dn11 = assign32030_e26256_d_n11;
        locals.var_qsrc_dn12 = assign32030_e26256_d_n12;

        let assign32040_e26267: f64 = if (((locals.var_b4soisoimod != 2.0) && (locals.var_b4soibodymod != 0.0)) && (locals.var_b4soiagbcp2 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1746 = assign32040_e26267;

        let (assign32050_e26297, assign32050_e26297_d_n3, assign32050_e26297_d_n4, assign32050_e26297_d_n5, assign32050_e26297_d_n6, assign32050_e26297_d_n7, assign32050_e26297_d_n8, assign32050_e26297_d_n9, assign32050_e26297_d_n10, assign32050_e26297_d_n11, assign32050_e26297_d_n12,) = {
    if ((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1745 != 0.0)) && (locals.var_guard1746 != 0.0)) {
        let assign32050_e26277: f64 = (-locals.var_coxwlcen2);
        let assign32050_e26280: f64 = (locals.var_vgsteff2 - locals.var_deltaphi2);
        let assign32050_e26282: f64 = (assign32050_e26280 / 2.0);
        let assign32050_e26285: f64 = (locals.var_t02 / 4.0);
        let assign32050_e26286: f64 = (assign32050_e26282 + assign32050_e26285);
        let assign32050_e26289: f64 = (0.5 * locals.var_t02);
        let assign32050_e26291: f64 = (assign32050_e26289 * locals.var_t02);
        let assign32050_e26293: f64 = (assign32050_e26291 / locals.var_t22);
        let assign32050_e26294: f64 = (assign32050_e26286 - assign32050_e26293);
        let assign32050_e26295: f64 = (assign32050_e26277 * assign32050_e26294);
        (assign32050_e26295, (((-locals.var_coxwlcen2_dn3) * assign32050_e26294) + (assign32050_e26277 * ((((locals.var_vgsteff2_dn3 - locals.var_deltaphi2_dn3) / 2.0) + (locals.var_t02_dn3 / 4.0)) - ((((((0.5 * locals.var_t02_dn3) * locals.var_t02) + (assign32050_e26289 * locals.var_t02_dn3)) * locals.var_t22) - (assign32050_e26291 * locals.var_t22_dn3)) / (locals.var_t22 * locals.var_t22))))), (((-locals.var_coxwlcen2_dn4) * assign32050_e26294) + (assign32050_e26277 * ((((locals.var_vgsteff2_dn4 - locals.var_deltaphi2_dn4) / 2.0) + (locals.var_t02_dn4 / 4.0)) - ((((((0.5 * locals.var_t02_dn4) * locals.var_t02) + (assign32050_e26289 * locals.var_t02_dn4)) * locals.var_t22) - (assign32050_e26291 * locals.var_t22_dn4)) / (locals.var_t22 * locals.var_t22))))), (((-locals.var_coxwlcen2_dn5) * assign32050_e26294) + (assign32050_e26277 * ((((locals.var_vgsteff2_dn5 - locals.var_deltaphi2_dn5) / 2.0) + (locals.var_t02_dn5 / 4.0)) - ((((((0.5 * locals.var_t02_dn5) * locals.var_t02) + (assign32050_e26289 * locals.var_t02_dn5)) * locals.var_t22) - (assign32050_e26291 * locals.var_t22_dn5)) / (locals.var_t22 * locals.var_t22))))), (((-locals.var_coxwlcen2_dn6) * assign32050_e26294) + (assign32050_e26277 * ((((locals.var_vgsteff2_dn6 - locals.var_deltaphi2_dn6) / 2.0) + (locals.var_t02_dn6 / 4.0)) - ((((((0.5 * locals.var_t02_dn6) * locals.var_t02) + (assign32050_e26289 * locals.var_t02_dn6)) * locals.var_t22) - (assign32050_e26291 * locals.var_t22_dn6)) / (locals.var_t22 * locals.var_t22))))), (((-locals.var_coxwlcen2_dn7) * assign32050_e26294) + (assign32050_e26277 * ((((locals.var_vgsteff2_dn7 - locals.var_deltaphi2_dn7) / 2.0) + (locals.var_t02_dn7 / 4.0)) - ((((((0.5 * locals.var_t02_dn7) * locals.var_t02) + (assign32050_e26289 * locals.var_t02_dn7)) * locals.var_t22) - (assign32050_e26291 * locals.var_t22_dn7)) / (locals.var_t22 * locals.var_t22))))), (((-locals.var_coxwlcen2_dn8) * assign32050_e26294) + (assign32050_e26277 * ((((locals.var_vgsteff2_dn8 - locals.var_deltaphi2_dn8) / 2.0) + (locals.var_t02_dn8 / 4.0)) - ((((((0.5 * locals.var_t02_dn8) * locals.var_t02) + (assign32050_e26289 * locals.var_t02_dn8)) * locals.var_t22) - (assign32050_e26291 * locals.var_t22_dn8)) / (locals.var_t22 * locals.var_t22))))), (((-locals.var_coxwlcen2_dn9) * assign32050_e26294) + (assign32050_e26277 * ((((locals.var_vgsteff2_dn9 - locals.var_deltaphi2_dn9) / 2.0) + (locals.var_t02_dn9 / 4.0)) - ((((((0.5 * locals.var_t02_dn9) * locals.var_t02) + (assign32050_e26289 * locals.var_t02_dn9)) * locals.var_t22) - (assign32050_e26291 * locals.var_t22_dn9)) / (locals.var_t22 * locals.var_t22))))), (((-locals.var_coxwlcen2_dn10) * assign32050_e26294) + (assign32050_e26277 * ((((locals.var_vgsteff2_dn10 - locals.var_deltaphi2_dn10) / 2.0) + (locals.var_t02_dn10 / 4.0)) - ((((((0.5 * locals.var_t02_dn10) * locals.var_t02) + (assign32050_e26289 * locals.var_t02_dn10)) * locals.var_t22) - (assign32050_e26291 * locals.var_t22_dn10)) / (locals.var_t22 * locals.var_t22))))), (((-locals.var_coxwlcen2_dn11) * assign32050_e26294) + (assign32050_e26277 * ((((locals.var_vgsteff2_dn11 - locals.var_deltaphi2_dn11) / 2.0) + (locals.var_t02_dn11 / 4.0)) - ((((((0.5 * locals.var_t02_dn11) * locals.var_t02) + (assign32050_e26289 * locals.var_t02_dn11)) * locals.var_t22) - (assign32050_e26291 * locals.var_t22_dn11)) / (locals.var_t22 * locals.var_t22))))), (((-locals.var_coxwlcen2_dn12) * assign32050_e26294) + (assign32050_e26277 * ((((locals.var_vgsteff2_dn12 - locals.var_deltaphi2_dn12) / 2.0) + (locals.var_t02_dn12 / 4.0)) - ((((((0.5 * locals.var_t02_dn12) * locals.var_t02) + (assign32050_e26289 * locals.var_t02_dn12)) * locals.var_t22) - (assign32050_e26291 * locals.var_t22_dn12)) / (locals.var_t22 * locals.var_t22))))),)
    } else {
        (locals.var_qsrc2, locals.var_qsrc2_dn3, locals.var_qsrc2_dn4, locals.var_qsrc2_dn5, locals.var_qsrc2_dn6, locals.var_qsrc2_dn7, locals.var_qsrc2_dn8, locals.var_qsrc2_dn9, locals.var_qsrc2_dn10, locals.var_qsrc2_dn11, locals.var_qsrc2_dn12,)
    }
};
        locals.var_qsrc2 = assign32050_e26297;
        locals.var_qsrc2_dn3 = assign32050_e26297_d_n3;
        locals.var_qsrc2_dn4 = assign32050_e26297_d_n4;
        locals.var_qsrc2_dn5 = assign32050_e26297_d_n5;
        locals.var_qsrc2_dn6 = assign32050_e26297_d_n6;
        locals.var_qsrc2_dn7 = assign32050_e26297_d_n7;
        locals.var_qsrc2_dn8 = assign32050_e26297_d_n8;
        locals.var_qsrc2_dn9 = assign32050_e26297_d_n9;
        locals.var_qsrc2_dn10 = assign32050_e26297_d_n10;
        locals.var_qsrc2_dn11 = assign32050_e26297_d_n11;
        locals.var_qsrc2_dn12 = assign32050_e26297_d_n12;

        let (assign32060_e26310, assign32060_e26310_d_n3, assign32060_e26310_d_n4, assign32060_e26310_d_n5, assign32060_e26310_d_n6, assign32060_e26310_d_n7, assign32060_e26310_d_n8, assign32060_e26310_d_n9, assign32060_e26310_d_n10, assign32060_e26310_d_n11, assign32060_e26310_d_n12,) = {
    if ((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1745 != 0.0)) && (locals.var_guard1746 != 0.0)) {
        let assign32060_e26308: f64 = (locals.var_qsrc + locals.var_qsrc2);
        (assign32060_e26308, (locals.var_qsrc_dn3 + locals.var_qsrc2_dn3), (locals.var_qsrc_dn4 + locals.var_qsrc2_dn4), (locals.var_qsrc_dn5 + locals.var_qsrc2_dn5), (locals.var_qsrc_dn6 + locals.var_qsrc2_dn6), (locals.var_qsrc_dn7 + locals.var_qsrc2_dn7), (locals.var_qsrc_dn8 + locals.var_qsrc2_dn8), (locals.var_qsrc_dn9 + locals.var_qsrc2_dn9), (locals.var_qsrc_dn10 + locals.var_qsrc2_dn10), (locals.var_qsrc_dn11 + locals.var_qsrc2_dn11), (locals.var_qsrc_dn12 + locals.var_qsrc2_dn12),)
    } else {
        (locals.var_qsrc, locals.var_qsrc_dn3, locals.var_qsrc_dn4, locals.var_qsrc_dn5, locals.var_qsrc_dn6, locals.var_qsrc_dn7, locals.var_qsrc_dn8, locals.var_qsrc_dn9, locals.var_qsrc_dn10, locals.var_qsrc_dn11, locals.var_qsrc_dn12,)
    }
};
        locals.var_qsrc = assign32060_e26310;
        locals.var_qsrc_dn3 = assign32060_e26310_d_n3;
        locals.var_qsrc_dn4 = assign32060_e26310_d_n4;
        locals.var_qsrc_dn5 = assign32060_e26310_d_n5;
        locals.var_qsrc_dn6 = assign32060_e26310_d_n6;
        locals.var_qsrc_dn7 = assign32060_e26310_d_n7;
        locals.var_qsrc_dn8 = assign32060_e26310_d_n8;
        locals.var_qsrc_dn9 = assign32060_e26310_d_n9;
        locals.var_qsrc_dn10 = assign32060_e26310_d_n10;
        locals.var_qsrc_dn11 = assign32060_e26310_d_n11;
        locals.var_qsrc_dn12 = assign32060_e26310_d_n12;

        let assign32070_e26313: f64 = if locals.var_b4soixpart < 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1747 = assign32070_e26313;

        let (assign32080_e26327, assign32080_e26327_d_n3, assign32080_e26327_d_n4, assign32080_e26327_d_n5, assign32080_e26327_d_n6, assign32080_e26327_d_n7, assign32080_e26327_d_n8, assign32080_e26327_d_n9, assign32080_e26327_d_n10, assign32080_e26327_d_n11, assign32080_e26327_d_n12,) = {
    if ((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1745 == 0.0)) && (locals.var_guard1747 != 0.0)) {
        let assign32080_e26325: f64 = (locals.var_t2__blk1146 / 12.0);
        (assign32080_e26325, (locals.var_t2__blk1146_dn3 / 12.0), (locals.var_t2__blk1146_dn4 / 12.0), (locals.var_t2__blk1146_dn5 / 12.0), (locals.var_t2__blk1146_dn6 / 12.0), (locals.var_t2__blk1146_dn7 / 12.0), (locals.var_t2__blk1146_dn8 / 12.0), (locals.var_t2__blk1146_dn9 / 12.0), (locals.var_t2__blk1146_dn10 / 12.0), (locals.var_t2__blk1146_dn11 / 12.0), (locals.var_t2__blk1146_dn12 / 12.0),)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign32080_e26327;
        locals.var_t2__blk1146_dn3 = assign32080_e26327_d_n3;
        locals.var_t2__blk1146_dn4 = assign32080_e26327_d_n4;
        locals.var_t2__blk1146_dn5 = assign32080_e26327_d_n5;
        locals.var_t2__blk1146_dn6 = assign32080_e26327_d_n6;
        locals.var_t2__blk1146_dn7 = assign32080_e26327_d_n7;
        locals.var_t2__blk1146_dn8 = assign32080_e26327_d_n8;
        locals.var_t2__blk1146_dn9 = assign32080_e26327_d_n9;
        locals.var_t2__blk1146_dn10 = assign32080_e26327_d_n10;
        locals.var_t2__blk1146_dn11 = assign32080_e26327_d_n11;
        locals.var_t2__blk1146_dn12 = assign32080_e26327_d_n12;

        let (assign32090_e26345, assign32090_e26345_d_n3, assign32090_e26345_d_n4, assign32090_e26345_d_n5, assign32090_e26345_d_n6, assign32090_e26345_d_n7, assign32090_e26345_d_n8, assign32090_e26345_d_n9, assign32090_e26345_d_n10, assign32090_e26345_d_n11, assign32090_e26345_d_n12,) = {
    if ((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1745 == 0.0)) && (locals.var_guard1747 != 0.0)) {
        let assign32090_e26339: f64 = (0.5 * locals.var_coxwlcen);
        let assign32090_e26342: f64 = (locals.var_t2__blk1146 * locals.var_t2__blk1146);
        let assign32090_e26343: f64 = (assign32090_e26339 / assign32090_e26342);
        (assign32090_e26343, ((((0.5 * locals.var_coxwlcen_dn3) * assign32090_e26342) - (assign32090_e26339 * ((locals.var_t2__blk1146_dn3 * locals.var_t2__blk1146) + (locals.var_t2__blk1146 * locals.var_t2__blk1146_dn3)))) / (assign32090_e26342 * assign32090_e26342)), ((((0.5 * locals.var_coxwlcen_dn4) * assign32090_e26342) - (assign32090_e26339 * ((locals.var_t2__blk1146_dn4 * locals.var_t2__blk1146) + (locals.var_t2__blk1146 * locals.var_t2__blk1146_dn4)))) / (assign32090_e26342 * assign32090_e26342)), ((((0.5 * locals.var_coxwlcen_dn5) * assign32090_e26342) - (assign32090_e26339 * ((locals.var_t2__blk1146_dn5 * locals.var_t2__blk1146) + (locals.var_t2__blk1146 * locals.var_t2__blk1146_dn5)))) / (assign32090_e26342 * assign32090_e26342)), ((((0.5 * locals.var_coxwlcen_dn6) * assign32090_e26342) - (assign32090_e26339 * ((locals.var_t2__blk1146_dn6 * locals.var_t2__blk1146) + (locals.var_t2__blk1146 * locals.var_t2__blk1146_dn6)))) / (assign32090_e26342 * assign32090_e26342)), ((((0.5 * locals.var_coxwlcen_dn7) * assign32090_e26342) - (assign32090_e26339 * ((locals.var_t2__blk1146_dn7 * locals.var_t2__blk1146) + (locals.var_t2__blk1146 * locals.var_t2__blk1146_dn7)))) / (assign32090_e26342 * assign32090_e26342)), ((((0.5 * locals.var_coxwlcen_dn8) * assign32090_e26342) - (assign32090_e26339 * ((locals.var_t2__blk1146_dn8 * locals.var_t2__blk1146) + (locals.var_t2__blk1146 * locals.var_t2__blk1146_dn8)))) / (assign32090_e26342 * assign32090_e26342)), ((((0.5 * locals.var_coxwlcen_dn9) * assign32090_e26342) - (assign32090_e26339 * ((locals.var_t2__blk1146_dn9 * locals.var_t2__blk1146) + (locals.var_t2__blk1146 * locals.var_t2__blk1146_dn9)))) / (assign32090_e26342 * assign32090_e26342)), ((((0.5 * locals.var_coxwlcen_dn10) * assign32090_e26342) - (assign32090_e26339 * ((locals.var_t2__blk1146_dn10 * locals.var_t2__blk1146) + (locals.var_t2__blk1146 * locals.var_t2__blk1146_dn10)))) / (assign32090_e26342 * assign32090_e26342)), ((((0.5 * locals.var_coxwlcen_dn11) * assign32090_e26342) - (assign32090_e26339 * ((locals.var_t2__blk1146_dn11 * locals.var_t2__blk1146) + (locals.var_t2__blk1146 * locals.var_t2__blk1146_dn11)))) / (assign32090_e26342 * assign32090_e26342)), ((((0.5 * locals.var_coxwlcen_dn12) * assign32090_e26342) - (assign32090_e26339 * ((locals.var_t2__blk1146_dn12 * locals.var_t2__blk1146) + (locals.var_t2__blk1146 * locals.var_t2__blk1146_dn12)))) / (assign32090_e26342 * assign32090_e26342)),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign32090_e26345;
        locals.var_t3__blk1147_dn3 = assign32090_e26345_d_n3;
        locals.var_t3__blk1147_dn4 = assign32090_e26345_d_n4;
        locals.var_t3__blk1147_dn5 = assign32090_e26345_d_n5;
        locals.var_t3__blk1147_dn6 = assign32090_e26345_d_n6;
        locals.var_t3__blk1147_dn7 = assign32090_e26345_d_n7;
        locals.var_t3__blk1147_dn8 = assign32090_e26345_d_n8;
        locals.var_t3__blk1147_dn9 = assign32090_e26345_d_n9;
        locals.var_t3__blk1147_dn10 = assign32090_e26345_d_n10;
        locals.var_t3__blk1147_dn11 = assign32090_e26345_d_n11;
        locals.var_t3__blk1147_dn12 = assign32090_e26345_d_n12;

        let (assign32100_e26385, assign32100_e26385_d_n3, assign32100_e26385_d_n4, assign32100_e26385_d_n5, assign32100_e26385_d_n6, assign32100_e26385_d_n7, assign32100_e26385_d_n8, assign32100_e26385_d_n9, assign32100_e26385_d_n10, assign32100_e26385_d_n11, assign32100_e26385_d_n12,) = {
    if ((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1745 == 0.0)) && (locals.var_guard1747 != 0.0)) {
        let assign32100_e26358: f64 = (2.0 * locals.var_t0__blk1144);
        let assign32100_e26360: f64 = (assign32100_e26358 * locals.var_t0__blk1144);
        let assign32100_e26362: f64 = (assign32100_e26360 / 3.0);
        let assign32100_e26367: f64 = (4.0 * locals.var_t0__blk1144);
        let assign32100_e26369: f64 = (assign32100_e26367 / 3.0);
        let assign32100_e26370: f64 = (locals.var_t1__blk1145 - assign32100_e26369);
        let assign32100_e26371: f64 = (locals.var_t1__blk1145 * assign32100_e26370);
        let assign32100_e26372: f64 = (assign32100_e26362 + assign32100_e26371);
        let assign32100_e26373: f64 = (locals.var_t1__blk1145 * assign32100_e26372);
        let assign32100_e26376: f64 = (2.0 * locals.var_t0__blk1144);
        let assign32100_e26378: f64 = (assign32100_e26376 * locals.var_t0__blk1144);
        let assign32100_e26380: f64 = (assign32100_e26378 * locals.var_t0__blk1144);
        let assign32100_e26382: f64 = (assign32100_e26380 / 15.0);
        let assign32100_e26383: f64 = (assign32100_e26373 - assign32100_e26382);
        (assign32100_e26383, (((locals.var_t1__blk1145_dn3 * assign32100_e26372) + (locals.var_t1__blk1145 * (((((2.0 * locals.var_t0__blk1144_dn3) * locals.var_t0__blk1144) + (assign32100_e26358 * locals.var_t0__blk1144_dn3)) / 3.0) + ((locals.var_t1__blk1145_dn3 * assign32100_e26370) + (locals.var_t1__blk1145 * (locals.var_t1__blk1145_dn3 - ((4.0 * locals.var_t0__blk1144_dn3) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk1144_dn3) * locals.var_t0__blk1144) + (assign32100_e26376 * locals.var_t0__blk1144_dn3)) * locals.var_t0__blk1144) + (assign32100_e26378 * locals.var_t0__blk1144_dn3)) / 15.0)), (((locals.var_t1__blk1145_dn4 * assign32100_e26372) + (locals.var_t1__blk1145 * (((((2.0 * locals.var_t0__blk1144_dn4) * locals.var_t0__blk1144) + (assign32100_e26358 * locals.var_t0__blk1144_dn4)) / 3.0) + ((locals.var_t1__blk1145_dn4 * assign32100_e26370) + (locals.var_t1__blk1145 * (locals.var_t1__blk1145_dn4 - ((4.0 * locals.var_t0__blk1144_dn4) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk1144_dn4) * locals.var_t0__blk1144) + (assign32100_e26376 * locals.var_t0__blk1144_dn4)) * locals.var_t0__blk1144) + (assign32100_e26378 * locals.var_t0__blk1144_dn4)) / 15.0)), (((locals.var_t1__blk1145_dn5 * assign32100_e26372) + (locals.var_t1__blk1145 * (((((2.0 * locals.var_t0__blk1144_dn5) * locals.var_t0__blk1144) + (assign32100_e26358 * locals.var_t0__blk1144_dn5)) / 3.0) + ((locals.var_t1__blk1145_dn5 * assign32100_e26370) + (locals.var_t1__blk1145 * (locals.var_t1__blk1145_dn5 - ((4.0 * locals.var_t0__blk1144_dn5) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk1144_dn5) * locals.var_t0__blk1144) + (assign32100_e26376 * locals.var_t0__blk1144_dn5)) * locals.var_t0__blk1144) + (assign32100_e26378 * locals.var_t0__blk1144_dn5)) / 15.0)), (((locals.var_t1__blk1145_dn6 * assign32100_e26372) + (locals.var_t1__blk1145 * (((((2.0 * locals.var_t0__blk1144_dn6) * locals.var_t0__blk1144) + (assign32100_e26358 * locals.var_t0__blk1144_dn6)) / 3.0) + ((locals.var_t1__blk1145_dn6 * assign32100_e26370) + (locals.var_t1__blk1145 * (locals.var_t1__blk1145_dn6 - ((4.0 * locals.var_t0__blk1144_dn6) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk1144_dn6) * locals.var_t0__blk1144) + (assign32100_e26376 * locals.var_t0__blk1144_dn6)) * locals.var_t0__blk1144) + (assign32100_e26378 * locals.var_t0__blk1144_dn6)) / 15.0)), (((locals.var_t1__blk1145_dn7 * assign32100_e26372) + (locals.var_t1__blk1145 * (((((2.0 * locals.var_t0__blk1144_dn7) * locals.var_t0__blk1144) + (assign32100_e26358 * locals.var_t0__blk1144_dn7)) / 3.0) + ((locals.var_t1__blk1145_dn7 * assign32100_e26370) + (locals.var_t1__blk1145 * (locals.var_t1__blk1145_dn7 - ((4.0 * locals.var_t0__blk1144_dn7) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk1144_dn7) * locals.var_t0__blk1144) + (assign32100_e26376 * locals.var_t0__blk1144_dn7)) * locals.var_t0__blk1144) + (assign32100_e26378 * locals.var_t0__blk1144_dn7)) / 15.0)), (((locals.var_t1__blk1145_dn8 * assign32100_e26372) + (locals.var_t1__blk1145 * (((((2.0 * locals.var_t0__blk1144_dn8) * locals.var_t0__blk1144) + (assign32100_e26358 * locals.var_t0__blk1144_dn8)) / 3.0) + ((locals.var_t1__blk1145_dn8 * assign32100_e26370) + (locals.var_t1__blk1145 * (locals.var_t1__blk1145_dn8 - ((4.0 * locals.var_t0__blk1144_dn8) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk1144_dn8) * locals.var_t0__blk1144) + (assign32100_e26376 * locals.var_t0__blk1144_dn8)) * locals.var_t0__blk1144) + (assign32100_e26378 * locals.var_t0__blk1144_dn8)) / 15.0)), (((locals.var_t1__blk1145_dn9 * assign32100_e26372) + (locals.var_t1__blk1145 * (((((2.0 * locals.var_t0__blk1144_dn9) * locals.var_t0__blk1144) + (assign32100_e26358 * locals.var_t0__blk1144_dn9)) / 3.0) + ((locals.var_t1__blk1145_dn9 * assign32100_e26370) + (locals.var_t1__blk1145 * (locals.var_t1__blk1145_dn9 - ((4.0 * locals.var_t0__blk1144_dn9) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk1144_dn9) * locals.var_t0__blk1144) + (assign32100_e26376 * locals.var_t0__blk1144_dn9)) * locals.var_t0__blk1144) + (assign32100_e26378 * locals.var_t0__blk1144_dn9)) / 15.0)), (((locals.var_t1__blk1145_dn10 * assign32100_e26372) + (locals.var_t1__blk1145 * (((((2.0 * locals.var_t0__blk1144_dn10) * locals.var_t0__blk1144) + (assign32100_e26358 * locals.var_t0__blk1144_dn10)) / 3.0) + ((locals.var_t1__blk1145_dn10 * assign32100_e26370) + (locals.var_t1__blk1145 * (locals.var_t1__blk1145_dn10 - ((4.0 * locals.var_t0__blk1144_dn10) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk1144_dn10) * locals.var_t0__blk1144) + (assign32100_e26376 * locals.var_t0__blk1144_dn10)) * locals.var_t0__blk1144) + (assign32100_e26378 * locals.var_t0__blk1144_dn10)) / 15.0)), (((locals.var_t1__blk1145_dn11 * assign32100_e26372) + (locals.var_t1__blk1145 * (((((2.0 * locals.var_t0__blk1144_dn11) * locals.var_t0__blk1144) + (assign32100_e26358 * locals.var_t0__blk1144_dn11)) / 3.0) + ((locals.var_t1__blk1145_dn11 * assign32100_e26370) + (locals.var_t1__blk1145 * (locals.var_t1__blk1145_dn11 - ((4.0 * locals.var_t0__blk1144_dn11) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk1144_dn11) * locals.var_t0__blk1144) + (assign32100_e26376 * locals.var_t0__blk1144_dn11)) * locals.var_t0__blk1144) + (assign32100_e26378 * locals.var_t0__blk1144_dn11)) / 15.0)), (((locals.var_t1__blk1145_dn12 * assign32100_e26372) + (locals.var_t1__blk1145 * (((((2.0 * locals.var_t0__blk1144_dn12) * locals.var_t0__blk1144) + (assign32100_e26358 * locals.var_t0__blk1144_dn12)) / 3.0) + ((locals.var_t1__blk1145_dn12 * assign32100_e26370) + (locals.var_t1__blk1145 * (locals.var_t1__blk1145_dn12 - ((4.0 * locals.var_t0__blk1144_dn12) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk1144_dn12) * locals.var_t0__blk1144) + (assign32100_e26376 * locals.var_t0__blk1144_dn12)) * locals.var_t0__blk1144) + (assign32100_e26378 * locals.var_t0__blk1144_dn12)) / 15.0)),)
    } else {
        (locals.var_t4__blk1148, locals.var_t4__blk1148_dn3, locals.var_t4__blk1148_dn4, locals.var_t4__blk1148_dn5, locals.var_t4__blk1148_dn6, locals.var_t4__blk1148_dn7, locals.var_t4__blk1148_dn8, locals.var_t4__blk1148_dn9, locals.var_t4__blk1148_dn10, locals.var_t4__blk1148_dn11, locals.var_t4__blk1148_dn12,)
    }
};
        locals.var_t4__blk1148 = assign32100_e26385;
        locals.var_t4__blk1148_dn3 = assign32100_e26385_d_n3;
        locals.var_t4__blk1148_dn4 = assign32100_e26385_d_n4;
        locals.var_t4__blk1148_dn5 = assign32100_e26385_d_n5;
        locals.var_t4__blk1148_dn6 = assign32100_e26385_d_n6;
        locals.var_t4__blk1148_dn7 = assign32100_e26385_d_n7;
        locals.var_t4__blk1148_dn8 = assign32100_e26385_d_n8;
        locals.var_t4__blk1148_dn9 = assign32100_e26385_d_n9;
        locals.var_t4__blk1148_dn10 = assign32100_e26385_d_n10;
        locals.var_t4__blk1148_dn11 = assign32100_e26385_d_n11;
        locals.var_t4__blk1148_dn12 = assign32100_e26385_d_n12;

        let (assign32110_e26400, assign32110_e26400_d_n3, assign32110_e26400_d_n4, assign32110_e26400_d_n5, assign32110_e26400_d_n6, assign32110_e26400_d_n7, assign32110_e26400_d_n8, assign32110_e26400_d_n9, assign32110_e26400_d_n10, assign32110_e26400_d_n11, assign32110_e26400_d_n12,) = {
    if ((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1745 == 0.0)) && (locals.var_guard1747 != 0.0)) {
        let assign32110_e26396: f64 = (-locals.var_t3__blk1147);
        let assign32110_e26398: f64 = (assign32110_e26396 * locals.var_t4__blk1148);
        (assign32110_e26398, (((-locals.var_t3__blk1147_dn3) * locals.var_t4__blk1148) + (assign32110_e26396 * locals.var_t4__blk1148_dn3)), (((-locals.var_t3__blk1147_dn4) * locals.var_t4__blk1148) + (assign32110_e26396 * locals.var_t4__blk1148_dn4)), (((-locals.var_t3__blk1147_dn5) * locals.var_t4__blk1148) + (assign32110_e26396 * locals.var_t4__blk1148_dn5)), (((-locals.var_t3__blk1147_dn6) * locals.var_t4__blk1148) + (assign32110_e26396 * locals.var_t4__blk1148_dn6)), (((-locals.var_t3__blk1147_dn7) * locals.var_t4__blk1148) + (assign32110_e26396 * locals.var_t4__blk1148_dn7)), (((-locals.var_t3__blk1147_dn8) * locals.var_t4__blk1148) + (assign32110_e26396 * locals.var_t4__blk1148_dn8)), (((-locals.var_t3__blk1147_dn9) * locals.var_t4__blk1148) + (assign32110_e26396 * locals.var_t4__blk1148_dn9)), (((-locals.var_t3__blk1147_dn10) * locals.var_t4__blk1148) + (assign32110_e26396 * locals.var_t4__blk1148_dn10)), (((-locals.var_t3__blk1147_dn11) * locals.var_t4__blk1148) + (assign32110_e26396 * locals.var_t4__blk1148_dn11)), (((-locals.var_t3__blk1147_dn12) * locals.var_t4__blk1148) + (assign32110_e26396 * locals.var_t4__blk1148_dn12)),)
    } else {
        (locals.var_qsrc, locals.var_qsrc_dn3, locals.var_qsrc_dn4, locals.var_qsrc_dn5, locals.var_qsrc_dn6, locals.var_qsrc_dn7, locals.var_qsrc_dn8, locals.var_qsrc_dn9, locals.var_qsrc_dn10, locals.var_qsrc_dn11, locals.var_qsrc_dn12,)
    }
};
        locals.var_qsrc = assign32110_e26400;
        locals.var_qsrc_dn3 = assign32110_e26400_d_n3;
        locals.var_qsrc_dn4 = assign32110_e26400_d_n4;
        locals.var_qsrc_dn5 = assign32110_e26400_d_n5;
        locals.var_qsrc_dn6 = assign32110_e26400_d_n6;
        locals.var_qsrc_dn7 = assign32110_e26400_d_n7;
        locals.var_qsrc_dn8 = assign32110_e26400_d_n8;
        locals.var_qsrc_dn9 = assign32110_e26400_d_n9;
        locals.var_qsrc_dn10 = assign32110_e26400_d_n10;
        locals.var_qsrc_dn11 = assign32110_e26400_d_n11;
        locals.var_qsrc_dn12 = assign32110_e26400_d_n12;

        let assign32120_e26411: f64 = if (((locals.var_b4soisoimod != 2.0) && (locals.var_b4soibodymod != 0.0)) && (locals.var_b4soiagbcp2 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1748 = assign32120_e26411;

        let (assign32130_e26427, assign32130_e26427_d_n3, assign32130_e26427_d_n4, assign32130_e26427_d_n5, assign32130_e26427_d_n6, assign32130_e26427_d_n7, assign32130_e26427_d_n8, assign32130_e26427_d_n9, assign32130_e26427_d_n10, assign32130_e26427_d_n11, assign32130_e26427_d_n12,) = {
    if (((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1745 == 0.0)) && (locals.var_guard1747 != 0.0)) && (locals.var_guard1748 != 0.0)) {
        let assign32130_e26425: f64 = (locals.var_t22 / 12.0);
        (assign32130_e26425, (locals.var_t22_dn3 / 12.0), (locals.var_t22_dn4 / 12.0), (locals.var_t22_dn5 / 12.0), (locals.var_t22_dn6 / 12.0), (locals.var_t22_dn7 / 12.0), (locals.var_t22_dn8 / 12.0), (locals.var_t22_dn9 / 12.0), (locals.var_t22_dn10 / 12.0), (locals.var_t22_dn11 / 12.0), (locals.var_t22_dn12 / 12.0),)
    } else {
        (locals.var_t22, locals.var_t22_dn3, locals.var_t22_dn4, locals.var_t22_dn5, locals.var_t22_dn6, locals.var_t22_dn7, locals.var_t22_dn8, locals.var_t22_dn9, locals.var_t22_dn10, locals.var_t22_dn11, locals.var_t22_dn12,)
    }
};
        locals.var_t22 = assign32130_e26427;
        locals.var_t22_dn3 = assign32130_e26427_d_n3;
        locals.var_t22_dn4 = assign32130_e26427_d_n4;
        locals.var_t22_dn5 = assign32130_e26427_d_n5;
        locals.var_t22_dn6 = assign32130_e26427_d_n6;
        locals.var_t22_dn7 = assign32130_e26427_d_n7;
        locals.var_t22_dn8 = assign32130_e26427_d_n8;
        locals.var_t22_dn9 = assign32130_e26427_d_n9;
        locals.var_t22_dn10 = assign32130_e26427_d_n10;
        locals.var_t22_dn11 = assign32130_e26427_d_n11;
        locals.var_t22_dn12 = assign32130_e26427_d_n12;

        let (assign32140_e26447, assign32140_e26447_d_n3, assign32140_e26447_d_n4, assign32140_e26447_d_n5, assign32140_e26447_d_n6, assign32140_e26447_d_n7, assign32140_e26447_d_n8, assign32140_e26447_d_n9, assign32140_e26447_d_n10, assign32140_e26447_d_n11, assign32140_e26447_d_n12,) = {
    if (((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1745 == 0.0)) && (locals.var_guard1747 != 0.0)) && (locals.var_guard1748 != 0.0)) {
        let assign32140_e26441: f64 = (0.5 * locals.var_coxwlcen2);
        let assign32140_e26444: f64 = (locals.var_t22 * locals.var_t22);
        let assign32140_e26445: f64 = (assign32140_e26441 / assign32140_e26444);
        (assign32140_e26445, ((((0.5 * locals.var_coxwlcen2_dn3) * assign32140_e26444) - (assign32140_e26441 * ((locals.var_t22_dn3 * locals.var_t22) + (locals.var_t22 * locals.var_t22_dn3)))) / (assign32140_e26444 * assign32140_e26444)), ((((0.5 * locals.var_coxwlcen2_dn4) * assign32140_e26444) - (assign32140_e26441 * ((locals.var_t22_dn4 * locals.var_t22) + (locals.var_t22 * locals.var_t22_dn4)))) / (assign32140_e26444 * assign32140_e26444)), ((((0.5 * locals.var_coxwlcen2_dn5) * assign32140_e26444) - (assign32140_e26441 * ((locals.var_t22_dn5 * locals.var_t22) + (locals.var_t22 * locals.var_t22_dn5)))) / (assign32140_e26444 * assign32140_e26444)), ((((0.5 * locals.var_coxwlcen2_dn6) * assign32140_e26444) - (assign32140_e26441 * ((locals.var_t22_dn6 * locals.var_t22) + (locals.var_t22 * locals.var_t22_dn6)))) / (assign32140_e26444 * assign32140_e26444)), ((((0.5 * locals.var_coxwlcen2_dn7) * assign32140_e26444) - (assign32140_e26441 * ((locals.var_t22_dn7 * locals.var_t22) + (locals.var_t22 * locals.var_t22_dn7)))) / (assign32140_e26444 * assign32140_e26444)), ((((0.5 * locals.var_coxwlcen2_dn8) * assign32140_e26444) - (assign32140_e26441 * ((locals.var_t22_dn8 * locals.var_t22) + (locals.var_t22 * locals.var_t22_dn8)))) / (assign32140_e26444 * assign32140_e26444)), ((((0.5 * locals.var_coxwlcen2_dn9) * assign32140_e26444) - (assign32140_e26441 * ((locals.var_t22_dn9 * locals.var_t22) + (locals.var_t22 * locals.var_t22_dn9)))) / (assign32140_e26444 * assign32140_e26444)), ((((0.5 * locals.var_coxwlcen2_dn10) * assign32140_e26444) - (assign32140_e26441 * ((locals.var_t22_dn10 * locals.var_t22) + (locals.var_t22 * locals.var_t22_dn10)))) / (assign32140_e26444 * assign32140_e26444)), ((((0.5 * locals.var_coxwlcen2_dn11) * assign32140_e26444) - (assign32140_e26441 * ((locals.var_t22_dn11 * locals.var_t22) + (locals.var_t22 * locals.var_t22_dn11)))) / (assign32140_e26444 * assign32140_e26444)), ((((0.5 * locals.var_coxwlcen2_dn12) * assign32140_e26444) - (assign32140_e26441 * ((locals.var_t22_dn12 * locals.var_t22) + (locals.var_t22 * locals.var_t22_dn12)))) / (assign32140_e26444 * assign32140_e26444)),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign32140_e26447;
        locals.var_t3__blk1147_dn3 = assign32140_e26447_d_n3;
        locals.var_t3__blk1147_dn4 = assign32140_e26447_d_n4;
        locals.var_t3__blk1147_dn5 = assign32140_e26447_d_n5;
        locals.var_t3__blk1147_dn6 = assign32140_e26447_d_n6;
        locals.var_t3__blk1147_dn7 = assign32140_e26447_d_n7;
        locals.var_t3__blk1147_dn8 = assign32140_e26447_d_n8;
        locals.var_t3__blk1147_dn9 = assign32140_e26447_d_n9;
        locals.var_t3__blk1147_dn10 = assign32140_e26447_d_n10;
        locals.var_t3__blk1147_dn11 = assign32140_e26447_d_n11;
        locals.var_t3__blk1147_dn12 = assign32140_e26447_d_n12;

        let (assign32150_e26489, assign32150_e26489_d_n3, assign32150_e26489_d_n4, assign32150_e26489_d_n5, assign32150_e26489_d_n6, assign32150_e26489_d_n7, assign32150_e26489_d_n8, assign32150_e26489_d_n9, assign32150_e26489_d_n10, assign32150_e26489_d_n11, assign32150_e26489_d_n12,) = {
    if (((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1745 == 0.0)) && (locals.var_guard1747 != 0.0)) && (locals.var_guard1748 != 0.0)) {
        let assign32150_e26462: f64 = (2.0 * locals.var_t02);
        let assign32150_e26464: f64 = (assign32150_e26462 * locals.var_t02);
        let assign32150_e26466: f64 = (assign32150_e26464 / 3.0);
        let assign32150_e26471: f64 = (4.0 * locals.var_t02);
        let assign32150_e26473: f64 = (assign32150_e26471 / 3.0);
        let assign32150_e26474: f64 = (locals.var_t12 - assign32150_e26473);
        let assign32150_e26475: f64 = (locals.var_t12 * assign32150_e26474);
        let assign32150_e26476: f64 = (assign32150_e26466 + assign32150_e26475);
        let assign32150_e26477: f64 = (locals.var_t12 * assign32150_e26476);
        let assign32150_e26480: f64 = (2.0 * locals.var_t02);
        let assign32150_e26482: f64 = (assign32150_e26480 * locals.var_t02);
        let assign32150_e26484: f64 = (assign32150_e26482 * locals.var_t02);
        let assign32150_e26486: f64 = (assign32150_e26484 / 15.0);
        let assign32150_e26487: f64 = (assign32150_e26477 - assign32150_e26486);
        (assign32150_e26487, (((locals.var_t12_dn3 * assign32150_e26476) + (locals.var_t12 * (((((2.0 * locals.var_t02_dn3) * locals.var_t02) + (assign32150_e26462 * locals.var_t02_dn3)) / 3.0) + ((locals.var_t12_dn3 * assign32150_e26474) + (locals.var_t12 * (locals.var_t12_dn3 - ((4.0 * locals.var_t02_dn3) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn3) * locals.var_t02) + (assign32150_e26480 * locals.var_t02_dn3)) * locals.var_t02) + (assign32150_e26482 * locals.var_t02_dn3)) / 15.0)), (((locals.var_t12_dn4 * assign32150_e26476) + (locals.var_t12 * (((((2.0 * locals.var_t02_dn4) * locals.var_t02) + (assign32150_e26462 * locals.var_t02_dn4)) / 3.0) + ((locals.var_t12_dn4 * assign32150_e26474) + (locals.var_t12 * (locals.var_t12_dn4 - ((4.0 * locals.var_t02_dn4) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn4) * locals.var_t02) + (assign32150_e26480 * locals.var_t02_dn4)) * locals.var_t02) + (assign32150_e26482 * locals.var_t02_dn4)) / 15.0)), (((locals.var_t12_dn5 * assign32150_e26476) + (locals.var_t12 * (((((2.0 * locals.var_t02_dn5) * locals.var_t02) + (assign32150_e26462 * locals.var_t02_dn5)) / 3.0) + ((locals.var_t12_dn5 * assign32150_e26474) + (locals.var_t12 * (locals.var_t12_dn5 - ((4.0 * locals.var_t02_dn5) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn5) * locals.var_t02) + (assign32150_e26480 * locals.var_t02_dn5)) * locals.var_t02) + (assign32150_e26482 * locals.var_t02_dn5)) / 15.0)), (((locals.var_t12_dn6 * assign32150_e26476) + (locals.var_t12 * (((((2.0 * locals.var_t02_dn6) * locals.var_t02) + (assign32150_e26462 * locals.var_t02_dn6)) / 3.0) + ((locals.var_t12_dn6 * assign32150_e26474) + (locals.var_t12 * (locals.var_t12_dn6 - ((4.0 * locals.var_t02_dn6) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn6) * locals.var_t02) + (assign32150_e26480 * locals.var_t02_dn6)) * locals.var_t02) + (assign32150_e26482 * locals.var_t02_dn6)) / 15.0)), (((locals.var_t12_dn7 * assign32150_e26476) + (locals.var_t12 * (((((2.0 * locals.var_t02_dn7) * locals.var_t02) + (assign32150_e26462 * locals.var_t02_dn7)) / 3.0) + ((locals.var_t12_dn7 * assign32150_e26474) + (locals.var_t12 * (locals.var_t12_dn7 - ((4.0 * locals.var_t02_dn7) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn7) * locals.var_t02) + (assign32150_e26480 * locals.var_t02_dn7)) * locals.var_t02) + (assign32150_e26482 * locals.var_t02_dn7)) / 15.0)), (((locals.var_t12_dn8 * assign32150_e26476) + (locals.var_t12 * (((((2.0 * locals.var_t02_dn8) * locals.var_t02) + (assign32150_e26462 * locals.var_t02_dn8)) / 3.0) + ((locals.var_t12_dn8 * assign32150_e26474) + (locals.var_t12 * (locals.var_t12_dn8 - ((4.0 * locals.var_t02_dn8) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn8) * locals.var_t02) + (assign32150_e26480 * locals.var_t02_dn8)) * locals.var_t02) + (assign32150_e26482 * locals.var_t02_dn8)) / 15.0)), (((locals.var_t12_dn9 * assign32150_e26476) + (locals.var_t12 * (((((2.0 * locals.var_t02_dn9) * locals.var_t02) + (assign32150_e26462 * locals.var_t02_dn9)) / 3.0) + ((locals.var_t12_dn9 * assign32150_e26474) + (locals.var_t12 * (locals.var_t12_dn9 - ((4.0 * locals.var_t02_dn9) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn9) * locals.var_t02) + (assign32150_e26480 * locals.var_t02_dn9)) * locals.var_t02) + (assign32150_e26482 * locals.var_t02_dn9)) / 15.0)), (((locals.var_t12_dn10 * assign32150_e26476) + (locals.var_t12 * (((((2.0 * locals.var_t02_dn10) * locals.var_t02) + (assign32150_e26462 * locals.var_t02_dn10)) / 3.0) + ((locals.var_t12_dn10 * assign32150_e26474) + (locals.var_t12 * (locals.var_t12_dn10 - ((4.0 * locals.var_t02_dn10) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn10) * locals.var_t02) + (assign32150_e26480 * locals.var_t02_dn10)) * locals.var_t02) + (assign32150_e26482 * locals.var_t02_dn10)) / 15.0)), (((locals.var_t12_dn11 * assign32150_e26476) + (locals.var_t12 * (((((2.0 * locals.var_t02_dn11) * locals.var_t02) + (assign32150_e26462 * locals.var_t02_dn11)) / 3.0) + ((locals.var_t12_dn11 * assign32150_e26474) + (locals.var_t12 * (locals.var_t12_dn11 - ((4.0 * locals.var_t02_dn11) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn11) * locals.var_t02) + (assign32150_e26480 * locals.var_t02_dn11)) * locals.var_t02) + (assign32150_e26482 * locals.var_t02_dn11)) / 15.0)), (((locals.var_t12_dn12 * assign32150_e26476) + (locals.var_t12 * (((((2.0 * locals.var_t02_dn12) * locals.var_t02) + (assign32150_e26462 * locals.var_t02_dn12)) / 3.0) + ((locals.var_t12_dn12 * assign32150_e26474) + (locals.var_t12 * (locals.var_t12_dn12 - ((4.0 * locals.var_t02_dn12) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn12) * locals.var_t02) + (assign32150_e26480 * locals.var_t02_dn12)) * locals.var_t02) + (assign32150_e26482 * locals.var_t02_dn12)) / 15.0)),)
    } else {
        (locals.var_t4__blk1148, locals.var_t4__blk1148_dn3, locals.var_t4__blk1148_dn4, locals.var_t4__blk1148_dn5, locals.var_t4__blk1148_dn6, locals.var_t4__blk1148_dn7, locals.var_t4__blk1148_dn8, locals.var_t4__blk1148_dn9, locals.var_t4__blk1148_dn10, locals.var_t4__blk1148_dn11, locals.var_t4__blk1148_dn12,)
    }
};
        locals.var_t4__blk1148 = assign32150_e26489;
        locals.var_t4__blk1148_dn3 = assign32150_e26489_d_n3;
        locals.var_t4__blk1148_dn4 = assign32150_e26489_d_n4;
        locals.var_t4__blk1148_dn5 = assign32150_e26489_d_n5;
        locals.var_t4__blk1148_dn6 = assign32150_e26489_d_n6;
        locals.var_t4__blk1148_dn7 = assign32150_e26489_d_n7;
        locals.var_t4__blk1148_dn8 = assign32150_e26489_d_n8;
        locals.var_t4__blk1148_dn9 = assign32150_e26489_d_n9;
        locals.var_t4__blk1148_dn10 = assign32150_e26489_d_n10;
        locals.var_t4__blk1148_dn11 = assign32150_e26489_d_n11;
        locals.var_t4__blk1148_dn12 = assign32150_e26489_d_n12;

        let (assign32160_e26506, assign32160_e26506_d_n3, assign32160_e26506_d_n4, assign32160_e26506_d_n5, assign32160_e26506_d_n6, assign32160_e26506_d_n7, assign32160_e26506_d_n8, assign32160_e26506_d_n9, assign32160_e26506_d_n10, assign32160_e26506_d_n11, assign32160_e26506_d_n12,) = {
    if (((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1745 == 0.0)) && (locals.var_guard1747 != 0.0)) && (locals.var_guard1748 != 0.0)) {
        let assign32160_e26502: f64 = (-locals.var_t3__blk1147);
        let assign32160_e26504: f64 = (assign32160_e26502 * locals.var_t4__blk1148);
        (assign32160_e26504, (((-locals.var_t3__blk1147_dn3) * locals.var_t4__blk1148) + (assign32160_e26502 * locals.var_t4__blk1148_dn3)), (((-locals.var_t3__blk1147_dn4) * locals.var_t4__blk1148) + (assign32160_e26502 * locals.var_t4__blk1148_dn4)), (((-locals.var_t3__blk1147_dn5) * locals.var_t4__blk1148) + (assign32160_e26502 * locals.var_t4__blk1148_dn5)), (((-locals.var_t3__blk1147_dn6) * locals.var_t4__blk1148) + (assign32160_e26502 * locals.var_t4__blk1148_dn6)), (((-locals.var_t3__blk1147_dn7) * locals.var_t4__blk1148) + (assign32160_e26502 * locals.var_t4__blk1148_dn7)), (((-locals.var_t3__blk1147_dn8) * locals.var_t4__blk1148) + (assign32160_e26502 * locals.var_t4__blk1148_dn8)), (((-locals.var_t3__blk1147_dn9) * locals.var_t4__blk1148) + (assign32160_e26502 * locals.var_t4__blk1148_dn9)), (((-locals.var_t3__blk1147_dn10) * locals.var_t4__blk1148) + (assign32160_e26502 * locals.var_t4__blk1148_dn10)), (((-locals.var_t3__blk1147_dn11) * locals.var_t4__blk1148) + (assign32160_e26502 * locals.var_t4__blk1148_dn11)), (((-locals.var_t3__blk1147_dn12) * locals.var_t4__blk1148) + (assign32160_e26502 * locals.var_t4__blk1148_dn12)),)
    } else {
        (locals.var_qsrc2, locals.var_qsrc2_dn3, locals.var_qsrc2_dn4, locals.var_qsrc2_dn5, locals.var_qsrc2_dn6, locals.var_qsrc2_dn7, locals.var_qsrc2_dn8, locals.var_qsrc2_dn9, locals.var_qsrc2_dn10, locals.var_qsrc2_dn11, locals.var_qsrc2_dn12,)
    }
};
        locals.var_qsrc2 = assign32160_e26506;
        locals.var_qsrc2_dn3 = assign32160_e26506_d_n3;
        locals.var_qsrc2_dn4 = assign32160_e26506_d_n4;
        locals.var_qsrc2_dn5 = assign32160_e26506_d_n5;
        locals.var_qsrc2_dn6 = assign32160_e26506_d_n6;
        locals.var_qsrc2_dn7 = assign32160_e26506_d_n7;
        locals.var_qsrc2_dn8 = assign32160_e26506_d_n8;
        locals.var_qsrc2_dn9 = assign32160_e26506_d_n9;
        locals.var_qsrc2_dn10 = assign32160_e26506_d_n10;
        locals.var_qsrc2_dn11 = assign32160_e26506_d_n11;
        locals.var_qsrc2_dn12 = assign32160_e26506_d_n12;

        let (assign32170_e26522, assign32170_e26522_d_n3, assign32170_e26522_d_n4, assign32170_e26522_d_n5, assign32170_e26522_d_n6, assign32170_e26522_d_n7, assign32170_e26522_d_n8, assign32170_e26522_d_n9, assign32170_e26522_d_n10, assign32170_e26522_d_n11, assign32170_e26522_d_n12,) = {
    if (((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1745 == 0.0)) && (locals.var_guard1747 != 0.0)) && (locals.var_guard1748 != 0.0)) {
        let assign32170_e26520: f64 = (locals.var_qsrc + locals.var_qsrc2);
        (assign32170_e26520, (locals.var_qsrc_dn3 + locals.var_qsrc2_dn3), (locals.var_qsrc_dn4 + locals.var_qsrc2_dn4), (locals.var_qsrc_dn5 + locals.var_qsrc2_dn5), (locals.var_qsrc_dn6 + locals.var_qsrc2_dn6), (locals.var_qsrc_dn7 + locals.var_qsrc2_dn7), (locals.var_qsrc_dn8 + locals.var_qsrc2_dn8), (locals.var_qsrc_dn9 + locals.var_qsrc2_dn9), (locals.var_qsrc_dn10 + locals.var_qsrc2_dn10), (locals.var_qsrc_dn11 + locals.var_qsrc2_dn11), (locals.var_qsrc_dn12 + locals.var_qsrc2_dn12),)
    } else {
        (locals.var_qsrc, locals.var_qsrc_dn3, locals.var_qsrc_dn4, locals.var_qsrc_dn5, locals.var_qsrc_dn6, locals.var_qsrc_dn7, locals.var_qsrc_dn8, locals.var_qsrc_dn9, locals.var_qsrc_dn10, locals.var_qsrc_dn11, locals.var_qsrc_dn12,)
    }
};
        locals.var_qsrc = assign32170_e26522;
        locals.var_qsrc_dn3 = assign32170_e26522_d_n3;
        locals.var_qsrc_dn4 = assign32170_e26522_d_n4;
        locals.var_qsrc_dn5 = assign32170_e26522_d_n5;
        locals.var_qsrc_dn6 = assign32170_e26522_d_n6;
        locals.var_qsrc_dn7 = assign32170_e26522_d_n7;
        locals.var_qsrc_dn8 = assign32170_e26522_d_n8;
        locals.var_qsrc_dn9 = assign32170_e26522_d_n9;
        locals.var_qsrc_dn10 = assign32170_e26522_d_n10;
        locals.var_qsrc_dn11 = assign32170_e26522_d_n11;
        locals.var_qsrc_dn12 = assign32170_e26522_d_n12;

        let (assign32180_e26538, assign32180_e26538_d_n3, assign32180_e26538_d_n4, assign32180_e26538_d_n5, assign32180_e26538_d_n6, assign32180_e26538_d_n7, assign32180_e26538_d_n8, assign32180_e26538_d_n9, assign32180_e26538_d_n10, assign32180_e26538_d_n11, assign32180_e26538_d_n12,) = {
    if ((((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1745 == 0.0)) && (locals.var_guard1747 == 0.0)) {
        let assign32180_e26534: f64 = (-0.5);
        let assign32180_e26536: f64 = (assign32180_e26534 * locals.var_qgate);
        (assign32180_e26536, (assign32180_e26534 * locals.var_qgate_dn3), (assign32180_e26534 * locals.var_qgate_dn4), (assign32180_e26534 * locals.var_qgate_dn5), (assign32180_e26534 * locals.var_qgate_dn6), (assign32180_e26534 * locals.var_qgate_dn7), (assign32180_e26534 * locals.var_qgate_dn8), (assign32180_e26534 * locals.var_qgate_dn9), (assign32180_e26534 * locals.var_qgate_dn10), (assign32180_e26534 * locals.var_qgate_dn11), (assign32180_e26534 * locals.var_qgate_dn12),)
    } else {
        (locals.var_qsrc, locals.var_qsrc_dn3, locals.var_qsrc_dn4, locals.var_qsrc_dn5, locals.var_qsrc_dn6, locals.var_qsrc_dn7, locals.var_qsrc_dn8, locals.var_qsrc_dn9, locals.var_qsrc_dn10, locals.var_qsrc_dn11, locals.var_qsrc_dn12,)
    }
};
        locals.var_qsrc = assign32180_e26538;
        locals.var_qsrc_dn3 = assign32180_e26538_d_n3;
        locals.var_qsrc_dn4 = assign32180_e26538_d_n4;
        locals.var_qsrc_dn5 = assign32180_e26538_d_n5;
        locals.var_qsrc_dn6 = assign32180_e26538_d_n6;
        locals.var_qsrc_dn7 = assign32180_e26538_d_n7;
        locals.var_qsrc_dn8 = assign32180_e26538_d_n8;
        locals.var_qsrc_dn9 = assign32180_e26538_d_n9;
        locals.var_qsrc_dn10 = assign32180_e26538_d_n10;
        locals.var_qsrc_dn11 = assign32180_e26538_d_n11;
        locals.var_qsrc_dn12 = assign32180_e26538_d_n12;

        let assign32190_e26541: f64 = if locals.var_b4soisoimod == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1749 = assign32190_e26541;

        let (assign32200_e26550, assign32200_e26550_d_n3, assign32200_e26550_d_n4, assign32200_e26550_d_n5, assign32200_e26550_d_n6, assign32200_e26550_d_n7, assign32200_e26550_d_n8, assign32200_e26550_d_n9, assign32200_e26550_d_n10, assign32200_e26550_d_n11, assign32200_e26550_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1749 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qe1, locals.var_qe1_dn3, locals.var_qe1_dn4, locals.var_qe1_dn5, locals.var_qe1_dn6, locals.var_qe1_dn7, locals.var_qe1_dn8, locals.var_qe1_dn9, locals.var_qe1_dn10, locals.var_qe1_dn11, locals.var_qe1_dn12,)
    }
};
        locals.var_qe1 = assign32200_e26550;
        locals.var_qe1_dn3 = assign32200_e26550_d_n3;
        locals.var_qe1_dn4 = assign32200_e26550_d_n4;
        locals.var_qe1_dn5 = assign32200_e26550_d_n5;
        locals.var_qe1_dn6 = assign32200_e26550_d_n6;
        locals.var_qe1_dn7 = assign32200_e26550_d_n7;
        locals.var_qe1_dn8 = assign32200_e26550_d_n8;
        locals.var_qe1_dn9 = assign32200_e26550_d_n9;
        locals.var_qe1_dn10 = assign32200_e26550_d_n10;
        locals.var_qe1_dn11 = assign32200_e26550_d_n11;
        locals.var_qe1_dn12 = assign32200_e26550_d_n12;

        let (assign32210_e26574, assign32210_e26574_d_n3, assign32210_e26574_d_n4, assign32210_e26574_d_n5, assign32210_e26574_d_n6, assign32210_e26574_d_n7, assign32210_e26574_d_n8, assign32210_e26574_d_n9, assign32210_e26574_d_n10, assign32210_e26574_d_n11, assign32210_e26574_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1749 == 0.0)) {
        let assign32210_e26560: f64 = (locals.var_pparam_b4soikb1 * locals.var_b4soifbody);
        let assign32210_e26562: f64 = (assign32210_e26560 * locals.var_cbox);
        let assign32210_e26565: f64 = (locals.var_pparam_b4soiweffcv / locals.var_b4soinseg);
        let assign32210_e26567: f64 = (assign32210_e26565 * locals.var_b4soinf);
        let assign32210_e26569: f64 = (assign32210_e26567 * locals.var_pparam_b4soileffcvbg);
        let assign32210_e26571: f64 = (assign32210_e26569 + locals.var_b4soiaebcp);
        let assign32210_e26572: f64 = (assign32210_e26562 * assign32210_e26571);
        (assign32210_e26572, ((((locals.var_pparam_b4soikb1_dn3 * locals.var_b4soifbody) * locals.var_cbox) * assign32210_e26571) + (assign32210_e26562 * ((((locals.var_pparam_b4soiweffcv_dn3 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcvbg) + (assign32210_e26567 * locals.var_pparam_b4soileffcvbg_dn3)))), ((((locals.var_pparam_b4soikb1_dn4 * locals.var_b4soifbody) * locals.var_cbox) * assign32210_e26571) + (assign32210_e26562 * ((((locals.var_pparam_b4soiweffcv_dn4 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcvbg) + (assign32210_e26567 * locals.var_pparam_b4soileffcvbg_dn4)))), ((((locals.var_pparam_b4soikb1_dn5 * locals.var_b4soifbody) * locals.var_cbox) * assign32210_e26571) + (assign32210_e26562 * ((((locals.var_pparam_b4soiweffcv_dn5 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcvbg) + (assign32210_e26567 * locals.var_pparam_b4soileffcvbg_dn5)))), ((((locals.var_pparam_b4soikb1_dn6 * locals.var_b4soifbody) * locals.var_cbox) * assign32210_e26571) + (assign32210_e26562 * ((((locals.var_pparam_b4soiweffcv_dn6 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcvbg) + (assign32210_e26567 * locals.var_pparam_b4soileffcvbg_dn6)))), ((((locals.var_pparam_b4soikb1_dn7 * locals.var_b4soifbody) * locals.var_cbox) * assign32210_e26571) + (assign32210_e26562 * ((((locals.var_pparam_b4soiweffcv_dn7 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcvbg) + (assign32210_e26567 * locals.var_pparam_b4soileffcvbg_dn7)))), ((((locals.var_pparam_b4soikb1_dn8 * locals.var_b4soifbody) * locals.var_cbox) * assign32210_e26571) + (assign32210_e26562 * ((((locals.var_pparam_b4soiweffcv_dn8 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcvbg) + (assign32210_e26567 * locals.var_pparam_b4soileffcvbg_dn8)))), ((((locals.var_pparam_b4soikb1_dn9 * locals.var_b4soifbody) * locals.var_cbox) * assign32210_e26571) + (assign32210_e26562 * ((((locals.var_pparam_b4soiweffcv_dn9 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcvbg) + (assign32210_e26567 * locals.var_pparam_b4soileffcvbg_dn9)))), ((((locals.var_pparam_b4soikb1_dn10 * locals.var_b4soifbody) * locals.var_cbox) * assign32210_e26571) + (assign32210_e26562 * ((((locals.var_pparam_b4soiweffcv_dn10 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcvbg) + (assign32210_e26567 * locals.var_pparam_b4soileffcvbg_dn10)))), ((((locals.var_pparam_b4soikb1_dn11 * locals.var_b4soifbody) * locals.var_cbox) * assign32210_e26571) + (assign32210_e26562 * ((((locals.var_pparam_b4soiweffcv_dn11 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcvbg) + (assign32210_e26567 * locals.var_pparam_b4soileffcvbg_dn11)))), ((((locals.var_pparam_b4soikb1_dn12 * locals.var_b4soifbody) * locals.var_cbox) * assign32210_e26571) + (assign32210_e26562 * ((((locals.var_pparam_b4soiweffcv_dn12 / locals.var_b4soinseg) * locals.var_b4soinf) * locals.var_pparam_b4soileffcvbg) + (assign32210_e26567 * locals.var_pparam_b4soileffcvbg_dn12)))),)
    } else {
        (locals.var_cboxwl, locals.var_cboxwl_dn3, locals.var_cboxwl_dn4, locals.var_cboxwl_dn5, locals.var_cboxwl_dn6, locals.var_cboxwl_dn7, locals.var_cboxwl_dn8, locals.var_cboxwl_dn9, locals.var_cboxwl_dn10, locals.var_cboxwl_dn11, locals.var_cboxwl_dn12,)
    }
};
        locals.var_cboxwl = assign32210_e26574;
        locals.var_cboxwl_dn3 = assign32210_e26574_d_n3;
        locals.var_cboxwl_dn4 = assign32210_e26574_d_n4;
        locals.var_cboxwl_dn5 = assign32210_e26574_d_n5;
        locals.var_cboxwl_dn6 = assign32210_e26574_d_n6;
        locals.var_cboxwl_dn7 = assign32210_e26574_d_n7;
        locals.var_cboxwl_dn8 = assign32210_e26574_d_n8;
        locals.var_cboxwl_dn9 = assign32210_e26574_d_n9;
        locals.var_cboxwl_dn10 = assign32210_e26574_d_n10;
        locals.var_cboxwl_dn11 = assign32210_e26574_d_n11;
        locals.var_cboxwl_dn12 = assign32210_e26574_d_n12;

        let (assign32220_e26588, assign32220_e26588_d_n3, assign32220_e26588_d_n4, assign32220_e26588_d_n5, assign32220_e26588_d_n6, assign32220_e26588_d_n7, assign32220_e26588_d_n8, assign32220_e26588_d_n9, assign32220_e26588_d_n10, assign32220_e26588_d_n11, assign32220_e26588_d_n12,) = {
    if (((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) && (locals.var_guard1749 == 0.0)) {
        let assign32220_e26585: f64 = (locals.var_vesfb - locals.var_vbs_1);
        let assign32220_e26586: f64 = (locals.var_cboxwl * assign32220_e26585);
        (assign32220_e26586, ((locals.var_cboxwl_dn3 * assign32220_e26585) + (locals.var_cboxwl * (locals.var_vesfb_dn3 - locals.var_vbs_1_dn3))), ((locals.var_cboxwl_dn4 * assign32220_e26585) + (locals.var_cboxwl * (locals.var_vesfb_dn4 - locals.var_vbs_1_dn4))), ((locals.var_cboxwl_dn5 * assign32220_e26585) + (locals.var_cboxwl * (locals.var_vesfb_dn5 - locals.var_vbs_1_dn5))), ((locals.var_cboxwl_dn6 * assign32220_e26585) + (locals.var_cboxwl * (locals.var_vesfb_dn6 - locals.var_vbs_1_dn6))), ((locals.var_cboxwl_dn7 * assign32220_e26585) + (locals.var_cboxwl * (locals.var_vesfb_dn7 - locals.var_vbs_1_dn7))), ((locals.var_cboxwl_dn8 * assign32220_e26585) + (locals.var_cboxwl * (locals.var_vesfb_dn8 - locals.var_vbs_1_dn8))), ((locals.var_cboxwl_dn9 * assign32220_e26585) + (locals.var_cboxwl * (locals.var_vesfb_dn9 - locals.var_vbs_1_dn9))), ((locals.var_cboxwl_dn10 * assign32220_e26585) + (locals.var_cboxwl * (locals.var_vesfb_dn10 - locals.var_vbs_1_dn10))), ((locals.var_cboxwl_dn11 * assign32220_e26585) + (locals.var_cboxwl * (locals.var_vesfb_dn11 - locals.var_vbs_1_dn11))), ((locals.var_cboxwl_dn12 * assign32220_e26585) + (locals.var_cboxwl * (locals.var_vesfb_dn12 - locals.var_vbs_1_dn12))),)
    } else {
        (locals.var_qe1, locals.var_qe1_dn3, locals.var_qe1_dn4, locals.var_qe1_dn5, locals.var_qe1_dn6, locals.var_qe1_dn7, locals.var_qe1_dn8, locals.var_qe1_dn9, locals.var_qe1_dn10, locals.var_qe1_dn11, locals.var_qe1_dn12,)
    }
};
        locals.var_qe1 = assign32220_e26588;
        locals.var_qe1_dn3 = assign32220_e26588_d_n3;
        locals.var_qe1_dn4 = assign32220_e26588_d_n4;
        locals.var_qe1_dn5 = assign32220_e26588_d_n5;
        locals.var_qe1_dn6 = assign32220_e26588_d_n6;
        locals.var_qe1_dn7 = assign32220_e26588_d_n7;
        locals.var_qe1_dn8 = assign32220_e26588_d_n8;
        locals.var_qe1_dn9 = assign32220_e26588_d_n9;
        locals.var_qe1_dn10 = assign32220_e26588_d_n10;
        locals.var_qe1_dn11 = assign32220_e26588_d_n11;
        locals.var_qe1_dn12 = assign32220_e26588_d_n12;

        let (assign32230_e26601, assign32230_e26601_d_n3, assign32230_e26601_d_n4, assign32230_e26601_d_n5, assign32230_e26601_d_n6, assign32230_e26601_d_n7, assign32230_e26601_d_n8, assign32230_e26601_d_n9, assign32230_e26601_d_n10, assign32230_e26601_d_n11, assign32230_e26601_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) {
        let assign32230_e26595: f64 = (locals.var_qgate + locals.var_qac0);
        let assign32230_e26597: f64 = (assign32230_e26595 + locals.var_qsub0);
        let assign32230_e26599: f64 = (assign32230_e26597 - locals.var_qbulk);
        (assign32230_e26599, (((locals.var_qgate_dn3 + locals.var_qac0_dn3) + locals.var_qsub0_dn3) - locals.var_qbulk_dn3), (((locals.var_qgate_dn4 + locals.var_qac0_dn4) + locals.var_qsub0_dn4) - locals.var_qbulk_dn4), (((locals.var_qgate_dn5 + locals.var_qac0_dn5) + locals.var_qsub0_dn5) - locals.var_qbulk_dn5), (((locals.var_qgate_dn6 + locals.var_qac0_dn6) + locals.var_qsub0_dn6) - locals.var_qbulk_dn6), (((locals.var_qgate_dn7 + locals.var_qac0_dn7) + locals.var_qsub0_dn7) - locals.var_qbulk_dn7), (((locals.var_qgate_dn8 + locals.var_qac0_dn8) + locals.var_qsub0_dn8) - locals.var_qbulk_dn8), (((locals.var_qgate_dn9 + locals.var_qac0_dn9) + locals.var_qsub0_dn9) - locals.var_qbulk_dn9), (((locals.var_qgate_dn10 + locals.var_qac0_dn10) + locals.var_qsub0_dn10) - locals.var_qbulk_dn10), (((locals.var_qgate_dn11 + locals.var_qac0_dn11) + locals.var_qsub0_dn11) - locals.var_qbulk_dn11), (((locals.var_qgate_dn12 + locals.var_qac0_dn12) + locals.var_qsub0_dn12) - locals.var_qbulk_dn12),)
    } else {
        (locals.var_qgate, locals.var_qgate_dn3, locals.var_qgate_dn4, locals.var_qgate_dn5, locals.var_qgate_dn6, locals.var_qgate_dn7, locals.var_qgate_dn8, locals.var_qgate_dn9, locals.var_qgate_dn10, locals.var_qgate_dn11, locals.var_qgate_dn12,)
    }
};
        locals.var_qgate = assign32230_e26601;
        locals.var_qgate_dn3 = assign32230_e26601_d_n3;
        locals.var_qgate_dn4 = assign32230_e26601_d_n4;
        locals.var_qgate_dn5 = assign32230_e26601_d_n5;
        locals.var_qgate_dn6 = assign32230_e26601_d_n6;
        locals.var_qgate_dn7 = assign32230_e26601_d_n7;
        locals.var_qgate_dn8 = assign32230_e26601_d_n8;
        locals.var_qgate_dn9 = assign32230_e26601_d_n9;
        locals.var_qgate_dn10 = assign32230_e26601_d_n10;
        locals.var_qgate_dn11 = assign32230_e26601_d_n11;
        locals.var_qgate_dn12 = assign32230_e26601_d_n12;

        let (assign32240_e26614, assign32240_e26614_d_n3, assign32240_e26614_d_n4, assign32240_e26614_d_n5, assign32240_e26614_d_n6, assign32240_e26614_d_n7, assign32240_e26614_d_n8, assign32240_e26614_d_n9, assign32240_e26614_d_n10, assign32240_e26614_d_n11, assign32240_e26614_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) {
        let assign32240_e26608: f64 = (locals.var_qbulk - locals.var_qac0);
        let assign32240_e26610: f64 = (assign32240_e26608 - locals.var_qsub0);
        let assign32240_e26612: f64 = (assign32240_e26610 - locals.var_qe1);
        (assign32240_e26612, (((locals.var_qbulk_dn3 - locals.var_qac0_dn3) - locals.var_qsub0_dn3) - locals.var_qe1_dn3), (((locals.var_qbulk_dn4 - locals.var_qac0_dn4) - locals.var_qsub0_dn4) - locals.var_qe1_dn4), (((locals.var_qbulk_dn5 - locals.var_qac0_dn5) - locals.var_qsub0_dn5) - locals.var_qe1_dn5), (((locals.var_qbulk_dn6 - locals.var_qac0_dn6) - locals.var_qsub0_dn6) - locals.var_qe1_dn6), (((locals.var_qbulk_dn7 - locals.var_qac0_dn7) - locals.var_qsub0_dn7) - locals.var_qe1_dn7), (((locals.var_qbulk_dn8 - locals.var_qac0_dn8) - locals.var_qsub0_dn8) - locals.var_qe1_dn8), (((locals.var_qbulk_dn9 - locals.var_qac0_dn9) - locals.var_qsub0_dn9) - locals.var_qe1_dn9), (((locals.var_qbulk_dn10 - locals.var_qac0_dn10) - locals.var_qsub0_dn10) - locals.var_qe1_dn10), (((locals.var_qbulk_dn11 - locals.var_qac0_dn11) - locals.var_qsub0_dn11) - locals.var_qe1_dn11), (((locals.var_qbulk_dn12 - locals.var_qac0_dn12) - locals.var_qsub0_dn12) - locals.var_qe1_dn12),)
    } else {
        (locals.var_qbody, locals.var_qbody_dn3, locals.var_qbody_dn4, locals.var_qbody_dn5, locals.var_qbody_dn6, locals.var_qbody_dn7, locals.var_qbody_dn8, locals.var_qbody_dn9, locals.var_qbody_dn10, locals.var_qbody_dn11, locals.var_qbody_dn12,)
    }
};
        locals.var_qbody = assign32240_e26614;
        locals.var_qbody_dn3 = assign32240_e26614_d_n3;
        locals.var_qbody_dn4 = assign32240_e26614_d_n4;
        locals.var_qbody_dn5 = assign32240_e26614_d_n5;
        locals.var_qbody_dn6 = assign32240_e26614_d_n6;
        locals.var_qbody_dn7 = assign32240_e26614_d_n7;
        locals.var_qbody_dn8 = assign32240_e26614_d_n8;
        locals.var_qbody_dn9 = assign32240_e26614_d_n9;
        locals.var_qbody_dn10 = assign32240_e26614_d_n10;
        locals.var_qbody_dn11 = assign32240_e26614_d_n11;
        locals.var_qbody_dn12 = assign32240_e26614_d_n12;

        let (assign32250_e26621, assign32250_e26621_d_n3, assign32250_e26621_d_n4, assign32250_e26621_d_n5, assign32250_e26621_d_n6, assign32250_e26621_d_n7, assign32250_e26621_d_n8, assign32250_e26621_d_n9, assign32250_e26621_d_n10, assign32250_e26621_d_n11, assign32250_e26621_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) {
        (locals.var_qe1, locals.var_qe1_dn3, locals.var_qe1_dn4, locals.var_qe1_dn5, locals.var_qe1_dn6, locals.var_qe1_dn7, locals.var_qe1_dn8, locals.var_qe1_dn9, locals.var_qe1_dn10, locals.var_qe1_dn11, locals.var_qe1_dn12,)
    } else {
        (locals.var_qsub, locals.var_qsub_dn3, locals.var_qsub_dn4, locals.var_qsub_dn5, locals.var_qsub_dn6, locals.var_qsub_dn7, locals.var_qsub_dn8, locals.var_qsub_dn9, locals.var_qsub_dn10, locals.var_qsub_dn11, locals.var_qsub_dn12,)
    }
};
        locals.var_qsub = assign32250_e26621;
        locals.var_qsub_dn3 = assign32250_e26621_d_n3;
        locals.var_qsub_dn4 = assign32250_e26621_d_n4;
        locals.var_qsub_dn5 = assign32250_e26621_d_n5;
        locals.var_qsub_dn6 = assign32250_e26621_d_n6;
        locals.var_qsub_dn7 = assign32250_e26621_d_n7;
        locals.var_qsub_dn8 = assign32250_e26621_d_n8;
        locals.var_qsub_dn9 = assign32250_e26621_d_n9;
        locals.var_qsub_dn10 = assign32250_e26621_d_n10;
        locals.var_qsub_dn11 = assign32250_e26621_d_n11;
        locals.var_qsub_dn12 = assign32250_e26621_d_n12;

        let (assign32260_e26635, assign32260_e26635_d_n3, assign32260_e26635_d_n4, assign32260_e26635_d_n5, assign32260_e26635_d_n6, assign32260_e26635_d_n7, assign32260_e26635_d_n8, assign32260_e26635_d_n9, assign32260_e26635_d_n10, assign32260_e26635_d_n11, assign32260_e26635_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 != 0.0)) {
        let assign32260_e26628: f64 = (locals.var_qgate + locals.var_qbody);
        let assign32260_e26630: f64 = (assign32260_e26628 + locals.var_qsub);
        let assign32260_e26632: f64 = (assign32260_e26630 + locals.var_qsrc);
        let assign32260_e26633: f64 = (-assign32260_e26632);
        (assign32260_e26633, (-(((locals.var_qgate_dn3 + locals.var_qbody_dn3) + locals.var_qsub_dn3) + locals.var_qsrc_dn3)), (-(((locals.var_qgate_dn4 + locals.var_qbody_dn4) + locals.var_qsub_dn4) + locals.var_qsrc_dn4)), (-(((locals.var_qgate_dn5 + locals.var_qbody_dn5) + locals.var_qsub_dn5) + locals.var_qsrc_dn5)), (-(((locals.var_qgate_dn6 + locals.var_qbody_dn6) + locals.var_qsub_dn6) + locals.var_qsrc_dn6)), (-(((locals.var_qgate_dn7 + locals.var_qbody_dn7) + locals.var_qsub_dn7) + locals.var_qsrc_dn7)), (-(((locals.var_qgate_dn8 + locals.var_qbody_dn8) + locals.var_qsub_dn8) + locals.var_qsrc_dn8)), (-(((locals.var_qgate_dn9 + locals.var_qbody_dn9) + locals.var_qsub_dn9) + locals.var_qsrc_dn9)), (-(((locals.var_qgate_dn10 + locals.var_qbody_dn10) + locals.var_qsub_dn10) + locals.var_qsrc_dn10)), (-(((locals.var_qgate_dn11 + locals.var_qbody_dn11) + locals.var_qsub_dn11) + locals.var_qsrc_dn11)), (-(((locals.var_qgate_dn12 + locals.var_qbody_dn12) + locals.var_qsub_dn12) + locals.var_qsrc_dn12)),)
    } else {
        (locals.var_qdrn, locals.var_qdrn_dn3, locals.var_qdrn_dn4, locals.var_qdrn_dn5, locals.var_qdrn_dn6, locals.var_qdrn_dn7, locals.var_qdrn_dn8, locals.var_qdrn_dn9, locals.var_qdrn_dn10, locals.var_qdrn_dn11, locals.var_qdrn_dn12,)
    }
};
        locals.var_qdrn = assign32260_e26635;
        locals.var_qdrn_dn3 = assign32260_e26635_d_n3;
        locals.var_qdrn_dn4 = assign32260_e26635_d_n4;
        locals.var_qdrn_dn5 = assign32260_e26635_d_n5;
        locals.var_qdrn_dn6 = assign32260_e26635_d_n6;
        locals.var_qdrn_dn7 = assign32260_e26635_d_n7;
        locals.var_qdrn_dn8 = assign32260_e26635_d_n8;
        locals.var_qdrn_dn9 = assign32260_e26635_d_n9;
        locals.var_qdrn_dn10 = assign32260_e26635_d_n10;
        locals.var_qdrn_dn11 = assign32260_e26635_d_n11;
        locals.var_qdrn_dn12 = assign32260_e26635_d_n12;

        let (assign32280_e26651, assign32280_e26651_d_n3, assign32280_e26651_d_n4, assign32280_e26651_d_n5, assign32280_e26651_d_n6, assign32280_e26651_d_n7, assign32280_e26651_d_n8, assign32280_e26651_d_n9, assign32280_e26651_d_n10, assign32280_e26651_d_n11, assign32280_e26651_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qac0, locals.var_qac0_dn3, locals.var_qac0_dn4, locals.var_qac0_dn5, locals.var_qac0_dn6, locals.var_qac0_dn7, locals.var_qac0_dn8, locals.var_qac0_dn9, locals.var_qac0_dn10, locals.var_qac0_dn11, locals.var_qac0_dn12,)
    }
};
        locals.var_qac0 = assign32280_e26651;
        locals.var_qac0_dn3 = assign32280_e26651_d_n3;
        locals.var_qac0_dn4 = assign32280_e26651_d_n4;
        locals.var_qac0_dn5 = assign32280_e26651_d_n5;
        locals.var_qac0_dn6 = assign32280_e26651_d_n6;
        locals.var_qac0_dn7 = assign32280_e26651_d_n7;
        locals.var_qac0_dn8 = assign32280_e26651_d_n8;
        locals.var_qac0_dn9 = assign32280_e26651_d_n9;
        locals.var_qac0_dn10 = assign32280_e26651_d_n10;
        locals.var_qac0_dn11 = assign32280_e26651_d_n11;
        locals.var_qac0_dn12 = assign32280_e26651_d_n12;

        let (assign32290_e26659, assign32290_e26659_d_n3, assign32290_e26659_d_n4, assign32290_e26659_d_n5, assign32290_e26659_d_n6, assign32290_e26659_d_n7, assign32290_e26659_d_n8, assign32290_e26659_d_n9, assign32290_e26659_d_n10, assign32290_e26659_d_n11, assign32290_e26659_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qsub0, locals.var_qsub0_dn3, locals.var_qsub0_dn4, locals.var_qsub0_dn5, locals.var_qsub0_dn6, locals.var_qsub0_dn7, locals.var_qsub0_dn8, locals.var_qsub0_dn9, locals.var_qsub0_dn10, locals.var_qsub0_dn11, locals.var_qsub0_dn12,)
    }
};
        locals.var_qsub0 = assign32290_e26659;
        locals.var_qsub0_dn3 = assign32290_e26659_d_n3;
        locals.var_qsub0_dn4 = assign32290_e26659_d_n4;
        locals.var_qsub0_dn5 = assign32290_e26659_d_n5;
        locals.var_qsub0_dn6 = assign32290_e26659_d_n6;
        locals.var_qsub0_dn7 = assign32290_e26659_d_n7;
        locals.var_qsub0_dn8 = assign32290_e26659_d_n8;
        locals.var_qsub0_dn9 = assign32290_e26659_d_n9;
        locals.var_qsub0_dn10 = assign32290_e26659_d_n10;
        locals.var_qsub0_dn11 = assign32290_e26659_d_n11;
        locals.var_qsub0_dn12 = assign32290_e26659_d_n12;

    }

    pub(super) fn stamp_transient_block_89(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign32300_e26667, assign32300_e26667_d_n3, assign32300_e26667_d_n4, assign32300_e26667_d_n5, assign32300_e26667_d_n6, assign32300_e26667_d_n7, assign32300_e26667_d_n8, assign32300_e26667_d_n9, assign32300_e26667_d_n10, assign32300_e26667_d_n11, assign32300_e26667_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qsub, locals.var_qsub_dn3, locals.var_qsub_dn4, locals.var_qsub_dn5, locals.var_qsub_dn6, locals.var_qsub_dn7, locals.var_qsub_dn8, locals.var_qsub_dn9, locals.var_qsub_dn10, locals.var_qsub_dn11, locals.var_qsub_dn12,)
    }
};
        locals.var_qsub = assign32300_e26667;
        locals.var_qsub_dn3 = assign32300_e26667_d_n3;
        locals.var_qsub_dn4 = assign32300_e26667_d_n4;
        locals.var_qsub_dn5 = assign32300_e26667_d_n5;
        locals.var_qsub_dn6 = assign32300_e26667_d_n6;
        locals.var_qsub_dn7 = assign32300_e26667_d_n7;
        locals.var_qsub_dn8 = assign32300_e26667_d_n8;
        locals.var_qsub_dn9 = assign32300_e26667_d_n9;
        locals.var_qsub_dn10 = assign32300_e26667_d_n10;
        locals.var_qsub_dn11 = assign32300_e26667_d_n11;
        locals.var_qsub_dn12 = assign32300_e26667_d_n12;

        let (assign32310_e26675, assign32310_e26675_d_n3, assign32310_e26675_d_n4, assign32310_e26675_d_n5, assign32310_e26675_d_n6, assign32310_e26675_d_n7, assign32310_e26675_d_n8, assign32310_e26675_d_n9, assign32310_e26675_d_n10, assign32310_e26675_d_n11, assign32310_e26675_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbody, locals.var_qbody_dn3, locals.var_qbody_dn4, locals.var_qbody_dn5, locals.var_qbody_dn6, locals.var_qbody_dn7, locals.var_qbody_dn8, locals.var_qbody_dn9, locals.var_qbody_dn10, locals.var_qbody_dn11, locals.var_qbody_dn12,)
    }
};
        locals.var_qbody = assign32310_e26675;
        locals.var_qbody_dn3 = assign32310_e26675_d_n3;
        locals.var_qbody_dn4 = assign32310_e26675_d_n4;
        locals.var_qbody_dn5 = assign32310_e26675_d_n5;
        locals.var_qbody_dn6 = assign32310_e26675_d_n6;
        locals.var_qbody_dn7 = assign32310_e26675_d_n7;
        locals.var_qbody_dn8 = assign32310_e26675_d_n8;
        locals.var_qbody_dn9 = assign32310_e26675_d_n9;
        locals.var_qbody_dn10 = assign32310_e26675_d_n10;
        locals.var_qbody_dn11 = assign32310_e26675_d_n11;
        locals.var_qbody_dn12 = assign32310_e26675_d_n12;

        let (assign32320_e26683, assign32320_e26683_d_n3, assign32320_e26683_d_n4, assign32320_e26683_d_n5, assign32320_e26683_d_n6, assign32320_e26683_d_n7, assign32320_e26683_d_n8, assign32320_e26683_d_n9, assign32320_e26683_d_n10, assign32320_e26683_d_n11, assign32320_e26683_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qsrc, locals.var_qsrc_dn3, locals.var_qsrc_dn4, locals.var_qsrc_dn5, locals.var_qsrc_dn6, locals.var_qsrc_dn7, locals.var_qsrc_dn8, locals.var_qsrc_dn9, locals.var_qsrc_dn10, locals.var_qsrc_dn11, locals.var_qsrc_dn12,)
    }
};
        locals.var_qsrc = assign32320_e26683;
        locals.var_qsrc_dn3 = assign32320_e26683_d_n3;
        locals.var_qsrc_dn4 = assign32320_e26683_d_n4;
        locals.var_qsrc_dn5 = assign32320_e26683_d_n5;
        locals.var_qsrc_dn6 = assign32320_e26683_d_n6;
        locals.var_qsrc_dn7 = assign32320_e26683_d_n7;
        locals.var_qsrc_dn8 = assign32320_e26683_d_n8;
        locals.var_qsrc_dn9 = assign32320_e26683_d_n9;
        locals.var_qsrc_dn10 = assign32320_e26683_d_n10;
        locals.var_qsrc_dn11 = assign32320_e26683_d_n11;
        locals.var_qsrc_dn12 = assign32320_e26683_d_n12;

        let (assign32330_e26691, assign32330_e26691_d_n3, assign32330_e26691_d_n4, assign32330_e26691_d_n5, assign32330_e26691_d_n6, assign32330_e26691_d_n7, assign32330_e26691_d_n8, assign32330_e26691_d_n9, assign32330_e26691_d_n10, assign32330_e26691_d_n11, assign32330_e26691_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qdrn, locals.var_qdrn_dn3, locals.var_qdrn_dn4, locals.var_qdrn_dn5, locals.var_qdrn_dn6, locals.var_qdrn_dn7, locals.var_qdrn_dn8, locals.var_qdrn_dn9, locals.var_qdrn_dn10, locals.var_qdrn_dn11, locals.var_qdrn_dn12,)
    }
};
        locals.var_qdrn = assign32330_e26691;
        locals.var_qdrn_dn3 = assign32330_e26691_d_n3;
        locals.var_qdrn_dn4 = assign32330_e26691_d_n4;
        locals.var_qdrn_dn5 = assign32330_e26691_d_n5;
        locals.var_qdrn_dn6 = assign32330_e26691_d_n6;
        locals.var_qdrn_dn7 = assign32330_e26691_d_n7;
        locals.var_qdrn_dn8 = assign32330_e26691_d_n8;
        locals.var_qdrn_dn9 = assign32330_e26691_d_n9;
        locals.var_qdrn_dn10 = assign32330_e26691_d_n10;
        locals.var_qdrn_dn11 = assign32330_e26691_d_n11;
        locals.var_qdrn_dn12 = assign32330_e26691_d_n12;

        let (assign32340_e26699, assign32340_e26699_d_n3, assign32340_e26699_d_n4, assign32340_e26699_d_n5, assign32340_e26699_d_n6, assign32340_e26699_d_n7, assign32340_e26699_d_n8, assign32340_e26699_d_n9, assign32340_e26699_d_n10, assign32340_e26699_d_n11, assign32340_e26699_d_n12,) = {
    if ((locals.var_guard1698 == 0.0) && (locals.var_guard1716 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qgate, locals.var_qgate_dn3, locals.var_qgate_dn4, locals.var_qgate_dn5, locals.var_qgate_dn6, locals.var_qgate_dn7, locals.var_qgate_dn8, locals.var_qgate_dn9, locals.var_qgate_dn10, locals.var_qgate_dn11, locals.var_qgate_dn12,)
    }
};
        locals.var_qgate = assign32340_e26699;
        locals.var_qgate_dn3 = assign32340_e26699_d_n3;
        locals.var_qgate_dn4 = assign32340_e26699_d_n4;
        locals.var_qgate_dn5 = assign32340_e26699_d_n5;
        locals.var_qgate_dn6 = assign32340_e26699_d_n6;
        locals.var_qgate_dn7 = assign32340_e26699_d_n7;
        locals.var_qgate_dn8 = assign32340_e26699_d_n8;
        locals.var_qgate_dn9 = assign32340_e26699_d_n9;
        locals.var_qgate_dn10 = assign32340_e26699_d_n10;
        locals.var_qgate_dn11 = assign32340_e26699_d_n11;
        locals.var_qgate_dn12 = assign32340_e26699_d_n12;

        let assign32350_e26702: f64 = if locals.var_b4soisoimod == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1750 = assign32350_e26702;

        let (assign32360_e26706, assign32360_e26706_d_n3, assign32360_e26706_d_n4, assign32360_e26706_d_n5, assign32360_e26706_d_n6, assign32360_e26706_d_n7, assign32360_e26706_d_n8, assign32360_e26706_d_n9, assign32360_e26706_d_n10, assign32360_e26706_d_n11, assign32360_e26706_d_n12,) = {
    if (locals.var_guard1750 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qjd_1, locals.var_qjd_1_dn3, locals.var_qjd_1_dn4, locals.var_qjd_1_dn5, locals.var_qjd_1_dn6, locals.var_qjd_1_dn7, locals.var_qjd_1_dn8, locals.var_qjd_1_dn9, locals.var_qjd_1_dn10, locals.var_qjd_1_dn11, locals.var_qjd_1_dn12,)
    }
};
        locals.var_qjd_1 = assign32360_e26706;
        locals.var_qjd_1_dn3 = assign32360_e26706_d_n3;
        locals.var_qjd_1_dn4 = assign32360_e26706_d_n4;
        locals.var_qjd_1_dn5 = assign32360_e26706_d_n5;
        locals.var_qjd_1_dn6 = assign32360_e26706_d_n6;
        locals.var_qjd_1_dn7 = assign32360_e26706_d_n7;
        locals.var_qjd_1_dn8 = assign32360_e26706_d_n8;
        locals.var_qjd_1_dn9 = assign32360_e26706_d_n9;
        locals.var_qjd_1_dn10 = assign32360_e26706_d_n10;
        locals.var_qjd_1_dn11 = assign32360_e26706_d_n11;
        locals.var_qjd_1_dn12 = assign32360_e26706_d_n12;

        let (assign32370_e26710, assign32370_e26710_d_n3, assign32370_e26710_d_n4, assign32370_e26710_d_n5, assign32370_e26710_d_n6, assign32370_e26710_d_n7, assign32370_e26710_d_n8, assign32370_e26710_d_n9, assign32370_e26710_d_n10, assign32370_e26710_d_n11, assign32370_e26710_d_n12,) = {
    if (locals.var_guard1750 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qjs_1, locals.var_qjs_1_dn3, locals.var_qjs_1_dn4, locals.var_qjs_1_dn5, locals.var_qjs_1_dn6, locals.var_qjs_1_dn7, locals.var_qjs_1_dn8, locals.var_qjs_1_dn9, locals.var_qjs_1_dn10, locals.var_qjs_1_dn11, locals.var_qjs_1_dn12,)
    }
};
        locals.var_qjs_1 = assign32370_e26710;
        locals.var_qjs_1_dn3 = assign32370_e26710_d_n3;
        locals.var_qjs_1_dn4 = assign32370_e26710_d_n4;
        locals.var_qjs_1_dn5 = assign32370_e26710_d_n5;
        locals.var_qjs_1_dn6 = assign32370_e26710_d_n6;
        locals.var_qjs_1_dn7 = assign32370_e26710_d_n7;
        locals.var_qjs_1_dn8 = assign32370_e26710_d_n8;
        locals.var_qjs_1_dn9 = assign32370_e26710_d_n9;
        locals.var_qjs_1_dn10 = assign32370_e26710_d_n10;
        locals.var_qjs_1_dn11 = assign32370_e26710_d_n11;
        locals.var_qjs_1_dn12 = assign32370_e26710_d_n12;

        let (assign32380_e26715, assign32380_e26715_d_n6,) = {
    if (locals.var_guard1750 == 0.0) {
        (locals.var_b4soigatesidewalljctspotential, 0.0,)
    } else {
        (locals.var_phibswg, locals.var_phibswg_dn6,)
    }
};
        locals.var_phibswg = assign32380_e26715;
        locals.var_phibswg_dn6 = assign32380_e26715_d_n6;

        let (assign32390_e26721,) = {
    if (locals.var_guard1750 == 0.0) {
        let assign32390_e26719: f64 = (-locals.var_b4soitpbswg);
        (assign32390_e26719,)
    } else {
        (locals.var_dphibswg_dt,)
    }
};
        locals.var_dphibswg_dt = assign32390_e26721;

        let (assign32400_e26732, assign32400_e26732_d_n6,) = {
    if (locals.var_guard1750 == 0.0) {
        let assign32400_e26728: f64 = (locals.var_devtemp - locals.var_b4soitnom);
        let assign32400_e26729: f64 = (locals.var_dphibswg_dt * assign32400_e26728);
        let assign32400_e26730: f64 = (locals.var_phibswg + assign32400_e26729);
        (assign32400_e26730, (locals.var_phibswg_dn6 + (locals.var_dphibswg_dt * locals.var_devtemp_dn6)),)
    } else {
        (locals.var_phibswg, locals.var_phibswg_dn6,)
    }
};
        locals.var_phibswg = assign32400_e26732;
        locals.var_phibswg_dn6 = assign32400_e26732_d_n6;

        let (assign32410_e26737,) = {
    if (locals.var_guard1750 == 0.0) {
        (locals.var_b4soibodyjctgatesidesgradingcoeff,)
    } else {
        (p.p173,)
    }
};
        locals.var_mjswg = assign32410_e26737;

        let (assign32420_e26750, assign32420_e26750_d_n3, assign32420_e26750_d_n4, assign32420_e26750_d_n5, assign32420_e26750_d_n6, assign32420_e26750_d_n7, assign32420_e26750_d_n8, assign32420_e26750_d_n9, assign32420_e26750_d_n10, assign32420_e26750_d_n11, assign32420_e26750_d_n12,) = {
    if (locals.var_guard1750 == 0.0) {
        let assign32420_e26742: f64 = (locals.var_b4soiunitlengthgatesidewalljctcaps * locals.var_pparam_b4soiwdioscv);
        let assign32420_e26744: f64 = (assign32420_e26742 * locals.var_b4soitsi);
        let assign32420_e26746: f64 = (assign32420_e26744 * locals.var_b4soinf);
        let assign32420_e26748: f64 = (assign32420_e26746 / 1e-7);
        (assign32420_e26748, ((((locals.var_b4soiunitlengthgatesidewalljctcaps * locals.var_pparam_b4soiwdioscv_dn3) * locals.var_b4soitsi) * locals.var_b4soinf) / 1e-7), ((((locals.var_b4soiunitlengthgatesidewalljctcaps * locals.var_pparam_b4soiwdioscv_dn4) * locals.var_b4soitsi) * locals.var_b4soinf) / 1e-7), ((((locals.var_b4soiunitlengthgatesidewalljctcaps * locals.var_pparam_b4soiwdioscv_dn5) * locals.var_b4soitsi) * locals.var_b4soinf) / 1e-7), ((((locals.var_b4soiunitlengthgatesidewalljctcaps * locals.var_pparam_b4soiwdioscv_dn6) * locals.var_b4soitsi) * locals.var_b4soinf) / 1e-7), ((((locals.var_b4soiunitlengthgatesidewalljctcaps * locals.var_pparam_b4soiwdioscv_dn7) * locals.var_b4soitsi) * locals.var_b4soinf) / 1e-7), ((((locals.var_b4soiunitlengthgatesidewalljctcaps * locals.var_pparam_b4soiwdioscv_dn8) * locals.var_b4soitsi) * locals.var_b4soinf) / 1e-7), ((((locals.var_b4soiunitlengthgatesidewalljctcaps * locals.var_pparam_b4soiwdioscv_dn9) * locals.var_b4soitsi) * locals.var_b4soinf) / 1e-7), ((((locals.var_b4soiunitlengthgatesidewalljctcaps * locals.var_pparam_b4soiwdioscv_dn10) * locals.var_b4soitsi) * locals.var_b4soinf) / 1e-7), ((((locals.var_b4soiunitlengthgatesidewalljctcaps * locals.var_pparam_b4soiwdioscv_dn11) * locals.var_b4soitsi) * locals.var_b4soinf) / 1e-7), ((((locals.var_b4soiunitlengthgatesidewalljctcaps * locals.var_pparam_b4soiwdioscv_dn12) * locals.var_b4soitsi) * locals.var_b4soinf) / 1e-7),)
    } else {
        (locals.var_cjsbs, locals.var_cjsbs_dn3, locals.var_cjsbs_dn4, locals.var_cjsbs_dn5, locals.var_cjsbs_dn6, locals.var_cjsbs_dn7, locals.var_cjsbs_dn8, locals.var_cjsbs_dn9, locals.var_cjsbs_dn10, locals.var_cjsbs_dn11, locals.var_cjsbs_dn12,)
    }
};
        locals.var_cjsbs = assign32420_e26750;
        locals.var_cjsbs_dn3 = assign32420_e26750_d_n3;
        locals.var_cjsbs_dn4 = assign32420_e26750_d_n4;
        locals.var_cjsbs_dn5 = assign32420_e26750_d_n5;
        locals.var_cjsbs_dn6 = assign32420_e26750_d_n6;
        locals.var_cjsbs_dn7 = assign32420_e26750_d_n7;
        locals.var_cjsbs_dn8 = assign32420_e26750_d_n8;
        locals.var_cjsbs_dn9 = assign32420_e26750_d_n9;
        locals.var_cjsbs_dn10 = assign32420_e26750_d_n10;
        locals.var_cjsbs_dn11 = assign32420_e26750_d_n11;
        locals.var_cjsbs_dn12 = assign32420_e26750_d_n12;

        let (assign32430_e26757, assign32430_e26757_d_n3, assign32430_e26757_d_n4, assign32430_e26757_d_n5, assign32430_e26757_d_n6, assign32430_e26757_d_n7, assign32430_e26757_d_n8, assign32430_e26757_d_n9, assign32430_e26757_d_n10, assign32430_e26757_d_n11, assign32430_e26757_d_n12,) = {
    if (locals.var_guard1750 == 0.0) {
        let assign32430_e26755: f64 = (locals.var_cjsbs * locals.var_b4soitcjswg);
        (assign32430_e26755, (locals.var_cjsbs_dn3 * locals.var_b4soitcjswg), (locals.var_cjsbs_dn4 * locals.var_b4soitcjswg), (locals.var_cjsbs_dn5 * locals.var_b4soitcjswg), (locals.var_cjsbs_dn6 * locals.var_b4soitcjswg), (locals.var_cjsbs_dn7 * locals.var_b4soitcjswg), (locals.var_cjsbs_dn8 * locals.var_b4soitcjswg), (locals.var_cjsbs_dn9 * locals.var_b4soitcjswg), (locals.var_cjsbs_dn10 * locals.var_b4soitcjswg), (locals.var_cjsbs_dn11 * locals.var_b4soitcjswg), (locals.var_cjsbs_dn12 * locals.var_b4soitcjswg),)
    } else {
        (locals.var_dcjsbs_dt, locals.var_dcjsbs_dt_dn3, locals.var_dcjsbs_dt_dn4, locals.var_dcjsbs_dt_dn5, locals.var_dcjsbs_dt_dn6, locals.var_dcjsbs_dt_dn7, locals.var_dcjsbs_dt_dn8, locals.var_dcjsbs_dt_dn9, locals.var_dcjsbs_dt_dn10, locals.var_dcjsbs_dt_dn11, locals.var_dcjsbs_dt_dn12,)
    }
};
        locals.var_dcjsbs_dt = assign32430_e26757;
        locals.var_dcjsbs_dt_dn3 = assign32430_e26757_d_n3;
        locals.var_dcjsbs_dt_dn4 = assign32430_e26757_d_n4;
        locals.var_dcjsbs_dt_dn5 = assign32430_e26757_d_n5;
        locals.var_dcjsbs_dt_dn6 = assign32430_e26757_d_n6;
        locals.var_dcjsbs_dt_dn7 = assign32430_e26757_d_n7;
        locals.var_dcjsbs_dt_dn8 = assign32430_e26757_d_n8;
        locals.var_dcjsbs_dt_dn9 = assign32430_e26757_d_n9;
        locals.var_dcjsbs_dt_dn10 = assign32430_e26757_d_n10;
        locals.var_dcjsbs_dt_dn11 = assign32430_e26757_d_n11;
        locals.var_dcjsbs_dt_dn12 = assign32430_e26757_d_n12;

        let (assign32440_e26768, assign32440_e26768_d_n3, assign32440_e26768_d_n4, assign32440_e26768_d_n5, assign32440_e26768_d_n6, assign32440_e26768_d_n7, assign32440_e26768_d_n8, assign32440_e26768_d_n9, assign32440_e26768_d_n10, assign32440_e26768_d_n11, assign32440_e26768_d_n12,) = {
    if (locals.var_guard1750 == 0.0) {
        let assign32440_e26764: f64 = (locals.var_devtemp - locals.var_b4soitnom);
        let assign32440_e26765: f64 = (locals.var_dcjsbs_dt * assign32440_e26764);
        let assign32440_e26766: f64 = (locals.var_cjsbs + assign32440_e26765);
        (assign32440_e26766, (locals.var_cjsbs_dn3 + (locals.var_dcjsbs_dt_dn3 * assign32440_e26764)), (locals.var_cjsbs_dn4 + (locals.var_dcjsbs_dt_dn4 * assign32440_e26764)), (locals.var_cjsbs_dn5 + (locals.var_dcjsbs_dt_dn5 * assign32440_e26764)), (locals.var_cjsbs_dn6 + ((locals.var_dcjsbs_dt_dn6 * assign32440_e26764) + (locals.var_dcjsbs_dt * locals.var_devtemp_dn6))), (locals.var_cjsbs_dn7 + (locals.var_dcjsbs_dt_dn7 * assign32440_e26764)), (locals.var_cjsbs_dn8 + (locals.var_dcjsbs_dt_dn8 * assign32440_e26764)), (locals.var_cjsbs_dn9 + (locals.var_dcjsbs_dt_dn9 * assign32440_e26764)), (locals.var_cjsbs_dn10 + (locals.var_dcjsbs_dt_dn10 * assign32440_e26764)), (locals.var_cjsbs_dn11 + (locals.var_dcjsbs_dt_dn11 * assign32440_e26764)), (locals.var_cjsbs_dn12 + (locals.var_dcjsbs_dt_dn12 * assign32440_e26764)),)
    } else {
        (locals.var_cjsbs, locals.var_cjsbs_dn3, locals.var_cjsbs_dn4, locals.var_cjsbs_dn5, locals.var_cjsbs_dn6, locals.var_cjsbs_dn7, locals.var_cjsbs_dn8, locals.var_cjsbs_dn9, locals.var_cjsbs_dn10, locals.var_cjsbs_dn11, locals.var_cjsbs_dn12,)
    }
};
        locals.var_cjsbs = assign32440_e26768;
        locals.var_cjsbs_dn3 = assign32440_e26768_d_n3;
        locals.var_cjsbs_dn4 = assign32440_e26768_d_n4;
        locals.var_cjsbs_dn5 = assign32440_e26768_d_n5;
        locals.var_cjsbs_dn6 = assign32440_e26768_d_n6;
        locals.var_cjsbs_dn7 = assign32440_e26768_d_n7;
        locals.var_cjsbs_dn8 = assign32440_e26768_d_n8;
        locals.var_cjsbs_dn9 = assign32440_e26768_d_n9;
        locals.var_cjsbs_dn10 = assign32440_e26768_d_n10;
        locals.var_cjsbs_dn11 = assign32440_e26768_d_n11;
        locals.var_cjsbs_dn12 = assign32440_e26768_d_n12;

        let (assign32450_e26781, assign32450_e26781_d_n3, assign32450_e26781_d_n4, assign32450_e26781_d_n5, assign32450_e26781_d_n6, assign32450_e26781_d_n7, assign32450_e26781_d_n8, assign32450_e26781_d_n9, assign32450_e26781_d_n10, assign32450_e26781_d_n11, assign32450_e26781_d_n12,) = {
    if (locals.var_guard1750 == 0.0) {
        let assign32450_e26773: f64 = (locals.var_b4soiunitlengthgatesidewalljctcapd * locals.var_pparam_b4soiwdiodcv);
        let assign32450_e26775: f64 = (assign32450_e26773 * locals.var_b4soitsi);
        let assign32450_e26777: f64 = (assign32450_e26775 * locals.var_b4soinf);
        let assign32450_e26779: f64 = (assign32450_e26777 / 1e-7);
        (assign32450_e26779, ((((locals.var_b4soiunitlengthgatesidewalljctcapd * locals.var_pparam_b4soiwdiodcv_dn3) * locals.var_b4soitsi) * locals.var_b4soinf) / 1e-7), ((((locals.var_b4soiunitlengthgatesidewalljctcapd * locals.var_pparam_b4soiwdiodcv_dn4) * locals.var_b4soitsi) * locals.var_b4soinf) / 1e-7), ((((locals.var_b4soiunitlengthgatesidewalljctcapd * locals.var_pparam_b4soiwdiodcv_dn5) * locals.var_b4soitsi) * locals.var_b4soinf) / 1e-7), ((((locals.var_b4soiunitlengthgatesidewalljctcapd * locals.var_pparam_b4soiwdiodcv_dn6) * locals.var_b4soitsi) * locals.var_b4soinf) / 1e-7), ((((locals.var_b4soiunitlengthgatesidewalljctcapd * locals.var_pparam_b4soiwdiodcv_dn7) * locals.var_b4soitsi) * locals.var_b4soinf) / 1e-7), ((((locals.var_b4soiunitlengthgatesidewalljctcapd * locals.var_pparam_b4soiwdiodcv_dn8) * locals.var_b4soitsi) * locals.var_b4soinf) / 1e-7), ((((locals.var_b4soiunitlengthgatesidewalljctcapd * locals.var_pparam_b4soiwdiodcv_dn9) * locals.var_b4soitsi) * locals.var_b4soinf) / 1e-7), ((((locals.var_b4soiunitlengthgatesidewalljctcapd * locals.var_pparam_b4soiwdiodcv_dn10) * locals.var_b4soitsi) * locals.var_b4soinf) / 1e-7), ((((locals.var_b4soiunitlengthgatesidewalljctcapd * locals.var_pparam_b4soiwdiodcv_dn11) * locals.var_b4soitsi) * locals.var_b4soinf) / 1e-7), ((((locals.var_b4soiunitlengthgatesidewalljctcapd * locals.var_pparam_b4soiwdiodcv_dn12) * locals.var_b4soitsi) * locals.var_b4soinf) / 1e-7),)
    } else {
        (locals.var_cjdbs, locals.var_cjdbs_dn3, locals.var_cjdbs_dn4, locals.var_cjdbs_dn5, locals.var_cjdbs_dn6, locals.var_cjdbs_dn7, locals.var_cjdbs_dn8, locals.var_cjdbs_dn9, locals.var_cjdbs_dn10, locals.var_cjdbs_dn11, locals.var_cjdbs_dn12,)
    }
};
        locals.var_cjdbs = assign32450_e26781;
        locals.var_cjdbs_dn3 = assign32450_e26781_d_n3;
        locals.var_cjdbs_dn4 = assign32450_e26781_d_n4;
        locals.var_cjdbs_dn5 = assign32450_e26781_d_n5;
        locals.var_cjdbs_dn6 = assign32450_e26781_d_n6;
        locals.var_cjdbs_dn7 = assign32450_e26781_d_n7;
        locals.var_cjdbs_dn8 = assign32450_e26781_d_n8;
        locals.var_cjdbs_dn9 = assign32450_e26781_d_n9;
        locals.var_cjdbs_dn10 = assign32450_e26781_d_n10;
        locals.var_cjdbs_dn11 = assign32450_e26781_d_n11;
        locals.var_cjdbs_dn12 = assign32450_e26781_d_n12;

        let (assign32460_e26788, assign32460_e26788_d_n3, assign32460_e26788_d_n4, assign32460_e26788_d_n5, assign32460_e26788_d_n6, assign32460_e26788_d_n7, assign32460_e26788_d_n8, assign32460_e26788_d_n9, assign32460_e26788_d_n10, assign32460_e26788_d_n11, assign32460_e26788_d_n12,) = {
    if (locals.var_guard1750 == 0.0) {
        let assign32460_e26786: f64 = (locals.var_cjdbs * locals.var_b4soitcjswgd);
        (assign32460_e26786, (locals.var_cjdbs_dn3 * locals.var_b4soitcjswgd), (locals.var_cjdbs_dn4 * locals.var_b4soitcjswgd), (locals.var_cjdbs_dn5 * locals.var_b4soitcjswgd), (locals.var_cjdbs_dn6 * locals.var_b4soitcjswgd), (locals.var_cjdbs_dn7 * locals.var_b4soitcjswgd), (locals.var_cjdbs_dn8 * locals.var_b4soitcjswgd), (locals.var_cjdbs_dn9 * locals.var_b4soitcjswgd), (locals.var_cjdbs_dn10 * locals.var_b4soitcjswgd), (locals.var_cjdbs_dn11 * locals.var_b4soitcjswgd), (locals.var_cjdbs_dn12 * locals.var_b4soitcjswgd),)
    } else {
        (locals.var_dcjdbs_dt, locals.var_dcjdbs_dt_dn3, locals.var_dcjdbs_dt_dn4, locals.var_dcjdbs_dt_dn5, locals.var_dcjdbs_dt_dn6, locals.var_dcjdbs_dt_dn7, locals.var_dcjdbs_dt_dn8, locals.var_dcjdbs_dt_dn9, locals.var_dcjdbs_dt_dn10, locals.var_dcjdbs_dt_dn11, locals.var_dcjdbs_dt_dn12,)
    }
};
        locals.var_dcjdbs_dt = assign32460_e26788;
        locals.var_dcjdbs_dt_dn3 = assign32460_e26788_d_n3;
        locals.var_dcjdbs_dt_dn4 = assign32460_e26788_d_n4;
        locals.var_dcjdbs_dt_dn5 = assign32460_e26788_d_n5;
        locals.var_dcjdbs_dt_dn6 = assign32460_e26788_d_n6;
        locals.var_dcjdbs_dt_dn7 = assign32460_e26788_d_n7;
        locals.var_dcjdbs_dt_dn8 = assign32460_e26788_d_n8;
        locals.var_dcjdbs_dt_dn9 = assign32460_e26788_d_n9;
        locals.var_dcjdbs_dt_dn10 = assign32460_e26788_d_n10;
        locals.var_dcjdbs_dt_dn11 = assign32460_e26788_d_n11;
        locals.var_dcjdbs_dt_dn12 = assign32460_e26788_d_n12;

        let (assign32470_e26799, assign32470_e26799_d_n3, assign32470_e26799_d_n4, assign32470_e26799_d_n5, assign32470_e26799_d_n6, assign32470_e26799_d_n7, assign32470_e26799_d_n8, assign32470_e26799_d_n9, assign32470_e26799_d_n10, assign32470_e26799_d_n11, assign32470_e26799_d_n12,) = {
    if (locals.var_guard1750 == 0.0) {
        let assign32470_e26795: f64 = (locals.var_devtemp - locals.var_b4soitnom);
        let assign32470_e26796: f64 = (locals.var_dcjdbs_dt * assign32470_e26795);
        let assign32470_e26797: f64 = (locals.var_cjdbs + assign32470_e26796);
        (assign32470_e26797, (locals.var_cjdbs_dn3 + (locals.var_dcjdbs_dt_dn3 * assign32470_e26795)), (locals.var_cjdbs_dn4 + (locals.var_dcjdbs_dt_dn4 * assign32470_e26795)), (locals.var_cjdbs_dn5 + (locals.var_dcjdbs_dt_dn5 * assign32470_e26795)), (locals.var_cjdbs_dn6 + ((locals.var_dcjdbs_dt_dn6 * assign32470_e26795) + (locals.var_dcjdbs_dt * locals.var_devtemp_dn6))), (locals.var_cjdbs_dn7 + (locals.var_dcjdbs_dt_dn7 * assign32470_e26795)), (locals.var_cjdbs_dn8 + (locals.var_dcjdbs_dt_dn8 * assign32470_e26795)), (locals.var_cjdbs_dn9 + (locals.var_dcjdbs_dt_dn9 * assign32470_e26795)), (locals.var_cjdbs_dn10 + (locals.var_dcjdbs_dt_dn10 * assign32470_e26795)), (locals.var_cjdbs_dn11 + (locals.var_dcjdbs_dt_dn11 * assign32470_e26795)), (locals.var_cjdbs_dn12 + (locals.var_dcjdbs_dt_dn12 * assign32470_e26795)),)
    } else {
        (locals.var_cjdbs, locals.var_cjdbs_dn3, locals.var_cjdbs_dn4, locals.var_cjdbs_dn5, locals.var_cjdbs_dn6, locals.var_cjdbs_dn7, locals.var_cjdbs_dn8, locals.var_cjdbs_dn9, locals.var_cjdbs_dn10, locals.var_cjdbs_dn11, locals.var_cjdbs_dn12,)
    }
};
        locals.var_cjdbs = assign32470_e26799;
        locals.var_cjdbs_dn3 = assign32470_e26799_d_n3;
        locals.var_cjdbs_dn4 = assign32470_e26799_d_n4;
        locals.var_cjdbs_dn5 = assign32470_e26799_d_n5;
        locals.var_cjdbs_dn6 = assign32470_e26799_d_n6;
        locals.var_cjdbs_dn7 = assign32470_e26799_d_n7;
        locals.var_cjdbs_dn8 = assign32470_e26799_d_n8;
        locals.var_cjdbs_dn9 = assign32470_e26799_d_n9;
        locals.var_cjdbs_dn10 = assign32470_e26799_d_n10;
        locals.var_cjdbs_dn11 = assign32470_e26799_d_n11;
        locals.var_cjdbs_dn12 = assign32470_e26799_d_n12;

        let (assign32480_e26806, assign32480_e26806_d_n6,) = {
    if (locals.var_guard1750 == 0.0) {
        let assign32480_e26804: f64 = (0.9 * locals.var_phibswg);
        (assign32480_e26804, (0.9 * locals.var_phibswg_dn6),)
    } else {
        (locals.var_diomax, locals.var_diomax_dn6,)
    }
};
        locals.var_diomax = assign32480_e26806;
        locals.var_diomax_dn6 = assign32480_e26806_d_n6;

        let (assign32490_e26820, assign32490_e26820_d_n6, assign32490_e26820_d_n7, assign32490_e26820_d_n8, assign32490_e26820_d_n11, assign32490_e26820_d_n12,) = {
    if (locals.var_guard1750 == 0.0) {
        let (assign32490_e26815, assign32490_e26815_d_n6, assign32490_e26815_d_n8, assign32490_e26815_d_n11,) = {
            if (locals.var_vsbs > locals.var_diomax) {
                (locals.var_diomax, locals.var_diomax_dn6, 0.0, 0.0,)
            } else {
                (locals.var_vsbs, 0.0, locals.var_vsbs_dn8, locals.var_vsbs_dn11,)
            }
        };
        let assign32490_e26817: f64 = (assign32490_e26815 / locals.var_phibswg);
        let assign32490_e26818: f64 = (1.0 - assign32490_e26817);
        (assign32490_e26818, (-(((assign32490_e26815_d_n6 * locals.var_phibswg) - (assign32490_e26815 * locals.var_phibswg_dn6)) / (locals.var_phibswg * locals.var_phibswg))), 0.0, (-(assign32490_e26815_d_n8 / locals.var_phibswg)), (-(assign32490_e26815_d_n11 / locals.var_phibswg)), 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn11, locals.var_arg_dn12,)
    }
};
        locals.var_arg = assign32490_e26820;
        locals.var_arg_dn6 = assign32490_e26820_d_n6;
        locals.var_arg_dn7 = assign32490_e26820_d_n7;
        locals.var_arg_dn8 = assign32490_e26820_d_n8;
        locals.var_arg_dn11 = assign32490_e26820_d_n11;
        locals.var_arg_dn12 = assign32490_e26820_d_n12;

        let assign32500_e26823: f64 = if p.p173 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1751 = assign32500_e26823;

        let (assign32510_e26833, assign32510_e26833_d_n6, assign32510_e26833_d_n7, assign32510_e26833_d_n8, assign32510_e26833_d_n11, assign32510_e26833_d_n12,) = {
    if ((locals.var_guard1750 == 0.0) && (locals.var_guard1751 != 0.0)) {
        let assign32510_e26830: f64 = (locals.var_arg).sqrt();
        let assign32510_e26831: f64 = (1.0 / assign32510_e26830);
        (assign32510_e26831, (-((locals.var_arg_dn6 / (2.0 * assign32510_e26830)) / (assign32510_e26830 * assign32510_e26830))), (-((locals.var_arg_dn7 / (2.0 * assign32510_e26830)) / (assign32510_e26830 * assign32510_e26830))), (-((locals.var_arg_dn8 / (2.0 * assign32510_e26830)) / (assign32510_e26830 * assign32510_e26830))), (-((locals.var_arg_dn11 / (2.0 * assign32510_e26830)) / (assign32510_e26830 * assign32510_e26830))), (-((locals.var_arg_dn12 / (2.0 * assign32510_e26830)) / (assign32510_e26830 * assign32510_e26830))),)
    } else {
        (locals.var_dt3_dvb, locals.var_dt3_dvb_dn6, locals.var_dt3_dvb_dn7, locals.var_dt3_dvb_dn8, locals.var_dt3_dvb_dn11, locals.var_dt3_dvb_dn12,)
    }
};
        locals.var_dt3_dvb = assign32510_e26833;
        locals.var_dt3_dvb_dn6 = assign32510_e26833_d_n6;
        locals.var_dt3_dvb_dn7 = assign32510_e26833_d_n7;
        locals.var_dt3_dvb_dn8 = assign32510_e26833_d_n8;
        locals.var_dt3_dvb_dn11 = assign32510_e26833_d_n11;
        locals.var_dt3_dvb_dn12 = assign32510_e26833_d_n12;

        let (assign32520_e26852, assign32520_e26852_d_n6, assign32520_e26852_d_n7, assign32520_e26852_d_n8, assign32520_e26852_d_n11, assign32520_e26852_d_n12,) = {
    if ((locals.var_guard1750 == 0.0) && (locals.var_guard1751 == 0.0)) {
        let assign32520_e26840: f64 = (-p.p173);
        let (assign32520_e26848, assign32520_e26848_d_n6, assign32520_e26848_d_n7, assign32520_e26848_d_n8, assign32520_e26848_d_n11, assign32520_e26848_d_n12,) = {
            if (locals.var_arg > 1e-38) {
                let assign32520_e26845: f64 = (locals.var_arg).ln();
                (assign32520_e26845, (locals.var_arg_dn6 / locals.var_arg), (locals.var_arg_dn7 / locals.var_arg), (locals.var_arg_dn8 / locals.var_arg), (locals.var_arg_dn11 / locals.var_arg), (locals.var_arg_dn12 / locals.var_arg),)
            } else {
                let assign32520_e26847: f64 = (-87.49823353377374);
                (assign32520_e26847, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign32520_e26849: f64 = (assign32520_e26840 * assign32520_e26848);
        let assign32520_e26850: f64 = (assign32520_e26849).exp();
        (assign32520_e26850, (assign32520_e26850 * (assign32520_e26840 * assign32520_e26848_d_n6)), (assign32520_e26850 * (assign32520_e26840 * assign32520_e26848_d_n7)), (assign32520_e26850 * (assign32520_e26840 * assign32520_e26848_d_n8)), (assign32520_e26850 * (assign32520_e26840 * assign32520_e26848_d_n11)), (assign32520_e26850 * (assign32520_e26840 * assign32520_e26848_d_n12)),)
    } else {
        (locals.var_dt3_dvb, locals.var_dt3_dvb_dn6, locals.var_dt3_dvb_dn7, locals.var_dt3_dvb_dn8, locals.var_dt3_dvb_dn11, locals.var_dt3_dvb_dn12,)
    }
};
        locals.var_dt3_dvb = assign32520_e26852;
        locals.var_dt3_dvb_dn6 = assign32520_e26852_d_n6;
        locals.var_dt3_dvb_dn7 = assign32520_e26852_d_n7;
        locals.var_dt3_dvb_dn8 = assign32520_e26852_d_n8;
        locals.var_dt3_dvb_dn11 = assign32520_e26852_d_n11;
        locals.var_dt3_dvb_dn12 = assign32520_e26852_d_n12;

        let (assign32530_e26867, assign32530_e26867_d_n3, assign32530_e26867_d_n4, assign32530_e26867_d_n5, assign32530_e26867_d_n6, assign32530_e26867_d_n7, assign32530_e26867_d_n8, assign32530_e26867_d_n9, assign32530_e26867_d_n10, assign32530_e26867_d_n11, assign32530_e26867_d_n12,) = {
    if (locals.var_guard1750 == 0.0) {
        let assign32530_e26858: f64 = (locals.var_arg * locals.var_dt3_dvb);
        let assign32530_e26859: f64 = (1.0 - assign32530_e26858);
        let assign32530_e26861: f64 = (assign32530_e26859 * locals.var_phibswg);
        let assign32530_e26864: f64 = (1.0 - p.p173);
        let assign32530_e26865: f64 = (assign32530_e26861 / assign32530_e26864);
        (assign32530_e26865, 0.0, 0.0, 0.0, ((((-((locals.var_arg_dn6 * locals.var_dt3_dvb) + (locals.var_arg * locals.var_dt3_dvb_dn6))) * locals.var_phibswg) + (assign32530_e26859 * locals.var_phibswg_dn6)) / assign32530_e26864), (((-((locals.var_arg_dn7 * locals.var_dt3_dvb) + (locals.var_arg * locals.var_dt3_dvb_dn7))) * locals.var_phibswg) / assign32530_e26864), (((-((locals.var_arg_dn8 * locals.var_dt3_dvb) + (locals.var_arg * locals.var_dt3_dvb_dn8))) * locals.var_phibswg) / assign32530_e26864), 0.0, 0.0, (((-((locals.var_arg_dn11 * locals.var_dt3_dvb) + (locals.var_arg * locals.var_dt3_dvb_dn11))) * locals.var_phibswg) / assign32530_e26864), (((-((locals.var_arg_dn12 * locals.var_dt3_dvb) + (locals.var_arg * locals.var_dt3_dvb_dn12))) * locals.var_phibswg) / assign32530_e26864),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign32530_e26867;
        locals.var_t3__blk1147_dn3 = assign32530_e26867_d_n3;
        locals.var_t3__blk1147_dn4 = assign32530_e26867_d_n4;
        locals.var_t3__blk1147_dn5 = assign32530_e26867_d_n5;
        locals.var_t3__blk1147_dn6 = assign32530_e26867_d_n6;
        locals.var_t3__blk1147_dn7 = assign32530_e26867_d_n7;
        locals.var_t3__blk1147_dn8 = assign32530_e26867_d_n8;
        locals.var_t3__blk1147_dn9 = assign32530_e26867_d_n9;
        locals.var_t3__blk1147_dn10 = assign32530_e26867_d_n10;
        locals.var_t3__blk1147_dn11 = assign32530_e26867_d_n11;
        locals.var_t3__blk1147_dn12 = assign32530_e26867_d_n12;

        let assign32540_e26870: f64 = if locals.var_vsbs > locals.var_diomax { 1.0 } else { 0.0 };
        locals.var_guard1752 = assign32540_e26870;

        let (assign32550_e26883, assign32550_e26883_d_n3, assign32550_e26883_d_n4, assign32550_e26883_d_n5, assign32550_e26883_d_n6, assign32550_e26883_d_n7, assign32550_e26883_d_n8, assign32550_e26883_d_n9, assign32550_e26883_d_n10, assign32550_e26883_d_n11, assign32550_e26883_d_n12,) = {
    if ((locals.var_guard1750 == 0.0) && (locals.var_guard1752 != 0.0)) {
        let assign32550_e26879: f64 = (locals.var_vsbs - locals.var_diomax);
        let assign32550_e26880: f64 = (locals.var_dt3_dvb * assign32550_e26879);
        let assign32550_e26881: f64 = (locals.var_t3__blk1147 + assign32550_e26880);
        (assign32550_e26881, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, (locals.var_t3__blk1147_dn6 + ((locals.var_dt3_dvb_dn6 * assign32550_e26879) + (locals.var_dt3_dvb * (-locals.var_diomax_dn6)))), (locals.var_t3__blk1147_dn7 + (locals.var_dt3_dvb_dn7 * assign32550_e26879)), (locals.var_t3__blk1147_dn8 + ((locals.var_dt3_dvb_dn8 * assign32550_e26879) + (locals.var_dt3_dvb * locals.var_vsbs_dn8))), locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, (locals.var_t3__blk1147_dn11 + ((locals.var_dt3_dvb_dn11 * assign32550_e26879) + (locals.var_dt3_dvb * locals.var_vsbs_dn11))), (locals.var_t3__blk1147_dn12 + (locals.var_dt3_dvb_dn12 * assign32550_e26879)),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign32550_e26883;
        locals.var_t3__blk1147_dn3 = assign32550_e26883_d_n3;
        locals.var_t3__blk1147_dn4 = assign32550_e26883_d_n4;
        locals.var_t3__blk1147_dn5 = assign32550_e26883_d_n5;
        locals.var_t3__blk1147_dn6 = assign32550_e26883_d_n6;
        locals.var_t3__blk1147_dn7 = assign32550_e26883_d_n7;
        locals.var_t3__blk1147_dn8 = assign32550_e26883_d_n8;
        locals.var_t3__blk1147_dn9 = assign32550_e26883_d_n9;
        locals.var_t3__blk1147_dn10 = assign32550_e26883_d_n10;
        locals.var_t3__blk1147_dn11 = assign32550_e26883_d_n11;
        locals.var_t3__blk1147_dn12 = assign32550_e26883_d_n12;

        let (assign32560_e26896, assign32560_e26896_d_n3, assign32560_e26896_d_n4, assign32560_e26896_d_n5, assign32560_e26896_d_n6, assign32560_e26896_d_n7, assign32560_e26896_d_n8, assign32560_e26896_d_n9, assign32560_e26896_d_n10, assign32560_e26896_d_n11, assign32560_e26896_d_n12,) = {
    if (locals.var_guard1750 == 0.0) {
        let assign32560_e26888: f64 = (locals.var_cjsbs * locals.var_t3__blk1147);
        let assign32560_e26891: f64 = (locals.var_b4soitt * locals.var_ibsdif);
        let assign32560_e26893: f64 = (assign32560_e26891 * locals.var_b4soinf);
        let assign32560_e26894: f64 = (assign32560_e26888 + assign32560_e26893);
        (assign32560_e26894, (((locals.var_cjsbs_dn3 * locals.var_t3__blk1147) + (locals.var_cjsbs * locals.var_t3__blk1147_dn3)) + ((locals.var_b4soitt * locals.var_ibsdif_dn3) * locals.var_b4soinf)), (((locals.var_cjsbs_dn4 * locals.var_t3__blk1147) + (locals.var_cjsbs * locals.var_t3__blk1147_dn4)) + ((locals.var_b4soitt * locals.var_ibsdif_dn4) * locals.var_b4soinf)), (((locals.var_cjsbs_dn5 * locals.var_t3__blk1147) + (locals.var_cjsbs * locals.var_t3__blk1147_dn5)) + ((locals.var_b4soitt * locals.var_ibsdif_dn5) * locals.var_b4soinf)), (((locals.var_cjsbs_dn6 * locals.var_t3__blk1147) + (locals.var_cjsbs * locals.var_t3__blk1147_dn6)) + ((locals.var_b4soitt * locals.var_ibsdif_dn6) * locals.var_b4soinf)), (((locals.var_cjsbs_dn7 * locals.var_t3__blk1147) + (locals.var_cjsbs * locals.var_t3__blk1147_dn7)) + ((locals.var_b4soitt * locals.var_ibsdif_dn7) * locals.var_b4soinf)), (((locals.var_cjsbs_dn8 * locals.var_t3__blk1147) + (locals.var_cjsbs * locals.var_t3__blk1147_dn8)) + ((locals.var_b4soitt * locals.var_ibsdif_dn8) * locals.var_b4soinf)), (((locals.var_cjsbs_dn9 * locals.var_t3__blk1147) + (locals.var_cjsbs * locals.var_t3__blk1147_dn9)) + ((locals.var_b4soitt * locals.var_ibsdif_dn9) * locals.var_b4soinf)), (((locals.var_cjsbs_dn10 * locals.var_t3__blk1147) + (locals.var_cjsbs * locals.var_t3__blk1147_dn10)) + ((locals.var_b4soitt * locals.var_ibsdif_dn10) * locals.var_b4soinf)), (((locals.var_cjsbs_dn11 * locals.var_t3__blk1147) + (locals.var_cjsbs * locals.var_t3__blk1147_dn11)) + ((locals.var_b4soitt * locals.var_ibsdif_dn11) * locals.var_b4soinf)), (((locals.var_cjsbs_dn12 * locals.var_t3__blk1147) + (locals.var_cjsbs * locals.var_t3__blk1147_dn12)) + ((locals.var_b4soitt * locals.var_ibsdif_dn12) * locals.var_b4soinf)),)
    } else {
        (locals.var_qjs_1, locals.var_qjs_1_dn3, locals.var_qjs_1_dn4, locals.var_qjs_1_dn5, locals.var_qjs_1_dn6, locals.var_qjs_1_dn7, locals.var_qjs_1_dn8, locals.var_qjs_1_dn9, locals.var_qjs_1_dn10, locals.var_qjs_1_dn11, locals.var_qjs_1_dn12,)
    }
};
        locals.var_qjs_1 = assign32560_e26896;
        locals.var_qjs_1_dn3 = assign32560_e26896_d_n3;
        locals.var_qjs_1_dn4 = assign32560_e26896_d_n4;
        locals.var_qjs_1_dn5 = assign32560_e26896_d_n5;
        locals.var_qjs_1_dn6 = assign32560_e26896_d_n6;
        locals.var_qjs_1_dn7 = assign32560_e26896_d_n7;
        locals.var_qjs_1_dn8 = assign32560_e26896_d_n8;
        locals.var_qjs_1_dn9 = assign32560_e26896_d_n9;
        locals.var_qjs_1_dn10 = assign32560_e26896_d_n10;
        locals.var_qjs_1_dn11 = assign32560_e26896_d_n11;
        locals.var_qjs_1_dn12 = assign32560_e26896_d_n12;

        let (assign32570_e26901, assign32570_e26901_d_n6,) = {
    if (locals.var_guard1750 == 0.0) {
        (locals.var_b4soigatesidewalljctdpotential, 0.0,)
    } else {
        (locals.var_phibswg, locals.var_phibswg_dn6,)
    }
};
        locals.var_phibswg = assign32570_e26901;
        locals.var_phibswg_dn6 = assign32570_e26901_d_n6;

        let (assign32580_e26907,) = {
    if (locals.var_guard1750 == 0.0) {
        let assign32580_e26905: f64 = (-locals.var_b4soitpbswgd);
        (assign32580_e26905,)
    } else {
        (locals.var_dphibswg_dt,)
    }
};
        locals.var_dphibswg_dt = assign32580_e26907;

        let (assign32590_e26918, assign32590_e26918_d_n6,) = {
    if (locals.var_guard1750 == 0.0) {
        let assign32590_e26914: f64 = (locals.var_devtemp - locals.var_b4soitnom);
        let assign32590_e26915: f64 = (locals.var_dphibswg_dt * assign32590_e26914);
        let assign32590_e26916: f64 = (locals.var_phibswg + assign32590_e26915);
        (assign32590_e26916, (locals.var_phibswg_dn6 + (locals.var_dphibswg_dt * locals.var_devtemp_dn6)),)
    } else {
        (locals.var_phibswg, locals.var_phibswg_dn6,)
    }
};
        locals.var_phibswg = assign32590_e26918;
        locals.var_phibswg_dn6 = assign32590_e26918_d_n6;

        let (assign32600_e26923,) = {
    if (locals.var_guard1750 == 0.0) {
        (locals.var_b4soibodyjctgatesidedgradingcoeff,)
    } else {
        (p.p173,)
    }
};
        locals.var_mjswg = assign32600_e26923;

        let (assign32610_e26930, assign32610_e26930_d_n6,) = {
    if (locals.var_guard1750 == 0.0) {
        let assign32610_e26928: f64 = (0.9 * locals.var_phibswg);
        (assign32610_e26928, (0.9 * locals.var_phibswg_dn6),)
    } else {
        (locals.var_diomax, locals.var_diomax_dn6,)
    }
};
        locals.var_diomax = assign32610_e26930;
        locals.var_diomax_dn6 = assign32610_e26930_d_n6;

    }

    pub(super) fn stamp_transient_block_90(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign32620_e26944, assign32620_e26944_d_n6, assign32620_e26944_d_n7, assign32620_e26944_d_n8, assign32620_e26944_d_n11, assign32620_e26944_d_n12,) = {
    if (locals.var_guard1750 == 0.0) {
        let (assign32620_e26939, assign32620_e26939_d_n6, assign32620_e26939_d_n7, assign32620_e26939_d_n12,) = {
            if (locals.var_vdbd > locals.var_diomax) {
                (locals.var_diomax, locals.var_diomax_dn6, 0.0, 0.0,)
            } else {
                (locals.var_vdbd, 0.0, locals.var_vdbd_dn7, locals.var_vdbd_dn12,)
            }
        };
        let assign32620_e26941: f64 = (assign32620_e26939 / locals.var_phibswg);
        let assign32620_e26942: f64 = (1.0 - assign32620_e26941);
        (assign32620_e26942, (-(((assign32620_e26939_d_n6 * locals.var_phibswg) - (assign32620_e26939 * locals.var_phibswg_dn6)) / (locals.var_phibswg * locals.var_phibswg))), (-(assign32620_e26939_d_n7 / locals.var_phibswg)), 0.0, 0.0, (-(assign32620_e26939_d_n12 / locals.var_phibswg)),)
    } else {
        (locals.var_arg, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn11, locals.var_arg_dn12,)
    }
};
        locals.var_arg = assign32620_e26944;
        locals.var_arg_dn6 = assign32620_e26944_d_n6;
        locals.var_arg_dn7 = assign32620_e26944_d_n7;
        locals.var_arg_dn8 = assign32620_e26944_d_n8;
        locals.var_arg_dn11 = assign32620_e26944_d_n11;
        locals.var_arg_dn12 = assign32620_e26944_d_n12;

        let assign32630_e26947: f64 = if p.p173 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1753 = assign32630_e26947;

        let (assign32640_e26957, assign32640_e26957_d_n6, assign32640_e26957_d_n7, assign32640_e26957_d_n8, assign32640_e26957_d_n11, assign32640_e26957_d_n12,) = {
    if ((locals.var_guard1750 == 0.0) && (locals.var_guard1753 != 0.0)) {
        let assign32640_e26954: f64 = (locals.var_arg).sqrt();
        let assign32640_e26955: f64 = (1.0 / assign32640_e26954);
        (assign32640_e26955, (-((locals.var_arg_dn6 / (2.0 * assign32640_e26954)) / (assign32640_e26954 * assign32640_e26954))), (-((locals.var_arg_dn7 / (2.0 * assign32640_e26954)) / (assign32640_e26954 * assign32640_e26954))), (-((locals.var_arg_dn8 / (2.0 * assign32640_e26954)) / (assign32640_e26954 * assign32640_e26954))), (-((locals.var_arg_dn11 / (2.0 * assign32640_e26954)) / (assign32640_e26954 * assign32640_e26954))), (-((locals.var_arg_dn12 / (2.0 * assign32640_e26954)) / (assign32640_e26954 * assign32640_e26954))),)
    } else {
        (locals.var_dt3_dvb, locals.var_dt3_dvb_dn6, locals.var_dt3_dvb_dn7, locals.var_dt3_dvb_dn8, locals.var_dt3_dvb_dn11, locals.var_dt3_dvb_dn12,)
    }
};
        locals.var_dt3_dvb = assign32640_e26957;
        locals.var_dt3_dvb_dn6 = assign32640_e26957_d_n6;
        locals.var_dt3_dvb_dn7 = assign32640_e26957_d_n7;
        locals.var_dt3_dvb_dn8 = assign32640_e26957_d_n8;
        locals.var_dt3_dvb_dn11 = assign32640_e26957_d_n11;
        locals.var_dt3_dvb_dn12 = assign32640_e26957_d_n12;

        let (assign32650_e26976, assign32650_e26976_d_n6, assign32650_e26976_d_n7, assign32650_e26976_d_n8, assign32650_e26976_d_n11, assign32650_e26976_d_n12,) = {
    if ((locals.var_guard1750 == 0.0) && (locals.var_guard1753 == 0.0)) {
        let assign32650_e26964: f64 = (-p.p173);
        let (assign32650_e26972, assign32650_e26972_d_n6, assign32650_e26972_d_n7, assign32650_e26972_d_n8, assign32650_e26972_d_n11, assign32650_e26972_d_n12,) = {
            if (locals.var_arg > 1e-38) {
                let assign32650_e26969: f64 = (locals.var_arg).ln();
                (assign32650_e26969, (locals.var_arg_dn6 / locals.var_arg), (locals.var_arg_dn7 / locals.var_arg), (locals.var_arg_dn8 / locals.var_arg), (locals.var_arg_dn11 / locals.var_arg), (locals.var_arg_dn12 / locals.var_arg),)
            } else {
                let assign32650_e26971: f64 = (-87.49823353377374);
                (assign32650_e26971, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign32650_e26973: f64 = (assign32650_e26964 * assign32650_e26972);
        let assign32650_e26974: f64 = (assign32650_e26973).exp();
        (assign32650_e26974, (assign32650_e26974 * (assign32650_e26964 * assign32650_e26972_d_n6)), (assign32650_e26974 * (assign32650_e26964 * assign32650_e26972_d_n7)), (assign32650_e26974 * (assign32650_e26964 * assign32650_e26972_d_n8)), (assign32650_e26974 * (assign32650_e26964 * assign32650_e26972_d_n11)), (assign32650_e26974 * (assign32650_e26964 * assign32650_e26972_d_n12)),)
    } else {
        (locals.var_dt3_dvb, locals.var_dt3_dvb_dn6, locals.var_dt3_dvb_dn7, locals.var_dt3_dvb_dn8, locals.var_dt3_dvb_dn11, locals.var_dt3_dvb_dn12,)
    }
};
        locals.var_dt3_dvb = assign32650_e26976;
        locals.var_dt3_dvb_dn6 = assign32650_e26976_d_n6;
        locals.var_dt3_dvb_dn7 = assign32650_e26976_d_n7;
        locals.var_dt3_dvb_dn8 = assign32650_e26976_d_n8;
        locals.var_dt3_dvb_dn11 = assign32650_e26976_d_n11;
        locals.var_dt3_dvb_dn12 = assign32650_e26976_d_n12;

        let (assign32660_e26991, assign32660_e26991_d_n3, assign32660_e26991_d_n4, assign32660_e26991_d_n5, assign32660_e26991_d_n6, assign32660_e26991_d_n7, assign32660_e26991_d_n8, assign32660_e26991_d_n9, assign32660_e26991_d_n10, assign32660_e26991_d_n11, assign32660_e26991_d_n12,) = {
    if (locals.var_guard1750 == 0.0) {
        let assign32660_e26982: f64 = (locals.var_arg * locals.var_dt3_dvb);
        let assign32660_e26983: f64 = (1.0 - assign32660_e26982);
        let assign32660_e26985: f64 = (assign32660_e26983 * locals.var_phibswg);
        let assign32660_e26988: f64 = (1.0 - p.p173);
        let assign32660_e26989: f64 = (assign32660_e26985 / assign32660_e26988);
        (assign32660_e26989, 0.0, 0.0, 0.0, ((((-((locals.var_arg_dn6 * locals.var_dt3_dvb) + (locals.var_arg * locals.var_dt3_dvb_dn6))) * locals.var_phibswg) + (assign32660_e26983 * locals.var_phibswg_dn6)) / assign32660_e26988), (((-((locals.var_arg_dn7 * locals.var_dt3_dvb) + (locals.var_arg * locals.var_dt3_dvb_dn7))) * locals.var_phibswg) / assign32660_e26988), (((-((locals.var_arg_dn8 * locals.var_dt3_dvb) + (locals.var_arg * locals.var_dt3_dvb_dn8))) * locals.var_phibswg) / assign32660_e26988), 0.0, 0.0, (((-((locals.var_arg_dn11 * locals.var_dt3_dvb) + (locals.var_arg * locals.var_dt3_dvb_dn11))) * locals.var_phibswg) / assign32660_e26988), (((-((locals.var_arg_dn12 * locals.var_dt3_dvb) + (locals.var_arg * locals.var_dt3_dvb_dn12))) * locals.var_phibswg) / assign32660_e26988),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign32660_e26991;
        locals.var_t3__blk1147_dn3 = assign32660_e26991_d_n3;
        locals.var_t3__blk1147_dn4 = assign32660_e26991_d_n4;
        locals.var_t3__blk1147_dn5 = assign32660_e26991_d_n5;
        locals.var_t3__blk1147_dn6 = assign32660_e26991_d_n6;
        locals.var_t3__blk1147_dn7 = assign32660_e26991_d_n7;
        locals.var_t3__blk1147_dn8 = assign32660_e26991_d_n8;
        locals.var_t3__blk1147_dn9 = assign32660_e26991_d_n9;
        locals.var_t3__blk1147_dn10 = assign32660_e26991_d_n10;
        locals.var_t3__blk1147_dn11 = assign32660_e26991_d_n11;
        locals.var_t3__blk1147_dn12 = assign32660_e26991_d_n12;

        let assign32670_e26994: f64 = if locals.var_vdbd > locals.var_diomax { 1.0 } else { 0.0 };
        locals.var_guard1754 = assign32670_e26994;

        let (assign32680_e27007, assign32680_e27007_d_n3, assign32680_e27007_d_n4, assign32680_e27007_d_n5, assign32680_e27007_d_n6, assign32680_e27007_d_n7, assign32680_e27007_d_n8, assign32680_e27007_d_n9, assign32680_e27007_d_n10, assign32680_e27007_d_n11, assign32680_e27007_d_n12,) = {
    if ((locals.var_guard1750 == 0.0) && (locals.var_guard1754 != 0.0)) {
        let assign32680_e27003: f64 = (locals.var_vdbd - locals.var_diomax);
        let assign32680_e27004: f64 = (locals.var_dt3_dvb * assign32680_e27003);
        let assign32680_e27005: f64 = (locals.var_t3__blk1147 + assign32680_e27004);
        (assign32680_e27005, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, (locals.var_t3__blk1147_dn6 + ((locals.var_dt3_dvb_dn6 * assign32680_e27003) + (locals.var_dt3_dvb * (-locals.var_diomax_dn6)))), (locals.var_t3__blk1147_dn7 + ((locals.var_dt3_dvb_dn7 * assign32680_e27003) + (locals.var_dt3_dvb * locals.var_vdbd_dn7))), (locals.var_t3__blk1147_dn8 + (locals.var_dt3_dvb_dn8 * assign32680_e27003)), locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, (locals.var_t3__blk1147_dn11 + (locals.var_dt3_dvb_dn11 * assign32680_e27003)), (locals.var_t3__blk1147_dn12 + ((locals.var_dt3_dvb_dn12 * assign32680_e27003) + (locals.var_dt3_dvb * locals.var_vdbd_dn12))),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign32680_e27007;
        locals.var_t3__blk1147_dn3 = assign32680_e27007_d_n3;
        locals.var_t3__blk1147_dn4 = assign32680_e27007_d_n4;
        locals.var_t3__blk1147_dn5 = assign32680_e27007_d_n5;
        locals.var_t3__blk1147_dn6 = assign32680_e27007_d_n6;
        locals.var_t3__blk1147_dn7 = assign32680_e27007_d_n7;
        locals.var_t3__blk1147_dn8 = assign32680_e27007_d_n8;
        locals.var_t3__blk1147_dn9 = assign32680_e27007_d_n9;
        locals.var_t3__blk1147_dn10 = assign32680_e27007_d_n10;
        locals.var_t3__blk1147_dn11 = assign32680_e27007_d_n11;
        locals.var_t3__blk1147_dn12 = assign32680_e27007_d_n12;

        let (assign32690_e27020, assign32690_e27020_d_n3, assign32690_e27020_d_n4, assign32690_e27020_d_n5, assign32690_e27020_d_n6, assign32690_e27020_d_n7, assign32690_e27020_d_n8, assign32690_e27020_d_n9, assign32690_e27020_d_n10, assign32690_e27020_d_n11, assign32690_e27020_d_n12,) = {
    if (locals.var_guard1750 == 0.0) {
        let assign32690_e27012: f64 = (locals.var_cjdbs * locals.var_t3__blk1147);
        let assign32690_e27015: f64 = (locals.var_b4soitt * locals.var_ibddif);
        let assign32690_e27017: f64 = (assign32690_e27015 * locals.var_b4soinf);
        let assign32690_e27018: f64 = (assign32690_e27012 + assign32690_e27017);
        (assign32690_e27018, (((locals.var_cjdbs_dn3 * locals.var_t3__blk1147) + (locals.var_cjdbs * locals.var_t3__blk1147_dn3)) + ((locals.var_b4soitt * locals.var_ibddif_dn3) * locals.var_b4soinf)), (((locals.var_cjdbs_dn4 * locals.var_t3__blk1147) + (locals.var_cjdbs * locals.var_t3__blk1147_dn4)) + ((locals.var_b4soitt * locals.var_ibddif_dn4) * locals.var_b4soinf)), (((locals.var_cjdbs_dn5 * locals.var_t3__blk1147) + (locals.var_cjdbs * locals.var_t3__blk1147_dn5)) + ((locals.var_b4soitt * locals.var_ibddif_dn5) * locals.var_b4soinf)), (((locals.var_cjdbs_dn6 * locals.var_t3__blk1147) + (locals.var_cjdbs * locals.var_t3__blk1147_dn6)) + ((locals.var_b4soitt * locals.var_ibddif_dn6) * locals.var_b4soinf)), (((locals.var_cjdbs_dn7 * locals.var_t3__blk1147) + (locals.var_cjdbs * locals.var_t3__blk1147_dn7)) + ((locals.var_b4soitt * locals.var_ibddif_dn7) * locals.var_b4soinf)), (((locals.var_cjdbs_dn8 * locals.var_t3__blk1147) + (locals.var_cjdbs * locals.var_t3__blk1147_dn8)) + ((locals.var_b4soitt * locals.var_ibddif_dn8) * locals.var_b4soinf)), (((locals.var_cjdbs_dn9 * locals.var_t3__blk1147) + (locals.var_cjdbs * locals.var_t3__blk1147_dn9)) + ((locals.var_b4soitt * locals.var_ibddif_dn9) * locals.var_b4soinf)), (((locals.var_cjdbs_dn10 * locals.var_t3__blk1147) + (locals.var_cjdbs * locals.var_t3__blk1147_dn10)) + ((locals.var_b4soitt * locals.var_ibddif_dn10) * locals.var_b4soinf)), (((locals.var_cjdbs_dn11 * locals.var_t3__blk1147) + (locals.var_cjdbs * locals.var_t3__blk1147_dn11)) + ((locals.var_b4soitt * locals.var_ibddif_dn11) * locals.var_b4soinf)), (((locals.var_cjdbs_dn12 * locals.var_t3__blk1147) + (locals.var_cjdbs * locals.var_t3__blk1147_dn12)) + ((locals.var_b4soitt * locals.var_ibddif_dn12) * locals.var_b4soinf)),)
    } else {
        (locals.var_qjd_1, locals.var_qjd_1_dn3, locals.var_qjd_1_dn4, locals.var_qjd_1_dn5, locals.var_qjd_1_dn6, locals.var_qjd_1_dn7, locals.var_qjd_1_dn8, locals.var_qjd_1_dn9, locals.var_qjd_1_dn10, locals.var_qjd_1_dn11, locals.var_qjd_1_dn12,)
    }
};
        locals.var_qjd_1 = assign32690_e27020;
        locals.var_qjd_1_dn3 = assign32690_e27020_d_n3;
        locals.var_qjd_1_dn4 = assign32690_e27020_d_n4;
        locals.var_qjd_1_dn5 = assign32690_e27020_d_n5;
        locals.var_qjd_1_dn6 = assign32690_e27020_d_n6;
        locals.var_qjd_1_dn7 = assign32690_e27020_d_n7;
        locals.var_qjd_1_dn8 = assign32690_e27020_d_n8;
        locals.var_qjd_1_dn9 = assign32690_e27020_d_n9;
        locals.var_qjd_1_dn10 = assign32690_e27020_d_n10;
        locals.var_qjd_1_dn11 = assign32690_e27020_d_n11;
        locals.var_qjd_1_dn12 = assign32690_e27020_d_n12;

        let assign32700_e27022: f64 = (-locals.var_b4soitype);
        let assign32700_e27024: f64 = (assign32700_e27022 * locals.var_ves);
        locals.var_t10__blk1154 = assign32700_e27024;
        locals.var_t10__blk1154_dn3 = (assign32700_e27022 * locals.var_ves_dn3);
        locals.var_t10__blk1154_dn4 = 0.0;
        locals.var_t10__blk1154_dn5 = 0.0;
        locals.var_t10__blk1154_dn6 = 0.0;
        locals.var_t10__blk1154_dn7 = 0.0;
        locals.var_t10__blk1154_dn8 = (assign32700_e27022 * locals.var_ves_dn8);
        locals.var_t10__blk1154_dn9 = 0.0;
        locals.var_t10__blk1154_dn10 = 0.0;
        locals.var_t10__blk1154_dn11 = 0.0;
        locals.var_t10__blk1154_dn12 = 0.0;

        let assign32710_e27028: f64 = (locals.var_vds - locals.var_ves);
        let assign32710_e27029: f64 = (locals.var_b4soitype * assign32710_e27028);
        locals.var_t11 = assign32710_e27029;
        locals.var_t11_dn3 = (locals.var_b4soitype * (-locals.var_ves_dn3));
        locals.var_t11_dn4 = 0.0;
        locals.var_t11_dn5 = 0.0;
        locals.var_t11_dn6 = 0.0;
        locals.var_t11_dn7 = (locals.var_b4soitype * locals.var_vds_dn7);
        locals.var_t11_dn8 = (locals.var_b4soitype * (locals.var_vds_dn8 - locals.var_ves_dn8));
        locals.var_t11_dn9 = 0.0;
        locals.var_t11_dn10 = 0.0;
        locals.var_t11_dn11 = 0.0;
        locals.var_t11_dn12 = 0.0;

        let assign32720_e27032: f64 = if locals.var_b4soicsdmin != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1755 = assign32720_e27032;

        let assign32730_e27047: f64 = if (((locals.var_pparam_b4soinsub > 0.0) && (locals.var_b4soitype > 0.0)) || ((locals.var_pparam_b4soinsub < 0.0) && (locals.var_b4soitype < 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard1756 = assign32730_e27047;

        let assign32740_e27050: f64 = if locals.var_t10__blk1154 < locals.var_pparam_b4soivsdfb { 1.0 } else { 0.0 };
        locals.var_guard1757 = assign32740_e27050;

        let (assign32750_e27062, assign32750_e27062_d_n3, assign32750_e27062_d_n4, assign32750_e27062_d_n5, assign32750_e27062_d_n6, assign32750_e27062_d_n7, assign32750_e27062_d_n8, assign32750_e27062_d_n9, assign32750_e27062_d_n10, assign32750_e27062_d_n11, assign32750_e27062_d_n12,) = {
    if (((locals.var_guard1755 != 0.0) && (locals.var_guard1756 != 0.0)) && (locals.var_guard1757 != 0.0)) {
        let assign32750_e27059: f64 = (locals.var_t10__blk1154 - locals.var_pparam_b4soivsdfb);
        let assign32750_e27060: f64 = (locals.var_b4soicsbox * assign32750_e27059);
        (assign32750_e27060, (locals.var_b4soicsbox * (locals.var_t10__blk1154_dn3 - locals.var_pparam_b4soivsdfb_dn3)), (locals.var_b4soicsbox * (locals.var_t10__blk1154_dn4 - locals.var_pparam_b4soivsdfb_dn4)), (locals.var_b4soicsbox * (locals.var_t10__blk1154_dn5 - locals.var_pparam_b4soivsdfb_dn5)), (locals.var_b4soicsbox * (locals.var_t10__blk1154_dn6 - locals.var_pparam_b4soivsdfb_dn6)), (locals.var_b4soicsbox * (locals.var_t10__blk1154_dn7 - locals.var_pparam_b4soivsdfb_dn7)), (locals.var_b4soicsbox * (locals.var_t10__blk1154_dn8 - locals.var_pparam_b4soivsdfb_dn8)), (locals.var_b4soicsbox * (locals.var_t10__blk1154_dn9 - locals.var_pparam_b4soivsdfb_dn9)), (locals.var_b4soicsbox * (locals.var_t10__blk1154_dn10 - locals.var_pparam_b4soivsdfb_dn10)), (locals.var_b4soicsbox * (locals.var_t10__blk1154_dn11 - locals.var_pparam_b4soivsdfb_dn11)), (locals.var_b4soicsbox * (locals.var_t10__blk1154_dn12 - locals.var_pparam_b4soivsdfb_dn12)),)
    } else {
        (locals.var_b4soiqse, locals.var_b4soiqse_dn3, locals.var_b4soiqse_dn4, locals.var_b4soiqse_dn5, locals.var_b4soiqse_dn6, locals.var_b4soiqse_dn7, locals.var_b4soiqse_dn8, locals.var_b4soiqse_dn9, locals.var_b4soiqse_dn10, locals.var_b4soiqse_dn11, locals.var_b4soiqse_dn12,)
    }
};
        locals.var_b4soiqse = assign32750_e27062;
        locals.var_b4soiqse_dn3 = assign32750_e27062_d_n3;
        locals.var_b4soiqse_dn4 = assign32750_e27062_d_n4;
        locals.var_b4soiqse_dn5 = assign32750_e27062_d_n5;
        locals.var_b4soiqse_dn6 = assign32750_e27062_d_n6;
        locals.var_b4soiqse_dn7 = assign32750_e27062_d_n7;
        locals.var_b4soiqse_dn8 = assign32750_e27062_d_n8;
        locals.var_b4soiqse_dn9 = assign32750_e27062_d_n9;
        locals.var_b4soiqse_dn10 = assign32750_e27062_d_n10;
        locals.var_b4soiqse_dn11 = assign32750_e27062_d_n11;
        locals.var_b4soiqse_dn12 = assign32750_e27062_d_n12;

        let assign32760_e27065: f64 = if locals.var_t10__blk1154 < locals.var_pparam_b4soisdt1 { 1.0 } else { 0.0 };
        locals.var_guard1758 = assign32760_e27065;

        let (assign32770_e27078, assign32770_e27078_d_n3, assign32770_e27078_d_n4, assign32770_e27078_d_n5, assign32770_e27078_d_n6, assign32770_e27078_d_n7, assign32770_e27078_d_n8, assign32770_e27078_d_n9, assign32770_e27078_d_n10, assign32770_e27078_d_n11, assign32770_e27078_d_n12,) = {
    if ((((locals.var_guard1755 != 0.0) && (locals.var_guard1756 != 0.0)) && (locals.var_guard1757 == 0.0)) && (locals.var_guard1758 != 0.0)) {
        let assign32770_e27076: f64 = (locals.var_t10__blk1154 - locals.var_pparam_b4soivsdfb);
        (assign32770_e27076, (locals.var_t10__blk1154_dn3 - locals.var_pparam_b4soivsdfb_dn3), (locals.var_t10__blk1154_dn4 - locals.var_pparam_b4soivsdfb_dn4), (locals.var_t10__blk1154_dn5 - locals.var_pparam_b4soivsdfb_dn5), (locals.var_t10__blk1154_dn6 - locals.var_pparam_b4soivsdfb_dn6), (locals.var_t10__blk1154_dn7 - locals.var_pparam_b4soivsdfb_dn7), (locals.var_t10__blk1154_dn8 - locals.var_pparam_b4soivsdfb_dn8), (locals.var_t10__blk1154_dn9 - locals.var_pparam_b4soivsdfb_dn9), (locals.var_t10__blk1154_dn10 - locals.var_pparam_b4soivsdfb_dn10), (locals.var_t10__blk1154_dn11 - locals.var_pparam_b4soivsdfb_dn11), (locals.var_t10__blk1154_dn12 - locals.var_pparam_b4soivsdfb_dn12),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign32770_e27078;
        locals.var_t0__blk1144_dn3 = assign32770_e27078_d_n3;
        locals.var_t0__blk1144_dn4 = assign32770_e27078_d_n4;
        locals.var_t0__blk1144_dn5 = assign32770_e27078_d_n5;
        locals.var_t0__blk1144_dn6 = assign32770_e27078_d_n6;
        locals.var_t0__blk1144_dn7 = assign32770_e27078_d_n7;
        locals.var_t0__blk1144_dn8 = assign32770_e27078_d_n8;
        locals.var_t0__blk1144_dn9 = assign32770_e27078_d_n9;
        locals.var_t0__blk1144_dn10 = assign32770_e27078_d_n10;
        locals.var_t0__blk1144_dn11 = assign32770_e27078_d_n11;
        locals.var_t0__blk1144_dn12 = assign32770_e27078_d_n12;

        let (assign32780_e27091, assign32780_e27091_d_n3, assign32780_e27091_d_n4, assign32780_e27091_d_n5, assign32780_e27091_d_n6, assign32780_e27091_d_n7, assign32780_e27091_d_n8, assign32780_e27091_d_n9, assign32780_e27091_d_n10, assign32780_e27091_d_n11, assign32780_e27091_d_n12,) = {
    if ((((locals.var_guard1755 != 0.0) && (locals.var_guard1756 != 0.0)) && (locals.var_guard1757 == 0.0)) && (locals.var_guard1758 != 0.0)) {
        let assign32780_e27089: f64 = (locals.var_t0__blk1144 * locals.var_t0__blk1144);
        (assign32780_e27089, ((locals.var_t0__blk1144_dn3 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn3)), ((locals.var_t0__blk1144_dn4 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn4)), ((locals.var_t0__blk1144_dn5 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn5)), ((locals.var_t0__blk1144_dn6 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn6)), ((locals.var_t0__blk1144_dn7 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn7)), ((locals.var_t0__blk1144_dn8 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn8)), ((locals.var_t0__blk1144_dn9 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn9)), ((locals.var_t0__blk1144_dn10 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn10)), ((locals.var_t0__blk1144_dn11 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn11)), ((locals.var_t0__blk1144_dn12 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn12)),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign32780_e27091;
        locals.var_t1__blk1145_dn3 = assign32780_e27091_d_n3;
        locals.var_t1__blk1145_dn4 = assign32780_e27091_d_n4;
        locals.var_t1__blk1145_dn5 = assign32780_e27091_d_n5;
        locals.var_t1__blk1145_dn6 = assign32780_e27091_d_n6;
        locals.var_t1__blk1145_dn7 = assign32780_e27091_d_n7;
        locals.var_t1__blk1145_dn8 = assign32780_e27091_d_n8;
        locals.var_t1__blk1145_dn9 = assign32780_e27091_d_n9;
        locals.var_t1__blk1145_dn10 = assign32780_e27091_d_n10;
        locals.var_t1__blk1145_dn11 = assign32780_e27091_d_n11;
        locals.var_t1__blk1145_dn12 = assign32780_e27091_d_n12;

        let (assign32790_e27110, assign32790_e27110_d_n3, assign32790_e27110_d_n4, assign32790_e27110_d_n5, assign32790_e27110_d_n6, assign32790_e27110_d_n7, assign32790_e27110_d_n8, assign32790_e27110_d_n9, assign32790_e27110_d_n10, assign32790_e27110_d_n11, assign32790_e27110_d_n12,) = {
    if ((((locals.var_guard1755 != 0.0) && (locals.var_guard1756 != 0.0)) && (locals.var_guard1757 == 0.0)) && (locals.var_guard1758 != 0.0)) {
        let assign32790_e27104: f64 = (locals.var_pparam_b4soist2 / 3.0);
        let assign32790_e27106: f64 = (assign32790_e27104 * locals.var_t1__blk1145);
        let assign32790_e27107: f64 = (locals.var_b4soicsbox - assign32790_e27106);
        let assign32790_e27108: f64 = (locals.var_t0__blk1144 * assign32790_e27107);
        (assign32790_e27108, ((locals.var_t0__blk1144_dn3 * assign32790_e27107) + (locals.var_t0__blk1144 * (-(((locals.var_pparam_b4soist2_dn3 / 3.0) * locals.var_t1__blk1145) + (assign32790_e27104 * locals.var_t1__blk1145_dn3))))), ((locals.var_t0__blk1144_dn4 * assign32790_e27107) + (locals.var_t0__blk1144 * (-(((locals.var_pparam_b4soist2_dn4 / 3.0) * locals.var_t1__blk1145) + (assign32790_e27104 * locals.var_t1__blk1145_dn4))))), ((locals.var_t0__blk1144_dn5 * assign32790_e27107) + (locals.var_t0__blk1144 * (-(((locals.var_pparam_b4soist2_dn5 / 3.0) * locals.var_t1__blk1145) + (assign32790_e27104 * locals.var_t1__blk1145_dn5))))), ((locals.var_t0__blk1144_dn6 * assign32790_e27107) + (locals.var_t0__blk1144 * (-(((locals.var_pparam_b4soist2_dn6 / 3.0) * locals.var_t1__blk1145) + (assign32790_e27104 * locals.var_t1__blk1145_dn6))))), ((locals.var_t0__blk1144_dn7 * assign32790_e27107) + (locals.var_t0__blk1144 * (-(((locals.var_pparam_b4soist2_dn7 / 3.0) * locals.var_t1__blk1145) + (assign32790_e27104 * locals.var_t1__blk1145_dn7))))), ((locals.var_t0__blk1144_dn8 * assign32790_e27107) + (locals.var_t0__blk1144 * (-(((locals.var_pparam_b4soist2_dn8 / 3.0) * locals.var_t1__blk1145) + (assign32790_e27104 * locals.var_t1__blk1145_dn8))))), ((locals.var_t0__blk1144_dn9 * assign32790_e27107) + (locals.var_t0__blk1144 * (-(((locals.var_pparam_b4soist2_dn9 / 3.0) * locals.var_t1__blk1145) + (assign32790_e27104 * locals.var_t1__blk1145_dn9))))), ((locals.var_t0__blk1144_dn10 * assign32790_e27107) + (locals.var_t0__blk1144 * (-(((locals.var_pparam_b4soist2_dn10 / 3.0) * locals.var_t1__blk1145) + (assign32790_e27104 * locals.var_t1__blk1145_dn10))))), ((locals.var_t0__blk1144_dn11 * assign32790_e27107) + (locals.var_t0__blk1144 * (-(((locals.var_pparam_b4soist2_dn11 / 3.0) * locals.var_t1__blk1145) + (assign32790_e27104 * locals.var_t1__blk1145_dn11))))), ((locals.var_t0__blk1144_dn12 * assign32790_e27107) + (locals.var_t0__blk1144 * (-(((locals.var_pparam_b4soist2_dn12 / 3.0) * locals.var_t1__blk1145) + (assign32790_e27104 * locals.var_t1__blk1145_dn12))))),)
    } else {
        (locals.var_b4soiqse, locals.var_b4soiqse_dn3, locals.var_b4soiqse_dn4, locals.var_b4soiqse_dn5, locals.var_b4soiqse_dn6, locals.var_b4soiqse_dn7, locals.var_b4soiqse_dn8, locals.var_b4soiqse_dn9, locals.var_b4soiqse_dn10, locals.var_b4soiqse_dn11, locals.var_b4soiqse_dn12,)
    }
};
        locals.var_b4soiqse = assign32790_e27110;
        locals.var_b4soiqse_dn3 = assign32790_e27110_d_n3;
        locals.var_b4soiqse_dn4 = assign32790_e27110_d_n4;
        locals.var_b4soiqse_dn5 = assign32790_e27110_d_n5;
        locals.var_b4soiqse_dn6 = assign32790_e27110_d_n6;
        locals.var_b4soiqse_dn7 = assign32790_e27110_d_n7;
        locals.var_b4soiqse_dn8 = assign32790_e27110_d_n8;
        locals.var_b4soiqse_dn9 = assign32790_e27110_d_n9;
        locals.var_b4soiqse_dn10 = assign32790_e27110_d_n10;
        locals.var_b4soiqse_dn11 = assign32790_e27110_d_n11;
        locals.var_b4soiqse_dn12 = assign32790_e27110_d_n12;

        let assign32800_e27113: f64 = if locals.var_t10__blk1154 < locals.var_pparam_b4soivsdth { 1.0 } else { 0.0 };
        locals.var_guard1759 = assign32800_e27113;

        let (assign32810_e27129, assign32810_e27129_d_n3, assign32810_e27129_d_n4, assign32810_e27129_d_n5, assign32810_e27129_d_n6, assign32810_e27129_d_n7, assign32810_e27129_d_n8, assign32810_e27129_d_n9, assign32810_e27129_d_n10, assign32810_e27129_d_n11, assign32810_e27129_d_n12,) = {
    if (((((locals.var_guard1755 != 0.0) && (locals.var_guard1756 != 0.0)) && (locals.var_guard1757 == 0.0)) && (locals.var_guard1758 == 0.0)) && (locals.var_guard1759 != 0.0)) {
        let assign32810_e27127: f64 = (locals.var_t10__blk1154 - locals.var_pparam_b4soivsdth);
        (assign32810_e27127, (locals.var_t10__blk1154_dn3 - locals.var_pparam_b4soivsdth_dn3), (locals.var_t10__blk1154_dn4 - locals.var_pparam_b4soivsdth_dn4), (locals.var_t10__blk1154_dn5 - locals.var_pparam_b4soivsdth_dn5), (locals.var_t10__blk1154_dn6 - locals.var_pparam_b4soivsdth_dn6), (locals.var_t10__blk1154_dn7 - locals.var_pparam_b4soivsdth_dn7), (locals.var_t10__blk1154_dn8 - locals.var_pparam_b4soivsdth_dn8), (locals.var_t10__blk1154_dn9 - locals.var_pparam_b4soivsdth_dn9), (locals.var_t10__blk1154_dn10 - locals.var_pparam_b4soivsdth_dn10), (locals.var_t10__blk1154_dn11 - locals.var_pparam_b4soivsdth_dn11), (locals.var_t10__blk1154_dn12 - locals.var_pparam_b4soivsdth_dn12),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign32810_e27129;
        locals.var_t0__blk1144_dn3 = assign32810_e27129_d_n3;
        locals.var_t0__blk1144_dn4 = assign32810_e27129_d_n4;
        locals.var_t0__blk1144_dn5 = assign32810_e27129_d_n5;
        locals.var_t0__blk1144_dn6 = assign32810_e27129_d_n6;
        locals.var_t0__blk1144_dn7 = assign32810_e27129_d_n7;
        locals.var_t0__blk1144_dn8 = assign32810_e27129_d_n8;
        locals.var_t0__blk1144_dn9 = assign32810_e27129_d_n9;
        locals.var_t0__blk1144_dn10 = assign32810_e27129_d_n10;
        locals.var_t0__blk1144_dn11 = assign32810_e27129_d_n11;
        locals.var_t0__blk1144_dn12 = assign32810_e27129_d_n12;

        let (assign32820_e27145, assign32820_e27145_d_n3, assign32820_e27145_d_n4, assign32820_e27145_d_n5, assign32820_e27145_d_n6, assign32820_e27145_d_n7, assign32820_e27145_d_n8, assign32820_e27145_d_n9, assign32820_e27145_d_n10, assign32820_e27145_d_n11, assign32820_e27145_d_n12,) = {
    if (((((locals.var_guard1755 != 0.0) && (locals.var_guard1756 != 0.0)) && (locals.var_guard1757 == 0.0)) && (locals.var_guard1758 == 0.0)) && (locals.var_guard1759 != 0.0)) {
        let assign32820_e27143: f64 = (locals.var_t0__blk1144 * locals.var_t0__blk1144);
        (assign32820_e27143, ((locals.var_t0__blk1144_dn3 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn3)), ((locals.var_t0__blk1144_dn4 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn4)), ((locals.var_t0__blk1144_dn5 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn5)), ((locals.var_t0__blk1144_dn6 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn6)), ((locals.var_t0__blk1144_dn7 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn7)), ((locals.var_t0__blk1144_dn8 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn8)), ((locals.var_t0__blk1144_dn9 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn9)), ((locals.var_t0__blk1144_dn10 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn10)), ((locals.var_t0__blk1144_dn11 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn11)), ((locals.var_t0__blk1144_dn12 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn12)),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign32820_e27145;
        locals.var_t1__blk1145_dn3 = assign32820_e27145_d_n3;
        locals.var_t1__blk1145_dn4 = assign32820_e27145_d_n4;
        locals.var_t1__blk1145_dn5 = assign32820_e27145_d_n5;
        locals.var_t1__blk1145_dn6 = assign32820_e27145_d_n6;
        locals.var_t1__blk1145_dn7 = assign32820_e27145_d_n7;
        locals.var_t1__blk1145_dn8 = assign32820_e27145_d_n8;
        locals.var_t1__blk1145_dn9 = assign32820_e27145_d_n9;
        locals.var_t1__blk1145_dn10 = assign32820_e27145_d_n10;
        locals.var_t1__blk1145_dn11 = assign32820_e27145_d_n11;
        locals.var_t1__blk1145_dn12 = assign32820_e27145_d_n12;

        let (assign32830_e27171, assign32830_e27171_d_n3, assign32830_e27171_d_n4, assign32830_e27171_d_n5, assign32830_e27171_d_n6, assign32830_e27171_d_n7, assign32830_e27171_d_n8, assign32830_e27171_d_n9, assign32830_e27171_d_n10, assign32830_e27171_d_n11, assign32830_e27171_d_n12,) = {
    if (((((locals.var_guard1755 != 0.0) && (locals.var_guard1756 != 0.0)) && (locals.var_guard1757 == 0.0)) && (locals.var_guard1758 == 0.0)) && (locals.var_guard1759 != 0.0)) {
        let assign32830_e27159: f64 = (locals.var_b4soicsmin * locals.var_t10__blk1154);
        let assign32830_e27161: f64 = (assign32830_e27159 + locals.var_b4soist4);
        let assign32830_e27164: f64 = (locals.var_pparam_b4soist3 / 3.0);
        let assign32830_e27166: f64 = (assign32830_e27164 * locals.var_t0__blk1144);
        let assign32830_e27168: f64 = (assign32830_e27166 * locals.var_t1__blk1145);
        let assign32830_e27169: f64 = (assign32830_e27161 + assign32830_e27168);
        (assign32830_e27169, ((((locals.var_b4soicsmin_dn3 * locals.var_t10__blk1154) + (locals.var_b4soicsmin * locals.var_t10__blk1154_dn3)) + locals.var_b4soist4_dn3) + (((((locals.var_pparam_b4soist3_dn3 / 3.0) * locals.var_t0__blk1144) + (assign32830_e27164 * locals.var_t0__blk1144_dn3)) * locals.var_t1__blk1145) + (assign32830_e27166 * locals.var_t1__blk1145_dn3))), ((((locals.var_b4soicsmin_dn4 * locals.var_t10__blk1154) + (locals.var_b4soicsmin * locals.var_t10__blk1154_dn4)) + locals.var_b4soist4_dn4) + (((((locals.var_pparam_b4soist3_dn4 / 3.0) * locals.var_t0__blk1144) + (assign32830_e27164 * locals.var_t0__blk1144_dn4)) * locals.var_t1__blk1145) + (assign32830_e27166 * locals.var_t1__blk1145_dn4))), ((((locals.var_b4soicsmin_dn5 * locals.var_t10__blk1154) + (locals.var_b4soicsmin * locals.var_t10__blk1154_dn5)) + locals.var_b4soist4_dn5) + (((((locals.var_pparam_b4soist3_dn5 / 3.0) * locals.var_t0__blk1144) + (assign32830_e27164 * locals.var_t0__blk1144_dn5)) * locals.var_t1__blk1145) + (assign32830_e27166 * locals.var_t1__blk1145_dn5))), ((((locals.var_b4soicsmin_dn6 * locals.var_t10__blk1154) + (locals.var_b4soicsmin * locals.var_t10__blk1154_dn6)) + locals.var_b4soist4_dn6) + (((((locals.var_pparam_b4soist3_dn6 / 3.0) * locals.var_t0__blk1144) + (assign32830_e27164 * locals.var_t0__blk1144_dn6)) * locals.var_t1__blk1145) + (assign32830_e27166 * locals.var_t1__blk1145_dn6))), ((((locals.var_b4soicsmin_dn7 * locals.var_t10__blk1154) + (locals.var_b4soicsmin * locals.var_t10__blk1154_dn7)) + locals.var_b4soist4_dn7) + (((((locals.var_pparam_b4soist3_dn7 / 3.0) * locals.var_t0__blk1144) + (assign32830_e27164 * locals.var_t0__blk1144_dn7)) * locals.var_t1__blk1145) + (assign32830_e27166 * locals.var_t1__blk1145_dn7))), ((((locals.var_b4soicsmin_dn8 * locals.var_t10__blk1154) + (locals.var_b4soicsmin * locals.var_t10__blk1154_dn8)) + locals.var_b4soist4_dn8) + (((((locals.var_pparam_b4soist3_dn8 / 3.0) * locals.var_t0__blk1144) + (assign32830_e27164 * locals.var_t0__blk1144_dn8)) * locals.var_t1__blk1145) + (assign32830_e27166 * locals.var_t1__blk1145_dn8))), ((((locals.var_b4soicsmin_dn9 * locals.var_t10__blk1154) + (locals.var_b4soicsmin * locals.var_t10__blk1154_dn9)) + locals.var_b4soist4_dn9) + (((((locals.var_pparam_b4soist3_dn9 / 3.0) * locals.var_t0__blk1144) + (assign32830_e27164 * locals.var_t0__blk1144_dn9)) * locals.var_t1__blk1145) + (assign32830_e27166 * locals.var_t1__blk1145_dn9))), ((((locals.var_b4soicsmin_dn10 * locals.var_t10__blk1154) + (locals.var_b4soicsmin * locals.var_t10__blk1154_dn10)) + locals.var_b4soist4_dn10) + (((((locals.var_pparam_b4soist3_dn10 / 3.0) * locals.var_t0__blk1144) + (assign32830_e27164 * locals.var_t0__blk1144_dn10)) * locals.var_t1__blk1145) + (assign32830_e27166 * locals.var_t1__blk1145_dn10))), ((((locals.var_b4soicsmin_dn11 * locals.var_t10__blk1154) + (locals.var_b4soicsmin * locals.var_t10__blk1154_dn11)) + locals.var_b4soist4_dn11) + (((((locals.var_pparam_b4soist3_dn11 / 3.0) * locals.var_t0__blk1144) + (assign32830_e27164 * locals.var_t0__blk1144_dn11)) * locals.var_t1__blk1145) + (assign32830_e27166 * locals.var_t1__blk1145_dn11))), ((((locals.var_b4soicsmin_dn12 * locals.var_t10__blk1154) + (locals.var_b4soicsmin * locals.var_t10__blk1154_dn12)) + locals.var_b4soist4_dn12) + (((((locals.var_pparam_b4soist3_dn12 / 3.0) * locals.var_t0__blk1144) + (assign32830_e27164 * locals.var_t0__blk1144_dn12)) * locals.var_t1__blk1145) + (assign32830_e27166 * locals.var_t1__blk1145_dn12))),)
    } else {
        (locals.var_b4soiqse, locals.var_b4soiqse_dn3, locals.var_b4soiqse_dn4, locals.var_b4soiqse_dn5, locals.var_b4soiqse_dn6, locals.var_b4soiqse_dn7, locals.var_b4soiqse_dn8, locals.var_b4soiqse_dn9, locals.var_b4soiqse_dn10, locals.var_b4soiqse_dn11, locals.var_b4soiqse_dn12,)
    }
};
        locals.var_b4soiqse = assign32830_e27171;
        locals.var_b4soiqse_dn3 = assign32830_e27171_d_n3;
        locals.var_b4soiqse_dn4 = assign32830_e27171_d_n4;
        locals.var_b4soiqse_dn5 = assign32830_e27171_d_n5;
        locals.var_b4soiqse_dn6 = assign32830_e27171_d_n6;
        locals.var_b4soiqse_dn7 = assign32830_e27171_d_n7;
        locals.var_b4soiqse_dn8 = assign32830_e27171_d_n8;
        locals.var_b4soiqse_dn9 = assign32830_e27171_d_n9;
        locals.var_b4soiqse_dn10 = assign32830_e27171_d_n10;
        locals.var_b4soiqse_dn11 = assign32830_e27171_d_n11;
        locals.var_b4soiqse_dn12 = assign32830_e27171_d_n12;

        let (assign32840_e27190, assign32840_e27190_d_n3, assign32840_e27190_d_n4, assign32840_e27190_d_n5, assign32840_e27190_d_n6, assign32840_e27190_d_n7, assign32840_e27190_d_n8, assign32840_e27190_d_n9, assign32840_e27190_d_n10, assign32840_e27190_d_n11, assign32840_e27190_d_n12,) = {
    if (((((locals.var_guard1755 != 0.0) && (locals.var_guard1756 != 0.0)) && (locals.var_guard1757 == 0.0)) && (locals.var_guard1758 == 0.0)) && (locals.var_guard1759 == 0.0)) {
        let assign32840_e27186: f64 = (locals.var_b4soicsmin * locals.var_t10__blk1154);
        let assign32840_e27188: f64 = (assign32840_e27186 + locals.var_b4soist4);
        (assign32840_e27188, (((locals.var_b4soicsmin_dn3 * locals.var_t10__blk1154) + (locals.var_b4soicsmin * locals.var_t10__blk1154_dn3)) + locals.var_b4soist4_dn3), (((locals.var_b4soicsmin_dn4 * locals.var_t10__blk1154) + (locals.var_b4soicsmin * locals.var_t10__blk1154_dn4)) + locals.var_b4soist4_dn4), (((locals.var_b4soicsmin_dn5 * locals.var_t10__blk1154) + (locals.var_b4soicsmin * locals.var_t10__blk1154_dn5)) + locals.var_b4soist4_dn5), (((locals.var_b4soicsmin_dn6 * locals.var_t10__blk1154) + (locals.var_b4soicsmin * locals.var_t10__blk1154_dn6)) + locals.var_b4soist4_dn6), (((locals.var_b4soicsmin_dn7 * locals.var_t10__blk1154) + (locals.var_b4soicsmin * locals.var_t10__blk1154_dn7)) + locals.var_b4soist4_dn7), (((locals.var_b4soicsmin_dn8 * locals.var_t10__blk1154) + (locals.var_b4soicsmin * locals.var_t10__blk1154_dn8)) + locals.var_b4soist4_dn8), (((locals.var_b4soicsmin_dn9 * locals.var_t10__blk1154) + (locals.var_b4soicsmin * locals.var_t10__blk1154_dn9)) + locals.var_b4soist4_dn9), (((locals.var_b4soicsmin_dn10 * locals.var_t10__blk1154) + (locals.var_b4soicsmin * locals.var_t10__blk1154_dn10)) + locals.var_b4soist4_dn10), (((locals.var_b4soicsmin_dn11 * locals.var_t10__blk1154) + (locals.var_b4soicsmin * locals.var_t10__blk1154_dn11)) + locals.var_b4soist4_dn11), (((locals.var_b4soicsmin_dn12 * locals.var_t10__blk1154) + (locals.var_b4soicsmin * locals.var_t10__blk1154_dn12)) + locals.var_b4soist4_dn12),)
    } else {
        (locals.var_b4soiqse, locals.var_b4soiqse_dn3, locals.var_b4soiqse_dn4, locals.var_b4soiqse_dn5, locals.var_b4soiqse_dn6, locals.var_b4soiqse_dn7, locals.var_b4soiqse_dn8, locals.var_b4soiqse_dn9, locals.var_b4soiqse_dn10, locals.var_b4soiqse_dn11, locals.var_b4soiqse_dn12,)
    }
};
        locals.var_b4soiqse = assign32840_e27190;
        locals.var_b4soiqse_dn3 = assign32840_e27190_d_n3;
        locals.var_b4soiqse_dn4 = assign32840_e27190_d_n4;
        locals.var_b4soiqse_dn5 = assign32840_e27190_d_n5;
        locals.var_b4soiqse_dn6 = assign32840_e27190_d_n6;
        locals.var_b4soiqse_dn7 = assign32840_e27190_d_n7;
        locals.var_b4soiqse_dn8 = assign32840_e27190_d_n8;
        locals.var_b4soiqse_dn9 = assign32840_e27190_d_n9;
        locals.var_b4soiqse_dn10 = assign32840_e27190_d_n10;
        locals.var_b4soiqse_dn11 = assign32840_e27190_d_n11;
        locals.var_b4soiqse_dn12 = assign32840_e27190_d_n12;

        let assign32850_e27193: f64 = if locals.var_t10__blk1154 < locals.var_pparam_b4soivsdth { 1.0 } else { 0.0 };
        locals.var_guard1760 = assign32850_e27193;

        let (assign32860_e27206, assign32860_e27206_d_n3, assign32860_e27206_d_n4, assign32860_e27206_d_n5, assign32860_e27206_d_n6, assign32860_e27206_d_n7, assign32860_e27206_d_n8, assign32860_e27206_d_n9, assign32860_e27206_d_n10, assign32860_e27206_d_n11, assign32860_e27206_d_n12,) = {
    if (((locals.var_guard1755 != 0.0) && (locals.var_guard1756 == 0.0)) && (locals.var_guard1760 != 0.0)) {
        let assign32860_e27203: f64 = (locals.var_t10__blk1154 - locals.var_pparam_b4soivsdth);
        let assign32860_e27204: f64 = (locals.var_b4soicsmin * assign32860_e27203);
        (assign32860_e27204, ((locals.var_b4soicsmin_dn3 * assign32860_e27203) + (locals.var_b4soicsmin * (locals.var_t10__blk1154_dn3 - locals.var_pparam_b4soivsdth_dn3))), ((locals.var_b4soicsmin_dn4 * assign32860_e27203) + (locals.var_b4soicsmin * (locals.var_t10__blk1154_dn4 - locals.var_pparam_b4soivsdth_dn4))), ((locals.var_b4soicsmin_dn5 * assign32860_e27203) + (locals.var_b4soicsmin * (locals.var_t10__blk1154_dn5 - locals.var_pparam_b4soivsdth_dn5))), ((locals.var_b4soicsmin_dn6 * assign32860_e27203) + (locals.var_b4soicsmin * (locals.var_t10__blk1154_dn6 - locals.var_pparam_b4soivsdth_dn6))), ((locals.var_b4soicsmin_dn7 * assign32860_e27203) + (locals.var_b4soicsmin * (locals.var_t10__blk1154_dn7 - locals.var_pparam_b4soivsdth_dn7))), ((locals.var_b4soicsmin_dn8 * assign32860_e27203) + (locals.var_b4soicsmin * (locals.var_t10__blk1154_dn8 - locals.var_pparam_b4soivsdth_dn8))), ((locals.var_b4soicsmin_dn9 * assign32860_e27203) + (locals.var_b4soicsmin * (locals.var_t10__blk1154_dn9 - locals.var_pparam_b4soivsdth_dn9))), ((locals.var_b4soicsmin_dn10 * assign32860_e27203) + (locals.var_b4soicsmin * (locals.var_t10__blk1154_dn10 - locals.var_pparam_b4soivsdth_dn10))), ((locals.var_b4soicsmin_dn11 * assign32860_e27203) + (locals.var_b4soicsmin * (locals.var_t10__blk1154_dn11 - locals.var_pparam_b4soivsdth_dn11))), ((locals.var_b4soicsmin_dn12 * assign32860_e27203) + (locals.var_b4soicsmin * (locals.var_t10__blk1154_dn12 - locals.var_pparam_b4soivsdth_dn12))),)
    } else {
        (locals.var_b4soiqse, locals.var_b4soiqse_dn3, locals.var_b4soiqse_dn4, locals.var_b4soiqse_dn5, locals.var_b4soiqse_dn6, locals.var_b4soiqse_dn7, locals.var_b4soiqse_dn8, locals.var_b4soiqse_dn9, locals.var_b4soiqse_dn10, locals.var_b4soiqse_dn11, locals.var_b4soiqse_dn12,)
    }
};
        locals.var_b4soiqse = assign32860_e27206;
        locals.var_b4soiqse_dn3 = assign32860_e27206_d_n3;
        locals.var_b4soiqse_dn4 = assign32860_e27206_d_n4;
        locals.var_b4soiqse_dn5 = assign32860_e27206_d_n5;
        locals.var_b4soiqse_dn6 = assign32860_e27206_d_n6;
        locals.var_b4soiqse_dn7 = assign32860_e27206_d_n7;
        locals.var_b4soiqse_dn8 = assign32860_e27206_d_n8;
        locals.var_b4soiqse_dn9 = assign32860_e27206_d_n9;
        locals.var_b4soiqse_dn10 = assign32860_e27206_d_n10;
        locals.var_b4soiqse_dn11 = assign32860_e27206_d_n11;
        locals.var_b4soiqse_dn12 = assign32860_e27206_d_n12;

        let assign32870_e27209: f64 = if locals.var_t10__blk1154 < locals.var_pparam_b4soisdt1 { 1.0 } else { 0.0 };
        locals.var_guard1761 = assign32870_e27209;

        let (assign32880_e27223, assign32880_e27223_d_n3, assign32880_e27223_d_n4, assign32880_e27223_d_n5, assign32880_e27223_d_n6, assign32880_e27223_d_n7, assign32880_e27223_d_n8, assign32880_e27223_d_n9, assign32880_e27223_d_n10, assign32880_e27223_d_n11, assign32880_e27223_d_n12,) = {
    if ((((locals.var_guard1755 != 0.0) && (locals.var_guard1756 == 0.0)) && (locals.var_guard1760 == 0.0)) && (locals.var_guard1761 != 0.0)) {
        let assign32880_e27221: f64 = (locals.var_t10__blk1154 - locals.var_pparam_b4soivsdth);
        (assign32880_e27221, (locals.var_t10__blk1154_dn3 - locals.var_pparam_b4soivsdth_dn3), (locals.var_t10__blk1154_dn4 - locals.var_pparam_b4soivsdth_dn4), (locals.var_t10__blk1154_dn5 - locals.var_pparam_b4soivsdth_dn5), (locals.var_t10__blk1154_dn6 - locals.var_pparam_b4soivsdth_dn6), (locals.var_t10__blk1154_dn7 - locals.var_pparam_b4soivsdth_dn7), (locals.var_t10__blk1154_dn8 - locals.var_pparam_b4soivsdth_dn8), (locals.var_t10__blk1154_dn9 - locals.var_pparam_b4soivsdth_dn9), (locals.var_t10__blk1154_dn10 - locals.var_pparam_b4soivsdth_dn10), (locals.var_t10__blk1154_dn11 - locals.var_pparam_b4soivsdth_dn11), (locals.var_t10__blk1154_dn12 - locals.var_pparam_b4soivsdth_dn12),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign32880_e27223;
        locals.var_t0__blk1144_dn3 = assign32880_e27223_d_n3;
        locals.var_t0__blk1144_dn4 = assign32880_e27223_d_n4;
        locals.var_t0__blk1144_dn5 = assign32880_e27223_d_n5;
        locals.var_t0__blk1144_dn6 = assign32880_e27223_d_n6;
        locals.var_t0__blk1144_dn7 = assign32880_e27223_d_n7;
        locals.var_t0__blk1144_dn8 = assign32880_e27223_d_n8;
        locals.var_t0__blk1144_dn9 = assign32880_e27223_d_n9;
        locals.var_t0__blk1144_dn10 = assign32880_e27223_d_n10;
        locals.var_t0__blk1144_dn11 = assign32880_e27223_d_n11;
        locals.var_t0__blk1144_dn12 = assign32880_e27223_d_n12;

        let (assign32890_e27237, assign32890_e27237_d_n3, assign32890_e27237_d_n4, assign32890_e27237_d_n5, assign32890_e27237_d_n6, assign32890_e27237_d_n7, assign32890_e27237_d_n8, assign32890_e27237_d_n9, assign32890_e27237_d_n10, assign32890_e27237_d_n11, assign32890_e27237_d_n12,) = {
    if ((((locals.var_guard1755 != 0.0) && (locals.var_guard1756 == 0.0)) && (locals.var_guard1760 == 0.0)) && (locals.var_guard1761 != 0.0)) {
        let assign32890_e27235: f64 = (locals.var_t0__blk1144 * locals.var_t0__blk1144);
        (assign32890_e27235, ((locals.var_t0__blk1144_dn3 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn3)), ((locals.var_t0__blk1144_dn4 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn4)), ((locals.var_t0__blk1144_dn5 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn5)), ((locals.var_t0__blk1144_dn6 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn6)), ((locals.var_t0__blk1144_dn7 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn7)), ((locals.var_t0__blk1144_dn8 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn8)), ((locals.var_t0__blk1144_dn9 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn9)), ((locals.var_t0__blk1144_dn10 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn10)), ((locals.var_t0__blk1144_dn11 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn11)), ((locals.var_t0__blk1144_dn12 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn12)),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign32890_e27237;
        locals.var_t1__blk1145_dn3 = assign32890_e27237_d_n3;
        locals.var_t1__blk1145_dn4 = assign32890_e27237_d_n4;
        locals.var_t1__blk1145_dn5 = assign32890_e27237_d_n5;
        locals.var_t1__blk1145_dn6 = assign32890_e27237_d_n6;
        locals.var_t1__blk1145_dn7 = assign32890_e27237_d_n7;
        locals.var_t1__blk1145_dn8 = assign32890_e27237_d_n8;
        locals.var_t1__blk1145_dn9 = assign32890_e27237_d_n9;
        locals.var_t1__blk1145_dn10 = assign32890_e27237_d_n10;
        locals.var_t1__blk1145_dn11 = assign32890_e27237_d_n11;
        locals.var_t1__blk1145_dn12 = assign32890_e27237_d_n12;

        let (assign32900_e27257, assign32900_e27257_d_n3, assign32900_e27257_d_n4, assign32900_e27257_d_n5, assign32900_e27257_d_n6, assign32900_e27257_d_n7, assign32900_e27257_d_n8, assign32900_e27257_d_n9, assign32900_e27257_d_n10, assign32900_e27257_d_n11, assign32900_e27257_d_n12,) = {
    if ((((locals.var_guard1755 != 0.0) && (locals.var_guard1756 == 0.0)) && (locals.var_guard1760 == 0.0)) && (locals.var_guard1761 != 0.0)) {
        let assign32900_e27251: f64 = (locals.var_pparam_b4soist2 / 3.0);
        let assign32900_e27253: f64 = (assign32900_e27251 * locals.var_t1__blk1145);
        let assign32900_e27254: f64 = (locals.var_b4soicsmin - assign32900_e27253);
        let assign32900_e27255: f64 = (locals.var_t0__blk1144 * assign32900_e27254);
        (assign32900_e27255, ((locals.var_t0__blk1144_dn3 * assign32900_e27254) + (locals.var_t0__blk1144 * (locals.var_b4soicsmin_dn3 - (((locals.var_pparam_b4soist2_dn3 / 3.0) * locals.var_t1__blk1145) + (assign32900_e27251 * locals.var_t1__blk1145_dn3))))), ((locals.var_t0__blk1144_dn4 * assign32900_e27254) + (locals.var_t0__blk1144 * (locals.var_b4soicsmin_dn4 - (((locals.var_pparam_b4soist2_dn4 / 3.0) * locals.var_t1__blk1145) + (assign32900_e27251 * locals.var_t1__blk1145_dn4))))), ((locals.var_t0__blk1144_dn5 * assign32900_e27254) + (locals.var_t0__blk1144 * (locals.var_b4soicsmin_dn5 - (((locals.var_pparam_b4soist2_dn5 / 3.0) * locals.var_t1__blk1145) + (assign32900_e27251 * locals.var_t1__blk1145_dn5))))), ((locals.var_t0__blk1144_dn6 * assign32900_e27254) + (locals.var_t0__blk1144 * (locals.var_b4soicsmin_dn6 - (((locals.var_pparam_b4soist2_dn6 / 3.0) * locals.var_t1__blk1145) + (assign32900_e27251 * locals.var_t1__blk1145_dn6))))), ((locals.var_t0__blk1144_dn7 * assign32900_e27254) + (locals.var_t0__blk1144 * (locals.var_b4soicsmin_dn7 - (((locals.var_pparam_b4soist2_dn7 / 3.0) * locals.var_t1__blk1145) + (assign32900_e27251 * locals.var_t1__blk1145_dn7))))), ((locals.var_t0__blk1144_dn8 * assign32900_e27254) + (locals.var_t0__blk1144 * (locals.var_b4soicsmin_dn8 - (((locals.var_pparam_b4soist2_dn8 / 3.0) * locals.var_t1__blk1145) + (assign32900_e27251 * locals.var_t1__blk1145_dn8))))), ((locals.var_t0__blk1144_dn9 * assign32900_e27254) + (locals.var_t0__blk1144 * (locals.var_b4soicsmin_dn9 - (((locals.var_pparam_b4soist2_dn9 / 3.0) * locals.var_t1__blk1145) + (assign32900_e27251 * locals.var_t1__blk1145_dn9))))), ((locals.var_t0__blk1144_dn10 * assign32900_e27254) + (locals.var_t0__blk1144 * (locals.var_b4soicsmin_dn10 - (((locals.var_pparam_b4soist2_dn10 / 3.0) * locals.var_t1__blk1145) + (assign32900_e27251 * locals.var_t1__blk1145_dn10))))), ((locals.var_t0__blk1144_dn11 * assign32900_e27254) + (locals.var_t0__blk1144 * (locals.var_b4soicsmin_dn11 - (((locals.var_pparam_b4soist2_dn11 / 3.0) * locals.var_t1__blk1145) + (assign32900_e27251 * locals.var_t1__blk1145_dn11))))), ((locals.var_t0__blk1144_dn12 * assign32900_e27254) + (locals.var_t0__blk1144 * (locals.var_b4soicsmin_dn12 - (((locals.var_pparam_b4soist2_dn12 / 3.0) * locals.var_t1__blk1145) + (assign32900_e27251 * locals.var_t1__blk1145_dn12))))),)
    } else {
        (locals.var_b4soiqse, locals.var_b4soiqse_dn3, locals.var_b4soiqse_dn4, locals.var_b4soiqse_dn5, locals.var_b4soiqse_dn6, locals.var_b4soiqse_dn7, locals.var_b4soiqse_dn8, locals.var_b4soiqse_dn9, locals.var_b4soiqse_dn10, locals.var_b4soiqse_dn11, locals.var_b4soiqse_dn12,)
    }
};
        locals.var_b4soiqse = assign32900_e27257;
        locals.var_b4soiqse_dn3 = assign32900_e27257_d_n3;
        locals.var_b4soiqse_dn4 = assign32900_e27257_d_n4;
        locals.var_b4soiqse_dn5 = assign32900_e27257_d_n5;
        locals.var_b4soiqse_dn6 = assign32900_e27257_d_n6;
        locals.var_b4soiqse_dn7 = assign32900_e27257_d_n7;
        locals.var_b4soiqse_dn8 = assign32900_e27257_d_n8;
        locals.var_b4soiqse_dn9 = assign32900_e27257_d_n9;
        locals.var_b4soiqse_dn10 = assign32900_e27257_d_n10;
        locals.var_b4soiqse_dn11 = assign32900_e27257_d_n11;
        locals.var_b4soiqse_dn12 = assign32900_e27257_d_n12;

        let assign32910_e27260: f64 = if locals.var_t10__blk1154 < locals.var_pparam_b4soivsdfb { 1.0 } else { 0.0 };
        locals.var_guard1762 = assign32910_e27260;

        let (assign32920_e27277, assign32920_e27277_d_n3, assign32920_e27277_d_n4, assign32920_e27277_d_n5, assign32920_e27277_d_n6, assign32920_e27277_d_n7, assign32920_e27277_d_n8, assign32920_e27277_d_n9, assign32920_e27277_d_n10, assign32920_e27277_d_n11, assign32920_e27277_d_n12,) = {
    if (((((locals.var_guard1755 != 0.0) && (locals.var_guard1756 == 0.0)) && (locals.var_guard1760 == 0.0)) && (locals.var_guard1761 == 0.0)) && (locals.var_guard1762 != 0.0)) {
        let assign32920_e27275: f64 = (locals.var_t10__blk1154 - locals.var_pparam_b4soivsdfb);
        (assign32920_e27275, (locals.var_t10__blk1154_dn3 - locals.var_pparam_b4soivsdfb_dn3), (locals.var_t10__blk1154_dn4 - locals.var_pparam_b4soivsdfb_dn4), (locals.var_t10__blk1154_dn5 - locals.var_pparam_b4soivsdfb_dn5), (locals.var_t10__blk1154_dn6 - locals.var_pparam_b4soivsdfb_dn6), (locals.var_t10__blk1154_dn7 - locals.var_pparam_b4soivsdfb_dn7), (locals.var_t10__blk1154_dn8 - locals.var_pparam_b4soivsdfb_dn8), (locals.var_t10__blk1154_dn9 - locals.var_pparam_b4soivsdfb_dn9), (locals.var_t10__blk1154_dn10 - locals.var_pparam_b4soivsdfb_dn10), (locals.var_t10__blk1154_dn11 - locals.var_pparam_b4soivsdfb_dn11), (locals.var_t10__blk1154_dn12 - locals.var_pparam_b4soivsdfb_dn12),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign32920_e27277;
        locals.var_t0__blk1144_dn3 = assign32920_e27277_d_n3;
        locals.var_t0__blk1144_dn4 = assign32920_e27277_d_n4;
        locals.var_t0__blk1144_dn5 = assign32920_e27277_d_n5;
        locals.var_t0__blk1144_dn6 = assign32920_e27277_d_n6;
        locals.var_t0__blk1144_dn7 = assign32920_e27277_d_n7;
        locals.var_t0__blk1144_dn8 = assign32920_e27277_d_n8;
        locals.var_t0__blk1144_dn9 = assign32920_e27277_d_n9;
        locals.var_t0__blk1144_dn10 = assign32920_e27277_d_n10;
        locals.var_t0__blk1144_dn11 = assign32920_e27277_d_n11;
        locals.var_t0__blk1144_dn12 = assign32920_e27277_d_n12;

        let (assign32930_e27294, assign32930_e27294_d_n3, assign32930_e27294_d_n4, assign32930_e27294_d_n5, assign32930_e27294_d_n6, assign32930_e27294_d_n7, assign32930_e27294_d_n8, assign32930_e27294_d_n9, assign32930_e27294_d_n10, assign32930_e27294_d_n11, assign32930_e27294_d_n12,) = {
    if (((((locals.var_guard1755 != 0.0) && (locals.var_guard1756 == 0.0)) && (locals.var_guard1760 == 0.0)) && (locals.var_guard1761 == 0.0)) && (locals.var_guard1762 != 0.0)) {
        let assign32930_e27292: f64 = (locals.var_t0__blk1144 * locals.var_t0__blk1144);
        (assign32930_e27292, ((locals.var_t0__blk1144_dn3 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn3)), ((locals.var_t0__blk1144_dn4 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn4)), ((locals.var_t0__blk1144_dn5 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn5)), ((locals.var_t0__blk1144_dn6 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn6)), ((locals.var_t0__blk1144_dn7 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn7)), ((locals.var_t0__blk1144_dn8 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn8)), ((locals.var_t0__blk1144_dn9 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn9)), ((locals.var_t0__blk1144_dn10 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn10)), ((locals.var_t0__blk1144_dn11 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn11)), ((locals.var_t0__blk1144_dn12 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn12)),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign32930_e27294;
        locals.var_t1__blk1145_dn3 = assign32930_e27294_d_n3;
        locals.var_t1__blk1145_dn4 = assign32930_e27294_d_n4;
        locals.var_t1__blk1145_dn5 = assign32930_e27294_d_n5;
        locals.var_t1__blk1145_dn6 = assign32930_e27294_d_n6;
        locals.var_t1__blk1145_dn7 = assign32930_e27294_d_n7;
        locals.var_t1__blk1145_dn8 = assign32930_e27294_d_n8;
        locals.var_t1__blk1145_dn9 = assign32930_e27294_d_n9;
        locals.var_t1__blk1145_dn10 = assign32930_e27294_d_n10;
        locals.var_t1__blk1145_dn11 = assign32930_e27294_d_n11;
        locals.var_t1__blk1145_dn12 = assign32930_e27294_d_n12;

        let (assign32940_e27321, assign32940_e27321_d_n3, assign32940_e27321_d_n4, assign32940_e27321_d_n5, assign32940_e27321_d_n6, assign32940_e27321_d_n7, assign32940_e27321_d_n8, assign32940_e27321_d_n9, assign32940_e27321_d_n10, assign32940_e27321_d_n11, assign32940_e27321_d_n12,) = {
    if (((((locals.var_guard1755 != 0.0) && (locals.var_guard1756 == 0.0)) && (locals.var_guard1760 == 0.0)) && (locals.var_guard1761 == 0.0)) && (locals.var_guard1762 != 0.0)) {
        let assign32940_e27309: f64 = (locals.var_b4soicsbox * locals.var_t10__blk1154);
        let assign32940_e27311: f64 = (assign32940_e27309 + locals.var_b4soist4);
        let assign32940_e27314: f64 = (locals.var_pparam_b4soist3 / 3.0);
        let assign32940_e27316: f64 = (assign32940_e27314 * locals.var_t0__blk1144);
        let assign32940_e27318: f64 = (assign32940_e27316 * locals.var_t1__blk1145);
        let assign32940_e27319: f64 = (assign32940_e27311 + assign32940_e27318);
        (assign32940_e27319, (((locals.var_b4soicsbox * locals.var_t10__blk1154_dn3) + locals.var_b4soist4_dn3) + (((((locals.var_pparam_b4soist3_dn3 / 3.0) * locals.var_t0__blk1144) + (assign32940_e27314 * locals.var_t0__blk1144_dn3)) * locals.var_t1__blk1145) + (assign32940_e27316 * locals.var_t1__blk1145_dn3))), (((locals.var_b4soicsbox * locals.var_t10__blk1154_dn4) + locals.var_b4soist4_dn4) + (((((locals.var_pparam_b4soist3_dn4 / 3.0) * locals.var_t0__blk1144) + (assign32940_e27314 * locals.var_t0__blk1144_dn4)) * locals.var_t1__blk1145) + (assign32940_e27316 * locals.var_t1__blk1145_dn4))), (((locals.var_b4soicsbox * locals.var_t10__blk1154_dn5) + locals.var_b4soist4_dn5) + (((((locals.var_pparam_b4soist3_dn5 / 3.0) * locals.var_t0__blk1144) + (assign32940_e27314 * locals.var_t0__blk1144_dn5)) * locals.var_t1__blk1145) + (assign32940_e27316 * locals.var_t1__blk1145_dn5))), (((locals.var_b4soicsbox * locals.var_t10__blk1154_dn6) + locals.var_b4soist4_dn6) + (((((locals.var_pparam_b4soist3_dn6 / 3.0) * locals.var_t0__blk1144) + (assign32940_e27314 * locals.var_t0__blk1144_dn6)) * locals.var_t1__blk1145) + (assign32940_e27316 * locals.var_t1__blk1145_dn6))), (((locals.var_b4soicsbox * locals.var_t10__blk1154_dn7) + locals.var_b4soist4_dn7) + (((((locals.var_pparam_b4soist3_dn7 / 3.0) * locals.var_t0__blk1144) + (assign32940_e27314 * locals.var_t0__blk1144_dn7)) * locals.var_t1__blk1145) + (assign32940_e27316 * locals.var_t1__blk1145_dn7))), (((locals.var_b4soicsbox * locals.var_t10__blk1154_dn8) + locals.var_b4soist4_dn8) + (((((locals.var_pparam_b4soist3_dn8 / 3.0) * locals.var_t0__blk1144) + (assign32940_e27314 * locals.var_t0__blk1144_dn8)) * locals.var_t1__blk1145) + (assign32940_e27316 * locals.var_t1__blk1145_dn8))), (((locals.var_b4soicsbox * locals.var_t10__blk1154_dn9) + locals.var_b4soist4_dn9) + (((((locals.var_pparam_b4soist3_dn9 / 3.0) * locals.var_t0__blk1144) + (assign32940_e27314 * locals.var_t0__blk1144_dn9)) * locals.var_t1__blk1145) + (assign32940_e27316 * locals.var_t1__blk1145_dn9))), (((locals.var_b4soicsbox * locals.var_t10__blk1154_dn10) + locals.var_b4soist4_dn10) + (((((locals.var_pparam_b4soist3_dn10 / 3.0) * locals.var_t0__blk1144) + (assign32940_e27314 * locals.var_t0__blk1144_dn10)) * locals.var_t1__blk1145) + (assign32940_e27316 * locals.var_t1__blk1145_dn10))), (((locals.var_b4soicsbox * locals.var_t10__blk1154_dn11) + locals.var_b4soist4_dn11) + (((((locals.var_pparam_b4soist3_dn11 / 3.0) * locals.var_t0__blk1144) + (assign32940_e27314 * locals.var_t0__blk1144_dn11)) * locals.var_t1__blk1145) + (assign32940_e27316 * locals.var_t1__blk1145_dn11))), (((locals.var_b4soicsbox * locals.var_t10__blk1154_dn12) + locals.var_b4soist4_dn12) + (((((locals.var_pparam_b4soist3_dn12 / 3.0) * locals.var_t0__blk1144) + (assign32940_e27314 * locals.var_t0__blk1144_dn12)) * locals.var_t1__blk1145) + (assign32940_e27316 * locals.var_t1__blk1145_dn12))),)
    } else {
        (locals.var_b4soiqse, locals.var_b4soiqse_dn3, locals.var_b4soiqse_dn4, locals.var_b4soiqse_dn5, locals.var_b4soiqse_dn6, locals.var_b4soiqse_dn7, locals.var_b4soiqse_dn8, locals.var_b4soiqse_dn9, locals.var_b4soiqse_dn10, locals.var_b4soiqse_dn11, locals.var_b4soiqse_dn12,)
    }
};
        locals.var_b4soiqse = assign32940_e27321;
        locals.var_b4soiqse_dn3 = assign32940_e27321_d_n3;
        locals.var_b4soiqse_dn4 = assign32940_e27321_d_n4;
        locals.var_b4soiqse_dn5 = assign32940_e27321_d_n5;
        locals.var_b4soiqse_dn6 = assign32940_e27321_d_n6;
        locals.var_b4soiqse_dn7 = assign32940_e27321_d_n7;
        locals.var_b4soiqse_dn8 = assign32940_e27321_d_n8;
        locals.var_b4soiqse_dn9 = assign32940_e27321_d_n9;
        locals.var_b4soiqse_dn10 = assign32940_e27321_d_n10;
        locals.var_b4soiqse_dn11 = assign32940_e27321_d_n11;
        locals.var_b4soiqse_dn12 = assign32940_e27321_d_n12;

    }

    pub(super) fn stamp_transient_block_91(
        locals: &mut StampLocals,
    ) {
        let (assign32950_e27341, assign32950_e27341_d_n3, assign32950_e27341_d_n4, assign32950_e27341_d_n5, assign32950_e27341_d_n6, assign32950_e27341_d_n7, assign32950_e27341_d_n8, assign32950_e27341_d_n9, assign32950_e27341_d_n10, assign32950_e27341_d_n11, assign32950_e27341_d_n12,) = {
    if (((((locals.var_guard1755 != 0.0) && (locals.var_guard1756 == 0.0)) && (locals.var_guard1760 == 0.0)) && (locals.var_guard1761 == 0.0)) && (locals.var_guard1762 == 0.0)) {
        let assign32950_e27337: f64 = (locals.var_b4soicsbox * locals.var_t10__blk1154);
        let assign32950_e27339: f64 = (assign32950_e27337 + locals.var_b4soist4);
        (assign32950_e27339, ((locals.var_b4soicsbox * locals.var_t10__blk1154_dn3) + locals.var_b4soist4_dn3), ((locals.var_b4soicsbox * locals.var_t10__blk1154_dn4) + locals.var_b4soist4_dn4), ((locals.var_b4soicsbox * locals.var_t10__blk1154_dn5) + locals.var_b4soist4_dn5), ((locals.var_b4soicsbox * locals.var_t10__blk1154_dn6) + locals.var_b4soist4_dn6), ((locals.var_b4soicsbox * locals.var_t10__blk1154_dn7) + locals.var_b4soist4_dn7), ((locals.var_b4soicsbox * locals.var_t10__blk1154_dn8) + locals.var_b4soist4_dn8), ((locals.var_b4soicsbox * locals.var_t10__blk1154_dn9) + locals.var_b4soist4_dn9), ((locals.var_b4soicsbox * locals.var_t10__blk1154_dn10) + locals.var_b4soist4_dn10), ((locals.var_b4soicsbox * locals.var_t10__blk1154_dn11) + locals.var_b4soist4_dn11), ((locals.var_b4soicsbox * locals.var_t10__blk1154_dn12) + locals.var_b4soist4_dn12),)
    } else {
        (locals.var_b4soiqse, locals.var_b4soiqse_dn3, locals.var_b4soiqse_dn4, locals.var_b4soiqse_dn5, locals.var_b4soiqse_dn6, locals.var_b4soiqse_dn7, locals.var_b4soiqse_dn8, locals.var_b4soiqse_dn9, locals.var_b4soiqse_dn10, locals.var_b4soiqse_dn11, locals.var_b4soiqse_dn12,)
    }
};
        locals.var_b4soiqse = assign32950_e27341;
        locals.var_b4soiqse_dn3 = assign32950_e27341_d_n3;
        locals.var_b4soiqse_dn4 = assign32950_e27341_d_n4;
        locals.var_b4soiqse_dn5 = assign32950_e27341_d_n5;
        locals.var_b4soiqse_dn6 = assign32950_e27341_d_n6;
        locals.var_b4soiqse_dn7 = assign32950_e27341_d_n7;
        locals.var_b4soiqse_dn8 = assign32950_e27341_d_n8;
        locals.var_b4soiqse_dn9 = assign32950_e27341_d_n9;
        locals.var_b4soiqse_dn10 = assign32950_e27341_d_n10;
        locals.var_b4soiqse_dn11 = assign32950_e27341_d_n11;
        locals.var_b4soiqse_dn12 = assign32950_e27341_d_n12;

        let assign32960_e27356: f64 = if (((locals.var_pparam_b4soinsub > 0.0) && (locals.var_b4soitype > 0.0)) || ((locals.var_pparam_b4soinsub < 0.0) && (locals.var_b4soitype < 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard1763 = assign32960_e27356;

        let assign32970_e27359: f64 = if locals.var_t11 < locals.var_pparam_b4soivsdfb { 1.0 } else { 0.0 };
        locals.var_guard1764 = assign32970_e27359;

        let (assign32980_e27371, assign32980_e27371_d_n3, assign32980_e27371_d_n4, assign32980_e27371_d_n5, assign32980_e27371_d_n6, assign32980_e27371_d_n7, assign32980_e27371_d_n8, assign32980_e27371_d_n9, assign32980_e27371_d_n10, assign32980_e27371_d_n11, assign32980_e27371_d_n12,) = {
    if (((locals.var_guard1755 != 0.0) && (locals.var_guard1763 != 0.0)) && (locals.var_guard1764 != 0.0)) {
        let assign32980_e27368: f64 = (locals.var_t11 - locals.var_pparam_b4soivsdfb);
        let assign32980_e27369: f64 = (locals.var_b4soicdbox * assign32980_e27368);
        (assign32980_e27369, (locals.var_b4soicdbox * (locals.var_t11_dn3 - locals.var_pparam_b4soivsdfb_dn3)), (locals.var_b4soicdbox * (locals.var_t11_dn4 - locals.var_pparam_b4soivsdfb_dn4)), (locals.var_b4soicdbox * (locals.var_t11_dn5 - locals.var_pparam_b4soivsdfb_dn5)), (locals.var_b4soicdbox * (locals.var_t11_dn6 - locals.var_pparam_b4soivsdfb_dn6)), (locals.var_b4soicdbox * (locals.var_t11_dn7 - locals.var_pparam_b4soivsdfb_dn7)), (locals.var_b4soicdbox * (locals.var_t11_dn8 - locals.var_pparam_b4soivsdfb_dn8)), (locals.var_b4soicdbox * (locals.var_t11_dn9 - locals.var_pparam_b4soivsdfb_dn9)), (locals.var_b4soicdbox * (locals.var_t11_dn10 - locals.var_pparam_b4soivsdfb_dn10)), (locals.var_b4soicdbox * (locals.var_t11_dn11 - locals.var_pparam_b4soivsdfb_dn11)), (locals.var_b4soicdbox * (locals.var_t11_dn12 - locals.var_pparam_b4soivsdfb_dn12)),)
    } else {
        (locals.var_b4soiqde, locals.var_b4soiqde_dn3, locals.var_b4soiqde_dn4, locals.var_b4soiqde_dn5, locals.var_b4soiqde_dn6, locals.var_b4soiqde_dn7, locals.var_b4soiqde_dn8, locals.var_b4soiqde_dn9, locals.var_b4soiqde_dn10, locals.var_b4soiqde_dn11, locals.var_b4soiqde_dn12,)
    }
};
        locals.var_b4soiqde = assign32980_e27371;
        locals.var_b4soiqde_dn3 = assign32980_e27371_d_n3;
        locals.var_b4soiqde_dn4 = assign32980_e27371_d_n4;
        locals.var_b4soiqde_dn5 = assign32980_e27371_d_n5;
        locals.var_b4soiqde_dn6 = assign32980_e27371_d_n6;
        locals.var_b4soiqde_dn7 = assign32980_e27371_d_n7;
        locals.var_b4soiqde_dn8 = assign32980_e27371_d_n8;
        locals.var_b4soiqde_dn9 = assign32980_e27371_d_n9;
        locals.var_b4soiqde_dn10 = assign32980_e27371_d_n10;
        locals.var_b4soiqde_dn11 = assign32980_e27371_d_n11;
        locals.var_b4soiqde_dn12 = assign32980_e27371_d_n12;

        let assign32990_e27374: f64 = if locals.var_t11 < locals.var_pparam_b4soisdt1 { 1.0 } else { 0.0 };
        locals.var_guard1765 = assign32990_e27374;

        let (assign33000_e27387, assign33000_e27387_d_n3, assign33000_e27387_d_n4, assign33000_e27387_d_n5, assign33000_e27387_d_n6, assign33000_e27387_d_n7, assign33000_e27387_d_n8, assign33000_e27387_d_n9, assign33000_e27387_d_n10, assign33000_e27387_d_n11, assign33000_e27387_d_n12,) = {
    if ((((locals.var_guard1755 != 0.0) && (locals.var_guard1763 != 0.0)) && (locals.var_guard1764 == 0.0)) && (locals.var_guard1765 != 0.0)) {
        let assign33000_e27385: f64 = (locals.var_t11 - locals.var_pparam_b4soivsdfb);
        (assign33000_e27385, (locals.var_t11_dn3 - locals.var_pparam_b4soivsdfb_dn3), (locals.var_t11_dn4 - locals.var_pparam_b4soivsdfb_dn4), (locals.var_t11_dn5 - locals.var_pparam_b4soivsdfb_dn5), (locals.var_t11_dn6 - locals.var_pparam_b4soivsdfb_dn6), (locals.var_t11_dn7 - locals.var_pparam_b4soivsdfb_dn7), (locals.var_t11_dn8 - locals.var_pparam_b4soivsdfb_dn8), (locals.var_t11_dn9 - locals.var_pparam_b4soivsdfb_dn9), (locals.var_t11_dn10 - locals.var_pparam_b4soivsdfb_dn10), (locals.var_t11_dn11 - locals.var_pparam_b4soivsdfb_dn11), (locals.var_t11_dn12 - locals.var_pparam_b4soivsdfb_dn12),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign33000_e27387;
        locals.var_t0__blk1144_dn3 = assign33000_e27387_d_n3;
        locals.var_t0__blk1144_dn4 = assign33000_e27387_d_n4;
        locals.var_t0__blk1144_dn5 = assign33000_e27387_d_n5;
        locals.var_t0__blk1144_dn6 = assign33000_e27387_d_n6;
        locals.var_t0__blk1144_dn7 = assign33000_e27387_d_n7;
        locals.var_t0__blk1144_dn8 = assign33000_e27387_d_n8;
        locals.var_t0__blk1144_dn9 = assign33000_e27387_d_n9;
        locals.var_t0__blk1144_dn10 = assign33000_e27387_d_n10;
        locals.var_t0__blk1144_dn11 = assign33000_e27387_d_n11;
        locals.var_t0__blk1144_dn12 = assign33000_e27387_d_n12;

        let (assign33010_e27400, assign33010_e27400_d_n3, assign33010_e27400_d_n4, assign33010_e27400_d_n5, assign33010_e27400_d_n6, assign33010_e27400_d_n7, assign33010_e27400_d_n8, assign33010_e27400_d_n9, assign33010_e27400_d_n10, assign33010_e27400_d_n11, assign33010_e27400_d_n12,) = {
    if ((((locals.var_guard1755 != 0.0) && (locals.var_guard1763 != 0.0)) && (locals.var_guard1764 == 0.0)) && (locals.var_guard1765 != 0.0)) {
        let assign33010_e27398: f64 = (locals.var_t0__blk1144 * locals.var_t0__blk1144);
        (assign33010_e27398, ((locals.var_t0__blk1144_dn3 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn3)), ((locals.var_t0__blk1144_dn4 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn4)), ((locals.var_t0__blk1144_dn5 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn5)), ((locals.var_t0__blk1144_dn6 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn6)), ((locals.var_t0__blk1144_dn7 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn7)), ((locals.var_t0__blk1144_dn8 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn8)), ((locals.var_t0__blk1144_dn9 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn9)), ((locals.var_t0__blk1144_dn10 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn10)), ((locals.var_t0__blk1144_dn11 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn11)), ((locals.var_t0__blk1144_dn12 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn12)),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign33010_e27400;
        locals.var_t1__blk1145_dn3 = assign33010_e27400_d_n3;
        locals.var_t1__blk1145_dn4 = assign33010_e27400_d_n4;
        locals.var_t1__blk1145_dn5 = assign33010_e27400_d_n5;
        locals.var_t1__blk1145_dn6 = assign33010_e27400_d_n6;
        locals.var_t1__blk1145_dn7 = assign33010_e27400_d_n7;
        locals.var_t1__blk1145_dn8 = assign33010_e27400_d_n8;
        locals.var_t1__blk1145_dn9 = assign33010_e27400_d_n9;
        locals.var_t1__blk1145_dn10 = assign33010_e27400_d_n10;
        locals.var_t1__blk1145_dn11 = assign33010_e27400_d_n11;
        locals.var_t1__blk1145_dn12 = assign33010_e27400_d_n12;

        let (assign33020_e27419, assign33020_e27419_d_n3, assign33020_e27419_d_n4, assign33020_e27419_d_n5, assign33020_e27419_d_n6, assign33020_e27419_d_n7, assign33020_e27419_d_n8, assign33020_e27419_d_n9, assign33020_e27419_d_n10, assign33020_e27419_d_n11, assign33020_e27419_d_n12,) = {
    if ((((locals.var_guard1755 != 0.0) && (locals.var_guard1763 != 0.0)) && (locals.var_guard1764 == 0.0)) && (locals.var_guard1765 != 0.0)) {
        let assign33020_e27413: f64 = (locals.var_pparam_b4soidt2 / 3.0);
        let assign33020_e27415: f64 = (assign33020_e27413 * locals.var_t1__blk1145);
        let assign33020_e27416: f64 = (locals.var_b4soicdbox - assign33020_e27415);
        let assign33020_e27417: f64 = (locals.var_t0__blk1144 * assign33020_e27416);
        (assign33020_e27417, ((locals.var_t0__blk1144_dn3 * assign33020_e27416) + (locals.var_t0__blk1144 * (-(((locals.var_pparam_b4soidt2_dn3 / 3.0) * locals.var_t1__blk1145) + (assign33020_e27413 * locals.var_t1__blk1145_dn3))))), ((locals.var_t0__blk1144_dn4 * assign33020_e27416) + (locals.var_t0__blk1144 * (-(((locals.var_pparam_b4soidt2_dn4 / 3.0) * locals.var_t1__blk1145) + (assign33020_e27413 * locals.var_t1__blk1145_dn4))))), ((locals.var_t0__blk1144_dn5 * assign33020_e27416) + (locals.var_t0__blk1144 * (-(((locals.var_pparam_b4soidt2_dn5 / 3.0) * locals.var_t1__blk1145) + (assign33020_e27413 * locals.var_t1__blk1145_dn5))))), ((locals.var_t0__blk1144_dn6 * assign33020_e27416) + (locals.var_t0__blk1144 * (-(((locals.var_pparam_b4soidt2_dn6 / 3.0) * locals.var_t1__blk1145) + (assign33020_e27413 * locals.var_t1__blk1145_dn6))))), ((locals.var_t0__blk1144_dn7 * assign33020_e27416) + (locals.var_t0__blk1144 * (-(((locals.var_pparam_b4soidt2_dn7 / 3.0) * locals.var_t1__blk1145) + (assign33020_e27413 * locals.var_t1__blk1145_dn7))))), ((locals.var_t0__blk1144_dn8 * assign33020_e27416) + (locals.var_t0__blk1144 * (-(((locals.var_pparam_b4soidt2_dn8 / 3.0) * locals.var_t1__blk1145) + (assign33020_e27413 * locals.var_t1__blk1145_dn8))))), ((locals.var_t0__blk1144_dn9 * assign33020_e27416) + (locals.var_t0__blk1144 * (-(((locals.var_pparam_b4soidt2_dn9 / 3.0) * locals.var_t1__blk1145) + (assign33020_e27413 * locals.var_t1__blk1145_dn9))))), ((locals.var_t0__blk1144_dn10 * assign33020_e27416) + (locals.var_t0__blk1144 * (-(((locals.var_pparam_b4soidt2_dn10 / 3.0) * locals.var_t1__blk1145) + (assign33020_e27413 * locals.var_t1__blk1145_dn10))))), ((locals.var_t0__blk1144_dn11 * assign33020_e27416) + (locals.var_t0__blk1144 * (-(((locals.var_pparam_b4soidt2_dn11 / 3.0) * locals.var_t1__blk1145) + (assign33020_e27413 * locals.var_t1__blk1145_dn11))))), ((locals.var_t0__blk1144_dn12 * assign33020_e27416) + (locals.var_t0__blk1144 * (-(((locals.var_pparam_b4soidt2_dn12 / 3.0) * locals.var_t1__blk1145) + (assign33020_e27413 * locals.var_t1__blk1145_dn12))))),)
    } else {
        (locals.var_b4soiqde, locals.var_b4soiqde_dn3, locals.var_b4soiqde_dn4, locals.var_b4soiqde_dn5, locals.var_b4soiqde_dn6, locals.var_b4soiqde_dn7, locals.var_b4soiqde_dn8, locals.var_b4soiqde_dn9, locals.var_b4soiqde_dn10, locals.var_b4soiqde_dn11, locals.var_b4soiqde_dn12,)
    }
};
        locals.var_b4soiqde = assign33020_e27419;
        locals.var_b4soiqde_dn3 = assign33020_e27419_d_n3;
        locals.var_b4soiqde_dn4 = assign33020_e27419_d_n4;
        locals.var_b4soiqde_dn5 = assign33020_e27419_d_n5;
        locals.var_b4soiqde_dn6 = assign33020_e27419_d_n6;
        locals.var_b4soiqde_dn7 = assign33020_e27419_d_n7;
        locals.var_b4soiqde_dn8 = assign33020_e27419_d_n8;
        locals.var_b4soiqde_dn9 = assign33020_e27419_d_n9;
        locals.var_b4soiqde_dn10 = assign33020_e27419_d_n10;
        locals.var_b4soiqde_dn11 = assign33020_e27419_d_n11;
        locals.var_b4soiqde_dn12 = assign33020_e27419_d_n12;

        let assign33030_e27422: f64 = if locals.var_t11 < locals.var_pparam_b4soivsdth { 1.0 } else { 0.0 };
        locals.var_guard1766 = assign33030_e27422;

        let (assign33040_e27438, assign33040_e27438_d_n3, assign33040_e27438_d_n4, assign33040_e27438_d_n5, assign33040_e27438_d_n6, assign33040_e27438_d_n7, assign33040_e27438_d_n8, assign33040_e27438_d_n9, assign33040_e27438_d_n10, assign33040_e27438_d_n11, assign33040_e27438_d_n12,) = {
    if (((((locals.var_guard1755 != 0.0) && (locals.var_guard1763 != 0.0)) && (locals.var_guard1764 == 0.0)) && (locals.var_guard1765 == 0.0)) && (locals.var_guard1766 != 0.0)) {
        let assign33040_e27436: f64 = (locals.var_t11 - locals.var_pparam_b4soivsdth);
        (assign33040_e27436, (locals.var_t11_dn3 - locals.var_pparam_b4soivsdth_dn3), (locals.var_t11_dn4 - locals.var_pparam_b4soivsdth_dn4), (locals.var_t11_dn5 - locals.var_pparam_b4soivsdth_dn5), (locals.var_t11_dn6 - locals.var_pparam_b4soivsdth_dn6), (locals.var_t11_dn7 - locals.var_pparam_b4soivsdth_dn7), (locals.var_t11_dn8 - locals.var_pparam_b4soivsdth_dn8), (locals.var_t11_dn9 - locals.var_pparam_b4soivsdth_dn9), (locals.var_t11_dn10 - locals.var_pparam_b4soivsdth_dn10), (locals.var_t11_dn11 - locals.var_pparam_b4soivsdth_dn11), (locals.var_t11_dn12 - locals.var_pparam_b4soivsdth_dn12),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign33040_e27438;
        locals.var_t0__blk1144_dn3 = assign33040_e27438_d_n3;
        locals.var_t0__blk1144_dn4 = assign33040_e27438_d_n4;
        locals.var_t0__blk1144_dn5 = assign33040_e27438_d_n5;
        locals.var_t0__blk1144_dn6 = assign33040_e27438_d_n6;
        locals.var_t0__blk1144_dn7 = assign33040_e27438_d_n7;
        locals.var_t0__blk1144_dn8 = assign33040_e27438_d_n8;
        locals.var_t0__blk1144_dn9 = assign33040_e27438_d_n9;
        locals.var_t0__blk1144_dn10 = assign33040_e27438_d_n10;
        locals.var_t0__blk1144_dn11 = assign33040_e27438_d_n11;
        locals.var_t0__blk1144_dn12 = assign33040_e27438_d_n12;

        let (assign33050_e27454, assign33050_e27454_d_n3, assign33050_e27454_d_n4, assign33050_e27454_d_n5, assign33050_e27454_d_n6, assign33050_e27454_d_n7, assign33050_e27454_d_n8, assign33050_e27454_d_n9, assign33050_e27454_d_n10, assign33050_e27454_d_n11, assign33050_e27454_d_n12,) = {
    if (((((locals.var_guard1755 != 0.0) && (locals.var_guard1763 != 0.0)) && (locals.var_guard1764 == 0.0)) && (locals.var_guard1765 == 0.0)) && (locals.var_guard1766 != 0.0)) {
        let assign33050_e27452: f64 = (locals.var_t0__blk1144 * locals.var_t0__blk1144);
        (assign33050_e27452, ((locals.var_t0__blk1144_dn3 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn3)), ((locals.var_t0__blk1144_dn4 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn4)), ((locals.var_t0__blk1144_dn5 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn5)), ((locals.var_t0__blk1144_dn6 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn6)), ((locals.var_t0__blk1144_dn7 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn7)), ((locals.var_t0__blk1144_dn8 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn8)), ((locals.var_t0__blk1144_dn9 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn9)), ((locals.var_t0__blk1144_dn10 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn10)), ((locals.var_t0__blk1144_dn11 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn11)), ((locals.var_t0__blk1144_dn12 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn12)),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign33050_e27454;
        locals.var_t1__blk1145_dn3 = assign33050_e27454_d_n3;
        locals.var_t1__blk1145_dn4 = assign33050_e27454_d_n4;
        locals.var_t1__blk1145_dn5 = assign33050_e27454_d_n5;
        locals.var_t1__blk1145_dn6 = assign33050_e27454_d_n6;
        locals.var_t1__blk1145_dn7 = assign33050_e27454_d_n7;
        locals.var_t1__blk1145_dn8 = assign33050_e27454_d_n8;
        locals.var_t1__blk1145_dn9 = assign33050_e27454_d_n9;
        locals.var_t1__blk1145_dn10 = assign33050_e27454_d_n10;
        locals.var_t1__blk1145_dn11 = assign33050_e27454_d_n11;
        locals.var_t1__blk1145_dn12 = assign33050_e27454_d_n12;

        let (assign33060_e27480, assign33060_e27480_d_n3, assign33060_e27480_d_n4, assign33060_e27480_d_n5, assign33060_e27480_d_n6, assign33060_e27480_d_n7, assign33060_e27480_d_n8, assign33060_e27480_d_n9, assign33060_e27480_d_n10, assign33060_e27480_d_n11, assign33060_e27480_d_n12,) = {
    if (((((locals.var_guard1755 != 0.0) && (locals.var_guard1763 != 0.0)) && (locals.var_guard1764 == 0.0)) && (locals.var_guard1765 == 0.0)) && (locals.var_guard1766 != 0.0)) {
        let assign33060_e27468: f64 = (locals.var_b4soicdmin * locals.var_t11);
        let assign33060_e27470: f64 = (assign33060_e27468 + locals.var_b4soidt4);
        let assign33060_e27473: f64 = (locals.var_pparam_b4soidt3 / 3.0);
        let assign33060_e27475: f64 = (assign33060_e27473 * locals.var_t0__blk1144);
        let assign33060_e27477: f64 = (assign33060_e27475 * locals.var_t1__blk1145);
        let assign33060_e27478: f64 = (assign33060_e27470 + assign33060_e27477);
        (assign33060_e27478, ((((locals.var_b4soicdmin_dn3 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn3)) + locals.var_b4soidt4_dn3) + (((((locals.var_pparam_b4soidt3_dn3 / 3.0) * locals.var_t0__blk1144) + (assign33060_e27473 * locals.var_t0__blk1144_dn3)) * locals.var_t1__blk1145) + (assign33060_e27475 * locals.var_t1__blk1145_dn3))), ((((locals.var_b4soicdmin_dn4 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn4)) + locals.var_b4soidt4_dn4) + (((((locals.var_pparam_b4soidt3_dn4 / 3.0) * locals.var_t0__blk1144) + (assign33060_e27473 * locals.var_t0__blk1144_dn4)) * locals.var_t1__blk1145) + (assign33060_e27475 * locals.var_t1__blk1145_dn4))), ((((locals.var_b4soicdmin_dn5 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn5)) + locals.var_b4soidt4_dn5) + (((((locals.var_pparam_b4soidt3_dn5 / 3.0) * locals.var_t0__blk1144) + (assign33060_e27473 * locals.var_t0__blk1144_dn5)) * locals.var_t1__blk1145) + (assign33060_e27475 * locals.var_t1__blk1145_dn5))), ((((locals.var_b4soicdmin_dn6 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn6)) + locals.var_b4soidt4_dn6) + (((((locals.var_pparam_b4soidt3_dn6 / 3.0) * locals.var_t0__blk1144) + (assign33060_e27473 * locals.var_t0__blk1144_dn6)) * locals.var_t1__blk1145) + (assign33060_e27475 * locals.var_t1__blk1145_dn6))), ((((locals.var_b4soicdmin_dn7 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn7)) + locals.var_b4soidt4_dn7) + (((((locals.var_pparam_b4soidt3_dn7 / 3.0) * locals.var_t0__blk1144) + (assign33060_e27473 * locals.var_t0__blk1144_dn7)) * locals.var_t1__blk1145) + (assign33060_e27475 * locals.var_t1__blk1145_dn7))), ((((locals.var_b4soicdmin_dn8 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn8)) + locals.var_b4soidt4_dn8) + (((((locals.var_pparam_b4soidt3_dn8 / 3.0) * locals.var_t0__blk1144) + (assign33060_e27473 * locals.var_t0__blk1144_dn8)) * locals.var_t1__blk1145) + (assign33060_e27475 * locals.var_t1__blk1145_dn8))), ((((locals.var_b4soicdmin_dn9 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn9)) + locals.var_b4soidt4_dn9) + (((((locals.var_pparam_b4soidt3_dn9 / 3.0) * locals.var_t0__blk1144) + (assign33060_e27473 * locals.var_t0__blk1144_dn9)) * locals.var_t1__blk1145) + (assign33060_e27475 * locals.var_t1__blk1145_dn9))), ((((locals.var_b4soicdmin_dn10 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn10)) + locals.var_b4soidt4_dn10) + (((((locals.var_pparam_b4soidt3_dn10 / 3.0) * locals.var_t0__blk1144) + (assign33060_e27473 * locals.var_t0__blk1144_dn10)) * locals.var_t1__blk1145) + (assign33060_e27475 * locals.var_t1__blk1145_dn10))), ((((locals.var_b4soicdmin_dn11 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn11)) + locals.var_b4soidt4_dn11) + (((((locals.var_pparam_b4soidt3_dn11 / 3.0) * locals.var_t0__blk1144) + (assign33060_e27473 * locals.var_t0__blk1144_dn11)) * locals.var_t1__blk1145) + (assign33060_e27475 * locals.var_t1__blk1145_dn11))), ((((locals.var_b4soicdmin_dn12 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn12)) + locals.var_b4soidt4_dn12) + (((((locals.var_pparam_b4soidt3_dn12 / 3.0) * locals.var_t0__blk1144) + (assign33060_e27473 * locals.var_t0__blk1144_dn12)) * locals.var_t1__blk1145) + (assign33060_e27475 * locals.var_t1__blk1145_dn12))),)
    } else {
        (locals.var_b4soiqde, locals.var_b4soiqde_dn3, locals.var_b4soiqde_dn4, locals.var_b4soiqde_dn5, locals.var_b4soiqde_dn6, locals.var_b4soiqde_dn7, locals.var_b4soiqde_dn8, locals.var_b4soiqde_dn9, locals.var_b4soiqde_dn10, locals.var_b4soiqde_dn11, locals.var_b4soiqde_dn12,)
    }
};
        locals.var_b4soiqde = assign33060_e27480;
        locals.var_b4soiqde_dn3 = assign33060_e27480_d_n3;
        locals.var_b4soiqde_dn4 = assign33060_e27480_d_n4;
        locals.var_b4soiqde_dn5 = assign33060_e27480_d_n5;
        locals.var_b4soiqde_dn6 = assign33060_e27480_d_n6;
        locals.var_b4soiqde_dn7 = assign33060_e27480_d_n7;
        locals.var_b4soiqde_dn8 = assign33060_e27480_d_n8;
        locals.var_b4soiqde_dn9 = assign33060_e27480_d_n9;
        locals.var_b4soiqde_dn10 = assign33060_e27480_d_n10;
        locals.var_b4soiqde_dn11 = assign33060_e27480_d_n11;
        locals.var_b4soiqde_dn12 = assign33060_e27480_d_n12;

        let (assign33070_e27499, assign33070_e27499_d_n3, assign33070_e27499_d_n4, assign33070_e27499_d_n5, assign33070_e27499_d_n6, assign33070_e27499_d_n7, assign33070_e27499_d_n8, assign33070_e27499_d_n9, assign33070_e27499_d_n10, assign33070_e27499_d_n11, assign33070_e27499_d_n12,) = {
    if (((((locals.var_guard1755 != 0.0) && (locals.var_guard1763 != 0.0)) && (locals.var_guard1764 == 0.0)) && (locals.var_guard1765 == 0.0)) && (locals.var_guard1766 == 0.0)) {
        let assign33070_e27495: f64 = (locals.var_b4soicdmin * locals.var_t11);
        let assign33070_e27497: f64 = (assign33070_e27495 + locals.var_b4soidt4);
        (assign33070_e27497, (((locals.var_b4soicdmin_dn3 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn3)) + locals.var_b4soidt4_dn3), (((locals.var_b4soicdmin_dn4 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn4)) + locals.var_b4soidt4_dn4), (((locals.var_b4soicdmin_dn5 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn5)) + locals.var_b4soidt4_dn5), (((locals.var_b4soicdmin_dn6 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn6)) + locals.var_b4soidt4_dn6), (((locals.var_b4soicdmin_dn7 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn7)) + locals.var_b4soidt4_dn7), (((locals.var_b4soicdmin_dn8 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn8)) + locals.var_b4soidt4_dn8), (((locals.var_b4soicdmin_dn9 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn9)) + locals.var_b4soidt4_dn9), (((locals.var_b4soicdmin_dn10 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn10)) + locals.var_b4soidt4_dn10), (((locals.var_b4soicdmin_dn11 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn11)) + locals.var_b4soidt4_dn11), (((locals.var_b4soicdmin_dn12 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn12)) + locals.var_b4soidt4_dn12),)
    } else {
        (locals.var_b4soiqde, locals.var_b4soiqde_dn3, locals.var_b4soiqde_dn4, locals.var_b4soiqde_dn5, locals.var_b4soiqde_dn6, locals.var_b4soiqde_dn7, locals.var_b4soiqde_dn8, locals.var_b4soiqde_dn9, locals.var_b4soiqde_dn10, locals.var_b4soiqde_dn11, locals.var_b4soiqde_dn12,)
    }
};
        locals.var_b4soiqde = assign33070_e27499;
        locals.var_b4soiqde_dn3 = assign33070_e27499_d_n3;
        locals.var_b4soiqde_dn4 = assign33070_e27499_d_n4;
        locals.var_b4soiqde_dn5 = assign33070_e27499_d_n5;
        locals.var_b4soiqde_dn6 = assign33070_e27499_d_n6;
        locals.var_b4soiqde_dn7 = assign33070_e27499_d_n7;
        locals.var_b4soiqde_dn8 = assign33070_e27499_d_n8;
        locals.var_b4soiqde_dn9 = assign33070_e27499_d_n9;
        locals.var_b4soiqde_dn10 = assign33070_e27499_d_n10;
        locals.var_b4soiqde_dn11 = assign33070_e27499_d_n11;
        locals.var_b4soiqde_dn12 = assign33070_e27499_d_n12;

        let assign33080_e27502: f64 = if locals.var_t11 < locals.var_pparam_b4soivsdth { 1.0 } else { 0.0 };
        locals.var_guard1767 = assign33080_e27502;

        let (assign33090_e27515, assign33090_e27515_d_n3, assign33090_e27515_d_n4, assign33090_e27515_d_n5, assign33090_e27515_d_n6, assign33090_e27515_d_n7, assign33090_e27515_d_n8, assign33090_e27515_d_n9, assign33090_e27515_d_n10, assign33090_e27515_d_n11, assign33090_e27515_d_n12,) = {
    if (((locals.var_guard1755 != 0.0) && (locals.var_guard1763 == 0.0)) && (locals.var_guard1767 != 0.0)) {
        let assign33090_e27512: f64 = (locals.var_t11 - locals.var_pparam_b4soivsdth);
        let assign33090_e27513: f64 = (locals.var_b4soicdmin * assign33090_e27512);
        (assign33090_e27513, ((locals.var_b4soicdmin_dn3 * assign33090_e27512) + (locals.var_b4soicdmin * (locals.var_t11_dn3 - locals.var_pparam_b4soivsdth_dn3))), ((locals.var_b4soicdmin_dn4 * assign33090_e27512) + (locals.var_b4soicdmin * (locals.var_t11_dn4 - locals.var_pparam_b4soivsdth_dn4))), ((locals.var_b4soicdmin_dn5 * assign33090_e27512) + (locals.var_b4soicdmin * (locals.var_t11_dn5 - locals.var_pparam_b4soivsdth_dn5))), ((locals.var_b4soicdmin_dn6 * assign33090_e27512) + (locals.var_b4soicdmin * (locals.var_t11_dn6 - locals.var_pparam_b4soivsdth_dn6))), ((locals.var_b4soicdmin_dn7 * assign33090_e27512) + (locals.var_b4soicdmin * (locals.var_t11_dn7 - locals.var_pparam_b4soivsdth_dn7))), ((locals.var_b4soicdmin_dn8 * assign33090_e27512) + (locals.var_b4soicdmin * (locals.var_t11_dn8 - locals.var_pparam_b4soivsdth_dn8))), ((locals.var_b4soicdmin_dn9 * assign33090_e27512) + (locals.var_b4soicdmin * (locals.var_t11_dn9 - locals.var_pparam_b4soivsdth_dn9))), ((locals.var_b4soicdmin_dn10 * assign33090_e27512) + (locals.var_b4soicdmin * (locals.var_t11_dn10 - locals.var_pparam_b4soivsdth_dn10))), ((locals.var_b4soicdmin_dn11 * assign33090_e27512) + (locals.var_b4soicdmin * (locals.var_t11_dn11 - locals.var_pparam_b4soivsdth_dn11))), ((locals.var_b4soicdmin_dn12 * assign33090_e27512) + (locals.var_b4soicdmin * (locals.var_t11_dn12 - locals.var_pparam_b4soivsdth_dn12))),)
    } else {
        (locals.var_b4soiqde, locals.var_b4soiqde_dn3, locals.var_b4soiqde_dn4, locals.var_b4soiqde_dn5, locals.var_b4soiqde_dn6, locals.var_b4soiqde_dn7, locals.var_b4soiqde_dn8, locals.var_b4soiqde_dn9, locals.var_b4soiqde_dn10, locals.var_b4soiqde_dn11, locals.var_b4soiqde_dn12,)
    }
};
        locals.var_b4soiqde = assign33090_e27515;
        locals.var_b4soiqde_dn3 = assign33090_e27515_d_n3;
        locals.var_b4soiqde_dn4 = assign33090_e27515_d_n4;
        locals.var_b4soiqde_dn5 = assign33090_e27515_d_n5;
        locals.var_b4soiqde_dn6 = assign33090_e27515_d_n6;
        locals.var_b4soiqde_dn7 = assign33090_e27515_d_n7;
        locals.var_b4soiqde_dn8 = assign33090_e27515_d_n8;
        locals.var_b4soiqde_dn9 = assign33090_e27515_d_n9;
        locals.var_b4soiqde_dn10 = assign33090_e27515_d_n10;
        locals.var_b4soiqde_dn11 = assign33090_e27515_d_n11;
        locals.var_b4soiqde_dn12 = assign33090_e27515_d_n12;

        let assign33100_e27518: f64 = if locals.var_t11 < locals.var_pparam_b4soisdt1 { 1.0 } else { 0.0 };
        locals.var_guard1768 = assign33100_e27518;

        let (assign33110_e27532, assign33110_e27532_d_n3, assign33110_e27532_d_n4, assign33110_e27532_d_n5, assign33110_e27532_d_n6, assign33110_e27532_d_n7, assign33110_e27532_d_n8, assign33110_e27532_d_n9, assign33110_e27532_d_n10, assign33110_e27532_d_n11, assign33110_e27532_d_n12,) = {
    if ((((locals.var_guard1755 != 0.0) && (locals.var_guard1763 == 0.0)) && (locals.var_guard1767 == 0.0)) && (locals.var_guard1768 != 0.0)) {
        let assign33110_e27530: f64 = (locals.var_t11 - locals.var_pparam_b4soivsdth);
        (assign33110_e27530, (locals.var_t11_dn3 - locals.var_pparam_b4soivsdth_dn3), (locals.var_t11_dn4 - locals.var_pparam_b4soivsdth_dn4), (locals.var_t11_dn5 - locals.var_pparam_b4soivsdth_dn5), (locals.var_t11_dn6 - locals.var_pparam_b4soivsdth_dn6), (locals.var_t11_dn7 - locals.var_pparam_b4soivsdth_dn7), (locals.var_t11_dn8 - locals.var_pparam_b4soivsdth_dn8), (locals.var_t11_dn9 - locals.var_pparam_b4soivsdth_dn9), (locals.var_t11_dn10 - locals.var_pparam_b4soivsdth_dn10), (locals.var_t11_dn11 - locals.var_pparam_b4soivsdth_dn11), (locals.var_t11_dn12 - locals.var_pparam_b4soivsdth_dn12),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign33110_e27532;
        locals.var_t0__blk1144_dn3 = assign33110_e27532_d_n3;
        locals.var_t0__blk1144_dn4 = assign33110_e27532_d_n4;
        locals.var_t0__blk1144_dn5 = assign33110_e27532_d_n5;
        locals.var_t0__blk1144_dn6 = assign33110_e27532_d_n6;
        locals.var_t0__blk1144_dn7 = assign33110_e27532_d_n7;
        locals.var_t0__blk1144_dn8 = assign33110_e27532_d_n8;
        locals.var_t0__blk1144_dn9 = assign33110_e27532_d_n9;
        locals.var_t0__blk1144_dn10 = assign33110_e27532_d_n10;
        locals.var_t0__blk1144_dn11 = assign33110_e27532_d_n11;
        locals.var_t0__blk1144_dn12 = assign33110_e27532_d_n12;

        let (assign33120_e27546, assign33120_e27546_d_n3, assign33120_e27546_d_n4, assign33120_e27546_d_n5, assign33120_e27546_d_n6, assign33120_e27546_d_n7, assign33120_e27546_d_n8, assign33120_e27546_d_n9, assign33120_e27546_d_n10, assign33120_e27546_d_n11, assign33120_e27546_d_n12,) = {
    if ((((locals.var_guard1755 != 0.0) && (locals.var_guard1763 == 0.0)) && (locals.var_guard1767 == 0.0)) && (locals.var_guard1768 != 0.0)) {
        let assign33120_e27544: f64 = (locals.var_t0__blk1144 * locals.var_t0__blk1144);
        (assign33120_e27544, ((locals.var_t0__blk1144_dn3 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn3)), ((locals.var_t0__blk1144_dn4 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn4)), ((locals.var_t0__blk1144_dn5 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn5)), ((locals.var_t0__blk1144_dn6 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn6)), ((locals.var_t0__blk1144_dn7 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn7)), ((locals.var_t0__blk1144_dn8 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn8)), ((locals.var_t0__blk1144_dn9 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn9)), ((locals.var_t0__blk1144_dn10 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn10)), ((locals.var_t0__blk1144_dn11 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn11)), ((locals.var_t0__blk1144_dn12 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn12)),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign33120_e27546;
        locals.var_t1__blk1145_dn3 = assign33120_e27546_d_n3;
        locals.var_t1__blk1145_dn4 = assign33120_e27546_d_n4;
        locals.var_t1__blk1145_dn5 = assign33120_e27546_d_n5;
        locals.var_t1__blk1145_dn6 = assign33120_e27546_d_n6;
        locals.var_t1__blk1145_dn7 = assign33120_e27546_d_n7;
        locals.var_t1__blk1145_dn8 = assign33120_e27546_d_n8;
        locals.var_t1__blk1145_dn9 = assign33120_e27546_d_n9;
        locals.var_t1__blk1145_dn10 = assign33120_e27546_d_n10;
        locals.var_t1__blk1145_dn11 = assign33120_e27546_d_n11;
        locals.var_t1__blk1145_dn12 = assign33120_e27546_d_n12;

        let (assign33130_e27566, assign33130_e27566_d_n3, assign33130_e27566_d_n4, assign33130_e27566_d_n5, assign33130_e27566_d_n6, assign33130_e27566_d_n7, assign33130_e27566_d_n8, assign33130_e27566_d_n9, assign33130_e27566_d_n10, assign33130_e27566_d_n11, assign33130_e27566_d_n12,) = {
    if ((((locals.var_guard1755 != 0.0) && (locals.var_guard1763 == 0.0)) && (locals.var_guard1767 == 0.0)) && (locals.var_guard1768 != 0.0)) {
        let assign33130_e27560: f64 = (locals.var_pparam_b4soidt2 / 3.0);
        let assign33130_e27562: f64 = (assign33130_e27560 * locals.var_t1__blk1145);
        let assign33130_e27563: f64 = (locals.var_b4soicdmin - assign33130_e27562);
        let assign33130_e27564: f64 = (locals.var_t0__blk1144 * assign33130_e27563);
        (assign33130_e27564, ((locals.var_t0__blk1144_dn3 * assign33130_e27563) + (locals.var_t0__blk1144 * (locals.var_b4soicdmin_dn3 - (((locals.var_pparam_b4soidt2_dn3 / 3.0) * locals.var_t1__blk1145) + (assign33130_e27560 * locals.var_t1__blk1145_dn3))))), ((locals.var_t0__blk1144_dn4 * assign33130_e27563) + (locals.var_t0__blk1144 * (locals.var_b4soicdmin_dn4 - (((locals.var_pparam_b4soidt2_dn4 / 3.0) * locals.var_t1__blk1145) + (assign33130_e27560 * locals.var_t1__blk1145_dn4))))), ((locals.var_t0__blk1144_dn5 * assign33130_e27563) + (locals.var_t0__blk1144 * (locals.var_b4soicdmin_dn5 - (((locals.var_pparam_b4soidt2_dn5 / 3.0) * locals.var_t1__blk1145) + (assign33130_e27560 * locals.var_t1__blk1145_dn5))))), ((locals.var_t0__blk1144_dn6 * assign33130_e27563) + (locals.var_t0__blk1144 * (locals.var_b4soicdmin_dn6 - (((locals.var_pparam_b4soidt2_dn6 / 3.0) * locals.var_t1__blk1145) + (assign33130_e27560 * locals.var_t1__blk1145_dn6))))), ((locals.var_t0__blk1144_dn7 * assign33130_e27563) + (locals.var_t0__blk1144 * (locals.var_b4soicdmin_dn7 - (((locals.var_pparam_b4soidt2_dn7 / 3.0) * locals.var_t1__blk1145) + (assign33130_e27560 * locals.var_t1__blk1145_dn7))))), ((locals.var_t0__blk1144_dn8 * assign33130_e27563) + (locals.var_t0__blk1144 * (locals.var_b4soicdmin_dn8 - (((locals.var_pparam_b4soidt2_dn8 / 3.0) * locals.var_t1__blk1145) + (assign33130_e27560 * locals.var_t1__blk1145_dn8))))), ((locals.var_t0__blk1144_dn9 * assign33130_e27563) + (locals.var_t0__blk1144 * (locals.var_b4soicdmin_dn9 - (((locals.var_pparam_b4soidt2_dn9 / 3.0) * locals.var_t1__blk1145) + (assign33130_e27560 * locals.var_t1__blk1145_dn9))))), ((locals.var_t0__blk1144_dn10 * assign33130_e27563) + (locals.var_t0__blk1144 * (locals.var_b4soicdmin_dn10 - (((locals.var_pparam_b4soidt2_dn10 / 3.0) * locals.var_t1__blk1145) + (assign33130_e27560 * locals.var_t1__blk1145_dn10))))), ((locals.var_t0__blk1144_dn11 * assign33130_e27563) + (locals.var_t0__blk1144 * (locals.var_b4soicdmin_dn11 - (((locals.var_pparam_b4soidt2_dn11 / 3.0) * locals.var_t1__blk1145) + (assign33130_e27560 * locals.var_t1__blk1145_dn11))))), ((locals.var_t0__blk1144_dn12 * assign33130_e27563) + (locals.var_t0__blk1144 * (locals.var_b4soicdmin_dn12 - (((locals.var_pparam_b4soidt2_dn12 / 3.0) * locals.var_t1__blk1145) + (assign33130_e27560 * locals.var_t1__blk1145_dn12))))),)
    } else {
        (locals.var_b4soiqde, locals.var_b4soiqde_dn3, locals.var_b4soiqde_dn4, locals.var_b4soiqde_dn5, locals.var_b4soiqde_dn6, locals.var_b4soiqde_dn7, locals.var_b4soiqde_dn8, locals.var_b4soiqde_dn9, locals.var_b4soiqde_dn10, locals.var_b4soiqde_dn11, locals.var_b4soiqde_dn12,)
    }
};
        locals.var_b4soiqde = assign33130_e27566;
        locals.var_b4soiqde_dn3 = assign33130_e27566_d_n3;
        locals.var_b4soiqde_dn4 = assign33130_e27566_d_n4;
        locals.var_b4soiqde_dn5 = assign33130_e27566_d_n5;
        locals.var_b4soiqde_dn6 = assign33130_e27566_d_n6;
        locals.var_b4soiqde_dn7 = assign33130_e27566_d_n7;
        locals.var_b4soiqde_dn8 = assign33130_e27566_d_n8;
        locals.var_b4soiqde_dn9 = assign33130_e27566_d_n9;
        locals.var_b4soiqde_dn10 = assign33130_e27566_d_n10;
        locals.var_b4soiqde_dn11 = assign33130_e27566_d_n11;
        locals.var_b4soiqde_dn12 = assign33130_e27566_d_n12;

        let assign33140_e27569: f64 = if locals.var_t11 < locals.var_pparam_b4soivsdfb { 1.0 } else { 0.0 };
        locals.var_guard1769 = assign33140_e27569;

        let (assign33150_e27586, assign33150_e27586_d_n3, assign33150_e27586_d_n4, assign33150_e27586_d_n5, assign33150_e27586_d_n6, assign33150_e27586_d_n7, assign33150_e27586_d_n8, assign33150_e27586_d_n9, assign33150_e27586_d_n10, assign33150_e27586_d_n11, assign33150_e27586_d_n12,) = {
    if (((((locals.var_guard1755 != 0.0) && (locals.var_guard1763 == 0.0)) && (locals.var_guard1767 == 0.0)) && (locals.var_guard1768 == 0.0)) && (locals.var_guard1769 != 0.0)) {
        let assign33150_e27584: f64 = (locals.var_t11 - locals.var_pparam_b4soivsdfb);
        (assign33150_e27584, (locals.var_t11_dn3 - locals.var_pparam_b4soivsdfb_dn3), (locals.var_t11_dn4 - locals.var_pparam_b4soivsdfb_dn4), (locals.var_t11_dn5 - locals.var_pparam_b4soivsdfb_dn5), (locals.var_t11_dn6 - locals.var_pparam_b4soivsdfb_dn6), (locals.var_t11_dn7 - locals.var_pparam_b4soivsdfb_dn7), (locals.var_t11_dn8 - locals.var_pparam_b4soivsdfb_dn8), (locals.var_t11_dn9 - locals.var_pparam_b4soivsdfb_dn9), (locals.var_t11_dn10 - locals.var_pparam_b4soivsdfb_dn10), (locals.var_t11_dn11 - locals.var_pparam_b4soivsdfb_dn11), (locals.var_t11_dn12 - locals.var_pparam_b4soivsdfb_dn12),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign33150_e27586;
        locals.var_t0__blk1144_dn3 = assign33150_e27586_d_n3;
        locals.var_t0__blk1144_dn4 = assign33150_e27586_d_n4;
        locals.var_t0__blk1144_dn5 = assign33150_e27586_d_n5;
        locals.var_t0__blk1144_dn6 = assign33150_e27586_d_n6;
        locals.var_t0__blk1144_dn7 = assign33150_e27586_d_n7;
        locals.var_t0__blk1144_dn8 = assign33150_e27586_d_n8;
        locals.var_t0__blk1144_dn9 = assign33150_e27586_d_n9;
        locals.var_t0__blk1144_dn10 = assign33150_e27586_d_n10;
        locals.var_t0__blk1144_dn11 = assign33150_e27586_d_n11;
        locals.var_t0__blk1144_dn12 = assign33150_e27586_d_n12;

        let (assign33160_e27603, assign33160_e27603_d_n3, assign33160_e27603_d_n4, assign33160_e27603_d_n5, assign33160_e27603_d_n6, assign33160_e27603_d_n7, assign33160_e27603_d_n8, assign33160_e27603_d_n9, assign33160_e27603_d_n10, assign33160_e27603_d_n11, assign33160_e27603_d_n12,) = {
    if (((((locals.var_guard1755 != 0.0) && (locals.var_guard1763 == 0.0)) && (locals.var_guard1767 == 0.0)) && (locals.var_guard1768 == 0.0)) && (locals.var_guard1769 != 0.0)) {
        let assign33160_e27601: f64 = (locals.var_t0__blk1144 * locals.var_t0__blk1144);
        (assign33160_e27601, ((locals.var_t0__blk1144_dn3 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn3)), ((locals.var_t0__blk1144_dn4 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn4)), ((locals.var_t0__blk1144_dn5 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn5)), ((locals.var_t0__blk1144_dn6 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn6)), ((locals.var_t0__blk1144_dn7 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn7)), ((locals.var_t0__blk1144_dn8 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn8)), ((locals.var_t0__blk1144_dn9 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn9)), ((locals.var_t0__blk1144_dn10 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn10)), ((locals.var_t0__blk1144_dn11 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn11)), ((locals.var_t0__blk1144_dn12 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn12)),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign33160_e27603;
        locals.var_t1__blk1145_dn3 = assign33160_e27603_d_n3;
        locals.var_t1__blk1145_dn4 = assign33160_e27603_d_n4;
        locals.var_t1__blk1145_dn5 = assign33160_e27603_d_n5;
        locals.var_t1__blk1145_dn6 = assign33160_e27603_d_n6;
        locals.var_t1__blk1145_dn7 = assign33160_e27603_d_n7;
        locals.var_t1__blk1145_dn8 = assign33160_e27603_d_n8;
        locals.var_t1__blk1145_dn9 = assign33160_e27603_d_n9;
        locals.var_t1__blk1145_dn10 = assign33160_e27603_d_n10;
        locals.var_t1__blk1145_dn11 = assign33160_e27603_d_n11;
        locals.var_t1__blk1145_dn12 = assign33160_e27603_d_n12;

        let (assign33170_e27630, assign33170_e27630_d_n3, assign33170_e27630_d_n4, assign33170_e27630_d_n5, assign33170_e27630_d_n6, assign33170_e27630_d_n7, assign33170_e27630_d_n8, assign33170_e27630_d_n9, assign33170_e27630_d_n10, assign33170_e27630_d_n11, assign33170_e27630_d_n12,) = {
    if (((((locals.var_guard1755 != 0.0) && (locals.var_guard1763 == 0.0)) && (locals.var_guard1767 == 0.0)) && (locals.var_guard1768 == 0.0)) && (locals.var_guard1769 != 0.0)) {
        let assign33170_e27618: f64 = (locals.var_b4soicdbox * locals.var_t11);
        let assign33170_e27620: f64 = (assign33170_e27618 + locals.var_b4soidt4);
        let assign33170_e27623: f64 = (locals.var_pparam_b4soidt3 / 3.0);
        let assign33170_e27625: f64 = (assign33170_e27623 * locals.var_t0__blk1144);
        let assign33170_e27627: f64 = (assign33170_e27625 * locals.var_t1__blk1145);
        let assign33170_e27628: f64 = (assign33170_e27620 + assign33170_e27627);
        (assign33170_e27628, (((locals.var_b4soicdbox * locals.var_t11_dn3) + locals.var_b4soidt4_dn3) + (((((locals.var_pparam_b4soidt3_dn3 / 3.0) * locals.var_t0__blk1144) + (assign33170_e27623 * locals.var_t0__blk1144_dn3)) * locals.var_t1__blk1145) + (assign33170_e27625 * locals.var_t1__blk1145_dn3))), (((locals.var_b4soicdbox * locals.var_t11_dn4) + locals.var_b4soidt4_dn4) + (((((locals.var_pparam_b4soidt3_dn4 / 3.0) * locals.var_t0__blk1144) + (assign33170_e27623 * locals.var_t0__blk1144_dn4)) * locals.var_t1__blk1145) + (assign33170_e27625 * locals.var_t1__blk1145_dn4))), (((locals.var_b4soicdbox * locals.var_t11_dn5) + locals.var_b4soidt4_dn5) + (((((locals.var_pparam_b4soidt3_dn5 / 3.0) * locals.var_t0__blk1144) + (assign33170_e27623 * locals.var_t0__blk1144_dn5)) * locals.var_t1__blk1145) + (assign33170_e27625 * locals.var_t1__blk1145_dn5))), (((locals.var_b4soicdbox * locals.var_t11_dn6) + locals.var_b4soidt4_dn6) + (((((locals.var_pparam_b4soidt3_dn6 / 3.0) * locals.var_t0__blk1144) + (assign33170_e27623 * locals.var_t0__blk1144_dn6)) * locals.var_t1__blk1145) + (assign33170_e27625 * locals.var_t1__blk1145_dn6))), (((locals.var_b4soicdbox * locals.var_t11_dn7) + locals.var_b4soidt4_dn7) + (((((locals.var_pparam_b4soidt3_dn7 / 3.0) * locals.var_t0__blk1144) + (assign33170_e27623 * locals.var_t0__blk1144_dn7)) * locals.var_t1__blk1145) + (assign33170_e27625 * locals.var_t1__blk1145_dn7))), (((locals.var_b4soicdbox * locals.var_t11_dn8) + locals.var_b4soidt4_dn8) + (((((locals.var_pparam_b4soidt3_dn8 / 3.0) * locals.var_t0__blk1144) + (assign33170_e27623 * locals.var_t0__blk1144_dn8)) * locals.var_t1__blk1145) + (assign33170_e27625 * locals.var_t1__blk1145_dn8))), (((locals.var_b4soicdbox * locals.var_t11_dn9) + locals.var_b4soidt4_dn9) + (((((locals.var_pparam_b4soidt3_dn9 / 3.0) * locals.var_t0__blk1144) + (assign33170_e27623 * locals.var_t0__blk1144_dn9)) * locals.var_t1__blk1145) + (assign33170_e27625 * locals.var_t1__blk1145_dn9))), (((locals.var_b4soicdbox * locals.var_t11_dn10) + locals.var_b4soidt4_dn10) + (((((locals.var_pparam_b4soidt3_dn10 / 3.0) * locals.var_t0__blk1144) + (assign33170_e27623 * locals.var_t0__blk1144_dn10)) * locals.var_t1__blk1145) + (assign33170_e27625 * locals.var_t1__blk1145_dn10))), (((locals.var_b4soicdbox * locals.var_t11_dn11) + locals.var_b4soidt4_dn11) + (((((locals.var_pparam_b4soidt3_dn11 / 3.0) * locals.var_t0__blk1144) + (assign33170_e27623 * locals.var_t0__blk1144_dn11)) * locals.var_t1__blk1145) + (assign33170_e27625 * locals.var_t1__blk1145_dn11))), (((locals.var_b4soicdbox * locals.var_t11_dn12) + locals.var_b4soidt4_dn12) + (((((locals.var_pparam_b4soidt3_dn12 / 3.0) * locals.var_t0__blk1144) + (assign33170_e27623 * locals.var_t0__blk1144_dn12)) * locals.var_t1__blk1145) + (assign33170_e27625 * locals.var_t1__blk1145_dn12))),)
    } else {
        (locals.var_b4soiqde, locals.var_b4soiqde_dn3, locals.var_b4soiqde_dn4, locals.var_b4soiqde_dn5, locals.var_b4soiqde_dn6, locals.var_b4soiqde_dn7, locals.var_b4soiqde_dn8, locals.var_b4soiqde_dn9, locals.var_b4soiqde_dn10, locals.var_b4soiqde_dn11, locals.var_b4soiqde_dn12,)
    }
};
        locals.var_b4soiqde = assign33170_e27630;
        locals.var_b4soiqde_dn3 = assign33170_e27630_d_n3;
        locals.var_b4soiqde_dn4 = assign33170_e27630_d_n4;
        locals.var_b4soiqde_dn5 = assign33170_e27630_d_n5;
        locals.var_b4soiqde_dn6 = assign33170_e27630_d_n6;
        locals.var_b4soiqde_dn7 = assign33170_e27630_d_n7;
        locals.var_b4soiqde_dn8 = assign33170_e27630_d_n8;
        locals.var_b4soiqde_dn9 = assign33170_e27630_d_n9;
        locals.var_b4soiqde_dn10 = assign33170_e27630_d_n10;
        locals.var_b4soiqde_dn11 = assign33170_e27630_d_n11;
        locals.var_b4soiqde_dn12 = assign33170_e27630_d_n12;

        let (assign33180_e27650, assign33180_e27650_d_n3, assign33180_e27650_d_n4, assign33180_e27650_d_n5, assign33180_e27650_d_n6, assign33180_e27650_d_n7, assign33180_e27650_d_n8, assign33180_e27650_d_n9, assign33180_e27650_d_n10, assign33180_e27650_d_n11, assign33180_e27650_d_n12,) = {
    if (((((locals.var_guard1755 != 0.0) && (locals.var_guard1763 == 0.0)) && (locals.var_guard1767 == 0.0)) && (locals.var_guard1768 == 0.0)) && (locals.var_guard1769 == 0.0)) {
        let assign33180_e27646: f64 = (locals.var_b4soicdbox * locals.var_t11);
        let assign33180_e27648: f64 = (assign33180_e27646 + locals.var_b4soidt4);
        (assign33180_e27648, ((locals.var_b4soicdbox * locals.var_t11_dn3) + locals.var_b4soidt4_dn3), ((locals.var_b4soicdbox * locals.var_t11_dn4) + locals.var_b4soidt4_dn4), ((locals.var_b4soicdbox * locals.var_t11_dn5) + locals.var_b4soidt4_dn5), ((locals.var_b4soicdbox * locals.var_t11_dn6) + locals.var_b4soidt4_dn6), ((locals.var_b4soicdbox * locals.var_t11_dn7) + locals.var_b4soidt4_dn7), ((locals.var_b4soicdbox * locals.var_t11_dn8) + locals.var_b4soidt4_dn8), ((locals.var_b4soicdbox * locals.var_t11_dn9) + locals.var_b4soidt4_dn9), ((locals.var_b4soicdbox * locals.var_t11_dn10) + locals.var_b4soidt4_dn10), ((locals.var_b4soicdbox * locals.var_t11_dn11) + locals.var_b4soidt4_dn11), ((locals.var_b4soicdbox * locals.var_t11_dn12) + locals.var_b4soidt4_dn12),)
    } else {
        (locals.var_b4soiqde, locals.var_b4soiqde_dn3, locals.var_b4soiqde_dn4, locals.var_b4soiqde_dn5, locals.var_b4soiqde_dn6, locals.var_b4soiqde_dn7, locals.var_b4soiqde_dn8, locals.var_b4soiqde_dn9, locals.var_b4soiqde_dn10, locals.var_b4soiqde_dn11, locals.var_b4soiqde_dn12,)
    }
};
        locals.var_b4soiqde = assign33180_e27650;
        locals.var_b4soiqde_dn3 = assign33180_e27650_d_n3;
        locals.var_b4soiqde_dn4 = assign33180_e27650_d_n4;
        locals.var_b4soiqde_dn5 = assign33180_e27650_d_n5;
        locals.var_b4soiqde_dn6 = assign33180_e27650_d_n6;
        locals.var_b4soiqde_dn7 = assign33180_e27650_d_n7;
        locals.var_b4soiqde_dn8 = assign33180_e27650_d_n8;
        locals.var_b4soiqde_dn9 = assign33180_e27650_d_n9;
        locals.var_b4soiqde_dn10 = assign33180_e27650_d_n10;
        locals.var_b4soiqde_dn11 = assign33180_e27650_d_n11;
        locals.var_b4soiqde_dn12 = assign33180_e27650_d_n12;

        let (assign33190_e27657, assign33190_e27657_d_n3, assign33190_e27657_d_n4, assign33190_e27657_d_n5, assign33190_e27657_d_n6, assign33190_e27657_d_n7, assign33190_e27657_d_n8, assign33190_e27657_d_n9, assign33190_e27657_d_n10, assign33190_e27657_d_n11, assign33190_e27657_d_n12,) = {
    if (locals.var_guard1755 == 0.0) {
        let assign33190_e27655: f64 = (locals.var_b4soicsbox * locals.var_t10__blk1154);
        (assign33190_e27655, (locals.var_b4soicsbox * locals.var_t10__blk1154_dn3), (locals.var_b4soicsbox * locals.var_t10__blk1154_dn4), (locals.var_b4soicsbox * locals.var_t10__blk1154_dn5), (locals.var_b4soicsbox * locals.var_t10__blk1154_dn6), (locals.var_b4soicsbox * locals.var_t10__blk1154_dn7), (locals.var_b4soicsbox * locals.var_t10__blk1154_dn8), (locals.var_b4soicsbox * locals.var_t10__blk1154_dn9), (locals.var_b4soicsbox * locals.var_t10__blk1154_dn10), (locals.var_b4soicsbox * locals.var_t10__blk1154_dn11), (locals.var_b4soicsbox * locals.var_t10__blk1154_dn12),)
    } else {
        (locals.var_b4soiqse, locals.var_b4soiqse_dn3, locals.var_b4soiqse_dn4, locals.var_b4soiqse_dn5, locals.var_b4soiqse_dn6, locals.var_b4soiqse_dn7, locals.var_b4soiqse_dn8, locals.var_b4soiqse_dn9, locals.var_b4soiqse_dn10, locals.var_b4soiqse_dn11, locals.var_b4soiqse_dn12,)
    }
};
        locals.var_b4soiqse = assign33190_e27657;
        locals.var_b4soiqse_dn3 = assign33190_e27657_d_n3;
        locals.var_b4soiqse_dn4 = assign33190_e27657_d_n4;
        locals.var_b4soiqse_dn5 = assign33190_e27657_d_n5;
        locals.var_b4soiqse_dn6 = assign33190_e27657_d_n6;
        locals.var_b4soiqse_dn7 = assign33190_e27657_d_n7;
        locals.var_b4soiqse_dn8 = assign33190_e27657_d_n8;
        locals.var_b4soiqse_dn9 = assign33190_e27657_d_n9;
        locals.var_b4soiqse_dn10 = assign33190_e27657_d_n10;
        locals.var_b4soiqse_dn11 = assign33190_e27657_d_n11;
        locals.var_b4soiqse_dn12 = assign33190_e27657_d_n12;

        let (assign33200_e27664, assign33200_e27664_d_n3, assign33200_e27664_d_n4, assign33200_e27664_d_n5, assign33200_e27664_d_n6, assign33200_e27664_d_n7, assign33200_e27664_d_n8, assign33200_e27664_d_n9, assign33200_e27664_d_n10, assign33200_e27664_d_n11, assign33200_e27664_d_n12,) = {
    if (locals.var_guard1755 == 0.0) {
        let assign33200_e27662: f64 = (locals.var_b4soicdbox * locals.var_t11);
        (assign33200_e27662, (locals.var_b4soicdbox * locals.var_t11_dn3), (locals.var_b4soicdbox * locals.var_t11_dn4), (locals.var_b4soicdbox * locals.var_t11_dn5), (locals.var_b4soicdbox * locals.var_t11_dn6), (locals.var_b4soicdbox * locals.var_t11_dn7), (locals.var_b4soicdbox * locals.var_t11_dn8), (locals.var_b4soicdbox * locals.var_t11_dn9), (locals.var_b4soicdbox * locals.var_t11_dn10), (locals.var_b4soicdbox * locals.var_t11_dn11), (locals.var_b4soicdbox * locals.var_t11_dn12),)
    } else {
        (locals.var_b4soiqde, locals.var_b4soiqde_dn3, locals.var_b4soiqde_dn4, locals.var_b4soiqde_dn5, locals.var_b4soiqde_dn6, locals.var_b4soiqde_dn7, locals.var_b4soiqde_dn8, locals.var_b4soiqde_dn9, locals.var_b4soiqde_dn10, locals.var_b4soiqde_dn11, locals.var_b4soiqde_dn12,)
    }
};
        locals.var_b4soiqde = assign33200_e27664;
        locals.var_b4soiqde_dn3 = assign33200_e27664_d_n3;
        locals.var_b4soiqde_dn4 = assign33200_e27664_d_n4;
        locals.var_b4soiqde_dn5 = assign33200_e27664_d_n5;
        locals.var_b4soiqde_dn6 = assign33200_e27664_d_n6;
        locals.var_b4soiqde_dn7 = assign33200_e27664_d_n7;
        locals.var_b4soiqde_dn8 = assign33200_e27664_d_n8;
        locals.var_b4soiqde_dn9 = assign33200_e27664_d_n9;
        locals.var_b4soiqde_dn10 = assign33200_e27664_d_n10;
        locals.var_b4soiqde_dn11 = assign33200_e27664_d_n11;
        locals.var_b4soiqde_dn12 = assign33200_e27664_d_n12;

        let assign33210_e27668: f64 = (locals.var_b4soicsesw * locals.var_t10__blk1154);
        let assign33210_e27669: f64 = (locals.var_b4soiqse + assign33210_e27668);
        locals.var_b4soiqse = assign33210_e27669;
        locals.var_b4soiqse_dn3 = (locals.var_b4soiqse_dn3 + ((locals.var_b4soicsesw_dn3 * locals.var_t10__blk1154) + (locals.var_b4soicsesw * locals.var_t10__blk1154_dn3)));
        locals.var_b4soiqse_dn4 = (locals.var_b4soiqse_dn4 + ((locals.var_b4soicsesw_dn4 * locals.var_t10__blk1154) + (locals.var_b4soicsesw * locals.var_t10__blk1154_dn4)));
        locals.var_b4soiqse_dn5 = (locals.var_b4soiqse_dn5 + ((locals.var_b4soicsesw_dn5 * locals.var_t10__blk1154) + (locals.var_b4soicsesw * locals.var_t10__blk1154_dn5)));
        locals.var_b4soiqse_dn6 = (locals.var_b4soiqse_dn6 + ((locals.var_b4soicsesw_dn6 * locals.var_t10__blk1154) + (locals.var_b4soicsesw * locals.var_t10__blk1154_dn6)));
        locals.var_b4soiqse_dn7 = (locals.var_b4soiqse_dn7 + ((locals.var_b4soicsesw_dn7 * locals.var_t10__blk1154) + (locals.var_b4soicsesw * locals.var_t10__blk1154_dn7)));
        locals.var_b4soiqse_dn8 = (locals.var_b4soiqse_dn8 + ((locals.var_b4soicsesw_dn8 * locals.var_t10__blk1154) + (locals.var_b4soicsesw * locals.var_t10__blk1154_dn8)));
        locals.var_b4soiqse_dn9 = (locals.var_b4soiqse_dn9 + ((locals.var_b4soicsesw_dn9 * locals.var_t10__blk1154) + (locals.var_b4soicsesw * locals.var_t10__blk1154_dn9)));
        locals.var_b4soiqse_dn10 = (locals.var_b4soiqse_dn10 + ((locals.var_b4soicsesw_dn10 * locals.var_t10__blk1154) + (locals.var_b4soicsesw * locals.var_t10__blk1154_dn10)));
        locals.var_b4soiqse_dn11 = (locals.var_b4soiqse_dn11 + ((locals.var_b4soicsesw_dn11 * locals.var_t10__blk1154) + (locals.var_b4soicsesw * locals.var_t10__blk1154_dn11)));
        locals.var_b4soiqse_dn12 = (locals.var_b4soiqse_dn12 + ((locals.var_b4soicsesw_dn12 * locals.var_t10__blk1154) + (locals.var_b4soicsesw * locals.var_t10__blk1154_dn12)));

        let assign33220_e27673: f64 = (locals.var_b4soicdesw * locals.var_t11);
        let assign33220_e27674: f64 = (locals.var_b4soiqde + assign33220_e27673);
        locals.var_b4soiqde = assign33220_e27674;
        locals.var_b4soiqde_dn3 = (locals.var_b4soiqde_dn3 + ((locals.var_b4soicdesw_dn3 * locals.var_t11) + (locals.var_b4soicdesw * locals.var_t11_dn3)));
        locals.var_b4soiqde_dn4 = (locals.var_b4soiqde_dn4 + ((locals.var_b4soicdesw_dn4 * locals.var_t11) + (locals.var_b4soicdesw * locals.var_t11_dn4)));
        locals.var_b4soiqde_dn5 = (locals.var_b4soiqde_dn5 + ((locals.var_b4soicdesw_dn5 * locals.var_t11) + (locals.var_b4soicdesw * locals.var_t11_dn5)));
        locals.var_b4soiqde_dn6 = (locals.var_b4soiqde_dn6 + ((locals.var_b4soicdesw_dn6 * locals.var_t11) + (locals.var_b4soicdesw * locals.var_t11_dn6)));
        locals.var_b4soiqde_dn7 = (locals.var_b4soiqde_dn7 + ((locals.var_b4soicdesw_dn7 * locals.var_t11) + (locals.var_b4soicdesw * locals.var_t11_dn7)));
        locals.var_b4soiqde_dn8 = (locals.var_b4soiqde_dn8 + ((locals.var_b4soicdesw_dn8 * locals.var_t11) + (locals.var_b4soicdesw * locals.var_t11_dn8)));
        locals.var_b4soiqde_dn9 = (locals.var_b4soiqde_dn9 + ((locals.var_b4soicdesw_dn9 * locals.var_t11) + (locals.var_b4soicdesw * locals.var_t11_dn9)));
        locals.var_b4soiqde_dn10 = (locals.var_b4soiqde_dn10 + ((locals.var_b4soicdesw_dn10 * locals.var_t11) + (locals.var_b4soicdesw * locals.var_t11_dn10)));
        locals.var_b4soiqde_dn11 = (locals.var_b4soiqde_dn11 + ((locals.var_b4soicdesw_dn11 * locals.var_t11) + (locals.var_b4soicdesw * locals.var_t11_dn11)));
        locals.var_b4soiqde_dn12 = (locals.var_b4soiqde_dn12 + ((locals.var_b4soicdesw_dn12 * locals.var_t11) + (locals.var_b4soicdesw * locals.var_t11_dn12)));

        let assign33230_e27677: f64 = if locals.var_b4soirgatemod == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1770 = assign33230_e27677;

        let (assign33240_e27683, assign33240_e27683_d_n3, assign33240_e27683_d_n4, assign33240_e27683_d_n5, assign33240_e27683_d_n6, assign33240_e27683_d_n7, assign33240_e27683_d_n8, assign33240_e27683_d_n9, assign33240_e27683_d_n10, assign33240_e27683_d_n11, assign33240_e27683_d_n12,) = {
    if (locals.var_guard1770 != 0.0) {
        let assign33240_e27681: f64 = (locals.var_vgmd + 0.02);
        (assign33240_e27681, 0.0, 0.0, 0.0, 0.0, locals.var_vgmd_dn7, locals.var_vgmd_dn8, 0.0, locals.var_vgmd_dn10, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign33240_e27683;
        locals.var_t0__blk1144_dn3 = assign33240_e27683_d_n3;
        locals.var_t0__blk1144_dn4 = assign33240_e27683_d_n4;
        locals.var_t0__blk1144_dn5 = assign33240_e27683_d_n5;
        locals.var_t0__blk1144_dn6 = assign33240_e27683_d_n6;
        locals.var_t0__blk1144_dn7 = assign33240_e27683_d_n7;
        locals.var_t0__blk1144_dn8 = assign33240_e27683_d_n8;
        locals.var_t0__blk1144_dn9 = assign33240_e27683_d_n9;
        locals.var_t0__blk1144_dn10 = assign33240_e27683_d_n10;
        locals.var_t0__blk1144_dn11 = assign33240_e27683_d_n11;
        locals.var_t0__blk1144_dn12 = assign33240_e27683_d_n12;

        let (assign33250_e27690, assign33250_e27690_d_n3, assign33250_e27690_d_n4, assign33250_e27690_d_n5, assign33250_e27690_d_n6, assign33250_e27690_d_n7, assign33250_e27690_d_n8, assign33250_e27690_d_n9, assign33250_e27690_d_n10, assign33250_e27690_d_n11, assign33250_e27690_d_n12,) = {
    if (locals.var_guard1770 == 0.0) {
        let assign33250_e27688: f64 = (locals.var_vgd + 0.02);
        (assign33250_e27688, 0.0, 0.0, 0.0, 0.0, locals.var_vgd_dn7, locals.var_vgd_dn8, locals.var_vgd_dn9, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign33250_e27690;
        locals.var_t0__blk1144_dn3 = assign33250_e27690_d_n3;
        locals.var_t0__blk1144_dn4 = assign33250_e27690_d_n4;
        locals.var_t0__blk1144_dn5 = assign33250_e27690_d_n5;
        locals.var_t0__blk1144_dn6 = assign33250_e27690_d_n6;
        locals.var_t0__blk1144_dn7 = assign33250_e27690_d_n7;
        locals.var_t0__blk1144_dn8 = assign33250_e27690_d_n8;
        locals.var_t0__blk1144_dn9 = assign33250_e27690_d_n9;
        locals.var_t0__blk1144_dn10 = assign33250_e27690_d_n10;
        locals.var_t0__blk1144_dn11 = assign33250_e27690_d_n11;
        locals.var_t0__blk1144_dn12 = assign33250_e27690_d_n12;

        let assign33260_e27693: f64 = (locals.var_t0__blk1144 * locals.var_t0__blk1144);
        let assign33260_e27696: f64 = (4.0 * 0.02);
        let assign33260_e27697: f64 = (assign33260_e27693 + assign33260_e27696);
        let assign33260_e27698: f64 = (assign33260_e27697).sqrt();
        locals.var_t1__blk1145 = assign33260_e27698;
        locals.var_t1__blk1145_dn3 = (((locals.var_t0__blk1144_dn3 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn3)) / (2.0 * assign33260_e27698));
        locals.var_t1__blk1145_dn4 = (((locals.var_t0__blk1144_dn4 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn4)) / (2.0 * assign33260_e27698));
        locals.var_t1__blk1145_dn5 = (((locals.var_t0__blk1144_dn5 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn5)) / (2.0 * assign33260_e27698));
        locals.var_t1__blk1145_dn6 = (((locals.var_t0__blk1144_dn6 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn6)) / (2.0 * assign33260_e27698));
        locals.var_t1__blk1145_dn7 = (((locals.var_t0__blk1144_dn7 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn7)) / (2.0 * assign33260_e27698));
        locals.var_t1__blk1145_dn8 = (((locals.var_t0__blk1144_dn8 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn8)) / (2.0 * assign33260_e27698));
        locals.var_t1__blk1145_dn9 = (((locals.var_t0__blk1144_dn9 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn9)) / (2.0 * assign33260_e27698));
        locals.var_t1__blk1145_dn10 = (((locals.var_t0__blk1144_dn10 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn10)) / (2.0 * assign33260_e27698));
        locals.var_t1__blk1145_dn11 = (((locals.var_t0__blk1144_dn11 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn11)) / (2.0 * assign33260_e27698));
        locals.var_t1__blk1145_dn12 = (((locals.var_t0__blk1144_dn12 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn12)) / (2.0 * assign33260_e27698));

    }

    pub(super) fn stamp_transient_block_92(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign33270_e27702: f64 = (locals.var_t0__blk1144 - locals.var_t1__blk1145);
        let assign33270_e27703: f64 = (0.5 * assign33270_e27702);
        locals.var_t2__blk1146 = assign33270_e27703;
        locals.var_t2__blk1146_dn3 = (0.5 * (locals.var_t0__blk1144_dn3 - locals.var_t1__blk1145_dn3));
        locals.var_t2__blk1146_dn4 = (0.5 * (locals.var_t0__blk1144_dn4 - locals.var_t1__blk1145_dn4));
        locals.var_t2__blk1146_dn5 = (0.5 * (locals.var_t0__blk1144_dn5 - locals.var_t1__blk1145_dn5));
        locals.var_t2__blk1146_dn6 = (0.5 * (locals.var_t0__blk1144_dn6 - locals.var_t1__blk1145_dn6));
        locals.var_t2__blk1146_dn7 = (0.5 * (locals.var_t0__blk1144_dn7 - locals.var_t1__blk1145_dn7));
        locals.var_t2__blk1146_dn8 = (0.5 * (locals.var_t0__blk1144_dn8 - locals.var_t1__blk1145_dn8));
        locals.var_t2__blk1146_dn9 = (0.5 * (locals.var_t0__blk1144_dn9 - locals.var_t1__blk1145_dn9));
        locals.var_t2__blk1146_dn10 = (0.5 * (locals.var_t0__blk1144_dn10 - locals.var_t1__blk1145_dn10));
        locals.var_t2__blk1146_dn11 = (0.5 * (locals.var_t0__blk1144_dn11 - locals.var_t1__blk1145_dn11));
        locals.var_t2__blk1146_dn12 = (0.5 * (locals.var_t0__blk1144_dn12 - locals.var_t1__blk1145_dn12));

        let assign33280_e27706: f64 = (locals.var_pparam_b4soiwdiodcv * locals.var_pparam_b4soicgdl);
        locals.var_t3__blk1147 = assign33280_e27706;
        locals.var_t3__blk1147_dn3 = ((locals.var_pparam_b4soiwdiodcv_dn3 * locals.var_pparam_b4soicgdl) + (locals.var_pparam_b4soiwdiodcv * locals.var_pparam_b4soicgdl_dn3));
        locals.var_t3__blk1147_dn4 = ((locals.var_pparam_b4soiwdiodcv_dn4 * locals.var_pparam_b4soicgdl) + (locals.var_pparam_b4soiwdiodcv * locals.var_pparam_b4soicgdl_dn4));
        locals.var_t3__blk1147_dn5 = ((locals.var_pparam_b4soiwdiodcv_dn5 * locals.var_pparam_b4soicgdl) + (locals.var_pparam_b4soiwdiodcv * locals.var_pparam_b4soicgdl_dn5));
        locals.var_t3__blk1147_dn6 = ((locals.var_pparam_b4soiwdiodcv_dn6 * locals.var_pparam_b4soicgdl) + (locals.var_pparam_b4soiwdiodcv * locals.var_pparam_b4soicgdl_dn6));
        locals.var_t3__blk1147_dn7 = ((locals.var_pparam_b4soiwdiodcv_dn7 * locals.var_pparam_b4soicgdl) + (locals.var_pparam_b4soiwdiodcv * locals.var_pparam_b4soicgdl_dn7));
        locals.var_t3__blk1147_dn8 = ((locals.var_pparam_b4soiwdiodcv_dn8 * locals.var_pparam_b4soicgdl) + (locals.var_pparam_b4soiwdiodcv * locals.var_pparam_b4soicgdl_dn8));
        locals.var_t3__blk1147_dn9 = ((locals.var_pparam_b4soiwdiodcv_dn9 * locals.var_pparam_b4soicgdl) + (locals.var_pparam_b4soiwdiodcv * locals.var_pparam_b4soicgdl_dn9));
        locals.var_t3__blk1147_dn10 = ((locals.var_pparam_b4soiwdiodcv_dn10 * locals.var_pparam_b4soicgdl) + (locals.var_pparam_b4soiwdiodcv * locals.var_pparam_b4soicgdl_dn10));
        locals.var_t3__blk1147_dn11 = ((locals.var_pparam_b4soiwdiodcv_dn11 * locals.var_pparam_b4soicgdl) + (locals.var_pparam_b4soiwdiodcv * locals.var_pparam_b4soicgdl_dn11));
        locals.var_t3__blk1147_dn12 = ((locals.var_pparam_b4soiwdiodcv_dn12 * locals.var_pparam_b4soicgdl) + (locals.var_pparam_b4soiwdiodcv * locals.var_pparam_b4soicgdl_dn12));

        let assign33290_e27710: f64 = (4.0 * locals.var_t2__blk1146);
        let assign33290_e27712: f64 = (assign33290_e27710 / locals.var_pparam_b4soickappa);
        let assign33290_e27713: f64 = (1.0 - assign33290_e27712);
        let assign33290_e27714: f64 = (assign33290_e27713).sqrt();
        locals.var_t4__blk1148 = assign33290_e27714;
        locals.var_t4__blk1148_dn3 = ((-((((4.0 * locals.var_t2__blk1146_dn3) * locals.var_pparam_b4soickappa) - (assign33290_e27710 * locals.var_pparam_b4soickappa_dn3)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign33290_e27714));
        locals.var_t4__blk1148_dn4 = ((-((((4.0 * locals.var_t2__blk1146_dn4) * locals.var_pparam_b4soickappa) - (assign33290_e27710 * locals.var_pparam_b4soickappa_dn4)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign33290_e27714));
        locals.var_t4__blk1148_dn5 = ((-((((4.0 * locals.var_t2__blk1146_dn5) * locals.var_pparam_b4soickappa) - (assign33290_e27710 * locals.var_pparam_b4soickappa_dn5)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign33290_e27714));
        locals.var_t4__blk1148_dn6 = ((-((((4.0 * locals.var_t2__blk1146_dn6) * locals.var_pparam_b4soickappa) - (assign33290_e27710 * locals.var_pparam_b4soickappa_dn6)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign33290_e27714));
        locals.var_t4__blk1148_dn7 = ((-((((4.0 * locals.var_t2__blk1146_dn7) * locals.var_pparam_b4soickappa) - (assign33290_e27710 * locals.var_pparam_b4soickappa_dn7)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign33290_e27714));
        locals.var_t4__blk1148_dn8 = ((-((((4.0 * locals.var_t2__blk1146_dn8) * locals.var_pparam_b4soickappa) - (assign33290_e27710 * locals.var_pparam_b4soickappa_dn8)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign33290_e27714));
        locals.var_t4__blk1148_dn9 = ((-((((4.0 * locals.var_t2__blk1146_dn9) * locals.var_pparam_b4soickappa) - (assign33290_e27710 * locals.var_pparam_b4soickappa_dn9)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign33290_e27714));
        locals.var_t4__blk1148_dn10 = ((-((((4.0 * locals.var_t2__blk1146_dn10) * locals.var_pparam_b4soickappa) - (assign33290_e27710 * locals.var_pparam_b4soickappa_dn10)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign33290_e27714));
        locals.var_t4__blk1148_dn11 = ((-((((4.0 * locals.var_t2__blk1146_dn11) * locals.var_pparam_b4soickappa) - (assign33290_e27710 * locals.var_pparam_b4soickappa_dn11)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign33290_e27714));
        locals.var_t4__blk1148_dn12 = ((-((((4.0 * locals.var_t2__blk1146_dn12) * locals.var_pparam_b4soickappa) - (assign33290_e27710 * locals.var_pparam_b4soickappa_dn12)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign33290_e27714));

        let assign33300_e27717: f64 = if locals.var_b4soirgatemod == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1771 = assign33300_e27717;

        let (assign33310_e27737, assign33310_e27737_d_n3, assign33310_e27737_d_n4, assign33310_e27737_d_n5, assign33310_e27737_d_n6, assign33310_e27737_d_n7, assign33310_e27737_d_n8, assign33310_e27737_d_n9, assign33310_e27737_d_n10, assign33310_e27737_d_n11, assign33310_e27737_d_n12,) = {
    if (locals.var_guard1771 != 0.0) {
        let assign33310_e27721: f64 = (locals.var_pparam_b4soicgdo + locals.var_t3__blk1147);
        let assign33310_e27723: f64 = (assign33310_e27721 * locals.var_vgmd);
        let assign33310_e27728: f64 = (0.5 * locals.var_pparam_b4soickappa);
        let assign33310_e27731: f64 = (locals.var_t4__blk1148 - 1.0);
        let assign33310_e27732: f64 = (assign33310_e27728 * assign33310_e27731);
        let assign33310_e27733: f64 = (locals.var_t2__blk1146 + assign33310_e27732);
        let assign33310_e27734: f64 = (locals.var_t3__blk1147 * assign33310_e27733);
        let assign33310_e27735: f64 = (assign33310_e27723 - assign33310_e27734);
        (assign33310_e27735, (((locals.var_pparam_b4soicgdo_dn3 + locals.var_t3__blk1147_dn3) * locals.var_vgmd) - ((locals.var_t3__blk1147_dn3 * assign33310_e27733) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn3 + (((0.5 * locals.var_pparam_b4soickappa_dn3) * assign33310_e27731) + (assign33310_e27728 * locals.var_t4__blk1148_dn3)))))), (((locals.var_pparam_b4soicgdo_dn4 + locals.var_t3__blk1147_dn4) * locals.var_vgmd) - ((locals.var_t3__blk1147_dn4 * assign33310_e27733) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn4 + (((0.5 * locals.var_pparam_b4soickappa_dn4) * assign33310_e27731) + (assign33310_e27728 * locals.var_t4__blk1148_dn4)))))), (((locals.var_pparam_b4soicgdo_dn5 + locals.var_t3__blk1147_dn5) * locals.var_vgmd) - ((locals.var_t3__blk1147_dn5 * assign33310_e27733) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn5 + (((0.5 * locals.var_pparam_b4soickappa_dn5) * assign33310_e27731) + (assign33310_e27728 * locals.var_t4__blk1148_dn5)))))), (((locals.var_pparam_b4soicgdo_dn6 + locals.var_t3__blk1147_dn6) * locals.var_vgmd) - ((locals.var_t3__blk1147_dn6 * assign33310_e27733) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn6 + (((0.5 * locals.var_pparam_b4soickappa_dn6) * assign33310_e27731) + (assign33310_e27728 * locals.var_t4__blk1148_dn6)))))), ((((locals.var_pparam_b4soicgdo_dn7 + locals.var_t3__blk1147_dn7) * locals.var_vgmd) + (assign33310_e27721 * locals.var_vgmd_dn7)) - ((locals.var_t3__blk1147_dn7 * assign33310_e27733) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn7 + (((0.5 * locals.var_pparam_b4soickappa_dn7) * assign33310_e27731) + (assign33310_e27728 * locals.var_t4__blk1148_dn7)))))), ((((locals.var_pparam_b4soicgdo_dn8 + locals.var_t3__blk1147_dn8) * locals.var_vgmd) + (assign33310_e27721 * locals.var_vgmd_dn8)) - ((locals.var_t3__blk1147_dn8 * assign33310_e27733) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn8 + (((0.5 * locals.var_pparam_b4soickappa_dn8) * assign33310_e27731) + (assign33310_e27728 * locals.var_t4__blk1148_dn8)))))), (((locals.var_pparam_b4soicgdo_dn9 + locals.var_t3__blk1147_dn9) * locals.var_vgmd) - ((locals.var_t3__blk1147_dn9 * assign33310_e27733) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn9 + (((0.5 * locals.var_pparam_b4soickappa_dn9) * assign33310_e27731) + (assign33310_e27728 * locals.var_t4__blk1148_dn9)))))), ((((locals.var_pparam_b4soicgdo_dn10 + locals.var_t3__blk1147_dn10) * locals.var_vgmd) + (assign33310_e27721 * locals.var_vgmd_dn10)) - ((locals.var_t3__blk1147_dn10 * assign33310_e27733) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn10 + (((0.5 * locals.var_pparam_b4soickappa_dn10) * assign33310_e27731) + (assign33310_e27728 * locals.var_t4__blk1148_dn10)))))), (((locals.var_pparam_b4soicgdo_dn11 + locals.var_t3__blk1147_dn11) * locals.var_vgmd) - ((locals.var_t3__blk1147_dn11 * assign33310_e27733) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn11 + (((0.5 * locals.var_pparam_b4soickappa_dn11) * assign33310_e27731) + (assign33310_e27728 * locals.var_t4__blk1148_dn11)))))), (((locals.var_pparam_b4soicgdo_dn12 + locals.var_t3__blk1147_dn12) * locals.var_vgmd) - ((locals.var_t3__blk1147_dn12 * assign33310_e27733) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn12 + (((0.5 * locals.var_pparam_b4soickappa_dn12) * assign33310_e27731) + (assign33310_e27728 * locals.var_t4__blk1148_dn12)))))),)
    } else {
        (locals.var_qgdo, locals.var_qgdo_dn3, locals.var_qgdo_dn4, locals.var_qgdo_dn5, locals.var_qgdo_dn6, locals.var_qgdo_dn7, locals.var_qgdo_dn8, locals.var_qgdo_dn9, locals.var_qgdo_dn10, locals.var_qgdo_dn11, locals.var_qgdo_dn12,)
    }
};
        locals.var_qgdo = assign33310_e27737;
        locals.var_qgdo_dn3 = assign33310_e27737_d_n3;
        locals.var_qgdo_dn4 = assign33310_e27737_d_n4;
        locals.var_qgdo_dn5 = assign33310_e27737_d_n5;
        locals.var_qgdo_dn6 = assign33310_e27737_d_n6;
        locals.var_qgdo_dn7 = assign33310_e27737_d_n7;
        locals.var_qgdo_dn8 = assign33310_e27737_d_n8;
        locals.var_qgdo_dn9 = assign33310_e27737_d_n9;
        locals.var_qgdo_dn10 = assign33310_e27737_d_n10;
        locals.var_qgdo_dn11 = assign33310_e27737_d_n11;
        locals.var_qgdo_dn12 = assign33310_e27737_d_n12;

        let (assign33320_e27758, assign33320_e27758_d_n3, assign33320_e27758_d_n4, assign33320_e27758_d_n5, assign33320_e27758_d_n6, assign33320_e27758_d_n7, assign33320_e27758_d_n8, assign33320_e27758_d_n9, assign33320_e27758_d_n10, assign33320_e27758_d_n11, assign33320_e27758_d_n12,) = {
    if (locals.var_guard1771 == 0.0) {
        let assign33320_e27742: f64 = (locals.var_pparam_b4soicgdo + locals.var_t3__blk1147);
        let assign33320_e27744: f64 = (assign33320_e27742 * locals.var_vgd);
        let assign33320_e27749: f64 = (0.5 * locals.var_pparam_b4soickappa);
        let assign33320_e27752: f64 = (locals.var_t4__blk1148 - 1.0);
        let assign33320_e27753: f64 = (assign33320_e27749 * assign33320_e27752);
        let assign33320_e27754: f64 = (locals.var_t2__blk1146 + assign33320_e27753);
        let assign33320_e27755: f64 = (locals.var_t3__blk1147 * assign33320_e27754);
        let assign33320_e27756: f64 = (assign33320_e27744 - assign33320_e27755);
        (assign33320_e27756, (((locals.var_pparam_b4soicgdo_dn3 + locals.var_t3__blk1147_dn3) * locals.var_vgd) - ((locals.var_t3__blk1147_dn3 * assign33320_e27754) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn3 + (((0.5 * locals.var_pparam_b4soickappa_dn3) * assign33320_e27752) + (assign33320_e27749 * locals.var_t4__blk1148_dn3)))))), (((locals.var_pparam_b4soicgdo_dn4 + locals.var_t3__blk1147_dn4) * locals.var_vgd) - ((locals.var_t3__blk1147_dn4 * assign33320_e27754) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn4 + (((0.5 * locals.var_pparam_b4soickappa_dn4) * assign33320_e27752) + (assign33320_e27749 * locals.var_t4__blk1148_dn4)))))), (((locals.var_pparam_b4soicgdo_dn5 + locals.var_t3__blk1147_dn5) * locals.var_vgd) - ((locals.var_t3__blk1147_dn5 * assign33320_e27754) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn5 + (((0.5 * locals.var_pparam_b4soickappa_dn5) * assign33320_e27752) + (assign33320_e27749 * locals.var_t4__blk1148_dn5)))))), (((locals.var_pparam_b4soicgdo_dn6 + locals.var_t3__blk1147_dn6) * locals.var_vgd) - ((locals.var_t3__blk1147_dn6 * assign33320_e27754) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn6 + (((0.5 * locals.var_pparam_b4soickappa_dn6) * assign33320_e27752) + (assign33320_e27749 * locals.var_t4__blk1148_dn6)))))), ((((locals.var_pparam_b4soicgdo_dn7 + locals.var_t3__blk1147_dn7) * locals.var_vgd) + (assign33320_e27742 * locals.var_vgd_dn7)) - ((locals.var_t3__blk1147_dn7 * assign33320_e27754) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn7 + (((0.5 * locals.var_pparam_b4soickappa_dn7) * assign33320_e27752) + (assign33320_e27749 * locals.var_t4__blk1148_dn7)))))), ((((locals.var_pparam_b4soicgdo_dn8 + locals.var_t3__blk1147_dn8) * locals.var_vgd) + (assign33320_e27742 * locals.var_vgd_dn8)) - ((locals.var_t3__blk1147_dn8 * assign33320_e27754) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn8 + (((0.5 * locals.var_pparam_b4soickappa_dn8) * assign33320_e27752) + (assign33320_e27749 * locals.var_t4__blk1148_dn8)))))), ((((locals.var_pparam_b4soicgdo_dn9 + locals.var_t3__blk1147_dn9) * locals.var_vgd) + (assign33320_e27742 * locals.var_vgd_dn9)) - ((locals.var_t3__blk1147_dn9 * assign33320_e27754) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn9 + (((0.5 * locals.var_pparam_b4soickappa_dn9) * assign33320_e27752) + (assign33320_e27749 * locals.var_t4__blk1148_dn9)))))), (((locals.var_pparam_b4soicgdo_dn10 + locals.var_t3__blk1147_dn10) * locals.var_vgd) - ((locals.var_t3__blk1147_dn10 * assign33320_e27754) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn10 + (((0.5 * locals.var_pparam_b4soickappa_dn10) * assign33320_e27752) + (assign33320_e27749 * locals.var_t4__blk1148_dn10)))))), (((locals.var_pparam_b4soicgdo_dn11 + locals.var_t3__blk1147_dn11) * locals.var_vgd) - ((locals.var_t3__blk1147_dn11 * assign33320_e27754) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn11 + (((0.5 * locals.var_pparam_b4soickappa_dn11) * assign33320_e27752) + (assign33320_e27749 * locals.var_t4__blk1148_dn11)))))), (((locals.var_pparam_b4soicgdo_dn12 + locals.var_t3__blk1147_dn12) * locals.var_vgd) - ((locals.var_t3__blk1147_dn12 * assign33320_e27754) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn12 + (((0.5 * locals.var_pparam_b4soickappa_dn12) * assign33320_e27752) + (assign33320_e27749 * locals.var_t4__blk1148_dn12)))))),)
    } else {
        (locals.var_qgdo, locals.var_qgdo_dn3, locals.var_qgdo_dn4, locals.var_qgdo_dn5, locals.var_qgdo_dn6, locals.var_qgdo_dn7, locals.var_qgdo_dn8, locals.var_qgdo_dn9, locals.var_qgdo_dn10, locals.var_qgdo_dn11, locals.var_qgdo_dn12,)
    }
};
        locals.var_qgdo = assign33320_e27758;
        locals.var_qgdo_dn3 = assign33320_e27758_d_n3;
        locals.var_qgdo_dn4 = assign33320_e27758_d_n4;
        locals.var_qgdo_dn5 = assign33320_e27758_d_n5;
        locals.var_qgdo_dn6 = assign33320_e27758_d_n6;
        locals.var_qgdo_dn7 = assign33320_e27758_d_n7;
        locals.var_qgdo_dn8 = assign33320_e27758_d_n8;
        locals.var_qgdo_dn9 = assign33320_e27758_d_n9;
        locals.var_qgdo_dn10 = assign33320_e27758_d_n10;
        locals.var_qgdo_dn11 = assign33320_e27758_d_n11;
        locals.var_qgdo_dn12 = assign33320_e27758_d_n12;

        let assign33330_e27761: f64 = if locals.var_b4soirgatemod == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1772 = assign33330_e27761;

        let (assign33340_e27767, assign33340_e27767_d_n3, assign33340_e27767_d_n4, assign33340_e27767_d_n5, assign33340_e27767_d_n6, assign33340_e27767_d_n7, assign33340_e27767_d_n8, assign33340_e27767_d_n9, assign33340_e27767_d_n10, assign33340_e27767_d_n11, assign33340_e27767_d_n12,) = {
    if (locals.var_guard1772 != 0.0) {
        let assign33340_e27765: f64 = (locals.var_vgms + 0.02);
        (assign33340_e27765, 0.0, 0.0, 0.0, 0.0, 0.0, locals.var_vgms_dn8, 0.0, locals.var_vgms_dn10, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign33340_e27767;
        locals.var_t0__blk1144_dn3 = assign33340_e27767_d_n3;
        locals.var_t0__blk1144_dn4 = assign33340_e27767_d_n4;
        locals.var_t0__blk1144_dn5 = assign33340_e27767_d_n5;
        locals.var_t0__blk1144_dn6 = assign33340_e27767_d_n6;
        locals.var_t0__blk1144_dn7 = assign33340_e27767_d_n7;
        locals.var_t0__blk1144_dn8 = assign33340_e27767_d_n8;
        locals.var_t0__blk1144_dn9 = assign33340_e27767_d_n9;
        locals.var_t0__blk1144_dn10 = assign33340_e27767_d_n10;
        locals.var_t0__blk1144_dn11 = assign33340_e27767_d_n11;
        locals.var_t0__blk1144_dn12 = assign33340_e27767_d_n12;

        let (assign33350_e27774, assign33350_e27774_d_n3, assign33350_e27774_d_n4, assign33350_e27774_d_n5, assign33350_e27774_d_n6, assign33350_e27774_d_n7, assign33350_e27774_d_n8, assign33350_e27774_d_n9, assign33350_e27774_d_n10, assign33350_e27774_d_n11, assign33350_e27774_d_n12,) = {
    if (locals.var_guard1772 == 0.0) {
        let assign33350_e27772: f64 = (locals.var_vgs + 0.02);
        (assign33350_e27772, 0.0, 0.0, 0.0, 0.0, 0.0, locals.var_vgs_dn8, locals.var_vgs_dn9, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign33350_e27774;
        locals.var_t0__blk1144_dn3 = assign33350_e27774_d_n3;
        locals.var_t0__blk1144_dn4 = assign33350_e27774_d_n4;
        locals.var_t0__blk1144_dn5 = assign33350_e27774_d_n5;
        locals.var_t0__blk1144_dn6 = assign33350_e27774_d_n6;
        locals.var_t0__blk1144_dn7 = assign33350_e27774_d_n7;
        locals.var_t0__blk1144_dn8 = assign33350_e27774_d_n8;
        locals.var_t0__blk1144_dn9 = assign33350_e27774_d_n9;
        locals.var_t0__blk1144_dn10 = assign33350_e27774_d_n10;
        locals.var_t0__blk1144_dn11 = assign33350_e27774_d_n11;
        locals.var_t0__blk1144_dn12 = assign33350_e27774_d_n12;

        let assign33360_e27777: f64 = (locals.var_t0__blk1144 * locals.var_t0__blk1144);
        let assign33360_e27780: f64 = (4.0 * 0.02);
        let assign33360_e27781: f64 = (assign33360_e27777 + assign33360_e27780);
        let assign33360_e27782: f64 = (assign33360_e27781).sqrt();
        locals.var_t1__blk1145 = assign33360_e27782;
        locals.var_t1__blk1145_dn3 = (((locals.var_t0__blk1144_dn3 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn3)) / (2.0 * assign33360_e27782));
        locals.var_t1__blk1145_dn4 = (((locals.var_t0__blk1144_dn4 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn4)) / (2.0 * assign33360_e27782));
        locals.var_t1__blk1145_dn5 = (((locals.var_t0__blk1144_dn5 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn5)) / (2.0 * assign33360_e27782));
        locals.var_t1__blk1145_dn6 = (((locals.var_t0__blk1144_dn6 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn6)) / (2.0 * assign33360_e27782));
        locals.var_t1__blk1145_dn7 = (((locals.var_t0__blk1144_dn7 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn7)) / (2.0 * assign33360_e27782));
        locals.var_t1__blk1145_dn8 = (((locals.var_t0__blk1144_dn8 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn8)) / (2.0 * assign33360_e27782));
        locals.var_t1__blk1145_dn9 = (((locals.var_t0__blk1144_dn9 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn9)) / (2.0 * assign33360_e27782));
        locals.var_t1__blk1145_dn10 = (((locals.var_t0__blk1144_dn10 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn10)) / (2.0 * assign33360_e27782));
        locals.var_t1__blk1145_dn11 = (((locals.var_t0__blk1144_dn11 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn11)) / (2.0 * assign33360_e27782));
        locals.var_t1__blk1145_dn12 = (((locals.var_t0__blk1144_dn12 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn12)) / (2.0 * assign33360_e27782));

        let assign33370_e27786: f64 = (locals.var_t0__blk1144 - locals.var_t1__blk1145);
        let assign33370_e27787: f64 = (0.5 * assign33370_e27786);
        locals.var_t2__blk1146 = assign33370_e27787;
        locals.var_t2__blk1146_dn3 = (0.5 * (locals.var_t0__blk1144_dn3 - locals.var_t1__blk1145_dn3));
        locals.var_t2__blk1146_dn4 = (0.5 * (locals.var_t0__blk1144_dn4 - locals.var_t1__blk1145_dn4));
        locals.var_t2__blk1146_dn5 = (0.5 * (locals.var_t0__blk1144_dn5 - locals.var_t1__blk1145_dn5));
        locals.var_t2__blk1146_dn6 = (0.5 * (locals.var_t0__blk1144_dn6 - locals.var_t1__blk1145_dn6));
        locals.var_t2__blk1146_dn7 = (0.5 * (locals.var_t0__blk1144_dn7 - locals.var_t1__blk1145_dn7));
        locals.var_t2__blk1146_dn8 = (0.5 * (locals.var_t0__blk1144_dn8 - locals.var_t1__blk1145_dn8));
        locals.var_t2__blk1146_dn9 = (0.5 * (locals.var_t0__blk1144_dn9 - locals.var_t1__blk1145_dn9));
        locals.var_t2__blk1146_dn10 = (0.5 * (locals.var_t0__blk1144_dn10 - locals.var_t1__blk1145_dn10));
        locals.var_t2__blk1146_dn11 = (0.5 * (locals.var_t0__blk1144_dn11 - locals.var_t1__blk1145_dn11));
        locals.var_t2__blk1146_dn12 = (0.5 * (locals.var_t0__blk1144_dn12 - locals.var_t1__blk1145_dn12));

        let assign33380_e27790: f64 = (locals.var_pparam_b4soiwdioscv * locals.var_pparam_b4soicgsl);
        locals.var_t3__blk1147 = assign33380_e27790;
        locals.var_t3__blk1147_dn3 = ((locals.var_pparam_b4soiwdioscv_dn3 * locals.var_pparam_b4soicgsl) + (locals.var_pparam_b4soiwdioscv * locals.var_pparam_b4soicgsl_dn3));
        locals.var_t3__blk1147_dn4 = ((locals.var_pparam_b4soiwdioscv_dn4 * locals.var_pparam_b4soicgsl) + (locals.var_pparam_b4soiwdioscv * locals.var_pparam_b4soicgsl_dn4));
        locals.var_t3__blk1147_dn5 = ((locals.var_pparam_b4soiwdioscv_dn5 * locals.var_pparam_b4soicgsl) + (locals.var_pparam_b4soiwdioscv * locals.var_pparam_b4soicgsl_dn5));
        locals.var_t3__blk1147_dn6 = ((locals.var_pparam_b4soiwdioscv_dn6 * locals.var_pparam_b4soicgsl) + (locals.var_pparam_b4soiwdioscv * locals.var_pparam_b4soicgsl_dn6));
        locals.var_t3__blk1147_dn7 = ((locals.var_pparam_b4soiwdioscv_dn7 * locals.var_pparam_b4soicgsl) + (locals.var_pparam_b4soiwdioscv * locals.var_pparam_b4soicgsl_dn7));
        locals.var_t3__blk1147_dn8 = ((locals.var_pparam_b4soiwdioscv_dn8 * locals.var_pparam_b4soicgsl) + (locals.var_pparam_b4soiwdioscv * locals.var_pparam_b4soicgsl_dn8));
        locals.var_t3__blk1147_dn9 = ((locals.var_pparam_b4soiwdioscv_dn9 * locals.var_pparam_b4soicgsl) + (locals.var_pparam_b4soiwdioscv * locals.var_pparam_b4soicgsl_dn9));
        locals.var_t3__blk1147_dn10 = ((locals.var_pparam_b4soiwdioscv_dn10 * locals.var_pparam_b4soicgsl) + (locals.var_pparam_b4soiwdioscv * locals.var_pparam_b4soicgsl_dn10));
        locals.var_t3__blk1147_dn11 = ((locals.var_pparam_b4soiwdioscv_dn11 * locals.var_pparam_b4soicgsl) + (locals.var_pparam_b4soiwdioscv * locals.var_pparam_b4soicgsl_dn11));
        locals.var_t3__blk1147_dn12 = ((locals.var_pparam_b4soiwdioscv_dn12 * locals.var_pparam_b4soicgsl) + (locals.var_pparam_b4soiwdioscv * locals.var_pparam_b4soicgsl_dn12));

        let assign33390_e27794: f64 = (4.0 * locals.var_t2__blk1146);
        let assign33390_e27796: f64 = (assign33390_e27794 / locals.var_pparam_b4soickappa);
        let assign33390_e27797: f64 = (1.0 - assign33390_e27796);
        let assign33390_e27798: f64 = (assign33390_e27797).sqrt();
        locals.var_t4__blk1148 = assign33390_e27798;
        locals.var_t4__blk1148_dn3 = ((-((((4.0 * locals.var_t2__blk1146_dn3) * locals.var_pparam_b4soickappa) - (assign33390_e27794 * locals.var_pparam_b4soickappa_dn3)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign33390_e27798));
        locals.var_t4__blk1148_dn4 = ((-((((4.0 * locals.var_t2__blk1146_dn4) * locals.var_pparam_b4soickappa) - (assign33390_e27794 * locals.var_pparam_b4soickappa_dn4)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign33390_e27798));
        locals.var_t4__blk1148_dn5 = ((-((((4.0 * locals.var_t2__blk1146_dn5) * locals.var_pparam_b4soickappa) - (assign33390_e27794 * locals.var_pparam_b4soickappa_dn5)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign33390_e27798));
        locals.var_t4__blk1148_dn6 = ((-((((4.0 * locals.var_t2__blk1146_dn6) * locals.var_pparam_b4soickappa) - (assign33390_e27794 * locals.var_pparam_b4soickappa_dn6)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign33390_e27798));
        locals.var_t4__blk1148_dn7 = ((-((((4.0 * locals.var_t2__blk1146_dn7) * locals.var_pparam_b4soickappa) - (assign33390_e27794 * locals.var_pparam_b4soickappa_dn7)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign33390_e27798));
        locals.var_t4__blk1148_dn8 = ((-((((4.0 * locals.var_t2__blk1146_dn8) * locals.var_pparam_b4soickappa) - (assign33390_e27794 * locals.var_pparam_b4soickappa_dn8)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign33390_e27798));
        locals.var_t4__blk1148_dn9 = ((-((((4.0 * locals.var_t2__blk1146_dn9) * locals.var_pparam_b4soickappa) - (assign33390_e27794 * locals.var_pparam_b4soickappa_dn9)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign33390_e27798));
        locals.var_t4__blk1148_dn10 = ((-((((4.0 * locals.var_t2__blk1146_dn10) * locals.var_pparam_b4soickappa) - (assign33390_e27794 * locals.var_pparam_b4soickappa_dn10)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign33390_e27798));
        locals.var_t4__blk1148_dn11 = ((-((((4.0 * locals.var_t2__blk1146_dn11) * locals.var_pparam_b4soickappa) - (assign33390_e27794 * locals.var_pparam_b4soickappa_dn11)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign33390_e27798));
        locals.var_t4__blk1148_dn12 = ((-((((4.0 * locals.var_t2__blk1146_dn12) * locals.var_pparam_b4soickappa) - (assign33390_e27794 * locals.var_pparam_b4soickappa_dn12)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign33390_e27798));

        let assign33400_e27801: f64 = if locals.var_b4soirgatemod == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1773 = assign33400_e27801;

        let (assign33410_e27821, assign33410_e27821_d_n3, assign33410_e27821_d_n4, assign33410_e27821_d_n5, assign33410_e27821_d_n6, assign33410_e27821_d_n7, assign33410_e27821_d_n8, assign33410_e27821_d_n9, assign33410_e27821_d_n10, assign33410_e27821_d_n11, assign33410_e27821_d_n12,) = {
    if (locals.var_guard1773 != 0.0) {
        let assign33410_e27805: f64 = (locals.var_pparam_b4soicgso + locals.var_t3__blk1147);
        let assign33410_e27807: f64 = (assign33410_e27805 * locals.var_vgms);
        let assign33410_e27812: f64 = (0.5 * locals.var_pparam_b4soickappa);
        let assign33410_e27815: f64 = (locals.var_t4__blk1148 - 1.0);
        let assign33410_e27816: f64 = (assign33410_e27812 * assign33410_e27815);
        let assign33410_e27817: f64 = (locals.var_t2__blk1146 + assign33410_e27816);
        let assign33410_e27818: f64 = (locals.var_t3__blk1147 * assign33410_e27817);
        let assign33410_e27819: f64 = (assign33410_e27807 - assign33410_e27818);
        (assign33410_e27819, (((locals.var_pparam_b4soicgso_dn3 + locals.var_t3__blk1147_dn3) * locals.var_vgms) - ((locals.var_t3__blk1147_dn3 * assign33410_e27817) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn3 + (((0.5 * locals.var_pparam_b4soickappa_dn3) * assign33410_e27815) + (assign33410_e27812 * locals.var_t4__blk1148_dn3)))))), (((locals.var_pparam_b4soicgso_dn4 + locals.var_t3__blk1147_dn4) * locals.var_vgms) - ((locals.var_t3__blk1147_dn4 * assign33410_e27817) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn4 + (((0.5 * locals.var_pparam_b4soickappa_dn4) * assign33410_e27815) + (assign33410_e27812 * locals.var_t4__blk1148_dn4)))))), (((locals.var_pparam_b4soicgso_dn5 + locals.var_t3__blk1147_dn5) * locals.var_vgms) - ((locals.var_t3__blk1147_dn5 * assign33410_e27817) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn5 + (((0.5 * locals.var_pparam_b4soickappa_dn5) * assign33410_e27815) + (assign33410_e27812 * locals.var_t4__blk1148_dn5)))))), (((locals.var_pparam_b4soicgso_dn6 + locals.var_t3__blk1147_dn6) * locals.var_vgms) - ((locals.var_t3__blk1147_dn6 * assign33410_e27817) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn6 + (((0.5 * locals.var_pparam_b4soickappa_dn6) * assign33410_e27815) + (assign33410_e27812 * locals.var_t4__blk1148_dn6)))))), (((locals.var_pparam_b4soicgso_dn7 + locals.var_t3__blk1147_dn7) * locals.var_vgms) - ((locals.var_t3__blk1147_dn7 * assign33410_e27817) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn7 + (((0.5 * locals.var_pparam_b4soickappa_dn7) * assign33410_e27815) + (assign33410_e27812 * locals.var_t4__blk1148_dn7)))))), ((((locals.var_pparam_b4soicgso_dn8 + locals.var_t3__blk1147_dn8) * locals.var_vgms) + (assign33410_e27805 * locals.var_vgms_dn8)) - ((locals.var_t3__blk1147_dn8 * assign33410_e27817) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn8 + (((0.5 * locals.var_pparam_b4soickappa_dn8) * assign33410_e27815) + (assign33410_e27812 * locals.var_t4__blk1148_dn8)))))), (((locals.var_pparam_b4soicgso_dn9 + locals.var_t3__blk1147_dn9) * locals.var_vgms) - ((locals.var_t3__blk1147_dn9 * assign33410_e27817) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn9 + (((0.5 * locals.var_pparam_b4soickappa_dn9) * assign33410_e27815) + (assign33410_e27812 * locals.var_t4__blk1148_dn9)))))), ((((locals.var_pparam_b4soicgso_dn10 + locals.var_t3__blk1147_dn10) * locals.var_vgms) + (assign33410_e27805 * locals.var_vgms_dn10)) - ((locals.var_t3__blk1147_dn10 * assign33410_e27817) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn10 + (((0.5 * locals.var_pparam_b4soickappa_dn10) * assign33410_e27815) + (assign33410_e27812 * locals.var_t4__blk1148_dn10)))))), (((locals.var_pparam_b4soicgso_dn11 + locals.var_t3__blk1147_dn11) * locals.var_vgms) - ((locals.var_t3__blk1147_dn11 * assign33410_e27817) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn11 + (((0.5 * locals.var_pparam_b4soickappa_dn11) * assign33410_e27815) + (assign33410_e27812 * locals.var_t4__blk1148_dn11)))))), (((locals.var_pparam_b4soicgso_dn12 + locals.var_t3__blk1147_dn12) * locals.var_vgms) - ((locals.var_t3__blk1147_dn12 * assign33410_e27817) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn12 + (((0.5 * locals.var_pparam_b4soickappa_dn12) * assign33410_e27815) + (assign33410_e27812 * locals.var_t4__blk1148_dn12)))))),)
    } else {
        (locals.var_qgso, locals.var_qgso_dn3, locals.var_qgso_dn4, locals.var_qgso_dn5, locals.var_qgso_dn6, locals.var_qgso_dn7, locals.var_qgso_dn8, locals.var_qgso_dn9, locals.var_qgso_dn10, locals.var_qgso_dn11, locals.var_qgso_dn12,)
    }
};
        locals.var_qgso = assign33410_e27821;
        locals.var_qgso_dn3 = assign33410_e27821_d_n3;
        locals.var_qgso_dn4 = assign33410_e27821_d_n4;
        locals.var_qgso_dn5 = assign33410_e27821_d_n5;
        locals.var_qgso_dn6 = assign33410_e27821_d_n6;
        locals.var_qgso_dn7 = assign33410_e27821_d_n7;
        locals.var_qgso_dn8 = assign33410_e27821_d_n8;
        locals.var_qgso_dn9 = assign33410_e27821_d_n9;
        locals.var_qgso_dn10 = assign33410_e27821_d_n10;
        locals.var_qgso_dn11 = assign33410_e27821_d_n11;
        locals.var_qgso_dn12 = assign33410_e27821_d_n12;

        let (assign33420_e27842, assign33420_e27842_d_n3, assign33420_e27842_d_n4, assign33420_e27842_d_n5, assign33420_e27842_d_n6, assign33420_e27842_d_n7, assign33420_e27842_d_n8, assign33420_e27842_d_n9, assign33420_e27842_d_n10, assign33420_e27842_d_n11, assign33420_e27842_d_n12,) = {
    if (locals.var_guard1773 == 0.0) {
        let assign33420_e27826: f64 = (locals.var_pparam_b4soicgso + locals.var_t3__blk1147);
        let assign33420_e27828: f64 = (assign33420_e27826 * locals.var_vgs);
        let assign33420_e27833: f64 = (0.5 * locals.var_pparam_b4soickappa);
        let assign33420_e27836: f64 = (locals.var_t4__blk1148 - 1.0);
        let assign33420_e27837: f64 = (assign33420_e27833 * assign33420_e27836);
        let assign33420_e27838: f64 = (locals.var_t2__blk1146 + assign33420_e27837);
        let assign33420_e27839: f64 = (locals.var_t3__blk1147 * assign33420_e27838);
        let assign33420_e27840: f64 = (assign33420_e27828 - assign33420_e27839);
        (assign33420_e27840, (((locals.var_pparam_b4soicgso_dn3 + locals.var_t3__blk1147_dn3) * locals.var_vgs) - ((locals.var_t3__blk1147_dn3 * assign33420_e27838) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn3 + (((0.5 * locals.var_pparam_b4soickappa_dn3) * assign33420_e27836) + (assign33420_e27833 * locals.var_t4__blk1148_dn3)))))), (((locals.var_pparam_b4soicgso_dn4 + locals.var_t3__blk1147_dn4) * locals.var_vgs) - ((locals.var_t3__blk1147_dn4 * assign33420_e27838) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn4 + (((0.5 * locals.var_pparam_b4soickappa_dn4) * assign33420_e27836) + (assign33420_e27833 * locals.var_t4__blk1148_dn4)))))), (((locals.var_pparam_b4soicgso_dn5 + locals.var_t3__blk1147_dn5) * locals.var_vgs) - ((locals.var_t3__blk1147_dn5 * assign33420_e27838) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn5 + (((0.5 * locals.var_pparam_b4soickappa_dn5) * assign33420_e27836) + (assign33420_e27833 * locals.var_t4__blk1148_dn5)))))), (((locals.var_pparam_b4soicgso_dn6 + locals.var_t3__blk1147_dn6) * locals.var_vgs) - ((locals.var_t3__blk1147_dn6 * assign33420_e27838) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn6 + (((0.5 * locals.var_pparam_b4soickappa_dn6) * assign33420_e27836) + (assign33420_e27833 * locals.var_t4__blk1148_dn6)))))), (((locals.var_pparam_b4soicgso_dn7 + locals.var_t3__blk1147_dn7) * locals.var_vgs) - ((locals.var_t3__blk1147_dn7 * assign33420_e27838) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn7 + (((0.5 * locals.var_pparam_b4soickappa_dn7) * assign33420_e27836) + (assign33420_e27833 * locals.var_t4__blk1148_dn7)))))), ((((locals.var_pparam_b4soicgso_dn8 + locals.var_t3__blk1147_dn8) * locals.var_vgs) + (assign33420_e27826 * locals.var_vgs_dn8)) - ((locals.var_t3__blk1147_dn8 * assign33420_e27838) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn8 + (((0.5 * locals.var_pparam_b4soickappa_dn8) * assign33420_e27836) + (assign33420_e27833 * locals.var_t4__blk1148_dn8)))))), ((((locals.var_pparam_b4soicgso_dn9 + locals.var_t3__blk1147_dn9) * locals.var_vgs) + (assign33420_e27826 * locals.var_vgs_dn9)) - ((locals.var_t3__blk1147_dn9 * assign33420_e27838) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn9 + (((0.5 * locals.var_pparam_b4soickappa_dn9) * assign33420_e27836) + (assign33420_e27833 * locals.var_t4__blk1148_dn9)))))), (((locals.var_pparam_b4soicgso_dn10 + locals.var_t3__blk1147_dn10) * locals.var_vgs) - ((locals.var_t3__blk1147_dn10 * assign33420_e27838) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn10 + (((0.5 * locals.var_pparam_b4soickappa_dn10) * assign33420_e27836) + (assign33420_e27833 * locals.var_t4__blk1148_dn10)))))), (((locals.var_pparam_b4soicgso_dn11 + locals.var_t3__blk1147_dn11) * locals.var_vgs) - ((locals.var_t3__blk1147_dn11 * assign33420_e27838) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn11 + (((0.5 * locals.var_pparam_b4soickappa_dn11) * assign33420_e27836) + (assign33420_e27833 * locals.var_t4__blk1148_dn11)))))), (((locals.var_pparam_b4soicgso_dn12 + locals.var_t3__blk1147_dn12) * locals.var_vgs) - ((locals.var_t3__blk1147_dn12 * assign33420_e27838) + (locals.var_t3__blk1147 * (locals.var_t2__blk1146_dn12 + (((0.5 * locals.var_pparam_b4soickappa_dn12) * assign33420_e27836) + (assign33420_e27833 * locals.var_t4__blk1148_dn12)))))),)
    } else {
        (locals.var_qgso, locals.var_qgso_dn3, locals.var_qgso_dn4, locals.var_qgso_dn5, locals.var_qgso_dn6, locals.var_qgso_dn7, locals.var_qgso_dn8, locals.var_qgso_dn9, locals.var_qgso_dn10, locals.var_qgso_dn11, locals.var_qgso_dn12,)
    }
};
        locals.var_qgso = assign33420_e27842;
        locals.var_qgso_dn3 = assign33420_e27842_d_n3;
        locals.var_qgso_dn4 = assign33420_e27842_d_n4;
        locals.var_qgso_dn5 = assign33420_e27842_d_n5;
        locals.var_qgso_dn6 = assign33420_e27842_d_n6;
        locals.var_qgso_dn7 = assign33420_e27842_d_n7;
        locals.var_qgso_dn8 = assign33420_e27842_d_n8;
        locals.var_qgso_dn9 = assign33420_e27842_d_n9;
        locals.var_qgso_dn10 = assign33420_e27842_d_n10;
        locals.var_qgso_dn11 = assign33420_e27842_d_n11;
        locals.var_qgso_dn12 = assign33420_e27842_d_n12;

        let assign33430_e27845: f64 = if locals.var_b4soinf != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1774 = assign33430_e27845;

        let (assign33440_e27851, assign33440_e27851_d_n3, assign33440_e27851_d_n4, assign33440_e27851_d_n5, assign33440_e27851_d_n6, assign33440_e27851_d_n7, assign33440_e27851_d_n8, assign33440_e27851_d_n9, assign33440_e27851_d_n10, assign33440_e27851_d_n11, assign33440_e27851_d_n12,) = {
    if (locals.var_guard1774 != 0.0) {
        let assign33440_e27849: f64 = (locals.var_qgdo * locals.var_b4soinf);
        (assign33440_e27849, (locals.var_qgdo_dn3 * locals.var_b4soinf), (locals.var_qgdo_dn4 * locals.var_b4soinf), (locals.var_qgdo_dn5 * locals.var_b4soinf), (locals.var_qgdo_dn6 * locals.var_b4soinf), (locals.var_qgdo_dn7 * locals.var_b4soinf), (locals.var_qgdo_dn8 * locals.var_b4soinf), (locals.var_qgdo_dn9 * locals.var_b4soinf), (locals.var_qgdo_dn10 * locals.var_b4soinf), (locals.var_qgdo_dn11 * locals.var_b4soinf), (locals.var_qgdo_dn12 * locals.var_b4soinf),)
    } else {
        (locals.var_qgdo, locals.var_qgdo_dn3, locals.var_qgdo_dn4, locals.var_qgdo_dn5, locals.var_qgdo_dn6, locals.var_qgdo_dn7, locals.var_qgdo_dn8, locals.var_qgdo_dn9, locals.var_qgdo_dn10, locals.var_qgdo_dn11, locals.var_qgdo_dn12,)
    }
};
        locals.var_qgdo = assign33440_e27851;
        locals.var_qgdo_dn3 = assign33440_e27851_d_n3;
        locals.var_qgdo_dn4 = assign33440_e27851_d_n4;
        locals.var_qgdo_dn5 = assign33440_e27851_d_n5;
        locals.var_qgdo_dn6 = assign33440_e27851_d_n6;
        locals.var_qgdo_dn7 = assign33440_e27851_d_n7;
        locals.var_qgdo_dn8 = assign33440_e27851_d_n8;
        locals.var_qgdo_dn9 = assign33440_e27851_d_n9;
        locals.var_qgdo_dn10 = assign33440_e27851_d_n10;
        locals.var_qgdo_dn11 = assign33440_e27851_d_n11;
        locals.var_qgdo_dn12 = assign33440_e27851_d_n12;

        let (assign33450_e27857, assign33450_e27857_d_n3, assign33450_e27857_d_n4, assign33450_e27857_d_n5, assign33450_e27857_d_n6, assign33450_e27857_d_n7, assign33450_e27857_d_n8, assign33450_e27857_d_n9, assign33450_e27857_d_n10, assign33450_e27857_d_n11, assign33450_e27857_d_n12,) = {
    if (locals.var_guard1774 != 0.0) {
        let assign33450_e27855: f64 = (locals.var_qgso * locals.var_b4soinf);
        (assign33450_e27855, (locals.var_qgso_dn3 * locals.var_b4soinf), (locals.var_qgso_dn4 * locals.var_b4soinf), (locals.var_qgso_dn5 * locals.var_b4soinf), (locals.var_qgso_dn6 * locals.var_b4soinf), (locals.var_qgso_dn7 * locals.var_b4soinf), (locals.var_qgso_dn8 * locals.var_b4soinf), (locals.var_qgso_dn9 * locals.var_b4soinf), (locals.var_qgso_dn10 * locals.var_b4soinf), (locals.var_qgso_dn11 * locals.var_b4soinf), (locals.var_qgso_dn12 * locals.var_b4soinf),)
    } else {
        (locals.var_qgso, locals.var_qgso_dn3, locals.var_qgso_dn4, locals.var_qgso_dn5, locals.var_qgso_dn6, locals.var_qgso_dn7, locals.var_qgso_dn8, locals.var_qgso_dn9, locals.var_qgso_dn10, locals.var_qgso_dn11, locals.var_qgso_dn12,)
    }
};
        locals.var_qgso = assign33450_e27857;
        locals.var_qgso_dn3 = assign33450_e27857_d_n3;
        locals.var_qgso_dn4 = assign33450_e27857_d_n4;
        locals.var_qgso_dn5 = assign33450_e27857_d_n5;
        locals.var_qgso_dn6 = assign33450_e27857_d_n6;
        locals.var_qgso_dn7 = assign33450_e27857_d_n7;
        locals.var_qgso_dn8 = assign33450_e27857_d_n8;
        locals.var_qgso_dn9 = assign33450_e27857_d_n9;
        locals.var_qgso_dn10 = assign33450_e27857_d_n10;
        locals.var_qgso_dn11 = assign33450_e27857_d_n11;
        locals.var_qgso_dn12 = assign33450_e27857_d_n12;

        locals.var_qgi_1 = locals.var_qgate;
        locals.var_qgi_1_dn3 = locals.var_qgate_dn3;
        locals.var_qgi_1_dn4 = locals.var_qgate_dn4;
        locals.var_qgi_1_dn5 = locals.var_qgate_dn5;
        locals.var_qgi_1_dn6 = locals.var_qgate_dn6;
        locals.var_qgi_1_dn7 = locals.var_qgate_dn7;
        locals.var_qgi_1_dn8 = locals.var_qgate_dn8;
        locals.var_qgi_1_dn9 = locals.var_qgate_dn9;
        locals.var_qgi_1_dn10 = locals.var_qgate_dn10;
        locals.var_qgi_1_dn11 = locals.var_qgate_dn11;
        locals.var_qgi_1_dn12 = locals.var_qgate_dn12;

        let assign33470_e27861: f64 = (locals.var_qgso + locals.var_qgdo);
        locals.var_qov_1 = assign33470_e27861;
        locals.var_qov_1_dn3 = (locals.var_qgso_dn3 + locals.var_qgdo_dn3);
        locals.var_qov_1_dn4 = (locals.var_qgso_dn4 + locals.var_qgdo_dn4);
        locals.var_qov_1_dn5 = (locals.var_qgso_dn5 + locals.var_qgdo_dn5);
        locals.var_qov_1_dn6 = (locals.var_qgso_dn6 + locals.var_qgdo_dn6);
        locals.var_qov_1_dn7 = (locals.var_qgso_dn7 + locals.var_qgdo_dn7);
        locals.var_qov_1_dn8 = (locals.var_qgso_dn8 + locals.var_qgdo_dn8);
        locals.var_qov_1_dn9 = (locals.var_qgso_dn9 + locals.var_qgdo_dn9);
        locals.var_qov_1_dn10 = (locals.var_qgso_dn10 + locals.var_qgdo_dn10);
        locals.var_qov_1_dn11 = (locals.var_qgso_dn11 + locals.var_qgdo_dn11);
        locals.var_qov_1_dn12 = (locals.var_qgso_dn12 + locals.var_qgdo_dn12);

        let assign33480_e27864: f64 = (locals.var_qgi_1 + locals.var_qov_1);
        locals.var_qgate = assign33480_e27864;
        locals.var_qgate_dn3 = (locals.var_qgi_1_dn3 + locals.var_qov_1_dn3);
        locals.var_qgate_dn4 = (locals.var_qgi_1_dn4 + locals.var_qov_1_dn4);
        locals.var_qgate_dn5 = (locals.var_qgi_1_dn5 + locals.var_qov_1_dn5);
        locals.var_qgate_dn6 = (locals.var_qgi_1_dn6 + locals.var_qov_1_dn6);
        locals.var_qgate_dn7 = (locals.var_qgi_1_dn7 + locals.var_qov_1_dn7);
        locals.var_qgate_dn8 = (locals.var_qgi_1_dn8 + locals.var_qov_1_dn8);
        locals.var_qgate_dn9 = (locals.var_qgi_1_dn9 + locals.var_qov_1_dn9);
        locals.var_qgate_dn10 = (locals.var_qgi_1_dn10 + locals.var_qov_1_dn10);
        locals.var_qgate_dn11 = (locals.var_qgi_1_dn11 + locals.var_qov_1_dn11);
        locals.var_qgate_dn12 = (locals.var_qgi_1_dn12 + locals.var_qov_1_dn12);

        let assign33580_e27927: f64 = if p.p213 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1788 = assign33580_e27927;

        let assign33590_e27930: f64 = if p.p213 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1789 = assign33590_e27930;

        let (assign33630_e27967, assign33630_e27967_d_n3, assign33630_e27967_d_n4, assign33630_e27967_d_n5, assign33630_e27967_d_n6, assign33630_e27967_d_n7, assign33630_e27967_d_n8, assign33630_e27967_d_n9, assign33630_e27967_d_n10, assign33630_e27967_d_n11, assign33630_e27967_d_n12,) = {
    if ((locals.var_guard1789 != 0.0) && (locals.var_guard1788 == 0.0)) {
        let assign33630_e27963: f64 = (locals.var_b4soigm + locals.var_b4soigds);
        let assign33630_e27965: f64 = (assign33630_e27963 + locals.var_b4soigmbs);
        (assign33630_e27965, ((locals.var_b4soigm_dn3 + locals.var_b4soigds_dn3) + locals.var_b4soigmbs_dn3), ((locals.var_b4soigm_dn4 + locals.var_b4soigds_dn4) + locals.var_b4soigmbs_dn4), ((locals.var_b4soigm_dn5 + locals.var_b4soigds_dn5) + locals.var_b4soigmbs_dn5), ((locals.var_b4soigm_dn6 + locals.var_b4soigds_dn6) + locals.var_b4soigmbs_dn6), ((locals.var_b4soigm_dn7 + locals.var_b4soigds_dn7) + locals.var_b4soigmbs_dn7), ((locals.var_b4soigm_dn8 + locals.var_b4soigds_dn8) + locals.var_b4soigmbs_dn8), ((locals.var_b4soigm_dn9 + locals.var_b4soigds_dn9) + locals.var_b4soigmbs_dn9), ((locals.var_b4soigm_dn10 + locals.var_b4soigds_dn10) + locals.var_b4soigmbs_dn10), ((locals.var_b4soigm_dn11 + locals.var_b4soigds_dn11) + locals.var_b4soigmbs_dn11), ((locals.var_b4soigm_dn12 + locals.var_b4soigds_dn12) + locals.var_b4soigmbs_dn12),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign33630_e27967;
        locals.var_t0__blk1144_dn3 = assign33630_e27967_d_n3;
        locals.var_t0__blk1144_dn4 = assign33630_e27967_d_n4;
        locals.var_t0__blk1144_dn5 = assign33630_e27967_d_n5;
        locals.var_t0__blk1144_dn6 = assign33630_e27967_d_n6;
        locals.var_t0__blk1144_dn7 = assign33630_e27967_d_n7;
        locals.var_t0__blk1144_dn8 = assign33630_e27967_d_n8;
        locals.var_t0__blk1144_dn9 = assign33630_e27967_d_n9;
        locals.var_t0__blk1144_dn10 = assign33630_e27967_d_n10;
        locals.var_t0__blk1144_dn11 = assign33630_e27967_d_n11;
        locals.var_t0__blk1144_dn12 = assign33630_e27967_d_n12;

        let (assign33640_e27976, assign33640_e27976_d_n3, assign33640_e27976_d_n4, assign33640_e27976_d_n5, assign33640_e27976_d_n6, assign33640_e27976_d_n7, assign33640_e27976_d_n8, assign33640_e27976_d_n9, assign33640_e27976_d_n10, assign33640_e27976_d_n11, assign33640_e27976_d_n12,) = {
    if ((locals.var_guard1789 != 0.0) && (locals.var_guard1788 == 0.0)) {
        let assign33640_e27974: f64 = (locals.var_t0__blk1144 * locals.var_t0__blk1144);
        (assign33640_e27974, ((locals.var_t0__blk1144_dn3 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn3)), ((locals.var_t0__blk1144_dn4 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn4)), ((locals.var_t0__blk1144_dn5 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn5)), ((locals.var_t0__blk1144_dn6 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn6)), ((locals.var_t0__blk1144_dn7 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn7)), ((locals.var_t0__blk1144_dn8 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn8)), ((locals.var_t0__blk1144_dn9 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn9)), ((locals.var_t0__blk1144_dn10 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn10)), ((locals.var_t0__blk1144_dn11 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn11)), ((locals.var_t0__blk1144_dn12 * locals.var_t0__blk1144) + (locals.var_t0__blk1144 * locals.var_t0__blk1144_dn12)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign33640_e27976;
        locals.var_t0__blk1144_dn3 = assign33640_e27976_d_n3;
        locals.var_t0__blk1144_dn4 = assign33640_e27976_d_n4;
        locals.var_t0__blk1144_dn5 = assign33640_e27976_d_n5;
        locals.var_t0__blk1144_dn6 = assign33640_e27976_d_n6;
        locals.var_t0__blk1144_dn7 = assign33640_e27976_d_n7;
        locals.var_t0__blk1144_dn8 = assign33640_e27976_d_n8;
        locals.var_t0__blk1144_dn9 = assign33640_e27976_d_n9;
        locals.var_t0__blk1144_dn10 = assign33640_e27976_d_n10;
        locals.var_t0__blk1144_dn11 = assign33640_e27976_d_n11;
        locals.var_t0__blk1144_dn12 = assign33640_e27976_d_n12;

        let (assign33650_e27987, assign33650_e27987_d_n3, assign33650_e27987_d_n4, assign33650_e27987_d_n5, assign33650_e27987_d_n6, assign33650_e27987_d_n7, assign33650_e27987_d_n8, assign33650_e27987_d_n9, assign33650_e27987_d_n10, assign33650_e27987_d_n11, assign33650_e27987_d_n12,) = {
    if ((locals.var_guard1789 != 0.0) && (locals.var_guard1788 == 0.0)) {
        let assign33650_e27983: f64 = (2.0 * locals.var_vsattemp);
        let assign33650_e27985: f64 = (assign33650_e27983 / locals.var_b4soiueff);
        (assign33650_e27985, ((((2.0 * locals.var_vsattemp_dn3) * locals.var_b4soiueff) - (assign33650_e27983 * locals.var_b4soiueff_dn3)) / (locals.var_b4soiueff * locals.var_b4soiueff)), ((((2.0 * locals.var_vsattemp_dn4) * locals.var_b4soiueff) - (assign33650_e27983 * locals.var_b4soiueff_dn4)) / (locals.var_b4soiueff * locals.var_b4soiueff)), ((((2.0 * locals.var_vsattemp_dn5) * locals.var_b4soiueff) - (assign33650_e27983 * locals.var_b4soiueff_dn5)) / (locals.var_b4soiueff * locals.var_b4soiueff)), ((((2.0 * locals.var_vsattemp_dn6) * locals.var_b4soiueff) - (assign33650_e27983 * locals.var_b4soiueff_dn6)) / (locals.var_b4soiueff * locals.var_b4soiueff)), ((((2.0 * locals.var_vsattemp_dn7) * locals.var_b4soiueff) - (assign33650_e27983 * locals.var_b4soiueff_dn7)) / (locals.var_b4soiueff * locals.var_b4soiueff)), ((((2.0 * locals.var_vsattemp_dn8) * locals.var_b4soiueff) - (assign33650_e27983 * locals.var_b4soiueff_dn8)) / (locals.var_b4soiueff * locals.var_b4soiueff)), ((((2.0 * locals.var_vsattemp_dn9) * locals.var_b4soiueff) - (assign33650_e27983 * locals.var_b4soiueff_dn9)) / (locals.var_b4soiueff * locals.var_b4soiueff)), ((((2.0 * locals.var_vsattemp_dn10) * locals.var_b4soiueff) - (assign33650_e27983 * locals.var_b4soiueff_dn10)) / (locals.var_b4soiueff * locals.var_b4soiueff)), ((((2.0 * locals.var_vsattemp_dn11) * locals.var_b4soiueff) - (assign33650_e27983 * locals.var_b4soiueff_dn11)) / (locals.var_b4soiueff * locals.var_b4soiueff)), ((((2.0 * locals.var_vsattemp_dn12) * locals.var_b4soiueff) - (assign33650_e27983 * locals.var_b4soiueff_dn12)) / (locals.var_b4soiueff * locals.var_b4soiueff)),)
    } else {
        (locals.var_esat_1, locals.var_esat_1_dn3, locals.var_esat_1_dn4, locals.var_esat_1_dn5, locals.var_esat_1_dn6, locals.var_esat_1_dn7, locals.var_esat_1_dn8, locals.var_esat_1_dn9, locals.var_esat_1_dn10, locals.var_esat_1_dn11, locals.var_esat_1_dn12,)
    }
};
        locals.var_esat_1 = assign33650_e27987;
        locals.var_esat_1_dn3 = assign33650_e27987_d_n3;
        locals.var_esat_1_dn4 = assign33650_e27987_d_n4;
        locals.var_esat_1_dn5 = assign33650_e27987_d_n5;
        locals.var_esat_1_dn6 = assign33650_e27987_d_n6;
        locals.var_esat_1_dn7 = assign33650_e27987_d_n7;
        locals.var_esat_1_dn8 = assign33650_e27987_d_n8;
        locals.var_esat_1_dn9 = assign33650_e27987_d_n9;
        locals.var_esat_1_dn10 = assign33650_e27987_d_n10;
        locals.var_esat_1_dn11 = assign33650_e27987_d_n11;
        locals.var_esat_1_dn12 = assign33650_e27987_d_n12;

        let (assign33660_e27998, assign33660_e27998_d_n3, assign33660_e27998_d_n4, assign33660_e27998_d_n5, assign33660_e27998_d_n6, assign33660_e27998_d_n7, assign33660_e27998_d_n8, assign33660_e27998_d_n9, assign33660_e27998_d_n10, assign33660_e27998_d_n11, assign33660_e27998_d_n12,) = {
    if ((locals.var_guard1789 != 0.0) && (locals.var_guard1788 == 0.0)) {
        let assign33660_e27995: f64 = (locals.var_esat_1 * locals.var_pparam_b4soileff);
        let assign33660_e27996: f64 = (locals.var_b4soivgsteff / assign33660_e27995);
        (assign33660_e27996, (((locals.var_b4soivgsteff_dn3 * assign33660_e27995) - (locals.var_b4soivgsteff * ((locals.var_esat_1_dn3 * locals.var_pparam_b4soileff) + (locals.var_esat_1 * locals.var_pparam_b4soileff_dn3)))) / (assign33660_e27995 * assign33660_e27995)), (((locals.var_b4soivgsteff_dn4 * assign33660_e27995) - (locals.var_b4soivgsteff * ((locals.var_esat_1_dn4 * locals.var_pparam_b4soileff) + (locals.var_esat_1 * locals.var_pparam_b4soileff_dn4)))) / (assign33660_e27995 * assign33660_e27995)), (((locals.var_b4soivgsteff_dn5 * assign33660_e27995) - (locals.var_b4soivgsteff * ((locals.var_esat_1_dn5 * locals.var_pparam_b4soileff) + (locals.var_esat_1 * locals.var_pparam_b4soileff_dn5)))) / (assign33660_e27995 * assign33660_e27995)), (((locals.var_b4soivgsteff_dn6 * assign33660_e27995) - (locals.var_b4soivgsteff * ((locals.var_esat_1_dn6 * locals.var_pparam_b4soileff) + (locals.var_esat_1 * locals.var_pparam_b4soileff_dn6)))) / (assign33660_e27995 * assign33660_e27995)), (((locals.var_b4soivgsteff_dn7 * assign33660_e27995) - (locals.var_b4soivgsteff * ((locals.var_esat_1_dn7 * locals.var_pparam_b4soileff) + (locals.var_esat_1 * locals.var_pparam_b4soileff_dn7)))) / (assign33660_e27995 * assign33660_e27995)), (((locals.var_b4soivgsteff_dn8 * assign33660_e27995) - (locals.var_b4soivgsteff * ((locals.var_esat_1_dn8 * locals.var_pparam_b4soileff) + (locals.var_esat_1 * locals.var_pparam_b4soileff_dn8)))) / (assign33660_e27995 * assign33660_e27995)), (((locals.var_b4soivgsteff_dn9 * assign33660_e27995) - (locals.var_b4soivgsteff * ((locals.var_esat_1_dn9 * locals.var_pparam_b4soileff) + (locals.var_esat_1 * locals.var_pparam_b4soileff_dn9)))) / (assign33660_e27995 * assign33660_e27995)), (((locals.var_b4soivgsteff_dn10 * assign33660_e27995) - (locals.var_b4soivgsteff * ((locals.var_esat_1_dn10 * locals.var_pparam_b4soileff) + (locals.var_esat_1 * locals.var_pparam_b4soileff_dn10)))) / (assign33660_e27995 * assign33660_e27995)), (((locals.var_b4soivgsteff_dn11 * assign33660_e27995) - (locals.var_b4soivgsteff * ((locals.var_esat_1_dn11 * locals.var_pparam_b4soileff) + (locals.var_esat_1 * locals.var_pparam_b4soileff_dn11)))) / (assign33660_e27995 * assign33660_e27995)), (((locals.var_b4soivgsteff_dn12 * assign33660_e27995) - (locals.var_b4soivgsteff * ((locals.var_esat_1_dn12 * locals.var_pparam_b4soileff) + (locals.var_esat_1 * locals.var_pparam_b4soileff_dn12)))) / (assign33660_e27995 * assign33660_e27995)),)
    } else {
        (locals.var_t5__blk1149, locals.var_t5__blk1149_dn3, locals.var_t5__blk1149_dn4, locals.var_t5__blk1149_dn5, locals.var_t5__blk1149_dn6, locals.var_t5__blk1149_dn7, locals.var_t5__blk1149_dn8, locals.var_t5__blk1149_dn9, locals.var_t5__blk1149_dn10, locals.var_t5__blk1149_dn11, locals.var_t5__blk1149_dn12,)
    }
};
        locals.var_t5__blk1149 = assign33660_e27998;
        locals.var_t5__blk1149_dn3 = assign33660_e27998_d_n3;
        locals.var_t5__blk1149_dn4 = assign33660_e27998_d_n4;
        locals.var_t5__blk1149_dn5 = assign33660_e27998_d_n5;
        locals.var_t5__blk1149_dn6 = assign33660_e27998_d_n6;
        locals.var_t5__blk1149_dn7 = assign33660_e27998_d_n7;
        locals.var_t5__blk1149_dn8 = assign33660_e27998_d_n8;
        locals.var_t5__blk1149_dn9 = assign33660_e27998_d_n9;
        locals.var_t5__blk1149_dn10 = assign33660_e27998_d_n10;
        locals.var_t5__blk1149_dn11 = assign33660_e27998_d_n11;
        locals.var_t5__blk1149_dn12 = assign33660_e27998_d_n12;

        let (assign33670_e28007, assign33670_e28007_d_n3, assign33670_e28007_d_n4, assign33670_e28007_d_n5, assign33670_e28007_d_n6, assign33670_e28007_d_n7, assign33670_e28007_d_n8, assign33670_e28007_d_n9, assign33670_e28007_d_n10, assign33670_e28007_d_n11, assign33670_e28007_d_n12,) = {
    if ((locals.var_guard1789 != 0.0) && (locals.var_guard1788 == 0.0)) {
        let assign33670_e28005: f64 = (locals.var_t5__blk1149 * locals.var_t5__blk1149);
        (assign33670_e28005, ((locals.var_t5__blk1149_dn3 * locals.var_t5__blk1149) + (locals.var_t5__blk1149 * locals.var_t5__blk1149_dn3)), ((locals.var_t5__blk1149_dn4 * locals.var_t5__blk1149) + (locals.var_t5__blk1149 * locals.var_t5__blk1149_dn4)), ((locals.var_t5__blk1149_dn5 * locals.var_t5__blk1149) + (locals.var_t5__blk1149 * locals.var_t5__blk1149_dn5)), ((locals.var_t5__blk1149_dn6 * locals.var_t5__blk1149) + (locals.var_t5__blk1149 * locals.var_t5__blk1149_dn6)), ((locals.var_t5__blk1149_dn7 * locals.var_t5__blk1149) + (locals.var_t5__blk1149 * locals.var_t5__blk1149_dn7)), ((locals.var_t5__blk1149_dn8 * locals.var_t5__blk1149) + (locals.var_t5__blk1149 * locals.var_t5__blk1149_dn8)), ((locals.var_t5__blk1149_dn9 * locals.var_t5__blk1149) + (locals.var_t5__blk1149 * locals.var_t5__blk1149_dn9)), ((locals.var_t5__blk1149_dn10 * locals.var_t5__blk1149) + (locals.var_t5__blk1149 * locals.var_t5__blk1149_dn10)), ((locals.var_t5__blk1149_dn11 * locals.var_t5__blk1149) + (locals.var_t5__blk1149 * locals.var_t5__blk1149_dn11)), ((locals.var_t5__blk1149_dn12 * locals.var_t5__blk1149) + (locals.var_t5__blk1149 * locals.var_t5__blk1149_dn12)),)
    } else {
        (locals.var_t5__blk1149, locals.var_t5__blk1149_dn3, locals.var_t5__blk1149_dn4, locals.var_t5__blk1149_dn5, locals.var_t5__blk1149_dn6, locals.var_t5__blk1149_dn7, locals.var_t5__blk1149_dn8, locals.var_t5__blk1149_dn9, locals.var_t5__blk1149_dn10, locals.var_t5__blk1149_dn11, locals.var_t5__blk1149_dn12,)
    }
};
        locals.var_t5__blk1149 = assign33670_e28007;
        locals.var_t5__blk1149_dn3 = assign33670_e28007_d_n3;
        locals.var_t5__blk1149_dn4 = assign33670_e28007_d_n4;
        locals.var_t5__blk1149_dn5 = assign33670_e28007_d_n5;
        locals.var_t5__blk1149_dn6 = assign33670_e28007_d_n6;
        locals.var_t5__blk1149_dn7 = assign33670_e28007_d_n7;
        locals.var_t5__blk1149_dn8 = assign33670_e28007_d_n8;
        locals.var_t5__blk1149_dn9 = assign33670_e28007_d_n9;
        locals.var_t5__blk1149_dn10 = assign33670_e28007_d_n10;
        locals.var_t5__blk1149_dn11 = assign33670_e28007_d_n11;
        locals.var_t5__blk1149_dn12 = assign33670_e28007_d_n12;

        let (assign33680_e28022, assign33680_e28022_d_n3, assign33680_e28022_d_n4, assign33680_e28022_d_n5, assign33680_e28022_d_n6, assign33680_e28022_d_n7, assign33680_e28022_d_n8, assign33680_e28022_d_n9, assign33680_e28022_d_n10, assign33680_e28022_d_n11, assign33680_e28022_d_n12,) = {
    if ((locals.var_guard1789 != 0.0) && (locals.var_guard1788 == 0.0)) {
        let assign33680_e28016: f64 = (locals.var_t5__blk1149 * locals.var_b4soitnoia);
        let assign33680_e28018: f64 = (assign33680_e28016 * locals.var_pparam_b4soileff);
        let assign33680_e28019: f64 = (1.0 + assign33680_e28018);
        let assign33680_e28020: f64 = (locals.var_b4soirnoia * assign33680_e28019);
        (assign33680_e28020, (locals.var_b4soirnoia * (((locals.var_t5__blk1149_dn3 * locals.var_b4soitnoia) * locals.var_pparam_b4soileff) + (assign33680_e28016 * locals.var_pparam_b4soileff_dn3))), (locals.var_b4soirnoia * (((locals.var_t5__blk1149_dn4 * locals.var_b4soitnoia) * locals.var_pparam_b4soileff) + (assign33680_e28016 * locals.var_pparam_b4soileff_dn4))), (locals.var_b4soirnoia * (((locals.var_t5__blk1149_dn5 * locals.var_b4soitnoia) * locals.var_pparam_b4soileff) + (assign33680_e28016 * locals.var_pparam_b4soileff_dn5))), (locals.var_b4soirnoia * (((locals.var_t5__blk1149_dn6 * locals.var_b4soitnoia) * locals.var_pparam_b4soileff) + (assign33680_e28016 * locals.var_pparam_b4soileff_dn6))), (locals.var_b4soirnoia * (((locals.var_t5__blk1149_dn7 * locals.var_b4soitnoia) * locals.var_pparam_b4soileff) + (assign33680_e28016 * locals.var_pparam_b4soileff_dn7))), (locals.var_b4soirnoia * (((locals.var_t5__blk1149_dn8 * locals.var_b4soitnoia) * locals.var_pparam_b4soileff) + (assign33680_e28016 * locals.var_pparam_b4soileff_dn8))), (locals.var_b4soirnoia * (((locals.var_t5__blk1149_dn9 * locals.var_b4soitnoia) * locals.var_pparam_b4soileff) + (assign33680_e28016 * locals.var_pparam_b4soileff_dn9))), (locals.var_b4soirnoia * (((locals.var_t5__blk1149_dn10 * locals.var_b4soitnoia) * locals.var_pparam_b4soileff) + (assign33680_e28016 * locals.var_pparam_b4soileff_dn10))), (locals.var_b4soirnoia * (((locals.var_t5__blk1149_dn11 * locals.var_b4soitnoia) * locals.var_pparam_b4soileff) + (assign33680_e28016 * locals.var_pparam_b4soileff_dn11))), (locals.var_b4soirnoia * (((locals.var_t5__blk1149_dn12 * locals.var_b4soitnoia) * locals.var_pparam_b4soileff) + (assign33680_e28016 * locals.var_pparam_b4soileff_dn12))),)
    } else {
        (locals.var_npart_beta, locals.var_npart_beta_dn3, locals.var_npart_beta_dn4, locals.var_npart_beta_dn5, locals.var_npart_beta_dn6, locals.var_npart_beta_dn7, locals.var_npart_beta_dn8, locals.var_npart_beta_dn9, locals.var_npart_beta_dn10, locals.var_npart_beta_dn11, locals.var_npart_beta_dn12,)
    }
};
        locals.var_npart_beta = assign33680_e28022;
        locals.var_npart_beta_dn3 = assign33680_e28022_d_n3;
        locals.var_npart_beta_dn4 = assign33680_e28022_d_n4;
        locals.var_npart_beta_dn5 = assign33680_e28022_d_n5;
        locals.var_npart_beta_dn6 = assign33680_e28022_d_n6;
        locals.var_npart_beta_dn7 = assign33680_e28022_d_n7;
        locals.var_npart_beta_dn8 = assign33680_e28022_d_n8;
        locals.var_npart_beta_dn9 = assign33680_e28022_d_n9;
        locals.var_npart_beta_dn10 = assign33680_e28022_d_n10;
        locals.var_npart_beta_dn11 = assign33680_e28022_d_n11;
        locals.var_npart_beta_dn12 = assign33680_e28022_d_n12;

        let (assign33750_e28091, assign33750_e28091_d_n3, assign33750_e28091_d_n4, assign33750_e28091_d_n5, assign33750_e28091_d_n6, assign33750_e28091_d_n7, assign33750_e28091_d_n8, assign33750_e28091_d_n9, assign33750_e28091_d_n10, assign33750_e28091_d_n11, assign33750_e28091_d_n12,) = {
    if ((locals.var_guard1789 != 0.0) && (locals.var_guard1788 == 0.0)) {
        let assign33750_e28086: f64 = (locals.var_b4soigm + locals.var_b4soigmbs);
        let assign33750_e28087: f64 = (locals.var_npart_beta * assign33750_e28086);
        let assign33750_e28089: f64 = (assign33750_e28087 + locals.var_b4soigds);
        (assign33750_e28089, (((locals.var_npart_beta_dn3 * assign33750_e28086) + (locals.var_npart_beta * (locals.var_b4soigm_dn3 + locals.var_b4soigmbs_dn3))) + locals.var_b4soigds_dn3), (((locals.var_npart_beta_dn4 * assign33750_e28086) + (locals.var_npart_beta * (locals.var_b4soigm_dn4 + locals.var_b4soigmbs_dn4))) + locals.var_b4soigds_dn4), (((locals.var_npart_beta_dn5 * assign33750_e28086) + (locals.var_npart_beta * (locals.var_b4soigm_dn5 + locals.var_b4soigmbs_dn5))) + locals.var_b4soigds_dn5), (((locals.var_npart_beta_dn6 * assign33750_e28086) + (locals.var_npart_beta * (locals.var_b4soigm_dn6 + locals.var_b4soigmbs_dn6))) + locals.var_b4soigds_dn6), (((locals.var_npart_beta_dn7 * assign33750_e28086) + (locals.var_npart_beta * (locals.var_b4soigm_dn7 + locals.var_b4soigmbs_dn7))) + locals.var_b4soigds_dn7), (((locals.var_npart_beta_dn8 * assign33750_e28086) + (locals.var_npart_beta * (locals.var_b4soigm_dn8 + locals.var_b4soigmbs_dn8))) + locals.var_b4soigds_dn8), (((locals.var_npart_beta_dn9 * assign33750_e28086) + (locals.var_npart_beta * (locals.var_b4soigm_dn9 + locals.var_b4soigmbs_dn9))) + locals.var_b4soigds_dn9), (((locals.var_npart_beta_dn10 * assign33750_e28086) + (locals.var_npart_beta * (locals.var_b4soigm_dn10 + locals.var_b4soigmbs_dn10))) + locals.var_b4soigds_dn10), (((locals.var_npart_beta_dn11 * assign33750_e28086) + (locals.var_npart_beta * (locals.var_b4soigm_dn11 + locals.var_b4soigmbs_dn11))) + locals.var_b4soigds_dn11), (((locals.var_npart_beta_dn12 * assign33750_e28086) + (locals.var_npart_beta * (locals.var_b4soigm_dn12 + locals.var_b4soigmbs_dn12))) + locals.var_b4soigds_dn12),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign33750_e28091;
        locals.var_t1__blk1145_dn3 = assign33750_e28091_d_n3;
        locals.var_t1__blk1145_dn4 = assign33750_e28091_d_n4;
        locals.var_t1__blk1145_dn5 = assign33750_e28091_d_n5;
        locals.var_t1__blk1145_dn6 = assign33750_e28091_d_n6;
        locals.var_t1__blk1145_dn7 = assign33750_e28091_d_n7;
        locals.var_t1__blk1145_dn8 = assign33750_e28091_d_n8;
        locals.var_t1__blk1145_dn9 = assign33750_e28091_d_n9;
        locals.var_t1__blk1145_dn10 = assign33750_e28091_d_n10;
        locals.var_t1__blk1145_dn11 = assign33750_e28091_d_n11;
        locals.var_t1__blk1145_dn12 = assign33750_e28091_d_n12;

    }

    pub(super) fn stamp_transient_block_93(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign33760_e28102, assign33760_e28102_d_n3, assign33760_e28102_d_n4, assign33760_e28102_d_n5, assign33760_e28102_d_n6, assign33760_e28102_d_n7, assign33760_e28102_d_n8, assign33760_e28102_d_n9, assign33760_e28102_d_n10, assign33760_e28102_d_n11, assign33760_e28102_d_n12,) = {
    if ((locals.var_guard1789 != 0.0) && (locals.var_guard1788 == 0.0)) {
        let assign33760_e28098: f64 = (locals.var_t1__blk1145 * locals.var_t1__blk1145);
        let assign33760_e28100: f64 = (assign33760_e28098 / locals.var_b4soiidovvds);
        (assign33760_e28100, (((((locals.var_t1__blk1145_dn3 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn3)) * locals.var_b4soiidovvds) - (assign33760_e28098 * locals.var_b4soiidovvds_dn3)) / (locals.var_b4soiidovvds * locals.var_b4soiidovvds)), (((((locals.var_t1__blk1145_dn4 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn4)) * locals.var_b4soiidovvds) - (assign33760_e28098 * locals.var_b4soiidovvds_dn4)) / (locals.var_b4soiidovvds * locals.var_b4soiidovvds)), (((((locals.var_t1__blk1145_dn5 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn5)) * locals.var_b4soiidovvds) - (assign33760_e28098 * locals.var_b4soiidovvds_dn5)) / (locals.var_b4soiidovvds * locals.var_b4soiidovvds)), (((((locals.var_t1__blk1145_dn6 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn6)) * locals.var_b4soiidovvds) - (assign33760_e28098 * locals.var_b4soiidovvds_dn6)) / (locals.var_b4soiidovvds * locals.var_b4soiidovvds)), (((((locals.var_t1__blk1145_dn7 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn7)) * locals.var_b4soiidovvds) - (assign33760_e28098 * locals.var_b4soiidovvds_dn7)) / (locals.var_b4soiidovvds * locals.var_b4soiidovvds)), (((((locals.var_t1__blk1145_dn8 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn8)) * locals.var_b4soiidovvds) - (assign33760_e28098 * locals.var_b4soiidovvds_dn8)) / (locals.var_b4soiidovvds * locals.var_b4soiidovvds)), (((((locals.var_t1__blk1145_dn9 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn9)) * locals.var_b4soiidovvds) - (assign33760_e28098 * locals.var_b4soiidovvds_dn9)) / (locals.var_b4soiidovvds * locals.var_b4soiidovvds)), (((((locals.var_t1__blk1145_dn10 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn10)) * locals.var_b4soiidovvds) - (assign33760_e28098 * locals.var_b4soiidovvds_dn10)) / (locals.var_b4soiidovvds * locals.var_b4soiidovvds)), (((((locals.var_t1__blk1145_dn11 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn11)) * locals.var_b4soiidovvds) - (assign33760_e28098 * locals.var_b4soiidovvds_dn11)) / (locals.var_b4soiidovvds * locals.var_b4soiidovvds)), (((((locals.var_t1__blk1145_dn12 * locals.var_t1__blk1145) + (locals.var_t1__blk1145 * locals.var_t1__blk1145_dn12)) * locals.var_b4soiidovvds) - (assign33760_e28098 * locals.var_b4soiidovvds_dn12)) / (locals.var_b4soiidovvds * locals.var_b4soiidovvds)),)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign33760_e28102;
        locals.var_t2__blk1146_dn3 = assign33760_e28102_d_n3;
        locals.var_t2__blk1146_dn4 = assign33760_e28102_d_n4;
        locals.var_t2__blk1146_dn5 = assign33760_e28102_d_n5;
        locals.var_t2__blk1146_dn6 = assign33760_e28102_d_n6;
        locals.var_t2__blk1146_dn7 = assign33760_e28102_d_n7;
        locals.var_t2__blk1146_dn8 = assign33760_e28102_d_n8;
        locals.var_t2__blk1146_dn9 = assign33760_e28102_d_n9;
        locals.var_t2__blk1146_dn10 = assign33760_e28102_d_n10;
        locals.var_t2__blk1146_dn11 = assign33760_e28102_d_n11;
        locals.var_t2__blk1146_dn12 = assign33760_e28102_d_n12;

        let assign34300_e28632: f64 = if locals.var_b4soirdsmod != 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1825 = assign34300_e28632;

        let assign34310_e28635: f64 = if locals.var_b4soimode > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1826 = assign34310_e28635;

        let (assign34320_e28641, assign34320_e28641_d_n3, assign34320_e28641_d_n4, assign34320_e28641_d_n5, assign34320_e28641_d_n6, assign34320_e28641_d_n7, assign34320_e28641_d_n8, assign34320_e28641_d_n9, assign34320_e28641_d_n10, assign34320_e28641_d_n11, assign34320_e28641_d_n12,) = {
    if (locals.var_guard1826 != 0.0) {
        let assign34320_e28639: f64 = (locals.var_b4soitype * locals.var_igidl_1);
        (assign34320_e28639, (locals.var_b4soitype * locals.var_igidl_1_dn3), (locals.var_b4soitype * locals.var_igidl_1_dn4), (locals.var_b4soitype * locals.var_igidl_1_dn5), (locals.var_b4soitype * locals.var_igidl_1_dn6), (locals.var_b4soitype * locals.var_igidl_1_dn7), (locals.var_b4soitype * locals.var_igidl_1_dn8), (locals.var_b4soitype * locals.var_igidl_1_dn9), (locals.var_b4soitype * locals.var_igidl_1_dn10), (locals.var_b4soitype * locals.var_igidl_1_dn11), (locals.var_b4soitype * locals.var_igidl_1_dn12),)
    } else {
        (locals.var_b4soiigidl, locals.var_b4soiigidl_dn3, locals.var_b4soiigidl_dn4, locals.var_b4soiigidl_dn5, locals.var_b4soiigidl_dn6, locals.var_b4soiigidl_dn7, locals.var_b4soiigidl_dn8, locals.var_b4soiigidl_dn9, locals.var_b4soiigidl_dn10, locals.var_b4soiigidl_dn11, locals.var_b4soiigidl_dn12,)
    }
};
        locals.var_b4soiigidl = assign34320_e28641;
        locals.var_b4soiigidl_dn3 = assign34320_e28641_d_n3;
        locals.var_b4soiigidl_dn4 = assign34320_e28641_d_n4;
        locals.var_b4soiigidl_dn5 = assign34320_e28641_d_n5;
        locals.var_b4soiigidl_dn6 = assign34320_e28641_d_n6;
        locals.var_b4soiigidl_dn7 = assign34320_e28641_d_n7;
        locals.var_b4soiigidl_dn8 = assign34320_e28641_d_n8;
        locals.var_b4soiigidl_dn9 = assign34320_e28641_d_n9;
        locals.var_b4soiigidl_dn10 = assign34320_e28641_d_n10;
        locals.var_b4soiigidl_dn11 = assign34320_e28641_d_n11;
        locals.var_b4soiigidl_dn12 = assign34320_e28641_d_n12;

        let (assign34330_e28647, assign34330_e28647_d_n3, assign34330_e28647_d_n4, assign34330_e28647_d_n5, assign34330_e28647_d_n6, assign34330_e28647_d_n7, assign34330_e28647_d_n8, assign34330_e28647_d_n9, assign34330_e28647_d_n10, assign34330_e28647_d_n11, assign34330_e28647_d_n12,) = {
    if (locals.var_guard1826 != 0.0) {
        let assign34330_e28645: f64 = (locals.var_b4soitype * locals.var_igisl_1);
        (assign34330_e28645, (locals.var_b4soitype * locals.var_igisl_1_dn3), (locals.var_b4soitype * locals.var_igisl_1_dn4), (locals.var_b4soitype * locals.var_igisl_1_dn5), (locals.var_b4soitype * locals.var_igisl_1_dn6), (locals.var_b4soitype * locals.var_igisl_1_dn7), (locals.var_b4soitype * locals.var_igisl_1_dn8), (locals.var_b4soitype * locals.var_igisl_1_dn9), (locals.var_b4soitype * locals.var_igisl_1_dn10), (locals.var_b4soitype * locals.var_igisl_1_dn11), (locals.var_b4soitype * locals.var_igisl_1_dn12),)
    } else {
        (locals.var_b4soiigisl, locals.var_b4soiigisl_dn3, locals.var_b4soiigisl_dn4, locals.var_b4soiigisl_dn5, locals.var_b4soiigisl_dn6, locals.var_b4soiigisl_dn7, locals.var_b4soiigisl_dn8, locals.var_b4soiigisl_dn9, locals.var_b4soiigisl_dn10, locals.var_b4soiigisl_dn11, locals.var_b4soiigisl_dn12,)
    }
};
        locals.var_b4soiigisl = assign34330_e28647;
        locals.var_b4soiigisl_dn3 = assign34330_e28647_d_n3;
        locals.var_b4soiigisl_dn4 = assign34330_e28647_d_n4;
        locals.var_b4soiigisl_dn5 = assign34330_e28647_d_n5;
        locals.var_b4soiigisl_dn6 = assign34330_e28647_d_n6;
        locals.var_b4soiigisl_dn7 = assign34330_e28647_d_n7;
        locals.var_b4soiigisl_dn8 = assign34330_e28647_d_n8;
        locals.var_b4soiigisl_dn9 = assign34330_e28647_d_n9;
        locals.var_b4soiigisl_dn10 = assign34330_e28647_d_n10;
        locals.var_b4soiigisl_dn11 = assign34330_e28647_d_n11;
        locals.var_b4soiigisl_dn12 = assign34330_e28647_d_n12;

        let (assign34340_e28653, assign34340_e28653_d_n3, assign34340_e28653_d_n4, assign34340_e28653_d_n5, assign34340_e28653_d_n6, assign34340_e28653_d_n7, assign34340_e28653_d_n8, assign34340_e28653_d_n9, assign34340_e28653_d_n10, assign34340_e28653_d_n11, assign34340_e28653_d_n12,) = {
    if (locals.var_guard1826 != 0.0) {
        let assign34340_e28651: f64 = (locals.var_b4soitype * locals.var_igcd_1);
        (assign34340_e28651, (locals.var_b4soitype * locals.var_igcd_1_dn3), (locals.var_b4soitype * locals.var_igcd_1_dn4), (locals.var_b4soitype * locals.var_igcd_1_dn5), (locals.var_b4soitype * locals.var_igcd_1_dn6), (locals.var_b4soitype * locals.var_igcd_1_dn7), (locals.var_b4soitype * locals.var_igcd_1_dn8), (locals.var_b4soitype * locals.var_igcd_1_dn9), (locals.var_b4soitype * locals.var_igcd_1_dn10), (locals.var_b4soitype * locals.var_igcd_1_dn11), (locals.var_b4soitype * locals.var_igcd_1_dn12),)
    } else {
        (locals.var_b4soiigcd, locals.var_b4soiigcd_dn3, locals.var_b4soiigcd_dn4, locals.var_b4soiigcd_dn5, locals.var_b4soiigcd_dn6, locals.var_b4soiigcd_dn7, locals.var_b4soiigcd_dn8, locals.var_b4soiigcd_dn9, locals.var_b4soiigcd_dn10, locals.var_b4soiigcd_dn11, locals.var_b4soiigcd_dn12,)
    }
};
        locals.var_b4soiigcd = assign34340_e28653;
        locals.var_b4soiigcd_dn3 = assign34340_e28653_d_n3;
        locals.var_b4soiigcd_dn4 = assign34340_e28653_d_n4;
        locals.var_b4soiigcd_dn5 = assign34340_e28653_d_n5;
        locals.var_b4soiigcd_dn6 = assign34340_e28653_d_n6;
        locals.var_b4soiigcd_dn7 = assign34340_e28653_d_n7;
        locals.var_b4soiigcd_dn8 = assign34340_e28653_d_n8;
        locals.var_b4soiigcd_dn9 = assign34340_e28653_d_n9;
        locals.var_b4soiigcd_dn10 = assign34340_e28653_d_n10;
        locals.var_b4soiigcd_dn11 = assign34340_e28653_d_n11;
        locals.var_b4soiigcd_dn12 = assign34340_e28653_d_n12;

        let (assign34350_e28659, assign34350_e28659_d_n3, assign34350_e28659_d_n4, assign34350_e28659_d_n5, assign34350_e28659_d_n6, assign34350_e28659_d_n7, assign34350_e28659_d_n8, assign34350_e28659_d_n9, assign34350_e28659_d_n10, assign34350_e28659_d_n11, assign34350_e28659_d_n12,) = {
    if (locals.var_guard1826 != 0.0) {
        let assign34350_e28657: f64 = (locals.var_b4soitype * locals.var_igcs_1);
        (assign34350_e28657, (locals.var_b4soitype * locals.var_igcs_1_dn3), (locals.var_b4soitype * locals.var_igcs_1_dn4), (locals.var_b4soitype * locals.var_igcs_1_dn5), (locals.var_b4soitype * locals.var_igcs_1_dn6), (locals.var_b4soitype * locals.var_igcs_1_dn7), (locals.var_b4soitype * locals.var_igcs_1_dn8), (locals.var_b4soitype * locals.var_igcs_1_dn9), (locals.var_b4soitype * locals.var_igcs_1_dn10), (locals.var_b4soitype * locals.var_igcs_1_dn11), (locals.var_b4soitype * locals.var_igcs_1_dn12),)
    } else {
        (locals.var_b4soiigcs, locals.var_b4soiigcs_dn3, locals.var_b4soiigcs_dn4, locals.var_b4soiigcs_dn5, locals.var_b4soiigcs_dn6, locals.var_b4soiigcs_dn7, locals.var_b4soiigcs_dn8, locals.var_b4soiigcs_dn9, locals.var_b4soiigcs_dn10, locals.var_b4soiigcs_dn11, locals.var_b4soiigcs_dn12,)
    }
};
        locals.var_b4soiigcs = assign34350_e28659;
        locals.var_b4soiigcs_dn3 = assign34350_e28659_d_n3;
        locals.var_b4soiigcs_dn4 = assign34350_e28659_d_n4;
        locals.var_b4soiigcs_dn5 = assign34350_e28659_d_n5;
        locals.var_b4soiigcs_dn6 = assign34350_e28659_d_n6;
        locals.var_b4soiigcs_dn7 = assign34350_e28659_d_n7;
        locals.var_b4soiigcs_dn8 = assign34350_e28659_d_n8;
        locals.var_b4soiigcs_dn9 = assign34350_e28659_d_n9;
        locals.var_b4soiigcs_dn10 = assign34350_e28659_d_n10;
        locals.var_b4soiigcs_dn11 = assign34350_e28659_d_n11;
        locals.var_b4soiigcs_dn12 = assign34350_e28659_d_n12;

        let (assign34360_e28665, assign34360_e28665_d_n3, assign34360_e28665_d_n4, assign34360_e28665_d_n5, assign34360_e28665_d_n6, assign34360_e28665_d_n7, assign34360_e28665_d_n8, assign34360_e28665_d_n9, assign34360_e28665_d_n10, assign34360_e28665_d_n11, assign34360_e28665_d_n12,) = {
    if (locals.var_guard1826 != 0.0) {
        let assign34360_e28663: f64 = (locals.var_b4soitype * locals.var_qdrn);
        (assign34360_e28663, (locals.var_b4soitype * locals.var_qdrn_dn3), (locals.var_b4soitype * locals.var_qdrn_dn4), (locals.var_b4soitype * locals.var_qdrn_dn5), (locals.var_b4soitype * locals.var_qdrn_dn6), (locals.var_b4soitype * locals.var_qdrn_dn7), (locals.var_b4soitype * locals.var_qdrn_dn8), (locals.var_b4soitype * locals.var_qdrn_dn9), (locals.var_b4soitype * locals.var_qdrn_dn10), (locals.var_b4soitype * locals.var_qdrn_dn11), (locals.var_b4soitype * locals.var_qdrn_dn12),)
    } else {
        (locals.var_b4soiqdrn, locals.var_b4soiqdrn_dn3, locals.var_b4soiqdrn_dn4, locals.var_b4soiqdrn_dn5, locals.var_b4soiqdrn_dn6, locals.var_b4soiqdrn_dn7, locals.var_b4soiqdrn_dn8, locals.var_b4soiqdrn_dn9, locals.var_b4soiqdrn_dn10, locals.var_b4soiqdrn_dn11, locals.var_b4soiqdrn_dn12,)
    }
};
        locals.var_b4soiqdrn = assign34360_e28665;
        locals.var_b4soiqdrn_dn3 = assign34360_e28665_d_n3;
        locals.var_b4soiqdrn_dn4 = assign34360_e28665_d_n4;
        locals.var_b4soiqdrn_dn5 = assign34360_e28665_d_n5;
        locals.var_b4soiqdrn_dn6 = assign34360_e28665_d_n6;
        locals.var_b4soiqdrn_dn7 = assign34360_e28665_d_n7;
        locals.var_b4soiqdrn_dn8 = assign34360_e28665_d_n8;
        locals.var_b4soiqdrn_dn9 = assign34360_e28665_d_n9;
        locals.var_b4soiqdrn_dn10 = assign34360_e28665_d_n10;
        locals.var_b4soiqdrn_dn11 = assign34360_e28665_d_n11;
        locals.var_b4soiqdrn_dn12 = assign34360_e28665_d_n12;

        let (assign34370_e28671, assign34370_e28671_d_n3, assign34370_e28671_d_n4, assign34370_e28671_d_n5, assign34370_e28671_d_n6, assign34370_e28671_d_n7, assign34370_e28671_d_n8, assign34370_e28671_d_n9, assign34370_e28671_d_n10, assign34370_e28671_d_n11, assign34370_e28671_d_n12,) = {
    if (locals.var_guard1826 != 0.0) {
        let assign34370_e28669: f64 = (locals.var_b4soitype * locals.var_qsrc);
        (assign34370_e28669, (locals.var_b4soitype * locals.var_qsrc_dn3), (locals.var_b4soitype * locals.var_qsrc_dn4), (locals.var_b4soitype * locals.var_qsrc_dn5), (locals.var_b4soitype * locals.var_qsrc_dn6), (locals.var_b4soitype * locals.var_qsrc_dn7), (locals.var_b4soitype * locals.var_qsrc_dn8), (locals.var_b4soitype * locals.var_qsrc_dn9), (locals.var_b4soitype * locals.var_qsrc_dn10), (locals.var_b4soitype * locals.var_qsrc_dn11), (locals.var_b4soitype * locals.var_qsrc_dn12),)
    } else {
        (locals.var_b4soiqsrc, locals.var_b4soiqsrc_dn3, locals.var_b4soiqsrc_dn4, locals.var_b4soiqsrc_dn5, locals.var_b4soiqsrc_dn6, locals.var_b4soiqsrc_dn7, locals.var_b4soiqsrc_dn8, locals.var_b4soiqsrc_dn9, locals.var_b4soiqsrc_dn10, locals.var_b4soiqsrc_dn11, locals.var_b4soiqsrc_dn12,)
    }
};
        locals.var_b4soiqsrc = assign34370_e28671;
        locals.var_b4soiqsrc_dn3 = assign34370_e28671_d_n3;
        locals.var_b4soiqsrc_dn4 = assign34370_e28671_d_n4;
        locals.var_b4soiqsrc_dn5 = assign34370_e28671_d_n5;
        locals.var_b4soiqsrc_dn6 = assign34370_e28671_d_n6;
        locals.var_b4soiqsrc_dn7 = assign34370_e28671_d_n7;
        locals.var_b4soiqsrc_dn8 = assign34370_e28671_d_n8;
        locals.var_b4soiqsrc_dn9 = assign34370_e28671_d_n9;
        locals.var_b4soiqsrc_dn10 = assign34370_e28671_d_n10;
        locals.var_b4soiqsrc_dn11 = assign34370_e28671_d_n11;
        locals.var_b4soiqsrc_dn12 = assign34370_e28671_d_n12;

        let (assign34380_e28678, assign34380_e28678_d_n3, assign34380_e28678_d_n4, assign34380_e28678_d_n5, assign34380_e28678_d_n6, assign34380_e28678_d_n7, assign34380_e28678_d_n8, assign34380_e28678_d_n9, assign34380_e28678_d_n10, assign34380_e28678_d_n11, assign34380_e28678_d_n12,) = {
    if (locals.var_guard1826 == 0.0) {
        let assign34380_e28676: f64 = (locals.var_b4soitype * locals.var_igidl_1);
        (assign34380_e28676, (locals.var_b4soitype * locals.var_igidl_1_dn3), (locals.var_b4soitype * locals.var_igidl_1_dn4), (locals.var_b4soitype * locals.var_igidl_1_dn5), (locals.var_b4soitype * locals.var_igidl_1_dn6), (locals.var_b4soitype * locals.var_igidl_1_dn7), (locals.var_b4soitype * locals.var_igidl_1_dn8), (locals.var_b4soitype * locals.var_igidl_1_dn9), (locals.var_b4soitype * locals.var_igidl_1_dn10), (locals.var_b4soitype * locals.var_igidl_1_dn11), (locals.var_b4soitype * locals.var_igidl_1_dn12),)
    } else {
        (locals.var_b4soiigisl, locals.var_b4soiigisl_dn3, locals.var_b4soiigisl_dn4, locals.var_b4soiigisl_dn5, locals.var_b4soiigisl_dn6, locals.var_b4soiigisl_dn7, locals.var_b4soiigisl_dn8, locals.var_b4soiigisl_dn9, locals.var_b4soiigisl_dn10, locals.var_b4soiigisl_dn11, locals.var_b4soiigisl_dn12,)
    }
};
        locals.var_b4soiigisl = assign34380_e28678;
        locals.var_b4soiigisl_dn3 = assign34380_e28678_d_n3;
        locals.var_b4soiigisl_dn4 = assign34380_e28678_d_n4;
        locals.var_b4soiigisl_dn5 = assign34380_e28678_d_n5;
        locals.var_b4soiigisl_dn6 = assign34380_e28678_d_n6;
        locals.var_b4soiigisl_dn7 = assign34380_e28678_d_n7;
        locals.var_b4soiigisl_dn8 = assign34380_e28678_d_n8;
        locals.var_b4soiigisl_dn9 = assign34380_e28678_d_n9;
        locals.var_b4soiigisl_dn10 = assign34380_e28678_d_n10;
        locals.var_b4soiigisl_dn11 = assign34380_e28678_d_n11;
        locals.var_b4soiigisl_dn12 = assign34380_e28678_d_n12;

        let (assign34390_e28685, assign34390_e28685_d_n3, assign34390_e28685_d_n4, assign34390_e28685_d_n5, assign34390_e28685_d_n6, assign34390_e28685_d_n7, assign34390_e28685_d_n8, assign34390_e28685_d_n9, assign34390_e28685_d_n10, assign34390_e28685_d_n11, assign34390_e28685_d_n12,) = {
    if (locals.var_guard1826 == 0.0) {
        let assign34390_e28683: f64 = (locals.var_b4soitype * locals.var_igisl_1);
        (assign34390_e28683, (locals.var_b4soitype * locals.var_igisl_1_dn3), (locals.var_b4soitype * locals.var_igisl_1_dn4), (locals.var_b4soitype * locals.var_igisl_1_dn5), (locals.var_b4soitype * locals.var_igisl_1_dn6), (locals.var_b4soitype * locals.var_igisl_1_dn7), (locals.var_b4soitype * locals.var_igisl_1_dn8), (locals.var_b4soitype * locals.var_igisl_1_dn9), (locals.var_b4soitype * locals.var_igisl_1_dn10), (locals.var_b4soitype * locals.var_igisl_1_dn11), (locals.var_b4soitype * locals.var_igisl_1_dn12),)
    } else {
        (locals.var_b4soiigidl, locals.var_b4soiigidl_dn3, locals.var_b4soiigidl_dn4, locals.var_b4soiigidl_dn5, locals.var_b4soiigidl_dn6, locals.var_b4soiigidl_dn7, locals.var_b4soiigidl_dn8, locals.var_b4soiigidl_dn9, locals.var_b4soiigidl_dn10, locals.var_b4soiigidl_dn11, locals.var_b4soiigidl_dn12,)
    }
};
        locals.var_b4soiigidl = assign34390_e28685;
        locals.var_b4soiigidl_dn3 = assign34390_e28685_d_n3;
        locals.var_b4soiigidl_dn4 = assign34390_e28685_d_n4;
        locals.var_b4soiigidl_dn5 = assign34390_e28685_d_n5;
        locals.var_b4soiigidl_dn6 = assign34390_e28685_d_n6;
        locals.var_b4soiigidl_dn7 = assign34390_e28685_d_n7;
        locals.var_b4soiigidl_dn8 = assign34390_e28685_d_n8;
        locals.var_b4soiigidl_dn9 = assign34390_e28685_d_n9;
        locals.var_b4soiigidl_dn10 = assign34390_e28685_d_n10;
        locals.var_b4soiigidl_dn11 = assign34390_e28685_d_n11;
        locals.var_b4soiigidl_dn12 = assign34390_e28685_d_n12;

        let (assign34400_e28692, assign34400_e28692_d_n3, assign34400_e28692_d_n4, assign34400_e28692_d_n5, assign34400_e28692_d_n6, assign34400_e28692_d_n7, assign34400_e28692_d_n8, assign34400_e28692_d_n9, assign34400_e28692_d_n10, assign34400_e28692_d_n11, assign34400_e28692_d_n12,) = {
    if (locals.var_guard1826 == 0.0) {
        let assign34400_e28690: f64 = (locals.var_b4soitype * locals.var_igcd_1);
        (assign34400_e28690, (locals.var_b4soitype * locals.var_igcd_1_dn3), (locals.var_b4soitype * locals.var_igcd_1_dn4), (locals.var_b4soitype * locals.var_igcd_1_dn5), (locals.var_b4soitype * locals.var_igcd_1_dn6), (locals.var_b4soitype * locals.var_igcd_1_dn7), (locals.var_b4soitype * locals.var_igcd_1_dn8), (locals.var_b4soitype * locals.var_igcd_1_dn9), (locals.var_b4soitype * locals.var_igcd_1_dn10), (locals.var_b4soitype * locals.var_igcd_1_dn11), (locals.var_b4soitype * locals.var_igcd_1_dn12),)
    } else {
        (locals.var_b4soiigcs, locals.var_b4soiigcs_dn3, locals.var_b4soiigcs_dn4, locals.var_b4soiigcs_dn5, locals.var_b4soiigcs_dn6, locals.var_b4soiigcs_dn7, locals.var_b4soiigcs_dn8, locals.var_b4soiigcs_dn9, locals.var_b4soiigcs_dn10, locals.var_b4soiigcs_dn11, locals.var_b4soiigcs_dn12,)
    }
};
        locals.var_b4soiigcs = assign34400_e28692;
        locals.var_b4soiigcs_dn3 = assign34400_e28692_d_n3;
        locals.var_b4soiigcs_dn4 = assign34400_e28692_d_n4;
        locals.var_b4soiigcs_dn5 = assign34400_e28692_d_n5;
        locals.var_b4soiigcs_dn6 = assign34400_e28692_d_n6;
        locals.var_b4soiigcs_dn7 = assign34400_e28692_d_n7;
        locals.var_b4soiigcs_dn8 = assign34400_e28692_d_n8;
        locals.var_b4soiigcs_dn9 = assign34400_e28692_d_n9;
        locals.var_b4soiigcs_dn10 = assign34400_e28692_d_n10;
        locals.var_b4soiigcs_dn11 = assign34400_e28692_d_n11;
        locals.var_b4soiigcs_dn12 = assign34400_e28692_d_n12;

        let (assign34410_e28699, assign34410_e28699_d_n3, assign34410_e28699_d_n4, assign34410_e28699_d_n5, assign34410_e28699_d_n6, assign34410_e28699_d_n7, assign34410_e28699_d_n8, assign34410_e28699_d_n9, assign34410_e28699_d_n10, assign34410_e28699_d_n11, assign34410_e28699_d_n12,) = {
    if (locals.var_guard1826 == 0.0) {
        let assign34410_e28697: f64 = (locals.var_b4soitype * locals.var_igcs_1);
        (assign34410_e28697, (locals.var_b4soitype * locals.var_igcs_1_dn3), (locals.var_b4soitype * locals.var_igcs_1_dn4), (locals.var_b4soitype * locals.var_igcs_1_dn5), (locals.var_b4soitype * locals.var_igcs_1_dn6), (locals.var_b4soitype * locals.var_igcs_1_dn7), (locals.var_b4soitype * locals.var_igcs_1_dn8), (locals.var_b4soitype * locals.var_igcs_1_dn9), (locals.var_b4soitype * locals.var_igcs_1_dn10), (locals.var_b4soitype * locals.var_igcs_1_dn11), (locals.var_b4soitype * locals.var_igcs_1_dn12),)
    } else {
        (locals.var_b4soiigcd, locals.var_b4soiigcd_dn3, locals.var_b4soiigcd_dn4, locals.var_b4soiigcd_dn5, locals.var_b4soiigcd_dn6, locals.var_b4soiigcd_dn7, locals.var_b4soiigcd_dn8, locals.var_b4soiigcd_dn9, locals.var_b4soiigcd_dn10, locals.var_b4soiigcd_dn11, locals.var_b4soiigcd_dn12,)
    }
};
        locals.var_b4soiigcd = assign34410_e28699;
        locals.var_b4soiigcd_dn3 = assign34410_e28699_d_n3;
        locals.var_b4soiigcd_dn4 = assign34410_e28699_d_n4;
        locals.var_b4soiigcd_dn5 = assign34410_e28699_d_n5;
        locals.var_b4soiigcd_dn6 = assign34410_e28699_d_n6;
        locals.var_b4soiigcd_dn7 = assign34410_e28699_d_n7;
        locals.var_b4soiigcd_dn8 = assign34410_e28699_d_n8;
        locals.var_b4soiigcd_dn9 = assign34410_e28699_d_n9;
        locals.var_b4soiigcd_dn10 = assign34410_e28699_d_n10;
        locals.var_b4soiigcd_dn11 = assign34410_e28699_d_n11;
        locals.var_b4soiigcd_dn12 = assign34410_e28699_d_n12;

        let (assign34420_e28706, assign34420_e28706_d_n3, assign34420_e28706_d_n4, assign34420_e28706_d_n5, assign34420_e28706_d_n6, assign34420_e28706_d_n7, assign34420_e28706_d_n8, assign34420_e28706_d_n9, assign34420_e28706_d_n10, assign34420_e28706_d_n11, assign34420_e28706_d_n12,) = {
    if (locals.var_guard1826 == 0.0) {
        let assign34420_e28704: f64 = (locals.var_b4soitype * locals.var_qdrn);
        (assign34420_e28704, (locals.var_b4soitype * locals.var_qdrn_dn3), (locals.var_b4soitype * locals.var_qdrn_dn4), (locals.var_b4soitype * locals.var_qdrn_dn5), (locals.var_b4soitype * locals.var_qdrn_dn6), (locals.var_b4soitype * locals.var_qdrn_dn7), (locals.var_b4soitype * locals.var_qdrn_dn8), (locals.var_b4soitype * locals.var_qdrn_dn9), (locals.var_b4soitype * locals.var_qdrn_dn10), (locals.var_b4soitype * locals.var_qdrn_dn11), (locals.var_b4soitype * locals.var_qdrn_dn12),)
    } else {
        (locals.var_b4soiqsrc, locals.var_b4soiqsrc_dn3, locals.var_b4soiqsrc_dn4, locals.var_b4soiqsrc_dn5, locals.var_b4soiqsrc_dn6, locals.var_b4soiqsrc_dn7, locals.var_b4soiqsrc_dn8, locals.var_b4soiqsrc_dn9, locals.var_b4soiqsrc_dn10, locals.var_b4soiqsrc_dn11, locals.var_b4soiqsrc_dn12,)
    }
};
        locals.var_b4soiqsrc = assign34420_e28706;
        locals.var_b4soiqsrc_dn3 = assign34420_e28706_d_n3;
        locals.var_b4soiqsrc_dn4 = assign34420_e28706_d_n4;
        locals.var_b4soiqsrc_dn5 = assign34420_e28706_d_n5;
        locals.var_b4soiqsrc_dn6 = assign34420_e28706_d_n6;
        locals.var_b4soiqsrc_dn7 = assign34420_e28706_d_n7;
        locals.var_b4soiqsrc_dn8 = assign34420_e28706_d_n8;
        locals.var_b4soiqsrc_dn9 = assign34420_e28706_d_n9;
        locals.var_b4soiqsrc_dn10 = assign34420_e28706_d_n10;
        locals.var_b4soiqsrc_dn11 = assign34420_e28706_d_n11;
        locals.var_b4soiqsrc_dn12 = assign34420_e28706_d_n12;

        let (assign34430_e28713, assign34430_e28713_d_n3, assign34430_e28713_d_n4, assign34430_e28713_d_n5, assign34430_e28713_d_n6, assign34430_e28713_d_n7, assign34430_e28713_d_n8, assign34430_e28713_d_n9, assign34430_e28713_d_n10, assign34430_e28713_d_n11, assign34430_e28713_d_n12,) = {
    if (locals.var_guard1826 == 0.0) {
        let assign34430_e28711: f64 = (locals.var_b4soitype * locals.var_qsrc);
        (assign34430_e28711, (locals.var_b4soitype * locals.var_qsrc_dn3), (locals.var_b4soitype * locals.var_qsrc_dn4), (locals.var_b4soitype * locals.var_qsrc_dn5), (locals.var_b4soitype * locals.var_qsrc_dn6), (locals.var_b4soitype * locals.var_qsrc_dn7), (locals.var_b4soitype * locals.var_qsrc_dn8), (locals.var_b4soitype * locals.var_qsrc_dn9), (locals.var_b4soitype * locals.var_qsrc_dn10), (locals.var_b4soitype * locals.var_qsrc_dn11), (locals.var_b4soitype * locals.var_qsrc_dn12),)
    } else {
        (locals.var_b4soiqdrn, locals.var_b4soiqdrn_dn3, locals.var_b4soiqdrn_dn4, locals.var_b4soiqdrn_dn5, locals.var_b4soiqdrn_dn6, locals.var_b4soiqdrn_dn7, locals.var_b4soiqdrn_dn8, locals.var_b4soiqdrn_dn9, locals.var_b4soiqdrn_dn10, locals.var_b4soiqdrn_dn11, locals.var_b4soiqdrn_dn12,)
    }
};
        locals.var_b4soiqdrn = assign34430_e28713;
        locals.var_b4soiqdrn_dn3 = assign34430_e28713_d_n3;
        locals.var_b4soiqdrn_dn4 = assign34430_e28713_d_n4;
        locals.var_b4soiqdrn_dn5 = assign34430_e28713_d_n5;
        locals.var_b4soiqdrn_dn6 = assign34430_e28713_d_n6;
        locals.var_b4soiqdrn_dn7 = assign34430_e28713_d_n7;
        locals.var_b4soiqdrn_dn8 = assign34430_e28713_d_n8;
        locals.var_b4soiqdrn_dn9 = assign34430_e28713_d_n9;
        locals.var_b4soiqdrn_dn10 = assign34430_e28713_d_n10;
        locals.var_b4soiqdrn_dn11 = assign34430_e28713_d_n11;
        locals.var_b4soiqdrn_dn12 = assign34430_e28713_d_n12;

        let assign34440_e28716: f64 = (locals.var_b4soitype * locals.var_igd_1);
        locals.var_b4soiigd = assign34440_e28716;
        locals.var_b4soiigd_dn3 = (locals.var_b4soitype * locals.var_igd_1_dn3);
        locals.var_b4soiigd_dn4 = (locals.var_b4soitype * locals.var_igd_1_dn4);
        locals.var_b4soiigd_dn5 = (locals.var_b4soitype * locals.var_igd_1_dn5);
        locals.var_b4soiigd_dn6 = (locals.var_b4soitype * locals.var_igd_1_dn6);
        locals.var_b4soiigd_dn7 = (locals.var_b4soitype * locals.var_igd_1_dn7);
        locals.var_b4soiigd_dn8 = (locals.var_b4soitype * locals.var_igd_1_dn8);
        locals.var_b4soiigd_dn9 = (locals.var_b4soitype * locals.var_igd_1_dn9);
        locals.var_b4soiigd_dn10 = (locals.var_b4soitype * locals.var_igd_1_dn10);
        locals.var_b4soiigd_dn11 = (locals.var_b4soitype * locals.var_igd_1_dn11);
        locals.var_b4soiigd_dn12 = (locals.var_b4soitype * locals.var_igd_1_dn12);

        let assign34450_e28719: f64 = (locals.var_b4soitype * locals.var_igs_1);
        locals.var_b4soiigs = assign34450_e28719;
        locals.var_b4soiigs_dn3 = (locals.var_b4soitype * locals.var_igs_1_dn3);
        locals.var_b4soiigs_dn4 = (locals.var_b4soitype * locals.var_igs_1_dn4);
        locals.var_b4soiigs_dn5 = (locals.var_b4soitype * locals.var_igs_1_dn5);
        locals.var_b4soiigs_dn6 = (locals.var_b4soitype * locals.var_igs_1_dn6);
        locals.var_b4soiigs_dn7 = (locals.var_b4soitype * locals.var_igs_1_dn7);
        locals.var_b4soiigs_dn8 = (locals.var_b4soitype * locals.var_igs_1_dn8);
        locals.var_b4soiigs_dn9 = (locals.var_b4soitype * locals.var_igs_1_dn9);
        locals.var_b4soiigs_dn10 = (locals.var_b4soitype * locals.var_igs_1_dn10);
        locals.var_b4soiigs_dn11 = (locals.var_b4soitype * locals.var_igs_1_dn11);
        locals.var_b4soiigs_dn12 = (locals.var_b4soitype * locals.var_igs_1_dn12);

        let assign34460_e28726: f64 = if ((locals.var_b4soibodymod == 0.0) || (locals.var_b4soibodymod == 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard1827 = assign34460_e28726;

        let assign34470_e28729: f64 = if p.p37 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1828 = assign34470_e28729;

        let assign34490_e28743: f64 = if ((p.p37 == 0.0) || (p.p37 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1830 = assign34490_e28743;

        let assign34530_e28767: f64 = if ((p.p33 == 1.0) && (p.p16 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1834 = assign34530_e28767;

    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let assign00_e1482: f64 = ctx_temp;
        let assign00_e1484: f64 = (assign00_e1482 + p.p0);
        locals.var_devtemp = assign00_e1484;
        locals.var_devtemp_dn6 = 0.0;
        locals.var_devtemp_rv = 0.0;

        locals.var_b4soitype = p.p34;
        locals.var_b4soitype_rv = 0.0;

        locals.var_b4soil = p.p1;
        locals.var_b4soil_rv = 0.0;

        locals.var_b4soiw = p.p2;
        locals.var_b4soiw_rv = 0.0;

        locals.var_b4soinf = p.p3;
        locals.var_b4soinf_rv = 0.0;

        locals.var_b4soisa = p.p4;
        locals.var_b4soisa_rv = 0.0;

        locals.var_b4soisb = p.p5;
        locals.var_b4soisb_rv = 0.0;

        locals.var_b4soisd = p.p6;
        locals.var_b4soisd_rv = 0.0;

        locals.var_b4soidrainarea = p.p7;
        locals.var_b4soidrainarea_rv = 0.0;

        locals.var_b4soisourcearea = p.p8;
        locals.var_b4soisourcearea_rv = 0.0;

        locals.var_b4soidrainperimeter = p.p9;
        locals.var_b4soidrainperimeter_rv = 0.0;

        locals.var_b4soisourceperimeter = p.p10;
        locals.var_b4soisourceperimeter_rv = 0.0;

        locals.var_b4soidrainsquares = p.p11;
        locals.var_b4soidrainsquares_rv = 0.0;

        locals.var_b4soisourcesquares = p.p12;
        locals.var_b4soisourcesquares_rv = 0.0;

        locals.var_b4soibjtoff = p.p14;
        locals.var_b4soibjtoff_rv = 0.0;

        locals.var_b4soicth0 = p.p17;
        locals.var_b4soicth0_rv = 0.0;

        locals.var_b4soibodysquares = p.p18;
        locals.var_b4soibodysquares_rv = 0.0;

        locals.var_b4soifrbody = p.p19;
        locals.var_b4soifrbody_rv = 0.0;

        locals.var_b4soidelvto = p.p22;
        locals.var_b4soidelvto_rv = 0.0;

        locals.var_b4soisoimod = p.p23;
        locals.var_b4soisoimod_rv = 0.0;

        locals.var_b4soinbc = p.p24;
        locals.var_b4soinbc_rv = 0.0;

        locals.var_b4soinseg = p.p25;
        locals.var_b4soinseg_rv = 0.0;

        locals.var_b4soipdbcp = p.p26;
        locals.var_b4soipdbcp_rv = 0.0;

        locals.var_b4soipsbcp = p.p27;
        locals.var_b4soipsbcp_rv = 0.0;

        locals.var_b4soiagbcp = p.p28;
        locals.var_b4soiagbcp_rv = 0.0;

        locals.var_b4soiagbcp2 = p.p29;
        locals.var_b4soiagbcp2_rv = 0.0;

        locals.var_b4soiagbcpd = p.p30;
        locals.var_b4soiagbcpd_rv = 0.0;

        locals.var_b4soiaebcp = p.p31;
        locals.var_b4soiaebcp_rv = 0.0;

        locals.var_b4soirgatemod = p.p37;
        locals.var_b4soirgatemod_rv = 0.0;

        locals.var_b4soimtrlmod = p.p39;
        locals.var_b4soimtrlmod_rv = 0.0;

        locals.var_b4soivgstcvmod = p.p40;
        locals.var_b4soivgstcvmod_rv = 0.0;

        locals.var_b4soigidlmod = p.p41;
        locals.var_b4soigidlmod_rv = 0.0;

        locals.var_b4soiiiimod = p.p42;
        locals.var_b4soiiiimod_rv = 0.0;

        locals.var_b4soieot = p.p43;
        locals.var_b4soieot_rv = 0.0;

        locals.var_b4soiepsrox = p.p44;
        locals.var_b4soiepsrox_rv = 0.0;

        locals.var_b4soiepsrsub = p.p45;
        locals.var_b4soiepsrsub_rv = 0.0;

        locals.var_b4soini0sub = p.p46;
        locals.var_b4soini0sub_rv = 0.0;

        locals.var_b4soibg0sub = p.p47;
        locals.var_b4soibg0sub_rv = 0.0;

        locals.var_b4soitbgasub = p.p48;
        locals.var_b4soitbgasub_rv = 0.0;

        locals.var_b4soitbgbsub = p.p49;
        locals.var_b4soitbgbsub_rv = 0.0;

        locals.var_b4soiphig = p.p50;
        locals.var_b4soiphig_rv = 0.0;

        locals.var_b4soieasub = p.p51;
        locals.var_b4soieasub_rv = 0.0;

        locals.var_b4soileffeot = p.p52;
        locals.var_b4soileffeot_rv = 0.0;

        locals.var_b4soiweffeot = p.p53;
        locals.var_b4soiweffeot_rv = 0.0;

        locals.var_b4soivddeot = p.p54;
        locals.var_b4soivddeot_rv = 0.0;

        locals.var_b4soitempeot = p.p55;
        locals.var_b4soitempeot_rv = 0.0;

        locals.var_b4soiados = p.p56;
        locals.var_b4soiados_rv = 0.0;

        locals.var_b4soibdos = p.p57;
        locals.var_b4soibdos_rv = 0.0;

        locals.var_b4soiepsrgate = p.p58;
        locals.var_b4soiepsrgate_rv = 0.0;

        locals.var_b4soicapmod = p.p59;
        locals.var_b4soicapmod_rv = 0.0;

        locals.var_b4soimobmod = p.p60;
        locals.var_b4soimobmod_rv = 0.0;

        locals.var_b4soibinunit = p.p63;
        locals.var_b4soibinunit_rv = 0.0;

        locals.var_b4soitox = p.p64;
        locals.var_b4soitox_rv = 0.0;

        locals.var_b4soitoxm = p.p66;
        locals.var_b4soitoxm_rv = 0.0;

        locals.var_b4soidtoxcv = p.p67;
        locals.var_b4soidtoxcv_rv = 0.0;

        locals.var_b4soicdsc = p.p68;
        locals.var_b4soicdsc_rv = 0.0;

        locals.var_b4soicdscb = p.p69;
        locals.var_b4soicdscb_rv = 0.0;

        locals.var_b4soicdscd = p.p70;
        locals.var_b4soicdscd_rv = 0.0;

        locals.var_b4soicit = p.p71;
        locals.var_b4soicit_rv = 0.0;

        locals.var_b4soinfactor = p.p72;
        locals.var_b4soinfactor_rv = 0.0;

        locals.var_b4soivsat = p.p73;
        locals.var_b4soivsat_rv = 0.0;

        locals.var_b4soiat = p.p74;
        locals.var_b4soiat_rv = 0.0;

        locals.var_b4soia0 = p.p75;
        locals.var_b4soia0_rv = 0.0;

        locals.var_b4soiags = p.p76;
        locals.var_b4soiags_rv = 0.0;

        locals.var_b4soia1 = p.p77;
        locals.var_b4soia1_rv = 0.0;

        locals.var_b4soia2 = p.p78;
        locals.var_b4soia2_rv = 0.0;

        locals.var_b4soiketa = p.p79;
        locals.var_b4soiketa_rv = 0.0;

        locals.var_b4soinsub = p.p80;
        locals.var_b4soinsub_rv = 0.0;

        locals.var_b4soinpeak = p.p81;
        locals.var_b4soinpeak_rv = 0.0;

        locals.var_b4soingate = p.p82;
        locals.var_b4soingate_rv = 0.0;

        locals.var_b4soinsd = p.p83;
        locals.var_b4soinsd_rv = 0.0;

        locals.var_b4soigamma1 = p.p84;
        locals.var_b4soigamma1_rv = 0.0;

        locals.var_b4soigamma2 = p.p85;
        locals.var_b4soigamma2_rv = 0.0;

        locals.var_b4soivbx = p.p86;
        locals.var_b4soivbx_rv = 0.0;

        locals.var_b4soivbm = p.p87;
        locals.var_b4soivbm_rv = 0.0;

        locals.var_b4soixt = p.p88;
        locals.var_b4soixt_rv = 0.0;

        locals.var_b4soik1 = p.p89;
        locals.var_b4soik1_rv = 0.0;

        locals.var_b4soikt1 = p.p90;
        locals.var_b4soikt1_rv = 0.0;

        locals.var_b4soikt1l = p.p91;
        locals.var_b4soikt1l_rv = 0.0;

        locals.var_b4soikt2 = p.p92;
        locals.var_b4soikt2_rv = 0.0;

        locals.var_b4soik2 = p.p93;
        locals.var_b4soik2_rv = 0.0;

        locals.var_b4soik3 = p.p94;
        locals.var_b4soik3_rv = 0.0;

        locals.var_b4soik3b = p.p95;
        locals.var_b4soik3b_rv = 0.0;

        locals.var_b4soiw0 = p.p96;
        locals.var_b4soiw0_rv = 0.0;

        locals.var_b4soilpe0 = p.p973;
        locals.var_b4soilpe0_rv = 0.0;

        locals.var_b4soilpeb = p.p97;
        locals.var_b4soilpeb_rv = 0.0;

        locals.var_b4soidvt0 = p.p98;
        locals.var_b4soidvt0_rv = 0.0;

        locals.var_b4soidvt1 = p.p99;
        locals.var_b4soidvt1_rv = 0.0;

        locals.var_b4soidvt2 = p.p100;
        locals.var_b4soidvt2_rv = 0.0;

        locals.var_b4soidvt0w = p.p101;
        locals.var_b4soidvt0w_rv = 0.0;

        locals.var_b4soidvt1w = p.p102;
        locals.var_b4soidvt1w_rv = 0.0;

        locals.var_b4soidvt2w = p.p103;
        locals.var_b4soidvt2w_rv = 0.0;

        locals.var_b4soidrout = p.p104;
        locals.var_b4soidrout_rv = 0.0;

        locals.var_b4soidsub = p.p105;
        locals.var_b4soidsub_rv = 0.0;

        locals.var_b4soivth0 = p.p107;
        locals.var_b4soivth0_rv = 0.0;

        locals.var_b4soivfb = p.p108;
        locals.var_b4soivfb_rv = 0.0;

        locals.var_b4soiua = p.p109;
        locals.var_b4soiua_rv = 0.0;

        locals.var_b4soiua1 = p.p110;
        locals.var_b4soiua1_rv = 0.0;

        locals.var_b4soiub = p.p111;
        locals.var_b4soiub_rv = 0.0;

        locals.var_b4soiub1 = p.p112;
        locals.var_b4soiub1_rv = 0.0;

        locals.var_b4soiuc = p.p113;
        locals.var_b4soiuc_rv = 0.0;

        locals.var_b4soiuc1 = p.p114;
        locals.var_b4soiuc1_rv = 0.0;

        locals.var_b4soiu0 = p.p115;
        locals.var_b4soiu0_rv = 0.0;

        locals.var_b4soieu = p.p116;
        locals.var_b4soieu_rv = 0.0;

        locals.var_b4soiute = p.p117;
        locals.var_b4soiute_rv = 0.0;

        locals.var_b4soiucs = p.p118;
        locals.var_b4soiucs_rv = 0.0;

        locals.var_b4soiucste = p.p119;
        locals.var_b4soiucste_rv = 0.0;

        locals.var_b4soiud = p.p120;
        locals.var_b4soiud_rv = 0.0;

        locals.var_b4soiud1 = p.p121;
        locals.var_b4soiud1_rv = 0.0;

        locals.var_b4soivoff = p.p122;
        locals.var_b4soivoff_rv = 0.0;

        let assign1140_e1600: f64 = (p.p123 + 273.15);
        locals.var_b4soitnom = assign1140_e1600;
        locals.var_b4soitnom_rv = 0.0;

        locals.var_b4soixpart = p.p126;
        locals.var_b4soixpart_rv = 0.0;

        locals.var_b4soidelta = p.p127;
        locals.var_b4soidelta_rv = 0.0;

        locals.var_b4soisheetresistance = p.p128;
        locals.var_b4soisheetresistance_rv = 0.0;

        locals.var_b4soirdsw = p.p129;
        locals.var_b4soirdsw_rv = 0.0;

        locals.var_b4soirsw = p.p130;
        locals.var_b4soirsw_rv = 0.0;

        locals.var_b4soirdw = p.p131;
        locals.var_b4soirdw_rv = 0.0;

        locals.var_b4soirswmin = p.p132;
        locals.var_b4soirswmin_rv = 0.0;

        locals.var_b4soirdwmin = p.p133;
        locals.var_b4soirdwmin_rv = 0.0;

        locals.var_b4soiprwg = p.p134;
        locals.var_b4soiprwg_rv = 0.0;

        locals.var_b4soiprwb = p.p135;
        locals.var_b4soiprwb_rv = 0.0;

        locals.var_b4soiprt = p.p136;
        locals.var_b4soiprt_rv = 0.0;

        locals.var_b4soieta0 = p.p137;
        locals.var_b4soieta0_rv = 0.0;

        locals.var_b4soietab = p.p138;
        locals.var_b4soietab_rv = 0.0;

        locals.var_b4soieta0cv = p.p139;
        locals.var_b4soieta0cv_rv = 0.0;

        locals.var_b4soietabcv = p.p140;
        locals.var_b4soietabcv_rv = 0.0;

        locals.var_b4soipclm = p.p141;
        locals.var_b4soipclm_rv = 0.0;

        locals.var_b4soipdibl1 = p.p142;
        locals.var_b4soipdibl1_rv = 0.0;

        locals.var_b4soipdibl2 = p.p143;
        locals.var_b4soipdibl2_rv = 0.0;

        locals.var_b4soipdiblb = p.p144;
        locals.var_b4soipdiblb_rv = 0.0;

        locals.var_b4soipvag = p.p145;
        locals.var_b4soipvag_rv = 0.0;

        locals.var_b4soitbox = p.p146;
        locals.var_b4soitbox_rv = 0.0;

        locals.var_b4soitsi = p.p147;
        locals.var_b4soitsi_rv = 0.0;

        locals.var_b4soietsi = p.p148;
        locals.var_b4soietsi_rv = 0.0;

        locals.var_b4soixj = p.p149;
        locals.var_b4soixj_rv = 0.0;

        locals.var_b4soiegidl = p.p974;
        locals.var_b4soiegidl_rv = 0.0;

        locals.var_b4soiagidl = p.p150;
        locals.var_b4soiagidl_rv = 0.0;

        locals.var_b4soibgidl = p.p151;
        locals.var_b4soibgidl_rv = 0.0;

        locals.var_b4soicgidl = p.p152;
        locals.var_b4soicgidl_rv = 0.0;

        locals.var_b4soirgidl = p.p153;
        locals.var_b4soirgidl_rv = 0.0;

        locals.var_b4soikgidl = p.p154;
        locals.var_b4soikgidl_rv = 0.0;

        locals.var_b4soifgidl = p.p155;
        locals.var_b4soifgidl_rv = 0.0;

        locals.var_b4soiegisl = p.p975;
        locals.var_b4soiegisl_rv = 0.0;

        locals.var_b4soiagisl = p.p156;
        locals.var_b4soiagisl_rv = 0.0;

        locals.var_b4soibgisl = p.p157;
        locals.var_b4soibgisl_rv = 0.0;

        locals.var_b4soicgisl = p.p158;
        locals.var_b4soicgisl_rv = 0.0;

        locals.var_b4soirgisl = p.p159;
        locals.var_b4soirgisl_rv = 0.0;

        locals.var_b4soikgisl = p.p160;
        locals.var_b4soikgisl_rv = 0.0;

        locals.var_b4soifgisl = p.p161;
        locals.var_b4soifgisl_rv = 0.0;

        locals.var_b4soindiode = p.p162;
        locals.var_b4soindiode_rv = 0.0;

        locals.var_b4soindioded = p.p163;
        locals.var_b4soindioded_rv = 0.0;

        locals.var_b4soixbjt = p.p164;
        locals.var_b4soixbjt_rv = 0.0;

        locals.var_b4soixdif = p.p165;
        locals.var_b4soixdif_rv = 0.0;

        locals.var_b4soixrec = p.p166;
        locals.var_b4soixrec_rv = 0.0;

        locals.var_b4soixtun = p.p167;
        locals.var_b4soixtun_rv = 0.0;

        locals.var_b4soixdifd = p.p168;
        locals.var_b4soixdifd_rv = 0.0;

        locals.var_b4soixrecd = p.p169;
        locals.var_b4soixrecd_rv = 0.0;

        locals.var_b4soixtund = p.p170;
        locals.var_b4soixtund_rv = 0.0;

        locals.var_b4soigatesidewalljctspotential = p.p171;
        locals.var_b4soigatesidewalljctspotential_rv = 0.0;

        locals.var_b4soigatesidewalljctdpotential = p.p172;
        locals.var_b4soigatesidewalljctdpotential_rv = 0.0;

        locals.var_b4soibodyjctgatesidesgradingcoeff = p.p173;
        locals.var_b4soibodyjctgatesidesgradingcoeff_rv = 0.0;

        locals.var_b4soibodyjctgatesidedgradingcoeff = p.p174;
        locals.var_b4soibodyjctgatesidedgradingcoeff_rv = 0.0;

        locals.var_b4soiunitlengthgatesidewalljctcaps = p.p175;
        locals.var_b4soiunitlengthgatesidewalljctcaps_rv = 0.0;

        locals.var_b4soiunitlengthgatesidewalljctcapd = p.p176;
        locals.var_b4soiunitlengthgatesidewalljctcapd_rv = 0.0;

        locals.var_b4soilint = p.p177;
        locals.var_b4soilint_rv = 0.0;

        locals.var_b4soill = p.p178;
        locals.var_b4soill_rv = 0.0;

        locals.var_b4soillc = p.p179;
        locals.var_b4soillc_rv = 0.0;

        locals.var_b4soilln = p.p180;
        locals.var_b4soilln_rv = 0.0;

        locals.var_b4soilw = p.p181;
        locals.var_b4soilw_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_1(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        locals.var_b4soilwc = p.p182;
        locals.var_b4soilwc_rv = 0.0;

        locals.var_b4soilwn = p.p183;
        locals.var_b4soilwn_rv = 0.0;

        locals.var_b4soilwl = p.p184;
        locals.var_b4soilwl_rv = 0.0;

        locals.var_b4soilwlc = p.p185;
        locals.var_b4soilwlc_rv = 0.0;

        locals.var_b4soiwr = p.p186;
        locals.var_b4soiwr_rv = 0.0;

        locals.var_b4soiwint = p.p187;
        locals.var_b4soiwint_rv = 0.0;

        locals.var_b4soidwg = p.p188;
        locals.var_b4soidwg_rv = 0.0;

        locals.var_b4soidwb = p.p189;
        locals.var_b4soidwb_rv = 0.0;

        locals.var_b4soiwl = p.p190;
        locals.var_b4soiwl_rv = 0.0;

        locals.var_b4soiwlc = p.p191;
        locals.var_b4soiwlc_rv = 0.0;

        locals.var_b4soiwln = p.p192;
        locals.var_b4soiwln_rv = 0.0;

        locals.var_b4soiww = p.p193;
        locals.var_b4soiww_rv = 0.0;

        locals.var_b4soiwwc = p.p194;
        locals.var_b4soiwwc_rv = 0.0;

        locals.var_b4soiwwn = p.p195;
        locals.var_b4soiwwn_rv = 0.0;

        locals.var_b4soiwwl = p.p196;
        locals.var_b4soiwwl_rv = 0.0;

        locals.var_b4soiwwlc = p.p197;
        locals.var_b4soiwwlc_rv = 0.0;

        locals.var_b4soib0 = p.p198;
        locals.var_b4soib0_rv = 0.0;

        locals.var_b4soib1 = p.p199;
        locals.var_b4soib1_rv = 0.0;

        locals.var_b4soicgsl = p.p200;
        locals.var_b4soicgsl_rv = 0.0;

        locals.var_b4soicgdl = p.p201;
        locals.var_b4soicgdl_rv = 0.0;

        locals.var_b4soickappa = p.p202;
        locals.var_b4soickappa_rv = 0.0;

        locals.var_b4soiclc = p.p204;
        locals.var_b4soiclc_rv = 0.0;

        locals.var_b4soicle = p.p205;
        locals.var_b4soicle_rv = 0.0;

        locals.var_b4soidwc = p.p206;
        locals.var_b4soidwc_rv = 0.0;

        locals.var_b4soidlc = p.p207;
        locals.var_b4soidlc_rv = 0.0;

        locals.var_b4soialpha0 = p.p208;
        locals.var_b4soialpha0_rv = 0.0;

        locals.var_b4soitnoia = p.p214;
        locals.var_b4soitnoia_rv = 0.0;

        locals.var_b4soirnoia = p.p216;
        locals.var_b4soirnoia_rv = 0.0;

        locals.var_b4soisaref = p.p219;
        locals.var_b4soisaref_rv = 0.0;

        locals.var_b4soisbref = p.p220;
        locals.var_b4soisbref_rv = 0.0;

        locals.var_b4soiwlod = p.p221;
        locals.var_b4soiwlod_rv = 0.0;

        locals.var_b4soiku0 = p.p222;
        locals.var_b4soiku0_rv = 0.0;

        locals.var_b4soikvsat = p.p223;
        locals.var_b4soikvsat_rv = 0.0;

        locals.var_b4soikvth0 = p.p224;
        locals.var_b4soikvth0_rv = 0.0;

        locals.var_b4soitku0 = p.p225;
        locals.var_b4soitku0_rv = 0.0;

        locals.var_b4soillodku0 = p.p226;
        locals.var_b4soillodku0_rv = 0.0;

        locals.var_b4soiwlodku0 = p.p227;
        locals.var_b4soiwlodku0_rv = 0.0;

        locals.var_b4soillodvth = p.p228;
        locals.var_b4soillodvth_rv = 0.0;

        locals.var_b4soiwlodvth = p.p229;
        locals.var_b4soiwlodvth_rv = 0.0;

        locals.var_b4soistk2 = p.p236;
        locals.var_b4soistk2_rv = 0.0;

        locals.var_b4soilodk2 = p.p237;
        locals.var_b4soilodk2_rv = 0.0;

        locals.var_b4soisteta0 = p.p238;
        locals.var_b4soisteta0_rv = 0.0;

        locals.var_b4soilodeta0 = p.p239;
        locals.var_b4soilodeta0_rv = 0.0;

        locals.var_b4soisteta0cv = p.p240;
        locals.var_b4soisteta0cv_rv = 0.0;

        locals.var_b4soilodeta0cv = p.p241;
        locals.var_b4soilodeta0cv_rv = 0.0;

        locals.var_b4soidvtp0 = p.p245;
        locals.var_b4soidvtp0_rv = 0.0;

        locals.var_b4soidvtp1 = p.p249;
        locals.var_b4soidvtp1_rv = 0.0;

        locals.var_b4soidvtp2 = p.p253;
        locals.var_b4soidvtp2_rv = 0.0;

        locals.var_b4soidvtp3 = p.p257;
        locals.var_b4soidvtp3_rv = 0.0;

        locals.var_b4soidvtp4 = p.p261;
        locals.var_b4soidvtp4_rv = 0.0;

        locals.var_b4soiminv = p.p265;
        locals.var_b4soiminv_rv = 0.0;

        locals.var_b4soipdits = p.p269;
        locals.var_b4soipdits_rv = 0.0;

        locals.var_b4soipditsl = p.p270;
        locals.var_b4soipditsl_rv = 0.0;

        locals.var_b4soipditsd = p.p271;
        locals.var_b4soipditsd_rv = 0.0;

        locals.var_b4soifprout = p.p272;
        locals.var_b4soifprout_rv = 0.0;

        locals.var_b4soik1w1 = p.p287;
        locals.var_b4soik1w1_rv = 0.0;

        locals.var_b4soik1w2 = p.p288;
        locals.var_b4soik1w2_rv = 0.0;

        locals.var_b4soiketas = p.p289;
        locals.var_b4soiketas_rv = 0.0;

        locals.var_b4soidwbc = p.p290;
        locals.var_b4soidwbc_rv = 0.0;

        locals.var_b4soibeta0 = p.p291;
        locals.var_b4soibeta0_rv = 0.0;

        locals.var_b4soibeta1 = p.p292;
        locals.var_b4soibeta1_rv = 0.0;

        locals.var_b4soibeta2 = p.p293;
        locals.var_b4soibeta2_rv = 0.0;

        locals.var_b4soivdsatii0 = p.p294;
        locals.var_b4soivdsatii0_rv = 0.0;

        locals.var_b4soitii = p.p295;
        locals.var_b4soitii_rv = 0.0;

        locals.var_b4soilii = p.p296;
        locals.var_b4soilii_rv = 0.0;

        locals.var_b4soisii0 = p.p297;
        locals.var_b4soisii0_rv = 0.0;

        locals.var_b4soisii1 = p.p298;
        locals.var_b4soisii1_rv = 0.0;

        locals.var_b4soisii2 = p.p299;
        locals.var_b4soisii2_rv = 0.0;

        locals.var_b4soisiid = p.p300;
        locals.var_b4soisiid_rv = 0.0;

        locals.var_b4soifbjtii = p.p301;
        locals.var_b4soifbjtii_rv = 0.0;

        locals.var_b4soiebjtii = p.p302;
        locals.var_b4soiebjtii_rv = 0.0;

        locals.var_b4soicbjtii = p.p303;
        locals.var_b4soicbjtii_rv = 0.0;

        locals.var_b4soivbci = p.p304;
        locals.var_b4soivbci_rv = 0.0;

        locals.var_b4soiabjtii = p.p305;
        locals.var_b4soiabjtii_rv = 0.0;

        locals.var_b4soimbjtii = p.p306;
        locals.var_b4soimbjtii_rv = 0.0;

        locals.var_b4soitvbci = p.p307;
        locals.var_b4soitvbci_rv = 0.0;

        locals.var_b4soiesatii = p.p308;
        locals.var_b4soiesatii_rv = 0.0;

        locals.var_b4sointun = p.p309;
        locals.var_b4sointun_rv = 0.0;

        locals.var_b4sointund = p.p310;
        locals.var_b4sointund_rv = 0.0;

        locals.var_b4soinrecf0 = p.p311;
        locals.var_b4soinrecf0_rv = 0.0;

        locals.var_b4soinrecf0d = p.p312;
        locals.var_b4soinrecf0d_rv = 0.0;

        locals.var_b4soinrecr0 = p.p313;
        locals.var_b4soinrecr0_rv = 0.0;

        locals.var_b4soinrecr0d = p.p314;
        locals.var_b4soinrecr0d_rv = 0.0;

        locals.var_b4soiisbjt = p.p315;
        locals.var_b4soiisbjt_rv = 0.0;

        locals.var_b4soiidbjt = p.p316;
        locals.var_b4soiidbjt_rv = 0.0;

        locals.var_b4soiisdif = p.p317;
        locals.var_b4soiisdif_rv = 0.0;

        locals.var_b4soiiddif = p.p318;
        locals.var_b4soiiddif_rv = 0.0;

        locals.var_b4soiisrec = p.p319;
        locals.var_b4soiisrec_rv = 0.0;

        locals.var_b4soiidrec = p.p320;
        locals.var_b4soiidrec_rv = 0.0;

        locals.var_b4soiistun = p.p321;
        locals.var_b4soiistun_rv = 0.0;

        locals.var_b4soiidtun = p.p322;
        locals.var_b4soiidtun_rv = 0.0;

        locals.var_b4soiln = p.p323;
        locals.var_b4soiln_rv = 0.0;

        locals.var_b4soivrec0 = p.p324;
        locals.var_b4soivrec0_rv = 0.0;

        locals.var_b4soivrec0d = p.p325;
        locals.var_b4soivrec0d_rv = 0.0;

        locals.var_b4soivtun0 = p.p326;
        locals.var_b4soivtun0_rv = 0.0;

        locals.var_b4soivtun0d = p.p327;
        locals.var_b4soivtun0d_rv = 0.0;

        locals.var_b4soinbjt = p.p328;
        locals.var_b4soinbjt_rv = 0.0;

        locals.var_b4soilbjt0 = p.p329;
        locals.var_b4soilbjt0_rv = 0.0;

        locals.var_b4soildif0 = p.p330;
        locals.var_b4soildif0_rv = 0.0;

        locals.var_b4soivabjt = p.p331;
        locals.var_b4soivabjt_rv = 0.0;

        locals.var_b4soiaely = p.p332;
        locals.var_b4soiaely_rv = 0.0;

        locals.var_b4soiahli = p.p333;
        locals.var_b4soiahli_rv = 0.0;

        locals.var_b4soiahlid = p.p334;
        locals.var_b4soiahlid_rv = 0.0;

        locals.var_b4soirbody = p.p335;
        locals.var_b4soirbody_rv = 0.0;

        locals.var_b4soirbsh = p.p336;
        locals.var_b4soirbsh_rv = 0.0;

        locals.var_b4soicgeo = p.p337;
        locals.var_b4soicgeo_rv = 0.0;

        locals.var_b4soitt = p.p338;
        locals.var_b4soitt_rv = 0.0;

        locals.var_b4soindif = p.p339;
        locals.var_b4soindif_rv = 0.0;

        locals.var_b4soivsdfb = p.p340;
        locals.var_b4soivsdfb_rv = 0.0;

        locals.var_b4soivsdth = p.p341;
        locals.var_b4soivsdth_rv = 0.0;

        locals.var_b4soicsdmin = p.p342;
        locals.var_b4soicsdmin_dn3 = 0.0;
        locals.var_b4soicsdmin_dn4 = 0.0;
        locals.var_b4soicsdmin_dn5 = 0.0;
        locals.var_b4soicsdmin_dn6 = 0.0;
        locals.var_b4soicsdmin_dn7 = 0.0;
        locals.var_b4soicsdmin_dn8 = 0.0;
        locals.var_b4soicsdmin_dn9 = 0.0;
        locals.var_b4soicsdmin_dn10 = 0.0;
        locals.var_b4soicsdmin_dn11 = 0.0;
        locals.var_b4soicsdmin_dn12 = 0.0;
        locals.var_b4soicsdmin_rv = 0.0;

        locals.var_b4soiasd = p.p343;
        locals.var_b4soiasd_rv = 0.0;

        locals.var_b4soicsdesw = p.p344;
        locals.var_b4soicsdesw_rv = 0.0;

        locals.var_b4sointrecf = p.p345;
        locals.var_b4sointrecf_rv = 0.0;

        locals.var_b4sointrecr = p.p346;
        locals.var_b4sointrecr_rv = 0.0;

        locals.var_b4soidlcb = p.p347;
        locals.var_b4soidlcb_rv = 0.0;

        locals.var_b4soifbody = p.p348;
        locals.var_b4soifbody_rv = 0.0;

        locals.var_b4soitcjswg = p.p349;
        locals.var_b4soitcjswg_rv = 0.0;

        locals.var_b4soitpbswg = p.p350;
        locals.var_b4soitpbswg_rv = 0.0;

        locals.var_b4soitcjswgd = p.p351;
        locals.var_b4soitcjswgd_rv = 0.0;

        locals.var_b4soitpbswgd = p.p352;
        locals.var_b4soitpbswgd_rv = 0.0;

        locals.var_b4soiacde = p.p353;
        locals.var_b4soiacde_rv = 0.0;

        locals.var_b4soimoin = p.p354;
        locals.var_b4soimoin_rv = 0.0;

        locals.var_b4soinoff = p.p355;
        locals.var_b4soinoff_rv = 0.0;

        locals.var_b4soinoff2 = p.p356;
        locals.var_b4soinoff2_rv = 0.0;

        locals.var_b4soidelvt = p.p357;
        locals.var_b4soidelvt_rv = 0.0;

        locals.var_b4soikb1 = p.p358;
        locals.var_b4soikb1_rv = 0.0;

        locals.var_b4soidlbg = p.p359;
        locals.var_b4soidlbg_rv = 0.0;

        locals.var_b4soicfrcoeff = p.p360;
        locals.var_b4soicfrcoeff_rv = 0.0;

        locals.var_b4soiigbmod = p.p362;
        locals.var_b4soiigbmod_rv = 0.0;

        locals.var_b4soiigcmod = p.p363;
        locals.var_b4soiigcmod_rv = 0.0;

        locals.var_b4soitoxqm = p.p364;
        locals.var_b4soitoxqm_rv = 0.0;

        locals.var_b4soiwth0 = p.p365;
        locals.var_b4soiwth0_rv = 0.0;

        locals.var_b4soirhalo = p.p366;
        locals.var_b4soirhalo_rv = 0.0;

        locals.var_b4sointox = p.p367;
        locals.var_b4sointox_rv = 0.0;

        locals.var_b4soitoxref = p.p368;
        locals.var_b4soitoxref_rv = 0.0;

        locals.var_b4soiebg = p.p369;
        locals.var_b4soiebg_rv = 0.0;

        locals.var_b4soivevb = p.p370;
        locals.var_b4soivevb_rv = 0.0;

        locals.var_b4soialphagb1 = p.p371;
        locals.var_b4soialphagb1_rv = 0.0;

        locals.var_b4soibetagb1 = p.p372;
        locals.var_b4soibetagb1_rv = 0.0;

        locals.var_b4soivgb1 = p.p373;
        locals.var_b4soivgb1_rv = 0.0;

        locals.var_b4soivecb = p.p374;
        locals.var_b4soivecb_rv = 0.0;

        locals.var_b4soialphagb2 = p.p375;
        locals.var_b4soialphagb2_rv = 0.0;

        locals.var_b4soibetagb2 = p.p376;
        locals.var_b4soibetagb2_rv = 0.0;

        locals.var_b4soivgb2 = p.p377;
        locals.var_b4soivgb2_rv = 0.0;

        locals.var_b4soiaigbcp2 = p.p378;
        locals.var_b4soiaigbcp2_rv = 0.0;

        locals.var_b4soibigbcp2 = p.p379;
        locals.var_b4soibigbcp2_rv = 0.0;

        locals.var_b4soicigbcp2 = p.p380;
        locals.var_b4soicigbcp2_rv = 0.0;

        locals.var_b4soivoxh = p.p381;
        locals.var_b4soivoxh_rv = 0.0;

        locals.var_b4soideltavox = p.p382;
        locals.var_b4soideltavox_rv = 0.0;

        locals.var_b4soiaigc = p.p383;
        locals.var_b4soiaigc_rv = 0.0;

        locals.var_b4soibigc = p.p384;
        locals.var_b4soibigc_rv = 0.0;

        locals.var_b4soicigc = p.p385;
        locals.var_b4soicigc_rv = 0.0;

        locals.var_b4soiaigsd = p.p386;
        locals.var_b4soiaigsd_rv = 0.0;

        locals.var_b4soibigsd = p.p387;
        locals.var_b4soibigsd_rv = 0.0;

        locals.var_b4soicigsd = p.p388;
        locals.var_b4soicigsd_rv = 0.0;

        locals.var_b4soinigc = p.p389;
        locals.var_b4soinigc_rv = 0.0;

        locals.var_b4soipigcd = p.p390;
        locals.var_b4soipigcd_rv = 0.0;

        locals.var_b4soipoxedge = p.p391;
        locals.var_b4soipoxedge_rv = 0.0;

        locals.var_b4soidlcig = p.p392;
        locals.var_b4soidlcig_rv = 0.0;

        locals.var_b4soivbsa = p.p395;
        locals.var_b4soivbsa_rv = 0.0;

        locals.var_b4soinofffd = p.p396;
        locals.var_b4soinofffd_rv = 0.0;

        locals.var_b4soivofffd = p.p397;
        locals.var_b4soivofffd_rv = 0.0;

        locals.var_b4soik1b = p.p398;
        locals.var_b4soik1b_rv = 0.0;

        locals.var_b4soik2b = p.p399;
        locals.var_b4soik2b_rv = 0.0;

        locals.var_b4soidk2b = p.p400;
        locals.var_b4soidk2b_rv = 0.0;

        locals.var_b4soidvbd0 = p.p401;
        locals.var_b4soidvbd0_rv = 0.0;

    }
}

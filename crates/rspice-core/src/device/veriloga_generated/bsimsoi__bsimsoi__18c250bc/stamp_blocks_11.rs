#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_80(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign26600_e24156, assign26600_e24156_d_n3, assign26600_e24156_d_n4, assign26600_e24156_d_n5, assign26600_e24156_d_n6, assign26600_e24156_d_n7, assign26600_e24156_d_n8, assign26600_e24156_d_n9, assign26600_e24156_d_n10, assign26600_e24156_d_n11, assign26600_e24156_d_n12,) = {
    if ((((locals.var_guard1367 != 0.0) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1370 != 0.0)) && (locals.var_guard1371 != 0.0)) {
        let assign26600_e24147: f64 = (locals.var_v3 * locals.var_v3);
        let assign26600_e24150: f64 = (100.0 * locals.var_delta_3_soi2);
        let assign26600_e24152: f64 = (assign26600_e24150 * locals.var_vfb2);
        let assign26600_e24153: f64 = (assign26600_e24147 - assign26600_e24152);
        let assign26600_e24154: f64 = (assign26600_e24153).sqrt();
        (assign26600_e24154, ((((locals.var_v3_dn3 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn3)) - (assign26600_e24150 * locals.var_vfb2_dn3)) / (2.0 * assign26600_e24154)), ((((locals.var_v3_dn4 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn4)) - (assign26600_e24150 * locals.var_vfb2_dn4)) / (2.0 * assign26600_e24154)), ((((locals.var_v3_dn5 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn5)) - (assign26600_e24150 * locals.var_vfb2_dn5)) / (2.0 * assign26600_e24154)), ((((locals.var_v3_dn6 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn6)) - (assign26600_e24150 * locals.var_vfb2_dn6)) / (2.0 * assign26600_e24154)), ((((locals.var_v3_dn7 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn7)) - (assign26600_e24150 * locals.var_vfb2_dn7)) / (2.0 * assign26600_e24154)), ((((locals.var_v3_dn8 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn8)) - (assign26600_e24150 * locals.var_vfb2_dn8)) / (2.0 * assign26600_e24154)), ((((locals.var_v3_dn9 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn9)) - (assign26600_e24150 * locals.var_vfb2_dn9)) / (2.0 * assign26600_e24154)), ((((locals.var_v3_dn10 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn10)) - (assign26600_e24150 * locals.var_vfb2_dn10)) / (2.0 * assign26600_e24154)), ((((locals.var_v3_dn11 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn11)) - (assign26600_e24150 * locals.var_vfb2_dn11)) / (2.0 * assign26600_e24154)), ((((locals.var_v3_dn12 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn12)) - (assign26600_e24150 * locals.var_vfb2_dn12)) / (2.0 * assign26600_e24154)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign26600_e24156;
        locals.var_t0__blk808_dn3 = assign26600_e24156_d_n3;
        locals.var_t0__blk808_dn4 = assign26600_e24156_d_n4;
        locals.var_t0__blk808_dn5 = assign26600_e24156_d_n5;
        locals.var_t0__blk808_dn6 = assign26600_e24156_d_n6;
        locals.var_t0__blk808_dn7 = assign26600_e24156_d_n7;
        locals.var_t0__blk808_dn8 = assign26600_e24156_d_n8;
        locals.var_t0__blk808_dn9 = assign26600_e24156_d_n9;
        locals.var_t0__blk808_dn10 = assign26600_e24156_d_n10;
        locals.var_t0__blk808_dn11 = assign26600_e24156_d_n11;
        locals.var_t0__blk808_dn12 = assign26600_e24156_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign26610_e24177, assign26610_e24177_d_n3, assign26610_e24177_d_n4, assign26610_e24177_d_n5, assign26610_e24177_d_n6, assign26610_e24177_d_n7, assign26610_e24177_d_n8, assign26610_e24177_d_n9, assign26610_e24177_d_n10, assign26610_e24177_d_n11, assign26610_e24177_d_n12,) = {
    if ((((locals.var_guard1367 != 0.0) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1370 != 0.0)) && (locals.var_guard1371 == 0.0)) {
        let assign26610_e24168: f64 = (locals.var_v3 * locals.var_v3);
        let assign26610_e24171: f64 = (100.0 * locals.var_delta_3_soi2);
        let assign26610_e24173: f64 = (assign26610_e24171 * locals.var_vfb2);
        let assign26610_e24174: f64 = (assign26610_e24168 + assign26610_e24173);
        let assign26610_e24175: f64 = (assign26610_e24174).sqrt();
        (assign26610_e24175, ((((locals.var_v3_dn3 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn3)) + (assign26610_e24171 * locals.var_vfb2_dn3)) / (2.0 * assign26610_e24175)), ((((locals.var_v3_dn4 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn4)) + (assign26610_e24171 * locals.var_vfb2_dn4)) / (2.0 * assign26610_e24175)), ((((locals.var_v3_dn5 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn5)) + (assign26610_e24171 * locals.var_vfb2_dn5)) / (2.0 * assign26610_e24175)), ((((locals.var_v3_dn6 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn6)) + (assign26610_e24171 * locals.var_vfb2_dn6)) / (2.0 * assign26610_e24175)), ((((locals.var_v3_dn7 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn7)) + (assign26610_e24171 * locals.var_vfb2_dn7)) / (2.0 * assign26610_e24175)), ((((locals.var_v3_dn8 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn8)) + (assign26610_e24171 * locals.var_vfb2_dn8)) / (2.0 * assign26610_e24175)), ((((locals.var_v3_dn9 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn9)) + (assign26610_e24171 * locals.var_vfb2_dn9)) / (2.0 * assign26610_e24175)), ((((locals.var_v3_dn10 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn10)) + (assign26610_e24171 * locals.var_vfb2_dn10)) / (2.0 * assign26610_e24175)), ((((locals.var_v3_dn11 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn11)) + (assign26610_e24171 * locals.var_vfb2_dn11)) / (2.0 * assign26610_e24175)), ((((locals.var_v3_dn12 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn12)) + (assign26610_e24171 * locals.var_vfb2_dn12)) / (2.0 * assign26610_e24175)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign26610_e24177;
        locals.var_t0__blk808_dn3 = assign26610_e24177_d_n3;
        locals.var_t0__blk808_dn4 = assign26610_e24177_d_n4;
        locals.var_t0__blk808_dn5 = assign26610_e24177_d_n5;
        locals.var_t0__blk808_dn6 = assign26610_e24177_d_n6;
        locals.var_t0__blk808_dn7 = assign26610_e24177_d_n7;
        locals.var_t0__blk808_dn8 = assign26610_e24177_d_n8;
        locals.var_t0__blk808_dn9 = assign26610_e24177_d_n9;
        locals.var_t0__blk808_dn10 = assign26610_e24177_d_n10;
        locals.var_t0__blk808_dn11 = assign26610_e24177_d_n11;
        locals.var_t0__blk808_dn12 = assign26610_e24177_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign26620_e24192, assign26620_e24192_d_n3, assign26620_e24192_d_n4, assign26620_e24192_d_n5, assign26620_e24192_d_n6, assign26620_e24192_d_n7, assign26620_e24192_d_n8, assign26620_e24192_d_n9, assign26620_e24192_d_n10, assign26620_e24192_d_n11, assign26620_e24192_d_n12,) = {
    if (((locals.var_guard1367 != 0.0) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1370 != 0.0)) {
        let assign26620_e24188: f64 = (locals.var_v3 + locals.var_t0__blk808);
        let assign26620_e24189: f64 = (0.5 * assign26620_e24188);
        let assign26620_e24190: f64 = (locals.var_vfb2 - assign26620_e24189);
        (assign26620_e24190, (locals.var_vfb2_dn3 - (0.5 * (locals.var_v3_dn3 + locals.var_t0__blk808_dn3))), (locals.var_vfb2_dn4 - (0.5 * (locals.var_v3_dn4 + locals.var_t0__blk808_dn4))), (locals.var_vfb2_dn5 - (0.5 * (locals.var_v3_dn5 + locals.var_t0__blk808_dn5))), (locals.var_vfb2_dn6 - (0.5 * (locals.var_v3_dn6 + locals.var_t0__blk808_dn6))), (locals.var_vfb2_dn7 - (0.5 * (locals.var_v3_dn7 + locals.var_t0__blk808_dn7))), (locals.var_vfb2_dn8 - (0.5 * (locals.var_v3_dn8 + locals.var_t0__blk808_dn8))), (locals.var_vfb2_dn9 - (0.5 * (locals.var_v3_dn9 + locals.var_t0__blk808_dn9))), (locals.var_vfb2_dn10 - (0.5 * (locals.var_v3_dn10 + locals.var_t0__blk808_dn10))), (locals.var_vfb2_dn11 - (0.5 * (locals.var_v3_dn11 + locals.var_t0__blk808_dn11))), (locals.var_vfb2_dn12 - (0.5 * (locals.var_v3_dn12 + locals.var_t0__blk808_dn12))),)
    } else {
        (locals.var_vfbeff2, locals.var_vfbeff2_dn3, locals.var_vfbeff2_dn4, locals.var_vfbeff2_dn5, locals.var_vfbeff2_dn6, locals.var_vfbeff2_dn7, locals.var_vfbeff2_dn8, locals.var_vfbeff2_dn9, locals.var_vfbeff2_dn10, locals.var_vfbeff2_dn11, locals.var_vfbeff2_dn12,)
    }
};
        locals.var_vfbeff2 = assign26620_e24192;
        locals.var_vfbeff2_dn3 = assign26620_e24192_d_n3;
        locals.var_vfbeff2_dn4 = assign26620_e24192_d_n4;
        locals.var_vfbeff2_dn5 = assign26620_e24192_d_n5;
        locals.var_vfbeff2_dn6 = assign26620_e24192_d_n6;
        locals.var_vfbeff2_dn7 = assign26620_e24192_d_n7;
        locals.var_vfbeff2_dn8 = assign26620_e24192_d_n8;
        locals.var_vfbeff2_dn9 = assign26620_e24192_d_n9;
        locals.var_vfbeff2_dn10 = assign26620_e24192_d_n10;
        locals.var_vfbeff2_dn11 = assign26620_e24192_d_n11;
        locals.var_vfbeff2_dn12 = assign26620_e24192_d_n12;
        locals.var_vfbeff2_rv = 0.0;

        let (assign26630_e24207, assign26630_e24207_d_n3, assign26630_e24207_d_n4, assign26630_e24207_d_n5, assign26630_e24207_d_n6, assign26630_e24207_d_n7, assign26630_e24207_d_n8, assign26630_e24207_d_n9, assign26630_e24207_d_n10, assign26630_e24207_d_n11, assign26630_e24207_d_n12,) = {
    if (((locals.var_guard1367 != 0.0) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1370 != 0.0)) {
        let assign26630_e24203: f64 = (locals.var_vfbeff2 - locals.var_vfb2);
        let assign26630_e24204: f64 = (locals.var_coxwlb2 * assign26630_e24203);
        let assign26630_e24205: f64 = (locals.var_qac0 + assign26630_e24204);
        (assign26630_e24205, (locals.var_qac0_dn3 + ((locals.var_coxwlb2_dn3 * assign26630_e24203) + (locals.var_coxwlb2 * (locals.var_vfbeff2_dn3 - locals.var_vfb2_dn3)))), (locals.var_qac0_dn4 + ((locals.var_coxwlb2_dn4 * assign26630_e24203) + (locals.var_coxwlb2 * (locals.var_vfbeff2_dn4 - locals.var_vfb2_dn4)))), (locals.var_qac0_dn5 + ((locals.var_coxwlb2_dn5 * assign26630_e24203) + (locals.var_coxwlb2 * (locals.var_vfbeff2_dn5 - locals.var_vfb2_dn5)))), (locals.var_qac0_dn6 + ((locals.var_coxwlb2_dn6 * assign26630_e24203) + (locals.var_coxwlb2 * (locals.var_vfbeff2_dn6 - locals.var_vfb2_dn6)))), (locals.var_qac0_dn7 + ((locals.var_coxwlb2_dn7 * assign26630_e24203) + (locals.var_coxwlb2 * (locals.var_vfbeff2_dn7 - locals.var_vfb2_dn7)))), (locals.var_qac0_dn8 + ((locals.var_coxwlb2_dn8 * assign26630_e24203) + (locals.var_coxwlb2 * (locals.var_vfbeff2_dn8 - locals.var_vfb2_dn8)))), (locals.var_qac0_dn9 + ((locals.var_coxwlb2_dn9 * assign26630_e24203) + (locals.var_coxwlb2 * (locals.var_vfbeff2_dn9 - locals.var_vfb2_dn9)))), (locals.var_qac0_dn10 + ((locals.var_coxwlb2_dn10 * assign26630_e24203) + (locals.var_coxwlb2 * (locals.var_vfbeff2_dn10 - locals.var_vfb2_dn10)))), (locals.var_qac0_dn11 + ((locals.var_coxwlb2_dn11 * assign26630_e24203) + (locals.var_coxwlb2 * (locals.var_vfbeff2_dn11 - locals.var_vfb2_dn11)))), (locals.var_qac0_dn12 + ((locals.var_coxwlb2_dn12 * assign26630_e24203) + (locals.var_coxwlb2 * (locals.var_vfbeff2_dn12 - locals.var_vfb2_dn12)))),)
    } else {
        (locals.var_qac0, locals.var_qac0_dn3, locals.var_qac0_dn4, locals.var_qac0_dn5, locals.var_qac0_dn6, locals.var_qac0_dn7, locals.var_qac0_dn8, locals.var_qac0_dn9, locals.var_qac0_dn10, locals.var_qac0_dn11, locals.var_qac0_dn12,)
    }
};
        locals.var_qac0 = assign26630_e24207;
        locals.var_qac0_dn3 = assign26630_e24207_d_n3;
        locals.var_qac0_dn4 = assign26630_e24207_d_n4;
        locals.var_qac0_dn5 = assign26630_e24207_d_n5;
        locals.var_qac0_dn6 = assign26630_e24207_d_n6;
        locals.var_qac0_dn7 = assign26630_e24207_d_n7;
        locals.var_qac0_dn8 = assign26630_e24207_d_n8;
        locals.var_qac0_dn9 = assign26630_e24207_d_n9;
        locals.var_qac0_dn10 = assign26630_e24207_d_n10;
        locals.var_qac0_dn11 = assign26630_e24207_d_n11;
        locals.var_qac0_dn12 = assign26630_e24207_d_n12;
        locals.var_qac0_rv = 0.0;

        let (assign26640_e24216, assign26640_e24216_d_n3, assign26640_e24216_d_n4, assign26640_e24216_d_n5, assign26640_e24216_d_n6, assign26640_e24216_d_n7, assign26640_e24216_d_n8, assign26640_e24216_d_n9, assign26640_e24216_d_n10, assign26640_e24216_d_n11, assign26640_e24216_d_n12,) = {
    if ((locals.var_guard1367 != 0.0) && (locals.var_guard1368 == 0.0)) {
        let assign26640_e24214: f64 = (0.5 * locals.var_here_b4soik1ox);
        (assign26640_e24214, (0.5 * locals.var_here_b4soik1ox_dn3), (0.5 * locals.var_here_b4soik1ox_dn4), (0.5 * locals.var_here_b4soik1ox_dn5), (0.5 * locals.var_here_b4soik1ox_dn6), (0.5 * locals.var_here_b4soik1ox_dn7), (0.5 * locals.var_here_b4soik1ox_dn8), (0.5 * locals.var_here_b4soik1ox_dn9), (0.5 * locals.var_here_b4soik1ox_dn10), (0.5 * locals.var_here_b4soik1ox_dn11), (0.5 * locals.var_here_b4soik1ox_dn12),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign26640_e24216;
        locals.var_t0__blk808_dn3 = assign26640_e24216_d_n3;
        locals.var_t0__blk808_dn4 = assign26640_e24216_d_n4;
        locals.var_t0__blk808_dn5 = assign26640_e24216_d_n5;
        locals.var_t0__blk808_dn6 = assign26640_e24216_d_n6;
        locals.var_t0__blk808_dn7 = assign26640_e24216_d_n7;
        locals.var_t0__blk808_dn8 = assign26640_e24216_d_n8;
        locals.var_t0__blk808_dn9 = assign26640_e24216_d_n9;
        locals.var_t0__blk808_dn10 = assign26640_e24216_d_n10;
        locals.var_t0__blk808_dn11 = assign26640_e24216_d_n11;
        locals.var_t0__blk808_dn12 = assign26640_e24216_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign26650_e24229, assign26650_e24229_d_n3, assign26650_e24229_d_n4, assign26650_e24229_d_n5, assign26650_e24229_d_n6, assign26650_e24229_d_n7, assign26650_e24229_d_n8, assign26650_e24229_d_n9, assign26650_e24229_d_n10, assign26650_e24229_d_n11, assign26650_e24229_d_n12,) = {
    if ((locals.var_guard1367 != 0.0) && (locals.var_guard1368 == 0.0)) {
        let assign26650_e24223: f64 = (locals.var_vgs_eff__blk790 - locals.var_vfbeff);
        let assign26650_e24225: f64 = (assign26650_e24223 - locals.var_vbseff);
        let assign26650_e24227: f64 = (assign26650_e24225 - locals.var_vgsteff__blk840);
        (assign26650_e24227, (((locals.var_vgs_eff__blk790_dn3 - locals.var_vfbeff_dn3) - locals.var_vbseff_dn3) - locals.var_vgsteff__blk840_dn3), (((locals.var_vgs_eff__blk790_dn4 - locals.var_vfbeff_dn4) - locals.var_vbseff_dn4) - locals.var_vgsteff__blk840_dn4), (((locals.var_vgs_eff__blk790_dn5 - locals.var_vfbeff_dn5) - locals.var_vbseff_dn5) - locals.var_vgsteff__blk840_dn5), (((locals.var_vgs_eff__blk790_dn6 - locals.var_vfbeff_dn6) - locals.var_vbseff_dn6) - locals.var_vgsteff__blk840_dn6), (((locals.var_vgs_eff__blk790_dn7 - locals.var_vfbeff_dn7) - locals.var_vbseff_dn7) - locals.var_vgsteff__blk840_dn7), (((locals.var_vgs_eff__blk790_dn8 - locals.var_vfbeff_dn8) - locals.var_vbseff_dn8) - locals.var_vgsteff__blk840_dn8), (((locals.var_vgs_eff__blk790_dn9 - locals.var_vfbeff_dn9) - locals.var_vbseff_dn9) - locals.var_vgsteff__blk840_dn9), (((locals.var_vgs_eff__blk790_dn10 - locals.var_vfbeff_dn10) - locals.var_vbseff_dn10) - locals.var_vgsteff__blk840_dn10), (((locals.var_vgs_eff__blk790_dn11 - locals.var_vfbeff_dn11) - locals.var_vbseff_dn11) - locals.var_vgsteff__blk840_dn11), (((locals.var_vgs_eff__blk790_dn12 - locals.var_vfbeff_dn12) - locals.var_vbseff_dn12) - locals.var_vgsteff__blk840_dn12),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign26650_e24229;
        locals.var_t3__blk811_dn3 = assign26650_e24229_d_n3;
        locals.var_t3__blk811_dn4 = assign26650_e24229_d_n4;
        locals.var_t3__blk811_dn5 = assign26650_e24229_d_n5;
        locals.var_t3__blk811_dn6 = assign26650_e24229_d_n6;
        locals.var_t3__blk811_dn7 = assign26650_e24229_d_n7;
        locals.var_t3__blk811_dn8 = assign26650_e24229_d_n8;
        locals.var_t3__blk811_dn9 = assign26650_e24229_d_n9;
        locals.var_t3__blk811_dn10 = assign26650_e24229_d_n10;
        locals.var_t3__blk811_dn11 = assign26650_e24229_d_n11;
        locals.var_t3__blk811_dn12 = assign26650_e24229_d_n12;
        locals.var_t3__blk811_rv = 0.0;

        let assign26660_e24232: f64 = if locals.var_here_b4soik1ox == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1372 = assign26660_e24232;
        locals.var_guard1372_rv = 0.0;

        let (assign26670_e24241, assign26670_e24241_d_n3, assign26670_e24241_d_n4, assign26670_e24241_d_n5, assign26670_e24241_d_n6, assign26670_e24241_d_n7, assign26670_e24241_d_n8, assign26670_e24241_d_n9, assign26670_e24241_d_n10, assign26670_e24241_d_n11, assign26670_e24241_d_n12,) = {
    if (((locals.var_guard1367 != 0.0) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1372 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign26670_e24241;
        locals.var_t1__blk809_dn3 = assign26670_e24241_d_n3;
        locals.var_t1__blk809_dn4 = assign26670_e24241_d_n4;
        locals.var_t1__blk809_dn5 = assign26670_e24241_d_n5;
        locals.var_t1__blk809_dn6 = assign26670_e24241_d_n6;
        locals.var_t1__blk809_dn7 = assign26670_e24241_d_n7;
        locals.var_t1__blk809_dn8 = assign26670_e24241_d_n8;
        locals.var_t1__blk809_dn9 = assign26670_e24241_d_n9;
        locals.var_t1__blk809_dn10 = assign26670_e24241_d_n10;
        locals.var_t1__blk809_dn11 = assign26670_e24241_d_n11;
        locals.var_t1__blk809_dn12 = assign26670_e24241_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let assign26680_e24244: f64 = if locals.var_t3__blk811 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1373 = assign26680_e24244;
        locals.var_guard1373_rv = 0.0;

        let (assign26690_e24260, assign26690_e24260_d_n3, assign26690_e24260_d_n4, assign26690_e24260_d_n5, assign26690_e24260_d_n6, assign26690_e24260_d_n7, assign26690_e24260_d_n8, assign26690_e24260_d_n9, assign26690_e24260_d_n10, assign26690_e24260_d_n11, assign26690_e24260_d_n12,) = {
    if ((((locals.var_guard1367 != 0.0) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 != 0.0)) {
        let assign26690_e24257: f64 = (locals.var_t3__blk811 / locals.var_here_b4soik1ox);
        let assign26690_e24258: f64 = (locals.var_t0__blk808 + assign26690_e24257);
        (assign26690_e24258, (locals.var_t0__blk808_dn3 + (((locals.var_t3__blk811_dn3 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn3)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn4 + (((locals.var_t3__blk811_dn4 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn4)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn5 + (((locals.var_t3__blk811_dn5 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn5)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn6 + (((locals.var_t3__blk811_dn6 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn6)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn7 + (((locals.var_t3__blk811_dn7 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn7)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn8 + (((locals.var_t3__blk811_dn8 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn8)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn9 + (((locals.var_t3__blk811_dn9 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn9)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn10 + (((locals.var_t3__blk811_dn10 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn10)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn11 + (((locals.var_t3__blk811_dn11 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn11)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn12 + (((locals.var_t3__blk811_dn12 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn12)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign26690_e24260;
        locals.var_t1__blk809_dn3 = assign26690_e24260_d_n3;
        locals.var_t1__blk809_dn4 = assign26690_e24260_d_n4;
        locals.var_t1__blk809_dn5 = assign26690_e24260_d_n5;
        locals.var_t1__blk809_dn6 = assign26690_e24260_d_n6;
        locals.var_t1__blk809_dn7 = assign26690_e24260_d_n7;
        locals.var_t1__blk809_dn8 = assign26690_e24260_d_n8;
        locals.var_t1__blk809_dn9 = assign26690_e24260_d_n9;
        locals.var_t1__blk809_dn10 = assign26690_e24260_d_n10;
        locals.var_t1__blk809_dn11 = assign26690_e24260_d_n11;
        locals.var_t1__blk809_dn12 = assign26690_e24260_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign26700_e24278, assign26700_e24278_d_n3, assign26700_e24278_d_n4, assign26700_e24278_d_n5, assign26700_e24278_d_n6, assign26700_e24278_d_n7, assign26700_e24278_d_n8, assign26700_e24278_d_n9, assign26700_e24278_d_n10, assign26700_e24278_d_n11, assign26700_e24278_d_n12,) = {
    if ((((locals.var_guard1367 != 0.0) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 == 0.0)) {
        let assign26700_e24273: f64 = (locals.var_t0__blk808 * locals.var_t0__blk808);
        let assign26700_e24275: f64 = (assign26700_e24273 + locals.var_t3__blk811);
        let assign26700_e24276: f64 = (assign26700_e24275).sqrt();
        (assign26700_e24276, ((((locals.var_t0__blk808_dn3 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn3)) + locals.var_t3__blk811_dn3) / (2.0 * assign26700_e24276)), ((((locals.var_t0__blk808_dn4 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn4)) + locals.var_t3__blk811_dn4) / (2.0 * assign26700_e24276)), ((((locals.var_t0__blk808_dn5 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn5)) + locals.var_t3__blk811_dn5) / (2.0 * assign26700_e24276)), ((((locals.var_t0__blk808_dn6 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn6)) + locals.var_t3__blk811_dn6) / (2.0 * assign26700_e24276)), ((((locals.var_t0__blk808_dn7 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn7)) + locals.var_t3__blk811_dn7) / (2.0 * assign26700_e24276)), ((((locals.var_t0__blk808_dn8 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn8)) + locals.var_t3__blk811_dn8) / (2.0 * assign26700_e24276)), ((((locals.var_t0__blk808_dn9 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn9)) + locals.var_t3__blk811_dn9) / (2.0 * assign26700_e24276)), ((((locals.var_t0__blk808_dn10 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn10)) + locals.var_t3__blk811_dn10) / (2.0 * assign26700_e24276)), ((((locals.var_t0__blk808_dn11 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn11)) + locals.var_t3__blk811_dn11) / (2.0 * assign26700_e24276)), ((((locals.var_t0__blk808_dn12 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn12)) + locals.var_t3__blk811_dn12) / (2.0 * assign26700_e24276)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign26700_e24278;
        locals.var_t1__blk809_dn3 = assign26700_e24278_d_n3;
        locals.var_t1__blk809_dn4 = assign26700_e24278_d_n4;
        locals.var_t1__blk809_dn5 = assign26700_e24278_d_n5;
        locals.var_t1__blk809_dn6 = assign26700_e24278_d_n6;
        locals.var_t1__blk809_dn7 = assign26700_e24278_d_n7;
        locals.var_t1__blk809_dn8 = assign26700_e24278_d_n8;
        locals.var_t1__blk809_dn9 = assign26700_e24278_d_n9;
        locals.var_t1__blk809_dn10 = assign26700_e24278_d_n10;
        locals.var_t1__blk809_dn11 = assign26700_e24278_d_n11;
        locals.var_t1__blk809_dn12 = assign26700_e24278_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign26710_e24291, assign26710_e24291_d_n3, assign26710_e24291_d_n4, assign26710_e24291_d_n5, assign26710_e24291_d_n6, assign26710_e24291_d_n7, assign26710_e24291_d_n8, assign26710_e24291_d_n9, assign26710_e24291_d_n10, assign26710_e24291_d_n11, assign26710_e24291_d_n12,) = {
    if ((locals.var_guard1367 != 0.0) && (locals.var_guard1368 == 0.0)) {
        let assign26710_e24285: f64 = (locals.var_coxwlb * locals.var_here_b4soik1ox);
        let assign26710_e24288: f64 = (locals.var_t1__blk809 - locals.var_t0__blk808);
        let assign26710_e24289: f64 = (assign26710_e24285 * assign26710_e24288);
        (assign26710_e24289, ((((locals.var_coxwlb_dn3 * locals.var_here_b4soik1ox) + (locals.var_coxwlb * locals.var_here_b4soik1ox_dn3)) * assign26710_e24288) + (assign26710_e24285 * (locals.var_t1__blk809_dn3 - locals.var_t0__blk808_dn3))), ((((locals.var_coxwlb_dn4 * locals.var_here_b4soik1ox) + (locals.var_coxwlb * locals.var_here_b4soik1ox_dn4)) * assign26710_e24288) + (assign26710_e24285 * (locals.var_t1__blk809_dn4 - locals.var_t0__blk808_dn4))), ((((locals.var_coxwlb_dn5 * locals.var_here_b4soik1ox) + (locals.var_coxwlb * locals.var_here_b4soik1ox_dn5)) * assign26710_e24288) + (assign26710_e24285 * (locals.var_t1__blk809_dn5 - locals.var_t0__blk808_dn5))), ((((locals.var_coxwlb_dn6 * locals.var_here_b4soik1ox) + (locals.var_coxwlb * locals.var_here_b4soik1ox_dn6)) * assign26710_e24288) + (assign26710_e24285 * (locals.var_t1__blk809_dn6 - locals.var_t0__blk808_dn6))), ((((locals.var_coxwlb_dn7 * locals.var_here_b4soik1ox) + (locals.var_coxwlb * locals.var_here_b4soik1ox_dn7)) * assign26710_e24288) + (assign26710_e24285 * (locals.var_t1__blk809_dn7 - locals.var_t0__blk808_dn7))), ((((locals.var_coxwlb_dn8 * locals.var_here_b4soik1ox) + (locals.var_coxwlb * locals.var_here_b4soik1ox_dn8)) * assign26710_e24288) + (assign26710_e24285 * (locals.var_t1__blk809_dn8 - locals.var_t0__blk808_dn8))), ((((locals.var_coxwlb_dn9 * locals.var_here_b4soik1ox) + (locals.var_coxwlb * locals.var_here_b4soik1ox_dn9)) * assign26710_e24288) + (assign26710_e24285 * (locals.var_t1__blk809_dn9 - locals.var_t0__blk808_dn9))), ((((locals.var_coxwlb_dn10 * locals.var_here_b4soik1ox) + (locals.var_coxwlb * locals.var_here_b4soik1ox_dn10)) * assign26710_e24288) + (assign26710_e24285 * (locals.var_t1__blk809_dn10 - locals.var_t0__blk808_dn10))), ((((locals.var_coxwlb_dn11 * locals.var_here_b4soik1ox) + (locals.var_coxwlb * locals.var_here_b4soik1ox_dn11)) * assign26710_e24288) + (assign26710_e24285 * (locals.var_t1__blk809_dn11 - locals.var_t0__blk808_dn11))), ((((locals.var_coxwlb_dn12 * locals.var_here_b4soik1ox) + (locals.var_coxwlb * locals.var_here_b4soik1ox_dn12)) * assign26710_e24288) + (assign26710_e24285 * (locals.var_t1__blk809_dn12 - locals.var_t0__blk808_dn12))),)
    } else {
        (locals.var_qsub0, locals.var_qsub0_dn3, locals.var_qsub0_dn4, locals.var_qsub0_dn5, locals.var_qsub0_dn6, locals.var_qsub0_dn7, locals.var_qsub0_dn8, locals.var_qsub0_dn9, locals.var_qsub0_dn10, locals.var_qsub0_dn11, locals.var_qsub0_dn12,)
    }
};
        locals.var_qsub0 = assign26710_e24291;
        locals.var_qsub0_dn3 = assign26710_e24291_d_n3;
        locals.var_qsub0_dn4 = assign26710_e24291_d_n4;
        locals.var_qsub0_dn5 = assign26710_e24291_d_n5;
        locals.var_qsub0_dn6 = assign26710_e24291_d_n6;
        locals.var_qsub0_dn7 = assign26710_e24291_d_n7;
        locals.var_qsub0_dn8 = assign26710_e24291_d_n8;
        locals.var_qsub0_dn9 = assign26710_e24291_d_n9;
        locals.var_qsub0_dn10 = assign26710_e24291_d_n10;
        locals.var_qsub0_dn11 = assign26710_e24291_d_n11;
        locals.var_qsub0_dn12 = assign26710_e24291_d_n12;
        locals.var_qsub0_rv = 0.0;

        let assign26720_e24302: f64 = if (((locals.var_b4soisoimod != 2.0) && (locals.var_b4soibodymod != 0.0)) && (p.p27 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1374 = assign26720_e24302;
        locals.var_guard1374_rv = 0.0;

        let (assign26730_e24317, assign26730_e24317_d_n3, assign26730_e24317_d_n4, assign26730_e24317_d_n5, assign26730_e24317_d_n6, assign26730_e24317_d_n7, assign26730_e24317_d_n8, assign26730_e24317_d_n9, assign26730_e24317_d_n10, assign26730_e24317_d_n11, assign26730_e24317_d_n12,) = {
    if (((locals.var_guard1367 != 0.0) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1374 != 0.0)) {
        let assign26730_e24311: f64 = (locals.var_vgs_eff2 - locals.var_vfbeff2);
        let assign26730_e24313: f64 = (assign26730_e24311 - locals.var_vbseff);
        let assign26730_e24315: f64 = (assign26730_e24313 - locals.var_vgsteff2);
        (assign26730_e24315, (((-locals.var_vfbeff2_dn3) - locals.var_vbseff_dn3) - locals.var_vgsteff2_dn3), (((-locals.var_vfbeff2_dn4) - locals.var_vbseff_dn4) - locals.var_vgsteff2_dn4), (((-locals.var_vfbeff2_dn5) - locals.var_vbseff_dn5) - locals.var_vgsteff2_dn5), (((-locals.var_vfbeff2_dn6) - locals.var_vbseff_dn6) - locals.var_vgsteff2_dn6), (((locals.var_vgs_eff2_dn7 - locals.var_vfbeff2_dn7) - locals.var_vbseff_dn7) - locals.var_vgsteff2_dn7), (((locals.var_vgs_eff2_dn8 - locals.var_vfbeff2_dn8) - locals.var_vbseff_dn8) - locals.var_vgsteff2_dn8), (((locals.var_vgs_eff2_dn9 - locals.var_vfbeff2_dn9) - locals.var_vbseff_dn9) - locals.var_vgsteff2_dn9), (((-locals.var_vfbeff2_dn10) - locals.var_vbseff_dn10) - locals.var_vgsteff2_dn10), (((-locals.var_vfbeff2_dn11) - locals.var_vbseff_dn11) - locals.var_vgsteff2_dn11), (((-locals.var_vfbeff2_dn12) - locals.var_vbseff_dn12) - locals.var_vgsteff2_dn12),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign26730_e24317;
        locals.var_t3__blk811_dn3 = assign26730_e24317_d_n3;
        locals.var_t3__blk811_dn4 = assign26730_e24317_d_n4;
        locals.var_t3__blk811_dn5 = assign26730_e24317_d_n5;
        locals.var_t3__blk811_dn6 = assign26730_e24317_d_n6;
        locals.var_t3__blk811_dn7 = assign26730_e24317_d_n7;
        locals.var_t3__blk811_dn8 = assign26730_e24317_d_n8;
        locals.var_t3__blk811_dn9 = assign26730_e24317_d_n9;
        locals.var_t3__blk811_dn10 = assign26730_e24317_d_n10;
        locals.var_t3__blk811_dn11 = assign26730_e24317_d_n11;
        locals.var_t3__blk811_dn12 = assign26730_e24317_d_n12;
        locals.var_t3__blk811_rv = 0.0;

        let assign26740_e24320: f64 = if locals.var_t3__blk811 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1375 = assign26740_e24320;
        locals.var_guard1375_rv = 0.0;

        let (assign26750_e24335, assign26750_e24335_d_n3, assign26750_e24335_d_n4, assign26750_e24335_d_n5, assign26750_e24335_d_n6, assign26750_e24335_d_n7, assign26750_e24335_d_n8, assign26750_e24335_d_n9, assign26750_e24335_d_n10, assign26750_e24335_d_n11, assign26750_e24335_d_n12,) = {
    if ((((locals.var_guard1367 != 0.0) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1374 != 0.0)) && (locals.var_guard1375 != 0.0)) {
        let assign26750_e24332: f64 = (locals.var_t3__blk811 / locals.var_here_b4soik1ox);
        let assign26750_e24333: f64 = (locals.var_t0__blk808 + assign26750_e24332);
        (assign26750_e24333, (locals.var_t0__blk808_dn3 + (((locals.var_t3__blk811_dn3 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn3)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn4 + (((locals.var_t3__blk811_dn4 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn4)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn5 + (((locals.var_t3__blk811_dn5 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn5)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn6 + (((locals.var_t3__blk811_dn6 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn6)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn7 + (((locals.var_t3__blk811_dn7 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn7)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn8 + (((locals.var_t3__blk811_dn8 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn8)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn9 + (((locals.var_t3__blk811_dn9 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn9)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn10 + (((locals.var_t3__blk811_dn10 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn10)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn11 + (((locals.var_t3__blk811_dn11 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn11)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn12 + (((locals.var_t3__blk811_dn12 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn12)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign26750_e24335;
        locals.var_t1__blk809_dn3 = assign26750_e24335_d_n3;
        locals.var_t1__blk809_dn4 = assign26750_e24335_d_n4;
        locals.var_t1__blk809_dn5 = assign26750_e24335_d_n5;
        locals.var_t1__blk809_dn6 = assign26750_e24335_d_n6;
        locals.var_t1__blk809_dn7 = assign26750_e24335_d_n7;
        locals.var_t1__blk809_dn8 = assign26750_e24335_d_n8;
        locals.var_t1__blk809_dn9 = assign26750_e24335_d_n9;
        locals.var_t1__blk809_dn10 = assign26750_e24335_d_n10;
        locals.var_t1__blk809_dn11 = assign26750_e24335_d_n11;
        locals.var_t1__blk809_dn12 = assign26750_e24335_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign26760_e24352, assign26760_e24352_d_n3, assign26760_e24352_d_n4, assign26760_e24352_d_n5, assign26760_e24352_d_n6, assign26760_e24352_d_n7, assign26760_e24352_d_n8, assign26760_e24352_d_n9, assign26760_e24352_d_n10, assign26760_e24352_d_n11, assign26760_e24352_d_n12,) = {
    if ((((locals.var_guard1367 != 0.0) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1374 != 0.0)) && (locals.var_guard1375 == 0.0)) {
        let assign26760_e24347: f64 = (locals.var_t0__blk808 * locals.var_t0__blk808);
        let assign26760_e24349: f64 = (assign26760_e24347 + locals.var_t3__blk811);
        let assign26760_e24350: f64 = (assign26760_e24349).sqrt();
        (assign26760_e24350, ((((locals.var_t0__blk808_dn3 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn3)) + locals.var_t3__blk811_dn3) / (2.0 * assign26760_e24350)), ((((locals.var_t0__blk808_dn4 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn4)) + locals.var_t3__blk811_dn4) / (2.0 * assign26760_e24350)), ((((locals.var_t0__blk808_dn5 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn5)) + locals.var_t3__blk811_dn5) / (2.0 * assign26760_e24350)), ((((locals.var_t0__blk808_dn6 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn6)) + locals.var_t3__blk811_dn6) / (2.0 * assign26760_e24350)), ((((locals.var_t0__blk808_dn7 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn7)) + locals.var_t3__blk811_dn7) / (2.0 * assign26760_e24350)), ((((locals.var_t0__blk808_dn8 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn8)) + locals.var_t3__blk811_dn8) / (2.0 * assign26760_e24350)), ((((locals.var_t0__blk808_dn9 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn9)) + locals.var_t3__blk811_dn9) / (2.0 * assign26760_e24350)), ((((locals.var_t0__blk808_dn10 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn10)) + locals.var_t3__blk811_dn10) / (2.0 * assign26760_e24350)), ((((locals.var_t0__blk808_dn11 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn11)) + locals.var_t3__blk811_dn11) / (2.0 * assign26760_e24350)), ((((locals.var_t0__blk808_dn12 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn12)) + locals.var_t3__blk811_dn12) / (2.0 * assign26760_e24350)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign26760_e24352;
        locals.var_t1__blk809_dn3 = assign26760_e24352_d_n3;
        locals.var_t1__blk809_dn4 = assign26760_e24352_d_n4;
        locals.var_t1__blk809_dn5 = assign26760_e24352_d_n5;
        locals.var_t1__blk809_dn6 = assign26760_e24352_d_n6;
        locals.var_t1__blk809_dn7 = assign26760_e24352_d_n7;
        locals.var_t1__blk809_dn8 = assign26760_e24352_d_n8;
        locals.var_t1__blk809_dn9 = assign26760_e24352_d_n9;
        locals.var_t1__blk809_dn10 = assign26760_e24352_d_n10;
        locals.var_t1__blk809_dn11 = assign26760_e24352_d_n11;
        locals.var_t1__blk809_dn12 = assign26760_e24352_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign26770_e24369, assign26770_e24369_d_n3, assign26770_e24369_d_n4, assign26770_e24369_d_n5, assign26770_e24369_d_n6, assign26770_e24369_d_n7, assign26770_e24369_d_n8, assign26770_e24369_d_n9, assign26770_e24369_d_n10, assign26770_e24369_d_n11, assign26770_e24369_d_n12,) = {
    if (((locals.var_guard1367 != 0.0) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1374 != 0.0)) {
        let assign26770_e24362: f64 = (locals.var_coxwlb2 * locals.var_here_b4soik1ox);
        let assign26770_e24365: f64 = (locals.var_t1__blk809 - locals.var_t0__blk808);
        let assign26770_e24366: f64 = (assign26770_e24362 * assign26770_e24365);
        let assign26770_e24367: f64 = (locals.var_qsub0 + assign26770_e24366);
        (assign26770_e24367, (locals.var_qsub0_dn3 + ((((locals.var_coxwlb2_dn3 * locals.var_here_b4soik1ox) + (locals.var_coxwlb2 * locals.var_here_b4soik1ox_dn3)) * assign26770_e24365) + (assign26770_e24362 * (locals.var_t1__blk809_dn3 - locals.var_t0__blk808_dn3)))), (locals.var_qsub0_dn4 + ((((locals.var_coxwlb2_dn4 * locals.var_here_b4soik1ox) + (locals.var_coxwlb2 * locals.var_here_b4soik1ox_dn4)) * assign26770_e24365) + (assign26770_e24362 * (locals.var_t1__blk809_dn4 - locals.var_t0__blk808_dn4)))), (locals.var_qsub0_dn5 + ((((locals.var_coxwlb2_dn5 * locals.var_here_b4soik1ox) + (locals.var_coxwlb2 * locals.var_here_b4soik1ox_dn5)) * assign26770_e24365) + (assign26770_e24362 * (locals.var_t1__blk809_dn5 - locals.var_t0__blk808_dn5)))), (locals.var_qsub0_dn6 + ((((locals.var_coxwlb2_dn6 * locals.var_here_b4soik1ox) + (locals.var_coxwlb2 * locals.var_here_b4soik1ox_dn6)) * assign26770_e24365) + (assign26770_e24362 * (locals.var_t1__blk809_dn6 - locals.var_t0__blk808_dn6)))), (locals.var_qsub0_dn7 + ((((locals.var_coxwlb2_dn7 * locals.var_here_b4soik1ox) + (locals.var_coxwlb2 * locals.var_here_b4soik1ox_dn7)) * assign26770_e24365) + (assign26770_e24362 * (locals.var_t1__blk809_dn7 - locals.var_t0__blk808_dn7)))), (locals.var_qsub0_dn8 + ((((locals.var_coxwlb2_dn8 * locals.var_here_b4soik1ox) + (locals.var_coxwlb2 * locals.var_here_b4soik1ox_dn8)) * assign26770_e24365) + (assign26770_e24362 * (locals.var_t1__blk809_dn8 - locals.var_t0__blk808_dn8)))), (locals.var_qsub0_dn9 + ((((locals.var_coxwlb2_dn9 * locals.var_here_b4soik1ox) + (locals.var_coxwlb2 * locals.var_here_b4soik1ox_dn9)) * assign26770_e24365) + (assign26770_e24362 * (locals.var_t1__blk809_dn9 - locals.var_t0__blk808_dn9)))), (locals.var_qsub0_dn10 + ((((locals.var_coxwlb2_dn10 * locals.var_here_b4soik1ox) + (locals.var_coxwlb2 * locals.var_here_b4soik1ox_dn10)) * assign26770_e24365) + (assign26770_e24362 * (locals.var_t1__blk809_dn10 - locals.var_t0__blk808_dn10)))), (locals.var_qsub0_dn11 + ((((locals.var_coxwlb2_dn11 * locals.var_here_b4soik1ox) + (locals.var_coxwlb2 * locals.var_here_b4soik1ox_dn11)) * assign26770_e24365) + (assign26770_e24362 * (locals.var_t1__blk809_dn11 - locals.var_t0__blk808_dn11)))), (locals.var_qsub0_dn12 + ((((locals.var_coxwlb2_dn12 * locals.var_here_b4soik1ox) + (locals.var_coxwlb2 * locals.var_here_b4soik1ox_dn12)) * assign26770_e24365) + (assign26770_e24362 * (locals.var_t1__blk809_dn12 - locals.var_t0__blk808_dn12)))),)
    } else {
        (locals.var_qsub0, locals.var_qsub0_dn3, locals.var_qsub0_dn4, locals.var_qsub0_dn5, locals.var_qsub0_dn6, locals.var_qsub0_dn7, locals.var_qsub0_dn8, locals.var_qsub0_dn9, locals.var_qsub0_dn10, locals.var_qsub0_dn11, locals.var_qsub0_dn12,)
    }
};
        locals.var_qsub0 = assign26770_e24369;
        locals.var_qsub0_dn3 = assign26770_e24369_d_n3;
        locals.var_qsub0_dn4 = assign26770_e24369_d_n4;
        locals.var_qsub0_dn5 = assign26770_e24369_d_n5;
        locals.var_qsub0_dn6 = assign26770_e24369_d_n6;
        locals.var_qsub0_dn7 = assign26770_e24369_d_n7;
        locals.var_qsub0_dn8 = assign26770_e24369_d_n8;
        locals.var_qsub0_dn9 = assign26770_e24369_d_n9;
        locals.var_qsub0_dn10 = assign26770_e24369_d_n10;
        locals.var_qsub0_dn11 = assign26770_e24369_d_n11;
        locals.var_qsub0_dn12 = assign26770_e24369_d_n12;
        locals.var_qsub0_rv = 0.0;

        let (assign26780_e24375, assign26780_e24375_d_n3, assign26780_e24375_d_n4, assign26780_e24375_d_n5, assign26780_e24375_d_n6, assign26780_e24375_d_n7, assign26780_e24375_d_n8, assign26780_e24375_d_n9, assign26780_e24375_d_n10, assign26780_e24375_d_n11, assign26780_e24375_d_n12,) = {
    if (locals.var_guard1367 != 0.0) {
        let assign26780_e24373: f64 = (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor);
        (assign26780_e24373, ((locals.var_abulk0_dn3 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn3)), ((locals.var_abulk0_dn4 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn4)), ((locals.var_abulk0_dn5 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn5)), ((locals.var_abulk0_dn6 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn6)), ((locals.var_abulk0_dn7 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn7)), ((locals.var_abulk0_dn8 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn8)), ((locals.var_abulk0_dn9 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn9)), ((locals.var_abulk0_dn10 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn10)), ((locals.var_abulk0_dn11 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn11)), ((locals.var_abulk0_dn12 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn12)),)
    } else {
        (locals.var_abulkcv, locals.var_abulkcv_dn3, locals.var_abulkcv_dn4, locals.var_abulkcv_dn5, locals.var_abulkcv_dn6, locals.var_abulkcv_dn7, locals.var_abulkcv_dn8, locals.var_abulkcv_dn9, locals.var_abulkcv_dn10, locals.var_abulkcv_dn11, locals.var_abulkcv_dn12,)
    }
};
        locals.var_abulkcv = assign26780_e24375;
        locals.var_abulkcv_dn3 = assign26780_e24375_d_n3;
        locals.var_abulkcv_dn4 = assign26780_e24375_d_n4;
        locals.var_abulkcv_dn5 = assign26780_e24375_d_n5;
        locals.var_abulkcv_dn6 = assign26780_e24375_d_n6;
        locals.var_abulkcv_dn7 = assign26780_e24375_d_n7;
        locals.var_abulkcv_dn8 = assign26780_e24375_d_n8;
        locals.var_abulkcv_dn9 = assign26780_e24375_d_n9;
        locals.var_abulkcv_dn10 = assign26780_e24375_d_n10;
        locals.var_abulkcv_dn11 = assign26780_e24375_d_n11;
        locals.var_abulkcv_dn12 = assign26780_e24375_d_n12;
        locals.var_abulkcv_rv = 0.0;

        let (assign26790_e24381, assign26790_e24381_d_n3, assign26790_e24381_d_n4, assign26790_e24381_d_n5, assign26790_e24381_d_n6, assign26790_e24381_d_n7, assign26790_e24381_d_n8, assign26790_e24381_d_n9, assign26790_e24381_d_n10, assign26790_e24381_d_n11, assign26790_e24381_d_n12,) = {
    if (locals.var_guard1367 != 0.0) {
        let assign26790_e24379: f64 = (locals.var_vgsteff__blk840 / locals.var_abulkcv);
        (assign26790_e24379, (((locals.var_vgsteff__blk840_dn3 * locals.var_abulkcv) - (locals.var_vgsteff__blk840 * locals.var_abulkcv_dn3)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff__blk840_dn4 * locals.var_abulkcv) - (locals.var_vgsteff__blk840 * locals.var_abulkcv_dn4)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff__blk840_dn5 * locals.var_abulkcv) - (locals.var_vgsteff__blk840 * locals.var_abulkcv_dn5)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff__blk840_dn6 * locals.var_abulkcv) - (locals.var_vgsteff__blk840 * locals.var_abulkcv_dn6)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff__blk840_dn7 * locals.var_abulkcv) - (locals.var_vgsteff__blk840 * locals.var_abulkcv_dn7)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff__blk840_dn8 * locals.var_abulkcv) - (locals.var_vgsteff__blk840 * locals.var_abulkcv_dn8)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff__blk840_dn9 * locals.var_abulkcv) - (locals.var_vgsteff__blk840 * locals.var_abulkcv_dn9)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff__blk840_dn10 * locals.var_abulkcv) - (locals.var_vgsteff__blk840 * locals.var_abulkcv_dn10)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff__blk840_dn11 * locals.var_abulkcv) - (locals.var_vgsteff__blk840 * locals.var_abulkcv_dn11)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff__blk840_dn12 * locals.var_abulkcv) - (locals.var_vgsteff__blk840 * locals.var_abulkcv_dn12)) / (locals.var_abulkcv * locals.var_abulkcv)),)
    } else {
        (locals.var_vdsatcv, locals.var_vdsatcv_dn3, locals.var_vdsatcv_dn4, locals.var_vdsatcv_dn5, locals.var_vdsatcv_dn6, locals.var_vdsatcv_dn7, locals.var_vdsatcv_dn8, locals.var_vdsatcv_dn9, locals.var_vdsatcv_dn10, locals.var_vdsatcv_dn11, locals.var_vdsatcv_dn12,)
    }
};
        locals.var_vdsatcv = assign26790_e24381;
        locals.var_vdsatcv_dn3 = assign26790_e24381_d_n3;
        locals.var_vdsatcv_dn4 = assign26790_e24381_d_n4;
        locals.var_vdsatcv_dn5 = assign26790_e24381_d_n5;
        locals.var_vdsatcv_dn6 = assign26790_e24381_d_n6;
        locals.var_vdsatcv_dn7 = assign26790_e24381_d_n7;
        locals.var_vdsatcv_dn8 = assign26790_e24381_d_n8;
        locals.var_vdsatcv_dn9 = assign26790_e24381_d_n9;
        locals.var_vdsatcv_dn10 = assign26790_e24381_d_n10;
        locals.var_vdsatcv_dn11 = assign26790_e24381_d_n11;
        locals.var_vdsatcv_dn12 = assign26790_e24381_d_n12;
        locals.var_vdsatcv_rv = 0.0;

        let (assign26800_e24389, assign26800_e24389_d_n3, assign26800_e24389_d_n4, assign26800_e24389_d_n5, assign26800_e24389_d_n6, assign26800_e24389_d_n7, assign26800_e24389_d_n8, assign26800_e24389_d_n9, assign26800_e24389_d_n10, assign26800_e24389_d_n11, assign26800_e24389_d_n12,) = {
    if (locals.var_guard1367 != 0.0) {
        let assign26800_e24385: f64 = (locals.var_vdsatcv - locals.var_vds_1);
        let assign26800_e24387: f64 = (assign26800_e24385 - 0.02);
        (assign26800_e24387, locals.var_vdsatcv_dn3, locals.var_vdsatcv_dn4, locals.var_vdsatcv_dn5, locals.var_vdsatcv_dn6, (locals.var_vdsatcv_dn7 - locals.var_vds_1_dn7), (locals.var_vdsatcv_dn8 - locals.var_vds_1_dn8), locals.var_vdsatcv_dn9, locals.var_vdsatcv_dn10, locals.var_vdsatcv_dn11, locals.var_vdsatcv_dn12,)
    } else {
        (locals.var_v4, locals.var_v4_dn3, locals.var_v4_dn4, locals.var_v4_dn5, locals.var_v4_dn6, locals.var_v4_dn7, locals.var_v4_dn8, locals.var_v4_dn9, locals.var_v4_dn10, locals.var_v4_dn11, locals.var_v4_dn12,)
    }
};
        locals.var_v4 = assign26800_e24389;
        locals.var_v4_dn3 = assign26800_e24389_d_n3;
        locals.var_v4_dn4 = assign26800_e24389_d_n4;
        locals.var_v4_dn5 = assign26800_e24389_d_n5;
        locals.var_v4_dn6 = assign26800_e24389_d_n6;
        locals.var_v4_dn7 = assign26800_e24389_d_n7;
        locals.var_v4_dn8 = assign26800_e24389_d_n8;
        locals.var_v4_dn9 = assign26800_e24389_d_n9;
        locals.var_v4_dn10 = assign26800_e24389_d_n10;
        locals.var_v4_dn11 = assign26800_e24389_d_n11;
        locals.var_v4_dn12 = assign26800_e24389_d_n12;
        locals.var_v4_rv = 0.0;

        let (assign26810_e24402, assign26810_e24402_d_n3, assign26810_e24402_d_n4, assign26810_e24402_d_n5, assign26810_e24402_d_n6, assign26810_e24402_d_n7, assign26810_e24402_d_n8, assign26810_e24402_d_n9, assign26810_e24402_d_n10, assign26810_e24402_d_n11, assign26810_e24402_d_n12,) = {
    if (locals.var_guard1367 != 0.0) {
        let assign26810_e24393: f64 = (locals.var_v4 * locals.var_v4);
        let assign26810_e24396: f64 = (4.0 * 0.02);
        let assign26810_e24398: f64 = (assign26810_e24396 * locals.var_vdsatcv);
        let assign26810_e24399: f64 = (assign26810_e24393 + assign26810_e24398);
        let assign26810_e24400: f64 = (assign26810_e24399).sqrt();
        (assign26810_e24400, ((((locals.var_v4_dn3 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn3)) + (assign26810_e24396 * locals.var_vdsatcv_dn3)) / (2.0 * assign26810_e24400)), ((((locals.var_v4_dn4 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn4)) + (assign26810_e24396 * locals.var_vdsatcv_dn4)) / (2.0 * assign26810_e24400)), ((((locals.var_v4_dn5 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn5)) + (assign26810_e24396 * locals.var_vdsatcv_dn5)) / (2.0 * assign26810_e24400)), ((((locals.var_v4_dn6 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn6)) + (assign26810_e24396 * locals.var_vdsatcv_dn6)) / (2.0 * assign26810_e24400)), ((((locals.var_v4_dn7 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn7)) + (assign26810_e24396 * locals.var_vdsatcv_dn7)) / (2.0 * assign26810_e24400)), ((((locals.var_v4_dn8 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn8)) + (assign26810_e24396 * locals.var_vdsatcv_dn8)) / (2.0 * assign26810_e24400)), ((((locals.var_v4_dn9 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn9)) + (assign26810_e24396 * locals.var_vdsatcv_dn9)) / (2.0 * assign26810_e24400)), ((((locals.var_v4_dn10 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn10)) + (assign26810_e24396 * locals.var_vdsatcv_dn10)) / (2.0 * assign26810_e24400)), ((((locals.var_v4_dn11 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn11)) + (assign26810_e24396 * locals.var_vdsatcv_dn11)) / (2.0 * assign26810_e24400)), ((((locals.var_v4_dn12 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn12)) + (assign26810_e24396 * locals.var_vdsatcv_dn12)) / (2.0 * assign26810_e24400)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign26810_e24402;
        locals.var_t0__blk808_dn3 = assign26810_e24402_d_n3;
        locals.var_t0__blk808_dn4 = assign26810_e24402_d_n4;
        locals.var_t0__blk808_dn5 = assign26810_e24402_d_n5;
        locals.var_t0__blk808_dn6 = assign26810_e24402_d_n6;
        locals.var_t0__blk808_dn7 = assign26810_e24402_d_n7;
        locals.var_t0__blk808_dn8 = assign26810_e24402_d_n8;
        locals.var_t0__blk808_dn9 = assign26810_e24402_d_n9;
        locals.var_t0__blk808_dn10 = assign26810_e24402_d_n10;
        locals.var_t0__blk808_dn11 = assign26810_e24402_d_n11;
        locals.var_t0__blk808_dn12 = assign26810_e24402_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign26820_e24412, assign26820_e24412_d_n3, assign26820_e24412_d_n4, assign26820_e24412_d_n5, assign26820_e24412_d_n6, assign26820_e24412_d_n7, assign26820_e24412_d_n8, assign26820_e24412_d_n9, assign26820_e24412_d_n10, assign26820_e24412_d_n11, assign26820_e24412_d_n12,) = {
    if (locals.var_guard1367 != 0.0) {
        let assign26820_e24408: f64 = (locals.var_v4 + locals.var_t0__blk808);
        let assign26820_e24409: f64 = (0.5 * assign26820_e24408);
        let assign26820_e24410: f64 = (locals.var_vdsatcv - assign26820_e24409);
        (assign26820_e24410, (locals.var_vdsatcv_dn3 - (0.5 * (locals.var_v4_dn3 + locals.var_t0__blk808_dn3))), (locals.var_vdsatcv_dn4 - (0.5 * (locals.var_v4_dn4 + locals.var_t0__blk808_dn4))), (locals.var_vdsatcv_dn5 - (0.5 * (locals.var_v4_dn5 + locals.var_t0__blk808_dn5))), (locals.var_vdsatcv_dn6 - (0.5 * (locals.var_v4_dn6 + locals.var_t0__blk808_dn6))), (locals.var_vdsatcv_dn7 - (0.5 * (locals.var_v4_dn7 + locals.var_t0__blk808_dn7))), (locals.var_vdsatcv_dn8 - (0.5 * (locals.var_v4_dn8 + locals.var_t0__blk808_dn8))), (locals.var_vdsatcv_dn9 - (0.5 * (locals.var_v4_dn9 + locals.var_t0__blk808_dn9))), (locals.var_vdsatcv_dn10 - (0.5 * (locals.var_v4_dn10 + locals.var_t0__blk808_dn10))), (locals.var_vdsatcv_dn11 - (0.5 * (locals.var_v4_dn11 + locals.var_t0__blk808_dn11))), (locals.var_vdsatcv_dn12 - (0.5 * (locals.var_v4_dn12 + locals.var_t0__blk808_dn12))),)
    } else {
        (locals.var_vdseffcv, locals.var_vdseffcv_dn3, locals.var_vdseffcv_dn4, locals.var_vdseffcv_dn5, locals.var_vdseffcv_dn6, locals.var_vdseffcv_dn7, locals.var_vdseffcv_dn8, locals.var_vdseffcv_dn9, locals.var_vdseffcv_dn10, locals.var_vdseffcv_dn11, locals.var_vdseffcv_dn12,)
    }
};
        locals.var_vdseffcv = assign26820_e24412;
        locals.var_vdseffcv_dn3 = assign26820_e24412_d_n3;
        locals.var_vdseffcv_dn4 = assign26820_e24412_d_n4;
        locals.var_vdseffcv_dn5 = assign26820_e24412_d_n5;
        locals.var_vdseffcv_dn6 = assign26820_e24412_d_n6;
        locals.var_vdseffcv_dn7 = assign26820_e24412_d_n7;
        locals.var_vdseffcv_dn8 = assign26820_e24412_d_n8;
        locals.var_vdseffcv_dn9 = assign26820_e24412_d_n9;
        locals.var_vdseffcv_dn10 = assign26820_e24412_d_n10;
        locals.var_vdseffcv_dn11 = assign26820_e24412_d_n11;
        locals.var_vdseffcv_dn12 = assign26820_e24412_d_n12;
        locals.var_vdseffcv_rv = 0.0;

        let assign26830_e24415: f64 = if p.p27 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1376 = assign26830_e24415;
        locals.var_guard1376_rv = 0.0;

        let (assign26840_e24423, assign26840_e24423_d_n3, assign26840_e24423_d_n4, assign26840_e24423_d_n5, assign26840_e24423_d_n6, assign26840_e24423_d_n7, assign26840_e24423_d_n8, assign26840_e24423_d_n9, assign26840_e24423_d_n10, assign26840_e24423_d_n11, assign26840_e24423_d_n12,) = {
    if ((locals.var_guard1367 != 0.0) && (locals.var_guard1376 != 0.0)) {
        let assign26840_e24421: f64 = (locals.var_vgsteff2 / locals.var_abulkcv);
        (assign26840_e24421, (((locals.var_vgsteff2_dn3 * locals.var_abulkcv) - (locals.var_vgsteff2 * locals.var_abulkcv_dn3)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff2_dn4 * locals.var_abulkcv) - (locals.var_vgsteff2 * locals.var_abulkcv_dn4)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff2_dn5 * locals.var_abulkcv) - (locals.var_vgsteff2 * locals.var_abulkcv_dn5)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff2_dn6 * locals.var_abulkcv) - (locals.var_vgsteff2 * locals.var_abulkcv_dn6)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff2_dn7 * locals.var_abulkcv) - (locals.var_vgsteff2 * locals.var_abulkcv_dn7)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff2_dn8 * locals.var_abulkcv) - (locals.var_vgsteff2 * locals.var_abulkcv_dn8)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff2_dn9 * locals.var_abulkcv) - (locals.var_vgsteff2 * locals.var_abulkcv_dn9)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff2_dn10 * locals.var_abulkcv) - (locals.var_vgsteff2 * locals.var_abulkcv_dn10)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff2_dn11 * locals.var_abulkcv) - (locals.var_vgsteff2 * locals.var_abulkcv_dn11)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_vgsteff2_dn12 * locals.var_abulkcv) - (locals.var_vgsteff2 * locals.var_abulkcv_dn12)) / (locals.var_abulkcv * locals.var_abulkcv)),)
    } else {
        (locals.var_vdsatcv2, locals.var_vdsatcv2_dn3, locals.var_vdsatcv2_dn4, locals.var_vdsatcv2_dn5, locals.var_vdsatcv2_dn6, locals.var_vdsatcv2_dn7, locals.var_vdsatcv2_dn8, locals.var_vdsatcv2_dn9, locals.var_vdsatcv2_dn10, locals.var_vdsatcv2_dn11, locals.var_vdsatcv2_dn12,)
    }
};
        locals.var_vdsatcv2 = assign26840_e24423;
        locals.var_vdsatcv2_dn3 = assign26840_e24423_d_n3;
        locals.var_vdsatcv2_dn4 = assign26840_e24423_d_n4;
        locals.var_vdsatcv2_dn5 = assign26840_e24423_d_n5;
        locals.var_vdsatcv2_dn6 = assign26840_e24423_d_n6;
        locals.var_vdsatcv2_dn7 = assign26840_e24423_d_n7;
        locals.var_vdsatcv2_dn8 = assign26840_e24423_d_n8;
        locals.var_vdsatcv2_dn9 = assign26840_e24423_d_n9;
        locals.var_vdsatcv2_dn10 = assign26840_e24423_d_n10;
        locals.var_vdsatcv2_dn11 = assign26840_e24423_d_n11;
        locals.var_vdsatcv2_dn12 = assign26840_e24423_d_n12;
        locals.var_vdsatcv2_rv = 0.0;

        let (assign26850_e24433, assign26850_e24433_d_n3, assign26850_e24433_d_n4, assign26850_e24433_d_n5, assign26850_e24433_d_n6, assign26850_e24433_d_n7, assign26850_e24433_d_n8, assign26850_e24433_d_n9, assign26850_e24433_d_n10, assign26850_e24433_d_n11, assign26850_e24433_d_n12,) = {
    if ((locals.var_guard1367 != 0.0) && (locals.var_guard1376 != 0.0)) {
        let assign26850_e24429: f64 = (locals.var_vdsatcv2 - locals.var_vds_1);
        let assign26850_e24431: f64 = (assign26850_e24429 - 0.02);
        (assign26850_e24431, locals.var_vdsatcv2_dn3, locals.var_vdsatcv2_dn4, locals.var_vdsatcv2_dn5, locals.var_vdsatcv2_dn6, (locals.var_vdsatcv2_dn7 - locals.var_vds_1_dn7), (locals.var_vdsatcv2_dn8 - locals.var_vds_1_dn8), locals.var_vdsatcv2_dn9, locals.var_vdsatcv2_dn10, locals.var_vdsatcv2_dn11, locals.var_vdsatcv2_dn12,)
    } else {
        (locals.var_v4, locals.var_v4_dn3, locals.var_v4_dn4, locals.var_v4_dn5, locals.var_v4_dn6, locals.var_v4_dn7, locals.var_v4_dn8, locals.var_v4_dn9, locals.var_v4_dn10, locals.var_v4_dn11, locals.var_v4_dn12,)
    }
};
        locals.var_v4 = assign26850_e24433;
        locals.var_v4_dn3 = assign26850_e24433_d_n3;
        locals.var_v4_dn4 = assign26850_e24433_d_n4;
        locals.var_v4_dn5 = assign26850_e24433_d_n5;
        locals.var_v4_dn6 = assign26850_e24433_d_n6;
        locals.var_v4_dn7 = assign26850_e24433_d_n7;
        locals.var_v4_dn8 = assign26850_e24433_d_n8;
        locals.var_v4_dn9 = assign26850_e24433_d_n9;
        locals.var_v4_dn10 = assign26850_e24433_d_n10;
        locals.var_v4_dn11 = assign26850_e24433_d_n11;
        locals.var_v4_dn12 = assign26850_e24433_d_n12;
        locals.var_v4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_81(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign26860_e24448, assign26860_e24448_d_n3, assign26860_e24448_d_n4, assign26860_e24448_d_n5, assign26860_e24448_d_n6, assign26860_e24448_d_n7, assign26860_e24448_d_n8, assign26860_e24448_d_n9, assign26860_e24448_d_n10, assign26860_e24448_d_n11, assign26860_e24448_d_n12,) = {
    if ((locals.var_guard1367 != 0.0) && (locals.var_guard1376 != 0.0)) {
        let assign26860_e24439: f64 = (locals.var_v4 * locals.var_v4);
        let assign26860_e24442: f64 = (4.0 * 0.02);
        let assign26860_e24444: f64 = (assign26860_e24442 * locals.var_vdsatcv2);
        let assign26860_e24445: f64 = (assign26860_e24439 + assign26860_e24444);
        let assign26860_e24446: f64 = (assign26860_e24445).sqrt();
        (assign26860_e24446, ((((locals.var_v4_dn3 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn3)) + (assign26860_e24442 * locals.var_vdsatcv2_dn3)) / (2.0 * assign26860_e24446)), ((((locals.var_v4_dn4 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn4)) + (assign26860_e24442 * locals.var_vdsatcv2_dn4)) / (2.0 * assign26860_e24446)), ((((locals.var_v4_dn5 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn5)) + (assign26860_e24442 * locals.var_vdsatcv2_dn5)) / (2.0 * assign26860_e24446)), ((((locals.var_v4_dn6 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn6)) + (assign26860_e24442 * locals.var_vdsatcv2_dn6)) / (2.0 * assign26860_e24446)), ((((locals.var_v4_dn7 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn7)) + (assign26860_e24442 * locals.var_vdsatcv2_dn7)) / (2.0 * assign26860_e24446)), ((((locals.var_v4_dn8 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn8)) + (assign26860_e24442 * locals.var_vdsatcv2_dn8)) / (2.0 * assign26860_e24446)), ((((locals.var_v4_dn9 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn9)) + (assign26860_e24442 * locals.var_vdsatcv2_dn9)) / (2.0 * assign26860_e24446)), ((((locals.var_v4_dn10 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn10)) + (assign26860_e24442 * locals.var_vdsatcv2_dn10)) / (2.0 * assign26860_e24446)), ((((locals.var_v4_dn11 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn11)) + (assign26860_e24442 * locals.var_vdsatcv2_dn11)) / (2.0 * assign26860_e24446)), ((((locals.var_v4_dn12 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn12)) + (assign26860_e24442 * locals.var_vdsatcv2_dn12)) / (2.0 * assign26860_e24446)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign26860_e24448;
        locals.var_t0__blk808_dn3 = assign26860_e24448_d_n3;
        locals.var_t0__blk808_dn4 = assign26860_e24448_d_n4;
        locals.var_t0__blk808_dn5 = assign26860_e24448_d_n5;
        locals.var_t0__blk808_dn6 = assign26860_e24448_d_n6;
        locals.var_t0__blk808_dn7 = assign26860_e24448_d_n7;
        locals.var_t0__blk808_dn8 = assign26860_e24448_d_n8;
        locals.var_t0__blk808_dn9 = assign26860_e24448_d_n9;
        locals.var_t0__blk808_dn10 = assign26860_e24448_d_n10;
        locals.var_t0__blk808_dn11 = assign26860_e24448_d_n11;
        locals.var_t0__blk808_dn12 = assign26860_e24448_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign26870_e24460, assign26870_e24460_d_n3, assign26870_e24460_d_n4, assign26870_e24460_d_n5, assign26870_e24460_d_n6, assign26870_e24460_d_n7, assign26870_e24460_d_n8, assign26870_e24460_d_n9, assign26870_e24460_d_n10, assign26870_e24460_d_n11, assign26870_e24460_d_n12,) = {
    if ((locals.var_guard1367 != 0.0) && (locals.var_guard1376 != 0.0)) {
        let assign26870_e24456: f64 = (locals.var_v4 + locals.var_t0__blk808);
        let assign26870_e24457: f64 = (0.5 * assign26870_e24456);
        let assign26870_e24458: f64 = (locals.var_vdsatcv2 - assign26870_e24457);
        (assign26870_e24458, (locals.var_vdsatcv2_dn3 - (0.5 * (locals.var_v4_dn3 + locals.var_t0__blk808_dn3))), (locals.var_vdsatcv2_dn4 - (0.5 * (locals.var_v4_dn4 + locals.var_t0__blk808_dn4))), (locals.var_vdsatcv2_dn5 - (0.5 * (locals.var_v4_dn5 + locals.var_t0__blk808_dn5))), (locals.var_vdsatcv2_dn6 - (0.5 * (locals.var_v4_dn6 + locals.var_t0__blk808_dn6))), (locals.var_vdsatcv2_dn7 - (0.5 * (locals.var_v4_dn7 + locals.var_t0__blk808_dn7))), (locals.var_vdsatcv2_dn8 - (0.5 * (locals.var_v4_dn8 + locals.var_t0__blk808_dn8))), (locals.var_vdsatcv2_dn9 - (0.5 * (locals.var_v4_dn9 + locals.var_t0__blk808_dn9))), (locals.var_vdsatcv2_dn10 - (0.5 * (locals.var_v4_dn10 + locals.var_t0__blk808_dn10))), (locals.var_vdsatcv2_dn11 - (0.5 * (locals.var_v4_dn11 + locals.var_t0__blk808_dn11))), (locals.var_vdsatcv2_dn12 - (0.5 * (locals.var_v4_dn12 + locals.var_t0__blk808_dn12))),)
    } else {
        (locals.var_vdseffcv2, locals.var_vdseffcv2_dn3, locals.var_vdseffcv2_dn4, locals.var_vdseffcv2_dn5, locals.var_vdseffcv2_dn6, locals.var_vdseffcv2_dn7, locals.var_vdseffcv2_dn8, locals.var_vdseffcv2_dn9, locals.var_vdseffcv2_dn10, locals.var_vdseffcv2_dn11, locals.var_vdseffcv2_dn12,)
    }
};
        locals.var_vdseffcv2 = assign26870_e24460;
        locals.var_vdseffcv2_dn3 = assign26870_e24460_d_n3;
        locals.var_vdseffcv2_dn4 = assign26870_e24460_d_n4;
        locals.var_vdseffcv2_dn5 = assign26870_e24460_d_n5;
        locals.var_vdseffcv2_dn6 = assign26870_e24460_d_n6;
        locals.var_vdseffcv2_dn7 = assign26870_e24460_d_n7;
        locals.var_vdseffcv2_dn8 = assign26870_e24460_d_n8;
        locals.var_vdseffcv2_dn9 = assign26870_e24460_d_n9;
        locals.var_vdseffcv2_dn10 = assign26870_e24460_d_n10;
        locals.var_vdseffcv2_dn11 = assign26870_e24460_d_n11;
        locals.var_vdseffcv2_dn12 = assign26870_e24460_d_n12;
        locals.var_vdseffcv2_rv = 0.0;

        let assign26880_e24463: f64 = if locals.var_b4soisoimod == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1377 = assign26880_e24463;
        locals.var_guard1377_rv = 0.0;

        let (assign26890_e24469, assign26890_e24469_d_n3, assign26890_e24469_d_n4, assign26890_e24469_d_n5, assign26890_e24469_d_n6, assign26890_e24469_d_n7, assign26890_e24469_d_n8, assign26890_e24469_d_n9, assign26890_e24469_d_n10, assign26890_e24469_d_n11, assign26890_e24469_d_n12,) = {
    if ((locals.var_guard1367 != 0.0) && (locals.var_guard1377 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbulk, locals.var_qbulk_dn3, locals.var_qbulk_dn4, locals.var_qbulk_dn5, locals.var_qbulk_dn6, locals.var_qbulk_dn7, locals.var_qbulk_dn8, locals.var_qbulk_dn9, locals.var_qbulk_dn10, locals.var_qbulk_dn11, locals.var_qbulk_dn12,)
    }
};
        locals.var_qbulk = assign26890_e24469;
        locals.var_qbulk_dn3 = assign26890_e24469_d_n3;
        locals.var_qbulk_dn4 = assign26890_e24469_d_n4;
        locals.var_qbulk_dn5 = assign26890_e24469_d_n5;
        locals.var_qbulk_dn6 = assign26890_e24469_d_n6;
        locals.var_qbulk_dn7 = assign26890_e24469_d_n7;
        locals.var_qbulk_dn8 = assign26890_e24469_d_n8;
        locals.var_qbulk_dn9 = assign26890_e24469_d_n9;
        locals.var_qbulk_dn10 = assign26890_e24469_d_n10;
        locals.var_qbulk_dn11 = assign26890_e24469_d_n11;
        locals.var_qbulk_dn12 = assign26890_e24469_d_n12;
        locals.var_qbulk_rv = 0.0;

        let (assign26900_e24478, assign26900_e24478_d_n3, assign26900_e24478_d_n4, assign26900_e24478_d_n5, assign26900_e24478_d_n6, assign26900_e24478_d_n7, assign26900_e24478_d_n8, assign26900_e24478_d_n9, assign26900_e24478_d_n10, assign26900_e24478_d_n11, assign26900_e24478_d_n12,) = {
    if ((locals.var_guard1367 != 0.0) && (locals.var_guard1377 == 0.0)) {
        let assign26900_e24476: f64 = (locals.var_abulkcv * locals.var_vdseffcv);
        (assign26900_e24476, ((locals.var_abulkcv_dn3 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn3)), ((locals.var_abulkcv_dn4 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn4)), ((locals.var_abulkcv_dn5 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn5)), ((locals.var_abulkcv_dn6 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn6)), ((locals.var_abulkcv_dn7 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn7)), ((locals.var_abulkcv_dn8 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn8)), ((locals.var_abulkcv_dn9 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn9)), ((locals.var_abulkcv_dn10 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn10)), ((locals.var_abulkcv_dn11 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn11)), ((locals.var_abulkcv_dn12 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn12)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign26900_e24478;
        locals.var_t0__blk808_dn3 = assign26900_e24478_d_n3;
        locals.var_t0__blk808_dn4 = assign26900_e24478_d_n4;
        locals.var_t0__blk808_dn5 = assign26900_e24478_d_n5;
        locals.var_t0__blk808_dn6 = assign26900_e24478_d_n6;
        locals.var_t0__blk808_dn7 = assign26900_e24478_d_n7;
        locals.var_t0__blk808_dn8 = assign26900_e24478_d_n8;
        locals.var_t0__blk808_dn9 = assign26900_e24478_d_n9;
        locals.var_t0__blk808_dn10 = assign26900_e24478_d_n10;
        locals.var_t0__blk808_dn11 = assign26900_e24478_d_n11;
        locals.var_t0__blk808_dn12 = assign26900_e24478_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign26910_e24493, assign26910_e24493_d_n3, assign26910_e24493_d_n4, assign26910_e24493_d_n5, assign26910_e24493_d_n6, assign26910_e24493_d_n7, assign26910_e24493_d_n8, assign26910_e24493_d_n9, assign26910_e24493_d_n10, assign26910_e24493_d_n11, assign26910_e24493_d_n12,) = {
    if ((locals.var_guard1367 != 0.0) && (locals.var_guard1377 == 0.0)) {
        let assign26910_e24487: f64 = (0.5 * locals.var_t0__blk808);
        let assign26910_e24488: f64 = (locals.var_vgsteff__blk840 - assign26910_e24487);
        let assign26910_e24490: f64 = (assign26910_e24488 + 1e-20);
        let assign26910_e24491: f64 = (12.0 * assign26910_e24490);
        (assign26910_e24491, (12.0 * (locals.var_vgsteff__blk840_dn3 - (0.5 * locals.var_t0__blk808_dn3))), (12.0 * (locals.var_vgsteff__blk840_dn4 - (0.5 * locals.var_t0__blk808_dn4))), (12.0 * (locals.var_vgsteff__blk840_dn5 - (0.5 * locals.var_t0__blk808_dn5))), (12.0 * (locals.var_vgsteff__blk840_dn6 - (0.5 * locals.var_t0__blk808_dn6))), (12.0 * (locals.var_vgsteff__blk840_dn7 - (0.5 * locals.var_t0__blk808_dn7))), (12.0 * (locals.var_vgsteff__blk840_dn8 - (0.5 * locals.var_t0__blk808_dn8))), (12.0 * (locals.var_vgsteff__blk840_dn9 - (0.5 * locals.var_t0__blk808_dn9))), (12.0 * (locals.var_vgsteff__blk840_dn10 - (0.5 * locals.var_t0__blk808_dn10))), (12.0 * (locals.var_vgsteff__blk840_dn11 - (0.5 * locals.var_t0__blk808_dn11))), (12.0 * (locals.var_vgsteff__blk840_dn12 - (0.5 * locals.var_t0__blk808_dn12))),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign26910_e24493;
        locals.var_t1__blk809_dn3 = assign26910_e24493_d_n3;
        locals.var_t1__blk809_dn4 = assign26910_e24493_d_n4;
        locals.var_t1__blk809_dn5 = assign26910_e24493_d_n5;
        locals.var_t1__blk809_dn6 = assign26910_e24493_d_n6;
        locals.var_t1__blk809_dn7 = assign26910_e24493_d_n7;
        locals.var_t1__blk809_dn8 = assign26910_e24493_d_n8;
        locals.var_t1__blk809_dn9 = assign26910_e24493_d_n9;
        locals.var_t1__blk809_dn10 = assign26910_e24493_d_n10;
        locals.var_t1__blk809_dn11 = assign26910_e24493_d_n11;
        locals.var_t1__blk809_dn12 = assign26910_e24493_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign26920_e24502, assign26920_e24502_d_n3, assign26920_e24502_d_n4, assign26920_e24502_d_n5, assign26920_e24502_d_n6, assign26920_e24502_d_n7, assign26920_e24502_d_n8, assign26920_e24502_d_n9, assign26920_e24502_d_n10, assign26920_e24502_d_n11, assign26920_e24502_d_n12,) = {
    if ((locals.var_guard1367 != 0.0) && (locals.var_guard1377 == 0.0)) {
        let assign26920_e24500: f64 = (locals.var_vdseffcv / locals.var_t1__blk809);
        (assign26920_e24500, (((locals.var_vdseffcv_dn3 * locals.var_t1__blk809) - (locals.var_vdseffcv * locals.var_t1__blk809_dn3)) / (locals.var_t1__blk809 * locals.var_t1__blk809)), (((locals.var_vdseffcv_dn4 * locals.var_t1__blk809) - (locals.var_vdseffcv * locals.var_t1__blk809_dn4)) / (locals.var_t1__blk809 * locals.var_t1__blk809)), (((locals.var_vdseffcv_dn5 * locals.var_t1__blk809) - (locals.var_vdseffcv * locals.var_t1__blk809_dn5)) / (locals.var_t1__blk809 * locals.var_t1__blk809)), (((locals.var_vdseffcv_dn6 * locals.var_t1__blk809) - (locals.var_vdseffcv * locals.var_t1__blk809_dn6)) / (locals.var_t1__blk809 * locals.var_t1__blk809)), (((locals.var_vdseffcv_dn7 * locals.var_t1__blk809) - (locals.var_vdseffcv * locals.var_t1__blk809_dn7)) / (locals.var_t1__blk809 * locals.var_t1__blk809)), (((locals.var_vdseffcv_dn8 * locals.var_t1__blk809) - (locals.var_vdseffcv * locals.var_t1__blk809_dn8)) / (locals.var_t1__blk809 * locals.var_t1__blk809)), (((locals.var_vdseffcv_dn9 * locals.var_t1__blk809) - (locals.var_vdseffcv * locals.var_t1__blk809_dn9)) / (locals.var_t1__blk809 * locals.var_t1__blk809)), (((locals.var_vdseffcv_dn10 * locals.var_t1__blk809) - (locals.var_vdseffcv * locals.var_t1__blk809_dn10)) / (locals.var_t1__blk809 * locals.var_t1__blk809)), (((locals.var_vdseffcv_dn11 * locals.var_t1__blk809) - (locals.var_vdseffcv * locals.var_t1__blk809_dn11)) / (locals.var_t1__blk809 * locals.var_t1__blk809)), (((locals.var_vdseffcv_dn12 * locals.var_t1__blk809) - (locals.var_vdseffcv * locals.var_t1__blk809_dn12)) / (locals.var_t1__blk809 * locals.var_t1__blk809)),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign26920_e24502;
        locals.var_t2__blk810_dn3 = assign26920_e24502_d_n3;
        locals.var_t2__blk810_dn4 = assign26920_e24502_d_n4;
        locals.var_t2__blk810_dn5 = assign26920_e24502_d_n5;
        locals.var_t2__blk810_dn6 = assign26920_e24502_d_n6;
        locals.var_t2__blk810_dn7 = assign26920_e24502_d_n7;
        locals.var_t2__blk810_dn8 = assign26920_e24502_d_n8;
        locals.var_t2__blk810_dn9 = assign26920_e24502_d_n9;
        locals.var_t2__blk810_dn10 = assign26920_e24502_d_n10;
        locals.var_t2__blk810_dn11 = assign26920_e24502_d_n11;
        locals.var_t2__blk810_dn12 = assign26920_e24502_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign26930_e24511, assign26930_e24511_d_n3, assign26930_e24511_d_n4, assign26930_e24511_d_n5, assign26930_e24511_d_n6, assign26930_e24511_d_n7, assign26930_e24511_d_n8, assign26930_e24511_d_n9, assign26930_e24511_d_n10, assign26930_e24511_d_n11, assign26930_e24511_d_n12,) = {
    if ((locals.var_guard1367 != 0.0) && (locals.var_guard1377 == 0.0)) {
        let assign26930_e24509: f64 = (locals.var_t0__blk808 * locals.var_t2__blk810);
        (assign26930_e24509, ((locals.var_t0__blk808_dn3 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn3)), ((locals.var_t0__blk808_dn4 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn4)), ((locals.var_t0__blk808_dn5 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn5)), ((locals.var_t0__blk808_dn6 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn6)), ((locals.var_t0__blk808_dn7 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn7)), ((locals.var_t0__blk808_dn8 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn8)), ((locals.var_t0__blk808_dn9 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn9)), ((locals.var_t0__blk808_dn10 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn10)), ((locals.var_t0__blk808_dn11 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn11)), ((locals.var_t0__blk808_dn12 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn12)),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign26930_e24511;
        locals.var_t3__blk811_dn3 = assign26930_e24511_d_n3;
        locals.var_t3__blk811_dn4 = assign26930_e24511_d_n4;
        locals.var_t3__blk811_dn5 = assign26930_e24511_d_n5;
        locals.var_t3__blk811_dn6 = assign26930_e24511_d_n6;
        locals.var_t3__blk811_dn7 = assign26930_e24511_d_n7;
        locals.var_t3__blk811_dn8 = assign26930_e24511_d_n8;
        locals.var_t3__blk811_dn9 = assign26930_e24511_d_n9;
        locals.var_t3__blk811_dn10 = assign26930_e24511_d_n10;
        locals.var_t3__blk811_dn11 = assign26930_e24511_d_n11;
        locals.var_t3__blk811_dn12 = assign26930_e24511_d_n12;
        locals.var_t3__blk811_rv = 0.0;

        let (assign26940_e24520, assign26940_e24520_d_n3, assign26940_e24520_d_n4, assign26940_e24520_d_n5, assign26940_e24520_d_n6, assign26940_e24520_d_n7, assign26940_e24520_d_n8, assign26940_e24520_d_n9, assign26940_e24520_d_n10, assign26940_e24520_d_n11, assign26940_e24520_d_n12,) = {
    if ((locals.var_guard1367 != 0.0) && (locals.var_guard1377 == 0.0)) {
        let assign26940_e24518: f64 = (1.0 - locals.var_abulkcv);
        (assign26940_e24518, (-locals.var_abulkcv_dn3), (-locals.var_abulkcv_dn4), (-locals.var_abulkcv_dn5), (-locals.var_abulkcv_dn6), (-locals.var_abulkcv_dn7), (-locals.var_abulkcv_dn8), (-locals.var_abulkcv_dn9), (-locals.var_abulkcv_dn10), (-locals.var_abulkcv_dn11), (-locals.var_abulkcv_dn12),)
    } else {
        (locals.var_t7__blk815, locals.var_t7__blk815_dn3, locals.var_t7__blk815_dn4, locals.var_t7__blk815_dn5, locals.var_t7__blk815_dn6, locals.var_t7__blk815_dn7, locals.var_t7__blk815_dn8, locals.var_t7__blk815_dn9, locals.var_t7__blk815_dn10, locals.var_t7__blk815_dn11, locals.var_t7__blk815_dn12,)
    }
};
        locals.var_t7__blk815 = assign26940_e24520;
        locals.var_t7__blk815_dn3 = assign26940_e24520_d_n3;
        locals.var_t7__blk815_dn4 = assign26940_e24520_d_n4;
        locals.var_t7__blk815_dn5 = assign26940_e24520_d_n5;
        locals.var_t7__blk815_dn6 = assign26940_e24520_d_n6;
        locals.var_t7__blk815_dn7 = assign26940_e24520_d_n7;
        locals.var_t7__blk815_dn8 = assign26940_e24520_d_n8;
        locals.var_t7__blk815_dn9 = assign26940_e24520_d_n9;
        locals.var_t7__blk815_dn10 = assign26940_e24520_d_n10;
        locals.var_t7__blk815_dn11 = assign26940_e24520_d_n11;
        locals.var_t7__blk815_dn12 = assign26940_e24520_d_n12;
        locals.var_t7__blk815_rv = 0.0;

        let (assign26950_e24535, assign26950_e24535_d_n3, assign26950_e24535_d_n4, assign26950_e24535_d_n5, assign26950_e24535_d_n6, assign26950_e24535_d_n7, assign26950_e24535_d_n8, assign26950_e24535_d_n9, assign26950_e24535_d_n10, assign26950_e24535_d_n11, assign26950_e24535_d_n12,) = {
    if ((locals.var_guard1367 != 0.0) && (locals.var_guard1377 == 0.0)) {
        let assign26950_e24527: f64 = (locals.var_coxwlb * locals.var_t7__blk815);
        let assign26950_e24530: f64 = (0.5 * locals.var_vdseffcv);
        let assign26950_e24532: f64 = (assign26950_e24530 - locals.var_t3__blk811);
        let assign26950_e24533: f64 = (assign26950_e24527 * assign26950_e24532);
        (assign26950_e24533, ((((locals.var_coxwlb_dn3 * locals.var_t7__blk815) + (locals.var_coxwlb * locals.var_t7__blk815_dn3)) * assign26950_e24532) + (assign26950_e24527 * ((0.5 * locals.var_vdseffcv_dn3) - locals.var_t3__blk811_dn3))), ((((locals.var_coxwlb_dn4 * locals.var_t7__blk815) + (locals.var_coxwlb * locals.var_t7__blk815_dn4)) * assign26950_e24532) + (assign26950_e24527 * ((0.5 * locals.var_vdseffcv_dn4) - locals.var_t3__blk811_dn4))), ((((locals.var_coxwlb_dn5 * locals.var_t7__blk815) + (locals.var_coxwlb * locals.var_t7__blk815_dn5)) * assign26950_e24532) + (assign26950_e24527 * ((0.5 * locals.var_vdseffcv_dn5) - locals.var_t3__blk811_dn5))), ((((locals.var_coxwlb_dn6 * locals.var_t7__blk815) + (locals.var_coxwlb * locals.var_t7__blk815_dn6)) * assign26950_e24532) + (assign26950_e24527 * ((0.5 * locals.var_vdseffcv_dn6) - locals.var_t3__blk811_dn6))), ((((locals.var_coxwlb_dn7 * locals.var_t7__blk815) + (locals.var_coxwlb * locals.var_t7__blk815_dn7)) * assign26950_e24532) + (assign26950_e24527 * ((0.5 * locals.var_vdseffcv_dn7) - locals.var_t3__blk811_dn7))), ((((locals.var_coxwlb_dn8 * locals.var_t7__blk815) + (locals.var_coxwlb * locals.var_t7__blk815_dn8)) * assign26950_e24532) + (assign26950_e24527 * ((0.5 * locals.var_vdseffcv_dn8) - locals.var_t3__blk811_dn8))), ((((locals.var_coxwlb_dn9 * locals.var_t7__blk815) + (locals.var_coxwlb * locals.var_t7__blk815_dn9)) * assign26950_e24532) + (assign26950_e24527 * ((0.5 * locals.var_vdseffcv_dn9) - locals.var_t3__blk811_dn9))), ((((locals.var_coxwlb_dn10 * locals.var_t7__blk815) + (locals.var_coxwlb * locals.var_t7__blk815_dn10)) * assign26950_e24532) + (assign26950_e24527 * ((0.5 * locals.var_vdseffcv_dn10) - locals.var_t3__blk811_dn10))), ((((locals.var_coxwlb_dn11 * locals.var_t7__blk815) + (locals.var_coxwlb * locals.var_t7__blk815_dn11)) * assign26950_e24532) + (assign26950_e24527 * ((0.5 * locals.var_vdseffcv_dn11) - locals.var_t3__blk811_dn11))), ((((locals.var_coxwlb_dn12 * locals.var_t7__blk815) + (locals.var_coxwlb * locals.var_t7__blk815_dn12)) * assign26950_e24532) + (assign26950_e24527 * ((0.5 * locals.var_vdseffcv_dn12) - locals.var_t3__blk811_dn12))),)
    } else {
        (locals.var_qbulk, locals.var_qbulk_dn3, locals.var_qbulk_dn4, locals.var_qbulk_dn5, locals.var_qbulk_dn6, locals.var_qbulk_dn7, locals.var_qbulk_dn8, locals.var_qbulk_dn9, locals.var_qbulk_dn10, locals.var_qbulk_dn11, locals.var_qbulk_dn12,)
    }
};
        locals.var_qbulk = assign26950_e24535;
        locals.var_qbulk_dn3 = assign26950_e24535_d_n3;
        locals.var_qbulk_dn4 = assign26950_e24535_d_n4;
        locals.var_qbulk_dn5 = assign26950_e24535_d_n5;
        locals.var_qbulk_dn6 = assign26950_e24535_d_n6;
        locals.var_qbulk_dn7 = assign26950_e24535_d_n7;
        locals.var_qbulk_dn8 = assign26950_e24535_d_n8;
        locals.var_qbulk_dn9 = assign26950_e24535_d_n9;
        locals.var_qbulk_dn10 = assign26950_e24535_d_n10;
        locals.var_qbulk_dn11 = assign26950_e24535_d_n11;
        locals.var_qbulk_dn12 = assign26950_e24535_d_n12;
        locals.var_qbulk_rv = 0.0;

        let assign26960_e24546: f64 = if (((locals.var_b4soisoimod != 2.0) && (locals.var_b4soibodymod != 0.0)) && (p.p27 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1378 = assign26960_e24546;
        locals.var_guard1378_rv = 0.0;

        let (assign26970_e24557, assign26970_e24557_d_n3, assign26970_e24557_d_n4, assign26970_e24557_d_n5, assign26970_e24557_d_n6, assign26970_e24557_d_n7, assign26970_e24557_d_n8, assign26970_e24557_d_n9, assign26970_e24557_d_n10, assign26970_e24557_d_n11, assign26970_e24557_d_n12,) = {
    if (((locals.var_guard1367 != 0.0) && (locals.var_guard1377 == 0.0)) && (locals.var_guard1378 != 0.0)) {
        let assign26970_e24555: f64 = (locals.var_abulkcv * locals.var_vdseffcv2);
        (assign26970_e24555, ((locals.var_abulkcv_dn3 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn3)), ((locals.var_abulkcv_dn4 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn4)), ((locals.var_abulkcv_dn5 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn5)), ((locals.var_abulkcv_dn6 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn6)), ((locals.var_abulkcv_dn7 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn7)), ((locals.var_abulkcv_dn8 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn8)), ((locals.var_abulkcv_dn9 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn9)), ((locals.var_abulkcv_dn10 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn10)), ((locals.var_abulkcv_dn11 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn11)), ((locals.var_abulkcv_dn12 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn12)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign26970_e24557;
        locals.var_t0__blk808_dn3 = assign26970_e24557_d_n3;
        locals.var_t0__blk808_dn4 = assign26970_e24557_d_n4;
        locals.var_t0__blk808_dn5 = assign26970_e24557_d_n5;
        locals.var_t0__blk808_dn6 = assign26970_e24557_d_n6;
        locals.var_t0__blk808_dn7 = assign26970_e24557_d_n7;
        locals.var_t0__blk808_dn8 = assign26970_e24557_d_n8;
        locals.var_t0__blk808_dn9 = assign26970_e24557_d_n9;
        locals.var_t0__blk808_dn10 = assign26970_e24557_d_n10;
        locals.var_t0__blk808_dn11 = assign26970_e24557_d_n11;
        locals.var_t0__blk808_dn12 = assign26970_e24557_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign26980_e24574, assign26980_e24574_d_n3, assign26980_e24574_d_n4, assign26980_e24574_d_n5, assign26980_e24574_d_n6, assign26980_e24574_d_n7, assign26980_e24574_d_n8, assign26980_e24574_d_n9, assign26980_e24574_d_n10, assign26980_e24574_d_n11, assign26980_e24574_d_n12,) = {
    if (((locals.var_guard1367 != 0.0) && (locals.var_guard1377 == 0.0)) && (locals.var_guard1378 != 0.0)) {
        let assign26980_e24568: f64 = (0.5 * locals.var_t0__blk808);
        let assign26980_e24569: f64 = (locals.var_vgsteff2 - assign26980_e24568);
        let assign26980_e24571: f64 = (assign26980_e24569 + 1e-20);
        let assign26980_e24572: f64 = (12.0 * assign26980_e24571);
        (assign26980_e24572, (12.0 * (locals.var_vgsteff2_dn3 - (0.5 * locals.var_t0__blk808_dn3))), (12.0 * (locals.var_vgsteff2_dn4 - (0.5 * locals.var_t0__blk808_dn4))), (12.0 * (locals.var_vgsteff2_dn5 - (0.5 * locals.var_t0__blk808_dn5))), (12.0 * (locals.var_vgsteff2_dn6 - (0.5 * locals.var_t0__blk808_dn6))), (12.0 * (locals.var_vgsteff2_dn7 - (0.5 * locals.var_t0__blk808_dn7))), (12.0 * (locals.var_vgsteff2_dn8 - (0.5 * locals.var_t0__blk808_dn8))), (12.0 * (locals.var_vgsteff2_dn9 - (0.5 * locals.var_t0__blk808_dn9))), (12.0 * (locals.var_vgsteff2_dn10 - (0.5 * locals.var_t0__blk808_dn10))), (12.0 * (locals.var_vgsteff2_dn11 - (0.5 * locals.var_t0__blk808_dn11))), (12.0 * (locals.var_vgsteff2_dn12 - (0.5 * locals.var_t0__blk808_dn12))),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign26980_e24574;
        locals.var_t1__blk809_dn3 = assign26980_e24574_d_n3;
        locals.var_t1__blk809_dn4 = assign26980_e24574_d_n4;
        locals.var_t1__blk809_dn5 = assign26980_e24574_d_n5;
        locals.var_t1__blk809_dn6 = assign26980_e24574_d_n6;
        locals.var_t1__blk809_dn7 = assign26980_e24574_d_n7;
        locals.var_t1__blk809_dn8 = assign26980_e24574_d_n8;
        locals.var_t1__blk809_dn9 = assign26980_e24574_d_n9;
        locals.var_t1__blk809_dn10 = assign26980_e24574_d_n10;
        locals.var_t1__blk809_dn11 = assign26980_e24574_d_n11;
        locals.var_t1__blk809_dn12 = assign26980_e24574_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign26990_e24585, assign26990_e24585_d_n3, assign26990_e24585_d_n4, assign26990_e24585_d_n5, assign26990_e24585_d_n6, assign26990_e24585_d_n7, assign26990_e24585_d_n8, assign26990_e24585_d_n9, assign26990_e24585_d_n10, assign26990_e24585_d_n11, assign26990_e24585_d_n12,) = {
    if (((locals.var_guard1367 != 0.0) && (locals.var_guard1377 == 0.0)) && (locals.var_guard1378 != 0.0)) {
        let assign26990_e24583: f64 = (locals.var_vdseffcv2 / locals.var_t1__blk809);
        (assign26990_e24583, (((locals.var_vdseffcv2_dn3 * locals.var_t1__blk809) - (locals.var_vdseffcv2 * locals.var_t1__blk809_dn3)) / (locals.var_t1__blk809 * locals.var_t1__blk809)), (((locals.var_vdseffcv2_dn4 * locals.var_t1__blk809) - (locals.var_vdseffcv2 * locals.var_t1__blk809_dn4)) / (locals.var_t1__blk809 * locals.var_t1__blk809)), (((locals.var_vdseffcv2_dn5 * locals.var_t1__blk809) - (locals.var_vdseffcv2 * locals.var_t1__blk809_dn5)) / (locals.var_t1__blk809 * locals.var_t1__blk809)), (((locals.var_vdseffcv2_dn6 * locals.var_t1__blk809) - (locals.var_vdseffcv2 * locals.var_t1__blk809_dn6)) / (locals.var_t1__blk809 * locals.var_t1__blk809)), (((locals.var_vdseffcv2_dn7 * locals.var_t1__blk809) - (locals.var_vdseffcv2 * locals.var_t1__blk809_dn7)) / (locals.var_t1__blk809 * locals.var_t1__blk809)), (((locals.var_vdseffcv2_dn8 * locals.var_t1__blk809) - (locals.var_vdseffcv2 * locals.var_t1__blk809_dn8)) / (locals.var_t1__blk809 * locals.var_t1__blk809)), (((locals.var_vdseffcv2_dn9 * locals.var_t1__blk809) - (locals.var_vdseffcv2 * locals.var_t1__blk809_dn9)) / (locals.var_t1__blk809 * locals.var_t1__blk809)), (((locals.var_vdseffcv2_dn10 * locals.var_t1__blk809) - (locals.var_vdseffcv2 * locals.var_t1__blk809_dn10)) / (locals.var_t1__blk809 * locals.var_t1__blk809)), (((locals.var_vdseffcv2_dn11 * locals.var_t1__blk809) - (locals.var_vdseffcv2 * locals.var_t1__blk809_dn11)) / (locals.var_t1__blk809 * locals.var_t1__blk809)), (((locals.var_vdseffcv2_dn12 * locals.var_t1__blk809) - (locals.var_vdseffcv2 * locals.var_t1__blk809_dn12)) / (locals.var_t1__blk809 * locals.var_t1__blk809)),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign26990_e24585;
        locals.var_t2__blk810_dn3 = assign26990_e24585_d_n3;
        locals.var_t2__blk810_dn4 = assign26990_e24585_d_n4;
        locals.var_t2__blk810_dn5 = assign26990_e24585_d_n5;
        locals.var_t2__blk810_dn6 = assign26990_e24585_d_n6;
        locals.var_t2__blk810_dn7 = assign26990_e24585_d_n7;
        locals.var_t2__blk810_dn8 = assign26990_e24585_d_n8;
        locals.var_t2__blk810_dn9 = assign26990_e24585_d_n9;
        locals.var_t2__blk810_dn10 = assign26990_e24585_d_n10;
        locals.var_t2__blk810_dn11 = assign26990_e24585_d_n11;
        locals.var_t2__blk810_dn12 = assign26990_e24585_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign27000_e24596, assign27000_e24596_d_n3, assign27000_e24596_d_n4, assign27000_e24596_d_n5, assign27000_e24596_d_n6, assign27000_e24596_d_n7, assign27000_e24596_d_n8, assign27000_e24596_d_n9, assign27000_e24596_d_n10, assign27000_e24596_d_n11, assign27000_e24596_d_n12,) = {
    if (((locals.var_guard1367 != 0.0) && (locals.var_guard1377 == 0.0)) && (locals.var_guard1378 != 0.0)) {
        let assign27000_e24594: f64 = (locals.var_t0__blk808 * locals.var_t2__blk810);
        (assign27000_e24594, ((locals.var_t0__blk808_dn3 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn3)), ((locals.var_t0__blk808_dn4 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn4)), ((locals.var_t0__blk808_dn5 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn5)), ((locals.var_t0__blk808_dn6 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn6)), ((locals.var_t0__blk808_dn7 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn7)), ((locals.var_t0__blk808_dn8 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn8)), ((locals.var_t0__blk808_dn9 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn9)), ((locals.var_t0__blk808_dn10 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn10)), ((locals.var_t0__blk808_dn11 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn11)), ((locals.var_t0__blk808_dn12 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn12)),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign27000_e24596;
        locals.var_t3__blk811_dn3 = assign27000_e24596_d_n3;
        locals.var_t3__blk811_dn4 = assign27000_e24596_d_n4;
        locals.var_t3__blk811_dn5 = assign27000_e24596_d_n5;
        locals.var_t3__blk811_dn6 = assign27000_e24596_d_n6;
        locals.var_t3__blk811_dn7 = assign27000_e24596_d_n7;
        locals.var_t3__blk811_dn8 = assign27000_e24596_d_n8;
        locals.var_t3__blk811_dn9 = assign27000_e24596_d_n9;
        locals.var_t3__blk811_dn10 = assign27000_e24596_d_n10;
        locals.var_t3__blk811_dn11 = assign27000_e24596_d_n11;
        locals.var_t3__blk811_dn12 = assign27000_e24596_d_n12;
        locals.var_t3__blk811_rv = 0.0;

        let (assign27010_e24607, assign27010_e24607_d_n3, assign27010_e24607_d_n4, assign27010_e24607_d_n5, assign27010_e24607_d_n6, assign27010_e24607_d_n7, assign27010_e24607_d_n8, assign27010_e24607_d_n9, assign27010_e24607_d_n10, assign27010_e24607_d_n11, assign27010_e24607_d_n12,) = {
    if (((locals.var_guard1367 != 0.0) && (locals.var_guard1377 == 0.0)) && (locals.var_guard1378 != 0.0)) {
        let assign27010_e24605: f64 = (1.0 - locals.var_abulkcv);
        (assign27010_e24605, (-locals.var_abulkcv_dn3), (-locals.var_abulkcv_dn4), (-locals.var_abulkcv_dn5), (-locals.var_abulkcv_dn6), (-locals.var_abulkcv_dn7), (-locals.var_abulkcv_dn8), (-locals.var_abulkcv_dn9), (-locals.var_abulkcv_dn10), (-locals.var_abulkcv_dn11), (-locals.var_abulkcv_dn12),)
    } else {
        (locals.var_t7__blk815, locals.var_t7__blk815_dn3, locals.var_t7__blk815_dn4, locals.var_t7__blk815_dn5, locals.var_t7__blk815_dn6, locals.var_t7__blk815_dn7, locals.var_t7__blk815_dn8, locals.var_t7__blk815_dn9, locals.var_t7__blk815_dn10, locals.var_t7__blk815_dn11, locals.var_t7__blk815_dn12,)
    }
};
        locals.var_t7__blk815 = assign27010_e24607;
        locals.var_t7__blk815_dn3 = assign27010_e24607_d_n3;
        locals.var_t7__blk815_dn4 = assign27010_e24607_d_n4;
        locals.var_t7__blk815_dn5 = assign27010_e24607_d_n5;
        locals.var_t7__blk815_dn6 = assign27010_e24607_d_n6;
        locals.var_t7__blk815_dn7 = assign27010_e24607_d_n7;
        locals.var_t7__blk815_dn8 = assign27010_e24607_d_n8;
        locals.var_t7__blk815_dn9 = assign27010_e24607_d_n9;
        locals.var_t7__blk815_dn10 = assign27010_e24607_d_n10;
        locals.var_t7__blk815_dn11 = assign27010_e24607_d_n11;
        locals.var_t7__blk815_dn12 = assign27010_e24607_d_n12;
        locals.var_t7__blk815_rv = 0.0;

        let (assign27020_e24626, assign27020_e24626_d_n3, assign27020_e24626_d_n4, assign27020_e24626_d_n5, assign27020_e24626_d_n6, assign27020_e24626_d_n7, assign27020_e24626_d_n8, assign27020_e24626_d_n9, assign27020_e24626_d_n10, assign27020_e24626_d_n11, assign27020_e24626_d_n12,) = {
    if (((locals.var_guard1367 != 0.0) && (locals.var_guard1377 == 0.0)) && (locals.var_guard1378 != 0.0)) {
        let assign27020_e24617: f64 = (locals.var_coxwlb2 * locals.var_t7__blk815);
        let assign27020_e24620: f64 = (0.5 * locals.var_vdseffcv2);
        let assign27020_e24622: f64 = (assign27020_e24620 - locals.var_t3__blk811);
        let assign27020_e24623: f64 = (assign27020_e24617 * assign27020_e24622);
        let assign27020_e24624: f64 = (locals.var_qbulk + assign27020_e24623);
        (assign27020_e24624, (locals.var_qbulk_dn3 + ((((locals.var_coxwlb2_dn3 * locals.var_t7__blk815) + (locals.var_coxwlb2 * locals.var_t7__blk815_dn3)) * assign27020_e24622) + (assign27020_e24617 * ((0.5 * locals.var_vdseffcv2_dn3) - locals.var_t3__blk811_dn3)))), (locals.var_qbulk_dn4 + ((((locals.var_coxwlb2_dn4 * locals.var_t7__blk815) + (locals.var_coxwlb2 * locals.var_t7__blk815_dn4)) * assign27020_e24622) + (assign27020_e24617 * ((0.5 * locals.var_vdseffcv2_dn4) - locals.var_t3__blk811_dn4)))), (locals.var_qbulk_dn5 + ((((locals.var_coxwlb2_dn5 * locals.var_t7__blk815) + (locals.var_coxwlb2 * locals.var_t7__blk815_dn5)) * assign27020_e24622) + (assign27020_e24617 * ((0.5 * locals.var_vdseffcv2_dn5) - locals.var_t3__blk811_dn5)))), (locals.var_qbulk_dn6 + ((((locals.var_coxwlb2_dn6 * locals.var_t7__blk815) + (locals.var_coxwlb2 * locals.var_t7__blk815_dn6)) * assign27020_e24622) + (assign27020_e24617 * ((0.5 * locals.var_vdseffcv2_dn6) - locals.var_t3__blk811_dn6)))), (locals.var_qbulk_dn7 + ((((locals.var_coxwlb2_dn7 * locals.var_t7__blk815) + (locals.var_coxwlb2 * locals.var_t7__blk815_dn7)) * assign27020_e24622) + (assign27020_e24617 * ((0.5 * locals.var_vdseffcv2_dn7) - locals.var_t3__blk811_dn7)))), (locals.var_qbulk_dn8 + ((((locals.var_coxwlb2_dn8 * locals.var_t7__blk815) + (locals.var_coxwlb2 * locals.var_t7__blk815_dn8)) * assign27020_e24622) + (assign27020_e24617 * ((0.5 * locals.var_vdseffcv2_dn8) - locals.var_t3__blk811_dn8)))), (locals.var_qbulk_dn9 + ((((locals.var_coxwlb2_dn9 * locals.var_t7__blk815) + (locals.var_coxwlb2 * locals.var_t7__blk815_dn9)) * assign27020_e24622) + (assign27020_e24617 * ((0.5 * locals.var_vdseffcv2_dn9) - locals.var_t3__blk811_dn9)))), (locals.var_qbulk_dn10 + ((((locals.var_coxwlb2_dn10 * locals.var_t7__blk815) + (locals.var_coxwlb2 * locals.var_t7__blk815_dn10)) * assign27020_e24622) + (assign27020_e24617 * ((0.5 * locals.var_vdseffcv2_dn10) - locals.var_t3__blk811_dn10)))), (locals.var_qbulk_dn11 + ((((locals.var_coxwlb2_dn11 * locals.var_t7__blk815) + (locals.var_coxwlb2 * locals.var_t7__blk815_dn11)) * assign27020_e24622) + (assign27020_e24617 * ((0.5 * locals.var_vdseffcv2_dn11) - locals.var_t3__blk811_dn11)))), (locals.var_qbulk_dn12 + ((((locals.var_coxwlb2_dn12 * locals.var_t7__blk815) + (locals.var_coxwlb2 * locals.var_t7__blk815_dn12)) * assign27020_e24622) + (assign27020_e24617 * ((0.5 * locals.var_vdseffcv2_dn12) - locals.var_t3__blk811_dn12)))),)
    } else {
        (locals.var_qbulk, locals.var_qbulk_dn3, locals.var_qbulk_dn4, locals.var_qbulk_dn5, locals.var_qbulk_dn6, locals.var_qbulk_dn7, locals.var_qbulk_dn8, locals.var_qbulk_dn9, locals.var_qbulk_dn10, locals.var_qbulk_dn11, locals.var_qbulk_dn12,)
    }
};
        locals.var_qbulk = assign27020_e24626;
        locals.var_qbulk_dn3 = assign27020_e24626_d_n3;
        locals.var_qbulk_dn4 = assign27020_e24626_d_n4;
        locals.var_qbulk_dn5 = assign27020_e24626_d_n5;
        locals.var_qbulk_dn6 = assign27020_e24626_d_n6;
        locals.var_qbulk_dn7 = assign27020_e24626_d_n7;
        locals.var_qbulk_dn8 = assign27020_e24626_d_n8;
        locals.var_qbulk_dn9 = assign27020_e24626_d_n9;
        locals.var_qbulk_dn10 = assign27020_e24626_d_n10;
        locals.var_qbulk_dn11 = assign27020_e24626_d_n11;
        locals.var_qbulk_dn12 = assign27020_e24626_d_n12;
        locals.var_qbulk_rv = 0.0;

        let (assign27030_e24632, assign27030_e24632_d_n3, assign27030_e24632_d_n4, assign27030_e24632_d_n5, assign27030_e24632_d_n6, assign27030_e24632_d_n7, assign27030_e24632_d_n8, assign27030_e24632_d_n9, assign27030_e24632_d_n10, assign27030_e24632_d_n11, assign27030_e24632_d_n12,) = {
    if (locals.var_guard1367 != 0.0) {
        let assign27030_e24630: f64 = (locals.var_abulkcv * locals.var_vdseffcv);
        (assign27030_e24630, ((locals.var_abulkcv_dn3 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn3)), ((locals.var_abulkcv_dn4 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn4)), ((locals.var_abulkcv_dn5 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn5)), ((locals.var_abulkcv_dn6 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn6)), ((locals.var_abulkcv_dn7 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn7)), ((locals.var_abulkcv_dn8 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn8)), ((locals.var_abulkcv_dn9 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn9)), ((locals.var_abulkcv_dn10 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn10)), ((locals.var_abulkcv_dn11 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn11)), ((locals.var_abulkcv_dn12 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn12)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign27030_e24632;
        locals.var_t0__blk808_dn3 = assign27030_e24632_d_n3;
        locals.var_t0__blk808_dn4 = assign27030_e24632_d_n4;
        locals.var_t0__blk808_dn5 = assign27030_e24632_d_n5;
        locals.var_t0__blk808_dn6 = assign27030_e24632_d_n6;
        locals.var_t0__blk808_dn7 = assign27030_e24632_d_n7;
        locals.var_t0__blk808_dn8 = assign27030_e24632_d_n8;
        locals.var_t0__blk808_dn9 = assign27030_e24632_d_n9;
        locals.var_t0__blk808_dn10 = assign27030_e24632_d_n10;
        locals.var_t0__blk808_dn11 = assign27030_e24632_d_n11;
        locals.var_t0__blk808_dn12 = assign27030_e24632_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign27040_e24644, assign27040_e24644_d_n3, assign27040_e24644_d_n4, assign27040_e24644_d_n5, assign27040_e24644_d_n6, assign27040_e24644_d_n7, assign27040_e24644_d_n8, assign27040_e24644_d_n9, assign27040_e24644_d_n10, assign27040_e24644_d_n11, assign27040_e24644_d_n12,) = {
    if (locals.var_guard1367 != 0.0) {
        let assign27040_e24638: f64 = (0.5 * locals.var_t0__blk808);
        let assign27040_e24639: f64 = (locals.var_vgsteff__blk840 - assign27040_e24638);
        let assign27040_e24641: f64 = (assign27040_e24639 + 1e-20);
        let assign27040_e24642: f64 = (12.0 * assign27040_e24641);
        (assign27040_e24642, (12.0 * (locals.var_vgsteff__blk840_dn3 - (0.5 * locals.var_t0__blk808_dn3))), (12.0 * (locals.var_vgsteff__blk840_dn4 - (0.5 * locals.var_t0__blk808_dn4))), (12.0 * (locals.var_vgsteff__blk840_dn5 - (0.5 * locals.var_t0__blk808_dn5))), (12.0 * (locals.var_vgsteff__blk840_dn6 - (0.5 * locals.var_t0__blk808_dn6))), (12.0 * (locals.var_vgsteff__blk840_dn7 - (0.5 * locals.var_t0__blk808_dn7))), (12.0 * (locals.var_vgsteff__blk840_dn8 - (0.5 * locals.var_t0__blk808_dn8))), (12.0 * (locals.var_vgsteff__blk840_dn9 - (0.5 * locals.var_t0__blk808_dn9))), (12.0 * (locals.var_vgsteff__blk840_dn10 - (0.5 * locals.var_t0__blk808_dn10))), (12.0 * (locals.var_vgsteff__blk840_dn11 - (0.5 * locals.var_t0__blk808_dn11))), (12.0 * (locals.var_vgsteff__blk840_dn12 - (0.5 * locals.var_t0__blk808_dn12))),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign27040_e24644;
        locals.var_t1__blk809_dn3 = assign27040_e24644_d_n3;
        locals.var_t1__blk809_dn4 = assign27040_e24644_d_n4;
        locals.var_t1__blk809_dn5 = assign27040_e24644_d_n5;
        locals.var_t1__blk809_dn6 = assign27040_e24644_d_n6;
        locals.var_t1__blk809_dn7 = assign27040_e24644_d_n7;
        locals.var_t1__blk809_dn8 = assign27040_e24644_d_n8;
        locals.var_t1__blk809_dn9 = assign27040_e24644_d_n9;
        locals.var_t1__blk809_dn10 = assign27040_e24644_d_n10;
        locals.var_t1__blk809_dn11 = assign27040_e24644_d_n11;
        locals.var_t1__blk809_dn12 = assign27040_e24644_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign27050_e24650, assign27050_e24650_d_n3, assign27050_e24650_d_n4, assign27050_e24650_d_n5, assign27050_e24650_d_n6, assign27050_e24650_d_n7, assign27050_e24650_d_n8, assign27050_e24650_d_n9, assign27050_e24650_d_n10, assign27050_e24650_d_n11, assign27050_e24650_d_n12,) = {
    if (locals.var_guard1367 != 0.0) {
        let assign27050_e24648: f64 = (locals.var_t0__blk808 / locals.var_t1__blk809);
        (assign27050_e24648, (((locals.var_t0__blk808_dn3 * locals.var_t1__blk809) - (locals.var_t0__blk808 * locals.var_t1__blk809_dn3)) / (locals.var_t1__blk809 * locals.var_t1__blk809)), (((locals.var_t0__blk808_dn4 * locals.var_t1__blk809) - (locals.var_t0__blk808 * locals.var_t1__blk809_dn4)) / (locals.var_t1__blk809 * locals.var_t1__blk809)), (((locals.var_t0__blk808_dn5 * locals.var_t1__blk809) - (locals.var_t0__blk808 * locals.var_t1__blk809_dn5)) / (locals.var_t1__blk809 * locals.var_t1__blk809)), (((locals.var_t0__blk808_dn6 * locals.var_t1__blk809) - (locals.var_t0__blk808 * locals.var_t1__blk809_dn6)) / (locals.var_t1__blk809 * locals.var_t1__blk809)), (((locals.var_t0__blk808_dn7 * locals.var_t1__blk809) - (locals.var_t0__blk808 * locals.var_t1__blk809_dn7)) / (locals.var_t1__blk809 * locals.var_t1__blk809)), (((locals.var_t0__blk808_dn8 * locals.var_t1__blk809) - (locals.var_t0__blk808 * locals.var_t1__blk809_dn8)) / (locals.var_t1__blk809 * locals.var_t1__blk809)), (((locals.var_t0__blk808_dn9 * locals.var_t1__blk809) - (locals.var_t0__blk808 * locals.var_t1__blk809_dn9)) / (locals.var_t1__blk809 * locals.var_t1__blk809)), (((locals.var_t0__blk808_dn10 * locals.var_t1__blk809) - (locals.var_t0__blk808 * locals.var_t1__blk809_dn10)) / (locals.var_t1__blk809 * locals.var_t1__blk809)), (((locals.var_t0__blk808_dn11 * locals.var_t1__blk809) - (locals.var_t0__blk808 * locals.var_t1__blk809_dn11)) / (locals.var_t1__blk809 * locals.var_t1__blk809)), (((locals.var_t0__blk808_dn12 * locals.var_t1__blk809) - (locals.var_t0__blk808 * locals.var_t1__blk809_dn12)) / (locals.var_t1__blk809 * locals.var_t1__blk809)),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign27050_e24650;
        locals.var_t2__blk810_dn3 = assign27050_e24650_d_n3;
        locals.var_t2__blk810_dn4 = assign27050_e24650_d_n4;
        locals.var_t2__blk810_dn5 = assign27050_e24650_d_n5;
        locals.var_t2__blk810_dn6 = assign27050_e24650_d_n6;
        locals.var_t2__blk810_dn7 = assign27050_e24650_d_n7;
        locals.var_t2__blk810_dn8 = assign27050_e24650_d_n8;
        locals.var_t2__blk810_dn9 = assign27050_e24650_d_n9;
        locals.var_t2__blk810_dn10 = assign27050_e24650_d_n10;
        locals.var_t2__blk810_dn11 = assign27050_e24650_d_n11;
        locals.var_t2__blk810_dn12 = assign27050_e24650_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign27060_e24656, assign27060_e24656_d_n3, assign27060_e24656_d_n4, assign27060_e24656_d_n5, assign27060_e24656_d_n6, assign27060_e24656_d_n7, assign27060_e24656_d_n8, assign27060_e24656_d_n9, assign27060_e24656_d_n10, assign27060_e24656_d_n11, assign27060_e24656_d_n12,) = {
    if (locals.var_guard1367 != 0.0) {
        let assign27060_e24654: f64 = (locals.var_t0__blk808 * locals.var_t2__blk810);
        (assign27060_e24654, ((locals.var_t0__blk808_dn3 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn3)), ((locals.var_t0__blk808_dn4 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn4)), ((locals.var_t0__blk808_dn5 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn5)), ((locals.var_t0__blk808_dn6 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn6)), ((locals.var_t0__blk808_dn7 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn7)), ((locals.var_t0__blk808_dn8 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn8)), ((locals.var_t0__blk808_dn9 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn9)), ((locals.var_t0__blk808_dn10 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn10)), ((locals.var_t0__blk808_dn11 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn11)), ((locals.var_t0__blk808_dn12 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn12)),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign27060_e24656;
        locals.var_t3__blk811_dn3 = assign27060_e24656_d_n3;
        locals.var_t3__blk811_dn4 = assign27060_e24656_d_n4;
        locals.var_t3__blk811_dn5 = assign27060_e24656_d_n5;
        locals.var_t3__blk811_dn6 = assign27060_e24656_d_n6;
        locals.var_t3__blk811_dn7 = assign27060_e24656_d_n7;
        locals.var_t3__blk811_dn8 = assign27060_e24656_d_n8;
        locals.var_t3__blk811_dn9 = assign27060_e24656_d_n9;
        locals.var_t3__blk811_dn10 = assign27060_e24656_d_n10;
        locals.var_t3__blk811_dn11 = assign27060_e24656_d_n11;
        locals.var_t3__blk811_dn12 = assign27060_e24656_d_n12;
        locals.var_t3__blk811_rv = 0.0;

        let (assign27070_e24668, assign27070_e24668_d_n3, assign27070_e24668_d_n4, assign27070_e24668_d_n5, assign27070_e24668_d_n6, assign27070_e24668_d_n7, assign27070_e24668_d_n8, assign27070_e24668_d_n9, assign27070_e24668_d_n10, assign27070_e24668_d_n11, assign27070_e24668_d_n12,) = {
    if (locals.var_guard1367 != 0.0) {
        let assign27070_e24662: f64 = (0.5 * locals.var_t0__blk808);
        let assign27070_e24663: f64 = (locals.var_vgsteff__blk840 - assign27070_e24662);
        let assign27070_e24665: f64 = (assign27070_e24663 + locals.var_t3__blk811);
        let assign27070_e24666: f64 = (locals.var_coxwl * assign27070_e24665);
        (assign27070_e24666, ((locals.var_coxwl_dn3 * assign27070_e24665) + (locals.var_coxwl * ((locals.var_vgsteff__blk840_dn3 - (0.5 * locals.var_t0__blk808_dn3)) + locals.var_t3__blk811_dn3))), ((locals.var_coxwl_dn4 * assign27070_e24665) + (locals.var_coxwl * ((locals.var_vgsteff__blk840_dn4 - (0.5 * locals.var_t0__blk808_dn4)) + locals.var_t3__blk811_dn4))), ((locals.var_coxwl_dn5 * assign27070_e24665) + (locals.var_coxwl * ((locals.var_vgsteff__blk840_dn5 - (0.5 * locals.var_t0__blk808_dn5)) + locals.var_t3__blk811_dn5))), ((locals.var_coxwl_dn6 * assign27070_e24665) + (locals.var_coxwl * ((locals.var_vgsteff__blk840_dn6 - (0.5 * locals.var_t0__blk808_dn6)) + locals.var_t3__blk811_dn6))), ((locals.var_coxwl_dn7 * assign27070_e24665) + (locals.var_coxwl * ((locals.var_vgsteff__blk840_dn7 - (0.5 * locals.var_t0__blk808_dn7)) + locals.var_t3__blk811_dn7))), ((locals.var_coxwl_dn8 * assign27070_e24665) + (locals.var_coxwl * ((locals.var_vgsteff__blk840_dn8 - (0.5 * locals.var_t0__blk808_dn8)) + locals.var_t3__blk811_dn8))), ((locals.var_coxwl_dn9 * assign27070_e24665) + (locals.var_coxwl * ((locals.var_vgsteff__blk840_dn9 - (0.5 * locals.var_t0__blk808_dn9)) + locals.var_t3__blk811_dn9))), ((locals.var_coxwl_dn10 * assign27070_e24665) + (locals.var_coxwl * ((locals.var_vgsteff__blk840_dn10 - (0.5 * locals.var_t0__blk808_dn10)) + locals.var_t3__blk811_dn10))), ((locals.var_coxwl_dn11 * assign27070_e24665) + (locals.var_coxwl * ((locals.var_vgsteff__blk840_dn11 - (0.5 * locals.var_t0__blk808_dn11)) + locals.var_t3__blk811_dn11))), ((locals.var_coxwl_dn12 * assign27070_e24665) + (locals.var_coxwl * ((locals.var_vgsteff__blk840_dn12 - (0.5 * locals.var_t0__blk808_dn12)) + locals.var_t3__blk811_dn12))),)
    } else {
        (locals.var_qinv, locals.var_qinv_dn3, locals.var_qinv_dn4, locals.var_qinv_dn5, locals.var_qinv_dn6, locals.var_qinv_dn7, locals.var_qinv_dn8, locals.var_qinv_dn9, locals.var_qinv_dn10, locals.var_qinv_dn11, locals.var_qinv_dn12,)
    }
};
        locals.var_qinv = assign27070_e24668;
        locals.var_qinv_dn3 = assign27070_e24668_d_n3;
        locals.var_qinv_dn4 = assign27070_e24668_d_n4;
        locals.var_qinv_dn5 = assign27070_e24668_d_n5;
        locals.var_qinv_dn6 = assign27070_e24668_d_n6;
        locals.var_qinv_dn7 = assign27070_e24668_d_n7;
        locals.var_qinv_dn8 = assign27070_e24668_d_n8;
        locals.var_qinv_dn9 = assign27070_e24668_d_n9;
        locals.var_qinv_dn10 = assign27070_e24668_d_n10;
        locals.var_qinv_dn11 = assign27070_e24668_d_n11;
        locals.var_qinv_dn12 = assign27070_e24668_d_n12;
        locals.var_qinv_rv = 0.0;

        let assign27090_e24684: f64 = if (((locals.var_b4soisoimod != 2.0) && (locals.var_b4soibodymod != 0.0)) && (p.p27 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1379 = assign27090_e24684;
        locals.var_guard1379_rv = 0.0;

        let (assign27100_e24692, assign27100_e24692_d_n3, assign27100_e24692_d_n4, assign27100_e24692_d_n5, assign27100_e24692_d_n6, assign27100_e24692_d_n7, assign27100_e24692_d_n8, assign27100_e24692_d_n9, assign27100_e24692_d_n10, assign27100_e24692_d_n11, assign27100_e24692_d_n12,) = {
    if ((locals.var_guard1367 != 0.0) && (locals.var_guard1379 != 0.0)) {
        let assign27100_e24690: f64 = (locals.var_abulkcv * locals.var_vdseffcv2);
        (assign27100_e24690, ((locals.var_abulkcv_dn3 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn3)), ((locals.var_abulkcv_dn4 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn4)), ((locals.var_abulkcv_dn5 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn5)), ((locals.var_abulkcv_dn6 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn6)), ((locals.var_abulkcv_dn7 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn7)), ((locals.var_abulkcv_dn8 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn8)), ((locals.var_abulkcv_dn9 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn9)), ((locals.var_abulkcv_dn10 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn10)), ((locals.var_abulkcv_dn11 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn11)), ((locals.var_abulkcv_dn12 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn12)),)
    } else {
        (locals.var_t02, locals.var_t02_dn3, locals.var_t02_dn4, locals.var_t02_dn5, locals.var_t02_dn6, locals.var_t02_dn7, locals.var_t02_dn8, locals.var_t02_dn9, locals.var_t02_dn10, locals.var_t02_dn11, locals.var_t02_dn12,)
    }
};
        locals.var_t02 = assign27100_e24692;
        locals.var_t02_dn3 = assign27100_e24692_d_n3;
        locals.var_t02_dn4 = assign27100_e24692_d_n4;
        locals.var_t02_dn5 = assign27100_e24692_d_n5;
        locals.var_t02_dn6 = assign27100_e24692_d_n6;
        locals.var_t02_dn7 = assign27100_e24692_d_n7;
        locals.var_t02_dn8 = assign27100_e24692_d_n8;
        locals.var_t02_dn9 = assign27100_e24692_d_n9;
        locals.var_t02_dn10 = assign27100_e24692_d_n10;
        locals.var_t02_dn11 = assign27100_e24692_d_n11;
        locals.var_t02_dn12 = assign27100_e24692_d_n12;
        locals.var_t02_rv = 0.0;

        let (assign27110_e24706, assign27110_e24706_d_n3, assign27110_e24706_d_n4, assign27110_e24706_d_n5, assign27110_e24706_d_n6, assign27110_e24706_d_n7, assign27110_e24706_d_n8, assign27110_e24706_d_n9, assign27110_e24706_d_n10, assign27110_e24706_d_n11, assign27110_e24706_d_n12,) = {
    if ((locals.var_guard1367 != 0.0) && (locals.var_guard1379 != 0.0)) {
        let assign27110_e24700: f64 = (0.5 * locals.var_t02);
        let assign27110_e24701: f64 = (locals.var_vgsteff2 - assign27110_e24700);
        let assign27110_e24703: f64 = (assign27110_e24701 + 1e-20);
        let assign27110_e24704: f64 = (12.0 * assign27110_e24703);
        (assign27110_e24704, (12.0 * (locals.var_vgsteff2_dn3 - (0.5 * locals.var_t02_dn3))), (12.0 * (locals.var_vgsteff2_dn4 - (0.5 * locals.var_t02_dn4))), (12.0 * (locals.var_vgsteff2_dn5 - (0.5 * locals.var_t02_dn5))), (12.0 * (locals.var_vgsteff2_dn6 - (0.5 * locals.var_t02_dn6))), (12.0 * (locals.var_vgsteff2_dn7 - (0.5 * locals.var_t02_dn7))), (12.0 * (locals.var_vgsteff2_dn8 - (0.5 * locals.var_t02_dn8))), (12.0 * (locals.var_vgsteff2_dn9 - (0.5 * locals.var_t02_dn9))), (12.0 * (locals.var_vgsteff2_dn10 - (0.5 * locals.var_t02_dn10))), (12.0 * (locals.var_vgsteff2_dn11 - (0.5 * locals.var_t02_dn11))), (12.0 * (locals.var_vgsteff2_dn12 - (0.5 * locals.var_t02_dn12))),)
    } else {
        (locals.var_t12, locals.var_t12_dn3, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn12,)
    }
};
        locals.var_t12 = assign27110_e24706;
        locals.var_t12_dn3 = assign27110_e24706_d_n3;
        locals.var_t12_dn4 = assign27110_e24706_d_n4;
        locals.var_t12_dn5 = assign27110_e24706_d_n5;
        locals.var_t12_dn6 = assign27110_e24706_d_n6;
        locals.var_t12_dn7 = assign27110_e24706_d_n7;
        locals.var_t12_dn8 = assign27110_e24706_d_n8;
        locals.var_t12_dn9 = assign27110_e24706_d_n9;
        locals.var_t12_dn10 = assign27110_e24706_d_n10;
        locals.var_t12_dn11 = assign27110_e24706_d_n11;
        locals.var_t12_dn12 = assign27110_e24706_d_n12;
        locals.var_t12_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_82(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign27120_e24714, assign27120_e24714_d_n3, assign27120_e24714_d_n4, assign27120_e24714_d_n5, assign27120_e24714_d_n6, assign27120_e24714_d_n7, assign27120_e24714_d_n8, assign27120_e24714_d_n9, assign27120_e24714_d_n10, assign27120_e24714_d_n11, assign27120_e24714_d_n12,) = {
    if ((locals.var_guard1367 != 0.0) && (locals.var_guard1379 != 0.0)) {
        let assign27120_e24712: f64 = (locals.var_t02 / locals.var_t12);
        (assign27120_e24712, (((locals.var_t02_dn3 * locals.var_t12) - (locals.var_t02 * locals.var_t12_dn3)) / (locals.var_t12 * locals.var_t12)), (((locals.var_t02_dn4 * locals.var_t12) - (locals.var_t02 * locals.var_t12_dn4)) / (locals.var_t12 * locals.var_t12)), (((locals.var_t02_dn5 * locals.var_t12) - (locals.var_t02 * locals.var_t12_dn5)) / (locals.var_t12 * locals.var_t12)), (((locals.var_t02_dn6 * locals.var_t12) - (locals.var_t02 * locals.var_t12_dn6)) / (locals.var_t12 * locals.var_t12)), (((locals.var_t02_dn7 * locals.var_t12) - (locals.var_t02 * locals.var_t12_dn7)) / (locals.var_t12 * locals.var_t12)), (((locals.var_t02_dn8 * locals.var_t12) - (locals.var_t02 * locals.var_t12_dn8)) / (locals.var_t12 * locals.var_t12)), (((locals.var_t02_dn9 * locals.var_t12) - (locals.var_t02 * locals.var_t12_dn9)) / (locals.var_t12 * locals.var_t12)), (((locals.var_t02_dn10 * locals.var_t12) - (locals.var_t02 * locals.var_t12_dn10)) / (locals.var_t12 * locals.var_t12)), (((locals.var_t02_dn11 * locals.var_t12) - (locals.var_t02 * locals.var_t12_dn11)) / (locals.var_t12 * locals.var_t12)), (((locals.var_t02_dn12 * locals.var_t12) - (locals.var_t02 * locals.var_t12_dn12)) / (locals.var_t12 * locals.var_t12)),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign27120_e24714;
        locals.var_t2__blk810_dn3 = assign27120_e24714_d_n3;
        locals.var_t2__blk810_dn4 = assign27120_e24714_d_n4;
        locals.var_t2__blk810_dn5 = assign27120_e24714_d_n5;
        locals.var_t2__blk810_dn6 = assign27120_e24714_d_n6;
        locals.var_t2__blk810_dn7 = assign27120_e24714_d_n7;
        locals.var_t2__blk810_dn8 = assign27120_e24714_d_n8;
        locals.var_t2__blk810_dn9 = assign27120_e24714_d_n9;
        locals.var_t2__blk810_dn10 = assign27120_e24714_d_n10;
        locals.var_t2__blk810_dn11 = assign27120_e24714_d_n11;
        locals.var_t2__blk810_dn12 = assign27120_e24714_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign27130_e24722, assign27130_e24722_d_n3, assign27130_e24722_d_n4, assign27130_e24722_d_n5, assign27130_e24722_d_n6, assign27130_e24722_d_n7, assign27130_e24722_d_n8, assign27130_e24722_d_n9, assign27130_e24722_d_n10, assign27130_e24722_d_n11, assign27130_e24722_d_n12,) = {
    if ((locals.var_guard1367 != 0.0) && (locals.var_guard1379 != 0.0)) {
        let assign27130_e24720: f64 = (locals.var_t02 * locals.var_t2__blk810);
        (assign27130_e24720, ((locals.var_t02_dn3 * locals.var_t2__blk810) + (locals.var_t02 * locals.var_t2__blk810_dn3)), ((locals.var_t02_dn4 * locals.var_t2__blk810) + (locals.var_t02 * locals.var_t2__blk810_dn4)), ((locals.var_t02_dn5 * locals.var_t2__blk810) + (locals.var_t02 * locals.var_t2__blk810_dn5)), ((locals.var_t02_dn6 * locals.var_t2__blk810) + (locals.var_t02 * locals.var_t2__blk810_dn6)), ((locals.var_t02_dn7 * locals.var_t2__blk810) + (locals.var_t02 * locals.var_t2__blk810_dn7)), ((locals.var_t02_dn8 * locals.var_t2__blk810) + (locals.var_t02 * locals.var_t2__blk810_dn8)), ((locals.var_t02_dn9 * locals.var_t2__blk810) + (locals.var_t02 * locals.var_t2__blk810_dn9)), ((locals.var_t02_dn10 * locals.var_t2__blk810) + (locals.var_t02 * locals.var_t2__blk810_dn10)), ((locals.var_t02_dn11 * locals.var_t2__blk810) + (locals.var_t02 * locals.var_t2__blk810_dn11)), ((locals.var_t02_dn12 * locals.var_t2__blk810) + (locals.var_t02 * locals.var_t2__blk810_dn12)),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign27130_e24722;
        locals.var_t3__blk811_dn3 = assign27130_e24722_d_n3;
        locals.var_t3__blk811_dn4 = assign27130_e24722_d_n4;
        locals.var_t3__blk811_dn5 = assign27130_e24722_d_n5;
        locals.var_t3__blk811_dn6 = assign27130_e24722_d_n6;
        locals.var_t3__blk811_dn7 = assign27130_e24722_d_n7;
        locals.var_t3__blk811_dn8 = assign27130_e24722_d_n8;
        locals.var_t3__blk811_dn9 = assign27130_e24722_d_n9;
        locals.var_t3__blk811_dn10 = assign27130_e24722_d_n10;
        locals.var_t3__blk811_dn11 = assign27130_e24722_d_n11;
        locals.var_t3__blk811_dn12 = assign27130_e24722_d_n12;
        locals.var_t3__blk811_rv = 0.0;

        let (assign27140_e24738, assign27140_e24738_d_n3, assign27140_e24738_d_n4, assign27140_e24738_d_n5, assign27140_e24738_d_n6, assign27140_e24738_d_n7, assign27140_e24738_d_n8, assign27140_e24738_d_n9, assign27140_e24738_d_n10, assign27140_e24738_d_n11, assign27140_e24738_d_n12,) = {
    if ((locals.var_guard1367 != 0.0) && (locals.var_guard1379 != 0.0)) {
        let assign27140_e24731: f64 = (0.5 * locals.var_t02);
        let assign27140_e24732: f64 = (locals.var_vgsteff2 - assign27140_e24731);
        let assign27140_e24734: f64 = (assign27140_e24732 + locals.var_t3__blk811);
        let assign27140_e24735: f64 = (locals.var_coxwl2 * assign27140_e24734);
        let assign27140_e24736: f64 = (locals.var_qinv + assign27140_e24735);
        (assign27140_e24736, (locals.var_qinv_dn3 + ((locals.var_coxwl2_dn3 * assign27140_e24734) + (locals.var_coxwl2 * ((locals.var_vgsteff2_dn3 - (0.5 * locals.var_t02_dn3)) + locals.var_t3__blk811_dn3)))), (locals.var_qinv_dn4 + ((locals.var_coxwl2_dn4 * assign27140_e24734) + (locals.var_coxwl2 * ((locals.var_vgsteff2_dn4 - (0.5 * locals.var_t02_dn4)) + locals.var_t3__blk811_dn4)))), (locals.var_qinv_dn5 + ((locals.var_coxwl2_dn5 * assign27140_e24734) + (locals.var_coxwl2 * ((locals.var_vgsteff2_dn5 - (0.5 * locals.var_t02_dn5)) + locals.var_t3__blk811_dn5)))), (locals.var_qinv_dn6 + ((locals.var_coxwl2_dn6 * assign27140_e24734) + (locals.var_coxwl2 * ((locals.var_vgsteff2_dn6 - (0.5 * locals.var_t02_dn6)) + locals.var_t3__blk811_dn6)))), (locals.var_qinv_dn7 + ((locals.var_coxwl2_dn7 * assign27140_e24734) + (locals.var_coxwl2 * ((locals.var_vgsteff2_dn7 - (0.5 * locals.var_t02_dn7)) + locals.var_t3__blk811_dn7)))), (locals.var_qinv_dn8 + ((locals.var_coxwl2_dn8 * assign27140_e24734) + (locals.var_coxwl2 * ((locals.var_vgsteff2_dn8 - (0.5 * locals.var_t02_dn8)) + locals.var_t3__blk811_dn8)))), (locals.var_qinv_dn9 + ((locals.var_coxwl2_dn9 * assign27140_e24734) + (locals.var_coxwl2 * ((locals.var_vgsteff2_dn9 - (0.5 * locals.var_t02_dn9)) + locals.var_t3__blk811_dn9)))), (locals.var_qinv_dn10 + ((locals.var_coxwl2_dn10 * assign27140_e24734) + (locals.var_coxwl2 * ((locals.var_vgsteff2_dn10 - (0.5 * locals.var_t02_dn10)) + locals.var_t3__blk811_dn10)))), (locals.var_qinv_dn11 + ((locals.var_coxwl2_dn11 * assign27140_e24734) + (locals.var_coxwl2 * ((locals.var_vgsteff2_dn11 - (0.5 * locals.var_t02_dn11)) + locals.var_t3__blk811_dn11)))), (locals.var_qinv_dn12 + ((locals.var_coxwl2_dn12 * assign27140_e24734) + (locals.var_coxwl2 * ((locals.var_vgsteff2_dn12 - (0.5 * locals.var_t02_dn12)) + locals.var_t3__blk811_dn12)))),)
    } else {
        (locals.var_qinv, locals.var_qinv_dn3, locals.var_qinv_dn4, locals.var_qinv_dn5, locals.var_qinv_dn6, locals.var_qinv_dn7, locals.var_qinv_dn8, locals.var_qinv_dn9, locals.var_qinv_dn10, locals.var_qinv_dn11, locals.var_qinv_dn12,)
    }
};
        locals.var_qinv = assign27140_e24738;
        locals.var_qinv_dn3 = assign27140_e24738_d_n3;
        locals.var_qinv_dn4 = assign27140_e24738_d_n4;
        locals.var_qinv_dn5 = assign27140_e24738_d_n5;
        locals.var_qinv_dn6 = assign27140_e24738_d_n6;
        locals.var_qinv_dn7 = assign27140_e24738_d_n7;
        locals.var_qinv_dn8 = assign27140_e24738_d_n8;
        locals.var_qinv_dn9 = assign27140_e24738_d_n9;
        locals.var_qinv_dn10 = assign27140_e24738_d_n10;
        locals.var_qinv_dn11 = assign27140_e24738_d_n11;
        locals.var_qinv_dn12 = assign27140_e24738_d_n12;
        locals.var_qinv_rv = 0.0;

        let assign27160_e24748: f64 = if p.p129 > 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1380 = assign27160_e24748;
        locals.var_guard1380_rv = 0.0;

        let (assign27170_e24756, assign27170_e24756_d_n3, assign27170_e24756_d_n4, assign27170_e24756_d_n5, assign27170_e24756_d_n6, assign27170_e24756_d_n7, assign27170_e24756_d_n8, assign27170_e24756_d_n9, assign27170_e24756_d_n10, assign27170_e24756_d_n11, assign27170_e24756_d_n12,) = {
    if ((locals.var_guard1367 != 0.0) && (locals.var_guard1380 != 0.0)) {
        let assign27170_e24754: f64 = (locals.var_t1__blk809 + locals.var_t1__blk809);
        (assign27170_e24754, (locals.var_t1__blk809_dn3 + locals.var_t1__blk809_dn3), (locals.var_t1__blk809_dn4 + locals.var_t1__blk809_dn4), (locals.var_t1__blk809_dn5 + locals.var_t1__blk809_dn5), (locals.var_t1__blk809_dn6 + locals.var_t1__blk809_dn6), (locals.var_t1__blk809_dn7 + locals.var_t1__blk809_dn7), (locals.var_t1__blk809_dn8 + locals.var_t1__blk809_dn8), (locals.var_t1__blk809_dn9 + locals.var_t1__blk809_dn9), (locals.var_t1__blk809_dn10 + locals.var_t1__blk809_dn10), (locals.var_t1__blk809_dn11 + locals.var_t1__blk809_dn11), (locals.var_t1__blk809_dn12 + locals.var_t1__blk809_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign27170_e24756;
        locals.var_t1__blk809_dn3 = assign27170_e24756_d_n3;
        locals.var_t1__blk809_dn4 = assign27170_e24756_d_n4;
        locals.var_t1__blk809_dn5 = assign27170_e24756_d_n5;
        locals.var_t1__blk809_dn6 = assign27170_e24756_d_n6;
        locals.var_t1__blk809_dn7 = assign27170_e24756_d_n7;
        locals.var_t1__blk809_dn8 = assign27170_e24756_d_n8;
        locals.var_t1__blk809_dn9 = assign27170_e24756_d_n9;
        locals.var_t1__blk809_dn10 = assign27170_e24756_d_n10;
        locals.var_t1__blk809_dn11 = assign27170_e24756_d_n11;
        locals.var_t1__blk809_dn12 = assign27170_e24756_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign27180_e24777, assign27180_e24777_d_n3, assign27180_e24777_d_n4, assign27180_e24777_d_n5, assign27180_e24777_d_n6, assign27180_e24777_d_n7, assign27180_e24777_d_n8, assign27180_e24777_d_n9, assign27180_e24777_d_n10, assign27180_e24777_d_n11, assign27180_e24777_d_n12,) = {
    if ((locals.var_guard1367 != 0.0) && (locals.var_guard1380 != 0.0)) {
        let assign27180_e24761: f64 = (-locals.var_coxwl);
        let assign27180_e24764: f64 = (0.5 * locals.var_vgsteff__blk840);
        let assign27180_e24767: f64 = (0.25 * locals.var_t0__blk808);
        let assign27180_e24768: f64 = (assign27180_e24764 + assign27180_e24767);
        let assign27180_e24771: f64 = (locals.var_t0__blk808 * locals.var_t0__blk808);
        let assign27180_e24773: f64 = (assign27180_e24771 / locals.var_t1__blk809);
        let assign27180_e24774: f64 = (assign27180_e24768 - assign27180_e24773);
        let assign27180_e24775: f64 = (assign27180_e24761 * assign27180_e24774);
        (assign27180_e24775, (((-locals.var_coxwl_dn3) * assign27180_e24774) + (assign27180_e24761 * (((0.5 * locals.var_vgsteff__blk840_dn3) + (0.25 * locals.var_t0__blk808_dn3)) - (((((locals.var_t0__blk808_dn3 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn3)) * locals.var_t1__blk809) - (assign27180_e24771 * locals.var_t1__blk809_dn3)) / (locals.var_t1__blk809 * locals.var_t1__blk809))))), (((-locals.var_coxwl_dn4) * assign27180_e24774) + (assign27180_e24761 * (((0.5 * locals.var_vgsteff__blk840_dn4) + (0.25 * locals.var_t0__blk808_dn4)) - (((((locals.var_t0__blk808_dn4 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn4)) * locals.var_t1__blk809) - (assign27180_e24771 * locals.var_t1__blk809_dn4)) / (locals.var_t1__blk809 * locals.var_t1__blk809))))), (((-locals.var_coxwl_dn5) * assign27180_e24774) + (assign27180_e24761 * (((0.5 * locals.var_vgsteff__blk840_dn5) + (0.25 * locals.var_t0__blk808_dn5)) - (((((locals.var_t0__blk808_dn5 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn5)) * locals.var_t1__blk809) - (assign27180_e24771 * locals.var_t1__blk809_dn5)) / (locals.var_t1__blk809 * locals.var_t1__blk809))))), (((-locals.var_coxwl_dn6) * assign27180_e24774) + (assign27180_e24761 * (((0.5 * locals.var_vgsteff__blk840_dn6) + (0.25 * locals.var_t0__blk808_dn6)) - (((((locals.var_t0__blk808_dn6 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn6)) * locals.var_t1__blk809) - (assign27180_e24771 * locals.var_t1__blk809_dn6)) / (locals.var_t1__blk809 * locals.var_t1__blk809))))), (((-locals.var_coxwl_dn7) * assign27180_e24774) + (assign27180_e24761 * (((0.5 * locals.var_vgsteff__blk840_dn7) + (0.25 * locals.var_t0__blk808_dn7)) - (((((locals.var_t0__blk808_dn7 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn7)) * locals.var_t1__blk809) - (assign27180_e24771 * locals.var_t1__blk809_dn7)) / (locals.var_t1__blk809 * locals.var_t1__blk809))))), (((-locals.var_coxwl_dn8) * assign27180_e24774) + (assign27180_e24761 * (((0.5 * locals.var_vgsteff__blk840_dn8) + (0.25 * locals.var_t0__blk808_dn8)) - (((((locals.var_t0__blk808_dn8 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn8)) * locals.var_t1__blk809) - (assign27180_e24771 * locals.var_t1__blk809_dn8)) / (locals.var_t1__blk809 * locals.var_t1__blk809))))), (((-locals.var_coxwl_dn9) * assign27180_e24774) + (assign27180_e24761 * (((0.5 * locals.var_vgsteff__blk840_dn9) + (0.25 * locals.var_t0__blk808_dn9)) - (((((locals.var_t0__blk808_dn9 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn9)) * locals.var_t1__blk809) - (assign27180_e24771 * locals.var_t1__blk809_dn9)) / (locals.var_t1__blk809 * locals.var_t1__blk809))))), (((-locals.var_coxwl_dn10) * assign27180_e24774) + (assign27180_e24761 * (((0.5 * locals.var_vgsteff__blk840_dn10) + (0.25 * locals.var_t0__blk808_dn10)) - (((((locals.var_t0__blk808_dn10 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn10)) * locals.var_t1__blk809) - (assign27180_e24771 * locals.var_t1__blk809_dn10)) / (locals.var_t1__blk809 * locals.var_t1__blk809))))), (((-locals.var_coxwl_dn11) * assign27180_e24774) + (assign27180_e24761 * (((0.5 * locals.var_vgsteff__blk840_dn11) + (0.25 * locals.var_t0__blk808_dn11)) - (((((locals.var_t0__blk808_dn11 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn11)) * locals.var_t1__blk809) - (assign27180_e24771 * locals.var_t1__blk809_dn11)) / (locals.var_t1__blk809 * locals.var_t1__blk809))))), (((-locals.var_coxwl_dn12) * assign27180_e24774) + (assign27180_e24761 * (((0.5 * locals.var_vgsteff__blk840_dn12) + (0.25 * locals.var_t0__blk808_dn12)) - (((((locals.var_t0__blk808_dn12 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn12)) * locals.var_t1__blk809) - (assign27180_e24771 * locals.var_t1__blk809_dn12)) / (locals.var_t1__blk809 * locals.var_t1__blk809))))),)
    } else {
        (locals.var_qsrc, locals.var_qsrc_dn3, locals.var_qsrc_dn4, locals.var_qsrc_dn5, locals.var_qsrc_dn6, locals.var_qsrc_dn7, locals.var_qsrc_dn8, locals.var_qsrc_dn9, locals.var_qsrc_dn10, locals.var_qsrc_dn11, locals.var_qsrc_dn12,)
    }
};
        locals.var_qsrc = assign27180_e24777;
        locals.var_qsrc_dn3 = assign27180_e24777_d_n3;
        locals.var_qsrc_dn4 = assign27180_e24777_d_n4;
        locals.var_qsrc_dn5 = assign27180_e24777_d_n5;
        locals.var_qsrc_dn6 = assign27180_e24777_d_n6;
        locals.var_qsrc_dn7 = assign27180_e24777_d_n7;
        locals.var_qsrc_dn8 = assign27180_e24777_d_n8;
        locals.var_qsrc_dn9 = assign27180_e24777_d_n9;
        locals.var_qsrc_dn10 = assign27180_e24777_d_n10;
        locals.var_qsrc_dn11 = assign27180_e24777_d_n11;
        locals.var_qsrc_dn12 = assign27180_e24777_d_n12;
        locals.var_qsrc_rv = 0.0;

        let assign27190_e24788: f64 = if (((locals.var_b4soisoimod != 2.0) && (locals.var_b4soibodymod != 0.0)) && (p.p27 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1381 = assign27190_e24788;
        locals.var_guard1381_rv = 0.0;

        let (assign27200_e24798, assign27200_e24798_d_n3, assign27200_e24798_d_n4, assign27200_e24798_d_n5, assign27200_e24798_d_n6, assign27200_e24798_d_n7, assign27200_e24798_d_n8, assign27200_e24798_d_n9, assign27200_e24798_d_n10, assign27200_e24798_d_n11, assign27200_e24798_d_n12,) = {
    if (((locals.var_guard1367 != 0.0) && (locals.var_guard1380 != 0.0)) && (locals.var_guard1381 != 0.0)) {
        let assign27200_e24796: f64 = (locals.var_t12 + locals.var_t12);
        (assign27200_e24796, (locals.var_t12_dn3 + locals.var_t12_dn3), (locals.var_t12_dn4 + locals.var_t12_dn4), (locals.var_t12_dn5 + locals.var_t12_dn5), (locals.var_t12_dn6 + locals.var_t12_dn6), (locals.var_t12_dn7 + locals.var_t12_dn7), (locals.var_t12_dn8 + locals.var_t12_dn8), (locals.var_t12_dn9 + locals.var_t12_dn9), (locals.var_t12_dn10 + locals.var_t12_dn10), (locals.var_t12_dn11 + locals.var_t12_dn11), (locals.var_t12_dn12 + locals.var_t12_dn12),)
    } else {
        (locals.var_t12, locals.var_t12_dn3, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn12,)
    }
};
        locals.var_t12 = assign27200_e24798;
        locals.var_t12_dn3 = assign27200_e24798_d_n3;
        locals.var_t12_dn4 = assign27200_e24798_d_n4;
        locals.var_t12_dn5 = assign27200_e24798_d_n5;
        locals.var_t12_dn6 = assign27200_e24798_d_n6;
        locals.var_t12_dn7 = assign27200_e24798_d_n7;
        locals.var_t12_dn8 = assign27200_e24798_d_n8;
        locals.var_t12_dn9 = assign27200_e24798_d_n9;
        locals.var_t12_dn10 = assign27200_e24798_d_n10;
        locals.var_t12_dn11 = assign27200_e24798_d_n11;
        locals.var_t12_dn12 = assign27200_e24798_d_n12;
        locals.var_t12_rv = 0.0;

        let (assign27210_e24822, assign27210_e24822_d_n3, assign27210_e24822_d_n4, assign27210_e24822_d_n5, assign27210_e24822_d_n6, assign27210_e24822_d_n7, assign27210_e24822_d_n8, assign27210_e24822_d_n9, assign27210_e24822_d_n10, assign27210_e24822_d_n11, assign27210_e24822_d_n12,) = {
    if (((locals.var_guard1367 != 0.0) && (locals.var_guard1380 != 0.0)) && (locals.var_guard1381 != 0.0)) {
        let assign27210_e24808: f64 = (0.5 * locals.var_vgsteff2);
        let assign27210_e24811: f64 = (0.25 * locals.var_t02);
        let assign27210_e24812: f64 = (assign27210_e24808 + assign27210_e24811);
        let assign27210_e24815: f64 = (locals.var_t02 * locals.var_t02);
        let assign27210_e24817: f64 = (assign27210_e24815 / locals.var_t12);
        let assign27210_e24818: f64 = (assign27210_e24812 - assign27210_e24817);
        let assign27210_e24819: f64 = (locals.var_coxwl2 * assign27210_e24818);
        let assign27210_e24820: f64 = (locals.var_qsrc - assign27210_e24819);
        (assign27210_e24820, (locals.var_qsrc_dn3 - ((locals.var_coxwl2_dn3 * assign27210_e24818) + (locals.var_coxwl2 * (((0.5 * locals.var_vgsteff2_dn3) + (0.25 * locals.var_t02_dn3)) - (((((locals.var_t02_dn3 * locals.var_t02) + (locals.var_t02 * locals.var_t02_dn3)) * locals.var_t12) - (assign27210_e24815 * locals.var_t12_dn3)) / (locals.var_t12 * locals.var_t12)))))), (locals.var_qsrc_dn4 - ((locals.var_coxwl2_dn4 * assign27210_e24818) + (locals.var_coxwl2 * (((0.5 * locals.var_vgsteff2_dn4) + (0.25 * locals.var_t02_dn4)) - (((((locals.var_t02_dn4 * locals.var_t02) + (locals.var_t02 * locals.var_t02_dn4)) * locals.var_t12) - (assign27210_e24815 * locals.var_t12_dn4)) / (locals.var_t12 * locals.var_t12)))))), (locals.var_qsrc_dn5 - ((locals.var_coxwl2_dn5 * assign27210_e24818) + (locals.var_coxwl2 * (((0.5 * locals.var_vgsteff2_dn5) + (0.25 * locals.var_t02_dn5)) - (((((locals.var_t02_dn5 * locals.var_t02) + (locals.var_t02 * locals.var_t02_dn5)) * locals.var_t12) - (assign27210_e24815 * locals.var_t12_dn5)) / (locals.var_t12 * locals.var_t12)))))), (locals.var_qsrc_dn6 - ((locals.var_coxwl2_dn6 * assign27210_e24818) + (locals.var_coxwl2 * (((0.5 * locals.var_vgsteff2_dn6) + (0.25 * locals.var_t02_dn6)) - (((((locals.var_t02_dn6 * locals.var_t02) + (locals.var_t02 * locals.var_t02_dn6)) * locals.var_t12) - (assign27210_e24815 * locals.var_t12_dn6)) / (locals.var_t12 * locals.var_t12)))))), (locals.var_qsrc_dn7 - ((locals.var_coxwl2_dn7 * assign27210_e24818) + (locals.var_coxwl2 * (((0.5 * locals.var_vgsteff2_dn7) + (0.25 * locals.var_t02_dn7)) - (((((locals.var_t02_dn7 * locals.var_t02) + (locals.var_t02 * locals.var_t02_dn7)) * locals.var_t12) - (assign27210_e24815 * locals.var_t12_dn7)) / (locals.var_t12 * locals.var_t12)))))), (locals.var_qsrc_dn8 - ((locals.var_coxwl2_dn8 * assign27210_e24818) + (locals.var_coxwl2 * (((0.5 * locals.var_vgsteff2_dn8) + (0.25 * locals.var_t02_dn8)) - (((((locals.var_t02_dn8 * locals.var_t02) + (locals.var_t02 * locals.var_t02_dn8)) * locals.var_t12) - (assign27210_e24815 * locals.var_t12_dn8)) / (locals.var_t12 * locals.var_t12)))))), (locals.var_qsrc_dn9 - ((locals.var_coxwl2_dn9 * assign27210_e24818) + (locals.var_coxwl2 * (((0.5 * locals.var_vgsteff2_dn9) + (0.25 * locals.var_t02_dn9)) - (((((locals.var_t02_dn9 * locals.var_t02) + (locals.var_t02 * locals.var_t02_dn9)) * locals.var_t12) - (assign27210_e24815 * locals.var_t12_dn9)) / (locals.var_t12 * locals.var_t12)))))), (locals.var_qsrc_dn10 - ((locals.var_coxwl2_dn10 * assign27210_e24818) + (locals.var_coxwl2 * (((0.5 * locals.var_vgsteff2_dn10) + (0.25 * locals.var_t02_dn10)) - (((((locals.var_t02_dn10 * locals.var_t02) + (locals.var_t02 * locals.var_t02_dn10)) * locals.var_t12) - (assign27210_e24815 * locals.var_t12_dn10)) / (locals.var_t12 * locals.var_t12)))))), (locals.var_qsrc_dn11 - ((locals.var_coxwl2_dn11 * assign27210_e24818) + (locals.var_coxwl2 * (((0.5 * locals.var_vgsteff2_dn11) + (0.25 * locals.var_t02_dn11)) - (((((locals.var_t02_dn11 * locals.var_t02) + (locals.var_t02 * locals.var_t02_dn11)) * locals.var_t12) - (assign27210_e24815 * locals.var_t12_dn11)) / (locals.var_t12 * locals.var_t12)))))), (locals.var_qsrc_dn12 - ((locals.var_coxwl2_dn12 * assign27210_e24818) + (locals.var_coxwl2 * (((0.5 * locals.var_vgsteff2_dn12) + (0.25 * locals.var_t02_dn12)) - (((((locals.var_t02_dn12 * locals.var_t02) + (locals.var_t02 * locals.var_t02_dn12)) * locals.var_t12) - (assign27210_e24815 * locals.var_t12_dn12)) / (locals.var_t12 * locals.var_t12)))))),)
    } else {
        (locals.var_qsrc, locals.var_qsrc_dn3, locals.var_qsrc_dn4, locals.var_qsrc_dn5, locals.var_qsrc_dn6, locals.var_qsrc_dn7, locals.var_qsrc_dn8, locals.var_qsrc_dn9, locals.var_qsrc_dn10, locals.var_qsrc_dn11, locals.var_qsrc_dn12,)
    }
};
        locals.var_qsrc = assign27210_e24822;
        locals.var_qsrc_dn3 = assign27210_e24822_d_n3;
        locals.var_qsrc_dn4 = assign27210_e24822_d_n4;
        locals.var_qsrc_dn5 = assign27210_e24822_d_n5;
        locals.var_qsrc_dn6 = assign27210_e24822_d_n6;
        locals.var_qsrc_dn7 = assign27210_e24822_d_n7;
        locals.var_qsrc_dn8 = assign27210_e24822_d_n8;
        locals.var_qsrc_dn9 = assign27210_e24822_d_n9;
        locals.var_qsrc_dn10 = assign27210_e24822_d_n10;
        locals.var_qsrc_dn11 = assign27210_e24822_d_n11;
        locals.var_qsrc_dn12 = assign27210_e24822_d_n12;
        locals.var_qsrc_rv = 0.0;

        let assign27220_e24825: f64 = if p.p129 < 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1382 = assign27220_e24825;
        locals.var_guard1382_rv = 0.0;

        let (assign27230_e24836, assign27230_e24836_d_n3, assign27230_e24836_d_n4, assign27230_e24836_d_n5, assign27230_e24836_d_n6, assign27230_e24836_d_n7, assign27230_e24836_d_n8, assign27230_e24836_d_n9, assign27230_e24836_d_n10, assign27230_e24836_d_n11, assign27230_e24836_d_n12,) = {
    if (((locals.var_guard1367 != 0.0) && (locals.var_guard1380 == 0.0)) && (locals.var_guard1382 != 0.0)) {
        let assign27230_e24834: f64 = (locals.var_t1__blk809 / 12.0);
        (assign27230_e24834, (locals.var_t1__blk809_dn3 / 12.0), (locals.var_t1__blk809_dn4 / 12.0), (locals.var_t1__blk809_dn5 / 12.0), (locals.var_t1__blk809_dn6 / 12.0), (locals.var_t1__blk809_dn7 / 12.0), (locals.var_t1__blk809_dn8 / 12.0), (locals.var_t1__blk809_dn9 / 12.0), (locals.var_t1__blk809_dn10 / 12.0), (locals.var_t1__blk809_dn11 / 12.0), (locals.var_t1__blk809_dn12 / 12.0),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign27230_e24836;
        locals.var_t1__blk809_dn3 = assign27230_e24836_d_n3;
        locals.var_t1__blk809_dn4 = assign27230_e24836_d_n4;
        locals.var_t1__blk809_dn5 = assign27230_e24836_d_n5;
        locals.var_t1__blk809_dn6 = assign27230_e24836_d_n6;
        locals.var_t1__blk809_dn7 = assign27230_e24836_d_n7;
        locals.var_t1__blk809_dn8 = assign27230_e24836_d_n8;
        locals.var_t1__blk809_dn9 = assign27230_e24836_d_n9;
        locals.var_t1__blk809_dn10 = assign27230_e24836_d_n10;
        locals.var_t1__blk809_dn11 = assign27230_e24836_d_n11;
        locals.var_t1__blk809_dn12 = assign27230_e24836_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign27240_e24851, assign27240_e24851_d_n3, assign27240_e24851_d_n4, assign27240_e24851_d_n5, assign27240_e24851_d_n6, assign27240_e24851_d_n7, assign27240_e24851_d_n8, assign27240_e24851_d_n9, assign27240_e24851_d_n10, assign27240_e24851_d_n11, assign27240_e24851_d_n12,) = {
    if (((locals.var_guard1367 != 0.0) && (locals.var_guard1380 == 0.0)) && (locals.var_guard1382 != 0.0)) {
        let assign27240_e24845: f64 = (0.5 * locals.var_coxwl);
        let assign27240_e24848: f64 = (locals.var_t1__blk809 * locals.var_t1__blk809);
        let assign27240_e24849: f64 = (assign27240_e24845 / assign27240_e24848);
        (assign27240_e24849, ((((0.5 * locals.var_coxwl_dn3) * assign27240_e24848) - (assign27240_e24845 * ((locals.var_t1__blk809_dn3 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn3)))) / (assign27240_e24848 * assign27240_e24848)), ((((0.5 * locals.var_coxwl_dn4) * assign27240_e24848) - (assign27240_e24845 * ((locals.var_t1__blk809_dn4 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn4)))) / (assign27240_e24848 * assign27240_e24848)), ((((0.5 * locals.var_coxwl_dn5) * assign27240_e24848) - (assign27240_e24845 * ((locals.var_t1__blk809_dn5 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn5)))) / (assign27240_e24848 * assign27240_e24848)), ((((0.5 * locals.var_coxwl_dn6) * assign27240_e24848) - (assign27240_e24845 * ((locals.var_t1__blk809_dn6 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn6)))) / (assign27240_e24848 * assign27240_e24848)), ((((0.5 * locals.var_coxwl_dn7) * assign27240_e24848) - (assign27240_e24845 * ((locals.var_t1__blk809_dn7 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn7)))) / (assign27240_e24848 * assign27240_e24848)), ((((0.5 * locals.var_coxwl_dn8) * assign27240_e24848) - (assign27240_e24845 * ((locals.var_t1__blk809_dn8 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn8)))) / (assign27240_e24848 * assign27240_e24848)), ((((0.5 * locals.var_coxwl_dn9) * assign27240_e24848) - (assign27240_e24845 * ((locals.var_t1__blk809_dn9 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn9)))) / (assign27240_e24848 * assign27240_e24848)), ((((0.5 * locals.var_coxwl_dn10) * assign27240_e24848) - (assign27240_e24845 * ((locals.var_t1__blk809_dn10 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn10)))) / (assign27240_e24848 * assign27240_e24848)), ((((0.5 * locals.var_coxwl_dn11) * assign27240_e24848) - (assign27240_e24845 * ((locals.var_t1__blk809_dn11 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn11)))) / (assign27240_e24848 * assign27240_e24848)), ((((0.5 * locals.var_coxwl_dn12) * assign27240_e24848) - (assign27240_e24845 * ((locals.var_t1__blk809_dn12 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn12)))) / (assign27240_e24848 * assign27240_e24848)),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign27240_e24851;
        locals.var_t2__blk810_dn3 = assign27240_e24851_d_n3;
        locals.var_t2__blk810_dn4 = assign27240_e24851_d_n4;
        locals.var_t2__blk810_dn5 = assign27240_e24851_d_n5;
        locals.var_t2__blk810_dn6 = assign27240_e24851_d_n6;
        locals.var_t2__blk810_dn7 = assign27240_e24851_d_n7;
        locals.var_t2__blk810_dn8 = assign27240_e24851_d_n8;
        locals.var_t2__blk810_dn9 = assign27240_e24851_d_n9;
        locals.var_t2__blk810_dn10 = assign27240_e24851_d_n10;
        locals.var_t2__blk810_dn11 = assign27240_e24851_d_n11;
        locals.var_t2__blk810_dn12 = assign27240_e24851_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign27250_e24888, assign27250_e24888_d_n3, assign27250_e24888_d_n4, assign27250_e24888_d_n5, assign27250_e24888_d_n6, assign27250_e24888_d_n7, assign27250_e24888_d_n8, assign27250_e24888_d_n9, assign27250_e24888_d_n10, assign27250_e24888_d_n11, assign27250_e24888_d_n12,) = {
    if (((locals.var_guard1367 != 0.0) && (locals.var_guard1380 == 0.0)) && (locals.var_guard1382 != 0.0)) {
        let assign27250_e24861: f64 = (2.0 * locals.var_t0__blk808);
        let assign27250_e24863: f64 = (assign27250_e24861 * locals.var_t0__blk808);
        let assign27250_e24865: f64 = (assign27250_e24863 / 3.0);
        let assign27250_e24870: f64 = (4.0 * locals.var_t0__blk808);
        let assign27250_e24872: f64 = (assign27250_e24870 / 3.0);
        let assign27250_e24873: f64 = (locals.var_vgsteff__blk840 - assign27250_e24872);
        let assign27250_e24874: f64 = (locals.var_vgsteff__blk840 * assign27250_e24873);
        let assign27250_e24875: f64 = (assign27250_e24865 + assign27250_e24874);
        let assign27250_e24876: f64 = (locals.var_vgsteff__blk840 * assign27250_e24875);
        let assign27250_e24879: f64 = (2.0 * locals.var_t0__blk808);
        let assign27250_e24881: f64 = (assign27250_e24879 * locals.var_t0__blk808);
        let assign27250_e24883: f64 = (assign27250_e24881 * locals.var_t0__blk808);
        let assign27250_e24885: f64 = (assign27250_e24883 / 15.0);
        let assign27250_e24886: f64 = (assign27250_e24876 - assign27250_e24885);
        (assign27250_e24886, (((locals.var_vgsteff__blk840_dn3 * assign27250_e24875) + (locals.var_vgsteff__blk840 * (((((2.0 * locals.var_t0__blk808_dn3) * locals.var_t0__blk808) + (assign27250_e24861 * locals.var_t0__blk808_dn3)) / 3.0) + ((locals.var_vgsteff__blk840_dn3 * assign27250_e24873) + (locals.var_vgsteff__blk840 * (locals.var_vgsteff__blk840_dn3 - ((4.0 * locals.var_t0__blk808_dn3) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk808_dn3) * locals.var_t0__blk808) + (assign27250_e24879 * locals.var_t0__blk808_dn3)) * locals.var_t0__blk808) + (assign27250_e24881 * locals.var_t0__blk808_dn3)) / 15.0)), (((locals.var_vgsteff__blk840_dn4 * assign27250_e24875) + (locals.var_vgsteff__blk840 * (((((2.0 * locals.var_t0__blk808_dn4) * locals.var_t0__blk808) + (assign27250_e24861 * locals.var_t0__blk808_dn4)) / 3.0) + ((locals.var_vgsteff__blk840_dn4 * assign27250_e24873) + (locals.var_vgsteff__blk840 * (locals.var_vgsteff__blk840_dn4 - ((4.0 * locals.var_t0__blk808_dn4) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk808_dn4) * locals.var_t0__blk808) + (assign27250_e24879 * locals.var_t0__blk808_dn4)) * locals.var_t0__blk808) + (assign27250_e24881 * locals.var_t0__blk808_dn4)) / 15.0)), (((locals.var_vgsteff__blk840_dn5 * assign27250_e24875) + (locals.var_vgsteff__blk840 * (((((2.0 * locals.var_t0__blk808_dn5) * locals.var_t0__blk808) + (assign27250_e24861 * locals.var_t0__blk808_dn5)) / 3.0) + ((locals.var_vgsteff__blk840_dn5 * assign27250_e24873) + (locals.var_vgsteff__blk840 * (locals.var_vgsteff__blk840_dn5 - ((4.0 * locals.var_t0__blk808_dn5) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk808_dn5) * locals.var_t0__blk808) + (assign27250_e24879 * locals.var_t0__blk808_dn5)) * locals.var_t0__blk808) + (assign27250_e24881 * locals.var_t0__blk808_dn5)) / 15.0)), (((locals.var_vgsteff__blk840_dn6 * assign27250_e24875) + (locals.var_vgsteff__blk840 * (((((2.0 * locals.var_t0__blk808_dn6) * locals.var_t0__blk808) + (assign27250_e24861 * locals.var_t0__blk808_dn6)) / 3.0) + ((locals.var_vgsteff__blk840_dn6 * assign27250_e24873) + (locals.var_vgsteff__blk840 * (locals.var_vgsteff__blk840_dn6 - ((4.0 * locals.var_t0__blk808_dn6) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk808_dn6) * locals.var_t0__blk808) + (assign27250_e24879 * locals.var_t0__blk808_dn6)) * locals.var_t0__blk808) + (assign27250_e24881 * locals.var_t0__blk808_dn6)) / 15.0)), (((locals.var_vgsteff__blk840_dn7 * assign27250_e24875) + (locals.var_vgsteff__blk840 * (((((2.0 * locals.var_t0__blk808_dn7) * locals.var_t0__blk808) + (assign27250_e24861 * locals.var_t0__blk808_dn7)) / 3.0) + ((locals.var_vgsteff__blk840_dn7 * assign27250_e24873) + (locals.var_vgsteff__blk840 * (locals.var_vgsteff__blk840_dn7 - ((4.0 * locals.var_t0__blk808_dn7) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk808_dn7) * locals.var_t0__blk808) + (assign27250_e24879 * locals.var_t0__blk808_dn7)) * locals.var_t0__blk808) + (assign27250_e24881 * locals.var_t0__blk808_dn7)) / 15.0)), (((locals.var_vgsteff__blk840_dn8 * assign27250_e24875) + (locals.var_vgsteff__blk840 * (((((2.0 * locals.var_t0__blk808_dn8) * locals.var_t0__blk808) + (assign27250_e24861 * locals.var_t0__blk808_dn8)) / 3.0) + ((locals.var_vgsteff__blk840_dn8 * assign27250_e24873) + (locals.var_vgsteff__blk840 * (locals.var_vgsteff__blk840_dn8 - ((4.0 * locals.var_t0__blk808_dn8) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk808_dn8) * locals.var_t0__blk808) + (assign27250_e24879 * locals.var_t0__blk808_dn8)) * locals.var_t0__blk808) + (assign27250_e24881 * locals.var_t0__blk808_dn8)) / 15.0)), (((locals.var_vgsteff__blk840_dn9 * assign27250_e24875) + (locals.var_vgsteff__blk840 * (((((2.0 * locals.var_t0__blk808_dn9) * locals.var_t0__blk808) + (assign27250_e24861 * locals.var_t0__blk808_dn9)) / 3.0) + ((locals.var_vgsteff__blk840_dn9 * assign27250_e24873) + (locals.var_vgsteff__blk840 * (locals.var_vgsteff__blk840_dn9 - ((4.0 * locals.var_t0__blk808_dn9) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk808_dn9) * locals.var_t0__blk808) + (assign27250_e24879 * locals.var_t0__blk808_dn9)) * locals.var_t0__blk808) + (assign27250_e24881 * locals.var_t0__blk808_dn9)) / 15.0)), (((locals.var_vgsteff__blk840_dn10 * assign27250_e24875) + (locals.var_vgsteff__blk840 * (((((2.0 * locals.var_t0__blk808_dn10) * locals.var_t0__blk808) + (assign27250_e24861 * locals.var_t0__blk808_dn10)) / 3.0) + ((locals.var_vgsteff__blk840_dn10 * assign27250_e24873) + (locals.var_vgsteff__blk840 * (locals.var_vgsteff__blk840_dn10 - ((4.0 * locals.var_t0__blk808_dn10) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk808_dn10) * locals.var_t0__blk808) + (assign27250_e24879 * locals.var_t0__blk808_dn10)) * locals.var_t0__blk808) + (assign27250_e24881 * locals.var_t0__blk808_dn10)) / 15.0)), (((locals.var_vgsteff__blk840_dn11 * assign27250_e24875) + (locals.var_vgsteff__blk840 * (((((2.0 * locals.var_t0__blk808_dn11) * locals.var_t0__blk808) + (assign27250_e24861 * locals.var_t0__blk808_dn11)) / 3.0) + ((locals.var_vgsteff__blk840_dn11 * assign27250_e24873) + (locals.var_vgsteff__blk840 * (locals.var_vgsteff__blk840_dn11 - ((4.0 * locals.var_t0__blk808_dn11) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk808_dn11) * locals.var_t0__blk808) + (assign27250_e24879 * locals.var_t0__blk808_dn11)) * locals.var_t0__blk808) + (assign27250_e24881 * locals.var_t0__blk808_dn11)) / 15.0)), (((locals.var_vgsteff__blk840_dn12 * assign27250_e24875) + (locals.var_vgsteff__blk840 * (((((2.0 * locals.var_t0__blk808_dn12) * locals.var_t0__blk808) + (assign27250_e24861 * locals.var_t0__blk808_dn12)) / 3.0) + ((locals.var_vgsteff__blk840_dn12 * assign27250_e24873) + (locals.var_vgsteff__blk840 * (locals.var_vgsteff__blk840_dn12 - ((4.0 * locals.var_t0__blk808_dn12) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk808_dn12) * locals.var_t0__blk808) + (assign27250_e24879 * locals.var_t0__blk808_dn12)) * locals.var_t0__blk808) + (assign27250_e24881 * locals.var_t0__blk808_dn12)) / 15.0)),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign27250_e24888;
        locals.var_t3__blk811_dn3 = assign27250_e24888_d_n3;
        locals.var_t3__blk811_dn4 = assign27250_e24888_d_n4;
        locals.var_t3__blk811_dn5 = assign27250_e24888_d_n5;
        locals.var_t3__blk811_dn6 = assign27250_e24888_d_n6;
        locals.var_t3__blk811_dn7 = assign27250_e24888_d_n7;
        locals.var_t3__blk811_dn8 = assign27250_e24888_d_n8;
        locals.var_t3__blk811_dn9 = assign27250_e24888_d_n9;
        locals.var_t3__blk811_dn10 = assign27250_e24888_d_n10;
        locals.var_t3__blk811_dn11 = assign27250_e24888_d_n11;
        locals.var_t3__blk811_dn12 = assign27250_e24888_d_n12;
        locals.var_t3__blk811_rv = 0.0;

        let (assign27260_e24900, assign27260_e24900_d_n3, assign27260_e24900_d_n4, assign27260_e24900_d_n5, assign27260_e24900_d_n6, assign27260_e24900_d_n7, assign27260_e24900_d_n8, assign27260_e24900_d_n9, assign27260_e24900_d_n10, assign27260_e24900_d_n11, assign27260_e24900_d_n12,) = {
    if (((locals.var_guard1367 != 0.0) && (locals.var_guard1380 == 0.0)) && (locals.var_guard1382 != 0.0)) {
        let assign27260_e24896: f64 = (-locals.var_t2__blk810);
        let assign27260_e24898: f64 = (assign27260_e24896 * locals.var_t3__blk811);
        (assign27260_e24898, (((-locals.var_t2__blk810_dn3) * locals.var_t3__blk811) + (assign27260_e24896 * locals.var_t3__blk811_dn3)), (((-locals.var_t2__blk810_dn4) * locals.var_t3__blk811) + (assign27260_e24896 * locals.var_t3__blk811_dn4)), (((-locals.var_t2__blk810_dn5) * locals.var_t3__blk811) + (assign27260_e24896 * locals.var_t3__blk811_dn5)), (((-locals.var_t2__blk810_dn6) * locals.var_t3__blk811) + (assign27260_e24896 * locals.var_t3__blk811_dn6)), (((-locals.var_t2__blk810_dn7) * locals.var_t3__blk811) + (assign27260_e24896 * locals.var_t3__blk811_dn7)), (((-locals.var_t2__blk810_dn8) * locals.var_t3__blk811) + (assign27260_e24896 * locals.var_t3__blk811_dn8)), (((-locals.var_t2__blk810_dn9) * locals.var_t3__blk811) + (assign27260_e24896 * locals.var_t3__blk811_dn9)), (((-locals.var_t2__blk810_dn10) * locals.var_t3__blk811) + (assign27260_e24896 * locals.var_t3__blk811_dn10)), (((-locals.var_t2__blk810_dn11) * locals.var_t3__blk811) + (assign27260_e24896 * locals.var_t3__blk811_dn11)), (((-locals.var_t2__blk810_dn12) * locals.var_t3__blk811) + (assign27260_e24896 * locals.var_t3__blk811_dn12)),)
    } else {
        (locals.var_qsrc, locals.var_qsrc_dn3, locals.var_qsrc_dn4, locals.var_qsrc_dn5, locals.var_qsrc_dn6, locals.var_qsrc_dn7, locals.var_qsrc_dn8, locals.var_qsrc_dn9, locals.var_qsrc_dn10, locals.var_qsrc_dn11, locals.var_qsrc_dn12,)
    }
};
        locals.var_qsrc = assign27260_e24900;
        locals.var_qsrc_dn3 = assign27260_e24900_d_n3;
        locals.var_qsrc_dn4 = assign27260_e24900_d_n4;
        locals.var_qsrc_dn5 = assign27260_e24900_d_n5;
        locals.var_qsrc_dn6 = assign27260_e24900_d_n6;
        locals.var_qsrc_dn7 = assign27260_e24900_d_n7;
        locals.var_qsrc_dn8 = assign27260_e24900_d_n8;
        locals.var_qsrc_dn9 = assign27260_e24900_d_n9;
        locals.var_qsrc_dn10 = assign27260_e24900_d_n10;
        locals.var_qsrc_dn11 = assign27260_e24900_d_n11;
        locals.var_qsrc_dn12 = assign27260_e24900_d_n12;
        locals.var_qsrc_rv = 0.0;

        let assign27270_e24911: f64 = if (((locals.var_b4soisoimod != 2.0) && (locals.var_b4soibodymod != 0.0)) && (p.p27 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1383 = assign27270_e24911;
        locals.var_guard1383_rv = 0.0;

        let (assign27280_e24924, assign27280_e24924_d_n3, assign27280_e24924_d_n4, assign27280_e24924_d_n5, assign27280_e24924_d_n6, assign27280_e24924_d_n7, assign27280_e24924_d_n8, assign27280_e24924_d_n9, assign27280_e24924_d_n10, assign27280_e24924_d_n11, assign27280_e24924_d_n12,) = {
    if ((((locals.var_guard1367 != 0.0) && (locals.var_guard1380 == 0.0)) && (locals.var_guard1382 != 0.0)) && (locals.var_guard1383 != 0.0)) {
        let assign27280_e24922: f64 = (locals.var_t12 / 12.0);
        (assign27280_e24922, (locals.var_t12_dn3 / 12.0), (locals.var_t12_dn4 / 12.0), (locals.var_t12_dn5 / 12.0), (locals.var_t12_dn6 / 12.0), (locals.var_t12_dn7 / 12.0), (locals.var_t12_dn8 / 12.0), (locals.var_t12_dn9 / 12.0), (locals.var_t12_dn10 / 12.0), (locals.var_t12_dn11 / 12.0), (locals.var_t12_dn12 / 12.0),)
    } else {
        (locals.var_t12, locals.var_t12_dn3, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn12,)
    }
};
        locals.var_t12 = assign27280_e24924;
        locals.var_t12_dn3 = assign27280_e24924_d_n3;
        locals.var_t12_dn4 = assign27280_e24924_d_n4;
        locals.var_t12_dn5 = assign27280_e24924_d_n5;
        locals.var_t12_dn6 = assign27280_e24924_d_n6;
        locals.var_t12_dn7 = assign27280_e24924_d_n7;
        locals.var_t12_dn8 = assign27280_e24924_d_n8;
        locals.var_t12_dn9 = assign27280_e24924_d_n9;
        locals.var_t12_dn10 = assign27280_e24924_d_n10;
        locals.var_t12_dn11 = assign27280_e24924_d_n11;
        locals.var_t12_dn12 = assign27280_e24924_d_n12;
        locals.var_t12_rv = 0.0;

        let (assign27290_e24941, assign27290_e24941_d_n3, assign27290_e24941_d_n4, assign27290_e24941_d_n5, assign27290_e24941_d_n6, assign27290_e24941_d_n7, assign27290_e24941_d_n8, assign27290_e24941_d_n9, assign27290_e24941_d_n10, assign27290_e24941_d_n11, assign27290_e24941_d_n12,) = {
    if ((((locals.var_guard1367 != 0.0) && (locals.var_guard1380 == 0.0)) && (locals.var_guard1382 != 0.0)) && (locals.var_guard1383 != 0.0)) {
        let assign27290_e24935: f64 = (0.5 * locals.var_coxwl2);
        let assign27290_e24938: f64 = (locals.var_t12 * locals.var_t12);
        let assign27290_e24939: f64 = (assign27290_e24935 / assign27290_e24938);
        (assign27290_e24939, ((((0.5 * locals.var_coxwl2_dn3) * assign27290_e24938) - (assign27290_e24935 * ((locals.var_t12_dn3 * locals.var_t12) + (locals.var_t12 * locals.var_t12_dn3)))) / (assign27290_e24938 * assign27290_e24938)), ((((0.5 * locals.var_coxwl2_dn4) * assign27290_e24938) - (assign27290_e24935 * ((locals.var_t12_dn4 * locals.var_t12) + (locals.var_t12 * locals.var_t12_dn4)))) / (assign27290_e24938 * assign27290_e24938)), ((((0.5 * locals.var_coxwl2_dn5) * assign27290_e24938) - (assign27290_e24935 * ((locals.var_t12_dn5 * locals.var_t12) + (locals.var_t12 * locals.var_t12_dn5)))) / (assign27290_e24938 * assign27290_e24938)), ((((0.5 * locals.var_coxwl2_dn6) * assign27290_e24938) - (assign27290_e24935 * ((locals.var_t12_dn6 * locals.var_t12) + (locals.var_t12 * locals.var_t12_dn6)))) / (assign27290_e24938 * assign27290_e24938)), ((((0.5 * locals.var_coxwl2_dn7) * assign27290_e24938) - (assign27290_e24935 * ((locals.var_t12_dn7 * locals.var_t12) + (locals.var_t12 * locals.var_t12_dn7)))) / (assign27290_e24938 * assign27290_e24938)), ((((0.5 * locals.var_coxwl2_dn8) * assign27290_e24938) - (assign27290_e24935 * ((locals.var_t12_dn8 * locals.var_t12) + (locals.var_t12 * locals.var_t12_dn8)))) / (assign27290_e24938 * assign27290_e24938)), ((((0.5 * locals.var_coxwl2_dn9) * assign27290_e24938) - (assign27290_e24935 * ((locals.var_t12_dn9 * locals.var_t12) + (locals.var_t12 * locals.var_t12_dn9)))) / (assign27290_e24938 * assign27290_e24938)), ((((0.5 * locals.var_coxwl2_dn10) * assign27290_e24938) - (assign27290_e24935 * ((locals.var_t12_dn10 * locals.var_t12) + (locals.var_t12 * locals.var_t12_dn10)))) / (assign27290_e24938 * assign27290_e24938)), ((((0.5 * locals.var_coxwl2_dn11) * assign27290_e24938) - (assign27290_e24935 * ((locals.var_t12_dn11 * locals.var_t12) + (locals.var_t12 * locals.var_t12_dn11)))) / (assign27290_e24938 * assign27290_e24938)), ((((0.5 * locals.var_coxwl2_dn12) * assign27290_e24938) - (assign27290_e24935 * ((locals.var_t12_dn12 * locals.var_t12) + (locals.var_t12 * locals.var_t12_dn12)))) / (assign27290_e24938 * assign27290_e24938)),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign27290_e24941;
        locals.var_t2__blk810_dn3 = assign27290_e24941_d_n3;
        locals.var_t2__blk810_dn4 = assign27290_e24941_d_n4;
        locals.var_t2__blk810_dn5 = assign27290_e24941_d_n5;
        locals.var_t2__blk810_dn6 = assign27290_e24941_d_n6;
        locals.var_t2__blk810_dn7 = assign27290_e24941_d_n7;
        locals.var_t2__blk810_dn8 = assign27290_e24941_d_n8;
        locals.var_t2__blk810_dn9 = assign27290_e24941_d_n9;
        locals.var_t2__blk810_dn10 = assign27290_e24941_d_n10;
        locals.var_t2__blk810_dn11 = assign27290_e24941_d_n11;
        locals.var_t2__blk810_dn12 = assign27290_e24941_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign27300_e24980, assign27300_e24980_d_n3, assign27300_e24980_d_n4, assign27300_e24980_d_n5, assign27300_e24980_d_n6, assign27300_e24980_d_n7, assign27300_e24980_d_n8, assign27300_e24980_d_n9, assign27300_e24980_d_n10, assign27300_e24980_d_n11, assign27300_e24980_d_n12,) = {
    if ((((locals.var_guard1367 != 0.0) && (locals.var_guard1380 == 0.0)) && (locals.var_guard1382 != 0.0)) && (locals.var_guard1383 != 0.0)) {
        let assign27300_e24953: f64 = (2.0 * locals.var_t02);
        let assign27300_e24955: f64 = (assign27300_e24953 * locals.var_t02);
        let assign27300_e24957: f64 = (assign27300_e24955 / 3.0);
        let assign27300_e24962: f64 = (4.0 * locals.var_t02);
        let assign27300_e24964: f64 = (assign27300_e24962 / 3.0);
        let assign27300_e24965: f64 = (locals.var_vgsteff2 - assign27300_e24964);
        let assign27300_e24966: f64 = (locals.var_vgsteff2 * assign27300_e24965);
        let assign27300_e24967: f64 = (assign27300_e24957 + assign27300_e24966);
        let assign27300_e24968: f64 = (locals.var_vgsteff2 * assign27300_e24967);
        let assign27300_e24971: f64 = (2.0 * locals.var_t02);
        let assign27300_e24973: f64 = (assign27300_e24971 * locals.var_t02);
        let assign27300_e24975: f64 = (assign27300_e24973 * locals.var_t02);
        let assign27300_e24977: f64 = (assign27300_e24975 / 15.0);
        let assign27300_e24978: f64 = (assign27300_e24968 - assign27300_e24977);
        (assign27300_e24978, (((locals.var_vgsteff2_dn3 * assign27300_e24967) + (locals.var_vgsteff2 * (((((2.0 * locals.var_t02_dn3) * locals.var_t02) + (assign27300_e24953 * locals.var_t02_dn3)) / 3.0) + ((locals.var_vgsteff2_dn3 * assign27300_e24965) + (locals.var_vgsteff2 * (locals.var_vgsteff2_dn3 - ((4.0 * locals.var_t02_dn3) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn3) * locals.var_t02) + (assign27300_e24971 * locals.var_t02_dn3)) * locals.var_t02) + (assign27300_e24973 * locals.var_t02_dn3)) / 15.0)), (((locals.var_vgsteff2_dn4 * assign27300_e24967) + (locals.var_vgsteff2 * (((((2.0 * locals.var_t02_dn4) * locals.var_t02) + (assign27300_e24953 * locals.var_t02_dn4)) / 3.0) + ((locals.var_vgsteff2_dn4 * assign27300_e24965) + (locals.var_vgsteff2 * (locals.var_vgsteff2_dn4 - ((4.0 * locals.var_t02_dn4) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn4) * locals.var_t02) + (assign27300_e24971 * locals.var_t02_dn4)) * locals.var_t02) + (assign27300_e24973 * locals.var_t02_dn4)) / 15.0)), (((locals.var_vgsteff2_dn5 * assign27300_e24967) + (locals.var_vgsteff2 * (((((2.0 * locals.var_t02_dn5) * locals.var_t02) + (assign27300_e24953 * locals.var_t02_dn5)) / 3.0) + ((locals.var_vgsteff2_dn5 * assign27300_e24965) + (locals.var_vgsteff2 * (locals.var_vgsteff2_dn5 - ((4.0 * locals.var_t02_dn5) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn5) * locals.var_t02) + (assign27300_e24971 * locals.var_t02_dn5)) * locals.var_t02) + (assign27300_e24973 * locals.var_t02_dn5)) / 15.0)), (((locals.var_vgsteff2_dn6 * assign27300_e24967) + (locals.var_vgsteff2 * (((((2.0 * locals.var_t02_dn6) * locals.var_t02) + (assign27300_e24953 * locals.var_t02_dn6)) / 3.0) + ((locals.var_vgsteff2_dn6 * assign27300_e24965) + (locals.var_vgsteff2 * (locals.var_vgsteff2_dn6 - ((4.0 * locals.var_t02_dn6) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn6) * locals.var_t02) + (assign27300_e24971 * locals.var_t02_dn6)) * locals.var_t02) + (assign27300_e24973 * locals.var_t02_dn6)) / 15.0)), (((locals.var_vgsteff2_dn7 * assign27300_e24967) + (locals.var_vgsteff2 * (((((2.0 * locals.var_t02_dn7) * locals.var_t02) + (assign27300_e24953 * locals.var_t02_dn7)) / 3.0) + ((locals.var_vgsteff2_dn7 * assign27300_e24965) + (locals.var_vgsteff2 * (locals.var_vgsteff2_dn7 - ((4.0 * locals.var_t02_dn7) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn7) * locals.var_t02) + (assign27300_e24971 * locals.var_t02_dn7)) * locals.var_t02) + (assign27300_e24973 * locals.var_t02_dn7)) / 15.0)), (((locals.var_vgsteff2_dn8 * assign27300_e24967) + (locals.var_vgsteff2 * (((((2.0 * locals.var_t02_dn8) * locals.var_t02) + (assign27300_e24953 * locals.var_t02_dn8)) / 3.0) + ((locals.var_vgsteff2_dn8 * assign27300_e24965) + (locals.var_vgsteff2 * (locals.var_vgsteff2_dn8 - ((4.0 * locals.var_t02_dn8) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn8) * locals.var_t02) + (assign27300_e24971 * locals.var_t02_dn8)) * locals.var_t02) + (assign27300_e24973 * locals.var_t02_dn8)) / 15.0)), (((locals.var_vgsteff2_dn9 * assign27300_e24967) + (locals.var_vgsteff2 * (((((2.0 * locals.var_t02_dn9) * locals.var_t02) + (assign27300_e24953 * locals.var_t02_dn9)) / 3.0) + ((locals.var_vgsteff2_dn9 * assign27300_e24965) + (locals.var_vgsteff2 * (locals.var_vgsteff2_dn9 - ((4.0 * locals.var_t02_dn9) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn9) * locals.var_t02) + (assign27300_e24971 * locals.var_t02_dn9)) * locals.var_t02) + (assign27300_e24973 * locals.var_t02_dn9)) / 15.0)), (((locals.var_vgsteff2_dn10 * assign27300_e24967) + (locals.var_vgsteff2 * (((((2.0 * locals.var_t02_dn10) * locals.var_t02) + (assign27300_e24953 * locals.var_t02_dn10)) / 3.0) + ((locals.var_vgsteff2_dn10 * assign27300_e24965) + (locals.var_vgsteff2 * (locals.var_vgsteff2_dn10 - ((4.0 * locals.var_t02_dn10) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn10) * locals.var_t02) + (assign27300_e24971 * locals.var_t02_dn10)) * locals.var_t02) + (assign27300_e24973 * locals.var_t02_dn10)) / 15.0)), (((locals.var_vgsteff2_dn11 * assign27300_e24967) + (locals.var_vgsteff2 * (((((2.0 * locals.var_t02_dn11) * locals.var_t02) + (assign27300_e24953 * locals.var_t02_dn11)) / 3.0) + ((locals.var_vgsteff2_dn11 * assign27300_e24965) + (locals.var_vgsteff2 * (locals.var_vgsteff2_dn11 - ((4.0 * locals.var_t02_dn11) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn11) * locals.var_t02) + (assign27300_e24971 * locals.var_t02_dn11)) * locals.var_t02) + (assign27300_e24973 * locals.var_t02_dn11)) / 15.0)), (((locals.var_vgsteff2_dn12 * assign27300_e24967) + (locals.var_vgsteff2 * (((((2.0 * locals.var_t02_dn12) * locals.var_t02) + (assign27300_e24953 * locals.var_t02_dn12)) / 3.0) + ((locals.var_vgsteff2_dn12 * assign27300_e24965) + (locals.var_vgsteff2 * (locals.var_vgsteff2_dn12 - ((4.0 * locals.var_t02_dn12) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn12) * locals.var_t02) + (assign27300_e24971 * locals.var_t02_dn12)) * locals.var_t02) + (assign27300_e24973 * locals.var_t02_dn12)) / 15.0)),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign27300_e24980;
        locals.var_t3__blk811_dn3 = assign27300_e24980_d_n3;
        locals.var_t3__blk811_dn4 = assign27300_e24980_d_n4;
        locals.var_t3__blk811_dn5 = assign27300_e24980_d_n5;
        locals.var_t3__blk811_dn6 = assign27300_e24980_d_n6;
        locals.var_t3__blk811_dn7 = assign27300_e24980_d_n7;
        locals.var_t3__blk811_dn8 = assign27300_e24980_d_n8;
        locals.var_t3__blk811_dn9 = assign27300_e24980_d_n9;
        locals.var_t3__blk811_dn10 = assign27300_e24980_d_n10;
        locals.var_t3__blk811_dn11 = assign27300_e24980_d_n11;
        locals.var_t3__blk811_dn12 = assign27300_e24980_d_n12;
        locals.var_t3__blk811_rv = 0.0;

        let (assign27310_e24994, assign27310_e24994_d_n3, assign27310_e24994_d_n4, assign27310_e24994_d_n5, assign27310_e24994_d_n6, assign27310_e24994_d_n7, assign27310_e24994_d_n8, assign27310_e24994_d_n9, assign27310_e24994_d_n10, assign27310_e24994_d_n11, assign27310_e24994_d_n12,) = {
    if ((((locals.var_guard1367 != 0.0) && (locals.var_guard1380 == 0.0)) && (locals.var_guard1382 != 0.0)) && (locals.var_guard1383 != 0.0)) {
        let assign27310_e24990: f64 = (-locals.var_t2__blk810);
        let assign27310_e24992: f64 = (assign27310_e24990 * locals.var_t3__blk811);
        (assign27310_e24992, (((-locals.var_t2__blk810_dn3) * locals.var_t3__blk811) + (assign27310_e24990 * locals.var_t3__blk811_dn3)), (((-locals.var_t2__blk810_dn4) * locals.var_t3__blk811) + (assign27310_e24990 * locals.var_t3__blk811_dn4)), (((-locals.var_t2__blk810_dn5) * locals.var_t3__blk811) + (assign27310_e24990 * locals.var_t3__blk811_dn5)), (((-locals.var_t2__blk810_dn6) * locals.var_t3__blk811) + (assign27310_e24990 * locals.var_t3__blk811_dn6)), (((-locals.var_t2__blk810_dn7) * locals.var_t3__blk811) + (assign27310_e24990 * locals.var_t3__blk811_dn7)), (((-locals.var_t2__blk810_dn8) * locals.var_t3__blk811) + (assign27310_e24990 * locals.var_t3__blk811_dn8)), (((-locals.var_t2__blk810_dn9) * locals.var_t3__blk811) + (assign27310_e24990 * locals.var_t3__blk811_dn9)), (((-locals.var_t2__blk810_dn10) * locals.var_t3__blk811) + (assign27310_e24990 * locals.var_t3__blk811_dn10)), (((-locals.var_t2__blk810_dn11) * locals.var_t3__blk811) + (assign27310_e24990 * locals.var_t3__blk811_dn11)), (((-locals.var_t2__blk810_dn12) * locals.var_t3__blk811) + (assign27310_e24990 * locals.var_t3__blk811_dn12)),)
    } else {
        (locals.var_qsrc2, locals.var_qsrc2_dn3, locals.var_qsrc2_dn4, locals.var_qsrc2_dn5, locals.var_qsrc2_dn6, locals.var_qsrc2_dn7, locals.var_qsrc2_dn8, locals.var_qsrc2_dn9, locals.var_qsrc2_dn10, locals.var_qsrc2_dn11, locals.var_qsrc2_dn12,)
    }
};
        locals.var_qsrc2 = assign27310_e24994;
        locals.var_qsrc2_dn3 = assign27310_e24994_d_n3;
        locals.var_qsrc2_dn4 = assign27310_e24994_d_n4;
        locals.var_qsrc2_dn5 = assign27310_e24994_d_n5;
        locals.var_qsrc2_dn6 = assign27310_e24994_d_n6;
        locals.var_qsrc2_dn7 = assign27310_e24994_d_n7;
        locals.var_qsrc2_dn8 = assign27310_e24994_d_n8;
        locals.var_qsrc2_dn9 = assign27310_e24994_d_n9;
        locals.var_qsrc2_dn10 = assign27310_e24994_d_n10;
        locals.var_qsrc2_dn11 = assign27310_e24994_d_n11;
        locals.var_qsrc2_dn12 = assign27310_e24994_d_n12;
        locals.var_qsrc2_rv = 0.0;

        let (assign27320_e25007, assign27320_e25007_d_n3, assign27320_e25007_d_n4, assign27320_e25007_d_n5, assign27320_e25007_d_n6, assign27320_e25007_d_n7, assign27320_e25007_d_n8, assign27320_e25007_d_n9, assign27320_e25007_d_n10, assign27320_e25007_d_n11, assign27320_e25007_d_n12,) = {
    if ((((locals.var_guard1367 != 0.0) && (locals.var_guard1380 == 0.0)) && (locals.var_guard1382 != 0.0)) && (locals.var_guard1383 != 0.0)) {
        let assign27320_e25005: f64 = (locals.var_qsrc + locals.var_qsrc2);
        (assign27320_e25005, (locals.var_qsrc_dn3 + locals.var_qsrc2_dn3), (locals.var_qsrc_dn4 + locals.var_qsrc2_dn4), (locals.var_qsrc_dn5 + locals.var_qsrc2_dn5), (locals.var_qsrc_dn6 + locals.var_qsrc2_dn6), (locals.var_qsrc_dn7 + locals.var_qsrc2_dn7), (locals.var_qsrc_dn8 + locals.var_qsrc2_dn8), (locals.var_qsrc_dn9 + locals.var_qsrc2_dn9), (locals.var_qsrc_dn10 + locals.var_qsrc2_dn10), (locals.var_qsrc_dn11 + locals.var_qsrc2_dn11), (locals.var_qsrc_dn12 + locals.var_qsrc2_dn12),)
    } else {
        (locals.var_qsrc, locals.var_qsrc_dn3, locals.var_qsrc_dn4, locals.var_qsrc_dn5, locals.var_qsrc_dn6, locals.var_qsrc_dn7, locals.var_qsrc_dn8, locals.var_qsrc_dn9, locals.var_qsrc_dn10, locals.var_qsrc_dn11, locals.var_qsrc_dn12,)
    }
};
        locals.var_qsrc = assign27320_e25007;
        locals.var_qsrc_dn3 = assign27320_e25007_d_n3;
        locals.var_qsrc_dn4 = assign27320_e25007_d_n4;
        locals.var_qsrc_dn5 = assign27320_e25007_d_n5;
        locals.var_qsrc_dn6 = assign27320_e25007_d_n6;
        locals.var_qsrc_dn7 = assign27320_e25007_d_n7;
        locals.var_qsrc_dn8 = assign27320_e25007_d_n8;
        locals.var_qsrc_dn9 = assign27320_e25007_d_n9;
        locals.var_qsrc_dn10 = assign27320_e25007_d_n10;
        locals.var_qsrc_dn11 = assign27320_e25007_d_n11;
        locals.var_qsrc_dn12 = assign27320_e25007_d_n12;
        locals.var_qsrc_rv = 0.0;

        let (assign27330_e25022, assign27330_e25022_d_n3, assign27330_e25022_d_n4, assign27330_e25022_d_n5, assign27330_e25022_d_n6, assign27330_e25022_d_n7, assign27330_e25022_d_n8, assign27330_e25022_d_n9, assign27330_e25022_d_n10, assign27330_e25022_d_n11, assign27330_e25022_d_n12,) = {
    if (((locals.var_guard1367 != 0.0) && (locals.var_guard1380 == 0.0)) && (locals.var_guard1382 == 0.0)) {
        let assign27330_e25016: f64 = (-0.5);
        let assign27330_e25019: f64 = (locals.var_qinv + locals.var_qbulk);
        let assign27330_e25020: f64 = (assign27330_e25016 * assign27330_e25019);
        (assign27330_e25020, (assign27330_e25016 * (locals.var_qinv_dn3 + locals.var_qbulk_dn3)), (assign27330_e25016 * (locals.var_qinv_dn4 + locals.var_qbulk_dn4)), (assign27330_e25016 * (locals.var_qinv_dn5 + locals.var_qbulk_dn5)), (assign27330_e25016 * (locals.var_qinv_dn6 + locals.var_qbulk_dn6)), (assign27330_e25016 * (locals.var_qinv_dn7 + locals.var_qbulk_dn7)), (assign27330_e25016 * (locals.var_qinv_dn8 + locals.var_qbulk_dn8)), (assign27330_e25016 * (locals.var_qinv_dn9 + locals.var_qbulk_dn9)), (assign27330_e25016 * (locals.var_qinv_dn10 + locals.var_qbulk_dn10)), (assign27330_e25016 * (locals.var_qinv_dn11 + locals.var_qbulk_dn11)), (assign27330_e25016 * (locals.var_qinv_dn12 + locals.var_qbulk_dn12)),)
    } else {
        (locals.var_qsrc, locals.var_qsrc_dn3, locals.var_qsrc_dn4, locals.var_qsrc_dn5, locals.var_qsrc_dn6, locals.var_qsrc_dn7, locals.var_qsrc_dn8, locals.var_qsrc_dn9, locals.var_qsrc_dn10, locals.var_qsrc_dn11, locals.var_qsrc_dn12,)
    }
};
        locals.var_qsrc = assign27330_e25022;
        locals.var_qsrc_dn3 = assign27330_e25022_d_n3;
        locals.var_qsrc_dn4 = assign27330_e25022_d_n4;
        locals.var_qsrc_dn5 = assign27330_e25022_d_n5;
        locals.var_qsrc_dn6 = assign27330_e25022_d_n6;
        locals.var_qsrc_dn7 = assign27330_e25022_d_n7;
        locals.var_qsrc_dn8 = assign27330_e25022_d_n8;
        locals.var_qsrc_dn9 = assign27330_e25022_d_n9;
        locals.var_qsrc_dn10 = assign27330_e25022_d_n10;
        locals.var_qsrc_dn11 = assign27330_e25022_d_n11;
        locals.var_qsrc_dn12 = assign27330_e25022_d_n12;
        locals.var_qsrc_rv = 0.0;

        let assign27340_e25025: f64 = if locals.var_b4soisoimod == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1384 = assign27340_e25025;
        locals.var_guard1384_rv = 0.0;

        let (assign27350_e25031, assign27350_e25031_d_n3, assign27350_e25031_d_n4, assign27350_e25031_d_n5, assign27350_e25031_d_n6, assign27350_e25031_d_n7, assign27350_e25031_d_n8, assign27350_e25031_d_n9, assign27350_e25031_d_n10, assign27350_e25031_d_n11, assign27350_e25031_d_n12,) = {
    if ((locals.var_guard1367 != 0.0) && (locals.var_guard1384 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qe1, locals.var_qe1_dn3, locals.var_qe1_dn4, locals.var_qe1_dn5, locals.var_qe1_dn6, locals.var_qe1_dn7, locals.var_qe1_dn8, locals.var_qe1_dn9, locals.var_qe1_dn10, locals.var_qe1_dn11, locals.var_qe1_dn12,)
    }
};
        locals.var_qe1 = assign27350_e25031;
        locals.var_qe1_dn3 = assign27350_e25031_d_n3;
        locals.var_qe1_dn4 = assign27350_e25031_d_n4;
        locals.var_qe1_dn5 = assign27350_e25031_d_n5;
        locals.var_qe1_dn6 = assign27350_e25031_d_n6;
        locals.var_qe1_dn7 = assign27350_e25031_d_n7;
        locals.var_qe1_dn8 = assign27350_e25031_d_n8;
        locals.var_qe1_dn9 = assign27350_e25031_d_n9;
        locals.var_qe1_dn10 = assign27350_e25031_d_n10;
        locals.var_qe1_dn11 = assign27350_e25031_d_n11;
        locals.var_qe1_dn12 = assign27350_e25031_d_n12;
        locals.var_qe1_rv = 0.0;

        let (assign27360_e25052, assign27360_e25052_d_n3, assign27360_e25052_d_n4, assign27360_e25052_d_n5, assign27360_e25052_d_n6, assign27360_e25052_d_n7, assign27360_e25052_d_n8, assign27360_e25052_d_n9, assign27360_e25052_d_n10, assign27360_e25052_d_n11, assign27360_e25052_d_n12,) = {
    if ((locals.var_guard1367 != 0.0) && (locals.var_guard1384 == 0.0)) {
        let assign27360_e25038: f64 = (locals.var_pparam_b4soikb1 * p.p361);
        let assign27360_e25040: f64 = (assign27360_e25038 * locals.var_cbox);
        let assign27360_e25043: f64 = (locals.var_pparam_b4soiweffcv / p.p23);
        let assign27360_e25045: f64 = (assign27360_e25043 * p.p3);
        let assign27360_e25047: f64 = (assign27360_e25045 * locals.var_pparam_b4soileffcvbg);
        let assign27360_e25049: f64 = (assign27360_e25047 + p.p29);
        let assign27360_e25050: f64 = (assign27360_e25040 * assign27360_e25049);
        (assign27360_e25050, ((((locals.var_pparam_b4soikb1_dn3 * p.p361) * locals.var_cbox) * assign27360_e25049) + (assign27360_e25040 * ((((locals.var_pparam_b4soiweffcv_dn3 / p.p23) * p.p3) * locals.var_pparam_b4soileffcvbg) + (assign27360_e25045 * locals.var_pparam_b4soileffcvbg_dn3)))), ((((locals.var_pparam_b4soikb1_dn4 * p.p361) * locals.var_cbox) * assign27360_e25049) + (assign27360_e25040 * ((((locals.var_pparam_b4soiweffcv_dn4 / p.p23) * p.p3) * locals.var_pparam_b4soileffcvbg) + (assign27360_e25045 * locals.var_pparam_b4soileffcvbg_dn4)))), ((((locals.var_pparam_b4soikb1_dn5 * p.p361) * locals.var_cbox) * assign27360_e25049) + (assign27360_e25040 * ((((locals.var_pparam_b4soiweffcv_dn5 / p.p23) * p.p3) * locals.var_pparam_b4soileffcvbg) + (assign27360_e25045 * locals.var_pparam_b4soileffcvbg_dn5)))), ((((locals.var_pparam_b4soikb1_dn6 * p.p361) * locals.var_cbox) * assign27360_e25049) + (assign27360_e25040 * ((((locals.var_pparam_b4soiweffcv_dn6 / p.p23) * p.p3) * locals.var_pparam_b4soileffcvbg) + (assign27360_e25045 * locals.var_pparam_b4soileffcvbg_dn6)))), ((((locals.var_pparam_b4soikb1_dn7 * p.p361) * locals.var_cbox) * assign27360_e25049) + (assign27360_e25040 * ((((locals.var_pparam_b4soiweffcv_dn7 / p.p23) * p.p3) * locals.var_pparam_b4soileffcvbg) + (assign27360_e25045 * locals.var_pparam_b4soileffcvbg_dn7)))), ((((locals.var_pparam_b4soikb1_dn8 * p.p361) * locals.var_cbox) * assign27360_e25049) + (assign27360_e25040 * ((((locals.var_pparam_b4soiweffcv_dn8 / p.p23) * p.p3) * locals.var_pparam_b4soileffcvbg) + (assign27360_e25045 * locals.var_pparam_b4soileffcvbg_dn8)))), ((((locals.var_pparam_b4soikb1_dn9 * p.p361) * locals.var_cbox) * assign27360_e25049) + (assign27360_e25040 * ((((locals.var_pparam_b4soiweffcv_dn9 / p.p23) * p.p3) * locals.var_pparam_b4soileffcvbg) + (assign27360_e25045 * locals.var_pparam_b4soileffcvbg_dn9)))), ((((locals.var_pparam_b4soikb1_dn10 * p.p361) * locals.var_cbox) * assign27360_e25049) + (assign27360_e25040 * ((((locals.var_pparam_b4soiweffcv_dn10 / p.p23) * p.p3) * locals.var_pparam_b4soileffcvbg) + (assign27360_e25045 * locals.var_pparam_b4soileffcvbg_dn10)))), ((((locals.var_pparam_b4soikb1_dn11 * p.p361) * locals.var_cbox) * assign27360_e25049) + (assign27360_e25040 * ((((locals.var_pparam_b4soiweffcv_dn11 / p.p23) * p.p3) * locals.var_pparam_b4soileffcvbg) + (assign27360_e25045 * locals.var_pparam_b4soileffcvbg_dn11)))), ((((locals.var_pparam_b4soikb1_dn12 * p.p361) * locals.var_cbox) * assign27360_e25049) + (assign27360_e25040 * ((((locals.var_pparam_b4soiweffcv_dn12 / p.p23) * p.p3) * locals.var_pparam_b4soileffcvbg) + (assign27360_e25045 * locals.var_pparam_b4soileffcvbg_dn12)))),)
    } else {
        (locals.var_cboxwl, locals.var_cboxwl_dn3, locals.var_cboxwl_dn4, locals.var_cboxwl_dn5, locals.var_cboxwl_dn6, locals.var_cboxwl_dn7, locals.var_cboxwl_dn8, locals.var_cboxwl_dn9, locals.var_cboxwl_dn10, locals.var_cboxwl_dn11, locals.var_cboxwl_dn12,)
    }
};
        locals.var_cboxwl = assign27360_e25052;
        locals.var_cboxwl_dn3 = assign27360_e25052_d_n3;
        locals.var_cboxwl_dn4 = assign27360_e25052_d_n4;
        locals.var_cboxwl_dn5 = assign27360_e25052_d_n5;
        locals.var_cboxwl_dn6 = assign27360_e25052_d_n6;
        locals.var_cboxwl_dn7 = assign27360_e25052_d_n7;
        locals.var_cboxwl_dn8 = assign27360_e25052_d_n8;
        locals.var_cboxwl_dn9 = assign27360_e25052_d_n9;
        locals.var_cboxwl_dn10 = assign27360_e25052_d_n10;
        locals.var_cboxwl_dn11 = assign27360_e25052_d_n11;
        locals.var_cboxwl_dn12 = assign27360_e25052_d_n12;
        locals.var_cboxwl_rv = 0.0;

        let (assign27370_e25063, assign27370_e25063_d_n3, assign27370_e25063_d_n4, assign27370_e25063_d_n5, assign27370_e25063_d_n6, assign27370_e25063_d_n7, assign27370_e25063_d_n8, assign27370_e25063_d_n9, assign27370_e25063_d_n10, assign27370_e25063_d_n11, assign27370_e25063_d_n12,) = {
    if ((locals.var_guard1367 != 0.0) && (locals.var_guard1384 == 0.0)) {
        let assign27370_e25060: f64 = (locals.var_vesfb - locals.var_vbs_1);
        let assign27370_e25061: f64 = (locals.var_cboxwl * assign27370_e25060);
        (assign27370_e25061, ((locals.var_cboxwl_dn3 * assign27370_e25060) + (locals.var_cboxwl * (locals.var_vesfb_dn3 - locals.var_vbs_1_dn3))), ((locals.var_cboxwl_dn4 * assign27370_e25060) + (locals.var_cboxwl * (locals.var_vesfb_dn4 - locals.var_vbs_1_dn4))), ((locals.var_cboxwl_dn5 * assign27370_e25060) + (locals.var_cboxwl * (locals.var_vesfb_dn5 - locals.var_vbs_1_dn5))), ((locals.var_cboxwl_dn6 * assign27370_e25060) + (locals.var_cboxwl * (locals.var_vesfb_dn6 - locals.var_vbs_1_dn6))), ((locals.var_cboxwl_dn7 * assign27370_e25060) + (locals.var_cboxwl * (locals.var_vesfb_dn7 - locals.var_vbs_1_dn7))), ((locals.var_cboxwl_dn8 * assign27370_e25060) + (locals.var_cboxwl * (locals.var_vesfb_dn8 - locals.var_vbs_1_dn8))), ((locals.var_cboxwl_dn9 * assign27370_e25060) + (locals.var_cboxwl * (locals.var_vesfb_dn9 - locals.var_vbs_1_dn9))), ((locals.var_cboxwl_dn10 * assign27370_e25060) + (locals.var_cboxwl * (locals.var_vesfb_dn10 - locals.var_vbs_1_dn10))), ((locals.var_cboxwl_dn11 * assign27370_e25060) + (locals.var_cboxwl * (locals.var_vesfb_dn11 - locals.var_vbs_1_dn11))), ((locals.var_cboxwl_dn12 * assign27370_e25060) + (locals.var_cboxwl * (locals.var_vesfb_dn12 - locals.var_vbs_1_dn12))),)
    } else {
        (locals.var_qe1, locals.var_qe1_dn3, locals.var_qe1_dn4, locals.var_qe1_dn5, locals.var_qe1_dn6, locals.var_qe1_dn7, locals.var_qe1_dn8, locals.var_qe1_dn9, locals.var_qe1_dn10, locals.var_qe1_dn11, locals.var_qe1_dn12,)
    }
};
        locals.var_qe1 = assign27370_e25063;
        locals.var_qe1_dn3 = assign27370_e25063_d_n3;
        locals.var_qe1_dn4 = assign27370_e25063_d_n4;
        locals.var_qe1_dn5 = assign27370_e25063_d_n5;
        locals.var_qe1_dn6 = assign27370_e25063_d_n6;
        locals.var_qe1_dn7 = assign27370_e25063_d_n7;
        locals.var_qe1_dn8 = assign27370_e25063_d_n8;
        locals.var_qe1_dn9 = assign27370_e25063_d_n9;
        locals.var_qe1_dn10 = assign27370_e25063_d_n10;
        locals.var_qe1_dn11 = assign27370_e25063_d_n11;
        locals.var_qe1_dn12 = assign27370_e25063_d_n12;
        locals.var_qe1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_83(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign27380_e25071, assign27380_e25071_d_n3, assign27380_e25071_d_n4, assign27380_e25071_d_n5, assign27380_e25071_d_n6, assign27380_e25071_d_n7, assign27380_e25071_d_n8, assign27380_e25071_d_n9, assign27380_e25071_d_n10, assign27380_e25071_d_n11, assign27380_e25071_d_n12,) = {
    if (locals.var_guard1367 != 0.0) {
        let assign27380_e25067: f64 = (locals.var_qinv + locals.var_qac0);
        let assign27380_e25069: f64 = (assign27380_e25067 + locals.var_qsub0);
        (assign27380_e25069, ((locals.var_qinv_dn3 + locals.var_qac0_dn3) + locals.var_qsub0_dn3), ((locals.var_qinv_dn4 + locals.var_qac0_dn4) + locals.var_qsub0_dn4), ((locals.var_qinv_dn5 + locals.var_qac0_dn5) + locals.var_qsub0_dn5), ((locals.var_qinv_dn6 + locals.var_qac0_dn6) + locals.var_qsub0_dn6), ((locals.var_qinv_dn7 + locals.var_qac0_dn7) + locals.var_qsub0_dn7), ((locals.var_qinv_dn8 + locals.var_qac0_dn8) + locals.var_qsub0_dn8), ((locals.var_qinv_dn9 + locals.var_qac0_dn9) + locals.var_qsub0_dn9), ((locals.var_qinv_dn10 + locals.var_qac0_dn10) + locals.var_qsub0_dn10), ((locals.var_qinv_dn11 + locals.var_qac0_dn11) + locals.var_qsub0_dn11), ((locals.var_qinv_dn12 + locals.var_qac0_dn12) + locals.var_qsub0_dn12),)
    } else {
        (locals.var_qgate, locals.var_qgate_dn3, locals.var_qgate_dn4, locals.var_qgate_dn5, locals.var_qgate_dn6, locals.var_qgate_dn7, locals.var_qgate_dn8, locals.var_qgate_dn9, locals.var_qgate_dn10, locals.var_qgate_dn11, locals.var_qgate_dn12,)
    }
};
        locals.var_qgate = assign27380_e25071;
        locals.var_qgate_dn3 = assign27380_e25071_d_n3;
        locals.var_qgate_dn4 = assign27380_e25071_d_n4;
        locals.var_qgate_dn5 = assign27380_e25071_d_n5;
        locals.var_qgate_dn6 = assign27380_e25071_d_n6;
        locals.var_qgate_dn7 = assign27380_e25071_d_n7;
        locals.var_qgate_dn8 = assign27380_e25071_d_n8;
        locals.var_qgate_dn9 = assign27380_e25071_d_n9;
        locals.var_qgate_dn10 = assign27380_e25071_d_n10;
        locals.var_qgate_dn11 = assign27380_e25071_d_n11;
        locals.var_qgate_dn12 = assign27380_e25071_d_n12;
        locals.var_qgate_rv = 0.0;

        let (assign27390_e25081, assign27390_e25081_d_n3, assign27390_e25081_d_n4, assign27390_e25081_d_n5, assign27390_e25081_d_n6, assign27390_e25081_d_n7, assign27390_e25081_d_n8, assign27390_e25081_d_n9, assign27390_e25081_d_n10, assign27390_e25081_d_n11, assign27390_e25081_d_n12,) = {
    if (locals.var_guard1367 != 0.0) {
        let assign27390_e25075: f64 = (locals.var_qbulk - locals.var_qac0);
        let assign27390_e25077: f64 = (assign27390_e25075 - locals.var_qsub0);
        let assign27390_e25079: f64 = (assign27390_e25077 - locals.var_qe1);
        (assign27390_e25079, (((locals.var_qbulk_dn3 - locals.var_qac0_dn3) - locals.var_qsub0_dn3) - locals.var_qe1_dn3), (((locals.var_qbulk_dn4 - locals.var_qac0_dn4) - locals.var_qsub0_dn4) - locals.var_qe1_dn4), (((locals.var_qbulk_dn5 - locals.var_qac0_dn5) - locals.var_qsub0_dn5) - locals.var_qe1_dn5), (((locals.var_qbulk_dn6 - locals.var_qac0_dn6) - locals.var_qsub0_dn6) - locals.var_qe1_dn6), (((locals.var_qbulk_dn7 - locals.var_qac0_dn7) - locals.var_qsub0_dn7) - locals.var_qe1_dn7), (((locals.var_qbulk_dn8 - locals.var_qac0_dn8) - locals.var_qsub0_dn8) - locals.var_qe1_dn8), (((locals.var_qbulk_dn9 - locals.var_qac0_dn9) - locals.var_qsub0_dn9) - locals.var_qe1_dn9), (((locals.var_qbulk_dn10 - locals.var_qac0_dn10) - locals.var_qsub0_dn10) - locals.var_qe1_dn10), (((locals.var_qbulk_dn11 - locals.var_qac0_dn11) - locals.var_qsub0_dn11) - locals.var_qe1_dn11), (((locals.var_qbulk_dn12 - locals.var_qac0_dn12) - locals.var_qsub0_dn12) - locals.var_qe1_dn12),)
    } else {
        (locals.var_qbody, locals.var_qbody_dn3, locals.var_qbody_dn4, locals.var_qbody_dn5, locals.var_qbody_dn6, locals.var_qbody_dn7, locals.var_qbody_dn8, locals.var_qbody_dn9, locals.var_qbody_dn10, locals.var_qbody_dn11, locals.var_qbody_dn12,)
    }
};
        locals.var_qbody = assign27390_e25081;
        locals.var_qbody_dn3 = assign27390_e25081_d_n3;
        locals.var_qbody_dn4 = assign27390_e25081_d_n4;
        locals.var_qbody_dn5 = assign27390_e25081_d_n5;
        locals.var_qbody_dn6 = assign27390_e25081_d_n6;
        locals.var_qbody_dn7 = assign27390_e25081_d_n7;
        locals.var_qbody_dn8 = assign27390_e25081_d_n8;
        locals.var_qbody_dn9 = assign27390_e25081_d_n9;
        locals.var_qbody_dn10 = assign27390_e25081_d_n10;
        locals.var_qbody_dn11 = assign27390_e25081_d_n11;
        locals.var_qbody_dn12 = assign27390_e25081_d_n12;
        locals.var_qbody_rv = 0.0;

        let (assign27400_e25085, assign27400_e25085_d_n3, assign27400_e25085_d_n4, assign27400_e25085_d_n5, assign27400_e25085_d_n6, assign27400_e25085_d_n7, assign27400_e25085_d_n8, assign27400_e25085_d_n9, assign27400_e25085_d_n10, assign27400_e25085_d_n11, assign27400_e25085_d_n12,) = {
    if (locals.var_guard1367 != 0.0) {
        (locals.var_qe1, locals.var_qe1_dn3, locals.var_qe1_dn4, locals.var_qe1_dn5, locals.var_qe1_dn6, locals.var_qe1_dn7, locals.var_qe1_dn8, locals.var_qe1_dn9, locals.var_qe1_dn10, locals.var_qe1_dn11, locals.var_qe1_dn12,)
    } else {
        (locals.var_qsub, locals.var_qsub_dn3, locals.var_qsub_dn4, locals.var_qsub_dn5, locals.var_qsub_dn6, locals.var_qsub_dn7, locals.var_qsub_dn8, locals.var_qsub_dn9, locals.var_qsub_dn10, locals.var_qsub_dn11, locals.var_qsub_dn12,)
    }
};
        locals.var_qsub = assign27400_e25085;
        locals.var_qsub_dn3 = assign27400_e25085_d_n3;
        locals.var_qsub_dn4 = assign27400_e25085_d_n4;
        locals.var_qsub_dn5 = assign27400_e25085_d_n5;
        locals.var_qsub_dn6 = assign27400_e25085_d_n6;
        locals.var_qsub_dn7 = assign27400_e25085_d_n7;
        locals.var_qsub_dn8 = assign27400_e25085_d_n8;
        locals.var_qsub_dn9 = assign27400_e25085_d_n9;
        locals.var_qsub_dn10 = assign27400_e25085_d_n10;
        locals.var_qsub_dn11 = assign27400_e25085_d_n11;
        locals.var_qsub_dn12 = assign27400_e25085_d_n12;
        locals.var_qsub_rv = 0.0;

        let (assign27410_e25096, assign27410_e25096_d_n3, assign27410_e25096_d_n4, assign27410_e25096_d_n5, assign27410_e25096_d_n6, assign27410_e25096_d_n7, assign27410_e25096_d_n8, assign27410_e25096_d_n9, assign27410_e25096_d_n10, assign27410_e25096_d_n11, assign27410_e25096_d_n12,) = {
    if (locals.var_guard1367 != 0.0) {
        let assign27410_e25089: f64 = (locals.var_qgate + locals.var_qsrc);
        let assign27410_e25091: f64 = (assign27410_e25089 + locals.var_qbody);
        let assign27410_e25093: f64 = (assign27410_e25091 + locals.var_qsub);
        let assign27410_e25094: f64 = (-assign27410_e25093);
        (assign27410_e25094, (-(((locals.var_qgate_dn3 + locals.var_qsrc_dn3) + locals.var_qbody_dn3) + locals.var_qsub_dn3)), (-(((locals.var_qgate_dn4 + locals.var_qsrc_dn4) + locals.var_qbody_dn4) + locals.var_qsub_dn4)), (-(((locals.var_qgate_dn5 + locals.var_qsrc_dn5) + locals.var_qbody_dn5) + locals.var_qsub_dn5)), (-(((locals.var_qgate_dn6 + locals.var_qsrc_dn6) + locals.var_qbody_dn6) + locals.var_qsub_dn6)), (-(((locals.var_qgate_dn7 + locals.var_qsrc_dn7) + locals.var_qbody_dn7) + locals.var_qsub_dn7)), (-(((locals.var_qgate_dn8 + locals.var_qsrc_dn8) + locals.var_qbody_dn8) + locals.var_qsub_dn8)), (-(((locals.var_qgate_dn9 + locals.var_qsrc_dn9) + locals.var_qbody_dn9) + locals.var_qsub_dn9)), (-(((locals.var_qgate_dn10 + locals.var_qsrc_dn10) + locals.var_qbody_dn10) + locals.var_qsub_dn10)), (-(((locals.var_qgate_dn11 + locals.var_qsrc_dn11) + locals.var_qbody_dn11) + locals.var_qsub_dn11)), (-(((locals.var_qgate_dn12 + locals.var_qsrc_dn12) + locals.var_qbody_dn12) + locals.var_qsub_dn12)),)
    } else {
        (locals.var_qdrn, locals.var_qdrn_dn3, locals.var_qdrn_dn4, locals.var_qdrn_dn5, locals.var_qdrn_dn6, locals.var_qdrn_dn7, locals.var_qdrn_dn8, locals.var_qdrn_dn9, locals.var_qdrn_dn10, locals.var_qdrn_dn11, locals.var_qdrn_dn12,)
    }
};
        locals.var_qdrn = assign27410_e25096;
        locals.var_qdrn_dn3 = assign27410_e25096_d_n3;
        locals.var_qdrn_dn4 = assign27410_e25096_d_n4;
        locals.var_qdrn_dn5 = assign27410_e25096_d_n5;
        locals.var_qdrn_dn6 = assign27410_e25096_d_n6;
        locals.var_qdrn_dn7 = assign27410_e25096_d_n7;
        locals.var_qdrn_dn8 = assign27410_e25096_d_n8;
        locals.var_qdrn_dn9 = assign27410_e25096_d_n9;
        locals.var_qdrn_dn10 = assign27410_e25096_d_n10;
        locals.var_qdrn_dn11 = assign27410_e25096_d_n11;
        locals.var_qdrn_dn12 = assign27410_e25096_d_n12;
        locals.var_qdrn_rv = 0.0;

        let assign27420_e25099: f64 = if p.p61 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1385 = assign27420_e25099;
        locals.var_guard1385_rv = 0.0;

        let assign27430_e25102: f64 = if p.p41 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1386 = assign27430_e25102;
        locals.var_guard1386_rv = 0.0;

        let (assign27440_e25113, assign27440_e25113_d_n3, assign27440_e25113_d_n4, assign27440_e25113_d_n5, assign27440_e25113_d_n6, assign27440_e25113_d_n7, assign27440_e25113_d_n8, assign27440_e25113_d_n9, assign27440_e25113_d_n10, assign27440_e25113_d_n11, assign27440_e25113_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1386 != 0.0)) {
        let assign27440_e25111: f64 = (3.453133e-11 / locals.var_b4soitoxp);
        (assign27440_e25111, (-((3.453133e-11 * locals.var_b4soitoxp_dn3) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((3.453133e-11 * locals.var_b4soitoxp_dn4) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((3.453133e-11 * locals.var_b4soitoxp_dn5) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((3.453133e-11 * locals.var_b4soitoxp_dn6) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((3.453133e-11 * locals.var_b4soitoxp_dn7) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((3.453133e-11 * locals.var_b4soitoxp_dn8) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((3.453133e-11 * locals.var_b4soitoxp_dn9) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((3.453133e-11 * locals.var_b4soitoxp_dn10) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((3.453133e-11 * locals.var_b4soitoxp_dn11) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((3.453133e-11 * locals.var_b4soitoxp_dn12) / (locals.var_b4soitoxp * locals.var_b4soitoxp))),)
    } else {
        (locals.var_cox, locals.var_cox_dn3, locals.var_cox_dn4, locals.var_cox_dn5, locals.var_cox_dn6, locals.var_cox_dn7, locals.var_cox_dn8, locals.var_cox_dn9, locals.var_cox_dn10, locals.var_cox_dn11, locals.var_cox_dn12,)
    }
};
        locals.var_cox = assign27440_e25113;
        locals.var_cox_dn3 = assign27440_e25113_d_n3;
        locals.var_cox_dn4 = assign27440_e25113_d_n4;
        locals.var_cox_dn5 = assign27440_e25113_d_n5;
        locals.var_cox_dn6 = assign27440_e25113_d_n6;
        locals.var_cox_dn7 = assign27440_e25113_d_n7;
        locals.var_cox_dn8 = assign27440_e25113_d_n8;
        locals.var_cox_dn9 = assign27440_e25113_d_n9;
        locals.var_cox_dn10 = assign27440_e25113_d_n10;
        locals.var_cox_dn11 = assign27440_e25113_d_n11;
        locals.var_cox_dn12 = assign27440_e25113_d_n12;
        locals.var_cox_rv = 0.0;

        let (assign27450_e25127, assign27450_e25127_d_n3, assign27450_e25127_d_n4, assign27450_e25127_d_n5, assign27450_e25127_d_n6, assign27450_e25127_d_n7, assign27450_e25127_d_n8, assign27450_e25127_d_n9, assign27450_e25127_d_n10, assign27450_e25127_d_n11, assign27450_e25127_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1386 == 0.0)) {
        let assign27450_e25123: f64 = (locals.var_epsrox * 8.85418e-12);
        let assign27450_e25125: f64 = (assign27450_e25123 / locals.var_b4soitoxp);
        (assign27450_e25125, (-((assign27450_e25123 * locals.var_b4soitoxp_dn3) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((assign27450_e25123 * locals.var_b4soitoxp_dn4) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((assign27450_e25123 * locals.var_b4soitoxp_dn5) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((assign27450_e25123 * locals.var_b4soitoxp_dn6) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((assign27450_e25123 * locals.var_b4soitoxp_dn7) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((assign27450_e25123 * locals.var_b4soitoxp_dn8) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((assign27450_e25123 * locals.var_b4soitoxp_dn9) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((assign27450_e25123 * locals.var_b4soitoxp_dn10) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((assign27450_e25123 * locals.var_b4soitoxp_dn11) / (locals.var_b4soitoxp * locals.var_b4soitoxp))), (-((assign27450_e25123 * locals.var_b4soitoxp_dn12) / (locals.var_b4soitoxp * locals.var_b4soitoxp))),)
    } else {
        (locals.var_cox, locals.var_cox_dn3, locals.var_cox_dn4, locals.var_cox_dn5, locals.var_cox_dn6, locals.var_cox_dn7, locals.var_cox_dn8, locals.var_cox_dn9, locals.var_cox_dn10, locals.var_cox_dn11, locals.var_cox_dn12,)
    }
};
        locals.var_cox = assign27450_e25127;
        locals.var_cox_dn3 = assign27450_e25127_d_n3;
        locals.var_cox_dn4 = assign27450_e25127_d_n4;
        locals.var_cox_dn5 = assign27450_e25127_d_n5;
        locals.var_cox_dn6 = assign27450_e25127_d_n6;
        locals.var_cox_dn7 = assign27450_e25127_d_n7;
        locals.var_cox_dn8 = assign27450_e25127_d_n8;
        locals.var_cox_dn9 = assign27450_e25127_d_n9;
        locals.var_cox_dn10 = assign27450_e25127_d_n10;
        locals.var_cox_dn11 = assign27450_e25127_d_n11;
        locals.var_cox_dn12 = assign27450_e25127_d_n12;
        locals.var_cox_rv = 0.0;

        let (assign27460_e25138, assign27460_e25138_d_n3, assign27460_e25138_d_n4, assign27460_e25138_d_n5, assign27460_e25138_d_n6, assign27460_e25138_d_n7, assign27460_e25138_d_n8, assign27460_e25138_d_n9, assign27460_e25138_d_n10, assign27460_e25138_d_n11, assign27460_e25138_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) {
        let assign27460_e25134: f64 = (locals.var_coxwl * locals.var_toxe);
        let assign27460_e25136: f64 = (assign27460_e25134 / locals.var_b4soitoxp);
        (assign27460_e25136, ((((locals.var_coxwl_dn3 * locals.var_toxe) * locals.var_b4soitoxp) - (assign27460_e25134 * locals.var_b4soitoxp_dn3)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl_dn4 * locals.var_toxe) * locals.var_b4soitoxp) - (assign27460_e25134 * locals.var_b4soitoxp_dn4)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl_dn5 * locals.var_toxe) * locals.var_b4soitoxp) - (assign27460_e25134 * locals.var_b4soitoxp_dn5)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl_dn6 * locals.var_toxe) * locals.var_b4soitoxp) - (assign27460_e25134 * locals.var_b4soitoxp_dn6)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl_dn7 * locals.var_toxe) * locals.var_b4soitoxp) - (assign27460_e25134 * locals.var_b4soitoxp_dn7)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl_dn8 * locals.var_toxe) * locals.var_b4soitoxp) - (assign27460_e25134 * locals.var_b4soitoxp_dn8)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl_dn9 * locals.var_toxe) * locals.var_b4soitoxp) - (assign27460_e25134 * locals.var_b4soitoxp_dn9)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl_dn10 * locals.var_toxe) * locals.var_b4soitoxp) - (assign27460_e25134 * locals.var_b4soitoxp_dn10)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl_dn11 * locals.var_toxe) * locals.var_b4soitoxp) - (assign27460_e25134 * locals.var_b4soitoxp_dn11)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl_dn12 * locals.var_toxe) * locals.var_b4soitoxp) - (assign27460_e25134 * locals.var_b4soitoxp_dn12)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)),)
    } else {
        (locals.var_coxwl, locals.var_coxwl_dn3, locals.var_coxwl_dn4, locals.var_coxwl_dn5, locals.var_coxwl_dn6, locals.var_coxwl_dn7, locals.var_coxwl_dn8, locals.var_coxwl_dn9, locals.var_coxwl_dn10, locals.var_coxwl_dn11, locals.var_coxwl_dn12,)
    }
};
        locals.var_coxwl = assign27460_e25138;
        locals.var_coxwl_dn3 = assign27460_e25138_d_n3;
        locals.var_coxwl_dn4 = assign27460_e25138_d_n4;
        locals.var_coxwl_dn5 = assign27460_e25138_d_n5;
        locals.var_coxwl_dn6 = assign27460_e25138_d_n6;
        locals.var_coxwl_dn7 = assign27460_e25138_d_n7;
        locals.var_coxwl_dn8 = assign27460_e25138_d_n8;
        locals.var_coxwl_dn9 = assign27460_e25138_d_n9;
        locals.var_coxwl_dn10 = assign27460_e25138_d_n10;
        locals.var_coxwl_dn11 = assign27460_e25138_d_n11;
        locals.var_coxwl_dn12 = assign27460_e25138_d_n12;
        locals.var_coxwl_rv = 0.0;

        let (assign27470_e25149, assign27470_e25149_d_n3, assign27470_e25149_d_n4, assign27470_e25149_d_n5, assign27470_e25149_d_n6, assign27470_e25149_d_n7, assign27470_e25149_d_n8, assign27470_e25149_d_n9, assign27470_e25149_d_n10, assign27470_e25149_d_n11, assign27470_e25149_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) {
        let assign27470_e25145: f64 = (locals.var_coxwlb * p.p66);
        let assign27470_e25147: f64 = (assign27470_e25145 / locals.var_b4soitoxp);
        (assign27470_e25147, ((((locals.var_coxwlb_dn3 * p.p66) * locals.var_b4soitoxp) - (assign27470_e25145 * locals.var_b4soitoxp_dn3)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb_dn4 * p.p66) * locals.var_b4soitoxp) - (assign27470_e25145 * locals.var_b4soitoxp_dn4)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb_dn5 * p.p66) * locals.var_b4soitoxp) - (assign27470_e25145 * locals.var_b4soitoxp_dn5)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb_dn6 * p.p66) * locals.var_b4soitoxp) - (assign27470_e25145 * locals.var_b4soitoxp_dn6)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb_dn7 * p.p66) * locals.var_b4soitoxp) - (assign27470_e25145 * locals.var_b4soitoxp_dn7)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb_dn8 * p.p66) * locals.var_b4soitoxp) - (assign27470_e25145 * locals.var_b4soitoxp_dn8)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb_dn9 * p.p66) * locals.var_b4soitoxp) - (assign27470_e25145 * locals.var_b4soitoxp_dn9)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb_dn10 * p.p66) * locals.var_b4soitoxp) - (assign27470_e25145 * locals.var_b4soitoxp_dn10)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb_dn11 * p.p66) * locals.var_b4soitoxp) - (assign27470_e25145 * locals.var_b4soitoxp_dn11)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb_dn12 * p.p66) * locals.var_b4soitoxp) - (assign27470_e25145 * locals.var_b4soitoxp_dn12)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)),)
    } else {
        (locals.var_coxwlb, locals.var_coxwlb_dn3, locals.var_coxwlb_dn4, locals.var_coxwlb_dn5, locals.var_coxwlb_dn6, locals.var_coxwlb_dn7, locals.var_coxwlb_dn8, locals.var_coxwlb_dn9, locals.var_coxwlb_dn10, locals.var_coxwlb_dn11, locals.var_coxwlb_dn12,)
    }
};
        locals.var_coxwlb = assign27470_e25149;
        locals.var_coxwlb_dn3 = assign27470_e25149_d_n3;
        locals.var_coxwlb_dn4 = assign27470_e25149_d_n4;
        locals.var_coxwlb_dn5 = assign27470_e25149_d_n5;
        locals.var_coxwlb_dn6 = assign27470_e25149_d_n6;
        locals.var_coxwlb_dn7 = assign27470_e25149_d_n7;
        locals.var_coxwlb_dn8 = assign27470_e25149_d_n8;
        locals.var_coxwlb_dn9 = assign27470_e25149_d_n9;
        locals.var_coxwlb_dn10 = assign27470_e25149_d_n10;
        locals.var_coxwlb_dn11 = assign27470_e25149_d_n11;
        locals.var_coxwlb_dn12 = assign27470_e25149_d_n12;
        locals.var_coxwlb_rv = 0.0;

        let (assign27480_e25158, assign27480_e25158_d_n3, assign27480_e25158_d_n4, assign27480_e25158_d_n5, assign27480_e25158_d_n6, assign27480_e25158_d_n7, assign27480_e25158_d_n8, assign27480_e25158_d_n9, assign27480_e25158_d_n10, assign27480_e25158_d_n11, assign27480_e25158_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) {
        let assign27480_e25156: f64 = (100000000.0 * locals.var_b4soitoxp);
        (assign27480_e25156, (100000000.0 * locals.var_b4soitoxp_dn3), (100000000.0 * locals.var_b4soitoxp_dn4), (100000000.0 * locals.var_b4soitoxp_dn5), (100000000.0 * locals.var_b4soitoxp_dn6), (100000000.0 * locals.var_b4soitoxp_dn7), (100000000.0 * locals.var_b4soitoxp_dn8), (100000000.0 * locals.var_b4soitoxp_dn9), (100000000.0 * locals.var_b4soitoxp_dn10), (100000000.0 * locals.var_b4soitoxp_dn11), (100000000.0 * locals.var_b4soitoxp_dn12),)
    } else {
        (locals.var_tox, locals.var_tox_dn3, locals.var_tox_dn4, locals.var_tox_dn5, locals.var_tox_dn6, locals.var_tox_dn7, locals.var_tox_dn8, locals.var_tox_dn9, locals.var_tox_dn10, locals.var_tox_dn11, locals.var_tox_dn12,)
    }
};
        locals.var_tox = assign27480_e25158;
        locals.var_tox_dn3 = assign27480_e25158_d_n3;
        locals.var_tox_dn4 = assign27480_e25158_d_n4;
        locals.var_tox_dn5 = assign27480_e25158_d_n5;
        locals.var_tox_dn6 = assign27480_e25158_d_n6;
        locals.var_tox_dn7 = assign27480_e25158_d_n7;
        locals.var_tox_dn8 = assign27480_e25158_d_n8;
        locals.var_tox_dn9 = assign27480_e25158_d_n9;
        locals.var_tox_dn10 = assign27480_e25158_d_n10;
        locals.var_tox_dn11 = assign27480_e25158_d_n11;
        locals.var_tox_dn12 = assign27480_e25158_d_n12;
        locals.var_tox_rv = 0.0;

        let assign27490_e25161: f64 = if p.p27 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1387 = assign27490_e25161;
        locals.var_guard1387_rv = 0.0;

        let (assign27500_e25174, assign27500_e25174_d_n3, assign27500_e25174_d_n4, assign27500_e25174_d_n5, assign27500_e25174_d_n6, assign27500_e25174_d_n7, assign27500_e25174_d_n8, assign27500_e25174_d_n9, assign27500_e25174_d_n10, assign27500_e25174_d_n11, assign27500_e25174_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1387 != 0.0)) {
        let assign27500_e25170: f64 = (locals.var_coxwl2 * p.p66);
        let assign27500_e25172: f64 = (assign27500_e25170 / locals.var_b4soitoxp);
        (assign27500_e25172, ((((locals.var_coxwl2_dn3 * p.p66) * locals.var_b4soitoxp) - (assign27500_e25170 * locals.var_b4soitoxp_dn3)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl2_dn4 * p.p66) * locals.var_b4soitoxp) - (assign27500_e25170 * locals.var_b4soitoxp_dn4)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl2_dn5 * p.p66) * locals.var_b4soitoxp) - (assign27500_e25170 * locals.var_b4soitoxp_dn5)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl2_dn6 * p.p66) * locals.var_b4soitoxp) - (assign27500_e25170 * locals.var_b4soitoxp_dn6)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl2_dn7 * p.p66) * locals.var_b4soitoxp) - (assign27500_e25170 * locals.var_b4soitoxp_dn7)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl2_dn8 * p.p66) * locals.var_b4soitoxp) - (assign27500_e25170 * locals.var_b4soitoxp_dn8)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl2_dn9 * p.p66) * locals.var_b4soitoxp) - (assign27500_e25170 * locals.var_b4soitoxp_dn9)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl2_dn10 * p.p66) * locals.var_b4soitoxp) - (assign27500_e25170 * locals.var_b4soitoxp_dn10)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl2_dn11 * p.p66) * locals.var_b4soitoxp) - (assign27500_e25170 * locals.var_b4soitoxp_dn11)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwl2_dn12 * p.p66) * locals.var_b4soitoxp) - (assign27500_e25170 * locals.var_b4soitoxp_dn12)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)),)
    } else {
        (locals.var_coxwl2, locals.var_coxwl2_dn3, locals.var_coxwl2_dn4, locals.var_coxwl2_dn5, locals.var_coxwl2_dn6, locals.var_coxwl2_dn7, locals.var_coxwl2_dn8, locals.var_coxwl2_dn9, locals.var_coxwl2_dn10, locals.var_coxwl2_dn11, locals.var_coxwl2_dn12,)
    }
};
        locals.var_coxwl2 = assign27500_e25174;
        locals.var_coxwl2_dn3 = assign27500_e25174_d_n3;
        locals.var_coxwl2_dn4 = assign27500_e25174_d_n4;
        locals.var_coxwl2_dn5 = assign27500_e25174_d_n5;
        locals.var_coxwl2_dn6 = assign27500_e25174_d_n6;
        locals.var_coxwl2_dn7 = assign27500_e25174_d_n7;
        locals.var_coxwl2_dn8 = assign27500_e25174_d_n8;
        locals.var_coxwl2_dn9 = assign27500_e25174_d_n9;
        locals.var_coxwl2_dn10 = assign27500_e25174_d_n10;
        locals.var_coxwl2_dn11 = assign27500_e25174_d_n11;
        locals.var_coxwl2_dn12 = assign27500_e25174_d_n12;
        locals.var_coxwl2_rv = 0.0;

        let (assign27510_e25187, assign27510_e25187_d_n3, assign27510_e25187_d_n4, assign27510_e25187_d_n5, assign27510_e25187_d_n6, assign27510_e25187_d_n7, assign27510_e25187_d_n8, assign27510_e25187_d_n9, assign27510_e25187_d_n10, assign27510_e25187_d_n11, assign27510_e25187_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1387 != 0.0)) {
        let assign27510_e25183: f64 = (locals.var_coxwlb2 * p.p66);
        let assign27510_e25185: f64 = (assign27510_e25183 / locals.var_b4soitoxp);
        (assign27510_e25185, ((((locals.var_coxwlb2_dn3 * p.p66) * locals.var_b4soitoxp) - (assign27510_e25183 * locals.var_b4soitoxp_dn3)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb2_dn4 * p.p66) * locals.var_b4soitoxp) - (assign27510_e25183 * locals.var_b4soitoxp_dn4)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb2_dn5 * p.p66) * locals.var_b4soitoxp) - (assign27510_e25183 * locals.var_b4soitoxp_dn5)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb2_dn6 * p.p66) * locals.var_b4soitoxp) - (assign27510_e25183 * locals.var_b4soitoxp_dn6)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb2_dn7 * p.p66) * locals.var_b4soitoxp) - (assign27510_e25183 * locals.var_b4soitoxp_dn7)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb2_dn8 * p.p66) * locals.var_b4soitoxp) - (assign27510_e25183 * locals.var_b4soitoxp_dn8)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb2_dn9 * p.p66) * locals.var_b4soitoxp) - (assign27510_e25183 * locals.var_b4soitoxp_dn9)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb2_dn10 * p.p66) * locals.var_b4soitoxp) - (assign27510_e25183 * locals.var_b4soitoxp_dn10)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb2_dn11 * p.p66) * locals.var_b4soitoxp) - (assign27510_e25183 * locals.var_b4soitoxp_dn11)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)), ((((locals.var_coxwlb2_dn12 * p.p66) * locals.var_b4soitoxp) - (assign27510_e25183 * locals.var_b4soitoxp_dn12)) / (locals.var_b4soitoxp * locals.var_b4soitoxp)),)
    } else {
        (locals.var_coxwlb2, locals.var_coxwlb2_dn3, locals.var_coxwlb2_dn4, locals.var_coxwlb2_dn5, locals.var_coxwlb2_dn6, locals.var_coxwlb2_dn7, locals.var_coxwlb2_dn8, locals.var_coxwlb2_dn9, locals.var_coxwlb2_dn10, locals.var_coxwlb2_dn11, locals.var_coxwlb2_dn12,)
    }
};
        locals.var_coxwlb2 = assign27510_e25187;
        locals.var_coxwlb2_dn3 = assign27510_e25187_d_n3;
        locals.var_coxwlb2_dn4 = assign27510_e25187_d_n4;
        locals.var_coxwlb2_dn5 = assign27510_e25187_d_n5;
        locals.var_coxwlb2_dn6 = assign27510_e25187_d_n6;
        locals.var_coxwlb2_dn7 = assign27510_e25187_d_n7;
        locals.var_coxwlb2_dn8 = assign27510_e25187_d_n8;
        locals.var_coxwlb2_dn9 = assign27510_e25187_d_n9;
        locals.var_coxwlb2_dn10 = assign27510_e25187_d_n10;
        locals.var_coxwlb2_dn11 = assign27510_e25187_d_n11;
        locals.var_coxwlb2_dn12 = assign27510_e25187_d_n12;
        locals.var_coxwlb2_rv = 0.0;

        let assign27520_e25190: f64 = if locals.var_b4soisoimod == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1388 = assign27520_e25190;
        locals.var_guard1388_rv = 0.0;

        let (assign27530_e25199, assign27530_e25199_d_n3, assign27530_e25199_d_n4, assign27530_e25199_d_n5, assign27530_e25199_d_n6, assign27530_e25199_d_n7, assign27530_e25199_d_n8, assign27530_e25199_d_n9, assign27530_e25199_d_n10, assign27530_e25199_d_n11, assign27530_e25199_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qac0, locals.var_qac0_dn3, locals.var_qac0_dn4, locals.var_qac0_dn5, locals.var_qac0_dn6, locals.var_qac0_dn7, locals.var_qac0_dn8, locals.var_qac0_dn9, locals.var_qac0_dn10, locals.var_qac0_dn11, locals.var_qac0_dn12,)
    }
};
        locals.var_qac0 = assign27530_e25199;
        locals.var_qac0_dn3 = assign27530_e25199_d_n3;
        locals.var_qac0_dn4 = assign27530_e25199_d_n4;
        locals.var_qac0_dn5 = assign27530_e25199_d_n5;
        locals.var_qac0_dn6 = assign27530_e25199_d_n6;
        locals.var_qac0_dn7 = assign27530_e25199_d_n7;
        locals.var_qac0_dn8 = assign27530_e25199_d_n8;
        locals.var_qac0_dn9 = assign27530_e25199_d_n9;
        locals.var_qac0_dn10 = assign27530_e25199_d_n10;
        locals.var_qac0_dn11 = assign27530_e25199_d_n11;
        locals.var_qac0_dn12 = assign27530_e25199_d_n12;
        locals.var_qac0_rv = 0.0;

        let (assign27540_e25208, assign27540_e25208_d_n3, assign27540_e25208_d_n4, assign27540_e25208_d_n5, assign27540_e25208_d_n6, assign27540_e25208_d_n7, assign27540_e25208_d_n8, assign27540_e25208_d_n9, assign27540_e25208_d_n10, assign27540_e25208_d_n11, assign27540_e25208_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qsub0, locals.var_qsub0_dn3, locals.var_qsub0_dn4, locals.var_qsub0_dn5, locals.var_qsub0_dn6, locals.var_qsub0_dn7, locals.var_qsub0_dn8, locals.var_qsub0_dn9, locals.var_qsub0_dn10, locals.var_qsub0_dn11, locals.var_qsub0_dn12,)
    }
};
        locals.var_qsub0 = assign27540_e25208;
        locals.var_qsub0_dn3 = assign27540_e25208_d_n3;
        locals.var_qsub0_dn4 = assign27540_e25208_d_n4;
        locals.var_qsub0_dn5 = assign27540_e25208_d_n5;
        locals.var_qsub0_dn6 = assign27540_e25208_d_n6;
        locals.var_qsub0_dn7 = assign27540_e25208_d_n7;
        locals.var_qsub0_dn8 = assign27540_e25208_d_n8;
        locals.var_qsub0_dn9 = assign27540_e25208_d_n9;
        locals.var_qsub0_dn10 = assign27540_e25208_d_n10;
        locals.var_qsub0_dn11 = assign27540_e25208_d_n11;
        locals.var_qsub0_dn12 = assign27540_e25208_d_n12;
        locals.var_qsub0_rv = 0.0;

        let (assign27550_e25217, assign27550_e25217_d_n3, assign27550_e25217_d_n4, assign27550_e25217_d_n5, assign27550_e25217_d_n6, assign27550_e25217_d_n7, assign27550_e25217_d_n8, assign27550_e25217_d_n9, assign27550_e25217_d_n10, assign27550_e25217_d_n11, assign27550_e25217_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vfbzb, locals.var_vfbzb_dn3, locals.var_vfbzb_dn4, locals.var_vfbzb_dn5, locals.var_vfbzb_dn6, locals.var_vfbzb_dn7, locals.var_vfbzb_dn8, locals.var_vfbzb_dn9, locals.var_vfbzb_dn10, locals.var_vfbzb_dn11, locals.var_vfbzb_dn12,)
    }
};
        locals.var_vfbzb = assign27550_e25217;
        locals.var_vfbzb_dn3 = assign27550_e25217_d_n3;
        locals.var_vfbzb_dn4 = assign27550_e25217_d_n4;
        locals.var_vfbzb_dn5 = assign27550_e25217_d_n5;
        locals.var_vfbzb_dn6 = assign27550_e25217_d_n6;
        locals.var_vfbzb_dn7 = assign27550_e25217_d_n7;
        locals.var_vfbzb_dn8 = assign27550_e25217_d_n8;
        locals.var_vfbzb_dn9 = assign27550_e25217_d_n9;
        locals.var_vfbzb_dn10 = assign27550_e25217_d_n10;
        locals.var_vfbzb_dn11 = assign27550_e25217_d_n11;
        locals.var_vfbzb_dn12 = assign27550_e25217_d_n12;
        locals.var_vfbzb_rv = 0.0;

        let assign27560_e25224: f64 = if ((p.p36 == 1.0) && (p.p14 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1389 = assign27560_e25224;
        locals.var_guard1389_rv = 0.0;

        let (assign27570_e25244, assign27570_e25244_d_n3, assign27570_e25244_d_n4, assign27570_e25244_d_n5, assign27570_e25244_d_n6, assign27570_e25244_d_n7, assign27570_e25244_d_n8, assign27570_e25244_d_n9, assign27570_e25244_d_n10, assign27570_e25244_d_n11, assign27570_e25244_d_n12,) = {
    if ((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1389 != 0.0)) {
        let assign27570_e25236: f64 = (locals.var_vthzb - locals.var_phi);
        let assign27570_e25239: f64 = (locals.var_here_b4soik1eff * locals.var_sqrtphi);
        let assign27570_e25240: f64 = (assign27570_e25236 - assign27570_e25239);
        let assign27570_e25242: f64 = (assign27570_e25240 + locals.var_pparam_b4soidelvt);
        (assign27570_e25242, (((locals.var_vthzb_dn3 - locals.var_phi_dn3) - ((locals.var_here_b4soik1eff_dn3 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn3))) + locals.var_pparam_b4soidelvt_dn3), (((locals.var_vthzb_dn4 - locals.var_phi_dn4) - ((locals.var_here_b4soik1eff_dn4 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn4))) + locals.var_pparam_b4soidelvt_dn4), (((locals.var_vthzb_dn5 - locals.var_phi_dn5) - ((locals.var_here_b4soik1eff_dn5 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn5))) + locals.var_pparam_b4soidelvt_dn5), (((locals.var_vthzb_dn6 - locals.var_phi_dn6) - ((locals.var_here_b4soik1eff_dn6 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn6))) + locals.var_pparam_b4soidelvt_dn6), (((locals.var_vthzb_dn7 - locals.var_phi_dn7) - ((locals.var_here_b4soik1eff_dn7 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn7))) + locals.var_pparam_b4soidelvt_dn7), (((locals.var_vthzb_dn8 - locals.var_phi_dn8) - ((locals.var_here_b4soik1eff_dn8 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn8))) + locals.var_pparam_b4soidelvt_dn8), (((locals.var_vthzb_dn9 - locals.var_phi_dn9) - ((locals.var_here_b4soik1eff_dn9 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn9))) + locals.var_pparam_b4soidelvt_dn9), (((locals.var_vthzb_dn10 - locals.var_phi_dn10) - ((locals.var_here_b4soik1eff_dn10 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn10))) + locals.var_pparam_b4soidelvt_dn10), (((locals.var_vthzb_dn11 - locals.var_phi_dn11) - ((locals.var_here_b4soik1eff_dn11 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn11))) + locals.var_pparam_b4soidelvt_dn11), (((locals.var_vthzb_dn12 - locals.var_phi_dn12) - ((locals.var_here_b4soik1eff_dn12 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn12))) + locals.var_pparam_b4soidelvt_dn12),)
    } else {
        (locals.var_vfbzb, locals.var_vfbzb_dn3, locals.var_vfbzb_dn4, locals.var_vfbzb_dn5, locals.var_vfbzb_dn6, locals.var_vfbzb_dn7, locals.var_vfbzb_dn8, locals.var_vfbzb_dn9, locals.var_vfbzb_dn10, locals.var_vfbzb_dn11, locals.var_vfbzb_dn12,)
    }
};
        locals.var_vfbzb = assign27570_e25244;
        locals.var_vfbzb_dn3 = assign27570_e25244_d_n3;
        locals.var_vfbzb_dn4 = assign27570_e25244_d_n4;
        locals.var_vfbzb_dn5 = assign27570_e25244_d_n5;
        locals.var_vfbzb_dn6 = assign27570_e25244_d_n6;
        locals.var_vfbzb_dn7 = assign27570_e25244_d_n7;
        locals.var_vfbzb_dn8 = assign27570_e25244_d_n8;
        locals.var_vfbzb_dn9 = assign27570_e25244_d_n9;
        locals.var_vfbzb_dn10 = assign27570_e25244_d_n10;
        locals.var_vfbzb_dn11 = assign27570_e25244_d_n11;
        locals.var_vfbzb_dn12 = assign27570_e25244_d_n12;
        locals.var_vfbzb_rv = 0.0;

        let (assign27580_e25259, assign27580_e25259_d_n3, assign27580_e25259_d_n4, assign27580_e25259_d_n5, assign27580_e25259_d_n6, assign27580_e25259_d_n7, assign27580_e25259_d_n8, assign27580_e25259_d_n9, assign27580_e25259_d_n10, assign27580_e25259_d_n11, assign27580_e25259_d_n12,) = {
    if ((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1389 == 0.0)) {
        let assign27580_e25257: f64 = (locals.var_b4soivfbzb + locals.var_pparam_b4soidelvt);
        (assign27580_e25257, (locals.var_b4soivfbzb_dn3 + locals.var_pparam_b4soidelvt_dn3), (locals.var_b4soivfbzb_dn4 + locals.var_pparam_b4soidelvt_dn4), (locals.var_b4soivfbzb_dn5 + locals.var_pparam_b4soidelvt_dn5), (locals.var_b4soivfbzb_dn6 + locals.var_pparam_b4soidelvt_dn6), (locals.var_b4soivfbzb_dn7 + locals.var_pparam_b4soidelvt_dn7), (locals.var_b4soivfbzb_dn8 + locals.var_pparam_b4soidelvt_dn8), (locals.var_b4soivfbzb_dn9 + locals.var_pparam_b4soidelvt_dn9), (locals.var_b4soivfbzb_dn10 + locals.var_pparam_b4soidelvt_dn10), (locals.var_b4soivfbzb_dn11 + locals.var_pparam_b4soidelvt_dn11), (locals.var_b4soivfbzb_dn12 + locals.var_pparam_b4soidelvt_dn12),)
    } else {
        (locals.var_vfbzb, locals.var_vfbzb_dn3, locals.var_vfbzb_dn4, locals.var_vfbzb_dn5, locals.var_vfbzb_dn6, locals.var_vfbzb_dn7, locals.var_vfbzb_dn8, locals.var_vfbzb_dn9, locals.var_vfbzb_dn10, locals.var_vfbzb_dn11, locals.var_vfbzb_dn12,)
    }
};
        locals.var_vfbzb = assign27580_e25259;
        locals.var_vfbzb_dn3 = assign27580_e25259_d_n3;
        locals.var_vfbzb_dn4 = assign27580_e25259_d_n4;
        locals.var_vfbzb_dn5 = assign27580_e25259_d_n5;
        locals.var_vfbzb_dn6 = assign27580_e25259_d_n6;
        locals.var_vfbzb_dn7 = assign27580_e25259_d_n7;
        locals.var_vfbzb_dn8 = assign27580_e25259_d_n8;
        locals.var_vfbzb_dn9 = assign27580_e25259_d_n9;
        locals.var_vfbzb_dn10 = assign27580_e25259_d_n10;
        locals.var_vfbzb_dn11 = assign27580_e25259_d_n11;
        locals.var_vfbzb_dn12 = assign27580_e25259_d_n12;
        locals.var_vfbzb_rv = 0.0;

        let (assign27590_e25275, assign27590_e25275_d_n3, assign27590_e25275_d_n4, assign27590_e25275_d_n5, assign27590_e25275_d_n6, assign27590_e25275_d_n7, assign27590_e25275_d_n8, assign27590_e25275_d_n9, assign27590_e25275_d_n10, assign27590_e25275_d_n11, assign27590_e25275_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) {
        let assign27590_e25269: f64 = (locals.var_vfbzb - locals.var_vgs_eff__blk790);
        let assign27590_e25271: f64 = (assign27590_e25269 + locals.var_vbseff);
        let assign27590_e25273: f64 = (assign27590_e25271 - 0.02);
        (assign27590_e25273, ((locals.var_vfbzb_dn3 - locals.var_vgs_eff__blk790_dn3) + locals.var_vbseff_dn3), ((locals.var_vfbzb_dn4 - locals.var_vgs_eff__blk790_dn4) + locals.var_vbseff_dn4), ((locals.var_vfbzb_dn5 - locals.var_vgs_eff__blk790_dn5) + locals.var_vbseff_dn5), ((locals.var_vfbzb_dn6 - locals.var_vgs_eff__blk790_dn6) + locals.var_vbseff_dn6), ((locals.var_vfbzb_dn7 - locals.var_vgs_eff__blk790_dn7) + locals.var_vbseff_dn7), ((locals.var_vfbzb_dn8 - locals.var_vgs_eff__blk790_dn8) + locals.var_vbseff_dn8), ((locals.var_vfbzb_dn9 - locals.var_vgs_eff__blk790_dn9) + locals.var_vbseff_dn9), ((locals.var_vfbzb_dn10 - locals.var_vgs_eff__blk790_dn10) + locals.var_vbseff_dn10), ((locals.var_vfbzb_dn11 - locals.var_vgs_eff__blk790_dn11) + locals.var_vbseff_dn11), ((locals.var_vfbzb_dn12 - locals.var_vgs_eff__blk790_dn12) + locals.var_vbseff_dn12),)
    } else {
        (locals.var_v3, locals.var_v3_dn3, locals.var_v3_dn4, locals.var_v3_dn5, locals.var_v3_dn6, locals.var_v3_dn7, locals.var_v3_dn8, locals.var_v3_dn9, locals.var_v3_dn10, locals.var_v3_dn11, locals.var_v3_dn12,)
    }
};
        locals.var_v3 = assign27590_e25275;
        locals.var_v3_dn3 = assign27590_e25275_d_n3;
        locals.var_v3_dn4 = assign27590_e25275_d_n4;
        locals.var_v3_dn5 = assign27590_e25275_d_n5;
        locals.var_v3_dn6 = assign27590_e25275_d_n6;
        locals.var_v3_dn7 = assign27590_e25275_d_n7;
        locals.var_v3_dn8 = assign27590_e25275_d_n8;
        locals.var_v3_dn9 = assign27590_e25275_d_n9;
        locals.var_v3_dn10 = assign27590_e25275_d_n10;
        locals.var_v3_dn11 = assign27590_e25275_d_n11;
        locals.var_v3_dn12 = assign27590_e25275_d_n12;
        locals.var_v3_rv = 0.0;

        let assign27600_e25278: f64 = if locals.var_vfbzb <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1390 = assign27600_e25278;
        locals.var_guard1390_rv = 0.0;

        let (assign27610_e25299, assign27610_e25299_d_n3, assign27610_e25299_d_n4, assign27610_e25299_d_n5, assign27610_e25299_d_n6, assign27610_e25299_d_n7, assign27610_e25299_d_n8, assign27610_e25299_d_n9, assign27610_e25299_d_n10, assign27610_e25299_d_n11, assign27610_e25299_d_n12,) = {
    if ((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1390 != 0.0)) {
        let assign27610_e25290: f64 = (locals.var_v3 * locals.var_v3);
        let assign27610_e25293: f64 = (4.0 * 0.02);
        let assign27610_e25295: f64 = (assign27610_e25293 * locals.var_vfbzb);
        let assign27610_e25296: f64 = (assign27610_e25290 - assign27610_e25295);
        let assign27610_e25297: f64 = (assign27610_e25296).sqrt();
        (assign27610_e25297, ((((locals.var_v3_dn3 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn3)) - (assign27610_e25293 * locals.var_vfbzb_dn3)) / (2.0 * assign27610_e25297)), ((((locals.var_v3_dn4 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn4)) - (assign27610_e25293 * locals.var_vfbzb_dn4)) / (2.0 * assign27610_e25297)), ((((locals.var_v3_dn5 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn5)) - (assign27610_e25293 * locals.var_vfbzb_dn5)) / (2.0 * assign27610_e25297)), ((((locals.var_v3_dn6 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn6)) - (assign27610_e25293 * locals.var_vfbzb_dn6)) / (2.0 * assign27610_e25297)), ((((locals.var_v3_dn7 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn7)) - (assign27610_e25293 * locals.var_vfbzb_dn7)) / (2.0 * assign27610_e25297)), ((((locals.var_v3_dn8 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn8)) - (assign27610_e25293 * locals.var_vfbzb_dn8)) / (2.0 * assign27610_e25297)), ((((locals.var_v3_dn9 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn9)) - (assign27610_e25293 * locals.var_vfbzb_dn9)) / (2.0 * assign27610_e25297)), ((((locals.var_v3_dn10 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn10)) - (assign27610_e25293 * locals.var_vfbzb_dn10)) / (2.0 * assign27610_e25297)), ((((locals.var_v3_dn11 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn11)) - (assign27610_e25293 * locals.var_vfbzb_dn11)) / (2.0 * assign27610_e25297)), ((((locals.var_v3_dn12 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn12)) - (assign27610_e25293 * locals.var_vfbzb_dn12)) / (2.0 * assign27610_e25297)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign27610_e25299;
        locals.var_t0__blk808_dn3 = assign27610_e25299_d_n3;
        locals.var_t0__blk808_dn4 = assign27610_e25299_d_n4;
        locals.var_t0__blk808_dn5 = assign27610_e25299_d_n5;
        locals.var_t0__blk808_dn6 = assign27610_e25299_d_n6;
        locals.var_t0__blk808_dn7 = assign27610_e25299_d_n7;
        locals.var_t0__blk808_dn8 = assign27610_e25299_d_n8;
        locals.var_t0__blk808_dn9 = assign27610_e25299_d_n9;
        locals.var_t0__blk808_dn10 = assign27610_e25299_d_n10;
        locals.var_t0__blk808_dn11 = assign27610_e25299_d_n11;
        locals.var_t0__blk808_dn12 = assign27610_e25299_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign27620_e25321, assign27620_e25321_d_n3, assign27620_e25321_d_n4, assign27620_e25321_d_n5, assign27620_e25321_d_n6, assign27620_e25321_d_n7, assign27620_e25321_d_n8, assign27620_e25321_d_n9, assign27620_e25321_d_n10, assign27620_e25321_d_n11, assign27620_e25321_d_n12,) = {
    if ((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1390 == 0.0)) {
        let assign27620_e25312: f64 = (locals.var_v3 * locals.var_v3);
        let assign27620_e25315: f64 = (4.0 * 0.02);
        let assign27620_e25317: f64 = (assign27620_e25315 * locals.var_vfbzb);
        let assign27620_e25318: f64 = (assign27620_e25312 + assign27620_e25317);
        let assign27620_e25319: f64 = (assign27620_e25318).sqrt();
        (assign27620_e25319, ((((locals.var_v3_dn3 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn3)) + (assign27620_e25315 * locals.var_vfbzb_dn3)) / (2.0 * assign27620_e25319)), ((((locals.var_v3_dn4 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn4)) + (assign27620_e25315 * locals.var_vfbzb_dn4)) / (2.0 * assign27620_e25319)), ((((locals.var_v3_dn5 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn5)) + (assign27620_e25315 * locals.var_vfbzb_dn5)) / (2.0 * assign27620_e25319)), ((((locals.var_v3_dn6 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn6)) + (assign27620_e25315 * locals.var_vfbzb_dn6)) / (2.0 * assign27620_e25319)), ((((locals.var_v3_dn7 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn7)) + (assign27620_e25315 * locals.var_vfbzb_dn7)) / (2.0 * assign27620_e25319)), ((((locals.var_v3_dn8 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn8)) + (assign27620_e25315 * locals.var_vfbzb_dn8)) / (2.0 * assign27620_e25319)), ((((locals.var_v3_dn9 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn9)) + (assign27620_e25315 * locals.var_vfbzb_dn9)) / (2.0 * assign27620_e25319)), ((((locals.var_v3_dn10 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn10)) + (assign27620_e25315 * locals.var_vfbzb_dn10)) / (2.0 * assign27620_e25319)), ((((locals.var_v3_dn11 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn11)) + (assign27620_e25315 * locals.var_vfbzb_dn11)) / (2.0 * assign27620_e25319)), ((((locals.var_v3_dn12 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn12)) + (assign27620_e25315 * locals.var_vfbzb_dn12)) / (2.0 * assign27620_e25319)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign27620_e25321;
        locals.var_t0__blk808_dn3 = assign27620_e25321_d_n3;
        locals.var_t0__blk808_dn4 = assign27620_e25321_d_n4;
        locals.var_t0__blk808_dn5 = assign27620_e25321_d_n5;
        locals.var_t0__blk808_dn6 = assign27620_e25321_d_n6;
        locals.var_t0__blk808_dn7 = assign27620_e25321_d_n7;
        locals.var_t0__blk808_dn8 = assign27620_e25321_d_n8;
        locals.var_t0__blk808_dn9 = assign27620_e25321_d_n9;
        locals.var_t0__blk808_dn10 = assign27620_e25321_d_n10;
        locals.var_t0__blk808_dn11 = assign27620_e25321_d_n11;
        locals.var_t0__blk808_dn12 = assign27620_e25321_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign27630_e25337, assign27630_e25337_d_n3, assign27630_e25337_d_n4, assign27630_e25337_d_n5, assign27630_e25337_d_n6, assign27630_e25337_d_n7, assign27630_e25337_d_n8, assign27630_e25337_d_n9, assign27630_e25337_d_n10, assign27630_e25337_d_n11, assign27630_e25337_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) {
        let assign27630_e25333: f64 = (locals.var_v3 + locals.var_t0__blk808);
        let assign27630_e25334: f64 = (0.5 * assign27630_e25333);
        let assign27630_e25335: f64 = (locals.var_vfbzb - assign27630_e25334);
        (assign27630_e25335, (locals.var_vfbzb_dn3 - (0.5 * (locals.var_v3_dn3 + locals.var_t0__blk808_dn3))), (locals.var_vfbzb_dn4 - (0.5 * (locals.var_v3_dn4 + locals.var_t0__blk808_dn4))), (locals.var_vfbzb_dn5 - (0.5 * (locals.var_v3_dn5 + locals.var_t0__blk808_dn5))), (locals.var_vfbzb_dn6 - (0.5 * (locals.var_v3_dn6 + locals.var_t0__blk808_dn6))), (locals.var_vfbzb_dn7 - (0.5 * (locals.var_v3_dn7 + locals.var_t0__blk808_dn7))), (locals.var_vfbzb_dn8 - (0.5 * (locals.var_v3_dn8 + locals.var_t0__blk808_dn8))), (locals.var_vfbzb_dn9 - (0.5 * (locals.var_v3_dn9 + locals.var_t0__blk808_dn9))), (locals.var_vfbzb_dn10 - (0.5 * (locals.var_v3_dn10 + locals.var_t0__blk808_dn10))), (locals.var_vfbzb_dn11 - (0.5 * (locals.var_v3_dn11 + locals.var_t0__blk808_dn11))), (locals.var_vfbzb_dn12 - (0.5 * (locals.var_v3_dn12 + locals.var_t0__blk808_dn12))),)
    } else {
        (locals.var_vfbeff, locals.var_vfbeff_dn3, locals.var_vfbeff_dn4, locals.var_vfbeff_dn5, locals.var_vfbeff_dn6, locals.var_vfbeff_dn7, locals.var_vfbeff_dn8, locals.var_vfbeff_dn9, locals.var_vfbeff_dn10, locals.var_vfbeff_dn11, locals.var_vfbeff_dn12,)
    }
};
        locals.var_vfbeff = assign27630_e25337;
        locals.var_vfbeff_dn3 = assign27630_e25337_d_n3;
        locals.var_vfbeff_dn4 = assign27630_e25337_d_n4;
        locals.var_vfbeff_dn5 = assign27630_e25337_d_n5;
        locals.var_vfbeff_dn6 = assign27630_e25337_d_n6;
        locals.var_vfbeff_dn7 = assign27630_e25337_d_n7;
        locals.var_vfbeff_dn8 = assign27630_e25337_d_n8;
        locals.var_vfbeff_dn9 = assign27630_e25337_d_n9;
        locals.var_vfbeff_dn10 = assign27630_e25337_d_n10;
        locals.var_vfbeff_dn11 = assign27630_e25337_d_n11;
        locals.var_vfbeff_dn12 = assign27630_e25337_d_n12;
        locals.var_vfbeff_rv = 0.0;

        let assign27640_e25340: f64 = if p.p27 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1391 = assign27640_e25340;
        locals.var_guard1391_rv = 0.0;

        let (assign27650_e25354, assign27650_e25354_d_n3, assign27650_e25354_d_n4, assign27650_e25354_d_n5, assign27650_e25354_d_n6, assign27650_e25354_d_n7, assign27650_e25354_d_n8, assign27650_e25354_d_n9, assign27650_e25354_d_n10, assign27650_e25354_d_n11, assign27650_e25354_d_n12,) = {
    if ((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1391 != 0.0)) {
        let assign27650_e25352: f64 = (locals.var_vfbzb + p.p1033);
        (assign27650_e25352, locals.var_vfbzb_dn3, locals.var_vfbzb_dn4, locals.var_vfbzb_dn5, locals.var_vfbzb_dn6, locals.var_vfbzb_dn7, locals.var_vfbzb_dn8, locals.var_vfbzb_dn9, locals.var_vfbzb_dn10, locals.var_vfbzb_dn11, locals.var_vfbzb_dn12,)
    } else {
        (locals.var_vfbzb2, locals.var_vfbzb2_dn3, locals.var_vfbzb2_dn4, locals.var_vfbzb2_dn5, locals.var_vfbzb2_dn6, locals.var_vfbzb2_dn7, locals.var_vfbzb2_dn8, locals.var_vfbzb2_dn9, locals.var_vfbzb2_dn10, locals.var_vfbzb2_dn11, locals.var_vfbzb2_dn12,)
    }
};
        locals.var_vfbzb2 = assign27650_e25354;
        locals.var_vfbzb2_dn3 = assign27650_e25354_d_n3;
        locals.var_vfbzb2_dn4 = assign27650_e25354_d_n4;
        locals.var_vfbzb2_dn5 = assign27650_e25354_d_n5;
        locals.var_vfbzb2_dn6 = assign27650_e25354_d_n6;
        locals.var_vfbzb2_dn7 = assign27650_e25354_d_n7;
        locals.var_vfbzb2_dn8 = assign27650_e25354_d_n8;
        locals.var_vfbzb2_dn9 = assign27650_e25354_d_n9;
        locals.var_vfbzb2_dn10 = assign27650_e25354_d_n10;
        locals.var_vfbzb2_dn11 = assign27650_e25354_d_n11;
        locals.var_vfbzb2_dn12 = assign27650_e25354_d_n12;
        locals.var_vfbzb2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_84(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign27660_e25372, assign27660_e25372_d_n3, assign27660_e25372_d_n4, assign27660_e25372_d_n5, assign27660_e25372_d_n6, assign27660_e25372_d_n7, assign27660_e25372_d_n8, assign27660_e25372_d_n9, assign27660_e25372_d_n10, assign27660_e25372_d_n11, assign27660_e25372_d_n12,) = {
    if ((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1391 != 0.0)) {
        let assign27660_e25366: f64 = (locals.var_vfbzb2 - locals.var_vgs_eff2);
        let assign27660_e25368: f64 = (assign27660_e25366 + locals.var_vbseff);
        let assign27660_e25370: f64 = (assign27660_e25368 - 0.02);
        (assign27660_e25370, (locals.var_vfbzb2_dn3 + locals.var_vbseff_dn3), (locals.var_vfbzb2_dn4 + locals.var_vbseff_dn4), (locals.var_vfbzb2_dn5 + locals.var_vbseff_dn5), (locals.var_vfbzb2_dn6 + locals.var_vbseff_dn6), ((locals.var_vfbzb2_dn7 - locals.var_vgs_eff2_dn7) + locals.var_vbseff_dn7), ((locals.var_vfbzb2_dn8 - locals.var_vgs_eff2_dn8) + locals.var_vbseff_dn8), ((locals.var_vfbzb2_dn9 - locals.var_vgs_eff2_dn9) + locals.var_vbseff_dn9), (locals.var_vfbzb2_dn10 + locals.var_vbseff_dn10), (locals.var_vfbzb2_dn11 + locals.var_vbseff_dn11), (locals.var_vfbzb2_dn12 + locals.var_vbseff_dn12),)
    } else {
        (locals.var_v3, locals.var_v3_dn3, locals.var_v3_dn4, locals.var_v3_dn5, locals.var_v3_dn6, locals.var_v3_dn7, locals.var_v3_dn8, locals.var_v3_dn9, locals.var_v3_dn10, locals.var_v3_dn11, locals.var_v3_dn12,)
    }
};
        locals.var_v3 = assign27660_e25372;
        locals.var_v3_dn3 = assign27660_e25372_d_n3;
        locals.var_v3_dn4 = assign27660_e25372_d_n4;
        locals.var_v3_dn5 = assign27660_e25372_d_n5;
        locals.var_v3_dn6 = assign27660_e25372_d_n6;
        locals.var_v3_dn7 = assign27660_e25372_d_n7;
        locals.var_v3_dn8 = assign27660_e25372_d_n8;
        locals.var_v3_dn9 = assign27660_e25372_d_n9;
        locals.var_v3_dn10 = assign27660_e25372_d_n10;
        locals.var_v3_dn11 = assign27660_e25372_d_n11;
        locals.var_v3_dn12 = assign27660_e25372_d_n12;
        locals.var_v3_rv = 0.0;

        let assign27670_e25375: f64 = if locals.var_vfbzb2 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1392 = assign27670_e25375;
        locals.var_guard1392_rv = 0.0;

        let (assign27680_e25398, assign27680_e25398_d_n3, assign27680_e25398_d_n4, assign27680_e25398_d_n5, assign27680_e25398_d_n6, assign27680_e25398_d_n7, assign27680_e25398_d_n8, assign27680_e25398_d_n9, assign27680_e25398_d_n10, assign27680_e25398_d_n11, assign27680_e25398_d_n12,) = {
    if (((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1391 != 0.0)) && (locals.var_guard1392 != 0.0)) {
        let assign27680_e25389: f64 = (locals.var_v3 * locals.var_v3);
        let assign27680_e25392: f64 = (100.0 * 0.02);
        let assign27680_e25394: f64 = (assign27680_e25392 * locals.var_vfbzb2);
        let assign27680_e25395: f64 = (assign27680_e25389 - assign27680_e25394);
        let assign27680_e25396: f64 = (assign27680_e25395).sqrt();
        (assign27680_e25396, ((((locals.var_v3_dn3 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn3)) - (assign27680_e25392 * locals.var_vfbzb2_dn3)) / (2.0 * assign27680_e25396)), ((((locals.var_v3_dn4 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn4)) - (assign27680_e25392 * locals.var_vfbzb2_dn4)) / (2.0 * assign27680_e25396)), ((((locals.var_v3_dn5 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn5)) - (assign27680_e25392 * locals.var_vfbzb2_dn5)) / (2.0 * assign27680_e25396)), ((((locals.var_v3_dn6 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn6)) - (assign27680_e25392 * locals.var_vfbzb2_dn6)) / (2.0 * assign27680_e25396)), ((((locals.var_v3_dn7 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn7)) - (assign27680_e25392 * locals.var_vfbzb2_dn7)) / (2.0 * assign27680_e25396)), ((((locals.var_v3_dn8 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn8)) - (assign27680_e25392 * locals.var_vfbzb2_dn8)) / (2.0 * assign27680_e25396)), ((((locals.var_v3_dn9 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn9)) - (assign27680_e25392 * locals.var_vfbzb2_dn9)) / (2.0 * assign27680_e25396)), ((((locals.var_v3_dn10 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn10)) - (assign27680_e25392 * locals.var_vfbzb2_dn10)) / (2.0 * assign27680_e25396)), ((((locals.var_v3_dn11 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn11)) - (assign27680_e25392 * locals.var_vfbzb2_dn11)) / (2.0 * assign27680_e25396)), ((((locals.var_v3_dn12 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn12)) - (assign27680_e25392 * locals.var_vfbzb2_dn12)) / (2.0 * assign27680_e25396)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign27680_e25398;
        locals.var_t0__blk808_dn3 = assign27680_e25398_d_n3;
        locals.var_t0__blk808_dn4 = assign27680_e25398_d_n4;
        locals.var_t0__blk808_dn5 = assign27680_e25398_d_n5;
        locals.var_t0__blk808_dn6 = assign27680_e25398_d_n6;
        locals.var_t0__blk808_dn7 = assign27680_e25398_d_n7;
        locals.var_t0__blk808_dn8 = assign27680_e25398_d_n8;
        locals.var_t0__blk808_dn9 = assign27680_e25398_d_n9;
        locals.var_t0__blk808_dn10 = assign27680_e25398_d_n10;
        locals.var_t0__blk808_dn11 = assign27680_e25398_d_n11;
        locals.var_t0__blk808_dn12 = assign27680_e25398_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign27690_e25422, assign27690_e25422_d_n3, assign27690_e25422_d_n4, assign27690_e25422_d_n5, assign27690_e25422_d_n6, assign27690_e25422_d_n7, assign27690_e25422_d_n8, assign27690_e25422_d_n9, assign27690_e25422_d_n10, assign27690_e25422_d_n11, assign27690_e25422_d_n12,) = {
    if (((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1391 != 0.0)) && (locals.var_guard1392 == 0.0)) {
        let assign27690_e25413: f64 = (locals.var_v3 * locals.var_v3);
        let assign27690_e25416: f64 = (100.0 * 0.02);
        let assign27690_e25418: f64 = (assign27690_e25416 * locals.var_vfbzb2);
        let assign27690_e25419: f64 = (assign27690_e25413 + assign27690_e25418);
        let assign27690_e25420: f64 = (assign27690_e25419).sqrt();
        (assign27690_e25420, ((((locals.var_v3_dn3 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn3)) + (assign27690_e25416 * locals.var_vfbzb2_dn3)) / (2.0 * assign27690_e25420)), ((((locals.var_v3_dn4 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn4)) + (assign27690_e25416 * locals.var_vfbzb2_dn4)) / (2.0 * assign27690_e25420)), ((((locals.var_v3_dn5 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn5)) + (assign27690_e25416 * locals.var_vfbzb2_dn5)) / (2.0 * assign27690_e25420)), ((((locals.var_v3_dn6 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn6)) + (assign27690_e25416 * locals.var_vfbzb2_dn6)) / (2.0 * assign27690_e25420)), ((((locals.var_v3_dn7 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn7)) + (assign27690_e25416 * locals.var_vfbzb2_dn7)) / (2.0 * assign27690_e25420)), ((((locals.var_v3_dn8 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn8)) + (assign27690_e25416 * locals.var_vfbzb2_dn8)) / (2.0 * assign27690_e25420)), ((((locals.var_v3_dn9 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn9)) + (assign27690_e25416 * locals.var_vfbzb2_dn9)) / (2.0 * assign27690_e25420)), ((((locals.var_v3_dn10 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn10)) + (assign27690_e25416 * locals.var_vfbzb2_dn10)) / (2.0 * assign27690_e25420)), ((((locals.var_v3_dn11 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn11)) + (assign27690_e25416 * locals.var_vfbzb2_dn11)) / (2.0 * assign27690_e25420)), ((((locals.var_v3_dn12 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn12)) + (assign27690_e25416 * locals.var_vfbzb2_dn12)) / (2.0 * assign27690_e25420)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign27690_e25422;
        locals.var_t0__blk808_dn3 = assign27690_e25422_d_n3;
        locals.var_t0__blk808_dn4 = assign27690_e25422_d_n4;
        locals.var_t0__blk808_dn5 = assign27690_e25422_d_n5;
        locals.var_t0__blk808_dn6 = assign27690_e25422_d_n6;
        locals.var_t0__blk808_dn7 = assign27690_e25422_d_n7;
        locals.var_t0__blk808_dn8 = assign27690_e25422_d_n8;
        locals.var_t0__blk808_dn9 = assign27690_e25422_d_n9;
        locals.var_t0__blk808_dn10 = assign27690_e25422_d_n10;
        locals.var_t0__blk808_dn11 = assign27690_e25422_d_n11;
        locals.var_t0__blk808_dn12 = assign27690_e25422_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign27700_e25440, assign27700_e25440_d_n3, assign27700_e25440_d_n4, assign27700_e25440_d_n5, assign27700_e25440_d_n6, assign27700_e25440_d_n7, assign27700_e25440_d_n8, assign27700_e25440_d_n9, assign27700_e25440_d_n10, assign27700_e25440_d_n11, assign27700_e25440_d_n12,) = {
    if ((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1391 != 0.0)) {
        let assign27700_e25436: f64 = (locals.var_v3 + locals.var_t0__blk808);
        let assign27700_e25437: f64 = (0.5 * assign27700_e25436);
        let assign27700_e25438: f64 = (locals.var_vfbzb2 - assign27700_e25437);
        (assign27700_e25438, (locals.var_vfbzb2_dn3 - (0.5 * (locals.var_v3_dn3 + locals.var_t0__blk808_dn3))), (locals.var_vfbzb2_dn4 - (0.5 * (locals.var_v3_dn4 + locals.var_t0__blk808_dn4))), (locals.var_vfbzb2_dn5 - (0.5 * (locals.var_v3_dn5 + locals.var_t0__blk808_dn5))), (locals.var_vfbzb2_dn6 - (0.5 * (locals.var_v3_dn6 + locals.var_t0__blk808_dn6))), (locals.var_vfbzb2_dn7 - (0.5 * (locals.var_v3_dn7 + locals.var_t0__blk808_dn7))), (locals.var_vfbzb2_dn8 - (0.5 * (locals.var_v3_dn8 + locals.var_t0__blk808_dn8))), (locals.var_vfbzb2_dn9 - (0.5 * (locals.var_v3_dn9 + locals.var_t0__blk808_dn9))), (locals.var_vfbzb2_dn10 - (0.5 * (locals.var_v3_dn10 + locals.var_t0__blk808_dn10))), (locals.var_vfbzb2_dn11 - (0.5 * (locals.var_v3_dn11 + locals.var_t0__blk808_dn11))), (locals.var_vfbzb2_dn12 - (0.5 * (locals.var_v3_dn12 + locals.var_t0__blk808_dn12))),)
    } else {
        (locals.var_vfbeff2, locals.var_vfbeff2_dn3, locals.var_vfbeff2_dn4, locals.var_vfbeff2_dn5, locals.var_vfbeff2_dn6, locals.var_vfbeff2_dn7, locals.var_vfbeff2_dn8, locals.var_vfbeff2_dn9, locals.var_vfbeff2_dn10, locals.var_vfbeff2_dn11, locals.var_vfbeff2_dn12,)
    }
};
        locals.var_vfbeff2 = assign27700_e25440;
        locals.var_vfbeff2_dn3 = assign27700_e25440_d_n3;
        locals.var_vfbeff2_dn4 = assign27700_e25440_d_n4;
        locals.var_vfbeff2_dn5 = assign27700_e25440_d_n5;
        locals.var_vfbeff2_dn6 = assign27700_e25440_d_n6;
        locals.var_vfbeff2_dn7 = assign27700_e25440_d_n7;
        locals.var_vfbeff2_dn8 = assign27700_e25440_d_n8;
        locals.var_vfbeff2_dn9 = assign27700_e25440_d_n9;
        locals.var_vfbeff2_dn10 = assign27700_e25440_d_n10;
        locals.var_vfbeff2_dn11 = assign27700_e25440_d_n11;
        locals.var_vfbeff2_dn12 = assign27700_e25440_d_n12;
        locals.var_vfbeff2_rv = 0.0;

        let (assign27710_e25456, assign27710_e25456_d_n3, assign27710_e25456_d_n4, assign27710_e25456_d_n5, assign27710_e25456_d_n6, assign27710_e25456_d_n7, assign27710_e25456_d_n8, assign27710_e25456_d_n9, assign27710_e25456_d_n10, assign27710_e25456_d_n11, assign27710_e25456_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) {
        let assign27710_e25450: f64 = (locals.var_vgs_eff__blk790 - locals.var_vbseff);
        let assign27710_e25452: f64 = (assign27710_e25450 - locals.var_vfbzb);
        let assign27710_e25454: f64 = (assign27710_e25452 / locals.var_tox);
        (assign27710_e25454, (((((locals.var_vgs_eff__blk790_dn3 - locals.var_vbseff_dn3) - locals.var_vfbzb_dn3) * locals.var_tox) - (assign27710_e25452 * locals.var_tox_dn3)) / (locals.var_tox * locals.var_tox)), (((((locals.var_vgs_eff__blk790_dn4 - locals.var_vbseff_dn4) - locals.var_vfbzb_dn4) * locals.var_tox) - (assign27710_e25452 * locals.var_tox_dn4)) / (locals.var_tox * locals.var_tox)), (((((locals.var_vgs_eff__blk790_dn5 - locals.var_vbseff_dn5) - locals.var_vfbzb_dn5) * locals.var_tox) - (assign27710_e25452 * locals.var_tox_dn5)) / (locals.var_tox * locals.var_tox)), (((((locals.var_vgs_eff__blk790_dn6 - locals.var_vbseff_dn6) - locals.var_vfbzb_dn6) * locals.var_tox) - (assign27710_e25452 * locals.var_tox_dn6)) / (locals.var_tox * locals.var_tox)), (((((locals.var_vgs_eff__blk790_dn7 - locals.var_vbseff_dn7) - locals.var_vfbzb_dn7) * locals.var_tox) - (assign27710_e25452 * locals.var_tox_dn7)) / (locals.var_tox * locals.var_tox)), (((((locals.var_vgs_eff__blk790_dn8 - locals.var_vbseff_dn8) - locals.var_vfbzb_dn8) * locals.var_tox) - (assign27710_e25452 * locals.var_tox_dn8)) / (locals.var_tox * locals.var_tox)), (((((locals.var_vgs_eff__blk790_dn9 - locals.var_vbseff_dn9) - locals.var_vfbzb_dn9) * locals.var_tox) - (assign27710_e25452 * locals.var_tox_dn9)) / (locals.var_tox * locals.var_tox)), (((((locals.var_vgs_eff__blk790_dn10 - locals.var_vbseff_dn10) - locals.var_vfbzb_dn10) * locals.var_tox) - (assign27710_e25452 * locals.var_tox_dn10)) / (locals.var_tox * locals.var_tox)), (((((locals.var_vgs_eff__blk790_dn11 - locals.var_vbseff_dn11) - locals.var_vfbzb_dn11) * locals.var_tox) - (assign27710_e25452 * locals.var_tox_dn11)) / (locals.var_tox * locals.var_tox)), (((((locals.var_vgs_eff__blk790_dn12 - locals.var_vbseff_dn12) - locals.var_vfbzb_dn12) * locals.var_tox) - (assign27710_e25452 * locals.var_tox_dn12)) / (locals.var_tox * locals.var_tox)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign27710_e25456;
        locals.var_t0__blk808_dn3 = assign27710_e25456_d_n3;
        locals.var_t0__blk808_dn4 = assign27710_e25456_d_n4;
        locals.var_t0__blk808_dn5 = assign27710_e25456_d_n5;
        locals.var_t0__blk808_dn6 = assign27710_e25456_d_n6;
        locals.var_t0__blk808_dn7 = assign27710_e25456_d_n7;
        locals.var_t0__blk808_dn8 = assign27710_e25456_d_n8;
        locals.var_t0__blk808_dn9 = assign27710_e25456_d_n9;
        locals.var_t0__blk808_dn10 = assign27710_e25456_d_n10;
        locals.var_t0__blk808_dn11 = assign27710_e25456_d_n11;
        locals.var_t0__blk808_dn12 = assign27710_e25456_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign27720_e25468, assign27720_e25468_d_n3, assign27720_e25468_d_n4, assign27720_e25468_d_n5, assign27720_e25468_d_n6, assign27720_e25468_d_n7, assign27720_e25468_d_n8, assign27720_e25468_d_n9, assign27720_e25468_d_n10, assign27720_e25468_d_n11, assign27720_e25468_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) {
        let assign27720_e25466: f64 = (locals.var_t0__blk808 * locals.var_pparam_b4soiacde);
        (assign27720_e25466, ((locals.var_t0__blk808_dn3 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk808 * locals.var_pparam_b4soiacde_dn3)), ((locals.var_t0__blk808_dn4 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk808 * locals.var_pparam_b4soiacde_dn4)), ((locals.var_t0__blk808_dn5 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk808 * locals.var_pparam_b4soiacde_dn5)), ((locals.var_t0__blk808_dn6 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk808 * locals.var_pparam_b4soiacde_dn6)), ((locals.var_t0__blk808_dn7 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk808 * locals.var_pparam_b4soiacde_dn7)), ((locals.var_t0__blk808_dn8 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk808 * locals.var_pparam_b4soiacde_dn8)), ((locals.var_t0__blk808_dn9 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk808 * locals.var_pparam_b4soiacde_dn9)), ((locals.var_t0__blk808_dn10 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk808 * locals.var_pparam_b4soiacde_dn10)), ((locals.var_t0__blk808_dn11 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk808 * locals.var_pparam_b4soiacde_dn11)), ((locals.var_t0__blk808_dn12 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk808 * locals.var_pparam_b4soiacde_dn12)),)
    } else {
        (locals.var_tmp__blk824, locals.var_tmp__blk824_dn3, locals.var_tmp__blk824_dn4, locals.var_tmp__blk824_dn5, locals.var_tmp__blk824_dn6, locals.var_tmp__blk824_dn7, locals.var_tmp__blk824_dn8, locals.var_tmp__blk824_dn9, locals.var_tmp__blk824_dn10, locals.var_tmp__blk824_dn11, locals.var_tmp__blk824_dn12,)
    }
};
        locals.var_tmp__blk824 = assign27720_e25468;
        locals.var_tmp__blk824_dn3 = assign27720_e25468_d_n3;
        locals.var_tmp__blk824_dn4 = assign27720_e25468_d_n4;
        locals.var_tmp__blk824_dn5 = assign27720_e25468_d_n5;
        locals.var_tmp__blk824_dn6 = assign27720_e25468_d_n6;
        locals.var_tmp__blk824_dn7 = assign27720_e25468_d_n7;
        locals.var_tmp__blk824_dn8 = assign27720_e25468_d_n8;
        locals.var_tmp__blk824_dn9 = assign27720_e25468_d_n9;
        locals.var_tmp__blk824_dn10 = assign27720_e25468_d_n10;
        locals.var_tmp__blk824_dn11 = assign27720_e25468_d_n11;
        locals.var_tmp__blk824_dn12 = assign27720_e25468_d_n12;
        locals.var_tmp__blk824_rv = 0.0;

        let assign27730_e25470: f64 = (-100.0);
        let assign27730_e25476: f64 = if ((assign27730_e25470 < locals.var_tmp__blk824) && (locals.var_tmp__blk824 < 100.0)) { 1.0 } else { 0.0 };
        locals.var_guard1393 = assign27730_e25476;
        locals.var_guard1393_rv = 0.0;

        let (assign27740_e25491, assign27740_e25491_d_n3, assign27740_e25491_d_n4, assign27740_e25491_d_n5, assign27740_e25491_d_n6, assign27740_e25491_d_n7, assign27740_e25491_d_n8, assign27740_e25491_d_n9, assign27740_e25491_d_n10, assign27740_e25491_d_n11, assign27740_e25491_d_n12,) = {
    if ((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1393 != 0.0)) {
        let assign27740_e25488: f64 = (locals.var_tmp__blk824).exp();
        let assign27740_e25489: f64 = (locals.var_pparam_b4soildeb * assign27740_e25488);
        (assign27740_e25489, ((locals.var_pparam_b4soildeb_dn3 * assign27740_e25488) + (locals.var_pparam_b4soildeb * (assign27740_e25488 * locals.var_tmp__blk824_dn3))), ((locals.var_pparam_b4soildeb_dn4 * assign27740_e25488) + (locals.var_pparam_b4soildeb * (assign27740_e25488 * locals.var_tmp__blk824_dn4))), ((locals.var_pparam_b4soildeb_dn5 * assign27740_e25488) + (locals.var_pparam_b4soildeb * (assign27740_e25488 * locals.var_tmp__blk824_dn5))), ((locals.var_pparam_b4soildeb_dn6 * assign27740_e25488) + (locals.var_pparam_b4soildeb * (assign27740_e25488 * locals.var_tmp__blk824_dn6))), ((locals.var_pparam_b4soildeb_dn7 * assign27740_e25488) + (locals.var_pparam_b4soildeb * (assign27740_e25488 * locals.var_tmp__blk824_dn7))), ((locals.var_pparam_b4soildeb_dn8 * assign27740_e25488) + (locals.var_pparam_b4soildeb * (assign27740_e25488 * locals.var_tmp__blk824_dn8))), ((locals.var_pparam_b4soildeb_dn9 * assign27740_e25488) + (locals.var_pparam_b4soildeb * (assign27740_e25488 * locals.var_tmp__blk824_dn9))), ((locals.var_pparam_b4soildeb_dn10 * assign27740_e25488) + (locals.var_pparam_b4soildeb * (assign27740_e25488 * locals.var_tmp__blk824_dn10))), ((locals.var_pparam_b4soildeb_dn11 * assign27740_e25488) + (locals.var_pparam_b4soildeb * (assign27740_e25488 * locals.var_tmp__blk824_dn11))), ((locals.var_pparam_b4soildeb_dn12 * assign27740_e25488) + (locals.var_pparam_b4soildeb * (assign27740_e25488 * locals.var_tmp__blk824_dn12))),)
    } else {
        (locals.var_tcen__blk964, locals.var_tcen__blk964_dn3, locals.var_tcen__blk964_dn4, locals.var_tcen__blk964_dn5, locals.var_tcen__blk964_dn6, locals.var_tcen__blk964_dn7, locals.var_tcen__blk964_dn8, locals.var_tcen__blk964_dn9, locals.var_tcen__blk964_dn10, locals.var_tcen__blk964_dn11, locals.var_tcen__blk964_dn12,)
    }
};
        locals.var_tcen__blk964 = assign27740_e25491;
        locals.var_tcen__blk964_dn3 = assign27740_e25491_d_n3;
        locals.var_tcen__blk964_dn4 = assign27740_e25491_d_n4;
        locals.var_tcen__blk964_dn5 = assign27740_e25491_d_n5;
        locals.var_tcen__blk964_dn6 = assign27740_e25491_d_n6;
        locals.var_tcen__blk964_dn7 = assign27740_e25491_d_n7;
        locals.var_tcen__blk964_dn8 = assign27740_e25491_d_n8;
        locals.var_tcen__blk964_dn9 = assign27740_e25491_d_n9;
        locals.var_tcen__blk964_dn10 = assign27740_e25491_d_n10;
        locals.var_tcen__blk964_dn11 = assign27740_e25491_d_n11;
        locals.var_tcen__blk964_dn12 = assign27740_e25491_d_n12;
        locals.var_tcen__blk964_rv = 0.0;

        let assign27750_e25494: f64 = (-100.0);
        let assign27750_e25495: f64 = if locals.var_tmp__blk824 <= assign27750_e25494 { 1.0 } else { 0.0 };
        locals.var_guard1394 = assign27750_e25495;
        locals.var_guard1394_rv = 0.0;

        let (assign27760_e25512, assign27760_e25512_d_n3, assign27760_e25512_d_n4, assign27760_e25512_d_n5, assign27760_e25512_d_n6, assign27760_e25512_d_n7, assign27760_e25512_d_n8, assign27760_e25512_d_n9, assign27760_e25512_d_n10, assign27760_e25512_d_n11, assign27760_e25512_d_n12,) = {
    if (((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1393 == 0.0)) && (locals.var_guard1394 != 0.0)) {
        let assign27760_e25510: f64 = (locals.var_pparam_b4soildeb * 3.720075976e-44);
        (assign27760_e25510, (locals.var_pparam_b4soildeb_dn3 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn4 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn5 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn6 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn7 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn8 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn9 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn10 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn11 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn12 * 3.720075976e-44),)
    } else {
        (locals.var_tcen__blk964, locals.var_tcen__blk964_dn3, locals.var_tcen__blk964_dn4, locals.var_tcen__blk964_dn5, locals.var_tcen__blk964_dn6, locals.var_tcen__blk964_dn7, locals.var_tcen__blk964_dn8, locals.var_tcen__blk964_dn9, locals.var_tcen__blk964_dn10, locals.var_tcen__blk964_dn11, locals.var_tcen__blk964_dn12,)
    }
};
        locals.var_tcen__blk964 = assign27760_e25512;
        locals.var_tcen__blk964_dn3 = assign27760_e25512_d_n3;
        locals.var_tcen__blk964_dn4 = assign27760_e25512_d_n4;
        locals.var_tcen__blk964_dn5 = assign27760_e25512_d_n5;
        locals.var_tcen__blk964_dn6 = assign27760_e25512_d_n6;
        locals.var_tcen__blk964_dn7 = assign27760_e25512_d_n7;
        locals.var_tcen__blk964_dn8 = assign27760_e25512_d_n8;
        locals.var_tcen__blk964_dn9 = assign27760_e25512_d_n9;
        locals.var_tcen__blk964_dn10 = assign27760_e25512_d_n10;
        locals.var_tcen__blk964_dn11 = assign27760_e25512_d_n11;
        locals.var_tcen__blk964_dn12 = assign27760_e25512_d_n12;
        locals.var_tcen__blk964_rv = 0.0;

        let (assign27770_e25530, assign27770_e25530_d_n3, assign27770_e25530_d_n4, assign27770_e25530_d_n5, assign27770_e25530_d_n6, assign27770_e25530_d_n7, assign27770_e25530_d_n8, assign27770_e25530_d_n9, assign27770_e25530_d_n10, assign27770_e25530_d_n11, assign27770_e25530_d_n12,) = {
    if (((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1393 == 0.0)) && (locals.var_guard1394 == 0.0)) {
        let assign27770_e25528: f64 = (locals.var_pparam_b4soildeb * 2.688117142e43);
        (assign27770_e25528, (locals.var_pparam_b4soildeb_dn3 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn4 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn5 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn6 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn7 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn8 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn9 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn10 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn11 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn12 * 2.688117142e43),)
    } else {
        (locals.var_tcen__blk964, locals.var_tcen__blk964_dn3, locals.var_tcen__blk964_dn4, locals.var_tcen__blk964_dn5, locals.var_tcen__blk964_dn6, locals.var_tcen__blk964_dn7, locals.var_tcen__blk964_dn8, locals.var_tcen__blk964_dn9, locals.var_tcen__blk964_dn10, locals.var_tcen__blk964_dn11, locals.var_tcen__blk964_dn12,)
    }
};
        locals.var_tcen__blk964 = assign27770_e25530;
        locals.var_tcen__blk964_dn3 = assign27770_e25530_d_n3;
        locals.var_tcen__blk964_dn4 = assign27770_e25530_d_n4;
        locals.var_tcen__blk964_dn5 = assign27770_e25530_d_n5;
        locals.var_tcen__blk964_dn6 = assign27770_e25530_d_n6;
        locals.var_tcen__blk964_dn7 = assign27770_e25530_d_n7;
        locals.var_tcen__blk964_dn8 = assign27770_e25530_d_n8;
        locals.var_tcen__blk964_dn9 = assign27770_e25530_d_n9;
        locals.var_tcen__blk964_dn10 = assign27770_e25530_d_n10;
        locals.var_tcen__blk964_dn11 = assign27770_e25530_d_n11;
        locals.var_tcen__blk964_dn12 = assign27770_e25530_d_n12;
        locals.var_tcen__blk964_rv = 0.0;

        let (assign27780_e25542, assign27780_e25542_d_n3, assign27780_e25542_d_n4, assign27780_e25542_d_n5, assign27780_e25542_d_n6, assign27780_e25542_d_n7, assign27780_e25542_d_n8, assign27780_e25542_d_n9, assign27780_e25542_d_n10, assign27780_e25542_d_n11, assign27780_e25542_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) {
        let assign27780_e25540: f64 = (0.001 * locals.var_b4soitoxp);
        (assign27780_e25540, (0.001 * locals.var_b4soitoxp_dn3), (0.001 * locals.var_b4soitoxp_dn4), (0.001 * locals.var_b4soitoxp_dn5), (0.001 * locals.var_b4soitoxp_dn6), (0.001 * locals.var_b4soitoxp_dn7), (0.001 * locals.var_b4soitoxp_dn8), (0.001 * locals.var_b4soitoxp_dn9), (0.001 * locals.var_b4soitoxp_dn10), (0.001 * locals.var_b4soitoxp_dn11), (0.001 * locals.var_b4soitoxp_dn12),)
    } else {
        (locals.var_link, locals.var_link_dn3, locals.var_link_dn4, locals.var_link_dn5, locals.var_link_dn6, locals.var_link_dn7, locals.var_link_dn8, locals.var_link_dn9, locals.var_link_dn10, locals.var_link_dn11, locals.var_link_dn12,)
    }
};
        locals.var_link = assign27780_e25542;
        locals.var_link_dn3 = assign27780_e25542_d_n3;
        locals.var_link_dn4 = assign27780_e25542_d_n4;
        locals.var_link_dn5 = assign27780_e25542_d_n5;
        locals.var_link_dn6 = assign27780_e25542_d_n6;
        locals.var_link_dn7 = assign27780_e25542_d_n7;
        locals.var_link_dn8 = assign27780_e25542_d_n8;
        locals.var_link_dn9 = assign27780_e25542_d_n9;
        locals.var_link_dn10 = assign27780_e25542_d_n10;
        locals.var_link_dn11 = assign27780_e25542_d_n11;
        locals.var_link_dn12 = assign27780_e25542_d_n12;
        locals.var_link_rv = 0.0;

        let (assign27790_e25556, assign27790_e25556_d_n3, assign27790_e25556_d_n4, assign27790_e25556_d_n5, assign27790_e25556_d_n6, assign27790_e25556_d_n7, assign27790_e25556_d_n8, assign27790_e25556_d_n9, assign27790_e25556_d_n10, assign27790_e25556_d_n11, assign27790_e25556_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) {
        let assign27790_e25552: f64 = (locals.var_pparam_b4soildeb - locals.var_tcen__blk964);
        let assign27790_e25554: f64 = (assign27790_e25552 - locals.var_link);
        (assign27790_e25554, ((locals.var_pparam_b4soildeb_dn3 - locals.var_tcen__blk964_dn3) - locals.var_link_dn3), ((locals.var_pparam_b4soildeb_dn4 - locals.var_tcen__blk964_dn4) - locals.var_link_dn4), ((locals.var_pparam_b4soildeb_dn5 - locals.var_tcen__blk964_dn5) - locals.var_link_dn5), ((locals.var_pparam_b4soildeb_dn6 - locals.var_tcen__blk964_dn6) - locals.var_link_dn6), ((locals.var_pparam_b4soildeb_dn7 - locals.var_tcen__blk964_dn7) - locals.var_link_dn7), ((locals.var_pparam_b4soildeb_dn8 - locals.var_tcen__blk964_dn8) - locals.var_link_dn8), ((locals.var_pparam_b4soildeb_dn9 - locals.var_tcen__blk964_dn9) - locals.var_link_dn9), ((locals.var_pparam_b4soildeb_dn10 - locals.var_tcen__blk964_dn10) - locals.var_link_dn10), ((locals.var_pparam_b4soildeb_dn11 - locals.var_tcen__blk964_dn11) - locals.var_link_dn11), ((locals.var_pparam_b4soildeb_dn12 - locals.var_tcen__blk964_dn12) - locals.var_link_dn12),)
    } else {
        (locals.var_v3, locals.var_v3_dn3, locals.var_v3_dn4, locals.var_v3_dn5, locals.var_v3_dn6, locals.var_v3_dn7, locals.var_v3_dn8, locals.var_v3_dn9, locals.var_v3_dn10, locals.var_v3_dn11, locals.var_v3_dn12,)
    }
};
        locals.var_v3 = assign27790_e25556;
        locals.var_v3_dn3 = assign27790_e25556_d_n3;
        locals.var_v3_dn4 = assign27790_e25556_d_n4;
        locals.var_v3_dn5 = assign27790_e25556_d_n5;
        locals.var_v3_dn6 = assign27790_e25556_d_n6;
        locals.var_v3_dn7 = assign27790_e25556_d_n7;
        locals.var_v3_dn8 = assign27790_e25556_d_n8;
        locals.var_v3_dn9 = assign27790_e25556_d_n9;
        locals.var_v3_dn10 = assign27790_e25556_d_n10;
        locals.var_v3_dn11 = assign27790_e25556_d_n11;
        locals.var_v3_dn12 = assign27790_e25556_d_n12;
        locals.var_v3_rv = 0.0;

        let (assign27800_e25575, assign27800_e25575_d_n3, assign27800_e25575_d_n4, assign27800_e25575_d_n5, assign27800_e25575_d_n6, assign27800_e25575_d_n7, assign27800_e25575_d_n8, assign27800_e25575_d_n9, assign27800_e25575_d_n10, assign27800_e25575_d_n11, assign27800_e25575_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) {
        let assign27800_e25566: f64 = (locals.var_v3 * locals.var_v3);
        let assign27800_e25569: f64 = (4.0 * locals.var_link);
        let assign27800_e25571: f64 = (assign27800_e25569 * locals.var_pparam_b4soildeb);
        let assign27800_e25572: f64 = (assign27800_e25566 + assign27800_e25571);
        let assign27800_e25573: f64 = (assign27800_e25572).sqrt();
        (assign27800_e25573, ((((locals.var_v3_dn3 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn3)) + (((4.0 * locals.var_link_dn3) * locals.var_pparam_b4soildeb) + (assign27800_e25569 * locals.var_pparam_b4soildeb_dn3))) / (2.0 * assign27800_e25573)), ((((locals.var_v3_dn4 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn4)) + (((4.0 * locals.var_link_dn4) * locals.var_pparam_b4soildeb) + (assign27800_e25569 * locals.var_pparam_b4soildeb_dn4))) / (2.0 * assign27800_e25573)), ((((locals.var_v3_dn5 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn5)) + (((4.0 * locals.var_link_dn5) * locals.var_pparam_b4soildeb) + (assign27800_e25569 * locals.var_pparam_b4soildeb_dn5))) / (2.0 * assign27800_e25573)), ((((locals.var_v3_dn6 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn6)) + (((4.0 * locals.var_link_dn6) * locals.var_pparam_b4soildeb) + (assign27800_e25569 * locals.var_pparam_b4soildeb_dn6))) / (2.0 * assign27800_e25573)), ((((locals.var_v3_dn7 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn7)) + (((4.0 * locals.var_link_dn7) * locals.var_pparam_b4soildeb) + (assign27800_e25569 * locals.var_pparam_b4soildeb_dn7))) / (2.0 * assign27800_e25573)), ((((locals.var_v3_dn8 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn8)) + (((4.0 * locals.var_link_dn8) * locals.var_pparam_b4soildeb) + (assign27800_e25569 * locals.var_pparam_b4soildeb_dn8))) / (2.0 * assign27800_e25573)), ((((locals.var_v3_dn9 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn9)) + (((4.0 * locals.var_link_dn9) * locals.var_pparam_b4soildeb) + (assign27800_e25569 * locals.var_pparam_b4soildeb_dn9))) / (2.0 * assign27800_e25573)), ((((locals.var_v3_dn10 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn10)) + (((4.0 * locals.var_link_dn10) * locals.var_pparam_b4soildeb) + (assign27800_e25569 * locals.var_pparam_b4soildeb_dn10))) / (2.0 * assign27800_e25573)), ((((locals.var_v3_dn11 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn11)) + (((4.0 * locals.var_link_dn11) * locals.var_pparam_b4soildeb) + (assign27800_e25569 * locals.var_pparam_b4soildeb_dn11))) / (2.0 * assign27800_e25573)), ((((locals.var_v3_dn12 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn12)) + (((4.0 * locals.var_link_dn12) * locals.var_pparam_b4soildeb) + (assign27800_e25569 * locals.var_pparam_b4soildeb_dn12))) / (2.0 * assign27800_e25573)),)
    } else {
        (locals.var_v4, locals.var_v4_dn3, locals.var_v4_dn4, locals.var_v4_dn5, locals.var_v4_dn6, locals.var_v4_dn7, locals.var_v4_dn8, locals.var_v4_dn9, locals.var_v4_dn10, locals.var_v4_dn11, locals.var_v4_dn12,)
    }
};
        locals.var_v4 = assign27800_e25575;
        locals.var_v4_dn3 = assign27800_e25575_d_n3;
        locals.var_v4_dn4 = assign27800_e25575_d_n4;
        locals.var_v4_dn5 = assign27800_e25575_d_n5;
        locals.var_v4_dn6 = assign27800_e25575_d_n6;
        locals.var_v4_dn7 = assign27800_e25575_d_n7;
        locals.var_v4_dn8 = assign27800_e25575_d_n8;
        locals.var_v4_dn9 = assign27800_e25575_d_n9;
        locals.var_v4_dn10 = assign27800_e25575_d_n10;
        locals.var_v4_dn11 = assign27800_e25575_d_n11;
        locals.var_v4_dn12 = assign27800_e25575_d_n12;
        locals.var_v4_rv = 0.0;

        let (assign27810_e25591, assign27810_e25591_d_n3, assign27810_e25591_d_n4, assign27810_e25591_d_n5, assign27810_e25591_d_n6, assign27810_e25591_d_n7, assign27810_e25591_d_n8, assign27810_e25591_d_n9, assign27810_e25591_d_n10, assign27810_e25591_d_n11, assign27810_e25591_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) {
        let assign27810_e25587: f64 = (locals.var_v3 + locals.var_v4);
        let assign27810_e25588: f64 = (0.5 * assign27810_e25587);
        let assign27810_e25589: f64 = (locals.var_pparam_b4soildeb - assign27810_e25588);
        (assign27810_e25589, (locals.var_pparam_b4soildeb_dn3 - (0.5 * (locals.var_v3_dn3 + locals.var_v4_dn3))), (locals.var_pparam_b4soildeb_dn4 - (0.5 * (locals.var_v3_dn4 + locals.var_v4_dn4))), (locals.var_pparam_b4soildeb_dn5 - (0.5 * (locals.var_v3_dn5 + locals.var_v4_dn5))), (locals.var_pparam_b4soildeb_dn6 - (0.5 * (locals.var_v3_dn6 + locals.var_v4_dn6))), (locals.var_pparam_b4soildeb_dn7 - (0.5 * (locals.var_v3_dn7 + locals.var_v4_dn7))), (locals.var_pparam_b4soildeb_dn8 - (0.5 * (locals.var_v3_dn8 + locals.var_v4_dn8))), (locals.var_pparam_b4soildeb_dn9 - (0.5 * (locals.var_v3_dn9 + locals.var_v4_dn9))), (locals.var_pparam_b4soildeb_dn10 - (0.5 * (locals.var_v3_dn10 + locals.var_v4_dn10))), (locals.var_pparam_b4soildeb_dn11 - (0.5 * (locals.var_v3_dn11 + locals.var_v4_dn11))), (locals.var_pparam_b4soildeb_dn12 - (0.5 * (locals.var_v3_dn12 + locals.var_v4_dn12))),)
    } else {
        (locals.var_tcen__blk964, locals.var_tcen__blk964_dn3, locals.var_tcen__blk964_dn4, locals.var_tcen__blk964_dn5, locals.var_tcen__blk964_dn6, locals.var_tcen__blk964_dn7, locals.var_tcen__blk964_dn8, locals.var_tcen__blk964_dn9, locals.var_tcen__blk964_dn10, locals.var_tcen__blk964_dn11, locals.var_tcen__blk964_dn12,)
    }
};
        locals.var_tcen__blk964 = assign27810_e25591;
        locals.var_tcen__blk964_dn3 = assign27810_e25591_d_n3;
        locals.var_tcen__blk964_dn4 = assign27810_e25591_d_n4;
        locals.var_tcen__blk964_dn5 = assign27810_e25591_d_n5;
        locals.var_tcen__blk964_dn6 = assign27810_e25591_d_n6;
        locals.var_tcen__blk964_dn7 = assign27810_e25591_d_n7;
        locals.var_tcen__blk964_dn8 = assign27810_e25591_d_n8;
        locals.var_tcen__blk964_dn9 = assign27810_e25591_d_n9;
        locals.var_tcen__blk964_dn10 = assign27810_e25591_d_n10;
        locals.var_tcen__blk964_dn11 = assign27810_e25591_d_n11;
        locals.var_tcen__blk964_dn12 = assign27810_e25591_d_n12;
        locals.var_tcen__blk964_rv = 0.0;

        let assign27820_e25594: f64 = if locals.var_tcen__blk964 < 1e-15 { 1.0 } else { 0.0 };
        locals.var_guard1395 = assign27820_e25594;
        locals.var_guard1395_rv = 0.0;

        let (assign27830_e25606, assign27830_e25606_d_n3, assign27830_e25606_d_n4, assign27830_e25606_d_n5, assign27830_e25606_d_n6, assign27830_e25606_d_n7, assign27830_e25606_d_n8, assign27830_e25606_d_n9, assign27830_e25606_d_n10, assign27830_e25606_d_n11, assign27830_e25606_d_n12,) = {
    if ((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1395 != 0.0)) {
        (1e-15, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tcen__blk964, locals.var_tcen__blk964_dn3, locals.var_tcen__blk964_dn4, locals.var_tcen__blk964_dn5, locals.var_tcen__blk964_dn6, locals.var_tcen__blk964_dn7, locals.var_tcen__blk964_dn8, locals.var_tcen__blk964_dn9, locals.var_tcen__blk964_dn10, locals.var_tcen__blk964_dn11, locals.var_tcen__blk964_dn12,)
    }
};
        locals.var_tcen__blk964 = assign27830_e25606;
        locals.var_tcen__blk964_dn3 = assign27830_e25606_d_n3;
        locals.var_tcen__blk964_dn4 = assign27830_e25606_d_n4;
        locals.var_tcen__blk964_dn5 = assign27830_e25606_d_n5;
        locals.var_tcen__blk964_dn6 = assign27830_e25606_d_n6;
        locals.var_tcen__blk964_dn7 = assign27830_e25606_d_n7;
        locals.var_tcen__blk964_dn8 = assign27830_e25606_d_n8;
        locals.var_tcen__blk964_dn9 = assign27830_e25606_d_n9;
        locals.var_tcen__blk964_dn10 = assign27830_e25606_d_n10;
        locals.var_tcen__blk964_dn11 = assign27830_e25606_d_n11;
        locals.var_tcen__blk964_dn12 = assign27830_e25606_d_n12;
        locals.var_tcen__blk964_rv = 0.0;

        let assign27840_e25609: f64 = if p.p27 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1396 = assign27840_e25609;
        locals.var_guard1396_rv = 0.0;

        let (assign27850_e25627, assign27850_e25627_d_n3, assign27850_e25627_d_n4, assign27850_e25627_d_n5, assign27850_e25627_d_n6, assign27850_e25627_d_n7, assign27850_e25627_d_n8, assign27850_e25627_d_n9, assign27850_e25627_d_n10, assign27850_e25627_d_n11, assign27850_e25627_d_n12,) = {
    if ((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1396 != 0.0)) {
        let assign27850_e25621: f64 = (locals.var_vgs_eff2 - locals.var_vbseff);
        let assign27850_e25623: f64 = (assign27850_e25621 - locals.var_vfbzb2);
        let assign27850_e25625: f64 = (assign27850_e25623 / locals.var_tox);
        (assign27850_e25625, (((((-locals.var_vbseff_dn3) - locals.var_vfbzb2_dn3) * locals.var_tox) - (assign27850_e25623 * locals.var_tox_dn3)) / (locals.var_tox * locals.var_tox)), (((((-locals.var_vbseff_dn4) - locals.var_vfbzb2_dn4) * locals.var_tox) - (assign27850_e25623 * locals.var_tox_dn4)) / (locals.var_tox * locals.var_tox)), (((((-locals.var_vbseff_dn5) - locals.var_vfbzb2_dn5) * locals.var_tox) - (assign27850_e25623 * locals.var_tox_dn5)) / (locals.var_tox * locals.var_tox)), (((((-locals.var_vbseff_dn6) - locals.var_vfbzb2_dn6) * locals.var_tox) - (assign27850_e25623 * locals.var_tox_dn6)) / (locals.var_tox * locals.var_tox)), (((((locals.var_vgs_eff2_dn7 - locals.var_vbseff_dn7) - locals.var_vfbzb2_dn7) * locals.var_tox) - (assign27850_e25623 * locals.var_tox_dn7)) / (locals.var_tox * locals.var_tox)), (((((locals.var_vgs_eff2_dn8 - locals.var_vbseff_dn8) - locals.var_vfbzb2_dn8) * locals.var_tox) - (assign27850_e25623 * locals.var_tox_dn8)) / (locals.var_tox * locals.var_tox)), (((((locals.var_vgs_eff2_dn9 - locals.var_vbseff_dn9) - locals.var_vfbzb2_dn9) * locals.var_tox) - (assign27850_e25623 * locals.var_tox_dn9)) / (locals.var_tox * locals.var_tox)), (((((-locals.var_vbseff_dn10) - locals.var_vfbzb2_dn10) * locals.var_tox) - (assign27850_e25623 * locals.var_tox_dn10)) / (locals.var_tox * locals.var_tox)), (((((-locals.var_vbseff_dn11) - locals.var_vfbzb2_dn11) * locals.var_tox) - (assign27850_e25623 * locals.var_tox_dn11)) / (locals.var_tox * locals.var_tox)), (((((-locals.var_vbseff_dn12) - locals.var_vfbzb2_dn12) * locals.var_tox) - (assign27850_e25623 * locals.var_tox_dn12)) / (locals.var_tox * locals.var_tox)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign27850_e25627;
        locals.var_t0__blk808_dn3 = assign27850_e25627_d_n3;
        locals.var_t0__blk808_dn4 = assign27850_e25627_d_n4;
        locals.var_t0__blk808_dn5 = assign27850_e25627_d_n5;
        locals.var_t0__blk808_dn6 = assign27850_e25627_d_n6;
        locals.var_t0__blk808_dn7 = assign27850_e25627_d_n7;
        locals.var_t0__blk808_dn8 = assign27850_e25627_d_n8;
        locals.var_t0__blk808_dn9 = assign27850_e25627_d_n9;
        locals.var_t0__blk808_dn10 = assign27850_e25627_d_n10;
        locals.var_t0__blk808_dn11 = assign27850_e25627_d_n11;
        locals.var_t0__blk808_dn12 = assign27850_e25627_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign27860_e25641, assign27860_e25641_d_n3, assign27860_e25641_d_n4, assign27860_e25641_d_n5, assign27860_e25641_d_n6, assign27860_e25641_d_n7, assign27860_e25641_d_n8, assign27860_e25641_d_n9, assign27860_e25641_d_n10, assign27860_e25641_d_n11, assign27860_e25641_d_n12,) = {
    if ((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1396 != 0.0)) {
        let assign27860_e25639: f64 = (locals.var_t0__blk808 * locals.var_pparam_b4soiacde);
        (assign27860_e25639, ((locals.var_t0__blk808_dn3 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk808 * locals.var_pparam_b4soiacde_dn3)), ((locals.var_t0__blk808_dn4 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk808 * locals.var_pparam_b4soiacde_dn4)), ((locals.var_t0__blk808_dn5 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk808 * locals.var_pparam_b4soiacde_dn5)), ((locals.var_t0__blk808_dn6 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk808 * locals.var_pparam_b4soiacde_dn6)), ((locals.var_t0__blk808_dn7 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk808 * locals.var_pparam_b4soiacde_dn7)), ((locals.var_t0__blk808_dn8 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk808 * locals.var_pparam_b4soiacde_dn8)), ((locals.var_t0__blk808_dn9 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk808 * locals.var_pparam_b4soiacde_dn9)), ((locals.var_t0__blk808_dn10 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk808 * locals.var_pparam_b4soiacde_dn10)), ((locals.var_t0__blk808_dn11 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk808 * locals.var_pparam_b4soiacde_dn11)), ((locals.var_t0__blk808_dn12 * locals.var_pparam_b4soiacde) + (locals.var_t0__blk808 * locals.var_pparam_b4soiacde_dn12)),)
    } else {
        (locals.var_tmp__blk824, locals.var_tmp__blk824_dn3, locals.var_tmp__blk824_dn4, locals.var_tmp__blk824_dn5, locals.var_tmp__blk824_dn6, locals.var_tmp__blk824_dn7, locals.var_tmp__blk824_dn8, locals.var_tmp__blk824_dn9, locals.var_tmp__blk824_dn10, locals.var_tmp__blk824_dn11, locals.var_tmp__blk824_dn12,)
    }
};
        locals.var_tmp__blk824 = assign27860_e25641;
        locals.var_tmp__blk824_dn3 = assign27860_e25641_d_n3;
        locals.var_tmp__blk824_dn4 = assign27860_e25641_d_n4;
        locals.var_tmp__blk824_dn5 = assign27860_e25641_d_n5;
        locals.var_tmp__blk824_dn6 = assign27860_e25641_d_n6;
        locals.var_tmp__blk824_dn7 = assign27860_e25641_d_n7;
        locals.var_tmp__blk824_dn8 = assign27860_e25641_d_n8;
        locals.var_tmp__blk824_dn9 = assign27860_e25641_d_n9;
        locals.var_tmp__blk824_dn10 = assign27860_e25641_d_n10;
        locals.var_tmp__blk824_dn11 = assign27860_e25641_d_n11;
        locals.var_tmp__blk824_dn12 = assign27860_e25641_d_n12;
        locals.var_tmp__blk824_rv = 0.0;

        let assign27870_e25643: f64 = (-100.0);
        let assign27870_e25649: f64 = if ((assign27870_e25643 < locals.var_tmp__blk824) && (locals.var_tmp__blk824 < 100.0)) { 1.0 } else { 0.0 };
        locals.var_guard1397 = assign27870_e25649;
        locals.var_guard1397_rv = 0.0;

        let (assign27880_e25666, assign27880_e25666_d_n3, assign27880_e25666_d_n4, assign27880_e25666_d_n5, assign27880_e25666_d_n6, assign27880_e25666_d_n7, assign27880_e25666_d_n8, assign27880_e25666_d_n9, assign27880_e25666_d_n10, assign27880_e25666_d_n11, assign27880_e25666_d_n12,) = {
    if (((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1396 != 0.0)) && (locals.var_guard1397 != 0.0)) {
        let assign27880_e25663: f64 = (locals.var_tmp__blk824).exp();
        let assign27880_e25664: f64 = (locals.var_pparam_b4soildeb * assign27880_e25663);
        (assign27880_e25664, ((locals.var_pparam_b4soildeb_dn3 * assign27880_e25663) + (locals.var_pparam_b4soildeb * (assign27880_e25663 * locals.var_tmp__blk824_dn3))), ((locals.var_pparam_b4soildeb_dn4 * assign27880_e25663) + (locals.var_pparam_b4soildeb * (assign27880_e25663 * locals.var_tmp__blk824_dn4))), ((locals.var_pparam_b4soildeb_dn5 * assign27880_e25663) + (locals.var_pparam_b4soildeb * (assign27880_e25663 * locals.var_tmp__blk824_dn5))), ((locals.var_pparam_b4soildeb_dn6 * assign27880_e25663) + (locals.var_pparam_b4soildeb * (assign27880_e25663 * locals.var_tmp__blk824_dn6))), ((locals.var_pparam_b4soildeb_dn7 * assign27880_e25663) + (locals.var_pparam_b4soildeb * (assign27880_e25663 * locals.var_tmp__blk824_dn7))), ((locals.var_pparam_b4soildeb_dn8 * assign27880_e25663) + (locals.var_pparam_b4soildeb * (assign27880_e25663 * locals.var_tmp__blk824_dn8))), ((locals.var_pparam_b4soildeb_dn9 * assign27880_e25663) + (locals.var_pparam_b4soildeb * (assign27880_e25663 * locals.var_tmp__blk824_dn9))), ((locals.var_pparam_b4soildeb_dn10 * assign27880_e25663) + (locals.var_pparam_b4soildeb * (assign27880_e25663 * locals.var_tmp__blk824_dn10))), ((locals.var_pparam_b4soildeb_dn11 * assign27880_e25663) + (locals.var_pparam_b4soildeb * (assign27880_e25663 * locals.var_tmp__blk824_dn11))), ((locals.var_pparam_b4soildeb_dn12 * assign27880_e25663) + (locals.var_pparam_b4soildeb * (assign27880_e25663 * locals.var_tmp__blk824_dn12))),)
    } else {
        (locals.var_tcen2, locals.var_tcen2_dn3, locals.var_tcen2_dn4, locals.var_tcen2_dn5, locals.var_tcen2_dn6, locals.var_tcen2_dn7, locals.var_tcen2_dn8, locals.var_tcen2_dn9, locals.var_tcen2_dn10, locals.var_tcen2_dn11, locals.var_tcen2_dn12,)
    }
};
        locals.var_tcen2 = assign27880_e25666;
        locals.var_tcen2_dn3 = assign27880_e25666_d_n3;
        locals.var_tcen2_dn4 = assign27880_e25666_d_n4;
        locals.var_tcen2_dn5 = assign27880_e25666_d_n5;
        locals.var_tcen2_dn6 = assign27880_e25666_d_n6;
        locals.var_tcen2_dn7 = assign27880_e25666_d_n7;
        locals.var_tcen2_dn8 = assign27880_e25666_d_n8;
        locals.var_tcen2_dn9 = assign27880_e25666_d_n9;
        locals.var_tcen2_dn10 = assign27880_e25666_d_n10;
        locals.var_tcen2_dn11 = assign27880_e25666_d_n11;
        locals.var_tcen2_dn12 = assign27880_e25666_d_n12;
        locals.var_tcen2_rv = 0.0;

        let assign27890_e25669: f64 = (-100.0);
        let assign27890_e25670: f64 = if locals.var_tmp__blk824 <= assign27890_e25669 { 1.0 } else { 0.0 };
        locals.var_guard1398 = assign27890_e25670;
        locals.var_guard1398_rv = 0.0;

        let (assign27900_e25689, assign27900_e25689_d_n3, assign27900_e25689_d_n4, assign27900_e25689_d_n5, assign27900_e25689_d_n6, assign27900_e25689_d_n7, assign27900_e25689_d_n8, assign27900_e25689_d_n9, assign27900_e25689_d_n10, assign27900_e25689_d_n11, assign27900_e25689_d_n12,) = {
    if ((((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1396 != 0.0)) && (locals.var_guard1397 == 0.0)) && (locals.var_guard1398 != 0.0)) {
        let assign27900_e25687: f64 = (locals.var_pparam_b4soildeb * 3.720075976e-44);
        (assign27900_e25687, (locals.var_pparam_b4soildeb_dn3 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn4 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn5 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn6 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn7 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn8 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn9 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn10 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn11 * 3.720075976e-44), (locals.var_pparam_b4soildeb_dn12 * 3.720075976e-44),)
    } else {
        (locals.var_tcen2, locals.var_tcen2_dn3, locals.var_tcen2_dn4, locals.var_tcen2_dn5, locals.var_tcen2_dn6, locals.var_tcen2_dn7, locals.var_tcen2_dn8, locals.var_tcen2_dn9, locals.var_tcen2_dn10, locals.var_tcen2_dn11, locals.var_tcen2_dn12,)
    }
};
        locals.var_tcen2 = assign27900_e25689;
        locals.var_tcen2_dn3 = assign27900_e25689_d_n3;
        locals.var_tcen2_dn4 = assign27900_e25689_d_n4;
        locals.var_tcen2_dn5 = assign27900_e25689_d_n5;
        locals.var_tcen2_dn6 = assign27900_e25689_d_n6;
        locals.var_tcen2_dn7 = assign27900_e25689_d_n7;
        locals.var_tcen2_dn8 = assign27900_e25689_d_n8;
        locals.var_tcen2_dn9 = assign27900_e25689_d_n9;
        locals.var_tcen2_dn10 = assign27900_e25689_d_n10;
        locals.var_tcen2_dn11 = assign27900_e25689_d_n11;
        locals.var_tcen2_dn12 = assign27900_e25689_d_n12;
        locals.var_tcen2_rv = 0.0;

        let (assign27910_e25709, assign27910_e25709_d_n3, assign27910_e25709_d_n4, assign27910_e25709_d_n5, assign27910_e25709_d_n6, assign27910_e25709_d_n7, assign27910_e25709_d_n8, assign27910_e25709_d_n9, assign27910_e25709_d_n10, assign27910_e25709_d_n11, assign27910_e25709_d_n12,) = {
    if ((((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1396 != 0.0)) && (locals.var_guard1397 == 0.0)) && (locals.var_guard1398 == 0.0)) {
        let assign27910_e25707: f64 = (locals.var_pparam_b4soildeb * 2.688117142e43);
        (assign27910_e25707, (locals.var_pparam_b4soildeb_dn3 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn4 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn5 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn6 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn7 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn8 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn9 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn10 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn11 * 2.688117142e43), (locals.var_pparam_b4soildeb_dn12 * 2.688117142e43),)
    } else {
        (locals.var_tcen2, locals.var_tcen2_dn3, locals.var_tcen2_dn4, locals.var_tcen2_dn5, locals.var_tcen2_dn6, locals.var_tcen2_dn7, locals.var_tcen2_dn8, locals.var_tcen2_dn9, locals.var_tcen2_dn10, locals.var_tcen2_dn11, locals.var_tcen2_dn12,)
    }
};
        locals.var_tcen2 = assign27910_e25709;
        locals.var_tcen2_dn3 = assign27910_e25709_d_n3;
        locals.var_tcen2_dn4 = assign27910_e25709_d_n4;
        locals.var_tcen2_dn5 = assign27910_e25709_d_n5;
        locals.var_tcen2_dn6 = assign27910_e25709_d_n6;
        locals.var_tcen2_dn7 = assign27910_e25709_d_n7;
        locals.var_tcen2_dn8 = assign27910_e25709_d_n8;
        locals.var_tcen2_dn9 = assign27910_e25709_d_n9;
        locals.var_tcen2_dn10 = assign27910_e25709_d_n10;
        locals.var_tcen2_dn11 = assign27910_e25709_d_n11;
        locals.var_tcen2_dn12 = assign27910_e25709_d_n12;
        locals.var_tcen2_rv = 0.0;

        let (assign27920_e25725, assign27920_e25725_d_n3, assign27920_e25725_d_n4, assign27920_e25725_d_n5, assign27920_e25725_d_n6, assign27920_e25725_d_n7, assign27920_e25725_d_n8, assign27920_e25725_d_n9, assign27920_e25725_d_n10, assign27920_e25725_d_n11, assign27920_e25725_d_n12,) = {
    if ((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1396 != 0.0)) {
        let assign27920_e25721: f64 = (locals.var_pparam_b4soildeb - locals.var_tcen2);
        let assign27920_e25723: f64 = (assign27920_e25721 - locals.var_link);
        (assign27920_e25723, ((locals.var_pparam_b4soildeb_dn3 - locals.var_tcen2_dn3) - locals.var_link_dn3), ((locals.var_pparam_b4soildeb_dn4 - locals.var_tcen2_dn4) - locals.var_link_dn4), ((locals.var_pparam_b4soildeb_dn5 - locals.var_tcen2_dn5) - locals.var_link_dn5), ((locals.var_pparam_b4soildeb_dn6 - locals.var_tcen2_dn6) - locals.var_link_dn6), ((locals.var_pparam_b4soildeb_dn7 - locals.var_tcen2_dn7) - locals.var_link_dn7), ((locals.var_pparam_b4soildeb_dn8 - locals.var_tcen2_dn8) - locals.var_link_dn8), ((locals.var_pparam_b4soildeb_dn9 - locals.var_tcen2_dn9) - locals.var_link_dn9), ((locals.var_pparam_b4soildeb_dn10 - locals.var_tcen2_dn10) - locals.var_link_dn10), ((locals.var_pparam_b4soildeb_dn11 - locals.var_tcen2_dn11) - locals.var_link_dn11), ((locals.var_pparam_b4soildeb_dn12 - locals.var_tcen2_dn12) - locals.var_link_dn12),)
    } else {
        (locals.var_v3, locals.var_v3_dn3, locals.var_v3_dn4, locals.var_v3_dn5, locals.var_v3_dn6, locals.var_v3_dn7, locals.var_v3_dn8, locals.var_v3_dn9, locals.var_v3_dn10, locals.var_v3_dn11, locals.var_v3_dn12,)
    }
};
        locals.var_v3 = assign27920_e25725;
        locals.var_v3_dn3 = assign27920_e25725_d_n3;
        locals.var_v3_dn4 = assign27920_e25725_d_n4;
        locals.var_v3_dn5 = assign27920_e25725_d_n5;
        locals.var_v3_dn6 = assign27920_e25725_d_n6;
        locals.var_v3_dn7 = assign27920_e25725_d_n7;
        locals.var_v3_dn8 = assign27920_e25725_d_n8;
        locals.var_v3_dn9 = assign27920_e25725_d_n9;
        locals.var_v3_dn10 = assign27920_e25725_d_n10;
        locals.var_v3_dn11 = assign27920_e25725_d_n11;
        locals.var_v3_dn12 = assign27920_e25725_d_n12;
        locals.var_v3_rv = 0.0;

        let (assign27930_e25746, assign27930_e25746_d_n3, assign27930_e25746_d_n4, assign27930_e25746_d_n5, assign27930_e25746_d_n6, assign27930_e25746_d_n7, assign27930_e25746_d_n8, assign27930_e25746_d_n9, assign27930_e25746_d_n10, assign27930_e25746_d_n11, assign27930_e25746_d_n12,) = {
    if ((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1396 != 0.0)) {
        let assign27930_e25737: f64 = (locals.var_v3 * locals.var_v3);
        let assign27930_e25740: f64 = (4.0 * locals.var_link);
        let assign27930_e25742: f64 = (assign27930_e25740 * locals.var_pparam_b4soildeb);
        let assign27930_e25743: f64 = (assign27930_e25737 + assign27930_e25742);
        let assign27930_e25744: f64 = (assign27930_e25743).sqrt();
        (assign27930_e25744, ((((locals.var_v3_dn3 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn3)) + (((4.0 * locals.var_link_dn3) * locals.var_pparam_b4soildeb) + (assign27930_e25740 * locals.var_pparam_b4soildeb_dn3))) / (2.0 * assign27930_e25744)), ((((locals.var_v3_dn4 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn4)) + (((4.0 * locals.var_link_dn4) * locals.var_pparam_b4soildeb) + (assign27930_e25740 * locals.var_pparam_b4soildeb_dn4))) / (2.0 * assign27930_e25744)), ((((locals.var_v3_dn5 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn5)) + (((4.0 * locals.var_link_dn5) * locals.var_pparam_b4soildeb) + (assign27930_e25740 * locals.var_pparam_b4soildeb_dn5))) / (2.0 * assign27930_e25744)), ((((locals.var_v3_dn6 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn6)) + (((4.0 * locals.var_link_dn6) * locals.var_pparam_b4soildeb) + (assign27930_e25740 * locals.var_pparam_b4soildeb_dn6))) / (2.0 * assign27930_e25744)), ((((locals.var_v3_dn7 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn7)) + (((4.0 * locals.var_link_dn7) * locals.var_pparam_b4soildeb) + (assign27930_e25740 * locals.var_pparam_b4soildeb_dn7))) / (2.0 * assign27930_e25744)), ((((locals.var_v3_dn8 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn8)) + (((4.0 * locals.var_link_dn8) * locals.var_pparam_b4soildeb) + (assign27930_e25740 * locals.var_pparam_b4soildeb_dn8))) / (2.0 * assign27930_e25744)), ((((locals.var_v3_dn9 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn9)) + (((4.0 * locals.var_link_dn9) * locals.var_pparam_b4soildeb) + (assign27930_e25740 * locals.var_pparam_b4soildeb_dn9))) / (2.0 * assign27930_e25744)), ((((locals.var_v3_dn10 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn10)) + (((4.0 * locals.var_link_dn10) * locals.var_pparam_b4soildeb) + (assign27930_e25740 * locals.var_pparam_b4soildeb_dn10))) / (2.0 * assign27930_e25744)), ((((locals.var_v3_dn11 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn11)) + (((4.0 * locals.var_link_dn11) * locals.var_pparam_b4soildeb) + (assign27930_e25740 * locals.var_pparam_b4soildeb_dn11))) / (2.0 * assign27930_e25744)), ((((locals.var_v3_dn12 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn12)) + (((4.0 * locals.var_link_dn12) * locals.var_pparam_b4soildeb) + (assign27930_e25740 * locals.var_pparam_b4soildeb_dn12))) / (2.0 * assign27930_e25744)),)
    } else {
        (locals.var_v4, locals.var_v4_dn3, locals.var_v4_dn4, locals.var_v4_dn5, locals.var_v4_dn6, locals.var_v4_dn7, locals.var_v4_dn8, locals.var_v4_dn9, locals.var_v4_dn10, locals.var_v4_dn11, locals.var_v4_dn12,)
    }
};
        locals.var_v4 = assign27930_e25746;
        locals.var_v4_dn3 = assign27930_e25746_d_n3;
        locals.var_v4_dn4 = assign27930_e25746_d_n4;
        locals.var_v4_dn5 = assign27930_e25746_d_n5;
        locals.var_v4_dn6 = assign27930_e25746_d_n6;
        locals.var_v4_dn7 = assign27930_e25746_d_n7;
        locals.var_v4_dn8 = assign27930_e25746_d_n8;
        locals.var_v4_dn9 = assign27930_e25746_d_n9;
        locals.var_v4_dn10 = assign27930_e25746_d_n10;
        locals.var_v4_dn11 = assign27930_e25746_d_n11;
        locals.var_v4_dn12 = assign27930_e25746_d_n12;
        locals.var_v4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_85(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign27940_e25764, assign27940_e25764_d_n3, assign27940_e25764_d_n4, assign27940_e25764_d_n5, assign27940_e25764_d_n6, assign27940_e25764_d_n7, assign27940_e25764_d_n8, assign27940_e25764_d_n9, assign27940_e25764_d_n10, assign27940_e25764_d_n11, assign27940_e25764_d_n12,) = {
    if ((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1396 != 0.0)) {
        let assign27940_e25760: f64 = (locals.var_v3 + locals.var_v4);
        let assign27940_e25761: f64 = (0.5 * assign27940_e25760);
        let assign27940_e25762: f64 = (locals.var_pparam_b4soildeb - assign27940_e25761);
        (assign27940_e25762, (locals.var_pparam_b4soildeb_dn3 - (0.5 * (locals.var_v3_dn3 + locals.var_v4_dn3))), (locals.var_pparam_b4soildeb_dn4 - (0.5 * (locals.var_v3_dn4 + locals.var_v4_dn4))), (locals.var_pparam_b4soildeb_dn5 - (0.5 * (locals.var_v3_dn5 + locals.var_v4_dn5))), (locals.var_pparam_b4soildeb_dn6 - (0.5 * (locals.var_v3_dn6 + locals.var_v4_dn6))), (locals.var_pparam_b4soildeb_dn7 - (0.5 * (locals.var_v3_dn7 + locals.var_v4_dn7))), (locals.var_pparam_b4soildeb_dn8 - (0.5 * (locals.var_v3_dn8 + locals.var_v4_dn8))), (locals.var_pparam_b4soildeb_dn9 - (0.5 * (locals.var_v3_dn9 + locals.var_v4_dn9))), (locals.var_pparam_b4soildeb_dn10 - (0.5 * (locals.var_v3_dn10 + locals.var_v4_dn10))), (locals.var_pparam_b4soildeb_dn11 - (0.5 * (locals.var_v3_dn11 + locals.var_v4_dn11))), (locals.var_pparam_b4soildeb_dn12 - (0.5 * (locals.var_v3_dn12 + locals.var_v4_dn12))),)
    } else {
        (locals.var_tcen2, locals.var_tcen2_dn3, locals.var_tcen2_dn4, locals.var_tcen2_dn5, locals.var_tcen2_dn6, locals.var_tcen2_dn7, locals.var_tcen2_dn8, locals.var_tcen2_dn9, locals.var_tcen2_dn10, locals.var_tcen2_dn11, locals.var_tcen2_dn12,)
    }
};
        locals.var_tcen2 = assign27940_e25764;
        locals.var_tcen2_dn3 = assign27940_e25764_d_n3;
        locals.var_tcen2_dn4 = assign27940_e25764_d_n4;
        locals.var_tcen2_dn5 = assign27940_e25764_d_n5;
        locals.var_tcen2_dn6 = assign27940_e25764_d_n6;
        locals.var_tcen2_dn7 = assign27940_e25764_d_n7;
        locals.var_tcen2_dn8 = assign27940_e25764_d_n8;
        locals.var_tcen2_dn9 = assign27940_e25764_d_n9;
        locals.var_tcen2_dn10 = assign27940_e25764_d_n10;
        locals.var_tcen2_dn11 = assign27940_e25764_d_n11;
        locals.var_tcen2_dn12 = assign27940_e25764_d_n12;
        locals.var_tcen2_rv = 0.0;

        let assign27950_e25767: f64 = if locals.var_tcen2 < 1e-15 { 1.0 } else { 0.0 };
        locals.var_guard1399 = assign27950_e25767;
        locals.var_guard1399_rv = 0.0;

        let (assign27960_e25781, assign27960_e25781_d_n3, assign27960_e25781_d_n4, assign27960_e25781_d_n5, assign27960_e25781_d_n6, assign27960_e25781_d_n7, assign27960_e25781_d_n8, assign27960_e25781_d_n9, assign27960_e25781_d_n10, assign27960_e25781_d_n11, assign27960_e25781_d_n12,) = {
    if (((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1396 != 0.0)) && (locals.var_guard1399 != 0.0)) {
        (1e-15, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tcen2, locals.var_tcen2_dn3, locals.var_tcen2_dn4, locals.var_tcen2_dn5, locals.var_tcen2_dn6, locals.var_tcen2_dn7, locals.var_tcen2_dn8, locals.var_tcen2_dn9, locals.var_tcen2_dn10, locals.var_tcen2_dn11, locals.var_tcen2_dn12,)
    }
};
        locals.var_tcen2 = assign27960_e25781;
        locals.var_tcen2_dn3 = assign27960_e25781_d_n3;
        locals.var_tcen2_dn4 = assign27960_e25781_d_n4;
        locals.var_tcen2_dn5 = assign27960_e25781_d_n5;
        locals.var_tcen2_dn6 = assign27960_e25781_d_n6;
        locals.var_tcen2_dn7 = assign27960_e25781_d_n7;
        locals.var_tcen2_dn8 = assign27960_e25781_d_n8;
        locals.var_tcen2_dn9 = assign27960_e25781_d_n9;
        locals.var_tcen2_dn10 = assign27960_e25781_d_n10;
        locals.var_tcen2_dn11 = assign27960_e25781_d_n11;
        locals.var_tcen2_dn12 = assign27960_e25781_d_n12;
        locals.var_tcen2_rv = 0.0;

        let (assign27970_e25793, assign27970_e25793_d_n3, assign27970_e25793_d_n4, assign27970_e25793_d_n5, assign27970_e25793_d_n6, assign27970_e25793_d_n7, assign27970_e25793_d_n8, assign27970_e25793_d_n9, assign27970_e25793_d_n10, assign27970_e25793_d_n11, assign27970_e25793_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) {
        let assign27970_e25791: f64 = (locals.var_epssub / locals.var_tcen__blk964);
        (assign27970_e25791, (-((locals.var_epssub * locals.var_tcen__blk964_dn3) / (locals.var_tcen__blk964 * locals.var_tcen__blk964))), (-((locals.var_epssub * locals.var_tcen__blk964_dn4) / (locals.var_tcen__blk964 * locals.var_tcen__blk964))), (-((locals.var_epssub * locals.var_tcen__blk964_dn5) / (locals.var_tcen__blk964 * locals.var_tcen__blk964))), (-((locals.var_epssub * locals.var_tcen__blk964_dn6) / (locals.var_tcen__blk964 * locals.var_tcen__blk964))), (-((locals.var_epssub * locals.var_tcen__blk964_dn7) / (locals.var_tcen__blk964 * locals.var_tcen__blk964))), (-((locals.var_epssub * locals.var_tcen__blk964_dn8) / (locals.var_tcen__blk964 * locals.var_tcen__blk964))), (-((locals.var_epssub * locals.var_tcen__blk964_dn9) / (locals.var_tcen__blk964 * locals.var_tcen__blk964))), (-((locals.var_epssub * locals.var_tcen__blk964_dn10) / (locals.var_tcen__blk964 * locals.var_tcen__blk964))), (-((locals.var_epssub * locals.var_tcen__blk964_dn11) / (locals.var_tcen__blk964 * locals.var_tcen__blk964))), (-((locals.var_epssub * locals.var_tcen__blk964_dn12) / (locals.var_tcen__blk964 * locals.var_tcen__blk964))),)
    } else {
        (locals.var_ccen, locals.var_ccen_dn3, locals.var_ccen_dn4, locals.var_ccen_dn5, locals.var_ccen_dn6, locals.var_ccen_dn7, locals.var_ccen_dn8, locals.var_ccen_dn9, locals.var_ccen_dn10, locals.var_ccen_dn11, locals.var_ccen_dn12,)
    }
};
        locals.var_ccen = assign27970_e25793;
        locals.var_ccen_dn3 = assign27970_e25793_d_n3;
        locals.var_ccen_dn4 = assign27970_e25793_d_n4;
        locals.var_ccen_dn5 = assign27970_e25793_d_n5;
        locals.var_ccen_dn6 = assign27970_e25793_d_n6;
        locals.var_ccen_dn7 = assign27970_e25793_d_n7;
        locals.var_ccen_dn8 = assign27970_e25793_d_n8;
        locals.var_ccen_dn9 = assign27970_e25793_d_n9;
        locals.var_ccen_dn10 = assign27970_e25793_d_n10;
        locals.var_ccen_dn11 = assign27970_e25793_d_n11;
        locals.var_ccen_dn12 = assign27970_e25793_d_n12;
        locals.var_ccen_rv = 0.0;

        let (assign27980_e25807, assign27980_e25807_d_n3, assign27980_e25807_d_n4, assign27980_e25807_d_n5, assign27980_e25807_d_n6, assign27980_e25807_d_n7, assign27980_e25807_d_n8, assign27980_e25807_d_n9, assign27980_e25807_d_n10, assign27980_e25807_d_n11, assign27980_e25807_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) {
        let assign27980_e25804: f64 = (locals.var_cox + locals.var_ccen);
        let assign27980_e25805: f64 = (locals.var_cox / assign27980_e25804);
        (assign27980_e25805, (((locals.var_cox_dn3 * assign27980_e25804) - (locals.var_cox * (locals.var_cox_dn3 + locals.var_ccen_dn3))) / (assign27980_e25804 * assign27980_e25804)), (((locals.var_cox_dn4 * assign27980_e25804) - (locals.var_cox * (locals.var_cox_dn4 + locals.var_ccen_dn4))) / (assign27980_e25804 * assign27980_e25804)), (((locals.var_cox_dn5 * assign27980_e25804) - (locals.var_cox * (locals.var_cox_dn5 + locals.var_ccen_dn5))) / (assign27980_e25804 * assign27980_e25804)), (((locals.var_cox_dn6 * assign27980_e25804) - (locals.var_cox * (locals.var_cox_dn6 + locals.var_ccen_dn6))) / (assign27980_e25804 * assign27980_e25804)), (((locals.var_cox_dn7 * assign27980_e25804) - (locals.var_cox * (locals.var_cox_dn7 + locals.var_ccen_dn7))) / (assign27980_e25804 * assign27980_e25804)), (((locals.var_cox_dn8 * assign27980_e25804) - (locals.var_cox * (locals.var_cox_dn8 + locals.var_ccen_dn8))) / (assign27980_e25804 * assign27980_e25804)), (((locals.var_cox_dn9 * assign27980_e25804) - (locals.var_cox * (locals.var_cox_dn9 + locals.var_ccen_dn9))) / (assign27980_e25804 * assign27980_e25804)), (((locals.var_cox_dn10 * assign27980_e25804) - (locals.var_cox * (locals.var_cox_dn10 + locals.var_ccen_dn10))) / (assign27980_e25804 * assign27980_e25804)), (((locals.var_cox_dn11 * assign27980_e25804) - (locals.var_cox * (locals.var_cox_dn11 + locals.var_ccen_dn11))) / (assign27980_e25804 * assign27980_e25804)), (((locals.var_cox_dn12 * assign27980_e25804) - (locals.var_cox * (locals.var_cox_dn12 + locals.var_ccen_dn12))) / (assign27980_e25804 * assign27980_e25804)),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign27980_e25807;
        locals.var_t2__blk810_dn3 = assign27980_e25807_d_n3;
        locals.var_t2__blk810_dn4 = assign27980_e25807_d_n4;
        locals.var_t2__blk810_dn5 = assign27980_e25807_d_n5;
        locals.var_t2__blk810_dn6 = assign27980_e25807_d_n6;
        locals.var_t2__blk810_dn7 = assign27980_e25807_d_n7;
        locals.var_t2__blk810_dn8 = assign27980_e25807_d_n8;
        locals.var_t2__blk810_dn9 = assign27980_e25807_d_n9;
        locals.var_t2__blk810_dn10 = assign27980_e25807_d_n10;
        locals.var_t2__blk810_dn11 = assign27980_e25807_d_n11;
        locals.var_t2__blk810_dn12 = assign27980_e25807_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign27990_e25819, assign27990_e25819_d_n3, assign27990_e25819_d_n4, assign27990_e25819_d_n5, assign27990_e25819_d_n6, assign27990_e25819_d_n7, assign27990_e25819_d_n8, assign27990_e25819_d_n9, assign27990_e25819_d_n10, assign27990_e25819_d_n11, assign27990_e25819_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) {
        let assign27990_e25817: f64 = (locals.var_t2__blk810 * locals.var_ccen);
        (assign27990_e25817, ((locals.var_t2__blk810_dn3 * locals.var_ccen) + (locals.var_t2__blk810 * locals.var_ccen_dn3)), ((locals.var_t2__blk810_dn4 * locals.var_ccen) + (locals.var_t2__blk810 * locals.var_ccen_dn4)), ((locals.var_t2__blk810_dn5 * locals.var_ccen) + (locals.var_t2__blk810 * locals.var_ccen_dn5)), ((locals.var_t2__blk810_dn6 * locals.var_ccen) + (locals.var_t2__blk810 * locals.var_ccen_dn6)), ((locals.var_t2__blk810_dn7 * locals.var_ccen) + (locals.var_t2__blk810 * locals.var_ccen_dn7)), ((locals.var_t2__blk810_dn8 * locals.var_ccen) + (locals.var_t2__blk810 * locals.var_ccen_dn8)), ((locals.var_t2__blk810_dn9 * locals.var_ccen) + (locals.var_t2__blk810 * locals.var_ccen_dn9)), ((locals.var_t2__blk810_dn10 * locals.var_ccen) + (locals.var_t2__blk810 * locals.var_ccen_dn10)), ((locals.var_t2__blk810_dn11 * locals.var_ccen) + (locals.var_t2__blk810 * locals.var_ccen_dn11)), ((locals.var_t2__blk810_dn12 * locals.var_ccen) + (locals.var_t2__blk810 * locals.var_ccen_dn12)),)
    } else {
        (locals.var_coxeff, locals.var_coxeff_dn3, locals.var_coxeff_dn4, locals.var_coxeff_dn5, locals.var_coxeff_dn6, locals.var_coxeff_dn7, locals.var_coxeff_dn8, locals.var_coxeff_dn9, locals.var_coxeff_dn10, locals.var_coxeff_dn11, locals.var_coxeff_dn12,)
    }
};
        locals.var_coxeff = assign27990_e25819;
        locals.var_coxeff_dn3 = assign27990_e25819_d_n3;
        locals.var_coxeff_dn4 = assign27990_e25819_d_n4;
        locals.var_coxeff_dn5 = assign27990_e25819_d_n5;
        locals.var_coxeff_dn6 = assign27990_e25819_d_n6;
        locals.var_coxeff_dn7 = assign27990_e25819_d_n7;
        locals.var_coxeff_dn8 = assign27990_e25819_d_n8;
        locals.var_coxeff_dn9 = assign27990_e25819_d_n9;
        locals.var_coxeff_dn10 = assign27990_e25819_d_n10;
        locals.var_coxeff_dn11 = assign27990_e25819_d_n11;
        locals.var_coxeff_dn12 = assign27990_e25819_d_n12;
        locals.var_coxeff_rv = 0.0;

        let assign28000_e25830: f64 = if (((locals.var_b4soisoimod != 2.0) && (locals.var_b4soibodymod != 0.0)) && (p.p27 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1400 = assign28000_e25830;
        locals.var_guard1400_rv = 0.0;

        let (assign28010_e25844, assign28010_e25844_d_n3, assign28010_e25844_d_n4, assign28010_e25844_d_n5, assign28010_e25844_d_n6, assign28010_e25844_d_n7, assign28010_e25844_d_n8, assign28010_e25844_d_n9, assign28010_e25844_d_n10, assign28010_e25844_d_n11, assign28010_e25844_d_n12,) = {
    if ((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1400 != 0.0)) {
        let assign28010_e25842: f64 = (locals.var_epssub / locals.var_tcen2);
        (assign28010_e25842, (-((locals.var_epssub * locals.var_tcen2_dn3) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn4) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn5) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn6) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn7) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn8) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn9) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn10) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn11) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn12) / (locals.var_tcen2 * locals.var_tcen2))),)
    } else {
        (locals.var_ccen2, locals.var_ccen2_dn3, locals.var_ccen2_dn4, locals.var_ccen2_dn5, locals.var_ccen2_dn6, locals.var_ccen2_dn7, locals.var_ccen2_dn8, locals.var_ccen2_dn9, locals.var_ccen2_dn10, locals.var_ccen2_dn11, locals.var_ccen2_dn12,)
    }
};
        locals.var_ccen2 = assign28010_e25844;
        locals.var_ccen2_dn3 = assign28010_e25844_d_n3;
        locals.var_ccen2_dn4 = assign28010_e25844_d_n4;
        locals.var_ccen2_dn5 = assign28010_e25844_d_n5;
        locals.var_ccen2_dn6 = assign28010_e25844_d_n6;
        locals.var_ccen2_dn7 = assign28010_e25844_d_n7;
        locals.var_ccen2_dn8 = assign28010_e25844_d_n8;
        locals.var_ccen2_dn9 = assign28010_e25844_d_n9;
        locals.var_ccen2_dn10 = assign28010_e25844_d_n10;
        locals.var_ccen2_dn11 = assign28010_e25844_d_n11;
        locals.var_ccen2_dn12 = assign28010_e25844_d_n12;
        locals.var_ccen2_rv = 0.0;

        let (assign28020_e25860, assign28020_e25860_d_n3, assign28020_e25860_d_n4, assign28020_e25860_d_n5, assign28020_e25860_d_n6, assign28020_e25860_d_n7, assign28020_e25860_d_n8, assign28020_e25860_d_n9, assign28020_e25860_d_n10, assign28020_e25860_d_n11, assign28020_e25860_d_n12,) = {
    if ((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1400 != 0.0)) {
        let assign28020_e25857: f64 = (locals.var_cox + locals.var_ccen2);
        let assign28020_e25858: f64 = (locals.var_cox / assign28020_e25857);
        (assign28020_e25858, (((locals.var_cox_dn3 * assign28020_e25857) - (locals.var_cox * (locals.var_cox_dn3 + locals.var_ccen2_dn3))) / (assign28020_e25857 * assign28020_e25857)), (((locals.var_cox_dn4 * assign28020_e25857) - (locals.var_cox * (locals.var_cox_dn4 + locals.var_ccen2_dn4))) / (assign28020_e25857 * assign28020_e25857)), (((locals.var_cox_dn5 * assign28020_e25857) - (locals.var_cox * (locals.var_cox_dn5 + locals.var_ccen2_dn5))) / (assign28020_e25857 * assign28020_e25857)), (((locals.var_cox_dn6 * assign28020_e25857) - (locals.var_cox * (locals.var_cox_dn6 + locals.var_ccen2_dn6))) / (assign28020_e25857 * assign28020_e25857)), (((locals.var_cox_dn7 * assign28020_e25857) - (locals.var_cox * (locals.var_cox_dn7 + locals.var_ccen2_dn7))) / (assign28020_e25857 * assign28020_e25857)), (((locals.var_cox_dn8 * assign28020_e25857) - (locals.var_cox * (locals.var_cox_dn8 + locals.var_ccen2_dn8))) / (assign28020_e25857 * assign28020_e25857)), (((locals.var_cox_dn9 * assign28020_e25857) - (locals.var_cox * (locals.var_cox_dn9 + locals.var_ccen2_dn9))) / (assign28020_e25857 * assign28020_e25857)), (((locals.var_cox_dn10 * assign28020_e25857) - (locals.var_cox * (locals.var_cox_dn10 + locals.var_ccen2_dn10))) / (assign28020_e25857 * assign28020_e25857)), (((locals.var_cox_dn11 * assign28020_e25857) - (locals.var_cox * (locals.var_cox_dn11 + locals.var_ccen2_dn11))) / (assign28020_e25857 * assign28020_e25857)), (((locals.var_cox_dn12 * assign28020_e25857) - (locals.var_cox * (locals.var_cox_dn12 + locals.var_ccen2_dn12))) / (assign28020_e25857 * assign28020_e25857)),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign28020_e25860;
        locals.var_t2__blk810_dn3 = assign28020_e25860_d_n3;
        locals.var_t2__blk810_dn4 = assign28020_e25860_d_n4;
        locals.var_t2__blk810_dn5 = assign28020_e25860_d_n5;
        locals.var_t2__blk810_dn6 = assign28020_e25860_d_n6;
        locals.var_t2__blk810_dn7 = assign28020_e25860_d_n7;
        locals.var_t2__blk810_dn8 = assign28020_e25860_d_n8;
        locals.var_t2__blk810_dn9 = assign28020_e25860_d_n9;
        locals.var_t2__blk810_dn10 = assign28020_e25860_d_n10;
        locals.var_t2__blk810_dn11 = assign28020_e25860_d_n11;
        locals.var_t2__blk810_dn12 = assign28020_e25860_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign28030_e25874, assign28030_e25874_d_n3, assign28030_e25874_d_n4, assign28030_e25874_d_n5, assign28030_e25874_d_n6, assign28030_e25874_d_n7, assign28030_e25874_d_n8, assign28030_e25874_d_n9, assign28030_e25874_d_n10, assign28030_e25874_d_n11, assign28030_e25874_d_n12,) = {
    if ((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1400 != 0.0)) {
        let assign28030_e25872: f64 = (locals.var_t2__blk810 * locals.var_ccen2);
        (assign28030_e25872, ((locals.var_t2__blk810_dn3 * locals.var_ccen2) + (locals.var_t2__blk810 * locals.var_ccen2_dn3)), ((locals.var_t2__blk810_dn4 * locals.var_ccen2) + (locals.var_t2__blk810 * locals.var_ccen2_dn4)), ((locals.var_t2__blk810_dn5 * locals.var_ccen2) + (locals.var_t2__blk810 * locals.var_ccen2_dn5)), ((locals.var_t2__blk810_dn6 * locals.var_ccen2) + (locals.var_t2__blk810 * locals.var_ccen2_dn6)), ((locals.var_t2__blk810_dn7 * locals.var_ccen2) + (locals.var_t2__blk810 * locals.var_ccen2_dn7)), ((locals.var_t2__blk810_dn8 * locals.var_ccen2) + (locals.var_t2__blk810 * locals.var_ccen2_dn8)), ((locals.var_t2__blk810_dn9 * locals.var_ccen2) + (locals.var_t2__blk810 * locals.var_ccen2_dn9)), ((locals.var_t2__blk810_dn10 * locals.var_ccen2) + (locals.var_t2__blk810 * locals.var_ccen2_dn10)), ((locals.var_t2__blk810_dn11 * locals.var_ccen2) + (locals.var_t2__blk810 * locals.var_ccen2_dn11)), ((locals.var_t2__blk810_dn12 * locals.var_ccen2) + (locals.var_t2__blk810 * locals.var_ccen2_dn12)),)
    } else {
        (locals.var_coxeff2, locals.var_coxeff2_dn3, locals.var_coxeff2_dn4, locals.var_coxeff2_dn5, locals.var_coxeff2_dn6, locals.var_coxeff2_dn7, locals.var_coxeff2_dn8, locals.var_coxeff2_dn9, locals.var_coxeff2_dn10, locals.var_coxeff2_dn11, locals.var_coxeff2_dn12,)
    }
};
        locals.var_coxeff2 = assign28030_e25874;
        locals.var_coxeff2_dn3 = assign28030_e25874_d_n3;
        locals.var_coxeff2_dn4 = assign28030_e25874_d_n4;
        locals.var_coxeff2_dn5 = assign28030_e25874_d_n5;
        locals.var_coxeff2_dn6 = assign28030_e25874_d_n6;
        locals.var_coxeff2_dn7 = assign28030_e25874_d_n7;
        locals.var_coxeff2_dn8 = assign28030_e25874_d_n8;
        locals.var_coxeff2_dn9 = assign28030_e25874_d_n9;
        locals.var_coxeff2_dn10 = assign28030_e25874_d_n10;
        locals.var_coxeff2_dn11 = assign28030_e25874_d_n11;
        locals.var_coxeff2_dn12 = assign28030_e25874_d_n12;
        locals.var_coxeff2_rv = 0.0;

        let (assign28040_e25888, assign28040_e25888_d_n3, assign28040_e25888_d_n4, assign28040_e25888_d_n5, assign28040_e25888_d_n6, assign28040_e25888_d_n7, assign28040_e25888_d_n8, assign28040_e25888_d_n9, assign28040_e25888_d_n10, assign28040_e25888_d_n11, assign28040_e25888_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) {
        let assign28040_e25884: f64 = (locals.var_coxwlb * locals.var_coxeff);
        let assign28040_e25886: f64 = (assign28040_e25884 / locals.var_cox);
        (assign28040_e25886, (((((locals.var_coxwlb_dn3 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn3)) * locals.var_cox) - (assign28040_e25884 * locals.var_cox_dn3)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn4 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn4)) * locals.var_cox) - (assign28040_e25884 * locals.var_cox_dn4)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn5 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn5)) * locals.var_cox) - (assign28040_e25884 * locals.var_cox_dn5)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn6 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn6)) * locals.var_cox) - (assign28040_e25884 * locals.var_cox_dn6)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn7 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn7)) * locals.var_cox) - (assign28040_e25884 * locals.var_cox_dn7)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn8 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn8)) * locals.var_cox) - (assign28040_e25884 * locals.var_cox_dn8)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn9 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn9)) * locals.var_cox) - (assign28040_e25884 * locals.var_cox_dn9)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn10 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn10)) * locals.var_cox) - (assign28040_e25884 * locals.var_cox_dn10)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn11 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn11)) * locals.var_cox) - (assign28040_e25884 * locals.var_cox_dn11)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn12 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn12)) * locals.var_cox) - (assign28040_e25884 * locals.var_cox_dn12)) / (locals.var_cox * locals.var_cox)),)
    } else {
        (locals.var_coxwlcenb, locals.var_coxwlcenb_dn3, locals.var_coxwlcenb_dn4, locals.var_coxwlcenb_dn5, locals.var_coxwlcenb_dn6, locals.var_coxwlcenb_dn7, locals.var_coxwlcenb_dn8, locals.var_coxwlcenb_dn9, locals.var_coxwlcenb_dn10, locals.var_coxwlcenb_dn11, locals.var_coxwlcenb_dn12,)
    }
};
        locals.var_coxwlcenb = assign28040_e25888;
        locals.var_coxwlcenb_dn3 = assign28040_e25888_d_n3;
        locals.var_coxwlcenb_dn4 = assign28040_e25888_d_n4;
        locals.var_coxwlcenb_dn5 = assign28040_e25888_d_n5;
        locals.var_coxwlcenb_dn6 = assign28040_e25888_d_n6;
        locals.var_coxwlcenb_dn7 = assign28040_e25888_d_n7;
        locals.var_coxwlcenb_dn8 = assign28040_e25888_d_n8;
        locals.var_coxwlcenb_dn9 = assign28040_e25888_d_n9;
        locals.var_coxwlcenb_dn10 = assign28040_e25888_d_n10;
        locals.var_coxwlcenb_dn11 = assign28040_e25888_d_n11;
        locals.var_coxwlcenb_dn12 = assign28040_e25888_d_n12;
        locals.var_coxwlcenb_rv = 0.0;

        let assign28050_e25891: f64 = if p.p27 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1401 = assign28050_e25891;
        locals.var_guard1401_rv = 0.0;

        let (assign28060_e25907, assign28060_e25907_d_n3, assign28060_e25907_d_n4, assign28060_e25907_d_n5, assign28060_e25907_d_n6, assign28060_e25907_d_n7, assign28060_e25907_d_n8, assign28060_e25907_d_n9, assign28060_e25907_d_n10, assign28060_e25907_d_n11, assign28060_e25907_d_n12,) = {
    if ((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1401 != 0.0)) {
        let assign28060_e25903: f64 = (locals.var_coxwlb2 * locals.var_coxeff2);
        let assign28060_e25905: f64 = (assign28060_e25903 / locals.var_cox);
        (assign28060_e25905, (((((locals.var_coxwlb2_dn3 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn3)) * locals.var_cox) - (assign28060_e25903 * locals.var_cox_dn3)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn4 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn4)) * locals.var_cox) - (assign28060_e25903 * locals.var_cox_dn4)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn5 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn5)) * locals.var_cox) - (assign28060_e25903 * locals.var_cox_dn5)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn6 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn6)) * locals.var_cox) - (assign28060_e25903 * locals.var_cox_dn6)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn7 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn7)) * locals.var_cox) - (assign28060_e25903 * locals.var_cox_dn7)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn8 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn8)) * locals.var_cox) - (assign28060_e25903 * locals.var_cox_dn8)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn9 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn9)) * locals.var_cox) - (assign28060_e25903 * locals.var_cox_dn9)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn10 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn10)) * locals.var_cox) - (assign28060_e25903 * locals.var_cox_dn10)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn11 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn11)) * locals.var_cox) - (assign28060_e25903 * locals.var_cox_dn11)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn12 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn12)) * locals.var_cox) - (assign28060_e25903 * locals.var_cox_dn12)) / (locals.var_cox * locals.var_cox)),)
    } else {
        (locals.var_coxwlcenb2, locals.var_coxwlcenb2_dn3, locals.var_coxwlcenb2_dn4, locals.var_coxwlcenb2_dn5, locals.var_coxwlcenb2_dn6, locals.var_coxwlcenb2_dn7, locals.var_coxwlcenb2_dn8, locals.var_coxwlcenb2_dn9, locals.var_coxwlcenb2_dn10, locals.var_coxwlcenb2_dn11, locals.var_coxwlcenb2_dn12,)
    }
};
        locals.var_coxwlcenb2 = assign28060_e25907;
        locals.var_coxwlcenb2_dn3 = assign28060_e25907_d_n3;
        locals.var_coxwlcenb2_dn4 = assign28060_e25907_d_n4;
        locals.var_coxwlcenb2_dn5 = assign28060_e25907_d_n5;
        locals.var_coxwlcenb2_dn6 = assign28060_e25907_d_n6;
        locals.var_coxwlcenb2_dn7 = assign28060_e25907_d_n7;
        locals.var_coxwlcenb2_dn8 = assign28060_e25907_d_n8;
        locals.var_coxwlcenb2_dn9 = assign28060_e25907_d_n9;
        locals.var_coxwlcenb2_dn10 = assign28060_e25907_d_n10;
        locals.var_coxwlcenb2_dn11 = assign28060_e25907_d_n11;
        locals.var_coxwlcenb2_dn12 = assign28060_e25907_d_n12;
        locals.var_coxwlcenb2_rv = 0.0;

        let (assign28070_e25921, assign28070_e25921_d_n3, assign28070_e25921_d_n4, assign28070_e25921_d_n5, assign28070_e25921_d_n6, assign28070_e25921_d_n7, assign28070_e25921_d_n8, assign28070_e25921_d_n9, assign28070_e25921_d_n10, assign28070_e25921_d_n11, assign28070_e25921_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) {
        let assign28070_e25918: f64 = (locals.var_vfbeff - locals.var_vfbzb);
        let assign28070_e25919: f64 = (locals.var_coxwlcenb * assign28070_e25918);
        (assign28070_e25919, ((locals.var_coxwlcenb_dn3 * assign28070_e25918) + (locals.var_coxwlcenb * (locals.var_vfbeff_dn3 - locals.var_vfbzb_dn3))), ((locals.var_coxwlcenb_dn4 * assign28070_e25918) + (locals.var_coxwlcenb * (locals.var_vfbeff_dn4 - locals.var_vfbzb_dn4))), ((locals.var_coxwlcenb_dn5 * assign28070_e25918) + (locals.var_coxwlcenb * (locals.var_vfbeff_dn5 - locals.var_vfbzb_dn5))), ((locals.var_coxwlcenb_dn6 * assign28070_e25918) + (locals.var_coxwlcenb * (locals.var_vfbeff_dn6 - locals.var_vfbzb_dn6))), ((locals.var_coxwlcenb_dn7 * assign28070_e25918) + (locals.var_coxwlcenb * (locals.var_vfbeff_dn7 - locals.var_vfbzb_dn7))), ((locals.var_coxwlcenb_dn8 * assign28070_e25918) + (locals.var_coxwlcenb * (locals.var_vfbeff_dn8 - locals.var_vfbzb_dn8))), ((locals.var_coxwlcenb_dn9 * assign28070_e25918) + (locals.var_coxwlcenb * (locals.var_vfbeff_dn9 - locals.var_vfbzb_dn9))), ((locals.var_coxwlcenb_dn10 * assign28070_e25918) + (locals.var_coxwlcenb * (locals.var_vfbeff_dn10 - locals.var_vfbzb_dn10))), ((locals.var_coxwlcenb_dn11 * assign28070_e25918) + (locals.var_coxwlcenb * (locals.var_vfbeff_dn11 - locals.var_vfbzb_dn11))), ((locals.var_coxwlcenb_dn12 * assign28070_e25918) + (locals.var_coxwlcenb * (locals.var_vfbeff_dn12 - locals.var_vfbzb_dn12))),)
    } else {
        (locals.var_qac0, locals.var_qac0_dn3, locals.var_qac0_dn4, locals.var_qac0_dn5, locals.var_qac0_dn6, locals.var_qac0_dn7, locals.var_qac0_dn8, locals.var_qac0_dn9, locals.var_qac0_dn10, locals.var_qac0_dn11, locals.var_qac0_dn12,)
    }
};
        locals.var_qac0 = assign28070_e25921;
        locals.var_qac0_dn3 = assign28070_e25921_d_n3;
        locals.var_qac0_dn4 = assign28070_e25921_d_n4;
        locals.var_qac0_dn5 = assign28070_e25921_d_n5;
        locals.var_qac0_dn6 = assign28070_e25921_d_n6;
        locals.var_qac0_dn7 = assign28070_e25921_d_n7;
        locals.var_qac0_dn8 = assign28070_e25921_d_n8;
        locals.var_qac0_dn9 = assign28070_e25921_d_n9;
        locals.var_qac0_dn10 = assign28070_e25921_d_n10;
        locals.var_qac0_dn11 = assign28070_e25921_d_n11;
        locals.var_qac0_dn12 = assign28070_e25921_d_n12;
        locals.var_qac0_rv = 0.0;

        let assign28080_e25932: f64 = if (((locals.var_b4soisoimod != 2.0) && (locals.var_b4soibodymod != 0.0)) && (p.p27 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1402 = assign28080_e25932;
        locals.var_guard1402_rv = 0.0;

        let (assign28090_e25948, assign28090_e25948_d_n3, assign28090_e25948_d_n4, assign28090_e25948_d_n5, assign28090_e25948_d_n6, assign28090_e25948_d_n7, assign28090_e25948_d_n8, assign28090_e25948_d_n9, assign28090_e25948_d_n10, assign28090_e25948_d_n11, assign28090_e25948_d_n12,) = {
    if ((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1402 != 0.0)) {
        let assign28090_e25945: f64 = (locals.var_vfbeff2 - locals.var_vfbzb2);
        let assign28090_e25946: f64 = (locals.var_coxwlcenb2 * assign28090_e25945);
        (assign28090_e25946, ((locals.var_coxwlcenb2_dn3 * assign28090_e25945) + (locals.var_coxwlcenb2 * (locals.var_vfbeff2_dn3 - locals.var_vfbzb2_dn3))), ((locals.var_coxwlcenb2_dn4 * assign28090_e25945) + (locals.var_coxwlcenb2 * (locals.var_vfbeff2_dn4 - locals.var_vfbzb2_dn4))), ((locals.var_coxwlcenb2_dn5 * assign28090_e25945) + (locals.var_coxwlcenb2 * (locals.var_vfbeff2_dn5 - locals.var_vfbzb2_dn5))), ((locals.var_coxwlcenb2_dn6 * assign28090_e25945) + (locals.var_coxwlcenb2 * (locals.var_vfbeff2_dn6 - locals.var_vfbzb2_dn6))), ((locals.var_coxwlcenb2_dn7 * assign28090_e25945) + (locals.var_coxwlcenb2 * (locals.var_vfbeff2_dn7 - locals.var_vfbzb2_dn7))), ((locals.var_coxwlcenb2_dn8 * assign28090_e25945) + (locals.var_coxwlcenb2 * (locals.var_vfbeff2_dn8 - locals.var_vfbzb2_dn8))), ((locals.var_coxwlcenb2_dn9 * assign28090_e25945) + (locals.var_coxwlcenb2 * (locals.var_vfbeff2_dn9 - locals.var_vfbzb2_dn9))), ((locals.var_coxwlcenb2_dn10 * assign28090_e25945) + (locals.var_coxwlcenb2 * (locals.var_vfbeff2_dn10 - locals.var_vfbzb2_dn10))), ((locals.var_coxwlcenb2_dn11 * assign28090_e25945) + (locals.var_coxwlcenb2 * (locals.var_vfbeff2_dn11 - locals.var_vfbzb2_dn11))), ((locals.var_coxwlcenb2_dn12 * assign28090_e25945) + (locals.var_coxwlcenb2 * (locals.var_vfbeff2_dn12 - locals.var_vfbzb2_dn12))),)
    } else {
        (locals.var_qac02, locals.var_qac02_dn3, locals.var_qac02_dn4, locals.var_qac02_dn5, locals.var_qac02_dn6, locals.var_qac02_dn7, locals.var_qac02_dn8, locals.var_qac02_dn9, locals.var_qac02_dn10, locals.var_qac02_dn11, locals.var_qac02_dn12,)
    }
};
        locals.var_qac02 = assign28090_e25948;
        locals.var_qac02_dn3 = assign28090_e25948_d_n3;
        locals.var_qac02_dn4 = assign28090_e25948_d_n4;
        locals.var_qac02_dn5 = assign28090_e25948_d_n5;
        locals.var_qac02_dn6 = assign28090_e25948_d_n6;
        locals.var_qac02_dn7 = assign28090_e25948_d_n7;
        locals.var_qac02_dn8 = assign28090_e25948_d_n8;
        locals.var_qac02_dn9 = assign28090_e25948_d_n9;
        locals.var_qac02_dn10 = assign28090_e25948_d_n10;
        locals.var_qac02_dn11 = assign28090_e25948_d_n11;
        locals.var_qac02_dn12 = assign28090_e25948_d_n12;
        locals.var_qac02_rv = 0.0;

        let (assign28100_e25962, assign28100_e25962_d_n3, assign28100_e25962_d_n4, assign28100_e25962_d_n5, assign28100_e25962_d_n6, assign28100_e25962_d_n7, assign28100_e25962_d_n8, assign28100_e25962_d_n9, assign28100_e25962_d_n10, assign28100_e25962_d_n11, assign28100_e25962_d_n12,) = {
    if ((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1402 != 0.0)) {
        let assign28100_e25960: f64 = (locals.var_qac0 + locals.var_qac02);
        (assign28100_e25960, (locals.var_qac0_dn3 + locals.var_qac02_dn3), (locals.var_qac0_dn4 + locals.var_qac02_dn4), (locals.var_qac0_dn5 + locals.var_qac02_dn5), (locals.var_qac0_dn6 + locals.var_qac02_dn6), (locals.var_qac0_dn7 + locals.var_qac02_dn7), (locals.var_qac0_dn8 + locals.var_qac02_dn8), (locals.var_qac0_dn9 + locals.var_qac02_dn9), (locals.var_qac0_dn10 + locals.var_qac02_dn10), (locals.var_qac0_dn11 + locals.var_qac02_dn11), (locals.var_qac0_dn12 + locals.var_qac02_dn12),)
    } else {
        (locals.var_qac0, locals.var_qac0_dn3, locals.var_qac0_dn4, locals.var_qac0_dn5, locals.var_qac0_dn6, locals.var_qac0_dn7, locals.var_qac0_dn8, locals.var_qac0_dn9, locals.var_qac0_dn10, locals.var_qac0_dn11, locals.var_qac0_dn12,)
    }
};
        locals.var_qac0 = assign28100_e25962;
        locals.var_qac0_dn3 = assign28100_e25962_d_n3;
        locals.var_qac0_dn4 = assign28100_e25962_d_n4;
        locals.var_qac0_dn5 = assign28100_e25962_d_n5;
        locals.var_qac0_dn6 = assign28100_e25962_d_n6;
        locals.var_qac0_dn7 = assign28100_e25962_d_n7;
        locals.var_qac0_dn8 = assign28100_e25962_d_n8;
        locals.var_qac0_dn9 = assign28100_e25962_d_n9;
        locals.var_qac0_dn10 = assign28100_e25962_d_n10;
        locals.var_qac0_dn11 = assign28100_e25962_d_n11;
        locals.var_qac0_dn12 = assign28100_e25962_d_n12;
        locals.var_qac0_rv = 0.0;

        let (assign28110_e25974, assign28110_e25974_d_n3, assign28110_e25974_d_n4, assign28110_e25974_d_n5, assign28110_e25974_d_n6, assign28110_e25974_d_n7, assign28110_e25974_d_n8, assign28110_e25974_d_n9, assign28110_e25974_d_n10, assign28110_e25974_d_n11, assign28110_e25974_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) {
        let assign28110_e25972: f64 = (0.5 * locals.var_here_b4soik1ox);
        (assign28110_e25972, (0.5 * locals.var_here_b4soik1ox_dn3), (0.5 * locals.var_here_b4soik1ox_dn4), (0.5 * locals.var_here_b4soik1ox_dn5), (0.5 * locals.var_here_b4soik1ox_dn6), (0.5 * locals.var_here_b4soik1ox_dn7), (0.5 * locals.var_here_b4soik1ox_dn8), (0.5 * locals.var_here_b4soik1ox_dn9), (0.5 * locals.var_here_b4soik1ox_dn10), (0.5 * locals.var_here_b4soik1ox_dn11), (0.5 * locals.var_here_b4soik1ox_dn12),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign28110_e25974;
        locals.var_t0__blk808_dn3 = assign28110_e25974_d_n3;
        locals.var_t0__blk808_dn4 = assign28110_e25974_d_n4;
        locals.var_t0__blk808_dn5 = assign28110_e25974_d_n5;
        locals.var_t0__blk808_dn6 = assign28110_e25974_d_n6;
        locals.var_t0__blk808_dn7 = assign28110_e25974_d_n7;
        locals.var_t0__blk808_dn8 = assign28110_e25974_d_n8;
        locals.var_t0__blk808_dn9 = assign28110_e25974_d_n9;
        locals.var_t0__blk808_dn10 = assign28110_e25974_d_n10;
        locals.var_t0__blk808_dn11 = assign28110_e25974_d_n11;
        locals.var_t0__blk808_dn12 = assign28110_e25974_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign28120_e25990, assign28120_e25990_d_n3, assign28120_e25990_d_n4, assign28120_e25990_d_n5, assign28120_e25990_d_n6, assign28120_e25990_d_n7, assign28120_e25990_d_n8, assign28120_e25990_d_n9, assign28120_e25990_d_n10, assign28120_e25990_d_n11, assign28120_e25990_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) {
        let assign28120_e25984: f64 = (locals.var_vgs_eff__blk790 - locals.var_vfbeff);
        let assign28120_e25986: f64 = (assign28120_e25984 - locals.var_vbseff);
        let assign28120_e25988: f64 = (assign28120_e25986 - locals.var_vgsteff__blk840);
        (assign28120_e25988, (((locals.var_vgs_eff__blk790_dn3 - locals.var_vfbeff_dn3) - locals.var_vbseff_dn3) - locals.var_vgsteff__blk840_dn3), (((locals.var_vgs_eff__blk790_dn4 - locals.var_vfbeff_dn4) - locals.var_vbseff_dn4) - locals.var_vgsteff__blk840_dn4), (((locals.var_vgs_eff__blk790_dn5 - locals.var_vfbeff_dn5) - locals.var_vbseff_dn5) - locals.var_vgsteff__blk840_dn5), (((locals.var_vgs_eff__blk790_dn6 - locals.var_vfbeff_dn6) - locals.var_vbseff_dn6) - locals.var_vgsteff__blk840_dn6), (((locals.var_vgs_eff__blk790_dn7 - locals.var_vfbeff_dn7) - locals.var_vbseff_dn7) - locals.var_vgsteff__blk840_dn7), (((locals.var_vgs_eff__blk790_dn8 - locals.var_vfbeff_dn8) - locals.var_vbseff_dn8) - locals.var_vgsteff__blk840_dn8), (((locals.var_vgs_eff__blk790_dn9 - locals.var_vfbeff_dn9) - locals.var_vbseff_dn9) - locals.var_vgsteff__blk840_dn9), (((locals.var_vgs_eff__blk790_dn10 - locals.var_vfbeff_dn10) - locals.var_vbseff_dn10) - locals.var_vgsteff__blk840_dn10), (((locals.var_vgs_eff__blk790_dn11 - locals.var_vfbeff_dn11) - locals.var_vbseff_dn11) - locals.var_vgsteff__blk840_dn11), (((locals.var_vgs_eff__blk790_dn12 - locals.var_vfbeff_dn12) - locals.var_vbseff_dn12) - locals.var_vgsteff__blk840_dn12),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign28120_e25990;
        locals.var_t3__blk811_dn3 = assign28120_e25990_d_n3;
        locals.var_t3__blk811_dn4 = assign28120_e25990_d_n4;
        locals.var_t3__blk811_dn5 = assign28120_e25990_d_n5;
        locals.var_t3__blk811_dn6 = assign28120_e25990_d_n6;
        locals.var_t3__blk811_dn7 = assign28120_e25990_d_n7;
        locals.var_t3__blk811_dn8 = assign28120_e25990_d_n8;
        locals.var_t3__blk811_dn9 = assign28120_e25990_d_n9;
        locals.var_t3__blk811_dn10 = assign28120_e25990_d_n10;
        locals.var_t3__blk811_dn11 = assign28120_e25990_d_n11;
        locals.var_t3__blk811_dn12 = assign28120_e25990_d_n12;
        locals.var_t3__blk811_rv = 0.0;

        let assign28130_e25993: f64 = if locals.var_here_b4soik1ox == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1403 = assign28130_e25993;
        locals.var_guard1403_rv = 0.0;

        let (assign28140_e26005, assign28140_e26005_d_n3, assign28140_e26005_d_n4, assign28140_e26005_d_n5, assign28140_e26005_d_n6, assign28140_e26005_d_n7, assign28140_e26005_d_n8, assign28140_e26005_d_n9, assign28140_e26005_d_n10, assign28140_e26005_d_n11, assign28140_e26005_d_n12,) = {
    if ((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1403 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign28140_e26005;
        locals.var_t1__blk809_dn3 = assign28140_e26005_d_n3;
        locals.var_t1__blk809_dn4 = assign28140_e26005_d_n4;
        locals.var_t1__blk809_dn5 = assign28140_e26005_d_n5;
        locals.var_t1__blk809_dn6 = assign28140_e26005_d_n6;
        locals.var_t1__blk809_dn7 = assign28140_e26005_d_n7;
        locals.var_t1__blk809_dn8 = assign28140_e26005_d_n8;
        locals.var_t1__blk809_dn9 = assign28140_e26005_d_n9;
        locals.var_t1__blk809_dn10 = assign28140_e26005_d_n10;
        locals.var_t1__blk809_dn11 = assign28140_e26005_d_n11;
        locals.var_t1__blk809_dn12 = assign28140_e26005_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let assign28150_e26008: f64 = if locals.var_t3__blk811 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1404 = assign28150_e26008;
        locals.var_guard1404_rv = 0.0;

        let (assign28160_e26027, assign28160_e26027_d_n3, assign28160_e26027_d_n4, assign28160_e26027_d_n5, assign28160_e26027_d_n6, assign28160_e26027_d_n7, assign28160_e26027_d_n8, assign28160_e26027_d_n9, assign28160_e26027_d_n10, assign28160_e26027_d_n11, assign28160_e26027_d_n12,) = {
    if (((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1403 == 0.0)) && (locals.var_guard1404 != 0.0)) {
        let assign28160_e26024: f64 = (locals.var_t3__blk811 / locals.var_here_b4soik1ox);
        let assign28160_e26025: f64 = (locals.var_t0__blk808 + assign28160_e26024);
        (assign28160_e26025, (locals.var_t0__blk808_dn3 + (((locals.var_t3__blk811_dn3 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn3)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn4 + (((locals.var_t3__blk811_dn4 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn4)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn5 + (((locals.var_t3__blk811_dn5 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn5)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn6 + (((locals.var_t3__blk811_dn6 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn6)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn7 + (((locals.var_t3__blk811_dn7 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn7)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn8 + (((locals.var_t3__blk811_dn8 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn8)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn9 + (((locals.var_t3__blk811_dn9 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn9)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn10 + (((locals.var_t3__blk811_dn10 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn10)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn11 + (((locals.var_t3__blk811_dn11 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn11)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn12 + (((locals.var_t3__blk811_dn12 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn12)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign28160_e26027;
        locals.var_t1__blk809_dn3 = assign28160_e26027_d_n3;
        locals.var_t1__blk809_dn4 = assign28160_e26027_d_n4;
        locals.var_t1__blk809_dn5 = assign28160_e26027_d_n5;
        locals.var_t1__blk809_dn6 = assign28160_e26027_d_n6;
        locals.var_t1__blk809_dn7 = assign28160_e26027_d_n7;
        locals.var_t1__blk809_dn8 = assign28160_e26027_d_n8;
        locals.var_t1__blk809_dn9 = assign28160_e26027_d_n9;
        locals.var_t1__blk809_dn10 = assign28160_e26027_d_n10;
        locals.var_t1__blk809_dn11 = assign28160_e26027_d_n11;
        locals.var_t1__blk809_dn12 = assign28160_e26027_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign28170_e26048, assign28170_e26048_d_n3, assign28170_e26048_d_n4, assign28170_e26048_d_n5, assign28170_e26048_d_n6, assign28170_e26048_d_n7, assign28170_e26048_d_n8, assign28170_e26048_d_n9, assign28170_e26048_d_n10, assign28170_e26048_d_n11, assign28170_e26048_d_n12,) = {
    if (((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1403 == 0.0)) && (locals.var_guard1404 == 0.0)) {
        let assign28170_e26043: f64 = (locals.var_t0__blk808 * locals.var_t0__blk808);
        let assign28170_e26045: f64 = (assign28170_e26043 + locals.var_t3__blk811);
        let assign28170_e26046: f64 = (assign28170_e26045).sqrt();
        (assign28170_e26046, ((((locals.var_t0__blk808_dn3 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn3)) + locals.var_t3__blk811_dn3) / (2.0 * assign28170_e26046)), ((((locals.var_t0__blk808_dn4 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn4)) + locals.var_t3__blk811_dn4) / (2.0 * assign28170_e26046)), ((((locals.var_t0__blk808_dn5 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn5)) + locals.var_t3__blk811_dn5) / (2.0 * assign28170_e26046)), ((((locals.var_t0__blk808_dn6 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn6)) + locals.var_t3__blk811_dn6) / (2.0 * assign28170_e26046)), ((((locals.var_t0__blk808_dn7 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn7)) + locals.var_t3__blk811_dn7) / (2.0 * assign28170_e26046)), ((((locals.var_t0__blk808_dn8 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn8)) + locals.var_t3__blk811_dn8) / (2.0 * assign28170_e26046)), ((((locals.var_t0__blk808_dn9 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn9)) + locals.var_t3__blk811_dn9) / (2.0 * assign28170_e26046)), ((((locals.var_t0__blk808_dn10 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn10)) + locals.var_t3__blk811_dn10) / (2.0 * assign28170_e26046)), ((((locals.var_t0__blk808_dn11 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn11)) + locals.var_t3__blk811_dn11) / (2.0 * assign28170_e26046)), ((((locals.var_t0__blk808_dn12 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn12)) + locals.var_t3__blk811_dn12) / (2.0 * assign28170_e26046)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign28170_e26048;
        locals.var_t1__blk809_dn3 = assign28170_e26048_d_n3;
        locals.var_t1__blk809_dn4 = assign28170_e26048_d_n4;
        locals.var_t1__blk809_dn5 = assign28170_e26048_d_n5;
        locals.var_t1__blk809_dn6 = assign28170_e26048_d_n6;
        locals.var_t1__blk809_dn7 = assign28170_e26048_d_n7;
        locals.var_t1__blk809_dn8 = assign28170_e26048_d_n8;
        locals.var_t1__blk809_dn9 = assign28170_e26048_d_n9;
        locals.var_t1__blk809_dn10 = assign28170_e26048_d_n10;
        locals.var_t1__blk809_dn11 = assign28170_e26048_d_n11;
        locals.var_t1__blk809_dn12 = assign28170_e26048_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign28180_e26064, assign28180_e26064_d_n3, assign28180_e26064_d_n4, assign28180_e26064_d_n5, assign28180_e26064_d_n6, assign28180_e26064_d_n7, assign28180_e26064_d_n8, assign28180_e26064_d_n9, assign28180_e26064_d_n10, assign28180_e26064_d_n11, assign28180_e26064_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) {
        let assign28180_e26058: f64 = (locals.var_coxwlcenb * locals.var_here_b4soik1ox);
        let assign28180_e26061: f64 = (locals.var_t1__blk809 - locals.var_t0__blk808);
        let assign28180_e26062: f64 = (assign28180_e26058 * assign28180_e26061);
        (assign28180_e26062, ((((locals.var_coxwlcenb_dn3 * locals.var_here_b4soik1ox) + (locals.var_coxwlcenb * locals.var_here_b4soik1ox_dn3)) * assign28180_e26061) + (assign28180_e26058 * (locals.var_t1__blk809_dn3 - locals.var_t0__blk808_dn3))), ((((locals.var_coxwlcenb_dn4 * locals.var_here_b4soik1ox) + (locals.var_coxwlcenb * locals.var_here_b4soik1ox_dn4)) * assign28180_e26061) + (assign28180_e26058 * (locals.var_t1__blk809_dn4 - locals.var_t0__blk808_dn4))), ((((locals.var_coxwlcenb_dn5 * locals.var_here_b4soik1ox) + (locals.var_coxwlcenb * locals.var_here_b4soik1ox_dn5)) * assign28180_e26061) + (assign28180_e26058 * (locals.var_t1__blk809_dn5 - locals.var_t0__blk808_dn5))), ((((locals.var_coxwlcenb_dn6 * locals.var_here_b4soik1ox) + (locals.var_coxwlcenb * locals.var_here_b4soik1ox_dn6)) * assign28180_e26061) + (assign28180_e26058 * (locals.var_t1__blk809_dn6 - locals.var_t0__blk808_dn6))), ((((locals.var_coxwlcenb_dn7 * locals.var_here_b4soik1ox) + (locals.var_coxwlcenb * locals.var_here_b4soik1ox_dn7)) * assign28180_e26061) + (assign28180_e26058 * (locals.var_t1__blk809_dn7 - locals.var_t0__blk808_dn7))), ((((locals.var_coxwlcenb_dn8 * locals.var_here_b4soik1ox) + (locals.var_coxwlcenb * locals.var_here_b4soik1ox_dn8)) * assign28180_e26061) + (assign28180_e26058 * (locals.var_t1__blk809_dn8 - locals.var_t0__blk808_dn8))), ((((locals.var_coxwlcenb_dn9 * locals.var_here_b4soik1ox) + (locals.var_coxwlcenb * locals.var_here_b4soik1ox_dn9)) * assign28180_e26061) + (assign28180_e26058 * (locals.var_t1__blk809_dn9 - locals.var_t0__blk808_dn9))), ((((locals.var_coxwlcenb_dn10 * locals.var_here_b4soik1ox) + (locals.var_coxwlcenb * locals.var_here_b4soik1ox_dn10)) * assign28180_e26061) + (assign28180_e26058 * (locals.var_t1__blk809_dn10 - locals.var_t0__blk808_dn10))), ((((locals.var_coxwlcenb_dn11 * locals.var_here_b4soik1ox) + (locals.var_coxwlcenb * locals.var_here_b4soik1ox_dn11)) * assign28180_e26061) + (assign28180_e26058 * (locals.var_t1__blk809_dn11 - locals.var_t0__blk808_dn11))), ((((locals.var_coxwlcenb_dn12 * locals.var_here_b4soik1ox) + (locals.var_coxwlcenb * locals.var_here_b4soik1ox_dn12)) * assign28180_e26061) + (assign28180_e26058 * (locals.var_t1__blk809_dn12 - locals.var_t0__blk808_dn12))),)
    } else {
        (locals.var_qsub0, locals.var_qsub0_dn3, locals.var_qsub0_dn4, locals.var_qsub0_dn5, locals.var_qsub0_dn6, locals.var_qsub0_dn7, locals.var_qsub0_dn8, locals.var_qsub0_dn9, locals.var_qsub0_dn10, locals.var_qsub0_dn11, locals.var_qsub0_dn12,)
    }
};
        locals.var_qsub0 = assign28180_e26064;
        locals.var_qsub0_dn3 = assign28180_e26064_d_n3;
        locals.var_qsub0_dn4 = assign28180_e26064_d_n4;
        locals.var_qsub0_dn5 = assign28180_e26064_d_n5;
        locals.var_qsub0_dn6 = assign28180_e26064_d_n6;
        locals.var_qsub0_dn7 = assign28180_e26064_d_n7;
        locals.var_qsub0_dn8 = assign28180_e26064_d_n8;
        locals.var_qsub0_dn9 = assign28180_e26064_d_n9;
        locals.var_qsub0_dn10 = assign28180_e26064_d_n10;
        locals.var_qsub0_dn11 = assign28180_e26064_d_n11;
        locals.var_qsub0_dn12 = assign28180_e26064_d_n12;
        locals.var_qsub0_rv = 0.0;

        let assign28190_e26075: f64 = if (((locals.var_b4soisoimod != 2.0) && (locals.var_b4soibodymod != 0.0)) && (p.p27 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1405 = assign28190_e26075;
        locals.var_guard1405_rv = 0.0;

        let (assign28200_e26093, assign28200_e26093_d_n3, assign28200_e26093_d_n4, assign28200_e26093_d_n5, assign28200_e26093_d_n6, assign28200_e26093_d_n7, assign28200_e26093_d_n8, assign28200_e26093_d_n9, assign28200_e26093_d_n10, assign28200_e26093_d_n11, assign28200_e26093_d_n12,) = {
    if ((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1405 != 0.0)) {
        let assign28200_e26087: f64 = (locals.var_vgs_eff2 - locals.var_vfbeff2);
        let assign28200_e26089: f64 = (assign28200_e26087 - locals.var_vbseff);
        let assign28200_e26091: f64 = (assign28200_e26089 - locals.var_vgsteff2);
        (assign28200_e26091, (((-locals.var_vfbeff2_dn3) - locals.var_vbseff_dn3) - locals.var_vgsteff2_dn3), (((-locals.var_vfbeff2_dn4) - locals.var_vbseff_dn4) - locals.var_vgsteff2_dn4), (((-locals.var_vfbeff2_dn5) - locals.var_vbseff_dn5) - locals.var_vgsteff2_dn5), (((-locals.var_vfbeff2_dn6) - locals.var_vbseff_dn6) - locals.var_vgsteff2_dn6), (((locals.var_vgs_eff2_dn7 - locals.var_vfbeff2_dn7) - locals.var_vbseff_dn7) - locals.var_vgsteff2_dn7), (((locals.var_vgs_eff2_dn8 - locals.var_vfbeff2_dn8) - locals.var_vbseff_dn8) - locals.var_vgsteff2_dn8), (((locals.var_vgs_eff2_dn9 - locals.var_vfbeff2_dn9) - locals.var_vbseff_dn9) - locals.var_vgsteff2_dn9), (((-locals.var_vfbeff2_dn10) - locals.var_vbseff_dn10) - locals.var_vgsteff2_dn10), (((-locals.var_vfbeff2_dn11) - locals.var_vbseff_dn11) - locals.var_vgsteff2_dn11), (((-locals.var_vfbeff2_dn12) - locals.var_vbseff_dn12) - locals.var_vgsteff2_dn12),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign28200_e26093;
        locals.var_t3__blk811_dn3 = assign28200_e26093_d_n3;
        locals.var_t3__blk811_dn4 = assign28200_e26093_d_n4;
        locals.var_t3__blk811_dn5 = assign28200_e26093_d_n5;
        locals.var_t3__blk811_dn6 = assign28200_e26093_d_n6;
        locals.var_t3__blk811_dn7 = assign28200_e26093_d_n7;
        locals.var_t3__blk811_dn8 = assign28200_e26093_d_n8;
        locals.var_t3__blk811_dn9 = assign28200_e26093_d_n9;
        locals.var_t3__blk811_dn10 = assign28200_e26093_d_n10;
        locals.var_t3__blk811_dn11 = assign28200_e26093_d_n11;
        locals.var_t3__blk811_dn12 = assign28200_e26093_d_n12;
        locals.var_t3__blk811_rv = 0.0;

        let assign28210_e26096: f64 = if locals.var_here_b4soik1ox == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1406 = assign28210_e26096;
        locals.var_guard1406_rv = 0.0;

        let (assign28220_e26110, assign28220_e26110_d_n3, assign28220_e26110_d_n4, assign28220_e26110_d_n5, assign28220_e26110_d_n6, assign28220_e26110_d_n7, assign28220_e26110_d_n8, assign28220_e26110_d_n9, assign28220_e26110_d_n10, assign28220_e26110_d_n11, assign28220_e26110_d_n12,) = {
    if (((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1405 != 0.0)) && (locals.var_guard1406 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign28220_e26110;
        locals.var_t1__blk809_dn3 = assign28220_e26110_d_n3;
        locals.var_t1__blk809_dn4 = assign28220_e26110_d_n4;
        locals.var_t1__blk809_dn5 = assign28220_e26110_d_n5;
        locals.var_t1__blk809_dn6 = assign28220_e26110_d_n6;
        locals.var_t1__blk809_dn7 = assign28220_e26110_d_n7;
        locals.var_t1__blk809_dn8 = assign28220_e26110_d_n8;
        locals.var_t1__blk809_dn9 = assign28220_e26110_d_n9;
        locals.var_t1__blk809_dn10 = assign28220_e26110_d_n10;
        locals.var_t1__blk809_dn11 = assign28220_e26110_d_n11;
        locals.var_t1__blk809_dn12 = assign28220_e26110_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let assign28230_e26113: f64 = if locals.var_t3__blk811 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1407 = assign28230_e26113;
        locals.var_guard1407_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_86(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign28240_e26134, assign28240_e26134_d_n3, assign28240_e26134_d_n4, assign28240_e26134_d_n5, assign28240_e26134_d_n6, assign28240_e26134_d_n7, assign28240_e26134_d_n8, assign28240_e26134_d_n9, assign28240_e26134_d_n10, assign28240_e26134_d_n11, assign28240_e26134_d_n12,) = {
    if ((((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1405 != 0.0)) && (locals.var_guard1406 == 0.0)) && (locals.var_guard1407 != 0.0)) {
        let assign28240_e26131: f64 = (locals.var_t3__blk811 / locals.var_here_b4soik1ox);
        let assign28240_e26132: f64 = (locals.var_t0__blk808 + assign28240_e26131);
        (assign28240_e26132, (locals.var_t0__blk808_dn3 + (((locals.var_t3__blk811_dn3 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn3)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn4 + (((locals.var_t3__blk811_dn4 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn4)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn5 + (((locals.var_t3__blk811_dn5 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn5)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn6 + (((locals.var_t3__blk811_dn6 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn6)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn7 + (((locals.var_t3__blk811_dn7 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn7)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn8 + (((locals.var_t3__blk811_dn8 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn8)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn9 + (((locals.var_t3__blk811_dn9 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn9)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn10 + (((locals.var_t3__blk811_dn10 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn10)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn11 + (((locals.var_t3__blk811_dn11 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn11)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))), (locals.var_t0__blk808_dn12 + (((locals.var_t3__blk811_dn12 * locals.var_here_b4soik1ox) - (locals.var_t3__blk811 * locals.var_here_b4soik1ox_dn12)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox))),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign28240_e26134;
        locals.var_t1__blk809_dn3 = assign28240_e26134_d_n3;
        locals.var_t1__blk809_dn4 = assign28240_e26134_d_n4;
        locals.var_t1__blk809_dn5 = assign28240_e26134_d_n5;
        locals.var_t1__blk809_dn6 = assign28240_e26134_d_n6;
        locals.var_t1__blk809_dn7 = assign28240_e26134_d_n7;
        locals.var_t1__blk809_dn8 = assign28240_e26134_d_n8;
        locals.var_t1__blk809_dn9 = assign28240_e26134_d_n9;
        locals.var_t1__blk809_dn10 = assign28240_e26134_d_n10;
        locals.var_t1__blk809_dn11 = assign28240_e26134_d_n11;
        locals.var_t1__blk809_dn12 = assign28240_e26134_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign28250_e26157, assign28250_e26157_d_n3, assign28250_e26157_d_n4, assign28250_e26157_d_n5, assign28250_e26157_d_n6, assign28250_e26157_d_n7, assign28250_e26157_d_n8, assign28250_e26157_d_n9, assign28250_e26157_d_n10, assign28250_e26157_d_n11, assign28250_e26157_d_n12,) = {
    if ((((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1405 != 0.0)) && (locals.var_guard1406 == 0.0)) && (locals.var_guard1407 == 0.0)) {
        let assign28250_e26152: f64 = (locals.var_t0__blk808 * locals.var_t0__blk808);
        let assign28250_e26154: f64 = (assign28250_e26152 + locals.var_t3__blk811);
        let assign28250_e26155: f64 = (assign28250_e26154).sqrt();
        (assign28250_e26155, ((((locals.var_t0__blk808_dn3 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn3)) + locals.var_t3__blk811_dn3) / (2.0 * assign28250_e26155)), ((((locals.var_t0__blk808_dn4 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn4)) + locals.var_t3__blk811_dn4) / (2.0 * assign28250_e26155)), ((((locals.var_t0__blk808_dn5 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn5)) + locals.var_t3__blk811_dn5) / (2.0 * assign28250_e26155)), ((((locals.var_t0__blk808_dn6 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn6)) + locals.var_t3__blk811_dn6) / (2.0 * assign28250_e26155)), ((((locals.var_t0__blk808_dn7 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn7)) + locals.var_t3__blk811_dn7) / (2.0 * assign28250_e26155)), ((((locals.var_t0__blk808_dn8 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn8)) + locals.var_t3__blk811_dn8) / (2.0 * assign28250_e26155)), ((((locals.var_t0__blk808_dn9 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn9)) + locals.var_t3__blk811_dn9) / (2.0 * assign28250_e26155)), ((((locals.var_t0__blk808_dn10 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn10)) + locals.var_t3__blk811_dn10) / (2.0 * assign28250_e26155)), ((((locals.var_t0__blk808_dn11 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn11)) + locals.var_t3__blk811_dn11) / (2.0 * assign28250_e26155)), ((((locals.var_t0__blk808_dn12 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn12)) + locals.var_t3__blk811_dn12) / (2.0 * assign28250_e26155)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign28250_e26157;
        locals.var_t1__blk809_dn3 = assign28250_e26157_d_n3;
        locals.var_t1__blk809_dn4 = assign28250_e26157_d_n4;
        locals.var_t1__blk809_dn5 = assign28250_e26157_d_n5;
        locals.var_t1__blk809_dn6 = assign28250_e26157_d_n6;
        locals.var_t1__blk809_dn7 = assign28250_e26157_d_n7;
        locals.var_t1__blk809_dn8 = assign28250_e26157_d_n8;
        locals.var_t1__blk809_dn9 = assign28250_e26157_d_n9;
        locals.var_t1__blk809_dn10 = assign28250_e26157_d_n10;
        locals.var_t1__blk809_dn11 = assign28250_e26157_d_n11;
        locals.var_t1__blk809_dn12 = assign28250_e26157_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign28260_e26175, assign28260_e26175_d_n3, assign28260_e26175_d_n4, assign28260_e26175_d_n5, assign28260_e26175_d_n6, assign28260_e26175_d_n7, assign28260_e26175_d_n8, assign28260_e26175_d_n9, assign28260_e26175_d_n10, assign28260_e26175_d_n11, assign28260_e26175_d_n12,) = {
    if ((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1405 != 0.0)) {
        let assign28260_e26169: f64 = (locals.var_coxwlcenb2 * locals.var_here_b4soik1ox);
        let assign28260_e26172: f64 = (locals.var_t1__blk809 - locals.var_t0__blk808);
        let assign28260_e26173: f64 = (assign28260_e26169 * assign28260_e26172);
        (assign28260_e26173, ((((locals.var_coxwlcenb2_dn3 * locals.var_here_b4soik1ox) + (locals.var_coxwlcenb2 * locals.var_here_b4soik1ox_dn3)) * assign28260_e26172) + (assign28260_e26169 * (locals.var_t1__blk809_dn3 - locals.var_t0__blk808_dn3))), ((((locals.var_coxwlcenb2_dn4 * locals.var_here_b4soik1ox) + (locals.var_coxwlcenb2 * locals.var_here_b4soik1ox_dn4)) * assign28260_e26172) + (assign28260_e26169 * (locals.var_t1__blk809_dn4 - locals.var_t0__blk808_dn4))), ((((locals.var_coxwlcenb2_dn5 * locals.var_here_b4soik1ox) + (locals.var_coxwlcenb2 * locals.var_here_b4soik1ox_dn5)) * assign28260_e26172) + (assign28260_e26169 * (locals.var_t1__blk809_dn5 - locals.var_t0__blk808_dn5))), ((((locals.var_coxwlcenb2_dn6 * locals.var_here_b4soik1ox) + (locals.var_coxwlcenb2 * locals.var_here_b4soik1ox_dn6)) * assign28260_e26172) + (assign28260_e26169 * (locals.var_t1__blk809_dn6 - locals.var_t0__blk808_dn6))), ((((locals.var_coxwlcenb2_dn7 * locals.var_here_b4soik1ox) + (locals.var_coxwlcenb2 * locals.var_here_b4soik1ox_dn7)) * assign28260_e26172) + (assign28260_e26169 * (locals.var_t1__blk809_dn7 - locals.var_t0__blk808_dn7))), ((((locals.var_coxwlcenb2_dn8 * locals.var_here_b4soik1ox) + (locals.var_coxwlcenb2 * locals.var_here_b4soik1ox_dn8)) * assign28260_e26172) + (assign28260_e26169 * (locals.var_t1__blk809_dn8 - locals.var_t0__blk808_dn8))), ((((locals.var_coxwlcenb2_dn9 * locals.var_here_b4soik1ox) + (locals.var_coxwlcenb2 * locals.var_here_b4soik1ox_dn9)) * assign28260_e26172) + (assign28260_e26169 * (locals.var_t1__blk809_dn9 - locals.var_t0__blk808_dn9))), ((((locals.var_coxwlcenb2_dn10 * locals.var_here_b4soik1ox) + (locals.var_coxwlcenb2 * locals.var_here_b4soik1ox_dn10)) * assign28260_e26172) + (assign28260_e26169 * (locals.var_t1__blk809_dn10 - locals.var_t0__blk808_dn10))), ((((locals.var_coxwlcenb2_dn11 * locals.var_here_b4soik1ox) + (locals.var_coxwlcenb2 * locals.var_here_b4soik1ox_dn11)) * assign28260_e26172) + (assign28260_e26169 * (locals.var_t1__blk809_dn11 - locals.var_t0__blk808_dn11))), ((((locals.var_coxwlcenb2_dn12 * locals.var_here_b4soik1ox) + (locals.var_coxwlcenb2 * locals.var_here_b4soik1ox_dn12)) * assign28260_e26172) + (assign28260_e26169 * (locals.var_t1__blk809_dn12 - locals.var_t0__blk808_dn12))),)
    } else {
        (locals.var_qsub02, locals.var_qsub02_dn3, locals.var_qsub02_dn4, locals.var_qsub02_dn5, locals.var_qsub02_dn6, locals.var_qsub02_dn7, locals.var_qsub02_dn8, locals.var_qsub02_dn9, locals.var_qsub02_dn10, locals.var_qsub02_dn11, locals.var_qsub02_dn12,)
    }
};
        locals.var_qsub02 = assign28260_e26175;
        locals.var_qsub02_dn3 = assign28260_e26175_d_n3;
        locals.var_qsub02_dn4 = assign28260_e26175_d_n4;
        locals.var_qsub02_dn5 = assign28260_e26175_d_n5;
        locals.var_qsub02_dn6 = assign28260_e26175_d_n6;
        locals.var_qsub02_dn7 = assign28260_e26175_d_n7;
        locals.var_qsub02_dn8 = assign28260_e26175_d_n8;
        locals.var_qsub02_dn9 = assign28260_e26175_d_n9;
        locals.var_qsub02_dn10 = assign28260_e26175_d_n10;
        locals.var_qsub02_dn11 = assign28260_e26175_d_n11;
        locals.var_qsub02_dn12 = assign28260_e26175_d_n12;
        locals.var_qsub02_rv = 0.0;

        let (assign28270_e26189, assign28270_e26189_d_n3, assign28270_e26189_d_n4, assign28270_e26189_d_n5, assign28270_e26189_d_n6, assign28270_e26189_d_n7, assign28270_e26189_d_n8, assign28270_e26189_d_n9, assign28270_e26189_d_n10, assign28270_e26189_d_n11, assign28270_e26189_d_n12,) = {
    if ((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_guard1405 != 0.0)) {
        let assign28270_e26187: f64 = (locals.var_qsub0 + locals.var_qsub02);
        (assign28270_e26187, (locals.var_qsub0_dn3 + locals.var_qsub02_dn3), (locals.var_qsub0_dn4 + locals.var_qsub02_dn4), (locals.var_qsub0_dn5 + locals.var_qsub02_dn5), (locals.var_qsub0_dn6 + locals.var_qsub02_dn6), (locals.var_qsub0_dn7 + locals.var_qsub02_dn7), (locals.var_qsub0_dn8 + locals.var_qsub02_dn8), (locals.var_qsub0_dn9 + locals.var_qsub02_dn9), (locals.var_qsub0_dn10 + locals.var_qsub02_dn10), (locals.var_qsub0_dn11 + locals.var_qsub02_dn11), (locals.var_qsub0_dn12 + locals.var_qsub02_dn12),)
    } else {
        (locals.var_qsub0, locals.var_qsub0_dn3, locals.var_qsub0_dn4, locals.var_qsub0_dn5, locals.var_qsub0_dn6, locals.var_qsub0_dn7, locals.var_qsub0_dn8, locals.var_qsub0_dn9, locals.var_qsub0_dn10, locals.var_qsub0_dn11, locals.var_qsub0_dn12,)
    }
};
        locals.var_qsub0 = assign28270_e26189;
        locals.var_qsub0_dn3 = assign28270_e26189_d_n3;
        locals.var_qsub0_dn4 = assign28270_e26189_d_n4;
        locals.var_qsub0_dn5 = assign28270_e26189_d_n5;
        locals.var_qsub0_dn6 = assign28270_e26189_d_n6;
        locals.var_qsub0_dn7 = assign28270_e26189_d_n7;
        locals.var_qsub0_dn8 = assign28270_e26189_d_n8;
        locals.var_qsub0_dn9 = assign28270_e26189_d_n9;
        locals.var_qsub0_dn10 = assign28270_e26189_d_n10;
        locals.var_qsub0_dn11 = assign28270_e26189_d_n11;
        locals.var_qsub0_dn12 = assign28270_e26189_d_n12;
        locals.var_qsub0_rv = 0.0;

        let assign28280_e26192: f64 = if locals.var_here_b4soik1ox <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1408 = assign28280_e26192;
        locals.var_guard1408_rv = 0.0;

        let (assign28290_e26205, assign28290_e26205_d_n3, assign28290_e26205_d_n4, assign28290_e26205_d_n5, assign28290_e26205_d_n6, assign28290_e26205_d_n7, assign28290_e26205_d_n8, assign28290_e26205_d_n9, assign28290_e26205_d_n10, assign28290_e26205_d_n11, assign28290_e26205_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1408 != 0.0)) {
        let assign28290_e26201: f64 = (0.25 * locals.var_pparam_b4soimoin);
        let assign28290_e26203: f64 = (assign28290_e26201 * locals.var_vtm);
        (assign28290_e26203, ((0.25 * locals.var_pparam_b4soimoin_dn3) * locals.var_vtm), (((0.25 * locals.var_pparam_b4soimoin_dn4) * locals.var_vtm) + (assign28290_e26201 * locals.var_vtm_dn4)), (((0.25 * locals.var_pparam_b4soimoin_dn5) * locals.var_vtm) + (assign28290_e26201 * locals.var_vtm_dn5)), (((0.25 * locals.var_pparam_b4soimoin_dn6) * locals.var_vtm) + (assign28290_e26201 * locals.var_vtm_dn6)), ((0.25 * locals.var_pparam_b4soimoin_dn7) * locals.var_vtm), ((0.25 * locals.var_pparam_b4soimoin_dn8) * locals.var_vtm), ((0.25 * locals.var_pparam_b4soimoin_dn9) * locals.var_vtm), ((0.25 * locals.var_pparam_b4soimoin_dn10) * locals.var_vtm), ((0.25 * locals.var_pparam_b4soimoin_dn11) * locals.var_vtm), ((0.25 * locals.var_pparam_b4soimoin_dn12) * locals.var_vtm),)
    } else {
        (locals.var_denomi, locals.var_denomi_dn3, locals.var_denomi_dn4, locals.var_denomi_dn5, locals.var_denomi_dn6, locals.var_denomi_dn7, locals.var_denomi_dn8, locals.var_denomi_dn9, locals.var_denomi_dn10, locals.var_denomi_dn11, locals.var_denomi_dn12,)
    }
};
        locals.var_denomi = assign28290_e26205;
        locals.var_denomi_dn3 = assign28290_e26205_d_n3;
        locals.var_denomi_dn4 = assign28290_e26205_d_n4;
        locals.var_denomi_dn5 = assign28290_e26205_d_n5;
        locals.var_denomi_dn6 = assign28290_e26205_d_n6;
        locals.var_denomi_dn7 = assign28290_e26205_d_n7;
        locals.var_denomi_dn8 = assign28290_e26205_d_n8;
        locals.var_denomi_dn9 = assign28290_e26205_d_n9;
        locals.var_denomi_dn10 = assign28290_e26205_d_n10;
        locals.var_denomi_dn11 = assign28290_e26205_d_n11;
        locals.var_denomi_dn12 = assign28290_e26205_d_n12;
        locals.var_denomi_rv = 0.0;

        let (assign28300_e26216, assign28300_e26216_d_n3, assign28300_e26216_d_n4, assign28300_e26216_d_n5, assign28300_e26216_d_n6, assign28300_e26216_d_n7, assign28300_e26216_d_n8, assign28300_e26216_d_n9, assign28300_e26216_d_n10, assign28300_e26216_d_n11, assign28300_e26216_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1408 != 0.0)) {
        let assign28300_e26214: f64 = (0.5 * locals.var_pparam_b4soisqrtphi);
        (assign28300_e26214, (0.5 * locals.var_pparam_b4soisqrtphi_dn3), (0.5 * locals.var_pparam_b4soisqrtphi_dn4), (0.5 * locals.var_pparam_b4soisqrtphi_dn5), (0.5 * locals.var_pparam_b4soisqrtphi_dn6), (0.5 * locals.var_pparam_b4soisqrtphi_dn7), (0.5 * locals.var_pparam_b4soisqrtphi_dn8), (0.5 * locals.var_pparam_b4soisqrtphi_dn9), (0.5 * locals.var_pparam_b4soisqrtphi_dn10), (0.5 * locals.var_pparam_b4soisqrtphi_dn11), (0.5 * locals.var_pparam_b4soisqrtphi_dn12),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign28300_e26216;
        locals.var_t0__blk808_dn3 = assign28300_e26216_d_n3;
        locals.var_t0__blk808_dn4 = assign28300_e26216_d_n4;
        locals.var_t0__blk808_dn5 = assign28300_e26216_d_n5;
        locals.var_t0__blk808_dn6 = assign28300_e26216_d_n6;
        locals.var_t0__blk808_dn7 = assign28300_e26216_d_n7;
        locals.var_t0__blk808_dn8 = assign28300_e26216_d_n8;
        locals.var_t0__blk808_dn9 = assign28300_e26216_d_n9;
        locals.var_t0__blk808_dn10 = assign28300_e26216_d_n10;
        locals.var_t0__blk808_dn11 = assign28300_e26216_d_n11;
        locals.var_t0__blk808_dn12 = assign28300_e26216_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign28310_e26232, assign28310_e26232_d_n3, assign28310_e26232_d_n4, assign28310_e26232_d_n5, assign28310_e26232_d_n6, assign28310_e26232_d_n7, assign28310_e26232_d_n8, assign28310_e26232_d_n9, assign28310_e26232_d_n10, assign28310_e26232_d_n11, assign28310_e26232_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1408 == 0.0)) {
        let assign28310_e26226: f64 = (locals.var_pparam_b4soimoin * locals.var_vtm);
        let assign28310_e26228: f64 = (assign28310_e26226 * locals.var_here_b4soik1ox);
        let assign28310_e26230: f64 = (assign28310_e26228 * locals.var_here_b4soik1ox);
        (assign28310_e26230, (((((locals.var_pparam_b4soimoin_dn3 * locals.var_vtm) * locals.var_here_b4soik1ox) + (assign28310_e26226 * locals.var_here_b4soik1ox_dn3)) * locals.var_here_b4soik1ox) + (assign28310_e26228 * locals.var_here_b4soik1ox_dn3)), ((((((locals.var_pparam_b4soimoin_dn4 * locals.var_vtm) + (locals.var_pparam_b4soimoin * locals.var_vtm_dn4)) * locals.var_here_b4soik1ox) + (assign28310_e26226 * locals.var_here_b4soik1ox_dn4)) * locals.var_here_b4soik1ox) + (assign28310_e26228 * locals.var_here_b4soik1ox_dn4)), ((((((locals.var_pparam_b4soimoin_dn5 * locals.var_vtm) + (locals.var_pparam_b4soimoin * locals.var_vtm_dn5)) * locals.var_here_b4soik1ox) + (assign28310_e26226 * locals.var_here_b4soik1ox_dn5)) * locals.var_here_b4soik1ox) + (assign28310_e26228 * locals.var_here_b4soik1ox_dn5)), ((((((locals.var_pparam_b4soimoin_dn6 * locals.var_vtm) + (locals.var_pparam_b4soimoin * locals.var_vtm_dn6)) * locals.var_here_b4soik1ox) + (assign28310_e26226 * locals.var_here_b4soik1ox_dn6)) * locals.var_here_b4soik1ox) + (assign28310_e26228 * locals.var_here_b4soik1ox_dn6)), (((((locals.var_pparam_b4soimoin_dn7 * locals.var_vtm) * locals.var_here_b4soik1ox) + (assign28310_e26226 * locals.var_here_b4soik1ox_dn7)) * locals.var_here_b4soik1ox) + (assign28310_e26228 * locals.var_here_b4soik1ox_dn7)), (((((locals.var_pparam_b4soimoin_dn8 * locals.var_vtm) * locals.var_here_b4soik1ox) + (assign28310_e26226 * locals.var_here_b4soik1ox_dn8)) * locals.var_here_b4soik1ox) + (assign28310_e26228 * locals.var_here_b4soik1ox_dn8)), (((((locals.var_pparam_b4soimoin_dn9 * locals.var_vtm) * locals.var_here_b4soik1ox) + (assign28310_e26226 * locals.var_here_b4soik1ox_dn9)) * locals.var_here_b4soik1ox) + (assign28310_e26228 * locals.var_here_b4soik1ox_dn9)), (((((locals.var_pparam_b4soimoin_dn10 * locals.var_vtm) * locals.var_here_b4soik1ox) + (assign28310_e26226 * locals.var_here_b4soik1ox_dn10)) * locals.var_here_b4soik1ox) + (assign28310_e26228 * locals.var_here_b4soik1ox_dn10)), (((((locals.var_pparam_b4soimoin_dn11 * locals.var_vtm) * locals.var_here_b4soik1ox) + (assign28310_e26226 * locals.var_here_b4soik1ox_dn11)) * locals.var_here_b4soik1ox) + (assign28310_e26228 * locals.var_here_b4soik1ox_dn11)), (((((locals.var_pparam_b4soimoin_dn12 * locals.var_vtm) * locals.var_here_b4soik1ox) + (assign28310_e26226 * locals.var_here_b4soik1ox_dn12)) * locals.var_here_b4soik1ox) + (assign28310_e26228 * locals.var_here_b4soik1ox_dn12)),)
    } else {
        (locals.var_denomi, locals.var_denomi_dn3, locals.var_denomi_dn4, locals.var_denomi_dn5, locals.var_denomi_dn6, locals.var_denomi_dn7, locals.var_denomi_dn8, locals.var_denomi_dn9, locals.var_denomi_dn10, locals.var_denomi_dn11, locals.var_denomi_dn12,)
    }
};
        locals.var_denomi = assign28310_e26232;
        locals.var_denomi_dn3 = assign28310_e26232_d_n3;
        locals.var_denomi_dn4 = assign28310_e26232_d_n4;
        locals.var_denomi_dn5 = assign28310_e26232_d_n5;
        locals.var_denomi_dn6 = assign28310_e26232_d_n6;
        locals.var_denomi_dn7 = assign28310_e26232_d_n7;
        locals.var_denomi_dn8 = assign28310_e26232_d_n8;
        locals.var_denomi_dn9 = assign28310_e26232_d_n9;
        locals.var_denomi_dn10 = assign28310_e26232_d_n10;
        locals.var_denomi_dn11 = assign28310_e26232_d_n11;
        locals.var_denomi_dn12 = assign28310_e26232_d_n12;
        locals.var_denomi_rv = 0.0;

        let (assign28320_e26244, assign28320_e26244_d_n3, assign28320_e26244_d_n4, assign28320_e26244_d_n5, assign28320_e26244_d_n6, assign28320_e26244_d_n7, assign28320_e26244_d_n8, assign28320_e26244_d_n9, assign28320_e26244_d_n10, assign28320_e26244_d_n11, assign28320_e26244_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1408 == 0.0)) {
        let assign28320_e26242: f64 = (locals.var_here_b4soik1ox * locals.var_pparam_b4soisqrtphi);
        (assign28320_e26242, ((locals.var_here_b4soik1ox_dn3 * locals.var_pparam_b4soisqrtphi) + (locals.var_here_b4soik1ox * locals.var_pparam_b4soisqrtphi_dn3)), ((locals.var_here_b4soik1ox_dn4 * locals.var_pparam_b4soisqrtphi) + (locals.var_here_b4soik1ox * locals.var_pparam_b4soisqrtphi_dn4)), ((locals.var_here_b4soik1ox_dn5 * locals.var_pparam_b4soisqrtphi) + (locals.var_here_b4soik1ox * locals.var_pparam_b4soisqrtphi_dn5)), ((locals.var_here_b4soik1ox_dn6 * locals.var_pparam_b4soisqrtphi) + (locals.var_here_b4soik1ox * locals.var_pparam_b4soisqrtphi_dn6)), ((locals.var_here_b4soik1ox_dn7 * locals.var_pparam_b4soisqrtphi) + (locals.var_here_b4soik1ox * locals.var_pparam_b4soisqrtphi_dn7)), ((locals.var_here_b4soik1ox_dn8 * locals.var_pparam_b4soisqrtphi) + (locals.var_here_b4soik1ox * locals.var_pparam_b4soisqrtphi_dn8)), ((locals.var_here_b4soik1ox_dn9 * locals.var_pparam_b4soisqrtphi) + (locals.var_here_b4soik1ox * locals.var_pparam_b4soisqrtphi_dn9)), ((locals.var_here_b4soik1ox_dn10 * locals.var_pparam_b4soisqrtphi) + (locals.var_here_b4soik1ox * locals.var_pparam_b4soisqrtphi_dn10)), ((locals.var_here_b4soik1ox_dn11 * locals.var_pparam_b4soisqrtphi) + (locals.var_here_b4soik1ox * locals.var_pparam_b4soisqrtphi_dn11)), ((locals.var_here_b4soik1ox_dn12 * locals.var_pparam_b4soisqrtphi) + (locals.var_here_b4soik1ox * locals.var_pparam_b4soisqrtphi_dn12)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign28320_e26244;
        locals.var_t0__blk808_dn3 = assign28320_e26244_d_n3;
        locals.var_t0__blk808_dn4 = assign28320_e26244_d_n4;
        locals.var_t0__blk808_dn5 = assign28320_e26244_d_n5;
        locals.var_t0__blk808_dn6 = assign28320_e26244_d_n6;
        locals.var_t0__blk808_dn7 = assign28320_e26244_d_n7;
        locals.var_t0__blk808_dn8 = assign28320_e26244_d_n8;
        locals.var_t0__blk808_dn9 = assign28320_e26244_d_n9;
        locals.var_t0__blk808_dn10 = assign28320_e26244_d_n10;
        locals.var_t0__blk808_dn11 = assign28320_e26244_d_n11;
        locals.var_t0__blk808_dn12 = assign28320_e26244_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign28330_e26255, assign28330_e26255_d_n3, assign28330_e26255_d_n4, assign28330_e26255_d_n5, assign28330_e26255_d_n6, assign28330_e26255_d_n7, assign28330_e26255_d_n8, assign28330_e26255_d_n9, assign28330_e26255_d_n10, assign28330_e26255_d_n11, assign28330_e26255_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) {
        let assign28330_e26251: f64 = (2.0 * locals.var_t0__blk808);
        let assign28330_e26253: f64 = (assign28330_e26251 + locals.var_vgsteff__blk840);
        (assign28330_e26253, ((2.0 * locals.var_t0__blk808_dn3) + locals.var_vgsteff__blk840_dn3), ((2.0 * locals.var_t0__blk808_dn4) + locals.var_vgsteff__blk840_dn4), ((2.0 * locals.var_t0__blk808_dn5) + locals.var_vgsteff__blk840_dn5), ((2.0 * locals.var_t0__blk808_dn6) + locals.var_vgsteff__blk840_dn6), ((2.0 * locals.var_t0__blk808_dn7) + locals.var_vgsteff__blk840_dn7), ((2.0 * locals.var_t0__blk808_dn8) + locals.var_vgsteff__blk840_dn8), ((2.0 * locals.var_t0__blk808_dn9) + locals.var_vgsteff__blk840_dn9), ((2.0 * locals.var_t0__blk808_dn10) + locals.var_vgsteff__blk840_dn10), ((2.0 * locals.var_t0__blk808_dn11) + locals.var_vgsteff__blk840_dn11), ((2.0 * locals.var_t0__blk808_dn12) + locals.var_vgsteff__blk840_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign28330_e26255;
        locals.var_t1__blk809_dn3 = assign28330_e26255_d_n3;
        locals.var_t1__blk809_dn4 = assign28330_e26255_d_n4;
        locals.var_t1__blk809_dn5 = assign28330_e26255_d_n5;
        locals.var_t1__blk809_dn6 = assign28330_e26255_d_n6;
        locals.var_t1__blk809_dn7 = assign28330_e26255_d_n7;
        locals.var_t1__blk809_dn8 = assign28330_e26255_d_n8;
        locals.var_t1__blk809_dn9 = assign28330_e26255_d_n9;
        locals.var_t1__blk809_dn10 = assign28330_e26255_d_n10;
        locals.var_t1__blk809_dn11 = assign28330_e26255_d_n11;
        locals.var_t1__blk809_dn12 = assign28330_e26255_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign28340_e26283, assign28340_e26283_d_n3, assign28340_e26283_d_n4, assign28340_e26283_d_n5, assign28340_e26283_d_n6, assign28340_e26283_d_n7, assign28340_e26283_d_n8, assign28340_e26283_d_n9, assign28340_e26283_d_n10, assign28340_e26283_d_n11, assign28340_e26283_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) {
        let assign28340_e26264: f64 = (locals.var_t1__blk809 * locals.var_vgsteff__blk840);
        let assign28340_e26266: f64 = (assign28340_e26264 / locals.var_denomi);
        let assign28340_e26267: f64 = (1.0 + assign28340_e26266);
        let (assign28340_e26280, assign28340_e26280_d_n3, assign28340_e26280_d_n4, assign28340_e26280_d_n5, assign28340_e26280_d_n6, assign28340_e26280_d_n7, assign28340_e26280_d_n8, assign28340_e26280_d_n9, assign28340_e26280_d_n10, assign28340_e26280_d_n11, assign28340_e26280_d_n12,) = {
            if (assign28340_e26267 > 1e-38) {
                let assign28340_e26273: f64 = (locals.var_t1__blk809 * locals.var_vgsteff__blk840);
                let assign28340_e26275: f64 = (assign28340_e26273 / locals.var_denomi);
                let assign28340_e26276: f64 = (1.0 + assign28340_e26275);
                let assign28340_e26277: f64 = (assign28340_e26276).ln();
                (assign28340_e26277, ((((((locals.var_t1__blk809_dn3 * locals.var_vgsteff__blk840) + (locals.var_t1__blk809 * locals.var_vgsteff__blk840_dn3)) * locals.var_denomi) - (assign28340_e26273 * locals.var_denomi_dn3)) / (locals.var_denomi * locals.var_denomi)) / assign28340_e26276), ((((((locals.var_t1__blk809_dn4 * locals.var_vgsteff__blk840) + (locals.var_t1__blk809 * locals.var_vgsteff__blk840_dn4)) * locals.var_denomi) - (assign28340_e26273 * locals.var_denomi_dn4)) / (locals.var_denomi * locals.var_denomi)) / assign28340_e26276), ((((((locals.var_t1__blk809_dn5 * locals.var_vgsteff__blk840) + (locals.var_t1__blk809 * locals.var_vgsteff__blk840_dn5)) * locals.var_denomi) - (assign28340_e26273 * locals.var_denomi_dn5)) / (locals.var_denomi * locals.var_denomi)) / assign28340_e26276), ((((((locals.var_t1__blk809_dn6 * locals.var_vgsteff__blk840) + (locals.var_t1__blk809 * locals.var_vgsteff__blk840_dn6)) * locals.var_denomi) - (assign28340_e26273 * locals.var_denomi_dn6)) / (locals.var_denomi * locals.var_denomi)) / assign28340_e26276), ((((((locals.var_t1__blk809_dn7 * locals.var_vgsteff__blk840) + (locals.var_t1__blk809 * locals.var_vgsteff__blk840_dn7)) * locals.var_denomi) - (assign28340_e26273 * locals.var_denomi_dn7)) / (locals.var_denomi * locals.var_denomi)) / assign28340_e26276), ((((((locals.var_t1__blk809_dn8 * locals.var_vgsteff__blk840) + (locals.var_t1__blk809 * locals.var_vgsteff__blk840_dn8)) * locals.var_denomi) - (assign28340_e26273 * locals.var_denomi_dn8)) / (locals.var_denomi * locals.var_denomi)) / assign28340_e26276), ((((((locals.var_t1__blk809_dn9 * locals.var_vgsteff__blk840) + (locals.var_t1__blk809 * locals.var_vgsteff__blk840_dn9)) * locals.var_denomi) - (assign28340_e26273 * locals.var_denomi_dn9)) / (locals.var_denomi * locals.var_denomi)) / assign28340_e26276), ((((((locals.var_t1__blk809_dn10 * locals.var_vgsteff__blk840) + (locals.var_t1__blk809 * locals.var_vgsteff__blk840_dn10)) * locals.var_denomi) - (assign28340_e26273 * locals.var_denomi_dn10)) / (locals.var_denomi * locals.var_denomi)) / assign28340_e26276), ((((((locals.var_t1__blk809_dn11 * locals.var_vgsteff__blk840) + (locals.var_t1__blk809 * locals.var_vgsteff__blk840_dn11)) * locals.var_denomi) - (assign28340_e26273 * locals.var_denomi_dn11)) / (locals.var_denomi * locals.var_denomi)) / assign28340_e26276), ((((((locals.var_t1__blk809_dn12 * locals.var_vgsteff__blk840) + (locals.var_t1__blk809 * locals.var_vgsteff__blk840_dn12)) * locals.var_denomi) - (assign28340_e26273 * locals.var_denomi_dn12)) / (locals.var_denomi * locals.var_denomi)) / assign28340_e26276),)
            } else {
                let assign28340_e26279: f64 = (-87.49823353377374);
                (assign28340_e26279, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign28340_e26281: f64 = (locals.var_vtm * assign28340_e26280);
        (assign28340_e26281, (locals.var_vtm * assign28340_e26280_d_n3), ((locals.var_vtm_dn4 * assign28340_e26280) + (locals.var_vtm * assign28340_e26280_d_n4)), ((locals.var_vtm_dn5 * assign28340_e26280) + (locals.var_vtm * assign28340_e26280_d_n5)), ((locals.var_vtm_dn6 * assign28340_e26280) + (locals.var_vtm * assign28340_e26280_d_n6)), (locals.var_vtm * assign28340_e26280_d_n7), (locals.var_vtm * assign28340_e26280_d_n8), (locals.var_vtm * assign28340_e26280_d_n9), (locals.var_vtm * assign28340_e26280_d_n10), (locals.var_vtm * assign28340_e26280_d_n11), (locals.var_vtm * assign28340_e26280_d_n12),)
    } else {
        (locals.var_deltaphi, locals.var_deltaphi_dn3, locals.var_deltaphi_dn4, locals.var_deltaphi_dn5, locals.var_deltaphi_dn6, locals.var_deltaphi_dn7, locals.var_deltaphi_dn8, locals.var_deltaphi_dn9, locals.var_deltaphi_dn10, locals.var_deltaphi_dn11, locals.var_deltaphi_dn12,)
    }
};
        locals.var_deltaphi = assign28340_e26283;
        locals.var_deltaphi_dn3 = assign28340_e26283_d_n3;
        locals.var_deltaphi_dn4 = assign28340_e26283_d_n4;
        locals.var_deltaphi_dn5 = assign28340_e26283_d_n5;
        locals.var_deltaphi_dn6 = assign28340_e26283_d_n6;
        locals.var_deltaphi_dn7 = assign28340_e26283_d_n7;
        locals.var_deltaphi_dn8 = assign28340_e26283_d_n8;
        locals.var_deltaphi_dn9 = assign28340_e26283_d_n9;
        locals.var_deltaphi_dn10 = assign28340_e26283_d_n10;
        locals.var_deltaphi_dn11 = assign28340_e26283_d_n11;
        locals.var_deltaphi_dn12 = assign28340_e26283_d_n12;
        locals.var_deltaphi_rv = 0.0;

        let assign28350_e26286: f64 = if p.p27 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1409 = assign28350_e26286;
        locals.var_guard1409_rv = 0.0;

        let (assign28360_e26299, assign28360_e26299_d_n3, assign28360_e26299_d_n4, assign28360_e26299_d_n5, assign28360_e26299_d_n6, assign28360_e26299_d_n7, assign28360_e26299_d_n8, assign28360_e26299_d_n9, assign28360_e26299_d_n10, assign28360_e26299_d_n11, assign28360_e26299_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1409 != 0.0)) {
        let assign28360_e26295: f64 = (2.0 * locals.var_t0__blk808);
        let assign28360_e26297: f64 = (assign28360_e26295 + locals.var_vgsteff2);
        (assign28360_e26297, ((2.0 * locals.var_t0__blk808_dn3) + locals.var_vgsteff2_dn3), ((2.0 * locals.var_t0__blk808_dn4) + locals.var_vgsteff2_dn4), ((2.0 * locals.var_t0__blk808_dn5) + locals.var_vgsteff2_dn5), ((2.0 * locals.var_t0__blk808_dn6) + locals.var_vgsteff2_dn6), ((2.0 * locals.var_t0__blk808_dn7) + locals.var_vgsteff2_dn7), ((2.0 * locals.var_t0__blk808_dn8) + locals.var_vgsteff2_dn8), ((2.0 * locals.var_t0__blk808_dn9) + locals.var_vgsteff2_dn9), ((2.0 * locals.var_t0__blk808_dn10) + locals.var_vgsteff2_dn10), ((2.0 * locals.var_t0__blk808_dn11) + locals.var_vgsteff2_dn11), ((2.0 * locals.var_t0__blk808_dn12) + locals.var_vgsteff2_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign28360_e26299;
        locals.var_t1__blk809_dn3 = assign28360_e26299_d_n3;
        locals.var_t1__blk809_dn4 = assign28360_e26299_d_n4;
        locals.var_t1__blk809_dn5 = assign28360_e26299_d_n5;
        locals.var_t1__blk809_dn6 = assign28360_e26299_d_n6;
        locals.var_t1__blk809_dn7 = assign28360_e26299_d_n7;
        locals.var_t1__blk809_dn8 = assign28360_e26299_d_n8;
        locals.var_t1__blk809_dn9 = assign28360_e26299_d_n9;
        locals.var_t1__blk809_dn10 = assign28360_e26299_d_n10;
        locals.var_t1__blk809_dn11 = assign28360_e26299_d_n11;
        locals.var_t1__blk809_dn12 = assign28360_e26299_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign28370_e26329, assign28370_e26329_d_n3, assign28370_e26329_d_n4, assign28370_e26329_d_n5, assign28370_e26329_d_n6, assign28370_e26329_d_n7, assign28370_e26329_d_n8, assign28370_e26329_d_n9, assign28370_e26329_d_n10, assign28370_e26329_d_n11, assign28370_e26329_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1409 != 0.0)) {
        let assign28370_e26310: f64 = (locals.var_t1__blk809 * locals.var_vgsteff2);
        let assign28370_e26312: f64 = (assign28370_e26310 / locals.var_denomi);
        let assign28370_e26313: f64 = (1.0 + assign28370_e26312);
        let (assign28370_e26326, assign28370_e26326_d_n3, assign28370_e26326_d_n4, assign28370_e26326_d_n5, assign28370_e26326_d_n6, assign28370_e26326_d_n7, assign28370_e26326_d_n8, assign28370_e26326_d_n9, assign28370_e26326_d_n10, assign28370_e26326_d_n11, assign28370_e26326_d_n12,) = {
            if (assign28370_e26313 > 1e-38) {
                let assign28370_e26319: f64 = (locals.var_t1__blk809 * locals.var_vgsteff2);
                let assign28370_e26321: f64 = (assign28370_e26319 / locals.var_denomi);
                let assign28370_e26322: f64 = (1.0 + assign28370_e26321);
                let assign28370_e26323: f64 = (assign28370_e26322).ln();
                (assign28370_e26323, ((((((locals.var_t1__blk809_dn3 * locals.var_vgsteff2) + (locals.var_t1__blk809 * locals.var_vgsteff2_dn3)) * locals.var_denomi) - (assign28370_e26319 * locals.var_denomi_dn3)) / (locals.var_denomi * locals.var_denomi)) / assign28370_e26322), ((((((locals.var_t1__blk809_dn4 * locals.var_vgsteff2) + (locals.var_t1__blk809 * locals.var_vgsteff2_dn4)) * locals.var_denomi) - (assign28370_e26319 * locals.var_denomi_dn4)) / (locals.var_denomi * locals.var_denomi)) / assign28370_e26322), ((((((locals.var_t1__blk809_dn5 * locals.var_vgsteff2) + (locals.var_t1__blk809 * locals.var_vgsteff2_dn5)) * locals.var_denomi) - (assign28370_e26319 * locals.var_denomi_dn5)) / (locals.var_denomi * locals.var_denomi)) / assign28370_e26322), ((((((locals.var_t1__blk809_dn6 * locals.var_vgsteff2) + (locals.var_t1__blk809 * locals.var_vgsteff2_dn6)) * locals.var_denomi) - (assign28370_e26319 * locals.var_denomi_dn6)) / (locals.var_denomi * locals.var_denomi)) / assign28370_e26322), ((((((locals.var_t1__blk809_dn7 * locals.var_vgsteff2) + (locals.var_t1__blk809 * locals.var_vgsteff2_dn7)) * locals.var_denomi) - (assign28370_e26319 * locals.var_denomi_dn7)) / (locals.var_denomi * locals.var_denomi)) / assign28370_e26322), ((((((locals.var_t1__blk809_dn8 * locals.var_vgsteff2) + (locals.var_t1__blk809 * locals.var_vgsteff2_dn8)) * locals.var_denomi) - (assign28370_e26319 * locals.var_denomi_dn8)) / (locals.var_denomi * locals.var_denomi)) / assign28370_e26322), ((((((locals.var_t1__blk809_dn9 * locals.var_vgsteff2) + (locals.var_t1__blk809 * locals.var_vgsteff2_dn9)) * locals.var_denomi) - (assign28370_e26319 * locals.var_denomi_dn9)) / (locals.var_denomi * locals.var_denomi)) / assign28370_e26322), ((((((locals.var_t1__blk809_dn10 * locals.var_vgsteff2) + (locals.var_t1__blk809 * locals.var_vgsteff2_dn10)) * locals.var_denomi) - (assign28370_e26319 * locals.var_denomi_dn10)) / (locals.var_denomi * locals.var_denomi)) / assign28370_e26322), ((((((locals.var_t1__blk809_dn11 * locals.var_vgsteff2) + (locals.var_t1__blk809 * locals.var_vgsteff2_dn11)) * locals.var_denomi) - (assign28370_e26319 * locals.var_denomi_dn11)) / (locals.var_denomi * locals.var_denomi)) / assign28370_e26322), ((((((locals.var_t1__blk809_dn12 * locals.var_vgsteff2) + (locals.var_t1__blk809 * locals.var_vgsteff2_dn12)) * locals.var_denomi) - (assign28370_e26319 * locals.var_denomi_dn12)) / (locals.var_denomi * locals.var_denomi)) / assign28370_e26322),)
            } else {
                let assign28370_e26325: f64 = (-87.49823353377374);
                (assign28370_e26325, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign28370_e26327: f64 = (locals.var_vtm * assign28370_e26326);
        (assign28370_e26327, (locals.var_vtm * assign28370_e26326_d_n3), ((locals.var_vtm_dn4 * assign28370_e26326) + (locals.var_vtm * assign28370_e26326_d_n4)), ((locals.var_vtm_dn5 * assign28370_e26326) + (locals.var_vtm * assign28370_e26326_d_n5)), ((locals.var_vtm_dn6 * assign28370_e26326) + (locals.var_vtm * assign28370_e26326_d_n6)), (locals.var_vtm * assign28370_e26326_d_n7), (locals.var_vtm * assign28370_e26326_d_n8), (locals.var_vtm * assign28370_e26326_d_n9), (locals.var_vtm * assign28370_e26326_d_n10), (locals.var_vtm * assign28370_e26326_d_n11), (locals.var_vtm * assign28370_e26326_d_n12),)
    } else {
        (locals.var_deltaphi2, locals.var_deltaphi2_dn3, locals.var_deltaphi2_dn4, locals.var_deltaphi2_dn5, locals.var_deltaphi2_dn6, locals.var_deltaphi2_dn7, locals.var_deltaphi2_dn8, locals.var_deltaphi2_dn9, locals.var_deltaphi2_dn10, locals.var_deltaphi2_dn11, locals.var_deltaphi2_dn12,)
    }
};
        locals.var_deltaphi2 = assign28370_e26329;
        locals.var_deltaphi2_dn3 = assign28370_e26329_d_n3;
        locals.var_deltaphi2_dn4 = assign28370_e26329_d_n4;
        locals.var_deltaphi2_dn5 = assign28370_e26329_d_n5;
        locals.var_deltaphi2_dn6 = assign28370_e26329_d_n6;
        locals.var_deltaphi2_dn7 = assign28370_e26329_d_n7;
        locals.var_deltaphi2_dn8 = assign28370_e26329_d_n8;
        locals.var_deltaphi2_dn9 = assign28370_e26329_d_n9;
        locals.var_deltaphi2_dn10 = assign28370_e26329_d_n10;
        locals.var_deltaphi2_dn11 = assign28370_e26329_d_n11;
        locals.var_deltaphi2_dn12 = assign28370_e26329_d_n12;
        locals.var_deltaphi2_rv = 0.0;

        let (assign28380_e26342, assign28380_e26342_d_n3, assign28380_e26342_d_n4, assign28380_e26342_d_n5, assign28380_e26342_d_n6, assign28380_e26342_d_n7, assign28380_e26342_d_n8, assign28380_e26342_d_n9, assign28380_e26342_d_n10, assign28380_e26342_d_n11, assign28380_e26342_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) {
        let assign28380_e26337: f64 = (locals.var_vth__blk794 - locals.var_vfbzb);
        let assign28380_e26339: f64 = (assign28380_e26337 - locals.var_phi);
        let assign28380_e26340: f64 = (4.0 * assign28380_e26339);
        (assign28380_e26340, (4.0 * ((locals.var_vth__blk794_dn3 - locals.var_vfbzb_dn3) - locals.var_phi_dn3)), (4.0 * ((locals.var_vth__blk794_dn4 - locals.var_vfbzb_dn4) - locals.var_phi_dn4)), (4.0 * ((locals.var_vth__blk794_dn5 - locals.var_vfbzb_dn5) - locals.var_phi_dn5)), (4.0 * ((locals.var_vth__blk794_dn6 - locals.var_vfbzb_dn6) - locals.var_phi_dn6)), (4.0 * ((locals.var_vth__blk794_dn7 - locals.var_vfbzb_dn7) - locals.var_phi_dn7)), (4.0 * ((locals.var_vth__blk794_dn8 - locals.var_vfbzb_dn8) - locals.var_phi_dn8)), (4.0 * ((locals.var_vth__blk794_dn9 - locals.var_vfbzb_dn9) - locals.var_phi_dn9)), (4.0 * ((locals.var_vth__blk794_dn10 - locals.var_vfbzb_dn10) - locals.var_phi_dn10)), (4.0 * ((locals.var_vth__blk794_dn11 - locals.var_vfbzb_dn11) - locals.var_phi_dn11)), (4.0 * ((locals.var_vth__blk794_dn12 - locals.var_vfbzb_dn12) - locals.var_phi_dn12)),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign28380_e26342;
        locals.var_t3__blk811_dn3 = assign28380_e26342_d_n3;
        locals.var_t3__blk811_dn4 = assign28380_e26342_d_n4;
        locals.var_t3__blk811_dn5 = assign28380_e26342_d_n5;
        locals.var_t3__blk811_dn6 = assign28380_e26342_d_n6;
        locals.var_t3__blk811_dn7 = assign28380_e26342_d_n7;
        locals.var_t3__blk811_dn8 = assign28380_e26342_d_n8;
        locals.var_t3__blk811_dn9 = assign28380_e26342_d_n9;
        locals.var_t3__blk811_dn10 = assign28380_e26342_d_n10;
        locals.var_t3__blk811_dn11 = assign28380_e26342_d_n11;
        locals.var_t3__blk811_dn12 = assign28380_e26342_d_n12;
        locals.var_t3__blk811_rv = 0.0;

        let (assign28390_e26354, assign28390_e26354_d_n3, assign28390_e26354_d_n4, assign28390_e26354_d_n5, assign28390_e26354_d_n6, assign28390_e26354_d_n7, assign28390_e26354_d_n8, assign28390_e26354_d_n9, assign28390_e26354_d_n10, assign28390_e26354_d_n11, assign28390_e26354_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) {
        let assign28390_e26349: f64 = (locals.var_t3__blk811 * locals.var_t3__blk811);
        let assign28390_e26351: f64 = (assign28390_e26349 + 0.0001);
        let assign28390_e26352: f64 = (assign28390_e26351).sqrt();
        (assign28390_e26352, (((locals.var_t3__blk811_dn3 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn3)) / (2.0 * assign28390_e26352)), (((locals.var_t3__blk811_dn4 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn4)) / (2.0 * assign28390_e26352)), (((locals.var_t3__blk811_dn5 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn5)) / (2.0 * assign28390_e26352)), (((locals.var_t3__blk811_dn6 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn6)) / (2.0 * assign28390_e26352)), (((locals.var_t3__blk811_dn7 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn7)) / (2.0 * assign28390_e26352)), (((locals.var_t3__blk811_dn8 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn8)) / (2.0 * assign28390_e26352)), (((locals.var_t3__blk811_dn9 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn9)) / (2.0 * assign28390_e26352)), (((locals.var_t3__blk811_dn10 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn10)) / (2.0 * assign28390_e26352)), (((locals.var_t3__blk811_dn11 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn11)) / (2.0 * assign28390_e26352)), (((locals.var_t3__blk811_dn12 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn12)) / (2.0 * assign28390_e26352)),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign28390_e26354;
        locals.var_t2__blk810_dn3 = assign28390_e26354_d_n3;
        locals.var_t2__blk810_dn4 = assign28390_e26354_d_n4;
        locals.var_t2__blk810_dn5 = assign28390_e26354_d_n5;
        locals.var_t2__blk810_dn6 = assign28390_e26354_d_n6;
        locals.var_t2__blk810_dn7 = assign28390_e26354_d_n7;
        locals.var_t2__blk810_dn8 = assign28390_e26354_d_n8;
        locals.var_t2__blk810_dn9 = assign28390_e26354_d_n9;
        locals.var_t2__blk810_dn10 = assign28390_e26354_d_n10;
        locals.var_t2__blk810_dn11 = assign28390_e26354_d_n11;
        locals.var_t2__blk810_dn12 = assign28390_e26354_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign28400_e26365, assign28400_e26365_d_n3, assign28400_e26365_d_n4, assign28400_e26365_d_n5, assign28400_e26365_d_n6, assign28400_e26365_d_n7, assign28400_e26365_d_n8, assign28400_e26365_d_n9, assign28400_e26365_d_n10, assign28400_e26365_d_n11, assign28400_e26365_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) {
        let assign28400_e26362: f64 = (locals.var_t3__blk811 + locals.var_t2__blk810);
        let assign28400_e26363: f64 = (0.5 * assign28400_e26362);
        (assign28400_e26363, (0.5 * (locals.var_t3__blk811_dn3 + locals.var_t2__blk810_dn3)), (0.5 * (locals.var_t3__blk811_dn4 + locals.var_t2__blk810_dn4)), (0.5 * (locals.var_t3__blk811_dn5 + locals.var_t2__blk810_dn5)), (0.5 * (locals.var_t3__blk811_dn6 + locals.var_t2__blk810_dn6)), (0.5 * (locals.var_t3__blk811_dn7 + locals.var_t2__blk810_dn7)), (0.5 * (locals.var_t3__blk811_dn8 + locals.var_t2__blk810_dn8)), (0.5 * (locals.var_t3__blk811_dn9 + locals.var_t2__blk810_dn9)), (0.5 * (locals.var_t3__blk811_dn10 + locals.var_t2__blk810_dn10)), (0.5 * (locals.var_t3__blk811_dn11 + locals.var_t2__blk810_dn11)), (0.5 * (locals.var_t3__blk811_dn12 + locals.var_t2__blk810_dn12)),)
    } else {
        (locals.var_t4__blk812, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    }
};
        locals.var_t4__blk812 = assign28400_e26365;
        locals.var_t4__blk812_dn3 = assign28400_e26365_d_n3;
        locals.var_t4__blk812_dn4 = assign28400_e26365_d_n4;
        locals.var_t4__blk812_dn5 = assign28400_e26365_d_n5;
        locals.var_t4__blk812_dn6 = assign28400_e26365_d_n6;
        locals.var_t4__blk812_dn7 = assign28400_e26365_d_n7;
        locals.var_t4__blk812_dn8 = assign28400_e26365_d_n8;
        locals.var_t4__blk812_dn9 = assign28400_e26365_d_n9;
        locals.var_t4__blk812_dn10 = assign28400_e26365_d_n10;
        locals.var_t4__blk812_dn11 = assign28400_e26365_d_n11;
        locals.var_t4__blk812_dn12 = assign28400_e26365_d_n12;
        locals.var_t4__blk812_rv = 0.0;

        let (assign28410_e26374, assign28410_e26374_d_n3, assign28410_e26374_d_n4, assign28410_e26374_d_n5, assign28410_e26374_d_n6, assign28410_e26374_d_n7, assign28410_e26374_d_n8, assign28410_e26374_d_n9, assign28410_e26374_d_n10, assign28410_e26374_d_n11, assign28410_e26374_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) {
        let assign28410_e26372: f64 = (locals.var_tox + locals.var_tox);
        (assign28410_e26372, (locals.var_tox_dn3 + locals.var_tox_dn3), (locals.var_tox_dn4 + locals.var_tox_dn4), (locals.var_tox_dn5 + locals.var_tox_dn5), (locals.var_tox_dn6 + locals.var_tox_dn6), (locals.var_tox_dn7 + locals.var_tox_dn7), (locals.var_tox_dn8 + locals.var_tox_dn8), (locals.var_tox_dn9 + locals.var_tox_dn9), (locals.var_tox_dn10 + locals.var_tox_dn10), (locals.var_tox_dn11 + locals.var_tox_dn11), (locals.var_tox_dn12 + locals.var_tox_dn12),)
    } else {
        (locals.var_tox, locals.var_tox_dn3, locals.var_tox_dn4, locals.var_tox_dn5, locals.var_tox_dn6, locals.var_tox_dn7, locals.var_tox_dn8, locals.var_tox_dn9, locals.var_tox_dn10, locals.var_tox_dn11, locals.var_tox_dn12,)
    }
};
        locals.var_tox = assign28410_e26374;
        locals.var_tox_dn3 = assign28410_e26374_d_n3;
        locals.var_tox_dn4 = assign28410_e26374_d_n4;
        locals.var_tox_dn5 = assign28410_e26374_d_n5;
        locals.var_tox_dn6 = assign28410_e26374_d_n6;
        locals.var_tox_dn7 = assign28410_e26374_d_n7;
        locals.var_tox_dn8 = assign28410_e26374_d_n8;
        locals.var_tox_dn9 = assign28410_e26374_d_n9;
        locals.var_tox_dn10 = assign28410_e26374_d_n10;
        locals.var_tox_dn11 = assign28410_e26374_d_n11;
        locals.var_tox_dn12 = assign28410_e26374_d_n12;
        locals.var_tox_rv = 0.0;

        let (assign28420_e26385, assign28420_e26385_d_n3, assign28420_e26385_d_n4, assign28420_e26385_d_n5, assign28420_e26385_d_n6, assign28420_e26385_d_n7, assign28420_e26385_d_n8, assign28420_e26385_d_n9, assign28420_e26385_d_n10, assign28420_e26385_d_n11, assign28420_e26385_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) {
        let assign28420_e26381: f64 = (locals.var_vgsteff__blk840 + locals.var_t4__blk812);
        let assign28420_e26383: f64 = (assign28420_e26381 / locals.var_tox);
        (assign28420_e26383, ((((locals.var_vgsteff__blk840_dn3 + locals.var_t4__blk812_dn3) * locals.var_tox) - (assign28420_e26381 * locals.var_tox_dn3)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff__blk840_dn4 + locals.var_t4__blk812_dn4) * locals.var_tox) - (assign28420_e26381 * locals.var_tox_dn4)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff__blk840_dn5 + locals.var_t4__blk812_dn5) * locals.var_tox) - (assign28420_e26381 * locals.var_tox_dn5)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff__blk840_dn6 + locals.var_t4__blk812_dn6) * locals.var_tox) - (assign28420_e26381 * locals.var_tox_dn6)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff__blk840_dn7 + locals.var_t4__blk812_dn7) * locals.var_tox) - (assign28420_e26381 * locals.var_tox_dn7)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff__blk840_dn8 + locals.var_t4__blk812_dn8) * locals.var_tox) - (assign28420_e26381 * locals.var_tox_dn8)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff__blk840_dn9 + locals.var_t4__blk812_dn9) * locals.var_tox) - (assign28420_e26381 * locals.var_tox_dn9)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff__blk840_dn10 + locals.var_t4__blk812_dn10) * locals.var_tox) - (assign28420_e26381 * locals.var_tox_dn10)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff__blk840_dn11 + locals.var_t4__blk812_dn11) * locals.var_tox) - (assign28420_e26381 * locals.var_tox_dn11)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff__blk840_dn12 + locals.var_t4__blk812_dn12) * locals.var_tox) - (assign28420_e26381 * locals.var_tox_dn12)) / (locals.var_tox * locals.var_tox)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign28420_e26385;
        locals.var_t0__blk808_dn3 = assign28420_e26385_d_n3;
        locals.var_t0__blk808_dn4 = assign28420_e26385_d_n4;
        locals.var_t0__blk808_dn5 = assign28420_e26385_d_n5;
        locals.var_t0__blk808_dn6 = assign28420_e26385_d_n6;
        locals.var_t0__blk808_dn7 = assign28420_e26385_d_n7;
        locals.var_t0__blk808_dn8 = assign28420_e26385_d_n8;
        locals.var_t0__blk808_dn9 = assign28420_e26385_d_n9;
        locals.var_t0__blk808_dn10 = assign28420_e26385_d_n10;
        locals.var_t0__blk808_dn11 = assign28420_e26385_d_n11;
        locals.var_t0__blk808_dn12 = assign28420_e26385_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign28430_e26404, assign28430_e26404_d_n3, assign28430_e26404_d_n4, assign28430_e26404_d_n5, assign28430_e26404_d_n6, assign28430_e26404_d_n7, assign28430_e26404_d_n8, assign28430_e26404_d_n9, assign28430_e26404_d_n10, assign28430_e26404_d_n11, assign28430_e26404_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) {
        let assign28430_e26392: f64 = (p.p59 * 0.7);
        let (assign28430_e26400, assign28430_e26400_d_n3, assign28430_e26400_d_n4, assign28430_e26400_d_n5, assign28430_e26400_d_n6, assign28430_e26400_d_n7, assign28430_e26400_d_n8, assign28430_e26400_d_n9, assign28430_e26400_d_n10, assign28430_e26400_d_n11, assign28430_e26400_d_n12,) = {
            if (locals.var_t0__blk808 > 1e-38) {
                let assign28430_e26397: f64 = (locals.var_t0__blk808).ln();
                (assign28430_e26397, (locals.var_t0__blk808_dn3 / locals.var_t0__blk808), (locals.var_t0__blk808_dn4 / locals.var_t0__blk808), (locals.var_t0__blk808_dn5 / locals.var_t0__blk808), (locals.var_t0__blk808_dn6 / locals.var_t0__blk808), (locals.var_t0__blk808_dn7 / locals.var_t0__blk808), (locals.var_t0__blk808_dn8 / locals.var_t0__blk808), (locals.var_t0__blk808_dn9 / locals.var_t0__blk808), (locals.var_t0__blk808_dn10 / locals.var_t0__blk808), (locals.var_t0__blk808_dn11 / locals.var_t0__blk808), (locals.var_t0__blk808_dn12 / locals.var_t0__blk808),)
            } else {
                let assign28430_e26399: f64 = (-87.49823353377374);
                (assign28430_e26399, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign28430_e26401: f64 = (assign28430_e26392 * assign28430_e26400);
        let assign28430_e26402: f64 = (assign28430_e26401).exp();
        (assign28430_e26402, (assign28430_e26402 * (assign28430_e26392 * assign28430_e26400_d_n3)), (assign28430_e26402 * (assign28430_e26392 * assign28430_e26400_d_n4)), (assign28430_e26402 * (assign28430_e26392 * assign28430_e26400_d_n5)), (assign28430_e26402 * (assign28430_e26392 * assign28430_e26400_d_n6)), (assign28430_e26402 * (assign28430_e26392 * assign28430_e26400_d_n7)), (assign28430_e26402 * (assign28430_e26392 * assign28430_e26400_d_n8)), (assign28430_e26402 * (assign28430_e26392 * assign28430_e26400_d_n9)), (assign28430_e26402 * (assign28430_e26392 * assign28430_e26400_d_n10)), (assign28430_e26402 * (assign28430_e26392 * assign28430_e26400_d_n11)), (assign28430_e26402 * (assign28430_e26392 * assign28430_e26400_d_n12)),)
    } else {
        (locals.var_tmp__blk824, locals.var_tmp__blk824_dn3, locals.var_tmp__blk824_dn4, locals.var_tmp__blk824_dn5, locals.var_tmp__blk824_dn6, locals.var_tmp__blk824_dn7, locals.var_tmp__blk824_dn8, locals.var_tmp__blk824_dn9, locals.var_tmp__blk824_dn10, locals.var_tmp__blk824_dn11, locals.var_tmp__blk824_dn12,)
    }
};
        locals.var_tmp__blk824 = assign28430_e26404;
        locals.var_tmp__blk824_dn3 = assign28430_e26404_d_n3;
        locals.var_tmp__blk824_dn4 = assign28430_e26404_d_n4;
        locals.var_tmp__blk824_dn5 = assign28430_e26404_d_n5;
        locals.var_tmp__blk824_dn6 = assign28430_e26404_d_n6;
        locals.var_tmp__blk824_dn7 = assign28430_e26404_d_n7;
        locals.var_tmp__blk824_dn8 = assign28430_e26404_d_n8;
        locals.var_tmp__blk824_dn9 = assign28430_e26404_d_n9;
        locals.var_tmp__blk824_dn10 = assign28430_e26404_d_n10;
        locals.var_tmp__blk824_dn11 = assign28430_e26404_d_n11;
        locals.var_tmp__blk824_dn12 = assign28430_e26404_d_n12;
        locals.var_tmp__blk824_rv = 0.0;

        let (assign28440_e26413, assign28440_e26413_d_n3, assign28440_e26413_d_n4, assign28440_e26413_d_n5, assign28440_e26413_d_n6, assign28440_e26413_d_n7, assign28440_e26413_d_n8, assign28440_e26413_d_n9, assign28440_e26413_d_n10, assign28440_e26413_d_n11, assign28440_e26413_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) {
        let assign28440_e26411: f64 = (1.0 + locals.var_tmp__blk824);
        (assign28440_e26411, locals.var_tmp__blk824_dn3, locals.var_tmp__blk824_dn4, locals.var_tmp__blk824_dn5, locals.var_tmp__blk824_dn6, locals.var_tmp__blk824_dn7, locals.var_tmp__blk824_dn8, locals.var_tmp__blk824_dn9, locals.var_tmp__blk824_dn10, locals.var_tmp__blk824_dn11, locals.var_tmp__blk824_dn12,)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign28440_e26413;
        locals.var_t1__blk809_dn3 = assign28440_e26413_d_n3;
        locals.var_t1__blk809_dn4 = assign28440_e26413_d_n4;
        locals.var_t1__blk809_dn5 = assign28440_e26413_d_n5;
        locals.var_t1__blk809_dn6 = assign28440_e26413_d_n6;
        locals.var_t1__blk809_dn7 = assign28440_e26413_d_n7;
        locals.var_t1__blk809_dn8 = assign28440_e26413_d_n8;
        locals.var_t1__blk809_dn9 = assign28440_e26413_d_n9;
        locals.var_t1__blk809_dn10 = assign28440_e26413_d_n10;
        locals.var_t1__blk809_dn11 = assign28440_e26413_d_n11;
        locals.var_t1__blk809_dn12 = assign28440_e26413_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign28450_e26424, assign28450_e26424_d_n3, assign28450_e26424_d_n4, assign28450_e26424_d_n5, assign28450_e26424_d_n6, assign28450_e26424_d_n7, assign28450_e26424_d_n8, assign28450_e26424_d_n9, assign28450_e26424_d_n10, assign28450_e26424_d_n11, assign28450_e26424_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) {
        let assign28450_e26420: f64 = (p.p58 * 1.9e-9);
        let assign28450_e26422: f64 = (assign28450_e26420 / locals.var_t1__blk809);
        (assign28450_e26422, (-((assign28450_e26420 * locals.var_t1__blk809_dn3) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (-((assign28450_e26420 * locals.var_t1__blk809_dn4) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (-((assign28450_e26420 * locals.var_t1__blk809_dn5) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (-((assign28450_e26420 * locals.var_t1__blk809_dn6) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (-((assign28450_e26420 * locals.var_t1__blk809_dn7) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (-((assign28450_e26420 * locals.var_t1__blk809_dn8) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (-((assign28450_e26420 * locals.var_t1__blk809_dn9) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (-((assign28450_e26420 * locals.var_t1__blk809_dn10) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (-((assign28450_e26420 * locals.var_t1__blk809_dn11) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (-((assign28450_e26420 * locals.var_t1__blk809_dn12) / (locals.var_t1__blk809 * locals.var_t1__blk809))),)
    } else {
        (locals.var_tcen__blk964, locals.var_tcen__blk964_dn3, locals.var_tcen__blk964_dn4, locals.var_tcen__blk964_dn5, locals.var_tcen__blk964_dn6, locals.var_tcen__blk964_dn7, locals.var_tcen__blk964_dn8, locals.var_tcen__blk964_dn9, locals.var_tcen__blk964_dn10, locals.var_tcen__blk964_dn11, locals.var_tcen__blk964_dn12,)
    }
};
        locals.var_tcen__blk964 = assign28450_e26424;
        locals.var_tcen__blk964_dn3 = assign28450_e26424_d_n3;
        locals.var_tcen__blk964_dn4 = assign28450_e26424_d_n4;
        locals.var_tcen__blk964_dn5 = assign28450_e26424_d_n5;
        locals.var_tcen__blk964_dn6 = assign28450_e26424_d_n6;
        locals.var_tcen__blk964_dn7 = assign28450_e26424_d_n7;
        locals.var_tcen__blk964_dn8 = assign28450_e26424_d_n8;
        locals.var_tcen__blk964_dn9 = assign28450_e26424_d_n9;
        locals.var_tcen__blk964_dn10 = assign28450_e26424_d_n10;
        locals.var_tcen__blk964_dn11 = assign28450_e26424_d_n11;
        locals.var_tcen__blk964_dn12 = assign28450_e26424_d_n12;
        locals.var_tcen__blk964_rv = 0.0;

        let (assign28460_e26433, assign28460_e26433_d_n3, assign28460_e26433_d_n4, assign28460_e26433_d_n5, assign28460_e26433_d_n6, assign28460_e26433_d_n7, assign28460_e26433_d_n8, assign28460_e26433_d_n9, assign28460_e26433_d_n10, assign28460_e26433_d_n11, assign28460_e26433_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) {
        let assign28460_e26431: f64 = (locals.var_epssub / locals.var_tcen__blk964);
        (assign28460_e26431, (-((locals.var_epssub * locals.var_tcen__blk964_dn3) / (locals.var_tcen__blk964 * locals.var_tcen__blk964))), (-((locals.var_epssub * locals.var_tcen__blk964_dn4) / (locals.var_tcen__blk964 * locals.var_tcen__blk964))), (-((locals.var_epssub * locals.var_tcen__blk964_dn5) / (locals.var_tcen__blk964 * locals.var_tcen__blk964))), (-((locals.var_epssub * locals.var_tcen__blk964_dn6) / (locals.var_tcen__blk964 * locals.var_tcen__blk964))), (-((locals.var_epssub * locals.var_tcen__blk964_dn7) / (locals.var_tcen__blk964 * locals.var_tcen__blk964))), (-((locals.var_epssub * locals.var_tcen__blk964_dn8) / (locals.var_tcen__blk964 * locals.var_tcen__blk964))), (-((locals.var_epssub * locals.var_tcen__blk964_dn9) / (locals.var_tcen__blk964 * locals.var_tcen__blk964))), (-((locals.var_epssub * locals.var_tcen__blk964_dn10) / (locals.var_tcen__blk964 * locals.var_tcen__blk964))), (-((locals.var_epssub * locals.var_tcen__blk964_dn11) / (locals.var_tcen__blk964 * locals.var_tcen__blk964))), (-((locals.var_epssub * locals.var_tcen__blk964_dn12) / (locals.var_tcen__blk964 * locals.var_tcen__blk964))),)
    } else {
        (locals.var_ccen, locals.var_ccen_dn3, locals.var_ccen_dn4, locals.var_ccen_dn5, locals.var_ccen_dn6, locals.var_ccen_dn7, locals.var_ccen_dn8, locals.var_ccen_dn9, locals.var_ccen_dn10, locals.var_ccen_dn11, locals.var_ccen_dn12,)
    }
};
        locals.var_ccen = assign28460_e26433;
        locals.var_ccen_dn3 = assign28460_e26433_d_n3;
        locals.var_ccen_dn4 = assign28460_e26433_d_n4;
        locals.var_ccen_dn5 = assign28460_e26433_d_n5;
        locals.var_ccen_dn6 = assign28460_e26433_d_n6;
        locals.var_ccen_dn7 = assign28460_e26433_d_n7;
        locals.var_ccen_dn8 = assign28460_e26433_d_n8;
        locals.var_ccen_dn9 = assign28460_e26433_d_n9;
        locals.var_ccen_dn10 = assign28460_e26433_d_n10;
        locals.var_ccen_dn11 = assign28460_e26433_d_n11;
        locals.var_ccen_dn12 = assign28460_e26433_d_n12;
        locals.var_ccen_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_87(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign28470_e26444, assign28470_e26444_d_n3, assign28470_e26444_d_n4, assign28470_e26444_d_n5, assign28470_e26444_d_n6, assign28470_e26444_d_n7, assign28470_e26444_d_n8, assign28470_e26444_d_n9, assign28470_e26444_d_n10, assign28470_e26444_d_n11, assign28470_e26444_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) {
        let assign28470_e26441: f64 = (locals.var_cox + locals.var_ccen);
        let assign28470_e26442: f64 = (locals.var_cox / assign28470_e26441);
        (assign28470_e26442, (((locals.var_cox_dn3 * assign28470_e26441) - (locals.var_cox * (locals.var_cox_dn3 + locals.var_ccen_dn3))) / (assign28470_e26441 * assign28470_e26441)), (((locals.var_cox_dn4 * assign28470_e26441) - (locals.var_cox * (locals.var_cox_dn4 + locals.var_ccen_dn4))) / (assign28470_e26441 * assign28470_e26441)), (((locals.var_cox_dn5 * assign28470_e26441) - (locals.var_cox * (locals.var_cox_dn5 + locals.var_ccen_dn5))) / (assign28470_e26441 * assign28470_e26441)), (((locals.var_cox_dn6 * assign28470_e26441) - (locals.var_cox * (locals.var_cox_dn6 + locals.var_ccen_dn6))) / (assign28470_e26441 * assign28470_e26441)), (((locals.var_cox_dn7 * assign28470_e26441) - (locals.var_cox * (locals.var_cox_dn7 + locals.var_ccen_dn7))) / (assign28470_e26441 * assign28470_e26441)), (((locals.var_cox_dn8 * assign28470_e26441) - (locals.var_cox * (locals.var_cox_dn8 + locals.var_ccen_dn8))) / (assign28470_e26441 * assign28470_e26441)), (((locals.var_cox_dn9 * assign28470_e26441) - (locals.var_cox * (locals.var_cox_dn9 + locals.var_ccen_dn9))) / (assign28470_e26441 * assign28470_e26441)), (((locals.var_cox_dn10 * assign28470_e26441) - (locals.var_cox * (locals.var_cox_dn10 + locals.var_ccen_dn10))) / (assign28470_e26441 * assign28470_e26441)), (((locals.var_cox_dn11 * assign28470_e26441) - (locals.var_cox * (locals.var_cox_dn11 + locals.var_ccen_dn11))) / (assign28470_e26441 * assign28470_e26441)), (((locals.var_cox_dn12 * assign28470_e26441) - (locals.var_cox * (locals.var_cox_dn12 + locals.var_ccen_dn12))) / (assign28470_e26441 * assign28470_e26441)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign28470_e26444;
        locals.var_t0__blk808_dn3 = assign28470_e26444_d_n3;
        locals.var_t0__blk808_dn4 = assign28470_e26444_d_n4;
        locals.var_t0__blk808_dn5 = assign28470_e26444_d_n5;
        locals.var_t0__blk808_dn6 = assign28470_e26444_d_n6;
        locals.var_t0__blk808_dn7 = assign28470_e26444_d_n7;
        locals.var_t0__blk808_dn8 = assign28470_e26444_d_n8;
        locals.var_t0__blk808_dn9 = assign28470_e26444_d_n9;
        locals.var_t0__blk808_dn10 = assign28470_e26444_d_n10;
        locals.var_t0__blk808_dn11 = assign28470_e26444_d_n11;
        locals.var_t0__blk808_dn12 = assign28470_e26444_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign28480_e26453, assign28480_e26453_d_n3, assign28480_e26453_d_n4, assign28480_e26453_d_n5, assign28480_e26453_d_n6, assign28480_e26453_d_n7, assign28480_e26453_d_n8, assign28480_e26453_d_n9, assign28480_e26453_d_n10, assign28480_e26453_d_n11, assign28480_e26453_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) {
        let assign28480_e26451: f64 = (locals.var_t0__blk808 * locals.var_ccen);
        (assign28480_e26451, ((locals.var_t0__blk808_dn3 * locals.var_ccen) + (locals.var_t0__blk808 * locals.var_ccen_dn3)), ((locals.var_t0__blk808_dn4 * locals.var_ccen) + (locals.var_t0__blk808 * locals.var_ccen_dn4)), ((locals.var_t0__blk808_dn5 * locals.var_ccen) + (locals.var_t0__blk808 * locals.var_ccen_dn5)), ((locals.var_t0__blk808_dn6 * locals.var_ccen) + (locals.var_t0__blk808 * locals.var_ccen_dn6)), ((locals.var_t0__blk808_dn7 * locals.var_ccen) + (locals.var_t0__blk808 * locals.var_ccen_dn7)), ((locals.var_t0__blk808_dn8 * locals.var_ccen) + (locals.var_t0__blk808 * locals.var_ccen_dn8)), ((locals.var_t0__blk808_dn9 * locals.var_ccen) + (locals.var_t0__blk808 * locals.var_ccen_dn9)), ((locals.var_t0__blk808_dn10 * locals.var_ccen) + (locals.var_t0__blk808 * locals.var_ccen_dn10)), ((locals.var_t0__blk808_dn11 * locals.var_ccen) + (locals.var_t0__blk808 * locals.var_ccen_dn11)), ((locals.var_t0__blk808_dn12 * locals.var_ccen) + (locals.var_t0__blk808 * locals.var_ccen_dn12)),)
    } else {
        (locals.var_coxeff, locals.var_coxeff_dn3, locals.var_coxeff_dn4, locals.var_coxeff_dn5, locals.var_coxeff_dn6, locals.var_coxeff_dn7, locals.var_coxeff_dn8, locals.var_coxeff_dn9, locals.var_coxeff_dn10, locals.var_coxeff_dn11, locals.var_coxeff_dn12,)
    }
};
        locals.var_coxeff = assign28480_e26453;
        locals.var_coxeff_dn3 = assign28480_e26453_d_n3;
        locals.var_coxeff_dn4 = assign28480_e26453_d_n4;
        locals.var_coxeff_dn5 = assign28480_e26453_d_n5;
        locals.var_coxeff_dn6 = assign28480_e26453_d_n6;
        locals.var_coxeff_dn7 = assign28480_e26453_d_n7;
        locals.var_coxeff_dn8 = assign28480_e26453_d_n8;
        locals.var_coxeff_dn9 = assign28480_e26453_d_n9;
        locals.var_coxeff_dn10 = assign28480_e26453_d_n10;
        locals.var_coxeff_dn11 = assign28480_e26453_d_n11;
        locals.var_coxeff_dn12 = assign28480_e26453_d_n12;
        locals.var_coxeff_rv = 0.0;

        let (assign28490_e26464, assign28490_e26464_d_n3, assign28490_e26464_d_n4, assign28490_e26464_d_n5, assign28490_e26464_d_n6, assign28490_e26464_d_n7, assign28490_e26464_d_n8, assign28490_e26464_d_n9, assign28490_e26464_d_n10, assign28490_e26464_d_n11, assign28490_e26464_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) {
        let assign28490_e26460: f64 = (locals.var_coxwl * locals.var_coxeff);
        let assign28490_e26462: f64 = (assign28490_e26460 / locals.var_cox);
        (assign28490_e26462, (((((locals.var_coxwl_dn3 * locals.var_coxeff) + (locals.var_coxwl * locals.var_coxeff_dn3)) * locals.var_cox) - (assign28490_e26460 * locals.var_cox_dn3)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl_dn4 * locals.var_coxeff) + (locals.var_coxwl * locals.var_coxeff_dn4)) * locals.var_cox) - (assign28490_e26460 * locals.var_cox_dn4)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl_dn5 * locals.var_coxeff) + (locals.var_coxwl * locals.var_coxeff_dn5)) * locals.var_cox) - (assign28490_e26460 * locals.var_cox_dn5)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl_dn6 * locals.var_coxeff) + (locals.var_coxwl * locals.var_coxeff_dn6)) * locals.var_cox) - (assign28490_e26460 * locals.var_cox_dn6)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl_dn7 * locals.var_coxeff) + (locals.var_coxwl * locals.var_coxeff_dn7)) * locals.var_cox) - (assign28490_e26460 * locals.var_cox_dn7)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl_dn8 * locals.var_coxeff) + (locals.var_coxwl * locals.var_coxeff_dn8)) * locals.var_cox) - (assign28490_e26460 * locals.var_cox_dn8)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl_dn9 * locals.var_coxeff) + (locals.var_coxwl * locals.var_coxeff_dn9)) * locals.var_cox) - (assign28490_e26460 * locals.var_cox_dn9)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl_dn10 * locals.var_coxeff) + (locals.var_coxwl * locals.var_coxeff_dn10)) * locals.var_cox) - (assign28490_e26460 * locals.var_cox_dn10)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl_dn11 * locals.var_coxeff) + (locals.var_coxwl * locals.var_coxeff_dn11)) * locals.var_cox) - (assign28490_e26460 * locals.var_cox_dn11)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl_dn12 * locals.var_coxeff) + (locals.var_coxwl * locals.var_coxeff_dn12)) * locals.var_cox) - (assign28490_e26460 * locals.var_cox_dn12)) / (locals.var_cox * locals.var_cox)),)
    } else {
        (locals.var_coxwlcen, locals.var_coxwlcen_dn3, locals.var_coxwlcen_dn4, locals.var_coxwlcen_dn5, locals.var_coxwlcen_dn6, locals.var_coxwlcen_dn7, locals.var_coxwlcen_dn8, locals.var_coxwlcen_dn9, locals.var_coxwlcen_dn10, locals.var_coxwlcen_dn11, locals.var_coxwlcen_dn12,)
    }
};
        locals.var_coxwlcen = assign28490_e26464;
        locals.var_coxwlcen_dn3 = assign28490_e26464_d_n3;
        locals.var_coxwlcen_dn4 = assign28490_e26464_d_n4;
        locals.var_coxwlcen_dn5 = assign28490_e26464_d_n5;
        locals.var_coxwlcen_dn6 = assign28490_e26464_d_n6;
        locals.var_coxwlcen_dn7 = assign28490_e26464_d_n7;
        locals.var_coxwlcen_dn8 = assign28490_e26464_d_n8;
        locals.var_coxwlcen_dn9 = assign28490_e26464_d_n9;
        locals.var_coxwlcen_dn10 = assign28490_e26464_d_n10;
        locals.var_coxwlcen_dn11 = assign28490_e26464_d_n11;
        locals.var_coxwlcen_dn12 = assign28490_e26464_d_n12;
        locals.var_coxwlcen_rv = 0.0;

        let (assign28500_e26475, assign28500_e26475_d_n3, assign28500_e26475_d_n4, assign28500_e26475_d_n5, assign28500_e26475_d_n6, assign28500_e26475_d_n7, assign28500_e26475_d_n8, assign28500_e26475_d_n9, assign28500_e26475_d_n10, assign28500_e26475_d_n11, assign28500_e26475_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) {
        let assign28500_e26471: f64 = (locals.var_coxwlb * locals.var_coxeff);
        let assign28500_e26473: f64 = (assign28500_e26471 / locals.var_cox);
        (assign28500_e26473, (((((locals.var_coxwlb_dn3 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn3)) * locals.var_cox) - (assign28500_e26471 * locals.var_cox_dn3)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn4 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn4)) * locals.var_cox) - (assign28500_e26471 * locals.var_cox_dn4)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn5 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn5)) * locals.var_cox) - (assign28500_e26471 * locals.var_cox_dn5)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn6 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn6)) * locals.var_cox) - (assign28500_e26471 * locals.var_cox_dn6)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn7 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn7)) * locals.var_cox) - (assign28500_e26471 * locals.var_cox_dn7)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn8 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn8)) * locals.var_cox) - (assign28500_e26471 * locals.var_cox_dn8)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn9 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn9)) * locals.var_cox) - (assign28500_e26471 * locals.var_cox_dn9)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn10 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn10)) * locals.var_cox) - (assign28500_e26471 * locals.var_cox_dn10)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn11 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn11)) * locals.var_cox) - (assign28500_e26471 * locals.var_cox_dn11)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb_dn12 * locals.var_coxeff) + (locals.var_coxwlb * locals.var_coxeff_dn12)) * locals.var_cox) - (assign28500_e26471 * locals.var_cox_dn12)) / (locals.var_cox * locals.var_cox)),)
    } else {
        (locals.var_coxwlcenb, locals.var_coxwlcenb_dn3, locals.var_coxwlcenb_dn4, locals.var_coxwlcenb_dn5, locals.var_coxwlcenb_dn6, locals.var_coxwlcenb_dn7, locals.var_coxwlcenb_dn8, locals.var_coxwlcenb_dn9, locals.var_coxwlcenb_dn10, locals.var_coxwlcenb_dn11, locals.var_coxwlcenb_dn12,)
    }
};
        locals.var_coxwlcenb = assign28500_e26475;
        locals.var_coxwlcenb_dn3 = assign28500_e26475_d_n3;
        locals.var_coxwlcenb_dn4 = assign28500_e26475_d_n4;
        locals.var_coxwlcenb_dn5 = assign28500_e26475_d_n5;
        locals.var_coxwlcenb_dn6 = assign28500_e26475_d_n6;
        locals.var_coxwlcenb_dn7 = assign28500_e26475_d_n7;
        locals.var_coxwlcenb_dn8 = assign28500_e26475_d_n8;
        locals.var_coxwlcenb_dn9 = assign28500_e26475_d_n9;
        locals.var_coxwlcenb_dn10 = assign28500_e26475_d_n10;
        locals.var_coxwlcenb_dn11 = assign28500_e26475_d_n11;
        locals.var_coxwlcenb_dn12 = assign28500_e26475_d_n12;
        locals.var_coxwlcenb_rv = 0.0;

        let assign28510_e26486: f64 = if (((locals.var_b4soisoimod != 2.0) && (locals.var_b4soibodymod != 0.0)) && (p.p27 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1410 = assign28510_e26486;
        locals.var_guard1410_rv = 0.0;

        let (assign28520_e26503, assign28520_e26503_d_n3, assign28520_e26503_d_n4, assign28520_e26503_d_n5, assign28520_e26503_d_n6, assign28520_e26503_d_n7, assign28520_e26503_d_n8, assign28520_e26503_d_n9, assign28520_e26503_d_n10, assign28520_e26503_d_n11, assign28520_e26503_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1410 != 0.0)) {
        let assign28520_e26496: f64 = (locals.var_vth__blk794 + p.p1033);
        let assign28520_e26498: f64 = (assign28520_e26496 - locals.var_vfbzb2);
        let assign28520_e26500: f64 = (assign28520_e26498 - locals.var_phi);
        let assign28520_e26501: f64 = (4.0 * assign28520_e26500);
        (assign28520_e26501, (4.0 * ((locals.var_vth__blk794_dn3 - locals.var_vfbzb2_dn3) - locals.var_phi_dn3)), (4.0 * ((locals.var_vth__blk794_dn4 - locals.var_vfbzb2_dn4) - locals.var_phi_dn4)), (4.0 * ((locals.var_vth__blk794_dn5 - locals.var_vfbzb2_dn5) - locals.var_phi_dn5)), (4.0 * ((locals.var_vth__blk794_dn6 - locals.var_vfbzb2_dn6) - locals.var_phi_dn6)), (4.0 * ((locals.var_vth__blk794_dn7 - locals.var_vfbzb2_dn7) - locals.var_phi_dn7)), (4.0 * ((locals.var_vth__blk794_dn8 - locals.var_vfbzb2_dn8) - locals.var_phi_dn8)), (4.0 * ((locals.var_vth__blk794_dn9 - locals.var_vfbzb2_dn9) - locals.var_phi_dn9)), (4.0 * ((locals.var_vth__blk794_dn10 - locals.var_vfbzb2_dn10) - locals.var_phi_dn10)), (4.0 * ((locals.var_vth__blk794_dn11 - locals.var_vfbzb2_dn11) - locals.var_phi_dn11)), (4.0 * ((locals.var_vth__blk794_dn12 - locals.var_vfbzb2_dn12) - locals.var_phi_dn12)),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign28520_e26503;
        locals.var_t3__blk811_dn3 = assign28520_e26503_d_n3;
        locals.var_t3__blk811_dn4 = assign28520_e26503_d_n4;
        locals.var_t3__blk811_dn5 = assign28520_e26503_d_n5;
        locals.var_t3__blk811_dn6 = assign28520_e26503_d_n6;
        locals.var_t3__blk811_dn7 = assign28520_e26503_d_n7;
        locals.var_t3__blk811_dn8 = assign28520_e26503_d_n8;
        locals.var_t3__blk811_dn9 = assign28520_e26503_d_n9;
        locals.var_t3__blk811_dn10 = assign28520_e26503_d_n10;
        locals.var_t3__blk811_dn11 = assign28520_e26503_d_n11;
        locals.var_t3__blk811_dn12 = assign28520_e26503_d_n12;
        locals.var_t3__blk811_rv = 0.0;

        let (assign28530_e26517, assign28530_e26517_d_n3, assign28530_e26517_d_n4, assign28530_e26517_d_n5, assign28530_e26517_d_n6, assign28530_e26517_d_n7, assign28530_e26517_d_n8, assign28530_e26517_d_n9, assign28530_e26517_d_n10, assign28530_e26517_d_n11, assign28530_e26517_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1410 != 0.0)) {
        let assign28530_e26512: f64 = (locals.var_t3__blk811 * locals.var_t3__blk811);
        let assign28530_e26514: f64 = (assign28530_e26512 + 0.0001);
        let assign28530_e26515: f64 = (assign28530_e26514).sqrt();
        (assign28530_e26515, (((locals.var_t3__blk811_dn3 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn3)) / (2.0 * assign28530_e26515)), (((locals.var_t3__blk811_dn4 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn4)) / (2.0 * assign28530_e26515)), (((locals.var_t3__blk811_dn5 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn5)) / (2.0 * assign28530_e26515)), (((locals.var_t3__blk811_dn6 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn6)) / (2.0 * assign28530_e26515)), (((locals.var_t3__blk811_dn7 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn7)) / (2.0 * assign28530_e26515)), (((locals.var_t3__blk811_dn8 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn8)) / (2.0 * assign28530_e26515)), (((locals.var_t3__blk811_dn9 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn9)) / (2.0 * assign28530_e26515)), (((locals.var_t3__blk811_dn10 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn10)) / (2.0 * assign28530_e26515)), (((locals.var_t3__blk811_dn11 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn11)) / (2.0 * assign28530_e26515)), (((locals.var_t3__blk811_dn12 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn12)) / (2.0 * assign28530_e26515)),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign28530_e26517;
        locals.var_t2__blk810_dn3 = assign28530_e26517_d_n3;
        locals.var_t2__blk810_dn4 = assign28530_e26517_d_n4;
        locals.var_t2__blk810_dn5 = assign28530_e26517_d_n5;
        locals.var_t2__blk810_dn6 = assign28530_e26517_d_n6;
        locals.var_t2__blk810_dn7 = assign28530_e26517_d_n7;
        locals.var_t2__blk810_dn8 = assign28530_e26517_d_n8;
        locals.var_t2__blk810_dn9 = assign28530_e26517_d_n9;
        locals.var_t2__blk810_dn10 = assign28530_e26517_d_n10;
        locals.var_t2__blk810_dn11 = assign28530_e26517_d_n11;
        locals.var_t2__blk810_dn12 = assign28530_e26517_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign28540_e26530, assign28540_e26530_d_n3, assign28540_e26530_d_n4, assign28540_e26530_d_n5, assign28540_e26530_d_n6, assign28540_e26530_d_n7, assign28540_e26530_d_n8, assign28540_e26530_d_n9, assign28540_e26530_d_n10, assign28540_e26530_d_n11, assign28540_e26530_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1410 != 0.0)) {
        let assign28540_e26527: f64 = (locals.var_t3__blk811 + locals.var_t2__blk810);
        let assign28540_e26528: f64 = (0.5 * assign28540_e26527);
        (assign28540_e26528, (0.5 * (locals.var_t3__blk811_dn3 + locals.var_t2__blk810_dn3)), (0.5 * (locals.var_t3__blk811_dn4 + locals.var_t2__blk810_dn4)), (0.5 * (locals.var_t3__blk811_dn5 + locals.var_t2__blk810_dn5)), (0.5 * (locals.var_t3__blk811_dn6 + locals.var_t2__blk810_dn6)), (0.5 * (locals.var_t3__blk811_dn7 + locals.var_t2__blk810_dn7)), (0.5 * (locals.var_t3__blk811_dn8 + locals.var_t2__blk810_dn8)), (0.5 * (locals.var_t3__blk811_dn9 + locals.var_t2__blk810_dn9)), (0.5 * (locals.var_t3__blk811_dn10 + locals.var_t2__blk810_dn10)), (0.5 * (locals.var_t3__blk811_dn11 + locals.var_t2__blk810_dn11)), (0.5 * (locals.var_t3__blk811_dn12 + locals.var_t2__blk810_dn12)),)
    } else {
        (locals.var_t4__blk812, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    }
};
        locals.var_t4__blk812 = assign28540_e26530;
        locals.var_t4__blk812_dn3 = assign28540_e26530_d_n3;
        locals.var_t4__blk812_dn4 = assign28540_e26530_d_n4;
        locals.var_t4__blk812_dn5 = assign28540_e26530_d_n5;
        locals.var_t4__blk812_dn6 = assign28540_e26530_d_n6;
        locals.var_t4__blk812_dn7 = assign28540_e26530_d_n7;
        locals.var_t4__blk812_dn8 = assign28540_e26530_d_n8;
        locals.var_t4__blk812_dn9 = assign28540_e26530_d_n9;
        locals.var_t4__blk812_dn10 = assign28540_e26530_d_n10;
        locals.var_t4__blk812_dn11 = assign28540_e26530_d_n11;
        locals.var_t4__blk812_dn12 = assign28540_e26530_d_n12;
        locals.var_t4__blk812_rv = 0.0;

        let (assign28550_e26543, assign28550_e26543_d_n3, assign28550_e26543_d_n4, assign28550_e26543_d_n5, assign28550_e26543_d_n6, assign28550_e26543_d_n7, assign28550_e26543_d_n8, assign28550_e26543_d_n9, assign28550_e26543_d_n10, assign28550_e26543_d_n11, assign28550_e26543_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1410 != 0.0)) {
        let assign28550_e26539: f64 = (locals.var_vgsteff2 + locals.var_t4__blk812);
        let assign28550_e26541: f64 = (assign28550_e26539 / locals.var_tox);
        (assign28550_e26541, ((((locals.var_vgsteff2_dn3 + locals.var_t4__blk812_dn3) * locals.var_tox) - (assign28550_e26539 * locals.var_tox_dn3)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff2_dn4 + locals.var_t4__blk812_dn4) * locals.var_tox) - (assign28550_e26539 * locals.var_tox_dn4)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff2_dn5 + locals.var_t4__blk812_dn5) * locals.var_tox) - (assign28550_e26539 * locals.var_tox_dn5)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff2_dn6 + locals.var_t4__blk812_dn6) * locals.var_tox) - (assign28550_e26539 * locals.var_tox_dn6)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff2_dn7 + locals.var_t4__blk812_dn7) * locals.var_tox) - (assign28550_e26539 * locals.var_tox_dn7)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff2_dn8 + locals.var_t4__blk812_dn8) * locals.var_tox) - (assign28550_e26539 * locals.var_tox_dn8)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff2_dn9 + locals.var_t4__blk812_dn9) * locals.var_tox) - (assign28550_e26539 * locals.var_tox_dn9)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff2_dn10 + locals.var_t4__blk812_dn10) * locals.var_tox) - (assign28550_e26539 * locals.var_tox_dn10)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff2_dn11 + locals.var_t4__blk812_dn11) * locals.var_tox) - (assign28550_e26539 * locals.var_tox_dn11)) / (locals.var_tox * locals.var_tox)), ((((locals.var_vgsteff2_dn12 + locals.var_t4__blk812_dn12) * locals.var_tox) - (assign28550_e26539 * locals.var_tox_dn12)) / (locals.var_tox * locals.var_tox)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign28550_e26543;
        locals.var_t0__blk808_dn3 = assign28550_e26543_d_n3;
        locals.var_t0__blk808_dn4 = assign28550_e26543_d_n4;
        locals.var_t0__blk808_dn5 = assign28550_e26543_d_n5;
        locals.var_t0__blk808_dn6 = assign28550_e26543_d_n6;
        locals.var_t0__blk808_dn7 = assign28550_e26543_d_n7;
        locals.var_t0__blk808_dn8 = assign28550_e26543_d_n8;
        locals.var_t0__blk808_dn9 = assign28550_e26543_d_n9;
        locals.var_t0__blk808_dn10 = assign28550_e26543_d_n10;
        locals.var_t0__blk808_dn11 = assign28550_e26543_d_n11;
        locals.var_t0__blk808_dn12 = assign28550_e26543_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign28560_e26564, assign28560_e26564_d_n3, assign28560_e26564_d_n4, assign28560_e26564_d_n5, assign28560_e26564_d_n6, assign28560_e26564_d_n7, assign28560_e26564_d_n8, assign28560_e26564_d_n9, assign28560_e26564_d_n10, assign28560_e26564_d_n11, assign28560_e26564_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1410 != 0.0)) {
        let assign28560_e26552: f64 = (p.p59 * 0.7);
        let (assign28560_e26560, assign28560_e26560_d_n3, assign28560_e26560_d_n4, assign28560_e26560_d_n5, assign28560_e26560_d_n6, assign28560_e26560_d_n7, assign28560_e26560_d_n8, assign28560_e26560_d_n9, assign28560_e26560_d_n10, assign28560_e26560_d_n11, assign28560_e26560_d_n12,) = {
            if (locals.var_t0__blk808 > 1e-38) {
                let assign28560_e26557: f64 = (locals.var_t0__blk808).ln();
                (assign28560_e26557, (locals.var_t0__blk808_dn3 / locals.var_t0__blk808), (locals.var_t0__blk808_dn4 / locals.var_t0__blk808), (locals.var_t0__blk808_dn5 / locals.var_t0__blk808), (locals.var_t0__blk808_dn6 / locals.var_t0__blk808), (locals.var_t0__blk808_dn7 / locals.var_t0__blk808), (locals.var_t0__blk808_dn8 / locals.var_t0__blk808), (locals.var_t0__blk808_dn9 / locals.var_t0__blk808), (locals.var_t0__blk808_dn10 / locals.var_t0__blk808), (locals.var_t0__blk808_dn11 / locals.var_t0__blk808), (locals.var_t0__blk808_dn12 / locals.var_t0__blk808),)
            } else {
                let assign28560_e26559: f64 = (-87.49823353377374);
                (assign28560_e26559, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign28560_e26561: f64 = (assign28560_e26552 * assign28560_e26560);
        let assign28560_e26562: f64 = (assign28560_e26561).exp();
        (assign28560_e26562, (assign28560_e26562 * (assign28560_e26552 * assign28560_e26560_d_n3)), (assign28560_e26562 * (assign28560_e26552 * assign28560_e26560_d_n4)), (assign28560_e26562 * (assign28560_e26552 * assign28560_e26560_d_n5)), (assign28560_e26562 * (assign28560_e26552 * assign28560_e26560_d_n6)), (assign28560_e26562 * (assign28560_e26552 * assign28560_e26560_d_n7)), (assign28560_e26562 * (assign28560_e26552 * assign28560_e26560_d_n8)), (assign28560_e26562 * (assign28560_e26552 * assign28560_e26560_d_n9)), (assign28560_e26562 * (assign28560_e26552 * assign28560_e26560_d_n10)), (assign28560_e26562 * (assign28560_e26552 * assign28560_e26560_d_n11)), (assign28560_e26562 * (assign28560_e26552 * assign28560_e26560_d_n12)),)
    } else {
        (locals.var_tmp__blk824, locals.var_tmp__blk824_dn3, locals.var_tmp__blk824_dn4, locals.var_tmp__blk824_dn5, locals.var_tmp__blk824_dn6, locals.var_tmp__blk824_dn7, locals.var_tmp__blk824_dn8, locals.var_tmp__blk824_dn9, locals.var_tmp__blk824_dn10, locals.var_tmp__blk824_dn11, locals.var_tmp__blk824_dn12,)
    }
};
        locals.var_tmp__blk824 = assign28560_e26564;
        locals.var_tmp__blk824_dn3 = assign28560_e26564_d_n3;
        locals.var_tmp__blk824_dn4 = assign28560_e26564_d_n4;
        locals.var_tmp__blk824_dn5 = assign28560_e26564_d_n5;
        locals.var_tmp__blk824_dn6 = assign28560_e26564_d_n6;
        locals.var_tmp__blk824_dn7 = assign28560_e26564_d_n7;
        locals.var_tmp__blk824_dn8 = assign28560_e26564_d_n8;
        locals.var_tmp__blk824_dn9 = assign28560_e26564_d_n9;
        locals.var_tmp__blk824_dn10 = assign28560_e26564_d_n10;
        locals.var_tmp__blk824_dn11 = assign28560_e26564_d_n11;
        locals.var_tmp__blk824_dn12 = assign28560_e26564_d_n12;
        locals.var_tmp__blk824_rv = 0.0;

        let (assign28570_e26575, assign28570_e26575_d_n3, assign28570_e26575_d_n4, assign28570_e26575_d_n5, assign28570_e26575_d_n6, assign28570_e26575_d_n7, assign28570_e26575_d_n8, assign28570_e26575_d_n9, assign28570_e26575_d_n10, assign28570_e26575_d_n11, assign28570_e26575_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1410 != 0.0)) {
        let assign28570_e26573: f64 = (1.0 + locals.var_tmp__blk824);
        (assign28570_e26573, locals.var_tmp__blk824_dn3, locals.var_tmp__blk824_dn4, locals.var_tmp__blk824_dn5, locals.var_tmp__blk824_dn6, locals.var_tmp__blk824_dn7, locals.var_tmp__blk824_dn8, locals.var_tmp__blk824_dn9, locals.var_tmp__blk824_dn10, locals.var_tmp__blk824_dn11, locals.var_tmp__blk824_dn12,)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign28570_e26575;
        locals.var_t1__blk809_dn3 = assign28570_e26575_d_n3;
        locals.var_t1__blk809_dn4 = assign28570_e26575_d_n4;
        locals.var_t1__blk809_dn5 = assign28570_e26575_d_n5;
        locals.var_t1__blk809_dn6 = assign28570_e26575_d_n6;
        locals.var_t1__blk809_dn7 = assign28570_e26575_d_n7;
        locals.var_t1__blk809_dn8 = assign28570_e26575_d_n8;
        locals.var_t1__blk809_dn9 = assign28570_e26575_d_n9;
        locals.var_t1__blk809_dn10 = assign28570_e26575_d_n10;
        locals.var_t1__blk809_dn11 = assign28570_e26575_d_n11;
        locals.var_t1__blk809_dn12 = assign28570_e26575_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign28580_e26588, assign28580_e26588_d_n3, assign28580_e26588_d_n4, assign28580_e26588_d_n5, assign28580_e26588_d_n6, assign28580_e26588_d_n7, assign28580_e26588_d_n8, assign28580_e26588_d_n9, assign28580_e26588_d_n10, assign28580_e26588_d_n11, assign28580_e26588_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1410 != 0.0)) {
        let assign28580_e26584: f64 = (p.p58 * 1.9e-9);
        let assign28580_e26586: f64 = (assign28580_e26584 / locals.var_t1__blk809);
        (assign28580_e26586, (-((assign28580_e26584 * locals.var_t1__blk809_dn3) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (-((assign28580_e26584 * locals.var_t1__blk809_dn4) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (-((assign28580_e26584 * locals.var_t1__blk809_dn5) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (-((assign28580_e26584 * locals.var_t1__blk809_dn6) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (-((assign28580_e26584 * locals.var_t1__blk809_dn7) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (-((assign28580_e26584 * locals.var_t1__blk809_dn8) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (-((assign28580_e26584 * locals.var_t1__blk809_dn9) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (-((assign28580_e26584 * locals.var_t1__blk809_dn10) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (-((assign28580_e26584 * locals.var_t1__blk809_dn11) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (-((assign28580_e26584 * locals.var_t1__blk809_dn12) / (locals.var_t1__blk809 * locals.var_t1__blk809))),)
    } else {
        (locals.var_tcen2, locals.var_tcen2_dn3, locals.var_tcen2_dn4, locals.var_tcen2_dn5, locals.var_tcen2_dn6, locals.var_tcen2_dn7, locals.var_tcen2_dn8, locals.var_tcen2_dn9, locals.var_tcen2_dn10, locals.var_tcen2_dn11, locals.var_tcen2_dn12,)
    }
};
        locals.var_tcen2 = assign28580_e26588;
        locals.var_tcen2_dn3 = assign28580_e26588_d_n3;
        locals.var_tcen2_dn4 = assign28580_e26588_d_n4;
        locals.var_tcen2_dn5 = assign28580_e26588_d_n5;
        locals.var_tcen2_dn6 = assign28580_e26588_d_n6;
        locals.var_tcen2_dn7 = assign28580_e26588_d_n7;
        locals.var_tcen2_dn8 = assign28580_e26588_d_n8;
        locals.var_tcen2_dn9 = assign28580_e26588_d_n9;
        locals.var_tcen2_dn10 = assign28580_e26588_d_n10;
        locals.var_tcen2_dn11 = assign28580_e26588_d_n11;
        locals.var_tcen2_dn12 = assign28580_e26588_d_n12;
        locals.var_tcen2_rv = 0.0;

        let (assign28590_e26599, assign28590_e26599_d_n3, assign28590_e26599_d_n4, assign28590_e26599_d_n5, assign28590_e26599_d_n6, assign28590_e26599_d_n7, assign28590_e26599_d_n8, assign28590_e26599_d_n9, assign28590_e26599_d_n10, assign28590_e26599_d_n11, assign28590_e26599_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1410 != 0.0)) {
        let assign28590_e26597: f64 = (locals.var_epssub / locals.var_tcen2);
        (assign28590_e26597, (-((locals.var_epssub * locals.var_tcen2_dn3) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn4) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn5) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn6) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn7) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn8) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn9) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn10) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn11) / (locals.var_tcen2 * locals.var_tcen2))), (-((locals.var_epssub * locals.var_tcen2_dn12) / (locals.var_tcen2 * locals.var_tcen2))),)
    } else {
        (locals.var_ccen2, locals.var_ccen2_dn3, locals.var_ccen2_dn4, locals.var_ccen2_dn5, locals.var_ccen2_dn6, locals.var_ccen2_dn7, locals.var_ccen2_dn8, locals.var_ccen2_dn9, locals.var_ccen2_dn10, locals.var_ccen2_dn11, locals.var_ccen2_dn12,)
    }
};
        locals.var_ccen2 = assign28590_e26599;
        locals.var_ccen2_dn3 = assign28590_e26599_d_n3;
        locals.var_ccen2_dn4 = assign28590_e26599_d_n4;
        locals.var_ccen2_dn5 = assign28590_e26599_d_n5;
        locals.var_ccen2_dn6 = assign28590_e26599_d_n6;
        locals.var_ccen2_dn7 = assign28590_e26599_d_n7;
        locals.var_ccen2_dn8 = assign28590_e26599_d_n8;
        locals.var_ccen2_dn9 = assign28590_e26599_d_n9;
        locals.var_ccen2_dn10 = assign28590_e26599_d_n10;
        locals.var_ccen2_dn11 = assign28590_e26599_d_n11;
        locals.var_ccen2_dn12 = assign28590_e26599_d_n12;
        locals.var_ccen2_rv = 0.0;

        let (assign28600_e26612, assign28600_e26612_d_n3, assign28600_e26612_d_n4, assign28600_e26612_d_n5, assign28600_e26612_d_n6, assign28600_e26612_d_n7, assign28600_e26612_d_n8, assign28600_e26612_d_n9, assign28600_e26612_d_n10, assign28600_e26612_d_n11, assign28600_e26612_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1410 != 0.0)) {
        let assign28600_e26609: f64 = (locals.var_cox + locals.var_ccen2);
        let assign28600_e26610: f64 = (locals.var_cox / assign28600_e26609);
        (assign28600_e26610, (((locals.var_cox_dn3 * assign28600_e26609) - (locals.var_cox * (locals.var_cox_dn3 + locals.var_ccen2_dn3))) / (assign28600_e26609 * assign28600_e26609)), (((locals.var_cox_dn4 * assign28600_e26609) - (locals.var_cox * (locals.var_cox_dn4 + locals.var_ccen2_dn4))) / (assign28600_e26609 * assign28600_e26609)), (((locals.var_cox_dn5 * assign28600_e26609) - (locals.var_cox * (locals.var_cox_dn5 + locals.var_ccen2_dn5))) / (assign28600_e26609 * assign28600_e26609)), (((locals.var_cox_dn6 * assign28600_e26609) - (locals.var_cox * (locals.var_cox_dn6 + locals.var_ccen2_dn6))) / (assign28600_e26609 * assign28600_e26609)), (((locals.var_cox_dn7 * assign28600_e26609) - (locals.var_cox * (locals.var_cox_dn7 + locals.var_ccen2_dn7))) / (assign28600_e26609 * assign28600_e26609)), (((locals.var_cox_dn8 * assign28600_e26609) - (locals.var_cox * (locals.var_cox_dn8 + locals.var_ccen2_dn8))) / (assign28600_e26609 * assign28600_e26609)), (((locals.var_cox_dn9 * assign28600_e26609) - (locals.var_cox * (locals.var_cox_dn9 + locals.var_ccen2_dn9))) / (assign28600_e26609 * assign28600_e26609)), (((locals.var_cox_dn10 * assign28600_e26609) - (locals.var_cox * (locals.var_cox_dn10 + locals.var_ccen2_dn10))) / (assign28600_e26609 * assign28600_e26609)), (((locals.var_cox_dn11 * assign28600_e26609) - (locals.var_cox * (locals.var_cox_dn11 + locals.var_ccen2_dn11))) / (assign28600_e26609 * assign28600_e26609)), (((locals.var_cox_dn12 * assign28600_e26609) - (locals.var_cox * (locals.var_cox_dn12 + locals.var_ccen2_dn12))) / (assign28600_e26609 * assign28600_e26609)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign28600_e26612;
        locals.var_t0__blk808_dn3 = assign28600_e26612_d_n3;
        locals.var_t0__blk808_dn4 = assign28600_e26612_d_n4;
        locals.var_t0__blk808_dn5 = assign28600_e26612_d_n5;
        locals.var_t0__blk808_dn6 = assign28600_e26612_d_n6;
        locals.var_t0__blk808_dn7 = assign28600_e26612_d_n7;
        locals.var_t0__blk808_dn8 = assign28600_e26612_d_n8;
        locals.var_t0__blk808_dn9 = assign28600_e26612_d_n9;
        locals.var_t0__blk808_dn10 = assign28600_e26612_d_n10;
        locals.var_t0__blk808_dn11 = assign28600_e26612_d_n11;
        locals.var_t0__blk808_dn12 = assign28600_e26612_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign28610_e26623, assign28610_e26623_d_n3, assign28610_e26623_d_n4, assign28610_e26623_d_n5, assign28610_e26623_d_n6, assign28610_e26623_d_n7, assign28610_e26623_d_n8, assign28610_e26623_d_n9, assign28610_e26623_d_n10, assign28610_e26623_d_n11, assign28610_e26623_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1410 != 0.0)) {
        let assign28610_e26621: f64 = (locals.var_t0__blk808 * locals.var_ccen2);
        (assign28610_e26621, ((locals.var_t0__blk808_dn3 * locals.var_ccen2) + (locals.var_t0__blk808 * locals.var_ccen2_dn3)), ((locals.var_t0__blk808_dn4 * locals.var_ccen2) + (locals.var_t0__blk808 * locals.var_ccen2_dn4)), ((locals.var_t0__blk808_dn5 * locals.var_ccen2) + (locals.var_t0__blk808 * locals.var_ccen2_dn5)), ((locals.var_t0__blk808_dn6 * locals.var_ccen2) + (locals.var_t0__blk808 * locals.var_ccen2_dn6)), ((locals.var_t0__blk808_dn7 * locals.var_ccen2) + (locals.var_t0__blk808 * locals.var_ccen2_dn7)), ((locals.var_t0__blk808_dn8 * locals.var_ccen2) + (locals.var_t0__blk808 * locals.var_ccen2_dn8)), ((locals.var_t0__blk808_dn9 * locals.var_ccen2) + (locals.var_t0__blk808 * locals.var_ccen2_dn9)), ((locals.var_t0__blk808_dn10 * locals.var_ccen2) + (locals.var_t0__blk808 * locals.var_ccen2_dn10)), ((locals.var_t0__blk808_dn11 * locals.var_ccen2) + (locals.var_t0__blk808 * locals.var_ccen2_dn11)), ((locals.var_t0__blk808_dn12 * locals.var_ccen2) + (locals.var_t0__blk808 * locals.var_ccen2_dn12)),)
    } else {
        (locals.var_coxeff2, locals.var_coxeff2_dn3, locals.var_coxeff2_dn4, locals.var_coxeff2_dn5, locals.var_coxeff2_dn6, locals.var_coxeff2_dn7, locals.var_coxeff2_dn8, locals.var_coxeff2_dn9, locals.var_coxeff2_dn10, locals.var_coxeff2_dn11, locals.var_coxeff2_dn12,)
    }
};
        locals.var_coxeff2 = assign28610_e26623;
        locals.var_coxeff2_dn3 = assign28610_e26623_d_n3;
        locals.var_coxeff2_dn4 = assign28610_e26623_d_n4;
        locals.var_coxeff2_dn5 = assign28610_e26623_d_n5;
        locals.var_coxeff2_dn6 = assign28610_e26623_d_n6;
        locals.var_coxeff2_dn7 = assign28610_e26623_d_n7;
        locals.var_coxeff2_dn8 = assign28610_e26623_d_n8;
        locals.var_coxeff2_dn9 = assign28610_e26623_d_n9;
        locals.var_coxeff2_dn10 = assign28610_e26623_d_n10;
        locals.var_coxeff2_dn11 = assign28610_e26623_d_n11;
        locals.var_coxeff2_dn12 = assign28610_e26623_d_n12;
        locals.var_coxeff2_rv = 0.0;

        let (assign28620_e26636, assign28620_e26636_d_n3, assign28620_e26636_d_n4, assign28620_e26636_d_n5, assign28620_e26636_d_n6, assign28620_e26636_d_n7, assign28620_e26636_d_n8, assign28620_e26636_d_n9, assign28620_e26636_d_n10, assign28620_e26636_d_n11, assign28620_e26636_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1410 != 0.0)) {
        let assign28620_e26632: f64 = (locals.var_coxwl2 * locals.var_coxeff2);
        let assign28620_e26634: f64 = (assign28620_e26632 / locals.var_cox);
        (assign28620_e26634, (((((locals.var_coxwl2_dn3 * locals.var_coxeff2) + (locals.var_coxwl2 * locals.var_coxeff2_dn3)) * locals.var_cox) - (assign28620_e26632 * locals.var_cox_dn3)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl2_dn4 * locals.var_coxeff2) + (locals.var_coxwl2 * locals.var_coxeff2_dn4)) * locals.var_cox) - (assign28620_e26632 * locals.var_cox_dn4)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl2_dn5 * locals.var_coxeff2) + (locals.var_coxwl2 * locals.var_coxeff2_dn5)) * locals.var_cox) - (assign28620_e26632 * locals.var_cox_dn5)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl2_dn6 * locals.var_coxeff2) + (locals.var_coxwl2 * locals.var_coxeff2_dn6)) * locals.var_cox) - (assign28620_e26632 * locals.var_cox_dn6)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl2_dn7 * locals.var_coxeff2) + (locals.var_coxwl2 * locals.var_coxeff2_dn7)) * locals.var_cox) - (assign28620_e26632 * locals.var_cox_dn7)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl2_dn8 * locals.var_coxeff2) + (locals.var_coxwl2 * locals.var_coxeff2_dn8)) * locals.var_cox) - (assign28620_e26632 * locals.var_cox_dn8)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl2_dn9 * locals.var_coxeff2) + (locals.var_coxwl2 * locals.var_coxeff2_dn9)) * locals.var_cox) - (assign28620_e26632 * locals.var_cox_dn9)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl2_dn10 * locals.var_coxeff2) + (locals.var_coxwl2 * locals.var_coxeff2_dn10)) * locals.var_cox) - (assign28620_e26632 * locals.var_cox_dn10)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl2_dn11 * locals.var_coxeff2) + (locals.var_coxwl2 * locals.var_coxeff2_dn11)) * locals.var_cox) - (assign28620_e26632 * locals.var_cox_dn11)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwl2_dn12 * locals.var_coxeff2) + (locals.var_coxwl2 * locals.var_coxeff2_dn12)) * locals.var_cox) - (assign28620_e26632 * locals.var_cox_dn12)) / (locals.var_cox * locals.var_cox)),)
    } else {
        (locals.var_coxwlcen2, locals.var_coxwlcen2_dn3, locals.var_coxwlcen2_dn4, locals.var_coxwlcen2_dn5, locals.var_coxwlcen2_dn6, locals.var_coxwlcen2_dn7, locals.var_coxwlcen2_dn8, locals.var_coxwlcen2_dn9, locals.var_coxwlcen2_dn10, locals.var_coxwlcen2_dn11, locals.var_coxwlcen2_dn12,)
    }
};
        locals.var_coxwlcen2 = assign28620_e26636;
        locals.var_coxwlcen2_dn3 = assign28620_e26636_d_n3;
        locals.var_coxwlcen2_dn4 = assign28620_e26636_d_n4;
        locals.var_coxwlcen2_dn5 = assign28620_e26636_d_n5;
        locals.var_coxwlcen2_dn6 = assign28620_e26636_d_n6;
        locals.var_coxwlcen2_dn7 = assign28620_e26636_d_n7;
        locals.var_coxwlcen2_dn8 = assign28620_e26636_d_n8;
        locals.var_coxwlcen2_dn9 = assign28620_e26636_d_n9;
        locals.var_coxwlcen2_dn10 = assign28620_e26636_d_n10;
        locals.var_coxwlcen2_dn11 = assign28620_e26636_d_n11;
        locals.var_coxwlcen2_dn12 = assign28620_e26636_d_n12;
        locals.var_coxwlcen2_rv = 0.0;

        let (assign28630_e26649, assign28630_e26649_d_n3, assign28630_e26649_d_n4, assign28630_e26649_d_n5, assign28630_e26649_d_n6, assign28630_e26649_d_n7, assign28630_e26649_d_n8, assign28630_e26649_d_n9, assign28630_e26649_d_n10, assign28630_e26649_d_n11, assign28630_e26649_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1410 != 0.0)) {
        let assign28630_e26645: f64 = (locals.var_coxwlb2 * locals.var_coxeff2);
        let assign28630_e26647: f64 = (assign28630_e26645 / locals.var_cox);
        (assign28630_e26647, (((((locals.var_coxwlb2_dn3 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn3)) * locals.var_cox) - (assign28630_e26645 * locals.var_cox_dn3)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn4 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn4)) * locals.var_cox) - (assign28630_e26645 * locals.var_cox_dn4)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn5 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn5)) * locals.var_cox) - (assign28630_e26645 * locals.var_cox_dn5)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn6 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn6)) * locals.var_cox) - (assign28630_e26645 * locals.var_cox_dn6)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn7 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn7)) * locals.var_cox) - (assign28630_e26645 * locals.var_cox_dn7)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn8 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn8)) * locals.var_cox) - (assign28630_e26645 * locals.var_cox_dn8)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn9 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn9)) * locals.var_cox) - (assign28630_e26645 * locals.var_cox_dn9)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn10 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn10)) * locals.var_cox) - (assign28630_e26645 * locals.var_cox_dn10)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn11 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn11)) * locals.var_cox) - (assign28630_e26645 * locals.var_cox_dn11)) / (locals.var_cox * locals.var_cox)), (((((locals.var_coxwlb2_dn12 * locals.var_coxeff2) + (locals.var_coxwlb2 * locals.var_coxeff2_dn12)) * locals.var_cox) - (assign28630_e26645 * locals.var_cox_dn12)) / (locals.var_cox * locals.var_cox)),)
    } else {
        (locals.var_coxwlcenb2, locals.var_coxwlcenb2_dn3, locals.var_coxwlcenb2_dn4, locals.var_coxwlcenb2_dn5, locals.var_coxwlcenb2_dn6, locals.var_coxwlcenb2_dn7, locals.var_coxwlcenb2_dn8, locals.var_coxwlcenb2_dn9, locals.var_coxwlcenb2_dn10, locals.var_coxwlcenb2_dn11, locals.var_coxwlcenb2_dn12,)
    }
};
        locals.var_coxwlcenb2 = assign28630_e26649;
        locals.var_coxwlcenb2_dn3 = assign28630_e26649_d_n3;
        locals.var_coxwlcenb2_dn4 = assign28630_e26649_d_n4;
        locals.var_coxwlcenb2_dn5 = assign28630_e26649_d_n5;
        locals.var_coxwlcenb2_dn6 = assign28630_e26649_d_n6;
        locals.var_coxwlcenb2_dn7 = assign28630_e26649_d_n7;
        locals.var_coxwlcenb2_dn8 = assign28630_e26649_d_n8;
        locals.var_coxwlcenb2_dn9 = assign28630_e26649_d_n9;
        locals.var_coxwlcenb2_dn10 = assign28630_e26649_d_n10;
        locals.var_coxwlcenb2_dn11 = assign28630_e26649_d_n11;
        locals.var_coxwlcenb2_dn12 = assign28630_e26649_d_n12;
        locals.var_coxwlcenb2_rv = 0.0;

        let (assign28640_e26658, assign28640_e26658_d_n3, assign28640_e26658_d_n4, assign28640_e26658_d_n5, assign28640_e26658_d_n6, assign28640_e26658_d_n7, assign28640_e26658_d_n8, assign28640_e26658_d_n9, assign28640_e26658_d_n10, assign28640_e26658_d_n11, assign28640_e26658_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) {
        let assign28640_e26656: f64 = (locals.var_vgsteff__blk840 - locals.var_deltaphi);
        (assign28640_e26656, (locals.var_vgsteff__blk840_dn3 - locals.var_deltaphi_dn3), (locals.var_vgsteff__blk840_dn4 - locals.var_deltaphi_dn4), (locals.var_vgsteff__blk840_dn5 - locals.var_deltaphi_dn5), (locals.var_vgsteff__blk840_dn6 - locals.var_deltaphi_dn6), (locals.var_vgsteff__blk840_dn7 - locals.var_deltaphi_dn7), (locals.var_vgsteff__blk840_dn8 - locals.var_deltaphi_dn8), (locals.var_vgsteff__blk840_dn9 - locals.var_deltaphi_dn9), (locals.var_vgsteff__blk840_dn10 - locals.var_deltaphi_dn10), (locals.var_vgsteff__blk840_dn11 - locals.var_deltaphi_dn11), (locals.var_vgsteff__blk840_dn12 - locals.var_deltaphi_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign28640_e26658;
        locals.var_t1__blk809_dn3 = assign28640_e26658_d_n3;
        locals.var_t1__blk809_dn4 = assign28640_e26658_d_n4;
        locals.var_t1__blk809_dn5 = assign28640_e26658_d_n5;
        locals.var_t1__blk809_dn6 = assign28640_e26658_d_n6;
        locals.var_t1__blk809_dn7 = assign28640_e26658_d_n7;
        locals.var_t1__blk809_dn8 = assign28640_e26658_d_n8;
        locals.var_t1__blk809_dn9 = assign28640_e26658_d_n9;
        locals.var_t1__blk809_dn10 = assign28640_e26658_d_n10;
        locals.var_t1__blk809_dn11 = assign28640_e26658_d_n11;
        locals.var_t1__blk809_dn12 = assign28640_e26658_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign28650_e26667, assign28650_e26667_d_n3, assign28650_e26667_d_n4, assign28650_e26667_d_n5, assign28650_e26667_d_n6, assign28650_e26667_d_n7, assign28650_e26667_d_n8, assign28650_e26667_d_n9, assign28650_e26667_d_n10, assign28650_e26667_d_n11, assign28650_e26667_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) {
        let assign28650_e26665: f64 = (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor);
        (assign28650_e26665, ((locals.var_abulk0_dn3 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn3)), ((locals.var_abulk0_dn4 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn4)), ((locals.var_abulk0_dn5 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn5)), ((locals.var_abulk0_dn6 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn6)), ((locals.var_abulk0_dn7 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn7)), ((locals.var_abulk0_dn8 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn8)), ((locals.var_abulk0_dn9 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn9)), ((locals.var_abulk0_dn10 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn10)), ((locals.var_abulk0_dn11 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn11)), ((locals.var_abulk0_dn12 * locals.var_pparam_b4soiabulkcvfactor) + (locals.var_abulk0 * locals.var_pparam_b4soiabulkcvfactor_dn12)),)
    } else {
        (locals.var_abulkcv, locals.var_abulkcv_dn3, locals.var_abulkcv_dn4, locals.var_abulkcv_dn5, locals.var_abulkcv_dn6, locals.var_abulkcv_dn7, locals.var_abulkcv_dn8, locals.var_abulkcv_dn9, locals.var_abulkcv_dn10, locals.var_abulkcv_dn11, locals.var_abulkcv_dn12,)
    }
};
        locals.var_abulkcv = assign28650_e26667;
        locals.var_abulkcv_dn3 = assign28650_e26667_d_n3;
        locals.var_abulkcv_dn4 = assign28650_e26667_d_n4;
        locals.var_abulkcv_dn5 = assign28650_e26667_d_n5;
        locals.var_abulkcv_dn6 = assign28650_e26667_d_n6;
        locals.var_abulkcv_dn7 = assign28650_e26667_d_n7;
        locals.var_abulkcv_dn8 = assign28650_e26667_d_n8;
        locals.var_abulkcv_dn9 = assign28650_e26667_d_n9;
        locals.var_abulkcv_dn10 = assign28650_e26667_d_n10;
        locals.var_abulkcv_dn11 = assign28650_e26667_d_n11;
        locals.var_abulkcv_dn12 = assign28650_e26667_d_n12;
        locals.var_abulkcv_rv = 0.0;

        let (assign28660_e26676, assign28660_e26676_d_n3, assign28660_e26676_d_n4, assign28660_e26676_d_n5, assign28660_e26676_d_n6, assign28660_e26676_d_n7, assign28660_e26676_d_n8, assign28660_e26676_d_n9, assign28660_e26676_d_n10, assign28660_e26676_d_n11, assign28660_e26676_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) {
        let assign28660_e26674: f64 = (locals.var_t1__blk809 / locals.var_abulkcv);
        (assign28660_e26674, (((locals.var_t1__blk809_dn3 * locals.var_abulkcv) - (locals.var_t1__blk809 * locals.var_abulkcv_dn3)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t1__blk809_dn4 * locals.var_abulkcv) - (locals.var_t1__blk809 * locals.var_abulkcv_dn4)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t1__blk809_dn5 * locals.var_abulkcv) - (locals.var_t1__blk809 * locals.var_abulkcv_dn5)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t1__blk809_dn6 * locals.var_abulkcv) - (locals.var_t1__blk809 * locals.var_abulkcv_dn6)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t1__blk809_dn7 * locals.var_abulkcv) - (locals.var_t1__blk809 * locals.var_abulkcv_dn7)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t1__blk809_dn8 * locals.var_abulkcv) - (locals.var_t1__blk809 * locals.var_abulkcv_dn8)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t1__blk809_dn9 * locals.var_abulkcv) - (locals.var_t1__blk809 * locals.var_abulkcv_dn9)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t1__blk809_dn10 * locals.var_abulkcv) - (locals.var_t1__blk809 * locals.var_abulkcv_dn10)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t1__blk809_dn11 * locals.var_abulkcv) - (locals.var_t1__blk809 * locals.var_abulkcv_dn11)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t1__blk809_dn12 * locals.var_abulkcv) - (locals.var_t1__blk809 * locals.var_abulkcv_dn12)) / (locals.var_abulkcv * locals.var_abulkcv)),)
    } else {
        (locals.var_vdsatcv, locals.var_vdsatcv_dn3, locals.var_vdsatcv_dn4, locals.var_vdsatcv_dn5, locals.var_vdsatcv_dn6, locals.var_vdsatcv_dn7, locals.var_vdsatcv_dn8, locals.var_vdsatcv_dn9, locals.var_vdsatcv_dn10, locals.var_vdsatcv_dn11, locals.var_vdsatcv_dn12,)
    }
};
        locals.var_vdsatcv = assign28660_e26676;
        locals.var_vdsatcv_dn3 = assign28660_e26676_d_n3;
        locals.var_vdsatcv_dn4 = assign28660_e26676_d_n4;
        locals.var_vdsatcv_dn5 = assign28660_e26676_d_n5;
        locals.var_vdsatcv_dn6 = assign28660_e26676_d_n6;
        locals.var_vdsatcv_dn7 = assign28660_e26676_d_n7;
        locals.var_vdsatcv_dn8 = assign28660_e26676_d_n8;
        locals.var_vdsatcv_dn9 = assign28660_e26676_d_n9;
        locals.var_vdsatcv_dn10 = assign28660_e26676_d_n10;
        locals.var_vdsatcv_dn11 = assign28660_e26676_d_n11;
        locals.var_vdsatcv_dn12 = assign28660_e26676_d_n12;
        locals.var_vdsatcv_rv = 0.0;

        let (assign28670_e26687, assign28670_e26687_d_n3, assign28670_e26687_d_n4, assign28670_e26687_d_n5, assign28670_e26687_d_n6, assign28670_e26687_d_n7, assign28670_e26687_d_n8, assign28670_e26687_d_n9, assign28670_e26687_d_n10, assign28670_e26687_d_n11, assign28670_e26687_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) {
        let assign28670_e26683: f64 = (locals.var_vdsatcv - locals.var_vds_1);
        let assign28670_e26685: f64 = (assign28670_e26683 - 0.02);
        (assign28670_e26685, locals.var_vdsatcv_dn3, locals.var_vdsatcv_dn4, locals.var_vdsatcv_dn5, locals.var_vdsatcv_dn6, (locals.var_vdsatcv_dn7 - locals.var_vds_1_dn7), (locals.var_vdsatcv_dn8 - locals.var_vds_1_dn8), locals.var_vdsatcv_dn9, locals.var_vdsatcv_dn10, locals.var_vdsatcv_dn11, locals.var_vdsatcv_dn12,)
    } else {
        (locals.var_v4, locals.var_v4_dn3, locals.var_v4_dn4, locals.var_v4_dn5, locals.var_v4_dn6, locals.var_v4_dn7, locals.var_v4_dn8, locals.var_v4_dn9, locals.var_v4_dn10, locals.var_v4_dn11, locals.var_v4_dn12,)
    }
};
        locals.var_v4 = assign28670_e26687;
        locals.var_v4_dn3 = assign28670_e26687_d_n3;
        locals.var_v4_dn4 = assign28670_e26687_d_n4;
        locals.var_v4_dn5 = assign28670_e26687_d_n5;
        locals.var_v4_dn6 = assign28670_e26687_d_n6;
        locals.var_v4_dn7 = assign28670_e26687_d_n7;
        locals.var_v4_dn8 = assign28670_e26687_d_n8;
        locals.var_v4_dn9 = assign28670_e26687_d_n9;
        locals.var_v4_dn10 = assign28670_e26687_d_n10;
        locals.var_v4_dn11 = assign28670_e26687_d_n11;
        locals.var_v4_dn12 = assign28670_e26687_d_n12;
        locals.var_v4_rv = 0.0;

        let (assign28680_e26703, assign28680_e26703_d_n3, assign28680_e26703_d_n4, assign28680_e26703_d_n5, assign28680_e26703_d_n6, assign28680_e26703_d_n7, assign28680_e26703_d_n8, assign28680_e26703_d_n9, assign28680_e26703_d_n10, assign28680_e26703_d_n11, assign28680_e26703_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) {
        let assign28680_e26694: f64 = (locals.var_v4 * locals.var_v4);
        let assign28680_e26697: f64 = (4.0 * 0.02);
        let assign28680_e26699: f64 = (assign28680_e26697 * locals.var_vdsatcv);
        let assign28680_e26700: f64 = (assign28680_e26694 + assign28680_e26699);
        let assign28680_e26701: f64 = (assign28680_e26700).sqrt();
        (assign28680_e26701, ((((locals.var_v4_dn3 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn3)) + (assign28680_e26697 * locals.var_vdsatcv_dn3)) / (2.0 * assign28680_e26701)), ((((locals.var_v4_dn4 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn4)) + (assign28680_e26697 * locals.var_vdsatcv_dn4)) / (2.0 * assign28680_e26701)), ((((locals.var_v4_dn5 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn5)) + (assign28680_e26697 * locals.var_vdsatcv_dn5)) / (2.0 * assign28680_e26701)), ((((locals.var_v4_dn6 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn6)) + (assign28680_e26697 * locals.var_vdsatcv_dn6)) / (2.0 * assign28680_e26701)), ((((locals.var_v4_dn7 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn7)) + (assign28680_e26697 * locals.var_vdsatcv_dn7)) / (2.0 * assign28680_e26701)), ((((locals.var_v4_dn8 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn8)) + (assign28680_e26697 * locals.var_vdsatcv_dn8)) / (2.0 * assign28680_e26701)), ((((locals.var_v4_dn9 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn9)) + (assign28680_e26697 * locals.var_vdsatcv_dn9)) / (2.0 * assign28680_e26701)), ((((locals.var_v4_dn10 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn10)) + (assign28680_e26697 * locals.var_vdsatcv_dn10)) / (2.0 * assign28680_e26701)), ((((locals.var_v4_dn11 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn11)) + (assign28680_e26697 * locals.var_vdsatcv_dn11)) / (2.0 * assign28680_e26701)), ((((locals.var_v4_dn12 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn12)) + (assign28680_e26697 * locals.var_vdsatcv_dn12)) / (2.0 * assign28680_e26701)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign28680_e26703;
        locals.var_t0__blk808_dn3 = assign28680_e26703_d_n3;
        locals.var_t0__blk808_dn4 = assign28680_e26703_d_n4;
        locals.var_t0__blk808_dn5 = assign28680_e26703_d_n5;
        locals.var_t0__blk808_dn6 = assign28680_e26703_d_n6;
        locals.var_t0__blk808_dn7 = assign28680_e26703_d_n7;
        locals.var_t0__blk808_dn8 = assign28680_e26703_d_n8;
        locals.var_t0__blk808_dn9 = assign28680_e26703_d_n9;
        locals.var_t0__blk808_dn10 = assign28680_e26703_d_n10;
        locals.var_t0__blk808_dn11 = assign28680_e26703_d_n11;
        locals.var_t0__blk808_dn12 = assign28680_e26703_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign28690_e26716, assign28690_e26716_d_n3, assign28690_e26716_d_n4, assign28690_e26716_d_n5, assign28690_e26716_d_n6, assign28690_e26716_d_n7, assign28690_e26716_d_n8, assign28690_e26716_d_n9, assign28690_e26716_d_n10, assign28690_e26716_d_n11, assign28690_e26716_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) {
        let assign28690_e26712: f64 = (locals.var_v4 + locals.var_t0__blk808);
        let assign28690_e26713: f64 = (0.5 * assign28690_e26712);
        let assign28690_e26714: f64 = (locals.var_vdsatcv - assign28690_e26713);
        (assign28690_e26714, (locals.var_vdsatcv_dn3 - (0.5 * (locals.var_v4_dn3 + locals.var_t0__blk808_dn3))), (locals.var_vdsatcv_dn4 - (0.5 * (locals.var_v4_dn4 + locals.var_t0__blk808_dn4))), (locals.var_vdsatcv_dn5 - (0.5 * (locals.var_v4_dn5 + locals.var_t0__blk808_dn5))), (locals.var_vdsatcv_dn6 - (0.5 * (locals.var_v4_dn6 + locals.var_t0__blk808_dn6))), (locals.var_vdsatcv_dn7 - (0.5 * (locals.var_v4_dn7 + locals.var_t0__blk808_dn7))), (locals.var_vdsatcv_dn8 - (0.5 * (locals.var_v4_dn8 + locals.var_t0__blk808_dn8))), (locals.var_vdsatcv_dn9 - (0.5 * (locals.var_v4_dn9 + locals.var_t0__blk808_dn9))), (locals.var_vdsatcv_dn10 - (0.5 * (locals.var_v4_dn10 + locals.var_t0__blk808_dn10))), (locals.var_vdsatcv_dn11 - (0.5 * (locals.var_v4_dn11 + locals.var_t0__blk808_dn11))), (locals.var_vdsatcv_dn12 - (0.5 * (locals.var_v4_dn12 + locals.var_t0__blk808_dn12))),)
    } else {
        (locals.var_vdseffcv, locals.var_vdseffcv_dn3, locals.var_vdseffcv_dn4, locals.var_vdseffcv_dn5, locals.var_vdseffcv_dn6, locals.var_vdseffcv_dn7, locals.var_vdseffcv_dn8, locals.var_vdseffcv_dn9, locals.var_vdseffcv_dn10, locals.var_vdseffcv_dn11, locals.var_vdseffcv_dn12,)
    }
};
        locals.var_vdseffcv = assign28690_e26716;
        locals.var_vdseffcv_dn3 = assign28690_e26716_d_n3;
        locals.var_vdseffcv_dn4 = assign28690_e26716_d_n4;
        locals.var_vdseffcv_dn5 = assign28690_e26716_d_n5;
        locals.var_vdseffcv_dn6 = assign28690_e26716_d_n6;
        locals.var_vdseffcv_dn7 = assign28690_e26716_d_n7;
        locals.var_vdseffcv_dn8 = assign28690_e26716_d_n8;
        locals.var_vdseffcv_dn9 = assign28690_e26716_d_n9;
        locals.var_vdseffcv_dn10 = assign28690_e26716_d_n10;
        locals.var_vdseffcv_dn11 = assign28690_e26716_d_n11;
        locals.var_vdseffcv_dn12 = assign28690_e26716_d_n12;
        locals.var_vdseffcv_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_88(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign28700_e26725, assign28700_e26725_d_n3, assign28700_e26725_d_n4, assign28700_e26725_d_n5, assign28700_e26725_d_n6, assign28700_e26725_d_n7, assign28700_e26725_d_n8, assign28700_e26725_d_n9, assign28700_e26725_d_n10, assign28700_e26725_d_n11, assign28700_e26725_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) {
        let assign28700_e26723: f64 = (locals.var_abulkcv * locals.var_vdseffcv);
        (assign28700_e26723, ((locals.var_abulkcv_dn3 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn3)), ((locals.var_abulkcv_dn4 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn4)), ((locals.var_abulkcv_dn5 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn5)), ((locals.var_abulkcv_dn6 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn6)), ((locals.var_abulkcv_dn7 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn7)), ((locals.var_abulkcv_dn8 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn8)), ((locals.var_abulkcv_dn9 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn9)), ((locals.var_abulkcv_dn10 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn10)), ((locals.var_abulkcv_dn11 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn11)), ((locals.var_abulkcv_dn12 * locals.var_vdseffcv) + (locals.var_abulkcv * locals.var_vdseffcv_dn12)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign28700_e26725;
        locals.var_t0__blk808_dn3 = assign28700_e26725_d_n3;
        locals.var_t0__blk808_dn4 = assign28700_e26725_d_n4;
        locals.var_t0__blk808_dn5 = assign28700_e26725_d_n5;
        locals.var_t0__blk808_dn6 = assign28700_e26725_d_n6;
        locals.var_t0__blk808_dn7 = assign28700_e26725_d_n7;
        locals.var_t0__blk808_dn8 = assign28700_e26725_d_n8;
        locals.var_t0__blk808_dn9 = assign28700_e26725_d_n9;
        locals.var_t0__blk808_dn10 = assign28700_e26725_d_n10;
        locals.var_t0__blk808_dn11 = assign28700_e26725_d_n11;
        locals.var_t0__blk808_dn12 = assign28700_e26725_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign28710_e26740, assign28710_e26740_d_n3, assign28710_e26740_d_n4, assign28710_e26740_d_n5, assign28710_e26740_d_n6, assign28710_e26740_d_n7, assign28710_e26740_d_n8, assign28710_e26740_d_n9, assign28710_e26740_d_n10, assign28710_e26740_d_n11, assign28710_e26740_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) {
        let assign28710_e26734: f64 = (0.5 * locals.var_t0__blk808);
        let assign28710_e26735: f64 = (locals.var_t1__blk809 - assign28710_e26734);
        let assign28710_e26737: f64 = (assign28710_e26735 + 1e-20);
        let assign28710_e26738: f64 = (12.0 * assign28710_e26737);
        (assign28710_e26738, (12.0 * (locals.var_t1__blk809_dn3 - (0.5 * locals.var_t0__blk808_dn3))), (12.0 * (locals.var_t1__blk809_dn4 - (0.5 * locals.var_t0__blk808_dn4))), (12.0 * (locals.var_t1__blk809_dn5 - (0.5 * locals.var_t0__blk808_dn5))), (12.0 * (locals.var_t1__blk809_dn6 - (0.5 * locals.var_t0__blk808_dn6))), (12.0 * (locals.var_t1__blk809_dn7 - (0.5 * locals.var_t0__blk808_dn7))), (12.0 * (locals.var_t1__blk809_dn8 - (0.5 * locals.var_t0__blk808_dn8))), (12.0 * (locals.var_t1__blk809_dn9 - (0.5 * locals.var_t0__blk808_dn9))), (12.0 * (locals.var_t1__blk809_dn10 - (0.5 * locals.var_t0__blk808_dn10))), (12.0 * (locals.var_t1__blk809_dn11 - (0.5 * locals.var_t0__blk808_dn11))), (12.0 * (locals.var_t1__blk809_dn12 - (0.5 * locals.var_t0__blk808_dn12))),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign28710_e26740;
        locals.var_t2__blk810_dn3 = assign28710_e26740_d_n3;
        locals.var_t2__blk810_dn4 = assign28710_e26740_d_n4;
        locals.var_t2__blk810_dn5 = assign28710_e26740_d_n5;
        locals.var_t2__blk810_dn6 = assign28710_e26740_d_n6;
        locals.var_t2__blk810_dn7 = assign28710_e26740_d_n7;
        locals.var_t2__blk810_dn8 = assign28710_e26740_d_n8;
        locals.var_t2__blk810_dn9 = assign28710_e26740_d_n9;
        locals.var_t2__blk810_dn10 = assign28710_e26740_d_n10;
        locals.var_t2__blk810_dn11 = assign28710_e26740_d_n11;
        locals.var_t2__blk810_dn12 = assign28710_e26740_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign28720_e26749, assign28720_e26749_d_n3, assign28720_e26749_d_n4, assign28720_e26749_d_n5, assign28720_e26749_d_n6, assign28720_e26749_d_n7, assign28720_e26749_d_n8, assign28720_e26749_d_n9, assign28720_e26749_d_n10, assign28720_e26749_d_n11, assign28720_e26749_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) {
        let assign28720_e26747: f64 = (locals.var_t0__blk808 / locals.var_t2__blk810);
        (assign28720_e26747, (((locals.var_t0__blk808_dn3 * locals.var_t2__blk810) - (locals.var_t0__blk808 * locals.var_t2__blk810_dn3)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t0__blk808_dn4 * locals.var_t2__blk810) - (locals.var_t0__blk808 * locals.var_t2__blk810_dn4)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t0__blk808_dn5 * locals.var_t2__blk810) - (locals.var_t0__blk808 * locals.var_t2__blk810_dn5)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t0__blk808_dn6 * locals.var_t2__blk810) - (locals.var_t0__blk808 * locals.var_t2__blk810_dn6)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t0__blk808_dn7 * locals.var_t2__blk810) - (locals.var_t0__blk808 * locals.var_t2__blk810_dn7)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t0__blk808_dn8 * locals.var_t2__blk810) - (locals.var_t0__blk808 * locals.var_t2__blk810_dn8)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t0__blk808_dn9 * locals.var_t2__blk810) - (locals.var_t0__blk808 * locals.var_t2__blk810_dn9)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t0__blk808_dn10 * locals.var_t2__blk810) - (locals.var_t0__blk808 * locals.var_t2__blk810_dn10)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t0__blk808_dn11 * locals.var_t2__blk810) - (locals.var_t0__blk808 * locals.var_t2__blk810_dn11)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t0__blk808_dn12 * locals.var_t2__blk810) - (locals.var_t0__blk808 * locals.var_t2__blk810_dn12)) / (locals.var_t2__blk810 * locals.var_t2__blk810)),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign28720_e26749;
        locals.var_t3__blk811_dn3 = assign28720_e26749_d_n3;
        locals.var_t3__blk811_dn4 = assign28720_e26749_d_n4;
        locals.var_t3__blk811_dn5 = assign28720_e26749_d_n5;
        locals.var_t3__blk811_dn6 = assign28720_e26749_d_n6;
        locals.var_t3__blk811_dn7 = assign28720_e26749_d_n7;
        locals.var_t3__blk811_dn8 = assign28720_e26749_d_n8;
        locals.var_t3__blk811_dn9 = assign28720_e26749_d_n9;
        locals.var_t3__blk811_dn10 = assign28720_e26749_d_n10;
        locals.var_t3__blk811_dn11 = assign28720_e26749_d_n11;
        locals.var_t3__blk811_dn12 = assign28720_e26749_d_n12;
        locals.var_t3__blk811_rv = 0.0;

        let (assign28730_e26764, assign28730_e26764_d_n3, assign28730_e26764_d_n4, assign28730_e26764_d_n5, assign28730_e26764_d_n6, assign28730_e26764_d_n7, assign28730_e26764_d_n8, assign28730_e26764_d_n9, assign28730_e26764_d_n10, assign28730_e26764_d_n11, assign28730_e26764_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) {
        let assign28730_e26759: f64 = (0.5 - locals.var_t3__blk811);
        let assign28730_e26760: f64 = (locals.var_t0__blk808 * assign28730_e26759);
        let assign28730_e26761: f64 = (locals.var_t1__blk809 - assign28730_e26760);
        let assign28730_e26762: f64 = (locals.var_coxwlcen * assign28730_e26761);
        (assign28730_e26762, ((locals.var_coxwlcen_dn3 * assign28730_e26761) + (locals.var_coxwlcen * (locals.var_t1__blk809_dn3 - ((locals.var_t0__blk808_dn3 * assign28730_e26759) + (locals.var_t0__blk808 * (-locals.var_t3__blk811_dn3)))))), ((locals.var_coxwlcen_dn4 * assign28730_e26761) + (locals.var_coxwlcen * (locals.var_t1__blk809_dn4 - ((locals.var_t0__blk808_dn4 * assign28730_e26759) + (locals.var_t0__blk808 * (-locals.var_t3__blk811_dn4)))))), ((locals.var_coxwlcen_dn5 * assign28730_e26761) + (locals.var_coxwlcen * (locals.var_t1__blk809_dn5 - ((locals.var_t0__blk808_dn5 * assign28730_e26759) + (locals.var_t0__blk808 * (-locals.var_t3__blk811_dn5)))))), ((locals.var_coxwlcen_dn6 * assign28730_e26761) + (locals.var_coxwlcen * (locals.var_t1__blk809_dn6 - ((locals.var_t0__blk808_dn6 * assign28730_e26759) + (locals.var_t0__blk808 * (-locals.var_t3__blk811_dn6)))))), ((locals.var_coxwlcen_dn7 * assign28730_e26761) + (locals.var_coxwlcen * (locals.var_t1__blk809_dn7 - ((locals.var_t0__blk808_dn7 * assign28730_e26759) + (locals.var_t0__blk808 * (-locals.var_t3__blk811_dn7)))))), ((locals.var_coxwlcen_dn8 * assign28730_e26761) + (locals.var_coxwlcen * (locals.var_t1__blk809_dn8 - ((locals.var_t0__blk808_dn8 * assign28730_e26759) + (locals.var_t0__blk808 * (-locals.var_t3__blk811_dn8)))))), ((locals.var_coxwlcen_dn9 * assign28730_e26761) + (locals.var_coxwlcen * (locals.var_t1__blk809_dn9 - ((locals.var_t0__blk808_dn9 * assign28730_e26759) + (locals.var_t0__blk808 * (-locals.var_t3__blk811_dn9)))))), ((locals.var_coxwlcen_dn10 * assign28730_e26761) + (locals.var_coxwlcen * (locals.var_t1__blk809_dn10 - ((locals.var_t0__blk808_dn10 * assign28730_e26759) + (locals.var_t0__blk808 * (-locals.var_t3__blk811_dn10)))))), ((locals.var_coxwlcen_dn11 * assign28730_e26761) + (locals.var_coxwlcen * (locals.var_t1__blk809_dn11 - ((locals.var_t0__blk808_dn11 * assign28730_e26759) + (locals.var_t0__blk808 * (-locals.var_t3__blk811_dn11)))))), ((locals.var_coxwlcen_dn12 * assign28730_e26761) + (locals.var_coxwlcen * (locals.var_t1__blk809_dn12 - ((locals.var_t0__blk808_dn12 * assign28730_e26759) + (locals.var_t0__blk808 * (-locals.var_t3__blk811_dn12)))))),)
    } else {
        (locals.var_qinv, locals.var_qinv_dn3, locals.var_qinv_dn4, locals.var_qinv_dn5, locals.var_qinv_dn6, locals.var_qinv_dn7, locals.var_qinv_dn8, locals.var_qinv_dn9, locals.var_qinv_dn10, locals.var_qinv_dn11, locals.var_qinv_dn12,)
    }
};
        locals.var_qinv = assign28730_e26764;
        locals.var_qinv_dn3 = assign28730_e26764_d_n3;
        locals.var_qinv_dn4 = assign28730_e26764_d_n4;
        locals.var_qinv_dn5 = assign28730_e26764_d_n5;
        locals.var_qinv_dn6 = assign28730_e26764_d_n6;
        locals.var_qinv_dn7 = assign28730_e26764_d_n7;
        locals.var_qinv_dn8 = assign28730_e26764_d_n8;
        locals.var_qinv_dn9 = assign28730_e26764_d_n9;
        locals.var_qinv_dn10 = assign28730_e26764_d_n10;
        locals.var_qinv_dn11 = assign28730_e26764_d_n11;
        locals.var_qinv_dn12 = assign28730_e26764_d_n12;
        locals.var_qinv_rv = 0.0;

        let (assign28750_e26778, assign28750_e26778_d_n3, assign28750_e26778_d_n4, assign28750_e26778_d_n5, assign28750_e26778_d_n6, assign28750_e26778_d_n7, assign28750_e26778_d_n8, assign28750_e26778_d_n9, assign28750_e26778_d_n10, assign28750_e26778_d_n11, assign28750_e26778_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) {
        (locals.var_qinv, locals.var_qinv_dn3, locals.var_qinv_dn4, locals.var_qinv_dn5, locals.var_qinv_dn6, locals.var_qinv_dn7, locals.var_qinv_dn8, locals.var_qinv_dn9, locals.var_qinv_dn10, locals.var_qinv_dn11, locals.var_qinv_dn12,)
    } else {
        (locals.var_qgate, locals.var_qgate_dn3, locals.var_qgate_dn4, locals.var_qgate_dn5, locals.var_qgate_dn6, locals.var_qgate_dn7, locals.var_qgate_dn8, locals.var_qgate_dn9, locals.var_qgate_dn10, locals.var_qgate_dn11, locals.var_qgate_dn12,)
    }
};
        locals.var_qgate = assign28750_e26778;
        locals.var_qgate_dn3 = assign28750_e26778_d_n3;
        locals.var_qgate_dn4 = assign28750_e26778_d_n4;
        locals.var_qgate_dn5 = assign28750_e26778_d_n5;
        locals.var_qgate_dn6 = assign28750_e26778_d_n6;
        locals.var_qgate_dn7 = assign28750_e26778_d_n7;
        locals.var_qgate_dn8 = assign28750_e26778_d_n8;
        locals.var_qgate_dn9 = assign28750_e26778_d_n9;
        locals.var_qgate_dn10 = assign28750_e26778_d_n10;
        locals.var_qgate_dn11 = assign28750_e26778_d_n11;
        locals.var_qgate_dn12 = assign28750_e26778_d_n12;
        locals.var_qgate_rv = 0.0;

        let assign28760_e26789: f64 = if (((locals.var_b4soisoimod != 2.0) && (locals.var_b4soibodymod != 0.0)) && (p.p27 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1411 = assign28760_e26789;
        locals.var_guard1411_rv = 0.0;

        let (assign28770_e26800, assign28770_e26800_d_n3, assign28770_e26800_d_n4, assign28770_e26800_d_n5, assign28770_e26800_d_n6, assign28770_e26800_d_n7, assign28770_e26800_d_n8, assign28770_e26800_d_n9, assign28770_e26800_d_n10, assign28770_e26800_d_n11, assign28770_e26800_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1411 != 0.0)) {
        let assign28770_e26798: f64 = (locals.var_vgsteff2 - locals.var_deltaphi2);
        (assign28770_e26798, (locals.var_vgsteff2_dn3 - locals.var_deltaphi2_dn3), (locals.var_vgsteff2_dn4 - locals.var_deltaphi2_dn4), (locals.var_vgsteff2_dn5 - locals.var_deltaphi2_dn5), (locals.var_vgsteff2_dn6 - locals.var_deltaphi2_dn6), (locals.var_vgsteff2_dn7 - locals.var_deltaphi2_dn7), (locals.var_vgsteff2_dn8 - locals.var_deltaphi2_dn8), (locals.var_vgsteff2_dn9 - locals.var_deltaphi2_dn9), (locals.var_vgsteff2_dn10 - locals.var_deltaphi2_dn10), (locals.var_vgsteff2_dn11 - locals.var_deltaphi2_dn11), (locals.var_vgsteff2_dn12 - locals.var_deltaphi2_dn12),)
    } else {
        (locals.var_t12, locals.var_t12_dn3, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn12,)
    }
};
        locals.var_t12 = assign28770_e26800;
        locals.var_t12_dn3 = assign28770_e26800_d_n3;
        locals.var_t12_dn4 = assign28770_e26800_d_n4;
        locals.var_t12_dn5 = assign28770_e26800_d_n5;
        locals.var_t12_dn6 = assign28770_e26800_d_n6;
        locals.var_t12_dn7 = assign28770_e26800_d_n7;
        locals.var_t12_dn8 = assign28770_e26800_d_n8;
        locals.var_t12_dn9 = assign28770_e26800_d_n9;
        locals.var_t12_dn10 = assign28770_e26800_d_n10;
        locals.var_t12_dn11 = assign28770_e26800_d_n11;
        locals.var_t12_dn12 = assign28770_e26800_d_n12;
        locals.var_t12_rv = 0.0;

        let (assign28780_e26811, assign28780_e26811_d_n3, assign28780_e26811_d_n4, assign28780_e26811_d_n5, assign28780_e26811_d_n6, assign28780_e26811_d_n7, assign28780_e26811_d_n8, assign28780_e26811_d_n9, assign28780_e26811_d_n10, assign28780_e26811_d_n11, assign28780_e26811_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1411 != 0.0)) {
        let assign28780_e26809: f64 = (locals.var_t12 / locals.var_abulkcv);
        (assign28780_e26809, (((locals.var_t12_dn3 * locals.var_abulkcv) - (locals.var_t12 * locals.var_abulkcv_dn3)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t12_dn4 * locals.var_abulkcv) - (locals.var_t12 * locals.var_abulkcv_dn4)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t12_dn5 * locals.var_abulkcv) - (locals.var_t12 * locals.var_abulkcv_dn5)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t12_dn6 * locals.var_abulkcv) - (locals.var_t12 * locals.var_abulkcv_dn6)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t12_dn7 * locals.var_abulkcv) - (locals.var_t12 * locals.var_abulkcv_dn7)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t12_dn8 * locals.var_abulkcv) - (locals.var_t12 * locals.var_abulkcv_dn8)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t12_dn9 * locals.var_abulkcv) - (locals.var_t12 * locals.var_abulkcv_dn9)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t12_dn10 * locals.var_abulkcv) - (locals.var_t12 * locals.var_abulkcv_dn10)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t12_dn11 * locals.var_abulkcv) - (locals.var_t12 * locals.var_abulkcv_dn11)) / (locals.var_abulkcv * locals.var_abulkcv)), (((locals.var_t12_dn12 * locals.var_abulkcv) - (locals.var_t12 * locals.var_abulkcv_dn12)) / (locals.var_abulkcv * locals.var_abulkcv)),)
    } else {
        (locals.var_vdsatcv2, locals.var_vdsatcv2_dn3, locals.var_vdsatcv2_dn4, locals.var_vdsatcv2_dn5, locals.var_vdsatcv2_dn6, locals.var_vdsatcv2_dn7, locals.var_vdsatcv2_dn8, locals.var_vdsatcv2_dn9, locals.var_vdsatcv2_dn10, locals.var_vdsatcv2_dn11, locals.var_vdsatcv2_dn12,)
    }
};
        locals.var_vdsatcv2 = assign28780_e26811;
        locals.var_vdsatcv2_dn3 = assign28780_e26811_d_n3;
        locals.var_vdsatcv2_dn4 = assign28780_e26811_d_n4;
        locals.var_vdsatcv2_dn5 = assign28780_e26811_d_n5;
        locals.var_vdsatcv2_dn6 = assign28780_e26811_d_n6;
        locals.var_vdsatcv2_dn7 = assign28780_e26811_d_n7;
        locals.var_vdsatcv2_dn8 = assign28780_e26811_d_n8;
        locals.var_vdsatcv2_dn9 = assign28780_e26811_d_n9;
        locals.var_vdsatcv2_dn10 = assign28780_e26811_d_n10;
        locals.var_vdsatcv2_dn11 = assign28780_e26811_d_n11;
        locals.var_vdsatcv2_dn12 = assign28780_e26811_d_n12;
        locals.var_vdsatcv2_rv = 0.0;

        let (assign28790_e26824, assign28790_e26824_d_n3, assign28790_e26824_d_n4, assign28790_e26824_d_n5, assign28790_e26824_d_n6, assign28790_e26824_d_n7, assign28790_e26824_d_n8, assign28790_e26824_d_n9, assign28790_e26824_d_n10, assign28790_e26824_d_n11, assign28790_e26824_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1411 != 0.0)) {
        let assign28790_e26820: f64 = (locals.var_vdsatcv2 - locals.var_vds_1);
        let assign28790_e26822: f64 = (assign28790_e26820 - 0.02);
        (assign28790_e26822, locals.var_vdsatcv2_dn3, locals.var_vdsatcv2_dn4, locals.var_vdsatcv2_dn5, locals.var_vdsatcv2_dn6, (locals.var_vdsatcv2_dn7 - locals.var_vds_1_dn7), (locals.var_vdsatcv2_dn8 - locals.var_vds_1_dn8), locals.var_vdsatcv2_dn9, locals.var_vdsatcv2_dn10, locals.var_vdsatcv2_dn11, locals.var_vdsatcv2_dn12,)
    } else {
        (locals.var_v4, locals.var_v4_dn3, locals.var_v4_dn4, locals.var_v4_dn5, locals.var_v4_dn6, locals.var_v4_dn7, locals.var_v4_dn8, locals.var_v4_dn9, locals.var_v4_dn10, locals.var_v4_dn11, locals.var_v4_dn12,)
    }
};
        locals.var_v4 = assign28790_e26824;
        locals.var_v4_dn3 = assign28790_e26824_d_n3;
        locals.var_v4_dn4 = assign28790_e26824_d_n4;
        locals.var_v4_dn5 = assign28790_e26824_d_n5;
        locals.var_v4_dn6 = assign28790_e26824_d_n6;
        locals.var_v4_dn7 = assign28790_e26824_d_n7;
        locals.var_v4_dn8 = assign28790_e26824_d_n8;
        locals.var_v4_dn9 = assign28790_e26824_d_n9;
        locals.var_v4_dn10 = assign28790_e26824_d_n10;
        locals.var_v4_dn11 = assign28790_e26824_d_n11;
        locals.var_v4_dn12 = assign28790_e26824_d_n12;
        locals.var_v4_rv = 0.0;

        let (assign28800_e26842, assign28800_e26842_d_n3, assign28800_e26842_d_n4, assign28800_e26842_d_n5, assign28800_e26842_d_n6, assign28800_e26842_d_n7, assign28800_e26842_d_n8, assign28800_e26842_d_n9, assign28800_e26842_d_n10, assign28800_e26842_d_n11, assign28800_e26842_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1411 != 0.0)) {
        let assign28800_e26833: f64 = (locals.var_v4 * locals.var_v4);
        let assign28800_e26836: f64 = (4.0 * 0.02);
        let assign28800_e26838: f64 = (assign28800_e26836 * locals.var_vdsatcv2);
        let assign28800_e26839: f64 = (assign28800_e26833 + assign28800_e26838);
        let assign28800_e26840: f64 = (assign28800_e26839).sqrt();
        (assign28800_e26840, ((((locals.var_v4_dn3 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn3)) + (assign28800_e26836 * locals.var_vdsatcv2_dn3)) / (2.0 * assign28800_e26840)), ((((locals.var_v4_dn4 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn4)) + (assign28800_e26836 * locals.var_vdsatcv2_dn4)) / (2.0 * assign28800_e26840)), ((((locals.var_v4_dn5 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn5)) + (assign28800_e26836 * locals.var_vdsatcv2_dn5)) / (2.0 * assign28800_e26840)), ((((locals.var_v4_dn6 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn6)) + (assign28800_e26836 * locals.var_vdsatcv2_dn6)) / (2.0 * assign28800_e26840)), ((((locals.var_v4_dn7 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn7)) + (assign28800_e26836 * locals.var_vdsatcv2_dn7)) / (2.0 * assign28800_e26840)), ((((locals.var_v4_dn8 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn8)) + (assign28800_e26836 * locals.var_vdsatcv2_dn8)) / (2.0 * assign28800_e26840)), ((((locals.var_v4_dn9 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn9)) + (assign28800_e26836 * locals.var_vdsatcv2_dn9)) / (2.0 * assign28800_e26840)), ((((locals.var_v4_dn10 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn10)) + (assign28800_e26836 * locals.var_vdsatcv2_dn10)) / (2.0 * assign28800_e26840)), ((((locals.var_v4_dn11 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn11)) + (assign28800_e26836 * locals.var_vdsatcv2_dn11)) / (2.0 * assign28800_e26840)), ((((locals.var_v4_dn12 * locals.var_v4) + (locals.var_v4 * locals.var_v4_dn12)) + (assign28800_e26836 * locals.var_vdsatcv2_dn12)) / (2.0 * assign28800_e26840)),)
    } else {
        (locals.var_t02, locals.var_t02_dn3, locals.var_t02_dn4, locals.var_t02_dn5, locals.var_t02_dn6, locals.var_t02_dn7, locals.var_t02_dn8, locals.var_t02_dn9, locals.var_t02_dn10, locals.var_t02_dn11, locals.var_t02_dn12,)
    }
};
        locals.var_t02 = assign28800_e26842;
        locals.var_t02_dn3 = assign28800_e26842_d_n3;
        locals.var_t02_dn4 = assign28800_e26842_d_n4;
        locals.var_t02_dn5 = assign28800_e26842_d_n5;
        locals.var_t02_dn6 = assign28800_e26842_d_n6;
        locals.var_t02_dn7 = assign28800_e26842_d_n7;
        locals.var_t02_dn8 = assign28800_e26842_d_n8;
        locals.var_t02_dn9 = assign28800_e26842_d_n9;
        locals.var_t02_dn10 = assign28800_e26842_d_n10;
        locals.var_t02_dn11 = assign28800_e26842_d_n11;
        locals.var_t02_dn12 = assign28800_e26842_d_n12;
        locals.var_t02_rv = 0.0;

        let (assign28810_e26857, assign28810_e26857_d_n3, assign28810_e26857_d_n4, assign28810_e26857_d_n5, assign28810_e26857_d_n6, assign28810_e26857_d_n7, assign28810_e26857_d_n8, assign28810_e26857_d_n9, assign28810_e26857_d_n10, assign28810_e26857_d_n11, assign28810_e26857_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1411 != 0.0)) {
        let assign28810_e26853: f64 = (locals.var_v4 + locals.var_t02);
        let assign28810_e26854: f64 = (0.5 * assign28810_e26853);
        let assign28810_e26855: f64 = (locals.var_vdsatcv2 - assign28810_e26854);
        (assign28810_e26855, (locals.var_vdsatcv2_dn3 - (0.5 * (locals.var_v4_dn3 + locals.var_t02_dn3))), (locals.var_vdsatcv2_dn4 - (0.5 * (locals.var_v4_dn4 + locals.var_t02_dn4))), (locals.var_vdsatcv2_dn5 - (0.5 * (locals.var_v4_dn5 + locals.var_t02_dn5))), (locals.var_vdsatcv2_dn6 - (0.5 * (locals.var_v4_dn6 + locals.var_t02_dn6))), (locals.var_vdsatcv2_dn7 - (0.5 * (locals.var_v4_dn7 + locals.var_t02_dn7))), (locals.var_vdsatcv2_dn8 - (0.5 * (locals.var_v4_dn8 + locals.var_t02_dn8))), (locals.var_vdsatcv2_dn9 - (0.5 * (locals.var_v4_dn9 + locals.var_t02_dn9))), (locals.var_vdsatcv2_dn10 - (0.5 * (locals.var_v4_dn10 + locals.var_t02_dn10))), (locals.var_vdsatcv2_dn11 - (0.5 * (locals.var_v4_dn11 + locals.var_t02_dn11))), (locals.var_vdsatcv2_dn12 - (0.5 * (locals.var_v4_dn12 + locals.var_t02_dn12))),)
    } else {
        (locals.var_vdseffcv2, locals.var_vdseffcv2_dn3, locals.var_vdseffcv2_dn4, locals.var_vdseffcv2_dn5, locals.var_vdseffcv2_dn6, locals.var_vdseffcv2_dn7, locals.var_vdseffcv2_dn8, locals.var_vdseffcv2_dn9, locals.var_vdseffcv2_dn10, locals.var_vdseffcv2_dn11, locals.var_vdseffcv2_dn12,)
    }
};
        locals.var_vdseffcv2 = assign28810_e26857;
        locals.var_vdseffcv2_dn3 = assign28810_e26857_d_n3;
        locals.var_vdseffcv2_dn4 = assign28810_e26857_d_n4;
        locals.var_vdseffcv2_dn5 = assign28810_e26857_d_n5;
        locals.var_vdseffcv2_dn6 = assign28810_e26857_d_n6;
        locals.var_vdseffcv2_dn7 = assign28810_e26857_d_n7;
        locals.var_vdseffcv2_dn8 = assign28810_e26857_d_n8;
        locals.var_vdseffcv2_dn9 = assign28810_e26857_d_n9;
        locals.var_vdseffcv2_dn10 = assign28810_e26857_d_n10;
        locals.var_vdseffcv2_dn11 = assign28810_e26857_d_n11;
        locals.var_vdseffcv2_dn12 = assign28810_e26857_d_n12;
        locals.var_vdseffcv2_rv = 0.0;

        let (assign28820_e26868, assign28820_e26868_d_n3, assign28820_e26868_d_n4, assign28820_e26868_d_n5, assign28820_e26868_d_n6, assign28820_e26868_d_n7, assign28820_e26868_d_n8, assign28820_e26868_d_n9, assign28820_e26868_d_n10, assign28820_e26868_d_n11, assign28820_e26868_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1411 != 0.0)) {
        let assign28820_e26866: f64 = (locals.var_abulkcv * locals.var_vdseffcv2);
        (assign28820_e26866, ((locals.var_abulkcv_dn3 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn3)), ((locals.var_abulkcv_dn4 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn4)), ((locals.var_abulkcv_dn5 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn5)), ((locals.var_abulkcv_dn6 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn6)), ((locals.var_abulkcv_dn7 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn7)), ((locals.var_abulkcv_dn8 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn8)), ((locals.var_abulkcv_dn9 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn9)), ((locals.var_abulkcv_dn10 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn10)), ((locals.var_abulkcv_dn11 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn11)), ((locals.var_abulkcv_dn12 * locals.var_vdseffcv2) + (locals.var_abulkcv * locals.var_vdseffcv2_dn12)),)
    } else {
        (locals.var_t02, locals.var_t02_dn3, locals.var_t02_dn4, locals.var_t02_dn5, locals.var_t02_dn6, locals.var_t02_dn7, locals.var_t02_dn8, locals.var_t02_dn9, locals.var_t02_dn10, locals.var_t02_dn11, locals.var_t02_dn12,)
    }
};
        locals.var_t02 = assign28820_e26868;
        locals.var_t02_dn3 = assign28820_e26868_d_n3;
        locals.var_t02_dn4 = assign28820_e26868_d_n4;
        locals.var_t02_dn5 = assign28820_e26868_d_n5;
        locals.var_t02_dn6 = assign28820_e26868_d_n6;
        locals.var_t02_dn7 = assign28820_e26868_d_n7;
        locals.var_t02_dn8 = assign28820_e26868_d_n8;
        locals.var_t02_dn9 = assign28820_e26868_d_n9;
        locals.var_t02_dn10 = assign28820_e26868_d_n10;
        locals.var_t02_dn11 = assign28820_e26868_d_n11;
        locals.var_t02_dn12 = assign28820_e26868_d_n12;
        locals.var_t02_rv = 0.0;

        let (assign28830_e26885, assign28830_e26885_d_n3, assign28830_e26885_d_n4, assign28830_e26885_d_n5, assign28830_e26885_d_n6, assign28830_e26885_d_n7, assign28830_e26885_d_n8, assign28830_e26885_d_n9, assign28830_e26885_d_n10, assign28830_e26885_d_n11, assign28830_e26885_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1411 != 0.0)) {
        let assign28830_e26879: f64 = (0.5 * locals.var_t02);
        let assign28830_e26880: f64 = (locals.var_t12 - assign28830_e26879);
        let assign28830_e26882: f64 = (assign28830_e26880 + 1e-20);
        let assign28830_e26883: f64 = (12.0 * assign28830_e26882);
        (assign28830_e26883, (12.0 * (locals.var_t12_dn3 - (0.5 * locals.var_t02_dn3))), (12.0 * (locals.var_t12_dn4 - (0.5 * locals.var_t02_dn4))), (12.0 * (locals.var_t12_dn5 - (0.5 * locals.var_t02_dn5))), (12.0 * (locals.var_t12_dn6 - (0.5 * locals.var_t02_dn6))), (12.0 * (locals.var_t12_dn7 - (0.5 * locals.var_t02_dn7))), (12.0 * (locals.var_t12_dn8 - (0.5 * locals.var_t02_dn8))), (12.0 * (locals.var_t12_dn9 - (0.5 * locals.var_t02_dn9))), (12.0 * (locals.var_t12_dn10 - (0.5 * locals.var_t02_dn10))), (12.0 * (locals.var_t12_dn11 - (0.5 * locals.var_t02_dn11))), (12.0 * (locals.var_t12_dn12 - (0.5 * locals.var_t02_dn12))),)
    } else {
        (locals.var_t22, locals.var_t22_dn3, locals.var_t22_dn4, locals.var_t22_dn5, locals.var_t22_dn6, locals.var_t22_dn7, locals.var_t22_dn8, locals.var_t22_dn9, locals.var_t22_dn10, locals.var_t22_dn11, locals.var_t22_dn12,)
    }
};
        locals.var_t22 = assign28830_e26885;
        locals.var_t22_dn3 = assign28830_e26885_d_n3;
        locals.var_t22_dn4 = assign28830_e26885_d_n4;
        locals.var_t22_dn5 = assign28830_e26885_d_n5;
        locals.var_t22_dn6 = assign28830_e26885_d_n6;
        locals.var_t22_dn7 = assign28830_e26885_d_n7;
        locals.var_t22_dn8 = assign28830_e26885_d_n8;
        locals.var_t22_dn9 = assign28830_e26885_d_n9;
        locals.var_t22_dn10 = assign28830_e26885_d_n10;
        locals.var_t22_dn11 = assign28830_e26885_d_n11;
        locals.var_t22_dn12 = assign28830_e26885_d_n12;
        locals.var_t22_rv = 0.0;

        let (assign28840_e26896, assign28840_e26896_d_n3, assign28840_e26896_d_n4, assign28840_e26896_d_n5, assign28840_e26896_d_n6, assign28840_e26896_d_n7, assign28840_e26896_d_n8, assign28840_e26896_d_n9, assign28840_e26896_d_n10, assign28840_e26896_d_n11, assign28840_e26896_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1411 != 0.0)) {
        let assign28840_e26894: f64 = (locals.var_t02 / locals.var_t22);
        (assign28840_e26894, (((locals.var_t02_dn3 * locals.var_t22) - (locals.var_t02 * locals.var_t22_dn3)) / (locals.var_t22 * locals.var_t22)), (((locals.var_t02_dn4 * locals.var_t22) - (locals.var_t02 * locals.var_t22_dn4)) / (locals.var_t22 * locals.var_t22)), (((locals.var_t02_dn5 * locals.var_t22) - (locals.var_t02 * locals.var_t22_dn5)) / (locals.var_t22 * locals.var_t22)), (((locals.var_t02_dn6 * locals.var_t22) - (locals.var_t02 * locals.var_t22_dn6)) / (locals.var_t22 * locals.var_t22)), (((locals.var_t02_dn7 * locals.var_t22) - (locals.var_t02 * locals.var_t22_dn7)) / (locals.var_t22 * locals.var_t22)), (((locals.var_t02_dn8 * locals.var_t22) - (locals.var_t02 * locals.var_t22_dn8)) / (locals.var_t22 * locals.var_t22)), (((locals.var_t02_dn9 * locals.var_t22) - (locals.var_t02 * locals.var_t22_dn9)) / (locals.var_t22 * locals.var_t22)), (((locals.var_t02_dn10 * locals.var_t22) - (locals.var_t02 * locals.var_t22_dn10)) / (locals.var_t22 * locals.var_t22)), (((locals.var_t02_dn11 * locals.var_t22) - (locals.var_t02 * locals.var_t22_dn11)) / (locals.var_t22 * locals.var_t22)), (((locals.var_t02_dn12 * locals.var_t22) - (locals.var_t02 * locals.var_t22_dn12)) / (locals.var_t22 * locals.var_t22)),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign28840_e26896;
        locals.var_t3__blk811_dn3 = assign28840_e26896_d_n3;
        locals.var_t3__blk811_dn4 = assign28840_e26896_d_n4;
        locals.var_t3__blk811_dn5 = assign28840_e26896_d_n5;
        locals.var_t3__blk811_dn6 = assign28840_e26896_d_n6;
        locals.var_t3__blk811_dn7 = assign28840_e26896_d_n7;
        locals.var_t3__blk811_dn8 = assign28840_e26896_d_n8;
        locals.var_t3__blk811_dn9 = assign28840_e26896_d_n9;
        locals.var_t3__blk811_dn10 = assign28840_e26896_d_n10;
        locals.var_t3__blk811_dn11 = assign28840_e26896_d_n11;
        locals.var_t3__blk811_dn12 = assign28840_e26896_d_n12;
        locals.var_t3__blk811_rv = 0.0;

        let (assign28850_e26913, assign28850_e26913_d_n3, assign28850_e26913_d_n4, assign28850_e26913_d_n5, assign28850_e26913_d_n6, assign28850_e26913_d_n7, assign28850_e26913_d_n8, assign28850_e26913_d_n9, assign28850_e26913_d_n10, assign28850_e26913_d_n11, assign28850_e26913_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1411 != 0.0)) {
        let assign28850_e26908: f64 = (0.5 - locals.var_t3__blk811);
        let assign28850_e26909: f64 = (locals.var_t02 * assign28850_e26908);
        let assign28850_e26910: f64 = (locals.var_t12 - assign28850_e26909);
        let assign28850_e26911: f64 = (locals.var_coxwlcen2 * assign28850_e26910);
        (assign28850_e26911, ((locals.var_coxwlcen2_dn3 * assign28850_e26910) + (locals.var_coxwlcen2 * (locals.var_t12_dn3 - ((locals.var_t02_dn3 * assign28850_e26908) + (locals.var_t02 * (-locals.var_t3__blk811_dn3)))))), ((locals.var_coxwlcen2_dn4 * assign28850_e26910) + (locals.var_coxwlcen2 * (locals.var_t12_dn4 - ((locals.var_t02_dn4 * assign28850_e26908) + (locals.var_t02 * (-locals.var_t3__blk811_dn4)))))), ((locals.var_coxwlcen2_dn5 * assign28850_e26910) + (locals.var_coxwlcen2 * (locals.var_t12_dn5 - ((locals.var_t02_dn5 * assign28850_e26908) + (locals.var_t02 * (-locals.var_t3__blk811_dn5)))))), ((locals.var_coxwlcen2_dn6 * assign28850_e26910) + (locals.var_coxwlcen2 * (locals.var_t12_dn6 - ((locals.var_t02_dn6 * assign28850_e26908) + (locals.var_t02 * (-locals.var_t3__blk811_dn6)))))), ((locals.var_coxwlcen2_dn7 * assign28850_e26910) + (locals.var_coxwlcen2 * (locals.var_t12_dn7 - ((locals.var_t02_dn7 * assign28850_e26908) + (locals.var_t02 * (-locals.var_t3__blk811_dn7)))))), ((locals.var_coxwlcen2_dn8 * assign28850_e26910) + (locals.var_coxwlcen2 * (locals.var_t12_dn8 - ((locals.var_t02_dn8 * assign28850_e26908) + (locals.var_t02 * (-locals.var_t3__blk811_dn8)))))), ((locals.var_coxwlcen2_dn9 * assign28850_e26910) + (locals.var_coxwlcen2 * (locals.var_t12_dn9 - ((locals.var_t02_dn9 * assign28850_e26908) + (locals.var_t02 * (-locals.var_t3__blk811_dn9)))))), ((locals.var_coxwlcen2_dn10 * assign28850_e26910) + (locals.var_coxwlcen2 * (locals.var_t12_dn10 - ((locals.var_t02_dn10 * assign28850_e26908) + (locals.var_t02 * (-locals.var_t3__blk811_dn10)))))), ((locals.var_coxwlcen2_dn11 * assign28850_e26910) + (locals.var_coxwlcen2 * (locals.var_t12_dn11 - ((locals.var_t02_dn11 * assign28850_e26908) + (locals.var_t02 * (-locals.var_t3__blk811_dn11)))))), ((locals.var_coxwlcen2_dn12 * assign28850_e26910) + (locals.var_coxwlcen2 * (locals.var_t12_dn12 - ((locals.var_t02_dn12 * assign28850_e26908) + (locals.var_t02 * (-locals.var_t3__blk811_dn12)))))),)
    } else {
        (locals.var_t7__blk815, locals.var_t7__blk815_dn3, locals.var_t7__blk815_dn4, locals.var_t7__blk815_dn5, locals.var_t7__blk815_dn6, locals.var_t7__blk815_dn7, locals.var_t7__blk815_dn8, locals.var_t7__blk815_dn9, locals.var_t7__blk815_dn10, locals.var_t7__blk815_dn11, locals.var_t7__blk815_dn12,)
    }
};
        locals.var_t7__blk815 = assign28850_e26913;
        locals.var_t7__blk815_dn3 = assign28850_e26913_d_n3;
        locals.var_t7__blk815_dn4 = assign28850_e26913_d_n4;
        locals.var_t7__blk815_dn5 = assign28850_e26913_d_n5;
        locals.var_t7__blk815_dn6 = assign28850_e26913_d_n6;
        locals.var_t7__blk815_dn7 = assign28850_e26913_d_n7;
        locals.var_t7__blk815_dn8 = assign28850_e26913_d_n8;
        locals.var_t7__blk815_dn9 = assign28850_e26913_d_n9;
        locals.var_t7__blk815_dn10 = assign28850_e26913_d_n10;
        locals.var_t7__blk815_dn11 = assign28850_e26913_d_n11;
        locals.var_t7__blk815_dn12 = assign28850_e26913_d_n12;
        locals.var_t7__blk815_rv = 0.0;

        let (assign28860_e26924, assign28860_e26924_d_n3, assign28860_e26924_d_n4, assign28860_e26924_d_n5, assign28860_e26924_d_n6, assign28860_e26924_d_n7, assign28860_e26924_d_n8, assign28860_e26924_d_n9, assign28860_e26924_d_n10, assign28860_e26924_d_n11, assign28860_e26924_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1411 != 0.0)) {
        let assign28860_e26922: f64 = (locals.var_qinv + locals.var_t7__blk815);
        (assign28860_e26922, (locals.var_qinv_dn3 + locals.var_t7__blk815_dn3), (locals.var_qinv_dn4 + locals.var_t7__blk815_dn4), (locals.var_qinv_dn5 + locals.var_t7__blk815_dn5), (locals.var_qinv_dn6 + locals.var_t7__blk815_dn6), (locals.var_qinv_dn7 + locals.var_t7__blk815_dn7), (locals.var_qinv_dn8 + locals.var_t7__blk815_dn8), (locals.var_qinv_dn9 + locals.var_t7__blk815_dn9), (locals.var_qinv_dn10 + locals.var_t7__blk815_dn10), (locals.var_qinv_dn11 + locals.var_t7__blk815_dn11), (locals.var_qinv_dn12 + locals.var_t7__blk815_dn12),)
    } else {
        (locals.var_qinv, locals.var_qinv_dn3, locals.var_qinv_dn4, locals.var_qinv_dn5, locals.var_qinv_dn6, locals.var_qinv_dn7, locals.var_qinv_dn8, locals.var_qinv_dn9, locals.var_qinv_dn10, locals.var_qinv_dn11, locals.var_qinv_dn12,)
    }
};
        locals.var_qinv = assign28860_e26924;
        locals.var_qinv_dn3 = assign28860_e26924_d_n3;
        locals.var_qinv_dn4 = assign28860_e26924_d_n4;
        locals.var_qinv_dn5 = assign28860_e26924_d_n5;
        locals.var_qinv_dn6 = assign28860_e26924_d_n6;
        locals.var_qinv_dn7 = assign28860_e26924_d_n7;
        locals.var_qinv_dn8 = assign28860_e26924_d_n8;
        locals.var_qinv_dn9 = assign28860_e26924_d_n9;
        locals.var_qinv_dn10 = assign28860_e26924_d_n10;
        locals.var_qinv_dn11 = assign28860_e26924_d_n11;
        locals.var_qinv_dn12 = assign28860_e26924_d_n12;
        locals.var_qinv_rv = 0.0;

        let (assign28880_e26942, assign28880_e26942_d_n3, assign28880_e26942_d_n4, assign28880_e26942_d_n5, assign28880_e26942_d_n6, assign28880_e26942_d_n7, assign28880_e26942_d_n8, assign28880_e26942_d_n9, assign28880_e26942_d_n10, assign28880_e26942_d_n11, assign28880_e26942_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1411 != 0.0)) {
        (locals.var_qinv, locals.var_qinv_dn3, locals.var_qinv_dn4, locals.var_qinv_dn5, locals.var_qinv_dn6, locals.var_qinv_dn7, locals.var_qinv_dn8, locals.var_qinv_dn9, locals.var_qinv_dn10, locals.var_qinv_dn11, locals.var_qinv_dn12,)
    } else {
        (locals.var_qgate, locals.var_qgate_dn3, locals.var_qgate_dn4, locals.var_qgate_dn5, locals.var_qgate_dn6, locals.var_qgate_dn7, locals.var_qgate_dn8, locals.var_qgate_dn9, locals.var_qgate_dn10, locals.var_qgate_dn11, locals.var_qgate_dn12,)
    }
};
        locals.var_qgate = assign28880_e26942;
        locals.var_qgate_dn3 = assign28880_e26942_d_n3;
        locals.var_qgate_dn4 = assign28880_e26942_d_n4;
        locals.var_qgate_dn5 = assign28880_e26942_d_n5;
        locals.var_qgate_dn6 = assign28880_e26942_d_n6;
        locals.var_qgate_dn7 = assign28880_e26942_d_n7;
        locals.var_qgate_dn8 = assign28880_e26942_d_n8;
        locals.var_qgate_dn9 = assign28880_e26942_d_n9;
        locals.var_qgate_dn10 = assign28880_e26942_d_n10;
        locals.var_qgate_dn11 = assign28880_e26942_d_n11;
        locals.var_qgate_dn12 = assign28880_e26942_d_n12;
        locals.var_qgate_rv = 0.0;

        let assign28890_e26945: f64 = if locals.var_b4soisoimod == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1412 = assign28890_e26945;
        locals.var_guard1412_rv = 0.0;

        let (assign28900_e26954, assign28900_e26954_d_n3, assign28900_e26954_d_n4, assign28900_e26954_d_n5, assign28900_e26954_d_n6, assign28900_e26954_d_n7, assign28900_e26954_d_n8, assign28900_e26954_d_n9, assign28900_e26954_d_n10, assign28900_e26954_d_n11, assign28900_e26954_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1412 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbulk, locals.var_qbulk_dn3, locals.var_qbulk_dn4, locals.var_qbulk_dn5, locals.var_qbulk_dn6, locals.var_qbulk_dn7, locals.var_qbulk_dn8, locals.var_qbulk_dn9, locals.var_qbulk_dn10, locals.var_qbulk_dn11, locals.var_qbulk_dn12,)
    }
};
        locals.var_qbulk = assign28900_e26954;
        locals.var_qbulk_dn3 = assign28900_e26954_d_n3;
        locals.var_qbulk_dn4 = assign28900_e26954_d_n4;
        locals.var_qbulk_dn5 = assign28900_e26954_d_n5;
        locals.var_qbulk_dn6 = assign28900_e26954_d_n6;
        locals.var_qbulk_dn7 = assign28900_e26954_d_n7;
        locals.var_qbulk_dn8 = assign28900_e26954_d_n8;
        locals.var_qbulk_dn9 = assign28900_e26954_d_n9;
        locals.var_qbulk_dn10 = assign28900_e26954_d_n10;
        locals.var_qbulk_dn11 = assign28900_e26954_d_n11;
        locals.var_qbulk_dn12 = assign28900_e26954_d_n12;
        locals.var_qbulk_rv = 0.0;

        let (assign28910_e26966, assign28910_e26966_d_n3, assign28910_e26966_d_n4, assign28910_e26966_d_n5, assign28910_e26966_d_n6, assign28910_e26966_d_n7, assign28910_e26966_d_n8, assign28910_e26966_d_n9, assign28910_e26966_d_n10, assign28910_e26966_d_n11, assign28910_e26966_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1412 == 0.0)) {
        let assign28910_e26964: f64 = (1.0 - locals.var_abulkcv);
        (assign28910_e26964, (-locals.var_abulkcv_dn3), (-locals.var_abulkcv_dn4), (-locals.var_abulkcv_dn5), (-locals.var_abulkcv_dn6), (-locals.var_abulkcv_dn7), (-locals.var_abulkcv_dn8), (-locals.var_abulkcv_dn9), (-locals.var_abulkcv_dn10), (-locals.var_abulkcv_dn11), (-locals.var_abulkcv_dn12),)
    } else {
        (locals.var_t7__blk815, locals.var_t7__blk815_dn3, locals.var_t7__blk815_dn4, locals.var_t7__blk815_dn5, locals.var_t7__blk815_dn6, locals.var_t7__blk815_dn7, locals.var_t7__blk815_dn8, locals.var_t7__blk815_dn9, locals.var_t7__blk815_dn10, locals.var_t7__blk815_dn11, locals.var_t7__blk815_dn12,)
    }
};
        locals.var_t7__blk815 = assign28910_e26966;
        locals.var_t7__blk815_dn3 = assign28910_e26966_d_n3;
        locals.var_t7__blk815_dn4 = assign28910_e26966_d_n4;
        locals.var_t7__blk815_dn5 = assign28910_e26966_d_n5;
        locals.var_t7__blk815_dn6 = assign28910_e26966_d_n6;
        locals.var_t7__blk815_dn7 = assign28910_e26966_d_n7;
        locals.var_t7__blk815_dn8 = assign28910_e26966_d_n8;
        locals.var_t7__blk815_dn9 = assign28910_e26966_d_n9;
        locals.var_t7__blk815_dn10 = assign28910_e26966_d_n10;
        locals.var_t7__blk815_dn11 = assign28910_e26966_d_n11;
        locals.var_t7__blk815_dn12 = assign28910_e26966_d_n12;
        locals.var_t7__blk815_rv = 0.0;

        let (assign28920_e26988, assign28920_e26988_d_n3, assign28920_e26988_d_n4, assign28920_e26988_d_n5, assign28920_e26988_d_n6, assign28920_e26988_d_n7, assign28920_e26988_d_n8, assign28920_e26988_d_n9, assign28920_e26988_d_n10, assign28920_e26988_d_n11, assign28920_e26988_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1412 == 0.0)) {
        let assign28920_e26976: f64 = (locals.var_coxwlcenb * locals.var_t7__blk815);
        let assign28920_e26979: f64 = (0.5 * locals.var_vdseffcv);
        let assign28920_e26982: f64 = (locals.var_t0__blk808 * locals.var_vdseffcv);
        let assign28920_e26984: f64 = (assign28920_e26982 / locals.var_t2__blk810);
        let assign28920_e26985: f64 = (assign28920_e26979 - assign28920_e26984);
        let assign28920_e26986: f64 = (assign28920_e26976 * assign28920_e26985);
        (assign28920_e26986, ((((locals.var_coxwlcenb_dn3 * locals.var_t7__blk815) + (locals.var_coxwlcenb * locals.var_t7__blk815_dn3)) * assign28920_e26985) + (assign28920_e26976 * ((0.5 * locals.var_vdseffcv_dn3) - (((((locals.var_t0__blk808_dn3 * locals.var_vdseffcv) + (locals.var_t0__blk808 * locals.var_vdseffcv_dn3)) * locals.var_t2__blk810) - (assign28920_e26982 * locals.var_t2__blk810_dn3)) / (locals.var_t2__blk810 * locals.var_t2__blk810))))), ((((locals.var_coxwlcenb_dn4 * locals.var_t7__blk815) + (locals.var_coxwlcenb * locals.var_t7__blk815_dn4)) * assign28920_e26985) + (assign28920_e26976 * ((0.5 * locals.var_vdseffcv_dn4) - (((((locals.var_t0__blk808_dn4 * locals.var_vdseffcv) + (locals.var_t0__blk808 * locals.var_vdseffcv_dn4)) * locals.var_t2__blk810) - (assign28920_e26982 * locals.var_t2__blk810_dn4)) / (locals.var_t2__blk810 * locals.var_t2__blk810))))), ((((locals.var_coxwlcenb_dn5 * locals.var_t7__blk815) + (locals.var_coxwlcenb * locals.var_t7__blk815_dn5)) * assign28920_e26985) + (assign28920_e26976 * ((0.5 * locals.var_vdseffcv_dn5) - (((((locals.var_t0__blk808_dn5 * locals.var_vdseffcv) + (locals.var_t0__blk808 * locals.var_vdseffcv_dn5)) * locals.var_t2__blk810) - (assign28920_e26982 * locals.var_t2__blk810_dn5)) / (locals.var_t2__blk810 * locals.var_t2__blk810))))), ((((locals.var_coxwlcenb_dn6 * locals.var_t7__blk815) + (locals.var_coxwlcenb * locals.var_t7__blk815_dn6)) * assign28920_e26985) + (assign28920_e26976 * ((0.5 * locals.var_vdseffcv_dn6) - (((((locals.var_t0__blk808_dn6 * locals.var_vdseffcv) + (locals.var_t0__blk808 * locals.var_vdseffcv_dn6)) * locals.var_t2__blk810) - (assign28920_e26982 * locals.var_t2__blk810_dn6)) / (locals.var_t2__blk810 * locals.var_t2__blk810))))), ((((locals.var_coxwlcenb_dn7 * locals.var_t7__blk815) + (locals.var_coxwlcenb * locals.var_t7__blk815_dn7)) * assign28920_e26985) + (assign28920_e26976 * ((0.5 * locals.var_vdseffcv_dn7) - (((((locals.var_t0__blk808_dn7 * locals.var_vdseffcv) + (locals.var_t0__blk808 * locals.var_vdseffcv_dn7)) * locals.var_t2__blk810) - (assign28920_e26982 * locals.var_t2__blk810_dn7)) / (locals.var_t2__blk810 * locals.var_t2__blk810))))), ((((locals.var_coxwlcenb_dn8 * locals.var_t7__blk815) + (locals.var_coxwlcenb * locals.var_t7__blk815_dn8)) * assign28920_e26985) + (assign28920_e26976 * ((0.5 * locals.var_vdseffcv_dn8) - (((((locals.var_t0__blk808_dn8 * locals.var_vdseffcv) + (locals.var_t0__blk808 * locals.var_vdseffcv_dn8)) * locals.var_t2__blk810) - (assign28920_e26982 * locals.var_t2__blk810_dn8)) / (locals.var_t2__blk810 * locals.var_t2__blk810))))), ((((locals.var_coxwlcenb_dn9 * locals.var_t7__blk815) + (locals.var_coxwlcenb * locals.var_t7__blk815_dn9)) * assign28920_e26985) + (assign28920_e26976 * ((0.5 * locals.var_vdseffcv_dn9) - (((((locals.var_t0__blk808_dn9 * locals.var_vdseffcv) + (locals.var_t0__blk808 * locals.var_vdseffcv_dn9)) * locals.var_t2__blk810) - (assign28920_e26982 * locals.var_t2__blk810_dn9)) / (locals.var_t2__blk810 * locals.var_t2__blk810))))), ((((locals.var_coxwlcenb_dn10 * locals.var_t7__blk815) + (locals.var_coxwlcenb * locals.var_t7__blk815_dn10)) * assign28920_e26985) + (assign28920_e26976 * ((0.5 * locals.var_vdseffcv_dn10) - (((((locals.var_t0__blk808_dn10 * locals.var_vdseffcv) + (locals.var_t0__blk808 * locals.var_vdseffcv_dn10)) * locals.var_t2__blk810) - (assign28920_e26982 * locals.var_t2__blk810_dn10)) / (locals.var_t2__blk810 * locals.var_t2__blk810))))), ((((locals.var_coxwlcenb_dn11 * locals.var_t7__blk815) + (locals.var_coxwlcenb * locals.var_t7__blk815_dn11)) * assign28920_e26985) + (assign28920_e26976 * ((0.5 * locals.var_vdseffcv_dn11) - (((((locals.var_t0__blk808_dn11 * locals.var_vdseffcv) + (locals.var_t0__blk808 * locals.var_vdseffcv_dn11)) * locals.var_t2__blk810) - (assign28920_e26982 * locals.var_t2__blk810_dn11)) / (locals.var_t2__blk810 * locals.var_t2__blk810))))), ((((locals.var_coxwlcenb_dn12 * locals.var_t7__blk815) + (locals.var_coxwlcenb * locals.var_t7__blk815_dn12)) * assign28920_e26985) + (assign28920_e26976 * ((0.5 * locals.var_vdseffcv_dn12) - (((((locals.var_t0__blk808_dn12 * locals.var_vdseffcv) + (locals.var_t0__blk808 * locals.var_vdseffcv_dn12)) * locals.var_t2__blk810) - (assign28920_e26982 * locals.var_t2__blk810_dn12)) / (locals.var_t2__blk810 * locals.var_t2__blk810))))),)
    } else {
        (locals.var_qbulk, locals.var_qbulk_dn3, locals.var_qbulk_dn4, locals.var_qbulk_dn5, locals.var_qbulk_dn6, locals.var_qbulk_dn7, locals.var_qbulk_dn8, locals.var_qbulk_dn9, locals.var_qbulk_dn10, locals.var_qbulk_dn11, locals.var_qbulk_dn12,)
    }
};
        locals.var_qbulk = assign28920_e26988;
        locals.var_qbulk_dn3 = assign28920_e26988_d_n3;
        locals.var_qbulk_dn4 = assign28920_e26988_d_n4;
        locals.var_qbulk_dn5 = assign28920_e26988_d_n5;
        locals.var_qbulk_dn6 = assign28920_e26988_d_n6;
        locals.var_qbulk_dn7 = assign28920_e26988_d_n7;
        locals.var_qbulk_dn8 = assign28920_e26988_d_n8;
        locals.var_qbulk_dn9 = assign28920_e26988_d_n9;
        locals.var_qbulk_dn10 = assign28920_e26988_d_n10;
        locals.var_qbulk_dn11 = assign28920_e26988_d_n11;
        locals.var_qbulk_dn12 = assign28920_e26988_d_n12;
        locals.var_qbulk_rv = 0.0;

        let assign28930_e26999: f64 = if (((locals.var_b4soisoimod != 2.0) && (locals.var_b4soibodymod != 0.0)) && (p.p27 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1413 = assign28930_e26999;
        locals.var_guard1413_rv = 0.0;

        let (assign28940_e27023, assign28940_e27023_d_n3, assign28940_e27023_d_n4, assign28940_e27023_d_n5, assign28940_e27023_d_n6, assign28940_e27023_d_n7, assign28940_e27023_d_n8, assign28940_e27023_d_n9, assign28940_e27023_d_n10, assign28940_e27023_d_n11, assign28940_e27023_d_n12,) = {
    if ((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1412 == 0.0)) && (locals.var_guard1413 != 0.0)) {
        let assign28940_e27011: f64 = (locals.var_coxwlcenb2 * locals.var_t7__blk815);
        let assign28940_e27014: f64 = (0.5 * locals.var_vdseffcv2);
        let assign28940_e27017: f64 = (locals.var_t02 * locals.var_vdseffcv2);
        let assign28940_e27019: f64 = (assign28940_e27017 / locals.var_t22);
        let assign28940_e27020: f64 = (assign28940_e27014 - assign28940_e27019);
        let assign28940_e27021: f64 = (assign28940_e27011 * assign28940_e27020);
        (assign28940_e27021, ((((locals.var_coxwlcenb2_dn3 * locals.var_t7__blk815) + (locals.var_coxwlcenb2 * locals.var_t7__blk815_dn3)) * assign28940_e27020) + (assign28940_e27011 * ((0.5 * locals.var_vdseffcv2_dn3) - (((((locals.var_t02_dn3 * locals.var_vdseffcv2) + (locals.var_t02 * locals.var_vdseffcv2_dn3)) * locals.var_t22) - (assign28940_e27017 * locals.var_t22_dn3)) / (locals.var_t22 * locals.var_t22))))), ((((locals.var_coxwlcenb2_dn4 * locals.var_t7__blk815) + (locals.var_coxwlcenb2 * locals.var_t7__blk815_dn4)) * assign28940_e27020) + (assign28940_e27011 * ((0.5 * locals.var_vdseffcv2_dn4) - (((((locals.var_t02_dn4 * locals.var_vdseffcv2) + (locals.var_t02 * locals.var_vdseffcv2_dn4)) * locals.var_t22) - (assign28940_e27017 * locals.var_t22_dn4)) / (locals.var_t22 * locals.var_t22))))), ((((locals.var_coxwlcenb2_dn5 * locals.var_t7__blk815) + (locals.var_coxwlcenb2 * locals.var_t7__blk815_dn5)) * assign28940_e27020) + (assign28940_e27011 * ((0.5 * locals.var_vdseffcv2_dn5) - (((((locals.var_t02_dn5 * locals.var_vdseffcv2) + (locals.var_t02 * locals.var_vdseffcv2_dn5)) * locals.var_t22) - (assign28940_e27017 * locals.var_t22_dn5)) / (locals.var_t22 * locals.var_t22))))), ((((locals.var_coxwlcenb2_dn6 * locals.var_t7__blk815) + (locals.var_coxwlcenb2 * locals.var_t7__blk815_dn6)) * assign28940_e27020) + (assign28940_e27011 * ((0.5 * locals.var_vdseffcv2_dn6) - (((((locals.var_t02_dn6 * locals.var_vdseffcv2) + (locals.var_t02 * locals.var_vdseffcv2_dn6)) * locals.var_t22) - (assign28940_e27017 * locals.var_t22_dn6)) / (locals.var_t22 * locals.var_t22))))), ((((locals.var_coxwlcenb2_dn7 * locals.var_t7__blk815) + (locals.var_coxwlcenb2 * locals.var_t7__blk815_dn7)) * assign28940_e27020) + (assign28940_e27011 * ((0.5 * locals.var_vdseffcv2_dn7) - (((((locals.var_t02_dn7 * locals.var_vdseffcv2) + (locals.var_t02 * locals.var_vdseffcv2_dn7)) * locals.var_t22) - (assign28940_e27017 * locals.var_t22_dn7)) / (locals.var_t22 * locals.var_t22))))), ((((locals.var_coxwlcenb2_dn8 * locals.var_t7__blk815) + (locals.var_coxwlcenb2 * locals.var_t7__blk815_dn8)) * assign28940_e27020) + (assign28940_e27011 * ((0.5 * locals.var_vdseffcv2_dn8) - (((((locals.var_t02_dn8 * locals.var_vdseffcv2) + (locals.var_t02 * locals.var_vdseffcv2_dn8)) * locals.var_t22) - (assign28940_e27017 * locals.var_t22_dn8)) / (locals.var_t22 * locals.var_t22))))), ((((locals.var_coxwlcenb2_dn9 * locals.var_t7__blk815) + (locals.var_coxwlcenb2 * locals.var_t7__blk815_dn9)) * assign28940_e27020) + (assign28940_e27011 * ((0.5 * locals.var_vdseffcv2_dn9) - (((((locals.var_t02_dn9 * locals.var_vdseffcv2) + (locals.var_t02 * locals.var_vdseffcv2_dn9)) * locals.var_t22) - (assign28940_e27017 * locals.var_t22_dn9)) / (locals.var_t22 * locals.var_t22))))), ((((locals.var_coxwlcenb2_dn10 * locals.var_t7__blk815) + (locals.var_coxwlcenb2 * locals.var_t7__blk815_dn10)) * assign28940_e27020) + (assign28940_e27011 * ((0.5 * locals.var_vdseffcv2_dn10) - (((((locals.var_t02_dn10 * locals.var_vdseffcv2) + (locals.var_t02 * locals.var_vdseffcv2_dn10)) * locals.var_t22) - (assign28940_e27017 * locals.var_t22_dn10)) / (locals.var_t22 * locals.var_t22))))), ((((locals.var_coxwlcenb2_dn11 * locals.var_t7__blk815) + (locals.var_coxwlcenb2 * locals.var_t7__blk815_dn11)) * assign28940_e27020) + (assign28940_e27011 * ((0.5 * locals.var_vdseffcv2_dn11) - (((((locals.var_t02_dn11 * locals.var_vdseffcv2) + (locals.var_t02 * locals.var_vdseffcv2_dn11)) * locals.var_t22) - (assign28940_e27017 * locals.var_t22_dn11)) / (locals.var_t22 * locals.var_t22))))), ((((locals.var_coxwlcenb2_dn12 * locals.var_t7__blk815) + (locals.var_coxwlcenb2 * locals.var_t7__blk815_dn12)) * assign28940_e27020) + (assign28940_e27011 * ((0.5 * locals.var_vdseffcv2_dn12) - (((((locals.var_t02_dn12 * locals.var_vdseffcv2) + (locals.var_t02 * locals.var_vdseffcv2_dn12)) * locals.var_t22) - (assign28940_e27017 * locals.var_t22_dn12)) / (locals.var_t22 * locals.var_t22))))),)
    } else {
        (locals.var_qbulk2, locals.var_qbulk2_dn3, locals.var_qbulk2_dn4, locals.var_qbulk2_dn5, locals.var_qbulk2_dn6, locals.var_qbulk2_dn7, locals.var_qbulk2_dn8, locals.var_qbulk2_dn9, locals.var_qbulk2_dn10, locals.var_qbulk2_dn11, locals.var_qbulk2_dn12,)
    }
};
        locals.var_qbulk2 = assign28940_e27023;
        locals.var_qbulk2_dn3 = assign28940_e27023_d_n3;
        locals.var_qbulk2_dn4 = assign28940_e27023_d_n4;
        locals.var_qbulk2_dn5 = assign28940_e27023_d_n5;
        locals.var_qbulk2_dn6 = assign28940_e27023_d_n6;
        locals.var_qbulk2_dn7 = assign28940_e27023_d_n7;
        locals.var_qbulk2_dn8 = assign28940_e27023_d_n8;
        locals.var_qbulk2_dn9 = assign28940_e27023_d_n9;
        locals.var_qbulk2_dn10 = assign28940_e27023_d_n10;
        locals.var_qbulk2_dn11 = assign28940_e27023_d_n11;
        locals.var_qbulk2_dn12 = assign28940_e27023_d_n12;
        locals.var_qbulk2_rv = 0.0;

        let (assign28950_e27037, assign28950_e27037_d_n3, assign28950_e27037_d_n4, assign28950_e27037_d_n5, assign28950_e27037_d_n6, assign28950_e27037_d_n7, assign28950_e27037_d_n8, assign28950_e27037_d_n9, assign28950_e27037_d_n10, assign28950_e27037_d_n11, assign28950_e27037_d_n12,) = {
    if ((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1412 == 0.0)) && (locals.var_guard1413 != 0.0)) {
        let assign28950_e27035: f64 = (locals.var_qbulk + locals.var_qbulk2);
        (assign28950_e27035, (locals.var_qbulk_dn3 + locals.var_qbulk2_dn3), (locals.var_qbulk_dn4 + locals.var_qbulk2_dn4), (locals.var_qbulk_dn5 + locals.var_qbulk2_dn5), (locals.var_qbulk_dn6 + locals.var_qbulk2_dn6), (locals.var_qbulk_dn7 + locals.var_qbulk2_dn7), (locals.var_qbulk_dn8 + locals.var_qbulk2_dn8), (locals.var_qbulk_dn9 + locals.var_qbulk2_dn9), (locals.var_qbulk_dn10 + locals.var_qbulk2_dn10), (locals.var_qbulk_dn11 + locals.var_qbulk2_dn11), (locals.var_qbulk_dn12 + locals.var_qbulk2_dn12),)
    } else {
        (locals.var_qbulk, locals.var_qbulk_dn3, locals.var_qbulk_dn4, locals.var_qbulk_dn5, locals.var_qbulk_dn6, locals.var_qbulk_dn7, locals.var_qbulk_dn8, locals.var_qbulk_dn9, locals.var_qbulk_dn10, locals.var_qbulk_dn11, locals.var_qbulk_dn12,)
    }
};
        locals.var_qbulk = assign28950_e27037;
        locals.var_qbulk_dn3 = assign28950_e27037_d_n3;
        locals.var_qbulk_dn4 = assign28950_e27037_d_n4;
        locals.var_qbulk_dn5 = assign28950_e27037_d_n5;
        locals.var_qbulk_dn6 = assign28950_e27037_d_n6;
        locals.var_qbulk_dn7 = assign28950_e27037_d_n7;
        locals.var_qbulk_dn8 = assign28950_e27037_d_n8;
        locals.var_qbulk_dn9 = assign28950_e27037_d_n9;
        locals.var_qbulk_dn10 = assign28950_e27037_d_n10;
        locals.var_qbulk_dn11 = assign28950_e27037_d_n11;
        locals.var_qbulk_dn12 = assign28950_e27037_d_n12;
        locals.var_qbulk_rv = 0.0;

        let assign28960_e27040: f64 = if p.p129 > 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1414 = assign28960_e27040;
        locals.var_guard1414_rv = 0.0;

        let (assign28970_e27066, assign28970_e27066_d_n3, assign28970_e27066_d_n4, assign28970_e27066_d_n5, assign28970_e27066_d_n6, assign28970_e27066_d_n7, assign28970_e27066_d_n8, assign28970_e27066_d_n9, assign28970_e27066_d_n10, assign28970_e27066_d_n11, assign28970_e27066_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1414 != 0.0)) {
        let assign28970_e27048: f64 = (-locals.var_coxwlcen);
        let assign28970_e27051: f64 = (locals.var_t1__blk809 / 2.0);
        let assign28970_e27054: f64 = (locals.var_t0__blk808 / 4.0);
        let assign28970_e27055: f64 = (assign28970_e27051 + assign28970_e27054);
        let assign28970_e27058: f64 = (0.5 * locals.var_t0__blk808);
        let assign28970_e27060: f64 = (assign28970_e27058 * locals.var_t0__blk808);
        let assign28970_e27062: f64 = (assign28970_e27060 / locals.var_t2__blk810);
        let assign28970_e27063: f64 = (assign28970_e27055 - assign28970_e27062);
        let assign28970_e27064: f64 = (assign28970_e27048 * assign28970_e27063);
        (assign28970_e27064, (((-locals.var_coxwlcen_dn3) * assign28970_e27063) + (assign28970_e27048 * (((locals.var_t1__blk809_dn3 / 2.0) + (locals.var_t0__blk808_dn3 / 4.0)) - ((((((0.5 * locals.var_t0__blk808_dn3) * locals.var_t0__blk808) + (assign28970_e27058 * locals.var_t0__blk808_dn3)) * locals.var_t2__blk810) - (assign28970_e27060 * locals.var_t2__blk810_dn3)) / (locals.var_t2__blk810 * locals.var_t2__blk810))))), (((-locals.var_coxwlcen_dn4) * assign28970_e27063) + (assign28970_e27048 * (((locals.var_t1__blk809_dn4 / 2.0) + (locals.var_t0__blk808_dn4 / 4.0)) - ((((((0.5 * locals.var_t0__blk808_dn4) * locals.var_t0__blk808) + (assign28970_e27058 * locals.var_t0__blk808_dn4)) * locals.var_t2__blk810) - (assign28970_e27060 * locals.var_t2__blk810_dn4)) / (locals.var_t2__blk810 * locals.var_t2__blk810))))), (((-locals.var_coxwlcen_dn5) * assign28970_e27063) + (assign28970_e27048 * (((locals.var_t1__blk809_dn5 / 2.0) + (locals.var_t0__blk808_dn5 / 4.0)) - ((((((0.5 * locals.var_t0__blk808_dn5) * locals.var_t0__blk808) + (assign28970_e27058 * locals.var_t0__blk808_dn5)) * locals.var_t2__blk810) - (assign28970_e27060 * locals.var_t2__blk810_dn5)) / (locals.var_t2__blk810 * locals.var_t2__blk810))))), (((-locals.var_coxwlcen_dn6) * assign28970_e27063) + (assign28970_e27048 * (((locals.var_t1__blk809_dn6 / 2.0) + (locals.var_t0__blk808_dn6 / 4.0)) - ((((((0.5 * locals.var_t0__blk808_dn6) * locals.var_t0__blk808) + (assign28970_e27058 * locals.var_t0__blk808_dn6)) * locals.var_t2__blk810) - (assign28970_e27060 * locals.var_t2__blk810_dn6)) / (locals.var_t2__blk810 * locals.var_t2__blk810))))), (((-locals.var_coxwlcen_dn7) * assign28970_e27063) + (assign28970_e27048 * (((locals.var_t1__blk809_dn7 / 2.0) + (locals.var_t0__blk808_dn7 / 4.0)) - ((((((0.5 * locals.var_t0__blk808_dn7) * locals.var_t0__blk808) + (assign28970_e27058 * locals.var_t0__blk808_dn7)) * locals.var_t2__blk810) - (assign28970_e27060 * locals.var_t2__blk810_dn7)) / (locals.var_t2__blk810 * locals.var_t2__blk810))))), (((-locals.var_coxwlcen_dn8) * assign28970_e27063) + (assign28970_e27048 * (((locals.var_t1__blk809_dn8 / 2.0) + (locals.var_t0__blk808_dn8 / 4.0)) - ((((((0.5 * locals.var_t0__blk808_dn8) * locals.var_t0__blk808) + (assign28970_e27058 * locals.var_t0__blk808_dn8)) * locals.var_t2__blk810) - (assign28970_e27060 * locals.var_t2__blk810_dn8)) / (locals.var_t2__blk810 * locals.var_t2__blk810))))), (((-locals.var_coxwlcen_dn9) * assign28970_e27063) + (assign28970_e27048 * (((locals.var_t1__blk809_dn9 / 2.0) + (locals.var_t0__blk808_dn9 / 4.0)) - ((((((0.5 * locals.var_t0__blk808_dn9) * locals.var_t0__blk808) + (assign28970_e27058 * locals.var_t0__blk808_dn9)) * locals.var_t2__blk810) - (assign28970_e27060 * locals.var_t2__blk810_dn9)) / (locals.var_t2__blk810 * locals.var_t2__blk810))))), (((-locals.var_coxwlcen_dn10) * assign28970_e27063) + (assign28970_e27048 * (((locals.var_t1__blk809_dn10 / 2.0) + (locals.var_t0__blk808_dn10 / 4.0)) - ((((((0.5 * locals.var_t0__blk808_dn10) * locals.var_t0__blk808) + (assign28970_e27058 * locals.var_t0__blk808_dn10)) * locals.var_t2__blk810) - (assign28970_e27060 * locals.var_t2__blk810_dn10)) / (locals.var_t2__blk810 * locals.var_t2__blk810))))), (((-locals.var_coxwlcen_dn11) * assign28970_e27063) + (assign28970_e27048 * (((locals.var_t1__blk809_dn11 / 2.0) + (locals.var_t0__blk808_dn11 / 4.0)) - ((((((0.5 * locals.var_t0__blk808_dn11) * locals.var_t0__blk808) + (assign28970_e27058 * locals.var_t0__blk808_dn11)) * locals.var_t2__blk810) - (assign28970_e27060 * locals.var_t2__blk810_dn11)) / (locals.var_t2__blk810 * locals.var_t2__blk810))))), (((-locals.var_coxwlcen_dn12) * assign28970_e27063) + (assign28970_e27048 * (((locals.var_t1__blk809_dn12 / 2.0) + (locals.var_t0__blk808_dn12 / 4.0)) - ((((((0.5 * locals.var_t0__blk808_dn12) * locals.var_t0__blk808) + (assign28970_e27058 * locals.var_t0__blk808_dn12)) * locals.var_t2__blk810) - (assign28970_e27060 * locals.var_t2__blk810_dn12)) / (locals.var_t2__blk810 * locals.var_t2__blk810))))),)
    } else {
        (locals.var_qsrc, locals.var_qsrc_dn3, locals.var_qsrc_dn4, locals.var_qsrc_dn5, locals.var_qsrc_dn6, locals.var_qsrc_dn7, locals.var_qsrc_dn8, locals.var_qsrc_dn9, locals.var_qsrc_dn10, locals.var_qsrc_dn11, locals.var_qsrc_dn12,)
    }
};
        locals.var_qsrc = assign28970_e27066;
        locals.var_qsrc_dn3 = assign28970_e27066_d_n3;
        locals.var_qsrc_dn4 = assign28970_e27066_d_n4;
        locals.var_qsrc_dn5 = assign28970_e27066_d_n5;
        locals.var_qsrc_dn6 = assign28970_e27066_d_n6;
        locals.var_qsrc_dn7 = assign28970_e27066_d_n7;
        locals.var_qsrc_dn8 = assign28970_e27066_d_n8;
        locals.var_qsrc_dn9 = assign28970_e27066_d_n9;
        locals.var_qsrc_dn10 = assign28970_e27066_d_n10;
        locals.var_qsrc_dn11 = assign28970_e27066_d_n11;
        locals.var_qsrc_dn12 = assign28970_e27066_d_n12;
        locals.var_qsrc_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_89(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign28980_e27077: f64 = if (((locals.var_b4soisoimod != 2.0) && (locals.var_b4soibodymod != 0.0)) && (p.p27 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1415 = assign28980_e27077;
        locals.var_guard1415_rv = 0.0;

        let (assign28990_e27107, assign28990_e27107_d_n3, assign28990_e27107_d_n4, assign28990_e27107_d_n5, assign28990_e27107_d_n6, assign28990_e27107_d_n7, assign28990_e27107_d_n8, assign28990_e27107_d_n9, assign28990_e27107_d_n10, assign28990_e27107_d_n11, assign28990_e27107_d_n12,) = {
    if ((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1414 != 0.0)) && (locals.var_guard1415 != 0.0)) {
        let assign28990_e27087: f64 = (-locals.var_coxwlcen2);
        let assign28990_e27090: f64 = (locals.var_vgsteff2 - locals.var_deltaphi2);
        let assign28990_e27092: f64 = (assign28990_e27090 / 2.0);
        let assign28990_e27095: f64 = (locals.var_t02 / 4.0);
        let assign28990_e27096: f64 = (assign28990_e27092 + assign28990_e27095);
        let assign28990_e27099: f64 = (0.5 * locals.var_t02);
        let assign28990_e27101: f64 = (assign28990_e27099 * locals.var_t02);
        let assign28990_e27103: f64 = (assign28990_e27101 / locals.var_t22);
        let assign28990_e27104: f64 = (assign28990_e27096 - assign28990_e27103);
        let assign28990_e27105: f64 = (assign28990_e27087 * assign28990_e27104);
        (assign28990_e27105, (((-locals.var_coxwlcen2_dn3) * assign28990_e27104) + (assign28990_e27087 * ((((locals.var_vgsteff2_dn3 - locals.var_deltaphi2_dn3) / 2.0) + (locals.var_t02_dn3 / 4.0)) - ((((((0.5 * locals.var_t02_dn3) * locals.var_t02) + (assign28990_e27099 * locals.var_t02_dn3)) * locals.var_t22) - (assign28990_e27101 * locals.var_t22_dn3)) / (locals.var_t22 * locals.var_t22))))), (((-locals.var_coxwlcen2_dn4) * assign28990_e27104) + (assign28990_e27087 * ((((locals.var_vgsteff2_dn4 - locals.var_deltaphi2_dn4) / 2.0) + (locals.var_t02_dn4 / 4.0)) - ((((((0.5 * locals.var_t02_dn4) * locals.var_t02) + (assign28990_e27099 * locals.var_t02_dn4)) * locals.var_t22) - (assign28990_e27101 * locals.var_t22_dn4)) / (locals.var_t22 * locals.var_t22))))), (((-locals.var_coxwlcen2_dn5) * assign28990_e27104) + (assign28990_e27087 * ((((locals.var_vgsteff2_dn5 - locals.var_deltaphi2_dn5) / 2.0) + (locals.var_t02_dn5 / 4.0)) - ((((((0.5 * locals.var_t02_dn5) * locals.var_t02) + (assign28990_e27099 * locals.var_t02_dn5)) * locals.var_t22) - (assign28990_e27101 * locals.var_t22_dn5)) / (locals.var_t22 * locals.var_t22))))), (((-locals.var_coxwlcen2_dn6) * assign28990_e27104) + (assign28990_e27087 * ((((locals.var_vgsteff2_dn6 - locals.var_deltaphi2_dn6) / 2.0) + (locals.var_t02_dn6 / 4.0)) - ((((((0.5 * locals.var_t02_dn6) * locals.var_t02) + (assign28990_e27099 * locals.var_t02_dn6)) * locals.var_t22) - (assign28990_e27101 * locals.var_t22_dn6)) / (locals.var_t22 * locals.var_t22))))), (((-locals.var_coxwlcen2_dn7) * assign28990_e27104) + (assign28990_e27087 * ((((locals.var_vgsteff2_dn7 - locals.var_deltaphi2_dn7) / 2.0) + (locals.var_t02_dn7 / 4.0)) - ((((((0.5 * locals.var_t02_dn7) * locals.var_t02) + (assign28990_e27099 * locals.var_t02_dn7)) * locals.var_t22) - (assign28990_e27101 * locals.var_t22_dn7)) / (locals.var_t22 * locals.var_t22))))), (((-locals.var_coxwlcen2_dn8) * assign28990_e27104) + (assign28990_e27087 * ((((locals.var_vgsteff2_dn8 - locals.var_deltaphi2_dn8) / 2.0) + (locals.var_t02_dn8 / 4.0)) - ((((((0.5 * locals.var_t02_dn8) * locals.var_t02) + (assign28990_e27099 * locals.var_t02_dn8)) * locals.var_t22) - (assign28990_e27101 * locals.var_t22_dn8)) / (locals.var_t22 * locals.var_t22))))), (((-locals.var_coxwlcen2_dn9) * assign28990_e27104) + (assign28990_e27087 * ((((locals.var_vgsteff2_dn9 - locals.var_deltaphi2_dn9) / 2.0) + (locals.var_t02_dn9 / 4.0)) - ((((((0.5 * locals.var_t02_dn9) * locals.var_t02) + (assign28990_e27099 * locals.var_t02_dn9)) * locals.var_t22) - (assign28990_e27101 * locals.var_t22_dn9)) / (locals.var_t22 * locals.var_t22))))), (((-locals.var_coxwlcen2_dn10) * assign28990_e27104) + (assign28990_e27087 * ((((locals.var_vgsteff2_dn10 - locals.var_deltaphi2_dn10) / 2.0) + (locals.var_t02_dn10 / 4.0)) - ((((((0.5 * locals.var_t02_dn10) * locals.var_t02) + (assign28990_e27099 * locals.var_t02_dn10)) * locals.var_t22) - (assign28990_e27101 * locals.var_t22_dn10)) / (locals.var_t22 * locals.var_t22))))), (((-locals.var_coxwlcen2_dn11) * assign28990_e27104) + (assign28990_e27087 * ((((locals.var_vgsteff2_dn11 - locals.var_deltaphi2_dn11) / 2.0) + (locals.var_t02_dn11 / 4.0)) - ((((((0.5 * locals.var_t02_dn11) * locals.var_t02) + (assign28990_e27099 * locals.var_t02_dn11)) * locals.var_t22) - (assign28990_e27101 * locals.var_t22_dn11)) / (locals.var_t22 * locals.var_t22))))), (((-locals.var_coxwlcen2_dn12) * assign28990_e27104) + (assign28990_e27087 * ((((locals.var_vgsteff2_dn12 - locals.var_deltaphi2_dn12) / 2.0) + (locals.var_t02_dn12 / 4.0)) - ((((((0.5 * locals.var_t02_dn12) * locals.var_t02) + (assign28990_e27099 * locals.var_t02_dn12)) * locals.var_t22) - (assign28990_e27101 * locals.var_t22_dn12)) / (locals.var_t22 * locals.var_t22))))),)
    } else {
        (locals.var_qsrc2, locals.var_qsrc2_dn3, locals.var_qsrc2_dn4, locals.var_qsrc2_dn5, locals.var_qsrc2_dn6, locals.var_qsrc2_dn7, locals.var_qsrc2_dn8, locals.var_qsrc2_dn9, locals.var_qsrc2_dn10, locals.var_qsrc2_dn11, locals.var_qsrc2_dn12,)
    }
};
        locals.var_qsrc2 = assign28990_e27107;
        locals.var_qsrc2_dn3 = assign28990_e27107_d_n3;
        locals.var_qsrc2_dn4 = assign28990_e27107_d_n4;
        locals.var_qsrc2_dn5 = assign28990_e27107_d_n5;
        locals.var_qsrc2_dn6 = assign28990_e27107_d_n6;
        locals.var_qsrc2_dn7 = assign28990_e27107_d_n7;
        locals.var_qsrc2_dn8 = assign28990_e27107_d_n8;
        locals.var_qsrc2_dn9 = assign28990_e27107_d_n9;
        locals.var_qsrc2_dn10 = assign28990_e27107_d_n10;
        locals.var_qsrc2_dn11 = assign28990_e27107_d_n11;
        locals.var_qsrc2_dn12 = assign28990_e27107_d_n12;
        locals.var_qsrc2_rv = 0.0;

        let (assign29000_e27120, assign29000_e27120_d_n3, assign29000_e27120_d_n4, assign29000_e27120_d_n5, assign29000_e27120_d_n6, assign29000_e27120_d_n7, assign29000_e27120_d_n8, assign29000_e27120_d_n9, assign29000_e27120_d_n10, assign29000_e27120_d_n11, assign29000_e27120_d_n12,) = {
    if ((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1414 != 0.0)) && (locals.var_guard1415 != 0.0)) {
        let assign29000_e27118: f64 = (locals.var_qsrc + locals.var_qsrc2);
        (assign29000_e27118, (locals.var_qsrc_dn3 + locals.var_qsrc2_dn3), (locals.var_qsrc_dn4 + locals.var_qsrc2_dn4), (locals.var_qsrc_dn5 + locals.var_qsrc2_dn5), (locals.var_qsrc_dn6 + locals.var_qsrc2_dn6), (locals.var_qsrc_dn7 + locals.var_qsrc2_dn7), (locals.var_qsrc_dn8 + locals.var_qsrc2_dn8), (locals.var_qsrc_dn9 + locals.var_qsrc2_dn9), (locals.var_qsrc_dn10 + locals.var_qsrc2_dn10), (locals.var_qsrc_dn11 + locals.var_qsrc2_dn11), (locals.var_qsrc_dn12 + locals.var_qsrc2_dn12),)
    } else {
        (locals.var_qsrc, locals.var_qsrc_dn3, locals.var_qsrc_dn4, locals.var_qsrc_dn5, locals.var_qsrc_dn6, locals.var_qsrc_dn7, locals.var_qsrc_dn8, locals.var_qsrc_dn9, locals.var_qsrc_dn10, locals.var_qsrc_dn11, locals.var_qsrc_dn12,)
    }
};
        locals.var_qsrc = assign29000_e27120;
        locals.var_qsrc_dn3 = assign29000_e27120_d_n3;
        locals.var_qsrc_dn4 = assign29000_e27120_d_n4;
        locals.var_qsrc_dn5 = assign29000_e27120_d_n5;
        locals.var_qsrc_dn6 = assign29000_e27120_d_n6;
        locals.var_qsrc_dn7 = assign29000_e27120_d_n7;
        locals.var_qsrc_dn8 = assign29000_e27120_d_n8;
        locals.var_qsrc_dn9 = assign29000_e27120_d_n9;
        locals.var_qsrc_dn10 = assign29000_e27120_d_n10;
        locals.var_qsrc_dn11 = assign29000_e27120_d_n11;
        locals.var_qsrc_dn12 = assign29000_e27120_d_n12;
        locals.var_qsrc_rv = 0.0;

        let assign29010_e27123: f64 = if p.p129 < 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1416 = assign29010_e27123;
        locals.var_guard1416_rv = 0.0;

        let (assign29020_e27137, assign29020_e27137_d_n3, assign29020_e27137_d_n4, assign29020_e27137_d_n5, assign29020_e27137_d_n6, assign29020_e27137_d_n7, assign29020_e27137_d_n8, assign29020_e27137_d_n9, assign29020_e27137_d_n10, assign29020_e27137_d_n11, assign29020_e27137_d_n12,) = {
    if ((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1414 == 0.0)) && (locals.var_guard1416 != 0.0)) {
        let assign29020_e27135: f64 = (locals.var_t2__blk810 / 12.0);
        (assign29020_e27135, (locals.var_t2__blk810_dn3 / 12.0), (locals.var_t2__blk810_dn4 / 12.0), (locals.var_t2__blk810_dn5 / 12.0), (locals.var_t2__blk810_dn6 / 12.0), (locals.var_t2__blk810_dn7 / 12.0), (locals.var_t2__blk810_dn8 / 12.0), (locals.var_t2__blk810_dn9 / 12.0), (locals.var_t2__blk810_dn10 / 12.0), (locals.var_t2__blk810_dn11 / 12.0), (locals.var_t2__blk810_dn12 / 12.0),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign29020_e27137;
        locals.var_t2__blk810_dn3 = assign29020_e27137_d_n3;
        locals.var_t2__blk810_dn4 = assign29020_e27137_d_n4;
        locals.var_t2__blk810_dn5 = assign29020_e27137_d_n5;
        locals.var_t2__blk810_dn6 = assign29020_e27137_d_n6;
        locals.var_t2__blk810_dn7 = assign29020_e27137_d_n7;
        locals.var_t2__blk810_dn8 = assign29020_e27137_d_n8;
        locals.var_t2__blk810_dn9 = assign29020_e27137_d_n9;
        locals.var_t2__blk810_dn10 = assign29020_e27137_d_n10;
        locals.var_t2__blk810_dn11 = assign29020_e27137_d_n11;
        locals.var_t2__blk810_dn12 = assign29020_e27137_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign29030_e27155, assign29030_e27155_d_n3, assign29030_e27155_d_n4, assign29030_e27155_d_n5, assign29030_e27155_d_n6, assign29030_e27155_d_n7, assign29030_e27155_d_n8, assign29030_e27155_d_n9, assign29030_e27155_d_n10, assign29030_e27155_d_n11, assign29030_e27155_d_n12,) = {
    if ((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1414 == 0.0)) && (locals.var_guard1416 != 0.0)) {
        let assign29030_e27149: f64 = (0.5 * locals.var_coxwlcen);
        let assign29030_e27152: f64 = (locals.var_t2__blk810 * locals.var_t2__blk810);
        let assign29030_e27153: f64 = (assign29030_e27149 / assign29030_e27152);
        (assign29030_e27153, ((((0.5 * locals.var_coxwlcen_dn3) * assign29030_e27152) - (assign29030_e27149 * ((locals.var_t2__blk810_dn3 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn3)))) / (assign29030_e27152 * assign29030_e27152)), ((((0.5 * locals.var_coxwlcen_dn4) * assign29030_e27152) - (assign29030_e27149 * ((locals.var_t2__blk810_dn4 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn4)))) / (assign29030_e27152 * assign29030_e27152)), ((((0.5 * locals.var_coxwlcen_dn5) * assign29030_e27152) - (assign29030_e27149 * ((locals.var_t2__blk810_dn5 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn5)))) / (assign29030_e27152 * assign29030_e27152)), ((((0.5 * locals.var_coxwlcen_dn6) * assign29030_e27152) - (assign29030_e27149 * ((locals.var_t2__blk810_dn6 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn6)))) / (assign29030_e27152 * assign29030_e27152)), ((((0.5 * locals.var_coxwlcen_dn7) * assign29030_e27152) - (assign29030_e27149 * ((locals.var_t2__blk810_dn7 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn7)))) / (assign29030_e27152 * assign29030_e27152)), ((((0.5 * locals.var_coxwlcen_dn8) * assign29030_e27152) - (assign29030_e27149 * ((locals.var_t2__blk810_dn8 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn8)))) / (assign29030_e27152 * assign29030_e27152)), ((((0.5 * locals.var_coxwlcen_dn9) * assign29030_e27152) - (assign29030_e27149 * ((locals.var_t2__blk810_dn9 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn9)))) / (assign29030_e27152 * assign29030_e27152)), ((((0.5 * locals.var_coxwlcen_dn10) * assign29030_e27152) - (assign29030_e27149 * ((locals.var_t2__blk810_dn10 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn10)))) / (assign29030_e27152 * assign29030_e27152)), ((((0.5 * locals.var_coxwlcen_dn11) * assign29030_e27152) - (assign29030_e27149 * ((locals.var_t2__blk810_dn11 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn11)))) / (assign29030_e27152 * assign29030_e27152)), ((((0.5 * locals.var_coxwlcen_dn12) * assign29030_e27152) - (assign29030_e27149 * ((locals.var_t2__blk810_dn12 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn12)))) / (assign29030_e27152 * assign29030_e27152)),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign29030_e27155;
        locals.var_t3__blk811_dn3 = assign29030_e27155_d_n3;
        locals.var_t3__blk811_dn4 = assign29030_e27155_d_n4;
        locals.var_t3__blk811_dn5 = assign29030_e27155_d_n5;
        locals.var_t3__blk811_dn6 = assign29030_e27155_d_n6;
        locals.var_t3__blk811_dn7 = assign29030_e27155_d_n7;
        locals.var_t3__blk811_dn8 = assign29030_e27155_d_n8;
        locals.var_t3__blk811_dn9 = assign29030_e27155_d_n9;
        locals.var_t3__blk811_dn10 = assign29030_e27155_d_n10;
        locals.var_t3__blk811_dn11 = assign29030_e27155_d_n11;
        locals.var_t3__blk811_dn12 = assign29030_e27155_d_n12;
        locals.var_t3__blk811_rv = 0.0;

        let (assign29040_e27195, assign29040_e27195_d_n3, assign29040_e27195_d_n4, assign29040_e27195_d_n5, assign29040_e27195_d_n6, assign29040_e27195_d_n7, assign29040_e27195_d_n8, assign29040_e27195_d_n9, assign29040_e27195_d_n10, assign29040_e27195_d_n11, assign29040_e27195_d_n12,) = {
    if ((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1414 == 0.0)) && (locals.var_guard1416 != 0.0)) {
        let assign29040_e27168: f64 = (2.0 * locals.var_t0__blk808);
        let assign29040_e27170: f64 = (assign29040_e27168 * locals.var_t0__blk808);
        let assign29040_e27172: f64 = (assign29040_e27170 / 3.0);
        let assign29040_e27177: f64 = (4.0 * locals.var_t0__blk808);
        let assign29040_e27179: f64 = (assign29040_e27177 / 3.0);
        let assign29040_e27180: f64 = (locals.var_t1__blk809 - assign29040_e27179);
        let assign29040_e27181: f64 = (locals.var_t1__blk809 * assign29040_e27180);
        let assign29040_e27182: f64 = (assign29040_e27172 + assign29040_e27181);
        let assign29040_e27183: f64 = (locals.var_t1__blk809 * assign29040_e27182);
        let assign29040_e27186: f64 = (2.0 * locals.var_t0__blk808);
        let assign29040_e27188: f64 = (assign29040_e27186 * locals.var_t0__blk808);
        let assign29040_e27190: f64 = (assign29040_e27188 * locals.var_t0__blk808);
        let assign29040_e27192: f64 = (assign29040_e27190 / 15.0);
        let assign29040_e27193: f64 = (assign29040_e27183 - assign29040_e27192);
        (assign29040_e27193, (((locals.var_t1__blk809_dn3 * assign29040_e27182) + (locals.var_t1__blk809 * (((((2.0 * locals.var_t0__blk808_dn3) * locals.var_t0__blk808) + (assign29040_e27168 * locals.var_t0__blk808_dn3)) / 3.0) + ((locals.var_t1__blk809_dn3 * assign29040_e27180) + (locals.var_t1__blk809 * (locals.var_t1__blk809_dn3 - ((4.0 * locals.var_t0__blk808_dn3) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk808_dn3) * locals.var_t0__blk808) + (assign29040_e27186 * locals.var_t0__blk808_dn3)) * locals.var_t0__blk808) + (assign29040_e27188 * locals.var_t0__blk808_dn3)) / 15.0)), (((locals.var_t1__blk809_dn4 * assign29040_e27182) + (locals.var_t1__blk809 * (((((2.0 * locals.var_t0__blk808_dn4) * locals.var_t0__blk808) + (assign29040_e27168 * locals.var_t0__blk808_dn4)) / 3.0) + ((locals.var_t1__blk809_dn4 * assign29040_e27180) + (locals.var_t1__blk809 * (locals.var_t1__blk809_dn4 - ((4.0 * locals.var_t0__blk808_dn4) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk808_dn4) * locals.var_t0__blk808) + (assign29040_e27186 * locals.var_t0__blk808_dn4)) * locals.var_t0__blk808) + (assign29040_e27188 * locals.var_t0__blk808_dn4)) / 15.0)), (((locals.var_t1__blk809_dn5 * assign29040_e27182) + (locals.var_t1__blk809 * (((((2.0 * locals.var_t0__blk808_dn5) * locals.var_t0__blk808) + (assign29040_e27168 * locals.var_t0__blk808_dn5)) / 3.0) + ((locals.var_t1__blk809_dn5 * assign29040_e27180) + (locals.var_t1__blk809 * (locals.var_t1__blk809_dn5 - ((4.0 * locals.var_t0__blk808_dn5) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk808_dn5) * locals.var_t0__blk808) + (assign29040_e27186 * locals.var_t0__blk808_dn5)) * locals.var_t0__blk808) + (assign29040_e27188 * locals.var_t0__blk808_dn5)) / 15.0)), (((locals.var_t1__blk809_dn6 * assign29040_e27182) + (locals.var_t1__blk809 * (((((2.0 * locals.var_t0__blk808_dn6) * locals.var_t0__blk808) + (assign29040_e27168 * locals.var_t0__blk808_dn6)) / 3.0) + ((locals.var_t1__blk809_dn6 * assign29040_e27180) + (locals.var_t1__blk809 * (locals.var_t1__blk809_dn6 - ((4.0 * locals.var_t0__blk808_dn6) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk808_dn6) * locals.var_t0__blk808) + (assign29040_e27186 * locals.var_t0__blk808_dn6)) * locals.var_t0__blk808) + (assign29040_e27188 * locals.var_t0__blk808_dn6)) / 15.0)), (((locals.var_t1__blk809_dn7 * assign29040_e27182) + (locals.var_t1__blk809 * (((((2.0 * locals.var_t0__blk808_dn7) * locals.var_t0__blk808) + (assign29040_e27168 * locals.var_t0__blk808_dn7)) / 3.0) + ((locals.var_t1__blk809_dn7 * assign29040_e27180) + (locals.var_t1__blk809 * (locals.var_t1__blk809_dn7 - ((4.0 * locals.var_t0__blk808_dn7) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk808_dn7) * locals.var_t0__blk808) + (assign29040_e27186 * locals.var_t0__blk808_dn7)) * locals.var_t0__blk808) + (assign29040_e27188 * locals.var_t0__blk808_dn7)) / 15.0)), (((locals.var_t1__blk809_dn8 * assign29040_e27182) + (locals.var_t1__blk809 * (((((2.0 * locals.var_t0__blk808_dn8) * locals.var_t0__blk808) + (assign29040_e27168 * locals.var_t0__blk808_dn8)) / 3.0) + ((locals.var_t1__blk809_dn8 * assign29040_e27180) + (locals.var_t1__blk809 * (locals.var_t1__blk809_dn8 - ((4.0 * locals.var_t0__blk808_dn8) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk808_dn8) * locals.var_t0__blk808) + (assign29040_e27186 * locals.var_t0__blk808_dn8)) * locals.var_t0__blk808) + (assign29040_e27188 * locals.var_t0__blk808_dn8)) / 15.0)), (((locals.var_t1__blk809_dn9 * assign29040_e27182) + (locals.var_t1__blk809 * (((((2.0 * locals.var_t0__blk808_dn9) * locals.var_t0__blk808) + (assign29040_e27168 * locals.var_t0__blk808_dn9)) / 3.0) + ((locals.var_t1__blk809_dn9 * assign29040_e27180) + (locals.var_t1__blk809 * (locals.var_t1__blk809_dn9 - ((4.0 * locals.var_t0__blk808_dn9) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk808_dn9) * locals.var_t0__blk808) + (assign29040_e27186 * locals.var_t0__blk808_dn9)) * locals.var_t0__blk808) + (assign29040_e27188 * locals.var_t0__blk808_dn9)) / 15.0)), (((locals.var_t1__blk809_dn10 * assign29040_e27182) + (locals.var_t1__blk809 * (((((2.0 * locals.var_t0__blk808_dn10) * locals.var_t0__blk808) + (assign29040_e27168 * locals.var_t0__blk808_dn10)) / 3.0) + ((locals.var_t1__blk809_dn10 * assign29040_e27180) + (locals.var_t1__blk809 * (locals.var_t1__blk809_dn10 - ((4.0 * locals.var_t0__blk808_dn10) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk808_dn10) * locals.var_t0__blk808) + (assign29040_e27186 * locals.var_t0__blk808_dn10)) * locals.var_t0__blk808) + (assign29040_e27188 * locals.var_t0__blk808_dn10)) / 15.0)), (((locals.var_t1__blk809_dn11 * assign29040_e27182) + (locals.var_t1__blk809 * (((((2.0 * locals.var_t0__blk808_dn11) * locals.var_t0__blk808) + (assign29040_e27168 * locals.var_t0__blk808_dn11)) / 3.0) + ((locals.var_t1__blk809_dn11 * assign29040_e27180) + (locals.var_t1__blk809 * (locals.var_t1__blk809_dn11 - ((4.0 * locals.var_t0__blk808_dn11) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk808_dn11) * locals.var_t0__blk808) + (assign29040_e27186 * locals.var_t0__blk808_dn11)) * locals.var_t0__blk808) + (assign29040_e27188 * locals.var_t0__blk808_dn11)) / 15.0)), (((locals.var_t1__blk809_dn12 * assign29040_e27182) + (locals.var_t1__blk809 * (((((2.0 * locals.var_t0__blk808_dn12) * locals.var_t0__blk808) + (assign29040_e27168 * locals.var_t0__blk808_dn12)) / 3.0) + ((locals.var_t1__blk809_dn12 * assign29040_e27180) + (locals.var_t1__blk809 * (locals.var_t1__blk809_dn12 - ((4.0 * locals.var_t0__blk808_dn12) / 3.0))))))) - ((((((2.0 * locals.var_t0__blk808_dn12) * locals.var_t0__blk808) + (assign29040_e27186 * locals.var_t0__blk808_dn12)) * locals.var_t0__blk808) + (assign29040_e27188 * locals.var_t0__blk808_dn12)) / 15.0)),)
    } else {
        (locals.var_t4__blk812, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    }
};
        locals.var_t4__blk812 = assign29040_e27195;
        locals.var_t4__blk812_dn3 = assign29040_e27195_d_n3;
        locals.var_t4__blk812_dn4 = assign29040_e27195_d_n4;
        locals.var_t4__blk812_dn5 = assign29040_e27195_d_n5;
        locals.var_t4__blk812_dn6 = assign29040_e27195_d_n6;
        locals.var_t4__blk812_dn7 = assign29040_e27195_d_n7;
        locals.var_t4__blk812_dn8 = assign29040_e27195_d_n8;
        locals.var_t4__blk812_dn9 = assign29040_e27195_d_n9;
        locals.var_t4__blk812_dn10 = assign29040_e27195_d_n10;
        locals.var_t4__blk812_dn11 = assign29040_e27195_d_n11;
        locals.var_t4__blk812_dn12 = assign29040_e27195_d_n12;
        locals.var_t4__blk812_rv = 0.0;

        let (assign29050_e27210, assign29050_e27210_d_n3, assign29050_e27210_d_n4, assign29050_e27210_d_n5, assign29050_e27210_d_n6, assign29050_e27210_d_n7, assign29050_e27210_d_n8, assign29050_e27210_d_n9, assign29050_e27210_d_n10, assign29050_e27210_d_n11, assign29050_e27210_d_n12,) = {
    if ((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1414 == 0.0)) && (locals.var_guard1416 != 0.0)) {
        let assign29050_e27206: f64 = (-locals.var_t3__blk811);
        let assign29050_e27208: f64 = (assign29050_e27206 * locals.var_t4__blk812);
        (assign29050_e27208, (((-locals.var_t3__blk811_dn3) * locals.var_t4__blk812) + (assign29050_e27206 * locals.var_t4__blk812_dn3)), (((-locals.var_t3__blk811_dn4) * locals.var_t4__blk812) + (assign29050_e27206 * locals.var_t4__blk812_dn4)), (((-locals.var_t3__blk811_dn5) * locals.var_t4__blk812) + (assign29050_e27206 * locals.var_t4__blk812_dn5)), (((-locals.var_t3__blk811_dn6) * locals.var_t4__blk812) + (assign29050_e27206 * locals.var_t4__blk812_dn6)), (((-locals.var_t3__blk811_dn7) * locals.var_t4__blk812) + (assign29050_e27206 * locals.var_t4__blk812_dn7)), (((-locals.var_t3__blk811_dn8) * locals.var_t4__blk812) + (assign29050_e27206 * locals.var_t4__blk812_dn8)), (((-locals.var_t3__blk811_dn9) * locals.var_t4__blk812) + (assign29050_e27206 * locals.var_t4__blk812_dn9)), (((-locals.var_t3__blk811_dn10) * locals.var_t4__blk812) + (assign29050_e27206 * locals.var_t4__blk812_dn10)), (((-locals.var_t3__blk811_dn11) * locals.var_t4__blk812) + (assign29050_e27206 * locals.var_t4__blk812_dn11)), (((-locals.var_t3__blk811_dn12) * locals.var_t4__blk812) + (assign29050_e27206 * locals.var_t4__blk812_dn12)),)
    } else {
        (locals.var_qsrc, locals.var_qsrc_dn3, locals.var_qsrc_dn4, locals.var_qsrc_dn5, locals.var_qsrc_dn6, locals.var_qsrc_dn7, locals.var_qsrc_dn8, locals.var_qsrc_dn9, locals.var_qsrc_dn10, locals.var_qsrc_dn11, locals.var_qsrc_dn12,)
    }
};
        locals.var_qsrc = assign29050_e27210;
        locals.var_qsrc_dn3 = assign29050_e27210_d_n3;
        locals.var_qsrc_dn4 = assign29050_e27210_d_n4;
        locals.var_qsrc_dn5 = assign29050_e27210_d_n5;
        locals.var_qsrc_dn6 = assign29050_e27210_d_n6;
        locals.var_qsrc_dn7 = assign29050_e27210_d_n7;
        locals.var_qsrc_dn8 = assign29050_e27210_d_n8;
        locals.var_qsrc_dn9 = assign29050_e27210_d_n9;
        locals.var_qsrc_dn10 = assign29050_e27210_d_n10;
        locals.var_qsrc_dn11 = assign29050_e27210_d_n11;
        locals.var_qsrc_dn12 = assign29050_e27210_d_n12;
        locals.var_qsrc_rv = 0.0;

        let assign29060_e27221: f64 = if (((locals.var_b4soisoimod != 2.0) && (locals.var_b4soibodymod != 0.0)) && (p.p27 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1417 = assign29060_e27221;
        locals.var_guard1417_rv = 0.0;

        let (assign29070_e27237, assign29070_e27237_d_n3, assign29070_e27237_d_n4, assign29070_e27237_d_n5, assign29070_e27237_d_n6, assign29070_e27237_d_n7, assign29070_e27237_d_n8, assign29070_e27237_d_n9, assign29070_e27237_d_n10, assign29070_e27237_d_n11, assign29070_e27237_d_n12,) = {
    if (((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1414 == 0.0)) && (locals.var_guard1416 != 0.0)) && (locals.var_guard1417 != 0.0)) {
        let assign29070_e27235: f64 = (locals.var_t22 / 12.0);
        (assign29070_e27235, (locals.var_t22_dn3 / 12.0), (locals.var_t22_dn4 / 12.0), (locals.var_t22_dn5 / 12.0), (locals.var_t22_dn6 / 12.0), (locals.var_t22_dn7 / 12.0), (locals.var_t22_dn8 / 12.0), (locals.var_t22_dn9 / 12.0), (locals.var_t22_dn10 / 12.0), (locals.var_t22_dn11 / 12.0), (locals.var_t22_dn12 / 12.0),)
    } else {
        (locals.var_t22, locals.var_t22_dn3, locals.var_t22_dn4, locals.var_t22_dn5, locals.var_t22_dn6, locals.var_t22_dn7, locals.var_t22_dn8, locals.var_t22_dn9, locals.var_t22_dn10, locals.var_t22_dn11, locals.var_t22_dn12,)
    }
};
        locals.var_t22 = assign29070_e27237;
        locals.var_t22_dn3 = assign29070_e27237_d_n3;
        locals.var_t22_dn4 = assign29070_e27237_d_n4;
        locals.var_t22_dn5 = assign29070_e27237_d_n5;
        locals.var_t22_dn6 = assign29070_e27237_d_n6;
        locals.var_t22_dn7 = assign29070_e27237_d_n7;
        locals.var_t22_dn8 = assign29070_e27237_d_n8;
        locals.var_t22_dn9 = assign29070_e27237_d_n9;
        locals.var_t22_dn10 = assign29070_e27237_d_n10;
        locals.var_t22_dn11 = assign29070_e27237_d_n11;
        locals.var_t22_dn12 = assign29070_e27237_d_n12;
        locals.var_t22_rv = 0.0;

        let (assign29080_e27257, assign29080_e27257_d_n3, assign29080_e27257_d_n4, assign29080_e27257_d_n5, assign29080_e27257_d_n6, assign29080_e27257_d_n7, assign29080_e27257_d_n8, assign29080_e27257_d_n9, assign29080_e27257_d_n10, assign29080_e27257_d_n11, assign29080_e27257_d_n12,) = {
    if (((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1414 == 0.0)) && (locals.var_guard1416 != 0.0)) && (locals.var_guard1417 != 0.0)) {
        let assign29080_e27251: f64 = (0.5 * locals.var_coxwlcen2);
        let assign29080_e27254: f64 = (locals.var_t22 * locals.var_t22);
        let assign29080_e27255: f64 = (assign29080_e27251 / assign29080_e27254);
        (assign29080_e27255, ((((0.5 * locals.var_coxwlcen2_dn3) * assign29080_e27254) - (assign29080_e27251 * ((locals.var_t22_dn3 * locals.var_t22) + (locals.var_t22 * locals.var_t22_dn3)))) / (assign29080_e27254 * assign29080_e27254)), ((((0.5 * locals.var_coxwlcen2_dn4) * assign29080_e27254) - (assign29080_e27251 * ((locals.var_t22_dn4 * locals.var_t22) + (locals.var_t22 * locals.var_t22_dn4)))) / (assign29080_e27254 * assign29080_e27254)), ((((0.5 * locals.var_coxwlcen2_dn5) * assign29080_e27254) - (assign29080_e27251 * ((locals.var_t22_dn5 * locals.var_t22) + (locals.var_t22 * locals.var_t22_dn5)))) / (assign29080_e27254 * assign29080_e27254)), ((((0.5 * locals.var_coxwlcen2_dn6) * assign29080_e27254) - (assign29080_e27251 * ((locals.var_t22_dn6 * locals.var_t22) + (locals.var_t22 * locals.var_t22_dn6)))) / (assign29080_e27254 * assign29080_e27254)), ((((0.5 * locals.var_coxwlcen2_dn7) * assign29080_e27254) - (assign29080_e27251 * ((locals.var_t22_dn7 * locals.var_t22) + (locals.var_t22 * locals.var_t22_dn7)))) / (assign29080_e27254 * assign29080_e27254)), ((((0.5 * locals.var_coxwlcen2_dn8) * assign29080_e27254) - (assign29080_e27251 * ((locals.var_t22_dn8 * locals.var_t22) + (locals.var_t22 * locals.var_t22_dn8)))) / (assign29080_e27254 * assign29080_e27254)), ((((0.5 * locals.var_coxwlcen2_dn9) * assign29080_e27254) - (assign29080_e27251 * ((locals.var_t22_dn9 * locals.var_t22) + (locals.var_t22 * locals.var_t22_dn9)))) / (assign29080_e27254 * assign29080_e27254)), ((((0.5 * locals.var_coxwlcen2_dn10) * assign29080_e27254) - (assign29080_e27251 * ((locals.var_t22_dn10 * locals.var_t22) + (locals.var_t22 * locals.var_t22_dn10)))) / (assign29080_e27254 * assign29080_e27254)), ((((0.5 * locals.var_coxwlcen2_dn11) * assign29080_e27254) - (assign29080_e27251 * ((locals.var_t22_dn11 * locals.var_t22) + (locals.var_t22 * locals.var_t22_dn11)))) / (assign29080_e27254 * assign29080_e27254)), ((((0.5 * locals.var_coxwlcen2_dn12) * assign29080_e27254) - (assign29080_e27251 * ((locals.var_t22_dn12 * locals.var_t22) + (locals.var_t22 * locals.var_t22_dn12)))) / (assign29080_e27254 * assign29080_e27254)),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign29080_e27257;
        locals.var_t3__blk811_dn3 = assign29080_e27257_d_n3;
        locals.var_t3__blk811_dn4 = assign29080_e27257_d_n4;
        locals.var_t3__blk811_dn5 = assign29080_e27257_d_n5;
        locals.var_t3__blk811_dn6 = assign29080_e27257_d_n6;
        locals.var_t3__blk811_dn7 = assign29080_e27257_d_n7;
        locals.var_t3__blk811_dn8 = assign29080_e27257_d_n8;
        locals.var_t3__blk811_dn9 = assign29080_e27257_d_n9;
        locals.var_t3__blk811_dn10 = assign29080_e27257_d_n10;
        locals.var_t3__blk811_dn11 = assign29080_e27257_d_n11;
        locals.var_t3__blk811_dn12 = assign29080_e27257_d_n12;
        locals.var_t3__blk811_rv = 0.0;

        let (assign29090_e27299, assign29090_e27299_d_n3, assign29090_e27299_d_n4, assign29090_e27299_d_n5, assign29090_e27299_d_n6, assign29090_e27299_d_n7, assign29090_e27299_d_n8, assign29090_e27299_d_n9, assign29090_e27299_d_n10, assign29090_e27299_d_n11, assign29090_e27299_d_n12,) = {
    if (((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1414 == 0.0)) && (locals.var_guard1416 != 0.0)) && (locals.var_guard1417 != 0.0)) {
        let assign29090_e27272: f64 = (2.0 * locals.var_t02);
        let assign29090_e27274: f64 = (assign29090_e27272 * locals.var_t02);
        let assign29090_e27276: f64 = (assign29090_e27274 / 3.0);
        let assign29090_e27281: f64 = (4.0 * locals.var_t02);
        let assign29090_e27283: f64 = (assign29090_e27281 / 3.0);
        let assign29090_e27284: f64 = (locals.var_t12 - assign29090_e27283);
        let assign29090_e27285: f64 = (locals.var_t12 * assign29090_e27284);
        let assign29090_e27286: f64 = (assign29090_e27276 + assign29090_e27285);
        let assign29090_e27287: f64 = (locals.var_t12 * assign29090_e27286);
        let assign29090_e27290: f64 = (2.0 * locals.var_t02);
        let assign29090_e27292: f64 = (assign29090_e27290 * locals.var_t02);
        let assign29090_e27294: f64 = (assign29090_e27292 * locals.var_t02);
        let assign29090_e27296: f64 = (assign29090_e27294 / 15.0);
        let assign29090_e27297: f64 = (assign29090_e27287 - assign29090_e27296);
        (assign29090_e27297, (((locals.var_t12_dn3 * assign29090_e27286) + (locals.var_t12 * (((((2.0 * locals.var_t02_dn3) * locals.var_t02) + (assign29090_e27272 * locals.var_t02_dn3)) / 3.0) + ((locals.var_t12_dn3 * assign29090_e27284) + (locals.var_t12 * (locals.var_t12_dn3 - ((4.0 * locals.var_t02_dn3) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn3) * locals.var_t02) + (assign29090_e27290 * locals.var_t02_dn3)) * locals.var_t02) + (assign29090_e27292 * locals.var_t02_dn3)) / 15.0)), (((locals.var_t12_dn4 * assign29090_e27286) + (locals.var_t12 * (((((2.0 * locals.var_t02_dn4) * locals.var_t02) + (assign29090_e27272 * locals.var_t02_dn4)) / 3.0) + ((locals.var_t12_dn4 * assign29090_e27284) + (locals.var_t12 * (locals.var_t12_dn4 - ((4.0 * locals.var_t02_dn4) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn4) * locals.var_t02) + (assign29090_e27290 * locals.var_t02_dn4)) * locals.var_t02) + (assign29090_e27292 * locals.var_t02_dn4)) / 15.0)), (((locals.var_t12_dn5 * assign29090_e27286) + (locals.var_t12 * (((((2.0 * locals.var_t02_dn5) * locals.var_t02) + (assign29090_e27272 * locals.var_t02_dn5)) / 3.0) + ((locals.var_t12_dn5 * assign29090_e27284) + (locals.var_t12 * (locals.var_t12_dn5 - ((4.0 * locals.var_t02_dn5) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn5) * locals.var_t02) + (assign29090_e27290 * locals.var_t02_dn5)) * locals.var_t02) + (assign29090_e27292 * locals.var_t02_dn5)) / 15.0)), (((locals.var_t12_dn6 * assign29090_e27286) + (locals.var_t12 * (((((2.0 * locals.var_t02_dn6) * locals.var_t02) + (assign29090_e27272 * locals.var_t02_dn6)) / 3.0) + ((locals.var_t12_dn6 * assign29090_e27284) + (locals.var_t12 * (locals.var_t12_dn6 - ((4.0 * locals.var_t02_dn6) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn6) * locals.var_t02) + (assign29090_e27290 * locals.var_t02_dn6)) * locals.var_t02) + (assign29090_e27292 * locals.var_t02_dn6)) / 15.0)), (((locals.var_t12_dn7 * assign29090_e27286) + (locals.var_t12 * (((((2.0 * locals.var_t02_dn7) * locals.var_t02) + (assign29090_e27272 * locals.var_t02_dn7)) / 3.0) + ((locals.var_t12_dn7 * assign29090_e27284) + (locals.var_t12 * (locals.var_t12_dn7 - ((4.0 * locals.var_t02_dn7) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn7) * locals.var_t02) + (assign29090_e27290 * locals.var_t02_dn7)) * locals.var_t02) + (assign29090_e27292 * locals.var_t02_dn7)) / 15.0)), (((locals.var_t12_dn8 * assign29090_e27286) + (locals.var_t12 * (((((2.0 * locals.var_t02_dn8) * locals.var_t02) + (assign29090_e27272 * locals.var_t02_dn8)) / 3.0) + ((locals.var_t12_dn8 * assign29090_e27284) + (locals.var_t12 * (locals.var_t12_dn8 - ((4.0 * locals.var_t02_dn8) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn8) * locals.var_t02) + (assign29090_e27290 * locals.var_t02_dn8)) * locals.var_t02) + (assign29090_e27292 * locals.var_t02_dn8)) / 15.0)), (((locals.var_t12_dn9 * assign29090_e27286) + (locals.var_t12 * (((((2.0 * locals.var_t02_dn9) * locals.var_t02) + (assign29090_e27272 * locals.var_t02_dn9)) / 3.0) + ((locals.var_t12_dn9 * assign29090_e27284) + (locals.var_t12 * (locals.var_t12_dn9 - ((4.0 * locals.var_t02_dn9) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn9) * locals.var_t02) + (assign29090_e27290 * locals.var_t02_dn9)) * locals.var_t02) + (assign29090_e27292 * locals.var_t02_dn9)) / 15.0)), (((locals.var_t12_dn10 * assign29090_e27286) + (locals.var_t12 * (((((2.0 * locals.var_t02_dn10) * locals.var_t02) + (assign29090_e27272 * locals.var_t02_dn10)) / 3.0) + ((locals.var_t12_dn10 * assign29090_e27284) + (locals.var_t12 * (locals.var_t12_dn10 - ((4.0 * locals.var_t02_dn10) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn10) * locals.var_t02) + (assign29090_e27290 * locals.var_t02_dn10)) * locals.var_t02) + (assign29090_e27292 * locals.var_t02_dn10)) / 15.0)), (((locals.var_t12_dn11 * assign29090_e27286) + (locals.var_t12 * (((((2.0 * locals.var_t02_dn11) * locals.var_t02) + (assign29090_e27272 * locals.var_t02_dn11)) / 3.0) + ((locals.var_t12_dn11 * assign29090_e27284) + (locals.var_t12 * (locals.var_t12_dn11 - ((4.0 * locals.var_t02_dn11) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn11) * locals.var_t02) + (assign29090_e27290 * locals.var_t02_dn11)) * locals.var_t02) + (assign29090_e27292 * locals.var_t02_dn11)) / 15.0)), (((locals.var_t12_dn12 * assign29090_e27286) + (locals.var_t12 * (((((2.0 * locals.var_t02_dn12) * locals.var_t02) + (assign29090_e27272 * locals.var_t02_dn12)) / 3.0) + ((locals.var_t12_dn12 * assign29090_e27284) + (locals.var_t12 * (locals.var_t12_dn12 - ((4.0 * locals.var_t02_dn12) / 3.0))))))) - ((((((2.0 * locals.var_t02_dn12) * locals.var_t02) + (assign29090_e27290 * locals.var_t02_dn12)) * locals.var_t02) + (assign29090_e27292 * locals.var_t02_dn12)) / 15.0)),)
    } else {
        (locals.var_t4__blk812, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    }
};
        locals.var_t4__blk812 = assign29090_e27299;
        locals.var_t4__blk812_dn3 = assign29090_e27299_d_n3;
        locals.var_t4__blk812_dn4 = assign29090_e27299_d_n4;
        locals.var_t4__blk812_dn5 = assign29090_e27299_d_n5;
        locals.var_t4__blk812_dn6 = assign29090_e27299_d_n6;
        locals.var_t4__blk812_dn7 = assign29090_e27299_d_n7;
        locals.var_t4__blk812_dn8 = assign29090_e27299_d_n8;
        locals.var_t4__blk812_dn9 = assign29090_e27299_d_n9;
        locals.var_t4__blk812_dn10 = assign29090_e27299_d_n10;
        locals.var_t4__blk812_dn11 = assign29090_e27299_d_n11;
        locals.var_t4__blk812_dn12 = assign29090_e27299_d_n12;
        locals.var_t4__blk812_rv = 0.0;

        let (assign29100_e27316, assign29100_e27316_d_n3, assign29100_e27316_d_n4, assign29100_e27316_d_n5, assign29100_e27316_d_n6, assign29100_e27316_d_n7, assign29100_e27316_d_n8, assign29100_e27316_d_n9, assign29100_e27316_d_n10, assign29100_e27316_d_n11, assign29100_e27316_d_n12,) = {
    if (((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1414 == 0.0)) && (locals.var_guard1416 != 0.0)) && (locals.var_guard1417 != 0.0)) {
        let assign29100_e27312: f64 = (-locals.var_t3__blk811);
        let assign29100_e27314: f64 = (assign29100_e27312 * locals.var_t4__blk812);
        (assign29100_e27314, (((-locals.var_t3__blk811_dn3) * locals.var_t4__blk812) + (assign29100_e27312 * locals.var_t4__blk812_dn3)), (((-locals.var_t3__blk811_dn4) * locals.var_t4__blk812) + (assign29100_e27312 * locals.var_t4__blk812_dn4)), (((-locals.var_t3__blk811_dn5) * locals.var_t4__blk812) + (assign29100_e27312 * locals.var_t4__blk812_dn5)), (((-locals.var_t3__blk811_dn6) * locals.var_t4__blk812) + (assign29100_e27312 * locals.var_t4__blk812_dn6)), (((-locals.var_t3__blk811_dn7) * locals.var_t4__blk812) + (assign29100_e27312 * locals.var_t4__blk812_dn7)), (((-locals.var_t3__blk811_dn8) * locals.var_t4__blk812) + (assign29100_e27312 * locals.var_t4__blk812_dn8)), (((-locals.var_t3__blk811_dn9) * locals.var_t4__blk812) + (assign29100_e27312 * locals.var_t4__blk812_dn9)), (((-locals.var_t3__blk811_dn10) * locals.var_t4__blk812) + (assign29100_e27312 * locals.var_t4__blk812_dn10)), (((-locals.var_t3__blk811_dn11) * locals.var_t4__blk812) + (assign29100_e27312 * locals.var_t4__blk812_dn11)), (((-locals.var_t3__blk811_dn12) * locals.var_t4__blk812) + (assign29100_e27312 * locals.var_t4__blk812_dn12)),)
    } else {
        (locals.var_qsrc2, locals.var_qsrc2_dn3, locals.var_qsrc2_dn4, locals.var_qsrc2_dn5, locals.var_qsrc2_dn6, locals.var_qsrc2_dn7, locals.var_qsrc2_dn8, locals.var_qsrc2_dn9, locals.var_qsrc2_dn10, locals.var_qsrc2_dn11, locals.var_qsrc2_dn12,)
    }
};
        locals.var_qsrc2 = assign29100_e27316;
        locals.var_qsrc2_dn3 = assign29100_e27316_d_n3;
        locals.var_qsrc2_dn4 = assign29100_e27316_d_n4;
        locals.var_qsrc2_dn5 = assign29100_e27316_d_n5;
        locals.var_qsrc2_dn6 = assign29100_e27316_d_n6;
        locals.var_qsrc2_dn7 = assign29100_e27316_d_n7;
        locals.var_qsrc2_dn8 = assign29100_e27316_d_n8;
        locals.var_qsrc2_dn9 = assign29100_e27316_d_n9;
        locals.var_qsrc2_dn10 = assign29100_e27316_d_n10;
        locals.var_qsrc2_dn11 = assign29100_e27316_d_n11;
        locals.var_qsrc2_dn12 = assign29100_e27316_d_n12;
        locals.var_qsrc2_rv = 0.0;

        let (assign29110_e27332, assign29110_e27332_d_n3, assign29110_e27332_d_n4, assign29110_e27332_d_n5, assign29110_e27332_d_n6, assign29110_e27332_d_n7, assign29110_e27332_d_n8, assign29110_e27332_d_n9, assign29110_e27332_d_n10, assign29110_e27332_d_n11, assign29110_e27332_d_n12,) = {
    if (((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1414 == 0.0)) && (locals.var_guard1416 != 0.0)) && (locals.var_guard1417 != 0.0)) {
        let assign29110_e27330: f64 = (locals.var_qsrc + locals.var_qsrc2);
        (assign29110_e27330, (locals.var_qsrc_dn3 + locals.var_qsrc2_dn3), (locals.var_qsrc_dn4 + locals.var_qsrc2_dn4), (locals.var_qsrc_dn5 + locals.var_qsrc2_dn5), (locals.var_qsrc_dn6 + locals.var_qsrc2_dn6), (locals.var_qsrc_dn7 + locals.var_qsrc2_dn7), (locals.var_qsrc_dn8 + locals.var_qsrc2_dn8), (locals.var_qsrc_dn9 + locals.var_qsrc2_dn9), (locals.var_qsrc_dn10 + locals.var_qsrc2_dn10), (locals.var_qsrc_dn11 + locals.var_qsrc2_dn11), (locals.var_qsrc_dn12 + locals.var_qsrc2_dn12),)
    } else {
        (locals.var_qsrc, locals.var_qsrc_dn3, locals.var_qsrc_dn4, locals.var_qsrc_dn5, locals.var_qsrc_dn6, locals.var_qsrc_dn7, locals.var_qsrc_dn8, locals.var_qsrc_dn9, locals.var_qsrc_dn10, locals.var_qsrc_dn11, locals.var_qsrc_dn12,)
    }
};
        locals.var_qsrc = assign29110_e27332;
        locals.var_qsrc_dn3 = assign29110_e27332_d_n3;
        locals.var_qsrc_dn4 = assign29110_e27332_d_n4;
        locals.var_qsrc_dn5 = assign29110_e27332_d_n5;
        locals.var_qsrc_dn6 = assign29110_e27332_d_n6;
        locals.var_qsrc_dn7 = assign29110_e27332_d_n7;
        locals.var_qsrc_dn8 = assign29110_e27332_d_n8;
        locals.var_qsrc_dn9 = assign29110_e27332_d_n9;
        locals.var_qsrc_dn10 = assign29110_e27332_d_n10;
        locals.var_qsrc_dn11 = assign29110_e27332_d_n11;
        locals.var_qsrc_dn12 = assign29110_e27332_d_n12;
        locals.var_qsrc_rv = 0.0;

        let (assign29120_e27348, assign29120_e27348_d_n3, assign29120_e27348_d_n4, assign29120_e27348_d_n5, assign29120_e27348_d_n6, assign29120_e27348_d_n7, assign29120_e27348_d_n8, assign29120_e27348_d_n9, assign29120_e27348_d_n10, assign29120_e27348_d_n11, assign29120_e27348_d_n12,) = {
    if ((((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1414 == 0.0)) && (locals.var_guard1416 == 0.0)) {
        let assign29120_e27344: f64 = (-0.5);
        let assign29120_e27346: f64 = (assign29120_e27344 * locals.var_qgate);
        (assign29120_e27346, (assign29120_e27344 * locals.var_qgate_dn3), (assign29120_e27344 * locals.var_qgate_dn4), (assign29120_e27344 * locals.var_qgate_dn5), (assign29120_e27344 * locals.var_qgate_dn6), (assign29120_e27344 * locals.var_qgate_dn7), (assign29120_e27344 * locals.var_qgate_dn8), (assign29120_e27344 * locals.var_qgate_dn9), (assign29120_e27344 * locals.var_qgate_dn10), (assign29120_e27344 * locals.var_qgate_dn11), (assign29120_e27344 * locals.var_qgate_dn12),)
    } else {
        (locals.var_qsrc, locals.var_qsrc_dn3, locals.var_qsrc_dn4, locals.var_qsrc_dn5, locals.var_qsrc_dn6, locals.var_qsrc_dn7, locals.var_qsrc_dn8, locals.var_qsrc_dn9, locals.var_qsrc_dn10, locals.var_qsrc_dn11, locals.var_qsrc_dn12,)
    }
};
        locals.var_qsrc = assign29120_e27348;
        locals.var_qsrc_dn3 = assign29120_e27348_d_n3;
        locals.var_qsrc_dn4 = assign29120_e27348_d_n4;
        locals.var_qsrc_dn5 = assign29120_e27348_d_n5;
        locals.var_qsrc_dn6 = assign29120_e27348_d_n6;
        locals.var_qsrc_dn7 = assign29120_e27348_d_n7;
        locals.var_qsrc_dn8 = assign29120_e27348_d_n8;
        locals.var_qsrc_dn9 = assign29120_e27348_d_n9;
        locals.var_qsrc_dn10 = assign29120_e27348_d_n10;
        locals.var_qsrc_dn11 = assign29120_e27348_d_n11;
        locals.var_qsrc_dn12 = assign29120_e27348_d_n12;
        locals.var_qsrc_rv = 0.0;

        let assign29130_e27351: f64 = if locals.var_b4soisoimod == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1418 = assign29130_e27351;
        locals.var_guard1418_rv = 0.0;

        let (assign29140_e27360, assign29140_e27360_d_n3, assign29140_e27360_d_n4, assign29140_e27360_d_n5, assign29140_e27360_d_n6, assign29140_e27360_d_n7, assign29140_e27360_d_n8, assign29140_e27360_d_n9, assign29140_e27360_d_n10, assign29140_e27360_d_n11, assign29140_e27360_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1418 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qe1, locals.var_qe1_dn3, locals.var_qe1_dn4, locals.var_qe1_dn5, locals.var_qe1_dn6, locals.var_qe1_dn7, locals.var_qe1_dn8, locals.var_qe1_dn9, locals.var_qe1_dn10, locals.var_qe1_dn11, locals.var_qe1_dn12,)
    }
};
        locals.var_qe1 = assign29140_e27360;
        locals.var_qe1_dn3 = assign29140_e27360_d_n3;
        locals.var_qe1_dn4 = assign29140_e27360_d_n4;
        locals.var_qe1_dn5 = assign29140_e27360_d_n5;
        locals.var_qe1_dn6 = assign29140_e27360_d_n6;
        locals.var_qe1_dn7 = assign29140_e27360_d_n7;
        locals.var_qe1_dn8 = assign29140_e27360_d_n8;
        locals.var_qe1_dn9 = assign29140_e27360_d_n9;
        locals.var_qe1_dn10 = assign29140_e27360_d_n10;
        locals.var_qe1_dn11 = assign29140_e27360_d_n11;
        locals.var_qe1_dn12 = assign29140_e27360_d_n12;
        locals.var_qe1_rv = 0.0;

        let (assign29150_e27384, assign29150_e27384_d_n3, assign29150_e27384_d_n4, assign29150_e27384_d_n5, assign29150_e27384_d_n6, assign29150_e27384_d_n7, assign29150_e27384_d_n8, assign29150_e27384_d_n9, assign29150_e27384_d_n10, assign29150_e27384_d_n11, assign29150_e27384_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1418 == 0.0)) {
        let assign29150_e27370: f64 = (locals.var_pparam_b4soikb1 * p.p361);
        let assign29150_e27372: f64 = (assign29150_e27370 * locals.var_cbox);
        let assign29150_e27375: f64 = (locals.var_pparam_b4soiweffcv / p.p23);
        let assign29150_e27377: f64 = (assign29150_e27375 * p.p3);
        let assign29150_e27379: f64 = (assign29150_e27377 * locals.var_pparam_b4soileffcvbg);
        let assign29150_e27381: f64 = (assign29150_e27379 + p.p29);
        let assign29150_e27382: f64 = (assign29150_e27372 * assign29150_e27381);
        (assign29150_e27382, ((((locals.var_pparam_b4soikb1_dn3 * p.p361) * locals.var_cbox) * assign29150_e27381) + (assign29150_e27372 * ((((locals.var_pparam_b4soiweffcv_dn3 / p.p23) * p.p3) * locals.var_pparam_b4soileffcvbg) + (assign29150_e27377 * locals.var_pparam_b4soileffcvbg_dn3)))), ((((locals.var_pparam_b4soikb1_dn4 * p.p361) * locals.var_cbox) * assign29150_e27381) + (assign29150_e27372 * ((((locals.var_pparam_b4soiweffcv_dn4 / p.p23) * p.p3) * locals.var_pparam_b4soileffcvbg) + (assign29150_e27377 * locals.var_pparam_b4soileffcvbg_dn4)))), ((((locals.var_pparam_b4soikb1_dn5 * p.p361) * locals.var_cbox) * assign29150_e27381) + (assign29150_e27372 * ((((locals.var_pparam_b4soiweffcv_dn5 / p.p23) * p.p3) * locals.var_pparam_b4soileffcvbg) + (assign29150_e27377 * locals.var_pparam_b4soileffcvbg_dn5)))), ((((locals.var_pparam_b4soikb1_dn6 * p.p361) * locals.var_cbox) * assign29150_e27381) + (assign29150_e27372 * ((((locals.var_pparam_b4soiweffcv_dn6 / p.p23) * p.p3) * locals.var_pparam_b4soileffcvbg) + (assign29150_e27377 * locals.var_pparam_b4soileffcvbg_dn6)))), ((((locals.var_pparam_b4soikb1_dn7 * p.p361) * locals.var_cbox) * assign29150_e27381) + (assign29150_e27372 * ((((locals.var_pparam_b4soiweffcv_dn7 / p.p23) * p.p3) * locals.var_pparam_b4soileffcvbg) + (assign29150_e27377 * locals.var_pparam_b4soileffcvbg_dn7)))), ((((locals.var_pparam_b4soikb1_dn8 * p.p361) * locals.var_cbox) * assign29150_e27381) + (assign29150_e27372 * ((((locals.var_pparam_b4soiweffcv_dn8 / p.p23) * p.p3) * locals.var_pparam_b4soileffcvbg) + (assign29150_e27377 * locals.var_pparam_b4soileffcvbg_dn8)))), ((((locals.var_pparam_b4soikb1_dn9 * p.p361) * locals.var_cbox) * assign29150_e27381) + (assign29150_e27372 * ((((locals.var_pparam_b4soiweffcv_dn9 / p.p23) * p.p3) * locals.var_pparam_b4soileffcvbg) + (assign29150_e27377 * locals.var_pparam_b4soileffcvbg_dn9)))), ((((locals.var_pparam_b4soikb1_dn10 * p.p361) * locals.var_cbox) * assign29150_e27381) + (assign29150_e27372 * ((((locals.var_pparam_b4soiweffcv_dn10 / p.p23) * p.p3) * locals.var_pparam_b4soileffcvbg) + (assign29150_e27377 * locals.var_pparam_b4soileffcvbg_dn10)))), ((((locals.var_pparam_b4soikb1_dn11 * p.p361) * locals.var_cbox) * assign29150_e27381) + (assign29150_e27372 * ((((locals.var_pparam_b4soiweffcv_dn11 / p.p23) * p.p3) * locals.var_pparam_b4soileffcvbg) + (assign29150_e27377 * locals.var_pparam_b4soileffcvbg_dn11)))), ((((locals.var_pparam_b4soikb1_dn12 * p.p361) * locals.var_cbox) * assign29150_e27381) + (assign29150_e27372 * ((((locals.var_pparam_b4soiweffcv_dn12 / p.p23) * p.p3) * locals.var_pparam_b4soileffcvbg) + (assign29150_e27377 * locals.var_pparam_b4soileffcvbg_dn12)))),)
    } else {
        (locals.var_cboxwl, locals.var_cboxwl_dn3, locals.var_cboxwl_dn4, locals.var_cboxwl_dn5, locals.var_cboxwl_dn6, locals.var_cboxwl_dn7, locals.var_cboxwl_dn8, locals.var_cboxwl_dn9, locals.var_cboxwl_dn10, locals.var_cboxwl_dn11, locals.var_cboxwl_dn12,)
    }
};
        locals.var_cboxwl = assign29150_e27384;
        locals.var_cboxwl_dn3 = assign29150_e27384_d_n3;
        locals.var_cboxwl_dn4 = assign29150_e27384_d_n4;
        locals.var_cboxwl_dn5 = assign29150_e27384_d_n5;
        locals.var_cboxwl_dn6 = assign29150_e27384_d_n6;
        locals.var_cboxwl_dn7 = assign29150_e27384_d_n7;
        locals.var_cboxwl_dn8 = assign29150_e27384_d_n8;
        locals.var_cboxwl_dn9 = assign29150_e27384_d_n9;
        locals.var_cboxwl_dn10 = assign29150_e27384_d_n10;
        locals.var_cboxwl_dn11 = assign29150_e27384_d_n11;
        locals.var_cboxwl_dn12 = assign29150_e27384_d_n12;
        locals.var_cboxwl_rv = 0.0;

        let (assign29160_e27398, assign29160_e27398_d_n3, assign29160_e27398_d_n4, assign29160_e27398_d_n5, assign29160_e27398_d_n6, assign29160_e27398_d_n7, assign29160_e27398_d_n8, assign29160_e27398_d_n9, assign29160_e27398_d_n10, assign29160_e27398_d_n11, assign29160_e27398_d_n12,) = {
    if (((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) && (locals.var_guard1418 == 0.0)) {
        let assign29160_e27395: f64 = (locals.var_vesfb - locals.var_vbs_1);
        let assign29160_e27396: f64 = (locals.var_cboxwl * assign29160_e27395);
        (assign29160_e27396, ((locals.var_cboxwl_dn3 * assign29160_e27395) + (locals.var_cboxwl * (locals.var_vesfb_dn3 - locals.var_vbs_1_dn3))), ((locals.var_cboxwl_dn4 * assign29160_e27395) + (locals.var_cboxwl * (locals.var_vesfb_dn4 - locals.var_vbs_1_dn4))), ((locals.var_cboxwl_dn5 * assign29160_e27395) + (locals.var_cboxwl * (locals.var_vesfb_dn5 - locals.var_vbs_1_dn5))), ((locals.var_cboxwl_dn6 * assign29160_e27395) + (locals.var_cboxwl * (locals.var_vesfb_dn6 - locals.var_vbs_1_dn6))), ((locals.var_cboxwl_dn7 * assign29160_e27395) + (locals.var_cboxwl * (locals.var_vesfb_dn7 - locals.var_vbs_1_dn7))), ((locals.var_cboxwl_dn8 * assign29160_e27395) + (locals.var_cboxwl * (locals.var_vesfb_dn8 - locals.var_vbs_1_dn8))), ((locals.var_cboxwl_dn9 * assign29160_e27395) + (locals.var_cboxwl * (locals.var_vesfb_dn9 - locals.var_vbs_1_dn9))), ((locals.var_cboxwl_dn10 * assign29160_e27395) + (locals.var_cboxwl * (locals.var_vesfb_dn10 - locals.var_vbs_1_dn10))), ((locals.var_cboxwl_dn11 * assign29160_e27395) + (locals.var_cboxwl * (locals.var_vesfb_dn11 - locals.var_vbs_1_dn11))), ((locals.var_cboxwl_dn12 * assign29160_e27395) + (locals.var_cboxwl * (locals.var_vesfb_dn12 - locals.var_vbs_1_dn12))),)
    } else {
        (locals.var_qe1, locals.var_qe1_dn3, locals.var_qe1_dn4, locals.var_qe1_dn5, locals.var_qe1_dn6, locals.var_qe1_dn7, locals.var_qe1_dn8, locals.var_qe1_dn9, locals.var_qe1_dn10, locals.var_qe1_dn11, locals.var_qe1_dn12,)
    }
};
        locals.var_qe1 = assign29160_e27398;
        locals.var_qe1_dn3 = assign29160_e27398_d_n3;
        locals.var_qe1_dn4 = assign29160_e27398_d_n4;
        locals.var_qe1_dn5 = assign29160_e27398_d_n5;
        locals.var_qe1_dn6 = assign29160_e27398_d_n6;
        locals.var_qe1_dn7 = assign29160_e27398_d_n7;
        locals.var_qe1_dn8 = assign29160_e27398_d_n8;
        locals.var_qe1_dn9 = assign29160_e27398_d_n9;
        locals.var_qe1_dn10 = assign29160_e27398_d_n10;
        locals.var_qe1_dn11 = assign29160_e27398_d_n11;
        locals.var_qe1_dn12 = assign29160_e27398_d_n12;
        locals.var_qe1_rv = 0.0;

        let (assign29170_e27411, assign29170_e27411_d_n3, assign29170_e27411_d_n4, assign29170_e27411_d_n5, assign29170_e27411_d_n6, assign29170_e27411_d_n7, assign29170_e27411_d_n8, assign29170_e27411_d_n9, assign29170_e27411_d_n10, assign29170_e27411_d_n11, assign29170_e27411_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) {
        let assign29170_e27405: f64 = (locals.var_qgate + locals.var_qac0);
        let assign29170_e27407: f64 = (assign29170_e27405 + locals.var_qsub0);
        let assign29170_e27409: f64 = (assign29170_e27407 - locals.var_qbulk);
        (assign29170_e27409, (((locals.var_qgate_dn3 + locals.var_qac0_dn3) + locals.var_qsub0_dn3) - locals.var_qbulk_dn3), (((locals.var_qgate_dn4 + locals.var_qac0_dn4) + locals.var_qsub0_dn4) - locals.var_qbulk_dn4), (((locals.var_qgate_dn5 + locals.var_qac0_dn5) + locals.var_qsub0_dn5) - locals.var_qbulk_dn5), (((locals.var_qgate_dn6 + locals.var_qac0_dn6) + locals.var_qsub0_dn6) - locals.var_qbulk_dn6), (((locals.var_qgate_dn7 + locals.var_qac0_dn7) + locals.var_qsub0_dn7) - locals.var_qbulk_dn7), (((locals.var_qgate_dn8 + locals.var_qac0_dn8) + locals.var_qsub0_dn8) - locals.var_qbulk_dn8), (((locals.var_qgate_dn9 + locals.var_qac0_dn9) + locals.var_qsub0_dn9) - locals.var_qbulk_dn9), (((locals.var_qgate_dn10 + locals.var_qac0_dn10) + locals.var_qsub0_dn10) - locals.var_qbulk_dn10), (((locals.var_qgate_dn11 + locals.var_qac0_dn11) + locals.var_qsub0_dn11) - locals.var_qbulk_dn11), (((locals.var_qgate_dn12 + locals.var_qac0_dn12) + locals.var_qsub0_dn12) - locals.var_qbulk_dn12),)
    } else {
        (locals.var_qgate, locals.var_qgate_dn3, locals.var_qgate_dn4, locals.var_qgate_dn5, locals.var_qgate_dn6, locals.var_qgate_dn7, locals.var_qgate_dn8, locals.var_qgate_dn9, locals.var_qgate_dn10, locals.var_qgate_dn11, locals.var_qgate_dn12,)
    }
};
        locals.var_qgate = assign29170_e27411;
        locals.var_qgate_dn3 = assign29170_e27411_d_n3;
        locals.var_qgate_dn4 = assign29170_e27411_d_n4;
        locals.var_qgate_dn5 = assign29170_e27411_d_n5;
        locals.var_qgate_dn6 = assign29170_e27411_d_n6;
        locals.var_qgate_dn7 = assign29170_e27411_d_n7;
        locals.var_qgate_dn8 = assign29170_e27411_d_n8;
        locals.var_qgate_dn9 = assign29170_e27411_d_n9;
        locals.var_qgate_dn10 = assign29170_e27411_d_n10;
        locals.var_qgate_dn11 = assign29170_e27411_d_n11;
        locals.var_qgate_dn12 = assign29170_e27411_d_n12;
        locals.var_qgate_rv = 0.0;

        let (assign29180_e27424, assign29180_e27424_d_n3, assign29180_e27424_d_n4, assign29180_e27424_d_n5, assign29180_e27424_d_n6, assign29180_e27424_d_n7, assign29180_e27424_d_n8, assign29180_e27424_d_n9, assign29180_e27424_d_n10, assign29180_e27424_d_n11, assign29180_e27424_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) {
        let assign29180_e27418: f64 = (locals.var_qbulk - locals.var_qac0);
        let assign29180_e27420: f64 = (assign29180_e27418 - locals.var_qsub0);
        let assign29180_e27422: f64 = (assign29180_e27420 - locals.var_qe1);
        (assign29180_e27422, (((locals.var_qbulk_dn3 - locals.var_qac0_dn3) - locals.var_qsub0_dn3) - locals.var_qe1_dn3), (((locals.var_qbulk_dn4 - locals.var_qac0_dn4) - locals.var_qsub0_dn4) - locals.var_qe1_dn4), (((locals.var_qbulk_dn5 - locals.var_qac0_dn5) - locals.var_qsub0_dn5) - locals.var_qe1_dn5), (((locals.var_qbulk_dn6 - locals.var_qac0_dn6) - locals.var_qsub0_dn6) - locals.var_qe1_dn6), (((locals.var_qbulk_dn7 - locals.var_qac0_dn7) - locals.var_qsub0_dn7) - locals.var_qe1_dn7), (((locals.var_qbulk_dn8 - locals.var_qac0_dn8) - locals.var_qsub0_dn8) - locals.var_qe1_dn8), (((locals.var_qbulk_dn9 - locals.var_qac0_dn9) - locals.var_qsub0_dn9) - locals.var_qe1_dn9), (((locals.var_qbulk_dn10 - locals.var_qac0_dn10) - locals.var_qsub0_dn10) - locals.var_qe1_dn10), (((locals.var_qbulk_dn11 - locals.var_qac0_dn11) - locals.var_qsub0_dn11) - locals.var_qe1_dn11), (((locals.var_qbulk_dn12 - locals.var_qac0_dn12) - locals.var_qsub0_dn12) - locals.var_qe1_dn12),)
    } else {
        (locals.var_qbody, locals.var_qbody_dn3, locals.var_qbody_dn4, locals.var_qbody_dn5, locals.var_qbody_dn6, locals.var_qbody_dn7, locals.var_qbody_dn8, locals.var_qbody_dn9, locals.var_qbody_dn10, locals.var_qbody_dn11, locals.var_qbody_dn12,)
    }
};
        locals.var_qbody = assign29180_e27424;
        locals.var_qbody_dn3 = assign29180_e27424_d_n3;
        locals.var_qbody_dn4 = assign29180_e27424_d_n4;
        locals.var_qbody_dn5 = assign29180_e27424_d_n5;
        locals.var_qbody_dn6 = assign29180_e27424_d_n6;
        locals.var_qbody_dn7 = assign29180_e27424_d_n7;
        locals.var_qbody_dn8 = assign29180_e27424_d_n8;
        locals.var_qbody_dn9 = assign29180_e27424_d_n9;
        locals.var_qbody_dn10 = assign29180_e27424_d_n10;
        locals.var_qbody_dn11 = assign29180_e27424_d_n11;
        locals.var_qbody_dn12 = assign29180_e27424_d_n12;
        locals.var_qbody_rv = 0.0;

        let (assign29190_e27431, assign29190_e27431_d_n3, assign29190_e27431_d_n4, assign29190_e27431_d_n5, assign29190_e27431_d_n6, assign29190_e27431_d_n7, assign29190_e27431_d_n8, assign29190_e27431_d_n9, assign29190_e27431_d_n10, assign29190_e27431_d_n11, assign29190_e27431_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) {
        (locals.var_qe1, locals.var_qe1_dn3, locals.var_qe1_dn4, locals.var_qe1_dn5, locals.var_qe1_dn6, locals.var_qe1_dn7, locals.var_qe1_dn8, locals.var_qe1_dn9, locals.var_qe1_dn10, locals.var_qe1_dn11, locals.var_qe1_dn12,)
    } else {
        (locals.var_qsub, locals.var_qsub_dn3, locals.var_qsub_dn4, locals.var_qsub_dn5, locals.var_qsub_dn6, locals.var_qsub_dn7, locals.var_qsub_dn8, locals.var_qsub_dn9, locals.var_qsub_dn10, locals.var_qsub_dn11, locals.var_qsub_dn12,)
    }
};
        locals.var_qsub = assign29190_e27431;
        locals.var_qsub_dn3 = assign29190_e27431_d_n3;
        locals.var_qsub_dn4 = assign29190_e27431_d_n4;
        locals.var_qsub_dn5 = assign29190_e27431_d_n5;
        locals.var_qsub_dn6 = assign29190_e27431_d_n6;
        locals.var_qsub_dn7 = assign29190_e27431_d_n7;
        locals.var_qsub_dn8 = assign29190_e27431_d_n8;
        locals.var_qsub_dn9 = assign29190_e27431_d_n9;
        locals.var_qsub_dn10 = assign29190_e27431_d_n10;
        locals.var_qsub_dn11 = assign29190_e27431_d_n11;
        locals.var_qsub_dn12 = assign29190_e27431_d_n12;
        locals.var_qsub_rv = 0.0;

        let (assign29200_e27445, assign29200_e27445_d_n3, assign29200_e27445_d_n4, assign29200_e27445_d_n5, assign29200_e27445_d_n6, assign29200_e27445_d_n7, assign29200_e27445_d_n8, assign29200_e27445_d_n9, assign29200_e27445_d_n10, assign29200_e27445_d_n11, assign29200_e27445_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 != 0.0)) {
        let assign29200_e27438: f64 = (locals.var_qgate + locals.var_qbody);
        let assign29200_e27440: f64 = (assign29200_e27438 + locals.var_qsub);
        let assign29200_e27442: f64 = (assign29200_e27440 + locals.var_qsrc);
        let assign29200_e27443: f64 = (-assign29200_e27442);
        (assign29200_e27443, (-(((locals.var_qgate_dn3 + locals.var_qbody_dn3) + locals.var_qsub_dn3) + locals.var_qsrc_dn3)), (-(((locals.var_qgate_dn4 + locals.var_qbody_dn4) + locals.var_qsub_dn4) + locals.var_qsrc_dn4)), (-(((locals.var_qgate_dn5 + locals.var_qbody_dn5) + locals.var_qsub_dn5) + locals.var_qsrc_dn5)), (-(((locals.var_qgate_dn6 + locals.var_qbody_dn6) + locals.var_qsub_dn6) + locals.var_qsrc_dn6)), (-(((locals.var_qgate_dn7 + locals.var_qbody_dn7) + locals.var_qsub_dn7) + locals.var_qsrc_dn7)), (-(((locals.var_qgate_dn8 + locals.var_qbody_dn8) + locals.var_qsub_dn8) + locals.var_qsrc_dn8)), (-(((locals.var_qgate_dn9 + locals.var_qbody_dn9) + locals.var_qsub_dn9) + locals.var_qsrc_dn9)), (-(((locals.var_qgate_dn10 + locals.var_qbody_dn10) + locals.var_qsub_dn10) + locals.var_qsrc_dn10)), (-(((locals.var_qgate_dn11 + locals.var_qbody_dn11) + locals.var_qsub_dn11) + locals.var_qsrc_dn11)), (-(((locals.var_qgate_dn12 + locals.var_qbody_dn12) + locals.var_qsub_dn12) + locals.var_qsrc_dn12)),)
    } else {
        (locals.var_qdrn, locals.var_qdrn_dn3, locals.var_qdrn_dn4, locals.var_qdrn_dn5, locals.var_qdrn_dn6, locals.var_qdrn_dn7, locals.var_qdrn_dn8, locals.var_qdrn_dn9, locals.var_qdrn_dn10, locals.var_qdrn_dn11, locals.var_qdrn_dn12,)
    }
};
        locals.var_qdrn = assign29200_e27445;
        locals.var_qdrn_dn3 = assign29200_e27445_d_n3;
        locals.var_qdrn_dn4 = assign29200_e27445_d_n4;
        locals.var_qdrn_dn5 = assign29200_e27445_d_n5;
        locals.var_qdrn_dn6 = assign29200_e27445_d_n6;
        locals.var_qdrn_dn7 = assign29200_e27445_d_n7;
        locals.var_qdrn_dn8 = assign29200_e27445_d_n8;
        locals.var_qdrn_dn9 = assign29200_e27445_d_n9;
        locals.var_qdrn_dn10 = assign29200_e27445_d_n10;
        locals.var_qdrn_dn11 = assign29200_e27445_d_n11;
        locals.var_qdrn_dn12 = assign29200_e27445_d_n12;
        locals.var_qdrn_rv = 0.0;

        let (assign29220_e27461, assign29220_e27461_d_n3, assign29220_e27461_d_n4, assign29220_e27461_d_n5, assign29220_e27461_d_n6, assign29220_e27461_d_n7, assign29220_e27461_d_n8, assign29220_e27461_d_n9, assign29220_e27461_d_n10, assign29220_e27461_d_n11, assign29220_e27461_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qac0, locals.var_qac0_dn3, locals.var_qac0_dn4, locals.var_qac0_dn5, locals.var_qac0_dn6, locals.var_qac0_dn7, locals.var_qac0_dn8, locals.var_qac0_dn9, locals.var_qac0_dn10, locals.var_qac0_dn11, locals.var_qac0_dn12,)
    }
};
        locals.var_qac0 = assign29220_e27461;
        locals.var_qac0_dn3 = assign29220_e27461_d_n3;
        locals.var_qac0_dn4 = assign29220_e27461_d_n4;
        locals.var_qac0_dn5 = assign29220_e27461_d_n5;
        locals.var_qac0_dn6 = assign29220_e27461_d_n6;
        locals.var_qac0_dn7 = assign29220_e27461_d_n7;
        locals.var_qac0_dn8 = assign29220_e27461_d_n8;
        locals.var_qac0_dn9 = assign29220_e27461_d_n9;
        locals.var_qac0_dn10 = assign29220_e27461_d_n10;
        locals.var_qac0_dn11 = assign29220_e27461_d_n11;
        locals.var_qac0_dn12 = assign29220_e27461_d_n12;
        locals.var_qac0_rv = 0.0;

        let (assign29230_e27469, assign29230_e27469_d_n3, assign29230_e27469_d_n4, assign29230_e27469_d_n5, assign29230_e27469_d_n6, assign29230_e27469_d_n7, assign29230_e27469_d_n8, assign29230_e27469_d_n9, assign29230_e27469_d_n10, assign29230_e27469_d_n11, assign29230_e27469_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qsub0, locals.var_qsub0_dn3, locals.var_qsub0_dn4, locals.var_qsub0_dn5, locals.var_qsub0_dn6, locals.var_qsub0_dn7, locals.var_qsub0_dn8, locals.var_qsub0_dn9, locals.var_qsub0_dn10, locals.var_qsub0_dn11, locals.var_qsub0_dn12,)
    }
};
        locals.var_qsub0 = assign29230_e27469;
        locals.var_qsub0_dn3 = assign29230_e27469_d_n3;
        locals.var_qsub0_dn4 = assign29230_e27469_d_n4;
        locals.var_qsub0_dn5 = assign29230_e27469_d_n5;
        locals.var_qsub0_dn6 = assign29230_e27469_d_n6;
        locals.var_qsub0_dn7 = assign29230_e27469_d_n7;
        locals.var_qsub0_dn8 = assign29230_e27469_d_n8;
        locals.var_qsub0_dn9 = assign29230_e27469_d_n9;
        locals.var_qsub0_dn10 = assign29230_e27469_d_n10;
        locals.var_qsub0_dn11 = assign29230_e27469_d_n11;
        locals.var_qsub0_dn12 = assign29230_e27469_d_n12;
        locals.var_qsub0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_90(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign29240_e27477, assign29240_e27477_d_n3, assign29240_e27477_d_n4, assign29240_e27477_d_n5, assign29240_e27477_d_n6, assign29240_e27477_d_n7, assign29240_e27477_d_n8, assign29240_e27477_d_n9, assign29240_e27477_d_n10, assign29240_e27477_d_n11, assign29240_e27477_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qsub, locals.var_qsub_dn3, locals.var_qsub_dn4, locals.var_qsub_dn5, locals.var_qsub_dn6, locals.var_qsub_dn7, locals.var_qsub_dn8, locals.var_qsub_dn9, locals.var_qsub_dn10, locals.var_qsub_dn11, locals.var_qsub_dn12,)
    }
};
        locals.var_qsub = assign29240_e27477;
        locals.var_qsub_dn3 = assign29240_e27477_d_n3;
        locals.var_qsub_dn4 = assign29240_e27477_d_n4;
        locals.var_qsub_dn5 = assign29240_e27477_d_n5;
        locals.var_qsub_dn6 = assign29240_e27477_d_n6;
        locals.var_qsub_dn7 = assign29240_e27477_d_n7;
        locals.var_qsub_dn8 = assign29240_e27477_d_n8;
        locals.var_qsub_dn9 = assign29240_e27477_d_n9;
        locals.var_qsub_dn10 = assign29240_e27477_d_n10;
        locals.var_qsub_dn11 = assign29240_e27477_d_n11;
        locals.var_qsub_dn12 = assign29240_e27477_d_n12;
        locals.var_qsub_rv = 0.0;

        let (assign29250_e27485, assign29250_e27485_d_n3, assign29250_e27485_d_n4, assign29250_e27485_d_n5, assign29250_e27485_d_n6, assign29250_e27485_d_n7, assign29250_e27485_d_n8, assign29250_e27485_d_n9, assign29250_e27485_d_n10, assign29250_e27485_d_n11, assign29250_e27485_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbody, locals.var_qbody_dn3, locals.var_qbody_dn4, locals.var_qbody_dn5, locals.var_qbody_dn6, locals.var_qbody_dn7, locals.var_qbody_dn8, locals.var_qbody_dn9, locals.var_qbody_dn10, locals.var_qbody_dn11, locals.var_qbody_dn12,)
    }
};
        locals.var_qbody = assign29250_e27485;
        locals.var_qbody_dn3 = assign29250_e27485_d_n3;
        locals.var_qbody_dn4 = assign29250_e27485_d_n4;
        locals.var_qbody_dn5 = assign29250_e27485_d_n5;
        locals.var_qbody_dn6 = assign29250_e27485_d_n6;
        locals.var_qbody_dn7 = assign29250_e27485_d_n7;
        locals.var_qbody_dn8 = assign29250_e27485_d_n8;
        locals.var_qbody_dn9 = assign29250_e27485_d_n9;
        locals.var_qbody_dn10 = assign29250_e27485_d_n10;
        locals.var_qbody_dn11 = assign29250_e27485_d_n11;
        locals.var_qbody_dn12 = assign29250_e27485_d_n12;
        locals.var_qbody_rv = 0.0;

        let (assign29260_e27493, assign29260_e27493_d_n3, assign29260_e27493_d_n4, assign29260_e27493_d_n5, assign29260_e27493_d_n6, assign29260_e27493_d_n7, assign29260_e27493_d_n8, assign29260_e27493_d_n9, assign29260_e27493_d_n10, assign29260_e27493_d_n11, assign29260_e27493_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qsrc, locals.var_qsrc_dn3, locals.var_qsrc_dn4, locals.var_qsrc_dn5, locals.var_qsrc_dn6, locals.var_qsrc_dn7, locals.var_qsrc_dn8, locals.var_qsrc_dn9, locals.var_qsrc_dn10, locals.var_qsrc_dn11, locals.var_qsrc_dn12,)
    }
};
        locals.var_qsrc = assign29260_e27493;
        locals.var_qsrc_dn3 = assign29260_e27493_d_n3;
        locals.var_qsrc_dn4 = assign29260_e27493_d_n4;
        locals.var_qsrc_dn5 = assign29260_e27493_d_n5;
        locals.var_qsrc_dn6 = assign29260_e27493_d_n6;
        locals.var_qsrc_dn7 = assign29260_e27493_d_n7;
        locals.var_qsrc_dn8 = assign29260_e27493_d_n8;
        locals.var_qsrc_dn9 = assign29260_e27493_d_n9;
        locals.var_qsrc_dn10 = assign29260_e27493_d_n10;
        locals.var_qsrc_dn11 = assign29260_e27493_d_n11;
        locals.var_qsrc_dn12 = assign29260_e27493_d_n12;
        locals.var_qsrc_rv = 0.0;

        let (assign29270_e27501, assign29270_e27501_d_n3, assign29270_e27501_d_n4, assign29270_e27501_d_n5, assign29270_e27501_d_n6, assign29270_e27501_d_n7, assign29270_e27501_d_n8, assign29270_e27501_d_n9, assign29270_e27501_d_n10, assign29270_e27501_d_n11, assign29270_e27501_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qdrn, locals.var_qdrn_dn3, locals.var_qdrn_dn4, locals.var_qdrn_dn5, locals.var_qdrn_dn6, locals.var_qdrn_dn7, locals.var_qdrn_dn8, locals.var_qdrn_dn9, locals.var_qdrn_dn10, locals.var_qdrn_dn11, locals.var_qdrn_dn12,)
    }
};
        locals.var_qdrn = assign29270_e27501;
        locals.var_qdrn_dn3 = assign29270_e27501_d_n3;
        locals.var_qdrn_dn4 = assign29270_e27501_d_n4;
        locals.var_qdrn_dn5 = assign29270_e27501_d_n5;
        locals.var_qdrn_dn6 = assign29270_e27501_d_n6;
        locals.var_qdrn_dn7 = assign29270_e27501_d_n7;
        locals.var_qdrn_dn8 = assign29270_e27501_d_n8;
        locals.var_qdrn_dn9 = assign29270_e27501_d_n9;
        locals.var_qdrn_dn10 = assign29270_e27501_d_n10;
        locals.var_qdrn_dn11 = assign29270_e27501_d_n11;
        locals.var_qdrn_dn12 = assign29270_e27501_d_n12;
        locals.var_qdrn_rv = 0.0;

        let (assign29280_e27509, assign29280_e27509_d_n3, assign29280_e27509_d_n4, assign29280_e27509_d_n5, assign29280_e27509_d_n6, assign29280_e27509_d_n7, assign29280_e27509_d_n8, assign29280_e27509_d_n9, assign29280_e27509_d_n10, assign29280_e27509_d_n11, assign29280_e27509_d_n12,) = {
    if ((locals.var_guard1367 == 0.0) && (locals.var_guard1385 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qgate, locals.var_qgate_dn3, locals.var_qgate_dn4, locals.var_qgate_dn5, locals.var_qgate_dn6, locals.var_qgate_dn7, locals.var_qgate_dn8, locals.var_qgate_dn9, locals.var_qgate_dn10, locals.var_qgate_dn11, locals.var_qgate_dn12,)
    }
};
        locals.var_qgate = assign29280_e27509;
        locals.var_qgate_dn3 = assign29280_e27509_d_n3;
        locals.var_qgate_dn4 = assign29280_e27509_d_n4;
        locals.var_qgate_dn5 = assign29280_e27509_d_n5;
        locals.var_qgate_dn6 = assign29280_e27509_d_n6;
        locals.var_qgate_dn7 = assign29280_e27509_d_n7;
        locals.var_qgate_dn8 = assign29280_e27509_d_n8;
        locals.var_qgate_dn9 = assign29280_e27509_d_n9;
        locals.var_qgate_dn10 = assign29280_e27509_d_n10;
        locals.var_qgate_dn11 = assign29280_e27509_d_n11;
        locals.var_qgate_dn12 = assign29280_e27509_d_n12;
        locals.var_qgate_rv = 0.0;

        let assign29290_e27512: f64 = if locals.var_b4soisoimod == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1419 = assign29290_e27512;
        locals.var_guard1419_rv = 0.0;

        let (assign29300_e27516, assign29300_e27516_d_n3, assign29300_e27516_d_n4, assign29300_e27516_d_n5, assign29300_e27516_d_n6, assign29300_e27516_d_n7, assign29300_e27516_d_n8, assign29300_e27516_d_n9, assign29300_e27516_d_n10, assign29300_e27516_d_n11, assign29300_e27516_d_n12,) = {
    if (locals.var_guard1419 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qjd_1, locals.var_qjd_1_dn3, locals.var_qjd_1_dn4, locals.var_qjd_1_dn5, locals.var_qjd_1_dn6, locals.var_qjd_1_dn7, locals.var_qjd_1_dn8, locals.var_qjd_1_dn9, locals.var_qjd_1_dn10, locals.var_qjd_1_dn11, locals.var_qjd_1_dn12,)
    }
};
        locals.var_qjd_1 = assign29300_e27516;
        locals.var_qjd_1_dn3 = assign29300_e27516_d_n3;
        locals.var_qjd_1_dn4 = assign29300_e27516_d_n4;
        locals.var_qjd_1_dn5 = assign29300_e27516_d_n5;
        locals.var_qjd_1_dn6 = assign29300_e27516_d_n6;
        locals.var_qjd_1_dn7 = assign29300_e27516_d_n7;
        locals.var_qjd_1_dn8 = assign29300_e27516_d_n8;
        locals.var_qjd_1_dn9 = assign29300_e27516_d_n9;
        locals.var_qjd_1_dn10 = assign29300_e27516_d_n10;
        locals.var_qjd_1_dn11 = assign29300_e27516_d_n11;
        locals.var_qjd_1_dn12 = assign29300_e27516_d_n12;
        locals.var_qjd_1_rv = 0.0;

        let (assign29310_e27520, assign29310_e27520_d_n3, assign29310_e27520_d_n4, assign29310_e27520_d_n5, assign29310_e27520_d_n6, assign29310_e27520_d_n7, assign29310_e27520_d_n8, assign29310_e27520_d_n9, assign29310_e27520_d_n10, assign29310_e27520_d_n11, assign29310_e27520_d_n12,) = {
    if (locals.var_guard1419 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qjs_1, locals.var_qjs_1_dn3, locals.var_qjs_1_dn4, locals.var_qjs_1_dn5, locals.var_qjs_1_dn6, locals.var_qjs_1_dn7, locals.var_qjs_1_dn8, locals.var_qjs_1_dn9, locals.var_qjs_1_dn10, locals.var_qjs_1_dn11, locals.var_qjs_1_dn12,)
    }
};
        locals.var_qjs_1 = assign29310_e27520;
        locals.var_qjs_1_dn3 = assign29310_e27520_d_n3;
        locals.var_qjs_1_dn4 = assign29310_e27520_d_n4;
        locals.var_qjs_1_dn5 = assign29310_e27520_d_n5;
        locals.var_qjs_1_dn6 = assign29310_e27520_d_n6;
        locals.var_qjs_1_dn7 = assign29310_e27520_d_n7;
        locals.var_qjs_1_dn8 = assign29310_e27520_d_n8;
        locals.var_qjs_1_dn9 = assign29310_e27520_d_n9;
        locals.var_qjs_1_dn10 = assign29310_e27520_d_n10;
        locals.var_qjs_1_dn11 = assign29310_e27520_d_n11;
        locals.var_qjs_1_dn12 = assign29310_e27520_d_n12;
        locals.var_qjs_1_rv = 0.0;

        let (assign29320_e27525, assign29320_e27525_d_n4, assign29320_e27525_d_n5, assign29320_e27525_d_n6,) = {
    if (locals.var_guard1419 == 0.0) {
        (locals.var_b4soigatesidewalljctspotential, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phibswg, locals.var_phibswg_dn4, locals.var_phibswg_dn5, locals.var_phibswg_dn6,)
    }
};
        locals.var_phibswg = assign29320_e27525;
        locals.var_phibswg_dn4 = assign29320_e27525_d_n4;
        locals.var_phibswg_dn5 = assign29320_e27525_d_n5;
        locals.var_phibswg_dn6 = assign29320_e27525_d_n6;
        locals.var_phibswg_rv = 0.0;

        let (assign29330_e27531,) = {
    if (locals.var_guard1419 == 0.0) {
        let assign29330_e27529: f64 = (-p.p363);
        (assign29330_e27529,)
    } else {
        (locals.var_dphibswg_dt,)
    }
};
        locals.var_dphibswg_dt = assign29330_e27531;
        locals.var_dphibswg_dt_rv = 0.0;

        let (assign29340_e27542, assign29340_e27542_d_n4, assign29340_e27542_d_n5, assign29340_e27542_d_n6,) = {
    if (locals.var_guard1419 == 0.0) {
        let assign29340_e27538: f64 = (locals.var_devtemp - locals.var_tnom);
        let assign29340_e27539: f64 = (locals.var_dphibswg_dt * assign29340_e27538);
        let assign29340_e27540: f64 = (locals.var_phibswg + assign29340_e27539);
        (assign29340_e27540, (locals.var_phibswg_dn4 + (locals.var_dphibswg_dt * locals.var_devtemp_dn4)), (locals.var_phibswg_dn5 + (locals.var_dphibswg_dt * locals.var_devtemp_dn5)), (locals.var_phibswg_dn6 + (locals.var_dphibswg_dt * locals.var_devtemp_dn6)),)
    } else {
        (locals.var_phibswg, locals.var_phibswg_dn4, locals.var_phibswg_dn5, locals.var_phibswg_dn6,)
    }
};
        locals.var_phibswg = assign29340_e27542;
        locals.var_phibswg_dn4 = assign29340_e27542_d_n4;
        locals.var_phibswg_dn5 = assign29340_e27542_d_n5;
        locals.var_phibswg_dn6 = assign29340_e27542_d_n6;
        locals.var_phibswg_rv = 0.0;

        let (assign29350_e27547,) = {
    if (locals.var_guard1419 == 0.0) {
        (p.p183,)
    } else {
        (locals.var_bmjswg,)
    }
};
        locals.var_bmjswg = assign29350_e27547;
        locals.var_bmjswg_rv = 0.0;

        let (assign29360_e27560, assign29360_e27560_d_n3, assign29360_e27560_d_n4, assign29360_e27560_d_n5, assign29360_e27560_d_n6, assign29360_e27560_d_n7, assign29360_e27560_d_n8, assign29360_e27560_d_n9, assign29360_e27560_d_n10, assign29360_e27560_d_n11, assign29360_e27560_d_n12,) = {
    if (locals.var_guard1419 == 0.0) {
        let assign29360_e27552: f64 = (p.p185 * locals.var_pparam_b4soiwdioscv);
        let assign29360_e27554: f64 = (assign29360_e27552 * p.p155);
        let assign29360_e27556: f64 = (assign29360_e27554 * p.p3);
        let assign29360_e27558: f64 = (assign29360_e27556 / 1e-7);
        (assign29360_e27558, ((((p.p185 * locals.var_pparam_b4soiwdioscv_dn3) * p.p155) * p.p3) / 1e-7), ((((p.p185 * locals.var_pparam_b4soiwdioscv_dn4) * p.p155) * p.p3) / 1e-7), ((((p.p185 * locals.var_pparam_b4soiwdioscv_dn5) * p.p155) * p.p3) / 1e-7), ((((p.p185 * locals.var_pparam_b4soiwdioscv_dn6) * p.p155) * p.p3) / 1e-7), ((((p.p185 * locals.var_pparam_b4soiwdioscv_dn7) * p.p155) * p.p3) / 1e-7), ((((p.p185 * locals.var_pparam_b4soiwdioscv_dn8) * p.p155) * p.p3) / 1e-7), ((((p.p185 * locals.var_pparam_b4soiwdioscv_dn9) * p.p155) * p.p3) / 1e-7), ((((p.p185 * locals.var_pparam_b4soiwdioscv_dn10) * p.p155) * p.p3) / 1e-7), ((((p.p185 * locals.var_pparam_b4soiwdioscv_dn11) * p.p155) * p.p3) / 1e-7), ((((p.p185 * locals.var_pparam_b4soiwdioscv_dn12) * p.p155) * p.p3) / 1e-7),)
    } else {
        (locals.var_cjsbs, locals.var_cjsbs_dn3, locals.var_cjsbs_dn4, locals.var_cjsbs_dn5, locals.var_cjsbs_dn6, locals.var_cjsbs_dn7, locals.var_cjsbs_dn8, locals.var_cjsbs_dn9, locals.var_cjsbs_dn10, locals.var_cjsbs_dn11, locals.var_cjsbs_dn12,)
    }
};
        locals.var_cjsbs = assign29360_e27560;
        locals.var_cjsbs_dn3 = assign29360_e27560_d_n3;
        locals.var_cjsbs_dn4 = assign29360_e27560_d_n4;
        locals.var_cjsbs_dn5 = assign29360_e27560_d_n5;
        locals.var_cjsbs_dn6 = assign29360_e27560_d_n6;
        locals.var_cjsbs_dn7 = assign29360_e27560_d_n7;
        locals.var_cjsbs_dn8 = assign29360_e27560_d_n8;
        locals.var_cjsbs_dn9 = assign29360_e27560_d_n9;
        locals.var_cjsbs_dn10 = assign29360_e27560_d_n10;
        locals.var_cjsbs_dn11 = assign29360_e27560_d_n11;
        locals.var_cjsbs_dn12 = assign29360_e27560_d_n12;
        locals.var_cjsbs_rv = 0.0;

        let (assign29370_e27567, assign29370_e27567_d_n3, assign29370_e27567_d_n4, assign29370_e27567_d_n5, assign29370_e27567_d_n6, assign29370_e27567_d_n7, assign29370_e27567_d_n8, assign29370_e27567_d_n9, assign29370_e27567_d_n10, assign29370_e27567_d_n11, assign29370_e27567_d_n12,) = {
    if (locals.var_guard1419 == 0.0) {
        let assign29370_e27565: f64 = (locals.var_cjsbs * p.p362);
        (assign29370_e27565, (locals.var_cjsbs_dn3 * p.p362), (locals.var_cjsbs_dn4 * p.p362), (locals.var_cjsbs_dn5 * p.p362), (locals.var_cjsbs_dn6 * p.p362), (locals.var_cjsbs_dn7 * p.p362), (locals.var_cjsbs_dn8 * p.p362), (locals.var_cjsbs_dn9 * p.p362), (locals.var_cjsbs_dn10 * p.p362), (locals.var_cjsbs_dn11 * p.p362), (locals.var_cjsbs_dn12 * p.p362),)
    } else {
        (locals.var_dcjsbs_dt, locals.var_dcjsbs_dt_dn3, locals.var_dcjsbs_dt_dn4, locals.var_dcjsbs_dt_dn5, locals.var_dcjsbs_dt_dn6, locals.var_dcjsbs_dt_dn7, locals.var_dcjsbs_dt_dn8, locals.var_dcjsbs_dt_dn9, locals.var_dcjsbs_dt_dn10, locals.var_dcjsbs_dt_dn11, locals.var_dcjsbs_dt_dn12,)
    }
};
        locals.var_dcjsbs_dt = assign29370_e27567;
        locals.var_dcjsbs_dt_dn3 = assign29370_e27567_d_n3;
        locals.var_dcjsbs_dt_dn4 = assign29370_e27567_d_n4;
        locals.var_dcjsbs_dt_dn5 = assign29370_e27567_d_n5;
        locals.var_dcjsbs_dt_dn6 = assign29370_e27567_d_n6;
        locals.var_dcjsbs_dt_dn7 = assign29370_e27567_d_n7;
        locals.var_dcjsbs_dt_dn8 = assign29370_e27567_d_n8;
        locals.var_dcjsbs_dt_dn9 = assign29370_e27567_d_n9;
        locals.var_dcjsbs_dt_dn10 = assign29370_e27567_d_n10;
        locals.var_dcjsbs_dt_dn11 = assign29370_e27567_d_n11;
        locals.var_dcjsbs_dt_dn12 = assign29370_e27567_d_n12;
        locals.var_dcjsbs_dt_rv = 0.0;

        let (assign29380_e27578, assign29380_e27578_d_n3, assign29380_e27578_d_n4, assign29380_e27578_d_n5, assign29380_e27578_d_n6, assign29380_e27578_d_n7, assign29380_e27578_d_n8, assign29380_e27578_d_n9, assign29380_e27578_d_n10, assign29380_e27578_d_n11, assign29380_e27578_d_n12,) = {
    if (locals.var_guard1419 == 0.0) {
        let assign29380_e27574: f64 = (locals.var_devtemp - locals.var_tnom);
        let assign29380_e27575: f64 = (locals.var_dcjsbs_dt * assign29380_e27574);
        let assign29380_e27576: f64 = (locals.var_cjsbs + assign29380_e27575);
        (assign29380_e27576, (locals.var_cjsbs_dn3 + (locals.var_dcjsbs_dt_dn3 * assign29380_e27574)), (locals.var_cjsbs_dn4 + ((locals.var_dcjsbs_dt_dn4 * assign29380_e27574) + (locals.var_dcjsbs_dt * locals.var_devtemp_dn4))), (locals.var_cjsbs_dn5 + ((locals.var_dcjsbs_dt_dn5 * assign29380_e27574) + (locals.var_dcjsbs_dt * locals.var_devtemp_dn5))), (locals.var_cjsbs_dn6 + ((locals.var_dcjsbs_dt_dn6 * assign29380_e27574) + (locals.var_dcjsbs_dt * locals.var_devtemp_dn6))), (locals.var_cjsbs_dn7 + (locals.var_dcjsbs_dt_dn7 * assign29380_e27574)), (locals.var_cjsbs_dn8 + (locals.var_dcjsbs_dt_dn8 * assign29380_e27574)), (locals.var_cjsbs_dn9 + (locals.var_dcjsbs_dt_dn9 * assign29380_e27574)), (locals.var_cjsbs_dn10 + (locals.var_dcjsbs_dt_dn10 * assign29380_e27574)), (locals.var_cjsbs_dn11 + (locals.var_dcjsbs_dt_dn11 * assign29380_e27574)), (locals.var_cjsbs_dn12 + (locals.var_dcjsbs_dt_dn12 * assign29380_e27574)),)
    } else {
        (locals.var_cjsbs, locals.var_cjsbs_dn3, locals.var_cjsbs_dn4, locals.var_cjsbs_dn5, locals.var_cjsbs_dn6, locals.var_cjsbs_dn7, locals.var_cjsbs_dn8, locals.var_cjsbs_dn9, locals.var_cjsbs_dn10, locals.var_cjsbs_dn11, locals.var_cjsbs_dn12,)
    }
};
        locals.var_cjsbs = assign29380_e27578;
        locals.var_cjsbs_dn3 = assign29380_e27578_d_n3;
        locals.var_cjsbs_dn4 = assign29380_e27578_d_n4;
        locals.var_cjsbs_dn5 = assign29380_e27578_d_n5;
        locals.var_cjsbs_dn6 = assign29380_e27578_d_n6;
        locals.var_cjsbs_dn7 = assign29380_e27578_d_n7;
        locals.var_cjsbs_dn8 = assign29380_e27578_d_n8;
        locals.var_cjsbs_dn9 = assign29380_e27578_d_n9;
        locals.var_cjsbs_dn10 = assign29380_e27578_d_n10;
        locals.var_cjsbs_dn11 = assign29380_e27578_d_n11;
        locals.var_cjsbs_dn12 = assign29380_e27578_d_n12;
        locals.var_cjsbs_rv = 0.0;

        let (assign29390_e27591, assign29390_e27591_d_n3, assign29390_e27591_d_n4, assign29390_e27591_d_n5, assign29390_e27591_d_n6, assign29390_e27591_d_n7, assign29390_e27591_d_n8, assign29390_e27591_d_n9, assign29390_e27591_d_n10, assign29390_e27591_d_n11, assign29390_e27591_d_n12,) = {
    if (locals.var_guard1419 == 0.0) {
        let assign29390_e27583: f64 = (p.p186 * locals.var_pparam_b4soiwdiodcv);
        let assign29390_e27585: f64 = (assign29390_e27583 * p.p155);
        let assign29390_e27587: f64 = (assign29390_e27585 * p.p3);
        let assign29390_e27589: f64 = (assign29390_e27587 / 1e-7);
        (assign29390_e27589, ((((p.p186 * locals.var_pparam_b4soiwdiodcv_dn3) * p.p155) * p.p3) / 1e-7), ((((p.p186 * locals.var_pparam_b4soiwdiodcv_dn4) * p.p155) * p.p3) / 1e-7), ((((p.p186 * locals.var_pparam_b4soiwdiodcv_dn5) * p.p155) * p.p3) / 1e-7), ((((p.p186 * locals.var_pparam_b4soiwdiodcv_dn6) * p.p155) * p.p3) / 1e-7), ((((p.p186 * locals.var_pparam_b4soiwdiodcv_dn7) * p.p155) * p.p3) / 1e-7), ((((p.p186 * locals.var_pparam_b4soiwdiodcv_dn8) * p.p155) * p.p3) / 1e-7), ((((p.p186 * locals.var_pparam_b4soiwdiodcv_dn9) * p.p155) * p.p3) / 1e-7), ((((p.p186 * locals.var_pparam_b4soiwdiodcv_dn10) * p.p155) * p.p3) / 1e-7), ((((p.p186 * locals.var_pparam_b4soiwdiodcv_dn11) * p.p155) * p.p3) / 1e-7), ((((p.p186 * locals.var_pparam_b4soiwdiodcv_dn12) * p.p155) * p.p3) / 1e-7),)
    } else {
        (locals.var_cjdbs, locals.var_cjdbs_dn3, locals.var_cjdbs_dn4, locals.var_cjdbs_dn5, locals.var_cjdbs_dn6, locals.var_cjdbs_dn7, locals.var_cjdbs_dn8, locals.var_cjdbs_dn9, locals.var_cjdbs_dn10, locals.var_cjdbs_dn11, locals.var_cjdbs_dn12,)
    }
};
        locals.var_cjdbs = assign29390_e27591;
        locals.var_cjdbs_dn3 = assign29390_e27591_d_n3;
        locals.var_cjdbs_dn4 = assign29390_e27591_d_n4;
        locals.var_cjdbs_dn5 = assign29390_e27591_d_n5;
        locals.var_cjdbs_dn6 = assign29390_e27591_d_n6;
        locals.var_cjdbs_dn7 = assign29390_e27591_d_n7;
        locals.var_cjdbs_dn8 = assign29390_e27591_d_n8;
        locals.var_cjdbs_dn9 = assign29390_e27591_d_n9;
        locals.var_cjdbs_dn10 = assign29390_e27591_d_n10;
        locals.var_cjdbs_dn11 = assign29390_e27591_d_n11;
        locals.var_cjdbs_dn12 = assign29390_e27591_d_n12;
        locals.var_cjdbs_rv = 0.0;

        let (assign29400_e27598, assign29400_e27598_d_n3, assign29400_e27598_d_n4, assign29400_e27598_d_n5, assign29400_e27598_d_n6, assign29400_e27598_d_n7, assign29400_e27598_d_n8, assign29400_e27598_d_n9, assign29400_e27598_d_n10, assign29400_e27598_d_n11, assign29400_e27598_d_n12,) = {
    if (locals.var_guard1419 == 0.0) {
        let assign29400_e27596: f64 = (locals.var_cjdbs * p.p364);
        (assign29400_e27596, (locals.var_cjdbs_dn3 * p.p364), (locals.var_cjdbs_dn4 * p.p364), (locals.var_cjdbs_dn5 * p.p364), (locals.var_cjdbs_dn6 * p.p364), (locals.var_cjdbs_dn7 * p.p364), (locals.var_cjdbs_dn8 * p.p364), (locals.var_cjdbs_dn9 * p.p364), (locals.var_cjdbs_dn10 * p.p364), (locals.var_cjdbs_dn11 * p.p364), (locals.var_cjdbs_dn12 * p.p364),)
    } else {
        (locals.var_dcjdbs_dt, locals.var_dcjdbs_dt_dn3, locals.var_dcjdbs_dt_dn4, locals.var_dcjdbs_dt_dn5, locals.var_dcjdbs_dt_dn6, locals.var_dcjdbs_dt_dn7, locals.var_dcjdbs_dt_dn8, locals.var_dcjdbs_dt_dn9, locals.var_dcjdbs_dt_dn10, locals.var_dcjdbs_dt_dn11, locals.var_dcjdbs_dt_dn12,)
    }
};
        locals.var_dcjdbs_dt = assign29400_e27598;
        locals.var_dcjdbs_dt_dn3 = assign29400_e27598_d_n3;
        locals.var_dcjdbs_dt_dn4 = assign29400_e27598_d_n4;
        locals.var_dcjdbs_dt_dn5 = assign29400_e27598_d_n5;
        locals.var_dcjdbs_dt_dn6 = assign29400_e27598_d_n6;
        locals.var_dcjdbs_dt_dn7 = assign29400_e27598_d_n7;
        locals.var_dcjdbs_dt_dn8 = assign29400_e27598_d_n8;
        locals.var_dcjdbs_dt_dn9 = assign29400_e27598_d_n9;
        locals.var_dcjdbs_dt_dn10 = assign29400_e27598_d_n10;
        locals.var_dcjdbs_dt_dn11 = assign29400_e27598_d_n11;
        locals.var_dcjdbs_dt_dn12 = assign29400_e27598_d_n12;
        locals.var_dcjdbs_dt_rv = 0.0;

        let (assign29410_e27609, assign29410_e27609_d_n3, assign29410_e27609_d_n4, assign29410_e27609_d_n5, assign29410_e27609_d_n6, assign29410_e27609_d_n7, assign29410_e27609_d_n8, assign29410_e27609_d_n9, assign29410_e27609_d_n10, assign29410_e27609_d_n11, assign29410_e27609_d_n12,) = {
    if (locals.var_guard1419 == 0.0) {
        let assign29410_e27605: f64 = (locals.var_devtemp - locals.var_tnom);
        let assign29410_e27606: f64 = (locals.var_dcjdbs_dt * assign29410_e27605);
        let assign29410_e27607: f64 = (locals.var_cjdbs + assign29410_e27606);
        (assign29410_e27607, (locals.var_cjdbs_dn3 + (locals.var_dcjdbs_dt_dn3 * assign29410_e27605)), (locals.var_cjdbs_dn4 + ((locals.var_dcjdbs_dt_dn4 * assign29410_e27605) + (locals.var_dcjdbs_dt * locals.var_devtemp_dn4))), (locals.var_cjdbs_dn5 + ((locals.var_dcjdbs_dt_dn5 * assign29410_e27605) + (locals.var_dcjdbs_dt * locals.var_devtemp_dn5))), (locals.var_cjdbs_dn6 + ((locals.var_dcjdbs_dt_dn6 * assign29410_e27605) + (locals.var_dcjdbs_dt * locals.var_devtemp_dn6))), (locals.var_cjdbs_dn7 + (locals.var_dcjdbs_dt_dn7 * assign29410_e27605)), (locals.var_cjdbs_dn8 + (locals.var_dcjdbs_dt_dn8 * assign29410_e27605)), (locals.var_cjdbs_dn9 + (locals.var_dcjdbs_dt_dn9 * assign29410_e27605)), (locals.var_cjdbs_dn10 + (locals.var_dcjdbs_dt_dn10 * assign29410_e27605)), (locals.var_cjdbs_dn11 + (locals.var_dcjdbs_dt_dn11 * assign29410_e27605)), (locals.var_cjdbs_dn12 + (locals.var_dcjdbs_dt_dn12 * assign29410_e27605)),)
    } else {
        (locals.var_cjdbs, locals.var_cjdbs_dn3, locals.var_cjdbs_dn4, locals.var_cjdbs_dn5, locals.var_cjdbs_dn6, locals.var_cjdbs_dn7, locals.var_cjdbs_dn8, locals.var_cjdbs_dn9, locals.var_cjdbs_dn10, locals.var_cjdbs_dn11, locals.var_cjdbs_dn12,)
    }
};
        locals.var_cjdbs = assign29410_e27609;
        locals.var_cjdbs_dn3 = assign29410_e27609_d_n3;
        locals.var_cjdbs_dn4 = assign29410_e27609_d_n4;
        locals.var_cjdbs_dn5 = assign29410_e27609_d_n5;
        locals.var_cjdbs_dn6 = assign29410_e27609_d_n6;
        locals.var_cjdbs_dn7 = assign29410_e27609_d_n7;
        locals.var_cjdbs_dn8 = assign29410_e27609_d_n8;
        locals.var_cjdbs_dn9 = assign29410_e27609_d_n9;
        locals.var_cjdbs_dn10 = assign29410_e27609_d_n10;
        locals.var_cjdbs_dn11 = assign29410_e27609_d_n11;
        locals.var_cjdbs_dn12 = assign29410_e27609_d_n12;
        locals.var_cjdbs_rv = 0.0;

        let (assign29420_e27616, assign29420_e27616_d_n4, assign29420_e27616_d_n5, assign29420_e27616_d_n6,) = {
    if (locals.var_guard1419 == 0.0) {
        let assign29420_e27614: f64 = (0.9 * locals.var_phibswg);
        (assign29420_e27614, (0.9 * locals.var_phibswg_dn4), (0.9 * locals.var_phibswg_dn5), (0.9 * locals.var_phibswg_dn6),)
    } else {
        (locals.var_diomax, locals.var_diomax_dn4, locals.var_diomax_dn5, locals.var_diomax_dn6,)
    }
};
        locals.var_diomax = assign29420_e27616;
        locals.var_diomax_dn4 = assign29420_e27616_d_n4;
        locals.var_diomax_dn5 = assign29420_e27616_d_n5;
        locals.var_diomax_dn6 = assign29420_e27616_d_n6;
        locals.var_diomax_rv = 0.0;

        let (assign29430_e27630, assign29430_e27630_d_n4, assign29430_e27630_d_n5, assign29430_e27630_d_n6, assign29430_e27630_d_n7, assign29430_e27630_d_n8, assign29430_e27630_d_n11, assign29430_e27630_d_n12,) = {
    if (locals.var_guard1419 == 0.0) {
        let (assign29430_e27625, assign29430_e27625_d_n4, assign29430_e27625_d_n5, assign29430_e27625_d_n6, assign29430_e27625_d_n8, assign29430_e27625_d_n11,) = {
            if (locals.var_vsbs > locals.var_diomax) {
                (locals.var_diomax, locals.var_diomax_dn4, locals.var_diomax_dn5, locals.var_diomax_dn6, 0.0, 0.0,)
            } else {
                (locals.var_vsbs, 0.0, 0.0, 0.0, locals.var_vsbs_dn8, locals.var_vsbs_dn11,)
            }
        };
        let assign29430_e27627: f64 = (assign29430_e27625 / locals.var_phibswg);
        let assign29430_e27628: f64 = (1.0 - assign29430_e27627);
        (assign29430_e27628, (-(((assign29430_e27625_d_n4 * locals.var_phibswg) - (assign29430_e27625 * locals.var_phibswg_dn4)) / (locals.var_phibswg * locals.var_phibswg))), (-(((assign29430_e27625_d_n5 * locals.var_phibswg) - (assign29430_e27625 * locals.var_phibswg_dn5)) / (locals.var_phibswg * locals.var_phibswg))), (-(((assign29430_e27625_d_n6 * locals.var_phibswg) - (assign29430_e27625 * locals.var_phibswg_dn6)) / (locals.var_phibswg * locals.var_phibswg))), 0.0, (-(assign29430_e27625_d_n8 / locals.var_phibswg)), (-(assign29430_e27625_d_n11 / locals.var_phibswg)), 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn11, locals.var_arg_dn12,)
    }
};
        locals.var_arg = assign29430_e27630;
        locals.var_arg_dn4 = assign29430_e27630_d_n4;
        locals.var_arg_dn5 = assign29430_e27630_d_n5;
        locals.var_arg_dn6 = assign29430_e27630_d_n6;
        locals.var_arg_dn7 = assign29430_e27630_d_n7;
        locals.var_arg_dn8 = assign29430_e27630_d_n8;
        locals.var_arg_dn11 = assign29430_e27630_d_n11;
        locals.var_arg_dn12 = assign29430_e27630_d_n12;
        locals.var_arg_rv = 0.0;

        let assign29440_e27633: f64 = if locals.var_bmjswg == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1420 = assign29440_e27633;
        locals.var_guard1420_rv = 0.0;

        let (assign29450_e27643, assign29450_e27643_d_n4, assign29450_e27643_d_n5, assign29450_e27643_d_n6, assign29450_e27643_d_n7, assign29450_e27643_d_n8, assign29450_e27643_d_n11, assign29450_e27643_d_n12,) = {
    if ((locals.var_guard1419 == 0.0) && (locals.var_guard1420 != 0.0)) {
        let assign29450_e27640: f64 = (locals.var_arg).sqrt();
        let assign29450_e27641: f64 = (1.0 / assign29450_e27640);
        (assign29450_e27641, (-((locals.var_arg_dn4 / (2.0 * assign29450_e27640)) / (assign29450_e27640 * assign29450_e27640))), (-((locals.var_arg_dn5 / (2.0 * assign29450_e27640)) / (assign29450_e27640 * assign29450_e27640))), (-((locals.var_arg_dn6 / (2.0 * assign29450_e27640)) / (assign29450_e27640 * assign29450_e27640))), (-((locals.var_arg_dn7 / (2.0 * assign29450_e27640)) / (assign29450_e27640 * assign29450_e27640))), (-((locals.var_arg_dn8 / (2.0 * assign29450_e27640)) / (assign29450_e27640 * assign29450_e27640))), (-((locals.var_arg_dn11 / (2.0 * assign29450_e27640)) / (assign29450_e27640 * assign29450_e27640))), (-((locals.var_arg_dn12 / (2.0 * assign29450_e27640)) / (assign29450_e27640 * assign29450_e27640))),)
    } else {
        (locals.var_dt3_dvb, locals.var_dt3_dvb_dn4, locals.var_dt3_dvb_dn5, locals.var_dt3_dvb_dn6, locals.var_dt3_dvb_dn7, locals.var_dt3_dvb_dn8, locals.var_dt3_dvb_dn11, locals.var_dt3_dvb_dn12,)
    }
};
        locals.var_dt3_dvb = assign29450_e27643;
        locals.var_dt3_dvb_dn4 = assign29450_e27643_d_n4;
        locals.var_dt3_dvb_dn5 = assign29450_e27643_d_n5;
        locals.var_dt3_dvb_dn6 = assign29450_e27643_d_n6;
        locals.var_dt3_dvb_dn7 = assign29450_e27643_d_n7;
        locals.var_dt3_dvb_dn8 = assign29450_e27643_d_n8;
        locals.var_dt3_dvb_dn11 = assign29450_e27643_d_n11;
        locals.var_dt3_dvb_dn12 = assign29450_e27643_d_n12;
        locals.var_dt3_dvb_rv = 0.0;

        let (assign29460_e27662, assign29460_e27662_d_n4, assign29460_e27662_d_n5, assign29460_e27662_d_n6, assign29460_e27662_d_n7, assign29460_e27662_d_n8, assign29460_e27662_d_n11, assign29460_e27662_d_n12,) = {
    if ((locals.var_guard1419 == 0.0) && (locals.var_guard1420 == 0.0)) {
        let assign29460_e27650: f64 = (-locals.var_bmjswg);
        let (assign29460_e27658, assign29460_e27658_d_n4, assign29460_e27658_d_n5, assign29460_e27658_d_n6, assign29460_e27658_d_n7, assign29460_e27658_d_n8, assign29460_e27658_d_n11, assign29460_e27658_d_n12,) = {
            if (locals.var_arg > 1e-38) {
                let assign29460_e27655: f64 = (locals.var_arg).ln();
                (assign29460_e27655, (locals.var_arg_dn4 / locals.var_arg), (locals.var_arg_dn5 / locals.var_arg), (locals.var_arg_dn6 / locals.var_arg), (locals.var_arg_dn7 / locals.var_arg), (locals.var_arg_dn8 / locals.var_arg), (locals.var_arg_dn11 / locals.var_arg), (locals.var_arg_dn12 / locals.var_arg),)
            } else {
                let assign29460_e27657: f64 = (-87.49823353377374);
                (assign29460_e27657, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign29460_e27659: f64 = (assign29460_e27650 * assign29460_e27658);
        let assign29460_e27660: f64 = (assign29460_e27659).exp();
        (assign29460_e27660, (assign29460_e27660 * (assign29460_e27650 * assign29460_e27658_d_n4)), (assign29460_e27660 * (assign29460_e27650 * assign29460_e27658_d_n5)), (assign29460_e27660 * (assign29460_e27650 * assign29460_e27658_d_n6)), (assign29460_e27660 * (assign29460_e27650 * assign29460_e27658_d_n7)), (assign29460_e27660 * (assign29460_e27650 * assign29460_e27658_d_n8)), (assign29460_e27660 * (assign29460_e27650 * assign29460_e27658_d_n11)), (assign29460_e27660 * (assign29460_e27650 * assign29460_e27658_d_n12)),)
    } else {
        (locals.var_dt3_dvb, locals.var_dt3_dvb_dn4, locals.var_dt3_dvb_dn5, locals.var_dt3_dvb_dn6, locals.var_dt3_dvb_dn7, locals.var_dt3_dvb_dn8, locals.var_dt3_dvb_dn11, locals.var_dt3_dvb_dn12,)
    }
};
        locals.var_dt3_dvb = assign29460_e27662;
        locals.var_dt3_dvb_dn4 = assign29460_e27662_d_n4;
        locals.var_dt3_dvb_dn5 = assign29460_e27662_d_n5;
        locals.var_dt3_dvb_dn6 = assign29460_e27662_d_n6;
        locals.var_dt3_dvb_dn7 = assign29460_e27662_d_n7;
        locals.var_dt3_dvb_dn8 = assign29460_e27662_d_n8;
        locals.var_dt3_dvb_dn11 = assign29460_e27662_d_n11;
        locals.var_dt3_dvb_dn12 = assign29460_e27662_d_n12;
        locals.var_dt3_dvb_rv = 0.0;

        let (assign29470_e27677, assign29470_e27677_d_n3, assign29470_e27677_d_n4, assign29470_e27677_d_n5, assign29470_e27677_d_n6, assign29470_e27677_d_n7, assign29470_e27677_d_n8, assign29470_e27677_d_n9, assign29470_e27677_d_n10, assign29470_e27677_d_n11, assign29470_e27677_d_n12,) = {
    if (locals.var_guard1419 == 0.0) {
        let assign29470_e27668: f64 = (locals.var_arg * locals.var_dt3_dvb);
        let assign29470_e27669: f64 = (1.0 - assign29470_e27668);
        let assign29470_e27671: f64 = (assign29470_e27669 * locals.var_phibswg);
        let assign29470_e27674: f64 = (1.0 - locals.var_bmjswg);
        let assign29470_e27675: f64 = (assign29470_e27671 / assign29470_e27674);
        (assign29470_e27675, 0.0, ((((-((locals.var_arg_dn4 * locals.var_dt3_dvb) + (locals.var_arg * locals.var_dt3_dvb_dn4))) * locals.var_phibswg) + (assign29470_e27669 * locals.var_phibswg_dn4)) / assign29470_e27674), ((((-((locals.var_arg_dn5 * locals.var_dt3_dvb) + (locals.var_arg * locals.var_dt3_dvb_dn5))) * locals.var_phibswg) + (assign29470_e27669 * locals.var_phibswg_dn5)) / assign29470_e27674), ((((-((locals.var_arg_dn6 * locals.var_dt3_dvb) + (locals.var_arg * locals.var_dt3_dvb_dn6))) * locals.var_phibswg) + (assign29470_e27669 * locals.var_phibswg_dn6)) / assign29470_e27674), (((-((locals.var_arg_dn7 * locals.var_dt3_dvb) + (locals.var_arg * locals.var_dt3_dvb_dn7))) * locals.var_phibswg) / assign29470_e27674), (((-((locals.var_arg_dn8 * locals.var_dt3_dvb) + (locals.var_arg * locals.var_dt3_dvb_dn8))) * locals.var_phibswg) / assign29470_e27674), 0.0, 0.0, (((-((locals.var_arg_dn11 * locals.var_dt3_dvb) + (locals.var_arg * locals.var_dt3_dvb_dn11))) * locals.var_phibswg) / assign29470_e27674), (((-((locals.var_arg_dn12 * locals.var_dt3_dvb) + (locals.var_arg * locals.var_dt3_dvb_dn12))) * locals.var_phibswg) / assign29470_e27674),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign29470_e27677;
        locals.var_t3__blk811_dn3 = assign29470_e27677_d_n3;
        locals.var_t3__blk811_dn4 = assign29470_e27677_d_n4;
        locals.var_t3__blk811_dn5 = assign29470_e27677_d_n5;
        locals.var_t3__blk811_dn6 = assign29470_e27677_d_n6;
        locals.var_t3__blk811_dn7 = assign29470_e27677_d_n7;
        locals.var_t3__blk811_dn8 = assign29470_e27677_d_n8;
        locals.var_t3__blk811_dn9 = assign29470_e27677_d_n9;
        locals.var_t3__blk811_dn10 = assign29470_e27677_d_n10;
        locals.var_t3__blk811_dn11 = assign29470_e27677_d_n11;
        locals.var_t3__blk811_dn12 = assign29470_e27677_d_n12;
        locals.var_t3__blk811_rv = 0.0;

        let assign29480_e27680: f64 = if locals.var_vsbs > locals.var_diomax { 1.0 } else { 0.0 };
        locals.var_guard1421 = assign29480_e27680;
        locals.var_guard1421_rv = 0.0;

        let (assign29490_e27693, assign29490_e27693_d_n3, assign29490_e27693_d_n4, assign29490_e27693_d_n5, assign29490_e27693_d_n6, assign29490_e27693_d_n7, assign29490_e27693_d_n8, assign29490_e27693_d_n9, assign29490_e27693_d_n10, assign29490_e27693_d_n11, assign29490_e27693_d_n12,) = {
    if ((locals.var_guard1419 == 0.0) && (locals.var_guard1421 != 0.0)) {
        let assign29490_e27689: f64 = (locals.var_vsbs - locals.var_diomax);
        let assign29490_e27690: f64 = (locals.var_dt3_dvb * assign29490_e27689);
        let assign29490_e27691: f64 = (locals.var_t3__blk811 + assign29490_e27690);
        (assign29490_e27691, locals.var_t3__blk811_dn3, (locals.var_t3__blk811_dn4 + ((locals.var_dt3_dvb_dn4 * assign29490_e27689) + (locals.var_dt3_dvb * (-locals.var_diomax_dn4)))), (locals.var_t3__blk811_dn5 + ((locals.var_dt3_dvb_dn5 * assign29490_e27689) + (locals.var_dt3_dvb * (-locals.var_diomax_dn5)))), (locals.var_t3__blk811_dn6 + ((locals.var_dt3_dvb_dn6 * assign29490_e27689) + (locals.var_dt3_dvb * (-locals.var_diomax_dn6)))), (locals.var_t3__blk811_dn7 + (locals.var_dt3_dvb_dn7 * assign29490_e27689)), (locals.var_t3__blk811_dn8 + ((locals.var_dt3_dvb_dn8 * assign29490_e27689) + (locals.var_dt3_dvb * locals.var_vsbs_dn8))), locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, (locals.var_t3__blk811_dn11 + ((locals.var_dt3_dvb_dn11 * assign29490_e27689) + (locals.var_dt3_dvb * locals.var_vsbs_dn11))), (locals.var_t3__blk811_dn12 + (locals.var_dt3_dvb_dn12 * assign29490_e27689)),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign29490_e27693;
        locals.var_t3__blk811_dn3 = assign29490_e27693_d_n3;
        locals.var_t3__blk811_dn4 = assign29490_e27693_d_n4;
        locals.var_t3__blk811_dn5 = assign29490_e27693_d_n5;
        locals.var_t3__blk811_dn6 = assign29490_e27693_d_n6;
        locals.var_t3__blk811_dn7 = assign29490_e27693_d_n7;
        locals.var_t3__blk811_dn8 = assign29490_e27693_d_n8;
        locals.var_t3__blk811_dn9 = assign29490_e27693_d_n9;
        locals.var_t3__blk811_dn10 = assign29490_e27693_d_n10;
        locals.var_t3__blk811_dn11 = assign29490_e27693_d_n11;
        locals.var_t3__blk811_dn12 = assign29490_e27693_d_n12;
        locals.var_t3__blk811_rv = 0.0;

        let (assign29500_e27706, assign29500_e27706_d_n3, assign29500_e27706_d_n4, assign29500_e27706_d_n5, assign29500_e27706_d_n6, assign29500_e27706_d_n7, assign29500_e27706_d_n8, assign29500_e27706_d_n9, assign29500_e27706_d_n10, assign29500_e27706_d_n11, assign29500_e27706_d_n12,) = {
    if (locals.var_guard1419 == 0.0) {
        let assign29500_e27698: f64 = (locals.var_cjsbs * locals.var_t3__blk811);
        let assign29500_e27701: f64 = (p.p351 * locals.var_ibsdif);
        let assign29500_e27703: f64 = (assign29500_e27701 * p.p3);
        let assign29500_e27704: f64 = (assign29500_e27698 + assign29500_e27703);
        (assign29500_e27704, (((locals.var_cjsbs_dn3 * locals.var_t3__blk811) + (locals.var_cjsbs * locals.var_t3__blk811_dn3)) + ((p.p351 * locals.var_ibsdif_dn3) * p.p3)), (((locals.var_cjsbs_dn4 * locals.var_t3__blk811) + (locals.var_cjsbs * locals.var_t3__blk811_dn4)) + ((p.p351 * locals.var_ibsdif_dn4) * p.p3)), (((locals.var_cjsbs_dn5 * locals.var_t3__blk811) + (locals.var_cjsbs * locals.var_t3__blk811_dn5)) + ((p.p351 * locals.var_ibsdif_dn5) * p.p3)), (((locals.var_cjsbs_dn6 * locals.var_t3__blk811) + (locals.var_cjsbs * locals.var_t3__blk811_dn6)) + ((p.p351 * locals.var_ibsdif_dn6) * p.p3)), (((locals.var_cjsbs_dn7 * locals.var_t3__blk811) + (locals.var_cjsbs * locals.var_t3__blk811_dn7)) + ((p.p351 * locals.var_ibsdif_dn7) * p.p3)), (((locals.var_cjsbs_dn8 * locals.var_t3__blk811) + (locals.var_cjsbs * locals.var_t3__blk811_dn8)) + ((p.p351 * locals.var_ibsdif_dn8) * p.p3)), (((locals.var_cjsbs_dn9 * locals.var_t3__blk811) + (locals.var_cjsbs * locals.var_t3__blk811_dn9)) + ((p.p351 * locals.var_ibsdif_dn9) * p.p3)), (((locals.var_cjsbs_dn10 * locals.var_t3__blk811) + (locals.var_cjsbs * locals.var_t3__blk811_dn10)) + ((p.p351 * locals.var_ibsdif_dn10) * p.p3)), (((locals.var_cjsbs_dn11 * locals.var_t3__blk811) + (locals.var_cjsbs * locals.var_t3__blk811_dn11)) + ((p.p351 * locals.var_ibsdif_dn11) * p.p3)), (((locals.var_cjsbs_dn12 * locals.var_t3__blk811) + (locals.var_cjsbs * locals.var_t3__blk811_dn12)) + ((p.p351 * locals.var_ibsdif_dn12) * p.p3)),)
    } else {
        (locals.var_qjs_1, locals.var_qjs_1_dn3, locals.var_qjs_1_dn4, locals.var_qjs_1_dn5, locals.var_qjs_1_dn6, locals.var_qjs_1_dn7, locals.var_qjs_1_dn8, locals.var_qjs_1_dn9, locals.var_qjs_1_dn10, locals.var_qjs_1_dn11, locals.var_qjs_1_dn12,)
    }
};
        locals.var_qjs_1 = assign29500_e27706;
        locals.var_qjs_1_dn3 = assign29500_e27706_d_n3;
        locals.var_qjs_1_dn4 = assign29500_e27706_d_n4;
        locals.var_qjs_1_dn5 = assign29500_e27706_d_n5;
        locals.var_qjs_1_dn6 = assign29500_e27706_d_n6;
        locals.var_qjs_1_dn7 = assign29500_e27706_d_n7;
        locals.var_qjs_1_dn8 = assign29500_e27706_d_n8;
        locals.var_qjs_1_dn9 = assign29500_e27706_d_n9;
        locals.var_qjs_1_dn10 = assign29500_e27706_d_n10;
        locals.var_qjs_1_dn11 = assign29500_e27706_d_n11;
        locals.var_qjs_1_dn12 = assign29500_e27706_d_n12;
        locals.var_qjs_1_rv = 0.0;

        let (assign29510_e27711, assign29510_e27711_d_n4, assign29510_e27711_d_n5, assign29510_e27711_d_n6,) = {
    if (locals.var_guard1419 == 0.0) {
        (locals.var_b4soigatesidewalljctdpotential, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phibswg, locals.var_phibswg_dn4, locals.var_phibswg_dn5, locals.var_phibswg_dn6,)
    }
};
        locals.var_phibswg = assign29510_e27711;
        locals.var_phibswg_dn4 = assign29510_e27711_d_n4;
        locals.var_phibswg_dn5 = assign29510_e27711_d_n5;
        locals.var_phibswg_dn6 = assign29510_e27711_d_n6;
        locals.var_phibswg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_91(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign29520_e27717,) = {
    if (locals.var_guard1419 == 0.0) {
        let assign29520_e27715: f64 = (-p.p365);
        (assign29520_e27715,)
    } else {
        (locals.var_dphibswg_dt,)
    }
};
        locals.var_dphibswg_dt = assign29520_e27717;
        locals.var_dphibswg_dt_rv = 0.0;

        let (assign29530_e27728, assign29530_e27728_d_n4, assign29530_e27728_d_n5, assign29530_e27728_d_n6,) = {
    if (locals.var_guard1419 == 0.0) {
        let assign29530_e27724: f64 = (locals.var_devtemp - locals.var_tnom);
        let assign29530_e27725: f64 = (locals.var_dphibswg_dt * assign29530_e27724);
        let assign29530_e27726: f64 = (locals.var_phibswg + assign29530_e27725);
        (assign29530_e27726, (locals.var_phibswg_dn4 + (locals.var_dphibswg_dt * locals.var_devtemp_dn4)), (locals.var_phibswg_dn5 + (locals.var_dphibswg_dt * locals.var_devtemp_dn5)), (locals.var_phibswg_dn6 + (locals.var_dphibswg_dt * locals.var_devtemp_dn6)),)
    } else {
        (locals.var_phibswg, locals.var_phibswg_dn4, locals.var_phibswg_dn5, locals.var_phibswg_dn6,)
    }
};
        locals.var_phibswg = assign29530_e27728;
        locals.var_phibswg_dn4 = assign29530_e27728_d_n4;
        locals.var_phibswg_dn5 = assign29530_e27728_d_n5;
        locals.var_phibswg_dn6 = assign29530_e27728_d_n6;
        locals.var_phibswg_rv = 0.0;

        let (assign29540_e27733,) = {
    if (locals.var_guard1419 == 0.0) {
        (p.p184,)
    } else {
        (locals.var_bmjswg,)
    }
};
        locals.var_bmjswg = assign29540_e27733;
        locals.var_bmjswg_rv = 0.0;

        let (assign29550_e27740, assign29550_e27740_d_n4, assign29550_e27740_d_n5, assign29550_e27740_d_n6,) = {
    if (locals.var_guard1419 == 0.0) {
        let assign29550_e27738: f64 = (0.9 * locals.var_phibswg);
        (assign29550_e27738, (0.9 * locals.var_phibswg_dn4), (0.9 * locals.var_phibswg_dn5), (0.9 * locals.var_phibswg_dn6),)
    } else {
        (locals.var_diomax, locals.var_diomax_dn4, locals.var_diomax_dn5, locals.var_diomax_dn6,)
    }
};
        locals.var_diomax = assign29550_e27740;
        locals.var_diomax_dn4 = assign29550_e27740_d_n4;
        locals.var_diomax_dn5 = assign29550_e27740_d_n5;
        locals.var_diomax_dn6 = assign29550_e27740_d_n6;
        locals.var_diomax_rv = 0.0;

        let (assign29560_e27754, assign29560_e27754_d_n4, assign29560_e27754_d_n5, assign29560_e27754_d_n6, assign29560_e27754_d_n7, assign29560_e27754_d_n8, assign29560_e27754_d_n11, assign29560_e27754_d_n12,) = {
    if (locals.var_guard1419 == 0.0) {
        let (assign29560_e27749, assign29560_e27749_d_n4, assign29560_e27749_d_n5, assign29560_e27749_d_n6, assign29560_e27749_d_n7, assign29560_e27749_d_n12,) = {
            if (locals.var_vdbd > locals.var_diomax) {
                (locals.var_diomax, locals.var_diomax_dn4, locals.var_diomax_dn5, locals.var_diomax_dn6, 0.0, 0.0,)
            } else {
                (locals.var_vdbd, 0.0, 0.0, 0.0, locals.var_vdbd_dn7, locals.var_vdbd_dn12,)
            }
        };
        let assign29560_e27751: f64 = (assign29560_e27749 / locals.var_phibswg);
        let assign29560_e27752: f64 = (1.0 - assign29560_e27751);
        (assign29560_e27752, (-(((assign29560_e27749_d_n4 * locals.var_phibswg) - (assign29560_e27749 * locals.var_phibswg_dn4)) / (locals.var_phibswg * locals.var_phibswg))), (-(((assign29560_e27749_d_n5 * locals.var_phibswg) - (assign29560_e27749 * locals.var_phibswg_dn5)) / (locals.var_phibswg * locals.var_phibswg))), (-(((assign29560_e27749_d_n6 * locals.var_phibswg) - (assign29560_e27749 * locals.var_phibswg_dn6)) / (locals.var_phibswg * locals.var_phibswg))), (-(assign29560_e27749_d_n7 / locals.var_phibswg)), 0.0, 0.0, (-(assign29560_e27749_d_n12 / locals.var_phibswg)),)
    } else {
        (locals.var_arg, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn11, locals.var_arg_dn12,)
    }
};
        locals.var_arg = assign29560_e27754;
        locals.var_arg_dn4 = assign29560_e27754_d_n4;
        locals.var_arg_dn5 = assign29560_e27754_d_n5;
        locals.var_arg_dn6 = assign29560_e27754_d_n6;
        locals.var_arg_dn7 = assign29560_e27754_d_n7;
        locals.var_arg_dn8 = assign29560_e27754_d_n8;
        locals.var_arg_dn11 = assign29560_e27754_d_n11;
        locals.var_arg_dn12 = assign29560_e27754_d_n12;
        locals.var_arg_rv = 0.0;

        let assign29570_e27757: f64 = if locals.var_bmjswg == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1422 = assign29570_e27757;
        locals.var_guard1422_rv = 0.0;

        let (assign29580_e27767, assign29580_e27767_d_n4, assign29580_e27767_d_n5, assign29580_e27767_d_n6, assign29580_e27767_d_n7, assign29580_e27767_d_n8, assign29580_e27767_d_n11, assign29580_e27767_d_n12,) = {
    if ((locals.var_guard1419 == 0.0) && (locals.var_guard1422 != 0.0)) {
        let assign29580_e27764: f64 = (locals.var_arg).sqrt();
        let assign29580_e27765: f64 = (1.0 / assign29580_e27764);
        (assign29580_e27765, (-((locals.var_arg_dn4 / (2.0 * assign29580_e27764)) / (assign29580_e27764 * assign29580_e27764))), (-((locals.var_arg_dn5 / (2.0 * assign29580_e27764)) / (assign29580_e27764 * assign29580_e27764))), (-((locals.var_arg_dn6 / (2.0 * assign29580_e27764)) / (assign29580_e27764 * assign29580_e27764))), (-((locals.var_arg_dn7 / (2.0 * assign29580_e27764)) / (assign29580_e27764 * assign29580_e27764))), (-((locals.var_arg_dn8 / (2.0 * assign29580_e27764)) / (assign29580_e27764 * assign29580_e27764))), (-((locals.var_arg_dn11 / (2.0 * assign29580_e27764)) / (assign29580_e27764 * assign29580_e27764))), (-((locals.var_arg_dn12 / (2.0 * assign29580_e27764)) / (assign29580_e27764 * assign29580_e27764))),)
    } else {
        (locals.var_dt3_dvb, locals.var_dt3_dvb_dn4, locals.var_dt3_dvb_dn5, locals.var_dt3_dvb_dn6, locals.var_dt3_dvb_dn7, locals.var_dt3_dvb_dn8, locals.var_dt3_dvb_dn11, locals.var_dt3_dvb_dn12,)
    }
};
        locals.var_dt3_dvb = assign29580_e27767;
        locals.var_dt3_dvb_dn4 = assign29580_e27767_d_n4;
        locals.var_dt3_dvb_dn5 = assign29580_e27767_d_n5;
        locals.var_dt3_dvb_dn6 = assign29580_e27767_d_n6;
        locals.var_dt3_dvb_dn7 = assign29580_e27767_d_n7;
        locals.var_dt3_dvb_dn8 = assign29580_e27767_d_n8;
        locals.var_dt3_dvb_dn11 = assign29580_e27767_d_n11;
        locals.var_dt3_dvb_dn12 = assign29580_e27767_d_n12;
        locals.var_dt3_dvb_rv = 0.0;

        let (assign29590_e27786, assign29590_e27786_d_n4, assign29590_e27786_d_n5, assign29590_e27786_d_n6, assign29590_e27786_d_n7, assign29590_e27786_d_n8, assign29590_e27786_d_n11, assign29590_e27786_d_n12,) = {
    if ((locals.var_guard1419 == 0.0) && (locals.var_guard1422 == 0.0)) {
        let assign29590_e27774: f64 = (-locals.var_bmjswg);
        let (assign29590_e27782, assign29590_e27782_d_n4, assign29590_e27782_d_n5, assign29590_e27782_d_n6, assign29590_e27782_d_n7, assign29590_e27782_d_n8, assign29590_e27782_d_n11, assign29590_e27782_d_n12,) = {
            if (locals.var_arg > 1e-38) {
                let assign29590_e27779: f64 = (locals.var_arg).ln();
                (assign29590_e27779, (locals.var_arg_dn4 / locals.var_arg), (locals.var_arg_dn5 / locals.var_arg), (locals.var_arg_dn6 / locals.var_arg), (locals.var_arg_dn7 / locals.var_arg), (locals.var_arg_dn8 / locals.var_arg), (locals.var_arg_dn11 / locals.var_arg), (locals.var_arg_dn12 / locals.var_arg),)
            } else {
                let assign29590_e27781: f64 = (-87.49823353377374);
                (assign29590_e27781, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign29590_e27783: f64 = (assign29590_e27774 * assign29590_e27782);
        let assign29590_e27784: f64 = (assign29590_e27783).exp();
        (assign29590_e27784, (assign29590_e27784 * (assign29590_e27774 * assign29590_e27782_d_n4)), (assign29590_e27784 * (assign29590_e27774 * assign29590_e27782_d_n5)), (assign29590_e27784 * (assign29590_e27774 * assign29590_e27782_d_n6)), (assign29590_e27784 * (assign29590_e27774 * assign29590_e27782_d_n7)), (assign29590_e27784 * (assign29590_e27774 * assign29590_e27782_d_n8)), (assign29590_e27784 * (assign29590_e27774 * assign29590_e27782_d_n11)), (assign29590_e27784 * (assign29590_e27774 * assign29590_e27782_d_n12)),)
    } else {
        (locals.var_dt3_dvb, locals.var_dt3_dvb_dn4, locals.var_dt3_dvb_dn5, locals.var_dt3_dvb_dn6, locals.var_dt3_dvb_dn7, locals.var_dt3_dvb_dn8, locals.var_dt3_dvb_dn11, locals.var_dt3_dvb_dn12,)
    }
};
        locals.var_dt3_dvb = assign29590_e27786;
        locals.var_dt3_dvb_dn4 = assign29590_e27786_d_n4;
        locals.var_dt3_dvb_dn5 = assign29590_e27786_d_n5;
        locals.var_dt3_dvb_dn6 = assign29590_e27786_d_n6;
        locals.var_dt3_dvb_dn7 = assign29590_e27786_d_n7;
        locals.var_dt3_dvb_dn8 = assign29590_e27786_d_n8;
        locals.var_dt3_dvb_dn11 = assign29590_e27786_d_n11;
        locals.var_dt3_dvb_dn12 = assign29590_e27786_d_n12;
        locals.var_dt3_dvb_rv = 0.0;

        let (assign29600_e27801, assign29600_e27801_d_n3, assign29600_e27801_d_n4, assign29600_e27801_d_n5, assign29600_e27801_d_n6, assign29600_e27801_d_n7, assign29600_e27801_d_n8, assign29600_e27801_d_n9, assign29600_e27801_d_n10, assign29600_e27801_d_n11, assign29600_e27801_d_n12,) = {
    if (locals.var_guard1419 == 0.0) {
        let assign29600_e27792: f64 = (locals.var_arg * locals.var_dt3_dvb);
        let assign29600_e27793: f64 = (1.0 - assign29600_e27792);
        let assign29600_e27795: f64 = (assign29600_e27793 * locals.var_phibswg);
        let assign29600_e27798: f64 = (1.0 - locals.var_bmjswg);
        let assign29600_e27799: f64 = (assign29600_e27795 / assign29600_e27798);
        (assign29600_e27799, 0.0, ((((-((locals.var_arg_dn4 * locals.var_dt3_dvb) + (locals.var_arg * locals.var_dt3_dvb_dn4))) * locals.var_phibswg) + (assign29600_e27793 * locals.var_phibswg_dn4)) / assign29600_e27798), ((((-((locals.var_arg_dn5 * locals.var_dt3_dvb) + (locals.var_arg * locals.var_dt3_dvb_dn5))) * locals.var_phibswg) + (assign29600_e27793 * locals.var_phibswg_dn5)) / assign29600_e27798), ((((-((locals.var_arg_dn6 * locals.var_dt3_dvb) + (locals.var_arg * locals.var_dt3_dvb_dn6))) * locals.var_phibswg) + (assign29600_e27793 * locals.var_phibswg_dn6)) / assign29600_e27798), (((-((locals.var_arg_dn7 * locals.var_dt3_dvb) + (locals.var_arg * locals.var_dt3_dvb_dn7))) * locals.var_phibswg) / assign29600_e27798), (((-((locals.var_arg_dn8 * locals.var_dt3_dvb) + (locals.var_arg * locals.var_dt3_dvb_dn8))) * locals.var_phibswg) / assign29600_e27798), 0.0, 0.0, (((-((locals.var_arg_dn11 * locals.var_dt3_dvb) + (locals.var_arg * locals.var_dt3_dvb_dn11))) * locals.var_phibswg) / assign29600_e27798), (((-((locals.var_arg_dn12 * locals.var_dt3_dvb) + (locals.var_arg * locals.var_dt3_dvb_dn12))) * locals.var_phibswg) / assign29600_e27798),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign29600_e27801;
        locals.var_t3__blk811_dn3 = assign29600_e27801_d_n3;
        locals.var_t3__blk811_dn4 = assign29600_e27801_d_n4;
        locals.var_t3__blk811_dn5 = assign29600_e27801_d_n5;
        locals.var_t3__blk811_dn6 = assign29600_e27801_d_n6;
        locals.var_t3__blk811_dn7 = assign29600_e27801_d_n7;
        locals.var_t3__blk811_dn8 = assign29600_e27801_d_n8;
        locals.var_t3__blk811_dn9 = assign29600_e27801_d_n9;
        locals.var_t3__blk811_dn10 = assign29600_e27801_d_n10;
        locals.var_t3__blk811_dn11 = assign29600_e27801_d_n11;
        locals.var_t3__blk811_dn12 = assign29600_e27801_d_n12;
        locals.var_t3__blk811_rv = 0.0;

        let assign29610_e27804: f64 = if locals.var_vdbd > locals.var_diomax { 1.0 } else { 0.0 };
        locals.var_guard1423 = assign29610_e27804;
        locals.var_guard1423_rv = 0.0;

        let (assign29620_e27817, assign29620_e27817_d_n3, assign29620_e27817_d_n4, assign29620_e27817_d_n5, assign29620_e27817_d_n6, assign29620_e27817_d_n7, assign29620_e27817_d_n8, assign29620_e27817_d_n9, assign29620_e27817_d_n10, assign29620_e27817_d_n11, assign29620_e27817_d_n12,) = {
    if ((locals.var_guard1419 == 0.0) && (locals.var_guard1423 != 0.0)) {
        let assign29620_e27813: f64 = (locals.var_vdbd - locals.var_diomax);
        let assign29620_e27814: f64 = (locals.var_dt3_dvb * assign29620_e27813);
        let assign29620_e27815: f64 = (locals.var_t3__blk811 + assign29620_e27814);
        (assign29620_e27815, locals.var_t3__blk811_dn3, (locals.var_t3__blk811_dn4 + ((locals.var_dt3_dvb_dn4 * assign29620_e27813) + (locals.var_dt3_dvb * (-locals.var_diomax_dn4)))), (locals.var_t3__blk811_dn5 + ((locals.var_dt3_dvb_dn5 * assign29620_e27813) + (locals.var_dt3_dvb * (-locals.var_diomax_dn5)))), (locals.var_t3__blk811_dn6 + ((locals.var_dt3_dvb_dn6 * assign29620_e27813) + (locals.var_dt3_dvb * (-locals.var_diomax_dn6)))), (locals.var_t3__blk811_dn7 + ((locals.var_dt3_dvb_dn7 * assign29620_e27813) + (locals.var_dt3_dvb * locals.var_vdbd_dn7))), (locals.var_t3__blk811_dn8 + (locals.var_dt3_dvb_dn8 * assign29620_e27813)), locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, (locals.var_t3__blk811_dn11 + (locals.var_dt3_dvb_dn11 * assign29620_e27813)), (locals.var_t3__blk811_dn12 + ((locals.var_dt3_dvb_dn12 * assign29620_e27813) + (locals.var_dt3_dvb * locals.var_vdbd_dn12))),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign29620_e27817;
        locals.var_t3__blk811_dn3 = assign29620_e27817_d_n3;
        locals.var_t3__blk811_dn4 = assign29620_e27817_d_n4;
        locals.var_t3__blk811_dn5 = assign29620_e27817_d_n5;
        locals.var_t3__blk811_dn6 = assign29620_e27817_d_n6;
        locals.var_t3__blk811_dn7 = assign29620_e27817_d_n7;
        locals.var_t3__blk811_dn8 = assign29620_e27817_d_n8;
        locals.var_t3__blk811_dn9 = assign29620_e27817_d_n9;
        locals.var_t3__blk811_dn10 = assign29620_e27817_d_n10;
        locals.var_t3__blk811_dn11 = assign29620_e27817_d_n11;
        locals.var_t3__blk811_dn12 = assign29620_e27817_d_n12;
        locals.var_t3__blk811_rv = 0.0;

        let (assign29630_e27830, assign29630_e27830_d_n3, assign29630_e27830_d_n4, assign29630_e27830_d_n5, assign29630_e27830_d_n6, assign29630_e27830_d_n7, assign29630_e27830_d_n8, assign29630_e27830_d_n9, assign29630_e27830_d_n10, assign29630_e27830_d_n11, assign29630_e27830_d_n12,) = {
    if (locals.var_guard1419 == 0.0) {
        let assign29630_e27822: f64 = (locals.var_cjdbs * locals.var_t3__blk811);
        let assign29630_e27825: f64 = (p.p351 * locals.var_ibddif);
        let assign29630_e27827: f64 = (assign29630_e27825 * p.p3);
        let assign29630_e27828: f64 = (assign29630_e27822 + assign29630_e27827);
        (assign29630_e27828, (((locals.var_cjdbs_dn3 * locals.var_t3__blk811) + (locals.var_cjdbs * locals.var_t3__blk811_dn3)) + ((p.p351 * locals.var_ibddif_dn3) * p.p3)), (((locals.var_cjdbs_dn4 * locals.var_t3__blk811) + (locals.var_cjdbs * locals.var_t3__blk811_dn4)) + ((p.p351 * locals.var_ibddif_dn4) * p.p3)), (((locals.var_cjdbs_dn5 * locals.var_t3__blk811) + (locals.var_cjdbs * locals.var_t3__blk811_dn5)) + ((p.p351 * locals.var_ibddif_dn5) * p.p3)), (((locals.var_cjdbs_dn6 * locals.var_t3__blk811) + (locals.var_cjdbs * locals.var_t3__blk811_dn6)) + ((p.p351 * locals.var_ibddif_dn6) * p.p3)), (((locals.var_cjdbs_dn7 * locals.var_t3__blk811) + (locals.var_cjdbs * locals.var_t3__blk811_dn7)) + ((p.p351 * locals.var_ibddif_dn7) * p.p3)), (((locals.var_cjdbs_dn8 * locals.var_t3__blk811) + (locals.var_cjdbs * locals.var_t3__blk811_dn8)) + ((p.p351 * locals.var_ibddif_dn8) * p.p3)), (((locals.var_cjdbs_dn9 * locals.var_t3__blk811) + (locals.var_cjdbs * locals.var_t3__blk811_dn9)) + ((p.p351 * locals.var_ibddif_dn9) * p.p3)), (((locals.var_cjdbs_dn10 * locals.var_t3__blk811) + (locals.var_cjdbs * locals.var_t3__blk811_dn10)) + ((p.p351 * locals.var_ibddif_dn10) * p.p3)), (((locals.var_cjdbs_dn11 * locals.var_t3__blk811) + (locals.var_cjdbs * locals.var_t3__blk811_dn11)) + ((p.p351 * locals.var_ibddif_dn11) * p.p3)), (((locals.var_cjdbs_dn12 * locals.var_t3__blk811) + (locals.var_cjdbs * locals.var_t3__blk811_dn12)) + ((p.p351 * locals.var_ibddif_dn12) * p.p3)),)
    } else {
        (locals.var_qjd_1, locals.var_qjd_1_dn3, locals.var_qjd_1_dn4, locals.var_qjd_1_dn5, locals.var_qjd_1_dn6, locals.var_qjd_1_dn7, locals.var_qjd_1_dn8, locals.var_qjd_1_dn9, locals.var_qjd_1_dn10, locals.var_qjd_1_dn11, locals.var_qjd_1_dn12,)
    }
};
        locals.var_qjd_1 = assign29630_e27830;
        locals.var_qjd_1_dn3 = assign29630_e27830_d_n3;
        locals.var_qjd_1_dn4 = assign29630_e27830_d_n4;
        locals.var_qjd_1_dn5 = assign29630_e27830_d_n5;
        locals.var_qjd_1_dn6 = assign29630_e27830_d_n6;
        locals.var_qjd_1_dn7 = assign29630_e27830_d_n7;
        locals.var_qjd_1_dn8 = assign29630_e27830_d_n8;
        locals.var_qjd_1_dn9 = assign29630_e27830_d_n9;
        locals.var_qjd_1_dn10 = assign29630_e27830_d_n10;
        locals.var_qjd_1_dn11 = assign29630_e27830_d_n11;
        locals.var_qjd_1_dn12 = assign29630_e27830_d_n12;
        locals.var_qjd_1_rv = 0.0;

        let assign29640_e27832: f64 = (-p.p37);
        let assign29640_e27834: f64 = (assign29640_e27832 * locals.var_ves);
        locals.var_t10__blk818 = assign29640_e27834;
        locals.var_t10__blk818_dn3 = (assign29640_e27832 * locals.var_ves_dn3);
        locals.var_t10__blk818_dn4 = 0.0;
        locals.var_t10__blk818_dn5 = 0.0;
        locals.var_t10__blk818_dn6 = 0.0;
        locals.var_t10__blk818_dn7 = 0.0;
        locals.var_t10__blk818_dn8 = (assign29640_e27832 * locals.var_ves_dn8);
        locals.var_t10__blk818_dn9 = 0.0;
        locals.var_t10__blk818_dn10 = 0.0;
        locals.var_t10__blk818_dn11 = 0.0;
        locals.var_t10__blk818_dn12 = 0.0;
        locals.var_t10__blk818_rv = 0.0;

        let assign29650_e27838: f64 = (locals.var_vds - locals.var_ves);
        let assign29650_e27839: f64 = (p.p37 * assign29650_e27838);
        locals.var_t11 = assign29650_e27839;
        locals.var_t11_dn3 = (p.p37 * (-locals.var_ves_dn3));
        locals.var_t11_dn4 = 0.0;
        locals.var_t11_dn5 = 0.0;
        locals.var_t11_dn6 = 0.0;
        locals.var_t11_dn7 = (p.p37 * locals.var_vds_dn7);
        locals.var_t11_dn8 = (p.p37 * (locals.var_vds_dn8 - locals.var_ves_dn8));
        locals.var_t11_dn9 = 0.0;
        locals.var_t11_dn10 = 0.0;
        locals.var_t11_dn11 = 0.0;
        locals.var_t11_dn12 = 0.0;
        locals.var_t11_rv = 0.0;

        let assign29660_e27842: f64 = if locals.var_b4soicsdmin != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1424 = assign29660_e27842;
        locals.var_guard1424_rv = 0.0;

        let assign29670_e27857: f64 = if (((locals.var_pparam_b4soinsub > 0.0) && (p.p37 > 0.0)) || ((locals.var_pparam_b4soinsub < 0.0) && (p.p37 < 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard1425 = assign29670_e27857;
        locals.var_guard1425_rv = 0.0;

        let assign29680_e27860: f64 = if locals.var_t10__blk818 < locals.var_pparam_b4soivsdfb { 1.0 } else { 0.0 };
        locals.var_guard1426 = assign29680_e27860;
        locals.var_guard1426_rv = 0.0;

        let (assign29690_e27872, assign29690_e27872_d_n3, assign29690_e27872_d_n4, assign29690_e27872_d_n5, assign29690_e27872_d_n6, assign29690_e27872_d_n7, assign29690_e27872_d_n8, assign29690_e27872_d_n9, assign29690_e27872_d_n10, assign29690_e27872_d_n11, assign29690_e27872_d_n12,) = {
    if (((locals.var_guard1424 != 0.0) && (locals.var_guard1425 != 0.0)) && (locals.var_guard1426 != 0.0)) {
        let assign29690_e27869: f64 = (locals.var_t10__blk818 - locals.var_pparam_b4soivsdfb);
        let assign29690_e27870: f64 = (locals.var_b4soicsbox * assign29690_e27869);
        (assign29690_e27870, (locals.var_b4soicsbox * (locals.var_t10__blk818_dn3 - locals.var_pparam_b4soivsdfb_dn3)), (locals.var_b4soicsbox * (locals.var_t10__blk818_dn4 - locals.var_pparam_b4soivsdfb_dn4)), (locals.var_b4soicsbox * (locals.var_t10__blk818_dn5 - locals.var_pparam_b4soivsdfb_dn5)), (locals.var_b4soicsbox * (locals.var_t10__blk818_dn6 - locals.var_pparam_b4soivsdfb_dn6)), (locals.var_b4soicsbox * (locals.var_t10__blk818_dn7 - locals.var_pparam_b4soivsdfb_dn7)), (locals.var_b4soicsbox * (locals.var_t10__blk818_dn8 - locals.var_pparam_b4soivsdfb_dn8)), (locals.var_b4soicsbox * (locals.var_t10__blk818_dn9 - locals.var_pparam_b4soivsdfb_dn9)), (locals.var_b4soicsbox * (locals.var_t10__blk818_dn10 - locals.var_pparam_b4soivsdfb_dn10)), (locals.var_b4soicsbox * (locals.var_t10__blk818_dn11 - locals.var_pparam_b4soivsdfb_dn11)), (locals.var_b4soicsbox * (locals.var_t10__blk818_dn12 - locals.var_pparam_b4soivsdfb_dn12)),)
    } else {
        (locals.var_b4soiqse, locals.var_b4soiqse_dn3, locals.var_b4soiqse_dn4, locals.var_b4soiqse_dn5, locals.var_b4soiqse_dn6, locals.var_b4soiqse_dn7, locals.var_b4soiqse_dn8, locals.var_b4soiqse_dn9, locals.var_b4soiqse_dn10, locals.var_b4soiqse_dn11, locals.var_b4soiqse_dn12,)
    }
};
        locals.var_b4soiqse = assign29690_e27872;
        locals.var_b4soiqse_dn3 = assign29690_e27872_d_n3;
        locals.var_b4soiqse_dn4 = assign29690_e27872_d_n4;
        locals.var_b4soiqse_dn5 = assign29690_e27872_d_n5;
        locals.var_b4soiqse_dn6 = assign29690_e27872_d_n6;
        locals.var_b4soiqse_dn7 = assign29690_e27872_d_n7;
        locals.var_b4soiqse_dn8 = assign29690_e27872_d_n8;
        locals.var_b4soiqse_dn9 = assign29690_e27872_d_n9;
        locals.var_b4soiqse_dn10 = assign29690_e27872_d_n10;
        locals.var_b4soiqse_dn11 = assign29690_e27872_d_n11;
        locals.var_b4soiqse_dn12 = assign29690_e27872_d_n12;
        locals.var_b4soiqse_rv = 0.0;

        let assign29700_e27875: f64 = if locals.var_t10__blk818 < locals.var_pparam_b4soisdt1 { 1.0 } else { 0.0 };
        locals.var_guard1427 = assign29700_e27875;
        locals.var_guard1427_rv = 0.0;

        let (assign29710_e27888, assign29710_e27888_d_n3, assign29710_e27888_d_n4, assign29710_e27888_d_n5, assign29710_e27888_d_n6, assign29710_e27888_d_n7, assign29710_e27888_d_n8, assign29710_e27888_d_n9, assign29710_e27888_d_n10, assign29710_e27888_d_n11, assign29710_e27888_d_n12,) = {
    if ((((locals.var_guard1424 != 0.0) && (locals.var_guard1425 != 0.0)) && (locals.var_guard1426 == 0.0)) && (locals.var_guard1427 != 0.0)) {
        let assign29710_e27886: f64 = (locals.var_t10__blk818 - locals.var_pparam_b4soivsdfb);
        (assign29710_e27886, (locals.var_t10__blk818_dn3 - locals.var_pparam_b4soivsdfb_dn3), (locals.var_t10__blk818_dn4 - locals.var_pparam_b4soivsdfb_dn4), (locals.var_t10__blk818_dn5 - locals.var_pparam_b4soivsdfb_dn5), (locals.var_t10__blk818_dn6 - locals.var_pparam_b4soivsdfb_dn6), (locals.var_t10__blk818_dn7 - locals.var_pparam_b4soivsdfb_dn7), (locals.var_t10__blk818_dn8 - locals.var_pparam_b4soivsdfb_dn8), (locals.var_t10__blk818_dn9 - locals.var_pparam_b4soivsdfb_dn9), (locals.var_t10__blk818_dn10 - locals.var_pparam_b4soivsdfb_dn10), (locals.var_t10__blk818_dn11 - locals.var_pparam_b4soivsdfb_dn11), (locals.var_t10__blk818_dn12 - locals.var_pparam_b4soivsdfb_dn12),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign29710_e27888;
        locals.var_t0__blk808_dn3 = assign29710_e27888_d_n3;
        locals.var_t0__blk808_dn4 = assign29710_e27888_d_n4;
        locals.var_t0__blk808_dn5 = assign29710_e27888_d_n5;
        locals.var_t0__blk808_dn6 = assign29710_e27888_d_n6;
        locals.var_t0__blk808_dn7 = assign29710_e27888_d_n7;
        locals.var_t0__blk808_dn8 = assign29710_e27888_d_n8;
        locals.var_t0__blk808_dn9 = assign29710_e27888_d_n9;
        locals.var_t0__blk808_dn10 = assign29710_e27888_d_n10;
        locals.var_t0__blk808_dn11 = assign29710_e27888_d_n11;
        locals.var_t0__blk808_dn12 = assign29710_e27888_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign29720_e27901, assign29720_e27901_d_n3, assign29720_e27901_d_n4, assign29720_e27901_d_n5, assign29720_e27901_d_n6, assign29720_e27901_d_n7, assign29720_e27901_d_n8, assign29720_e27901_d_n9, assign29720_e27901_d_n10, assign29720_e27901_d_n11, assign29720_e27901_d_n12,) = {
    if ((((locals.var_guard1424 != 0.0) && (locals.var_guard1425 != 0.0)) && (locals.var_guard1426 == 0.0)) && (locals.var_guard1427 != 0.0)) {
        let assign29720_e27899: f64 = (locals.var_t0__blk808 * locals.var_t0__blk808);
        (assign29720_e27899, ((locals.var_t0__blk808_dn3 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn3)), ((locals.var_t0__blk808_dn4 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn4)), ((locals.var_t0__blk808_dn5 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn5)), ((locals.var_t0__blk808_dn6 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn6)), ((locals.var_t0__blk808_dn7 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn7)), ((locals.var_t0__blk808_dn8 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn8)), ((locals.var_t0__blk808_dn9 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn9)), ((locals.var_t0__blk808_dn10 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn10)), ((locals.var_t0__blk808_dn11 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn11)), ((locals.var_t0__blk808_dn12 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn12)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign29720_e27901;
        locals.var_t1__blk809_dn3 = assign29720_e27901_d_n3;
        locals.var_t1__blk809_dn4 = assign29720_e27901_d_n4;
        locals.var_t1__blk809_dn5 = assign29720_e27901_d_n5;
        locals.var_t1__blk809_dn6 = assign29720_e27901_d_n6;
        locals.var_t1__blk809_dn7 = assign29720_e27901_d_n7;
        locals.var_t1__blk809_dn8 = assign29720_e27901_d_n8;
        locals.var_t1__blk809_dn9 = assign29720_e27901_d_n9;
        locals.var_t1__blk809_dn10 = assign29720_e27901_d_n10;
        locals.var_t1__blk809_dn11 = assign29720_e27901_d_n11;
        locals.var_t1__blk809_dn12 = assign29720_e27901_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign29730_e27920, assign29730_e27920_d_n3, assign29730_e27920_d_n4, assign29730_e27920_d_n5, assign29730_e27920_d_n6, assign29730_e27920_d_n7, assign29730_e27920_d_n8, assign29730_e27920_d_n9, assign29730_e27920_d_n10, assign29730_e27920_d_n11, assign29730_e27920_d_n12,) = {
    if ((((locals.var_guard1424 != 0.0) && (locals.var_guard1425 != 0.0)) && (locals.var_guard1426 == 0.0)) && (locals.var_guard1427 != 0.0)) {
        let assign29730_e27914: f64 = (locals.var_pparam_b4soist2 / 3.0);
        let assign29730_e27916: f64 = (assign29730_e27914 * locals.var_t1__blk809);
        let assign29730_e27917: f64 = (locals.var_b4soicsbox - assign29730_e27916);
        let assign29730_e27918: f64 = (locals.var_t0__blk808 * assign29730_e27917);
        (assign29730_e27918, ((locals.var_t0__blk808_dn3 * assign29730_e27917) + (locals.var_t0__blk808 * (-(((locals.var_pparam_b4soist2_dn3 / 3.0) * locals.var_t1__blk809) + (assign29730_e27914 * locals.var_t1__blk809_dn3))))), ((locals.var_t0__blk808_dn4 * assign29730_e27917) + (locals.var_t0__blk808 * (-(((locals.var_pparam_b4soist2_dn4 / 3.0) * locals.var_t1__blk809) + (assign29730_e27914 * locals.var_t1__blk809_dn4))))), ((locals.var_t0__blk808_dn5 * assign29730_e27917) + (locals.var_t0__blk808 * (-(((locals.var_pparam_b4soist2_dn5 / 3.0) * locals.var_t1__blk809) + (assign29730_e27914 * locals.var_t1__blk809_dn5))))), ((locals.var_t0__blk808_dn6 * assign29730_e27917) + (locals.var_t0__blk808 * (-(((locals.var_pparam_b4soist2_dn6 / 3.0) * locals.var_t1__blk809) + (assign29730_e27914 * locals.var_t1__blk809_dn6))))), ((locals.var_t0__blk808_dn7 * assign29730_e27917) + (locals.var_t0__blk808 * (-(((locals.var_pparam_b4soist2_dn7 / 3.0) * locals.var_t1__blk809) + (assign29730_e27914 * locals.var_t1__blk809_dn7))))), ((locals.var_t0__blk808_dn8 * assign29730_e27917) + (locals.var_t0__blk808 * (-(((locals.var_pparam_b4soist2_dn8 / 3.0) * locals.var_t1__blk809) + (assign29730_e27914 * locals.var_t1__blk809_dn8))))), ((locals.var_t0__blk808_dn9 * assign29730_e27917) + (locals.var_t0__blk808 * (-(((locals.var_pparam_b4soist2_dn9 / 3.0) * locals.var_t1__blk809) + (assign29730_e27914 * locals.var_t1__blk809_dn9))))), ((locals.var_t0__blk808_dn10 * assign29730_e27917) + (locals.var_t0__blk808 * (-(((locals.var_pparam_b4soist2_dn10 / 3.0) * locals.var_t1__blk809) + (assign29730_e27914 * locals.var_t1__blk809_dn10))))), ((locals.var_t0__blk808_dn11 * assign29730_e27917) + (locals.var_t0__blk808 * (-(((locals.var_pparam_b4soist2_dn11 / 3.0) * locals.var_t1__blk809) + (assign29730_e27914 * locals.var_t1__blk809_dn11))))), ((locals.var_t0__blk808_dn12 * assign29730_e27917) + (locals.var_t0__blk808 * (-(((locals.var_pparam_b4soist2_dn12 / 3.0) * locals.var_t1__blk809) + (assign29730_e27914 * locals.var_t1__blk809_dn12))))),)
    } else {
        (locals.var_b4soiqse, locals.var_b4soiqse_dn3, locals.var_b4soiqse_dn4, locals.var_b4soiqse_dn5, locals.var_b4soiqse_dn6, locals.var_b4soiqse_dn7, locals.var_b4soiqse_dn8, locals.var_b4soiqse_dn9, locals.var_b4soiqse_dn10, locals.var_b4soiqse_dn11, locals.var_b4soiqse_dn12,)
    }
};
        locals.var_b4soiqse = assign29730_e27920;
        locals.var_b4soiqse_dn3 = assign29730_e27920_d_n3;
        locals.var_b4soiqse_dn4 = assign29730_e27920_d_n4;
        locals.var_b4soiqse_dn5 = assign29730_e27920_d_n5;
        locals.var_b4soiqse_dn6 = assign29730_e27920_d_n6;
        locals.var_b4soiqse_dn7 = assign29730_e27920_d_n7;
        locals.var_b4soiqse_dn8 = assign29730_e27920_d_n8;
        locals.var_b4soiqse_dn9 = assign29730_e27920_d_n9;
        locals.var_b4soiqse_dn10 = assign29730_e27920_d_n10;
        locals.var_b4soiqse_dn11 = assign29730_e27920_d_n11;
        locals.var_b4soiqse_dn12 = assign29730_e27920_d_n12;
        locals.var_b4soiqse_rv = 0.0;

        let assign29740_e27923: f64 = if locals.var_t10__blk818 < locals.var_pparam_b4soivsdth { 1.0 } else { 0.0 };
        locals.var_guard1428 = assign29740_e27923;
        locals.var_guard1428_rv = 0.0;

        let (assign29750_e27939, assign29750_e27939_d_n3, assign29750_e27939_d_n4, assign29750_e27939_d_n5, assign29750_e27939_d_n6, assign29750_e27939_d_n7, assign29750_e27939_d_n8, assign29750_e27939_d_n9, assign29750_e27939_d_n10, assign29750_e27939_d_n11, assign29750_e27939_d_n12,) = {
    if (((((locals.var_guard1424 != 0.0) && (locals.var_guard1425 != 0.0)) && (locals.var_guard1426 == 0.0)) && (locals.var_guard1427 == 0.0)) && (locals.var_guard1428 != 0.0)) {
        let assign29750_e27937: f64 = (locals.var_t10__blk818 - locals.var_pparam_b4soivsdth);
        (assign29750_e27937, (locals.var_t10__blk818_dn3 - locals.var_pparam_b4soivsdth_dn3), (locals.var_t10__blk818_dn4 - locals.var_pparam_b4soivsdth_dn4), (locals.var_t10__blk818_dn5 - locals.var_pparam_b4soivsdth_dn5), (locals.var_t10__blk818_dn6 - locals.var_pparam_b4soivsdth_dn6), (locals.var_t10__blk818_dn7 - locals.var_pparam_b4soivsdth_dn7), (locals.var_t10__blk818_dn8 - locals.var_pparam_b4soivsdth_dn8), (locals.var_t10__blk818_dn9 - locals.var_pparam_b4soivsdth_dn9), (locals.var_t10__blk818_dn10 - locals.var_pparam_b4soivsdth_dn10), (locals.var_t10__blk818_dn11 - locals.var_pparam_b4soivsdth_dn11), (locals.var_t10__blk818_dn12 - locals.var_pparam_b4soivsdth_dn12),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign29750_e27939;
        locals.var_t0__blk808_dn3 = assign29750_e27939_d_n3;
        locals.var_t0__blk808_dn4 = assign29750_e27939_d_n4;
        locals.var_t0__blk808_dn5 = assign29750_e27939_d_n5;
        locals.var_t0__blk808_dn6 = assign29750_e27939_d_n6;
        locals.var_t0__blk808_dn7 = assign29750_e27939_d_n7;
        locals.var_t0__blk808_dn8 = assign29750_e27939_d_n8;
        locals.var_t0__blk808_dn9 = assign29750_e27939_d_n9;
        locals.var_t0__blk808_dn10 = assign29750_e27939_d_n10;
        locals.var_t0__blk808_dn11 = assign29750_e27939_d_n11;
        locals.var_t0__blk808_dn12 = assign29750_e27939_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign29760_e27955, assign29760_e27955_d_n3, assign29760_e27955_d_n4, assign29760_e27955_d_n5, assign29760_e27955_d_n6, assign29760_e27955_d_n7, assign29760_e27955_d_n8, assign29760_e27955_d_n9, assign29760_e27955_d_n10, assign29760_e27955_d_n11, assign29760_e27955_d_n12,) = {
    if (((((locals.var_guard1424 != 0.0) && (locals.var_guard1425 != 0.0)) && (locals.var_guard1426 == 0.0)) && (locals.var_guard1427 == 0.0)) && (locals.var_guard1428 != 0.0)) {
        let assign29760_e27953: f64 = (locals.var_t0__blk808 * locals.var_t0__blk808);
        (assign29760_e27953, ((locals.var_t0__blk808_dn3 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn3)), ((locals.var_t0__blk808_dn4 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn4)), ((locals.var_t0__blk808_dn5 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn5)), ((locals.var_t0__blk808_dn6 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn6)), ((locals.var_t0__blk808_dn7 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn7)), ((locals.var_t0__blk808_dn8 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn8)), ((locals.var_t0__blk808_dn9 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn9)), ((locals.var_t0__blk808_dn10 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn10)), ((locals.var_t0__blk808_dn11 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn11)), ((locals.var_t0__blk808_dn12 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn12)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign29760_e27955;
        locals.var_t1__blk809_dn3 = assign29760_e27955_d_n3;
        locals.var_t1__blk809_dn4 = assign29760_e27955_d_n4;
        locals.var_t1__blk809_dn5 = assign29760_e27955_d_n5;
        locals.var_t1__blk809_dn6 = assign29760_e27955_d_n6;
        locals.var_t1__blk809_dn7 = assign29760_e27955_d_n7;
        locals.var_t1__blk809_dn8 = assign29760_e27955_d_n8;
        locals.var_t1__blk809_dn9 = assign29760_e27955_d_n9;
        locals.var_t1__blk809_dn10 = assign29760_e27955_d_n10;
        locals.var_t1__blk809_dn11 = assign29760_e27955_d_n11;
        locals.var_t1__blk809_dn12 = assign29760_e27955_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign29770_e27981, assign29770_e27981_d_n3, assign29770_e27981_d_n4, assign29770_e27981_d_n5, assign29770_e27981_d_n6, assign29770_e27981_d_n7, assign29770_e27981_d_n8, assign29770_e27981_d_n9, assign29770_e27981_d_n10, assign29770_e27981_d_n11, assign29770_e27981_d_n12,) = {
    if (((((locals.var_guard1424 != 0.0) && (locals.var_guard1425 != 0.0)) && (locals.var_guard1426 == 0.0)) && (locals.var_guard1427 == 0.0)) && (locals.var_guard1428 != 0.0)) {
        let assign29770_e27969: f64 = (locals.var_b4soicsmin * locals.var_t10__blk818);
        let assign29770_e27971: f64 = (assign29770_e27969 + locals.var_b4soist4);
        let assign29770_e27974: f64 = (locals.var_pparam_b4soist3 / 3.0);
        let assign29770_e27976: f64 = (assign29770_e27974 * locals.var_t0__blk808);
        let assign29770_e27978: f64 = (assign29770_e27976 * locals.var_t1__blk809);
        let assign29770_e27979: f64 = (assign29770_e27971 + assign29770_e27978);
        (assign29770_e27979, ((((locals.var_b4soicsmin_dn3 * locals.var_t10__blk818) + (locals.var_b4soicsmin * locals.var_t10__blk818_dn3)) + locals.var_b4soist4_dn3) + (((((locals.var_pparam_b4soist3_dn3 / 3.0) * locals.var_t0__blk808) + (assign29770_e27974 * locals.var_t0__blk808_dn3)) * locals.var_t1__blk809) + (assign29770_e27976 * locals.var_t1__blk809_dn3))), ((((locals.var_b4soicsmin_dn4 * locals.var_t10__blk818) + (locals.var_b4soicsmin * locals.var_t10__blk818_dn4)) + locals.var_b4soist4_dn4) + (((((locals.var_pparam_b4soist3_dn4 / 3.0) * locals.var_t0__blk808) + (assign29770_e27974 * locals.var_t0__blk808_dn4)) * locals.var_t1__blk809) + (assign29770_e27976 * locals.var_t1__blk809_dn4))), ((((locals.var_b4soicsmin_dn5 * locals.var_t10__blk818) + (locals.var_b4soicsmin * locals.var_t10__blk818_dn5)) + locals.var_b4soist4_dn5) + (((((locals.var_pparam_b4soist3_dn5 / 3.0) * locals.var_t0__blk808) + (assign29770_e27974 * locals.var_t0__blk808_dn5)) * locals.var_t1__blk809) + (assign29770_e27976 * locals.var_t1__blk809_dn5))), ((((locals.var_b4soicsmin_dn6 * locals.var_t10__blk818) + (locals.var_b4soicsmin * locals.var_t10__blk818_dn6)) + locals.var_b4soist4_dn6) + (((((locals.var_pparam_b4soist3_dn6 / 3.0) * locals.var_t0__blk808) + (assign29770_e27974 * locals.var_t0__blk808_dn6)) * locals.var_t1__blk809) + (assign29770_e27976 * locals.var_t1__blk809_dn6))), ((((locals.var_b4soicsmin_dn7 * locals.var_t10__blk818) + (locals.var_b4soicsmin * locals.var_t10__blk818_dn7)) + locals.var_b4soist4_dn7) + (((((locals.var_pparam_b4soist3_dn7 / 3.0) * locals.var_t0__blk808) + (assign29770_e27974 * locals.var_t0__blk808_dn7)) * locals.var_t1__blk809) + (assign29770_e27976 * locals.var_t1__blk809_dn7))), ((((locals.var_b4soicsmin_dn8 * locals.var_t10__blk818) + (locals.var_b4soicsmin * locals.var_t10__blk818_dn8)) + locals.var_b4soist4_dn8) + (((((locals.var_pparam_b4soist3_dn8 / 3.0) * locals.var_t0__blk808) + (assign29770_e27974 * locals.var_t0__blk808_dn8)) * locals.var_t1__blk809) + (assign29770_e27976 * locals.var_t1__blk809_dn8))), ((((locals.var_b4soicsmin_dn9 * locals.var_t10__blk818) + (locals.var_b4soicsmin * locals.var_t10__blk818_dn9)) + locals.var_b4soist4_dn9) + (((((locals.var_pparam_b4soist3_dn9 / 3.0) * locals.var_t0__blk808) + (assign29770_e27974 * locals.var_t0__blk808_dn9)) * locals.var_t1__blk809) + (assign29770_e27976 * locals.var_t1__blk809_dn9))), ((((locals.var_b4soicsmin_dn10 * locals.var_t10__blk818) + (locals.var_b4soicsmin * locals.var_t10__blk818_dn10)) + locals.var_b4soist4_dn10) + (((((locals.var_pparam_b4soist3_dn10 / 3.0) * locals.var_t0__blk808) + (assign29770_e27974 * locals.var_t0__blk808_dn10)) * locals.var_t1__blk809) + (assign29770_e27976 * locals.var_t1__blk809_dn10))), ((((locals.var_b4soicsmin_dn11 * locals.var_t10__blk818) + (locals.var_b4soicsmin * locals.var_t10__blk818_dn11)) + locals.var_b4soist4_dn11) + (((((locals.var_pparam_b4soist3_dn11 / 3.0) * locals.var_t0__blk808) + (assign29770_e27974 * locals.var_t0__blk808_dn11)) * locals.var_t1__blk809) + (assign29770_e27976 * locals.var_t1__blk809_dn11))), ((((locals.var_b4soicsmin_dn12 * locals.var_t10__blk818) + (locals.var_b4soicsmin * locals.var_t10__blk818_dn12)) + locals.var_b4soist4_dn12) + (((((locals.var_pparam_b4soist3_dn12 / 3.0) * locals.var_t0__blk808) + (assign29770_e27974 * locals.var_t0__blk808_dn12)) * locals.var_t1__blk809) + (assign29770_e27976 * locals.var_t1__blk809_dn12))),)
    } else {
        (locals.var_b4soiqse, locals.var_b4soiqse_dn3, locals.var_b4soiqse_dn4, locals.var_b4soiqse_dn5, locals.var_b4soiqse_dn6, locals.var_b4soiqse_dn7, locals.var_b4soiqse_dn8, locals.var_b4soiqse_dn9, locals.var_b4soiqse_dn10, locals.var_b4soiqse_dn11, locals.var_b4soiqse_dn12,)
    }
};
        locals.var_b4soiqse = assign29770_e27981;
        locals.var_b4soiqse_dn3 = assign29770_e27981_d_n3;
        locals.var_b4soiqse_dn4 = assign29770_e27981_d_n4;
        locals.var_b4soiqse_dn5 = assign29770_e27981_d_n5;
        locals.var_b4soiqse_dn6 = assign29770_e27981_d_n6;
        locals.var_b4soiqse_dn7 = assign29770_e27981_d_n7;
        locals.var_b4soiqse_dn8 = assign29770_e27981_d_n8;
        locals.var_b4soiqse_dn9 = assign29770_e27981_d_n9;
        locals.var_b4soiqse_dn10 = assign29770_e27981_d_n10;
        locals.var_b4soiqse_dn11 = assign29770_e27981_d_n11;
        locals.var_b4soiqse_dn12 = assign29770_e27981_d_n12;
        locals.var_b4soiqse_rv = 0.0;

        let (assign29780_e28000, assign29780_e28000_d_n3, assign29780_e28000_d_n4, assign29780_e28000_d_n5, assign29780_e28000_d_n6, assign29780_e28000_d_n7, assign29780_e28000_d_n8, assign29780_e28000_d_n9, assign29780_e28000_d_n10, assign29780_e28000_d_n11, assign29780_e28000_d_n12,) = {
    if (((((locals.var_guard1424 != 0.0) && (locals.var_guard1425 != 0.0)) && (locals.var_guard1426 == 0.0)) && (locals.var_guard1427 == 0.0)) && (locals.var_guard1428 == 0.0)) {
        let assign29780_e27996: f64 = (locals.var_b4soicsmin * locals.var_t10__blk818);
        let assign29780_e27998: f64 = (assign29780_e27996 + locals.var_b4soist4);
        (assign29780_e27998, (((locals.var_b4soicsmin_dn3 * locals.var_t10__blk818) + (locals.var_b4soicsmin * locals.var_t10__blk818_dn3)) + locals.var_b4soist4_dn3), (((locals.var_b4soicsmin_dn4 * locals.var_t10__blk818) + (locals.var_b4soicsmin * locals.var_t10__blk818_dn4)) + locals.var_b4soist4_dn4), (((locals.var_b4soicsmin_dn5 * locals.var_t10__blk818) + (locals.var_b4soicsmin * locals.var_t10__blk818_dn5)) + locals.var_b4soist4_dn5), (((locals.var_b4soicsmin_dn6 * locals.var_t10__blk818) + (locals.var_b4soicsmin * locals.var_t10__blk818_dn6)) + locals.var_b4soist4_dn6), (((locals.var_b4soicsmin_dn7 * locals.var_t10__blk818) + (locals.var_b4soicsmin * locals.var_t10__blk818_dn7)) + locals.var_b4soist4_dn7), (((locals.var_b4soicsmin_dn8 * locals.var_t10__blk818) + (locals.var_b4soicsmin * locals.var_t10__blk818_dn8)) + locals.var_b4soist4_dn8), (((locals.var_b4soicsmin_dn9 * locals.var_t10__blk818) + (locals.var_b4soicsmin * locals.var_t10__blk818_dn9)) + locals.var_b4soist4_dn9), (((locals.var_b4soicsmin_dn10 * locals.var_t10__blk818) + (locals.var_b4soicsmin * locals.var_t10__blk818_dn10)) + locals.var_b4soist4_dn10), (((locals.var_b4soicsmin_dn11 * locals.var_t10__blk818) + (locals.var_b4soicsmin * locals.var_t10__blk818_dn11)) + locals.var_b4soist4_dn11), (((locals.var_b4soicsmin_dn12 * locals.var_t10__blk818) + (locals.var_b4soicsmin * locals.var_t10__blk818_dn12)) + locals.var_b4soist4_dn12),)
    } else {
        (locals.var_b4soiqse, locals.var_b4soiqse_dn3, locals.var_b4soiqse_dn4, locals.var_b4soiqse_dn5, locals.var_b4soiqse_dn6, locals.var_b4soiqse_dn7, locals.var_b4soiqse_dn8, locals.var_b4soiqse_dn9, locals.var_b4soiqse_dn10, locals.var_b4soiqse_dn11, locals.var_b4soiqse_dn12,)
    }
};
        locals.var_b4soiqse = assign29780_e28000;
        locals.var_b4soiqse_dn3 = assign29780_e28000_d_n3;
        locals.var_b4soiqse_dn4 = assign29780_e28000_d_n4;
        locals.var_b4soiqse_dn5 = assign29780_e28000_d_n5;
        locals.var_b4soiqse_dn6 = assign29780_e28000_d_n6;
        locals.var_b4soiqse_dn7 = assign29780_e28000_d_n7;
        locals.var_b4soiqse_dn8 = assign29780_e28000_d_n8;
        locals.var_b4soiqse_dn9 = assign29780_e28000_d_n9;
        locals.var_b4soiqse_dn10 = assign29780_e28000_d_n10;
        locals.var_b4soiqse_dn11 = assign29780_e28000_d_n11;
        locals.var_b4soiqse_dn12 = assign29780_e28000_d_n12;
        locals.var_b4soiqse_rv = 0.0;

        let assign29790_e28003: f64 = if locals.var_t10__blk818 < locals.var_pparam_b4soivsdth { 1.0 } else { 0.0 };
        locals.var_guard1429 = assign29790_e28003;
        locals.var_guard1429_rv = 0.0;

        let (assign29800_e28016, assign29800_e28016_d_n3, assign29800_e28016_d_n4, assign29800_e28016_d_n5, assign29800_e28016_d_n6, assign29800_e28016_d_n7, assign29800_e28016_d_n8, assign29800_e28016_d_n9, assign29800_e28016_d_n10, assign29800_e28016_d_n11, assign29800_e28016_d_n12,) = {
    if (((locals.var_guard1424 != 0.0) && (locals.var_guard1425 == 0.0)) && (locals.var_guard1429 != 0.0)) {
        let assign29800_e28013: f64 = (locals.var_t10__blk818 - locals.var_pparam_b4soivsdth);
        let assign29800_e28014: f64 = (locals.var_b4soicsmin * assign29800_e28013);
        (assign29800_e28014, ((locals.var_b4soicsmin_dn3 * assign29800_e28013) + (locals.var_b4soicsmin * (locals.var_t10__blk818_dn3 - locals.var_pparam_b4soivsdth_dn3))), ((locals.var_b4soicsmin_dn4 * assign29800_e28013) + (locals.var_b4soicsmin * (locals.var_t10__blk818_dn4 - locals.var_pparam_b4soivsdth_dn4))), ((locals.var_b4soicsmin_dn5 * assign29800_e28013) + (locals.var_b4soicsmin * (locals.var_t10__blk818_dn5 - locals.var_pparam_b4soivsdth_dn5))), ((locals.var_b4soicsmin_dn6 * assign29800_e28013) + (locals.var_b4soicsmin * (locals.var_t10__blk818_dn6 - locals.var_pparam_b4soivsdth_dn6))), ((locals.var_b4soicsmin_dn7 * assign29800_e28013) + (locals.var_b4soicsmin * (locals.var_t10__blk818_dn7 - locals.var_pparam_b4soivsdth_dn7))), ((locals.var_b4soicsmin_dn8 * assign29800_e28013) + (locals.var_b4soicsmin * (locals.var_t10__blk818_dn8 - locals.var_pparam_b4soivsdth_dn8))), ((locals.var_b4soicsmin_dn9 * assign29800_e28013) + (locals.var_b4soicsmin * (locals.var_t10__blk818_dn9 - locals.var_pparam_b4soivsdth_dn9))), ((locals.var_b4soicsmin_dn10 * assign29800_e28013) + (locals.var_b4soicsmin * (locals.var_t10__blk818_dn10 - locals.var_pparam_b4soivsdth_dn10))), ((locals.var_b4soicsmin_dn11 * assign29800_e28013) + (locals.var_b4soicsmin * (locals.var_t10__blk818_dn11 - locals.var_pparam_b4soivsdth_dn11))), ((locals.var_b4soicsmin_dn12 * assign29800_e28013) + (locals.var_b4soicsmin * (locals.var_t10__blk818_dn12 - locals.var_pparam_b4soivsdth_dn12))),)
    } else {
        (locals.var_b4soiqse, locals.var_b4soiqse_dn3, locals.var_b4soiqse_dn4, locals.var_b4soiqse_dn5, locals.var_b4soiqse_dn6, locals.var_b4soiqse_dn7, locals.var_b4soiqse_dn8, locals.var_b4soiqse_dn9, locals.var_b4soiqse_dn10, locals.var_b4soiqse_dn11, locals.var_b4soiqse_dn12,)
    }
};
        locals.var_b4soiqse = assign29800_e28016;
        locals.var_b4soiqse_dn3 = assign29800_e28016_d_n3;
        locals.var_b4soiqse_dn4 = assign29800_e28016_d_n4;
        locals.var_b4soiqse_dn5 = assign29800_e28016_d_n5;
        locals.var_b4soiqse_dn6 = assign29800_e28016_d_n6;
        locals.var_b4soiqse_dn7 = assign29800_e28016_d_n7;
        locals.var_b4soiqse_dn8 = assign29800_e28016_d_n8;
        locals.var_b4soiqse_dn9 = assign29800_e28016_d_n9;
        locals.var_b4soiqse_dn10 = assign29800_e28016_d_n10;
        locals.var_b4soiqse_dn11 = assign29800_e28016_d_n11;
        locals.var_b4soiqse_dn12 = assign29800_e28016_d_n12;
        locals.var_b4soiqse_rv = 0.0;

        let assign29810_e28019: f64 = if locals.var_t10__blk818 < locals.var_pparam_b4soisdt1 { 1.0 } else { 0.0 };
        locals.var_guard1430 = assign29810_e28019;
        locals.var_guard1430_rv = 0.0;

        let (assign29820_e28033, assign29820_e28033_d_n3, assign29820_e28033_d_n4, assign29820_e28033_d_n5, assign29820_e28033_d_n6, assign29820_e28033_d_n7, assign29820_e28033_d_n8, assign29820_e28033_d_n9, assign29820_e28033_d_n10, assign29820_e28033_d_n11, assign29820_e28033_d_n12,) = {
    if ((((locals.var_guard1424 != 0.0) && (locals.var_guard1425 == 0.0)) && (locals.var_guard1429 == 0.0)) && (locals.var_guard1430 != 0.0)) {
        let assign29820_e28031: f64 = (locals.var_t10__blk818 - locals.var_pparam_b4soivsdth);
        (assign29820_e28031, (locals.var_t10__blk818_dn3 - locals.var_pparam_b4soivsdth_dn3), (locals.var_t10__blk818_dn4 - locals.var_pparam_b4soivsdth_dn4), (locals.var_t10__blk818_dn5 - locals.var_pparam_b4soivsdth_dn5), (locals.var_t10__blk818_dn6 - locals.var_pparam_b4soivsdth_dn6), (locals.var_t10__blk818_dn7 - locals.var_pparam_b4soivsdth_dn7), (locals.var_t10__blk818_dn8 - locals.var_pparam_b4soivsdth_dn8), (locals.var_t10__blk818_dn9 - locals.var_pparam_b4soivsdth_dn9), (locals.var_t10__blk818_dn10 - locals.var_pparam_b4soivsdth_dn10), (locals.var_t10__blk818_dn11 - locals.var_pparam_b4soivsdth_dn11), (locals.var_t10__blk818_dn12 - locals.var_pparam_b4soivsdth_dn12),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign29820_e28033;
        locals.var_t0__blk808_dn3 = assign29820_e28033_d_n3;
        locals.var_t0__blk808_dn4 = assign29820_e28033_d_n4;
        locals.var_t0__blk808_dn5 = assign29820_e28033_d_n5;
        locals.var_t0__blk808_dn6 = assign29820_e28033_d_n6;
        locals.var_t0__blk808_dn7 = assign29820_e28033_d_n7;
        locals.var_t0__blk808_dn8 = assign29820_e28033_d_n8;
        locals.var_t0__blk808_dn9 = assign29820_e28033_d_n9;
        locals.var_t0__blk808_dn10 = assign29820_e28033_d_n10;
        locals.var_t0__blk808_dn11 = assign29820_e28033_d_n11;
        locals.var_t0__blk808_dn12 = assign29820_e28033_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign29830_e28047, assign29830_e28047_d_n3, assign29830_e28047_d_n4, assign29830_e28047_d_n5, assign29830_e28047_d_n6, assign29830_e28047_d_n7, assign29830_e28047_d_n8, assign29830_e28047_d_n9, assign29830_e28047_d_n10, assign29830_e28047_d_n11, assign29830_e28047_d_n12,) = {
    if ((((locals.var_guard1424 != 0.0) && (locals.var_guard1425 == 0.0)) && (locals.var_guard1429 == 0.0)) && (locals.var_guard1430 != 0.0)) {
        let assign29830_e28045: f64 = (locals.var_t0__blk808 * locals.var_t0__blk808);
        (assign29830_e28045, ((locals.var_t0__blk808_dn3 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn3)), ((locals.var_t0__blk808_dn4 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn4)), ((locals.var_t0__blk808_dn5 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn5)), ((locals.var_t0__blk808_dn6 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn6)), ((locals.var_t0__blk808_dn7 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn7)), ((locals.var_t0__blk808_dn8 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn8)), ((locals.var_t0__blk808_dn9 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn9)), ((locals.var_t0__blk808_dn10 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn10)), ((locals.var_t0__blk808_dn11 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn11)), ((locals.var_t0__blk808_dn12 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn12)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign29830_e28047;
        locals.var_t1__blk809_dn3 = assign29830_e28047_d_n3;
        locals.var_t1__blk809_dn4 = assign29830_e28047_d_n4;
        locals.var_t1__blk809_dn5 = assign29830_e28047_d_n5;
        locals.var_t1__blk809_dn6 = assign29830_e28047_d_n6;
        locals.var_t1__blk809_dn7 = assign29830_e28047_d_n7;
        locals.var_t1__blk809_dn8 = assign29830_e28047_d_n8;
        locals.var_t1__blk809_dn9 = assign29830_e28047_d_n9;
        locals.var_t1__blk809_dn10 = assign29830_e28047_d_n10;
        locals.var_t1__blk809_dn11 = assign29830_e28047_d_n11;
        locals.var_t1__blk809_dn12 = assign29830_e28047_d_n12;
        locals.var_t1__blk809_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_92(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign29840_e28067, assign29840_e28067_d_n3, assign29840_e28067_d_n4, assign29840_e28067_d_n5, assign29840_e28067_d_n6, assign29840_e28067_d_n7, assign29840_e28067_d_n8, assign29840_e28067_d_n9, assign29840_e28067_d_n10, assign29840_e28067_d_n11, assign29840_e28067_d_n12,) = {
    if ((((locals.var_guard1424 != 0.0) && (locals.var_guard1425 == 0.0)) && (locals.var_guard1429 == 0.0)) && (locals.var_guard1430 != 0.0)) {
        let assign29840_e28061: f64 = (locals.var_pparam_b4soist2 / 3.0);
        let assign29840_e28063: f64 = (assign29840_e28061 * locals.var_t1__blk809);
        let assign29840_e28064: f64 = (locals.var_b4soicsmin - assign29840_e28063);
        let assign29840_e28065: f64 = (locals.var_t0__blk808 * assign29840_e28064);
        (assign29840_e28065, ((locals.var_t0__blk808_dn3 * assign29840_e28064) + (locals.var_t0__blk808 * (locals.var_b4soicsmin_dn3 - (((locals.var_pparam_b4soist2_dn3 / 3.0) * locals.var_t1__blk809) + (assign29840_e28061 * locals.var_t1__blk809_dn3))))), ((locals.var_t0__blk808_dn4 * assign29840_e28064) + (locals.var_t0__blk808 * (locals.var_b4soicsmin_dn4 - (((locals.var_pparam_b4soist2_dn4 / 3.0) * locals.var_t1__blk809) + (assign29840_e28061 * locals.var_t1__blk809_dn4))))), ((locals.var_t0__blk808_dn5 * assign29840_e28064) + (locals.var_t0__blk808 * (locals.var_b4soicsmin_dn5 - (((locals.var_pparam_b4soist2_dn5 / 3.0) * locals.var_t1__blk809) + (assign29840_e28061 * locals.var_t1__blk809_dn5))))), ((locals.var_t0__blk808_dn6 * assign29840_e28064) + (locals.var_t0__blk808 * (locals.var_b4soicsmin_dn6 - (((locals.var_pparam_b4soist2_dn6 / 3.0) * locals.var_t1__blk809) + (assign29840_e28061 * locals.var_t1__blk809_dn6))))), ((locals.var_t0__blk808_dn7 * assign29840_e28064) + (locals.var_t0__blk808 * (locals.var_b4soicsmin_dn7 - (((locals.var_pparam_b4soist2_dn7 / 3.0) * locals.var_t1__blk809) + (assign29840_e28061 * locals.var_t1__blk809_dn7))))), ((locals.var_t0__blk808_dn8 * assign29840_e28064) + (locals.var_t0__blk808 * (locals.var_b4soicsmin_dn8 - (((locals.var_pparam_b4soist2_dn8 / 3.0) * locals.var_t1__blk809) + (assign29840_e28061 * locals.var_t1__blk809_dn8))))), ((locals.var_t0__blk808_dn9 * assign29840_e28064) + (locals.var_t0__blk808 * (locals.var_b4soicsmin_dn9 - (((locals.var_pparam_b4soist2_dn9 / 3.0) * locals.var_t1__blk809) + (assign29840_e28061 * locals.var_t1__blk809_dn9))))), ((locals.var_t0__blk808_dn10 * assign29840_e28064) + (locals.var_t0__blk808 * (locals.var_b4soicsmin_dn10 - (((locals.var_pparam_b4soist2_dn10 / 3.0) * locals.var_t1__blk809) + (assign29840_e28061 * locals.var_t1__blk809_dn10))))), ((locals.var_t0__blk808_dn11 * assign29840_e28064) + (locals.var_t0__blk808 * (locals.var_b4soicsmin_dn11 - (((locals.var_pparam_b4soist2_dn11 / 3.0) * locals.var_t1__blk809) + (assign29840_e28061 * locals.var_t1__blk809_dn11))))), ((locals.var_t0__blk808_dn12 * assign29840_e28064) + (locals.var_t0__blk808 * (locals.var_b4soicsmin_dn12 - (((locals.var_pparam_b4soist2_dn12 / 3.0) * locals.var_t1__blk809) + (assign29840_e28061 * locals.var_t1__blk809_dn12))))),)
    } else {
        (locals.var_b4soiqse, locals.var_b4soiqse_dn3, locals.var_b4soiqse_dn4, locals.var_b4soiqse_dn5, locals.var_b4soiqse_dn6, locals.var_b4soiqse_dn7, locals.var_b4soiqse_dn8, locals.var_b4soiqse_dn9, locals.var_b4soiqse_dn10, locals.var_b4soiqse_dn11, locals.var_b4soiqse_dn12,)
    }
};
        locals.var_b4soiqse = assign29840_e28067;
        locals.var_b4soiqse_dn3 = assign29840_e28067_d_n3;
        locals.var_b4soiqse_dn4 = assign29840_e28067_d_n4;
        locals.var_b4soiqse_dn5 = assign29840_e28067_d_n5;
        locals.var_b4soiqse_dn6 = assign29840_e28067_d_n6;
        locals.var_b4soiqse_dn7 = assign29840_e28067_d_n7;
        locals.var_b4soiqse_dn8 = assign29840_e28067_d_n8;
        locals.var_b4soiqse_dn9 = assign29840_e28067_d_n9;
        locals.var_b4soiqse_dn10 = assign29840_e28067_d_n10;
        locals.var_b4soiqse_dn11 = assign29840_e28067_d_n11;
        locals.var_b4soiqse_dn12 = assign29840_e28067_d_n12;
        locals.var_b4soiqse_rv = 0.0;

        let assign29850_e28070: f64 = if locals.var_t10__blk818 < locals.var_pparam_b4soivsdfb { 1.0 } else { 0.0 };
        locals.var_guard1431 = assign29850_e28070;
        locals.var_guard1431_rv = 0.0;

        let (assign29860_e28087, assign29860_e28087_d_n3, assign29860_e28087_d_n4, assign29860_e28087_d_n5, assign29860_e28087_d_n6, assign29860_e28087_d_n7, assign29860_e28087_d_n8, assign29860_e28087_d_n9, assign29860_e28087_d_n10, assign29860_e28087_d_n11, assign29860_e28087_d_n12,) = {
    if (((((locals.var_guard1424 != 0.0) && (locals.var_guard1425 == 0.0)) && (locals.var_guard1429 == 0.0)) && (locals.var_guard1430 == 0.0)) && (locals.var_guard1431 != 0.0)) {
        let assign29860_e28085: f64 = (locals.var_t10__blk818 - locals.var_pparam_b4soivsdfb);
        (assign29860_e28085, (locals.var_t10__blk818_dn3 - locals.var_pparam_b4soivsdfb_dn3), (locals.var_t10__blk818_dn4 - locals.var_pparam_b4soivsdfb_dn4), (locals.var_t10__blk818_dn5 - locals.var_pparam_b4soivsdfb_dn5), (locals.var_t10__blk818_dn6 - locals.var_pparam_b4soivsdfb_dn6), (locals.var_t10__blk818_dn7 - locals.var_pparam_b4soivsdfb_dn7), (locals.var_t10__blk818_dn8 - locals.var_pparam_b4soivsdfb_dn8), (locals.var_t10__blk818_dn9 - locals.var_pparam_b4soivsdfb_dn9), (locals.var_t10__blk818_dn10 - locals.var_pparam_b4soivsdfb_dn10), (locals.var_t10__blk818_dn11 - locals.var_pparam_b4soivsdfb_dn11), (locals.var_t10__blk818_dn12 - locals.var_pparam_b4soivsdfb_dn12),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign29860_e28087;
        locals.var_t0__blk808_dn3 = assign29860_e28087_d_n3;
        locals.var_t0__blk808_dn4 = assign29860_e28087_d_n4;
        locals.var_t0__blk808_dn5 = assign29860_e28087_d_n5;
        locals.var_t0__blk808_dn6 = assign29860_e28087_d_n6;
        locals.var_t0__blk808_dn7 = assign29860_e28087_d_n7;
        locals.var_t0__blk808_dn8 = assign29860_e28087_d_n8;
        locals.var_t0__blk808_dn9 = assign29860_e28087_d_n9;
        locals.var_t0__blk808_dn10 = assign29860_e28087_d_n10;
        locals.var_t0__blk808_dn11 = assign29860_e28087_d_n11;
        locals.var_t0__blk808_dn12 = assign29860_e28087_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign29870_e28104, assign29870_e28104_d_n3, assign29870_e28104_d_n4, assign29870_e28104_d_n5, assign29870_e28104_d_n6, assign29870_e28104_d_n7, assign29870_e28104_d_n8, assign29870_e28104_d_n9, assign29870_e28104_d_n10, assign29870_e28104_d_n11, assign29870_e28104_d_n12,) = {
    if (((((locals.var_guard1424 != 0.0) && (locals.var_guard1425 == 0.0)) && (locals.var_guard1429 == 0.0)) && (locals.var_guard1430 == 0.0)) && (locals.var_guard1431 != 0.0)) {
        let assign29870_e28102: f64 = (locals.var_t0__blk808 * locals.var_t0__blk808);
        (assign29870_e28102, ((locals.var_t0__blk808_dn3 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn3)), ((locals.var_t0__blk808_dn4 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn4)), ((locals.var_t0__blk808_dn5 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn5)), ((locals.var_t0__blk808_dn6 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn6)), ((locals.var_t0__blk808_dn7 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn7)), ((locals.var_t0__blk808_dn8 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn8)), ((locals.var_t0__blk808_dn9 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn9)), ((locals.var_t0__blk808_dn10 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn10)), ((locals.var_t0__blk808_dn11 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn11)), ((locals.var_t0__blk808_dn12 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn12)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign29870_e28104;
        locals.var_t1__blk809_dn3 = assign29870_e28104_d_n3;
        locals.var_t1__blk809_dn4 = assign29870_e28104_d_n4;
        locals.var_t1__blk809_dn5 = assign29870_e28104_d_n5;
        locals.var_t1__blk809_dn6 = assign29870_e28104_d_n6;
        locals.var_t1__blk809_dn7 = assign29870_e28104_d_n7;
        locals.var_t1__blk809_dn8 = assign29870_e28104_d_n8;
        locals.var_t1__blk809_dn9 = assign29870_e28104_d_n9;
        locals.var_t1__blk809_dn10 = assign29870_e28104_d_n10;
        locals.var_t1__blk809_dn11 = assign29870_e28104_d_n11;
        locals.var_t1__blk809_dn12 = assign29870_e28104_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign29880_e28131, assign29880_e28131_d_n3, assign29880_e28131_d_n4, assign29880_e28131_d_n5, assign29880_e28131_d_n6, assign29880_e28131_d_n7, assign29880_e28131_d_n8, assign29880_e28131_d_n9, assign29880_e28131_d_n10, assign29880_e28131_d_n11, assign29880_e28131_d_n12,) = {
    if (((((locals.var_guard1424 != 0.0) && (locals.var_guard1425 == 0.0)) && (locals.var_guard1429 == 0.0)) && (locals.var_guard1430 == 0.0)) && (locals.var_guard1431 != 0.0)) {
        let assign29880_e28119: f64 = (locals.var_b4soicsbox * locals.var_t10__blk818);
        let assign29880_e28121: f64 = (assign29880_e28119 + locals.var_b4soist4);
        let assign29880_e28124: f64 = (locals.var_pparam_b4soist3 / 3.0);
        let assign29880_e28126: f64 = (assign29880_e28124 * locals.var_t0__blk808);
        let assign29880_e28128: f64 = (assign29880_e28126 * locals.var_t1__blk809);
        let assign29880_e28129: f64 = (assign29880_e28121 + assign29880_e28128);
        (assign29880_e28129, (((locals.var_b4soicsbox * locals.var_t10__blk818_dn3) + locals.var_b4soist4_dn3) + (((((locals.var_pparam_b4soist3_dn3 / 3.0) * locals.var_t0__blk808) + (assign29880_e28124 * locals.var_t0__blk808_dn3)) * locals.var_t1__blk809) + (assign29880_e28126 * locals.var_t1__blk809_dn3))), (((locals.var_b4soicsbox * locals.var_t10__blk818_dn4) + locals.var_b4soist4_dn4) + (((((locals.var_pparam_b4soist3_dn4 / 3.0) * locals.var_t0__blk808) + (assign29880_e28124 * locals.var_t0__blk808_dn4)) * locals.var_t1__blk809) + (assign29880_e28126 * locals.var_t1__blk809_dn4))), (((locals.var_b4soicsbox * locals.var_t10__blk818_dn5) + locals.var_b4soist4_dn5) + (((((locals.var_pparam_b4soist3_dn5 / 3.0) * locals.var_t0__blk808) + (assign29880_e28124 * locals.var_t0__blk808_dn5)) * locals.var_t1__blk809) + (assign29880_e28126 * locals.var_t1__blk809_dn5))), (((locals.var_b4soicsbox * locals.var_t10__blk818_dn6) + locals.var_b4soist4_dn6) + (((((locals.var_pparam_b4soist3_dn6 / 3.0) * locals.var_t0__blk808) + (assign29880_e28124 * locals.var_t0__blk808_dn6)) * locals.var_t1__blk809) + (assign29880_e28126 * locals.var_t1__blk809_dn6))), (((locals.var_b4soicsbox * locals.var_t10__blk818_dn7) + locals.var_b4soist4_dn7) + (((((locals.var_pparam_b4soist3_dn7 / 3.0) * locals.var_t0__blk808) + (assign29880_e28124 * locals.var_t0__blk808_dn7)) * locals.var_t1__blk809) + (assign29880_e28126 * locals.var_t1__blk809_dn7))), (((locals.var_b4soicsbox * locals.var_t10__blk818_dn8) + locals.var_b4soist4_dn8) + (((((locals.var_pparam_b4soist3_dn8 / 3.0) * locals.var_t0__blk808) + (assign29880_e28124 * locals.var_t0__blk808_dn8)) * locals.var_t1__blk809) + (assign29880_e28126 * locals.var_t1__blk809_dn8))), (((locals.var_b4soicsbox * locals.var_t10__blk818_dn9) + locals.var_b4soist4_dn9) + (((((locals.var_pparam_b4soist3_dn9 / 3.0) * locals.var_t0__blk808) + (assign29880_e28124 * locals.var_t0__blk808_dn9)) * locals.var_t1__blk809) + (assign29880_e28126 * locals.var_t1__blk809_dn9))), (((locals.var_b4soicsbox * locals.var_t10__blk818_dn10) + locals.var_b4soist4_dn10) + (((((locals.var_pparam_b4soist3_dn10 / 3.0) * locals.var_t0__blk808) + (assign29880_e28124 * locals.var_t0__blk808_dn10)) * locals.var_t1__blk809) + (assign29880_e28126 * locals.var_t1__blk809_dn10))), (((locals.var_b4soicsbox * locals.var_t10__blk818_dn11) + locals.var_b4soist4_dn11) + (((((locals.var_pparam_b4soist3_dn11 / 3.0) * locals.var_t0__blk808) + (assign29880_e28124 * locals.var_t0__blk808_dn11)) * locals.var_t1__blk809) + (assign29880_e28126 * locals.var_t1__blk809_dn11))), (((locals.var_b4soicsbox * locals.var_t10__blk818_dn12) + locals.var_b4soist4_dn12) + (((((locals.var_pparam_b4soist3_dn12 / 3.0) * locals.var_t0__blk808) + (assign29880_e28124 * locals.var_t0__blk808_dn12)) * locals.var_t1__blk809) + (assign29880_e28126 * locals.var_t1__blk809_dn12))),)
    } else {
        (locals.var_b4soiqse, locals.var_b4soiqse_dn3, locals.var_b4soiqse_dn4, locals.var_b4soiqse_dn5, locals.var_b4soiqse_dn6, locals.var_b4soiqse_dn7, locals.var_b4soiqse_dn8, locals.var_b4soiqse_dn9, locals.var_b4soiqse_dn10, locals.var_b4soiqse_dn11, locals.var_b4soiqse_dn12,)
    }
};
        locals.var_b4soiqse = assign29880_e28131;
        locals.var_b4soiqse_dn3 = assign29880_e28131_d_n3;
        locals.var_b4soiqse_dn4 = assign29880_e28131_d_n4;
        locals.var_b4soiqse_dn5 = assign29880_e28131_d_n5;
        locals.var_b4soiqse_dn6 = assign29880_e28131_d_n6;
        locals.var_b4soiqse_dn7 = assign29880_e28131_d_n7;
        locals.var_b4soiqse_dn8 = assign29880_e28131_d_n8;
        locals.var_b4soiqse_dn9 = assign29880_e28131_d_n9;
        locals.var_b4soiqse_dn10 = assign29880_e28131_d_n10;
        locals.var_b4soiqse_dn11 = assign29880_e28131_d_n11;
        locals.var_b4soiqse_dn12 = assign29880_e28131_d_n12;
        locals.var_b4soiqse_rv = 0.0;

        let (assign29890_e28151, assign29890_e28151_d_n3, assign29890_e28151_d_n4, assign29890_e28151_d_n5, assign29890_e28151_d_n6, assign29890_e28151_d_n7, assign29890_e28151_d_n8, assign29890_e28151_d_n9, assign29890_e28151_d_n10, assign29890_e28151_d_n11, assign29890_e28151_d_n12,) = {
    if (((((locals.var_guard1424 != 0.0) && (locals.var_guard1425 == 0.0)) && (locals.var_guard1429 == 0.0)) && (locals.var_guard1430 == 0.0)) && (locals.var_guard1431 == 0.0)) {
        let assign29890_e28147: f64 = (locals.var_b4soicsbox * locals.var_t10__blk818);
        let assign29890_e28149: f64 = (assign29890_e28147 + locals.var_b4soist4);
        (assign29890_e28149, ((locals.var_b4soicsbox * locals.var_t10__blk818_dn3) + locals.var_b4soist4_dn3), ((locals.var_b4soicsbox * locals.var_t10__blk818_dn4) + locals.var_b4soist4_dn4), ((locals.var_b4soicsbox * locals.var_t10__blk818_dn5) + locals.var_b4soist4_dn5), ((locals.var_b4soicsbox * locals.var_t10__blk818_dn6) + locals.var_b4soist4_dn6), ((locals.var_b4soicsbox * locals.var_t10__blk818_dn7) + locals.var_b4soist4_dn7), ((locals.var_b4soicsbox * locals.var_t10__blk818_dn8) + locals.var_b4soist4_dn8), ((locals.var_b4soicsbox * locals.var_t10__blk818_dn9) + locals.var_b4soist4_dn9), ((locals.var_b4soicsbox * locals.var_t10__blk818_dn10) + locals.var_b4soist4_dn10), ((locals.var_b4soicsbox * locals.var_t10__blk818_dn11) + locals.var_b4soist4_dn11), ((locals.var_b4soicsbox * locals.var_t10__blk818_dn12) + locals.var_b4soist4_dn12),)
    } else {
        (locals.var_b4soiqse, locals.var_b4soiqse_dn3, locals.var_b4soiqse_dn4, locals.var_b4soiqse_dn5, locals.var_b4soiqse_dn6, locals.var_b4soiqse_dn7, locals.var_b4soiqse_dn8, locals.var_b4soiqse_dn9, locals.var_b4soiqse_dn10, locals.var_b4soiqse_dn11, locals.var_b4soiqse_dn12,)
    }
};
        locals.var_b4soiqse = assign29890_e28151;
        locals.var_b4soiqse_dn3 = assign29890_e28151_d_n3;
        locals.var_b4soiqse_dn4 = assign29890_e28151_d_n4;
        locals.var_b4soiqse_dn5 = assign29890_e28151_d_n5;
        locals.var_b4soiqse_dn6 = assign29890_e28151_d_n6;
        locals.var_b4soiqse_dn7 = assign29890_e28151_d_n7;
        locals.var_b4soiqse_dn8 = assign29890_e28151_d_n8;
        locals.var_b4soiqse_dn9 = assign29890_e28151_d_n9;
        locals.var_b4soiqse_dn10 = assign29890_e28151_d_n10;
        locals.var_b4soiqse_dn11 = assign29890_e28151_d_n11;
        locals.var_b4soiqse_dn12 = assign29890_e28151_d_n12;
        locals.var_b4soiqse_rv = 0.0;

        let assign29900_e28166: f64 = if (((locals.var_pparam_b4soinsub > 0.0) && (p.p37 > 0.0)) || ((locals.var_pparam_b4soinsub < 0.0) && (p.p37 < 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard1432 = assign29900_e28166;
        locals.var_guard1432_rv = 0.0;

        let assign29910_e28169: f64 = if locals.var_t11 < locals.var_pparam_b4soivsdfb { 1.0 } else { 0.0 };
        locals.var_guard1433 = assign29910_e28169;
        locals.var_guard1433_rv = 0.0;

        let (assign29920_e28181, assign29920_e28181_d_n3, assign29920_e28181_d_n4, assign29920_e28181_d_n5, assign29920_e28181_d_n6, assign29920_e28181_d_n7, assign29920_e28181_d_n8, assign29920_e28181_d_n9, assign29920_e28181_d_n10, assign29920_e28181_d_n11, assign29920_e28181_d_n12,) = {
    if (((locals.var_guard1424 != 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1433 != 0.0)) {
        let assign29920_e28178: f64 = (locals.var_t11 - locals.var_pparam_b4soivsdfb);
        let assign29920_e28179: f64 = (locals.var_b4soicdbox * assign29920_e28178);
        (assign29920_e28179, (locals.var_b4soicdbox * (locals.var_t11_dn3 - locals.var_pparam_b4soivsdfb_dn3)), (locals.var_b4soicdbox * (locals.var_t11_dn4 - locals.var_pparam_b4soivsdfb_dn4)), (locals.var_b4soicdbox * (locals.var_t11_dn5 - locals.var_pparam_b4soivsdfb_dn5)), (locals.var_b4soicdbox * (locals.var_t11_dn6 - locals.var_pparam_b4soivsdfb_dn6)), (locals.var_b4soicdbox * (locals.var_t11_dn7 - locals.var_pparam_b4soivsdfb_dn7)), (locals.var_b4soicdbox * (locals.var_t11_dn8 - locals.var_pparam_b4soivsdfb_dn8)), (locals.var_b4soicdbox * (locals.var_t11_dn9 - locals.var_pparam_b4soivsdfb_dn9)), (locals.var_b4soicdbox * (locals.var_t11_dn10 - locals.var_pparam_b4soivsdfb_dn10)), (locals.var_b4soicdbox * (locals.var_t11_dn11 - locals.var_pparam_b4soivsdfb_dn11)), (locals.var_b4soicdbox * (locals.var_t11_dn12 - locals.var_pparam_b4soivsdfb_dn12)),)
    } else {
        (locals.var_b4soiqde, locals.var_b4soiqde_dn3, locals.var_b4soiqde_dn4, locals.var_b4soiqde_dn5, locals.var_b4soiqde_dn6, locals.var_b4soiqde_dn7, locals.var_b4soiqde_dn8, locals.var_b4soiqde_dn9, locals.var_b4soiqde_dn10, locals.var_b4soiqde_dn11, locals.var_b4soiqde_dn12,)
    }
};
        locals.var_b4soiqde = assign29920_e28181;
        locals.var_b4soiqde_dn3 = assign29920_e28181_d_n3;
        locals.var_b4soiqde_dn4 = assign29920_e28181_d_n4;
        locals.var_b4soiqde_dn5 = assign29920_e28181_d_n5;
        locals.var_b4soiqde_dn6 = assign29920_e28181_d_n6;
        locals.var_b4soiqde_dn7 = assign29920_e28181_d_n7;
        locals.var_b4soiqde_dn8 = assign29920_e28181_d_n8;
        locals.var_b4soiqde_dn9 = assign29920_e28181_d_n9;
        locals.var_b4soiqde_dn10 = assign29920_e28181_d_n10;
        locals.var_b4soiqde_dn11 = assign29920_e28181_d_n11;
        locals.var_b4soiqde_dn12 = assign29920_e28181_d_n12;
        locals.var_b4soiqde_rv = 0.0;

        let assign29930_e28184: f64 = if locals.var_t11 < locals.var_pparam_b4soisdt1 { 1.0 } else { 0.0 };
        locals.var_guard1434 = assign29930_e28184;
        locals.var_guard1434_rv = 0.0;

        let (assign29940_e28197, assign29940_e28197_d_n3, assign29940_e28197_d_n4, assign29940_e28197_d_n5, assign29940_e28197_d_n6, assign29940_e28197_d_n7, assign29940_e28197_d_n8, assign29940_e28197_d_n9, assign29940_e28197_d_n10, assign29940_e28197_d_n11, assign29940_e28197_d_n12,) = {
    if ((((locals.var_guard1424 != 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1433 == 0.0)) && (locals.var_guard1434 != 0.0)) {
        let assign29940_e28195: f64 = (locals.var_t11 - locals.var_pparam_b4soivsdfb);
        (assign29940_e28195, (locals.var_t11_dn3 - locals.var_pparam_b4soivsdfb_dn3), (locals.var_t11_dn4 - locals.var_pparam_b4soivsdfb_dn4), (locals.var_t11_dn5 - locals.var_pparam_b4soivsdfb_dn5), (locals.var_t11_dn6 - locals.var_pparam_b4soivsdfb_dn6), (locals.var_t11_dn7 - locals.var_pparam_b4soivsdfb_dn7), (locals.var_t11_dn8 - locals.var_pparam_b4soivsdfb_dn8), (locals.var_t11_dn9 - locals.var_pparam_b4soivsdfb_dn9), (locals.var_t11_dn10 - locals.var_pparam_b4soivsdfb_dn10), (locals.var_t11_dn11 - locals.var_pparam_b4soivsdfb_dn11), (locals.var_t11_dn12 - locals.var_pparam_b4soivsdfb_dn12),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign29940_e28197;
        locals.var_t0__blk808_dn3 = assign29940_e28197_d_n3;
        locals.var_t0__blk808_dn4 = assign29940_e28197_d_n4;
        locals.var_t0__blk808_dn5 = assign29940_e28197_d_n5;
        locals.var_t0__blk808_dn6 = assign29940_e28197_d_n6;
        locals.var_t0__blk808_dn7 = assign29940_e28197_d_n7;
        locals.var_t0__blk808_dn8 = assign29940_e28197_d_n8;
        locals.var_t0__blk808_dn9 = assign29940_e28197_d_n9;
        locals.var_t0__blk808_dn10 = assign29940_e28197_d_n10;
        locals.var_t0__blk808_dn11 = assign29940_e28197_d_n11;
        locals.var_t0__blk808_dn12 = assign29940_e28197_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign29950_e28210, assign29950_e28210_d_n3, assign29950_e28210_d_n4, assign29950_e28210_d_n5, assign29950_e28210_d_n6, assign29950_e28210_d_n7, assign29950_e28210_d_n8, assign29950_e28210_d_n9, assign29950_e28210_d_n10, assign29950_e28210_d_n11, assign29950_e28210_d_n12,) = {
    if ((((locals.var_guard1424 != 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1433 == 0.0)) && (locals.var_guard1434 != 0.0)) {
        let assign29950_e28208: f64 = (locals.var_t0__blk808 * locals.var_t0__blk808);
        (assign29950_e28208, ((locals.var_t0__blk808_dn3 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn3)), ((locals.var_t0__blk808_dn4 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn4)), ((locals.var_t0__blk808_dn5 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn5)), ((locals.var_t0__blk808_dn6 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn6)), ((locals.var_t0__blk808_dn7 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn7)), ((locals.var_t0__blk808_dn8 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn8)), ((locals.var_t0__blk808_dn9 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn9)), ((locals.var_t0__blk808_dn10 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn10)), ((locals.var_t0__blk808_dn11 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn11)), ((locals.var_t0__blk808_dn12 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn12)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign29950_e28210;
        locals.var_t1__blk809_dn3 = assign29950_e28210_d_n3;
        locals.var_t1__blk809_dn4 = assign29950_e28210_d_n4;
        locals.var_t1__blk809_dn5 = assign29950_e28210_d_n5;
        locals.var_t1__blk809_dn6 = assign29950_e28210_d_n6;
        locals.var_t1__blk809_dn7 = assign29950_e28210_d_n7;
        locals.var_t1__blk809_dn8 = assign29950_e28210_d_n8;
        locals.var_t1__blk809_dn9 = assign29950_e28210_d_n9;
        locals.var_t1__blk809_dn10 = assign29950_e28210_d_n10;
        locals.var_t1__blk809_dn11 = assign29950_e28210_d_n11;
        locals.var_t1__blk809_dn12 = assign29950_e28210_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign29960_e28229, assign29960_e28229_d_n3, assign29960_e28229_d_n4, assign29960_e28229_d_n5, assign29960_e28229_d_n6, assign29960_e28229_d_n7, assign29960_e28229_d_n8, assign29960_e28229_d_n9, assign29960_e28229_d_n10, assign29960_e28229_d_n11, assign29960_e28229_d_n12,) = {
    if ((((locals.var_guard1424 != 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1433 == 0.0)) && (locals.var_guard1434 != 0.0)) {
        let assign29960_e28223: f64 = (locals.var_pparam_b4soidt2 / 3.0);
        let assign29960_e28225: f64 = (assign29960_e28223 * locals.var_t1__blk809);
        let assign29960_e28226: f64 = (locals.var_b4soicdbox - assign29960_e28225);
        let assign29960_e28227: f64 = (locals.var_t0__blk808 * assign29960_e28226);
        (assign29960_e28227, ((locals.var_t0__blk808_dn3 * assign29960_e28226) + (locals.var_t0__blk808 * (-(((locals.var_pparam_b4soidt2_dn3 / 3.0) * locals.var_t1__blk809) + (assign29960_e28223 * locals.var_t1__blk809_dn3))))), ((locals.var_t0__blk808_dn4 * assign29960_e28226) + (locals.var_t0__blk808 * (-(((locals.var_pparam_b4soidt2_dn4 / 3.0) * locals.var_t1__blk809) + (assign29960_e28223 * locals.var_t1__blk809_dn4))))), ((locals.var_t0__blk808_dn5 * assign29960_e28226) + (locals.var_t0__blk808 * (-(((locals.var_pparam_b4soidt2_dn5 / 3.0) * locals.var_t1__blk809) + (assign29960_e28223 * locals.var_t1__blk809_dn5))))), ((locals.var_t0__blk808_dn6 * assign29960_e28226) + (locals.var_t0__blk808 * (-(((locals.var_pparam_b4soidt2_dn6 / 3.0) * locals.var_t1__blk809) + (assign29960_e28223 * locals.var_t1__blk809_dn6))))), ((locals.var_t0__blk808_dn7 * assign29960_e28226) + (locals.var_t0__blk808 * (-(((locals.var_pparam_b4soidt2_dn7 / 3.0) * locals.var_t1__blk809) + (assign29960_e28223 * locals.var_t1__blk809_dn7))))), ((locals.var_t0__blk808_dn8 * assign29960_e28226) + (locals.var_t0__blk808 * (-(((locals.var_pparam_b4soidt2_dn8 / 3.0) * locals.var_t1__blk809) + (assign29960_e28223 * locals.var_t1__blk809_dn8))))), ((locals.var_t0__blk808_dn9 * assign29960_e28226) + (locals.var_t0__blk808 * (-(((locals.var_pparam_b4soidt2_dn9 / 3.0) * locals.var_t1__blk809) + (assign29960_e28223 * locals.var_t1__blk809_dn9))))), ((locals.var_t0__blk808_dn10 * assign29960_e28226) + (locals.var_t0__blk808 * (-(((locals.var_pparam_b4soidt2_dn10 / 3.0) * locals.var_t1__blk809) + (assign29960_e28223 * locals.var_t1__blk809_dn10))))), ((locals.var_t0__blk808_dn11 * assign29960_e28226) + (locals.var_t0__blk808 * (-(((locals.var_pparam_b4soidt2_dn11 / 3.0) * locals.var_t1__blk809) + (assign29960_e28223 * locals.var_t1__blk809_dn11))))), ((locals.var_t0__blk808_dn12 * assign29960_e28226) + (locals.var_t0__blk808 * (-(((locals.var_pparam_b4soidt2_dn12 / 3.0) * locals.var_t1__blk809) + (assign29960_e28223 * locals.var_t1__blk809_dn12))))),)
    } else {
        (locals.var_b4soiqde, locals.var_b4soiqde_dn3, locals.var_b4soiqde_dn4, locals.var_b4soiqde_dn5, locals.var_b4soiqde_dn6, locals.var_b4soiqde_dn7, locals.var_b4soiqde_dn8, locals.var_b4soiqde_dn9, locals.var_b4soiqde_dn10, locals.var_b4soiqde_dn11, locals.var_b4soiqde_dn12,)
    }
};
        locals.var_b4soiqde = assign29960_e28229;
        locals.var_b4soiqde_dn3 = assign29960_e28229_d_n3;
        locals.var_b4soiqde_dn4 = assign29960_e28229_d_n4;
        locals.var_b4soiqde_dn5 = assign29960_e28229_d_n5;
        locals.var_b4soiqde_dn6 = assign29960_e28229_d_n6;
        locals.var_b4soiqde_dn7 = assign29960_e28229_d_n7;
        locals.var_b4soiqde_dn8 = assign29960_e28229_d_n8;
        locals.var_b4soiqde_dn9 = assign29960_e28229_d_n9;
        locals.var_b4soiqde_dn10 = assign29960_e28229_d_n10;
        locals.var_b4soiqde_dn11 = assign29960_e28229_d_n11;
        locals.var_b4soiqde_dn12 = assign29960_e28229_d_n12;
        locals.var_b4soiqde_rv = 0.0;

        let assign29970_e28232: f64 = if locals.var_t11 < locals.var_pparam_b4soivsdth { 1.0 } else { 0.0 };
        locals.var_guard1435 = assign29970_e28232;
        locals.var_guard1435_rv = 0.0;

        let (assign29980_e28248, assign29980_e28248_d_n3, assign29980_e28248_d_n4, assign29980_e28248_d_n5, assign29980_e28248_d_n6, assign29980_e28248_d_n7, assign29980_e28248_d_n8, assign29980_e28248_d_n9, assign29980_e28248_d_n10, assign29980_e28248_d_n11, assign29980_e28248_d_n12,) = {
    if (((((locals.var_guard1424 != 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1433 == 0.0)) && (locals.var_guard1434 == 0.0)) && (locals.var_guard1435 != 0.0)) {
        let assign29980_e28246: f64 = (locals.var_t11 - locals.var_pparam_b4soivsdth);
        (assign29980_e28246, (locals.var_t11_dn3 - locals.var_pparam_b4soivsdth_dn3), (locals.var_t11_dn4 - locals.var_pparam_b4soivsdth_dn4), (locals.var_t11_dn5 - locals.var_pparam_b4soivsdth_dn5), (locals.var_t11_dn6 - locals.var_pparam_b4soivsdth_dn6), (locals.var_t11_dn7 - locals.var_pparam_b4soivsdth_dn7), (locals.var_t11_dn8 - locals.var_pparam_b4soivsdth_dn8), (locals.var_t11_dn9 - locals.var_pparam_b4soivsdth_dn9), (locals.var_t11_dn10 - locals.var_pparam_b4soivsdth_dn10), (locals.var_t11_dn11 - locals.var_pparam_b4soivsdth_dn11), (locals.var_t11_dn12 - locals.var_pparam_b4soivsdth_dn12),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign29980_e28248;
        locals.var_t0__blk808_dn3 = assign29980_e28248_d_n3;
        locals.var_t0__blk808_dn4 = assign29980_e28248_d_n4;
        locals.var_t0__blk808_dn5 = assign29980_e28248_d_n5;
        locals.var_t0__blk808_dn6 = assign29980_e28248_d_n6;
        locals.var_t0__blk808_dn7 = assign29980_e28248_d_n7;
        locals.var_t0__blk808_dn8 = assign29980_e28248_d_n8;
        locals.var_t0__blk808_dn9 = assign29980_e28248_d_n9;
        locals.var_t0__blk808_dn10 = assign29980_e28248_d_n10;
        locals.var_t0__blk808_dn11 = assign29980_e28248_d_n11;
        locals.var_t0__blk808_dn12 = assign29980_e28248_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign29990_e28264, assign29990_e28264_d_n3, assign29990_e28264_d_n4, assign29990_e28264_d_n5, assign29990_e28264_d_n6, assign29990_e28264_d_n7, assign29990_e28264_d_n8, assign29990_e28264_d_n9, assign29990_e28264_d_n10, assign29990_e28264_d_n11, assign29990_e28264_d_n12,) = {
    if (((((locals.var_guard1424 != 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1433 == 0.0)) && (locals.var_guard1434 == 0.0)) && (locals.var_guard1435 != 0.0)) {
        let assign29990_e28262: f64 = (locals.var_t0__blk808 * locals.var_t0__blk808);
        (assign29990_e28262, ((locals.var_t0__blk808_dn3 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn3)), ((locals.var_t0__blk808_dn4 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn4)), ((locals.var_t0__blk808_dn5 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn5)), ((locals.var_t0__blk808_dn6 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn6)), ((locals.var_t0__blk808_dn7 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn7)), ((locals.var_t0__blk808_dn8 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn8)), ((locals.var_t0__blk808_dn9 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn9)), ((locals.var_t0__blk808_dn10 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn10)), ((locals.var_t0__blk808_dn11 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn11)), ((locals.var_t0__blk808_dn12 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn12)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign29990_e28264;
        locals.var_t1__blk809_dn3 = assign29990_e28264_d_n3;
        locals.var_t1__blk809_dn4 = assign29990_e28264_d_n4;
        locals.var_t1__blk809_dn5 = assign29990_e28264_d_n5;
        locals.var_t1__blk809_dn6 = assign29990_e28264_d_n6;
        locals.var_t1__blk809_dn7 = assign29990_e28264_d_n7;
        locals.var_t1__blk809_dn8 = assign29990_e28264_d_n8;
        locals.var_t1__blk809_dn9 = assign29990_e28264_d_n9;
        locals.var_t1__blk809_dn10 = assign29990_e28264_d_n10;
        locals.var_t1__blk809_dn11 = assign29990_e28264_d_n11;
        locals.var_t1__blk809_dn12 = assign29990_e28264_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign30000_e28290, assign30000_e28290_d_n3, assign30000_e28290_d_n4, assign30000_e28290_d_n5, assign30000_e28290_d_n6, assign30000_e28290_d_n7, assign30000_e28290_d_n8, assign30000_e28290_d_n9, assign30000_e28290_d_n10, assign30000_e28290_d_n11, assign30000_e28290_d_n12,) = {
    if (((((locals.var_guard1424 != 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1433 == 0.0)) && (locals.var_guard1434 == 0.0)) && (locals.var_guard1435 != 0.0)) {
        let assign30000_e28278: f64 = (locals.var_b4soicdmin * locals.var_t11);
        let assign30000_e28280: f64 = (assign30000_e28278 + locals.var_b4soidt4);
        let assign30000_e28283: f64 = (locals.var_pparam_b4soidt3 / 3.0);
        let assign30000_e28285: f64 = (assign30000_e28283 * locals.var_t0__blk808);
        let assign30000_e28287: f64 = (assign30000_e28285 * locals.var_t1__blk809);
        let assign30000_e28288: f64 = (assign30000_e28280 + assign30000_e28287);
        (assign30000_e28288, ((((locals.var_b4soicdmin_dn3 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn3)) + locals.var_b4soidt4_dn3) + (((((locals.var_pparam_b4soidt3_dn3 / 3.0) * locals.var_t0__blk808) + (assign30000_e28283 * locals.var_t0__blk808_dn3)) * locals.var_t1__blk809) + (assign30000_e28285 * locals.var_t1__blk809_dn3))), ((((locals.var_b4soicdmin_dn4 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn4)) + locals.var_b4soidt4_dn4) + (((((locals.var_pparam_b4soidt3_dn4 / 3.0) * locals.var_t0__blk808) + (assign30000_e28283 * locals.var_t0__blk808_dn4)) * locals.var_t1__blk809) + (assign30000_e28285 * locals.var_t1__blk809_dn4))), ((((locals.var_b4soicdmin_dn5 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn5)) + locals.var_b4soidt4_dn5) + (((((locals.var_pparam_b4soidt3_dn5 / 3.0) * locals.var_t0__blk808) + (assign30000_e28283 * locals.var_t0__blk808_dn5)) * locals.var_t1__blk809) + (assign30000_e28285 * locals.var_t1__blk809_dn5))), ((((locals.var_b4soicdmin_dn6 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn6)) + locals.var_b4soidt4_dn6) + (((((locals.var_pparam_b4soidt3_dn6 / 3.0) * locals.var_t0__blk808) + (assign30000_e28283 * locals.var_t0__blk808_dn6)) * locals.var_t1__blk809) + (assign30000_e28285 * locals.var_t1__blk809_dn6))), ((((locals.var_b4soicdmin_dn7 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn7)) + locals.var_b4soidt4_dn7) + (((((locals.var_pparam_b4soidt3_dn7 / 3.0) * locals.var_t0__blk808) + (assign30000_e28283 * locals.var_t0__blk808_dn7)) * locals.var_t1__blk809) + (assign30000_e28285 * locals.var_t1__blk809_dn7))), ((((locals.var_b4soicdmin_dn8 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn8)) + locals.var_b4soidt4_dn8) + (((((locals.var_pparam_b4soidt3_dn8 / 3.0) * locals.var_t0__blk808) + (assign30000_e28283 * locals.var_t0__blk808_dn8)) * locals.var_t1__blk809) + (assign30000_e28285 * locals.var_t1__blk809_dn8))), ((((locals.var_b4soicdmin_dn9 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn9)) + locals.var_b4soidt4_dn9) + (((((locals.var_pparam_b4soidt3_dn9 / 3.0) * locals.var_t0__blk808) + (assign30000_e28283 * locals.var_t0__blk808_dn9)) * locals.var_t1__blk809) + (assign30000_e28285 * locals.var_t1__blk809_dn9))), ((((locals.var_b4soicdmin_dn10 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn10)) + locals.var_b4soidt4_dn10) + (((((locals.var_pparam_b4soidt3_dn10 / 3.0) * locals.var_t0__blk808) + (assign30000_e28283 * locals.var_t0__blk808_dn10)) * locals.var_t1__blk809) + (assign30000_e28285 * locals.var_t1__blk809_dn10))), ((((locals.var_b4soicdmin_dn11 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn11)) + locals.var_b4soidt4_dn11) + (((((locals.var_pparam_b4soidt3_dn11 / 3.0) * locals.var_t0__blk808) + (assign30000_e28283 * locals.var_t0__blk808_dn11)) * locals.var_t1__blk809) + (assign30000_e28285 * locals.var_t1__blk809_dn11))), ((((locals.var_b4soicdmin_dn12 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn12)) + locals.var_b4soidt4_dn12) + (((((locals.var_pparam_b4soidt3_dn12 / 3.0) * locals.var_t0__blk808) + (assign30000_e28283 * locals.var_t0__blk808_dn12)) * locals.var_t1__blk809) + (assign30000_e28285 * locals.var_t1__blk809_dn12))),)
    } else {
        (locals.var_b4soiqde, locals.var_b4soiqde_dn3, locals.var_b4soiqde_dn4, locals.var_b4soiqde_dn5, locals.var_b4soiqde_dn6, locals.var_b4soiqde_dn7, locals.var_b4soiqde_dn8, locals.var_b4soiqde_dn9, locals.var_b4soiqde_dn10, locals.var_b4soiqde_dn11, locals.var_b4soiqde_dn12,)
    }
};
        locals.var_b4soiqde = assign30000_e28290;
        locals.var_b4soiqde_dn3 = assign30000_e28290_d_n3;
        locals.var_b4soiqde_dn4 = assign30000_e28290_d_n4;
        locals.var_b4soiqde_dn5 = assign30000_e28290_d_n5;
        locals.var_b4soiqde_dn6 = assign30000_e28290_d_n6;
        locals.var_b4soiqde_dn7 = assign30000_e28290_d_n7;
        locals.var_b4soiqde_dn8 = assign30000_e28290_d_n8;
        locals.var_b4soiqde_dn9 = assign30000_e28290_d_n9;
        locals.var_b4soiqde_dn10 = assign30000_e28290_d_n10;
        locals.var_b4soiqde_dn11 = assign30000_e28290_d_n11;
        locals.var_b4soiqde_dn12 = assign30000_e28290_d_n12;
        locals.var_b4soiqde_rv = 0.0;

        let (assign30010_e28309, assign30010_e28309_d_n3, assign30010_e28309_d_n4, assign30010_e28309_d_n5, assign30010_e28309_d_n6, assign30010_e28309_d_n7, assign30010_e28309_d_n8, assign30010_e28309_d_n9, assign30010_e28309_d_n10, assign30010_e28309_d_n11, assign30010_e28309_d_n12,) = {
    if (((((locals.var_guard1424 != 0.0) && (locals.var_guard1432 != 0.0)) && (locals.var_guard1433 == 0.0)) && (locals.var_guard1434 == 0.0)) && (locals.var_guard1435 == 0.0)) {
        let assign30010_e28305: f64 = (locals.var_b4soicdmin * locals.var_t11);
        let assign30010_e28307: f64 = (assign30010_e28305 + locals.var_b4soidt4);
        (assign30010_e28307, (((locals.var_b4soicdmin_dn3 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn3)) + locals.var_b4soidt4_dn3), (((locals.var_b4soicdmin_dn4 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn4)) + locals.var_b4soidt4_dn4), (((locals.var_b4soicdmin_dn5 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn5)) + locals.var_b4soidt4_dn5), (((locals.var_b4soicdmin_dn6 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn6)) + locals.var_b4soidt4_dn6), (((locals.var_b4soicdmin_dn7 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn7)) + locals.var_b4soidt4_dn7), (((locals.var_b4soicdmin_dn8 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn8)) + locals.var_b4soidt4_dn8), (((locals.var_b4soicdmin_dn9 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn9)) + locals.var_b4soidt4_dn9), (((locals.var_b4soicdmin_dn10 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn10)) + locals.var_b4soidt4_dn10), (((locals.var_b4soicdmin_dn11 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn11)) + locals.var_b4soidt4_dn11), (((locals.var_b4soicdmin_dn12 * locals.var_t11) + (locals.var_b4soicdmin * locals.var_t11_dn12)) + locals.var_b4soidt4_dn12),)
    } else {
        (locals.var_b4soiqde, locals.var_b4soiqde_dn3, locals.var_b4soiqde_dn4, locals.var_b4soiqde_dn5, locals.var_b4soiqde_dn6, locals.var_b4soiqde_dn7, locals.var_b4soiqde_dn8, locals.var_b4soiqde_dn9, locals.var_b4soiqde_dn10, locals.var_b4soiqde_dn11, locals.var_b4soiqde_dn12,)
    }
};
        locals.var_b4soiqde = assign30010_e28309;
        locals.var_b4soiqde_dn3 = assign30010_e28309_d_n3;
        locals.var_b4soiqde_dn4 = assign30010_e28309_d_n4;
        locals.var_b4soiqde_dn5 = assign30010_e28309_d_n5;
        locals.var_b4soiqde_dn6 = assign30010_e28309_d_n6;
        locals.var_b4soiqde_dn7 = assign30010_e28309_d_n7;
        locals.var_b4soiqde_dn8 = assign30010_e28309_d_n8;
        locals.var_b4soiqde_dn9 = assign30010_e28309_d_n9;
        locals.var_b4soiqde_dn10 = assign30010_e28309_d_n10;
        locals.var_b4soiqde_dn11 = assign30010_e28309_d_n11;
        locals.var_b4soiqde_dn12 = assign30010_e28309_d_n12;
        locals.var_b4soiqde_rv = 0.0;

        let assign30020_e28312: f64 = if locals.var_t11 < locals.var_pparam_b4soivsdth { 1.0 } else { 0.0 };
        locals.var_guard1436 = assign30020_e28312;
        locals.var_guard1436_rv = 0.0;

        let (assign30030_e28325, assign30030_e28325_d_n3, assign30030_e28325_d_n4, assign30030_e28325_d_n5, assign30030_e28325_d_n6, assign30030_e28325_d_n7, assign30030_e28325_d_n8, assign30030_e28325_d_n9, assign30030_e28325_d_n10, assign30030_e28325_d_n11, assign30030_e28325_d_n12,) = {
    if (((locals.var_guard1424 != 0.0) && (locals.var_guard1432 == 0.0)) && (locals.var_guard1436 != 0.0)) {
        let assign30030_e28322: f64 = (locals.var_t11 - locals.var_pparam_b4soivsdth);
        let assign30030_e28323: f64 = (locals.var_b4soicdmin * assign30030_e28322);
        (assign30030_e28323, ((locals.var_b4soicdmin_dn3 * assign30030_e28322) + (locals.var_b4soicdmin * (locals.var_t11_dn3 - locals.var_pparam_b4soivsdth_dn3))), ((locals.var_b4soicdmin_dn4 * assign30030_e28322) + (locals.var_b4soicdmin * (locals.var_t11_dn4 - locals.var_pparam_b4soivsdth_dn4))), ((locals.var_b4soicdmin_dn5 * assign30030_e28322) + (locals.var_b4soicdmin * (locals.var_t11_dn5 - locals.var_pparam_b4soivsdth_dn5))), ((locals.var_b4soicdmin_dn6 * assign30030_e28322) + (locals.var_b4soicdmin * (locals.var_t11_dn6 - locals.var_pparam_b4soivsdth_dn6))), ((locals.var_b4soicdmin_dn7 * assign30030_e28322) + (locals.var_b4soicdmin * (locals.var_t11_dn7 - locals.var_pparam_b4soivsdth_dn7))), ((locals.var_b4soicdmin_dn8 * assign30030_e28322) + (locals.var_b4soicdmin * (locals.var_t11_dn8 - locals.var_pparam_b4soivsdth_dn8))), ((locals.var_b4soicdmin_dn9 * assign30030_e28322) + (locals.var_b4soicdmin * (locals.var_t11_dn9 - locals.var_pparam_b4soivsdth_dn9))), ((locals.var_b4soicdmin_dn10 * assign30030_e28322) + (locals.var_b4soicdmin * (locals.var_t11_dn10 - locals.var_pparam_b4soivsdth_dn10))), ((locals.var_b4soicdmin_dn11 * assign30030_e28322) + (locals.var_b4soicdmin * (locals.var_t11_dn11 - locals.var_pparam_b4soivsdth_dn11))), ((locals.var_b4soicdmin_dn12 * assign30030_e28322) + (locals.var_b4soicdmin * (locals.var_t11_dn12 - locals.var_pparam_b4soivsdth_dn12))),)
    } else {
        (locals.var_b4soiqde, locals.var_b4soiqde_dn3, locals.var_b4soiqde_dn4, locals.var_b4soiqde_dn5, locals.var_b4soiqde_dn6, locals.var_b4soiqde_dn7, locals.var_b4soiqde_dn8, locals.var_b4soiqde_dn9, locals.var_b4soiqde_dn10, locals.var_b4soiqde_dn11, locals.var_b4soiqde_dn12,)
    }
};
        locals.var_b4soiqde = assign30030_e28325;
        locals.var_b4soiqde_dn3 = assign30030_e28325_d_n3;
        locals.var_b4soiqde_dn4 = assign30030_e28325_d_n4;
        locals.var_b4soiqde_dn5 = assign30030_e28325_d_n5;
        locals.var_b4soiqde_dn6 = assign30030_e28325_d_n6;
        locals.var_b4soiqde_dn7 = assign30030_e28325_d_n7;
        locals.var_b4soiqde_dn8 = assign30030_e28325_d_n8;
        locals.var_b4soiqde_dn9 = assign30030_e28325_d_n9;
        locals.var_b4soiqde_dn10 = assign30030_e28325_d_n10;
        locals.var_b4soiqde_dn11 = assign30030_e28325_d_n11;
        locals.var_b4soiqde_dn12 = assign30030_e28325_d_n12;
        locals.var_b4soiqde_rv = 0.0;

        let assign30040_e28328: f64 = if locals.var_t11 < locals.var_pparam_b4soisdt1 { 1.0 } else { 0.0 };
        locals.var_guard1437 = assign30040_e28328;
        locals.var_guard1437_rv = 0.0;

        let (assign30050_e28342, assign30050_e28342_d_n3, assign30050_e28342_d_n4, assign30050_e28342_d_n5, assign30050_e28342_d_n6, assign30050_e28342_d_n7, assign30050_e28342_d_n8, assign30050_e28342_d_n9, assign30050_e28342_d_n10, assign30050_e28342_d_n11, assign30050_e28342_d_n12,) = {
    if ((((locals.var_guard1424 != 0.0) && (locals.var_guard1432 == 0.0)) && (locals.var_guard1436 == 0.0)) && (locals.var_guard1437 != 0.0)) {
        let assign30050_e28340: f64 = (locals.var_t11 - locals.var_pparam_b4soivsdth);
        (assign30050_e28340, (locals.var_t11_dn3 - locals.var_pparam_b4soivsdth_dn3), (locals.var_t11_dn4 - locals.var_pparam_b4soivsdth_dn4), (locals.var_t11_dn5 - locals.var_pparam_b4soivsdth_dn5), (locals.var_t11_dn6 - locals.var_pparam_b4soivsdth_dn6), (locals.var_t11_dn7 - locals.var_pparam_b4soivsdth_dn7), (locals.var_t11_dn8 - locals.var_pparam_b4soivsdth_dn8), (locals.var_t11_dn9 - locals.var_pparam_b4soivsdth_dn9), (locals.var_t11_dn10 - locals.var_pparam_b4soivsdth_dn10), (locals.var_t11_dn11 - locals.var_pparam_b4soivsdth_dn11), (locals.var_t11_dn12 - locals.var_pparam_b4soivsdth_dn12),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign30050_e28342;
        locals.var_t0__blk808_dn3 = assign30050_e28342_d_n3;
        locals.var_t0__blk808_dn4 = assign30050_e28342_d_n4;
        locals.var_t0__blk808_dn5 = assign30050_e28342_d_n5;
        locals.var_t0__blk808_dn6 = assign30050_e28342_d_n6;
        locals.var_t0__blk808_dn7 = assign30050_e28342_d_n7;
        locals.var_t0__blk808_dn8 = assign30050_e28342_d_n8;
        locals.var_t0__blk808_dn9 = assign30050_e28342_d_n9;
        locals.var_t0__blk808_dn10 = assign30050_e28342_d_n10;
        locals.var_t0__blk808_dn11 = assign30050_e28342_d_n11;
        locals.var_t0__blk808_dn12 = assign30050_e28342_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign30060_e28356, assign30060_e28356_d_n3, assign30060_e28356_d_n4, assign30060_e28356_d_n5, assign30060_e28356_d_n6, assign30060_e28356_d_n7, assign30060_e28356_d_n8, assign30060_e28356_d_n9, assign30060_e28356_d_n10, assign30060_e28356_d_n11, assign30060_e28356_d_n12,) = {
    if ((((locals.var_guard1424 != 0.0) && (locals.var_guard1432 == 0.0)) && (locals.var_guard1436 == 0.0)) && (locals.var_guard1437 != 0.0)) {
        let assign30060_e28354: f64 = (locals.var_t0__blk808 * locals.var_t0__blk808);
        (assign30060_e28354, ((locals.var_t0__blk808_dn3 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn3)), ((locals.var_t0__blk808_dn4 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn4)), ((locals.var_t0__blk808_dn5 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn5)), ((locals.var_t0__blk808_dn6 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn6)), ((locals.var_t0__blk808_dn7 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn7)), ((locals.var_t0__blk808_dn8 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn8)), ((locals.var_t0__blk808_dn9 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn9)), ((locals.var_t0__blk808_dn10 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn10)), ((locals.var_t0__blk808_dn11 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn11)), ((locals.var_t0__blk808_dn12 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn12)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign30060_e28356;
        locals.var_t1__blk809_dn3 = assign30060_e28356_d_n3;
        locals.var_t1__blk809_dn4 = assign30060_e28356_d_n4;
        locals.var_t1__blk809_dn5 = assign30060_e28356_d_n5;
        locals.var_t1__blk809_dn6 = assign30060_e28356_d_n6;
        locals.var_t1__blk809_dn7 = assign30060_e28356_d_n7;
        locals.var_t1__blk809_dn8 = assign30060_e28356_d_n8;
        locals.var_t1__blk809_dn9 = assign30060_e28356_d_n9;
        locals.var_t1__blk809_dn10 = assign30060_e28356_d_n10;
        locals.var_t1__blk809_dn11 = assign30060_e28356_d_n11;
        locals.var_t1__blk809_dn12 = assign30060_e28356_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign30070_e28376, assign30070_e28376_d_n3, assign30070_e28376_d_n4, assign30070_e28376_d_n5, assign30070_e28376_d_n6, assign30070_e28376_d_n7, assign30070_e28376_d_n8, assign30070_e28376_d_n9, assign30070_e28376_d_n10, assign30070_e28376_d_n11, assign30070_e28376_d_n12,) = {
    if ((((locals.var_guard1424 != 0.0) && (locals.var_guard1432 == 0.0)) && (locals.var_guard1436 == 0.0)) && (locals.var_guard1437 != 0.0)) {
        let assign30070_e28370: f64 = (locals.var_pparam_b4soidt2 / 3.0);
        let assign30070_e28372: f64 = (assign30070_e28370 * locals.var_t1__blk809);
        let assign30070_e28373: f64 = (locals.var_b4soicdmin - assign30070_e28372);
        let assign30070_e28374: f64 = (locals.var_t0__blk808 * assign30070_e28373);
        (assign30070_e28374, ((locals.var_t0__blk808_dn3 * assign30070_e28373) + (locals.var_t0__blk808 * (locals.var_b4soicdmin_dn3 - (((locals.var_pparam_b4soidt2_dn3 / 3.0) * locals.var_t1__blk809) + (assign30070_e28370 * locals.var_t1__blk809_dn3))))), ((locals.var_t0__blk808_dn4 * assign30070_e28373) + (locals.var_t0__blk808 * (locals.var_b4soicdmin_dn4 - (((locals.var_pparam_b4soidt2_dn4 / 3.0) * locals.var_t1__blk809) + (assign30070_e28370 * locals.var_t1__blk809_dn4))))), ((locals.var_t0__blk808_dn5 * assign30070_e28373) + (locals.var_t0__blk808 * (locals.var_b4soicdmin_dn5 - (((locals.var_pparam_b4soidt2_dn5 / 3.0) * locals.var_t1__blk809) + (assign30070_e28370 * locals.var_t1__blk809_dn5))))), ((locals.var_t0__blk808_dn6 * assign30070_e28373) + (locals.var_t0__blk808 * (locals.var_b4soicdmin_dn6 - (((locals.var_pparam_b4soidt2_dn6 / 3.0) * locals.var_t1__blk809) + (assign30070_e28370 * locals.var_t1__blk809_dn6))))), ((locals.var_t0__blk808_dn7 * assign30070_e28373) + (locals.var_t0__blk808 * (locals.var_b4soicdmin_dn7 - (((locals.var_pparam_b4soidt2_dn7 / 3.0) * locals.var_t1__blk809) + (assign30070_e28370 * locals.var_t1__blk809_dn7))))), ((locals.var_t0__blk808_dn8 * assign30070_e28373) + (locals.var_t0__blk808 * (locals.var_b4soicdmin_dn8 - (((locals.var_pparam_b4soidt2_dn8 / 3.0) * locals.var_t1__blk809) + (assign30070_e28370 * locals.var_t1__blk809_dn8))))), ((locals.var_t0__blk808_dn9 * assign30070_e28373) + (locals.var_t0__blk808 * (locals.var_b4soicdmin_dn9 - (((locals.var_pparam_b4soidt2_dn9 / 3.0) * locals.var_t1__blk809) + (assign30070_e28370 * locals.var_t1__blk809_dn9))))), ((locals.var_t0__blk808_dn10 * assign30070_e28373) + (locals.var_t0__blk808 * (locals.var_b4soicdmin_dn10 - (((locals.var_pparam_b4soidt2_dn10 / 3.0) * locals.var_t1__blk809) + (assign30070_e28370 * locals.var_t1__blk809_dn10))))), ((locals.var_t0__blk808_dn11 * assign30070_e28373) + (locals.var_t0__blk808 * (locals.var_b4soicdmin_dn11 - (((locals.var_pparam_b4soidt2_dn11 / 3.0) * locals.var_t1__blk809) + (assign30070_e28370 * locals.var_t1__blk809_dn11))))), ((locals.var_t0__blk808_dn12 * assign30070_e28373) + (locals.var_t0__blk808 * (locals.var_b4soicdmin_dn12 - (((locals.var_pparam_b4soidt2_dn12 / 3.0) * locals.var_t1__blk809) + (assign30070_e28370 * locals.var_t1__blk809_dn12))))),)
    } else {
        (locals.var_b4soiqde, locals.var_b4soiqde_dn3, locals.var_b4soiqde_dn4, locals.var_b4soiqde_dn5, locals.var_b4soiqde_dn6, locals.var_b4soiqde_dn7, locals.var_b4soiqde_dn8, locals.var_b4soiqde_dn9, locals.var_b4soiqde_dn10, locals.var_b4soiqde_dn11, locals.var_b4soiqde_dn12,)
    }
};
        locals.var_b4soiqde = assign30070_e28376;
        locals.var_b4soiqde_dn3 = assign30070_e28376_d_n3;
        locals.var_b4soiqde_dn4 = assign30070_e28376_d_n4;
        locals.var_b4soiqde_dn5 = assign30070_e28376_d_n5;
        locals.var_b4soiqde_dn6 = assign30070_e28376_d_n6;
        locals.var_b4soiqde_dn7 = assign30070_e28376_d_n7;
        locals.var_b4soiqde_dn8 = assign30070_e28376_d_n8;
        locals.var_b4soiqde_dn9 = assign30070_e28376_d_n9;
        locals.var_b4soiqde_dn10 = assign30070_e28376_d_n10;
        locals.var_b4soiqde_dn11 = assign30070_e28376_d_n11;
        locals.var_b4soiqde_dn12 = assign30070_e28376_d_n12;
        locals.var_b4soiqde_rv = 0.0;

        let assign30080_e28379: f64 = if locals.var_t11 < locals.var_pparam_b4soivsdfb { 1.0 } else { 0.0 };
        locals.var_guard1438 = assign30080_e28379;
        locals.var_guard1438_rv = 0.0;

        let (assign30090_e28396, assign30090_e28396_d_n3, assign30090_e28396_d_n4, assign30090_e28396_d_n5, assign30090_e28396_d_n6, assign30090_e28396_d_n7, assign30090_e28396_d_n8, assign30090_e28396_d_n9, assign30090_e28396_d_n10, assign30090_e28396_d_n11, assign30090_e28396_d_n12,) = {
    if (((((locals.var_guard1424 != 0.0) && (locals.var_guard1432 == 0.0)) && (locals.var_guard1436 == 0.0)) && (locals.var_guard1437 == 0.0)) && (locals.var_guard1438 != 0.0)) {
        let assign30090_e28394: f64 = (locals.var_t11 - locals.var_pparam_b4soivsdfb);
        (assign30090_e28394, (locals.var_t11_dn3 - locals.var_pparam_b4soivsdfb_dn3), (locals.var_t11_dn4 - locals.var_pparam_b4soivsdfb_dn4), (locals.var_t11_dn5 - locals.var_pparam_b4soivsdfb_dn5), (locals.var_t11_dn6 - locals.var_pparam_b4soivsdfb_dn6), (locals.var_t11_dn7 - locals.var_pparam_b4soivsdfb_dn7), (locals.var_t11_dn8 - locals.var_pparam_b4soivsdfb_dn8), (locals.var_t11_dn9 - locals.var_pparam_b4soivsdfb_dn9), (locals.var_t11_dn10 - locals.var_pparam_b4soivsdfb_dn10), (locals.var_t11_dn11 - locals.var_pparam_b4soivsdfb_dn11), (locals.var_t11_dn12 - locals.var_pparam_b4soivsdfb_dn12),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign30090_e28396;
        locals.var_t0__blk808_dn3 = assign30090_e28396_d_n3;
        locals.var_t0__blk808_dn4 = assign30090_e28396_d_n4;
        locals.var_t0__blk808_dn5 = assign30090_e28396_d_n5;
        locals.var_t0__blk808_dn6 = assign30090_e28396_d_n6;
        locals.var_t0__blk808_dn7 = assign30090_e28396_d_n7;
        locals.var_t0__blk808_dn8 = assign30090_e28396_d_n8;
        locals.var_t0__blk808_dn9 = assign30090_e28396_d_n9;
        locals.var_t0__blk808_dn10 = assign30090_e28396_d_n10;
        locals.var_t0__blk808_dn11 = assign30090_e28396_d_n11;
        locals.var_t0__blk808_dn12 = assign30090_e28396_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign30100_e28413, assign30100_e28413_d_n3, assign30100_e28413_d_n4, assign30100_e28413_d_n5, assign30100_e28413_d_n6, assign30100_e28413_d_n7, assign30100_e28413_d_n8, assign30100_e28413_d_n9, assign30100_e28413_d_n10, assign30100_e28413_d_n11, assign30100_e28413_d_n12,) = {
    if (((((locals.var_guard1424 != 0.0) && (locals.var_guard1432 == 0.0)) && (locals.var_guard1436 == 0.0)) && (locals.var_guard1437 == 0.0)) && (locals.var_guard1438 != 0.0)) {
        let assign30100_e28411: f64 = (locals.var_t0__blk808 * locals.var_t0__blk808);
        (assign30100_e28411, ((locals.var_t0__blk808_dn3 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn3)), ((locals.var_t0__blk808_dn4 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn4)), ((locals.var_t0__blk808_dn5 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn5)), ((locals.var_t0__blk808_dn6 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn6)), ((locals.var_t0__blk808_dn7 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn7)), ((locals.var_t0__blk808_dn8 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn8)), ((locals.var_t0__blk808_dn9 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn9)), ((locals.var_t0__blk808_dn10 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn10)), ((locals.var_t0__blk808_dn11 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn11)), ((locals.var_t0__blk808_dn12 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn12)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign30100_e28413;
        locals.var_t1__blk809_dn3 = assign30100_e28413_d_n3;
        locals.var_t1__blk809_dn4 = assign30100_e28413_d_n4;
        locals.var_t1__blk809_dn5 = assign30100_e28413_d_n5;
        locals.var_t1__blk809_dn6 = assign30100_e28413_d_n6;
        locals.var_t1__blk809_dn7 = assign30100_e28413_d_n7;
        locals.var_t1__blk809_dn8 = assign30100_e28413_d_n8;
        locals.var_t1__blk809_dn9 = assign30100_e28413_d_n9;
        locals.var_t1__blk809_dn10 = assign30100_e28413_d_n10;
        locals.var_t1__blk809_dn11 = assign30100_e28413_d_n11;
        locals.var_t1__blk809_dn12 = assign30100_e28413_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign30110_e28440, assign30110_e28440_d_n3, assign30110_e28440_d_n4, assign30110_e28440_d_n5, assign30110_e28440_d_n6, assign30110_e28440_d_n7, assign30110_e28440_d_n8, assign30110_e28440_d_n9, assign30110_e28440_d_n10, assign30110_e28440_d_n11, assign30110_e28440_d_n12,) = {
    if (((((locals.var_guard1424 != 0.0) && (locals.var_guard1432 == 0.0)) && (locals.var_guard1436 == 0.0)) && (locals.var_guard1437 == 0.0)) && (locals.var_guard1438 != 0.0)) {
        let assign30110_e28428: f64 = (locals.var_b4soicdbox * locals.var_t11);
        let assign30110_e28430: f64 = (assign30110_e28428 + locals.var_b4soidt4);
        let assign30110_e28433: f64 = (locals.var_pparam_b4soidt3 / 3.0);
        let assign30110_e28435: f64 = (assign30110_e28433 * locals.var_t0__blk808);
        let assign30110_e28437: f64 = (assign30110_e28435 * locals.var_t1__blk809);
        let assign30110_e28438: f64 = (assign30110_e28430 + assign30110_e28437);
        (assign30110_e28438, (((locals.var_b4soicdbox * locals.var_t11_dn3) + locals.var_b4soidt4_dn3) + (((((locals.var_pparam_b4soidt3_dn3 / 3.0) * locals.var_t0__blk808) + (assign30110_e28433 * locals.var_t0__blk808_dn3)) * locals.var_t1__blk809) + (assign30110_e28435 * locals.var_t1__blk809_dn3))), (((locals.var_b4soicdbox * locals.var_t11_dn4) + locals.var_b4soidt4_dn4) + (((((locals.var_pparam_b4soidt3_dn4 / 3.0) * locals.var_t0__blk808) + (assign30110_e28433 * locals.var_t0__blk808_dn4)) * locals.var_t1__blk809) + (assign30110_e28435 * locals.var_t1__blk809_dn4))), (((locals.var_b4soicdbox * locals.var_t11_dn5) + locals.var_b4soidt4_dn5) + (((((locals.var_pparam_b4soidt3_dn5 / 3.0) * locals.var_t0__blk808) + (assign30110_e28433 * locals.var_t0__blk808_dn5)) * locals.var_t1__blk809) + (assign30110_e28435 * locals.var_t1__blk809_dn5))), (((locals.var_b4soicdbox * locals.var_t11_dn6) + locals.var_b4soidt4_dn6) + (((((locals.var_pparam_b4soidt3_dn6 / 3.0) * locals.var_t0__blk808) + (assign30110_e28433 * locals.var_t0__blk808_dn6)) * locals.var_t1__blk809) + (assign30110_e28435 * locals.var_t1__blk809_dn6))), (((locals.var_b4soicdbox * locals.var_t11_dn7) + locals.var_b4soidt4_dn7) + (((((locals.var_pparam_b4soidt3_dn7 / 3.0) * locals.var_t0__blk808) + (assign30110_e28433 * locals.var_t0__blk808_dn7)) * locals.var_t1__blk809) + (assign30110_e28435 * locals.var_t1__blk809_dn7))), (((locals.var_b4soicdbox * locals.var_t11_dn8) + locals.var_b4soidt4_dn8) + (((((locals.var_pparam_b4soidt3_dn8 / 3.0) * locals.var_t0__blk808) + (assign30110_e28433 * locals.var_t0__blk808_dn8)) * locals.var_t1__blk809) + (assign30110_e28435 * locals.var_t1__blk809_dn8))), (((locals.var_b4soicdbox * locals.var_t11_dn9) + locals.var_b4soidt4_dn9) + (((((locals.var_pparam_b4soidt3_dn9 / 3.0) * locals.var_t0__blk808) + (assign30110_e28433 * locals.var_t0__blk808_dn9)) * locals.var_t1__blk809) + (assign30110_e28435 * locals.var_t1__blk809_dn9))), (((locals.var_b4soicdbox * locals.var_t11_dn10) + locals.var_b4soidt4_dn10) + (((((locals.var_pparam_b4soidt3_dn10 / 3.0) * locals.var_t0__blk808) + (assign30110_e28433 * locals.var_t0__blk808_dn10)) * locals.var_t1__blk809) + (assign30110_e28435 * locals.var_t1__blk809_dn10))), (((locals.var_b4soicdbox * locals.var_t11_dn11) + locals.var_b4soidt4_dn11) + (((((locals.var_pparam_b4soidt3_dn11 / 3.0) * locals.var_t0__blk808) + (assign30110_e28433 * locals.var_t0__blk808_dn11)) * locals.var_t1__blk809) + (assign30110_e28435 * locals.var_t1__blk809_dn11))), (((locals.var_b4soicdbox * locals.var_t11_dn12) + locals.var_b4soidt4_dn12) + (((((locals.var_pparam_b4soidt3_dn12 / 3.0) * locals.var_t0__blk808) + (assign30110_e28433 * locals.var_t0__blk808_dn12)) * locals.var_t1__blk809) + (assign30110_e28435 * locals.var_t1__blk809_dn12))),)
    } else {
        (locals.var_b4soiqde, locals.var_b4soiqde_dn3, locals.var_b4soiqde_dn4, locals.var_b4soiqde_dn5, locals.var_b4soiqde_dn6, locals.var_b4soiqde_dn7, locals.var_b4soiqde_dn8, locals.var_b4soiqde_dn9, locals.var_b4soiqde_dn10, locals.var_b4soiqde_dn11, locals.var_b4soiqde_dn12,)
    }
};
        locals.var_b4soiqde = assign30110_e28440;
        locals.var_b4soiqde_dn3 = assign30110_e28440_d_n3;
        locals.var_b4soiqde_dn4 = assign30110_e28440_d_n4;
        locals.var_b4soiqde_dn5 = assign30110_e28440_d_n5;
        locals.var_b4soiqde_dn6 = assign30110_e28440_d_n6;
        locals.var_b4soiqde_dn7 = assign30110_e28440_d_n7;
        locals.var_b4soiqde_dn8 = assign30110_e28440_d_n8;
        locals.var_b4soiqde_dn9 = assign30110_e28440_d_n9;
        locals.var_b4soiqde_dn10 = assign30110_e28440_d_n10;
        locals.var_b4soiqde_dn11 = assign30110_e28440_d_n11;
        locals.var_b4soiqde_dn12 = assign30110_e28440_d_n12;
        locals.var_b4soiqde_rv = 0.0;

        let (assign30120_e28460, assign30120_e28460_d_n3, assign30120_e28460_d_n4, assign30120_e28460_d_n5, assign30120_e28460_d_n6, assign30120_e28460_d_n7, assign30120_e28460_d_n8, assign30120_e28460_d_n9, assign30120_e28460_d_n10, assign30120_e28460_d_n11, assign30120_e28460_d_n12,) = {
    if (((((locals.var_guard1424 != 0.0) && (locals.var_guard1432 == 0.0)) && (locals.var_guard1436 == 0.0)) && (locals.var_guard1437 == 0.0)) && (locals.var_guard1438 == 0.0)) {
        let assign30120_e28456: f64 = (locals.var_b4soicdbox * locals.var_t11);
        let assign30120_e28458: f64 = (assign30120_e28456 + locals.var_b4soidt4);
        (assign30120_e28458, ((locals.var_b4soicdbox * locals.var_t11_dn3) + locals.var_b4soidt4_dn3), ((locals.var_b4soicdbox * locals.var_t11_dn4) + locals.var_b4soidt4_dn4), ((locals.var_b4soicdbox * locals.var_t11_dn5) + locals.var_b4soidt4_dn5), ((locals.var_b4soicdbox * locals.var_t11_dn6) + locals.var_b4soidt4_dn6), ((locals.var_b4soicdbox * locals.var_t11_dn7) + locals.var_b4soidt4_dn7), ((locals.var_b4soicdbox * locals.var_t11_dn8) + locals.var_b4soidt4_dn8), ((locals.var_b4soicdbox * locals.var_t11_dn9) + locals.var_b4soidt4_dn9), ((locals.var_b4soicdbox * locals.var_t11_dn10) + locals.var_b4soidt4_dn10), ((locals.var_b4soicdbox * locals.var_t11_dn11) + locals.var_b4soidt4_dn11), ((locals.var_b4soicdbox * locals.var_t11_dn12) + locals.var_b4soidt4_dn12),)
    } else {
        (locals.var_b4soiqde, locals.var_b4soiqde_dn3, locals.var_b4soiqde_dn4, locals.var_b4soiqde_dn5, locals.var_b4soiqde_dn6, locals.var_b4soiqde_dn7, locals.var_b4soiqde_dn8, locals.var_b4soiqde_dn9, locals.var_b4soiqde_dn10, locals.var_b4soiqde_dn11, locals.var_b4soiqde_dn12,)
    }
};
        locals.var_b4soiqde = assign30120_e28460;
        locals.var_b4soiqde_dn3 = assign30120_e28460_d_n3;
        locals.var_b4soiqde_dn4 = assign30120_e28460_d_n4;
        locals.var_b4soiqde_dn5 = assign30120_e28460_d_n5;
        locals.var_b4soiqde_dn6 = assign30120_e28460_d_n6;
        locals.var_b4soiqde_dn7 = assign30120_e28460_d_n7;
        locals.var_b4soiqde_dn8 = assign30120_e28460_d_n8;
        locals.var_b4soiqde_dn9 = assign30120_e28460_d_n9;
        locals.var_b4soiqde_dn10 = assign30120_e28460_d_n10;
        locals.var_b4soiqde_dn11 = assign30120_e28460_d_n11;
        locals.var_b4soiqde_dn12 = assign30120_e28460_d_n12;
        locals.var_b4soiqde_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_93(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign30130_e28467, assign30130_e28467_d_n3, assign30130_e28467_d_n4, assign30130_e28467_d_n5, assign30130_e28467_d_n6, assign30130_e28467_d_n7, assign30130_e28467_d_n8, assign30130_e28467_d_n9, assign30130_e28467_d_n10, assign30130_e28467_d_n11, assign30130_e28467_d_n12,) = {
    if (locals.var_guard1424 == 0.0) {
        let assign30130_e28465: f64 = (locals.var_b4soicsbox * locals.var_t10__blk818);
        (assign30130_e28465, (locals.var_b4soicsbox * locals.var_t10__blk818_dn3), (locals.var_b4soicsbox * locals.var_t10__blk818_dn4), (locals.var_b4soicsbox * locals.var_t10__blk818_dn5), (locals.var_b4soicsbox * locals.var_t10__blk818_dn6), (locals.var_b4soicsbox * locals.var_t10__blk818_dn7), (locals.var_b4soicsbox * locals.var_t10__blk818_dn8), (locals.var_b4soicsbox * locals.var_t10__blk818_dn9), (locals.var_b4soicsbox * locals.var_t10__blk818_dn10), (locals.var_b4soicsbox * locals.var_t10__blk818_dn11), (locals.var_b4soicsbox * locals.var_t10__blk818_dn12),)
    } else {
        (locals.var_b4soiqse, locals.var_b4soiqse_dn3, locals.var_b4soiqse_dn4, locals.var_b4soiqse_dn5, locals.var_b4soiqse_dn6, locals.var_b4soiqse_dn7, locals.var_b4soiqse_dn8, locals.var_b4soiqse_dn9, locals.var_b4soiqse_dn10, locals.var_b4soiqse_dn11, locals.var_b4soiqse_dn12,)
    }
};
        locals.var_b4soiqse = assign30130_e28467;
        locals.var_b4soiqse_dn3 = assign30130_e28467_d_n3;
        locals.var_b4soiqse_dn4 = assign30130_e28467_d_n4;
        locals.var_b4soiqse_dn5 = assign30130_e28467_d_n5;
        locals.var_b4soiqse_dn6 = assign30130_e28467_d_n6;
        locals.var_b4soiqse_dn7 = assign30130_e28467_d_n7;
        locals.var_b4soiqse_dn8 = assign30130_e28467_d_n8;
        locals.var_b4soiqse_dn9 = assign30130_e28467_d_n9;
        locals.var_b4soiqse_dn10 = assign30130_e28467_d_n10;
        locals.var_b4soiqse_dn11 = assign30130_e28467_d_n11;
        locals.var_b4soiqse_dn12 = assign30130_e28467_d_n12;
        locals.var_b4soiqse_rv = 0.0;

        let (assign30140_e28474, assign30140_e28474_d_n3, assign30140_e28474_d_n4, assign30140_e28474_d_n5, assign30140_e28474_d_n6, assign30140_e28474_d_n7, assign30140_e28474_d_n8, assign30140_e28474_d_n9, assign30140_e28474_d_n10, assign30140_e28474_d_n11, assign30140_e28474_d_n12,) = {
    if (locals.var_guard1424 == 0.0) {
        let assign30140_e28472: f64 = (locals.var_b4soicdbox * locals.var_t11);
        (assign30140_e28472, (locals.var_b4soicdbox * locals.var_t11_dn3), (locals.var_b4soicdbox * locals.var_t11_dn4), (locals.var_b4soicdbox * locals.var_t11_dn5), (locals.var_b4soicdbox * locals.var_t11_dn6), (locals.var_b4soicdbox * locals.var_t11_dn7), (locals.var_b4soicdbox * locals.var_t11_dn8), (locals.var_b4soicdbox * locals.var_t11_dn9), (locals.var_b4soicdbox * locals.var_t11_dn10), (locals.var_b4soicdbox * locals.var_t11_dn11), (locals.var_b4soicdbox * locals.var_t11_dn12),)
    } else {
        (locals.var_b4soiqde, locals.var_b4soiqde_dn3, locals.var_b4soiqde_dn4, locals.var_b4soiqde_dn5, locals.var_b4soiqde_dn6, locals.var_b4soiqde_dn7, locals.var_b4soiqde_dn8, locals.var_b4soiqde_dn9, locals.var_b4soiqde_dn10, locals.var_b4soiqde_dn11, locals.var_b4soiqde_dn12,)
    }
};
        locals.var_b4soiqde = assign30140_e28474;
        locals.var_b4soiqde_dn3 = assign30140_e28474_d_n3;
        locals.var_b4soiqde_dn4 = assign30140_e28474_d_n4;
        locals.var_b4soiqde_dn5 = assign30140_e28474_d_n5;
        locals.var_b4soiqde_dn6 = assign30140_e28474_d_n6;
        locals.var_b4soiqde_dn7 = assign30140_e28474_d_n7;
        locals.var_b4soiqde_dn8 = assign30140_e28474_d_n8;
        locals.var_b4soiqde_dn9 = assign30140_e28474_d_n9;
        locals.var_b4soiqde_dn10 = assign30140_e28474_d_n10;
        locals.var_b4soiqde_dn11 = assign30140_e28474_d_n11;
        locals.var_b4soiqde_dn12 = assign30140_e28474_d_n12;
        locals.var_b4soiqde_rv = 0.0;

        let assign30150_e28478: f64 = (locals.var_b4soicsesw * locals.var_t10__blk818);
        let assign30150_e28479: f64 = (locals.var_b4soiqse + assign30150_e28478);
        locals.var_b4soiqse = assign30150_e28479;
        locals.var_b4soiqse_dn3 = (locals.var_b4soiqse_dn3 + ((locals.var_b4soicsesw_dn3 * locals.var_t10__blk818) + (locals.var_b4soicsesw * locals.var_t10__blk818_dn3)));
        locals.var_b4soiqse_dn4 = (locals.var_b4soiqse_dn4 + ((locals.var_b4soicsesw_dn4 * locals.var_t10__blk818) + (locals.var_b4soicsesw * locals.var_t10__blk818_dn4)));
        locals.var_b4soiqse_dn5 = (locals.var_b4soiqse_dn5 + ((locals.var_b4soicsesw_dn5 * locals.var_t10__blk818) + (locals.var_b4soicsesw * locals.var_t10__blk818_dn5)));
        locals.var_b4soiqse_dn6 = (locals.var_b4soiqse_dn6 + ((locals.var_b4soicsesw_dn6 * locals.var_t10__blk818) + (locals.var_b4soicsesw * locals.var_t10__blk818_dn6)));
        locals.var_b4soiqse_dn7 = (locals.var_b4soiqse_dn7 + ((locals.var_b4soicsesw_dn7 * locals.var_t10__blk818) + (locals.var_b4soicsesw * locals.var_t10__blk818_dn7)));
        locals.var_b4soiqse_dn8 = (locals.var_b4soiqse_dn8 + ((locals.var_b4soicsesw_dn8 * locals.var_t10__blk818) + (locals.var_b4soicsesw * locals.var_t10__blk818_dn8)));
        locals.var_b4soiqse_dn9 = (locals.var_b4soiqse_dn9 + ((locals.var_b4soicsesw_dn9 * locals.var_t10__blk818) + (locals.var_b4soicsesw * locals.var_t10__blk818_dn9)));
        locals.var_b4soiqse_dn10 = (locals.var_b4soiqse_dn10 + ((locals.var_b4soicsesw_dn10 * locals.var_t10__blk818) + (locals.var_b4soicsesw * locals.var_t10__blk818_dn10)));
        locals.var_b4soiqse_dn11 = (locals.var_b4soiqse_dn11 + ((locals.var_b4soicsesw_dn11 * locals.var_t10__blk818) + (locals.var_b4soicsesw * locals.var_t10__blk818_dn11)));
        locals.var_b4soiqse_dn12 = (locals.var_b4soiqse_dn12 + ((locals.var_b4soicsesw_dn12 * locals.var_t10__blk818) + (locals.var_b4soicsesw * locals.var_t10__blk818_dn12)));
        locals.var_b4soiqse_rv = 0.0;

        let assign30160_e28483: f64 = (locals.var_b4soicdesw * locals.var_t11);
        let assign30160_e28484: f64 = (locals.var_b4soiqde + assign30160_e28483);
        locals.var_b4soiqde = assign30160_e28484;
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
        locals.var_b4soiqde_rv = 0.0;

        let assign30170_e28487: f64 = if p.p39 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1439 = assign30170_e28487;
        locals.var_guard1439_rv = 0.0;

        let (assign30180_e28493, assign30180_e28493_d_n3, assign30180_e28493_d_n4, assign30180_e28493_d_n5, assign30180_e28493_d_n6, assign30180_e28493_d_n7, assign30180_e28493_d_n8, assign30180_e28493_d_n9, assign30180_e28493_d_n10, assign30180_e28493_d_n11, assign30180_e28493_d_n12,) = {
    if (locals.var_guard1439 != 0.0) {
        let assign30180_e28491: f64 = (locals.var_vgmd + 0.02);
        (assign30180_e28491, 0.0, 0.0, 0.0, 0.0, locals.var_vgmd_dn7, locals.var_vgmd_dn8, 0.0, locals.var_vgmd_dn10, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign30180_e28493;
        locals.var_t0__blk808_dn3 = assign30180_e28493_d_n3;
        locals.var_t0__blk808_dn4 = assign30180_e28493_d_n4;
        locals.var_t0__blk808_dn5 = assign30180_e28493_d_n5;
        locals.var_t0__blk808_dn6 = assign30180_e28493_d_n6;
        locals.var_t0__blk808_dn7 = assign30180_e28493_d_n7;
        locals.var_t0__blk808_dn8 = assign30180_e28493_d_n8;
        locals.var_t0__blk808_dn9 = assign30180_e28493_d_n9;
        locals.var_t0__blk808_dn10 = assign30180_e28493_d_n10;
        locals.var_t0__blk808_dn11 = assign30180_e28493_d_n11;
        locals.var_t0__blk808_dn12 = assign30180_e28493_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign30190_e28500, assign30190_e28500_d_n3, assign30190_e28500_d_n4, assign30190_e28500_d_n5, assign30190_e28500_d_n6, assign30190_e28500_d_n7, assign30190_e28500_d_n8, assign30190_e28500_d_n9, assign30190_e28500_d_n10, assign30190_e28500_d_n11, assign30190_e28500_d_n12,) = {
    if (locals.var_guard1439 == 0.0) {
        let assign30190_e28498: f64 = (locals.var_vgd + 0.02);
        (assign30190_e28498, 0.0, 0.0, 0.0, 0.0, locals.var_vgd_dn7, locals.var_vgd_dn8, locals.var_vgd_dn9, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign30190_e28500;
        locals.var_t0__blk808_dn3 = assign30190_e28500_d_n3;
        locals.var_t0__blk808_dn4 = assign30190_e28500_d_n4;
        locals.var_t0__blk808_dn5 = assign30190_e28500_d_n5;
        locals.var_t0__blk808_dn6 = assign30190_e28500_d_n6;
        locals.var_t0__blk808_dn7 = assign30190_e28500_d_n7;
        locals.var_t0__blk808_dn8 = assign30190_e28500_d_n8;
        locals.var_t0__blk808_dn9 = assign30190_e28500_d_n9;
        locals.var_t0__blk808_dn10 = assign30190_e28500_d_n10;
        locals.var_t0__blk808_dn11 = assign30190_e28500_d_n11;
        locals.var_t0__blk808_dn12 = assign30190_e28500_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let assign30200_e28503: f64 = (locals.var_t0__blk808 * locals.var_t0__blk808);
        let assign30200_e28506: f64 = (4.0 * 0.02);
        let assign30200_e28507: f64 = (assign30200_e28503 + assign30200_e28506);
        let assign30200_e28508: f64 = (assign30200_e28507).sqrt();
        locals.var_t1__blk809 = assign30200_e28508;
        locals.var_t1__blk809_dn3 = (((locals.var_t0__blk808_dn3 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn3)) / (2.0 * assign30200_e28508));
        locals.var_t1__blk809_dn4 = (((locals.var_t0__blk808_dn4 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn4)) / (2.0 * assign30200_e28508));
        locals.var_t1__blk809_dn5 = (((locals.var_t0__blk808_dn5 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn5)) / (2.0 * assign30200_e28508));
        locals.var_t1__blk809_dn6 = (((locals.var_t0__blk808_dn6 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn6)) / (2.0 * assign30200_e28508));
        locals.var_t1__blk809_dn7 = (((locals.var_t0__blk808_dn7 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn7)) / (2.0 * assign30200_e28508));
        locals.var_t1__blk809_dn8 = (((locals.var_t0__blk808_dn8 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn8)) / (2.0 * assign30200_e28508));
        locals.var_t1__blk809_dn9 = (((locals.var_t0__blk808_dn9 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn9)) / (2.0 * assign30200_e28508));
        locals.var_t1__blk809_dn10 = (((locals.var_t0__blk808_dn10 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn10)) / (2.0 * assign30200_e28508));
        locals.var_t1__blk809_dn11 = (((locals.var_t0__blk808_dn11 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn11)) / (2.0 * assign30200_e28508));
        locals.var_t1__blk809_dn12 = (((locals.var_t0__blk808_dn12 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn12)) / (2.0 * assign30200_e28508));
        locals.var_t1__blk809_rv = 0.0;

        let assign30210_e28512: f64 = (locals.var_t0__blk808 - locals.var_t1__blk809);
        let assign30210_e28513: f64 = (0.5 * assign30210_e28512);
        locals.var_t2__blk810 = assign30210_e28513;
        locals.var_t2__blk810_dn3 = (0.5 * (locals.var_t0__blk808_dn3 - locals.var_t1__blk809_dn3));
        locals.var_t2__blk810_dn4 = (0.5 * (locals.var_t0__blk808_dn4 - locals.var_t1__blk809_dn4));
        locals.var_t2__blk810_dn5 = (0.5 * (locals.var_t0__blk808_dn5 - locals.var_t1__blk809_dn5));
        locals.var_t2__blk810_dn6 = (0.5 * (locals.var_t0__blk808_dn6 - locals.var_t1__blk809_dn6));
        locals.var_t2__blk810_dn7 = (0.5 * (locals.var_t0__blk808_dn7 - locals.var_t1__blk809_dn7));
        locals.var_t2__blk810_dn8 = (0.5 * (locals.var_t0__blk808_dn8 - locals.var_t1__blk809_dn8));
        locals.var_t2__blk810_dn9 = (0.5 * (locals.var_t0__blk808_dn9 - locals.var_t1__blk809_dn9));
        locals.var_t2__blk810_dn10 = (0.5 * (locals.var_t0__blk808_dn10 - locals.var_t1__blk809_dn10));
        locals.var_t2__blk810_dn11 = (0.5 * (locals.var_t0__blk808_dn11 - locals.var_t1__blk809_dn11));
        locals.var_t2__blk810_dn12 = (0.5 * (locals.var_t0__blk808_dn12 - locals.var_t1__blk809_dn12));
        locals.var_t2__blk810_rv = 0.0;

        let assign30220_e28516: f64 = (locals.var_pparam_b4soiwdiodcv * locals.var_pparam_b4soicgdl);
        locals.var_t3__blk811 = assign30220_e28516;
        locals.var_t3__blk811_dn3 = ((locals.var_pparam_b4soiwdiodcv_dn3 * locals.var_pparam_b4soicgdl) + (locals.var_pparam_b4soiwdiodcv * locals.var_pparam_b4soicgdl_dn3));
        locals.var_t3__blk811_dn4 = ((locals.var_pparam_b4soiwdiodcv_dn4 * locals.var_pparam_b4soicgdl) + (locals.var_pparam_b4soiwdiodcv * locals.var_pparam_b4soicgdl_dn4));
        locals.var_t3__blk811_dn5 = ((locals.var_pparam_b4soiwdiodcv_dn5 * locals.var_pparam_b4soicgdl) + (locals.var_pparam_b4soiwdiodcv * locals.var_pparam_b4soicgdl_dn5));
        locals.var_t3__blk811_dn6 = ((locals.var_pparam_b4soiwdiodcv_dn6 * locals.var_pparam_b4soicgdl) + (locals.var_pparam_b4soiwdiodcv * locals.var_pparam_b4soicgdl_dn6));
        locals.var_t3__blk811_dn7 = ((locals.var_pparam_b4soiwdiodcv_dn7 * locals.var_pparam_b4soicgdl) + (locals.var_pparam_b4soiwdiodcv * locals.var_pparam_b4soicgdl_dn7));
        locals.var_t3__blk811_dn8 = ((locals.var_pparam_b4soiwdiodcv_dn8 * locals.var_pparam_b4soicgdl) + (locals.var_pparam_b4soiwdiodcv * locals.var_pparam_b4soicgdl_dn8));
        locals.var_t3__blk811_dn9 = ((locals.var_pparam_b4soiwdiodcv_dn9 * locals.var_pparam_b4soicgdl) + (locals.var_pparam_b4soiwdiodcv * locals.var_pparam_b4soicgdl_dn9));
        locals.var_t3__blk811_dn10 = ((locals.var_pparam_b4soiwdiodcv_dn10 * locals.var_pparam_b4soicgdl) + (locals.var_pparam_b4soiwdiodcv * locals.var_pparam_b4soicgdl_dn10));
        locals.var_t3__blk811_dn11 = ((locals.var_pparam_b4soiwdiodcv_dn11 * locals.var_pparam_b4soicgdl) + (locals.var_pparam_b4soiwdiodcv * locals.var_pparam_b4soicgdl_dn11));
        locals.var_t3__blk811_dn12 = ((locals.var_pparam_b4soiwdiodcv_dn12 * locals.var_pparam_b4soicgdl) + (locals.var_pparam_b4soiwdiodcv * locals.var_pparam_b4soicgdl_dn12));
        locals.var_t3__blk811_rv = 0.0;

        let assign30230_e28520: f64 = (4.0 * locals.var_t2__blk810);
        let assign30230_e28522: f64 = (assign30230_e28520 / locals.var_pparam_b4soickappa);
        let assign30230_e28523: f64 = (1.0 - assign30230_e28522);
        let assign30230_e28524: f64 = (assign30230_e28523).sqrt();
        locals.var_t4__blk812 = assign30230_e28524;
        locals.var_t4__blk812_dn3 = ((-((((4.0 * locals.var_t2__blk810_dn3) * locals.var_pparam_b4soickappa) - (assign30230_e28520 * locals.var_pparam_b4soickappa_dn3)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign30230_e28524));
        locals.var_t4__blk812_dn4 = ((-((((4.0 * locals.var_t2__blk810_dn4) * locals.var_pparam_b4soickappa) - (assign30230_e28520 * locals.var_pparam_b4soickappa_dn4)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign30230_e28524));
        locals.var_t4__blk812_dn5 = ((-((((4.0 * locals.var_t2__blk810_dn5) * locals.var_pparam_b4soickappa) - (assign30230_e28520 * locals.var_pparam_b4soickappa_dn5)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign30230_e28524));
        locals.var_t4__blk812_dn6 = ((-((((4.0 * locals.var_t2__blk810_dn6) * locals.var_pparam_b4soickappa) - (assign30230_e28520 * locals.var_pparam_b4soickappa_dn6)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign30230_e28524));
        locals.var_t4__blk812_dn7 = ((-((((4.0 * locals.var_t2__blk810_dn7) * locals.var_pparam_b4soickappa) - (assign30230_e28520 * locals.var_pparam_b4soickappa_dn7)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign30230_e28524));
        locals.var_t4__blk812_dn8 = ((-((((4.0 * locals.var_t2__blk810_dn8) * locals.var_pparam_b4soickappa) - (assign30230_e28520 * locals.var_pparam_b4soickappa_dn8)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign30230_e28524));
        locals.var_t4__blk812_dn9 = ((-((((4.0 * locals.var_t2__blk810_dn9) * locals.var_pparam_b4soickappa) - (assign30230_e28520 * locals.var_pparam_b4soickappa_dn9)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign30230_e28524));
        locals.var_t4__blk812_dn10 = ((-((((4.0 * locals.var_t2__blk810_dn10) * locals.var_pparam_b4soickappa) - (assign30230_e28520 * locals.var_pparam_b4soickappa_dn10)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign30230_e28524));
        locals.var_t4__blk812_dn11 = ((-((((4.0 * locals.var_t2__blk810_dn11) * locals.var_pparam_b4soickappa) - (assign30230_e28520 * locals.var_pparam_b4soickappa_dn11)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign30230_e28524));
        locals.var_t4__blk812_dn12 = ((-((((4.0 * locals.var_t2__blk810_dn12) * locals.var_pparam_b4soickappa) - (assign30230_e28520 * locals.var_pparam_b4soickappa_dn12)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign30230_e28524));
        locals.var_t4__blk812_rv = 0.0;

        let assign30240_e28527: f64 = if p.p39 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1440 = assign30240_e28527;
        locals.var_guard1440_rv = 0.0;

        let (assign30250_e28547, assign30250_e28547_d_n3, assign30250_e28547_d_n4, assign30250_e28547_d_n5, assign30250_e28547_d_n6, assign30250_e28547_d_n7, assign30250_e28547_d_n8, assign30250_e28547_d_n9, assign30250_e28547_d_n10, assign30250_e28547_d_n11, assign30250_e28547_d_n12,) = {
    if (locals.var_guard1440 != 0.0) {
        let assign30250_e28531: f64 = (locals.var_pparam_b4soicgdo + locals.var_t3__blk811);
        let assign30250_e28533: f64 = (assign30250_e28531 * locals.var_vgmd);
        let assign30250_e28538: f64 = (0.5 * locals.var_pparam_b4soickappa);
        let assign30250_e28541: f64 = (locals.var_t4__blk812 - 1.0);
        let assign30250_e28542: f64 = (assign30250_e28538 * assign30250_e28541);
        let assign30250_e28543: f64 = (locals.var_t2__blk810 + assign30250_e28542);
        let assign30250_e28544: f64 = (locals.var_t3__blk811 * assign30250_e28543);
        let assign30250_e28545: f64 = (assign30250_e28533 - assign30250_e28544);
        (assign30250_e28545, (((locals.var_pparam_b4soicgdo_dn3 + locals.var_t3__blk811_dn3) * locals.var_vgmd) - ((locals.var_t3__blk811_dn3 * assign30250_e28543) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn3 + (((0.5 * locals.var_pparam_b4soickappa_dn3) * assign30250_e28541) + (assign30250_e28538 * locals.var_t4__blk812_dn3)))))), (((locals.var_pparam_b4soicgdo_dn4 + locals.var_t3__blk811_dn4) * locals.var_vgmd) - ((locals.var_t3__blk811_dn4 * assign30250_e28543) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn4 + (((0.5 * locals.var_pparam_b4soickappa_dn4) * assign30250_e28541) + (assign30250_e28538 * locals.var_t4__blk812_dn4)))))), (((locals.var_pparam_b4soicgdo_dn5 + locals.var_t3__blk811_dn5) * locals.var_vgmd) - ((locals.var_t3__blk811_dn5 * assign30250_e28543) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn5 + (((0.5 * locals.var_pparam_b4soickappa_dn5) * assign30250_e28541) + (assign30250_e28538 * locals.var_t4__blk812_dn5)))))), (((locals.var_pparam_b4soicgdo_dn6 + locals.var_t3__blk811_dn6) * locals.var_vgmd) - ((locals.var_t3__blk811_dn6 * assign30250_e28543) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn6 + (((0.5 * locals.var_pparam_b4soickappa_dn6) * assign30250_e28541) + (assign30250_e28538 * locals.var_t4__blk812_dn6)))))), ((((locals.var_pparam_b4soicgdo_dn7 + locals.var_t3__blk811_dn7) * locals.var_vgmd) + (assign30250_e28531 * locals.var_vgmd_dn7)) - ((locals.var_t3__blk811_dn7 * assign30250_e28543) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn7 + (((0.5 * locals.var_pparam_b4soickappa_dn7) * assign30250_e28541) + (assign30250_e28538 * locals.var_t4__blk812_dn7)))))), ((((locals.var_pparam_b4soicgdo_dn8 + locals.var_t3__blk811_dn8) * locals.var_vgmd) + (assign30250_e28531 * locals.var_vgmd_dn8)) - ((locals.var_t3__blk811_dn8 * assign30250_e28543) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn8 + (((0.5 * locals.var_pparam_b4soickappa_dn8) * assign30250_e28541) + (assign30250_e28538 * locals.var_t4__blk812_dn8)))))), (((locals.var_pparam_b4soicgdo_dn9 + locals.var_t3__blk811_dn9) * locals.var_vgmd) - ((locals.var_t3__blk811_dn9 * assign30250_e28543) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn9 + (((0.5 * locals.var_pparam_b4soickappa_dn9) * assign30250_e28541) + (assign30250_e28538 * locals.var_t4__blk812_dn9)))))), ((((locals.var_pparam_b4soicgdo_dn10 + locals.var_t3__blk811_dn10) * locals.var_vgmd) + (assign30250_e28531 * locals.var_vgmd_dn10)) - ((locals.var_t3__blk811_dn10 * assign30250_e28543) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn10 + (((0.5 * locals.var_pparam_b4soickappa_dn10) * assign30250_e28541) + (assign30250_e28538 * locals.var_t4__blk812_dn10)))))), (((locals.var_pparam_b4soicgdo_dn11 + locals.var_t3__blk811_dn11) * locals.var_vgmd) - ((locals.var_t3__blk811_dn11 * assign30250_e28543) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn11 + (((0.5 * locals.var_pparam_b4soickappa_dn11) * assign30250_e28541) + (assign30250_e28538 * locals.var_t4__blk812_dn11)))))), (((locals.var_pparam_b4soicgdo_dn12 + locals.var_t3__blk811_dn12) * locals.var_vgmd) - ((locals.var_t3__blk811_dn12 * assign30250_e28543) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn12 + (((0.5 * locals.var_pparam_b4soickappa_dn12) * assign30250_e28541) + (assign30250_e28538 * locals.var_t4__blk812_dn12)))))),)
    } else {
        (locals.var_qgdo, locals.var_qgdo_dn3, locals.var_qgdo_dn4, locals.var_qgdo_dn5, locals.var_qgdo_dn6, locals.var_qgdo_dn7, locals.var_qgdo_dn8, locals.var_qgdo_dn9, locals.var_qgdo_dn10, locals.var_qgdo_dn11, locals.var_qgdo_dn12,)
    }
};
        locals.var_qgdo = assign30250_e28547;
        locals.var_qgdo_dn3 = assign30250_e28547_d_n3;
        locals.var_qgdo_dn4 = assign30250_e28547_d_n4;
        locals.var_qgdo_dn5 = assign30250_e28547_d_n5;
        locals.var_qgdo_dn6 = assign30250_e28547_d_n6;
        locals.var_qgdo_dn7 = assign30250_e28547_d_n7;
        locals.var_qgdo_dn8 = assign30250_e28547_d_n8;
        locals.var_qgdo_dn9 = assign30250_e28547_d_n9;
        locals.var_qgdo_dn10 = assign30250_e28547_d_n10;
        locals.var_qgdo_dn11 = assign30250_e28547_d_n11;
        locals.var_qgdo_dn12 = assign30250_e28547_d_n12;
        locals.var_qgdo_rv = 0.0;

        let (assign30260_e28568, assign30260_e28568_d_n3, assign30260_e28568_d_n4, assign30260_e28568_d_n5, assign30260_e28568_d_n6, assign30260_e28568_d_n7, assign30260_e28568_d_n8, assign30260_e28568_d_n9, assign30260_e28568_d_n10, assign30260_e28568_d_n11, assign30260_e28568_d_n12,) = {
    if (locals.var_guard1440 == 0.0) {
        let assign30260_e28552: f64 = (locals.var_pparam_b4soicgdo + locals.var_t3__blk811);
        let assign30260_e28554: f64 = (assign30260_e28552 * locals.var_vgd);
        let assign30260_e28559: f64 = (0.5 * locals.var_pparam_b4soickappa);
        let assign30260_e28562: f64 = (locals.var_t4__blk812 - 1.0);
        let assign30260_e28563: f64 = (assign30260_e28559 * assign30260_e28562);
        let assign30260_e28564: f64 = (locals.var_t2__blk810 + assign30260_e28563);
        let assign30260_e28565: f64 = (locals.var_t3__blk811 * assign30260_e28564);
        let assign30260_e28566: f64 = (assign30260_e28554 - assign30260_e28565);
        (assign30260_e28566, (((locals.var_pparam_b4soicgdo_dn3 + locals.var_t3__blk811_dn3) * locals.var_vgd) - ((locals.var_t3__blk811_dn3 * assign30260_e28564) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn3 + (((0.5 * locals.var_pparam_b4soickappa_dn3) * assign30260_e28562) + (assign30260_e28559 * locals.var_t4__blk812_dn3)))))), (((locals.var_pparam_b4soicgdo_dn4 + locals.var_t3__blk811_dn4) * locals.var_vgd) - ((locals.var_t3__blk811_dn4 * assign30260_e28564) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn4 + (((0.5 * locals.var_pparam_b4soickappa_dn4) * assign30260_e28562) + (assign30260_e28559 * locals.var_t4__blk812_dn4)))))), (((locals.var_pparam_b4soicgdo_dn5 + locals.var_t3__blk811_dn5) * locals.var_vgd) - ((locals.var_t3__blk811_dn5 * assign30260_e28564) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn5 + (((0.5 * locals.var_pparam_b4soickappa_dn5) * assign30260_e28562) + (assign30260_e28559 * locals.var_t4__blk812_dn5)))))), (((locals.var_pparam_b4soicgdo_dn6 + locals.var_t3__blk811_dn6) * locals.var_vgd) - ((locals.var_t3__blk811_dn6 * assign30260_e28564) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn6 + (((0.5 * locals.var_pparam_b4soickappa_dn6) * assign30260_e28562) + (assign30260_e28559 * locals.var_t4__blk812_dn6)))))), ((((locals.var_pparam_b4soicgdo_dn7 + locals.var_t3__blk811_dn7) * locals.var_vgd) + (assign30260_e28552 * locals.var_vgd_dn7)) - ((locals.var_t3__blk811_dn7 * assign30260_e28564) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn7 + (((0.5 * locals.var_pparam_b4soickappa_dn7) * assign30260_e28562) + (assign30260_e28559 * locals.var_t4__blk812_dn7)))))), ((((locals.var_pparam_b4soicgdo_dn8 + locals.var_t3__blk811_dn8) * locals.var_vgd) + (assign30260_e28552 * locals.var_vgd_dn8)) - ((locals.var_t3__blk811_dn8 * assign30260_e28564) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn8 + (((0.5 * locals.var_pparam_b4soickappa_dn8) * assign30260_e28562) + (assign30260_e28559 * locals.var_t4__blk812_dn8)))))), ((((locals.var_pparam_b4soicgdo_dn9 + locals.var_t3__blk811_dn9) * locals.var_vgd) + (assign30260_e28552 * locals.var_vgd_dn9)) - ((locals.var_t3__blk811_dn9 * assign30260_e28564) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn9 + (((0.5 * locals.var_pparam_b4soickappa_dn9) * assign30260_e28562) + (assign30260_e28559 * locals.var_t4__blk812_dn9)))))), (((locals.var_pparam_b4soicgdo_dn10 + locals.var_t3__blk811_dn10) * locals.var_vgd) - ((locals.var_t3__blk811_dn10 * assign30260_e28564) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn10 + (((0.5 * locals.var_pparam_b4soickappa_dn10) * assign30260_e28562) + (assign30260_e28559 * locals.var_t4__blk812_dn10)))))), (((locals.var_pparam_b4soicgdo_dn11 + locals.var_t3__blk811_dn11) * locals.var_vgd) - ((locals.var_t3__blk811_dn11 * assign30260_e28564) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn11 + (((0.5 * locals.var_pparam_b4soickappa_dn11) * assign30260_e28562) + (assign30260_e28559 * locals.var_t4__blk812_dn11)))))), (((locals.var_pparam_b4soicgdo_dn12 + locals.var_t3__blk811_dn12) * locals.var_vgd) - ((locals.var_t3__blk811_dn12 * assign30260_e28564) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn12 + (((0.5 * locals.var_pparam_b4soickappa_dn12) * assign30260_e28562) + (assign30260_e28559 * locals.var_t4__blk812_dn12)))))),)
    } else {
        (locals.var_qgdo, locals.var_qgdo_dn3, locals.var_qgdo_dn4, locals.var_qgdo_dn5, locals.var_qgdo_dn6, locals.var_qgdo_dn7, locals.var_qgdo_dn8, locals.var_qgdo_dn9, locals.var_qgdo_dn10, locals.var_qgdo_dn11, locals.var_qgdo_dn12,)
    }
};
        locals.var_qgdo = assign30260_e28568;
        locals.var_qgdo_dn3 = assign30260_e28568_d_n3;
        locals.var_qgdo_dn4 = assign30260_e28568_d_n4;
        locals.var_qgdo_dn5 = assign30260_e28568_d_n5;
        locals.var_qgdo_dn6 = assign30260_e28568_d_n6;
        locals.var_qgdo_dn7 = assign30260_e28568_d_n7;
        locals.var_qgdo_dn8 = assign30260_e28568_d_n8;
        locals.var_qgdo_dn9 = assign30260_e28568_d_n9;
        locals.var_qgdo_dn10 = assign30260_e28568_d_n10;
        locals.var_qgdo_dn11 = assign30260_e28568_d_n11;
        locals.var_qgdo_dn12 = assign30260_e28568_d_n12;
        locals.var_qgdo_rv = 0.0;

        let assign30270_e28571: f64 = if p.p39 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1441 = assign30270_e28571;
        locals.var_guard1441_rv = 0.0;

        let (assign30280_e28577, assign30280_e28577_d_n3, assign30280_e28577_d_n4, assign30280_e28577_d_n5, assign30280_e28577_d_n6, assign30280_e28577_d_n7, assign30280_e28577_d_n8, assign30280_e28577_d_n9, assign30280_e28577_d_n10, assign30280_e28577_d_n11, assign30280_e28577_d_n12,) = {
    if (locals.var_guard1441 != 0.0) {
        let assign30280_e28575: f64 = (locals.var_vgms + 0.02);
        (assign30280_e28575, 0.0, 0.0, 0.0, 0.0, 0.0, locals.var_vgms_dn8, 0.0, locals.var_vgms_dn10, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign30280_e28577;
        locals.var_t0__blk808_dn3 = assign30280_e28577_d_n3;
        locals.var_t0__blk808_dn4 = assign30280_e28577_d_n4;
        locals.var_t0__blk808_dn5 = assign30280_e28577_d_n5;
        locals.var_t0__blk808_dn6 = assign30280_e28577_d_n6;
        locals.var_t0__blk808_dn7 = assign30280_e28577_d_n7;
        locals.var_t0__blk808_dn8 = assign30280_e28577_d_n8;
        locals.var_t0__blk808_dn9 = assign30280_e28577_d_n9;
        locals.var_t0__blk808_dn10 = assign30280_e28577_d_n10;
        locals.var_t0__blk808_dn11 = assign30280_e28577_d_n11;
        locals.var_t0__blk808_dn12 = assign30280_e28577_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign30290_e28584, assign30290_e28584_d_n3, assign30290_e28584_d_n4, assign30290_e28584_d_n5, assign30290_e28584_d_n6, assign30290_e28584_d_n7, assign30290_e28584_d_n8, assign30290_e28584_d_n9, assign30290_e28584_d_n10, assign30290_e28584_d_n11, assign30290_e28584_d_n12,) = {
    if (locals.var_guard1441 == 0.0) {
        let assign30290_e28582: f64 = (locals.var_vgs + 0.02);
        (assign30290_e28582, 0.0, 0.0, 0.0, 0.0, 0.0, locals.var_vgs_dn8, locals.var_vgs_dn9, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign30290_e28584;
        locals.var_t0__blk808_dn3 = assign30290_e28584_d_n3;
        locals.var_t0__blk808_dn4 = assign30290_e28584_d_n4;
        locals.var_t0__blk808_dn5 = assign30290_e28584_d_n5;
        locals.var_t0__blk808_dn6 = assign30290_e28584_d_n6;
        locals.var_t0__blk808_dn7 = assign30290_e28584_d_n7;
        locals.var_t0__blk808_dn8 = assign30290_e28584_d_n8;
        locals.var_t0__blk808_dn9 = assign30290_e28584_d_n9;
        locals.var_t0__blk808_dn10 = assign30290_e28584_d_n10;
        locals.var_t0__blk808_dn11 = assign30290_e28584_d_n11;
        locals.var_t0__blk808_dn12 = assign30290_e28584_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let assign30300_e28587: f64 = (locals.var_t0__blk808 * locals.var_t0__blk808);
        let assign30300_e28590: f64 = (4.0 * 0.02);
        let assign30300_e28591: f64 = (assign30300_e28587 + assign30300_e28590);
        let assign30300_e28592: f64 = (assign30300_e28591).sqrt();
        locals.var_t1__blk809 = assign30300_e28592;
        locals.var_t1__blk809_dn3 = (((locals.var_t0__blk808_dn3 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn3)) / (2.0 * assign30300_e28592));
        locals.var_t1__blk809_dn4 = (((locals.var_t0__blk808_dn4 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn4)) / (2.0 * assign30300_e28592));
        locals.var_t1__blk809_dn5 = (((locals.var_t0__blk808_dn5 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn5)) / (2.0 * assign30300_e28592));
        locals.var_t1__blk809_dn6 = (((locals.var_t0__blk808_dn6 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn6)) / (2.0 * assign30300_e28592));
        locals.var_t1__blk809_dn7 = (((locals.var_t0__blk808_dn7 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn7)) / (2.0 * assign30300_e28592));
        locals.var_t1__blk809_dn8 = (((locals.var_t0__blk808_dn8 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn8)) / (2.0 * assign30300_e28592));
        locals.var_t1__blk809_dn9 = (((locals.var_t0__blk808_dn9 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn9)) / (2.0 * assign30300_e28592));
        locals.var_t1__blk809_dn10 = (((locals.var_t0__blk808_dn10 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn10)) / (2.0 * assign30300_e28592));
        locals.var_t1__blk809_dn11 = (((locals.var_t0__blk808_dn11 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn11)) / (2.0 * assign30300_e28592));
        locals.var_t1__blk809_dn12 = (((locals.var_t0__blk808_dn12 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn12)) / (2.0 * assign30300_e28592));
        locals.var_t1__blk809_rv = 0.0;

        let assign30310_e28596: f64 = (locals.var_t0__blk808 - locals.var_t1__blk809);
        let assign30310_e28597: f64 = (0.5 * assign30310_e28596);
        locals.var_t2__blk810 = assign30310_e28597;
        locals.var_t2__blk810_dn3 = (0.5 * (locals.var_t0__blk808_dn3 - locals.var_t1__blk809_dn3));
        locals.var_t2__blk810_dn4 = (0.5 * (locals.var_t0__blk808_dn4 - locals.var_t1__blk809_dn4));
        locals.var_t2__blk810_dn5 = (0.5 * (locals.var_t0__blk808_dn5 - locals.var_t1__blk809_dn5));
        locals.var_t2__blk810_dn6 = (0.5 * (locals.var_t0__blk808_dn6 - locals.var_t1__blk809_dn6));
        locals.var_t2__blk810_dn7 = (0.5 * (locals.var_t0__blk808_dn7 - locals.var_t1__blk809_dn7));
        locals.var_t2__blk810_dn8 = (0.5 * (locals.var_t0__blk808_dn8 - locals.var_t1__blk809_dn8));
        locals.var_t2__blk810_dn9 = (0.5 * (locals.var_t0__blk808_dn9 - locals.var_t1__blk809_dn9));
        locals.var_t2__blk810_dn10 = (0.5 * (locals.var_t0__blk808_dn10 - locals.var_t1__blk809_dn10));
        locals.var_t2__blk810_dn11 = (0.5 * (locals.var_t0__blk808_dn11 - locals.var_t1__blk809_dn11));
        locals.var_t2__blk810_dn12 = (0.5 * (locals.var_t0__blk808_dn12 - locals.var_t1__blk809_dn12));
        locals.var_t2__blk810_rv = 0.0;

        let assign30320_e28600: f64 = (locals.var_pparam_b4soiwdioscv * locals.var_pparam_b4soicgsl);
        locals.var_t3__blk811 = assign30320_e28600;
        locals.var_t3__blk811_dn3 = ((locals.var_pparam_b4soiwdioscv_dn3 * locals.var_pparam_b4soicgsl) + (locals.var_pparam_b4soiwdioscv * locals.var_pparam_b4soicgsl_dn3));
        locals.var_t3__blk811_dn4 = ((locals.var_pparam_b4soiwdioscv_dn4 * locals.var_pparam_b4soicgsl) + (locals.var_pparam_b4soiwdioscv * locals.var_pparam_b4soicgsl_dn4));
        locals.var_t3__blk811_dn5 = ((locals.var_pparam_b4soiwdioscv_dn5 * locals.var_pparam_b4soicgsl) + (locals.var_pparam_b4soiwdioscv * locals.var_pparam_b4soicgsl_dn5));
        locals.var_t3__blk811_dn6 = ((locals.var_pparam_b4soiwdioscv_dn6 * locals.var_pparam_b4soicgsl) + (locals.var_pparam_b4soiwdioscv * locals.var_pparam_b4soicgsl_dn6));
        locals.var_t3__blk811_dn7 = ((locals.var_pparam_b4soiwdioscv_dn7 * locals.var_pparam_b4soicgsl) + (locals.var_pparam_b4soiwdioscv * locals.var_pparam_b4soicgsl_dn7));
        locals.var_t3__blk811_dn8 = ((locals.var_pparam_b4soiwdioscv_dn8 * locals.var_pparam_b4soicgsl) + (locals.var_pparam_b4soiwdioscv * locals.var_pparam_b4soicgsl_dn8));
        locals.var_t3__blk811_dn9 = ((locals.var_pparam_b4soiwdioscv_dn9 * locals.var_pparam_b4soicgsl) + (locals.var_pparam_b4soiwdioscv * locals.var_pparam_b4soicgsl_dn9));
        locals.var_t3__blk811_dn10 = ((locals.var_pparam_b4soiwdioscv_dn10 * locals.var_pparam_b4soicgsl) + (locals.var_pparam_b4soiwdioscv * locals.var_pparam_b4soicgsl_dn10));
        locals.var_t3__blk811_dn11 = ((locals.var_pparam_b4soiwdioscv_dn11 * locals.var_pparam_b4soicgsl) + (locals.var_pparam_b4soiwdioscv * locals.var_pparam_b4soicgsl_dn11));
        locals.var_t3__blk811_dn12 = ((locals.var_pparam_b4soiwdioscv_dn12 * locals.var_pparam_b4soicgsl) + (locals.var_pparam_b4soiwdioscv * locals.var_pparam_b4soicgsl_dn12));
        locals.var_t3__blk811_rv = 0.0;

        let assign30330_e28604: f64 = (4.0 * locals.var_t2__blk810);
        let assign30330_e28606: f64 = (assign30330_e28604 / locals.var_pparam_b4soickappa);
        let assign30330_e28607: f64 = (1.0 - assign30330_e28606);
        let assign30330_e28608: f64 = (assign30330_e28607).sqrt();
        locals.var_t4__blk812 = assign30330_e28608;
        locals.var_t4__blk812_dn3 = ((-((((4.0 * locals.var_t2__blk810_dn3) * locals.var_pparam_b4soickappa) - (assign30330_e28604 * locals.var_pparam_b4soickappa_dn3)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign30330_e28608));
        locals.var_t4__blk812_dn4 = ((-((((4.0 * locals.var_t2__blk810_dn4) * locals.var_pparam_b4soickappa) - (assign30330_e28604 * locals.var_pparam_b4soickappa_dn4)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign30330_e28608));
        locals.var_t4__blk812_dn5 = ((-((((4.0 * locals.var_t2__blk810_dn5) * locals.var_pparam_b4soickappa) - (assign30330_e28604 * locals.var_pparam_b4soickappa_dn5)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign30330_e28608));
        locals.var_t4__blk812_dn6 = ((-((((4.0 * locals.var_t2__blk810_dn6) * locals.var_pparam_b4soickappa) - (assign30330_e28604 * locals.var_pparam_b4soickappa_dn6)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign30330_e28608));
        locals.var_t4__blk812_dn7 = ((-((((4.0 * locals.var_t2__blk810_dn7) * locals.var_pparam_b4soickappa) - (assign30330_e28604 * locals.var_pparam_b4soickappa_dn7)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign30330_e28608));
        locals.var_t4__blk812_dn8 = ((-((((4.0 * locals.var_t2__blk810_dn8) * locals.var_pparam_b4soickappa) - (assign30330_e28604 * locals.var_pparam_b4soickappa_dn8)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign30330_e28608));
        locals.var_t4__blk812_dn9 = ((-((((4.0 * locals.var_t2__blk810_dn9) * locals.var_pparam_b4soickappa) - (assign30330_e28604 * locals.var_pparam_b4soickappa_dn9)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign30330_e28608));
        locals.var_t4__blk812_dn10 = ((-((((4.0 * locals.var_t2__blk810_dn10) * locals.var_pparam_b4soickappa) - (assign30330_e28604 * locals.var_pparam_b4soickappa_dn10)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign30330_e28608));
        locals.var_t4__blk812_dn11 = ((-((((4.0 * locals.var_t2__blk810_dn11) * locals.var_pparam_b4soickappa) - (assign30330_e28604 * locals.var_pparam_b4soickappa_dn11)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign30330_e28608));
        locals.var_t4__blk812_dn12 = ((-((((4.0 * locals.var_t2__blk810_dn12) * locals.var_pparam_b4soickappa) - (assign30330_e28604 * locals.var_pparam_b4soickappa_dn12)) / (locals.var_pparam_b4soickappa * locals.var_pparam_b4soickappa))) / (2.0 * assign30330_e28608));
        locals.var_t4__blk812_rv = 0.0;

        let assign30340_e28611: f64 = if p.p39 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1442 = assign30340_e28611;
        locals.var_guard1442_rv = 0.0;

        let (assign30350_e28631, assign30350_e28631_d_n3, assign30350_e28631_d_n4, assign30350_e28631_d_n5, assign30350_e28631_d_n6, assign30350_e28631_d_n7, assign30350_e28631_d_n8, assign30350_e28631_d_n9, assign30350_e28631_d_n10, assign30350_e28631_d_n11, assign30350_e28631_d_n12,) = {
    if (locals.var_guard1442 != 0.0) {
        let assign30350_e28615: f64 = (locals.var_pparam_b4soicgso + locals.var_t3__blk811);
        let assign30350_e28617: f64 = (assign30350_e28615 * locals.var_vgms);
        let assign30350_e28622: f64 = (0.5 * locals.var_pparam_b4soickappa);
        let assign30350_e28625: f64 = (locals.var_t4__blk812 - 1.0);
        let assign30350_e28626: f64 = (assign30350_e28622 * assign30350_e28625);
        let assign30350_e28627: f64 = (locals.var_t2__blk810 + assign30350_e28626);
        let assign30350_e28628: f64 = (locals.var_t3__blk811 * assign30350_e28627);
        let assign30350_e28629: f64 = (assign30350_e28617 - assign30350_e28628);
        (assign30350_e28629, (((locals.var_pparam_b4soicgso_dn3 + locals.var_t3__blk811_dn3) * locals.var_vgms) - ((locals.var_t3__blk811_dn3 * assign30350_e28627) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn3 + (((0.5 * locals.var_pparam_b4soickappa_dn3) * assign30350_e28625) + (assign30350_e28622 * locals.var_t4__blk812_dn3)))))), (((locals.var_pparam_b4soicgso_dn4 + locals.var_t3__blk811_dn4) * locals.var_vgms) - ((locals.var_t3__blk811_dn4 * assign30350_e28627) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn4 + (((0.5 * locals.var_pparam_b4soickappa_dn4) * assign30350_e28625) + (assign30350_e28622 * locals.var_t4__blk812_dn4)))))), (((locals.var_pparam_b4soicgso_dn5 + locals.var_t3__blk811_dn5) * locals.var_vgms) - ((locals.var_t3__blk811_dn5 * assign30350_e28627) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn5 + (((0.5 * locals.var_pparam_b4soickappa_dn5) * assign30350_e28625) + (assign30350_e28622 * locals.var_t4__blk812_dn5)))))), (((locals.var_pparam_b4soicgso_dn6 + locals.var_t3__blk811_dn6) * locals.var_vgms) - ((locals.var_t3__blk811_dn6 * assign30350_e28627) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn6 + (((0.5 * locals.var_pparam_b4soickappa_dn6) * assign30350_e28625) + (assign30350_e28622 * locals.var_t4__blk812_dn6)))))), (((locals.var_pparam_b4soicgso_dn7 + locals.var_t3__blk811_dn7) * locals.var_vgms) - ((locals.var_t3__blk811_dn7 * assign30350_e28627) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn7 + (((0.5 * locals.var_pparam_b4soickappa_dn7) * assign30350_e28625) + (assign30350_e28622 * locals.var_t4__blk812_dn7)))))), ((((locals.var_pparam_b4soicgso_dn8 + locals.var_t3__blk811_dn8) * locals.var_vgms) + (assign30350_e28615 * locals.var_vgms_dn8)) - ((locals.var_t3__blk811_dn8 * assign30350_e28627) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn8 + (((0.5 * locals.var_pparam_b4soickappa_dn8) * assign30350_e28625) + (assign30350_e28622 * locals.var_t4__blk812_dn8)))))), (((locals.var_pparam_b4soicgso_dn9 + locals.var_t3__blk811_dn9) * locals.var_vgms) - ((locals.var_t3__blk811_dn9 * assign30350_e28627) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn9 + (((0.5 * locals.var_pparam_b4soickappa_dn9) * assign30350_e28625) + (assign30350_e28622 * locals.var_t4__blk812_dn9)))))), ((((locals.var_pparam_b4soicgso_dn10 + locals.var_t3__blk811_dn10) * locals.var_vgms) + (assign30350_e28615 * locals.var_vgms_dn10)) - ((locals.var_t3__blk811_dn10 * assign30350_e28627) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn10 + (((0.5 * locals.var_pparam_b4soickappa_dn10) * assign30350_e28625) + (assign30350_e28622 * locals.var_t4__blk812_dn10)))))), (((locals.var_pparam_b4soicgso_dn11 + locals.var_t3__blk811_dn11) * locals.var_vgms) - ((locals.var_t3__blk811_dn11 * assign30350_e28627) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn11 + (((0.5 * locals.var_pparam_b4soickappa_dn11) * assign30350_e28625) + (assign30350_e28622 * locals.var_t4__blk812_dn11)))))), (((locals.var_pparam_b4soicgso_dn12 + locals.var_t3__blk811_dn12) * locals.var_vgms) - ((locals.var_t3__blk811_dn12 * assign30350_e28627) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn12 + (((0.5 * locals.var_pparam_b4soickappa_dn12) * assign30350_e28625) + (assign30350_e28622 * locals.var_t4__blk812_dn12)))))),)
    } else {
        (locals.var_qgso, locals.var_qgso_dn3, locals.var_qgso_dn4, locals.var_qgso_dn5, locals.var_qgso_dn6, locals.var_qgso_dn7, locals.var_qgso_dn8, locals.var_qgso_dn9, locals.var_qgso_dn10, locals.var_qgso_dn11, locals.var_qgso_dn12,)
    }
};
        locals.var_qgso = assign30350_e28631;
        locals.var_qgso_dn3 = assign30350_e28631_d_n3;
        locals.var_qgso_dn4 = assign30350_e28631_d_n4;
        locals.var_qgso_dn5 = assign30350_e28631_d_n5;
        locals.var_qgso_dn6 = assign30350_e28631_d_n6;
        locals.var_qgso_dn7 = assign30350_e28631_d_n7;
        locals.var_qgso_dn8 = assign30350_e28631_d_n8;
        locals.var_qgso_dn9 = assign30350_e28631_d_n9;
        locals.var_qgso_dn10 = assign30350_e28631_d_n10;
        locals.var_qgso_dn11 = assign30350_e28631_d_n11;
        locals.var_qgso_dn12 = assign30350_e28631_d_n12;
        locals.var_qgso_rv = 0.0;

        let (assign30360_e28652, assign30360_e28652_d_n3, assign30360_e28652_d_n4, assign30360_e28652_d_n5, assign30360_e28652_d_n6, assign30360_e28652_d_n7, assign30360_e28652_d_n8, assign30360_e28652_d_n9, assign30360_e28652_d_n10, assign30360_e28652_d_n11, assign30360_e28652_d_n12,) = {
    if (locals.var_guard1442 == 0.0) {
        let assign30360_e28636: f64 = (locals.var_pparam_b4soicgso + locals.var_t3__blk811);
        let assign30360_e28638: f64 = (assign30360_e28636 * locals.var_vgs);
        let assign30360_e28643: f64 = (0.5 * locals.var_pparam_b4soickappa);
        let assign30360_e28646: f64 = (locals.var_t4__blk812 - 1.0);
        let assign30360_e28647: f64 = (assign30360_e28643 * assign30360_e28646);
        let assign30360_e28648: f64 = (locals.var_t2__blk810 + assign30360_e28647);
        let assign30360_e28649: f64 = (locals.var_t3__blk811 * assign30360_e28648);
        let assign30360_e28650: f64 = (assign30360_e28638 - assign30360_e28649);
        (assign30360_e28650, (((locals.var_pparam_b4soicgso_dn3 + locals.var_t3__blk811_dn3) * locals.var_vgs) - ((locals.var_t3__blk811_dn3 * assign30360_e28648) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn3 + (((0.5 * locals.var_pparam_b4soickappa_dn3) * assign30360_e28646) + (assign30360_e28643 * locals.var_t4__blk812_dn3)))))), (((locals.var_pparam_b4soicgso_dn4 + locals.var_t3__blk811_dn4) * locals.var_vgs) - ((locals.var_t3__blk811_dn4 * assign30360_e28648) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn4 + (((0.5 * locals.var_pparam_b4soickappa_dn4) * assign30360_e28646) + (assign30360_e28643 * locals.var_t4__blk812_dn4)))))), (((locals.var_pparam_b4soicgso_dn5 + locals.var_t3__blk811_dn5) * locals.var_vgs) - ((locals.var_t3__blk811_dn5 * assign30360_e28648) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn5 + (((0.5 * locals.var_pparam_b4soickappa_dn5) * assign30360_e28646) + (assign30360_e28643 * locals.var_t4__blk812_dn5)))))), (((locals.var_pparam_b4soicgso_dn6 + locals.var_t3__blk811_dn6) * locals.var_vgs) - ((locals.var_t3__blk811_dn6 * assign30360_e28648) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn6 + (((0.5 * locals.var_pparam_b4soickappa_dn6) * assign30360_e28646) + (assign30360_e28643 * locals.var_t4__blk812_dn6)))))), (((locals.var_pparam_b4soicgso_dn7 + locals.var_t3__blk811_dn7) * locals.var_vgs) - ((locals.var_t3__blk811_dn7 * assign30360_e28648) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn7 + (((0.5 * locals.var_pparam_b4soickappa_dn7) * assign30360_e28646) + (assign30360_e28643 * locals.var_t4__blk812_dn7)))))), ((((locals.var_pparam_b4soicgso_dn8 + locals.var_t3__blk811_dn8) * locals.var_vgs) + (assign30360_e28636 * locals.var_vgs_dn8)) - ((locals.var_t3__blk811_dn8 * assign30360_e28648) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn8 + (((0.5 * locals.var_pparam_b4soickappa_dn8) * assign30360_e28646) + (assign30360_e28643 * locals.var_t4__blk812_dn8)))))), ((((locals.var_pparam_b4soicgso_dn9 + locals.var_t3__blk811_dn9) * locals.var_vgs) + (assign30360_e28636 * locals.var_vgs_dn9)) - ((locals.var_t3__blk811_dn9 * assign30360_e28648) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn9 + (((0.5 * locals.var_pparam_b4soickappa_dn9) * assign30360_e28646) + (assign30360_e28643 * locals.var_t4__blk812_dn9)))))), (((locals.var_pparam_b4soicgso_dn10 + locals.var_t3__blk811_dn10) * locals.var_vgs) - ((locals.var_t3__blk811_dn10 * assign30360_e28648) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn10 + (((0.5 * locals.var_pparam_b4soickappa_dn10) * assign30360_e28646) + (assign30360_e28643 * locals.var_t4__blk812_dn10)))))), (((locals.var_pparam_b4soicgso_dn11 + locals.var_t3__blk811_dn11) * locals.var_vgs) - ((locals.var_t3__blk811_dn11 * assign30360_e28648) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn11 + (((0.5 * locals.var_pparam_b4soickappa_dn11) * assign30360_e28646) + (assign30360_e28643 * locals.var_t4__blk812_dn11)))))), (((locals.var_pparam_b4soicgso_dn12 + locals.var_t3__blk811_dn12) * locals.var_vgs) - ((locals.var_t3__blk811_dn12 * assign30360_e28648) + (locals.var_t3__blk811 * (locals.var_t2__blk810_dn12 + (((0.5 * locals.var_pparam_b4soickappa_dn12) * assign30360_e28646) + (assign30360_e28643 * locals.var_t4__blk812_dn12)))))),)
    } else {
        (locals.var_qgso, locals.var_qgso_dn3, locals.var_qgso_dn4, locals.var_qgso_dn5, locals.var_qgso_dn6, locals.var_qgso_dn7, locals.var_qgso_dn8, locals.var_qgso_dn9, locals.var_qgso_dn10, locals.var_qgso_dn11, locals.var_qgso_dn12,)
    }
};
        locals.var_qgso = assign30360_e28652;
        locals.var_qgso_dn3 = assign30360_e28652_d_n3;
        locals.var_qgso_dn4 = assign30360_e28652_d_n4;
        locals.var_qgso_dn5 = assign30360_e28652_d_n5;
        locals.var_qgso_dn6 = assign30360_e28652_d_n6;
        locals.var_qgso_dn7 = assign30360_e28652_d_n7;
        locals.var_qgso_dn8 = assign30360_e28652_d_n8;
        locals.var_qgso_dn9 = assign30360_e28652_d_n9;
        locals.var_qgso_dn10 = assign30360_e28652_d_n10;
        locals.var_qgso_dn11 = assign30360_e28652_d_n11;
        locals.var_qgso_dn12 = assign30360_e28652_d_n12;
        locals.var_qgso_rv = 0.0;

        let assign30370_e28655: f64 = if p.p3 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1443 = assign30370_e28655;
        locals.var_guard1443_rv = 0.0;

        let (assign30380_e28661, assign30380_e28661_d_n3, assign30380_e28661_d_n4, assign30380_e28661_d_n5, assign30380_e28661_d_n6, assign30380_e28661_d_n7, assign30380_e28661_d_n8, assign30380_e28661_d_n9, assign30380_e28661_d_n10, assign30380_e28661_d_n11, assign30380_e28661_d_n12,) = {
    if (locals.var_guard1443 != 0.0) {
        let assign30380_e28659: f64 = (locals.var_qgdo * p.p3);
        (assign30380_e28659, (locals.var_qgdo_dn3 * p.p3), (locals.var_qgdo_dn4 * p.p3), (locals.var_qgdo_dn5 * p.p3), (locals.var_qgdo_dn6 * p.p3), (locals.var_qgdo_dn7 * p.p3), (locals.var_qgdo_dn8 * p.p3), (locals.var_qgdo_dn9 * p.p3), (locals.var_qgdo_dn10 * p.p3), (locals.var_qgdo_dn11 * p.p3), (locals.var_qgdo_dn12 * p.p3),)
    } else {
        (locals.var_qgdo, locals.var_qgdo_dn3, locals.var_qgdo_dn4, locals.var_qgdo_dn5, locals.var_qgdo_dn6, locals.var_qgdo_dn7, locals.var_qgdo_dn8, locals.var_qgdo_dn9, locals.var_qgdo_dn10, locals.var_qgdo_dn11, locals.var_qgdo_dn12,)
    }
};
        locals.var_qgdo = assign30380_e28661;
        locals.var_qgdo_dn3 = assign30380_e28661_d_n3;
        locals.var_qgdo_dn4 = assign30380_e28661_d_n4;
        locals.var_qgdo_dn5 = assign30380_e28661_d_n5;
        locals.var_qgdo_dn6 = assign30380_e28661_d_n6;
        locals.var_qgdo_dn7 = assign30380_e28661_d_n7;
        locals.var_qgdo_dn8 = assign30380_e28661_d_n8;
        locals.var_qgdo_dn9 = assign30380_e28661_d_n9;
        locals.var_qgdo_dn10 = assign30380_e28661_d_n10;
        locals.var_qgdo_dn11 = assign30380_e28661_d_n11;
        locals.var_qgdo_dn12 = assign30380_e28661_d_n12;
        locals.var_qgdo_rv = 0.0;

        let (assign30390_e28667, assign30390_e28667_d_n3, assign30390_e28667_d_n4, assign30390_e28667_d_n5, assign30390_e28667_d_n6, assign30390_e28667_d_n7, assign30390_e28667_d_n8, assign30390_e28667_d_n9, assign30390_e28667_d_n10, assign30390_e28667_d_n11, assign30390_e28667_d_n12,) = {
    if (locals.var_guard1443 != 0.0) {
        let assign30390_e28665: f64 = (locals.var_qgso * p.p3);
        (assign30390_e28665, (locals.var_qgso_dn3 * p.p3), (locals.var_qgso_dn4 * p.p3), (locals.var_qgso_dn5 * p.p3), (locals.var_qgso_dn6 * p.p3), (locals.var_qgso_dn7 * p.p3), (locals.var_qgso_dn8 * p.p3), (locals.var_qgso_dn9 * p.p3), (locals.var_qgso_dn10 * p.p3), (locals.var_qgso_dn11 * p.p3), (locals.var_qgso_dn12 * p.p3),)
    } else {
        (locals.var_qgso, locals.var_qgso_dn3, locals.var_qgso_dn4, locals.var_qgso_dn5, locals.var_qgso_dn6, locals.var_qgso_dn7, locals.var_qgso_dn8, locals.var_qgso_dn9, locals.var_qgso_dn10, locals.var_qgso_dn11, locals.var_qgso_dn12,)
    }
};
        locals.var_qgso = assign30390_e28667;
        locals.var_qgso_dn3 = assign30390_e28667_d_n3;
        locals.var_qgso_dn4 = assign30390_e28667_d_n4;
        locals.var_qgso_dn5 = assign30390_e28667_d_n5;
        locals.var_qgso_dn6 = assign30390_e28667_d_n6;
        locals.var_qgso_dn7 = assign30390_e28667_d_n7;
        locals.var_qgso_dn8 = assign30390_e28667_d_n8;
        locals.var_qgso_dn9 = assign30390_e28667_d_n9;
        locals.var_qgso_dn10 = assign30390_e28667_d_n10;
        locals.var_qgso_dn11 = assign30390_e28667_d_n11;
        locals.var_qgso_dn12 = assign30390_e28667_d_n12;
        locals.var_qgso_rv = 0.0;

        let assign30500_e28725: f64 = if p.p223 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1470 = assign30500_e28725;
        locals.var_guard1470_rv = 0.0;

        let assign30510_e28728: f64 = if p.p223 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1471 = assign30510_e28728;
        locals.var_guard1471_rv = 0.0;

        let assign30520_e28731: f64 = if p.p223 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1472 = assign30520_e28731;
        locals.var_guard1472_rv = 0.0;

        let assign30530_e28734: f64 = if p.p223 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1473 = assign30530_e28734;
        locals.var_guard1473_rv = 0.0;

        let (assign30550_e28765, assign30550_e28765_d_n3, assign30550_e28765_d_n4, assign30550_e28765_d_n5, assign30550_e28765_d_n6, assign30550_e28765_d_n7, assign30550_e28765_d_n8, assign30550_e28765_d_n9, assign30550_e28765_d_n10, assign30550_e28765_d_n11, assign30550_e28765_d_n12,) = {
    if ((locals.var_guard1471 != 0.0) && (locals.var_guard1470 == 0.0)) {
        let assign30550_e28761: f64 = (locals.var_b4soigm + locals.var_b4soigds);
        let assign30550_e28763: f64 = (assign30550_e28761 + locals.var_b4soigmbs);
        (assign30550_e28763, ((locals.var_b4soigm_dn3 + locals.var_b4soigds_dn3) + locals.var_b4soigmbs_dn3), ((locals.var_b4soigm_dn4 + locals.var_b4soigds_dn4) + locals.var_b4soigmbs_dn4), ((locals.var_b4soigm_dn5 + locals.var_b4soigds_dn5) + locals.var_b4soigmbs_dn5), ((locals.var_b4soigm_dn6 + locals.var_b4soigds_dn6) + locals.var_b4soigmbs_dn6), ((locals.var_b4soigm_dn7 + locals.var_b4soigds_dn7) + locals.var_b4soigmbs_dn7), ((locals.var_b4soigm_dn8 + locals.var_b4soigds_dn8) + locals.var_b4soigmbs_dn8), ((locals.var_b4soigm_dn9 + locals.var_b4soigds_dn9) + locals.var_b4soigmbs_dn9), ((locals.var_b4soigm_dn10 + locals.var_b4soigds_dn10) + locals.var_b4soigmbs_dn10), ((locals.var_b4soigm_dn11 + locals.var_b4soigds_dn11) + locals.var_b4soigmbs_dn11), ((locals.var_b4soigm_dn12 + locals.var_b4soigds_dn12) + locals.var_b4soigmbs_dn12),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign30550_e28765;
        locals.var_t0__blk808_dn3 = assign30550_e28765_d_n3;
        locals.var_t0__blk808_dn4 = assign30550_e28765_d_n4;
        locals.var_t0__blk808_dn5 = assign30550_e28765_d_n5;
        locals.var_t0__blk808_dn6 = assign30550_e28765_d_n6;
        locals.var_t0__blk808_dn7 = assign30550_e28765_d_n7;
        locals.var_t0__blk808_dn8 = assign30550_e28765_d_n8;
        locals.var_t0__blk808_dn9 = assign30550_e28765_d_n9;
        locals.var_t0__blk808_dn10 = assign30550_e28765_d_n10;
        locals.var_t0__blk808_dn11 = assign30550_e28765_d_n11;
        locals.var_t0__blk808_dn12 = assign30550_e28765_d_n12;
        locals.var_t0__blk808_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_94(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign30560_e28774, assign30560_e28774_d_n3, assign30560_e28774_d_n4, assign30560_e28774_d_n5, assign30560_e28774_d_n6, assign30560_e28774_d_n7, assign30560_e28774_d_n8, assign30560_e28774_d_n9, assign30560_e28774_d_n10, assign30560_e28774_d_n11, assign30560_e28774_d_n12,) = {
    if ((locals.var_guard1471 != 0.0) && (locals.var_guard1470 == 0.0)) {
        let assign30560_e28772: f64 = (locals.var_t0__blk808 * locals.var_t0__blk808);
        (assign30560_e28772, ((locals.var_t0__blk808_dn3 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn3)), ((locals.var_t0__blk808_dn4 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn4)), ((locals.var_t0__blk808_dn5 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn5)), ((locals.var_t0__blk808_dn6 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn6)), ((locals.var_t0__blk808_dn7 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn7)), ((locals.var_t0__blk808_dn8 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn8)), ((locals.var_t0__blk808_dn9 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn9)), ((locals.var_t0__blk808_dn10 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn10)), ((locals.var_t0__blk808_dn11 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn11)), ((locals.var_t0__blk808_dn12 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn12)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign30560_e28774;
        locals.var_t0__blk808_dn3 = assign30560_e28774_d_n3;
        locals.var_t0__blk808_dn4 = assign30560_e28774_d_n4;
        locals.var_t0__blk808_dn5 = assign30560_e28774_d_n5;
        locals.var_t0__blk808_dn6 = assign30560_e28774_d_n6;
        locals.var_t0__blk808_dn7 = assign30560_e28774_d_n7;
        locals.var_t0__blk808_dn8 = assign30560_e28774_d_n8;
        locals.var_t0__blk808_dn9 = assign30560_e28774_d_n9;
        locals.var_t0__blk808_dn10 = assign30560_e28774_d_n10;
        locals.var_t0__blk808_dn11 = assign30560_e28774_d_n11;
        locals.var_t0__blk808_dn12 = assign30560_e28774_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign30570_e28785, assign30570_e28785_d_n3, assign30570_e28785_d_n4, assign30570_e28785_d_n5, assign30570_e28785_d_n6, assign30570_e28785_d_n7, assign30570_e28785_d_n8, assign30570_e28785_d_n9, assign30570_e28785_d_n10, assign30570_e28785_d_n11, assign30570_e28785_d_n12,) = {
    if ((locals.var_guard1471 != 0.0) && (locals.var_guard1470 == 0.0)) {
        let assign30570_e28781: f64 = (2.0 * locals.var_vsattemp);
        let assign30570_e28783: f64 = (assign30570_e28781 / locals.var_b4soiueff);
        (assign30570_e28783, ((((2.0 * locals.var_vsattemp_dn3) * locals.var_b4soiueff) - (assign30570_e28781 * locals.var_b4soiueff_dn3)) / (locals.var_b4soiueff * locals.var_b4soiueff)), ((((2.0 * locals.var_vsattemp_dn4) * locals.var_b4soiueff) - (assign30570_e28781 * locals.var_b4soiueff_dn4)) / (locals.var_b4soiueff * locals.var_b4soiueff)), ((((2.0 * locals.var_vsattemp_dn5) * locals.var_b4soiueff) - (assign30570_e28781 * locals.var_b4soiueff_dn5)) / (locals.var_b4soiueff * locals.var_b4soiueff)), ((((2.0 * locals.var_vsattemp_dn6) * locals.var_b4soiueff) - (assign30570_e28781 * locals.var_b4soiueff_dn6)) / (locals.var_b4soiueff * locals.var_b4soiueff)), ((((2.0 * locals.var_vsattemp_dn7) * locals.var_b4soiueff) - (assign30570_e28781 * locals.var_b4soiueff_dn7)) / (locals.var_b4soiueff * locals.var_b4soiueff)), ((((2.0 * locals.var_vsattemp_dn8) * locals.var_b4soiueff) - (assign30570_e28781 * locals.var_b4soiueff_dn8)) / (locals.var_b4soiueff * locals.var_b4soiueff)), ((((2.0 * locals.var_vsattemp_dn9) * locals.var_b4soiueff) - (assign30570_e28781 * locals.var_b4soiueff_dn9)) / (locals.var_b4soiueff * locals.var_b4soiueff)), ((((2.0 * locals.var_vsattemp_dn10) * locals.var_b4soiueff) - (assign30570_e28781 * locals.var_b4soiueff_dn10)) / (locals.var_b4soiueff * locals.var_b4soiueff)), ((((2.0 * locals.var_vsattemp_dn11) * locals.var_b4soiueff) - (assign30570_e28781 * locals.var_b4soiueff_dn11)) / (locals.var_b4soiueff * locals.var_b4soiueff)), ((((2.0 * locals.var_vsattemp_dn12) * locals.var_b4soiueff) - (assign30570_e28781 * locals.var_b4soiueff_dn12)) / (locals.var_b4soiueff * locals.var_b4soiueff)),)
    } else {
        (locals.var_esat_1, locals.var_esat_1_dn3, locals.var_esat_1_dn4, locals.var_esat_1_dn5, locals.var_esat_1_dn6, locals.var_esat_1_dn7, locals.var_esat_1_dn8, locals.var_esat_1_dn9, locals.var_esat_1_dn10, locals.var_esat_1_dn11, locals.var_esat_1_dn12,)
    }
};
        locals.var_esat_1 = assign30570_e28785;
        locals.var_esat_1_dn3 = assign30570_e28785_d_n3;
        locals.var_esat_1_dn4 = assign30570_e28785_d_n4;
        locals.var_esat_1_dn5 = assign30570_e28785_d_n5;
        locals.var_esat_1_dn6 = assign30570_e28785_d_n6;
        locals.var_esat_1_dn7 = assign30570_e28785_d_n7;
        locals.var_esat_1_dn8 = assign30570_e28785_d_n8;
        locals.var_esat_1_dn9 = assign30570_e28785_d_n9;
        locals.var_esat_1_dn10 = assign30570_e28785_d_n10;
        locals.var_esat_1_dn11 = assign30570_e28785_d_n11;
        locals.var_esat_1_dn12 = assign30570_e28785_d_n12;
        locals.var_esat_1_rv = 0.0;

        let (assign30580_e28796, assign30580_e28796_d_n3, assign30580_e28796_d_n4, assign30580_e28796_d_n5, assign30580_e28796_d_n6, assign30580_e28796_d_n7, assign30580_e28796_d_n8, assign30580_e28796_d_n9, assign30580_e28796_d_n10, assign30580_e28796_d_n11, assign30580_e28796_d_n12,) = {
    if ((locals.var_guard1471 != 0.0) && (locals.var_guard1470 == 0.0)) {
        let assign30580_e28793: f64 = (locals.var_esat_1 * locals.var_pparam_b4soileff);
        let assign30580_e28794: f64 = (locals.var_b4soivgsteff / assign30580_e28793);
        (assign30580_e28794, (((locals.var_b4soivgsteff_dn3 * assign30580_e28793) - (locals.var_b4soivgsteff * ((locals.var_esat_1_dn3 * locals.var_pparam_b4soileff) + (locals.var_esat_1 * locals.var_pparam_b4soileff_dn3)))) / (assign30580_e28793 * assign30580_e28793)), (((locals.var_b4soivgsteff_dn4 * assign30580_e28793) - (locals.var_b4soivgsteff * ((locals.var_esat_1_dn4 * locals.var_pparam_b4soileff) + (locals.var_esat_1 * locals.var_pparam_b4soileff_dn4)))) / (assign30580_e28793 * assign30580_e28793)), (((locals.var_b4soivgsteff_dn5 * assign30580_e28793) - (locals.var_b4soivgsteff * ((locals.var_esat_1_dn5 * locals.var_pparam_b4soileff) + (locals.var_esat_1 * locals.var_pparam_b4soileff_dn5)))) / (assign30580_e28793 * assign30580_e28793)), (((locals.var_b4soivgsteff_dn6 * assign30580_e28793) - (locals.var_b4soivgsteff * ((locals.var_esat_1_dn6 * locals.var_pparam_b4soileff) + (locals.var_esat_1 * locals.var_pparam_b4soileff_dn6)))) / (assign30580_e28793 * assign30580_e28793)), (((locals.var_b4soivgsteff_dn7 * assign30580_e28793) - (locals.var_b4soivgsteff * ((locals.var_esat_1_dn7 * locals.var_pparam_b4soileff) + (locals.var_esat_1 * locals.var_pparam_b4soileff_dn7)))) / (assign30580_e28793 * assign30580_e28793)), (((locals.var_b4soivgsteff_dn8 * assign30580_e28793) - (locals.var_b4soivgsteff * ((locals.var_esat_1_dn8 * locals.var_pparam_b4soileff) + (locals.var_esat_1 * locals.var_pparam_b4soileff_dn8)))) / (assign30580_e28793 * assign30580_e28793)), (((locals.var_b4soivgsteff_dn9 * assign30580_e28793) - (locals.var_b4soivgsteff * ((locals.var_esat_1_dn9 * locals.var_pparam_b4soileff) + (locals.var_esat_1 * locals.var_pparam_b4soileff_dn9)))) / (assign30580_e28793 * assign30580_e28793)), (((locals.var_b4soivgsteff_dn10 * assign30580_e28793) - (locals.var_b4soivgsteff * ((locals.var_esat_1_dn10 * locals.var_pparam_b4soileff) + (locals.var_esat_1 * locals.var_pparam_b4soileff_dn10)))) / (assign30580_e28793 * assign30580_e28793)), (((locals.var_b4soivgsteff_dn11 * assign30580_e28793) - (locals.var_b4soivgsteff * ((locals.var_esat_1_dn11 * locals.var_pparam_b4soileff) + (locals.var_esat_1 * locals.var_pparam_b4soileff_dn11)))) / (assign30580_e28793 * assign30580_e28793)), (((locals.var_b4soivgsteff_dn12 * assign30580_e28793) - (locals.var_b4soivgsteff * ((locals.var_esat_1_dn12 * locals.var_pparam_b4soileff) + (locals.var_esat_1 * locals.var_pparam_b4soileff_dn12)))) / (assign30580_e28793 * assign30580_e28793)),)
    } else {
        (locals.var_t5__blk813, locals.var_t5__blk813_dn3, locals.var_t5__blk813_dn4, locals.var_t5__blk813_dn5, locals.var_t5__blk813_dn6, locals.var_t5__blk813_dn7, locals.var_t5__blk813_dn8, locals.var_t5__blk813_dn9, locals.var_t5__blk813_dn10, locals.var_t5__blk813_dn11, locals.var_t5__blk813_dn12,)
    }
};
        locals.var_t5__blk813 = assign30580_e28796;
        locals.var_t5__blk813_dn3 = assign30580_e28796_d_n3;
        locals.var_t5__blk813_dn4 = assign30580_e28796_d_n4;
        locals.var_t5__blk813_dn5 = assign30580_e28796_d_n5;
        locals.var_t5__blk813_dn6 = assign30580_e28796_d_n6;
        locals.var_t5__blk813_dn7 = assign30580_e28796_d_n7;
        locals.var_t5__blk813_dn8 = assign30580_e28796_d_n8;
        locals.var_t5__blk813_dn9 = assign30580_e28796_d_n9;
        locals.var_t5__blk813_dn10 = assign30580_e28796_d_n10;
        locals.var_t5__blk813_dn11 = assign30580_e28796_d_n11;
        locals.var_t5__blk813_dn12 = assign30580_e28796_d_n12;
        locals.var_t5__blk813_rv = 0.0;

        let (assign30590_e28805, assign30590_e28805_d_n3, assign30590_e28805_d_n4, assign30590_e28805_d_n5, assign30590_e28805_d_n6, assign30590_e28805_d_n7, assign30590_e28805_d_n8, assign30590_e28805_d_n9, assign30590_e28805_d_n10, assign30590_e28805_d_n11, assign30590_e28805_d_n12,) = {
    if ((locals.var_guard1471 != 0.0) && (locals.var_guard1470 == 0.0)) {
        let assign30590_e28803: f64 = (locals.var_t5__blk813 * locals.var_t5__blk813);
        (assign30590_e28803, ((locals.var_t5__blk813_dn3 * locals.var_t5__blk813) + (locals.var_t5__blk813 * locals.var_t5__blk813_dn3)), ((locals.var_t5__blk813_dn4 * locals.var_t5__blk813) + (locals.var_t5__blk813 * locals.var_t5__blk813_dn4)), ((locals.var_t5__blk813_dn5 * locals.var_t5__blk813) + (locals.var_t5__blk813 * locals.var_t5__blk813_dn5)), ((locals.var_t5__blk813_dn6 * locals.var_t5__blk813) + (locals.var_t5__blk813 * locals.var_t5__blk813_dn6)), ((locals.var_t5__blk813_dn7 * locals.var_t5__blk813) + (locals.var_t5__blk813 * locals.var_t5__blk813_dn7)), ((locals.var_t5__blk813_dn8 * locals.var_t5__blk813) + (locals.var_t5__blk813 * locals.var_t5__blk813_dn8)), ((locals.var_t5__blk813_dn9 * locals.var_t5__blk813) + (locals.var_t5__blk813 * locals.var_t5__blk813_dn9)), ((locals.var_t5__blk813_dn10 * locals.var_t5__blk813) + (locals.var_t5__blk813 * locals.var_t5__blk813_dn10)), ((locals.var_t5__blk813_dn11 * locals.var_t5__blk813) + (locals.var_t5__blk813 * locals.var_t5__blk813_dn11)), ((locals.var_t5__blk813_dn12 * locals.var_t5__blk813) + (locals.var_t5__blk813 * locals.var_t5__blk813_dn12)),)
    } else {
        (locals.var_t5__blk813, locals.var_t5__blk813_dn3, locals.var_t5__blk813_dn4, locals.var_t5__blk813_dn5, locals.var_t5__blk813_dn6, locals.var_t5__blk813_dn7, locals.var_t5__blk813_dn8, locals.var_t5__blk813_dn9, locals.var_t5__blk813_dn10, locals.var_t5__blk813_dn11, locals.var_t5__blk813_dn12,)
    }
};
        locals.var_t5__blk813 = assign30590_e28805;
        locals.var_t5__blk813_dn3 = assign30590_e28805_d_n3;
        locals.var_t5__blk813_dn4 = assign30590_e28805_d_n4;
        locals.var_t5__blk813_dn5 = assign30590_e28805_d_n5;
        locals.var_t5__blk813_dn6 = assign30590_e28805_d_n6;
        locals.var_t5__blk813_dn7 = assign30590_e28805_d_n7;
        locals.var_t5__blk813_dn8 = assign30590_e28805_d_n8;
        locals.var_t5__blk813_dn9 = assign30590_e28805_d_n9;
        locals.var_t5__blk813_dn10 = assign30590_e28805_d_n10;
        locals.var_t5__blk813_dn11 = assign30590_e28805_d_n11;
        locals.var_t5__blk813_dn12 = assign30590_e28805_d_n12;
        locals.var_t5__blk813_rv = 0.0;

        let (assign30600_e28820, assign30600_e28820_d_n3, assign30600_e28820_d_n4, assign30600_e28820_d_n5, assign30600_e28820_d_n6, assign30600_e28820_d_n7, assign30600_e28820_d_n8, assign30600_e28820_d_n9, assign30600_e28820_d_n10, assign30600_e28820_d_n11, assign30600_e28820_d_n12,) = {
    if ((locals.var_guard1471 != 0.0) && (locals.var_guard1470 == 0.0)) {
        let assign30600_e28814: f64 = (locals.var_t5__blk813 * p.p227);
        let assign30600_e28816: f64 = (assign30600_e28814 * locals.var_pparam_b4soileff);
        let assign30600_e28817: f64 = (1.0 + assign30600_e28816);
        let assign30600_e28818: f64 = (p.p229 * assign30600_e28817);
        (assign30600_e28818, (p.p229 * (((locals.var_t5__blk813_dn3 * p.p227) * locals.var_pparam_b4soileff) + (assign30600_e28814 * locals.var_pparam_b4soileff_dn3))), (p.p229 * (((locals.var_t5__blk813_dn4 * p.p227) * locals.var_pparam_b4soileff) + (assign30600_e28814 * locals.var_pparam_b4soileff_dn4))), (p.p229 * (((locals.var_t5__blk813_dn5 * p.p227) * locals.var_pparam_b4soileff) + (assign30600_e28814 * locals.var_pparam_b4soileff_dn5))), (p.p229 * (((locals.var_t5__blk813_dn6 * p.p227) * locals.var_pparam_b4soileff) + (assign30600_e28814 * locals.var_pparam_b4soileff_dn6))), (p.p229 * (((locals.var_t5__blk813_dn7 * p.p227) * locals.var_pparam_b4soileff) + (assign30600_e28814 * locals.var_pparam_b4soileff_dn7))), (p.p229 * (((locals.var_t5__blk813_dn8 * p.p227) * locals.var_pparam_b4soileff) + (assign30600_e28814 * locals.var_pparam_b4soileff_dn8))), (p.p229 * (((locals.var_t5__blk813_dn9 * p.p227) * locals.var_pparam_b4soileff) + (assign30600_e28814 * locals.var_pparam_b4soileff_dn9))), (p.p229 * (((locals.var_t5__blk813_dn10 * p.p227) * locals.var_pparam_b4soileff) + (assign30600_e28814 * locals.var_pparam_b4soileff_dn10))), (p.p229 * (((locals.var_t5__blk813_dn11 * p.p227) * locals.var_pparam_b4soileff) + (assign30600_e28814 * locals.var_pparam_b4soileff_dn11))), (p.p229 * (((locals.var_t5__blk813_dn12 * p.p227) * locals.var_pparam_b4soileff) + (assign30600_e28814 * locals.var_pparam_b4soileff_dn12))),)
    } else {
        (locals.var_npart_beta, locals.var_npart_beta_dn3, locals.var_npart_beta_dn4, locals.var_npart_beta_dn5, locals.var_npart_beta_dn6, locals.var_npart_beta_dn7, locals.var_npart_beta_dn8, locals.var_npart_beta_dn9, locals.var_npart_beta_dn10, locals.var_npart_beta_dn11, locals.var_npart_beta_dn12,)
    }
};
        locals.var_npart_beta = assign30600_e28820;
        locals.var_npart_beta_dn3 = assign30600_e28820_d_n3;
        locals.var_npart_beta_dn4 = assign30600_e28820_d_n4;
        locals.var_npart_beta_dn5 = assign30600_e28820_d_n5;
        locals.var_npart_beta_dn6 = assign30600_e28820_d_n6;
        locals.var_npart_beta_dn7 = assign30600_e28820_d_n7;
        locals.var_npart_beta_dn8 = assign30600_e28820_d_n8;
        locals.var_npart_beta_dn9 = assign30600_e28820_d_n9;
        locals.var_npart_beta_dn10 = assign30600_e28820_d_n10;
        locals.var_npart_beta_dn11 = assign30600_e28820_d_n11;
        locals.var_npart_beta_dn12 = assign30600_e28820_d_n12;
        locals.var_npart_beta_rv = 0.0;

        let (assign30670_e28889, assign30670_e28889_d_n3, assign30670_e28889_d_n4, assign30670_e28889_d_n5, assign30670_e28889_d_n6, assign30670_e28889_d_n7, assign30670_e28889_d_n8, assign30670_e28889_d_n9, assign30670_e28889_d_n10, assign30670_e28889_d_n11, assign30670_e28889_d_n12,) = {
    if ((locals.var_guard1471 != 0.0) && (locals.var_guard1470 == 0.0)) {
        let assign30670_e28884: f64 = (locals.var_b4soigm + locals.var_b4soigmbs);
        let assign30670_e28885: f64 = (locals.var_npart_beta * assign30670_e28884);
        let assign30670_e28887: f64 = (assign30670_e28885 + locals.var_b4soigds);
        (assign30670_e28887, (((locals.var_npart_beta_dn3 * assign30670_e28884) + (locals.var_npart_beta * (locals.var_b4soigm_dn3 + locals.var_b4soigmbs_dn3))) + locals.var_b4soigds_dn3), (((locals.var_npart_beta_dn4 * assign30670_e28884) + (locals.var_npart_beta * (locals.var_b4soigm_dn4 + locals.var_b4soigmbs_dn4))) + locals.var_b4soigds_dn4), (((locals.var_npart_beta_dn5 * assign30670_e28884) + (locals.var_npart_beta * (locals.var_b4soigm_dn5 + locals.var_b4soigmbs_dn5))) + locals.var_b4soigds_dn5), (((locals.var_npart_beta_dn6 * assign30670_e28884) + (locals.var_npart_beta * (locals.var_b4soigm_dn6 + locals.var_b4soigmbs_dn6))) + locals.var_b4soigds_dn6), (((locals.var_npart_beta_dn7 * assign30670_e28884) + (locals.var_npart_beta * (locals.var_b4soigm_dn7 + locals.var_b4soigmbs_dn7))) + locals.var_b4soigds_dn7), (((locals.var_npart_beta_dn8 * assign30670_e28884) + (locals.var_npart_beta * (locals.var_b4soigm_dn8 + locals.var_b4soigmbs_dn8))) + locals.var_b4soigds_dn8), (((locals.var_npart_beta_dn9 * assign30670_e28884) + (locals.var_npart_beta * (locals.var_b4soigm_dn9 + locals.var_b4soigmbs_dn9))) + locals.var_b4soigds_dn9), (((locals.var_npart_beta_dn10 * assign30670_e28884) + (locals.var_npart_beta * (locals.var_b4soigm_dn10 + locals.var_b4soigmbs_dn10))) + locals.var_b4soigds_dn10), (((locals.var_npart_beta_dn11 * assign30670_e28884) + (locals.var_npart_beta * (locals.var_b4soigm_dn11 + locals.var_b4soigmbs_dn11))) + locals.var_b4soigds_dn11), (((locals.var_npart_beta_dn12 * assign30670_e28884) + (locals.var_npart_beta * (locals.var_b4soigm_dn12 + locals.var_b4soigmbs_dn12))) + locals.var_b4soigds_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign30670_e28889;
        locals.var_t1__blk809_dn3 = assign30670_e28889_d_n3;
        locals.var_t1__blk809_dn4 = assign30670_e28889_d_n4;
        locals.var_t1__blk809_dn5 = assign30670_e28889_d_n5;
        locals.var_t1__blk809_dn6 = assign30670_e28889_d_n6;
        locals.var_t1__blk809_dn7 = assign30670_e28889_d_n7;
        locals.var_t1__blk809_dn8 = assign30670_e28889_d_n8;
        locals.var_t1__blk809_dn9 = assign30670_e28889_d_n9;
        locals.var_t1__blk809_dn10 = assign30670_e28889_d_n10;
        locals.var_t1__blk809_dn11 = assign30670_e28889_d_n11;
        locals.var_t1__blk809_dn12 = assign30670_e28889_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign30680_e28900, assign30680_e28900_d_n3, assign30680_e28900_d_n4, assign30680_e28900_d_n5, assign30680_e28900_d_n6, assign30680_e28900_d_n7, assign30680_e28900_d_n8, assign30680_e28900_d_n9, assign30680_e28900_d_n10, assign30680_e28900_d_n11, assign30680_e28900_d_n12,) = {
    if ((locals.var_guard1471 != 0.0) && (locals.var_guard1470 == 0.0)) {
        let assign30680_e28896: f64 = (locals.var_t1__blk809 * locals.var_t1__blk809);
        let assign30680_e28898: f64 = (assign30680_e28896 / locals.var_b4soiidovvds);
        (assign30680_e28898, (((((locals.var_t1__blk809_dn3 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn3)) * locals.var_b4soiidovvds) - (assign30680_e28896 * locals.var_b4soiidovvds_dn3)) / (locals.var_b4soiidovvds * locals.var_b4soiidovvds)), (((((locals.var_t1__blk809_dn4 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn4)) * locals.var_b4soiidovvds) - (assign30680_e28896 * locals.var_b4soiidovvds_dn4)) / (locals.var_b4soiidovvds * locals.var_b4soiidovvds)), (((((locals.var_t1__blk809_dn5 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn5)) * locals.var_b4soiidovvds) - (assign30680_e28896 * locals.var_b4soiidovvds_dn5)) / (locals.var_b4soiidovvds * locals.var_b4soiidovvds)), (((((locals.var_t1__blk809_dn6 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn6)) * locals.var_b4soiidovvds) - (assign30680_e28896 * locals.var_b4soiidovvds_dn6)) / (locals.var_b4soiidovvds * locals.var_b4soiidovvds)), (((((locals.var_t1__blk809_dn7 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn7)) * locals.var_b4soiidovvds) - (assign30680_e28896 * locals.var_b4soiidovvds_dn7)) / (locals.var_b4soiidovvds * locals.var_b4soiidovvds)), (((((locals.var_t1__blk809_dn8 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn8)) * locals.var_b4soiidovvds) - (assign30680_e28896 * locals.var_b4soiidovvds_dn8)) / (locals.var_b4soiidovvds * locals.var_b4soiidovvds)), (((((locals.var_t1__blk809_dn9 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn9)) * locals.var_b4soiidovvds) - (assign30680_e28896 * locals.var_b4soiidovvds_dn9)) / (locals.var_b4soiidovvds * locals.var_b4soiidovvds)), (((((locals.var_t1__blk809_dn10 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn10)) * locals.var_b4soiidovvds) - (assign30680_e28896 * locals.var_b4soiidovvds_dn10)) / (locals.var_b4soiidovvds * locals.var_b4soiidovvds)), (((((locals.var_t1__blk809_dn11 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn11)) * locals.var_b4soiidovvds) - (assign30680_e28896 * locals.var_b4soiidovvds_dn11)) / (locals.var_b4soiidovvds * locals.var_b4soiidovvds)), (((((locals.var_t1__blk809_dn12 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn12)) * locals.var_b4soiidovvds) - (assign30680_e28896 * locals.var_b4soiidovvds_dn12)) / (locals.var_b4soiidovvds * locals.var_b4soiidovvds)),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign30680_e28900;
        locals.var_t2__blk810_dn3 = assign30680_e28900_d_n3;
        locals.var_t2__blk810_dn4 = assign30680_e28900_d_n4;
        locals.var_t2__blk810_dn5 = assign30680_e28900_d_n5;
        locals.var_t2__blk810_dn6 = assign30680_e28900_d_n6;
        locals.var_t2__blk810_dn7 = assign30680_e28900_d_n7;
        locals.var_t2__blk810_dn8 = assign30680_e28900_d_n8;
        locals.var_t2__blk810_dn9 = assign30680_e28900_d_n9;
        locals.var_t2__blk810_dn10 = assign30680_e28900_d_n10;
        locals.var_t2__blk810_dn11 = assign30680_e28900_d_n11;
        locals.var_t2__blk810_dn12 = assign30680_e28900_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign30740_e28986, assign30740_e28986_d_n3, assign30740_e28986_d_n4, assign30740_e28986_d_n5, assign30740_e28986_d_n6, assign30740_e28986_d_n7, assign30740_e28986_d_n8, assign30740_e28986_d_n9, assign30740_e28986_d_n10, assign30740_e28986_d_n11, assign30740_e28986_d_n12,) = {
    if ((locals.var_guard1473 != 0.0) && (!(((locals.var_guard1470 != 0.0) || (locals.var_guard1471 != 0.0)) || (locals.var_guard1472 != 0.0)))) {
        let assign30740_e28983: f64 = (locals.var_b4soivdseff * locals.var_b4soiabovvgst2vtm);
        let assign30740_e28984: f64 = (1.0 - assign30740_e28983);
        (assign30740_e28984, (-((locals.var_b4soivdseff_dn3 * locals.var_b4soiabovvgst2vtm) + (locals.var_b4soivdseff * locals.var_b4soiabovvgst2vtm_dn3))), (-((locals.var_b4soivdseff_dn4 * locals.var_b4soiabovvgst2vtm) + (locals.var_b4soivdseff * locals.var_b4soiabovvgst2vtm_dn4))), (-((locals.var_b4soivdseff_dn5 * locals.var_b4soiabovvgst2vtm) + (locals.var_b4soivdseff * locals.var_b4soiabovvgst2vtm_dn5))), (-((locals.var_b4soivdseff_dn6 * locals.var_b4soiabovvgst2vtm) + (locals.var_b4soivdseff * locals.var_b4soiabovvgst2vtm_dn6))), (-((locals.var_b4soivdseff_dn7 * locals.var_b4soiabovvgst2vtm) + (locals.var_b4soivdseff * locals.var_b4soiabovvgst2vtm_dn7))), (-((locals.var_b4soivdseff_dn8 * locals.var_b4soiabovvgst2vtm) + (locals.var_b4soivdseff * locals.var_b4soiabovvgst2vtm_dn8))), (-((locals.var_b4soivdseff_dn9 * locals.var_b4soiabovvgst2vtm) + (locals.var_b4soivdseff * locals.var_b4soiabovvgst2vtm_dn9))), (-((locals.var_b4soivdseff_dn10 * locals.var_b4soiabovvgst2vtm) + (locals.var_b4soivdseff * locals.var_b4soiabovvgst2vtm_dn10))), (-((locals.var_b4soivdseff_dn11 * locals.var_b4soiabovvgst2vtm) + (locals.var_b4soivdseff * locals.var_b4soiabovvgst2vtm_dn11))), (-((locals.var_b4soivdseff_dn12 * locals.var_b4soiabovvgst2vtm) + (locals.var_b4soivdseff * locals.var_b4soiabovvgst2vtm_dn12))),)
    } else {
        (locals.var_eta, locals.var_eta_dn3, locals.var_eta_dn4, locals.var_eta_dn5, locals.var_eta_dn6, locals.var_eta_dn7, locals.var_eta_dn8, locals.var_eta_dn9, locals.var_eta_dn10, locals.var_eta_dn11, locals.var_eta_dn12,)
    }
};
        locals.var_eta = assign30740_e28986;
        locals.var_eta_dn3 = assign30740_e28986_d_n3;
        locals.var_eta_dn4 = assign30740_e28986_d_n4;
        locals.var_eta_dn5 = assign30740_e28986_d_n5;
        locals.var_eta_dn6 = assign30740_e28986_d_n6;
        locals.var_eta_dn7 = assign30740_e28986_d_n7;
        locals.var_eta_dn8 = assign30740_e28986_d_n8;
        locals.var_eta_dn9 = assign30740_e28986_d_n9;
        locals.var_eta_dn10 = assign30740_e28986_d_n10;
        locals.var_eta_dn11 = assign30740_e28986_d_n11;
        locals.var_eta_dn12 = assign30740_e28986_d_n12;
        locals.var_eta_rv = 0.0;

        let (assign30750_e28999, assign30750_e28999_d_n3, assign30750_e28999_d_n4, assign30750_e28999_d_n5, assign30750_e28999_d_n6, assign30750_e28999_d_n7, assign30750_e28999_d_n8, assign30750_e28999_d_n9, assign30750_e28999_d_n10, assign30750_e28999_d_n11, assign30750_e28999_d_n12,) = {
    if ((locals.var_guard1473 != 0.0) && (!(((locals.var_guard1470 != 0.0) || (locals.var_guard1471 != 0.0)) || (locals.var_guard1472 != 0.0)))) {
        let assign30750_e28997: f64 = (1.0 - locals.var_eta);
        (assign30750_e28997, (-locals.var_eta_dn3), (-locals.var_eta_dn4), (-locals.var_eta_dn5), (-locals.var_eta_dn6), (-locals.var_eta_dn7), (-locals.var_eta_dn8), (-locals.var_eta_dn9), (-locals.var_eta_dn10), (-locals.var_eta_dn11), (-locals.var_eta_dn12),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign30750_e28999;
        locals.var_t0__blk808_dn3 = assign30750_e28999_d_n3;
        locals.var_t0__blk808_dn4 = assign30750_e28999_d_n4;
        locals.var_t0__blk808_dn5 = assign30750_e28999_d_n5;
        locals.var_t0__blk808_dn6 = assign30750_e28999_d_n6;
        locals.var_t0__blk808_dn7 = assign30750_e28999_d_n7;
        locals.var_t0__blk808_dn8 = assign30750_e28999_d_n8;
        locals.var_t0__blk808_dn9 = assign30750_e28999_d_n9;
        locals.var_t0__blk808_dn10 = assign30750_e28999_d_n10;
        locals.var_t0__blk808_dn11 = assign30750_e28999_d_n11;
        locals.var_t0__blk808_dn12 = assign30750_e28999_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign30760_e29012, assign30760_e29012_d_n3, assign30760_e29012_d_n4, assign30760_e29012_d_n5, assign30760_e29012_d_n6, assign30760_e29012_d_n7, assign30760_e29012_d_n8, assign30760_e29012_d_n9, assign30760_e29012_d_n10, assign30760_e29012_d_n11, assign30760_e29012_d_n12,) = {
    if ((locals.var_guard1473 != 0.0) && (!(((locals.var_guard1470 != 0.0) || (locals.var_guard1471 != 0.0)) || (locals.var_guard1472 != 0.0)))) {
        let assign30760_e29010: f64 = (1.0 + locals.var_eta);
        (assign30760_e29010, locals.var_eta_dn3, locals.var_eta_dn4, locals.var_eta_dn5, locals.var_eta_dn6, locals.var_eta_dn7, locals.var_eta_dn8, locals.var_eta_dn9, locals.var_eta_dn10, locals.var_eta_dn11, locals.var_eta_dn12,)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign30760_e29012;
        locals.var_t1__blk809_dn3 = assign30760_e29012_d_n3;
        locals.var_t1__blk809_dn4 = assign30760_e29012_d_n4;
        locals.var_t1__blk809_dn5 = assign30760_e29012_d_n5;
        locals.var_t1__blk809_dn6 = assign30760_e29012_d_n6;
        locals.var_t1__blk809_dn7 = assign30760_e29012_d_n7;
        locals.var_t1__blk809_dn8 = assign30760_e29012_d_n8;
        locals.var_t1__blk809_dn9 = assign30760_e29012_d_n9;
        locals.var_t1__blk809_dn10 = assign30760_e29012_d_n10;
        locals.var_t1__blk809_dn11 = assign30760_e29012_d_n11;
        locals.var_t1__blk809_dn12 = assign30760_e29012_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign30770_e29033, assign30770_e29033_d_n3, assign30770_e29033_d_n4, assign30770_e29033_d_n5, assign30770_e29033_d_n6, assign30770_e29033_d_n7, assign30770_e29033_d_n8, assign30770_e29033_d_n9, assign30770_e29033_d_n10, assign30770_e29033_d_n11, assign30770_e29033_d_n12,) = {
    if ((locals.var_guard1473 != 0.0) && (!(((locals.var_guard1470 != 0.0) || (locals.var_guard1471 != 0.0)) || (locals.var_guard1472 != 0.0)))) {
        let assign30770_e29024: f64 = (2.0 * locals.var_b4soiabulk);
        let assign30770_e29026: f64 = (assign30770_e29024 * locals.var_b4soivtm);
        let assign30770_e29029: f64 = (locals.var_b4soivgsteff + 1e-10);
        let assign30770_e29030: f64 = (assign30770_e29026 / assign30770_e29029);
        let assign30770_e29031: f64 = (locals.var_t1__blk809 + assign30770_e29030);
        (assign30770_e29031, (locals.var_t1__blk809_dn3 + (((((2.0 * locals.var_b4soiabulk_dn3) * locals.var_b4soivtm) * assign30770_e29029) - (assign30770_e29026 * locals.var_b4soivgsteff_dn3)) / (assign30770_e29029 * assign30770_e29029))), (locals.var_t1__blk809_dn4 + ((((((2.0 * locals.var_b4soiabulk_dn4) * locals.var_b4soivtm) + (assign30770_e29024 * locals.var_b4soivtm_dn4)) * assign30770_e29029) - (assign30770_e29026 * locals.var_b4soivgsteff_dn4)) / (assign30770_e29029 * assign30770_e29029))), (locals.var_t1__blk809_dn5 + ((((((2.0 * locals.var_b4soiabulk_dn5) * locals.var_b4soivtm) + (assign30770_e29024 * locals.var_b4soivtm_dn5)) * assign30770_e29029) - (assign30770_e29026 * locals.var_b4soivgsteff_dn5)) / (assign30770_e29029 * assign30770_e29029))), (locals.var_t1__blk809_dn6 + ((((((2.0 * locals.var_b4soiabulk_dn6) * locals.var_b4soivtm) + (assign30770_e29024 * locals.var_b4soivtm_dn6)) * assign30770_e29029) - (assign30770_e29026 * locals.var_b4soivgsteff_dn6)) / (assign30770_e29029 * assign30770_e29029))), (locals.var_t1__blk809_dn7 + (((((2.0 * locals.var_b4soiabulk_dn7) * locals.var_b4soivtm) * assign30770_e29029) - (assign30770_e29026 * locals.var_b4soivgsteff_dn7)) / (assign30770_e29029 * assign30770_e29029))), (locals.var_t1__blk809_dn8 + (((((2.0 * locals.var_b4soiabulk_dn8) * locals.var_b4soivtm) * assign30770_e29029) - (assign30770_e29026 * locals.var_b4soivgsteff_dn8)) / (assign30770_e29029 * assign30770_e29029))), (locals.var_t1__blk809_dn9 + (((((2.0 * locals.var_b4soiabulk_dn9) * locals.var_b4soivtm) * assign30770_e29029) - (assign30770_e29026 * locals.var_b4soivgsteff_dn9)) / (assign30770_e29029 * assign30770_e29029))), (locals.var_t1__blk809_dn10 + (((((2.0 * locals.var_b4soiabulk_dn10) * locals.var_b4soivtm) * assign30770_e29029) - (assign30770_e29026 * locals.var_b4soivgsteff_dn10)) / (assign30770_e29029 * assign30770_e29029))), (locals.var_t1__blk809_dn11 + (((((2.0 * locals.var_b4soiabulk_dn11) * locals.var_b4soivtm) * assign30770_e29029) - (assign30770_e29026 * locals.var_b4soivgsteff_dn11)) / (assign30770_e29029 * assign30770_e29029))), (locals.var_t1__blk809_dn12 + (((((2.0 * locals.var_b4soiabulk_dn12) * locals.var_b4soivtm) * assign30770_e29029) - (assign30770_e29026 * locals.var_b4soivgsteff_dn12)) / (assign30770_e29029 * assign30770_e29029))),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign30770_e29033;
        locals.var_t2__blk810_dn3 = assign30770_e29033_d_n3;
        locals.var_t2__blk810_dn4 = assign30770_e29033_d_n4;
        locals.var_t2__blk810_dn5 = assign30770_e29033_d_n5;
        locals.var_t2__blk810_dn6 = assign30770_e29033_d_n6;
        locals.var_t2__blk810_dn7 = assign30770_e29033_d_n7;
        locals.var_t2__blk810_dn8 = assign30770_e29033_d_n8;
        locals.var_t2__blk810_dn9 = assign30770_e29033_d_n9;
        locals.var_t2__blk810_dn10 = assign30770_e29033_d_n10;
        locals.var_t2__blk810_dn11 = assign30770_e29033_d_n11;
        locals.var_t2__blk810_dn12 = assign30770_e29033_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign30780_e29050, assign30780_e29050_d_n3, assign30780_e29050_d_n4, assign30780_e29050_d_n5, assign30780_e29050_d_n6, assign30780_e29050_d_n7, assign30780_e29050_d_n8, assign30780_e29050_d_n9, assign30780_e29050_d_n10, assign30780_e29050_d_n11, assign30780_e29050_d_n12,) = {
    if ((locals.var_guard1473 != 0.0) && (!(((locals.var_guard1470 != 0.0) || (locals.var_guard1471 != 0.0)) || (locals.var_guard1472 != 0.0)))) {
        let assign30780_e29046: f64 = (locals.var_b4soivdseff / locals.var_esatl);
        let assign30780_e29047: f64 = (1.0 + assign30780_e29046);
        let assign30780_e29048: f64 = (locals.var_leff * assign30780_e29047);
        (assign30780_e29048, ((locals.var_leff_dn3 * assign30780_e29047) + (locals.var_leff * (((locals.var_b4soivdseff_dn3 * locals.var_esatl) - (locals.var_b4soivdseff * locals.var_esatl_dn3)) / (locals.var_esatl * locals.var_esatl)))), ((locals.var_leff_dn4 * assign30780_e29047) + (locals.var_leff * (((locals.var_b4soivdseff_dn4 * locals.var_esatl) - (locals.var_b4soivdseff * locals.var_esatl_dn4)) / (locals.var_esatl * locals.var_esatl)))), ((locals.var_leff_dn5 * assign30780_e29047) + (locals.var_leff * (((locals.var_b4soivdseff_dn5 * locals.var_esatl) - (locals.var_b4soivdseff * locals.var_esatl_dn5)) / (locals.var_esatl * locals.var_esatl)))), ((locals.var_leff_dn6 * assign30780_e29047) + (locals.var_leff * (((locals.var_b4soivdseff_dn6 * locals.var_esatl) - (locals.var_b4soivdseff * locals.var_esatl_dn6)) / (locals.var_esatl * locals.var_esatl)))), ((locals.var_leff_dn7 * assign30780_e29047) + (locals.var_leff * (((locals.var_b4soivdseff_dn7 * locals.var_esatl) - (locals.var_b4soivdseff * locals.var_esatl_dn7)) / (locals.var_esatl * locals.var_esatl)))), ((locals.var_leff_dn8 * assign30780_e29047) + (locals.var_leff * (((locals.var_b4soivdseff_dn8 * locals.var_esatl) - (locals.var_b4soivdseff * locals.var_esatl_dn8)) / (locals.var_esatl * locals.var_esatl)))), ((locals.var_leff_dn9 * assign30780_e29047) + (locals.var_leff * (((locals.var_b4soivdseff_dn9 * locals.var_esatl) - (locals.var_b4soivdseff * locals.var_esatl_dn9)) / (locals.var_esatl * locals.var_esatl)))), ((locals.var_leff_dn10 * assign30780_e29047) + (locals.var_leff * (((locals.var_b4soivdseff_dn10 * locals.var_esatl) - (locals.var_b4soivdseff * locals.var_esatl_dn10)) / (locals.var_esatl * locals.var_esatl)))), ((locals.var_leff_dn11 * assign30780_e29047) + (locals.var_leff * (((locals.var_b4soivdseff_dn11 * locals.var_esatl) - (locals.var_b4soivdseff * locals.var_esatl_dn11)) / (locals.var_esatl * locals.var_esatl)))), ((locals.var_leff_dn12 * assign30780_e29047) + (locals.var_leff * (((locals.var_b4soivdseff_dn12 * locals.var_esatl) - (locals.var_b4soivdseff * locals.var_esatl_dn12)) / (locals.var_esatl * locals.var_esatl)))),)
    } else {
        (locals.var_lvsat, locals.var_lvsat_dn3, locals.var_lvsat_dn4, locals.var_lvsat_dn5, locals.var_lvsat_dn6, locals.var_lvsat_dn7, locals.var_lvsat_dn8, locals.var_lvsat_dn9, locals.var_lvsat_dn10, locals.var_lvsat_dn11, locals.var_lvsat_dn12,)
    }
};
        locals.var_lvsat = assign30780_e29050;
        locals.var_lvsat_dn3 = assign30780_e29050_d_n3;
        locals.var_lvsat_dn4 = assign30780_e29050_d_n4;
        locals.var_lvsat_dn5 = assign30780_e29050_d_n5;
        locals.var_lvsat_dn6 = assign30780_e29050_d_n6;
        locals.var_lvsat_dn7 = assign30780_e29050_d_n7;
        locals.var_lvsat_dn8 = assign30780_e29050_d_n8;
        locals.var_lvsat_dn9 = assign30780_e29050_d_n9;
        locals.var_lvsat_dn10 = assign30780_e29050_d_n10;
        locals.var_lvsat_dn11 = assign30780_e29050_d_n11;
        locals.var_lvsat_dn12 = assign30780_e29050_d_n12;
        locals.var_lvsat_rv = 0.0;

        let (assign30790_e29063, assign30790_e29063_d_n3, assign30790_e29063_d_n4, assign30790_e29063_d_n5, assign30790_e29063_d_n6, assign30790_e29063_d_n7, assign30790_e29063_d_n8, assign30790_e29063_d_n9, assign30790_e29063_d_n10, assign30790_e29063_d_n11, assign30790_e29063_d_n12,) = {
    if ((locals.var_guard1473 != 0.0) && (!(((locals.var_guard1470 != 0.0) || (locals.var_guard1471 != 0.0)) || (locals.var_guard1472 != 0.0)))) {
        let assign30790_e29061: f64 = (locals.var_leff / locals.var_lvsat);
        (assign30790_e29061, (((locals.var_leff_dn3 * locals.var_lvsat) - (locals.var_leff * locals.var_lvsat_dn3)) / (locals.var_lvsat * locals.var_lvsat)), (((locals.var_leff_dn4 * locals.var_lvsat) - (locals.var_leff * locals.var_lvsat_dn4)) / (locals.var_lvsat * locals.var_lvsat)), (((locals.var_leff_dn5 * locals.var_lvsat) - (locals.var_leff * locals.var_lvsat_dn5)) / (locals.var_lvsat * locals.var_lvsat)), (((locals.var_leff_dn6 * locals.var_lvsat) - (locals.var_leff * locals.var_lvsat_dn6)) / (locals.var_lvsat * locals.var_lvsat)), (((locals.var_leff_dn7 * locals.var_lvsat) - (locals.var_leff * locals.var_lvsat_dn7)) / (locals.var_lvsat * locals.var_lvsat)), (((locals.var_leff_dn8 * locals.var_lvsat) - (locals.var_leff * locals.var_lvsat_dn8)) / (locals.var_lvsat * locals.var_lvsat)), (((locals.var_leff_dn9 * locals.var_lvsat) - (locals.var_leff * locals.var_lvsat_dn9)) / (locals.var_lvsat * locals.var_lvsat)), (((locals.var_leff_dn10 * locals.var_lvsat) - (locals.var_leff * locals.var_lvsat_dn10)) / (locals.var_lvsat * locals.var_lvsat)), (((locals.var_leff_dn11 * locals.var_lvsat) - (locals.var_leff * locals.var_lvsat_dn11)) / (locals.var_lvsat * locals.var_lvsat)), (((locals.var_leff_dn12 * locals.var_lvsat) - (locals.var_leff * locals.var_lvsat_dn12)) / (locals.var_lvsat * locals.var_lvsat)),)
    } else {
        (locals.var_t6__blk814, locals.var_t6__blk814_dn3, locals.var_t6__blk814_dn4, locals.var_t6__blk814_dn5, locals.var_t6__blk814_dn6, locals.var_t6__blk814_dn7, locals.var_t6__blk814_dn8, locals.var_t6__blk814_dn9, locals.var_t6__blk814_dn10, locals.var_t6__blk814_dn11, locals.var_t6__blk814_dn12,)
    }
};
        locals.var_t6__blk814 = assign30790_e29063;
        locals.var_t6__blk814_dn3 = assign30790_e29063_d_n3;
        locals.var_t6__blk814_dn4 = assign30790_e29063_d_n4;
        locals.var_t6__blk814_dn5 = assign30790_e29063_d_n5;
        locals.var_t6__blk814_dn6 = assign30790_e29063_d_n6;
        locals.var_t6__blk814_dn7 = assign30790_e29063_d_n7;
        locals.var_t6__blk814_dn8 = assign30790_e29063_d_n8;
        locals.var_t6__blk814_dn9 = assign30790_e29063_d_n9;
        locals.var_t6__blk814_dn10 = assign30790_e29063_d_n10;
        locals.var_t6__blk814_dn11 = assign30790_e29063_d_n11;
        locals.var_t6__blk814_dn12 = assign30790_e29063_d_n12;
        locals.var_t6__blk814_rv = 0.0;

        let (assign30810_e29099, assign30810_e29099_d_n3, assign30810_e29099_d_n4, assign30810_e29099_d_n5, assign30810_e29099_d_n6, assign30810_e29099_d_n7, assign30810_e29099_d_n8, assign30810_e29099_d_n9, assign30810_e29099_d_n10, assign30810_e29099_d_n11, assign30810_e29099_d_n12,) = {
    if ((locals.var_guard1473 != 0.0) && (!(((locals.var_guard1470 != 0.0) || (locals.var_guard1471 != 0.0)) || (locals.var_guard1472 != 0.0)))) {
        let assign30810_e29097: f64 = (locals.var_t2__blk810 * locals.var_t2__blk810);
        (assign30810_e29097, ((locals.var_t2__blk810_dn3 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn3)), ((locals.var_t2__blk810_dn4 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn4)), ((locals.var_t2__blk810_dn5 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn5)), ((locals.var_t2__blk810_dn6 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn6)), ((locals.var_t2__blk810_dn7 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn7)), ((locals.var_t2__blk810_dn8 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn8)), ((locals.var_t2__blk810_dn9 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn9)), ((locals.var_t2__blk810_dn10 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn10)), ((locals.var_t2__blk810_dn11 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn11)), ((locals.var_t2__blk810_dn12 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn12)),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign30810_e29099;
        locals.var_t3__blk811_dn3 = assign30810_e29099_d_n3;
        locals.var_t3__blk811_dn4 = assign30810_e29099_d_n4;
        locals.var_t3__blk811_dn5 = assign30810_e29099_d_n5;
        locals.var_t3__blk811_dn6 = assign30810_e29099_d_n6;
        locals.var_t3__blk811_dn7 = assign30810_e29099_d_n7;
        locals.var_t3__blk811_dn8 = assign30810_e29099_d_n8;
        locals.var_t3__blk811_dn9 = assign30810_e29099_d_n9;
        locals.var_t3__blk811_dn10 = assign30810_e29099_d_n10;
        locals.var_t3__blk811_dn11 = assign30810_e29099_d_n11;
        locals.var_t3__blk811_dn12 = assign30810_e29099_d_n12;
        locals.var_t3__blk811_rv = 0.0;

        let (assign30820_e29112, assign30820_e29112_d_n3, assign30820_e29112_d_n4, assign30820_e29112_d_n5, assign30820_e29112_d_n6, assign30820_e29112_d_n7, assign30820_e29112_d_n8, assign30820_e29112_d_n9, assign30820_e29112_d_n10, assign30820_e29112_d_n11, assign30820_e29112_d_n12,) = {
    if ((locals.var_guard1473 != 0.0) && (!(((locals.var_guard1470 != 0.0) || (locals.var_guard1471 != 0.0)) || (locals.var_guard1472 != 0.0)))) {
        let assign30820_e29110: f64 = (locals.var_t0__blk808 * locals.var_t0__blk808);
        (assign30820_e29110, ((locals.var_t0__blk808_dn3 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn3)), ((locals.var_t0__blk808_dn4 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn4)), ((locals.var_t0__blk808_dn5 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn5)), ((locals.var_t0__blk808_dn6 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn6)), ((locals.var_t0__blk808_dn7 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn7)), ((locals.var_t0__blk808_dn8 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn8)), ((locals.var_t0__blk808_dn9 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn9)), ((locals.var_t0__blk808_dn10 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn10)), ((locals.var_t0__blk808_dn11 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn11)), ((locals.var_t0__blk808_dn12 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn12)),)
    } else {
        (locals.var_t4__blk812, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    }
};
        locals.var_t4__blk812 = assign30820_e29112;
        locals.var_t4__blk812_dn3 = assign30820_e29112_d_n3;
        locals.var_t4__blk812_dn4 = assign30820_e29112_d_n4;
        locals.var_t4__blk812_dn5 = assign30820_e29112_d_n5;
        locals.var_t4__blk812_dn6 = assign30820_e29112_d_n6;
        locals.var_t4__blk812_dn7 = assign30820_e29112_d_n7;
        locals.var_t4__blk812_dn8 = assign30820_e29112_d_n8;
        locals.var_t4__blk812_dn9 = assign30820_e29112_d_n9;
        locals.var_t4__blk812_dn10 = assign30820_e29112_d_n10;
        locals.var_t4__blk812_dn11 = assign30820_e29112_d_n11;
        locals.var_t4__blk812_dn12 = assign30820_e29112_d_n12;
        locals.var_t4__blk812_rv = 0.0;

        let (assign30830_e29125, assign30830_e29125_d_n3, assign30830_e29125_d_n4, assign30830_e29125_d_n5, assign30830_e29125_d_n6, assign30830_e29125_d_n7, assign30830_e29125_d_n8, assign30830_e29125_d_n9, assign30830_e29125_d_n10, assign30830_e29125_d_n11, assign30830_e29125_d_n12,) = {
    if ((locals.var_guard1473 != 0.0) && (!(((locals.var_guard1470 != 0.0) || (locals.var_guard1471 != 0.0)) || (locals.var_guard1472 != 0.0)))) {
        let assign30830_e29123: f64 = (locals.var_t3__blk811 * locals.var_t3__blk811);
        (assign30830_e29123, ((locals.var_t3__blk811_dn3 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn3)), ((locals.var_t3__blk811_dn4 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn4)), ((locals.var_t3__blk811_dn5 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn5)), ((locals.var_t3__blk811_dn6 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn6)), ((locals.var_t3__blk811_dn7 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn7)), ((locals.var_t3__blk811_dn8 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn8)), ((locals.var_t3__blk811_dn9 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn9)), ((locals.var_t3__blk811_dn10 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn10)), ((locals.var_t3__blk811_dn11 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn11)), ((locals.var_t3__blk811_dn12 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn12)),)
    } else {
        (locals.var_t5__blk813, locals.var_t5__blk813_dn3, locals.var_t5__blk813_dn4, locals.var_t5__blk813_dn5, locals.var_t5__blk813_dn6, locals.var_t5__blk813_dn7, locals.var_t5__blk813_dn8, locals.var_t5__blk813_dn9, locals.var_t5__blk813_dn10, locals.var_t5__blk813_dn11, locals.var_t5__blk813_dn12,)
    }
};
        locals.var_t5__blk813 = assign30830_e29125;
        locals.var_t5__blk813_dn3 = assign30830_e29125_d_n3;
        locals.var_t5__blk813_dn4 = assign30830_e29125_d_n4;
        locals.var_t5__blk813_dn5 = assign30830_e29125_d_n5;
        locals.var_t5__blk813_dn6 = assign30830_e29125_d_n6;
        locals.var_t5__blk813_dn7 = assign30830_e29125_d_n7;
        locals.var_t5__blk813_dn8 = assign30830_e29125_d_n8;
        locals.var_t5__blk813_dn9 = assign30830_e29125_d_n9;
        locals.var_t5__blk813_dn10 = assign30830_e29125_d_n10;
        locals.var_t5__blk813_dn11 = assign30830_e29125_d_n11;
        locals.var_t5__blk813_dn12 = assign30830_e29125_d_n12;
        locals.var_t5__blk813_rv = 0.0;

        let (assign30850_e29181, assign30850_e29181_d_n3, assign30850_e29181_d_n4, assign30850_e29181_d_n5, assign30850_e29181_d_n6, assign30850_e29181_d_n7, assign30850_e29181_d_n8, assign30850_e29181_d_n9, assign30850_e29181_d_n10, assign30850_e29181_d_n11, assign30850_e29181_d_n12,) = {
    if ((locals.var_guard1473 != 0.0) && (!(((locals.var_guard1470 != 0.0) || (locals.var_guard1471 != 0.0)) || (locals.var_guard1472 != 0.0)))) {
        let assign30850_e29179: f64 = (locals.var_t0__blk808 / locals.var_t2__blk810);
        (assign30850_e29179, (((locals.var_t0__blk808_dn3 * locals.var_t2__blk810) - (locals.var_t0__blk808 * locals.var_t2__blk810_dn3)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t0__blk808_dn4 * locals.var_t2__blk810) - (locals.var_t0__blk808 * locals.var_t2__blk810_dn4)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t0__blk808_dn5 * locals.var_t2__blk810) - (locals.var_t0__blk808 * locals.var_t2__blk810_dn5)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t0__blk808_dn6 * locals.var_t2__blk810) - (locals.var_t0__blk808 * locals.var_t2__blk810_dn6)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t0__blk808_dn7 * locals.var_t2__blk810) - (locals.var_t0__blk808 * locals.var_t2__blk810_dn7)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t0__blk808_dn8 * locals.var_t2__blk810) - (locals.var_t0__blk808 * locals.var_t2__blk810_dn8)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t0__blk808_dn9 * locals.var_t2__blk810) - (locals.var_t0__blk808 * locals.var_t2__blk810_dn9)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t0__blk808_dn10 * locals.var_t2__blk810) - (locals.var_t0__blk808 * locals.var_t2__blk810_dn10)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t0__blk808_dn11 * locals.var_t2__blk810) - (locals.var_t0__blk808 * locals.var_t2__blk810_dn11)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t0__blk808_dn12 * locals.var_t2__blk810) - (locals.var_t0__blk808 * locals.var_t2__blk810_dn12)) / (locals.var_t2__blk810 * locals.var_t2__blk810)),)
    } else {
        (locals.var_t7__blk815, locals.var_t7__blk815_dn3, locals.var_t7__blk815_dn4, locals.var_t7__blk815_dn5, locals.var_t7__blk815_dn6, locals.var_t7__blk815_dn7, locals.var_t7__blk815_dn8, locals.var_t7__blk815_dn9, locals.var_t7__blk815_dn10, locals.var_t7__blk815_dn11, locals.var_t7__blk815_dn12,)
    }
};
        locals.var_t7__blk815 = assign30850_e29181;
        locals.var_t7__blk815_dn3 = assign30850_e29181_d_n3;
        locals.var_t7__blk815_dn4 = assign30850_e29181_d_n4;
        locals.var_t7__blk815_dn5 = assign30850_e29181_d_n5;
        locals.var_t7__blk815_dn6 = assign30850_e29181_d_n6;
        locals.var_t7__blk815_dn7 = assign30850_e29181_d_n7;
        locals.var_t7__blk815_dn8 = assign30850_e29181_d_n8;
        locals.var_t7__blk815_dn9 = assign30850_e29181_d_n9;
        locals.var_t7__blk815_dn10 = assign30850_e29181_d_n10;
        locals.var_t7__blk815_dn11 = assign30850_e29181_d_n11;
        locals.var_t7__blk815_dn12 = assign30850_e29181_d_n12;
        locals.var_t7__blk815_rv = 0.0;

        let (assign30870_e29217, assign30870_e29217_d_n3, assign30870_e29217_d_n4, assign30870_e29217_d_n5, assign30870_e29217_d_n6, assign30870_e29217_d_n7, assign30870_e29217_d_n8, assign30870_e29217_d_n9, assign30870_e29217_d_n10, assign30870_e29217_d_n11, assign30870_e29217_d_n12,) = {
    if ((locals.var_guard1473 != 0.0) && (!(((locals.var_guard1470 != 0.0) || (locals.var_guard1471 != 0.0)) || (locals.var_guard1472 != 0.0)))) {
        let assign30870_e29215: f64 = (locals.var_b4soivgsteff / locals.var_esatl);
        (assign30870_e29215, (((locals.var_b4soivgsteff_dn3 * locals.var_esatl) - (locals.var_b4soivgsteff * locals.var_esatl_dn3)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_b4soivgsteff_dn4 * locals.var_esatl) - (locals.var_b4soivgsteff * locals.var_esatl_dn4)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_b4soivgsteff_dn5 * locals.var_esatl) - (locals.var_b4soivgsteff * locals.var_esatl_dn5)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_b4soivgsteff_dn6 * locals.var_esatl) - (locals.var_b4soivgsteff * locals.var_esatl_dn6)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_b4soivgsteff_dn7 * locals.var_esatl) - (locals.var_b4soivgsteff * locals.var_esatl_dn7)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_b4soivgsteff_dn8 * locals.var_esatl) - (locals.var_b4soivgsteff * locals.var_esatl_dn8)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_b4soivgsteff_dn9 * locals.var_esatl) - (locals.var_b4soivgsteff * locals.var_esatl_dn9)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_b4soivgsteff_dn10 * locals.var_esatl) - (locals.var_b4soivgsteff * locals.var_esatl_dn10)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_b4soivgsteff_dn11 * locals.var_esatl) - (locals.var_b4soivgsteff * locals.var_esatl_dn11)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_b4soivgsteff_dn12 * locals.var_esatl) - (locals.var_b4soivgsteff * locals.var_esatl_dn12)) / (locals.var_esatl * locals.var_esatl)),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12,)
    }
};
        locals.var_t8 = assign30870_e29217;
        locals.var_t8_dn3 = assign30870_e29217_d_n3;
        locals.var_t8_dn4 = assign30870_e29217_d_n4;
        locals.var_t8_dn5 = assign30870_e29217_d_n5;
        locals.var_t8_dn6 = assign30870_e29217_d_n6;
        locals.var_t8_dn7 = assign30870_e29217_d_n7;
        locals.var_t8_dn8 = assign30870_e29217_d_n8;
        locals.var_t8_dn9 = assign30870_e29217_d_n9;
        locals.var_t8_dn10 = assign30870_e29217_d_n10;
        locals.var_t8_dn11 = assign30870_e29217_d_n11;
        locals.var_t8_dn12 = assign30870_e29217_d_n12;
        locals.var_t8_rv = 0.0;

        let (assign30880_e29230, assign30880_e29230_d_n3, assign30880_e29230_d_n4, assign30880_e29230_d_n5, assign30880_e29230_d_n6, assign30880_e29230_d_n7, assign30880_e29230_d_n8, assign30880_e29230_d_n9, assign30880_e29230_d_n10, assign30880_e29230_d_n11, assign30880_e29230_d_n12,) = {
    if ((locals.var_guard1473 != 0.0) && (!(((locals.var_guard1470 != 0.0) || (locals.var_guard1471 != 0.0)) || (locals.var_guard1472 != 0.0)))) {
        let assign30880_e29228: f64 = (locals.var_t8 * locals.var_t8);
        (assign30880_e29228, ((locals.var_t8_dn3 * locals.var_t8) + (locals.var_t8 * locals.var_t8_dn3)), ((locals.var_t8_dn4 * locals.var_t8) + (locals.var_t8 * locals.var_t8_dn4)), ((locals.var_t8_dn5 * locals.var_t8) + (locals.var_t8 * locals.var_t8_dn5)), ((locals.var_t8_dn6 * locals.var_t8) + (locals.var_t8 * locals.var_t8_dn6)), ((locals.var_t8_dn7 * locals.var_t8) + (locals.var_t8 * locals.var_t8_dn7)), ((locals.var_t8_dn8 * locals.var_t8) + (locals.var_t8 * locals.var_t8_dn8)), ((locals.var_t8_dn9 * locals.var_t8) + (locals.var_t8 * locals.var_t8_dn9)), ((locals.var_t8_dn10 * locals.var_t8) + (locals.var_t8 * locals.var_t8_dn10)), ((locals.var_t8_dn11 * locals.var_t8) + (locals.var_t8 * locals.var_t8_dn11)), ((locals.var_t8_dn12 * locals.var_t8) + (locals.var_t8 * locals.var_t8_dn12)),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12,)
    }
};
        locals.var_t8 = assign30880_e29230;
        locals.var_t8_dn3 = assign30880_e29230_d_n3;
        locals.var_t8_dn4 = assign30880_e29230_d_n4;
        locals.var_t8_dn5 = assign30880_e29230_d_n5;
        locals.var_t8_dn6 = assign30880_e29230_d_n6;
        locals.var_t8_dn7 = assign30880_e29230_d_n7;
        locals.var_t8_dn8 = assign30880_e29230_d_n8;
        locals.var_t8_dn9 = assign30880_e29230_d_n9;
        locals.var_t8_dn10 = assign30880_e29230_d_n10;
        locals.var_t8_dn11 = assign30880_e29230_d_n11;
        locals.var_t8_dn12 = assign30880_e29230_d_n12;
        locals.var_t8_rv = 0.0;

        let (assign30950_e29320, assign30950_e29320_d_n3, assign30950_e29320_d_n4, assign30950_e29320_d_n5, assign30950_e29320_d_n6, assign30950_e29320_d_n7, assign30950_e29320_d_n8, assign30950_e29320_d_n9, assign30950_e29320_d_n10, assign30950_e29320_d_n11, assign30950_e29320_d_n12,) = {
    if ((locals.var_guard1473 != 0.0) && (!(((locals.var_guard1470 != 0.0) || (locals.var_guard1471 != 0.0)) || (locals.var_guard1472 != 0.0)))) {
        let assign30950_e29314: f64 = (locals.var_t8 * p.p227);
        let assign30950_e29316: f64 = (assign30950_e29314 * locals.var_leff);
        let assign30950_e29317: f64 = (1.0 + assign30950_e29316);
        let assign30950_e29318: f64 = (p.p229 * assign30950_e29317);
        (assign30950_e29318, (p.p229 * (((locals.var_t8_dn3 * p.p227) * locals.var_leff) + (assign30950_e29314 * locals.var_leff_dn3))), (p.p229 * (((locals.var_t8_dn4 * p.p227) * locals.var_leff) + (assign30950_e29314 * locals.var_leff_dn4))), (p.p229 * (((locals.var_t8_dn5 * p.p227) * locals.var_leff) + (assign30950_e29314 * locals.var_leff_dn5))), (p.p229 * (((locals.var_t8_dn6 * p.p227) * locals.var_leff) + (assign30950_e29314 * locals.var_leff_dn6))), (p.p229 * (((locals.var_t8_dn7 * p.p227) * locals.var_leff) + (assign30950_e29314 * locals.var_leff_dn7))), (p.p229 * (((locals.var_t8_dn8 * p.p227) * locals.var_leff) + (assign30950_e29314 * locals.var_leff_dn8))), (p.p229 * (((locals.var_t8_dn9 * p.p227) * locals.var_leff) + (assign30950_e29314 * locals.var_leff_dn9))), (p.p229 * (((locals.var_t8_dn10 * p.p227) * locals.var_leff) + (assign30950_e29314 * locals.var_leff_dn10))), (p.p229 * (((locals.var_t8_dn11 * p.p227) * locals.var_leff) + (assign30950_e29314 * locals.var_leff_dn11))), (p.p229 * (((locals.var_t8_dn12 * p.p227) * locals.var_leff) + (assign30950_e29314 * locals.var_leff_dn12))),)
    } else {
        (locals.var_npart_beta, locals.var_npart_beta_dn3, locals.var_npart_beta_dn4, locals.var_npart_beta_dn5, locals.var_npart_beta_dn6, locals.var_npart_beta_dn7, locals.var_npart_beta_dn8, locals.var_npart_beta_dn9, locals.var_npart_beta_dn10, locals.var_npart_beta_dn11, locals.var_npart_beta_dn12,)
    }
};
        locals.var_npart_beta = assign30950_e29320;
        locals.var_npart_beta_dn3 = assign30950_e29320_d_n3;
        locals.var_npart_beta_dn4 = assign30950_e29320_d_n4;
        locals.var_npart_beta_dn5 = assign30950_e29320_d_n5;
        locals.var_npart_beta_dn6 = assign30950_e29320_d_n6;
        locals.var_npart_beta_dn7 = assign30950_e29320_d_n7;
        locals.var_npart_beta_dn8 = assign30950_e29320_d_n8;
        locals.var_npart_beta_dn9 = assign30950_e29320_d_n9;
        locals.var_npart_beta_dn10 = assign30950_e29320_d_n10;
        locals.var_npart_beta_dn11 = assign30950_e29320_d_n11;
        locals.var_npart_beta_dn12 = assign30950_e29320_d_n12;
        locals.var_npart_beta_rv = 0.0;

        let (assign31020_e29437, assign31020_e29437_d_n3, assign31020_e29437_d_n4, assign31020_e29437_d_n5, assign31020_e29437_d_n6, assign31020_e29437_d_n7, assign31020_e29437_d_n8, assign31020_e29437_d_n9, assign31020_e29437_d_n10, assign31020_e29437_d_n11, assign31020_e29437_d_n12,) = {
    if ((locals.var_guard1473 != 0.0) && (!(((locals.var_guard1470 != 0.0) || (locals.var_guard1471 != 0.0)) || (locals.var_guard1472 != 0.0)))) {
        let assign31020_e29431: f64 = (p.p3 * locals.var_b4soicox);
        let assign31020_e29433: f64 = (assign31020_e29431 * locals.var_pparam_b4soiweffcv);
        let assign31020_e29435: f64 = (assign31020_e29433 * locals.var_pparam_b4soileffcv);
        (assign31020_e29435, (((assign31020_e29431 * locals.var_pparam_b4soiweffcv_dn3) * locals.var_pparam_b4soileffcv) + (assign31020_e29433 * locals.var_pparam_b4soileffcv_dn3)), (((assign31020_e29431 * locals.var_pparam_b4soiweffcv_dn4) * locals.var_pparam_b4soileffcv) + (assign31020_e29433 * locals.var_pparam_b4soileffcv_dn4)), (((assign31020_e29431 * locals.var_pparam_b4soiweffcv_dn5) * locals.var_pparam_b4soileffcv) + (assign31020_e29433 * locals.var_pparam_b4soileffcv_dn5)), (((assign31020_e29431 * locals.var_pparam_b4soiweffcv_dn6) * locals.var_pparam_b4soileffcv) + (assign31020_e29433 * locals.var_pparam_b4soileffcv_dn6)), (((assign31020_e29431 * locals.var_pparam_b4soiweffcv_dn7) * locals.var_pparam_b4soileffcv) + (assign31020_e29433 * locals.var_pparam_b4soileffcv_dn7)), (((assign31020_e29431 * locals.var_pparam_b4soiweffcv_dn8) * locals.var_pparam_b4soileffcv) + (assign31020_e29433 * locals.var_pparam_b4soileffcv_dn8)), (((assign31020_e29431 * locals.var_pparam_b4soiweffcv_dn9) * locals.var_pparam_b4soileffcv) + (assign31020_e29433 * locals.var_pparam_b4soileffcv_dn9)), (((assign31020_e29431 * locals.var_pparam_b4soiweffcv_dn10) * locals.var_pparam_b4soileffcv) + (assign31020_e29433 * locals.var_pparam_b4soileffcv_dn10)), (((assign31020_e29431 * locals.var_pparam_b4soiweffcv_dn11) * locals.var_pparam_b4soileffcv) + (assign31020_e29433 * locals.var_pparam_b4soileffcv_dn11)), (((assign31020_e29431 * locals.var_pparam_b4soiweffcv_dn12) * locals.var_pparam_b4soileffcv) + (assign31020_e29433 * locals.var_pparam_b4soileffcv_dn12)),)
    } else {
        (locals.var_c0, locals.var_c0_dn3, locals.var_c0_dn4, locals.var_c0_dn5, locals.var_c0_dn6, locals.var_c0_dn7, locals.var_c0_dn8, locals.var_c0_dn9, locals.var_c0_dn10, locals.var_c0_dn11, locals.var_c0_dn12,)
    }
};
        locals.var_c0 = assign31020_e29437;
        locals.var_c0_dn3 = assign31020_e29437_d_n3;
        locals.var_c0_dn4 = assign31020_e29437_d_n4;
        locals.var_c0_dn5 = assign31020_e29437_d_n5;
        locals.var_c0_dn6 = assign31020_e29437_d_n6;
        locals.var_c0_dn7 = assign31020_e29437_d_n7;
        locals.var_c0_dn8 = assign31020_e29437_d_n8;
        locals.var_c0_dn9 = assign31020_e29437_d_n9;
        locals.var_c0_dn10 = assign31020_e29437_d_n10;
        locals.var_c0_dn11 = assign31020_e29437_d_n11;
        locals.var_c0_dn12 = assign31020_e29437_d_n12;
        locals.var_c0_rv = 0.0;

        let assign31570_e29941: f64 = if locals.var_b4soimode > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1513 = assign31570_e29941;
        locals.var_guard1513_rv = 0.0;

        let (assign31670_e30026, assign31670_e30026_d_n3, assign31670_e30026_d_n4, assign31670_e30026_d_n5, assign31670_e30026_d_n6, assign31670_e30026_d_n7, assign31670_e30026_d_n8, assign31670_e30026_d_n9, assign31670_e30026_d_n10, assign31670_e30026_d_n11, assign31670_e30026_d_n12,) = {
    if (locals.var_guard1513 != 0.0) {
        let assign31670_e30024: f64 = (p.p37 * locals.var_qdrn);
        (assign31670_e30024, (p.p37 * locals.var_qdrn_dn3), (p.p37 * locals.var_qdrn_dn4), (p.p37 * locals.var_qdrn_dn5), (p.p37 * locals.var_qdrn_dn6), (p.p37 * locals.var_qdrn_dn7), (p.p37 * locals.var_qdrn_dn8), (p.p37 * locals.var_qdrn_dn9), (p.p37 * locals.var_qdrn_dn10), (p.p37 * locals.var_qdrn_dn11), (p.p37 * locals.var_qdrn_dn12),)
    } else {
        (locals.var_b4soiqdrn, locals.var_b4soiqdrn_dn3, locals.var_b4soiqdrn_dn4, locals.var_b4soiqdrn_dn5, locals.var_b4soiqdrn_dn6, locals.var_b4soiqdrn_dn7, locals.var_b4soiqdrn_dn8, locals.var_b4soiqdrn_dn9, locals.var_b4soiqdrn_dn10, locals.var_b4soiqdrn_dn11, locals.var_b4soiqdrn_dn12,)
    }
};
        locals.var_b4soiqdrn = assign31670_e30026;
        locals.var_b4soiqdrn_dn3 = assign31670_e30026_d_n3;
        locals.var_b4soiqdrn_dn4 = assign31670_e30026_d_n4;
        locals.var_b4soiqdrn_dn5 = assign31670_e30026_d_n5;
        locals.var_b4soiqdrn_dn6 = assign31670_e30026_d_n6;
        locals.var_b4soiqdrn_dn7 = assign31670_e30026_d_n7;
        locals.var_b4soiqdrn_dn8 = assign31670_e30026_d_n8;
        locals.var_b4soiqdrn_dn9 = assign31670_e30026_d_n9;
        locals.var_b4soiqdrn_dn10 = assign31670_e30026_d_n10;
        locals.var_b4soiqdrn_dn11 = assign31670_e30026_d_n11;
        locals.var_b4soiqdrn_dn12 = assign31670_e30026_d_n12;
        locals.var_b4soiqdrn_rv = 0.0;

        let (assign31680_e30032, assign31680_e30032_d_n3, assign31680_e30032_d_n4, assign31680_e30032_d_n5, assign31680_e30032_d_n6, assign31680_e30032_d_n7, assign31680_e30032_d_n8, assign31680_e30032_d_n9, assign31680_e30032_d_n10, assign31680_e30032_d_n11, assign31680_e30032_d_n12,) = {
    if (locals.var_guard1513 != 0.0) {
        let assign31680_e30030: f64 = (p.p37 * locals.var_qsrc);
        (assign31680_e30030, (p.p37 * locals.var_qsrc_dn3), (p.p37 * locals.var_qsrc_dn4), (p.p37 * locals.var_qsrc_dn5), (p.p37 * locals.var_qsrc_dn6), (p.p37 * locals.var_qsrc_dn7), (p.p37 * locals.var_qsrc_dn8), (p.p37 * locals.var_qsrc_dn9), (p.p37 * locals.var_qsrc_dn10), (p.p37 * locals.var_qsrc_dn11), (p.p37 * locals.var_qsrc_dn12),)
    } else {
        (locals.var_b4soiqsrc, locals.var_b4soiqsrc_dn3, locals.var_b4soiqsrc_dn4, locals.var_b4soiqsrc_dn5, locals.var_b4soiqsrc_dn6, locals.var_b4soiqsrc_dn7, locals.var_b4soiqsrc_dn8, locals.var_b4soiqsrc_dn9, locals.var_b4soiqsrc_dn10, locals.var_b4soiqsrc_dn11, locals.var_b4soiqsrc_dn12,)
    }
};
        locals.var_b4soiqsrc = assign31680_e30032;
        locals.var_b4soiqsrc_dn3 = assign31680_e30032_d_n3;
        locals.var_b4soiqsrc_dn4 = assign31680_e30032_d_n4;
        locals.var_b4soiqsrc_dn5 = assign31680_e30032_d_n5;
        locals.var_b4soiqsrc_dn6 = assign31680_e30032_d_n6;
        locals.var_b4soiqsrc_dn7 = assign31680_e30032_d_n7;
        locals.var_b4soiqsrc_dn8 = assign31680_e30032_d_n8;
        locals.var_b4soiqsrc_dn9 = assign31680_e30032_d_n9;
        locals.var_b4soiqsrc_dn10 = assign31680_e30032_d_n10;
        locals.var_b4soiqsrc_dn11 = assign31680_e30032_d_n11;
        locals.var_b4soiqsrc_dn12 = assign31680_e30032_d_n12;
        locals.var_b4soiqsrc_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_95(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign31780_e30126, assign31780_e30126_d_n3, assign31780_e30126_d_n4, assign31780_e30126_d_n5, assign31780_e30126_d_n6, assign31780_e30126_d_n7, assign31780_e30126_d_n8, assign31780_e30126_d_n9, assign31780_e30126_d_n10, assign31780_e30126_d_n11, assign31780_e30126_d_n12,) = {
    if (locals.var_guard1513 == 0.0) {
        let assign31780_e30124: f64 = (p.p37 * locals.var_qdrn);
        (assign31780_e30124, (p.p37 * locals.var_qdrn_dn3), (p.p37 * locals.var_qdrn_dn4), (p.p37 * locals.var_qdrn_dn5), (p.p37 * locals.var_qdrn_dn6), (p.p37 * locals.var_qdrn_dn7), (p.p37 * locals.var_qdrn_dn8), (p.p37 * locals.var_qdrn_dn9), (p.p37 * locals.var_qdrn_dn10), (p.p37 * locals.var_qdrn_dn11), (p.p37 * locals.var_qdrn_dn12),)
    } else {
        (locals.var_b4soiqsrc, locals.var_b4soiqsrc_dn3, locals.var_b4soiqsrc_dn4, locals.var_b4soiqsrc_dn5, locals.var_b4soiqsrc_dn6, locals.var_b4soiqsrc_dn7, locals.var_b4soiqsrc_dn8, locals.var_b4soiqsrc_dn9, locals.var_b4soiqsrc_dn10, locals.var_b4soiqsrc_dn11, locals.var_b4soiqsrc_dn12,)
    }
};
        locals.var_b4soiqsrc = assign31780_e30126;
        locals.var_b4soiqsrc_dn3 = assign31780_e30126_d_n3;
        locals.var_b4soiqsrc_dn4 = assign31780_e30126_d_n4;
        locals.var_b4soiqsrc_dn5 = assign31780_e30126_d_n5;
        locals.var_b4soiqsrc_dn6 = assign31780_e30126_d_n6;
        locals.var_b4soiqsrc_dn7 = assign31780_e30126_d_n7;
        locals.var_b4soiqsrc_dn8 = assign31780_e30126_d_n8;
        locals.var_b4soiqsrc_dn9 = assign31780_e30126_d_n9;
        locals.var_b4soiqsrc_dn10 = assign31780_e30126_d_n10;
        locals.var_b4soiqsrc_dn11 = assign31780_e30126_d_n11;
        locals.var_b4soiqsrc_dn12 = assign31780_e30126_d_n12;
        locals.var_b4soiqsrc_rv = 0.0;

        let (assign31790_e30133, assign31790_e30133_d_n3, assign31790_e30133_d_n4, assign31790_e30133_d_n5, assign31790_e30133_d_n6, assign31790_e30133_d_n7, assign31790_e30133_d_n8, assign31790_e30133_d_n9, assign31790_e30133_d_n10, assign31790_e30133_d_n11, assign31790_e30133_d_n12,) = {
    if (locals.var_guard1513 == 0.0) {
        let assign31790_e30131: f64 = (p.p37 * locals.var_qsrc);
        (assign31790_e30131, (p.p37 * locals.var_qsrc_dn3), (p.p37 * locals.var_qsrc_dn4), (p.p37 * locals.var_qsrc_dn5), (p.p37 * locals.var_qsrc_dn6), (p.p37 * locals.var_qsrc_dn7), (p.p37 * locals.var_qsrc_dn8), (p.p37 * locals.var_qsrc_dn9), (p.p37 * locals.var_qsrc_dn10), (p.p37 * locals.var_qsrc_dn11), (p.p37 * locals.var_qsrc_dn12),)
    } else {
        (locals.var_b4soiqdrn, locals.var_b4soiqdrn_dn3, locals.var_b4soiqdrn_dn4, locals.var_b4soiqdrn_dn5, locals.var_b4soiqdrn_dn6, locals.var_b4soiqdrn_dn7, locals.var_b4soiqdrn_dn8, locals.var_b4soiqdrn_dn9, locals.var_b4soiqdrn_dn10, locals.var_b4soiqdrn_dn11, locals.var_b4soiqdrn_dn12,)
    }
};
        locals.var_b4soiqdrn = assign31790_e30133;
        locals.var_b4soiqdrn_dn3 = assign31790_e30133_d_n3;
        locals.var_b4soiqdrn_dn4 = assign31790_e30133_d_n4;
        locals.var_b4soiqdrn_dn5 = assign31790_e30133_d_n5;
        locals.var_b4soiqdrn_dn6 = assign31790_e30133_d_n6;
        locals.var_b4soiqdrn_dn7 = assign31790_e30133_d_n7;
        locals.var_b4soiqdrn_dn8 = assign31790_e30133_d_n8;
        locals.var_b4soiqdrn_dn9 = assign31790_e30133_d_n9;
        locals.var_b4soiqdrn_dn10 = assign31790_e30133_d_n10;
        locals.var_b4soiqdrn_dn11 = assign31790_e30133_d_n11;
        locals.var_b4soiqdrn_dn12 = assign31790_e30133_d_n12;
        locals.var_b4soiqdrn_rv = 0.0;

        let assign31860_e30176: f64 = if p.p39 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1518 = assign31860_e30176;
        locals.var_guard1518_rv = 0.0;

        let assign31920_e30214: f64 = if ((p.p36 == 1.0) && (p.p14 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1524 = assign31920_e30214;
        locals.var_guard1524_rv = 0.0;

        let assign31930_e30219: f64 = if ((p.p35 != 0.0) && (!true)) { 1.0 } else { 0.0 };
        locals.var_guard1525 = assign31930_e30219;
        locals.var_guard1525_rv = 0.0;

        let assign31940_e30221: f64 = 1.0;
        locals.var_guard1526 = assign31940_e30221;
        locals.var_guard1526_rv = 0.0;

        let assign31950_e30223: f64 = 1.0;
        locals.var_guard1527 = assign31950_e30223;
        locals.var_guard1527_rv = 0.0;

        let assign31960_e30226: f64 = if p.p430 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1528 = assign31960_e30226;
        locals.var_guard1528_rv = 0.0;

        let assign31970_e30229: f64 = if p.p430 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1529 = assign31970_e30229;
        locals.var_guard1529_rv = 0.0;

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
        locals.var_qgi_1_rv = 0.0;

        locals.var_qdi = locals.var_qdrn;
        locals.var_qdi_dn3 = locals.var_qdrn_dn3;
        locals.var_qdi_dn4 = locals.var_qdrn_dn4;
        locals.var_qdi_dn5 = locals.var_qdrn_dn5;
        locals.var_qdi_dn6 = locals.var_qdrn_dn6;
        locals.var_qdi_dn7 = locals.var_qdrn_dn7;
        locals.var_qdi_dn8 = locals.var_qdrn_dn8;
        locals.var_qdi_dn9 = locals.var_qdrn_dn9;
        locals.var_qdi_dn10 = locals.var_qdrn_dn10;
        locals.var_qdi_dn11 = locals.var_qdrn_dn11;
        locals.var_qdi_dn12 = locals.var_qdrn_dn12;
        locals.var_qdi_rv = 0.0;

        locals.var_qsi_1 = locals.var_qsrc;
        locals.var_qsi_1_dn3 = locals.var_qsrc_dn3;
        locals.var_qsi_1_dn4 = locals.var_qsrc_dn4;
        locals.var_qsi_1_dn5 = locals.var_qsrc_dn5;
        locals.var_qsi_1_dn6 = locals.var_qsrc_dn6;
        locals.var_qsi_1_dn7 = locals.var_qsrc_dn7;
        locals.var_qsi_1_dn8 = locals.var_qsrc_dn8;
        locals.var_qsi_1_dn9 = locals.var_qsrc_dn9;
        locals.var_qsi_1_dn10 = locals.var_qsrc_dn10;
        locals.var_qsi_1_dn11 = locals.var_qsrc_dn11;
        locals.var_qsi_1_dn12 = locals.var_qsrc_dn12;
        locals.var_qsi_1_rv = 0.0;

        let assign32040_e30244: f64 = (locals.var_qgso + locals.var_qgdo);
        locals.var_qov_1 = assign32040_e30244;
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
        locals.var_qov_1_rv = 0.0;

        let assign32050_e30247: f64 = (locals.var_qdi - locals.var_qgdo);
        locals.var_qdrn = assign32050_e30247;
        locals.var_qdrn_dn3 = (locals.var_qdi_dn3 - locals.var_qgdo_dn3);
        locals.var_qdrn_dn4 = (locals.var_qdi_dn4 - locals.var_qgdo_dn4);
        locals.var_qdrn_dn5 = (locals.var_qdi_dn5 - locals.var_qgdo_dn5);
        locals.var_qdrn_dn6 = (locals.var_qdi_dn6 - locals.var_qgdo_dn6);
        locals.var_qdrn_dn7 = (locals.var_qdi_dn7 - locals.var_qgdo_dn7);
        locals.var_qdrn_dn8 = (locals.var_qdi_dn8 - locals.var_qgdo_dn8);
        locals.var_qdrn_dn9 = (locals.var_qdi_dn9 - locals.var_qgdo_dn9);
        locals.var_qdrn_dn10 = (locals.var_qdi_dn10 - locals.var_qgdo_dn10);
        locals.var_qdrn_dn11 = (locals.var_qdi_dn11 - locals.var_qgdo_dn11);
        locals.var_qdrn_dn12 = (locals.var_qdi_dn12 - locals.var_qgdo_dn12);
        locals.var_qdrn_rv = 0.0;

        let assign32060_e30250: f64 = (locals.var_qsi_1 - locals.var_qgso);
        locals.var_qsrc = assign32060_e30250;
        locals.var_qsrc_dn3 = (locals.var_qsi_1_dn3 - locals.var_qgso_dn3);
        locals.var_qsrc_dn4 = (locals.var_qsi_1_dn4 - locals.var_qgso_dn4);
        locals.var_qsrc_dn5 = (locals.var_qsi_1_dn5 - locals.var_qgso_dn5);
        locals.var_qsrc_dn6 = (locals.var_qsi_1_dn6 - locals.var_qgso_dn6);
        locals.var_qsrc_dn7 = (locals.var_qsi_1_dn7 - locals.var_qgso_dn7);
        locals.var_qsrc_dn8 = (locals.var_qsi_1_dn8 - locals.var_qgso_dn8);
        locals.var_qsrc_dn9 = (locals.var_qsi_1_dn9 - locals.var_qgso_dn9);
        locals.var_qsrc_dn10 = (locals.var_qsi_1_dn10 - locals.var_qgso_dn10);
        locals.var_qsrc_dn11 = (locals.var_qsi_1_dn11 - locals.var_qgso_dn11);
        locals.var_qsrc_dn12 = (locals.var_qsi_1_dn12 - locals.var_qgso_dn12);
        locals.var_qsrc_rv = 0.0;

        let assign32070_e30253: f64 = (locals.var_qgi_1 + locals.var_qov_1);
        locals.var_qgate = assign32070_e30253;
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
        locals.var_qgate_rv = 0.0;

    }
}

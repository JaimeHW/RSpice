#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_104(
        locals: &mut StampLocals,
    ) {
        let (assign32320_e36498, assign32320_e36498_d_n0, assign32320_e36498_d_n2, assign32320_e36498_d_n4, assign32320_e36498_d_n5, assign32320_e36498_d_n6, assign32320_e36498_d_n7, assign32320_e36498_d_n8, assign32320_e36498_d_n9, assign32320_e36498_d_n10, assign32320_e36498_d_n11, assign32320_e36498_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard769 != 0.0)) && (locals.var_guard770 != 0.0)) {
        let assign32320_e36492: f64 = (0.02 * locals.var_xmp);
        let assign32320_e36494: f64 = (assign32320_e36492 * locals.var_dnm);
        let assign32320_e36496: f64 = (assign32320_e36494 / locals.var_arg);
        (assign32320_e36496, ((((((0.02 * locals.var_xmp_dn0) * locals.var_dnm) + (assign32320_e36492 * locals.var_dnm_dn0)) * locals.var_arg) - (assign32320_e36494 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn2) * locals.var_dnm) + (assign32320_e36492 * locals.var_dnm_dn2)) * locals.var_arg) - (assign32320_e36494 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn4) * locals.var_dnm) + (assign32320_e36492 * locals.var_dnm_dn4)) * locals.var_arg) - (assign32320_e36494 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn5) * locals.var_dnm) + (assign32320_e36492 * locals.var_dnm_dn5)) * locals.var_arg) - (assign32320_e36494 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn6) * locals.var_dnm) + (assign32320_e36492 * locals.var_dnm_dn6)) * locals.var_arg) - (assign32320_e36494 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn7) * locals.var_dnm) + (assign32320_e36492 * locals.var_dnm_dn7)) * locals.var_arg) - (assign32320_e36494 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn8) * locals.var_dnm) + (assign32320_e36492 * locals.var_dnm_dn8)) * locals.var_arg) - (assign32320_e36494 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn9) * locals.var_dnm) + (assign32320_e36492 * locals.var_dnm_dn9)) * locals.var_arg) - (assign32320_e36494 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn10) * locals.var_dnm) + (assign32320_e36492 * locals.var_dnm_dn10)) * locals.var_arg) - (assign32320_e36494 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn11) * locals.var_dnm) + (assign32320_e36492 * locals.var_dnm_dn11)) * locals.var_arg) - (assign32320_e36494 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn14) * locals.var_dnm) + (assign32320_e36492 * locals.var_dnm_dn14)) * locals.var_arg) - (assign32320_e36494 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign32320_e36498;
        locals.var_t1_dn0 = assign32320_e36498_d_n0;
        locals.var_t1_dn2 = assign32320_e36498_d_n2;
        locals.var_t1_dn4 = assign32320_e36498_d_n4;
        locals.var_t1_dn5 = assign32320_e36498_d_n5;
        locals.var_t1_dn6 = assign32320_e36498_d_n6;
        locals.var_t1_dn7 = assign32320_e36498_d_n7;
        locals.var_t1_dn8 = assign32320_e36498_d_n8;
        locals.var_t1_dn9 = assign32320_e36498_d_n9;
        locals.var_t1_dn10 = assign32320_e36498_d_n10;
        locals.var_t1_dn11 = assign32320_e36498_d_n11;
        locals.var_t1_dn14 = assign32320_e36498_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign32330_e36515, assign32330_e36515_d_n0, assign32330_e36515_d_n2, assign32330_e36515_d_n4, assign32330_e36515_d_n5, assign32330_e36515_d_n6, assign32330_e36515_d_n7, assign32330_e36515_d_n8, assign32330_e36515_d_n9, assign32330_e36515_d_n10, assign32330_e36515_d_n11, assign32330_e36515_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard769 != 0.0)) && (locals.var_guard770 != 0.0)) {
        let assign32330_e36511: f64 = (locals.var_phi_sl_dep - 0.02);
        let assign32330_e36513: f64 = (assign32330_e36511 + locals.var_tmf0);
        (assign32330_e36513, (locals.var_phi_sl_dep_dn0 + locals.var_tmf0_dn0), (locals.var_phi_sl_dep_dn2 + locals.var_tmf0_dn2), (locals.var_phi_sl_dep_dn4 + locals.var_tmf0_dn4), (locals.var_phi_sl_dep_dn5 + locals.var_tmf0_dn5), (locals.var_phi_sl_dep_dn6 + locals.var_tmf0_dn6), (locals.var_phi_sl_dep_dn7 + locals.var_tmf0_dn7), (locals.var_phi_sl_dep_dn8 + locals.var_tmf0_dn8), (locals.var_phi_sl_dep_dn9 + locals.var_tmf0_dn9), (locals.var_phi_sl_dep_dn10 + locals.var_tmf0_dn10), (locals.var_phi_sl_dep_dn11 + locals.var_tmf0_dn11), (locals.var_phi_sl_dep_dn14 + locals.var_tmf0_dn14),)
    } else {
        (locals.var_phi_bl_dep, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    }
};
        locals.var_phi_bl_dep = assign32330_e36515;
        locals.var_phi_bl_dep_dn0 = assign32330_e36515_d_n0;
        locals.var_phi_bl_dep_dn2 = assign32330_e36515_d_n2;
        locals.var_phi_bl_dep_dn4 = assign32330_e36515_d_n4;
        locals.var_phi_bl_dep_dn5 = assign32330_e36515_d_n5;
        locals.var_phi_bl_dep_dn6 = assign32330_e36515_d_n6;
        locals.var_phi_bl_dep_dn7 = assign32330_e36515_d_n7;
        locals.var_phi_bl_dep_dn8 = assign32330_e36515_d_n8;
        locals.var_phi_bl_dep_dn9 = assign32330_e36515_d_n9;
        locals.var_phi_bl_dep_dn10 = assign32330_e36515_d_n10;
        locals.var_phi_bl_dep_dn11 = assign32330_e36515_d_n11;
        locals.var_phi_bl_dep_dn14 = assign32330_e36515_d_n14;
        locals.var_phi_bl_dep_rv = 0.0;

        let (assign32340_e36528, assign32340_e36528_d_n0, assign32340_e36528_d_n2, assign32340_e36528_d_n4, assign32340_e36528_d_n5, assign32340_e36528_d_n6, assign32340_e36528_d_n7, assign32340_e36528_d_n8, assign32340_e36528_d_n9, assign32340_e36528_d_n10, assign32340_e36528_d_n11, assign32340_e36528_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard769 != 0.0)) && (locals.var_guard770 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign32340_e36528;
        locals.var_t1_dn0 = assign32340_e36528_d_n0;
        locals.var_t1_dn2 = assign32340_e36528_d_n2;
        locals.var_t1_dn4 = assign32340_e36528_d_n4;
        locals.var_t1_dn5 = assign32340_e36528_d_n5;
        locals.var_t1_dn6 = assign32340_e36528_d_n6;
        locals.var_t1_dn7 = assign32340_e36528_d_n7;
        locals.var_t1_dn8 = assign32340_e36528_d_n8;
        locals.var_t1_dn9 = assign32340_e36528_d_n9;
        locals.var_t1_dn10 = assign32340_e36528_d_n10;
        locals.var_t1_dn11 = assign32340_e36528_d_n11;
        locals.var_t1_dn14 = assign32340_e36528_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign32350_e36542, assign32350_e36542_d_n0, assign32350_e36542_d_n2, assign32350_e36542_d_n4, assign32350_e36542_d_n5, assign32350_e36542_d_n6, assign32350_e36542_d_n7, assign32350_e36542_d_n8, assign32350_e36542_d_n9, assign32350_e36542_d_n10, assign32350_e36542_d_n11, assign32350_e36542_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard769 != 0.0)) && (locals.var_guard770 == 0.0)) {
        (locals.var_phi_bl_dep, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    } else {
        (locals.var_phi_bl_dep, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    }
};
        locals.var_phi_bl_dep = assign32350_e36542;
        locals.var_phi_bl_dep_dn0 = assign32350_e36542_d_n0;
        locals.var_phi_bl_dep_dn2 = assign32350_e36542_d_n2;
        locals.var_phi_bl_dep_dn4 = assign32350_e36542_d_n4;
        locals.var_phi_bl_dep_dn5 = assign32350_e36542_d_n5;
        locals.var_phi_bl_dep_dn6 = assign32350_e36542_d_n6;
        locals.var_phi_bl_dep_dn7 = assign32350_e36542_d_n7;
        locals.var_phi_bl_dep_dn8 = assign32350_e36542_d_n8;
        locals.var_phi_bl_dep_dn9 = assign32350_e36542_d_n9;
        locals.var_phi_bl_dep_dn10 = assign32350_e36542_d_n10;
        locals.var_phi_bl_dep_dn11 = assign32350_e36542_d_n11;
        locals.var_phi_bl_dep_dn14 = assign32350_e36542_d_n14;
        locals.var_phi_bl_dep_rv = 0.0;

        let (assign32360_e36556, assign32360_e36556_d_n0, assign32360_e36556_d_n2, assign32360_e36556_d_n4, assign32360_e36556_d_n5, assign32360_e36556_d_n6, assign32360_e36556_d_n7, assign32360_e36556_d_n8, assign32360_e36556_d_n9, assign32360_e36556_d_n10, assign32360_e36556_d_n11, assign32360_e36556_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard769 != 0.0)) && (locals.var_guard770 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign32360_e36556;
        locals.var_t1_dn0 = assign32360_e36556_d_n0;
        locals.var_t1_dn2 = assign32360_e36556_d_n2;
        locals.var_t1_dn4 = assign32360_e36556_d_n4;
        locals.var_t1_dn5 = assign32360_e36556_d_n5;
        locals.var_t1_dn6 = assign32360_e36556_d_n6;
        locals.var_t1_dn7 = assign32360_e36556_d_n7;
        locals.var_t1_dn8 = assign32360_e36556_d_n8;
        locals.var_t1_dn9 = assign32360_e36556_d_n9;
        locals.var_t1_dn10 = assign32360_e36556_d_n10;
        locals.var_t1_dn11 = assign32360_e36556_d_n11;
        locals.var_t1_dn14 = assign32360_e36556_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign32370_e36573, assign32370_e36573_d_n0, assign32370_e36573_d_n2, assign32370_e36573_d_n4, assign32370_e36573_d_n5, assign32370_e36573_d_n6, assign32370_e36573_d_n7, assign32370_e36573_d_n8, assign32370_e36573_d_n9, assign32370_e36573_d_n10, assign32370_e36573_d_n11, assign32370_e36573_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) {
        let assign32370_e36566: f64 = (locals.var_ndepmpnsub * locals.var_phi_bl_dep);
        let assign32370_e36568: f64 = (assign32370_e36566 + locals.var_vbscl__blk439);
        let assign32370_e36570: f64 = (assign32370_e36568 - locals.var_vbi_dep);
        let assign32370_e36571: f64 = (locals.var_ndepmpnsub_inv1 * assign32370_e36570);
        (assign32370_e36571, ((locals.var_ndepmpnsub_inv1_dn0 * assign32370_e36570) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn0 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn0)) + locals.var_vbscl__blk439_dn0) - locals.var_vbi_dep_dn0))), ((locals.var_ndepmpnsub_inv1_dn2 * assign32370_e36570) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn2 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn2)) + locals.var_vbscl__blk439_dn2) - locals.var_vbi_dep_dn2))), ((locals.var_ndepmpnsub_inv1_dn4 * assign32370_e36570) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn4 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn4)) + locals.var_vbscl__blk439_dn4) - locals.var_vbi_dep_dn4))), ((locals.var_ndepmpnsub_inv1_dn5 * assign32370_e36570) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn5 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn5)) + locals.var_vbscl__blk439_dn5) - locals.var_vbi_dep_dn5))), ((locals.var_ndepmpnsub_inv1_dn6 * assign32370_e36570) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn6 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn6)) + locals.var_vbscl__blk439_dn6) - locals.var_vbi_dep_dn6))), ((locals.var_ndepmpnsub_inv1_dn7 * assign32370_e36570) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn7 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn7)) + locals.var_vbscl__blk439_dn7) - locals.var_vbi_dep_dn7))), ((locals.var_ndepmpnsub_inv1_dn8 * assign32370_e36570) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn8 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn8)) + locals.var_vbscl__blk439_dn8) - locals.var_vbi_dep_dn8))), ((locals.var_ndepmpnsub_inv1_dn9 * assign32370_e36570) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn9 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn9)) + locals.var_vbscl__blk439_dn9) - locals.var_vbi_dep_dn9))), ((locals.var_ndepmpnsub_inv1_dn10 * assign32370_e36570) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn10 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn10)) + locals.var_vbscl__blk439_dn10) - locals.var_vbi_dep_dn10))), ((locals.var_ndepmpnsub_inv1_dn11 * assign32370_e36570) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn11 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn11)) + locals.var_vbscl__blk439_dn11) - locals.var_vbi_dep_dn11))), ((locals.var_ndepmpnsub_inv1_dn14 * assign32370_e36570) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn14 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn14)) + locals.var_vbscl__blk439_dn14) - locals.var_vbi_dep_dn14))),)
    } else {
        (locals.var_phi_jl_dep, locals.var_phi_jl_dep_dn0, locals.var_phi_jl_dep_dn2, locals.var_phi_jl_dep_dn4, locals.var_phi_jl_dep_dn5, locals.var_phi_jl_dep_dn6, locals.var_phi_jl_dep_dn7, locals.var_phi_jl_dep_dn8, locals.var_phi_jl_dep_dn9, locals.var_phi_jl_dep_dn10, locals.var_phi_jl_dep_dn11, locals.var_phi_jl_dep_dn14,)
    }
};
        locals.var_phi_jl_dep = assign32370_e36573;
        locals.var_phi_jl_dep_dn0 = assign32370_e36573_d_n0;
        locals.var_phi_jl_dep_dn2 = assign32370_e36573_d_n2;
        locals.var_phi_jl_dep_dn4 = assign32370_e36573_d_n4;
        locals.var_phi_jl_dep_dn5 = assign32370_e36573_d_n5;
        locals.var_phi_jl_dep_dn6 = assign32370_e36573_d_n6;
        locals.var_phi_jl_dep_dn7 = assign32370_e36573_d_n7;
        locals.var_phi_jl_dep_dn8 = assign32370_e36573_d_n8;
        locals.var_phi_jl_dep_dn9 = assign32370_e36573_d_n9;
        locals.var_phi_jl_dep_dn10 = assign32370_e36573_d_n10;
        locals.var_phi_jl_dep_dn11 = assign32370_e36573_d_n11;
        locals.var_phi_jl_dep_dn14 = assign32370_e36573_d_n14;
        locals.var_phi_jl_dep_rv = 0.0;

        let (assign32380_e36586, assign32380_e36586_d_n0, assign32380_e36586_d_n2, assign32380_e36586_d_n4, assign32380_e36586_d_n5, assign32380_e36586_d_n6, assign32380_e36586_d_n7, assign32380_e36586_d_n8, assign32380_e36586_d_n9, assign32380_e36586_d_n10, assign32380_e36586_d_n11, assign32380_e36586_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) {
        let assign32380_e36583: f64 = (locals.var_phi_sl_dep - locals.var_phi_bl_dep);
        let assign32380_e36584: f64 = (locals.var_beta * assign32380_e36583);
        (assign32380_e36584, ((locals.var_beta_dn0 * assign32380_e36583) + (locals.var_beta * (locals.var_phi_sl_dep_dn0 - locals.var_phi_bl_dep_dn0))), ((locals.var_beta_dn2 * assign32380_e36583) + (locals.var_beta * (locals.var_phi_sl_dep_dn2 - locals.var_phi_bl_dep_dn2))), ((locals.var_beta_dn4 * assign32380_e36583) + (locals.var_beta * (locals.var_phi_sl_dep_dn4 - locals.var_phi_bl_dep_dn4))), ((locals.var_beta_dn5 * assign32380_e36583) + (locals.var_beta * (locals.var_phi_sl_dep_dn5 - locals.var_phi_bl_dep_dn5))), ((locals.var_beta_dn6 * assign32380_e36583) + (locals.var_beta * (locals.var_phi_sl_dep_dn6 - locals.var_phi_bl_dep_dn6))), ((locals.var_beta_dn7 * assign32380_e36583) + (locals.var_beta * (locals.var_phi_sl_dep_dn7 - locals.var_phi_bl_dep_dn7))), ((locals.var_beta_dn8 * assign32380_e36583) + (locals.var_beta * (locals.var_phi_sl_dep_dn8 - locals.var_phi_bl_dep_dn8))), ((locals.var_beta_dn9 * assign32380_e36583) + (locals.var_beta * (locals.var_phi_sl_dep_dn9 - locals.var_phi_bl_dep_dn9))), ((locals.var_beta_dn10 * assign32380_e36583) + (locals.var_beta * (locals.var_phi_sl_dep_dn10 - locals.var_phi_bl_dep_dn10))), ((locals.var_beta_dn11 * assign32380_e36583) + (locals.var_beta * (locals.var_phi_sl_dep_dn11 - locals.var_phi_bl_dep_dn11))), ((locals.var_beta_dn14 * assign32380_e36583) + (locals.var_beta * (locals.var_phi_sl_dep_dn14 - locals.var_phi_bl_dep_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign32380_e36586;
        locals.var_t1_dn0 = assign32380_e36586_d_n0;
        locals.var_t1_dn2 = assign32380_e36586_d_n2;
        locals.var_t1_dn4 = assign32380_e36586_d_n4;
        locals.var_t1_dn5 = assign32380_e36586_d_n5;
        locals.var_t1_dn6 = assign32380_e36586_d_n6;
        locals.var_t1_dn7 = assign32380_e36586_d_n7;
        locals.var_t1_dn8 = assign32380_e36586_d_n8;
        locals.var_t1_dn9 = assign32380_e36586_d_n9;
        locals.var_t1_dn10 = assign32380_e36586_d_n10;
        locals.var_t1_dn11 = assign32380_e36586_d_n11;
        locals.var_t1_dn14 = assign32380_e36586_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign32390_e36596, assign32390_e36596_d_n0, assign32390_e36596_d_n2, assign32390_e36596_d_n4, assign32390_e36596_d_n5, assign32390_e36596_d_n6, assign32390_e36596_d_n7, assign32390_e36596_d_n8, assign32390_e36596_d_n9, assign32390_e36596_d_n10, assign32390_e36596_d_n11, assign32390_e36596_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) {
        let assign32390_e36594: f64 = (locals.var_t1).exp();
        (assign32390_e36594, (assign32390_e36594 * locals.var_t1_dn0), (assign32390_e36594 * locals.var_t1_dn2), (assign32390_e36594 * locals.var_t1_dn4), (assign32390_e36594 * locals.var_t1_dn5), (assign32390_e36594 * locals.var_t1_dn6), (assign32390_e36594 * locals.var_t1_dn7), (assign32390_e36594 * locals.var_t1_dn8), (assign32390_e36594 * locals.var_t1_dn9), (assign32390_e36594 * locals.var_t1_dn10), (assign32390_e36594 * locals.var_t1_dn11), (assign32390_e36594 * locals.var_t1_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign32390_e36596;
        locals.var_t2_dn0 = assign32390_e36596_d_n0;
        locals.var_t2_dn2 = assign32390_e36596_d_n2;
        locals.var_t2_dn4 = assign32390_e36596_d_n4;
        locals.var_t2_dn5 = assign32390_e36596_d_n5;
        locals.var_t2_dn6 = assign32390_e36596_d_n6;
        locals.var_t2_dn7 = assign32390_e36596_d_n7;
        locals.var_t2_dn8 = assign32390_e36596_d_n8;
        locals.var_t2_dn9 = assign32390_e36596_d_n9;
        locals.var_t2_dn10 = assign32390_e36596_d_n10;
        locals.var_t2_dn11 = assign32390_e36596_d_n11;
        locals.var_t2_dn14 = assign32390_e36596_d_n14;
        locals.var_t2_rv = 0.0;

        let assign32400_e36599: f64 = if locals.var_phi_sl_dep >= locals.var_phi_bl_dep { 1.0 } else { 0.0 };
        locals.var_guard776 = assign32400_e36599;
        locals.var_guard776_rv = 0.0;

        let (assign32410_e36620, assign32410_e36620_d_n0, assign32410_e36620_d_n2, assign32410_e36620_d_n4, assign32410_e36620_d_n5, assign32410_e36620_d_n6, assign32410_e36620_d_n7, assign32410_e36620_d_n8, assign32410_e36620_d_n9, assign32410_e36620_d_n10, assign32410_e36620_d_n11, assign32410_e36620_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) {
        let assign32410_e36609: f64 = (-locals.var_cnst0);
        let assign32410_e36612: f64 = (locals.var_t2 - 1.0);
        let assign32410_e36614: f64 = (assign32410_e36612 - locals.var_t1);
        let assign32410_e36616: f64 = (assign32410_e36614 + 1e-15);
        let assign32410_e36617: f64 = (assign32410_e36616).sqrt();
        let assign32410_e36618: f64 = (assign32410_e36609 * assign32410_e36617);
        (assign32410_e36618, (((-locals.var_cnst0_dn0) * assign32410_e36617) + (assign32410_e36609 * ((locals.var_t2_dn0 - locals.var_t1_dn0) / (2.0 * assign32410_e36617)))), (((-locals.var_cnst0_dn2) * assign32410_e36617) + (assign32410_e36609 * ((locals.var_t2_dn2 - locals.var_t1_dn2) / (2.0 * assign32410_e36617)))), (((-locals.var_cnst0_dn4) * assign32410_e36617) + (assign32410_e36609 * ((locals.var_t2_dn4 - locals.var_t1_dn4) / (2.0 * assign32410_e36617)))), (((-locals.var_cnst0_dn5) * assign32410_e36617) + (assign32410_e36609 * ((locals.var_t2_dn5 - locals.var_t1_dn5) / (2.0 * assign32410_e36617)))), (((-locals.var_cnst0_dn6) * assign32410_e36617) + (assign32410_e36609 * ((locals.var_t2_dn6 - locals.var_t1_dn6) / (2.0 * assign32410_e36617)))), (((-locals.var_cnst0_dn7) * assign32410_e36617) + (assign32410_e36609 * ((locals.var_t2_dn7 - locals.var_t1_dn7) / (2.0 * assign32410_e36617)))), (((-locals.var_cnst0_dn8) * assign32410_e36617) + (assign32410_e36609 * ((locals.var_t2_dn8 - locals.var_t1_dn8) / (2.0 * assign32410_e36617)))), (((-locals.var_cnst0_dn9) * assign32410_e36617) + (assign32410_e36609 * ((locals.var_t2_dn9 - locals.var_t1_dn9) / (2.0 * assign32410_e36617)))), (((-locals.var_cnst0_dn10) * assign32410_e36617) + (assign32410_e36609 * ((locals.var_t2_dn10 - locals.var_t1_dn10) / (2.0 * assign32410_e36617)))), (((-locals.var_cnst0_dn11) * assign32410_e36617) + (assign32410_e36609 * ((locals.var_t2_dn11 - locals.var_t1_dn11) / (2.0 * assign32410_e36617)))), (((-locals.var_cnst0_dn14) * assign32410_e36617) + (assign32410_e36609 * ((locals.var_t2_dn14 - locals.var_t1_dn14) / (2.0 * assign32410_e36617)))),)
    } else {
        (locals.var_q_sl, locals.var_q_sl_dn0, locals.var_q_sl_dn2, locals.var_q_sl_dn4, locals.var_q_sl_dn5, locals.var_q_sl_dn6, locals.var_q_sl_dn7, locals.var_q_sl_dn8, locals.var_q_sl_dn9, locals.var_q_sl_dn10, locals.var_q_sl_dn11, locals.var_q_sl_dn14,)
    }
};
        locals.var_q_sl = assign32410_e36620;
        locals.var_q_sl_dn0 = assign32410_e36620_d_n0;
        locals.var_q_sl_dn2 = assign32410_e36620_d_n2;
        locals.var_q_sl_dn4 = assign32410_e36620_d_n4;
        locals.var_q_sl_dn5 = assign32410_e36620_d_n5;
        locals.var_q_sl_dn6 = assign32410_e36620_d_n6;
        locals.var_q_sl_dn7 = assign32410_e36620_d_n7;
        locals.var_q_sl_dn8 = assign32410_e36620_d_n8;
        locals.var_q_sl_dn9 = assign32410_e36620_d_n9;
        locals.var_q_sl_dn10 = assign32410_e36620_d_n10;
        locals.var_q_sl_dn11 = assign32410_e36620_d_n11;
        locals.var_q_sl_dn14 = assign32410_e36620_d_n14;
        locals.var_q_sl_rv = 0.0;

        let (assign32420_e36631, assign32420_e36631_d_n0, assign32420_e36631_d_n2, assign32420_e36631_d_n4, assign32420_e36631_d_n5, assign32420_e36631_d_n6, assign32420_e36631_d_n7, assign32420_e36631_d_n8, assign32420_e36631_d_n9, assign32420_e36631_d_n10, assign32420_e36631_d_n11, assign32420_e36631_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) {
        (locals.var_q_sl, locals.var_q_sl_dn0, locals.var_q_sl_dn2, locals.var_q_sl_dn4, locals.var_q_sl_dn5, locals.var_q_sl_dn6, locals.var_q_sl_dn7, locals.var_q_sl_dn8, locals.var_q_sl_dn9, locals.var_q_sl_dn10, locals.var_q_sl_dn11, locals.var_q_sl_dn14,)
    } else {
        (locals.var_q_nl, locals.var_q_nl_dn0, locals.var_q_nl_dn2, locals.var_q_nl_dn4, locals.var_q_nl_dn5, locals.var_q_nl_dn6, locals.var_q_nl_dn7, locals.var_q_nl_dn8, locals.var_q_nl_dn9, locals.var_q_nl_dn10, locals.var_q_nl_dn11, locals.var_q_nl_dn14,)
    }
};
        locals.var_q_nl = assign32420_e36631;
        locals.var_q_nl_dn0 = assign32420_e36631_d_n0;
        locals.var_q_nl_dn2 = assign32420_e36631_d_n2;
        locals.var_q_nl_dn4 = assign32420_e36631_d_n4;
        locals.var_q_nl_dn5 = assign32420_e36631_d_n5;
        locals.var_q_nl_dn6 = assign32420_e36631_d_n6;
        locals.var_q_nl_dn7 = assign32420_e36631_d_n7;
        locals.var_q_nl_dn8 = assign32420_e36631_d_n8;
        locals.var_q_nl_dn9 = assign32420_e36631_d_n9;
        locals.var_q_nl_dn10 = assign32420_e36631_d_n10;
        locals.var_q_nl_dn11 = assign32420_e36631_d_n11;
        locals.var_q_nl_dn14 = assign32420_e36631_d_n14;
        locals.var_q_nl_rv = 0.0;

        let (assign32430_e36642, assign32430_e36642_d_n0, assign32430_e36642_d_n2, assign32430_e36642_d_n4, assign32430_e36642_d_n5, assign32430_e36642_d_n6, assign32430_e36642_d_n7, assign32430_e36642_d_n8, assign32430_e36642_d_n9, assign32430_e36642_d_n10, assign32430_e36642_d_n11, assign32430_e36642_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_sl_dep, locals.var_q_sl_dep_dn0, locals.var_q_sl_dep_dn2, locals.var_q_sl_dep_dn4, locals.var_q_sl_dep_dn5, locals.var_q_sl_dep_dn6, locals.var_q_sl_dep_dn7, locals.var_q_sl_dep_dn8, locals.var_q_sl_dep_dn9, locals.var_q_sl_dep_dn10, locals.var_q_sl_dep_dn11, locals.var_q_sl_dep_dn14,)
    }
};
        locals.var_q_sl_dep = assign32430_e36642;
        locals.var_q_sl_dep_dn0 = assign32430_e36642_d_n0;
        locals.var_q_sl_dep_dn2 = assign32430_e36642_d_n2;
        locals.var_q_sl_dep_dn4 = assign32430_e36642_d_n4;
        locals.var_q_sl_dep_dn5 = assign32430_e36642_d_n5;
        locals.var_q_sl_dep_dn6 = assign32430_e36642_d_n6;
        locals.var_q_sl_dep_dn7 = assign32430_e36642_d_n7;
        locals.var_q_sl_dep_dn8 = assign32430_e36642_d_n8;
        locals.var_q_sl_dep_dn9 = assign32430_e36642_d_n9;
        locals.var_q_sl_dep_dn10 = assign32430_e36642_d_n10;
        locals.var_q_sl_dep_dn11 = assign32430_e36642_d_n11;
        locals.var_q_sl_dep_dn14 = assign32430_e36642_d_n14;
        locals.var_q_sl_dep_rv = 0.0;

        let (assign32440_e36653, assign32440_e36653_d_n0, assign32440_e36653_d_n2, assign32440_e36653_d_n4, assign32440_e36653_d_n5, assign32440_e36653_d_n6, assign32440_e36653_d_n7, assign32440_e36653_d_n8, assign32440_e36653_d_n9, assign32440_e36653_d_n10, assign32440_e36653_d_n11, assign32440_e36653_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_subl, locals.var_q_subl_dn0, locals.var_q_subl_dn2, locals.var_q_subl_dn4, locals.var_q_subl_dn5, locals.var_q_subl_dn6, locals.var_q_subl_dn7, locals.var_q_subl_dn8, locals.var_q_subl_dn9, locals.var_q_subl_dn10, locals.var_q_subl_dn11, locals.var_q_subl_dn14,)
    }
};
        locals.var_q_subl = assign32440_e36653;
        locals.var_q_subl_dn0 = assign32440_e36653_d_n0;
        locals.var_q_subl_dn2 = assign32440_e36653_d_n2;
        locals.var_q_subl_dn4 = assign32440_e36653_d_n4;
        locals.var_q_subl_dn5 = assign32440_e36653_d_n5;
        locals.var_q_subl_dn6 = assign32440_e36653_d_n6;
        locals.var_q_subl_dn7 = assign32440_e36653_d_n7;
        locals.var_q_subl_dn8 = assign32440_e36653_d_n8;
        locals.var_q_subl_dn9 = assign32440_e36653_d_n9;
        locals.var_q_subl_dn10 = assign32440_e36653_d_n10;
        locals.var_q_subl_dn11 = assign32440_e36653_d_n11;
        locals.var_q_subl_dn14 = assign32440_e36653_d_n14;
        locals.var_q_subl_rv = 0.0;

        let (assign32450_e36669, assign32450_e36669_d_n0, assign32450_e36669_d_n2, assign32450_e36669_d_n4, assign32450_e36669_d_n5, assign32450_e36669_d_n6, assign32450_e36669_d_n7, assign32450_e36669_d_n8, assign32450_e36669_d_n9, assign32450_e36669_d_n10, assign32450_e36669_d_n11, assign32450_e36669_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) {
        let assign32450_e36665: f64 = (locals.var_phi_bl_dep - locals.var_phi_jl_dep);
        let assign32450_e36666: f64 = (locals.var_c_2esipq_ndepm * assign32450_e36665);
        let assign32450_e36667: f64 = (assign32450_e36666).sqrt();
        (assign32450_e36667, (((locals.var_c_2esipq_ndepm_dn0 * assign32450_e36665) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn0 - locals.var_phi_jl_dep_dn0))) / (2.0 * assign32450_e36667)), (((locals.var_c_2esipq_ndepm_dn2 * assign32450_e36665) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn2 - locals.var_phi_jl_dep_dn2))) / (2.0 * assign32450_e36667)), (((locals.var_c_2esipq_ndepm_dn4 * assign32450_e36665) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn4 - locals.var_phi_jl_dep_dn4))) / (2.0 * assign32450_e36667)), (((locals.var_c_2esipq_ndepm_dn5 * assign32450_e36665) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn5 - locals.var_phi_jl_dep_dn5))) / (2.0 * assign32450_e36667)), (((locals.var_c_2esipq_ndepm_dn6 * assign32450_e36665) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn6 - locals.var_phi_jl_dep_dn6))) / (2.0 * assign32450_e36667)), (((locals.var_c_2esipq_ndepm_dn7 * assign32450_e36665) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn7 - locals.var_phi_jl_dep_dn7))) / (2.0 * assign32450_e36667)), (((locals.var_c_2esipq_ndepm_dn8 * assign32450_e36665) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn8 - locals.var_phi_jl_dep_dn8))) / (2.0 * assign32450_e36667)), (((locals.var_c_2esipq_ndepm_dn9 * assign32450_e36665) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn9 - locals.var_phi_jl_dep_dn9))) / (2.0 * assign32450_e36667)), (((locals.var_c_2esipq_ndepm_dn10 * assign32450_e36665) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn10 - locals.var_phi_jl_dep_dn10))) / (2.0 * assign32450_e36667)), (((locals.var_c_2esipq_ndepm_dn11 * assign32450_e36665) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn11 - locals.var_phi_jl_dep_dn11))) / (2.0 * assign32450_e36667)), (((locals.var_c_2esipq_ndepm_dn14 * assign32450_e36665) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn14 - locals.var_phi_jl_dep_dn14))) / (2.0 * assign32450_e36667)),)
    } else {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn11, locals.var_w_bl_dn14,)
    }
};
        locals.var_w_bl = assign32450_e36669;
        locals.var_w_bl_dn0 = assign32450_e36669_d_n0;
        locals.var_w_bl_dn2 = assign32450_e36669_d_n2;
        locals.var_w_bl_dn4 = assign32450_e36669_d_n4;
        locals.var_w_bl_dn5 = assign32450_e36669_d_n5;
        locals.var_w_bl_dn6 = assign32450_e36669_d_n6;
        locals.var_w_bl_dn7 = assign32450_e36669_d_n7;
        locals.var_w_bl_dn8 = assign32450_e36669_d_n8;
        locals.var_w_bl_dn9 = assign32450_e36669_d_n9;
        locals.var_w_bl_dn10 = assign32450_e36669_d_n10;
        locals.var_w_bl_dn11 = assign32450_e36669_d_n11;
        locals.var_w_bl_dn14 = assign32450_e36669_d_n14;
        locals.var_w_bl_rv = 0.0;

        let assign32460_e36673: f64 = (locals.var_uc_depthn - 1e-8);
        let assign32460_e36678: f64 = if ((locals.var_w_bl > assign32460_e36673) && (1e-8 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard777 = assign32460_e36678;
        locals.var_guard777_rv = 0.0;

        let (assign32470_e36695, assign32470_e36695_d_n0, assign32470_e36695_d_n2, assign32470_e36695_d_n4, assign32470_e36695_d_n5, assign32470_e36695_d_n6, assign32470_e36695_d_n7, assign32470_e36695_d_n8, assign32470_e36695_d_n9, assign32470_e36695_d_n10, assign32470_e36695_d_n11, assign32470_e36695_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign32470_e36691: f64 = (locals.var_w_bl - locals.var_uc_depthn);
        let assign32470_e36693: f64 = (assign32470_e36691 + 1e-8);
        (assign32470_e36693, (locals.var_w_bl_dn0 - locals.var_uc_depthn_dn0), (locals.var_w_bl_dn2 - locals.var_uc_depthn_dn2), (locals.var_w_bl_dn4 - locals.var_uc_depthn_dn4), (locals.var_w_bl_dn5 - locals.var_uc_depthn_dn5), (locals.var_w_bl_dn6 - locals.var_uc_depthn_dn6), (locals.var_w_bl_dn7 - locals.var_uc_depthn_dn7), (locals.var_w_bl_dn8 - locals.var_uc_depthn_dn8), (locals.var_w_bl_dn9 - locals.var_uc_depthn_dn9), (locals.var_w_bl_dn10 - locals.var_uc_depthn_dn10), (locals.var_w_bl_dn11 - locals.var_uc_depthn_dn11), (locals.var_w_bl_dn14 - locals.var_uc_depthn_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign32470_e36695;
        locals.var_tmf1_dn0 = assign32470_e36695_d_n0;
        locals.var_tmf1_dn2 = assign32470_e36695_d_n2;
        locals.var_tmf1_dn4 = assign32470_e36695_d_n4;
        locals.var_tmf1_dn5 = assign32470_e36695_d_n5;
        locals.var_tmf1_dn6 = assign32470_e36695_d_n6;
        locals.var_tmf1_dn7 = assign32470_e36695_d_n7;
        locals.var_tmf1_dn8 = assign32470_e36695_d_n8;
        locals.var_tmf1_dn9 = assign32470_e36695_d_n9;
        locals.var_tmf1_dn10 = assign32470_e36695_d_n10;
        locals.var_tmf1_dn11 = assign32470_e36695_d_n11;
        locals.var_tmf1_dn14 = assign32470_e36695_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign32480_e36710, assign32480_e36710_d_n0, assign32480_e36710_d_n2, assign32480_e36710_d_n4, assign32480_e36710_d_n5, assign32480_e36710_d_n6, assign32480_e36710_d_n7, assign32480_e36710_d_n8, assign32480_e36710_d_n9, assign32480_e36710_d_n10, assign32480_e36710_d_n11, assign32480_e36710_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign32480_e36708: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign32480_e36708, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign32480_e36710;
        locals.var_x2_dn0 = assign32480_e36710_d_n0;
        locals.var_x2_dn2 = assign32480_e36710_d_n2;
        locals.var_x2_dn4 = assign32480_e36710_d_n4;
        locals.var_x2_dn5 = assign32480_e36710_d_n5;
        locals.var_x2_dn6 = assign32480_e36710_d_n6;
        locals.var_x2_dn7 = assign32480_e36710_d_n7;
        locals.var_x2_dn8 = assign32480_e36710_d_n8;
        locals.var_x2_dn9 = assign32480_e36710_d_n9;
        locals.var_x2_dn10 = assign32480_e36710_d_n10;
        locals.var_x2_dn11 = assign32480_e36710_d_n11;
        locals.var_x2_dn14 = assign32480_e36710_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign32490_e36725, assign32490_e36725_d_n0, assign32490_e36725_d_n2, assign32490_e36725_d_n4, assign32490_e36725_d_n5, assign32490_e36725_d_n6, assign32490_e36725_d_n7, assign32490_e36725_d_n8, assign32490_e36725_d_n9, assign32490_e36725_d_n10, assign32490_e36725_d_n11, assign32490_e36725_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign32490_e36723: f64 = (1e-8 * 1e-8);
        (assign32490_e36723, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign32490_e36725;
        locals.var_xmax2_dn0 = assign32490_e36725_d_n0;
        locals.var_xmax2_dn2 = assign32490_e36725_d_n2;
        locals.var_xmax2_dn4 = assign32490_e36725_d_n4;
        locals.var_xmax2_dn5 = assign32490_e36725_d_n5;
        locals.var_xmax2_dn6 = assign32490_e36725_d_n6;
        locals.var_xmax2_dn7 = assign32490_e36725_d_n7;
        locals.var_xmax2_dn8 = assign32490_e36725_d_n8;
        locals.var_xmax2_dn9 = assign32490_e36725_d_n9;
        locals.var_xmax2_dn10 = assign32490_e36725_d_n10;
        locals.var_xmax2_dn11 = assign32490_e36725_d_n11;
        locals.var_xmax2_dn14 = assign32490_e36725_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign32500_e36738, assign32500_e36738_d_n0, assign32500_e36738_d_n2, assign32500_e36738_d_n4, assign32500_e36738_d_n5, assign32500_e36738_d_n6, assign32500_e36738_d_n7, assign32500_e36738_d_n8, assign32500_e36738_d_n9, assign32500_e36738_d_n10, assign32500_e36738_d_n11, assign32500_e36738_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign32500_e36738;
        locals.var_xp_dn0 = assign32500_e36738_d_n0;
        locals.var_xp_dn2 = assign32500_e36738_d_n2;
        locals.var_xp_dn4 = assign32500_e36738_d_n4;
        locals.var_xp_dn5 = assign32500_e36738_d_n5;
        locals.var_xp_dn6 = assign32500_e36738_d_n6;
        locals.var_xp_dn7 = assign32500_e36738_d_n7;
        locals.var_xp_dn8 = assign32500_e36738_d_n8;
        locals.var_xp_dn9 = assign32500_e36738_d_n9;
        locals.var_xp_dn10 = assign32500_e36738_d_n10;
        locals.var_xp_dn11 = assign32500_e36738_d_n11;
        locals.var_xp_dn14 = assign32500_e36738_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign32510_e36751, assign32510_e36751_d_n0, assign32510_e36751_d_n2, assign32510_e36751_d_n4, assign32510_e36751_d_n5, assign32510_e36751_d_n6, assign32510_e36751_d_n7, assign32510_e36751_d_n8, assign32510_e36751_d_n9, assign32510_e36751_d_n10, assign32510_e36751_d_n11, assign32510_e36751_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign32510_e36751;
        locals.var_xmp_dn0 = assign32510_e36751_d_n0;
        locals.var_xmp_dn2 = assign32510_e36751_d_n2;
        locals.var_xmp_dn4 = assign32510_e36751_d_n4;
        locals.var_xmp_dn5 = assign32510_e36751_d_n5;
        locals.var_xmp_dn6 = assign32510_e36751_d_n6;
        locals.var_xmp_dn7 = assign32510_e36751_d_n7;
        locals.var_xmp_dn8 = assign32510_e36751_d_n8;
        locals.var_xmp_dn9 = assign32510_e36751_d_n9;
        locals.var_xmp_dn10 = assign32510_e36751_d_n10;
        locals.var_xmp_dn11 = assign32510_e36751_d_n11;
        locals.var_xmp_dn14 = assign32510_e36751_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign32520_e36764,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign32520_e36764;
        locals.var_m0_rv = 0.0;

        let (assign32530_e36777,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign32530_e36777;
        locals.var_mm_rv = 0.0;

        let (assign32540_e36790, assign32540_e36790_d_n0, assign32540_e36790_d_n2, assign32540_e36790_d_n4, assign32540_e36790_d_n5, assign32540_e36790_d_n6, assign32540_e36790_d_n7, assign32540_e36790_d_n8, assign32540_e36790_d_n9, assign32540_e36790_d_n10, assign32540_e36790_d_n11, assign32540_e36790_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign32540_e36790;
        locals.var_arg_dn0 = assign32540_e36790_d_n0;
        locals.var_arg_dn2 = assign32540_e36790_d_n2;
        locals.var_arg_dn4 = assign32540_e36790_d_n4;
        locals.var_arg_dn5 = assign32540_e36790_d_n5;
        locals.var_arg_dn6 = assign32540_e36790_d_n6;
        locals.var_arg_dn7 = assign32540_e36790_d_n7;
        locals.var_arg_dn8 = assign32540_e36790_d_n8;
        locals.var_arg_dn9 = assign32540_e36790_d_n9;
        locals.var_arg_dn10 = assign32540_e36790_d_n10;
        locals.var_arg_dn11 = assign32540_e36790_d_n11;
        locals.var_arg_dn14 = assign32540_e36790_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign32550_e36803, assign32550_e36803_d_n0, assign32550_e36803_d_n2, assign32550_e36803_d_n4, assign32550_e36803_d_n5, assign32550_e36803_d_n6, assign32550_e36803_d_n7, assign32550_e36803_d_n8, assign32550_e36803_d_n9, assign32550_e36803_d_n10, assign32550_e36803_d_n11, assign32550_e36803_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign32550_e36803;
        locals.var_dnm_dn0 = assign32550_e36803_d_n0;
        locals.var_dnm_dn2 = assign32550_e36803_d_n2;
        locals.var_dnm_dn4 = assign32550_e36803_d_n4;
        locals.var_dnm_dn5 = assign32550_e36803_d_n5;
        locals.var_dnm_dn6 = assign32550_e36803_d_n6;
        locals.var_dnm_dn7 = assign32550_e36803_d_n7;
        locals.var_dnm_dn8 = assign32550_e36803_d_n8;
        locals.var_dnm_dn9 = assign32550_e36803_d_n9;
        locals.var_dnm_dn10 = assign32550_e36803_d_n10;
        locals.var_dnm_dn11 = assign32550_e36803_d_n11;
        locals.var_dnm_dn14 = assign32550_e36803_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign32560_e36818, assign32560_e36818_d_n0, assign32560_e36818_d_n2, assign32560_e36818_d_n4, assign32560_e36818_d_n5, assign32560_e36818_d_n6, assign32560_e36818_d_n7, assign32560_e36818_d_n8, assign32560_e36818_d_n9, assign32560_e36818_d_n10, assign32560_e36818_d_n11, assign32560_e36818_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign32560_e36816: f64 = (locals.var_xp * locals.var_x2);
        (assign32560_e36816, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign32560_e36818;
        locals.var_xp_dn0 = assign32560_e36818_d_n0;
        locals.var_xp_dn2 = assign32560_e36818_d_n2;
        locals.var_xp_dn4 = assign32560_e36818_d_n4;
        locals.var_xp_dn5 = assign32560_e36818_d_n5;
        locals.var_xp_dn6 = assign32560_e36818_d_n6;
        locals.var_xp_dn7 = assign32560_e36818_d_n7;
        locals.var_xp_dn8 = assign32560_e36818_d_n8;
        locals.var_xp_dn9 = assign32560_e36818_d_n9;
        locals.var_xp_dn10 = assign32560_e36818_d_n10;
        locals.var_xp_dn11 = assign32560_e36818_d_n11;
        locals.var_xp_dn14 = assign32560_e36818_d_n14;
        locals.var_xp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_105(
        locals: &mut StampLocals,
    ) {
        let (assign32570_e36833, assign32570_e36833_d_n0, assign32570_e36833_d_n2, assign32570_e36833_d_n4, assign32570_e36833_d_n5, assign32570_e36833_d_n6, assign32570_e36833_d_n7, assign32570_e36833_d_n8, assign32570_e36833_d_n9, assign32570_e36833_d_n10, assign32570_e36833_d_n11, assign32570_e36833_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign32570_e36831: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign32570_e36831, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign32570_e36833;
        locals.var_xmp_dn0 = assign32570_e36833_d_n0;
        locals.var_xmp_dn2 = assign32570_e36833_d_n2;
        locals.var_xmp_dn4 = assign32570_e36833_d_n4;
        locals.var_xmp_dn5 = assign32570_e36833_d_n5;
        locals.var_xmp_dn6 = assign32570_e36833_d_n6;
        locals.var_xmp_dn7 = assign32570_e36833_d_n7;
        locals.var_xmp_dn8 = assign32570_e36833_d_n8;
        locals.var_xmp_dn9 = assign32570_e36833_d_n9;
        locals.var_xmp_dn10 = assign32570_e36833_d_n10;
        locals.var_xmp_dn11 = assign32570_e36833_d_n11;
        locals.var_xmp_dn14 = assign32570_e36833_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign32580_e36848, assign32580_e36848_d_n0, assign32580_e36848_d_n2, assign32580_e36848_d_n4, assign32580_e36848_d_n5, assign32580_e36848_d_n6, assign32580_e36848_d_n7, assign32580_e36848_d_n8, assign32580_e36848_d_n9, assign32580_e36848_d_n10, assign32580_e36848_d_n11, assign32580_e36848_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign32580_e36846: f64 = (locals.var_xp * locals.var_x2);
        (assign32580_e36846, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign32580_e36848;
        locals.var_xp_dn0 = assign32580_e36848_d_n0;
        locals.var_xp_dn2 = assign32580_e36848_d_n2;
        locals.var_xp_dn4 = assign32580_e36848_d_n4;
        locals.var_xp_dn5 = assign32580_e36848_d_n5;
        locals.var_xp_dn6 = assign32580_e36848_d_n6;
        locals.var_xp_dn7 = assign32580_e36848_d_n7;
        locals.var_xp_dn8 = assign32580_e36848_d_n8;
        locals.var_xp_dn9 = assign32580_e36848_d_n9;
        locals.var_xp_dn10 = assign32580_e36848_d_n10;
        locals.var_xp_dn11 = assign32580_e36848_d_n11;
        locals.var_xp_dn14 = assign32580_e36848_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign32590_e36863, assign32590_e36863_d_n0, assign32590_e36863_d_n2, assign32590_e36863_d_n4, assign32590_e36863_d_n5, assign32590_e36863_d_n6, assign32590_e36863_d_n7, assign32590_e36863_d_n8, assign32590_e36863_d_n9, assign32590_e36863_d_n10, assign32590_e36863_d_n11, assign32590_e36863_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign32590_e36861: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign32590_e36861, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign32590_e36863;
        locals.var_xmp_dn0 = assign32590_e36863_d_n0;
        locals.var_xmp_dn2 = assign32590_e36863_d_n2;
        locals.var_xmp_dn4 = assign32590_e36863_d_n4;
        locals.var_xmp_dn5 = assign32590_e36863_d_n5;
        locals.var_xmp_dn6 = assign32590_e36863_d_n6;
        locals.var_xmp_dn7 = assign32590_e36863_d_n7;
        locals.var_xmp_dn8 = assign32590_e36863_d_n8;
        locals.var_xmp_dn9 = assign32590_e36863_d_n9;
        locals.var_xmp_dn10 = assign32590_e36863_d_n10;
        locals.var_xmp_dn11 = assign32590_e36863_d_n11;
        locals.var_xmp_dn14 = assign32590_e36863_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign32600_e36878, assign32600_e36878_d_n0, assign32600_e36878_d_n2, assign32600_e36878_d_n4, assign32600_e36878_d_n5, assign32600_e36878_d_n6, assign32600_e36878_d_n7, assign32600_e36878_d_n8, assign32600_e36878_d_n9, assign32600_e36878_d_n10, assign32600_e36878_d_n11, assign32600_e36878_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign32600_e36876: f64 = (locals.var_xp + locals.var_xmp);
        (assign32600_e36876, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign32600_e36878;
        locals.var_arg_dn0 = assign32600_e36878_d_n0;
        locals.var_arg_dn2 = assign32600_e36878_d_n2;
        locals.var_arg_dn4 = assign32600_e36878_d_n4;
        locals.var_arg_dn5 = assign32600_e36878_d_n5;
        locals.var_arg_dn6 = assign32600_e36878_d_n6;
        locals.var_arg_dn7 = assign32600_e36878_d_n7;
        locals.var_arg_dn8 = assign32600_e36878_d_n8;
        locals.var_arg_dn9 = assign32600_e36878_d_n9;
        locals.var_arg_dn10 = assign32600_e36878_d_n10;
        locals.var_arg_dn11 = assign32600_e36878_d_n11;
        locals.var_arg_dn14 = assign32600_e36878_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign32610_e36891, assign32610_e36891_d_n0, assign32610_e36891_d_n2, assign32610_e36891_d_n4, assign32610_e36891_d_n5, assign32610_e36891_d_n6, assign32610_e36891_d_n7, assign32610_e36891_d_n8, assign32610_e36891_d_n9, assign32610_e36891_d_n10, assign32610_e36891_d_n11, assign32610_e36891_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign32610_e36891;
        locals.var_dnm_dn0 = assign32610_e36891_d_n0;
        locals.var_dnm_dn2 = assign32610_e36891_d_n2;
        locals.var_dnm_dn4 = assign32610_e36891_d_n4;
        locals.var_dnm_dn5 = assign32610_e36891_d_n5;
        locals.var_dnm_dn6 = assign32610_e36891_d_n6;
        locals.var_dnm_dn7 = assign32610_e36891_d_n7;
        locals.var_dnm_dn8 = assign32610_e36891_d_n8;
        locals.var_dnm_dn9 = assign32610_e36891_d_n9;
        locals.var_dnm_dn10 = assign32610_e36891_d_n10;
        locals.var_dnm_dn11 = assign32610_e36891_d_n11;
        locals.var_dnm_dn14 = assign32610_e36891_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign32620_e36906: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard778 = assign32620_e36906;
        locals.var_guard778_rv = 0.0;

        let assign32630_e36909: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard779 = assign32630_e36909;
        locals.var_guard779_rv = 0.0;

        let (assign32640_e36926,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) && (locals.var_guard778 != 0.0)) && (locals.var_guard779 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign32640_e36926;
        locals.var_mm_rv = 0.0;

        let assign32650_e36929: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard780 = assign32650_e36929;
        locals.var_guard780_rv = 0.0;

        let (assign32660_e36949,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) && (locals.var_guard778 != 0.0)) && (locals.var_guard779 == 0.0)) && (locals.var_guard780 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign32660_e36949;
        locals.var_mm_rv = 0.0;

        let assign32670_e36952: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard781 = assign32670_e36952;
        locals.var_guard781_rv = 0.0;

        let (assign32680_e36975,) = {
    if (((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) && (locals.var_guard778 != 0.0)) && (locals.var_guard779 == 0.0)) && (locals.var_guard780 == 0.0)) && (locals.var_guard781 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign32680_e36975;
        locals.var_mm_rv = 0.0;

        let assign32690_e36978: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard782 = assign32690_e36978;
        locals.var_guard782_rv = 0.0;

        let (assign32700_e37004,) = {
    if ((((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) && (locals.var_guard778 != 0.0)) && (locals.var_guard779 == 0.0)) && (locals.var_guard780 == 0.0)) && (locals.var_guard781 == 0.0)) && (locals.var_guard782 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign32700_e37004;
        locals.var_mm_rv = 0.0;

        let (assign32710_e37019,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) && (locals.var_guard778 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign32710_e37019;
        locals.var_m0_rv = 0.0;

        let mut assign32720_loop_guard: usize = 0;
        while {
            let assign32720_cond_e37035: f64 = if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) && (locals.var_guard778 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign32720_cond_e37035 != 0.0
        } {
            assign32720_loop_guard += 1;
            assert!(assign32720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign32720_body0_e37051, assign32720_body0_e37051_d_n0, assign32720_body0_e37051_d_n2, assign32720_body0_e37051_d_n4, assign32720_body0_e37051_d_n5, assign32720_body0_e37051_d_n6, assign32720_body0_e37051_d_n7, assign32720_body0_e37051_d_n8, assign32720_body0_e37051_d_n9, assign32720_body0_e37051_d_n10, assign32720_body0_e37051_d_n11, assign32720_body0_e37051_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) && (locals.var_guard778 != 0.0)) {
        let assign32720_body0_e37049: f64 = (locals.var_dnm).sqrt();
        (assign32720_body0_e37049, (locals.var_dnm_dn0 / (2.0 * assign32720_body0_e37049)), (locals.var_dnm_dn2 / (2.0 * assign32720_body0_e37049)), (locals.var_dnm_dn4 / (2.0 * assign32720_body0_e37049)), (locals.var_dnm_dn5 / (2.0 * assign32720_body0_e37049)), (locals.var_dnm_dn6 / (2.0 * assign32720_body0_e37049)), (locals.var_dnm_dn7 / (2.0 * assign32720_body0_e37049)), (locals.var_dnm_dn8 / (2.0 * assign32720_body0_e37049)), (locals.var_dnm_dn9 / (2.0 * assign32720_body0_e37049)), (locals.var_dnm_dn10 / (2.0 * assign32720_body0_e37049)), (locals.var_dnm_dn11 / (2.0 * assign32720_body0_e37049)), (locals.var_dnm_dn14 / (2.0 * assign32720_body0_e37049)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign32720_body0_e37051;
            locals.var_dnm_dn0 = assign32720_body0_e37051_d_n0;
            locals.var_dnm_dn2 = assign32720_body0_e37051_d_n2;
            locals.var_dnm_dn4 = assign32720_body0_e37051_d_n4;
            locals.var_dnm_dn5 = assign32720_body0_e37051_d_n5;
            locals.var_dnm_dn6 = assign32720_body0_e37051_d_n6;
            locals.var_dnm_dn7 = assign32720_body0_e37051_d_n7;
            locals.var_dnm_dn8 = assign32720_body0_e37051_d_n8;
            locals.var_dnm_dn9 = assign32720_body0_e37051_d_n9;
            locals.var_dnm_dn10 = assign32720_body0_e37051_d_n10;
            locals.var_dnm_dn11 = assign32720_body0_e37051_d_n11;
            locals.var_dnm_dn14 = assign32720_body0_e37051_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign32720_body1_e37068,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) && (locals.var_guard778 != 0.0)) {
        let assign32720_body1_e37066: f64 = (locals.var_m0 + 1.0);
        (assign32720_body1_e37066,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign32720_body1_e37068;
            locals.var_m0_rv = 0.0;
        }

        let (assign32730_e37095, assign32730_e37095_d_n0, assign32730_e37095_d_n2, assign32730_e37095_d_n4, assign32730_e37095_d_n5, assign32730_e37095_d_n6, assign32730_e37095_d_n7, assign32730_e37095_d_n8, assign32730_e37095_d_n9, assign32730_e37095_d_n10, assign32730_e37095_d_n11, assign32730_e37095_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) && (locals.var_guard778 == 0.0)) {
        let (assign32730_e37093, assign32730_e37093_d_n0, assign32730_e37093_d_n2, assign32730_e37093_d_n4, assign32730_e37093_d_n5, assign32730_e37093_d_n6, assign32730_e37093_d_n7, assign32730_e37093_d_n8, assign32730_e37093_d_n9, assign32730_e37093_d_n10, assign32730_e37093_d_n11, assign32730_e37093_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign32730_e37090: f64 = (2.0 * 2.0);
                let assign32730_e37091: f64 = (1.0 / assign32730_e37090);
                let assign32730_e37092: f64 = (locals.var_dnm).powf(assign32730_e37091);
                (assign32730_e37092, if 0.0 == 0.0 && ((assign32730_e37091) as f64).is_finite() && ((assign32730_e37091) as f64).fract() == 0.0 { if assign32730_e37091 == 0.0 { 0.0 } else { (assign32730_e37091 * ((locals.var_dnm).powf(assign32730_e37091 - 1.0) * locals.var_dnm_dn0)) } } else { (assign32730_e37092 * (assign32730_e37091 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign32730_e37091) as f64).is_finite() && ((assign32730_e37091) as f64).fract() == 0.0 { if assign32730_e37091 == 0.0 { 0.0 } else { (assign32730_e37091 * ((locals.var_dnm).powf(assign32730_e37091 - 1.0) * locals.var_dnm_dn2)) } } else { (assign32730_e37092 * (assign32730_e37091 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign32730_e37091) as f64).is_finite() && ((assign32730_e37091) as f64).fract() == 0.0 { if assign32730_e37091 == 0.0 { 0.0 } else { (assign32730_e37091 * ((locals.var_dnm).powf(assign32730_e37091 - 1.0) * locals.var_dnm_dn4)) } } else { (assign32730_e37092 * (assign32730_e37091 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign32730_e37091) as f64).is_finite() && ((assign32730_e37091) as f64).fract() == 0.0 { if assign32730_e37091 == 0.0 { 0.0 } else { (assign32730_e37091 * ((locals.var_dnm).powf(assign32730_e37091 - 1.0) * locals.var_dnm_dn5)) } } else { (assign32730_e37092 * (assign32730_e37091 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign32730_e37091) as f64).is_finite() && ((assign32730_e37091) as f64).fract() == 0.0 { if assign32730_e37091 == 0.0 { 0.0 } else { (assign32730_e37091 * ((locals.var_dnm).powf(assign32730_e37091 - 1.0) * locals.var_dnm_dn6)) } } else { (assign32730_e37092 * (assign32730_e37091 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign32730_e37091) as f64).is_finite() && ((assign32730_e37091) as f64).fract() == 0.0 { if assign32730_e37091 == 0.0 { 0.0 } else { (assign32730_e37091 * ((locals.var_dnm).powf(assign32730_e37091 - 1.0) * locals.var_dnm_dn7)) } } else { (assign32730_e37092 * (assign32730_e37091 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign32730_e37091) as f64).is_finite() && ((assign32730_e37091) as f64).fract() == 0.0 { if assign32730_e37091 == 0.0 { 0.0 } else { (assign32730_e37091 * ((locals.var_dnm).powf(assign32730_e37091 - 1.0) * locals.var_dnm_dn8)) } } else { (assign32730_e37092 * (assign32730_e37091 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign32730_e37091) as f64).is_finite() && ((assign32730_e37091) as f64).fract() == 0.0 { if assign32730_e37091 == 0.0 { 0.0 } else { (assign32730_e37091 * ((locals.var_dnm).powf(assign32730_e37091 - 1.0) * locals.var_dnm_dn9)) } } else { (assign32730_e37092 * (assign32730_e37091 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign32730_e37091) as f64).is_finite() && ((assign32730_e37091) as f64).fract() == 0.0 { if assign32730_e37091 == 0.0 { 0.0 } else { (assign32730_e37091 * ((locals.var_dnm).powf(assign32730_e37091 - 1.0) * locals.var_dnm_dn10)) } } else { (assign32730_e37092 * (assign32730_e37091 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign32730_e37091) as f64).is_finite() && ((assign32730_e37091) as f64).fract() == 0.0 { if assign32730_e37091 == 0.0 { 0.0 } else { (assign32730_e37091 * ((locals.var_dnm).powf(assign32730_e37091 - 1.0) * locals.var_dnm_dn11)) } } else { (assign32730_e37092 * (assign32730_e37091 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign32730_e37091) as f64).is_finite() && ((assign32730_e37091) as f64).fract() == 0.0 { if assign32730_e37091 == 0.0 { 0.0 } else { (assign32730_e37091 * ((locals.var_dnm).powf(assign32730_e37091 - 1.0) * locals.var_dnm_dn14)) } } else { (assign32730_e37092 * (assign32730_e37091 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign32730_e37093, assign32730_e37093_d_n0, assign32730_e37093_d_n2, assign32730_e37093_d_n4, assign32730_e37093_d_n5, assign32730_e37093_d_n6, assign32730_e37093_d_n7, assign32730_e37093_d_n8, assign32730_e37093_d_n9, assign32730_e37093_d_n10, assign32730_e37093_d_n11, assign32730_e37093_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign32730_e37095;
        locals.var_dnm_dn0 = assign32730_e37095_d_n0;
        locals.var_dnm_dn2 = assign32730_e37095_d_n2;
        locals.var_dnm_dn4 = assign32730_e37095_d_n4;
        locals.var_dnm_dn5 = assign32730_e37095_d_n5;
        locals.var_dnm_dn6 = assign32730_e37095_d_n6;
        locals.var_dnm_dn7 = assign32730_e37095_d_n7;
        locals.var_dnm_dn8 = assign32730_e37095_d_n8;
        locals.var_dnm_dn9 = assign32730_e37095_d_n9;
        locals.var_dnm_dn10 = assign32730_e37095_d_n10;
        locals.var_dnm_dn11 = assign32730_e37095_d_n11;
        locals.var_dnm_dn14 = assign32730_e37095_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign32740_e37110, assign32740_e37110_d_n0, assign32740_e37110_d_n2, assign32740_e37110_d_n4, assign32740_e37110_d_n5, assign32740_e37110_d_n6, assign32740_e37110_d_n7, assign32740_e37110_d_n8, assign32740_e37110_d_n9, assign32740_e37110_d_n10, assign32740_e37110_d_n11, assign32740_e37110_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign32740_e37108: f64 = (1.0 / locals.var_dnm);
        (assign32740_e37108, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign32740_e37110;
        locals.var_dnm_dn0 = assign32740_e37110_d_n0;
        locals.var_dnm_dn2 = assign32740_e37110_d_n2;
        locals.var_dnm_dn4 = assign32740_e37110_d_n4;
        locals.var_dnm_dn5 = assign32740_e37110_d_n5;
        locals.var_dnm_dn6 = assign32740_e37110_d_n6;
        locals.var_dnm_dn7 = assign32740_e37110_d_n7;
        locals.var_dnm_dn8 = assign32740_e37110_d_n8;
        locals.var_dnm_dn9 = assign32740_e37110_d_n9;
        locals.var_dnm_dn10 = assign32740_e37110_d_n10;
        locals.var_dnm_dn11 = assign32740_e37110_d_n11;
        locals.var_dnm_dn14 = assign32740_e37110_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign32750_e37127, assign32750_e37127_d_n0, assign32750_e37127_d_n2, assign32750_e37127_d_n4, assign32750_e37127_d_n5, assign32750_e37127_d_n6, assign32750_e37127_d_n7, assign32750_e37127_d_n8, assign32750_e37127_d_n9, assign32750_e37127_d_n10, assign32750_e37127_d_n11, assign32750_e37127_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign32750_e37123: f64 = (locals.var_tmf1 * 1e-8);
        let assign32750_e37125: f64 = (assign32750_e37123 * locals.var_dnm);
        (assign32750_e37125, (((locals.var_tmf1_dn0 * 1e-8) * locals.var_dnm) + (assign32750_e37123 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-8) * locals.var_dnm) + (assign32750_e37123 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-8) * locals.var_dnm) + (assign32750_e37123 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-8) * locals.var_dnm) + (assign32750_e37123 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-8) * locals.var_dnm) + (assign32750_e37123 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-8) * locals.var_dnm) + (assign32750_e37123 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-8) * locals.var_dnm) + (assign32750_e37123 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-8) * locals.var_dnm) + (assign32750_e37123 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-8) * locals.var_dnm) + (assign32750_e37123 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1e-8) * locals.var_dnm) + (assign32750_e37123 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 1e-8) * locals.var_dnm) + (assign32750_e37123 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign32750_e37127;
        locals.var_tmf0_dn0 = assign32750_e37127_d_n0;
        locals.var_tmf0_dn2 = assign32750_e37127_d_n2;
        locals.var_tmf0_dn4 = assign32750_e37127_d_n4;
        locals.var_tmf0_dn5 = assign32750_e37127_d_n5;
        locals.var_tmf0_dn6 = assign32750_e37127_d_n6;
        locals.var_tmf0_dn7 = assign32750_e37127_d_n7;
        locals.var_tmf0_dn8 = assign32750_e37127_d_n8;
        locals.var_tmf0_dn9 = assign32750_e37127_d_n9;
        locals.var_tmf0_dn10 = assign32750_e37127_d_n10;
        locals.var_tmf0_dn11 = assign32750_e37127_d_n11;
        locals.var_tmf0_dn14 = assign32750_e37127_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign32760_e37146, assign32760_e37146_d_n0, assign32760_e37146_d_n2, assign32760_e37146_d_n4, assign32760_e37146_d_n5, assign32760_e37146_d_n6, assign32760_e37146_d_n7, assign32760_e37146_d_n8, assign32760_e37146_d_n9, assign32760_e37146_d_n10, assign32760_e37146_d_n11, assign32760_e37146_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign32760_e37140: f64 = (1e-8 * locals.var_xmp);
        let assign32760_e37142: f64 = (assign32760_e37140 * locals.var_dnm);
        let assign32760_e37144: f64 = (assign32760_e37142 / locals.var_arg);
        (assign32760_e37144, ((((((1e-8 * locals.var_xmp_dn0) * locals.var_dnm) + (assign32760_e37140 * locals.var_dnm_dn0)) * locals.var_arg) - (assign32760_e37142 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn2) * locals.var_dnm) + (assign32760_e37140 * locals.var_dnm_dn2)) * locals.var_arg) - (assign32760_e37142 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn4) * locals.var_dnm) + (assign32760_e37140 * locals.var_dnm_dn4)) * locals.var_arg) - (assign32760_e37142 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn5) * locals.var_dnm) + (assign32760_e37140 * locals.var_dnm_dn5)) * locals.var_arg) - (assign32760_e37142 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn6) * locals.var_dnm) + (assign32760_e37140 * locals.var_dnm_dn6)) * locals.var_arg) - (assign32760_e37142 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn7) * locals.var_dnm) + (assign32760_e37140 * locals.var_dnm_dn7)) * locals.var_arg) - (assign32760_e37142 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn8) * locals.var_dnm) + (assign32760_e37140 * locals.var_dnm_dn8)) * locals.var_arg) - (assign32760_e37142 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn9) * locals.var_dnm) + (assign32760_e37140 * locals.var_dnm_dn9)) * locals.var_arg) - (assign32760_e37142 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn10) * locals.var_dnm) + (assign32760_e37140 * locals.var_dnm_dn10)) * locals.var_arg) - (assign32760_e37142 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn11) * locals.var_dnm) + (assign32760_e37140 * locals.var_dnm_dn11)) * locals.var_arg) - (assign32760_e37142 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn14) * locals.var_dnm) + (assign32760_e37140 * locals.var_dnm_dn14)) * locals.var_arg) - (assign32760_e37142 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32760_e37146;
        locals.var_t3_dn0 = assign32760_e37146_d_n0;
        locals.var_t3_dn2 = assign32760_e37146_d_n2;
        locals.var_t3_dn4 = assign32760_e37146_d_n4;
        locals.var_t3_dn5 = assign32760_e37146_d_n5;
        locals.var_t3_dn6 = assign32760_e37146_d_n6;
        locals.var_t3_dn7 = assign32760_e37146_d_n7;
        locals.var_t3_dn8 = assign32760_e37146_d_n8;
        locals.var_t3_dn9 = assign32760_e37146_d_n9;
        locals.var_t3_dn10 = assign32760_e37146_d_n10;
        locals.var_t3_dn11 = assign32760_e37146_d_n11;
        locals.var_t3_dn14 = assign32760_e37146_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign32770_e37163, assign32770_e37163_d_n0, assign32770_e37163_d_n2, assign32770_e37163_d_n4, assign32770_e37163_d_n5, assign32770_e37163_d_n6, assign32770_e37163_d_n7, assign32770_e37163_d_n8, assign32770_e37163_d_n9, assign32770_e37163_d_n10, assign32770_e37163_d_n11, assign32770_e37163_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign32770_e37159: f64 = (locals.var_uc_depthn - 1e-8);
        let assign32770_e37161: f64 = (assign32770_e37159 + locals.var_tmf0);
        (assign32770_e37161, (locals.var_uc_depthn_dn0 + locals.var_tmf0_dn0), (locals.var_uc_depthn_dn2 + locals.var_tmf0_dn2), (locals.var_uc_depthn_dn4 + locals.var_tmf0_dn4), (locals.var_uc_depthn_dn5 + locals.var_tmf0_dn5), (locals.var_uc_depthn_dn6 + locals.var_tmf0_dn6), (locals.var_uc_depthn_dn7 + locals.var_tmf0_dn7), (locals.var_uc_depthn_dn8 + locals.var_tmf0_dn8), (locals.var_uc_depthn_dn9 + locals.var_tmf0_dn9), (locals.var_uc_depthn_dn10 + locals.var_tmf0_dn10), (locals.var_uc_depthn_dn11 + locals.var_tmf0_dn11), (locals.var_uc_depthn_dn14 + locals.var_tmf0_dn14),)
    } else {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn11, locals.var_w_bl_dn14,)
    }
};
        locals.var_w_bl = assign32770_e37163;
        locals.var_w_bl_dn0 = assign32770_e37163_d_n0;
        locals.var_w_bl_dn2 = assign32770_e37163_d_n2;
        locals.var_w_bl_dn4 = assign32770_e37163_d_n4;
        locals.var_w_bl_dn5 = assign32770_e37163_d_n5;
        locals.var_w_bl_dn6 = assign32770_e37163_d_n6;
        locals.var_w_bl_dn7 = assign32770_e37163_d_n7;
        locals.var_w_bl_dn8 = assign32770_e37163_d_n8;
        locals.var_w_bl_dn9 = assign32770_e37163_d_n9;
        locals.var_w_bl_dn10 = assign32770_e37163_d_n10;
        locals.var_w_bl_dn11 = assign32770_e37163_d_n11;
        locals.var_w_bl_dn14 = assign32770_e37163_d_n14;
        locals.var_w_bl_rv = 0.0;

        let (assign32780_e37176, assign32780_e37176_d_n0, assign32780_e37176_d_n2, assign32780_e37176_d_n4, assign32780_e37176_d_n5, assign32780_e37176_d_n6, assign32780_e37176_d_n7, assign32780_e37176_d_n8, assign32780_e37176_d_n9, assign32780_e37176_d_n10, assign32780_e37176_d_n11, assign32780_e37176_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32780_e37176;
        locals.var_t3_dn0 = assign32780_e37176_d_n0;
        locals.var_t3_dn2 = assign32780_e37176_d_n2;
        locals.var_t3_dn4 = assign32780_e37176_d_n4;
        locals.var_t3_dn5 = assign32780_e37176_d_n5;
        locals.var_t3_dn6 = assign32780_e37176_d_n6;
        locals.var_t3_dn7 = assign32780_e37176_d_n7;
        locals.var_t3_dn8 = assign32780_e37176_d_n8;
        locals.var_t3_dn9 = assign32780_e37176_d_n9;
        locals.var_t3_dn10 = assign32780_e37176_d_n10;
        locals.var_t3_dn11 = assign32780_e37176_d_n11;
        locals.var_t3_dn14 = assign32780_e37176_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign32790_e37190, assign32790_e37190_d_n0, assign32790_e37190_d_n2, assign32790_e37190_d_n4, assign32790_e37190_d_n5, assign32790_e37190_d_n6, assign32790_e37190_d_n7, assign32790_e37190_d_n8, assign32790_e37190_d_n9, assign32790_e37190_d_n10, assign32790_e37190_d_n11, assign32790_e37190_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 == 0.0)) {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn11, locals.var_w_bl_dn14,)
    } else {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn11, locals.var_w_bl_dn14,)
    }
};
        locals.var_w_bl = assign32790_e37190;
        locals.var_w_bl_dn0 = assign32790_e37190_d_n0;
        locals.var_w_bl_dn2 = assign32790_e37190_d_n2;
        locals.var_w_bl_dn4 = assign32790_e37190_d_n4;
        locals.var_w_bl_dn5 = assign32790_e37190_d_n5;
        locals.var_w_bl_dn6 = assign32790_e37190_d_n6;
        locals.var_w_bl_dn7 = assign32790_e37190_d_n7;
        locals.var_w_bl_dn8 = assign32790_e37190_d_n8;
        locals.var_w_bl_dn9 = assign32790_e37190_d_n9;
        locals.var_w_bl_dn10 = assign32790_e37190_d_n10;
        locals.var_w_bl_dn11 = assign32790_e37190_d_n11;
        locals.var_w_bl_dn14 = assign32790_e37190_d_n14;
        locals.var_w_bl_rv = 0.0;

        let (assign32800_e37204, assign32800_e37204_d_n0, assign32800_e37204_d_n2, assign32800_e37204_d_n4, assign32800_e37204_d_n5, assign32800_e37204_d_n6, assign32800_e37204_d_n7, assign32800_e37204_d_n8, assign32800_e37204_d_n9, assign32800_e37204_d_n10, assign32800_e37204_d_n11, assign32800_e37204_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32800_e37204;
        locals.var_t3_dn0 = assign32800_e37204_d_n0;
        locals.var_t3_dn2 = assign32800_e37204_d_n2;
        locals.var_t3_dn4 = assign32800_e37204_d_n4;
        locals.var_t3_dn5 = assign32800_e37204_d_n5;
        locals.var_t3_dn6 = assign32800_e37204_d_n6;
        locals.var_t3_dn7 = assign32800_e37204_d_n7;
        locals.var_t3_dn8 = assign32800_e37204_d_n8;
        locals.var_t3_dn9 = assign32800_e37204_d_n9;
        locals.var_t3_dn10 = assign32800_e37204_d_n10;
        locals.var_t3_dn11 = assign32800_e37204_d_n11;
        locals.var_t3_dn14 = assign32800_e37204_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign32810_e37222, assign32810_e37222_d_n0, assign32810_e37222_d_n2, assign32810_e37222_d_n4, assign32810_e37222_d_n5, assign32810_e37222_d_n6, assign32810_e37222_d_n7, assign32810_e37222_d_n8, assign32810_e37222_d_n9, assign32810_e37222_d_n10, assign32810_e37222_d_n11, assign32810_e37222_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) {
        let assign32810_e37216: f64 = (locals.var_phi_jl_dep - locals.var_vbscl__blk439);
        let assign32810_e37218: f64 = (assign32810_e37216 + locals.var_vbi_dep);
        let assign32810_e37219: f64 = (locals.var_c_2esipq_nsub * assign32810_e37218);
        let assign32810_e37220: f64 = (assign32810_e37219).sqrt();
        (assign32810_e37220, (((locals.var_c_2esipq_nsub_dn0 * assign32810_e37218) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn0 - locals.var_vbscl__blk439_dn0) + locals.var_vbi_dep_dn0))) / (2.0 * assign32810_e37220)), (((locals.var_c_2esipq_nsub_dn2 * assign32810_e37218) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn2 - locals.var_vbscl__blk439_dn2) + locals.var_vbi_dep_dn2))) / (2.0 * assign32810_e37220)), (((locals.var_c_2esipq_nsub_dn4 * assign32810_e37218) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn4 - locals.var_vbscl__blk439_dn4) + locals.var_vbi_dep_dn4))) / (2.0 * assign32810_e37220)), (((locals.var_c_2esipq_nsub_dn5 * assign32810_e37218) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn5 - locals.var_vbscl__blk439_dn5) + locals.var_vbi_dep_dn5))) / (2.0 * assign32810_e37220)), (((locals.var_c_2esipq_nsub_dn6 * assign32810_e37218) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn6 - locals.var_vbscl__blk439_dn6) + locals.var_vbi_dep_dn6))) / (2.0 * assign32810_e37220)), (((locals.var_c_2esipq_nsub_dn7 * assign32810_e37218) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn7 - locals.var_vbscl__blk439_dn7) + locals.var_vbi_dep_dn7))) / (2.0 * assign32810_e37220)), (((locals.var_c_2esipq_nsub_dn8 * assign32810_e37218) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn8 - locals.var_vbscl__blk439_dn8) + locals.var_vbi_dep_dn8))) / (2.0 * assign32810_e37220)), (((locals.var_c_2esipq_nsub_dn9 * assign32810_e37218) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn9 - locals.var_vbscl__blk439_dn9) + locals.var_vbi_dep_dn9))) / (2.0 * assign32810_e37220)), (((locals.var_c_2esipq_nsub_dn10 * assign32810_e37218) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn10 - locals.var_vbscl__blk439_dn10) + locals.var_vbi_dep_dn10))) / (2.0 * assign32810_e37220)), (((locals.var_c_2esipq_nsub_dn11 * assign32810_e37218) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn11 - locals.var_vbscl__blk439_dn11) + locals.var_vbi_dep_dn11))) / (2.0 * assign32810_e37220)), (((locals.var_c_2esipq_nsub_dn14 * assign32810_e37218) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn14 - locals.var_vbscl__blk439_dn14) + locals.var_vbi_dep_dn14))) / (2.0 * assign32810_e37220)),)
    } else {
        (locals.var_w_subl, locals.var_w_subl_dn0, locals.var_w_subl_dn2, locals.var_w_subl_dn4, locals.var_w_subl_dn5, locals.var_w_subl_dn6, locals.var_w_subl_dn7, locals.var_w_subl_dn8, locals.var_w_subl_dn9, locals.var_w_subl_dn10, locals.var_w_subl_dn11, locals.var_w_subl_dn14,)
    }
};
        locals.var_w_subl = assign32810_e37222;
        locals.var_w_subl_dn0 = assign32810_e37222_d_n0;
        locals.var_w_subl_dn2 = assign32810_e37222_d_n2;
        locals.var_w_subl_dn4 = assign32810_e37222_d_n4;
        locals.var_w_subl_dn5 = assign32810_e37222_d_n5;
        locals.var_w_subl_dn6 = assign32810_e37222_d_n6;
        locals.var_w_subl_dn7 = assign32810_e37222_d_n7;
        locals.var_w_subl_dn8 = assign32810_e37222_d_n8;
        locals.var_w_subl_dn9 = assign32810_e37222_d_n9;
        locals.var_w_subl_dn10 = assign32810_e37222_d_n10;
        locals.var_w_subl_dn11 = assign32810_e37222_d_n11;
        locals.var_w_subl_dn14 = assign32810_e37222_d_n14;
        locals.var_w_subl_rv = 0.0;

        let (assign32820_e37235, assign32820_e37235_d_n0, assign32820_e37235_d_n2, assign32820_e37235_d_n4, assign32820_e37235_d_n5, assign32820_e37235_d_n6, assign32820_e37235_d_n7, assign32820_e37235_d_n8, assign32820_e37235_d_n9, assign32820_e37235_d_n10, assign32820_e37235_d_n11, assign32820_e37235_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) {
        let assign32820_e37233: f64 = (locals.var_w_bl * locals.var_q_ndepm);
        (assign32820_e37233, ((locals.var_w_bl_dn0 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn0)), ((locals.var_w_bl_dn2 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn2)), ((locals.var_w_bl_dn4 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn4)), ((locals.var_w_bl_dn5 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn5)), ((locals.var_w_bl_dn6 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn6)), ((locals.var_w_bl_dn7 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn7)), ((locals.var_w_bl_dn8 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn8)), ((locals.var_w_bl_dn9 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn9)), ((locals.var_w_bl_dn10 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn10)), ((locals.var_w_bl_dn11 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn11)), ((locals.var_w_bl_dn14 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn14)),)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn4, locals.var_q_bl_dep_dn5, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn8, locals.var_q_bl_dep_dn9, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn14,)
    }
};
        locals.var_q_bl_dep = assign32820_e37235;
        locals.var_q_bl_dep_dn0 = assign32820_e37235_d_n0;
        locals.var_q_bl_dep_dn2 = assign32820_e37235_d_n2;
        locals.var_q_bl_dep_dn4 = assign32820_e37235_d_n4;
        locals.var_q_bl_dep_dn5 = assign32820_e37235_d_n5;
        locals.var_q_bl_dep_dn6 = assign32820_e37235_d_n6;
        locals.var_q_bl_dep_dn7 = assign32820_e37235_d_n7;
        locals.var_q_bl_dep_dn8 = assign32820_e37235_d_n8;
        locals.var_q_bl_dep_dn9 = assign32820_e37235_d_n9;
        locals.var_q_bl_dep_dn10 = assign32820_e37235_d_n10;
        locals.var_q_bl_dep_dn11 = assign32820_e37235_d_n11;
        locals.var_q_bl_dep_dn14 = assign32820_e37235_d_n14;
        locals.var_q_bl_dep_rv = 0.0;

        let (assign32830_e37249, assign32830_e37249_d_n0, assign32830_e37249_d_n2, assign32830_e37249_d_n4, assign32830_e37249_d_n5, assign32830_e37249_d_n6, assign32830_e37249_d_n7, assign32830_e37249_d_n8, assign32830_e37249_d_n9, assign32830_e37249_d_n10, assign32830_e37249_d_n11, assign32830_e37249_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 != 0.0)) {
        let assign32830_e37245: f64 = (-locals.var_w_subl);
        let assign32830_e37247: f64 = (assign32830_e37245 * locals.var_q_nsub__blk548);
        (assign32830_e37247, (((-locals.var_w_subl_dn0) * locals.var_q_nsub__blk548) + (assign32830_e37245 * locals.var_q_nsub__blk548_dn0)), (((-locals.var_w_subl_dn2) * locals.var_q_nsub__blk548) + (assign32830_e37245 * locals.var_q_nsub__blk548_dn2)), (((-locals.var_w_subl_dn4) * locals.var_q_nsub__blk548) + (assign32830_e37245 * locals.var_q_nsub__blk548_dn4)), (((-locals.var_w_subl_dn5) * locals.var_q_nsub__blk548) + (assign32830_e37245 * locals.var_q_nsub__blk548_dn5)), (((-locals.var_w_subl_dn6) * locals.var_q_nsub__blk548) + (assign32830_e37245 * locals.var_q_nsub__blk548_dn6)), (((-locals.var_w_subl_dn7) * locals.var_q_nsub__blk548) + (assign32830_e37245 * locals.var_q_nsub__blk548_dn7)), (((-locals.var_w_subl_dn8) * locals.var_q_nsub__blk548) + (assign32830_e37245 * locals.var_q_nsub__blk548_dn8)), (((-locals.var_w_subl_dn9) * locals.var_q_nsub__blk548) + (assign32830_e37245 * locals.var_q_nsub__blk548_dn9)), (((-locals.var_w_subl_dn10) * locals.var_q_nsub__blk548) + (assign32830_e37245 * locals.var_q_nsub__blk548_dn10)), (((-locals.var_w_subl_dn11) * locals.var_q_nsub__blk548) + (assign32830_e37245 * locals.var_q_nsub__blk548_dn11)), (((-locals.var_w_subl_dn14) * locals.var_q_nsub__blk548) + (assign32830_e37245 * locals.var_q_nsub__blk548_dn14)),)
    } else {
        (locals.var_q_subl_dep, locals.var_q_subl_dep_dn0, locals.var_q_subl_dep_dn2, locals.var_q_subl_dep_dn4, locals.var_q_subl_dep_dn5, locals.var_q_subl_dep_dn6, locals.var_q_subl_dep_dn7, locals.var_q_subl_dep_dn8, locals.var_q_subl_dep_dn9, locals.var_q_subl_dep_dn10, locals.var_q_subl_dep_dn11, locals.var_q_subl_dep_dn14,)
    }
};
        locals.var_q_subl_dep = assign32830_e37249;
        locals.var_q_subl_dep_dn0 = assign32830_e37249_d_n0;
        locals.var_q_subl_dep_dn2 = assign32830_e37249_d_n2;
        locals.var_q_subl_dep_dn4 = assign32830_e37249_d_n4;
        locals.var_q_subl_dep_dn5 = assign32830_e37249_d_n5;
        locals.var_q_subl_dep_dn6 = assign32830_e37249_d_n6;
        locals.var_q_subl_dep_dn7 = assign32830_e37249_d_n7;
        locals.var_q_subl_dep_dn8 = assign32830_e37249_d_n8;
        locals.var_q_subl_dep_dn9 = assign32830_e37249_d_n9;
        locals.var_q_subl_dep_dn10 = assign32830_e37249_d_n10;
        locals.var_q_subl_dep_dn11 = assign32830_e37249_d_n11;
        locals.var_q_subl_dep_dn14 = assign32830_e37249_d_n14;
        locals.var_q_subl_dep_rv = 0.0;

        let (assign32840_e37267, assign32840_e37267_d_n0, assign32840_e37267_d_n2, assign32840_e37267_d_n4, assign32840_e37267_d_n5, assign32840_e37267_d_n6, assign32840_e37267_d_n7, assign32840_e37267_d_n8, assign32840_e37267_d_n9, assign32840_e37267_d_n10, assign32840_e37267_d_n11, assign32840_e37267_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) {
        let assign32840_e37260: f64 = (-locals.var_beta);
        let assign32840_e37263: f64 = (locals.var_phi_sl_dep - locals.var_vbscl__blk439);
        let assign32840_e37264: f64 = (assign32840_e37260 * assign32840_e37263);
        let assign32840_e37265: f64 = (assign32840_e37264).exp();
        (assign32840_e37265, (assign32840_e37265 * (((-locals.var_beta_dn0) * assign32840_e37263) + (assign32840_e37260 * (locals.var_phi_sl_dep_dn0 - locals.var_vbscl__blk439_dn0)))), (assign32840_e37265 * (((-locals.var_beta_dn2) * assign32840_e37263) + (assign32840_e37260 * (locals.var_phi_sl_dep_dn2 - locals.var_vbscl__blk439_dn2)))), (assign32840_e37265 * (((-locals.var_beta_dn4) * assign32840_e37263) + (assign32840_e37260 * (locals.var_phi_sl_dep_dn4 - locals.var_vbscl__blk439_dn4)))), (assign32840_e37265 * (((-locals.var_beta_dn5) * assign32840_e37263) + (assign32840_e37260 * (locals.var_phi_sl_dep_dn5 - locals.var_vbscl__blk439_dn5)))), (assign32840_e37265 * (((-locals.var_beta_dn6) * assign32840_e37263) + (assign32840_e37260 * (locals.var_phi_sl_dep_dn6 - locals.var_vbscl__blk439_dn6)))), (assign32840_e37265 * (((-locals.var_beta_dn7) * assign32840_e37263) + (assign32840_e37260 * (locals.var_phi_sl_dep_dn7 - locals.var_vbscl__blk439_dn7)))), (assign32840_e37265 * (((-locals.var_beta_dn8) * assign32840_e37263) + (assign32840_e37260 * (locals.var_phi_sl_dep_dn8 - locals.var_vbscl__blk439_dn8)))), (assign32840_e37265 * (((-locals.var_beta_dn9) * assign32840_e37263) + (assign32840_e37260 * (locals.var_phi_sl_dep_dn9 - locals.var_vbscl__blk439_dn9)))), (assign32840_e37265 * (((-locals.var_beta_dn10) * assign32840_e37263) + (assign32840_e37260 * (locals.var_phi_sl_dep_dn10 - locals.var_vbscl__blk439_dn10)))), (assign32840_e37265 * (((-locals.var_beta_dn11) * assign32840_e37263) + (assign32840_e37260 * (locals.var_phi_sl_dep_dn11 - locals.var_vbscl__blk439_dn11)))), (assign32840_e37265 * (((-locals.var_beta_dn14) * assign32840_e37263) + (assign32840_e37260 * (locals.var_phi_sl_dep_dn14 - locals.var_vbscl__blk439_dn14)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32840_e37267;
        locals.var_t3_dn0 = assign32840_e37267_d_n0;
        locals.var_t3_dn2 = assign32840_e37267_d_n2;
        locals.var_t3_dn4 = assign32840_e37267_d_n4;
        locals.var_t3_dn5 = assign32840_e37267_d_n5;
        locals.var_t3_dn6 = assign32840_e37267_d_n6;
        locals.var_t3_dn7 = assign32840_e37267_d_n7;
        locals.var_t3_dn8 = assign32840_e37267_d_n8;
        locals.var_t3_dn9 = assign32840_e37267_d_n9;
        locals.var_t3_dn10 = assign32840_e37267_d_n10;
        locals.var_t3_dn11 = assign32840_e37267_d_n11;
        locals.var_t3_dn14 = assign32840_e37267_d_n14;
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_106(
        locals: &mut StampLocals,
    ) {
        let (assign32850_e37285, assign32850_e37285_d_n0, assign32850_e37285_d_n2, assign32850_e37285_d_n4, assign32850_e37285_d_n5, assign32850_e37285_d_n6, assign32850_e37285_d_n7, assign32850_e37285_d_n8, assign32850_e37285_d_n9, assign32850_e37285_d_n10, assign32850_e37285_d_n11, assign32850_e37285_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) {
        let assign32850_e37278: f64 = (-locals.var_beta);
        let assign32850_e37281: f64 = (locals.var_phi_bl_dep - locals.var_vbscl__blk439);
        let assign32850_e37282: f64 = (assign32850_e37278 * assign32850_e37281);
        let assign32850_e37283: f64 = (assign32850_e37282).exp();
        (assign32850_e37283, (assign32850_e37283 * (((-locals.var_beta_dn0) * assign32850_e37281) + (assign32850_e37278 * (locals.var_phi_bl_dep_dn0 - locals.var_vbscl__blk439_dn0)))), (assign32850_e37283 * (((-locals.var_beta_dn2) * assign32850_e37281) + (assign32850_e37278 * (locals.var_phi_bl_dep_dn2 - locals.var_vbscl__blk439_dn2)))), (assign32850_e37283 * (((-locals.var_beta_dn4) * assign32850_e37281) + (assign32850_e37278 * (locals.var_phi_bl_dep_dn4 - locals.var_vbscl__blk439_dn4)))), (assign32850_e37283 * (((-locals.var_beta_dn5) * assign32850_e37281) + (assign32850_e37278 * (locals.var_phi_bl_dep_dn5 - locals.var_vbscl__blk439_dn5)))), (assign32850_e37283 * (((-locals.var_beta_dn6) * assign32850_e37281) + (assign32850_e37278 * (locals.var_phi_bl_dep_dn6 - locals.var_vbscl__blk439_dn6)))), (assign32850_e37283 * (((-locals.var_beta_dn7) * assign32850_e37281) + (assign32850_e37278 * (locals.var_phi_bl_dep_dn7 - locals.var_vbscl__blk439_dn7)))), (assign32850_e37283 * (((-locals.var_beta_dn8) * assign32850_e37281) + (assign32850_e37278 * (locals.var_phi_bl_dep_dn8 - locals.var_vbscl__blk439_dn8)))), (assign32850_e37283 * (((-locals.var_beta_dn9) * assign32850_e37281) + (assign32850_e37278 * (locals.var_phi_bl_dep_dn9 - locals.var_vbscl__blk439_dn9)))), (assign32850_e37283 * (((-locals.var_beta_dn10) * assign32850_e37281) + (assign32850_e37278 * (locals.var_phi_bl_dep_dn10 - locals.var_vbscl__blk439_dn10)))), (assign32850_e37283 * (((-locals.var_beta_dn11) * assign32850_e37281) + (assign32850_e37278 * (locals.var_phi_bl_dep_dn11 - locals.var_vbscl__blk439_dn11)))), (assign32850_e37283 * (((-locals.var_beta_dn14) * assign32850_e37281) + (assign32850_e37278 * (locals.var_phi_bl_dep_dn14 - locals.var_vbscl__blk439_dn14)))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign32850_e37285;
        locals.var_t4_dn0 = assign32850_e37285_d_n0;
        locals.var_t4_dn2 = assign32850_e37285_d_n2;
        locals.var_t4_dn4 = assign32850_e37285_d_n4;
        locals.var_t4_dn5 = assign32850_e37285_d_n5;
        locals.var_t4_dn6 = assign32850_e37285_d_n6;
        locals.var_t4_dn7 = assign32850_e37285_d_n7;
        locals.var_t4_dn8 = assign32850_e37285_d_n8;
        locals.var_t4_dn9 = assign32850_e37285_d_n9;
        locals.var_t4_dn10 = assign32850_e37285_d_n10;
        locals.var_t4_dn11 = assign32850_e37285_d_n11;
        locals.var_t4_dn14 = assign32850_e37285_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign32860_e37312, assign32860_e37312_d_n0, assign32860_e37312_d_n2, assign32860_e37312_d_n4, assign32860_e37312_d_n5, assign32860_e37312_d_n6, assign32860_e37312_d_n7, assign32860_e37312_d_n8, assign32860_e37312_d_n9, assign32860_e37312_d_n10, assign32860_e37312_d_n11, assign32860_e37312_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) {
        let assign32860_e37298: f64 = (locals.var_t2 - 1.0);
        let assign32860_e37300: f64 = (assign32860_e37298 - locals.var_t1);
        let assign32860_e37304: f64 = (locals.var_t3 - locals.var_t4);
        let assign32860_e37305: f64 = (locals.var_cnst1 * assign32860_e37304);
        let assign32860_e37306: f64 = (assign32860_e37300 + assign32860_e37305);
        let assign32860_e37308: f64 = (assign32860_e37306 + 1e-15);
        let assign32860_e37309: f64 = (assign32860_e37308).sqrt();
        let assign32860_e37310: f64 = (locals.var_cnst0 * assign32860_e37309);
        (assign32860_e37310, ((locals.var_cnst0_dn0 * assign32860_e37309) + (locals.var_cnst0 * (((locals.var_t2_dn0 - locals.var_t1_dn0) + ((locals.var_cnst1_dn0 * assign32860_e37304) + (locals.var_cnst1 * (locals.var_t3_dn0 - locals.var_t4_dn0)))) / (2.0 * assign32860_e37309)))), ((locals.var_cnst0_dn2 * assign32860_e37309) + (locals.var_cnst0 * (((locals.var_t2_dn2 - locals.var_t1_dn2) + ((locals.var_cnst1_dn2 * assign32860_e37304) + (locals.var_cnst1 * (locals.var_t3_dn2 - locals.var_t4_dn2)))) / (2.0 * assign32860_e37309)))), ((locals.var_cnst0_dn4 * assign32860_e37309) + (locals.var_cnst0 * (((locals.var_t2_dn4 - locals.var_t1_dn4) + ((locals.var_cnst1_dn4 * assign32860_e37304) + (locals.var_cnst1 * (locals.var_t3_dn4 - locals.var_t4_dn4)))) / (2.0 * assign32860_e37309)))), ((locals.var_cnst0_dn5 * assign32860_e37309) + (locals.var_cnst0 * (((locals.var_t2_dn5 - locals.var_t1_dn5) + ((locals.var_cnst1_dn5 * assign32860_e37304) + (locals.var_cnst1 * (locals.var_t3_dn5 - locals.var_t4_dn5)))) / (2.0 * assign32860_e37309)))), ((locals.var_cnst0_dn6 * assign32860_e37309) + (locals.var_cnst0 * (((locals.var_t2_dn6 - locals.var_t1_dn6) + ((locals.var_cnst1_dn6 * assign32860_e37304) + (locals.var_cnst1 * (locals.var_t3_dn6 - locals.var_t4_dn6)))) / (2.0 * assign32860_e37309)))), ((locals.var_cnst0_dn7 * assign32860_e37309) + (locals.var_cnst0 * (((locals.var_t2_dn7 - locals.var_t1_dn7) + ((locals.var_cnst1_dn7 * assign32860_e37304) + (locals.var_cnst1 * (locals.var_t3_dn7 - locals.var_t4_dn7)))) / (2.0 * assign32860_e37309)))), ((locals.var_cnst0_dn8 * assign32860_e37309) + (locals.var_cnst0 * (((locals.var_t2_dn8 - locals.var_t1_dn8) + ((locals.var_cnst1_dn8 * assign32860_e37304) + (locals.var_cnst1 * (locals.var_t3_dn8 - locals.var_t4_dn8)))) / (2.0 * assign32860_e37309)))), ((locals.var_cnst0_dn9 * assign32860_e37309) + (locals.var_cnst0 * (((locals.var_t2_dn9 - locals.var_t1_dn9) + ((locals.var_cnst1_dn9 * assign32860_e37304) + (locals.var_cnst1 * (locals.var_t3_dn9 - locals.var_t4_dn9)))) / (2.0 * assign32860_e37309)))), ((locals.var_cnst0_dn10 * assign32860_e37309) + (locals.var_cnst0 * (((locals.var_t2_dn10 - locals.var_t1_dn10) + ((locals.var_cnst1_dn10 * assign32860_e37304) + (locals.var_cnst1 * (locals.var_t3_dn10 - locals.var_t4_dn10)))) / (2.0 * assign32860_e37309)))), ((locals.var_cnst0_dn11 * assign32860_e37309) + (locals.var_cnst0 * (((locals.var_t2_dn11 - locals.var_t1_dn11) + ((locals.var_cnst1_dn11 * assign32860_e37304) + (locals.var_cnst1 * (locals.var_t3_dn11 - locals.var_t4_dn11)))) / (2.0 * assign32860_e37309)))), ((locals.var_cnst0_dn14 * assign32860_e37309) + (locals.var_cnst0 * (((locals.var_t2_dn14 - locals.var_t1_dn14) + ((locals.var_cnst1_dn14 * assign32860_e37304) + (locals.var_cnst1 * (locals.var_t3_dn14 - locals.var_t4_dn14)))) / (2.0 * assign32860_e37309)))),)
    } else {
        (locals.var_q_sl, locals.var_q_sl_dn0, locals.var_q_sl_dn2, locals.var_q_sl_dn4, locals.var_q_sl_dn5, locals.var_q_sl_dn6, locals.var_q_sl_dn7, locals.var_q_sl_dn8, locals.var_q_sl_dn9, locals.var_q_sl_dn10, locals.var_q_sl_dn11, locals.var_q_sl_dn14,)
    }
};
        locals.var_q_sl = assign32860_e37312;
        locals.var_q_sl_dn0 = assign32860_e37312_d_n0;
        locals.var_q_sl_dn2 = assign32860_e37312_d_n2;
        locals.var_q_sl_dn4 = assign32860_e37312_d_n4;
        locals.var_q_sl_dn5 = assign32860_e37312_d_n5;
        locals.var_q_sl_dn6 = assign32860_e37312_d_n6;
        locals.var_q_sl_dn7 = assign32860_e37312_d_n7;
        locals.var_q_sl_dn8 = assign32860_e37312_d_n8;
        locals.var_q_sl_dn9 = assign32860_e37312_d_n9;
        locals.var_q_sl_dn10 = assign32860_e37312_d_n10;
        locals.var_q_sl_dn11 = assign32860_e37312_d_n11;
        locals.var_q_sl_dn14 = assign32860_e37312_d_n14;
        locals.var_q_sl_rv = 0.0;

        let assign32870_e37319: f64 = if ((locals.var_w_bsubl > locals.var_uc_depthn) && (locals.var_depmode != 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard783 = assign32870_e37319;
        locals.var_guard783_rv = 0.0;

        let (assign32880_e37333, assign32880_e37333_d_n0, assign32880_e37333_d_n2, assign32880_e37333_d_n4, assign32880_e37333_d_n5, assign32880_e37333_d_n6, assign32880_e37333_d_n7, assign32880_e37333_d_n8, assign32880_e37333_d_n9, assign32880_e37333_d_n10, assign32880_e37333_d_n11, assign32880_e37333_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard783 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_subl, locals.var_q_subl_dn0, locals.var_q_subl_dn2, locals.var_q_subl_dn4, locals.var_q_subl_dn5, locals.var_q_subl_dn6, locals.var_q_subl_dn7, locals.var_q_subl_dn8, locals.var_q_subl_dn9, locals.var_q_subl_dn10, locals.var_q_subl_dn11, locals.var_q_subl_dn14,)
    }
};
        locals.var_q_subl = assign32880_e37333;
        locals.var_q_subl_dn0 = assign32880_e37333_d_n0;
        locals.var_q_subl_dn2 = assign32880_e37333_d_n2;
        locals.var_q_subl_dn4 = assign32880_e37333_d_n4;
        locals.var_q_subl_dn5 = assign32880_e37333_d_n5;
        locals.var_q_subl_dn6 = assign32880_e37333_d_n6;
        locals.var_q_subl_dn7 = assign32880_e37333_d_n7;
        locals.var_q_subl_dn8 = assign32880_e37333_d_n8;
        locals.var_q_subl_dn9 = assign32880_e37333_d_n9;
        locals.var_q_subl_dn10 = assign32880_e37333_d_n10;
        locals.var_q_subl_dn11 = assign32880_e37333_d_n11;
        locals.var_q_subl_dn14 = assign32880_e37333_d_n14;
        locals.var_q_subl_rv = 0.0;

        let (assign32890_e37347, assign32890_e37347_d_n0, assign32890_e37347_d_n2, assign32890_e37347_d_n4, assign32890_e37347_d_n5, assign32890_e37347_d_n6, assign32890_e37347_d_n7, assign32890_e37347_d_n8, assign32890_e37347_d_n9, assign32890_e37347_d_n10, assign32890_e37347_d_n11, assign32890_e37347_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard783 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_sl_dep, locals.var_q_sl_dep_dn0, locals.var_q_sl_dep_dn2, locals.var_q_sl_dep_dn4, locals.var_q_sl_dep_dn5, locals.var_q_sl_dep_dn6, locals.var_q_sl_dep_dn7, locals.var_q_sl_dep_dn8, locals.var_q_sl_dep_dn9, locals.var_q_sl_dep_dn10, locals.var_q_sl_dep_dn11, locals.var_q_sl_dep_dn14,)
    }
};
        locals.var_q_sl_dep = assign32890_e37347;
        locals.var_q_sl_dep_dn0 = assign32890_e37347_d_n0;
        locals.var_q_sl_dep_dn2 = assign32890_e37347_d_n2;
        locals.var_q_sl_dep_dn4 = assign32890_e37347_d_n4;
        locals.var_q_sl_dep_dn5 = assign32890_e37347_d_n5;
        locals.var_q_sl_dep_dn6 = assign32890_e37347_d_n6;
        locals.var_q_sl_dep_dn7 = assign32890_e37347_d_n7;
        locals.var_q_sl_dep_dn8 = assign32890_e37347_d_n8;
        locals.var_q_sl_dep_dn9 = assign32890_e37347_d_n9;
        locals.var_q_sl_dep_dn10 = assign32890_e37347_d_n10;
        locals.var_q_sl_dep_dn11 = assign32890_e37347_d_n11;
        locals.var_q_sl_dep_dn14 = assign32890_e37347_d_n14;
        locals.var_q_sl_dep_rv = 0.0;

        let (assign32900_e37384, assign32900_e37384_d_n0, assign32900_e37384_d_n2, assign32900_e37384_d_n4, assign32900_e37384_d_n5, assign32900_e37384_d_n6, assign32900_e37384_d_n7, assign32900_e37384_d_n8, assign32900_e37384_d_n9, assign32900_e37384_d_n10, assign32900_e37384_d_n11, assign32900_e37384_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard783 == 0.0)) {
        let assign32900_e37362: f64 = (-locals.var_t1);
        let assign32900_e37365: f64 = (-locals.var_beta);
        let assign32900_e37368: f64 = (locals.var_phi_sl_dep - locals.var_vbscl__blk439);
        let assign32900_e37369: f64 = (assign32900_e37365 * assign32900_e37368);
        let assign32900_e37370: f64 = (assign32900_e37369).exp();
        let assign32900_e37372: f64 = (-locals.var_beta);
        let assign32900_e37375: f64 = (locals.var_phi_bl_dep - locals.var_vbscl__blk439);
        let assign32900_e37376: f64 = (assign32900_e37372 * assign32900_e37375);
        let assign32900_e37377: f64 = (assign32900_e37376).exp();
        let assign32900_e37378: f64 = (assign32900_e37370 - assign32900_e37377);
        let assign32900_e37379: f64 = (locals.var_cnst1 * assign32900_e37378);
        let assign32900_e37380: f64 = (assign32900_e37362 + assign32900_e37379);
        let assign32900_e37381: f64 = (assign32900_e37380).sqrt();
        let assign32900_e37382: f64 = (locals.var_cnst0 * assign32900_e37381);
        (assign32900_e37382, ((locals.var_cnst0_dn0 * assign32900_e37381) + (locals.var_cnst0 * (((-locals.var_t1_dn0) + ((locals.var_cnst1_dn0 * assign32900_e37378) + (locals.var_cnst1 * ((assign32900_e37370 * (((-locals.var_beta_dn0) * assign32900_e37368) + (assign32900_e37365 * (locals.var_phi_sl_dep_dn0 - locals.var_vbscl__blk439_dn0)))) - (assign32900_e37377 * (((-locals.var_beta_dn0) * assign32900_e37375) + (assign32900_e37372 * (locals.var_phi_bl_dep_dn0 - locals.var_vbscl__blk439_dn0)))))))) / (2.0 * assign32900_e37381)))), ((locals.var_cnst0_dn2 * assign32900_e37381) + (locals.var_cnst0 * (((-locals.var_t1_dn2) + ((locals.var_cnst1_dn2 * assign32900_e37378) + (locals.var_cnst1 * ((assign32900_e37370 * (((-locals.var_beta_dn2) * assign32900_e37368) + (assign32900_e37365 * (locals.var_phi_sl_dep_dn2 - locals.var_vbscl__blk439_dn2)))) - (assign32900_e37377 * (((-locals.var_beta_dn2) * assign32900_e37375) + (assign32900_e37372 * (locals.var_phi_bl_dep_dn2 - locals.var_vbscl__blk439_dn2)))))))) / (2.0 * assign32900_e37381)))), ((locals.var_cnst0_dn4 * assign32900_e37381) + (locals.var_cnst0 * (((-locals.var_t1_dn4) + ((locals.var_cnst1_dn4 * assign32900_e37378) + (locals.var_cnst1 * ((assign32900_e37370 * (((-locals.var_beta_dn4) * assign32900_e37368) + (assign32900_e37365 * (locals.var_phi_sl_dep_dn4 - locals.var_vbscl__blk439_dn4)))) - (assign32900_e37377 * (((-locals.var_beta_dn4) * assign32900_e37375) + (assign32900_e37372 * (locals.var_phi_bl_dep_dn4 - locals.var_vbscl__blk439_dn4)))))))) / (2.0 * assign32900_e37381)))), ((locals.var_cnst0_dn5 * assign32900_e37381) + (locals.var_cnst0 * (((-locals.var_t1_dn5) + ((locals.var_cnst1_dn5 * assign32900_e37378) + (locals.var_cnst1 * ((assign32900_e37370 * (((-locals.var_beta_dn5) * assign32900_e37368) + (assign32900_e37365 * (locals.var_phi_sl_dep_dn5 - locals.var_vbscl__blk439_dn5)))) - (assign32900_e37377 * (((-locals.var_beta_dn5) * assign32900_e37375) + (assign32900_e37372 * (locals.var_phi_bl_dep_dn5 - locals.var_vbscl__blk439_dn5)))))))) / (2.0 * assign32900_e37381)))), ((locals.var_cnst0_dn6 * assign32900_e37381) + (locals.var_cnst0 * (((-locals.var_t1_dn6) + ((locals.var_cnst1_dn6 * assign32900_e37378) + (locals.var_cnst1 * ((assign32900_e37370 * (((-locals.var_beta_dn6) * assign32900_e37368) + (assign32900_e37365 * (locals.var_phi_sl_dep_dn6 - locals.var_vbscl__blk439_dn6)))) - (assign32900_e37377 * (((-locals.var_beta_dn6) * assign32900_e37375) + (assign32900_e37372 * (locals.var_phi_bl_dep_dn6 - locals.var_vbscl__blk439_dn6)))))))) / (2.0 * assign32900_e37381)))), ((locals.var_cnst0_dn7 * assign32900_e37381) + (locals.var_cnst0 * (((-locals.var_t1_dn7) + ((locals.var_cnst1_dn7 * assign32900_e37378) + (locals.var_cnst1 * ((assign32900_e37370 * (((-locals.var_beta_dn7) * assign32900_e37368) + (assign32900_e37365 * (locals.var_phi_sl_dep_dn7 - locals.var_vbscl__blk439_dn7)))) - (assign32900_e37377 * (((-locals.var_beta_dn7) * assign32900_e37375) + (assign32900_e37372 * (locals.var_phi_bl_dep_dn7 - locals.var_vbscl__blk439_dn7)))))))) / (2.0 * assign32900_e37381)))), ((locals.var_cnst0_dn8 * assign32900_e37381) + (locals.var_cnst0 * (((-locals.var_t1_dn8) + ((locals.var_cnst1_dn8 * assign32900_e37378) + (locals.var_cnst1 * ((assign32900_e37370 * (((-locals.var_beta_dn8) * assign32900_e37368) + (assign32900_e37365 * (locals.var_phi_sl_dep_dn8 - locals.var_vbscl__blk439_dn8)))) - (assign32900_e37377 * (((-locals.var_beta_dn8) * assign32900_e37375) + (assign32900_e37372 * (locals.var_phi_bl_dep_dn8 - locals.var_vbscl__blk439_dn8)))))))) / (2.0 * assign32900_e37381)))), ((locals.var_cnst0_dn9 * assign32900_e37381) + (locals.var_cnst0 * (((-locals.var_t1_dn9) + ((locals.var_cnst1_dn9 * assign32900_e37378) + (locals.var_cnst1 * ((assign32900_e37370 * (((-locals.var_beta_dn9) * assign32900_e37368) + (assign32900_e37365 * (locals.var_phi_sl_dep_dn9 - locals.var_vbscl__blk439_dn9)))) - (assign32900_e37377 * (((-locals.var_beta_dn9) * assign32900_e37375) + (assign32900_e37372 * (locals.var_phi_bl_dep_dn9 - locals.var_vbscl__blk439_dn9)))))))) / (2.0 * assign32900_e37381)))), ((locals.var_cnst0_dn10 * assign32900_e37381) + (locals.var_cnst0 * (((-locals.var_t1_dn10) + ((locals.var_cnst1_dn10 * assign32900_e37378) + (locals.var_cnst1 * ((assign32900_e37370 * (((-locals.var_beta_dn10) * assign32900_e37368) + (assign32900_e37365 * (locals.var_phi_sl_dep_dn10 - locals.var_vbscl__blk439_dn10)))) - (assign32900_e37377 * (((-locals.var_beta_dn10) * assign32900_e37375) + (assign32900_e37372 * (locals.var_phi_bl_dep_dn10 - locals.var_vbscl__blk439_dn10)))))))) / (2.0 * assign32900_e37381)))), ((locals.var_cnst0_dn11 * assign32900_e37381) + (locals.var_cnst0 * (((-locals.var_t1_dn11) + ((locals.var_cnst1_dn11 * assign32900_e37378) + (locals.var_cnst1 * ((assign32900_e37370 * (((-locals.var_beta_dn11) * assign32900_e37368) + (assign32900_e37365 * (locals.var_phi_sl_dep_dn11 - locals.var_vbscl__blk439_dn11)))) - (assign32900_e37377 * (((-locals.var_beta_dn11) * assign32900_e37375) + (assign32900_e37372 * (locals.var_phi_bl_dep_dn11 - locals.var_vbscl__blk439_dn11)))))))) / (2.0 * assign32900_e37381)))), ((locals.var_cnst0_dn14 * assign32900_e37381) + (locals.var_cnst0 * (((-locals.var_t1_dn14) + ((locals.var_cnst1_dn14 * assign32900_e37378) + (locals.var_cnst1 * ((assign32900_e37370 * (((-locals.var_beta_dn14) * assign32900_e37368) + (assign32900_e37365 * (locals.var_phi_sl_dep_dn14 - locals.var_vbscl__blk439_dn14)))) - (assign32900_e37377 * (((-locals.var_beta_dn14) * assign32900_e37375) + (assign32900_e37372 * (locals.var_phi_bl_dep_dn14 - locals.var_vbscl__blk439_dn14)))))))) / (2.0 * assign32900_e37381)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32900_e37384;
        locals.var_t3_dn0 = assign32900_e37384_d_n0;
        locals.var_t3_dn2 = assign32900_e37384_d_n2;
        locals.var_t3_dn4 = assign32900_e37384_d_n4;
        locals.var_t3_dn5 = assign32900_e37384_d_n5;
        locals.var_t3_dn6 = assign32900_e37384_d_n6;
        locals.var_t3_dn7 = assign32900_e37384_d_n7;
        locals.var_t3_dn8 = assign32900_e37384_d_n8;
        locals.var_t3_dn9 = assign32900_e37384_d_n9;
        locals.var_t3_dn10 = assign32900_e37384_d_n10;
        locals.var_t3_dn11 = assign32900_e37384_d_n11;
        locals.var_t3_dn14 = assign32900_e37384_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign32910_e37405, assign32910_e37405_d_n0, assign32910_e37405_d_n2, assign32910_e37405_d_n4, assign32910_e37405_d_n5, assign32910_e37405_d_n6, assign32910_e37405_d_n7, assign32910_e37405_d_n8, assign32910_e37405_d_n9, assign32910_e37405_d_n10, assign32910_e37405_d_n11, assign32910_e37405_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard783 == 0.0)) {
        let assign32910_e37400: f64 = (-locals.var_t1);
        let assign32910_e37401: f64 = (assign32910_e37400).sqrt();
        let assign32910_e37402: f64 = (locals.var_cnst0 * assign32910_e37401);
        let assign32910_e37403: f64 = (locals.var_t3 - assign32910_e37402);
        (assign32910_e37403, (locals.var_t3_dn0 - ((locals.var_cnst0_dn0 * assign32910_e37401) + (locals.var_cnst0 * ((-locals.var_t1_dn0) / (2.0 * assign32910_e37401))))), (locals.var_t3_dn2 - ((locals.var_cnst0_dn2 * assign32910_e37401) + (locals.var_cnst0 * ((-locals.var_t1_dn2) / (2.0 * assign32910_e37401))))), (locals.var_t3_dn4 - ((locals.var_cnst0_dn4 * assign32910_e37401) + (locals.var_cnst0 * ((-locals.var_t1_dn4) / (2.0 * assign32910_e37401))))), (locals.var_t3_dn5 - ((locals.var_cnst0_dn5 * assign32910_e37401) + (locals.var_cnst0 * ((-locals.var_t1_dn5) / (2.0 * assign32910_e37401))))), (locals.var_t3_dn6 - ((locals.var_cnst0_dn6 * assign32910_e37401) + (locals.var_cnst0 * ((-locals.var_t1_dn6) / (2.0 * assign32910_e37401))))), (locals.var_t3_dn7 - ((locals.var_cnst0_dn7 * assign32910_e37401) + (locals.var_cnst0 * ((-locals.var_t1_dn7) / (2.0 * assign32910_e37401))))), (locals.var_t3_dn8 - ((locals.var_cnst0_dn8 * assign32910_e37401) + (locals.var_cnst0 * ((-locals.var_t1_dn8) / (2.0 * assign32910_e37401))))), (locals.var_t3_dn9 - ((locals.var_cnst0_dn9 * assign32910_e37401) + (locals.var_cnst0 * ((-locals.var_t1_dn9) / (2.0 * assign32910_e37401))))), (locals.var_t3_dn10 - ((locals.var_cnst0_dn10 * assign32910_e37401) + (locals.var_cnst0 * ((-locals.var_t1_dn10) / (2.0 * assign32910_e37401))))), (locals.var_t3_dn11 - ((locals.var_cnst0_dn11 * assign32910_e37401) + (locals.var_cnst0 * ((-locals.var_t1_dn11) / (2.0 * assign32910_e37401))))), (locals.var_t3_dn14 - ((locals.var_cnst0_dn14 * assign32910_e37401) + (locals.var_cnst0 * ((-locals.var_t1_dn14) / (2.0 * assign32910_e37401))))),)
    } else {
        (locals.var_q_subl, locals.var_q_subl_dn0, locals.var_q_subl_dn2, locals.var_q_subl_dn4, locals.var_q_subl_dn5, locals.var_q_subl_dn6, locals.var_q_subl_dn7, locals.var_q_subl_dn8, locals.var_q_subl_dn9, locals.var_q_subl_dn10, locals.var_q_subl_dn11, locals.var_q_subl_dn14,)
    }
};
        locals.var_q_subl = assign32910_e37405;
        locals.var_q_subl_dn0 = assign32910_e37405_d_n0;
        locals.var_q_subl_dn2 = assign32910_e37405_d_n2;
        locals.var_q_subl_dn4 = assign32910_e37405_d_n4;
        locals.var_q_subl_dn5 = assign32910_e37405_d_n5;
        locals.var_q_subl_dn6 = assign32910_e37405_d_n6;
        locals.var_q_subl_dn7 = assign32910_e37405_d_n7;
        locals.var_q_subl_dn8 = assign32910_e37405_d_n8;
        locals.var_q_subl_dn9 = assign32910_e37405_d_n9;
        locals.var_q_subl_dn10 = assign32910_e37405_d_n10;
        locals.var_q_subl_dn11 = assign32910_e37405_d_n11;
        locals.var_q_subl_dn14 = assign32910_e37405_d_n14;
        locals.var_q_subl_rv = 0.0;

        let (assign32920_e37429, assign32920_e37429_d_n0, assign32920_e37429_d_n2, assign32920_e37429_d_n4, assign32920_e37429_d_n5, assign32920_e37429_d_n6, assign32920_e37429_d_n7, assign32920_e37429_d_n8, assign32920_e37429_d_n9, assign32920_e37429_d_n10, assign32920_e37429_d_n11, assign32920_e37429_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard783 == 0.0)) {
        let assign32920_e37421: f64 = (locals.var_t2 - 1.0);
        let assign32920_e37423: f64 = (assign32920_e37421 - locals.var_t1);
        let assign32920_e37425: f64 = (assign32920_e37423 + 1e-15);
        let assign32920_e37426: f64 = (assign32920_e37425).sqrt();
        let assign32920_e37427: f64 = (locals.var_cnst0 * assign32920_e37426);
        (assign32920_e37427, ((locals.var_cnst0_dn0 * assign32920_e37426) + (locals.var_cnst0 * ((locals.var_t2_dn0 - locals.var_t1_dn0) / (2.0 * assign32920_e37426)))), ((locals.var_cnst0_dn2 * assign32920_e37426) + (locals.var_cnst0 * ((locals.var_t2_dn2 - locals.var_t1_dn2) / (2.0 * assign32920_e37426)))), ((locals.var_cnst0_dn4 * assign32920_e37426) + (locals.var_cnst0 * ((locals.var_t2_dn4 - locals.var_t1_dn4) / (2.0 * assign32920_e37426)))), ((locals.var_cnst0_dn5 * assign32920_e37426) + (locals.var_cnst0 * ((locals.var_t2_dn5 - locals.var_t1_dn5) / (2.0 * assign32920_e37426)))), ((locals.var_cnst0_dn6 * assign32920_e37426) + (locals.var_cnst0 * ((locals.var_t2_dn6 - locals.var_t1_dn6) / (2.0 * assign32920_e37426)))), ((locals.var_cnst0_dn7 * assign32920_e37426) + (locals.var_cnst0 * ((locals.var_t2_dn7 - locals.var_t1_dn7) / (2.0 * assign32920_e37426)))), ((locals.var_cnst0_dn8 * assign32920_e37426) + (locals.var_cnst0 * ((locals.var_t2_dn8 - locals.var_t1_dn8) / (2.0 * assign32920_e37426)))), ((locals.var_cnst0_dn9 * assign32920_e37426) + (locals.var_cnst0 * ((locals.var_t2_dn9 - locals.var_t1_dn9) / (2.0 * assign32920_e37426)))), ((locals.var_cnst0_dn10 * assign32920_e37426) + (locals.var_cnst0 * ((locals.var_t2_dn10 - locals.var_t1_dn10) / (2.0 * assign32920_e37426)))), ((locals.var_cnst0_dn11 * assign32920_e37426) + (locals.var_cnst0 * ((locals.var_t2_dn11 - locals.var_t1_dn11) / (2.0 * assign32920_e37426)))), ((locals.var_cnst0_dn14 * assign32920_e37426) + (locals.var_cnst0 * ((locals.var_t2_dn14 - locals.var_t1_dn14) / (2.0 * assign32920_e37426)))),)
    } else {
        (locals.var_q_sl_dep, locals.var_q_sl_dep_dn0, locals.var_q_sl_dep_dn2, locals.var_q_sl_dep_dn4, locals.var_q_sl_dep_dn5, locals.var_q_sl_dep_dn6, locals.var_q_sl_dep_dn7, locals.var_q_sl_dep_dn8, locals.var_q_sl_dep_dn9, locals.var_q_sl_dep_dn10, locals.var_q_sl_dep_dn11, locals.var_q_sl_dep_dn14,)
    }
};
        locals.var_q_sl_dep = assign32920_e37429;
        locals.var_q_sl_dep_dn0 = assign32920_e37429_d_n0;
        locals.var_q_sl_dep_dn2 = assign32920_e37429_d_n2;
        locals.var_q_sl_dep_dn4 = assign32920_e37429_d_n4;
        locals.var_q_sl_dep_dn5 = assign32920_e37429_d_n5;
        locals.var_q_sl_dep_dn6 = assign32920_e37429_d_n6;
        locals.var_q_sl_dep_dn7 = assign32920_e37429_d_n7;
        locals.var_q_sl_dep_dn8 = assign32920_e37429_d_n8;
        locals.var_q_sl_dep_dn9 = assign32920_e37429_d_n9;
        locals.var_q_sl_dep_dn10 = assign32920_e37429_d_n10;
        locals.var_q_sl_dep_dn11 = assign32920_e37429_d_n11;
        locals.var_q_sl_dep_dn14 = assign32920_e37429_d_n14;
        locals.var_q_sl_dep_rv = 0.0;

        let (assign32930_e37441, assign32930_e37441_d_n0, assign32930_e37441_d_n2, assign32930_e37441_d_n4, assign32930_e37441_d_n5, assign32930_e37441_d_n6, assign32930_e37441_d_n7, assign32930_e37441_d_n8, assign32930_e37441_d_n9, assign32930_e37441_d_n10, assign32930_e37441_d_n11, assign32930_e37441_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_nl, locals.var_q_nl_dn0, locals.var_q_nl_dn2, locals.var_q_nl_dn4, locals.var_q_nl_dn5, locals.var_q_nl_dn6, locals.var_q_nl_dn7, locals.var_q_nl_dn8, locals.var_q_nl_dn9, locals.var_q_nl_dn10, locals.var_q_nl_dn11, locals.var_q_nl_dn14,)
    }
};
        locals.var_q_nl = assign32930_e37441;
        locals.var_q_nl_dn0 = assign32930_e37441_d_n0;
        locals.var_q_nl_dn2 = assign32930_e37441_d_n2;
        locals.var_q_nl_dn4 = assign32930_e37441_d_n4;
        locals.var_q_nl_dn5 = assign32930_e37441_d_n5;
        locals.var_q_nl_dn6 = assign32930_e37441_d_n6;
        locals.var_q_nl_dn7 = assign32930_e37441_d_n7;
        locals.var_q_nl_dn8 = assign32930_e37441_d_n8;
        locals.var_q_nl_dn9 = assign32930_e37441_d_n9;
        locals.var_q_nl_dn10 = assign32930_e37441_d_n10;
        locals.var_q_nl_dn11 = assign32930_e37441_d_n11;
        locals.var_q_nl_dn14 = assign32930_e37441_d_n14;
        locals.var_q_nl_rv = 0.0;

        let (assign32940_e37455, assign32940_e37455_d_n0, assign32940_e37455_d_n2, assign32940_e37455_d_n4, assign32940_e37455_d_n5, assign32940_e37455_d_n6, assign32940_e37455_d_n7, assign32940_e37455_d_n8, assign32940_e37455_d_n9, assign32940_e37455_d_n10, assign32940_e37455_d_n11, assign32940_e37455_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) {
        let assign32940_e37453: f64 = (locals.var_phi_bl_dep - locals.var_phi_jl_dep);
        (assign32940_e37453, (locals.var_phi_bl_dep_dn0 - locals.var_phi_jl_dep_dn0), (locals.var_phi_bl_dep_dn2 - locals.var_phi_jl_dep_dn2), (locals.var_phi_bl_dep_dn4 - locals.var_phi_jl_dep_dn4), (locals.var_phi_bl_dep_dn5 - locals.var_phi_jl_dep_dn5), (locals.var_phi_bl_dep_dn6 - locals.var_phi_jl_dep_dn6), (locals.var_phi_bl_dep_dn7 - locals.var_phi_jl_dep_dn7), (locals.var_phi_bl_dep_dn8 - locals.var_phi_jl_dep_dn8), (locals.var_phi_bl_dep_dn9 - locals.var_phi_jl_dep_dn9), (locals.var_phi_bl_dep_dn10 - locals.var_phi_jl_dep_dn10), (locals.var_phi_bl_dep_dn11 - locals.var_phi_jl_dep_dn11), (locals.var_phi_bl_dep_dn14 - locals.var_phi_jl_dep_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign32940_e37455;
        locals.var_t1_dn0 = assign32940_e37455_d_n0;
        locals.var_t1_dn2 = assign32940_e37455_d_n2;
        locals.var_t1_dn4 = assign32940_e37455_d_n4;
        locals.var_t1_dn5 = assign32940_e37455_d_n5;
        locals.var_t1_dn6 = assign32940_e37455_d_n6;
        locals.var_t1_dn7 = assign32940_e37455_d_n7;
        locals.var_t1_dn8 = assign32940_e37455_d_n8;
        locals.var_t1_dn9 = assign32940_e37455_d_n9;
        locals.var_t1_dn10 = assign32940_e37455_d_n10;
        locals.var_t1_dn11 = assign32940_e37455_d_n11;
        locals.var_t1_dn14 = assign32940_e37455_d_n14;
        locals.var_t1_rv = 0.0;

        let assign32950_e37459: f64 = 0.1;
        let assign32950_e37464: f64 = if ((locals.var_t1 < assign32950_e37459) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard784 = assign32950_e37464;
        locals.var_guard784_rv = 0.0;

        let (assign32960_e37482, assign32960_e37482_d_n0, assign32960_e37482_d_n2, assign32960_e37482_d_n4, assign32960_e37482_d_n5, assign32960_e37482_d_n6, assign32960_e37482_d_n7, assign32960_e37482_d_n8, assign32960_e37482_d_n9, assign32960_e37482_d_n10, assign32960_e37482_d_n11, assign32960_e37482_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard784 != 0.0)) {
        let assign32960_e37478: f64 = 0.1;
        let assign32960_e37480: f64 = (assign32960_e37478 - locals.var_t1);
        (assign32960_e37480, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11), (-locals.var_t1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign32960_e37482;
        locals.var_tmf1_dn0 = assign32960_e37482_d_n0;
        locals.var_tmf1_dn2 = assign32960_e37482_d_n2;
        locals.var_tmf1_dn4 = assign32960_e37482_d_n4;
        locals.var_tmf1_dn5 = assign32960_e37482_d_n5;
        locals.var_tmf1_dn6 = assign32960_e37482_d_n6;
        locals.var_tmf1_dn7 = assign32960_e37482_d_n7;
        locals.var_tmf1_dn8 = assign32960_e37482_d_n8;
        locals.var_tmf1_dn9 = assign32960_e37482_d_n9;
        locals.var_tmf1_dn10 = assign32960_e37482_d_n10;
        locals.var_tmf1_dn11 = assign32960_e37482_d_n11;
        locals.var_tmf1_dn14 = assign32960_e37482_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign32970_e37498, assign32970_e37498_d_n0, assign32970_e37498_d_n2, assign32970_e37498_d_n4, assign32970_e37498_d_n5, assign32970_e37498_d_n6, assign32970_e37498_d_n7, assign32970_e37498_d_n8, assign32970_e37498_d_n9, assign32970_e37498_d_n10, assign32970_e37498_d_n11, assign32970_e37498_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard784 != 0.0)) {
        let assign32970_e37496: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign32970_e37496, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign32970_e37498;
        locals.var_x2_dn0 = assign32970_e37498_d_n0;
        locals.var_x2_dn2 = assign32970_e37498_d_n2;
        locals.var_x2_dn4 = assign32970_e37498_d_n4;
        locals.var_x2_dn5 = assign32970_e37498_d_n5;
        locals.var_x2_dn6 = assign32970_e37498_d_n6;
        locals.var_x2_dn7 = assign32970_e37498_d_n7;
        locals.var_x2_dn8 = assign32970_e37498_d_n8;
        locals.var_x2_dn9 = assign32970_e37498_d_n9;
        locals.var_x2_dn10 = assign32970_e37498_d_n10;
        locals.var_x2_dn11 = assign32970_e37498_d_n11;
        locals.var_x2_dn14 = assign32970_e37498_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign32980_e37514, assign32980_e37514_d_n0, assign32980_e37514_d_n2, assign32980_e37514_d_n4, assign32980_e37514_d_n5, assign32980_e37514_d_n6, assign32980_e37514_d_n7, assign32980_e37514_d_n8, assign32980_e37514_d_n9, assign32980_e37514_d_n10, assign32980_e37514_d_n11, assign32980_e37514_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard784 != 0.0)) {
        let assign32980_e37512: f64 = (0.1 * 0.1);
        (assign32980_e37512, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign32980_e37514;
        locals.var_xmax2_dn0 = assign32980_e37514_d_n0;
        locals.var_xmax2_dn2 = assign32980_e37514_d_n2;
        locals.var_xmax2_dn4 = assign32980_e37514_d_n4;
        locals.var_xmax2_dn5 = assign32980_e37514_d_n5;
        locals.var_xmax2_dn6 = assign32980_e37514_d_n6;
        locals.var_xmax2_dn7 = assign32980_e37514_d_n7;
        locals.var_xmax2_dn8 = assign32980_e37514_d_n8;
        locals.var_xmax2_dn9 = assign32980_e37514_d_n9;
        locals.var_xmax2_dn10 = assign32980_e37514_d_n10;
        locals.var_xmax2_dn11 = assign32980_e37514_d_n11;
        locals.var_xmax2_dn14 = assign32980_e37514_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign32990_e37528, assign32990_e37528_d_n0, assign32990_e37528_d_n2, assign32990_e37528_d_n4, assign32990_e37528_d_n5, assign32990_e37528_d_n6, assign32990_e37528_d_n7, assign32990_e37528_d_n8, assign32990_e37528_d_n9, assign32990_e37528_d_n10, assign32990_e37528_d_n11, assign32990_e37528_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard784 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign32990_e37528;
        locals.var_xp_dn0 = assign32990_e37528_d_n0;
        locals.var_xp_dn2 = assign32990_e37528_d_n2;
        locals.var_xp_dn4 = assign32990_e37528_d_n4;
        locals.var_xp_dn5 = assign32990_e37528_d_n5;
        locals.var_xp_dn6 = assign32990_e37528_d_n6;
        locals.var_xp_dn7 = assign32990_e37528_d_n7;
        locals.var_xp_dn8 = assign32990_e37528_d_n8;
        locals.var_xp_dn9 = assign32990_e37528_d_n9;
        locals.var_xp_dn10 = assign32990_e37528_d_n10;
        locals.var_xp_dn11 = assign32990_e37528_d_n11;
        locals.var_xp_dn14 = assign32990_e37528_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign33000_e37542, assign33000_e37542_d_n0, assign33000_e37542_d_n2, assign33000_e37542_d_n4, assign33000_e37542_d_n5, assign33000_e37542_d_n6, assign33000_e37542_d_n7, assign33000_e37542_d_n8, assign33000_e37542_d_n9, assign33000_e37542_d_n10, assign33000_e37542_d_n11, assign33000_e37542_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard784 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign33000_e37542;
        locals.var_xmp_dn0 = assign33000_e37542_d_n0;
        locals.var_xmp_dn2 = assign33000_e37542_d_n2;
        locals.var_xmp_dn4 = assign33000_e37542_d_n4;
        locals.var_xmp_dn5 = assign33000_e37542_d_n5;
        locals.var_xmp_dn6 = assign33000_e37542_d_n6;
        locals.var_xmp_dn7 = assign33000_e37542_d_n7;
        locals.var_xmp_dn8 = assign33000_e37542_d_n8;
        locals.var_xmp_dn9 = assign33000_e37542_d_n9;
        locals.var_xmp_dn10 = assign33000_e37542_d_n10;
        locals.var_xmp_dn11 = assign33000_e37542_d_n11;
        locals.var_xmp_dn14 = assign33000_e37542_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign33010_e37556,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard784 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign33010_e37556;
        locals.var_m0_rv = 0.0;

        let (assign33020_e37570,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard784 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33020_e37570;
        locals.var_mm_rv = 0.0;

        let (assign33030_e37584, assign33030_e37584_d_n0, assign33030_e37584_d_n2, assign33030_e37584_d_n4, assign33030_e37584_d_n5, assign33030_e37584_d_n6, assign33030_e37584_d_n7, assign33030_e37584_d_n8, assign33030_e37584_d_n9, assign33030_e37584_d_n10, assign33030_e37584_d_n11, assign33030_e37584_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard784 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign33030_e37584;
        locals.var_arg_dn0 = assign33030_e37584_d_n0;
        locals.var_arg_dn2 = assign33030_e37584_d_n2;
        locals.var_arg_dn4 = assign33030_e37584_d_n4;
        locals.var_arg_dn5 = assign33030_e37584_d_n5;
        locals.var_arg_dn6 = assign33030_e37584_d_n6;
        locals.var_arg_dn7 = assign33030_e37584_d_n7;
        locals.var_arg_dn8 = assign33030_e37584_d_n8;
        locals.var_arg_dn9 = assign33030_e37584_d_n9;
        locals.var_arg_dn10 = assign33030_e37584_d_n10;
        locals.var_arg_dn11 = assign33030_e37584_d_n11;
        locals.var_arg_dn14 = assign33030_e37584_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign33040_e37598, assign33040_e37598_d_n0, assign33040_e37598_d_n2, assign33040_e37598_d_n4, assign33040_e37598_d_n5, assign33040_e37598_d_n6, assign33040_e37598_d_n7, assign33040_e37598_d_n8, assign33040_e37598_d_n9, assign33040_e37598_d_n10, assign33040_e37598_d_n11, assign33040_e37598_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard784 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign33040_e37598;
        locals.var_dnm_dn0 = assign33040_e37598_d_n0;
        locals.var_dnm_dn2 = assign33040_e37598_d_n2;
        locals.var_dnm_dn4 = assign33040_e37598_d_n4;
        locals.var_dnm_dn5 = assign33040_e37598_d_n5;
        locals.var_dnm_dn6 = assign33040_e37598_d_n6;
        locals.var_dnm_dn7 = assign33040_e37598_d_n7;
        locals.var_dnm_dn8 = assign33040_e37598_d_n8;
        locals.var_dnm_dn9 = assign33040_e37598_d_n9;
        locals.var_dnm_dn10 = assign33040_e37598_d_n10;
        locals.var_dnm_dn11 = assign33040_e37598_d_n11;
        locals.var_dnm_dn14 = assign33040_e37598_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign33050_e37614, assign33050_e37614_d_n0, assign33050_e37614_d_n2, assign33050_e37614_d_n4, assign33050_e37614_d_n5, assign33050_e37614_d_n6, assign33050_e37614_d_n7, assign33050_e37614_d_n8, assign33050_e37614_d_n9, assign33050_e37614_d_n10, assign33050_e37614_d_n11, assign33050_e37614_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard784 != 0.0)) {
        let assign33050_e37612: f64 = (locals.var_xp * locals.var_x2);
        (assign33050_e37612, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign33050_e37614;
        locals.var_xp_dn0 = assign33050_e37614_d_n0;
        locals.var_xp_dn2 = assign33050_e37614_d_n2;
        locals.var_xp_dn4 = assign33050_e37614_d_n4;
        locals.var_xp_dn5 = assign33050_e37614_d_n5;
        locals.var_xp_dn6 = assign33050_e37614_d_n6;
        locals.var_xp_dn7 = assign33050_e37614_d_n7;
        locals.var_xp_dn8 = assign33050_e37614_d_n8;
        locals.var_xp_dn9 = assign33050_e37614_d_n9;
        locals.var_xp_dn10 = assign33050_e37614_d_n10;
        locals.var_xp_dn11 = assign33050_e37614_d_n11;
        locals.var_xp_dn14 = assign33050_e37614_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign33060_e37630, assign33060_e37630_d_n0, assign33060_e37630_d_n2, assign33060_e37630_d_n4, assign33060_e37630_d_n5, assign33060_e37630_d_n6, assign33060_e37630_d_n7, assign33060_e37630_d_n8, assign33060_e37630_d_n9, assign33060_e37630_d_n10, assign33060_e37630_d_n11, assign33060_e37630_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard784 != 0.0)) {
        let assign33060_e37628: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign33060_e37628, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign33060_e37630;
        locals.var_xmp_dn0 = assign33060_e37630_d_n0;
        locals.var_xmp_dn2 = assign33060_e37630_d_n2;
        locals.var_xmp_dn4 = assign33060_e37630_d_n4;
        locals.var_xmp_dn5 = assign33060_e37630_d_n5;
        locals.var_xmp_dn6 = assign33060_e37630_d_n6;
        locals.var_xmp_dn7 = assign33060_e37630_d_n7;
        locals.var_xmp_dn8 = assign33060_e37630_d_n8;
        locals.var_xmp_dn9 = assign33060_e37630_d_n9;
        locals.var_xmp_dn10 = assign33060_e37630_d_n10;
        locals.var_xmp_dn11 = assign33060_e37630_d_n11;
        locals.var_xmp_dn14 = assign33060_e37630_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign33070_e37646, assign33070_e37646_d_n0, assign33070_e37646_d_n2, assign33070_e37646_d_n4, assign33070_e37646_d_n5, assign33070_e37646_d_n6, assign33070_e37646_d_n7, assign33070_e37646_d_n8, assign33070_e37646_d_n9, assign33070_e37646_d_n10, assign33070_e37646_d_n11, assign33070_e37646_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard784 != 0.0)) {
        let assign33070_e37644: f64 = (locals.var_xp * locals.var_x2);
        (assign33070_e37644, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign33070_e37646;
        locals.var_xp_dn0 = assign33070_e37646_d_n0;
        locals.var_xp_dn2 = assign33070_e37646_d_n2;
        locals.var_xp_dn4 = assign33070_e37646_d_n4;
        locals.var_xp_dn5 = assign33070_e37646_d_n5;
        locals.var_xp_dn6 = assign33070_e37646_d_n6;
        locals.var_xp_dn7 = assign33070_e37646_d_n7;
        locals.var_xp_dn8 = assign33070_e37646_d_n8;
        locals.var_xp_dn9 = assign33070_e37646_d_n9;
        locals.var_xp_dn10 = assign33070_e37646_d_n10;
        locals.var_xp_dn11 = assign33070_e37646_d_n11;
        locals.var_xp_dn14 = assign33070_e37646_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign33080_e37662, assign33080_e37662_d_n0, assign33080_e37662_d_n2, assign33080_e37662_d_n4, assign33080_e37662_d_n5, assign33080_e37662_d_n6, assign33080_e37662_d_n7, assign33080_e37662_d_n8, assign33080_e37662_d_n9, assign33080_e37662_d_n10, assign33080_e37662_d_n11, assign33080_e37662_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard784 != 0.0)) {
        let assign33080_e37660: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign33080_e37660, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign33080_e37662;
        locals.var_xmp_dn0 = assign33080_e37662_d_n0;
        locals.var_xmp_dn2 = assign33080_e37662_d_n2;
        locals.var_xmp_dn4 = assign33080_e37662_d_n4;
        locals.var_xmp_dn5 = assign33080_e37662_d_n5;
        locals.var_xmp_dn6 = assign33080_e37662_d_n6;
        locals.var_xmp_dn7 = assign33080_e37662_d_n7;
        locals.var_xmp_dn8 = assign33080_e37662_d_n8;
        locals.var_xmp_dn9 = assign33080_e37662_d_n9;
        locals.var_xmp_dn10 = assign33080_e37662_d_n10;
        locals.var_xmp_dn11 = assign33080_e37662_d_n11;
        locals.var_xmp_dn14 = assign33080_e37662_d_n14;
        locals.var_xmp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_107(
        locals: &mut StampLocals,
    ) {
        let (assign33090_e37678, assign33090_e37678_d_n0, assign33090_e37678_d_n2, assign33090_e37678_d_n4, assign33090_e37678_d_n5, assign33090_e37678_d_n6, assign33090_e37678_d_n7, assign33090_e37678_d_n8, assign33090_e37678_d_n9, assign33090_e37678_d_n10, assign33090_e37678_d_n11, assign33090_e37678_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard784 != 0.0)) {
        let assign33090_e37676: f64 = (locals.var_xp + locals.var_xmp);
        (assign33090_e37676, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign33090_e37678;
        locals.var_arg_dn0 = assign33090_e37678_d_n0;
        locals.var_arg_dn2 = assign33090_e37678_d_n2;
        locals.var_arg_dn4 = assign33090_e37678_d_n4;
        locals.var_arg_dn5 = assign33090_e37678_d_n5;
        locals.var_arg_dn6 = assign33090_e37678_d_n6;
        locals.var_arg_dn7 = assign33090_e37678_d_n7;
        locals.var_arg_dn8 = assign33090_e37678_d_n8;
        locals.var_arg_dn9 = assign33090_e37678_d_n9;
        locals.var_arg_dn10 = assign33090_e37678_d_n10;
        locals.var_arg_dn11 = assign33090_e37678_d_n11;
        locals.var_arg_dn14 = assign33090_e37678_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign33100_e37692, assign33100_e37692_d_n0, assign33100_e37692_d_n2, assign33100_e37692_d_n4, assign33100_e37692_d_n5, assign33100_e37692_d_n6, assign33100_e37692_d_n7, assign33100_e37692_d_n8, assign33100_e37692_d_n9, assign33100_e37692_d_n10, assign33100_e37692_d_n11, assign33100_e37692_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard784 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign33100_e37692;
        locals.var_dnm_dn0 = assign33100_e37692_d_n0;
        locals.var_dnm_dn2 = assign33100_e37692_d_n2;
        locals.var_dnm_dn4 = assign33100_e37692_d_n4;
        locals.var_dnm_dn5 = assign33100_e37692_d_n5;
        locals.var_dnm_dn6 = assign33100_e37692_d_n6;
        locals.var_dnm_dn7 = assign33100_e37692_d_n7;
        locals.var_dnm_dn8 = assign33100_e37692_d_n8;
        locals.var_dnm_dn9 = assign33100_e37692_d_n9;
        locals.var_dnm_dn10 = assign33100_e37692_d_n10;
        locals.var_dnm_dn11 = assign33100_e37692_d_n11;
        locals.var_dnm_dn14 = assign33100_e37692_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign33110_e37707: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard785 = assign33110_e37707;
        locals.var_guard785_rv = 0.0;

        let assign33120_e37710: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard786 = assign33120_e37710;
        locals.var_guard786_rv = 0.0;

        let (assign33130_e37728,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard784 != 0.0)) && (locals.var_guard785 != 0.0)) && (locals.var_guard786 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33130_e37728;
        locals.var_mm_rv = 0.0;

        let assign33140_e37731: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard787 = assign33140_e37731;
        locals.var_guard787_rv = 0.0;

        let (assign33150_e37752,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard784 != 0.0)) && (locals.var_guard785 != 0.0)) && (locals.var_guard786 == 0.0)) && (locals.var_guard787 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33150_e37752;
        locals.var_mm_rv = 0.0;

        let assign33160_e37755: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard788 = assign33160_e37755;
        locals.var_guard788_rv = 0.0;

        let (assign33170_e37779,) = {
    if (((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard784 != 0.0)) && (locals.var_guard785 != 0.0)) && (locals.var_guard786 == 0.0)) && (locals.var_guard787 == 0.0)) && (locals.var_guard788 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33170_e37779;
        locals.var_mm_rv = 0.0;

        let assign33180_e37782: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard789 = assign33180_e37782;
        locals.var_guard789_rv = 0.0;

        let (assign33190_e37809,) = {
    if ((((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard784 != 0.0)) && (locals.var_guard785 != 0.0)) && (locals.var_guard786 == 0.0)) && (locals.var_guard787 == 0.0)) && (locals.var_guard788 == 0.0)) && (locals.var_guard789 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33190_e37809;
        locals.var_mm_rv = 0.0;

        let (assign33200_e37825,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard784 != 0.0)) && (locals.var_guard785 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign33200_e37825;
        locals.var_m0_rv = 0.0;

        let mut assign33210_loop_guard: usize = 0;
        while {
            let assign33210_cond_e37842: f64 = if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard784 != 0.0)) && (locals.var_guard785 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign33210_cond_e37842 != 0.0
        } {
            assign33210_loop_guard += 1;
            assert!(assign33210_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign33210_body0_e37859, assign33210_body0_e37859_d_n0, assign33210_body0_e37859_d_n2, assign33210_body0_e37859_d_n4, assign33210_body0_e37859_d_n5, assign33210_body0_e37859_d_n6, assign33210_body0_e37859_d_n7, assign33210_body0_e37859_d_n8, assign33210_body0_e37859_d_n9, assign33210_body0_e37859_d_n10, assign33210_body0_e37859_d_n11, assign33210_body0_e37859_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard784 != 0.0)) && (locals.var_guard785 != 0.0)) {
        let assign33210_body0_e37857: f64 = (locals.var_dnm).sqrt();
        (assign33210_body0_e37857, (locals.var_dnm_dn0 / (2.0 * assign33210_body0_e37857)), (locals.var_dnm_dn2 / (2.0 * assign33210_body0_e37857)), (locals.var_dnm_dn4 / (2.0 * assign33210_body0_e37857)), (locals.var_dnm_dn5 / (2.0 * assign33210_body0_e37857)), (locals.var_dnm_dn6 / (2.0 * assign33210_body0_e37857)), (locals.var_dnm_dn7 / (2.0 * assign33210_body0_e37857)), (locals.var_dnm_dn8 / (2.0 * assign33210_body0_e37857)), (locals.var_dnm_dn9 / (2.0 * assign33210_body0_e37857)), (locals.var_dnm_dn10 / (2.0 * assign33210_body0_e37857)), (locals.var_dnm_dn11 / (2.0 * assign33210_body0_e37857)), (locals.var_dnm_dn14 / (2.0 * assign33210_body0_e37857)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign33210_body0_e37859;
            locals.var_dnm_dn0 = assign33210_body0_e37859_d_n0;
            locals.var_dnm_dn2 = assign33210_body0_e37859_d_n2;
            locals.var_dnm_dn4 = assign33210_body0_e37859_d_n4;
            locals.var_dnm_dn5 = assign33210_body0_e37859_d_n5;
            locals.var_dnm_dn6 = assign33210_body0_e37859_d_n6;
            locals.var_dnm_dn7 = assign33210_body0_e37859_d_n7;
            locals.var_dnm_dn8 = assign33210_body0_e37859_d_n8;
            locals.var_dnm_dn9 = assign33210_body0_e37859_d_n9;
            locals.var_dnm_dn10 = assign33210_body0_e37859_d_n10;
            locals.var_dnm_dn11 = assign33210_body0_e37859_d_n11;
            locals.var_dnm_dn14 = assign33210_body0_e37859_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign33210_body1_e37877,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard784 != 0.0)) && (locals.var_guard785 != 0.0)) {
        let assign33210_body1_e37875: f64 = (locals.var_m0 + 1.0);
        (assign33210_body1_e37875,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign33210_body1_e37877;
            locals.var_m0_rv = 0.0;
        }

        let (assign33220_e37905, assign33220_e37905_d_n0, assign33220_e37905_d_n2, assign33220_e37905_d_n4, assign33220_e37905_d_n5, assign33220_e37905_d_n6, assign33220_e37905_d_n7, assign33220_e37905_d_n8, assign33220_e37905_d_n9, assign33220_e37905_d_n10, assign33220_e37905_d_n11, assign33220_e37905_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard784 != 0.0)) && (locals.var_guard785 == 0.0)) {
        let (assign33220_e37903, assign33220_e37903_d_n0, assign33220_e37903_d_n2, assign33220_e37903_d_n4, assign33220_e37903_d_n5, assign33220_e37903_d_n6, assign33220_e37903_d_n7, assign33220_e37903_d_n8, assign33220_e37903_d_n9, assign33220_e37903_d_n10, assign33220_e37903_d_n11, assign33220_e37903_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign33220_e37900: f64 = (2.0 * 2.0);
                let assign33220_e37901: f64 = (1.0 / assign33220_e37900);
                let assign33220_e37902: f64 = (locals.var_dnm).powf(assign33220_e37901);
                (assign33220_e37902, if 0.0 == 0.0 && ((assign33220_e37901) as f64).is_finite() && ((assign33220_e37901) as f64).fract() == 0.0 { if assign33220_e37901 == 0.0 { 0.0 } else { (assign33220_e37901 * ((locals.var_dnm).powf(assign33220_e37901 - 1.0) * locals.var_dnm_dn0)) } } else { (assign33220_e37902 * (assign33220_e37901 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33220_e37901) as f64).is_finite() && ((assign33220_e37901) as f64).fract() == 0.0 { if assign33220_e37901 == 0.0 { 0.0 } else { (assign33220_e37901 * ((locals.var_dnm).powf(assign33220_e37901 - 1.0) * locals.var_dnm_dn2)) } } else { (assign33220_e37902 * (assign33220_e37901 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33220_e37901) as f64).is_finite() && ((assign33220_e37901) as f64).fract() == 0.0 { if assign33220_e37901 == 0.0 { 0.0 } else { (assign33220_e37901 * ((locals.var_dnm).powf(assign33220_e37901 - 1.0) * locals.var_dnm_dn4)) } } else { (assign33220_e37902 * (assign33220_e37901 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33220_e37901) as f64).is_finite() && ((assign33220_e37901) as f64).fract() == 0.0 { if assign33220_e37901 == 0.0 { 0.0 } else { (assign33220_e37901 * ((locals.var_dnm).powf(assign33220_e37901 - 1.0) * locals.var_dnm_dn5)) } } else { (assign33220_e37902 * (assign33220_e37901 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33220_e37901) as f64).is_finite() && ((assign33220_e37901) as f64).fract() == 0.0 { if assign33220_e37901 == 0.0 { 0.0 } else { (assign33220_e37901 * ((locals.var_dnm).powf(assign33220_e37901 - 1.0) * locals.var_dnm_dn6)) } } else { (assign33220_e37902 * (assign33220_e37901 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33220_e37901) as f64).is_finite() && ((assign33220_e37901) as f64).fract() == 0.0 { if assign33220_e37901 == 0.0 { 0.0 } else { (assign33220_e37901 * ((locals.var_dnm).powf(assign33220_e37901 - 1.0) * locals.var_dnm_dn7)) } } else { (assign33220_e37902 * (assign33220_e37901 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33220_e37901) as f64).is_finite() && ((assign33220_e37901) as f64).fract() == 0.0 { if assign33220_e37901 == 0.0 { 0.0 } else { (assign33220_e37901 * ((locals.var_dnm).powf(assign33220_e37901 - 1.0) * locals.var_dnm_dn8)) } } else { (assign33220_e37902 * (assign33220_e37901 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33220_e37901) as f64).is_finite() && ((assign33220_e37901) as f64).fract() == 0.0 { if assign33220_e37901 == 0.0 { 0.0 } else { (assign33220_e37901 * ((locals.var_dnm).powf(assign33220_e37901 - 1.0) * locals.var_dnm_dn9)) } } else { (assign33220_e37902 * (assign33220_e37901 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33220_e37901) as f64).is_finite() && ((assign33220_e37901) as f64).fract() == 0.0 { if assign33220_e37901 == 0.0 { 0.0 } else { (assign33220_e37901 * ((locals.var_dnm).powf(assign33220_e37901 - 1.0) * locals.var_dnm_dn10)) } } else { (assign33220_e37902 * (assign33220_e37901 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33220_e37901) as f64).is_finite() && ((assign33220_e37901) as f64).fract() == 0.0 { if assign33220_e37901 == 0.0 { 0.0 } else { (assign33220_e37901 * ((locals.var_dnm).powf(assign33220_e37901 - 1.0) * locals.var_dnm_dn11)) } } else { (assign33220_e37902 * (assign33220_e37901 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33220_e37901) as f64).is_finite() && ((assign33220_e37901) as f64).fract() == 0.0 { if assign33220_e37901 == 0.0 { 0.0 } else { (assign33220_e37901 * ((locals.var_dnm).powf(assign33220_e37901 - 1.0) * locals.var_dnm_dn14)) } } else { (assign33220_e37902 * (assign33220_e37901 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign33220_e37903, assign33220_e37903_d_n0, assign33220_e37903_d_n2, assign33220_e37903_d_n4, assign33220_e37903_d_n5, assign33220_e37903_d_n6, assign33220_e37903_d_n7, assign33220_e37903_d_n8, assign33220_e37903_d_n9, assign33220_e37903_d_n10, assign33220_e37903_d_n11, assign33220_e37903_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign33220_e37905;
        locals.var_dnm_dn0 = assign33220_e37905_d_n0;
        locals.var_dnm_dn2 = assign33220_e37905_d_n2;
        locals.var_dnm_dn4 = assign33220_e37905_d_n4;
        locals.var_dnm_dn5 = assign33220_e37905_d_n5;
        locals.var_dnm_dn6 = assign33220_e37905_d_n6;
        locals.var_dnm_dn7 = assign33220_e37905_d_n7;
        locals.var_dnm_dn8 = assign33220_e37905_d_n8;
        locals.var_dnm_dn9 = assign33220_e37905_d_n9;
        locals.var_dnm_dn10 = assign33220_e37905_d_n10;
        locals.var_dnm_dn11 = assign33220_e37905_d_n11;
        locals.var_dnm_dn14 = assign33220_e37905_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign33230_e37921, assign33230_e37921_d_n0, assign33230_e37921_d_n2, assign33230_e37921_d_n4, assign33230_e37921_d_n5, assign33230_e37921_d_n6, assign33230_e37921_d_n7, assign33230_e37921_d_n8, assign33230_e37921_d_n9, assign33230_e37921_d_n10, assign33230_e37921_d_n11, assign33230_e37921_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard784 != 0.0)) {
        let assign33230_e37919: f64 = (1.0 / locals.var_dnm);
        (assign33230_e37919, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign33230_e37921;
        locals.var_dnm_dn0 = assign33230_e37921_d_n0;
        locals.var_dnm_dn2 = assign33230_e37921_d_n2;
        locals.var_dnm_dn4 = assign33230_e37921_d_n4;
        locals.var_dnm_dn5 = assign33230_e37921_d_n5;
        locals.var_dnm_dn6 = assign33230_e37921_d_n6;
        locals.var_dnm_dn7 = assign33230_e37921_d_n7;
        locals.var_dnm_dn8 = assign33230_e37921_d_n8;
        locals.var_dnm_dn9 = assign33230_e37921_d_n9;
        locals.var_dnm_dn10 = assign33230_e37921_d_n10;
        locals.var_dnm_dn11 = assign33230_e37921_d_n11;
        locals.var_dnm_dn14 = assign33230_e37921_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign33240_e37939, assign33240_e37939_d_n0, assign33240_e37939_d_n2, assign33240_e37939_d_n4, assign33240_e37939_d_n5, assign33240_e37939_d_n6, assign33240_e37939_d_n7, assign33240_e37939_d_n8, assign33240_e37939_d_n9, assign33240_e37939_d_n10, assign33240_e37939_d_n11, assign33240_e37939_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard784 != 0.0)) {
        let assign33240_e37935: f64 = (locals.var_tmf1 * 0.1);
        let assign33240_e37937: f64 = (assign33240_e37935 * locals.var_dnm);
        (assign33240_e37937, (((locals.var_tmf1_dn0 * 0.1) * locals.var_dnm) + (assign33240_e37935 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.1) * locals.var_dnm) + (assign33240_e37935 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.1) * locals.var_dnm) + (assign33240_e37935 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.1) * locals.var_dnm) + (assign33240_e37935 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.1) * locals.var_dnm) + (assign33240_e37935 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.1) * locals.var_dnm) + (assign33240_e37935 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.1) * locals.var_dnm) + (assign33240_e37935 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.1) * locals.var_dnm) + (assign33240_e37935 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.1) * locals.var_dnm) + (assign33240_e37935 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.1) * locals.var_dnm) + (assign33240_e37935 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.1) * locals.var_dnm) + (assign33240_e37935 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign33240_e37939;
        locals.var_tmf0_dn0 = assign33240_e37939_d_n0;
        locals.var_tmf0_dn2 = assign33240_e37939_d_n2;
        locals.var_tmf0_dn4 = assign33240_e37939_d_n4;
        locals.var_tmf0_dn5 = assign33240_e37939_d_n5;
        locals.var_tmf0_dn6 = assign33240_e37939_d_n6;
        locals.var_tmf0_dn7 = assign33240_e37939_d_n7;
        locals.var_tmf0_dn8 = assign33240_e37939_d_n8;
        locals.var_tmf0_dn9 = assign33240_e37939_d_n9;
        locals.var_tmf0_dn10 = assign33240_e37939_d_n10;
        locals.var_tmf0_dn11 = assign33240_e37939_d_n11;
        locals.var_tmf0_dn14 = assign33240_e37939_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign33250_e37959, assign33250_e37959_d_n0, assign33250_e37959_d_n2, assign33250_e37959_d_n4, assign33250_e37959_d_n5, assign33250_e37959_d_n6, assign33250_e37959_d_n7, assign33250_e37959_d_n8, assign33250_e37959_d_n9, assign33250_e37959_d_n10, assign33250_e37959_d_n11, assign33250_e37959_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard784 != 0.0)) {
        let assign33250_e37953: f64 = (0.1 * locals.var_xmp);
        let assign33250_e37955: f64 = (assign33250_e37953 * locals.var_dnm);
        let assign33250_e37957: f64 = (assign33250_e37955 / locals.var_arg);
        (assign33250_e37957, ((((((0.1 * locals.var_xmp_dn0) * locals.var_dnm) + (assign33250_e37953 * locals.var_dnm_dn0)) * locals.var_arg) - (assign33250_e37955 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn2) * locals.var_dnm) + (assign33250_e37953 * locals.var_dnm_dn2)) * locals.var_arg) - (assign33250_e37955 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn4) * locals.var_dnm) + (assign33250_e37953 * locals.var_dnm_dn4)) * locals.var_arg) - (assign33250_e37955 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn5) * locals.var_dnm) + (assign33250_e37953 * locals.var_dnm_dn5)) * locals.var_arg) - (assign33250_e37955 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn6) * locals.var_dnm) + (assign33250_e37953 * locals.var_dnm_dn6)) * locals.var_arg) - (assign33250_e37955 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn7) * locals.var_dnm) + (assign33250_e37953 * locals.var_dnm_dn7)) * locals.var_arg) - (assign33250_e37955 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn8) * locals.var_dnm) + (assign33250_e37953 * locals.var_dnm_dn8)) * locals.var_arg) - (assign33250_e37955 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn9) * locals.var_dnm) + (assign33250_e37953 * locals.var_dnm_dn9)) * locals.var_arg) - (assign33250_e37955 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn10) * locals.var_dnm) + (assign33250_e37953 * locals.var_dnm_dn10)) * locals.var_arg) - (assign33250_e37955 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn11) * locals.var_dnm) + (assign33250_e37953 * locals.var_dnm_dn11)) * locals.var_arg) - (assign33250_e37955 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn14) * locals.var_dnm) + (assign33250_e37953 * locals.var_dnm_dn14)) * locals.var_arg) - (assign33250_e37955 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign33250_e37959;
        locals.var_t0_dn0 = assign33250_e37959_d_n0;
        locals.var_t0_dn2 = assign33250_e37959_d_n2;
        locals.var_t0_dn4 = assign33250_e37959_d_n4;
        locals.var_t0_dn5 = assign33250_e37959_d_n5;
        locals.var_t0_dn6 = assign33250_e37959_d_n6;
        locals.var_t0_dn7 = assign33250_e37959_d_n7;
        locals.var_t0_dn8 = assign33250_e37959_d_n8;
        locals.var_t0_dn9 = assign33250_e37959_d_n9;
        locals.var_t0_dn10 = assign33250_e37959_d_n10;
        locals.var_t0_dn11 = assign33250_e37959_d_n11;
        locals.var_t0_dn14 = assign33250_e37959_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign33260_e37977, assign33260_e37977_d_n0, assign33260_e37977_d_n2, assign33260_e37977_d_n4, assign33260_e37977_d_n5, assign33260_e37977_d_n6, assign33260_e37977_d_n7, assign33260_e37977_d_n8, assign33260_e37977_d_n9, assign33260_e37977_d_n10, assign33260_e37977_d_n11, assign33260_e37977_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard784 != 0.0)) {
        let assign33260_e37973: f64 = 0.1;
        let assign33260_e37975: f64 = (assign33260_e37973 - locals.var_tmf0);
        (assign33260_e37975, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign33260_e37977;
        locals.var_t2_dn0 = assign33260_e37977_d_n0;
        locals.var_t2_dn2 = assign33260_e37977_d_n2;
        locals.var_t2_dn4 = assign33260_e37977_d_n4;
        locals.var_t2_dn5 = assign33260_e37977_d_n5;
        locals.var_t2_dn6 = assign33260_e37977_d_n6;
        locals.var_t2_dn7 = assign33260_e37977_d_n7;
        locals.var_t2_dn8 = assign33260_e37977_d_n8;
        locals.var_t2_dn9 = assign33260_e37977_d_n9;
        locals.var_t2_dn10 = assign33260_e37977_d_n10;
        locals.var_t2_dn11 = assign33260_e37977_d_n11;
        locals.var_t2_dn14 = assign33260_e37977_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign33270_e37991, assign33270_e37991_d_n0, assign33270_e37991_d_n2, assign33270_e37991_d_n4, assign33270_e37991_d_n5, assign33270_e37991_d_n6, assign33270_e37991_d_n7, assign33270_e37991_d_n8, assign33270_e37991_d_n9, assign33270_e37991_d_n10, assign33270_e37991_d_n11, assign33270_e37991_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard784 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign33270_e37991;
        locals.var_t0_dn0 = assign33270_e37991_d_n0;
        locals.var_t0_dn2 = assign33270_e37991_d_n2;
        locals.var_t0_dn4 = assign33270_e37991_d_n4;
        locals.var_t0_dn5 = assign33270_e37991_d_n5;
        locals.var_t0_dn6 = assign33270_e37991_d_n6;
        locals.var_t0_dn7 = assign33270_e37991_d_n7;
        locals.var_t0_dn8 = assign33270_e37991_d_n8;
        locals.var_t0_dn9 = assign33270_e37991_d_n9;
        locals.var_t0_dn10 = assign33270_e37991_d_n10;
        locals.var_t0_dn11 = assign33270_e37991_d_n11;
        locals.var_t0_dn14 = assign33270_e37991_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign33280_e38006, assign33280_e38006_d_n0, assign33280_e38006_d_n2, assign33280_e38006_d_n4, assign33280_e38006_d_n5, assign33280_e38006_d_n6, assign33280_e38006_d_n7, assign33280_e38006_d_n8, assign33280_e38006_d_n9, assign33280_e38006_d_n10, assign33280_e38006_d_n11, assign33280_e38006_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard784 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign33280_e38006;
        locals.var_t2_dn0 = assign33280_e38006_d_n0;
        locals.var_t2_dn2 = assign33280_e38006_d_n2;
        locals.var_t2_dn4 = assign33280_e38006_d_n4;
        locals.var_t2_dn5 = assign33280_e38006_d_n5;
        locals.var_t2_dn6 = assign33280_e38006_d_n6;
        locals.var_t2_dn7 = assign33280_e38006_d_n7;
        locals.var_t2_dn8 = assign33280_e38006_d_n8;
        locals.var_t2_dn9 = assign33280_e38006_d_n9;
        locals.var_t2_dn10 = assign33280_e38006_d_n10;
        locals.var_t2_dn11 = assign33280_e38006_d_n11;
        locals.var_t2_dn14 = assign33280_e38006_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign33290_e38021, assign33290_e38021_d_n0, assign33290_e38021_d_n2, assign33290_e38021_d_n4, assign33290_e38021_d_n5, assign33290_e38021_d_n6, assign33290_e38021_d_n7, assign33290_e38021_d_n8, assign33290_e38021_d_n9, assign33290_e38021_d_n10, assign33290_e38021_d_n11, assign33290_e38021_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard784 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign33290_e38021;
        locals.var_t0_dn0 = assign33290_e38021_d_n0;
        locals.var_t0_dn2 = assign33290_e38021_d_n2;
        locals.var_t0_dn4 = assign33290_e38021_d_n4;
        locals.var_t0_dn5 = assign33290_e38021_d_n5;
        locals.var_t0_dn6 = assign33290_e38021_d_n6;
        locals.var_t0_dn7 = assign33290_e38021_d_n7;
        locals.var_t0_dn8 = assign33290_e38021_d_n8;
        locals.var_t0_dn9 = assign33290_e38021_d_n9;
        locals.var_t0_dn10 = assign33290_e38021_d_n10;
        locals.var_t0_dn11 = assign33290_e38021_d_n11;
        locals.var_t0_dn14 = assign33290_e38021_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign33300_e38036, assign33300_e38036_d_n0, assign33300_e38036_d_n2, assign33300_e38036_d_n4, assign33300_e38036_d_n5, assign33300_e38036_d_n6, assign33300_e38036_d_n7, assign33300_e38036_d_n8, assign33300_e38036_d_n9, assign33300_e38036_d_n10, assign33300_e38036_d_n11, assign33300_e38036_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) {
        let assign33300_e38033: f64 = (locals.var_c_2esipq_ndepm * locals.var_t2);
        let assign33300_e38034: f64 = (assign33300_e38033).sqrt();
        (assign33300_e38034, (((locals.var_c_2esipq_ndepm_dn0 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn0)) / (2.0 * assign33300_e38034)), (((locals.var_c_2esipq_ndepm_dn2 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn2)) / (2.0 * assign33300_e38034)), (((locals.var_c_2esipq_ndepm_dn4 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn4)) / (2.0 * assign33300_e38034)), (((locals.var_c_2esipq_ndepm_dn5 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn5)) / (2.0 * assign33300_e38034)), (((locals.var_c_2esipq_ndepm_dn6 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn6)) / (2.0 * assign33300_e38034)), (((locals.var_c_2esipq_ndepm_dn7 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn7)) / (2.0 * assign33300_e38034)), (((locals.var_c_2esipq_ndepm_dn8 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn8)) / (2.0 * assign33300_e38034)), (((locals.var_c_2esipq_ndepm_dn9 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn9)) / (2.0 * assign33300_e38034)), (((locals.var_c_2esipq_ndepm_dn10 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn10)) / (2.0 * assign33300_e38034)), (((locals.var_c_2esipq_ndepm_dn11 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn11)) / (2.0 * assign33300_e38034)), (((locals.var_c_2esipq_ndepm_dn14 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn14)) / (2.0 * assign33300_e38034)),)
    } else {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn11, locals.var_w_bl_dn14,)
    }
};
        locals.var_w_bl = assign33300_e38036;
        locals.var_w_bl_dn0 = assign33300_e38036_d_n0;
        locals.var_w_bl_dn2 = assign33300_e38036_d_n2;
        locals.var_w_bl_dn4 = assign33300_e38036_d_n4;
        locals.var_w_bl_dn5 = assign33300_e38036_d_n5;
        locals.var_w_bl_dn6 = assign33300_e38036_d_n6;
        locals.var_w_bl_dn7 = assign33300_e38036_d_n7;
        locals.var_w_bl_dn8 = assign33300_e38036_d_n8;
        locals.var_w_bl_dn9 = assign33300_e38036_d_n9;
        locals.var_w_bl_dn10 = assign33300_e38036_d_n10;
        locals.var_w_bl_dn11 = assign33300_e38036_d_n11;
        locals.var_w_bl_dn14 = assign33300_e38036_d_n14;
        locals.var_w_bl_rv = 0.0;

        let assign33310_e38040: f64 = (locals.var_uc_depthn - 1e-8);
        let assign33310_e38045: f64 = if ((locals.var_w_bl > assign33310_e38040) && (1e-8 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard790 = assign33310_e38045;
        locals.var_guard790_rv = 0.0;

        let (assign33320_e38063, assign33320_e38063_d_n0, assign33320_e38063_d_n2, assign33320_e38063_d_n4, assign33320_e38063_d_n5, assign33320_e38063_d_n6, assign33320_e38063_d_n7, assign33320_e38063_d_n8, assign33320_e38063_d_n9, assign33320_e38063_d_n10, assign33320_e38063_d_n11, assign33320_e38063_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard790 != 0.0)) {
        let assign33320_e38059: f64 = (locals.var_w_bl - locals.var_uc_depthn);
        let assign33320_e38061: f64 = (assign33320_e38059 + 1e-8);
        (assign33320_e38061, (locals.var_w_bl_dn0 - locals.var_uc_depthn_dn0), (locals.var_w_bl_dn2 - locals.var_uc_depthn_dn2), (locals.var_w_bl_dn4 - locals.var_uc_depthn_dn4), (locals.var_w_bl_dn5 - locals.var_uc_depthn_dn5), (locals.var_w_bl_dn6 - locals.var_uc_depthn_dn6), (locals.var_w_bl_dn7 - locals.var_uc_depthn_dn7), (locals.var_w_bl_dn8 - locals.var_uc_depthn_dn8), (locals.var_w_bl_dn9 - locals.var_uc_depthn_dn9), (locals.var_w_bl_dn10 - locals.var_uc_depthn_dn10), (locals.var_w_bl_dn11 - locals.var_uc_depthn_dn11), (locals.var_w_bl_dn14 - locals.var_uc_depthn_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign33320_e38063;
        locals.var_tmf1_dn0 = assign33320_e38063_d_n0;
        locals.var_tmf1_dn2 = assign33320_e38063_d_n2;
        locals.var_tmf1_dn4 = assign33320_e38063_d_n4;
        locals.var_tmf1_dn5 = assign33320_e38063_d_n5;
        locals.var_tmf1_dn6 = assign33320_e38063_d_n6;
        locals.var_tmf1_dn7 = assign33320_e38063_d_n7;
        locals.var_tmf1_dn8 = assign33320_e38063_d_n8;
        locals.var_tmf1_dn9 = assign33320_e38063_d_n9;
        locals.var_tmf1_dn10 = assign33320_e38063_d_n10;
        locals.var_tmf1_dn11 = assign33320_e38063_d_n11;
        locals.var_tmf1_dn14 = assign33320_e38063_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign33330_e38079, assign33330_e38079_d_n0, assign33330_e38079_d_n2, assign33330_e38079_d_n4, assign33330_e38079_d_n5, assign33330_e38079_d_n6, assign33330_e38079_d_n7, assign33330_e38079_d_n8, assign33330_e38079_d_n9, assign33330_e38079_d_n10, assign33330_e38079_d_n11, assign33330_e38079_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard790 != 0.0)) {
        let assign33330_e38077: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign33330_e38077, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign33330_e38079;
        locals.var_x2_dn0 = assign33330_e38079_d_n0;
        locals.var_x2_dn2 = assign33330_e38079_d_n2;
        locals.var_x2_dn4 = assign33330_e38079_d_n4;
        locals.var_x2_dn5 = assign33330_e38079_d_n5;
        locals.var_x2_dn6 = assign33330_e38079_d_n6;
        locals.var_x2_dn7 = assign33330_e38079_d_n7;
        locals.var_x2_dn8 = assign33330_e38079_d_n8;
        locals.var_x2_dn9 = assign33330_e38079_d_n9;
        locals.var_x2_dn10 = assign33330_e38079_d_n10;
        locals.var_x2_dn11 = assign33330_e38079_d_n11;
        locals.var_x2_dn14 = assign33330_e38079_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign33340_e38095, assign33340_e38095_d_n0, assign33340_e38095_d_n2, assign33340_e38095_d_n4, assign33340_e38095_d_n5, assign33340_e38095_d_n6, assign33340_e38095_d_n7, assign33340_e38095_d_n8, assign33340_e38095_d_n9, assign33340_e38095_d_n10, assign33340_e38095_d_n11, assign33340_e38095_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard790 != 0.0)) {
        let assign33340_e38093: f64 = (1e-8 * 1e-8);
        (assign33340_e38093, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign33340_e38095;
        locals.var_xmax2_dn0 = assign33340_e38095_d_n0;
        locals.var_xmax2_dn2 = assign33340_e38095_d_n2;
        locals.var_xmax2_dn4 = assign33340_e38095_d_n4;
        locals.var_xmax2_dn5 = assign33340_e38095_d_n5;
        locals.var_xmax2_dn6 = assign33340_e38095_d_n6;
        locals.var_xmax2_dn7 = assign33340_e38095_d_n7;
        locals.var_xmax2_dn8 = assign33340_e38095_d_n8;
        locals.var_xmax2_dn9 = assign33340_e38095_d_n9;
        locals.var_xmax2_dn10 = assign33340_e38095_d_n10;
        locals.var_xmax2_dn11 = assign33340_e38095_d_n11;
        locals.var_xmax2_dn14 = assign33340_e38095_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign33350_e38109, assign33350_e38109_d_n0, assign33350_e38109_d_n2, assign33350_e38109_d_n4, assign33350_e38109_d_n5, assign33350_e38109_d_n6, assign33350_e38109_d_n7, assign33350_e38109_d_n8, assign33350_e38109_d_n9, assign33350_e38109_d_n10, assign33350_e38109_d_n11, assign33350_e38109_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard790 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign33350_e38109;
        locals.var_xp_dn0 = assign33350_e38109_d_n0;
        locals.var_xp_dn2 = assign33350_e38109_d_n2;
        locals.var_xp_dn4 = assign33350_e38109_d_n4;
        locals.var_xp_dn5 = assign33350_e38109_d_n5;
        locals.var_xp_dn6 = assign33350_e38109_d_n6;
        locals.var_xp_dn7 = assign33350_e38109_d_n7;
        locals.var_xp_dn8 = assign33350_e38109_d_n8;
        locals.var_xp_dn9 = assign33350_e38109_d_n9;
        locals.var_xp_dn10 = assign33350_e38109_d_n10;
        locals.var_xp_dn11 = assign33350_e38109_d_n11;
        locals.var_xp_dn14 = assign33350_e38109_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign33360_e38123, assign33360_e38123_d_n0, assign33360_e38123_d_n2, assign33360_e38123_d_n4, assign33360_e38123_d_n5, assign33360_e38123_d_n6, assign33360_e38123_d_n7, assign33360_e38123_d_n8, assign33360_e38123_d_n9, assign33360_e38123_d_n10, assign33360_e38123_d_n11, assign33360_e38123_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard790 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign33360_e38123;
        locals.var_xmp_dn0 = assign33360_e38123_d_n0;
        locals.var_xmp_dn2 = assign33360_e38123_d_n2;
        locals.var_xmp_dn4 = assign33360_e38123_d_n4;
        locals.var_xmp_dn5 = assign33360_e38123_d_n5;
        locals.var_xmp_dn6 = assign33360_e38123_d_n6;
        locals.var_xmp_dn7 = assign33360_e38123_d_n7;
        locals.var_xmp_dn8 = assign33360_e38123_d_n8;
        locals.var_xmp_dn9 = assign33360_e38123_d_n9;
        locals.var_xmp_dn10 = assign33360_e38123_d_n10;
        locals.var_xmp_dn11 = assign33360_e38123_d_n11;
        locals.var_xmp_dn14 = assign33360_e38123_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign33370_e38137,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard790 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign33370_e38137;
        locals.var_m0_rv = 0.0;

        let (assign33380_e38151,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard790 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33380_e38151;
        locals.var_mm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_108(
        locals: &mut StampLocals,
    ) {
        let (assign33390_e38165, assign33390_e38165_d_n0, assign33390_e38165_d_n2, assign33390_e38165_d_n4, assign33390_e38165_d_n5, assign33390_e38165_d_n6, assign33390_e38165_d_n7, assign33390_e38165_d_n8, assign33390_e38165_d_n9, assign33390_e38165_d_n10, assign33390_e38165_d_n11, assign33390_e38165_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard790 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign33390_e38165;
        locals.var_arg_dn0 = assign33390_e38165_d_n0;
        locals.var_arg_dn2 = assign33390_e38165_d_n2;
        locals.var_arg_dn4 = assign33390_e38165_d_n4;
        locals.var_arg_dn5 = assign33390_e38165_d_n5;
        locals.var_arg_dn6 = assign33390_e38165_d_n6;
        locals.var_arg_dn7 = assign33390_e38165_d_n7;
        locals.var_arg_dn8 = assign33390_e38165_d_n8;
        locals.var_arg_dn9 = assign33390_e38165_d_n9;
        locals.var_arg_dn10 = assign33390_e38165_d_n10;
        locals.var_arg_dn11 = assign33390_e38165_d_n11;
        locals.var_arg_dn14 = assign33390_e38165_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign33400_e38179, assign33400_e38179_d_n0, assign33400_e38179_d_n2, assign33400_e38179_d_n4, assign33400_e38179_d_n5, assign33400_e38179_d_n6, assign33400_e38179_d_n7, assign33400_e38179_d_n8, assign33400_e38179_d_n9, assign33400_e38179_d_n10, assign33400_e38179_d_n11, assign33400_e38179_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard790 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign33400_e38179;
        locals.var_dnm_dn0 = assign33400_e38179_d_n0;
        locals.var_dnm_dn2 = assign33400_e38179_d_n2;
        locals.var_dnm_dn4 = assign33400_e38179_d_n4;
        locals.var_dnm_dn5 = assign33400_e38179_d_n5;
        locals.var_dnm_dn6 = assign33400_e38179_d_n6;
        locals.var_dnm_dn7 = assign33400_e38179_d_n7;
        locals.var_dnm_dn8 = assign33400_e38179_d_n8;
        locals.var_dnm_dn9 = assign33400_e38179_d_n9;
        locals.var_dnm_dn10 = assign33400_e38179_d_n10;
        locals.var_dnm_dn11 = assign33400_e38179_d_n11;
        locals.var_dnm_dn14 = assign33400_e38179_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign33410_e38195, assign33410_e38195_d_n0, assign33410_e38195_d_n2, assign33410_e38195_d_n4, assign33410_e38195_d_n5, assign33410_e38195_d_n6, assign33410_e38195_d_n7, assign33410_e38195_d_n8, assign33410_e38195_d_n9, assign33410_e38195_d_n10, assign33410_e38195_d_n11, assign33410_e38195_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard790 != 0.0)) {
        let assign33410_e38193: f64 = (locals.var_xp * locals.var_x2);
        (assign33410_e38193, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign33410_e38195;
        locals.var_xp_dn0 = assign33410_e38195_d_n0;
        locals.var_xp_dn2 = assign33410_e38195_d_n2;
        locals.var_xp_dn4 = assign33410_e38195_d_n4;
        locals.var_xp_dn5 = assign33410_e38195_d_n5;
        locals.var_xp_dn6 = assign33410_e38195_d_n6;
        locals.var_xp_dn7 = assign33410_e38195_d_n7;
        locals.var_xp_dn8 = assign33410_e38195_d_n8;
        locals.var_xp_dn9 = assign33410_e38195_d_n9;
        locals.var_xp_dn10 = assign33410_e38195_d_n10;
        locals.var_xp_dn11 = assign33410_e38195_d_n11;
        locals.var_xp_dn14 = assign33410_e38195_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign33420_e38211, assign33420_e38211_d_n0, assign33420_e38211_d_n2, assign33420_e38211_d_n4, assign33420_e38211_d_n5, assign33420_e38211_d_n6, assign33420_e38211_d_n7, assign33420_e38211_d_n8, assign33420_e38211_d_n9, assign33420_e38211_d_n10, assign33420_e38211_d_n11, assign33420_e38211_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard790 != 0.0)) {
        let assign33420_e38209: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign33420_e38209, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign33420_e38211;
        locals.var_xmp_dn0 = assign33420_e38211_d_n0;
        locals.var_xmp_dn2 = assign33420_e38211_d_n2;
        locals.var_xmp_dn4 = assign33420_e38211_d_n4;
        locals.var_xmp_dn5 = assign33420_e38211_d_n5;
        locals.var_xmp_dn6 = assign33420_e38211_d_n6;
        locals.var_xmp_dn7 = assign33420_e38211_d_n7;
        locals.var_xmp_dn8 = assign33420_e38211_d_n8;
        locals.var_xmp_dn9 = assign33420_e38211_d_n9;
        locals.var_xmp_dn10 = assign33420_e38211_d_n10;
        locals.var_xmp_dn11 = assign33420_e38211_d_n11;
        locals.var_xmp_dn14 = assign33420_e38211_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign33430_e38227, assign33430_e38227_d_n0, assign33430_e38227_d_n2, assign33430_e38227_d_n4, assign33430_e38227_d_n5, assign33430_e38227_d_n6, assign33430_e38227_d_n7, assign33430_e38227_d_n8, assign33430_e38227_d_n9, assign33430_e38227_d_n10, assign33430_e38227_d_n11, assign33430_e38227_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard790 != 0.0)) {
        let assign33430_e38225: f64 = (locals.var_xp * locals.var_x2);
        (assign33430_e38225, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign33430_e38227;
        locals.var_xp_dn0 = assign33430_e38227_d_n0;
        locals.var_xp_dn2 = assign33430_e38227_d_n2;
        locals.var_xp_dn4 = assign33430_e38227_d_n4;
        locals.var_xp_dn5 = assign33430_e38227_d_n5;
        locals.var_xp_dn6 = assign33430_e38227_d_n6;
        locals.var_xp_dn7 = assign33430_e38227_d_n7;
        locals.var_xp_dn8 = assign33430_e38227_d_n8;
        locals.var_xp_dn9 = assign33430_e38227_d_n9;
        locals.var_xp_dn10 = assign33430_e38227_d_n10;
        locals.var_xp_dn11 = assign33430_e38227_d_n11;
        locals.var_xp_dn14 = assign33430_e38227_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign33440_e38243, assign33440_e38243_d_n0, assign33440_e38243_d_n2, assign33440_e38243_d_n4, assign33440_e38243_d_n5, assign33440_e38243_d_n6, assign33440_e38243_d_n7, assign33440_e38243_d_n8, assign33440_e38243_d_n9, assign33440_e38243_d_n10, assign33440_e38243_d_n11, assign33440_e38243_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard790 != 0.0)) {
        let assign33440_e38241: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign33440_e38241, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign33440_e38243;
        locals.var_xmp_dn0 = assign33440_e38243_d_n0;
        locals.var_xmp_dn2 = assign33440_e38243_d_n2;
        locals.var_xmp_dn4 = assign33440_e38243_d_n4;
        locals.var_xmp_dn5 = assign33440_e38243_d_n5;
        locals.var_xmp_dn6 = assign33440_e38243_d_n6;
        locals.var_xmp_dn7 = assign33440_e38243_d_n7;
        locals.var_xmp_dn8 = assign33440_e38243_d_n8;
        locals.var_xmp_dn9 = assign33440_e38243_d_n9;
        locals.var_xmp_dn10 = assign33440_e38243_d_n10;
        locals.var_xmp_dn11 = assign33440_e38243_d_n11;
        locals.var_xmp_dn14 = assign33440_e38243_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign33450_e38259, assign33450_e38259_d_n0, assign33450_e38259_d_n2, assign33450_e38259_d_n4, assign33450_e38259_d_n5, assign33450_e38259_d_n6, assign33450_e38259_d_n7, assign33450_e38259_d_n8, assign33450_e38259_d_n9, assign33450_e38259_d_n10, assign33450_e38259_d_n11, assign33450_e38259_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard790 != 0.0)) {
        let assign33450_e38257: f64 = (locals.var_xp + locals.var_xmp);
        (assign33450_e38257, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign33450_e38259;
        locals.var_arg_dn0 = assign33450_e38259_d_n0;
        locals.var_arg_dn2 = assign33450_e38259_d_n2;
        locals.var_arg_dn4 = assign33450_e38259_d_n4;
        locals.var_arg_dn5 = assign33450_e38259_d_n5;
        locals.var_arg_dn6 = assign33450_e38259_d_n6;
        locals.var_arg_dn7 = assign33450_e38259_d_n7;
        locals.var_arg_dn8 = assign33450_e38259_d_n8;
        locals.var_arg_dn9 = assign33450_e38259_d_n9;
        locals.var_arg_dn10 = assign33450_e38259_d_n10;
        locals.var_arg_dn11 = assign33450_e38259_d_n11;
        locals.var_arg_dn14 = assign33450_e38259_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign33460_e38273, assign33460_e38273_d_n0, assign33460_e38273_d_n2, assign33460_e38273_d_n4, assign33460_e38273_d_n5, assign33460_e38273_d_n6, assign33460_e38273_d_n7, assign33460_e38273_d_n8, assign33460_e38273_d_n9, assign33460_e38273_d_n10, assign33460_e38273_d_n11, assign33460_e38273_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard790 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign33460_e38273;
        locals.var_dnm_dn0 = assign33460_e38273_d_n0;
        locals.var_dnm_dn2 = assign33460_e38273_d_n2;
        locals.var_dnm_dn4 = assign33460_e38273_d_n4;
        locals.var_dnm_dn5 = assign33460_e38273_d_n5;
        locals.var_dnm_dn6 = assign33460_e38273_d_n6;
        locals.var_dnm_dn7 = assign33460_e38273_d_n7;
        locals.var_dnm_dn8 = assign33460_e38273_d_n8;
        locals.var_dnm_dn9 = assign33460_e38273_d_n9;
        locals.var_dnm_dn10 = assign33460_e38273_d_n10;
        locals.var_dnm_dn11 = assign33460_e38273_d_n11;
        locals.var_dnm_dn14 = assign33460_e38273_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign33470_e38288: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard791 = assign33470_e38288;
        locals.var_guard791_rv = 0.0;

        let assign33480_e38291: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard792 = assign33480_e38291;
        locals.var_guard792_rv = 0.0;

        let (assign33490_e38309,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard790 != 0.0)) && (locals.var_guard791 != 0.0)) && (locals.var_guard792 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33490_e38309;
        locals.var_mm_rv = 0.0;

        let assign33500_e38312: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard793 = assign33500_e38312;
        locals.var_guard793_rv = 0.0;

        let (assign33510_e38333,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard790 != 0.0)) && (locals.var_guard791 != 0.0)) && (locals.var_guard792 == 0.0)) && (locals.var_guard793 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33510_e38333;
        locals.var_mm_rv = 0.0;

        let assign33520_e38336: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard794 = assign33520_e38336;
        locals.var_guard794_rv = 0.0;

        let (assign33530_e38360,) = {
    if (((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard790 != 0.0)) && (locals.var_guard791 != 0.0)) && (locals.var_guard792 == 0.0)) && (locals.var_guard793 == 0.0)) && (locals.var_guard794 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33530_e38360;
        locals.var_mm_rv = 0.0;

        let assign33540_e38363: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard795 = assign33540_e38363;
        locals.var_guard795_rv = 0.0;

        let (assign33550_e38390,) = {
    if ((((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard790 != 0.0)) && (locals.var_guard791 != 0.0)) && (locals.var_guard792 == 0.0)) && (locals.var_guard793 == 0.0)) && (locals.var_guard794 == 0.0)) && (locals.var_guard795 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33550_e38390;
        locals.var_mm_rv = 0.0;

        let (assign33560_e38406,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard790 != 0.0)) && (locals.var_guard791 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign33560_e38406;
        locals.var_m0_rv = 0.0;

        let mut assign33570_loop_guard: usize = 0;
        while {
            let assign33570_cond_e38423: f64 = if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard790 != 0.0)) && (locals.var_guard791 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign33570_cond_e38423 != 0.0
        } {
            assign33570_loop_guard += 1;
            assert!(assign33570_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign33570_body0_e38440, assign33570_body0_e38440_d_n0, assign33570_body0_e38440_d_n2, assign33570_body0_e38440_d_n4, assign33570_body0_e38440_d_n5, assign33570_body0_e38440_d_n6, assign33570_body0_e38440_d_n7, assign33570_body0_e38440_d_n8, assign33570_body0_e38440_d_n9, assign33570_body0_e38440_d_n10, assign33570_body0_e38440_d_n11, assign33570_body0_e38440_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard790 != 0.0)) && (locals.var_guard791 != 0.0)) {
        let assign33570_body0_e38438: f64 = (locals.var_dnm).sqrt();
        (assign33570_body0_e38438, (locals.var_dnm_dn0 / (2.0 * assign33570_body0_e38438)), (locals.var_dnm_dn2 / (2.0 * assign33570_body0_e38438)), (locals.var_dnm_dn4 / (2.0 * assign33570_body0_e38438)), (locals.var_dnm_dn5 / (2.0 * assign33570_body0_e38438)), (locals.var_dnm_dn6 / (2.0 * assign33570_body0_e38438)), (locals.var_dnm_dn7 / (2.0 * assign33570_body0_e38438)), (locals.var_dnm_dn8 / (2.0 * assign33570_body0_e38438)), (locals.var_dnm_dn9 / (2.0 * assign33570_body0_e38438)), (locals.var_dnm_dn10 / (2.0 * assign33570_body0_e38438)), (locals.var_dnm_dn11 / (2.0 * assign33570_body0_e38438)), (locals.var_dnm_dn14 / (2.0 * assign33570_body0_e38438)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign33570_body0_e38440;
            locals.var_dnm_dn0 = assign33570_body0_e38440_d_n0;
            locals.var_dnm_dn2 = assign33570_body0_e38440_d_n2;
            locals.var_dnm_dn4 = assign33570_body0_e38440_d_n4;
            locals.var_dnm_dn5 = assign33570_body0_e38440_d_n5;
            locals.var_dnm_dn6 = assign33570_body0_e38440_d_n6;
            locals.var_dnm_dn7 = assign33570_body0_e38440_d_n7;
            locals.var_dnm_dn8 = assign33570_body0_e38440_d_n8;
            locals.var_dnm_dn9 = assign33570_body0_e38440_d_n9;
            locals.var_dnm_dn10 = assign33570_body0_e38440_d_n10;
            locals.var_dnm_dn11 = assign33570_body0_e38440_d_n11;
            locals.var_dnm_dn14 = assign33570_body0_e38440_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign33570_body1_e38458,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard790 != 0.0)) && (locals.var_guard791 != 0.0)) {
        let assign33570_body1_e38456: f64 = (locals.var_m0 + 1.0);
        (assign33570_body1_e38456,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign33570_body1_e38458;
            locals.var_m0_rv = 0.0;
        }

        let (assign33580_e38486, assign33580_e38486_d_n0, assign33580_e38486_d_n2, assign33580_e38486_d_n4, assign33580_e38486_d_n5, assign33580_e38486_d_n6, assign33580_e38486_d_n7, assign33580_e38486_d_n8, assign33580_e38486_d_n9, assign33580_e38486_d_n10, assign33580_e38486_d_n11, assign33580_e38486_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard790 != 0.0)) && (locals.var_guard791 == 0.0)) {
        let (assign33580_e38484, assign33580_e38484_d_n0, assign33580_e38484_d_n2, assign33580_e38484_d_n4, assign33580_e38484_d_n5, assign33580_e38484_d_n6, assign33580_e38484_d_n7, assign33580_e38484_d_n8, assign33580_e38484_d_n9, assign33580_e38484_d_n10, assign33580_e38484_d_n11, assign33580_e38484_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign33580_e38481: f64 = (2.0 * 2.0);
                let assign33580_e38482: f64 = (1.0 / assign33580_e38481);
                let assign33580_e38483: f64 = (locals.var_dnm).powf(assign33580_e38482);
                (assign33580_e38483, if 0.0 == 0.0 && ((assign33580_e38482) as f64).is_finite() && ((assign33580_e38482) as f64).fract() == 0.0 { if assign33580_e38482 == 0.0 { 0.0 } else { (assign33580_e38482 * ((locals.var_dnm).powf(assign33580_e38482 - 1.0) * locals.var_dnm_dn0)) } } else { (assign33580_e38483 * (assign33580_e38482 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33580_e38482) as f64).is_finite() && ((assign33580_e38482) as f64).fract() == 0.0 { if assign33580_e38482 == 0.0 { 0.0 } else { (assign33580_e38482 * ((locals.var_dnm).powf(assign33580_e38482 - 1.0) * locals.var_dnm_dn2)) } } else { (assign33580_e38483 * (assign33580_e38482 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33580_e38482) as f64).is_finite() && ((assign33580_e38482) as f64).fract() == 0.0 { if assign33580_e38482 == 0.0 { 0.0 } else { (assign33580_e38482 * ((locals.var_dnm).powf(assign33580_e38482 - 1.0) * locals.var_dnm_dn4)) } } else { (assign33580_e38483 * (assign33580_e38482 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33580_e38482) as f64).is_finite() && ((assign33580_e38482) as f64).fract() == 0.0 { if assign33580_e38482 == 0.0 { 0.0 } else { (assign33580_e38482 * ((locals.var_dnm).powf(assign33580_e38482 - 1.0) * locals.var_dnm_dn5)) } } else { (assign33580_e38483 * (assign33580_e38482 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33580_e38482) as f64).is_finite() && ((assign33580_e38482) as f64).fract() == 0.0 { if assign33580_e38482 == 0.0 { 0.0 } else { (assign33580_e38482 * ((locals.var_dnm).powf(assign33580_e38482 - 1.0) * locals.var_dnm_dn6)) } } else { (assign33580_e38483 * (assign33580_e38482 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33580_e38482) as f64).is_finite() && ((assign33580_e38482) as f64).fract() == 0.0 { if assign33580_e38482 == 0.0 { 0.0 } else { (assign33580_e38482 * ((locals.var_dnm).powf(assign33580_e38482 - 1.0) * locals.var_dnm_dn7)) } } else { (assign33580_e38483 * (assign33580_e38482 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33580_e38482) as f64).is_finite() && ((assign33580_e38482) as f64).fract() == 0.0 { if assign33580_e38482 == 0.0 { 0.0 } else { (assign33580_e38482 * ((locals.var_dnm).powf(assign33580_e38482 - 1.0) * locals.var_dnm_dn8)) } } else { (assign33580_e38483 * (assign33580_e38482 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33580_e38482) as f64).is_finite() && ((assign33580_e38482) as f64).fract() == 0.0 { if assign33580_e38482 == 0.0 { 0.0 } else { (assign33580_e38482 * ((locals.var_dnm).powf(assign33580_e38482 - 1.0) * locals.var_dnm_dn9)) } } else { (assign33580_e38483 * (assign33580_e38482 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33580_e38482) as f64).is_finite() && ((assign33580_e38482) as f64).fract() == 0.0 { if assign33580_e38482 == 0.0 { 0.0 } else { (assign33580_e38482 * ((locals.var_dnm).powf(assign33580_e38482 - 1.0) * locals.var_dnm_dn10)) } } else { (assign33580_e38483 * (assign33580_e38482 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33580_e38482) as f64).is_finite() && ((assign33580_e38482) as f64).fract() == 0.0 { if assign33580_e38482 == 0.0 { 0.0 } else { (assign33580_e38482 * ((locals.var_dnm).powf(assign33580_e38482 - 1.0) * locals.var_dnm_dn11)) } } else { (assign33580_e38483 * (assign33580_e38482 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33580_e38482) as f64).is_finite() && ((assign33580_e38482) as f64).fract() == 0.0 { if assign33580_e38482 == 0.0 { 0.0 } else { (assign33580_e38482 * ((locals.var_dnm).powf(assign33580_e38482 - 1.0) * locals.var_dnm_dn14)) } } else { (assign33580_e38483 * (assign33580_e38482 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign33580_e38484, assign33580_e38484_d_n0, assign33580_e38484_d_n2, assign33580_e38484_d_n4, assign33580_e38484_d_n5, assign33580_e38484_d_n6, assign33580_e38484_d_n7, assign33580_e38484_d_n8, assign33580_e38484_d_n9, assign33580_e38484_d_n10, assign33580_e38484_d_n11, assign33580_e38484_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign33580_e38486;
        locals.var_dnm_dn0 = assign33580_e38486_d_n0;
        locals.var_dnm_dn2 = assign33580_e38486_d_n2;
        locals.var_dnm_dn4 = assign33580_e38486_d_n4;
        locals.var_dnm_dn5 = assign33580_e38486_d_n5;
        locals.var_dnm_dn6 = assign33580_e38486_d_n6;
        locals.var_dnm_dn7 = assign33580_e38486_d_n7;
        locals.var_dnm_dn8 = assign33580_e38486_d_n8;
        locals.var_dnm_dn9 = assign33580_e38486_d_n9;
        locals.var_dnm_dn10 = assign33580_e38486_d_n10;
        locals.var_dnm_dn11 = assign33580_e38486_d_n11;
        locals.var_dnm_dn14 = assign33580_e38486_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign33590_e38502, assign33590_e38502_d_n0, assign33590_e38502_d_n2, assign33590_e38502_d_n4, assign33590_e38502_d_n5, assign33590_e38502_d_n6, assign33590_e38502_d_n7, assign33590_e38502_d_n8, assign33590_e38502_d_n9, assign33590_e38502_d_n10, assign33590_e38502_d_n11, assign33590_e38502_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard790 != 0.0)) {
        let assign33590_e38500: f64 = (1.0 / locals.var_dnm);
        (assign33590_e38500, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign33590_e38502;
        locals.var_dnm_dn0 = assign33590_e38502_d_n0;
        locals.var_dnm_dn2 = assign33590_e38502_d_n2;
        locals.var_dnm_dn4 = assign33590_e38502_d_n4;
        locals.var_dnm_dn5 = assign33590_e38502_d_n5;
        locals.var_dnm_dn6 = assign33590_e38502_d_n6;
        locals.var_dnm_dn7 = assign33590_e38502_d_n7;
        locals.var_dnm_dn8 = assign33590_e38502_d_n8;
        locals.var_dnm_dn9 = assign33590_e38502_d_n9;
        locals.var_dnm_dn10 = assign33590_e38502_d_n10;
        locals.var_dnm_dn11 = assign33590_e38502_d_n11;
        locals.var_dnm_dn14 = assign33590_e38502_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign33600_e38520, assign33600_e38520_d_n0, assign33600_e38520_d_n2, assign33600_e38520_d_n4, assign33600_e38520_d_n5, assign33600_e38520_d_n6, assign33600_e38520_d_n7, assign33600_e38520_d_n8, assign33600_e38520_d_n9, assign33600_e38520_d_n10, assign33600_e38520_d_n11, assign33600_e38520_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard790 != 0.0)) {
        let assign33600_e38516: f64 = (locals.var_tmf1 * 1e-8);
        let assign33600_e38518: f64 = (assign33600_e38516 * locals.var_dnm);
        (assign33600_e38518, (((locals.var_tmf1_dn0 * 1e-8) * locals.var_dnm) + (assign33600_e38516 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-8) * locals.var_dnm) + (assign33600_e38516 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-8) * locals.var_dnm) + (assign33600_e38516 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-8) * locals.var_dnm) + (assign33600_e38516 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-8) * locals.var_dnm) + (assign33600_e38516 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-8) * locals.var_dnm) + (assign33600_e38516 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-8) * locals.var_dnm) + (assign33600_e38516 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-8) * locals.var_dnm) + (assign33600_e38516 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-8) * locals.var_dnm) + (assign33600_e38516 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1e-8) * locals.var_dnm) + (assign33600_e38516 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 1e-8) * locals.var_dnm) + (assign33600_e38516 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign33600_e38520;
        locals.var_tmf0_dn0 = assign33600_e38520_d_n0;
        locals.var_tmf0_dn2 = assign33600_e38520_d_n2;
        locals.var_tmf0_dn4 = assign33600_e38520_d_n4;
        locals.var_tmf0_dn5 = assign33600_e38520_d_n5;
        locals.var_tmf0_dn6 = assign33600_e38520_d_n6;
        locals.var_tmf0_dn7 = assign33600_e38520_d_n7;
        locals.var_tmf0_dn8 = assign33600_e38520_d_n8;
        locals.var_tmf0_dn9 = assign33600_e38520_d_n9;
        locals.var_tmf0_dn10 = assign33600_e38520_d_n10;
        locals.var_tmf0_dn11 = assign33600_e38520_d_n11;
        locals.var_tmf0_dn14 = assign33600_e38520_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign33610_e38540, assign33610_e38540_d_n0, assign33610_e38540_d_n2, assign33610_e38540_d_n4, assign33610_e38540_d_n5, assign33610_e38540_d_n6, assign33610_e38540_d_n7, assign33610_e38540_d_n8, assign33610_e38540_d_n9, assign33610_e38540_d_n10, assign33610_e38540_d_n11, assign33610_e38540_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard790 != 0.0)) {
        let assign33610_e38534: f64 = (1e-8 * locals.var_xmp);
        let assign33610_e38536: f64 = (assign33610_e38534 * locals.var_dnm);
        let assign33610_e38538: f64 = (assign33610_e38536 / locals.var_arg);
        (assign33610_e38538, ((((((1e-8 * locals.var_xmp_dn0) * locals.var_dnm) + (assign33610_e38534 * locals.var_dnm_dn0)) * locals.var_arg) - (assign33610_e38536 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn2) * locals.var_dnm) + (assign33610_e38534 * locals.var_dnm_dn2)) * locals.var_arg) - (assign33610_e38536 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn4) * locals.var_dnm) + (assign33610_e38534 * locals.var_dnm_dn4)) * locals.var_arg) - (assign33610_e38536 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn5) * locals.var_dnm) + (assign33610_e38534 * locals.var_dnm_dn5)) * locals.var_arg) - (assign33610_e38536 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn6) * locals.var_dnm) + (assign33610_e38534 * locals.var_dnm_dn6)) * locals.var_arg) - (assign33610_e38536 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn7) * locals.var_dnm) + (assign33610_e38534 * locals.var_dnm_dn7)) * locals.var_arg) - (assign33610_e38536 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn8) * locals.var_dnm) + (assign33610_e38534 * locals.var_dnm_dn8)) * locals.var_arg) - (assign33610_e38536 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn9) * locals.var_dnm) + (assign33610_e38534 * locals.var_dnm_dn9)) * locals.var_arg) - (assign33610_e38536 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn10) * locals.var_dnm) + (assign33610_e38534 * locals.var_dnm_dn10)) * locals.var_arg) - (assign33610_e38536 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn11) * locals.var_dnm) + (assign33610_e38534 * locals.var_dnm_dn11)) * locals.var_arg) - (assign33610_e38536 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn14) * locals.var_dnm) + (assign33610_e38534 * locals.var_dnm_dn14)) * locals.var_arg) - (assign33610_e38536 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign33610_e38540;
        locals.var_t3_dn0 = assign33610_e38540_d_n0;
        locals.var_t3_dn2 = assign33610_e38540_d_n2;
        locals.var_t3_dn4 = assign33610_e38540_d_n4;
        locals.var_t3_dn5 = assign33610_e38540_d_n5;
        locals.var_t3_dn6 = assign33610_e38540_d_n6;
        locals.var_t3_dn7 = assign33610_e38540_d_n7;
        locals.var_t3_dn8 = assign33610_e38540_d_n8;
        locals.var_t3_dn9 = assign33610_e38540_d_n9;
        locals.var_t3_dn10 = assign33610_e38540_d_n10;
        locals.var_t3_dn11 = assign33610_e38540_d_n11;
        locals.var_t3_dn14 = assign33610_e38540_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign33620_e38558, assign33620_e38558_d_n0, assign33620_e38558_d_n2, assign33620_e38558_d_n4, assign33620_e38558_d_n5, assign33620_e38558_d_n6, assign33620_e38558_d_n7, assign33620_e38558_d_n8, assign33620_e38558_d_n9, assign33620_e38558_d_n10, assign33620_e38558_d_n11, assign33620_e38558_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard790 != 0.0)) {
        let assign33620_e38554: f64 = (locals.var_uc_depthn - 1e-8);
        let assign33620_e38556: f64 = (assign33620_e38554 + locals.var_tmf0);
        (assign33620_e38556, (locals.var_uc_depthn_dn0 + locals.var_tmf0_dn0), (locals.var_uc_depthn_dn2 + locals.var_tmf0_dn2), (locals.var_uc_depthn_dn4 + locals.var_tmf0_dn4), (locals.var_uc_depthn_dn5 + locals.var_tmf0_dn5), (locals.var_uc_depthn_dn6 + locals.var_tmf0_dn6), (locals.var_uc_depthn_dn7 + locals.var_tmf0_dn7), (locals.var_uc_depthn_dn8 + locals.var_tmf0_dn8), (locals.var_uc_depthn_dn9 + locals.var_tmf0_dn9), (locals.var_uc_depthn_dn10 + locals.var_tmf0_dn10), (locals.var_uc_depthn_dn11 + locals.var_tmf0_dn11), (locals.var_uc_depthn_dn14 + locals.var_tmf0_dn14),)
    } else {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn11, locals.var_w_bl_dn14,)
    }
};
        locals.var_w_bl = assign33620_e38558;
        locals.var_w_bl_dn0 = assign33620_e38558_d_n0;
        locals.var_w_bl_dn2 = assign33620_e38558_d_n2;
        locals.var_w_bl_dn4 = assign33620_e38558_d_n4;
        locals.var_w_bl_dn5 = assign33620_e38558_d_n5;
        locals.var_w_bl_dn6 = assign33620_e38558_d_n6;
        locals.var_w_bl_dn7 = assign33620_e38558_d_n7;
        locals.var_w_bl_dn8 = assign33620_e38558_d_n8;
        locals.var_w_bl_dn9 = assign33620_e38558_d_n9;
        locals.var_w_bl_dn10 = assign33620_e38558_d_n10;
        locals.var_w_bl_dn11 = assign33620_e38558_d_n11;
        locals.var_w_bl_dn14 = assign33620_e38558_d_n14;
        locals.var_w_bl_rv = 0.0;

        let (assign33630_e38572, assign33630_e38572_d_n0, assign33630_e38572_d_n2, assign33630_e38572_d_n4, assign33630_e38572_d_n5, assign33630_e38572_d_n6, assign33630_e38572_d_n7, assign33630_e38572_d_n8, assign33630_e38572_d_n9, assign33630_e38572_d_n10, assign33630_e38572_d_n11, assign33630_e38572_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard790 != 0.0)) {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign33630_e38572;
        locals.var_t3_dn0 = assign33630_e38572_d_n0;
        locals.var_t3_dn2 = assign33630_e38572_d_n2;
        locals.var_t3_dn4 = assign33630_e38572_d_n4;
        locals.var_t3_dn5 = assign33630_e38572_d_n5;
        locals.var_t3_dn6 = assign33630_e38572_d_n6;
        locals.var_t3_dn7 = assign33630_e38572_d_n7;
        locals.var_t3_dn8 = assign33630_e38572_d_n8;
        locals.var_t3_dn9 = assign33630_e38572_d_n9;
        locals.var_t3_dn10 = assign33630_e38572_d_n10;
        locals.var_t3_dn11 = assign33630_e38572_d_n11;
        locals.var_t3_dn14 = assign33630_e38572_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign33640_e38587, assign33640_e38587_d_n0, assign33640_e38587_d_n2, assign33640_e38587_d_n4, assign33640_e38587_d_n5, assign33640_e38587_d_n6, assign33640_e38587_d_n7, assign33640_e38587_d_n8, assign33640_e38587_d_n9, assign33640_e38587_d_n10, assign33640_e38587_d_n11, assign33640_e38587_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard790 == 0.0)) {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn11, locals.var_w_bl_dn14,)
    } else {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn11, locals.var_w_bl_dn14,)
    }
};
        locals.var_w_bl = assign33640_e38587;
        locals.var_w_bl_dn0 = assign33640_e38587_d_n0;
        locals.var_w_bl_dn2 = assign33640_e38587_d_n2;
        locals.var_w_bl_dn4 = assign33640_e38587_d_n4;
        locals.var_w_bl_dn5 = assign33640_e38587_d_n5;
        locals.var_w_bl_dn6 = assign33640_e38587_d_n6;
        locals.var_w_bl_dn7 = assign33640_e38587_d_n7;
        locals.var_w_bl_dn8 = assign33640_e38587_d_n8;
        locals.var_w_bl_dn9 = assign33640_e38587_d_n9;
        locals.var_w_bl_dn10 = assign33640_e38587_d_n10;
        locals.var_w_bl_dn11 = assign33640_e38587_d_n11;
        locals.var_w_bl_dn14 = assign33640_e38587_d_n14;
        locals.var_w_bl_rv = 0.0;

        let (assign33650_e38602, assign33650_e38602_d_n0, assign33650_e38602_d_n2, assign33650_e38602_d_n4, assign33650_e38602_d_n5, assign33650_e38602_d_n6, assign33650_e38602_d_n7, assign33650_e38602_d_n8, assign33650_e38602_d_n9, assign33650_e38602_d_n10, assign33650_e38602_d_n11, assign33650_e38602_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) && (locals.var_guard790 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign33650_e38602;
        locals.var_t3_dn0 = assign33650_e38602_d_n0;
        locals.var_t3_dn2 = assign33650_e38602_d_n2;
        locals.var_t3_dn4 = assign33650_e38602_d_n4;
        locals.var_t3_dn5 = assign33650_e38602_d_n5;
        locals.var_t3_dn6 = assign33650_e38602_d_n6;
        locals.var_t3_dn7 = assign33650_e38602_d_n7;
        locals.var_t3_dn8 = assign33650_e38602_d_n8;
        locals.var_t3_dn9 = assign33650_e38602_d_n9;
        locals.var_t3_dn10 = assign33650_e38602_d_n10;
        locals.var_t3_dn11 = assign33650_e38602_d_n11;
        locals.var_t3_dn14 = assign33650_e38602_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign33660_e38621, assign33660_e38621_d_n0, assign33660_e38621_d_n2, assign33660_e38621_d_n4, assign33660_e38621_d_n5, assign33660_e38621_d_n6, assign33660_e38621_d_n7, assign33660_e38621_d_n8, assign33660_e38621_d_n9, assign33660_e38621_d_n10, assign33660_e38621_d_n11, assign33660_e38621_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) {
        let assign33660_e38615: f64 = (locals.var_phi_jl_dep - locals.var_vbscl__blk439);
        let assign33660_e38617: f64 = (assign33660_e38615 + locals.var_vbi_dep);
        let assign33660_e38618: f64 = (locals.var_c_2esipq_nsub * assign33660_e38617);
        let assign33660_e38619: f64 = (assign33660_e38618).sqrt();
        (assign33660_e38619, (((locals.var_c_2esipq_nsub_dn0 * assign33660_e38617) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn0 - locals.var_vbscl__blk439_dn0) + locals.var_vbi_dep_dn0))) / (2.0 * assign33660_e38619)), (((locals.var_c_2esipq_nsub_dn2 * assign33660_e38617) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn2 - locals.var_vbscl__blk439_dn2) + locals.var_vbi_dep_dn2))) / (2.0 * assign33660_e38619)), (((locals.var_c_2esipq_nsub_dn4 * assign33660_e38617) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn4 - locals.var_vbscl__blk439_dn4) + locals.var_vbi_dep_dn4))) / (2.0 * assign33660_e38619)), (((locals.var_c_2esipq_nsub_dn5 * assign33660_e38617) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn5 - locals.var_vbscl__blk439_dn5) + locals.var_vbi_dep_dn5))) / (2.0 * assign33660_e38619)), (((locals.var_c_2esipq_nsub_dn6 * assign33660_e38617) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn6 - locals.var_vbscl__blk439_dn6) + locals.var_vbi_dep_dn6))) / (2.0 * assign33660_e38619)), (((locals.var_c_2esipq_nsub_dn7 * assign33660_e38617) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn7 - locals.var_vbscl__blk439_dn7) + locals.var_vbi_dep_dn7))) / (2.0 * assign33660_e38619)), (((locals.var_c_2esipq_nsub_dn8 * assign33660_e38617) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn8 - locals.var_vbscl__blk439_dn8) + locals.var_vbi_dep_dn8))) / (2.0 * assign33660_e38619)), (((locals.var_c_2esipq_nsub_dn9 * assign33660_e38617) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn9 - locals.var_vbscl__blk439_dn9) + locals.var_vbi_dep_dn9))) / (2.0 * assign33660_e38619)), (((locals.var_c_2esipq_nsub_dn10 * assign33660_e38617) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn10 - locals.var_vbscl__blk439_dn10) + locals.var_vbi_dep_dn10))) / (2.0 * assign33660_e38619)), (((locals.var_c_2esipq_nsub_dn11 * assign33660_e38617) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn11 - locals.var_vbscl__blk439_dn11) + locals.var_vbi_dep_dn11))) / (2.0 * assign33660_e38619)), (((locals.var_c_2esipq_nsub_dn14 * assign33660_e38617) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn14 - locals.var_vbscl__blk439_dn14) + locals.var_vbi_dep_dn14))) / (2.0 * assign33660_e38619)),)
    } else {
        (locals.var_w_subl, locals.var_w_subl_dn0, locals.var_w_subl_dn2, locals.var_w_subl_dn4, locals.var_w_subl_dn5, locals.var_w_subl_dn6, locals.var_w_subl_dn7, locals.var_w_subl_dn8, locals.var_w_subl_dn9, locals.var_w_subl_dn10, locals.var_w_subl_dn11, locals.var_w_subl_dn14,)
    }
};
        locals.var_w_subl = assign33660_e38621;
        locals.var_w_subl_dn0 = assign33660_e38621_d_n0;
        locals.var_w_subl_dn2 = assign33660_e38621_d_n2;
        locals.var_w_subl_dn4 = assign33660_e38621_d_n4;
        locals.var_w_subl_dn5 = assign33660_e38621_d_n5;
        locals.var_w_subl_dn6 = assign33660_e38621_d_n6;
        locals.var_w_subl_dn7 = assign33660_e38621_d_n7;
        locals.var_w_subl_dn8 = assign33660_e38621_d_n8;
        locals.var_w_subl_dn9 = assign33660_e38621_d_n9;
        locals.var_w_subl_dn10 = assign33660_e38621_d_n10;
        locals.var_w_subl_dn11 = assign33660_e38621_d_n11;
        locals.var_w_subl_dn14 = assign33660_e38621_d_n14;
        locals.var_w_subl_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_109(
        locals: &mut StampLocals,
    ) {
        let (assign33670_e38635, assign33670_e38635_d_n0, assign33670_e38635_d_n2, assign33670_e38635_d_n4, assign33670_e38635_d_n5, assign33670_e38635_d_n6, assign33670_e38635_d_n7, assign33670_e38635_d_n8, assign33670_e38635_d_n9, assign33670_e38635_d_n10, assign33670_e38635_d_n11, assign33670_e38635_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) {
        let assign33670_e38633: f64 = (locals.var_w_bl * locals.var_q_ndepm);
        (assign33670_e38633, ((locals.var_w_bl_dn0 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn0)), ((locals.var_w_bl_dn2 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn2)), ((locals.var_w_bl_dn4 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn4)), ((locals.var_w_bl_dn5 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn5)), ((locals.var_w_bl_dn6 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn6)), ((locals.var_w_bl_dn7 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn7)), ((locals.var_w_bl_dn8 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn8)), ((locals.var_w_bl_dn9 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn9)), ((locals.var_w_bl_dn10 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn10)), ((locals.var_w_bl_dn11 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn11)), ((locals.var_w_bl_dn14 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn14)),)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn4, locals.var_q_bl_dep_dn5, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn8, locals.var_q_bl_dep_dn9, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn14,)
    }
};
        locals.var_q_bl_dep = assign33670_e38635;
        locals.var_q_bl_dep_dn0 = assign33670_e38635_d_n0;
        locals.var_q_bl_dep_dn2 = assign33670_e38635_d_n2;
        locals.var_q_bl_dep_dn4 = assign33670_e38635_d_n4;
        locals.var_q_bl_dep_dn5 = assign33670_e38635_d_n5;
        locals.var_q_bl_dep_dn6 = assign33670_e38635_d_n6;
        locals.var_q_bl_dep_dn7 = assign33670_e38635_d_n7;
        locals.var_q_bl_dep_dn8 = assign33670_e38635_d_n8;
        locals.var_q_bl_dep_dn9 = assign33670_e38635_d_n9;
        locals.var_q_bl_dep_dn10 = assign33670_e38635_d_n10;
        locals.var_q_bl_dep_dn11 = assign33670_e38635_d_n11;
        locals.var_q_bl_dep_dn14 = assign33670_e38635_d_n14;
        locals.var_q_bl_dep_rv = 0.0;

        let (assign33680_e38650, assign33680_e38650_d_n0, assign33680_e38650_d_n2, assign33680_e38650_d_n4, assign33680_e38650_d_n5, assign33680_e38650_d_n6, assign33680_e38650_d_n7, assign33680_e38650_d_n8, assign33680_e38650_d_n9, assign33680_e38650_d_n10, assign33680_e38650_d_n11, assign33680_e38650_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard776 == 0.0)) {
        let assign33680_e38646: f64 = (-locals.var_w_subl);
        let assign33680_e38648: f64 = (assign33680_e38646 * locals.var_q_nsub__blk548);
        (assign33680_e38648, (((-locals.var_w_subl_dn0) * locals.var_q_nsub__blk548) + (assign33680_e38646 * locals.var_q_nsub__blk548_dn0)), (((-locals.var_w_subl_dn2) * locals.var_q_nsub__blk548) + (assign33680_e38646 * locals.var_q_nsub__blk548_dn2)), (((-locals.var_w_subl_dn4) * locals.var_q_nsub__blk548) + (assign33680_e38646 * locals.var_q_nsub__blk548_dn4)), (((-locals.var_w_subl_dn5) * locals.var_q_nsub__blk548) + (assign33680_e38646 * locals.var_q_nsub__blk548_dn5)), (((-locals.var_w_subl_dn6) * locals.var_q_nsub__blk548) + (assign33680_e38646 * locals.var_q_nsub__blk548_dn6)), (((-locals.var_w_subl_dn7) * locals.var_q_nsub__blk548) + (assign33680_e38646 * locals.var_q_nsub__blk548_dn7)), (((-locals.var_w_subl_dn8) * locals.var_q_nsub__blk548) + (assign33680_e38646 * locals.var_q_nsub__blk548_dn8)), (((-locals.var_w_subl_dn9) * locals.var_q_nsub__blk548) + (assign33680_e38646 * locals.var_q_nsub__blk548_dn9)), (((-locals.var_w_subl_dn10) * locals.var_q_nsub__blk548) + (assign33680_e38646 * locals.var_q_nsub__blk548_dn10)), (((-locals.var_w_subl_dn11) * locals.var_q_nsub__blk548) + (assign33680_e38646 * locals.var_q_nsub__blk548_dn11)), (((-locals.var_w_subl_dn14) * locals.var_q_nsub__blk548) + (assign33680_e38646 * locals.var_q_nsub__blk548_dn14)),)
    } else {
        (locals.var_q_subl_dep, locals.var_q_subl_dep_dn0, locals.var_q_subl_dep_dn2, locals.var_q_subl_dep_dn4, locals.var_q_subl_dep_dn5, locals.var_q_subl_dep_dn6, locals.var_q_subl_dep_dn7, locals.var_q_subl_dep_dn8, locals.var_q_subl_dep_dn9, locals.var_q_subl_dep_dn10, locals.var_q_subl_dep_dn11, locals.var_q_subl_dep_dn14,)
    }
};
        locals.var_q_subl_dep = assign33680_e38650;
        locals.var_q_subl_dep_dn0 = assign33680_e38650_d_n0;
        locals.var_q_subl_dep_dn2 = assign33680_e38650_d_n2;
        locals.var_q_subl_dep_dn4 = assign33680_e38650_d_n4;
        locals.var_q_subl_dep_dn5 = assign33680_e38650_d_n5;
        locals.var_q_subl_dep_dn6 = assign33680_e38650_d_n6;
        locals.var_q_subl_dep_dn7 = assign33680_e38650_d_n7;
        locals.var_q_subl_dep_dn8 = assign33680_e38650_d_n8;
        locals.var_q_subl_dep_dn9 = assign33680_e38650_d_n9;
        locals.var_q_subl_dep_dn10 = assign33680_e38650_d_n10;
        locals.var_q_subl_dep_dn11 = assign33680_e38650_d_n11;
        locals.var_q_subl_dep_dn14 = assign33680_e38650_d_n14;
        locals.var_q_subl_dep_rv = 0.0;

        let assign33690_e38653: f64 = (locals.var_phi_sl_dep - locals.var_vds_maxbl);
        let assign33690_e38656: f64 = 0.06;
        let assign33690_e38661: f64 = if ((assign33690_e38653 < assign33690_e38656) && (0.06 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard796 = assign33690_e38661;
        locals.var_guard796_rv = 0.0;

        let (assign33700_e38678, assign33700_e38678_d_n0, assign33700_e38678_d_n2, assign33700_e38678_d_n4, assign33700_e38678_d_n5, assign33700_e38678_d_n6, assign33700_e38678_d_n7, assign33700_e38678_d_n8, assign33700_e38678_d_n9, assign33700_e38678_d_n10, assign33700_e38678_d_n11, assign33700_e38678_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard796 != 0.0)) {
        let assign33700_e38672: f64 = 0.06;
        let assign33700_e38675: f64 = (locals.var_phi_sl_dep - locals.var_vds_maxbl);
        let assign33700_e38676: f64 = (assign33700_e38672 - assign33700_e38675);
        (assign33700_e38676, (-(locals.var_phi_sl_dep_dn0 - locals.var_vds_maxbl_dn0)), (-(locals.var_phi_sl_dep_dn2 - locals.var_vds_maxbl_dn2)), (-(locals.var_phi_sl_dep_dn4 - locals.var_vds_maxbl_dn4)), (-(locals.var_phi_sl_dep_dn5 - locals.var_vds_maxbl_dn5)), (-(locals.var_phi_sl_dep_dn6 - locals.var_vds_maxbl_dn6)), (-(locals.var_phi_sl_dep_dn7 - locals.var_vds_maxbl_dn7)), (-(locals.var_phi_sl_dep_dn8 - locals.var_vds_maxbl_dn8)), (-(locals.var_phi_sl_dep_dn9 - locals.var_vds_maxbl_dn9)), (-(locals.var_phi_sl_dep_dn10 - locals.var_vds_maxbl_dn10)), (-(locals.var_phi_sl_dep_dn11 - locals.var_vds_maxbl_dn11)), (-(locals.var_phi_sl_dep_dn14 - locals.var_vds_maxbl_dn14)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign33700_e38678;
        locals.var_tmf1_dn0 = assign33700_e38678_d_n0;
        locals.var_tmf1_dn2 = assign33700_e38678_d_n2;
        locals.var_tmf1_dn4 = assign33700_e38678_d_n4;
        locals.var_tmf1_dn5 = assign33700_e38678_d_n5;
        locals.var_tmf1_dn6 = assign33700_e38678_d_n6;
        locals.var_tmf1_dn7 = assign33700_e38678_d_n7;
        locals.var_tmf1_dn8 = assign33700_e38678_d_n8;
        locals.var_tmf1_dn9 = assign33700_e38678_d_n9;
        locals.var_tmf1_dn10 = assign33700_e38678_d_n10;
        locals.var_tmf1_dn11 = assign33700_e38678_d_n11;
        locals.var_tmf1_dn14 = assign33700_e38678_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign33710_e38691, assign33710_e38691_d_n0, assign33710_e38691_d_n2, assign33710_e38691_d_n4, assign33710_e38691_d_n5, assign33710_e38691_d_n6, assign33710_e38691_d_n7, assign33710_e38691_d_n8, assign33710_e38691_d_n9, assign33710_e38691_d_n10, assign33710_e38691_d_n11, assign33710_e38691_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard796 != 0.0)) {
        let assign33710_e38689: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign33710_e38689, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign33710_e38691;
        locals.var_x2_dn0 = assign33710_e38691_d_n0;
        locals.var_x2_dn2 = assign33710_e38691_d_n2;
        locals.var_x2_dn4 = assign33710_e38691_d_n4;
        locals.var_x2_dn5 = assign33710_e38691_d_n5;
        locals.var_x2_dn6 = assign33710_e38691_d_n6;
        locals.var_x2_dn7 = assign33710_e38691_d_n7;
        locals.var_x2_dn8 = assign33710_e38691_d_n8;
        locals.var_x2_dn9 = assign33710_e38691_d_n9;
        locals.var_x2_dn10 = assign33710_e38691_d_n10;
        locals.var_x2_dn11 = assign33710_e38691_d_n11;
        locals.var_x2_dn14 = assign33710_e38691_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign33720_e38704, assign33720_e38704_d_n0, assign33720_e38704_d_n2, assign33720_e38704_d_n4, assign33720_e38704_d_n5, assign33720_e38704_d_n6, assign33720_e38704_d_n7, assign33720_e38704_d_n8, assign33720_e38704_d_n9, assign33720_e38704_d_n10, assign33720_e38704_d_n11, assign33720_e38704_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard796 != 0.0)) {
        let assign33720_e38702: f64 = (0.06 * 0.06);
        (assign33720_e38702, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign33720_e38704;
        locals.var_xmax2_dn0 = assign33720_e38704_d_n0;
        locals.var_xmax2_dn2 = assign33720_e38704_d_n2;
        locals.var_xmax2_dn4 = assign33720_e38704_d_n4;
        locals.var_xmax2_dn5 = assign33720_e38704_d_n5;
        locals.var_xmax2_dn6 = assign33720_e38704_d_n6;
        locals.var_xmax2_dn7 = assign33720_e38704_d_n7;
        locals.var_xmax2_dn8 = assign33720_e38704_d_n8;
        locals.var_xmax2_dn9 = assign33720_e38704_d_n9;
        locals.var_xmax2_dn10 = assign33720_e38704_d_n10;
        locals.var_xmax2_dn11 = assign33720_e38704_d_n11;
        locals.var_xmax2_dn14 = assign33720_e38704_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign33730_e38715, assign33730_e38715_d_n0, assign33730_e38715_d_n2, assign33730_e38715_d_n4, assign33730_e38715_d_n5, assign33730_e38715_d_n6, assign33730_e38715_d_n7, assign33730_e38715_d_n8, assign33730_e38715_d_n9, assign33730_e38715_d_n10, assign33730_e38715_d_n11, assign33730_e38715_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard796 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign33730_e38715;
        locals.var_xp_dn0 = assign33730_e38715_d_n0;
        locals.var_xp_dn2 = assign33730_e38715_d_n2;
        locals.var_xp_dn4 = assign33730_e38715_d_n4;
        locals.var_xp_dn5 = assign33730_e38715_d_n5;
        locals.var_xp_dn6 = assign33730_e38715_d_n6;
        locals.var_xp_dn7 = assign33730_e38715_d_n7;
        locals.var_xp_dn8 = assign33730_e38715_d_n8;
        locals.var_xp_dn9 = assign33730_e38715_d_n9;
        locals.var_xp_dn10 = assign33730_e38715_d_n10;
        locals.var_xp_dn11 = assign33730_e38715_d_n11;
        locals.var_xp_dn14 = assign33730_e38715_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign33740_e38726, assign33740_e38726_d_n0, assign33740_e38726_d_n2, assign33740_e38726_d_n4, assign33740_e38726_d_n5, assign33740_e38726_d_n6, assign33740_e38726_d_n7, assign33740_e38726_d_n8, assign33740_e38726_d_n9, assign33740_e38726_d_n10, assign33740_e38726_d_n11, assign33740_e38726_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard796 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign33740_e38726;
        locals.var_xmp_dn0 = assign33740_e38726_d_n0;
        locals.var_xmp_dn2 = assign33740_e38726_d_n2;
        locals.var_xmp_dn4 = assign33740_e38726_d_n4;
        locals.var_xmp_dn5 = assign33740_e38726_d_n5;
        locals.var_xmp_dn6 = assign33740_e38726_d_n6;
        locals.var_xmp_dn7 = assign33740_e38726_d_n7;
        locals.var_xmp_dn8 = assign33740_e38726_d_n8;
        locals.var_xmp_dn9 = assign33740_e38726_d_n9;
        locals.var_xmp_dn10 = assign33740_e38726_d_n10;
        locals.var_xmp_dn11 = assign33740_e38726_d_n11;
        locals.var_xmp_dn14 = assign33740_e38726_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign33750_e38737,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard796 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign33750_e38737;
        locals.var_m0_rv = 0.0;

        let (assign33760_e38748,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard796 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33760_e38748;
        locals.var_mm_rv = 0.0;

        let (assign33770_e38759, assign33770_e38759_d_n0, assign33770_e38759_d_n2, assign33770_e38759_d_n4, assign33770_e38759_d_n5, assign33770_e38759_d_n6, assign33770_e38759_d_n7, assign33770_e38759_d_n8, assign33770_e38759_d_n9, assign33770_e38759_d_n10, assign33770_e38759_d_n11, assign33770_e38759_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard796 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign33770_e38759;
        locals.var_arg_dn0 = assign33770_e38759_d_n0;
        locals.var_arg_dn2 = assign33770_e38759_d_n2;
        locals.var_arg_dn4 = assign33770_e38759_d_n4;
        locals.var_arg_dn5 = assign33770_e38759_d_n5;
        locals.var_arg_dn6 = assign33770_e38759_d_n6;
        locals.var_arg_dn7 = assign33770_e38759_d_n7;
        locals.var_arg_dn8 = assign33770_e38759_d_n8;
        locals.var_arg_dn9 = assign33770_e38759_d_n9;
        locals.var_arg_dn10 = assign33770_e38759_d_n10;
        locals.var_arg_dn11 = assign33770_e38759_d_n11;
        locals.var_arg_dn14 = assign33770_e38759_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign33780_e38770, assign33780_e38770_d_n0, assign33780_e38770_d_n2, assign33780_e38770_d_n4, assign33780_e38770_d_n5, assign33780_e38770_d_n6, assign33780_e38770_d_n7, assign33780_e38770_d_n8, assign33780_e38770_d_n9, assign33780_e38770_d_n10, assign33780_e38770_d_n11, assign33780_e38770_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard796 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign33780_e38770;
        locals.var_dnm_dn0 = assign33780_e38770_d_n0;
        locals.var_dnm_dn2 = assign33780_e38770_d_n2;
        locals.var_dnm_dn4 = assign33780_e38770_d_n4;
        locals.var_dnm_dn5 = assign33780_e38770_d_n5;
        locals.var_dnm_dn6 = assign33780_e38770_d_n6;
        locals.var_dnm_dn7 = assign33780_e38770_d_n7;
        locals.var_dnm_dn8 = assign33780_e38770_d_n8;
        locals.var_dnm_dn9 = assign33780_e38770_d_n9;
        locals.var_dnm_dn10 = assign33780_e38770_d_n10;
        locals.var_dnm_dn11 = assign33780_e38770_d_n11;
        locals.var_dnm_dn14 = assign33780_e38770_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign33790_e38783, assign33790_e38783_d_n0, assign33790_e38783_d_n2, assign33790_e38783_d_n4, assign33790_e38783_d_n5, assign33790_e38783_d_n6, assign33790_e38783_d_n7, assign33790_e38783_d_n8, assign33790_e38783_d_n9, assign33790_e38783_d_n10, assign33790_e38783_d_n11, assign33790_e38783_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard796 != 0.0)) {
        let assign33790_e38781: f64 = (locals.var_xp * locals.var_x2);
        (assign33790_e38781, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign33790_e38783;
        locals.var_xp_dn0 = assign33790_e38783_d_n0;
        locals.var_xp_dn2 = assign33790_e38783_d_n2;
        locals.var_xp_dn4 = assign33790_e38783_d_n4;
        locals.var_xp_dn5 = assign33790_e38783_d_n5;
        locals.var_xp_dn6 = assign33790_e38783_d_n6;
        locals.var_xp_dn7 = assign33790_e38783_d_n7;
        locals.var_xp_dn8 = assign33790_e38783_d_n8;
        locals.var_xp_dn9 = assign33790_e38783_d_n9;
        locals.var_xp_dn10 = assign33790_e38783_d_n10;
        locals.var_xp_dn11 = assign33790_e38783_d_n11;
        locals.var_xp_dn14 = assign33790_e38783_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign33800_e38796, assign33800_e38796_d_n0, assign33800_e38796_d_n2, assign33800_e38796_d_n4, assign33800_e38796_d_n5, assign33800_e38796_d_n6, assign33800_e38796_d_n7, assign33800_e38796_d_n8, assign33800_e38796_d_n9, assign33800_e38796_d_n10, assign33800_e38796_d_n11, assign33800_e38796_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard796 != 0.0)) {
        let assign33800_e38794: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign33800_e38794, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign33800_e38796;
        locals.var_xmp_dn0 = assign33800_e38796_d_n0;
        locals.var_xmp_dn2 = assign33800_e38796_d_n2;
        locals.var_xmp_dn4 = assign33800_e38796_d_n4;
        locals.var_xmp_dn5 = assign33800_e38796_d_n5;
        locals.var_xmp_dn6 = assign33800_e38796_d_n6;
        locals.var_xmp_dn7 = assign33800_e38796_d_n7;
        locals.var_xmp_dn8 = assign33800_e38796_d_n8;
        locals.var_xmp_dn9 = assign33800_e38796_d_n9;
        locals.var_xmp_dn10 = assign33800_e38796_d_n10;
        locals.var_xmp_dn11 = assign33800_e38796_d_n11;
        locals.var_xmp_dn14 = assign33800_e38796_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign33810_e38809, assign33810_e38809_d_n0, assign33810_e38809_d_n2, assign33810_e38809_d_n4, assign33810_e38809_d_n5, assign33810_e38809_d_n6, assign33810_e38809_d_n7, assign33810_e38809_d_n8, assign33810_e38809_d_n9, assign33810_e38809_d_n10, assign33810_e38809_d_n11, assign33810_e38809_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard796 != 0.0)) {
        let assign33810_e38807: f64 = (locals.var_xp * locals.var_x2);
        (assign33810_e38807, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign33810_e38809;
        locals.var_xp_dn0 = assign33810_e38809_d_n0;
        locals.var_xp_dn2 = assign33810_e38809_d_n2;
        locals.var_xp_dn4 = assign33810_e38809_d_n4;
        locals.var_xp_dn5 = assign33810_e38809_d_n5;
        locals.var_xp_dn6 = assign33810_e38809_d_n6;
        locals.var_xp_dn7 = assign33810_e38809_d_n7;
        locals.var_xp_dn8 = assign33810_e38809_d_n8;
        locals.var_xp_dn9 = assign33810_e38809_d_n9;
        locals.var_xp_dn10 = assign33810_e38809_d_n10;
        locals.var_xp_dn11 = assign33810_e38809_d_n11;
        locals.var_xp_dn14 = assign33810_e38809_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign33820_e38822, assign33820_e38822_d_n0, assign33820_e38822_d_n2, assign33820_e38822_d_n4, assign33820_e38822_d_n5, assign33820_e38822_d_n6, assign33820_e38822_d_n7, assign33820_e38822_d_n8, assign33820_e38822_d_n9, assign33820_e38822_d_n10, assign33820_e38822_d_n11, assign33820_e38822_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard796 != 0.0)) {
        let assign33820_e38820: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign33820_e38820, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign33820_e38822;
        locals.var_xmp_dn0 = assign33820_e38822_d_n0;
        locals.var_xmp_dn2 = assign33820_e38822_d_n2;
        locals.var_xmp_dn4 = assign33820_e38822_d_n4;
        locals.var_xmp_dn5 = assign33820_e38822_d_n5;
        locals.var_xmp_dn6 = assign33820_e38822_d_n6;
        locals.var_xmp_dn7 = assign33820_e38822_d_n7;
        locals.var_xmp_dn8 = assign33820_e38822_d_n8;
        locals.var_xmp_dn9 = assign33820_e38822_d_n9;
        locals.var_xmp_dn10 = assign33820_e38822_d_n10;
        locals.var_xmp_dn11 = assign33820_e38822_d_n11;
        locals.var_xmp_dn14 = assign33820_e38822_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign33830_e38835, assign33830_e38835_d_n0, assign33830_e38835_d_n2, assign33830_e38835_d_n4, assign33830_e38835_d_n5, assign33830_e38835_d_n6, assign33830_e38835_d_n7, assign33830_e38835_d_n8, assign33830_e38835_d_n9, assign33830_e38835_d_n10, assign33830_e38835_d_n11, assign33830_e38835_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard796 != 0.0)) {
        let assign33830_e38833: f64 = (locals.var_xp + locals.var_xmp);
        (assign33830_e38833, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign33830_e38835;
        locals.var_arg_dn0 = assign33830_e38835_d_n0;
        locals.var_arg_dn2 = assign33830_e38835_d_n2;
        locals.var_arg_dn4 = assign33830_e38835_d_n4;
        locals.var_arg_dn5 = assign33830_e38835_d_n5;
        locals.var_arg_dn6 = assign33830_e38835_d_n6;
        locals.var_arg_dn7 = assign33830_e38835_d_n7;
        locals.var_arg_dn8 = assign33830_e38835_d_n8;
        locals.var_arg_dn9 = assign33830_e38835_d_n9;
        locals.var_arg_dn10 = assign33830_e38835_d_n10;
        locals.var_arg_dn11 = assign33830_e38835_d_n11;
        locals.var_arg_dn14 = assign33830_e38835_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign33840_e38846, assign33840_e38846_d_n0, assign33840_e38846_d_n2, assign33840_e38846_d_n4, assign33840_e38846_d_n5, assign33840_e38846_d_n6, assign33840_e38846_d_n7, assign33840_e38846_d_n8, assign33840_e38846_d_n9, assign33840_e38846_d_n10, assign33840_e38846_d_n11, assign33840_e38846_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard796 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign33840_e38846;
        locals.var_dnm_dn0 = assign33840_e38846_d_n0;
        locals.var_dnm_dn2 = assign33840_e38846_d_n2;
        locals.var_dnm_dn4 = assign33840_e38846_d_n4;
        locals.var_dnm_dn5 = assign33840_e38846_d_n5;
        locals.var_dnm_dn6 = assign33840_e38846_d_n6;
        locals.var_dnm_dn7 = assign33840_e38846_d_n7;
        locals.var_dnm_dn8 = assign33840_e38846_d_n8;
        locals.var_dnm_dn9 = assign33840_e38846_d_n9;
        locals.var_dnm_dn10 = assign33840_e38846_d_n10;
        locals.var_dnm_dn11 = assign33840_e38846_d_n11;
        locals.var_dnm_dn14 = assign33840_e38846_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign33850_e38861: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard797 = assign33850_e38861;
        locals.var_guard797_rv = 0.0;

        let assign33860_e38864: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard798 = assign33860_e38864;
        locals.var_guard798_rv = 0.0;

        let (assign33870_e38879,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard796 != 0.0)) && (locals.var_guard797 != 0.0)) && (locals.var_guard798 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33870_e38879;
        locals.var_mm_rv = 0.0;

        let assign33880_e38882: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard799 = assign33880_e38882;
        locals.var_guard799_rv = 0.0;

        let (assign33890_e38900,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard796 != 0.0)) && (locals.var_guard797 != 0.0)) && (locals.var_guard798 == 0.0)) && (locals.var_guard799 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33890_e38900;
        locals.var_mm_rv = 0.0;

        let assign33900_e38903: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard800 = assign33900_e38903;
        locals.var_guard800_rv = 0.0;

        let (assign33910_e38924,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard796 != 0.0)) && (locals.var_guard797 != 0.0)) && (locals.var_guard798 == 0.0)) && (locals.var_guard799 == 0.0)) && (locals.var_guard800 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33910_e38924;
        locals.var_mm_rv = 0.0;

        let assign33920_e38927: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard801 = assign33920_e38927;
        locals.var_guard801_rv = 0.0;

        let (assign33930_e38951,) = {
    if (((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard796 != 0.0)) && (locals.var_guard797 != 0.0)) && (locals.var_guard798 == 0.0)) && (locals.var_guard799 == 0.0)) && (locals.var_guard800 == 0.0)) && (locals.var_guard801 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33930_e38951;
        locals.var_mm_rv = 0.0;

        let (assign33940_e38964,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard796 != 0.0)) && (locals.var_guard797 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign33940_e38964;
        locals.var_m0_rv = 0.0;

        let mut assign33950_loop_guard: usize = 0;
        while {
            let assign33950_cond_e38978: f64 = if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard796 != 0.0)) && (locals.var_guard797 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign33950_cond_e38978 != 0.0
        } {
            assign33950_loop_guard += 1;
            assert!(assign33950_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign33950_body0_e38992, assign33950_body0_e38992_d_n0, assign33950_body0_e38992_d_n2, assign33950_body0_e38992_d_n4, assign33950_body0_e38992_d_n5, assign33950_body0_e38992_d_n6, assign33950_body0_e38992_d_n7, assign33950_body0_e38992_d_n8, assign33950_body0_e38992_d_n9, assign33950_body0_e38992_d_n10, assign33950_body0_e38992_d_n11, assign33950_body0_e38992_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard796 != 0.0)) && (locals.var_guard797 != 0.0)) {
        let assign33950_body0_e38990: f64 = (locals.var_dnm).sqrt();
        (assign33950_body0_e38990, (locals.var_dnm_dn0 / (2.0 * assign33950_body0_e38990)), (locals.var_dnm_dn2 / (2.0 * assign33950_body0_e38990)), (locals.var_dnm_dn4 / (2.0 * assign33950_body0_e38990)), (locals.var_dnm_dn5 / (2.0 * assign33950_body0_e38990)), (locals.var_dnm_dn6 / (2.0 * assign33950_body0_e38990)), (locals.var_dnm_dn7 / (2.0 * assign33950_body0_e38990)), (locals.var_dnm_dn8 / (2.0 * assign33950_body0_e38990)), (locals.var_dnm_dn9 / (2.0 * assign33950_body0_e38990)), (locals.var_dnm_dn10 / (2.0 * assign33950_body0_e38990)), (locals.var_dnm_dn11 / (2.0 * assign33950_body0_e38990)), (locals.var_dnm_dn14 / (2.0 * assign33950_body0_e38990)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign33950_body0_e38992;
            locals.var_dnm_dn0 = assign33950_body0_e38992_d_n0;
            locals.var_dnm_dn2 = assign33950_body0_e38992_d_n2;
            locals.var_dnm_dn4 = assign33950_body0_e38992_d_n4;
            locals.var_dnm_dn5 = assign33950_body0_e38992_d_n5;
            locals.var_dnm_dn6 = assign33950_body0_e38992_d_n6;
            locals.var_dnm_dn7 = assign33950_body0_e38992_d_n7;
            locals.var_dnm_dn8 = assign33950_body0_e38992_d_n8;
            locals.var_dnm_dn9 = assign33950_body0_e38992_d_n9;
            locals.var_dnm_dn10 = assign33950_body0_e38992_d_n10;
            locals.var_dnm_dn11 = assign33950_body0_e38992_d_n11;
            locals.var_dnm_dn14 = assign33950_body0_e38992_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign33950_body1_e39007,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard796 != 0.0)) && (locals.var_guard797 != 0.0)) {
        let assign33950_body1_e39005: f64 = (locals.var_m0 + 1.0);
        (assign33950_body1_e39005,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign33950_body1_e39007;
            locals.var_m0_rv = 0.0;
        }

        let (assign33960_e39032, assign33960_e39032_d_n0, assign33960_e39032_d_n2, assign33960_e39032_d_n4, assign33960_e39032_d_n5, assign33960_e39032_d_n6, assign33960_e39032_d_n7, assign33960_e39032_d_n8, assign33960_e39032_d_n9, assign33960_e39032_d_n10, assign33960_e39032_d_n11, assign33960_e39032_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard796 != 0.0)) && (locals.var_guard797 == 0.0)) {
        let (assign33960_e39030, assign33960_e39030_d_n0, assign33960_e39030_d_n2, assign33960_e39030_d_n4, assign33960_e39030_d_n5, assign33960_e39030_d_n6, assign33960_e39030_d_n7, assign33960_e39030_d_n8, assign33960_e39030_d_n9, assign33960_e39030_d_n10, assign33960_e39030_d_n11, assign33960_e39030_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign33960_e39027: f64 = (2.0 * 2.0);
                let assign33960_e39028: f64 = (1.0 / assign33960_e39027);
                let assign33960_e39029: f64 = (locals.var_dnm).powf(assign33960_e39028);
                (assign33960_e39029, if 0.0 == 0.0 && ((assign33960_e39028) as f64).is_finite() && ((assign33960_e39028) as f64).fract() == 0.0 { if assign33960_e39028 == 0.0 { 0.0 } else { (assign33960_e39028 * ((locals.var_dnm).powf(assign33960_e39028 - 1.0) * locals.var_dnm_dn0)) } } else { (assign33960_e39029 * (assign33960_e39028 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33960_e39028) as f64).is_finite() && ((assign33960_e39028) as f64).fract() == 0.0 { if assign33960_e39028 == 0.0 { 0.0 } else { (assign33960_e39028 * ((locals.var_dnm).powf(assign33960_e39028 - 1.0) * locals.var_dnm_dn2)) } } else { (assign33960_e39029 * (assign33960_e39028 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33960_e39028) as f64).is_finite() && ((assign33960_e39028) as f64).fract() == 0.0 { if assign33960_e39028 == 0.0 { 0.0 } else { (assign33960_e39028 * ((locals.var_dnm).powf(assign33960_e39028 - 1.0) * locals.var_dnm_dn4)) } } else { (assign33960_e39029 * (assign33960_e39028 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33960_e39028) as f64).is_finite() && ((assign33960_e39028) as f64).fract() == 0.0 { if assign33960_e39028 == 0.0 { 0.0 } else { (assign33960_e39028 * ((locals.var_dnm).powf(assign33960_e39028 - 1.0) * locals.var_dnm_dn5)) } } else { (assign33960_e39029 * (assign33960_e39028 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33960_e39028) as f64).is_finite() && ((assign33960_e39028) as f64).fract() == 0.0 { if assign33960_e39028 == 0.0 { 0.0 } else { (assign33960_e39028 * ((locals.var_dnm).powf(assign33960_e39028 - 1.0) * locals.var_dnm_dn6)) } } else { (assign33960_e39029 * (assign33960_e39028 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33960_e39028) as f64).is_finite() && ((assign33960_e39028) as f64).fract() == 0.0 { if assign33960_e39028 == 0.0 { 0.0 } else { (assign33960_e39028 * ((locals.var_dnm).powf(assign33960_e39028 - 1.0) * locals.var_dnm_dn7)) } } else { (assign33960_e39029 * (assign33960_e39028 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33960_e39028) as f64).is_finite() && ((assign33960_e39028) as f64).fract() == 0.0 { if assign33960_e39028 == 0.0 { 0.0 } else { (assign33960_e39028 * ((locals.var_dnm).powf(assign33960_e39028 - 1.0) * locals.var_dnm_dn8)) } } else { (assign33960_e39029 * (assign33960_e39028 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33960_e39028) as f64).is_finite() && ((assign33960_e39028) as f64).fract() == 0.0 { if assign33960_e39028 == 0.0 { 0.0 } else { (assign33960_e39028 * ((locals.var_dnm).powf(assign33960_e39028 - 1.0) * locals.var_dnm_dn9)) } } else { (assign33960_e39029 * (assign33960_e39028 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33960_e39028) as f64).is_finite() && ((assign33960_e39028) as f64).fract() == 0.0 { if assign33960_e39028 == 0.0 { 0.0 } else { (assign33960_e39028 * ((locals.var_dnm).powf(assign33960_e39028 - 1.0) * locals.var_dnm_dn10)) } } else { (assign33960_e39029 * (assign33960_e39028 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33960_e39028) as f64).is_finite() && ((assign33960_e39028) as f64).fract() == 0.0 { if assign33960_e39028 == 0.0 { 0.0 } else { (assign33960_e39028 * ((locals.var_dnm).powf(assign33960_e39028 - 1.0) * locals.var_dnm_dn11)) } } else { (assign33960_e39029 * (assign33960_e39028 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33960_e39028) as f64).is_finite() && ((assign33960_e39028) as f64).fract() == 0.0 { if assign33960_e39028 == 0.0 { 0.0 } else { (assign33960_e39028 * ((locals.var_dnm).powf(assign33960_e39028 - 1.0) * locals.var_dnm_dn14)) } } else { (assign33960_e39029 * (assign33960_e39028 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign33960_e39030, assign33960_e39030_d_n0, assign33960_e39030_d_n2, assign33960_e39030_d_n4, assign33960_e39030_d_n5, assign33960_e39030_d_n6, assign33960_e39030_d_n7, assign33960_e39030_d_n8, assign33960_e39030_d_n9, assign33960_e39030_d_n10, assign33960_e39030_d_n11, assign33960_e39030_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign33960_e39032;
        locals.var_dnm_dn0 = assign33960_e39032_d_n0;
        locals.var_dnm_dn2 = assign33960_e39032_d_n2;
        locals.var_dnm_dn4 = assign33960_e39032_d_n4;
        locals.var_dnm_dn5 = assign33960_e39032_d_n5;
        locals.var_dnm_dn6 = assign33960_e39032_d_n6;
        locals.var_dnm_dn7 = assign33960_e39032_d_n7;
        locals.var_dnm_dn8 = assign33960_e39032_d_n8;
        locals.var_dnm_dn9 = assign33960_e39032_d_n9;
        locals.var_dnm_dn10 = assign33960_e39032_d_n10;
        locals.var_dnm_dn11 = assign33960_e39032_d_n11;
        locals.var_dnm_dn14 = assign33960_e39032_d_n14;
        locals.var_dnm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_110(
        locals: &mut StampLocals,
    ) {
        let (assign33970_e39045, assign33970_e39045_d_n0, assign33970_e39045_d_n2, assign33970_e39045_d_n4, assign33970_e39045_d_n5, assign33970_e39045_d_n6, assign33970_e39045_d_n7, assign33970_e39045_d_n8, assign33970_e39045_d_n9, assign33970_e39045_d_n10, assign33970_e39045_d_n11, assign33970_e39045_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard796 != 0.0)) {
        let assign33970_e39043: f64 = (1.0 / locals.var_dnm);
        (assign33970_e39043, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign33970_e39045;
        locals.var_dnm_dn0 = assign33970_e39045_d_n0;
        locals.var_dnm_dn2 = assign33970_e39045_d_n2;
        locals.var_dnm_dn4 = assign33970_e39045_d_n4;
        locals.var_dnm_dn5 = assign33970_e39045_d_n5;
        locals.var_dnm_dn6 = assign33970_e39045_d_n6;
        locals.var_dnm_dn7 = assign33970_e39045_d_n7;
        locals.var_dnm_dn8 = assign33970_e39045_d_n8;
        locals.var_dnm_dn9 = assign33970_e39045_d_n9;
        locals.var_dnm_dn10 = assign33970_e39045_d_n10;
        locals.var_dnm_dn11 = assign33970_e39045_d_n11;
        locals.var_dnm_dn14 = assign33970_e39045_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign33980_e39060, assign33980_e39060_d_n0, assign33980_e39060_d_n2, assign33980_e39060_d_n4, assign33980_e39060_d_n5, assign33980_e39060_d_n6, assign33980_e39060_d_n7, assign33980_e39060_d_n8, assign33980_e39060_d_n9, assign33980_e39060_d_n10, assign33980_e39060_d_n11, assign33980_e39060_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard796 != 0.0)) {
        let assign33980_e39056: f64 = (locals.var_tmf1 * 0.06);
        let assign33980_e39058: f64 = (assign33980_e39056 * locals.var_dnm);
        (assign33980_e39058, (((locals.var_tmf1_dn0 * 0.06) * locals.var_dnm) + (assign33980_e39056 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.06) * locals.var_dnm) + (assign33980_e39056 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.06) * locals.var_dnm) + (assign33980_e39056 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.06) * locals.var_dnm) + (assign33980_e39056 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.06) * locals.var_dnm) + (assign33980_e39056 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.06) * locals.var_dnm) + (assign33980_e39056 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.06) * locals.var_dnm) + (assign33980_e39056 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.06) * locals.var_dnm) + (assign33980_e39056 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.06) * locals.var_dnm) + (assign33980_e39056 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.06) * locals.var_dnm) + (assign33980_e39056 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.06) * locals.var_dnm) + (assign33980_e39056 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign33980_e39060;
        locals.var_tmf0_dn0 = assign33980_e39060_d_n0;
        locals.var_tmf0_dn2 = assign33980_e39060_d_n2;
        locals.var_tmf0_dn4 = assign33980_e39060_d_n4;
        locals.var_tmf0_dn5 = assign33980_e39060_d_n5;
        locals.var_tmf0_dn6 = assign33980_e39060_d_n6;
        locals.var_tmf0_dn7 = assign33980_e39060_d_n7;
        locals.var_tmf0_dn8 = assign33980_e39060_d_n8;
        locals.var_tmf0_dn9 = assign33980_e39060_d_n9;
        locals.var_tmf0_dn10 = assign33980_e39060_d_n10;
        locals.var_tmf0_dn11 = assign33980_e39060_d_n11;
        locals.var_tmf0_dn14 = assign33980_e39060_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign33990_e39077, assign33990_e39077_d_n0, assign33990_e39077_d_n2, assign33990_e39077_d_n4, assign33990_e39077_d_n5, assign33990_e39077_d_n6, assign33990_e39077_d_n7, assign33990_e39077_d_n8, assign33990_e39077_d_n9, assign33990_e39077_d_n10, assign33990_e39077_d_n11, assign33990_e39077_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard796 != 0.0)) {
        let assign33990_e39071: f64 = (0.06 * locals.var_xmp);
        let assign33990_e39073: f64 = (assign33990_e39071 * locals.var_dnm);
        let assign33990_e39075: f64 = (assign33990_e39073 / locals.var_arg);
        (assign33990_e39075, ((((((0.06 * locals.var_xmp_dn0) * locals.var_dnm) + (assign33990_e39071 * locals.var_dnm_dn0)) * locals.var_arg) - (assign33990_e39073 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn2) * locals.var_dnm) + (assign33990_e39071 * locals.var_dnm_dn2)) * locals.var_arg) - (assign33990_e39073 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn4) * locals.var_dnm) + (assign33990_e39071 * locals.var_dnm_dn4)) * locals.var_arg) - (assign33990_e39073 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn5) * locals.var_dnm) + (assign33990_e39071 * locals.var_dnm_dn5)) * locals.var_arg) - (assign33990_e39073 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn6) * locals.var_dnm) + (assign33990_e39071 * locals.var_dnm_dn6)) * locals.var_arg) - (assign33990_e39073 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn7) * locals.var_dnm) + (assign33990_e39071 * locals.var_dnm_dn7)) * locals.var_arg) - (assign33990_e39073 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn8) * locals.var_dnm) + (assign33990_e39071 * locals.var_dnm_dn8)) * locals.var_arg) - (assign33990_e39073 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn9) * locals.var_dnm) + (assign33990_e39071 * locals.var_dnm_dn9)) * locals.var_arg) - (assign33990_e39073 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn10) * locals.var_dnm) + (assign33990_e39071 * locals.var_dnm_dn10)) * locals.var_arg) - (assign33990_e39073 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn11) * locals.var_dnm) + (assign33990_e39071 * locals.var_dnm_dn11)) * locals.var_arg) - (assign33990_e39073 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn14) * locals.var_dnm) + (assign33990_e39071 * locals.var_dnm_dn14)) * locals.var_arg) - (assign33990_e39073 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign33990_e39077;
        locals.var_t0_dn0 = assign33990_e39077_d_n0;
        locals.var_t0_dn2 = assign33990_e39077_d_n2;
        locals.var_t0_dn4 = assign33990_e39077_d_n4;
        locals.var_t0_dn5 = assign33990_e39077_d_n5;
        locals.var_t0_dn6 = assign33990_e39077_d_n6;
        locals.var_t0_dn7 = assign33990_e39077_d_n7;
        locals.var_t0_dn8 = assign33990_e39077_d_n8;
        locals.var_t0_dn9 = assign33990_e39077_d_n9;
        locals.var_t0_dn10 = assign33990_e39077_d_n10;
        locals.var_t0_dn11 = assign33990_e39077_d_n11;
        locals.var_t0_dn14 = assign33990_e39077_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign34000_e39092, assign34000_e39092_d_n0, assign34000_e39092_d_n2, assign34000_e39092_d_n4, assign34000_e39092_d_n5, assign34000_e39092_d_n6, assign34000_e39092_d_n7, assign34000_e39092_d_n8, assign34000_e39092_d_n9, assign34000_e39092_d_n10, assign34000_e39092_d_n11, assign34000_e39092_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard796 != 0.0)) {
        let assign34000_e39088: f64 = 0.06;
        let assign34000_e39090: f64 = (assign34000_e39088 - locals.var_tmf0);
        (assign34000_e39090, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign34000_e39092;
        locals.var_t2_dn0 = assign34000_e39092_d_n0;
        locals.var_t2_dn2 = assign34000_e39092_d_n2;
        locals.var_t2_dn4 = assign34000_e39092_d_n4;
        locals.var_t2_dn5 = assign34000_e39092_d_n5;
        locals.var_t2_dn6 = assign34000_e39092_d_n6;
        locals.var_t2_dn7 = assign34000_e39092_d_n7;
        locals.var_t2_dn8 = assign34000_e39092_d_n8;
        locals.var_t2_dn9 = assign34000_e39092_d_n9;
        locals.var_t2_dn10 = assign34000_e39092_d_n10;
        locals.var_t2_dn11 = assign34000_e39092_d_n11;
        locals.var_t2_dn14 = assign34000_e39092_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign34010_e39103, assign34010_e39103_d_n0, assign34010_e39103_d_n2, assign34010_e39103_d_n4, assign34010_e39103_d_n5, assign34010_e39103_d_n6, assign34010_e39103_d_n7, assign34010_e39103_d_n8, assign34010_e39103_d_n9, assign34010_e39103_d_n10, assign34010_e39103_d_n11, assign34010_e39103_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard796 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign34010_e39103;
        locals.var_t0_dn0 = assign34010_e39103_d_n0;
        locals.var_t0_dn2 = assign34010_e39103_d_n2;
        locals.var_t0_dn4 = assign34010_e39103_d_n4;
        locals.var_t0_dn5 = assign34010_e39103_d_n5;
        locals.var_t0_dn6 = assign34010_e39103_d_n6;
        locals.var_t0_dn7 = assign34010_e39103_d_n7;
        locals.var_t0_dn8 = assign34010_e39103_d_n8;
        locals.var_t0_dn9 = assign34010_e39103_d_n9;
        locals.var_t0_dn10 = assign34010_e39103_d_n10;
        locals.var_t0_dn11 = assign34010_e39103_d_n11;
        locals.var_t0_dn14 = assign34010_e39103_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign34020_e39117, assign34020_e39117_d_n0, assign34020_e39117_d_n2, assign34020_e39117_d_n4, assign34020_e39117_d_n5, assign34020_e39117_d_n6, assign34020_e39117_d_n7, assign34020_e39117_d_n8, assign34020_e39117_d_n9, assign34020_e39117_d_n10, assign34020_e39117_d_n11, assign34020_e39117_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard796 == 0.0)) {
        let assign34020_e39115: f64 = (locals.var_phi_sl_dep - locals.var_vds_maxbl);
        (assign34020_e39115, (locals.var_phi_sl_dep_dn0 - locals.var_vds_maxbl_dn0), (locals.var_phi_sl_dep_dn2 - locals.var_vds_maxbl_dn2), (locals.var_phi_sl_dep_dn4 - locals.var_vds_maxbl_dn4), (locals.var_phi_sl_dep_dn5 - locals.var_vds_maxbl_dn5), (locals.var_phi_sl_dep_dn6 - locals.var_vds_maxbl_dn6), (locals.var_phi_sl_dep_dn7 - locals.var_vds_maxbl_dn7), (locals.var_phi_sl_dep_dn8 - locals.var_vds_maxbl_dn8), (locals.var_phi_sl_dep_dn9 - locals.var_vds_maxbl_dn9), (locals.var_phi_sl_dep_dn10 - locals.var_vds_maxbl_dn10), (locals.var_phi_sl_dep_dn11 - locals.var_vds_maxbl_dn11), (locals.var_phi_sl_dep_dn14 - locals.var_vds_maxbl_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign34020_e39117;
        locals.var_t2_dn0 = assign34020_e39117_d_n0;
        locals.var_t2_dn2 = assign34020_e39117_d_n2;
        locals.var_t2_dn4 = assign34020_e39117_d_n4;
        locals.var_t2_dn5 = assign34020_e39117_d_n5;
        locals.var_t2_dn6 = assign34020_e39117_d_n6;
        locals.var_t2_dn7 = assign34020_e39117_d_n7;
        locals.var_t2_dn8 = assign34020_e39117_d_n8;
        locals.var_t2_dn9 = assign34020_e39117_d_n9;
        locals.var_t2_dn10 = assign34020_e39117_d_n10;
        locals.var_t2_dn11 = assign34020_e39117_d_n11;
        locals.var_t2_dn14 = assign34020_e39117_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign34030_e39129, assign34030_e39129_d_n0, assign34030_e39129_d_n2, assign34030_e39129_d_n4, assign34030_e39129_d_n5, assign34030_e39129_d_n6, assign34030_e39129_d_n7, assign34030_e39129_d_n8, assign34030_e39129_d_n9, assign34030_e39129_d_n10, assign34030_e39129_d_n11, assign34030_e39129_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard796 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign34030_e39129;
        locals.var_t0_dn0 = assign34030_e39129_d_n0;
        locals.var_t0_dn2 = assign34030_e39129_d_n2;
        locals.var_t0_dn4 = assign34030_e39129_d_n4;
        locals.var_t0_dn5 = assign34030_e39129_d_n5;
        locals.var_t0_dn6 = assign34030_e39129_d_n6;
        locals.var_t0_dn7 = assign34030_e39129_d_n7;
        locals.var_t0_dn8 = assign34030_e39129_d_n8;
        locals.var_t0_dn9 = assign34030_e39129_d_n9;
        locals.var_t0_dn10 = assign34030_e39129_d_n10;
        locals.var_t0_dn11 = assign34030_e39129_d_n11;
        locals.var_t0_dn14 = assign34030_e39129_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign34040_e39151, assign34040_e39151_d_n0, assign34040_e39151_d_n2, assign34040_e39151_d_n4, assign34040_e39151_d_n5, assign34040_e39151_d_n6, assign34040_e39151_d_n7, assign34040_e39151_d_n8, assign34040_e39151_d_n9, assign34040_e39151_d_n10, assign34040_e39151_d_n11, assign34040_e39151_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) {
        let assign34040_e39138: f64 = (locals.var_beta * locals.var_t2);
        let assign34040_e39139: f64 = (assign34040_e39138).exp();
        let assign34040_e39141: f64 = (assign34040_e39139 - 1.0);
        let assign34040_e39144: f64 = (locals.var_beta * locals.var_t2);
        let assign34040_e39145: f64 = (assign34040_e39141 - assign34040_e39144);
        let assign34040_e39148: f64 = (10.0 * 2.220446049250313e-16);
        let assign34040_e39149: f64 = (assign34040_e39145 + assign34040_e39148);
        (assign34040_e39149, ((assign34040_e39139 * ((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0))) - ((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0))), ((assign34040_e39139 * ((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2))) - ((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2))), ((assign34040_e39139 * ((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4))) - ((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4))), ((assign34040_e39139 * ((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5))) - ((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5))), ((assign34040_e39139 * ((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6))) - ((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6))), ((assign34040_e39139 * ((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7))) - ((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7))), ((assign34040_e39139 * ((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8))) - ((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8))), ((assign34040_e39139 * ((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9))) - ((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9))), ((assign34040_e39139 * ((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10))) - ((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10))), ((assign34040_e39139 * ((locals.var_beta_dn11 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn11))) - ((locals.var_beta_dn11 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn11))), ((assign34040_e39139 * ((locals.var_beta_dn14 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn14))) - ((locals.var_beta_dn14 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign34040_e39151;
        locals.var_t4_dn0 = assign34040_e39151_d_n0;
        locals.var_t4_dn2 = assign34040_e39151_d_n2;
        locals.var_t4_dn4 = assign34040_e39151_d_n4;
        locals.var_t4_dn5 = assign34040_e39151_d_n5;
        locals.var_t4_dn6 = assign34040_e39151_d_n6;
        locals.var_t4_dn7 = assign34040_e39151_d_n7;
        locals.var_t4_dn8 = assign34040_e39151_d_n8;
        locals.var_t4_dn9 = assign34040_e39151_d_n9;
        locals.var_t4_dn10 = assign34040_e39151_d_n10;
        locals.var_t4_dn11 = assign34040_e39151_d_n11;
        locals.var_t4_dn14 = assign34040_e39151_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign34050_e39164, assign34050_e39164_d_n0, assign34050_e39164_d_n2, assign34050_e39164_d_n4, assign34050_e39164_d_n5, assign34050_e39164_d_n6, assign34050_e39164_d_n7, assign34050_e39164_d_n8, assign34050_e39164_d_n9, assign34050_e39164_d_n10, assign34050_e39164_d_n11, assign34050_e39164_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard715 == 0.0)) {
        let assign34050_e39159: f64 = (-locals.var_cnst0);
        let assign34050_e39161: f64 = (locals.var_t4).sqrt();
        let assign34050_e39162: f64 = (assign34050_e39159 * assign34050_e39161);
        (assign34050_e39162, (((-locals.var_cnst0_dn0) * assign34050_e39161) + (assign34050_e39159 * (locals.var_t4_dn0 / (2.0 * assign34050_e39161)))), (((-locals.var_cnst0_dn2) * assign34050_e39161) + (assign34050_e39159 * (locals.var_t4_dn2 / (2.0 * assign34050_e39161)))), (((-locals.var_cnst0_dn4) * assign34050_e39161) + (assign34050_e39159 * (locals.var_t4_dn4 / (2.0 * assign34050_e39161)))), (((-locals.var_cnst0_dn5) * assign34050_e39161) + (assign34050_e39159 * (locals.var_t4_dn5 / (2.0 * assign34050_e39161)))), (((-locals.var_cnst0_dn6) * assign34050_e39161) + (assign34050_e39159 * (locals.var_t4_dn6 / (2.0 * assign34050_e39161)))), (((-locals.var_cnst0_dn7) * assign34050_e39161) + (assign34050_e39159 * (locals.var_t4_dn7 / (2.0 * assign34050_e39161)))), (((-locals.var_cnst0_dn8) * assign34050_e39161) + (assign34050_e39159 * (locals.var_t4_dn8 / (2.0 * assign34050_e39161)))), (((-locals.var_cnst0_dn9) * assign34050_e39161) + (assign34050_e39159 * (locals.var_t4_dn9 / (2.0 * assign34050_e39161)))), (((-locals.var_cnst0_dn10) * assign34050_e39161) + (assign34050_e39159 * (locals.var_t4_dn10 / (2.0 * assign34050_e39161)))), (((-locals.var_cnst0_dn11) * assign34050_e39161) + (assign34050_e39159 * (locals.var_t4_dn11 / (2.0 * assign34050_e39161)))), (((-locals.var_cnst0_dn14) * assign34050_e39161) + (assign34050_e39159 * (locals.var_t4_dn14 / (2.0 * assign34050_e39161)))),)
    } else {
        (locals.var_q_nl_cur, locals.var_q_nl_cur_dn0, locals.var_q_nl_cur_dn2, locals.var_q_nl_cur_dn4, locals.var_q_nl_cur_dn5, locals.var_q_nl_cur_dn6, locals.var_q_nl_cur_dn7, locals.var_q_nl_cur_dn8, locals.var_q_nl_cur_dn9, locals.var_q_nl_cur_dn10, locals.var_q_nl_cur_dn11, locals.var_q_nl_cur_dn14,)
    }
};
        locals.var_q_nl_cur = assign34050_e39164;
        locals.var_q_nl_cur_dn0 = assign34050_e39164_d_n0;
        locals.var_q_nl_cur_dn2 = assign34050_e39164_d_n2;
        locals.var_q_nl_cur_dn4 = assign34050_e39164_d_n4;
        locals.var_q_nl_cur_dn5 = assign34050_e39164_d_n5;
        locals.var_q_nl_cur_dn6 = assign34050_e39164_d_n6;
        locals.var_q_nl_cur_dn7 = assign34050_e39164_d_n7;
        locals.var_q_nl_cur_dn8 = assign34050_e39164_d_n8;
        locals.var_q_nl_cur_dn9 = assign34050_e39164_d_n9;
        locals.var_q_nl_cur_dn10 = assign34050_e39164_d_n10;
        locals.var_q_nl_cur_dn11 = assign34050_e39164_d_n11;
        locals.var_q_nl_cur_dn14 = assign34050_e39164_d_n14;
        locals.var_q_nl_cur_rv = 0.0;

        let (assign34060_e39170, assign34060_e39170_d_n0, assign34060_e39170_d_n2, assign34060_e39170_d_n4, assign34060_e39170_d_n5, assign34060_e39170_d_n6, assign34060_e39170_d_n7, assign34060_e39170_d_n8, assign34060_e39170_d_n9, assign34060_e39170_d_n10, assign34060_e39170_d_n11, assign34060_e39170_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        (locals.var_phi_s0_dep, locals.var_phi_s0_dep_dn0, locals.var_phi_s0_dep_dn2, locals.var_phi_s0_dep_dn4, locals.var_phi_s0_dep_dn5, locals.var_phi_s0_dep_dn6, locals.var_phi_s0_dep_dn7, locals.var_phi_s0_dep_dn8, locals.var_phi_s0_dep_dn9, locals.var_phi_s0_dep_dn10, locals.var_phi_s0_dep_dn11, locals.var_phi_s0_dep_dn14,)
    } else {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn14,)
    }
};
        locals.var_ps0 = assign34060_e39170;
        locals.var_ps0_dn0 = assign34060_e39170_d_n0;
        locals.var_ps0_dn2 = assign34060_e39170_d_n2;
        locals.var_ps0_dn4 = assign34060_e39170_d_n4;
        locals.var_ps0_dn5 = assign34060_e39170_d_n5;
        locals.var_ps0_dn6 = assign34060_e39170_d_n6;
        locals.var_ps0_dn7 = assign34060_e39170_d_n7;
        locals.var_ps0_dn8 = assign34060_e39170_d_n8;
        locals.var_ps0_dn9 = assign34060_e39170_d_n9;
        locals.var_ps0_dn10 = assign34060_e39170_d_n10;
        locals.var_ps0_dn11 = assign34060_e39170_d_n11;
        locals.var_ps0_dn14 = assign34060_e39170_d_n14;
        locals.var_ps0_rv = 0.0;

        let (assign34070_e39176, assign34070_e39176_d_n0, assign34070_e39176_d_n2, assign34070_e39176_d_n4, assign34070_e39176_d_n5, assign34070_e39176_d_n6, assign34070_e39176_d_n7, assign34070_e39176_d_n8, assign34070_e39176_d_n9, assign34070_e39176_d_n10, assign34070_e39176_d_n11, assign34070_e39176_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        (locals.var_phi_sl_dep, locals.var_phi_sl_dep_dn0, locals.var_phi_sl_dep_dn2, locals.var_phi_sl_dep_dn4, locals.var_phi_sl_dep_dn5, locals.var_phi_sl_dep_dn6, locals.var_phi_sl_dep_dn7, locals.var_phi_sl_dep_dn8, locals.var_phi_sl_dep_dn9, locals.var_phi_sl_dep_dn10, locals.var_phi_sl_dep_dn11, locals.var_phi_sl_dep_dn14,)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn4, locals.var_psl_dn5, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn8, locals.var_psl_dn9, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn14,)
    }
};
        locals.var_psl = assign34070_e39176;
        locals.var_psl_dn0 = assign34070_e39176_d_n0;
        locals.var_psl_dn2 = assign34070_e39176_d_n2;
        locals.var_psl_dn4 = assign34070_e39176_d_n4;
        locals.var_psl_dn5 = assign34070_e39176_d_n5;
        locals.var_psl_dn6 = assign34070_e39176_d_n6;
        locals.var_psl_dn7 = assign34070_e39176_d_n7;
        locals.var_psl_dn8 = assign34070_e39176_d_n8;
        locals.var_psl_dn9 = assign34070_e39176_d_n9;
        locals.var_psl_dn10 = assign34070_e39176_d_n10;
        locals.var_psl_dn11 = assign34070_e39176_d_n11;
        locals.var_psl_dn14 = assign34070_e39176_d_n14;
        locals.var_psl_rv = 0.0;

        let (assign34080_e39184, assign34080_e39184_d_n0, assign34080_e39184_d_n2, assign34080_e39184_d_n4, assign34080_e39184_d_n5, assign34080_e39184_d_n6, assign34080_e39184_d_n7, assign34080_e39184_d_n8, assign34080_e39184_d_n9, assign34080_e39184_d_n10, assign34080_e39184_d_n11, assign34080_e39184_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign34080_e39182: f64 = (locals.var_phi_sl_dep - locals.var_phi_s0_dep);
        (assign34080_e39182, (locals.var_phi_sl_dep_dn0 - locals.var_phi_s0_dep_dn0), (locals.var_phi_sl_dep_dn2 - locals.var_phi_s0_dep_dn2), (locals.var_phi_sl_dep_dn4 - locals.var_phi_s0_dep_dn4), (locals.var_phi_sl_dep_dn5 - locals.var_phi_s0_dep_dn5), (locals.var_phi_sl_dep_dn6 - locals.var_phi_s0_dep_dn6), (locals.var_phi_sl_dep_dn7 - locals.var_phi_s0_dep_dn7), (locals.var_phi_sl_dep_dn8 - locals.var_phi_s0_dep_dn8), (locals.var_phi_sl_dep_dn9 - locals.var_phi_s0_dep_dn9), (locals.var_phi_sl_dep_dn10 - locals.var_phi_s0_dep_dn10), (locals.var_phi_sl_dep_dn11 - locals.var_phi_s0_dep_dn11), (locals.var_phi_sl_dep_dn14 - locals.var_phi_s0_dep_dn14),)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn4, locals.var_pds_dn5, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn8, locals.var_pds_dn9, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn14,)
    }
};
        locals.var_pds = assign34080_e39184;
        locals.var_pds_dn0 = assign34080_e39184_d_n0;
        locals.var_pds_dn2 = assign34080_e39184_d_n2;
        locals.var_pds_dn4 = assign34080_e39184_d_n4;
        locals.var_pds_dn5 = assign34080_e39184_d_n5;
        locals.var_pds_dn6 = assign34080_e39184_d_n6;
        locals.var_pds_dn7 = assign34080_e39184_d_n7;
        locals.var_pds_dn8 = assign34080_e39184_d_n8;
        locals.var_pds_dn9 = assign34080_e39184_d_n9;
        locals.var_pds_dn10 = assign34080_e39184_d_n10;
        locals.var_pds_dn11 = assign34080_e39184_d_n11;
        locals.var_pds_dn14 = assign34080_e39184_d_n14;
        locals.var_pds_rv = 0.0;

        let (assign34090_e39193, assign34090_e39193_d_n0, assign34090_e39193_d_n2, assign34090_e39193_d_n4, assign34090_e39193_d_n5, assign34090_e39193_d_n6, assign34090_e39193_d_n7, assign34090_e39193_d_n8, assign34090_e39193_d_n9, assign34090_e39193_d_n10, assign34090_e39193_d_n11, assign34090_e39193_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign34090_e39190: f64 = (locals.var_q_s0 + locals.var_q_sl);
        let assign34090_e39191: f64 = (-assign34090_e39190);
        (assign34090_e39191, (-(locals.var_q_s0_dn0 + locals.var_q_sl_dn0)), (-(locals.var_q_s0_dn2 + locals.var_q_sl_dn2)), (-(locals.var_q_s0_dn4 + locals.var_q_sl_dn4)), (-(locals.var_q_s0_dn5 + locals.var_q_sl_dn5)), (-(locals.var_q_s0_dn6 + locals.var_q_sl_dn6)), (-(locals.var_q_s0_dn7 + locals.var_q_sl_dn7)), (-(locals.var_q_s0_dn8 + locals.var_q_sl_dn8)), (-(locals.var_q_s0_dn9 + locals.var_q_sl_dn9)), (-(locals.var_q_s0_dn10 + locals.var_q_sl_dn10)), (-(locals.var_q_s0_dn11 + locals.var_q_sl_dn11)), (-(locals.var_q_s0_dn14 + locals.var_q_sl_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign34090_e39193;
        locals.var_t1_dn0 = assign34090_e39193_d_n0;
        locals.var_t1_dn2 = assign34090_e39193_d_n2;
        locals.var_t1_dn4 = assign34090_e39193_d_n4;
        locals.var_t1_dn5 = assign34090_e39193_d_n5;
        locals.var_t1_dn6 = assign34090_e39193_d_n6;
        locals.var_t1_dn7 = assign34090_e39193_d_n7;
        locals.var_t1_dn8 = assign34090_e39193_d_n8;
        locals.var_t1_dn9 = assign34090_e39193_d_n9;
        locals.var_t1_dn10 = assign34090_e39193_d_n10;
        locals.var_t1_dn11 = assign34090_e39193_d_n11;
        locals.var_t1_dn14 = assign34090_e39193_d_n14;
        locals.var_t1_rv = 0.0;

        let assign34100_e39197: f64 = locals.var_qn_delta;
        let assign34100_e39202: f64 = if ((locals.var_t1 < assign34100_e39197) && (locals.var_qn_delta >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard802 = assign34100_e39202;
        locals.var_guard802_rv = 0.0;

        let (assign34110_e39214, assign34110_e39214_d_n0, assign34110_e39214_d_n2, assign34110_e39214_d_n4, assign34110_e39214_d_n5, assign34110_e39214_d_n6, assign34110_e39214_d_n7, assign34110_e39214_d_n8, assign34110_e39214_d_n9, assign34110_e39214_d_n10, assign34110_e39214_d_n11, assign34110_e39214_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard802 != 0.0)) {
        let assign34110_e39210: f64 = locals.var_qn_delta;
        let assign34110_e39212: f64 = (assign34110_e39210 - locals.var_t1);
        (assign34110_e39212, (locals.var_qn_delta_dn0 - locals.var_t1_dn0), (locals.var_qn_delta_dn2 - locals.var_t1_dn2), (locals.var_qn_delta_dn4 - locals.var_t1_dn4), (locals.var_qn_delta_dn5 - locals.var_t1_dn5), (locals.var_qn_delta_dn6 - locals.var_t1_dn6), (locals.var_qn_delta_dn7 - locals.var_t1_dn7), (locals.var_qn_delta_dn8 - locals.var_t1_dn8), (locals.var_qn_delta_dn9 - locals.var_t1_dn9), (locals.var_qn_delta_dn10 - locals.var_t1_dn10), (locals.var_qn_delta_dn11 - locals.var_t1_dn11), (locals.var_qn_delta_dn14 - locals.var_t1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign34110_e39214;
        locals.var_tmf1_dn0 = assign34110_e39214_d_n0;
        locals.var_tmf1_dn2 = assign34110_e39214_d_n2;
        locals.var_tmf1_dn4 = assign34110_e39214_d_n4;
        locals.var_tmf1_dn5 = assign34110_e39214_d_n5;
        locals.var_tmf1_dn6 = assign34110_e39214_d_n6;
        locals.var_tmf1_dn7 = assign34110_e39214_d_n7;
        locals.var_tmf1_dn8 = assign34110_e39214_d_n8;
        locals.var_tmf1_dn9 = assign34110_e39214_d_n9;
        locals.var_tmf1_dn10 = assign34110_e39214_d_n10;
        locals.var_tmf1_dn11 = assign34110_e39214_d_n11;
        locals.var_tmf1_dn14 = assign34110_e39214_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign34120_e39224, assign34120_e39224_d_n0, assign34120_e39224_d_n2, assign34120_e39224_d_n4, assign34120_e39224_d_n5, assign34120_e39224_d_n6, assign34120_e39224_d_n7, assign34120_e39224_d_n8, assign34120_e39224_d_n9, assign34120_e39224_d_n10, assign34120_e39224_d_n11, assign34120_e39224_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard802 != 0.0)) {
        let assign34120_e39222: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign34120_e39222, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign34120_e39224;
        locals.var_x2_dn0 = assign34120_e39224_d_n0;
        locals.var_x2_dn2 = assign34120_e39224_d_n2;
        locals.var_x2_dn4 = assign34120_e39224_d_n4;
        locals.var_x2_dn5 = assign34120_e39224_d_n5;
        locals.var_x2_dn6 = assign34120_e39224_d_n6;
        locals.var_x2_dn7 = assign34120_e39224_d_n7;
        locals.var_x2_dn8 = assign34120_e39224_d_n8;
        locals.var_x2_dn9 = assign34120_e39224_d_n9;
        locals.var_x2_dn10 = assign34120_e39224_d_n10;
        locals.var_x2_dn11 = assign34120_e39224_d_n11;
        locals.var_x2_dn14 = assign34120_e39224_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign34130_e39234, assign34130_e39234_d_n0, assign34130_e39234_d_n2, assign34130_e39234_d_n4, assign34130_e39234_d_n5, assign34130_e39234_d_n6, assign34130_e39234_d_n7, assign34130_e39234_d_n8, assign34130_e39234_d_n9, assign34130_e39234_d_n10, assign34130_e39234_d_n11, assign34130_e39234_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard802 != 0.0)) {
        let assign34130_e39232: f64 = (locals.var_qn_delta * locals.var_qn_delta);
        (assign34130_e39232, ((locals.var_qn_delta_dn0 * locals.var_qn_delta) + (locals.var_qn_delta * locals.var_qn_delta_dn0)), ((locals.var_qn_delta_dn2 * locals.var_qn_delta) + (locals.var_qn_delta * locals.var_qn_delta_dn2)), ((locals.var_qn_delta_dn4 * locals.var_qn_delta) + (locals.var_qn_delta * locals.var_qn_delta_dn4)), ((locals.var_qn_delta_dn5 * locals.var_qn_delta) + (locals.var_qn_delta * locals.var_qn_delta_dn5)), ((locals.var_qn_delta_dn6 * locals.var_qn_delta) + (locals.var_qn_delta * locals.var_qn_delta_dn6)), ((locals.var_qn_delta_dn7 * locals.var_qn_delta) + (locals.var_qn_delta * locals.var_qn_delta_dn7)), ((locals.var_qn_delta_dn8 * locals.var_qn_delta) + (locals.var_qn_delta * locals.var_qn_delta_dn8)), ((locals.var_qn_delta_dn9 * locals.var_qn_delta) + (locals.var_qn_delta * locals.var_qn_delta_dn9)), ((locals.var_qn_delta_dn10 * locals.var_qn_delta) + (locals.var_qn_delta * locals.var_qn_delta_dn10)), ((locals.var_qn_delta_dn11 * locals.var_qn_delta) + (locals.var_qn_delta * locals.var_qn_delta_dn11)), ((locals.var_qn_delta_dn14 * locals.var_qn_delta) + (locals.var_qn_delta * locals.var_qn_delta_dn14)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign34130_e39234;
        locals.var_xmax2_dn0 = assign34130_e39234_d_n0;
        locals.var_xmax2_dn2 = assign34130_e39234_d_n2;
        locals.var_xmax2_dn4 = assign34130_e39234_d_n4;
        locals.var_xmax2_dn5 = assign34130_e39234_d_n5;
        locals.var_xmax2_dn6 = assign34130_e39234_d_n6;
        locals.var_xmax2_dn7 = assign34130_e39234_d_n7;
        locals.var_xmax2_dn8 = assign34130_e39234_d_n8;
        locals.var_xmax2_dn9 = assign34130_e39234_d_n9;
        locals.var_xmax2_dn10 = assign34130_e39234_d_n10;
        locals.var_xmax2_dn11 = assign34130_e39234_d_n11;
        locals.var_xmax2_dn14 = assign34130_e39234_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign34140_e39242, assign34140_e39242_d_n0, assign34140_e39242_d_n2, assign34140_e39242_d_n4, assign34140_e39242_d_n5, assign34140_e39242_d_n6, assign34140_e39242_d_n7, assign34140_e39242_d_n8, assign34140_e39242_d_n9, assign34140_e39242_d_n10, assign34140_e39242_d_n11, assign34140_e39242_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard802 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign34140_e39242;
        locals.var_xp_dn0 = assign34140_e39242_d_n0;
        locals.var_xp_dn2 = assign34140_e39242_d_n2;
        locals.var_xp_dn4 = assign34140_e39242_d_n4;
        locals.var_xp_dn5 = assign34140_e39242_d_n5;
        locals.var_xp_dn6 = assign34140_e39242_d_n6;
        locals.var_xp_dn7 = assign34140_e39242_d_n7;
        locals.var_xp_dn8 = assign34140_e39242_d_n8;
        locals.var_xp_dn9 = assign34140_e39242_d_n9;
        locals.var_xp_dn10 = assign34140_e39242_d_n10;
        locals.var_xp_dn11 = assign34140_e39242_d_n11;
        locals.var_xp_dn14 = assign34140_e39242_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign34150_e39250, assign34150_e39250_d_n0, assign34150_e39250_d_n2, assign34150_e39250_d_n4, assign34150_e39250_d_n5, assign34150_e39250_d_n6, assign34150_e39250_d_n7, assign34150_e39250_d_n8, assign34150_e39250_d_n9, assign34150_e39250_d_n10, assign34150_e39250_d_n11, assign34150_e39250_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard802 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign34150_e39250;
        locals.var_xmp_dn0 = assign34150_e39250_d_n0;
        locals.var_xmp_dn2 = assign34150_e39250_d_n2;
        locals.var_xmp_dn4 = assign34150_e39250_d_n4;
        locals.var_xmp_dn5 = assign34150_e39250_d_n5;
        locals.var_xmp_dn6 = assign34150_e39250_d_n6;
        locals.var_xmp_dn7 = assign34150_e39250_d_n7;
        locals.var_xmp_dn8 = assign34150_e39250_d_n8;
        locals.var_xmp_dn9 = assign34150_e39250_d_n9;
        locals.var_xmp_dn10 = assign34150_e39250_d_n10;
        locals.var_xmp_dn11 = assign34150_e39250_d_n11;
        locals.var_xmp_dn14 = assign34150_e39250_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign34160_e39258,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard802 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign34160_e39258;
        locals.var_m0_rv = 0.0;

        let (assign34170_e39266,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard802 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign34170_e39266;
        locals.var_mm_rv = 0.0;

        let (assign34180_e39274, assign34180_e39274_d_n0, assign34180_e39274_d_n2, assign34180_e39274_d_n4, assign34180_e39274_d_n5, assign34180_e39274_d_n6, assign34180_e39274_d_n7, assign34180_e39274_d_n8, assign34180_e39274_d_n9, assign34180_e39274_d_n10, assign34180_e39274_d_n11, assign34180_e39274_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard802 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign34180_e39274;
        locals.var_arg_dn0 = assign34180_e39274_d_n0;
        locals.var_arg_dn2 = assign34180_e39274_d_n2;
        locals.var_arg_dn4 = assign34180_e39274_d_n4;
        locals.var_arg_dn5 = assign34180_e39274_d_n5;
        locals.var_arg_dn6 = assign34180_e39274_d_n6;
        locals.var_arg_dn7 = assign34180_e39274_d_n7;
        locals.var_arg_dn8 = assign34180_e39274_d_n8;
        locals.var_arg_dn9 = assign34180_e39274_d_n9;
        locals.var_arg_dn10 = assign34180_e39274_d_n10;
        locals.var_arg_dn11 = assign34180_e39274_d_n11;
        locals.var_arg_dn14 = assign34180_e39274_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign34190_e39282, assign34190_e39282_d_n0, assign34190_e39282_d_n2, assign34190_e39282_d_n4, assign34190_e39282_d_n5, assign34190_e39282_d_n6, assign34190_e39282_d_n7, assign34190_e39282_d_n8, assign34190_e39282_d_n9, assign34190_e39282_d_n10, assign34190_e39282_d_n11, assign34190_e39282_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard802 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign34190_e39282;
        locals.var_dnm_dn0 = assign34190_e39282_d_n0;
        locals.var_dnm_dn2 = assign34190_e39282_d_n2;
        locals.var_dnm_dn4 = assign34190_e39282_d_n4;
        locals.var_dnm_dn5 = assign34190_e39282_d_n5;
        locals.var_dnm_dn6 = assign34190_e39282_d_n6;
        locals.var_dnm_dn7 = assign34190_e39282_d_n7;
        locals.var_dnm_dn8 = assign34190_e39282_d_n8;
        locals.var_dnm_dn9 = assign34190_e39282_d_n9;
        locals.var_dnm_dn10 = assign34190_e39282_d_n10;
        locals.var_dnm_dn11 = assign34190_e39282_d_n11;
        locals.var_dnm_dn14 = assign34190_e39282_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign34200_e39292, assign34200_e39292_d_n0, assign34200_e39292_d_n2, assign34200_e39292_d_n4, assign34200_e39292_d_n5, assign34200_e39292_d_n6, assign34200_e39292_d_n7, assign34200_e39292_d_n8, assign34200_e39292_d_n9, assign34200_e39292_d_n10, assign34200_e39292_d_n11, assign34200_e39292_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard802 != 0.0)) {
        let assign34200_e39290: f64 = (locals.var_xp * locals.var_x2);
        (assign34200_e39290, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign34200_e39292;
        locals.var_xp_dn0 = assign34200_e39292_d_n0;
        locals.var_xp_dn2 = assign34200_e39292_d_n2;
        locals.var_xp_dn4 = assign34200_e39292_d_n4;
        locals.var_xp_dn5 = assign34200_e39292_d_n5;
        locals.var_xp_dn6 = assign34200_e39292_d_n6;
        locals.var_xp_dn7 = assign34200_e39292_d_n7;
        locals.var_xp_dn8 = assign34200_e39292_d_n8;
        locals.var_xp_dn9 = assign34200_e39292_d_n9;
        locals.var_xp_dn10 = assign34200_e39292_d_n10;
        locals.var_xp_dn11 = assign34200_e39292_d_n11;
        locals.var_xp_dn14 = assign34200_e39292_d_n14;
        locals.var_xp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_111(
        locals: &mut StampLocals,
    ) {
        let (assign34210_e39302, assign34210_e39302_d_n0, assign34210_e39302_d_n2, assign34210_e39302_d_n4, assign34210_e39302_d_n5, assign34210_e39302_d_n6, assign34210_e39302_d_n7, assign34210_e39302_d_n8, assign34210_e39302_d_n9, assign34210_e39302_d_n10, assign34210_e39302_d_n11, assign34210_e39302_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard802 != 0.0)) {
        let assign34210_e39300: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign34210_e39300, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign34210_e39302;
        locals.var_xmp_dn0 = assign34210_e39302_d_n0;
        locals.var_xmp_dn2 = assign34210_e39302_d_n2;
        locals.var_xmp_dn4 = assign34210_e39302_d_n4;
        locals.var_xmp_dn5 = assign34210_e39302_d_n5;
        locals.var_xmp_dn6 = assign34210_e39302_d_n6;
        locals.var_xmp_dn7 = assign34210_e39302_d_n7;
        locals.var_xmp_dn8 = assign34210_e39302_d_n8;
        locals.var_xmp_dn9 = assign34210_e39302_d_n9;
        locals.var_xmp_dn10 = assign34210_e39302_d_n10;
        locals.var_xmp_dn11 = assign34210_e39302_d_n11;
        locals.var_xmp_dn14 = assign34210_e39302_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign34220_e39312, assign34220_e39312_d_n0, assign34220_e39312_d_n2, assign34220_e39312_d_n4, assign34220_e39312_d_n5, assign34220_e39312_d_n6, assign34220_e39312_d_n7, assign34220_e39312_d_n8, assign34220_e39312_d_n9, assign34220_e39312_d_n10, assign34220_e39312_d_n11, assign34220_e39312_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard802 != 0.0)) {
        let assign34220_e39310: f64 = (locals.var_xp * locals.var_x2);
        (assign34220_e39310, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign34220_e39312;
        locals.var_xp_dn0 = assign34220_e39312_d_n0;
        locals.var_xp_dn2 = assign34220_e39312_d_n2;
        locals.var_xp_dn4 = assign34220_e39312_d_n4;
        locals.var_xp_dn5 = assign34220_e39312_d_n5;
        locals.var_xp_dn6 = assign34220_e39312_d_n6;
        locals.var_xp_dn7 = assign34220_e39312_d_n7;
        locals.var_xp_dn8 = assign34220_e39312_d_n8;
        locals.var_xp_dn9 = assign34220_e39312_d_n9;
        locals.var_xp_dn10 = assign34220_e39312_d_n10;
        locals.var_xp_dn11 = assign34220_e39312_d_n11;
        locals.var_xp_dn14 = assign34220_e39312_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign34230_e39322, assign34230_e39322_d_n0, assign34230_e39322_d_n2, assign34230_e39322_d_n4, assign34230_e39322_d_n5, assign34230_e39322_d_n6, assign34230_e39322_d_n7, assign34230_e39322_d_n8, assign34230_e39322_d_n9, assign34230_e39322_d_n10, assign34230_e39322_d_n11, assign34230_e39322_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard802 != 0.0)) {
        let assign34230_e39320: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign34230_e39320, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign34230_e39322;
        locals.var_xmp_dn0 = assign34230_e39322_d_n0;
        locals.var_xmp_dn2 = assign34230_e39322_d_n2;
        locals.var_xmp_dn4 = assign34230_e39322_d_n4;
        locals.var_xmp_dn5 = assign34230_e39322_d_n5;
        locals.var_xmp_dn6 = assign34230_e39322_d_n6;
        locals.var_xmp_dn7 = assign34230_e39322_d_n7;
        locals.var_xmp_dn8 = assign34230_e39322_d_n8;
        locals.var_xmp_dn9 = assign34230_e39322_d_n9;
        locals.var_xmp_dn10 = assign34230_e39322_d_n10;
        locals.var_xmp_dn11 = assign34230_e39322_d_n11;
        locals.var_xmp_dn14 = assign34230_e39322_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign34240_e39332, assign34240_e39332_d_n0, assign34240_e39332_d_n2, assign34240_e39332_d_n4, assign34240_e39332_d_n5, assign34240_e39332_d_n6, assign34240_e39332_d_n7, assign34240_e39332_d_n8, assign34240_e39332_d_n9, assign34240_e39332_d_n10, assign34240_e39332_d_n11, assign34240_e39332_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard802 != 0.0)) {
        let assign34240_e39330: f64 = (locals.var_xp + locals.var_xmp);
        (assign34240_e39330, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign34240_e39332;
        locals.var_arg_dn0 = assign34240_e39332_d_n0;
        locals.var_arg_dn2 = assign34240_e39332_d_n2;
        locals.var_arg_dn4 = assign34240_e39332_d_n4;
        locals.var_arg_dn5 = assign34240_e39332_d_n5;
        locals.var_arg_dn6 = assign34240_e39332_d_n6;
        locals.var_arg_dn7 = assign34240_e39332_d_n7;
        locals.var_arg_dn8 = assign34240_e39332_d_n8;
        locals.var_arg_dn9 = assign34240_e39332_d_n9;
        locals.var_arg_dn10 = assign34240_e39332_d_n10;
        locals.var_arg_dn11 = assign34240_e39332_d_n11;
        locals.var_arg_dn14 = assign34240_e39332_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign34250_e39340, assign34250_e39340_d_n0, assign34250_e39340_d_n2, assign34250_e39340_d_n4, assign34250_e39340_d_n5, assign34250_e39340_d_n6, assign34250_e39340_d_n7, assign34250_e39340_d_n8, assign34250_e39340_d_n9, assign34250_e39340_d_n10, assign34250_e39340_d_n11, assign34250_e39340_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard802 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign34250_e39340;
        locals.var_dnm_dn0 = assign34250_e39340_d_n0;
        locals.var_dnm_dn2 = assign34250_e39340_d_n2;
        locals.var_dnm_dn4 = assign34250_e39340_d_n4;
        locals.var_dnm_dn5 = assign34250_e39340_d_n5;
        locals.var_dnm_dn6 = assign34250_e39340_d_n6;
        locals.var_dnm_dn7 = assign34250_e39340_d_n7;
        locals.var_dnm_dn8 = assign34250_e39340_d_n8;
        locals.var_dnm_dn9 = assign34250_e39340_d_n9;
        locals.var_dnm_dn10 = assign34250_e39340_d_n10;
        locals.var_dnm_dn11 = assign34250_e39340_d_n11;
        locals.var_dnm_dn14 = assign34250_e39340_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign34260_e39355: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard803 = assign34260_e39355;
        locals.var_guard803_rv = 0.0;

        let assign34270_e39358: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard804 = assign34270_e39358;
        locals.var_guard804_rv = 0.0;

        let (assign34280_e39370,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard802 != 0.0)) && (locals.var_guard803 != 0.0)) && (locals.var_guard804 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign34280_e39370;
        locals.var_mm_rv = 0.0;

        let assign34290_e39373: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard805 = assign34290_e39373;
        locals.var_guard805_rv = 0.0;

        let (assign34300_e39388,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard802 != 0.0)) && (locals.var_guard803 != 0.0)) && (locals.var_guard804 == 0.0)) && (locals.var_guard805 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign34300_e39388;
        locals.var_mm_rv = 0.0;

        let assign34310_e39391: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard806 = assign34310_e39391;
        locals.var_guard806_rv = 0.0;

        let (assign34320_e39409,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard802 != 0.0)) && (locals.var_guard803 != 0.0)) && (locals.var_guard804 == 0.0)) && (locals.var_guard805 == 0.0)) && (locals.var_guard806 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign34320_e39409;
        locals.var_mm_rv = 0.0;

        let assign34330_e39412: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard807 = assign34330_e39412;
        locals.var_guard807_rv = 0.0;

        let (assign34340_e39433,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard802 != 0.0)) && (locals.var_guard803 != 0.0)) && (locals.var_guard804 == 0.0)) && (locals.var_guard805 == 0.0)) && (locals.var_guard806 == 0.0)) && (locals.var_guard807 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign34340_e39433;
        locals.var_mm_rv = 0.0;

        let (assign34350_e39443,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard802 != 0.0)) && (locals.var_guard803 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign34350_e39443;
        locals.var_m0_rv = 0.0;

        let mut assign34360_loop_guard: usize = 0;
        while {
            let assign34360_cond_e39454: f64 = if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard802 != 0.0)) && (locals.var_guard803 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign34360_cond_e39454 != 0.0
        } {
            assign34360_loop_guard += 1;
            assert!(assign34360_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign34360_body0_e39465, assign34360_body0_e39465_d_n0, assign34360_body0_e39465_d_n2, assign34360_body0_e39465_d_n4, assign34360_body0_e39465_d_n5, assign34360_body0_e39465_d_n6, assign34360_body0_e39465_d_n7, assign34360_body0_e39465_d_n8, assign34360_body0_e39465_d_n9, assign34360_body0_e39465_d_n10, assign34360_body0_e39465_d_n11, assign34360_body0_e39465_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard802 != 0.0)) && (locals.var_guard803 != 0.0)) {
        let assign34360_body0_e39463: f64 = (locals.var_dnm).sqrt();
        (assign34360_body0_e39463, (locals.var_dnm_dn0 / (2.0 * assign34360_body0_e39463)), (locals.var_dnm_dn2 / (2.0 * assign34360_body0_e39463)), (locals.var_dnm_dn4 / (2.0 * assign34360_body0_e39463)), (locals.var_dnm_dn5 / (2.0 * assign34360_body0_e39463)), (locals.var_dnm_dn6 / (2.0 * assign34360_body0_e39463)), (locals.var_dnm_dn7 / (2.0 * assign34360_body0_e39463)), (locals.var_dnm_dn8 / (2.0 * assign34360_body0_e39463)), (locals.var_dnm_dn9 / (2.0 * assign34360_body0_e39463)), (locals.var_dnm_dn10 / (2.0 * assign34360_body0_e39463)), (locals.var_dnm_dn11 / (2.0 * assign34360_body0_e39463)), (locals.var_dnm_dn14 / (2.0 * assign34360_body0_e39463)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign34360_body0_e39465;
            locals.var_dnm_dn0 = assign34360_body0_e39465_d_n0;
            locals.var_dnm_dn2 = assign34360_body0_e39465_d_n2;
            locals.var_dnm_dn4 = assign34360_body0_e39465_d_n4;
            locals.var_dnm_dn5 = assign34360_body0_e39465_d_n5;
            locals.var_dnm_dn6 = assign34360_body0_e39465_d_n6;
            locals.var_dnm_dn7 = assign34360_body0_e39465_d_n7;
            locals.var_dnm_dn8 = assign34360_body0_e39465_d_n8;
            locals.var_dnm_dn9 = assign34360_body0_e39465_d_n9;
            locals.var_dnm_dn10 = assign34360_body0_e39465_d_n10;
            locals.var_dnm_dn11 = assign34360_body0_e39465_d_n11;
            locals.var_dnm_dn14 = assign34360_body0_e39465_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign34360_body1_e39477,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard802 != 0.0)) && (locals.var_guard803 != 0.0)) {
        let assign34360_body1_e39475: f64 = (locals.var_m0 + 1.0);
        (assign34360_body1_e39475,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign34360_body1_e39477;
            locals.var_m0_rv = 0.0;
        }

        let (assign34370_e39499, assign34370_e39499_d_n0, assign34370_e39499_d_n2, assign34370_e39499_d_n4, assign34370_e39499_d_n5, assign34370_e39499_d_n6, assign34370_e39499_d_n7, assign34370_e39499_d_n8, assign34370_e39499_d_n9, assign34370_e39499_d_n10, assign34370_e39499_d_n11, assign34370_e39499_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard802 != 0.0)) && (locals.var_guard803 == 0.0)) {
        let (assign34370_e39497, assign34370_e39497_d_n0, assign34370_e39497_d_n2, assign34370_e39497_d_n4, assign34370_e39497_d_n5, assign34370_e39497_d_n6, assign34370_e39497_d_n7, assign34370_e39497_d_n8, assign34370_e39497_d_n9, assign34370_e39497_d_n10, assign34370_e39497_d_n11, assign34370_e39497_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign34370_e39494: f64 = (2.0 * 2.0);
                let assign34370_e39495: f64 = (1.0 / assign34370_e39494);
                let assign34370_e39496: f64 = (locals.var_dnm).powf(assign34370_e39495);
                (assign34370_e39496, if 0.0 == 0.0 && ((assign34370_e39495) as f64).is_finite() && ((assign34370_e39495) as f64).fract() == 0.0 { if assign34370_e39495 == 0.0 { 0.0 } else { (assign34370_e39495 * ((locals.var_dnm).powf(assign34370_e39495 - 1.0) * locals.var_dnm_dn0)) } } else { (assign34370_e39496 * (assign34370_e39495 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign34370_e39495) as f64).is_finite() && ((assign34370_e39495) as f64).fract() == 0.0 { if assign34370_e39495 == 0.0 { 0.0 } else { (assign34370_e39495 * ((locals.var_dnm).powf(assign34370_e39495 - 1.0) * locals.var_dnm_dn2)) } } else { (assign34370_e39496 * (assign34370_e39495 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign34370_e39495) as f64).is_finite() && ((assign34370_e39495) as f64).fract() == 0.0 { if assign34370_e39495 == 0.0 { 0.0 } else { (assign34370_e39495 * ((locals.var_dnm).powf(assign34370_e39495 - 1.0) * locals.var_dnm_dn4)) } } else { (assign34370_e39496 * (assign34370_e39495 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign34370_e39495) as f64).is_finite() && ((assign34370_e39495) as f64).fract() == 0.0 { if assign34370_e39495 == 0.0 { 0.0 } else { (assign34370_e39495 * ((locals.var_dnm).powf(assign34370_e39495 - 1.0) * locals.var_dnm_dn5)) } } else { (assign34370_e39496 * (assign34370_e39495 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign34370_e39495) as f64).is_finite() && ((assign34370_e39495) as f64).fract() == 0.0 { if assign34370_e39495 == 0.0 { 0.0 } else { (assign34370_e39495 * ((locals.var_dnm).powf(assign34370_e39495 - 1.0) * locals.var_dnm_dn6)) } } else { (assign34370_e39496 * (assign34370_e39495 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign34370_e39495) as f64).is_finite() && ((assign34370_e39495) as f64).fract() == 0.0 { if assign34370_e39495 == 0.0 { 0.0 } else { (assign34370_e39495 * ((locals.var_dnm).powf(assign34370_e39495 - 1.0) * locals.var_dnm_dn7)) } } else { (assign34370_e39496 * (assign34370_e39495 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign34370_e39495) as f64).is_finite() && ((assign34370_e39495) as f64).fract() == 0.0 { if assign34370_e39495 == 0.0 { 0.0 } else { (assign34370_e39495 * ((locals.var_dnm).powf(assign34370_e39495 - 1.0) * locals.var_dnm_dn8)) } } else { (assign34370_e39496 * (assign34370_e39495 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign34370_e39495) as f64).is_finite() && ((assign34370_e39495) as f64).fract() == 0.0 { if assign34370_e39495 == 0.0 { 0.0 } else { (assign34370_e39495 * ((locals.var_dnm).powf(assign34370_e39495 - 1.0) * locals.var_dnm_dn9)) } } else { (assign34370_e39496 * (assign34370_e39495 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign34370_e39495) as f64).is_finite() && ((assign34370_e39495) as f64).fract() == 0.0 { if assign34370_e39495 == 0.0 { 0.0 } else { (assign34370_e39495 * ((locals.var_dnm).powf(assign34370_e39495 - 1.0) * locals.var_dnm_dn10)) } } else { (assign34370_e39496 * (assign34370_e39495 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign34370_e39495) as f64).is_finite() && ((assign34370_e39495) as f64).fract() == 0.0 { if assign34370_e39495 == 0.0 { 0.0 } else { (assign34370_e39495 * ((locals.var_dnm).powf(assign34370_e39495 - 1.0) * locals.var_dnm_dn11)) } } else { (assign34370_e39496 * (assign34370_e39495 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign34370_e39495) as f64).is_finite() && ((assign34370_e39495) as f64).fract() == 0.0 { if assign34370_e39495 == 0.0 { 0.0 } else { (assign34370_e39495 * ((locals.var_dnm).powf(assign34370_e39495 - 1.0) * locals.var_dnm_dn14)) } } else { (assign34370_e39496 * (assign34370_e39495 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign34370_e39497, assign34370_e39497_d_n0, assign34370_e39497_d_n2, assign34370_e39497_d_n4, assign34370_e39497_d_n5, assign34370_e39497_d_n6, assign34370_e39497_d_n7, assign34370_e39497_d_n8, assign34370_e39497_d_n9, assign34370_e39497_d_n10, assign34370_e39497_d_n11, assign34370_e39497_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign34370_e39499;
        locals.var_dnm_dn0 = assign34370_e39499_d_n0;
        locals.var_dnm_dn2 = assign34370_e39499_d_n2;
        locals.var_dnm_dn4 = assign34370_e39499_d_n4;
        locals.var_dnm_dn5 = assign34370_e39499_d_n5;
        locals.var_dnm_dn6 = assign34370_e39499_d_n6;
        locals.var_dnm_dn7 = assign34370_e39499_d_n7;
        locals.var_dnm_dn8 = assign34370_e39499_d_n8;
        locals.var_dnm_dn9 = assign34370_e39499_d_n9;
        locals.var_dnm_dn10 = assign34370_e39499_d_n10;
        locals.var_dnm_dn11 = assign34370_e39499_d_n11;
        locals.var_dnm_dn14 = assign34370_e39499_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign34380_e39509, assign34380_e39509_d_n0, assign34380_e39509_d_n2, assign34380_e39509_d_n4, assign34380_e39509_d_n5, assign34380_e39509_d_n6, assign34380_e39509_d_n7, assign34380_e39509_d_n8, assign34380_e39509_d_n9, assign34380_e39509_d_n10, assign34380_e39509_d_n11, assign34380_e39509_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard802 != 0.0)) {
        let assign34380_e39507: f64 = (1.0 / locals.var_dnm);
        (assign34380_e39507, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign34380_e39509;
        locals.var_dnm_dn0 = assign34380_e39509_d_n0;
        locals.var_dnm_dn2 = assign34380_e39509_d_n2;
        locals.var_dnm_dn4 = assign34380_e39509_d_n4;
        locals.var_dnm_dn5 = assign34380_e39509_d_n5;
        locals.var_dnm_dn6 = assign34380_e39509_d_n6;
        locals.var_dnm_dn7 = assign34380_e39509_d_n7;
        locals.var_dnm_dn8 = assign34380_e39509_d_n8;
        locals.var_dnm_dn9 = assign34380_e39509_d_n9;
        locals.var_dnm_dn10 = assign34380_e39509_d_n10;
        locals.var_dnm_dn11 = assign34380_e39509_d_n11;
        locals.var_dnm_dn14 = assign34380_e39509_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign34390_e39521, assign34390_e39521_d_n0, assign34390_e39521_d_n2, assign34390_e39521_d_n4, assign34390_e39521_d_n5, assign34390_e39521_d_n6, assign34390_e39521_d_n7, assign34390_e39521_d_n8, assign34390_e39521_d_n9, assign34390_e39521_d_n10, assign34390_e39521_d_n11, assign34390_e39521_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard802 != 0.0)) {
        let assign34390_e39517: f64 = (locals.var_tmf1 * locals.var_qn_delta);
        let assign34390_e39519: f64 = (assign34390_e39517 * locals.var_dnm);
        (assign34390_e39519, ((((locals.var_tmf1_dn0 * locals.var_qn_delta) + (locals.var_tmf1 * locals.var_qn_delta_dn0)) * locals.var_dnm) + (assign34390_e39517 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_qn_delta) + (locals.var_tmf1 * locals.var_qn_delta_dn2)) * locals.var_dnm) + (assign34390_e39517 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_qn_delta) + (locals.var_tmf1 * locals.var_qn_delta_dn4)) * locals.var_dnm) + (assign34390_e39517 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_qn_delta) + (locals.var_tmf1 * locals.var_qn_delta_dn5)) * locals.var_dnm) + (assign34390_e39517 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_qn_delta) + (locals.var_tmf1 * locals.var_qn_delta_dn6)) * locals.var_dnm) + (assign34390_e39517 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_qn_delta) + (locals.var_tmf1 * locals.var_qn_delta_dn7)) * locals.var_dnm) + (assign34390_e39517 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_qn_delta) + (locals.var_tmf1 * locals.var_qn_delta_dn8)) * locals.var_dnm) + (assign34390_e39517 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_qn_delta) + (locals.var_tmf1 * locals.var_qn_delta_dn9)) * locals.var_dnm) + (assign34390_e39517 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_qn_delta) + (locals.var_tmf1 * locals.var_qn_delta_dn10)) * locals.var_dnm) + (assign34390_e39517 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_qn_delta) + (locals.var_tmf1 * locals.var_qn_delta_dn11)) * locals.var_dnm) + (assign34390_e39517 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * locals.var_qn_delta) + (locals.var_tmf1 * locals.var_qn_delta_dn14)) * locals.var_dnm) + (assign34390_e39517 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign34390_e39521;
        locals.var_tmf0_dn0 = assign34390_e39521_d_n0;
        locals.var_tmf0_dn2 = assign34390_e39521_d_n2;
        locals.var_tmf0_dn4 = assign34390_e39521_d_n4;
        locals.var_tmf0_dn5 = assign34390_e39521_d_n5;
        locals.var_tmf0_dn6 = assign34390_e39521_d_n6;
        locals.var_tmf0_dn7 = assign34390_e39521_d_n7;
        locals.var_tmf0_dn8 = assign34390_e39521_d_n8;
        locals.var_tmf0_dn9 = assign34390_e39521_d_n9;
        locals.var_tmf0_dn10 = assign34390_e39521_d_n10;
        locals.var_tmf0_dn11 = assign34390_e39521_d_n11;
        locals.var_tmf0_dn14 = assign34390_e39521_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign34400_e39535, assign34400_e39535_d_n0, assign34400_e39535_d_n2, assign34400_e39535_d_n4, assign34400_e39535_d_n5, assign34400_e39535_d_n6, assign34400_e39535_d_n7, assign34400_e39535_d_n8, assign34400_e39535_d_n9, assign34400_e39535_d_n10, assign34400_e39535_d_n11, assign34400_e39535_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard802 != 0.0)) {
        let assign34400_e39529: f64 = (locals.var_qn_delta * locals.var_xmp);
        let assign34400_e39531: f64 = (assign34400_e39529 * locals.var_dnm);
        let assign34400_e39533: f64 = (assign34400_e39531 / locals.var_arg);
        (assign34400_e39533, (((((((locals.var_qn_delta_dn0 * locals.var_xmp) + (locals.var_qn_delta * locals.var_xmp_dn0)) * locals.var_dnm) + (assign34400_e39529 * locals.var_dnm_dn0)) * locals.var_arg) - (assign34400_e39531 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_qn_delta_dn2 * locals.var_xmp) + (locals.var_qn_delta * locals.var_xmp_dn2)) * locals.var_dnm) + (assign34400_e39529 * locals.var_dnm_dn2)) * locals.var_arg) - (assign34400_e39531 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_qn_delta_dn4 * locals.var_xmp) + (locals.var_qn_delta * locals.var_xmp_dn4)) * locals.var_dnm) + (assign34400_e39529 * locals.var_dnm_dn4)) * locals.var_arg) - (assign34400_e39531 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_qn_delta_dn5 * locals.var_xmp) + (locals.var_qn_delta * locals.var_xmp_dn5)) * locals.var_dnm) + (assign34400_e39529 * locals.var_dnm_dn5)) * locals.var_arg) - (assign34400_e39531 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_qn_delta_dn6 * locals.var_xmp) + (locals.var_qn_delta * locals.var_xmp_dn6)) * locals.var_dnm) + (assign34400_e39529 * locals.var_dnm_dn6)) * locals.var_arg) - (assign34400_e39531 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_qn_delta_dn7 * locals.var_xmp) + (locals.var_qn_delta * locals.var_xmp_dn7)) * locals.var_dnm) + (assign34400_e39529 * locals.var_dnm_dn7)) * locals.var_arg) - (assign34400_e39531 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_qn_delta_dn8 * locals.var_xmp) + (locals.var_qn_delta * locals.var_xmp_dn8)) * locals.var_dnm) + (assign34400_e39529 * locals.var_dnm_dn8)) * locals.var_arg) - (assign34400_e39531 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_qn_delta_dn9 * locals.var_xmp) + (locals.var_qn_delta * locals.var_xmp_dn9)) * locals.var_dnm) + (assign34400_e39529 * locals.var_dnm_dn9)) * locals.var_arg) - (assign34400_e39531 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_qn_delta_dn10 * locals.var_xmp) + (locals.var_qn_delta * locals.var_xmp_dn10)) * locals.var_dnm) + (assign34400_e39529 * locals.var_dnm_dn10)) * locals.var_arg) - (assign34400_e39531 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_qn_delta_dn11 * locals.var_xmp) + (locals.var_qn_delta * locals.var_xmp_dn11)) * locals.var_dnm) + (assign34400_e39529 * locals.var_dnm_dn11)) * locals.var_arg) - (assign34400_e39531 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_qn_delta_dn14 * locals.var_xmp) + (locals.var_qn_delta * locals.var_xmp_dn14)) * locals.var_dnm) + (assign34400_e39529 * locals.var_dnm_dn14)) * locals.var_arg) - (assign34400_e39531 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign34400_e39535;
        locals.var_t0_dn0 = assign34400_e39535_d_n0;
        locals.var_t0_dn2 = assign34400_e39535_d_n2;
        locals.var_t0_dn4 = assign34400_e39535_d_n4;
        locals.var_t0_dn5 = assign34400_e39535_d_n5;
        locals.var_t0_dn6 = assign34400_e39535_d_n6;
        locals.var_t0_dn7 = assign34400_e39535_d_n7;
        locals.var_t0_dn8 = assign34400_e39535_d_n8;
        locals.var_t0_dn9 = assign34400_e39535_d_n9;
        locals.var_t0_dn10 = assign34400_e39535_d_n10;
        locals.var_t0_dn11 = assign34400_e39535_d_n11;
        locals.var_t0_dn14 = assign34400_e39535_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign34410_e39547, assign34410_e39547_d_n0, assign34410_e39547_d_n2, assign34410_e39547_d_n4, assign34410_e39547_d_n5, assign34410_e39547_d_n6, assign34410_e39547_d_n7, assign34410_e39547_d_n8, assign34410_e39547_d_n9, assign34410_e39547_d_n10, assign34410_e39547_d_n11, assign34410_e39547_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard802 != 0.0)) {
        let assign34410_e39543: f64 = locals.var_qn_delta;
        let assign34410_e39545: f64 = (assign34410_e39543 - locals.var_tmf0);
        (assign34410_e39545, (locals.var_qn_delta_dn0 - locals.var_tmf0_dn0), (locals.var_qn_delta_dn2 - locals.var_tmf0_dn2), (locals.var_qn_delta_dn4 - locals.var_tmf0_dn4), (locals.var_qn_delta_dn5 - locals.var_tmf0_dn5), (locals.var_qn_delta_dn6 - locals.var_tmf0_dn6), (locals.var_qn_delta_dn7 - locals.var_tmf0_dn7), (locals.var_qn_delta_dn8 - locals.var_tmf0_dn8), (locals.var_qn_delta_dn9 - locals.var_tmf0_dn9), (locals.var_qn_delta_dn10 - locals.var_tmf0_dn10), (locals.var_qn_delta_dn11 - locals.var_tmf0_dn11), (locals.var_qn_delta_dn14 - locals.var_tmf0_dn14),)
    } else {
        (locals.var_qn_drift, locals.var_qn_drift_dn0, locals.var_qn_drift_dn2, locals.var_qn_drift_dn4, locals.var_qn_drift_dn5, locals.var_qn_drift_dn6, locals.var_qn_drift_dn7, locals.var_qn_drift_dn8, locals.var_qn_drift_dn9, locals.var_qn_drift_dn10, locals.var_qn_drift_dn11, locals.var_qn_drift_dn14,)
    }
};
        locals.var_qn_drift = assign34410_e39547;
        locals.var_qn_drift_dn0 = assign34410_e39547_d_n0;
        locals.var_qn_drift_dn2 = assign34410_e39547_d_n2;
        locals.var_qn_drift_dn4 = assign34410_e39547_d_n4;
        locals.var_qn_drift_dn5 = assign34410_e39547_d_n5;
        locals.var_qn_drift_dn6 = assign34410_e39547_d_n6;
        locals.var_qn_drift_dn7 = assign34410_e39547_d_n7;
        locals.var_qn_drift_dn8 = assign34410_e39547_d_n8;
        locals.var_qn_drift_dn9 = assign34410_e39547_d_n9;
        locals.var_qn_drift_dn10 = assign34410_e39547_d_n10;
        locals.var_qn_drift_dn11 = assign34410_e39547_d_n11;
        locals.var_qn_drift_dn14 = assign34410_e39547_d_n14;
        locals.var_qn_drift_rv = 0.0;

        let (assign34420_e39555, assign34420_e39555_d_n0, assign34420_e39555_d_n2, assign34420_e39555_d_n4, assign34420_e39555_d_n5, assign34420_e39555_d_n6, assign34420_e39555_d_n7, assign34420_e39555_d_n8, assign34420_e39555_d_n9, assign34420_e39555_d_n10, assign34420_e39555_d_n11, assign34420_e39555_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard802 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign34420_e39555;
        locals.var_t0_dn0 = assign34420_e39555_d_n0;
        locals.var_t0_dn2 = assign34420_e39555_d_n2;
        locals.var_t0_dn4 = assign34420_e39555_d_n4;
        locals.var_t0_dn5 = assign34420_e39555_d_n5;
        locals.var_t0_dn6 = assign34420_e39555_d_n6;
        locals.var_t0_dn7 = assign34420_e39555_d_n7;
        locals.var_t0_dn8 = assign34420_e39555_d_n8;
        locals.var_t0_dn9 = assign34420_e39555_d_n9;
        locals.var_t0_dn10 = assign34420_e39555_d_n10;
        locals.var_t0_dn11 = assign34420_e39555_d_n11;
        locals.var_t0_dn14 = assign34420_e39555_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign34430_e39564, assign34430_e39564_d_n0, assign34430_e39564_d_n2, assign34430_e39564_d_n4, assign34430_e39564_d_n5, assign34430_e39564_d_n6, assign34430_e39564_d_n7, assign34430_e39564_d_n8, assign34430_e39564_d_n9, assign34430_e39564_d_n10, assign34430_e39564_d_n11, assign34430_e39564_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard802 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_qn_drift, locals.var_qn_drift_dn0, locals.var_qn_drift_dn2, locals.var_qn_drift_dn4, locals.var_qn_drift_dn5, locals.var_qn_drift_dn6, locals.var_qn_drift_dn7, locals.var_qn_drift_dn8, locals.var_qn_drift_dn9, locals.var_qn_drift_dn10, locals.var_qn_drift_dn11, locals.var_qn_drift_dn14,)
    }
};
        locals.var_qn_drift = assign34430_e39564;
        locals.var_qn_drift_dn0 = assign34430_e39564_d_n0;
        locals.var_qn_drift_dn2 = assign34430_e39564_d_n2;
        locals.var_qn_drift_dn4 = assign34430_e39564_d_n4;
        locals.var_qn_drift_dn5 = assign34430_e39564_d_n5;
        locals.var_qn_drift_dn6 = assign34430_e39564_d_n6;
        locals.var_qn_drift_dn7 = assign34430_e39564_d_n7;
        locals.var_qn_drift_dn8 = assign34430_e39564_d_n8;
        locals.var_qn_drift_dn9 = assign34430_e39564_d_n9;
        locals.var_qn_drift_dn10 = assign34430_e39564_d_n10;
        locals.var_qn_drift_dn11 = assign34430_e39564_d_n11;
        locals.var_qn_drift_dn14 = assign34430_e39564_d_n14;
        locals.var_qn_drift_rv = 0.0;

        let (assign34440_e39573, assign34440_e39573_d_n0, assign34440_e39573_d_n2, assign34440_e39573_d_n4, assign34440_e39573_d_n5, assign34440_e39573_d_n6, assign34440_e39573_d_n7, assign34440_e39573_d_n8, assign34440_e39573_d_n9, assign34440_e39573_d_n10, assign34440_e39573_d_n11, assign34440_e39573_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard802 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign34440_e39573;
        locals.var_t0_dn0 = assign34440_e39573_d_n0;
        locals.var_t0_dn2 = assign34440_e39573_d_n2;
        locals.var_t0_dn4 = assign34440_e39573_d_n4;
        locals.var_t0_dn5 = assign34440_e39573_d_n5;
        locals.var_t0_dn6 = assign34440_e39573_d_n6;
        locals.var_t0_dn7 = assign34440_e39573_d_n7;
        locals.var_t0_dn8 = assign34440_e39573_d_n8;
        locals.var_t0_dn9 = assign34440_e39573_d_n9;
        locals.var_t0_dn10 = assign34440_e39573_d_n10;
        locals.var_t0_dn11 = assign34440_e39573_d_n11;
        locals.var_t0_dn14 = assign34440_e39573_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign34450_e39585, assign34450_e39585_d_n0, assign34450_e39585_d_n2, assign34450_e39585_d_n4, assign34450_e39585_d_n5, assign34450_e39585_d_n6, assign34450_e39585_d_n7, assign34450_e39585_d_n8, assign34450_e39585_d_n9, assign34450_e39585_d_n10, assign34450_e39585_d_n11, assign34450_e39585_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign34450_e39579: f64 = (locals.var_beta * locals.var_qn_drift);
        let assign34450_e39581: f64 = (assign34450_e39579 / 2.0);
        let assign34450_e39583: f64 = (assign34450_e39581 * locals.var_pds);
        (assign34450_e39583, (((((locals.var_beta_dn0 * locals.var_qn_drift) + (locals.var_beta * locals.var_qn_drift_dn0)) / 2.0) * locals.var_pds) + (assign34450_e39581 * locals.var_pds_dn0)), (((((locals.var_beta_dn2 * locals.var_qn_drift) + (locals.var_beta * locals.var_qn_drift_dn2)) / 2.0) * locals.var_pds) + (assign34450_e39581 * locals.var_pds_dn2)), (((((locals.var_beta_dn4 * locals.var_qn_drift) + (locals.var_beta * locals.var_qn_drift_dn4)) / 2.0) * locals.var_pds) + (assign34450_e39581 * locals.var_pds_dn4)), (((((locals.var_beta_dn5 * locals.var_qn_drift) + (locals.var_beta * locals.var_qn_drift_dn5)) / 2.0) * locals.var_pds) + (assign34450_e39581 * locals.var_pds_dn5)), (((((locals.var_beta_dn6 * locals.var_qn_drift) + (locals.var_beta * locals.var_qn_drift_dn6)) / 2.0) * locals.var_pds) + (assign34450_e39581 * locals.var_pds_dn6)), (((((locals.var_beta_dn7 * locals.var_qn_drift) + (locals.var_beta * locals.var_qn_drift_dn7)) / 2.0) * locals.var_pds) + (assign34450_e39581 * locals.var_pds_dn7)), (((((locals.var_beta_dn8 * locals.var_qn_drift) + (locals.var_beta * locals.var_qn_drift_dn8)) / 2.0) * locals.var_pds) + (assign34450_e39581 * locals.var_pds_dn8)), (((((locals.var_beta_dn9 * locals.var_qn_drift) + (locals.var_beta * locals.var_qn_drift_dn9)) / 2.0) * locals.var_pds) + (assign34450_e39581 * locals.var_pds_dn9)), (((((locals.var_beta_dn10 * locals.var_qn_drift) + (locals.var_beta * locals.var_qn_drift_dn10)) / 2.0) * locals.var_pds) + (assign34450_e39581 * locals.var_pds_dn10)), (((((locals.var_beta_dn11 * locals.var_qn_drift) + (locals.var_beta * locals.var_qn_drift_dn11)) / 2.0) * locals.var_pds) + (assign34450_e39581 * locals.var_pds_dn11)), (((((locals.var_beta_dn14 * locals.var_qn_drift) + (locals.var_beta * locals.var_qn_drift_dn14)) / 2.0) * locals.var_pds) + (assign34450_e39581 * locals.var_pds_dn14)),)
    } else {
        (locals.var_idd_drift, locals.var_idd_drift_dn0, locals.var_idd_drift_dn2, locals.var_idd_drift_dn4, locals.var_idd_drift_dn5, locals.var_idd_drift_dn6, locals.var_idd_drift_dn7, locals.var_idd_drift_dn8, locals.var_idd_drift_dn9, locals.var_idd_drift_dn10, locals.var_idd_drift_dn11, locals.var_idd_drift_dn14,)
    }
};
        locals.var_idd_drift = assign34450_e39585;
        locals.var_idd_drift_dn0 = assign34450_e39585_d_n0;
        locals.var_idd_drift_dn2 = assign34450_e39585_d_n2;
        locals.var_idd_drift_dn4 = assign34450_e39585_d_n4;
        locals.var_idd_drift_dn5 = assign34450_e39585_d_n5;
        locals.var_idd_drift_dn6 = assign34450_e39585_d_n6;
        locals.var_idd_drift_dn7 = assign34450_e39585_d_n7;
        locals.var_idd_drift_dn8 = assign34450_e39585_d_n8;
        locals.var_idd_drift_dn9 = assign34450_e39585_d_n9;
        locals.var_idd_drift_dn10 = assign34450_e39585_d_n10;
        locals.var_idd_drift_dn11 = assign34450_e39585_d_n11;
        locals.var_idd_drift_dn14 = assign34450_e39585_d_n14;
        locals.var_idd_drift_rv = 0.0;

        let (assign34460_e39595, assign34460_e39595_d_n0, assign34460_e39595_d_n2, assign34460_e39595_d_n4, assign34460_e39595_d_n5, assign34460_e39595_d_n6, assign34460_e39595_d_n7, assign34460_e39595_d_n8, assign34460_e39595_d_n9, assign34460_e39595_d_n10, assign34460_e39595_d_n11, assign34460_e39595_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign34460_e39590: f64 = (-locals.var_q_nl_cur);
        let assign34460_e39592: f64 = (assign34460_e39590 + locals.var_q_n0_cur);
        let assign34460_e39593: f64 = (-assign34460_e39592);
        (assign34460_e39593, (-((-locals.var_q_nl_cur_dn0) + locals.var_q_n0_cur_dn0)), (-((-locals.var_q_nl_cur_dn2) + locals.var_q_n0_cur_dn2)), (-((-locals.var_q_nl_cur_dn4) + locals.var_q_n0_cur_dn4)), (-((-locals.var_q_nl_cur_dn5) + locals.var_q_n0_cur_dn5)), (-((-locals.var_q_nl_cur_dn6) + locals.var_q_n0_cur_dn6)), (-((-locals.var_q_nl_cur_dn7) + locals.var_q_n0_cur_dn7)), (-((-locals.var_q_nl_cur_dn8) + locals.var_q_n0_cur_dn8)), (-((-locals.var_q_nl_cur_dn9) + locals.var_q_n0_cur_dn9)), (-((-locals.var_q_nl_cur_dn10) + locals.var_q_n0_cur_dn10)), (-((-locals.var_q_nl_cur_dn11) + locals.var_q_n0_cur_dn11)), (-((-locals.var_q_nl_cur_dn14) + locals.var_q_n0_cur_dn14)),)
    } else {
        (locals.var_idd_diffu, locals.var_idd_diffu_dn0, locals.var_idd_diffu_dn2, locals.var_idd_diffu_dn4, locals.var_idd_diffu_dn5, locals.var_idd_diffu_dn6, locals.var_idd_diffu_dn7, locals.var_idd_diffu_dn8, locals.var_idd_diffu_dn9, locals.var_idd_diffu_dn10, locals.var_idd_diffu_dn11, locals.var_idd_diffu_dn14,)
    }
};
        locals.var_idd_diffu = assign34460_e39595;
        locals.var_idd_diffu_dn0 = assign34460_e39595_d_n0;
        locals.var_idd_diffu_dn2 = assign34460_e39595_d_n2;
        locals.var_idd_diffu_dn4 = assign34460_e39595_d_n4;
        locals.var_idd_diffu_dn5 = assign34460_e39595_d_n5;
        locals.var_idd_diffu_dn6 = assign34460_e39595_d_n6;
        locals.var_idd_diffu_dn7 = assign34460_e39595_d_n7;
        locals.var_idd_diffu_dn8 = assign34460_e39595_d_n8;
        locals.var_idd_diffu_dn9 = assign34460_e39595_d_n9;
        locals.var_idd_diffu_dn10 = assign34460_e39595_d_n10;
        locals.var_idd_diffu_dn11 = assign34460_e39595_d_n11;
        locals.var_idd_diffu_dn14 = assign34460_e39595_d_n14;
        locals.var_idd_diffu_rv = 0.0;

        let (assign34470_e39603, assign34470_e39603_d_n0, assign34470_e39603_d_n2, assign34470_e39603_d_n4, assign34470_e39603_d_n5, assign34470_e39603_d_n6, assign34470_e39603_d_n7, assign34470_e39603_d_n8, assign34470_e39603_d_n9, assign34470_e39603_d_n10, assign34470_e39603_d_n11, assign34470_e39603_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign34470_e39601: f64 = (locals.var_idd_drift + locals.var_idd_diffu);
        (assign34470_e39601, (locals.var_idd_drift_dn0 + locals.var_idd_diffu_dn0), (locals.var_idd_drift_dn2 + locals.var_idd_diffu_dn2), (locals.var_idd_drift_dn4 + locals.var_idd_diffu_dn4), (locals.var_idd_drift_dn5 + locals.var_idd_diffu_dn5), (locals.var_idd_drift_dn6 + locals.var_idd_diffu_dn6), (locals.var_idd_drift_dn7 + locals.var_idd_diffu_dn7), (locals.var_idd_drift_dn8 + locals.var_idd_diffu_dn8), (locals.var_idd_drift_dn9 + locals.var_idd_diffu_dn9), (locals.var_idd_drift_dn10 + locals.var_idd_diffu_dn10), (locals.var_idd_drift_dn11 + locals.var_idd_diffu_dn11), (locals.var_idd_drift_dn14 + locals.var_idd_diffu_dn14),)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn4, locals.var_idd_dn5, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn8, locals.var_idd_dn9, locals.var_idd_dn10, locals.var_idd_dn11, locals.var_idd_dn14,)
    }
};
        locals.var_idd = assign34470_e39603;
        locals.var_idd_dn0 = assign34470_e39603_d_n0;
        locals.var_idd_dn2 = assign34470_e39603_d_n2;
        locals.var_idd_dn4 = assign34470_e39603_d_n4;
        locals.var_idd_dn5 = assign34470_e39603_d_n5;
        locals.var_idd_dn6 = assign34470_e39603_d_n6;
        locals.var_idd_dn7 = assign34470_e39603_d_n7;
        locals.var_idd_dn8 = assign34470_e39603_d_n8;
        locals.var_idd_dn9 = assign34470_e39603_d_n9;
        locals.var_idd_dn10 = assign34470_e39603_d_n10;
        locals.var_idd_dn11 = assign34470_e39603_d_n11;
        locals.var_idd_dn14 = assign34470_e39603_d_n14;
        locals.var_idd_rv = 0.0;

        let (assign34480_e39610, assign34480_e39610_d_n0, assign34480_e39610_d_n2, assign34480_e39610_d_n4, assign34480_e39610_d_n5, assign34480_e39610_d_n6, assign34480_e39610_d_n7, assign34480_e39610_d_n8, assign34480_e39610_d_n9, assign34480_e39610_d_n10, assign34480_e39610_d_n11, assign34480_e39610_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign34480_e39608: f64 = (-locals.var_q_n0_cur);
        (assign34480_e39608, (-locals.var_q_n0_cur_dn0), (-locals.var_q_n0_cur_dn2), (-locals.var_q_n0_cur_dn4), (-locals.var_q_n0_cur_dn5), (-locals.var_q_n0_cur_dn6), (-locals.var_q_n0_cur_dn7), (-locals.var_q_n0_cur_dn8), (-locals.var_q_n0_cur_dn9), (-locals.var_q_n0_cur_dn10), (-locals.var_q_n0_cur_dn11), (-locals.var_q_n0_cur_dn14),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn14,)
    }
};
        locals.var_qiu = assign34480_e39610;
        locals.var_qiu_dn0 = assign34480_e39610_d_n0;
        locals.var_qiu_dn2 = assign34480_e39610_d_n2;
        locals.var_qiu_dn4 = assign34480_e39610_d_n4;
        locals.var_qiu_dn5 = assign34480_e39610_d_n5;
        locals.var_qiu_dn6 = assign34480_e39610_d_n6;
        locals.var_qiu_dn7 = assign34480_e39610_d_n7;
        locals.var_qiu_dn8 = assign34480_e39610_d_n8;
        locals.var_qiu_dn9 = assign34480_e39610_d_n9;
        locals.var_qiu_dn10 = assign34480_e39610_d_n10;
        locals.var_qiu_dn11 = assign34480_e39610_d_n11;
        locals.var_qiu_dn14 = assign34480_e39610_d_n14;
        locals.var_qiu_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_112(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign34490_e39616, assign34490_e39616_d_n0, assign34490_e39616_d_n2, assign34490_e39616_d_n4, assign34490_e39616_d_n5, assign34490_e39616_d_n6, assign34490_e39616_d_n7, assign34490_e39616_d_n8, assign34490_e39616_d_n9, assign34490_e39616_d_n10, assign34490_e39616_d_n11, assign34490_e39616_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        (locals.var_leff, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn4, locals.var_lch_dn5, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn8, locals.var_lch_dn9, locals.var_lch_dn10, locals.var_lch_dn11, locals.var_lch_dn14,)
    }
};
        locals.var_lch = assign34490_e39616;
        locals.var_lch_dn0 = assign34490_e39616_d_n0;
        locals.var_lch_dn2 = assign34490_e39616_d_n2;
        locals.var_lch_dn4 = assign34490_e39616_d_n4;
        locals.var_lch_dn5 = assign34490_e39616_d_n5;
        locals.var_lch_dn6 = assign34490_e39616_d_n6;
        locals.var_lch_dn7 = assign34490_e39616_d_n7;
        locals.var_lch_dn8 = assign34490_e39616_d_n8;
        locals.var_lch_dn9 = assign34490_e39616_d_n9;
        locals.var_lch_dn10 = assign34490_e39616_d_n10;
        locals.var_lch_dn11 = assign34490_e39616_d_n11;
        locals.var_lch_dn14 = assign34490_e39616_d_n14;
        locals.var_lch_rv = 0.0;

        let (assign34500_e39624, assign34500_e39624_d_n0, assign34500_e39624_d_n2, assign34500_e39624_d_n4, assign34500_e39624_d_n5, assign34500_e39624_d_n6, assign34500_e39624_d_n7, assign34500_e39624_d_n8, assign34500_e39624_d_n9, assign34500_e39624_d_n10, assign34500_e39624_d_n11, assign34500_e39624_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign34500_e39622: f64 = (locals.var_ninv_o_esi / 100.0);
        (assign34500_e39622, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign34500_e39624;
        locals.var_t2_dn0 = assign34500_e39624_d_n0;
        locals.var_t2_dn2 = assign34500_e39624_d_n2;
        locals.var_t2_dn4 = assign34500_e39624_d_n4;
        locals.var_t2_dn5 = assign34500_e39624_d_n5;
        locals.var_t2_dn6 = assign34500_e39624_d_n6;
        locals.var_t2_dn7 = assign34500_e39624_d_n7;
        locals.var_t2_dn8 = assign34500_e39624_d_n8;
        locals.var_t2_dn9 = assign34500_e39624_d_n9;
        locals.var_t2_dn10 = assign34500_e39624_d_n10;
        locals.var_t2_dn11 = assign34500_e39624_d_n11;
        locals.var_t2_dn14 = assign34500_e39624_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign34510_e39636, assign34510_e39636_d_n0, assign34510_e39636_d_n2, assign34510_e39636_d_n4, assign34510_e39636_d_n5, assign34510_e39636_d_n6, assign34510_e39636_d_n7, assign34510_e39636_d_n8, assign34510_e39636_d_n9, assign34510_e39636_d_n10, assign34510_e39636_d_n11, assign34510_e39636_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign34510_e39631: f64 = (locals.var_phi_sl_dep - locals.var_phi_s0_dep);
        let assign34510_e39633: f64 = (assign34510_e39631 * locals.var_ninvde);
        let assign34510_e39634: f64 = (1.0 + assign34510_e39633);
        (assign34510_e39634, (((locals.var_phi_sl_dep_dn0 - locals.var_phi_s0_dep_dn0) * locals.var_ninvde) + (assign34510_e39631 * locals.var_ninvde_dn0)), (((locals.var_phi_sl_dep_dn2 - locals.var_phi_s0_dep_dn2) * locals.var_ninvde) + (assign34510_e39631 * locals.var_ninvde_dn2)), (((locals.var_phi_sl_dep_dn4 - locals.var_phi_s0_dep_dn4) * locals.var_ninvde) + (assign34510_e39631 * locals.var_ninvde_dn4)), (((locals.var_phi_sl_dep_dn5 - locals.var_phi_s0_dep_dn5) * locals.var_ninvde) + (assign34510_e39631 * locals.var_ninvde_dn5)), (((locals.var_phi_sl_dep_dn6 - locals.var_phi_s0_dep_dn6) * locals.var_ninvde) + (assign34510_e39631 * locals.var_ninvde_dn6)), (((locals.var_phi_sl_dep_dn7 - locals.var_phi_s0_dep_dn7) * locals.var_ninvde) + (assign34510_e39631 * locals.var_ninvde_dn7)), (((locals.var_phi_sl_dep_dn8 - locals.var_phi_s0_dep_dn8) * locals.var_ninvde) + (assign34510_e39631 * locals.var_ninvde_dn8)), (((locals.var_phi_sl_dep_dn9 - locals.var_phi_s0_dep_dn9) * locals.var_ninvde) + (assign34510_e39631 * locals.var_ninvde_dn9)), (((locals.var_phi_sl_dep_dn10 - locals.var_phi_s0_dep_dn10) * locals.var_ninvde) + (assign34510_e39631 * locals.var_ninvde_dn10)), (((locals.var_phi_sl_dep_dn11 - locals.var_phi_s0_dep_dn11) * locals.var_ninvde) + (assign34510_e39631 * locals.var_ninvde_dn11)), (((locals.var_phi_sl_dep_dn14 - locals.var_phi_s0_dep_dn14) * locals.var_ninvde) + (assign34510_e39631 * locals.var_ninvde_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign34510_e39636;
        locals.var_t4_dn0 = assign34510_e39636_d_n0;
        locals.var_t4_dn2 = assign34510_e39636_d_n2;
        locals.var_t4_dn4 = assign34510_e39636_d_n4;
        locals.var_t4_dn5 = assign34510_e39636_d_n5;
        locals.var_t4_dn6 = assign34510_e39636_d_n6;
        locals.var_t4_dn7 = assign34510_e39636_d_n7;
        locals.var_t4_dn8 = assign34510_e39636_d_n8;
        locals.var_t4_dn9 = assign34510_e39636_d_n9;
        locals.var_t4_dn10 = assign34510_e39636_d_n10;
        locals.var_t4_dn11 = assign34510_e39636_d_n11;
        locals.var_t4_dn14 = assign34510_e39636_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign34520_e39644, assign34520_e39644_d_n0, assign34520_e39644_d_n2, assign34520_e39644_d_n4, assign34520_e39644_d_n5, assign34520_e39644_d_n6, assign34520_e39644_d_n7, assign34520_e39644_d_n8, assign34520_e39644_d_n9, assign34520_e39644_d_n10, assign34520_e39644_d_n11, assign34520_e39644_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign34520_e39642: f64 = (locals.var_t2 * locals.var_qiu);
        (assign34520_e39642, ((locals.var_t2_dn0 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn0)), ((locals.var_t2_dn2 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn2)), ((locals.var_t2_dn4 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn4)), ((locals.var_t2_dn5 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn5)), ((locals.var_t2_dn6 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn6)), ((locals.var_t2_dn7 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn7)), ((locals.var_t2_dn8 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn8)), ((locals.var_t2_dn9 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn9)), ((locals.var_t2_dn10 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn10)), ((locals.var_t2_dn11 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn11)), ((locals.var_t2_dn14 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign34520_e39644;
        locals.var_t5_dn0 = assign34520_e39644_d_n0;
        locals.var_t5_dn2 = assign34520_e39644_d_n2;
        locals.var_t5_dn4 = assign34520_e39644_d_n4;
        locals.var_t5_dn5 = assign34520_e39644_d_n5;
        locals.var_t5_dn6 = assign34520_e39644_d_n6;
        locals.var_t5_dn7 = assign34520_e39644_d_n7;
        locals.var_t5_dn8 = assign34520_e39644_d_n8;
        locals.var_t5_dn9 = assign34520_e39644_d_n9;
        locals.var_t5_dn10 = assign34520_e39644_d_n10;
        locals.var_t5_dn11 = assign34520_e39644_d_n11;
        locals.var_t5_dn14 = assign34520_e39644_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign34530_e39652, assign34530_e39652_d_n0, assign34530_e39652_d_n2, assign34530_e39652_d_n4, assign34530_e39652_d_n5, assign34530_e39652_d_n6, assign34530_e39652_d_n7, assign34530_e39652_d_n8, assign34530_e39652_d_n9, assign34530_e39652_d_n10, assign34530_e39652_d_n11, assign34530_e39652_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign34530_e39650: f64 = (locals.var_t5 / locals.var_t4);
        (assign34530_e39650, (((locals.var_t5_dn0 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn2 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn4 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn5 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn6 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn7 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn8 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn9 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn10 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn11 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn14 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign34530_e39652;
        locals.var_t3_dn0 = assign34530_e39652_d_n0;
        locals.var_t3_dn2 = assign34530_e39652_d_n2;
        locals.var_t3_dn4 = assign34530_e39652_d_n4;
        locals.var_t3_dn5 = assign34530_e39652_d_n5;
        locals.var_t3_dn6 = assign34530_e39652_d_n6;
        locals.var_t3_dn7 = assign34530_e39652_d_n7;
        locals.var_t3_dn8 = assign34530_e39652_d_n8;
        locals.var_t3_dn9 = assign34530_e39652_d_n9;
        locals.var_t3_dn10 = assign34530_e39652_d_n10;
        locals.var_t3_dn11 = assign34530_e39652_d_n11;
        locals.var_t3_dn14 = assign34530_e39652_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign34540_e39658, assign34540_e39658_d_n0, assign34540_e39658_d_n2, assign34540_e39658_d_n4, assign34540_e39658_d_n5, assign34540_e39658_d_n6, assign34540_e39658_d_n7, assign34540_e39658_d_n8, assign34540_e39658_d_n9, assign34540_e39658_d_n10, assign34540_e39658_d_n11, assign34540_e39658_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    } else {
        (locals.var_eeff, locals.var_eeff_dn0, locals.var_eeff_dn2, locals.var_eeff_dn4, locals.var_eeff_dn5, locals.var_eeff_dn6, locals.var_eeff_dn7, locals.var_eeff_dn8, locals.var_eeff_dn9, locals.var_eeff_dn10, locals.var_eeff_dn11, locals.var_eeff_dn14,)
    }
};
        locals.var_eeff = assign34540_e39658;
        locals.var_eeff_dn0 = assign34540_e39658_d_n0;
        locals.var_eeff_dn2 = assign34540_e39658_d_n2;
        locals.var_eeff_dn4 = assign34540_e39658_d_n4;
        locals.var_eeff_dn5 = assign34540_e39658_d_n5;
        locals.var_eeff_dn6 = assign34540_e39658_d_n6;
        locals.var_eeff_dn7 = assign34540_e39658_d_n7;
        locals.var_eeff_dn8 = assign34540_e39658_d_n8;
        locals.var_eeff_dn9 = assign34540_e39658_d_n9;
        locals.var_eeff_dn10 = assign34540_e39658_d_n10;
        locals.var_eeff_dn11 = assign34540_e39658_d_n11;
        locals.var_eeff_dn14 = assign34540_e39658_d_n14;
        locals.var_eeff_rv = 0.0;

        let (assign34550_e39673, assign34550_e39673_d_n0, assign34550_e39673_d_n2, assign34550_e39673_d_n4, assign34550_e39673_d_n5, assign34550_e39673_d_n6, assign34550_e39673_d_n7, assign34550_e39673_d_n8, assign34550_e39673_d_n9, assign34550_e39673_d_n10, assign34550_e39673_d_n11, assign34550_e39673_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let (assign34550_e39671, assign34550_e39671_d_n0, assign34550_e39671_d_n2, assign34550_e39671_d_n4, assign34550_e39671_d_n5, assign34550_e39671_d_n6, assign34550_e39671_d_n7, assign34550_e39671_d_n8, assign34550_e39671_d_n9, assign34550_e39671_d_n10, assign34550_e39671_d_n11, assign34550_e39671_d_n14,) = {
            if (locals.var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign34550_e39669: f64 = (p.p160 - 1.0);
                let assign34550_e39670: f64 = (locals.var_eeff).powf(assign34550_e39669);
                (assign34550_e39670, if 0.0 == 0.0 && ((assign34550_e39669) as f64).is_finite() && ((assign34550_e39669) as f64).fract() == 0.0 { if assign34550_e39669 == 0.0 { 0.0 } else { (assign34550_e39669 * ((locals.var_eeff).powf(assign34550_e39669 - 1.0) * locals.var_eeff_dn0)) } } else { (assign34550_e39670 * (assign34550_e39669 * (locals.var_eeff_dn0 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34550_e39669) as f64).is_finite() && ((assign34550_e39669) as f64).fract() == 0.0 { if assign34550_e39669 == 0.0 { 0.0 } else { (assign34550_e39669 * ((locals.var_eeff).powf(assign34550_e39669 - 1.0) * locals.var_eeff_dn2)) } } else { (assign34550_e39670 * (assign34550_e39669 * (locals.var_eeff_dn2 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34550_e39669) as f64).is_finite() && ((assign34550_e39669) as f64).fract() == 0.0 { if assign34550_e39669 == 0.0 { 0.0 } else { (assign34550_e39669 * ((locals.var_eeff).powf(assign34550_e39669 - 1.0) * locals.var_eeff_dn4)) } } else { (assign34550_e39670 * (assign34550_e39669 * (locals.var_eeff_dn4 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34550_e39669) as f64).is_finite() && ((assign34550_e39669) as f64).fract() == 0.0 { if assign34550_e39669 == 0.0 { 0.0 } else { (assign34550_e39669 * ((locals.var_eeff).powf(assign34550_e39669 - 1.0) * locals.var_eeff_dn5)) } } else { (assign34550_e39670 * (assign34550_e39669 * (locals.var_eeff_dn5 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34550_e39669) as f64).is_finite() && ((assign34550_e39669) as f64).fract() == 0.0 { if assign34550_e39669 == 0.0 { 0.0 } else { (assign34550_e39669 * ((locals.var_eeff).powf(assign34550_e39669 - 1.0) * locals.var_eeff_dn6)) } } else { (assign34550_e39670 * (assign34550_e39669 * (locals.var_eeff_dn6 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34550_e39669) as f64).is_finite() && ((assign34550_e39669) as f64).fract() == 0.0 { if assign34550_e39669 == 0.0 { 0.0 } else { (assign34550_e39669 * ((locals.var_eeff).powf(assign34550_e39669 - 1.0) * locals.var_eeff_dn7)) } } else { (assign34550_e39670 * (assign34550_e39669 * (locals.var_eeff_dn7 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34550_e39669) as f64).is_finite() && ((assign34550_e39669) as f64).fract() == 0.0 { if assign34550_e39669 == 0.0 { 0.0 } else { (assign34550_e39669 * ((locals.var_eeff).powf(assign34550_e39669 - 1.0) * locals.var_eeff_dn8)) } } else { (assign34550_e39670 * (assign34550_e39669 * (locals.var_eeff_dn8 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34550_e39669) as f64).is_finite() && ((assign34550_e39669) as f64).fract() == 0.0 { if assign34550_e39669 == 0.0 { 0.0 } else { (assign34550_e39669 * ((locals.var_eeff).powf(assign34550_e39669 - 1.0) * locals.var_eeff_dn9)) } } else { (assign34550_e39670 * (assign34550_e39669 * (locals.var_eeff_dn9 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34550_e39669) as f64).is_finite() && ((assign34550_e39669) as f64).fract() == 0.0 { if assign34550_e39669 == 0.0 { 0.0 } else { (assign34550_e39669 * ((locals.var_eeff).powf(assign34550_e39669 - 1.0) * locals.var_eeff_dn10)) } } else { (assign34550_e39670 * (assign34550_e39669 * (locals.var_eeff_dn10 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34550_e39669) as f64).is_finite() && ((assign34550_e39669) as f64).fract() == 0.0 { if assign34550_e39669 == 0.0 { 0.0 } else { (assign34550_e39669 * ((locals.var_eeff).powf(assign34550_e39669 - 1.0) * locals.var_eeff_dn11)) } } else { (assign34550_e39670 * (assign34550_e39669 * (locals.var_eeff_dn11 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34550_e39669) as f64).is_finite() && ((assign34550_e39669) as f64).fract() == 0.0 { if assign34550_e39669 == 0.0 { 0.0 } else { (assign34550_e39669 * ((locals.var_eeff).powf(assign34550_e39669 - 1.0) * locals.var_eeff_dn14)) } } else { (assign34550_e39670 * (assign34550_e39669 * (locals.var_eeff_dn14 / locals.var_eeff))) },)
            }
        };
        (assign34550_e39671, assign34550_e39671_d_n0, assign34550_e39671_d_n2, assign34550_e39671_d_n4, assign34550_e39671_d_n5, assign34550_e39671_d_n6, assign34550_e39671_d_n7, assign34550_e39671_d_n8, assign34550_e39671_d_n9, assign34550_e39671_d_n10, assign34550_e39671_d_n11, assign34550_e39671_d_n14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign34550_e39673;
        locals.var_t5_dn0 = assign34550_e39673_d_n0;
        locals.var_t5_dn2 = assign34550_e39673_d_n2;
        locals.var_t5_dn4 = assign34550_e39673_d_n4;
        locals.var_t5_dn5 = assign34550_e39673_d_n5;
        locals.var_t5_dn6 = assign34550_e39673_d_n6;
        locals.var_t5_dn7 = assign34550_e39673_d_n7;
        locals.var_t5_dn8 = assign34550_e39673_d_n8;
        locals.var_t5_dn9 = assign34550_e39673_d_n9;
        locals.var_t5_dn10 = assign34550_e39673_d_n10;
        locals.var_t5_dn11 = assign34550_e39673_d_n11;
        locals.var_t5_dn14 = assign34550_e39673_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign34560_e39681, assign34560_e39681_d_n0, assign34560_e39681_d_n2, assign34560_e39681_d_n4, assign34560_e39681_d_n5, assign34560_e39681_d_n6, assign34560_e39681_d_n7, assign34560_e39681_d_n8, assign34560_e39681_d_n9, assign34560_e39681_d_n10, assign34560_e39681_d_n11, assign34560_e39681_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign34560_e39679: f64 = (locals.var_t5 * locals.var_eeff);
        (assign34560_e39679, ((locals.var_t5_dn0 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn0)), ((locals.var_t5_dn2 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn2)), ((locals.var_t5_dn4 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn4)), ((locals.var_t5_dn5 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn5)), ((locals.var_t5_dn6 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn6)), ((locals.var_t5_dn7 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn7)), ((locals.var_t5_dn8 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn8)), ((locals.var_t5_dn9 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn9)), ((locals.var_t5_dn10 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn10)), ((locals.var_t5_dn11 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn11)), ((locals.var_t5_dn14 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn14)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign34560_e39681;
        locals.var_t8_dn0 = assign34560_e39681_d_n0;
        locals.var_t8_dn2 = assign34560_e39681_d_n2;
        locals.var_t8_dn4 = assign34560_e39681_d_n4;
        locals.var_t8_dn5 = assign34560_e39681_d_n5;
        locals.var_t8_dn6 = assign34560_e39681_d_n6;
        locals.var_t8_dn7 = assign34560_e39681_d_n7;
        locals.var_t8_dn8 = assign34560_e39681_d_n8;
        locals.var_t8_dn9 = assign34560_e39681_d_n9;
        locals.var_t8_dn10 = assign34560_e39681_d_n10;
        locals.var_t8_dn11 = assign34560_e39681_d_n11;
        locals.var_t8_dn14 = assign34560_e39681_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign34570_e39696, assign34570_e39696_d_n0, assign34570_e39696_d_n2, assign34570_e39696_d_n4, assign34570_e39696_d_n5, assign34570_e39696_d_n6, assign34570_e39696_d_n7, assign34570_e39696_d_n8, assign34570_e39696_d_n9, assign34570_e39696_d_n10, assign34570_e39696_d_n11, assign34570_e39696_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let (assign34570_e39694, assign34570_e39694_d_n0, assign34570_e39694_d_n2, assign34570_e39694_d_n4, assign34570_e39694_d_n5, assign34570_e39694_d_n6, assign34570_e39694_d_n7, assign34570_e39694_d_n8, assign34570_e39694_d_n9, assign34570_e39694_d_n10, assign34570_e39694_d_n11, assign34570_e39694_d_n14,) = {
            if (locals.var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign34570_e39692: f64 = (locals.var_muesr - 1.0);
                let assign34570_e39693: f64 = (locals.var_eeff).powf(assign34570_e39692);
                (assign34570_e39693, if 0.0 == 0.0 && ((assign34570_e39692) as f64).is_finite() && ((assign34570_e39692) as f64).fract() == 0.0 { if assign34570_e39692 == 0.0 { 0.0 } else { (assign34570_e39692 * ((locals.var_eeff).powf(assign34570_e39692 - 1.0) * locals.var_eeff_dn0)) } } else { (assign34570_e39693 * (assign34570_e39692 * (locals.var_eeff_dn0 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34570_e39692) as f64).is_finite() && ((assign34570_e39692) as f64).fract() == 0.0 { if assign34570_e39692 == 0.0 { 0.0 } else { (assign34570_e39692 * ((locals.var_eeff).powf(assign34570_e39692 - 1.0) * locals.var_eeff_dn2)) } } else { (assign34570_e39693 * (assign34570_e39692 * (locals.var_eeff_dn2 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34570_e39692) as f64).is_finite() && ((assign34570_e39692) as f64).fract() == 0.0 { if assign34570_e39692 == 0.0 { 0.0 } else { (assign34570_e39692 * ((locals.var_eeff).powf(assign34570_e39692 - 1.0) * locals.var_eeff_dn4)) } } else { (assign34570_e39693 * (assign34570_e39692 * (locals.var_eeff_dn4 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34570_e39692) as f64).is_finite() && ((assign34570_e39692) as f64).fract() == 0.0 { if assign34570_e39692 == 0.0 { 0.0 } else { (assign34570_e39692 * ((locals.var_eeff).powf(assign34570_e39692 - 1.0) * locals.var_eeff_dn5)) } } else { (assign34570_e39693 * (assign34570_e39692 * (locals.var_eeff_dn5 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34570_e39692) as f64).is_finite() && ((assign34570_e39692) as f64).fract() == 0.0 { if assign34570_e39692 == 0.0 { 0.0 } else { (assign34570_e39692 * ((locals.var_eeff).powf(assign34570_e39692 - 1.0) * locals.var_eeff_dn6)) } } else { (assign34570_e39693 * (assign34570_e39692 * (locals.var_eeff_dn6 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34570_e39692) as f64).is_finite() && ((assign34570_e39692) as f64).fract() == 0.0 { if assign34570_e39692 == 0.0 { 0.0 } else { (assign34570_e39692 * ((locals.var_eeff).powf(assign34570_e39692 - 1.0) * locals.var_eeff_dn7)) } } else { (assign34570_e39693 * (assign34570_e39692 * (locals.var_eeff_dn7 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34570_e39692) as f64).is_finite() && ((assign34570_e39692) as f64).fract() == 0.0 { if assign34570_e39692 == 0.0 { 0.0 } else { (assign34570_e39692 * ((locals.var_eeff).powf(assign34570_e39692 - 1.0) * locals.var_eeff_dn8)) } } else { (assign34570_e39693 * (assign34570_e39692 * (locals.var_eeff_dn8 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34570_e39692) as f64).is_finite() && ((assign34570_e39692) as f64).fract() == 0.0 { if assign34570_e39692 == 0.0 { 0.0 } else { (assign34570_e39692 * ((locals.var_eeff).powf(assign34570_e39692 - 1.0) * locals.var_eeff_dn9)) } } else { (assign34570_e39693 * (assign34570_e39692 * (locals.var_eeff_dn9 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34570_e39692) as f64).is_finite() && ((assign34570_e39692) as f64).fract() == 0.0 { if assign34570_e39692 == 0.0 { 0.0 } else { (assign34570_e39692 * ((locals.var_eeff).powf(assign34570_e39692 - 1.0) * locals.var_eeff_dn10)) } } else { (assign34570_e39693 * (assign34570_e39692 * (locals.var_eeff_dn10 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34570_e39692) as f64).is_finite() && ((assign34570_e39692) as f64).fract() == 0.0 { if assign34570_e39692 == 0.0 { 0.0 } else { (assign34570_e39692 * ((locals.var_eeff).powf(assign34570_e39692 - 1.0) * locals.var_eeff_dn11)) } } else { (assign34570_e39693 * (assign34570_e39692 * (locals.var_eeff_dn11 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34570_e39692) as f64).is_finite() && ((assign34570_e39692) as f64).fract() == 0.0 { if assign34570_e39692 == 0.0 { 0.0 } else { (assign34570_e39692 * ((locals.var_eeff).powf(assign34570_e39692 - 1.0) * locals.var_eeff_dn14)) } } else { (assign34570_e39693 * (assign34570_e39692 * (locals.var_eeff_dn14 / locals.var_eeff))) },)
            }
        };
        (assign34570_e39694, assign34570_e39694_d_n0, assign34570_e39694_d_n2, assign34570_e39694_d_n4, assign34570_e39694_d_n5, assign34570_e39694_d_n6, assign34570_e39694_d_n7, assign34570_e39694_d_n8, assign34570_e39694_d_n9, assign34570_e39694_d_n10, assign34570_e39694_d_n11, assign34570_e39694_d_n14,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign34570_e39696;
        locals.var_t7_dn0 = assign34570_e39696_d_n0;
        locals.var_t7_dn2 = assign34570_e39696_d_n2;
        locals.var_t7_dn4 = assign34570_e39696_d_n4;
        locals.var_t7_dn5 = assign34570_e39696_d_n5;
        locals.var_t7_dn6 = assign34570_e39696_d_n6;
        locals.var_t7_dn7 = assign34570_e39696_d_n7;
        locals.var_t7_dn8 = assign34570_e39696_d_n8;
        locals.var_t7_dn9 = assign34570_e39696_d_n9;
        locals.var_t7_dn10 = assign34570_e39696_d_n10;
        locals.var_t7_dn11 = assign34570_e39696_d_n11;
        locals.var_t7_dn14 = assign34570_e39696_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign34580_e39704, assign34580_e39704_d_n0, assign34580_e39704_d_n2, assign34580_e39704_d_n4, assign34580_e39704_d_n5, assign34580_e39704_d_n6, assign34580_e39704_d_n7, assign34580_e39704_d_n8, assign34580_e39704_d_n9, assign34580_e39704_d_n10, assign34580_e39704_d_n11, assign34580_e39704_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign34580_e39702: f64 = (locals.var_t7 * locals.var_eeff);
        (assign34580_e39702, ((locals.var_t7_dn0 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn0)), ((locals.var_t7_dn2 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn2)), ((locals.var_t7_dn4 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn4)), ((locals.var_t7_dn5 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn5)), ((locals.var_t7_dn6 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn6)), ((locals.var_t7_dn7 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn7)), ((locals.var_t7_dn8 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn8)), ((locals.var_t7_dn9 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn9)), ((locals.var_t7_dn10 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn10)), ((locals.var_t7_dn11 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn11)), ((locals.var_t7_dn14 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign34580_e39704;
        locals.var_t6_dn0 = assign34580_e39704_d_n0;
        locals.var_t6_dn2 = assign34580_e39704_d_n2;
        locals.var_t6_dn4 = assign34580_e39704_d_n4;
        locals.var_t6_dn5 = assign34580_e39704_d_n5;
        locals.var_t6_dn6 = assign34580_e39704_d_n6;
        locals.var_t6_dn7 = assign34580_e39704_d_n7;
        locals.var_t6_dn8 = assign34580_e39704_d_n8;
        locals.var_t6_dn9 = assign34580_e39704_d_n9;
        locals.var_t6_dn10 = assign34580_e39704_d_n10;
        locals.var_t6_dn11 = assign34580_e39704_d_n11;
        locals.var_t6_dn14 = assign34580_e39704_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign34590_e39712, assign34590_e39712_d_n0, assign34590_e39712_d_n2, assign34590_e39712_d_n4, assign34590_e39712_d_n5, assign34590_e39712_d_n6, assign34590_e39712_d_n7, assign34590_e39712_d_n8, assign34590_e39712_d_n9, assign34590_e39712_d_n10, assign34590_e39712_d_n11, assign34590_e39712_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign34590_e39710: f64 = (1.6021918e-19 * 10000.0);
        (assign34590_e39710, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign34590_e39712;
        locals.var_t9_dn0 = assign34590_e39712_d_n0;
        locals.var_t9_dn2 = assign34590_e39712_d_n2;
        locals.var_t9_dn4 = assign34590_e39712_d_n4;
        locals.var_t9_dn5 = assign34590_e39712_d_n5;
        locals.var_t9_dn6 = assign34590_e39712_d_n6;
        locals.var_t9_dn7 = assign34590_e39712_d_n7;
        locals.var_t9_dn8 = assign34590_e39712_d_n8;
        locals.var_t9_dn9 = assign34590_e39712_d_n9;
        locals.var_t9_dn10 = assign34590_e39712_d_n10;
        locals.var_t9_dn11 = assign34590_e39712_d_n11;
        locals.var_t9_dn14 = assign34590_e39712_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign34600_e39720, assign34600_e39720_d_n0, assign34600_e39720_d_n2, assign34600_e39720_d_n4, assign34600_e39720_d_n5, assign34600_e39720_d_n6, assign34600_e39720_d_n7, assign34600_e39720_d_n8, assign34600_e39720_d_n9, assign34600_e39720_d_n10, assign34600_e39720_d_n11, assign34600_e39720_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign34600_e39718: f64 = (locals.var_qiu / locals.var_t9);
        (assign34600_e39718, (((locals.var_qiu_dn0 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn0)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn2 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn2)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn4 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn4)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn5 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn5)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn6 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn6)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn7 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn7)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn8 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn8)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn9 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn9)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn10 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn10)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn11 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn11)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn14 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn14)) / (locals.var_t9 * locals.var_t9)),)
    } else {
        (locals.var_rns, locals.var_rns_dn0, locals.var_rns_dn2, locals.var_rns_dn4, locals.var_rns_dn5, locals.var_rns_dn6, locals.var_rns_dn7, locals.var_rns_dn8, locals.var_rns_dn9, locals.var_rns_dn10, locals.var_rns_dn11, locals.var_rns_dn14,)
    }
};
        locals.var_rns = assign34600_e39720;
        locals.var_rns_dn0 = assign34600_e39720_d_n0;
        locals.var_rns_dn2 = assign34600_e39720_d_n2;
        locals.var_rns_dn4 = assign34600_e39720_d_n4;
        locals.var_rns_dn5 = assign34600_e39720_d_n5;
        locals.var_rns_dn6 = assign34600_e39720_d_n6;
        locals.var_rns_dn7 = assign34600_e39720_d_n7;
        locals.var_rns_dn8 = assign34600_e39720_d_n8;
        locals.var_rns_dn9 = assign34600_e39720_d_n9;
        locals.var_rns_dn10 = assign34600_e39720_d_n10;
        locals.var_rns_dn11 = assign34600_e39720_d_n11;
        locals.var_rns_dn14 = assign34600_e39720_d_n14;
        locals.var_rns_rv = 0.0;

        let (assign34610_e39744, assign34610_e39744_d_n0, assign34610_e39744_d_n2, assign34610_e39744_d_n4, assign34610_e39744_d_n5, assign34610_e39744_d_n6, assign34610_e39744_d_n7, assign34610_e39744_d_n8, assign34610_e39744_d_n9, assign34610_e39744_d_n10, assign34610_e39744_d_n11, assign34610_e39744_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign34610_e39728: f64 = (locals.var_uc_muecb1 * locals.var_rns);
        let assign34610_e39730: f64 = (assign34610_e39728 / 100000000000.0);
        let assign34610_e39731: f64 = (locals.var_uc_muecb0 + assign34610_e39730);
        let assign34610_e39733: f64 = (assign34610_e39731 + 1e-25);
        let assign34610_e39734: f64 = (1.0 / assign34610_e39733);
        let assign34610_e39737: f64 = (locals.var_mphn0 * locals.var_t8);
        let assign34610_e39738: f64 = (assign34610_e39734 + assign34610_e39737);
        let assign34610_e39741: f64 = (locals.var_t6 / locals.var_uc_muesr1);
        let assign34610_e39742: f64 = (assign34610_e39738 + assign34610_e39741);
        (assign34610_e39742, (((-(((locals.var_uc_muecb1 * locals.var_rns_dn0) / 100000000000.0) / (assign34610_e39733 * assign34610_e39733))) + ((locals.var_mphn0_dn0 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn0))) + (locals.var_t6_dn0 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn2) / 100000000000.0) / (assign34610_e39733 * assign34610_e39733))) + ((locals.var_mphn0_dn2 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn2))) + (locals.var_t6_dn2 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn4) / 100000000000.0) / (assign34610_e39733 * assign34610_e39733))) + ((locals.var_mphn0_dn4 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn4))) + (locals.var_t6_dn4 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn5) / 100000000000.0) / (assign34610_e39733 * assign34610_e39733))) + ((locals.var_mphn0_dn5 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn5))) + (locals.var_t6_dn5 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn6) / 100000000000.0) / (assign34610_e39733 * assign34610_e39733))) + ((locals.var_mphn0_dn6 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn6))) + (locals.var_t6_dn6 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn7) / 100000000000.0) / (assign34610_e39733 * assign34610_e39733))) + ((locals.var_mphn0_dn7 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn7))) + (locals.var_t6_dn7 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn8) / 100000000000.0) / (assign34610_e39733 * assign34610_e39733))) + ((locals.var_mphn0_dn8 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn8))) + (locals.var_t6_dn8 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn9) / 100000000000.0) / (assign34610_e39733 * assign34610_e39733))) + ((locals.var_mphn0_dn9 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn9))) + (locals.var_t6_dn9 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn10) / 100000000000.0) / (assign34610_e39733 * assign34610_e39733))) + ((locals.var_mphn0_dn10 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn10))) + (locals.var_t6_dn10 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn11) / 100000000000.0) / (assign34610_e39733 * assign34610_e39733))) + ((locals.var_mphn0_dn11 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn11))) + (locals.var_t6_dn11 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn14) / 100000000000.0) / (assign34610_e39733 * assign34610_e39733))) + ((locals.var_mphn0_dn14 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn14))) + (locals.var_t6_dn14 / locals.var_uc_muesr1)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign34610_e39744;
        locals.var_t1_dn0 = assign34610_e39744_d_n0;
        locals.var_t1_dn2 = assign34610_e39744_d_n2;
        locals.var_t1_dn4 = assign34610_e39744_d_n4;
        locals.var_t1_dn5 = assign34610_e39744_d_n5;
        locals.var_t1_dn6 = assign34610_e39744_d_n6;
        locals.var_t1_dn7 = assign34610_e39744_d_n7;
        locals.var_t1_dn8 = assign34610_e39744_d_n8;
        locals.var_t1_dn9 = assign34610_e39744_d_n9;
        locals.var_t1_dn10 = assign34610_e39744_d_n10;
        locals.var_t1_dn11 = assign34610_e39744_d_n11;
        locals.var_t1_dn14 = assign34610_e39744_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign34620_e39752, assign34620_e39752_d_n0, assign34620_e39752_d_n2, assign34620_e39752_d_n4, assign34620_e39752_d_n5, assign34620_e39752_d_n6, assign34620_e39752_d_n7, assign34620_e39752_d_n8, assign34620_e39752_d_n9, assign34620_e39752_d_n10, assign34620_e39752_d_n11, assign34620_e39752_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign34620_e39750: f64 = (1.0 / locals.var_t1);
        (assign34620_e39750, (-(locals.var_t1_dn0 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn2 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn11 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn14 / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn14,)
    }
};
        locals.var_muun = assign34620_e39752;
        locals.var_muun_dn0 = assign34620_e39752_d_n0;
        locals.var_muun_dn2 = assign34620_e39752_d_n2;
        locals.var_muun_dn4 = assign34620_e39752_d_n4;
        locals.var_muun_dn5 = assign34620_e39752_d_n5;
        locals.var_muun_dn6 = assign34620_e39752_d_n6;
        locals.var_muun_dn7 = assign34620_e39752_d_n7;
        locals.var_muun_dn8 = assign34620_e39752_d_n8;
        locals.var_muun_dn9 = assign34620_e39752_d_n9;
        locals.var_muun_dn10 = assign34620_e39752_d_n10;
        locals.var_muun_dn11 = assign34620_e39752_d_n11;
        locals.var_muun_dn14 = assign34620_e39752_d_n14;
        locals.var_muun_rv = 0.0;

        let (assign34630_e39760, assign34630_e39760_d_n0, assign34630_e39760_d_n2, assign34630_e39760_d_n4, assign34630_e39760_d_n5, assign34630_e39760_d_n6, assign34630_e39760_d_n7, assign34630_e39760_d_n8, assign34630_e39760_d_n9, assign34630_e39760_d_n10, assign34630_e39760_d_n11, assign34630_e39760_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign34630_e39758: f64 = (locals.var_muun / 10000.0);
        (assign34630_e39758, (locals.var_muun_dn0 / 10000.0), (locals.var_muun_dn2 / 10000.0), (locals.var_muun_dn4 / 10000.0), (locals.var_muun_dn5 / 10000.0), (locals.var_muun_dn6 / 10000.0), (locals.var_muun_dn7 / 10000.0), (locals.var_muun_dn8 / 10000.0), (locals.var_muun_dn9 / 10000.0), (locals.var_muun_dn10 / 10000.0), (locals.var_muun_dn11 / 10000.0), (locals.var_muun_dn14 / 10000.0),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn14,)
    }
};
        locals.var_muun = assign34630_e39760;
        locals.var_muun_dn0 = assign34630_e39760_d_n0;
        locals.var_muun_dn2 = assign34630_e39760_d_n2;
        locals.var_muun_dn4 = assign34630_e39760_d_n4;
        locals.var_muun_dn5 = assign34630_e39760_d_n5;
        locals.var_muun_dn6 = assign34630_e39760_d_n6;
        locals.var_muun_dn7 = assign34630_e39760_d_n7;
        locals.var_muun_dn8 = assign34630_e39760_d_n8;
        locals.var_muun_dn9 = assign34630_e39760_d_n9;
        locals.var_muun_dn10 = assign34630_e39760_d_n10;
        locals.var_muun_dn11 = assign34630_e39760_d_n11;
        locals.var_muun_dn14 = assign34630_e39760_d_n14;
        locals.var_muun_rv = 0.0;

        let (assign34640_e39772, assign34640_e39772_d_n0, assign34640_e39772_d_n2, assign34640_e39772_d_n4, assign34640_e39772_d_n5, assign34640_e39772_d_n6, assign34640_e39772_d_n7, assign34640_e39772_d_n8, assign34640_e39772_d_n9, assign34640_e39772_d_n10, assign34640_e39772_d_n11, assign34640_e39772_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign34640_e39767: f64 = (locals.var_qiu + 1e-25);
        let assign34640_e39768: f64 = (locals.var_beta * assign34640_e39767);
        let assign34640_e39770: f64 = (assign34640_e39768 * locals.var_lch);
        (assign34640_e39770, ((((locals.var_beta_dn0 * assign34640_e39767) + (locals.var_beta * locals.var_qiu_dn0)) * locals.var_lch) + (assign34640_e39768 * locals.var_lch_dn0)), ((((locals.var_beta_dn2 * assign34640_e39767) + (locals.var_beta * locals.var_qiu_dn2)) * locals.var_lch) + (assign34640_e39768 * locals.var_lch_dn2)), ((((locals.var_beta_dn4 * assign34640_e39767) + (locals.var_beta * locals.var_qiu_dn4)) * locals.var_lch) + (assign34640_e39768 * locals.var_lch_dn4)), ((((locals.var_beta_dn5 * assign34640_e39767) + (locals.var_beta * locals.var_qiu_dn5)) * locals.var_lch) + (assign34640_e39768 * locals.var_lch_dn5)), ((((locals.var_beta_dn6 * assign34640_e39767) + (locals.var_beta * locals.var_qiu_dn6)) * locals.var_lch) + (assign34640_e39768 * locals.var_lch_dn6)), ((((locals.var_beta_dn7 * assign34640_e39767) + (locals.var_beta * locals.var_qiu_dn7)) * locals.var_lch) + (assign34640_e39768 * locals.var_lch_dn7)), ((((locals.var_beta_dn8 * assign34640_e39767) + (locals.var_beta * locals.var_qiu_dn8)) * locals.var_lch) + (assign34640_e39768 * locals.var_lch_dn8)), ((((locals.var_beta_dn9 * assign34640_e39767) + (locals.var_beta * locals.var_qiu_dn9)) * locals.var_lch) + (assign34640_e39768 * locals.var_lch_dn9)), ((((locals.var_beta_dn10 * assign34640_e39767) + (locals.var_beta * locals.var_qiu_dn10)) * locals.var_lch) + (assign34640_e39768 * locals.var_lch_dn10)), ((((locals.var_beta_dn11 * assign34640_e39767) + (locals.var_beta * locals.var_qiu_dn11)) * locals.var_lch) + (assign34640_e39768 * locals.var_lch_dn11)), ((((locals.var_beta_dn14 * assign34640_e39767) + (locals.var_beta * locals.var_qiu_dn14)) * locals.var_lch) + (assign34640_e39768 * locals.var_lch_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign34640_e39772;
        locals.var_t2_dn0 = assign34640_e39772_d_n0;
        locals.var_t2_dn2 = assign34640_e39772_d_n2;
        locals.var_t2_dn4 = assign34640_e39772_d_n4;
        locals.var_t2_dn5 = assign34640_e39772_d_n5;
        locals.var_t2_dn6 = assign34640_e39772_d_n6;
        locals.var_t2_dn7 = assign34640_e39772_d_n7;
        locals.var_t2_dn8 = assign34640_e39772_d_n8;
        locals.var_t2_dn9 = assign34640_e39772_d_n9;
        locals.var_t2_dn10 = assign34640_e39772_d_n10;
        locals.var_t2_dn11 = assign34640_e39772_d_n11;
        locals.var_t2_dn14 = assign34640_e39772_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign34650_e39780, assign34650_e39780_d_n0, assign34650_e39780_d_n2, assign34650_e39780_d_n4, assign34650_e39780_d_n5, assign34650_e39780_d_n6, assign34650_e39780_d_n7, assign34650_e39780_d_n8, assign34650_e39780_d_n9, assign34650_e39780_d_n10, assign34650_e39780_d_n11, assign34650_e39780_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign34650_e39778: f64 = (1.0 / locals.var_t2);
        (assign34650_e39778, (-(locals.var_t2_dn0 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn2 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn11 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn14 / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign34650_e39780;
        locals.var_t1_dn0 = assign34650_e39780_d_n0;
        locals.var_t1_dn2 = assign34650_e39780_d_n2;
        locals.var_t1_dn4 = assign34650_e39780_d_n4;
        locals.var_t1_dn5 = assign34650_e39780_d_n5;
        locals.var_t1_dn6 = assign34650_e39780_d_n6;
        locals.var_t1_dn7 = assign34650_e39780_d_n7;
        locals.var_t1_dn8 = assign34650_e39780_d_n8;
        locals.var_t1_dn9 = assign34650_e39780_d_n9;
        locals.var_t1_dn10 = assign34650_e39780_d_n10;
        locals.var_t1_dn11 = assign34650_e39780_d_n11;
        locals.var_t1_dn14 = assign34650_e39780_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign34660_e39788, assign34660_e39788_d_n0, assign34660_e39788_d_n2, assign34660_e39788_d_n4, assign34660_e39788_d_n5, assign34660_e39788_d_n6, assign34660_e39788_d_n7, assign34660_e39788_d_n8, assign34660_e39788_d_n9, assign34660_e39788_d_n10, assign34660_e39788_d_n11, assign34660_e39788_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign34660_e39786: f64 = (locals.var_idd * locals.var_t1);
        (assign34660_e39786, ((locals.var_idd_dn0 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn0)), ((locals.var_idd_dn2 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn2)), ((locals.var_idd_dn4 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn4)), ((locals.var_idd_dn5 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn5)), ((locals.var_idd_dn6 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn6)), ((locals.var_idd_dn7 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn7)), ((locals.var_idd_dn8 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn8)), ((locals.var_idd_dn9 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn9)), ((locals.var_idd_dn10 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn10)), ((locals.var_idd_dn11 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn11)), ((locals.var_idd_dn14 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign34660_e39788;
        locals.var_ty_dn0 = assign34660_e39788_d_n0;
        locals.var_ty_dn2 = assign34660_e39788_d_n2;
        locals.var_ty_dn4 = assign34660_e39788_d_n4;
        locals.var_ty_dn5 = assign34660_e39788_d_n5;
        locals.var_ty_dn6 = assign34660_e39788_d_n6;
        locals.var_ty_dn7 = assign34660_e39788_d_n7;
        locals.var_ty_dn8 = assign34660_e39788_d_n8;
        locals.var_ty_dn9 = assign34660_e39788_d_n9;
        locals.var_ty_dn10 = assign34660_e39788_d_n10;
        locals.var_ty_dn11 = assign34660_e39788_d_n11;
        locals.var_ty_dn14 = assign34660_e39788_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign34670_e39798, assign34670_e39798_d_n0, assign34670_e39798_d_n2, assign34670_e39798_d_n4, assign34670_e39798_d_n5, assign34670_e39798_d_n6, assign34670_e39798_d_n7, assign34670_e39798_d_n8, assign34670_e39798_d_n9, assign34670_e39798_d_n10, assign34670_e39798_d_n11, assign34670_e39798_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign34670_e39794: f64 = (0.2 * locals.var_vmaxe);
        let assign34670_e39796: f64 = (assign34670_e39794 / locals.var_muun);
        (assign34670_e39796, ((((0.2 * locals.var_vmaxe_dn0) * locals.var_muun) - (assign34670_e39794 * locals.var_muun_dn0)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn2) * locals.var_muun) - (assign34670_e39794 * locals.var_muun_dn2)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn4) * locals.var_muun) - (assign34670_e39794 * locals.var_muun_dn4)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn5) * locals.var_muun) - (assign34670_e39794 * locals.var_muun_dn5)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn6) * locals.var_muun) - (assign34670_e39794 * locals.var_muun_dn6)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn7) * locals.var_muun) - (assign34670_e39794 * locals.var_muun_dn7)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn8) * locals.var_muun) - (assign34670_e39794 * locals.var_muun_dn8)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn9) * locals.var_muun) - (assign34670_e39794 * locals.var_muun_dn9)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn10) * locals.var_muun) - (assign34670_e39794 * locals.var_muun_dn10)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn11) * locals.var_muun) - (assign34670_e39794 * locals.var_muun_dn11)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn14) * locals.var_muun) - (assign34670_e39794 * locals.var_muun_dn14)) / (locals.var_muun * locals.var_muun)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign34670_e39798;
        locals.var_t2_dn0 = assign34670_e39798_d_n0;
        locals.var_t2_dn2 = assign34670_e39798_d_n2;
        locals.var_t2_dn4 = assign34670_e39798_d_n4;
        locals.var_t2_dn5 = assign34670_e39798_d_n5;
        locals.var_t2_dn6 = assign34670_e39798_d_n6;
        locals.var_t2_dn7 = assign34670_e39798_d_n7;
        locals.var_t2_dn8 = assign34670_e39798_d_n8;
        locals.var_t2_dn9 = assign34670_e39798_d_n9;
        locals.var_t2_dn10 = assign34670_e39798_d_n10;
        locals.var_t2_dn11 = assign34670_e39798_d_n11;
        locals.var_t2_dn14 = assign34670_e39798_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign34680_e39811, assign34680_e39811_d_n0, assign34680_e39811_d_n2, assign34680_e39811_d_n4, assign34680_e39811_d_n5, assign34680_e39811_d_n6, assign34680_e39811_d_n7, assign34680_e39811_d_n8, assign34680_e39811_d_n9, assign34680_e39811_d_n10, assign34680_e39811_d_n11, assign34680_e39811_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign34680_e39804: f64 = (locals.var_ty * locals.var_ty);
        let assign34680_e39807: f64 = (locals.var_t2 * locals.var_t2);
        let assign34680_e39808: f64 = (assign34680_e39804 + assign34680_e39807);
        let assign34680_e39809: f64 = (assign34680_e39808).sqrt();
        (assign34680_e39809, ((((locals.var_ty_dn0 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn0)) + ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0))) / (2.0 * assign34680_e39809)), ((((locals.var_ty_dn2 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn2)) + ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2))) / (2.0 * assign34680_e39809)), ((((locals.var_ty_dn4 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn4)) + ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4))) / (2.0 * assign34680_e39809)), ((((locals.var_ty_dn5 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn5)) + ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5))) / (2.0 * assign34680_e39809)), ((((locals.var_ty_dn6 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn6)) + ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6))) / (2.0 * assign34680_e39809)), ((((locals.var_ty_dn7 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn7)) + ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7))) / (2.0 * assign34680_e39809)), ((((locals.var_ty_dn8 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn8)) + ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8))) / (2.0 * assign34680_e39809)), ((((locals.var_ty_dn9 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn9)) + ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9))) / (2.0 * assign34680_e39809)), ((((locals.var_ty_dn10 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn10)) + ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10))) / (2.0 * assign34680_e39809)), ((((locals.var_ty_dn11 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn11)) + ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11))) / (2.0 * assign34680_e39809)), ((((locals.var_ty_dn14 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn14)) + ((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14))) / (2.0 * assign34680_e39809)),)
    } else {
        (locals.var_ey, locals.var_ey_dn0, locals.var_ey_dn2, locals.var_ey_dn4, locals.var_ey_dn5, locals.var_ey_dn6, locals.var_ey_dn7, locals.var_ey_dn8, locals.var_ey_dn9, locals.var_ey_dn10, locals.var_ey_dn11, locals.var_ey_dn14,)
    }
};
        locals.var_ey = assign34680_e39811;
        locals.var_ey_dn0 = assign34680_e39811_d_n0;
        locals.var_ey_dn2 = assign34680_e39811_d_n2;
        locals.var_ey_dn4 = assign34680_e39811_d_n4;
        locals.var_ey_dn5 = assign34680_e39811_d_n5;
        locals.var_ey_dn6 = assign34680_e39811_d_n6;
        locals.var_ey_dn7 = assign34680_e39811_d_n7;
        locals.var_ey_dn8 = assign34680_e39811_d_n8;
        locals.var_ey_dn9 = assign34680_e39811_d_n9;
        locals.var_ey_dn10 = assign34680_e39811_d_n10;
        locals.var_ey_dn11 = assign34680_e39811_d_n11;
        locals.var_ey_dn14 = assign34680_e39811_d_n14;
        locals.var_ey_rv = 0.0;

        let (assign34690_e39819, assign34690_e39819_d_n0, assign34690_e39819_d_n2, assign34690_e39819_d_n4, assign34690_e39819_d_n5, assign34690_e39819_d_n6, assign34690_e39819_d_n7, assign34690_e39819_d_n8, assign34690_e39819_d_n9, assign34690_e39819_d_n10, assign34690_e39819_d_n11, assign34690_e39819_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign34690_e39817: f64 = (1.0 / locals.var_ey);
        (assign34690_e39817, (-(locals.var_ey_dn0 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn2 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn4 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn5 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn6 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn7 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn8 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn9 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn10 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn11 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn14 / (locals.var_ey * locals.var_ey))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign34690_e39819;
        locals.var_t4_dn0 = assign34690_e39819_d_n0;
        locals.var_t4_dn2 = assign34690_e39819_d_n2;
        locals.var_t4_dn4 = assign34690_e39819_d_n4;
        locals.var_t4_dn5 = assign34690_e39819_d_n5;
        locals.var_t4_dn6 = assign34690_e39819_d_n6;
        locals.var_t4_dn7 = assign34690_e39819_d_n7;
        locals.var_t4_dn8 = assign34690_e39819_d_n8;
        locals.var_t4_dn9 = assign34690_e39819_d_n9;
        locals.var_t4_dn10 = assign34690_e39819_d_n10;
        locals.var_t4_dn11 = assign34690_e39819_d_n11;
        locals.var_t4_dn14 = assign34690_e39819_d_n14;
        locals.var_t4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_113(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign34700_e39827, assign34700_e39827_d_n0, assign34700_e39827_d_n2, assign34700_e39827_d_n4, assign34700_e39827_d_n5, assign34700_e39827_d_n6, assign34700_e39827_d_n7, assign34700_e39827_d_n8, assign34700_e39827_d_n9, assign34700_e39827_d_n10, assign34700_e39827_d_n11, assign34700_e39827_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign34700_e39825: f64 = (locals.var_muun * locals.var_ey);
        (assign34700_e39825, ((locals.var_muun_dn0 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn0)), ((locals.var_muun_dn2 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn2)), ((locals.var_muun_dn4 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn4)), ((locals.var_muun_dn5 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn5)), ((locals.var_muun_dn6 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn6)), ((locals.var_muun_dn7 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn7)), ((locals.var_muun_dn8 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn8)), ((locals.var_muun_dn9 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn9)), ((locals.var_muun_dn10 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn10)), ((locals.var_muun_dn11 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn11)), ((locals.var_muun_dn14 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn14)),)
    } else {
        (locals.var_em, locals.var_em_dn0, locals.var_em_dn2, locals.var_em_dn4, locals.var_em_dn5, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, locals.var_em_dn9, locals.var_em_dn10, locals.var_em_dn11, locals.var_em_dn14,)
    }
};
        locals.var_em = assign34700_e39827;
        locals.var_em_dn0 = assign34700_e39827_d_n0;
        locals.var_em_dn2 = assign34700_e39827_d_n2;
        locals.var_em_dn4 = assign34700_e39827_d_n4;
        locals.var_em_dn5 = assign34700_e39827_d_n5;
        locals.var_em_dn6 = assign34700_e39827_d_n6;
        locals.var_em_dn7 = assign34700_e39827_d_n7;
        locals.var_em_dn8 = assign34700_e39827_d_n8;
        locals.var_em_dn9 = assign34700_e39827_d_n9;
        locals.var_em_dn10 = assign34700_e39827_d_n10;
        locals.var_em_dn11 = assign34700_e39827_d_n11;
        locals.var_em_dn14 = assign34700_e39827_d_n14;
        locals.var_em_rv = 0.0;

        let (assign34710_e39835, assign34710_e39835_d_n0, assign34710_e39835_d_n2, assign34710_e39835_d_n4, assign34710_e39835_d_n5, assign34710_e39835_d_n6, assign34710_e39835_d_n7, assign34710_e39835_d_n8, assign34710_e39835_d_n9, assign34710_e39835_d_n10, assign34710_e39835_d_n11, assign34710_e39835_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign34710_e39833: f64 = (locals.var_em / locals.var_vmaxe);
        (assign34710_e39833, (((locals.var_em_dn0 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn0)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn2 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn2)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn4 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn4)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn5 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn5)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn6 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn6)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn7 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn7)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn8 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn8)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn9 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn9)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn10 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn10)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn11 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn11)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn14 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn14)) / (locals.var_vmaxe * locals.var_vmaxe)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign34710_e39835;
        locals.var_t1_dn0 = assign34710_e39835_d_n0;
        locals.var_t1_dn2 = assign34710_e39835_d_n2;
        locals.var_t1_dn4 = assign34710_e39835_d_n4;
        locals.var_t1_dn5 = assign34710_e39835_d_n5;
        locals.var_t1_dn6 = assign34710_e39835_d_n6;
        locals.var_t1_dn7 = assign34710_e39835_d_n7;
        locals.var_t1_dn8 = assign34710_e39835_d_n8;
        locals.var_t1_dn9 = assign34710_e39835_d_n9;
        locals.var_t1_dn10 = assign34710_e39835_d_n10;
        locals.var_t1_dn11 = assign34710_e39835_d_n11;
        locals.var_t1_dn14 = assign34710_e39835_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign34720_e39841, assign34720_e39841_d_n0, assign34720_e39841_d_n2, assign34720_e39841_d_n4, assign34720_e39841_d_n5, assign34720_e39841_d_n6, assign34720_e39841_d_n7, assign34720_e39841_d_n8, assign34720_e39841_d_n9, assign34720_e39841_d_n10, assign34720_e39841_d_n11, assign34720_e39841_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        (locals.var_ey, locals.var_ey_dn0, locals.var_ey_dn2, locals.var_ey_dn4, locals.var_ey_dn5, locals.var_ey_dn6, locals.var_ey_dn7, locals.var_ey_dn8, locals.var_ey_dn9, locals.var_ey_dn10, locals.var_ey_dn11, locals.var_ey_dn14,)
    } else {
        (locals.var_ey_suf, locals.var_ey_suf_dn0, locals.var_ey_suf_dn2, locals.var_ey_suf_dn4, locals.var_ey_suf_dn5, locals.var_ey_suf_dn6, locals.var_ey_suf_dn7, locals.var_ey_suf_dn8, locals.var_ey_suf_dn9, locals.var_ey_suf_dn10, locals.var_ey_suf_dn11, locals.var_ey_suf_dn14,)
    }
};
        locals.var_ey_suf = assign34720_e39841;
        locals.var_ey_suf_dn0 = assign34720_e39841_d_n0;
        locals.var_ey_suf_dn2 = assign34720_e39841_d_n2;
        locals.var_ey_suf_dn4 = assign34720_e39841_d_n4;
        locals.var_ey_suf_dn5 = assign34720_e39841_d_n5;
        locals.var_ey_suf_dn6 = assign34720_e39841_d_n6;
        locals.var_ey_suf_dn7 = assign34720_e39841_d_n7;
        locals.var_ey_suf_dn8 = assign34720_e39841_d_n8;
        locals.var_ey_suf_dn9 = assign34720_e39841_d_n9;
        locals.var_ey_suf_dn10 = assign34720_e39841_d_n10;
        locals.var_ey_suf_dn11 = assign34720_e39841_d_n11;
        locals.var_ey_suf_dn14 = assign34720_e39841_d_n14;
        locals.var_ey_suf_rv = 0.0;

        let assign34730_e39845: f64 = (10.0 * 2.220446049250313e-16);
        let assign34730_e39846: f64 = (1.0 - assign34730_e39845);
        let assign34730_e39853: f64 = (10.0 * 2.220446049250313e-16);
        let assign34730_e39854: f64 = (1.0 + assign34730_e39853);
        let assign34730_e39856: f64 = if ((assign34730_e39846 <= p.p178) && (p.p178 <= assign34730_e39854)) { 1.0 } else { 0.0 };
        locals.var_guard808 = assign34730_e39856;
        locals.var_guard808_rv = 0.0;

        let (assign34740_e39864, assign34740_e39864_d_n0, assign34740_e39864_d_n2, assign34740_e39864_d_n4, assign34740_e39864_d_n5, assign34740_e39864_d_n6, assign34740_e39864_d_n7, assign34740_e39864_d_n8, assign34740_e39864_d_n9, assign34740_e39864_d_n10, assign34740_e39864_d_n11, assign34740_e39864_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard808 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign34740_e39864;
        locals.var_t3_dn0 = assign34740_e39864_d_n0;
        locals.var_t3_dn2 = assign34740_e39864_d_n2;
        locals.var_t3_dn4 = assign34740_e39864_d_n4;
        locals.var_t3_dn5 = assign34740_e39864_d_n5;
        locals.var_t3_dn6 = assign34740_e39864_d_n6;
        locals.var_t3_dn7 = assign34740_e39864_d_n7;
        locals.var_t3_dn8 = assign34740_e39864_d_n8;
        locals.var_t3_dn9 = assign34740_e39864_d_n9;
        locals.var_t3_dn10 = assign34740_e39864_d_n10;
        locals.var_t3_dn11 = assign34740_e39864_d_n11;
        locals.var_t3_dn14 = assign34740_e39864_d_n14;
        locals.var_t3_rv = 0.0;

        let assign34750_e39868: f64 = (10.0 * 2.220446049250313e-16);
        let assign34750_e39869: f64 = (2.0 - assign34750_e39868);
        let assign34750_e39876: f64 = (10.0 * 2.220446049250313e-16);
        let assign34750_e39877: f64 = (2.0 + assign34750_e39876);
        let assign34750_e39879: f64 = if ((assign34750_e39869 <= p.p178) && (p.p178 <= assign34750_e39877)) { 1.0 } else { 0.0 };
        locals.var_guard809 = assign34750_e39879;
        locals.var_guard809_rv = 0.0;

        let (assign34760_e39890, assign34760_e39890_d_n0, assign34760_e39890_d_n2, assign34760_e39890_d_n4, assign34760_e39890_d_n5, assign34760_e39890_d_n6, assign34760_e39890_d_n7, assign34760_e39890_d_n8, assign34760_e39890_d_n9, assign34760_e39890_d_n10, assign34760_e39890_d_n11, assign34760_e39890_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard808 == 0.0)) && (locals.var_guard809 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign34760_e39890;
        locals.var_t3_dn0 = assign34760_e39890_d_n0;
        locals.var_t3_dn2 = assign34760_e39890_d_n2;
        locals.var_t3_dn4 = assign34760_e39890_d_n4;
        locals.var_t3_dn5 = assign34760_e39890_d_n5;
        locals.var_t3_dn6 = assign34760_e39890_d_n6;
        locals.var_t3_dn7 = assign34760_e39890_d_n7;
        locals.var_t3_dn8 = assign34760_e39890_d_n8;
        locals.var_t3_dn9 = assign34760_e39890_d_n9;
        locals.var_t3_dn10 = assign34760_e39890_d_n10;
        locals.var_t3_dn11 = assign34760_e39890_d_n11;
        locals.var_t3_dn14 = assign34760_e39890_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign34770_e39911, assign34770_e39911_d_n0, assign34770_e39911_d_n2, assign34770_e39911_d_n4, assign34770_e39911_d_n5, assign34770_e39911_d_n6, assign34770_e39911_d_n7, assign34770_e39911_d_n8, assign34770_e39911_d_n9, assign34770_e39911_d_n10, assign34770_e39911_d_n11, assign34770_e39911_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard808 == 0.0)) && (locals.var_guard809 == 0.0)) {
        let (assign34770_e39909, assign34770_e39909_d_n0, assign34770_e39909_d_n2, assign34770_e39909_d_n4, assign34770_e39909_d_n5, assign34770_e39909_d_n6, assign34770_e39909_d_n7, assign34770_e39909_d_n8, assign34770_e39909_d_n9, assign34770_e39909_d_n10, assign34770_e39909_d_n11, assign34770_e39909_d_n14,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign34770_e39907: f64 = (p.p178 - 1.0);
                let assign34770_e39908: f64 = (locals.var_t1).powf(assign34770_e39907);
                (assign34770_e39908, if 0.0 == 0.0 && ((assign34770_e39907) as f64).is_finite() && ((assign34770_e39907) as f64).fract() == 0.0 { if assign34770_e39907 == 0.0 { 0.0 } else { (assign34770_e39907 * ((locals.var_t1).powf(assign34770_e39907 - 1.0) * locals.var_t1_dn0)) } } else { (assign34770_e39908 * (assign34770_e39907 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign34770_e39907) as f64).is_finite() && ((assign34770_e39907) as f64).fract() == 0.0 { if assign34770_e39907 == 0.0 { 0.0 } else { (assign34770_e39907 * ((locals.var_t1).powf(assign34770_e39907 - 1.0) * locals.var_t1_dn2)) } } else { (assign34770_e39908 * (assign34770_e39907 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign34770_e39907) as f64).is_finite() && ((assign34770_e39907) as f64).fract() == 0.0 { if assign34770_e39907 == 0.0 { 0.0 } else { (assign34770_e39907 * ((locals.var_t1).powf(assign34770_e39907 - 1.0) * locals.var_t1_dn4)) } } else { (assign34770_e39908 * (assign34770_e39907 * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign34770_e39907) as f64).is_finite() && ((assign34770_e39907) as f64).fract() == 0.0 { if assign34770_e39907 == 0.0 { 0.0 } else { (assign34770_e39907 * ((locals.var_t1).powf(assign34770_e39907 - 1.0) * locals.var_t1_dn5)) } } else { (assign34770_e39908 * (assign34770_e39907 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign34770_e39907) as f64).is_finite() && ((assign34770_e39907) as f64).fract() == 0.0 { if assign34770_e39907 == 0.0 { 0.0 } else { (assign34770_e39907 * ((locals.var_t1).powf(assign34770_e39907 - 1.0) * locals.var_t1_dn6)) } } else { (assign34770_e39908 * (assign34770_e39907 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign34770_e39907) as f64).is_finite() && ((assign34770_e39907) as f64).fract() == 0.0 { if assign34770_e39907 == 0.0 { 0.0 } else { (assign34770_e39907 * ((locals.var_t1).powf(assign34770_e39907 - 1.0) * locals.var_t1_dn7)) } } else { (assign34770_e39908 * (assign34770_e39907 * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign34770_e39907) as f64).is_finite() && ((assign34770_e39907) as f64).fract() == 0.0 { if assign34770_e39907 == 0.0 { 0.0 } else { (assign34770_e39907 * ((locals.var_t1).powf(assign34770_e39907 - 1.0) * locals.var_t1_dn8)) } } else { (assign34770_e39908 * (assign34770_e39907 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign34770_e39907) as f64).is_finite() && ((assign34770_e39907) as f64).fract() == 0.0 { if assign34770_e39907 == 0.0 { 0.0 } else { (assign34770_e39907 * ((locals.var_t1).powf(assign34770_e39907 - 1.0) * locals.var_t1_dn9)) } } else { (assign34770_e39908 * (assign34770_e39907 * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign34770_e39907) as f64).is_finite() && ((assign34770_e39907) as f64).fract() == 0.0 { if assign34770_e39907 == 0.0 { 0.0 } else { (assign34770_e39907 * ((locals.var_t1).powf(assign34770_e39907 - 1.0) * locals.var_t1_dn10)) } } else { (assign34770_e39908 * (assign34770_e39907 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign34770_e39907) as f64).is_finite() && ((assign34770_e39907) as f64).fract() == 0.0 { if assign34770_e39907 == 0.0 { 0.0 } else { (assign34770_e39907 * ((locals.var_t1).powf(assign34770_e39907 - 1.0) * locals.var_t1_dn11)) } } else { (assign34770_e39908 * (assign34770_e39907 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign34770_e39907) as f64).is_finite() && ((assign34770_e39907) as f64).fract() == 0.0 { if assign34770_e39907 == 0.0 { 0.0 } else { (assign34770_e39907 * ((locals.var_t1).powf(assign34770_e39907 - 1.0) * locals.var_t1_dn14)) } } else { (assign34770_e39908 * (assign34770_e39907 * (locals.var_t1_dn14 / locals.var_t1))) },)
            }
        };
        (assign34770_e39909, assign34770_e39909_d_n0, assign34770_e39909_d_n2, assign34770_e39909_d_n4, assign34770_e39909_d_n5, assign34770_e39909_d_n6, assign34770_e39909_d_n7, assign34770_e39909_d_n8, assign34770_e39909_d_n9, assign34770_e39909_d_n10, assign34770_e39909_d_n11, assign34770_e39909_d_n14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign34770_e39911;
        locals.var_t3_dn0 = assign34770_e39911_d_n0;
        locals.var_t3_dn2 = assign34770_e39911_d_n2;
        locals.var_t3_dn4 = assign34770_e39911_d_n4;
        locals.var_t3_dn5 = assign34770_e39911_d_n5;
        locals.var_t3_dn6 = assign34770_e39911_d_n6;
        locals.var_t3_dn7 = assign34770_e39911_d_n7;
        locals.var_t3_dn8 = assign34770_e39911_d_n8;
        locals.var_t3_dn9 = assign34770_e39911_d_n9;
        locals.var_t3_dn10 = assign34770_e39911_d_n10;
        locals.var_t3_dn11 = assign34770_e39911_d_n11;
        locals.var_t3_dn14 = assign34770_e39911_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign34780_e39919, assign34780_e39919_d_n0, assign34780_e39919_d_n2, assign34780_e39919_d_n4, assign34780_e39919_d_n5, assign34780_e39919_d_n6, assign34780_e39919_d_n7, assign34780_e39919_d_n8, assign34780_e39919_d_n9, assign34780_e39919_d_n10, assign34780_e39919_d_n11, assign34780_e39919_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign34780_e39917: f64 = (locals.var_t1 * locals.var_t3);
        (assign34780_e39917, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn14 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign34780_e39919;
        locals.var_t2_dn0 = assign34780_e39919_d_n0;
        locals.var_t2_dn2 = assign34780_e39919_d_n2;
        locals.var_t2_dn4 = assign34780_e39919_d_n4;
        locals.var_t2_dn5 = assign34780_e39919_d_n5;
        locals.var_t2_dn6 = assign34780_e39919_d_n6;
        locals.var_t2_dn7 = assign34780_e39919_d_n7;
        locals.var_t2_dn8 = assign34780_e39919_d_n8;
        locals.var_t2_dn9 = assign34780_e39919_d_n9;
        locals.var_t2_dn10 = assign34780_e39919_d_n10;
        locals.var_t2_dn11 = assign34780_e39919_d_n11;
        locals.var_t2_dn14 = assign34780_e39919_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign34790_e39927, assign34790_e39927_d_n0, assign34790_e39927_d_n2, assign34790_e39927_d_n4, assign34790_e39927_d_n5, assign34790_e39927_d_n6, assign34790_e39927_d_n7, assign34790_e39927_d_n8, assign34790_e39927_d_n9, assign34790_e39927_d_n10, assign34790_e39927_d_n11, assign34790_e39927_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign34790_e39925: f64 = (1.0 + locals.var_t2);
        (assign34790_e39925, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign34790_e39927;
        locals.var_t4_dn0 = assign34790_e39927_d_n0;
        locals.var_t4_dn2 = assign34790_e39927_d_n2;
        locals.var_t4_dn4 = assign34790_e39927_d_n4;
        locals.var_t4_dn5 = assign34790_e39927_d_n5;
        locals.var_t4_dn6 = assign34790_e39927_d_n6;
        locals.var_t4_dn7 = assign34790_e39927_d_n7;
        locals.var_t4_dn8 = assign34790_e39927_d_n8;
        locals.var_t4_dn9 = assign34790_e39927_d_n9;
        locals.var_t4_dn10 = assign34790_e39927_d_n10;
        locals.var_t4_dn11 = assign34790_e39927_d_n11;
        locals.var_t4_dn14 = assign34790_e39927_d_n14;
        locals.var_t4_rv = 0.0;

        let assign34800_e39931: f64 = (10.0 * 2.220446049250313e-16);
        let assign34800_e39932: f64 = (1.0 - assign34800_e39931);
        let assign34800_e39939: f64 = (10.0 * 2.220446049250313e-16);
        let assign34800_e39940: f64 = (1.0 + assign34800_e39939);
        let assign34800_e39942: f64 = if ((assign34800_e39932 <= p.p178) && (p.p178 <= assign34800_e39940)) { 1.0 } else { 0.0 };
        locals.var_guard810 = assign34800_e39942;
        locals.var_guard810_rv = 0.0;

        let (assign34810_e39952, assign34810_e39952_d_n0, assign34810_e39952_d_n2, assign34810_e39952_d_n4, assign34810_e39952_d_n5, assign34810_e39952_d_n6, assign34810_e39952_d_n7, assign34810_e39952_d_n8, assign34810_e39952_d_n9, assign34810_e39952_d_n10, assign34810_e39952_d_n11, assign34810_e39952_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard810 != 0.0)) {
        let assign34810_e39950: f64 = (1.0 / locals.var_t4);
        (assign34810_e39950, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign34810_e39952;
        locals.var_t5_dn0 = assign34810_e39952_d_n0;
        locals.var_t5_dn2 = assign34810_e39952_d_n2;
        locals.var_t5_dn4 = assign34810_e39952_d_n4;
        locals.var_t5_dn5 = assign34810_e39952_d_n5;
        locals.var_t5_dn6 = assign34810_e39952_d_n6;
        locals.var_t5_dn7 = assign34810_e39952_d_n7;
        locals.var_t5_dn8 = assign34810_e39952_d_n8;
        locals.var_t5_dn9 = assign34810_e39952_d_n9;
        locals.var_t5_dn10 = assign34810_e39952_d_n10;
        locals.var_t5_dn11 = assign34810_e39952_d_n11;
        locals.var_t5_dn14 = assign34810_e39952_d_n14;
        locals.var_t5_rv = 0.0;

        let assign34820_e39956: f64 = (10.0 * 2.220446049250313e-16);
        let assign34820_e39957: f64 = (2.0 - assign34820_e39956);
        let assign34820_e39964: f64 = (10.0 * 2.220446049250313e-16);
        let assign34820_e39965: f64 = (2.0 + assign34820_e39964);
        let assign34820_e39967: f64 = if ((assign34820_e39957 <= p.p178) && (p.p178 <= assign34820_e39965)) { 1.0 } else { 0.0 };
        locals.var_guard811 = assign34820_e39967;
        locals.var_guard811_rv = 0.0;

        let (assign34830_e39981, assign34830_e39981_d_n0, assign34830_e39981_d_n2, assign34830_e39981_d_n4, assign34830_e39981_d_n5, assign34830_e39981_d_n6, assign34830_e39981_d_n7, assign34830_e39981_d_n8, assign34830_e39981_d_n9, assign34830_e39981_d_n10, assign34830_e39981_d_n11, assign34830_e39981_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard810 == 0.0)) && (locals.var_guard811 != 0.0)) {
        let assign34830_e39978: f64 = (locals.var_t4).sqrt();
        let assign34830_e39979: f64 = (1.0 / assign34830_e39978);
        (assign34830_e39979, (-((locals.var_t4_dn0 / (2.0 * assign34830_e39978)) / (assign34830_e39978 * assign34830_e39978))), (-((locals.var_t4_dn2 / (2.0 * assign34830_e39978)) / (assign34830_e39978 * assign34830_e39978))), (-((locals.var_t4_dn4 / (2.0 * assign34830_e39978)) / (assign34830_e39978 * assign34830_e39978))), (-((locals.var_t4_dn5 / (2.0 * assign34830_e39978)) / (assign34830_e39978 * assign34830_e39978))), (-((locals.var_t4_dn6 / (2.0 * assign34830_e39978)) / (assign34830_e39978 * assign34830_e39978))), (-((locals.var_t4_dn7 / (2.0 * assign34830_e39978)) / (assign34830_e39978 * assign34830_e39978))), (-((locals.var_t4_dn8 / (2.0 * assign34830_e39978)) / (assign34830_e39978 * assign34830_e39978))), (-((locals.var_t4_dn9 / (2.0 * assign34830_e39978)) / (assign34830_e39978 * assign34830_e39978))), (-((locals.var_t4_dn10 / (2.0 * assign34830_e39978)) / (assign34830_e39978 * assign34830_e39978))), (-((locals.var_t4_dn11 / (2.0 * assign34830_e39978)) / (assign34830_e39978 * assign34830_e39978))), (-((locals.var_t4_dn14 / (2.0 * assign34830_e39978)) / (assign34830_e39978 * assign34830_e39978))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign34830_e39981;
        locals.var_t5_dn0 = assign34830_e39981_d_n0;
        locals.var_t5_dn2 = assign34830_e39981_d_n2;
        locals.var_t5_dn4 = assign34830_e39981_d_n4;
        locals.var_t5_dn5 = assign34830_e39981_d_n5;
        locals.var_t5_dn6 = assign34830_e39981_d_n6;
        locals.var_t5_dn7 = assign34830_e39981_d_n7;
        locals.var_t5_dn8 = assign34830_e39981_d_n8;
        locals.var_t5_dn9 = assign34830_e39981_d_n9;
        locals.var_t5_dn10 = assign34830_e39981_d_n10;
        locals.var_t5_dn11 = assign34830_e39981_d_n11;
        locals.var_t5_dn14 = assign34830_e39981_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign34840_e40005, assign34840_e40005_d_n0, assign34840_e40005_d_n2, assign34840_e40005_d_n4, assign34840_e40005_d_n5, assign34840_e40005_d_n6, assign34840_e40005_d_n7, assign34840_e40005_d_n8, assign34840_e40005_d_n9, assign34840_e40005_d_n10, assign34840_e40005_d_n11, assign34840_e40005_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard810 == 0.0)) && (locals.var_guard811 == 0.0)) {
        let (assign34840_e40003, assign34840_e40003_d_n0, assign34840_e40003_d_n2, assign34840_e40003_d_n4, assign34840_e40003_d_n5, assign34840_e40003_d_n6, assign34840_e40003_d_n7, assign34840_e40003_d_n8, assign34840_e40003_d_n9, assign34840_e40003_d_n10, assign34840_e40003_d_n11, assign34840_e40003_d_n14,) = {
            if (locals.var_t4 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign34840_e39997: f64 = (-1.0);
                let assign34840_e39999: f64 = (assign34840_e39997 / p.p178);
                let assign34840_e40001: f64 = (assign34840_e39999 - 1.0);
                let assign34840_e40002: f64 = (locals.var_t4).powf(assign34840_e40001);
                (assign34840_e40002, if 0.0 == 0.0 && ((assign34840_e40001) as f64).is_finite() && ((assign34840_e40001) as f64).fract() == 0.0 { if assign34840_e40001 == 0.0 { 0.0 } else { (assign34840_e40001 * ((locals.var_t4).powf(assign34840_e40001 - 1.0) * locals.var_t4_dn0)) } } else { (assign34840_e40002 * (assign34840_e40001 * (locals.var_t4_dn0 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign34840_e40001) as f64).is_finite() && ((assign34840_e40001) as f64).fract() == 0.0 { if assign34840_e40001 == 0.0 { 0.0 } else { (assign34840_e40001 * ((locals.var_t4).powf(assign34840_e40001 - 1.0) * locals.var_t4_dn2)) } } else { (assign34840_e40002 * (assign34840_e40001 * (locals.var_t4_dn2 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign34840_e40001) as f64).is_finite() && ((assign34840_e40001) as f64).fract() == 0.0 { if assign34840_e40001 == 0.0 { 0.0 } else { (assign34840_e40001 * ((locals.var_t4).powf(assign34840_e40001 - 1.0) * locals.var_t4_dn4)) } } else { (assign34840_e40002 * (assign34840_e40001 * (locals.var_t4_dn4 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign34840_e40001) as f64).is_finite() && ((assign34840_e40001) as f64).fract() == 0.0 { if assign34840_e40001 == 0.0 { 0.0 } else { (assign34840_e40001 * ((locals.var_t4).powf(assign34840_e40001 - 1.0) * locals.var_t4_dn5)) } } else { (assign34840_e40002 * (assign34840_e40001 * (locals.var_t4_dn5 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign34840_e40001) as f64).is_finite() && ((assign34840_e40001) as f64).fract() == 0.0 { if assign34840_e40001 == 0.0 { 0.0 } else { (assign34840_e40001 * ((locals.var_t4).powf(assign34840_e40001 - 1.0) * locals.var_t4_dn6)) } } else { (assign34840_e40002 * (assign34840_e40001 * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign34840_e40001) as f64).is_finite() && ((assign34840_e40001) as f64).fract() == 0.0 { if assign34840_e40001 == 0.0 { 0.0 } else { (assign34840_e40001 * ((locals.var_t4).powf(assign34840_e40001 - 1.0) * locals.var_t4_dn7)) } } else { (assign34840_e40002 * (assign34840_e40001 * (locals.var_t4_dn7 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign34840_e40001) as f64).is_finite() && ((assign34840_e40001) as f64).fract() == 0.0 { if assign34840_e40001 == 0.0 { 0.0 } else { (assign34840_e40001 * ((locals.var_t4).powf(assign34840_e40001 - 1.0) * locals.var_t4_dn8)) } } else { (assign34840_e40002 * (assign34840_e40001 * (locals.var_t4_dn8 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign34840_e40001) as f64).is_finite() && ((assign34840_e40001) as f64).fract() == 0.0 { if assign34840_e40001 == 0.0 { 0.0 } else { (assign34840_e40001 * ((locals.var_t4).powf(assign34840_e40001 - 1.0) * locals.var_t4_dn9)) } } else { (assign34840_e40002 * (assign34840_e40001 * (locals.var_t4_dn9 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign34840_e40001) as f64).is_finite() && ((assign34840_e40001) as f64).fract() == 0.0 { if assign34840_e40001 == 0.0 { 0.0 } else { (assign34840_e40001 * ((locals.var_t4).powf(assign34840_e40001 - 1.0) * locals.var_t4_dn10)) } } else { (assign34840_e40002 * (assign34840_e40001 * (locals.var_t4_dn10 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign34840_e40001) as f64).is_finite() && ((assign34840_e40001) as f64).fract() == 0.0 { if assign34840_e40001 == 0.0 { 0.0 } else { (assign34840_e40001 * ((locals.var_t4).powf(assign34840_e40001 - 1.0) * locals.var_t4_dn11)) } } else { (assign34840_e40002 * (assign34840_e40001 * (locals.var_t4_dn11 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign34840_e40001) as f64).is_finite() && ((assign34840_e40001) as f64).fract() == 0.0 { if assign34840_e40001 == 0.0 { 0.0 } else { (assign34840_e40001 * ((locals.var_t4).powf(assign34840_e40001 - 1.0) * locals.var_t4_dn14)) } } else { (assign34840_e40002 * (assign34840_e40001 * (locals.var_t4_dn14 / locals.var_t4))) },)
            }
        };
        (assign34840_e40003, assign34840_e40003_d_n0, assign34840_e40003_d_n2, assign34840_e40003_d_n4, assign34840_e40003_d_n5, assign34840_e40003_d_n6, assign34840_e40003_d_n7, assign34840_e40003_d_n8, assign34840_e40003_d_n9, assign34840_e40003_d_n10, assign34840_e40003_d_n11, assign34840_e40003_d_n14,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign34840_e40005;
        locals.var_t6_dn0 = assign34840_e40005_d_n0;
        locals.var_t6_dn2 = assign34840_e40005_d_n2;
        locals.var_t6_dn4 = assign34840_e40005_d_n4;
        locals.var_t6_dn5 = assign34840_e40005_d_n5;
        locals.var_t6_dn6 = assign34840_e40005_d_n6;
        locals.var_t6_dn7 = assign34840_e40005_d_n7;
        locals.var_t6_dn8 = assign34840_e40005_d_n8;
        locals.var_t6_dn9 = assign34840_e40005_d_n9;
        locals.var_t6_dn10 = assign34840_e40005_d_n10;
        locals.var_t6_dn11 = assign34840_e40005_d_n11;
        locals.var_t6_dn14 = assign34840_e40005_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign34850_e40019, assign34850_e40019_d_n0, assign34850_e40019_d_n2, assign34850_e40019_d_n4, assign34850_e40019_d_n5, assign34850_e40019_d_n6, assign34850_e40019_d_n7, assign34850_e40019_d_n8, assign34850_e40019_d_n9, assign34850_e40019_d_n10, assign34850_e40019_d_n11, assign34850_e40019_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard810 == 0.0)) && (locals.var_guard811 == 0.0)) {
        let assign34850_e40017: f64 = (locals.var_t4 * locals.var_t6);
        (assign34850_e40017, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn4 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn4)), ((locals.var_t4_dn5 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn5)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn7 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn7)), ((locals.var_t4_dn8 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn8)), ((locals.var_t4_dn9 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn9)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn11 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn11)), ((locals.var_t4_dn14 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign34850_e40019;
        locals.var_t5_dn0 = assign34850_e40019_d_n0;
        locals.var_t5_dn2 = assign34850_e40019_d_n2;
        locals.var_t5_dn4 = assign34850_e40019_d_n4;
        locals.var_t5_dn5 = assign34850_e40019_d_n5;
        locals.var_t5_dn6 = assign34850_e40019_d_n6;
        locals.var_t5_dn7 = assign34850_e40019_d_n7;
        locals.var_t5_dn8 = assign34850_e40019_d_n8;
        locals.var_t5_dn9 = assign34850_e40019_d_n9;
        locals.var_t5_dn10 = assign34850_e40019_d_n10;
        locals.var_t5_dn11 = assign34850_e40019_d_n11;
        locals.var_t5_dn14 = assign34850_e40019_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign34860_e40027, assign34860_e40027_d_n0, assign34860_e40027_d_n2, assign34860_e40027_d_n4, assign34860_e40027_d_n5, assign34860_e40027_d_n6, assign34860_e40027_d_n7, assign34860_e40027_d_n8, assign34860_e40027_d_n9, assign34860_e40027_d_n10, assign34860_e40027_d_n11, assign34860_e40027_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign34860_e40025: f64 = (locals.var_muun * locals.var_t5);
        (assign34860_e40025, ((locals.var_muun_dn0 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn0)), ((locals.var_muun_dn2 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn2)), ((locals.var_muun_dn4 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn4)), ((locals.var_muun_dn5 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn5)), ((locals.var_muun_dn6 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn6)), ((locals.var_muun_dn7 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn7)), ((locals.var_muun_dn8 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn8)), ((locals.var_muun_dn9 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn9)), ((locals.var_muun_dn10 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn10)), ((locals.var_muun_dn11 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn11)), ((locals.var_muun_dn14 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn14)),)
    } else {
        (locals.var_mu, locals.var_mu_dn0, locals.var_mu_dn2, locals.var_mu_dn4, locals.var_mu_dn5, locals.var_mu_dn6, locals.var_mu_dn7, locals.var_mu_dn8, locals.var_mu_dn9, locals.var_mu_dn10, locals.var_mu_dn11, locals.var_mu_dn14,)
    }
};
        locals.var_mu = assign34860_e40027;
        locals.var_mu_dn0 = assign34860_e40027_d_n0;
        locals.var_mu_dn2 = assign34860_e40027_d_n2;
        locals.var_mu_dn4 = assign34860_e40027_d_n4;
        locals.var_mu_dn5 = assign34860_e40027_d_n5;
        locals.var_mu_dn6 = assign34860_e40027_d_n6;
        locals.var_mu_dn7 = assign34860_e40027_d_n7;
        locals.var_mu_dn8 = assign34860_e40027_d_n8;
        locals.var_mu_dn9 = assign34860_e40027_d_n9;
        locals.var_mu_dn10 = assign34860_e40027_d_n10;
        locals.var_mu_dn11 = assign34860_e40027_d_n11;
        locals.var_mu_dn14 = assign34860_e40027_d_n14;
        locals.var_mu_rv = 0.0;

        let assign34870_e40030: f64 = if locals.var_vdsorg > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard812 = assign34870_e40030;
        locals.var_guard812_rv = 0.0;

        let (assign34880_e40042, assign34880_e40042_d_n0, assign34880_e40042_d_n2, assign34880_e40042_d_n4, assign34880_e40042_d_n5, assign34880_e40042_d_n6, assign34880_e40042_d_n7, assign34880_e40042_d_n8, assign34880_e40042_d_n9, assign34880_e40042_d_n10, assign34880_e40042_d_n11, assign34880_e40042_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) {
        let assign34880_e40039: f64 = (locals.var_cox * locals.var_cox);
        let assign34880_e40040: f64 = (locals.var_q_ndepm_esi / assign34880_e40039);
        (assign34880_e40040, (((locals.var_q_ndepm_esi_dn0 * assign34880_e40039) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn0 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn0)))) / (assign34880_e40039 * assign34880_e40039)), (((locals.var_q_ndepm_esi_dn2 * assign34880_e40039) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn2 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn2)))) / (assign34880_e40039 * assign34880_e40039)), (((locals.var_q_ndepm_esi_dn4 * assign34880_e40039) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn4 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn4)))) / (assign34880_e40039 * assign34880_e40039)), (((locals.var_q_ndepm_esi_dn5 * assign34880_e40039) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn5 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn5)))) / (assign34880_e40039 * assign34880_e40039)), (((locals.var_q_ndepm_esi_dn6 * assign34880_e40039) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn6 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn6)))) / (assign34880_e40039 * assign34880_e40039)), (((locals.var_q_ndepm_esi_dn7 * assign34880_e40039) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn7 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn7)))) / (assign34880_e40039 * assign34880_e40039)), (((locals.var_q_ndepm_esi_dn8 * assign34880_e40039) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn8 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn8)))) / (assign34880_e40039 * assign34880_e40039)), (((locals.var_q_ndepm_esi_dn9 * assign34880_e40039) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn9 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn9)))) / (assign34880_e40039 * assign34880_e40039)), (((locals.var_q_ndepm_esi_dn10 * assign34880_e40039) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn10 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn10)))) / (assign34880_e40039 * assign34880_e40039)), (((locals.var_q_ndepm_esi_dn11 * assign34880_e40039) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn11 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn11)))) / (assign34880_e40039 * assign34880_e40039)), (((locals.var_q_ndepm_esi_dn14 * assign34880_e40039) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn14 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn14)))) / (assign34880_e40039 * assign34880_e40039)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign34880_e40042;
        locals.var_t2_dn0 = assign34880_e40042_d_n0;
        locals.var_t2_dn2 = assign34880_e40042_d_n2;
        locals.var_t2_dn4 = assign34880_e40042_d_n4;
        locals.var_t2_dn5 = assign34880_e40042_d_n5;
        locals.var_t2_dn6 = assign34880_e40042_d_n6;
        locals.var_t2_dn7 = assign34880_e40042_d_n7;
        locals.var_t2_dn8 = assign34880_e40042_d_n8;
        locals.var_t2_dn9 = assign34880_e40042_d_n9;
        locals.var_t2_dn10 = assign34880_e40042_d_n10;
        locals.var_t2_dn11 = assign34880_e40042_d_n11;
        locals.var_t2_dn14 = assign34880_e40042_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign34890_e40056, assign34890_e40056_d_n0, assign34890_e40056_d_n2, assign34890_e40056_d_n4, assign34890_e40056_d_n5, assign34890_e40056_d_n6, assign34890_e40056_d_n7, assign34890_e40056_d_n8, assign34890_e40056_d_n9, assign34890_e40056_d_n10, assign34890_e40056_d_n11, assign34890_e40056_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) {
        let assign34890_e40050: f64 = (locals.var_vgp + locals.var_uc_depvdsef1);
        let assign34890_e40052: f64 = (assign34890_e40050 - locals.var_beta_inv);
        let assign34890_e40054: f64 = (assign34890_e40052 - locals.var_vbsz__blk442);
        (assign34890_e40054, (((locals.var_vgp_dn0 + locals.var_uc_depvdsef1_dn0) - locals.var_beta_inv_dn0) - locals.var_vbsz__blk442_dn0), (((locals.var_vgp_dn2 + locals.var_uc_depvdsef1_dn2) - locals.var_beta_inv_dn2) - locals.var_vbsz__blk442_dn2), (((locals.var_vgp_dn4 + locals.var_uc_depvdsef1_dn4) - locals.var_beta_inv_dn4) - locals.var_vbsz__blk442_dn4), (((locals.var_vgp_dn5 + locals.var_uc_depvdsef1_dn5) - locals.var_beta_inv_dn5) - locals.var_vbsz__blk442_dn5), (((locals.var_vgp_dn6 + locals.var_uc_depvdsef1_dn6) - locals.var_beta_inv_dn6) - locals.var_vbsz__blk442_dn6), (((locals.var_vgp_dn7 + locals.var_uc_depvdsef1_dn7) - locals.var_beta_inv_dn7) - locals.var_vbsz__blk442_dn7), (((locals.var_vgp_dn8 + locals.var_uc_depvdsef1_dn8) - locals.var_beta_inv_dn8) - locals.var_vbsz__blk442_dn8), (((locals.var_vgp_dn9 + locals.var_uc_depvdsef1_dn9) - locals.var_beta_inv_dn9) - locals.var_vbsz__blk442_dn9), (((locals.var_vgp_dn10 + locals.var_uc_depvdsef1_dn10) - locals.var_beta_inv_dn10) - locals.var_vbsz__blk442_dn10), (((locals.var_vgp_dn11 + locals.var_uc_depvdsef1_dn11) - locals.var_beta_inv_dn11) - locals.var_vbsz__blk442_dn11), (((locals.var_vgp_dn14 + locals.var_uc_depvdsef1_dn14) - locals.var_beta_inv_dn14) - locals.var_vbsz__blk442_dn14),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign34890_e40056;
        locals.var_t0_dn0 = assign34890_e40056_d_n0;
        locals.var_t0_dn2 = assign34890_e40056_d_n2;
        locals.var_t0_dn4 = assign34890_e40056_d_n4;
        locals.var_t0_dn5 = assign34890_e40056_d_n5;
        locals.var_t0_dn6 = assign34890_e40056_d_n6;
        locals.var_t0_dn7 = assign34890_e40056_d_n7;
        locals.var_t0_dn8 = assign34890_e40056_d_n8;
        locals.var_t0_dn9 = assign34890_e40056_d_n9;
        locals.var_t0_dn10 = assign34890_e40056_d_n10;
        locals.var_t0_dn11 = assign34890_e40056_d_n11;
        locals.var_t0_dn14 = assign34890_e40056_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign34900_e40070, assign34900_e40070_d_n0, assign34900_e40070_d_n2, assign34900_e40070_d_n4, assign34900_e40070_d_n5, assign34900_e40070_d_n6, assign34900_e40070_d_n7, assign34900_e40070_d_n8, assign34900_e40070_d_n9, assign34900_e40070_d_n10, assign34900_e40070_d_n11, assign34900_e40070_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) {
        let assign34900_e40065: f64 = (2.0 / locals.var_t2);
        let assign34900_e40067: f64 = (assign34900_e40065 * locals.var_t0);
        let assign34900_e40068: f64 = (1.0 + assign34900_e40067);
        (assign34900_e40068, (((-((2.0 * locals.var_t2_dn0) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign34900_e40065 * locals.var_t0_dn0)), (((-((2.0 * locals.var_t2_dn2) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign34900_e40065 * locals.var_t0_dn2)), (((-((2.0 * locals.var_t2_dn4) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign34900_e40065 * locals.var_t0_dn4)), (((-((2.0 * locals.var_t2_dn5) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign34900_e40065 * locals.var_t0_dn5)), (((-((2.0 * locals.var_t2_dn6) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign34900_e40065 * locals.var_t0_dn6)), (((-((2.0 * locals.var_t2_dn7) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign34900_e40065 * locals.var_t0_dn7)), (((-((2.0 * locals.var_t2_dn8) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign34900_e40065 * locals.var_t0_dn8)), (((-((2.0 * locals.var_t2_dn9) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign34900_e40065 * locals.var_t0_dn9)), (((-((2.0 * locals.var_t2_dn10) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign34900_e40065 * locals.var_t0_dn10)), (((-((2.0 * locals.var_t2_dn11) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign34900_e40065 * locals.var_t0_dn11)), (((-((2.0 * locals.var_t2_dn14) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign34900_e40065 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign34900_e40070;
        locals.var_t4_dn0 = assign34900_e40070_d_n0;
        locals.var_t4_dn2 = assign34900_e40070_d_n2;
        locals.var_t4_dn4 = assign34900_e40070_d_n4;
        locals.var_t4_dn5 = assign34900_e40070_d_n5;
        locals.var_t4_dn6 = assign34900_e40070_d_n6;
        locals.var_t4_dn7 = assign34900_e40070_d_n7;
        locals.var_t4_dn8 = assign34900_e40070_d_n8;
        locals.var_t4_dn9 = assign34900_e40070_d_n9;
        locals.var_t4_dn10 = assign34900_e40070_d_n10;
        locals.var_t4_dn11 = assign34900_e40070_d_n11;
        locals.var_t4_dn14 = assign34900_e40070_d_n14;
        locals.var_t4_rv = 0.0;

        let assign34910_e40074: f64 = 2.0;
        let assign34910_e40079: f64 = if ((locals.var_t4 < assign34910_e40074) && (2.0 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard813 = assign34910_e40079;
        locals.var_guard813_rv = 0.0;

        let (assign34920_e40093, assign34920_e40093_d_n0, assign34920_e40093_d_n2, assign34920_e40093_d_n4, assign34920_e40093_d_n5, assign34920_e40093_d_n6, assign34920_e40093_d_n7, assign34920_e40093_d_n8, assign34920_e40093_d_n9, assign34920_e40093_d_n10, assign34920_e40093_d_n11, assign34920_e40093_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) {
        let assign34920_e40089: f64 = 2.0;
        let assign34920_e40091: f64 = (assign34920_e40089 - locals.var_t4);
        (assign34920_e40091, (-locals.var_t4_dn0), (-locals.var_t4_dn2), (-locals.var_t4_dn4), (-locals.var_t4_dn5), (-locals.var_t4_dn6), (-locals.var_t4_dn7), (-locals.var_t4_dn8), (-locals.var_t4_dn9), (-locals.var_t4_dn10), (-locals.var_t4_dn11), (-locals.var_t4_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign34920_e40093;
        locals.var_tmf1_dn0 = assign34920_e40093_d_n0;
        locals.var_tmf1_dn2 = assign34920_e40093_d_n2;
        locals.var_tmf1_dn4 = assign34920_e40093_d_n4;
        locals.var_tmf1_dn5 = assign34920_e40093_d_n5;
        locals.var_tmf1_dn6 = assign34920_e40093_d_n6;
        locals.var_tmf1_dn7 = assign34920_e40093_d_n7;
        locals.var_tmf1_dn8 = assign34920_e40093_d_n8;
        locals.var_tmf1_dn9 = assign34920_e40093_d_n9;
        locals.var_tmf1_dn10 = assign34920_e40093_d_n10;
        locals.var_tmf1_dn11 = assign34920_e40093_d_n11;
        locals.var_tmf1_dn14 = assign34920_e40093_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign34930_e40105, assign34930_e40105_d_n0, assign34930_e40105_d_n2, assign34930_e40105_d_n4, assign34930_e40105_d_n5, assign34930_e40105_d_n6, assign34930_e40105_d_n7, assign34930_e40105_d_n8, assign34930_e40105_d_n9, assign34930_e40105_d_n10, assign34930_e40105_d_n11, assign34930_e40105_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) {
        let assign34930_e40103: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign34930_e40103, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign34930_e40105;
        locals.var_x2_dn0 = assign34930_e40105_d_n0;
        locals.var_x2_dn2 = assign34930_e40105_d_n2;
        locals.var_x2_dn4 = assign34930_e40105_d_n4;
        locals.var_x2_dn5 = assign34930_e40105_d_n5;
        locals.var_x2_dn6 = assign34930_e40105_d_n6;
        locals.var_x2_dn7 = assign34930_e40105_d_n7;
        locals.var_x2_dn8 = assign34930_e40105_d_n8;
        locals.var_x2_dn9 = assign34930_e40105_d_n9;
        locals.var_x2_dn10 = assign34930_e40105_d_n10;
        locals.var_x2_dn11 = assign34930_e40105_d_n11;
        locals.var_x2_dn14 = assign34930_e40105_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign34940_e40117, assign34940_e40117_d_n0, assign34940_e40117_d_n2, assign34940_e40117_d_n4, assign34940_e40117_d_n5, assign34940_e40117_d_n6, assign34940_e40117_d_n7, assign34940_e40117_d_n8, assign34940_e40117_d_n9, assign34940_e40117_d_n10, assign34940_e40117_d_n11, assign34940_e40117_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) {
        let assign34940_e40115: f64 = (2.0 * 2.0);
        (assign34940_e40115, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign34940_e40117;
        locals.var_xmax2_dn0 = assign34940_e40117_d_n0;
        locals.var_xmax2_dn2 = assign34940_e40117_d_n2;
        locals.var_xmax2_dn4 = assign34940_e40117_d_n4;
        locals.var_xmax2_dn5 = assign34940_e40117_d_n5;
        locals.var_xmax2_dn6 = assign34940_e40117_d_n6;
        locals.var_xmax2_dn7 = assign34940_e40117_d_n7;
        locals.var_xmax2_dn8 = assign34940_e40117_d_n8;
        locals.var_xmax2_dn9 = assign34940_e40117_d_n9;
        locals.var_xmax2_dn10 = assign34940_e40117_d_n10;
        locals.var_xmax2_dn11 = assign34940_e40117_d_n11;
        locals.var_xmax2_dn14 = assign34940_e40117_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign34950_e40127, assign34950_e40127_d_n0, assign34950_e40127_d_n2, assign34950_e40127_d_n4, assign34950_e40127_d_n5, assign34950_e40127_d_n6, assign34950_e40127_d_n7, assign34950_e40127_d_n8, assign34950_e40127_d_n9, assign34950_e40127_d_n10, assign34950_e40127_d_n11, assign34950_e40127_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign34950_e40127;
        locals.var_xp_dn0 = assign34950_e40127_d_n0;
        locals.var_xp_dn2 = assign34950_e40127_d_n2;
        locals.var_xp_dn4 = assign34950_e40127_d_n4;
        locals.var_xp_dn5 = assign34950_e40127_d_n5;
        locals.var_xp_dn6 = assign34950_e40127_d_n6;
        locals.var_xp_dn7 = assign34950_e40127_d_n7;
        locals.var_xp_dn8 = assign34950_e40127_d_n8;
        locals.var_xp_dn9 = assign34950_e40127_d_n9;
        locals.var_xp_dn10 = assign34950_e40127_d_n10;
        locals.var_xp_dn11 = assign34950_e40127_d_n11;
        locals.var_xp_dn14 = assign34950_e40127_d_n14;
        locals.var_xp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_114(
        locals: &mut StampLocals,
    ) {
        let (assign34960_e40137, assign34960_e40137_d_n0, assign34960_e40137_d_n2, assign34960_e40137_d_n4, assign34960_e40137_d_n5, assign34960_e40137_d_n6, assign34960_e40137_d_n7, assign34960_e40137_d_n8, assign34960_e40137_d_n9, assign34960_e40137_d_n10, assign34960_e40137_d_n11, assign34960_e40137_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign34960_e40137;
        locals.var_xmp_dn0 = assign34960_e40137_d_n0;
        locals.var_xmp_dn2 = assign34960_e40137_d_n2;
        locals.var_xmp_dn4 = assign34960_e40137_d_n4;
        locals.var_xmp_dn5 = assign34960_e40137_d_n5;
        locals.var_xmp_dn6 = assign34960_e40137_d_n6;
        locals.var_xmp_dn7 = assign34960_e40137_d_n7;
        locals.var_xmp_dn8 = assign34960_e40137_d_n8;
        locals.var_xmp_dn9 = assign34960_e40137_d_n9;
        locals.var_xmp_dn10 = assign34960_e40137_d_n10;
        locals.var_xmp_dn11 = assign34960_e40137_d_n11;
        locals.var_xmp_dn14 = assign34960_e40137_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign34970_e40147,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign34970_e40147;
        locals.var_m0_rv = 0.0;

        let (assign34980_e40157,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign34980_e40157;
        locals.var_mm_rv = 0.0;

        let (assign34990_e40167, assign34990_e40167_d_n0, assign34990_e40167_d_n2, assign34990_e40167_d_n4, assign34990_e40167_d_n5, assign34990_e40167_d_n6, assign34990_e40167_d_n7, assign34990_e40167_d_n8, assign34990_e40167_d_n9, assign34990_e40167_d_n10, assign34990_e40167_d_n11, assign34990_e40167_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign34990_e40167;
        locals.var_arg_dn0 = assign34990_e40167_d_n0;
        locals.var_arg_dn2 = assign34990_e40167_d_n2;
        locals.var_arg_dn4 = assign34990_e40167_d_n4;
        locals.var_arg_dn5 = assign34990_e40167_d_n5;
        locals.var_arg_dn6 = assign34990_e40167_d_n6;
        locals.var_arg_dn7 = assign34990_e40167_d_n7;
        locals.var_arg_dn8 = assign34990_e40167_d_n8;
        locals.var_arg_dn9 = assign34990_e40167_d_n9;
        locals.var_arg_dn10 = assign34990_e40167_d_n10;
        locals.var_arg_dn11 = assign34990_e40167_d_n11;
        locals.var_arg_dn14 = assign34990_e40167_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign35000_e40177, assign35000_e40177_d_n0, assign35000_e40177_d_n2, assign35000_e40177_d_n4, assign35000_e40177_d_n5, assign35000_e40177_d_n6, assign35000_e40177_d_n7, assign35000_e40177_d_n8, assign35000_e40177_d_n9, assign35000_e40177_d_n10, assign35000_e40177_d_n11, assign35000_e40177_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign35000_e40177;
        locals.var_dnm_dn0 = assign35000_e40177_d_n0;
        locals.var_dnm_dn2 = assign35000_e40177_d_n2;
        locals.var_dnm_dn4 = assign35000_e40177_d_n4;
        locals.var_dnm_dn5 = assign35000_e40177_d_n5;
        locals.var_dnm_dn6 = assign35000_e40177_d_n6;
        locals.var_dnm_dn7 = assign35000_e40177_d_n7;
        locals.var_dnm_dn8 = assign35000_e40177_d_n8;
        locals.var_dnm_dn9 = assign35000_e40177_d_n9;
        locals.var_dnm_dn10 = assign35000_e40177_d_n10;
        locals.var_dnm_dn11 = assign35000_e40177_d_n11;
        locals.var_dnm_dn14 = assign35000_e40177_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign35010_e40189, assign35010_e40189_d_n0, assign35010_e40189_d_n2, assign35010_e40189_d_n4, assign35010_e40189_d_n5, assign35010_e40189_d_n6, assign35010_e40189_d_n7, assign35010_e40189_d_n8, assign35010_e40189_d_n9, assign35010_e40189_d_n10, assign35010_e40189_d_n11, assign35010_e40189_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) {
        let assign35010_e40187: f64 = (locals.var_xp * locals.var_x2);
        (assign35010_e40187, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign35010_e40189;
        locals.var_xp_dn0 = assign35010_e40189_d_n0;
        locals.var_xp_dn2 = assign35010_e40189_d_n2;
        locals.var_xp_dn4 = assign35010_e40189_d_n4;
        locals.var_xp_dn5 = assign35010_e40189_d_n5;
        locals.var_xp_dn6 = assign35010_e40189_d_n6;
        locals.var_xp_dn7 = assign35010_e40189_d_n7;
        locals.var_xp_dn8 = assign35010_e40189_d_n8;
        locals.var_xp_dn9 = assign35010_e40189_d_n9;
        locals.var_xp_dn10 = assign35010_e40189_d_n10;
        locals.var_xp_dn11 = assign35010_e40189_d_n11;
        locals.var_xp_dn14 = assign35010_e40189_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign35020_e40201, assign35020_e40201_d_n0, assign35020_e40201_d_n2, assign35020_e40201_d_n4, assign35020_e40201_d_n5, assign35020_e40201_d_n6, assign35020_e40201_d_n7, assign35020_e40201_d_n8, assign35020_e40201_d_n9, assign35020_e40201_d_n10, assign35020_e40201_d_n11, assign35020_e40201_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) {
        let assign35020_e40199: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign35020_e40199, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign35020_e40201;
        locals.var_xmp_dn0 = assign35020_e40201_d_n0;
        locals.var_xmp_dn2 = assign35020_e40201_d_n2;
        locals.var_xmp_dn4 = assign35020_e40201_d_n4;
        locals.var_xmp_dn5 = assign35020_e40201_d_n5;
        locals.var_xmp_dn6 = assign35020_e40201_d_n6;
        locals.var_xmp_dn7 = assign35020_e40201_d_n7;
        locals.var_xmp_dn8 = assign35020_e40201_d_n8;
        locals.var_xmp_dn9 = assign35020_e40201_d_n9;
        locals.var_xmp_dn10 = assign35020_e40201_d_n10;
        locals.var_xmp_dn11 = assign35020_e40201_d_n11;
        locals.var_xmp_dn14 = assign35020_e40201_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign35030_e40213, assign35030_e40213_d_n0, assign35030_e40213_d_n2, assign35030_e40213_d_n4, assign35030_e40213_d_n5, assign35030_e40213_d_n6, assign35030_e40213_d_n7, assign35030_e40213_d_n8, assign35030_e40213_d_n9, assign35030_e40213_d_n10, assign35030_e40213_d_n11, assign35030_e40213_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) {
        let assign35030_e40211: f64 = (locals.var_xp * locals.var_x2);
        (assign35030_e40211, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign35030_e40213;
        locals.var_xp_dn0 = assign35030_e40213_d_n0;
        locals.var_xp_dn2 = assign35030_e40213_d_n2;
        locals.var_xp_dn4 = assign35030_e40213_d_n4;
        locals.var_xp_dn5 = assign35030_e40213_d_n5;
        locals.var_xp_dn6 = assign35030_e40213_d_n6;
        locals.var_xp_dn7 = assign35030_e40213_d_n7;
        locals.var_xp_dn8 = assign35030_e40213_d_n8;
        locals.var_xp_dn9 = assign35030_e40213_d_n9;
        locals.var_xp_dn10 = assign35030_e40213_d_n10;
        locals.var_xp_dn11 = assign35030_e40213_d_n11;
        locals.var_xp_dn14 = assign35030_e40213_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign35040_e40225, assign35040_e40225_d_n0, assign35040_e40225_d_n2, assign35040_e40225_d_n4, assign35040_e40225_d_n5, assign35040_e40225_d_n6, assign35040_e40225_d_n7, assign35040_e40225_d_n8, assign35040_e40225_d_n9, assign35040_e40225_d_n10, assign35040_e40225_d_n11, assign35040_e40225_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) {
        let assign35040_e40223: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign35040_e40223, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign35040_e40225;
        locals.var_xmp_dn0 = assign35040_e40225_d_n0;
        locals.var_xmp_dn2 = assign35040_e40225_d_n2;
        locals.var_xmp_dn4 = assign35040_e40225_d_n4;
        locals.var_xmp_dn5 = assign35040_e40225_d_n5;
        locals.var_xmp_dn6 = assign35040_e40225_d_n6;
        locals.var_xmp_dn7 = assign35040_e40225_d_n7;
        locals.var_xmp_dn8 = assign35040_e40225_d_n8;
        locals.var_xmp_dn9 = assign35040_e40225_d_n9;
        locals.var_xmp_dn10 = assign35040_e40225_d_n10;
        locals.var_xmp_dn11 = assign35040_e40225_d_n11;
        locals.var_xmp_dn14 = assign35040_e40225_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign35050_e40237, assign35050_e40237_d_n0, assign35050_e40237_d_n2, assign35050_e40237_d_n4, assign35050_e40237_d_n5, assign35050_e40237_d_n6, assign35050_e40237_d_n7, assign35050_e40237_d_n8, assign35050_e40237_d_n9, assign35050_e40237_d_n10, assign35050_e40237_d_n11, assign35050_e40237_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) {
        let assign35050_e40235: f64 = (locals.var_xp + locals.var_xmp);
        (assign35050_e40235, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign35050_e40237;
        locals.var_arg_dn0 = assign35050_e40237_d_n0;
        locals.var_arg_dn2 = assign35050_e40237_d_n2;
        locals.var_arg_dn4 = assign35050_e40237_d_n4;
        locals.var_arg_dn5 = assign35050_e40237_d_n5;
        locals.var_arg_dn6 = assign35050_e40237_d_n6;
        locals.var_arg_dn7 = assign35050_e40237_d_n7;
        locals.var_arg_dn8 = assign35050_e40237_d_n8;
        locals.var_arg_dn9 = assign35050_e40237_d_n9;
        locals.var_arg_dn10 = assign35050_e40237_d_n10;
        locals.var_arg_dn11 = assign35050_e40237_d_n11;
        locals.var_arg_dn14 = assign35050_e40237_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign35060_e40247, assign35060_e40247_d_n0, assign35060_e40247_d_n2, assign35060_e40247_d_n4, assign35060_e40247_d_n5, assign35060_e40247_d_n6, assign35060_e40247_d_n7, assign35060_e40247_d_n8, assign35060_e40247_d_n9, assign35060_e40247_d_n10, assign35060_e40247_d_n11, assign35060_e40247_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign35060_e40247;
        locals.var_dnm_dn0 = assign35060_e40247_d_n0;
        locals.var_dnm_dn2 = assign35060_e40247_d_n2;
        locals.var_dnm_dn4 = assign35060_e40247_d_n4;
        locals.var_dnm_dn5 = assign35060_e40247_d_n5;
        locals.var_dnm_dn6 = assign35060_e40247_d_n6;
        locals.var_dnm_dn7 = assign35060_e40247_d_n7;
        locals.var_dnm_dn8 = assign35060_e40247_d_n8;
        locals.var_dnm_dn9 = assign35060_e40247_d_n9;
        locals.var_dnm_dn10 = assign35060_e40247_d_n10;
        locals.var_dnm_dn11 = assign35060_e40247_d_n11;
        locals.var_dnm_dn14 = assign35060_e40247_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign35070_e40262: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard814 = assign35070_e40262;
        locals.var_guard814_rv = 0.0;

        let assign35080_e40265: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard815 = assign35080_e40265;
        locals.var_guard815_rv = 0.0;

        let (assign35090_e40279,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) && (locals.var_guard814 != 0.0)) && (locals.var_guard815 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign35090_e40279;
        locals.var_mm_rv = 0.0;

        let assign35100_e40282: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard816 = assign35100_e40282;
        locals.var_guard816_rv = 0.0;

        let (assign35110_e40299,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) && (locals.var_guard814 != 0.0)) && (locals.var_guard815 == 0.0)) && (locals.var_guard816 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign35110_e40299;
        locals.var_mm_rv = 0.0;

        let assign35120_e40302: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard817 = assign35120_e40302;
        locals.var_guard817_rv = 0.0;

        let (assign35130_e40322,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) && (locals.var_guard814 != 0.0)) && (locals.var_guard815 == 0.0)) && (locals.var_guard816 == 0.0)) && (locals.var_guard817 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign35130_e40322;
        locals.var_mm_rv = 0.0;

        let assign35140_e40325: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard818 = assign35140_e40325;
        locals.var_guard818_rv = 0.0;

        let (assign35150_e40348,) = {
    if (((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) && (locals.var_guard814 != 0.0)) && (locals.var_guard815 == 0.0)) && (locals.var_guard816 == 0.0)) && (locals.var_guard817 == 0.0)) && (locals.var_guard818 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign35150_e40348;
        locals.var_mm_rv = 0.0;

        let (assign35160_e40360,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) && (locals.var_guard814 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign35160_e40360;
        locals.var_m0_rv = 0.0;

        let mut assign35170_loop_guard: usize = 0;
        while {
            let assign35170_cond_e40373: f64 = if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) && (locals.var_guard814 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign35170_cond_e40373 != 0.0
        } {
            assign35170_loop_guard += 1;
            assert!(assign35170_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign35170_body0_e40386, assign35170_body0_e40386_d_n0, assign35170_body0_e40386_d_n2, assign35170_body0_e40386_d_n4, assign35170_body0_e40386_d_n5, assign35170_body0_e40386_d_n6, assign35170_body0_e40386_d_n7, assign35170_body0_e40386_d_n8, assign35170_body0_e40386_d_n9, assign35170_body0_e40386_d_n10, assign35170_body0_e40386_d_n11, assign35170_body0_e40386_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) && (locals.var_guard814 != 0.0)) {
        let assign35170_body0_e40384: f64 = (locals.var_dnm).sqrt();
        (assign35170_body0_e40384, (locals.var_dnm_dn0 / (2.0 * assign35170_body0_e40384)), (locals.var_dnm_dn2 / (2.0 * assign35170_body0_e40384)), (locals.var_dnm_dn4 / (2.0 * assign35170_body0_e40384)), (locals.var_dnm_dn5 / (2.0 * assign35170_body0_e40384)), (locals.var_dnm_dn6 / (2.0 * assign35170_body0_e40384)), (locals.var_dnm_dn7 / (2.0 * assign35170_body0_e40384)), (locals.var_dnm_dn8 / (2.0 * assign35170_body0_e40384)), (locals.var_dnm_dn9 / (2.0 * assign35170_body0_e40384)), (locals.var_dnm_dn10 / (2.0 * assign35170_body0_e40384)), (locals.var_dnm_dn11 / (2.0 * assign35170_body0_e40384)), (locals.var_dnm_dn14 / (2.0 * assign35170_body0_e40384)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign35170_body0_e40386;
            locals.var_dnm_dn0 = assign35170_body0_e40386_d_n0;
            locals.var_dnm_dn2 = assign35170_body0_e40386_d_n2;
            locals.var_dnm_dn4 = assign35170_body0_e40386_d_n4;
            locals.var_dnm_dn5 = assign35170_body0_e40386_d_n5;
            locals.var_dnm_dn6 = assign35170_body0_e40386_d_n6;
            locals.var_dnm_dn7 = assign35170_body0_e40386_d_n7;
            locals.var_dnm_dn8 = assign35170_body0_e40386_d_n8;
            locals.var_dnm_dn9 = assign35170_body0_e40386_d_n9;
            locals.var_dnm_dn10 = assign35170_body0_e40386_d_n10;
            locals.var_dnm_dn11 = assign35170_body0_e40386_d_n11;
            locals.var_dnm_dn14 = assign35170_body0_e40386_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign35170_body1_e40400,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) && (locals.var_guard814 != 0.0)) {
        let assign35170_body1_e40398: f64 = (locals.var_m0 + 1.0);
        (assign35170_body1_e40398,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign35170_body1_e40400;
            locals.var_m0_rv = 0.0;
        }

        let (assign35180_e40424, assign35180_e40424_d_n0, assign35180_e40424_d_n2, assign35180_e40424_d_n4, assign35180_e40424_d_n5, assign35180_e40424_d_n6, assign35180_e40424_d_n7, assign35180_e40424_d_n8, assign35180_e40424_d_n9, assign35180_e40424_d_n10, assign35180_e40424_d_n11, assign35180_e40424_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) && (locals.var_guard814 == 0.0)) {
        let (assign35180_e40422, assign35180_e40422_d_n0, assign35180_e40422_d_n2, assign35180_e40422_d_n4, assign35180_e40422_d_n5, assign35180_e40422_d_n6, assign35180_e40422_d_n7, assign35180_e40422_d_n8, assign35180_e40422_d_n9, assign35180_e40422_d_n10, assign35180_e40422_d_n11, assign35180_e40422_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign35180_e40419: f64 = (2.0 * 2.0);
                let assign35180_e40420: f64 = (1.0 / assign35180_e40419);
                let assign35180_e40421: f64 = (locals.var_dnm).powf(assign35180_e40420);
                (assign35180_e40421, if 0.0 == 0.0 && ((assign35180_e40420) as f64).is_finite() && ((assign35180_e40420) as f64).fract() == 0.0 { if assign35180_e40420 == 0.0 { 0.0 } else { (assign35180_e40420 * ((locals.var_dnm).powf(assign35180_e40420 - 1.0) * locals.var_dnm_dn0)) } } else { (assign35180_e40421 * (assign35180_e40420 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35180_e40420) as f64).is_finite() && ((assign35180_e40420) as f64).fract() == 0.0 { if assign35180_e40420 == 0.0 { 0.0 } else { (assign35180_e40420 * ((locals.var_dnm).powf(assign35180_e40420 - 1.0) * locals.var_dnm_dn2)) } } else { (assign35180_e40421 * (assign35180_e40420 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35180_e40420) as f64).is_finite() && ((assign35180_e40420) as f64).fract() == 0.0 { if assign35180_e40420 == 0.0 { 0.0 } else { (assign35180_e40420 * ((locals.var_dnm).powf(assign35180_e40420 - 1.0) * locals.var_dnm_dn4)) } } else { (assign35180_e40421 * (assign35180_e40420 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35180_e40420) as f64).is_finite() && ((assign35180_e40420) as f64).fract() == 0.0 { if assign35180_e40420 == 0.0 { 0.0 } else { (assign35180_e40420 * ((locals.var_dnm).powf(assign35180_e40420 - 1.0) * locals.var_dnm_dn5)) } } else { (assign35180_e40421 * (assign35180_e40420 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35180_e40420) as f64).is_finite() && ((assign35180_e40420) as f64).fract() == 0.0 { if assign35180_e40420 == 0.0 { 0.0 } else { (assign35180_e40420 * ((locals.var_dnm).powf(assign35180_e40420 - 1.0) * locals.var_dnm_dn6)) } } else { (assign35180_e40421 * (assign35180_e40420 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35180_e40420) as f64).is_finite() && ((assign35180_e40420) as f64).fract() == 0.0 { if assign35180_e40420 == 0.0 { 0.0 } else { (assign35180_e40420 * ((locals.var_dnm).powf(assign35180_e40420 - 1.0) * locals.var_dnm_dn7)) } } else { (assign35180_e40421 * (assign35180_e40420 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35180_e40420) as f64).is_finite() && ((assign35180_e40420) as f64).fract() == 0.0 { if assign35180_e40420 == 0.0 { 0.0 } else { (assign35180_e40420 * ((locals.var_dnm).powf(assign35180_e40420 - 1.0) * locals.var_dnm_dn8)) } } else { (assign35180_e40421 * (assign35180_e40420 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35180_e40420) as f64).is_finite() && ((assign35180_e40420) as f64).fract() == 0.0 { if assign35180_e40420 == 0.0 { 0.0 } else { (assign35180_e40420 * ((locals.var_dnm).powf(assign35180_e40420 - 1.0) * locals.var_dnm_dn9)) } } else { (assign35180_e40421 * (assign35180_e40420 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35180_e40420) as f64).is_finite() && ((assign35180_e40420) as f64).fract() == 0.0 { if assign35180_e40420 == 0.0 { 0.0 } else { (assign35180_e40420 * ((locals.var_dnm).powf(assign35180_e40420 - 1.0) * locals.var_dnm_dn10)) } } else { (assign35180_e40421 * (assign35180_e40420 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35180_e40420) as f64).is_finite() && ((assign35180_e40420) as f64).fract() == 0.0 { if assign35180_e40420 == 0.0 { 0.0 } else { (assign35180_e40420 * ((locals.var_dnm).powf(assign35180_e40420 - 1.0) * locals.var_dnm_dn11)) } } else { (assign35180_e40421 * (assign35180_e40420 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35180_e40420) as f64).is_finite() && ((assign35180_e40420) as f64).fract() == 0.0 { if assign35180_e40420 == 0.0 { 0.0 } else { (assign35180_e40420 * ((locals.var_dnm).powf(assign35180_e40420 - 1.0) * locals.var_dnm_dn14)) } } else { (assign35180_e40421 * (assign35180_e40420 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign35180_e40422, assign35180_e40422_d_n0, assign35180_e40422_d_n2, assign35180_e40422_d_n4, assign35180_e40422_d_n5, assign35180_e40422_d_n6, assign35180_e40422_d_n7, assign35180_e40422_d_n8, assign35180_e40422_d_n9, assign35180_e40422_d_n10, assign35180_e40422_d_n11, assign35180_e40422_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign35180_e40424;
        locals.var_dnm_dn0 = assign35180_e40424_d_n0;
        locals.var_dnm_dn2 = assign35180_e40424_d_n2;
        locals.var_dnm_dn4 = assign35180_e40424_d_n4;
        locals.var_dnm_dn5 = assign35180_e40424_d_n5;
        locals.var_dnm_dn6 = assign35180_e40424_d_n6;
        locals.var_dnm_dn7 = assign35180_e40424_d_n7;
        locals.var_dnm_dn8 = assign35180_e40424_d_n8;
        locals.var_dnm_dn9 = assign35180_e40424_d_n9;
        locals.var_dnm_dn10 = assign35180_e40424_d_n10;
        locals.var_dnm_dn11 = assign35180_e40424_d_n11;
        locals.var_dnm_dn14 = assign35180_e40424_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign35190_e40436, assign35190_e40436_d_n0, assign35190_e40436_d_n2, assign35190_e40436_d_n4, assign35190_e40436_d_n5, assign35190_e40436_d_n6, assign35190_e40436_d_n7, assign35190_e40436_d_n8, assign35190_e40436_d_n9, assign35190_e40436_d_n10, assign35190_e40436_d_n11, assign35190_e40436_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) {
        let assign35190_e40434: f64 = (1.0 / locals.var_dnm);
        (assign35190_e40434, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign35190_e40436;
        locals.var_dnm_dn0 = assign35190_e40436_d_n0;
        locals.var_dnm_dn2 = assign35190_e40436_d_n2;
        locals.var_dnm_dn4 = assign35190_e40436_d_n4;
        locals.var_dnm_dn5 = assign35190_e40436_d_n5;
        locals.var_dnm_dn6 = assign35190_e40436_d_n6;
        locals.var_dnm_dn7 = assign35190_e40436_d_n7;
        locals.var_dnm_dn8 = assign35190_e40436_d_n8;
        locals.var_dnm_dn9 = assign35190_e40436_d_n9;
        locals.var_dnm_dn10 = assign35190_e40436_d_n10;
        locals.var_dnm_dn11 = assign35190_e40436_d_n11;
        locals.var_dnm_dn14 = assign35190_e40436_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign35200_e40450, assign35200_e40450_d_n0, assign35200_e40450_d_n2, assign35200_e40450_d_n4, assign35200_e40450_d_n5, assign35200_e40450_d_n6, assign35200_e40450_d_n7, assign35200_e40450_d_n8, assign35200_e40450_d_n9, assign35200_e40450_d_n10, assign35200_e40450_d_n11, assign35200_e40450_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) {
        let assign35200_e40446: f64 = (locals.var_tmf1 * 2.0);
        let assign35200_e40448: f64 = (assign35200_e40446 * locals.var_dnm);
        (assign35200_e40448, (((locals.var_tmf1_dn0 * 2.0) * locals.var_dnm) + (assign35200_e40446 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 2.0) * locals.var_dnm) + (assign35200_e40446 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 2.0) * locals.var_dnm) + (assign35200_e40446 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 2.0) * locals.var_dnm) + (assign35200_e40446 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 2.0) * locals.var_dnm) + (assign35200_e40446 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 2.0) * locals.var_dnm) + (assign35200_e40446 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 2.0) * locals.var_dnm) + (assign35200_e40446 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 2.0) * locals.var_dnm) + (assign35200_e40446 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 2.0) * locals.var_dnm) + (assign35200_e40446 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 2.0) * locals.var_dnm) + (assign35200_e40446 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 2.0) * locals.var_dnm) + (assign35200_e40446 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign35200_e40450;
        locals.var_tmf0_dn0 = assign35200_e40450_d_n0;
        locals.var_tmf0_dn2 = assign35200_e40450_d_n2;
        locals.var_tmf0_dn4 = assign35200_e40450_d_n4;
        locals.var_tmf0_dn5 = assign35200_e40450_d_n5;
        locals.var_tmf0_dn6 = assign35200_e40450_d_n6;
        locals.var_tmf0_dn7 = assign35200_e40450_d_n7;
        locals.var_tmf0_dn8 = assign35200_e40450_d_n8;
        locals.var_tmf0_dn9 = assign35200_e40450_d_n9;
        locals.var_tmf0_dn10 = assign35200_e40450_d_n10;
        locals.var_tmf0_dn11 = assign35200_e40450_d_n11;
        locals.var_tmf0_dn14 = assign35200_e40450_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign35210_e40466, assign35210_e40466_d_n0, assign35210_e40466_d_n2, assign35210_e40466_d_n4, assign35210_e40466_d_n5, assign35210_e40466_d_n6, assign35210_e40466_d_n7, assign35210_e40466_d_n8, assign35210_e40466_d_n9, assign35210_e40466_d_n10, assign35210_e40466_d_n11, assign35210_e40466_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) {
        let assign35210_e40460: f64 = (2.0 * locals.var_xmp);
        let assign35210_e40462: f64 = (assign35210_e40460 * locals.var_dnm);
        let assign35210_e40464: f64 = (assign35210_e40462 / locals.var_arg);
        (assign35210_e40464, ((((((2.0 * locals.var_xmp_dn0) * locals.var_dnm) + (assign35210_e40460 * locals.var_dnm_dn0)) * locals.var_arg) - (assign35210_e40462 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn2) * locals.var_dnm) + (assign35210_e40460 * locals.var_dnm_dn2)) * locals.var_arg) - (assign35210_e40462 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn4) * locals.var_dnm) + (assign35210_e40460 * locals.var_dnm_dn4)) * locals.var_arg) - (assign35210_e40462 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn5) * locals.var_dnm) + (assign35210_e40460 * locals.var_dnm_dn5)) * locals.var_arg) - (assign35210_e40462 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn6) * locals.var_dnm) + (assign35210_e40460 * locals.var_dnm_dn6)) * locals.var_arg) - (assign35210_e40462 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn7) * locals.var_dnm) + (assign35210_e40460 * locals.var_dnm_dn7)) * locals.var_arg) - (assign35210_e40462 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn8) * locals.var_dnm) + (assign35210_e40460 * locals.var_dnm_dn8)) * locals.var_arg) - (assign35210_e40462 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn9) * locals.var_dnm) + (assign35210_e40460 * locals.var_dnm_dn9)) * locals.var_arg) - (assign35210_e40462 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn10) * locals.var_dnm) + (assign35210_e40460 * locals.var_dnm_dn10)) * locals.var_arg) - (assign35210_e40462 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn11) * locals.var_dnm) + (assign35210_e40460 * locals.var_dnm_dn11)) * locals.var_arg) - (assign35210_e40462 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn14) * locals.var_dnm) + (assign35210_e40460 * locals.var_dnm_dn14)) * locals.var_arg) - (assign35210_e40462 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign35210_e40466;
        locals.var_t0_dn0 = assign35210_e40466_d_n0;
        locals.var_t0_dn2 = assign35210_e40466_d_n2;
        locals.var_t0_dn4 = assign35210_e40466_d_n4;
        locals.var_t0_dn5 = assign35210_e40466_d_n5;
        locals.var_t0_dn6 = assign35210_e40466_d_n6;
        locals.var_t0_dn7 = assign35210_e40466_d_n7;
        locals.var_t0_dn8 = assign35210_e40466_d_n8;
        locals.var_t0_dn9 = assign35210_e40466_d_n9;
        locals.var_t0_dn10 = assign35210_e40466_d_n10;
        locals.var_t0_dn11 = assign35210_e40466_d_n11;
        locals.var_t0_dn14 = assign35210_e40466_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign35220_e40480, assign35220_e40480_d_n0, assign35220_e40480_d_n2, assign35220_e40480_d_n4, assign35220_e40480_d_n5, assign35220_e40480_d_n6, assign35220_e40480_d_n7, assign35220_e40480_d_n8, assign35220_e40480_d_n9, assign35220_e40480_d_n10, assign35220_e40480_d_n11, assign35220_e40480_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) {
        let assign35220_e40476: f64 = 2.0;
        let assign35220_e40478: f64 = (assign35220_e40476 - locals.var_tmf0);
        (assign35220_e40478, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign35220_e40480;
        locals.var_t9_dn0 = assign35220_e40480_d_n0;
        locals.var_t9_dn2 = assign35220_e40480_d_n2;
        locals.var_t9_dn4 = assign35220_e40480_d_n4;
        locals.var_t9_dn5 = assign35220_e40480_d_n5;
        locals.var_t9_dn6 = assign35220_e40480_d_n6;
        locals.var_t9_dn7 = assign35220_e40480_d_n7;
        locals.var_t9_dn8 = assign35220_e40480_d_n8;
        locals.var_t9_dn9 = assign35220_e40480_d_n9;
        locals.var_t9_dn10 = assign35220_e40480_d_n10;
        locals.var_t9_dn11 = assign35220_e40480_d_n11;
        locals.var_t9_dn14 = assign35220_e40480_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign35230_e40490, assign35230_e40490_d_n0, assign35230_e40490_d_n2, assign35230_e40490_d_n4, assign35230_e40490_d_n5, assign35230_e40490_d_n6, assign35230_e40490_d_n7, assign35230_e40490_d_n8, assign35230_e40490_d_n9, assign35230_e40490_d_n10, assign35230_e40490_d_n11, assign35230_e40490_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign35230_e40490;
        locals.var_t0_dn0 = assign35230_e40490_d_n0;
        locals.var_t0_dn2 = assign35230_e40490_d_n2;
        locals.var_t0_dn4 = assign35230_e40490_d_n4;
        locals.var_t0_dn5 = assign35230_e40490_d_n5;
        locals.var_t0_dn6 = assign35230_e40490_d_n6;
        locals.var_t0_dn7 = assign35230_e40490_d_n7;
        locals.var_t0_dn8 = assign35230_e40490_d_n8;
        locals.var_t0_dn9 = assign35230_e40490_d_n9;
        locals.var_t0_dn10 = assign35230_e40490_d_n10;
        locals.var_t0_dn11 = assign35230_e40490_d_n11;
        locals.var_t0_dn14 = assign35230_e40490_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign35240_e40501, assign35240_e40501_d_n0, assign35240_e40501_d_n2, assign35240_e40501_d_n4, assign35240_e40501_d_n5, assign35240_e40501_d_n6, assign35240_e40501_d_n7, assign35240_e40501_d_n8, assign35240_e40501_d_n9, assign35240_e40501_d_n10, assign35240_e40501_d_n11, assign35240_e40501_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 == 0.0)) {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign35240_e40501;
        locals.var_t9_dn0 = assign35240_e40501_d_n0;
        locals.var_t9_dn2 = assign35240_e40501_d_n2;
        locals.var_t9_dn4 = assign35240_e40501_d_n4;
        locals.var_t9_dn5 = assign35240_e40501_d_n5;
        locals.var_t9_dn6 = assign35240_e40501_d_n6;
        locals.var_t9_dn7 = assign35240_e40501_d_n7;
        locals.var_t9_dn8 = assign35240_e40501_d_n8;
        locals.var_t9_dn9 = assign35240_e40501_d_n9;
        locals.var_t9_dn10 = assign35240_e40501_d_n10;
        locals.var_t9_dn11 = assign35240_e40501_d_n11;
        locals.var_t9_dn14 = assign35240_e40501_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign35250_e40512, assign35250_e40512_d_n0, assign35250_e40512_d_n2, assign35250_e40512_d_n4, assign35250_e40512_d_n5, assign35250_e40512_d_n6, assign35250_e40512_d_n7, assign35250_e40512_d_n8, assign35250_e40512_d_n9, assign35250_e40512_d_n10, assign35250_e40512_d_n11, assign35250_e40512_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign35250_e40512;
        locals.var_t0_dn0 = assign35250_e40512_d_n0;
        locals.var_t0_dn2 = assign35250_e40512_d_n2;
        locals.var_t0_dn4 = assign35250_e40512_d_n4;
        locals.var_t0_dn5 = assign35250_e40512_d_n5;
        locals.var_t0_dn6 = assign35250_e40512_d_n6;
        locals.var_t0_dn7 = assign35250_e40512_d_n7;
        locals.var_t0_dn8 = assign35250_e40512_d_n8;
        locals.var_t0_dn9 = assign35250_e40512_d_n9;
        locals.var_t0_dn10 = assign35250_e40512_d_n10;
        locals.var_t0_dn11 = assign35250_e40512_d_n11;
        locals.var_t0_dn14 = assign35250_e40512_d_n14;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_115(
        locals: &mut StampLocals,
    ) {
        let (assign35260_e40522, assign35260_e40522_d_n0, assign35260_e40522_d_n2, assign35260_e40522_d_n4, assign35260_e40522_d_n5, assign35260_e40522_d_n6, assign35260_e40522_d_n7, assign35260_e40522_d_n8, assign35260_e40522_d_n9, assign35260_e40522_d_n10, assign35260_e40522_d_n11, assign35260_e40522_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) {
        let assign35260_e40520: f64 = (locals.var_t9 + 1e-25);
        (assign35260_e40520, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign35260_e40522;
        locals.var_t9_dn0 = assign35260_e40522_d_n0;
        locals.var_t9_dn2 = assign35260_e40522_d_n2;
        locals.var_t9_dn4 = assign35260_e40522_d_n4;
        locals.var_t9_dn5 = assign35260_e40522_d_n5;
        locals.var_t9_dn6 = assign35260_e40522_d_n6;
        locals.var_t9_dn7 = assign35260_e40522_d_n7;
        locals.var_t9_dn8 = assign35260_e40522_d_n8;
        locals.var_t9_dn9 = assign35260_e40522_d_n9;
        locals.var_t9_dn10 = assign35260_e40522_d_n10;
        locals.var_t9_dn11 = assign35260_e40522_d_n11;
        locals.var_t9_dn14 = assign35260_e40522_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign35270_e40531, assign35270_e40531_d_n0, assign35270_e40531_d_n2, assign35270_e40531_d_n4, assign35270_e40531_d_n5, assign35270_e40531_d_n6, assign35270_e40531_d_n7, assign35270_e40531_d_n8, assign35270_e40531_d_n9, assign35270_e40531_d_n10, assign35270_e40531_d_n11, assign35270_e40531_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) {
        let assign35270_e40529: f64 = (locals.var_t9).sqrt();
        (assign35270_e40529, (locals.var_t9_dn0 / (2.0 * assign35270_e40529)), (locals.var_t9_dn2 / (2.0 * assign35270_e40529)), (locals.var_t9_dn4 / (2.0 * assign35270_e40529)), (locals.var_t9_dn5 / (2.0 * assign35270_e40529)), (locals.var_t9_dn6 / (2.0 * assign35270_e40529)), (locals.var_t9_dn7 / (2.0 * assign35270_e40529)), (locals.var_t9_dn8 / (2.0 * assign35270_e40529)), (locals.var_t9_dn9 / (2.0 * assign35270_e40529)), (locals.var_t9_dn10 / (2.0 * assign35270_e40529)), (locals.var_t9_dn11 / (2.0 * assign35270_e40529)), (locals.var_t9_dn14 / (2.0 * assign35270_e40529)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign35270_e40531;
        locals.var_t3_dn0 = assign35270_e40531_d_n0;
        locals.var_t3_dn2 = assign35270_e40531_d_n2;
        locals.var_t3_dn4 = assign35270_e40531_d_n4;
        locals.var_t3_dn5 = assign35270_e40531_d_n5;
        locals.var_t3_dn6 = assign35270_e40531_d_n6;
        locals.var_t3_dn7 = assign35270_e40531_d_n7;
        locals.var_t3_dn8 = assign35270_e40531_d_n8;
        locals.var_t3_dn9 = assign35270_e40531_d_n9;
        locals.var_t3_dn10 = assign35270_e40531_d_n10;
        locals.var_t3_dn11 = assign35270_e40531_d_n11;
        locals.var_t3_dn14 = assign35270_e40531_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign35280_e40543, assign35280_e40543_d_n0, assign35280_e40543_d_n2, assign35280_e40543_d_n4, assign35280_e40543_d_n5, assign35280_e40543_d_n6, assign35280_e40543_d_n7, assign35280_e40543_d_n8, assign35280_e40543_d_n9, assign35280_e40543_d_n10, assign35280_e40543_d_n11, assign35280_e40543_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) {
        let assign35280_e40540: f64 = (1.0 - locals.var_t3);
        let assign35280_e40541: f64 = (locals.var_t2 * assign35280_e40540);
        (assign35280_e40541, ((locals.var_t2_dn0 * assign35280_e40540) + (locals.var_t2 * (-locals.var_t3_dn0))), ((locals.var_t2_dn2 * assign35280_e40540) + (locals.var_t2 * (-locals.var_t3_dn2))), ((locals.var_t2_dn4 * assign35280_e40540) + (locals.var_t2 * (-locals.var_t3_dn4))), ((locals.var_t2_dn5 * assign35280_e40540) + (locals.var_t2 * (-locals.var_t3_dn5))), ((locals.var_t2_dn6 * assign35280_e40540) + (locals.var_t2 * (-locals.var_t3_dn6))), ((locals.var_t2_dn7 * assign35280_e40540) + (locals.var_t2 * (-locals.var_t3_dn7))), ((locals.var_t2_dn8 * assign35280_e40540) + (locals.var_t2 * (-locals.var_t3_dn8))), ((locals.var_t2_dn9 * assign35280_e40540) + (locals.var_t2 * (-locals.var_t3_dn9))), ((locals.var_t2_dn10 * assign35280_e40540) + (locals.var_t2 * (-locals.var_t3_dn10))), ((locals.var_t2_dn11 * assign35280_e40540) + (locals.var_t2 * (-locals.var_t3_dn11))), ((locals.var_t2_dn14 * assign35280_e40540) + (locals.var_t2 * (-locals.var_t3_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign35280_e40543;
        locals.var_t4_dn0 = assign35280_e40543_d_n0;
        locals.var_t4_dn2 = assign35280_e40543_d_n2;
        locals.var_t4_dn4 = assign35280_e40543_d_n4;
        locals.var_t4_dn5 = assign35280_e40543_d_n5;
        locals.var_t4_dn6 = assign35280_e40543_d_n6;
        locals.var_t4_dn7 = assign35280_e40543_d_n7;
        locals.var_t4_dn8 = assign35280_e40543_d_n8;
        locals.var_t4_dn9 = assign35280_e40543_d_n9;
        locals.var_t4_dn10 = assign35280_e40543_d_n10;
        locals.var_t4_dn11 = assign35280_e40543_d_n11;
        locals.var_t4_dn14 = assign35280_e40543_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign35290_e40555, assign35290_e40555_d_n0, assign35290_e40555_d_n2, assign35290_e40555_d_n4, assign35290_e40555_d_n5, assign35290_e40555_d_n6, assign35290_e40555_d_n7, assign35290_e40555_d_n8, assign35290_e40555_d_n9, assign35290_e40555_d_n10, assign35290_e40555_d_n11, assign35290_e40555_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) {
        let assign35290_e40551: f64 = (locals.var_vgp + locals.var_uc_depvdsef1);
        let assign35290_e40553: f64 = (assign35290_e40551 + locals.var_t4);
        (assign35290_e40553, ((locals.var_vgp_dn0 + locals.var_uc_depvdsef1_dn0) + locals.var_t4_dn0), ((locals.var_vgp_dn2 + locals.var_uc_depvdsef1_dn2) + locals.var_t4_dn2), ((locals.var_vgp_dn4 + locals.var_uc_depvdsef1_dn4) + locals.var_t4_dn4), ((locals.var_vgp_dn5 + locals.var_uc_depvdsef1_dn5) + locals.var_t4_dn5), ((locals.var_vgp_dn6 + locals.var_uc_depvdsef1_dn6) + locals.var_t4_dn6), ((locals.var_vgp_dn7 + locals.var_uc_depvdsef1_dn7) + locals.var_t4_dn7), ((locals.var_vgp_dn8 + locals.var_uc_depvdsef1_dn8) + locals.var_t4_dn8), ((locals.var_vgp_dn9 + locals.var_uc_depvdsef1_dn9) + locals.var_t4_dn9), ((locals.var_vgp_dn10 + locals.var_uc_depvdsef1_dn10) + locals.var_t4_dn10), ((locals.var_vgp_dn11 + locals.var_uc_depvdsef1_dn11) + locals.var_t4_dn11), ((locals.var_vgp_dn14 + locals.var_uc_depvdsef1_dn14) + locals.var_t4_dn14),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign35290_e40555;
        locals.var_t10_dn0 = assign35290_e40555_d_n0;
        locals.var_t10_dn2 = assign35290_e40555_d_n2;
        locals.var_t10_dn4 = assign35290_e40555_d_n4;
        locals.var_t10_dn5 = assign35290_e40555_d_n5;
        locals.var_t10_dn6 = assign35290_e40555_d_n6;
        locals.var_t10_dn7 = assign35290_e40555_d_n7;
        locals.var_t10_dn8 = assign35290_e40555_d_n8;
        locals.var_t10_dn9 = assign35290_e40555_d_n9;
        locals.var_t10_dn10 = assign35290_e40555_d_n10;
        locals.var_t10_dn11 = assign35290_e40555_d_n11;
        locals.var_t10_dn14 = assign35290_e40555_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign35300_e40565, assign35300_e40565_d_n0, assign35300_e40565_d_n2, assign35300_e40565_d_n4, assign35300_e40565_d_n5, assign35300_e40565_d_n6, assign35300_e40565_d_n7, assign35300_e40565_d_n8, assign35300_e40565_d_n9, assign35300_e40565_d_n10, assign35300_e40565_d_n11, assign35300_e40565_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) {
        let assign35300_e40563: f64 = (locals.var_t10 * locals.var_uc_depvdsef2);
        (assign35300_e40563, ((locals.var_t10_dn0 * locals.var_uc_depvdsef2) + (locals.var_t10 * locals.var_uc_depvdsef2_dn0)), ((locals.var_t10_dn2 * locals.var_uc_depvdsef2) + (locals.var_t10 * locals.var_uc_depvdsef2_dn2)), ((locals.var_t10_dn4 * locals.var_uc_depvdsef2) + (locals.var_t10 * locals.var_uc_depvdsef2_dn4)), ((locals.var_t10_dn5 * locals.var_uc_depvdsef2) + (locals.var_t10 * locals.var_uc_depvdsef2_dn5)), ((locals.var_t10_dn6 * locals.var_uc_depvdsef2) + (locals.var_t10 * locals.var_uc_depvdsef2_dn6)), ((locals.var_t10_dn7 * locals.var_uc_depvdsef2) + (locals.var_t10 * locals.var_uc_depvdsef2_dn7)), ((locals.var_t10_dn8 * locals.var_uc_depvdsef2) + (locals.var_t10 * locals.var_uc_depvdsef2_dn8)), ((locals.var_t10_dn9 * locals.var_uc_depvdsef2) + (locals.var_t10 * locals.var_uc_depvdsef2_dn9)), ((locals.var_t10_dn10 * locals.var_uc_depvdsef2) + (locals.var_t10 * locals.var_uc_depvdsef2_dn10)), ((locals.var_t10_dn11 * locals.var_uc_depvdsef2) + (locals.var_t10 * locals.var_uc_depvdsef2_dn11)), ((locals.var_t10_dn14 * locals.var_uc_depvdsef2) + (locals.var_t10 * locals.var_uc_depvdsef2_dn14)),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign35300_e40565;
        locals.var_t10_dn0 = assign35300_e40565_d_n0;
        locals.var_t10_dn2 = assign35300_e40565_d_n2;
        locals.var_t10_dn4 = assign35300_e40565_d_n4;
        locals.var_t10_dn5 = assign35300_e40565_d_n5;
        locals.var_t10_dn6 = assign35300_e40565_d_n6;
        locals.var_t10_dn7 = assign35300_e40565_d_n7;
        locals.var_t10_dn8 = assign35300_e40565_d_n8;
        locals.var_t10_dn9 = assign35300_e40565_d_n9;
        locals.var_t10_dn10 = assign35300_e40565_d_n10;
        locals.var_t10_dn11 = assign35300_e40565_d_n11;
        locals.var_t10_dn14 = assign35300_e40565_d_n14;
        locals.var_t10_rv = 0.0;

        let assign35310_e40569: f64 = (locals.var_uc_depleak + 4.0);
        let assign35310_e40574: f64 = if ((locals.var_t10 < assign35310_e40569) && (4.0 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard819 = assign35310_e40574;
        locals.var_guard819_rv = 0.0;

        let (assign35320_e40588, assign35320_e40588_d_n0, assign35320_e40588_d_n2, assign35320_e40588_d_n4, assign35320_e40588_d_n5, assign35320_e40588_d_n6, assign35320_e40588_d_n7, assign35320_e40588_d_n8, assign35320_e40588_d_n9, assign35320_e40588_d_n10, assign35320_e40588_d_n11, assign35320_e40588_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 != 0.0)) {
        let assign35320_e40584: f64 = (locals.var_uc_depleak + 4.0);
        let assign35320_e40586: f64 = (assign35320_e40584 - locals.var_t10);
        (assign35320_e40586, (locals.var_uc_depleak_dn0 - locals.var_t10_dn0), (locals.var_uc_depleak_dn2 - locals.var_t10_dn2), (locals.var_uc_depleak_dn4 - locals.var_t10_dn4), (locals.var_uc_depleak_dn5 - locals.var_t10_dn5), (locals.var_uc_depleak_dn6 - locals.var_t10_dn6), (locals.var_uc_depleak_dn7 - locals.var_t10_dn7), (locals.var_uc_depleak_dn8 - locals.var_t10_dn8), (locals.var_uc_depleak_dn9 - locals.var_t10_dn9), (locals.var_uc_depleak_dn10 - locals.var_t10_dn10), (locals.var_uc_depleak_dn11 - locals.var_t10_dn11), (locals.var_uc_depleak_dn14 - locals.var_t10_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign35320_e40588;
        locals.var_tmf1_dn0 = assign35320_e40588_d_n0;
        locals.var_tmf1_dn2 = assign35320_e40588_d_n2;
        locals.var_tmf1_dn4 = assign35320_e40588_d_n4;
        locals.var_tmf1_dn5 = assign35320_e40588_d_n5;
        locals.var_tmf1_dn6 = assign35320_e40588_d_n6;
        locals.var_tmf1_dn7 = assign35320_e40588_d_n7;
        locals.var_tmf1_dn8 = assign35320_e40588_d_n8;
        locals.var_tmf1_dn9 = assign35320_e40588_d_n9;
        locals.var_tmf1_dn10 = assign35320_e40588_d_n10;
        locals.var_tmf1_dn11 = assign35320_e40588_d_n11;
        locals.var_tmf1_dn14 = assign35320_e40588_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign35330_e40600, assign35330_e40600_d_n0, assign35330_e40600_d_n2, assign35330_e40600_d_n4, assign35330_e40600_d_n5, assign35330_e40600_d_n6, assign35330_e40600_d_n7, assign35330_e40600_d_n8, assign35330_e40600_d_n9, assign35330_e40600_d_n10, assign35330_e40600_d_n11, assign35330_e40600_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 != 0.0)) {
        let assign35330_e40598: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign35330_e40598, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign35330_e40600;
        locals.var_x2_dn0 = assign35330_e40600_d_n0;
        locals.var_x2_dn2 = assign35330_e40600_d_n2;
        locals.var_x2_dn4 = assign35330_e40600_d_n4;
        locals.var_x2_dn5 = assign35330_e40600_d_n5;
        locals.var_x2_dn6 = assign35330_e40600_d_n6;
        locals.var_x2_dn7 = assign35330_e40600_d_n7;
        locals.var_x2_dn8 = assign35330_e40600_d_n8;
        locals.var_x2_dn9 = assign35330_e40600_d_n9;
        locals.var_x2_dn10 = assign35330_e40600_d_n10;
        locals.var_x2_dn11 = assign35330_e40600_d_n11;
        locals.var_x2_dn14 = assign35330_e40600_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign35340_e40612, assign35340_e40612_d_n0, assign35340_e40612_d_n2, assign35340_e40612_d_n4, assign35340_e40612_d_n5, assign35340_e40612_d_n6, assign35340_e40612_d_n7, assign35340_e40612_d_n8, assign35340_e40612_d_n9, assign35340_e40612_d_n10, assign35340_e40612_d_n11, assign35340_e40612_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 != 0.0)) {
        let assign35340_e40610: f64 = (4.0 * 4.0);
        (assign35340_e40610, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign35340_e40612;
        locals.var_xmax2_dn0 = assign35340_e40612_d_n0;
        locals.var_xmax2_dn2 = assign35340_e40612_d_n2;
        locals.var_xmax2_dn4 = assign35340_e40612_d_n4;
        locals.var_xmax2_dn5 = assign35340_e40612_d_n5;
        locals.var_xmax2_dn6 = assign35340_e40612_d_n6;
        locals.var_xmax2_dn7 = assign35340_e40612_d_n7;
        locals.var_xmax2_dn8 = assign35340_e40612_d_n8;
        locals.var_xmax2_dn9 = assign35340_e40612_d_n9;
        locals.var_xmax2_dn10 = assign35340_e40612_d_n10;
        locals.var_xmax2_dn11 = assign35340_e40612_d_n11;
        locals.var_xmax2_dn14 = assign35340_e40612_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign35350_e40622, assign35350_e40622_d_n0, assign35350_e40622_d_n2, assign35350_e40622_d_n4, assign35350_e40622_d_n5, assign35350_e40622_d_n6, assign35350_e40622_d_n7, assign35350_e40622_d_n8, assign35350_e40622_d_n9, assign35350_e40622_d_n10, assign35350_e40622_d_n11, assign35350_e40622_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign35350_e40622;
        locals.var_xp_dn0 = assign35350_e40622_d_n0;
        locals.var_xp_dn2 = assign35350_e40622_d_n2;
        locals.var_xp_dn4 = assign35350_e40622_d_n4;
        locals.var_xp_dn5 = assign35350_e40622_d_n5;
        locals.var_xp_dn6 = assign35350_e40622_d_n6;
        locals.var_xp_dn7 = assign35350_e40622_d_n7;
        locals.var_xp_dn8 = assign35350_e40622_d_n8;
        locals.var_xp_dn9 = assign35350_e40622_d_n9;
        locals.var_xp_dn10 = assign35350_e40622_d_n10;
        locals.var_xp_dn11 = assign35350_e40622_d_n11;
        locals.var_xp_dn14 = assign35350_e40622_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign35360_e40632, assign35360_e40632_d_n0, assign35360_e40632_d_n2, assign35360_e40632_d_n4, assign35360_e40632_d_n5, assign35360_e40632_d_n6, assign35360_e40632_d_n7, assign35360_e40632_d_n8, assign35360_e40632_d_n9, assign35360_e40632_d_n10, assign35360_e40632_d_n11, assign35360_e40632_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign35360_e40632;
        locals.var_xmp_dn0 = assign35360_e40632_d_n0;
        locals.var_xmp_dn2 = assign35360_e40632_d_n2;
        locals.var_xmp_dn4 = assign35360_e40632_d_n4;
        locals.var_xmp_dn5 = assign35360_e40632_d_n5;
        locals.var_xmp_dn6 = assign35360_e40632_d_n6;
        locals.var_xmp_dn7 = assign35360_e40632_d_n7;
        locals.var_xmp_dn8 = assign35360_e40632_d_n8;
        locals.var_xmp_dn9 = assign35360_e40632_d_n9;
        locals.var_xmp_dn10 = assign35360_e40632_d_n10;
        locals.var_xmp_dn11 = assign35360_e40632_d_n11;
        locals.var_xmp_dn14 = assign35360_e40632_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign35370_e40642,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign35370_e40642;
        locals.var_m0_rv = 0.0;

        let (assign35380_e40652,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign35380_e40652;
        locals.var_mm_rv = 0.0;

        let (assign35390_e40662, assign35390_e40662_d_n0, assign35390_e40662_d_n2, assign35390_e40662_d_n4, assign35390_e40662_d_n5, assign35390_e40662_d_n6, assign35390_e40662_d_n7, assign35390_e40662_d_n8, assign35390_e40662_d_n9, assign35390_e40662_d_n10, assign35390_e40662_d_n11, assign35390_e40662_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign35390_e40662;
        locals.var_arg_dn0 = assign35390_e40662_d_n0;
        locals.var_arg_dn2 = assign35390_e40662_d_n2;
        locals.var_arg_dn4 = assign35390_e40662_d_n4;
        locals.var_arg_dn5 = assign35390_e40662_d_n5;
        locals.var_arg_dn6 = assign35390_e40662_d_n6;
        locals.var_arg_dn7 = assign35390_e40662_d_n7;
        locals.var_arg_dn8 = assign35390_e40662_d_n8;
        locals.var_arg_dn9 = assign35390_e40662_d_n9;
        locals.var_arg_dn10 = assign35390_e40662_d_n10;
        locals.var_arg_dn11 = assign35390_e40662_d_n11;
        locals.var_arg_dn14 = assign35390_e40662_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign35400_e40672, assign35400_e40672_d_n0, assign35400_e40672_d_n2, assign35400_e40672_d_n4, assign35400_e40672_d_n5, assign35400_e40672_d_n6, assign35400_e40672_d_n7, assign35400_e40672_d_n8, assign35400_e40672_d_n9, assign35400_e40672_d_n10, assign35400_e40672_d_n11, assign35400_e40672_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign35400_e40672;
        locals.var_dnm_dn0 = assign35400_e40672_d_n0;
        locals.var_dnm_dn2 = assign35400_e40672_d_n2;
        locals.var_dnm_dn4 = assign35400_e40672_d_n4;
        locals.var_dnm_dn5 = assign35400_e40672_d_n5;
        locals.var_dnm_dn6 = assign35400_e40672_d_n6;
        locals.var_dnm_dn7 = assign35400_e40672_d_n7;
        locals.var_dnm_dn8 = assign35400_e40672_d_n8;
        locals.var_dnm_dn9 = assign35400_e40672_d_n9;
        locals.var_dnm_dn10 = assign35400_e40672_d_n10;
        locals.var_dnm_dn11 = assign35400_e40672_d_n11;
        locals.var_dnm_dn14 = assign35400_e40672_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign35410_e40684, assign35410_e40684_d_n0, assign35410_e40684_d_n2, assign35410_e40684_d_n4, assign35410_e40684_d_n5, assign35410_e40684_d_n6, assign35410_e40684_d_n7, assign35410_e40684_d_n8, assign35410_e40684_d_n9, assign35410_e40684_d_n10, assign35410_e40684_d_n11, assign35410_e40684_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 != 0.0)) {
        let assign35410_e40682: f64 = (locals.var_xp * locals.var_x2);
        (assign35410_e40682, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign35410_e40684;
        locals.var_xp_dn0 = assign35410_e40684_d_n0;
        locals.var_xp_dn2 = assign35410_e40684_d_n2;
        locals.var_xp_dn4 = assign35410_e40684_d_n4;
        locals.var_xp_dn5 = assign35410_e40684_d_n5;
        locals.var_xp_dn6 = assign35410_e40684_d_n6;
        locals.var_xp_dn7 = assign35410_e40684_d_n7;
        locals.var_xp_dn8 = assign35410_e40684_d_n8;
        locals.var_xp_dn9 = assign35410_e40684_d_n9;
        locals.var_xp_dn10 = assign35410_e40684_d_n10;
        locals.var_xp_dn11 = assign35410_e40684_d_n11;
        locals.var_xp_dn14 = assign35410_e40684_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign35420_e40696, assign35420_e40696_d_n0, assign35420_e40696_d_n2, assign35420_e40696_d_n4, assign35420_e40696_d_n5, assign35420_e40696_d_n6, assign35420_e40696_d_n7, assign35420_e40696_d_n8, assign35420_e40696_d_n9, assign35420_e40696_d_n10, assign35420_e40696_d_n11, assign35420_e40696_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 != 0.0)) {
        let assign35420_e40694: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign35420_e40694, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign35420_e40696;
        locals.var_xmp_dn0 = assign35420_e40696_d_n0;
        locals.var_xmp_dn2 = assign35420_e40696_d_n2;
        locals.var_xmp_dn4 = assign35420_e40696_d_n4;
        locals.var_xmp_dn5 = assign35420_e40696_d_n5;
        locals.var_xmp_dn6 = assign35420_e40696_d_n6;
        locals.var_xmp_dn7 = assign35420_e40696_d_n7;
        locals.var_xmp_dn8 = assign35420_e40696_d_n8;
        locals.var_xmp_dn9 = assign35420_e40696_d_n9;
        locals.var_xmp_dn10 = assign35420_e40696_d_n10;
        locals.var_xmp_dn11 = assign35420_e40696_d_n11;
        locals.var_xmp_dn14 = assign35420_e40696_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign35430_e40708, assign35430_e40708_d_n0, assign35430_e40708_d_n2, assign35430_e40708_d_n4, assign35430_e40708_d_n5, assign35430_e40708_d_n6, assign35430_e40708_d_n7, assign35430_e40708_d_n8, assign35430_e40708_d_n9, assign35430_e40708_d_n10, assign35430_e40708_d_n11, assign35430_e40708_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 != 0.0)) {
        let assign35430_e40706: f64 = (locals.var_xp * locals.var_x2);
        (assign35430_e40706, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign35430_e40708;
        locals.var_xp_dn0 = assign35430_e40708_d_n0;
        locals.var_xp_dn2 = assign35430_e40708_d_n2;
        locals.var_xp_dn4 = assign35430_e40708_d_n4;
        locals.var_xp_dn5 = assign35430_e40708_d_n5;
        locals.var_xp_dn6 = assign35430_e40708_d_n6;
        locals.var_xp_dn7 = assign35430_e40708_d_n7;
        locals.var_xp_dn8 = assign35430_e40708_d_n8;
        locals.var_xp_dn9 = assign35430_e40708_d_n9;
        locals.var_xp_dn10 = assign35430_e40708_d_n10;
        locals.var_xp_dn11 = assign35430_e40708_d_n11;
        locals.var_xp_dn14 = assign35430_e40708_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign35440_e40720, assign35440_e40720_d_n0, assign35440_e40720_d_n2, assign35440_e40720_d_n4, assign35440_e40720_d_n5, assign35440_e40720_d_n6, assign35440_e40720_d_n7, assign35440_e40720_d_n8, assign35440_e40720_d_n9, assign35440_e40720_d_n10, assign35440_e40720_d_n11, assign35440_e40720_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 != 0.0)) {
        let assign35440_e40718: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign35440_e40718, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign35440_e40720;
        locals.var_xmp_dn0 = assign35440_e40720_d_n0;
        locals.var_xmp_dn2 = assign35440_e40720_d_n2;
        locals.var_xmp_dn4 = assign35440_e40720_d_n4;
        locals.var_xmp_dn5 = assign35440_e40720_d_n5;
        locals.var_xmp_dn6 = assign35440_e40720_d_n6;
        locals.var_xmp_dn7 = assign35440_e40720_d_n7;
        locals.var_xmp_dn8 = assign35440_e40720_d_n8;
        locals.var_xmp_dn9 = assign35440_e40720_d_n9;
        locals.var_xmp_dn10 = assign35440_e40720_d_n10;
        locals.var_xmp_dn11 = assign35440_e40720_d_n11;
        locals.var_xmp_dn14 = assign35440_e40720_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign35450_e40732, assign35450_e40732_d_n0, assign35450_e40732_d_n2, assign35450_e40732_d_n4, assign35450_e40732_d_n5, assign35450_e40732_d_n6, assign35450_e40732_d_n7, assign35450_e40732_d_n8, assign35450_e40732_d_n9, assign35450_e40732_d_n10, assign35450_e40732_d_n11, assign35450_e40732_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 != 0.0)) {
        let assign35450_e40730: f64 = (locals.var_xp * locals.var_x2);
        (assign35450_e40730, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign35450_e40732;
        locals.var_xp_dn0 = assign35450_e40732_d_n0;
        locals.var_xp_dn2 = assign35450_e40732_d_n2;
        locals.var_xp_dn4 = assign35450_e40732_d_n4;
        locals.var_xp_dn5 = assign35450_e40732_d_n5;
        locals.var_xp_dn6 = assign35450_e40732_d_n6;
        locals.var_xp_dn7 = assign35450_e40732_d_n7;
        locals.var_xp_dn8 = assign35450_e40732_d_n8;
        locals.var_xp_dn9 = assign35450_e40732_d_n9;
        locals.var_xp_dn10 = assign35450_e40732_d_n10;
        locals.var_xp_dn11 = assign35450_e40732_d_n11;
        locals.var_xp_dn14 = assign35450_e40732_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign35460_e40744, assign35460_e40744_d_n0, assign35460_e40744_d_n2, assign35460_e40744_d_n4, assign35460_e40744_d_n5, assign35460_e40744_d_n6, assign35460_e40744_d_n7, assign35460_e40744_d_n8, assign35460_e40744_d_n9, assign35460_e40744_d_n10, assign35460_e40744_d_n11, assign35460_e40744_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 != 0.0)) {
        let assign35460_e40742: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign35460_e40742, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign35460_e40744;
        locals.var_xmp_dn0 = assign35460_e40744_d_n0;
        locals.var_xmp_dn2 = assign35460_e40744_d_n2;
        locals.var_xmp_dn4 = assign35460_e40744_d_n4;
        locals.var_xmp_dn5 = assign35460_e40744_d_n5;
        locals.var_xmp_dn6 = assign35460_e40744_d_n6;
        locals.var_xmp_dn7 = assign35460_e40744_d_n7;
        locals.var_xmp_dn8 = assign35460_e40744_d_n8;
        locals.var_xmp_dn9 = assign35460_e40744_d_n9;
        locals.var_xmp_dn10 = assign35460_e40744_d_n10;
        locals.var_xmp_dn11 = assign35460_e40744_d_n11;
        locals.var_xmp_dn14 = assign35460_e40744_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign35470_e40756, assign35470_e40756_d_n0, assign35470_e40756_d_n2, assign35470_e40756_d_n4, assign35470_e40756_d_n5, assign35470_e40756_d_n6, assign35470_e40756_d_n7, assign35470_e40756_d_n8, assign35470_e40756_d_n9, assign35470_e40756_d_n10, assign35470_e40756_d_n11, assign35470_e40756_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 != 0.0)) {
        let assign35470_e40754: f64 = (locals.var_xp * locals.var_x2);
        (assign35470_e40754, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign35470_e40756;
        locals.var_xp_dn0 = assign35470_e40756_d_n0;
        locals.var_xp_dn2 = assign35470_e40756_d_n2;
        locals.var_xp_dn4 = assign35470_e40756_d_n4;
        locals.var_xp_dn5 = assign35470_e40756_d_n5;
        locals.var_xp_dn6 = assign35470_e40756_d_n6;
        locals.var_xp_dn7 = assign35470_e40756_d_n7;
        locals.var_xp_dn8 = assign35470_e40756_d_n8;
        locals.var_xp_dn9 = assign35470_e40756_d_n9;
        locals.var_xp_dn10 = assign35470_e40756_d_n10;
        locals.var_xp_dn11 = assign35470_e40756_d_n11;
        locals.var_xp_dn14 = assign35470_e40756_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign35480_e40768, assign35480_e40768_d_n0, assign35480_e40768_d_n2, assign35480_e40768_d_n4, assign35480_e40768_d_n5, assign35480_e40768_d_n6, assign35480_e40768_d_n7, assign35480_e40768_d_n8, assign35480_e40768_d_n9, assign35480_e40768_d_n10, assign35480_e40768_d_n11, assign35480_e40768_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 != 0.0)) {
        let assign35480_e40766: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign35480_e40766, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign35480_e40768;
        locals.var_xmp_dn0 = assign35480_e40768_d_n0;
        locals.var_xmp_dn2 = assign35480_e40768_d_n2;
        locals.var_xmp_dn4 = assign35480_e40768_d_n4;
        locals.var_xmp_dn5 = assign35480_e40768_d_n5;
        locals.var_xmp_dn6 = assign35480_e40768_d_n6;
        locals.var_xmp_dn7 = assign35480_e40768_d_n7;
        locals.var_xmp_dn8 = assign35480_e40768_d_n8;
        locals.var_xmp_dn9 = assign35480_e40768_d_n9;
        locals.var_xmp_dn10 = assign35480_e40768_d_n10;
        locals.var_xmp_dn11 = assign35480_e40768_d_n11;
        locals.var_xmp_dn14 = assign35480_e40768_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign35490_e40780, assign35490_e40780_d_n0, assign35490_e40780_d_n2, assign35490_e40780_d_n4, assign35490_e40780_d_n5, assign35490_e40780_d_n6, assign35490_e40780_d_n7, assign35490_e40780_d_n8, assign35490_e40780_d_n9, assign35490_e40780_d_n10, assign35490_e40780_d_n11, assign35490_e40780_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 != 0.0)) {
        let assign35490_e40778: f64 = (locals.var_xp + locals.var_xmp);
        (assign35490_e40778, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign35490_e40780;
        locals.var_arg_dn0 = assign35490_e40780_d_n0;
        locals.var_arg_dn2 = assign35490_e40780_d_n2;
        locals.var_arg_dn4 = assign35490_e40780_d_n4;
        locals.var_arg_dn5 = assign35490_e40780_d_n5;
        locals.var_arg_dn6 = assign35490_e40780_d_n6;
        locals.var_arg_dn7 = assign35490_e40780_d_n7;
        locals.var_arg_dn8 = assign35490_e40780_d_n8;
        locals.var_arg_dn9 = assign35490_e40780_d_n9;
        locals.var_arg_dn10 = assign35490_e40780_d_n10;
        locals.var_arg_dn11 = assign35490_e40780_d_n11;
        locals.var_arg_dn14 = assign35490_e40780_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign35500_e40790, assign35500_e40790_d_n0, assign35500_e40790_d_n2, assign35500_e40790_d_n4, assign35500_e40790_d_n5, assign35500_e40790_d_n6, assign35500_e40790_d_n7, assign35500_e40790_d_n8, assign35500_e40790_d_n9, assign35500_e40790_d_n10, assign35500_e40790_d_n11, assign35500_e40790_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign35500_e40790;
        locals.var_dnm_dn0 = assign35500_e40790_d_n0;
        locals.var_dnm_dn2 = assign35500_e40790_d_n2;
        locals.var_dnm_dn4 = assign35500_e40790_d_n4;
        locals.var_dnm_dn5 = assign35500_e40790_d_n5;
        locals.var_dnm_dn6 = assign35500_e40790_d_n6;
        locals.var_dnm_dn7 = assign35500_e40790_d_n7;
        locals.var_dnm_dn8 = assign35500_e40790_d_n8;
        locals.var_dnm_dn9 = assign35500_e40790_d_n9;
        locals.var_dnm_dn10 = assign35500_e40790_d_n10;
        locals.var_dnm_dn11 = assign35500_e40790_d_n11;
        locals.var_dnm_dn14 = assign35500_e40790_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign35510_e40805: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard820 = assign35510_e40805;
        locals.var_guard820_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_116(
        locals: &mut StampLocals,
    ) {
        let assign35520_e40808: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard821 = assign35520_e40808;
        locals.var_guard821_rv = 0.0;

        let (assign35530_e40822,) = {
    if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 != 0.0)) && (locals.var_guard820 != 0.0)) && (locals.var_guard821 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign35530_e40822;
        locals.var_mm_rv = 0.0;

        let assign35540_e40825: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard822 = assign35540_e40825;
        locals.var_guard822_rv = 0.0;

        let (assign35550_e40842,) = {
    if (((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 != 0.0)) && (locals.var_guard820 != 0.0)) && (locals.var_guard821 == 0.0)) && (locals.var_guard822 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign35550_e40842;
        locals.var_mm_rv = 0.0;

        let assign35560_e40845: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard823 = assign35560_e40845;
        locals.var_guard823_rv = 0.0;

        let (assign35570_e40865,) = {
    if ((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 != 0.0)) && (locals.var_guard820 != 0.0)) && (locals.var_guard821 == 0.0)) && (locals.var_guard822 == 0.0)) && (locals.var_guard823 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign35570_e40865;
        locals.var_mm_rv = 0.0;

        let assign35580_e40868: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard824 = assign35580_e40868;
        locals.var_guard824_rv = 0.0;

        let (assign35590_e40891,) = {
    if (((((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 != 0.0)) && (locals.var_guard820 != 0.0)) && (locals.var_guard821 == 0.0)) && (locals.var_guard822 == 0.0)) && (locals.var_guard823 == 0.0)) && (locals.var_guard824 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign35590_e40891;
        locals.var_mm_rv = 0.0;

        let (assign35600_e40903,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 != 0.0)) && (locals.var_guard820 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign35600_e40903;
        locals.var_m0_rv = 0.0;

        let mut assign35610_loop_guard: usize = 0;
        while {
            let assign35610_cond_e40916: f64 = if ((((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 != 0.0)) && (locals.var_guard820 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign35610_cond_e40916 != 0.0
        } {
            assign35610_loop_guard += 1;
            assert!(assign35610_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign35610_body0_e40929, assign35610_body0_e40929_d_n0, assign35610_body0_e40929_d_n2, assign35610_body0_e40929_d_n4, assign35610_body0_e40929_d_n5, assign35610_body0_e40929_d_n6, assign35610_body0_e40929_d_n7, assign35610_body0_e40929_d_n8, assign35610_body0_e40929_d_n9, assign35610_body0_e40929_d_n10, assign35610_body0_e40929_d_n11, assign35610_body0_e40929_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 != 0.0)) && (locals.var_guard820 != 0.0)) {
        let assign35610_body0_e40927: f64 = (locals.var_dnm).sqrt();
        (assign35610_body0_e40927, (locals.var_dnm_dn0 / (2.0 * assign35610_body0_e40927)), (locals.var_dnm_dn2 / (2.0 * assign35610_body0_e40927)), (locals.var_dnm_dn4 / (2.0 * assign35610_body0_e40927)), (locals.var_dnm_dn5 / (2.0 * assign35610_body0_e40927)), (locals.var_dnm_dn6 / (2.0 * assign35610_body0_e40927)), (locals.var_dnm_dn7 / (2.0 * assign35610_body0_e40927)), (locals.var_dnm_dn8 / (2.0 * assign35610_body0_e40927)), (locals.var_dnm_dn9 / (2.0 * assign35610_body0_e40927)), (locals.var_dnm_dn10 / (2.0 * assign35610_body0_e40927)), (locals.var_dnm_dn11 / (2.0 * assign35610_body0_e40927)), (locals.var_dnm_dn14 / (2.0 * assign35610_body0_e40927)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign35610_body0_e40929;
            locals.var_dnm_dn0 = assign35610_body0_e40929_d_n0;
            locals.var_dnm_dn2 = assign35610_body0_e40929_d_n2;
            locals.var_dnm_dn4 = assign35610_body0_e40929_d_n4;
            locals.var_dnm_dn5 = assign35610_body0_e40929_d_n5;
            locals.var_dnm_dn6 = assign35610_body0_e40929_d_n6;
            locals.var_dnm_dn7 = assign35610_body0_e40929_d_n7;
            locals.var_dnm_dn8 = assign35610_body0_e40929_d_n8;
            locals.var_dnm_dn9 = assign35610_body0_e40929_d_n9;
            locals.var_dnm_dn10 = assign35610_body0_e40929_d_n10;
            locals.var_dnm_dn11 = assign35610_body0_e40929_d_n11;
            locals.var_dnm_dn14 = assign35610_body0_e40929_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign35610_body1_e40943,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 != 0.0)) && (locals.var_guard820 != 0.0)) {
        let assign35610_body1_e40941: f64 = (locals.var_m0 + 1.0);
        (assign35610_body1_e40941,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign35610_body1_e40943;
            locals.var_m0_rv = 0.0;
        }

        let (assign35620_e40967, assign35620_e40967_d_n0, assign35620_e40967_d_n2, assign35620_e40967_d_n4, assign35620_e40967_d_n5, assign35620_e40967_d_n6, assign35620_e40967_d_n7, assign35620_e40967_d_n8, assign35620_e40967_d_n9, assign35620_e40967_d_n10, assign35620_e40967_d_n11, assign35620_e40967_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 != 0.0)) && (locals.var_guard820 == 0.0)) {
        let (assign35620_e40965, assign35620_e40965_d_n0, assign35620_e40965_d_n2, assign35620_e40965_d_n4, assign35620_e40965_d_n5, assign35620_e40965_d_n6, assign35620_e40965_d_n7, assign35620_e40965_d_n8, assign35620_e40965_d_n9, assign35620_e40965_d_n10, assign35620_e40965_d_n11, assign35620_e40965_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign35620_e40962: f64 = (2.0 * 4.0);
                let assign35620_e40963: f64 = (1.0 / assign35620_e40962);
                let assign35620_e40964: f64 = (locals.var_dnm).powf(assign35620_e40963);
                (assign35620_e40964, if 0.0 == 0.0 && ((assign35620_e40963) as f64).is_finite() && ((assign35620_e40963) as f64).fract() == 0.0 { if assign35620_e40963 == 0.0 { 0.0 } else { (assign35620_e40963 * ((locals.var_dnm).powf(assign35620_e40963 - 1.0) * locals.var_dnm_dn0)) } } else { (assign35620_e40964 * (assign35620_e40963 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35620_e40963) as f64).is_finite() && ((assign35620_e40963) as f64).fract() == 0.0 { if assign35620_e40963 == 0.0 { 0.0 } else { (assign35620_e40963 * ((locals.var_dnm).powf(assign35620_e40963 - 1.0) * locals.var_dnm_dn2)) } } else { (assign35620_e40964 * (assign35620_e40963 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35620_e40963) as f64).is_finite() && ((assign35620_e40963) as f64).fract() == 0.0 { if assign35620_e40963 == 0.0 { 0.0 } else { (assign35620_e40963 * ((locals.var_dnm).powf(assign35620_e40963 - 1.0) * locals.var_dnm_dn4)) } } else { (assign35620_e40964 * (assign35620_e40963 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35620_e40963) as f64).is_finite() && ((assign35620_e40963) as f64).fract() == 0.0 { if assign35620_e40963 == 0.0 { 0.0 } else { (assign35620_e40963 * ((locals.var_dnm).powf(assign35620_e40963 - 1.0) * locals.var_dnm_dn5)) } } else { (assign35620_e40964 * (assign35620_e40963 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35620_e40963) as f64).is_finite() && ((assign35620_e40963) as f64).fract() == 0.0 { if assign35620_e40963 == 0.0 { 0.0 } else { (assign35620_e40963 * ((locals.var_dnm).powf(assign35620_e40963 - 1.0) * locals.var_dnm_dn6)) } } else { (assign35620_e40964 * (assign35620_e40963 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35620_e40963) as f64).is_finite() && ((assign35620_e40963) as f64).fract() == 0.0 { if assign35620_e40963 == 0.0 { 0.0 } else { (assign35620_e40963 * ((locals.var_dnm).powf(assign35620_e40963 - 1.0) * locals.var_dnm_dn7)) } } else { (assign35620_e40964 * (assign35620_e40963 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35620_e40963) as f64).is_finite() && ((assign35620_e40963) as f64).fract() == 0.0 { if assign35620_e40963 == 0.0 { 0.0 } else { (assign35620_e40963 * ((locals.var_dnm).powf(assign35620_e40963 - 1.0) * locals.var_dnm_dn8)) } } else { (assign35620_e40964 * (assign35620_e40963 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35620_e40963) as f64).is_finite() && ((assign35620_e40963) as f64).fract() == 0.0 { if assign35620_e40963 == 0.0 { 0.0 } else { (assign35620_e40963 * ((locals.var_dnm).powf(assign35620_e40963 - 1.0) * locals.var_dnm_dn9)) } } else { (assign35620_e40964 * (assign35620_e40963 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35620_e40963) as f64).is_finite() && ((assign35620_e40963) as f64).fract() == 0.0 { if assign35620_e40963 == 0.0 { 0.0 } else { (assign35620_e40963 * ((locals.var_dnm).powf(assign35620_e40963 - 1.0) * locals.var_dnm_dn10)) } } else { (assign35620_e40964 * (assign35620_e40963 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35620_e40963) as f64).is_finite() && ((assign35620_e40963) as f64).fract() == 0.0 { if assign35620_e40963 == 0.0 { 0.0 } else { (assign35620_e40963 * ((locals.var_dnm).powf(assign35620_e40963 - 1.0) * locals.var_dnm_dn11)) } } else { (assign35620_e40964 * (assign35620_e40963 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35620_e40963) as f64).is_finite() && ((assign35620_e40963) as f64).fract() == 0.0 { if assign35620_e40963 == 0.0 { 0.0 } else { (assign35620_e40963 * ((locals.var_dnm).powf(assign35620_e40963 - 1.0) * locals.var_dnm_dn14)) } } else { (assign35620_e40964 * (assign35620_e40963 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign35620_e40965, assign35620_e40965_d_n0, assign35620_e40965_d_n2, assign35620_e40965_d_n4, assign35620_e40965_d_n5, assign35620_e40965_d_n6, assign35620_e40965_d_n7, assign35620_e40965_d_n8, assign35620_e40965_d_n9, assign35620_e40965_d_n10, assign35620_e40965_d_n11, assign35620_e40965_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign35620_e40967;
        locals.var_dnm_dn0 = assign35620_e40967_d_n0;
        locals.var_dnm_dn2 = assign35620_e40967_d_n2;
        locals.var_dnm_dn4 = assign35620_e40967_d_n4;
        locals.var_dnm_dn5 = assign35620_e40967_d_n5;
        locals.var_dnm_dn6 = assign35620_e40967_d_n6;
        locals.var_dnm_dn7 = assign35620_e40967_d_n7;
        locals.var_dnm_dn8 = assign35620_e40967_d_n8;
        locals.var_dnm_dn9 = assign35620_e40967_d_n9;
        locals.var_dnm_dn10 = assign35620_e40967_d_n10;
        locals.var_dnm_dn11 = assign35620_e40967_d_n11;
        locals.var_dnm_dn14 = assign35620_e40967_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign35630_e40979, assign35630_e40979_d_n0, assign35630_e40979_d_n2, assign35630_e40979_d_n4, assign35630_e40979_d_n5, assign35630_e40979_d_n6, assign35630_e40979_d_n7, assign35630_e40979_d_n8, assign35630_e40979_d_n9, assign35630_e40979_d_n10, assign35630_e40979_d_n11, assign35630_e40979_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 != 0.0)) {
        let assign35630_e40977: f64 = (1.0 / locals.var_dnm);
        (assign35630_e40977, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign35630_e40979;
        locals.var_dnm_dn0 = assign35630_e40979_d_n0;
        locals.var_dnm_dn2 = assign35630_e40979_d_n2;
        locals.var_dnm_dn4 = assign35630_e40979_d_n4;
        locals.var_dnm_dn5 = assign35630_e40979_d_n5;
        locals.var_dnm_dn6 = assign35630_e40979_d_n6;
        locals.var_dnm_dn7 = assign35630_e40979_d_n7;
        locals.var_dnm_dn8 = assign35630_e40979_d_n8;
        locals.var_dnm_dn9 = assign35630_e40979_d_n9;
        locals.var_dnm_dn10 = assign35630_e40979_d_n10;
        locals.var_dnm_dn11 = assign35630_e40979_d_n11;
        locals.var_dnm_dn14 = assign35630_e40979_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign35640_e40993, assign35640_e40993_d_n0, assign35640_e40993_d_n2, assign35640_e40993_d_n4, assign35640_e40993_d_n5, assign35640_e40993_d_n6, assign35640_e40993_d_n7, assign35640_e40993_d_n8, assign35640_e40993_d_n9, assign35640_e40993_d_n10, assign35640_e40993_d_n11, assign35640_e40993_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 != 0.0)) {
        let assign35640_e40989: f64 = (locals.var_tmf1 * 4.0);
        let assign35640_e40991: f64 = (assign35640_e40989 * locals.var_dnm);
        (assign35640_e40991, (((locals.var_tmf1_dn0 * 4.0) * locals.var_dnm) + (assign35640_e40989 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 4.0) * locals.var_dnm) + (assign35640_e40989 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 4.0) * locals.var_dnm) + (assign35640_e40989 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 4.0) * locals.var_dnm) + (assign35640_e40989 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 4.0) * locals.var_dnm) + (assign35640_e40989 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 4.0) * locals.var_dnm) + (assign35640_e40989 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 4.0) * locals.var_dnm) + (assign35640_e40989 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 4.0) * locals.var_dnm) + (assign35640_e40989 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 4.0) * locals.var_dnm) + (assign35640_e40989 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 4.0) * locals.var_dnm) + (assign35640_e40989 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 4.0) * locals.var_dnm) + (assign35640_e40989 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign35640_e40993;
        locals.var_tmf0_dn0 = assign35640_e40993_d_n0;
        locals.var_tmf0_dn2 = assign35640_e40993_d_n2;
        locals.var_tmf0_dn4 = assign35640_e40993_d_n4;
        locals.var_tmf0_dn5 = assign35640_e40993_d_n5;
        locals.var_tmf0_dn6 = assign35640_e40993_d_n6;
        locals.var_tmf0_dn7 = assign35640_e40993_d_n7;
        locals.var_tmf0_dn8 = assign35640_e40993_d_n8;
        locals.var_tmf0_dn9 = assign35640_e40993_d_n9;
        locals.var_tmf0_dn10 = assign35640_e40993_d_n10;
        locals.var_tmf0_dn11 = assign35640_e40993_d_n11;
        locals.var_tmf0_dn14 = assign35640_e40993_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign35650_e41009, assign35650_e41009_d_n0, assign35650_e41009_d_n2, assign35650_e41009_d_n4, assign35650_e41009_d_n5, assign35650_e41009_d_n6, assign35650_e41009_d_n7, assign35650_e41009_d_n8, assign35650_e41009_d_n9, assign35650_e41009_d_n10, assign35650_e41009_d_n11, assign35650_e41009_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 != 0.0)) {
        let assign35650_e41003: f64 = (4.0 * locals.var_xmp);
        let assign35650_e41005: f64 = (assign35650_e41003 * locals.var_dnm);
        let assign35650_e41007: f64 = (assign35650_e41005 / locals.var_arg);
        (assign35650_e41007, ((((((4.0 * locals.var_xmp_dn0) * locals.var_dnm) + (assign35650_e41003 * locals.var_dnm_dn0)) * locals.var_arg) - (assign35650_e41005 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((4.0 * locals.var_xmp_dn2) * locals.var_dnm) + (assign35650_e41003 * locals.var_dnm_dn2)) * locals.var_arg) - (assign35650_e41005 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((4.0 * locals.var_xmp_dn4) * locals.var_dnm) + (assign35650_e41003 * locals.var_dnm_dn4)) * locals.var_arg) - (assign35650_e41005 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((4.0 * locals.var_xmp_dn5) * locals.var_dnm) + (assign35650_e41003 * locals.var_dnm_dn5)) * locals.var_arg) - (assign35650_e41005 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((4.0 * locals.var_xmp_dn6) * locals.var_dnm) + (assign35650_e41003 * locals.var_dnm_dn6)) * locals.var_arg) - (assign35650_e41005 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((4.0 * locals.var_xmp_dn7) * locals.var_dnm) + (assign35650_e41003 * locals.var_dnm_dn7)) * locals.var_arg) - (assign35650_e41005 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((4.0 * locals.var_xmp_dn8) * locals.var_dnm) + (assign35650_e41003 * locals.var_dnm_dn8)) * locals.var_arg) - (assign35650_e41005 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((4.0 * locals.var_xmp_dn9) * locals.var_dnm) + (assign35650_e41003 * locals.var_dnm_dn9)) * locals.var_arg) - (assign35650_e41005 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((4.0 * locals.var_xmp_dn10) * locals.var_dnm) + (assign35650_e41003 * locals.var_dnm_dn10)) * locals.var_arg) - (assign35650_e41005 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((4.0 * locals.var_xmp_dn11) * locals.var_dnm) + (assign35650_e41003 * locals.var_dnm_dn11)) * locals.var_arg) - (assign35650_e41005 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((4.0 * locals.var_xmp_dn14) * locals.var_dnm) + (assign35650_e41003 * locals.var_dnm_dn14)) * locals.var_arg) - (assign35650_e41005 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign35650_e41009;
        locals.var_t0_dn0 = assign35650_e41009_d_n0;
        locals.var_t0_dn2 = assign35650_e41009_d_n2;
        locals.var_t0_dn4 = assign35650_e41009_d_n4;
        locals.var_t0_dn5 = assign35650_e41009_d_n5;
        locals.var_t0_dn6 = assign35650_e41009_d_n6;
        locals.var_t0_dn7 = assign35650_e41009_d_n7;
        locals.var_t0_dn8 = assign35650_e41009_d_n8;
        locals.var_t0_dn9 = assign35650_e41009_d_n9;
        locals.var_t0_dn10 = assign35650_e41009_d_n10;
        locals.var_t0_dn11 = assign35650_e41009_d_n11;
        locals.var_t0_dn14 = assign35650_e41009_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign35660_e41023, assign35660_e41023_d_n0, assign35660_e41023_d_n2, assign35660_e41023_d_n4, assign35660_e41023_d_n5, assign35660_e41023_d_n6, assign35660_e41023_d_n7, assign35660_e41023_d_n8, assign35660_e41023_d_n9, assign35660_e41023_d_n10, assign35660_e41023_d_n11, assign35660_e41023_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 != 0.0)) {
        let assign35660_e41019: f64 = (locals.var_uc_depleak + 4.0);
        let assign35660_e41021: f64 = (assign35660_e41019 - locals.var_tmf0);
        (assign35660_e41021, (locals.var_uc_depleak_dn0 - locals.var_tmf0_dn0), (locals.var_uc_depleak_dn2 - locals.var_tmf0_dn2), (locals.var_uc_depleak_dn4 - locals.var_tmf0_dn4), (locals.var_uc_depleak_dn5 - locals.var_tmf0_dn5), (locals.var_uc_depleak_dn6 - locals.var_tmf0_dn6), (locals.var_uc_depleak_dn7 - locals.var_tmf0_dn7), (locals.var_uc_depleak_dn8 - locals.var_tmf0_dn8), (locals.var_uc_depleak_dn9 - locals.var_tmf0_dn9), (locals.var_uc_depleak_dn10 - locals.var_tmf0_dn10), (locals.var_uc_depleak_dn11 - locals.var_tmf0_dn11), (locals.var_uc_depleak_dn14 - locals.var_tmf0_dn14),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign35660_e41023;
        locals.var_t10_dn0 = assign35660_e41023_d_n0;
        locals.var_t10_dn2 = assign35660_e41023_d_n2;
        locals.var_t10_dn4 = assign35660_e41023_d_n4;
        locals.var_t10_dn5 = assign35660_e41023_d_n5;
        locals.var_t10_dn6 = assign35660_e41023_d_n6;
        locals.var_t10_dn7 = assign35660_e41023_d_n7;
        locals.var_t10_dn8 = assign35660_e41023_d_n8;
        locals.var_t10_dn9 = assign35660_e41023_d_n9;
        locals.var_t10_dn10 = assign35660_e41023_d_n10;
        locals.var_t10_dn11 = assign35660_e41023_d_n11;
        locals.var_t10_dn14 = assign35660_e41023_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign35670_e41033, assign35670_e41033_d_n0, assign35670_e41033_d_n2, assign35670_e41033_d_n4, assign35670_e41033_d_n5, assign35670_e41033_d_n6, assign35670_e41033_d_n7, assign35670_e41033_d_n8, assign35670_e41033_d_n9, assign35670_e41033_d_n10, assign35670_e41033_d_n11, assign35670_e41033_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign35670_e41033;
        locals.var_t0_dn0 = assign35670_e41033_d_n0;
        locals.var_t0_dn2 = assign35670_e41033_d_n2;
        locals.var_t0_dn4 = assign35670_e41033_d_n4;
        locals.var_t0_dn5 = assign35670_e41033_d_n5;
        locals.var_t0_dn6 = assign35670_e41033_d_n6;
        locals.var_t0_dn7 = assign35670_e41033_d_n7;
        locals.var_t0_dn8 = assign35670_e41033_d_n8;
        locals.var_t0_dn9 = assign35670_e41033_d_n9;
        locals.var_t0_dn10 = assign35670_e41033_d_n10;
        locals.var_t0_dn11 = assign35670_e41033_d_n11;
        locals.var_t0_dn14 = assign35670_e41033_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign35680_e41044, assign35680_e41044_d_n0, assign35680_e41044_d_n2, assign35680_e41044_d_n4, assign35680_e41044_d_n5, assign35680_e41044_d_n6, assign35680_e41044_d_n7, assign35680_e41044_d_n8, assign35680_e41044_d_n9, assign35680_e41044_d_n10, assign35680_e41044_d_n11, assign35680_e41044_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 == 0.0)) {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign35680_e41044;
        locals.var_t10_dn0 = assign35680_e41044_d_n0;
        locals.var_t10_dn2 = assign35680_e41044_d_n2;
        locals.var_t10_dn4 = assign35680_e41044_d_n4;
        locals.var_t10_dn5 = assign35680_e41044_d_n5;
        locals.var_t10_dn6 = assign35680_e41044_d_n6;
        locals.var_t10_dn7 = assign35680_e41044_d_n7;
        locals.var_t10_dn8 = assign35680_e41044_d_n8;
        locals.var_t10_dn9 = assign35680_e41044_d_n9;
        locals.var_t10_dn10 = assign35680_e41044_d_n10;
        locals.var_t10_dn11 = assign35680_e41044_d_n11;
        locals.var_t10_dn14 = assign35680_e41044_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign35690_e41055, assign35690_e41055_d_n0, assign35690_e41055_d_n2, assign35690_e41055_d_n4, assign35690_e41055_d_n5, assign35690_e41055_d_n6, assign35690_e41055_d_n7, assign35690_e41055_d_n8, assign35690_e41055_d_n9, assign35690_e41055_d_n10, assign35690_e41055_d_n11, assign35690_e41055_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard819 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign35690_e41055;
        locals.var_t0_dn0 = assign35690_e41055_d_n0;
        locals.var_t0_dn2 = assign35690_e41055_d_n2;
        locals.var_t0_dn4 = assign35690_e41055_d_n4;
        locals.var_t0_dn5 = assign35690_e41055_d_n5;
        locals.var_t0_dn6 = assign35690_e41055_d_n6;
        locals.var_t0_dn7 = assign35690_e41055_d_n7;
        locals.var_t0_dn8 = assign35690_e41055_d_n8;
        locals.var_t0_dn9 = assign35690_e41055_d_n9;
        locals.var_t0_dn10 = assign35690_e41055_d_n10;
        locals.var_t0_dn11 = assign35690_e41055_d_n11;
        locals.var_t0_dn14 = assign35690_e41055_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign35700_e41065, assign35700_e41065_d_n0, assign35700_e41065_d_n2, assign35700_e41065_d_n4, assign35700_e41065_d_n5, assign35700_e41065_d_n6, assign35700_e41065_d_n7, assign35700_e41065_d_n8, assign35700_e41065_d_n9, assign35700_e41065_d_n10, assign35700_e41065_d_n11, assign35700_e41065_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) {
        let assign35700_e41063: f64 = (locals.var_vdsorg / locals.var_t10);
        (assign35700_e41063, (((locals.var_vdsorg_dn0 * locals.var_t10) - (locals.var_vdsorg * locals.var_t10_dn0)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vdsorg_dn2 * locals.var_t10) - (locals.var_vdsorg * locals.var_t10_dn2)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vdsorg_dn4 * locals.var_t10) - (locals.var_vdsorg * locals.var_t10_dn4)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vdsorg_dn5 * locals.var_t10) - (locals.var_vdsorg * locals.var_t10_dn5)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vdsorg_dn6 * locals.var_t10) - (locals.var_vdsorg * locals.var_t10_dn6)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vdsorg_dn7 * locals.var_t10) - (locals.var_vdsorg * locals.var_t10_dn7)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vdsorg_dn8 * locals.var_t10) - (locals.var_vdsorg * locals.var_t10_dn8)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vdsorg_dn9 * locals.var_t10) - (locals.var_vdsorg * locals.var_t10_dn9)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vdsorg_dn10 * locals.var_t10) - (locals.var_vdsorg * locals.var_t10_dn10)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vdsorg_dn11 * locals.var_t10) - (locals.var_vdsorg * locals.var_t10_dn11)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vdsorg_dn14 * locals.var_t10) - (locals.var_vdsorg * locals.var_t10_dn14)) / (locals.var_t10 * locals.var_t10)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign35700_e41065;
        locals.var_t1_dn0 = assign35700_e41065_d_n0;
        locals.var_t1_dn2 = assign35700_e41065_d_n2;
        locals.var_t1_dn4 = assign35700_e41065_d_n4;
        locals.var_t1_dn5 = assign35700_e41065_d_n5;
        locals.var_t1_dn6 = assign35700_e41065_d_n6;
        locals.var_t1_dn7 = assign35700_e41065_d_n7;
        locals.var_t1_dn8 = assign35700_e41065_d_n8;
        locals.var_t1_dn9 = assign35700_e41065_d_n9;
        locals.var_t1_dn10 = assign35700_e41065_d_n10;
        locals.var_t1_dn11 = assign35700_e41065_d_n11;
        locals.var_t1_dn14 = assign35700_e41065_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign35710_e41082, assign35710_e41082_d_n0, assign35710_e41082_d_n2, assign35710_e41082_d_n4, assign35710_e41082_d_n5, assign35710_e41082_d_n6, assign35710_e41082_d_n7, assign35710_e41082_d_n8, assign35710_e41082_d_n9, assign35710_e41082_d_n10, assign35710_e41082_d_n11, assign35710_e41082_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) {
        let (assign35710_e41080, assign35710_e41080_d_n0, assign35710_e41080_d_n2, assign35710_e41080_d_n4, assign35710_e41080_d_n5, assign35710_e41080_d_n6, assign35710_e41080_d_n7, assign35710_e41080_d_n8, assign35710_e41080_d_n9, assign35710_e41080_d_n10, assign35710_e41080_d_n11, assign35710_e41080_d_n14,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign35710_e41078: f64 = (locals.var_ddlte - 1.0);
                let assign35710_e41079: f64 = (locals.var_t1).powf(assign35710_e41078);
                (assign35710_e41079, if locals.var_ddlte_dn0 == 0.0 && ((assign35710_e41078) as f64).is_finite() && ((assign35710_e41078) as f64).fract() == 0.0 { if assign35710_e41078 == 0.0 { 0.0 } else { (assign35710_e41078 * ((locals.var_t1).powf(assign35710_e41078 - 1.0) * locals.var_t1_dn0)) } } else { (assign35710_e41079 * ((locals.var_ddlte_dn0 * (locals.var_t1).ln()) + (assign35710_e41078 * (locals.var_t1_dn0 / locals.var_t1)))) }, if locals.var_ddlte_dn2 == 0.0 && ((assign35710_e41078) as f64).is_finite() && ((assign35710_e41078) as f64).fract() == 0.0 { if assign35710_e41078 == 0.0 { 0.0 } else { (assign35710_e41078 * ((locals.var_t1).powf(assign35710_e41078 - 1.0) * locals.var_t1_dn2)) } } else { (assign35710_e41079 * ((locals.var_ddlte_dn2 * (locals.var_t1).ln()) + (assign35710_e41078 * (locals.var_t1_dn2 / locals.var_t1)))) }, if locals.var_ddlte_dn4 == 0.0 && ((assign35710_e41078) as f64).is_finite() && ((assign35710_e41078) as f64).fract() == 0.0 { if assign35710_e41078 == 0.0 { 0.0 } else { (assign35710_e41078 * ((locals.var_t1).powf(assign35710_e41078 - 1.0) * locals.var_t1_dn4)) } } else { (assign35710_e41079 * ((locals.var_ddlte_dn4 * (locals.var_t1).ln()) + (assign35710_e41078 * (locals.var_t1_dn4 / locals.var_t1)))) }, if locals.var_ddlte_dn5 == 0.0 && ((assign35710_e41078) as f64).is_finite() && ((assign35710_e41078) as f64).fract() == 0.0 { if assign35710_e41078 == 0.0 { 0.0 } else { (assign35710_e41078 * ((locals.var_t1).powf(assign35710_e41078 - 1.0) * locals.var_t1_dn5)) } } else { (assign35710_e41079 * ((locals.var_ddlte_dn5 * (locals.var_t1).ln()) + (assign35710_e41078 * (locals.var_t1_dn5 / locals.var_t1)))) }, if locals.var_ddlte_dn6 == 0.0 && ((assign35710_e41078) as f64).is_finite() && ((assign35710_e41078) as f64).fract() == 0.0 { if assign35710_e41078 == 0.0 { 0.0 } else { (assign35710_e41078 * ((locals.var_t1).powf(assign35710_e41078 - 1.0) * locals.var_t1_dn6)) } } else { (assign35710_e41079 * ((locals.var_ddlte_dn6 * (locals.var_t1).ln()) + (assign35710_e41078 * (locals.var_t1_dn6 / locals.var_t1)))) }, if locals.var_ddlte_dn7 == 0.0 && ((assign35710_e41078) as f64).is_finite() && ((assign35710_e41078) as f64).fract() == 0.0 { if assign35710_e41078 == 0.0 { 0.0 } else { (assign35710_e41078 * ((locals.var_t1).powf(assign35710_e41078 - 1.0) * locals.var_t1_dn7)) } } else { (assign35710_e41079 * ((locals.var_ddlte_dn7 * (locals.var_t1).ln()) + (assign35710_e41078 * (locals.var_t1_dn7 / locals.var_t1)))) }, if locals.var_ddlte_dn8 == 0.0 && ((assign35710_e41078) as f64).is_finite() && ((assign35710_e41078) as f64).fract() == 0.0 { if assign35710_e41078 == 0.0 { 0.0 } else { (assign35710_e41078 * ((locals.var_t1).powf(assign35710_e41078 - 1.0) * locals.var_t1_dn8)) } } else { (assign35710_e41079 * ((locals.var_ddlte_dn8 * (locals.var_t1).ln()) + (assign35710_e41078 * (locals.var_t1_dn8 / locals.var_t1)))) }, if locals.var_ddlte_dn9 == 0.0 && ((assign35710_e41078) as f64).is_finite() && ((assign35710_e41078) as f64).fract() == 0.0 { if assign35710_e41078 == 0.0 { 0.0 } else { (assign35710_e41078 * ((locals.var_t1).powf(assign35710_e41078 - 1.0) * locals.var_t1_dn9)) } } else { (assign35710_e41079 * ((locals.var_ddlte_dn9 * (locals.var_t1).ln()) + (assign35710_e41078 * (locals.var_t1_dn9 / locals.var_t1)))) }, if locals.var_ddlte_dn10 == 0.0 && ((assign35710_e41078) as f64).is_finite() && ((assign35710_e41078) as f64).fract() == 0.0 { if assign35710_e41078 == 0.0 { 0.0 } else { (assign35710_e41078 * ((locals.var_t1).powf(assign35710_e41078 - 1.0) * locals.var_t1_dn10)) } } else { (assign35710_e41079 * ((locals.var_ddlte_dn10 * (locals.var_t1).ln()) + (assign35710_e41078 * (locals.var_t1_dn10 / locals.var_t1)))) }, if locals.var_ddlte_dn11 == 0.0 && ((assign35710_e41078) as f64).is_finite() && ((assign35710_e41078) as f64).fract() == 0.0 { if assign35710_e41078 == 0.0 { 0.0 } else { (assign35710_e41078 * ((locals.var_t1).powf(assign35710_e41078 - 1.0) * locals.var_t1_dn11)) } } else { (assign35710_e41079 * ((locals.var_ddlte_dn11 * (locals.var_t1).ln()) + (assign35710_e41078 * (locals.var_t1_dn11 / locals.var_t1)))) }, if locals.var_ddlte_dn14 == 0.0 && ((assign35710_e41078) as f64).is_finite() && ((assign35710_e41078) as f64).fract() == 0.0 { if assign35710_e41078 == 0.0 { 0.0 } else { (assign35710_e41078 * ((locals.var_t1).powf(assign35710_e41078 - 1.0) * locals.var_t1_dn14)) } } else { (assign35710_e41079 * ((locals.var_ddlte_dn14 * (locals.var_t1).ln()) + (assign35710_e41078 * (locals.var_t1_dn14 / locals.var_t1)))) },)
            }
        };
        (assign35710_e41080, assign35710_e41080_d_n0, assign35710_e41080_d_n2, assign35710_e41080_d_n4, assign35710_e41080_d_n5, assign35710_e41080_d_n6, assign35710_e41080_d_n7, assign35710_e41080_d_n8, assign35710_e41080_d_n9, assign35710_e41080_d_n10, assign35710_e41080_d_n11, assign35710_e41080_d_n14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign35710_e41082;
        locals.var_t2_dn0 = assign35710_e41082_d_n0;
        locals.var_t2_dn2 = assign35710_e41082_d_n2;
        locals.var_t2_dn4 = assign35710_e41082_d_n4;
        locals.var_t2_dn5 = assign35710_e41082_d_n5;
        locals.var_t2_dn6 = assign35710_e41082_d_n6;
        locals.var_t2_dn7 = assign35710_e41082_d_n7;
        locals.var_t2_dn8 = assign35710_e41082_d_n8;
        locals.var_t2_dn9 = assign35710_e41082_d_n9;
        locals.var_t2_dn10 = assign35710_e41082_d_n10;
        locals.var_t2_dn11 = assign35710_e41082_d_n11;
        locals.var_t2_dn14 = assign35710_e41082_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign35720_e41092, assign35720_e41092_d_n0, assign35720_e41092_d_n2, assign35720_e41092_d_n4, assign35720_e41092_d_n5, assign35720_e41092_d_n6, assign35720_e41092_d_n7, assign35720_e41092_d_n8, assign35720_e41092_d_n9, assign35720_e41092_d_n10, assign35720_e41092_d_n11, assign35720_e41092_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) {
        let assign35720_e41090: f64 = (locals.var_t2 * locals.var_t1);
        (assign35720_e41090, ((locals.var_t2_dn0 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn0)), ((locals.var_t2_dn2 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn2)), ((locals.var_t2_dn4 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn4)), ((locals.var_t2_dn5 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn5)), ((locals.var_t2_dn6 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn6)), ((locals.var_t2_dn7 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn7)), ((locals.var_t2_dn8 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn8)), ((locals.var_t2_dn9 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn9)), ((locals.var_t2_dn10 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn10)), ((locals.var_t2_dn11 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn11)), ((locals.var_t2_dn14 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign35720_e41092;
        locals.var_t7_dn0 = assign35720_e41092_d_n0;
        locals.var_t7_dn2 = assign35720_e41092_d_n2;
        locals.var_t7_dn4 = assign35720_e41092_d_n4;
        locals.var_t7_dn5 = assign35720_e41092_d_n5;
        locals.var_t7_dn6 = assign35720_e41092_d_n6;
        locals.var_t7_dn7 = assign35720_e41092_d_n7;
        locals.var_t7_dn8 = assign35720_e41092_d_n8;
        locals.var_t7_dn9 = assign35720_e41092_d_n9;
        locals.var_t7_dn10 = assign35720_e41092_d_n10;
        locals.var_t7_dn11 = assign35720_e41092_d_n11;
        locals.var_t7_dn14 = assign35720_e41092_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign35730_e41102, assign35730_e41102_d_n0, assign35730_e41102_d_n2, assign35730_e41102_d_n4, assign35730_e41102_d_n5, assign35730_e41102_d_n6, assign35730_e41102_d_n7, assign35730_e41102_d_n8, assign35730_e41102_d_n9, assign35730_e41102_d_n10, assign35730_e41102_d_n11, assign35730_e41102_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) {
        let assign35730_e41100: f64 = (1.0 + locals.var_t7);
        (assign35730_e41100, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign35730_e41102;
        locals.var_t3_dn0 = assign35730_e41102_d_n0;
        locals.var_t3_dn2 = assign35730_e41102_d_n2;
        locals.var_t3_dn4 = assign35730_e41102_d_n4;
        locals.var_t3_dn5 = assign35730_e41102_d_n5;
        locals.var_t3_dn6 = assign35730_e41102_d_n6;
        locals.var_t3_dn7 = assign35730_e41102_d_n7;
        locals.var_t3_dn8 = assign35730_e41102_d_n8;
        locals.var_t3_dn9 = assign35730_e41102_d_n9;
        locals.var_t3_dn10 = assign35730_e41102_d_n10;
        locals.var_t3_dn11 = assign35730_e41102_d_n11;
        locals.var_t3_dn14 = assign35730_e41102_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign35740_e41121, assign35740_e41121_d_n0, assign35740_e41121_d_n2, assign35740_e41121_d_n4, assign35740_e41121_d_n5, assign35740_e41121_d_n6, assign35740_e41121_d_n7, assign35740_e41121_d_n8, assign35740_e41121_d_n9, assign35740_e41121_d_n10, assign35740_e41121_d_n11, assign35740_e41121_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) {
        let (assign35740_e41119, assign35740_e41119_d_n0, assign35740_e41119_d_n2, assign35740_e41119_d_n4, assign35740_e41119_d_n5, assign35740_e41119_d_n6, assign35740_e41119_d_n7, assign35740_e41119_d_n8, assign35740_e41119_d_n9, assign35740_e41119_d_n10, assign35740_e41119_d_n11, assign35740_e41119_d_n14,) = {
            if (locals.var_t3 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign35740_e41115: f64 = (1.0 / locals.var_ddlte);
                let assign35740_e41117: f64 = (assign35740_e41115 - 1.0);
                let assign35740_e41118: f64 = (locals.var_t3).powf(assign35740_e41117);
                (assign35740_e41118, if (-(locals.var_ddlte_dn0 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign35740_e41117) as f64).is_finite() && ((assign35740_e41117) as f64).fract() == 0.0 { if assign35740_e41117 == 0.0 { 0.0 } else { (assign35740_e41117 * ((locals.var_t3).powf(assign35740_e41117 - 1.0) * locals.var_t3_dn0)) } } else { (assign35740_e41118 * (((-(locals.var_ddlte_dn0 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign35740_e41117 * (locals.var_t3_dn0 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn2 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign35740_e41117) as f64).is_finite() && ((assign35740_e41117) as f64).fract() == 0.0 { if assign35740_e41117 == 0.0 { 0.0 } else { (assign35740_e41117 * ((locals.var_t3).powf(assign35740_e41117 - 1.0) * locals.var_t3_dn2)) } } else { (assign35740_e41118 * (((-(locals.var_ddlte_dn2 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign35740_e41117 * (locals.var_t3_dn2 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn4 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign35740_e41117) as f64).is_finite() && ((assign35740_e41117) as f64).fract() == 0.0 { if assign35740_e41117 == 0.0 { 0.0 } else { (assign35740_e41117 * ((locals.var_t3).powf(assign35740_e41117 - 1.0) * locals.var_t3_dn4)) } } else { (assign35740_e41118 * (((-(locals.var_ddlte_dn4 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign35740_e41117 * (locals.var_t3_dn4 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn5 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign35740_e41117) as f64).is_finite() && ((assign35740_e41117) as f64).fract() == 0.0 { if assign35740_e41117 == 0.0 { 0.0 } else { (assign35740_e41117 * ((locals.var_t3).powf(assign35740_e41117 - 1.0) * locals.var_t3_dn5)) } } else { (assign35740_e41118 * (((-(locals.var_ddlte_dn5 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign35740_e41117 * (locals.var_t3_dn5 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn6 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign35740_e41117) as f64).is_finite() && ((assign35740_e41117) as f64).fract() == 0.0 { if assign35740_e41117 == 0.0 { 0.0 } else { (assign35740_e41117 * ((locals.var_t3).powf(assign35740_e41117 - 1.0) * locals.var_t3_dn6)) } } else { (assign35740_e41118 * (((-(locals.var_ddlte_dn6 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign35740_e41117 * (locals.var_t3_dn6 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn7 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign35740_e41117) as f64).is_finite() && ((assign35740_e41117) as f64).fract() == 0.0 { if assign35740_e41117 == 0.0 { 0.0 } else { (assign35740_e41117 * ((locals.var_t3).powf(assign35740_e41117 - 1.0) * locals.var_t3_dn7)) } } else { (assign35740_e41118 * (((-(locals.var_ddlte_dn7 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign35740_e41117 * (locals.var_t3_dn7 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn8 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign35740_e41117) as f64).is_finite() && ((assign35740_e41117) as f64).fract() == 0.0 { if assign35740_e41117 == 0.0 { 0.0 } else { (assign35740_e41117 * ((locals.var_t3).powf(assign35740_e41117 - 1.0) * locals.var_t3_dn8)) } } else { (assign35740_e41118 * (((-(locals.var_ddlte_dn8 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign35740_e41117 * (locals.var_t3_dn8 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn9 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign35740_e41117) as f64).is_finite() && ((assign35740_e41117) as f64).fract() == 0.0 { if assign35740_e41117 == 0.0 { 0.0 } else { (assign35740_e41117 * ((locals.var_t3).powf(assign35740_e41117 - 1.0) * locals.var_t3_dn9)) } } else { (assign35740_e41118 * (((-(locals.var_ddlte_dn9 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign35740_e41117 * (locals.var_t3_dn9 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn10 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign35740_e41117) as f64).is_finite() && ((assign35740_e41117) as f64).fract() == 0.0 { if assign35740_e41117 == 0.0 { 0.0 } else { (assign35740_e41117 * ((locals.var_t3).powf(assign35740_e41117 - 1.0) * locals.var_t3_dn10)) } } else { (assign35740_e41118 * (((-(locals.var_ddlte_dn10 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign35740_e41117 * (locals.var_t3_dn10 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn11 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign35740_e41117) as f64).is_finite() && ((assign35740_e41117) as f64).fract() == 0.0 { if assign35740_e41117 == 0.0 { 0.0 } else { (assign35740_e41117 * ((locals.var_t3).powf(assign35740_e41117 - 1.0) * locals.var_t3_dn11)) } } else { (assign35740_e41118 * (((-(locals.var_ddlte_dn11 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign35740_e41117 * (locals.var_t3_dn11 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn14 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign35740_e41117) as f64).is_finite() && ((assign35740_e41117) as f64).fract() == 0.0 { if assign35740_e41117 == 0.0 { 0.0 } else { (assign35740_e41117 * ((locals.var_t3).powf(assign35740_e41117 - 1.0) * locals.var_t3_dn14)) } } else { (assign35740_e41118 * (((-(locals.var_ddlte_dn14 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign35740_e41117 * (locals.var_t3_dn14 / locals.var_t3)))) },)
            }
        };
        (assign35740_e41119, assign35740_e41119_d_n0, assign35740_e41119_d_n2, assign35740_e41119_d_n4, assign35740_e41119_d_n5, assign35740_e41119_d_n6, assign35740_e41119_d_n7, assign35740_e41119_d_n8, assign35740_e41119_d_n9, assign35740_e41119_d_n10, assign35740_e41119_d_n11, assign35740_e41119_d_n14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign35740_e41121;
        locals.var_t4_dn0 = assign35740_e41121_d_n0;
        locals.var_t4_dn2 = assign35740_e41121_d_n2;
        locals.var_t4_dn4 = assign35740_e41121_d_n4;
        locals.var_t4_dn5 = assign35740_e41121_d_n5;
        locals.var_t4_dn6 = assign35740_e41121_d_n6;
        locals.var_t4_dn7 = assign35740_e41121_d_n7;
        locals.var_t4_dn8 = assign35740_e41121_d_n8;
        locals.var_t4_dn9 = assign35740_e41121_d_n9;
        locals.var_t4_dn10 = assign35740_e41121_d_n10;
        locals.var_t4_dn11 = assign35740_e41121_d_n11;
        locals.var_t4_dn14 = assign35740_e41121_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign35750_e41131, assign35750_e41131_d_n0, assign35750_e41131_d_n2, assign35750_e41131_d_n4, assign35750_e41131_d_n5, assign35750_e41131_d_n6, assign35750_e41131_d_n7, assign35750_e41131_d_n8, assign35750_e41131_d_n9, assign35750_e41131_d_n10, assign35750_e41131_d_n11, assign35750_e41131_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) {
        let assign35750_e41129: f64 = (locals.var_t4 * locals.var_t3);
        (assign35750_e41129, ((locals.var_t4_dn0 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn0)), ((locals.var_t4_dn2 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn2)), ((locals.var_t4_dn4 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn4)), ((locals.var_t4_dn5 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn5)), ((locals.var_t4_dn6 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn6)), ((locals.var_t4_dn7 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn7)), ((locals.var_t4_dn8 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn8)), ((locals.var_t4_dn9 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn9)), ((locals.var_t4_dn10 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn10)), ((locals.var_t4_dn11 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn11)), ((locals.var_t4_dn14 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign35750_e41131;
        locals.var_t6_dn0 = assign35750_e41131_d_n0;
        locals.var_t6_dn2 = assign35750_e41131_d_n2;
        locals.var_t6_dn4 = assign35750_e41131_d_n4;
        locals.var_t6_dn5 = assign35750_e41131_d_n5;
        locals.var_t6_dn6 = assign35750_e41131_d_n6;
        locals.var_t6_dn7 = assign35750_e41131_d_n7;
        locals.var_t6_dn8 = assign35750_e41131_d_n8;
        locals.var_t6_dn9 = assign35750_e41131_d_n9;
        locals.var_t6_dn10 = assign35750_e41131_d_n10;
        locals.var_t6_dn11 = assign35750_e41131_d_n11;
        locals.var_t6_dn14 = assign35750_e41131_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign35760_e41141, assign35760_e41141_d_n0, assign35760_e41141_d_n2, assign35760_e41141_d_n4, assign35760_e41141_d_n5, assign35760_e41141_d_n6, assign35760_e41141_d_n7, assign35760_e41141_d_n8, assign35760_e41141_d_n9, assign35760_e41141_d_n10, assign35760_e41141_d_n11, assign35760_e41141_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 != 0.0)) {
        let assign35760_e41139: f64 = (locals.var_vdsorg / locals.var_t6);
        (assign35760_e41139, (((locals.var_vdsorg_dn0 * locals.var_t6) - (locals.var_vdsorg * locals.var_t6_dn0)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vdsorg_dn2 * locals.var_t6) - (locals.var_vdsorg * locals.var_t6_dn2)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vdsorg_dn4 * locals.var_t6) - (locals.var_vdsorg * locals.var_t6_dn4)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vdsorg_dn5 * locals.var_t6) - (locals.var_vdsorg * locals.var_t6_dn5)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vdsorg_dn6 * locals.var_t6) - (locals.var_vdsorg * locals.var_t6_dn6)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vdsorg_dn7 * locals.var_t6) - (locals.var_vdsorg * locals.var_t6_dn7)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vdsorg_dn8 * locals.var_t6) - (locals.var_vdsorg * locals.var_t6_dn8)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vdsorg_dn9 * locals.var_t6) - (locals.var_vdsorg * locals.var_t6_dn9)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vdsorg_dn10 * locals.var_t6) - (locals.var_vdsorg * locals.var_t6_dn10)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vdsorg_dn11 * locals.var_t6) - (locals.var_vdsorg * locals.var_t6_dn11)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vdsorg_dn14 * locals.var_t6) - (locals.var_vdsorg * locals.var_t6_dn14)) / (locals.var_t6 * locals.var_t6)),)
    } else {
        (locals.var_vdseff0, locals.var_vdseff0_dn0, locals.var_vdseff0_dn2, locals.var_vdseff0_dn4, locals.var_vdseff0_dn5, locals.var_vdseff0_dn6, locals.var_vdseff0_dn7, locals.var_vdseff0_dn8, locals.var_vdseff0_dn9, locals.var_vdseff0_dn10, locals.var_vdseff0_dn11, locals.var_vdseff0_dn14,)
    }
};
        locals.var_vdseff0 = assign35760_e41141;
        locals.var_vdseff0_dn0 = assign35760_e41141_d_n0;
        locals.var_vdseff0_dn2 = assign35760_e41141_d_n2;
        locals.var_vdseff0_dn4 = assign35760_e41141_d_n4;
        locals.var_vdseff0_dn5 = assign35760_e41141_d_n5;
        locals.var_vdseff0_dn6 = assign35760_e41141_d_n6;
        locals.var_vdseff0_dn7 = assign35760_e41141_d_n7;
        locals.var_vdseff0_dn8 = assign35760_e41141_d_n8;
        locals.var_vdseff0_dn9 = assign35760_e41141_d_n9;
        locals.var_vdseff0_dn10 = assign35760_e41141_d_n10;
        locals.var_vdseff0_dn11 = assign35760_e41141_d_n11;
        locals.var_vdseff0_dn14 = assign35760_e41141_d_n14;
        locals.var_vdseff0_rv = 0.0;

        let (assign35770_e41150, assign35770_e41150_d_n0, assign35770_e41150_d_n2, assign35770_e41150_d_n4, assign35770_e41150_d_n5, assign35770_e41150_d_n6, assign35770_e41150_d_n7, assign35770_e41150_d_n8, assign35770_e41150_d_n9, assign35770_e41150_d_n10, assign35770_e41150_d_n11, assign35770_e41150_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard812 == 0.0)) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn14,)
    } else {
        (locals.var_vdseff0, locals.var_vdseff0_dn0, locals.var_vdseff0_dn2, locals.var_vdseff0_dn4, locals.var_vdseff0_dn5, locals.var_vdseff0_dn6, locals.var_vdseff0_dn7, locals.var_vdseff0_dn8, locals.var_vdseff0_dn9, locals.var_vdseff0_dn10, locals.var_vdseff0_dn11, locals.var_vdseff0_dn14,)
    }
};
        locals.var_vdseff0 = assign35770_e41150;
        locals.var_vdseff0_dn0 = assign35770_e41150_d_n0;
        locals.var_vdseff0_dn2 = assign35770_e41150_d_n2;
        locals.var_vdseff0_dn4 = assign35770_e41150_d_n4;
        locals.var_vdseff0_dn5 = assign35770_e41150_d_n5;
        locals.var_vdseff0_dn6 = assign35770_e41150_d_n6;
        locals.var_vdseff0_dn7 = assign35770_e41150_d_n7;
        locals.var_vdseff0_dn8 = assign35770_e41150_d_n8;
        locals.var_vdseff0_dn9 = assign35770_e41150_d_n9;
        locals.var_vdseff0_dn10 = assign35770_e41150_d_n10;
        locals.var_vdseff0_dn11 = assign35770_e41150_d_n11;
        locals.var_vdseff0_dn14 = assign35770_e41150_d_n14;
        locals.var_vdseff0_rv = 0.0;

        let (assign35780_e41162, assign35780_e41162_d_n0, assign35780_e41162_d_n2, assign35780_e41162_d_n4, assign35780_e41162_d_n5, assign35780_e41162_d_n6, assign35780_e41162_d_n7, assign35780_e41162_d_n8, assign35780_e41162_d_n9, assign35780_e41162_d_n10, assign35780_e41162_d_n11, assign35780_e41162_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign35780_e41157: f64 = (locals.var_phi_sl_dep - locals.var_phi_s0_dep);
        let assign35780_e41159: f64 = (assign35780_e41157 * locals.var_ninvde);
        let assign35780_e41160: f64 = (1.0 + assign35780_e41159);
        (assign35780_e41160, (((locals.var_phi_sl_dep_dn0 - locals.var_phi_s0_dep_dn0) * locals.var_ninvde) + (assign35780_e41157 * locals.var_ninvde_dn0)), (((locals.var_phi_sl_dep_dn2 - locals.var_phi_s0_dep_dn2) * locals.var_ninvde) + (assign35780_e41157 * locals.var_ninvde_dn2)), (((locals.var_phi_sl_dep_dn4 - locals.var_phi_s0_dep_dn4) * locals.var_ninvde) + (assign35780_e41157 * locals.var_ninvde_dn4)), (((locals.var_phi_sl_dep_dn5 - locals.var_phi_s0_dep_dn5) * locals.var_ninvde) + (assign35780_e41157 * locals.var_ninvde_dn5)), (((locals.var_phi_sl_dep_dn6 - locals.var_phi_s0_dep_dn6) * locals.var_ninvde) + (assign35780_e41157 * locals.var_ninvde_dn6)), (((locals.var_phi_sl_dep_dn7 - locals.var_phi_s0_dep_dn7) * locals.var_ninvde) + (assign35780_e41157 * locals.var_ninvde_dn7)), (((locals.var_phi_sl_dep_dn8 - locals.var_phi_s0_dep_dn8) * locals.var_ninvde) + (assign35780_e41157 * locals.var_ninvde_dn8)), (((locals.var_phi_sl_dep_dn9 - locals.var_phi_s0_dep_dn9) * locals.var_ninvde) + (assign35780_e41157 * locals.var_ninvde_dn9)), (((locals.var_phi_sl_dep_dn10 - locals.var_phi_s0_dep_dn10) * locals.var_ninvde) + (assign35780_e41157 * locals.var_ninvde_dn10)), (((locals.var_phi_sl_dep_dn11 - locals.var_phi_s0_dep_dn11) * locals.var_ninvde) + (assign35780_e41157 * locals.var_ninvde_dn11)), (((locals.var_phi_sl_dep_dn14 - locals.var_phi_s0_dep_dn14) * locals.var_ninvde) + (assign35780_e41157 * locals.var_ninvde_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign35780_e41162;
        locals.var_t4_dn0 = assign35780_e41162_d_n0;
        locals.var_t4_dn2 = assign35780_e41162_d_n2;
        locals.var_t4_dn4 = assign35780_e41162_d_n4;
        locals.var_t4_dn5 = assign35780_e41162_d_n5;
        locals.var_t4_dn6 = assign35780_e41162_d_n6;
        locals.var_t4_dn7 = assign35780_e41162_d_n7;
        locals.var_t4_dn8 = assign35780_e41162_d_n8;
        locals.var_t4_dn9 = assign35780_e41162_d_n9;
        locals.var_t4_dn10 = assign35780_e41162_d_n10;
        locals.var_t4_dn11 = assign35780_e41162_d_n11;
        locals.var_t4_dn14 = assign35780_e41162_d_n14;
        locals.var_t4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_117(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign35790_e41169, assign35790_e41169_d_n0, assign35790_e41169_d_n2, assign35790_e41169_d_n4, assign35790_e41169_d_n5, assign35790_e41169_d_n6, assign35790_e41169_d_n7, assign35790_e41169_d_n8, assign35790_e41169_d_n9, assign35790_e41169_d_n10, assign35790_e41169_d_n11, assign35790_e41169_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign35790_e41167: f64 = (-locals.var_qn_res0);
        (assign35790_e41167, (-locals.var_qn_res0_dn0), (-locals.var_qn_res0_dn2), (-locals.var_qn_res0_dn4), (-locals.var_qn_res0_dn5), (-locals.var_qn_res0_dn6), (-locals.var_qn_res0_dn7), (-locals.var_qn_res0_dn8), (-locals.var_qn_res0_dn9), (-locals.var_qn_res0_dn10), (-locals.var_qn_res0_dn11), (-locals.var_qn_res0_dn14),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn14,)
    }
};
        locals.var_qiu = assign35790_e41169;
        locals.var_qiu_dn0 = assign35790_e41169_d_n0;
        locals.var_qiu_dn2 = assign35790_e41169_d_n2;
        locals.var_qiu_dn4 = assign35790_e41169_d_n4;
        locals.var_qiu_dn5 = assign35790_e41169_d_n5;
        locals.var_qiu_dn6 = assign35790_e41169_d_n6;
        locals.var_qiu_dn7 = assign35790_e41169_d_n7;
        locals.var_qiu_dn8 = assign35790_e41169_d_n8;
        locals.var_qiu_dn9 = assign35790_e41169_d_n9;
        locals.var_qiu_dn10 = assign35790_e41169_d_n10;
        locals.var_qiu_dn11 = assign35790_e41169_d_n11;
        locals.var_qiu_dn14 = assign35790_e41169_d_n14;
        locals.var_qiu_rv = 0.0;

        let (assign35800_e41175, assign35800_e41175_d_n0, assign35800_e41175_d_n2, assign35800_e41175_d_n4, assign35800_e41175_d_n5, assign35800_e41175_d_n6, assign35800_e41175_d_n7, assign35800_e41175_d_n8, assign35800_e41175_d_n9, assign35800_e41175_d_n10, assign35800_e41175_d_n11, assign35800_e41175_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign35800_e41175;
        locals.var_t5_dn0 = assign35800_e41175_d_n0;
        locals.var_t5_dn2 = assign35800_e41175_d_n2;
        locals.var_t5_dn4 = assign35800_e41175_d_n4;
        locals.var_t5_dn5 = assign35800_e41175_d_n5;
        locals.var_t5_dn6 = assign35800_e41175_d_n6;
        locals.var_t5_dn7 = assign35800_e41175_d_n7;
        locals.var_t5_dn8 = assign35800_e41175_d_n8;
        locals.var_t5_dn9 = assign35800_e41175_d_n9;
        locals.var_t5_dn10 = assign35800_e41175_d_n10;
        locals.var_t5_dn11 = assign35800_e41175_d_n11;
        locals.var_t5_dn14 = assign35800_e41175_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign35810_e41183, assign35810_e41183_d_n0, assign35810_e41183_d_n2, assign35810_e41183_d_n4, assign35810_e41183_d_n5, assign35810_e41183_d_n6, assign35810_e41183_d_n7, assign35810_e41183_d_n8, assign35810_e41183_d_n9, assign35810_e41183_d_n10, assign35810_e41183_d_n11, assign35810_e41183_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign35810_e41181: f64 = (locals.var_t5 / locals.var_t4);
        (assign35810_e41181, (((locals.var_t5_dn0 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn2 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn4 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn5 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn6 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn7 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn8 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn9 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn10 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn11 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn14 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign35810_e41183;
        locals.var_t3_dn0 = assign35810_e41183_d_n0;
        locals.var_t3_dn2 = assign35810_e41183_d_n2;
        locals.var_t3_dn4 = assign35810_e41183_d_n4;
        locals.var_t3_dn5 = assign35810_e41183_d_n5;
        locals.var_t3_dn6 = assign35810_e41183_d_n6;
        locals.var_t3_dn7 = assign35810_e41183_d_n7;
        locals.var_t3_dn8 = assign35810_e41183_d_n8;
        locals.var_t3_dn9 = assign35810_e41183_d_n9;
        locals.var_t3_dn10 = assign35810_e41183_d_n10;
        locals.var_t3_dn11 = assign35810_e41183_d_n11;
        locals.var_t3_dn14 = assign35810_e41183_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign35820_e41189, assign35820_e41189_d_n0, assign35820_e41189_d_n2, assign35820_e41189_d_n4, assign35820_e41189_d_n5, assign35820_e41189_d_n6, assign35820_e41189_d_n7, assign35820_e41189_d_n8, assign35820_e41189_d_n9, assign35820_e41189_d_n10, assign35820_e41189_d_n11, assign35820_e41189_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    } else {
        (locals.var_eeff, locals.var_eeff_dn0, locals.var_eeff_dn2, locals.var_eeff_dn4, locals.var_eeff_dn5, locals.var_eeff_dn6, locals.var_eeff_dn7, locals.var_eeff_dn8, locals.var_eeff_dn9, locals.var_eeff_dn10, locals.var_eeff_dn11, locals.var_eeff_dn14,)
    }
};
        locals.var_eeff = assign35820_e41189;
        locals.var_eeff_dn0 = assign35820_e41189_d_n0;
        locals.var_eeff_dn2 = assign35820_e41189_d_n2;
        locals.var_eeff_dn4 = assign35820_e41189_d_n4;
        locals.var_eeff_dn5 = assign35820_e41189_d_n5;
        locals.var_eeff_dn6 = assign35820_e41189_d_n6;
        locals.var_eeff_dn7 = assign35820_e41189_d_n7;
        locals.var_eeff_dn8 = assign35820_e41189_d_n8;
        locals.var_eeff_dn9 = assign35820_e41189_d_n9;
        locals.var_eeff_dn10 = assign35820_e41189_d_n10;
        locals.var_eeff_dn11 = assign35820_e41189_d_n11;
        locals.var_eeff_dn14 = assign35820_e41189_d_n14;
        locals.var_eeff_rv = 0.0;

        let (assign35830_e41204, assign35830_e41204_d_n0, assign35830_e41204_d_n2, assign35830_e41204_d_n4, assign35830_e41204_d_n5, assign35830_e41204_d_n6, assign35830_e41204_d_n7, assign35830_e41204_d_n8, assign35830_e41204_d_n9, assign35830_e41204_d_n10, assign35830_e41204_d_n11, assign35830_e41204_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let (assign35830_e41202, assign35830_e41202_d_n0, assign35830_e41202_d_n2, assign35830_e41202_d_n4, assign35830_e41202_d_n5, assign35830_e41202_d_n6, assign35830_e41202_d_n7, assign35830_e41202_d_n8, assign35830_e41202_d_n9, assign35830_e41202_d_n10, assign35830_e41202_d_n11, assign35830_e41202_d_n14,) = {
            if (locals.var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign35830_e41200: f64 = (p.p376 - 1.0);
                let assign35830_e41201: f64 = (locals.var_eeff).powf(assign35830_e41200);
                (assign35830_e41201, if 0.0 == 0.0 && ((assign35830_e41200) as f64).is_finite() && ((assign35830_e41200) as f64).fract() == 0.0 { if assign35830_e41200 == 0.0 { 0.0 } else { (assign35830_e41200 * ((locals.var_eeff).powf(assign35830_e41200 - 1.0) * locals.var_eeff_dn0)) } } else { (assign35830_e41201 * (assign35830_e41200 * (locals.var_eeff_dn0 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35830_e41200) as f64).is_finite() && ((assign35830_e41200) as f64).fract() == 0.0 { if assign35830_e41200 == 0.0 { 0.0 } else { (assign35830_e41200 * ((locals.var_eeff).powf(assign35830_e41200 - 1.0) * locals.var_eeff_dn2)) } } else { (assign35830_e41201 * (assign35830_e41200 * (locals.var_eeff_dn2 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35830_e41200) as f64).is_finite() && ((assign35830_e41200) as f64).fract() == 0.0 { if assign35830_e41200 == 0.0 { 0.0 } else { (assign35830_e41200 * ((locals.var_eeff).powf(assign35830_e41200 - 1.0) * locals.var_eeff_dn4)) } } else { (assign35830_e41201 * (assign35830_e41200 * (locals.var_eeff_dn4 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35830_e41200) as f64).is_finite() && ((assign35830_e41200) as f64).fract() == 0.0 { if assign35830_e41200 == 0.0 { 0.0 } else { (assign35830_e41200 * ((locals.var_eeff).powf(assign35830_e41200 - 1.0) * locals.var_eeff_dn5)) } } else { (assign35830_e41201 * (assign35830_e41200 * (locals.var_eeff_dn5 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35830_e41200) as f64).is_finite() && ((assign35830_e41200) as f64).fract() == 0.0 { if assign35830_e41200 == 0.0 { 0.0 } else { (assign35830_e41200 * ((locals.var_eeff).powf(assign35830_e41200 - 1.0) * locals.var_eeff_dn6)) } } else { (assign35830_e41201 * (assign35830_e41200 * (locals.var_eeff_dn6 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35830_e41200) as f64).is_finite() && ((assign35830_e41200) as f64).fract() == 0.0 { if assign35830_e41200 == 0.0 { 0.0 } else { (assign35830_e41200 * ((locals.var_eeff).powf(assign35830_e41200 - 1.0) * locals.var_eeff_dn7)) } } else { (assign35830_e41201 * (assign35830_e41200 * (locals.var_eeff_dn7 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35830_e41200) as f64).is_finite() && ((assign35830_e41200) as f64).fract() == 0.0 { if assign35830_e41200 == 0.0 { 0.0 } else { (assign35830_e41200 * ((locals.var_eeff).powf(assign35830_e41200 - 1.0) * locals.var_eeff_dn8)) } } else { (assign35830_e41201 * (assign35830_e41200 * (locals.var_eeff_dn8 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35830_e41200) as f64).is_finite() && ((assign35830_e41200) as f64).fract() == 0.0 { if assign35830_e41200 == 0.0 { 0.0 } else { (assign35830_e41200 * ((locals.var_eeff).powf(assign35830_e41200 - 1.0) * locals.var_eeff_dn9)) } } else { (assign35830_e41201 * (assign35830_e41200 * (locals.var_eeff_dn9 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35830_e41200) as f64).is_finite() && ((assign35830_e41200) as f64).fract() == 0.0 { if assign35830_e41200 == 0.0 { 0.0 } else { (assign35830_e41200 * ((locals.var_eeff).powf(assign35830_e41200 - 1.0) * locals.var_eeff_dn10)) } } else { (assign35830_e41201 * (assign35830_e41200 * (locals.var_eeff_dn10 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35830_e41200) as f64).is_finite() && ((assign35830_e41200) as f64).fract() == 0.0 { if assign35830_e41200 == 0.0 { 0.0 } else { (assign35830_e41200 * ((locals.var_eeff).powf(assign35830_e41200 - 1.0) * locals.var_eeff_dn11)) } } else { (assign35830_e41201 * (assign35830_e41200 * (locals.var_eeff_dn11 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35830_e41200) as f64).is_finite() && ((assign35830_e41200) as f64).fract() == 0.0 { if assign35830_e41200 == 0.0 { 0.0 } else { (assign35830_e41200 * ((locals.var_eeff).powf(assign35830_e41200 - 1.0) * locals.var_eeff_dn14)) } } else { (assign35830_e41201 * (assign35830_e41200 * (locals.var_eeff_dn14 / locals.var_eeff))) },)
            }
        };
        (assign35830_e41202, assign35830_e41202_d_n0, assign35830_e41202_d_n2, assign35830_e41202_d_n4, assign35830_e41202_d_n5, assign35830_e41202_d_n6, assign35830_e41202_d_n7, assign35830_e41202_d_n8, assign35830_e41202_d_n9, assign35830_e41202_d_n10, assign35830_e41202_d_n11, assign35830_e41202_d_n14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign35830_e41204;
        locals.var_t5_dn0 = assign35830_e41204_d_n0;
        locals.var_t5_dn2 = assign35830_e41204_d_n2;
        locals.var_t5_dn4 = assign35830_e41204_d_n4;
        locals.var_t5_dn5 = assign35830_e41204_d_n5;
        locals.var_t5_dn6 = assign35830_e41204_d_n6;
        locals.var_t5_dn7 = assign35830_e41204_d_n7;
        locals.var_t5_dn8 = assign35830_e41204_d_n8;
        locals.var_t5_dn9 = assign35830_e41204_d_n9;
        locals.var_t5_dn10 = assign35830_e41204_d_n10;
        locals.var_t5_dn11 = assign35830_e41204_d_n11;
        locals.var_t5_dn14 = assign35830_e41204_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign35840_e41212, assign35840_e41212_d_n0, assign35840_e41212_d_n2, assign35840_e41212_d_n4, assign35840_e41212_d_n5, assign35840_e41212_d_n6, assign35840_e41212_d_n7, assign35840_e41212_d_n8, assign35840_e41212_d_n9, assign35840_e41212_d_n10, assign35840_e41212_d_n11, assign35840_e41212_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign35840_e41210: f64 = (locals.var_t5 * locals.var_eeff);
        (assign35840_e41210, ((locals.var_t5_dn0 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn0)), ((locals.var_t5_dn2 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn2)), ((locals.var_t5_dn4 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn4)), ((locals.var_t5_dn5 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn5)), ((locals.var_t5_dn6 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn6)), ((locals.var_t5_dn7 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn7)), ((locals.var_t5_dn8 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn8)), ((locals.var_t5_dn9 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn9)), ((locals.var_t5_dn10 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn10)), ((locals.var_t5_dn11 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn11)), ((locals.var_t5_dn14 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn14)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign35840_e41212;
        locals.var_t8_dn0 = assign35840_e41212_d_n0;
        locals.var_t8_dn2 = assign35840_e41212_d_n2;
        locals.var_t8_dn4 = assign35840_e41212_d_n4;
        locals.var_t8_dn5 = assign35840_e41212_d_n5;
        locals.var_t8_dn6 = assign35840_e41212_d_n6;
        locals.var_t8_dn7 = assign35840_e41212_d_n7;
        locals.var_t8_dn8 = assign35840_e41212_d_n8;
        locals.var_t8_dn9 = assign35840_e41212_d_n9;
        locals.var_t8_dn10 = assign35840_e41212_d_n10;
        locals.var_t8_dn11 = assign35840_e41212_d_n11;
        locals.var_t8_dn14 = assign35840_e41212_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign35850_e41220, assign35850_e41220_d_n0, assign35850_e41220_d_n2, assign35850_e41220_d_n4, assign35850_e41220_d_n5, assign35850_e41220_d_n6, assign35850_e41220_d_n7, assign35850_e41220_d_n8, assign35850_e41220_d_n9, assign35850_e41220_d_n10, assign35850_e41220_d_n11, assign35850_e41220_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign35850_e41218: f64 = (1.6021918e-19 * 10000.0);
        (assign35850_e41218, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign35850_e41220;
        locals.var_t9_dn0 = assign35850_e41220_d_n0;
        locals.var_t9_dn2 = assign35850_e41220_d_n2;
        locals.var_t9_dn4 = assign35850_e41220_d_n4;
        locals.var_t9_dn5 = assign35850_e41220_d_n5;
        locals.var_t9_dn6 = assign35850_e41220_d_n6;
        locals.var_t9_dn7 = assign35850_e41220_d_n7;
        locals.var_t9_dn8 = assign35850_e41220_d_n8;
        locals.var_t9_dn9 = assign35850_e41220_d_n9;
        locals.var_t9_dn10 = assign35850_e41220_d_n10;
        locals.var_t9_dn11 = assign35850_e41220_d_n11;
        locals.var_t9_dn14 = assign35850_e41220_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign35860_e41228, assign35860_e41228_d_n0, assign35860_e41228_d_n2, assign35860_e41228_d_n4, assign35860_e41228_d_n5, assign35860_e41228_d_n6, assign35860_e41228_d_n7, assign35860_e41228_d_n8, assign35860_e41228_d_n9, assign35860_e41228_d_n10, assign35860_e41228_d_n11, assign35860_e41228_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign35860_e41226: f64 = (locals.var_qiu / locals.var_t9);
        (assign35860_e41226, (((locals.var_qiu_dn0 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn0)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn2 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn2)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn4 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn4)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn5 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn5)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn6 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn6)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn7 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn7)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn8 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn8)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn9 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn9)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn10 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn10)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn11 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn11)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn14 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn14)) / (locals.var_t9 * locals.var_t9)),)
    } else {
        (locals.var_rns, locals.var_rns_dn0, locals.var_rns_dn2, locals.var_rns_dn4, locals.var_rns_dn5, locals.var_rns_dn6, locals.var_rns_dn7, locals.var_rns_dn8, locals.var_rns_dn9, locals.var_rns_dn10, locals.var_rns_dn11, locals.var_rns_dn14,)
    }
};
        locals.var_rns = assign35860_e41228;
        locals.var_rns_dn0 = assign35860_e41228_d_n0;
        locals.var_rns_dn2 = assign35860_e41228_d_n2;
        locals.var_rns_dn4 = assign35860_e41228_d_n4;
        locals.var_rns_dn5 = assign35860_e41228_d_n5;
        locals.var_rns_dn6 = assign35860_e41228_d_n6;
        locals.var_rns_dn7 = assign35860_e41228_d_n7;
        locals.var_rns_dn8 = assign35860_e41228_d_n8;
        locals.var_rns_dn9 = assign35860_e41228_d_n9;
        locals.var_rns_dn10 = assign35860_e41228_d_n10;
        locals.var_rns_dn11 = assign35860_e41228_d_n11;
        locals.var_rns_dn14 = assign35860_e41228_d_n14;
        locals.var_rns_rv = 0.0;

        let (assign35870_e41248, assign35870_e41248_d_n0, assign35870_e41248_d_n2, assign35870_e41248_d_n4, assign35870_e41248_d_n5, assign35870_e41248_d_n6, assign35870_e41248_d_n7, assign35870_e41248_d_n8, assign35870_e41248_d_n9, assign35870_e41248_d_n10, assign35870_e41248_d_n11, assign35870_e41248_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign35870_e41236: f64 = (locals.var_uc_depmue1 * locals.var_rns);
        let assign35870_e41238: f64 = (assign35870_e41236 / 100000000000.0);
        let assign35870_e41239: f64 = (locals.var_uc_depmue0 + assign35870_e41238);
        let assign35870_e41241: f64 = (assign35870_e41239 + 1e-25);
        let assign35870_e41242: f64 = (1.0 / assign35870_e41241);
        let assign35870_e41245: f64 = (locals.var_depmphn0 * locals.var_t8);
        let assign35870_e41246: f64 = (assign35870_e41242 + assign35870_e41245);
        (assign35870_e41246, ((-((locals.var_uc_depmue0_dn0 + (((locals.var_uc_depmue1_dn0 * locals.var_rns) + (locals.var_uc_depmue1 * locals.var_rns_dn0)) / 100000000000.0)) / (assign35870_e41241 * assign35870_e41241))) + ((locals.var_depmphn0_dn0 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn0))), ((-((locals.var_uc_depmue0_dn2 + (((locals.var_uc_depmue1_dn2 * locals.var_rns) + (locals.var_uc_depmue1 * locals.var_rns_dn2)) / 100000000000.0)) / (assign35870_e41241 * assign35870_e41241))) + ((locals.var_depmphn0_dn2 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn2))), ((-((locals.var_uc_depmue0_dn4 + (((locals.var_uc_depmue1_dn4 * locals.var_rns) + (locals.var_uc_depmue1 * locals.var_rns_dn4)) / 100000000000.0)) / (assign35870_e41241 * assign35870_e41241))) + ((locals.var_depmphn0_dn4 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn4))), ((-((locals.var_uc_depmue0_dn5 + (((locals.var_uc_depmue1_dn5 * locals.var_rns) + (locals.var_uc_depmue1 * locals.var_rns_dn5)) / 100000000000.0)) / (assign35870_e41241 * assign35870_e41241))) + ((locals.var_depmphn0_dn5 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn5))), ((-((locals.var_uc_depmue0_dn6 + (((locals.var_uc_depmue1_dn6 * locals.var_rns) + (locals.var_uc_depmue1 * locals.var_rns_dn6)) / 100000000000.0)) / (assign35870_e41241 * assign35870_e41241))) + ((locals.var_depmphn0_dn6 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn6))), ((-((locals.var_uc_depmue0_dn7 + (((locals.var_uc_depmue1_dn7 * locals.var_rns) + (locals.var_uc_depmue1 * locals.var_rns_dn7)) / 100000000000.0)) / (assign35870_e41241 * assign35870_e41241))) + ((locals.var_depmphn0_dn7 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn7))), ((-((locals.var_uc_depmue0_dn8 + (((locals.var_uc_depmue1_dn8 * locals.var_rns) + (locals.var_uc_depmue1 * locals.var_rns_dn8)) / 100000000000.0)) / (assign35870_e41241 * assign35870_e41241))) + ((locals.var_depmphn0_dn8 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn8))), ((-((locals.var_uc_depmue0_dn9 + (((locals.var_uc_depmue1_dn9 * locals.var_rns) + (locals.var_uc_depmue1 * locals.var_rns_dn9)) / 100000000000.0)) / (assign35870_e41241 * assign35870_e41241))) + ((locals.var_depmphn0_dn9 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn9))), ((-((locals.var_uc_depmue0_dn10 + (((locals.var_uc_depmue1_dn10 * locals.var_rns) + (locals.var_uc_depmue1 * locals.var_rns_dn10)) / 100000000000.0)) / (assign35870_e41241 * assign35870_e41241))) + ((locals.var_depmphn0_dn10 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn10))), ((-((locals.var_uc_depmue0_dn11 + (((locals.var_uc_depmue1_dn11 * locals.var_rns) + (locals.var_uc_depmue1 * locals.var_rns_dn11)) / 100000000000.0)) / (assign35870_e41241 * assign35870_e41241))) + ((locals.var_depmphn0_dn11 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn11))), ((-((locals.var_uc_depmue0_dn14 + (((locals.var_uc_depmue1_dn14 * locals.var_rns) + (locals.var_uc_depmue1 * locals.var_rns_dn14)) / 100000000000.0)) / (assign35870_e41241 * assign35870_e41241))) + ((locals.var_depmphn0_dn14 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign35870_e41248;
        locals.var_t1_dn0 = assign35870_e41248_d_n0;
        locals.var_t1_dn2 = assign35870_e41248_d_n2;
        locals.var_t1_dn4 = assign35870_e41248_d_n4;
        locals.var_t1_dn5 = assign35870_e41248_d_n5;
        locals.var_t1_dn6 = assign35870_e41248_d_n6;
        locals.var_t1_dn7 = assign35870_e41248_d_n7;
        locals.var_t1_dn8 = assign35870_e41248_d_n8;
        locals.var_t1_dn9 = assign35870_e41248_d_n9;
        locals.var_t1_dn10 = assign35870_e41248_d_n10;
        locals.var_t1_dn11 = assign35870_e41248_d_n11;
        locals.var_t1_dn14 = assign35870_e41248_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign35880_e41256, assign35880_e41256_d_n0, assign35880_e41256_d_n2, assign35880_e41256_d_n4, assign35880_e41256_d_n5, assign35880_e41256_d_n6, assign35880_e41256_d_n7, assign35880_e41256_d_n8, assign35880_e41256_d_n9, assign35880_e41256_d_n10, assign35880_e41256_d_n11, assign35880_e41256_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign35880_e41254: f64 = (1.0 / locals.var_t1);
        (assign35880_e41254, (-(locals.var_t1_dn0 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn2 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn11 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn14 / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn14,)
    }
};
        locals.var_muun = assign35880_e41256;
        locals.var_muun_dn0 = assign35880_e41256_d_n0;
        locals.var_muun_dn2 = assign35880_e41256_d_n2;
        locals.var_muun_dn4 = assign35880_e41256_d_n4;
        locals.var_muun_dn5 = assign35880_e41256_d_n5;
        locals.var_muun_dn6 = assign35880_e41256_d_n6;
        locals.var_muun_dn7 = assign35880_e41256_d_n7;
        locals.var_muun_dn8 = assign35880_e41256_d_n8;
        locals.var_muun_dn9 = assign35880_e41256_d_n9;
        locals.var_muun_dn10 = assign35880_e41256_d_n10;
        locals.var_muun_dn11 = assign35880_e41256_d_n11;
        locals.var_muun_dn14 = assign35880_e41256_d_n14;
        locals.var_muun_rv = 0.0;

        let (assign35890_e41264, assign35890_e41264_d_n0, assign35890_e41264_d_n2, assign35890_e41264_d_n4, assign35890_e41264_d_n5, assign35890_e41264_d_n6, assign35890_e41264_d_n7, assign35890_e41264_d_n8, assign35890_e41264_d_n9, assign35890_e41264_d_n10, assign35890_e41264_d_n11, assign35890_e41264_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign35890_e41262: f64 = (locals.var_muun / 10000.0);
        (assign35890_e41262, (locals.var_muun_dn0 / 10000.0), (locals.var_muun_dn2 / 10000.0), (locals.var_muun_dn4 / 10000.0), (locals.var_muun_dn5 / 10000.0), (locals.var_muun_dn6 / 10000.0), (locals.var_muun_dn7 / 10000.0), (locals.var_muun_dn8 / 10000.0), (locals.var_muun_dn9 / 10000.0), (locals.var_muun_dn10 / 10000.0), (locals.var_muun_dn11 / 10000.0), (locals.var_muun_dn14 / 10000.0),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn14,)
    }
};
        locals.var_muun = assign35890_e41264;
        locals.var_muun_dn0 = assign35890_e41264_d_n0;
        locals.var_muun_dn2 = assign35890_e41264_d_n2;
        locals.var_muun_dn4 = assign35890_e41264_d_n4;
        locals.var_muun_dn5 = assign35890_e41264_d_n5;
        locals.var_muun_dn6 = assign35890_e41264_d_n6;
        locals.var_muun_dn7 = assign35890_e41264_d_n7;
        locals.var_muun_dn8 = assign35890_e41264_d_n8;
        locals.var_muun_dn9 = assign35890_e41264_d_n9;
        locals.var_muun_dn10 = assign35890_e41264_d_n10;
        locals.var_muun_dn11 = assign35890_e41264_d_n11;
        locals.var_muun_dn14 = assign35890_e41264_d_n14;
        locals.var_muun_rv = 0.0;

        let (assign35900_e41272, assign35900_e41272_d_n0, assign35900_e41272_d_n2, assign35900_e41272_d_n4, assign35900_e41272_d_n5, assign35900_e41272_d_n6, assign35900_e41272_d_n7, assign35900_e41272_d_n8, assign35900_e41272_d_n9, assign35900_e41272_d_n10, assign35900_e41272_d_n11, assign35900_e41272_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign35900_e41270: f64 = (locals.var_vdseff0 / locals.var_lch);
        (assign35900_e41270, (((locals.var_vdseff0_dn0 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn2 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn4 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn5 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn6 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn7 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn8 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn9 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn10 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn11 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn14 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn14)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_edri__blk559, locals.var_edri__blk559_dn0, locals.var_edri__blk559_dn2, locals.var_edri__blk559_dn4, locals.var_edri__blk559_dn5, locals.var_edri__blk559_dn6, locals.var_edri__blk559_dn7, locals.var_edri__blk559_dn8, locals.var_edri__blk559_dn9, locals.var_edri__blk559_dn10, locals.var_edri__blk559_dn11, locals.var_edri__blk559_dn14,)
    }
};
        locals.var_edri__blk559 = assign35900_e41272;
        locals.var_edri__blk559_dn0 = assign35900_e41272_d_n0;
        locals.var_edri__blk559_dn2 = assign35900_e41272_d_n2;
        locals.var_edri__blk559_dn4 = assign35900_e41272_d_n4;
        locals.var_edri__blk559_dn5 = assign35900_e41272_d_n5;
        locals.var_edri__blk559_dn6 = assign35900_e41272_d_n6;
        locals.var_edri__blk559_dn7 = assign35900_e41272_d_n7;
        locals.var_edri__blk559_dn8 = assign35900_e41272_d_n8;
        locals.var_edri__blk559_dn9 = assign35900_e41272_d_n9;
        locals.var_edri__blk559_dn10 = assign35900_e41272_d_n10;
        locals.var_edri__blk559_dn11 = assign35900_e41272_d_n11;
        locals.var_edri__blk559_dn14 = assign35900_e41272_d_n14;
        locals.var_edri__blk559_rv = 0.0;

        let (assign35910_e41282, assign35910_e41282_d_n0, assign35910_e41282_d_n2, assign35910_e41282_d_n4, assign35910_e41282_d_n5, assign35910_e41282_d_n6, assign35910_e41282_d_n7, assign35910_e41282_d_n8, assign35910_e41282_d_n9, assign35910_e41282_d_n10, assign35910_e41282_d_n11, assign35910_e41282_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign35910_e41278: f64 = (locals.var_muun * locals.var_edri__blk559);
        let assign35910_e41280: f64 = (assign35910_e41278 / locals.var_uc_depvmax);
        (assign35910_e41280, (((((locals.var_muun_dn0 * locals.var_edri__blk559) + (locals.var_muun * locals.var_edri__blk559_dn0)) * locals.var_uc_depvmax) - (assign35910_e41278 * locals.var_uc_depvmax_dn0)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn2 * locals.var_edri__blk559) + (locals.var_muun * locals.var_edri__blk559_dn2)) * locals.var_uc_depvmax) - (assign35910_e41278 * locals.var_uc_depvmax_dn2)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn4 * locals.var_edri__blk559) + (locals.var_muun * locals.var_edri__blk559_dn4)) * locals.var_uc_depvmax) - (assign35910_e41278 * locals.var_uc_depvmax_dn4)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn5 * locals.var_edri__blk559) + (locals.var_muun * locals.var_edri__blk559_dn5)) * locals.var_uc_depvmax) - (assign35910_e41278 * locals.var_uc_depvmax_dn5)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn6 * locals.var_edri__blk559) + (locals.var_muun * locals.var_edri__blk559_dn6)) * locals.var_uc_depvmax) - (assign35910_e41278 * locals.var_uc_depvmax_dn6)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn7 * locals.var_edri__blk559) + (locals.var_muun * locals.var_edri__blk559_dn7)) * locals.var_uc_depvmax) - (assign35910_e41278 * locals.var_uc_depvmax_dn7)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn8 * locals.var_edri__blk559) + (locals.var_muun * locals.var_edri__blk559_dn8)) * locals.var_uc_depvmax) - (assign35910_e41278 * locals.var_uc_depvmax_dn8)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn9 * locals.var_edri__blk559) + (locals.var_muun * locals.var_edri__blk559_dn9)) * locals.var_uc_depvmax) - (assign35910_e41278 * locals.var_uc_depvmax_dn9)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn10 * locals.var_edri__blk559) + (locals.var_muun * locals.var_edri__blk559_dn10)) * locals.var_uc_depvmax) - (assign35910_e41278 * locals.var_uc_depvmax_dn10)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn11 * locals.var_edri__blk559) + (locals.var_muun * locals.var_edri__blk559_dn11)) * locals.var_uc_depvmax) - (assign35910_e41278 * locals.var_uc_depvmax_dn11)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn14 * locals.var_edri__blk559) + (locals.var_muun * locals.var_edri__blk559_dn14)) * locals.var_uc_depvmax) - (assign35910_e41278 * locals.var_uc_depvmax_dn14)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign35910_e41282;
        locals.var_t1_dn0 = assign35910_e41282_d_n0;
        locals.var_t1_dn2 = assign35910_e41282_d_n2;
        locals.var_t1_dn4 = assign35910_e41282_d_n4;
        locals.var_t1_dn5 = assign35910_e41282_d_n5;
        locals.var_t1_dn6 = assign35910_e41282_d_n6;
        locals.var_t1_dn7 = assign35910_e41282_d_n7;
        locals.var_t1_dn8 = assign35910_e41282_d_n8;
        locals.var_t1_dn9 = assign35910_e41282_d_n9;
        locals.var_t1_dn10 = assign35910_e41282_d_n10;
        locals.var_t1_dn11 = assign35910_e41282_d_n11;
        locals.var_t1_dn14 = assign35910_e41282_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign35920_e41295, assign35920_e41295_d_n0, assign35920_e41295_d_n2, assign35920_e41295_d_n4, assign35920_e41295_d_n5, assign35920_e41295_d_n6, assign35920_e41295_d_n7, assign35920_e41295_d_n8, assign35920_e41295_d_n9, assign35920_e41295_d_n10, assign35920_e41295_d_n11, assign35920_e41295_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let (assign35920_e41293, assign35920_e41293_d_n0, assign35920_e41293_d_n2, assign35920_e41293_d_n4, assign35920_e41293_d_n5, assign35920_e41293_d_n6, assign35920_e41293_d_n7, assign35920_e41293_d_n8, assign35920_e41293_d_n9, assign35920_e41293_d_n10, assign35920_e41293_d_n11, assign35920_e41293_d_n14,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign35920_e41292: f64 = (locals.var_t1).powf(p.p378);
                (assign35920_e41292, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn0)) } } else { (assign35920_e41292 * (p.p378 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn2)) } } else { (assign35920_e41292 * (p.p378 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn4)) } } else { (assign35920_e41292 * (p.p378 * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn5)) } } else { (assign35920_e41292 * (p.p378 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn6)) } } else { (assign35920_e41292 * (p.p378 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn7)) } } else { (assign35920_e41292 * (p.p378 * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn8)) } } else { (assign35920_e41292 * (p.p378 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn9)) } } else { (assign35920_e41292 * (p.p378 * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn10)) } } else { (assign35920_e41292 * (p.p378 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn11)) } } else { (assign35920_e41292 * (p.p378 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn14)) } } else { (assign35920_e41292 * (p.p378 * (locals.var_t1_dn14 / locals.var_t1))) },)
            }
        };
        (assign35920_e41293, assign35920_e41293_d_n0, assign35920_e41293_d_n2, assign35920_e41293_d_n4, assign35920_e41293_d_n5, assign35920_e41293_d_n6, assign35920_e41293_d_n7, assign35920_e41293_d_n8, assign35920_e41293_d_n9, assign35920_e41293_d_n10, assign35920_e41293_d_n11, assign35920_e41293_d_n14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign35920_e41295;
        locals.var_t2_dn0 = assign35920_e41295_d_n0;
        locals.var_t2_dn2 = assign35920_e41295_d_n2;
        locals.var_t2_dn4 = assign35920_e41295_d_n4;
        locals.var_t2_dn5 = assign35920_e41295_d_n5;
        locals.var_t2_dn6 = assign35920_e41295_d_n6;
        locals.var_t2_dn7 = assign35920_e41295_d_n7;
        locals.var_t2_dn8 = assign35920_e41295_d_n8;
        locals.var_t2_dn9 = assign35920_e41295_d_n9;
        locals.var_t2_dn10 = assign35920_e41295_d_n10;
        locals.var_t2_dn11 = assign35920_e41295_d_n11;
        locals.var_t2_dn14 = assign35920_e41295_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign35930_e41303, assign35930_e41303_d_n0, assign35930_e41303_d_n2, assign35930_e41303_d_n4, assign35930_e41303_d_n5, assign35930_e41303_d_n6, assign35930_e41303_d_n7, assign35930_e41303_d_n8, assign35930_e41303_d_n9, assign35930_e41303_d_n10, assign35930_e41303_d_n11, assign35930_e41303_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign35930_e41301: f64 = (1.0 + locals.var_t2);
        (assign35930_e41301, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign35930_e41303;
        locals.var_t3_dn0 = assign35930_e41303_d_n0;
        locals.var_t3_dn2 = assign35930_e41303_d_n2;
        locals.var_t3_dn4 = assign35930_e41303_d_n4;
        locals.var_t3_dn5 = assign35930_e41303_d_n5;
        locals.var_t3_dn6 = assign35930_e41303_d_n6;
        locals.var_t3_dn7 = assign35930_e41303_d_n7;
        locals.var_t3_dn8 = assign35930_e41303_d_n8;
        locals.var_t3_dn9 = assign35930_e41303_d_n9;
        locals.var_t3_dn10 = assign35930_e41303_d_n10;
        locals.var_t3_dn11 = assign35930_e41303_d_n11;
        locals.var_t3_dn14 = assign35930_e41303_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign35940_e41318, assign35940_e41318_d_n0, assign35940_e41318_d_n2, assign35940_e41318_d_n4, assign35940_e41318_d_n5, assign35940_e41318_d_n6, assign35940_e41318_d_n7, assign35940_e41318_d_n8, assign35940_e41318_d_n9, assign35940_e41318_d_n10, assign35940_e41318_d_n11, assign35940_e41318_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let (assign35940_e41316, assign35940_e41316_d_n0, assign35940_e41316_d_n2, assign35940_e41316_d_n4, assign35940_e41316_d_n5, assign35940_e41316_d_n6, assign35940_e41316_d_n7, assign35940_e41316_d_n8, assign35940_e41316_d_n9, assign35940_e41316_d_n10, assign35940_e41316_d_n11, assign35940_e41316_d_n14,) = {
            if (locals.var_t3 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign35940_e41314: f64 = (1.0 / p.p378);
                let assign35940_e41315: f64 = (locals.var_t3).powf(assign35940_e41314);
                (assign35940_e41315, if 0.0 == 0.0 && ((assign35940_e41314) as f64).is_finite() && ((assign35940_e41314) as f64).fract() == 0.0 { if assign35940_e41314 == 0.0 { 0.0 } else { (assign35940_e41314 * ((locals.var_t3).powf(assign35940_e41314 - 1.0) * locals.var_t3_dn0)) } } else { (assign35940_e41315 * (assign35940_e41314 * (locals.var_t3_dn0 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign35940_e41314) as f64).is_finite() && ((assign35940_e41314) as f64).fract() == 0.0 { if assign35940_e41314 == 0.0 { 0.0 } else { (assign35940_e41314 * ((locals.var_t3).powf(assign35940_e41314 - 1.0) * locals.var_t3_dn2)) } } else { (assign35940_e41315 * (assign35940_e41314 * (locals.var_t3_dn2 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign35940_e41314) as f64).is_finite() && ((assign35940_e41314) as f64).fract() == 0.0 { if assign35940_e41314 == 0.0 { 0.0 } else { (assign35940_e41314 * ((locals.var_t3).powf(assign35940_e41314 - 1.0) * locals.var_t3_dn4)) } } else { (assign35940_e41315 * (assign35940_e41314 * (locals.var_t3_dn4 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign35940_e41314) as f64).is_finite() && ((assign35940_e41314) as f64).fract() == 0.0 { if assign35940_e41314 == 0.0 { 0.0 } else { (assign35940_e41314 * ((locals.var_t3).powf(assign35940_e41314 - 1.0) * locals.var_t3_dn5)) } } else { (assign35940_e41315 * (assign35940_e41314 * (locals.var_t3_dn5 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign35940_e41314) as f64).is_finite() && ((assign35940_e41314) as f64).fract() == 0.0 { if assign35940_e41314 == 0.0 { 0.0 } else { (assign35940_e41314 * ((locals.var_t3).powf(assign35940_e41314 - 1.0) * locals.var_t3_dn6)) } } else { (assign35940_e41315 * (assign35940_e41314 * (locals.var_t3_dn6 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign35940_e41314) as f64).is_finite() && ((assign35940_e41314) as f64).fract() == 0.0 { if assign35940_e41314 == 0.0 { 0.0 } else { (assign35940_e41314 * ((locals.var_t3).powf(assign35940_e41314 - 1.0) * locals.var_t3_dn7)) } } else { (assign35940_e41315 * (assign35940_e41314 * (locals.var_t3_dn7 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign35940_e41314) as f64).is_finite() && ((assign35940_e41314) as f64).fract() == 0.0 { if assign35940_e41314 == 0.0 { 0.0 } else { (assign35940_e41314 * ((locals.var_t3).powf(assign35940_e41314 - 1.0) * locals.var_t3_dn8)) } } else { (assign35940_e41315 * (assign35940_e41314 * (locals.var_t3_dn8 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign35940_e41314) as f64).is_finite() && ((assign35940_e41314) as f64).fract() == 0.0 { if assign35940_e41314 == 0.0 { 0.0 } else { (assign35940_e41314 * ((locals.var_t3).powf(assign35940_e41314 - 1.0) * locals.var_t3_dn9)) } } else { (assign35940_e41315 * (assign35940_e41314 * (locals.var_t3_dn9 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign35940_e41314) as f64).is_finite() && ((assign35940_e41314) as f64).fract() == 0.0 { if assign35940_e41314 == 0.0 { 0.0 } else { (assign35940_e41314 * ((locals.var_t3).powf(assign35940_e41314 - 1.0) * locals.var_t3_dn10)) } } else { (assign35940_e41315 * (assign35940_e41314 * (locals.var_t3_dn10 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign35940_e41314) as f64).is_finite() && ((assign35940_e41314) as f64).fract() == 0.0 { if assign35940_e41314 == 0.0 { 0.0 } else { (assign35940_e41314 * ((locals.var_t3).powf(assign35940_e41314 - 1.0) * locals.var_t3_dn11)) } } else { (assign35940_e41315 * (assign35940_e41314 * (locals.var_t3_dn11 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign35940_e41314) as f64).is_finite() && ((assign35940_e41314) as f64).fract() == 0.0 { if assign35940_e41314 == 0.0 { 0.0 } else { (assign35940_e41314 * ((locals.var_t3).powf(assign35940_e41314 - 1.0) * locals.var_t3_dn14)) } } else { (assign35940_e41315 * (assign35940_e41314 * (locals.var_t3_dn14 / locals.var_t3))) },)
            }
        };
        (assign35940_e41316, assign35940_e41316_d_n0, assign35940_e41316_d_n2, assign35940_e41316_d_n4, assign35940_e41316_d_n5, assign35940_e41316_d_n6, assign35940_e41316_d_n7, assign35940_e41316_d_n8, assign35940_e41316_d_n9, assign35940_e41316_d_n10, assign35940_e41316_d_n11, assign35940_e41316_d_n14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign35940_e41318;
        locals.var_t4_dn0 = assign35940_e41318_d_n0;
        locals.var_t4_dn2 = assign35940_e41318_d_n2;
        locals.var_t4_dn4 = assign35940_e41318_d_n4;
        locals.var_t4_dn5 = assign35940_e41318_d_n5;
        locals.var_t4_dn6 = assign35940_e41318_d_n6;
        locals.var_t4_dn7 = assign35940_e41318_d_n7;
        locals.var_t4_dn8 = assign35940_e41318_d_n8;
        locals.var_t4_dn9 = assign35940_e41318_d_n9;
        locals.var_t4_dn10 = assign35940_e41318_d_n10;
        locals.var_t4_dn11 = assign35940_e41318_d_n11;
        locals.var_t4_dn14 = assign35940_e41318_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign35950_e41326, assign35950_e41326_d_n0, assign35950_e41326_d_n2, assign35950_e41326_d_n4, assign35950_e41326_d_n5, assign35950_e41326_d_n6, assign35950_e41326_d_n7, assign35950_e41326_d_n8, assign35950_e41326_d_n9, assign35950_e41326_d_n10, assign35950_e41326_d_n11, assign35950_e41326_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign35950_e41324: f64 = (locals.var_muun / locals.var_t4);
        (assign35950_e41324, (((locals.var_muun_dn0 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn2 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn4 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn5 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn6 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn7 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn8 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn9 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn10 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn11 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn14 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_mu_res__blk510, locals.var_mu_res__blk510_dn0, locals.var_mu_res__blk510_dn2, locals.var_mu_res__blk510_dn4, locals.var_mu_res__blk510_dn5, locals.var_mu_res__blk510_dn6, locals.var_mu_res__blk510_dn7, locals.var_mu_res__blk510_dn8, locals.var_mu_res__blk510_dn9, locals.var_mu_res__blk510_dn10, locals.var_mu_res__blk510_dn11, locals.var_mu_res__blk510_dn14,)
    }
};
        locals.var_mu_res__blk510 = assign35950_e41326;
        locals.var_mu_res__blk510_dn0 = assign35950_e41326_d_n0;
        locals.var_mu_res__blk510_dn2 = assign35950_e41326_d_n2;
        locals.var_mu_res__blk510_dn4 = assign35950_e41326_d_n4;
        locals.var_mu_res__blk510_dn5 = assign35950_e41326_d_n5;
        locals.var_mu_res__blk510_dn6 = assign35950_e41326_d_n6;
        locals.var_mu_res__blk510_dn7 = assign35950_e41326_d_n7;
        locals.var_mu_res__blk510_dn8 = assign35950_e41326_d_n8;
        locals.var_mu_res__blk510_dn9 = assign35950_e41326_d_n9;
        locals.var_mu_res__blk510_dn10 = assign35950_e41326_d_n10;
        locals.var_mu_res__blk510_dn11 = assign35950_e41326_d_n11;
        locals.var_mu_res__blk510_dn14 = assign35950_e41326_d_n14;
        locals.var_mu_res__blk510_rv = 0.0;

        let (assign35960_e41339, assign35960_e41339_d_n0, assign35960_e41339_d_n2, assign35960_e41339_d_n4, assign35960_e41339_d_n5, assign35960_e41339_d_n6, assign35960_e41339_d_n7, assign35960_e41339_d_n8, assign35960_e41339_d_n9, assign35960_e41339_d_n10, assign35960_e41339_d_n11, assign35960_e41339_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign35960_e41332: f64 = (-locals.var_qn_res0);
        let assign35960_e41333: f64 = (locals.var_weff_nf * assign35960_e41332);
        let assign35960_e41335: f64 = (assign35960_e41333 * locals.var_mu_res__blk510);
        let assign35960_e41337: f64 = (assign35960_e41335 * locals.var_edri__blk559);
        (assign35960_e41337, (((((locals.var_weff_nf * (-locals.var_qn_res0_dn0)) * locals.var_mu_res__blk510) + (assign35960_e41333 * locals.var_mu_res__blk510_dn0)) * locals.var_edri__blk559) + (assign35960_e41335 * locals.var_edri__blk559_dn0)), (((((locals.var_weff_nf * (-locals.var_qn_res0_dn2)) * locals.var_mu_res__blk510) + (assign35960_e41333 * locals.var_mu_res__blk510_dn2)) * locals.var_edri__blk559) + (assign35960_e41335 * locals.var_edri__blk559_dn2)), (((((locals.var_weff_nf * (-locals.var_qn_res0_dn4)) * locals.var_mu_res__blk510) + (assign35960_e41333 * locals.var_mu_res__blk510_dn4)) * locals.var_edri__blk559) + (assign35960_e41335 * locals.var_edri__blk559_dn4)), (((((locals.var_weff_nf * (-locals.var_qn_res0_dn5)) * locals.var_mu_res__blk510) + (assign35960_e41333 * locals.var_mu_res__blk510_dn5)) * locals.var_edri__blk559) + (assign35960_e41335 * locals.var_edri__blk559_dn5)), (((((locals.var_weff_nf * (-locals.var_qn_res0_dn6)) * locals.var_mu_res__blk510) + (assign35960_e41333 * locals.var_mu_res__blk510_dn6)) * locals.var_edri__blk559) + (assign35960_e41335 * locals.var_edri__blk559_dn6)), (((((locals.var_weff_nf * (-locals.var_qn_res0_dn7)) * locals.var_mu_res__blk510) + (assign35960_e41333 * locals.var_mu_res__blk510_dn7)) * locals.var_edri__blk559) + (assign35960_e41335 * locals.var_edri__blk559_dn7)), (((((locals.var_weff_nf * (-locals.var_qn_res0_dn8)) * locals.var_mu_res__blk510) + (assign35960_e41333 * locals.var_mu_res__blk510_dn8)) * locals.var_edri__blk559) + (assign35960_e41335 * locals.var_edri__blk559_dn8)), (((((locals.var_weff_nf * (-locals.var_qn_res0_dn9)) * locals.var_mu_res__blk510) + (assign35960_e41333 * locals.var_mu_res__blk510_dn9)) * locals.var_edri__blk559) + (assign35960_e41335 * locals.var_edri__blk559_dn9)), (((((locals.var_weff_nf * (-locals.var_qn_res0_dn10)) * locals.var_mu_res__blk510) + (assign35960_e41333 * locals.var_mu_res__blk510_dn10)) * locals.var_edri__blk559) + (assign35960_e41335 * locals.var_edri__blk559_dn10)), (((((locals.var_weff_nf * (-locals.var_qn_res0_dn11)) * locals.var_mu_res__blk510) + (assign35960_e41333 * locals.var_mu_res__blk510_dn11)) * locals.var_edri__blk559) + (assign35960_e41335 * locals.var_edri__blk559_dn11)), (((((locals.var_weff_nf * (-locals.var_qn_res0_dn14)) * locals.var_mu_res__blk510) + (assign35960_e41333 * locals.var_mu_res__blk510_dn14)) * locals.var_edri__blk559) + (assign35960_e41335 * locals.var_edri__blk559_dn14)),)
    } else {
        (locals.var_ids_res, locals.var_ids_res_dn0, locals.var_ids_res_dn2, locals.var_ids_res_dn4, locals.var_ids_res_dn5, locals.var_ids_res_dn6, locals.var_ids_res_dn7, locals.var_ids_res_dn8, locals.var_ids_res_dn9, locals.var_ids_res_dn10, locals.var_ids_res_dn11, locals.var_ids_res_dn14,)
    }
};
        locals.var_ids_res = assign35960_e41339;
        locals.var_ids_res_dn0 = assign35960_e41339_d_n0;
        locals.var_ids_res_dn2 = assign35960_e41339_d_n2;
        locals.var_ids_res_dn4 = assign35960_e41339_d_n4;
        locals.var_ids_res_dn5 = assign35960_e41339_d_n5;
        locals.var_ids_res_dn6 = assign35960_e41339_d_n6;
        locals.var_ids_res_dn7 = assign35960_e41339_d_n7;
        locals.var_ids_res_dn8 = assign35960_e41339_d_n8;
        locals.var_ids_res_dn9 = assign35960_e41339_d_n9;
        locals.var_ids_res_dn10 = assign35960_e41339_d_n10;
        locals.var_ids_res_dn11 = assign35960_e41339_d_n11;
        locals.var_ids_res_dn14 = assign35960_e41339_d_n14;
        locals.var_ids_res_rv = 0.0;

        let (assign35970_e41351, assign35970_e41351_d_n0, assign35970_e41351_d_n2, assign35970_e41351_d_n4, assign35970_e41351_d_n5, assign35970_e41351_d_n6, assign35970_e41351_d_n7, assign35970_e41351_d_n8, assign35970_e41351_d_n9, assign35970_e41351_d_n10, assign35970_e41351_d_n11, assign35970_e41351_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign35970_e41346: f64 = (locals.var_phi_sl_dep - locals.var_phi_s0_dep);
        let assign35970_e41348: f64 = (assign35970_e41346 * locals.var_ninvde);
        let assign35970_e41349: f64 = (1.0 + assign35970_e41348);
        (assign35970_e41349, (((locals.var_phi_sl_dep_dn0 - locals.var_phi_s0_dep_dn0) * locals.var_ninvde) + (assign35970_e41346 * locals.var_ninvde_dn0)), (((locals.var_phi_sl_dep_dn2 - locals.var_phi_s0_dep_dn2) * locals.var_ninvde) + (assign35970_e41346 * locals.var_ninvde_dn2)), (((locals.var_phi_sl_dep_dn4 - locals.var_phi_s0_dep_dn4) * locals.var_ninvde) + (assign35970_e41346 * locals.var_ninvde_dn4)), (((locals.var_phi_sl_dep_dn5 - locals.var_phi_s0_dep_dn5) * locals.var_ninvde) + (assign35970_e41346 * locals.var_ninvde_dn5)), (((locals.var_phi_sl_dep_dn6 - locals.var_phi_s0_dep_dn6) * locals.var_ninvde) + (assign35970_e41346 * locals.var_ninvde_dn6)), (((locals.var_phi_sl_dep_dn7 - locals.var_phi_s0_dep_dn7) * locals.var_ninvde) + (assign35970_e41346 * locals.var_ninvde_dn7)), (((locals.var_phi_sl_dep_dn8 - locals.var_phi_s0_dep_dn8) * locals.var_ninvde) + (assign35970_e41346 * locals.var_ninvde_dn8)), (((locals.var_phi_sl_dep_dn9 - locals.var_phi_s0_dep_dn9) * locals.var_ninvde) + (assign35970_e41346 * locals.var_ninvde_dn9)), (((locals.var_phi_sl_dep_dn10 - locals.var_phi_s0_dep_dn10) * locals.var_ninvde) + (assign35970_e41346 * locals.var_ninvde_dn10)), (((locals.var_phi_sl_dep_dn11 - locals.var_phi_s0_dep_dn11) * locals.var_ninvde) + (assign35970_e41346 * locals.var_ninvde_dn11)), (((locals.var_phi_sl_dep_dn14 - locals.var_phi_s0_dep_dn14) * locals.var_ninvde) + (assign35970_e41346 * locals.var_ninvde_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign35970_e41351;
        locals.var_t4_dn0 = assign35970_e41351_d_n0;
        locals.var_t4_dn2 = assign35970_e41351_d_n2;
        locals.var_t4_dn4 = assign35970_e41351_d_n4;
        locals.var_t4_dn5 = assign35970_e41351_d_n5;
        locals.var_t4_dn6 = assign35970_e41351_d_n6;
        locals.var_t4_dn7 = assign35970_e41351_d_n7;
        locals.var_t4_dn8 = assign35970_e41351_d_n8;
        locals.var_t4_dn9 = assign35970_e41351_d_n9;
        locals.var_t4_dn10 = assign35970_e41351_d_n10;
        locals.var_t4_dn11 = assign35970_e41351_d_n11;
        locals.var_t4_dn14 = assign35970_e41351_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign35980_e41358, assign35980_e41358_d_n0, assign35980_e41358_d_n2, assign35980_e41358_d_n4, assign35980_e41358_d_n5, assign35980_e41358_d_n6, assign35980_e41358_d_n7, assign35980_e41358_d_n8, assign35980_e41358_d_n9, assign35980_e41358_d_n10, assign35980_e41358_d_n11, assign35980_e41358_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign35980_e41356: f64 = (-locals.var_qn_bac);
        (assign35980_e41356, (-locals.var_qn_bac_dn0), (-locals.var_qn_bac_dn2), (-locals.var_qn_bac_dn4), (-locals.var_qn_bac_dn5), (-locals.var_qn_bac_dn6), (-locals.var_qn_bac_dn7), (-locals.var_qn_bac_dn8), (-locals.var_qn_bac_dn9), (-locals.var_qn_bac_dn10), (-locals.var_qn_bac_dn11), (-locals.var_qn_bac_dn14),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn14,)
    }
};
        locals.var_qiu = assign35980_e41358;
        locals.var_qiu_dn0 = assign35980_e41358_d_n0;
        locals.var_qiu_dn2 = assign35980_e41358_d_n2;
        locals.var_qiu_dn4 = assign35980_e41358_d_n4;
        locals.var_qiu_dn5 = assign35980_e41358_d_n5;
        locals.var_qiu_dn6 = assign35980_e41358_d_n6;
        locals.var_qiu_dn7 = assign35980_e41358_d_n7;
        locals.var_qiu_dn8 = assign35980_e41358_d_n8;
        locals.var_qiu_dn9 = assign35980_e41358_d_n9;
        locals.var_qiu_dn10 = assign35980_e41358_d_n10;
        locals.var_qiu_dn11 = assign35980_e41358_d_n11;
        locals.var_qiu_dn14 = assign35980_e41358_d_n14;
        locals.var_qiu_rv = 0.0;

        let (assign35990_e41364, assign35990_e41364_d_n0, assign35990_e41364_d_n2, assign35990_e41364_d_n4, assign35990_e41364_d_n5, assign35990_e41364_d_n6, assign35990_e41364_d_n7, assign35990_e41364_d_n8, assign35990_e41364_d_n9, assign35990_e41364_d_n10, assign35990_e41364_d_n11, assign35990_e41364_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign35990_e41364;
        locals.var_t5_dn0 = assign35990_e41364_d_n0;
        locals.var_t5_dn2 = assign35990_e41364_d_n2;
        locals.var_t5_dn4 = assign35990_e41364_d_n4;
        locals.var_t5_dn5 = assign35990_e41364_d_n5;
        locals.var_t5_dn6 = assign35990_e41364_d_n6;
        locals.var_t5_dn7 = assign35990_e41364_d_n7;
        locals.var_t5_dn8 = assign35990_e41364_d_n8;
        locals.var_t5_dn9 = assign35990_e41364_d_n9;
        locals.var_t5_dn10 = assign35990_e41364_d_n10;
        locals.var_t5_dn11 = assign35990_e41364_d_n11;
        locals.var_t5_dn14 = assign35990_e41364_d_n14;
        locals.var_t5_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_118(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign36000_e41372, assign36000_e41372_d_n0, assign36000_e41372_d_n2, assign36000_e41372_d_n4, assign36000_e41372_d_n5, assign36000_e41372_d_n6, assign36000_e41372_d_n7, assign36000_e41372_d_n8, assign36000_e41372_d_n9, assign36000_e41372_d_n10, assign36000_e41372_d_n11, assign36000_e41372_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign36000_e41370: f64 = (locals.var_t5 / locals.var_t4);
        (assign36000_e41370, (((locals.var_t5_dn0 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn2 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn4 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn5 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn6 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn7 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn8 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn9 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn10 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn11 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn14 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign36000_e41372;
        locals.var_t3_dn0 = assign36000_e41372_d_n0;
        locals.var_t3_dn2 = assign36000_e41372_d_n2;
        locals.var_t3_dn4 = assign36000_e41372_d_n4;
        locals.var_t3_dn5 = assign36000_e41372_d_n5;
        locals.var_t3_dn6 = assign36000_e41372_d_n6;
        locals.var_t3_dn7 = assign36000_e41372_d_n7;
        locals.var_t3_dn8 = assign36000_e41372_d_n8;
        locals.var_t3_dn9 = assign36000_e41372_d_n9;
        locals.var_t3_dn10 = assign36000_e41372_d_n10;
        locals.var_t3_dn11 = assign36000_e41372_d_n11;
        locals.var_t3_dn14 = assign36000_e41372_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign36010_e41378, assign36010_e41378_d_n0, assign36010_e41378_d_n2, assign36010_e41378_d_n4, assign36010_e41378_d_n5, assign36010_e41378_d_n6, assign36010_e41378_d_n7, assign36010_e41378_d_n8, assign36010_e41378_d_n9, assign36010_e41378_d_n10, assign36010_e41378_d_n11, assign36010_e41378_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    } else {
        (locals.var_eeff, locals.var_eeff_dn0, locals.var_eeff_dn2, locals.var_eeff_dn4, locals.var_eeff_dn5, locals.var_eeff_dn6, locals.var_eeff_dn7, locals.var_eeff_dn8, locals.var_eeff_dn9, locals.var_eeff_dn10, locals.var_eeff_dn11, locals.var_eeff_dn14,)
    }
};
        locals.var_eeff = assign36010_e41378;
        locals.var_eeff_dn0 = assign36010_e41378_d_n0;
        locals.var_eeff_dn2 = assign36010_e41378_d_n2;
        locals.var_eeff_dn4 = assign36010_e41378_d_n4;
        locals.var_eeff_dn5 = assign36010_e41378_d_n5;
        locals.var_eeff_dn6 = assign36010_e41378_d_n6;
        locals.var_eeff_dn7 = assign36010_e41378_d_n7;
        locals.var_eeff_dn8 = assign36010_e41378_d_n8;
        locals.var_eeff_dn9 = assign36010_e41378_d_n9;
        locals.var_eeff_dn10 = assign36010_e41378_d_n10;
        locals.var_eeff_dn11 = assign36010_e41378_d_n11;
        locals.var_eeff_dn14 = assign36010_e41378_d_n14;
        locals.var_eeff_rv = 0.0;

        let (assign36020_e41393, assign36020_e41393_d_n0, assign36020_e41393_d_n2, assign36020_e41393_d_n4, assign36020_e41393_d_n5, assign36020_e41393_d_n6, assign36020_e41393_d_n7, assign36020_e41393_d_n8, assign36020_e41393_d_n9, assign36020_e41393_d_n10, assign36020_e41393_d_n11, assign36020_e41393_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let (assign36020_e41391, assign36020_e41391_d_n0, assign36020_e41391_d_n2, assign36020_e41391_d_n4, assign36020_e41391_d_n5, assign36020_e41391_d_n6, assign36020_e41391_d_n7, assign36020_e41391_d_n8, assign36020_e41391_d_n9, assign36020_e41391_d_n10, assign36020_e41391_d_n11, assign36020_e41391_d_n14,) = {
            if (locals.var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign36020_e41389: f64 = (p.p376 - 1.0);
                let assign36020_e41390: f64 = (locals.var_eeff).powf(assign36020_e41389);
                (assign36020_e41390, if 0.0 == 0.0 && ((assign36020_e41389) as f64).is_finite() && ((assign36020_e41389) as f64).fract() == 0.0 { if assign36020_e41389 == 0.0 { 0.0 } else { (assign36020_e41389 * ((locals.var_eeff).powf(assign36020_e41389 - 1.0) * locals.var_eeff_dn0)) } } else { (assign36020_e41390 * (assign36020_e41389 * (locals.var_eeff_dn0 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign36020_e41389) as f64).is_finite() && ((assign36020_e41389) as f64).fract() == 0.0 { if assign36020_e41389 == 0.0 { 0.0 } else { (assign36020_e41389 * ((locals.var_eeff).powf(assign36020_e41389 - 1.0) * locals.var_eeff_dn2)) } } else { (assign36020_e41390 * (assign36020_e41389 * (locals.var_eeff_dn2 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign36020_e41389) as f64).is_finite() && ((assign36020_e41389) as f64).fract() == 0.0 { if assign36020_e41389 == 0.0 { 0.0 } else { (assign36020_e41389 * ((locals.var_eeff).powf(assign36020_e41389 - 1.0) * locals.var_eeff_dn4)) } } else { (assign36020_e41390 * (assign36020_e41389 * (locals.var_eeff_dn4 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign36020_e41389) as f64).is_finite() && ((assign36020_e41389) as f64).fract() == 0.0 { if assign36020_e41389 == 0.0 { 0.0 } else { (assign36020_e41389 * ((locals.var_eeff).powf(assign36020_e41389 - 1.0) * locals.var_eeff_dn5)) } } else { (assign36020_e41390 * (assign36020_e41389 * (locals.var_eeff_dn5 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign36020_e41389) as f64).is_finite() && ((assign36020_e41389) as f64).fract() == 0.0 { if assign36020_e41389 == 0.0 { 0.0 } else { (assign36020_e41389 * ((locals.var_eeff).powf(assign36020_e41389 - 1.0) * locals.var_eeff_dn6)) } } else { (assign36020_e41390 * (assign36020_e41389 * (locals.var_eeff_dn6 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign36020_e41389) as f64).is_finite() && ((assign36020_e41389) as f64).fract() == 0.0 { if assign36020_e41389 == 0.0 { 0.0 } else { (assign36020_e41389 * ((locals.var_eeff).powf(assign36020_e41389 - 1.0) * locals.var_eeff_dn7)) } } else { (assign36020_e41390 * (assign36020_e41389 * (locals.var_eeff_dn7 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign36020_e41389) as f64).is_finite() && ((assign36020_e41389) as f64).fract() == 0.0 { if assign36020_e41389 == 0.0 { 0.0 } else { (assign36020_e41389 * ((locals.var_eeff).powf(assign36020_e41389 - 1.0) * locals.var_eeff_dn8)) } } else { (assign36020_e41390 * (assign36020_e41389 * (locals.var_eeff_dn8 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign36020_e41389) as f64).is_finite() && ((assign36020_e41389) as f64).fract() == 0.0 { if assign36020_e41389 == 0.0 { 0.0 } else { (assign36020_e41389 * ((locals.var_eeff).powf(assign36020_e41389 - 1.0) * locals.var_eeff_dn9)) } } else { (assign36020_e41390 * (assign36020_e41389 * (locals.var_eeff_dn9 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign36020_e41389) as f64).is_finite() && ((assign36020_e41389) as f64).fract() == 0.0 { if assign36020_e41389 == 0.0 { 0.0 } else { (assign36020_e41389 * ((locals.var_eeff).powf(assign36020_e41389 - 1.0) * locals.var_eeff_dn10)) } } else { (assign36020_e41390 * (assign36020_e41389 * (locals.var_eeff_dn10 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign36020_e41389) as f64).is_finite() && ((assign36020_e41389) as f64).fract() == 0.0 { if assign36020_e41389 == 0.0 { 0.0 } else { (assign36020_e41389 * ((locals.var_eeff).powf(assign36020_e41389 - 1.0) * locals.var_eeff_dn11)) } } else { (assign36020_e41390 * (assign36020_e41389 * (locals.var_eeff_dn11 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign36020_e41389) as f64).is_finite() && ((assign36020_e41389) as f64).fract() == 0.0 { if assign36020_e41389 == 0.0 { 0.0 } else { (assign36020_e41389 * ((locals.var_eeff).powf(assign36020_e41389 - 1.0) * locals.var_eeff_dn14)) } } else { (assign36020_e41390 * (assign36020_e41389 * (locals.var_eeff_dn14 / locals.var_eeff))) },)
            }
        };
        (assign36020_e41391, assign36020_e41391_d_n0, assign36020_e41391_d_n2, assign36020_e41391_d_n4, assign36020_e41391_d_n5, assign36020_e41391_d_n6, assign36020_e41391_d_n7, assign36020_e41391_d_n8, assign36020_e41391_d_n9, assign36020_e41391_d_n10, assign36020_e41391_d_n11, assign36020_e41391_d_n14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign36020_e41393;
        locals.var_t5_dn0 = assign36020_e41393_d_n0;
        locals.var_t5_dn2 = assign36020_e41393_d_n2;
        locals.var_t5_dn4 = assign36020_e41393_d_n4;
        locals.var_t5_dn5 = assign36020_e41393_d_n5;
        locals.var_t5_dn6 = assign36020_e41393_d_n6;
        locals.var_t5_dn7 = assign36020_e41393_d_n7;
        locals.var_t5_dn8 = assign36020_e41393_d_n8;
        locals.var_t5_dn9 = assign36020_e41393_d_n9;
        locals.var_t5_dn10 = assign36020_e41393_d_n10;
        locals.var_t5_dn11 = assign36020_e41393_d_n11;
        locals.var_t5_dn14 = assign36020_e41393_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign36030_e41401, assign36030_e41401_d_n0, assign36030_e41401_d_n2, assign36030_e41401_d_n4, assign36030_e41401_d_n5, assign36030_e41401_d_n6, assign36030_e41401_d_n7, assign36030_e41401_d_n8, assign36030_e41401_d_n9, assign36030_e41401_d_n10, assign36030_e41401_d_n11, assign36030_e41401_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign36030_e41399: f64 = (locals.var_t5 * locals.var_eeff);
        (assign36030_e41399, ((locals.var_t5_dn0 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn0)), ((locals.var_t5_dn2 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn2)), ((locals.var_t5_dn4 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn4)), ((locals.var_t5_dn5 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn5)), ((locals.var_t5_dn6 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn6)), ((locals.var_t5_dn7 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn7)), ((locals.var_t5_dn8 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn8)), ((locals.var_t5_dn9 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn9)), ((locals.var_t5_dn10 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn10)), ((locals.var_t5_dn11 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn11)), ((locals.var_t5_dn14 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn14)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign36030_e41401;
        locals.var_t8_dn0 = assign36030_e41401_d_n0;
        locals.var_t8_dn2 = assign36030_e41401_d_n2;
        locals.var_t8_dn4 = assign36030_e41401_d_n4;
        locals.var_t8_dn5 = assign36030_e41401_d_n5;
        locals.var_t8_dn6 = assign36030_e41401_d_n6;
        locals.var_t8_dn7 = assign36030_e41401_d_n7;
        locals.var_t8_dn8 = assign36030_e41401_d_n8;
        locals.var_t8_dn9 = assign36030_e41401_d_n9;
        locals.var_t8_dn10 = assign36030_e41401_d_n10;
        locals.var_t8_dn11 = assign36030_e41401_d_n11;
        locals.var_t8_dn14 = assign36030_e41401_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign36040_e41409, assign36040_e41409_d_n0, assign36040_e41409_d_n2, assign36040_e41409_d_n4, assign36040_e41409_d_n5, assign36040_e41409_d_n6, assign36040_e41409_d_n7, assign36040_e41409_d_n8, assign36040_e41409_d_n9, assign36040_e41409_d_n10, assign36040_e41409_d_n11, assign36040_e41409_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign36040_e41407: f64 = (1.6021918e-19 * 10000.0);
        (assign36040_e41407, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign36040_e41409;
        locals.var_t9_dn0 = assign36040_e41409_d_n0;
        locals.var_t9_dn2 = assign36040_e41409_d_n2;
        locals.var_t9_dn4 = assign36040_e41409_d_n4;
        locals.var_t9_dn5 = assign36040_e41409_d_n5;
        locals.var_t9_dn6 = assign36040_e41409_d_n6;
        locals.var_t9_dn7 = assign36040_e41409_d_n7;
        locals.var_t9_dn8 = assign36040_e41409_d_n8;
        locals.var_t9_dn9 = assign36040_e41409_d_n9;
        locals.var_t9_dn10 = assign36040_e41409_d_n10;
        locals.var_t9_dn11 = assign36040_e41409_d_n11;
        locals.var_t9_dn14 = assign36040_e41409_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign36050_e41417, assign36050_e41417_d_n0, assign36050_e41417_d_n2, assign36050_e41417_d_n4, assign36050_e41417_d_n5, assign36050_e41417_d_n6, assign36050_e41417_d_n7, assign36050_e41417_d_n8, assign36050_e41417_d_n9, assign36050_e41417_d_n10, assign36050_e41417_d_n11, assign36050_e41417_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign36050_e41415: f64 = (locals.var_qiu / locals.var_t9);
        (assign36050_e41415, (((locals.var_qiu_dn0 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn0)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn2 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn2)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn4 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn4)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn5 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn5)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn6 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn6)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn7 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn7)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn8 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn8)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn9 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn9)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn10 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn10)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn11 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn11)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn14 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn14)) / (locals.var_t9 * locals.var_t9)),)
    } else {
        (locals.var_rns, locals.var_rns_dn0, locals.var_rns_dn2, locals.var_rns_dn4, locals.var_rns_dn5, locals.var_rns_dn6, locals.var_rns_dn7, locals.var_rns_dn8, locals.var_rns_dn9, locals.var_rns_dn10, locals.var_rns_dn11, locals.var_rns_dn14,)
    }
};
        locals.var_rns = assign36050_e41417;
        locals.var_rns_dn0 = assign36050_e41417_d_n0;
        locals.var_rns_dn2 = assign36050_e41417_d_n2;
        locals.var_rns_dn4 = assign36050_e41417_d_n4;
        locals.var_rns_dn5 = assign36050_e41417_d_n5;
        locals.var_rns_dn6 = assign36050_e41417_d_n6;
        locals.var_rns_dn7 = assign36050_e41417_d_n7;
        locals.var_rns_dn8 = assign36050_e41417_d_n8;
        locals.var_rns_dn9 = assign36050_e41417_d_n9;
        locals.var_rns_dn10 = assign36050_e41417_d_n10;
        locals.var_rns_dn11 = assign36050_e41417_d_n11;
        locals.var_rns_dn14 = assign36050_e41417_d_n14;
        locals.var_rns_rv = 0.0;

        let (assign36060_e41437, assign36060_e41437_d_n0, assign36060_e41437_d_n2, assign36060_e41437_d_n4, assign36060_e41437_d_n5, assign36060_e41437_d_n6, assign36060_e41437_d_n7, assign36060_e41437_d_n8, assign36060_e41437_d_n9, assign36060_e41437_d_n10, assign36060_e41437_d_n11, assign36060_e41437_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign36060_e41425: f64 = (locals.var_uc_depmueback1 * locals.var_rns);
        let assign36060_e41427: f64 = (assign36060_e41425 / 100000000000.0);
        let assign36060_e41428: f64 = (locals.var_uc_depmueback0 + assign36060_e41427);
        let assign36060_e41430: f64 = (assign36060_e41428 + 1e-25);
        let assign36060_e41431: f64 = (1.0 / assign36060_e41430);
        let assign36060_e41434: f64 = (locals.var_depmphn0 * locals.var_t8);
        let assign36060_e41435: f64 = (assign36060_e41431 + assign36060_e41434);
        (assign36060_e41435, ((-((locals.var_uc_depmueback0_dn0 + (((locals.var_uc_depmueback1_dn0 * locals.var_rns) + (locals.var_uc_depmueback1 * locals.var_rns_dn0)) / 100000000000.0)) / (assign36060_e41430 * assign36060_e41430))) + ((locals.var_depmphn0_dn0 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn0))), ((-((locals.var_uc_depmueback0_dn2 + (((locals.var_uc_depmueback1_dn2 * locals.var_rns) + (locals.var_uc_depmueback1 * locals.var_rns_dn2)) / 100000000000.0)) / (assign36060_e41430 * assign36060_e41430))) + ((locals.var_depmphn0_dn2 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn2))), ((-((locals.var_uc_depmueback0_dn4 + (((locals.var_uc_depmueback1_dn4 * locals.var_rns) + (locals.var_uc_depmueback1 * locals.var_rns_dn4)) / 100000000000.0)) / (assign36060_e41430 * assign36060_e41430))) + ((locals.var_depmphn0_dn4 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn4))), ((-((locals.var_uc_depmueback0_dn5 + (((locals.var_uc_depmueback1_dn5 * locals.var_rns) + (locals.var_uc_depmueback1 * locals.var_rns_dn5)) / 100000000000.0)) / (assign36060_e41430 * assign36060_e41430))) + ((locals.var_depmphn0_dn5 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn5))), ((-((locals.var_uc_depmueback0_dn6 + (((locals.var_uc_depmueback1_dn6 * locals.var_rns) + (locals.var_uc_depmueback1 * locals.var_rns_dn6)) / 100000000000.0)) / (assign36060_e41430 * assign36060_e41430))) + ((locals.var_depmphn0_dn6 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn6))), ((-((locals.var_uc_depmueback0_dn7 + (((locals.var_uc_depmueback1_dn7 * locals.var_rns) + (locals.var_uc_depmueback1 * locals.var_rns_dn7)) / 100000000000.0)) / (assign36060_e41430 * assign36060_e41430))) + ((locals.var_depmphn0_dn7 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn7))), ((-((locals.var_uc_depmueback0_dn8 + (((locals.var_uc_depmueback1_dn8 * locals.var_rns) + (locals.var_uc_depmueback1 * locals.var_rns_dn8)) / 100000000000.0)) / (assign36060_e41430 * assign36060_e41430))) + ((locals.var_depmphn0_dn8 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn8))), ((-((locals.var_uc_depmueback0_dn9 + (((locals.var_uc_depmueback1_dn9 * locals.var_rns) + (locals.var_uc_depmueback1 * locals.var_rns_dn9)) / 100000000000.0)) / (assign36060_e41430 * assign36060_e41430))) + ((locals.var_depmphn0_dn9 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn9))), ((-((locals.var_uc_depmueback0_dn10 + (((locals.var_uc_depmueback1_dn10 * locals.var_rns) + (locals.var_uc_depmueback1 * locals.var_rns_dn10)) / 100000000000.0)) / (assign36060_e41430 * assign36060_e41430))) + ((locals.var_depmphn0_dn10 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn10))), ((-((locals.var_uc_depmueback0_dn11 + (((locals.var_uc_depmueback1_dn11 * locals.var_rns) + (locals.var_uc_depmueback1 * locals.var_rns_dn11)) / 100000000000.0)) / (assign36060_e41430 * assign36060_e41430))) + ((locals.var_depmphn0_dn11 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn11))), ((-((locals.var_uc_depmueback0_dn14 + (((locals.var_uc_depmueback1_dn14 * locals.var_rns) + (locals.var_uc_depmueback1 * locals.var_rns_dn14)) / 100000000000.0)) / (assign36060_e41430 * assign36060_e41430))) + ((locals.var_depmphn0_dn14 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign36060_e41437;
        locals.var_t1_dn0 = assign36060_e41437_d_n0;
        locals.var_t1_dn2 = assign36060_e41437_d_n2;
        locals.var_t1_dn4 = assign36060_e41437_d_n4;
        locals.var_t1_dn5 = assign36060_e41437_d_n5;
        locals.var_t1_dn6 = assign36060_e41437_d_n6;
        locals.var_t1_dn7 = assign36060_e41437_d_n7;
        locals.var_t1_dn8 = assign36060_e41437_d_n8;
        locals.var_t1_dn9 = assign36060_e41437_d_n9;
        locals.var_t1_dn10 = assign36060_e41437_d_n10;
        locals.var_t1_dn11 = assign36060_e41437_d_n11;
        locals.var_t1_dn14 = assign36060_e41437_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign36070_e41445, assign36070_e41445_d_n0, assign36070_e41445_d_n2, assign36070_e41445_d_n4, assign36070_e41445_d_n5, assign36070_e41445_d_n6, assign36070_e41445_d_n7, assign36070_e41445_d_n8, assign36070_e41445_d_n9, assign36070_e41445_d_n10, assign36070_e41445_d_n11, assign36070_e41445_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign36070_e41443: f64 = (1.0 / locals.var_t1);
        (assign36070_e41443, (-(locals.var_t1_dn0 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn2 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn11 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn14 / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn14,)
    }
};
        locals.var_muun = assign36070_e41445;
        locals.var_muun_dn0 = assign36070_e41445_d_n0;
        locals.var_muun_dn2 = assign36070_e41445_d_n2;
        locals.var_muun_dn4 = assign36070_e41445_d_n4;
        locals.var_muun_dn5 = assign36070_e41445_d_n5;
        locals.var_muun_dn6 = assign36070_e41445_d_n6;
        locals.var_muun_dn7 = assign36070_e41445_d_n7;
        locals.var_muun_dn8 = assign36070_e41445_d_n8;
        locals.var_muun_dn9 = assign36070_e41445_d_n9;
        locals.var_muun_dn10 = assign36070_e41445_d_n10;
        locals.var_muun_dn11 = assign36070_e41445_d_n11;
        locals.var_muun_dn14 = assign36070_e41445_d_n14;
        locals.var_muun_rv = 0.0;

        let (assign36080_e41453, assign36080_e41453_d_n0, assign36080_e41453_d_n2, assign36080_e41453_d_n4, assign36080_e41453_d_n5, assign36080_e41453_d_n6, assign36080_e41453_d_n7, assign36080_e41453_d_n8, assign36080_e41453_d_n9, assign36080_e41453_d_n10, assign36080_e41453_d_n11, assign36080_e41453_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign36080_e41451: f64 = (locals.var_muun / 10000.0);
        (assign36080_e41451, (locals.var_muun_dn0 / 10000.0), (locals.var_muun_dn2 / 10000.0), (locals.var_muun_dn4 / 10000.0), (locals.var_muun_dn5 / 10000.0), (locals.var_muun_dn6 / 10000.0), (locals.var_muun_dn7 / 10000.0), (locals.var_muun_dn8 / 10000.0), (locals.var_muun_dn9 / 10000.0), (locals.var_muun_dn10 / 10000.0), (locals.var_muun_dn11 / 10000.0), (locals.var_muun_dn14 / 10000.0),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn14,)
    }
};
        locals.var_muun = assign36080_e41453;
        locals.var_muun_dn0 = assign36080_e41453_d_n0;
        locals.var_muun_dn2 = assign36080_e41453_d_n2;
        locals.var_muun_dn4 = assign36080_e41453_d_n4;
        locals.var_muun_dn5 = assign36080_e41453_d_n5;
        locals.var_muun_dn6 = assign36080_e41453_d_n6;
        locals.var_muun_dn7 = assign36080_e41453_d_n7;
        locals.var_muun_dn8 = assign36080_e41453_d_n8;
        locals.var_muun_dn9 = assign36080_e41453_d_n9;
        locals.var_muun_dn10 = assign36080_e41453_d_n10;
        locals.var_muun_dn11 = assign36080_e41453_d_n11;
        locals.var_muun_dn14 = assign36080_e41453_d_n14;
        locals.var_muun_rv = 0.0;

        let (assign36090_e41461, assign36090_e41461_d_n0, assign36090_e41461_d_n2, assign36090_e41461_d_n4, assign36090_e41461_d_n5, assign36090_e41461_d_n6, assign36090_e41461_d_n7, assign36090_e41461_d_n8, assign36090_e41461_d_n9, assign36090_e41461_d_n10, assign36090_e41461_d_n11, assign36090_e41461_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign36090_e41459: f64 = (locals.var_vdseff0 / locals.var_lch);
        (assign36090_e41459, (((locals.var_vdseff0_dn0 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn2 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn4 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn5 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn6 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn7 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn8 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn9 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn10 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn11 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn14 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn14)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_edri__blk559, locals.var_edri__blk559_dn0, locals.var_edri__blk559_dn2, locals.var_edri__blk559_dn4, locals.var_edri__blk559_dn5, locals.var_edri__blk559_dn6, locals.var_edri__blk559_dn7, locals.var_edri__blk559_dn8, locals.var_edri__blk559_dn9, locals.var_edri__blk559_dn10, locals.var_edri__blk559_dn11, locals.var_edri__blk559_dn14,)
    }
};
        locals.var_edri__blk559 = assign36090_e41461;
        locals.var_edri__blk559_dn0 = assign36090_e41461_d_n0;
        locals.var_edri__blk559_dn2 = assign36090_e41461_d_n2;
        locals.var_edri__blk559_dn4 = assign36090_e41461_d_n4;
        locals.var_edri__blk559_dn5 = assign36090_e41461_d_n5;
        locals.var_edri__blk559_dn6 = assign36090_e41461_d_n6;
        locals.var_edri__blk559_dn7 = assign36090_e41461_d_n7;
        locals.var_edri__blk559_dn8 = assign36090_e41461_d_n8;
        locals.var_edri__blk559_dn9 = assign36090_e41461_d_n9;
        locals.var_edri__blk559_dn10 = assign36090_e41461_d_n10;
        locals.var_edri__blk559_dn11 = assign36090_e41461_d_n11;
        locals.var_edri__blk559_dn14 = assign36090_e41461_d_n14;
        locals.var_edri__blk559_rv = 0.0;

        let (assign36100_e41471, assign36100_e41471_d_n0, assign36100_e41471_d_n2, assign36100_e41471_d_n4, assign36100_e41471_d_n5, assign36100_e41471_d_n6, assign36100_e41471_d_n7, assign36100_e41471_d_n8, assign36100_e41471_d_n9, assign36100_e41471_d_n10, assign36100_e41471_d_n11, assign36100_e41471_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign36100_e41467: f64 = (locals.var_muun * locals.var_edri__blk559);
        let assign36100_e41469: f64 = (assign36100_e41467 / locals.var_uc_depvmax);
        (assign36100_e41469, (((((locals.var_muun_dn0 * locals.var_edri__blk559) + (locals.var_muun * locals.var_edri__blk559_dn0)) * locals.var_uc_depvmax) - (assign36100_e41467 * locals.var_uc_depvmax_dn0)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn2 * locals.var_edri__blk559) + (locals.var_muun * locals.var_edri__blk559_dn2)) * locals.var_uc_depvmax) - (assign36100_e41467 * locals.var_uc_depvmax_dn2)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn4 * locals.var_edri__blk559) + (locals.var_muun * locals.var_edri__blk559_dn4)) * locals.var_uc_depvmax) - (assign36100_e41467 * locals.var_uc_depvmax_dn4)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn5 * locals.var_edri__blk559) + (locals.var_muun * locals.var_edri__blk559_dn5)) * locals.var_uc_depvmax) - (assign36100_e41467 * locals.var_uc_depvmax_dn5)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn6 * locals.var_edri__blk559) + (locals.var_muun * locals.var_edri__blk559_dn6)) * locals.var_uc_depvmax) - (assign36100_e41467 * locals.var_uc_depvmax_dn6)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn7 * locals.var_edri__blk559) + (locals.var_muun * locals.var_edri__blk559_dn7)) * locals.var_uc_depvmax) - (assign36100_e41467 * locals.var_uc_depvmax_dn7)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn8 * locals.var_edri__blk559) + (locals.var_muun * locals.var_edri__blk559_dn8)) * locals.var_uc_depvmax) - (assign36100_e41467 * locals.var_uc_depvmax_dn8)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn9 * locals.var_edri__blk559) + (locals.var_muun * locals.var_edri__blk559_dn9)) * locals.var_uc_depvmax) - (assign36100_e41467 * locals.var_uc_depvmax_dn9)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn10 * locals.var_edri__blk559) + (locals.var_muun * locals.var_edri__blk559_dn10)) * locals.var_uc_depvmax) - (assign36100_e41467 * locals.var_uc_depvmax_dn10)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn11 * locals.var_edri__blk559) + (locals.var_muun * locals.var_edri__blk559_dn11)) * locals.var_uc_depvmax) - (assign36100_e41467 * locals.var_uc_depvmax_dn11)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn14 * locals.var_edri__blk559) + (locals.var_muun * locals.var_edri__blk559_dn14)) * locals.var_uc_depvmax) - (assign36100_e41467 * locals.var_uc_depvmax_dn14)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign36100_e41471;
        locals.var_t1_dn0 = assign36100_e41471_d_n0;
        locals.var_t1_dn2 = assign36100_e41471_d_n2;
        locals.var_t1_dn4 = assign36100_e41471_d_n4;
        locals.var_t1_dn5 = assign36100_e41471_d_n5;
        locals.var_t1_dn6 = assign36100_e41471_d_n6;
        locals.var_t1_dn7 = assign36100_e41471_d_n7;
        locals.var_t1_dn8 = assign36100_e41471_d_n8;
        locals.var_t1_dn9 = assign36100_e41471_d_n9;
        locals.var_t1_dn10 = assign36100_e41471_d_n10;
        locals.var_t1_dn11 = assign36100_e41471_d_n11;
        locals.var_t1_dn14 = assign36100_e41471_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign36110_e41484, assign36110_e41484_d_n0, assign36110_e41484_d_n2, assign36110_e41484_d_n4, assign36110_e41484_d_n5, assign36110_e41484_d_n6, assign36110_e41484_d_n7, assign36110_e41484_d_n8, assign36110_e41484_d_n9, assign36110_e41484_d_n10, assign36110_e41484_d_n11, assign36110_e41484_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let (assign36110_e41482, assign36110_e41482_d_n0, assign36110_e41482_d_n2, assign36110_e41482_d_n4, assign36110_e41482_d_n5, assign36110_e41482_d_n6, assign36110_e41482_d_n7, assign36110_e41482_d_n8, assign36110_e41482_d_n9, assign36110_e41482_d_n10, assign36110_e41482_d_n11, assign36110_e41482_d_n14,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign36110_e41481: f64 = (locals.var_t1).powf(p.p378);
                (assign36110_e41481, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn0)) } } else { (assign36110_e41481 * (p.p378 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn2)) } } else { (assign36110_e41481 * (p.p378 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn4)) } } else { (assign36110_e41481 * (p.p378 * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn5)) } } else { (assign36110_e41481 * (p.p378 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn6)) } } else { (assign36110_e41481 * (p.p378 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn7)) } } else { (assign36110_e41481 * (p.p378 * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn8)) } } else { (assign36110_e41481 * (p.p378 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn9)) } } else { (assign36110_e41481 * (p.p378 * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn10)) } } else { (assign36110_e41481 * (p.p378 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn11)) } } else { (assign36110_e41481 * (p.p378 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn14)) } } else { (assign36110_e41481 * (p.p378 * (locals.var_t1_dn14 / locals.var_t1))) },)
            }
        };
        (assign36110_e41482, assign36110_e41482_d_n0, assign36110_e41482_d_n2, assign36110_e41482_d_n4, assign36110_e41482_d_n5, assign36110_e41482_d_n6, assign36110_e41482_d_n7, assign36110_e41482_d_n8, assign36110_e41482_d_n9, assign36110_e41482_d_n10, assign36110_e41482_d_n11, assign36110_e41482_d_n14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign36110_e41484;
        locals.var_t2_dn0 = assign36110_e41484_d_n0;
        locals.var_t2_dn2 = assign36110_e41484_d_n2;
        locals.var_t2_dn4 = assign36110_e41484_d_n4;
        locals.var_t2_dn5 = assign36110_e41484_d_n5;
        locals.var_t2_dn6 = assign36110_e41484_d_n6;
        locals.var_t2_dn7 = assign36110_e41484_d_n7;
        locals.var_t2_dn8 = assign36110_e41484_d_n8;
        locals.var_t2_dn9 = assign36110_e41484_d_n9;
        locals.var_t2_dn10 = assign36110_e41484_d_n10;
        locals.var_t2_dn11 = assign36110_e41484_d_n11;
        locals.var_t2_dn14 = assign36110_e41484_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign36120_e41492, assign36120_e41492_d_n0, assign36120_e41492_d_n2, assign36120_e41492_d_n4, assign36120_e41492_d_n5, assign36120_e41492_d_n6, assign36120_e41492_d_n7, assign36120_e41492_d_n8, assign36120_e41492_d_n9, assign36120_e41492_d_n10, assign36120_e41492_d_n11, assign36120_e41492_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign36120_e41490: f64 = (1.0 + locals.var_t2);
        (assign36120_e41490, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign36120_e41492;
        locals.var_t3_dn0 = assign36120_e41492_d_n0;
        locals.var_t3_dn2 = assign36120_e41492_d_n2;
        locals.var_t3_dn4 = assign36120_e41492_d_n4;
        locals.var_t3_dn5 = assign36120_e41492_d_n5;
        locals.var_t3_dn6 = assign36120_e41492_d_n6;
        locals.var_t3_dn7 = assign36120_e41492_d_n7;
        locals.var_t3_dn8 = assign36120_e41492_d_n8;
        locals.var_t3_dn9 = assign36120_e41492_d_n9;
        locals.var_t3_dn10 = assign36120_e41492_d_n10;
        locals.var_t3_dn11 = assign36120_e41492_d_n11;
        locals.var_t3_dn14 = assign36120_e41492_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign36130_e41507, assign36130_e41507_d_n0, assign36130_e41507_d_n2, assign36130_e41507_d_n4, assign36130_e41507_d_n5, assign36130_e41507_d_n6, assign36130_e41507_d_n7, assign36130_e41507_d_n8, assign36130_e41507_d_n9, assign36130_e41507_d_n10, assign36130_e41507_d_n11, assign36130_e41507_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let (assign36130_e41505, assign36130_e41505_d_n0, assign36130_e41505_d_n2, assign36130_e41505_d_n4, assign36130_e41505_d_n5, assign36130_e41505_d_n6, assign36130_e41505_d_n7, assign36130_e41505_d_n8, assign36130_e41505_d_n9, assign36130_e41505_d_n10, assign36130_e41505_d_n11, assign36130_e41505_d_n14,) = {
            if (locals.var_t3 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign36130_e41503: f64 = (1.0 / p.p378);
                let assign36130_e41504: f64 = (locals.var_t3).powf(assign36130_e41503);
                (assign36130_e41504, if 0.0 == 0.0 && ((assign36130_e41503) as f64).is_finite() && ((assign36130_e41503) as f64).fract() == 0.0 { if assign36130_e41503 == 0.0 { 0.0 } else { (assign36130_e41503 * ((locals.var_t3).powf(assign36130_e41503 - 1.0) * locals.var_t3_dn0)) } } else { (assign36130_e41504 * (assign36130_e41503 * (locals.var_t3_dn0 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign36130_e41503) as f64).is_finite() && ((assign36130_e41503) as f64).fract() == 0.0 { if assign36130_e41503 == 0.0 { 0.0 } else { (assign36130_e41503 * ((locals.var_t3).powf(assign36130_e41503 - 1.0) * locals.var_t3_dn2)) } } else { (assign36130_e41504 * (assign36130_e41503 * (locals.var_t3_dn2 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign36130_e41503) as f64).is_finite() && ((assign36130_e41503) as f64).fract() == 0.0 { if assign36130_e41503 == 0.0 { 0.0 } else { (assign36130_e41503 * ((locals.var_t3).powf(assign36130_e41503 - 1.0) * locals.var_t3_dn4)) } } else { (assign36130_e41504 * (assign36130_e41503 * (locals.var_t3_dn4 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign36130_e41503) as f64).is_finite() && ((assign36130_e41503) as f64).fract() == 0.0 { if assign36130_e41503 == 0.0 { 0.0 } else { (assign36130_e41503 * ((locals.var_t3).powf(assign36130_e41503 - 1.0) * locals.var_t3_dn5)) } } else { (assign36130_e41504 * (assign36130_e41503 * (locals.var_t3_dn5 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign36130_e41503) as f64).is_finite() && ((assign36130_e41503) as f64).fract() == 0.0 { if assign36130_e41503 == 0.0 { 0.0 } else { (assign36130_e41503 * ((locals.var_t3).powf(assign36130_e41503 - 1.0) * locals.var_t3_dn6)) } } else { (assign36130_e41504 * (assign36130_e41503 * (locals.var_t3_dn6 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign36130_e41503) as f64).is_finite() && ((assign36130_e41503) as f64).fract() == 0.0 { if assign36130_e41503 == 0.0 { 0.0 } else { (assign36130_e41503 * ((locals.var_t3).powf(assign36130_e41503 - 1.0) * locals.var_t3_dn7)) } } else { (assign36130_e41504 * (assign36130_e41503 * (locals.var_t3_dn7 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign36130_e41503) as f64).is_finite() && ((assign36130_e41503) as f64).fract() == 0.0 { if assign36130_e41503 == 0.0 { 0.0 } else { (assign36130_e41503 * ((locals.var_t3).powf(assign36130_e41503 - 1.0) * locals.var_t3_dn8)) } } else { (assign36130_e41504 * (assign36130_e41503 * (locals.var_t3_dn8 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign36130_e41503) as f64).is_finite() && ((assign36130_e41503) as f64).fract() == 0.0 { if assign36130_e41503 == 0.0 { 0.0 } else { (assign36130_e41503 * ((locals.var_t3).powf(assign36130_e41503 - 1.0) * locals.var_t3_dn9)) } } else { (assign36130_e41504 * (assign36130_e41503 * (locals.var_t3_dn9 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign36130_e41503) as f64).is_finite() && ((assign36130_e41503) as f64).fract() == 0.0 { if assign36130_e41503 == 0.0 { 0.0 } else { (assign36130_e41503 * ((locals.var_t3).powf(assign36130_e41503 - 1.0) * locals.var_t3_dn10)) } } else { (assign36130_e41504 * (assign36130_e41503 * (locals.var_t3_dn10 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign36130_e41503) as f64).is_finite() && ((assign36130_e41503) as f64).fract() == 0.0 { if assign36130_e41503 == 0.0 { 0.0 } else { (assign36130_e41503 * ((locals.var_t3).powf(assign36130_e41503 - 1.0) * locals.var_t3_dn11)) } } else { (assign36130_e41504 * (assign36130_e41503 * (locals.var_t3_dn11 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign36130_e41503) as f64).is_finite() && ((assign36130_e41503) as f64).fract() == 0.0 { if assign36130_e41503 == 0.0 { 0.0 } else { (assign36130_e41503 * ((locals.var_t3).powf(assign36130_e41503 - 1.0) * locals.var_t3_dn14)) } } else { (assign36130_e41504 * (assign36130_e41503 * (locals.var_t3_dn14 / locals.var_t3))) },)
            }
        };
        (assign36130_e41505, assign36130_e41505_d_n0, assign36130_e41505_d_n2, assign36130_e41505_d_n4, assign36130_e41505_d_n5, assign36130_e41505_d_n6, assign36130_e41505_d_n7, assign36130_e41505_d_n8, assign36130_e41505_d_n9, assign36130_e41505_d_n10, assign36130_e41505_d_n11, assign36130_e41505_d_n14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign36130_e41507;
        locals.var_t4_dn0 = assign36130_e41507_d_n0;
        locals.var_t4_dn2 = assign36130_e41507_d_n2;
        locals.var_t4_dn4 = assign36130_e41507_d_n4;
        locals.var_t4_dn5 = assign36130_e41507_d_n5;
        locals.var_t4_dn6 = assign36130_e41507_d_n6;
        locals.var_t4_dn7 = assign36130_e41507_d_n7;
        locals.var_t4_dn8 = assign36130_e41507_d_n8;
        locals.var_t4_dn9 = assign36130_e41507_d_n9;
        locals.var_t4_dn10 = assign36130_e41507_d_n10;
        locals.var_t4_dn11 = assign36130_e41507_d_n11;
        locals.var_t4_dn14 = assign36130_e41507_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign36140_e41515, assign36140_e41515_d_n0, assign36140_e41515_d_n2, assign36140_e41515_d_n4, assign36140_e41515_d_n5, assign36140_e41515_d_n6, assign36140_e41515_d_n7, assign36140_e41515_d_n8, assign36140_e41515_d_n9, assign36140_e41515_d_n10, assign36140_e41515_d_n11, assign36140_e41515_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign36140_e41513: f64 = (locals.var_muun / locals.var_t4);
        (assign36140_e41513, (((locals.var_muun_dn0 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn2 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn4 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn5 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn6 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn7 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn8 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn9 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn10 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn11 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn14 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_mu_bac, locals.var_mu_bac_dn0, locals.var_mu_bac_dn2, locals.var_mu_bac_dn4, locals.var_mu_bac_dn5, locals.var_mu_bac_dn6, locals.var_mu_bac_dn7, locals.var_mu_bac_dn8, locals.var_mu_bac_dn9, locals.var_mu_bac_dn10, locals.var_mu_bac_dn11, locals.var_mu_bac_dn14,)
    }
};
        locals.var_mu_bac = assign36140_e41515;
        locals.var_mu_bac_dn0 = assign36140_e41515_d_n0;
        locals.var_mu_bac_dn2 = assign36140_e41515_d_n2;
        locals.var_mu_bac_dn4 = assign36140_e41515_d_n4;
        locals.var_mu_bac_dn5 = assign36140_e41515_d_n5;
        locals.var_mu_bac_dn6 = assign36140_e41515_d_n6;
        locals.var_mu_bac_dn7 = assign36140_e41515_d_n7;
        locals.var_mu_bac_dn8 = assign36140_e41515_d_n8;
        locals.var_mu_bac_dn9 = assign36140_e41515_d_n9;
        locals.var_mu_bac_dn10 = assign36140_e41515_d_n10;
        locals.var_mu_bac_dn11 = assign36140_e41515_d_n11;
        locals.var_mu_bac_dn14 = assign36140_e41515_d_n14;
        locals.var_mu_bac_rv = 0.0;

        let (assign36150_e41528, assign36150_e41528_d_n0, assign36150_e41528_d_n2, assign36150_e41528_d_n4, assign36150_e41528_d_n5, assign36150_e41528_d_n6, assign36150_e41528_d_n7, assign36150_e41528_d_n8, assign36150_e41528_d_n9, assign36150_e41528_d_n10, assign36150_e41528_d_n11, assign36150_e41528_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign36150_e41521: f64 = (-locals.var_qn_bac);
        let assign36150_e41522: f64 = (locals.var_weff_nf * assign36150_e41521);
        let assign36150_e41524: f64 = (assign36150_e41522 * locals.var_mu_bac);
        let assign36150_e41526: f64 = (assign36150_e41524 * locals.var_edri__blk559);
        (assign36150_e41526, (((((locals.var_weff_nf * (-locals.var_qn_bac_dn0)) * locals.var_mu_bac) + (assign36150_e41522 * locals.var_mu_bac_dn0)) * locals.var_edri__blk559) + (assign36150_e41524 * locals.var_edri__blk559_dn0)), (((((locals.var_weff_nf * (-locals.var_qn_bac_dn2)) * locals.var_mu_bac) + (assign36150_e41522 * locals.var_mu_bac_dn2)) * locals.var_edri__blk559) + (assign36150_e41524 * locals.var_edri__blk559_dn2)), (((((locals.var_weff_nf * (-locals.var_qn_bac_dn4)) * locals.var_mu_bac) + (assign36150_e41522 * locals.var_mu_bac_dn4)) * locals.var_edri__blk559) + (assign36150_e41524 * locals.var_edri__blk559_dn4)), (((((locals.var_weff_nf * (-locals.var_qn_bac_dn5)) * locals.var_mu_bac) + (assign36150_e41522 * locals.var_mu_bac_dn5)) * locals.var_edri__blk559) + (assign36150_e41524 * locals.var_edri__blk559_dn5)), (((((locals.var_weff_nf * (-locals.var_qn_bac_dn6)) * locals.var_mu_bac) + (assign36150_e41522 * locals.var_mu_bac_dn6)) * locals.var_edri__blk559) + (assign36150_e41524 * locals.var_edri__blk559_dn6)), (((((locals.var_weff_nf * (-locals.var_qn_bac_dn7)) * locals.var_mu_bac) + (assign36150_e41522 * locals.var_mu_bac_dn7)) * locals.var_edri__blk559) + (assign36150_e41524 * locals.var_edri__blk559_dn7)), (((((locals.var_weff_nf * (-locals.var_qn_bac_dn8)) * locals.var_mu_bac) + (assign36150_e41522 * locals.var_mu_bac_dn8)) * locals.var_edri__blk559) + (assign36150_e41524 * locals.var_edri__blk559_dn8)), (((((locals.var_weff_nf * (-locals.var_qn_bac_dn9)) * locals.var_mu_bac) + (assign36150_e41522 * locals.var_mu_bac_dn9)) * locals.var_edri__blk559) + (assign36150_e41524 * locals.var_edri__blk559_dn9)), (((((locals.var_weff_nf * (-locals.var_qn_bac_dn10)) * locals.var_mu_bac) + (assign36150_e41522 * locals.var_mu_bac_dn10)) * locals.var_edri__blk559) + (assign36150_e41524 * locals.var_edri__blk559_dn10)), (((((locals.var_weff_nf * (-locals.var_qn_bac_dn11)) * locals.var_mu_bac) + (assign36150_e41522 * locals.var_mu_bac_dn11)) * locals.var_edri__blk559) + (assign36150_e41524 * locals.var_edri__blk559_dn11)), (((((locals.var_weff_nf * (-locals.var_qn_bac_dn14)) * locals.var_mu_bac) + (assign36150_e41522 * locals.var_mu_bac_dn14)) * locals.var_edri__blk559) + (assign36150_e41524 * locals.var_edri__blk559_dn14)),)
    } else {
        (locals.var_ids_bac, locals.var_ids_bac_dn0, locals.var_ids_bac_dn2, locals.var_ids_bac_dn4, locals.var_ids_bac_dn5, locals.var_ids_bac_dn6, locals.var_ids_bac_dn7, locals.var_ids_bac_dn8, locals.var_ids_bac_dn9, locals.var_ids_bac_dn10, locals.var_ids_bac_dn11, locals.var_ids_bac_dn14,)
    }
};
        locals.var_ids_bac = assign36150_e41528;
        locals.var_ids_bac_dn0 = assign36150_e41528_d_n0;
        locals.var_ids_bac_dn2 = assign36150_e41528_d_n2;
        locals.var_ids_bac_dn4 = assign36150_e41528_d_n4;
        locals.var_ids_bac_dn5 = assign36150_e41528_d_n5;
        locals.var_ids_bac_dn6 = assign36150_e41528_d_n6;
        locals.var_ids_bac_dn7 = assign36150_e41528_d_n7;
        locals.var_ids_bac_dn8 = assign36150_e41528_d_n8;
        locals.var_ids_bac_dn9 = assign36150_e41528_d_n9;
        locals.var_ids_bac_dn10 = assign36150_e41528_d_n10;
        locals.var_ids_bac_dn11 = assign36150_e41528_d_n11;
        locals.var_ids_bac_dn14 = assign36150_e41528_d_n14;
        locals.var_ids_bac_rv = 0.0;

        let (assign36160_e41538, assign36160_e41538_d_n0, assign36160_e41538_d_n2, assign36160_e41538_d_n4, assign36160_e41538_d_n5, assign36160_e41538_d_n6, assign36160_e41538_d_n7, assign36160_e41538_d_n8, assign36160_e41538_d_n9, assign36160_e41538_d_n10, assign36160_e41538_d_n11, assign36160_e41538_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign36160_e41534: f64 = (locals.var_weff_nf * locals.var_beta_inv);
        let assign36160_e41536: f64 = (assign36160_e41534 / locals.var_lch);
        (assign36160_e41536, ((((locals.var_weff_nf * locals.var_beta_inv_dn0) * locals.var_lch) - (assign36160_e41534 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn2) * locals.var_lch) - (assign36160_e41534 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn4) * locals.var_lch) - (assign36160_e41534 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn5) * locals.var_lch) - (assign36160_e41534 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn6) * locals.var_lch) - (assign36160_e41534 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn7) * locals.var_lch) - (assign36160_e41534 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn8) * locals.var_lch) - (assign36160_e41534 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn9) * locals.var_lch) - (assign36160_e41534 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn10) * locals.var_lch) - (assign36160_e41534 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn11) * locals.var_lch) - (assign36160_e41534 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn14) * locals.var_lch) - (assign36160_e41534 * locals.var_lch_dn14)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_betawl, locals.var_betawl_dn0, locals.var_betawl_dn2, locals.var_betawl_dn4, locals.var_betawl_dn5, locals.var_betawl_dn6, locals.var_betawl_dn7, locals.var_betawl_dn8, locals.var_betawl_dn9, locals.var_betawl_dn10, locals.var_betawl_dn11, locals.var_betawl_dn14,)
    }
};
        locals.var_betawl = assign36160_e41538;
        locals.var_betawl_dn0 = assign36160_e41538_d_n0;
        locals.var_betawl_dn2 = assign36160_e41538_d_n2;
        locals.var_betawl_dn4 = assign36160_e41538_d_n4;
        locals.var_betawl_dn5 = assign36160_e41538_d_n5;
        locals.var_betawl_dn6 = assign36160_e41538_d_n6;
        locals.var_betawl_dn7 = assign36160_e41538_d_n7;
        locals.var_betawl_dn8 = assign36160_e41538_d_n8;
        locals.var_betawl_dn9 = assign36160_e41538_d_n9;
        locals.var_betawl_dn10 = assign36160_e41538_d_n10;
        locals.var_betawl_dn11 = assign36160_e41538_d_n11;
        locals.var_betawl_dn14 = assign36160_e41538_d_n14;
        locals.var_betawl_rv = 0.0;

        let (assign36170_e41552, assign36170_e41552_d_n0, assign36170_e41552_d_n2, assign36170_e41552_d_n4, assign36170_e41552_d_n5, assign36170_e41552_d_n6, assign36170_e41552_d_n7, assign36170_e41552_d_n8, assign36170_e41552_d_n9, assign36170_e41552_d_n10, assign36170_e41552_d_n11, assign36170_e41552_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign36170_e41544: f64 = (locals.var_betawl * locals.var_idd);
        let assign36170_e41546: f64 = (assign36170_e41544 * locals.var_mu);
        let assign36170_e41548: f64 = (assign36170_e41546 + locals.var_ids_res);
        let assign36170_e41550: f64 = (assign36170_e41548 + locals.var_ids_bac);
        (assign36170_e41550, ((((((locals.var_betawl_dn0 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn0)) * locals.var_mu) + (assign36170_e41544 * locals.var_mu_dn0)) + locals.var_ids_res_dn0) + locals.var_ids_bac_dn0), ((((((locals.var_betawl_dn2 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn2)) * locals.var_mu) + (assign36170_e41544 * locals.var_mu_dn2)) + locals.var_ids_res_dn2) + locals.var_ids_bac_dn2), ((((((locals.var_betawl_dn4 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn4)) * locals.var_mu) + (assign36170_e41544 * locals.var_mu_dn4)) + locals.var_ids_res_dn4) + locals.var_ids_bac_dn4), ((((((locals.var_betawl_dn5 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn5)) * locals.var_mu) + (assign36170_e41544 * locals.var_mu_dn5)) + locals.var_ids_res_dn5) + locals.var_ids_bac_dn5), ((((((locals.var_betawl_dn6 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn6)) * locals.var_mu) + (assign36170_e41544 * locals.var_mu_dn6)) + locals.var_ids_res_dn6) + locals.var_ids_bac_dn6), ((((((locals.var_betawl_dn7 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn7)) * locals.var_mu) + (assign36170_e41544 * locals.var_mu_dn7)) + locals.var_ids_res_dn7) + locals.var_ids_bac_dn7), ((((((locals.var_betawl_dn8 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn8)) * locals.var_mu) + (assign36170_e41544 * locals.var_mu_dn8)) + locals.var_ids_res_dn8) + locals.var_ids_bac_dn8), ((((((locals.var_betawl_dn9 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn9)) * locals.var_mu) + (assign36170_e41544 * locals.var_mu_dn9)) + locals.var_ids_res_dn9) + locals.var_ids_bac_dn9), ((((((locals.var_betawl_dn10 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn10)) * locals.var_mu) + (assign36170_e41544 * locals.var_mu_dn10)) + locals.var_ids_res_dn10) + locals.var_ids_bac_dn10), ((((((locals.var_betawl_dn11 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn11)) * locals.var_mu) + (assign36170_e41544 * locals.var_mu_dn11)) + locals.var_ids_res_dn11) + locals.var_ids_bac_dn11), ((((((locals.var_betawl_dn14 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn14)) * locals.var_mu) + (assign36170_e41544 * locals.var_mu_dn14)) + locals.var_ids_res_dn14) + locals.var_ids_bac_dn14),)
    } else {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn4, locals.var_ids0_dn5, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn8, locals.var_ids0_dn9, locals.var_ids0_dn10, locals.var_ids0_dn11, locals.var_ids0_dn14,)
    }
};
        locals.var_ids0 = assign36170_e41552;
        locals.var_ids0_dn0 = assign36170_e41552_d_n0;
        locals.var_ids0_dn2 = assign36170_e41552_d_n2;
        locals.var_ids0_dn4 = assign36170_e41552_d_n4;
        locals.var_ids0_dn5 = assign36170_e41552_d_n5;
        locals.var_ids0_dn6 = assign36170_e41552_d_n6;
        locals.var_ids0_dn7 = assign36170_e41552_d_n7;
        locals.var_ids0_dn8 = assign36170_e41552_d_n8;
        locals.var_ids0_dn9 = assign36170_e41552_d_n9;
        locals.var_ids0_dn10 = assign36170_e41552_d_n10;
        locals.var_ids0_dn11 = assign36170_e41552_d_n11;
        locals.var_ids0_dn14 = assign36170_e41552_d_n14;
        locals.var_ids0_rv = 0.0;

        let (assign36180_e41562, assign36180_e41562_d_n0, assign36180_e41562_d_n2, assign36180_e41562_d_n4, assign36180_e41562_d_n5, assign36180_e41562_d_n6, assign36180_e41562_d_n7, assign36180_e41562_d_n8, assign36180_e41562_d_n9, assign36180_e41562_d_n10, assign36180_e41562_d_n11, assign36180_e41562_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        let assign36180_e41558: f64 = (locals.var_betawl * locals.var_idd);
        let assign36180_e41560: f64 = (assign36180_e41558 * locals.var_mu);
        (assign36180_e41560, ((((locals.var_betawl_dn0 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn0)) * locals.var_mu) + (assign36180_e41558 * locals.var_mu_dn0)), ((((locals.var_betawl_dn2 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn2)) * locals.var_mu) + (assign36180_e41558 * locals.var_mu_dn2)), ((((locals.var_betawl_dn4 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn4)) * locals.var_mu) + (assign36180_e41558 * locals.var_mu_dn4)), ((((locals.var_betawl_dn5 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn5)) * locals.var_mu) + (assign36180_e41558 * locals.var_mu_dn5)), ((((locals.var_betawl_dn6 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn6)) * locals.var_mu) + (assign36180_e41558 * locals.var_mu_dn6)), ((((locals.var_betawl_dn7 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn7)) * locals.var_mu) + (assign36180_e41558 * locals.var_mu_dn7)), ((((locals.var_betawl_dn8 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn8)) * locals.var_mu) + (assign36180_e41558 * locals.var_mu_dn8)), ((((locals.var_betawl_dn9 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn9)) * locals.var_mu) + (assign36180_e41558 * locals.var_mu_dn9)), ((((locals.var_betawl_dn10 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn10)) * locals.var_mu) + (assign36180_e41558 * locals.var_mu_dn10)), ((((locals.var_betawl_dn11 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn11)) * locals.var_mu) + (assign36180_e41558 * locals.var_mu_dn11)), ((((locals.var_betawl_dn14 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn14)) * locals.var_mu) + (assign36180_e41558 * locals.var_mu_dn14)),)
    } else {
        (locals.var_ids_acc, locals.var_ids_acc_dn0, locals.var_ids_acc_dn2, locals.var_ids_acc_dn4, locals.var_ids_acc_dn5, locals.var_ids_acc_dn6, locals.var_ids_acc_dn7, locals.var_ids_acc_dn8, locals.var_ids_acc_dn9, locals.var_ids_acc_dn10, locals.var_ids_acc_dn11, locals.var_ids_acc_dn14,)
    }
};
        locals.var_ids_acc = assign36180_e41562;
        locals.var_ids_acc_dn0 = assign36180_e41562_d_n0;
        locals.var_ids_acc_dn2 = assign36180_e41562_d_n2;
        locals.var_ids_acc_dn4 = assign36180_e41562_d_n4;
        locals.var_ids_acc_dn5 = assign36180_e41562_d_n5;
        locals.var_ids_acc_dn6 = assign36180_e41562_d_n6;
        locals.var_ids_acc_dn7 = assign36180_e41562_d_n7;
        locals.var_ids_acc_dn8 = assign36180_e41562_d_n8;
        locals.var_ids_acc_dn9 = assign36180_e41562_d_n9;
        locals.var_ids_acc_dn10 = assign36180_e41562_d_n10;
        locals.var_ids_acc_dn11 = assign36180_e41562_d_n11;
        locals.var_ids_acc_dn14 = assign36180_e41562_d_n14;
        locals.var_ids_acc_rv = 0.0;

        let (assign36190_e41568, assign36190_e41568_d_n0, assign36190_e41568_d_n2, assign36190_e41568_d_n4, assign36190_e41568_d_n5, assign36190_e41568_d_n6, assign36190_e41568_d_n7, assign36190_e41568_d_n8, assign36190_e41568_d_n9, assign36190_e41568_d_n10, assign36190_e41568_d_n11, assign36190_e41568_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        (locals.var_mu, locals.var_mu_dn0, locals.var_mu_dn2, locals.var_mu_dn4, locals.var_mu_dn5, locals.var_mu_dn6, locals.var_mu_dn7, locals.var_mu_dn8, locals.var_mu_dn9, locals.var_mu_dn10, locals.var_mu_dn11, locals.var_mu_dn14,)
    } else {
        (locals.var_mu_acc, locals.var_mu_acc_dn0, locals.var_mu_acc_dn2, locals.var_mu_acc_dn4, locals.var_mu_acc_dn5, locals.var_mu_acc_dn6, locals.var_mu_acc_dn7, locals.var_mu_acc_dn8, locals.var_mu_acc_dn9, locals.var_mu_acc_dn10, locals.var_mu_acc_dn11, locals.var_mu_acc_dn14,)
    }
};
        locals.var_mu_acc = assign36190_e41568;
        locals.var_mu_acc_dn0 = assign36190_e41568_d_n0;
        locals.var_mu_acc_dn2 = assign36190_e41568_d_n2;
        locals.var_mu_acc_dn4 = assign36190_e41568_d_n4;
        locals.var_mu_acc_dn5 = assign36190_e41568_d_n5;
        locals.var_mu_acc_dn6 = assign36190_e41568_d_n6;
        locals.var_mu_acc_dn7 = assign36190_e41568_d_n7;
        locals.var_mu_acc_dn8 = assign36190_e41568_d_n8;
        locals.var_mu_acc_dn9 = assign36190_e41568_d_n9;
        locals.var_mu_acc_dn10 = assign36190_e41568_d_n10;
        locals.var_mu_acc_dn11 = assign36190_e41568_d_n11;
        locals.var_mu_acc_dn14 = assign36190_e41568_d_n14;
        locals.var_mu_acc_rv = 0.0;

        let (assign36200_e41574, assign36200_e41574_d_n0, assign36200_e41574_d_n2, assign36200_e41574_d_n4, assign36200_e41574_d_n5, assign36200_e41574_d_n6, assign36200_e41574_d_n7, assign36200_e41574_d_n8, assign36200_e41574_d_n9, assign36200_e41574_d_n10, assign36200_e41574_d_n11, assign36200_e41574_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn14,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    }
};
        locals.var_vds = assign36200_e41574;
        locals.var_vds_dn0 = assign36200_e41574_d_n0;
        locals.var_vds_dn2 = assign36200_e41574_d_n2;
        locals.var_vds_dn4 = assign36200_e41574_d_n4;
        locals.var_vds_dn5 = assign36200_e41574_d_n5;
        locals.var_vds_dn6 = assign36200_e41574_d_n6;
        locals.var_vds_dn7 = assign36200_e41574_d_n7;
        locals.var_vds_dn8 = assign36200_e41574_d_n8;
        locals.var_vds_dn9 = assign36200_e41574_d_n9;
        locals.var_vds_dn10 = assign36200_e41574_d_n10;
        locals.var_vds_dn11 = assign36200_e41574_d_n11;
        locals.var_vds_dn14 = assign36200_e41574_d_n14;
        locals.var_vds_rv = 0.0;

        let assign36210_e41577: f64 = if p.p283 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard825 = assign36210_e41577;
        locals.var_guard825_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_119(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign36220_e41589, assign36220_e41589_d_n0, assign36220_e41589_d_n2, assign36220_e41589_d_n4, assign36220_e41589_d_n5, assign36220_e41589_d_n6, assign36220_e41589_d_n7, assign36220_e41589_d_n8, assign36220_e41589_d_n9, assign36220_e41589_d_n10, assign36220_e41589_d_n11, assign36220_e41589_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard825 != 0.0)) {
        let assign36220_e41586: f64 = (locals.var_vds - locals.var_pds);
        let assign36220_e41587: f64 = (0.5 * assign36220_e41586);
        (assign36220_e41587, (0.5 * (locals.var_vds_dn0 - locals.var_pds_dn0)), (0.5 * (locals.var_vds_dn2 - locals.var_pds_dn2)), (0.5 * (locals.var_vds_dn4 - locals.var_pds_dn4)), (0.5 * (locals.var_vds_dn5 - locals.var_pds_dn5)), (0.5 * (locals.var_vds_dn6 - locals.var_pds_dn6)), (0.5 * (locals.var_vds_dn7 - locals.var_pds_dn7)), (0.5 * (locals.var_vds_dn8 - locals.var_pds_dn8)), (0.5 * (locals.var_vds_dn9 - locals.var_pds_dn9)), (0.5 * (locals.var_vds_dn10 - locals.var_pds_dn10)), (0.5 * (locals.var_vds_dn11 - locals.var_pds_dn11)), (0.5 * (locals.var_vds_dn14 - locals.var_pds_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign36220_e41589;
        locals.var_t1_dn0 = assign36220_e41589_d_n0;
        locals.var_t1_dn2 = assign36220_e41589_d_n2;
        locals.var_t1_dn4 = assign36220_e41589_d_n4;
        locals.var_t1_dn5 = assign36220_e41589_d_n5;
        locals.var_t1_dn6 = assign36220_e41589_d_n6;
        locals.var_t1_dn7 = assign36220_e41589_d_n7;
        locals.var_t1_dn8 = assign36220_e41589_d_n8;
        locals.var_t1_dn9 = assign36220_e41589_d_n9;
        locals.var_t1_dn10 = assign36220_e41589_d_n10;
        locals.var_t1_dn11 = assign36220_e41589_d_n11;
        locals.var_t1_dn14 = assign36220_e41589_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign36230_e41601, assign36230_e41601_d_n0, assign36230_e41601_d_n2, assign36230_e41601_d_n4, assign36230_e41601_d_n5, assign36230_e41601_d_n6, assign36230_e41601_d_n7, assign36230_e41601_d_n8, assign36230_e41601_d_n9, assign36230_e41601_d_n10, assign36230_e41601_d_n11, assign36230_e41601_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard825 != 0.0)) {
        let assign36230_e41597: f64 = (2.0 * locals.var_t1);
        let assign36230_e41599: f64 = (assign36230_e41597 / 0.01);
        (assign36230_e41599, ((2.0 * locals.var_t1_dn0) / 0.01), ((2.0 * locals.var_t1_dn2) / 0.01), ((2.0 * locals.var_t1_dn4) / 0.01), ((2.0 * locals.var_t1_dn5) / 0.01), ((2.0 * locals.var_t1_dn6) / 0.01), ((2.0 * locals.var_t1_dn7) / 0.01), ((2.0 * locals.var_t1_dn8) / 0.01), ((2.0 * locals.var_t1_dn9) / 0.01), ((2.0 * locals.var_t1_dn10) / 0.01), ((2.0 * locals.var_t1_dn11) / 0.01), ((2.0 * locals.var_t1_dn14) / 0.01),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign36230_e41601;
        locals.var_tmf1_dn0 = assign36230_e41601_d_n0;
        locals.var_tmf1_dn2 = assign36230_e41601_d_n2;
        locals.var_tmf1_dn4 = assign36230_e41601_d_n4;
        locals.var_tmf1_dn5 = assign36230_e41601_d_n5;
        locals.var_tmf1_dn6 = assign36230_e41601_d_n6;
        locals.var_tmf1_dn7 = assign36230_e41601_d_n7;
        locals.var_tmf1_dn8 = assign36230_e41601_d_n8;
        locals.var_tmf1_dn9 = assign36230_e41601_d_n9;
        locals.var_tmf1_dn10 = assign36230_e41601_d_n10;
        locals.var_tmf1_dn11 = assign36230_e41601_d_n11;
        locals.var_tmf1_dn14 = assign36230_e41601_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign36240_e41645, assign36240_e41645_d_n0, assign36240_e41645_d_n2, assign36240_e41645_d_n4, assign36240_e41645_d_n5, assign36240_e41645_d_n6, assign36240_e41645_d_n7, assign36240_e41645_d_n8, assign36240_e41645_d_n9, assign36240_e41645_d_n10, assign36240_e41645_d_n11, assign36240_e41645_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard825 != 0.0)) {
        let assign36240_e41611: f64 = (1.0 / 2.0);
        let assign36240_e41615: f64 = (1.0 / 6.0);
        let assign36240_e41619: f64 = (1.0 / 24.0);
        let assign36240_e41623: f64 = (1.0 / 120.0);
        let assign36240_e41627: f64 = (1.0 / 720.0);
        let assign36240_e41631: f64 = (1.0 / 5040.0);
        let assign36240_e41632: f64 = (locals.var_tmf1 * assign36240_e41631);
        let assign36240_e41633: f64 = (assign36240_e41627 + assign36240_e41632);
        let assign36240_e41634: f64 = (locals.var_tmf1 * assign36240_e41633);
        let assign36240_e41635: f64 = (assign36240_e41623 + assign36240_e41634);
        let assign36240_e41636: f64 = (locals.var_tmf1 * assign36240_e41635);
        let assign36240_e41637: f64 = (assign36240_e41619 + assign36240_e41636);
        let assign36240_e41638: f64 = (locals.var_tmf1 * assign36240_e41637);
        let assign36240_e41639: f64 = (assign36240_e41615 + assign36240_e41638);
        let assign36240_e41640: f64 = (locals.var_tmf1 * assign36240_e41639);
        let assign36240_e41641: f64 = (assign36240_e41611 + assign36240_e41640);
        let assign36240_e41642: f64 = (locals.var_tmf1 * assign36240_e41641);
        let assign36240_e41643: f64 = (1.0 + assign36240_e41642);
        (assign36240_e41643, ((locals.var_tmf1_dn0 * assign36240_e41641) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign36240_e41639) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign36240_e41637) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign36240_e41635) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign36240_e41633) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign36240_e41631))))))))))), ((locals.var_tmf1_dn2 * assign36240_e41641) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign36240_e41639) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign36240_e41637) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign36240_e41635) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign36240_e41633) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign36240_e41631))))))))))), ((locals.var_tmf1_dn4 * assign36240_e41641) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign36240_e41639) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign36240_e41637) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign36240_e41635) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign36240_e41633) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign36240_e41631))))))))))), ((locals.var_tmf1_dn5 * assign36240_e41641) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign36240_e41639) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign36240_e41637) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign36240_e41635) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign36240_e41633) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign36240_e41631))))))))))), ((locals.var_tmf1_dn6 * assign36240_e41641) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign36240_e41639) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign36240_e41637) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign36240_e41635) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign36240_e41633) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign36240_e41631))))))))))), ((locals.var_tmf1_dn7 * assign36240_e41641) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign36240_e41639) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign36240_e41637) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign36240_e41635) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign36240_e41633) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign36240_e41631))))))))))), ((locals.var_tmf1_dn8 * assign36240_e41641) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign36240_e41639) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign36240_e41637) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign36240_e41635) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign36240_e41633) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign36240_e41631))))))))))), ((locals.var_tmf1_dn9 * assign36240_e41641) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign36240_e41639) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign36240_e41637) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign36240_e41635) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign36240_e41633) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign36240_e41631))))))))))), ((locals.var_tmf1_dn10 * assign36240_e41641) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign36240_e41639) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign36240_e41637) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign36240_e41635) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign36240_e41633) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign36240_e41631))))))))))), ((locals.var_tmf1_dn11 * assign36240_e41641) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign36240_e41639) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign36240_e41637) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign36240_e41635) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign36240_e41633) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign36240_e41631))))))))))), ((locals.var_tmf1_dn14 * assign36240_e41641) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign36240_e41639) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign36240_e41637) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign36240_e41635) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign36240_e41633) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign36240_e41631))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign36240_e41645;
        locals.var_tmf2_dn0 = assign36240_e41645_d_n0;
        locals.var_tmf2_dn2 = assign36240_e41645_d_n2;
        locals.var_tmf2_dn4 = assign36240_e41645_d_n4;
        locals.var_tmf2_dn5 = assign36240_e41645_d_n5;
        locals.var_tmf2_dn6 = assign36240_e41645_d_n6;
        locals.var_tmf2_dn7 = assign36240_e41645_d_n7;
        locals.var_tmf2_dn8 = assign36240_e41645_d_n8;
        locals.var_tmf2_dn9 = assign36240_e41645_d_n9;
        locals.var_tmf2_dn10 = assign36240_e41645_d_n10;
        locals.var_tmf2_dn11 = assign36240_e41645_d_n11;
        locals.var_tmf2_dn14 = assign36240_e41645_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign36250_e41685, assign36250_e41685_d_n0, assign36250_e41685_d_n2, assign36250_e41685_d_n4, assign36250_e41685_d_n5, assign36250_e41685_d_n6, assign36250_e41685_d_n7, assign36250_e41685_d_n8, assign36250_e41685_d_n9, assign36250_e41685_d_n10, assign36250_e41685_d_n11, assign36250_e41685_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard825 != 0.0)) {
        let assign36250_e41653: f64 = (1.0 / 2.0);
        let assign36250_e41657: f64 = (1.0 / 3.0);
        let assign36250_e41661: f64 = (1.0 / 8.0);
        let assign36250_e41665: f64 = (1.0 / 30.0);
        let assign36250_e41669: f64 = (1.0 / 144.0);
        let assign36250_e41673: f64 = (1.0 / 840.0);
        let assign36250_e41674: f64 = (locals.var_tmf1 * assign36250_e41673);
        let assign36250_e41675: f64 = (assign36250_e41669 + assign36250_e41674);
        let assign36250_e41676: f64 = (locals.var_tmf1 * assign36250_e41675);
        let assign36250_e41677: f64 = (assign36250_e41665 + assign36250_e41676);
        let assign36250_e41678: f64 = (locals.var_tmf1 * assign36250_e41677);
        let assign36250_e41679: f64 = (assign36250_e41661 + assign36250_e41678);
        let assign36250_e41680: f64 = (locals.var_tmf1 * assign36250_e41679);
        let assign36250_e41681: f64 = (assign36250_e41657 + assign36250_e41680);
        let assign36250_e41682: f64 = (locals.var_tmf1 * assign36250_e41681);
        let assign36250_e41683: f64 = (assign36250_e41653 + assign36250_e41682);
        (assign36250_e41683, ((locals.var_tmf1_dn0 * assign36250_e41681) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign36250_e41679) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign36250_e41677) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign36250_e41675) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign36250_e41673))))))))), ((locals.var_tmf1_dn2 * assign36250_e41681) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign36250_e41679) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign36250_e41677) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign36250_e41675) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign36250_e41673))))))))), ((locals.var_tmf1_dn4 * assign36250_e41681) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign36250_e41679) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign36250_e41677) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign36250_e41675) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign36250_e41673))))))))), ((locals.var_tmf1_dn5 * assign36250_e41681) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign36250_e41679) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign36250_e41677) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign36250_e41675) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign36250_e41673))))))))), ((locals.var_tmf1_dn6 * assign36250_e41681) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign36250_e41679) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign36250_e41677) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign36250_e41675) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign36250_e41673))))))))), ((locals.var_tmf1_dn7 * assign36250_e41681) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign36250_e41679) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign36250_e41677) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign36250_e41675) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign36250_e41673))))))))), ((locals.var_tmf1_dn8 * assign36250_e41681) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign36250_e41679) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign36250_e41677) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign36250_e41675) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign36250_e41673))))))))), ((locals.var_tmf1_dn9 * assign36250_e41681) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign36250_e41679) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign36250_e41677) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign36250_e41675) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign36250_e41673))))))))), ((locals.var_tmf1_dn10 * assign36250_e41681) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign36250_e41679) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign36250_e41677) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign36250_e41675) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign36250_e41673))))))))), ((locals.var_tmf1_dn11 * assign36250_e41681) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign36250_e41679) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign36250_e41677) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign36250_e41675) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign36250_e41673))))))))), ((locals.var_tmf1_dn14 * assign36250_e41681) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign36250_e41679) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign36250_e41677) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign36250_e41675) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign36250_e41673))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign36250_e41685;
        locals.var_tmf3_dn0 = assign36250_e41685_d_n0;
        locals.var_tmf3_dn2 = assign36250_e41685_d_n2;
        locals.var_tmf3_dn4 = assign36250_e41685_d_n4;
        locals.var_tmf3_dn5 = assign36250_e41685_d_n5;
        locals.var_tmf3_dn6 = assign36250_e41685_d_n6;
        locals.var_tmf3_dn7 = assign36250_e41685_d_n7;
        locals.var_tmf3_dn8 = assign36250_e41685_d_n8;
        locals.var_tmf3_dn9 = assign36250_e41685_d_n9;
        locals.var_tmf3_dn10 = assign36250_e41685_d_n10;
        locals.var_tmf3_dn11 = assign36250_e41685_d_n11;
        locals.var_tmf3_dn14 = assign36250_e41685_d_n14;
        locals.var_tmf3_rv = 0.0;

        let (assign36260_e41695, assign36260_e41695_d_n0, assign36260_e41695_d_n2, assign36260_e41695_d_n4, assign36260_e41695_d_n5, assign36260_e41695_d_n6, assign36260_e41695_d_n7, assign36260_e41695_d_n8, assign36260_e41695_d_n9, assign36260_e41695_d_n10, assign36260_e41695_d_n11, assign36260_e41695_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard825 != 0.0)) {
        let assign36260_e41693: f64 = (0.01 / locals.var_tmf2);
        (assign36260_e41693, (-((0.01 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign36260_e41695;
        locals.var_t6_dn0 = assign36260_e41695_d_n0;
        locals.var_t6_dn2 = assign36260_e41695_d_n2;
        locals.var_t6_dn4 = assign36260_e41695_d_n4;
        locals.var_t6_dn5 = assign36260_e41695_d_n5;
        locals.var_t6_dn6 = assign36260_e41695_d_n6;
        locals.var_t6_dn7 = assign36260_e41695_d_n7;
        locals.var_t6_dn8 = assign36260_e41695_d_n8;
        locals.var_t6_dn9 = assign36260_e41695_d_n9;
        locals.var_t6_dn10 = assign36260_e41695_d_n10;
        locals.var_t6_dn11 = assign36260_e41695_d_n11;
        locals.var_t6_dn14 = assign36260_e41695_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign36270_e41710, assign36270_e41710_d_n0, assign36270_e41710_d_n2, assign36270_e41710_d_n4, assign36270_e41710_d_n5, assign36270_e41710_d_n6, assign36270_e41710_d_n7, assign36270_e41710_d_n8, assign36270_e41710_d_n9, assign36270_e41710_d_n10, assign36270_e41710_d_n11, assign36270_e41710_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard825 != 0.0)) {
        let assign36270_e41702: f64 = (-2.0);
        let assign36270_e41704: f64 = (assign36270_e41702 * locals.var_tmf3);
        let assign36270_e41707: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign36270_e41708: f64 = (assign36270_e41704 / assign36270_e41707);
        (assign36270_e41708, ((((assign36270_e41702 * locals.var_tmf3_dn0) * assign36270_e41707) - (assign36270_e41704 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign36270_e41707 * assign36270_e41707)), ((((assign36270_e41702 * locals.var_tmf3_dn2) * assign36270_e41707) - (assign36270_e41704 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign36270_e41707 * assign36270_e41707)), ((((assign36270_e41702 * locals.var_tmf3_dn4) * assign36270_e41707) - (assign36270_e41704 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign36270_e41707 * assign36270_e41707)), ((((assign36270_e41702 * locals.var_tmf3_dn5) * assign36270_e41707) - (assign36270_e41704 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign36270_e41707 * assign36270_e41707)), ((((assign36270_e41702 * locals.var_tmf3_dn6) * assign36270_e41707) - (assign36270_e41704 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign36270_e41707 * assign36270_e41707)), ((((assign36270_e41702 * locals.var_tmf3_dn7) * assign36270_e41707) - (assign36270_e41704 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign36270_e41707 * assign36270_e41707)), ((((assign36270_e41702 * locals.var_tmf3_dn8) * assign36270_e41707) - (assign36270_e41704 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign36270_e41707 * assign36270_e41707)), ((((assign36270_e41702 * locals.var_tmf3_dn9) * assign36270_e41707) - (assign36270_e41704 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign36270_e41707 * assign36270_e41707)), ((((assign36270_e41702 * locals.var_tmf3_dn10) * assign36270_e41707) - (assign36270_e41704 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign36270_e41707 * assign36270_e41707)), ((((assign36270_e41702 * locals.var_tmf3_dn11) * assign36270_e41707) - (assign36270_e41704 * ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)))) / (assign36270_e41707 * assign36270_e41707)), ((((assign36270_e41702 * locals.var_tmf3_dn14) * assign36270_e41707) - (assign36270_e41704 * ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)))) / (assign36270_e41707 * assign36270_e41707)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign36270_e41710;
        locals.var_t2_dn0 = assign36270_e41710_d_n0;
        locals.var_t2_dn2 = assign36270_e41710_d_n2;
        locals.var_t2_dn4 = assign36270_e41710_d_n4;
        locals.var_t2_dn5 = assign36270_e41710_d_n5;
        locals.var_t2_dn6 = assign36270_e41710_d_n6;
        locals.var_t2_dn7 = assign36270_e41710_d_n7;
        locals.var_t2_dn8 = assign36270_e41710_d_n8;
        locals.var_t2_dn9 = assign36270_e41710_d_n9;
        locals.var_t2_dn10 = assign36270_e41710_d_n10;
        locals.var_t2_dn11 = assign36270_e41710_d_n11;
        locals.var_t2_dn14 = assign36270_e41710_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign36280_e41722, assign36280_e41722_d_n0, assign36280_e41722_d_n2, assign36280_e41722_d_n4, assign36280_e41722_d_n5, assign36280_e41722_d_n6, assign36280_e41722_d_n7, assign36280_e41722_d_n8, assign36280_e41722_d_n9, assign36280_e41722_d_n10, assign36280_e41722_d_n11, assign36280_e41722_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard825 != 0.0)) {
        let assign36280_e41719: f64 = (locals.var_phi_s0_dep + locals.var_t6);
        let assign36280_e41720: f64 = (1.1 - assign36280_e41719);
        (assign36280_e41720, (-(locals.var_phi_s0_dep_dn0 + locals.var_t6_dn0)), (-(locals.var_phi_s0_dep_dn2 + locals.var_t6_dn2)), (-(locals.var_phi_s0_dep_dn4 + locals.var_t6_dn4)), (-(locals.var_phi_s0_dep_dn5 + locals.var_t6_dn5)), (-(locals.var_phi_s0_dep_dn6 + locals.var_t6_dn6)), (-(locals.var_phi_s0_dep_dn7 + locals.var_t6_dn7)), (-(locals.var_phi_s0_dep_dn8 + locals.var_t6_dn8)), (-(locals.var_phi_s0_dep_dn9 + locals.var_t6_dn9)), (-(locals.var_phi_s0_dep_dn10 + locals.var_t6_dn10)), (-(locals.var_phi_s0_dep_dn11 + locals.var_t6_dn11)), (-(locals.var_phi_s0_dep_dn14 + locals.var_t6_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign36280_e41722;
        locals.var_t1_dn0 = assign36280_e41722_d_n0;
        locals.var_t1_dn2 = assign36280_e41722_d_n2;
        locals.var_t1_dn4 = assign36280_e41722_d_n4;
        locals.var_t1_dn5 = assign36280_e41722_d_n5;
        locals.var_t1_dn6 = assign36280_e41722_d_n6;
        locals.var_t1_dn7 = assign36280_e41722_d_n7;
        locals.var_t1_dn8 = assign36280_e41722_d_n8;
        locals.var_t1_dn9 = assign36280_e41722_d_n9;
        locals.var_t1_dn10 = assign36280_e41722_d_n10;
        locals.var_t1_dn11 = assign36280_e41722_d_n11;
        locals.var_t1_dn14 = assign36280_e41722_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign36290_e41739, assign36290_e41739_d_n0, assign36290_e41739_d_n2, assign36290_e41739_d_n4, assign36290_e41739_d_n5, assign36290_e41739_d_n6, assign36290_e41739_d_n7, assign36290_e41739_d_n8, assign36290_e41739_d_n9, assign36290_e41739_d_n10, assign36290_e41739_d_n11, assign36290_e41739_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard825 != 0.0)) {
        let assign36290_e41730: f64 = (locals.var_t1 * locals.var_t1);
        let assign36290_e41733: f64 = (4.0 * 0.05);
        let assign36290_e41735: f64 = (assign36290_e41733 * 0.05);
        let assign36290_e41736: f64 = (assign36290_e41730 + assign36290_e41735);
        let assign36290_e41737: f64 = (assign36290_e41736).sqrt();
        (assign36290_e41737, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign36290_e41737)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign36290_e41737)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign36290_e41737)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign36290_e41737)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign36290_e41737)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign36290_e41737)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign36290_e41737)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign36290_e41737)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign36290_e41737)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign36290_e41737)), (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign36290_e41737)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign36290_e41739;
        locals.var_tmf2_dn0 = assign36290_e41739_d_n0;
        locals.var_tmf2_dn2 = assign36290_e41739_d_n2;
        locals.var_tmf2_dn4 = assign36290_e41739_d_n4;
        locals.var_tmf2_dn5 = assign36290_e41739_d_n5;
        locals.var_tmf2_dn6 = assign36290_e41739_d_n6;
        locals.var_tmf2_dn7 = assign36290_e41739_d_n7;
        locals.var_tmf2_dn8 = assign36290_e41739_d_n8;
        locals.var_tmf2_dn9 = assign36290_e41739_d_n9;
        locals.var_tmf2_dn10 = assign36290_e41739_d_n10;
        locals.var_tmf2_dn11 = assign36290_e41739_d_n11;
        locals.var_tmf2_dn14 = assign36290_e41739_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign36300_e41753, assign36300_e41753_d_n0, assign36300_e41753_d_n2, assign36300_e41753_d_n4, assign36300_e41753_d_n5, assign36300_e41753_d_n6, assign36300_e41753_d_n7, assign36300_e41753_d_n8, assign36300_e41753_d_n9, assign36300_e41753_d_n10, assign36300_e41753_d_n11, assign36300_e41753_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard825 != 0.0)) {
        let assign36300_e41749: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign36300_e41750: f64 = (1.0 + assign36300_e41749);
        let assign36300_e41751: f64 = (0.5 * assign36300_e41750);
        (assign36300_e41751, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn14 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign36300_e41753;
        locals.var_t0_dn0 = assign36300_e41753_d_n0;
        locals.var_t0_dn2 = assign36300_e41753_d_n2;
        locals.var_t0_dn4 = assign36300_e41753_d_n4;
        locals.var_t0_dn5 = assign36300_e41753_d_n5;
        locals.var_t0_dn6 = assign36300_e41753_d_n6;
        locals.var_t0_dn7 = assign36300_e41753_d_n7;
        locals.var_t0_dn8 = assign36300_e41753_d_n8;
        locals.var_t0_dn9 = assign36300_e41753_d_n9;
        locals.var_t0_dn10 = assign36300_e41753_d_n10;
        locals.var_t0_dn11 = assign36300_e41753_d_n11;
        locals.var_t0_dn14 = assign36300_e41753_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign36310_e41765, assign36310_e41765_d_n0, assign36310_e41765_d_n2, assign36310_e41765_d_n4, assign36310_e41765_d_n5, assign36310_e41765_d_n6, assign36310_e41765_d_n7, assign36310_e41765_d_n8, assign36310_e41765_d_n9, assign36310_e41765_d_n10, assign36310_e41765_d_n11, assign36310_e41765_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard825 != 0.0)) {
        let assign36310_e41762: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign36310_e41763: f64 = (0.5 * assign36310_e41762);
        (assign36310_e41763, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign36310_e41765;
        locals.var_t2_dn0 = assign36310_e41765_d_n0;
        locals.var_t2_dn2 = assign36310_e41765_d_n2;
        locals.var_t2_dn4 = assign36310_e41765_d_n4;
        locals.var_t2_dn5 = assign36310_e41765_d_n5;
        locals.var_t2_dn6 = assign36310_e41765_d_n6;
        locals.var_t2_dn7 = assign36310_e41765_d_n7;
        locals.var_t2_dn8 = assign36310_e41765_d_n8;
        locals.var_t2_dn9 = assign36310_e41765_d_n9;
        locals.var_t2_dn10 = assign36310_e41765_d_n10;
        locals.var_t2_dn11 = assign36310_e41765_d_n11;
        locals.var_t2_dn14 = assign36310_e41765_d_n14;
        locals.var_t2_rv = 0.0;

        let assign36320_e41768: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard826 = assign36320_e41768;
        locals.var_guard826_rv = 0.0;

        let (assign36330_e41778, assign36330_e41778_d_n0, assign36330_e41778_d_n2, assign36330_e41778_d_n4, assign36330_e41778_d_n5, assign36330_e41778_d_n6, assign36330_e41778_d_n7, assign36330_e41778_d_n8, assign36330_e41778_d_n9, assign36330_e41778_d_n10, assign36330_e41778_d_n11, assign36330_e41778_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard825 != 0.0)) && (locals.var_guard826 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign36330_e41778;
        locals.var_t2_dn0 = assign36330_e41778_d_n0;
        locals.var_t2_dn2 = assign36330_e41778_d_n2;
        locals.var_t2_dn4 = assign36330_e41778_d_n4;
        locals.var_t2_dn5 = assign36330_e41778_d_n5;
        locals.var_t2_dn6 = assign36330_e41778_d_n6;
        locals.var_t2_dn7 = assign36330_e41778_d_n7;
        locals.var_t2_dn8 = assign36330_e41778_d_n8;
        locals.var_t2_dn9 = assign36330_e41778_d_n9;
        locals.var_t2_dn10 = assign36330_e41778_d_n10;
        locals.var_t2_dn11 = assign36330_e41778_d_n11;
        locals.var_t2_dn14 = assign36330_e41778_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign36340_e41788, assign36340_e41788_d_n0, assign36340_e41788_d_n2, assign36340_e41788_d_n4, assign36340_e41788_d_n5, assign36340_e41788_d_n6, assign36340_e41788_d_n7, assign36340_e41788_d_n8, assign36340_e41788_d_n9, assign36340_e41788_d_n10, assign36340_e41788_d_n11, assign36340_e41788_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard825 != 0.0)) && (locals.var_guard826 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign36340_e41788;
        locals.var_t0_dn0 = assign36340_e41788_d_n0;
        locals.var_t0_dn2 = assign36340_e41788_d_n2;
        locals.var_t0_dn4 = assign36340_e41788_d_n4;
        locals.var_t0_dn5 = assign36340_e41788_d_n5;
        locals.var_t0_dn6 = assign36340_e41788_d_n6;
        locals.var_t0_dn7 = assign36340_e41788_d_n7;
        locals.var_t0_dn8 = assign36340_e41788_d_n8;
        locals.var_t0_dn9 = assign36340_e41788_d_n9;
        locals.var_t0_dn10 = assign36340_e41788_d_n10;
        locals.var_t0_dn11 = assign36340_e41788_d_n11;
        locals.var_t0_dn14 = assign36340_e41788_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign36350_e41798, assign36350_e41798_d_n0, assign36350_e41798_d_n2, assign36350_e41798_d_n4, assign36350_e41798_d_n5, assign36350_e41798_d_n6, assign36350_e41798_d_n7, assign36350_e41798_d_n8, assign36350_e41798_d_n9, assign36350_e41798_d_n10, assign36350_e41798_d_n11, assign36350_e41798_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard825 != 0.0)) {
        let assign36350_e41796: f64 = (locals.var_t2 + 1e-25);
        (assign36350_e41796, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign36350_e41798;
        locals.var_t2_dn0 = assign36350_e41798_d_n0;
        locals.var_t2_dn2 = assign36350_e41798_d_n2;
        locals.var_t2_dn4 = assign36350_e41798_d_n4;
        locals.var_t2_dn5 = assign36350_e41798_d_n5;
        locals.var_t2_dn6 = assign36350_e41798_d_n6;
        locals.var_t2_dn7 = assign36350_e41798_d_n7;
        locals.var_t2_dn8 = assign36350_e41798_d_n8;
        locals.var_t2_dn9 = assign36350_e41798_d_n9;
        locals.var_t2_dn10 = assign36350_e41798_d_n10;
        locals.var_t2_dn11 = assign36350_e41798_d_n11;
        locals.var_t2_dn14 = assign36350_e41798_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign36360_e41808, assign36360_e41808_d_n0, assign36360_e41808_d_n2, assign36360_e41808_d_n4, assign36360_e41808_d_n5, assign36360_e41808_d_n6, assign36360_e41808_d_n7, assign36360_e41808_d_n8, assign36360_e41808_d_n9, assign36360_e41808_d_n10, assign36360_e41808_d_n11, assign36360_e41808_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard825 != 0.0)) {
        let assign36360_e41806: f64 = (locals.var_beta * locals.var_ptl0);
        (assign36360_e41806, (locals.var_beta_dn0 * locals.var_ptl0), (locals.var_beta_dn2 * locals.var_ptl0), (locals.var_beta_dn4 * locals.var_ptl0), (locals.var_beta_dn5 * locals.var_ptl0), (locals.var_beta_dn6 * locals.var_ptl0), (locals.var_beta_dn7 * locals.var_ptl0), (locals.var_beta_dn8 * locals.var_ptl0), (locals.var_beta_dn9 * locals.var_ptl0), (locals.var_beta_dn10 * locals.var_ptl0), (locals.var_beta_dn11 * locals.var_ptl0), (locals.var_beta_dn14 * locals.var_ptl0),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign36360_e41808;
        locals.var_t0_dn0 = assign36360_e41808_d_n0;
        locals.var_t0_dn2 = assign36360_e41808_d_n2;
        locals.var_t0_dn4 = assign36360_e41808_d_n4;
        locals.var_t0_dn5 = assign36360_e41808_d_n5;
        locals.var_t0_dn6 = assign36360_e41808_d_n6;
        locals.var_t0_dn7 = assign36360_e41808_d_n7;
        locals.var_t0_dn8 = assign36360_e41808_d_n8;
        locals.var_t0_dn9 = assign36360_e41808_d_n9;
        locals.var_t0_dn10 = assign36360_e41808_d_n10;
        locals.var_t0_dn11 = assign36360_e41808_d_n11;
        locals.var_t0_dn14 = assign36360_e41808_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign36370_e41818, assign36370_e41818_d_n0, assign36370_e41818_d_n2, assign36370_e41818_d_n4, assign36370_e41818_d_n5, assign36370_e41818_d_n6, assign36370_e41818_d_n7, assign36370_e41818_d_n8, assign36370_e41818_d_n9, assign36370_e41818_d_n10, assign36370_e41818_d_n11, assign36370_e41818_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard825 != 0.0)) {
        let assign36370_e41816: f64 = (locals.var_cox * locals.var_t0);
        (assign36370_e41816, ((locals.var_cox_dn0 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn0)), ((locals.var_cox_dn2 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn2)), ((locals.var_cox_dn4 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn4)), ((locals.var_cox_dn5 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn5)), ((locals.var_cox_dn6 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn6)), ((locals.var_cox_dn7 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn7)), ((locals.var_cox_dn8 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn8)), ((locals.var_cox_dn9 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn9)), ((locals.var_cox_dn10 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn10)), ((locals.var_cox_dn11 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn11)), ((locals.var_cox_dn14 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign36370_e41818;
        locals.var_t3_dn0 = assign36370_e41818_d_n0;
        locals.var_t3_dn2 = assign36370_e41818_d_n2;
        locals.var_t3_dn4 = assign36370_e41818_d_n4;
        locals.var_t3_dn5 = assign36370_e41818_d_n5;
        locals.var_t3_dn6 = assign36370_e41818_d_n6;
        locals.var_t3_dn7 = assign36370_e41818_d_n7;
        locals.var_t3_dn8 = assign36370_e41818_d_n8;
        locals.var_t3_dn9 = assign36370_e41818_d_n9;
        locals.var_t3_dn10 = assign36370_e41818_d_n10;
        locals.var_t3_dn11 = assign36370_e41818_d_n11;
        locals.var_t3_dn14 = assign36370_e41818_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign36380_e41828, assign36380_e41828_d_n0, assign36380_e41828_d_n2, assign36380_e41828_d_n4, assign36380_e41828_d_n5, assign36380_e41828_d_n6, assign36380_e41828_d_n7, assign36380_e41828_d_n8, assign36380_e41828_d_n9, assign36380_e41828_d_n10, assign36380_e41828_d_n11, assign36380_e41828_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard825 != 0.0)) {
        let assign36380_e41826: f64 = (locals.var_t2).powf(p.p284);
        (assign36380_e41826, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn0)) } } else { (assign36380_e41826 * (p.p284 * (locals.var_t2_dn0 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn2)) } } else { (assign36380_e41826 * (p.p284 * (locals.var_t2_dn2 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn4)) } } else { (assign36380_e41826 * (p.p284 * (locals.var_t2_dn4 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn5)) } } else { (assign36380_e41826 * (p.p284 * (locals.var_t2_dn5 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn6)) } } else { (assign36380_e41826 * (p.p284 * (locals.var_t2_dn6 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn7)) } } else { (assign36380_e41826 * (p.p284 * (locals.var_t2_dn7 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn8)) } } else { (assign36380_e41826 * (p.p284 * (locals.var_t2_dn8 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn9)) } } else { (assign36380_e41826 * (p.p284 * (locals.var_t2_dn9 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn10)) } } else { (assign36380_e41826 * (p.p284 * (locals.var_t2_dn10 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn11)) } } else { (assign36380_e41826 * (p.p284 * (locals.var_t2_dn11 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn14)) } } else { (assign36380_e41826 * (p.p284 * (locals.var_t2_dn14 / locals.var_t2))) },)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign36380_e41828;
        locals.var_t0_dn0 = assign36380_e41828_d_n0;
        locals.var_t0_dn2 = assign36380_e41828_d_n2;
        locals.var_t0_dn4 = assign36380_e41828_d_n4;
        locals.var_t0_dn5 = assign36380_e41828_d_n5;
        locals.var_t0_dn6 = assign36380_e41828_d_n6;
        locals.var_t0_dn7 = assign36380_e41828_d_n7;
        locals.var_t0_dn8 = assign36380_e41828_d_n8;
        locals.var_t0_dn9 = assign36380_e41828_d_n9;
        locals.var_t0_dn10 = assign36380_e41828_d_n10;
        locals.var_t0_dn11 = assign36380_e41828_d_n11;
        locals.var_t0_dn14 = assign36380_e41828_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign36390_e41838, assign36390_e41838_d_n0, assign36390_e41838_d_n2, assign36390_e41838_d_n4, assign36390_e41838_d_n5, assign36390_e41838_d_n6, assign36390_e41838_d_n7, assign36390_e41838_d_n8, assign36390_e41838_d_n9, assign36390_e41838_d_n10, assign36390_e41838_d_n11, assign36390_e41838_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard825 != 0.0)) {
        let assign36390_e41836: f64 = (locals.var_t3 * locals.var_t0);
        (assign36390_e41836, ((locals.var_t3_dn0 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn0)), ((locals.var_t3_dn2 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn2)), ((locals.var_t3_dn4 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn4)), ((locals.var_t3_dn5 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn5)), ((locals.var_t3_dn6 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn6)), ((locals.var_t3_dn7 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn7)), ((locals.var_t3_dn8 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn8)), ((locals.var_t3_dn9 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn9)), ((locals.var_t3_dn10 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn10)), ((locals.var_t3_dn11 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn11)), ((locals.var_t3_dn14 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign36390_e41838;
        locals.var_t9_dn0 = assign36390_e41838_d_n0;
        locals.var_t9_dn2 = assign36390_e41838_d_n2;
        locals.var_t9_dn4 = assign36390_e41838_d_n4;
        locals.var_t9_dn5 = assign36390_e41838_d_n5;
        locals.var_t9_dn6 = assign36390_e41838_d_n6;
        locals.var_t9_dn7 = assign36390_e41838_d_n7;
        locals.var_t9_dn8 = assign36390_e41838_d_n8;
        locals.var_t9_dn9 = assign36390_e41838_d_n9;
        locals.var_t9_dn10 = assign36390_e41838_d_n10;
        locals.var_t9_dn11 = assign36390_e41838_d_n11;
        locals.var_t9_dn14 = assign36390_e41838_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign36400_e41850, assign36400_e41850_d_n0, assign36400_e41850_d_n2, assign36400_e41850_d_n4, assign36400_e41850_d_n5, assign36400_e41850_d_n6, assign36400_e41850_d_n7, assign36400_e41850_d_n8, assign36400_e41850_d_n9, assign36400_e41850_d_n10, assign36400_e41850_d_n11, assign36400_e41850_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard825 != 0.0)) {
        let assign36400_e41847: f64 = (locals.var_vdsz__blk443 * p.p285);
        let assign36400_e41848: f64 = (1.0 + assign36400_e41847);
        (assign36400_e41848, (locals.var_vdsz__blk443_dn0 * p.p285), (locals.var_vdsz__blk443_dn2 * p.p285), (locals.var_vdsz__blk443_dn4 * p.p285), (locals.var_vdsz__blk443_dn5 * p.p285), (locals.var_vdsz__blk443_dn6 * p.p285), (locals.var_vdsz__blk443_dn7 * p.p285), (locals.var_vdsz__blk443_dn8 * p.p285), (locals.var_vdsz__blk443_dn9 * p.p285), (locals.var_vdsz__blk443_dn10 * p.p285), (locals.var_vdsz__blk443_dn11 * p.p285), (locals.var_vdsz__blk443_dn14 * p.p285),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign36400_e41850;
        locals.var_t4_dn0 = assign36400_e41850_d_n0;
        locals.var_t4_dn2 = assign36400_e41850_d_n2;
        locals.var_t4_dn4 = assign36400_e41850_d_n4;
        locals.var_t4_dn5 = assign36400_e41850_d_n5;
        locals.var_t4_dn6 = assign36400_e41850_d_n6;
        locals.var_t4_dn7 = assign36400_e41850_d_n7;
        locals.var_t4_dn8 = assign36400_e41850_d_n8;
        locals.var_t4_dn9 = assign36400_e41850_d_n9;
        locals.var_t4_dn10 = assign36400_e41850_d_n10;
        locals.var_t4_dn11 = assign36400_e41850_d_n11;
        locals.var_t4_dn14 = assign36400_e41850_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign36410_e41858, assign36410_e41858_d_n0, assign36410_e41858_d_n2, assign36410_e41858_d_n4, assign36410_e41858_d_n5, assign36410_e41858_d_n6, assign36410_e41858_d_n7, assign36410_e41858_d_n8, assign36410_e41858_d_n9, assign36410_e41858_d_n10, assign36410_e41858_d_n11, assign36410_e41858_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard825 != 0.0)) {
        (locals.var_pt40, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign36410_e41858;
        locals.var_t0_dn0 = assign36410_e41858_d_n0;
        locals.var_t0_dn2 = assign36410_e41858_d_n2;
        locals.var_t0_dn4 = assign36410_e41858_d_n4;
        locals.var_t0_dn5 = assign36410_e41858_d_n5;
        locals.var_t0_dn6 = assign36410_e41858_d_n6;
        locals.var_t0_dn7 = assign36410_e41858_d_n7;
        locals.var_t0_dn8 = assign36410_e41858_d_n8;
        locals.var_t0_dn9 = assign36410_e41858_d_n9;
        locals.var_t0_dn10 = assign36410_e41858_d_n10;
        locals.var_t0_dn11 = assign36410_e41858_d_n11;
        locals.var_t0_dn14 = assign36410_e41858_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign36420_e41870, assign36420_e41870_d_n0, assign36420_e41870_d_n2, assign36420_e41870_d_n4, assign36420_e41870_d_n5, assign36420_e41870_d_n6, assign36420_e41870_d_n7, assign36420_e41870_d_n8, assign36420_e41870_d_n9, assign36420_e41870_d_n10, assign36420_e41870_d_n11, assign36420_e41870_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard825 != 0.0)) {
        let assign36420_e41866: f64 = (locals.var_phi_s0_dep + locals.var_t6);
        let assign36420_e41868: f64 = (assign36420_e41866 - locals.var_vbsz__blk442);
        (assign36420_e41868, ((locals.var_phi_s0_dep_dn0 + locals.var_t6_dn0) - locals.var_vbsz__blk442_dn0), ((locals.var_phi_s0_dep_dn2 + locals.var_t6_dn2) - locals.var_vbsz__blk442_dn2), ((locals.var_phi_s0_dep_dn4 + locals.var_t6_dn4) - locals.var_vbsz__blk442_dn4), ((locals.var_phi_s0_dep_dn5 + locals.var_t6_dn5) - locals.var_vbsz__blk442_dn5), ((locals.var_phi_s0_dep_dn6 + locals.var_t6_dn6) - locals.var_vbsz__blk442_dn6), ((locals.var_phi_s0_dep_dn7 + locals.var_t6_dn7) - locals.var_vbsz__blk442_dn7), ((locals.var_phi_s0_dep_dn8 + locals.var_t6_dn8) - locals.var_vbsz__blk442_dn8), ((locals.var_phi_s0_dep_dn9 + locals.var_t6_dn9) - locals.var_vbsz__blk442_dn9), ((locals.var_phi_s0_dep_dn10 + locals.var_t6_dn10) - locals.var_vbsz__blk442_dn10), ((locals.var_phi_s0_dep_dn11 + locals.var_t6_dn11) - locals.var_vbsz__blk442_dn11), ((locals.var_phi_s0_dep_dn14 + locals.var_t6_dn14) - locals.var_vbsz__blk442_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign36420_e41870;
        locals.var_t5_dn0 = assign36420_e41870_d_n0;
        locals.var_t5_dn2 = assign36420_e41870_d_n2;
        locals.var_t5_dn4 = assign36420_e41870_d_n4;
        locals.var_t5_dn5 = assign36420_e41870_d_n5;
        locals.var_t5_dn6 = assign36420_e41870_d_n6;
        locals.var_t5_dn7 = assign36420_e41870_d_n7;
        locals.var_t5_dn8 = assign36420_e41870_d_n8;
        locals.var_t5_dn9 = assign36420_e41870_d_n9;
        locals.var_t5_dn10 = assign36420_e41870_d_n10;
        locals.var_t5_dn11 = assign36420_e41870_d_n11;
        locals.var_t5_dn14 = assign36420_e41870_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign36430_e41884, assign36430_e41884_d_n0, assign36430_e41884_d_n2, assign36430_e41884_d_n4, assign36430_e41884_d_n5, assign36430_e41884_d_n6, assign36430_e41884_d_n7, assign36430_e41884_d_n8, assign36430_e41884_d_n9, assign36430_e41884_d_n10, assign36430_e41884_d_n11, assign36430_e41884_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && (locals.var_guard448 != 0.0)) && (locals.var_guard825 != 0.0)) {
        let assign36430_e41879: f64 = (locals.var_vdsz__blk443 * locals.var_t0);
        let assign36430_e41881: f64 = (assign36430_e41879 * locals.var_t5);
        let assign36430_e41882: f64 = (locals.var_t4 + assign36430_e41881);
        (assign36430_e41882, (locals.var_t4_dn0 + ((((locals.var_vdsz__blk443_dn0 * locals.var_t0) + (locals.var_vdsz__blk443 * locals.var_t0_dn0)) * locals.var_t5) + (assign36430_e41879 * locals.var_t5_dn0))), (locals.var_t4_dn2 + ((((locals.var_vdsz__blk443_dn2 * locals.var_t0) + (locals.var_vdsz__blk443 * locals.var_t0_dn2)) * locals.var_t5) + (assign36430_e41879 * locals.var_t5_dn2))), (locals.var_t4_dn4 + ((((locals.var_vdsz__blk443_dn4 * locals.var_t0) + (locals.var_vdsz__blk443 * locals.var_t0_dn4)) * locals.var_t5) + (assign36430_e41879 * locals.var_t5_dn4))), (locals.var_t4_dn5 + ((((locals.var_vdsz__blk443_dn5 * locals.var_t0) + (locals.var_vdsz__blk443 * locals.var_t0_dn5)) * locals.var_t5) + (assign36430_e41879 * locals.var_t5_dn5))), (locals.var_t4_dn6 + ((((locals.var_vdsz__blk443_dn6 * locals.var_t0) + (locals.var_vdsz__blk443 * locals.var_t0_dn6)) * locals.var_t5) + (assign36430_e41879 * locals.var_t5_dn6))), (locals.var_t4_dn7 + ((((locals.var_vdsz__blk443_dn7 * locals.var_t0) + (locals.var_vdsz__blk443 * locals.var_t0_dn7)) * locals.var_t5) + (assign36430_e41879 * locals.var_t5_dn7))), (locals.var_t4_dn8 + ((((locals.var_vdsz__blk443_dn8 * locals.var_t0) + (locals.var_vdsz__blk443 * locals.var_t0_dn8)) * locals.var_t5) + (assign36430_e41879 * locals.var_t5_dn8))), (locals.var_t4_dn9 + ((((locals.var_vdsz__blk443_dn9 * locals.var_t0) + (locals.var_vdsz__blk443 * locals.var_t0_dn9)) * locals.var_t5) + (assign36430_e41879 * locals.var_t5_dn9))), (locals.var_t4_dn10 + ((((locals.var_vdsz__blk443_dn10 * locals.var_t0) + (locals.var_vdsz__blk443 * locals.var_t0_dn10)) * locals.var_t5) + (assign36430_e41879 * locals.var_t5_dn10))), (locals.var_t4_dn11 + ((((locals.var_vdsz__blk443_dn11 * locals.var_t0) + (locals.var_vdsz__blk443 * locals.var_t0_dn11)) * locals.var_t5) + (assign36430_e41879 * locals.var_t5_dn11))), (locals.var_t4_dn14 + ((((locals.var_vdsz__blk443_dn14 * locals.var_t0) + (locals.var_vdsz__blk443 * locals.var_t0_dn14)) * locals.var_t5) + (assign36430_e41879 * locals.var_t5_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign36430_e41884;
        locals.var_t4_dn0 = assign36430_e41884_d_n0;
        locals.var_t4_dn2 = assign36430_e41884_d_n2;
        locals.var_t4_dn4 = assign36430_e41884_d_n4;
        locals.var_t4_dn5 = assign36430_e41884_d_n5;
        locals.var_t4_dn6 = assign36430_e41884_d_n6;
        locals.var_t4_dn7 = assign36430_e41884_d_n7;
        locals.var_t4_dn8 = assign36430_e41884_d_n8;
        locals.var_t4_dn9 = assign36430_e41884_d_n9;
        locals.var_t4_dn10 = assign36430_e41884_d_n10;
        locals.var_t4_dn11 = assign36430_e41884_d_n11;
        locals.var_t4_dn14 = assign36430_e41884_d_n14;
        locals.var_t4_rv = 0.0;

    }
}

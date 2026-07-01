#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_104(
        locals: &mut StampLocals,
    ) {
        let (assign32300_e36491, assign32300_e36491_d_n0, assign32300_e36491_d_n2, assign32300_e36491_d_n4, assign32300_e36491_d_n5, assign32300_e36491_d_n6, assign32300_e36491_d_n7, assign32300_e36491_d_n8, assign32300_e36491_d_n9, assign32300_e36491_d_n10, assign32300_e36491_d_n11, assign32300_e36491_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard767 != 0.0)) && (locals.var_guard768 != 0.0)) {
        let assign32300_e36485: f64 = (0.02 * locals.var_xmp);
        let assign32300_e36487: f64 = (assign32300_e36485 * locals.var_dnm);
        let assign32300_e36489: f64 = (assign32300_e36487 / locals.var_arg);
        (assign32300_e36489, ((((((0.02 * locals.var_xmp_dn0) * locals.var_dnm) + (assign32300_e36485 * locals.var_dnm_dn0)) * locals.var_arg) - (assign32300_e36487 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn2) * locals.var_dnm) + (assign32300_e36485 * locals.var_dnm_dn2)) * locals.var_arg) - (assign32300_e36487 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn4) * locals.var_dnm) + (assign32300_e36485 * locals.var_dnm_dn4)) * locals.var_arg) - (assign32300_e36487 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn5) * locals.var_dnm) + (assign32300_e36485 * locals.var_dnm_dn5)) * locals.var_arg) - (assign32300_e36487 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn6) * locals.var_dnm) + (assign32300_e36485 * locals.var_dnm_dn6)) * locals.var_arg) - (assign32300_e36487 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn7) * locals.var_dnm) + (assign32300_e36485 * locals.var_dnm_dn7)) * locals.var_arg) - (assign32300_e36487 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn8) * locals.var_dnm) + (assign32300_e36485 * locals.var_dnm_dn8)) * locals.var_arg) - (assign32300_e36487 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn9) * locals.var_dnm) + (assign32300_e36485 * locals.var_dnm_dn9)) * locals.var_arg) - (assign32300_e36487 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn10) * locals.var_dnm) + (assign32300_e36485 * locals.var_dnm_dn10)) * locals.var_arg) - (assign32300_e36487 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn11) * locals.var_dnm) + (assign32300_e36485 * locals.var_dnm_dn11)) * locals.var_arg) - (assign32300_e36487 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.02 * locals.var_xmp_dn14) * locals.var_dnm) + (assign32300_e36485 * locals.var_dnm_dn14)) * locals.var_arg) - (assign32300_e36487 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign32300_e36491;
        locals.var_t1_dn0 = assign32300_e36491_d_n0;
        locals.var_t1_dn2 = assign32300_e36491_d_n2;
        locals.var_t1_dn4 = assign32300_e36491_d_n4;
        locals.var_t1_dn5 = assign32300_e36491_d_n5;
        locals.var_t1_dn6 = assign32300_e36491_d_n6;
        locals.var_t1_dn7 = assign32300_e36491_d_n7;
        locals.var_t1_dn8 = assign32300_e36491_d_n8;
        locals.var_t1_dn9 = assign32300_e36491_d_n9;
        locals.var_t1_dn10 = assign32300_e36491_d_n10;
        locals.var_t1_dn11 = assign32300_e36491_d_n11;
        locals.var_t1_dn14 = assign32300_e36491_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign32310_e36508, assign32310_e36508_d_n0, assign32310_e36508_d_n2, assign32310_e36508_d_n4, assign32310_e36508_d_n5, assign32310_e36508_d_n6, assign32310_e36508_d_n7, assign32310_e36508_d_n8, assign32310_e36508_d_n9, assign32310_e36508_d_n10, assign32310_e36508_d_n11, assign32310_e36508_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard767 != 0.0)) && (locals.var_guard768 != 0.0)) {
        let assign32310_e36504: f64 = (locals.var_phi_sl_dep - 0.02);
        let assign32310_e36506: f64 = (assign32310_e36504 + locals.var_tmf0);
        (assign32310_e36506, (locals.var_phi_sl_dep_dn0 + locals.var_tmf0_dn0), (locals.var_phi_sl_dep_dn2 + locals.var_tmf0_dn2), (locals.var_phi_sl_dep_dn4 + locals.var_tmf0_dn4), (locals.var_phi_sl_dep_dn5 + locals.var_tmf0_dn5), (locals.var_phi_sl_dep_dn6 + locals.var_tmf0_dn6), (locals.var_phi_sl_dep_dn7 + locals.var_tmf0_dn7), (locals.var_phi_sl_dep_dn8 + locals.var_tmf0_dn8), (locals.var_phi_sl_dep_dn9 + locals.var_tmf0_dn9), (locals.var_phi_sl_dep_dn10 + locals.var_tmf0_dn10), (locals.var_phi_sl_dep_dn11 + locals.var_tmf0_dn11), (locals.var_phi_sl_dep_dn14 + locals.var_tmf0_dn14),)
    } else {
        (locals.var_phi_bl_dep, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    }
};
        locals.var_phi_bl_dep = assign32310_e36508;
        locals.var_phi_bl_dep_dn0 = assign32310_e36508_d_n0;
        locals.var_phi_bl_dep_dn2 = assign32310_e36508_d_n2;
        locals.var_phi_bl_dep_dn4 = assign32310_e36508_d_n4;
        locals.var_phi_bl_dep_dn5 = assign32310_e36508_d_n5;
        locals.var_phi_bl_dep_dn6 = assign32310_e36508_d_n6;
        locals.var_phi_bl_dep_dn7 = assign32310_e36508_d_n7;
        locals.var_phi_bl_dep_dn8 = assign32310_e36508_d_n8;
        locals.var_phi_bl_dep_dn9 = assign32310_e36508_d_n9;
        locals.var_phi_bl_dep_dn10 = assign32310_e36508_d_n10;
        locals.var_phi_bl_dep_dn11 = assign32310_e36508_d_n11;
        locals.var_phi_bl_dep_dn14 = assign32310_e36508_d_n14;
        locals.var_phi_bl_dep_rv = 0.0;

        let (assign32320_e36521, assign32320_e36521_d_n0, assign32320_e36521_d_n2, assign32320_e36521_d_n4, assign32320_e36521_d_n5, assign32320_e36521_d_n6, assign32320_e36521_d_n7, assign32320_e36521_d_n8, assign32320_e36521_d_n9, assign32320_e36521_d_n10, assign32320_e36521_d_n11, assign32320_e36521_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard767 != 0.0)) && (locals.var_guard768 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign32320_e36521;
        locals.var_t1_dn0 = assign32320_e36521_d_n0;
        locals.var_t1_dn2 = assign32320_e36521_d_n2;
        locals.var_t1_dn4 = assign32320_e36521_d_n4;
        locals.var_t1_dn5 = assign32320_e36521_d_n5;
        locals.var_t1_dn6 = assign32320_e36521_d_n6;
        locals.var_t1_dn7 = assign32320_e36521_d_n7;
        locals.var_t1_dn8 = assign32320_e36521_d_n8;
        locals.var_t1_dn9 = assign32320_e36521_d_n9;
        locals.var_t1_dn10 = assign32320_e36521_d_n10;
        locals.var_t1_dn11 = assign32320_e36521_d_n11;
        locals.var_t1_dn14 = assign32320_e36521_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign32330_e36535, assign32330_e36535_d_n0, assign32330_e36535_d_n2, assign32330_e36535_d_n4, assign32330_e36535_d_n5, assign32330_e36535_d_n6, assign32330_e36535_d_n7, assign32330_e36535_d_n8, assign32330_e36535_d_n9, assign32330_e36535_d_n10, assign32330_e36535_d_n11, assign32330_e36535_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard767 != 0.0)) && (locals.var_guard768 == 0.0)) {
        (locals.var_phi_bl_dep, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    } else {
        (locals.var_phi_bl_dep, locals.var_phi_bl_dep_dn0, locals.var_phi_bl_dep_dn2, locals.var_phi_bl_dep_dn4, locals.var_phi_bl_dep_dn5, locals.var_phi_bl_dep_dn6, locals.var_phi_bl_dep_dn7, locals.var_phi_bl_dep_dn8, locals.var_phi_bl_dep_dn9, locals.var_phi_bl_dep_dn10, locals.var_phi_bl_dep_dn11, locals.var_phi_bl_dep_dn14,)
    }
};
        locals.var_phi_bl_dep = assign32330_e36535;
        locals.var_phi_bl_dep_dn0 = assign32330_e36535_d_n0;
        locals.var_phi_bl_dep_dn2 = assign32330_e36535_d_n2;
        locals.var_phi_bl_dep_dn4 = assign32330_e36535_d_n4;
        locals.var_phi_bl_dep_dn5 = assign32330_e36535_d_n5;
        locals.var_phi_bl_dep_dn6 = assign32330_e36535_d_n6;
        locals.var_phi_bl_dep_dn7 = assign32330_e36535_d_n7;
        locals.var_phi_bl_dep_dn8 = assign32330_e36535_d_n8;
        locals.var_phi_bl_dep_dn9 = assign32330_e36535_d_n9;
        locals.var_phi_bl_dep_dn10 = assign32330_e36535_d_n10;
        locals.var_phi_bl_dep_dn11 = assign32330_e36535_d_n11;
        locals.var_phi_bl_dep_dn14 = assign32330_e36535_d_n14;
        locals.var_phi_bl_dep_rv = 0.0;

        let (assign32340_e36549, assign32340_e36549_d_n0, assign32340_e36549_d_n2, assign32340_e36549_d_n4, assign32340_e36549_d_n5, assign32340_e36549_d_n6, assign32340_e36549_d_n7, assign32340_e36549_d_n8, assign32340_e36549_d_n9, assign32340_e36549_d_n10, assign32340_e36549_d_n11, assign32340_e36549_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard767 != 0.0)) && (locals.var_guard768 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign32340_e36549;
        locals.var_t1_dn0 = assign32340_e36549_d_n0;
        locals.var_t1_dn2 = assign32340_e36549_d_n2;
        locals.var_t1_dn4 = assign32340_e36549_d_n4;
        locals.var_t1_dn5 = assign32340_e36549_d_n5;
        locals.var_t1_dn6 = assign32340_e36549_d_n6;
        locals.var_t1_dn7 = assign32340_e36549_d_n7;
        locals.var_t1_dn8 = assign32340_e36549_d_n8;
        locals.var_t1_dn9 = assign32340_e36549_d_n9;
        locals.var_t1_dn10 = assign32340_e36549_d_n10;
        locals.var_t1_dn11 = assign32340_e36549_d_n11;
        locals.var_t1_dn14 = assign32340_e36549_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign32350_e36566, assign32350_e36566_d_n0, assign32350_e36566_d_n2, assign32350_e36566_d_n4, assign32350_e36566_d_n5, assign32350_e36566_d_n6, assign32350_e36566_d_n7, assign32350_e36566_d_n8, assign32350_e36566_d_n9, assign32350_e36566_d_n10, assign32350_e36566_d_n11, assign32350_e36566_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) {
        let assign32350_e36559: f64 = (locals.var_ndepmpnsub * locals.var_phi_bl_dep);
        let assign32350_e36561: f64 = (assign32350_e36559 + locals.var_vbscl__blk437);
        let assign32350_e36563: f64 = (assign32350_e36561 - locals.var_vbi_dep);
        let assign32350_e36564: f64 = (locals.var_ndepmpnsub_inv1 * assign32350_e36563);
        (assign32350_e36564, ((locals.var_ndepmpnsub_inv1_dn0 * assign32350_e36563) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn0 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn0)) + locals.var_vbscl__blk437_dn0) - locals.var_vbi_dep_dn0))), ((locals.var_ndepmpnsub_inv1_dn2 * assign32350_e36563) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn2 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn2)) + locals.var_vbscl__blk437_dn2) - locals.var_vbi_dep_dn2))), ((locals.var_ndepmpnsub_inv1_dn4 * assign32350_e36563) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn4 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn4)) + locals.var_vbscl__blk437_dn4) - locals.var_vbi_dep_dn4))), ((locals.var_ndepmpnsub_inv1_dn5 * assign32350_e36563) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn5 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn5)) + locals.var_vbscl__blk437_dn5) - locals.var_vbi_dep_dn5))), ((locals.var_ndepmpnsub_inv1_dn6 * assign32350_e36563) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn6 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn6)) + locals.var_vbscl__blk437_dn6) - locals.var_vbi_dep_dn6))), ((locals.var_ndepmpnsub_inv1_dn7 * assign32350_e36563) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn7 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn7)) + locals.var_vbscl__blk437_dn7) - locals.var_vbi_dep_dn7))), ((locals.var_ndepmpnsub_inv1_dn8 * assign32350_e36563) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn8 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn8)) + locals.var_vbscl__blk437_dn8) - locals.var_vbi_dep_dn8))), ((locals.var_ndepmpnsub_inv1_dn9 * assign32350_e36563) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn9 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn9)) + locals.var_vbscl__blk437_dn9) - locals.var_vbi_dep_dn9))), ((locals.var_ndepmpnsub_inv1_dn10 * assign32350_e36563) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn10 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn10)) + locals.var_vbscl__blk437_dn10) - locals.var_vbi_dep_dn10))), ((locals.var_ndepmpnsub_inv1_dn11 * assign32350_e36563) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn11 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn11)) + locals.var_vbscl__blk437_dn11) - locals.var_vbi_dep_dn11))), ((locals.var_ndepmpnsub_inv1_dn14 * assign32350_e36563) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn14 * locals.var_phi_bl_dep) + (locals.var_ndepmpnsub * locals.var_phi_bl_dep_dn14)) + locals.var_vbscl__blk437_dn14) - locals.var_vbi_dep_dn14))),)
    } else {
        (locals.var_phi_jl_dep, locals.var_phi_jl_dep_dn0, locals.var_phi_jl_dep_dn2, locals.var_phi_jl_dep_dn4, locals.var_phi_jl_dep_dn5, locals.var_phi_jl_dep_dn6, locals.var_phi_jl_dep_dn7, locals.var_phi_jl_dep_dn8, locals.var_phi_jl_dep_dn9, locals.var_phi_jl_dep_dn10, locals.var_phi_jl_dep_dn11, locals.var_phi_jl_dep_dn14,)
    }
};
        locals.var_phi_jl_dep = assign32350_e36566;
        locals.var_phi_jl_dep_dn0 = assign32350_e36566_d_n0;
        locals.var_phi_jl_dep_dn2 = assign32350_e36566_d_n2;
        locals.var_phi_jl_dep_dn4 = assign32350_e36566_d_n4;
        locals.var_phi_jl_dep_dn5 = assign32350_e36566_d_n5;
        locals.var_phi_jl_dep_dn6 = assign32350_e36566_d_n6;
        locals.var_phi_jl_dep_dn7 = assign32350_e36566_d_n7;
        locals.var_phi_jl_dep_dn8 = assign32350_e36566_d_n8;
        locals.var_phi_jl_dep_dn9 = assign32350_e36566_d_n9;
        locals.var_phi_jl_dep_dn10 = assign32350_e36566_d_n10;
        locals.var_phi_jl_dep_dn11 = assign32350_e36566_d_n11;
        locals.var_phi_jl_dep_dn14 = assign32350_e36566_d_n14;
        locals.var_phi_jl_dep_rv = 0.0;

        let (assign32360_e36579, assign32360_e36579_d_n0, assign32360_e36579_d_n2, assign32360_e36579_d_n4, assign32360_e36579_d_n5, assign32360_e36579_d_n6, assign32360_e36579_d_n7, assign32360_e36579_d_n8, assign32360_e36579_d_n9, assign32360_e36579_d_n10, assign32360_e36579_d_n11, assign32360_e36579_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) {
        let assign32360_e36576: f64 = (locals.var_phi_sl_dep - locals.var_phi_bl_dep);
        let assign32360_e36577: f64 = (locals.var_beta * assign32360_e36576);
        (assign32360_e36577, ((locals.var_beta_dn0 * assign32360_e36576) + (locals.var_beta * (locals.var_phi_sl_dep_dn0 - locals.var_phi_bl_dep_dn0))), ((locals.var_beta_dn2 * assign32360_e36576) + (locals.var_beta * (locals.var_phi_sl_dep_dn2 - locals.var_phi_bl_dep_dn2))), ((locals.var_beta_dn4 * assign32360_e36576) + (locals.var_beta * (locals.var_phi_sl_dep_dn4 - locals.var_phi_bl_dep_dn4))), ((locals.var_beta_dn5 * assign32360_e36576) + (locals.var_beta * (locals.var_phi_sl_dep_dn5 - locals.var_phi_bl_dep_dn5))), ((locals.var_beta_dn6 * assign32360_e36576) + (locals.var_beta * (locals.var_phi_sl_dep_dn6 - locals.var_phi_bl_dep_dn6))), ((locals.var_beta_dn7 * assign32360_e36576) + (locals.var_beta * (locals.var_phi_sl_dep_dn7 - locals.var_phi_bl_dep_dn7))), ((locals.var_beta_dn8 * assign32360_e36576) + (locals.var_beta * (locals.var_phi_sl_dep_dn8 - locals.var_phi_bl_dep_dn8))), ((locals.var_beta_dn9 * assign32360_e36576) + (locals.var_beta * (locals.var_phi_sl_dep_dn9 - locals.var_phi_bl_dep_dn9))), ((locals.var_beta_dn10 * assign32360_e36576) + (locals.var_beta * (locals.var_phi_sl_dep_dn10 - locals.var_phi_bl_dep_dn10))), ((locals.var_beta_dn11 * assign32360_e36576) + (locals.var_beta * (locals.var_phi_sl_dep_dn11 - locals.var_phi_bl_dep_dn11))), ((locals.var_beta_dn14 * assign32360_e36576) + (locals.var_beta * (locals.var_phi_sl_dep_dn14 - locals.var_phi_bl_dep_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign32360_e36579;
        locals.var_t1_dn0 = assign32360_e36579_d_n0;
        locals.var_t1_dn2 = assign32360_e36579_d_n2;
        locals.var_t1_dn4 = assign32360_e36579_d_n4;
        locals.var_t1_dn5 = assign32360_e36579_d_n5;
        locals.var_t1_dn6 = assign32360_e36579_d_n6;
        locals.var_t1_dn7 = assign32360_e36579_d_n7;
        locals.var_t1_dn8 = assign32360_e36579_d_n8;
        locals.var_t1_dn9 = assign32360_e36579_d_n9;
        locals.var_t1_dn10 = assign32360_e36579_d_n10;
        locals.var_t1_dn11 = assign32360_e36579_d_n11;
        locals.var_t1_dn14 = assign32360_e36579_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign32370_e36589, assign32370_e36589_d_n0, assign32370_e36589_d_n2, assign32370_e36589_d_n4, assign32370_e36589_d_n5, assign32370_e36589_d_n6, assign32370_e36589_d_n7, assign32370_e36589_d_n8, assign32370_e36589_d_n9, assign32370_e36589_d_n10, assign32370_e36589_d_n11, assign32370_e36589_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) {
        let assign32370_e36587: f64 = (locals.var_t1).exp();
        (assign32370_e36587, (assign32370_e36587 * locals.var_t1_dn0), (assign32370_e36587 * locals.var_t1_dn2), (assign32370_e36587 * locals.var_t1_dn4), (assign32370_e36587 * locals.var_t1_dn5), (assign32370_e36587 * locals.var_t1_dn6), (assign32370_e36587 * locals.var_t1_dn7), (assign32370_e36587 * locals.var_t1_dn8), (assign32370_e36587 * locals.var_t1_dn9), (assign32370_e36587 * locals.var_t1_dn10), (assign32370_e36587 * locals.var_t1_dn11), (assign32370_e36587 * locals.var_t1_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign32370_e36589;
        locals.var_t2_dn0 = assign32370_e36589_d_n0;
        locals.var_t2_dn2 = assign32370_e36589_d_n2;
        locals.var_t2_dn4 = assign32370_e36589_d_n4;
        locals.var_t2_dn5 = assign32370_e36589_d_n5;
        locals.var_t2_dn6 = assign32370_e36589_d_n6;
        locals.var_t2_dn7 = assign32370_e36589_d_n7;
        locals.var_t2_dn8 = assign32370_e36589_d_n8;
        locals.var_t2_dn9 = assign32370_e36589_d_n9;
        locals.var_t2_dn10 = assign32370_e36589_d_n10;
        locals.var_t2_dn11 = assign32370_e36589_d_n11;
        locals.var_t2_dn14 = assign32370_e36589_d_n14;
        locals.var_t2_rv = 0.0;

        let assign32380_e36592: f64 = if locals.var_phi_sl_dep >= locals.var_phi_bl_dep { 1.0 } else { 0.0 };
        locals.var_guard774 = assign32380_e36592;
        locals.var_guard774_rv = 0.0;

        let (assign32390_e36613, assign32390_e36613_d_n0, assign32390_e36613_d_n2, assign32390_e36613_d_n4, assign32390_e36613_d_n5, assign32390_e36613_d_n6, assign32390_e36613_d_n7, assign32390_e36613_d_n8, assign32390_e36613_d_n9, assign32390_e36613_d_n10, assign32390_e36613_d_n11, assign32390_e36613_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) {
        let assign32390_e36602: f64 = (-locals.var_cnst0);
        let assign32390_e36605: f64 = (locals.var_t2 - 1.0);
        let assign32390_e36607: f64 = (assign32390_e36605 - locals.var_t1);
        let assign32390_e36609: f64 = (assign32390_e36607 + 1e-15);
        let assign32390_e36610: f64 = (assign32390_e36609).sqrt();
        let assign32390_e36611: f64 = (assign32390_e36602 * assign32390_e36610);
        (assign32390_e36611, (((-locals.var_cnst0_dn0) * assign32390_e36610) + (assign32390_e36602 * ((locals.var_t2_dn0 - locals.var_t1_dn0) / (2.0 * assign32390_e36610)))), (((-locals.var_cnst0_dn2) * assign32390_e36610) + (assign32390_e36602 * ((locals.var_t2_dn2 - locals.var_t1_dn2) / (2.0 * assign32390_e36610)))), (((-locals.var_cnst0_dn4) * assign32390_e36610) + (assign32390_e36602 * ((locals.var_t2_dn4 - locals.var_t1_dn4) / (2.0 * assign32390_e36610)))), (((-locals.var_cnst0_dn5) * assign32390_e36610) + (assign32390_e36602 * ((locals.var_t2_dn5 - locals.var_t1_dn5) / (2.0 * assign32390_e36610)))), (((-locals.var_cnst0_dn6) * assign32390_e36610) + (assign32390_e36602 * ((locals.var_t2_dn6 - locals.var_t1_dn6) / (2.0 * assign32390_e36610)))), (((-locals.var_cnst0_dn7) * assign32390_e36610) + (assign32390_e36602 * ((locals.var_t2_dn7 - locals.var_t1_dn7) / (2.0 * assign32390_e36610)))), (((-locals.var_cnst0_dn8) * assign32390_e36610) + (assign32390_e36602 * ((locals.var_t2_dn8 - locals.var_t1_dn8) / (2.0 * assign32390_e36610)))), (((-locals.var_cnst0_dn9) * assign32390_e36610) + (assign32390_e36602 * ((locals.var_t2_dn9 - locals.var_t1_dn9) / (2.0 * assign32390_e36610)))), (((-locals.var_cnst0_dn10) * assign32390_e36610) + (assign32390_e36602 * ((locals.var_t2_dn10 - locals.var_t1_dn10) / (2.0 * assign32390_e36610)))), (((-locals.var_cnst0_dn11) * assign32390_e36610) + (assign32390_e36602 * ((locals.var_t2_dn11 - locals.var_t1_dn11) / (2.0 * assign32390_e36610)))), (((-locals.var_cnst0_dn14) * assign32390_e36610) + (assign32390_e36602 * ((locals.var_t2_dn14 - locals.var_t1_dn14) / (2.0 * assign32390_e36610)))),)
    } else {
        (locals.var_q_sl, locals.var_q_sl_dn0, locals.var_q_sl_dn2, locals.var_q_sl_dn4, locals.var_q_sl_dn5, locals.var_q_sl_dn6, locals.var_q_sl_dn7, locals.var_q_sl_dn8, locals.var_q_sl_dn9, locals.var_q_sl_dn10, locals.var_q_sl_dn11, locals.var_q_sl_dn14,)
    }
};
        locals.var_q_sl = assign32390_e36613;
        locals.var_q_sl_dn0 = assign32390_e36613_d_n0;
        locals.var_q_sl_dn2 = assign32390_e36613_d_n2;
        locals.var_q_sl_dn4 = assign32390_e36613_d_n4;
        locals.var_q_sl_dn5 = assign32390_e36613_d_n5;
        locals.var_q_sl_dn6 = assign32390_e36613_d_n6;
        locals.var_q_sl_dn7 = assign32390_e36613_d_n7;
        locals.var_q_sl_dn8 = assign32390_e36613_d_n8;
        locals.var_q_sl_dn9 = assign32390_e36613_d_n9;
        locals.var_q_sl_dn10 = assign32390_e36613_d_n10;
        locals.var_q_sl_dn11 = assign32390_e36613_d_n11;
        locals.var_q_sl_dn14 = assign32390_e36613_d_n14;
        locals.var_q_sl_rv = 0.0;

        let (assign32400_e36624, assign32400_e36624_d_n0, assign32400_e36624_d_n2, assign32400_e36624_d_n4, assign32400_e36624_d_n5, assign32400_e36624_d_n6, assign32400_e36624_d_n7, assign32400_e36624_d_n8, assign32400_e36624_d_n9, assign32400_e36624_d_n10, assign32400_e36624_d_n11, assign32400_e36624_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) {
        (locals.var_q_sl, locals.var_q_sl_dn0, locals.var_q_sl_dn2, locals.var_q_sl_dn4, locals.var_q_sl_dn5, locals.var_q_sl_dn6, locals.var_q_sl_dn7, locals.var_q_sl_dn8, locals.var_q_sl_dn9, locals.var_q_sl_dn10, locals.var_q_sl_dn11, locals.var_q_sl_dn14,)
    } else {
        (locals.var_q_nl, locals.var_q_nl_dn0, locals.var_q_nl_dn2, locals.var_q_nl_dn4, locals.var_q_nl_dn5, locals.var_q_nl_dn6, locals.var_q_nl_dn7, locals.var_q_nl_dn8, locals.var_q_nl_dn9, locals.var_q_nl_dn10, locals.var_q_nl_dn11, locals.var_q_nl_dn14,)
    }
};
        locals.var_q_nl = assign32400_e36624;
        locals.var_q_nl_dn0 = assign32400_e36624_d_n0;
        locals.var_q_nl_dn2 = assign32400_e36624_d_n2;
        locals.var_q_nl_dn4 = assign32400_e36624_d_n4;
        locals.var_q_nl_dn5 = assign32400_e36624_d_n5;
        locals.var_q_nl_dn6 = assign32400_e36624_d_n6;
        locals.var_q_nl_dn7 = assign32400_e36624_d_n7;
        locals.var_q_nl_dn8 = assign32400_e36624_d_n8;
        locals.var_q_nl_dn9 = assign32400_e36624_d_n9;
        locals.var_q_nl_dn10 = assign32400_e36624_d_n10;
        locals.var_q_nl_dn11 = assign32400_e36624_d_n11;
        locals.var_q_nl_dn14 = assign32400_e36624_d_n14;
        locals.var_q_nl_rv = 0.0;

        let (assign32410_e36635, assign32410_e36635_d_n0, assign32410_e36635_d_n2, assign32410_e36635_d_n4, assign32410_e36635_d_n5, assign32410_e36635_d_n6, assign32410_e36635_d_n7, assign32410_e36635_d_n8, assign32410_e36635_d_n9, assign32410_e36635_d_n10, assign32410_e36635_d_n11, assign32410_e36635_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_sl_dep, locals.var_q_sl_dep_dn0, locals.var_q_sl_dep_dn2, locals.var_q_sl_dep_dn4, locals.var_q_sl_dep_dn5, locals.var_q_sl_dep_dn6, locals.var_q_sl_dep_dn7, locals.var_q_sl_dep_dn8, locals.var_q_sl_dep_dn9, locals.var_q_sl_dep_dn10, locals.var_q_sl_dep_dn11, locals.var_q_sl_dep_dn14,)
    }
};
        locals.var_q_sl_dep = assign32410_e36635;
        locals.var_q_sl_dep_dn0 = assign32410_e36635_d_n0;
        locals.var_q_sl_dep_dn2 = assign32410_e36635_d_n2;
        locals.var_q_sl_dep_dn4 = assign32410_e36635_d_n4;
        locals.var_q_sl_dep_dn5 = assign32410_e36635_d_n5;
        locals.var_q_sl_dep_dn6 = assign32410_e36635_d_n6;
        locals.var_q_sl_dep_dn7 = assign32410_e36635_d_n7;
        locals.var_q_sl_dep_dn8 = assign32410_e36635_d_n8;
        locals.var_q_sl_dep_dn9 = assign32410_e36635_d_n9;
        locals.var_q_sl_dep_dn10 = assign32410_e36635_d_n10;
        locals.var_q_sl_dep_dn11 = assign32410_e36635_d_n11;
        locals.var_q_sl_dep_dn14 = assign32410_e36635_d_n14;
        locals.var_q_sl_dep_rv = 0.0;

        let (assign32420_e36646, assign32420_e36646_d_n0, assign32420_e36646_d_n2, assign32420_e36646_d_n4, assign32420_e36646_d_n5, assign32420_e36646_d_n6, assign32420_e36646_d_n7, assign32420_e36646_d_n8, assign32420_e36646_d_n9, assign32420_e36646_d_n10, assign32420_e36646_d_n11, assign32420_e36646_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_subl, locals.var_q_subl_dn0, locals.var_q_subl_dn2, locals.var_q_subl_dn4, locals.var_q_subl_dn5, locals.var_q_subl_dn6, locals.var_q_subl_dn7, locals.var_q_subl_dn8, locals.var_q_subl_dn9, locals.var_q_subl_dn10, locals.var_q_subl_dn11, locals.var_q_subl_dn14,)
    }
};
        locals.var_q_subl = assign32420_e36646;
        locals.var_q_subl_dn0 = assign32420_e36646_d_n0;
        locals.var_q_subl_dn2 = assign32420_e36646_d_n2;
        locals.var_q_subl_dn4 = assign32420_e36646_d_n4;
        locals.var_q_subl_dn5 = assign32420_e36646_d_n5;
        locals.var_q_subl_dn6 = assign32420_e36646_d_n6;
        locals.var_q_subl_dn7 = assign32420_e36646_d_n7;
        locals.var_q_subl_dn8 = assign32420_e36646_d_n8;
        locals.var_q_subl_dn9 = assign32420_e36646_d_n9;
        locals.var_q_subl_dn10 = assign32420_e36646_d_n10;
        locals.var_q_subl_dn11 = assign32420_e36646_d_n11;
        locals.var_q_subl_dn14 = assign32420_e36646_d_n14;
        locals.var_q_subl_rv = 0.0;

        let (assign32430_e36662, assign32430_e36662_d_n0, assign32430_e36662_d_n2, assign32430_e36662_d_n4, assign32430_e36662_d_n5, assign32430_e36662_d_n6, assign32430_e36662_d_n7, assign32430_e36662_d_n8, assign32430_e36662_d_n9, assign32430_e36662_d_n10, assign32430_e36662_d_n11, assign32430_e36662_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) {
        let assign32430_e36658: f64 = (locals.var_phi_bl_dep - locals.var_phi_jl_dep);
        let assign32430_e36659: f64 = (locals.var_c_2esipq_ndepm * assign32430_e36658);
        let assign32430_e36660: f64 = (assign32430_e36659).sqrt();
        (assign32430_e36660, (((locals.var_c_2esipq_ndepm_dn0 * assign32430_e36658) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn0 - locals.var_phi_jl_dep_dn0))) / (2.0 * assign32430_e36660)), (((locals.var_c_2esipq_ndepm_dn2 * assign32430_e36658) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn2 - locals.var_phi_jl_dep_dn2))) / (2.0 * assign32430_e36660)), (((locals.var_c_2esipq_ndepm_dn4 * assign32430_e36658) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn4 - locals.var_phi_jl_dep_dn4))) / (2.0 * assign32430_e36660)), (((locals.var_c_2esipq_ndepm_dn5 * assign32430_e36658) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn5 - locals.var_phi_jl_dep_dn5))) / (2.0 * assign32430_e36660)), (((locals.var_c_2esipq_ndepm_dn6 * assign32430_e36658) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn6 - locals.var_phi_jl_dep_dn6))) / (2.0 * assign32430_e36660)), (((locals.var_c_2esipq_ndepm_dn7 * assign32430_e36658) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn7 - locals.var_phi_jl_dep_dn7))) / (2.0 * assign32430_e36660)), (((locals.var_c_2esipq_ndepm_dn8 * assign32430_e36658) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn8 - locals.var_phi_jl_dep_dn8))) / (2.0 * assign32430_e36660)), (((locals.var_c_2esipq_ndepm_dn9 * assign32430_e36658) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn9 - locals.var_phi_jl_dep_dn9))) / (2.0 * assign32430_e36660)), (((locals.var_c_2esipq_ndepm_dn10 * assign32430_e36658) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn10 - locals.var_phi_jl_dep_dn10))) / (2.0 * assign32430_e36660)), (((locals.var_c_2esipq_ndepm_dn11 * assign32430_e36658) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn11 - locals.var_phi_jl_dep_dn11))) / (2.0 * assign32430_e36660)), (((locals.var_c_2esipq_ndepm_dn14 * assign32430_e36658) + (locals.var_c_2esipq_ndepm * (locals.var_phi_bl_dep_dn14 - locals.var_phi_jl_dep_dn14))) / (2.0 * assign32430_e36660)),)
    } else {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn11, locals.var_w_bl_dn14,)
    }
};
        locals.var_w_bl = assign32430_e36662;
        locals.var_w_bl_dn0 = assign32430_e36662_d_n0;
        locals.var_w_bl_dn2 = assign32430_e36662_d_n2;
        locals.var_w_bl_dn4 = assign32430_e36662_d_n4;
        locals.var_w_bl_dn5 = assign32430_e36662_d_n5;
        locals.var_w_bl_dn6 = assign32430_e36662_d_n6;
        locals.var_w_bl_dn7 = assign32430_e36662_d_n7;
        locals.var_w_bl_dn8 = assign32430_e36662_d_n8;
        locals.var_w_bl_dn9 = assign32430_e36662_d_n9;
        locals.var_w_bl_dn10 = assign32430_e36662_d_n10;
        locals.var_w_bl_dn11 = assign32430_e36662_d_n11;
        locals.var_w_bl_dn14 = assign32430_e36662_d_n14;
        locals.var_w_bl_rv = 0.0;

        let assign32440_e36666: f64 = (locals.var_uc_depthn - 1e-8);
        let assign32440_e36671: f64 = if ((locals.var_w_bl > assign32440_e36666) && (1e-8 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard775 = assign32440_e36671;
        locals.var_guard775_rv = 0.0;

        let (assign32450_e36688, assign32450_e36688_d_n0, assign32450_e36688_d_n2, assign32450_e36688_d_n4, assign32450_e36688_d_n5, assign32450_e36688_d_n6, assign32450_e36688_d_n7, assign32450_e36688_d_n8, assign32450_e36688_d_n9, assign32450_e36688_d_n10, assign32450_e36688_d_n11, assign32450_e36688_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) && (locals.var_guard775 != 0.0)) {
        let assign32450_e36684: f64 = (locals.var_w_bl - locals.var_uc_depthn);
        let assign32450_e36686: f64 = (assign32450_e36684 + 1e-8);
        (assign32450_e36686, (locals.var_w_bl_dn0 - locals.var_uc_depthn_dn0), (locals.var_w_bl_dn2 - locals.var_uc_depthn_dn2), (locals.var_w_bl_dn4 - locals.var_uc_depthn_dn4), (locals.var_w_bl_dn5 - locals.var_uc_depthn_dn5), (locals.var_w_bl_dn6 - locals.var_uc_depthn_dn6), (locals.var_w_bl_dn7 - locals.var_uc_depthn_dn7), (locals.var_w_bl_dn8 - locals.var_uc_depthn_dn8), (locals.var_w_bl_dn9 - locals.var_uc_depthn_dn9), (locals.var_w_bl_dn10 - locals.var_uc_depthn_dn10), (locals.var_w_bl_dn11 - locals.var_uc_depthn_dn11), (locals.var_w_bl_dn14 - locals.var_uc_depthn_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign32450_e36688;
        locals.var_tmf1_dn0 = assign32450_e36688_d_n0;
        locals.var_tmf1_dn2 = assign32450_e36688_d_n2;
        locals.var_tmf1_dn4 = assign32450_e36688_d_n4;
        locals.var_tmf1_dn5 = assign32450_e36688_d_n5;
        locals.var_tmf1_dn6 = assign32450_e36688_d_n6;
        locals.var_tmf1_dn7 = assign32450_e36688_d_n7;
        locals.var_tmf1_dn8 = assign32450_e36688_d_n8;
        locals.var_tmf1_dn9 = assign32450_e36688_d_n9;
        locals.var_tmf1_dn10 = assign32450_e36688_d_n10;
        locals.var_tmf1_dn11 = assign32450_e36688_d_n11;
        locals.var_tmf1_dn14 = assign32450_e36688_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign32460_e36703, assign32460_e36703_d_n0, assign32460_e36703_d_n2, assign32460_e36703_d_n4, assign32460_e36703_d_n5, assign32460_e36703_d_n6, assign32460_e36703_d_n7, assign32460_e36703_d_n8, assign32460_e36703_d_n9, assign32460_e36703_d_n10, assign32460_e36703_d_n11, assign32460_e36703_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) && (locals.var_guard775 != 0.0)) {
        let assign32460_e36701: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign32460_e36701, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign32460_e36703;
        locals.var_x2_dn0 = assign32460_e36703_d_n0;
        locals.var_x2_dn2 = assign32460_e36703_d_n2;
        locals.var_x2_dn4 = assign32460_e36703_d_n4;
        locals.var_x2_dn5 = assign32460_e36703_d_n5;
        locals.var_x2_dn6 = assign32460_e36703_d_n6;
        locals.var_x2_dn7 = assign32460_e36703_d_n7;
        locals.var_x2_dn8 = assign32460_e36703_d_n8;
        locals.var_x2_dn9 = assign32460_e36703_d_n9;
        locals.var_x2_dn10 = assign32460_e36703_d_n10;
        locals.var_x2_dn11 = assign32460_e36703_d_n11;
        locals.var_x2_dn14 = assign32460_e36703_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign32470_e36718, assign32470_e36718_d_n0, assign32470_e36718_d_n2, assign32470_e36718_d_n4, assign32470_e36718_d_n5, assign32470_e36718_d_n6, assign32470_e36718_d_n7, assign32470_e36718_d_n8, assign32470_e36718_d_n9, assign32470_e36718_d_n10, assign32470_e36718_d_n11, assign32470_e36718_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) && (locals.var_guard775 != 0.0)) {
        let assign32470_e36716: f64 = (1e-8 * 1e-8);
        (assign32470_e36716, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign32470_e36718;
        locals.var_xmax2_dn0 = assign32470_e36718_d_n0;
        locals.var_xmax2_dn2 = assign32470_e36718_d_n2;
        locals.var_xmax2_dn4 = assign32470_e36718_d_n4;
        locals.var_xmax2_dn5 = assign32470_e36718_d_n5;
        locals.var_xmax2_dn6 = assign32470_e36718_d_n6;
        locals.var_xmax2_dn7 = assign32470_e36718_d_n7;
        locals.var_xmax2_dn8 = assign32470_e36718_d_n8;
        locals.var_xmax2_dn9 = assign32470_e36718_d_n9;
        locals.var_xmax2_dn10 = assign32470_e36718_d_n10;
        locals.var_xmax2_dn11 = assign32470_e36718_d_n11;
        locals.var_xmax2_dn14 = assign32470_e36718_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign32480_e36731, assign32480_e36731_d_n0, assign32480_e36731_d_n2, assign32480_e36731_d_n4, assign32480_e36731_d_n5, assign32480_e36731_d_n6, assign32480_e36731_d_n7, assign32480_e36731_d_n8, assign32480_e36731_d_n9, assign32480_e36731_d_n10, assign32480_e36731_d_n11, assign32480_e36731_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) && (locals.var_guard775 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign32480_e36731;
        locals.var_xp_dn0 = assign32480_e36731_d_n0;
        locals.var_xp_dn2 = assign32480_e36731_d_n2;
        locals.var_xp_dn4 = assign32480_e36731_d_n4;
        locals.var_xp_dn5 = assign32480_e36731_d_n5;
        locals.var_xp_dn6 = assign32480_e36731_d_n6;
        locals.var_xp_dn7 = assign32480_e36731_d_n7;
        locals.var_xp_dn8 = assign32480_e36731_d_n8;
        locals.var_xp_dn9 = assign32480_e36731_d_n9;
        locals.var_xp_dn10 = assign32480_e36731_d_n10;
        locals.var_xp_dn11 = assign32480_e36731_d_n11;
        locals.var_xp_dn14 = assign32480_e36731_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign32490_e36744, assign32490_e36744_d_n0, assign32490_e36744_d_n2, assign32490_e36744_d_n4, assign32490_e36744_d_n5, assign32490_e36744_d_n6, assign32490_e36744_d_n7, assign32490_e36744_d_n8, assign32490_e36744_d_n9, assign32490_e36744_d_n10, assign32490_e36744_d_n11, assign32490_e36744_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) && (locals.var_guard775 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign32490_e36744;
        locals.var_xmp_dn0 = assign32490_e36744_d_n0;
        locals.var_xmp_dn2 = assign32490_e36744_d_n2;
        locals.var_xmp_dn4 = assign32490_e36744_d_n4;
        locals.var_xmp_dn5 = assign32490_e36744_d_n5;
        locals.var_xmp_dn6 = assign32490_e36744_d_n6;
        locals.var_xmp_dn7 = assign32490_e36744_d_n7;
        locals.var_xmp_dn8 = assign32490_e36744_d_n8;
        locals.var_xmp_dn9 = assign32490_e36744_d_n9;
        locals.var_xmp_dn10 = assign32490_e36744_d_n10;
        locals.var_xmp_dn11 = assign32490_e36744_d_n11;
        locals.var_xmp_dn14 = assign32490_e36744_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign32500_e36757,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) && (locals.var_guard775 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign32500_e36757;
        locals.var_m0_rv = 0.0;

        let (assign32510_e36770,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) && (locals.var_guard775 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign32510_e36770;
        locals.var_mm_rv = 0.0;

        let (assign32520_e36783, assign32520_e36783_d_n0, assign32520_e36783_d_n2, assign32520_e36783_d_n4, assign32520_e36783_d_n5, assign32520_e36783_d_n6, assign32520_e36783_d_n7, assign32520_e36783_d_n8, assign32520_e36783_d_n9, assign32520_e36783_d_n10, assign32520_e36783_d_n11, assign32520_e36783_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) && (locals.var_guard775 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign32520_e36783;
        locals.var_arg_dn0 = assign32520_e36783_d_n0;
        locals.var_arg_dn2 = assign32520_e36783_d_n2;
        locals.var_arg_dn4 = assign32520_e36783_d_n4;
        locals.var_arg_dn5 = assign32520_e36783_d_n5;
        locals.var_arg_dn6 = assign32520_e36783_d_n6;
        locals.var_arg_dn7 = assign32520_e36783_d_n7;
        locals.var_arg_dn8 = assign32520_e36783_d_n8;
        locals.var_arg_dn9 = assign32520_e36783_d_n9;
        locals.var_arg_dn10 = assign32520_e36783_d_n10;
        locals.var_arg_dn11 = assign32520_e36783_d_n11;
        locals.var_arg_dn14 = assign32520_e36783_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign32530_e36796, assign32530_e36796_d_n0, assign32530_e36796_d_n2, assign32530_e36796_d_n4, assign32530_e36796_d_n5, assign32530_e36796_d_n6, assign32530_e36796_d_n7, assign32530_e36796_d_n8, assign32530_e36796_d_n9, assign32530_e36796_d_n10, assign32530_e36796_d_n11, assign32530_e36796_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) && (locals.var_guard775 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign32530_e36796;
        locals.var_dnm_dn0 = assign32530_e36796_d_n0;
        locals.var_dnm_dn2 = assign32530_e36796_d_n2;
        locals.var_dnm_dn4 = assign32530_e36796_d_n4;
        locals.var_dnm_dn5 = assign32530_e36796_d_n5;
        locals.var_dnm_dn6 = assign32530_e36796_d_n6;
        locals.var_dnm_dn7 = assign32530_e36796_d_n7;
        locals.var_dnm_dn8 = assign32530_e36796_d_n8;
        locals.var_dnm_dn9 = assign32530_e36796_d_n9;
        locals.var_dnm_dn10 = assign32530_e36796_d_n10;
        locals.var_dnm_dn11 = assign32530_e36796_d_n11;
        locals.var_dnm_dn14 = assign32530_e36796_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign32540_e36811, assign32540_e36811_d_n0, assign32540_e36811_d_n2, assign32540_e36811_d_n4, assign32540_e36811_d_n5, assign32540_e36811_d_n6, assign32540_e36811_d_n7, assign32540_e36811_d_n8, assign32540_e36811_d_n9, assign32540_e36811_d_n10, assign32540_e36811_d_n11, assign32540_e36811_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) && (locals.var_guard775 != 0.0)) {
        let assign32540_e36809: f64 = (locals.var_xp * locals.var_x2);
        (assign32540_e36809, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign32540_e36811;
        locals.var_xp_dn0 = assign32540_e36811_d_n0;
        locals.var_xp_dn2 = assign32540_e36811_d_n2;
        locals.var_xp_dn4 = assign32540_e36811_d_n4;
        locals.var_xp_dn5 = assign32540_e36811_d_n5;
        locals.var_xp_dn6 = assign32540_e36811_d_n6;
        locals.var_xp_dn7 = assign32540_e36811_d_n7;
        locals.var_xp_dn8 = assign32540_e36811_d_n8;
        locals.var_xp_dn9 = assign32540_e36811_d_n9;
        locals.var_xp_dn10 = assign32540_e36811_d_n10;
        locals.var_xp_dn11 = assign32540_e36811_d_n11;
        locals.var_xp_dn14 = assign32540_e36811_d_n14;
        locals.var_xp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_105(
        locals: &mut StampLocals,
    ) {
        let (assign32550_e36826, assign32550_e36826_d_n0, assign32550_e36826_d_n2, assign32550_e36826_d_n4, assign32550_e36826_d_n5, assign32550_e36826_d_n6, assign32550_e36826_d_n7, assign32550_e36826_d_n8, assign32550_e36826_d_n9, assign32550_e36826_d_n10, assign32550_e36826_d_n11, assign32550_e36826_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) && (locals.var_guard775 != 0.0)) {
        let assign32550_e36824: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign32550_e36824, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign32550_e36826;
        locals.var_xmp_dn0 = assign32550_e36826_d_n0;
        locals.var_xmp_dn2 = assign32550_e36826_d_n2;
        locals.var_xmp_dn4 = assign32550_e36826_d_n4;
        locals.var_xmp_dn5 = assign32550_e36826_d_n5;
        locals.var_xmp_dn6 = assign32550_e36826_d_n6;
        locals.var_xmp_dn7 = assign32550_e36826_d_n7;
        locals.var_xmp_dn8 = assign32550_e36826_d_n8;
        locals.var_xmp_dn9 = assign32550_e36826_d_n9;
        locals.var_xmp_dn10 = assign32550_e36826_d_n10;
        locals.var_xmp_dn11 = assign32550_e36826_d_n11;
        locals.var_xmp_dn14 = assign32550_e36826_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign32560_e36841, assign32560_e36841_d_n0, assign32560_e36841_d_n2, assign32560_e36841_d_n4, assign32560_e36841_d_n5, assign32560_e36841_d_n6, assign32560_e36841_d_n7, assign32560_e36841_d_n8, assign32560_e36841_d_n9, assign32560_e36841_d_n10, assign32560_e36841_d_n11, assign32560_e36841_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) && (locals.var_guard775 != 0.0)) {
        let assign32560_e36839: f64 = (locals.var_xp * locals.var_x2);
        (assign32560_e36839, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign32560_e36841;
        locals.var_xp_dn0 = assign32560_e36841_d_n0;
        locals.var_xp_dn2 = assign32560_e36841_d_n2;
        locals.var_xp_dn4 = assign32560_e36841_d_n4;
        locals.var_xp_dn5 = assign32560_e36841_d_n5;
        locals.var_xp_dn6 = assign32560_e36841_d_n6;
        locals.var_xp_dn7 = assign32560_e36841_d_n7;
        locals.var_xp_dn8 = assign32560_e36841_d_n8;
        locals.var_xp_dn9 = assign32560_e36841_d_n9;
        locals.var_xp_dn10 = assign32560_e36841_d_n10;
        locals.var_xp_dn11 = assign32560_e36841_d_n11;
        locals.var_xp_dn14 = assign32560_e36841_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign32570_e36856, assign32570_e36856_d_n0, assign32570_e36856_d_n2, assign32570_e36856_d_n4, assign32570_e36856_d_n5, assign32570_e36856_d_n6, assign32570_e36856_d_n7, assign32570_e36856_d_n8, assign32570_e36856_d_n9, assign32570_e36856_d_n10, assign32570_e36856_d_n11, assign32570_e36856_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) && (locals.var_guard775 != 0.0)) {
        let assign32570_e36854: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign32570_e36854, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign32570_e36856;
        locals.var_xmp_dn0 = assign32570_e36856_d_n0;
        locals.var_xmp_dn2 = assign32570_e36856_d_n2;
        locals.var_xmp_dn4 = assign32570_e36856_d_n4;
        locals.var_xmp_dn5 = assign32570_e36856_d_n5;
        locals.var_xmp_dn6 = assign32570_e36856_d_n6;
        locals.var_xmp_dn7 = assign32570_e36856_d_n7;
        locals.var_xmp_dn8 = assign32570_e36856_d_n8;
        locals.var_xmp_dn9 = assign32570_e36856_d_n9;
        locals.var_xmp_dn10 = assign32570_e36856_d_n10;
        locals.var_xmp_dn11 = assign32570_e36856_d_n11;
        locals.var_xmp_dn14 = assign32570_e36856_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign32580_e36871, assign32580_e36871_d_n0, assign32580_e36871_d_n2, assign32580_e36871_d_n4, assign32580_e36871_d_n5, assign32580_e36871_d_n6, assign32580_e36871_d_n7, assign32580_e36871_d_n8, assign32580_e36871_d_n9, assign32580_e36871_d_n10, assign32580_e36871_d_n11, assign32580_e36871_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) && (locals.var_guard775 != 0.0)) {
        let assign32580_e36869: f64 = (locals.var_xp + locals.var_xmp);
        (assign32580_e36869, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign32580_e36871;
        locals.var_arg_dn0 = assign32580_e36871_d_n0;
        locals.var_arg_dn2 = assign32580_e36871_d_n2;
        locals.var_arg_dn4 = assign32580_e36871_d_n4;
        locals.var_arg_dn5 = assign32580_e36871_d_n5;
        locals.var_arg_dn6 = assign32580_e36871_d_n6;
        locals.var_arg_dn7 = assign32580_e36871_d_n7;
        locals.var_arg_dn8 = assign32580_e36871_d_n8;
        locals.var_arg_dn9 = assign32580_e36871_d_n9;
        locals.var_arg_dn10 = assign32580_e36871_d_n10;
        locals.var_arg_dn11 = assign32580_e36871_d_n11;
        locals.var_arg_dn14 = assign32580_e36871_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign32590_e36884, assign32590_e36884_d_n0, assign32590_e36884_d_n2, assign32590_e36884_d_n4, assign32590_e36884_d_n5, assign32590_e36884_d_n6, assign32590_e36884_d_n7, assign32590_e36884_d_n8, assign32590_e36884_d_n9, assign32590_e36884_d_n10, assign32590_e36884_d_n11, assign32590_e36884_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) && (locals.var_guard775 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign32590_e36884;
        locals.var_dnm_dn0 = assign32590_e36884_d_n0;
        locals.var_dnm_dn2 = assign32590_e36884_d_n2;
        locals.var_dnm_dn4 = assign32590_e36884_d_n4;
        locals.var_dnm_dn5 = assign32590_e36884_d_n5;
        locals.var_dnm_dn6 = assign32590_e36884_d_n6;
        locals.var_dnm_dn7 = assign32590_e36884_d_n7;
        locals.var_dnm_dn8 = assign32590_e36884_d_n8;
        locals.var_dnm_dn9 = assign32590_e36884_d_n9;
        locals.var_dnm_dn10 = assign32590_e36884_d_n10;
        locals.var_dnm_dn11 = assign32590_e36884_d_n11;
        locals.var_dnm_dn14 = assign32590_e36884_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign32600_e36899: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard776 = assign32600_e36899;
        locals.var_guard776_rv = 0.0;

        let assign32610_e36902: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard777 = assign32610_e36902;
        locals.var_guard777_rv = 0.0;

        let (assign32620_e36919,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) && (locals.var_guard775 != 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign32620_e36919;
        locals.var_mm_rv = 0.0;

        let assign32630_e36922: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard778 = assign32630_e36922;
        locals.var_guard778_rv = 0.0;

        let (assign32640_e36942,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) && (locals.var_guard775 != 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 == 0.0)) && (locals.var_guard778 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign32640_e36942;
        locals.var_mm_rv = 0.0;

        let assign32650_e36945: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard779 = assign32650_e36945;
        locals.var_guard779_rv = 0.0;

        let (assign32660_e36968,) = {
    if (((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) && (locals.var_guard775 != 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 == 0.0)) && (locals.var_guard778 == 0.0)) && (locals.var_guard779 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign32660_e36968;
        locals.var_mm_rv = 0.0;

        let assign32670_e36971: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard780 = assign32670_e36971;
        locals.var_guard780_rv = 0.0;

        let (assign32680_e36997,) = {
    if ((((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) && (locals.var_guard775 != 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 == 0.0)) && (locals.var_guard778 == 0.0)) && (locals.var_guard779 == 0.0)) && (locals.var_guard780 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign32680_e36997;
        locals.var_mm_rv = 0.0;

        let (assign32690_e37012,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) && (locals.var_guard775 != 0.0)) && (locals.var_guard776 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign32690_e37012;
        locals.var_m0_rv = 0.0;

        let mut assign32700_loop_guard: usize = 0;
        while {
            let assign32700_cond_e37028: f64 = if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) && (locals.var_guard775 != 0.0)) && (locals.var_guard776 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign32700_cond_e37028 != 0.0
        } {
            assign32700_loop_guard += 1;
            assert!(assign32700_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign32700_body0_e37044, assign32700_body0_e37044_d_n0, assign32700_body0_e37044_d_n2, assign32700_body0_e37044_d_n4, assign32700_body0_e37044_d_n5, assign32700_body0_e37044_d_n6, assign32700_body0_e37044_d_n7, assign32700_body0_e37044_d_n8, assign32700_body0_e37044_d_n9, assign32700_body0_e37044_d_n10, assign32700_body0_e37044_d_n11, assign32700_body0_e37044_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) && (locals.var_guard775 != 0.0)) && (locals.var_guard776 != 0.0)) {
        let assign32700_body0_e37042: f64 = (locals.var_dnm).sqrt();
        (assign32700_body0_e37042, (locals.var_dnm_dn0 / (2.0 * assign32700_body0_e37042)), (locals.var_dnm_dn2 / (2.0 * assign32700_body0_e37042)), (locals.var_dnm_dn4 / (2.0 * assign32700_body0_e37042)), (locals.var_dnm_dn5 / (2.0 * assign32700_body0_e37042)), (locals.var_dnm_dn6 / (2.0 * assign32700_body0_e37042)), (locals.var_dnm_dn7 / (2.0 * assign32700_body0_e37042)), (locals.var_dnm_dn8 / (2.0 * assign32700_body0_e37042)), (locals.var_dnm_dn9 / (2.0 * assign32700_body0_e37042)), (locals.var_dnm_dn10 / (2.0 * assign32700_body0_e37042)), (locals.var_dnm_dn11 / (2.0 * assign32700_body0_e37042)), (locals.var_dnm_dn14 / (2.0 * assign32700_body0_e37042)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign32700_body0_e37044;
            locals.var_dnm_dn0 = assign32700_body0_e37044_d_n0;
            locals.var_dnm_dn2 = assign32700_body0_e37044_d_n2;
            locals.var_dnm_dn4 = assign32700_body0_e37044_d_n4;
            locals.var_dnm_dn5 = assign32700_body0_e37044_d_n5;
            locals.var_dnm_dn6 = assign32700_body0_e37044_d_n6;
            locals.var_dnm_dn7 = assign32700_body0_e37044_d_n7;
            locals.var_dnm_dn8 = assign32700_body0_e37044_d_n8;
            locals.var_dnm_dn9 = assign32700_body0_e37044_d_n9;
            locals.var_dnm_dn10 = assign32700_body0_e37044_d_n10;
            locals.var_dnm_dn11 = assign32700_body0_e37044_d_n11;
            locals.var_dnm_dn14 = assign32700_body0_e37044_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign32700_body1_e37061,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) && (locals.var_guard775 != 0.0)) && (locals.var_guard776 != 0.0)) {
        let assign32700_body1_e37059: f64 = (locals.var_m0 + 1.0);
        (assign32700_body1_e37059,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign32700_body1_e37061;
            locals.var_m0_rv = 0.0;
        }

        let (assign32710_e37088, assign32710_e37088_d_n0, assign32710_e37088_d_n2, assign32710_e37088_d_n4, assign32710_e37088_d_n5, assign32710_e37088_d_n6, assign32710_e37088_d_n7, assign32710_e37088_d_n8, assign32710_e37088_d_n9, assign32710_e37088_d_n10, assign32710_e37088_d_n11, assign32710_e37088_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) && (locals.var_guard775 != 0.0)) && (locals.var_guard776 == 0.0)) {
        let (assign32710_e37086, assign32710_e37086_d_n0, assign32710_e37086_d_n2, assign32710_e37086_d_n4, assign32710_e37086_d_n5, assign32710_e37086_d_n6, assign32710_e37086_d_n7, assign32710_e37086_d_n8, assign32710_e37086_d_n9, assign32710_e37086_d_n10, assign32710_e37086_d_n11, assign32710_e37086_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign32710_e37083: f64 = (2.0 * 2.0);
                let assign32710_e37084: f64 = (1.0 / assign32710_e37083);
                let assign32710_e37085: f64 = (locals.var_dnm).powf(assign32710_e37084);
                (assign32710_e37085, if 0.0 == 0.0 && ((assign32710_e37084) as f64).is_finite() && ((assign32710_e37084) as f64).fract() == 0.0 { if assign32710_e37084 == 0.0 { 0.0 } else { (assign32710_e37084 * ((locals.var_dnm).powf(assign32710_e37084 - 1.0) * locals.var_dnm_dn0)) } } else { (assign32710_e37085 * (assign32710_e37084 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign32710_e37084) as f64).is_finite() && ((assign32710_e37084) as f64).fract() == 0.0 { if assign32710_e37084 == 0.0 { 0.0 } else { (assign32710_e37084 * ((locals.var_dnm).powf(assign32710_e37084 - 1.0) * locals.var_dnm_dn2)) } } else { (assign32710_e37085 * (assign32710_e37084 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign32710_e37084) as f64).is_finite() && ((assign32710_e37084) as f64).fract() == 0.0 { if assign32710_e37084 == 0.0 { 0.0 } else { (assign32710_e37084 * ((locals.var_dnm).powf(assign32710_e37084 - 1.0) * locals.var_dnm_dn4)) } } else { (assign32710_e37085 * (assign32710_e37084 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign32710_e37084) as f64).is_finite() && ((assign32710_e37084) as f64).fract() == 0.0 { if assign32710_e37084 == 0.0 { 0.0 } else { (assign32710_e37084 * ((locals.var_dnm).powf(assign32710_e37084 - 1.0) * locals.var_dnm_dn5)) } } else { (assign32710_e37085 * (assign32710_e37084 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign32710_e37084) as f64).is_finite() && ((assign32710_e37084) as f64).fract() == 0.0 { if assign32710_e37084 == 0.0 { 0.0 } else { (assign32710_e37084 * ((locals.var_dnm).powf(assign32710_e37084 - 1.0) * locals.var_dnm_dn6)) } } else { (assign32710_e37085 * (assign32710_e37084 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign32710_e37084) as f64).is_finite() && ((assign32710_e37084) as f64).fract() == 0.0 { if assign32710_e37084 == 0.0 { 0.0 } else { (assign32710_e37084 * ((locals.var_dnm).powf(assign32710_e37084 - 1.0) * locals.var_dnm_dn7)) } } else { (assign32710_e37085 * (assign32710_e37084 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign32710_e37084) as f64).is_finite() && ((assign32710_e37084) as f64).fract() == 0.0 { if assign32710_e37084 == 0.0 { 0.0 } else { (assign32710_e37084 * ((locals.var_dnm).powf(assign32710_e37084 - 1.0) * locals.var_dnm_dn8)) } } else { (assign32710_e37085 * (assign32710_e37084 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign32710_e37084) as f64).is_finite() && ((assign32710_e37084) as f64).fract() == 0.0 { if assign32710_e37084 == 0.0 { 0.0 } else { (assign32710_e37084 * ((locals.var_dnm).powf(assign32710_e37084 - 1.0) * locals.var_dnm_dn9)) } } else { (assign32710_e37085 * (assign32710_e37084 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign32710_e37084) as f64).is_finite() && ((assign32710_e37084) as f64).fract() == 0.0 { if assign32710_e37084 == 0.0 { 0.0 } else { (assign32710_e37084 * ((locals.var_dnm).powf(assign32710_e37084 - 1.0) * locals.var_dnm_dn10)) } } else { (assign32710_e37085 * (assign32710_e37084 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign32710_e37084) as f64).is_finite() && ((assign32710_e37084) as f64).fract() == 0.0 { if assign32710_e37084 == 0.0 { 0.0 } else { (assign32710_e37084 * ((locals.var_dnm).powf(assign32710_e37084 - 1.0) * locals.var_dnm_dn11)) } } else { (assign32710_e37085 * (assign32710_e37084 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign32710_e37084) as f64).is_finite() && ((assign32710_e37084) as f64).fract() == 0.0 { if assign32710_e37084 == 0.0 { 0.0 } else { (assign32710_e37084 * ((locals.var_dnm).powf(assign32710_e37084 - 1.0) * locals.var_dnm_dn14)) } } else { (assign32710_e37085 * (assign32710_e37084 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign32710_e37086, assign32710_e37086_d_n0, assign32710_e37086_d_n2, assign32710_e37086_d_n4, assign32710_e37086_d_n5, assign32710_e37086_d_n6, assign32710_e37086_d_n7, assign32710_e37086_d_n8, assign32710_e37086_d_n9, assign32710_e37086_d_n10, assign32710_e37086_d_n11, assign32710_e37086_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign32710_e37088;
        locals.var_dnm_dn0 = assign32710_e37088_d_n0;
        locals.var_dnm_dn2 = assign32710_e37088_d_n2;
        locals.var_dnm_dn4 = assign32710_e37088_d_n4;
        locals.var_dnm_dn5 = assign32710_e37088_d_n5;
        locals.var_dnm_dn6 = assign32710_e37088_d_n6;
        locals.var_dnm_dn7 = assign32710_e37088_d_n7;
        locals.var_dnm_dn8 = assign32710_e37088_d_n8;
        locals.var_dnm_dn9 = assign32710_e37088_d_n9;
        locals.var_dnm_dn10 = assign32710_e37088_d_n10;
        locals.var_dnm_dn11 = assign32710_e37088_d_n11;
        locals.var_dnm_dn14 = assign32710_e37088_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign32720_e37103, assign32720_e37103_d_n0, assign32720_e37103_d_n2, assign32720_e37103_d_n4, assign32720_e37103_d_n5, assign32720_e37103_d_n6, assign32720_e37103_d_n7, assign32720_e37103_d_n8, assign32720_e37103_d_n9, assign32720_e37103_d_n10, assign32720_e37103_d_n11, assign32720_e37103_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) && (locals.var_guard775 != 0.0)) {
        let assign32720_e37101: f64 = (1.0 / locals.var_dnm);
        (assign32720_e37101, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign32720_e37103;
        locals.var_dnm_dn0 = assign32720_e37103_d_n0;
        locals.var_dnm_dn2 = assign32720_e37103_d_n2;
        locals.var_dnm_dn4 = assign32720_e37103_d_n4;
        locals.var_dnm_dn5 = assign32720_e37103_d_n5;
        locals.var_dnm_dn6 = assign32720_e37103_d_n6;
        locals.var_dnm_dn7 = assign32720_e37103_d_n7;
        locals.var_dnm_dn8 = assign32720_e37103_d_n8;
        locals.var_dnm_dn9 = assign32720_e37103_d_n9;
        locals.var_dnm_dn10 = assign32720_e37103_d_n10;
        locals.var_dnm_dn11 = assign32720_e37103_d_n11;
        locals.var_dnm_dn14 = assign32720_e37103_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign32730_e37120, assign32730_e37120_d_n0, assign32730_e37120_d_n2, assign32730_e37120_d_n4, assign32730_e37120_d_n5, assign32730_e37120_d_n6, assign32730_e37120_d_n7, assign32730_e37120_d_n8, assign32730_e37120_d_n9, assign32730_e37120_d_n10, assign32730_e37120_d_n11, assign32730_e37120_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) && (locals.var_guard775 != 0.0)) {
        let assign32730_e37116: f64 = (locals.var_tmf1 * 1e-8);
        let assign32730_e37118: f64 = (assign32730_e37116 * locals.var_dnm);
        (assign32730_e37118, (((locals.var_tmf1_dn0 * 1e-8) * locals.var_dnm) + (assign32730_e37116 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-8) * locals.var_dnm) + (assign32730_e37116 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-8) * locals.var_dnm) + (assign32730_e37116 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-8) * locals.var_dnm) + (assign32730_e37116 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-8) * locals.var_dnm) + (assign32730_e37116 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-8) * locals.var_dnm) + (assign32730_e37116 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-8) * locals.var_dnm) + (assign32730_e37116 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-8) * locals.var_dnm) + (assign32730_e37116 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-8) * locals.var_dnm) + (assign32730_e37116 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1e-8) * locals.var_dnm) + (assign32730_e37116 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 1e-8) * locals.var_dnm) + (assign32730_e37116 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign32730_e37120;
        locals.var_tmf0_dn0 = assign32730_e37120_d_n0;
        locals.var_tmf0_dn2 = assign32730_e37120_d_n2;
        locals.var_tmf0_dn4 = assign32730_e37120_d_n4;
        locals.var_tmf0_dn5 = assign32730_e37120_d_n5;
        locals.var_tmf0_dn6 = assign32730_e37120_d_n6;
        locals.var_tmf0_dn7 = assign32730_e37120_d_n7;
        locals.var_tmf0_dn8 = assign32730_e37120_d_n8;
        locals.var_tmf0_dn9 = assign32730_e37120_d_n9;
        locals.var_tmf0_dn10 = assign32730_e37120_d_n10;
        locals.var_tmf0_dn11 = assign32730_e37120_d_n11;
        locals.var_tmf0_dn14 = assign32730_e37120_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign32740_e37139, assign32740_e37139_d_n0, assign32740_e37139_d_n2, assign32740_e37139_d_n4, assign32740_e37139_d_n5, assign32740_e37139_d_n6, assign32740_e37139_d_n7, assign32740_e37139_d_n8, assign32740_e37139_d_n9, assign32740_e37139_d_n10, assign32740_e37139_d_n11, assign32740_e37139_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) && (locals.var_guard775 != 0.0)) {
        let assign32740_e37133: f64 = (1e-8 * locals.var_xmp);
        let assign32740_e37135: f64 = (assign32740_e37133 * locals.var_dnm);
        let assign32740_e37137: f64 = (assign32740_e37135 / locals.var_arg);
        (assign32740_e37137, ((((((1e-8 * locals.var_xmp_dn0) * locals.var_dnm) + (assign32740_e37133 * locals.var_dnm_dn0)) * locals.var_arg) - (assign32740_e37135 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn2) * locals.var_dnm) + (assign32740_e37133 * locals.var_dnm_dn2)) * locals.var_arg) - (assign32740_e37135 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn4) * locals.var_dnm) + (assign32740_e37133 * locals.var_dnm_dn4)) * locals.var_arg) - (assign32740_e37135 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn5) * locals.var_dnm) + (assign32740_e37133 * locals.var_dnm_dn5)) * locals.var_arg) - (assign32740_e37135 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn6) * locals.var_dnm) + (assign32740_e37133 * locals.var_dnm_dn6)) * locals.var_arg) - (assign32740_e37135 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn7) * locals.var_dnm) + (assign32740_e37133 * locals.var_dnm_dn7)) * locals.var_arg) - (assign32740_e37135 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn8) * locals.var_dnm) + (assign32740_e37133 * locals.var_dnm_dn8)) * locals.var_arg) - (assign32740_e37135 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn9) * locals.var_dnm) + (assign32740_e37133 * locals.var_dnm_dn9)) * locals.var_arg) - (assign32740_e37135 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn10) * locals.var_dnm) + (assign32740_e37133 * locals.var_dnm_dn10)) * locals.var_arg) - (assign32740_e37135 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn11) * locals.var_dnm) + (assign32740_e37133 * locals.var_dnm_dn11)) * locals.var_arg) - (assign32740_e37135 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn14) * locals.var_dnm) + (assign32740_e37133 * locals.var_dnm_dn14)) * locals.var_arg) - (assign32740_e37135 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32740_e37139;
        locals.var_t3_dn0 = assign32740_e37139_d_n0;
        locals.var_t3_dn2 = assign32740_e37139_d_n2;
        locals.var_t3_dn4 = assign32740_e37139_d_n4;
        locals.var_t3_dn5 = assign32740_e37139_d_n5;
        locals.var_t3_dn6 = assign32740_e37139_d_n6;
        locals.var_t3_dn7 = assign32740_e37139_d_n7;
        locals.var_t3_dn8 = assign32740_e37139_d_n8;
        locals.var_t3_dn9 = assign32740_e37139_d_n9;
        locals.var_t3_dn10 = assign32740_e37139_d_n10;
        locals.var_t3_dn11 = assign32740_e37139_d_n11;
        locals.var_t3_dn14 = assign32740_e37139_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign32750_e37156, assign32750_e37156_d_n0, assign32750_e37156_d_n2, assign32750_e37156_d_n4, assign32750_e37156_d_n5, assign32750_e37156_d_n6, assign32750_e37156_d_n7, assign32750_e37156_d_n8, assign32750_e37156_d_n9, assign32750_e37156_d_n10, assign32750_e37156_d_n11, assign32750_e37156_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) && (locals.var_guard775 != 0.0)) {
        let assign32750_e37152: f64 = (locals.var_uc_depthn - 1e-8);
        let assign32750_e37154: f64 = (assign32750_e37152 + locals.var_tmf0);
        (assign32750_e37154, (locals.var_uc_depthn_dn0 + locals.var_tmf0_dn0), (locals.var_uc_depthn_dn2 + locals.var_tmf0_dn2), (locals.var_uc_depthn_dn4 + locals.var_tmf0_dn4), (locals.var_uc_depthn_dn5 + locals.var_tmf0_dn5), (locals.var_uc_depthn_dn6 + locals.var_tmf0_dn6), (locals.var_uc_depthn_dn7 + locals.var_tmf0_dn7), (locals.var_uc_depthn_dn8 + locals.var_tmf0_dn8), (locals.var_uc_depthn_dn9 + locals.var_tmf0_dn9), (locals.var_uc_depthn_dn10 + locals.var_tmf0_dn10), (locals.var_uc_depthn_dn11 + locals.var_tmf0_dn11), (locals.var_uc_depthn_dn14 + locals.var_tmf0_dn14),)
    } else {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn11, locals.var_w_bl_dn14,)
    }
};
        locals.var_w_bl = assign32750_e37156;
        locals.var_w_bl_dn0 = assign32750_e37156_d_n0;
        locals.var_w_bl_dn2 = assign32750_e37156_d_n2;
        locals.var_w_bl_dn4 = assign32750_e37156_d_n4;
        locals.var_w_bl_dn5 = assign32750_e37156_d_n5;
        locals.var_w_bl_dn6 = assign32750_e37156_d_n6;
        locals.var_w_bl_dn7 = assign32750_e37156_d_n7;
        locals.var_w_bl_dn8 = assign32750_e37156_d_n8;
        locals.var_w_bl_dn9 = assign32750_e37156_d_n9;
        locals.var_w_bl_dn10 = assign32750_e37156_d_n10;
        locals.var_w_bl_dn11 = assign32750_e37156_d_n11;
        locals.var_w_bl_dn14 = assign32750_e37156_d_n14;
        locals.var_w_bl_rv = 0.0;

        let (assign32760_e37169, assign32760_e37169_d_n0, assign32760_e37169_d_n2, assign32760_e37169_d_n4, assign32760_e37169_d_n5, assign32760_e37169_d_n6, assign32760_e37169_d_n7, assign32760_e37169_d_n8, assign32760_e37169_d_n9, assign32760_e37169_d_n10, assign32760_e37169_d_n11, assign32760_e37169_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) && (locals.var_guard775 != 0.0)) {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32760_e37169;
        locals.var_t3_dn0 = assign32760_e37169_d_n0;
        locals.var_t3_dn2 = assign32760_e37169_d_n2;
        locals.var_t3_dn4 = assign32760_e37169_d_n4;
        locals.var_t3_dn5 = assign32760_e37169_d_n5;
        locals.var_t3_dn6 = assign32760_e37169_d_n6;
        locals.var_t3_dn7 = assign32760_e37169_d_n7;
        locals.var_t3_dn8 = assign32760_e37169_d_n8;
        locals.var_t3_dn9 = assign32760_e37169_d_n9;
        locals.var_t3_dn10 = assign32760_e37169_d_n10;
        locals.var_t3_dn11 = assign32760_e37169_d_n11;
        locals.var_t3_dn14 = assign32760_e37169_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign32770_e37183, assign32770_e37183_d_n0, assign32770_e37183_d_n2, assign32770_e37183_d_n4, assign32770_e37183_d_n5, assign32770_e37183_d_n6, assign32770_e37183_d_n7, assign32770_e37183_d_n8, assign32770_e37183_d_n9, assign32770_e37183_d_n10, assign32770_e37183_d_n11, assign32770_e37183_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) && (locals.var_guard775 == 0.0)) {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn11, locals.var_w_bl_dn14,)
    } else {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn11, locals.var_w_bl_dn14,)
    }
};
        locals.var_w_bl = assign32770_e37183;
        locals.var_w_bl_dn0 = assign32770_e37183_d_n0;
        locals.var_w_bl_dn2 = assign32770_e37183_d_n2;
        locals.var_w_bl_dn4 = assign32770_e37183_d_n4;
        locals.var_w_bl_dn5 = assign32770_e37183_d_n5;
        locals.var_w_bl_dn6 = assign32770_e37183_d_n6;
        locals.var_w_bl_dn7 = assign32770_e37183_d_n7;
        locals.var_w_bl_dn8 = assign32770_e37183_d_n8;
        locals.var_w_bl_dn9 = assign32770_e37183_d_n9;
        locals.var_w_bl_dn10 = assign32770_e37183_d_n10;
        locals.var_w_bl_dn11 = assign32770_e37183_d_n11;
        locals.var_w_bl_dn14 = assign32770_e37183_d_n14;
        locals.var_w_bl_rv = 0.0;

        let (assign32780_e37197, assign32780_e37197_d_n0, assign32780_e37197_d_n2, assign32780_e37197_d_n4, assign32780_e37197_d_n5, assign32780_e37197_d_n6, assign32780_e37197_d_n7, assign32780_e37197_d_n8, assign32780_e37197_d_n9, assign32780_e37197_d_n10, assign32780_e37197_d_n11, assign32780_e37197_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) && (locals.var_guard775 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32780_e37197;
        locals.var_t3_dn0 = assign32780_e37197_d_n0;
        locals.var_t3_dn2 = assign32780_e37197_d_n2;
        locals.var_t3_dn4 = assign32780_e37197_d_n4;
        locals.var_t3_dn5 = assign32780_e37197_d_n5;
        locals.var_t3_dn6 = assign32780_e37197_d_n6;
        locals.var_t3_dn7 = assign32780_e37197_d_n7;
        locals.var_t3_dn8 = assign32780_e37197_d_n8;
        locals.var_t3_dn9 = assign32780_e37197_d_n9;
        locals.var_t3_dn10 = assign32780_e37197_d_n10;
        locals.var_t3_dn11 = assign32780_e37197_d_n11;
        locals.var_t3_dn14 = assign32780_e37197_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign32790_e37215, assign32790_e37215_d_n0, assign32790_e37215_d_n2, assign32790_e37215_d_n4, assign32790_e37215_d_n5, assign32790_e37215_d_n6, assign32790_e37215_d_n7, assign32790_e37215_d_n8, assign32790_e37215_d_n9, assign32790_e37215_d_n10, assign32790_e37215_d_n11, assign32790_e37215_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) {
        let assign32790_e37209: f64 = (locals.var_phi_jl_dep - locals.var_vbscl__blk437);
        let assign32790_e37211: f64 = (assign32790_e37209 + locals.var_vbi_dep);
        let assign32790_e37212: f64 = (locals.var_c_2esipq_nsub * assign32790_e37211);
        let assign32790_e37213: f64 = (assign32790_e37212).sqrt();
        (assign32790_e37213, (((locals.var_c_2esipq_nsub_dn0 * assign32790_e37211) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn0 - locals.var_vbscl__blk437_dn0) + locals.var_vbi_dep_dn0))) / (2.0 * assign32790_e37213)), (((locals.var_c_2esipq_nsub_dn2 * assign32790_e37211) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn2 - locals.var_vbscl__blk437_dn2) + locals.var_vbi_dep_dn2))) / (2.0 * assign32790_e37213)), (((locals.var_c_2esipq_nsub_dn4 * assign32790_e37211) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn4 - locals.var_vbscl__blk437_dn4) + locals.var_vbi_dep_dn4))) / (2.0 * assign32790_e37213)), (((locals.var_c_2esipq_nsub_dn5 * assign32790_e37211) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn5 - locals.var_vbscl__blk437_dn5) + locals.var_vbi_dep_dn5))) / (2.0 * assign32790_e37213)), (((locals.var_c_2esipq_nsub_dn6 * assign32790_e37211) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn6 - locals.var_vbscl__blk437_dn6) + locals.var_vbi_dep_dn6))) / (2.0 * assign32790_e37213)), (((locals.var_c_2esipq_nsub_dn7 * assign32790_e37211) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn7 - locals.var_vbscl__blk437_dn7) + locals.var_vbi_dep_dn7))) / (2.0 * assign32790_e37213)), (((locals.var_c_2esipq_nsub_dn8 * assign32790_e37211) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn8 - locals.var_vbscl__blk437_dn8) + locals.var_vbi_dep_dn8))) / (2.0 * assign32790_e37213)), (((locals.var_c_2esipq_nsub_dn9 * assign32790_e37211) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn9 - locals.var_vbscl__blk437_dn9) + locals.var_vbi_dep_dn9))) / (2.0 * assign32790_e37213)), (((locals.var_c_2esipq_nsub_dn10 * assign32790_e37211) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn10 - locals.var_vbscl__blk437_dn10) + locals.var_vbi_dep_dn10))) / (2.0 * assign32790_e37213)), (((locals.var_c_2esipq_nsub_dn11 * assign32790_e37211) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn11 - locals.var_vbscl__blk437_dn11) + locals.var_vbi_dep_dn11))) / (2.0 * assign32790_e37213)), (((locals.var_c_2esipq_nsub_dn14 * assign32790_e37211) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn14 - locals.var_vbscl__blk437_dn14) + locals.var_vbi_dep_dn14))) / (2.0 * assign32790_e37213)),)
    } else {
        (locals.var_w_subl, locals.var_w_subl_dn0, locals.var_w_subl_dn2, locals.var_w_subl_dn4, locals.var_w_subl_dn5, locals.var_w_subl_dn6, locals.var_w_subl_dn7, locals.var_w_subl_dn8, locals.var_w_subl_dn9, locals.var_w_subl_dn10, locals.var_w_subl_dn11, locals.var_w_subl_dn14,)
    }
};
        locals.var_w_subl = assign32790_e37215;
        locals.var_w_subl_dn0 = assign32790_e37215_d_n0;
        locals.var_w_subl_dn2 = assign32790_e37215_d_n2;
        locals.var_w_subl_dn4 = assign32790_e37215_d_n4;
        locals.var_w_subl_dn5 = assign32790_e37215_d_n5;
        locals.var_w_subl_dn6 = assign32790_e37215_d_n6;
        locals.var_w_subl_dn7 = assign32790_e37215_d_n7;
        locals.var_w_subl_dn8 = assign32790_e37215_d_n8;
        locals.var_w_subl_dn9 = assign32790_e37215_d_n9;
        locals.var_w_subl_dn10 = assign32790_e37215_d_n10;
        locals.var_w_subl_dn11 = assign32790_e37215_d_n11;
        locals.var_w_subl_dn14 = assign32790_e37215_d_n14;
        locals.var_w_subl_rv = 0.0;

        let (assign32800_e37228, assign32800_e37228_d_n0, assign32800_e37228_d_n2, assign32800_e37228_d_n4, assign32800_e37228_d_n5, assign32800_e37228_d_n6, assign32800_e37228_d_n7, assign32800_e37228_d_n8, assign32800_e37228_d_n9, assign32800_e37228_d_n10, assign32800_e37228_d_n11, assign32800_e37228_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) {
        let assign32800_e37226: f64 = (locals.var_w_bl * locals.var_q_ndepm);
        (assign32800_e37226, ((locals.var_w_bl_dn0 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn0)), ((locals.var_w_bl_dn2 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn2)), ((locals.var_w_bl_dn4 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn4)), ((locals.var_w_bl_dn5 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn5)), ((locals.var_w_bl_dn6 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn6)), ((locals.var_w_bl_dn7 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn7)), ((locals.var_w_bl_dn8 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn8)), ((locals.var_w_bl_dn9 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn9)), ((locals.var_w_bl_dn10 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn10)), ((locals.var_w_bl_dn11 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn11)), ((locals.var_w_bl_dn14 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn14)),)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn4, locals.var_q_bl_dep_dn5, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn8, locals.var_q_bl_dep_dn9, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn14,)
    }
};
        locals.var_q_bl_dep = assign32800_e37228;
        locals.var_q_bl_dep_dn0 = assign32800_e37228_d_n0;
        locals.var_q_bl_dep_dn2 = assign32800_e37228_d_n2;
        locals.var_q_bl_dep_dn4 = assign32800_e37228_d_n4;
        locals.var_q_bl_dep_dn5 = assign32800_e37228_d_n5;
        locals.var_q_bl_dep_dn6 = assign32800_e37228_d_n6;
        locals.var_q_bl_dep_dn7 = assign32800_e37228_d_n7;
        locals.var_q_bl_dep_dn8 = assign32800_e37228_d_n8;
        locals.var_q_bl_dep_dn9 = assign32800_e37228_d_n9;
        locals.var_q_bl_dep_dn10 = assign32800_e37228_d_n10;
        locals.var_q_bl_dep_dn11 = assign32800_e37228_d_n11;
        locals.var_q_bl_dep_dn14 = assign32800_e37228_d_n14;
        locals.var_q_bl_dep_rv = 0.0;

        let (assign32810_e37242, assign32810_e37242_d_n0, assign32810_e37242_d_n2, assign32810_e37242_d_n4, assign32810_e37242_d_n5, assign32810_e37242_d_n6, assign32810_e37242_d_n7, assign32810_e37242_d_n8, assign32810_e37242_d_n9, assign32810_e37242_d_n10, assign32810_e37242_d_n11, assign32810_e37242_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 != 0.0)) {
        let assign32810_e37238: f64 = (-locals.var_w_subl);
        let assign32810_e37240: f64 = (assign32810_e37238 * locals.var_q_nsub__blk546);
        (assign32810_e37240, (((-locals.var_w_subl_dn0) * locals.var_q_nsub__blk546) + (assign32810_e37238 * locals.var_q_nsub__blk546_dn0)), (((-locals.var_w_subl_dn2) * locals.var_q_nsub__blk546) + (assign32810_e37238 * locals.var_q_nsub__blk546_dn2)), (((-locals.var_w_subl_dn4) * locals.var_q_nsub__blk546) + (assign32810_e37238 * locals.var_q_nsub__blk546_dn4)), (((-locals.var_w_subl_dn5) * locals.var_q_nsub__blk546) + (assign32810_e37238 * locals.var_q_nsub__blk546_dn5)), (((-locals.var_w_subl_dn6) * locals.var_q_nsub__blk546) + (assign32810_e37238 * locals.var_q_nsub__blk546_dn6)), (((-locals.var_w_subl_dn7) * locals.var_q_nsub__blk546) + (assign32810_e37238 * locals.var_q_nsub__blk546_dn7)), (((-locals.var_w_subl_dn8) * locals.var_q_nsub__blk546) + (assign32810_e37238 * locals.var_q_nsub__blk546_dn8)), (((-locals.var_w_subl_dn9) * locals.var_q_nsub__blk546) + (assign32810_e37238 * locals.var_q_nsub__blk546_dn9)), (((-locals.var_w_subl_dn10) * locals.var_q_nsub__blk546) + (assign32810_e37238 * locals.var_q_nsub__blk546_dn10)), (((-locals.var_w_subl_dn11) * locals.var_q_nsub__blk546) + (assign32810_e37238 * locals.var_q_nsub__blk546_dn11)), (((-locals.var_w_subl_dn14) * locals.var_q_nsub__blk546) + (assign32810_e37238 * locals.var_q_nsub__blk546_dn14)),)
    } else {
        (locals.var_q_subl_dep, locals.var_q_subl_dep_dn0, locals.var_q_subl_dep_dn2, locals.var_q_subl_dep_dn4, locals.var_q_subl_dep_dn5, locals.var_q_subl_dep_dn6, locals.var_q_subl_dep_dn7, locals.var_q_subl_dep_dn8, locals.var_q_subl_dep_dn9, locals.var_q_subl_dep_dn10, locals.var_q_subl_dep_dn11, locals.var_q_subl_dep_dn14,)
    }
};
        locals.var_q_subl_dep = assign32810_e37242;
        locals.var_q_subl_dep_dn0 = assign32810_e37242_d_n0;
        locals.var_q_subl_dep_dn2 = assign32810_e37242_d_n2;
        locals.var_q_subl_dep_dn4 = assign32810_e37242_d_n4;
        locals.var_q_subl_dep_dn5 = assign32810_e37242_d_n5;
        locals.var_q_subl_dep_dn6 = assign32810_e37242_d_n6;
        locals.var_q_subl_dep_dn7 = assign32810_e37242_d_n7;
        locals.var_q_subl_dep_dn8 = assign32810_e37242_d_n8;
        locals.var_q_subl_dep_dn9 = assign32810_e37242_d_n9;
        locals.var_q_subl_dep_dn10 = assign32810_e37242_d_n10;
        locals.var_q_subl_dep_dn11 = assign32810_e37242_d_n11;
        locals.var_q_subl_dep_dn14 = assign32810_e37242_d_n14;
        locals.var_q_subl_dep_rv = 0.0;

        let (assign32820_e37260, assign32820_e37260_d_n0, assign32820_e37260_d_n2, assign32820_e37260_d_n4, assign32820_e37260_d_n5, assign32820_e37260_d_n6, assign32820_e37260_d_n7, assign32820_e37260_d_n8, assign32820_e37260_d_n9, assign32820_e37260_d_n10, assign32820_e37260_d_n11, assign32820_e37260_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) {
        let assign32820_e37253: f64 = (-locals.var_beta);
        let assign32820_e37256: f64 = (locals.var_phi_sl_dep - locals.var_vbscl__blk437);
        let assign32820_e37257: f64 = (assign32820_e37253 * assign32820_e37256);
        let assign32820_e37258: f64 = (assign32820_e37257).exp();
        (assign32820_e37258, (assign32820_e37258 * (((-locals.var_beta_dn0) * assign32820_e37256) + (assign32820_e37253 * (locals.var_phi_sl_dep_dn0 - locals.var_vbscl__blk437_dn0)))), (assign32820_e37258 * (((-locals.var_beta_dn2) * assign32820_e37256) + (assign32820_e37253 * (locals.var_phi_sl_dep_dn2 - locals.var_vbscl__blk437_dn2)))), (assign32820_e37258 * (((-locals.var_beta_dn4) * assign32820_e37256) + (assign32820_e37253 * (locals.var_phi_sl_dep_dn4 - locals.var_vbscl__blk437_dn4)))), (assign32820_e37258 * (((-locals.var_beta_dn5) * assign32820_e37256) + (assign32820_e37253 * (locals.var_phi_sl_dep_dn5 - locals.var_vbscl__blk437_dn5)))), (assign32820_e37258 * (((-locals.var_beta_dn6) * assign32820_e37256) + (assign32820_e37253 * (locals.var_phi_sl_dep_dn6 - locals.var_vbscl__blk437_dn6)))), (assign32820_e37258 * (((-locals.var_beta_dn7) * assign32820_e37256) + (assign32820_e37253 * (locals.var_phi_sl_dep_dn7 - locals.var_vbscl__blk437_dn7)))), (assign32820_e37258 * (((-locals.var_beta_dn8) * assign32820_e37256) + (assign32820_e37253 * (locals.var_phi_sl_dep_dn8 - locals.var_vbscl__blk437_dn8)))), (assign32820_e37258 * (((-locals.var_beta_dn9) * assign32820_e37256) + (assign32820_e37253 * (locals.var_phi_sl_dep_dn9 - locals.var_vbscl__blk437_dn9)))), (assign32820_e37258 * (((-locals.var_beta_dn10) * assign32820_e37256) + (assign32820_e37253 * (locals.var_phi_sl_dep_dn10 - locals.var_vbscl__blk437_dn10)))), (assign32820_e37258 * (((-locals.var_beta_dn11) * assign32820_e37256) + (assign32820_e37253 * (locals.var_phi_sl_dep_dn11 - locals.var_vbscl__blk437_dn11)))), (assign32820_e37258 * (((-locals.var_beta_dn14) * assign32820_e37256) + (assign32820_e37253 * (locals.var_phi_sl_dep_dn14 - locals.var_vbscl__blk437_dn14)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32820_e37260;
        locals.var_t3_dn0 = assign32820_e37260_d_n0;
        locals.var_t3_dn2 = assign32820_e37260_d_n2;
        locals.var_t3_dn4 = assign32820_e37260_d_n4;
        locals.var_t3_dn5 = assign32820_e37260_d_n5;
        locals.var_t3_dn6 = assign32820_e37260_d_n6;
        locals.var_t3_dn7 = assign32820_e37260_d_n7;
        locals.var_t3_dn8 = assign32820_e37260_d_n8;
        locals.var_t3_dn9 = assign32820_e37260_d_n9;
        locals.var_t3_dn10 = assign32820_e37260_d_n10;
        locals.var_t3_dn11 = assign32820_e37260_d_n11;
        locals.var_t3_dn14 = assign32820_e37260_d_n14;
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_106(
        locals: &mut StampLocals,
    ) {
        let (assign32830_e37278, assign32830_e37278_d_n0, assign32830_e37278_d_n2, assign32830_e37278_d_n4, assign32830_e37278_d_n5, assign32830_e37278_d_n6, assign32830_e37278_d_n7, assign32830_e37278_d_n8, assign32830_e37278_d_n9, assign32830_e37278_d_n10, assign32830_e37278_d_n11, assign32830_e37278_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) {
        let assign32830_e37271: f64 = (-locals.var_beta);
        let assign32830_e37274: f64 = (locals.var_phi_bl_dep - locals.var_vbscl__blk437);
        let assign32830_e37275: f64 = (assign32830_e37271 * assign32830_e37274);
        let assign32830_e37276: f64 = (assign32830_e37275).exp();
        (assign32830_e37276, (assign32830_e37276 * (((-locals.var_beta_dn0) * assign32830_e37274) + (assign32830_e37271 * (locals.var_phi_bl_dep_dn0 - locals.var_vbscl__blk437_dn0)))), (assign32830_e37276 * (((-locals.var_beta_dn2) * assign32830_e37274) + (assign32830_e37271 * (locals.var_phi_bl_dep_dn2 - locals.var_vbscl__blk437_dn2)))), (assign32830_e37276 * (((-locals.var_beta_dn4) * assign32830_e37274) + (assign32830_e37271 * (locals.var_phi_bl_dep_dn4 - locals.var_vbscl__blk437_dn4)))), (assign32830_e37276 * (((-locals.var_beta_dn5) * assign32830_e37274) + (assign32830_e37271 * (locals.var_phi_bl_dep_dn5 - locals.var_vbscl__blk437_dn5)))), (assign32830_e37276 * (((-locals.var_beta_dn6) * assign32830_e37274) + (assign32830_e37271 * (locals.var_phi_bl_dep_dn6 - locals.var_vbscl__blk437_dn6)))), (assign32830_e37276 * (((-locals.var_beta_dn7) * assign32830_e37274) + (assign32830_e37271 * (locals.var_phi_bl_dep_dn7 - locals.var_vbscl__blk437_dn7)))), (assign32830_e37276 * (((-locals.var_beta_dn8) * assign32830_e37274) + (assign32830_e37271 * (locals.var_phi_bl_dep_dn8 - locals.var_vbscl__blk437_dn8)))), (assign32830_e37276 * (((-locals.var_beta_dn9) * assign32830_e37274) + (assign32830_e37271 * (locals.var_phi_bl_dep_dn9 - locals.var_vbscl__blk437_dn9)))), (assign32830_e37276 * (((-locals.var_beta_dn10) * assign32830_e37274) + (assign32830_e37271 * (locals.var_phi_bl_dep_dn10 - locals.var_vbscl__blk437_dn10)))), (assign32830_e37276 * (((-locals.var_beta_dn11) * assign32830_e37274) + (assign32830_e37271 * (locals.var_phi_bl_dep_dn11 - locals.var_vbscl__blk437_dn11)))), (assign32830_e37276 * (((-locals.var_beta_dn14) * assign32830_e37274) + (assign32830_e37271 * (locals.var_phi_bl_dep_dn14 - locals.var_vbscl__blk437_dn14)))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign32830_e37278;
        locals.var_t4_dn0 = assign32830_e37278_d_n0;
        locals.var_t4_dn2 = assign32830_e37278_d_n2;
        locals.var_t4_dn4 = assign32830_e37278_d_n4;
        locals.var_t4_dn5 = assign32830_e37278_d_n5;
        locals.var_t4_dn6 = assign32830_e37278_d_n6;
        locals.var_t4_dn7 = assign32830_e37278_d_n7;
        locals.var_t4_dn8 = assign32830_e37278_d_n8;
        locals.var_t4_dn9 = assign32830_e37278_d_n9;
        locals.var_t4_dn10 = assign32830_e37278_d_n10;
        locals.var_t4_dn11 = assign32830_e37278_d_n11;
        locals.var_t4_dn14 = assign32830_e37278_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign32840_e37305, assign32840_e37305_d_n0, assign32840_e37305_d_n2, assign32840_e37305_d_n4, assign32840_e37305_d_n5, assign32840_e37305_d_n6, assign32840_e37305_d_n7, assign32840_e37305_d_n8, assign32840_e37305_d_n9, assign32840_e37305_d_n10, assign32840_e37305_d_n11, assign32840_e37305_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) {
        let assign32840_e37291: f64 = (locals.var_t2 - 1.0);
        let assign32840_e37293: f64 = (assign32840_e37291 - locals.var_t1);
        let assign32840_e37297: f64 = (locals.var_t3 - locals.var_t4);
        let assign32840_e37298: f64 = (locals.var_cnst1 * assign32840_e37297);
        let assign32840_e37299: f64 = (assign32840_e37293 + assign32840_e37298);
        let assign32840_e37301: f64 = (assign32840_e37299 + 1e-15);
        let assign32840_e37302: f64 = (assign32840_e37301).sqrt();
        let assign32840_e37303: f64 = (locals.var_cnst0 * assign32840_e37302);
        (assign32840_e37303, ((locals.var_cnst0_dn0 * assign32840_e37302) + (locals.var_cnst0 * (((locals.var_t2_dn0 - locals.var_t1_dn0) + ((locals.var_cnst1_dn0 * assign32840_e37297) + (locals.var_cnst1 * (locals.var_t3_dn0 - locals.var_t4_dn0)))) / (2.0 * assign32840_e37302)))), ((locals.var_cnst0_dn2 * assign32840_e37302) + (locals.var_cnst0 * (((locals.var_t2_dn2 - locals.var_t1_dn2) + ((locals.var_cnst1_dn2 * assign32840_e37297) + (locals.var_cnst1 * (locals.var_t3_dn2 - locals.var_t4_dn2)))) / (2.0 * assign32840_e37302)))), ((locals.var_cnst0_dn4 * assign32840_e37302) + (locals.var_cnst0 * (((locals.var_t2_dn4 - locals.var_t1_dn4) + ((locals.var_cnst1_dn4 * assign32840_e37297) + (locals.var_cnst1 * (locals.var_t3_dn4 - locals.var_t4_dn4)))) / (2.0 * assign32840_e37302)))), ((locals.var_cnst0_dn5 * assign32840_e37302) + (locals.var_cnst0 * (((locals.var_t2_dn5 - locals.var_t1_dn5) + ((locals.var_cnst1_dn5 * assign32840_e37297) + (locals.var_cnst1 * (locals.var_t3_dn5 - locals.var_t4_dn5)))) / (2.0 * assign32840_e37302)))), ((locals.var_cnst0_dn6 * assign32840_e37302) + (locals.var_cnst0 * (((locals.var_t2_dn6 - locals.var_t1_dn6) + ((locals.var_cnst1_dn6 * assign32840_e37297) + (locals.var_cnst1 * (locals.var_t3_dn6 - locals.var_t4_dn6)))) / (2.0 * assign32840_e37302)))), ((locals.var_cnst0_dn7 * assign32840_e37302) + (locals.var_cnst0 * (((locals.var_t2_dn7 - locals.var_t1_dn7) + ((locals.var_cnst1_dn7 * assign32840_e37297) + (locals.var_cnst1 * (locals.var_t3_dn7 - locals.var_t4_dn7)))) / (2.0 * assign32840_e37302)))), ((locals.var_cnst0_dn8 * assign32840_e37302) + (locals.var_cnst0 * (((locals.var_t2_dn8 - locals.var_t1_dn8) + ((locals.var_cnst1_dn8 * assign32840_e37297) + (locals.var_cnst1 * (locals.var_t3_dn8 - locals.var_t4_dn8)))) / (2.0 * assign32840_e37302)))), ((locals.var_cnst0_dn9 * assign32840_e37302) + (locals.var_cnst0 * (((locals.var_t2_dn9 - locals.var_t1_dn9) + ((locals.var_cnst1_dn9 * assign32840_e37297) + (locals.var_cnst1 * (locals.var_t3_dn9 - locals.var_t4_dn9)))) / (2.0 * assign32840_e37302)))), ((locals.var_cnst0_dn10 * assign32840_e37302) + (locals.var_cnst0 * (((locals.var_t2_dn10 - locals.var_t1_dn10) + ((locals.var_cnst1_dn10 * assign32840_e37297) + (locals.var_cnst1 * (locals.var_t3_dn10 - locals.var_t4_dn10)))) / (2.0 * assign32840_e37302)))), ((locals.var_cnst0_dn11 * assign32840_e37302) + (locals.var_cnst0 * (((locals.var_t2_dn11 - locals.var_t1_dn11) + ((locals.var_cnst1_dn11 * assign32840_e37297) + (locals.var_cnst1 * (locals.var_t3_dn11 - locals.var_t4_dn11)))) / (2.0 * assign32840_e37302)))), ((locals.var_cnst0_dn14 * assign32840_e37302) + (locals.var_cnst0 * (((locals.var_t2_dn14 - locals.var_t1_dn14) + ((locals.var_cnst1_dn14 * assign32840_e37297) + (locals.var_cnst1 * (locals.var_t3_dn14 - locals.var_t4_dn14)))) / (2.0 * assign32840_e37302)))),)
    } else {
        (locals.var_q_sl, locals.var_q_sl_dn0, locals.var_q_sl_dn2, locals.var_q_sl_dn4, locals.var_q_sl_dn5, locals.var_q_sl_dn6, locals.var_q_sl_dn7, locals.var_q_sl_dn8, locals.var_q_sl_dn9, locals.var_q_sl_dn10, locals.var_q_sl_dn11, locals.var_q_sl_dn14,)
    }
};
        locals.var_q_sl = assign32840_e37305;
        locals.var_q_sl_dn0 = assign32840_e37305_d_n0;
        locals.var_q_sl_dn2 = assign32840_e37305_d_n2;
        locals.var_q_sl_dn4 = assign32840_e37305_d_n4;
        locals.var_q_sl_dn5 = assign32840_e37305_d_n5;
        locals.var_q_sl_dn6 = assign32840_e37305_d_n6;
        locals.var_q_sl_dn7 = assign32840_e37305_d_n7;
        locals.var_q_sl_dn8 = assign32840_e37305_d_n8;
        locals.var_q_sl_dn9 = assign32840_e37305_d_n9;
        locals.var_q_sl_dn10 = assign32840_e37305_d_n10;
        locals.var_q_sl_dn11 = assign32840_e37305_d_n11;
        locals.var_q_sl_dn14 = assign32840_e37305_d_n14;
        locals.var_q_sl_rv = 0.0;

        let assign32850_e37312: f64 = if ((locals.var_w_bsubl > locals.var_uc_depthn) && (locals.var_depmode != 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard781 = assign32850_e37312;
        locals.var_guard781_rv = 0.0;

        let (assign32860_e37326, assign32860_e37326_d_n0, assign32860_e37326_d_n2, assign32860_e37326_d_n4, assign32860_e37326_d_n5, assign32860_e37326_d_n6, assign32860_e37326_d_n7, assign32860_e37326_d_n8, assign32860_e37326_d_n9, assign32860_e37326_d_n10, assign32860_e37326_d_n11, assign32860_e37326_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard781 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_subl, locals.var_q_subl_dn0, locals.var_q_subl_dn2, locals.var_q_subl_dn4, locals.var_q_subl_dn5, locals.var_q_subl_dn6, locals.var_q_subl_dn7, locals.var_q_subl_dn8, locals.var_q_subl_dn9, locals.var_q_subl_dn10, locals.var_q_subl_dn11, locals.var_q_subl_dn14,)
    }
};
        locals.var_q_subl = assign32860_e37326;
        locals.var_q_subl_dn0 = assign32860_e37326_d_n0;
        locals.var_q_subl_dn2 = assign32860_e37326_d_n2;
        locals.var_q_subl_dn4 = assign32860_e37326_d_n4;
        locals.var_q_subl_dn5 = assign32860_e37326_d_n5;
        locals.var_q_subl_dn6 = assign32860_e37326_d_n6;
        locals.var_q_subl_dn7 = assign32860_e37326_d_n7;
        locals.var_q_subl_dn8 = assign32860_e37326_d_n8;
        locals.var_q_subl_dn9 = assign32860_e37326_d_n9;
        locals.var_q_subl_dn10 = assign32860_e37326_d_n10;
        locals.var_q_subl_dn11 = assign32860_e37326_d_n11;
        locals.var_q_subl_dn14 = assign32860_e37326_d_n14;
        locals.var_q_subl_rv = 0.0;

        let (assign32870_e37340, assign32870_e37340_d_n0, assign32870_e37340_d_n2, assign32870_e37340_d_n4, assign32870_e37340_d_n5, assign32870_e37340_d_n6, assign32870_e37340_d_n7, assign32870_e37340_d_n8, assign32870_e37340_d_n9, assign32870_e37340_d_n10, assign32870_e37340_d_n11, assign32870_e37340_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard781 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_sl_dep, locals.var_q_sl_dep_dn0, locals.var_q_sl_dep_dn2, locals.var_q_sl_dep_dn4, locals.var_q_sl_dep_dn5, locals.var_q_sl_dep_dn6, locals.var_q_sl_dep_dn7, locals.var_q_sl_dep_dn8, locals.var_q_sl_dep_dn9, locals.var_q_sl_dep_dn10, locals.var_q_sl_dep_dn11, locals.var_q_sl_dep_dn14,)
    }
};
        locals.var_q_sl_dep = assign32870_e37340;
        locals.var_q_sl_dep_dn0 = assign32870_e37340_d_n0;
        locals.var_q_sl_dep_dn2 = assign32870_e37340_d_n2;
        locals.var_q_sl_dep_dn4 = assign32870_e37340_d_n4;
        locals.var_q_sl_dep_dn5 = assign32870_e37340_d_n5;
        locals.var_q_sl_dep_dn6 = assign32870_e37340_d_n6;
        locals.var_q_sl_dep_dn7 = assign32870_e37340_d_n7;
        locals.var_q_sl_dep_dn8 = assign32870_e37340_d_n8;
        locals.var_q_sl_dep_dn9 = assign32870_e37340_d_n9;
        locals.var_q_sl_dep_dn10 = assign32870_e37340_d_n10;
        locals.var_q_sl_dep_dn11 = assign32870_e37340_d_n11;
        locals.var_q_sl_dep_dn14 = assign32870_e37340_d_n14;
        locals.var_q_sl_dep_rv = 0.0;

        let (assign32880_e37377, assign32880_e37377_d_n0, assign32880_e37377_d_n2, assign32880_e37377_d_n4, assign32880_e37377_d_n5, assign32880_e37377_d_n6, assign32880_e37377_d_n7, assign32880_e37377_d_n8, assign32880_e37377_d_n9, assign32880_e37377_d_n10, assign32880_e37377_d_n11, assign32880_e37377_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard781 == 0.0)) {
        let assign32880_e37355: f64 = (-locals.var_t1);
        let assign32880_e37358: f64 = (-locals.var_beta);
        let assign32880_e37361: f64 = (locals.var_phi_sl_dep - locals.var_vbscl__blk437);
        let assign32880_e37362: f64 = (assign32880_e37358 * assign32880_e37361);
        let assign32880_e37363: f64 = (assign32880_e37362).exp();
        let assign32880_e37365: f64 = (-locals.var_beta);
        let assign32880_e37368: f64 = (locals.var_phi_bl_dep - locals.var_vbscl__blk437);
        let assign32880_e37369: f64 = (assign32880_e37365 * assign32880_e37368);
        let assign32880_e37370: f64 = (assign32880_e37369).exp();
        let assign32880_e37371: f64 = (assign32880_e37363 - assign32880_e37370);
        let assign32880_e37372: f64 = (locals.var_cnst1 * assign32880_e37371);
        let assign32880_e37373: f64 = (assign32880_e37355 + assign32880_e37372);
        let assign32880_e37374: f64 = (assign32880_e37373).sqrt();
        let assign32880_e37375: f64 = (locals.var_cnst0 * assign32880_e37374);
        (assign32880_e37375, ((locals.var_cnst0_dn0 * assign32880_e37374) + (locals.var_cnst0 * (((-locals.var_t1_dn0) + ((locals.var_cnst1_dn0 * assign32880_e37371) + (locals.var_cnst1 * ((assign32880_e37363 * (((-locals.var_beta_dn0) * assign32880_e37361) + (assign32880_e37358 * (locals.var_phi_sl_dep_dn0 - locals.var_vbscl__blk437_dn0)))) - (assign32880_e37370 * (((-locals.var_beta_dn0) * assign32880_e37368) + (assign32880_e37365 * (locals.var_phi_bl_dep_dn0 - locals.var_vbscl__blk437_dn0)))))))) / (2.0 * assign32880_e37374)))), ((locals.var_cnst0_dn2 * assign32880_e37374) + (locals.var_cnst0 * (((-locals.var_t1_dn2) + ((locals.var_cnst1_dn2 * assign32880_e37371) + (locals.var_cnst1 * ((assign32880_e37363 * (((-locals.var_beta_dn2) * assign32880_e37361) + (assign32880_e37358 * (locals.var_phi_sl_dep_dn2 - locals.var_vbscl__blk437_dn2)))) - (assign32880_e37370 * (((-locals.var_beta_dn2) * assign32880_e37368) + (assign32880_e37365 * (locals.var_phi_bl_dep_dn2 - locals.var_vbscl__blk437_dn2)))))))) / (2.0 * assign32880_e37374)))), ((locals.var_cnst0_dn4 * assign32880_e37374) + (locals.var_cnst0 * (((-locals.var_t1_dn4) + ((locals.var_cnst1_dn4 * assign32880_e37371) + (locals.var_cnst1 * ((assign32880_e37363 * (((-locals.var_beta_dn4) * assign32880_e37361) + (assign32880_e37358 * (locals.var_phi_sl_dep_dn4 - locals.var_vbscl__blk437_dn4)))) - (assign32880_e37370 * (((-locals.var_beta_dn4) * assign32880_e37368) + (assign32880_e37365 * (locals.var_phi_bl_dep_dn4 - locals.var_vbscl__blk437_dn4)))))))) / (2.0 * assign32880_e37374)))), ((locals.var_cnst0_dn5 * assign32880_e37374) + (locals.var_cnst0 * (((-locals.var_t1_dn5) + ((locals.var_cnst1_dn5 * assign32880_e37371) + (locals.var_cnst1 * ((assign32880_e37363 * (((-locals.var_beta_dn5) * assign32880_e37361) + (assign32880_e37358 * (locals.var_phi_sl_dep_dn5 - locals.var_vbscl__blk437_dn5)))) - (assign32880_e37370 * (((-locals.var_beta_dn5) * assign32880_e37368) + (assign32880_e37365 * (locals.var_phi_bl_dep_dn5 - locals.var_vbscl__blk437_dn5)))))))) / (2.0 * assign32880_e37374)))), ((locals.var_cnst0_dn6 * assign32880_e37374) + (locals.var_cnst0 * (((-locals.var_t1_dn6) + ((locals.var_cnst1_dn6 * assign32880_e37371) + (locals.var_cnst1 * ((assign32880_e37363 * (((-locals.var_beta_dn6) * assign32880_e37361) + (assign32880_e37358 * (locals.var_phi_sl_dep_dn6 - locals.var_vbscl__blk437_dn6)))) - (assign32880_e37370 * (((-locals.var_beta_dn6) * assign32880_e37368) + (assign32880_e37365 * (locals.var_phi_bl_dep_dn6 - locals.var_vbscl__blk437_dn6)))))))) / (2.0 * assign32880_e37374)))), ((locals.var_cnst0_dn7 * assign32880_e37374) + (locals.var_cnst0 * (((-locals.var_t1_dn7) + ((locals.var_cnst1_dn7 * assign32880_e37371) + (locals.var_cnst1 * ((assign32880_e37363 * (((-locals.var_beta_dn7) * assign32880_e37361) + (assign32880_e37358 * (locals.var_phi_sl_dep_dn7 - locals.var_vbscl__blk437_dn7)))) - (assign32880_e37370 * (((-locals.var_beta_dn7) * assign32880_e37368) + (assign32880_e37365 * (locals.var_phi_bl_dep_dn7 - locals.var_vbscl__blk437_dn7)))))))) / (2.0 * assign32880_e37374)))), ((locals.var_cnst0_dn8 * assign32880_e37374) + (locals.var_cnst0 * (((-locals.var_t1_dn8) + ((locals.var_cnst1_dn8 * assign32880_e37371) + (locals.var_cnst1 * ((assign32880_e37363 * (((-locals.var_beta_dn8) * assign32880_e37361) + (assign32880_e37358 * (locals.var_phi_sl_dep_dn8 - locals.var_vbscl__blk437_dn8)))) - (assign32880_e37370 * (((-locals.var_beta_dn8) * assign32880_e37368) + (assign32880_e37365 * (locals.var_phi_bl_dep_dn8 - locals.var_vbscl__blk437_dn8)))))))) / (2.0 * assign32880_e37374)))), ((locals.var_cnst0_dn9 * assign32880_e37374) + (locals.var_cnst0 * (((-locals.var_t1_dn9) + ((locals.var_cnst1_dn9 * assign32880_e37371) + (locals.var_cnst1 * ((assign32880_e37363 * (((-locals.var_beta_dn9) * assign32880_e37361) + (assign32880_e37358 * (locals.var_phi_sl_dep_dn9 - locals.var_vbscl__blk437_dn9)))) - (assign32880_e37370 * (((-locals.var_beta_dn9) * assign32880_e37368) + (assign32880_e37365 * (locals.var_phi_bl_dep_dn9 - locals.var_vbscl__blk437_dn9)))))))) / (2.0 * assign32880_e37374)))), ((locals.var_cnst0_dn10 * assign32880_e37374) + (locals.var_cnst0 * (((-locals.var_t1_dn10) + ((locals.var_cnst1_dn10 * assign32880_e37371) + (locals.var_cnst1 * ((assign32880_e37363 * (((-locals.var_beta_dn10) * assign32880_e37361) + (assign32880_e37358 * (locals.var_phi_sl_dep_dn10 - locals.var_vbscl__blk437_dn10)))) - (assign32880_e37370 * (((-locals.var_beta_dn10) * assign32880_e37368) + (assign32880_e37365 * (locals.var_phi_bl_dep_dn10 - locals.var_vbscl__blk437_dn10)))))))) / (2.0 * assign32880_e37374)))), ((locals.var_cnst0_dn11 * assign32880_e37374) + (locals.var_cnst0 * (((-locals.var_t1_dn11) + ((locals.var_cnst1_dn11 * assign32880_e37371) + (locals.var_cnst1 * ((assign32880_e37363 * (((-locals.var_beta_dn11) * assign32880_e37361) + (assign32880_e37358 * (locals.var_phi_sl_dep_dn11 - locals.var_vbscl__blk437_dn11)))) - (assign32880_e37370 * (((-locals.var_beta_dn11) * assign32880_e37368) + (assign32880_e37365 * (locals.var_phi_bl_dep_dn11 - locals.var_vbscl__blk437_dn11)))))))) / (2.0 * assign32880_e37374)))), ((locals.var_cnst0_dn14 * assign32880_e37374) + (locals.var_cnst0 * (((-locals.var_t1_dn14) + ((locals.var_cnst1_dn14 * assign32880_e37371) + (locals.var_cnst1 * ((assign32880_e37363 * (((-locals.var_beta_dn14) * assign32880_e37361) + (assign32880_e37358 * (locals.var_phi_sl_dep_dn14 - locals.var_vbscl__blk437_dn14)))) - (assign32880_e37370 * (((-locals.var_beta_dn14) * assign32880_e37368) + (assign32880_e37365 * (locals.var_phi_bl_dep_dn14 - locals.var_vbscl__blk437_dn14)))))))) / (2.0 * assign32880_e37374)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32880_e37377;
        locals.var_t3_dn0 = assign32880_e37377_d_n0;
        locals.var_t3_dn2 = assign32880_e37377_d_n2;
        locals.var_t3_dn4 = assign32880_e37377_d_n4;
        locals.var_t3_dn5 = assign32880_e37377_d_n5;
        locals.var_t3_dn6 = assign32880_e37377_d_n6;
        locals.var_t3_dn7 = assign32880_e37377_d_n7;
        locals.var_t3_dn8 = assign32880_e37377_d_n8;
        locals.var_t3_dn9 = assign32880_e37377_d_n9;
        locals.var_t3_dn10 = assign32880_e37377_d_n10;
        locals.var_t3_dn11 = assign32880_e37377_d_n11;
        locals.var_t3_dn14 = assign32880_e37377_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign32890_e37398, assign32890_e37398_d_n0, assign32890_e37398_d_n2, assign32890_e37398_d_n4, assign32890_e37398_d_n5, assign32890_e37398_d_n6, assign32890_e37398_d_n7, assign32890_e37398_d_n8, assign32890_e37398_d_n9, assign32890_e37398_d_n10, assign32890_e37398_d_n11, assign32890_e37398_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard781 == 0.0)) {
        let assign32890_e37393: f64 = (-locals.var_t1);
        let assign32890_e37394: f64 = (assign32890_e37393).sqrt();
        let assign32890_e37395: f64 = (locals.var_cnst0 * assign32890_e37394);
        let assign32890_e37396: f64 = (locals.var_t3 - assign32890_e37395);
        (assign32890_e37396, (locals.var_t3_dn0 - ((locals.var_cnst0_dn0 * assign32890_e37394) + (locals.var_cnst0 * ((-locals.var_t1_dn0) / (2.0 * assign32890_e37394))))), (locals.var_t3_dn2 - ((locals.var_cnst0_dn2 * assign32890_e37394) + (locals.var_cnst0 * ((-locals.var_t1_dn2) / (2.0 * assign32890_e37394))))), (locals.var_t3_dn4 - ((locals.var_cnst0_dn4 * assign32890_e37394) + (locals.var_cnst0 * ((-locals.var_t1_dn4) / (2.0 * assign32890_e37394))))), (locals.var_t3_dn5 - ((locals.var_cnst0_dn5 * assign32890_e37394) + (locals.var_cnst0 * ((-locals.var_t1_dn5) / (2.0 * assign32890_e37394))))), (locals.var_t3_dn6 - ((locals.var_cnst0_dn6 * assign32890_e37394) + (locals.var_cnst0 * ((-locals.var_t1_dn6) / (2.0 * assign32890_e37394))))), (locals.var_t3_dn7 - ((locals.var_cnst0_dn7 * assign32890_e37394) + (locals.var_cnst0 * ((-locals.var_t1_dn7) / (2.0 * assign32890_e37394))))), (locals.var_t3_dn8 - ((locals.var_cnst0_dn8 * assign32890_e37394) + (locals.var_cnst0 * ((-locals.var_t1_dn8) / (2.0 * assign32890_e37394))))), (locals.var_t3_dn9 - ((locals.var_cnst0_dn9 * assign32890_e37394) + (locals.var_cnst0 * ((-locals.var_t1_dn9) / (2.0 * assign32890_e37394))))), (locals.var_t3_dn10 - ((locals.var_cnst0_dn10 * assign32890_e37394) + (locals.var_cnst0 * ((-locals.var_t1_dn10) / (2.0 * assign32890_e37394))))), (locals.var_t3_dn11 - ((locals.var_cnst0_dn11 * assign32890_e37394) + (locals.var_cnst0 * ((-locals.var_t1_dn11) / (2.0 * assign32890_e37394))))), (locals.var_t3_dn14 - ((locals.var_cnst0_dn14 * assign32890_e37394) + (locals.var_cnst0 * ((-locals.var_t1_dn14) / (2.0 * assign32890_e37394))))),)
    } else {
        (locals.var_q_subl, locals.var_q_subl_dn0, locals.var_q_subl_dn2, locals.var_q_subl_dn4, locals.var_q_subl_dn5, locals.var_q_subl_dn6, locals.var_q_subl_dn7, locals.var_q_subl_dn8, locals.var_q_subl_dn9, locals.var_q_subl_dn10, locals.var_q_subl_dn11, locals.var_q_subl_dn14,)
    }
};
        locals.var_q_subl = assign32890_e37398;
        locals.var_q_subl_dn0 = assign32890_e37398_d_n0;
        locals.var_q_subl_dn2 = assign32890_e37398_d_n2;
        locals.var_q_subl_dn4 = assign32890_e37398_d_n4;
        locals.var_q_subl_dn5 = assign32890_e37398_d_n5;
        locals.var_q_subl_dn6 = assign32890_e37398_d_n6;
        locals.var_q_subl_dn7 = assign32890_e37398_d_n7;
        locals.var_q_subl_dn8 = assign32890_e37398_d_n8;
        locals.var_q_subl_dn9 = assign32890_e37398_d_n9;
        locals.var_q_subl_dn10 = assign32890_e37398_d_n10;
        locals.var_q_subl_dn11 = assign32890_e37398_d_n11;
        locals.var_q_subl_dn14 = assign32890_e37398_d_n14;
        locals.var_q_subl_rv = 0.0;

        let (assign32900_e37422, assign32900_e37422_d_n0, assign32900_e37422_d_n2, assign32900_e37422_d_n4, assign32900_e37422_d_n5, assign32900_e37422_d_n6, assign32900_e37422_d_n7, assign32900_e37422_d_n8, assign32900_e37422_d_n9, assign32900_e37422_d_n10, assign32900_e37422_d_n11, assign32900_e37422_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard781 == 0.0)) {
        let assign32900_e37414: f64 = (locals.var_t2 - 1.0);
        let assign32900_e37416: f64 = (assign32900_e37414 - locals.var_t1);
        let assign32900_e37418: f64 = (assign32900_e37416 + 1e-15);
        let assign32900_e37419: f64 = (assign32900_e37418).sqrt();
        let assign32900_e37420: f64 = (locals.var_cnst0 * assign32900_e37419);
        (assign32900_e37420, ((locals.var_cnst0_dn0 * assign32900_e37419) + (locals.var_cnst0 * ((locals.var_t2_dn0 - locals.var_t1_dn0) / (2.0 * assign32900_e37419)))), ((locals.var_cnst0_dn2 * assign32900_e37419) + (locals.var_cnst0 * ((locals.var_t2_dn2 - locals.var_t1_dn2) / (2.0 * assign32900_e37419)))), ((locals.var_cnst0_dn4 * assign32900_e37419) + (locals.var_cnst0 * ((locals.var_t2_dn4 - locals.var_t1_dn4) / (2.0 * assign32900_e37419)))), ((locals.var_cnst0_dn5 * assign32900_e37419) + (locals.var_cnst0 * ((locals.var_t2_dn5 - locals.var_t1_dn5) / (2.0 * assign32900_e37419)))), ((locals.var_cnst0_dn6 * assign32900_e37419) + (locals.var_cnst0 * ((locals.var_t2_dn6 - locals.var_t1_dn6) / (2.0 * assign32900_e37419)))), ((locals.var_cnst0_dn7 * assign32900_e37419) + (locals.var_cnst0 * ((locals.var_t2_dn7 - locals.var_t1_dn7) / (2.0 * assign32900_e37419)))), ((locals.var_cnst0_dn8 * assign32900_e37419) + (locals.var_cnst0 * ((locals.var_t2_dn8 - locals.var_t1_dn8) / (2.0 * assign32900_e37419)))), ((locals.var_cnst0_dn9 * assign32900_e37419) + (locals.var_cnst0 * ((locals.var_t2_dn9 - locals.var_t1_dn9) / (2.0 * assign32900_e37419)))), ((locals.var_cnst0_dn10 * assign32900_e37419) + (locals.var_cnst0 * ((locals.var_t2_dn10 - locals.var_t1_dn10) / (2.0 * assign32900_e37419)))), ((locals.var_cnst0_dn11 * assign32900_e37419) + (locals.var_cnst0 * ((locals.var_t2_dn11 - locals.var_t1_dn11) / (2.0 * assign32900_e37419)))), ((locals.var_cnst0_dn14 * assign32900_e37419) + (locals.var_cnst0 * ((locals.var_t2_dn14 - locals.var_t1_dn14) / (2.0 * assign32900_e37419)))),)
    } else {
        (locals.var_q_sl_dep, locals.var_q_sl_dep_dn0, locals.var_q_sl_dep_dn2, locals.var_q_sl_dep_dn4, locals.var_q_sl_dep_dn5, locals.var_q_sl_dep_dn6, locals.var_q_sl_dep_dn7, locals.var_q_sl_dep_dn8, locals.var_q_sl_dep_dn9, locals.var_q_sl_dep_dn10, locals.var_q_sl_dep_dn11, locals.var_q_sl_dep_dn14,)
    }
};
        locals.var_q_sl_dep = assign32900_e37422;
        locals.var_q_sl_dep_dn0 = assign32900_e37422_d_n0;
        locals.var_q_sl_dep_dn2 = assign32900_e37422_d_n2;
        locals.var_q_sl_dep_dn4 = assign32900_e37422_d_n4;
        locals.var_q_sl_dep_dn5 = assign32900_e37422_d_n5;
        locals.var_q_sl_dep_dn6 = assign32900_e37422_d_n6;
        locals.var_q_sl_dep_dn7 = assign32900_e37422_d_n7;
        locals.var_q_sl_dep_dn8 = assign32900_e37422_d_n8;
        locals.var_q_sl_dep_dn9 = assign32900_e37422_d_n9;
        locals.var_q_sl_dep_dn10 = assign32900_e37422_d_n10;
        locals.var_q_sl_dep_dn11 = assign32900_e37422_d_n11;
        locals.var_q_sl_dep_dn14 = assign32900_e37422_d_n14;
        locals.var_q_sl_dep_rv = 0.0;

        let (assign32910_e37434, assign32910_e37434_d_n0, assign32910_e37434_d_n2, assign32910_e37434_d_n4, assign32910_e37434_d_n5, assign32910_e37434_d_n6, assign32910_e37434_d_n7, assign32910_e37434_d_n8, assign32910_e37434_d_n9, assign32910_e37434_d_n10, assign32910_e37434_d_n11, assign32910_e37434_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_nl, locals.var_q_nl_dn0, locals.var_q_nl_dn2, locals.var_q_nl_dn4, locals.var_q_nl_dn5, locals.var_q_nl_dn6, locals.var_q_nl_dn7, locals.var_q_nl_dn8, locals.var_q_nl_dn9, locals.var_q_nl_dn10, locals.var_q_nl_dn11, locals.var_q_nl_dn14,)
    }
};
        locals.var_q_nl = assign32910_e37434;
        locals.var_q_nl_dn0 = assign32910_e37434_d_n0;
        locals.var_q_nl_dn2 = assign32910_e37434_d_n2;
        locals.var_q_nl_dn4 = assign32910_e37434_d_n4;
        locals.var_q_nl_dn5 = assign32910_e37434_d_n5;
        locals.var_q_nl_dn6 = assign32910_e37434_d_n6;
        locals.var_q_nl_dn7 = assign32910_e37434_d_n7;
        locals.var_q_nl_dn8 = assign32910_e37434_d_n8;
        locals.var_q_nl_dn9 = assign32910_e37434_d_n9;
        locals.var_q_nl_dn10 = assign32910_e37434_d_n10;
        locals.var_q_nl_dn11 = assign32910_e37434_d_n11;
        locals.var_q_nl_dn14 = assign32910_e37434_d_n14;
        locals.var_q_nl_rv = 0.0;

        let (assign32920_e37448, assign32920_e37448_d_n0, assign32920_e37448_d_n2, assign32920_e37448_d_n4, assign32920_e37448_d_n5, assign32920_e37448_d_n6, assign32920_e37448_d_n7, assign32920_e37448_d_n8, assign32920_e37448_d_n9, assign32920_e37448_d_n10, assign32920_e37448_d_n11, assign32920_e37448_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) {
        let assign32920_e37446: f64 = (locals.var_phi_bl_dep - locals.var_phi_jl_dep);
        (assign32920_e37446, (locals.var_phi_bl_dep_dn0 - locals.var_phi_jl_dep_dn0), (locals.var_phi_bl_dep_dn2 - locals.var_phi_jl_dep_dn2), (locals.var_phi_bl_dep_dn4 - locals.var_phi_jl_dep_dn4), (locals.var_phi_bl_dep_dn5 - locals.var_phi_jl_dep_dn5), (locals.var_phi_bl_dep_dn6 - locals.var_phi_jl_dep_dn6), (locals.var_phi_bl_dep_dn7 - locals.var_phi_jl_dep_dn7), (locals.var_phi_bl_dep_dn8 - locals.var_phi_jl_dep_dn8), (locals.var_phi_bl_dep_dn9 - locals.var_phi_jl_dep_dn9), (locals.var_phi_bl_dep_dn10 - locals.var_phi_jl_dep_dn10), (locals.var_phi_bl_dep_dn11 - locals.var_phi_jl_dep_dn11), (locals.var_phi_bl_dep_dn14 - locals.var_phi_jl_dep_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign32920_e37448;
        locals.var_t1_dn0 = assign32920_e37448_d_n0;
        locals.var_t1_dn2 = assign32920_e37448_d_n2;
        locals.var_t1_dn4 = assign32920_e37448_d_n4;
        locals.var_t1_dn5 = assign32920_e37448_d_n5;
        locals.var_t1_dn6 = assign32920_e37448_d_n6;
        locals.var_t1_dn7 = assign32920_e37448_d_n7;
        locals.var_t1_dn8 = assign32920_e37448_d_n8;
        locals.var_t1_dn9 = assign32920_e37448_d_n9;
        locals.var_t1_dn10 = assign32920_e37448_d_n10;
        locals.var_t1_dn11 = assign32920_e37448_d_n11;
        locals.var_t1_dn14 = assign32920_e37448_d_n14;
        locals.var_t1_rv = 0.0;

        let assign32930_e37452: f64 = 0.1;
        let assign32930_e37457: f64 = if ((locals.var_t1 < assign32930_e37452) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard782 = assign32930_e37457;
        locals.var_guard782_rv = 0.0;

        let (assign32940_e37475, assign32940_e37475_d_n0, assign32940_e37475_d_n2, assign32940_e37475_d_n4, assign32940_e37475_d_n5, assign32940_e37475_d_n6, assign32940_e37475_d_n7, assign32940_e37475_d_n8, assign32940_e37475_d_n9, assign32940_e37475_d_n10, assign32940_e37475_d_n11, assign32940_e37475_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign32940_e37471: f64 = 0.1;
        let assign32940_e37473: f64 = (assign32940_e37471 - locals.var_t1);
        (assign32940_e37473, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11), (-locals.var_t1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign32940_e37475;
        locals.var_tmf1_dn0 = assign32940_e37475_d_n0;
        locals.var_tmf1_dn2 = assign32940_e37475_d_n2;
        locals.var_tmf1_dn4 = assign32940_e37475_d_n4;
        locals.var_tmf1_dn5 = assign32940_e37475_d_n5;
        locals.var_tmf1_dn6 = assign32940_e37475_d_n6;
        locals.var_tmf1_dn7 = assign32940_e37475_d_n7;
        locals.var_tmf1_dn8 = assign32940_e37475_d_n8;
        locals.var_tmf1_dn9 = assign32940_e37475_d_n9;
        locals.var_tmf1_dn10 = assign32940_e37475_d_n10;
        locals.var_tmf1_dn11 = assign32940_e37475_d_n11;
        locals.var_tmf1_dn14 = assign32940_e37475_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign32950_e37491, assign32950_e37491_d_n0, assign32950_e37491_d_n2, assign32950_e37491_d_n4, assign32950_e37491_d_n5, assign32950_e37491_d_n6, assign32950_e37491_d_n7, assign32950_e37491_d_n8, assign32950_e37491_d_n9, assign32950_e37491_d_n10, assign32950_e37491_d_n11, assign32950_e37491_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign32950_e37489: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign32950_e37489, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign32950_e37491;
        locals.var_x2_dn0 = assign32950_e37491_d_n0;
        locals.var_x2_dn2 = assign32950_e37491_d_n2;
        locals.var_x2_dn4 = assign32950_e37491_d_n4;
        locals.var_x2_dn5 = assign32950_e37491_d_n5;
        locals.var_x2_dn6 = assign32950_e37491_d_n6;
        locals.var_x2_dn7 = assign32950_e37491_d_n7;
        locals.var_x2_dn8 = assign32950_e37491_d_n8;
        locals.var_x2_dn9 = assign32950_e37491_d_n9;
        locals.var_x2_dn10 = assign32950_e37491_d_n10;
        locals.var_x2_dn11 = assign32950_e37491_d_n11;
        locals.var_x2_dn14 = assign32950_e37491_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign32960_e37507, assign32960_e37507_d_n0, assign32960_e37507_d_n2, assign32960_e37507_d_n4, assign32960_e37507_d_n5, assign32960_e37507_d_n6, assign32960_e37507_d_n7, assign32960_e37507_d_n8, assign32960_e37507_d_n9, assign32960_e37507_d_n10, assign32960_e37507_d_n11, assign32960_e37507_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign32960_e37505: f64 = (0.1 * 0.1);
        (assign32960_e37505, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign32960_e37507;
        locals.var_xmax2_dn0 = assign32960_e37507_d_n0;
        locals.var_xmax2_dn2 = assign32960_e37507_d_n2;
        locals.var_xmax2_dn4 = assign32960_e37507_d_n4;
        locals.var_xmax2_dn5 = assign32960_e37507_d_n5;
        locals.var_xmax2_dn6 = assign32960_e37507_d_n6;
        locals.var_xmax2_dn7 = assign32960_e37507_d_n7;
        locals.var_xmax2_dn8 = assign32960_e37507_d_n8;
        locals.var_xmax2_dn9 = assign32960_e37507_d_n9;
        locals.var_xmax2_dn10 = assign32960_e37507_d_n10;
        locals.var_xmax2_dn11 = assign32960_e37507_d_n11;
        locals.var_xmax2_dn14 = assign32960_e37507_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign32970_e37521, assign32970_e37521_d_n0, assign32970_e37521_d_n2, assign32970_e37521_d_n4, assign32970_e37521_d_n5, assign32970_e37521_d_n6, assign32970_e37521_d_n7, assign32970_e37521_d_n8, assign32970_e37521_d_n9, assign32970_e37521_d_n10, assign32970_e37521_d_n11, assign32970_e37521_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard782 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign32970_e37521;
        locals.var_xp_dn0 = assign32970_e37521_d_n0;
        locals.var_xp_dn2 = assign32970_e37521_d_n2;
        locals.var_xp_dn4 = assign32970_e37521_d_n4;
        locals.var_xp_dn5 = assign32970_e37521_d_n5;
        locals.var_xp_dn6 = assign32970_e37521_d_n6;
        locals.var_xp_dn7 = assign32970_e37521_d_n7;
        locals.var_xp_dn8 = assign32970_e37521_d_n8;
        locals.var_xp_dn9 = assign32970_e37521_d_n9;
        locals.var_xp_dn10 = assign32970_e37521_d_n10;
        locals.var_xp_dn11 = assign32970_e37521_d_n11;
        locals.var_xp_dn14 = assign32970_e37521_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign32980_e37535, assign32980_e37535_d_n0, assign32980_e37535_d_n2, assign32980_e37535_d_n4, assign32980_e37535_d_n5, assign32980_e37535_d_n6, assign32980_e37535_d_n7, assign32980_e37535_d_n8, assign32980_e37535_d_n9, assign32980_e37535_d_n10, assign32980_e37535_d_n11, assign32980_e37535_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard782 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign32980_e37535;
        locals.var_xmp_dn0 = assign32980_e37535_d_n0;
        locals.var_xmp_dn2 = assign32980_e37535_d_n2;
        locals.var_xmp_dn4 = assign32980_e37535_d_n4;
        locals.var_xmp_dn5 = assign32980_e37535_d_n5;
        locals.var_xmp_dn6 = assign32980_e37535_d_n6;
        locals.var_xmp_dn7 = assign32980_e37535_d_n7;
        locals.var_xmp_dn8 = assign32980_e37535_d_n8;
        locals.var_xmp_dn9 = assign32980_e37535_d_n9;
        locals.var_xmp_dn10 = assign32980_e37535_d_n10;
        locals.var_xmp_dn11 = assign32980_e37535_d_n11;
        locals.var_xmp_dn14 = assign32980_e37535_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign32990_e37549,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard782 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign32990_e37549;
        locals.var_m0_rv = 0.0;

        let (assign33000_e37563,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard782 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33000_e37563;
        locals.var_mm_rv = 0.0;

        let (assign33010_e37577, assign33010_e37577_d_n0, assign33010_e37577_d_n2, assign33010_e37577_d_n4, assign33010_e37577_d_n5, assign33010_e37577_d_n6, assign33010_e37577_d_n7, assign33010_e37577_d_n8, assign33010_e37577_d_n9, assign33010_e37577_d_n10, assign33010_e37577_d_n11, assign33010_e37577_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard782 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign33010_e37577;
        locals.var_arg_dn0 = assign33010_e37577_d_n0;
        locals.var_arg_dn2 = assign33010_e37577_d_n2;
        locals.var_arg_dn4 = assign33010_e37577_d_n4;
        locals.var_arg_dn5 = assign33010_e37577_d_n5;
        locals.var_arg_dn6 = assign33010_e37577_d_n6;
        locals.var_arg_dn7 = assign33010_e37577_d_n7;
        locals.var_arg_dn8 = assign33010_e37577_d_n8;
        locals.var_arg_dn9 = assign33010_e37577_d_n9;
        locals.var_arg_dn10 = assign33010_e37577_d_n10;
        locals.var_arg_dn11 = assign33010_e37577_d_n11;
        locals.var_arg_dn14 = assign33010_e37577_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign33020_e37591, assign33020_e37591_d_n0, assign33020_e37591_d_n2, assign33020_e37591_d_n4, assign33020_e37591_d_n5, assign33020_e37591_d_n6, assign33020_e37591_d_n7, assign33020_e37591_d_n8, assign33020_e37591_d_n9, assign33020_e37591_d_n10, assign33020_e37591_d_n11, assign33020_e37591_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard782 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign33020_e37591;
        locals.var_dnm_dn0 = assign33020_e37591_d_n0;
        locals.var_dnm_dn2 = assign33020_e37591_d_n2;
        locals.var_dnm_dn4 = assign33020_e37591_d_n4;
        locals.var_dnm_dn5 = assign33020_e37591_d_n5;
        locals.var_dnm_dn6 = assign33020_e37591_d_n6;
        locals.var_dnm_dn7 = assign33020_e37591_d_n7;
        locals.var_dnm_dn8 = assign33020_e37591_d_n8;
        locals.var_dnm_dn9 = assign33020_e37591_d_n9;
        locals.var_dnm_dn10 = assign33020_e37591_d_n10;
        locals.var_dnm_dn11 = assign33020_e37591_d_n11;
        locals.var_dnm_dn14 = assign33020_e37591_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign33030_e37607, assign33030_e37607_d_n0, assign33030_e37607_d_n2, assign33030_e37607_d_n4, assign33030_e37607_d_n5, assign33030_e37607_d_n6, assign33030_e37607_d_n7, assign33030_e37607_d_n8, assign33030_e37607_d_n9, assign33030_e37607_d_n10, assign33030_e37607_d_n11, assign33030_e37607_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign33030_e37605: f64 = (locals.var_xp * locals.var_x2);
        (assign33030_e37605, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign33030_e37607;
        locals.var_xp_dn0 = assign33030_e37607_d_n0;
        locals.var_xp_dn2 = assign33030_e37607_d_n2;
        locals.var_xp_dn4 = assign33030_e37607_d_n4;
        locals.var_xp_dn5 = assign33030_e37607_d_n5;
        locals.var_xp_dn6 = assign33030_e37607_d_n6;
        locals.var_xp_dn7 = assign33030_e37607_d_n7;
        locals.var_xp_dn8 = assign33030_e37607_d_n8;
        locals.var_xp_dn9 = assign33030_e37607_d_n9;
        locals.var_xp_dn10 = assign33030_e37607_d_n10;
        locals.var_xp_dn11 = assign33030_e37607_d_n11;
        locals.var_xp_dn14 = assign33030_e37607_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign33040_e37623, assign33040_e37623_d_n0, assign33040_e37623_d_n2, assign33040_e37623_d_n4, assign33040_e37623_d_n5, assign33040_e37623_d_n6, assign33040_e37623_d_n7, assign33040_e37623_d_n8, assign33040_e37623_d_n9, assign33040_e37623_d_n10, assign33040_e37623_d_n11, assign33040_e37623_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign33040_e37621: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign33040_e37621, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign33040_e37623;
        locals.var_xmp_dn0 = assign33040_e37623_d_n0;
        locals.var_xmp_dn2 = assign33040_e37623_d_n2;
        locals.var_xmp_dn4 = assign33040_e37623_d_n4;
        locals.var_xmp_dn5 = assign33040_e37623_d_n5;
        locals.var_xmp_dn6 = assign33040_e37623_d_n6;
        locals.var_xmp_dn7 = assign33040_e37623_d_n7;
        locals.var_xmp_dn8 = assign33040_e37623_d_n8;
        locals.var_xmp_dn9 = assign33040_e37623_d_n9;
        locals.var_xmp_dn10 = assign33040_e37623_d_n10;
        locals.var_xmp_dn11 = assign33040_e37623_d_n11;
        locals.var_xmp_dn14 = assign33040_e37623_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign33050_e37639, assign33050_e37639_d_n0, assign33050_e37639_d_n2, assign33050_e37639_d_n4, assign33050_e37639_d_n5, assign33050_e37639_d_n6, assign33050_e37639_d_n7, assign33050_e37639_d_n8, assign33050_e37639_d_n9, assign33050_e37639_d_n10, assign33050_e37639_d_n11, assign33050_e37639_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign33050_e37637: f64 = (locals.var_xp * locals.var_x2);
        (assign33050_e37637, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign33050_e37639;
        locals.var_xp_dn0 = assign33050_e37639_d_n0;
        locals.var_xp_dn2 = assign33050_e37639_d_n2;
        locals.var_xp_dn4 = assign33050_e37639_d_n4;
        locals.var_xp_dn5 = assign33050_e37639_d_n5;
        locals.var_xp_dn6 = assign33050_e37639_d_n6;
        locals.var_xp_dn7 = assign33050_e37639_d_n7;
        locals.var_xp_dn8 = assign33050_e37639_d_n8;
        locals.var_xp_dn9 = assign33050_e37639_d_n9;
        locals.var_xp_dn10 = assign33050_e37639_d_n10;
        locals.var_xp_dn11 = assign33050_e37639_d_n11;
        locals.var_xp_dn14 = assign33050_e37639_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign33060_e37655, assign33060_e37655_d_n0, assign33060_e37655_d_n2, assign33060_e37655_d_n4, assign33060_e37655_d_n5, assign33060_e37655_d_n6, assign33060_e37655_d_n7, assign33060_e37655_d_n8, assign33060_e37655_d_n9, assign33060_e37655_d_n10, assign33060_e37655_d_n11, assign33060_e37655_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign33060_e37653: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign33060_e37653, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign33060_e37655;
        locals.var_xmp_dn0 = assign33060_e37655_d_n0;
        locals.var_xmp_dn2 = assign33060_e37655_d_n2;
        locals.var_xmp_dn4 = assign33060_e37655_d_n4;
        locals.var_xmp_dn5 = assign33060_e37655_d_n5;
        locals.var_xmp_dn6 = assign33060_e37655_d_n6;
        locals.var_xmp_dn7 = assign33060_e37655_d_n7;
        locals.var_xmp_dn8 = assign33060_e37655_d_n8;
        locals.var_xmp_dn9 = assign33060_e37655_d_n9;
        locals.var_xmp_dn10 = assign33060_e37655_d_n10;
        locals.var_xmp_dn11 = assign33060_e37655_d_n11;
        locals.var_xmp_dn14 = assign33060_e37655_d_n14;
        locals.var_xmp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_107(
        locals: &mut StampLocals,
    ) {
        let (assign33070_e37671, assign33070_e37671_d_n0, assign33070_e37671_d_n2, assign33070_e37671_d_n4, assign33070_e37671_d_n5, assign33070_e37671_d_n6, assign33070_e37671_d_n7, assign33070_e37671_d_n8, assign33070_e37671_d_n9, assign33070_e37671_d_n10, assign33070_e37671_d_n11, assign33070_e37671_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign33070_e37669: f64 = (locals.var_xp + locals.var_xmp);
        (assign33070_e37669, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign33070_e37671;
        locals.var_arg_dn0 = assign33070_e37671_d_n0;
        locals.var_arg_dn2 = assign33070_e37671_d_n2;
        locals.var_arg_dn4 = assign33070_e37671_d_n4;
        locals.var_arg_dn5 = assign33070_e37671_d_n5;
        locals.var_arg_dn6 = assign33070_e37671_d_n6;
        locals.var_arg_dn7 = assign33070_e37671_d_n7;
        locals.var_arg_dn8 = assign33070_e37671_d_n8;
        locals.var_arg_dn9 = assign33070_e37671_d_n9;
        locals.var_arg_dn10 = assign33070_e37671_d_n10;
        locals.var_arg_dn11 = assign33070_e37671_d_n11;
        locals.var_arg_dn14 = assign33070_e37671_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign33080_e37685, assign33080_e37685_d_n0, assign33080_e37685_d_n2, assign33080_e37685_d_n4, assign33080_e37685_d_n5, assign33080_e37685_d_n6, assign33080_e37685_d_n7, assign33080_e37685_d_n8, assign33080_e37685_d_n9, assign33080_e37685_d_n10, assign33080_e37685_d_n11, assign33080_e37685_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard782 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign33080_e37685;
        locals.var_dnm_dn0 = assign33080_e37685_d_n0;
        locals.var_dnm_dn2 = assign33080_e37685_d_n2;
        locals.var_dnm_dn4 = assign33080_e37685_d_n4;
        locals.var_dnm_dn5 = assign33080_e37685_d_n5;
        locals.var_dnm_dn6 = assign33080_e37685_d_n6;
        locals.var_dnm_dn7 = assign33080_e37685_d_n7;
        locals.var_dnm_dn8 = assign33080_e37685_d_n8;
        locals.var_dnm_dn9 = assign33080_e37685_d_n9;
        locals.var_dnm_dn10 = assign33080_e37685_d_n10;
        locals.var_dnm_dn11 = assign33080_e37685_d_n11;
        locals.var_dnm_dn14 = assign33080_e37685_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign33090_e37700: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard783 = assign33090_e37700;
        locals.var_guard783_rv = 0.0;

        let assign33100_e37703: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard784 = assign33100_e37703;
        locals.var_guard784_rv = 0.0;

        let (assign33110_e37721,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard782 != 0.0)) && (locals.var_guard783 != 0.0)) && (locals.var_guard784 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33110_e37721;
        locals.var_mm_rv = 0.0;

        let assign33120_e37724: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard785 = assign33120_e37724;
        locals.var_guard785_rv = 0.0;

        let (assign33130_e37745,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard782 != 0.0)) && (locals.var_guard783 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard785 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33130_e37745;
        locals.var_mm_rv = 0.0;

        let assign33140_e37748: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard786 = assign33140_e37748;
        locals.var_guard786_rv = 0.0;

        let (assign33150_e37772,) = {
    if (((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard782 != 0.0)) && (locals.var_guard783 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard785 == 0.0)) && (locals.var_guard786 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33150_e37772;
        locals.var_mm_rv = 0.0;

        let assign33160_e37775: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard787 = assign33160_e37775;
        locals.var_guard787_rv = 0.0;

        let (assign33170_e37802,) = {
    if ((((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard782 != 0.0)) && (locals.var_guard783 != 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard785 == 0.0)) && (locals.var_guard786 == 0.0)) && (locals.var_guard787 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33170_e37802;
        locals.var_mm_rv = 0.0;

        let (assign33180_e37818,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard782 != 0.0)) && (locals.var_guard783 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign33180_e37818;
        locals.var_m0_rv = 0.0;

        let mut assign33190_loop_guard: usize = 0;
        while {
            let assign33190_cond_e37835: f64 = if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard782 != 0.0)) && (locals.var_guard783 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign33190_cond_e37835 != 0.0
        } {
            assign33190_loop_guard += 1;
            assert!(assign33190_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign33190_body0_e37852, assign33190_body0_e37852_d_n0, assign33190_body0_e37852_d_n2, assign33190_body0_e37852_d_n4, assign33190_body0_e37852_d_n5, assign33190_body0_e37852_d_n6, assign33190_body0_e37852_d_n7, assign33190_body0_e37852_d_n8, assign33190_body0_e37852_d_n9, assign33190_body0_e37852_d_n10, assign33190_body0_e37852_d_n11, assign33190_body0_e37852_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard782 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign33190_body0_e37850: f64 = (locals.var_dnm).sqrt();
        (assign33190_body0_e37850, (locals.var_dnm_dn0 / (2.0 * assign33190_body0_e37850)), (locals.var_dnm_dn2 / (2.0 * assign33190_body0_e37850)), (locals.var_dnm_dn4 / (2.0 * assign33190_body0_e37850)), (locals.var_dnm_dn5 / (2.0 * assign33190_body0_e37850)), (locals.var_dnm_dn6 / (2.0 * assign33190_body0_e37850)), (locals.var_dnm_dn7 / (2.0 * assign33190_body0_e37850)), (locals.var_dnm_dn8 / (2.0 * assign33190_body0_e37850)), (locals.var_dnm_dn9 / (2.0 * assign33190_body0_e37850)), (locals.var_dnm_dn10 / (2.0 * assign33190_body0_e37850)), (locals.var_dnm_dn11 / (2.0 * assign33190_body0_e37850)), (locals.var_dnm_dn14 / (2.0 * assign33190_body0_e37850)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign33190_body0_e37852;
            locals.var_dnm_dn0 = assign33190_body0_e37852_d_n0;
            locals.var_dnm_dn2 = assign33190_body0_e37852_d_n2;
            locals.var_dnm_dn4 = assign33190_body0_e37852_d_n4;
            locals.var_dnm_dn5 = assign33190_body0_e37852_d_n5;
            locals.var_dnm_dn6 = assign33190_body0_e37852_d_n6;
            locals.var_dnm_dn7 = assign33190_body0_e37852_d_n7;
            locals.var_dnm_dn8 = assign33190_body0_e37852_d_n8;
            locals.var_dnm_dn9 = assign33190_body0_e37852_d_n9;
            locals.var_dnm_dn10 = assign33190_body0_e37852_d_n10;
            locals.var_dnm_dn11 = assign33190_body0_e37852_d_n11;
            locals.var_dnm_dn14 = assign33190_body0_e37852_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign33190_body1_e37870,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard782 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign33190_body1_e37868: f64 = (locals.var_m0 + 1.0);
        (assign33190_body1_e37868,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign33190_body1_e37870;
            locals.var_m0_rv = 0.0;
        }

        let (assign33200_e37898, assign33200_e37898_d_n0, assign33200_e37898_d_n2, assign33200_e37898_d_n4, assign33200_e37898_d_n5, assign33200_e37898_d_n6, assign33200_e37898_d_n7, assign33200_e37898_d_n8, assign33200_e37898_d_n9, assign33200_e37898_d_n10, assign33200_e37898_d_n11, assign33200_e37898_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard782 != 0.0)) && (locals.var_guard783 == 0.0)) {
        let (assign33200_e37896, assign33200_e37896_d_n0, assign33200_e37896_d_n2, assign33200_e37896_d_n4, assign33200_e37896_d_n5, assign33200_e37896_d_n6, assign33200_e37896_d_n7, assign33200_e37896_d_n8, assign33200_e37896_d_n9, assign33200_e37896_d_n10, assign33200_e37896_d_n11, assign33200_e37896_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign33200_e37893: f64 = (2.0 * 2.0);
                let assign33200_e37894: f64 = (1.0 / assign33200_e37893);
                let assign33200_e37895: f64 = (locals.var_dnm).powf(assign33200_e37894);
                (assign33200_e37895, if 0.0 == 0.0 && ((assign33200_e37894) as f64).is_finite() && ((assign33200_e37894) as f64).fract() == 0.0 { if assign33200_e37894 == 0.0 { 0.0 } else { (assign33200_e37894 * ((locals.var_dnm).powf(assign33200_e37894 - 1.0) * locals.var_dnm_dn0)) } } else { (assign33200_e37895 * (assign33200_e37894 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33200_e37894) as f64).is_finite() && ((assign33200_e37894) as f64).fract() == 0.0 { if assign33200_e37894 == 0.0 { 0.0 } else { (assign33200_e37894 * ((locals.var_dnm).powf(assign33200_e37894 - 1.0) * locals.var_dnm_dn2)) } } else { (assign33200_e37895 * (assign33200_e37894 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33200_e37894) as f64).is_finite() && ((assign33200_e37894) as f64).fract() == 0.0 { if assign33200_e37894 == 0.0 { 0.0 } else { (assign33200_e37894 * ((locals.var_dnm).powf(assign33200_e37894 - 1.0) * locals.var_dnm_dn4)) } } else { (assign33200_e37895 * (assign33200_e37894 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33200_e37894) as f64).is_finite() && ((assign33200_e37894) as f64).fract() == 0.0 { if assign33200_e37894 == 0.0 { 0.0 } else { (assign33200_e37894 * ((locals.var_dnm).powf(assign33200_e37894 - 1.0) * locals.var_dnm_dn5)) } } else { (assign33200_e37895 * (assign33200_e37894 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33200_e37894) as f64).is_finite() && ((assign33200_e37894) as f64).fract() == 0.0 { if assign33200_e37894 == 0.0 { 0.0 } else { (assign33200_e37894 * ((locals.var_dnm).powf(assign33200_e37894 - 1.0) * locals.var_dnm_dn6)) } } else { (assign33200_e37895 * (assign33200_e37894 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33200_e37894) as f64).is_finite() && ((assign33200_e37894) as f64).fract() == 0.0 { if assign33200_e37894 == 0.0 { 0.0 } else { (assign33200_e37894 * ((locals.var_dnm).powf(assign33200_e37894 - 1.0) * locals.var_dnm_dn7)) } } else { (assign33200_e37895 * (assign33200_e37894 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33200_e37894) as f64).is_finite() && ((assign33200_e37894) as f64).fract() == 0.0 { if assign33200_e37894 == 0.0 { 0.0 } else { (assign33200_e37894 * ((locals.var_dnm).powf(assign33200_e37894 - 1.0) * locals.var_dnm_dn8)) } } else { (assign33200_e37895 * (assign33200_e37894 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33200_e37894) as f64).is_finite() && ((assign33200_e37894) as f64).fract() == 0.0 { if assign33200_e37894 == 0.0 { 0.0 } else { (assign33200_e37894 * ((locals.var_dnm).powf(assign33200_e37894 - 1.0) * locals.var_dnm_dn9)) } } else { (assign33200_e37895 * (assign33200_e37894 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33200_e37894) as f64).is_finite() && ((assign33200_e37894) as f64).fract() == 0.0 { if assign33200_e37894 == 0.0 { 0.0 } else { (assign33200_e37894 * ((locals.var_dnm).powf(assign33200_e37894 - 1.0) * locals.var_dnm_dn10)) } } else { (assign33200_e37895 * (assign33200_e37894 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33200_e37894) as f64).is_finite() && ((assign33200_e37894) as f64).fract() == 0.0 { if assign33200_e37894 == 0.0 { 0.0 } else { (assign33200_e37894 * ((locals.var_dnm).powf(assign33200_e37894 - 1.0) * locals.var_dnm_dn11)) } } else { (assign33200_e37895 * (assign33200_e37894 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33200_e37894) as f64).is_finite() && ((assign33200_e37894) as f64).fract() == 0.0 { if assign33200_e37894 == 0.0 { 0.0 } else { (assign33200_e37894 * ((locals.var_dnm).powf(assign33200_e37894 - 1.0) * locals.var_dnm_dn14)) } } else { (assign33200_e37895 * (assign33200_e37894 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign33200_e37896, assign33200_e37896_d_n0, assign33200_e37896_d_n2, assign33200_e37896_d_n4, assign33200_e37896_d_n5, assign33200_e37896_d_n6, assign33200_e37896_d_n7, assign33200_e37896_d_n8, assign33200_e37896_d_n9, assign33200_e37896_d_n10, assign33200_e37896_d_n11, assign33200_e37896_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign33200_e37898;
        locals.var_dnm_dn0 = assign33200_e37898_d_n0;
        locals.var_dnm_dn2 = assign33200_e37898_d_n2;
        locals.var_dnm_dn4 = assign33200_e37898_d_n4;
        locals.var_dnm_dn5 = assign33200_e37898_d_n5;
        locals.var_dnm_dn6 = assign33200_e37898_d_n6;
        locals.var_dnm_dn7 = assign33200_e37898_d_n7;
        locals.var_dnm_dn8 = assign33200_e37898_d_n8;
        locals.var_dnm_dn9 = assign33200_e37898_d_n9;
        locals.var_dnm_dn10 = assign33200_e37898_d_n10;
        locals.var_dnm_dn11 = assign33200_e37898_d_n11;
        locals.var_dnm_dn14 = assign33200_e37898_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign33210_e37914, assign33210_e37914_d_n0, assign33210_e37914_d_n2, assign33210_e37914_d_n4, assign33210_e37914_d_n5, assign33210_e37914_d_n6, assign33210_e37914_d_n7, assign33210_e37914_d_n8, assign33210_e37914_d_n9, assign33210_e37914_d_n10, assign33210_e37914_d_n11, assign33210_e37914_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign33210_e37912: f64 = (1.0 / locals.var_dnm);
        (assign33210_e37912, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign33210_e37914;
        locals.var_dnm_dn0 = assign33210_e37914_d_n0;
        locals.var_dnm_dn2 = assign33210_e37914_d_n2;
        locals.var_dnm_dn4 = assign33210_e37914_d_n4;
        locals.var_dnm_dn5 = assign33210_e37914_d_n5;
        locals.var_dnm_dn6 = assign33210_e37914_d_n6;
        locals.var_dnm_dn7 = assign33210_e37914_d_n7;
        locals.var_dnm_dn8 = assign33210_e37914_d_n8;
        locals.var_dnm_dn9 = assign33210_e37914_d_n9;
        locals.var_dnm_dn10 = assign33210_e37914_d_n10;
        locals.var_dnm_dn11 = assign33210_e37914_d_n11;
        locals.var_dnm_dn14 = assign33210_e37914_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign33220_e37932, assign33220_e37932_d_n0, assign33220_e37932_d_n2, assign33220_e37932_d_n4, assign33220_e37932_d_n5, assign33220_e37932_d_n6, assign33220_e37932_d_n7, assign33220_e37932_d_n8, assign33220_e37932_d_n9, assign33220_e37932_d_n10, assign33220_e37932_d_n11, assign33220_e37932_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign33220_e37928: f64 = (locals.var_tmf1 * 0.1);
        let assign33220_e37930: f64 = (assign33220_e37928 * locals.var_dnm);
        (assign33220_e37930, (((locals.var_tmf1_dn0 * 0.1) * locals.var_dnm) + (assign33220_e37928 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.1) * locals.var_dnm) + (assign33220_e37928 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.1) * locals.var_dnm) + (assign33220_e37928 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.1) * locals.var_dnm) + (assign33220_e37928 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.1) * locals.var_dnm) + (assign33220_e37928 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.1) * locals.var_dnm) + (assign33220_e37928 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.1) * locals.var_dnm) + (assign33220_e37928 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.1) * locals.var_dnm) + (assign33220_e37928 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.1) * locals.var_dnm) + (assign33220_e37928 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.1) * locals.var_dnm) + (assign33220_e37928 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.1) * locals.var_dnm) + (assign33220_e37928 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign33220_e37932;
        locals.var_tmf0_dn0 = assign33220_e37932_d_n0;
        locals.var_tmf0_dn2 = assign33220_e37932_d_n2;
        locals.var_tmf0_dn4 = assign33220_e37932_d_n4;
        locals.var_tmf0_dn5 = assign33220_e37932_d_n5;
        locals.var_tmf0_dn6 = assign33220_e37932_d_n6;
        locals.var_tmf0_dn7 = assign33220_e37932_d_n7;
        locals.var_tmf0_dn8 = assign33220_e37932_d_n8;
        locals.var_tmf0_dn9 = assign33220_e37932_d_n9;
        locals.var_tmf0_dn10 = assign33220_e37932_d_n10;
        locals.var_tmf0_dn11 = assign33220_e37932_d_n11;
        locals.var_tmf0_dn14 = assign33220_e37932_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign33230_e37952, assign33230_e37952_d_n0, assign33230_e37952_d_n2, assign33230_e37952_d_n4, assign33230_e37952_d_n5, assign33230_e37952_d_n6, assign33230_e37952_d_n7, assign33230_e37952_d_n8, assign33230_e37952_d_n9, assign33230_e37952_d_n10, assign33230_e37952_d_n11, assign33230_e37952_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign33230_e37946: f64 = (0.1 * locals.var_xmp);
        let assign33230_e37948: f64 = (assign33230_e37946 * locals.var_dnm);
        let assign33230_e37950: f64 = (assign33230_e37948 / locals.var_arg);
        (assign33230_e37950, ((((((0.1 * locals.var_xmp_dn0) * locals.var_dnm) + (assign33230_e37946 * locals.var_dnm_dn0)) * locals.var_arg) - (assign33230_e37948 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn2) * locals.var_dnm) + (assign33230_e37946 * locals.var_dnm_dn2)) * locals.var_arg) - (assign33230_e37948 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn4) * locals.var_dnm) + (assign33230_e37946 * locals.var_dnm_dn4)) * locals.var_arg) - (assign33230_e37948 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn5) * locals.var_dnm) + (assign33230_e37946 * locals.var_dnm_dn5)) * locals.var_arg) - (assign33230_e37948 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn6) * locals.var_dnm) + (assign33230_e37946 * locals.var_dnm_dn6)) * locals.var_arg) - (assign33230_e37948 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn7) * locals.var_dnm) + (assign33230_e37946 * locals.var_dnm_dn7)) * locals.var_arg) - (assign33230_e37948 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn8) * locals.var_dnm) + (assign33230_e37946 * locals.var_dnm_dn8)) * locals.var_arg) - (assign33230_e37948 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn9) * locals.var_dnm) + (assign33230_e37946 * locals.var_dnm_dn9)) * locals.var_arg) - (assign33230_e37948 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn10) * locals.var_dnm) + (assign33230_e37946 * locals.var_dnm_dn10)) * locals.var_arg) - (assign33230_e37948 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn11) * locals.var_dnm) + (assign33230_e37946 * locals.var_dnm_dn11)) * locals.var_arg) - (assign33230_e37948 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn14) * locals.var_dnm) + (assign33230_e37946 * locals.var_dnm_dn14)) * locals.var_arg) - (assign33230_e37948 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign33230_e37952;
        locals.var_t0_dn0 = assign33230_e37952_d_n0;
        locals.var_t0_dn2 = assign33230_e37952_d_n2;
        locals.var_t0_dn4 = assign33230_e37952_d_n4;
        locals.var_t0_dn5 = assign33230_e37952_d_n5;
        locals.var_t0_dn6 = assign33230_e37952_d_n6;
        locals.var_t0_dn7 = assign33230_e37952_d_n7;
        locals.var_t0_dn8 = assign33230_e37952_d_n8;
        locals.var_t0_dn9 = assign33230_e37952_d_n9;
        locals.var_t0_dn10 = assign33230_e37952_d_n10;
        locals.var_t0_dn11 = assign33230_e37952_d_n11;
        locals.var_t0_dn14 = assign33230_e37952_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign33240_e37970, assign33240_e37970_d_n0, assign33240_e37970_d_n2, assign33240_e37970_d_n4, assign33240_e37970_d_n5, assign33240_e37970_d_n6, assign33240_e37970_d_n7, assign33240_e37970_d_n8, assign33240_e37970_d_n9, assign33240_e37970_d_n10, assign33240_e37970_d_n11, assign33240_e37970_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign33240_e37966: f64 = 0.1;
        let assign33240_e37968: f64 = (assign33240_e37966 - locals.var_tmf0);
        (assign33240_e37968, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign33240_e37970;
        locals.var_t2_dn0 = assign33240_e37970_d_n0;
        locals.var_t2_dn2 = assign33240_e37970_d_n2;
        locals.var_t2_dn4 = assign33240_e37970_d_n4;
        locals.var_t2_dn5 = assign33240_e37970_d_n5;
        locals.var_t2_dn6 = assign33240_e37970_d_n6;
        locals.var_t2_dn7 = assign33240_e37970_d_n7;
        locals.var_t2_dn8 = assign33240_e37970_d_n8;
        locals.var_t2_dn9 = assign33240_e37970_d_n9;
        locals.var_t2_dn10 = assign33240_e37970_d_n10;
        locals.var_t2_dn11 = assign33240_e37970_d_n11;
        locals.var_t2_dn14 = assign33240_e37970_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign33250_e37984, assign33250_e37984_d_n0, assign33250_e37984_d_n2, assign33250_e37984_d_n4, assign33250_e37984_d_n5, assign33250_e37984_d_n6, assign33250_e37984_d_n7, assign33250_e37984_d_n8, assign33250_e37984_d_n9, assign33250_e37984_d_n10, assign33250_e37984_d_n11, assign33250_e37984_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard782 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign33250_e37984;
        locals.var_t0_dn0 = assign33250_e37984_d_n0;
        locals.var_t0_dn2 = assign33250_e37984_d_n2;
        locals.var_t0_dn4 = assign33250_e37984_d_n4;
        locals.var_t0_dn5 = assign33250_e37984_d_n5;
        locals.var_t0_dn6 = assign33250_e37984_d_n6;
        locals.var_t0_dn7 = assign33250_e37984_d_n7;
        locals.var_t0_dn8 = assign33250_e37984_d_n8;
        locals.var_t0_dn9 = assign33250_e37984_d_n9;
        locals.var_t0_dn10 = assign33250_e37984_d_n10;
        locals.var_t0_dn11 = assign33250_e37984_d_n11;
        locals.var_t0_dn14 = assign33250_e37984_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign33260_e37999, assign33260_e37999_d_n0, assign33260_e37999_d_n2, assign33260_e37999_d_n4, assign33260_e37999_d_n5, assign33260_e37999_d_n6, assign33260_e37999_d_n7, assign33260_e37999_d_n8, assign33260_e37999_d_n9, assign33260_e37999_d_n10, assign33260_e37999_d_n11, assign33260_e37999_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard782 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign33260_e37999;
        locals.var_t2_dn0 = assign33260_e37999_d_n0;
        locals.var_t2_dn2 = assign33260_e37999_d_n2;
        locals.var_t2_dn4 = assign33260_e37999_d_n4;
        locals.var_t2_dn5 = assign33260_e37999_d_n5;
        locals.var_t2_dn6 = assign33260_e37999_d_n6;
        locals.var_t2_dn7 = assign33260_e37999_d_n7;
        locals.var_t2_dn8 = assign33260_e37999_d_n8;
        locals.var_t2_dn9 = assign33260_e37999_d_n9;
        locals.var_t2_dn10 = assign33260_e37999_d_n10;
        locals.var_t2_dn11 = assign33260_e37999_d_n11;
        locals.var_t2_dn14 = assign33260_e37999_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign33270_e38014, assign33270_e38014_d_n0, assign33270_e38014_d_n2, assign33270_e38014_d_n4, assign33270_e38014_d_n5, assign33270_e38014_d_n6, assign33270_e38014_d_n7, assign33270_e38014_d_n8, assign33270_e38014_d_n9, assign33270_e38014_d_n10, assign33270_e38014_d_n11, assign33270_e38014_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard782 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign33270_e38014;
        locals.var_t0_dn0 = assign33270_e38014_d_n0;
        locals.var_t0_dn2 = assign33270_e38014_d_n2;
        locals.var_t0_dn4 = assign33270_e38014_d_n4;
        locals.var_t0_dn5 = assign33270_e38014_d_n5;
        locals.var_t0_dn6 = assign33270_e38014_d_n6;
        locals.var_t0_dn7 = assign33270_e38014_d_n7;
        locals.var_t0_dn8 = assign33270_e38014_d_n8;
        locals.var_t0_dn9 = assign33270_e38014_d_n9;
        locals.var_t0_dn10 = assign33270_e38014_d_n10;
        locals.var_t0_dn11 = assign33270_e38014_d_n11;
        locals.var_t0_dn14 = assign33270_e38014_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign33280_e38029, assign33280_e38029_d_n0, assign33280_e38029_d_n2, assign33280_e38029_d_n4, assign33280_e38029_d_n5, assign33280_e38029_d_n6, assign33280_e38029_d_n7, assign33280_e38029_d_n8, assign33280_e38029_d_n9, assign33280_e38029_d_n10, assign33280_e38029_d_n11, assign33280_e38029_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) {
        let assign33280_e38026: f64 = (locals.var_c_2esipq_ndepm * locals.var_t2);
        let assign33280_e38027: f64 = (assign33280_e38026).sqrt();
        (assign33280_e38027, (((locals.var_c_2esipq_ndepm_dn0 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn0)) / (2.0 * assign33280_e38027)), (((locals.var_c_2esipq_ndepm_dn2 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn2)) / (2.0 * assign33280_e38027)), (((locals.var_c_2esipq_ndepm_dn4 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn4)) / (2.0 * assign33280_e38027)), (((locals.var_c_2esipq_ndepm_dn5 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn5)) / (2.0 * assign33280_e38027)), (((locals.var_c_2esipq_ndepm_dn6 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn6)) / (2.0 * assign33280_e38027)), (((locals.var_c_2esipq_ndepm_dn7 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn7)) / (2.0 * assign33280_e38027)), (((locals.var_c_2esipq_ndepm_dn8 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn8)) / (2.0 * assign33280_e38027)), (((locals.var_c_2esipq_ndepm_dn9 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn9)) / (2.0 * assign33280_e38027)), (((locals.var_c_2esipq_ndepm_dn10 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn10)) / (2.0 * assign33280_e38027)), (((locals.var_c_2esipq_ndepm_dn11 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn11)) / (2.0 * assign33280_e38027)), (((locals.var_c_2esipq_ndepm_dn14 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn14)) / (2.0 * assign33280_e38027)),)
    } else {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn11, locals.var_w_bl_dn14,)
    }
};
        locals.var_w_bl = assign33280_e38029;
        locals.var_w_bl_dn0 = assign33280_e38029_d_n0;
        locals.var_w_bl_dn2 = assign33280_e38029_d_n2;
        locals.var_w_bl_dn4 = assign33280_e38029_d_n4;
        locals.var_w_bl_dn5 = assign33280_e38029_d_n5;
        locals.var_w_bl_dn6 = assign33280_e38029_d_n6;
        locals.var_w_bl_dn7 = assign33280_e38029_d_n7;
        locals.var_w_bl_dn8 = assign33280_e38029_d_n8;
        locals.var_w_bl_dn9 = assign33280_e38029_d_n9;
        locals.var_w_bl_dn10 = assign33280_e38029_d_n10;
        locals.var_w_bl_dn11 = assign33280_e38029_d_n11;
        locals.var_w_bl_dn14 = assign33280_e38029_d_n14;
        locals.var_w_bl_rv = 0.0;

        let assign33290_e38033: f64 = (locals.var_uc_depthn - 1e-8);
        let assign33290_e38038: f64 = if ((locals.var_w_bl > assign33290_e38033) && (1e-8 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard788 = assign33290_e38038;
        locals.var_guard788_rv = 0.0;

        let (assign33300_e38056, assign33300_e38056_d_n0, assign33300_e38056_d_n2, assign33300_e38056_d_n4, assign33300_e38056_d_n5, assign33300_e38056_d_n6, assign33300_e38056_d_n7, assign33300_e38056_d_n8, assign33300_e38056_d_n9, assign33300_e38056_d_n10, assign33300_e38056_d_n11, assign33300_e38056_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard788 != 0.0)) {
        let assign33300_e38052: f64 = (locals.var_w_bl - locals.var_uc_depthn);
        let assign33300_e38054: f64 = (assign33300_e38052 + 1e-8);
        (assign33300_e38054, (locals.var_w_bl_dn0 - locals.var_uc_depthn_dn0), (locals.var_w_bl_dn2 - locals.var_uc_depthn_dn2), (locals.var_w_bl_dn4 - locals.var_uc_depthn_dn4), (locals.var_w_bl_dn5 - locals.var_uc_depthn_dn5), (locals.var_w_bl_dn6 - locals.var_uc_depthn_dn6), (locals.var_w_bl_dn7 - locals.var_uc_depthn_dn7), (locals.var_w_bl_dn8 - locals.var_uc_depthn_dn8), (locals.var_w_bl_dn9 - locals.var_uc_depthn_dn9), (locals.var_w_bl_dn10 - locals.var_uc_depthn_dn10), (locals.var_w_bl_dn11 - locals.var_uc_depthn_dn11), (locals.var_w_bl_dn14 - locals.var_uc_depthn_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign33300_e38056;
        locals.var_tmf1_dn0 = assign33300_e38056_d_n0;
        locals.var_tmf1_dn2 = assign33300_e38056_d_n2;
        locals.var_tmf1_dn4 = assign33300_e38056_d_n4;
        locals.var_tmf1_dn5 = assign33300_e38056_d_n5;
        locals.var_tmf1_dn6 = assign33300_e38056_d_n6;
        locals.var_tmf1_dn7 = assign33300_e38056_d_n7;
        locals.var_tmf1_dn8 = assign33300_e38056_d_n8;
        locals.var_tmf1_dn9 = assign33300_e38056_d_n9;
        locals.var_tmf1_dn10 = assign33300_e38056_d_n10;
        locals.var_tmf1_dn11 = assign33300_e38056_d_n11;
        locals.var_tmf1_dn14 = assign33300_e38056_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign33310_e38072, assign33310_e38072_d_n0, assign33310_e38072_d_n2, assign33310_e38072_d_n4, assign33310_e38072_d_n5, assign33310_e38072_d_n6, assign33310_e38072_d_n7, assign33310_e38072_d_n8, assign33310_e38072_d_n9, assign33310_e38072_d_n10, assign33310_e38072_d_n11, assign33310_e38072_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard788 != 0.0)) {
        let assign33310_e38070: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign33310_e38070, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign33310_e38072;
        locals.var_x2_dn0 = assign33310_e38072_d_n0;
        locals.var_x2_dn2 = assign33310_e38072_d_n2;
        locals.var_x2_dn4 = assign33310_e38072_d_n4;
        locals.var_x2_dn5 = assign33310_e38072_d_n5;
        locals.var_x2_dn6 = assign33310_e38072_d_n6;
        locals.var_x2_dn7 = assign33310_e38072_d_n7;
        locals.var_x2_dn8 = assign33310_e38072_d_n8;
        locals.var_x2_dn9 = assign33310_e38072_d_n9;
        locals.var_x2_dn10 = assign33310_e38072_d_n10;
        locals.var_x2_dn11 = assign33310_e38072_d_n11;
        locals.var_x2_dn14 = assign33310_e38072_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign33320_e38088, assign33320_e38088_d_n0, assign33320_e38088_d_n2, assign33320_e38088_d_n4, assign33320_e38088_d_n5, assign33320_e38088_d_n6, assign33320_e38088_d_n7, assign33320_e38088_d_n8, assign33320_e38088_d_n9, assign33320_e38088_d_n10, assign33320_e38088_d_n11, assign33320_e38088_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard788 != 0.0)) {
        let assign33320_e38086: f64 = (1e-8 * 1e-8);
        (assign33320_e38086, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign33320_e38088;
        locals.var_xmax2_dn0 = assign33320_e38088_d_n0;
        locals.var_xmax2_dn2 = assign33320_e38088_d_n2;
        locals.var_xmax2_dn4 = assign33320_e38088_d_n4;
        locals.var_xmax2_dn5 = assign33320_e38088_d_n5;
        locals.var_xmax2_dn6 = assign33320_e38088_d_n6;
        locals.var_xmax2_dn7 = assign33320_e38088_d_n7;
        locals.var_xmax2_dn8 = assign33320_e38088_d_n8;
        locals.var_xmax2_dn9 = assign33320_e38088_d_n9;
        locals.var_xmax2_dn10 = assign33320_e38088_d_n10;
        locals.var_xmax2_dn11 = assign33320_e38088_d_n11;
        locals.var_xmax2_dn14 = assign33320_e38088_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign33330_e38102, assign33330_e38102_d_n0, assign33330_e38102_d_n2, assign33330_e38102_d_n4, assign33330_e38102_d_n5, assign33330_e38102_d_n6, assign33330_e38102_d_n7, assign33330_e38102_d_n8, assign33330_e38102_d_n9, assign33330_e38102_d_n10, assign33330_e38102_d_n11, assign33330_e38102_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard788 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign33330_e38102;
        locals.var_xp_dn0 = assign33330_e38102_d_n0;
        locals.var_xp_dn2 = assign33330_e38102_d_n2;
        locals.var_xp_dn4 = assign33330_e38102_d_n4;
        locals.var_xp_dn5 = assign33330_e38102_d_n5;
        locals.var_xp_dn6 = assign33330_e38102_d_n6;
        locals.var_xp_dn7 = assign33330_e38102_d_n7;
        locals.var_xp_dn8 = assign33330_e38102_d_n8;
        locals.var_xp_dn9 = assign33330_e38102_d_n9;
        locals.var_xp_dn10 = assign33330_e38102_d_n10;
        locals.var_xp_dn11 = assign33330_e38102_d_n11;
        locals.var_xp_dn14 = assign33330_e38102_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign33340_e38116, assign33340_e38116_d_n0, assign33340_e38116_d_n2, assign33340_e38116_d_n4, assign33340_e38116_d_n5, assign33340_e38116_d_n6, assign33340_e38116_d_n7, assign33340_e38116_d_n8, assign33340_e38116_d_n9, assign33340_e38116_d_n10, assign33340_e38116_d_n11, assign33340_e38116_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard788 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign33340_e38116;
        locals.var_xmp_dn0 = assign33340_e38116_d_n0;
        locals.var_xmp_dn2 = assign33340_e38116_d_n2;
        locals.var_xmp_dn4 = assign33340_e38116_d_n4;
        locals.var_xmp_dn5 = assign33340_e38116_d_n5;
        locals.var_xmp_dn6 = assign33340_e38116_d_n6;
        locals.var_xmp_dn7 = assign33340_e38116_d_n7;
        locals.var_xmp_dn8 = assign33340_e38116_d_n8;
        locals.var_xmp_dn9 = assign33340_e38116_d_n9;
        locals.var_xmp_dn10 = assign33340_e38116_d_n10;
        locals.var_xmp_dn11 = assign33340_e38116_d_n11;
        locals.var_xmp_dn14 = assign33340_e38116_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign33350_e38130,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard788 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign33350_e38130;
        locals.var_m0_rv = 0.0;

        let (assign33360_e38144,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard788 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33360_e38144;
        locals.var_mm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_108(
        locals: &mut StampLocals,
    ) {
        let (assign33370_e38158, assign33370_e38158_d_n0, assign33370_e38158_d_n2, assign33370_e38158_d_n4, assign33370_e38158_d_n5, assign33370_e38158_d_n6, assign33370_e38158_d_n7, assign33370_e38158_d_n8, assign33370_e38158_d_n9, assign33370_e38158_d_n10, assign33370_e38158_d_n11, assign33370_e38158_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard788 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign33370_e38158;
        locals.var_arg_dn0 = assign33370_e38158_d_n0;
        locals.var_arg_dn2 = assign33370_e38158_d_n2;
        locals.var_arg_dn4 = assign33370_e38158_d_n4;
        locals.var_arg_dn5 = assign33370_e38158_d_n5;
        locals.var_arg_dn6 = assign33370_e38158_d_n6;
        locals.var_arg_dn7 = assign33370_e38158_d_n7;
        locals.var_arg_dn8 = assign33370_e38158_d_n8;
        locals.var_arg_dn9 = assign33370_e38158_d_n9;
        locals.var_arg_dn10 = assign33370_e38158_d_n10;
        locals.var_arg_dn11 = assign33370_e38158_d_n11;
        locals.var_arg_dn14 = assign33370_e38158_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign33380_e38172, assign33380_e38172_d_n0, assign33380_e38172_d_n2, assign33380_e38172_d_n4, assign33380_e38172_d_n5, assign33380_e38172_d_n6, assign33380_e38172_d_n7, assign33380_e38172_d_n8, assign33380_e38172_d_n9, assign33380_e38172_d_n10, assign33380_e38172_d_n11, assign33380_e38172_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard788 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign33380_e38172;
        locals.var_dnm_dn0 = assign33380_e38172_d_n0;
        locals.var_dnm_dn2 = assign33380_e38172_d_n2;
        locals.var_dnm_dn4 = assign33380_e38172_d_n4;
        locals.var_dnm_dn5 = assign33380_e38172_d_n5;
        locals.var_dnm_dn6 = assign33380_e38172_d_n6;
        locals.var_dnm_dn7 = assign33380_e38172_d_n7;
        locals.var_dnm_dn8 = assign33380_e38172_d_n8;
        locals.var_dnm_dn9 = assign33380_e38172_d_n9;
        locals.var_dnm_dn10 = assign33380_e38172_d_n10;
        locals.var_dnm_dn11 = assign33380_e38172_d_n11;
        locals.var_dnm_dn14 = assign33380_e38172_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign33390_e38188, assign33390_e38188_d_n0, assign33390_e38188_d_n2, assign33390_e38188_d_n4, assign33390_e38188_d_n5, assign33390_e38188_d_n6, assign33390_e38188_d_n7, assign33390_e38188_d_n8, assign33390_e38188_d_n9, assign33390_e38188_d_n10, assign33390_e38188_d_n11, assign33390_e38188_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard788 != 0.0)) {
        let assign33390_e38186: f64 = (locals.var_xp * locals.var_x2);
        (assign33390_e38186, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign33390_e38188;
        locals.var_xp_dn0 = assign33390_e38188_d_n0;
        locals.var_xp_dn2 = assign33390_e38188_d_n2;
        locals.var_xp_dn4 = assign33390_e38188_d_n4;
        locals.var_xp_dn5 = assign33390_e38188_d_n5;
        locals.var_xp_dn6 = assign33390_e38188_d_n6;
        locals.var_xp_dn7 = assign33390_e38188_d_n7;
        locals.var_xp_dn8 = assign33390_e38188_d_n8;
        locals.var_xp_dn9 = assign33390_e38188_d_n9;
        locals.var_xp_dn10 = assign33390_e38188_d_n10;
        locals.var_xp_dn11 = assign33390_e38188_d_n11;
        locals.var_xp_dn14 = assign33390_e38188_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign33400_e38204, assign33400_e38204_d_n0, assign33400_e38204_d_n2, assign33400_e38204_d_n4, assign33400_e38204_d_n5, assign33400_e38204_d_n6, assign33400_e38204_d_n7, assign33400_e38204_d_n8, assign33400_e38204_d_n9, assign33400_e38204_d_n10, assign33400_e38204_d_n11, assign33400_e38204_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard788 != 0.0)) {
        let assign33400_e38202: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign33400_e38202, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign33400_e38204;
        locals.var_xmp_dn0 = assign33400_e38204_d_n0;
        locals.var_xmp_dn2 = assign33400_e38204_d_n2;
        locals.var_xmp_dn4 = assign33400_e38204_d_n4;
        locals.var_xmp_dn5 = assign33400_e38204_d_n5;
        locals.var_xmp_dn6 = assign33400_e38204_d_n6;
        locals.var_xmp_dn7 = assign33400_e38204_d_n7;
        locals.var_xmp_dn8 = assign33400_e38204_d_n8;
        locals.var_xmp_dn9 = assign33400_e38204_d_n9;
        locals.var_xmp_dn10 = assign33400_e38204_d_n10;
        locals.var_xmp_dn11 = assign33400_e38204_d_n11;
        locals.var_xmp_dn14 = assign33400_e38204_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign33410_e38220, assign33410_e38220_d_n0, assign33410_e38220_d_n2, assign33410_e38220_d_n4, assign33410_e38220_d_n5, assign33410_e38220_d_n6, assign33410_e38220_d_n7, assign33410_e38220_d_n8, assign33410_e38220_d_n9, assign33410_e38220_d_n10, assign33410_e38220_d_n11, assign33410_e38220_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard788 != 0.0)) {
        let assign33410_e38218: f64 = (locals.var_xp * locals.var_x2);
        (assign33410_e38218, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign33410_e38220;
        locals.var_xp_dn0 = assign33410_e38220_d_n0;
        locals.var_xp_dn2 = assign33410_e38220_d_n2;
        locals.var_xp_dn4 = assign33410_e38220_d_n4;
        locals.var_xp_dn5 = assign33410_e38220_d_n5;
        locals.var_xp_dn6 = assign33410_e38220_d_n6;
        locals.var_xp_dn7 = assign33410_e38220_d_n7;
        locals.var_xp_dn8 = assign33410_e38220_d_n8;
        locals.var_xp_dn9 = assign33410_e38220_d_n9;
        locals.var_xp_dn10 = assign33410_e38220_d_n10;
        locals.var_xp_dn11 = assign33410_e38220_d_n11;
        locals.var_xp_dn14 = assign33410_e38220_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign33420_e38236, assign33420_e38236_d_n0, assign33420_e38236_d_n2, assign33420_e38236_d_n4, assign33420_e38236_d_n5, assign33420_e38236_d_n6, assign33420_e38236_d_n7, assign33420_e38236_d_n8, assign33420_e38236_d_n9, assign33420_e38236_d_n10, assign33420_e38236_d_n11, assign33420_e38236_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard788 != 0.0)) {
        let assign33420_e38234: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign33420_e38234, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign33420_e38236;
        locals.var_xmp_dn0 = assign33420_e38236_d_n0;
        locals.var_xmp_dn2 = assign33420_e38236_d_n2;
        locals.var_xmp_dn4 = assign33420_e38236_d_n4;
        locals.var_xmp_dn5 = assign33420_e38236_d_n5;
        locals.var_xmp_dn6 = assign33420_e38236_d_n6;
        locals.var_xmp_dn7 = assign33420_e38236_d_n7;
        locals.var_xmp_dn8 = assign33420_e38236_d_n8;
        locals.var_xmp_dn9 = assign33420_e38236_d_n9;
        locals.var_xmp_dn10 = assign33420_e38236_d_n10;
        locals.var_xmp_dn11 = assign33420_e38236_d_n11;
        locals.var_xmp_dn14 = assign33420_e38236_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign33430_e38252, assign33430_e38252_d_n0, assign33430_e38252_d_n2, assign33430_e38252_d_n4, assign33430_e38252_d_n5, assign33430_e38252_d_n6, assign33430_e38252_d_n7, assign33430_e38252_d_n8, assign33430_e38252_d_n9, assign33430_e38252_d_n10, assign33430_e38252_d_n11, assign33430_e38252_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard788 != 0.0)) {
        let assign33430_e38250: f64 = (locals.var_xp + locals.var_xmp);
        (assign33430_e38250, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign33430_e38252;
        locals.var_arg_dn0 = assign33430_e38252_d_n0;
        locals.var_arg_dn2 = assign33430_e38252_d_n2;
        locals.var_arg_dn4 = assign33430_e38252_d_n4;
        locals.var_arg_dn5 = assign33430_e38252_d_n5;
        locals.var_arg_dn6 = assign33430_e38252_d_n6;
        locals.var_arg_dn7 = assign33430_e38252_d_n7;
        locals.var_arg_dn8 = assign33430_e38252_d_n8;
        locals.var_arg_dn9 = assign33430_e38252_d_n9;
        locals.var_arg_dn10 = assign33430_e38252_d_n10;
        locals.var_arg_dn11 = assign33430_e38252_d_n11;
        locals.var_arg_dn14 = assign33430_e38252_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign33440_e38266, assign33440_e38266_d_n0, assign33440_e38266_d_n2, assign33440_e38266_d_n4, assign33440_e38266_d_n5, assign33440_e38266_d_n6, assign33440_e38266_d_n7, assign33440_e38266_d_n8, assign33440_e38266_d_n9, assign33440_e38266_d_n10, assign33440_e38266_d_n11, assign33440_e38266_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard788 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign33440_e38266;
        locals.var_dnm_dn0 = assign33440_e38266_d_n0;
        locals.var_dnm_dn2 = assign33440_e38266_d_n2;
        locals.var_dnm_dn4 = assign33440_e38266_d_n4;
        locals.var_dnm_dn5 = assign33440_e38266_d_n5;
        locals.var_dnm_dn6 = assign33440_e38266_d_n6;
        locals.var_dnm_dn7 = assign33440_e38266_d_n7;
        locals.var_dnm_dn8 = assign33440_e38266_d_n8;
        locals.var_dnm_dn9 = assign33440_e38266_d_n9;
        locals.var_dnm_dn10 = assign33440_e38266_d_n10;
        locals.var_dnm_dn11 = assign33440_e38266_d_n11;
        locals.var_dnm_dn14 = assign33440_e38266_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign33450_e38281: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard789 = assign33450_e38281;
        locals.var_guard789_rv = 0.0;

        let assign33460_e38284: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard790 = assign33460_e38284;
        locals.var_guard790_rv = 0.0;

        let (assign33470_e38302,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard788 != 0.0)) && (locals.var_guard789 != 0.0)) && (locals.var_guard790 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33470_e38302;
        locals.var_mm_rv = 0.0;

        let assign33480_e38305: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard791 = assign33480_e38305;
        locals.var_guard791_rv = 0.0;

        let (assign33490_e38326,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard788 != 0.0)) && (locals.var_guard789 != 0.0)) && (locals.var_guard790 == 0.0)) && (locals.var_guard791 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33490_e38326;
        locals.var_mm_rv = 0.0;

        let assign33500_e38329: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard792 = assign33500_e38329;
        locals.var_guard792_rv = 0.0;

        let (assign33510_e38353,) = {
    if (((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard788 != 0.0)) && (locals.var_guard789 != 0.0)) && (locals.var_guard790 == 0.0)) && (locals.var_guard791 == 0.0)) && (locals.var_guard792 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33510_e38353;
        locals.var_mm_rv = 0.0;

        let assign33520_e38356: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard793 = assign33520_e38356;
        locals.var_guard793_rv = 0.0;

        let (assign33530_e38383,) = {
    if ((((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard788 != 0.0)) && (locals.var_guard789 != 0.0)) && (locals.var_guard790 == 0.0)) && (locals.var_guard791 == 0.0)) && (locals.var_guard792 == 0.0)) && (locals.var_guard793 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33530_e38383;
        locals.var_mm_rv = 0.0;

        let (assign33540_e38399,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard788 != 0.0)) && (locals.var_guard789 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign33540_e38399;
        locals.var_m0_rv = 0.0;

        let mut assign33550_loop_guard: usize = 0;
        while {
            let assign33550_cond_e38416: f64 = if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard788 != 0.0)) && (locals.var_guard789 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign33550_cond_e38416 != 0.0
        } {
            assign33550_loop_guard += 1;
            assert!(assign33550_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign33550_body0_e38433, assign33550_body0_e38433_d_n0, assign33550_body0_e38433_d_n2, assign33550_body0_e38433_d_n4, assign33550_body0_e38433_d_n5, assign33550_body0_e38433_d_n6, assign33550_body0_e38433_d_n7, assign33550_body0_e38433_d_n8, assign33550_body0_e38433_d_n9, assign33550_body0_e38433_d_n10, assign33550_body0_e38433_d_n11, assign33550_body0_e38433_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard788 != 0.0)) && (locals.var_guard789 != 0.0)) {
        let assign33550_body0_e38431: f64 = (locals.var_dnm).sqrt();
        (assign33550_body0_e38431, (locals.var_dnm_dn0 / (2.0 * assign33550_body0_e38431)), (locals.var_dnm_dn2 / (2.0 * assign33550_body0_e38431)), (locals.var_dnm_dn4 / (2.0 * assign33550_body0_e38431)), (locals.var_dnm_dn5 / (2.0 * assign33550_body0_e38431)), (locals.var_dnm_dn6 / (2.0 * assign33550_body0_e38431)), (locals.var_dnm_dn7 / (2.0 * assign33550_body0_e38431)), (locals.var_dnm_dn8 / (2.0 * assign33550_body0_e38431)), (locals.var_dnm_dn9 / (2.0 * assign33550_body0_e38431)), (locals.var_dnm_dn10 / (2.0 * assign33550_body0_e38431)), (locals.var_dnm_dn11 / (2.0 * assign33550_body0_e38431)), (locals.var_dnm_dn14 / (2.0 * assign33550_body0_e38431)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign33550_body0_e38433;
            locals.var_dnm_dn0 = assign33550_body0_e38433_d_n0;
            locals.var_dnm_dn2 = assign33550_body0_e38433_d_n2;
            locals.var_dnm_dn4 = assign33550_body0_e38433_d_n4;
            locals.var_dnm_dn5 = assign33550_body0_e38433_d_n5;
            locals.var_dnm_dn6 = assign33550_body0_e38433_d_n6;
            locals.var_dnm_dn7 = assign33550_body0_e38433_d_n7;
            locals.var_dnm_dn8 = assign33550_body0_e38433_d_n8;
            locals.var_dnm_dn9 = assign33550_body0_e38433_d_n9;
            locals.var_dnm_dn10 = assign33550_body0_e38433_d_n10;
            locals.var_dnm_dn11 = assign33550_body0_e38433_d_n11;
            locals.var_dnm_dn14 = assign33550_body0_e38433_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign33550_body1_e38451,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard788 != 0.0)) && (locals.var_guard789 != 0.0)) {
        let assign33550_body1_e38449: f64 = (locals.var_m0 + 1.0);
        (assign33550_body1_e38449,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign33550_body1_e38451;
            locals.var_m0_rv = 0.0;
        }

        let (assign33560_e38479, assign33560_e38479_d_n0, assign33560_e38479_d_n2, assign33560_e38479_d_n4, assign33560_e38479_d_n5, assign33560_e38479_d_n6, assign33560_e38479_d_n7, assign33560_e38479_d_n8, assign33560_e38479_d_n9, assign33560_e38479_d_n10, assign33560_e38479_d_n11, assign33560_e38479_d_n14,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard788 != 0.0)) && (locals.var_guard789 == 0.0)) {
        let (assign33560_e38477, assign33560_e38477_d_n0, assign33560_e38477_d_n2, assign33560_e38477_d_n4, assign33560_e38477_d_n5, assign33560_e38477_d_n6, assign33560_e38477_d_n7, assign33560_e38477_d_n8, assign33560_e38477_d_n9, assign33560_e38477_d_n10, assign33560_e38477_d_n11, assign33560_e38477_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign33560_e38474: f64 = (2.0 * 2.0);
                let assign33560_e38475: f64 = (1.0 / assign33560_e38474);
                let assign33560_e38476: f64 = (locals.var_dnm).powf(assign33560_e38475);
                (assign33560_e38476, if 0.0 == 0.0 && ((assign33560_e38475) as f64).is_finite() && ((assign33560_e38475) as f64).fract() == 0.0 { if assign33560_e38475 == 0.0 { 0.0 } else { (assign33560_e38475 * ((locals.var_dnm).powf(assign33560_e38475 - 1.0) * locals.var_dnm_dn0)) } } else { (assign33560_e38476 * (assign33560_e38475 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33560_e38475) as f64).is_finite() && ((assign33560_e38475) as f64).fract() == 0.0 { if assign33560_e38475 == 0.0 { 0.0 } else { (assign33560_e38475 * ((locals.var_dnm).powf(assign33560_e38475 - 1.0) * locals.var_dnm_dn2)) } } else { (assign33560_e38476 * (assign33560_e38475 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33560_e38475) as f64).is_finite() && ((assign33560_e38475) as f64).fract() == 0.0 { if assign33560_e38475 == 0.0 { 0.0 } else { (assign33560_e38475 * ((locals.var_dnm).powf(assign33560_e38475 - 1.0) * locals.var_dnm_dn4)) } } else { (assign33560_e38476 * (assign33560_e38475 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33560_e38475) as f64).is_finite() && ((assign33560_e38475) as f64).fract() == 0.0 { if assign33560_e38475 == 0.0 { 0.0 } else { (assign33560_e38475 * ((locals.var_dnm).powf(assign33560_e38475 - 1.0) * locals.var_dnm_dn5)) } } else { (assign33560_e38476 * (assign33560_e38475 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33560_e38475) as f64).is_finite() && ((assign33560_e38475) as f64).fract() == 0.0 { if assign33560_e38475 == 0.0 { 0.0 } else { (assign33560_e38475 * ((locals.var_dnm).powf(assign33560_e38475 - 1.0) * locals.var_dnm_dn6)) } } else { (assign33560_e38476 * (assign33560_e38475 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33560_e38475) as f64).is_finite() && ((assign33560_e38475) as f64).fract() == 0.0 { if assign33560_e38475 == 0.0 { 0.0 } else { (assign33560_e38475 * ((locals.var_dnm).powf(assign33560_e38475 - 1.0) * locals.var_dnm_dn7)) } } else { (assign33560_e38476 * (assign33560_e38475 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33560_e38475) as f64).is_finite() && ((assign33560_e38475) as f64).fract() == 0.0 { if assign33560_e38475 == 0.0 { 0.0 } else { (assign33560_e38475 * ((locals.var_dnm).powf(assign33560_e38475 - 1.0) * locals.var_dnm_dn8)) } } else { (assign33560_e38476 * (assign33560_e38475 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33560_e38475) as f64).is_finite() && ((assign33560_e38475) as f64).fract() == 0.0 { if assign33560_e38475 == 0.0 { 0.0 } else { (assign33560_e38475 * ((locals.var_dnm).powf(assign33560_e38475 - 1.0) * locals.var_dnm_dn9)) } } else { (assign33560_e38476 * (assign33560_e38475 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33560_e38475) as f64).is_finite() && ((assign33560_e38475) as f64).fract() == 0.0 { if assign33560_e38475 == 0.0 { 0.0 } else { (assign33560_e38475 * ((locals.var_dnm).powf(assign33560_e38475 - 1.0) * locals.var_dnm_dn10)) } } else { (assign33560_e38476 * (assign33560_e38475 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33560_e38475) as f64).is_finite() && ((assign33560_e38475) as f64).fract() == 0.0 { if assign33560_e38475 == 0.0 { 0.0 } else { (assign33560_e38475 * ((locals.var_dnm).powf(assign33560_e38475 - 1.0) * locals.var_dnm_dn11)) } } else { (assign33560_e38476 * (assign33560_e38475 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33560_e38475) as f64).is_finite() && ((assign33560_e38475) as f64).fract() == 0.0 { if assign33560_e38475 == 0.0 { 0.0 } else { (assign33560_e38475 * ((locals.var_dnm).powf(assign33560_e38475 - 1.0) * locals.var_dnm_dn14)) } } else { (assign33560_e38476 * (assign33560_e38475 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign33560_e38477, assign33560_e38477_d_n0, assign33560_e38477_d_n2, assign33560_e38477_d_n4, assign33560_e38477_d_n5, assign33560_e38477_d_n6, assign33560_e38477_d_n7, assign33560_e38477_d_n8, assign33560_e38477_d_n9, assign33560_e38477_d_n10, assign33560_e38477_d_n11, assign33560_e38477_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign33560_e38479;
        locals.var_dnm_dn0 = assign33560_e38479_d_n0;
        locals.var_dnm_dn2 = assign33560_e38479_d_n2;
        locals.var_dnm_dn4 = assign33560_e38479_d_n4;
        locals.var_dnm_dn5 = assign33560_e38479_d_n5;
        locals.var_dnm_dn6 = assign33560_e38479_d_n6;
        locals.var_dnm_dn7 = assign33560_e38479_d_n7;
        locals.var_dnm_dn8 = assign33560_e38479_d_n8;
        locals.var_dnm_dn9 = assign33560_e38479_d_n9;
        locals.var_dnm_dn10 = assign33560_e38479_d_n10;
        locals.var_dnm_dn11 = assign33560_e38479_d_n11;
        locals.var_dnm_dn14 = assign33560_e38479_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign33570_e38495, assign33570_e38495_d_n0, assign33570_e38495_d_n2, assign33570_e38495_d_n4, assign33570_e38495_d_n5, assign33570_e38495_d_n6, assign33570_e38495_d_n7, assign33570_e38495_d_n8, assign33570_e38495_d_n9, assign33570_e38495_d_n10, assign33570_e38495_d_n11, assign33570_e38495_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard788 != 0.0)) {
        let assign33570_e38493: f64 = (1.0 / locals.var_dnm);
        (assign33570_e38493, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign33570_e38495;
        locals.var_dnm_dn0 = assign33570_e38495_d_n0;
        locals.var_dnm_dn2 = assign33570_e38495_d_n2;
        locals.var_dnm_dn4 = assign33570_e38495_d_n4;
        locals.var_dnm_dn5 = assign33570_e38495_d_n5;
        locals.var_dnm_dn6 = assign33570_e38495_d_n6;
        locals.var_dnm_dn7 = assign33570_e38495_d_n7;
        locals.var_dnm_dn8 = assign33570_e38495_d_n8;
        locals.var_dnm_dn9 = assign33570_e38495_d_n9;
        locals.var_dnm_dn10 = assign33570_e38495_d_n10;
        locals.var_dnm_dn11 = assign33570_e38495_d_n11;
        locals.var_dnm_dn14 = assign33570_e38495_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign33580_e38513, assign33580_e38513_d_n0, assign33580_e38513_d_n2, assign33580_e38513_d_n4, assign33580_e38513_d_n5, assign33580_e38513_d_n6, assign33580_e38513_d_n7, assign33580_e38513_d_n8, assign33580_e38513_d_n9, assign33580_e38513_d_n10, assign33580_e38513_d_n11, assign33580_e38513_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard788 != 0.0)) {
        let assign33580_e38509: f64 = (locals.var_tmf1 * 1e-8);
        let assign33580_e38511: f64 = (assign33580_e38509 * locals.var_dnm);
        (assign33580_e38511, (((locals.var_tmf1_dn0 * 1e-8) * locals.var_dnm) + (assign33580_e38509 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-8) * locals.var_dnm) + (assign33580_e38509 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-8) * locals.var_dnm) + (assign33580_e38509 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-8) * locals.var_dnm) + (assign33580_e38509 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-8) * locals.var_dnm) + (assign33580_e38509 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-8) * locals.var_dnm) + (assign33580_e38509 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-8) * locals.var_dnm) + (assign33580_e38509 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-8) * locals.var_dnm) + (assign33580_e38509 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-8) * locals.var_dnm) + (assign33580_e38509 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1e-8) * locals.var_dnm) + (assign33580_e38509 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 1e-8) * locals.var_dnm) + (assign33580_e38509 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign33580_e38513;
        locals.var_tmf0_dn0 = assign33580_e38513_d_n0;
        locals.var_tmf0_dn2 = assign33580_e38513_d_n2;
        locals.var_tmf0_dn4 = assign33580_e38513_d_n4;
        locals.var_tmf0_dn5 = assign33580_e38513_d_n5;
        locals.var_tmf0_dn6 = assign33580_e38513_d_n6;
        locals.var_tmf0_dn7 = assign33580_e38513_d_n7;
        locals.var_tmf0_dn8 = assign33580_e38513_d_n8;
        locals.var_tmf0_dn9 = assign33580_e38513_d_n9;
        locals.var_tmf0_dn10 = assign33580_e38513_d_n10;
        locals.var_tmf0_dn11 = assign33580_e38513_d_n11;
        locals.var_tmf0_dn14 = assign33580_e38513_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign33590_e38533, assign33590_e38533_d_n0, assign33590_e38533_d_n2, assign33590_e38533_d_n4, assign33590_e38533_d_n5, assign33590_e38533_d_n6, assign33590_e38533_d_n7, assign33590_e38533_d_n8, assign33590_e38533_d_n9, assign33590_e38533_d_n10, assign33590_e38533_d_n11, assign33590_e38533_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard788 != 0.0)) {
        let assign33590_e38527: f64 = (1e-8 * locals.var_xmp);
        let assign33590_e38529: f64 = (assign33590_e38527 * locals.var_dnm);
        let assign33590_e38531: f64 = (assign33590_e38529 / locals.var_arg);
        (assign33590_e38531, ((((((1e-8 * locals.var_xmp_dn0) * locals.var_dnm) + (assign33590_e38527 * locals.var_dnm_dn0)) * locals.var_arg) - (assign33590_e38529 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn2) * locals.var_dnm) + (assign33590_e38527 * locals.var_dnm_dn2)) * locals.var_arg) - (assign33590_e38529 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn4) * locals.var_dnm) + (assign33590_e38527 * locals.var_dnm_dn4)) * locals.var_arg) - (assign33590_e38529 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn5) * locals.var_dnm) + (assign33590_e38527 * locals.var_dnm_dn5)) * locals.var_arg) - (assign33590_e38529 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn6) * locals.var_dnm) + (assign33590_e38527 * locals.var_dnm_dn6)) * locals.var_arg) - (assign33590_e38529 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn7) * locals.var_dnm) + (assign33590_e38527 * locals.var_dnm_dn7)) * locals.var_arg) - (assign33590_e38529 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn8) * locals.var_dnm) + (assign33590_e38527 * locals.var_dnm_dn8)) * locals.var_arg) - (assign33590_e38529 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn9) * locals.var_dnm) + (assign33590_e38527 * locals.var_dnm_dn9)) * locals.var_arg) - (assign33590_e38529 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn10) * locals.var_dnm) + (assign33590_e38527 * locals.var_dnm_dn10)) * locals.var_arg) - (assign33590_e38529 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn11) * locals.var_dnm) + (assign33590_e38527 * locals.var_dnm_dn11)) * locals.var_arg) - (assign33590_e38529 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn14) * locals.var_dnm) + (assign33590_e38527 * locals.var_dnm_dn14)) * locals.var_arg) - (assign33590_e38529 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign33590_e38533;
        locals.var_t3_dn0 = assign33590_e38533_d_n0;
        locals.var_t3_dn2 = assign33590_e38533_d_n2;
        locals.var_t3_dn4 = assign33590_e38533_d_n4;
        locals.var_t3_dn5 = assign33590_e38533_d_n5;
        locals.var_t3_dn6 = assign33590_e38533_d_n6;
        locals.var_t3_dn7 = assign33590_e38533_d_n7;
        locals.var_t3_dn8 = assign33590_e38533_d_n8;
        locals.var_t3_dn9 = assign33590_e38533_d_n9;
        locals.var_t3_dn10 = assign33590_e38533_d_n10;
        locals.var_t3_dn11 = assign33590_e38533_d_n11;
        locals.var_t3_dn14 = assign33590_e38533_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign33600_e38551, assign33600_e38551_d_n0, assign33600_e38551_d_n2, assign33600_e38551_d_n4, assign33600_e38551_d_n5, assign33600_e38551_d_n6, assign33600_e38551_d_n7, assign33600_e38551_d_n8, assign33600_e38551_d_n9, assign33600_e38551_d_n10, assign33600_e38551_d_n11, assign33600_e38551_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard788 != 0.0)) {
        let assign33600_e38547: f64 = (locals.var_uc_depthn - 1e-8);
        let assign33600_e38549: f64 = (assign33600_e38547 + locals.var_tmf0);
        (assign33600_e38549, (locals.var_uc_depthn_dn0 + locals.var_tmf0_dn0), (locals.var_uc_depthn_dn2 + locals.var_tmf0_dn2), (locals.var_uc_depthn_dn4 + locals.var_tmf0_dn4), (locals.var_uc_depthn_dn5 + locals.var_tmf0_dn5), (locals.var_uc_depthn_dn6 + locals.var_tmf0_dn6), (locals.var_uc_depthn_dn7 + locals.var_tmf0_dn7), (locals.var_uc_depthn_dn8 + locals.var_tmf0_dn8), (locals.var_uc_depthn_dn9 + locals.var_tmf0_dn9), (locals.var_uc_depthn_dn10 + locals.var_tmf0_dn10), (locals.var_uc_depthn_dn11 + locals.var_tmf0_dn11), (locals.var_uc_depthn_dn14 + locals.var_tmf0_dn14),)
    } else {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn11, locals.var_w_bl_dn14,)
    }
};
        locals.var_w_bl = assign33600_e38551;
        locals.var_w_bl_dn0 = assign33600_e38551_d_n0;
        locals.var_w_bl_dn2 = assign33600_e38551_d_n2;
        locals.var_w_bl_dn4 = assign33600_e38551_d_n4;
        locals.var_w_bl_dn5 = assign33600_e38551_d_n5;
        locals.var_w_bl_dn6 = assign33600_e38551_d_n6;
        locals.var_w_bl_dn7 = assign33600_e38551_d_n7;
        locals.var_w_bl_dn8 = assign33600_e38551_d_n8;
        locals.var_w_bl_dn9 = assign33600_e38551_d_n9;
        locals.var_w_bl_dn10 = assign33600_e38551_d_n10;
        locals.var_w_bl_dn11 = assign33600_e38551_d_n11;
        locals.var_w_bl_dn14 = assign33600_e38551_d_n14;
        locals.var_w_bl_rv = 0.0;

        let (assign33610_e38565, assign33610_e38565_d_n0, assign33610_e38565_d_n2, assign33610_e38565_d_n4, assign33610_e38565_d_n5, assign33610_e38565_d_n6, assign33610_e38565_d_n7, assign33610_e38565_d_n8, assign33610_e38565_d_n9, assign33610_e38565_d_n10, assign33610_e38565_d_n11, assign33610_e38565_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard788 != 0.0)) {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign33610_e38565;
        locals.var_t3_dn0 = assign33610_e38565_d_n0;
        locals.var_t3_dn2 = assign33610_e38565_d_n2;
        locals.var_t3_dn4 = assign33610_e38565_d_n4;
        locals.var_t3_dn5 = assign33610_e38565_d_n5;
        locals.var_t3_dn6 = assign33610_e38565_d_n6;
        locals.var_t3_dn7 = assign33610_e38565_d_n7;
        locals.var_t3_dn8 = assign33610_e38565_d_n8;
        locals.var_t3_dn9 = assign33610_e38565_d_n9;
        locals.var_t3_dn10 = assign33610_e38565_d_n10;
        locals.var_t3_dn11 = assign33610_e38565_d_n11;
        locals.var_t3_dn14 = assign33610_e38565_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign33620_e38580, assign33620_e38580_d_n0, assign33620_e38580_d_n2, assign33620_e38580_d_n4, assign33620_e38580_d_n5, assign33620_e38580_d_n6, assign33620_e38580_d_n7, assign33620_e38580_d_n8, assign33620_e38580_d_n9, assign33620_e38580_d_n10, assign33620_e38580_d_n11, assign33620_e38580_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard788 == 0.0)) {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn11, locals.var_w_bl_dn14,)
    } else {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn11, locals.var_w_bl_dn14,)
    }
};
        locals.var_w_bl = assign33620_e38580;
        locals.var_w_bl_dn0 = assign33620_e38580_d_n0;
        locals.var_w_bl_dn2 = assign33620_e38580_d_n2;
        locals.var_w_bl_dn4 = assign33620_e38580_d_n4;
        locals.var_w_bl_dn5 = assign33620_e38580_d_n5;
        locals.var_w_bl_dn6 = assign33620_e38580_d_n6;
        locals.var_w_bl_dn7 = assign33620_e38580_d_n7;
        locals.var_w_bl_dn8 = assign33620_e38580_d_n8;
        locals.var_w_bl_dn9 = assign33620_e38580_d_n9;
        locals.var_w_bl_dn10 = assign33620_e38580_d_n10;
        locals.var_w_bl_dn11 = assign33620_e38580_d_n11;
        locals.var_w_bl_dn14 = assign33620_e38580_d_n14;
        locals.var_w_bl_rv = 0.0;

        let (assign33630_e38595, assign33630_e38595_d_n0, assign33630_e38595_d_n2, assign33630_e38595_d_n4, assign33630_e38595_d_n5, assign33630_e38595_d_n6, assign33630_e38595_d_n7, assign33630_e38595_d_n8, assign33630_e38595_d_n9, assign33630_e38595_d_n10, assign33630_e38595_d_n11, assign33630_e38595_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) && (locals.var_guard788 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign33630_e38595;
        locals.var_t3_dn0 = assign33630_e38595_d_n0;
        locals.var_t3_dn2 = assign33630_e38595_d_n2;
        locals.var_t3_dn4 = assign33630_e38595_d_n4;
        locals.var_t3_dn5 = assign33630_e38595_d_n5;
        locals.var_t3_dn6 = assign33630_e38595_d_n6;
        locals.var_t3_dn7 = assign33630_e38595_d_n7;
        locals.var_t3_dn8 = assign33630_e38595_d_n8;
        locals.var_t3_dn9 = assign33630_e38595_d_n9;
        locals.var_t3_dn10 = assign33630_e38595_d_n10;
        locals.var_t3_dn11 = assign33630_e38595_d_n11;
        locals.var_t3_dn14 = assign33630_e38595_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign33640_e38614, assign33640_e38614_d_n0, assign33640_e38614_d_n2, assign33640_e38614_d_n4, assign33640_e38614_d_n5, assign33640_e38614_d_n6, assign33640_e38614_d_n7, assign33640_e38614_d_n8, assign33640_e38614_d_n9, assign33640_e38614_d_n10, assign33640_e38614_d_n11, assign33640_e38614_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) {
        let assign33640_e38608: f64 = (locals.var_phi_jl_dep - locals.var_vbscl__blk437);
        let assign33640_e38610: f64 = (assign33640_e38608 + locals.var_vbi_dep);
        let assign33640_e38611: f64 = (locals.var_c_2esipq_nsub * assign33640_e38610);
        let assign33640_e38612: f64 = (assign33640_e38611).sqrt();
        (assign33640_e38612, (((locals.var_c_2esipq_nsub_dn0 * assign33640_e38610) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn0 - locals.var_vbscl__blk437_dn0) + locals.var_vbi_dep_dn0))) / (2.0 * assign33640_e38612)), (((locals.var_c_2esipq_nsub_dn2 * assign33640_e38610) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn2 - locals.var_vbscl__blk437_dn2) + locals.var_vbi_dep_dn2))) / (2.0 * assign33640_e38612)), (((locals.var_c_2esipq_nsub_dn4 * assign33640_e38610) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn4 - locals.var_vbscl__blk437_dn4) + locals.var_vbi_dep_dn4))) / (2.0 * assign33640_e38612)), (((locals.var_c_2esipq_nsub_dn5 * assign33640_e38610) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn5 - locals.var_vbscl__blk437_dn5) + locals.var_vbi_dep_dn5))) / (2.0 * assign33640_e38612)), (((locals.var_c_2esipq_nsub_dn6 * assign33640_e38610) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn6 - locals.var_vbscl__blk437_dn6) + locals.var_vbi_dep_dn6))) / (2.0 * assign33640_e38612)), (((locals.var_c_2esipq_nsub_dn7 * assign33640_e38610) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn7 - locals.var_vbscl__blk437_dn7) + locals.var_vbi_dep_dn7))) / (2.0 * assign33640_e38612)), (((locals.var_c_2esipq_nsub_dn8 * assign33640_e38610) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn8 - locals.var_vbscl__blk437_dn8) + locals.var_vbi_dep_dn8))) / (2.0 * assign33640_e38612)), (((locals.var_c_2esipq_nsub_dn9 * assign33640_e38610) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn9 - locals.var_vbscl__blk437_dn9) + locals.var_vbi_dep_dn9))) / (2.0 * assign33640_e38612)), (((locals.var_c_2esipq_nsub_dn10 * assign33640_e38610) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn10 - locals.var_vbscl__blk437_dn10) + locals.var_vbi_dep_dn10))) / (2.0 * assign33640_e38612)), (((locals.var_c_2esipq_nsub_dn11 * assign33640_e38610) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn11 - locals.var_vbscl__blk437_dn11) + locals.var_vbi_dep_dn11))) / (2.0 * assign33640_e38612)), (((locals.var_c_2esipq_nsub_dn14 * assign33640_e38610) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn14 - locals.var_vbscl__blk437_dn14) + locals.var_vbi_dep_dn14))) / (2.0 * assign33640_e38612)),)
    } else {
        (locals.var_w_subl, locals.var_w_subl_dn0, locals.var_w_subl_dn2, locals.var_w_subl_dn4, locals.var_w_subl_dn5, locals.var_w_subl_dn6, locals.var_w_subl_dn7, locals.var_w_subl_dn8, locals.var_w_subl_dn9, locals.var_w_subl_dn10, locals.var_w_subl_dn11, locals.var_w_subl_dn14,)
    }
};
        locals.var_w_subl = assign33640_e38614;
        locals.var_w_subl_dn0 = assign33640_e38614_d_n0;
        locals.var_w_subl_dn2 = assign33640_e38614_d_n2;
        locals.var_w_subl_dn4 = assign33640_e38614_d_n4;
        locals.var_w_subl_dn5 = assign33640_e38614_d_n5;
        locals.var_w_subl_dn6 = assign33640_e38614_d_n6;
        locals.var_w_subl_dn7 = assign33640_e38614_d_n7;
        locals.var_w_subl_dn8 = assign33640_e38614_d_n8;
        locals.var_w_subl_dn9 = assign33640_e38614_d_n9;
        locals.var_w_subl_dn10 = assign33640_e38614_d_n10;
        locals.var_w_subl_dn11 = assign33640_e38614_d_n11;
        locals.var_w_subl_dn14 = assign33640_e38614_d_n14;
        locals.var_w_subl_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_109(
        locals: &mut StampLocals,
    ) {
        let (assign33650_e38628, assign33650_e38628_d_n0, assign33650_e38628_d_n2, assign33650_e38628_d_n4, assign33650_e38628_d_n5, assign33650_e38628_d_n6, assign33650_e38628_d_n7, assign33650_e38628_d_n8, assign33650_e38628_d_n9, assign33650_e38628_d_n10, assign33650_e38628_d_n11, assign33650_e38628_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) {
        let assign33650_e38626: f64 = (locals.var_w_bl * locals.var_q_ndepm);
        (assign33650_e38626, ((locals.var_w_bl_dn0 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn0)), ((locals.var_w_bl_dn2 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn2)), ((locals.var_w_bl_dn4 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn4)), ((locals.var_w_bl_dn5 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn5)), ((locals.var_w_bl_dn6 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn6)), ((locals.var_w_bl_dn7 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn7)), ((locals.var_w_bl_dn8 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn8)), ((locals.var_w_bl_dn9 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn9)), ((locals.var_w_bl_dn10 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn10)), ((locals.var_w_bl_dn11 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn11)), ((locals.var_w_bl_dn14 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn14)),)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn4, locals.var_q_bl_dep_dn5, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn8, locals.var_q_bl_dep_dn9, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn14,)
    }
};
        locals.var_q_bl_dep = assign33650_e38628;
        locals.var_q_bl_dep_dn0 = assign33650_e38628_d_n0;
        locals.var_q_bl_dep_dn2 = assign33650_e38628_d_n2;
        locals.var_q_bl_dep_dn4 = assign33650_e38628_d_n4;
        locals.var_q_bl_dep_dn5 = assign33650_e38628_d_n5;
        locals.var_q_bl_dep_dn6 = assign33650_e38628_d_n6;
        locals.var_q_bl_dep_dn7 = assign33650_e38628_d_n7;
        locals.var_q_bl_dep_dn8 = assign33650_e38628_d_n8;
        locals.var_q_bl_dep_dn9 = assign33650_e38628_d_n9;
        locals.var_q_bl_dep_dn10 = assign33650_e38628_d_n10;
        locals.var_q_bl_dep_dn11 = assign33650_e38628_d_n11;
        locals.var_q_bl_dep_dn14 = assign33650_e38628_d_n14;
        locals.var_q_bl_dep_rv = 0.0;

        let (assign33660_e38643, assign33660_e38643_d_n0, assign33660_e38643_d_n2, assign33660_e38643_d_n4, assign33660_e38643_d_n5, assign33660_e38643_d_n6, assign33660_e38643_d_n7, assign33660_e38643_d_n8, assign33660_e38643_d_n9, assign33660_e38643_d_n10, assign33660_e38643_d_n11, assign33660_e38643_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard774 == 0.0)) {
        let assign33660_e38639: f64 = (-locals.var_w_subl);
        let assign33660_e38641: f64 = (assign33660_e38639 * locals.var_q_nsub__blk546);
        (assign33660_e38641, (((-locals.var_w_subl_dn0) * locals.var_q_nsub__blk546) + (assign33660_e38639 * locals.var_q_nsub__blk546_dn0)), (((-locals.var_w_subl_dn2) * locals.var_q_nsub__blk546) + (assign33660_e38639 * locals.var_q_nsub__blk546_dn2)), (((-locals.var_w_subl_dn4) * locals.var_q_nsub__blk546) + (assign33660_e38639 * locals.var_q_nsub__blk546_dn4)), (((-locals.var_w_subl_dn5) * locals.var_q_nsub__blk546) + (assign33660_e38639 * locals.var_q_nsub__blk546_dn5)), (((-locals.var_w_subl_dn6) * locals.var_q_nsub__blk546) + (assign33660_e38639 * locals.var_q_nsub__blk546_dn6)), (((-locals.var_w_subl_dn7) * locals.var_q_nsub__blk546) + (assign33660_e38639 * locals.var_q_nsub__blk546_dn7)), (((-locals.var_w_subl_dn8) * locals.var_q_nsub__blk546) + (assign33660_e38639 * locals.var_q_nsub__blk546_dn8)), (((-locals.var_w_subl_dn9) * locals.var_q_nsub__blk546) + (assign33660_e38639 * locals.var_q_nsub__blk546_dn9)), (((-locals.var_w_subl_dn10) * locals.var_q_nsub__blk546) + (assign33660_e38639 * locals.var_q_nsub__blk546_dn10)), (((-locals.var_w_subl_dn11) * locals.var_q_nsub__blk546) + (assign33660_e38639 * locals.var_q_nsub__blk546_dn11)), (((-locals.var_w_subl_dn14) * locals.var_q_nsub__blk546) + (assign33660_e38639 * locals.var_q_nsub__blk546_dn14)),)
    } else {
        (locals.var_q_subl_dep, locals.var_q_subl_dep_dn0, locals.var_q_subl_dep_dn2, locals.var_q_subl_dep_dn4, locals.var_q_subl_dep_dn5, locals.var_q_subl_dep_dn6, locals.var_q_subl_dep_dn7, locals.var_q_subl_dep_dn8, locals.var_q_subl_dep_dn9, locals.var_q_subl_dep_dn10, locals.var_q_subl_dep_dn11, locals.var_q_subl_dep_dn14,)
    }
};
        locals.var_q_subl_dep = assign33660_e38643;
        locals.var_q_subl_dep_dn0 = assign33660_e38643_d_n0;
        locals.var_q_subl_dep_dn2 = assign33660_e38643_d_n2;
        locals.var_q_subl_dep_dn4 = assign33660_e38643_d_n4;
        locals.var_q_subl_dep_dn5 = assign33660_e38643_d_n5;
        locals.var_q_subl_dep_dn6 = assign33660_e38643_d_n6;
        locals.var_q_subl_dep_dn7 = assign33660_e38643_d_n7;
        locals.var_q_subl_dep_dn8 = assign33660_e38643_d_n8;
        locals.var_q_subl_dep_dn9 = assign33660_e38643_d_n9;
        locals.var_q_subl_dep_dn10 = assign33660_e38643_d_n10;
        locals.var_q_subl_dep_dn11 = assign33660_e38643_d_n11;
        locals.var_q_subl_dep_dn14 = assign33660_e38643_d_n14;
        locals.var_q_subl_dep_rv = 0.0;

        let assign33670_e38646: f64 = (locals.var_phi_sl_dep - locals.var_vds_maxbl);
        let assign33670_e38649: f64 = 0.06;
        let assign33670_e38654: f64 = if ((assign33670_e38646 < assign33670_e38649) && (0.06 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard794 = assign33670_e38654;
        locals.var_guard794_rv = 0.0;

        let (assign33680_e38671, assign33680_e38671_d_n0, assign33680_e38671_d_n2, assign33680_e38671_d_n4, assign33680_e38671_d_n5, assign33680_e38671_d_n6, assign33680_e38671_d_n7, assign33680_e38671_d_n8, assign33680_e38671_d_n9, assign33680_e38671_d_n10, assign33680_e38671_d_n11, assign33680_e38671_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard794 != 0.0)) {
        let assign33680_e38665: f64 = 0.06;
        let assign33680_e38668: f64 = (locals.var_phi_sl_dep - locals.var_vds_maxbl);
        let assign33680_e38669: f64 = (assign33680_e38665 - assign33680_e38668);
        (assign33680_e38669, (-(locals.var_phi_sl_dep_dn0 - locals.var_vds_maxbl_dn0)), (-(locals.var_phi_sl_dep_dn2 - locals.var_vds_maxbl_dn2)), (-(locals.var_phi_sl_dep_dn4 - locals.var_vds_maxbl_dn4)), (-(locals.var_phi_sl_dep_dn5 - locals.var_vds_maxbl_dn5)), (-(locals.var_phi_sl_dep_dn6 - locals.var_vds_maxbl_dn6)), (-(locals.var_phi_sl_dep_dn7 - locals.var_vds_maxbl_dn7)), (-(locals.var_phi_sl_dep_dn8 - locals.var_vds_maxbl_dn8)), (-(locals.var_phi_sl_dep_dn9 - locals.var_vds_maxbl_dn9)), (-(locals.var_phi_sl_dep_dn10 - locals.var_vds_maxbl_dn10)), (-(locals.var_phi_sl_dep_dn11 - locals.var_vds_maxbl_dn11)), (-(locals.var_phi_sl_dep_dn14 - locals.var_vds_maxbl_dn14)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign33680_e38671;
        locals.var_tmf1_dn0 = assign33680_e38671_d_n0;
        locals.var_tmf1_dn2 = assign33680_e38671_d_n2;
        locals.var_tmf1_dn4 = assign33680_e38671_d_n4;
        locals.var_tmf1_dn5 = assign33680_e38671_d_n5;
        locals.var_tmf1_dn6 = assign33680_e38671_d_n6;
        locals.var_tmf1_dn7 = assign33680_e38671_d_n7;
        locals.var_tmf1_dn8 = assign33680_e38671_d_n8;
        locals.var_tmf1_dn9 = assign33680_e38671_d_n9;
        locals.var_tmf1_dn10 = assign33680_e38671_d_n10;
        locals.var_tmf1_dn11 = assign33680_e38671_d_n11;
        locals.var_tmf1_dn14 = assign33680_e38671_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign33690_e38684, assign33690_e38684_d_n0, assign33690_e38684_d_n2, assign33690_e38684_d_n4, assign33690_e38684_d_n5, assign33690_e38684_d_n6, assign33690_e38684_d_n7, assign33690_e38684_d_n8, assign33690_e38684_d_n9, assign33690_e38684_d_n10, assign33690_e38684_d_n11, assign33690_e38684_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard794 != 0.0)) {
        let assign33690_e38682: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign33690_e38682, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign33690_e38684;
        locals.var_x2_dn0 = assign33690_e38684_d_n0;
        locals.var_x2_dn2 = assign33690_e38684_d_n2;
        locals.var_x2_dn4 = assign33690_e38684_d_n4;
        locals.var_x2_dn5 = assign33690_e38684_d_n5;
        locals.var_x2_dn6 = assign33690_e38684_d_n6;
        locals.var_x2_dn7 = assign33690_e38684_d_n7;
        locals.var_x2_dn8 = assign33690_e38684_d_n8;
        locals.var_x2_dn9 = assign33690_e38684_d_n9;
        locals.var_x2_dn10 = assign33690_e38684_d_n10;
        locals.var_x2_dn11 = assign33690_e38684_d_n11;
        locals.var_x2_dn14 = assign33690_e38684_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign33700_e38697, assign33700_e38697_d_n0, assign33700_e38697_d_n2, assign33700_e38697_d_n4, assign33700_e38697_d_n5, assign33700_e38697_d_n6, assign33700_e38697_d_n7, assign33700_e38697_d_n8, assign33700_e38697_d_n9, assign33700_e38697_d_n10, assign33700_e38697_d_n11, assign33700_e38697_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard794 != 0.0)) {
        let assign33700_e38695: f64 = (0.06 * 0.06);
        (assign33700_e38695, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign33700_e38697;
        locals.var_xmax2_dn0 = assign33700_e38697_d_n0;
        locals.var_xmax2_dn2 = assign33700_e38697_d_n2;
        locals.var_xmax2_dn4 = assign33700_e38697_d_n4;
        locals.var_xmax2_dn5 = assign33700_e38697_d_n5;
        locals.var_xmax2_dn6 = assign33700_e38697_d_n6;
        locals.var_xmax2_dn7 = assign33700_e38697_d_n7;
        locals.var_xmax2_dn8 = assign33700_e38697_d_n8;
        locals.var_xmax2_dn9 = assign33700_e38697_d_n9;
        locals.var_xmax2_dn10 = assign33700_e38697_d_n10;
        locals.var_xmax2_dn11 = assign33700_e38697_d_n11;
        locals.var_xmax2_dn14 = assign33700_e38697_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign33710_e38708, assign33710_e38708_d_n0, assign33710_e38708_d_n2, assign33710_e38708_d_n4, assign33710_e38708_d_n5, assign33710_e38708_d_n6, assign33710_e38708_d_n7, assign33710_e38708_d_n8, assign33710_e38708_d_n9, assign33710_e38708_d_n10, assign33710_e38708_d_n11, assign33710_e38708_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard794 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign33710_e38708;
        locals.var_xp_dn0 = assign33710_e38708_d_n0;
        locals.var_xp_dn2 = assign33710_e38708_d_n2;
        locals.var_xp_dn4 = assign33710_e38708_d_n4;
        locals.var_xp_dn5 = assign33710_e38708_d_n5;
        locals.var_xp_dn6 = assign33710_e38708_d_n6;
        locals.var_xp_dn7 = assign33710_e38708_d_n7;
        locals.var_xp_dn8 = assign33710_e38708_d_n8;
        locals.var_xp_dn9 = assign33710_e38708_d_n9;
        locals.var_xp_dn10 = assign33710_e38708_d_n10;
        locals.var_xp_dn11 = assign33710_e38708_d_n11;
        locals.var_xp_dn14 = assign33710_e38708_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign33720_e38719, assign33720_e38719_d_n0, assign33720_e38719_d_n2, assign33720_e38719_d_n4, assign33720_e38719_d_n5, assign33720_e38719_d_n6, assign33720_e38719_d_n7, assign33720_e38719_d_n8, assign33720_e38719_d_n9, assign33720_e38719_d_n10, assign33720_e38719_d_n11, assign33720_e38719_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard794 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign33720_e38719;
        locals.var_xmp_dn0 = assign33720_e38719_d_n0;
        locals.var_xmp_dn2 = assign33720_e38719_d_n2;
        locals.var_xmp_dn4 = assign33720_e38719_d_n4;
        locals.var_xmp_dn5 = assign33720_e38719_d_n5;
        locals.var_xmp_dn6 = assign33720_e38719_d_n6;
        locals.var_xmp_dn7 = assign33720_e38719_d_n7;
        locals.var_xmp_dn8 = assign33720_e38719_d_n8;
        locals.var_xmp_dn9 = assign33720_e38719_d_n9;
        locals.var_xmp_dn10 = assign33720_e38719_d_n10;
        locals.var_xmp_dn11 = assign33720_e38719_d_n11;
        locals.var_xmp_dn14 = assign33720_e38719_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign33730_e38730,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard794 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign33730_e38730;
        locals.var_m0_rv = 0.0;

        let (assign33740_e38741,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard794 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33740_e38741;
        locals.var_mm_rv = 0.0;

        let (assign33750_e38752, assign33750_e38752_d_n0, assign33750_e38752_d_n2, assign33750_e38752_d_n4, assign33750_e38752_d_n5, assign33750_e38752_d_n6, assign33750_e38752_d_n7, assign33750_e38752_d_n8, assign33750_e38752_d_n9, assign33750_e38752_d_n10, assign33750_e38752_d_n11, assign33750_e38752_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard794 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign33750_e38752;
        locals.var_arg_dn0 = assign33750_e38752_d_n0;
        locals.var_arg_dn2 = assign33750_e38752_d_n2;
        locals.var_arg_dn4 = assign33750_e38752_d_n4;
        locals.var_arg_dn5 = assign33750_e38752_d_n5;
        locals.var_arg_dn6 = assign33750_e38752_d_n6;
        locals.var_arg_dn7 = assign33750_e38752_d_n7;
        locals.var_arg_dn8 = assign33750_e38752_d_n8;
        locals.var_arg_dn9 = assign33750_e38752_d_n9;
        locals.var_arg_dn10 = assign33750_e38752_d_n10;
        locals.var_arg_dn11 = assign33750_e38752_d_n11;
        locals.var_arg_dn14 = assign33750_e38752_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign33760_e38763, assign33760_e38763_d_n0, assign33760_e38763_d_n2, assign33760_e38763_d_n4, assign33760_e38763_d_n5, assign33760_e38763_d_n6, assign33760_e38763_d_n7, assign33760_e38763_d_n8, assign33760_e38763_d_n9, assign33760_e38763_d_n10, assign33760_e38763_d_n11, assign33760_e38763_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard794 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign33760_e38763;
        locals.var_dnm_dn0 = assign33760_e38763_d_n0;
        locals.var_dnm_dn2 = assign33760_e38763_d_n2;
        locals.var_dnm_dn4 = assign33760_e38763_d_n4;
        locals.var_dnm_dn5 = assign33760_e38763_d_n5;
        locals.var_dnm_dn6 = assign33760_e38763_d_n6;
        locals.var_dnm_dn7 = assign33760_e38763_d_n7;
        locals.var_dnm_dn8 = assign33760_e38763_d_n8;
        locals.var_dnm_dn9 = assign33760_e38763_d_n9;
        locals.var_dnm_dn10 = assign33760_e38763_d_n10;
        locals.var_dnm_dn11 = assign33760_e38763_d_n11;
        locals.var_dnm_dn14 = assign33760_e38763_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign33770_e38776, assign33770_e38776_d_n0, assign33770_e38776_d_n2, assign33770_e38776_d_n4, assign33770_e38776_d_n5, assign33770_e38776_d_n6, assign33770_e38776_d_n7, assign33770_e38776_d_n8, assign33770_e38776_d_n9, assign33770_e38776_d_n10, assign33770_e38776_d_n11, assign33770_e38776_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard794 != 0.0)) {
        let assign33770_e38774: f64 = (locals.var_xp * locals.var_x2);
        (assign33770_e38774, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign33770_e38776;
        locals.var_xp_dn0 = assign33770_e38776_d_n0;
        locals.var_xp_dn2 = assign33770_e38776_d_n2;
        locals.var_xp_dn4 = assign33770_e38776_d_n4;
        locals.var_xp_dn5 = assign33770_e38776_d_n5;
        locals.var_xp_dn6 = assign33770_e38776_d_n6;
        locals.var_xp_dn7 = assign33770_e38776_d_n7;
        locals.var_xp_dn8 = assign33770_e38776_d_n8;
        locals.var_xp_dn9 = assign33770_e38776_d_n9;
        locals.var_xp_dn10 = assign33770_e38776_d_n10;
        locals.var_xp_dn11 = assign33770_e38776_d_n11;
        locals.var_xp_dn14 = assign33770_e38776_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign33780_e38789, assign33780_e38789_d_n0, assign33780_e38789_d_n2, assign33780_e38789_d_n4, assign33780_e38789_d_n5, assign33780_e38789_d_n6, assign33780_e38789_d_n7, assign33780_e38789_d_n8, assign33780_e38789_d_n9, assign33780_e38789_d_n10, assign33780_e38789_d_n11, assign33780_e38789_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard794 != 0.0)) {
        let assign33780_e38787: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign33780_e38787, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign33780_e38789;
        locals.var_xmp_dn0 = assign33780_e38789_d_n0;
        locals.var_xmp_dn2 = assign33780_e38789_d_n2;
        locals.var_xmp_dn4 = assign33780_e38789_d_n4;
        locals.var_xmp_dn5 = assign33780_e38789_d_n5;
        locals.var_xmp_dn6 = assign33780_e38789_d_n6;
        locals.var_xmp_dn7 = assign33780_e38789_d_n7;
        locals.var_xmp_dn8 = assign33780_e38789_d_n8;
        locals.var_xmp_dn9 = assign33780_e38789_d_n9;
        locals.var_xmp_dn10 = assign33780_e38789_d_n10;
        locals.var_xmp_dn11 = assign33780_e38789_d_n11;
        locals.var_xmp_dn14 = assign33780_e38789_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign33790_e38802, assign33790_e38802_d_n0, assign33790_e38802_d_n2, assign33790_e38802_d_n4, assign33790_e38802_d_n5, assign33790_e38802_d_n6, assign33790_e38802_d_n7, assign33790_e38802_d_n8, assign33790_e38802_d_n9, assign33790_e38802_d_n10, assign33790_e38802_d_n11, assign33790_e38802_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard794 != 0.0)) {
        let assign33790_e38800: f64 = (locals.var_xp * locals.var_x2);
        (assign33790_e38800, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign33790_e38802;
        locals.var_xp_dn0 = assign33790_e38802_d_n0;
        locals.var_xp_dn2 = assign33790_e38802_d_n2;
        locals.var_xp_dn4 = assign33790_e38802_d_n4;
        locals.var_xp_dn5 = assign33790_e38802_d_n5;
        locals.var_xp_dn6 = assign33790_e38802_d_n6;
        locals.var_xp_dn7 = assign33790_e38802_d_n7;
        locals.var_xp_dn8 = assign33790_e38802_d_n8;
        locals.var_xp_dn9 = assign33790_e38802_d_n9;
        locals.var_xp_dn10 = assign33790_e38802_d_n10;
        locals.var_xp_dn11 = assign33790_e38802_d_n11;
        locals.var_xp_dn14 = assign33790_e38802_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign33800_e38815, assign33800_e38815_d_n0, assign33800_e38815_d_n2, assign33800_e38815_d_n4, assign33800_e38815_d_n5, assign33800_e38815_d_n6, assign33800_e38815_d_n7, assign33800_e38815_d_n8, assign33800_e38815_d_n9, assign33800_e38815_d_n10, assign33800_e38815_d_n11, assign33800_e38815_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard794 != 0.0)) {
        let assign33800_e38813: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign33800_e38813, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign33800_e38815;
        locals.var_xmp_dn0 = assign33800_e38815_d_n0;
        locals.var_xmp_dn2 = assign33800_e38815_d_n2;
        locals.var_xmp_dn4 = assign33800_e38815_d_n4;
        locals.var_xmp_dn5 = assign33800_e38815_d_n5;
        locals.var_xmp_dn6 = assign33800_e38815_d_n6;
        locals.var_xmp_dn7 = assign33800_e38815_d_n7;
        locals.var_xmp_dn8 = assign33800_e38815_d_n8;
        locals.var_xmp_dn9 = assign33800_e38815_d_n9;
        locals.var_xmp_dn10 = assign33800_e38815_d_n10;
        locals.var_xmp_dn11 = assign33800_e38815_d_n11;
        locals.var_xmp_dn14 = assign33800_e38815_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign33810_e38828, assign33810_e38828_d_n0, assign33810_e38828_d_n2, assign33810_e38828_d_n4, assign33810_e38828_d_n5, assign33810_e38828_d_n6, assign33810_e38828_d_n7, assign33810_e38828_d_n8, assign33810_e38828_d_n9, assign33810_e38828_d_n10, assign33810_e38828_d_n11, assign33810_e38828_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard794 != 0.0)) {
        let assign33810_e38826: f64 = (locals.var_xp + locals.var_xmp);
        (assign33810_e38826, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign33810_e38828;
        locals.var_arg_dn0 = assign33810_e38828_d_n0;
        locals.var_arg_dn2 = assign33810_e38828_d_n2;
        locals.var_arg_dn4 = assign33810_e38828_d_n4;
        locals.var_arg_dn5 = assign33810_e38828_d_n5;
        locals.var_arg_dn6 = assign33810_e38828_d_n6;
        locals.var_arg_dn7 = assign33810_e38828_d_n7;
        locals.var_arg_dn8 = assign33810_e38828_d_n8;
        locals.var_arg_dn9 = assign33810_e38828_d_n9;
        locals.var_arg_dn10 = assign33810_e38828_d_n10;
        locals.var_arg_dn11 = assign33810_e38828_d_n11;
        locals.var_arg_dn14 = assign33810_e38828_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign33820_e38839, assign33820_e38839_d_n0, assign33820_e38839_d_n2, assign33820_e38839_d_n4, assign33820_e38839_d_n5, assign33820_e38839_d_n6, assign33820_e38839_d_n7, assign33820_e38839_d_n8, assign33820_e38839_d_n9, assign33820_e38839_d_n10, assign33820_e38839_d_n11, assign33820_e38839_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard794 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign33820_e38839;
        locals.var_dnm_dn0 = assign33820_e38839_d_n0;
        locals.var_dnm_dn2 = assign33820_e38839_d_n2;
        locals.var_dnm_dn4 = assign33820_e38839_d_n4;
        locals.var_dnm_dn5 = assign33820_e38839_d_n5;
        locals.var_dnm_dn6 = assign33820_e38839_d_n6;
        locals.var_dnm_dn7 = assign33820_e38839_d_n7;
        locals.var_dnm_dn8 = assign33820_e38839_d_n8;
        locals.var_dnm_dn9 = assign33820_e38839_d_n9;
        locals.var_dnm_dn10 = assign33820_e38839_d_n10;
        locals.var_dnm_dn11 = assign33820_e38839_d_n11;
        locals.var_dnm_dn14 = assign33820_e38839_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign33830_e38854: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard795 = assign33830_e38854;
        locals.var_guard795_rv = 0.0;

        let assign33840_e38857: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard796 = assign33840_e38857;
        locals.var_guard796_rv = 0.0;

        let (assign33850_e38872,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard794 != 0.0)) && (locals.var_guard795 != 0.0)) && (locals.var_guard796 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33850_e38872;
        locals.var_mm_rv = 0.0;

        let assign33860_e38875: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard797 = assign33860_e38875;
        locals.var_guard797_rv = 0.0;

        let (assign33870_e38893,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard794 != 0.0)) && (locals.var_guard795 != 0.0)) && (locals.var_guard796 == 0.0)) && (locals.var_guard797 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33870_e38893;
        locals.var_mm_rv = 0.0;

        let assign33880_e38896: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard798 = assign33880_e38896;
        locals.var_guard798_rv = 0.0;

        let (assign33890_e38917,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard794 != 0.0)) && (locals.var_guard795 != 0.0)) && (locals.var_guard796 == 0.0)) && (locals.var_guard797 == 0.0)) && (locals.var_guard798 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33890_e38917;
        locals.var_mm_rv = 0.0;

        let assign33900_e38920: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard799 = assign33900_e38920;
        locals.var_guard799_rv = 0.0;

        let (assign33910_e38944,) = {
    if (((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard794 != 0.0)) && (locals.var_guard795 != 0.0)) && (locals.var_guard796 == 0.0)) && (locals.var_guard797 == 0.0)) && (locals.var_guard798 == 0.0)) && (locals.var_guard799 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33910_e38944;
        locals.var_mm_rv = 0.0;

        let (assign33920_e38957,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard794 != 0.0)) && (locals.var_guard795 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign33920_e38957;
        locals.var_m0_rv = 0.0;

        let mut assign33930_loop_guard: usize = 0;
        while {
            let assign33930_cond_e38971: f64 = if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard794 != 0.0)) && (locals.var_guard795 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign33930_cond_e38971 != 0.0
        } {
            assign33930_loop_guard += 1;
            assert!(assign33930_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign33930_body0_e38985, assign33930_body0_e38985_d_n0, assign33930_body0_e38985_d_n2, assign33930_body0_e38985_d_n4, assign33930_body0_e38985_d_n5, assign33930_body0_e38985_d_n6, assign33930_body0_e38985_d_n7, assign33930_body0_e38985_d_n8, assign33930_body0_e38985_d_n9, assign33930_body0_e38985_d_n10, assign33930_body0_e38985_d_n11, assign33930_body0_e38985_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard794 != 0.0)) && (locals.var_guard795 != 0.0)) {
        let assign33930_body0_e38983: f64 = (locals.var_dnm).sqrt();
        (assign33930_body0_e38983, (locals.var_dnm_dn0 / (2.0 * assign33930_body0_e38983)), (locals.var_dnm_dn2 / (2.0 * assign33930_body0_e38983)), (locals.var_dnm_dn4 / (2.0 * assign33930_body0_e38983)), (locals.var_dnm_dn5 / (2.0 * assign33930_body0_e38983)), (locals.var_dnm_dn6 / (2.0 * assign33930_body0_e38983)), (locals.var_dnm_dn7 / (2.0 * assign33930_body0_e38983)), (locals.var_dnm_dn8 / (2.0 * assign33930_body0_e38983)), (locals.var_dnm_dn9 / (2.0 * assign33930_body0_e38983)), (locals.var_dnm_dn10 / (2.0 * assign33930_body0_e38983)), (locals.var_dnm_dn11 / (2.0 * assign33930_body0_e38983)), (locals.var_dnm_dn14 / (2.0 * assign33930_body0_e38983)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign33930_body0_e38985;
            locals.var_dnm_dn0 = assign33930_body0_e38985_d_n0;
            locals.var_dnm_dn2 = assign33930_body0_e38985_d_n2;
            locals.var_dnm_dn4 = assign33930_body0_e38985_d_n4;
            locals.var_dnm_dn5 = assign33930_body0_e38985_d_n5;
            locals.var_dnm_dn6 = assign33930_body0_e38985_d_n6;
            locals.var_dnm_dn7 = assign33930_body0_e38985_d_n7;
            locals.var_dnm_dn8 = assign33930_body0_e38985_d_n8;
            locals.var_dnm_dn9 = assign33930_body0_e38985_d_n9;
            locals.var_dnm_dn10 = assign33930_body0_e38985_d_n10;
            locals.var_dnm_dn11 = assign33930_body0_e38985_d_n11;
            locals.var_dnm_dn14 = assign33930_body0_e38985_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign33930_body1_e39000,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard794 != 0.0)) && (locals.var_guard795 != 0.0)) {
        let assign33930_body1_e38998: f64 = (locals.var_m0 + 1.0);
        (assign33930_body1_e38998,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign33930_body1_e39000;
            locals.var_m0_rv = 0.0;
        }

        let (assign33940_e39025, assign33940_e39025_d_n0, assign33940_e39025_d_n2, assign33940_e39025_d_n4, assign33940_e39025_d_n5, assign33940_e39025_d_n6, assign33940_e39025_d_n7, assign33940_e39025_d_n8, assign33940_e39025_d_n9, assign33940_e39025_d_n10, assign33940_e39025_d_n11, assign33940_e39025_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard794 != 0.0)) && (locals.var_guard795 == 0.0)) {
        let (assign33940_e39023, assign33940_e39023_d_n0, assign33940_e39023_d_n2, assign33940_e39023_d_n4, assign33940_e39023_d_n5, assign33940_e39023_d_n6, assign33940_e39023_d_n7, assign33940_e39023_d_n8, assign33940_e39023_d_n9, assign33940_e39023_d_n10, assign33940_e39023_d_n11, assign33940_e39023_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign33940_e39020: f64 = (2.0 * 2.0);
                let assign33940_e39021: f64 = (1.0 / assign33940_e39020);
                let assign33940_e39022: f64 = (locals.var_dnm).powf(assign33940_e39021);
                (assign33940_e39022, if 0.0 == 0.0 && ((assign33940_e39021) as f64).is_finite() && ((assign33940_e39021) as f64).fract() == 0.0 { if assign33940_e39021 == 0.0 { 0.0 } else { (assign33940_e39021 * ((locals.var_dnm).powf(assign33940_e39021 - 1.0) * locals.var_dnm_dn0)) } } else { (assign33940_e39022 * (assign33940_e39021 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33940_e39021) as f64).is_finite() && ((assign33940_e39021) as f64).fract() == 0.0 { if assign33940_e39021 == 0.0 { 0.0 } else { (assign33940_e39021 * ((locals.var_dnm).powf(assign33940_e39021 - 1.0) * locals.var_dnm_dn2)) } } else { (assign33940_e39022 * (assign33940_e39021 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33940_e39021) as f64).is_finite() && ((assign33940_e39021) as f64).fract() == 0.0 { if assign33940_e39021 == 0.0 { 0.0 } else { (assign33940_e39021 * ((locals.var_dnm).powf(assign33940_e39021 - 1.0) * locals.var_dnm_dn4)) } } else { (assign33940_e39022 * (assign33940_e39021 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33940_e39021) as f64).is_finite() && ((assign33940_e39021) as f64).fract() == 0.0 { if assign33940_e39021 == 0.0 { 0.0 } else { (assign33940_e39021 * ((locals.var_dnm).powf(assign33940_e39021 - 1.0) * locals.var_dnm_dn5)) } } else { (assign33940_e39022 * (assign33940_e39021 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33940_e39021) as f64).is_finite() && ((assign33940_e39021) as f64).fract() == 0.0 { if assign33940_e39021 == 0.0 { 0.0 } else { (assign33940_e39021 * ((locals.var_dnm).powf(assign33940_e39021 - 1.0) * locals.var_dnm_dn6)) } } else { (assign33940_e39022 * (assign33940_e39021 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33940_e39021) as f64).is_finite() && ((assign33940_e39021) as f64).fract() == 0.0 { if assign33940_e39021 == 0.0 { 0.0 } else { (assign33940_e39021 * ((locals.var_dnm).powf(assign33940_e39021 - 1.0) * locals.var_dnm_dn7)) } } else { (assign33940_e39022 * (assign33940_e39021 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33940_e39021) as f64).is_finite() && ((assign33940_e39021) as f64).fract() == 0.0 { if assign33940_e39021 == 0.0 { 0.0 } else { (assign33940_e39021 * ((locals.var_dnm).powf(assign33940_e39021 - 1.0) * locals.var_dnm_dn8)) } } else { (assign33940_e39022 * (assign33940_e39021 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33940_e39021) as f64).is_finite() && ((assign33940_e39021) as f64).fract() == 0.0 { if assign33940_e39021 == 0.0 { 0.0 } else { (assign33940_e39021 * ((locals.var_dnm).powf(assign33940_e39021 - 1.0) * locals.var_dnm_dn9)) } } else { (assign33940_e39022 * (assign33940_e39021 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33940_e39021) as f64).is_finite() && ((assign33940_e39021) as f64).fract() == 0.0 { if assign33940_e39021 == 0.0 { 0.0 } else { (assign33940_e39021 * ((locals.var_dnm).powf(assign33940_e39021 - 1.0) * locals.var_dnm_dn10)) } } else { (assign33940_e39022 * (assign33940_e39021 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33940_e39021) as f64).is_finite() && ((assign33940_e39021) as f64).fract() == 0.0 { if assign33940_e39021 == 0.0 { 0.0 } else { (assign33940_e39021 * ((locals.var_dnm).powf(assign33940_e39021 - 1.0) * locals.var_dnm_dn11)) } } else { (assign33940_e39022 * (assign33940_e39021 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33940_e39021) as f64).is_finite() && ((assign33940_e39021) as f64).fract() == 0.0 { if assign33940_e39021 == 0.0 { 0.0 } else { (assign33940_e39021 * ((locals.var_dnm).powf(assign33940_e39021 - 1.0) * locals.var_dnm_dn14)) } } else { (assign33940_e39022 * (assign33940_e39021 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign33940_e39023, assign33940_e39023_d_n0, assign33940_e39023_d_n2, assign33940_e39023_d_n4, assign33940_e39023_d_n5, assign33940_e39023_d_n6, assign33940_e39023_d_n7, assign33940_e39023_d_n8, assign33940_e39023_d_n9, assign33940_e39023_d_n10, assign33940_e39023_d_n11, assign33940_e39023_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign33940_e39025;
        locals.var_dnm_dn0 = assign33940_e39025_d_n0;
        locals.var_dnm_dn2 = assign33940_e39025_d_n2;
        locals.var_dnm_dn4 = assign33940_e39025_d_n4;
        locals.var_dnm_dn5 = assign33940_e39025_d_n5;
        locals.var_dnm_dn6 = assign33940_e39025_d_n6;
        locals.var_dnm_dn7 = assign33940_e39025_d_n7;
        locals.var_dnm_dn8 = assign33940_e39025_d_n8;
        locals.var_dnm_dn9 = assign33940_e39025_d_n9;
        locals.var_dnm_dn10 = assign33940_e39025_d_n10;
        locals.var_dnm_dn11 = assign33940_e39025_d_n11;
        locals.var_dnm_dn14 = assign33940_e39025_d_n14;
        locals.var_dnm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_110(
        locals: &mut StampLocals,
    ) {
        let (assign33950_e39038, assign33950_e39038_d_n0, assign33950_e39038_d_n2, assign33950_e39038_d_n4, assign33950_e39038_d_n5, assign33950_e39038_d_n6, assign33950_e39038_d_n7, assign33950_e39038_d_n8, assign33950_e39038_d_n9, assign33950_e39038_d_n10, assign33950_e39038_d_n11, assign33950_e39038_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard794 != 0.0)) {
        let assign33950_e39036: f64 = (1.0 / locals.var_dnm);
        (assign33950_e39036, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign33950_e39038;
        locals.var_dnm_dn0 = assign33950_e39038_d_n0;
        locals.var_dnm_dn2 = assign33950_e39038_d_n2;
        locals.var_dnm_dn4 = assign33950_e39038_d_n4;
        locals.var_dnm_dn5 = assign33950_e39038_d_n5;
        locals.var_dnm_dn6 = assign33950_e39038_d_n6;
        locals.var_dnm_dn7 = assign33950_e39038_d_n7;
        locals.var_dnm_dn8 = assign33950_e39038_d_n8;
        locals.var_dnm_dn9 = assign33950_e39038_d_n9;
        locals.var_dnm_dn10 = assign33950_e39038_d_n10;
        locals.var_dnm_dn11 = assign33950_e39038_d_n11;
        locals.var_dnm_dn14 = assign33950_e39038_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign33960_e39053, assign33960_e39053_d_n0, assign33960_e39053_d_n2, assign33960_e39053_d_n4, assign33960_e39053_d_n5, assign33960_e39053_d_n6, assign33960_e39053_d_n7, assign33960_e39053_d_n8, assign33960_e39053_d_n9, assign33960_e39053_d_n10, assign33960_e39053_d_n11, assign33960_e39053_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard794 != 0.0)) {
        let assign33960_e39049: f64 = (locals.var_tmf1 * 0.06);
        let assign33960_e39051: f64 = (assign33960_e39049 * locals.var_dnm);
        (assign33960_e39051, (((locals.var_tmf1_dn0 * 0.06) * locals.var_dnm) + (assign33960_e39049 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.06) * locals.var_dnm) + (assign33960_e39049 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.06) * locals.var_dnm) + (assign33960_e39049 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.06) * locals.var_dnm) + (assign33960_e39049 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.06) * locals.var_dnm) + (assign33960_e39049 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.06) * locals.var_dnm) + (assign33960_e39049 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.06) * locals.var_dnm) + (assign33960_e39049 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.06) * locals.var_dnm) + (assign33960_e39049 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.06) * locals.var_dnm) + (assign33960_e39049 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.06) * locals.var_dnm) + (assign33960_e39049 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.06) * locals.var_dnm) + (assign33960_e39049 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign33960_e39053;
        locals.var_tmf0_dn0 = assign33960_e39053_d_n0;
        locals.var_tmf0_dn2 = assign33960_e39053_d_n2;
        locals.var_tmf0_dn4 = assign33960_e39053_d_n4;
        locals.var_tmf0_dn5 = assign33960_e39053_d_n5;
        locals.var_tmf0_dn6 = assign33960_e39053_d_n6;
        locals.var_tmf0_dn7 = assign33960_e39053_d_n7;
        locals.var_tmf0_dn8 = assign33960_e39053_d_n8;
        locals.var_tmf0_dn9 = assign33960_e39053_d_n9;
        locals.var_tmf0_dn10 = assign33960_e39053_d_n10;
        locals.var_tmf0_dn11 = assign33960_e39053_d_n11;
        locals.var_tmf0_dn14 = assign33960_e39053_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign33970_e39070, assign33970_e39070_d_n0, assign33970_e39070_d_n2, assign33970_e39070_d_n4, assign33970_e39070_d_n5, assign33970_e39070_d_n6, assign33970_e39070_d_n7, assign33970_e39070_d_n8, assign33970_e39070_d_n9, assign33970_e39070_d_n10, assign33970_e39070_d_n11, assign33970_e39070_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard794 != 0.0)) {
        let assign33970_e39064: f64 = (0.06 * locals.var_xmp);
        let assign33970_e39066: f64 = (assign33970_e39064 * locals.var_dnm);
        let assign33970_e39068: f64 = (assign33970_e39066 / locals.var_arg);
        (assign33970_e39068, ((((((0.06 * locals.var_xmp_dn0) * locals.var_dnm) + (assign33970_e39064 * locals.var_dnm_dn0)) * locals.var_arg) - (assign33970_e39066 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn2) * locals.var_dnm) + (assign33970_e39064 * locals.var_dnm_dn2)) * locals.var_arg) - (assign33970_e39066 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn4) * locals.var_dnm) + (assign33970_e39064 * locals.var_dnm_dn4)) * locals.var_arg) - (assign33970_e39066 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn5) * locals.var_dnm) + (assign33970_e39064 * locals.var_dnm_dn5)) * locals.var_arg) - (assign33970_e39066 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn6) * locals.var_dnm) + (assign33970_e39064 * locals.var_dnm_dn6)) * locals.var_arg) - (assign33970_e39066 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn7) * locals.var_dnm) + (assign33970_e39064 * locals.var_dnm_dn7)) * locals.var_arg) - (assign33970_e39066 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn8) * locals.var_dnm) + (assign33970_e39064 * locals.var_dnm_dn8)) * locals.var_arg) - (assign33970_e39066 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn9) * locals.var_dnm) + (assign33970_e39064 * locals.var_dnm_dn9)) * locals.var_arg) - (assign33970_e39066 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn10) * locals.var_dnm) + (assign33970_e39064 * locals.var_dnm_dn10)) * locals.var_arg) - (assign33970_e39066 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn11) * locals.var_dnm) + (assign33970_e39064 * locals.var_dnm_dn11)) * locals.var_arg) - (assign33970_e39066 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn14) * locals.var_dnm) + (assign33970_e39064 * locals.var_dnm_dn14)) * locals.var_arg) - (assign33970_e39066 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign33970_e39070;
        locals.var_t0_dn0 = assign33970_e39070_d_n0;
        locals.var_t0_dn2 = assign33970_e39070_d_n2;
        locals.var_t0_dn4 = assign33970_e39070_d_n4;
        locals.var_t0_dn5 = assign33970_e39070_d_n5;
        locals.var_t0_dn6 = assign33970_e39070_d_n6;
        locals.var_t0_dn7 = assign33970_e39070_d_n7;
        locals.var_t0_dn8 = assign33970_e39070_d_n8;
        locals.var_t0_dn9 = assign33970_e39070_d_n9;
        locals.var_t0_dn10 = assign33970_e39070_d_n10;
        locals.var_t0_dn11 = assign33970_e39070_d_n11;
        locals.var_t0_dn14 = assign33970_e39070_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign33980_e39085, assign33980_e39085_d_n0, assign33980_e39085_d_n2, assign33980_e39085_d_n4, assign33980_e39085_d_n5, assign33980_e39085_d_n6, assign33980_e39085_d_n7, assign33980_e39085_d_n8, assign33980_e39085_d_n9, assign33980_e39085_d_n10, assign33980_e39085_d_n11, assign33980_e39085_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard794 != 0.0)) {
        let assign33980_e39081: f64 = 0.06;
        let assign33980_e39083: f64 = (assign33980_e39081 - locals.var_tmf0);
        (assign33980_e39083, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign33980_e39085;
        locals.var_t2_dn0 = assign33980_e39085_d_n0;
        locals.var_t2_dn2 = assign33980_e39085_d_n2;
        locals.var_t2_dn4 = assign33980_e39085_d_n4;
        locals.var_t2_dn5 = assign33980_e39085_d_n5;
        locals.var_t2_dn6 = assign33980_e39085_d_n6;
        locals.var_t2_dn7 = assign33980_e39085_d_n7;
        locals.var_t2_dn8 = assign33980_e39085_d_n8;
        locals.var_t2_dn9 = assign33980_e39085_d_n9;
        locals.var_t2_dn10 = assign33980_e39085_d_n10;
        locals.var_t2_dn11 = assign33980_e39085_d_n11;
        locals.var_t2_dn14 = assign33980_e39085_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign33990_e39096, assign33990_e39096_d_n0, assign33990_e39096_d_n2, assign33990_e39096_d_n4, assign33990_e39096_d_n5, assign33990_e39096_d_n6, assign33990_e39096_d_n7, assign33990_e39096_d_n8, assign33990_e39096_d_n9, assign33990_e39096_d_n10, assign33990_e39096_d_n11, assign33990_e39096_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard794 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign33990_e39096;
        locals.var_t0_dn0 = assign33990_e39096_d_n0;
        locals.var_t0_dn2 = assign33990_e39096_d_n2;
        locals.var_t0_dn4 = assign33990_e39096_d_n4;
        locals.var_t0_dn5 = assign33990_e39096_d_n5;
        locals.var_t0_dn6 = assign33990_e39096_d_n6;
        locals.var_t0_dn7 = assign33990_e39096_d_n7;
        locals.var_t0_dn8 = assign33990_e39096_d_n8;
        locals.var_t0_dn9 = assign33990_e39096_d_n9;
        locals.var_t0_dn10 = assign33990_e39096_d_n10;
        locals.var_t0_dn11 = assign33990_e39096_d_n11;
        locals.var_t0_dn14 = assign33990_e39096_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign34000_e39110, assign34000_e39110_d_n0, assign34000_e39110_d_n2, assign34000_e39110_d_n4, assign34000_e39110_d_n5, assign34000_e39110_d_n6, assign34000_e39110_d_n7, assign34000_e39110_d_n8, assign34000_e39110_d_n9, assign34000_e39110_d_n10, assign34000_e39110_d_n11, assign34000_e39110_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard794 == 0.0)) {
        let assign34000_e39108: f64 = (locals.var_phi_sl_dep - locals.var_vds_maxbl);
        (assign34000_e39108, (locals.var_phi_sl_dep_dn0 - locals.var_vds_maxbl_dn0), (locals.var_phi_sl_dep_dn2 - locals.var_vds_maxbl_dn2), (locals.var_phi_sl_dep_dn4 - locals.var_vds_maxbl_dn4), (locals.var_phi_sl_dep_dn5 - locals.var_vds_maxbl_dn5), (locals.var_phi_sl_dep_dn6 - locals.var_vds_maxbl_dn6), (locals.var_phi_sl_dep_dn7 - locals.var_vds_maxbl_dn7), (locals.var_phi_sl_dep_dn8 - locals.var_vds_maxbl_dn8), (locals.var_phi_sl_dep_dn9 - locals.var_vds_maxbl_dn9), (locals.var_phi_sl_dep_dn10 - locals.var_vds_maxbl_dn10), (locals.var_phi_sl_dep_dn11 - locals.var_vds_maxbl_dn11), (locals.var_phi_sl_dep_dn14 - locals.var_vds_maxbl_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign34000_e39110;
        locals.var_t2_dn0 = assign34000_e39110_d_n0;
        locals.var_t2_dn2 = assign34000_e39110_d_n2;
        locals.var_t2_dn4 = assign34000_e39110_d_n4;
        locals.var_t2_dn5 = assign34000_e39110_d_n5;
        locals.var_t2_dn6 = assign34000_e39110_d_n6;
        locals.var_t2_dn7 = assign34000_e39110_d_n7;
        locals.var_t2_dn8 = assign34000_e39110_d_n8;
        locals.var_t2_dn9 = assign34000_e39110_d_n9;
        locals.var_t2_dn10 = assign34000_e39110_d_n10;
        locals.var_t2_dn11 = assign34000_e39110_d_n11;
        locals.var_t2_dn14 = assign34000_e39110_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign34010_e39122, assign34010_e39122_d_n0, assign34010_e39122_d_n2, assign34010_e39122_d_n4, assign34010_e39122_d_n5, assign34010_e39122_d_n6, assign34010_e39122_d_n7, assign34010_e39122_d_n8, assign34010_e39122_d_n9, assign34010_e39122_d_n10, assign34010_e39122_d_n11, assign34010_e39122_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) && (locals.var_guard794 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign34010_e39122;
        locals.var_t0_dn0 = assign34010_e39122_d_n0;
        locals.var_t0_dn2 = assign34010_e39122_d_n2;
        locals.var_t0_dn4 = assign34010_e39122_d_n4;
        locals.var_t0_dn5 = assign34010_e39122_d_n5;
        locals.var_t0_dn6 = assign34010_e39122_d_n6;
        locals.var_t0_dn7 = assign34010_e39122_d_n7;
        locals.var_t0_dn8 = assign34010_e39122_d_n8;
        locals.var_t0_dn9 = assign34010_e39122_d_n9;
        locals.var_t0_dn10 = assign34010_e39122_d_n10;
        locals.var_t0_dn11 = assign34010_e39122_d_n11;
        locals.var_t0_dn14 = assign34010_e39122_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign34020_e39144, assign34020_e39144_d_n0, assign34020_e39144_d_n2, assign34020_e39144_d_n4, assign34020_e39144_d_n5, assign34020_e39144_d_n6, assign34020_e39144_d_n7, assign34020_e39144_d_n8, assign34020_e39144_d_n9, assign34020_e39144_d_n10, assign34020_e39144_d_n11, assign34020_e39144_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) {
        let assign34020_e39131: f64 = (locals.var_beta * locals.var_t2);
        let assign34020_e39132: f64 = (assign34020_e39131).exp();
        let assign34020_e39134: f64 = (assign34020_e39132 - 1.0);
        let assign34020_e39137: f64 = (locals.var_beta * locals.var_t2);
        let assign34020_e39138: f64 = (assign34020_e39134 - assign34020_e39137);
        let assign34020_e39141: f64 = (10.0 * 2.220446049250313e-16);
        let assign34020_e39142: f64 = (assign34020_e39138 + assign34020_e39141);
        (assign34020_e39142, ((assign34020_e39132 * ((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0))) - ((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0))), ((assign34020_e39132 * ((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2))) - ((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2))), ((assign34020_e39132 * ((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4))) - ((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4))), ((assign34020_e39132 * ((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5))) - ((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5))), ((assign34020_e39132 * ((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6))) - ((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6))), ((assign34020_e39132 * ((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7))) - ((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7))), ((assign34020_e39132 * ((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8))) - ((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8))), ((assign34020_e39132 * ((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9))) - ((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9))), ((assign34020_e39132 * ((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10))) - ((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10))), ((assign34020_e39132 * ((locals.var_beta_dn11 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn11))) - ((locals.var_beta_dn11 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn11))), ((assign34020_e39132 * ((locals.var_beta_dn14 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn14))) - ((locals.var_beta_dn14 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign34020_e39144;
        locals.var_t4_dn0 = assign34020_e39144_d_n0;
        locals.var_t4_dn2 = assign34020_e39144_d_n2;
        locals.var_t4_dn4 = assign34020_e39144_d_n4;
        locals.var_t4_dn5 = assign34020_e39144_d_n5;
        locals.var_t4_dn6 = assign34020_e39144_d_n6;
        locals.var_t4_dn7 = assign34020_e39144_d_n7;
        locals.var_t4_dn8 = assign34020_e39144_d_n8;
        locals.var_t4_dn9 = assign34020_e39144_d_n9;
        locals.var_t4_dn10 = assign34020_e39144_d_n10;
        locals.var_t4_dn11 = assign34020_e39144_d_n11;
        locals.var_t4_dn14 = assign34020_e39144_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign34030_e39157, assign34030_e39157_d_n0, assign34030_e39157_d_n2, assign34030_e39157_d_n4, assign34030_e39157_d_n5, assign34030_e39157_d_n6, assign34030_e39157_d_n7, assign34030_e39157_d_n8, assign34030_e39157_d_n9, assign34030_e39157_d_n10, assign34030_e39157_d_n11, assign34030_e39157_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard713 == 0.0)) {
        let assign34030_e39152: f64 = (-locals.var_cnst0);
        let assign34030_e39154: f64 = (locals.var_t4).sqrt();
        let assign34030_e39155: f64 = (assign34030_e39152 * assign34030_e39154);
        (assign34030_e39155, (((-locals.var_cnst0_dn0) * assign34030_e39154) + (assign34030_e39152 * (locals.var_t4_dn0 / (2.0 * assign34030_e39154)))), (((-locals.var_cnst0_dn2) * assign34030_e39154) + (assign34030_e39152 * (locals.var_t4_dn2 / (2.0 * assign34030_e39154)))), (((-locals.var_cnst0_dn4) * assign34030_e39154) + (assign34030_e39152 * (locals.var_t4_dn4 / (2.0 * assign34030_e39154)))), (((-locals.var_cnst0_dn5) * assign34030_e39154) + (assign34030_e39152 * (locals.var_t4_dn5 / (2.0 * assign34030_e39154)))), (((-locals.var_cnst0_dn6) * assign34030_e39154) + (assign34030_e39152 * (locals.var_t4_dn6 / (2.0 * assign34030_e39154)))), (((-locals.var_cnst0_dn7) * assign34030_e39154) + (assign34030_e39152 * (locals.var_t4_dn7 / (2.0 * assign34030_e39154)))), (((-locals.var_cnst0_dn8) * assign34030_e39154) + (assign34030_e39152 * (locals.var_t4_dn8 / (2.0 * assign34030_e39154)))), (((-locals.var_cnst0_dn9) * assign34030_e39154) + (assign34030_e39152 * (locals.var_t4_dn9 / (2.0 * assign34030_e39154)))), (((-locals.var_cnst0_dn10) * assign34030_e39154) + (assign34030_e39152 * (locals.var_t4_dn10 / (2.0 * assign34030_e39154)))), (((-locals.var_cnst0_dn11) * assign34030_e39154) + (assign34030_e39152 * (locals.var_t4_dn11 / (2.0 * assign34030_e39154)))), (((-locals.var_cnst0_dn14) * assign34030_e39154) + (assign34030_e39152 * (locals.var_t4_dn14 / (2.0 * assign34030_e39154)))),)
    } else {
        (locals.var_q_nl_cur, locals.var_q_nl_cur_dn0, locals.var_q_nl_cur_dn2, locals.var_q_nl_cur_dn4, locals.var_q_nl_cur_dn5, locals.var_q_nl_cur_dn6, locals.var_q_nl_cur_dn7, locals.var_q_nl_cur_dn8, locals.var_q_nl_cur_dn9, locals.var_q_nl_cur_dn10, locals.var_q_nl_cur_dn11, locals.var_q_nl_cur_dn14,)
    }
};
        locals.var_q_nl_cur = assign34030_e39157;
        locals.var_q_nl_cur_dn0 = assign34030_e39157_d_n0;
        locals.var_q_nl_cur_dn2 = assign34030_e39157_d_n2;
        locals.var_q_nl_cur_dn4 = assign34030_e39157_d_n4;
        locals.var_q_nl_cur_dn5 = assign34030_e39157_d_n5;
        locals.var_q_nl_cur_dn6 = assign34030_e39157_d_n6;
        locals.var_q_nl_cur_dn7 = assign34030_e39157_d_n7;
        locals.var_q_nl_cur_dn8 = assign34030_e39157_d_n8;
        locals.var_q_nl_cur_dn9 = assign34030_e39157_d_n9;
        locals.var_q_nl_cur_dn10 = assign34030_e39157_d_n10;
        locals.var_q_nl_cur_dn11 = assign34030_e39157_d_n11;
        locals.var_q_nl_cur_dn14 = assign34030_e39157_d_n14;
        locals.var_q_nl_cur_rv = 0.0;

        let (assign34040_e39163, assign34040_e39163_d_n0, assign34040_e39163_d_n2, assign34040_e39163_d_n4, assign34040_e39163_d_n5, assign34040_e39163_d_n6, assign34040_e39163_d_n7, assign34040_e39163_d_n8, assign34040_e39163_d_n9, assign34040_e39163_d_n10, assign34040_e39163_d_n11, assign34040_e39163_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        (locals.var_phi_s0_dep, locals.var_phi_s0_dep_dn0, locals.var_phi_s0_dep_dn2, locals.var_phi_s0_dep_dn4, locals.var_phi_s0_dep_dn5, locals.var_phi_s0_dep_dn6, locals.var_phi_s0_dep_dn7, locals.var_phi_s0_dep_dn8, locals.var_phi_s0_dep_dn9, locals.var_phi_s0_dep_dn10, locals.var_phi_s0_dep_dn11, locals.var_phi_s0_dep_dn14,)
    } else {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn14,)
    }
};
        locals.var_ps0 = assign34040_e39163;
        locals.var_ps0_dn0 = assign34040_e39163_d_n0;
        locals.var_ps0_dn2 = assign34040_e39163_d_n2;
        locals.var_ps0_dn4 = assign34040_e39163_d_n4;
        locals.var_ps0_dn5 = assign34040_e39163_d_n5;
        locals.var_ps0_dn6 = assign34040_e39163_d_n6;
        locals.var_ps0_dn7 = assign34040_e39163_d_n7;
        locals.var_ps0_dn8 = assign34040_e39163_d_n8;
        locals.var_ps0_dn9 = assign34040_e39163_d_n9;
        locals.var_ps0_dn10 = assign34040_e39163_d_n10;
        locals.var_ps0_dn11 = assign34040_e39163_d_n11;
        locals.var_ps0_dn14 = assign34040_e39163_d_n14;
        locals.var_ps0_rv = 0.0;

        let (assign34050_e39169, assign34050_e39169_d_n0, assign34050_e39169_d_n2, assign34050_e39169_d_n4, assign34050_e39169_d_n5, assign34050_e39169_d_n6, assign34050_e39169_d_n7, assign34050_e39169_d_n8, assign34050_e39169_d_n9, assign34050_e39169_d_n10, assign34050_e39169_d_n11, assign34050_e39169_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        (locals.var_phi_sl_dep, locals.var_phi_sl_dep_dn0, locals.var_phi_sl_dep_dn2, locals.var_phi_sl_dep_dn4, locals.var_phi_sl_dep_dn5, locals.var_phi_sl_dep_dn6, locals.var_phi_sl_dep_dn7, locals.var_phi_sl_dep_dn8, locals.var_phi_sl_dep_dn9, locals.var_phi_sl_dep_dn10, locals.var_phi_sl_dep_dn11, locals.var_phi_sl_dep_dn14,)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn4, locals.var_psl_dn5, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn8, locals.var_psl_dn9, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn14,)
    }
};
        locals.var_psl = assign34050_e39169;
        locals.var_psl_dn0 = assign34050_e39169_d_n0;
        locals.var_psl_dn2 = assign34050_e39169_d_n2;
        locals.var_psl_dn4 = assign34050_e39169_d_n4;
        locals.var_psl_dn5 = assign34050_e39169_d_n5;
        locals.var_psl_dn6 = assign34050_e39169_d_n6;
        locals.var_psl_dn7 = assign34050_e39169_d_n7;
        locals.var_psl_dn8 = assign34050_e39169_d_n8;
        locals.var_psl_dn9 = assign34050_e39169_d_n9;
        locals.var_psl_dn10 = assign34050_e39169_d_n10;
        locals.var_psl_dn11 = assign34050_e39169_d_n11;
        locals.var_psl_dn14 = assign34050_e39169_d_n14;
        locals.var_psl_rv = 0.0;

        let (assign34060_e39177, assign34060_e39177_d_n0, assign34060_e39177_d_n2, assign34060_e39177_d_n4, assign34060_e39177_d_n5, assign34060_e39177_d_n6, assign34060_e39177_d_n7, assign34060_e39177_d_n8, assign34060_e39177_d_n9, assign34060_e39177_d_n10, assign34060_e39177_d_n11, assign34060_e39177_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign34060_e39175: f64 = (locals.var_phi_sl_dep - locals.var_phi_s0_dep);
        (assign34060_e39175, (locals.var_phi_sl_dep_dn0 - locals.var_phi_s0_dep_dn0), (locals.var_phi_sl_dep_dn2 - locals.var_phi_s0_dep_dn2), (locals.var_phi_sl_dep_dn4 - locals.var_phi_s0_dep_dn4), (locals.var_phi_sl_dep_dn5 - locals.var_phi_s0_dep_dn5), (locals.var_phi_sl_dep_dn6 - locals.var_phi_s0_dep_dn6), (locals.var_phi_sl_dep_dn7 - locals.var_phi_s0_dep_dn7), (locals.var_phi_sl_dep_dn8 - locals.var_phi_s0_dep_dn8), (locals.var_phi_sl_dep_dn9 - locals.var_phi_s0_dep_dn9), (locals.var_phi_sl_dep_dn10 - locals.var_phi_s0_dep_dn10), (locals.var_phi_sl_dep_dn11 - locals.var_phi_s0_dep_dn11), (locals.var_phi_sl_dep_dn14 - locals.var_phi_s0_dep_dn14),)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn4, locals.var_pds_dn5, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn8, locals.var_pds_dn9, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn14,)
    }
};
        locals.var_pds = assign34060_e39177;
        locals.var_pds_dn0 = assign34060_e39177_d_n0;
        locals.var_pds_dn2 = assign34060_e39177_d_n2;
        locals.var_pds_dn4 = assign34060_e39177_d_n4;
        locals.var_pds_dn5 = assign34060_e39177_d_n5;
        locals.var_pds_dn6 = assign34060_e39177_d_n6;
        locals.var_pds_dn7 = assign34060_e39177_d_n7;
        locals.var_pds_dn8 = assign34060_e39177_d_n8;
        locals.var_pds_dn9 = assign34060_e39177_d_n9;
        locals.var_pds_dn10 = assign34060_e39177_d_n10;
        locals.var_pds_dn11 = assign34060_e39177_d_n11;
        locals.var_pds_dn14 = assign34060_e39177_d_n14;
        locals.var_pds_rv = 0.0;

        let (assign34070_e39186, assign34070_e39186_d_n0, assign34070_e39186_d_n2, assign34070_e39186_d_n4, assign34070_e39186_d_n5, assign34070_e39186_d_n6, assign34070_e39186_d_n7, assign34070_e39186_d_n8, assign34070_e39186_d_n9, assign34070_e39186_d_n10, assign34070_e39186_d_n11, assign34070_e39186_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign34070_e39183: f64 = (locals.var_q_s0 + locals.var_q_sl);
        let assign34070_e39184: f64 = (-assign34070_e39183);
        (assign34070_e39184, (-(locals.var_q_s0_dn0 + locals.var_q_sl_dn0)), (-(locals.var_q_s0_dn2 + locals.var_q_sl_dn2)), (-(locals.var_q_s0_dn4 + locals.var_q_sl_dn4)), (-(locals.var_q_s0_dn5 + locals.var_q_sl_dn5)), (-(locals.var_q_s0_dn6 + locals.var_q_sl_dn6)), (-(locals.var_q_s0_dn7 + locals.var_q_sl_dn7)), (-(locals.var_q_s0_dn8 + locals.var_q_sl_dn8)), (-(locals.var_q_s0_dn9 + locals.var_q_sl_dn9)), (-(locals.var_q_s0_dn10 + locals.var_q_sl_dn10)), (-(locals.var_q_s0_dn11 + locals.var_q_sl_dn11)), (-(locals.var_q_s0_dn14 + locals.var_q_sl_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign34070_e39186;
        locals.var_t1_dn0 = assign34070_e39186_d_n0;
        locals.var_t1_dn2 = assign34070_e39186_d_n2;
        locals.var_t1_dn4 = assign34070_e39186_d_n4;
        locals.var_t1_dn5 = assign34070_e39186_d_n5;
        locals.var_t1_dn6 = assign34070_e39186_d_n6;
        locals.var_t1_dn7 = assign34070_e39186_d_n7;
        locals.var_t1_dn8 = assign34070_e39186_d_n8;
        locals.var_t1_dn9 = assign34070_e39186_d_n9;
        locals.var_t1_dn10 = assign34070_e39186_d_n10;
        locals.var_t1_dn11 = assign34070_e39186_d_n11;
        locals.var_t1_dn14 = assign34070_e39186_d_n14;
        locals.var_t1_rv = 0.0;

        let assign34080_e39190: f64 = locals.var_qn_delta;
        let assign34080_e39195: f64 = if ((locals.var_t1 < assign34080_e39190) && (locals.var_qn_delta >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard800 = assign34080_e39195;
        locals.var_guard800_rv = 0.0;

        let (assign34090_e39207, assign34090_e39207_d_n0, assign34090_e39207_d_n2, assign34090_e39207_d_n4, assign34090_e39207_d_n5, assign34090_e39207_d_n6, assign34090_e39207_d_n7, assign34090_e39207_d_n8, assign34090_e39207_d_n9, assign34090_e39207_d_n10, assign34090_e39207_d_n11, assign34090_e39207_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard800 != 0.0)) {
        let assign34090_e39203: f64 = locals.var_qn_delta;
        let assign34090_e39205: f64 = (assign34090_e39203 - locals.var_t1);
        (assign34090_e39205, (locals.var_qn_delta_dn0 - locals.var_t1_dn0), (locals.var_qn_delta_dn2 - locals.var_t1_dn2), (locals.var_qn_delta_dn4 - locals.var_t1_dn4), (locals.var_qn_delta_dn5 - locals.var_t1_dn5), (locals.var_qn_delta_dn6 - locals.var_t1_dn6), (locals.var_qn_delta_dn7 - locals.var_t1_dn7), (locals.var_qn_delta_dn8 - locals.var_t1_dn8), (locals.var_qn_delta_dn9 - locals.var_t1_dn9), (locals.var_qn_delta_dn10 - locals.var_t1_dn10), (locals.var_qn_delta_dn11 - locals.var_t1_dn11), (locals.var_qn_delta_dn14 - locals.var_t1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign34090_e39207;
        locals.var_tmf1_dn0 = assign34090_e39207_d_n0;
        locals.var_tmf1_dn2 = assign34090_e39207_d_n2;
        locals.var_tmf1_dn4 = assign34090_e39207_d_n4;
        locals.var_tmf1_dn5 = assign34090_e39207_d_n5;
        locals.var_tmf1_dn6 = assign34090_e39207_d_n6;
        locals.var_tmf1_dn7 = assign34090_e39207_d_n7;
        locals.var_tmf1_dn8 = assign34090_e39207_d_n8;
        locals.var_tmf1_dn9 = assign34090_e39207_d_n9;
        locals.var_tmf1_dn10 = assign34090_e39207_d_n10;
        locals.var_tmf1_dn11 = assign34090_e39207_d_n11;
        locals.var_tmf1_dn14 = assign34090_e39207_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign34100_e39217, assign34100_e39217_d_n0, assign34100_e39217_d_n2, assign34100_e39217_d_n4, assign34100_e39217_d_n5, assign34100_e39217_d_n6, assign34100_e39217_d_n7, assign34100_e39217_d_n8, assign34100_e39217_d_n9, assign34100_e39217_d_n10, assign34100_e39217_d_n11, assign34100_e39217_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard800 != 0.0)) {
        let assign34100_e39215: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign34100_e39215, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign34100_e39217;
        locals.var_x2_dn0 = assign34100_e39217_d_n0;
        locals.var_x2_dn2 = assign34100_e39217_d_n2;
        locals.var_x2_dn4 = assign34100_e39217_d_n4;
        locals.var_x2_dn5 = assign34100_e39217_d_n5;
        locals.var_x2_dn6 = assign34100_e39217_d_n6;
        locals.var_x2_dn7 = assign34100_e39217_d_n7;
        locals.var_x2_dn8 = assign34100_e39217_d_n8;
        locals.var_x2_dn9 = assign34100_e39217_d_n9;
        locals.var_x2_dn10 = assign34100_e39217_d_n10;
        locals.var_x2_dn11 = assign34100_e39217_d_n11;
        locals.var_x2_dn14 = assign34100_e39217_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign34110_e39227, assign34110_e39227_d_n0, assign34110_e39227_d_n2, assign34110_e39227_d_n4, assign34110_e39227_d_n5, assign34110_e39227_d_n6, assign34110_e39227_d_n7, assign34110_e39227_d_n8, assign34110_e39227_d_n9, assign34110_e39227_d_n10, assign34110_e39227_d_n11, assign34110_e39227_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard800 != 0.0)) {
        let assign34110_e39225: f64 = (locals.var_qn_delta * locals.var_qn_delta);
        (assign34110_e39225, ((locals.var_qn_delta_dn0 * locals.var_qn_delta) + (locals.var_qn_delta * locals.var_qn_delta_dn0)), ((locals.var_qn_delta_dn2 * locals.var_qn_delta) + (locals.var_qn_delta * locals.var_qn_delta_dn2)), ((locals.var_qn_delta_dn4 * locals.var_qn_delta) + (locals.var_qn_delta * locals.var_qn_delta_dn4)), ((locals.var_qn_delta_dn5 * locals.var_qn_delta) + (locals.var_qn_delta * locals.var_qn_delta_dn5)), ((locals.var_qn_delta_dn6 * locals.var_qn_delta) + (locals.var_qn_delta * locals.var_qn_delta_dn6)), ((locals.var_qn_delta_dn7 * locals.var_qn_delta) + (locals.var_qn_delta * locals.var_qn_delta_dn7)), ((locals.var_qn_delta_dn8 * locals.var_qn_delta) + (locals.var_qn_delta * locals.var_qn_delta_dn8)), ((locals.var_qn_delta_dn9 * locals.var_qn_delta) + (locals.var_qn_delta * locals.var_qn_delta_dn9)), ((locals.var_qn_delta_dn10 * locals.var_qn_delta) + (locals.var_qn_delta * locals.var_qn_delta_dn10)), ((locals.var_qn_delta_dn11 * locals.var_qn_delta) + (locals.var_qn_delta * locals.var_qn_delta_dn11)), ((locals.var_qn_delta_dn14 * locals.var_qn_delta) + (locals.var_qn_delta * locals.var_qn_delta_dn14)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign34110_e39227;
        locals.var_xmax2_dn0 = assign34110_e39227_d_n0;
        locals.var_xmax2_dn2 = assign34110_e39227_d_n2;
        locals.var_xmax2_dn4 = assign34110_e39227_d_n4;
        locals.var_xmax2_dn5 = assign34110_e39227_d_n5;
        locals.var_xmax2_dn6 = assign34110_e39227_d_n6;
        locals.var_xmax2_dn7 = assign34110_e39227_d_n7;
        locals.var_xmax2_dn8 = assign34110_e39227_d_n8;
        locals.var_xmax2_dn9 = assign34110_e39227_d_n9;
        locals.var_xmax2_dn10 = assign34110_e39227_d_n10;
        locals.var_xmax2_dn11 = assign34110_e39227_d_n11;
        locals.var_xmax2_dn14 = assign34110_e39227_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign34120_e39235, assign34120_e39235_d_n0, assign34120_e39235_d_n2, assign34120_e39235_d_n4, assign34120_e39235_d_n5, assign34120_e39235_d_n6, assign34120_e39235_d_n7, assign34120_e39235_d_n8, assign34120_e39235_d_n9, assign34120_e39235_d_n10, assign34120_e39235_d_n11, assign34120_e39235_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard800 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign34120_e39235;
        locals.var_xp_dn0 = assign34120_e39235_d_n0;
        locals.var_xp_dn2 = assign34120_e39235_d_n2;
        locals.var_xp_dn4 = assign34120_e39235_d_n4;
        locals.var_xp_dn5 = assign34120_e39235_d_n5;
        locals.var_xp_dn6 = assign34120_e39235_d_n6;
        locals.var_xp_dn7 = assign34120_e39235_d_n7;
        locals.var_xp_dn8 = assign34120_e39235_d_n8;
        locals.var_xp_dn9 = assign34120_e39235_d_n9;
        locals.var_xp_dn10 = assign34120_e39235_d_n10;
        locals.var_xp_dn11 = assign34120_e39235_d_n11;
        locals.var_xp_dn14 = assign34120_e39235_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign34130_e39243, assign34130_e39243_d_n0, assign34130_e39243_d_n2, assign34130_e39243_d_n4, assign34130_e39243_d_n5, assign34130_e39243_d_n6, assign34130_e39243_d_n7, assign34130_e39243_d_n8, assign34130_e39243_d_n9, assign34130_e39243_d_n10, assign34130_e39243_d_n11, assign34130_e39243_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard800 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign34130_e39243;
        locals.var_xmp_dn0 = assign34130_e39243_d_n0;
        locals.var_xmp_dn2 = assign34130_e39243_d_n2;
        locals.var_xmp_dn4 = assign34130_e39243_d_n4;
        locals.var_xmp_dn5 = assign34130_e39243_d_n5;
        locals.var_xmp_dn6 = assign34130_e39243_d_n6;
        locals.var_xmp_dn7 = assign34130_e39243_d_n7;
        locals.var_xmp_dn8 = assign34130_e39243_d_n8;
        locals.var_xmp_dn9 = assign34130_e39243_d_n9;
        locals.var_xmp_dn10 = assign34130_e39243_d_n10;
        locals.var_xmp_dn11 = assign34130_e39243_d_n11;
        locals.var_xmp_dn14 = assign34130_e39243_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign34140_e39251,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard800 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign34140_e39251;
        locals.var_m0_rv = 0.0;

        let (assign34150_e39259,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard800 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign34150_e39259;
        locals.var_mm_rv = 0.0;

        let (assign34160_e39267, assign34160_e39267_d_n0, assign34160_e39267_d_n2, assign34160_e39267_d_n4, assign34160_e39267_d_n5, assign34160_e39267_d_n6, assign34160_e39267_d_n7, assign34160_e39267_d_n8, assign34160_e39267_d_n9, assign34160_e39267_d_n10, assign34160_e39267_d_n11, assign34160_e39267_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard800 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign34160_e39267;
        locals.var_arg_dn0 = assign34160_e39267_d_n0;
        locals.var_arg_dn2 = assign34160_e39267_d_n2;
        locals.var_arg_dn4 = assign34160_e39267_d_n4;
        locals.var_arg_dn5 = assign34160_e39267_d_n5;
        locals.var_arg_dn6 = assign34160_e39267_d_n6;
        locals.var_arg_dn7 = assign34160_e39267_d_n7;
        locals.var_arg_dn8 = assign34160_e39267_d_n8;
        locals.var_arg_dn9 = assign34160_e39267_d_n9;
        locals.var_arg_dn10 = assign34160_e39267_d_n10;
        locals.var_arg_dn11 = assign34160_e39267_d_n11;
        locals.var_arg_dn14 = assign34160_e39267_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign34170_e39275, assign34170_e39275_d_n0, assign34170_e39275_d_n2, assign34170_e39275_d_n4, assign34170_e39275_d_n5, assign34170_e39275_d_n6, assign34170_e39275_d_n7, assign34170_e39275_d_n8, assign34170_e39275_d_n9, assign34170_e39275_d_n10, assign34170_e39275_d_n11, assign34170_e39275_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard800 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign34170_e39275;
        locals.var_dnm_dn0 = assign34170_e39275_d_n0;
        locals.var_dnm_dn2 = assign34170_e39275_d_n2;
        locals.var_dnm_dn4 = assign34170_e39275_d_n4;
        locals.var_dnm_dn5 = assign34170_e39275_d_n5;
        locals.var_dnm_dn6 = assign34170_e39275_d_n6;
        locals.var_dnm_dn7 = assign34170_e39275_d_n7;
        locals.var_dnm_dn8 = assign34170_e39275_d_n8;
        locals.var_dnm_dn9 = assign34170_e39275_d_n9;
        locals.var_dnm_dn10 = assign34170_e39275_d_n10;
        locals.var_dnm_dn11 = assign34170_e39275_d_n11;
        locals.var_dnm_dn14 = assign34170_e39275_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign34180_e39285, assign34180_e39285_d_n0, assign34180_e39285_d_n2, assign34180_e39285_d_n4, assign34180_e39285_d_n5, assign34180_e39285_d_n6, assign34180_e39285_d_n7, assign34180_e39285_d_n8, assign34180_e39285_d_n9, assign34180_e39285_d_n10, assign34180_e39285_d_n11, assign34180_e39285_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard800 != 0.0)) {
        let assign34180_e39283: f64 = (locals.var_xp * locals.var_x2);
        (assign34180_e39283, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign34180_e39285;
        locals.var_xp_dn0 = assign34180_e39285_d_n0;
        locals.var_xp_dn2 = assign34180_e39285_d_n2;
        locals.var_xp_dn4 = assign34180_e39285_d_n4;
        locals.var_xp_dn5 = assign34180_e39285_d_n5;
        locals.var_xp_dn6 = assign34180_e39285_d_n6;
        locals.var_xp_dn7 = assign34180_e39285_d_n7;
        locals.var_xp_dn8 = assign34180_e39285_d_n8;
        locals.var_xp_dn9 = assign34180_e39285_d_n9;
        locals.var_xp_dn10 = assign34180_e39285_d_n10;
        locals.var_xp_dn11 = assign34180_e39285_d_n11;
        locals.var_xp_dn14 = assign34180_e39285_d_n14;
        locals.var_xp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_111(
        locals: &mut StampLocals,
    ) {
        let (assign34190_e39295, assign34190_e39295_d_n0, assign34190_e39295_d_n2, assign34190_e39295_d_n4, assign34190_e39295_d_n5, assign34190_e39295_d_n6, assign34190_e39295_d_n7, assign34190_e39295_d_n8, assign34190_e39295_d_n9, assign34190_e39295_d_n10, assign34190_e39295_d_n11, assign34190_e39295_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard800 != 0.0)) {
        let assign34190_e39293: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign34190_e39293, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign34190_e39295;
        locals.var_xmp_dn0 = assign34190_e39295_d_n0;
        locals.var_xmp_dn2 = assign34190_e39295_d_n2;
        locals.var_xmp_dn4 = assign34190_e39295_d_n4;
        locals.var_xmp_dn5 = assign34190_e39295_d_n5;
        locals.var_xmp_dn6 = assign34190_e39295_d_n6;
        locals.var_xmp_dn7 = assign34190_e39295_d_n7;
        locals.var_xmp_dn8 = assign34190_e39295_d_n8;
        locals.var_xmp_dn9 = assign34190_e39295_d_n9;
        locals.var_xmp_dn10 = assign34190_e39295_d_n10;
        locals.var_xmp_dn11 = assign34190_e39295_d_n11;
        locals.var_xmp_dn14 = assign34190_e39295_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign34200_e39305, assign34200_e39305_d_n0, assign34200_e39305_d_n2, assign34200_e39305_d_n4, assign34200_e39305_d_n5, assign34200_e39305_d_n6, assign34200_e39305_d_n7, assign34200_e39305_d_n8, assign34200_e39305_d_n9, assign34200_e39305_d_n10, assign34200_e39305_d_n11, assign34200_e39305_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard800 != 0.0)) {
        let assign34200_e39303: f64 = (locals.var_xp * locals.var_x2);
        (assign34200_e39303, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign34200_e39305;
        locals.var_xp_dn0 = assign34200_e39305_d_n0;
        locals.var_xp_dn2 = assign34200_e39305_d_n2;
        locals.var_xp_dn4 = assign34200_e39305_d_n4;
        locals.var_xp_dn5 = assign34200_e39305_d_n5;
        locals.var_xp_dn6 = assign34200_e39305_d_n6;
        locals.var_xp_dn7 = assign34200_e39305_d_n7;
        locals.var_xp_dn8 = assign34200_e39305_d_n8;
        locals.var_xp_dn9 = assign34200_e39305_d_n9;
        locals.var_xp_dn10 = assign34200_e39305_d_n10;
        locals.var_xp_dn11 = assign34200_e39305_d_n11;
        locals.var_xp_dn14 = assign34200_e39305_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign34210_e39315, assign34210_e39315_d_n0, assign34210_e39315_d_n2, assign34210_e39315_d_n4, assign34210_e39315_d_n5, assign34210_e39315_d_n6, assign34210_e39315_d_n7, assign34210_e39315_d_n8, assign34210_e39315_d_n9, assign34210_e39315_d_n10, assign34210_e39315_d_n11, assign34210_e39315_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard800 != 0.0)) {
        let assign34210_e39313: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign34210_e39313, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign34210_e39315;
        locals.var_xmp_dn0 = assign34210_e39315_d_n0;
        locals.var_xmp_dn2 = assign34210_e39315_d_n2;
        locals.var_xmp_dn4 = assign34210_e39315_d_n4;
        locals.var_xmp_dn5 = assign34210_e39315_d_n5;
        locals.var_xmp_dn6 = assign34210_e39315_d_n6;
        locals.var_xmp_dn7 = assign34210_e39315_d_n7;
        locals.var_xmp_dn8 = assign34210_e39315_d_n8;
        locals.var_xmp_dn9 = assign34210_e39315_d_n9;
        locals.var_xmp_dn10 = assign34210_e39315_d_n10;
        locals.var_xmp_dn11 = assign34210_e39315_d_n11;
        locals.var_xmp_dn14 = assign34210_e39315_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign34220_e39325, assign34220_e39325_d_n0, assign34220_e39325_d_n2, assign34220_e39325_d_n4, assign34220_e39325_d_n5, assign34220_e39325_d_n6, assign34220_e39325_d_n7, assign34220_e39325_d_n8, assign34220_e39325_d_n9, assign34220_e39325_d_n10, assign34220_e39325_d_n11, assign34220_e39325_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard800 != 0.0)) {
        let assign34220_e39323: f64 = (locals.var_xp + locals.var_xmp);
        (assign34220_e39323, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign34220_e39325;
        locals.var_arg_dn0 = assign34220_e39325_d_n0;
        locals.var_arg_dn2 = assign34220_e39325_d_n2;
        locals.var_arg_dn4 = assign34220_e39325_d_n4;
        locals.var_arg_dn5 = assign34220_e39325_d_n5;
        locals.var_arg_dn6 = assign34220_e39325_d_n6;
        locals.var_arg_dn7 = assign34220_e39325_d_n7;
        locals.var_arg_dn8 = assign34220_e39325_d_n8;
        locals.var_arg_dn9 = assign34220_e39325_d_n9;
        locals.var_arg_dn10 = assign34220_e39325_d_n10;
        locals.var_arg_dn11 = assign34220_e39325_d_n11;
        locals.var_arg_dn14 = assign34220_e39325_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign34230_e39333, assign34230_e39333_d_n0, assign34230_e39333_d_n2, assign34230_e39333_d_n4, assign34230_e39333_d_n5, assign34230_e39333_d_n6, assign34230_e39333_d_n7, assign34230_e39333_d_n8, assign34230_e39333_d_n9, assign34230_e39333_d_n10, assign34230_e39333_d_n11, assign34230_e39333_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard800 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign34230_e39333;
        locals.var_dnm_dn0 = assign34230_e39333_d_n0;
        locals.var_dnm_dn2 = assign34230_e39333_d_n2;
        locals.var_dnm_dn4 = assign34230_e39333_d_n4;
        locals.var_dnm_dn5 = assign34230_e39333_d_n5;
        locals.var_dnm_dn6 = assign34230_e39333_d_n6;
        locals.var_dnm_dn7 = assign34230_e39333_d_n7;
        locals.var_dnm_dn8 = assign34230_e39333_d_n8;
        locals.var_dnm_dn9 = assign34230_e39333_d_n9;
        locals.var_dnm_dn10 = assign34230_e39333_d_n10;
        locals.var_dnm_dn11 = assign34230_e39333_d_n11;
        locals.var_dnm_dn14 = assign34230_e39333_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign34240_e39348: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard801 = assign34240_e39348;
        locals.var_guard801_rv = 0.0;

        let assign34250_e39351: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard802 = assign34250_e39351;
        locals.var_guard802_rv = 0.0;

        let (assign34260_e39363,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard800 != 0.0)) && (locals.var_guard801 != 0.0)) && (locals.var_guard802 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign34260_e39363;
        locals.var_mm_rv = 0.0;

        let assign34270_e39366: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard803 = assign34270_e39366;
        locals.var_guard803_rv = 0.0;

        let (assign34280_e39381,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard800 != 0.0)) && (locals.var_guard801 != 0.0)) && (locals.var_guard802 == 0.0)) && (locals.var_guard803 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign34280_e39381;
        locals.var_mm_rv = 0.0;

        let assign34290_e39384: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard804 = assign34290_e39384;
        locals.var_guard804_rv = 0.0;

        let (assign34300_e39402,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard800 != 0.0)) && (locals.var_guard801 != 0.0)) && (locals.var_guard802 == 0.0)) && (locals.var_guard803 == 0.0)) && (locals.var_guard804 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign34300_e39402;
        locals.var_mm_rv = 0.0;

        let assign34310_e39405: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard805 = assign34310_e39405;
        locals.var_guard805_rv = 0.0;

        let (assign34320_e39426,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard800 != 0.0)) && (locals.var_guard801 != 0.0)) && (locals.var_guard802 == 0.0)) && (locals.var_guard803 == 0.0)) && (locals.var_guard804 == 0.0)) && (locals.var_guard805 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign34320_e39426;
        locals.var_mm_rv = 0.0;

        let (assign34330_e39436,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard800 != 0.0)) && (locals.var_guard801 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign34330_e39436;
        locals.var_m0_rv = 0.0;

        let mut assign34340_loop_guard: usize = 0;
        while {
            let assign34340_cond_e39447: f64 = if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard800 != 0.0)) && (locals.var_guard801 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign34340_cond_e39447 != 0.0
        } {
            assign34340_loop_guard += 1;
            assert!(assign34340_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign34340_body0_e39458, assign34340_body0_e39458_d_n0, assign34340_body0_e39458_d_n2, assign34340_body0_e39458_d_n4, assign34340_body0_e39458_d_n5, assign34340_body0_e39458_d_n6, assign34340_body0_e39458_d_n7, assign34340_body0_e39458_d_n8, assign34340_body0_e39458_d_n9, assign34340_body0_e39458_d_n10, assign34340_body0_e39458_d_n11, assign34340_body0_e39458_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard800 != 0.0)) && (locals.var_guard801 != 0.0)) {
        let assign34340_body0_e39456: f64 = (locals.var_dnm).sqrt();
        (assign34340_body0_e39456, (locals.var_dnm_dn0 / (2.0 * assign34340_body0_e39456)), (locals.var_dnm_dn2 / (2.0 * assign34340_body0_e39456)), (locals.var_dnm_dn4 / (2.0 * assign34340_body0_e39456)), (locals.var_dnm_dn5 / (2.0 * assign34340_body0_e39456)), (locals.var_dnm_dn6 / (2.0 * assign34340_body0_e39456)), (locals.var_dnm_dn7 / (2.0 * assign34340_body0_e39456)), (locals.var_dnm_dn8 / (2.0 * assign34340_body0_e39456)), (locals.var_dnm_dn9 / (2.0 * assign34340_body0_e39456)), (locals.var_dnm_dn10 / (2.0 * assign34340_body0_e39456)), (locals.var_dnm_dn11 / (2.0 * assign34340_body0_e39456)), (locals.var_dnm_dn14 / (2.0 * assign34340_body0_e39456)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign34340_body0_e39458;
            locals.var_dnm_dn0 = assign34340_body0_e39458_d_n0;
            locals.var_dnm_dn2 = assign34340_body0_e39458_d_n2;
            locals.var_dnm_dn4 = assign34340_body0_e39458_d_n4;
            locals.var_dnm_dn5 = assign34340_body0_e39458_d_n5;
            locals.var_dnm_dn6 = assign34340_body0_e39458_d_n6;
            locals.var_dnm_dn7 = assign34340_body0_e39458_d_n7;
            locals.var_dnm_dn8 = assign34340_body0_e39458_d_n8;
            locals.var_dnm_dn9 = assign34340_body0_e39458_d_n9;
            locals.var_dnm_dn10 = assign34340_body0_e39458_d_n10;
            locals.var_dnm_dn11 = assign34340_body0_e39458_d_n11;
            locals.var_dnm_dn14 = assign34340_body0_e39458_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign34340_body1_e39470,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard800 != 0.0)) && (locals.var_guard801 != 0.0)) {
        let assign34340_body1_e39468: f64 = (locals.var_m0 + 1.0);
        (assign34340_body1_e39468,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign34340_body1_e39470;
            locals.var_m0_rv = 0.0;
        }

        let (assign34350_e39492, assign34350_e39492_d_n0, assign34350_e39492_d_n2, assign34350_e39492_d_n4, assign34350_e39492_d_n5, assign34350_e39492_d_n6, assign34350_e39492_d_n7, assign34350_e39492_d_n8, assign34350_e39492_d_n9, assign34350_e39492_d_n10, assign34350_e39492_d_n11, assign34350_e39492_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard800 != 0.0)) && (locals.var_guard801 == 0.0)) {
        let (assign34350_e39490, assign34350_e39490_d_n0, assign34350_e39490_d_n2, assign34350_e39490_d_n4, assign34350_e39490_d_n5, assign34350_e39490_d_n6, assign34350_e39490_d_n7, assign34350_e39490_d_n8, assign34350_e39490_d_n9, assign34350_e39490_d_n10, assign34350_e39490_d_n11, assign34350_e39490_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign34350_e39487: f64 = (2.0 * 2.0);
                let assign34350_e39488: f64 = (1.0 / assign34350_e39487);
                let assign34350_e39489: f64 = (locals.var_dnm).powf(assign34350_e39488);
                (assign34350_e39489, if 0.0 == 0.0 && ((assign34350_e39488) as f64).is_finite() && ((assign34350_e39488) as f64).fract() == 0.0 { if assign34350_e39488 == 0.0 { 0.0 } else { (assign34350_e39488 * ((locals.var_dnm).powf(assign34350_e39488 - 1.0) * locals.var_dnm_dn0)) } } else { (assign34350_e39489 * (assign34350_e39488 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign34350_e39488) as f64).is_finite() && ((assign34350_e39488) as f64).fract() == 0.0 { if assign34350_e39488 == 0.0 { 0.0 } else { (assign34350_e39488 * ((locals.var_dnm).powf(assign34350_e39488 - 1.0) * locals.var_dnm_dn2)) } } else { (assign34350_e39489 * (assign34350_e39488 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign34350_e39488) as f64).is_finite() && ((assign34350_e39488) as f64).fract() == 0.0 { if assign34350_e39488 == 0.0 { 0.0 } else { (assign34350_e39488 * ((locals.var_dnm).powf(assign34350_e39488 - 1.0) * locals.var_dnm_dn4)) } } else { (assign34350_e39489 * (assign34350_e39488 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign34350_e39488) as f64).is_finite() && ((assign34350_e39488) as f64).fract() == 0.0 { if assign34350_e39488 == 0.0 { 0.0 } else { (assign34350_e39488 * ((locals.var_dnm).powf(assign34350_e39488 - 1.0) * locals.var_dnm_dn5)) } } else { (assign34350_e39489 * (assign34350_e39488 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign34350_e39488) as f64).is_finite() && ((assign34350_e39488) as f64).fract() == 0.0 { if assign34350_e39488 == 0.0 { 0.0 } else { (assign34350_e39488 * ((locals.var_dnm).powf(assign34350_e39488 - 1.0) * locals.var_dnm_dn6)) } } else { (assign34350_e39489 * (assign34350_e39488 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign34350_e39488) as f64).is_finite() && ((assign34350_e39488) as f64).fract() == 0.0 { if assign34350_e39488 == 0.0 { 0.0 } else { (assign34350_e39488 * ((locals.var_dnm).powf(assign34350_e39488 - 1.0) * locals.var_dnm_dn7)) } } else { (assign34350_e39489 * (assign34350_e39488 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign34350_e39488) as f64).is_finite() && ((assign34350_e39488) as f64).fract() == 0.0 { if assign34350_e39488 == 0.0 { 0.0 } else { (assign34350_e39488 * ((locals.var_dnm).powf(assign34350_e39488 - 1.0) * locals.var_dnm_dn8)) } } else { (assign34350_e39489 * (assign34350_e39488 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign34350_e39488) as f64).is_finite() && ((assign34350_e39488) as f64).fract() == 0.0 { if assign34350_e39488 == 0.0 { 0.0 } else { (assign34350_e39488 * ((locals.var_dnm).powf(assign34350_e39488 - 1.0) * locals.var_dnm_dn9)) } } else { (assign34350_e39489 * (assign34350_e39488 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign34350_e39488) as f64).is_finite() && ((assign34350_e39488) as f64).fract() == 0.0 { if assign34350_e39488 == 0.0 { 0.0 } else { (assign34350_e39488 * ((locals.var_dnm).powf(assign34350_e39488 - 1.0) * locals.var_dnm_dn10)) } } else { (assign34350_e39489 * (assign34350_e39488 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign34350_e39488) as f64).is_finite() && ((assign34350_e39488) as f64).fract() == 0.0 { if assign34350_e39488 == 0.0 { 0.0 } else { (assign34350_e39488 * ((locals.var_dnm).powf(assign34350_e39488 - 1.0) * locals.var_dnm_dn11)) } } else { (assign34350_e39489 * (assign34350_e39488 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign34350_e39488) as f64).is_finite() && ((assign34350_e39488) as f64).fract() == 0.0 { if assign34350_e39488 == 0.0 { 0.0 } else { (assign34350_e39488 * ((locals.var_dnm).powf(assign34350_e39488 - 1.0) * locals.var_dnm_dn14)) } } else { (assign34350_e39489 * (assign34350_e39488 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign34350_e39490, assign34350_e39490_d_n0, assign34350_e39490_d_n2, assign34350_e39490_d_n4, assign34350_e39490_d_n5, assign34350_e39490_d_n6, assign34350_e39490_d_n7, assign34350_e39490_d_n8, assign34350_e39490_d_n9, assign34350_e39490_d_n10, assign34350_e39490_d_n11, assign34350_e39490_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign34350_e39492;
        locals.var_dnm_dn0 = assign34350_e39492_d_n0;
        locals.var_dnm_dn2 = assign34350_e39492_d_n2;
        locals.var_dnm_dn4 = assign34350_e39492_d_n4;
        locals.var_dnm_dn5 = assign34350_e39492_d_n5;
        locals.var_dnm_dn6 = assign34350_e39492_d_n6;
        locals.var_dnm_dn7 = assign34350_e39492_d_n7;
        locals.var_dnm_dn8 = assign34350_e39492_d_n8;
        locals.var_dnm_dn9 = assign34350_e39492_d_n9;
        locals.var_dnm_dn10 = assign34350_e39492_d_n10;
        locals.var_dnm_dn11 = assign34350_e39492_d_n11;
        locals.var_dnm_dn14 = assign34350_e39492_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign34360_e39502, assign34360_e39502_d_n0, assign34360_e39502_d_n2, assign34360_e39502_d_n4, assign34360_e39502_d_n5, assign34360_e39502_d_n6, assign34360_e39502_d_n7, assign34360_e39502_d_n8, assign34360_e39502_d_n9, assign34360_e39502_d_n10, assign34360_e39502_d_n11, assign34360_e39502_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard800 != 0.0)) {
        let assign34360_e39500: f64 = (1.0 / locals.var_dnm);
        (assign34360_e39500, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign34360_e39502;
        locals.var_dnm_dn0 = assign34360_e39502_d_n0;
        locals.var_dnm_dn2 = assign34360_e39502_d_n2;
        locals.var_dnm_dn4 = assign34360_e39502_d_n4;
        locals.var_dnm_dn5 = assign34360_e39502_d_n5;
        locals.var_dnm_dn6 = assign34360_e39502_d_n6;
        locals.var_dnm_dn7 = assign34360_e39502_d_n7;
        locals.var_dnm_dn8 = assign34360_e39502_d_n8;
        locals.var_dnm_dn9 = assign34360_e39502_d_n9;
        locals.var_dnm_dn10 = assign34360_e39502_d_n10;
        locals.var_dnm_dn11 = assign34360_e39502_d_n11;
        locals.var_dnm_dn14 = assign34360_e39502_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign34370_e39514, assign34370_e39514_d_n0, assign34370_e39514_d_n2, assign34370_e39514_d_n4, assign34370_e39514_d_n5, assign34370_e39514_d_n6, assign34370_e39514_d_n7, assign34370_e39514_d_n8, assign34370_e39514_d_n9, assign34370_e39514_d_n10, assign34370_e39514_d_n11, assign34370_e39514_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard800 != 0.0)) {
        let assign34370_e39510: f64 = (locals.var_tmf1 * locals.var_qn_delta);
        let assign34370_e39512: f64 = (assign34370_e39510 * locals.var_dnm);
        (assign34370_e39512, ((((locals.var_tmf1_dn0 * locals.var_qn_delta) + (locals.var_tmf1 * locals.var_qn_delta_dn0)) * locals.var_dnm) + (assign34370_e39510 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_qn_delta) + (locals.var_tmf1 * locals.var_qn_delta_dn2)) * locals.var_dnm) + (assign34370_e39510 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_qn_delta) + (locals.var_tmf1 * locals.var_qn_delta_dn4)) * locals.var_dnm) + (assign34370_e39510 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_qn_delta) + (locals.var_tmf1 * locals.var_qn_delta_dn5)) * locals.var_dnm) + (assign34370_e39510 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_qn_delta) + (locals.var_tmf1 * locals.var_qn_delta_dn6)) * locals.var_dnm) + (assign34370_e39510 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_qn_delta) + (locals.var_tmf1 * locals.var_qn_delta_dn7)) * locals.var_dnm) + (assign34370_e39510 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_qn_delta) + (locals.var_tmf1 * locals.var_qn_delta_dn8)) * locals.var_dnm) + (assign34370_e39510 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_qn_delta) + (locals.var_tmf1 * locals.var_qn_delta_dn9)) * locals.var_dnm) + (assign34370_e39510 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_qn_delta) + (locals.var_tmf1 * locals.var_qn_delta_dn10)) * locals.var_dnm) + (assign34370_e39510 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_qn_delta) + (locals.var_tmf1 * locals.var_qn_delta_dn11)) * locals.var_dnm) + (assign34370_e39510 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * locals.var_qn_delta) + (locals.var_tmf1 * locals.var_qn_delta_dn14)) * locals.var_dnm) + (assign34370_e39510 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign34370_e39514;
        locals.var_tmf0_dn0 = assign34370_e39514_d_n0;
        locals.var_tmf0_dn2 = assign34370_e39514_d_n2;
        locals.var_tmf0_dn4 = assign34370_e39514_d_n4;
        locals.var_tmf0_dn5 = assign34370_e39514_d_n5;
        locals.var_tmf0_dn6 = assign34370_e39514_d_n6;
        locals.var_tmf0_dn7 = assign34370_e39514_d_n7;
        locals.var_tmf0_dn8 = assign34370_e39514_d_n8;
        locals.var_tmf0_dn9 = assign34370_e39514_d_n9;
        locals.var_tmf0_dn10 = assign34370_e39514_d_n10;
        locals.var_tmf0_dn11 = assign34370_e39514_d_n11;
        locals.var_tmf0_dn14 = assign34370_e39514_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign34380_e39528, assign34380_e39528_d_n0, assign34380_e39528_d_n2, assign34380_e39528_d_n4, assign34380_e39528_d_n5, assign34380_e39528_d_n6, assign34380_e39528_d_n7, assign34380_e39528_d_n8, assign34380_e39528_d_n9, assign34380_e39528_d_n10, assign34380_e39528_d_n11, assign34380_e39528_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard800 != 0.0)) {
        let assign34380_e39522: f64 = (locals.var_qn_delta * locals.var_xmp);
        let assign34380_e39524: f64 = (assign34380_e39522 * locals.var_dnm);
        let assign34380_e39526: f64 = (assign34380_e39524 / locals.var_arg);
        (assign34380_e39526, (((((((locals.var_qn_delta_dn0 * locals.var_xmp) + (locals.var_qn_delta * locals.var_xmp_dn0)) * locals.var_dnm) + (assign34380_e39522 * locals.var_dnm_dn0)) * locals.var_arg) - (assign34380_e39524 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_qn_delta_dn2 * locals.var_xmp) + (locals.var_qn_delta * locals.var_xmp_dn2)) * locals.var_dnm) + (assign34380_e39522 * locals.var_dnm_dn2)) * locals.var_arg) - (assign34380_e39524 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_qn_delta_dn4 * locals.var_xmp) + (locals.var_qn_delta * locals.var_xmp_dn4)) * locals.var_dnm) + (assign34380_e39522 * locals.var_dnm_dn4)) * locals.var_arg) - (assign34380_e39524 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_qn_delta_dn5 * locals.var_xmp) + (locals.var_qn_delta * locals.var_xmp_dn5)) * locals.var_dnm) + (assign34380_e39522 * locals.var_dnm_dn5)) * locals.var_arg) - (assign34380_e39524 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_qn_delta_dn6 * locals.var_xmp) + (locals.var_qn_delta * locals.var_xmp_dn6)) * locals.var_dnm) + (assign34380_e39522 * locals.var_dnm_dn6)) * locals.var_arg) - (assign34380_e39524 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_qn_delta_dn7 * locals.var_xmp) + (locals.var_qn_delta * locals.var_xmp_dn7)) * locals.var_dnm) + (assign34380_e39522 * locals.var_dnm_dn7)) * locals.var_arg) - (assign34380_e39524 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_qn_delta_dn8 * locals.var_xmp) + (locals.var_qn_delta * locals.var_xmp_dn8)) * locals.var_dnm) + (assign34380_e39522 * locals.var_dnm_dn8)) * locals.var_arg) - (assign34380_e39524 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_qn_delta_dn9 * locals.var_xmp) + (locals.var_qn_delta * locals.var_xmp_dn9)) * locals.var_dnm) + (assign34380_e39522 * locals.var_dnm_dn9)) * locals.var_arg) - (assign34380_e39524 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_qn_delta_dn10 * locals.var_xmp) + (locals.var_qn_delta * locals.var_xmp_dn10)) * locals.var_dnm) + (assign34380_e39522 * locals.var_dnm_dn10)) * locals.var_arg) - (assign34380_e39524 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_qn_delta_dn11 * locals.var_xmp) + (locals.var_qn_delta * locals.var_xmp_dn11)) * locals.var_dnm) + (assign34380_e39522 * locals.var_dnm_dn11)) * locals.var_arg) - (assign34380_e39524 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_qn_delta_dn14 * locals.var_xmp) + (locals.var_qn_delta * locals.var_xmp_dn14)) * locals.var_dnm) + (assign34380_e39522 * locals.var_dnm_dn14)) * locals.var_arg) - (assign34380_e39524 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign34380_e39528;
        locals.var_t0_dn0 = assign34380_e39528_d_n0;
        locals.var_t0_dn2 = assign34380_e39528_d_n2;
        locals.var_t0_dn4 = assign34380_e39528_d_n4;
        locals.var_t0_dn5 = assign34380_e39528_d_n5;
        locals.var_t0_dn6 = assign34380_e39528_d_n6;
        locals.var_t0_dn7 = assign34380_e39528_d_n7;
        locals.var_t0_dn8 = assign34380_e39528_d_n8;
        locals.var_t0_dn9 = assign34380_e39528_d_n9;
        locals.var_t0_dn10 = assign34380_e39528_d_n10;
        locals.var_t0_dn11 = assign34380_e39528_d_n11;
        locals.var_t0_dn14 = assign34380_e39528_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign34390_e39540, assign34390_e39540_d_n0, assign34390_e39540_d_n2, assign34390_e39540_d_n4, assign34390_e39540_d_n5, assign34390_e39540_d_n6, assign34390_e39540_d_n7, assign34390_e39540_d_n8, assign34390_e39540_d_n9, assign34390_e39540_d_n10, assign34390_e39540_d_n11, assign34390_e39540_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard800 != 0.0)) {
        let assign34390_e39536: f64 = locals.var_qn_delta;
        let assign34390_e39538: f64 = (assign34390_e39536 - locals.var_tmf0);
        (assign34390_e39538, (locals.var_qn_delta_dn0 - locals.var_tmf0_dn0), (locals.var_qn_delta_dn2 - locals.var_tmf0_dn2), (locals.var_qn_delta_dn4 - locals.var_tmf0_dn4), (locals.var_qn_delta_dn5 - locals.var_tmf0_dn5), (locals.var_qn_delta_dn6 - locals.var_tmf0_dn6), (locals.var_qn_delta_dn7 - locals.var_tmf0_dn7), (locals.var_qn_delta_dn8 - locals.var_tmf0_dn8), (locals.var_qn_delta_dn9 - locals.var_tmf0_dn9), (locals.var_qn_delta_dn10 - locals.var_tmf0_dn10), (locals.var_qn_delta_dn11 - locals.var_tmf0_dn11), (locals.var_qn_delta_dn14 - locals.var_tmf0_dn14),)
    } else {
        (locals.var_qn_drift, locals.var_qn_drift_dn0, locals.var_qn_drift_dn2, locals.var_qn_drift_dn4, locals.var_qn_drift_dn5, locals.var_qn_drift_dn6, locals.var_qn_drift_dn7, locals.var_qn_drift_dn8, locals.var_qn_drift_dn9, locals.var_qn_drift_dn10, locals.var_qn_drift_dn11, locals.var_qn_drift_dn14,)
    }
};
        locals.var_qn_drift = assign34390_e39540;
        locals.var_qn_drift_dn0 = assign34390_e39540_d_n0;
        locals.var_qn_drift_dn2 = assign34390_e39540_d_n2;
        locals.var_qn_drift_dn4 = assign34390_e39540_d_n4;
        locals.var_qn_drift_dn5 = assign34390_e39540_d_n5;
        locals.var_qn_drift_dn6 = assign34390_e39540_d_n6;
        locals.var_qn_drift_dn7 = assign34390_e39540_d_n7;
        locals.var_qn_drift_dn8 = assign34390_e39540_d_n8;
        locals.var_qn_drift_dn9 = assign34390_e39540_d_n9;
        locals.var_qn_drift_dn10 = assign34390_e39540_d_n10;
        locals.var_qn_drift_dn11 = assign34390_e39540_d_n11;
        locals.var_qn_drift_dn14 = assign34390_e39540_d_n14;
        locals.var_qn_drift_rv = 0.0;

        let (assign34400_e39548, assign34400_e39548_d_n0, assign34400_e39548_d_n2, assign34400_e39548_d_n4, assign34400_e39548_d_n5, assign34400_e39548_d_n6, assign34400_e39548_d_n7, assign34400_e39548_d_n8, assign34400_e39548_d_n9, assign34400_e39548_d_n10, assign34400_e39548_d_n11, assign34400_e39548_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard800 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign34400_e39548;
        locals.var_t0_dn0 = assign34400_e39548_d_n0;
        locals.var_t0_dn2 = assign34400_e39548_d_n2;
        locals.var_t0_dn4 = assign34400_e39548_d_n4;
        locals.var_t0_dn5 = assign34400_e39548_d_n5;
        locals.var_t0_dn6 = assign34400_e39548_d_n6;
        locals.var_t0_dn7 = assign34400_e39548_d_n7;
        locals.var_t0_dn8 = assign34400_e39548_d_n8;
        locals.var_t0_dn9 = assign34400_e39548_d_n9;
        locals.var_t0_dn10 = assign34400_e39548_d_n10;
        locals.var_t0_dn11 = assign34400_e39548_d_n11;
        locals.var_t0_dn14 = assign34400_e39548_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign34410_e39557, assign34410_e39557_d_n0, assign34410_e39557_d_n2, assign34410_e39557_d_n4, assign34410_e39557_d_n5, assign34410_e39557_d_n6, assign34410_e39557_d_n7, assign34410_e39557_d_n8, assign34410_e39557_d_n9, assign34410_e39557_d_n10, assign34410_e39557_d_n11, assign34410_e39557_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard800 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_qn_drift, locals.var_qn_drift_dn0, locals.var_qn_drift_dn2, locals.var_qn_drift_dn4, locals.var_qn_drift_dn5, locals.var_qn_drift_dn6, locals.var_qn_drift_dn7, locals.var_qn_drift_dn8, locals.var_qn_drift_dn9, locals.var_qn_drift_dn10, locals.var_qn_drift_dn11, locals.var_qn_drift_dn14,)
    }
};
        locals.var_qn_drift = assign34410_e39557;
        locals.var_qn_drift_dn0 = assign34410_e39557_d_n0;
        locals.var_qn_drift_dn2 = assign34410_e39557_d_n2;
        locals.var_qn_drift_dn4 = assign34410_e39557_d_n4;
        locals.var_qn_drift_dn5 = assign34410_e39557_d_n5;
        locals.var_qn_drift_dn6 = assign34410_e39557_d_n6;
        locals.var_qn_drift_dn7 = assign34410_e39557_d_n7;
        locals.var_qn_drift_dn8 = assign34410_e39557_d_n8;
        locals.var_qn_drift_dn9 = assign34410_e39557_d_n9;
        locals.var_qn_drift_dn10 = assign34410_e39557_d_n10;
        locals.var_qn_drift_dn11 = assign34410_e39557_d_n11;
        locals.var_qn_drift_dn14 = assign34410_e39557_d_n14;
        locals.var_qn_drift_rv = 0.0;

        let (assign34420_e39566, assign34420_e39566_d_n0, assign34420_e39566_d_n2, assign34420_e39566_d_n4, assign34420_e39566_d_n5, assign34420_e39566_d_n6, assign34420_e39566_d_n7, assign34420_e39566_d_n8, assign34420_e39566_d_n9, assign34420_e39566_d_n10, assign34420_e39566_d_n11, assign34420_e39566_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard800 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign34420_e39566;
        locals.var_t0_dn0 = assign34420_e39566_d_n0;
        locals.var_t0_dn2 = assign34420_e39566_d_n2;
        locals.var_t0_dn4 = assign34420_e39566_d_n4;
        locals.var_t0_dn5 = assign34420_e39566_d_n5;
        locals.var_t0_dn6 = assign34420_e39566_d_n6;
        locals.var_t0_dn7 = assign34420_e39566_d_n7;
        locals.var_t0_dn8 = assign34420_e39566_d_n8;
        locals.var_t0_dn9 = assign34420_e39566_d_n9;
        locals.var_t0_dn10 = assign34420_e39566_d_n10;
        locals.var_t0_dn11 = assign34420_e39566_d_n11;
        locals.var_t0_dn14 = assign34420_e39566_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign34430_e39578, assign34430_e39578_d_n0, assign34430_e39578_d_n2, assign34430_e39578_d_n4, assign34430_e39578_d_n5, assign34430_e39578_d_n6, assign34430_e39578_d_n7, assign34430_e39578_d_n8, assign34430_e39578_d_n9, assign34430_e39578_d_n10, assign34430_e39578_d_n11, assign34430_e39578_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign34430_e39572: f64 = (locals.var_beta * locals.var_qn_drift);
        let assign34430_e39574: f64 = (assign34430_e39572 / 2.0);
        let assign34430_e39576: f64 = (assign34430_e39574 * locals.var_pds);
        (assign34430_e39576, (((((locals.var_beta_dn0 * locals.var_qn_drift) + (locals.var_beta * locals.var_qn_drift_dn0)) / 2.0) * locals.var_pds) + (assign34430_e39574 * locals.var_pds_dn0)), (((((locals.var_beta_dn2 * locals.var_qn_drift) + (locals.var_beta * locals.var_qn_drift_dn2)) / 2.0) * locals.var_pds) + (assign34430_e39574 * locals.var_pds_dn2)), (((((locals.var_beta_dn4 * locals.var_qn_drift) + (locals.var_beta * locals.var_qn_drift_dn4)) / 2.0) * locals.var_pds) + (assign34430_e39574 * locals.var_pds_dn4)), (((((locals.var_beta_dn5 * locals.var_qn_drift) + (locals.var_beta * locals.var_qn_drift_dn5)) / 2.0) * locals.var_pds) + (assign34430_e39574 * locals.var_pds_dn5)), (((((locals.var_beta_dn6 * locals.var_qn_drift) + (locals.var_beta * locals.var_qn_drift_dn6)) / 2.0) * locals.var_pds) + (assign34430_e39574 * locals.var_pds_dn6)), (((((locals.var_beta_dn7 * locals.var_qn_drift) + (locals.var_beta * locals.var_qn_drift_dn7)) / 2.0) * locals.var_pds) + (assign34430_e39574 * locals.var_pds_dn7)), (((((locals.var_beta_dn8 * locals.var_qn_drift) + (locals.var_beta * locals.var_qn_drift_dn8)) / 2.0) * locals.var_pds) + (assign34430_e39574 * locals.var_pds_dn8)), (((((locals.var_beta_dn9 * locals.var_qn_drift) + (locals.var_beta * locals.var_qn_drift_dn9)) / 2.0) * locals.var_pds) + (assign34430_e39574 * locals.var_pds_dn9)), (((((locals.var_beta_dn10 * locals.var_qn_drift) + (locals.var_beta * locals.var_qn_drift_dn10)) / 2.0) * locals.var_pds) + (assign34430_e39574 * locals.var_pds_dn10)), (((((locals.var_beta_dn11 * locals.var_qn_drift) + (locals.var_beta * locals.var_qn_drift_dn11)) / 2.0) * locals.var_pds) + (assign34430_e39574 * locals.var_pds_dn11)), (((((locals.var_beta_dn14 * locals.var_qn_drift) + (locals.var_beta * locals.var_qn_drift_dn14)) / 2.0) * locals.var_pds) + (assign34430_e39574 * locals.var_pds_dn14)),)
    } else {
        (locals.var_idd_drift, locals.var_idd_drift_dn0, locals.var_idd_drift_dn2, locals.var_idd_drift_dn4, locals.var_idd_drift_dn5, locals.var_idd_drift_dn6, locals.var_idd_drift_dn7, locals.var_idd_drift_dn8, locals.var_idd_drift_dn9, locals.var_idd_drift_dn10, locals.var_idd_drift_dn11, locals.var_idd_drift_dn14,)
    }
};
        locals.var_idd_drift = assign34430_e39578;
        locals.var_idd_drift_dn0 = assign34430_e39578_d_n0;
        locals.var_idd_drift_dn2 = assign34430_e39578_d_n2;
        locals.var_idd_drift_dn4 = assign34430_e39578_d_n4;
        locals.var_idd_drift_dn5 = assign34430_e39578_d_n5;
        locals.var_idd_drift_dn6 = assign34430_e39578_d_n6;
        locals.var_idd_drift_dn7 = assign34430_e39578_d_n7;
        locals.var_idd_drift_dn8 = assign34430_e39578_d_n8;
        locals.var_idd_drift_dn9 = assign34430_e39578_d_n9;
        locals.var_idd_drift_dn10 = assign34430_e39578_d_n10;
        locals.var_idd_drift_dn11 = assign34430_e39578_d_n11;
        locals.var_idd_drift_dn14 = assign34430_e39578_d_n14;
        locals.var_idd_drift_rv = 0.0;

        let (assign34440_e39588, assign34440_e39588_d_n0, assign34440_e39588_d_n2, assign34440_e39588_d_n4, assign34440_e39588_d_n5, assign34440_e39588_d_n6, assign34440_e39588_d_n7, assign34440_e39588_d_n8, assign34440_e39588_d_n9, assign34440_e39588_d_n10, assign34440_e39588_d_n11, assign34440_e39588_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign34440_e39583: f64 = (-locals.var_q_nl_cur);
        let assign34440_e39585: f64 = (assign34440_e39583 + locals.var_q_n0_cur);
        let assign34440_e39586: f64 = (-assign34440_e39585);
        (assign34440_e39586, (-((-locals.var_q_nl_cur_dn0) + locals.var_q_n0_cur_dn0)), (-((-locals.var_q_nl_cur_dn2) + locals.var_q_n0_cur_dn2)), (-((-locals.var_q_nl_cur_dn4) + locals.var_q_n0_cur_dn4)), (-((-locals.var_q_nl_cur_dn5) + locals.var_q_n0_cur_dn5)), (-((-locals.var_q_nl_cur_dn6) + locals.var_q_n0_cur_dn6)), (-((-locals.var_q_nl_cur_dn7) + locals.var_q_n0_cur_dn7)), (-((-locals.var_q_nl_cur_dn8) + locals.var_q_n0_cur_dn8)), (-((-locals.var_q_nl_cur_dn9) + locals.var_q_n0_cur_dn9)), (-((-locals.var_q_nl_cur_dn10) + locals.var_q_n0_cur_dn10)), (-((-locals.var_q_nl_cur_dn11) + locals.var_q_n0_cur_dn11)), (-((-locals.var_q_nl_cur_dn14) + locals.var_q_n0_cur_dn14)),)
    } else {
        (locals.var_idd_diffu, locals.var_idd_diffu_dn0, locals.var_idd_diffu_dn2, locals.var_idd_diffu_dn4, locals.var_idd_diffu_dn5, locals.var_idd_diffu_dn6, locals.var_idd_diffu_dn7, locals.var_idd_diffu_dn8, locals.var_idd_diffu_dn9, locals.var_idd_diffu_dn10, locals.var_idd_diffu_dn11, locals.var_idd_diffu_dn14,)
    }
};
        locals.var_idd_diffu = assign34440_e39588;
        locals.var_idd_diffu_dn0 = assign34440_e39588_d_n0;
        locals.var_idd_diffu_dn2 = assign34440_e39588_d_n2;
        locals.var_idd_diffu_dn4 = assign34440_e39588_d_n4;
        locals.var_idd_diffu_dn5 = assign34440_e39588_d_n5;
        locals.var_idd_diffu_dn6 = assign34440_e39588_d_n6;
        locals.var_idd_diffu_dn7 = assign34440_e39588_d_n7;
        locals.var_idd_diffu_dn8 = assign34440_e39588_d_n8;
        locals.var_idd_diffu_dn9 = assign34440_e39588_d_n9;
        locals.var_idd_diffu_dn10 = assign34440_e39588_d_n10;
        locals.var_idd_diffu_dn11 = assign34440_e39588_d_n11;
        locals.var_idd_diffu_dn14 = assign34440_e39588_d_n14;
        locals.var_idd_diffu_rv = 0.0;

        let (assign34450_e39596, assign34450_e39596_d_n0, assign34450_e39596_d_n2, assign34450_e39596_d_n4, assign34450_e39596_d_n5, assign34450_e39596_d_n6, assign34450_e39596_d_n7, assign34450_e39596_d_n8, assign34450_e39596_d_n9, assign34450_e39596_d_n10, assign34450_e39596_d_n11, assign34450_e39596_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign34450_e39594: f64 = (locals.var_idd_drift + locals.var_idd_diffu);
        (assign34450_e39594, (locals.var_idd_drift_dn0 + locals.var_idd_diffu_dn0), (locals.var_idd_drift_dn2 + locals.var_idd_diffu_dn2), (locals.var_idd_drift_dn4 + locals.var_idd_diffu_dn4), (locals.var_idd_drift_dn5 + locals.var_idd_diffu_dn5), (locals.var_idd_drift_dn6 + locals.var_idd_diffu_dn6), (locals.var_idd_drift_dn7 + locals.var_idd_diffu_dn7), (locals.var_idd_drift_dn8 + locals.var_idd_diffu_dn8), (locals.var_idd_drift_dn9 + locals.var_idd_diffu_dn9), (locals.var_idd_drift_dn10 + locals.var_idd_diffu_dn10), (locals.var_idd_drift_dn11 + locals.var_idd_diffu_dn11), (locals.var_idd_drift_dn14 + locals.var_idd_diffu_dn14),)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn4, locals.var_idd_dn5, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn8, locals.var_idd_dn9, locals.var_idd_dn10, locals.var_idd_dn11, locals.var_idd_dn14,)
    }
};
        locals.var_idd = assign34450_e39596;
        locals.var_idd_dn0 = assign34450_e39596_d_n0;
        locals.var_idd_dn2 = assign34450_e39596_d_n2;
        locals.var_idd_dn4 = assign34450_e39596_d_n4;
        locals.var_idd_dn5 = assign34450_e39596_d_n5;
        locals.var_idd_dn6 = assign34450_e39596_d_n6;
        locals.var_idd_dn7 = assign34450_e39596_d_n7;
        locals.var_idd_dn8 = assign34450_e39596_d_n8;
        locals.var_idd_dn9 = assign34450_e39596_d_n9;
        locals.var_idd_dn10 = assign34450_e39596_d_n10;
        locals.var_idd_dn11 = assign34450_e39596_d_n11;
        locals.var_idd_dn14 = assign34450_e39596_d_n14;
        locals.var_idd_rv = 0.0;

        let (assign34460_e39603, assign34460_e39603_d_n0, assign34460_e39603_d_n2, assign34460_e39603_d_n4, assign34460_e39603_d_n5, assign34460_e39603_d_n6, assign34460_e39603_d_n7, assign34460_e39603_d_n8, assign34460_e39603_d_n9, assign34460_e39603_d_n10, assign34460_e39603_d_n11, assign34460_e39603_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign34460_e39601: f64 = (-locals.var_q_n0_cur);
        (assign34460_e39601, (-locals.var_q_n0_cur_dn0), (-locals.var_q_n0_cur_dn2), (-locals.var_q_n0_cur_dn4), (-locals.var_q_n0_cur_dn5), (-locals.var_q_n0_cur_dn6), (-locals.var_q_n0_cur_dn7), (-locals.var_q_n0_cur_dn8), (-locals.var_q_n0_cur_dn9), (-locals.var_q_n0_cur_dn10), (-locals.var_q_n0_cur_dn11), (-locals.var_q_n0_cur_dn14),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn14,)
    }
};
        locals.var_qiu = assign34460_e39603;
        locals.var_qiu_dn0 = assign34460_e39603_d_n0;
        locals.var_qiu_dn2 = assign34460_e39603_d_n2;
        locals.var_qiu_dn4 = assign34460_e39603_d_n4;
        locals.var_qiu_dn5 = assign34460_e39603_d_n5;
        locals.var_qiu_dn6 = assign34460_e39603_d_n6;
        locals.var_qiu_dn7 = assign34460_e39603_d_n7;
        locals.var_qiu_dn8 = assign34460_e39603_d_n8;
        locals.var_qiu_dn9 = assign34460_e39603_d_n9;
        locals.var_qiu_dn10 = assign34460_e39603_d_n10;
        locals.var_qiu_dn11 = assign34460_e39603_d_n11;
        locals.var_qiu_dn14 = assign34460_e39603_d_n14;
        locals.var_qiu_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_112(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign34470_e39609, assign34470_e39609_d_n0, assign34470_e39609_d_n2, assign34470_e39609_d_n4, assign34470_e39609_d_n5, assign34470_e39609_d_n6, assign34470_e39609_d_n7, assign34470_e39609_d_n8, assign34470_e39609_d_n9, assign34470_e39609_d_n10, assign34470_e39609_d_n11, assign34470_e39609_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        (locals.var_leff, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn4, locals.var_lch_dn5, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn8, locals.var_lch_dn9, locals.var_lch_dn10, locals.var_lch_dn11, locals.var_lch_dn14,)
    }
};
        locals.var_lch = assign34470_e39609;
        locals.var_lch_dn0 = assign34470_e39609_d_n0;
        locals.var_lch_dn2 = assign34470_e39609_d_n2;
        locals.var_lch_dn4 = assign34470_e39609_d_n4;
        locals.var_lch_dn5 = assign34470_e39609_d_n5;
        locals.var_lch_dn6 = assign34470_e39609_d_n6;
        locals.var_lch_dn7 = assign34470_e39609_d_n7;
        locals.var_lch_dn8 = assign34470_e39609_d_n8;
        locals.var_lch_dn9 = assign34470_e39609_d_n9;
        locals.var_lch_dn10 = assign34470_e39609_d_n10;
        locals.var_lch_dn11 = assign34470_e39609_d_n11;
        locals.var_lch_dn14 = assign34470_e39609_d_n14;
        locals.var_lch_rv = 0.0;

        let (assign34480_e39617, assign34480_e39617_d_n0, assign34480_e39617_d_n2, assign34480_e39617_d_n4, assign34480_e39617_d_n5, assign34480_e39617_d_n6, assign34480_e39617_d_n7, assign34480_e39617_d_n8, assign34480_e39617_d_n9, assign34480_e39617_d_n10, assign34480_e39617_d_n11, assign34480_e39617_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign34480_e39615: f64 = (locals.var_ninv_o_esi / 100.0);
        (assign34480_e39615, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign34480_e39617;
        locals.var_t2_dn0 = assign34480_e39617_d_n0;
        locals.var_t2_dn2 = assign34480_e39617_d_n2;
        locals.var_t2_dn4 = assign34480_e39617_d_n4;
        locals.var_t2_dn5 = assign34480_e39617_d_n5;
        locals.var_t2_dn6 = assign34480_e39617_d_n6;
        locals.var_t2_dn7 = assign34480_e39617_d_n7;
        locals.var_t2_dn8 = assign34480_e39617_d_n8;
        locals.var_t2_dn9 = assign34480_e39617_d_n9;
        locals.var_t2_dn10 = assign34480_e39617_d_n10;
        locals.var_t2_dn11 = assign34480_e39617_d_n11;
        locals.var_t2_dn14 = assign34480_e39617_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign34490_e39629, assign34490_e39629_d_n0, assign34490_e39629_d_n2, assign34490_e39629_d_n4, assign34490_e39629_d_n5, assign34490_e39629_d_n6, assign34490_e39629_d_n7, assign34490_e39629_d_n8, assign34490_e39629_d_n9, assign34490_e39629_d_n10, assign34490_e39629_d_n11, assign34490_e39629_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign34490_e39624: f64 = (locals.var_phi_sl_dep - locals.var_phi_s0_dep);
        let assign34490_e39626: f64 = (assign34490_e39624 * locals.var_ninvde);
        let assign34490_e39627: f64 = (1.0 + assign34490_e39626);
        (assign34490_e39627, (((locals.var_phi_sl_dep_dn0 - locals.var_phi_s0_dep_dn0) * locals.var_ninvde) + (assign34490_e39624 * locals.var_ninvde_dn0)), (((locals.var_phi_sl_dep_dn2 - locals.var_phi_s0_dep_dn2) * locals.var_ninvde) + (assign34490_e39624 * locals.var_ninvde_dn2)), (((locals.var_phi_sl_dep_dn4 - locals.var_phi_s0_dep_dn4) * locals.var_ninvde) + (assign34490_e39624 * locals.var_ninvde_dn4)), (((locals.var_phi_sl_dep_dn5 - locals.var_phi_s0_dep_dn5) * locals.var_ninvde) + (assign34490_e39624 * locals.var_ninvde_dn5)), (((locals.var_phi_sl_dep_dn6 - locals.var_phi_s0_dep_dn6) * locals.var_ninvde) + (assign34490_e39624 * locals.var_ninvde_dn6)), (((locals.var_phi_sl_dep_dn7 - locals.var_phi_s0_dep_dn7) * locals.var_ninvde) + (assign34490_e39624 * locals.var_ninvde_dn7)), (((locals.var_phi_sl_dep_dn8 - locals.var_phi_s0_dep_dn8) * locals.var_ninvde) + (assign34490_e39624 * locals.var_ninvde_dn8)), (((locals.var_phi_sl_dep_dn9 - locals.var_phi_s0_dep_dn9) * locals.var_ninvde) + (assign34490_e39624 * locals.var_ninvde_dn9)), (((locals.var_phi_sl_dep_dn10 - locals.var_phi_s0_dep_dn10) * locals.var_ninvde) + (assign34490_e39624 * locals.var_ninvde_dn10)), (((locals.var_phi_sl_dep_dn11 - locals.var_phi_s0_dep_dn11) * locals.var_ninvde) + (assign34490_e39624 * locals.var_ninvde_dn11)), (((locals.var_phi_sl_dep_dn14 - locals.var_phi_s0_dep_dn14) * locals.var_ninvde) + (assign34490_e39624 * locals.var_ninvde_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign34490_e39629;
        locals.var_t4_dn0 = assign34490_e39629_d_n0;
        locals.var_t4_dn2 = assign34490_e39629_d_n2;
        locals.var_t4_dn4 = assign34490_e39629_d_n4;
        locals.var_t4_dn5 = assign34490_e39629_d_n5;
        locals.var_t4_dn6 = assign34490_e39629_d_n6;
        locals.var_t4_dn7 = assign34490_e39629_d_n7;
        locals.var_t4_dn8 = assign34490_e39629_d_n8;
        locals.var_t4_dn9 = assign34490_e39629_d_n9;
        locals.var_t4_dn10 = assign34490_e39629_d_n10;
        locals.var_t4_dn11 = assign34490_e39629_d_n11;
        locals.var_t4_dn14 = assign34490_e39629_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign34500_e39637, assign34500_e39637_d_n0, assign34500_e39637_d_n2, assign34500_e39637_d_n4, assign34500_e39637_d_n5, assign34500_e39637_d_n6, assign34500_e39637_d_n7, assign34500_e39637_d_n8, assign34500_e39637_d_n9, assign34500_e39637_d_n10, assign34500_e39637_d_n11, assign34500_e39637_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign34500_e39635: f64 = (locals.var_t2 * locals.var_qiu);
        (assign34500_e39635, ((locals.var_t2_dn0 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn0)), ((locals.var_t2_dn2 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn2)), ((locals.var_t2_dn4 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn4)), ((locals.var_t2_dn5 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn5)), ((locals.var_t2_dn6 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn6)), ((locals.var_t2_dn7 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn7)), ((locals.var_t2_dn8 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn8)), ((locals.var_t2_dn9 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn9)), ((locals.var_t2_dn10 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn10)), ((locals.var_t2_dn11 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn11)), ((locals.var_t2_dn14 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign34500_e39637;
        locals.var_t5_dn0 = assign34500_e39637_d_n0;
        locals.var_t5_dn2 = assign34500_e39637_d_n2;
        locals.var_t5_dn4 = assign34500_e39637_d_n4;
        locals.var_t5_dn5 = assign34500_e39637_d_n5;
        locals.var_t5_dn6 = assign34500_e39637_d_n6;
        locals.var_t5_dn7 = assign34500_e39637_d_n7;
        locals.var_t5_dn8 = assign34500_e39637_d_n8;
        locals.var_t5_dn9 = assign34500_e39637_d_n9;
        locals.var_t5_dn10 = assign34500_e39637_d_n10;
        locals.var_t5_dn11 = assign34500_e39637_d_n11;
        locals.var_t5_dn14 = assign34500_e39637_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign34510_e39645, assign34510_e39645_d_n0, assign34510_e39645_d_n2, assign34510_e39645_d_n4, assign34510_e39645_d_n5, assign34510_e39645_d_n6, assign34510_e39645_d_n7, assign34510_e39645_d_n8, assign34510_e39645_d_n9, assign34510_e39645_d_n10, assign34510_e39645_d_n11, assign34510_e39645_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign34510_e39643: f64 = (locals.var_t5 / locals.var_t4);
        (assign34510_e39643, (((locals.var_t5_dn0 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn2 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn4 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn5 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn6 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn7 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn8 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn9 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn10 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn11 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn14 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign34510_e39645;
        locals.var_t3_dn0 = assign34510_e39645_d_n0;
        locals.var_t3_dn2 = assign34510_e39645_d_n2;
        locals.var_t3_dn4 = assign34510_e39645_d_n4;
        locals.var_t3_dn5 = assign34510_e39645_d_n5;
        locals.var_t3_dn6 = assign34510_e39645_d_n6;
        locals.var_t3_dn7 = assign34510_e39645_d_n7;
        locals.var_t3_dn8 = assign34510_e39645_d_n8;
        locals.var_t3_dn9 = assign34510_e39645_d_n9;
        locals.var_t3_dn10 = assign34510_e39645_d_n10;
        locals.var_t3_dn11 = assign34510_e39645_d_n11;
        locals.var_t3_dn14 = assign34510_e39645_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign34520_e39651, assign34520_e39651_d_n0, assign34520_e39651_d_n2, assign34520_e39651_d_n4, assign34520_e39651_d_n5, assign34520_e39651_d_n6, assign34520_e39651_d_n7, assign34520_e39651_d_n8, assign34520_e39651_d_n9, assign34520_e39651_d_n10, assign34520_e39651_d_n11, assign34520_e39651_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    } else {
        (locals.var_eeff, locals.var_eeff_dn0, locals.var_eeff_dn2, locals.var_eeff_dn4, locals.var_eeff_dn5, locals.var_eeff_dn6, locals.var_eeff_dn7, locals.var_eeff_dn8, locals.var_eeff_dn9, locals.var_eeff_dn10, locals.var_eeff_dn11, locals.var_eeff_dn14,)
    }
};
        locals.var_eeff = assign34520_e39651;
        locals.var_eeff_dn0 = assign34520_e39651_d_n0;
        locals.var_eeff_dn2 = assign34520_e39651_d_n2;
        locals.var_eeff_dn4 = assign34520_e39651_d_n4;
        locals.var_eeff_dn5 = assign34520_e39651_d_n5;
        locals.var_eeff_dn6 = assign34520_e39651_d_n6;
        locals.var_eeff_dn7 = assign34520_e39651_d_n7;
        locals.var_eeff_dn8 = assign34520_e39651_d_n8;
        locals.var_eeff_dn9 = assign34520_e39651_d_n9;
        locals.var_eeff_dn10 = assign34520_e39651_d_n10;
        locals.var_eeff_dn11 = assign34520_e39651_d_n11;
        locals.var_eeff_dn14 = assign34520_e39651_d_n14;
        locals.var_eeff_rv = 0.0;

        let (assign34530_e39666, assign34530_e39666_d_n0, assign34530_e39666_d_n2, assign34530_e39666_d_n4, assign34530_e39666_d_n5, assign34530_e39666_d_n6, assign34530_e39666_d_n7, assign34530_e39666_d_n8, assign34530_e39666_d_n9, assign34530_e39666_d_n10, assign34530_e39666_d_n11, assign34530_e39666_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let (assign34530_e39664, assign34530_e39664_d_n0, assign34530_e39664_d_n2, assign34530_e39664_d_n4, assign34530_e39664_d_n5, assign34530_e39664_d_n6, assign34530_e39664_d_n7, assign34530_e39664_d_n8, assign34530_e39664_d_n9, assign34530_e39664_d_n10, assign34530_e39664_d_n11, assign34530_e39664_d_n14,) = {
            if (locals.var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign34530_e39662: f64 = (p.p160 - 1.0);
                let assign34530_e39663: f64 = (locals.var_eeff).powf(assign34530_e39662);
                (assign34530_e39663, if 0.0 == 0.0 && ((assign34530_e39662) as f64).is_finite() && ((assign34530_e39662) as f64).fract() == 0.0 { if assign34530_e39662 == 0.0 { 0.0 } else { (assign34530_e39662 * ((locals.var_eeff).powf(assign34530_e39662 - 1.0) * locals.var_eeff_dn0)) } } else { (assign34530_e39663 * (assign34530_e39662 * (locals.var_eeff_dn0 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34530_e39662) as f64).is_finite() && ((assign34530_e39662) as f64).fract() == 0.0 { if assign34530_e39662 == 0.0 { 0.0 } else { (assign34530_e39662 * ((locals.var_eeff).powf(assign34530_e39662 - 1.0) * locals.var_eeff_dn2)) } } else { (assign34530_e39663 * (assign34530_e39662 * (locals.var_eeff_dn2 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34530_e39662) as f64).is_finite() && ((assign34530_e39662) as f64).fract() == 0.0 { if assign34530_e39662 == 0.0 { 0.0 } else { (assign34530_e39662 * ((locals.var_eeff).powf(assign34530_e39662 - 1.0) * locals.var_eeff_dn4)) } } else { (assign34530_e39663 * (assign34530_e39662 * (locals.var_eeff_dn4 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34530_e39662) as f64).is_finite() && ((assign34530_e39662) as f64).fract() == 0.0 { if assign34530_e39662 == 0.0 { 0.0 } else { (assign34530_e39662 * ((locals.var_eeff).powf(assign34530_e39662 - 1.0) * locals.var_eeff_dn5)) } } else { (assign34530_e39663 * (assign34530_e39662 * (locals.var_eeff_dn5 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34530_e39662) as f64).is_finite() && ((assign34530_e39662) as f64).fract() == 0.0 { if assign34530_e39662 == 0.0 { 0.0 } else { (assign34530_e39662 * ((locals.var_eeff).powf(assign34530_e39662 - 1.0) * locals.var_eeff_dn6)) } } else { (assign34530_e39663 * (assign34530_e39662 * (locals.var_eeff_dn6 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34530_e39662) as f64).is_finite() && ((assign34530_e39662) as f64).fract() == 0.0 { if assign34530_e39662 == 0.0 { 0.0 } else { (assign34530_e39662 * ((locals.var_eeff).powf(assign34530_e39662 - 1.0) * locals.var_eeff_dn7)) } } else { (assign34530_e39663 * (assign34530_e39662 * (locals.var_eeff_dn7 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34530_e39662) as f64).is_finite() && ((assign34530_e39662) as f64).fract() == 0.0 { if assign34530_e39662 == 0.0 { 0.0 } else { (assign34530_e39662 * ((locals.var_eeff).powf(assign34530_e39662 - 1.0) * locals.var_eeff_dn8)) } } else { (assign34530_e39663 * (assign34530_e39662 * (locals.var_eeff_dn8 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34530_e39662) as f64).is_finite() && ((assign34530_e39662) as f64).fract() == 0.0 { if assign34530_e39662 == 0.0 { 0.0 } else { (assign34530_e39662 * ((locals.var_eeff).powf(assign34530_e39662 - 1.0) * locals.var_eeff_dn9)) } } else { (assign34530_e39663 * (assign34530_e39662 * (locals.var_eeff_dn9 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34530_e39662) as f64).is_finite() && ((assign34530_e39662) as f64).fract() == 0.0 { if assign34530_e39662 == 0.0 { 0.0 } else { (assign34530_e39662 * ((locals.var_eeff).powf(assign34530_e39662 - 1.0) * locals.var_eeff_dn10)) } } else { (assign34530_e39663 * (assign34530_e39662 * (locals.var_eeff_dn10 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34530_e39662) as f64).is_finite() && ((assign34530_e39662) as f64).fract() == 0.0 { if assign34530_e39662 == 0.0 { 0.0 } else { (assign34530_e39662 * ((locals.var_eeff).powf(assign34530_e39662 - 1.0) * locals.var_eeff_dn11)) } } else { (assign34530_e39663 * (assign34530_e39662 * (locals.var_eeff_dn11 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34530_e39662) as f64).is_finite() && ((assign34530_e39662) as f64).fract() == 0.0 { if assign34530_e39662 == 0.0 { 0.0 } else { (assign34530_e39662 * ((locals.var_eeff).powf(assign34530_e39662 - 1.0) * locals.var_eeff_dn14)) } } else { (assign34530_e39663 * (assign34530_e39662 * (locals.var_eeff_dn14 / locals.var_eeff))) },)
            }
        };
        (assign34530_e39664, assign34530_e39664_d_n0, assign34530_e39664_d_n2, assign34530_e39664_d_n4, assign34530_e39664_d_n5, assign34530_e39664_d_n6, assign34530_e39664_d_n7, assign34530_e39664_d_n8, assign34530_e39664_d_n9, assign34530_e39664_d_n10, assign34530_e39664_d_n11, assign34530_e39664_d_n14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign34530_e39666;
        locals.var_t5_dn0 = assign34530_e39666_d_n0;
        locals.var_t5_dn2 = assign34530_e39666_d_n2;
        locals.var_t5_dn4 = assign34530_e39666_d_n4;
        locals.var_t5_dn5 = assign34530_e39666_d_n5;
        locals.var_t5_dn6 = assign34530_e39666_d_n6;
        locals.var_t5_dn7 = assign34530_e39666_d_n7;
        locals.var_t5_dn8 = assign34530_e39666_d_n8;
        locals.var_t5_dn9 = assign34530_e39666_d_n9;
        locals.var_t5_dn10 = assign34530_e39666_d_n10;
        locals.var_t5_dn11 = assign34530_e39666_d_n11;
        locals.var_t5_dn14 = assign34530_e39666_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign34540_e39674, assign34540_e39674_d_n0, assign34540_e39674_d_n2, assign34540_e39674_d_n4, assign34540_e39674_d_n5, assign34540_e39674_d_n6, assign34540_e39674_d_n7, assign34540_e39674_d_n8, assign34540_e39674_d_n9, assign34540_e39674_d_n10, assign34540_e39674_d_n11, assign34540_e39674_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign34540_e39672: f64 = (locals.var_t5 * locals.var_eeff);
        (assign34540_e39672, ((locals.var_t5_dn0 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn0)), ((locals.var_t5_dn2 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn2)), ((locals.var_t5_dn4 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn4)), ((locals.var_t5_dn5 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn5)), ((locals.var_t5_dn6 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn6)), ((locals.var_t5_dn7 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn7)), ((locals.var_t5_dn8 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn8)), ((locals.var_t5_dn9 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn9)), ((locals.var_t5_dn10 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn10)), ((locals.var_t5_dn11 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn11)), ((locals.var_t5_dn14 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn14)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign34540_e39674;
        locals.var_t8_dn0 = assign34540_e39674_d_n0;
        locals.var_t8_dn2 = assign34540_e39674_d_n2;
        locals.var_t8_dn4 = assign34540_e39674_d_n4;
        locals.var_t8_dn5 = assign34540_e39674_d_n5;
        locals.var_t8_dn6 = assign34540_e39674_d_n6;
        locals.var_t8_dn7 = assign34540_e39674_d_n7;
        locals.var_t8_dn8 = assign34540_e39674_d_n8;
        locals.var_t8_dn9 = assign34540_e39674_d_n9;
        locals.var_t8_dn10 = assign34540_e39674_d_n10;
        locals.var_t8_dn11 = assign34540_e39674_d_n11;
        locals.var_t8_dn14 = assign34540_e39674_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign34550_e39689, assign34550_e39689_d_n0, assign34550_e39689_d_n2, assign34550_e39689_d_n4, assign34550_e39689_d_n5, assign34550_e39689_d_n6, assign34550_e39689_d_n7, assign34550_e39689_d_n8, assign34550_e39689_d_n9, assign34550_e39689_d_n10, assign34550_e39689_d_n11, assign34550_e39689_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let (assign34550_e39687, assign34550_e39687_d_n0, assign34550_e39687_d_n2, assign34550_e39687_d_n4, assign34550_e39687_d_n5, assign34550_e39687_d_n6, assign34550_e39687_d_n7, assign34550_e39687_d_n8, assign34550_e39687_d_n9, assign34550_e39687_d_n10, assign34550_e39687_d_n11, assign34550_e39687_d_n14,) = {
            if (locals.var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign34550_e39685: f64 = (locals.var_muesr - 1.0);
                let assign34550_e39686: f64 = (locals.var_eeff).powf(assign34550_e39685);
                (assign34550_e39686, if 0.0 == 0.0 && ((assign34550_e39685) as f64).is_finite() && ((assign34550_e39685) as f64).fract() == 0.0 { if assign34550_e39685 == 0.0 { 0.0 } else { (assign34550_e39685 * ((locals.var_eeff).powf(assign34550_e39685 - 1.0) * locals.var_eeff_dn0)) } } else { (assign34550_e39686 * (assign34550_e39685 * (locals.var_eeff_dn0 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34550_e39685) as f64).is_finite() && ((assign34550_e39685) as f64).fract() == 0.0 { if assign34550_e39685 == 0.0 { 0.0 } else { (assign34550_e39685 * ((locals.var_eeff).powf(assign34550_e39685 - 1.0) * locals.var_eeff_dn2)) } } else { (assign34550_e39686 * (assign34550_e39685 * (locals.var_eeff_dn2 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34550_e39685) as f64).is_finite() && ((assign34550_e39685) as f64).fract() == 0.0 { if assign34550_e39685 == 0.0 { 0.0 } else { (assign34550_e39685 * ((locals.var_eeff).powf(assign34550_e39685 - 1.0) * locals.var_eeff_dn4)) } } else { (assign34550_e39686 * (assign34550_e39685 * (locals.var_eeff_dn4 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34550_e39685) as f64).is_finite() && ((assign34550_e39685) as f64).fract() == 0.0 { if assign34550_e39685 == 0.0 { 0.0 } else { (assign34550_e39685 * ((locals.var_eeff).powf(assign34550_e39685 - 1.0) * locals.var_eeff_dn5)) } } else { (assign34550_e39686 * (assign34550_e39685 * (locals.var_eeff_dn5 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34550_e39685) as f64).is_finite() && ((assign34550_e39685) as f64).fract() == 0.0 { if assign34550_e39685 == 0.0 { 0.0 } else { (assign34550_e39685 * ((locals.var_eeff).powf(assign34550_e39685 - 1.0) * locals.var_eeff_dn6)) } } else { (assign34550_e39686 * (assign34550_e39685 * (locals.var_eeff_dn6 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34550_e39685) as f64).is_finite() && ((assign34550_e39685) as f64).fract() == 0.0 { if assign34550_e39685 == 0.0 { 0.0 } else { (assign34550_e39685 * ((locals.var_eeff).powf(assign34550_e39685 - 1.0) * locals.var_eeff_dn7)) } } else { (assign34550_e39686 * (assign34550_e39685 * (locals.var_eeff_dn7 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34550_e39685) as f64).is_finite() && ((assign34550_e39685) as f64).fract() == 0.0 { if assign34550_e39685 == 0.0 { 0.0 } else { (assign34550_e39685 * ((locals.var_eeff).powf(assign34550_e39685 - 1.0) * locals.var_eeff_dn8)) } } else { (assign34550_e39686 * (assign34550_e39685 * (locals.var_eeff_dn8 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34550_e39685) as f64).is_finite() && ((assign34550_e39685) as f64).fract() == 0.0 { if assign34550_e39685 == 0.0 { 0.0 } else { (assign34550_e39685 * ((locals.var_eeff).powf(assign34550_e39685 - 1.0) * locals.var_eeff_dn9)) } } else { (assign34550_e39686 * (assign34550_e39685 * (locals.var_eeff_dn9 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34550_e39685) as f64).is_finite() && ((assign34550_e39685) as f64).fract() == 0.0 { if assign34550_e39685 == 0.0 { 0.0 } else { (assign34550_e39685 * ((locals.var_eeff).powf(assign34550_e39685 - 1.0) * locals.var_eeff_dn10)) } } else { (assign34550_e39686 * (assign34550_e39685 * (locals.var_eeff_dn10 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34550_e39685) as f64).is_finite() && ((assign34550_e39685) as f64).fract() == 0.0 { if assign34550_e39685 == 0.0 { 0.0 } else { (assign34550_e39685 * ((locals.var_eeff).powf(assign34550_e39685 - 1.0) * locals.var_eeff_dn11)) } } else { (assign34550_e39686 * (assign34550_e39685 * (locals.var_eeff_dn11 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34550_e39685) as f64).is_finite() && ((assign34550_e39685) as f64).fract() == 0.0 { if assign34550_e39685 == 0.0 { 0.0 } else { (assign34550_e39685 * ((locals.var_eeff).powf(assign34550_e39685 - 1.0) * locals.var_eeff_dn14)) } } else { (assign34550_e39686 * (assign34550_e39685 * (locals.var_eeff_dn14 / locals.var_eeff))) },)
            }
        };
        (assign34550_e39687, assign34550_e39687_d_n0, assign34550_e39687_d_n2, assign34550_e39687_d_n4, assign34550_e39687_d_n5, assign34550_e39687_d_n6, assign34550_e39687_d_n7, assign34550_e39687_d_n8, assign34550_e39687_d_n9, assign34550_e39687_d_n10, assign34550_e39687_d_n11, assign34550_e39687_d_n14,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign34550_e39689;
        locals.var_t7_dn0 = assign34550_e39689_d_n0;
        locals.var_t7_dn2 = assign34550_e39689_d_n2;
        locals.var_t7_dn4 = assign34550_e39689_d_n4;
        locals.var_t7_dn5 = assign34550_e39689_d_n5;
        locals.var_t7_dn6 = assign34550_e39689_d_n6;
        locals.var_t7_dn7 = assign34550_e39689_d_n7;
        locals.var_t7_dn8 = assign34550_e39689_d_n8;
        locals.var_t7_dn9 = assign34550_e39689_d_n9;
        locals.var_t7_dn10 = assign34550_e39689_d_n10;
        locals.var_t7_dn11 = assign34550_e39689_d_n11;
        locals.var_t7_dn14 = assign34550_e39689_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign34560_e39697, assign34560_e39697_d_n0, assign34560_e39697_d_n2, assign34560_e39697_d_n4, assign34560_e39697_d_n5, assign34560_e39697_d_n6, assign34560_e39697_d_n7, assign34560_e39697_d_n8, assign34560_e39697_d_n9, assign34560_e39697_d_n10, assign34560_e39697_d_n11, assign34560_e39697_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign34560_e39695: f64 = (locals.var_t7 * locals.var_eeff);
        (assign34560_e39695, ((locals.var_t7_dn0 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn0)), ((locals.var_t7_dn2 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn2)), ((locals.var_t7_dn4 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn4)), ((locals.var_t7_dn5 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn5)), ((locals.var_t7_dn6 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn6)), ((locals.var_t7_dn7 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn7)), ((locals.var_t7_dn8 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn8)), ((locals.var_t7_dn9 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn9)), ((locals.var_t7_dn10 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn10)), ((locals.var_t7_dn11 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn11)), ((locals.var_t7_dn14 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign34560_e39697;
        locals.var_t6_dn0 = assign34560_e39697_d_n0;
        locals.var_t6_dn2 = assign34560_e39697_d_n2;
        locals.var_t6_dn4 = assign34560_e39697_d_n4;
        locals.var_t6_dn5 = assign34560_e39697_d_n5;
        locals.var_t6_dn6 = assign34560_e39697_d_n6;
        locals.var_t6_dn7 = assign34560_e39697_d_n7;
        locals.var_t6_dn8 = assign34560_e39697_d_n8;
        locals.var_t6_dn9 = assign34560_e39697_d_n9;
        locals.var_t6_dn10 = assign34560_e39697_d_n10;
        locals.var_t6_dn11 = assign34560_e39697_d_n11;
        locals.var_t6_dn14 = assign34560_e39697_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign34570_e39705, assign34570_e39705_d_n0, assign34570_e39705_d_n2, assign34570_e39705_d_n4, assign34570_e39705_d_n5, assign34570_e39705_d_n6, assign34570_e39705_d_n7, assign34570_e39705_d_n8, assign34570_e39705_d_n9, assign34570_e39705_d_n10, assign34570_e39705_d_n11, assign34570_e39705_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign34570_e39703: f64 = (1.6021918e-19 * 10000.0);
        (assign34570_e39703, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign34570_e39705;
        locals.var_t9_dn0 = assign34570_e39705_d_n0;
        locals.var_t9_dn2 = assign34570_e39705_d_n2;
        locals.var_t9_dn4 = assign34570_e39705_d_n4;
        locals.var_t9_dn5 = assign34570_e39705_d_n5;
        locals.var_t9_dn6 = assign34570_e39705_d_n6;
        locals.var_t9_dn7 = assign34570_e39705_d_n7;
        locals.var_t9_dn8 = assign34570_e39705_d_n8;
        locals.var_t9_dn9 = assign34570_e39705_d_n9;
        locals.var_t9_dn10 = assign34570_e39705_d_n10;
        locals.var_t9_dn11 = assign34570_e39705_d_n11;
        locals.var_t9_dn14 = assign34570_e39705_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign34580_e39713, assign34580_e39713_d_n0, assign34580_e39713_d_n2, assign34580_e39713_d_n4, assign34580_e39713_d_n5, assign34580_e39713_d_n6, assign34580_e39713_d_n7, assign34580_e39713_d_n8, assign34580_e39713_d_n9, assign34580_e39713_d_n10, assign34580_e39713_d_n11, assign34580_e39713_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign34580_e39711: f64 = (locals.var_qiu / locals.var_t9);
        (assign34580_e39711, (((locals.var_qiu_dn0 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn0)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn2 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn2)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn4 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn4)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn5 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn5)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn6 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn6)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn7 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn7)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn8 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn8)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn9 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn9)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn10 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn10)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn11 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn11)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn14 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn14)) / (locals.var_t9 * locals.var_t9)),)
    } else {
        (locals.var_rns, locals.var_rns_dn0, locals.var_rns_dn2, locals.var_rns_dn4, locals.var_rns_dn5, locals.var_rns_dn6, locals.var_rns_dn7, locals.var_rns_dn8, locals.var_rns_dn9, locals.var_rns_dn10, locals.var_rns_dn11, locals.var_rns_dn14,)
    }
};
        locals.var_rns = assign34580_e39713;
        locals.var_rns_dn0 = assign34580_e39713_d_n0;
        locals.var_rns_dn2 = assign34580_e39713_d_n2;
        locals.var_rns_dn4 = assign34580_e39713_d_n4;
        locals.var_rns_dn5 = assign34580_e39713_d_n5;
        locals.var_rns_dn6 = assign34580_e39713_d_n6;
        locals.var_rns_dn7 = assign34580_e39713_d_n7;
        locals.var_rns_dn8 = assign34580_e39713_d_n8;
        locals.var_rns_dn9 = assign34580_e39713_d_n9;
        locals.var_rns_dn10 = assign34580_e39713_d_n10;
        locals.var_rns_dn11 = assign34580_e39713_d_n11;
        locals.var_rns_dn14 = assign34580_e39713_d_n14;
        locals.var_rns_rv = 0.0;

        let (assign34590_e39737, assign34590_e39737_d_n0, assign34590_e39737_d_n2, assign34590_e39737_d_n4, assign34590_e39737_d_n5, assign34590_e39737_d_n6, assign34590_e39737_d_n7, assign34590_e39737_d_n8, assign34590_e39737_d_n9, assign34590_e39737_d_n10, assign34590_e39737_d_n11, assign34590_e39737_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign34590_e39721: f64 = (locals.var_uc_muecb1 * locals.var_rns);
        let assign34590_e39723: f64 = (assign34590_e39721 / 100000000000.0);
        let assign34590_e39724: f64 = (locals.var_uc_muecb0 + assign34590_e39723);
        let assign34590_e39726: f64 = (assign34590_e39724 + 1e-25);
        let assign34590_e39727: f64 = (1.0 / assign34590_e39726);
        let assign34590_e39730: f64 = (locals.var_mphn0 * locals.var_t8);
        let assign34590_e39731: f64 = (assign34590_e39727 + assign34590_e39730);
        let assign34590_e39734: f64 = (locals.var_t6 / locals.var_uc_muesr1);
        let assign34590_e39735: f64 = (assign34590_e39731 + assign34590_e39734);
        (assign34590_e39735, (((-(((locals.var_uc_muecb1 * locals.var_rns_dn0) / 100000000000.0) / (assign34590_e39726 * assign34590_e39726))) + ((locals.var_mphn0_dn0 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn0))) + (locals.var_t6_dn0 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn2) / 100000000000.0) / (assign34590_e39726 * assign34590_e39726))) + ((locals.var_mphn0_dn2 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn2))) + (locals.var_t6_dn2 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn4) / 100000000000.0) / (assign34590_e39726 * assign34590_e39726))) + ((locals.var_mphn0_dn4 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn4))) + (locals.var_t6_dn4 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn5) / 100000000000.0) / (assign34590_e39726 * assign34590_e39726))) + ((locals.var_mphn0_dn5 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn5))) + (locals.var_t6_dn5 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn6) / 100000000000.0) / (assign34590_e39726 * assign34590_e39726))) + ((locals.var_mphn0_dn6 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn6))) + (locals.var_t6_dn6 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn7) / 100000000000.0) / (assign34590_e39726 * assign34590_e39726))) + ((locals.var_mphn0_dn7 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn7))) + (locals.var_t6_dn7 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn8) / 100000000000.0) / (assign34590_e39726 * assign34590_e39726))) + ((locals.var_mphn0_dn8 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn8))) + (locals.var_t6_dn8 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn9) / 100000000000.0) / (assign34590_e39726 * assign34590_e39726))) + ((locals.var_mphn0_dn9 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn9))) + (locals.var_t6_dn9 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn10) / 100000000000.0) / (assign34590_e39726 * assign34590_e39726))) + ((locals.var_mphn0_dn10 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn10))) + (locals.var_t6_dn10 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn11) / 100000000000.0) / (assign34590_e39726 * assign34590_e39726))) + ((locals.var_mphn0_dn11 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn11))) + (locals.var_t6_dn11 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn14) / 100000000000.0) / (assign34590_e39726 * assign34590_e39726))) + ((locals.var_mphn0_dn14 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn14))) + (locals.var_t6_dn14 / locals.var_uc_muesr1)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign34590_e39737;
        locals.var_t1_dn0 = assign34590_e39737_d_n0;
        locals.var_t1_dn2 = assign34590_e39737_d_n2;
        locals.var_t1_dn4 = assign34590_e39737_d_n4;
        locals.var_t1_dn5 = assign34590_e39737_d_n5;
        locals.var_t1_dn6 = assign34590_e39737_d_n6;
        locals.var_t1_dn7 = assign34590_e39737_d_n7;
        locals.var_t1_dn8 = assign34590_e39737_d_n8;
        locals.var_t1_dn9 = assign34590_e39737_d_n9;
        locals.var_t1_dn10 = assign34590_e39737_d_n10;
        locals.var_t1_dn11 = assign34590_e39737_d_n11;
        locals.var_t1_dn14 = assign34590_e39737_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign34600_e39745, assign34600_e39745_d_n0, assign34600_e39745_d_n2, assign34600_e39745_d_n4, assign34600_e39745_d_n5, assign34600_e39745_d_n6, assign34600_e39745_d_n7, assign34600_e39745_d_n8, assign34600_e39745_d_n9, assign34600_e39745_d_n10, assign34600_e39745_d_n11, assign34600_e39745_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign34600_e39743: f64 = (1.0 / locals.var_t1);
        (assign34600_e39743, (-(locals.var_t1_dn0 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn2 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn11 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn14 / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn14,)
    }
};
        locals.var_muun = assign34600_e39745;
        locals.var_muun_dn0 = assign34600_e39745_d_n0;
        locals.var_muun_dn2 = assign34600_e39745_d_n2;
        locals.var_muun_dn4 = assign34600_e39745_d_n4;
        locals.var_muun_dn5 = assign34600_e39745_d_n5;
        locals.var_muun_dn6 = assign34600_e39745_d_n6;
        locals.var_muun_dn7 = assign34600_e39745_d_n7;
        locals.var_muun_dn8 = assign34600_e39745_d_n8;
        locals.var_muun_dn9 = assign34600_e39745_d_n9;
        locals.var_muun_dn10 = assign34600_e39745_d_n10;
        locals.var_muun_dn11 = assign34600_e39745_d_n11;
        locals.var_muun_dn14 = assign34600_e39745_d_n14;
        locals.var_muun_rv = 0.0;

        let (assign34610_e39753, assign34610_e39753_d_n0, assign34610_e39753_d_n2, assign34610_e39753_d_n4, assign34610_e39753_d_n5, assign34610_e39753_d_n6, assign34610_e39753_d_n7, assign34610_e39753_d_n8, assign34610_e39753_d_n9, assign34610_e39753_d_n10, assign34610_e39753_d_n11, assign34610_e39753_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign34610_e39751: f64 = (locals.var_muun / 10000.0);
        (assign34610_e39751, (locals.var_muun_dn0 / 10000.0), (locals.var_muun_dn2 / 10000.0), (locals.var_muun_dn4 / 10000.0), (locals.var_muun_dn5 / 10000.0), (locals.var_muun_dn6 / 10000.0), (locals.var_muun_dn7 / 10000.0), (locals.var_muun_dn8 / 10000.0), (locals.var_muun_dn9 / 10000.0), (locals.var_muun_dn10 / 10000.0), (locals.var_muun_dn11 / 10000.0), (locals.var_muun_dn14 / 10000.0),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn14,)
    }
};
        locals.var_muun = assign34610_e39753;
        locals.var_muun_dn0 = assign34610_e39753_d_n0;
        locals.var_muun_dn2 = assign34610_e39753_d_n2;
        locals.var_muun_dn4 = assign34610_e39753_d_n4;
        locals.var_muun_dn5 = assign34610_e39753_d_n5;
        locals.var_muun_dn6 = assign34610_e39753_d_n6;
        locals.var_muun_dn7 = assign34610_e39753_d_n7;
        locals.var_muun_dn8 = assign34610_e39753_d_n8;
        locals.var_muun_dn9 = assign34610_e39753_d_n9;
        locals.var_muun_dn10 = assign34610_e39753_d_n10;
        locals.var_muun_dn11 = assign34610_e39753_d_n11;
        locals.var_muun_dn14 = assign34610_e39753_d_n14;
        locals.var_muun_rv = 0.0;

        let (assign34620_e39765, assign34620_e39765_d_n0, assign34620_e39765_d_n2, assign34620_e39765_d_n4, assign34620_e39765_d_n5, assign34620_e39765_d_n6, assign34620_e39765_d_n7, assign34620_e39765_d_n8, assign34620_e39765_d_n9, assign34620_e39765_d_n10, assign34620_e39765_d_n11, assign34620_e39765_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign34620_e39760: f64 = (locals.var_qiu + 1e-25);
        let assign34620_e39761: f64 = (locals.var_beta * assign34620_e39760);
        let assign34620_e39763: f64 = (assign34620_e39761 * locals.var_lch);
        (assign34620_e39763, ((((locals.var_beta_dn0 * assign34620_e39760) + (locals.var_beta * locals.var_qiu_dn0)) * locals.var_lch) + (assign34620_e39761 * locals.var_lch_dn0)), ((((locals.var_beta_dn2 * assign34620_e39760) + (locals.var_beta * locals.var_qiu_dn2)) * locals.var_lch) + (assign34620_e39761 * locals.var_lch_dn2)), ((((locals.var_beta_dn4 * assign34620_e39760) + (locals.var_beta * locals.var_qiu_dn4)) * locals.var_lch) + (assign34620_e39761 * locals.var_lch_dn4)), ((((locals.var_beta_dn5 * assign34620_e39760) + (locals.var_beta * locals.var_qiu_dn5)) * locals.var_lch) + (assign34620_e39761 * locals.var_lch_dn5)), ((((locals.var_beta_dn6 * assign34620_e39760) + (locals.var_beta * locals.var_qiu_dn6)) * locals.var_lch) + (assign34620_e39761 * locals.var_lch_dn6)), ((((locals.var_beta_dn7 * assign34620_e39760) + (locals.var_beta * locals.var_qiu_dn7)) * locals.var_lch) + (assign34620_e39761 * locals.var_lch_dn7)), ((((locals.var_beta_dn8 * assign34620_e39760) + (locals.var_beta * locals.var_qiu_dn8)) * locals.var_lch) + (assign34620_e39761 * locals.var_lch_dn8)), ((((locals.var_beta_dn9 * assign34620_e39760) + (locals.var_beta * locals.var_qiu_dn9)) * locals.var_lch) + (assign34620_e39761 * locals.var_lch_dn9)), ((((locals.var_beta_dn10 * assign34620_e39760) + (locals.var_beta * locals.var_qiu_dn10)) * locals.var_lch) + (assign34620_e39761 * locals.var_lch_dn10)), ((((locals.var_beta_dn11 * assign34620_e39760) + (locals.var_beta * locals.var_qiu_dn11)) * locals.var_lch) + (assign34620_e39761 * locals.var_lch_dn11)), ((((locals.var_beta_dn14 * assign34620_e39760) + (locals.var_beta * locals.var_qiu_dn14)) * locals.var_lch) + (assign34620_e39761 * locals.var_lch_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign34620_e39765;
        locals.var_t2_dn0 = assign34620_e39765_d_n0;
        locals.var_t2_dn2 = assign34620_e39765_d_n2;
        locals.var_t2_dn4 = assign34620_e39765_d_n4;
        locals.var_t2_dn5 = assign34620_e39765_d_n5;
        locals.var_t2_dn6 = assign34620_e39765_d_n6;
        locals.var_t2_dn7 = assign34620_e39765_d_n7;
        locals.var_t2_dn8 = assign34620_e39765_d_n8;
        locals.var_t2_dn9 = assign34620_e39765_d_n9;
        locals.var_t2_dn10 = assign34620_e39765_d_n10;
        locals.var_t2_dn11 = assign34620_e39765_d_n11;
        locals.var_t2_dn14 = assign34620_e39765_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign34630_e39773, assign34630_e39773_d_n0, assign34630_e39773_d_n2, assign34630_e39773_d_n4, assign34630_e39773_d_n5, assign34630_e39773_d_n6, assign34630_e39773_d_n7, assign34630_e39773_d_n8, assign34630_e39773_d_n9, assign34630_e39773_d_n10, assign34630_e39773_d_n11, assign34630_e39773_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign34630_e39771: f64 = (1.0 / locals.var_t2);
        (assign34630_e39771, (-(locals.var_t2_dn0 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn2 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn11 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn14 / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign34630_e39773;
        locals.var_t1_dn0 = assign34630_e39773_d_n0;
        locals.var_t1_dn2 = assign34630_e39773_d_n2;
        locals.var_t1_dn4 = assign34630_e39773_d_n4;
        locals.var_t1_dn5 = assign34630_e39773_d_n5;
        locals.var_t1_dn6 = assign34630_e39773_d_n6;
        locals.var_t1_dn7 = assign34630_e39773_d_n7;
        locals.var_t1_dn8 = assign34630_e39773_d_n8;
        locals.var_t1_dn9 = assign34630_e39773_d_n9;
        locals.var_t1_dn10 = assign34630_e39773_d_n10;
        locals.var_t1_dn11 = assign34630_e39773_d_n11;
        locals.var_t1_dn14 = assign34630_e39773_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign34640_e39781, assign34640_e39781_d_n0, assign34640_e39781_d_n2, assign34640_e39781_d_n4, assign34640_e39781_d_n5, assign34640_e39781_d_n6, assign34640_e39781_d_n7, assign34640_e39781_d_n8, assign34640_e39781_d_n9, assign34640_e39781_d_n10, assign34640_e39781_d_n11, assign34640_e39781_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign34640_e39779: f64 = (locals.var_idd * locals.var_t1);
        (assign34640_e39779, ((locals.var_idd_dn0 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn0)), ((locals.var_idd_dn2 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn2)), ((locals.var_idd_dn4 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn4)), ((locals.var_idd_dn5 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn5)), ((locals.var_idd_dn6 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn6)), ((locals.var_idd_dn7 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn7)), ((locals.var_idd_dn8 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn8)), ((locals.var_idd_dn9 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn9)), ((locals.var_idd_dn10 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn10)), ((locals.var_idd_dn11 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn11)), ((locals.var_idd_dn14 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign34640_e39781;
        locals.var_ty_dn0 = assign34640_e39781_d_n0;
        locals.var_ty_dn2 = assign34640_e39781_d_n2;
        locals.var_ty_dn4 = assign34640_e39781_d_n4;
        locals.var_ty_dn5 = assign34640_e39781_d_n5;
        locals.var_ty_dn6 = assign34640_e39781_d_n6;
        locals.var_ty_dn7 = assign34640_e39781_d_n7;
        locals.var_ty_dn8 = assign34640_e39781_d_n8;
        locals.var_ty_dn9 = assign34640_e39781_d_n9;
        locals.var_ty_dn10 = assign34640_e39781_d_n10;
        locals.var_ty_dn11 = assign34640_e39781_d_n11;
        locals.var_ty_dn14 = assign34640_e39781_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign34650_e39791, assign34650_e39791_d_n0, assign34650_e39791_d_n2, assign34650_e39791_d_n4, assign34650_e39791_d_n5, assign34650_e39791_d_n6, assign34650_e39791_d_n7, assign34650_e39791_d_n8, assign34650_e39791_d_n9, assign34650_e39791_d_n10, assign34650_e39791_d_n11, assign34650_e39791_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign34650_e39787: f64 = (0.2 * locals.var_vmaxe);
        let assign34650_e39789: f64 = (assign34650_e39787 / locals.var_muun);
        (assign34650_e39789, ((((0.2 * locals.var_vmaxe_dn0) * locals.var_muun) - (assign34650_e39787 * locals.var_muun_dn0)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn2) * locals.var_muun) - (assign34650_e39787 * locals.var_muun_dn2)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn4) * locals.var_muun) - (assign34650_e39787 * locals.var_muun_dn4)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn5) * locals.var_muun) - (assign34650_e39787 * locals.var_muun_dn5)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn6) * locals.var_muun) - (assign34650_e39787 * locals.var_muun_dn6)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn7) * locals.var_muun) - (assign34650_e39787 * locals.var_muun_dn7)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn8) * locals.var_muun) - (assign34650_e39787 * locals.var_muun_dn8)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn9) * locals.var_muun) - (assign34650_e39787 * locals.var_muun_dn9)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn10) * locals.var_muun) - (assign34650_e39787 * locals.var_muun_dn10)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn11) * locals.var_muun) - (assign34650_e39787 * locals.var_muun_dn11)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn14) * locals.var_muun) - (assign34650_e39787 * locals.var_muun_dn14)) / (locals.var_muun * locals.var_muun)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign34650_e39791;
        locals.var_t2_dn0 = assign34650_e39791_d_n0;
        locals.var_t2_dn2 = assign34650_e39791_d_n2;
        locals.var_t2_dn4 = assign34650_e39791_d_n4;
        locals.var_t2_dn5 = assign34650_e39791_d_n5;
        locals.var_t2_dn6 = assign34650_e39791_d_n6;
        locals.var_t2_dn7 = assign34650_e39791_d_n7;
        locals.var_t2_dn8 = assign34650_e39791_d_n8;
        locals.var_t2_dn9 = assign34650_e39791_d_n9;
        locals.var_t2_dn10 = assign34650_e39791_d_n10;
        locals.var_t2_dn11 = assign34650_e39791_d_n11;
        locals.var_t2_dn14 = assign34650_e39791_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign34660_e39804, assign34660_e39804_d_n0, assign34660_e39804_d_n2, assign34660_e39804_d_n4, assign34660_e39804_d_n5, assign34660_e39804_d_n6, assign34660_e39804_d_n7, assign34660_e39804_d_n8, assign34660_e39804_d_n9, assign34660_e39804_d_n10, assign34660_e39804_d_n11, assign34660_e39804_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign34660_e39797: f64 = (locals.var_ty * locals.var_ty);
        let assign34660_e39800: f64 = (locals.var_t2 * locals.var_t2);
        let assign34660_e39801: f64 = (assign34660_e39797 + assign34660_e39800);
        let assign34660_e39802: f64 = (assign34660_e39801).sqrt();
        (assign34660_e39802, ((((locals.var_ty_dn0 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn0)) + ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0))) / (2.0 * assign34660_e39802)), ((((locals.var_ty_dn2 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn2)) + ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2))) / (2.0 * assign34660_e39802)), ((((locals.var_ty_dn4 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn4)) + ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4))) / (2.0 * assign34660_e39802)), ((((locals.var_ty_dn5 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn5)) + ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5))) / (2.0 * assign34660_e39802)), ((((locals.var_ty_dn6 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn6)) + ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6))) / (2.0 * assign34660_e39802)), ((((locals.var_ty_dn7 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn7)) + ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7))) / (2.0 * assign34660_e39802)), ((((locals.var_ty_dn8 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn8)) + ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8))) / (2.0 * assign34660_e39802)), ((((locals.var_ty_dn9 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn9)) + ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9))) / (2.0 * assign34660_e39802)), ((((locals.var_ty_dn10 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn10)) + ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10))) / (2.0 * assign34660_e39802)), ((((locals.var_ty_dn11 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn11)) + ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11))) / (2.0 * assign34660_e39802)), ((((locals.var_ty_dn14 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn14)) + ((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14))) / (2.0 * assign34660_e39802)),)
    } else {
        (locals.var_ey, locals.var_ey_dn0, locals.var_ey_dn2, locals.var_ey_dn4, locals.var_ey_dn5, locals.var_ey_dn6, locals.var_ey_dn7, locals.var_ey_dn8, locals.var_ey_dn9, locals.var_ey_dn10, locals.var_ey_dn11, locals.var_ey_dn14,)
    }
};
        locals.var_ey = assign34660_e39804;
        locals.var_ey_dn0 = assign34660_e39804_d_n0;
        locals.var_ey_dn2 = assign34660_e39804_d_n2;
        locals.var_ey_dn4 = assign34660_e39804_d_n4;
        locals.var_ey_dn5 = assign34660_e39804_d_n5;
        locals.var_ey_dn6 = assign34660_e39804_d_n6;
        locals.var_ey_dn7 = assign34660_e39804_d_n7;
        locals.var_ey_dn8 = assign34660_e39804_d_n8;
        locals.var_ey_dn9 = assign34660_e39804_d_n9;
        locals.var_ey_dn10 = assign34660_e39804_d_n10;
        locals.var_ey_dn11 = assign34660_e39804_d_n11;
        locals.var_ey_dn14 = assign34660_e39804_d_n14;
        locals.var_ey_rv = 0.0;

        let (assign34670_e39812, assign34670_e39812_d_n0, assign34670_e39812_d_n2, assign34670_e39812_d_n4, assign34670_e39812_d_n5, assign34670_e39812_d_n6, assign34670_e39812_d_n7, assign34670_e39812_d_n8, assign34670_e39812_d_n9, assign34670_e39812_d_n10, assign34670_e39812_d_n11, assign34670_e39812_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign34670_e39810: f64 = (1.0 / locals.var_ey);
        (assign34670_e39810, (-(locals.var_ey_dn0 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn2 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn4 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn5 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn6 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn7 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn8 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn9 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn10 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn11 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn14 / (locals.var_ey * locals.var_ey))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign34670_e39812;
        locals.var_t4_dn0 = assign34670_e39812_d_n0;
        locals.var_t4_dn2 = assign34670_e39812_d_n2;
        locals.var_t4_dn4 = assign34670_e39812_d_n4;
        locals.var_t4_dn5 = assign34670_e39812_d_n5;
        locals.var_t4_dn6 = assign34670_e39812_d_n6;
        locals.var_t4_dn7 = assign34670_e39812_d_n7;
        locals.var_t4_dn8 = assign34670_e39812_d_n8;
        locals.var_t4_dn9 = assign34670_e39812_d_n9;
        locals.var_t4_dn10 = assign34670_e39812_d_n10;
        locals.var_t4_dn11 = assign34670_e39812_d_n11;
        locals.var_t4_dn14 = assign34670_e39812_d_n14;
        locals.var_t4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_113(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign34680_e39820, assign34680_e39820_d_n0, assign34680_e39820_d_n2, assign34680_e39820_d_n4, assign34680_e39820_d_n5, assign34680_e39820_d_n6, assign34680_e39820_d_n7, assign34680_e39820_d_n8, assign34680_e39820_d_n9, assign34680_e39820_d_n10, assign34680_e39820_d_n11, assign34680_e39820_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign34680_e39818: f64 = (locals.var_muun * locals.var_ey);
        (assign34680_e39818, ((locals.var_muun_dn0 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn0)), ((locals.var_muun_dn2 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn2)), ((locals.var_muun_dn4 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn4)), ((locals.var_muun_dn5 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn5)), ((locals.var_muun_dn6 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn6)), ((locals.var_muun_dn7 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn7)), ((locals.var_muun_dn8 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn8)), ((locals.var_muun_dn9 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn9)), ((locals.var_muun_dn10 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn10)), ((locals.var_muun_dn11 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn11)), ((locals.var_muun_dn14 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn14)),)
    } else {
        (locals.var_em, locals.var_em_dn0, locals.var_em_dn2, locals.var_em_dn4, locals.var_em_dn5, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, locals.var_em_dn9, locals.var_em_dn10, locals.var_em_dn11, locals.var_em_dn14,)
    }
};
        locals.var_em = assign34680_e39820;
        locals.var_em_dn0 = assign34680_e39820_d_n0;
        locals.var_em_dn2 = assign34680_e39820_d_n2;
        locals.var_em_dn4 = assign34680_e39820_d_n4;
        locals.var_em_dn5 = assign34680_e39820_d_n5;
        locals.var_em_dn6 = assign34680_e39820_d_n6;
        locals.var_em_dn7 = assign34680_e39820_d_n7;
        locals.var_em_dn8 = assign34680_e39820_d_n8;
        locals.var_em_dn9 = assign34680_e39820_d_n9;
        locals.var_em_dn10 = assign34680_e39820_d_n10;
        locals.var_em_dn11 = assign34680_e39820_d_n11;
        locals.var_em_dn14 = assign34680_e39820_d_n14;
        locals.var_em_rv = 0.0;

        let (assign34690_e39828, assign34690_e39828_d_n0, assign34690_e39828_d_n2, assign34690_e39828_d_n4, assign34690_e39828_d_n5, assign34690_e39828_d_n6, assign34690_e39828_d_n7, assign34690_e39828_d_n8, assign34690_e39828_d_n9, assign34690_e39828_d_n10, assign34690_e39828_d_n11, assign34690_e39828_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign34690_e39826: f64 = (locals.var_em / locals.var_vmaxe);
        (assign34690_e39826, (((locals.var_em_dn0 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn0)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn2 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn2)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn4 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn4)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn5 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn5)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn6 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn6)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn7 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn7)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn8 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn8)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn9 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn9)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn10 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn10)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn11 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn11)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn14 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn14)) / (locals.var_vmaxe * locals.var_vmaxe)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign34690_e39828;
        locals.var_t1_dn0 = assign34690_e39828_d_n0;
        locals.var_t1_dn2 = assign34690_e39828_d_n2;
        locals.var_t1_dn4 = assign34690_e39828_d_n4;
        locals.var_t1_dn5 = assign34690_e39828_d_n5;
        locals.var_t1_dn6 = assign34690_e39828_d_n6;
        locals.var_t1_dn7 = assign34690_e39828_d_n7;
        locals.var_t1_dn8 = assign34690_e39828_d_n8;
        locals.var_t1_dn9 = assign34690_e39828_d_n9;
        locals.var_t1_dn10 = assign34690_e39828_d_n10;
        locals.var_t1_dn11 = assign34690_e39828_d_n11;
        locals.var_t1_dn14 = assign34690_e39828_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign34700_e39834, assign34700_e39834_d_n0, assign34700_e39834_d_n2, assign34700_e39834_d_n4, assign34700_e39834_d_n5, assign34700_e39834_d_n6, assign34700_e39834_d_n7, assign34700_e39834_d_n8, assign34700_e39834_d_n9, assign34700_e39834_d_n10, assign34700_e39834_d_n11, assign34700_e39834_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        (locals.var_ey, locals.var_ey_dn0, locals.var_ey_dn2, locals.var_ey_dn4, locals.var_ey_dn5, locals.var_ey_dn6, locals.var_ey_dn7, locals.var_ey_dn8, locals.var_ey_dn9, locals.var_ey_dn10, locals.var_ey_dn11, locals.var_ey_dn14,)
    } else {
        (locals.var_ey_suf, locals.var_ey_suf_dn0, locals.var_ey_suf_dn2, locals.var_ey_suf_dn4, locals.var_ey_suf_dn5, locals.var_ey_suf_dn6, locals.var_ey_suf_dn7, locals.var_ey_suf_dn8, locals.var_ey_suf_dn9, locals.var_ey_suf_dn10, locals.var_ey_suf_dn11, locals.var_ey_suf_dn14,)
    }
};
        locals.var_ey_suf = assign34700_e39834;
        locals.var_ey_suf_dn0 = assign34700_e39834_d_n0;
        locals.var_ey_suf_dn2 = assign34700_e39834_d_n2;
        locals.var_ey_suf_dn4 = assign34700_e39834_d_n4;
        locals.var_ey_suf_dn5 = assign34700_e39834_d_n5;
        locals.var_ey_suf_dn6 = assign34700_e39834_d_n6;
        locals.var_ey_suf_dn7 = assign34700_e39834_d_n7;
        locals.var_ey_suf_dn8 = assign34700_e39834_d_n8;
        locals.var_ey_suf_dn9 = assign34700_e39834_d_n9;
        locals.var_ey_suf_dn10 = assign34700_e39834_d_n10;
        locals.var_ey_suf_dn11 = assign34700_e39834_d_n11;
        locals.var_ey_suf_dn14 = assign34700_e39834_d_n14;
        locals.var_ey_suf_rv = 0.0;

        let assign34710_e39838: f64 = (10.0 * 2.220446049250313e-16);
        let assign34710_e39839: f64 = (1.0 - assign34710_e39838);
        let assign34710_e39846: f64 = (10.0 * 2.220446049250313e-16);
        let assign34710_e39847: f64 = (1.0 + assign34710_e39846);
        let assign34710_e39849: f64 = if ((assign34710_e39839 <= p.p178) && (p.p178 <= assign34710_e39847)) { 1.0 } else { 0.0 };
        locals.var_guard806 = assign34710_e39849;
        locals.var_guard806_rv = 0.0;

        let (assign34720_e39857, assign34720_e39857_d_n0, assign34720_e39857_d_n2, assign34720_e39857_d_n4, assign34720_e39857_d_n5, assign34720_e39857_d_n6, assign34720_e39857_d_n7, assign34720_e39857_d_n8, assign34720_e39857_d_n9, assign34720_e39857_d_n10, assign34720_e39857_d_n11, assign34720_e39857_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard806 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign34720_e39857;
        locals.var_t3_dn0 = assign34720_e39857_d_n0;
        locals.var_t3_dn2 = assign34720_e39857_d_n2;
        locals.var_t3_dn4 = assign34720_e39857_d_n4;
        locals.var_t3_dn5 = assign34720_e39857_d_n5;
        locals.var_t3_dn6 = assign34720_e39857_d_n6;
        locals.var_t3_dn7 = assign34720_e39857_d_n7;
        locals.var_t3_dn8 = assign34720_e39857_d_n8;
        locals.var_t3_dn9 = assign34720_e39857_d_n9;
        locals.var_t3_dn10 = assign34720_e39857_d_n10;
        locals.var_t3_dn11 = assign34720_e39857_d_n11;
        locals.var_t3_dn14 = assign34720_e39857_d_n14;
        locals.var_t3_rv = 0.0;

        let assign34730_e39861: f64 = (10.0 * 2.220446049250313e-16);
        let assign34730_e39862: f64 = (2.0 - assign34730_e39861);
        let assign34730_e39869: f64 = (10.0 * 2.220446049250313e-16);
        let assign34730_e39870: f64 = (2.0 + assign34730_e39869);
        let assign34730_e39872: f64 = if ((assign34730_e39862 <= p.p178) && (p.p178 <= assign34730_e39870)) { 1.0 } else { 0.0 };
        locals.var_guard807 = assign34730_e39872;
        locals.var_guard807_rv = 0.0;

        let (assign34740_e39883, assign34740_e39883_d_n0, assign34740_e39883_d_n2, assign34740_e39883_d_n4, assign34740_e39883_d_n5, assign34740_e39883_d_n6, assign34740_e39883_d_n7, assign34740_e39883_d_n8, assign34740_e39883_d_n9, assign34740_e39883_d_n10, assign34740_e39883_d_n11, assign34740_e39883_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard806 == 0.0)) && (locals.var_guard807 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign34740_e39883;
        locals.var_t3_dn0 = assign34740_e39883_d_n0;
        locals.var_t3_dn2 = assign34740_e39883_d_n2;
        locals.var_t3_dn4 = assign34740_e39883_d_n4;
        locals.var_t3_dn5 = assign34740_e39883_d_n5;
        locals.var_t3_dn6 = assign34740_e39883_d_n6;
        locals.var_t3_dn7 = assign34740_e39883_d_n7;
        locals.var_t3_dn8 = assign34740_e39883_d_n8;
        locals.var_t3_dn9 = assign34740_e39883_d_n9;
        locals.var_t3_dn10 = assign34740_e39883_d_n10;
        locals.var_t3_dn11 = assign34740_e39883_d_n11;
        locals.var_t3_dn14 = assign34740_e39883_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign34750_e39904, assign34750_e39904_d_n0, assign34750_e39904_d_n2, assign34750_e39904_d_n4, assign34750_e39904_d_n5, assign34750_e39904_d_n6, assign34750_e39904_d_n7, assign34750_e39904_d_n8, assign34750_e39904_d_n9, assign34750_e39904_d_n10, assign34750_e39904_d_n11, assign34750_e39904_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard806 == 0.0)) && (locals.var_guard807 == 0.0)) {
        let (assign34750_e39902, assign34750_e39902_d_n0, assign34750_e39902_d_n2, assign34750_e39902_d_n4, assign34750_e39902_d_n5, assign34750_e39902_d_n6, assign34750_e39902_d_n7, assign34750_e39902_d_n8, assign34750_e39902_d_n9, assign34750_e39902_d_n10, assign34750_e39902_d_n11, assign34750_e39902_d_n14,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign34750_e39900: f64 = (p.p178 - 1.0);
                let assign34750_e39901: f64 = (locals.var_t1).powf(assign34750_e39900);
                (assign34750_e39901, if 0.0 == 0.0 && ((assign34750_e39900) as f64).is_finite() && ((assign34750_e39900) as f64).fract() == 0.0 { if assign34750_e39900 == 0.0 { 0.0 } else { (assign34750_e39900 * ((locals.var_t1).powf(assign34750_e39900 - 1.0) * locals.var_t1_dn0)) } } else { (assign34750_e39901 * (assign34750_e39900 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign34750_e39900) as f64).is_finite() && ((assign34750_e39900) as f64).fract() == 0.0 { if assign34750_e39900 == 0.0 { 0.0 } else { (assign34750_e39900 * ((locals.var_t1).powf(assign34750_e39900 - 1.0) * locals.var_t1_dn2)) } } else { (assign34750_e39901 * (assign34750_e39900 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign34750_e39900) as f64).is_finite() && ((assign34750_e39900) as f64).fract() == 0.0 { if assign34750_e39900 == 0.0 { 0.0 } else { (assign34750_e39900 * ((locals.var_t1).powf(assign34750_e39900 - 1.0) * locals.var_t1_dn4)) } } else { (assign34750_e39901 * (assign34750_e39900 * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign34750_e39900) as f64).is_finite() && ((assign34750_e39900) as f64).fract() == 0.0 { if assign34750_e39900 == 0.0 { 0.0 } else { (assign34750_e39900 * ((locals.var_t1).powf(assign34750_e39900 - 1.0) * locals.var_t1_dn5)) } } else { (assign34750_e39901 * (assign34750_e39900 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign34750_e39900) as f64).is_finite() && ((assign34750_e39900) as f64).fract() == 0.0 { if assign34750_e39900 == 0.0 { 0.0 } else { (assign34750_e39900 * ((locals.var_t1).powf(assign34750_e39900 - 1.0) * locals.var_t1_dn6)) } } else { (assign34750_e39901 * (assign34750_e39900 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign34750_e39900) as f64).is_finite() && ((assign34750_e39900) as f64).fract() == 0.0 { if assign34750_e39900 == 0.0 { 0.0 } else { (assign34750_e39900 * ((locals.var_t1).powf(assign34750_e39900 - 1.0) * locals.var_t1_dn7)) } } else { (assign34750_e39901 * (assign34750_e39900 * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign34750_e39900) as f64).is_finite() && ((assign34750_e39900) as f64).fract() == 0.0 { if assign34750_e39900 == 0.0 { 0.0 } else { (assign34750_e39900 * ((locals.var_t1).powf(assign34750_e39900 - 1.0) * locals.var_t1_dn8)) } } else { (assign34750_e39901 * (assign34750_e39900 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign34750_e39900) as f64).is_finite() && ((assign34750_e39900) as f64).fract() == 0.0 { if assign34750_e39900 == 0.0 { 0.0 } else { (assign34750_e39900 * ((locals.var_t1).powf(assign34750_e39900 - 1.0) * locals.var_t1_dn9)) } } else { (assign34750_e39901 * (assign34750_e39900 * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign34750_e39900) as f64).is_finite() && ((assign34750_e39900) as f64).fract() == 0.0 { if assign34750_e39900 == 0.0 { 0.0 } else { (assign34750_e39900 * ((locals.var_t1).powf(assign34750_e39900 - 1.0) * locals.var_t1_dn10)) } } else { (assign34750_e39901 * (assign34750_e39900 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign34750_e39900) as f64).is_finite() && ((assign34750_e39900) as f64).fract() == 0.0 { if assign34750_e39900 == 0.0 { 0.0 } else { (assign34750_e39900 * ((locals.var_t1).powf(assign34750_e39900 - 1.0) * locals.var_t1_dn11)) } } else { (assign34750_e39901 * (assign34750_e39900 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign34750_e39900) as f64).is_finite() && ((assign34750_e39900) as f64).fract() == 0.0 { if assign34750_e39900 == 0.0 { 0.0 } else { (assign34750_e39900 * ((locals.var_t1).powf(assign34750_e39900 - 1.0) * locals.var_t1_dn14)) } } else { (assign34750_e39901 * (assign34750_e39900 * (locals.var_t1_dn14 / locals.var_t1))) },)
            }
        };
        (assign34750_e39902, assign34750_e39902_d_n0, assign34750_e39902_d_n2, assign34750_e39902_d_n4, assign34750_e39902_d_n5, assign34750_e39902_d_n6, assign34750_e39902_d_n7, assign34750_e39902_d_n8, assign34750_e39902_d_n9, assign34750_e39902_d_n10, assign34750_e39902_d_n11, assign34750_e39902_d_n14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign34750_e39904;
        locals.var_t3_dn0 = assign34750_e39904_d_n0;
        locals.var_t3_dn2 = assign34750_e39904_d_n2;
        locals.var_t3_dn4 = assign34750_e39904_d_n4;
        locals.var_t3_dn5 = assign34750_e39904_d_n5;
        locals.var_t3_dn6 = assign34750_e39904_d_n6;
        locals.var_t3_dn7 = assign34750_e39904_d_n7;
        locals.var_t3_dn8 = assign34750_e39904_d_n8;
        locals.var_t3_dn9 = assign34750_e39904_d_n9;
        locals.var_t3_dn10 = assign34750_e39904_d_n10;
        locals.var_t3_dn11 = assign34750_e39904_d_n11;
        locals.var_t3_dn14 = assign34750_e39904_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign34760_e39912, assign34760_e39912_d_n0, assign34760_e39912_d_n2, assign34760_e39912_d_n4, assign34760_e39912_d_n5, assign34760_e39912_d_n6, assign34760_e39912_d_n7, assign34760_e39912_d_n8, assign34760_e39912_d_n9, assign34760_e39912_d_n10, assign34760_e39912_d_n11, assign34760_e39912_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign34760_e39910: f64 = (locals.var_t1 * locals.var_t3);
        (assign34760_e39910, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn14 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign34760_e39912;
        locals.var_t2_dn0 = assign34760_e39912_d_n0;
        locals.var_t2_dn2 = assign34760_e39912_d_n2;
        locals.var_t2_dn4 = assign34760_e39912_d_n4;
        locals.var_t2_dn5 = assign34760_e39912_d_n5;
        locals.var_t2_dn6 = assign34760_e39912_d_n6;
        locals.var_t2_dn7 = assign34760_e39912_d_n7;
        locals.var_t2_dn8 = assign34760_e39912_d_n8;
        locals.var_t2_dn9 = assign34760_e39912_d_n9;
        locals.var_t2_dn10 = assign34760_e39912_d_n10;
        locals.var_t2_dn11 = assign34760_e39912_d_n11;
        locals.var_t2_dn14 = assign34760_e39912_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign34770_e39920, assign34770_e39920_d_n0, assign34770_e39920_d_n2, assign34770_e39920_d_n4, assign34770_e39920_d_n5, assign34770_e39920_d_n6, assign34770_e39920_d_n7, assign34770_e39920_d_n8, assign34770_e39920_d_n9, assign34770_e39920_d_n10, assign34770_e39920_d_n11, assign34770_e39920_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign34770_e39918: f64 = (1.0 + locals.var_t2);
        (assign34770_e39918, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign34770_e39920;
        locals.var_t4_dn0 = assign34770_e39920_d_n0;
        locals.var_t4_dn2 = assign34770_e39920_d_n2;
        locals.var_t4_dn4 = assign34770_e39920_d_n4;
        locals.var_t4_dn5 = assign34770_e39920_d_n5;
        locals.var_t4_dn6 = assign34770_e39920_d_n6;
        locals.var_t4_dn7 = assign34770_e39920_d_n7;
        locals.var_t4_dn8 = assign34770_e39920_d_n8;
        locals.var_t4_dn9 = assign34770_e39920_d_n9;
        locals.var_t4_dn10 = assign34770_e39920_d_n10;
        locals.var_t4_dn11 = assign34770_e39920_d_n11;
        locals.var_t4_dn14 = assign34770_e39920_d_n14;
        locals.var_t4_rv = 0.0;

        let assign34780_e39924: f64 = (10.0 * 2.220446049250313e-16);
        let assign34780_e39925: f64 = (1.0 - assign34780_e39924);
        let assign34780_e39932: f64 = (10.0 * 2.220446049250313e-16);
        let assign34780_e39933: f64 = (1.0 + assign34780_e39932);
        let assign34780_e39935: f64 = if ((assign34780_e39925 <= p.p178) && (p.p178 <= assign34780_e39933)) { 1.0 } else { 0.0 };
        locals.var_guard808 = assign34780_e39935;
        locals.var_guard808_rv = 0.0;

        let (assign34790_e39945, assign34790_e39945_d_n0, assign34790_e39945_d_n2, assign34790_e39945_d_n4, assign34790_e39945_d_n5, assign34790_e39945_d_n6, assign34790_e39945_d_n7, assign34790_e39945_d_n8, assign34790_e39945_d_n9, assign34790_e39945_d_n10, assign34790_e39945_d_n11, assign34790_e39945_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard808 != 0.0)) {
        let assign34790_e39943: f64 = (1.0 / locals.var_t4);
        (assign34790_e39943, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign34790_e39945;
        locals.var_t5_dn0 = assign34790_e39945_d_n0;
        locals.var_t5_dn2 = assign34790_e39945_d_n2;
        locals.var_t5_dn4 = assign34790_e39945_d_n4;
        locals.var_t5_dn5 = assign34790_e39945_d_n5;
        locals.var_t5_dn6 = assign34790_e39945_d_n6;
        locals.var_t5_dn7 = assign34790_e39945_d_n7;
        locals.var_t5_dn8 = assign34790_e39945_d_n8;
        locals.var_t5_dn9 = assign34790_e39945_d_n9;
        locals.var_t5_dn10 = assign34790_e39945_d_n10;
        locals.var_t5_dn11 = assign34790_e39945_d_n11;
        locals.var_t5_dn14 = assign34790_e39945_d_n14;
        locals.var_t5_rv = 0.0;

        let assign34800_e39949: f64 = (10.0 * 2.220446049250313e-16);
        let assign34800_e39950: f64 = (2.0 - assign34800_e39949);
        let assign34800_e39957: f64 = (10.0 * 2.220446049250313e-16);
        let assign34800_e39958: f64 = (2.0 + assign34800_e39957);
        let assign34800_e39960: f64 = if ((assign34800_e39950 <= p.p178) && (p.p178 <= assign34800_e39958)) { 1.0 } else { 0.0 };
        locals.var_guard809 = assign34800_e39960;
        locals.var_guard809_rv = 0.0;

        let (assign34810_e39974, assign34810_e39974_d_n0, assign34810_e39974_d_n2, assign34810_e39974_d_n4, assign34810_e39974_d_n5, assign34810_e39974_d_n6, assign34810_e39974_d_n7, assign34810_e39974_d_n8, assign34810_e39974_d_n9, assign34810_e39974_d_n10, assign34810_e39974_d_n11, assign34810_e39974_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard808 == 0.0)) && (locals.var_guard809 != 0.0)) {
        let assign34810_e39971: f64 = (locals.var_t4).sqrt();
        let assign34810_e39972: f64 = (1.0 / assign34810_e39971);
        (assign34810_e39972, (-((locals.var_t4_dn0 / (2.0 * assign34810_e39971)) / (assign34810_e39971 * assign34810_e39971))), (-((locals.var_t4_dn2 / (2.0 * assign34810_e39971)) / (assign34810_e39971 * assign34810_e39971))), (-((locals.var_t4_dn4 / (2.0 * assign34810_e39971)) / (assign34810_e39971 * assign34810_e39971))), (-((locals.var_t4_dn5 / (2.0 * assign34810_e39971)) / (assign34810_e39971 * assign34810_e39971))), (-((locals.var_t4_dn6 / (2.0 * assign34810_e39971)) / (assign34810_e39971 * assign34810_e39971))), (-((locals.var_t4_dn7 / (2.0 * assign34810_e39971)) / (assign34810_e39971 * assign34810_e39971))), (-((locals.var_t4_dn8 / (2.0 * assign34810_e39971)) / (assign34810_e39971 * assign34810_e39971))), (-((locals.var_t4_dn9 / (2.0 * assign34810_e39971)) / (assign34810_e39971 * assign34810_e39971))), (-((locals.var_t4_dn10 / (2.0 * assign34810_e39971)) / (assign34810_e39971 * assign34810_e39971))), (-((locals.var_t4_dn11 / (2.0 * assign34810_e39971)) / (assign34810_e39971 * assign34810_e39971))), (-((locals.var_t4_dn14 / (2.0 * assign34810_e39971)) / (assign34810_e39971 * assign34810_e39971))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign34810_e39974;
        locals.var_t5_dn0 = assign34810_e39974_d_n0;
        locals.var_t5_dn2 = assign34810_e39974_d_n2;
        locals.var_t5_dn4 = assign34810_e39974_d_n4;
        locals.var_t5_dn5 = assign34810_e39974_d_n5;
        locals.var_t5_dn6 = assign34810_e39974_d_n6;
        locals.var_t5_dn7 = assign34810_e39974_d_n7;
        locals.var_t5_dn8 = assign34810_e39974_d_n8;
        locals.var_t5_dn9 = assign34810_e39974_d_n9;
        locals.var_t5_dn10 = assign34810_e39974_d_n10;
        locals.var_t5_dn11 = assign34810_e39974_d_n11;
        locals.var_t5_dn14 = assign34810_e39974_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign34820_e39998, assign34820_e39998_d_n0, assign34820_e39998_d_n2, assign34820_e39998_d_n4, assign34820_e39998_d_n5, assign34820_e39998_d_n6, assign34820_e39998_d_n7, assign34820_e39998_d_n8, assign34820_e39998_d_n9, assign34820_e39998_d_n10, assign34820_e39998_d_n11, assign34820_e39998_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard808 == 0.0)) && (locals.var_guard809 == 0.0)) {
        let (assign34820_e39996, assign34820_e39996_d_n0, assign34820_e39996_d_n2, assign34820_e39996_d_n4, assign34820_e39996_d_n5, assign34820_e39996_d_n6, assign34820_e39996_d_n7, assign34820_e39996_d_n8, assign34820_e39996_d_n9, assign34820_e39996_d_n10, assign34820_e39996_d_n11, assign34820_e39996_d_n14,) = {
            if (locals.var_t4 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign34820_e39990: f64 = (-1.0);
                let assign34820_e39992: f64 = (assign34820_e39990 / p.p178);
                let assign34820_e39994: f64 = (assign34820_e39992 - 1.0);
                let assign34820_e39995: f64 = (locals.var_t4).powf(assign34820_e39994);
                (assign34820_e39995, if 0.0 == 0.0 && ((assign34820_e39994) as f64).is_finite() && ((assign34820_e39994) as f64).fract() == 0.0 { if assign34820_e39994 == 0.0 { 0.0 } else { (assign34820_e39994 * ((locals.var_t4).powf(assign34820_e39994 - 1.0) * locals.var_t4_dn0)) } } else { (assign34820_e39995 * (assign34820_e39994 * (locals.var_t4_dn0 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign34820_e39994) as f64).is_finite() && ((assign34820_e39994) as f64).fract() == 0.0 { if assign34820_e39994 == 0.0 { 0.0 } else { (assign34820_e39994 * ((locals.var_t4).powf(assign34820_e39994 - 1.0) * locals.var_t4_dn2)) } } else { (assign34820_e39995 * (assign34820_e39994 * (locals.var_t4_dn2 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign34820_e39994) as f64).is_finite() && ((assign34820_e39994) as f64).fract() == 0.0 { if assign34820_e39994 == 0.0 { 0.0 } else { (assign34820_e39994 * ((locals.var_t4).powf(assign34820_e39994 - 1.0) * locals.var_t4_dn4)) } } else { (assign34820_e39995 * (assign34820_e39994 * (locals.var_t4_dn4 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign34820_e39994) as f64).is_finite() && ((assign34820_e39994) as f64).fract() == 0.0 { if assign34820_e39994 == 0.0 { 0.0 } else { (assign34820_e39994 * ((locals.var_t4).powf(assign34820_e39994 - 1.0) * locals.var_t4_dn5)) } } else { (assign34820_e39995 * (assign34820_e39994 * (locals.var_t4_dn5 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign34820_e39994) as f64).is_finite() && ((assign34820_e39994) as f64).fract() == 0.0 { if assign34820_e39994 == 0.0 { 0.0 } else { (assign34820_e39994 * ((locals.var_t4).powf(assign34820_e39994 - 1.0) * locals.var_t4_dn6)) } } else { (assign34820_e39995 * (assign34820_e39994 * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign34820_e39994) as f64).is_finite() && ((assign34820_e39994) as f64).fract() == 0.0 { if assign34820_e39994 == 0.0 { 0.0 } else { (assign34820_e39994 * ((locals.var_t4).powf(assign34820_e39994 - 1.0) * locals.var_t4_dn7)) } } else { (assign34820_e39995 * (assign34820_e39994 * (locals.var_t4_dn7 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign34820_e39994) as f64).is_finite() && ((assign34820_e39994) as f64).fract() == 0.0 { if assign34820_e39994 == 0.0 { 0.0 } else { (assign34820_e39994 * ((locals.var_t4).powf(assign34820_e39994 - 1.0) * locals.var_t4_dn8)) } } else { (assign34820_e39995 * (assign34820_e39994 * (locals.var_t4_dn8 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign34820_e39994) as f64).is_finite() && ((assign34820_e39994) as f64).fract() == 0.0 { if assign34820_e39994 == 0.0 { 0.0 } else { (assign34820_e39994 * ((locals.var_t4).powf(assign34820_e39994 - 1.0) * locals.var_t4_dn9)) } } else { (assign34820_e39995 * (assign34820_e39994 * (locals.var_t4_dn9 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign34820_e39994) as f64).is_finite() && ((assign34820_e39994) as f64).fract() == 0.0 { if assign34820_e39994 == 0.0 { 0.0 } else { (assign34820_e39994 * ((locals.var_t4).powf(assign34820_e39994 - 1.0) * locals.var_t4_dn10)) } } else { (assign34820_e39995 * (assign34820_e39994 * (locals.var_t4_dn10 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign34820_e39994) as f64).is_finite() && ((assign34820_e39994) as f64).fract() == 0.0 { if assign34820_e39994 == 0.0 { 0.0 } else { (assign34820_e39994 * ((locals.var_t4).powf(assign34820_e39994 - 1.0) * locals.var_t4_dn11)) } } else { (assign34820_e39995 * (assign34820_e39994 * (locals.var_t4_dn11 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign34820_e39994) as f64).is_finite() && ((assign34820_e39994) as f64).fract() == 0.0 { if assign34820_e39994 == 0.0 { 0.0 } else { (assign34820_e39994 * ((locals.var_t4).powf(assign34820_e39994 - 1.0) * locals.var_t4_dn14)) } } else { (assign34820_e39995 * (assign34820_e39994 * (locals.var_t4_dn14 / locals.var_t4))) },)
            }
        };
        (assign34820_e39996, assign34820_e39996_d_n0, assign34820_e39996_d_n2, assign34820_e39996_d_n4, assign34820_e39996_d_n5, assign34820_e39996_d_n6, assign34820_e39996_d_n7, assign34820_e39996_d_n8, assign34820_e39996_d_n9, assign34820_e39996_d_n10, assign34820_e39996_d_n11, assign34820_e39996_d_n14,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign34820_e39998;
        locals.var_t6_dn0 = assign34820_e39998_d_n0;
        locals.var_t6_dn2 = assign34820_e39998_d_n2;
        locals.var_t6_dn4 = assign34820_e39998_d_n4;
        locals.var_t6_dn5 = assign34820_e39998_d_n5;
        locals.var_t6_dn6 = assign34820_e39998_d_n6;
        locals.var_t6_dn7 = assign34820_e39998_d_n7;
        locals.var_t6_dn8 = assign34820_e39998_d_n8;
        locals.var_t6_dn9 = assign34820_e39998_d_n9;
        locals.var_t6_dn10 = assign34820_e39998_d_n10;
        locals.var_t6_dn11 = assign34820_e39998_d_n11;
        locals.var_t6_dn14 = assign34820_e39998_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign34830_e40012, assign34830_e40012_d_n0, assign34830_e40012_d_n2, assign34830_e40012_d_n4, assign34830_e40012_d_n5, assign34830_e40012_d_n6, assign34830_e40012_d_n7, assign34830_e40012_d_n8, assign34830_e40012_d_n9, assign34830_e40012_d_n10, assign34830_e40012_d_n11, assign34830_e40012_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard808 == 0.0)) && (locals.var_guard809 == 0.0)) {
        let assign34830_e40010: f64 = (locals.var_t4 * locals.var_t6);
        (assign34830_e40010, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn4 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn4)), ((locals.var_t4_dn5 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn5)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn7 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn7)), ((locals.var_t4_dn8 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn8)), ((locals.var_t4_dn9 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn9)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn11 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn11)), ((locals.var_t4_dn14 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign34830_e40012;
        locals.var_t5_dn0 = assign34830_e40012_d_n0;
        locals.var_t5_dn2 = assign34830_e40012_d_n2;
        locals.var_t5_dn4 = assign34830_e40012_d_n4;
        locals.var_t5_dn5 = assign34830_e40012_d_n5;
        locals.var_t5_dn6 = assign34830_e40012_d_n6;
        locals.var_t5_dn7 = assign34830_e40012_d_n7;
        locals.var_t5_dn8 = assign34830_e40012_d_n8;
        locals.var_t5_dn9 = assign34830_e40012_d_n9;
        locals.var_t5_dn10 = assign34830_e40012_d_n10;
        locals.var_t5_dn11 = assign34830_e40012_d_n11;
        locals.var_t5_dn14 = assign34830_e40012_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign34840_e40020, assign34840_e40020_d_n0, assign34840_e40020_d_n2, assign34840_e40020_d_n4, assign34840_e40020_d_n5, assign34840_e40020_d_n6, assign34840_e40020_d_n7, assign34840_e40020_d_n8, assign34840_e40020_d_n9, assign34840_e40020_d_n10, assign34840_e40020_d_n11, assign34840_e40020_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign34840_e40018: f64 = (locals.var_muun * locals.var_t5);
        (assign34840_e40018, ((locals.var_muun_dn0 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn0)), ((locals.var_muun_dn2 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn2)), ((locals.var_muun_dn4 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn4)), ((locals.var_muun_dn5 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn5)), ((locals.var_muun_dn6 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn6)), ((locals.var_muun_dn7 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn7)), ((locals.var_muun_dn8 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn8)), ((locals.var_muun_dn9 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn9)), ((locals.var_muun_dn10 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn10)), ((locals.var_muun_dn11 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn11)), ((locals.var_muun_dn14 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn14)),)
    } else {
        (locals.var_mu, locals.var_mu_dn0, locals.var_mu_dn2, locals.var_mu_dn4, locals.var_mu_dn5, locals.var_mu_dn6, locals.var_mu_dn7, locals.var_mu_dn8, locals.var_mu_dn9, locals.var_mu_dn10, locals.var_mu_dn11, locals.var_mu_dn14,)
    }
};
        locals.var_mu = assign34840_e40020;
        locals.var_mu_dn0 = assign34840_e40020_d_n0;
        locals.var_mu_dn2 = assign34840_e40020_d_n2;
        locals.var_mu_dn4 = assign34840_e40020_d_n4;
        locals.var_mu_dn5 = assign34840_e40020_d_n5;
        locals.var_mu_dn6 = assign34840_e40020_d_n6;
        locals.var_mu_dn7 = assign34840_e40020_d_n7;
        locals.var_mu_dn8 = assign34840_e40020_d_n8;
        locals.var_mu_dn9 = assign34840_e40020_d_n9;
        locals.var_mu_dn10 = assign34840_e40020_d_n10;
        locals.var_mu_dn11 = assign34840_e40020_d_n11;
        locals.var_mu_dn14 = assign34840_e40020_d_n14;
        locals.var_mu_rv = 0.0;

        let assign34850_e40023: f64 = if locals.var_vdsorg > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard810 = assign34850_e40023;
        locals.var_guard810_rv = 0.0;

        let (assign34860_e40035, assign34860_e40035_d_n0, assign34860_e40035_d_n2, assign34860_e40035_d_n4, assign34860_e40035_d_n5, assign34860_e40035_d_n6, assign34860_e40035_d_n7, assign34860_e40035_d_n8, assign34860_e40035_d_n9, assign34860_e40035_d_n10, assign34860_e40035_d_n11, assign34860_e40035_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) {
        let assign34860_e40032: f64 = (locals.var_cox * locals.var_cox);
        let assign34860_e40033: f64 = (locals.var_q_ndepm_esi / assign34860_e40032);
        (assign34860_e40033, (((locals.var_q_ndepm_esi_dn0 * assign34860_e40032) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn0 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn0)))) / (assign34860_e40032 * assign34860_e40032)), (((locals.var_q_ndepm_esi_dn2 * assign34860_e40032) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn2 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn2)))) / (assign34860_e40032 * assign34860_e40032)), (((locals.var_q_ndepm_esi_dn4 * assign34860_e40032) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn4 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn4)))) / (assign34860_e40032 * assign34860_e40032)), (((locals.var_q_ndepm_esi_dn5 * assign34860_e40032) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn5 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn5)))) / (assign34860_e40032 * assign34860_e40032)), (((locals.var_q_ndepm_esi_dn6 * assign34860_e40032) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn6 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn6)))) / (assign34860_e40032 * assign34860_e40032)), (((locals.var_q_ndepm_esi_dn7 * assign34860_e40032) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn7 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn7)))) / (assign34860_e40032 * assign34860_e40032)), (((locals.var_q_ndepm_esi_dn8 * assign34860_e40032) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn8 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn8)))) / (assign34860_e40032 * assign34860_e40032)), (((locals.var_q_ndepm_esi_dn9 * assign34860_e40032) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn9 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn9)))) / (assign34860_e40032 * assign34860_e40032)), (((locals.var_q_ndepm_esi_dn10 * assign34860_e40032) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn10 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn10)))) / (assign34860_e40032 * assign34860_e40032)), (((locals.var_q_ndepm_esi_dn11 * assign34860_e40032) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn11 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn11)))) / (assign34860_e40032 * assign34860_e40032)), (((locals.var_q_ndepm_esi_dn14 * assign34860_e40032) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn14 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn14)))) / (assign34860_e40032 * assign34860_e40032)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign34860_e40035;
        locals.var_t2_dn0 = assign34860_e40035_d_n0;
        locals.var_t2_dn2 = assign34860_e40035_d_n2;
        locals.var_t2_dn4 = assign34860_e40035_d_n4;
        locals.var_t2_dn5 = assign34860_e40035_d_n5;
        locals.var_t2_dn6 = assign34860_e40035_d_n6;
        locals.var_t2_dn7 = assign34860_e40035_d_n7;
        locals.var_t2_dn8 = assign34860_e40035_d_n8;
        locals.var_t2_dn9 = assign34860_e40035_d_n9;
        locals.var_t2_dn10 = assign34860_e40035_d_n10;
        locals.var_t2_dn11 = assign34860_e40035_d_n11;
        locals.var_t2_dn14 = assign34860_e40035_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign34870_e40049, assign34870_e40049_d_n0, assign34870_e40049_d_n2, assign34870_e40049_d_n4, assign34870_e40049_d_n5, assign34870_e40049_d_n6, assign34870_e40049_d_n7, assign34870_e40049_d_n8, assign34870_e40049_d_n9, assign34870_e40049_d_n10, assign34870_e40049_d_n11, assign34870_e40049_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) {
        let assign34870_e40043: f64 = (locals.var_vgp + locals.var_uc_depvdsef1);
        let assign34870_e40045: f64 = (assign34870_e40043 - locals.var_beta_inv);
        let assign34870_e40047: f64 = (assign34870_e40045 - locals.var_vbsz__blk440);
        (assign34870_e40047, (((locals.var_vgp_dn0 + locals.var_uc_depvdsef1_dn0) - locals.var_beta_inv_dn0) - locals.var_vbsz__blk440_dn0), (((locals.var_vgp_dn2 + locals.var_uc_depvdsef1_dn2) - locals.var_beta_inv_dn2) - locals.var_vbsz__blk440_dn2), (((locals.var_vgp_dn4 + locals.var_uc_depvdsef1_dn4) - locals.var_beta_inv_dn4) - locals.var_vbsz__blk440_dn4), (((locals.var_vgp_dn5 + locals.var_uc_depvdsef1_dn5) - locals.var_beta_inv_dn5) - locals.var_vbsz__blk440_dn5), (((locals.var_vgp_dn6 + locals.var_uc_depvdsef1_dn6) - locals.var_beta_inv_dn6) - locals.var_vbsz__blk440_dn6), (((locals.var_vgp_dn7 + locals.var_uc_depvdsef1_dn7) - locals.var_beta_inv_dn7) - locals.var_vbsz__blk440_dn7), (((locals.var_vgp_dn8 + locals.var_uc_depvdsef1_dn8) - locals.var_beta_inv_dn8) - locals.var_vbsz__blk440_dn8), (((locals.var_vgp_dn9 + locals.var_uc_depvdsef1_dn9) - locals.var_beta_inv_dn9) - locals.var_vbsz__blk440_dn9), (((locals.var_vgp_dn10 + locals.var_uc_depvdsef1_dn10) - locals.var_beta_inv_dn10) - locals.var_vbsz__blk440_dn10), (((locals.var_vgp_dn11 + locals.var_uc_depvdsef1_dn11) - locals.var_beta_inv_dn11) - locals.var_vbsz__blk440_dn11), (((locals.var_vgp_dn14 + locals.var_uc_depvdsef1_dn14) - locals.var_beta_inv_dn14) - locals.var_vbsz__blk440_dn14),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign34870_e40049;
        locals.var_t0_dn0 = assign34870_e40049_d_n0;
        locals.var_t0_dn2 = assign34870_e40049_d_n2;
        locals.var_t0_dn4 = assign34870_e40049_d_n4;
        locals.var_t0_dn5 = assign34870_e40049_d_n5;
        locals.var_t0_dn6 = assign34870_e40049_d_n6;
        locals.var_t0_dn7 = assign34870_e40049_d_n7;
        locals.var_t0_dn8 = assign34870_e40049_d_n8;
        locals.var_t0_dn9 = assign34870_e40049_d_n9;
        locals.var_t0_dn10 = assign34870_e40049_d_n10;
        locals.var_t0_dn11 = assign34870_e40049_d_n11;
        locals.var_t0_dn14 = assign34870_e40049_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign34880_e40063, assign34880_e40063_d_n0, assign34880_e40063_d_n2, assign34880_e40063_d_n4, assign34880_e40063_d_n5, assign34880_e40063_d_n6, assign34880_e40063_d_n7, assign34880_e40063_d_n8, assign34880_e40063_d_n9, assign34880_e40063_d_n10, assign34880_e40063_d_n11, assign34880_e40063_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) {
        let assign34880_e40058: f64 = (2.0 / locals.var_t2);
        let assign34880_e40060: f64 = (assign34880_e40058 * locals.var_t0);
        let assign34880_e40061: f64 = (1.0 + assign34880_e40060);
        (assign34880_e40061, (((-((2.0 * locals.var_t2_dn0) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign34880_e40058 * locals.var_t0_dn0)), (((-((2.0 * locals.var_t2_dn2) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign34880_e40058 * locals.var_t0_dn2)), (((-((2.0 * locals.var_t2_dn4) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign34880_e40058 * locals.var_t0_dn4)), (((-((2.0 * locals.var_t2_dn5) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign34880_e40058 * locals.var_t0_dn5)), (((-((2.0 * locals.var_t2_dn6) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign34880_e40058 * locals.var_t0_dn6)), (((-((2.0 * locals.var_t2_dn7) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign34880_e40058 * locals.var_t0_dn7)), (((-((2.0 * locals.var_t2_dn8) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign34880_e40058 * locals.var_t0_dn8)), (((-((2.0 * locals.var_t2_dn9) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign34880_e40058 * locals.var_t0_dn9)), (((-((2.0 * locals.var_t2_dn10) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign34880_e40058 * locals.var_t0_dn10)), (((-((2.0 * locals.var_t2_dn11) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign34880_e40058 * locals.var_t0_dn11)), (((-((2.0 * locals.var_t2_dn14) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign34880_e40058 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign34880_e40063;
        locals.var_t4_dn0 = assign34880_e40063_d_n0;
        locals.var_t4_dn2 = assign34880_e40063_d_n2;
        locals.var_t4_dn4 = assign34880_e40063_d_n4;
        locals.var_t4_dn5 = assign34880_e40063_d_n5;
        locals.var_t4_dn6 = assign34880_e40063_d_n6;
        locals.var_t4_dn7 = assign34880_e40063_d_n7;
        locals.var_t4_dn8 = assign34880_e40063_d_n8;
        locals.var_t4_dn9 = assign34880_e40063_d_n9;
        locals.var_t4_dn10 = assign34880_e40063_d_n10;
        locals.var_t4_dn11 = assign34880_e40063_d_n11;
        locals.var_t4_dn14 = assign34880_e40063_d_n14;
        locals.var_t4_rv = 0.0;

        let assign34890_e40067: f64 = 2.0;
        let assign34890_e40072: f64 = if ((locals.var_t4 < assign34890_e40067) && (2.0 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard811 = assign34890_e40072;
        locals.var_guard811_rv = 0.0;

        let (assign34900_e40086, assign34900_e40086_d_n0, assign34900_e40086_d_n2, assign34900_e40086_d_n4, assign34900_e40086_d_n5, assign34900_e40086_d_n6, assign34900_e40086_d_n7, assign34900_e40086_d_n8, assign34900_e40086_d_n9, assign34900_e40086_d_n10, assign34900_e40086_d_n11, assign34900_e40086_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 != 0.0)) {
        let assign34900_e40082: f64 = 2.0;
        let assign34900_e40084: f64 = (assign34900_e40082 - locals.var_t4);
        (assign34900_e40084, (-locals.var_t4_dn0), (-locals.var_t4_dn2), (-locals.var_t4_dn4), (-locals.var_t4_dn5), (-locals.var_t4_dn6), (-locals.var_t4_dn7), (-locals.var_t4_dn8), (-locals.var_t4_dn9), (-locals.var_t4_dn10), (-locals.var_t4_dn11), (-locals.var_t4_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign34900_e40086;
        locals.var_tmf1_dn0 = assign34900_e40086_d_n0;
        locals.var_tmf1_dn2 = assign34900_e40086_d_n2;
        locals.var_tmf1_dn4 = assign34900_e40086_d_n4;
        locals.var_tmf1_dn5 = assign34900_e40086_d_n5;
        locals.var_tmf1_dn6 = assign34900_e40086_d_n6;
        locals.var_tmf1_dn7 = assign34900_e40086_d_n7;
        locals.var_tmf1_dn8 = assign34900_e40086_d_n8;
        locals.var_tmf1_dn9 = assign34900_e40086_d_n9;
        locals.var_tmf1_dn10 = assign34900_e40086_d_n10;
        locals.var_tmf1_dn11 = assign34900_e40086_d_n11;
        locals.var_tmf1_dn14 = assign34900_e40086_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign34910_e40098, assign34910_e40098_d_n0, assign34910_e40098_d_n2, assign34910_e40098_d_n4, assign34910_e40098_d_n5, assign34910_e40098_d_n6, assign34910_e40098_d_n7, assign34910_e40098_d_n8, assign34910_e40098_d_n9, assign34910_e40098_d_n10, assign34910_e40098_d_n11, assign34910_e40098_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 != 0.0)) {
        let assign34910_e40096: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign34910_e40096, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign34910_e40098;
        locals.var_x2_dn0 = assign34910_e40098_d_n0;
        locals.var_x2_dn2 = assign34910_e40098_d_n2;
        locals.var_x2_dn4 = assign34910_e40098_d_n4;
        locals.var_x2_dn5 = assign34910_e40098_d_n5;
        locals.var_x2_dn6 = assign34910_e40098_d_n6;
        locals.var_x2_dn7 = assign34910_e40098_d_n7;
        locals.var_x2_dn8 = assign34910_e40098_d_n8;
        locals.var_x2_dn9 = assign34910_e40098_d_n9;
        locals.var_x2_dn10 = assign34910_e40098_d_n10;
        locals.var_x2_dn11 = assign34910_e40098_d_n11;
        locals.var_x2_dn14 = assign34910_e40098_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign34920_e40110, assign34920_e40110_d_n0, assign34920_e40110_d_n2, assign34920_e40110_d_n4, assign34920_e40110_d_n5, assign34920_e40110_d_n6, assign34920_e40110_d_n7, assign34920_e40110_d_n8, assign34920_e40110_d_n9, assign34920_e40110_d_n10, assign34920_e40110_d_n11, assign34920_e40110_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 != 0.0)) {
        let assign34920_e40108: f64 = (2.0 * 2.0);
        (assign34920_e40108, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign34920_e40110;
        locals.var_xmax2_dn0 = assign34920_e40110_d_n0;
        locals.var_xmax2_dn2 = assign34920_e40110_d_n2;
        locals.var_xmax2_dn4 = assign34920_e40110_d_n4;
        locals.var_xmax2_dn5 = assign34920_e40110_d_n5;
        locals.var_xmax2_dn6 = assign34920_e40110_d_n6;
        locals.var_xmax2_dn7 = assign34920_e40110_d_n7;
        locals.var_xmax2_dn8 = assign34920_e40110_d_n8;
        locals.var_xmax2_dn9 = assign34920_e40110_d_n9;
        locals.var_xmax2_dn10 = assign34920_e40110_d_n10;
        locals.var_xmax2_dn11 = assign34920_e40110_d_n11;
        locals.var_xmax2_dn14 = assign34920_e40110_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign34930_e40120, assign34930_e40120_d_n0, assign34930_e40120_d_n2, assign34930_e40120_d_n4, assign34930_e40120_d_n5, assign34930_e40120_d_n6, assign34930_e40120_d_n7, assign34930_e40120_d_n8, assign34930_e40120_d_n9, assign34930_e40120_d_n10, assign34930_e40120_d_n11, assign34930_e40120_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign34930_e40120;
        locals.var_xp_dn0 = assign34930_e40120_d_n0;
        locals.var_xp_dn2 = assign34930_e40120_d_n2;
        locals.var_xp_dn4 = assign34930_e40120_d_n4;
        locals.var_xp_dn5 = assign34930_e40120_d_n5;
        locals.var_xp_dn6 = assign34930_e40120_d_n6;
        locals.var_xp_dn7 = assign34930_e40120_d_n7;
        locals.var_xp_dn8 = assign34930_e40120_d_n8;
        locals.var_xp_dn9 = assign34930_e40120_d_n9;
        locals.var_xp_dn10 = assign34930_e40120_d_n10;
        locals.var_xp_dn11 = assign34930_e40120_d_n11;
        locals.var_xp_dn14 = assign34930_e40120_d_n14;
        locals.var_xp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_114(
        locals: &mut StampLocals,
    ) {
        let (assign34940_e40130, assign34940_e40130_d_n0, assign34940_e40130_d_n2, assign34940_e40130_d_n4, assign34940_e40130_d_n5, assign34940_e40130_d_n6, assign34940_e40130_d_n7, assign34940_e40130_d_n8, assign34940_e40130_d_n9, assign34940_e40130_d_n10, assign34940_e40130_d_n11, assign34940_e40130_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign34940_e40130;
        locals.var_xmp_dn0 = assign34940_e40130_d_n0;
        locals.var_xmp_dn2 = assign34940_e40130_d_n2;
        locals.var_xmp_dn4 = assign34940_e40130_d_n4;
        locals.var_xmp_dn5 = assign34940_e40130_d_n5;
        locals.var_xmp_dn6 = assign34940_e40130_d_n6;
        locals.var_xmp_dn7 = assign34940_e40130_d_n7;
        locals.var_xmp_dn8 = assign34940_e40130_d_n8;
        locals.var_xmp_dn9 = assign34940_e40130_d_n9;
        locals.var_xmp_dn10 = assign34940_e40130_d_n10;
        locals.var_xmp_dn11 = assign34940_e40130_d_n11;
        locals.var_xmp_dn14 = assign34940_e40130_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign34950_e40140,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign34950_e40140;
        locals.var_m0_rv = 0.0;

        let (assign34960_e40150,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign34960_e40150;
        locals.var_mm_rv = 0.0;

        let (assign34970_e40160, assign34970_e40160_d_n0, assign34970_e40160_d_n2, assign34970_e40160_d_n4, assign34970_e40160_d_n5, assign34970_e40160_d_n6, assign34970_e40160_d_n7, assign34970_e40160_d_n8, assign34970_e40160_d_n9, assign34970_e40160_d_n10, assign34970_e40160_d_n11, assign34970_e40160_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign34970_e40160;
        locals.var_arg_dn0 = assign34970_e40160_d_n0;
        locals.var_arg_dn2 = assign34970_e40160_d_n2;
        locals.var_arg_dn4 = assign34970_e40160_d_n4;
        locals.var_arg_dn5 = assign34970_e40160_d_n5;
        locals.var_arg_dn6 = assign34970_e40160_d_n6;
        locals.var_arg_dn7 = assign34970_e40160_d_n7;
        locals.var_arg_dn8 = assign34970_e40160_d_n8;
        locals.var_arg_dn9 = assign34970_e40160_d_n9;
        locals.var_arg_dn10 = assign34970_e40160_d_n10;
        locals.var_arg_dn11 = assign34970_e40160_d_n11;
        locals.var_arg_dn14 = assign34970_e40160_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign34980_e40170, assign34980_e40170_d_n0, assign34980_e40170_d_n2, assign34980_e40170_d_n4, assign34980_e40170_d_n5, assign34980_e40170_d_n6, assign34980_e40170_d_n7, assign34980_e40170_d_n8, assign34980_e40170_d_n9, assign34980_e40170_d_n10, assign34980_e40170_d_n11, assign34980_e40170_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign34980_e40170;
        locals.var_dnm_dn0 = assign34980_e40170_d_n0;
        locals.var_dnm_dn2 = assign34980_e40170_d_n2;
        locals.var_dnm_dn4 = assign34980_e40170_d_n4;
        locals.var_dnm_dn5 = assign34980_e40170_d_n5;
        locals.var_dnm_dn6 = assign34980_e40170_d_n6;
        locals.var_dnm_dn7 = assign34980_e40170_d_n7;
        locals.var_dnm_dn8 = assign34980_e40170_d_n8;
        locals.var_dnm_dn9 = assign34980_e40170_d_n9;
        locals.var_dnm_dn10 = assign34980_e40170_d_n10;
        locals.var_dnm_dn11 = assign34980_e40170_d_n11;
        locals.var_dnm_dn14 = assign34980_e40170_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign34990_e40182, assign34990_e40182_d_n0, assign34990_e40182_d_n2, assign34990_e40182_d_n4, assign34990_e40182_d_n5, assign34990_e40182_d_n6, assign34990_e40182_d_n7, assign34990_e40182_d_n8, assign34990_e40182_d_n9, assign34990_e40182_d_n10, assign34990_e40182_d_n11, assign34990_e40182_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 != 0.0)) {
        let assign34990_e40180: f64 = (locals.var_xp * locals.var_x2);
        (assign34990_e40180, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign34990_e40182;
        locals.var_xp_dn0 = assign34990_e40182_d_n0;
        locals.var_xp_dn2 = assign34990_e40182_d_n2;
        locals.var_xp_dn4 = assign34990_e40182_d_n4;
        locals.var_xp_dn5 = assign34990_e40182_d_n5;
        locals.var_xp_dn6 = assign34990_e40182_d_n6;
        locals.var_xp_dn7 = assign34990_e40182_d_n7;
        locals.var_xp_dn8 = assign34990_e40182_d_n8;
        locals.var_xp_dn9 = assign34990_e40182_d_n9;
        locals.var_xp_dn10 = assign34990_e40182_d_n10;
        locals.var_xp_dn11 = assign34990_e40182_d_n11;
        locals.var_xp_dn14 = assign34990_e40182_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign35000_e40194, assign35000_e40194_d_n0, assign35000_e40194_d_n2, assign35000_e40194_d_n4, assign35000_e40194_d_n5, assign35000_e40194_d_n6, assign35000_e40194_d_n7, assign35000_e40194_d_n8, assign35000_e40194_d_n9, assign35000_e40194_d_n10, assign35000_e40194_d_n11, assign35000_e40194_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 != 0.0)) {
        let assign35000_e40192: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign35000_e40192, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign35000_e40194;
        locals.var_xmp_dn0 = assign35000_e40194_d_n0;
        locals.var_xmp_dn2 = assign35000_e40194_d_n2;
        locals.var_xmp_dn4 = assign35000_e40194_d_n4;
        locals.var_xmp_dn5 = assign35000_e40194_d_n5;
        locals.var_xmp_dn6 = assign35000_e40194_d_n6;
        locals.var_xmp_dn7 = assign35000_e40194_d_n7;
        locals.var_xmp_dn8 = assign35000_e40194_d_n8;
        locals.var_xmp_dn9 = assign35000_e40194_d_n9;
        locals.var_xmp_dn10 = assign35000_e40194_d_n10;
        locals.var_xmp_dn11 = assign35000_e40194_d_n11;
        locals.var_xmp_dn14 = assign35000_e40194_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign35010_e40206, assign35010_e40206_d_n0, assign35010_e40206_d_n2, assign35010_e40206_d_n4, assign35010_e40206_d_n5, assign35010_e40206_d_n6, assign35010_e40206_d_n7, assign35010_e40206_d_n8, assign35010_e40206_d_n9, assign35010_e40206_d_n10, assign35010_e40206_d_n11, assign35010_e40206_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 != 0.0)) {
        let assign35010_e40204: f64 = (locals.var_xp * locals.var_x2);
        (assign35010_e40204, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign35010_e40206;
        locals.var_xp_dn0 = assign35010_e40206_d_n0;
        locals.var_xp_dn2 = assign35010_e40206_d_n2;
        locals.var_xp_dn4 = assign35010_e40206_d_n4;
        locals.var_xp_dn5 = assign35010_e40206_d_n5;
        locals.var_xp_dn6 = assign35010_e40206_d_n6;
        locals.var_xp_dn7 = assign35010_e40206_d_n7;
        locals.var_xp_dn8 = assign35010_e40206_d_n8;
        locals.var_xp_dn9 = assign35010_e40206_d_n9;
        locals.var_xp_dn10 = assign35010_e40206_d_n10;
        locals.var_xp_dn11 = assign35010_e40206_d_n11;
        locals.var_xp_dn14 = assign35010_e40206_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign35020_e40218, assign35020_e40218_d_n0, assign35020_e40218_d_n2, assign35020_e40218_d_n4, assign35020_e40218_d_n5, assign35020_e40218_d_n6, assign35020_e40218_d_n7, assign35020_e40218_d_n8, assign35020_e40218_d_n9, assign35020_e40218_d_n10, assign35020_e40218_d_n11, assign35020_e40218_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 != 0.0)) {
        let assign35020_e40216: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign35020_e40216, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign35020_e40218;
        locals.var_xmp_dn0 = assign35020_e40218_d_n0;
        locals.var_xmp_dn2 = assign35020_e40218_d_n2;
        locals.var_xmp_dn4 = assign35020_e40218_d_n4;
        locals.var_xmp_dn5 = assign35020_e40218_d_n5;
        locals.var_xmp_dn6 = assign35020_e40218_d_n6;
        locals.var_xmp_dn7 = assign35020_e40218_d_n7;
        locals.var_xmp_dn8 = assign35020_e40218_d_n8;
        locals.var_xmp_dn9 = assign35020_e40218_d_n9;
        locals.var_xmp_dn10 = assign35020_e40218_d_n10;
        locals.var_xmp_dn11 = assign35020_e40218_d_n11;
        locals.var_xmp_dn14 = assign35020_e40218_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign35030_e40230, assign35030_e40230_d_n0, assign35030_e40230_d_n2, assign35030_e40230_d_n4, assign35030_e40230_d_n5, assign35030_e40230_d_n6, assign35030_e40230_d_n7, assign35030_e40230_d_n8, assign35030_e40230_d_n9, assign35030_e40230_d_n10, assign35030_e40230_d_n11, assign35030_e40230_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 != 0.0)) {
        let assign35030_e40228: f64 = (locals.var_xp + locals.var_xmp);
        (assign35030_e40228, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign35030_e40230;
        locals.var_arg_dn0 = assign35030_e40230_d_n0;
        locals.var_arg_dn2 = assign35030_e40230_d_n2;
        locals.var_arg_dn4 = assign35030_e40230_d_n4;
        locals.var_arg_dn5 = assign35030_e40230_d_n5;
        locals.var_arg_dn6 = assign35030_e40230_d_n6;
        locals.var_arg_dn7 = assign35030_e40230_d_n7;
        locals.var_arg_dn8 = assign35030_e40230_d_n8;
        locals.var_arg_dn9 = assign35030_e40230_d_n9;
        locals.var_arg_dn10 = assign35030_e40230_d_n10;
        locals.var_arg_dn11 = assign35030_e40230_d_n11;
        locals.var_arg_dn14 = assign35030_e40230_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign35040_e40240, assign35040_e40240_d_n0, assign35040_e40240_d_n2, assign35040_e40240_d_n4, assign35040_e40240_d_n5, assign35040_e40240_d_n6, assign35040_e40240_d_n7, assign35040_e40240_d_n8, assign35040_e40240_d_n9, assign35040_e40240_d_n10, assign35040_e40240_d_n11, assign35040_e40240_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign35040_e40240;
        locals.var_dnm_dn0 = assign35040_e40240_d_n0;
        locals.var_dnm_dn2 = assign35040_e40240_d_n2;
        locals.var_dnm_dn4 = assign35040_e40240_d_n4;
        locals.var_dnm_dn5 = assign35040_e40240_d_n5;
        locals.var_dnm_dn6 = assign35040_e40240_d_n6;
        locals.var_dnm_dn7 = assign35040_e40240_d_n7;
        locals.var_dnm_dn8 = assign35040_e40240_d_n8;
        locals.var_dnm_dn9 = assign35040_e40240_d_n9;
        locals.var_dnm_dn10 = assign35040_e40240_d_n10;
        locals.var_dnm_dn11 = assign35040_e40240_d_n11;
        locals.var_dnm_dn14 = assign35040_e40240_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign35050_e40255: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard812 = assign35050_e40255;
        locals.var_guard812_rv = 0.0;

        let assign35060_e40258: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard813 = assign35060_e40258;
        locals.var_guard813_rv = 0.0;

        let (assign35070_e40272,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign35070_e40272;
        locals.var_mm_rv = 0.0;

        let assign35080_e40275: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard814 = assign35080_e40275;
        locals.var_guard814_rv = 0.0;

        let (assign35090_e40292,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 == 0.0)) && (locals.var_guard814 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign35090_e40292;
        locals.var_mm_rv = 0.0;

        let assign35100_e40295: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard815 = assign35100_e40295;
        locals.var_guard815_rv = 0.0;

        let (assign35110_e40315,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 == 0.0)) && (locals.var_guard814 == 0.0)) && (locals.var_guard815 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign35110_e40315;
        locals.var_mm_rv = 0.0;

        let assign35120_e40318: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard816 = assign35120_e40318;
        locals.var_guard816_rv = 0.0;

        let (assign35130_e40341,) = {
    if (((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_guard813 == 0.0)) && (locals.var_guard814 == 0.0)) && (locals.var_guard815 == 0.0)) && (locals.var_guard816 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign35130_e40341;
        locals.var_mm_rv = 0.0;

        let (assign35140_e40353,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 != 0.0)) && (locals.var_guard812 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign35140_e40353;
        locals.var_m0_rv = 0.0;

        let mut assign35150_loop_guard: usize = 0;
        while {
            let assign35150_cond_e40366: f64 = if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 != 0.0)) && (locals.var_guard812 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign35150_cond_e40366 != 0.0
        } {
            assign35150_loop_guard += 1;
            assert!(assign35150_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign35150_body0_e40379, assign35150_body0_e40379_d_n0, assign35150_body0_e40379_d_n2, assign35150_body0_e40379_d_n4, assign35150_body0_e40379_d_n5, assign35150_body0_e40379_d_n6, assign35150_body0_e40379_d_n7, assign35150_body0_e40379_d_n8, assign35150_body0_e40379_d_n9, assign35150_body0_e40379_d_n10, assign35150_body0_e40379_d_n11, assign35150_body0_e40379_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 != 0.0)) && (locals.var_guard812 != 0.0)) {
        let assign35150_body0_e40377: f64 = (locals.var_dnm).sqrt();
        (assign35150_body0_e40377, (locals.var_dnm_dn0 / (2.0 * assign35150_body0_e40377)), (locals.var_dnm_dn2 / (2.0 * assign35150_body0_e40377)), (locals.var_dnm_dn4 / (2.0 * assign35150_body0_e40377)), (locals.var_dnm_dn5 / (2.0 * assign35150_body0_e40377)), (locals.var_dnm_dn6 / (2.0 * assign35150_body0_e40377)), (locals.var_dnm_dn7 / (2.0 * assign35150_body0_e40377)), (locals.var_dnm_dn8 / (2.0 * assign35150_body0_e40377)), (locals.var_dnm_dn9 / (2.0 * assign35150_body0_e40377)), (locals.var_dnm_dn10 / (2.0 * assign35150_body0_e40377)), (locals.var_dnm_dn11 / (2.0 * assign35150_body0_e40377)), (locals.var_dnm_dn14 / (2.0 * assign35150_body0_e40377)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign35150_body0_e40379;
            locals.var_dnm_dn0 = assign35150_body0_e40379_d_n0;
            locals.var_dnm_dn2 = assign35150_body0_e40379_d_n2;
            locals.var_dnm_dn4 = assign35150_body0_e40379_d_n4;
            locals.var_dnm_dn5 = assign35150_body0_e40379_d_n5;
            locals.var_dnm_dn6 = assign35150_body0_e40379_d_n6;
            locals.var_dnm_dn7 = assign35150_body0_e40379_d_n7;
            locals.var_dnm_dn8 = assign35150_body0_e40379_d_n8;
            locals.var_dnm_dn9 = assign35150_body0_e40379_d_n9;
            locals.var_dnm_dn10 = assign35150_body0_e40379_d_n10;
            locals.var_dnm_dn11 = assign35150_body0_e40379_d_n11;
            locals.var_dnm_dn14 = assign35150_body0_e40379_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign35150_body1_e40393,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 != 0.0)) && (locals.var_guard812 != 0.0)) {
        let assign35150_body1_e40391: f64 = (locals.var_m0 + 1.0);
        (assign35150_body1_e40391,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign35150_body1_e40393;
            locals.var_m0_rv = 0.0;
        }

        let (assign35160_e40417, assign35160_e40417_d_n0, assign35160_e40417_d_n2, assign35160_e40417_d_n4, assign35160_e40417_d_n5, assign35160_e40417_d_n6, assign35160_e40417_d_n7, assign35160_e40417_d_n8, assign35160_e40417_d_n9, assign35160_e40417_d_n10, assign35160_e40417_d_n11, assign35160_e40417_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 != 0.0)) && (locals.var_guard812 == 0.0)) {
        let (assign35160_e40415, assign35160_e40415_d_n0, assign35160_e40415_d_n2, assign35160_e40415_d_n4, assign35160_e40415_d_n5, assign35160_e40415_d_n6, assign35160_e40415_d_n7, assign35160_e40415_d_n8, assign35160_e40415_d_n9, assign35160_e40415_d_n10, assign35160_e40415_d_n11, assign35160_e40415_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign35160_e40412: f64 = (2.0 * 2.0);
                let assign35160_e40413: f64 = (1.0 / assign35160_e40412);
                let assign35160_e40414: f64 = (locals.var_dnm).powf(assign35160_e40413);
                (assign35160_e40414, if 0.0 == 0.0 && ((assign35160_e40413) as f64).is_finite() && ((assign35160_e40413) as f64).fract() == 0.0 { if assign35160_e40413 == 0.0 { 0.0 } else { (assign35160_e40413 * ((locals.var_dnm).powf(assign35160_e40413 - 1.0) * locals.var_dnm_dn0)) } } else { (assign35160_e40414 * (assign35160_e40413 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35160_e40413) as f64).is_finite() && ((assign35160_e40413) as f64).fract() == 0.0 { if assign35160_e40413 == 0.0 { 0.0 } else { (assign35160_e40413 * ((locals.var_dnm).powf(assign35160_e40413 - 1.0) * locals.var_dnm_dn2)) } } else { (assign35160_e40414 * (assign35160_e40413 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35160_e40413) as f64).is_finite() && ((assign35160_e40413) as f64).fract() == 0.0 { if assign35160_e40413 == 0.0 { 0.0 } else { (assign35160_e40413 * ((locals.var_dnm).powf(assign35160_e40413 - 1.0) * locals.var_dnm_dn4)) } } else { (assign35160_e40414 * (assign35160_e40413 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35160_e40413) as f64).is_finite() && ((assign35160_e40413) as f64).fract() == 0.0 { if assign35160_e40413 == 0.0 { 0.0 } else { (assign35160_e40413 * ((locals.var_dnm).powf(assign35160_e40413 - 1.0) * locals.var_dnm_dn5)) } } else { (assign35160_e40414 * (assign35160_e40413 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35160_e40413) as f64).is_finite() && ((assign35160_e40413) as f64).fract() == 0.0 { if assign35160_e40413 == 0.0 { 0.0 } else { (assign35160_e40413 * ((locals.var_dnm).powf(assign35160_e40413 - 1.0) * locals.var_dnm_dn6)) } } else { (assign35160_e40414 * (assign35160_e40413 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35160_e40413) as f64).is_finite() && ((assign35160_e40413) as f64).fract() == 0.0 { if assign35160_e40413 == 0.0 { 0.0 } else { (assign35160_e40413 * ((locals.var_dnm).powf(assign35160_e40413 - 1.0) * locals.var_dnm_dn7)) } } else { (assign35160_e40414 * (assign35160_e40413 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35160_e40413) as f64).is_finite() && ((assign35160_e40413) as f64).fract() == 0.0 { if assign35160_e40413 == 0.0 { 0.0 } else { (assign35160_e40413 * ((locals.var_dnm).powf(assign35160_e40413 - 1.0) * locals.var_dnm_dn8)) } } else { (assign35160_e40414 * (assign35160_e40413 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35160_e40413) as f64).is_finite() && ((assign35160_e40413) as f64).fract() == 0.0 { if assign35160_e40413 == 0.0 { 0.0 } else { (assign35160_e40413 * ((locals.var_dnm).powf(assign35160_e40413 - 1.0) * locals.var_dnm_dn9)) } } else { (assign35160_e40414 * (assign35160_e40413 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35160_e40413) as f64).is_finite() && ((assign35160_e40413) as f64).fract() == 0.0 { if assign35160_e40413 == 0.0 { 0.0 } else { (assign35160_e40413 * ((locals.var_dnm).powf(assign35160_e40413 - 1.0) * locals.var_dnm_dn10)) } } else { (assign35160_e40414 * (assign35160_e40413 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35160_e40413) as f64).is_finite() && ((assign35160_e40413) as f64).fract() == 0.0 { if assign35160_e40413 == 0.0 { 0.0 } else { (assign35160_e40413 * ((locals.var_dnm).powf(assign35160_e40413 - 1.0) * locals.var_dnm_dn11)) } } else { (assign35160_e40414 * (assign35160_e40413 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35160_e40413) as f64).is_finite() && ((assign35160_e40413) as f64).fract() == 0.0 { if assign35160_e40413 == 0.0 { 0.0 } else { (assign35160_e40413 * ((locals.var_dnm).powf(assign35160_e40413 - 1.0) * locals.var_dnm_dn14)) } } else { (assign35160_e40414 * (assign35160_e40413 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign35160_e40415, assign35160_e40415_d_n0, assign35160_e40415_d_n2, assign35160_e40415_d_n4, assign35160_e40415_d_n5, assign35160_e40415_d_n6, assign35160_e40415_d_n7, assign35160_e40415_d_n8, assign35160_e40415_d_n9, assign35160_e40415_d_n10, assign35160_e40415_d_n11, assign35160_e40415_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign35160_e40417;
        locals.var_dnm_dn0 = assign35160_e40417_d_n0;
        locals.var_dnm_dn2 = assign35160_e40417_d_n2;
        locals.var_dnm_dn4 = assign35160_e40417_d_n4;
        locals.var_dnm_dn5 = assign35160_e40417_d_n5;
        locals.var_dnm_dn6 = assign35160_e40417_d_n6;
        locals.var_dnm_dn7 = assign35160_e40417_d_n7;
        locals.var_dnm_dn8 = assign35160_e40417_d_n8;
        locals.var_dnm_dn9 = assign35160_e40417_d_n9;
        locals.var_dnm_dn10 = assign35160_e40417_d_n10;
        locals.var_dnm_dn11 = assign35160_e40417_d_n11;
        locals.var_dnm_dn14 = assign35160_e40417_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign35170_e40429, assign35170_e40429_d_n0, assign35170_e40429_d_n2, assign35170_e40429_d_n4, assign35170_e40429_d_n5, assign35170_e40429_d_n6, assign35170_e40429_d_n7, assign35170_e40429_d_n8, assign35170_e40429_d_n9, assign35170_e40429_d_n10, assign35170_e40429_d_n11, assign35170_e40429_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 != 0.0)) {
        let assign35170_e40427: f64 = (1.0 / locals.var_dnm);
        (assign35170_e40427, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign35170_e40429;
        locals.var_dnm_dn0 = assign35170_e40429_d_n0;
        locals.var_dnm_dn2 = assign35170_e40429_d_n2;
        locals.var_dnm_dn4 = assign35170_e40429_d_n4;
        locals.var_dnm_dn5 = assign35170_e40429_d_n5;
        locals.var_dnm_dn6 = assign35170_e40429_d_n6;
        locals.var_dnm_dn7 = assign35170_e40429_d_n7;
        locals.var_dnm_dn8 = assign35170_e40429_d_n8;
        locals.var_dnm_dn9 = assign35170_e40429_d_n9;
        locals.var_dnm_dn10 = assign35170_e40429_d_n10;
        locals.var_dnm_dn11 = assign35170_e40429_d_n11;
        locals.var_dnm_dn14 = assign35170_e40429_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign35180_e40443, assign35180_e40443_d_n0, assign35180_e40443_d_n2, assign35180_e40443_d_n4, assign35180_e40443_d_n5, assign35180_e40443_d_n6, assign35180_e40443_d_n7, assign35180_e40443_d_n8, assign35180_e40443_d_n9, assign35180_e40443_d_n10, assign35180_e40443_d_n11, assign35180_e40443_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 != 0.0)) {
        let assign35180_e40439: f64 = (locals.var_tmf1 * 2.0);
        let assign35180_e40441: f64 = (assign35180_e40439 * locals.var_dnm);
        (assign35180_e40441, (((locals.var_tmf1_dn0 * 2.0) * locals.var_dnm) + (assign35180_e40439 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 2.0) * locals.var_dnm) + (assign35180_e40439 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 2.0) * locals.var_dnm) + (assign35180_e40439 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 2.0) * locals.var_dnm) + (assign35180_e40439 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 2.0) * locals.var_dnm) + (assign35180_e40439 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 2.0) * locals.var_dnm) + (assign35180_e40439 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 2.0) * locals.var_dnm) + (assign35180_e40439 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 2.0) * locals.var_dnm) + (assign35180_e40439 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 2.0) * locals.var_dnm) + (assign35180_e40439 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 2.0) * locals.var_dnm) + (assign35180_e40439 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 2.0) * locals.var_dnm) + (assign35180_e40439 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign35180_e40443;
        locals.var_tmf0_dn0 = assign35180_e40443_d_n0;
        locals.var_tmf0_dn2 = assign35180_e40443_d_n2;
        locals.var_tmf0_dn4 = assign35180_e40443_d_n4;
        locals.var_tmf0_dn5 = assign35180_e40443_d_n5;
        locals.var_tmf0_dn6 = assign35180_e40443_d_n6;
        locals.var_tmf0_dn7 = assign35180_e40443_d_n7;
        locals.var_tmf0_dn8 = assign35180_e40443_d_n8;
        locals.var_tmf0_dn9 = assign35180_e40443_d_n9;
        locals.var_tmf0_dn10 = assign35180_e40443_d_n10;
        locals.var_tmf0_dn11 = assign35180_e40443_d_n11;
        locals.var_tmf0_dn14 = assign35180_e40443_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign35190_e40459, assign35190_e40459_d_n0, assign35190_e40459_d_n2, assign35190_e40459_d_n4, assign35190_e40459_d_n5, assign35190_e40459_d_n6, assign35190_e40459_d_n7, assign35190_e40459_d_n8, assign35190_e40459_d_n9, assign35190_e40459_d_n10, assign35190_e40459_d_n11, assign35190_e40459_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 != 0.0)) {
        let assign35190_e40453: f64 = (2.0 * locals.var_xmp);
        let assign35190_e40455: f64 = (assign35190_e40453 * locals.var_dnm);
        let assign35190_e40457: f64 = (assign35190_e40455 / locals.var_arg);
        (assign35190_e40457, ((((((2.0 * locals.var_xmp_dn0) * locals.var_dnm) + (assign35190_e40453 * locals.var_dnm_dn0)) * locals.var_arg) - (assign35190_e40455 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn2) * locals.var_dnm) + (assign35190_e40453 * locals.var_dnm_dn2)) * locals.var_arg) - (assign35190_e40455 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn4) * locals.var_dnm) + (assign35190_e40453 * locals.var_dnm_dn4)) * locals.var_arg) - (assign35190_e40455 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn5) * locals.var_dnm) + (assign35190_e40453 * locals.var_dnm_dn5)) * locals.var_arg) - (assign35190_e40455 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn6) * locals.var_dnm) + (assign35190_e40453 * locals.var_dnm_dn6)) * locals.var_arg) - (assign35190_e40455 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn7) * locals.var_dnm) + (assign35190_e40453 * locals.var_dnm_dn7)) * locals.var_arg) - (assign35190_e40455 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn8) * locals.var_dnm) + (assign35190_e40453 * locals.var_dnm_dn8)) * locals.var_arg) - (assign35190_e40455 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn9) * locals.var_dnm) + (assign35190_e40453 * locals.var_dnm_dn9)) * locals.var_arg) - (assign35190_e40455 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn10) * locals.var_dnm) + (assign35190_e40453 * locals.var_dnm_dn10)) * locals.var_arg) - (assign35190_e40455 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn11) * locals.var_dnm) + (assign35190_e40453 * locals.var_dnm_dn11)) * locals.var_arg) - (assign35190_e40455 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn14) * locals.var_dnm) + (assign35190_e40453 * locals.var_dnm_dn14)) * locals.var_arg) - (assign35190_e40455 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign35190_e40459;
        locals.var_t0_dn0 = assign35190_e40459_d_n0;
        locals.var_t0_dn2 = assign35190_e40459_d_n2;
        locals.var_t0_dn4 = assign35190_e40459_d_n4;
        locals.var_t0_dn5 = assign35190_e40459_d_n5;
        locals.var_t0_dn6 = assign35190_e40459_d_n6;
        locals.var_t0_dn7 = assign35190_e40459_d_n7;
        locals.var_t0_dn8 = assign35190_e40459_d_n8;
        locals.var_t0_dn9 = assign35190_e40459_d_n9;
        locals.var_t0_dn10 = assign35190_e40459_d_n10;
        locals.var_t0_dn11 = assign35190_e40459_d_n11;
        locals.var_t0_dn14 = assign35190_e40459_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign35200_e40473, assign35200_e40473_d_n0, assign35200_e40473_d_n2, assign35200_e40473_d_n4, assign35200_e40473_d_n5, assign35200_e40473_d_n6, assign35200_e40473_d_n7, assign35200_e40473_d_n8, assign35200_e40473_d_n9, assign35200_e40473_d_n10, assign35200_e40473_d_n11, assign35200_e40473_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 != 0.0)) {
        let assign35200_e40469: f64 = 2.0;
        let assign35200_e40471: f64 = (assign35200_e40469 - locals.var_tmf0);
        (assign35200_e40471, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign35200_e40473;
        locals.var_t9_dn0 = assign35200_e40473_d_n0;
        locals.var_t9_dn2 = assign35200_e40473_d_n2;
        locals.var_t9_dn4 = assign35200_e40473_d_n4;
        locals.var_t9_dn5 = assign35200_e40473_d_n5;
        locals.var_t9_dn6 = assign35200_e40473_d_n6;
        locals.var_t9_dn7 = assign35200_e40473_d_n7;
        locals.var_t9_dn8 = assign35200_e40473_d_n8;
        locals.var_t9_dn9 = assign35200_e40473_d_n9;
        locals.var_t9_dn10 = assign35200_e40473_d_n10;
        locals.var_t9_dn11 = assign35200_e40473_d_n11;
        locals.var_t9_dn14 = assign35200_e40473_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign35210_e40483, assign35210_e40483_d_n0, assign35210_e40483_d_n2, assign35210_e40483_d_n4, assign35210_e40483_d_n5, assign35210_e40483_d_n6, assign35210_e40483_d_n7, assign35210_e40483_d_n8, assign35210_e40483_d_n9, assign35210_e40483_d_n10, assign35210_e40483_d_n11, assign35210_e40483_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign35210_e40483;
        locals.var_t0_dn0 = assign35210_e40483_d_n0;
        locals.var_t0_dn2 = assign35210_e40483_d_n2;
        locals.var_t0_dn4 = assign35210_e40483_d_n4;
        locals.var_t0_dn5 = assign35210_e40483_d_n5;
        locals.var_t0_dn6 = assign35210_e40483_d_n6;
        locals.var_t0_dn7 = assign35210_e40483_d_n7;
        locals.var_t0_dn8 = assign35210_e40483_d_n8;
        locals.var_t0_dn9 = assign35210_e40483_d_n9;
        locals.var_t0_dn10 = assign35210_e40483_d_n10;
        locals.var_t0_dn11 = assign35210_e40483_d_n11;
        locals.var_t0_dn14 = assign35210_e40483_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign35220_e40494, assign35220_e40494_d_n0, assign35220_e40494_d_n2, assign35220_e40494_d_n4, assign35220_e40494_d_n5, assign35220_e40494_d_n6, assign35220_e40494_d_n7, assign35220_e40494_d_n8, assign35220_e40494_d_n9, assign35220_e40494_d_n10, assign35220_e40494_d_n11, assign35220_e40494_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 == 0.0)) {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign35220_e40494;
        locals.var_t9_dn0 = assign35220_e40494_d_n0;
        locals.var_t9_dn2 = assign35220_e40494_d_n2;
        locals.var_t9_dn4 = assign35220_e40494_d_n4;
        locals.var_t9_dn5 = assign35220_e40494_d_n5;
        locals.var_t9_dn6 = assign35220_e40494_d_n6;
        locals.var_t9_dn7 = assign35220_e40494_d_n7;
        locals.var_t9_dn8 = assign35220_e40494_d_n8;
        locals.var_t9_dn9 = assign35220_e40494_d_n9;
        locals.var_t9_dn10 = assign35220_e40494_d_n10;
        locals.var_t9_dn11 = assign35220_e40494_d_n11;
        locals.var_t9_dn14 = assign35220_e40494_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign35230_e40505, assign35230_e40505_d_n0, assign35230_e40505_d_n2, assign35230_e40505_d_n4, assign35230_e40505_d_n5, assign35230_e40505_d_n6, assign35230_e40505_d_n7, assign35230_e40505_d_n8, assign35230_e40505_d_n9, assign35230_e40505_d_n10, assign35230_e40505_d_n11, assign35230_e40505_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign35230_e40505;
        locals.var_t0_dn0 = assign35230_e40505_d_n0;
        locals.var_t0_dn2 = assign35230_e40505_d_n2;
        locals.var_t0_dn4 = assign35230_e40505_d_n4;
        locals.var_t0_dn5 = assign35230_e40505_d_n5;
        locals.var_t0_dn6 = assign35230_e40505_d_n6;
        locals.var_t0_dn7 = assign35230_e40505_d_n7;
        locals.var_t0_dn8 = assign35230_e40505_d_n8;
        locals.var_t0_dn9 = assign35230_e40505_d_n9;
        locals.var_t0_dn10 = assign35230_e40505_d_n10;
        locals.var_t0_dn11 = assign35230_e40505_d_n11;
        locals.var_t0_dn14 = assign35230_e40505_d_n14;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_115(
        locals: &mut StampLocals,
    ) {
        let (assign35240_e40515, assign35240_e40515_d_n0, assign35240_e40515_d_n2, assign35240_e40515_d_n4, assign35240_e40515_d_n5, assign35240_e40515_d_n6, assign35240_e40515_d_n7, assign35240_e40515_d_n8, assign35240_e40515_d_n9, assign35240_e40515_d_n10, assign35240_e40515_d_n11, assign35240_e40515_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) {
        let assign35240_e40513: f64 = (locals.var_t9 + 1e-25);
        (assign35240_e40513, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign35240_e40515;
        locals.var_t9_dn0 = assign35240_e40515_d_n0;
        locals.var_t9_dn2 = assign35240_e40515_d_n2;
        locals.var_t9_dn4 = assign35240_e40515_d_n4;
        locals.var_t9_dn5 = assign35240_e40515_d_n5;
        locals.var_t9_dn6 = assign35240_e40515_d_n6;
        locals.var_t9_dn7 = assign35240_e40515_d_n7;
        locals.var_t9_dn8 = assign35240_e40515_d_n8;
        locals.var_t9_dn9 = assign35240_e40515_d_n9;
        locals.var_t9_dn10 = assign35240_e40515_d_n10;
        locals.var_t9_dn11 = assign35240_e40515_d_n11;
        locals.var_t9_dn14 = assign35240_e40515_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign35250_e40524, assign35250_e40524_d_n0, assign35250_e40524_d_n2, assign35250_e40524_d_n4, assign35250_e40524_d_n5, assign35250_e40524_d_n6, assign35250_e40524_d_n7, assign35250_e40524_d_n8, assign35250_e40524_d_n9, assign35250_e40524_d_n10, assign35250_e40524_d_n11, assign35250_e40524_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) {
        let assign35250_e40522: f64 = (locals.var_t9).sqrt();
        (assign35250_e40522, (locals.var_t9_dn0 / (2.0 * assign35250_e40522)), (locals.var_t9_dn2 / (2.0 * assign35250_e40522)), (locals.var_t9_dn4 / (2.0 * assign35250_e40522)), (locals.var_t9_dn5 / (2.0 * assign35250_e40522)), (locals.var_t9_dn6 / (2.0 * assign35250_e40522)), (locals.var_t9_dn7 / (2.0 * assign35250_e40522)), (locals.var_t9_dn8 / (2.0 * assign35250_e40522)), (locals.var_t9_dn9 / (2.0 * assign35250_e40522)), (locals.var_t9_dn10 / (2.0 * assign35250_e40522)), (locals.var_t9_dn11 / (2.0 * assign35250_e40522)), (locals.var_t9_dn14 / (2.0 * assign35250_e40522)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign35250_e40524;
        locals.var_t3_dn0 = assign35250_e40524_d_n0;
        locals.var_t3_dn2 = assign35250_e40524_d_n2;
        locals.var_t3_dn4 = assign35250_e40524_d_n4;
        locals.var_t3_dn5 = assign35250_e40524_d_n5;
        locals.var_t3_dn6 = assign35250_e40524_d_n6;
        locals.var_t3_dn7 = assign35250_e40524_d_n7;
        locals.var_t3_dn8 = assign35250_e40524_d_n8;
        locals.var_t3_dn9 = assign35250_e40524_d_n9;
        locals.var_t3_dn10 = assign35250_e40524_d_n10;
        locals.var_t3_dn11 = assign35250_e40524_d_n11;
        locals.var_t3_dn14 = assign35250_e40524_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign35260_e40536, assign35260_e40536_d_n0, assign35260_e40536_d_n2, assign35260_e40536_d_n4, assign35260_e40536_d_n5, assign35260_e40536_d_n6, assign35260_e40536_d_n7, assign35260_e40536_d_n8, assign35260_e40536_d_n9, assign35260_e40536_d_n10, assign35260_e40536_d_n11, assign35260_e40536_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) {
        let assign35260_e40533: f64 = (1.0 - locals.var_t3);
        let assign35260_e40534: f64 = (locals.var_t2 * assign35260_e40533);
        (assign35260_e40534, ((locals.var_t2_dn0 * assign35260_e40533) + (locals.var_t2 * (-locals.var_t3_dn0))), ((locals.var_t2_dn2 * assign35260_e40533) + (locals.var_t2 * (-locals.var_t3_dn2))), ((locals.var_t2_dn4 * assign35260_e40533) + (locals.var_t2 * (-locals.var_t3_dn4))), ((locals.var_t2_dn5 * assign35260_e40533) + (locals.var_t2 * (-locals.var_t3_dn5))), ((locals.var_t2_dn6 * assign35260_e40533) + (locals.var_t2 * (-locals.var_t3_dn6))), ((locals.var_t2_dn7 * assign35260_e40533) + (locals.var_t2 * (-locals.var_t3_dn7))), ((locals.var_t2_dn8 * assign35260_e40533) + (locals.var_t2 * (-locals.var_t3_dn8))), ((locals.var_t2_dn9 * assign35260_e40533) + (locals.var_t2 * (-locals.var_t3_dn9))), ((locals.var_t2_dn10 * assign35260_e40533) + (locals.var_t2 * (-locals.var_t3_dn10))), ((locals.var_t2_dn11 * assign35260_e40533) + (locals.var_t2 * (-locals.var_t3_dn11))), ((locals.var_t2_dn14 * assign35260_e40533) + (locals.var_t2 * (-locals.var_t3_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign35260_e40536;
        locals.var_t4_dn0 = assign35260_e40536_d_n0;
        locals.var_t4_dn2 = assign35260_e40536_d_n2;
        locals.var_t4_dn4 = assign35260_e40536_d_n4;
        locals.var_t4_dn5 = assign35260_e40536_d_n5;
        locals.var_t4_dn6 = assign35260_e40536_d_n6;
        locals.var_t4_dn7 = assign35260_e40536_d_n7;
        locals.var_t4_dn8 = assign35260_e40536_d_n8;
        locals.var_t4_dn9 = assign35260_e40536_d_n9;
        locals.var_t4_dn10 = assign35260_e40536_d_n10;
        locals.var_t4_dn11 = assign35260_e40536_d_n11;
        locals.var_t4_dn14 = assign35260_e40536_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign35270_e40548, assign35270_e40548_d_n0, assign35270_e40548_d_n2, assign35270_e40548_d_n4, assign35270_e40548_d_n5, assign35270_e40548_d_n6, assign35270_e40548_d_n7, assign35270_e40548_d_n8, assign35270_e40548_d_n9, assign35270_e40548_d_n10, assign35270_e40548_d_n11, assign35270_e40548_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) {
        let assign35270_e40544: f64 = (locals.var_vgp + locals.var_uc_depvdsef1);
        let assign35270_e40546: f64 = (assign35270_e40544 + locals.var_t4);
        (assign35270_e40546, ((locals.var_vgp_dn0 + locals.var_uc_depvdsef1_dn0) + locals.var_t4_dn0), ((locals.var_vgp_dn2 + locals.var_uc_depvdsef1_dn2) + locals.var_t4_dn2), ((locals.var_vgp_dn4 + locals.var_uc_depvdsef1_dn4) + locals.var_t4_dn4), ((locals.var_vgp_dn5 + locals.var_uc_depvdsef1_dn5) + locals.var_t4_dn5), ((locals.var_vgp_dn6 + locals.var_uc_depvdsef1_dn6) + locals.var_t4_dn6), ((locals.var_vgp_dn7 + locals.var_uc_depvdsef1_dn7) + locals.var_t4_dn7), ((locals.var_vgp_dn8 + locals.var_uc_depvdsef1_dn8) + locals.var_t4_dn8), ((locals.var_vgp_dn9 + locals.var_uc_depvdsef1_dn9) + locals.var_t4_dn9), ((locals.var_vgp_dn10 + locals.var_uc_depvdsef1_dn10) + locals.var_t4_dn10), ((locals.var_vgp_dn11 + locals.var_uc_depvdsef1_dn11) + locals.var_t4_dn11), ((locals.var_vgp_dn14 + locals.var_uc_depvdsef1_dn14) + locals.var_t4_dn14),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign35270_e40548;
        locals.var_t10_dn0 = assign35270_e40548_d_n0;
        locals.var_t10_dn2 = assign35270_e40548_d_n2;
        locals.var_t10_dn4 = assign35270_e40548_d_n4;
        locals.var_t10_dn5 = assign35270_e40548_d_n5;
        locals.var_t10_dn6 = assign35270_e40548_d_n6;
        locals.var_t10_dn7 = assign35270_e40548_d_n7;
        locals.var_t10_dn8 = assign35270_e40548_d_n8;
        locals.var_t10_dn9 = assign35270_e40548_d_n9;
        locals.var_t10_dn10 = assign35270_e40548_d_n10;
        locals.var_t10_dn11 = assign35270_e40548_d_n11;
        locals.var_t10_dn14 = assign35270_e40548_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign35280_e40558, assign35280_e40558_d_n0, assign35280_e40558_d_n2, assign35280_e40558_d_n4, assign35280_e40558_d_n5, assign35280_e40558_d_n6, assign35280_e40558_d_n7, assign35280_e40558_d_n8, assign35280_e40558_d_n9, assign35280_e40558_d_n10, assign35280_e40558_d_n11, assign35280_e40558_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) {
        let assign35280_e40556: f64 = (locals.var_t10 * locals.var_uc_depvdsef2);
        (assign35280_e40556, ((locals.var_t10_dn0 * locals.var_uc_depvdsef2) + (locals.var_t10 * locals.var_uc_depvdsef2_dn0)), ((locals.var_t10_dn2 * locals.var_uc_depvdsef2) + (locals.var_t10 * locals.var_uc_depvdsef2_dn2)), ((locals.var_t10_dn4 * locals.var_uc_depvdsef2) + (locals.var_t10 * locals.var_uc_depvdsef2_dn4)), ((locals.var_t10_dn5 * locals.var_uc_depvdsef2) + (locals.var_t10 * locals.var_uc_depvdsef2_dn5)), ((locals.var_t10_dn6 * locals.var_uc_depvdsef2) + (locals.var_t10 * locals.var_uc_depvdsef2_dn6)), ((locals.var_t10_dn7 * locals.var_uc_depvdsef2) + (locals.var_t10 * locals.var_uc_depvdsef2_dn7)), ((locals.var_t10_dn8 * locals.var_uc_depvdsef2) + (locals.var_t10 * locals.var_uc_depvdsef2_dn8)), ((locals.var_t10_dn9 * locals.var_uc_depvdsef2) + (locals.var_t10 * locals.var_uc_depvdsef2_dn9)), ((locals.var_t10_dn10 * locals.var_uc_depvdsef2) + (locals.var_t10 * locals.var_uc_depvdsef2_dn10)), ((locals.var_t10_dn11 * locals.var_uc_depvdsef2) + (locals.var_t10 * locals.var_uc_depvdsef2_dn11)), ((locals.var_t10_dn14 * locals.var_uc_depvdsef2) + (locals.var_t10 * locals.var_uc_depvdsef2_dn14)),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign35280_e40558;
        locals.var_t10_dn0 = assign35280_e40558_d_n0;
        locals.var_t10_dn2 = assign35280_e40558_d_n2;
        locals.var_t10_dn4 = assign35280_e40558_d_n4;
        locals.var_t10_dn5 = assign35280_e40558_d_n5;
        locals.var_t10_dn6 = assign35280_e40558_d_n6;
        locals.var_t10_dn7 = assign35280_e40558_d_n7;
        locals.var_t10_dn8 = assign35280_e40558_d_n8;
        locals.var_t10_dn9 = assign35280_e40558_d_n9;
        locals.var_t10_dn10 = assign35280_e40558_d_n10;
        locals.var_t10_dn11 = assign35280_e40558_d_n11;
        locals.var_t10_dn14 = assign35280_e40558_d_n14;
        locals.var_t10_rv = 0.0;

        let assign35290_e40562: f64 = (locals.var_uc_depleak + 4.0);
        let assign35290_e40567: f64 = if ((locals.var_t10 < assign35290_e40562) && (4.0 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard817 = assign35290_e40567;
        locals.var_guard817_rv = 0.0;

        let (assign35300_e40581, assign35300_e40581_d_n0, assign35300_e40581_d_n2, assign35300_e40581_d_n4, assign35300_e40581_d_n5, assign35300_e40581_d_n6, assign35300_e40581_d_n7, assign35300_e40581_d_n8, assign35300_e40581_d_n9, assign35300_e40581_d_n10, assign35300_e40581_d_n11, assign35300_e40581_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 != 0.0)) {
        let assign35300_e40577: f64 = (locals.var_uc_depleak + 4.0);
        let assign35300_e40579: f64 = (assign35300_e40577 - locals.var_t10);
        (assign35300_e40579, (locals.var_uc_depleak_dn0 - locals.var_t10_dn0), (locals.var_uc_depleak_dn2 - locals.var_t10_dn2), (locals.var_uc_depleak_dn4 - locals.var_t10_dn4), (locals.var_uc_depleak_dn5 - locals.var_t10_dn5), (locals.var_uc_depleak_dn6 - locals.var_t10_dn6), (locals.var_uc_depleak_dn7 - locals.var_t10_dn7), (locals.var_uc_depleak_dn8 - locals.var_t10_dn8), (locals.var_uc_depleak_dn9 - locals.var_t10_dn9), (locals.var_uc_depleak_dn10 - locals.var_t10_dn10), (locals.var_uc_depleak_dn11 - locals.var_t10_dn11), (locals.var_uc_depleak_dn14 - locals.var_t10_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign35300_e40581;
        locals.var_tmf1_dn0 = assign35300_e40581_d_n0;
        locals.var_tmf1_dn2 = assign35300_e40581_d_n2;
        locals.var_tmf1_dn4 = assign35300_e40581_d_n4;
        locals.var_tmf1_dn5 = assign35300_e40581_d_n5;
        locals.var_tmf1_dn6 = assign35300_e40581_d_n6;
        locals.var_tmf1_dn7 = assign35300_e40581_d_n7;
        locals.var_tmf1_dn8 = assign35300_e40581_d_n8;
        locals.var_tmf1_dn9 = assign35300_e40581_d_n9;
        locals.var_tmf1_dn10 = assign35300_e40581_d_n10;
        locals.var_tmf1_dn11 = assign35300_e40581_d_n11;
        locals.var_tmf1_dn14 = assign35300_e40581_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign35310_e40593, assign35310_e40593_d_n0, assign35310_e40593_d_n2, assign35310_e40593_d_n4, assign35310_e40593_d_n5, assign35310_e40593_d_n6, assign35310_e40593_d_n7, assign35310_e40593_d_n8, assign35310_e40593_d_n9, assign35310_e40593_d_n10, assign35310_e40593_d_n11, assign35310_e40593_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 != 0.0)) {
        let assign35310_e40591: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign35310_e40591, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign35310_e40593;
        locals.var_x2_dn0 = assign35310_e40593_d_n0;
        locals.var_x2_dn2 = assign35310_e40593_d_n2;
        locals.var_x2_dn4 = assign35310_e40593_d_n4;
        locals.var_x2_dn5 = assign35310_e40593_d_n5;
        locals.var_x2_dn6 = assign35310_e40593_d_n6;
        locals.var_x2_dn7 = assign35310_e40593_d_n7;
        locals.var_x2_dn8 = assign35310_e40593_d_n8;
        locals.var_x2_dn9 = assign35310_e40593_d_n9;
        locals.var_x2_dn10 = assign35310_e40593_d_n10;
        locals.var_x2_dn11 = assign35310_e40593_d_n11;
        locals.var_x2_dn14 = assign35310_e40593_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign35320_e40605, assign35320_e40605_d_n0, assign35320_e40605_d_n2, assign35320_e40605_d_n4, assign35320_e40605_d_n5, assign35320_e40605_d_n6, assign35320_e40605_d_n7, assign35320_e40605_d_n8, assign35320_e40605_d_n9, assign35320_e40605_d_n10, assign35320_e40605_d_n11, assign35320_e40605_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 != 0.0)) {
        let assign35320_e40603: f64 = (4.0 * 4.0);
        (assign35320_e40603, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign35320_e40605;
        locals.var_xmax2_dn0 = assign35320_e40605_d_n0;
        locals.var_xmax2_dn2 = assign35320_e40605_d_n2;
        locals.var_xmax2_dn4 = assign35320_e40605_d_n4;
        locals.var_xmax2_dn5 = assign35320_e40605_d_n5;
        locals.var_xmax2_dn6 = assign35320_e40605_d_n6;
        locals.var_xmax2_dn7 = assign35320_e40605_d_n7;
        locals.var_xmax2_dn8 = assign35320_e40605_d_n8;
        locals.var_xmax2_dn9 = assign35320_e40605_d_n9;
        locals.var_xmax2_dn10 = assign35320_e40605_d_n10;
        locals.var_xmax2_dn11 = assign35320_e40605_d_n11;
        locals.var_xmax2_dn14 = assign35320_e40605_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign35330_e40615, assign35330_e40615_d_n0, assign35330_e40615_d_n2, assign35330_e40615_d_n4, assign35330_e40615_d_n5, assign35330_e40615_d_n6, assign35330_e40615_d_n7, assign35330_e40615_d_n8, assign35330_e40615_d_n9, assign35330_e40615_d_n10, assign35330_e40615_d_n11, assign35330_e40615_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign35330_e40615;
        locals.var_xp_dn0 = assign35330_e40615_d_n0;
        locals.var_xp_dn2 = assign35330_e40615_d_n2;
        locals.var_xp_dn4 = assign35330_e40615_d_n4;
        locals.var_xp_dn5 = assign35330_e40615_d_n5;
        locals.var_xp_dn6 = assign35330_e40615_d_n6;
        locals.var_xp_dn7 = assign35330_e40615_d_n7;
        locals.var_xp_dn8 = assign35330_e40615_d_n8;
        locals.var_xp_dn9 = assign35330_e40615_d_n9;
        locals.var_xp_dn10 = assign35330_e40615_d_n10;
        locals.var_xp_dn11 = assign35330_e40615_d_n11;
        locals.var_xp_dn14 = assign35330_e40615_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign35340_e40625, assign35340_e40625_d_n0, assign35340_e40625_d_n2, assign35340_e40625_d_n4, assign35340_e40625_d_n5, assign35340_e40625_d_n6, assign35340_e40625_d_n7, assign35340_e40625_d_n8, assign35340_e40625_d_n9, assign35340_e40625_d_n10, assign35340_e40625_d_n11, assign35340_e40625_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign35340_e40625;
        locals.var_xmp_dn0 = assign35340_e40625_d_n0;
        locals.var_xmp_dn2 = assign35340_e40625_d_n2;
        locals.var_xmp_dn4 = assign35340_e40625_d_n4;
        locals.var_xmp_dn5 = assign35340_e40625_d_n5;
        locals.var_xmp_dn6 = assign35340_e40625_d_n6;
        locals.var_xmp_dn7 = assign35340_e40625_d_n7;
        locals.var_xmp_dn8 = assign35340_e40625_d_n8;
        locals.var_xmp_dn9 = assign35340_e40625_d_n9;
        locals.var_xmp_dn10 = assign35340_e40625_d_n10;
        locals.var_xmp_dn11 = assign35340_e40625_d_n11;
        locals.var_xmp_dn14 = assign35340_e40625_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign35350_e40635,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign35350_e40635;
        locals.var_m0_rv = 0.0;

        let (assign35360_e40645,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign35360_e40645;
        locals.var_mm_rv = 0.0;

        let (assign35370_e40655, assign35370_e40655_d_n0, assign35370_e40655_d_n2, assign35370_e40655_d_n4, assign35370_e40655_d_n5, assign35370_e40655_d_n6, assign35370_e40655_d_n7, assign35370_e40655_d_n8, assign35370_e40655_d_n9, assign35370_e40655_d_n10, assign35370_e40655_d_n11, assign35370_e40655_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign35370_e40655;
        locals.var_arg_dn0 = assign35370_e40655_d_n0;
        locals.var_arg_dn2 = assign35370_e40655_d_n2;
        locals.var_arg_dn4 = assign35370_e40655_d_n4;
        locals.var_arg_dn5 = assign35370_e40655_d_n5;
        locals.var_arg_dn6 = assign35370_e40655_d_n6;
        locals.var_arg_dn7 = assign35370_e40655_d_n7;
        locals.var_arg_dn8 = assign35370_e40655_d_n8;
        locals.var_arg_dn9 = assign35370_e40655_d_n9;
        locals.var_arg_dn10 = assign35370_e40655_d_n10;
        locals.var_arg_dn11 = assign35370_e40655_d_n11;
        locals.var_arg_dn14 = assign35370_e40655_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign35380_e40665, assign35380_e40665_d_n0, assign35380_e40665_d_n2, assign35380_e40665_d_n4, assign35380_e40665_d_n5, assign35380_e40665_d_n6, assign35380_e40665_d_n7, assign35380_e40665_d_n8, assign35380_e40665_d_n9, assign35380_e40665_d_n10, assign35380_e40665_d_n11, assign35380_e40665_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign35380_e40665;
        locals.var_dnm_dn0 = assign35380_e40665_d_n0;
        locals.var_dnm_dn2 = assign35380_e40665_d_n2;
        locals.var_dnm_dn4 = assign35380_e40665_d_n4;
        locals.var_dnm_dn5 = assign35380_e40665_d_n5;
        locals.var_dnm_dn6 = assign35380_e40665_d_n6;
        locals.var_dnm_dn7 = assign35380_e40665_d_n7;
        locals.var_dnm_dn8 = assign35380_e40665_d_n8;
        locals.var_dnm_dn9 = assign35380_e40665_d_n9;
        locals.var_dnm_dn10 = assign35380_e40665_d_n10;
        locals.var_dnm_dn11 = assign35380_e40665_d_n11;
        locals.var_dnm_dn14 = assign35380_e40665_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign35390_e40677, assign35390_e40677_d_n0, assign35390_e40677_d_n2, assign35390_e40677_d_n4, assign35390_e40677_d_n5, assign35390_e40677_d_n6, assign35390_e40677_d_n7, assign35390_e40677_d_n8, assign35390_e40677_d_n9, assign35390_e40677_d_n10, assign35390_e40677_d_n11, assign35390_e40677_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 != 0.0)) {
        let assign35390_e40675: f64 = (locals.var_xp * locals.var_x2);
        (assign35390_e40675, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign35390_e40677;
        locals.var_xp_dn0 = assign35390_e40677_d_n0;
        locals.var_xp_dn2 = assign35390_e40677_d_n2;
        locals.var_xp_dn4 = assign35390_e40677_d_n4;
        locals.var_xp_dn5 = assign35390_e40677_d_n5;
        locals.var_xp_dn6 = assign35390_e40677_d_n6;
        locals.var_xp_dn7 = assign35390_e40677_d_n7;
        locals.var_xp_dn8 = assign35390_e40677_d_n8;
        locals.var_xp_dn9 = assign35390_e40677_d_n9;
        locals.var_xp_dn10 = assign35390_e40677_d_n10;
        locals.var_xp_dn11 = assign35390_e40677_d_n11;
        locals.var_xp_dn14 = assign35390_e40677_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign35400_e40689, assign35400_e40689_d_n0, assign35400_e40689_d_n2, assign35400_e40689_d_n4, assign35400_e40689_d_n5, assign35400_e40689_d_n6, assign35400_e40689_d_n7, assign35400_e40689_d_n8, assign35400_e40689_d_n9, assign35400_e40689_d_n10, assign35400_e40689_d_n11, assign35400_e40689_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 != 0.0)) {
        let assign35400_e40687: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign35400_e40687, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign35400_e40689;
        locals.var_xmp_dn0 = assign35400_e40689_d_n0;
        locals.var_xmp_dn2 = assign35400_e40689_d_n2;
        locals.var_xmp_dn4 = assign35400_e40689_d_n4;
        locals.var_xmp_dn5 = assign35400_e40689_d_n5;
        locals.var_xmp_dn6 = assign35400_e40689_d_n6;
        locals.var_xmp_dn7 = assign35400_e40689_d_n7;
        locals.var_xmp_dn8 = assign35400_e40689_d_n8;
        locals.var_xmp_dn9 = assign35400_e40689_d_n9;
        locals.var_xmp_dn10 = assign35400_e40689_d_n10;
        locals.var_xmp_dn11 = assign35400_e40689_d_n11;
        locals.var_xmp_dn14 = assign35400_e40689_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign35410_e40701, assign35410_e40701_d_n0, assign35410_e40701_d_n2, assign35410_e40701_d_n4, assign35410_e40701_d_n5, assign35410_e40701_d_n6, assign35410_e40701_d_n7, assign35410_e40701_d_n8, assign35410_e40701_d_n9, assign35410_e40701_d_n10, assign35410_e40701_d_n11, assign35410_e40701_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 != 0.0)) {
        let assign35410_e40699: f64 = (locals.var_xp * locals.var_x2);
        (assign35410_e40699, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign35410_e40701;
        locals.var_xp_dn0 = assign35410_e40701_d_n0;
        locals.var_xp_dn2 = assign35410_e40701_d_n2;
        locals.var_xp_dn4 = assign35410_e40701_d_n4;
        locals.var_xp_dn5 = assign35410_e40701_d_n5;
        locals.var_xp_dn6 = assign35410_e40701_d_n6;
        locals.var_xp_dn7 = assign35410_e40701_d_n7;
        locals.var_xp_dn8 = assign35410_e40701_d_n8;
        locals.var_xp_dn9 = assign35410_e40701_d_n9;
        locals.var_xp_dn10 = assign35410_e40701_d_n10;
        locals.var_xp_dn11 = assign35410_e40701_d_n11;
        locals.var_xp_dn14 = assign35410_e40701_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign35420_e40713, assign35420_e40713_d_n0, assign35420_e40713_d_n2, assign35420_e40713_d_n4, assign35420_e40713_d_n5, assign35420_e40713_d_n6, assign35420_e40713_d_n7, assign35420_e40713_d_n8, assign35420_e40713_d_n9, assign35420_e40713_d_n10, assign35420_e40713_d_n11, assign35420_e40713_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 != 0.0)) {
        let assign35420_e40711: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign35420_e40711, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign35420_e40713;
        locals.var_xmp_dn0 = assign35420_e40713_d_n0;
        locals.var_xmp_dn2 = assign35420_e40713_d_n2;
        locals.var_xmp_dn4 = assign35420_e40713_d_n4;
        locals.var_xmp_dn5 = assign35420_e40713_d_n5;
        locals.var_xmp_dn6 = assign35420_e40713_d_n6;
        locals.var_xmp_dn7 = assign35420_e40713_d_n7;
        locals.var_xmp_dn8 = assign35420_e40713_d_n8;
        locals.var_xmp_dn9 = assign35420_e40713_d_n9;
        locals.var_xmp_dn10 = assign35420_e40713_d_n10;
        locals.var_xmp_dn11 = assign35420_e40713_d_n11;
        locals.var_xmp_dn14 = assign35420_e40713_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign35430_e40725, assign35430_e40725_d_n0, assign35430_e40725_d_n2, assign35430_e40725_d_n4, assign35430_e40725_d_n5, assign35430_e40725_d_n6, assign35430_e40725_d_n7, assign35430_e40725_d_n8, assign35430_e40725_d_n9, assign35430_e40725_d_n10, assign35430_e40725_d_n11, assign35430_e40725_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 != 0.0)) {
        let assign35430_e40723: f64 = (locals.var_xp * locals.var_x2);
        (assign35430_e40723, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign35430_e40725;
        locals.var_xp_dn0 = assign35430_e40725_d_n0;
        locals.var_xp_dn2 = assign35430_e40725_d_n2;
        locals.var_xp_dn4 = assign35430_e40725_d_n4;
        locals.var_xp_dn5 = assign35430_e40725_d_n5;
        locals.var_xp_dn6 = assign35430_e40725_d_n6;
        locals.var_xp_dn7 = assign35430_e40725_d_n7;
        locals.var_xp_dn8 = assign35430_e40725_d_n8;
        locals.var_xp_dn9 = assign35430_e40725_d_n9;
        locals.var_xp_dn10 = assign35430_e40725_d_n10;
        locals.var_xp_dn11 = assign35430_e40725_d_n11;
        locals.var_xp_dn14 = assign35430_e40725_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign35440_e40737, assign35440_e40737_d_n0, assign35440_e40737_d_n2, assign35440_e40737_d_n4, assign35440_e40737_d_n5, assign35440_e40737_d_n6, assign35440_e40737_d_n7, assign35440_e40737_d_n8, assign35440_e40737_d_n9, assign35440_e40737_d_n10, assign35440_e40737_d_n11, assign35440_e40737_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 != 0.0)) {
        let assign35440_e40735: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign35440_e40735, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign35440_e40737;
        locals.var_xmp_dn0 = assign35440_e40737_d_n0;
        locals.var_xmp_dn2 = assign35440_e40737_d_n2;
        locals.var_xmp_dn4 = assign35440_e40737_d_n4;
        locals.var_xmp_dn5 = assign35440_e40737_d_n5;
        locals.var_xmp_dn6 = assign35440_e40737_d_n6;
        locals.var_xmp_dn7 = assign35440_e40737_d_n7;
        locals.var_xmp_dn8 = assign35440_e40737_d_n8;
        locals.var_xmp_dn9 = assign35440_e40737_d_n9;
        locals.var_xmp_dn10 = assign35440_e40737_d_n10;
        locals.var_xmp_dn11 = assign35440_e40737_d_n11;
        locals.var_xmp_dn14 = assign35440_e40737_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign35450_e40749, assign35450_e40749_d_n0, assign35450_e40749_d_n2, assign35450_e40749_d_n4, assign35450_e40749_d_n5, assign35450_e40749_d_n6, assign35450_e40749_d_n7, assign35450_e40749_d_n8, assign35450_e40749_d_n9, assign35450_e40749_d_n10, assign35450_e40749_d_n11, assign35450_e40749_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 != 0.0)) {
        let assign35450_e40747: f64 = (locals.var_xp * locals.var_x2);
        (assign35450_e40747, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign35450_e40749;
        locals.var_xp_dn0 = assign35450_e40749_d_n0;
        locals.var_xp_dn2 = assign35450_e40749_d_n2;
        locals.var_xp_dn4 = assign35450_e40749_d_n4;
        locals.var_xp_dn5 = assign35450_e40749_d_n5;
        locals.var_xp_dn6 = assign35450_e40749_d_n6;
        locals.var_xp_dn7 = assign35450_e40749_d_n7;
        locals.var_xp_dn8 = assign35450_e40749_d_n8;
        locals.var_xp_dn9 = assign35450_e40749_d_n9;
        locals.var_xp_dn10 = assign35450_e40749_d_n10;
        locals.var_xp_dn11 = assign35450_e40749_d_n11;
        locals.var_xp_dn14 = assign35450_e40749_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign35460_e40761, assign35460_e40761_d_n0, assign35460_e40761_d_n2, assign35460_e40761_d_n4, assign35460_e40761_d_n5, assign35460_e40761_d_n6, assign35460_e40761_d_n7, assign35460_e40761_d_n8, assign35460_e40761_d_n9, assign35460_e40761_d_n10, assign35460_e40761_d_n11, assign35460_e40761_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 != 0.0)) {
        let assign35460_e40759: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign35460_e40759, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign35460_e40761;
        locals.var_xmp_dn0 = assign35460_e40761_d_n0;
        locals.var_xmp_dn2 = assign35460_e40761_d_n2;
        locals.var_xmp_dn4 = assign35460_e40761_d_n4;
        locals.var_xmp_dn5 = assign35460_e40761_d_n5;
        locals.var_xmp_dn6 = assign35460_e40761_d_n6;
        locals.var_xmp_dn7 = assign35460_e40761_d_n7;
        locals.var_xmp_dn8 = assign35460_e40761_d_n8;
        locals.var_xmp_dn9 = assign35460_e40761_d_n9;
        locals.var_xmp_dn10 = assign35460_e40761_d_n10;
        locals.var_xmp_dn11 = assign35460_e40761_d_n11;
        locals.var_xmp_dn14 = assign35460_e40761_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign35470_e40773, assign35470_e40773_d_n0, assign35470_e40773_d_n2, assign35470_e40773_d_n4, assign35470_e40773_d_n5, assign35470_e40773_d_n6, assign35470_e40773_d_n7, assign35470_e40773_d_n8, assign35470_e40773_d_n9, assign35470_e40773_d_n10, assign35470_e40773_d_n11, assign35470_e40773_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 != 0.0)) {
        let assign35470_e40771: f64 = (locals.var_xp + locals.var_xmp);
        (assign35470_e40771, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign35470_e40773;
        locals.var_arg_dn0 = assign35470_e40773_d_n0;
        locals.var_arg_dn2 = assign35470_e40773_d_n2;
        locals.var_arg_dn4 = assign35470_e40773_d_n4;
        locals.var_arg_dn5 = assign35470_e40773_d_n5;
        locals.var_arg_dn6 = assign35470_e40773_d_n6;
        locals.var_arg_dn7 = assign35470_e40773_d_n7;
        locals.var_arg_dn8 = assign35470_e40773_d_n8;
        locals.var_arg_dn9 = assign35470_e40773_d_n9;
        locals.var_arg_dn10 = assign35470_e40773_d_n10;
        locals.var_arg_dn11 = assign35470_e40773_d_n11;
        locals.var_arg_dn14 = assign35470_e40773_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign35480_e40783, assign35480_e40783_d_n0, assign35480_e40783_d_n2, assign35480_e40783_d_n4, assign35480_e40783_d_n5, assign35480_e40783_d_n6, assign35480_e40783_d_n7, assign35480_e40783_d_n8, assign35480_e40783_d_n9, assign35480_e40783_d_n10, assign35480_e40783_d_n11, assign35480_e40783_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign35480_e40783;
        locals.var_dnm_dn0 = assign35480_e40783_d_n0;
        locals.var_dnm_dn2 = assign35480_e40783_d_n2;
        locals.var_dnm_dn4 = assign35480_e40783_d_n4;
        locals.var_dnm_dn5 = assign35480_e40783_d_n5;
        locals.var_dnm_dn6 = assign35480_e40783_d_n6;
        locals.var_dnm_dn7 = assign35480_e40783_d_n7;
        locals.var_dnm_dn8 = assign35480_e40783_d_n8;
        locals.var_dnm_dn9 = assign35480_e40783_d_n9;
        locals.var_dnm_dn10 = assign35480_e40783_d_n10;
        locals.var_dnm_dn11 = assign35480_e40783_d_n11;
        locals.var_dnm_dn14 = assign35480_e40783_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign35490_e40798: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard818 = assign35490_e40798;
        locals.var_guard818_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_116(
        locals: &mut StampLocals,
    ) {
        let assign35500_e40801: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard819 = assign35500_e40801;
        locals.var_guard819_rv = 0.0;

        let (assign35510_e40815,) = {
    if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 != 0.0)) && (locals.var_guard818 != 0.0)) && (locals.var_guard819 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign35510_e40815;
        locals.var_mm_rv = 0.0;

        let assign35520_e40818: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard820 = assign35520_e40818;
        locals.var_guard820_rv = 0.0;

        let (assign35530_e40835,) = {
    if (((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 != 0.0)) && (locals.var_guard818 != 0.0)) && (locals.var_guard819 == 0.0)) && (locals.var_guard820 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign35530_e40835;
        locals.var_mm_rv = 0.0;

        let assign35540_e40838: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard821 = assign35540_e40838;
        locals.var_guard821_rv = 0.0;

        let (assign35550_e40858,) = {
    if ((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 != 0.0)) && (locals.var_guard818 != 0.0)) && (locals.var_guard819 == 0.0)) && (locals.var_guard820 == 0.0)) && (locals.var_guard821 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign35550_e40858;
        locals.var_mm_rv = 0.0;

        let assign35560_e40861: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard822 = assign35560_e40861;
        locals.var_guard822_rv = 0.0;

        let (assign35570_e40884,) = {
    if (((((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 != 0.0)) && (locals.var_guard818 != 0.0)) && (locals.var_guard819 == 0.0)) && (locals.var_guard820 == 0.0)) && (locals.var_guard821 == 0.0)) && (locals.var_guard822 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign35570_e40884;
        locals.var_mm_rv = 0.0;

        let (assign35580_e40896,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 != 0.0)) && (locals.var_guard818 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign35580_e40896;
        locals.var_m0_rv = 0.0;

        let mut assign35590_loop_guard: usize = 0;
        while {
            let assign35590_cond_e40909: f64 = if ((((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 != 0.0)) && (locals.var_guard818 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign35590_cond_e40909 != 0.0
        } {
            assign35590_loop_guard += 1;
            assert!(assign35590_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign35590_body0_e40922, assign35590_body0_e40922_d_n0, assign35590_body0_e40922_d_n2, assign35590_body0_e40922_d_n4, assign35590_body0_e40922_d_n5, assign35590_body0_e40922_d_n6, assign35590_body0_e40922_d_n7, assign35590_body0_e40922_d_n8, assign35590_body0_e40922_d_n9, assign35590_body0_e40922_d_n10, assign35590_body0_e40922_d_n11, assign35590_body0_e40922_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 != 0.0)) && (locals.var_guard818 != 0.0)) {
        let assign35590_body0_e40920: f64 = (locals.var_dnm).sqrt();
        (assign35590_body0_e40920, (locals.var_dnm_dn0 / (2.0 * assign35590_body0_e40920)), (locals.var_dnm_dn2 / (2.0 * assign35590_body0_e40920)), (locals.var_dnm_dn4 / (2.0 * assign35590_body0_e40920)), (locals.var_dnm_dn5 / (2.0 * assign35590_body0_e40920)), (locals.var_dnm_dn6 / (2.0 * assign35590_body0_e40920)), (locals.var_dnm_dn7 / (2.0 * assign35590_body0_e40920)), (locals.var_dnm_dn8 / (2.0 * assign35590_body0_e40920)), (locals.var_dnm_dn9 / (2.0 * assign35590_body0_e40920)), (locals.var_dnm_dn10 / (2.0 * assign35590_body0_e40920)), (locals.var_dnm_dn11 / (2.0 * assign35590_body0_e40920)), (locals.var_dnm_dn14 / (2.0 * assign35590_body0_e40920)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign35590_body0_e40922;
            locals.var_dnm_dn0 = assign35590_body0_e40922_d_n0;
            locals.var_dnm_dn2 = assign35590_body0_e40922_d_n2;
            locals.var_dnm_dn4 = assign35590_body0_e40922_d_n4;
            locals.var_dnm_dn5 = assign35590_body0_e40922_d_n5;
            locals.var_dnm_dn6 = assign35590_body0_e40922_d_n6;
            locals.var_dnm_dn7 = assign35590_body0_e40922_d_n7;
            locals.var_dnm_dn8 = assign35590_body0_e40922_d_n8;
            locals.var_dnm_dn9 = assign35590_body0_e40922_d_n9;
            locals.var_dnm_dn10 = assign35590_body0_e40922_d_n10;
            locals.var_dnm_dn11 = assign35590_body0_e40922_d_n11;
            locals.var_dnm_dn14 = assign35590_body0_e40922_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign35590_body1_e40936,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 != 0.0)) && (locals.var_guard818 != 0.0)) {
        let assign35590_body1_e40934: f64 = (locals.var_m0 + 1.0);
        (assign35590_body1_e40934,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign35590_body1_e40936;
            locals.var_m0_rv = 0.0;
        }

        let (assign35600_e40960, assign35600_e40960_d_n0, assign35600_e40960_d_n2, assign35600_e40960_d_n4, assign35600_e40960_d_n5, assign35600_e40960_d_n6, assign35600_e40960_d_n7, assign35600_e40960_d_n8, assign35600_e40960_d_n9, assign35600_e40960_d_n10, assign35600_e40960_d_n11, assign35600_e40960_d_n14,) = {
    if (((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 != 0.0)) && (locals.var_guard818 == 0.0)) {
        let (assign35600_e40958, assign35600_e40958_d_n0, assign35600_e40958_d_n2, assign35600_e40958_d_n4, assign35600_e40958_d_n5, assign35600_e40958_d_n6, assign35600_e40958_d_n7, assign35600_e40958_d_n8, assign35600_e40958_d_n9, assign35600_e40958_d_n10, assign35600_e40958_d_n11, assign35600_e40958_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign35600_e40955: f64 = (2.0 * 4.0);
                let assign35600_e40956: f64 = (1.0 / assign35600_e40955);
                let assign35600_e40957: f64 = (locals.var_dnm).powf(assign35600_e40956);
                (assign35600_e40957, if 0.0 == 0.0 && ((assign35600_e40956) as f64).is_finite() && ((assign35600_e40956) as f64).fract() == 0.0 { if assign35600_e40956 == 0.0 { 0.0 } else { (assign35600_e40956 * ((locals.var_dnm).powf(assign35600_e40956 - 1.0) * locals.var_dnm_dn0)) } } else { (assign35600_e40957 * (assign35600_e40956 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35600_e40956) as f64).is_finite() && ((assign35600_e40956) as f64).fract() == 0.0 { if assign35600_e40956 == 0.0 { 0.0 } else { (assign35600_e40956 * ((locals.var_dnm).powf(assign35600_e40956 - 1.0) * locals.var_dnm_dn2)) } } else { (assign35600_e40957 * (assign35600_e40956 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35600_e40956) as f64).is_finite() && ((assign35600_e40956) as f64).fract() == 0.0 { if assign35600_e40956 == 0.0 { 0.0 } else { (assign35600_e40956 * ((locals.var_dnm).powf(assign35600_e40956 - 1.0) * locals.var_dnm_dn4)) } } else { (assign35600_e40957 * (assign35600_e40956 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35600_e40956) as f64).is_finite() && ((assign35600_e40956) as f64).fract() == 0.0 { if assign35600_e40956 == 0.0 { 0.0 } else { (assign35600_e40956 * ((locals.var_dnm).powf(assign35600_e40956 - 1.0) * locals.var_dnm_dn5)) } } else { (assign35600_e40957 * (assign35600_e40956 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35600_e40956) as f64).is_finite() && ((assign35600_e40956) as f64).fract() == 0.0 { if assign35600_e40956 == 0.0 { 0.0 } else { (assign35600_e40956 * ((locals.var_dnm).powf(assign35600_e40956 - 1.0) * locals.var_dnm_dn6)) } } else { (assign35600_e40957 * (assign35600_e40956 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35600_e40956) as f64).is_finite() && ((assign35600_e40956) as f64).fract() == 0.0 { if assign35600_e40956 == 0.0 { 0.0 } else { (assign35600_e40956 * ((locals.var_dnm).powf(assign35600_e40956 - 1.0) * locals.var_dnm_dn7)) } } else { (assign35600_e40957 * (assign35600_e40956 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35600_e40956) as f64).is_finite() && ((assign35600_e40956) as f64).fract() == 0.0 { if assign35600_e40956 == 0.0 { 0.0 } else { (assign35600_e40956 * ((locals.var_dnm).powf(assign35600_e40956 - 1.0) * locals.var_dnm_dn8)) } } else { (assign35600_e40957 * (assign35600_e40956 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35600_e40956) as f64).is_finite() && ((assign35600_e40956) as f64).fract() == 0.0 { if assign35600_e40956 == 0.0 { 0.0 } else { (assign35600_e40956 * ((locals.var_dnm).powf(assign35600_e40956 - 1.0) * locals.var_dnm_dn9)) } } else { (assign35600_e40957 * (assign35600_e40956 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35600_e40956) as f64).is_finite() && ((assign35600_e40956) as f64).fract() == 0.0 { if assign35600_e40956 == 0.0 { 0.0 } else { (assign35600_e40956 * ((locals.var_dnm).powf(assign35600_e40956 - 1.0) * locals.var_dnm_dn10)) } } else { (assign35600_e40957 * (assign35600_e40956 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35600_e40956) as f64).is_finite() && ((assign35600_e40956) as f64).fract() == 0.0 { if assign35600_e40956 == 0.0 { 0.0 } else { (assign35600_e40956 * ((locals.var_dnm).powf(assign35600_e40956 - 1.0) * locals.var_dnm_dn11)) } } else { (assign35600_e40957 * (assign35600_e40956 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35600_e40956) as f64).is_finite() && ((assign35600_e40956) as f64).fract() == 0.0 { if assign35600_e40956 == 0.0 { 0.0 } else { (assign35600_e40956 * ((locals.var_dnm).powf(assign35600_e40956 - 1.0) * locals.var_dnm_dn14)) } } else { (assign35600_e40957 * (assign35600_e40956 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign35600_e40958, assign35600_e40958_d_n0, assign35600_e40958_d_n2, assign35600_e40958_d_n4, assign35600_e40958_d_n5, assign35600_e40958_d_n6, assign35600_e40958_d_n7, assign35600_e40958_d_n8, assign35600_e40958_d_n9, assign35600_e40958_d_n10, assign35600_e40958_d_n11, assign35600_e40958_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign35600_e40960;
        locals.var_dnm_dn0 = assign35600_e40960_d_n0;
        locals.var_dnm_dn2 = assign35600_e40960_d_n2;
        locals.var_dnm_dn4 = assign35600_e40960_d_n4;
        locals.var_dnm_dn5 = assign35600_e40960_d_n5;
        locals.var_dnm_dn6 = assign35600_e40960_d_n6;
        locals.var_dnm_dn7 = assign35600_e40960_d_n7;
        locals.var_dnm_dn8 = assign35600_e40960_d_n8;
        locals.var_dnm_dn9 = assign35600_e40960_d_n9;
        locals.var_dnm_dn10 = assign35600_e40960_d_n10;
        locals.var_dnm_dn11 = assign35600_e40960_d_n11;
        locals.var_dnm_dn14 = assign35600_e40960_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign35610_e40972, assign35610_e40972_d_n0, assign35610_e40972_d_n2, assign35610_e40972_d_n4, assign35610_e40972_d_n5, assign35610_e40972_d_n6, assign35610_e40972_d_n7, assign35610_e40972_d_n8, assign35610_e40972_d_n9, assign35610_e40972_d_n10, assign35610_e40972_d_n11, assign35610_e40972_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 != 0.0)) {
        let assign35610_e40970: f64 = (1.0 / locals.var_dnm);
        (assign35610_e40970, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign35610_e40972;
        locals.var_dnm_dn0 = assign35610_e40972_d_n0;
        locals.var_dnm_dn2 = assign35610_e40972_d_n2;
        locals.var_dnm_dn4 = assign35610_e40972_d_n4;
        locals.var_dnm_dn5 = assign35610_e40972_d_n5;
        locals.var_dnm_dn6 = assign35610_e40972_d_n6;
        locals.var_dnm_dn7 = assign35610_e40972_d_n7;
        locals.var_dnm_dn8 = assign35610_e40972_d_n8;
        locals.var_dnm_dn9 = assign35610_e40972_d_n9;
        locals.var_dnm_dn10 = assign35610_e40972_d_n10;
        locals.var_dnm_dn11 = assign35610_e40972_d_n11;
        locals.var_dnm_dn14 = assign35610_e40972_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign35620_e40986, assign35620_e40986_d_n0, assign35620_e40986_d_n2, assign35620_e40986_d_n4, assign35620_e40986_d_n5, assign35620_e40986_d_n6, assign35620_e40986_d_n7, assign35620_e40986_d_n8, assign35620_e40986_d_n9, assign35620_e40986_d_n10, assign35620_e40986_d_n11, assign35620_e40986_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 != 0.0)) {
        let assign35620_e40982: f64 = (locals.var_tmf1 * 4.0);
        let assign35620_e40984: f64 = (assign35620_e40982 * locals.var_dnm);
        (assign35620_e40984, (((locals.var_tmf1_dn0 * 4.0) * locals.var_dnm) + (assign35620_e40982 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 4.0) * locals.var_dnm) + (assign35620_e40982 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 4.0) * locals.var_dnm) + (assign35620_e40982 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 4.0) * locals.var_dnm) + (assign35620_e40982 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 4.0) * locals.var_dnm) + (assign35620_e40982 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 4.0) * locals.var_dnm) + (assign35620_e40982 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 4.0) * locals.var_dnm) + (assign35620_e40982 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 4.0) * locals.var_dnm) + (assign35620_e40982 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 4.0) * locals.var_dnm) + (assign35620_e40982 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 4.0) * locals.var_dnm) + (assign35620_e40982 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 4.0) * locals.var_dnm) + (assign35620_e40982 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign35620_e40986;
        locals.var_tmf0_dn0 = assign35620_e40986_d_n0;
        locals.var_tmf0_dn2 = assign35620_e40986_d_n2;
        locals.var_tmf0_dn4 = assign35620_e40986_d_n4;
        locals.var_tmf0_dn5 = assign35620_e40986_d_n5;
        locals.var_tmf0_dn6 = assign35620_e40986_d_n6;
        locals.var_tmf0_dn7 = assign35620_e40986_d_n7;
        locals.var_tmf0_dn8 = assign35620_e40986_d_n8;
        locals.var_tmf0_dn9 = assign35620_e40986_d_n9;
        locals.var_tmf0_dn10 = assign35620_e40986_d_n10;
        locals.var_tmf0_dn11 = assign35620_e40986_d_n11;
        locals.var_tmf0_dn14 = assign35620_e40986_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign35630_e41002, assign35630_e41002_d_n0, assign35630_e41002_d_n2, assign35630_e41002_d_n4, assign35630_e41002_d_n5, assign35630_e41002_d_n6, assign35630_e41002_d_n7, assign35630_e41002_d_n8, assign35630_e41002_d_n9, assign35630_e41002_d_n10, assign35630_e41002_d_n11, assign35630_e41002_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 != 0.0)) {
        let assign35630_e40996: f64 = (4.0 * locals.var_xmp);
        let assign35630_e40998: f64 = (assign35630_e40996 * locals.var_dnm);
        let assign35630_e41000: f64 = (assign35630_e40998 / locals.var_arg);
        (assign35630_e41000, ((((((4.0 * locals.var_xmp_dn0) * locals.var_dnm) + (assign35630_e40996 * locals.var_dnm_dn0)) * locals.var_arg) - (assign35630_e40998 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((4.0 * locals.var_xmp_dn2) * locals.var_dnm) + (assign35630_e40996 * locals.var_dnm_dn2)) * locals.var_arg) - (assign35630_e40998 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((4.0 * locals.var_xmp_dn4) * locals.var_dnm) + (assign35630_e40996 * locals.var_dnm_dn4)) * locals.var_arg) - (assign35630_e40998 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((4.0 * locals.var_xmp_dn5) * locals.var_dnm) + (assign35630_e40996 * locals.var_dnm_dn5)) * locals.var_arg) - (assign35630_e40998 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((4.0 * locals.var_xmp_dn6) * locals.var_dnm) + (assign35630_e40996 * locals.var_dnm_dn6)) * locals.var_arg) - (assign35630_e40998 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((4.0 * locals.var_xmp_dn7) * locals.var_dnm) + (assign35630_e40996 * locals.var_dnm_dn7)) * locals.var_arg) - (assign35630_e40998 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((4.0 * locals.var_xmp_dn8) * locals.var_dnm) + (assign35630_e40996 * locals.var_dnm_dn8)) * locals.var_arg) - (assign35630_e40998 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((4.0 * locals.var_xmp_dn9) * locals.var_dnm) + (assign35630_e40996 * locals.var_dnm_dn9)) * locals.var_arg) - (assign35630_e40998 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((4.0 * locals.var_xmp_dn10) * locals.var_dnm) + (assign35630_e40996 * locals.var_dnm_dn10)) * locals.var_arg) - (assign35630_e40998 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((4.0 * locals.var_xmp_dn11) * locals.var_dnm) + (assign35630_e40996 * locals.var_dnm_dn11)) * locals.var_arg) - (assign35630_e40998 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((4.0 * locals.var_xmp_dn14) * locals.var_dnm) + (assign35630_e40996 * locals.var_dnm_dn14)) * locals.var_arg) - (assign35630_e40998 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign35630_e41002;
        locals.var_t0_dn0 = assign35630_e41002_d_n0;
        locals.var_t0_dn2 = assign35630_e41002_d_n2;
        locals.var_t0_dn4 = assign35630_e41002_d_n4;
        locals.var_t0_dn5 = assign35630_e41002_d_n5;
        locals.var_t0_dn6 = assign35630_e41002_d_n6;
        locals.var_t0_dn7 = assign35630_e41002_d_n7;
        locals.var_t0_dn8 = assign35630_e41002_d_n8;
        locals.var_t0_dn9 = assign35630_e41002_d_n9;
        locals.var_t0_dn10 = assign35630_e41002_d_n10;
        locals.var_t0_dn11 = assign35630_e41002_d_n11;
        locals.var_t0_dn14 = assign35630_e41002_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign35640_e41016, assign35640_e41016_d_n0, assign35640_e41016_d_n2, assign35640_e41016_d_n4, assign35640_e41016_d_n5, assign35640_e41016_d_n6, assign35640_e41016_d_n7, assign35640_e41016_d_n8, assign35640_e41016_d_n9, assign35640_e41016_d_n10, assign35640_e41016_d_n11, assign35640_e41016_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 != 0.0)) {
        let assign35640_e41012: f64 = (locals.var_uc_depleak + 4.0);
        let assign35640_e41014: f64 = (assign35640_e41012 - locals.var_tmf0);
        (assign35640_e41014, (locals.var_uc_depleak_dn0 - locals.var_tmf0_dn0), (locals.var_uc_depleak_dn2 - locals.var_tmf0_dn2), (locals.var_uc_depleak_dn4 - locals.var_tmf0_dn4), (locals.var_uc_depleak_dn5 - locals.var_tmf0_dn5), (locals.var_uc_depleak_dn6 - locals.var_tmf0_dn6), (locals.var_uc_depleak_dn7 - locals.var_tmf0_dn7), (locals.var_uc_depleak_dn8 - locals.var_tmf0_dn8), (locals.var_uc_depleak_dn9 - locals.var_tmf0_dn9), (locals.var_uc_depleak_dn10 - locals.var_tmf0_dn10), (locals.var_uc_depleak_dn11 - locals.var_tmf0_dn11), (locals.var_uc_depleak_dn14 - locals.var_tmf0_dn14),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign35640_e41016;
        locals.var_t10_dn0 = assign35640_e41016_d_n0;
        locals.var_t10_dn2 = assign35640_e41016_d_n2;
        locals.var_t10_dn4 = assign35640_e41016_d_n4;
        locals.var_t10_dn5 = assign35640_e41016_d_n5;
        locals.var_t10_dn6 = assign35640_e41016_d_n6;
        locals.var_t10_dn7 = assign35640_e41016_d_n7;
        locals.var_t10_dn8 = assign35640_e41016_d_n8;
        locals.var_t10_dn9 = assign35640_e41016_d_n9;
        locals.var_t10_dn10 = assign35640_e41016_d_n10;
        locals.var_t10_dn11 = assign35640_e41016_d_n11;
        locals.var_t10_dn14 = assign35640_e41016_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign35650_e41026, assign35650_e41026_d_n0, assign35650_e41026_d_n2, assign35650_e41026_d_n4, assign35650_e41026_d_n5, assign35650_e41026_d_n6, assign35650_e41026_d_n7, assign35650_e41026_d_n8, assign35650_e41026_d_n9, assign35650_e41026_d_n10, assign35650_e41026_d_n11, assign35650_e41026_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign35650_e41026;
        locals.var_t0_dn0 = assign35650_e41026_d_n0;
        locals.var_t0_dn2 = assign35650_e41026_d_n2;
        locals.var_t0_dn4 = assign35650_e41026_d_n4;
        locals.var_t0_dn5 = assign35650_e41026_d_n5;
        locals.var_t0_dn6 = assign35650_e41026_d_n6;
        locals.var_t0_dn7 = assign35650_e41026_d_n7;
        locals.var_t0_dn8 = assign35650_e41026_d_n8;
        locals.var_t0_dn9 = assign35650_e41026_d_n9;
        locals.var_t0_dn10 = assign35650_e41026_d_n10;
        locals.var_t0_dn11 = assign35650_e41026_d_n11;
        locals.var_t0_dn14 = assign35650_e41026_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign35660_e41037, assign35660_e41037_d_n0, assign35660_e41037_d_n2, assign35660_e41037_d_n4, assign35660_e41037_d_n5, assign35660_e41037_d_n6, assign35660_e41037_d_n7, assign35660_e41037_d_n8, assign35660_e41037_d_n9, assign35660_e41037_d_n10, assign35660_e41037_d_n11, assign35660_e41037_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 == 0.0)) {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign35660_e41037;
        locals.var_t10_dn0 = assign35660_e41037_d_n0;
        locals.var_t10_dn2 = assign35660_e41037_d_n2;
        locals.var_t10_dn4 = assign35660_e41037_d_n4;
        locals.var_t10_dn5 = assign35660_e41037_d_n5;
        locals.var_t10_dn6 = assign35660_e41037_d_n6;
        locals.var_t10_dn7 = assign35660_e41037_d_n7;
        locals.var_t10_dn8 = assign35660_e41037_d_n8;
        locals.var_t10_dn9 = assign35660_e41037_d_n9;
        locals.var_t10_dn10 = assign35660_e41037_d_n10;
        locals.var_t10_dn11 = assign35660_e41037_d_n11;
        locals.var_t10_dn14 = assign35660_e41037_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign35670_e41048, assign35670_e41048_d_n0, assign35670_e41048_d_n2, assign35670_e41048_d_n4, assign35670_e41048_d_n5, assign35670_e41048_d_n6, assign35670_e41048_d_n7, assign35670_e41048_d_n8, assign35670_e41048_d_n9, assign35670_e41048_d_n10, assign35670_e41048_d_n11, assign35670_e41048_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard817 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign35670_e41048;
        locals.var_t0_dn0 = assign35670_e41048_d_n0;
        locals.var_t0_dn2 = assign35670_e41048_d_n2;
        locals.var_t0_dn4 = assign35670_e41048_d_n4;
        locals.var_t0_dn5 = assign35670_e41048_d_n5;
        locals.var_t0_dn6 = assign35670_e41048_d_n6;
        locals.var_t0_dn7 = assign35670_e41048_d_n7;
        locals.var_t0_dn8 = assign35670_e41048_d_n8;
        locals.var_t0_dn9 = assign35670_e41048_d_n9;
        locals.var_t0_dn10 = assign35670_e41048_d_n10;
        locals.var_t0_dn11 = assign35670_e41048_d_n11;
        locals.var_t0_dn14 = assign35670_e41048_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign35680_e41058, assign35680_e41058_d_n0, assign35680_e41058_d_n2, assign35680_e41058_d_n4, assign35680_e41058_d_n5, assign35680_e41058_d_n6, assign35680_e41058_d_n7, assign35680_e41058_d_n8, assign35680_e41058_d_n9, assign35680_e41058_d_n10, assign35680_e41058_d_n11, assign35680_e41058_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) {
        let assign35680_e41056: f64 = (locals.var_vdsorg / locals.var_t10);
        (assign35680_e41056, (((locals.var_vdsorg_dn0 * locals.var_t10) - (locals.var_vdsorg * locals.var_t10_dn0)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vdsorg_dn2 * locals.var_t10) - (locals.var_vdsorg * locals.var_t10_dn2)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vdsorg_dn4 * locals.var_t10) - (locals.var_vdsorg * locals.var_t10_dn4)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vdsorg_dn5 * locals.var_t10) - (locals.var_vdsorg * locals.var_t10_dn5)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vdsorg_dn6 * locals.var_t10) - (locals.var_vdsorg * locals.var_t10_dn6)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vdsorg_dn7 * locals.var_t10) - (locals.var_vdsorg * locals.var_t10_dn7)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vdsorg_dn8 * locals.var_t10) - (locals.var_vdsorg * locals.var_t10_dn8)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vdsorg_dn9 * locals.var_t10) - (locals.var_vdsorg * locals.var_t10_dn9)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vdsorg_dn10 * locals.var_t10) - (locals.var_vdsorg * locals.var_t10_dn10)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vdsorg_dn11 * locals.var_t10) - (locals.var_vdsorg * locals.var_t10_dn11)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vdsorg_dn14 * locals.var_t10) - (locals.var_vdsorg * locals.var_t10_dn14)) / (locals.var_t10 * locals.var_t10)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign35680_e41058;
        locals.var_t1_dn0 = assign35680_e41058_d_n0;
        locals.var_t1_dn2 = assign35680_e41058_d_n2;
        locals.var_t1_dn4 = assign35680_e41058_d_n4;
        locals.var_t1_dn5 = assign35680_e41058_d_n5;
        locals.var_t1_dn6 = assign35680_e41058_d_n6;
        locals.var_t1_dn7 = assign35680_e41058_d_n7;
        locals.var_t1_dn8 = assign35680_e41058_d_n8;
        locals.var_t1_dn9 = assign35680_e41058_d_n9;
        locals.var_t1_dn10 = assign35680_e41058_d_n10;
        locals.var_t1_dn11 = assign35680_e41058_d_n11;
        locals.var_t1_dn14 = assign35680_e41058_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign35690_e41075, assign35690_e41075_d_n0, assign35690_e41075_d_n2, assign35690_e41075_d_n4, assign35690_e41075_d_n5, assign35690_e41075_d_n6, assign35690_e41075_d_n7, assign35690_e41075_d_n8, assign35690_e41075_d_n9, assign35690_e41075_d_n10, assign35690_e41075_d_n11, assign35690_e41075_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) {
        let (assign35690_e41073, assign35690_e41073_d_n0, assign35690_e41073_d_n2, assign35690_e41073_d_n4, assign35690_e41073_d_n5, assign35690_e41073_d_n6, assign35690_e41073_d_n7, assign35690_e41073_d_n8, assign35690_e41073_d_n9, assign35690_e41073_d_n10, assign35690_e41073_d_n11, assign35690_e41073_d_n14,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign35690_e41071: f64 = (locals.var_ddlte - 1.0);
                let assign35690_e41072: f64 = (locals.var_t1).powf(assign35690_e41071);
                (assign35690_e41072, if locals.var_ddlte_dn0 == 0.0 && ((assign35690_e41071) as f64).is_finite() && ((assign35690_e41071) as f64).fract() == 0.0 { if assign35690_e41071 == 0.0 { 0.0 } else { (assign35690_e41071 * ((locals.var_t1).powf(assign35690_e41071 - 1.0) * locals.var_t1_dn0)) } } else { (assign35690_e41072 * ((locals.var_ddlte_dn0 * (locals.var_t1).ln()) + (assign35690_e41071 * (locals.var_t1_dn0 / locals.var_t1)))) }, if locals.var_ddlte_dn2 == 0.0 && ((assign35690_e41071) as f64).is_finite() && ((assign35690_e41071) as f64).fract() == 0.0 { if assign35690_e41071 == 0.0 { 0.0 } else { (assign35690_e41071 * ((locals.var_t1).powf(assign35690_e41071 - 1.0) * locals.var_t1_dn2)) } } else { (assign35690_e41072 * ((locals.var_ddlte_dn2 * (locals.var_t1).ln()) + (assign35690_e41071 * (locals.var_t1_dn2 / locals.var_t1)))) }, if locals.var_ddlte_dn4 == 0.0 && ((assign35690_e41071) as f64).is_finite() && ((assign35690_e41071) as f64).fract() == 0.0 { if assign35690_e41071 == 0.0 { 0.0 } else { (assign35690_e41071 * ((locals.var_t1).powf(assign35690_e41071 - 1.0) * locals.var_t1_dn4)) } } else { (assign35690_e41072 * ((locals.var_ddlte_dn4 * (locals.var_t1).ln()) + (assign35690_e41071 * (locals.var_t1_dn4 / locals.var_t1)))) }, if locals.var_ddlte_dn5 == 0.0 && ((assign35690_e41071) as f64).is_finite() && ((assign35690_e41071) as f64).fract() == 0.0 { if assign35690_e41071 == 0.0 { 0.0 } else { (assign35690_e41071 * ((locals.var_t1).powf(assign35690_e41071 - 1.0) * locals.var_t1_dn5)) } } else { (assign35690_e41072 * ((locals.var_ddlte_dn5 * (locals.var_t1).ln()) + (assign35690_e41071 * (locals.var_t1_dn5 / locals.var_t1)))) }, if locals.var_ddlte_dn6 == 0.0 && ((assign35690_e41071) as f64).is_finite() && ((assign35690_e41071) as f64).fract() == 0.0 { if assign35690_e41071 == 0.0 { 0.0 } else { (assign35690_e41071 * ((locals.var_t1).powf(assign35690_e41071 - 1.0) * locals.var_t1_dn6)) } } else { (assign35690_e41072 * ((locals.var_ddlte_dn6 * (locals.var_t1).ln()) + (assign35690_e41071 * (locals.var_t1_dn6 / locals.var_t1)))) }, if locals.var_ddlte_dn7 == 0.0 && ((assign35690_e41071) as f64).is_finite() && ((assign35690_e41071) as f64).fract() == 0.0 { if assign35690_e41071 == 0.0 { 0.0 } else { (assign35690_e41071 * ((locals.var_t1).powf(assign35690_e41071 - 1.0) * locals.var_t1_dn7)) } } else { (assign35690_e41072 * ((locals.var_ddlte_dn7 * (locals.var_t1).ln()) + (assign35690_e41071 * (locals.var_t1_dn7 / locals.var_t1)))) }, if locals.var_ddlte_dn8 == 0.0 && ((assign35690_e41071) as f64).is_finite() && ((assign35690_e41071) as f64).fract() == 0.0 { if assign35690_e41071 == 0.0 { 0.0 } else { (assign35690_e41071 * ((locals.var_t1).powf(assign35690_e41071 - 1.0) * locals.var_t1_dn8)) } } else { (assign35690_e41072 * ((locals.var_ddlte_dn8 * (locals.var_t1).ln()) + (assign35690_e41071 * (locals.var_t1_dn8 / locals.var_t1)))) }, if locals.var_ddlte_dn9 == 0.0 && ((assign35690_e41071) as f64).is_finite() && ((assign35690_e41071) as f64).fract() == 0.0 { if assign35690_e41071 == 0.0 { 0.0 } else { (assign35690_e41071 * ((locals.var_t1).powf(assign35690_e41071 - 1.0) * locals.var_t1_dn9)) } } else { (assign35690_e41072 * ((locals.var_ddlte_dn9 * (locals.var_t1).ln()) + (assign35690_e41071 * (locals.var_t1_dn9 / locals.var_t1)))) }, if locals.var_ddlte_dn10 == 0.0 && ((assign35690_e41071) as f64).is_finite() && ((assign35690_e41071) as f64).fract() == 0.0 { if assign35690_e41071 == 0.0 { 0.0 } else { (assign35690_e41071 * ((locals.var_t1).powf(assign35690_e41071 - 1.0) * locals.var_t1_dn10)) } } else { (assign35690_e41072 * ((locals.var_ddlte_dn10 * (locals.var_t1).ln()) + (assign35690_e41071 * (locals.var_t1_dn10 / locals.var_t1)))) }, if locals.var_ddlte_dn11 == 0.0 && ((assign35690_e41071) as f64).is_finite() && ((assign35690_e41071) as f64).fract() == 0.0 { if assign35690_e41071 == 0.0 { 0.0 } else { (assign35690_e41071 * ((locals.var_t1).powf(assign35690_e41071 - 1.0) * locals.var_t1_dn11)) } } else { (assign35690_e41072 * ((locals.var_ddlte_dn11 * (locals.var_t1).ln()) + (assign35690_e41071 * (locals.var_t1_dn11 / locals.var_t1)))) }, if locals.var_ddlte_dn14 == 0.0 && ((assign35690_e41071) as f64).is_finite() && ((assign35690_e41071) as f64).fract() == 0.0 { if assign35690_e41071 == 0.0 { 0.0 } else { (assign35690_e41071 * ((locals.var_t1).powf(assign35690_e41071 - 1.0) * locals.var_t1_dn14)) } } else { (assign35690_e41072 * ((locals.var_ddlte_dn14 * (locals.var_t1).ln()) + (assign35690_e41071 * (locals.var_t1_dn14 / locals.var_t1)))) },)
            }
        };
        (assign35690_e41073, assign35690_e41073_d_n0, assign35690_e41073_d_n2, assign35690_e41073_d_n4, assign35690_e41073_d_n5, assign35690_e41073_d_n6, assign35690_e41073_d_n7, assign35690_e41073_d_n8, assign35690_e41073_d_n9, assign35690_e41073_d_n10, assign35690_e41073_d_n11, assign35690_e41073_d_n14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign35690_e41075;
        locals.var_t2_dn0 = assign35690_e41075_d_n0;
        locals.var_t2_dn2 = assign35690_e41075_d_n2;
        locals.var_t2_dn4 = assign35690_e41075_d_n4;
        locals.var_t2_dn5 = assign35690_e41075_d_n5;
        locals.var_t2_dn6 = assign35690_e41075_d_n6;
        locals.var_t2_dn7 = assign35690_e41075_d_n7;
        locals.var_t2_dn8 = assign35690_e41075_d_n8;
        locals.var_t2_dn9 = assign35690_e41075_d_n9;
        locals.var_t2_dn10 = assign35690_e41075_d_n10;
        locals.var_t2_dn11 = assign35690_e41075_d_n11;
        locals.var_t2_dn14 = assign35690_e41075_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign35700_e41085, assign35700_e41085_d_n0, assign35700_e41085_d_n2, assign35700_e41085_d_n4, assign35700_e41085_d_n5, assign35700_e41085_d_n6, assign35700_e41085_d_n7, assign35700_e41085_d_n8, assign35700_e41085_d_n9, assign35700_e41085_d_n10, assign35700_e41085_d_n11, assign35700_e41085_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) {
        let assign35700_e41083: f64 = (locals.var_t2 * locals.var_t1);
        (assign35700_e41083, ((locals.var_t2_dn0 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn0)), ((locals.var_t2_dn2 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn2)), ((locals.var_t2_dn4 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn4)), ((locals.var_t2_dn5 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn5)), ((locals.var_t2_dn6 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn6)), ((locals.var_t2_dn7 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn7)), ((locals.var_t2_dn8 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn8)), ((locals.var_t2_dn9 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn9)), ((locals.var_t2_dn10 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn10)), ((locals.var_t2_dn11 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn11)), ((locals.var_t2_dn14 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign35700_e41085;
        locals.var_t7_dn0 = assign35700_e41085_d_n0;
        locals.var_t7_dn2 = assign35700_e41085_d_n2;
        locals.var_t7_dn4 = assign35700_e41085_d_n4;
        locals.var_t7_dn5 = assign35700_e41085_d_n5;
        locals.var_t7_dn6 = assign35700_e41085_d_n6;
        locals.var_t7_dn7 = assign35700_e41085_d_n7;
        locals.var_t7_dn8 = assign35700_e41085_d_n8;
        locals.var_t7_dn9 = assign35700_e41085_d_n9;
        locals.var_t7_dn10 = assign35700_e41085_d_n10;
        locals.var_t7_dn11 = assign35700_e41085_d_n11;
        locals.var_t7_dn14 = assign35700_e41085_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign35710_e41095, assign35710_e41095_d_n0, assign35710_e41095_d_n2, assign35710_e41095_d_n4, assign35710_e41095_d_n5, assign35710_e41095_d_n6, assign35710_e41095_d_n7, assign35710_e41095_d_n8, assign35710_e41095_d_n9, assign35710_e41095_d_n10, assign35710_e41095_d_n11, assign35710_e41095_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) {
        let assign35710_e41093: f64 = (1.0 + locals.var_t7);
        (assign35710_e41093, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign35710_e41095;
        locals.var_t3_dn0 = assign35710_e41095_d_n0;
        locals.var_t3_dn2 = assign35710_e41095_d_n2;
        locals.var_t3_dn4 = assign35710_e41095_d_n4;
        locals.var_t3_dn5 = assign35710_e41095_d_n5;
        locals.var_t3_dn6 = assign35710_e41095_d_n6;
        locals.var_t3_dn7 = assign35710_e41095_d_n7;
        locals.var_t3_dn8 = assign35710_e41095_d_n8;
        locals.var_t3_dn9 = assign35710_e41095_d_n9;
        locals.var_t3_dn10 = assign35710_e41095_d_n10;
        locals.var_t3_dn11 = assign35710_e41095_d_n11;
        locals.var_t3_dn14 = assign35710_e41095_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign35720_e41114, assign35720_e41114_d_n0, assign35720_e41114_d_n2, assign35720_e41114_d_n4, assign35720_e41114_d_n5, assign35720_e41114_d_n6, assign35720_e41114_d_n7, assign35720_e41114_d_n8, assign35720_e41114_d_n9, assign35720_e41114_d_n10, assign35720_e41114_d_n11, assign35720_e41114_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) {
        let (assign35720_e41112, assign35720_e41112_d_n0, assign35720_e41112_d_n2, assign35720_e41112_d_n4, assign35720_e41112_d_n5, assign35720_e41112_d_n6, assign35720_e41112_d_n7, assign35720_e41112_d_n8, assign35720_e41112_d_n9, assign35720_e41112_d_n10, assign35720_e41112_d_n11, assign35720_e41112_d_n14,) = {
            if (locals.var_t3 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign35720_e41108: f64 = (1.0 / locals.var_ddlte);
                let assign35720_e41110: f64 = (assign35720_e41108 - 1.0);
                let assign35720_e41111: f64 = (locals.var_t3).powf(assign35720_e41110);
                (assign35720_e41111, if (-(locals.var_ddlte_dn0 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign35720_e41110) as f64).is_finite() && ((assign35720_e41110) as f64).fract() == 0.0 { if assign35720_e41110 == 0.0 { 0.0 } else { (assign35720_e41110 * ((locals.var_t3).powf(assign35720_e41110 - 1.0) * locals.var_t3_dn0)) } } else { (assign35720_e41111 * (((-(locals.var_ddlte_dn0 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign35720_e41110 * (locals.var_t3_dn0 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn2 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign35720_e41110) as f64).is_finite() && ((assign35720_e41110) as f64).fract() == 0.0 { if assign35720_e41110 == 0.0 { 0.0 } else { (assign35720_e41110 * ((locals.var_t3).powf(assign35720_e41110 - 1.0) * locals.var_t3_dn2)) } } else { (assign35720_e41111 * (((-(locals.var_ddlte_dn2 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign35720_e41110 * (locals.var_t3_dn2 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn4 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign35720_e41110) as f64).is_finite() && ((assign35720_e41110) as f64).fract() == 0.0 { if assign35720_e41110 == 0.0 { 0.0 } else { (assign35720_e41110 * ((locals.var_t3).powf(assign35720_e41110 - 1.0) * locals.var_t3_dn4)) } } else { (assign35720_e41111 * (((-(locals.var_ddlte_dn4 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign35720_e41110 * (locals.var_t3_dn4 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn5 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign35720_e41110) as f64).is_finite() && ((assign35720_e41110) as f64).fract() == 0.0 { if assign35720_e41110 == 0.0 { 0.0 } else { (assign35720_e41110 * ((locals.var_t3).powf(assign35720_e41110 - 1.0) * locals.var_t3_dn5)) } } else { (assign35720_e41111 * (((-(locals.var_ddlte_dn5 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign35720_e41110 * (locals.var_t3_dn5 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn6 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign35720_e41110) as f64).is_finite() && ((assign35720_e41110) as f64).fract() == 0.0 { if assign35720_e41110 == 0.0 { 0.0 } else { (assign35720_e41110 * ((locals.var_t3).powf(assign35720_e41110 - 1.0) * locals.var_t3_dn6)) } } else { (assign35720_e41111 * (((-(locals.var_ddlte_dn6 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign35720_e41110 * (locals.var_t3_dn6 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn7 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign35720_e41110) as f64).is_finite() && ((assign35720_e41110) as f64).fract() == 0.0 { if assign35720_e41110 == 0.0 { 0.0 } else { (assign35720_e41110 * ((locals.var_t3).powf(assign35720_e41110 - 1.0) * locals.var_t3_dn7)) } } else { (assign35720_e41111 * (((-(locals.var_ddlte_dn7 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign35720_e41110 * (locals.var_t3_dn7 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn8 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign35720_e41110) as f64).is_finite() && ((assign35720_e41110) as f64).fract() == 0.0 { if assign35720_e41110 == 0.0 { 0.0 } else { (assign35720_e41110 * ((locals.var_t3).powf(assign35720_e41110 - 1.0) * locals.var_t3_dn8)) } } else { (assign35720_e41111 * (((-(locals.var_ddlte_dn8 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign35720_e41110 * (locals.var_t3_dn8 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn9 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign35720_e41110) as f64).is_finite() && ((assign35720_e41110) as f64).fract() == 0.0 { if assign35720_e41110 == 0.0 { 0.0 } else { (assign35720_e41110 * ((locals.var_t3).powf(assign35720_e41110 - 1.0) * locals.var_t3_dn9)) } } else { (assign35720_e41111 * (((-(locals.var_ddlte_dn9 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign35720_e41110 * (locals.var_t3_dn9 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn10 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign35720_e41110) as f64).is_finite() && ((assign35720_e41110) as f64).fract() == 0.0 { if assign35720_e41110 == 0.0 { 0.0 } else { (assign35720_e41110 * ((locals.var_t3).powf(assign35720_e41110 - 1.0) * locals.var_t3_dn10)) } } else { (assign35720_e41111 * (((-(locals.var_ddlte_dn10 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign35720_e41110 * (locals.var_t3_dn10 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn11 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign35720_e41110) as f64).is_finite() && ((assign35720_e41110) as f64).fract() == 0.0 { if assign35720_e41110 == 0.0 { 0.0 } else { (assign35720_e41110 * ((locals.var_t3).powf(assign35720_e41110 - 1.0) * locals.var_t3_dn11)) } } else { (assign35720_e41111 * (((-(locals.var_ddlte_dn11 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign35720_e41110 * (locals.var_t3_dn11 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn14 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign35720_e41110) as f64).is_finite() && ((assign35720_e41110) as f64).fract() == 0.0 { if assign35720_e41110 == 0.0 { 0.0 } else { (assign35720_e41110 * ((locals.var_t3).powf(assign35720_e41110 - 1.0) * locals.var_t3_dn14)) } } else { (assign35720_e41111 * (((-(locals.var_ddlte_dn14 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign35720_e41110 * (locals.var_t3_dn14 / locals.var_t3)))) },)
            }
        };
        (assign35720_e41112, assign35720_e41112_d_n0, assign35720_e41112_d_n2, assign35720_e41112_d_n4, assign35720_e41112_d_n5, assign35720_e41112_d_n6, assign35720_e41112_d_n7, assign35720_e41112_d_n8, assign35720_e41112_d_n9, assign35720_e41112_d_n10, assign35720_e41112_d_n11, assign35720_e41112_d_n14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign35720_e41114;
        locals.var_t4_dn0 = assign35720_e41114_d_n0;
        locals.var_t4_dn2 = assign35720_e41114_d_n2;
        locals.var_t4_dn4 = assign35720_e41114_d_n4;
        locals.var_t4_dn5 = assign35720_e41114_d_n5;
        locals.var_t4_dn6 = assign35720_e41114_d_n6;
        locals.var_t4_dn7 = assign35720_e41114_d_n7;
        locals.var_t4_dn8 = assign35720_e41114_d_n8;
        locals.var_t4_dn9 = assign35720_e41114_d_n9;
        locals.var_t4_dn10 = assign35720_e41114_d_n10;
        locals.var_t4_dn11 = assign35720_e41114_d_n11;
        locals.var_t4_dn14 = assign35720_e41114_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign35730_e41124, assign35730_e41124_d_n0, assign35730_e41124_d_n2, assign35730_e41124_d_n4, assign35730_e41124_d_n5, assign35730_e41124_d_n6, assign35730_e41124_d_n7, assign35730_e41124_d_n8, assign35730_e41124_d_n9, assign35730_e41124_d_n10, assign35730_e41124_d_n11, assign35730_e41124_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) {
        let assign35730_e41122: f64 = (locals.var_t4 * locals.var_t3);
        (assign35730_e41122, ((locals.var_t4_dn0 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn0)), ((locals.var_t4_dn2 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn2)), ((locals.var_t4_dn4 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn4)), ((locals.var_t4_dn5 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn5)), ((locals.var_t4_dn6 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn6)), ((locals.var_t4_dn7 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn7)), ((locals.var_t4_dn8 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn8)), ((locals.var_t4_dn9 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn9)), ((locals.var_t4_dn10 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn10)), ((locals.var_t4_dn11 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn11)), ((locals.var_t4_dn14 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign35730_e41124;
        locals.var_t6_dn0 = assign35730_e41124_d_n0;
        locals.var_t6_dn2 = assign35730_e41124_d_n2;
        locals.var_t6_dn4 = assign35730_e41124_d_n4;
        locals.var_t6_dn5 = assign35730_e41124_d_n5;
        locals.var_t6_dn6 = assign35730_e41124_d_n6;
        locals.var_t6_dn7 = assign35730_e41124_d_n7;
        locals.var_t6_dn8 = assign35730_e41124_d_n8;
        locals.var_t6_dn9 = assign35730_e41124_d_n9;
        locals.var_t6_dn10 = assign35730_e41124_d_n10;
        locals.var_t6_dn11 = assign35730_e41124_d_n11;
        locals.var_t6_dn14 = assign35730_e41124_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign35740_e41134, assign35740_e41134_d_n0, assign35740_e41134_d_n2, assign35740_e41134_d_n4, assign35740_e41134_d_n5, assign35740_e41134_d_n6, assign35740_e41134_d_n7, assign35740_e41134_d_n8, assign35740_e41134_d_n9, assign35740_e41134_d_n10, assign35740_e41134_d_n11, assign35740_e41134_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 != 0.0)) {
        let assign35740_e41132: f64 = (locals.var_vdsorg / locals.var_t6);
        (assign35740_e41132, (((locals.var_vdsorg_dn0 * locals.var_t6) - (locals.var_vdsorg * locals.var_t6_dn0)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vdsorg_dn2 * locals.var_t6) - (locals.var_vdsorg * locals.var_t6_dn2)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vdsorg_dn4 * locals.var_t6) - (locals.var_vdsorg * locals.var_t6_dn4)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vdsorg_dn5 * locals.var_t6) - (locals.var_vdsorg * locals.var_t6_dn5)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vdsorg_dn6 * locals.var_t6) - (locals.var_vdsorg * locals.var_t6_dn6)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vdsorg_dn7 * locals.var_t6) - (locals.var_vdsorg * locals.var_t6_dn7)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vdsorg_dn8 * locals.var_t6) - (locals.var_vdsorg * locals.var_t6_dn8)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vdsorg_dn9 * locals.var_t6) - (locals.var_vdsorg * locals.var_t6_dn9)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vdsorg_dn10 * locals.var_t6) - (locals.var_vdsorg * locals.var_t6_dn10)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vdsorg_dn11 * locals.var_t6) - (locals.var_vdsorg * locals.var_t6_dn11)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vdsorg_dn14 * locals.var_t6) - (locals.var_vdsorg * locals.var_t6_dn14)) / (locals.var_t6 * locals.var_t6)),)
    } else {
        (locals.var_vdseff0, locals.var_vdseff0_dn0, locals.var_vdseff0_dn2, locals.var_vdseff0_dn4, locals.var_vdseff0_dn5, locals.var_vdseff0_dn6, locals.var_vdseff0_dn7, locals.var_vdseff0_dn8, locals.var_vdseff0_dn9, locals.var_vdseff0_dn10, locals.var_vdseff0_dn11, locals.var_vdseff0_dn14,)
    }
};
        locals.var_vdseff0 = assign35740_e41134;
        locals.var_vdseff0_dn0 = assign35740_e41134_d_n0;
        locals.var_vdseff0_dn2 = assign35740_e41134_d_n2;
        locals.var_vdseff0_dn4 = assign35740_e41134_d_n4;
        locals.var_vdseff0_dn5 = assign35740_e41134_d_n5;
        locals.var_vdseff0_dn6 = assign35740_e41134_d_n6;
        locals.var_vdseff0_dn7 = assign35740_e41134_d_n7;
        locals.var_vdseff0_dn8 = assign35740_e41134_d_n8;
        locals.var_vdseff0_dn9 = assign35740_e41134_d_n9;
        locals.var_vdseff0_dn10 = assign35740_e41134_d_n10;
        locals.var_vdseff0_dn11 = assign35740_e41134_d_n11;
        locals.var_vdseff0_dn14 = assign35740_e41134_d_n14;
        locals.var_vdseff0_rv = 0.0;

        let (assign35750_e41143, assign35750_e41143_d_n0, assign35750_e41143_d_n2, assign35750_e41143_d_n4, assign35750_e41143_d_n5, assign35750_e41143_d_n6, assign35750_e41143_d_n7, assign35750_e41143_d_n8, assign35750_e41143_d_n9, assign35750_e41143_d_n10, assign35750_e41143_d_n11, assign35750_e41143_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard810 == 0.0)) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn14,)
    } else {
        (locals.var_vdseff0, locals.var_vdseff0_dn0, locals.var_vdseff0_dn2, locals.var_vdseff0_dn4, locals.var_vdseff0_dn5, locals.var_vdseff0_dn6, locals.var_vdseff0_dn7, locals.var_vdseff0_dn8, locals.var_vdseff0_dn9, locals.var_vdseff0_dn10, locals.var_vdseff0_dn11, locals.var_vdseff0_dn14,)
    }
};
        locals.var_vdseff0 = assign35750_e41143;
        locals.var_vdseff0_dn0 = assign35750_e41143_d_n0;
        locals.var_vdseff0_dn2 = assign35750_e41143_d_n2;
        locals.var_vdseff0_dn4 = assign35750_e41143_d_n4;
        locals.var_vdseff0_dn5 = assign35750_e41143_d_n5;
        locals.var_vdseff0_dn6 = assign35750_e41143_d_n6;
        locals.var_vdseff0_dn7 = assign35750_e41143_d_n7;
        locals.var_vdseff0_dn8 = assign35750_e41143_d_n8;
        locals.var_vdseff0_dn9 = assign35750_e41143_d_n9;
        locals.var_vdseff0_dn10 = assign35750_e41143_d_n10;
        locals.var_vdseff0_dn11 = assign35750_e41143_d_n11;
        locals.var_vdseff0_dn14 = assign35750_e41143_d_n14;
        locals.var_vdseff0_rv = 0.0;

        let (assign35760_e41155, assign35760_e41155_d_n0, assign35760_e41155_d_n2, assign35760_e41155_d_n4, assign35760_e41155_d_n5, assign35760_e41155_d_n6, assign35760_e41155_d_n7, assign35760_e41155_d_n8, assign35760_e41155_d_n9, assign35760_e41155_d_n10, assign35760_e41155_d_n11, assign35760_e41155_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign35760_e41150: f64 = (locals.var_phi_sl_dep - locals.var_phi_s0_dep);
        let assign35760_e41152: f64 = (assign35760_e41150 * locals.var_ninvde);
        let assign35760_e41153: f64 = (1.0 + assign35760_e41152);
        (assign35760_e41153, (((locals.var_phi_sl_dep_dn0 - locals.var_phi_s0_dep_dn0) * locals.var_ninvde) + (assign35760_e41150 * locals.var_ninvde_dn0)), (((locals.var_phi_sl_dep_dn2 - locals.var_phi_s0_dep_dn2) * locals.var_ninvde) + (assign35760_e41150 * locals.var_ninvde_dn2)), (((locals.var_phi_sl_dep_dn4 - locals.var_phi_s0_dep_dn4) * locals.var_ninvde) + (assign35760_e41150 * locals.var_ninvde_dn4)), (((locals.var_phi_sl_dep_dn5 - locals.var_phi_s0_dep_dn5) * locals.var_ninvde) + (assign35760_e41150 * locals.var_ninvde_dn5)), (((locals.var_phi_sl_dep_dn6 - locals.var_phi_s0_dep_dn6) * locals.var_ninvde) + (assign35760_e41150 * locals.var_ninvde_dn6)), (((locals.var_phi_sl_dep_dn7 - locals.var_phi_s0_dep_dn7) * locals.var_ninvde) + (assign35760_e41150 * locals.var_ninvde_dn7)), (((locals.var_phi_sl_dep_dn8 - locals.var_phi_s0_dep_dn8) * locals.var_ninvde) + (assign35760_e41150 * locals.var_ninvde_dn8)), (((locals.var_phi_sl_dep_dn9 - locals.var_phi_s0_dep_dn9) * locals.var_ninvde) + (assign35760_e41150 * locals.var_ninvde_dn9)), (((locals.var_phi_sl_dep_dn10 - locals.var_phi_s0_dep_dn10) * locals.var_ninvde) + (assign35760_e41150 * locals.var_ninvde_dn10)), (((locals.var_phi_sl_dep_dn11 - locals.var_phi_s0_dep_dn11) * locals.var_ninvde) + (assign35760_e41150 * locals.var_ninvde_dn11)), (((locals.var_phi_sl_dep_dn14 - locals.var_phi_s0_dep_dn14) * locals.var_ninvde) + (assign35760_e41150 * locals.var_ninvde_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign35760_e41155;
        locals.var_t4_dn0 = assign35760_e41155_d_n0;
        locals.var_t4_dn2 = assign35760_e41155_d_n2;
        locals.var_t4_dn4 = assign35760_e41155_d_n4;
        locals.var_t4_dn5 = assign35760_e41155_d_n5;
        locals.var_t4_dn6 = assign35760_e41155_d_n6;
        locals.var_t4_dn7 = assign35760_e41155_d_n7;
        locals.var_t4_dn8 = assign35760_e41155_d_n8;
        locals.var_t4_dn9 = assign35760_e41155_d_n9;
        locals.var_t4_dn10 = assign35760_e41155_d_n10;
        locals.var_t4_dn11 = assign35760_e41155_d_n11;
        locals.var_t4_dn14 = assign35760_e41155_d_n14;
        locals.var_t4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_117(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign35770_e41162, assign35770_e41162_d_n0, assign35770_e41162_d_n2, assign35770_e41162_d_n4, assign35770_e41162_d_n5, assign35770_e41162_d_n6, assign35770_e41162_d_n7, assign35770_e41162_d_n8, assign35770_e41162_d_n9, assign35770_e41162_d_n10, assign35770_e41162_d_n11, assign35770_e41162_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign35770_e41160: f64 = (-locals.var_qn_res0);
        (assign35770_e41160, (-locals.var_qn_res0_dn0), (-locals.var_qn_res0_dn2), (-locals.var_qn_res0_dn4), (-locals.var_qn_res0_dn5), (-locals.var_qn_res0_dn6), (-locals.var_qn_res0_dn7), (-locals.var_qn_res0_dn8), (-locals.var_qn_res0_dn9), (-locals.var_qn_res0_dn10), (-locals.var_qn_res0_dn11), (-locals.var_qn_res0_dn14),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn14,)
    }
};
        locals.var_qiu = assign35770_e41162;
        locals.var_qiu_dn0 = assign35770_e41162_d_n0;
        locals.var_qiu_dn2 = assign35770_e41162_d_n2;
        locals.var_qiu_dn4 = assign35770_e41162_d_n4;
        locals.var_qiu_dn5 = assign35770_e41162_d_n5;
        locals.var_qiu_dn6 = assign35770_e41162_d_n6;
        locals.var_qiu_dn7 = assign35770_e41162_d_n7;
        locals.var_qiu_dn8 = assign35770_e41162_d_n8;
        locals.var_qiu_dn9 = assign35770_e41162_d_n9;
        locals.var_qiu_dn10 = assign35770_e41162_d_n10;
        locals.var_qiu_dn11 = assign35770_e41162_d_n11;
        locals.var_qiu_dn14 = assign35770_e41162_d_n14;
        locals.var_qiu_rv = 0.0;

        let (assign35780_e41168, assign35780_e41168_d_n0, assign35780_e41168_d_n2, assign35780_e41168_d_n4, assign35780_e41168_d_n5, assign35780_e41168_d_n6, assign35780_e41168_d_n7, assign35780_e41168_d_n8, assign35780_e41168_d_n9, assign35780_e41168_d_n10, assign35780_e41168_d_n11, assign35780_e41168_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign35780_e41168;
        locals.var_t5_dn0 = assign35780_e41168_d_n0;
        locals.var_t5_dn2 = assign35780_e41168_d_n2;
        locals.var_t5_dn4 = assign35780_e41168_d_n4;
        locals.var_t5_dn5 = assign35780_e41168_d_n5;
        locals.var_t5_dn6 = assign35780_e41168_d_n6;
        locals.var_t5_dn7 = assign35780_e41168_d_n7;
        locals.var_t5_dn8 = assign35780_e41168_d_n8;
        locals.var_t5_dn9 = assign35780_e41168_d_n9;
        locals.var_t5_dn10 = assign35780_e41168_d_n10;
        locals.var_t5_dn11 = assign35780_e41168_d_n11;
        locals.var_t5_dn14 = assign35780_e41168_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign35790_e41176, assign35790_e41176_d_n0, assign35790_e41176_d_n2, assign35790_e41176_d_n4, assign35790_e41176_d_n5, assign35790_e41176_d_n6, assign35790_e41176_d_n7, assign35790_e41176_d_n8, assign35790_e41176_d_n9, assign35790_e41176_d_n10, assign35790_e41176_d_n11, assign35790_e41176_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign35790_e41174: f64 = (locals.var_t5 / locals.var_t4);
        (assign35790_e41174, (((locals.var_t5_dn0 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn2 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn4 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn5 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn6 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn7 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn8 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn9 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn10 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn11 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn14 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign35790_e41176;
        locals.var_t3_dn0 = assign35790_e41176_d_n0;
        locals.var_t3_dn2 = assign35790_e41176_d_n2;
        locals.var_t3_dn4 = assign35790_e41176_d_n4;
        locals.var_t3_dn5 = assign35790_e41176_d_n5;
        locals.var_t3_dn6 = assign35790_e41176_d_n6;
        locals.var_t3_dn7 = assign35790_e41176_d_n7;
        locals.var_t3_dn8 = assign35790_e41176_d_n8;
        locals.var_t3_dn9 = assign35790_e41176_d_n9;
        locals.var_t3_dn10 = assign35790_e41176_d_n10;
        locals.var_t3_dn11 = assign35790_e41176_d_n11;
        locals.var_t3_dn14 = assign35790_e41176_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign35800_e41182, assign35800_e41182_d_n0, assign35800_e41182_d_n2, assign35800_e41182_d_n4, assign35800_e41182_d_n5, assign35800_e41182_d_n6, assign35800_e41182_d_n7, assign35800_e41182_d_n8, assign35800_e41182_d_n9, assign35800_e41182_d_n10, assign35800_e41182_d_n11, assign35800_e41182_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    } else {
        (locals.var_eeff, locals.var_eeff_dn0, locals.var_eeff_dn2, locals.var_eeff_dn4, locals.var_eeff_dn5, locals.var_eeff_dn6, locals.var_eeff_dn7, locals.var_eeff_dn8, locals.var_eeff_dn9, locals.var_eeff_dn10, locals.var_eeff_dn11, locals.var_eeff_dn14,)
    }
};
        locals.var_eeff = assign35800_e41182;
        locals.var_eeff_dn0 = assign35800_e41182_d_n0;
        locals.var_eeff_dn2 = assign35800_e41182_d_n2;
        locals.var_eeff_dn4 = assign35800_e41182_d_n4;
        locals.var_eeff_dn5 = assign35800_e41182_d_n5;
        locals.var_eeff_dn6 = assign35800_e41182_d_n6;
        locals.var_eeff_dn7 = assign35800_e41182_d_n7;
        locals.var_eeff_dn8 = assign35800_e41182_d_n8;
        locals.var_eeff_dn9 = assign35800_e41182_d_n9;
        locals.var_eeff_dn10 = assign35800_e41182_d_n10;
        locals.var_eeff_dn11 = assign35800_e41182_d_n11;
        locals.var_eeff_dn14 = assign35800_e41182_d_n14;
        locals.var_eeff_rv = 0.0;

        let (assign35810_e41197, assign35810_e41197_d_n0, assign35810_e41197_d_n2, assign35810_e41197_d_n4, assign35810_e41197_d_n5, assign35810_e41197_d_n6, assign35810_e41197_d_n7, assign35810_e41197_d_n8, assign35810_e41197_d_n9, assign35810_e41197_d_n10, assign35810_e41197_d_n11, assign35810_e41197_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let (assign35810_e41195, assign35810_e41195_d_n0, assign35810_e41195_d_n2, assign35810_e41195_d_n4, assign35810_e41195_d_n5, assign35810_e41195_d_n6, assign35810_e41195_d_n7, assign35810_e41195_d_n8, assign35810_e41195_d_n9, assign35810_e41195_d_n10, assign35810_e41195_d_n11, assign35810_e41195_d_n14,) = {
            if (locals.var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign35810_e41193: f64 = (p.p376 - 1.0);
                let assign35810_e41194: f64 = (locals.var_eeff).powf(assign35810_e41193);
                (assign35810_e41194, if 0.0 == 0.0 && ((assign35810_e41193) as f64).is_finite() && ((assign35810_e41193) as f64).fract() == 0.0 { if assign35810_e41193 == 0.0 { 0.0 } else { (assign35810_e41193 * ((locals.var_eeff).powf(assign35810_e41193 - 1.0) * locals.var_eeff_dn0)) } } else { (assign35810_e41194 * (assign35810_e41193 * (locals.var_eeff_dn0 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35810_e41193) as f64).is_finite() && ((assign35810_e41193) as f64).fract() == 0.0 { if assign35810_e41193 == 0.0 { 0.0 } else { (assign35810_e41193 * ((locals.var_eeff).powf(assign35810_e41193 - 1.0) * locals.var_eeff_dn2)) } } else { (assign35810_e41194 * (assign35810_e41193 * (locals.var_eeff_dn2 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35810_e41193) as f64).is_finite() && ((assign35810_e41193) as f64).fract() == 0.0 { if assign35810_e41193 == 0.0 { 0.0 } else { (assign35810_e41193 * ((locals.var_eeff).powf(assign35810_e41193 - 1.0) * locals.var_eeff_dn4)) } } else { (assign35810_e41194 * (assign35810_e41193 * (locals.var_eeff_dn4 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35810_e41193) as f64).is_finite() && ((assign35810_e41193) as f64).fract() == 0.0 { if assign35810_e41193 == 0.0 { 0.0 } else { (assign35810_e41193 * ((locals.var_eeff).powf(assign35810_e41193 - 1.0) * locals.var_eeff_dn5)) } } else { (assign35810_e41194 * (assign35810_e41193 * (locals.var_eeff_dn5 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35810_e41193) as f64).is_finite() && ((assign35810_e41193) as f64).fract() == 0.0 { if assign35810_e41193 == 0.0 { 0.0 } else { (assign35810_e41193 * ((locals.var_eeff).powf(assign35810_e41193 - 1.0) * locals.var_eeff_dn6)) } } else { (assign35810_e41194 * (assign35810_e41193 * (locals.var_eeff_dn6 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35810_e41193) as f64).is_finite() && ((assign35810_e41193) as f64).fract() == 0.0 { if assign35810_e41193 == 0.0 { 0.0 } else { (assign35810_e41193 * ((locals.var_eeff).powf(assign35810_e41193 - 1.0) * locals.var_eeff_dn7)) } } else { (assign35810_e41194 * (assign35810_e41193 * (locals.var_eeff_dn7 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35810_e41193) as f64).is_finite() && ((assign35810_e41193) as f64).fract() == 0.0 { if assign35810_e41193 == 0.0 { 0.0 } else { (assign35810_e41193 * ((locals.var_eeff).powf(assign35810_e41193 - 1.0) * locals.var_eeff_dn8)) } } else { (assign35810_e41194 * (assign35810_e41193 * (locals.var_eeff_dn8 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35810_e41193) as f64).is_finite() && ((assign35810_e41193) as f64).fract() == 0.0 { if assign35810_e41193 == 0.0 { 0.0 } else { (assign35810_e41193 * ((locals.var_eeff).powf(assign35810_e41193 - 1.0) * locals.var_eeff_dn9)) } } else { (assign35810_e41194 * (assign35810_e41193 * (locals.var_eeff_dn9 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35810_e41193) as f64).is_finite() && ((assign35810_e41193) as f64).fract() == 0.0 { if assign35810_e41193 == 0.0 { 0.0 } else { (assign35810_e41193 * ((locals.var_eeff).powf(assign35810_e41193 - 1.0) * locals.var_eeff_dn10)) } } else { (assign35810_e41194 * (assign35810_e41193 * (locals.var_eeff_dn10 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35810_e41193) as f64).is_finite() && ((assign35810_e41193) as f64).fract() == 0.0 { if assign35810_e41193 == 0.0 { 0.0 } else { (assign35810_e41193 * ((locals.var_eeff).powf(assign35810_e41193 - 1.0) * locals.var_eeff_dn11)) } } else { (assign35810_e41194 * (assign35810_e41193 * (locals.var_eeff_dn11 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35810_e41193) as f64).is_finite() && ((assign35810_e41193) as f64).fract() == 0.0 { if assign35810_e41193 == 0.0 { 0.0 } else { (assign35810_e41193 * ((locals.var_eeff).powf(assign35810_e41193 - 1.0) * locals.var_eeff_dn14)) } } else { (assign35810_e41194 * (assign35810_e41193 * (locals.var_eeff_dn14 / locals.var_eeff))) },)
            }
        };
        (assign35810_e41195, assign35810_e41195_d_n0, assign35810_e41195_d_n2, assign35810_e41195_d_n4, assign35810_e41195_d_n5, assign35810_e41195_d_n6, assign35810_e41195_d_n7, assign35810_e41195_d_n8, assign35810_e41195_d_n9, assign35810_e41195_d_n10, assign35810_e41195_d_n11, assign35810_e41195_d_n14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign35810_e41197;
        locals.var_t5_dn0 = assign35810_e41197_d_n0;
        locals.var_t5_dn2 = assign35810_e41197_d_n2;
        locals.var_t5_dn4 = assign35810_e41197_d_n4;
        locals.var_t5_dn5 = assign35810_e41197_d_n5;
        locals.var_t5_dn6 = assign35810_e41197_d_n6;
        locals.var_t5_dn7 = assign35810_e41197_d_n7;
        locals.var_t5_dn8 = assign35810_e41197_d_n8;
        locals.var_t5_dn9 = assign35810_e41197_d_n9;
        locals.var_t5_dn10 = assign35810_e41197_d_n10;
        locals.var_t5_dn11 = assign35810_e41197_d_n11;
        locals.var_t5_dn14 = assign35810_e41197_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign35820_e41205, assign35820_e41205_d_n0, assign35820_e41205_d_n2, assign35820_e41205_d_n4, assign35820_e41205_d_n5, assign35820_e41205_d_n6, assign35820_e41205_d_n7, assign35820_e41205_d_n8, assign35820_e41205_d_n9, assign35820_e41205_d_n10, assign35820_e41205_d_n11, assign35820_e41205_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign35820_e41203: f64 = (locals.var_t5 * locals.var_eeff);
        (assign35820_e41203, ((locals.var_t5_dn0 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn0)), ((locals.var_t5_dn2 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn2)), ((locals.var_t5_dn4 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn4)), ((locals.var_t5_dn5 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn5)), ((locals.var_t5_dn6 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn6)), ((locals.var_t5_dn7 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn7)), ((locals.var_t5_dn8 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn8)), ((locals.var_t5_dn9 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn9)), ((locals.var_t5_dn10 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn10)), ((locals.var_t5_dn11 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn11)), ((locals.var_t5_dn14 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn14)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign35820_e41205;
        locals.var_t8_dn0 = assign35820_e41205_d_n0;
        locals.var_t8_dn2 = assign35820_e41205_d_n2;
        locals.var_t8_dn4 = assign35820_e41205_d_n4;
        locals.var_t8_dn5 = assign35820_e41205_d_n5;
        locals.var_t8_dn6 = assign35820_e41205_d_n6;
        locals.var_t8_dn7 = assign35820_e41205_d_n7;
        locals.var_t8_dn8 = assign35820_e41205_d_n8;
        locals.var_t8_dn9 = assign35820_e41205_d_n9;
        locals.var_t8_dn10 = assign35820_e41205_d_n10;
        locals.var_t8_dn11 = assign35820_e41205_d_n11;
        locals.var_t8_dn14 = assign35820_e41205_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign35830_e41213, assign35830_e41213_d_n0, assign35830_e41213_d_n2, assign35830_e41213_d_n4, assign35830_e41213_d_n5, assign35830_e41213_d_n6, assign35830_e41213_d_n7, assign35830_e41213_d_n8, assign35830_e41213_d_n9, assign35830_e41213_d_n10, assign35830_e41213_d_n11, assign35830_e41213_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign35830_e41211: f64 = (1.6021918e-19 * 10000.0);
        (assign35830_e41211, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign35830_e41213;
        locals.var_t9_dn0 = assign35830_e41213_d_n0;
        locals.var_t9_dn2 = assign35830_e41213_d_n2;
        locals.var_t9_dn4 = assign35830_e41213_d_n4;
        locals.var_t9_dn5 = assign35830_e41213_d_n5;
        locals.var_t9_dn6 = assign35830_e41213_d_n6;
        locals.var_t9_dn7 = assign35830_e41213_d_n7;
        locals.var_t9_dn8 = assign35830_e41213_d_n8;
        locals.var_t9_dn9 = assign35830_e41213_d_n9;
        locals.var_t9_dn10 = assign35830_e41213_d_n10;
        locals.var_t9_dn11 = assign35830_e41213_d_n11;
        locals.var_t9_dn14 = assign35830_e41213_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign35840_e41221, assign35840_e41221_d_n0, assign35840_e41221_d_n2, assign35840_e41221_d_n4, assign35840_e41221_d_n5, assign35840_e41221_d_n6, assign35840_e41221_d_n7, assign35840_e41221_d_n8, assign35840_e41221_d_n9, assign35840_e41221_d_n10, assign35840_e41221_d_n11, assign35840_e41221_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign35840_e41219: f64 = (locals.var_qiu / locals.var_t9);
        (assign35840_e41219, (((locals.var_qiu_dn0 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn0)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn2 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn2)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn4 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn4)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn5 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn5)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn6 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn6)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn7 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn7)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn8 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn8)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn9 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn9)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn10 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn10)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn11 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn11)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn14 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn14)) / (locals.var_t9 * locals.var_t9)),)
    } else {
        (locals.var_rns, locals.var_rns_dn0, locals.var_rns_dn2, locals.var_rns_dn4, locals.var_rns_dn5, locals.var_rns_dn6, locals.var_rns_dn7, locals.var_rns_dn8, locals.var_rns_dn9, locals.var_rns_dn10, locals.var_rns_dn11, locals.var_rns_dn14,)
    }
};
        locals.var_rns = assign35840_e41221;
        locals.var_rns_dn0 = assign35840_e41221_d_n0;
        locals.var_rns_dn2 = assign35840_e41221_d_n2;
        locals.var_rns_dn4 = assign35840_e41221_d_n4;
        locals.var_rns_dn5 = assign35840_e41221_d_n5;
        locals.var_rns_dn6 = assign35840_e41221_d_n6;
        locals.var_rns_dn7 = assign35840_e41221_d_n7;
        locals.var_rns_dn8 = assign35840_e41221_d_n8;
        locals.var_rns_dn9 = assign35840_e41221_d_n9;
        locals.var_rns_dn10 = assign35840_e41221_d_n10;
        locals.var_rns_dn11 = assign35840_e41221_d_n11;
        locals.var_rns_dn14 = assign35840_e41221_d_n14;
        locals.var_rns_rv = 0.0;

        let (assign35850_e41241, assign35850_e41241_d_n0, assign35850_e41241_d_n2, assign35850_e41241_d_n4, assign35850_e41241_d_n5, assign35850_e41241_d_n6, assign35850_e41241_d_n7, assign35850_e41241_d_n8, assign35850_e41241_d_n9, assign35850_e41241_d_n10, assign35850_e41241_d_n11, assign35850_e41241_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign35850_e41229: f64 = (locals.var_uc_depmue1 * locals.var_rns);
        let assign35850_e41231: f64 = (assign35850_e41229 / 100000000000.0);
        let assign35850_e41232: f64 = (locals.var_uc_depmue0 + assign35850_e41231);
        let assign35850_e41234: f64 = (assign35850_e41232 + 1e-25);
        let assign35850_e41235: f64 = (1.0 / assign35850_e41234);
        let assign35850_e41238: f64 = (locals.var_depmphn0 * locals.var_t8);
        let assign35850_e41239: f64 = (assign35850_e41235 + assign35850_e41238);
        (assign35850_e41239, ((-((locals.var_uc_depmue0_dn0 + (((locals.var_uc_depmue1_dn0 * locals.var_rns) + (locals.var_uc_depmue1 * locals.var_rns_dn0)) / 100000000000.0)) / (assign35850_e41234 * assign35850_e41234))) + ((locals.var_depmphn0_dn0 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn0))), ((-((locals.var_uc_depmue0_dn2 + (((locals.var_uc_depmue1_dn2 * locals.var_rns) + (locals.var_uc_depmue1 * locals.var_rns_dn2)) / 100000000000.0)) / (assign35850_e41234 * assign35850_e41234))) + ((locals.var_depmphn0_dn2 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn2))), ((-((locals.var_uc_depmue0_dn4 + (((locals.var_uc_depmue1_dn4 * locals.var_rns) + (locals.var_uc_depmue1 * locals.var_rns_dn4)) / 100000000000.0)) / (assign35850_e41234 * assign35850_e41234))) + ((locals.var_depmphn0_dn4 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn4))), ((-((locals.var_uc_depmue0_dn5 + (((locals.var_uc_depmue1_dn5 * locals.var_rns) + (locals.var_uc_depmue1 * locals.var_rns_dn5)) / 100000000000.0)) / (assign35850_e41234 * assign35850_e41234))) + ((locals.var_depmphn0_dn5 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn5))), ((-((locals.var_uc_depmue0_dn6 + (((locals.var_uc_depmue1_dn6 * locals.var_rns) + (locals.var_uc_depmue1 * locals.var_rns_dn6)) / 100000000000.0)) / (assign35850_e41234 * assign35850_e41234))) + ((locals.var_depmphn0_dn6 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn6))), ((-((locals.var_uc_depmue0_dn7 + (((locals.var_uc_depmue1_dn7 * locals.var_rns) + (locals.var_uc_depmue1 * locals.var_rns_dn7)) / 100000000000.0)) / (assign35850_e41234 * assign35850_e41234))) + ((locals.var_depmphn0_dn7 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn7))), ((-((locals.var_uc_depmue0_dn8 + (((locals.var_uc_depmue1_dn8 * locals.var_rns) + (locals.var_uc_depmue1 * locals.var_rns_dn8)) / 100000000000.0)) / (assign35850_e41234 * assign35850_e41234))) + ((locals.var_depmphn0_dn8 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn8))), ((-((locals.var_uc_depmue0_dn9 + (((locals.var_uc_depmue1_dn9 * locals.var_rns) + (locals.var_uc_depmue1 * locals.var_rns_dn9)) / 100000000000.0)) / (assign35850_e41234 * assign35850_e41234))) + ((locals.var_depmphn0_dn9 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn9))), ((-((locals.var_uc_depmue0_dn10 + (((locals.var_uc_depmue1_dn10 * locals.var_rns) + (locals.var_uc_depmue1 * locals.var_rns_dn10)) / 100000000000.0)) / (assign35850_e41234 * assign35850_e41234))) + ((locals.var_depmphn0_dn10 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn10))), ((-((locals.var_uc_depmue0_dn11 + (((locals.var_uc_depmue1_dn11 * locals.var_rns) + (locals.var_uc_depmue1 * locals.var_rns_dn11)) / 100000000000.0)) / (assign35850_e41234 * assign35850_e41234))) + ((locals.var_depmphn0_dn11 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn11))), ((-((locals.var_uc_depmue0_dn14 + (((locals.var_uc_depmue1_dn14 * locals.var_rns) + (locals.var_uc_depmue1 * locals.var_rns_dn14)) / 100000000000.0)) / (assign35850_e41234 * assign35850_e41234))) + ((locals.var_depmphn0_dn14 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign35850_e41241;
        locals.var_t1_dn0 = assign35850_e41241_d_n0;
        locals.var_t1_dn2 = assign35850_e41241_d_n2;
        locals.var_t1_dn4 = assign35850_e41241_d_n4;
        locals.var_t1_dn5 = assign35850_e41241_d_n5;
        locals.var_t1_dn6 = assign35850_e41241_d_n6;
        locals.var_t1_dn7 = assign35850_e41241_d_n7;
        locals.var_t1_dn8 = assign35850_e41241_d_n8;
        locals.var_t1_dn9 = assign35850_e41241_d_n9;
        locals.var_t1_dn10 = assign35850_e41241_d_n10;
        locals.var_t1_dn11 = assign35850_e41241_d_n11;
        locals.var_t1_dn14 = assign35850_e41241_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign35860_e41249, assign35860_e41249_d_n0, assign35860_e41249_d_n2, assign35860_e41249_d_n4, assign35860_e41249_d_n5, assign35860_e41249_d_n6, assign35860_e41249_d_n7, assign35860_e41249_d_n8, assign35860_e41249_d_n9, assign35860_e41249_d_n10, assign35860_e41249_d_n11, assign35860_e41249_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign35860_e41247: f64 = (1.0 / locals.var_t1);
        (assign35860_e41247, (-(locals.var_t1_dn0 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn2 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn11 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn14 / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn14,)
    }
};
        locals.var_muun = assign35860_e41249;
        locals.var_muun_dn0 = assign35860_e41249_d_n0;
        locals.var_muun_dn2 = assign35860_e41249_d_n2;
        locals.var_muun_dn4 = assign35860_e41249_d_n4;
        locals.var_muun_dn5 = assign35860_e41249_d_n5;
        locals.var_muun_dn6 = assign35860_e41249_d_n6;
        locals.var_muun_dn7 = assign35860_e41249_d_n7;
        locals.var_muun_dn8 = assign35860_e41249_d_n8;
        locals.var_muun_dn9 = assign35860_e41249_d_n9;
        locals.var_muun_dn10 = assign35860_e41249_d_n10;
        locals.var_muun_dn11 = assign35860_e41249_d_n11;
        locals.var_muun_dn14 = assign35860_e41249_d_n14;
        locals.var_muun_rv = 0.0;

        let (assign35870_e41257, assign35870_e41257_d_n0, assign35870_e41257_d_n2, assign35870_e41257_d_n4, assign35870_e41257_d_n5, assign35870_e41257_d_n6, assign35870_e41257_d_n7, assign35870_e41257_d_n8, assign35870_e41257_d_n9, assign35870_e41257_d_n10, assign35870_e41257_d_n11, assign35870_e41257_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign35870_e41255: f64 = (locals.var_muun / 10000.0);
        (assign35870_e41255, (locals.var_muun_dn0 / 10000.0), (locals.var_muun_dn2 / 10000.0), (locals.var_muun_dn4 / 10000.0), (locals.var_muun_dn5 / 10000.0), (locals.var_muun_dn6 / 10000.0), (locals.var_muun_dn7 / 10000.0), (locals.var_muun_dn8 / 10000.0), (locals.var_muun_dn9 / 10000.0), (locals.var_muun_dn10 / 10000.0), (locals.var_muun_dn11 / 10000.0), (locals.var_muun_dn14 / 10000.0),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn14,)
    }
};
        locals.var_muun = assign35870_e41257;
        locals.var_muun_dn0 = assign35870_e41257_d_n0;
        locals.var_muun_dn2 = assign35870_e41257_d_n2;
        locals.var_muun_dn4 = assign35870_e41257_d_n4;
        locals.var_muun_dn5 = assign35870_e41257_d_n5;
        locals.var_muun_dn6 = assign35870_e41257_d_n6;
        locals.var_muun_dn7 = assign35870_e41257_d_n7;
        locals.var_muun_dn8 = assign35870_e41257_d_n8;
        locals.var_muun_dn9 = assign35870_e41257_d_n9;
        locals.var_muun_dn10 = assign35870_e41257_d_n10;
        locals.var_muun_dn11 = assign35870_e41257_d_n11;
        locals.var_muun_dn14 = assign35870_e41257_d_n14;
        locals.var_muun_rv = 0.0;

        let (assign35880_e41265, assign35880_e41265_d_n0, assign35880_e41265_d_n2, assign35880_e41265_d_n4, assign35880_e41265_d_n5, assign35880_e41265_d_n6, assign35880_e41265_d_n7, assign35880_e41265_d_n8, assign35880_e41265_d_n9, assign35880_e41265_d_n10, assign35880_e41265_d_n11, assign35880_e41265_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign35880_e41263: f64 = (locals.var_vdseff0 / locals.var_lch);
        (assign35880_e41263, (((locals.var_vdseff0_dn0 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn2 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn4 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn5 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn6 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn7 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn8 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn9 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn10 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn11 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn14 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn14)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_edri__blk557, locals.var_edri__blk557_dn0, locals.var_edri__blk557_dn2, locals.var_edri__blk557_dn4, locals.var_edri__blk557_dn5, locals.var_edri__blk557_dn6, locals.var_edri__blk557_dn7, locals.var_edri__blk557_dn8, locals.var_edri__blk557_dn9, locals.var_edri__blk557_dn10, locals.var_edri__blk557_dn11, locals.var_edri__blk557_dn14,)
    }
};
        locals.var_edri__blk557 = assign35880_e41265;
        locals.var_edri__blk557_dn0 = assign35880_e41265_d_n0;
        locals.var_edri__blk557_dn2 = assign35880_e41265_d_n2;
        locals.var_edri__blk557_dn4 = assign35880_e41265_d_n4;
        locals.var_edri__blk557_dn5 = assign35880_e41265_d_n5;
        locals.var_edri__blk557_dn6 = assign35880_e41265_d_n6;
        locals.var_edri__blk557_dn7 = assign35880_e41265_d_n7;
        locals.var_edri__blk557_dn8 = assign35880_e41265_d_n8;
        locals.var_edri__blk557_dn9 = assign35880_e41265_d_n9;
        locals.var_edri__blk557_dn10 = assign35880_e41265_d_n10;
        locals.var_edri__blk557_dn11 = assign35880_e41265_d_n11;
        locals.var_edri__blk557_dn14 = assign35880_e41265_d_n14;
        locals.var_edri__blk557_rv = 0.0;

        let (assign35890_e41275, assign35890_e41275_d_n0, assign35890_e41275_d_n2, assign35890_e41275_d_n4, assign35890_e41275_d_n5, assign35890_e41275_d_n6, assign35890_e41275_d_n7, assign35890_e41275_d_n8, assign35890_e41275_d_n9, assign35890_e41275_d_n10, assign35890_e41275_d_n11, assign35890_e41275_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign35890_e41271: f64 = (locals.var_muun * locals.var_edri__blk557);
        let assign35890_e41273: f64 = (assign35890_e41271 / locals.var_uc_depvmax);
        (assign35890_e41273, (((((locals.var_muun_dn0 * locals.var_edri__blk557) + (locals.var_muun * locals.var_edri__blk557_dn0)) * locals.var_uc_depvmax) - (assign35890_e41271 * locals.var_uc_depvmax_dn0)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn2 * locals.var_edri__blk557) + (locals.var_muun * locals.var_edri__blk557_dn2)) * locals.var_uc_depvmax) - (assign35890_e41271 * locals.var_uc_depvmax_dn2)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn4 * locals.var_edri__blk557) + (locals.var_muun * locals.var_edri__blk557_dn4)) * locals.var_uc_depvmax) - (assign35890_e41271 * locals.var_uc_depvmax_dn4)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn5 * locals.var_edri__blk557) + (locals.var_muun * locals.var_edri__blk557_dn5)) * locals.var_uc_depvmax) - (assign35890_e41271 * locals.var_uc_depvmax_dn5)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn6 * locals.var_edri__blk557) + (locals.var_muun * locals.var_edri__blk557_dn6)) * locals.var_uc_depvmax) - (assign35890_e41271 * locals.var_uc_depvmax_dn6)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn7 * locals.var_edri__blk557) + (locals.var_muun * locals.var_edri__blk557_dn7)) * locals.var_uc_depvmax) - (assign35890_e41271 * locals.var_uc_depvmax_dn7)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn8 * locals.var_edri__blk557) + (locals.var_muun * locals.var_edri__blk557_dn8)) * locals.var_uc_depvmax) - (assign35890_e41271 * locals.var_uc_depvmax_dn8)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn9 * locals.var_edri__blk557) + (locals.var_muun * locals.var_edri__blk557_dn9)) * locals.var_uc_depvmax) - (assign35890_e41271 * locals.var_uc_depvmax_dn9)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn10 * locals.var_edri__blk557) + (locals.var_muun * locals.var_edri__blk557_dn10)) * locals.var_uc_depvmax) - (assign35890_e41271 * locals.var_uc_depvmax_dn10)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn11 * locals.var_edri__blk557) + (locals.var_muun * locals.var_edri__blk557_dn11)) * locals.var_uc_depvmax) - (assign35890_e41271 * locals.var_uc_depvmax_dn11)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn14 * locals.var_edri__blk557) + (locals.var_muun * locals.var_edri__blk557_dn14)) * locals.var_uc_depvmax) - (assign35890_e41271 * locals.var_uc_depvmax_dn14)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign35890_e41275;
        locals.var_t1_dn0 = assign35890_e41275_d_n0;
        locals.var_t1_dn2 = assign35890_e41275_d_n2;
        locals.var_t1_dn4 = assign35890_e41275_d_n4;
        locals.var_t1_dn5 = assign35890_e41275_d_n5;
        locals.var_t1_dn6 = assign35890_e41275_d_n6;
        locals.var_t1_dn7 = assign35890_e41275_d_n7;
        locals.var_t1_dn8 = assign35890_e41275_d_n8;
        locals.var_t1_dn9 = assign35890_e41275_d_n9;
        locals.var_t1_dn10 = assign35890_e41275_d_n10;
        locals.var_t1_dn11 = assign35890_e41275_d_n11;
        locals.var_t1_dn14 = assign35890_e41275_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign35900_e41288, assign35900_e41288_d_n0, assign35900_e41288_d_n2, assign35900_e41288_d_n4, assign35900_e41288_d_n5, assign35900_e41288_d_n6, assign35900_e41288_d_n7, assign35900_e41288_d_n8, assign35900_e41288_d_n9, assign35900_e41288_d_n10, assign35900_e41288_d_n11, assign35900_e41288_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let (assign35900_e41286, assign35900_e41286_d_n0, assign35900_e41286_d_n2, assign35900_e41286_d_n4, assign35900_e41286_d_n5, assign35900_e41286_d_n6, assign35900_e41286_d_n7, assign35900_e41286_d_n8, assign35900_e41286_d_n9, assign35900_e41286_d_n10, assign35900_e41286_d_n11, assign35900_e41286_d_n14,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign35900_e41285: f64 = (locals.var_t1).powf(p.p378);
                (assign35900_e41285, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn0)) } } else { (assign35900_e41285 * (p.p378 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn2)) } } else { (assign35900_e41285 * (p.p378 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn4)) } } else { (assign35900_e41285 * (p.p378 * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn5)) } } else { (assign35900_e41285 * (p.p378 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn6)) } } else { (assign35900_e41285 * (p.p378 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn7)) } } else { (assign35900_e41285 * (p.p378 * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn8)) } } else { (assign35900_e41285 * (p.p378 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn9)) } } else { (assign35900_e41285 * (p.p378 * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn10)) } } else { (assign35900_e41285 * (p.p378 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn11)) } } else { (assign35900_e41285 * (p.p378 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn14)) } } else { (assign35900_e41285 * (p.p378 * (locals.var_t1_dn14 / locals.var_t1))) },)
            }
        };
        (assign35900_e41286, assign35900_e41286_d_n0, assign35900_e41286_d_n2, assign35900_e41286_d_n4, assign35900_e41286_d_n5, assign35900_e41286_d_n6, assign35900_e41286_d_n7, assign35900_e41286_d_n8, assign35900_e41286_d_n9, assign35900_e41286_d_n10, assign35900_e41286_d_n11, assign35900_e41286_d_n14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign35900_e41288;
        locals.var_t2_dn0 = assign35900_e41288_d_n0;
        locals.var_t2_dn2 = assign35900_e41288_d_n2;
        locals.var_t2_dn4 = assign35900_e41288_d_n4;
        locals.var_t2_dn5 = assign35900_e41288_d_n5;
        locals.var_t2_dn6 = assign35900_e41288_d_n6;
        locals.var_t2_dn7 = assign35900_e41288_d_n7;
        locals.var_t2_dn8 = assign35900_e41288_d_n8;
        locals.var_t2_dn9 = assign35900_e41288_d_n9;
        locals.var_t2_dn10 = assign35900_e41288_d_n10;
        locals.var_t2_dn11 = assign35900_e41288_d_n11;
        locals.var_t2_dn14 = assign35900_e41288_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign35910_e41296, assign35910_e41296_d_n0, assign35910_e41296_d_n2, assign35910_e41296_d_n4, assign35910_e41296_d_n5, assign35910_e41296_d_n6, assign35910_e41296_d_n7, assign35910_e41296_d_n8, assign35910_e41296_d_n9, assign35910_e41296_d_n10, assign35910_e41296_d_n11, assign35910_e41296_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign35910_e41294: f64 = (1.0 + locals.var_t2);
        (assign35910_e41294, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign35910_e41296;
        locals.var_t3_dn0 = assign35910_e41296_d_n0;
        locals.var_t3_dn2 = assign35910_e41296_d_n2;
        locals.var_t3_dn4 = assign35910_e41296_d_n4;
        locals.var_t3_dn5 = assign35910_e41296_d_n5;
        locals.var_t3_dn6 = assign35910_e41296_d_n6;
        locals.var_t3_dn7 = assign35910_e41296_d_n7;
        locals.var_t3_dn8 = assign35910_e41296_d_n8;
        locals.var_t3_dn9 = assign35910_e41296_d_n9;
        locals.var_t3_dn10 = assign35910_e41296_d_n10;
        locals.var_t3_dn11 = assign35910_e41296_d_n11;
        locals.var_t3_dn14 = assign35910_e41296_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign35920_e41311, assign35920_e41311_d_n0, assign35920_e41311_d_n2, assign35920_e41311_d_n4, assign35920_e41311_d_n5, assign35920_e41311_d_n6, assign35920_e41311_d_n7, assign35920_e41311_d_n8, assign35920_e41311_d_n9, assign35920_e41311_d_n10, assign35920_e41311_d_n11, assign35920_e41311_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let (assign35920_e41309, assign35920_e41309_d_n0, assign35920_e41309_d_n2, assign35920_e41309_d_n4, assign35920_e41309_d_n5, assign35920_e41309_d_n6, assign35920_e41309_d_n7, assign35920_e41309_d_n8, assign35920_e41309_d_n9, assign35920_e41309_d_n10, assign35920_e41309_d_n11, assign35920_e41309_d_n14,) = {
            if (locals.var_t3 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign35920_e41307: f64 = (1.0 / p.p378);
                let assign35920_e41308: f64 = (locals.var_t3).powf(assign35920_e41307);
                (assign35920_e41308, if 0.0 == 0.0 && ((assign35920_e41307) as f64).is_finite() && ((assign35920_e41307) as f64).fract() == 0.0 { if assign35920_e41307 == 0.0 { 0.0 } else { (assign35920_e41307 * ((locals.var_t3).powf(assign35920_e41307 - 1.0) * locals.var_t3_dn0)) } } else { (assign35920_e41308 * (assign35920_e41307 * (locals.var_t3_dn0 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign35920_e41307) as f64).is_finite() && ((assign35920_e41307) as f64).fract() == 0.0 { if assign35920_e41307 == 0.0 { 0.0 } else { (assign35920_e41307 * ((locals.var_t3).powf(assign35920_e41307 - 1.0) * locals.var_t3_dn2)) } } else { (assign35920_e41308 * (assign35920_e41307 * (locals.var_t3_dn2 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign35920_e41307) as f64).is_finite() && ((assign35920_e41307) as f64).fract() == 0.0 { if assign35920_e41307 == 0.0 { 0.0 } else { (assign35920_e41307 * ((locals.var_t3).powf(assign35920_e41307 - 1.0) * locals.var_t3_dn4)) } } else { (assign35920_e41308 * (assign35920_e41307 * (locals.var_t3_dn4 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign35920_e41307) as f64).is_finite() && ((assign35920_e41307) as f64).fract() == 0.0 { if assign35920_e41307 == 0.0 { 0.0 } else { (assign35920_e41307 * ((locals.var_t3).powf(assign35920_e41307 - 1.0) * locals.var_t3_dn5)) } } else { (assign35920_e41308 * (assign35920_e41307 * (locals.var_t3_dn5 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign35920_e41307) as f64).is_finite() && ((assign35920_e41307) as f64).fract() == 0.0 { if assign35920_e41307 == 0.0 { 0.0 } else { (assign35920_e41307 * ((locals.var_t3).powf(assign35920_e41307 - 1.0) * locals.var_t3_dn6)) } } else { (assign35920_e41308 * (assign35920_e41307 * (locals.var_t3_dn6 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign35920_e41307) as f64).is_finite() && ((assign35920_e41307) as f64).fract() == 0.0 { if assign35920_e41307 == 0.0 { 0.0 } else { (assign35920_e41307 * ((locals.var_t3).powf(assign35920_e41307 - 1.0) * locals.var_t3_dn7)) } } else { (assign35920_e41308 * (assign35920_e41307 * (locals.var_t3_dn7 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign35920_e41307) as f64).is_finite() && ((assign35920_e41307) as f64).fract() == 0.0 { if assign35920_e41307 == 0.0 { 0.0 } else { (assign35920_e41307 * ((locals.var_t3).powf(assign35920_e41307 - 1.0) * locals.var_t3_dn8)) } } else { (assign35920_e41308 * (assign35920_e41307 * (locals.var_t3_dn8 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign35920_e41307) as f64).is_finite() && ((assign35920_e41307) as f64).fract() == 0.0 { if assign35920_e41307 == 0.0 { 0.0 } else { (assign35920_e41307 * ((locals.var_t3).powf(assign35920_e41307 - 1.0) * locals.var_t3_dn9)) } } else { (assign35920_e41308 * (assign35920_e41307 * (locals.var_t3_dn9 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign35920_e41307) as f64).is_finite() && ((assign35920_e41307) as f64).fract() == 0.0 { if assign35920_e41307 == 0.0 { 0.0 } else { (assign35920_e41307 * ((locals.var_t3).powf(assign35920_e41307 - 1.0) * locals.var_t3_dn10)) } } else { (assign35920_e41308 * (assign35920_e41307 * (locals.var_t3_dn10 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign35920_e41307) as f64).is_finite() && ((assign35920_e41307) as f64).fract() == 0.0 { if assign35920_e41307 == 0.0 { 0.0 } else { (assign35920_e41307 * ((locals.var_t3).powf(assign35920_e41307 - 1.0) * locals.var_t3_dn11)) } } else { (assign35920_e41308 * (assign35920_e41307 * (locals.var_t3_dn11 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign35920_e41307) as f64).is_finite() && ((assign35920_e41307) as f64).fract() == 0.0 { if assign35920_e41307 == 0.0 { 0.0 } else { (assign35920_e41307 * ((locals.var_t3).powf(assign35920_e41307 - 1.0) * locals.var_t3_dn14)) } } else { (assign35920_e41308 * (assign35920_e41307 * (locals.var_t3_dn14 / locals.var_t3))) },)
            }
        };
        (assign35920_e41309, assign35920_e41309_d_n0, assign35920_e41309_d_n2, assign35920_e41309_d_n4, assign35920_e41309_d_n5, assign35920_e41309_d_n6, assign35920_e41309_d_n7, assign35920_e41309_d_n8, assign35920_e41309_d_n9, assign35920_e41309_d_n10, assign35920_e41309_d_n11, assign35920_e41309_d_n14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign35920_e41311;
        locals.var_t4_dn0 = assign35920_e41311_d_n0;
        locals.var_t4_dn2 = assign35920_e41311_d_n2;
        locals.var_t4_dn4 = assign35920_e41311_d_n4;
        locals.var_t4_dn5 = assign35920_e41311_d_n5;
        locals.var_t4_dn6 = assign35920_e41311_d_n6;
        locals.var_t4_dn7 = assign35920_e41311_d_n7;
        locals.var_t4_dn8 = assign35920_e41311_d_n8;
        locals.var_t4_dn9 = assign35920_e41311_d_n9;
        locals.var_t4_dn10 = assign35920_e41311_d_n10;
        locals.var_t4_dn11 = assign35920_e41311_d_n11;
        locals.var_t4_dn14 = assign35920_e41311_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign35930_e41319, assign35930_e41319_d_n0, assign35930_e41319_d_n2, assign35930_e41319_d_n4, assign35930_e41319_d_n5, assign35930_e41319_d_n6, assign35930_e41319_d_n7, assign35930_e41319_d_n8, assign35930_e41319_d_n9, assign35930_e41319_d_n10, assign35930_e41319_d_n11, assign35930_e41319_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign35930_e41317: f64 = (locals.var_muun / locals.var_t4);
        (assign35930_e41317, (((locals.var_muun_dn0 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn2 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn4 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn5 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn6 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn7 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn8 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn9 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn10 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn11 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn14 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_mu_res__blk508, locals.var_mu_res__blk508_dn0, locals.var_mu_res__blk508_dn2, locals.var_mu_res__blk508_dn4, locals.var_mu_res__blk508_dn5, locals.var_mu_res__blk508_dn6, locals.var_mu_res__blk508_dn7, locals.var_mu_res__blk508_dn8, locals.var_mu_res__blk508_dn9, locals.var_mu_res__blk508_dn10, locals.var_mu_res__blk508_dn11, locals.var_mu_res__blk508_dn14,)
    }
};
        locals.var_mu_res__blk508 = assign35930_e41319;
        locals.var_mu_res__blk508_dn0 = assign35930_e41319_d_n0;
        locals.var_mu_res__blk508_dn2 = assign35930_e41319_d_n2;
        locals.var_mu_res__blk508_dn4 = assign35930_e41319_d_n4;
        locals.var_mu_res__blk508_dn5 = assign35930_e41319_d_n5;
        locals.var_mu_res__blk508_dn6 = assign35930_e41319_d_n6;
        locals.var_mu_res__blk508_dn7 = assign35930_e41319_d_n7;
        locals.var_mu_res__blk508_dn8 = assign35930_e41319_d_n8;
        locals.var_mu_res__blk508_dn9 = assign35930_e41319_d_n9;
        locals.var_mu_res__blk508_dn10 = assign35930_e41319_d_n10;
        locals.var_mu_res__blk508_dn11 = assign35930_e41319_d_n11;
        locals.var_mu_res__blk508_dn14 = assign35930_e41319_d_n14;
        locals.var_mu_res__blk508_rv = 0.0;

        let (assign35940_e41332, assign35940_e41332_d_n0, assign35940_e41332_d_n2, assign35940_e41332_d_n4, assign35940_e41332_d_n5, assign35940_e41332_d_n6, assign35940_e41332_d_n7, assign35940_e41332_d_n8, assign35940_e41332_d_n9, assign35940_e41332_d_n10, assign35940_e41332_d_n11, assign35940_e41332_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign35940_e41325: f64 = (-locals.var_qn_res0);
        let assign35940_e41326: f64 = (locals.var_weff_nf * assign35940_e41325);
        let assign35940_e41328: f64 = (assign35940_e41326 * locals.var_mu_res__blk508);
        let assign35940_e41330: f64 = (assign35940_e41328 * locals.var_edri__blk557);
        (assign35940_e41330, (((((locals.var_weff_nf * (-locals.var_qn_res0_dn0)) * locals.var_mu_res__blk508) + (assign35940_e41326 * locals.var_mu_res__blk508_dn0)) * locals.var_edri__blk557) + (assign35940_e41328 * locals.var_edri__blk557_dn0)), (((((locals.var_weff_nf * (-locals.var_qn_res0_dn2)) * locals.var_mu_res__blk508) + (assign35940_e41326 * locals.var_mu_res__blk508_dn2)) * locals.var_edri__blk557) + (assign35940_e41328 * locals.var_edri__blk557_dn2)), (((((locals.var_weff_nf * (-locals.var_qn_res0_dn4)) * locals.var_mu_res__blk508) + (assign35940_e41326 * locals.var_mu_res__blk508_dn4)) * locals.var_edri__blk557) + (assign35940_e41328 * locals.var_edri__blk557_dn4)), (((((locals.var_weff_nf * (-locals.var_qn_res0_dn5)) * locals.var_mu_res__blk508) + (assign35940_e41326 * locals.var_mu_res__blk508_dn5)) * locals.var_edri__blk557) + (assign35940_e41328 * locals.var_edri__blk557_dn5)), (((((locals.var_weff_nf * (-locals.var_qn_res0_dn6)) * locals.var_mu_res__blk508) + (assign35940_e41326 * locals.var_mu_res__blk508_dn6)) * locals.var_edri__blk557) + (assign35940_e41328 * locals.var_edri__blk557_dn6)), (((((locals.var_weff_nf * (-locals.var_qn_res0_dn7)) * locals.var_mu_res__blk508) + (assign35940_e41326 * locals.var_mu_res__blk508_dn7)) * locals.var_edri__blk557) + (assign35940_e41328 * locals.var_edri__blk557_dn7)), (((((locals.var_weff_nf * (-locals.var_qn_res0_dn8)) * locals.var_mu_res__blk508) + (assign35940_e41326 * locals.var_mu_res__blk508_dn8)) * locals.var_edri__blk557) + (assign35940_e41328 * locals.var_edri__blk557_dn8)), (((((locals.var_weff_nf * (-locals.var_qn_res0_dn9)) * locals.var_mu_res__blk508) + (assign35940_e41326 * locals.var_mu_res__blk508_dn9)) * locals.var_edri__blk557) + (assign35940_e41328 * locals.var_edri__blk557_dn9)), (((((locals.var_weff_nf * (-locals.var_qn_res0_dn10)) * locals.var_mu_res__blk508) + (assign35940_e41326 * locals.var_mu_res__blk508_dn10)) * locals.var_edri__blk557) + (assign35940_e41328 * locals.var_edri__blk557_dn10)), (((((locals.var_weff_nf * (-locals.var_qn_res0_dn11)) * locals.var_mu_res__blk508) + (assign35940_e41326 * locals.var_mu_res__blk508_dn11)) * locals.var_edri__blk557) + (assign35940_e41328 * locals.var_edri__blk557_dn11)), (((((locals.var_weff_nf * (-locals.var_qn_res0_dn14)) * locals.var_mu_res__blk508) + (assign35940_e41326 * locals.var_mu_res__blk508_dn14)) * locals.var_edri__blk557) + (assign35940_e41328 * locals.var_edri__blk557_dn14)),)
    } else {
        (locals.var_ids_res, locals.var_ids_res_dn0, locals.var_ids_res_dn2, locals.var_ids_res_dn4, locals.var_ids_res_dn5, locals.var_ids_res_dn6, locals.var_ids_res_dn7, locals.var_ids_res_dn8, locals.var_ids_res_dn9, locals.var_ids_res_dn10, locals.var_ids_res_dn11, locals.var_ids_res_dn14,)
    }
};
        locals.var_ids_res = assign35940_e41332;
        locals.var_ids_res_dn0 = assign35940_e41332_d_n0;
        locals.var_ids_res_dn2 = assign35940_e41332_d_n2;
        locals.var_ids_res_dn4 = assign35940_e41332_d_n4;
        locals.var_ids_res_dn5 = assign35940_e41332_d_n5;
        locals.var_ids_res_dn6 = assign35940_e41332_d_n6;
        locals.var_ids_res_dn7 = assign35940_e41332_d_n7;
        locals.var_ids_res_dn8 = assign35940_e41332_d_n8;
        locals.var_ids_res_dn9 = assign35940_e41332_d_n9;
        locals.var_ids_res_dn10 = assign35940_e41332_d_n10;
        locals.var_ids_res_dn11 = assign35940_e41332_d_n11;
        locals.var_ids_res_dn14 = assign35940_e41332_d_n14;
        locals.var_ids_res_rv = 0.0;

        let (assign35950_e41344, assign35950_e41344_d_n0, assign35950_e41344_d_n2, assign35950_e41344_d_n4, assign35950_e41344_d_n5, assign35950_e41344_d_n6, assign35950_e41344_d_n7, assign35950_e41344_d_n8, assign35950_e41344_d_n9, assign35950_e41344_d_n10, assign35950_e41344_d_n11, assign35950_e41344_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign35950_e41339: f64 = (locals.var_phi_sl_dep - locals.var_phi_s0_dep);
        let assign35950_e41341: f64 = (assign35950_e41339 * locals.var_ninvde);
        let assign35950_e41342: f64 = (1.0 + assign35950_e41341);
        (assign35950_e41342, (((locals.var_phi_sl_dep_dn0 - locals.var_phi_s0_dep_dn0) * locals.var_ninvde) + (assign35950_e41339 * locals.var_ninvde_dn0)), (((locals.var_phi_sl_dep_dn2 - locals.var_phi_s0_dep_dn2) * locals.var_ninvde) + (assign35950_e41339 * locals.var_ninvde_dn2)), (((locals.var_phi_sl_dep_dn4 - locals.var_phi_s0_dep_dn4) * locals.var_ninvde) + (assign35950_e41339 * locals.var_ninvde_dn4)), (((locals.var_phi_sl_dep_dn5 - locals.var_phi_s0_dep_dn5) * locals.var_ninvde) + (assign35950_e41339 * locals.var_ninvde_dn5)), (((locals.var_phi_sl_dep_dn6 - locals.var_phi_s0_dep_dn6) * locals.var_ninvde) + (assign35950_e41339 * locals.var_ninvde_dn6)), (((locals.var_phi_sl_dep_dn7 - locals.var_phi_s0_dep_dn7) * locals.var_ninvde) + (assign35950_e41339 * locals.var_ninvde_dn7)), (((locals.var_phi_sl_dep_dn8 - locals.var_phi_s0_dep_dn8) * locals.var_ninvde) + (assign35950_e41339 * locals.var_ninvde_dn8)), (((locals.var_phi_sl_dep_dn9 - locals.var_phi_s0_dep_dn9) * locals.var_ninvde) + (assign35950_e41339 * locals.var_ninvde_dn9)), (((locals.var_phi_sl_dep_dn10 - locals.var_phi_s0_dep_dn10) * locals.var_ninvde) + (assign35950_e41339 * locals.var_ninvde_dn10)), (((locals.var_phi_sl_dep_dn11 - locals.var_phi_s0_dep_dn11) * locals.var_ninvde) + (assign35950_e41339 * locals.var_ninvde_dn11)), (((locals.var_phi_sl_dep_dn14 - locals.var_phi_s0_dep_dn14) * locals.var_ninvde) + (assign35950_e41339 * locals.var_ninvde_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign35950_e41344;
        locals.var_t4_dn0 = assign35950_e41344_d_n0;
        locals.var_t4_dn2 = assign35950_e41344_d_n2;
        locals.var_t4_dn4 = assign35950_e41344_d_n4;
        locals.var_t4_dn5 = assign35950_e41344_d_n5;
        locals.var_t4_dn6 = assign35950_e41344_d_n6;
        locals.var_t4_dn7 = assign35950_e41344_d_n7;
        locals.var_t4_dn8 = assign35950_e41344_d_n8;
        locals.var_t4_dn9 = assign35950_e41344_d_n9;
        locals.var_t4_dn10 = assign35950_e41344_d_n10;
        locals.var_t4_dn11 = assign35950_e41344_d_n11;
        locals.var_t4_dn14 = assign35950_e41344_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign35960_e41351, assign35960_e41351_d_n0, assign35960_e41351_d_n2, assign35960_e41351_d_n4, assign35960_e41351_d_n5, assign35960_e41351_d_n6, assign35960_e41351_d_n7, assign35960_e41351_d_n8, assign35960_e41351_d_n9, assign35960_e41351_d_n10, assign35960_e41351_d_n11, assign35960_e41351_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign35960_e41349: f64 = (-locals.var_qn_bac);
        (assign35960_e41349, (-locals.var_qn_bac_dn0), (-locals.var_qn_bac_dn2), (-locals.var_qn_bac_dn4), (-locals.var_qn_bac_dn5), (-locals.var_qn_bac_dn6), (-locals.var_qn_bac_dn7), (-locals.var_qn_bac_dn8), (-locals.var_qn_bac_dn9), (-locals.var_qn_bac_dn10), (-locals.var_qn_bac_dn11), (-locals.var_qn_bac_dn14),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn14,)
    }
};
        locals.var_qiu = assign35960_e41351;
        locals.var_qiu_dn0 = assign35960_e41351_d_n0;
        locals.var_qiu_dn2 = assign35960_e41351_d_n2;
        locals.var_qiu_dn4 = assign35960_e41351_d_n4;
        locals.var_qiu_dn5 = assign35960_e41351_d_n5;
        locals.var_qiu_dn6 = assign35960_e41351_d_n6;
        locals.var_qiu_dn7 = assign35960_e41351_d_n7;
        locals.var_qiu_dn8 = assign35960_e41351_d_n8;
        locals.var_qiu_dn9 = assign35960_e41351_d_n9;
        locals.var_qiu_dn10 = assign35960_e41351_d_n10;
        locals.var_qiu_dn11 = assign35960_e41351_d_n11;
        locals.var_qiu_dn14 = assign35960_e41351_d_n14;
        locals.var_qiu_rv = 0.0;

        let (assign35970_e41357, assign35970_e41357_d_n0, assign35970_e41357_d_n2, assign35970_e41357_d_n4, assign35970_e41357_d_n5, assign35970_e41357_d_n6, assign35970_e41357_d_n7, assign35970_e41357_d_n8, assign35970_e41357_d_n9, assign35970_e41357_d_n10, assign35970_e41357_d_n11, assign35970_e41357_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign35970_e41357;
        locals.var_t5_dn0 = assign35970_e41357_d_n0;
        locals.var_t5_dn2 = assign35970_e41357_d_n2;
        locals.var_t5_dn4 = assign35970_e41357_d_n4;
        locals.var_t5_dn5 = assign35970_e41357_d_n5;
        locals.var_t5_dn6 = assign35970_e41357_d_n6;
        locals.var_t5_dn7 = assign35970_e41357_d_n7;
        locals.var_t5_dn8 = assign35970_e41357_d_n8;
        locals.var_t5_dn9 = assign35970_e41357_d_n9;
        locals.var_t5_dn10 = assign35970_e41357_d_n10;
        locals.var_t5_dn11 = assign35970_e41357_d_n11;
        locals.var_t5_dn14 = assign35970_e41357_d_n14;
        locals.var_t5_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_118(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign35980_e41365, assign35980_e41365_d_n0, assign35980_e41365_d_n2, assign35980_e41365_d_n4, assign35980_e41365_d_n5, assign35980_e41365_d_n6, assign35980_e41365_d_n7, assign35980_e41365_d_n8, assign35980_e41365_d_n9, assign35980_e41365_d_n10, assign35980_e41365_d_n11, assign35980_e41365_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign35980_e41363: f64 = (locals.var_t5 / locals.var_t4);
        (assign35980_e41363, (((locals.var_t5_dn0 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn2 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn4 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn5 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn6 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn7 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn8 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn9 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn10 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn11 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn14 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign35980_e41365;
        locals.var_t3_dn0 = assign35980_e41365_d_n0;
        locals.var_t3_dn2 = assign35980_e41365_d_n2;
        locals.var_t3_dn4 = assign35980_e41365_d_n4;
        locals.var_t3_dn5 = assign35980_e41365_d_n5;
        locals.var_t3_dn6 = assign35980_e41365_d_n6;
        locals.var_t3_dn7 = assign35980_e41365_d_n7;
        locals.var_t3_dn8 = assign35980_e41365_d_n8;
        locals.var_t3_dn9 = assign35980_e41365_d_n9;
        locals.var_t3_dn10 = assign35980_e41365_d_n10;
        locals.var_t3_dn11 = assign35980_e41365_d_n11;
        locals.var_t3_dn14 = assign35980_e41365_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign35990_e41371, assign35990_e41371_d_n0, assign35990_e41371_d_n2, assign35990_e41371_d_n4, assign35990_e41371_d_n5, assign35990_e41371_d_n6, assign35990_e41371_d_n7, assign35990_e41371_d_n8, assign35990_e41371_d_n9, assign35990_e41371_d_n10, assign35990_e41371_d_n11, assign35990_e41371_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    } else {
        (locals.var_eeff, locals.var_eeff_dn0, locals.var_eeff_dn2, locals.var_eeff_dn4, locals.var_eeff_dn5, locals.var_eeff_dn6, locals.var_eeff_dn7, locals.var_eeff_dn8, locals.var_eeff_dn9, locals.var_eeff_dn10, locals.var_eeff_dn11, locals.var_eeff_dn14,)
    }
};
        locals.var_eeff = assign35990_e41371;
        locals.var_eeff_dn0 = assign35990_e41371_d_n0;
        locals.var_eeff_dn2 = assign35990_e41371_d_n2;
        locals.var_eeff_dn4 = assign35990_e41371_d_n4;
        locals.var_eeff_dn5 = assign35990_e41371_d_n5;
        locals.var_eeff_dn6 = assign35990_e41371_d_n6;
        locals.var_eeff_dn7 = assign35990_e41371_d_n7;
        locals.var_eeff_dn8 = assign35990_e41371_d_n8;
        locals.var_eeff_dn9 = assign35990_e41371_d_n9;
        locals.var_eeff_dn10 = assign35990_e41371_d_n10;
        locals.var_eeff_dn11 = assign35990_e41371_d_n11;
        locals.var_eeff_dn14 = assign35990_e41371_d_n14;
        locals.var_eeff_rv = 0.0;

        let (assign36000_e41386, assign36000_e41386_d_n0, assign36000_e41386_d_n2, assign36000_e41386_d_n4, assign36000_e41386_d_n5, assign36000_e41386_d_n6, assign36000_e41386_d_n7, assign36000_e41386_d_n8, assign36000_e41386_d_n9, assign36000_e41386_d_n10, assign36000_e41386_d_n11, assign36000_e41386_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let (assign36000_e41384, assign36000_e41384_d_n0, assign36000_e41384_d_n2, assign36000_e41384_d_n4, assign36000_e41384_d_n5, assign36000_e41384_d_n6, assign36000_e41384_d_n7, assign36000_e41384_d_n8, assign36000_e41384_d_n9, assign36000_e41384_d_n10, assign36000_e41384_d_n11, assign36000_e41384_d_n14,) = {
            if (locals.var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign36000_e41382: f64 = (p.p376 - 1.0);
                let assign36000_e41383: f64 = (locals.var_eeff).powf(assign36000_e41382);
                (assign36000_e41383, if 0.0 == 0.0 && ((assign36000_e41382) as f64).is_finite() && ((assign36000_e41382) as f64).fract() == 0.0 { if assign36000_e41382 == 0.0 { 0.0 } else { (assign36000_e41382 * ((locals.var_eeff).powf(assign36000_e41382 - 1.0) * locals.var_eeff_dn0)) } } else { (assign36000_e41383 * (assign36000_e41382 * (locals.var_eeff_dn0 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign36000_e41382) as f64).is_finite() && ((assign36000_e41382) as f64).fract() == 0.0 { if assign36000_e41382 == 0.0 { 0.0 } else { (assign36000_e41382 * ((locals.var_eeff).powf(assign36000_e41382 - 1.0) * locals.var_eeff_dn2)) } } else { (assign36000_e41383 * (assign36000_e41382 * (locals.var_eeff_dn2 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign36000_e41382) as f64).is_finite() && ((assign36000_e41382) as f64).fract() == 0.0 { if assign36000_e41382 == 0.0 { 0.0 } else { (assign36000_e41382 * ((locals.var_eeff).powf(assign36000_e41382 - 1.0) * locals.var_eeff_dn4)) } } else { (assign36000_e41383 * (assign36000_e41382 * (locals.var_eeff_dn4 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign36000_e41382) as f64).is_finite() && ((assign36000_e41382) as f64).fract() == 0.0 { if assign36000_e41382 == 0.0 { 0.0 } else { (assign36000_e41382 * ((locals.var_eeff).powf(assign36000_e41382 - 1.0) * locals.var_eeff_dn5)) } } else { (assign36000_e41383 * (assign36000_e41382 * (locals.var_eeff_dn5 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign36000_e41382) as f64).is_finite() && ((assign36000_e41382) as f64).fract() == 0.0 { if assign36000_e41382 == 0.0 { 0.0 } else { (assign36000_e41382 * ((locals.var_eeff).powf(assign36000_e41382 - 1.0) * locals.var_eeff_dn6)) } } else { (assign36000_e41383 * (assign36000_e41382 * (locals.var_eeff_dn6 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign36000_e41382) as f64).is_finite() && ((assign36000_e41382) as f64).fract() == 0.0 { if assign36000_e41382 == 0.0 { 0.0 } else { (assign36000_e41382 * ((locals.var_eeff).powf(assign36000_e41382 - 1.0) * locals.var_eeff_dn7)) } } else { (assign36000_e41383 * (assign36000_e41382 * (locals.var_eeff_dn7 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign36000_e41382) as f64).is_finite() && ((assign36000_e41382) as f64).fract() == 0.0 { if assign36000_e41382 == 0.0 { 0.0 } else { (assign36000_e41382 * ((locals.var_eeff).powf(assign36000_e41382 - 1.0) * locals.var_eeff_dn8)) } } else { (assign36000_e41383 * (assign36000_e41382 * (locals.var_eeff_dn8 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign36000_e41382) as f64).is_finite() && ((assign36000_e41382) as f64).fract() == 0.0 { if assign36000_e41382 == 0.0 { 0.0 } else { (assign36000_e41382 * ((locals.var_eeff).powf(assign36000_e41382 - 1.0) * locals.var_eeff_dn9)) } } else { (assign36000_e41383 * (assign36000_e41382 * (locals.var_eeff_dn9 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign36000_e41382) as f64).is_finite() && ((assign36000_e41382) as f64).fract() == 0.0 { if assign36000_e41382 == 0.0 { 0.0 } else { (assign36000_e41382 * ((locals.var_eeff).powf(assign36000_e41382 - 1.0) * locals.var_eeff_dn10)) } } else { (assign36000_e41383 * (assign36000_e41382 * (locals.var_eeff_dn10 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign36000_e41382) as f64).is_finite() && ((assign36000_e41382) as f64).fract() == 0.0 { if assign36000_e41382 == 0.0 { 0.0 } else { (assign36000_e41382 * ((locals.var_eeff).powf(assign36000_e41382 - 1.0) * locals.var_eeff_dn11)) } } else { (assign36000_e41383 * (assign36000_e41382 * (locals.var_eeff_dn11 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign36000_e41382) as f64).is_finite() && ((assign36000_e41382) as f64).fract() == 0.0 { if assign36000_e41382 == 0.0 { 0.0 } else { (assign36000_e41382 * ((locals.var_eeff).powf(assign36000_e41382 - 1.0) * locals.var_eeff_dn14)) } } else { (assign36000_e41383 * (assign36000_e41382 * (locals.var_eeff_dn14 / locals.var_eeff))) },)
            }
        };
        (assign36000_e41384, assign36000_e41384_d_n0, assign36000_e41384_d_n2, assign36000_e41384_d_n4, assign36000_e41384_d_n5, assign36000_e41384_d_n6, assign36000_e41384_d_n7, assign36000_e41384_d_n8, assign36000_e41384_d_n9, assign36000_e41384_d_n10, assign36000_e41384_d_n11, assign36000_e41384_d_n14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign36000_e41386;
        locals.var_t5_dn0 = assign36000_e41386_d_n0;
        locals.var_t5_dn2 = assign36000_e41386_d_n2;
        locals.var_t5_dn4 = assign36000_e41386_d_n4;
        locals.var_t5_dn5 = assign36000_e41386_d_n5;
        locals.var_t5_dn6 = assign36000_e41386_d_n6;
        locals.var_t5_dn7 = assign36000_e41386_d_n7;
        locals.var_t5_dn8 = assign36000_e41386_d_n8;
        locals.var_t5_dn9 = assign36000_e41386_d_n9;
        locals.var_t5_dn10 = assign36000_e41386_d_n10;
        locals.var_t5_dn11 = assign36000_e41386_d_n11;
        locals.var_t5_dn14 = assign36000_e41386_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign36010_e41394, assign36010_e41394_d_n0, assign36010_e41394_d_n2, assign36010_e41394_d_n4, assign36010_e41394_d_n5, assign36010_e41394_d_n6, assign36010_e41394_d_n7, assign36010_e41394_d_n8, assign36010_e41394_d_n9, assign36010_e41394_d_n10, assign36010_e41394_d_n11, assign36010_e41394_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign36010_e41392: f64 = (locals.var_t5 * locals.var_eeff);
        (assign36010_e41392, ((locals.var_t5_dn0 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn0)), ((locals.var_t5_dn2 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn2)), ((locals.var_t5_dn4 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn4)), ((locals.var_t5_dn5 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn5)), ((locals.var_t5_dn6 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn6)), ((locals.var_t5_dn7 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn7)), ((locals.var_t5_dn8 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn8)), ((locals.var_t5_dn9 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn9)), ((locals.var_t5_dn10 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn10)), ((locals.var_t5_dn11 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn11)), ((locals.var_t5_dn14 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn14)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign36010_e41394;
        locals.var_t8_dn0 = assign36010_e41394_d_n0;
        locals.var_t8_dn2 = assign36010_e41394_d_n2;
        locals.var_t8_dn4 = assign36010_e41394_d_n4;
        locals.var_t8_dn5 = assign36010_e41394_d_n5;
        locals.var_t8_dn6 = assign36010_e41394_d_n6;
        locals.var_t8_dn7 = assign36010_e41394_d_n7;
        locals.var_t8_dn8 = assign36010_e41394_d_n8;
        locals.var_t8_dn9 = assign36010_e41394_d_n9;
        locals.var_t8_dn10 = assign36010_e41394_d_n10;
        locals.var_t8_dn11 = assign36010_e41394_d_n11;
        locals.var_t8_dn14 = assign36010_e41394_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign36020_e41402, assign36020_e41402_d_n0, assign36020_e41402_d_n2, assign36020_e41402_d_n4, assign36020_e41402_d_n5, assign36020_e41402_d_n6, assign36020_e41402_d_n7, assign36020_e41402_d_n8, assign36020_e41402_d_n9, assign36020_e41402_d_n10, assign36020_e41402_d_n11, assign36020_e41402_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign36020_e41400: f64 = (1.6021918e-19 * 10000.0);
        (assign36020_e41400, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign36020_e41402;
        locals.var_t9_dn0 = assign36020_e41402_d_n0;
        locals.var_t9_dn2 = assign36020_e41402_d_n2;
        locals.var_t9_dn4 = assign36020_e41402_d_n4;
        locals.var_t9_dn5 = assign36020_e41402_d_n5;
        locals.var_t9_dn6 = assign36020_e41402_d_n6;
        locals.var_t9_dn7 = assign36020_e41402_d_n7;
        locals.var_t9_dn8 = assign36020_e41402_d_n8;
        locals.var_t9_dn9 = assign36020_e41402_d_n9;
        locals.var_t9_dn10 = assign36020_e41402_d_n10;
        locals.var_t9_dn11 = assign36020_e41402_d_n11;
        locals.var_t9_dn14 = assign36020_e41402_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign36030_e41410, assign36030_e41410_d_n0, assign36030_e41410_d_n2, assign36030_e41410_d_n4, assign36030_e41410_d_n5, assign36030_e41410_d_n6, assign36030_e41410_d_n7, assign36030_e41410_d_n8, assign36030_e41410_d_n9, assign36030_e41410_d_n10, assign36030_e41410_d_n11, assign36030_e41410_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign36030_e41408: f64 = (locals.var_qiu / locals.var_t9);
        (assign36030_e41408, (((locals.var_qiu_dn0 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn0)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn2 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn2)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn4 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn4)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn5 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn5)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn6 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn6)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn7 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn7)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn8 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn8)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn9 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn9)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn10 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn10)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn11 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn11)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn14 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn14)) / (locals.var_t9 * locals.var_t9)),)
    } else {
        (locals.var_rns, locals.var_rns_dn0, locals.var_rns_dn2, locals.var_rns_dn4, locals.var_rns_dn5, locals.var_rns_dn6, locals.var_rns_dn7, locals.var_rns_dn8, locals.var_rns_dn9, locals.var_rns_dn10, locals.var_rns_dn11, locals.var_rns_dn14,)
    }
};
        locals.var_rns = assign36030_e41410;
        locals.var_rns_dn0 = assign36030_e41410_d_n0;
        locals.var_rns_dn2 = assign36030_e41410_d_n2;
        locals.var_rns_dn4 = assign36030_e41410_d_n4;
        locals.var_rns_dn5 = assign36030_e41410_d_n5;
        locals.var_rns_dn6 = assign36030_e41410_d_n6;
        locals.var_rns_dn7 = assign36030_e41410_d_n7;
        locals.var_rns_dn8 = assign36030_e41410_d_n8;
        locals.var_rns_dn9 = assign36030_e41410_d_n9;
        locals.var_rns_dn10 = assign36030_e41410_d_n10;
        locals.var_rns_dn11 = assign36030_e41410_d_n11;
        locals.var_rns_dn14 = assign36030_e41410_d_n14;
        locals.var_rns_rv = 0.0;

        let (assign36040_e41430, assign36040_e41430_d_n0, assign36040_e41430_d_n2, assign36040_e41430_d_n4, assign36040_e41430_d_n5, assign36040_e41430_d_n6, assign36040_e41430_d_n7, assign36040_e41430_d_n8, assign36040_e41430_d_n9, assign36040_e41430_d_n10, assign36040_e41430_d_n11, assign36040_e41430_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign36040_e41418: f64 = (locals.var_uc_depmueback1 * locals.var_rns);
        let assign36040_e41420: f64 = (assign36040_e41418 / 100000000000.0);
        let assign36040_e41421: f64 = (locals.var_uc_depmueback0 + assign36040_e41420);
        let assign36040_e41423: f64 = (assign36040_e41421 + 1e-25);
        let assign36040_e41424: f64 = (1.0 / assign36040_e41423);
        let assign36040_e41427: f64 = (locals.var_depmphn0 * locals.var_t8);
        let assign36040_e41428: f64 = (assign36040_e41424 + assign36040_e41427);
        (assign36040_e41428, ((-((locals.var_uc_depmueback0_dn0 + (((locals.var_uc_depmueback1_dn0 * locals.var_rns) + (locals.var_uc_depmueback1 * locals.var_rns_dn0)) / 100000000000.0)) / (assign36040_e41423 * assign36040_e41423))) + ((locals.var_depmphn0_dn0 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn0))), ((-((locals.var_uc_depmueback0_dn2 + (((locals.var_uc_depmueback1_dn2 * locals.var_rns) + (locals.var_uc_depmueback1 * locals.var_rns_dn2)) / 100000000000.0)) / (assign36040_e41423 * assign36040_e41423))) + ((locals.var_depmphn0_dn2 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn2))), ((-((locals.var_uc_depmueback0_dn4 + (((locals.var_uc_depmueback1_dn4 * locals.var_rns) + (locals.var_uc_depmueback1 * locals.var_rns_dn4)) / 100000000000.0)) / (assign36040_e41423 * assign36040_e41423))) + ((locals.var_depmphn0_dn4 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn4))), ((-((locals.var_uc_depmueback0_dn5 + (((locals.var_uc_depmueback1_dn5 * locals.var_rns) + (locals.var_uc_depmueback1 * locals.var_rns_dn5)) / 100000000000.0)) / (assign36040_e41423 * assign36040_e41423))) + ((locals.var_depmphn0_dn5 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn5))), ((-((locals.var_uc_depmueback0_dn6 + (((locals.var_uc_depmueback1_dn6 * locals.var_rns) + (locals.var_uc_depmueback1 * locals.var_rns_dn6)) / 100000000000.0)) / (assign36040_e41423 * assign36040_e41423))) + ((locals.var_depmphn0_dn6 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn6))), ((-((locals.var_uc_depmueback0_dn7 + (((locals.var_uc_depmueback1_dn7 * locals.var_rns) + (locals.var_uc_depmueback1 * locals.var_rns_dn7)) / 100000000000.0)) / (assign36040_e41423 * assign36040_e41423))) + ((locals.var_depmphn0_dn7 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn7))), ((-((locals.var_uc_depmueback0_dn8 + (((locals.var_uc_depmueback1_dn8 * locals.var_rns) + (locals.var_uc_depmueback1 * locals.var_rns_dn8)) / 100000000000.0)) / (assign36040_e41423 * assign36040_e41423))) + ((locals.var_depmphn0_dn8 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn8))), ((-((locals.var_uc_depmueback0_dn9 + (((locals.var_uc_depmueback1_dn9 * locals.var_rns) + (locals.var_uc_depmueback1 * locals.var_rns_dn9)) / 100000000000.0)) / (assign36040_e41423 * assign36040_e41423))) + ((locals.var_depmphn0_dn9 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn9))), ((-((locals.var_uc_depmueback0_dn10 + (((locals.var_uc_depmueback1_dn10 * locals.var_rns) + (locals.var_uc_depmueback1 * locals.var_rns_dn10)) / 100000000000.0)) / (assign36040_e41423 * assign36040_e41423))) + ((locals.var_depmphn0_dn10 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn10))), ((-((locals.var_uc_depmueback0_dn11 + (((locals.var_uc_depmueback1_dn11 * locals.var_rns) + (locals.var_uc_depmueback1 * locals.var_rns_dn11)) / 100000000000.0)) / (assign36040_e41423 * assign36040_e41423))) + ((locals.var_depmphn0_dn11 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn11))), ((-((locals.var_uc_depmueback0_dn14 + (((locals.var_uc_depmueback1_dn14 * locals.var_rns) + (locals.var_uc_depmueback1 * locals.var_rns_dn14)) / 100000000000.0)) / (assign36040_e41423 * assign36040_e41423))) + ((locals.var_depmphn0_dn14 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign36040_e41430;
        locals.var_t1_dn0 = assign36040_e41430_d_n0;
        locals.var_t1_dn2 = assign36040_e41430_d_n2;
        locals.var_t1_dn4 = assign36040_e41430_d_n4;
        locals.var_t1_dn5 = assign36040_e41430_d_n5;
        locals.var_t1_dn6 = assign36040_e41430_d_n6;
        locals.var_t1_dn7 = assign36040_e41430_d_n7;
        locals.var_t1_dn8 = assign36040_e41430_d_n8;
        locals.var_t1_dn9 = assign36040_e41430_d_n9;
        locals.var_t1_dn10 = assign36040_e41430_d_n10;
        locals.var_t1_dn11 = assign36040_e41430_d_n11;
        locals.var_t1_dn14 = assign36040_e41430_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign36050_e41438, assign36050_e41438_d_n0, assign36050_e41438_d_n2, assign36050_e41438_d_n4, assign36050_e41438_d_n5, assign36050_e41438_d_n6, assign36050_e41438_d_n7, assign36050_e41438_d_n8, assign36050_e41438_d_n9, assign36050_e41438_d_n10, assign36050_e41438_d_n11, assign36050_e41438_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign36050_e41436: f64 = (1.0 / locals.var_t1);
        (assign36050_e41436, (-(locals.var_t1_dn0 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn2 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn11 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn14 / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn14,)
    }
};
        locals.var_muun = assign36050_e41438;
        locals.var_muun_dn0 = assign36050_e41438_d_n0;
        locals.var_muun_dn2 = assign36050_e41438_d_n2;
        locals.var_muun_dn4 = assign36050_e41438_d_n4;
        locals.var_muun_dn5 = assign36050_e41438_d_n5;
        locals.var_muun_dn6 = assign36050_e41438_d_n6;
        locals.var_muun_dn7 = assign36050_e41438_d_n7;
        locals.var_muun_dn8 = assign36050_e41438_d_n8;
        locals.var_muun_dn9 = assign36050_e41438_d_n9;
        locals.var_muun_dn10 = assign36050_e41438_d_n10;
        locals.var_muun_dn11 = assign36050_e41438_d_n11;
        locals.var_muun_dn14 = assign36050_e41438_d_n14;
        locals.var_muun_rv = 0.0;

        let (assign36060_e41446, assign36060_e41446_d_n0, assign36060_e41446_d_n2, assign36060_e41446_d_n4, assign36060_e41446_d_n5, assign36060_e41446_d_n6, assign36060_e41446_d_n7, assign36060_e41446_d_n8, assign36060_e41446_d_n9, assign36060_e41446_d_n10, assign36060_e41446_d_n11, assign36060_e41446_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign36060_e41444: f64 = (locals.var_muun / 10000.0);
        (assign36060_e41444, (locals.var_muun_dn0 / 10000.0), (locals.var_muun_dn2 / 10000.0), (locals.var_muun_dn4 / 10000.0), (locals.var_muun_dn5 / 10000.0), (locals.var_muun_dn6 / 10000.0), (locals.var_muun_dn7 / 10000.0), (locals.var_muun_dn8 / 10000.0), (locals.var_muun_dn9 / 10000.0), (locals.var_muun_dn10 / 10000.0), (locals.var_muun_dn11 / 10000.0), (locals.var_muun_dn14 / 10000.0),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn14,)
    }
};
        locals.var_muun = assign36060_e41446;
        locals.var_muun_dn0 = assign36060_e41446_d_n0;
        locals.var_muun_dn2 = assign36060_e41446_d_n2;
        locals.var_muun_dn4 = assign36060_e41446_d_n4;
        locals.var_muun_dn5 = assign36060_e41446_d_n5;
        locals.var_muun_dn6 = assign36060_e41446_d_n6;
        locals.var_muun_dn7 = assign36060_e41446_d_n7;
        locals.var_muun_dn8 = assign36060_e41446_d_n8;
        locals.var_muun_dn9 = assign36060_e41446_d_n9;
        locals.var_muun_dn10 = assign36060_e41446_d_n10;
        locals.var_muun_dn11 = assign36060_e41446_d_n11;
        locals.var_muun_dn14 = assign36060_e41446_d_n14;
        locals.var_muun_rv = 0.0;

        let (assign36070_e41454, assign36070_e41454_d_n0, assign36070_e41454_d_n2, assign36070_e41454_d_n4, assign36070_e41454_d_n5, assign36070_e41454_d_n6, assign36070_e41454_d_n7, assign36070_e41454_d_n8, assign36070_e41454_d_n9, assign36070_e41454_d_n10, assign36070_e41454_d_n11, assign36070_e41454_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign36070_e41452: f64 = (locals.var_vdseff0 / locals.var_lch);
        (assign36070_e41452, (((locals.var_vdseff0_dn0 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn2 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn4 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn5 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn6 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn7 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn8 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn9 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn10 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn11 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn14 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn14)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_edri__blk557, locals.var_edri__blk557_dn0, locals.var_edri__blk557_dn2, locals.var_edri__blk557_dn4, locals.var_edri__blk557_dn5, locals.var_edri__blk557_dn6, locals.var_edri__blk557_dn7, locals.var_edri__blk557_dn8, locals.var_edri__blk557_dn9, locals.var_edri__blk557_dn10, locals.var_edri__blk557_dn11, locals.var_edri__blk557_dn14,)
    }
};
        locals.var_edri__blk557 = assign36070_e41454;
        locals.var_edri__blk557_dn0 = assign36070_e41454_d_n0;
        locals.var_edri__blk557_dn2 = assign36070_e41454_d_n2;
        locals.var_edri__blk557_dn4 = assign36070_e41454_d_n4;
        locals.var_edri__blk557_dn5 = assign36070_e41454_d_n5;
        locals.var_edri__blk557_dn6 = assign36070_e41454_d_n6;
        locals.var_edri__blk557_dn7 = assign36070_e41454_d_n7;
        locals.var_edri__blk557_dn8 = assign36070_e41454_d_n8;
        locals.var_edri__blk557_dn9 = assign36070_e41454_d_n9;
        locals.var_edri__blk557_dn10 = assign36070_e41454_d_n10;
        locals.var_edri__blk557_dn11 = assign36070_e41454_d_n11;
        locals.var_edri__blk557_dn14 = assign36070_e41454_d_n14;
        locals.var_edri__blk557_rv = 0.0;

        let (assign36080_e41464, assign36080_e41464_d_n0, assign36080_e41464_d_n2, assign36080_e41464_d_n4, assign36080_e41464_d_n5, assign36080_e41464_d_n6, assign36080_e41464_d_n7, assign36080_e41464_d_n8, assign36080_e41464_d_n9, assign36080_e41464_d_n10, assign36080_e41464_d_n11, assign36080_e41464_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign36080_e41460: f64 = (locals.var_muun * locals.var_edri__blk557);
        let assign36080_e41462: f64 = (assign36080_e41460 / locals.var_uc_depvmax);
        (assign36080_e41462, (((((locals.var_muun_dn0 * locals.var_edri__blk557) + (locals.var_muun * locals.var_edri__blk557_dn0)) * locals.var_uc_depvmax) - (assign36080_e41460 * locals.var_uc_depvmax_dn0)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn2 * locals.var_edri__blk557) + (locals.var_muun * locals.var_edri__blk557_dn2)) * locals.var_uc_depvmax) - (assign36080_e41460 * locals.var_uc_depvmax_dn2)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn4 * locals.var_edri__blk557) + (locals.var_muun * locals.var_edri__blk557_dn4)) * locals.var_uc_depvmax) - (assign36080_e41460 * locals.var_uc_depvmax_dn4)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn5 * locals.var_edri__blk557) + (locals.var_muun * locals.var_edri__blk557_dn5)) * locals.var_uc_depvmax) - (assign36080_e41460 * locals.var_uc_depvmax_dn5)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn6 * locals.var_edri__blk557) + (locals.var_muun * locals.var_edri__blk557_dn6)) * locals.var_uc_depvmax) - (assign36080_e41460 * locals.var_uc_depvmax_dn6)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn7 * locals.var_edri__blk557) + (locals.var_muun * locals.var_edri__blk557_dn7)) * locals.var_uc_depvmax) - (assign36080_e41460 * locals.var_uc_depvmax_dn7)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn8 * locals.var_edri__blk557) + (locals.var_muun * locals.var_edri__blk557_dn8)) * locals.var_uc_depvmax) - (assign36080_e41460 * locals.var_uc_depvmax_dn8)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn9 * locals.var_edri__blk557) + (locals.var_muun * locals.var_edri__blk557_dn9)) * locals.var_uc_depvmax) - (assign36080_e41460 * locals.var_uc_depvmax_dn9)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn10 * locals.var_edri__blk557) + (locals.var_muun * locals.var_edri__blk557_dn10)) * locals.var_uc_depvmax) - (assign36080_e41460 * locals.var_uc_depvmax_dn10)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn11 * locals.var_edri__blk557) + (locals.var_muun * locals.var_edri__blk557_dn11)) * locals.var_uc_depvmax) - (assign36080_e41460 * locals.var_uc_depvmax_dn11)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn14 * locals.var_edri__blk557) + (locals.var_muun * locals.var_edri__blk557_dn14)) * locals.var_uc_depvmax) - (assign36080_e41460 * locals.var_uc_depvmax_dn14)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign36080_e41464;
        locals.var_t1_dn0 = assign36080_e41464_d_n0;
        locals.var_t1_dn2 = assign36080_e41464_d_n2;
        locals.var_t1_dn4 = assign36080_e41464_d_n4;
        locals.var_t1_dn5 = assign36080_e41464_d_n5;
        locals.var_t1_dn6 = assign36080_e41464_d_n6;
        locals.var_t1_dn7 = assign36080_e41464_d_n7;
        locals.var_t1_dn8 = assign36080_e41464_d_n8;
        locals.var_t1_dn9 = assign36080_e41464_d_n9;
        locals.var_t1_dn10 = assign36080_e41464_d_n10;
        locals.var_t1_dn11 = assign36080_e41464_d_n11;
        locals.var_t1_dn14 = assign36080_e41464_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign36090_e41477, assign36090_e41477_d_n0, assign36090_e41477_d_n2, assign36090_e41477_d_n4, assign36090_e41477_d_n5, assign36090_e41477_d_n6, assign36090_e41477_d_n7, assign36090_e41477_d_n8, assign36090_e41477_d_n9, assign36090_e41477_d_n10, assign36090_e41477_d_n11, assign36090_e41477_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let (assign36090_e41475, assign36090_e41475_d_n0, assign36090_e41475_d_n2, assign36090_e41475_d_n4, assign36090_e41475_d_n5, assign36090_e41475_d_n6, assign36090_e41475_d_n7, assign36090_e41475_d_n8, assign36090_e41475_d_n9, assign36090_e41475_d_n10, assign36090_e41475_d_n11, assign36090_e41475_d_n14,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign36090_e41474: f64 = (locals.var_t1).powf(p.p378);
                (assign36090_e41474, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn0)) } } else { (assign36090_e41474 * (p.p378 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn2)) } } else { (assign36090_e41474 * (p.p378 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn4)) } } else { (assign36090_e41474 * (p.p378 * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn5)) } } else { (assign36090_e41474 * (p.p378 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn6)) } } else { (assign36090_e41474 * (p.p378 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn7)) } } else { (assign36090_e41474 * (p.p378 * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn8)) } } else { (assign36090_e41474 * (p.p378 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn9)) } } else { (assign36090_e41474 * (p.p378 * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn10)) } } else { (assign36090_e41474 * (p.p378 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn11)) } } else { (assign36090_e41474 * (p.p378 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn14)) } } else { (assign36090_e41474 * (p.p378 * (locals.var_t1_dn14 / locals.var_t1))) },)
            }
        };
        (assign36090_e41475, assign36090_e41475_d_n0, assign36090_e41475_d_n2, assign36090_e41475_d_n4, assign36090_e41475_d_n5, assign36090_e41475_d_n6, assign36090_e41475_d_n7, assign36090_e41475_d_n8, assign36090_e41475_d_n9, assign36090_e41475_d_n10, assign36090_e41475_d_n11, assign36090_e41475_d_n14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign36090_e41477;
        locals.var_t2_dn0 = assign36090_e41477_d_n0;
        locals.var_t2_dn2 = assign36090_e41477_d_n2;
        locals.var_t2_dn4 = assign36090_e41477_d_n4;
        locals.var_t2_dn5 = assign36090_e41477_d_n5;
        locals.var_t2_dn6 = assign36090_e41477_d_n6;
        locals.var_t2_dn7 = assign36090_e41477_d_n7;
        locals.var_t2_dn8 = assign36090_e41477_d_n8;
        locals.var_t2_dn9 = assign36090_e41477_d_n9;
        locals.var_t2_dn10 = assign36090_e41477_d_n10;
        locals.var_t2_dn11 = assign36090_e41477_d_n11;
        locals.var_t2_dn14 = assign36090_e41477_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign36100_e41485, assign36100_e41485_d_n0, assign36100_e41485_d_n2, assign36100_e41485_d_n4, assign36100_e41485_d_n5, assign36100_e41485_d_n6, assign36100_e41485_d_n7, assign36100_e41485_d_n8, assign36100_e41485_d_n9, assign36100_e41485_d_n10, assign36100_e41485_d_n11, assign36100_e41485_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign36100_e41483: f64 = (1.0 + locals.var_t2);
        (assign36100_e41483, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign36100_e41485;
        locals.var_t3_dn0 = assign36100_e41485_d_n0;
        locals.var_t3_dn2 = assign36100_e41485_d_n2;
        locals.var_t3_dn4 = assign36100_e41485_d_n4;
        locals.var_t3_dn5 = assign36100_e41485_d_n5;
        locals.var_t3_dn6 = assign36100_e41485_d_n6;
        locals.var_t3_dn7 = assign36100_e41485_d_n7;
        locals.var_t3_dn8 = assign36100_e41485_d_n8;
        locals.var_t3_dn9 = assign36100_e41485_d_n9;
        locals.var_t3_dn10 = assign36100_e41485_d_n10;
        locals.var_t3_dn11 = assign36100_e41485_d_n11;
        locals.var_t3_dn14 = assign36100_e41485_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign36110_e41500, assign36110_e41500_d_n0, assign36110_e41500_d_n2, assign36110_e41500_d_n4, assign36110_e41500_d_n5, assign36110_e41500_d_n6, assign36110_e41500_d_n7, assign36110_e41500_d_n8, assign36110_e41500_d_n9, assign36110_e41500_d_n10, assign36110_e41500_d_n11, assign36110_e41500_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let (assign36110_e41498, assign36110_e41498_d_n0, assign36110_e41498_d_n2, assign36110_e41498_d_n4, assign36110_e41498_d_n5, assign36110_e41498_d_n6, assign36110_e41498_d_n7, assign36110_e41498_d_n8, assign36110_e41498_d_n9, assign36110_e41498_d_n10, assign36110_e41498_d_n11, assign36110_e41498_d_n14,) = {
            if (locals.var_t3 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign36110_e41496: f64 = (1.0 / p.p378);
                let assign36110_e41497: f64 = (locals.var_t3).powf(assign36110_e41496);
                (assign36110_e41497, if 0.0 == 0.0 && ((assign36110_e41496) as f64).is_finite() && ((assign36110_e41496) as f64).fract() == 0.0 { if assign36110_e41496 == 0.0 { 0.0 } else { (assign36110_e41496 * ((locals.var_t3).powf(assign36110_e41496 - 1.0) * locals.var_t3_dn0)) } } else { (assign36110_e41497 * (assign36110_e41496 * (locals.var_t3_dn0 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign36110_e41496) as f64).is_finite() && ((assign36110_e41496) as f64).fract() == 0.0 { if assign36110_e41496 == 0.0 { 0.0 } else { (assign36110_e41496 * ((locals.var_t3).powf(assign36110_e41496 - 1.0) * locals.var_t3_dn2)) } } else { (assign36110_e41497 * (assign36110_e41496 * (locals.var_t3_dn2 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign36110_e41496) as f64).is_finite() && ((assign36110_e41496) as f64).fract() == 0.0 { if assign36110_e41496 == 0.0 { 0.0 } else { (assign36110_e41496 * ((locals.var_t3).powf(assign36110_e41496 - 1.0) * locals.var_t3_dn4)) } } else { (assign36110_e41497 * (assign36110_e41496 * (locals.var_t3_dn4 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign36110_e41496) as f64).is_finite() && ((assign36110_e41496) as f64).fract() == 0.0 { if assign36110_e41496 == 0.0 { 0.0 } else { (assign36110_e41496 * ((locals.var_t3).powf(assign36110_e41496 - 1.0) * locals.var_t3_dn5)) } } else { (assign36110_e41497 * (assign36110_e41496 * (locals.var_t3_dn5 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign36110_e41496) as f64).is_finite() && ((assign36110_e41496) as f64).fract() == 0.0 { if assign36110_e41496 == 0.0 { 0.0 } else { (assign36110_e41496 * ((locals.var_t3).powf(assign36110_e41496 - 1.0) * locals.var_t3_dn6)) } } else { (assign36110_e41497 * (assign36110_e41496 * (locals.var_t3_dn6 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign36110_e41496) as f64).is_finite() && ((assign36110_e41496) as f64).fract() == 0.0 { if assign36110_e41496 == 0.0 { 0.0 } else { (assign36110_e41496 * ((locals.var_t3).powf(assign36110_e41496 - 1.0) * locals.var_t3_dn7)) } } else { (assign36110_e41497 * (assign36110_e41496 * (locals.var_t3_dn7 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign36110_e41496) as f64).is_finite() && ((assign36110_e41496) as f64).fract() == 0.0 { if assign36110_e41496 == 0.0 { 0.0 } else { (assign36110_e41496 * ((locals.var_t3).powf(assign36110_e41496 - 1.0) * locals.var_t3_dn8)) } } else { (assign36110_e41497 * (assign36110_e41496 * (locals.var_t3_dn8 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign36110_e41496) as f64).is_finite() && ((assign36110_e41496) as f64).fract() == 0.0 { if assign36110_e41496 == 0.0 { 0.0 } else { (assign36110_e41496 * ((locals.var_t3).powf(assign36110_e41496 - 1.0) * locals.var_t3_dn9)) } } else { (assign36110_e41497 * (assign36110_e41496 * (locals.var_t3_dn9 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign36110_e41496) as f64).is_finite() && ((assign36110_e41496) as f64).fract() == 0.0 { if assign36110_e41496 == 0.0 { 0.0 } else { (assign36110_e41496 * ((locals.var_t3).powf(assign36110_e41496 - 1.0) * locals.var_t3_dn10)) } } else { (assign36110_e41497 * (assign36110_e41496 * (locals.var_t3_dn10 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign36110_e41496) as f64).is_finite() && ((assign36110_e41496) as f64).fract() == 0.0 { if assign36110_e41496 == 0.0 { 0.0 } else { (assign36110_e41496 * ((locals.var_t3).powf(assign36110_e41496 - 1.0) * locals.var_t3_dn11)) } } else { (assign36110_e41497 * (assign36110_e41496 * (locals.var_t3_dn11 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign36110_e41496) as f64).is_finite() && ((assign36110_e41496) as f64).fract() == 0.0 { if assign36110_e41496 == 0.0 { 0.0 } else { (assign36110_e41496 * ((locals.var_t3).powf(assign36110_e41496 - 1.0) * locals.var_t3_dn14)) } } else { (assign36110_e41497 * (assign36110_e41496 * (locals.var_t3_dn14 / locals.var_t3))) },)
            }
        };
        (assign36110_e41498, assign36110_e41498_d_n0, assign36110_e41498_d_n2, assign36110_e41498_d_n4, assign36110_e41498_d_n5, assign36110_e41498_d_n6, assign36110_e41498_d_n7, assign36110_e41498_d_n8, assign36110_e41498_d_n9, assign36110_e41498_d_n10, assign36110_e41498_d_n11, assign36110_e41498_d_n14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign36110_e41500;
        locals.var_t4_dn0 = assign36110_e41500_d_n0;
        locals.var_t4_dn2 = assign36110_e41500_d_n2;
        locals.var_t4_dn4 = assign36110_e41500_d_n4;
        locals.var_t4_dn5 = assign36110_e41500_d_n5;
        locals.var_t4_dn6 = assign36110_e41500_d_n6;
        locals.var_t4_dn7 = assign36110_e41500_d_n7;
        locals.var_t4_dn8 = assign36110_e41500_d_n8;
        locals.var_t4_dn9 = assign36110_e41500_d_n9;
        locals.var_t4_dn10 = assign36110_e41500_d_n10;
        locals.var_t4_dn11 = assign36110_e41500_d_n11;
        locals.var_t4_dn14 = assign36110_e41500_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign36120_e41508, assign36120_e41508_d_n0, assign36120_e41508_d_n2, assign36120_e41508_d_n4, assign36120_e41508_d_n5, assign36120_e41508_d_n6, assign36120_e41508_d_n7, assign36120_e41508_d_n8, assign36120_e41508_d_n9, assign36120_e41508_d_n10, assign36120_e41508_d_n11, assign36120_e41508_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign36120_e41506: f64 = (locals.var_muun / locals.var_t4);
        (assign36120_e41506, (((locals.var_muun_dn0 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn2 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn4 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn5 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn6 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn7 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn8 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn9 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn10 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn11 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn14 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_mu_bac, locals.var_mu_bac_dn0, locals.var_mu_bac_dn2, locals.var_mu_bac_dn4, locals.var_mu_bac_dn5, locals.var_mu_bac_dn6, locals.var_mu_bac_dn7, locals.var_mu_bac_dn8, locals.var_mu_bac_dn9, locals.var_mu_bac_dn10, locals.var_mu_bac_dn11, locals.var_mu_bac_dn14,)
    }
};
        locals.var_mu_bac = assign36120_e41508;
        locals.var_mu_bac_dn0 = assign36120_e41508_d_n0;
        locals.var_mu_bac_dn2 = assign36120_e41508_d_n2;
        locals.var_mu_bac_dn4 = assign36120_e41508_d_n4;
        locals.var_mu_bac_dn5 = assign36120_e41508_d_n5;
        locals.var_mu_bac_dn6 = assign36120_e41508_d_n6;
        locals.var_mu_bac_dn7 = assign36120_e41508_d_n7;
        locals.var_mu_bac_dn8 = assign36120_e41508_d_n8;
        locals.var_mu_bac_dn9 = assign36120_e41508_d_n9;
        locals.var_mu_bac_dn10 = assign36120_e41508_d_n10;
        locals.var_mu_bac_dn11 = assign36120_e41508_d_n11;
        locals.var_mu_bac_dn14 = assign36120_e41508_d_n14;
        locals.var_mu_bac_rv = 0.0;

        let (assign36130_e41521, assign36130_e41521_d_n0, assign36130_e41521_d_n2, assign36130_e41521_d_n4, assign36130_e41521_d_n5, assign36130_e41521_d_n6, assign36130_e41521_d_n7, assign36130_e41521_d_n8, assign36130_e41521_d_n9, assign36130_e41521_d_n10, assign36130_e41521_d_n11, assign36130_e41521_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign36130_e41514: f64 = (-locals.var_qn_bac);
        let assign36130_e41515: f64 = (locals.var_weff_nf * assign36130_e41514);
        let assign36130_e41517: f64 = (assign36130_e41515 * locals.var_mu_bac);
        let assign36130_e41519: f64 = (assign36130_e41517 * locals.var_edri__blk557);
        (assign36130_e41519, (((((locals.var_weff_nf * (-locals.var_qn_bac_dn0)) * locals.var_mu_bac) + (assign36130_e41515 * locals.var_mu_bac_dn0)) * locals.var_edri__blk557) + (assign36130_e41517 * locals.var_edri__blk557_dn0)), (((((locals.var_weff_nf * (-locals.var_qn_bac_dn2)) * locals.var_mu_bac) + (assign36130_e41515 * locals.var_mu_bac_dn2)) * locals.var_edri__blk557) + (assign36130_e41517 * locals.var_edri__blk557_dn2)), (((((locals.var_weff_nf * (-locals.var_qn_bac_dn4)) * locals.var_mu_bac) + (assign36130_e41515 * locals.var_mu_bac_dn4)) * locals.var_edri__blk557) + (assign36130_e41517 * locals.var_edri__blk557_dn4)), (((((locals.var_weff_nf * (-locals.var_qn_bac_dn5)) * locals.var_mu_bac) + (assign36130_e41515 * locals.var_mu_bac_dn5)) * locals.var_edri__blk557) + (assign36130_e41517 * locals.var_edri__blk557_dn5)), (((((locals.var_weff_nf * (-locals.var_qn_bac_dn6)) * locals.var_mu_bac) + (assign36130_e41515 * locals.var_mu_bac_dn6)) * locals.var_edri__blk557) + (assign36130_e41517 * locals.var_edri__blk557_dn6)), (((((locals.var_weff_nf * (-locals.var_qn_bac_dn7)) * locals.var_mu_bac) + (assign36130_e41515 * locals.var_mu_bac_dn7)) * locals.var_edri__blk557) + (assign36130_e41517 * locals.var_edri__blk557_dn7)), (((((locals.var_weff_nf * (-locals.var_qn_bac_dn8)) * locals.var_mu_bac) + (assign36130_e41515 * locals.var_mu_bac_dn8)) * locals.var_edri__blk557) + (assign36130_e41517 * locals.var_edri__blk557_dn8)), (((((locals.var_weff_nf * (-locals.var_qn_bac_dn9)) * locals.var_mu_bac) + (assign36130_e41515 * locals.var_mu_bac_dn9)) * locals.var_edri__blk557) + (assign36130_e41517 * locals.var_edri__blk557_dn9)), (((((locals.var_weff_nf * (-locals.var_qn_bac_dn10)) * locals.var_mu_bac) + (assign36130_e41515 * locals.var_mu_bac_dn10)) * locals.var_edri__blk557) + (assign36130_e41517 * locals.var_edri__blk557_dn10)), (((((locals.var_weff_nf * (-locals.var_qn_bac_dn11)) * locals.var_mu_bac) + (assign36130_e41515 * locals.var_mu_bac_dn11)) * locals.var_edri__blk557) + (assign36130_e41517 * locals.var_edri__blk557_dn11)), (((((locals.var_weff_nf * (-locals.var_qn_bac_dn14)) * locals.var_mu_bac) + (assign36130_e41515 * locals.var_mu_bac_dn14)) * locals.var_edri__blk557) + (assign36130_e41517 * locals.var_edri__blk557_dn14)),)
    } else {
        (locals.var_ids_bac, locals.var_ids_bac_dn0, locals.var_ids_bac_dn2, locals.var_ids_bac_dn4, locals.var_ids_bac_dn5, locals.var_ids_bac_dn6, locals.var_ids_bac_dn7, locals.var_ids_bac_dn8, locals.var_ids_bac_dn9, locals.var_ids_bac_dn10, locals.var_ids_bac_dn11, locals.var_ids_bac_dn14,)
    }
};
        locals.var_ids_bac = assign36130_e41521;
        locals.var_ids_bac_dn0 = assign36130_e41521_d_n0;
        locals.var_ids_bac_dn2 = assign36130_e41521_d_n2;
        locals.var_ids_bac_dn4 = assign36130_e41521_d_n4;
        locals.var_ids_bac_dn5 = assign36130_e41521_d_n5;
        locals.var_ids_bac_dn6 = assign36130_e41521_d_n6;
        locals.var_ids_bac_dn7 = assign36130_e41521_d_n7;
        locals.var_ids_bac_dn8 = assign36130_e41521_d_n8;
        locals.var_ids_bac_dn9 = assign36130_e41521_d_n9;
        locals.var_ids_bac_dn10 = assign36130_e41521_d_n10;
        locals.var_ids_bac_dn11 = assign36130_e41521_d_n11;
        locals.var_ids_bac_dn14 = assign36130_e41521_d_n14;
        locals.var_ids_bac_rv = 0.0;

        let (assign36140_e41531, assign36140_e41531_d_n0, assign36140_e41531_d_n2, assign36140_e41531_d_n4, assign36140_e41531_d_n5, assign36140_e41531_d_n6, assign36140_e41531_d_n7, assign36140_e41531_d_n8, assign36140_e41531_d_n9, assign36140_e41531_d_n10, assign36140_e41531_d_n11, assign36140_e41531_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign36140_e41527: f64 = (locals.var_weff_nf * locals.var_beta_inv);
        let assign36140_e41529: f64 = (assign36140_e41527 / locals.var_lch);
        (assign36140_e41529, ((((locals.var_weff_nf * locals.var_beta_inv_dn0) * locals.var_lch) - (assign36140_e41527 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn2) * locals.var_lch) - (assign36140_e41527 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn4) * locals.var_lch) - (assign36140_e41527 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn5) * locals.var_lch) - (assign36140_e41527 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn6) * locals.var_lch) - (assign36140_e41527 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn7) * locals.var_lch) - (assign36140_e41527 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn8) * locals.var_lch) - (assign36140_e41527 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn9) * locals.var_lch) - (assign36140_e41527 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn10) * locals.var_lch) - (assign36140_e41527 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn11) * locals.var_lch) - (assign36140_e41527 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn14) * locals.var_lch) - (assign36140_e41527 * locals.var_lch_dn14)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_betawl, locals.var_betawl_dn0, locals.var_betawl_dn2, locals.var_betawl_dn4, locals.var_betawl_dn5, locals.var_betawl_dn6, locals.var_betawl_dn7, locals.var_betawl_dn8, locals.var_betawl_dn9, locals.var_betawl_dn10, locals.var_betawl_dn11, locals.var_betawl_dn14,)
    }
};
        locals.var_betawl = assign36140_e41531;
        locals.var_betawl_dn0 = assign36140_e41531_d_n0;
        locals.var_betawl_dn2 = assign36140_e41531_d_n2;
        locals.var_betawl_dn4 = assign36140_e41531_d_n4;
        locals.var_betawl_dn5 = assign36140_e41531_d_n5;
        locals.var_betawl_dn6 = assign36140_e41531_d_n6;
        locals.var_betawl_dn7 = assign36140_e41531_d_n7;
        locals.var_betawl_dn8 = assign36140_e41531_d_n8;
        locals.var_betawl_dn9 = assign36140_e41531_d_n9;
        locals.var_betawl_dn10 = assign36140_e41531_d_n10;
        locals.var_betawl_dn11 = assign36140_e41531_d_n11;
        locals.var_betawl_dn14 = assign36140_e41531_d_n14;
        locals.var_betawl_rv = 0.0;

        let (assign36150_e41545, assign36150_e41545_d_n0, assign36150_e41545_d_n2, assign36150_e41545_d_n4, assign36150_e41545_d_n5, assign36150_e41545_d_n6, assign36150_e41545_d_n7, assign36150_e41545_d_n8, assign36150_e41545_d_n9, assign36150_e41545_d_n10, assign36150_e41545_d_n11, assign36150_e41545_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign36150_e41537: f64 = (locals.var_betawl * locals.var_idd);
        let assign36150_e41539: f64 = (assign36150_e41537 * locals.var_mu);
        let assign36150_e41541: f64 = (assign36150_e41539 + locals.var_ids_res);
        let assign36150_e41543: f64 = (assign36150_e41541 + locals.var_ids_bac);
        (assign36150_e41543, ((((((locals.var_betawl_dn0 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn0)) * locals.var_mu) + (assign36150_e41537 * locals.var_mu_dn0)) + locals.var_ids_res_dn0) + locals.var_ids_bac_dn0), ((((((locals.var_betawl_dn2 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn2)) * locals.var_mu) + (assign36150_e41537 * locals.var_mu_dn2)) + locals.var_ids_res_dn2) + locals.var_ids_bac_dn2), ((((((locals.var_betawl_dn4 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn4)) * locals.var_mu) + (assign36150_e41537 * locals.var_mu_dn4)) + locals.var_ids_res_dn4) + locals.var_ids_bac_dn4), ((((((locals.var_betawl_dn5 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn5)) * locals.var_mu) + (assign36150_e41537 * locals.var_mu_dn5)) + locals.var_ids_res_dn5) + locals.var_ids_bac_dn5), ((((((locals.var_betawl_dn6 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn6)) * locals.var_mu) + (assign36150_e41537 * locals.var_mu_dn6)) + locals.var_ids_res_dn6) + locals.var_ids_bac_dn6), ((((((locals.var_betawl_dn7 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn7)) * locals.var_mu) + (assign36150_e41537 * locals.var_mu_dn7)) + locals.var_ids_res_dn7) + locals.var_ids_bac_dn7), ((((((locals.var_betawl_dn8 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn8)) * locals.var_mu) + (assign36150_e41537 * locals.var_mu_dn8)) + locals.var_ids_res_dn8) + locals.var_ids_bac_dn8), ((((((locals.var_betawl_dn9 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn9)) * locals.var_mu) + (assign36150_e41537 * locals.var_mu_dn9)) + locals.var_ids_res_dn9) + locals.var_ids_bac_dn9), ((((((locals.var_betawl_dn10 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn10)) * locals.var_mu) + (assign36150_e41537 * locals.var_mu_dn10)) + locals.var_ids_res_dn10) + locals.var_ids_bac_dn10), ((((((locals.var_betawl_dn11 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn11)) * locals.var_mu) + (assign36150_e41537 * locals.var_mu_dn11)) + locals.var_ids_res_dn11) + locals.var_ids_bac_dn11), ((((((locals.var_betawl_dn14 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn14)) * locals.var_mu) + (assign36150_e41537 * locals.var_mu_dn14)) + locals.var_ids_res_dn14) + locals.var_ids_bac_dn14),)
    } else {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn4, locals.var_ids0_dn5, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn8, locals.var_ids0_dn9, locals.var_ids0_dn10, locals.var_ids0_dn11, locals.var_ids0_dn14,)
    }
};
        locals.var_ids0 = assign36150_e41545;
        locals.var_ids0_dn0 = assign36150_e41545_d_n0;
        locals.var_ids0_dn2 = assign36150_e41545_d_n2;
        locals.var_ids0_dn4 = assign36150_e41545_d_n4;
        locals.var_ids0_dn5 = assign36150_e41545_d_n5;
        locals.var_ids0_dn6 = assign36150_e41545_d_n6;
        locals.var_ids0_dn7 = assign36150_e41545_d_n7;
        locals.var_ids0_dn8 = assign36150_e41545_d_n8;
        locals.var_ids0_dn9 = assign36150_e41545_d_n9;
        locals.var_ids0_dn10 = assign36150_e41545_d_n10;
        locals.var_ids0_dn11 = assign36150_e41545_d_n11;
        locals.var_ids0_dn14 = assign36150_e41545_d_n14;
        locals.var_ids0_rv = 0.0;

        let (assign36160_e41555, assign36160_e41555_d_n0, assign36160_e41555_d_n2, assign36160_e41555_d_n4, assign36160_e41555_d_n5, assign36160_e41555_d_n6, assign36160_e41555_d_n7, assign36160_e41555_d_n8, assign36160_e41555_d_n9, assign36160_e41555_d_n10, assign36160_e41555_d_n11, assign36160_e41555_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign36160_e41551: f64 = (locals.var_betawl * locals.var_idd);
        let assign36160_e41553: f64 = (assign36160_e41551 * locals.var_mu);
        (assign36160_e41553, ((((locals.var_betawl_dn0 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn0)) * locals.var_mu) + (assign36160_e41551 * locals.var_mu_dn0)), ((((locals.var_betawl_dn2 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn2)) * locals.var_mu) + (assign36160_e41551 * locals.var_mu_dn2)), ((((locals.var_betawl_dn4 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn4)) * locals.var_mu) + (assign36160_e41551 * locals.var_mu_dn4)), ((((locals.var_betawl_dn5 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn5)) * locals.var_mu) + (assign36160_e41551 * locals.var_mu_dn5)), ((((locals.var_betawl_dn6 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn6)) * locals.var_mu) + (assign36160_e41551 * locals.var_mu_dn6)), ((((locals.var_betawl_dn7 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn7)) * locals.var_mu) + (assign36160_e41551 * locals.var_mu_dn7)), ((((locals.var_betawl_dn8 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn8)) * locals.var_mu) + (assign36160_e41551 * locals.var_mu_dn8)), ((((locals.var_betawl_dn9 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn9)) * locals.var_mu) + (assign36160_e41551 * locals.var_mu_dn9)), ((((locals.var_betawl_dn10 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn10)) * locals.var_mu) + (assign36160_e41551 * locals.var_mu_dn10)), ((((locals.var_betawl_dn11 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn11)) * locals.var_mu) + (assign36160_e41551 * locals.var_mu_dn11)), ((((locals.var_betawl_dn14 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn14)) * locals.var_mu) + (assign36160_e41551 * locals.var_mu_dn14)),)
    } else {
        (locals.var_ids_acc, locals.var_ids_acc_dn0, locals.var_ids_acc_dn2, locals.var_ids_acc_dn4, locals.var_ids_acc_dn5, locals.var_ids_acc_dn6, locals.var_ids_acc_dn7, locals.var_ids_acc_dn8, locals.var_ids_acc_dn9, locals.var_ids_acc_dn10, locals.var_ids_acc_dn11, locals.var_ids_acc_dn14,)
    }
};
        locals.var_ids_acc = assign36160_e41555;
        locals.var_ids_acc_dn0 = assign36160_e41555_d_n0;
        locals.var_ids_acc_dn2 = assign36160_e41555_d_n2;
        locals.var_ids_acc_dn4 = assign36160_e41555_d_n4;
        locals.var_ids_acc_dn5 = assign36160_e41555_d_n5;
        locals.var_ids_acc_dn6 = assign36160_e41555_d_n6;
        locals.var_ids_acc_dn7 = assign36160_e41555_d_n7;
        locals.var_ids_acc_dn8 = assign36160_e41555_d_n8;
        locals.var_ids_acc_dn9 = assign36160_e41555_d_n9;
        locals.var_ids_acc_dn10 = assign36160_e41555_d_n10;
        locals.var_ids_acc_dn11 = assign36160_e41555_d_n11;
        locals.var_ids_acc_dn14 = assign36160_e41555_d_n14;
        locals.var_ids_acc_rv = 0.0;

        let (assign36170_e41561, assign36170_e41561_d_n0, assign36170_e41561_d_n2, assign36170_e41561_d_n4, assign36170_e41561_d_n5, assign36170_e41561_d_n6, assign36170_e41561_d_n7, assign36170_e41561_d_n8, assign36170_e41561_d_n9, assign36170_e41561_d_n10, assign36170_e41561_d_n11, assign36170_e41561_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        (locals.var_mu, locals.var_mu_dn0, locals.var_mu_dn2, locals.var_mu_dn4, locals.var_mu_dn5, locals.var_mu_dn6, locals.var_mu_dn7, locals.var_mu_dn8, locals.var_mu_dn9, locals.var_mu_dn10, locals.var_mu_dn11, locals.var_mu_dn14,)
    } else {
        (locals.var_mu_acc, locals.var_mu_acc_dn0, locals.var_mu_acc_dn2, locals.var_mu_acc_dn4, locals.var_mu_acc_dn5, locals.var_mu_acc_dn6, locals.var_mu_acc_dn7, locals.var_mu_acc_dn8, locals.var_mu_acc_dn9, locals.var_mu_acc_dn10, locals.var_mu_acc_dn11, locals.var_mu_acc_dn14,)
    }
};
        locals.var_mu_acc = assign36170_e41561;
        locals.var_mu_acc_dn0 = assign36170_e41561_d_n0;
        locals.var_mu_acc_dn2 = assign36170_e41561_d_n2;
        locals.var_mu_acc_dn4 = assign36170_e41561_d_n4;
        locals.var_mu_acc_dn5 = assign36170_e41561_d_n5;
        locals.var_mu_acc_dn6 = assign36170_e41561_d_n6;
        locals.var_mu_acc_dn7 = assign36170_e41561_d_n7;
        locals.var_mu_acc_dn8 = assign36170_e41561_d_n8;
        locals.var_mu_acc_dn9 = assign36170_e41561_d_n9;
        locals.var_mu_acc_dn10 = assign36170_e41561_d_n10;
        locals.var_mu_acc_dn11 = assign36170_e41561_d_n11;
        locals.var_mu_acc_dn14 = assign36170_e41561_d_n14;
        locals.var_mu_acc_rv = 0.0;

        let (assign36180_e41567, assign36180_e41567_d_n0, assign36180_e41567_d_n2, assign36180_e41567_d_n4, assign36180_e41567_d_n5, assign36180_e41567_d_n6, assign36180_e41567_d_n7, assign36180_e41567_d_n8, assign36180_e41567_d_n9, assign36180_e41567_d_n10, assign36180_e41567_d_n11, assign36180_e41567_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn14,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    }
};
        locals.var_vds = assign36180_e41567;
        locals.var_vds_dn0 = assign36180_e41567_d_n0;
        locals.var_vds_dn2 = assign36180_e41567_d_n2;
        locals.var_vds_dn4 = assign36180_e41567_d_n4;
        locals.var_vds_dn5 = assign36180_e41567_d_n5;
        locals.var_vds_dn6 = assign36180_e41567_d_n6;
        locals.var_vds_dn7 = assign36180_e41567_d_n7;
        locals.var_vds_dn8 = assign36180_e41567_d_n8;
        locals.var_vds_dn9 = assign36180_e41567_d_n9;
        locals.var_vds_dn10 = assign36180_e41567_d_n10;
        locals.var_vds_dn11 = assign36180_e41567_d_n11;
        locals.var_vds_dn14 = assign36180_e41567_d_n14;
        locals.var_vds_rv = 0.0;

        let assign36190_e41570: f64 = if p.p283 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard823 = assign36190_e41570;
        locals.var_guard823_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_119(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign36200_e41582, assign36200_e41582_d_n0, assign36200_e41582_d_n2, assign36200_e41582_d_n4, assign36200_e41582_d_n5, assign36200_e41582_d_n6, assign36200_e41582_d_n7, assign36200_e41582_d_n8, assign36200_e41582_d_n9, assign36200_e41582_d_n10, assign36200_e41582_d_n11, assign36200_e41582_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard823 != 0.0)) {
        let assign36200_e41579: f64 = (locals.var_vds - locals.var_pds);
        let assign36200_e41580: f64 = (0.5 * assign36200_e41579);
        (assign36200_e41580, (0.5 * (locals.var_vds_dn0 - locals.var_pds_dn0)), (0.5 * (locals.var_vds_dn2 - locals.var_pds_dn2)), (0.5 * (locals.var_vds_dn4 - locals.var_pds_dn4)), (0.5 * (locals.var_vds_dn5 - locals.var_pds_dn5)), (0.5 * (locals.var_vds_dn6 - locals.var_pds_dn6)), (0.5 * (locals.var_vds_dn7 - locals.var_pds_dn7)), (0.5 * (locals.var_vds_dn8 - locals.var_pds_dn8)), (0.5 * (locals.var_vds_dn9 - locals.var_pds_dn9)), (0.5 * (locals.var_vds_dn10 - locals.var_pds_dn10)), (0.5 * (locals.var_vds_dn11 - locals.var_pds_dn11)), (0.5 * (locals.var_vds_dn14 - locals.var_pds_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign36200_e41582;
        locals.var_t1_dn0 = assign36200_e41582_d_n0;
        locals.var_t1_dn2 = assign36200_e41582_d_n2;
        locals.var_t1_dn4 = assign36200_e41582_d_n4;
        locals.var_t1_dn5 = assign36200_e41582_d_n5;
        locals.var_t1_dn6 = assign36200_e41582_d_n6;
        locals.var_t1_dn7 = assign36200_e41582_d_n7;
        locals.var_t1_dn8 = assign36200_e41582_d_n8;
        locals.var_t1_dn9 = assign36200_e41582_d_n9;
        locals.var_t1_dn10 = assign36200_e41582_d_n10;
        locals.var_t1_dn11 = assign36200_e41582_d_n11;
        locals.var_t1_dn14 = assign36200_e41582_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign36210_e41594, assign36210_e41594_d_n0, assign36210_e41594_d_n2, assign36210_e41594_d_n4, assign36210_e41594_d_n5, assign36210_e41594_d_n6, assign36210_e41594_d_n7, assign36210_e41594_d_n8, assign36210_e41594_d_n9, assign36210_e41594_d_n10, assign36210_e41594_d_n11, assign36210_e41594_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard823 != 0.0)) {
        let assign36210_e41590: f64 = (2.0 * locals.var_t1);
        let assign36210_e41592: f64 = (assign36210_e41590 / 0.01);
        (assign36210_e41592, ((2.0 * locals.var_t1_dn0) / 0.01), ((2.0 * locals.var_t1_dn2) / 0.01), ((2.0 * locals.var_t1_dn4) / 0.01), ((2.0 * locals.var_t1_dn5) / 0.01), ((2.0 * locals.var_t1_dn6) / 0.01), ((2.0 * locals.var_t1_dn7) / 0.01), ((2.0 * locals.var_t1_dn8) / 0.01), ((2.0 * locals.var_t1_dn9) / 0.01), ((2.0 * locals.var_t1_dn10) / 0.01), ((2.0 * locals.var_t1_dn11) / 0.01), ((2.0 * locals.var_t1_dn14) / 0.01),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign36210_e41594;
        locals.var_tmf1_dn0 = assign36210_e41594_d_n0;
        locals.var_tmf1_dn2 = assign36210_e41594_d_n2;
        locals.var_tmf1_dn4 = assign36210_e41594_d_n4;
        locals.var_tmf1_dn5 = assign36210_e41594_d_n5;
        locals.var_tmf1_dn6 = assign36210_e41594_d_n6;
        locals.var_tmf1_dn7 = assign36210_e41594_d_n7;
        locals.var_tmf1_dn8 = assign36210_e41594_d_n8;
        locals.var_tmf1_dn9 = assign36210_e41594_d_n9;
        locals.var_tmf1_dn10 = assign36210_e41594_d_n10;
        locals.var_tmf1_dn11 = assign36210_e41594_d_n11;
        locals.var_tmf1_dn14 = assign36210_e41594_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign36220_e41638, assign36220_e41638_d_n0, assign36220_e41638_d_n2, assign36220_e41638_d_n4, assign36220_e41638_d_n5, assign36220_e41638_d_n6, assign36220_e41638_d_n7, assign36220_e41638_d_n8, assign36220_e41638_d_n9, assign36220_e41638_d_n10, assign36220_e41638_d_n11, assign36220_e41638_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard823 != 0.0)) {
        let assign36220_e41604: f64 = (1.0 / 2.0);
        let assign36220_e41608: f64 = (1.0 / 6.0);
        let assign36220_e41612: f64 = (1.0 / 24.0);
        let assign36220_e41616: f64 = (1.0 / 120.0);
        let assign36220_e41620: f64 = (1.0 / 720.0);
        let assign36220_e41624: f64 = (1.0 / 5040.0);
        let assign36220_e41625: f64 = (locals.var_tmf1 * assign36220_e41624);
        let assign36220_e41626: f64 = (assign36220_e41620 + assign36220_e41625);
        let assign36220_e41627: f64 = (locals.var_tmf1 * assign36220_e41626);
        let assign36220_e41628: f64 = (assign36220_e41616 + assign36220_e41627);
        let assign36220_e41629: f64 = (locals.var_tmf1 * assign36220_e41628);
        let assign36220_e41630: f64 = (assign36220_e41612 + assign36220_e41629);
        let assign36220_e41631: f64 = (locals.var_tmf1 * assign36220_e41630);
        let assign36220_e41632: f64 = (assign36220_e41608 + assign36220_e41631);
        let assign36220_e41633: f64 = (locals.var_tmf1 * assign36220_e41632);
        let assign36220_e41634: f64 = (assign36220_e41604 + assign36220_e41633);
        let assign36220_e41635: f64 = (locals.var_tmf1 * assign36220_e41634);
        let assign36220_e41636: f64 = (1.0 + assign36220_e41635);
        (assign36220_e41636, ((locals.var_tmf1_dn0 * assign36220_e41634) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign36220_e41632) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign36220_e41630) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign36220_e41628) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign36220_e41626) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign36220_e41624))))))))))), ((locals.var_tmf1_dn2 * assign36220_e41634) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign36220_e41632) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign36220_e41630) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign36220_e41628) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign36220_e41626) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign36220_e41624))))))))))), ((locals.var_tmf1_dn4 * assign36220_e41634) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign36220_e41632) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign36220_e41630) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign36220_e41628) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign36220_e41626) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign36220_e41624))))))))))), ((locals.var_tmf1_dn5 * assign36220_e41634) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign36220_e41632) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign36220_e41630) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign36220_e41628) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign36220_e41626) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign36220_e41624))))))))))), ((locals.var_tmf1_dn6 * assign36220_e41634) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign36220_e41632) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign36220_e41630) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign36220_e41628) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign36220_e41626) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign36220_e41624))))))))))), ((locals.var_tmf1_dn7 * assign36220_e41634) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign36220_e41632) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign36220_e41630) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign36220_e41628) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign36220_e41626) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign36220_e41624))))))))))), ((locals.var_tmf1_dn8 * assign36220_e41634) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign36220_e41632) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign36220_e41630) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign36220_e41628) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign36220_e41626) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign36220_e41624))))))))))), ((locals.var_tmf1_dn9 * assign36220_e41634) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign36220_e41632) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign36220_e41630) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign36220_e41628) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign36220_e41626) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign36220_e41624))))))))))), ((locals.var_tmf1_dn10 * assign36220_e41634) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign36220_e41632) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign36220_e41630) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign36220_e41628) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign36220_e41626) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign36220_e41624))))))))))), ((locals.var_tmf1_dn11 * assign36220_e41634) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign36220_e41632) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign36220_e41630) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign36220_e41628) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign36220_e41626) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign36220_e41624))))))))))), ((locals.var_tmf1_dn14 * assign36220_e41634) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign36220_e41632) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign36220_e41630) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign36220_e41628) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign36220_e41626) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign36220_e41624))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign36220_e41638;
        locals.var_tmf2_dn0 = assign36220_e41638_d_n0;
        locals.var_tmf2_dn2 = assign36220_e41638_d_n2;
        locals.var_tmf2_dn4 = assign36220_e41638_d_n4;
        locals.var_tmf2_dn5 = assign36220_e41638_d_n5;
        locals.var_tmf2_dn6 = assign36220_e41638_d_n6;
        locals.var_tmf2_dn7 = assign36220_e41638_d_n7;
        locals.var_tmf2_dn8 = assign36220_e41638_d_n8;
        locals.var_tmf2_dn9 = assign36220_e41638_d_n9;
        locals.var_tmf2_dn10 = assign36220_e41638_d_n10;
        locals.var_tmf2_dn11 = assign36220_e41638_d_n11;
        locals.var_tmf2_dn14 = assign36220_e41638_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign36230_e41678, assign36230_e41678_d_n0, assign36230_e41678_d_n2, assign36230_e41678_d_n4, assign36230_e41678_d_n5, assign36230_e41678_d_n6, assign36230_e41678_d_n7, assign36230_e41678_d_n8, assign36230_e41678_d_n9, assign36230_e41678_d_n10, assign36230_e41678_d_n11, assign36230_e41678_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard823 != 0.0)) {
        let assign36230_e41646: f64 = (1.0 / 2.0);
        let assign36230_e41650: f64 = (1.0 / 3.0);
        let assign36230_e41654: f64 = (1.0 / 8.0);
        let assign36230_e41658: f64 = (1.0 / 30.0);
        let assign36230_e41662: f64 = (1.0 / 144.0);
        let assign36230_e41666: f64 = (1.0 / 840.0);
        let assign36230_e41667: f64 = (locals.var_tmf1 * assign36230_e41666);
        let assign36230_e41668: f64 = (assign36230_e41662 + assign36230_e41667);
        let assign36230_e41669: f64 = (locals.var_tmf1 * assign36230_e41668);
        let assign36230_e41670: f64 = (assign36230_e41658 + assign36230_e41669);
        let assign36230_e41671: f64 = (locals.var_tmf1 * assign36230_e41670);
        let assign36230_e41672: f64 = (assign36230_e41654 + assign36230_e41671);
        let assign36230_e41673: f64 = (locals.var_tmf1 * assign36230_e41672);
        let assign36230_e41674: f64 = (assign36230_e41650 + assign36230_e41673);
        let assign36230_e41675: f64 = (locals.var_tmf1 * assign36230_e41674);
        let assign36230_e41676: f64 = (assign36230_e41646 + assign36230_e41675);
        (assign36230_e41676, ((locals.var_tmf1_dn0 * assign36230_e41674) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign36230_e41672) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign36230_e41670) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign36230_e41668) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign36230_e41666))))))))), ((locals.var_tmf1_dn2 * assign36230_e41674) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign36230_e41672) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign36230_e41670) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign36230_e41668) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign36230_e41666))))))))), ((locals.var_tmf1_dn4 * assign36230_e41674) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign36230_e41672) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign36230_e41670) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign36230_e41668) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign36230_e41666))))))))), ((locals.var_tmf1_dn5 * assign36230_e41674) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign36230_e41672) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign36230_e41670) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign36230_e41668) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign36230_e41666))))))))), ((locals.var_tmf1_dn6 * assign36230_e41674) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign36230_e41672) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign36230_e41670) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign36230_e41668) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign36230_e41666))))))))), ((locals.var_tmf1_dn7 * assign36230_e41674) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign36230_e41672) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign36230_e41670) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign36230_e41668) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign36230_e41666))))))))), ((locals.var_tmf1_dn8 * assign36230_e41674) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign36230_e41672) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign36230_e41670) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign36230_e41668) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign36230_e41666))))))))), ((locals.var_tmf1_dn9 * assign36230_e41674) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign36230_e41672) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign36230_e41670) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign36230_e41668) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign36230_e41666))))))))), ((locals.var_tmf1_dn10 * assign36230_e41674) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign36230_e41672) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign36230_e41670) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign36230_e41668) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign36230_e41666))))))))), ((locals.var_tmf1_dn11 * assign36230_e41674) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign36230_e41672) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign36230_e41670) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign36230_e41668) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign36230_e41666))))))))), ((locals.var_tmf1_dn14 * assign36230_e41674) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign36230_e41672) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign36230_e41670) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign36230_e41668) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign36230_e41666))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign36230_e41678;
        locals.var_tmf3_dn0 = assign36230_e41678_d_n0;
        locals.var_tmf3_dn2 = assign36230_e41678_d_n2;
        locals.var_tmf3_dn4 = assign36230_e41678_d_n4;
        locals.var_tmf3_dn5 = assign36230_e41678_d_n5;
        locals.var_tmf3_dn6 = assign36230_e41678_d_n6;
        locals.var_tmf3_dn7 = assign36230_e41678_d_n7;
        locals.var_tmf3_dn8 = assign36230_e41678_d_n8;
        locals.var_tmf3_dn9 = assign36230_e41678_d_n9;
        locals.var_tmf3_dn10 = assign36230_e41678_d_n10;
        locals.var_tmf3_dn11 = assign36230_e41678_d_n11;
        locals.var_tmf3_dn14 = assign36230_e41678_d_n14;
        locals.var_tmf3_rv = 0.0;

        let (assign36240_e41688, assign36240_e41688_d_n0, assign36240_e41688_d_n2, assign36240_e41688_d_n4, assign36240_e41688_d_n5, assign36240_e41688_d_n6, assign36240_e41688_d_n7, assign36240_e41688_d_n8, assign36240_e41688_d_n9, assign36240_e41688_d_n10, assign36240_e41688_d_n11, assign36240_e41688_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard823 != 0.0)) {
        let assign36240_e41686: f64 = (0.01 / locals.var_tmf2);
        (assign36240_e41686, (-((0.01 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign36240_e41688;
        locals.var_t6_dn0 = assign36240_e41688_d_n0;
        locals.var_t6_dn2 = assign36240_e41688_d_n2;
        locals.var_t6_dn4 = assign36240_e41688_d_n4;
        locals.var_t6_dn5 = assign36240_e41688_d_n5;
        locals.var_t6_dn6 = assign36240_e41688_d_n6;
        locals.var_t6_dn7 = assign36240_e41688_d_n7;
        locals.var_t6_dn8 = assign36240_e41688_d_n8;
        locals.var_t6_dn9 = assign36240_e41688_d_n9;
        locals.var_t6_dn10 = assign36240_e41688_d_n10;
        locals.var_t6_dn11 = assign36240_e41688_d_n11;
        locals.var_t6_dn14 = assign36240_e41688_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign36250_e41703, assign36250_e41703_d_n0, assign36250_e41703_d_n2, assign36250_e41703_d_n4, assign36250_e41703_d_n5, assign36250_e41703_d_n6, assign36250_e41703_d_n7, assign36250_e41703_d_n8, assign36250_e41703_d_n9, assign36250_e41703_d_n10, assign36250_e41703_d_n11, assign36250_e41703_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard823 != 0.0)) {
        let assign36250_e41695: f64 = (-2.0);
        let assign36250_e41697: f64 = (assign36250_e41695 * locals.var_tmf3);
        let assign36250_e41700: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign36250_e41701: f64 = (assign36250_e41697 / assign36250_e41700);
        (assign36250_e41701, ((((assign36250_e41695 * locals.var_tmf3_dn0) * assign36250_e41700) - (assign36250_e41697 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign36250_e41700 * assign36250_e41700)), ((((assign36250_e41695 * locals.var_tmf3_dn2) * assign36250_e41700) - (assign36250_e41697 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign36250_e41700 * assign36250_e41700)), ((((assign36250_e41695 * locals.var_tmf3_dn4) * assign36250_e41700) - (assign36250_e41697 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign36250_e41700 * assign36250_e41700)), ((((assign36250_e41695 * locals.var_tmf3_dn5) * assign36250_e41700) - (assign36250_e41697 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign36250_e41700 * assign36250_e41700)), ((((assign36250_e41695 * locals.var_tmf3_dn6) * assign36250_e41700) - (assign36250_e41697 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign36250_e41700 * assign36250_e41700)), ((((assign36250_e41695 * locals.var_tmf3_dn7) * assign36250_e41700) - (assign36250_e41697 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign36250_e41700 * assign36250_e41700)), ((((assign36250_e41695 * locals.var_tmf3_dn8) * assign36250_e41700) - (assign36250_e41697 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign36250_e41700 * assign36250_e41700)), ((((assign36250_e41695 * locals.var_tmf3_dn9) * assign36250_e41700) - (assign36250_e41697 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign36250_e41700 * assign36250_e41700)), ((((assign36250_e41695 * locals.var_tmf3_dn10) * assign36250_e41700) - (assign36250_e41697 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign36250_e41700 * assign36250_e41700)), ((((assign36250_e41695 * locals.var_tmf3_dn11) * assign36250_e41700) - (assign36250_e41697 * ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)))) / (assign36250_e41700 * assign36250_e41700)), ((((assign36250_e41695 * locals.var_tmf3_dn14) * assign36250_e41700) - (assign36250_e41697 * ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)))) / (assign36250_e41700 * assign36250_e41700)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign36250_e41703;
        locals.var_t2_dn0 = assign36250_e41703_d_n0;
        locals.var_t2_dn2 = assign36250_e41703_d_n2;
        locals.var_t2_dn4 = assign36250_e41703_d_n4;
        locals.var_t2_dn5 = assign36250_e41703_d_n5;
        locals.var_t2_dn6 = assign36250_e41703_d_n6;
        locals.var_t2_dn7 = assign36250_e41703_d_n7;
        locals.var_t2_dn8 = assign36250_e41703_d_n8;
        locals.var_t2_dn9 = assign36250_e41703_d_n9;
        locals.var_t2_dn10 = assign36250_e41703_d_n10;
        locals.var_t2_dn11 = assign36250_e41703_d_n11;
        locals.var_t2_dn14 = assign36250_e41703_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign36260_e41715, assign36260_e41715_d_n0, assign36260_e41715_d_n2, assign36260_e41715_d_n4, assign36260_e41715_d_n5, assign36260_e41715_d_n6, assign36260_e41715_d_n7, assign36260_e41715_d_n8, assign36260_e41715_d_n9, assign36260_e41715_d_n10, assign36260_e41715_d_n11, assign36260_e41715_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard823 != 0.0)) {
        let assign36260_e41712: f64 = (locals.var_phi_s0_dep + locals.var_t6);
        let assign36260_e41713: f64 = (1.1 - assign36260_e41712);
        (assign36260_e41713, (-(locals.var_phi_s0_dep_dn0 + locals.var_t6_dn0)), (-(locals.var_phi_s0_dep_dn2 + locals.var_t6_dn2)), (-(locals.var_phi_s0_dep_dn4 + locals.var_t6_dn4)), (-(locals.var_phi_s0_dep_dn5 + locals.var_t6_dn5)), (-(locals.var_phi_s0_dep_dn6 + locals.var_t6_dn6)), (-(locals.var_phi_s0_dep_dn7 + locals.var_t6_dn7)), (-(locals.var_phi_s0_dep_dn8 + locals.var_t6_dn8)), (-(locals.var_phi_s0_dep_dn9 + locals.var_t6_dn9)), (-(locals.var_phi_s0_dep_dn10 + locals.var_t6_dn10)), (-(locals.var_phi_s0_dep_dn11 + locals.var_t6_dn11)), (-(locals.var_phi_s0_dep_dn14 + locals.var_t6_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign36260_e41715;
        locals.var_t1_dn0 = assign36260_e41715_d_n0;
        locals.var_t1_dn2 = assign36260_e41715_d_n2;
        locals.var_t1_dn4 = assign36260_e41715_d_n4;
        locals.var_t1_dn5 = assign36260_e41715_d_n5;
        locals.var_t1_dn6 = assign36260_e41715_d_n6;
        locals.var_t1_dn7 = assign36260_e41715_d_n7;
        locals.var_t1_dn8 = assign36260_e41715_d_n8;
        locals.var_t1_dn9 = assign36260_e41715_d_n9;
        locals.var_t1_dn10 = assign36260_e41715_d_n10;
        locals.var_t1_dn11 = assign36260_e41715_d_n11;
        locals.var_t1_dn14 = assign36260_e41715_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign36270_e41732, assign36270_e41732_d_n0, assign36270_e41732_d_n2, assign36270_e41732_d_n4, assign36270_e41732_d_n5, assign36270_e41732_d_n6, assign36270_e41732_d_n7, assign36270_e41732_d_n8, assign36270_e41732_d_n9, assign36270_e41732_d_n10, assign36270_e41732_d_n11, assign36270_e41732_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard823 != 0.0)) {
        let assign36270_e41723: f64 = (locals.var_t1 * locals.var_t1);
        let assign36270_e41726: f64 = (4.0 * 0.05);
        let assign36270_e41728: f64 = (assign36270_e41726 * 0.05);
        let assign36270_e41729: f64 = (assign36270_e41723 + assign36270_e41728);
        let assign36270_e41730: f64 = (assign36270_e41729).sqrt();
        (assign36270_e41730, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign36270_e41730)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign36270_e41730)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign36270_e41730)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign36270_e41730)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign36270_e41730)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign36270_e41730)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign36270_e41730)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign36270_e41730)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign36270_e41730)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign36270_e41730)), (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign36270_e41730)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign36270_e41732;
        locals.var_tmf2_dn0 = assign36270_e41732_d_n0;
        locals.var_tmf2_dn2 = assign36270_e41732_d_n2;
        locals.var_tmf2_dn4 = assign36270_e41732_d_n4;
        locals.var_tmf2_dn5 = assign36270_e41732_d_n5;
        locals.var_tmf2_dn6 = assign36270_e41732_d_n6;
        locals.var_tmf2_dn7 = assign36270_e41732_d_n7;
        locals.var_tmf2_dn8 = assign36270_e41732_d_n8;
        locals.var_tmf2_dn9 = assign36270_e41732_d_n9;
        locals.var_tmf2_dn10 = assign36270_e41732_d_n10;
        locals.var_tmf2_dn11 = assign36270_e41732_d_n11;
        locals.var_tmf2_dn14 = assign36270_e41732_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign36280_e41746, assign36280_e41746_d_n0, assign36280_e41746_d_n2, assign36280_e41746_d_n4, assign36280_e41746_d_n5, assign36280_e41746_d_n6, assign36280_e41746_d_n7, assign36280_e41746_d_n8, assign36280_e41746_d_n9, assign36280_e41746_d_n10, assign36280_e41746_d_n11, assign36280_e41746_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard823 != 0.0)) {
        let assign36280_e41742: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign36280_e41743: f64 = (1.0 + assign36280_e41742);
        let assign36280_e41744: f64 = (0.5 * assign36280_e41743);
        (assign36280_e41744, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn14 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign36280_e41746;
        locals.var_t0_dn0 = assign36280_e41746_d_n0;
        locals.var_t0_dn2 = assign36280_e41746_d_n2;
        locals.var_t0_dn4 = assign36280_e41746_d_n4;
        locals.var_t0_dn5 = assign36280_e41746_d_n5;
        locals.var_t0_dn6 = assign36280_e41746_d_n6;
        locals.var_t0_dn7 = assign36280_e41746_d_n7;
        locals.var_t0_dn8 = assign36280_e41746_d_n8;
        locals.var_t0_dn9 = assign36280_e41746_d_n9;
        locals.var_t0_dn10 = assign36280_e41746_d_n10;
        locals.var_t0_dn11 = assign36280_e41746_d_n11;
        locals.var_t0_dn14 = assign36280_e41746_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign36290_e41758, assign36290_e41758_d_n0, assign36290_e41758_d_n2, assign36290_e41758_d_n4, assign36290_e41758_d_n5, assign36290_e41758_d_n6, assign36290_e41758_d_n7, assign36290_e41758_d_n8, assign36290_e41758_d_n9, assign36290_e41758_d_n10, assign36290_e41758_d_n11, assign36290_e41758_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard823 != 0.0)) {
        let assign36290_e41755: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign36290_e41756: f64 = (0.5 * assign36290_e41755);
        (assign36290_e41756, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign36290_e41758;
        locals.var_t2_dn0 = assign36290_e41758_d_n0;
        locals.var_t2_dn2 = assign36290_e41758_d_n2;
        locals.var_t2_dn4 = assign36290_e41758_d_n4;
        locals.var_t2_dn5 = assign36290_e41758_d_n5;
        locals.var_t2_dn6 = assign36290_e41758_d_n6;
        locals.var_t2_dn7 = assign36290_e41758_d_n7;
        locals.var_t2_dn8 = assign36290_e41758_d_n8;
        locals.var_t2_dn9 = assign36290_e41758_d_n9;
        locals.var_t2_dn10 = assign36290_e41758_d_n10;
        locals.var_t2_dn11 = assign36290_e41758_d_n11;
        locals.var_t2_dn14 = assign36290_e41758_d_n14;
        locals.var_t2_rv = 0.0;

        let assign36300_e41761: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard824 = assign36300_e41761;
        locals.var_guard824_rv = 0.0;

        let (assign36310_e41771, assign36310_e41771_d_n0, assign36310_e41771_d_n2, assign36310_e41771_d_n4, assign36310_e41771_d_n5, assign36310_e41771_d_n6, assign36310_e41771_d_n7, assign36310_e41771_d_n8, assign36310_e41771_d_n9, assign36310_e41771_d_n10, assign36310_e41771_d_n11, assign36310_e41771_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard823 != 0.0)) && (locals.var_guard824 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign36310_e41771;
        locals.var_t2_dn0 = assign36310_e41771_d_n0;
        locals.var_t2_dn2 = assign36310_e41771_d_n2;
        locals.var_t2_dn4 = assign36310_e41771_d_n4;
        locals.var_t2_dn5 = assign36310_e41771_d_n5;
        locals.var_t2_dn6 = assign36310_e41771_d_n6;
        locals.var_t2_dn7 = assign36310_e41771_d_n7;
        locals.var_t2_dn8 = assign36310_e41771_d_n8;
        locals.var_t2_dn9 = assign36310_e41771_d_n9;
        locals.var_t2_dn10 = assign36310_e41771_d_n10;
        locals.var_t2_dn11 = assign36310_e41771_d_n11;
        locals.var_t2_dn14 = assign36310_e41771_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign36320_e41781, assign36320_e41781_d_n0, assign36320_e41781_d_n2, assign36320_e41781_d_n4, assign36320_e41781_d_n5, assign36320_e41781_d_n6, assign36320_e41781_d_n7, assign36320_e41781_d_n8, assign36320_e41781_d_n9, assign36320_e41781_d_n10, assign36320_e41781_d_n11, assign36320_e41781_d_n14,) = {
    if ((((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard823 != 0.0)) && (locals.var_guard824 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign36320_e41781;
        locals.var_t0_dn0 = assign36320_e41781_d_n0;
        locals.var_t0_dn2 = assign36320_e41781_d_n2;
        locals.var_t0_dn4 = assign36320_e41781_d_n4;
        locals.var_t0_dn5 = assign36320_e41781_d_n5;
        locals.var_t0_dn6 = assign36320_e41781_d_n6;
        locals.var_t0_dn7 = assign36320_e41781_d_n7;
        locals.var_t0_dn8 = assign36320_e41781_d_n8;
        locals.var_t0_dn9 = assign36320_e41781_d_n9;
        locals.var_t0_dn10 = assign36320_e41781_d_n10;
        locals.var_t0_dn11 = assign36320_e41781_d_n11;
        locals.var_t0_dn14 = assign36320_e41781_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign36330_e41791, assign36330_e41791_d_n0, assign36330_e41791_d_n2, assign36330_e41791_d_n4, assign36330_e41791_d_n5, assign36330_e41791_d_n6, assign36330_e41791_d_n7, assign36330_e41791_d_n8, assign36330_e41791_d_n9, assign36330_e41791_d_n10, assign36330_e41791_d_n11, assign36330_e41791_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard823 != 0.0)) {
        let assign36330_e41789: f64 = (locals.var_t2 + 1e-25);
        (assign36330_e41789, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign36330_e41791;
        locals.var_t2_dn0 = assign36330_e41791_d_n0;
        locals.var_t2_dn2 = assign36330_e41791_d_n2;
        locals.var_t2_dn4 = assign36330_e41791_d_n4;
        locals.var_t2_dn5 = assign36330_e41791_d_n5;
        locals.var_t2_dn6 = assign36330_e41791_d_n6;
        locals.var_t2_dn7 = assign36330_e41791_d_n7;
        locals.var_t2_dn8 = assign36330_e41791_d_n8;
        locals.var_t2_dn9 = assign36330_e41791_d_n9;
        locals.var_t2_dn10 = assign36330_e41791_d_n10;
        locals.var_t2_dn11 = assign36330_e41791_d_n11;
        locals.var_t2_dn14 = assign36330_e41791_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign36340_e41801, assign36340_e41801_d_n0, assign36340_e41801_d_n2, assign36340_e41801_d_n4, assign36340_e41801_d_n5, assign36340_e41801_d_n6, assign36340_e41801_d_n7, assign36340_e41801_d_n8, assign36340_e41801_d_n9, assign36340_e41801_d_n10, assign36340_e41801_d_n11, assign36340_e41801_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard823 != 0.0)) {
        let assign36340_e41799: f64 = (locals.var_beta * locals.var_ptl0);
        (assign36340_e41799, (locals.var_beta_dn0 * locals.var_ptl0), (locals.var_beta_dn2 * locals.var_ptl0), (locals.var_beta_dn4 * locals.var_ptl0), (locals.var_beta_dn5 * locals.var_ptl0), (locals.var_beta_dn6 * locals.var_ptl0), (locals.var_beta_dn7 * locals.var_ptl0), (locals.var_beta_dn8 * locals.var_ptl0), (locals.var_beta_dn9 * locals.var_ptl0), (locals.var_beta_dn10 * locals.var_ptl0), (locals.var_beta_dn11 * locals.var_ptl0), (locals.var_beta_dn14 * locals.var_ptl0),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign36340_e41801;
        locals.var_t0_dn0 = assign36340_e41801_d_n0;
        locals.var_t0_dn2 = assign36340_e41801_d_n2;
        locals.var_t0_dn4 = assign36340_e41801_d_n4;
        locals.var_t0_dn5 = assign36340_e41801_d_n5;
        locals.var_t0_dn6 = assign36340_e41801_d_n6;
        locals.var_t0_dn7 = assign36340_e41801_d_n7;
        locals.var_t0_dn8 = assign36340_e41801_d_n8;
        locals.var_t0_dn9 = assign36340_e41801_d_n9;
        locals.var_t0_dn10 = assign36340_e41801_d_n10;
        locals.var_t0_dn11 = assign36340_e41801_d_n11;
        locals.var_t0_dn14 = assign36340_e41801_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign36350_e41811, assign36350_e41811_d_n0, assign36350_e41811_d_n2, assign36350_e41811_d_n4, assign36350_e41811_d_n5, assign36350_e41811_d_n6, assign36350_e41811_d_n7, assign36350_e41811_d_n8, assign36350_e41811_d_n9, assign36350_e41811_d_n10, assign36350_e41811_d_n11, assign36350_e41811_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard823 != 0.0)) {
        let assign36350_e41809: f64 = (locals.var_cox * locals.var_t0);
        (assign36350_e41809, ((locals.var_cox_dn0 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn0)), ((locals.var_cox_dn2 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn2)), ((locals.var_cox_dn4 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn4)), ((locals.var_cox_dn5 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn5)), ((locals.var_cox_dn6 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn6)), ((locals.var_cox_dn7 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn7)), ((locals.var_cox_dn8 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn8)), ((locals.var_cox_dn9 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn9)), ((locals.var_cox_dn10 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn10)), ((locals.var_cox_dn11 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn11)), ((locals.var_cox_dn14 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign36350_e41811;
        locals.var_t3_dn0 = assign36350_e41811_d_n0;
        locals.var_t3_dn2 = assign36350_e41811_d_n2;
        locals.var_t3_dn4 = assign36350_e41811_d_n4;
        locals.var_t3_dn5 = assign36350_e41811_d_n5;
        locals.var_t3_dn6 = assign36350_e41811_d_n6;
        locals.var_t3_dn7 = assign36350_e41811_d_n7;
        locals.var_t3_dn8 = assign36350_e41811_d_n8;
        locals.var_t3_dn9 = assign36350_e41811_d_n9;
        locals.var_t3_dn10 = assign36350_e41811_d_n10;
        locals.var_t3_dn11 = assign36350_e41811_d_n11;
        locals.var_t3_dn14 = assign36350_e41811_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign36360_e41821, assign36360_e41821_d_n0, assign36360_e41821_d_n2, assign36360_e41821_d_n4, assign36360_e41821_d_n5, assign36360_e41821_d_n6, assign36360_e41821_d_n7, assign36360_e41821_d_n8, assign36360_e41821_d_n9, assign36360_e41821_d_n10, assign36360_e41821_d_n11, assign36360_e41821_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard823 != 0.0)) {
        let assign36360_e41819: f64 = (locals.var_t2).powf(p.p284);
        (assign36360_e41819, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn0)) } } else { (assign36360_e41819 * (p.p284 * (locals.var_t2_dn0 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn2)) } } else { (assign36360_e41819 * (p.p284 * (locals.var_t2_dn2 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn4)) } } else { (assign36360_e41819 * (p.p284 * (locals.var_t2_dn4 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn5)) } } else { (assign36360_e41819 * (p.p284 * (locals.var_t2_dn5 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn6)) } } else { (assign36360_e41819 * (p.p284 * (locals.var_t2_dn6 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn7)) } } else { (assign36360_e41819 * (p.p284 * (locals.var_t2_dn7 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn8)) } } else { (assign36360_e41819 * (p.p284 * (locals.var_t2_dn8 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn9)) } } else { (assign36360_e41819 * (p.p284 * (locals.var_t2_dn9 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn10)) } } else { (assign36360_e41819 * (p.p284 * (locals.var_t2_dn10 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn11)) } } else { (assign36360_e41819 * (p.p284 * (locals.var_t2_dn11 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn14)) } } else { (assign36360_e41819 * (p.p284 * (locals.var_t2_dn14 / locals.var_t2))) },)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign36360_e41821;
        locals.var_t0_dn0 = assign36360_e41821_d_n0;
        locals.var_t0_dn2 = assign36360_e41821_d_n2;
        locals.var_t0_dn4 = assign36360_e41821_d_n4;
        locals.var_t0_dn5 = assign36360_e41821_d_n5;
        locals.var_t0_dn6 = assign36360_e41821_d_n6;
        locals.var_t0_dn7 = assign36360_e41821_d_n7;
        locals.var_t0_dn8 = assign36360_e41821_d_n8;
        locals.var_t0_dn9 = assign36360_e41821_d_n9;
        locals.var_t0_dn10 = assign36360_e41821_d_n10;
        locals.var_t0_dn11 = assign36360_e41821_d_n11;
        locals.var_t0_dn14 = assign36360_e41821_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign36370_e41831, assign36370_e41831_d_n0, assign36370_e41831_d_n2, assign36370_e41831_d_n4, assign36370_e41831_d_n5, assign36370_e41831_d_n6, assign36370_e41831_d_n7, assign36370_e41831_d_n8, assign36370_e41831_d_n9, assign36370_e41831_d_n10, assign36370_e41831_d_n11, assign36370_e41831_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard823 != 0.0)) {
        let assign36370_e41829: f64 = (locals.var_t3 * locals.var_t0);
        (assign36370_e41829, ((locals.var_t3_dn0 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn0)), ((locals.var_t3_dn2 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn2)), ((locals.var_t3_dn4 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn4)), ((locals.var_t3_dn5 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn5)), ((locals.var_t3_dn6 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn6)), ((locals.var_t3_dn7 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn7)), ((locals.var_t3_dn8 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn8)), ((locals.var_t3_dn9 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn9)), ((locals.var_t3_dn10 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn10)), ((locals.var_t3_dn11 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn11)), ((locals.var_t3_dn14 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign36370_e41831;
        locals.var_t9_dn0 = assign36370_e41831_d_n0;
        locals.var_t9_dn2 = assign36370_e41831_d_n2;
        locals.var_t9_dn4 = assign36370_e41831_d_n4;
        locals.var_t9_dn5 = assign36370_e41831_d_n5;
        locals.var_t9_dn6 = assign36370_e41831_d_n6;
        locals.var_t9_dn7 = assign36370_e41831_d_n7;
        locals.var_t9_dn8 = assign36370_e41831_d_n8;
        locals.var_t9_dn9 = assign36370_e41831_d_n9;
        locals.var_t9_dn10 = assign36370_e41831_d_n10;
        locals.var_t9_dn11 = assign36370_e41831_d_n11;
        locals.var_t9_dn14 = assign36370_e41831_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign36380_e41843, assign36380_e41843_d_n0, assign36380_e41843_d_n2, assign36380_e41843_d_n4, assign36380_e41843_d_n5, assign36380_e41843_d_n6, assign36380_e41843_d_n7, assign36380_e41843_d_n8, assign36380_e41843_d_n9, assign36380_e41843_d_n10, assign36380_e41843_d_n11, assign36380_e41843_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard823 != 0.0)) {
        let assign36380_e41840: f64 = (locals.var_vdsz__blk441 * p.p285);
        let assign36380_e41841: f64 = (1.0 + assign36380_e41840);
        (assign36380_e41841, (locals.var_vdsz__blk441_dn0 * p.p285), (locals.var_vdsz__blk441_dn2 * p.p285), (locals.var_vdsz__blk441_dn4 * p.p285), (locals.var_vdsz__blk441_dn5 * p.p285), (locals.var_vdsz__blk441_dn6 * p.p285), (locals.var_vdsz__blk441_dn7 * p.p285), (locals.var_vdsz__blk441_dn8 * p.p285), (locals.var_vdsz__blk441_dn9 * p.p285), (locals.var_vdsz__blk441_dn10 * p.p285), (locals.var_vdsz__blk441_dn11 * p.p285), (locals.var_vdsz__blk441_dn14 * p.p285),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign36380_e41843;
        locals.var_t4_dn0 = assign36380_e41843_d_n0;
        locals.var_t4_dn2 = assign36380_e41843_d_n2;
        locals.var_t4_dn4 = assign36380_e41843_d_n4;
        locals.var_t4_dn5 = assign36380_e41843_d_n5;
        locals.var_t4_dn6 = assign36380_e41843_d_n6;
        locals.var_t4_dn7 = assign36380_e41843_d_n7;
        locals.var_t4_dn8 = assign36380_e41843_d_n8;
        locals.var_t4_dn9 = assign36380_e41843_d_n9;
        locals.var_t4_dn10 = assign36380_e41843_d_n10;
        locals.var_t4_dn11 = assign36380_e41843_d_n11;
        locals.var_t4_dn14 = assign36380_e41843_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign36390_e41851, assign36390_e41851_d_n0, assign36390_e41851_d_n2, assign36390_e41851_d_n4, assign36390_e41851_d_n5, assign36390_e41851_d_n6, assign36390_e41851_d_n7, assign36390_e41851_d_n8, assign36390_e41851_d_n9, assign36390_e41851_d_n10, assign36390_e41851_d_n11, assign36390_e41851_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard823 != 0.0)) {
        (locals.var_pt40, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign36390_e41851;
        locals.var_t0_dn0 = assign36390_e41851_d_n0;
        locals.var_t0_dn2 = assign36390_e41851_d_n2;
        locals.var_t0_dn4 = assign36390_e41851_d_n4;
        locals.var_t0_dn5 = assign36390_e41851_d_n5;
        locals.var_t0_dn6 = assign36390_e41851_d_n6;
        locals.var_t0_dn7 = assign36390_e41851_d_n7;
        locals.var_t0_dn8 = assign36390_e41851_d_n8;
        locals.var_t0_dn9 = assign36390_e41851_d_n9;
        locals.var_t0_dn10 = assign36390_e41851_d_n10;
        locals.var_t0_dn11 = assign36390_e41851_d_n11;
        locals.var_t0_dn14 = assign36390_e41851_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign36400_e41863, assign36400_e41863_d_n0, assign36400_e41863_d_n2, assign36400_e41863_d_n4, assign36400_e41863_d_n5, assign36400_e41863_d_n6, assign36400_e41863_d_n7, assign36400_e41863_d_n8, assign36400_e41863_d_n9, assign36400_e41863_d_n10, assign36400_e41863_d_n11, assign36400_e41863_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard823 != 0.0)) {
        let assign36400_e41859: f64 = (locals.var_phi_s0_dep + locals.var_t6);
        let assign36400_e41861: f64 = (assign36400_e41859 - locals.var_vbsz__blk440);
        (assign36400_e41861, ((locals.var_phi_s0_dep_dn0 + locals.var_t6_dn0) - locals.var_vbsz__blk440_dn0), ((locals.var_phi_s0_dep_dn2 + locals.var_t6_dn2) - locals.var_vbsz__blk440_dn2), ((locals.var_phi_s0_dep_dn4 + locals.var_t6_dn4) - locals.var_vbsz__blk440_dn4), ((locals.var_phi_s0_dep_dn5 + locals.var_t6_dn5) - locals.var_vbsz__blk440_dn5), ((locals.var_phi_s0_dep_dn6 + locals.var_t6_dn6) - locals.var_vbsz__blk440_dn6), ((locals.var_phi_s0_dep_dn7 + locals.var_t6_dn7) - locals.var_vbsz__blk440_dn7), ((locals.var_phi_s0_dep_dn8 + locals.var_t6_dn8) - locals.var_vbsz__blk440_dn8), ((locals.var_phi_s0_dep_dn9 + locals.var_t6_dn9) - locals.var_vbsz__blk440_dn9), ((locals.var_phi_s0_dep_dn10 + locals.var_t6_dn10) - locals.var_vbsz__blk440_dn10), ((locals.var_phi_s0_dep_dn11 + locals.var_t6_dn11) - locals.var_vbsz__blk440_dn11), ((locals.var_phi_s0_dep_dn14 + locals.var_t6_dn14) - locals.var_vbsz__blk440_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign36400_e41863;
        locals.var_t5_dn0 = assign36400_e41863_d_n0;
        locals.var_t5_dn2 = assign36400_e41863_d_n2;
        locals.var_t5_dn4 = assign36400_e41863_d_n4;
        locals.var_t5_dn5 = assign36400_e41863_d_n5;
        locals.var_t5_dn6 = assign36400_e41863_d_n6;
        locals.var_t5_dn7 = assign36400_e41863_d_n7;
        locals.var_t5_dn8 = assign36400_e41863_d_n8;
        locals.var_t5_dn9 = assign36400_e41863_d_n9;
        locals.var_t5_dn10 = assign36400_e41863_d_n10;
        locals.var_t5_dn11 = assign36400_e41863_d_n11;
        locals.var_t5_dn14 = assign36400_e41863_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign36410_e41877, assign36410_e41877_d_n0, assign36410_e41877_d_n2, assign36410_e41877_d_n4, assign36410_e41877_d_n5, assign36410_e41877_d_n6, assign36410_e41877_d_n7, assign36410_e41877_d_n8, assign36410_e41877_d_n9, assign36410_e41877_d_n10, assign36410_e41877_d_n11, assign36410_e41877_d_n14,) = {
    if (((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) && (locals.var_guard823 != 0.0)) {
        let assign36410_e41872: f64 = (locals.var_vdsz__blk441 * locals.var_t0);
        let assign36410_e41874: f64 = (assign36410_e41872 * locals.var_t5);
        let assign36410_e41875: f64 = (locals.var_t4 + assign36410_e41874);
        (assign36410_e41875, (locals.var_t4_dn0 + ((((locals.var_vdsz__blk441_dn0 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn0)) * locals.var_t5) + (assign36410_e41872 * locals.var_t5_dn0))), (locals.var_t4_dn2 + ((((locals.var_vdsz__blk441_dn2 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn2)) * locals.var_t5) + (assign36410_e41872 * locals.var_t5_dn2))), (locals.var_t4_dn4 + ((((locals.var_vdsz__blk441_dn4 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn4)) * locals.var_t5) + (assign36410_e41872 * locals.var_t5_dn4))), (locals.var_t4_dn5 + ((((locals.var_vdsz__blk441_dn5 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn5)) * locals.var_t5) + (assign36410_e41872 * locals.var_t5_dn5))), (locals.var_t4_dn6 + ((((locals.var_vdsz__blk441_dn6 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn6)) * locals.var_t5) + (assign36410_e41872 * locals.var_t5_dn6))), (locals.var_t4_dn7 + ((((locals.var_vdsz__blk441_dn7 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn7)) * locals.var_t5) + (assign36410_e41872 * locals.var_t5_dn7))), (locals.var_t4_dn8 + ((((locals.var_vdsz__blk441_dn8 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn8)) * locals.var_t5) + (assign36410_e41872 * locals.var_t5_dn8))), (locals.var_t4_dn9 + ((((locals.var_vdsz__blk441_dn9 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn9)) * locals.var_t5) + (assign36410_e41872 * locals.var_t5_dn9))), (locals.var_t4_dn10 + ((((locals.var_vdsz__blk441_dn10 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn10)) * locals.var_t5) + (assign36410_e41872 * locals.var_t5_dn10))), (locals.var_t4_dn11 + ((((locals.var_vdsz__blk441_dn11 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn11)) * locals.var_t5) + (assign36410_e41872 * locals.var_t5_dn11))), (locals.var_t4_dn14 + ((((locals.var_vdsz__blk441_dn14 * locals.var_t0) + (locals.var_vdsz__blk441 * locals.var_t0_dn14)) * locals.var_t5) + (assign36410_e41872 * locals.var_t5_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign36410_e41877;
        locals.var_t4_dn0 = assign36410_e41877_d_n0;
        locals.var_t4_dn2 = assign36410_e41877_d_n2;
        locals.var_t4_dn4 = assign36410_e41877_d_n4;
        locals.var_t4_dn5 = assign36410_e41877_d_n5;
        locals.var_t4_dn6 = assign36410_e41877_d_n6;
        locals.var_t4_dn7 = assign36410_e41877_d_n7;
        locals.var_t4_dn8 = assign36410_e41877_d_n8;
        locals.var_t4_dn9 = assign36410_e41877_d_n9;
        locals.var_t4_dn10 = assign36410_e41877_d_n10;
        locals.var_t4_dn11 = assign36410_e41877_d_n11;
        locals.var_t4_dn14 = assign36410_e41877_d_n14;
        locals.var_t4_rv = 0.0;

    }
}

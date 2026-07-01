#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_32(
        locals: &mut StampLocals,
    ) {
        let (assign11170_e12860,) = {
    if (locals.var_guard113 != 0.0) {
        let (assign11170_e12858,) = {
            if (locals.var_flg_brk8 > 0.0) {
                (locals.var_flg_brk8,)
            } else {
                (locals.var_lp_s0,)
            }
        };
        (assign11170_e12858,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign11170_e12860;

        let assign11180_e12863: f64 = if locals.var_flg_conv == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard303 = assign11180_e12863;

        let (assign11190_e12869, assign11190_e12869_d_n0, assign11190_e12869_d_n2, assign11190_e12869_d_n6, assign11190_e12869_d_n7, assign11190_e12869_d_n10, assign11190_e12869_d_n11, assign11190_e12869_d_n12, assign11190_e12869_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard303 != 0.0)) {
        (locals.var_phi_s0_soi_ini, locals.var_phi_s0_soi_ini_dn0, locals.var_phi_s0_soi_ini_dn2, locals.var_phi_s0_soi_ini_dn6, locals.var_phi_s0_soi_ini_dn7, locals.var_phi_s0_soi_ini_dn10, locals.var_phi_s0_soi_ini_dn11, locals.var_phi_s0_soi_ini_dn12, locals.var_phi_s0_soi_ini_dn17,)
    } else {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    }
};
        locals.var_phi_s0_soi = assign11190_e12869;
        locals.var_phi_s0_soi_dn0 = assign11190_e12869_d_n0;
        locals.var_phi_s0_soi_dn2 = assign11190_e12869_d_n2;
        locals.var_phi_s0_soi_dn6 = assign11190_e12869_d_n6;
        locals.var_phi_s0_soi_dn7 = assign11190_e12869_d_n7;
        locals.var_phi_s0_soi_dn10 = assign11190_e12869_d_n10;
        locals.var_phi_s0_soi_dn11 = assign11190_e12869_d_n11;
        locals.var_phi_s0_soi_dn12 = assign11190_e12869_d_n12;
        locals.var_phi_s0_soi_dn17 = assign11190_e12869_d_n17;

        let (assign11200_e12875, assign11200_e12875_d_n0, assign11200_e12875_d_n2, assign11200_e12875_d_n6, assign11200_e12875_d_n7, assign11200_e12875_d_n10, assign11200_e12875_d_n11, assign11200_e12875_d_n12, assign11200_e12875_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard303 != 0.0)) {
        (locals.var_phi_b0_soi_ini, locals.var_phi_b0_soi_ini_dn0, locals.var_phi_b0_soi_ini_dn2, locals.var_phi_b0_soi_ini_dn6, locals.var_phi_b0_soi_ini_dn7, locals.var_phi_b0_soi_ini_dn10, locals.var_phi_b0_soi_ini_dn11, locals.var_phi_b0_soi_ini_dn12, locals.var_phi_b0_soi_ini_dn17,)
    } else {
        (locals.var_phi_b0_soi, locals.var_phi_b0_soi_dn0, locals.var_phi_b0_soi_dn2, locals.var_phi_b0_soi_dn6, locals.var_phi_b0_soi_dn7, locals.var_phi_b0_soi_dn10, locals.var_phi_b0_soi_dn11, locals.var_phi_b0_soi_dn12, locals.var_phi_b0_soi_dn17,)
    }
};
        locals.var_phi_b0_soi = assign11200_e12875;
        locals.var_phi_b0_soi_dn0 = assign11200_e12875_d_n0;
        locals.var_phi_b0_soi_dn2 = assign11200_e12875_d_n2;
        locals.var_phi_b0_soi_dn6 = assign11200_e12875_d_n6;
        locals.var_phi_b0_soi_dn7 = assign11200_e12875_d_n7;
        locals.var_phi_b0_soi_dn10 = assign11200_e12875_d_n10;
        locals.var_phi_b0_soi_dn11 = assign11200_e12875_d_n11;
        locals.var_phi_b0_soi_dn12 = assign11200_e12875_d_n12;
        locals.var_phi_b0_soi_dn17 = assign11200_e12875_d_n17;

        let (assign11210_e12881, assign11210_e12881_d_n0, assign11210_e12881_d_n2, assign11210_e12881_d_n6, assign11210_e12881_d_n7, assign11210_e12881_d_n10, assign11210_e12881_d_n11, assign11210_e12881_d_n12, assign11210_e12881_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard303 != 0.0)) {
        (locals.var_phi_s0_bulk_ini, locals.var_phi_s0_bulk_ini_dn0, locals.var_phi_s0_bulk_ini_dn2, locals.var_phi_s0_bulk_ini_dn6, locals.var_phi_s0_bulk_ini_dn7, locals.var_phi_s0_bulk_ini_dn10, locals.var_phi_s0_bulk_ini_dn11, locals.var_phi_s0_bulk_ini_dn12, locals.var_phi_s0_bulk_ini_dn17,)
    } else {
        (locals.var_phi_s0_bulk, locals.var_phi_s0_bulk_dn0, locals.var_phi_s0_bulk_dn2, locals.var_phi_s0_bulk_dn6, locals.var_phi_s0_bulk_dn7, locals.var_phi_s0_bulk_dn10, locals.var_phi_s0_bulk_dn11, locals.var_phi_s0_bulk_dn12, locals.var_phi_s0_bulk_dn17,)
    }
};
        locals.var_phi_s0_bulk = assign11210_e12881;
        locals.var_phi_s0_bulk_dn0 = assign11210_e12881_d_n0;
        locals.var_phi_s0_bulk_dn2 = assign11210_e12881_d_n2;
        locals.var_phi_s0_bulk_dn6 = assign11210_e12881_d_n6;
        locals.var_phi_s0_bulk_dn7 = assign11210_e12881_d_n7;
        locals.var_phi_s0_bulk_dn10 = assign11210_e12881_d_n10;
        locals.var_phi_s0_bulk_dn11 = assign11210_e12881_d_n11;
        locals.var_phi_s0_bulk_dn12 = assign11210_e12881_d_n12;
        locals.var_phi_s0_bulk_dn17 = assign11210_e12881_d_n17;

        let (assign11220_e12885, assign11220_e12885_d_n0, assign11220_e12885_d_n2, assign11220_e12885_d_n6, assign11220_e12885_d_n7, assign11220_e12885_d_n10, assign11220_e12885_d_n11, assign11220_e12885_d_n12, assign11220_e12885_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    } else {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    }
};
        locals.var_ps0 = assign11220_e12885;
        locals.var_ps0_dn0 = assign11220_e12885_d_n0;
        locals.var_ps0_dn2 = assign11220_e12885_d_n2;
        locals.var_ps0_dn6 = assign11220_e12885_d_n6;
        locals.var_ps0_dn7 = assign11220_e12885_d_n7;
        locals.var_ps0_dn10 = assign11220_e12885_d_n10;
        locals.var_ps0_dn11 = assign11220_e12885_d_n11;
        locals.var_ps0_dn12 = assign11220_e12885_d_n12;
        locals.var_ps0_dn17 = assign11220_e12885_d_n17;

        let (assign11230_e12890, assign11230_e12890_d_n0, assign11230_e12890_d_n2, assign11230_e12890_d_n6, assign11230_e12890_d_n7, assign11230_e12890_d_n10, assign11230_e12890_d_n11, assign11230_e12890_d_n12, assign11230_e12890_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign11230_e12888: f64 = (-locals.var_q_n0);
        (assign11230_e12888, (-locals.var_q_n0_dn0), (-locals.var_q_n0_dn2), (-locals.var_q_n0_dn6), (-locals.var_q_n0_dn7), (-locals.var_q_n0_dn10), (-locals.var_q_n0_dn11), (-locals.var_q_n0_dn12), (-locals.var_q_n0_dn17),)
    } else {
        (locals.var_qn0, locals.var_qn0_dn0, locals.var_qn0_dn2, locals.var_qn0_dn6, locals.var_qn0_dn7, locals.var_qn0_dn10, locals.var_qn0_dn11, locals.var_qn0_dn12, locals.var_qn0_dn17,)
    }
};
        locals.var_qn0 = assign11230_e12890;
        locals.var_qn0_dn0 = assign11230_e12890_d_n0;
        locals.var_qn0_dn2 = assign11230_e12890_d_n2;
        locals.var_qn0_dn6 = assign11230_e12890_d_n6;
        locals.var_qn0_dn7 = assign11230_e12890_d_n7;
        locals.var_qn0_dn10 = assign11230_e12890_d_n10;
        locals.var_qn0_dn11 = assign11230_e12890_d_n11;
        locals.var_qn0_dn12 = assign11230_e12890_d_n12;
        locals.var_qn0_dn17 = assign11230_e12890_d_n17;

        let assign11240_e12893: f64 = if locals.var_qn0 <= 1e-50 { 1.0 } else { 0.0 };
        locals.var_guard304 = assign11240_e12893;

        let (assign11250_e12899, assign11250_e12899_d_n0, assign11250_e12899_d_n2, assign11250_e12899_d_n6, assign11250_e12899_d_n7, assign11250_e12899_d_n10, assign11250_e12899_d_n11, assign11250_e12899_d_n12, assign11250_e12899_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard304 != 0.0)) {
        (1e-50, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qn0, locals.var_qn0_dn0, locals.var_qn0_dn2, locals.var_qn0_dn6, locals.var_qn0_dn7, locals.var_qn0_dn10, locals.var_qn0_dn11, locals.var_qn0_dn12, locals.var_qn0_dn17,)
    }
};
        locals.var_qn0 = assign11250_e12899;
        locals.var_qn0_dn0 = assign11250_e12899_d_n0;
        locals.var_qn0_dn2 = assign11250_e12899_d_n2;
        locals.var_qn0_dn6 = assign11250_e12899_d_n6;
        locals.var_qn0_dn7 = assign11250_e12899_d_n7;
        locals.var_qn0_dn10 = assign11250_e12899_d_n10;
        locals.var_qn0_dn11 = assign11250_e12899_d_n11;
        locals.var_qn0_dn12 = assign11250_e12899_d_n12;
        locals.var_qn0_dn17 = assign11250_e12899_d_n17;

        let (assign11270_e12909, assign11270_e12909_d_n0, assign11270_e12909_d_n2, assign11270_e12909_d_n6, assign11270_e12909_d_n7, assign11270_e12909_d_n10, assign11270_e12909_d_n11, assign11270_e12909_d_n12, assign11270_e12909_d_n17,) = {
    if (locals.var_guard113 != 0.0) {
        let assign11270_e12907: f64 = (locals.var_qn0 * locals.var_c_fox_inv);
        (assign11270_e12907, ((locals.var_qn0_dn0 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn0)), ((locals.var_qn0_dn2 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn2)), ((locals.var_qn0_dn6 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn6)), ((locals.var_qn0_dn7 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn7)), ((locals.var_qn0_dn10 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn10)), ((locals.var_qn0_dn11 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn11)), ((locals.var_qn0_dn12 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn12)), ((locals.var_qn0_dn17 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn17)),)
    } else {
        (locals.var_vgvt, locals.var_vgvt_dn0, locals.var_vgvt_dn2, locals.var_vgvt_dn6, locals.var_vgvt_dn7, locals.var_vgvt_dn10, locals.var_vgvt_dn11, locals.var_vgvt_dn12, locals.var_vgvt_dn17,)
    }
};
        locals.var_vgvt = assign11270_e12909;
        locals.var_vgvt_dn0 = assign11270_e12909_d_n0;
        locals.var_vgvt_dn2 = assign11270_e12909_d_n2;
        locals.var_vgvt_dn6 = assign11270_e12909_d_n6;
        locals.var_vgvt_dn7 = assign11270_e12909_d_n7;
        locals.var_vgvt_dn10 = assign11270_e12909_d_n10;
        locals.var_vgvt_dn11 = assign11270_e12909_d_n11;
        locals.var_vgvt_dn12 = assign11270_e12909_d_n12;
        locals.var_vgvt_dn17 = assign11270_e12909_d_n17;

        let assign11280_e12914: f64 = if ((locals.var_phi_s0_soi <= 0.0) && (locals.var_flg_skipacc != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard305 = assign11280_e12914;

        let (assign11300_e12929, assign11300_e12929_d_n0, assign11300_e12929_d_n2, assign11300_e12929_d_n6, assign11300_e12929_d_n7, assign11300_e12929_d_n10, assign11300_e12929_d_n11, assign11300_e12929_d_n12, assign11300_e12929_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 != 0.0)) {
        let assign11300_e12925: f64 = (-locals.var_weffcv_nf);
        let assign11300_e12927: f64 = (assign11300_e12925 * locals.var_leff_cv);
        (assign11300_e12927, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign11300_e12929;
        locals.var_t0_dn0 = assign11300_e12929_d_n0;
        locals.var_t0_dn2 = assign11300_e12929_d_n2;
        locals.var_t0_dn6 = assign11300_e12929_d_n6;
        locals.var_t0_dn7 = assign11300_e12929_d_n7;
        locals.var_t0_dn10 = assign11300_e12929_d_n10;
        locals.var_t0_dn11 = assign11300_e12929_d_n11;
        locals.var_t0_dn12 = assign11300_e12929_d_n12;
        locals.var_t0_dn17 = assign11300_e12929_d_n17;

        let (assign11310_e12935, assign11310_e12935_d_n0, assign11310_e12935_d_n2, assign11310_e12935_d_n6, assign11310_e12935_d_n7, assign11310_e12935_d_n10, assign11310_e12935_d_n11, assign11310_e12935_d_n12, assign11310_e12935_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 != 0.0)) {
        (locals.var_q_s0_dep_ini, locals.var_q_s0_dep_ini_dn0, locals.var_q_s0_dep_ini_dn2, locals.var_q_s0_dep_ini_dn6, locals.var_q_s0_dep_ini_dn7, locals.var_q_s0_dep_ini_dn10, locals.var_q_s0_dep_ini_dn11, locals.var_q_s0_dep_ini_dn12, locals.var_q_s0_dep_ini_dn17,)
    } else {
        (locals.var_q_sl_dep, locals.var_q_sl_dep_dn0, locals.var_q_sl_dep_dn2, locals.var_q_sl_dep_dn6, locals.var_q_sl_dep_dn7, locals.var_q_sl_dep_dn10, locals.var_q_sl_dep_dn11, locals.var_q_sl_dep_dn12, locals.var_q_sl_dep_dn17,)
    }
};
        locals.var_q_sl_dep = assign11310_e12935;
        locals.var_q_sl_dep_dn0 = assign11310_e12935_d_n0;
        locals.var_q_sl_dep_dn2 = assign11310_e12935_d_n2;
        locals.var_q_sl_dep_dn6 = assign11310_e12935_d_n6;
        locals.var_q_sl_dep_dn7 = assign11310_e12935_d_n7;
        locals.var_q_sl_dep_dn10 = assign11310_e12935_d_n10;
        locals.var_q_sl_dep_dn11 = assign11310_e12935_d_n11;
        locals.var_q_sl_dep_dn12 = assign11310_e12935_d_n12;
        locals.var_q_sl_dep_dn17 = assign11310_e12935_d_n17;

        let (assign11320_e12941, assign11320_e12941_d_n0, assign11320_e12941_d_n2, assign11320_e12941_d_n6, assign11320_e12941_d_n7, assign11320_e12941_d_n10, assign11320_e12941_d_n11, assign11320_e12941_d_n12, assign11320_e12941_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 != 0.0)) {
        (locals.var_q_b0_dep, locals.var_q_b0_dep_dn0, locals.var_q_b0_dep_dn2, locals.var_q_b0_dep_dn6, locals.var_q_b0_dep_dn7, locals.var_q_b0_dep_dn10, locals.var_q_b0_dep_dn11, locals.var_q_b0_dep_dn12, locals.var_q_b0_dep_dn17,)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn12, locals.var_q_bl_dep_dn17,)
    }
};
        locals.var_q_bl_dep = assign11320_e12941;
        locals.var_q_bl_dep_dn0 = assign11320_e12941_d_n0;
        locals.var_q_bl_dep_dn2 = assign11320_e12941_d_n2;
        locals.var_q_bl_dep_dn6 = assign11320_e12941_d_n6;
        locals.var_q_bl_dep_dn7 = assign11320_e12941_d_n7;
        locals.var_q_bl_dep_dn10 = assign11320_e12941_d_n10;
        locals.var_q_bl_dep_dn11 = assign11320_e12941_d_n11;
        locals.var_q_bl_dep_dn12 = assign11320_e12941_d_n12;
        locals.var_q_bl_dep_dn17 = assign11320_e12941_d_n17;

        let (assign11330_e12949, assign11330_e12949_d_n0, assign11330_e12949_d_n2, assign11330_e12949_d_n6, assign11330_e12949_d_n7, assign11330_e12949_d_n10, assign11330_e12949_d_n11, assign11330_e12949_d_n12, assign11330_e12949_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 != 0.0)) {
        let assign11330_e12947: f64 = (locals.var_q_sl_dep + locals.var_q_bl_dep);
        (assign11330_e12947, (locals.var_q_sl_dep_dn0 + locals.var_q_bl_dep_dn0), (locals.var_q_sl_dep_dn2 + locals.var_q_bl_dep_dn2), (locals.var_q_sl_dep_dn6 + locals.var_q_bl_dep_dn6), (locals.var_q_sl_dep_dn7 + locals.var_q_bl_dep_dn7), (locals.var_q_sl_dep_dn10 + locals.var_q_bl_dep_dn10), (locals.var_q_sl_dep_dn11 + locals.var_q_bl_dep_dn11), (locals.var_q_sl_dep_dn12 + locals.var_q_bl_dep_dn12), (locals.var_q_sl_dep_dn17 + locals.var_q_bl_dep_dn17),)
    } else {
        (locals.var_q_depl, locals.var_q_depl_dn0, locals.var_q_depl_dn2, locals.var_q_depl_dn6, locals.var_q_depl_dn7, locals.var_q_depl_dn10, locals.var_q_depl_dn11, locals.var_q_depl_dn12, locals.var_q_depl_dn17,)
    }
};
        locals.var_q_depl = assign11330_e12949;
        locals.var_q_depl_dn0 = assign11330_e12949_d_n0;
        locals.var_q_depl_dn2 = assign11330_e12949_d_n2;
        locals.var_q_depl_dn6 = assign11330_e12949_d_n6;
        locals.var_q_depl_dn7 = assign11330_e12949_d_n7;
        locals.var_q_depl_dn10 = assign11330_e12949_d_n10;
        locals.var_q_depl_dn11 = assign11330_e12949_d_n11;
        locals.var_q_depl_dn12 = assign11330_e12949_d_n12;
        locals.var_q_depl_dn17 = assign11330_e12949_d_n17;

        let (assign11340_e12960, assign11340_e12960_d_n0, assign11340_e12960_d_n2, assign11340_e12960_d_n6, assign11340_e12960_d_n7, assign11340_e12960_d_n10, assign11340_e12960_d_n11, assign11340_e12960_d_n12, assign11340_e12960_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 != 0.0)) {
        let assign11340_e12954: f64 = (-0.5);
        let assign11340_e12957: f64 = (locals.var_q_depl + locals.var_q_dep0);
        let assign11340_e12958: f64 = (assign11340_e12954 * assign11340_e12957);
        (assign11340_e12958, (assign11340_e12954 * (locals.var_q_depl_dn0 + locals.var_q_dep0_dn0)), (assign11340_e12954 * (locals.var_q_depl_dn2 + locals.var_q_dep0_dn2)), (assign11340_e12954 * (locals.var_q_depl_dn6 + locals.var_q_dep0_dn6)), (assign11340_e12954 * (locals.var_q_depl_dn7 + locals.var_q_dep0_dn7)), (assign11340_e12954 * (locals.var_q_depl_dn10 + locals.var_q_dep0_dn10)), (assign11340_e12954 * (locals.var_q_depl_dn11 + locals.var_q_dep0_dn11)), (assign11340_e12954 * (locals.var_q_depl_dn12 + locals.var_q_dep0_dn12)), (assign11340_e12954 * (locals.var_q_depl_dn17 + locals.var_q_dep0_dn17)),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign11340_e12960;
        locals.var_qbu_dn0 = assign11340_e12960_d_n0;
        locals.var_qbu_dn2 = assign11340_e12960_d_n2;
        locals.var_qbu_dn6 = assign11340_e12960_d_n6;
        locals.var_qbu_dn7 = assign11340_e12960_d_n7;
        locals.var_qbu_dn10 = assign11340_e12960_d_n10;
        locals.var_qbu_dn11 = assign11340_e12960_d_n11;
        locals.var_qbu_dn12 = assign11340_e12960_d_n12;
        locals.var_qbu_dn17 = assign11340_e12960_d_n17;

        let (assign11350_e12968, assign11350_e12968_d_n0, assign11350_e12968_d_n2, assign11350_e12968_d_n6, assign11350_e12968_d_n7, assign11350_e12968_d_n10, assign11350_e12968_d_n11, assign11350_e12968_d_n12, assign11350_e12968_d_n13, assign11350_e12968_d_n15, assign11350_e12968_d_n16, assign11350_e12968_d_n17, assign11350_e12968_d_n18,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 != 0.0)) {
        let assign11350_e12966: f64 = (locals.var_t0 * locals.var_qbu);
        (assign11350_e12966, ((locals.var_t0_dn0 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn0)), ((locals.var_t0_dn2 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn2)), ((locals.var_t0_dn6 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn6)), ((locals.var_t0_dn7 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn7)), ((locals.var_t0_dn10 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn10)), ((locals.var_t0_dn11 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn11)), ((locals.var_t0_dn12 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn12)), 0.0, 0.0, 0.0, ((locals.var_t0_dn17 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn17)), 0.0,)
    } else {
        (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn12, locals.var_qb_dn13, locals.var_qb_dn15, locals.var_qb_dn16, locals.var_qb_dn17, locals.var_qb_dn18,)
    }
};
        locals.var_qb = assign11350_e12968;
        locals.var_qb_dn0 = assign11350_e12968_d_n0;
        locals.var_qb_dn2 = assign11350_e12968_d_n2;
        locals.var_qb_dn6 = assign11350_e12968_d_n6;
        locals.var_qb_dn7 = assign11350_e12968_d_n7;
        locals.var_qb_dn10 = assign11350_e12968_d_n10;
        locals.var_qb_dn11 = assign11350_e12968_d_n11;
        locals.var_qb_dn12 = assign11350_e12968_d_n12;
        locals.var_qb_dn13 = assign11350_e12968_d_n13;
        locals.var_qb_dn15 = assign11350_e12968_d_n15;
        locals.var_qb_dn16 = assign11350_e12968_d_n16;
        locals.var_qb_dn17 = assign11350_e12968_d_n17;
        locals.var_qb_dn18 = assign11350_e12968_d_n18;

        let (assign11360_e12976, assign11360_e12976_d_n0, assign11360_e12976_d_n2, assign11360_e12976_d_n6, assign11360_e12976_d_n7, assign11360_e12976_d_n10, assign11360_e12976_d_n11, assign11360_e12976_d_n12, assign11360_e12976_d_n13, assign11360_e12976_d_n15, assign11360_e12976_d_n16, assign11360_e12976_d_n17, assign11360_e12976_d_n18,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 != 0.0)) {
        let assign11360_e12974: f64 = (locals.var_qb * 0.5);
        (assign11360_e12974, (locals.var_qb_dn0 * 0.5), (locals.var_qb_dn2 * 0.5), (locals.var_qb_dn6 * 0.5), (locals.var_qb_dn7 * 0.5), (locals.var_qb_dn10 * 0.5), (locals.var_qb_dn11 * 0.5), (locals.var_qb_dn12 * 0.5), (locals.var_qb_dn13 * 0.5), (locals.var_qb_dn15 * 0.5), (locals.var_qb_dn16 * 0.5), (locals.var_qb_dn17 * 0.5), (locals.var_qb_dn18 * 0.5),)
    } else {
        (locals.var_qd_fb, locals.var_qd_fb_dn0, locals.var_qd_fb_dn2, locals.var_qd_fb_dn6, locals.var_qd_fb_dn7, locals.var_qd_fb_dn10, locals.var_qd_fb_dn11, locals.var_qd_fb_dn12, locals.var_qd_fb_dn13, locals.var_qd_fb_dn15, locals.var_qd_fb_dn16, locals.var_qd_fb_dn17, locals.var_qd_fb_dn18,)
    }
};
        locals.var_qd_fb = assign11360_e12976;
        locals.var_qd_fb_dn0 = assign11360_e12976_d_n0;
        locals.var_qd_fb_dn2 = assign11360_e12976_d_n2;
        locals.var_qd_fb_dn6 = assign11360_e12976_d_n6;
        locals.var_qd_fb_dn7 = assign11360_e12976_d_n7;
        locals.var_qd_fb_dn10 = assign11360_e12976_d_n10;
        locals.var_qd_fb_dn11 = assign11360_e12976_d_n11;
        locals.var_qd_fb_dn12 = assign11360_e12976_d_n12;
        locals.var_qd_fb_dn13 = assign11360_e12976_d_n13;
        locals.var_qd_fb_dn15 = assign11360_e12976_d_n15;
        locals.var_qd_fb_dn16 = assign11360_e12976_d_n16;
        locals.var_qd_fb_dn17 = assign11360_e12976_d_n17;
        locals.var_qd_fb_dn18 = assign11360_e12976_d_n18;

        let (assign11370_e12986, assign11370_e12986_d_n0, assign11370_e12986_d_n2, assign11370_e12986_d_n6, assign11370_e12986_d_n7, assign11370_e12986_d_n10, assign11370_e12986_d_n11, assign11370_e12986_d_n12, assign11370_e12986_d_n13, assign11370_e12986_d_n15, assign11370_e12986_d_n16, assign11370_e12986_d_n17, assign11370_e12986_d_n18,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 != 0.0)) {
        let assign11370_e12983: f64 = (1.0 - 0.5);
        let assign11370_e12984: f64 = (locals.var_qb * assign11370_e12983);
        (assign11370_e12984, (locals.var_qb_dn0 * assign11370_e12983), (locals.var_qb_dn2 * assign11370_e12983), (locals.var_qb_dn6 * assign11370_e12983), (locals.var_qb_dn7 * assign11370_e12983), (locals.var_qb_dn10 * assign11370_e12983), (locals.var_qb_dn11 * assign11370_e12983), (locals.var_qb_dn12 * assign11370_e12983), (locals.var_qb_dn13 * assign11370_e12983), (locals.var_qb_dn15 * assign11370_e12983), (locals.var_qb_dn16 * assign11370_e12983), (locals.var_qb_dn17 * assign11370_e12983), (locals.var_qb_dn18 * assign11370_e12983),)
    } else {
        (locals.var_qs_fb, locals.var_qs_fb_dn0, locals.var_qs_fb_dn2, locals.var_qs_fb_dn6, locals.var_qs_fb_dn7, locals.var_qs_fb_dn10, locals.var_qs_fb_dn11, locals.var_qs_fb_dn12, locals.var_qs_fb_dn13, locals.var_qs_fb_dn15, locals.var_qs_fb_dn16, locals.var_qs_fb_dn17, locals.var_qs_fb_dn18,)
    }
};
        locals.var_qs_fb = assign11370_e12986;
        locals.var_qs_fb_dn0 = assign11370_e12986_d_n0;
        locals.var_qs_fb_dn2 = assign11370_e12986_d_n2;
        locals.var_qs_fb_dn6 = assign11370_e12986_d_n6;
        locals.var_qs_fb_dn7 = assign11370_e12986_d_n7;
        locals.var_qs_fb_dn10 = assign11370_e12986_d_n10;
        locals.var_qs_fb_dn11 = assign11370_e12986_d_n11;
        locals.var_qs_fb_dn12 = assign11370_e12986_d_n12;
        locals.var_qs_fb_dn13 = assign11370_e12986_d_n13;
        locals.var_qs_fb_dn15 = assign11370_e12986_d_n15;
        locals.var_qs_fb_dn16 = assign11370_e12986_d_n16;
        locals.var_qs_fb_dn17 = assign11370_e12986_d_n17;
        locals.var_qs_fb_dn18 = assign11370_e12986_d_n18;

        let (assign11380_e12992, assign11380_e12992_d_n0, assign11380_e12992_d_n2, assign11380_e12992_d_n6, assign11380_e12992_d_n7, assign11380_e12992_d_n10, assign11380_e12992_d_n11, assign11380_e12992_d_n12, assign11380_e12992_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qi, locals.var_qi_dn0, locals.var_qi_dn2, locals.var_qi_dn6, locals.var_qi_dn7, locals.var_qi_dn10, locals.var_qi_dn11, locals.var_qi_dn12, locals.var_qi_dn17,)
    }
};
        locals.var_qi = assign11380_e12992;
        locals.var_qi_dn0 = assign11380_e12992_d_n0;
        locals.var_qi_dn2 = assign11380_e12992_d_n2;
        locals.var_qi_dn6 = assign11380_e12992_d_n6;
        locals.var_qi_dn7 = assign11380_e12992_d_n7;
        locals.var_qi_dn10 = assign11380_e12992_d_n10;
        locals.var_qi_dn11 = assign11380_e12992_d_n11;
        locals.var_qi_dn12 = assign11380_e12992_d_n12;
        locals.var_qi_dn17 = assign11380_e12992_d_n17;

        let (assign11390_e13002, assign11390_e13002_d_n0, assign11390_e13002_d_n2, assign11390_e13002_d_n6, assign11390_e13002_d_n7, assign11390_e13002_d_n10, assign11390_e13002_d_n11, assign11390_e13002_d_n12, assign11390_e13002_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 != 0.0)) {
        let assign11390_e12998: f64 = (locals.var_q_s0_bulk * locals.var_leff_cv);
        let assign11390_e13000: f64 = (assign11390_e12998 * locals.var_weffcv_nf);
        (assign11390_e13000, ((locals.var_q_s0_bulk_dn0 * locals.var_leff_cv) * locals.var_weffcv_nf), ((locals.var_q_s0_bulk_dn2 * locals.var_leff_cv) * locals.var_weffcv_nf), ((locals.var_q_s0_bulk_dn6 * locals.var_leff_cv) * locals.var_weffcv_nf), ((locals.var_q_s0_bulk_dn7 * locals.var_leff_cv) * locals.var_weffcv_nf), ((locals.var_q_s0_bulk_dn10 * locals.var_leff_cv) * locals.var_weffcv_nf), ((locals.var_q_s0_bulk_dn11 * locals.var_leff_cv) * locals.var_weffcv_nf), ((locals.var_q_s0_bulk_dn12 * locals.var_leff_cv) * locals.var_weffcv_nf), ((locals.var_q_s0_bulk_dn17 * locals.var_leff_cv) * locals.var_weffcv_nf),)
    } else {
        (locals.var_qsub, locals.var_qsub_dn0, locals.var_qsub_dn2, locals.var_qsub_dn6, locals.var_qsub_dn7, locals.var_qsub_dn10, locals.var_qsub_dn11, locals.var_qsub_dn12, locals.var_qsub_dn17,)
    }
};
        locals.var_qsub = assign11390_e13002;
        locals.var_qsub_dn0 = assign11390_e13002_d_n0;
        locals.var_qsub_dn2 = assign11390_e13002_d_n2;
        locals.var_qsub_dn6 = assign11390_e13002_d_n6;
        locals.var_qsub_dn7 = assign11390_e13002_d_n7;
        locals.var_qsub_dn10 = assign11390_e13002_d_n10;
        locals.var_qsub_dn11 = assign11390_e13002_d_n11;
        locals.var_qsub_dn12 = assign11390_e13002_d_n12;
        locals.var_qsub_dn17 = assign11390_e13002_d_n17;

        let (assign11400_e13008, assign11400_e13008_d_n0, assign11400_e13008_d_n2, assign11400_e13008_d_n6, assign11400_e13008_d_n7, assign11400_e13008_d_n10, assign11400_e13008_d_n11, assign11400_e13008_d_n12, assign11400_e13008_d_n13, assign11400_e13008_d_n15, assign11400_e13008_d_n16, assign11400_e13008_d_n17, assign11400_e13008_d_n18,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18,)
    }
};
        locals.var_qd = assign11400_e13008;
        locals.var_qd_dn0 = assign11400_e13008_d_n0;
        locals.var_qd_dn2 = assign11400_e13008_d_n2;
        locals.var_qd_dn6 = assign11400_e13008_d_n6;
        locals.var_qd_dn7 = assign11400_e13008_d_n7;
        locals.var_qd_dn10 = assign11400_e13008_d_n10;
        locals.var_qd_dn11 = assign11400_e13008_d_n11;
        locals.var_qd_dn12 = assign11400_e13008_d_n12;
        locals.var_qd_dn13 = assign11400_e13008_d_n13;
        locals.var_qd_dn15 = assign11400_e13008_d_n15;
        locals.var_qd_dn16 = assign11400_e13008_d_n16;
        locals.var_qd_dn17 = assign11400_e13008_d_n17;
        locals.var_qd_dn18 = assign11400_e13008_d_n18;

        let (assign11410_e13014, assign11410_e13014_d_n0, assign11410_e13014_d_n2, assign11410_e13014_d_n6, assign11410_e13014_d_n7, assign11410_e13014_d_n10, assign11410_e13014_d_n11, assign11410_e13014_d_n12, assign11410_e13014_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17,)
    }
};
        locals.var_ids = assign11410_e13014;
        locals.var_ids_dn0 = assign11410_e13014_d_n0;
        locals.var_ids_dn2 = assign11410_e13014_d_n2;
        locals.var_ids_dn6 = assign11410_e13014_d_n6;
        locals.var_ids_dn7 = assign11410_e13014_d_n7;
        locals.var_ids_dn10 = assign11410_e13014_d_n10;
        locals.var_ids_dn11 = assign11410_e13014_d_n11;
        locals.var_ids_dn12 = assign11410_e13014_d_n12;
        locals.var_ids_dn17 = assign11410_e13014_d_n17;

        let (assign11420_e13020, assign11420_e13020_d_n0, assign11420_e13020_d_n2, assign11420_e13020_d_n6, assign11420_e13020_d_n7, assign11420_e13020_d_n10, assign11420_e13020_d_n11, assign11420_e13020_d_n12, assign11420_e13020_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgvt, locals.var_vgvt_dn0, locals.var_vgvt_dn2, locals.var_vgvt_dn6, locals.var_vgvt_dn7, locals.var_vgvt_dn10, locals.var_vgvt_dn11, locals.var_vgvt_dn12, locals.var_vgvt_dn17,)
    }
};
        locals.var_vgvt = assign11420_e13020;
        locals.var_vgvt_dn0 = assign11420_e13020_d_n0;
        locals.var_vgvt_dn2 = assign11420_e13020_d_n2;
        locals.var_vgvt_dn6 = assign11420_e13020_d_n6;
        locals.var_vgvt_dn7 = assign11420_e13020_d_n7;
        locals.var_vgvt_dn10 = assign11420_e13020_d_n10;
        locals.var_vgvt_dn11 = assign11420_e13020_d_n11;
        locals.var_vgvt_dn12 = assign11420_e13020_d_n12;
        locals.var_vgvt_dn17 = assign11420_e13020_d_n17;

        let (assign11430_e13026,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_noqi,)
    }
};
        locals.var_flg_noqi = assign11430_e13026;

        let (assign11440_e13032, assign11440_e13032_d_n0, assign11440_e13032_d_n2, assign11440_e13032_d_n6, assign11440_e13032_d_n7, assign11440_e13032_d_n10, assign11440_e13032_d_n11, assign11440_e13032_d_n12, assign11440_e13032_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 != 0.0)) {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
        locals.var_phi_sl_soi = assign11440_e13032;
        locals.var_phi_sl_soi_dn0 = assign11440_e13032_d_n0;
        locals.var_phi_sl_soi_dn2 = assign11440_e13032_d_n2;
        locals.var_phi_sl_soi_dn6 = assign11440_e13032_d_n6;
        locals.var_phi_sl_soi_dn7 = assign11440_e13032_d_n7;
        locals.var_phi_sl_soi_dn10 = assign11440_e13032_d_n10;
        locals.var_phi_sl_soi_dn11 = assign11440_e13032_d_n11;
        locals.var_phi_sl_soi_dn12 = assign11440_e13032_d_n12;
        locals.var_phi_sl_soi_dn17 = assign11440_e13032_d_n17;

        let (assign11450_e13038, assign11450_e13038_d_n0, assign11450_e13038_d_n2, assign11450_e13038_d_n6, assign11450_e13038_d_n7, assign11450_e13038_d_n10, assign11450_e13038_d_n11, assign11450_e13038_d_n12, assign11450_e13038_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 != 0.0)) {
        (locals.var_phi_b0_soi, locals.var_phi_b0_soi_dn0, locals.var_phi_b0_soi_dn2, locals.var_phi_b0_soi_dn6, locals.var_phi_b0_soi_dn7, locals.var_phi_b0_soi_dn10, locals.var_phi_b0_soi_dn11, locals.var_phi_b0_soi_dn12, locals.var_phi_b0_soi_dn17,)
    } else {
        (locals.var_phi_bl_soi, locals.var_phi_bl_soi_dn0, locals.var_phi_bl_soi_dn2, locals.var_phi_bl_soi_dn6, locals.var_phi_bl_soi_dn7, locals.var_phi_bl_soi_dn10, locals.var_phi_bl_soi_dn11, locals.var_phi_bl_soi_dn12, locals.var_phi_bl_soi_dn17,)
    }
};
        locals.var_phi_bl_soi = assign11450_e13038;
        locals.var_phi_bl_soi_dn0 = assign11450_e13038_d_n0;
        locals.var_phi_bl_soi_dn2 = assign11450_e13038_d_n2;
        locals.var_phi_bl_soi_dn6 = assign11450_e13038_d_n6;
        locals.var_phi_bl_soi_dn7 = assign11450_e13038_d_n7;
        locals.var_phi_bl_soi_dn10 = assign11450_e13038_d_n10;
        locals.var_phi_bl_soi_dn11 = assign11450_e13038_d_n11;
        locals.var_phi_bl_soi_dn12 = assign11450_e13038_d_n12;
        locals.var_phi_bl_soi_dn17 = assign11450_e13038_d_n17;

        let (assign11460_e13044, assign11460_e13044_d_n0, assign11460_e13044_d_n2, assign11460_e13044_d_n6, assign11460_e13044_d_n7, assign11460_e13044_d_n10, assign11460_e13044_d_n11, assign11460_e13044_d_n12, assign11460_e13044_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 != 0.0)) {
        (locals.var_phi_s0_bulk, locals.var_phi_s0_bulk_dn0, locals.var_phi_s0_bulk_dn2, locals.var_phi_s0_bulk_dn6, locals.var_phi_s0_bulk_dn7, locals.var_phi_s0_bulk_dn10, locals.var_phi_s0_bulk_dn11, locals.var_phi_s0_bulk_dn12, locals.var_phi_s0_bulk_dn17,)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
        locals.var_phi_sl_bulk = assign11460_e13044;
        locals.var_phi_sl_bulk_dn0 = assign11460_e13044_d_n0;
        locals.var_phi_sl_bulk_dn2 = assign11460_e13044_d_n2;
        locals.var_phi_sl_bulk_dn6 = assign11460_e13044_d_n6;
        locals.var_phi_sl_bulk_dn7 = assign11460_e13044_d_n7;
        locals.var_phi_sl_bulk_dn10 = assign11460_e13044_d_n10;
        locals.var_phi_sl_bulk_dn11 = assign11460_e13044_d_n11;
        locals.var_phi_sl_bulk_dn12 = assign11460_e13044_d_n12;
        locals.var_phi_sl_bulk_dn17 = assign11460_e13044_d_n17;

        let (assign11470_e13050, assign11470_e13050_d_n0, assign11470_e13050_d_n2, assign11470_e13050_d_n6, assign11470_e13050_d_n7, assign11470_e13050_d_n10, assign11470_e13050_d_n11, assign11470_e13050_d_n12, assign11470_e13050_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 != 0.0)) {
        (locals.var_q_s0_bulk, locals.var_q_s0_bulk_dn0, locals.var_q_s0_bulk_dn2, locals.var_q_s0_bulk_dn6, locals.var_q_s0_bulk_dn7, locals.var_q_s0_bulk_dn10, locals.var_q_s0_bulk_dn11, locals.var_q_s0_bulk_dn12, locals.var_q_s0_bulk_dn17,)
    } else {
        (locals.var_q_sl_bulk, locals.var_q_sl_bulk_dn0, locals.var_q_sl_bulk_dn2, locals.var_q_sl_bulk_dn6, locals.var_q_sl_bulk_dn7, locals.var_q_sl_bulk_dn10, locals.var_q_sl_bulk_dn11, locals.var_q_sl_bulk_dn12, locals.var_q_sl_bulk_dn17,)
    }
};
        locals.var_q_sl_bulk = assign11470_e13050;
        locals.var_q_sl_bulk_dn0 = assign11470_e13050_d_n0;
        locals.var_q_sl_bulk_dn2 = assign11470_e13050_d_n2;
        locals.var_q_sl_bulk_dn6 = assign11470_e13050_d_n6;
        locals.var_q_sl_bulk_dn7 = assign11470_e13050_d_n7;
        locals.var_q_sl_bulk_dn10 = assign11470_e13050_d_n10;
        locals.var_q_sl_bulk_dn11 = assign11470_e13050_d_n11;
        locals.var_q_sl_bulk_dn12 = assign11470_e13050_d_n12;
        locals.var_q_sl_bulk_dn17 = assign11470_e13050_d_n17;

        let (assign11480_e13056, assign11480_e13056_d_n0, assign11480_e13056_d_n2, assign11480_e13056_d_n6, assign11480_e13056_d_n7, assign11480_e13056_d_n10, assign11480_e13056_d_n11, assign11480_e13056_d_n12, assign11480_e13056_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 != 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    }
};
        locals.var_psl = assign11480_e13056;
        locals.var_psl_dn0 = assign11480_e13056_d_n0;
        locals.var_psl_dn2 = assign11480_e13056_d_n2;
        locals.var_psl_dn6 = assign11480_e13056_d_n6;
        locals.var_psl_dn7 = assign11480_e13056_d_n7;
        locals.var_psl_dn10 = assign11480_e13056_d_n10;
        locals.var_psl_dn11 = assign11480_e13056_d_n11;
        locals.var_psl_dn12 = assign11480_e13056_d_n12;
        locals.var_psl_dn17 = assign11480_e13056_d_n17;

        let (assign11490_e13062, assign11490_e13062_d_n0, assign11490_e13062_d_n2, assign11490_e13062_d_n6, assign11490_e13062_d_n7, assign11490_e13062_d_n10, assign11490_e13062_d_n11, assign11490_e13062_d_n12, assign11490_e13062_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 != 0.0)) {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17,)
    }
};
        locals.var_psdl = assign11490_e13062;
        locals.var_psdl_dn0 = assign11490_e13062_d_n0;
        locals.var_psdl_dn2 = assign11490_e13062_d_n2;
        locals.var_psdl_dn6 = assign11490_e13062_d_n6;
        locals.var_psdl_dn7 = assign11490_e13062_d_n7;
        locals.var_psdl_dn10 = assign11490_e13062_d_n10;
        locals.var_psdl_dn11 = assign11490_e13062_d_n11;
        locals.var_psdl_dn12 = assign11490_e13062_d_n12;
        locals.var_psdl_dn17 = assign11490_e13062_d_n17;

    }

    pub(super) fn stamp_transient_block_33(
        locals: &mut StampLocals,
    ) {
        let (assign11510_e13075, assign11510_e13075_d_n0, assign11510_e13075_d_n2, assign11510_e13075_d_n6, assign11510_e13075_d_n7, assign11510_e13075_d_n10, assign11510_e13075_d_n11, assign11510_e13075_d_n12, assign11510_e13075_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    } else {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn12, locals.var_vdsorg_dn17,)
    }
};
        locals.var_vdsorg = assign11510_e13075;
        locals.var_vdsorg_dn0 = assign11510_e13075_d_n0;
        locals.var_vdsorg_dn2 = assign11510_e13075_d_n2;
        locals.var_vdsorg_dn6 = assign11510_e13075_d_n6;
        locals.var_vdsorg_dn7 = assign11510_e13075_d_n7;
        locals.var_vdsorg_dn10 = assign11510_e13075_d_n10;
        locals.var_vdsorg_dn11 = assign11510_e13075_d_n11;
        locals.var_vdsorg_dn12 = assign11510_e13075_d_n12;
        locals.var_vdsorg_dn17 = assign11510_e13075_d_n17;

        let (assign11520_e13082, assign11520_e13082_d_n0, assign11520_e13082_d_n2, assign11520_e13082_d_n6, assign11520_e13082_d_n7, assign11520_e13082_d_n10, assign11520_e13082_d_n11, assign11520_e13082_d_n12, assign11520_e13082_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        (1e-50, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12, locals.var_t10_dn17,)
    }
};
        locals.var_t10 = assign11520_e13082;
        locals.var_t10_dn0 = assign11520_e13082_d_n0;
        locals.var_t10_dn2 = assign11520_e13082_d_n2;
        locals.var_t10_dn6 = assign11520_e13082_d_n6;
        locals.var_t10_dn7 = assign11520_e13082_d_n7;
        locals.var_t10_dn10 = assign11520_e13082_d_n10;
        locals.var_t10_dn11 = assign11520_e13082_d_n11;
        locals.var_t10_dn12 = assign11520_e13082_d_n12;
        locals.var_t10_dn17 = assign11520_e13082_d_n17;

        let (assign11530_e13093, assign11530_e13093_d_n0, assign11530_e13093_d_n2, assign11530_e13093_d_n6, assign11530_e13093_d_n7, assign11530_e13093_d_n10, assign11530_e13093_d_n11, assign11530_e13093_d_n12, assign11530_e13093_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign11530_e13090: f64 = (locals.var_c_fox * locals.var_c_fox);
        let assign11530_e13091: f64 = (locals.var_qnsub_esi / assign11530_e13090);
        (assign11530_e13091, (((locals.var_qnsub_esi_dn0 * assign11530_e13090) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn0 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn0)))) / (assign11530_e13090 * assign11530_e13090)), (((locals.var_qnsub_esi_dn2 * assign11530_e13090) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn2 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn2)))) / (assign11530_e13090 * assign11530_e13090)), (((locals.var_qnsub_esi_dn6 * assign11530_e13090) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn6 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn6)))) / (assign11530_e13090 * assign11530_e13090)), (((locals.var_qnsub_esi_dn7 * assign11530_e13090) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn7 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn7)))) / (assign11530_e13090 * assign11530_e13090)), (((locals.var_qnsub_esi_dn10 * assign11530_e13090) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn10 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn10)))) / (assign11530_e13090 * assign11530_e13090)), (((locals.var_qnsub_esi_dn11 * assign11530_e13090) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn11 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn11)))) / (assign11530_e13090 * assign11530_e13090)), (((locals.var_qnsub_esi_dn12 * assign11530_e13090) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn12 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn12)))) / (assign11530_e13090 * assign11530_e13090)), (((locals.var_qnsub_esi_dn17 * assign11530_e13090) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn17 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn17)))) / (assign11530_e13090 * assign11530_e13090)),)
    } else {
        (locals.var_t2__blk307, locals.var_t2__blk307_dn0, locals.var_t2__blk307_dn2, locals.var_t2__blk307_dn6, locals.var_t2__blk307_dn7, locals.var_t2__blk307_dn10, locals.var_t2__blk307_dn11, locals.var_t2__blk307_dn12, locals.var_t2__blk307_dn17,)
    }
};
        locals.var_t2__blk307 = assign11530_e13093;
        locals.var_t2__blk307_dn0 = assign11530_e13093_d_n0;
        locals.var_t2__blk307_dn2 = assign11530_e13093_d_n2;
        locals.var_t2__blk307_dn6 = assign11530_e13093_d_n6;
        locals.var_t2__blk307_dn7 = assign11530_e13093_d_n7;
        locals.var_t2__blk307_dn10 = assign11530_e13093_d_n10;
        locals.var_t2__blk307_dn11 = assign11530_e13093_d_n11;
        locals.var_t2__blk307_dn12 = assign11530_e13093_d_n12;
        locals.var_t2__blk307_dn17 = assign11530_e13093_d_n17;

        let (assign11540_e13108, assign11540_e13108_d_n0, assign11540_e13108_d_n2, assign11540_e13108_d_n6, assign11540_e13108_d_n7, assign11540_e13108_d_n10, assign11540_e13108_d_n11, assign11540_e13108_d_n12, assign11540_e13108_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign11540_e13101: f64 = (2.0 / locals.var_t2__blk307);
        let assign11540_e13104: f64 = (locals.var_vgp - locals.var_t10);
        let assign11540_e13105: f64 = (assign11540_e13101 * assign11540_e13104);
        let assign11540_e13106: f64 = (1.0 + assign11540_e13105);
        (assign11540_e13106, (((-((2.0 * locals.var_t2__blk307_dn0) / (locals.var_t2__blk307 * locals.var_t2__blk307))) * assign11540_e13104) + (assign11540_e13101 * (locals.var_vgp_dn0 - locals.var_t10_dn0))), (((-((2.0 * locals.var_t2__blk307_dn2) / (locals.var_t2__blk307 * locals.var_t2__blk307))) * assign11540_e13104) + (assign11540_e13101 * (locals.var_vgp_dn2 - locals.var_t10_dn2))), (((-((2.0 * locals.var_t2__blk307_dn6) / (locals.var_t2__blk307 * locals.var_t2__blk307))) * assign11540_e13104) + (assign11540_e13101 * (locals.var_vgp_dn6 - locals.var_t10_dn6))), (((-((2.0 * locals.var_t2__blk307_dn7) / (locals.var_t2__blk307 * locals.var_t2__blk307))) * assign11540_e13104) + (assign11540_e13101 * (locals.var_vgp_dn7 - locals.var_t10_dn7))), (((-((2.0 * locals.var_t2__blk307_dn10) / (locals.var_t2__blk307 * locals.var_t2__blk307))) * assign11540_e13104) + (assign11540_e13101 * (locals.var_vgp_dn10 - locals.var_t10_dn10))), (((-((2.0 * locals.var_t2__blk307_dn11) / (locals.var_t2__blk307 * locals.var_t2__blk307))) * assign11540_e13104) + (assign11540_e13101 * (locals.var_vgp_dn11 - locals.var_t10_dn11))), (((-((2.0 * locals.var_t2__blk307_dn12) / (locals.var_t2__blk307 * locals.var_t2__blk307))) * assign11540_e13104) + (assign11540_e13101 * (locals.var_vgp_dn12 - locals.var_t10_dn12))), (((-((2.0 * locals.var_t2__blk307_dn17) / (locals.var_t2__blk307 * locals.var_t2__blk307))) * assign11540_e13104) + (assign11540_e13101 * (locals.var_vgp_dn17 - locals.var_t10_dn17))),)
    } else {
        (locals.var_t4__blk309, locals.var_t4__blk309_dn0, locals.var_t4__blk309_dn2, locals.var_t4__blk309_dn6, locals.var_t4__blk309_dn7, locals.var_t4__blk309_dn10, locals.var_t4__blk309_dn11, locals.var_t4__blk309_dn12, locals.var_t4__blk309_dn17,)
    }
};
        locals.var_t4__blk309 = assign11540_e13108;
        locals.var_t4__blk309_dn0 = assign11540_e13108_d_n0;
        locals.var_t4__blk309_dn2 = assign11540_e13108_d_n2;
        locals.var_t4__blk309_dn6 = assign11540_e13108_d_n6;
        locals.var_t4__blk309_dn7 = assign11540_e13108_d_n7;
        locals.var_t4__blk309_dn10 = assign11540_e13108_d_n10;
        locals.var_t4__blk309_dn11 = assign11540_e13108_d_n11;
        locals.var_t4__blk309_dn12 = assign11540_e13108_d_n12;
        locals.var_t4__blk309_dn17 = assign11540_e13108_d_n17;

        let (assign11550_e13119, assign11550_e13119_d_n0, assign11550_e13119_d_n2, assign11550_e13119_d_n6, assign11550_e13119_d_n7, assign11550_e13119_d_n10, assign11550_e13119_d_n11, assign11550_e13119_d_n12, assign11550_e13119_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign11550_e13116: f64 = (2.0 / locals.var_t2__blk307);
        let assign11550_e13117: f64 = (1.0 + assign11550_e13116);
        (assign11550_e13117, (-((2.0 * locals.var_t2__blk307_dn0) / (locals.var_t2__blk307 * locals.var_t2__blk307))), (-((2.0 * locals.var_t2__blk307_dn2) / (locals.var_t2__blk307 * locals.var_t2__blk307))), (-((2.0 * locals.var_t2__blk307_dn6) / (locals.var_t2__blk307 * locals.var_t2__blk307))), (-((2.0 * locals.var_t2__blk307_dn7) / (locals.var_t2__blk307 * locals.var_t2__blk307))), (-((2.0 * locals.var_t2__blk307_dn10) / (locals.var_t2__blk307 * locals.var_t2__blk307))), (-((2.0 * locals.var_t2__blk307_dn11) / (locals.var_t2__blk307 * locals.var_t2__blk307))), (-((2.0 * locals.var_t2__blk307_dn12) / (locals.var_t2__blk307 * locals.var_t2__blk307))), (-((2.0 * locals.var_t2__blk307_dn17) / (locals.var_t2__blk307 * locals.var_t2__blk307))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign11550_e13119;
        locals.var_t5_dn0 = assign11550_e13119_d_n0;
        locals.var_t5_dn2 = assign11550_e13119_d_n2;
        locals.var_t5_dn6 = assign11550_e13119_d_n6;
        locals.var_t5_dn7 = assign11550_e13119_d_n7;
        locals.var_t5_dn10 = assign11550_e13119_d_n10;
        locals.var_t5_dn11 = assign11550_e13119_d_n11;
        locals.var_t5_dn12 = assign11550_e13119_d_n12;
        locals.var_t5_dn17 = assign11550_e13119_d_n17;

        let assign11560_e13123: f64 = locals.var_t5;
        let assign11560_e13128: f64 = if ((locals.var_t4__blk309 < assign11560_e13123) && (locals.var_t5 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard313 = assign11560_e13128;

        let (assign11570_e13141, assign11570_e13141_d_n0, assign11570_e13141_d_n2, assign11570_e13141_d_n6, assign11570_e13141_d_n7, assign11570_e13141_d_n10, assign11570_e13141_d_n11, assign11570_e13141_d_n12, assign11570_e13141_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard313 != 0.0)) {
        let assign11570_e13137: f64 = locals.var_t5;
        let assign11570_e13139: f64 = (assign11570_e13137 - locals.var_t4__blk309);
        (assign11570_e13139, (locals.var_t5_dn0 - locals.var_t4__blk309_dn0), (locals.var_t5_dn2 - locals.var_t4__blk309_dn2), (locals.var_t5_dn6 - locals.var_t4__blk309_dn6), (locals.var_t5_dn7 - locals.var_t4__blk309_dn7), (locals.var_t5_dn10 - locals.var_t4__blk309_dn10), (locals.var_t5_dn11 - locals.var_t4__blk309_dn11), (locals.var_t5_dn12 - locals.var_t4__blk309_dn12), (locals.var_t5_dn17 - locals.var_t4__blk309_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign11570_e13141;
        locals.var_tmf1_dn0 = assign11570_e13141_d_n0;
        locals.var_tmf1_dn2 = assign11570_e13141_d_n2;
        locals.var_tmf1_dn6 = assign11570_e13141_d_n6;
        locals.var_tmf1_dn7 = assign11570_e13141_d_n7;
        locals.var_tmf1_dn10 = assign11570_e13141_d_n10;
        locals.var_tmf1_dn11 = assign11570_e13141_d_n11;
        locals.var_tmf1_dn12 = assign11570_e13141_d_n12;
        locals.var_tmf1_dn17 = assign11570_e13141_d_n17;

        let (assign11580_e13152, assign11580_e13152_d_n0, assign11580_e13152_d_n2, assign11580_e13152_d_n6, assign11580_e13152_d_n7, assign11580_e13152_d_n10, assign11580_e13152_d_n11, assign11580_e13152_d_n12, assign11580_e13152_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard313 != 0.0)) {
        let assign11580_e13150: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign11580_e13150, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign11580_e13152;
        locals.var_x2_dn0 = assign11580_e13152_d_n0;
        locals.var_x2_dn2 = assign11580_e13152_d_n2;
        locals.var_x2_dn6 = assign11580_e13152_d_n6;
        locals.var_x2_dn7 = assign11580_e13152_d_n7;
        locals.var_x2_dn10 = assign11580_e13152_d_n10;
        locals.var_x2_dn11 = assign11580_e13152_d_n11;
        locals.var_x2_dn12 = assign11580_e13152_d_n12;
        locals.var_x2_dn17 = assign11580_e13152_d_n17;

        let (assign11590_e13163, assign11590_e13163_d_n0, assign11590_e13163_d_n2, assign11590_e13163_d_n6, assign11590_e13163_d_n7, assign11590_e13163_d_n10, assign11590_e13163_d_n11, assign11590_e13163_d_n12, assign11590_e13163_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard313 != 0.0)) {
        let assign11590_e13161: f64 = (locals.var_t5 * locals.var_t5);
        (assign11590_e13161, ((locals.var_t5_dn0 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn0)), ((locals.var_t5_dn2 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn2)), ((locals.var_t5_dn6 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn6)), ((locals.var_t5_dn7 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn7)), ((locals.var_t5_dn10 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn10)), ((locals.var_t5_dn11 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn11)), ((locals.var_t5_dn12 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn12)), ((locals.var_t5_dn17 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn17)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign11590_e13163;
        locals.var_xmax2_dn0 = assign11590_e13163_d_n0;
        locals.var_xmax2_dn2 = assign11590_e13163_d_n2;
        locals.var_xmax2_dn6 = assign11590_e13163_d_n6;
        locals.var_xmax2_dn7 = assign11590_e13163_d_n7;
        locals.var_xmax2_dn10 = assign11590_e13163_d_n10;
        locals.var_xmax2_dn11 = assign11590_e13163_d_n11;
        locals.var_xmax2_dn12 = assign11590_e13163_d_n12;
        locals.var_xmax2_dn17 = assign11590_e13163_d_n17;

        let (assign11600_e13172, assign11600_e13172_d_n0, assign11600_e13172_d_n2, assign11600_e13172_d_n6, assign11600_e13172_d_n7, assign11600_e13172_d_n10, assign11600_e13172_d_n11, assign11600_e13172_d_n12, assign11600_e13172_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard313 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign11600_e13172;
        locals.var_xp_dn0 = assign11600_e13172_d_n0;
        locals.var_xp_dn2 = assign11600_e13172_d_n2;
        locals.var_xp_dn6 = assign11600_e13172_d_n6;
        locals.var_xp_dn7 = assign11600_e13172_d_n7;
        locals.var_xp_dn10 = assign11600_e13172_d_n10;
        locals.var_xp_dn11 = assign11600_e13172_d_n11;
        locals.var_xp_dn12 = assign11600_e13172_d_n12;
        locals.var_xp_dn17 = assign11600_e13172_d_n17;

        let (assign11610_e13181, assign11610_e13181_d_n0, assign11610_e13181_d_n2, assign11610_e13181_d_n6, assign11610_e13181_d_n7, assign11610_e13181_d_n10, assign11610_e13181_d_n11, assign11610_e13181_d_n12, assign11610_e13181_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard313 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign11610_e13181;
        locals.var_xmp_dn0 = assign11610_e13181_d_n0;
        locals.var_xmp_dn2 = assign11610_e13181_d_n2;
        locals.var_xmp_dn6 = assign11610_e13181_d_n6;
        locals.var_xmp_dn7 = assign11610_e13181_d_n7;
        locals.var_xmp_dn10 = assign11610_e13181_d_n10;
        locals.var_xmp_dn11 = assign11610_e13181_d_n11;
        locals.var_xmp_dn12 = assign11610_e13181_d_n12;
        locals.var_xmp_dn17 = assign11610_e13181_d_n17;

        let (assign11620_e13190,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard313 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign11620_e13190;

        let (assign11630_e13199,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard313 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign11630_e13199;

        let (assign11640_e13208, assign11640_e13208_d_n0, assign11640_e13208_d_n2, assign11640_e13208_d_n6, assign11640_e13208_d_n7, assign11640_e13208_d_n10, assign11640_e13208_d_n11, assign11640_e13208_d_n12, assign11640_e13208_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard313 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign11640_e13208;
        locals.var_arg_dn0 = assign11640_e13208_d_n0;
        locals.var_arg_dn2 = assign11640_e13208_d_n2;
        locals.var_arg_dn6 = assign11640_e13208_d_n6;
        locals.var_arg_dn7 = assign11640_e13208_d_n7;
        locals.var_arg_dn10 = assign11640_e13208_d_n10;
        locals.var_arg_dn11 = assign11640_e13208_d_n11;
        locals.var_arg_dn12 = assign11640_e13208_d_n12;
        locals.var_arg_dn17 = assign11640_e13208_d_n17;

        let (assign11650_e13217, assign11650_e13217_d_n0, assign11650_e13217_d_n2, assign11650_e13217_d_n6, assign11650_e13217_d_n7, assign11650_e13217_d_n10, assign11650_e13217_d_n11, assign11650_e13217_d_n12, assign11650_e13217_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard313 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign11650_e13217;
        locals.var_dnm_dn0 = assign11650_e13217_d_n0;
        locals.var_dnm_dn2 = assign11650_e13217_d_n2;
        locals.var_dnm_dn6 = assign11650_e13217_d_n6;
        locals.var_dnm_dn7 = assign11650_e13217_d_n7;
        locals.var_dnm_dn10 = assign11650_e13217_d_n10;
        locals.var_dnm_dn11 = assign11650_e13217_d_n11;
        locals.var_dnm_dn12 = assign11650_e13217_d_n12;
        locals.var_dnm_dn17 = assign11650_e13217_d_n17;

        let (assign11660_e13228, assign11660_e13228_d_n0, assign11660_e13228_d_n2, assign11660_e13228_d_n6, assign11660_e13228_d_n7, assign11660_e13228_d_n10, assign11660_e13228_d_n11, assign11660_e13228_d_n12, assign11660_e13228_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard313 != 0.0)) {
        let assign11660_e13226: f64 = (locals.var_xp * locals.var_x2);
        (assign11660_e13226, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign11660_e13228;
        locals.var_xp_dn0 = assign11660_e13228_d_n0;
        locals.var_xp_dn2 = assign11660_e13228_d_n2;
        locals.var_xp_dn6 = assign11660_e13228_d_n6;
        locals.var_xp_dn7 = assign11660_e13228_d_n7;
        locals.var_xp_dn10 = assign11660_e13228_d_n10;
        locals.var_xp_dn11 = assign11660_e13228_d_n11;
        locals.var_xp_dn12 = assign11660_e13228_d_n12;
        locals.var_xp_dn17 = assign11660_e13228_d_n17;

        let (assign11670_e13239, assign11670_e13239_d_n0, assign11670_e13239_d_n2, assign11670_e13239_d_n6, assign11670_e13239_d_n7, assign11670_e13239_d_n10, assign11670_e13239_d_n11, assign11670_e13239_d_n12, assign11670_e13239_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard313 != 0.0)) {
        let assign11670_e13237: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign11670_e13237, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign11670_e13239;
        locals.var_xmp_dn0 = assign11670_e13239_d_n0;
        locals.var_xmp_dn2 = assign11670_e13239_d_n2;
        locals.var_xmp_dn6 = assign11670_e13239_d_n6;
        locals.var_xmp_dn7 = assign11670_e13239_d_n7;
        locals.var_xmp_dn10 = assign11670_e13239_d_n10;
        locals.var_xmp_dn11 = assign11670_e13239_d_n11;
        locals.var_xmp_dn12 = assign11670_e13239_d_n12;
        locals.var_xmp_dn17 = assign11670_e13239_d_n17;

        let (assign11680_e13250, assign11680_e13250_d_n0, assign11680_e13250_d_n2, assign11680_e13250_d_n6, assign11680_e13250_d_n7, assign11680_e13250_d_n10, assign11680_e13250_d_n11, assign11680_e13250_d_n12, assign11680_e13250_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard313 != 0.0)) {
        let assign11680_e13248: f64 = (locals.var_xp * locals.var_x2);
        (assign11680_e13248, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign11680_e13250;
        locals.var_xp_dn0 = assign11680_e13250_d_n0;
        locals.var_xp_dn2 = assign11680_e13250_d_n2;
        locals.var_xp_dn6 = assign11680_e13250_d_n6;
        locals.var_xp_dn7 = assign11680_e13250_d_n7;
        locals.var_xp_dn10 = assign11680_e13250_d_n10;
        locals.var_xp_dn11 = assign11680_e13250_d_n11;
        locals.var_xp_dn12 = assign11680_e13250_d_n12;
        locals.var_xp_dn17 = assign11680_e13250_d_n17;

        let (assign11690_e13261, assign11690_e13261_d_n0, assign11690_e13261_d_n2, assign11690_e13261_d_n6, assign11690_e13261_d_n7, assign11690_e13261_d_n10, assign11690_e13261_d_n11, assign11690_e13261_d_n12, assign11690_e13261_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard313 != 0.0)) {
        let assign11690_e13259: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign11690_e13259, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign11690_e13261;
        locals.var_xmp_dn0 = assign11690_e13261_d_n0;
        locals.var_xmp_dn2 = assign11690_e13261_d_n2;
        locals.var_xmp_dn6 = assign11690_e13261_d_n6;
        locals.var_xmp_dn7 = assign11690_e13261_d_n7;
        locals.var_xmp_dn10 = assign11690_e13261_d_n10;
        locals.var_xmp_dn11 = assign11690_e13261_d_n11;
        locals.var_xmp_dn12 = assign11690_e13261_d_n12;
        locals.var_xmp_dn17 = assign11690_e13261_d_n17;

        let (assign11700_e13272, assign11700_e13272_d_n0, assign11700_e13272_d_n2, assign11700_e13272_d_n6, assign11700_e13272_d_n7, assign11700_e13272_d_n10, assign11700_e13272_d_n11, assign11700_e13272_d_n12, assign11700_e13272_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard313 != 0.0)) {
        let assign11700_e13270: f64 = (locals.var_xp * locals.var_x2);
        (assign11700_e13270, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign11700_e13272;
        locals.var_xp_dn0 = assign11700_e13272_d_n0;
        locals.var_xp_dn2 = assign11700_e13272_d_n2;
        locals.var_xp_dn6 = assign11700_e13272_d_n6;
        locals.var_xp_dn7 = assign11700_e13272_d_n7;
        locals.var_xp_dn10 = assign11700_e13272_d_n10;
        locals.var_xp_dn11 = assign11700_e13272_d_n11;
        locals.var_xp_dn12 = assign11700_e13272_d_n12;
        locals.var_xp_dn17 = assign11700_e13272_d_n17;

        let (assign11710_e13283, assign11710_e13283_d_n0, assign11710_e13283_d_n2, assign11710_e13283_d_n6, assign11710_e13283_d_n7, assign11710_e13283_d_n10, assign11710_e13283_d_n11, assign11710_e13283_d_n12, assign11710_e13283_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard313 != 0.0)) {
        let assign11710_e13281: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign11710_e13281, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign11710_e13283;
        locals.var_xmp_dn0 = assign11710_e13283_d_n0;
        locals.var_xmp_dn2 = assign11710_e13283_d_n2;
        locals.var_xmp_dn6 = assign11710_e13283_d_n6;
        locals.var_xmp_dn7 = assign11710_e13283_d_n7;
        locals.var_xmp_dn10 = assign11710_e13283_d_n10;
        locals.var_xmp_dn11 = assign11710_e13283_d_n11;
        locals.var_xmp_dn12 = assign11710_e13283_d_n12;
        locals.var_xmp_dn17 = assign11710_e13283_d_n17;

        let (assign11720_e13294, assign11720_e13294_d_n0, assign11720_e13294_d_n2, assign11720_e13294_d_n6, assign11720_e13294_d_n7, assign11720_e13294_d_n10, assign11720_e13294_d_n11, assign11720_e13294_d_n12, assign11720_e13294_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard313 != 0.0)) {
        let assign11720_e13292: f64 = (locals.var_xp * locals.var_x2);
        (assign11720_e13292, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign11720_e13294;
        locals.var_xp_dn0 = assign11720_e13294_d_n0;
        locals.var_xp_dn2 = assign11720_e13294_d_n2;
        locals.var_xp_dn6 = assign11720_e13294_d_n6;
        locals.var_xp_dn7 = assign11720_e13294_d_n7;
        locals.var_xp_dn10 = assign11720_e13294_d_n10;
        locals.var_xp_dn11 = assign11720_e13294_d_n11;
        locals.var_xp_dn12 = assign11720_e13294_d_n12;
        locals.var_xp_dn17 = assign11720_e13294_d_n17;

        let (assign11730_e13305, assign11730_e13305_d_n0, assign11730_e13305_d_n2, assign11730_e13305_d_n6, assign11730_e13305_d_n7, assign11730_e13305_d_n10, assign11730_e13305_d_n11, assign11730_e13305_d_n12, assign11730_e13305_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard313 != 0.0)) {
        let assign11730_e13303: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign11730_e13303, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign11730_e13305;
        locals.var_xmp_dn0 = assign11730_e13305_d_n0;
        locals.var_xmp_dn2 = assign11730_e13305_d_n2;
        locals.var_xmp_dn6 = assign11730_e13305_d_n6;
        locals.var_xmp_dn7 = assign11730_e13305_d_n7;
        locals.var_xmp_dn10 = assign11730_e13305_d_n10;
        locals.var_xmp_dn11 = assign11730_e13305_d_n11;
        locals.var_xmp_dn12 = assign11730_e13305_d_n12;
        locals.var_xmp_dn17 = assign11730_e13305_d_n17;

        let (assign11740_e13316, assign11740_e13316_d_n0, assign11740_e13316_d_n2, assign11740_e13316_d_n6, assign11740_e13316_d_n7, assign11740_e13316_d_n10, assign11740_e13316_d_n11, assign11740_e13316_d_n12, assign11740_e13316_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard313 != 0.0)) {
        let assign11740_e13314: f64 = (locals.var_xp + locals.var_xmp);
        (assign11740_e13314, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign11740_e13316;
        locals.var_arg_dn0 = assign11740_e13316_d_n0;
        locals.var_arg_dn2 = assign11740_e13316_d_n2;
        locals.var_arg_dn6 = assign11740_e13316_d_n6;
        locals.var_arg_dn7 = assign11740_e13316_d_n7;
        locals.var_arg_dn10 = assign11740_e13316_d_n10;
        locals.var_arg_dn11 = assign11740_e13316_d_n11;
        locals.var_arg_dn12 = assign11740_e13316_d_n12;
        locals.var_arg_dn17 = assign11740_e13316_d_n17;

        let (assign11750_e13325, assign11750_e13325_d_n0, assign11750_e13325_d_n2, assign11750_e13325_d_n6, assign11750_e13325_d_n7, assign11750_e13325_d_n10, assign11750_e13325_d_n11, assign11750_e13325_d_n12, assign11750_e13325_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard313 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign11750_e13325;
        locals.var_dnm_dn0 = assign11750_e13325_d_n0;
        locals.var_dnm_dn2 = assign11750_e13325_d_n2;
        locals.var_dnm_dn6 = assign11750_e13325_d_n6;
        locals.var_dnm_dn7 = assign11750_e13325_d_n7;
        locals.var_dnm_dn10 = assign11750_e13325_d_n10;
        locals.var_dnm_dn11 = assign11750_e13325_d_n11;
        locals.var_dnm_dn12 = assign11750_e13325_d_n12;
        locals.var_dnm_dn17 = assign11750_e13325_d_n17;

        let assign11760_e13340: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard314 = assign11760_e13340;

        let assign11770_e13343: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard315 = assign11770_e13343;

        let (assign11780_e13356,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign11780_e13356;

        let assign11790_e13359: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard316 = assign11790_e13359;

        let (assign11800_e13375,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 == 0.0)) && (locals.var_guard316 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign11800_e13375;

        let assign11810_e13378: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard317 = assign11810_e13378;

        let (assign11820_e13397,) = {
    if (((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign11820_e13397;

        let assign11830_e13400: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard318 = assign11830_e13400;

        let (assign11840_e13422,) = {
    if ((((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 == 0.0)) && (locals.var_guard316 == 0.0)) && (locals.var_guard317 == 0.0)) && (locals.var_guard318 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign11840_e13422;

        let (assign11850_e13433,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign11850_e13433;

        let mut assign11860_loop_guard: usize = 0;
        while {
            let assign11860_cond_e13445: f64 = if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign11860_cond_e13445 != 0.0
        } {
            assign11860_loop_guard += 1;
            assert!(assign11860_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign11860_body0_e13457, assign11860_body0_e13457_d_n0, assign11860_body0_e13457_d_n2, assign11860_body0_e13457_d_n6, assign11860_body0_e13457_d_n7, assign11860_body0_e13457_d_n10, assign11860_body0_e13457_d_n11, assign11860_body0_e13457_d_n12, assign11860_body0_e13457_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) {
        let assign11860_body0_e13455: f64 = (locals.var_dnm).sqrt();
        (assign11860_body0_e13455, (locals.var_dnm_dn0 / (2.0 * assign11860_body0_e13455)), (locals.var_dnm_dn2 / (2.0 * assign11860_body0_e13455)), (locals.var_dnm_dn6 / (2.0 * assign11860_body0_e13455)), (locals.var_dnm_dn7 / (2.0 * assign11860_body0_e13455)), (locals.var_dnm_dn10 / (2.0 * assign11860_body0_e13455)), (locals.var_dnm_dn11 / (2.0 * assign11860_body0_e13455)), (locals.var_dnm_dn12 / (2.0 * assign11860_body0_e13455)), (locals.var_dnm_dn17 / (2.0 * assign11860_body0_e13455)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign11860_body0_e13457;
            locals.var_dnm_dn0 = assign11860_body0_e13457_d_n0;
            locals.var_dnm_dn2 = assign11860_body0_e13457_d_n2;
            locals.var_dnm_dn6 = assign11860_body0_e13457_d_n6;
            locals.var_dnm_dn7 = assign11860_body0_e13457_d_n7;
            locals.var_dnm_dn10 = assign11860_body0_e13457_d_n10;
            locals.var_dnm_dn11 = assign11860_body0_e13457_d_n11;
            locals.var_dnm_dn12 = assign11860_body0_e13457_d_n12;
            locals.var_dnm_dn17 = assign11860_body0_e13457_d_n17;
            let (assign11860_body1_e13470,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 != 0.0)) {
        let assign11860_body1_e13468: f64 = (locals.var_m0 + 1.0);
        (assign11860_body1_e13468,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign11860_body1_e13470;
        }

    }

    pub(super) fn stamp_transient_block_34(
        locals: &mut StampLocals,
    ) {
        let (assign11870_e13488, assign11870_e13488_d_n0, assign11870_e13488_d_n2, assign11870_e13488_d_n6, assign11870_e13488_d_n7, assign11870_e13488_d_n10, assign11870_e13488_d_n11, assign11870_e13488_d_n12, assign11870_e13488_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard313 != 0.0)) && (locals.var_guard314 == 0.0)) {
        let assign11870_e13484: f64 = (2.0 * 4.0);
        let assign11870_e13485: f64 = (1.0 / assign11870_e13484);
        let assign11870_e13486: f64 = (locals.var_dnm).powf(assign11870_e13485);
        (assign11870_e13486, if 0.0 == 0.0 && ((assign11870_e13485) as f64).is_finite() && ((assign11870_e13485) as f64).fract() == 0.0 { if assign11870_e13485 == 0.0 { 0.0 } else { (assign11870_e13485 * ((locals.var_dnm).powf(assign11870_e13485 - 1.0) * locals.var_dnm_dn0)) } } else { (assign11870_e13486 * (assign11870_e13485 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign11870_e13485) as f64).is_finite() && ((assign11870_e13485) as f64).fract() == 0.0 { if assign11870_e13485 == 0.0 { 0.0 } else { (assign11870_e13485 * ((locals.var_dnm).powf(assign11870_e13485 - 1.0) * locals.var_dnm_dn2)) } } else { (assign11870_e13486 * (assign11870_e13485 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign11870_e13485) as f64).is_finite() && ((assign11870_e13485) as f64).fract() == 0.0 { if assign11870_e13485 == 0.0 { 0.0 } else { (assign11870_e13485 * ((locals.var_dnm).powf(assign11870_e13485 - 1.0) * locals.var_dnm_dn6)) } } else { (assign11870_e13486 * (assign11870_e13485 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign11870_e13485) as f64).is_finite() && ((assign11870_e13485) as f64).fract() == 0.0 { if assign11870_e13485 == 0.0 { 0.0 } else { (assign11870_e13485 * ((locals.var_dnm).powf(assign11870_e13485 - 1.0) * locals.var_dnm_dn7)) } } else { (assign11870_e13486 * (assign11870_e13485 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign11870_e13485) as f64).is_finite() && ((assign11870_e13485) as f64).fract() == 0.0 { if assign11870_e13485 == 0.0 { 0.0 } else { (assign11870_e13485 * ((locals.var_dnm).powf(assign11870_e13485 - 1.0) * locals.var_dnm_dn10)) } } else { (assign11870_e13486 * (assign11870_e13485 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign11870_e13485) as f64).is_finite() && ((assign11870_e13485) as f64).fract() == 0.0 { if assign11870_e13485 == 0.0 { 0.0 } else { (assign11870_e13485 * ((locals.var_dnm).powf(assign11870_e13485 - 1.0) * locals.var_dnm_dn11)) } } else { (assign11870_e13486 * (assign11870_e13485 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign11870_e13485) as f64).is_finite() && ((assign11870_e13485) as f64).fract() == 0.0 { if assign11870_e13485 == 0.0 { 0.0 } else { (assign11870_e13485 * ((locals.var_dnm).powf(assign11870_e13485 - 1.0) * locals.var_dnm_dn12)) } } else { (assign11870_e13486 * (assign11870_e13485 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign11870_e13485) as f64).is_finite() && ((assign11870_e13485) as f64).fract() == 0.0 { if assign11870_e13485 == 0.0 { 0.0 } else { (assign11870_e13485 * ((locals.var_dnm).powf(assign11870_e13485 - 1.0) * locals.var_dnm_dn17)) } } else { (assign11870_e13486 * (assign11870_e13485 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign11870_e13488;
        locals.var_dnm_dn0 = assign11870_e13488_d_n0;
        locals.var_dnm_dn2 = assign11870_e13488_d_n2;
        locals.var_dnm_dn6 = assign11870_e13488_d_n6;
        locals.var_dnm_dn7 = assign11870_e13488_d_n7;
        locals.var_dnm_dn10 = assign11870_e13488_d_n10;
        locals.var_dnm_dn11 = assign11870_e13488_d_n11;
        locals.var_dnm_dn12 = assign11870_e13488_d_n12;
        locals.var_dnm_dn17 = assign11870_e13488_d_n17;

        let (assign11880_e13499, assign11880_e13499_d_n0, assign11880_e13499_d_n2, assign11880_e13499_d_n6, assign11880_e13499_d_n7, assign11880_e13499_d_n10, assign11880_e13499_d_n11, assign11880_e13499_d_n12, assign11880_e13499_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard313 != 0.0)) {
        let assign11880_e13497: f64 = (1.0 / locals.var_dnm);
        (assign11880_e13497, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign11880_e13499;
        locals.var_dnm_dn0 = assign11880_e13499_d_n0;
        locals.var_dnm_dn2 = assign11880_e13499_d_n2;
        locals.var_dnm_dn6 = assign11880_e13499_d_n6;
        locals.var_dnm_dn7 = assign11880_e13499_d_n7;
        locals.var_dnm_dn10 = assign11880_e13499_d_n10;
        locals.var_dnm_dn11 = assign11880_e13499_d_n11;
        locals.var_dnm_dn12 = assign11880_e13499_d_n12;
        locals.var_dnm_dn17 = assign11880_e13499_d_n17;

        let (assign11890_e13512, assign11890_e13512_d_n0, assign11890_e13512_d_n2, assign11890_e13512_d_n6, assign11890_e13512_d_n7, assign11890_e13512_d_n10, assign11890_e13512_d_n11, assign11890_e13512_d_n12, assign11890_e13512_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard313 != 0.0)) {
        let assign11890_e13508: f64 = (locals.var_tmf1 * locals.var_t5);
        let assign11890_e13510: f64 = (assign11890_e13508 * locals.var_dnm);
        (assign11890_e13510, ((((locals.var_tmf1_dn0 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn0)) * locals.var_dnm) + (assign11890_e13508 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn2)) * locals.var_dnm) + (assign11890_e13508 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn6 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn6)) * locals.var_dnm) + (assign11890_e13508 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn7)) * locals.var_dnm) + (assign11890_e13508 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn10 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn10)) * locals.var_dnm) + (assign11890_e13508 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn11)) * locals.var_dnm) + (assign11890_e13508 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn12 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn12)) * locals.var_dnm) + (assign11890_e13508 * locals.var_dnm_dn12)), ((((locals.var_tmf1_dn17 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn17)) * locals.var_dnm) + (assign11890_e13508 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign11890_e13512;
        locals.var_tmf0_dn0 = assign11890_e13512_d_n0;
        locals.var_tmf0_dn2 = assign11890_e13512_d_n2;
        locals.var_tmf0_dn6 = assign11890_e13512_d_n6;
        locals.var_tmf0_dn7 = assign11890_e13512_d_n7;
        locals.var_tmf0_dn10 = assign11890_e13512_d_n10;
        locals.var_tmf0_dn11 = assign11890_e13512_d_n11;
        locals.var_tmf0_dn12 = assign11890_e13512_d_n12;
        locals.var_tmf0_dn17 = assign11890_e13512_d_n17;

        let (assign11900_e13525, assign11900_e13525_d_n0, assign11900_e13525_d_n2, assign11900_e13525_d_n6, assign11900_e13525_d_n7, assign11900_e13525_d_n10, assign11900_e13525_d_n11, assign11900_e13525_d_n12, assign11900_e13525_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard313 != 0.0)) {
        let assign11900_e13521: f64 = locals.var_t5;
        let assign11900_e13523: f64 = (assign11900_e13521 - locals.var_tmf0);
        (assign11900_e13523, (locals.var_t5_dn0 - locals.var_tmf0_dn0), (locals.var_t5_dn2 - locals.var_tmf0_dn2), (locals.var_t5_dn6 - locals.var_tmf0_dn6), (locals.var_t5_dn7 - locals.var_tmf0_dn7), (locals.var_t5_dn10 - locals.var_tmf0_dn10), (locals.var_t5_dn11 - locals.var_tmf0_dn11), (locals.var_t5_dn12 - locals.var_tmf0_dn12), (locals.var_t5_dn17 - locals.var_tmf0_dn17),)
    } else {
        (locals.var_t4__blk309, locals.var_t4__blk309_dn0, locals.var_t4__blk309_dn2, locals.var_t4__blk309_dn6, locals.var_t4__blk309_dn7, locals.var_t4__blk309_dn10, locals.var_t4__blk309_dn11, locals.var_t4__blk309_dn12, locals.var_t4__blk309_dn17,)
    }
};
        locals.var_t4__blk309 = assign11900_e13525;
        locals.var_t4__blk309_dn0 = assign11900_e13525_d_n0;
        locals.var_t4__blk309_dn2 = assign11900_e13525_d_n2;
        locals.var_t4__blk309_dn6 = assign11900_e13525_d_n6;
        locals.var_t4__blk309_dn7 = assign11900_e13525_d_n7;
        locals.var_t4__blk309_dn10 = assign11900_e13525_d_n10;
        locals.var_t4__blk309_dn11 = assign11900_e13525_d_n11;
        locals.var_t4__blk309_dn12 = assign11900_e13525_d_n12;
        locals.var_t4__blk309_dn17 = assign11900_e13525_d_n17;

        let (assign11910_e13535, assign11910_e13535_d_n0, assign11910_e13535_d_n2, assign11910_e13535_d_n6, assign11910_e13535_d_n7, assign11910_e13535_d_n10, assign11910_e13535_d_n11, assign11910_e13535_d_n12, assign11910_e13535_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard313 == 0.0)) {
        (locals.var_t4__blk309, locals.var_t4__blk309_dn0, locals.var_t4__blk309_dn2, locals.var_t4__blk309_dn6, locals.var_t4__blk309_dn7, locals.var_t4__blk309_dn10, locals.var_t4__blk309_dn11, locals.var_t4__blk309_dn12, locals.var_t4__blk309_dn17,)
    } else {
        (locals.var_t4__blk309, locals.var_t4__blk309_dn0, locals.var_t4__blk309_dn2, locals.var_t4__blk309_dn6, locals.var_t4__blk309_dn7, locals.var_t4__blk309_dn10, locals.var_t4__blk309_dn11, locals.var_t4__blk309_dn12, locals.var_t4__blk309_dn17,)
    }
};
        locals.var_t4__blk309 = assign11910_e13535;
        locals.var_t4__blk309_dn0 = assign11910_e13535_d_n0;
        locals.var_t4__blk309_dn2 = assign11910_e13535_d_n2;
        locals.var_t4__blk309_dn6 = assign11910_e13535_d_n6;
        locals.var_t4__blk309_dn7 = assign11910_e13535_d_n7;
        locals.var_t4__blk309_dn10 = assign11910_e13535_d_n10;
        locals.var_t4__blk309_dn11 = assign11910_e13535_d_n11;
        locals.var_t4__blk309_dn12 = assign11910_e13535_d_n12;
        locals.var_t4__blk309_dn17 = assign11910_e13535_d_n17;

        let (assign11920_e13543, assign11920_e13543_d_n0, assign11920_e13543_d_n2, assign11920_e13543_d_n6, assign11920_e13543_d_n7, assign11920_e13543_d_n10, assign11920_e13543_d_n11, assign11920_e13543_d_n12, assign11920_e13543_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign11920_e13541: f64 = (locals.var_t4__blk309).sqrt();
        (assign11920_e13541, (locals.var_t4__blk309_dn0 / (2.0 * assign11920_e13541)), (locals.var_t4__blk309_dn2 / (2.0 * assign11920_e13541)), (locals.var_t4__blk309_dn6 / (2.0 * assign11920_e13541)), (locals.var_t4__blk309_dn7 / (2.0 * assign11920_e13541)), (locals.var_t4__blk309_dn10 / (2.0 * assign11920_e13541)), (locals.var_t4__blk309_dn11 / (2.0 * assign11920_e13541)), (locals.var_t4__blk309_dn12 / (2.0 * assign11920_e13541)), (locals.var_t4__blk309_dn17 / (2.0 * assign11920_e13541)),)
    } else {
        (locals.var_t3__blk308, locals.var_t3__blk308_dn0, locals.var_t3__blk308_dn2, locals.var_t3__blk308_dn6, locals.var_t3__blk308_dn7, locals.var_t3__blk308_dn10, locals.var_t3__blk308_dn11, locals.var_t3__blk308_dn12, locals.var_t3__blk308_dn17,)
    }
};
        locals.var_t3__blk308 = assign11920_e13543;
        locals.var_t3__blk308_dn0 = assign11920_e13543_d_n0;
        locals.var_t3__blk308_dn2 = assign11920_e13543_d_n2;
        locals.var_t3__blk308_dn6 = assign11920_e13543_d_n6;
        locals.var_t3__blk308_dn7 = assign11920_e13543_d_n7;
        locals.var_t3__blk308_dn10 = assign11920_e13543_d_n10;
        locals.var_t3__blk308_dn11 = assign11920_e13543_d_n11;
        locals.var_t3__blk308_dn12 = assign11920_e13543_d_n12;
        locals.var_t3__blk308_dn17 = assign11920_e13543_d_n17;

        let (assign11930_e13556, assign11930_e13556_d_n0, assign11930_e13556_d_n2, assign11930_e13556_d_n6, assign11930_e13556_d_n7, assign11930_e13556_d_n10, assign11930_e13556_d_n11, assign11930_e13556_d_n12, assign11930_e13556_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign11930_e13552: f64 = (1.0 - locals.var_t3__blk308);
        let assign11930_e13553: f64 = (locals.var_t2__blk307 * assign11930_e13552);
        let assign11930_e13554: f64 = (locals.var_vgp + assign11930_e13553);
        (assign11930_e13554, (locals.var_vgp_dn0 + ((locals.var_t2__blk307_dn0 * assign11930_e13552) + (locals.var_t2__blk307 * (-locals.var_t3__blk308_dn0)))), (locals.var_vgp_dn2 + ((locals.var_t2__blk307_dn2 * assign11930_e13552) + (locals.var_t2__blk307 * (-locals.var_t3__blk308_dn2)))), (locals.var_vgp_dn6 + ((locals.var_t2__blk307_dn6 * assign11930_e13552) + (locals.var_t2__blk307 * (-locals.var_t3__blk308_dn6)))), (locals.var_vgp_dn7 + ((locals.var_t2__blk307_dn7 * assign11930_e13552) + (locals.var_t2__blk307 * (-locals.var_t3__blk308_dn7)))), (locals.var_vgp_dn10 + ((locals.var_t2__blk307_dn10 * assign11930_e13552) + (locals.var_t2__blk307 * (-locals.var_t3__blk308_dn10)))), (locals.var_vgp_dn11 + ((locals.var_t2__blk307_dn11 * assign11930_e13552) + (locals.var_t2__blk307 * (-locals.var_t3__blk308_dn11)))), (locals.var_vgp_dn12 + ((locals.var_t2__blk307_dn12 * assign11930_e13552) + (locals.var_t2__blk307 * (-locals.var_t3__blk308_dn12)))), (locals.var_vgp_dn17 + ((locals.var_t2__blk307_dn17 * assign11930_e13552) + (locals.var_t2__blk307 * (-locals.var_t3__blk308_dn17)))),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12, locals.var_t10_dn17,)
    }
};
        locals.var_t10 = assign11930_e13556;
        locals.var_t10_dn0 = assign11930_e13556_d_n0;
        locals.var_t10_dn2 = assign11930_e13556_d_n2;
        locals.var_t10_dn6 = assign11930_e13556_d_n6;
        locals.var_t10_dn7 = assign11930_e13556_d_n7;
        locals.var_t10_dn10 = assign11930_e13556_d_n10;
        locals.var_t10_dn11 = assign11930_e13556_d_n11;
        locals.var_t10_dn12 = assign11930_e13556_d_n12;
        locals.var_t10_dn17 = assign11930_e13556_d_n17;

        let (assign11940_e13572, assign11940_e13572_d_n0, assign11940_e13572_d_n2, assign11940_e13572_d_n6, assign11940_e13572_d_n7, assign11940_e13572_d_n10, assign11940_e13572_d_n11, assign11940_e13572_d_n12, assign11940_e13572_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign11940_e13563: f64 = (locals.var_t10 * locals.var_t10);
        let assign11940_e13566: f64 = (4.0 * 0.01);
        let assign11940_e13568: f64 = (assign11940_e13566 * 0.01);
        let assign11940_e13569: f64 = (assign11940_e13563 + assign11940_e13568);
        let assign11940_e13570: f64 = (assign11940_e13569).sqrt();
        (assign11940_e13570, (((locals.var_t10_dn0 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn0)) / (2.0 * assign11940_e13570)), (((locals.var_t10_dn2 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn2)) / (2.0 * assign11940_e13570)), (((locals.var_t10_dn6 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn6)) / (2.0 * assign11940_e13570)), (((locals.var_t10_dn7 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn7)) / (2.0 * assign11940_e13570)), (((locals.var_t10_dn10 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn10)) / (2.0 * assign11940_e13570)), (((locals.var_t10_dn11 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn11)) / (2.0 * assign11940_e13570)), (((locals.var_t10_dn12 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn12)) / (2.0 * assign11940_e13570)), (((locals.var_t10_dn17 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn17)) / (2.0 * assign11940_e13570)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign11940_e13572;
        locals.var_tmf1_dn0 = assign11940_e13572_d_n0;
        locals.var_tmf1_dn2 = assign11940_e13572_d_n2;
        locals.var_tmf1_dn6 = assign11940_e13572_d_n6;
        locals.var_tmf1_dn7 = assign11940_e13572_d_n7;
        locals.var_tmf1_dn10 = assign11940_e13572_d_n10;
        locals.var_tmf1_dn11 = assign11940_e13572_d_n11;
        locals.var_tmf1_dn12 = assign11940_e13572_d_n12;
        locals.var_tmf1_dn17 = assign11940_e13572_d_n17;

        let (assign11950_e13587, assign11950_e13587_d_n0, assign11950_e13587_d_n2, assign11950_e13587_d_n6, assign11950_e13587_d_n7, assign11950_e13587_d_n10, assign11950_e13587_d_n11, assign11950_e13587_d_n12, assign11950_e13587_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign11950_e13580: f64 = (locals.var_t10 + locals.var_tmf1);
        let assign11950_e13581: f64 = (0.5 * assign11950_e13580);
        let assign11950_e13584: f64 = (1e-10 * 0.01);
        let assign11950_e13585: f64 = (assign11950_e13581 + assign11950_e13584);
        (assign11950_e13585, (0.5 * (locals.var_t10_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t10_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t10_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t10_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t10_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t10_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t10_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t10_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12, locals.var_t10_dn17,)
    }
};
        locals.var_t10 = assign11950_e13587;
        locals.var_t10_dn0 = assign11950_e13587_d_n0;
        locals.var_t10_dn2 = assign11950_e13587_d_n2;
        locals.var_t10_dn6 = assign11950_e13587_d_n6;
        locals.var_t10_dn7 = assign11950_e13587_d_n7;
        locals.var_t10_dn10 = assign11950_e13587_d_n10;
        locals.var_t10_dn11 = assign11950_e13587_d_n11;
        locals.var_t10_dn12 = assign11950_e13587_d_n12;
        locals.var_t10_dn17 = assign11950_e13587_d_n17;

        let assign11960_e13590: f64 = if locals.var_t10 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard319 = assign11960_e13590;

        let (assign11970_e13599, assign11970_e13599_d_n0, assign11970_e13599_d_n2, assign11970_e13599_d_n6, assign11970_e13599_d_n7, assign11970_e13599_d_n10, assign11970_e13599_d_n11, assign11970_e13599_d_n12, assign11970_e13599_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard319 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12, locals.var_t10_dn17,)
    }
};
        locals.var_t10 = assign11970_e13599;
        locals.var_t10_dn0 = assign11970_e13599_d_n0;
        locals.var_t10_dn2 = assign11970_e13599_d_n2;
        locals.var_t10_dn6 = assign11970_e13599_d_n6;
        locals.var_t10_dn7 = assign11970_e13599_d_n7;
        locals.var_t10_dn10 = assign11970_e13599_d_n10;
        locals.var_t10_dn11 = assign11970_e13599_d_n11;
        locals.var_t10_dn12 = assign11970_e13599_d_n12;
        locals.var_t10_dn17 = assign11970_e13599_d_n17;

        let (assign11990_e13615, assign11990_e13615_d_n0, assign11990_e13615_d_n2, assign11990_e13615_d_n6, assign11990_e13615_d_n7, assign11990_e13615_d_n10, assign11990_e13615_d_n11, assign11990_e13615_d_n12, assign11990_e13615_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign11990_e13613: f64 = (locals.var_vds / locals.var_t10);
        (assign11990_e13613, (((locals.var_vds_dn0 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn0)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn2 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn2)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn6 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn6)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn7 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn7)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn10 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn10)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn11 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn11)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn12 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn12)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn17 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn17)) / (locals.var_t10 * locals.var_t10)),)
    } else {
        (locals.var_t1__blk306, locals.var_t1__blk306_dn0, locals.var_t1__blk306_dn2, locals.var_t1__blk306_dn6, locals.var_t1__blk306_dn7, locals.var_t1__blk306_dn10, locals.var_t1__blk306_dn11, locals.var_t1__blk306_dn12, locals.var_t1__blk306_dn17,)
    }
};
        locals.var_t1__blk306 = assign11990_e13615;
        locals.var_t1__blk306_dn0 = assign11990_e13615_d_n0;
        locals.var_t1__blk306_dn2 = assign11990_e13615_d_n2;
        locals.var_t1__blk306_dn6 = assign11990_e13615_d_n6;
        locals.var_t1__blk306_dn7 = assign11990_e13615_d_n7;
        locals.var_t1__blk306_dn10 = assign11990_e13615_d_n10;
        locals.var_t1__blk306_dn11 = assign11990_e13615_d_n11;
        locals.var_t1__blk306_dn12 = assign11990_e13615_d_n12;
        locals.var_t1__blk306_dn17 = assign11990_e13615_d_n17;

        let (assign12000_e13626, assign12000_e13626_d_n0, assign12000_e13626_d_n2, assign12000_e13626_d_n6, assign12000_e13626_d_n7, assign12000_e13626_d_n10, assign12000_e13626_d_n11, assign12000_e13626_d_n12, assign12000_e13626_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign12000_e13623: f64 = (locals.var_ddlte - 1.0);
        let assign12000_e13624: f64 = (locals.var_t1__blk306).powf(assign12000_e13623);
        (assign12000_e13624, if 0.0 == 0.0 && ((assign12000_e13623) as f64).is_finite() && ((assign12000_e13623) as f64).fract() == 0.0 { if assign12000_e13623 == 0.0 { 0.0 } else { (assign12000_e13623 * ((locals.var_t1__blk306).powf(assign12000_e13623 - 1.0) * locals.var_t1__blk306_dn0)) } } else { (assign12000_e13624 * (assign12000_e13623 * (locals.var_t1__blk306_dn0 / locals.var_t1__blk306))) }, if 0.0 == 0.0 && ((assign12000_e13623) as f64).is_finite() && ((assign12000_e13623) as f64).fract() == 0.0 { if assign12000_e13623 == 0.0 { 0.0 } else { (assign12000_e13623 * ((locals.var_t1__blk306).powf(assign12000_e13623 - 1.0) * locals.var_t1__blk306_dn2)) } } else { (assign12000_e13624 * (assign12000_e13623 * (locals.var_t1__blk306_dn2 / locals.var_t1__blk306))) }, if 0.0 == 0.0 && ((assign12000_e13623) as f64).is_finite() && ((assign12000_e13623) as f64).fract() == 0.0 { if assign12000_e13623 == 0.0 { 0.0 } else { (assign12000_e13623 * ((locals.var_t1__blk306).powf(assign12000_e13623 - 1.0) * locals.var_t1__blk306_dn6)) } } else { (assign12000_e13624 * (assign12000_e13623 * (locals.var_t1__blk306_dn6 / locals.var_t1__blk306))) }, if 0.0 == 0.0 && ((assign12000_e13623) as f64).is_finite() && ((assign12000_e13623) as f64).fract() == 0.0 { if assign12000_e13623 == 0.0 { 0.0 } else { (assign12000_e13623 * ((locals.var_t1__blk306).powf(assign12000_e13623 - 1.0) * locals.var_t1__blk306_dn7)) } } else { (assign12000_e13624 * (assign12000_e13623 * (locals.var_t1__blk306_dn7 / locals.var_t1__blk306))) }, if 0.0 == 0.0 && ((assign12000_e13623) as f64).is_finite() && ((assign12000_e13623) as f64).fract() == 0.0 { if assign12000_e13623 == 0.0 { 0.0 } else { (assign12000_e13623 * ((locals.var_t1__blk306).powf(assign12000_e13623 - 1.0) * locals.var_t1__blk306_dn10)) } } else { (assign12000_e13624 * (assign12000_e13623 * (locals.var_t1__blk306_dn10 / locals.var_t1__blk306))) }, if 0.0 == 0.0 && ((assign12000_e13623) as f64).is_finite() && ((assign12000_e13623) as f64).fract() == 0.0 { if assign12000_e13623 == 0.0 { 0.0 } else { (assign12000_e13623 * ((locals.var_t1__blk306).powf(assign12000_e13623 - 1.0) * locals.var_t1__blk306_dn11)) } } else { (assign12000_e13624 * (assign12000_e13623 * (locals.var_t1__blk306_dn11 / locals.var_t1__blk306))) }, if 0.0 == 0.0 && ((assign12000_e13623) as f64).is_finite() && ((assign12000_e13623) as f64).fract() == 0.0 { if assign12000_e13623 == 0.0 { 0.0 } else { (assign12000_e13623 * ((locals.var_t1__blk306).powf(assign12000_e13623 - 1.0) * locals.var_t1__blk306_dn12)) } } else { (assign12000_e13624 * (assign12000_e13623 * (locals.var_t1__blk306_dn12 / locals.var_t1__blk306))) }, if 0.0 == 0.0 && ((assign12000_e13623) as f64).is_finite() && ((assign12000_e13623) as f64).fract() == 0.0 { if assign12000_e13623 == 0.0 { 0.0 } else { (assign12000_e13623 * ((locals.var_t1__blk306).powf(assign12000_e13623 - 1.0) * locals.var_t1__blk306_dn17)) } } else { (assign12000_e13624 * (assign12000_e13623 * (locals.var_t1__blk306_dn17 / locals.var_t1__blk306))) },)
    } else {
        (locals.var_t2__blk307, locals.var_t2__blk307_dn0, locals.var_t2__blk307_dn2, locals.var_t2__blk307_dn6, locals.var_t2__blk307_dn7, locals.var_t2__blk307_dn10, locals.var_t2__blk307_dn11, locals.var_t2__blk307_dn12, locals.var_t2__blk307_dn17,)
    }
};
        locals.var_t2__blk307 = assign12000_e13626;
        locals.var_t2__blk307_dn0 = assign12000_e13626_d_n0;
        locals.var_t2__blk307_dn2 = assign12000_e13626_d_n2;
        locals.var_t2__blk307_dn6 = assign12000_e13626_d_n6;
        locals.var_t2__blk307_dn7 = assign12000_e13626_d_n7;
        locals.var_t2__blk307_dn10 = assign12000_e13626_d_n10;
        locals.var_t2__blk307_dn11 = assign12000_e13626_d_n11;
        locals.var_t2__blk307_dn12 = assign12000_e13626_d_n12;
        locals.var_t2__blk307_dn17 = assign12000_e13626_d_n17;

        let (assign12010_e13635, assign12010_e13635_d_n0, assign12010_e13635_d_n2, assign12010_e13635_d_n6, assign12010_e13635_d_n7, assign12010_e13635_d_n10, assign12010_e13635_d_n11, assign12010_e13635_d_n12, assign12010_e13635_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign12010_e13633: f64 = (locals.var_t2__blk307 * locals.var_t1__blk306);
        (assign12010_e13633, ((locals.var_t2__blk307_dn0 * locals.var_t1__blk306) + (locals.var_t2__blk307 * locals.var_t1__blk306_dn0)), ((locals.var_t2__blk307_dn2 * locals.var_t1__blk306) + (locals.var_t2__blk307 * locals.var_t1__blk306_dn2)), ((locals.var_t2__blk307_dn6 * locals.var_t1__blk306) + (locals.var_t2__blk307 * locals.var_t1__blk306_dn6)), ((locals.var_t2__blk307_dn7 * locals.var_t1__blk306) + (locals.var_t2__blk307 * locals.var_t1__blk306_dn7)), ((locals.var_t2__blk307_dn10 * locals.var_t1__blk306) + (locals.var_t2__blk307 * locals.var_t1__blk306_dn10)), ((locals.var_t2__blk307_dn11 * locals.var_t1__blk306) + (locals.var_t2__blk307 * locals.var_t1__blk306_dn11)), ((locals.var_t2__blk307_dn12 * locals.var_t1__blk306) + (locals.var_t2__blk307 * locals.var_t1__blk306_dn12)), ((locals.var_t2__blk307_dn17 * locals.var_t1__blk306) + (locals.var_t2__blk307 * locals.var_t1__blk306_dn17)),)
    } else {
        (locals.var_t7__blk311, locals.var_t7__blk311_dn0, locals.var_t7__blk311_dn2, locals.var_t7__blk311_dn6, locals.var_t7__blk311_dn7, locals.var_t7__blk311_dn10, locals.var_t7__blk311_dn11, locals.var_t7__blk311_dn12, locals.var_t7__blk311_dn17,)
    }
};
        locals.var_t7__blk311 = assign12010_e13635;
        locals.var_t7__blk311_dn0 = assign12010_e13635_d_n0;
        locals.var_t7__blk311_dn2 = assign12010_e13635_d_n2;
        locals.var_t7__blk311_dn6 = assign12010_e13635_d_n6;
        locals.var_t7__blk311_dn7 = assign12010_e13635_d_n7;
        locals.var_t7__blk311_dn10 = assign12010_e13635_d_n10;
        locals.var_t7__blk311_dn11 = assign12010_e13635_d_n11;
        locals.var_t7__blk311_dn12 = assign12010_e13635_d_n12;
        locals.var_t7__blk311_dn17 = assign12010_e13635_d_n17;

        let (assign12020_e13644, assign12020_e13644_d_n0, assign12020_e13644_d_n2, assign12020_e13644_d_n6, assign12020_e13644_d_n7, assign12020_e13644_d_n10, assign12020_e13644_d_n11, assign12020_e13644_d_n12, assign12020_e13644_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign12020_e13642: f64 = (1.0 + locals.var_t7__blk311);
        (assign12020_e13642, locals.var_t7__blk311_dn0, locals.var_t7__blk311_dn2, locals.var_t7__blk311_dn6, locals.var_t7__blk311_dn7, locals.var_t7__blk311_dn10, locals.var_t7__blk311_dn11, locals.var_t7__blk311_dn12, locals.var_t7__blk311_dn17,)
    } else {
        (locals.var_t3__blk308, locals.var_t3__blk308_dn0, locals.var_t3__blk308_dn2, locals.var_t3__blk308_dn6, locals.var_t3__blk308_dn7, locals.var_t3__blk308_dn10, locals.var_t3__blk308_dn11, locals.var_t3__blk308_dn12, locals.var_t3__blk308_dn17,)
    }
};
        locals.var_t3__blk308 = assign12020_e13644;
        locals.var_t3__blk308_dn0 = assign12020_e13644_d_n0;
        locals.var_t3__blk308_dn2 = assign12020_e13644_d_n2;
        locals.var_t3__blk308_dn6 = assign12020_e13644_d_n6;
        locals.var_t3__blk308_dn7 = assign12020_e13644_d_n7;
        locals.var_t3__blk308_dn10 = assign12020_e13644_d_n10;
        locals.var_t3__blk308_dn11 = assign12020_e13644_d_n11;
        locals.var_t3__blk308_dn12 = assign12020_e13644_d_n12;
        locals.var_t3__blk308_dn17 = assign12020_e13644_d_n17;

        let (assign12030_e13657, assign12030_e13657_d_n0, assign12030_e13657_d_n2, assign12030_e13657_d_n6, assign12030_e13657_d_n7, assign12030_e13657_d_n10, assign12030_e13657_d_n11, assign12030_e13657_d_n12, assign12030_e13657_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign12030_e13652: f64 = (1.0 / locals.var_ddlte);
        let assign12030_e13654: f64 = (assign12030_e13652 - 1.0);
        let assign12030_e13655: f64 = (locals.var_t3__blk308).powf(assign12030_e13654);
        (assign12030_e13655, if 0.0 == 0.0 && ((assign12030_e13654) as f64).is_finite() && ((assign12030_e13654) as f64).fract() == 0.0 { if assign12030_e13654 == 0.0 { 0.0 } else { (assign12030_e13654 * ((locals.var_t3__blk308).powf(assign12030_e13654 - 1.0) * locals.var_t3__blk308_dn0)) } } else { (assign12030_e13655 * (assign12030_e13654 * (locals.var_t3__blk308_dn0 / locals.var_t3__blk308))) }, if 0.0 == 0.0 && ((assign12030_e13654) as f64).is_finite() && ((assign12030_e13654) as f64).fract() == 0.0 { if assign12030_e13654 == 0.0 { 0.0 } else { (assign12030_e13654 * ((locals.var_t3__blk308).powf(assign12030_e13654 - 1.0) * locals.var_t3__blk308_dn2)) } } else { (assign12030_e13655 * (assign12030_e13654 * (locals.var_t3__blk308_dn2 / locals.var_t3__blk308))) }, if 0.0 == 0.0 && ((assign12030_e13654) as f64).is_finite() && ((assign12030_e13654) as f64).fract() == 0.0 { if assign12030_e13654 == 0.0 { 0.0 } else { (assign12030_e13654 * ((locals.var_t3__blk308).powf(assign12030_e13654 - 1.0) * locals.var_t3__blk308_dn6)) } } else { (assign12030_e13655 * (assign12030_e13654 * (locals.var_t3__blk308_dn6 / locals.var_t3__blk308))) }, if 0.0 == 0.0 && ((assign12030_e13654) as f64).is_finite() && ((assign12030_e13654) as f64).fract() == 0.0 { if assign12030_e13654 == 0.0 { 0.0 } else { (assign12030_e13654 * ((locals.var_t3__blk308).powf(assign12030_e13654 - 1.0) * locals.var_t3__blk308_dn7)) } } else { (assign12030_e13655 * (assign12030_e13654 * (locals.var_t3__blk308_dn7 / locals.var_t3__blk308))) }, if 0.0 == 0.0 && ((assign12030_e13654) as f64).is_finite() && ((assign12030_e13654) as f64).fract() == 0.0 { if assign12030_e13654 == 0.0 { 0.0 } else { (assign12030_e13654 * ((locals.var_t3__blk308).powf(assign12030_e13654 - 1.0) * locals.var_t3__blk308_dn10)) } } else { (assign12030_e13655 * (assign12030_e13654 * (locals.var_t3__blk308_dn10 / locals.var_t3__blk308))) }, if 0.0 == 0.0 && ((assign12030_e13654) as f64).is_finite() && ((assign12030_e13654) as f64).fract() == 0.0 { if assign12030_e13654 == 0.0 { 0.0 } else { (assign12030_e13654 * ((locals.var_t3__blk308).powf(assign12030_e13654 - 1.0) * locals.var_t3__blk308_dn11)) } } else { (assign12030_e13655 * (assign12030_e13654 * (locals.var_t3__blk308_dn11 / locals.var_t3__blk308))) }, if 0.0 == 0.0 && ((assign12030_e13654) as f64).is_finite() && ((assign12030_e13654) as f64).fract() == 0.0 { if assign12030_e13654 == 0.0 { 0.0 } else { (assign12030_e13654 * ((locals.var_t3__blk308).powf(assign12030_e13654 - 1.0) * locals.var_t3__blk308_dn12)) } } else { (assign12030_e13655 * (assign12030_e13654 * (locals.var_t3__blk308_dn12 / locals.var_t3__blk308))) }, if 0.0 == 0.0 && ((assign12030_e13654) as f64).is_finite() && ((assign12030_e13654) as f64).fract() == 0.0 { if assign12030_e13654 == 0.0 { 0.0 } else { (assign12030_e13654 * ((locals.var_t3__blk308).powf(assign12030_e13654 - 1.0) * locals.var_t3__blk308_dn17)) } } else { (assign12030_e13655 * (assign12030_e13654 * (locals.var_t3__blk308_dn17 / locals.var_t3__blk308))) },)
    } else {
        (locals.var_t4__blk309, locals.var_t4__blk309_dn0, locals.var_t4__blk309_dn2, locals.var_t4__blk309_dn6, locals.var_t4__blk309_dn7, locals.var_t4__blk309_dn10, locals.var_t4__blk309_dn11, locals.var_t4__blk309_dn12, locals.var_t4__blk309_dn17,)
    }
};
        locals.var_t4__blk309 = assign12030_e13657;
        locals.var_t4__blk309_dn0 = assign12030_e13657_d_n0;
        locals.var_t4__blk309_dn2 = assign12030_e13657_d_n2;
        locals.var_t4__blk309_dn6 = assign12030_e13657_d_n6;
        locals.var_t4__blk309_dn7 = assign12030_e13657_d_n7;
        locals.var_t4__blk309_dn10 = assign12030_e13657_d_n10;
        locals.var_t4__blk309_dn11 = assign12030_e13657_d_n11;
        locals.var_t4__blk309_dn12 = assign12030_e13657_d_n12;
        locals.var_t4__blk309_dn17 = assign12030_e13657_d_n17;

        let (assign12040_e13666, assign12040_e13666_d_n0, assign12040_e13666_d_n2, assign12040_e13666_d_n6, assign12040_e13666_d_n7, assign12040_e13666_d_n10, assign12040_e13666_d_n11, assign12040_e13666_d_n12, assign12040_e13666_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign12040_e13664: f64 = (locals.var_t4__blk309 * locals.var_t3__blk308);
        (assign12040_e13664, ((locals.var_t4__blk309_dn0 * locals.var_t3__blk308) + (locals.var_t4__blk309 * locals.var_t3__blk308_dn0)), ((locals.var_t4__blk309_dn2 * locals.var_t3__blk308) + (locals.var_t4__blk309 * locals.var_t3__blk308_dn2)), ((locals.var_t4__blk309_dn6 * locals.var_t3__blk308) + (locals.var_t4__blk309 * locals.var_t3__blk308_dn6)), ((locals.var_t4__blk309_dn7 * locals.var_t3__blk308) + (locals.var_t4__blk309 * locals.var_t3__blk308_dn7)), ((locals.var_t4__blk309_dn10 * locals.var_t3__blk308) + (locals.var_t4__blk309 * locals.var_t3__blk308_dn10)), ((locals.var_t4__blk309_dn11 * locals.var_t3__blk308) + (locals.var_t4__blk309 * locals.var_t3__blk308_dn11)), ((locals.var_t4__blk309_dn12 * locals.var_t3__blk308) + (locals.var_t4__blk309 * locals.var_t3__blk308_dn12)), ((locals.var_t4__blk309_dn17 * locals.var_t3__blk308) + (locals.var_t4__blk309 * locals.var_t3__blk308_dn17)),)
    } else {
        (locals.var_t6__blk310, locals.var_t6__blk310_dn0, locals.var_t6__blk310_dn2, locals.var_t6__blk310_dn6, locals.var_t6__blk310_dn7, locals.var_t6__blk310_dn10, locals.var_t6__blk310_dn11, locals.var_t6__blk310_dn12, locals.var_t6__blk310_dn17,)
    }
};
        locals.var_t6__blk310 = assign12040_e13666;
        locals.var_t6__blk310_dn0 = assign12040_e13666_d_n0;
        locals.var_t6__blk310_dn2 = assign12040_e13666_d_n2;
        locals.var_t6__blk310_dn6 = assign12040_e13666_d_n6;
        locals.var_t6__blk310_dn7 = assign12040_e13666_d_n7;
        locals.var_t6__blk310_dn10 = assign12040_e13666_d_n10;
        locals.var_t6__blk310_dn11 = assign12040_e13666_d_n11;
        locals.var_t6__blk310_dn12 = assign12040_e13666_d_n12;
        locals.var_t6__blk310_dn17 = assign12040_e13666_d_n17;

        let (assign12050_e13675, assign12050_e13675_d_n0, assign12050_e13675_d_n2, assign12050_e13675_d_n6, assign12050_e13675_d_n7, assign12050_e13675_d_n10, assign12050_e13675_d_n11, assign12050_e13675_d_n12, assign12050_e13675_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign12050_e13673: f64 = (locals.var_vds / locals.var_t6__blk310);
        (assign12050_e13673, (((locals.var_vds_dn0 * locals.var_t6__blk310) - (locals.var_vds * locals.var_t6__blk310_dn0)) / (locals.var_t6__blk310 * locals.var_t6__blk310)), (((locals.var_vds_dn2 * locals.var_t6__blk310) - (locals.var_vds * locals.var_t6__blk310_dn2)) / (locals.var_t6__blk310 * locals.var_t6__blk310)), (((locals.var_vds_dn6 * locals.var_t6__blk310) - (locals.var_vds * locals.var_t6__blk310_dn6)) / (locals.var_t6__blk310 * locals.var_t6__blk310)), (((locals.var_vds_dn7 * locals.var_t6__blk310) - (locals.var_vds * locals.var_t6__blk310_dn7)) / (locals.var_t6__blk310 * locals.var_t6__blk310)), (((locals.var_vds_dn10 * locals.var_t6__blk310) - (locals.var_vds * locals.var_t6__blk310_dn10)) / (locals.var_t6__blk310 * locals.var_t6__blk310)), (((locals.var_vds_dn11 * locals.var_t6__blk310) - (locals.var_vds * locals.var_t6__blk310_dn11)) / (locals.var_t6__blk310 * locals.var_t6__blk310)), (((locals.var_vds_dn12 * locals.var_t6__blk310) - (locals.var_vds * locals.var_t6__blk310_dn12)) / (locals.var_t6__blk310 * locals.var_t6__blk310)), (((locals.var_vds_dn17 * locals.var_t6__blk310) - (locals.var_vds * locals.var_t6__blk310_dn17)) / (locals.var_t6__blk310 * locals.var_t6__blk310)),)
    } else {
        (locals.var_vdseff, locals.var_vdseff_dn0, locals.var_vdseff_dn2, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn10, locals.var_vdseff_dn11, locals.var_vdseff_dn12, locals.var_vdseff_dn17,)
    }
};
        locals.var_vdseff = assign12050_e13675;
        locals.var_vdseff_dn0 = assign12050_e13675_d_n0;
        locals.var_vdseff_dn2 = assign12050_e13675_d_n2;
        locals.var_vdseff_dn6 = assign12050_e13675_d_n6;
        locals.var_vdseff_dn7 = assign12050_e13675_d_n7;
        locals.var_vdseff_dn10 = assign12050_e13675_d_n10;
        locals.var_vdseff_dn11 = assign12050_e13675_d_n11;
        locals.var_vdseff_dn12 = assign12050_e13675_d_n12;
        locals.var_vdseff_dn17 = assign12050_e13675_d_n17;

        let (assign12060_e13682, assign12060_e13682_d_n0, assign12060_e13682_d_n2, assign12060_e13682_d_n6, assign12060_e13682_d_n7, assign12060_e13682_d_n10, assign12060_e13682_d_n11, assign12060_e13682_d_n12, assign12060_e13682_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        (locals.var_vdseff, locals.var_vdseff_dn0, locals.var_vdseff_dn2, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn10, locals.var_vdseff_dn11, locals.var_vdseff_dn12, locals.var_vdseff_dn17,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    }
};
        locals.var_vds = assign12060_e13682;
        locals.var_vds_dn0 = assign12060_e13682_d_n0;
        locals.var_vds_dn2 = assign12060_e13682_d_n2;
        locals.var_vds_dn6 = assign12060_e13682_d_n6;
        locals.var_vds_dn7 = assign12060_e13682_d_n7;
        locals.var_vds_dn10 = assign12060_e13682_d_n10;
        locals.var_vds_dn11 = assign12060_e13682_d_n11;
        locals.var_vds_dn12 = assign12060_e13682_d_n12;
        locals.var_vds_dn17 = assign12060_e13682_d_n17;

        let assign12070_e13685: f64 = if locals.var_vds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard320 = assign12070_e13685;

        let (assign12080_e13694, assign12080_e13694_d_n0, assign12080_e13694_d_n2, assign12080_e13694_d_n6, assign12080_e13694_d_n7, assign12080_e13694_d_n10, assign12080_e13694_d_n11, assign12080_e13694_d_n12, assign12080_e13694_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 != 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    }
};
        locals.var_psl = assign12080_e13694;
        locals.var_psl_dn0 = assign12080_e13694_d_n0;
        locals.var_psl_dn2 = assign12080_e13694_d_n2;
        locals.var_psl_dn6 = assign12080_e13694_d_n6;
        locals.var_psl_dn7 = assign12080_e13694_d_n7;
        locals.var_psl_dn10 = assign12080_e13694_d_n10;
        locals.var_psl_dn11 = assign12080_e13694_d_n11;
        locals.var_psl_dn12 = assign12080_e13694_d_n12;
        locals.var_psl_dn17 = assign12080_e13694_d_n17;

        let (assign12090_e13705, assign12090_e13705_d_n0, assign12090_e13705_d_n2, assign12090_e13705_d_n6, assign12090_e13705_d_n7, assign12090_e13705_d_n10, assign12090_e13705_d_n11, assign12090_e13705_d_n12, assign12090_e13705_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign12090_e13703: f64 = (locals.var_psl - locals.var_ps0);
        (assign12090_e13703, (locals.var_psl_dn0 - locals.var_ps0_dn0), (locals.var_psl_dn2 - locals.var_ps0_dn2), (locals.var_psl_dn6 - locals.var_ps0_dn6), (locals.var_psl_dn7 - locals.var_ps0_dn7), (locals.var_psl_dn10 - locals.var_ps0_dn10), (locals.var_psl_dn11 - locals.var_ps0_dn11), (locals.var_psl_dn12 - locals.var_ps0_dn12), (locals.var_psl_dn17 - locals.var_ps0_dn17),)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn12, locals.var_pds_dn17,)
    }
};
        locals.var_pds = assign12090_e13705;
        locals.var_pds_dn0 = assign12090_e13705_d_n0;
        locals.var_pds_dn2 = assign12090_e13705_d_n2;
        locals.var_pds_dn6 = assign12090_e13705_d_n6;
        locals.var_pds_dn7 = assign12090_e13705_d_n7;
        locals.var_pds_dn10 = assign12090_e13705_d_n10;
        locals.var_pds_dn11 = assign12090_e13705_d_n11;
        locals.var_pds_dn12 = assign12090_e13705_d_n12;
        locals.var_pds_dn17 = assign12090_e13705_d_n17;

        let (assign12100_e13714, assign12100_e13714_d_n0, assign12100_e13714_d_n2, assign12100_e13714_d_n6, assign12100_e13714_d_n7, assign12100_e13714_d_n10, assign12100_e13714_d_n11, assign12100_e13714_d_n12, assign12100_e13714_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 != 0.0)) {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
        locals.var_phi_sl_soi = assign12100_e13714;
        locals.var_phi_sl_soi_dn0 = assign12100_e13714_d_n0;
        locals.var_phi_sl_soi_dn2 = assign12100_e13714_d_n2;
        locals.var_phi_sl_soi_dn6 = assign12100_e13714_d_n6;
        locals.var_phi_sl_soi_dn7 = assign12100_e13714_d_n7;
        locals.var_phi_sl_soi_dn10 = assign12100_e13714_d_n10;
        locals.var_phi_sl_soi_dn11 = assign12100_e13714_d_n11;
        locals.var_phi_sl_soi_dn12 = assign12100_e13714_d_n12;
        locals.var_phi_sl_soi_dn17 = assign12100_e13714_d_n17;

        let (assign12110_e13723, assign12110_e13723_d_n0, assign12110_e13723_d_n2, assign12110_e13723_d_n6, assign12110_e13723_d_n7, assign12110_e13723_d_n10, assign12110_e13723_d_n11, assign12110_e13723_d_n12, assign12110_e13723_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 != 0.0)) {
        (locals.var_phi_b0_soi, locals.var_phi_b0_soi_dn0, locals.var_phi_b0_soi_dn2, locals.var_phi_b0_soi_dn6, locals.var_phi_b0_soi_dn7, locals.var_phi_b0_soi_dn10, locals.var_phi_b0_soi_dn11, locals.var_phi_b0_soi_dn12, locals.var_phi_b0_soi_dn17,)
    } else {
        (locals.var_phi_bl_soi, locals.var_phi_bl_soi_dn0, locals.var_phi_bl_soi_dn2, locals.var_phi_bl_soi_dn6, locals.var_phi_bl_soi_dn7, locals.var_phi_bl_soi_dn10, locals.var_phi_bl_soi_dn11, locals.var_phi_bl_soi_dn12, locals.var_phi_bl_soi_dn17,)
    }
};
        locals.var_phi_bl_soi = assign12110_e13723;
        locals.var_phi_bl_soi_dn0 = assign12110_e13723_d_n0;
        locals.var_phi_bl_soi_dn2 = assign12110_e13723_d_n2;
        locals.var_phi_bl_soi_dn6 = assign12110_e13723_d_n6;
        locals.var_phi_bl_soi_dn7 = assign12110_e13723_d_n7;
        locals.var_phi_bl_soi_dn10 = assign12110_e13723_d_n10;
        locals.var_phi_bl_soi_dn11 = assign12110_e13723_d_n11;
        locals.var_phi_bl_soi_dn12 = assign12110_e13723_d_n12;
        locals.var_phi_bl_soi_dn17 = assign12110_e13723_d_n17;

        let (assign12120_e13732, assign12120_e13732_d_n0, assign12120_e13732_d_n2, assign12120_e13732_d_n6, assign12120_e13732_d_n7, assign12120_e13732_d_n10, assign12120_e13732_d_n11, assign12120_e13732_d_n12, assign12120_e13732_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 != 0.0)) {
        (locals.var_phi_s0_bulk, locals.var_phi_s0_bulk_dn0, locals.var_phi_s0_bulk_dn2, locals.var_phi_s0_bulk_dn6, locals.var_phi_s0_bulk_dn7, locals.var_phi_s0_bulk_dn10, locals.var_phi_s0_bulk_dn11, locals.var_phi_s0_bulk_dn12, locals.var_phi_s0_bulk_dn17,)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
        locals.var_phi_sl_bulk = assign12120_e13732;
        locals.var_phi_sl_bulk_dn0 = assign12120_e13732_d_n0;
        locals.var_phi_sl_bulk_dn2 = assign12120_e13732_d_n2;
        locals.var_phi_sl_bulk_dn6 = assign12120_e13732_d_n6;
        locals.var_phi_sl_bulk_dn7 = assign12120_e13732_d_n7;
        locals.var_phi_sl_bulk_dn10 = assign12120_e13732_d_n10;
        locals.var_phi_sl_bulk_dn11 = assign12120_e13732_d_n11;
        locals.var_phi_sl_bulk_dn12 = assign12120_e13732_d_n12;
        locals.var_phi_sl_bulk_dn17 = assign12120_e13732_d_n17;

        let (assign12130_e13741,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign12130_e13741;

        let assign12150_e13753: f64 = if locals.var_flg_pprv >= 1.0 { 1.0 } else { 0.0 };
        locals.var_guard321 = assign12150_e13753;

        let (assign12160_e13765, assign12160_e13765_d_n0, assign12160_e13765_d_n2, assign12160_e13765_d_n6, assign12160_e13765_d_n7, assign12160_e13765_d_n10, assign12160_e13765_d_n11, assign12160_e13765_d_n12, assign12160_e13765_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 != 0.0)) {
        (locals.var_pssl_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
        locals.var_phi_sl_soi = assign12160_e13765;
        locals.var_phi_sl_soi_dn0 = assign12160_e13765_d_n0;
        locals.var_phi_sl_soi_dn2 = assign12160_e13765_d_n2;
        locals.var_phi_sl_soi_dn6 = assign12160_e13765_d_n6;
        locals.var_phi_sl_soi_dn7 = assign12160_e13765_d_n7;
        locals.var_phi_sl_soi_dn10 = assign12160_e13765_d_n10;
        locals.var_phi_sl_soi_dn11 = assign12160_e13765_d_n11;
        locals.var_phi_sl_soi_dn12 = assign12160_e13765_d_n12;
        locals.var_phi_sl_soi_dn17 = assign12160_e13765_d_n17;

        let (assign12170_e13777, assign12170_e13777_d_n0, assign12170_e13777_d_n2, assign12170_e13777_d_n6, assign12170_e13777_d_n7, assign12170_e13777_d_n10, assign12170_e13777_d_n11, assign12170_e13777_d_n12, assign12170_e13777_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 != 0.0)) {
        (locals.var_pbsl_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_bl_soi, locals.var_phi_bl_soi_dn0, locals.var_phi_bl_soi_dn2, locals.var_phi_bl_soi_dn6, locals.var_phi_bl_soi_dn7, locals.var_phi_bl_soi_dn10, locals.var_phi_bl_soi_dn11, locals.var_phi_bl_soi_dn12, locals.var_phi_bl_soi_dn17,)
    }
};
        locals.var_phi_bl_soi = assign12170_e13777;
        locals.var_phi_bl_soi_dn0 = assign12170_e13777_d_n0;
        locals.var_phi_bl_soi_dn2 = assign12170_e13777_d_n2;
        locals.var_phi_bl_soi_dn6 = assign12170_e13777_d_n6;
        locals.var_phi_bl_soi_dn7 = assign12170_e13777_d_n7;
        locals.var_phi_bl_soi_dn10 = assign12170_e13777_d_n10;
        locals.var_phi_bl_soi_dn11 = assign12170_e13777_d_n11;
        locals.var_phi_bl_soi_dn12 = assign12170_e13777_d_n12;
        locals.var_phi_bl_soi_dn17 = assign12170_e13777_d_n17;

        let (assign12180_e13789, assign12180_e13789_d_n0, assign12180_e13789_d_n2, assign12180_e13789_d_n6, assign12180_e13789_d_n7, assign12180_e13789_d_n10, assign12180_e13789_d_n11, assign12180_e13789_d_n12, assign12180_e13789_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 != 0.0)) {
        (locals.var_psbl_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
        locals.var_phi_sl_bulk = assign12180_e13789;
        locals.var_phi_sl_bulk_dn0 = assign12180_e13789_d_n0;
        locals.var_phi_sl_bulk_dn2 = assign12180_e13789_d_n2;
        locals.var_phi_sl_bulk_dn6 = assign12180_e13789_d_n6;
        locals.var_phi_sl_bulk_dn7 = assign12180_e13789_d_n7;
        locals.var_phi_sl_bulk_dn10 = assign12180_e13789_d_n10;
        locals.var_phi_sl_bulk_dn11 = assign12180_e13789_d_n11;
        locals.var_phi_sl_bulk_dn12 = assign12180_e13789_d_n12;
        locals.var_phi_sl_bulk_dn17 = assign12180_e13789_d_n17;

    }

    pub(super) fn stamp_transient_block_35(
        locals: &mut StampLocals,
    ) {
        let (assign12200_e13828, assign12200_e13828_d_n0, assign12200_e13828_d_n2, assign12200_e13828_d_n6, assign12200_e13828_d_n7, assign12200_e13828_d_n10, assign12200_e13828_d_n11, assign12200_e13828_d_n12, assign12200_e13828_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) {
        let assign12200_e13819: f64 = (locals.var_psl_lim - locals.var_phi_s0_soi);
        let (assign12200_e13826, assign12200_e13826_d_n0, assign12200_e13826_d_n2, assign12200_e13826_d_n6, assign12200_e13826_d_n7, assign12200_e13826_d_n10, assign12200_e13826_d_n11, assign12200_e13826_d_n12, assign12200_e13826_d_n17,) = {
            if (assign12200_e13819 >= 0.0) {
                let assign12200_e13824: f64 = (locals.var_psl_lim - locals.var_phi_s0_soi);
                (assign12200_e13824, (locals.var_psl_lim_dn0 - locals.var_phi_s0_soi_dn0), (locals.var_psl_lim_dn2 - locals.var_phi_s0_soi_dn2), (locals.var_psl_lim_dn6 - locals.var_phi_s0_soi_dn6), (locals.var_psl_lim_dn7 - locals.var_phi_s0_soi_dn7), (locals.var_psl_lim_dn10 - locals.var_phi_s0_soi_dn10), (locals.var_psl_lim_dn11 - locals.var_phi_s0_soi_dn11), (locals.var_psl_lim_dn12 - locals.var_phi_s0_soi_dn12), (locals.var_psl_lim_dn17 - locals.var_phi_s0_soi_dn17),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign12200_e13826, assign12200_e13826_d_n0, assign12200_e13826_d_n2, assign12200_e13826_d_n6, assign12200_e13826_d_n7, assign12200_e13826_d_n10, assign12200_e13826_d_n11, assign12200_e13826_d_n12, assign12200_e13826_d_n17,)
    } else {
        (locals.var_pds_max, locals.var_pds_max_dn0, locals.var_pds_max_dn2, locals.var_pds_max_dn6, locals.var_pds_max_dn7, locals.var_pds_max_dn10, locals.var_pds_max_dn11, locals.var_pds_max_dn12, locals.var_pds_max_dn17,)
    }
};
        locals.var_pds_max = assign12200_e13828;
        locals.var_pds_max_dn0 = assign12200_e13828_d_n0;
        locals.var_pds_max_dn2 = assign12200_e13828_d_n2;
        locals.var_pds_max_dn6 = assign12200_e13828_d_n6;
        locals.var_pds_max_dn7 = assign12200_e13828_d_n7;
        locals.var_pds_max_dn10 = assign12200_e13828_d_n10;
        locals.var_pds_max_dn11 = assign12200_e13828_d_n11;
        locals.var_pds_max_dn12 = assign12200_e13828_d_n12;
        locals.var_pds_max_dn17 = assign12200_e13828_d_n17;

        let (assign12210_e13849, assign12210_e13849_d_n0, assign12210_e13849_d_n2, assign12210_e13849_d_n6, assign12210_e13849_d_n7, assign12210_e13849_d_n10, assign12210_e13849_d_n11, assign12210_e13849_d_n12, assign12210_e13849_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) {
        let assign12210_e13841: f64 = (1.0 + 0.3);
        let assign12210_e13843: f64 = (assign12210_e13841 * locals.var_pds_max);
        let assign12210_e13845: f64 = (assign12210_e13843 - locals.var_vds);
        let assign12210_e13847: f64 = (assign12210_e13845 - 0.03);
        (assign12210_e13847, ((assign12210_e13841 * locals.var_pds_max_dn0) - locals.var_vds_dn0), ((assign12210_e13841 * locals.var_pds_max_dn2) - locals.var_vds_dn2), ((assign12210_e13841 * locals.var_pds_max_dn6) - locals.var_vds_dn6), ((assign12210_e13841 * locals.var_pds_max_dn7) - locals.var_vds_dn7), ((assign12210_e13841 * locals.var_pds_max_dn10) - locals.var_vds_dn10), ((assign12210_e13841 * locals.var_pds_max_dn11) - locals.var_vds_dn11), ((assign12210_e13841 * locals.var_pds_max_dn12) - locals.var_vds_dn12), ((assign12210_e13841 * locals.var_pds_max_dn17) - locals.var_vds_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign12210_e13849;
        locals.var_tmf1_dn0 = assign12210_e13849_d_n0;
        locals.var_tmf1_dn2 = assign12210_e13849_d_n2;
        locals.var_tmf1_dn6 = assign12210_e13849_d_n6;
        locals.var_tmf1_dn7 = assign12210_e13849_d_n7;
        locals.var_tmf1_dn10 = assign12210_e13849_d_n10;
        locals.var_tmf1_dn11 = assign12210_e13849_d_n11;
        locals.var_tmf1_dn12 = assign12210_e13849_d_n12;
        locals.var_tmf1_dn17 = assign12210_e13849_d_n17;

        let (assign12220_e13870, assign12220_e13870_d_n0, assign12220_e13870_d_n2, assign12220_e13870_d_n6, assign12220_e13870_d_n7, assign12220_e13870_d_n10, assign12220_e13870_d_n11, assign12220_e13870_d_n12, assign12220_e13870_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) {
        let assign12220_e13863: f64 = (1.0 + 0.3);
        let assign12220_e13865: f64 = (assign12220_e13863 * locals.var_pds_max);
        let assign12220_e13866: f64 = (4.0 * assign12220_e13865);
        let assign12220_e13868: f64 = (assign12220_e13866 * 0.03);
        (assign12220_e13868, ((4.0 * (assign12220_e13863 * locals.var_pds_max_dn0)) * 0.03), ((4.0 * (assign12220_e13863 * locals.var_pds_max_dn2)) * 0.03), ((4.0 * (assign12220_e13863 * locals.var_pds_max_dn6)) * 0.03), ((4.0 * (assign12220_e13863 * locals.var_pds_max_dn7)) * 0.03), ((4.0 * (assign12220_e13863 * locals.var_pds_max_dn10)) * 0.03), ((4.0 * (assign12220_e13863 * locals.var_pds_max_dn11)) * 0.03), ((4.0 * (assign12220_e13863 * locals.var_pds_max_dn12)) * 0.03), ((4.0 * (assign12220_e13863 * locals.var_pds_max_dn17)) * 0.03),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign12220_e13870;
        locals.var_tmf2_dn0 = assign12220_e13870_d_n0;
        locals.var_tmf2_dn2 = assign12220_e13870_d_n2;
        locals.var_tmf2_dn6 = assign12220_e13870_d_n6;
        locals.var_tmf2_dn7 = assign12220_e13870_d_n7;
        locals.var_tmf2_dn10 = assign12220_e13870_d_n10;
        locals.var_tmf2_dn11 = assign12220_e13870_d_n11;
        locals.var_tmf2_dn12 = assign12220_e13870_d_n12;
        locals.var_tmf2_dn17 = assign12220_e13870_d_n17;

        let (assign12230_e13889, assign12230_e13889_d_n0, assign12230_e13889_d_n2, assign12230_e13889_d_n6, assign12230_e13889_d_n7, assign12230_e13889_d_n10, assign12230_e13889_d_n11, assign12230_e13889_d_n12, assign12230_e13889_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) {
        let (assign12230_e13887, assign12230_e13887_d_n0, assign12230_e13887_d_n2, assign12230_e13887_d_n6, assign12230_e13887_d_n7, assign12230_e13887_d_n10, assign12230_e13887_d_n11, assign12230_e13887_d_n12, assign12230_e13887_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign12230_e13886: f64 = (-locals.var_tmf2);
                (assign12230_e13886, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign12230_e13887, assign12230_e13887_d_n0, assign12230_e13887_d_n2, assign12230_e13887_d_n6, assign12230_e13887_d_n7, assign12230_e13887_d_n10, assign12230_e13887_d_n11, assign12230_e13887_d_n12, assign12230_e13887_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign12230_e13889;
        locals.var_tmf2_dn0 = assign12230_e13889_d_n0;
        locals.var_tmf2_dn2 = assign12230_e13889_d_n2;
        locals.var_tmf2_dn6 = assign12230_e13889_d_n6;
        locals.var_tmf2_dn7 = assign12230_e13889_d_n7;
        locals.var_tmf2_dn10 = assign12230_e13889_d_n10;
        locals.var_tmf2_dn11 = assign12230_e13889_d_n11;
        locals.var_tmf2_dn12 = assign12230_e13889_d_n12;
        locals.var_tmf2_dn17 = assign12230_e13889_d_n17;

        let (assign12240_e13907, assign12240_e13907_d_n0, assign12240_e13907_d_n2, assign12240_e13907_d_n6, assign12240_e13907_d_n7, assign12240_e13907_d_n10, assign12240_e13907_d_n11, assign12240_e13907_d_n12, assign12240_e13907_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) {
        let assign12240_e13902: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign12240_e13904: f64 = (assign12240_e13902 + locals.var_tmf2);
        let assign12240_e13905: f64 = (assign12240_e13904).sqrt();
        (assign12240_e13905, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign12240_e13905)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign12240_e13905)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign12240_e13905)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign12240_e13905)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign12240_e13905)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign12240_e13905)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign12240_e13905)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign12240_e13905)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign12240_e13907;
        locals.var_tmf2_dn0 = assign12240_e13907_d_n0;
        locals.var_tmf2_dn2 = assign12240_e13907_d_n2;
        locals.var_tmf2_dn6 = assign12240_e13907_d_n6;
        locals.var_tmf2_dn7 = assign12240_e13907_d_n7;
        locals.var_tmf2_dn10 = assign12240_e13907_d_n10;
        locals.var_tmf2_dn11 = assign12240_e13907_d_n11;
        locals.var_tmf2_dn12 = assign12240_e13907_d_n12;
        locals.var_tmf2_dn17 = assign12240_e13907_d_n17;

        let (assign12250_e13930, assign12250_e13930_d_n0, assign12250_e13930_d_n2, assign12250_e13930_d_n6, assign12250_e13930_d_n7, assign12250_e13930_d_n10, assign12250_e13930_d_n11, assign12250_e13930_d_n12, assign12250_e13930_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) {
        let assign12250_e13920: f64 = (1.0 + 0.3);
        let assign12250_e13922: f64 = (assign12250_e13920 * locals.var_pds_max);
        let assign12250_e13926: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign12250_e13927: f64 = (0.5 * assign12250_e13926);
        let assign12250_e13928: f64 = (assign12250_e13922 - assign12250_e13927);
        (assign12250_e13928, ((assign12250_e13920 * locals.var_pds_max_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((assign12250_e13920 * locals.var_pds_max_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((assign12250_e13920 * locals.var_pds_max_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((assign12250_e13920 * locals.var_pds_max_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((assign12250_e13920 * locals.var_pds_max_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((assign12250_e13920 * locals.var_pds_max_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((assign12250_e13920 * locals.var_pds_max_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((assign12250_e13920 * locals.var_pds_max_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign12250_e13930;
        locals.var_pds_ini_dn0 = assign12250_e13930_d_n0;
        locals.var_pds_ini_dn2 = assign12250_e13930_d_n2;
        locals.var_pds_ini_dn6 = assign12250_e13930_d_n6;
        locals.var_pds_ini_dn7 = assign12250_e13930_d_n7;
        locals.var_pds_ini_dn10 = assign12250_e13930_d_n10;
        locals.var_pds_ini_dn11 = assign12250_e13930_d_n11;
        locals.var_pds_ini_dn12 = assign12250_e13930_d_n12;
        locals.var_pds_ini_dn17 = assign12250_e13930_d_n17;

        let (assign12260_e13948, assign12260_e13948_d_n0, assign12260_e13948_d_n2, assign12260_e13948_d_n6, assign12260_e13948_d_n7, assign12260_e13948_d_n10, assign12260_e13948_d_n11, assign12260_e13948_d_n12, assign12260_e13948_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) {
        let (assign12260_e13946, assign12260_e13946_d_n0, assign12260_e13946_d_n2, assign12260_e13946_d_n6, assign12260_e13946_d_n7, assign12260_e13946_d_n10, assign12260_e13946_d_n11, assign12260_e13946_d_n12, assign12260_e13946_d_n17,) = {
            if (locals.var_pds_ini <= locals.var_pds_max) {
                (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
            } else {
                (locals.var_pds_max, locals.var_pds_max_dn0, locals.var_pds_max_dn2, locals.var_pds_max_dn6, locals.var_pds_max_dn7, locals.var_pds_max_dn10, locals.var_pds_max_dn11, locals.var_pds_max_dn12, locals.var_pds_max_dn17,)
            }
        };
        (assign12260_e13946, assign12260_e13946_d_n0, assign12260_e13946_d_n2, assign12260_e13946_d_n6, assign12260_e13946_d_n7, assign12260_e13946_d_n10, assign12260_e13946_d_n11, assign12260_e13946_d_n12, assign12260_e13946_d_n17,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign12260_e13948;
        locals.var_pds_ini_dn0 = assign12260_e13948_d_n0;
        locals.var_pds_ini_dn2 = assign12260_e13948_d_n2;
        locals.var_pds_ini_dn6 = assign12260_e13948_d_n6;
        locals.var_pds_ini_dn7 = assign12260_e13948_d_n7;
        locals.var_pds_ini_dn10 = assign12260_e13948_d_n10;
        locals.var_pds_ini_dn11 = assign12260_e13948_d_n11;
        locals.var_pds_ini_dn12 = assign12260_e13948_d_n12;
        locals.var_pds_ini_dn17 = assign12260_e13948_d_n17;

        let assign12270_e13951: f64 = if locals.var_pds_ini < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard322 = assign12270_e13951;

        let (assign12280_e13966, assign12280_e13966_d_n0, assign12280_e13966_d_n2, assign12280_e13966_d_n6, assign12280_e13966_d_n7, assign12280_e13966_d_n10, assign12280_e13966_d_n11, assign12280_e13966_d_n12, assign12280_e13966_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard322 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign12280_e13966;
        locals.var_pds_ini_dn0 = assign12280_e13966_d_n0;
        locals.var_pds_ini_dn2 = assign12280_e13966_d_n2;
        locals.var_pds_ini_dn6 = assign12280_e13966_d_n6;
        locals.var_pds_ini_dn7 = assign12280_e13966_d_n7;
        locals.var_pds_ini_dn10 = assign12280_e13966_d_n10;
        locals.var_pds_ini_dn11 = assign12280_e13966_d_n11;
        locals.var_pds_ini_dn12 = assign12280_e13966_d_n12;
        locals.var_pds_ini_dn17 = assign12280_e13966_d_n17;

        let assign12290_e13969: f64 = if locals.var_pds_ini > locals.var_vds { 1.0 } else { 0.0 };
        locals.var_guard323 = assign12290_e13969;

        let (assign12300_e13987, assign12300_e13987_d_n0, assign12300_e13987_d_n2, assign12300_e13987_d_n6, assign12300_e13987_d_n7, assign12300_e13987_d_n10, assign12300_e13987_d_n11, assign12300_e13987_d_n12, assign12300_e13987_d_n17,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard322 == 0.0)) && (locals.var_guard323 != 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign12300_e13987;
        locals.var_pds_ini_dn0 = assign12300_e13987_d_n0;
        locals.var_pds_ini_dn2 = assign12300_e13987_d_n2;
        locals.var_pds_ini_dn6 = assign12300_e13987_d_n6;
        locals.var_pds_ini_dn7 = assign12300_e13987_d_n7;
        locals.var_pds_ini_dn10 = assign12300_e13987_d_n10;
        locals.var_pds_ini_dn11 = assign12300_e13987_d_n11;
        locals.var_pds_ini_dn12 = assign12300_e13987_d_n12;
        locals.var_pds_ini_dn17 = assign12300_e13987_d_n17;

        let (assign12310_e14000, assign12310_e14000_d_n0, assign12310_e14000_d_n2, assign12310_e14000_d_n6, assign12310_e14000_d_n7, assign12310_e14000_d_n10, assign12310_e14000_d_n11, assign12310_e14000_d_n12, assign12310_e14000_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn12, locals.var_pds_dn17,)
    }
};
        locals.var_pds = assign12310_e14000;
        locals.var_pds_dn0 = assign12310_e14000_d_n0;
        locals.var_pds_dn2 = assign12310_e14000_d_n2;
        locals.var_pds_dn6 = assign12310_e14000_d_n6;
        locals.var_pds_dn7 = assign12310_e14000_d_n7;
        locals.var_pds_dn10 = assign12310_e14000_d_n10;
        locals.var_pds_dn11 = assign12310_e14000_d_n11;
        locals.var_pds_dn12 = assign12310_e14000_d_n12;
        locals.var_pds_dn17 = assign12310_e14000_d_n17;

        let (assign12320_e14015, assign12320_e14015_d_n0, assign12320_e14015_d_n2, assign12320_e14015_d_n6, assign12320_e14015_d_n7, assign12320_e14015_d_n10, assign12320_e14015_d_n11, assign12320_e14015_d_n12, assign12320_e14015_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) {
        let assign12320_e14013: f64 = (locals.var_phi_s0_soi + locals.var_pds);
        (assign12320_e14013, (locals.var_phi_s0_soi_dn0 + locals.var_pds_dn0), (locals.var_phi_s0_soi_dn2 + locals.var_pds_dn2), (locals.var_phi_s0_soi_dn6 + locals.var_pds_dn6), (locals.var_phi_s0_soi_dn7 + locals.var_pds_dn7), (locals.var_phi_s0_soi_dn10 + locals.var_pds_dn10), (locals.var_phi_s0_soi_dn11 + locals.var_pds_dn11), (locals.var_phi_s0_soi_dn12 + locals.var_pds_dn12), (locals.var_phi_s0_soi_dn17 + locals.var_pds_dn17),)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    }
};
        locals.var_psl = assign12320_e14015;
        locals.var_psl_dn0 = assign12320_e14015_d_n0;
        locals.var_psl_dn2 = assign12320_e14015_d_n2;
        locals.var_psl_dn6 = assign12320_e14015_d_n6;
        locals.var_psl_dn7 = assign12320_e14015_d_n7;
        locals.var_psl_dn10 = assign12320_e14015_d_n10;
        locals.var_psl_dn11 = assign12320_e14015_d_n11;
        locals.var_psl_dn12 = assign12320_e14015_d_n12;
        locals.var_psl_dn17 = assign12320_e14015_d_n17;

        let (assign12330_e14028, assign12330_e14028_d_n0, assign12330_e14028_d_n2, assign12330_e14028_d_n6, assign12330_e14028_d_n7, assign12330_e14028_d_n10, assign12330_e14028_d_n11, assign12330_e14028_d_n12, assign12330_e14028_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
        locals.var_phi_sl_soi = assign12330_e14028;
        locals.var_phi_sl_soi_dn0 = assign12330_e14028_d_n0;
        locals.var_phi_sl_soi_dn2 = assign12330_e14028_d_n2;
        locals.var_phi_sl_soi_dn6 = assign12330_e14028_d_n6;
        locals.var_phi_sl_soi_dn7 = assign12330_e14028_d_n7;
        locals.var_phi_sl_soi_dn10 = assign12330_e14028_d_n10;
        locals.var_phi_sl_soi_dn11 = assign12330_e14028_d_n11;
        locals.var_phi_sl_soi_dn12 = assign12330_e14028_d_n12;
        locals.var_phi_sl_soi_dn17 = assign12330_e14028_d_n17;

        let (assign12340_e14041, assign12340_e14041_d_n0, assign12340_e14041_d_n2, assign12340_e14041_d_n6, assign12340_e14041_d_n7, assign12340_e14041_d_n10, assign12340_e14041_d_n11, assign12340_e14041_d_n12, assign12340_e14041_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) {
        (locals.var_phi_b_dep0, locals.var_phi_b_dep0_dn0, locals.var_phi_b_dep0_dn2, locals.var_phi_b_dep0_dn6, locals.var_phi_b_dep0_dn7, locals.var_phi_b_dep0_dn10, locals.var_phi_b_dep0_dn11, locals.var_phi_b_dep0_dn12, locals.var_phi_b_dep0_dn17,)
    } else {
        (locals.var_phi_b_dep, locals.var_phi_b_dep_dn0, locals.var_phi_b_dep_dn2, locals.var_phi_b_dep_dn6, locals.var_phi_b_dep_dn7, locals.var_phi_b_dep_dn10, locals.var_phi_b_dep_dn11, locals.var_phi_b_dep_dn12, locals.var_phi_b_dep_dn17,)
    }
};
        locals.var_phi_b_dep = assign12340_e14041;
        locals.var_phi_b_dep_dn0 = assign12340_e14041_d_n0;
        locals.var_phi_b_dep_dn2 = assign12340_e14041_d_n2;
        locals.var_phi_b_dep_dn6 = assign12340_e14041_d_n6;
        locals.var_phi_b_dep_dn7 = assign12340_e14041_d_n7;
        locals.var_phi_b_dep_dn10 = assign12340_e14041_d_n10;
        locals.var_phi_b_dep_dn11 = assign12340_e14041_d_n11;
        locals.var_phi_b_dep_dn12 = assign12340_e14041_d_n12;
        locals.var_phi_b_dep_dn17 = assign12340_e14041_d_n17;

        let (assign12350_e14060, assign12350_e14060_d_n10,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) {
        let assign12350_e14054: f64 = (locals.var_cnst0bulk * locals.var_cnst0bulk);
        let assign12350_e14056: f64 = (assign12350_e14054 * locals.var_c_box_fd_inv);
        let assign12350_e14058: f64 = (assign12350_e14056 * locals.var_c_box_fd_inv);
        (assign12350_e14058, ((((locals.var_cnst0bulk_dn10 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn10)) * locals.var_c_box_fd_inv) * locals.var_c_box_fd_inv),)
    } else {
        (locals.var_t0__blk324, locals.var_t0__blk324_dn10,)
    }
};
        locals.var_t0__blk324 = assign12350_e14060;
        locals.var_t0__blk324_dn10 = assign12350_e14060_d_n10;

        let assign12360_e14063: f64 = if locals.var_phi_sl_soi < locals.var_fd_end { 1.0 } else { 0.0 };
        locals.var_guard330 = assign12360_e14063;

        let (assign12370_e14079, assign12370_e14079_d_n0, assign12370_e14079_d_n2, assign12370_e14079_d_n6, assign12370_e14079_d_n7, assign12370_e14079_d_n10, assign12370_e14079_d_n11, assign12370_e14079_d_n12, assign12370_e14079_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard330 != 0.0)) {
        let assign12370_e14077: f64 = (-locals.var_vbsbiz);
        (assign12370_e14077, (-locals.var_vbsbiz_dn0), (-locals.var_vbsbiz_dn2), (-locals.var_vbsbiz_dn6), (-locals.var_vbsbiz_dn7), (-locals.var_vbsbiz_dn10), (-locals.var_vbsbiz_dn11), (-locals.var_vbsbiz_dn12), (-locals.var_vbsbiz_dn17),)
    } else {
        (locals.var_t1__blk325, locals.var_t1__blk325_dn0, locals.var_t1__blk325_dn2, locals.var_t1__blk325_dn6, locals.var_t1__blk325_dn7, locals.var_t1__blk325_dn10, locals.var_t1__blk325_dn11, locals.var_t1__blk325_dn12, locals.var_t1__blk325_dn17,)
    }
};
        locals.var_t1__blk325 = assign12370_e14079;
        locals.var_t1__blk325_dn0 = assign12370_e14079_d_n0;
        locals.var_t1__blk325_dn2 = assign12370_e14079_d_n2;
        locals.var_t1__blk325_dn6 = assign12370_e14079_d_n6;
        locals.var_t1__blk325_dn7 = assign12370_e14079_d_n7;
        locals.var_t1__blk325_dn10 = assign12370_e14079_d_n10;
        locals.var_t1__blk325_dn11 = assign12370_e14079_d_n11;
        locals.var_t1__blk325_dn12 = assign12370_e14079_d_n12;
        locals.var_t1__blk325_dn17 = assign12370_e14079_d_n17;

        let (assign12380_e14116, assign12380_e14116_d_n0, assign12380_e14116_d_n2, assign12380_e14116_d_n6, assign12380_e14116_d_n7, assign12380_e14116_d_n10, assign12380_e14116_d_n11, assign12380_e14116_d_n12, assign12380_e14116_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard330 != 0.0)) {
        let assign12380_e14094: f64 = (2.0 * locals.var_t1__blk325);
        let assign12380_e14097: f64 = (locals.var_t0__blk324 * locals.var_beta);
        let assign12380_e14098: f64 = (assign12380_e14094 + assign12380_e14097);
        let assign12380_e14101: f64 = (2.0 * locals.var_t1__blk325);
        let assign12380_e14104: f64 = (locals.var_t0__blk324 * locals.var_beta);
        let assign12380_e14105: f64 = (assign12380_e14101 + assign12380_e14104);
        let assign12380_e14106: f64 = (assign12380_e14098 * assign12380_e14105);
        let assign12380_e14110: f64 = (locals.var_t1__blk325 * locals.var_t1__blk325);
        let assign12380_e14112: f64 = (assign12380_e14110 + locals.var_t0__blk324);
        let assign12380_e14113: f64 = (4.0 * assign12380_e14112);
        let assign12380_e14114: f64 = (assign12380_e14106 - assign12380_e14113);
        (assign12380_e14114, ((((2.0 * locals.var_t1__blk325_dn0) * assign12380_e14105) + (assign12380_e14098 * (2.0 * locals.var_t1__blk325_dn0))) - (4.0 * ((locals.var_t1__blk325_dn0 * locals.var_t1__blk325) + (locals.var_t1__blk325 * locals.var_t1__blk325_dn0)))), ((((2.0 * locals.var_t1__blk325_dn2) * assign12380_e14105) + (assign12380_e14098 * (2.0 * locals.var_t1__blk325_dn2))) - (4.0 * ((locals.var_t1__blk325_dn2 * locals.var_t1__blk325) + (locals.var_t1__blk325 * locals.var_t1__blk325_dn2)))), ((((2.0 * locals.var_t1__blk325_dn6) * assign12380_e14105) + (assign12380_e14098 * (2.0 * locals.var_t1__blk325_dn6))) - (4.0 * ((locals.var_t1__blk325_dn6 * locals.var_t1__blk325) + (locals.var_t1__blk325 * locals.var_t1__blk325_dn6)))), ((((2.0 * locals.var_t1__blk325_dn7) * assign12380_e14105) + (assign12380_e14098 * (2.0 * locals.var_t1__blk325_dn7))) - (4.0 * ((locals.var_t1__blk325_dn7 * locals.var_t1__blk325) + (locals.var_t1__blk325 * locals.var_t1__blk325_dn7)))), (((((2.0 * locals.var_t1__blk325_dn10) + ((locals.var_t0__blk324_dn10 * locals.var_beta) + (locals.var_t0__blk324 * locals.var_beta_dn10))) * assign12380_e14105) + (assign12380_e14098 * ((2.0 * locals.var_t1__blk325_dn10) + ((locals.var_t0__blk324_dn10 * locals.var_beta) + (locals.var_t0__blk324 * locals.var_beta_dn10))))) - (4.0 * (((locals.var_t1__blk325_dn10 * locals.var_t1__blk325) + (locals.var_t1__blk325 * locals.var_t1__blk325_dn10)) + locals.var_t0__blk324_dn10))), ((((2.0 * locals.var_t1__blk325_dn11) * assign12380_e14105) + (assign12380_e14098 * (2.0 * locals.var_t1__blk325_dn11))) - (4.0 * ((locals.var_t1__blk325_dn11 * locals.var_t1__blk325) + (locals.var_t1__blk325 * locals.var_t1__blk325_dn11)))), ((((2.0 * locals.var_t1__blk325_dn12) * assign12380_e14105) + (assign12380_e14098 * (2.0 * locals.var_t1__blk325_dn12))) - (4.0 * ((locals.var_t1__blk325_dn12 * locals.var_t1__blk325) + (locals.var_t1__blk325 * locals.var_t1__blk325_dn12)))), ((((2.0 * locals.var_t1__blk325_dn17) * assign12380_e14105) + (assign12380_e14098 * (2.0 * locals.var_t1__blk325_dn17))) - (4.0 * ((locals.var_t1__blk325_dn17 * locals.var_t1__blk325) + (locals.var_t1__blk325 * locals.var_t1__blk325_dn17)))),)
    } else {
        (locals.var_t2__blk326, locals.var_t2__blk326_dn0, locals.var_t2__blk326_dn2, locals.var_t2__blk326_dn6, locals.var_t2__blk326_dn7, locals.var_t2__blk326_dn10, locals.var_t2__blk326_dn11, locals.var_t2__blk326_dn12, locals.var_t2__blk326_dn17,)
    }
};
        locals.var_t2__blk326 = assign12380_e14116;
        locals.var_t2__blk326_dn0 = assign12380_e14116_d_n0;
        locals.var_t2__blk326_dn2 = assign12380_e14116_d_n2;
        locals.var_t2__blk326_dn6 = assign12380_e14116_d_n6;
        locals.var_t2__blk326_dn7 = assign12380_e14116_d_n7;
        locals.var_t2__blk326_dn10 = assign12380_e14116_d_n10;
        locals.var_t2__blk326_dn11 = assign12380_e14116_d_n11;
        locals.var_t2__blk326_dn12 = assign12380_e14116_d_n12;
        locals.var_t2__blk326_dn17 = assign12380_e14116_d_n17;

        let (assign12390_e14140, assign12390_e14140_d_n0, assign12390_e14140_d_n2, assign12390_e14140_d_n6, assign12390_e14140_d_n7, assign12390_e14140_d_n10, assign12390_e14140_d_n11, assign12390_e14140_d_n12, assign12390_e14140_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard330 != 0.0)) {
        let assign12390_e14132: f64 = (10.0 * 2.220446049250313e-16);
        let (assign12390_e14138, assign12390_e14138_d_n0, assign12390_e14138_d_n2, assign12390_e14138_d_n6, assign12390_e14138_d_n7, assign12390_e14138_d_n10, assign12390_e14138_d_n11, assign12390_e14138_d_n12, assign12390_e14138_d_n17,) = {
            if (locals.var_t2__blk326 >= assign12390_e14132) {
                (locals.var_t2__blk326, locals.var_t2__blk326_dn0, locals.var_t2__blk326_dn2, locals.var_t2__blk326_dn6, locals.var_t2__blk326_dn7, locals.var_t2__blk326_dn10, locals.var_t2__blk326_dn11, locals.var_t2__blk326_dn12, locals.var_t2__blk326_dn17,)
            } else {
                let assign12390_e14137: f64 = (10.0 * 2.220446049250313e-16);
                (assign12390_e14137, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign12390_e14138, assign12390_e14138_d_n0, assign12390_e14138_d_n2, assign12390_e14138_d_n6, assign12390_e14138_d_n7, assign12390_e14138_d_n10, assign12390_e14138_d_n11, assign12390_e14138_d_n12, assign12390_e14138_d_n17,)
    } else {
        (locals.var_t2__blk326, locals.var_t2__blk326_dn0, locals.var_t2__blk326_dn2, locals.var_t2__blk326_dn6, locals.var_t2__blk326_dn7, locals.var_t2__blk326_dn10, locals.var_t2__blk326_dn11, locals.var_t2__blk326_dn12, locals.var_t2__blk326_dn17,)
    }
};
        locals.var_t2__blk326 = assign12390_e14140;
        locals.var_t2__blk326_dn0 = assign12390_e14140_d_n0;
        locals.var_t2__blk326_dn2 = assign12390_e14140_d_n2;
        locals.var_t2__blk326_dn6 = assign12390_e14140_d_n6;
        locals.var_t2__blk326_dn7 = assign12390_e14140_d_n7;
        locals.var_t2__blk326_dn10 = assign12390_e14140_d_n10;
        locals.var_t2__blk326_dn11 = assign12390_e14140_d_n11;
        locals.var_t2__blk326_dn12 = assign12390_e14140_d_n12;
        locals.var_t2__blk326_dn17 = assign12390_e14140_d_n17;

        let (assign12400_e14156, assign12400_e14156_d_n0, assign12400_e14156_d_n2, assign12400_e14156_d_n6, assign12400_e14156_d_n7, assign12400_e14156_d_n10, assign12400_e14156_d_n11, assign12400_e14156_d_n12, assign12400_e14156_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard330 != 0.0)) {
        let assign12400_e14154: f64 = (locals.var_t2__blk326).sqrt();
        (assign12400_e14154, (locals.var_t2__blk326_dn0 / (2.0 * assign12400_e14154)), (locals.var_t2__blk326_dn2 / (2.0 * assign12400_e14154)), (locals.var_t2__blk326_dn6 / (2.0 * assign12400_e14154)), (locals.var_t2__blk326_dn7 / (2.0 * assign12400_e14154)), (locals.var_t2__blk326_dn10 / (2.0 * assign12400_e14154)), (locals.var_t2__blk326_dn11 / (2.0 * assign12400_e14154)), (locals.var_t2__blk326_dn12 / (2.0 * assign12400_e14154)), (locals.var_t2__blk326_dn17 / (2.0 * assign12400_e14154)),)
    } else {
        (locals.var_t2__blk326, locals.var_t2__blk326_dn0, locals.var_t2__blk326_dn2, locals.var_t2__blk326_dn6, locals.var_t2__blk326_dn7, locals.var_t2__blk326_dn10, locals.var_t2__blk326_dn11, locals.var_t2__blk326_dn12, locals.var_t2__blk326_dn17,)
    }
};
        locals.var_t2__blk326 = assign12400_e14156;
        locals.var_t2__blk326_dn0 = assign12400_e14156_d_n0;
        locals.var_t2__blk326_dn2 = assign12400_e14156_d_n2;
        locals.var_t2__blk326_dn6 = assign12400_e14156_d_n6;
        locals.var_t2__blk326_dn7 = assign12400_e14156_d_n7;
        locals.var_t2__blk326_dn10 = assign12400_e14156_d_n10;
        locals.var_t2__blk326_dn11 = assign12400_e14156_d_n11;
        locals.var_t2__blk326_dn12 = assign12400_e14156_d_n12;
        locals.var_t2__blk326_dn17 = assign12400_e14156_d_n17;

        let (assign12410_e14177, assign12410_e14177_d_n0, assign12410_e14177_d_n2, assign12410_e14177_d_n6, assign12410_e14177_d_n7, assign12410_e14177_d_n10, assign12410_e14177_d_n11, assign12410_e14177_d_n12, assign12410_e14177_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard330 != 0.0)) {
        let assign12410_e14171: f64 = (2.0 * locals.var_t1__blk325);
        let assign12410_e14174: f64 = (locals.var_t0__blk324 * locals.var_beta);
        let assign12410_e14175: f64 = (assign12410_e14171 + assign12410_e14174);
        (assign12410_e14175, (2.0 * locals.var_t1__blk325_dn0), (2.0 * locals.var_t1__blk325_dn2), (2.0 * locals.var_t1__blk325_dn6), (2.0 * locals.var_t1__blk325_dn7), ((2.0 * locals.var_t1__blk325_dn10) + ((locals.var_t0__blk324_dn10 * locals.var_beta) + (locals.var_t0__blk324 * locals.var_beta_dn10))), (2.0 * locals.var_t1__blk325_dn11), (2.0 * locals.var_t1__blk325_dn12), (2.0 * locals.var_t1__blk325_dn17),)
    } else {
        (locals.var_t3__blk327, locals.var_t3__blk327_dn0, locals.var_t3__blk327_dn2, locals.var_t3__blk327_dn6, locals.var_t3__blk327_dn7, locals.var_t3__blk327_dn10, locals.var_t3__blk327_dn11, locals.var_t3__blk327_dn12, locals.var_t3__blk327_dn17,)
    }
};
        locals.var_t3__blk327 = assign12410_e14177;
        locals.var_t3__blk327_dn0 = assign12410_e14177_d_n0;
        locals.var_t3__blk327_dn2 = assign12410_e14177_d_n2;
        locals.var_t3__blk327_dn6 = assign12410_e14177_d_n6;
        locals.var_t3__blk327_dn7 = assign12410_e14177_d_n7;
        locals.var_t3__blk327_dn10 = assign12410_e14177_d_n10;
        locals.var_t3__blk327_dn11 = assign12410_e14177_d_n11;
        locals.var_t3__blk327_dn12 = assign12410_e14177_d_n12;
        locals.var_t3__blk327_dn17 = assign12410_e14177_d_n17;

        let (assign12420_e14196, assign12420_e14196_d_n0, assign12420_e14196_d_n2, assign12420_e14196_d_n6, assign12420_e14196_d_n7, assign12420_e14196_d_n10, assign12420_e14196_d_n11, assign12420_e14196_d_n12, assign12420_e14196_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard330 != 0.0)) {
        let assign12420_e14192: f64 = (locals.var_t3__blk327 - locals.var_t2__blk326);
        let assign12420_e14194: f64 = (assign12420_e14192 / 2.0);
        (assign12420_e14194, ((locals.var_t3__blk327_dn0 - locals.var_t2__blk326_dn0) / 2.0), ((locals.var_t3__blk327_dn2 - locals.var_t2__blk326_dn2) / 2.0), ((locals.var_t3__blk327_dn6 - locals.var_t2__blk326_dn6) / 2.0), ((locals.var_t3__blk327_dn7 - locals.var_t2__blk326_dn7) / 2.0), ((locals.var_t3__blk327_dn10 - locals.var_t2__blk326_dn10) / 2.0), ((locals.var_t3__blk327_dn11 - locals.var_t2__blk326_dn11) / 2.0), ((locals.var_t3__blk327_dn12 - locals.var_t2__blk326_dn12) / 2.0), ((locals.var_t3__blk327_dn17 - locals.var_t2__blk326_dn17) / 2.0),)
    } else {
        (locals.var_psb_inia__blk328, locals.var_psb_inia__blk328_dn0, locals.var_psb_inia__blk328_dn2, locals.var_psb_inia__blk328_dn6, locals.var_psb_inia__blk328_dn7, locals.var_psb_inia__blk328_dn10, locals.var_psb_inia__blk328_dn11, locals.var_psb_inia__blk328_dn12, locals.var_psb_inia__blk328_dn17,)
    }
};
        locals.var_psb_inia__blk328 = assign12420_e14196;
        locals.var_psb_inia__blk328_dn0 = assign12420_e14196_d_n0;
        locals.var_psb_inia__blk328_dn2 = assign12420_e14196_d_n2;
        locals.var_psb_inia__blk328_dn6 = assign12420_e14196_d_n6;
        locals.var_psb_inia__blk328_dn7 = assign12420_e14196_d_n7;
        locals.var_psb_inia__blk328_dn10 = assign12420_e14196_d_n10;
        locals.var_psb_inia__blk328_dn11 = assign12420_e14196_d_n11;
        locals.var_psb_inia__blk328_dn12 = assign12420_e14196_d_n12;
        locals.var_psb_inia__blk328_dn17 = assign12420_e14196_d_n17;

        let (assign12430_e14224, assign12430_e14224_d_n0, assign12430_e14224_d_n2, assign12430_e14224_d_n6, assign12430_e14224_d_n7, assign12430_e14224_d_n10, assign12430_e14224_d_n11, assign12430_e14224_d_n12, assign12430_e14224_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard330 != 0.0)) {
        let assign12430_e14211: f64 = (locals.var_t1__blk325 * locals.var_t1__blk325);
        let assign12430_e14213: f64 = (assign12430_e14211 / locals.var_t0__blk324);
        let assign12430_e14215: f64 = (assign12430_e14213 / locals.var_cnst1bulk);
        let assign12430_e14216: f64 = (assign12430_e14215).ln();
        let assign12430_e14220: f64 = (2.0 / locals.var_t1__blk325);
        let assign12430_e14221: f64 = (locals.var_beta + assign12430_e14220);
        let assign12430_e14222: f64 = (assign12430_e14216 / assign12430_e14221);
        (assign12430_e14222, ((((((((((locals.var_t1__blk325_dn0 * locals.var_t1__blk325) + (locals.var_t1__blk325 * locals.var_t1__blk325_dn0)) / locals.var_t0__blk324) * locals.var_cnst1bulk) - (assign12430_e14213 * locals.var_cnst1bulk_dn0)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12430_e14215) * assign12430_e14221) - (assign12430_e14216 * (-((2.0 * locals.var_t1__blk325_dn0) / (locals.var_t1__blk325 * locals.var_t1__blk325))))) / (assign12430_e14221 * assign12430_e14221)), ((((((((((locals.var_t1__blk325_dn2 * locals.var_t1__blk325) + (locals.var_t1__blk325 * locals.var_t1__blk325_dn2)) / locals.var_t0__blk324) * locals.var_cnst1bulk) - (assign12430_e14213 * locals.var_cnst1bulk_dn2)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12430_e14215) * assign12430_e14221) - (assign12430_e14216 * (-((2.0 * locals.var_t1__blk325_dn2) / (locals.var_t1__blk325 * locals.var_t1__blk325))))) / (assign12430_e14221 * assign12430_e14221)), ((((((((((locals.var_t1__blk325_dn6 * locals.var_t1__blk325) + (locals.var_t1__blk325 * locals.var_t1__blk325_dn6)) / locals.var_t0__blk324) * locals.var_cnst1bulk) - (assign12430_e14213 * locals.var_cnst1bulk_dn6)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12430_e14215) * assign12430_e14221) - (assign12430_e14216 * (-((2.0 * locals.var_t1__blk325_dn6) / (locals.var_t1__blk325 * locals.var_t1__blk325))))) / (assign12430_e14221 * assign12430_e14221)), ((((((((((locals.var_t1__blk325_dn7 * locals.var_t1__blk325) + (locals.var_t1__blk325 * locals.var_t1__blk325_dn7)) / locals.var_t0__blk324) * locals.var_cnst1bulk) - (assign12430_e14213 * locals.var_cnst1bulk_dn7)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12430_e14215) * assign12430_e14221) - (assign12430_e14216 * (-((2.0 * locals.var_t1__blk325_dn7) / (locals.var_t1__blk325 * locals.var_t1__blk325))))) / (assign12430_e14221 * assign12430_e14221)), ((((((((((((locals.var_t1__blk325_dn10 * locals.var_t1__blk325) + (locals.var_t1__blk325 * locals.var_t1__blk325_dn10)) * locals.var_t0__blk324) - (assign12430_e14211 * locals.var_t0__blk324_dn10)) / (locals.var_t0__blk324 * locals.var_t0__blk324)) * locals.var_cnst1bulk) - (assign12430_e14213 * locals.var_cnst1bulk_dn10)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12430_e14215) * assign12430_e14221) - (assign12430_e14216 * (locals.var_beta_dn10 + (-((2.0 * locals.var_t1__blk325_dn10) / (locals.var_t1__blk325 * locals.var_t1__blk325)))))) / (assign12430_e14221 * assign12430_e14221)), ((((((((((locals.var_t1__blk325_dn11 * locals.var_t1__blk325) + (locals.var_t1__blk325 * locals.var_t1__blk325_dn11)) / locals.var_t0__blk324) * locals.var_cnst1bulk) - (assign12430_e14213 * locals.var_cnst1bulk_dn11)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12430_e14215) * assign12430_e14221) - (assign12430_e14216 * (-((2.0 * locals.var_t1__blk325_dn11) / (locals.var_t1__blk325 * locals.var_t1__blk325))))) / (assign12430_e14221 * assign12430_e14221)), ((((((((((locals.var_t1__blk325_dn12 * locals.var_t1__blk325) + (locals.var_t1__blk325 * locals.var_t1__blk325_dn12)) / locals.var_t0__blk324) * locals.var_cnst1bulk) - (assign12430_e14213 * locals.var_cnst1bulk_dn12)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12430_e14215) * assign12430_e14221) - (assign12430_e14216 * (-((2.0 * locals.var_t1__blk325_dn12) / (locals.var_t1__blk325 * locals.var_t1__blk325))))) / (assign12430_e14221 * assign12430_e14221)), ((((((((((locals.var_t1__blk325_dn17 * locals.var_t1__blk325) + (locals.var_t1__blk325 * locals.var_t1__blk325_dn17)) / locals.var_t0__blk324) * locals.var_cnst1bulk) - (assign12430_e14213 * locals.var_cnst1bulk_dn17)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12430_e14215) * assign12430_e14221) - (assign12430_e14216 * (-((2.0 * locals.var_t1__blk325_dn17) / (locals.var_t1__blk325 * locals.var_t1__blk325))))) / (assign12430_e14221 * assign12430_e14221)),)
    } else {
        (locals.var_psb_inib__blk329, locals.var_psb_inib__blk329_dn0, locals.var_psb_inib__blk329_dn2, locals.var_psb_inib__blk329_dn6, locals.var_psb_inib__blk329_dn7, locals.var_psb_inib__blk329_dn10, locals.var_psb_inib__blk329_dn11, locals.var_psb_inib__blk329_dn12, locals.var_psb_inib__blk329_dn17,)
    }
};
        locals.var_psb_inib__blk329 = assign12430_e14224;
        locals.var_psb_inib__blk329_dn0 = assign12430_e14224_d_n0;
        locals.var_psb_inib__blk329_dn2 = assign12430_e14224_d_n2;
        locals.var_psb_inib__blk329_dn6 = assign12430_e14224_d_n6;
        locals.var_psb_inib__blk329_dn7 = assign12430_e14224_d_n7;
        locals.var_psb_inib__blk329_dn10 = assign12430_e14224_d_n10;
        locals.var_psb_inib__blk329_dn11 = assign12430_e14224_d_n11;
        locals.var_psb_inib__blk329_dn12 = assign12430_e14224_d_n12;
        locals.var_psb_inib__blk329_dn17 = assign12430_e14224_d_n17;

        let assign12440_e14227: f64 = if locals.var_psb_inia__blk328 < locals.var_pb2_bulk { 1.0 } else { 0.0 };
        locals.var_guard331 = assign12440_e14227;

        let (assign12450_e14244, assign12450_e14244_d_n0, assign12450_e14244_d_n2, assign12450_e14244_d_n6, assign12450_e14244_d_n7, assign12450_e14244_d_n10, assign12450_e14244_d_n11, assign12450_e14244_d_n12, assign12450_e14244_d_n17,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard330 != 0.0)) && (locals.var_guard331 != 0.0)) {
        (locals.var_psb_inia__blk328, locals.var_psb_inia__blk328_dn0, locals.var_psb_inia__blk328_dn2, locals.var_psb_inia__blk328_dn6, locals.var_psb_inia__blk328_dn7, locals.var_psb_inia__blk328_dn10, locals.var_psb_inia__blk328_dn11, locals.var_psb_inia__blk328_dn12, locals.var_psb_inia__blk328_dn17,)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
        locals.var_phi_sl_bulk = assign12450_e14244;
        locals.var_phi_sl_bulk_dn0 = assign12450_e14244_d_n0;
        locals.var_phi_sl_bulk_dn2 = assign12450_e14244_d_n2;
        locals.var_phi_sl_bulk_dn6 = assign12450_e14244_d_n6;
        locals.var_phi_sl_bulk_dn7 = assign12450_e14244_d_n7;
        locals.var_phi_sl_bulk_dn10 = assign12450_e14244_d_n10;
        locals.var_phi_sl_bulk_dn11 = assign12450_e14244_d_n11;
        locals.var_phi_sl_bulk_dn12 = assign12450_e14244_d_n12;
        locals.var_phi_sl_bulk_dn17 = assign12450_e14244_d_n17;

        let (assign12460_e14266, assign12460_e14266_d_n0, assign12460_e14266_d_n2, assign12460_e14266_d_n6, assign12460_e14266_d_n7, assign12460_e14266_d_n10, assign12460_e14266_d_n11, assign12460_e14266_d_n12, assign12460_e14266_d_n17,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard330 != 0.0)) && (locals.var_guard331 == 0.0)) {
        let assign12460_e14262: f64 = (locals.var_psb_inib__blk329 - locals.var_psb_inia__blk328);
        let assign12460_e14264: f64 = (assign12460_e14262 - 0.0008);
        (assign12460_e14264, (locals.var_psb_inib__blk329_dn0 - locals.var_psb_inia__blk328_dn0), (locals.var_psb_inib__blk329_dn2 - locals.var_psb_inia__blk328_dn2), (locals.var_psb_inib__blk329_dn6 - locals.var_psb_inia__blk328_dn6), (locals.var_psb_inib__blk329_dn7 - locals.var_psb_inia__blk328_dn7), (locals.var_psb_inib__blk329_dn10 - locals.var_psb_inia__blk328_dn10), (locals.var_psb_inib__blk329_dn11 - locals.var_psb_inia__blk328_dn11), (locals.var_psb_inib__blk329_dn12 - locals.var_psb_inia__blk328_dn12), (locals.var_psb_inib__blk329_dn17 - locals.var_psb_inia__blk328_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign12460_e14266;
        locals.var_tmf1_dn0 = assign12460_e14266_d_n0;
        locals.var_tmf1_dn2 = assign12460_e14266_d_n2;
        locals.var_tmf1_dn6 = assign12460_e14266_d_n6;
        locals.var_tmf1_dn7 = assign12460_e14266_d_n7;
        locals.var_tmf1_dn10 = assign12460_e14266_d_n10;
        locals.var_tmf1_dn11 = assign12460_e14266_d_n11;
        locals.var_tmf1_dn12 = assign12460_e14266_d_n12;
        locals.var_tmf1_dn17 = assign12460_e14266_d_n17;

        let (assign12470_e14288, assign12470_e14288_d_n0, assign12470_e14288_d_n2, assign12470_e14288_d_n6, assign12470_e14288_d_n7, assign12470_e14288_d_n10, assign12470_e14288_d_n11, assign12470_e14288_d_n12, assign12470_e14288_d_n17,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard330 != 0.0)) && (locals.var_guard331 == 0.0)) {
        let assign12470_e14284: f64 = (4.0 * locals.var_psb_inib__blk329);
        let assign12470_e14286: f64 = (assign12470_e14284 * 0.0008);
        (assign12470_e14286, ((4.0 * locals.var_psb_inib__blk329_dn0) * 0.0008), ((4.0 * locals.var_psb_inib__blk329_dn2) * 0.0008), ((4.0 * locals.var_psb_inib__blk329_dn6) * 0.0008), ((4.0 * locals.var_psb_inib__blk329_dn7) * 0.0008), ((4.0 * locals.var_psb_inib__blk329_dn10) * 0.0008), ((4.0 * locals.var_psb_inib__blk329_dn11) * 0.0008), ((4.0 * locals.var_psb_inib__blk329_dn12) * 0.0008), ((4.0 * locals.var_psb_inib__blk329_dn17) * 0.0008),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign12470_e14288;
        locals.var_tmf2_dn0 = assign12470_e14288_d_n0;
        locals.var_tmf2_dn2 = assign12470_e14288_d_n2;
        locals.var_tmf2_dn6 = assign12470_e14288_d_n6;
        locals.var_tmf2_dn7 = assign12470_e14288_d_n7;
        locals.var_tmf2_dn10 = assign12470_e14288_d_n10;
        locals.var_tmf2_dn11 = assign12470_e14288_d_n11;
        locals.var_tmf2_dn12 = assign12470_e14288_d_n12;
        locals.var_tmf2_dn17 = assign12470_e14288_d_n17;

    }

    pub(super) fn stamp_transient_block_36(
        locals: &mut StampLocals,
    ) {
        let (assign12480_e14312, assign12480_e14312_d_n0, assign12480_e14312_d_n2, assign12480_e14312_d_n6, assign12480_e14312_d_n7, assign12480_e14312_d_n10, assign12480_e14312_d_n11, assign12480_e14312_d_n12, assign12480_e14312_d_n17,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard330 != 0.0)) && (locals.var_guard331 == 0.0)) {
        let (assign12480_e14310, assign12480_e14310_d_n0, assign12480_e14310_d_n2, assign12480_e14310_d_n6, assign12480_e14310_d_n7, assign12480_e14310_d_n10, assign12480_e14310_d_n11, assign12480_e14310_d_n12, assign12480_e14310_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign12480_e14309: f64 = (-locals.var_tmf2);
                (assign12480_e14309, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign12480_e14310, assign12480_e14310_d_n0, assign12480_e14310_d_n2, assign12480_e14310_d_n6, assign12480_e14310_d_n7, assign12480_e14310_d_n10, assign12480_e14310_d_n11, assign12480_e14310_d_n12, assign12480_e14310_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign12480_e14312;
        locals.var_tmf2_dn0 = assign12480_e14312_d_n0;
        locals.var_tmf2_dn2 = assign12480_e14312_d_n2;
        locals.var_tmf2_dn6 = assign12480_e14312_d_n6;
        locals.var_tmf2_dn7 = assign12480_e14312_d_n7;
        locals.var_tmf2_dn10 = assign12480_e14312_d_n10;
        locals.var_tmf2_dn11 = assign12480_e14312_d_n11;
        locals.var_tmf2_dn12 = assign12480_e14312_d_n12;
        locals.var_tmf2_dn17 = assign12480_e14312_d_n17;

        let (assign12490_e14335, assign12490_e14335_d_n0, assign12490_e14335_d_n2, assign12490_e14335_d_n6, assign12490_e14335_d_n7, assign12490_e14335_d_n10, assign12490_e14335_d_n11, assign12490_e14335_d_n12, assign12490_e14335_d_n17,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard330 != 0.0)) && (locals.var_guard331 == 0.0)) {
        let assign12490_e14330: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign12490_e14332: f64 = (assign12490_e14330 + locals.var_tmf2);
        let assign12490_e14333: f64 = (assign12490_e14332).sqrt();
        (assign12490_e14333, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign12490_e14333)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign12490_e14333)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign12490_e14333)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign12490_e14333)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign12490_e14333)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign12490_e14333)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign12490_e14333)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign12490_e14333)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign12490_e14335;
        locals.var_tmf2_dn0 = assign12490_e14335_d_n0;
        locals.var_tmf2_dn2 = assign12490_e14335_d_n2;
        locals.var_tmf2_dn6 = assign12490_e14335_d_n6;
        locals.var_tmf2_dn7 = assign12490_e14335_d_n7;
        locals.var_tmf2_dn10 = assign12490_e14335_d_n10;
        locals.var_tmf2_dn11 = assign12490_e14335_d_n11;
        locals.var_tmf2_dn12 = assign12490_e14335_d_n12;
        locals.var_tmf2_dn17 = assign12490_e14335_d_n17;

        let (assign12500_e14359, assign12500_e14359_d_n0, assign12500_e14359_d_n2, assign12500_e14359_d_n6, assign12500_e14359_d_n7, assign12500_e14359_d_n10, assign12500_e14359_d_n11, assign12500_e14359_d_n12, assign12500_e14359_d_n17,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard330 != 0.0)) && (locals.var_guard331 == 0.0)) {
        let assign12500_e14355: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign12500_e14356: f64 = (0.5 * assign12500_e14355);
        let assign12500_e14357: f64 = (locals.var_psb_inib__blk329 - assign12500_e14356);
        (assign12500_e14357, (locals.var_psb_inib__blk329_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psb_inib__blk329_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psb_inib__blk329_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psb_inib__blk329_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psb_inib__blk329_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psb_inib__blk329_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psb_inib__blk329_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_psb_inib__blk329_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
        locals.var_phi_sl_bulk = assign12500_e14359;
        locals.var_phi_sl_bulk_dn0 = assign12500_e14359_d_n0;
        locals.var_phi_sl_bulk_dn2 = assign12500_e14359_d_n2;
        locals.var_phi_sl_bulk_dn6 = assign12500_e14359_d_n6;
        locals.var_phi_sl_bulk_dn7 = assign12500_e14359_d_n7;
        locals.var_phi_sl_bulk_dn10 = assign12500_e14359_d_n10;
        locals.var_phi_sl_bulk_dn11 = assign12500_e14359_d_n11;
        locals.var_phi_sl_bulk_dn12 = assign12500_e14359_d_n12;
        locals.var_phi_sl_bulk_dn17 = assign12500_e14359_d_n17;

        let (assign12510_e14386, assign12510_e14386_d_n0, assign12510_e14386_d_n2, assign12510_e14386_d_n6, assign12510_e14386_d_n7, assign12510_e14386_d_n10, assign12510_e14386_d_n11, assign12510_e14386_d_n12, assign12510_e14386_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard330 == 0.0)) {
        let assign12510_e14375: f64 = (locals.var_vbsbiz - locals.var_phi_sl_soi);
        let assign12510_e14378: f64 = (locals.var_q_fd_soi / 2.0);
        let assign12510_e14380: f64 = (assign12510_e14378 * locals.var_t_soi);
        let assign12510_e14382: f64 = (assign12510_e14380 / 1.034943e-10);
        let assign12510_e14383: f64 = (assign12510_e14375 - assign12510_e14382);
        let assign12510_e14384: f64 = (-assign12510_e14383);
        (assign12510_e14384, (-((locals.var_vbsbiz_dn0 - locals.var_phi_sl_soi_dn0) - (((locals.var_q_fd_soi_dn0 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn2 - locals.var_phi_sl_soi_dn2) - (((locals.var_q_fd_soi_dn2 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn6 - locals.var_phi_sl_soi_dn6) - (((locals.var_q_fd_soi_dn6 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn7 - locals.var_phi_sl_soi_dn7) - (((locals.var_q_fd_soi_dn7 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn10 - locals.var_phi_sl_soi_dn10) - (((locals.var_q_fd_soi_dn10 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn11 - locals.var_phi_sl_soi_dn11) - (((locals.var_q_fd_soi_dn11 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn12 - locals.var_phi_sl_soi_dn12) - (((locals.var_q_fd_soi_dn12 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn17 - locals.var_phi_sl_soi_dn17) - (((locals.var_q_fd_soi_dn17 / 2.0) * locals.var_t_soi) / 1.034943e-10))),)
    } else {
        (locals.var_t1__blk325, locals.var_t1__blk325_dn0, locals.var_t1__blk325_dn2, locals.var_t1__blk325_dn6, locals.var_t1__blk325_dn7, locals.var_t1__blk325_dn10, locals.var_t1__blk325_dn11, locals.var_t1__blk325_dn12, locals.var_t1__blk325_dn17,)
    }
};
        locals.var_t1__blk325 = assign12510_e14386;
        locals.var_t1__blk325_dn0 = assign12510_e14386_d_n0;
        locals.var_t1__blk325_dn2 = assign12510_e14386_d_n2;
        locals.var_t1__blk325_dn6 = assign12510_e14386_d_n6;
        locals.var_t1__blk325_dn7 = assign12510_e14386_d_n7;
        locals.var_t1__blk325_dn10 = assign12510_e14386_d_n10;
        locals.var_t1__blk325_dn11 = assign12510_e14386_d_n11;
        locals.var_t1__blk325_dn12 = assign12510_e14386_d_n12;
        locals.var_t1__blk325_dn17 = assign12510_e14386_d_n17;

        let (assign12520_e14424, assign12520_e14424_d_n0, assign12520_e14424_d_n2, assign12520_e14424_d_n6, assign12520_e14424_d_n7, assign12520_e14424_d_n10, assign12520_e14424_d_n11, assign12520_e14424_d_n12, assign12520_e14424_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard330 == 0.0)) {
        let assign12520_e14402: f64 = (2.0 * locals.var_t1__blk325);
        let assign12520_e14405: f64 = (locals.var_t0__blk324 * locals.var_beta);
        let assign12520_e14406: f64 = (assign12520_e14402 + assign12520_e14405);
        let assign12520_e14409: f64 = (2.0 * locals.var_t1__blk325);
        let assign12520_e14412: f64 = (locals.var_t0__blk324 * locals.var_beta);
        let assign12520_e14413: f64 = (assign12520_e14409 + assign12520_e14412);
        let assign12520_e14414: f64 = (assign12520_e14406 * assign12520_e14413);
        let assign12520_e14418: f64 = (locals.var_t1__blk325 * locals.var_t1__blk325);
        let assign12520_e14420: f64 = (assign12520_e14418 + locals.var_t0__blk324);
        let assign12520_e14421: f64 = (4.0 * assign12520_e14420);
        let assign12520_e14422: f64 = (assign12520_e14414 - assign12520_e14421);
        (assign12520_e14422, ((((2.0 * locals.var_t1__blk325_dn0) * assign12520_e14413) + (assign12520_e14406 * (2.0 * locals.var_t1__blk325_dn0))) - (4.0 * ((locals.var_t1__blk325_dn0 * locals.var_t1__blk325) + (locals.var_t1__blk325 * locals.var_t1__blk325_dn0)))), ((((2.0 * locals.var_t1__blk325_dn2) * assign12520_e14413) + (assign12520_e14406 * (2.0 * locals.var_t1__blk325_dn2))) - (4.0 * ((locals.var_t1__blk325_dn2 * locals.var_t1__blk325) + (locals.var_t1__blk325 * locals.var_t1__blk325_dn2)))), ((((2.0 * locals.var_t1__blk325_dn6) * assign12520_e14413) + (assign12520_e14406 * (2.0 * locals.var_t1__blk325_dn6))) - (4.0 * ((locals.var_t1__blk325_dn6 * locals.var_t1__blk325) + (locals.var_t1__blk325 * locals.var_t1__blk325_dn6)))), ((((2.0 * locals.var_t1__blk325_dn7) * assign12520_e14413) + (assign12520_e14406 * (2.0 * locals.var_t1__blk325_dn7))) - (4.0 * ((locals.var_t1__blk325_dn7 * locals.var_t1__blk325) + (locals.var_t1__blk325 * locals.var_t1__blk325_dn7)))), (((((2.0 * locals.var_t1__blk325_dn10) + ((locals.var_t0__blk324_dn10 * locals.var_beta) + (locals.var_t0__blk324 * locals.var_beta_dn10))) * assign12520_e14413) + (assign12520_e14406 * ((2.0 * locals.var_t1__blk325_dn10) + ((locals.var_t0__blk324_dn10 * locals.var_beta) + (locals.var_t0__blk324 * locals.var_beta_dn10))))) - (4.0 * (((locals.var_t1__blk325_dn10 * locals.var_t1__blk325) + (locals.var_t1__blk325 * locals.var_t1__blk325_dn10)) + locals.var_t0__blk324_dn10))), ((((2.0 * locals.var_t1__blk325_dn11) * assign12520_e14413) + (assign12520_e14406 * (2.0 * locals.var_t1__blk325_dn11))) - (4.0 * ((locals.var_t1__blk325_dn11 * locals.var_t1__blk325) + (locals.var_t1__blk325 * locals.var_t1__blk325_dn11)))), ((((2.0 * locals.var_t1__blk325_dn12) * assign12520_e14413) + (assign12520_e14406 * (2.0 * locals.var_t1__blk325_dn12))) - (4.0 * ((locals.var_t1__blk325_dn12 * locals.var_t1__blk325) + (locals.var_t1__blk325 * locals.var_t1__blk325_dn12)))), ((((2.0 * locals.var_t1__blk325_dn17) * assign12520_e14413) + (assign12520_e14406 * (2.0 * locals.var_t1__blk325_dn17))) - (4.0 * ((locals.var_t1__blk325_dn17 * locals.var_t1__blk325) + (locals.var_t1__blk325 * locals.var_t1__blk325_dn17)))),)
    } else {
        (locals.var_t2__blk326, locals.var_t2__blk326_dn0, locals.var_t2__blk326_dn2, locals.var_t2__blk326_dn6, locals.var_t2__blk326_dn7, locals.var_t2__blk326_dn10, locals.var_t2__blk326_dn11, locals.var_t2__blk326_dn12, locals.var_t2__blk326_dn17,)
    }
};
        locals.var_t2__blk326 = assign12520_e14424;
        locals.var_t2__blk326_dn0 = assign12520_e14424_d_n0;
        locals.var_t2__blk326_dn2 = assign12520_e14424_d_n2;
        locals.var_t2__blk326_dn6 = assign12520_e14424_d_n6;
        locals.var_t2__blk326_dn7 = assign12520_e14424_d_n7;
        locals.var_t2__blk326_dn10 = assign12520_e14424_d_n10;
        locals.var_t2__blk326_dn11 = assign12520_e14424_d_n11;
        locals.var_t2__blk326_dn12 = assign12520_e14424_d_n12;
        locals.var_t2__blk326_dn17 = assign12520_e14424_d_n17;

        let (assign12530_e14449, assign12530_e14449_d_n0, assign12530_e14449_d_n2, assign12530_e14449_d_n6, assign12530_e14449_d_n7, assign12530_e14449_d_n10, assign12530_e14449_d_n11, assign12530_e14449_d_n12, assign12530_e14449_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard330 == 0.0)) {
        let assign12530_e14441: f64 = (10.0 * 2.220446049250313e-16);
        let (assign12530_e14447, assign12530_e14447_d_n0, assign12530_e14447_d_n2, assign12530_e14447_d_n6, assign12530_e14447_d_n7, assign12530_e14447_d_n10, assign12530_e14447_d_n11, assign12530_e14447_d_n12, assign12530_e14447_d_n17,) = {
            if (locals.var_t2__blk326 >= assign12530_e14441) {
                (locals.var_t2__blk326, locals.var_t2__blk326_dn0, locals.var_t2__blk326_dn2, locals.var_t2__blk326_dn6, locals.var_t2__blk326_dn7, locals.var_t2__blk326_dn10, locals.var_t2__blk326_dn11, locals.var_t2__blk326_dn12, locals.var_t2__blk326_dn17,)
            } else {
                let assign12530_e14446: f64 = (10.0 * 2.220446049250313e-16);
                (assign12530_e14446, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign12530_e14447, assign12530_e14447_d_n0, assign12530_e14447_d_n2, assign12530_e14447_d_n6, assign12530_e14447_d_n7, assign12530_e14447_d_n10, assign12530_e14447_d_n11, assign12530_e14447_d_n12, assign12530_e14447_d_n17,)
    } else {
        (locals.var_t2__blk326, locals.var_t2__blk326_dn0, locals.var_t2__blk326_dn2, locals.var_t2__blk326_dn6, locals.var_t2__blk326_dn7, locals.var_t2__blk326_dn10, locals.var_t2__blk326_dn11, locals.var_t2__blk326_dn12, locals.var_t2__blk326_dn17,)
    }
};
        locals.var_t2__blk326 = assign12530_e14449;
        locals.var_t2__blk326_dn0 = assign12530_e14449_d_n0;
        locals.var_t2__blk326_dn2 = assign12530_e14449_d_n2;
        locals.var_t2__blk326_dn6 = assign12530_e14449_d_n6;
        locals.var_t2__blk326_dn7 = assign12530_e14449_d_n7;
        locals.var_t2__blk326_dn10 = assign12530_e14449_d_n10;
        locals.var_t2__blk326_dn11 = assign12530_e14449_d_n11;
        locals.var_t2__blk326_dn12 = assign12530_e14449_d_n12;
        locals.var_t2__blk326_dn17 = assign12530_e14449_d_n17;

        let (assign12540_e14466, assign12540_e14466_d_n0, assign12540_e14466_d_n2, assign12540_e14466_d_n6, assign12540_e14466_d_n7, assign12540_e14466_d_n10, assign12540_e14466_d_n11, assign12540_e14466_d_n12, assign12540_e14466_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard330 == 0.0)) {
        let assign12540_e14464: f64 = (locals.var_t2__blk326).sqrt();
        (assign12540_e14464, (locals.var_t2__blk326_dn0 / (2.0 * assign12540_e14464)), (locals.var_t2__blk326_dn2 / (2.0 * assign12540_e14464)), (locals.var_t2__blk326_dn6 / (2.0 * assign12540_e14464)), (locals.var_t2__blk326_dn7 / (2.0 * assign12540_e14464)), (locals.var_t2__blk326_dn10 / (2.0 * assign12540_e14464)), (locals.var_t2__blk326_dn11 / (2.0 * assign12540_e14464)), (locals.var_t2__blk326_dn12 / (2.0 * assign12540_e14464)), (locals.var_t2__blk326_dn17 / (2.0 * assign12540_e14464)),)
    } else {
        (locals.var_t2__blk326, locals.var_t2__blk326_dn0, locals.var_t2__blk326_dn2, locals.var_t2__blk326_dn6, locals.var_t2__blk326_dn7, locals.var_t2__blk326_dn10, locals.var_t2__blk326_dn11, locals.var_t2__blk326_dn12, locals.var_t2__blk326_dn17,)
    }
};
        locals.var_t2__blk326 = assign12540_e14466;
        locals.var_t2__blk326_dn0 = assign12540_e14466_d_n0;
        locals.var_t2__blk326_dn2 = assign12540_e14466_d_n2;
        locals.var_t2__blk326_dn6 = assign12540_e14466_d_n6;
        locals.var_t2__blk326_dn7 = assign12540_e14466_d_n7;
        locals.var_t2__blk326_dn10 = assign12540_e14466_d_n10;
        locals.var_t2__blk326_dn11 = assign12540_e14466_d_n11;
        locals.var_t2__blk326_dn12 = assign12540_e14466_d_n12;
        locals.var_t2__blk326_dn17 = assign12540_e14466_d_n17;

        let (assign12550_e14488, assign12550_e14488_d_n0, assign12550_e14488_d_n2, assign12550_e14488_d_n6, assign12550_e14488_d_n7, assign12550_e14488_d_n10, assign12550_e14488_d_n11, assign12550_e14488_d_n12, assign12550_e14488_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard330 == 0.0)) {
        let assign12550_e14482: f64 = (2.0 * locals.var_t1__blk325);
        let assign12550_e14485: f64 = (locals.var_t0__blk324 * locals.var_beta);
        let assign12550_e14486: f64 = (assign12550_e14482 + assign12550_e14485);
        (assign12550_e14486, (2.0 * locals.var_t1__blk325_dn0), (2.0 * locals.var_t1__blk325_dn2), (2.0 * locals.var_t1__blk325_dn6), (2.0 * locals.var_t1__blk325_dn7), ((2.0 * locals.var_t1__blk325_dn10) + ((locals.var_t0__blk324_dn10 * locals.var_beta) + (locals.var_t0__blk324 * locals.var_beta_dn10))), (2.0 * locals.var_t1__blk325_dn11), (2.0 * locals.var_t1__blk325_dn12), (2.0 * locals.var_t1__blk325_dn17),)
    } else {
        (locals.var_t3__blk327, locals.var_t3__blk327_dn0, locals.var_t3__blk327_dn2, locals.var_t3__blk327_dn6, locals.var_t3__blk327_dn7, locals.var_t3__blk327_dn10, locals.var_t3__blk327_dn11, locals.var_t3__blk327_dn12, locals.var_t3__blk327_dn17,)
    }
};
        locals.var_t3__blk327 = assign12550_e14488;
        locals.var_t3__blk327_dn0 = assign12550_e14488_d_n0;
        locals.var_t3__blk327_dn2 = assign12550_e14488_d_n2;
        locals.var_t3__blk327_dn6 = assign12550_e14488_d_n6;
        locals.var_t3__blk327_dn7 = assign12550_e14488_d_n7;
        locals.var_t3__blk327_dn10 = assign12550_e14488_d_n10;
        locals.var_t3__blk327_dn11 = assign12550_e14488_d_n11;
        locals.var_t3__blk327_dn12 = assign12550_e14488_d_n12;
        locals.var_t3__blk327_dn17 = assign12550_e14488_d_n17;

        let (assign12560_e14508, assign12560_e14508_d_n0, assign12560_e14508_d_n2, assign12560_e14508_d_n6, assign12560_e14508_d_n7, assign12560_e14508_d_n10, assign12560_e14508_d_n11, assign12560_e14508_d_n12, assign12560_e14508_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard330 == 0.0)) {
        let assign12560_e14504: f64 = (locals.var_t3__blk327 - locals.var_t2__blk326);
        let assign12560_e14506: f64 = (assign12560_e14504 / 2.0);
        (assign12560_e14506, ((locals.var_t3__blk327_dn0 - locals.var_t2__blk326_dn0) / 2.0), ((locals.var_t3__blk327_dn2 - locals.var_t2__blk326_dn2) / 2.0), ((locals.var_t3__blk327_dn6 - locals.var_t2__blk326_dn6) / 2.0), ((locals.var_t3__blk327_dn7 - locals.var_t2__blk326_dn7) / 2.0), ((locals.var_t3__blk327_dn10 - locals.var_t2__blk326_dn10) / 2.0), ((locals.var_t3__blk327_dn11 - locals.var_t2__blk326_dn11) / 2.0), ((locals.var_t3__blk327_dn12 - locals.var_t2__blk326_dn12) / 2.0), ((locals.var_t3__blk327_dn17 - locals.var_t2__blk326_dn17) / 2.0),)
    } else {
        (locals.var_psb_inia__blk328, locals.var_psb_inia__blk328_dn0, locals.var_psb_inia__blk328_dn2, locals.var_psb_inia__blk328_dn6, locals.var_psb_inia__blk328_dn7, locals.var_psb_inia__blk328_dn10, locals.var_psb_inia__blk328_dn11, locals.var_psb_inia__blk328_dn12, locals.var_psb_inia__blk328_dn17,)
    }
};
        locals.var_psb_inia__blk328 = assign12560_e14508;
        locals.var_psb_inia__blk328_dn0 = assign12560_e14508_d_n0;
        locals.var_psb_inia__blk328_dn2 = assign12560_e14508_d_n2;
        locals.var_psb_inia__blk328_dn6 = assign12560_e14508_d_n6;
        locals.var_psb_inia__blk328_dn7 = assign12560_e14508_d_n7;
        locals.var_psb_inia__blk328_dn10 = assign12560_e14508_d_n10;
        locals.var_psb_inia__blk328_dn11 = assign12560_e14508_d_n11;
        locals.var_psb_inia__blk328_dn12 = assign12560_e14508_d_n12;
        locals.var_psb_inia__blk328_dn17 = assign12560_e14508_d_n17;

        let (assign12570_e14537, assign12570_e14537_d_n0, assign12570_e14537_d_n2, assign12570_e14537_d_n6, assign12570_e14537_d_n7, assign12570_e14537_d_n10, assign12570_e14537_d_n11, assign12570_e14537_d_n12, assign12570_e14537_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard330 == 0.0)) {
        let assign12570_e14524: f64 = (locals.var_t1__blk325 * locals.var_t1__blk325);
        let assign12570_e14526: f64 = (assign12570_e14524 / locals.var_t0__blk324);
        let assign12570_e14528: f64 = (assign12570_e14526 / locals.var_cnst1bulk);
        let assign12570_e14529: f64 = (assign12570_e14528).ln();
        let assign12570_e14533: f64 = (2.0 / locals.var_t1__blk325);
        let assign12570_e14534: f64 = (locals.var_beta + assign12570_e14533);
        let assign12570_e14535: f64 = (assign12570_e14529 / assign12570_e14534);
        (assign12570_e14535, ((((((((((locals.var_t1__blk325_dn0 * locals.var_t1__blk325) + (locals.var_t1__blk325 * locals.var_t1__blk325_dn0)) / locals.var_t0__blk324) * locals.var_cnst1bulk) - (assign12570_e14526 * locals.var_cnst1bulk_dn0)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12570_e14528) * assign12570_e14534) - (assign12570_e14529 * (-((2.0 * locals.var_t1__blk325_dn0) / (locals.var_t1__blk325 * locals.var_t1__blk325))))) / (assign12570_e14534 * assign12570_e14534)), ((((((((((locals.var_t1__blk325_dn2 * locals.var_t1__blk325) + (locals.var_t1__blk325 * locals.var_t1__blk325_dn2)) / locals.var_t0__blk324) * locals.var_cnst1bulk) - (assign12570_e14526 * locals.var_cnst1bulk_dn2)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12570_e14528) * assign12570_e14534) - (assign12570_e14529 * (-((2.0 * locals.var_t1__blk325_dn2) / (locals.var_t1__blk325 * locals.var_t1__blk325))))) / (assign12570_e14534 * assign12570_e14534)), ((((((((((locals.var_t1__blk325_dn6 * locals.var_t1__blk325) + (locals.var_t1__blk325 * locals.var_t1__blk325_dn6)) / locals.var_t0__blk324) * locals.var_cnst1bulk) - (assign12570_e14526 * locals.var_cnst1bulk_dn6)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12570_e14528) * assign12570_e14534) - (assign12570_e14529 * (-((2.0 * locals.var_t1__blk325_dn6) / (locals.var_t1__blk325 * locals.var_t1__blk325))))) / (assign12570_e14534 * assign12570_e14534)), ((((((((((locals.var_t1__blk325_dn7 * locals.var_t1__blk325) + (locals.var_t1__blk325 * locals.var_t1__blk325_dn7)) / locals.var_t0__blk324) * locals.var_cnst1bulk) - (assign12570_e14526 * locals.var_cnst1bulk_dn7)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12570_e14528) * assign12570_e14534) - (assign12570_e14529 * (-((2.0 * locals.var_t1__blk325_dn7) / (locals.var_t1__blk325 * locals.var_t1__blk325))))) / (assign12570_e14534 * assign12570_e14534)), ((((((((((((locals.var_t1__blk325_dn10 * locals.var_t1__blk325) + (locals.var_t1__blk325 * locals.var_t1__blk325_dn10)) * locals.var_t0__blk324) - (assign12570_e14524 * locals.var_t0__blk324_dn10)) / (locals.var_t0__blk324 * locals.var_t0__blk324)) * locals.var_cnst1bulk) - (assign12570_e14526 * locals.var_cnst1bulk_dn10)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12570_e14528) * assign12570_e14534) - (assign12570_e14529 * (locals.var_beta_dn10 + (-((2.0 * locals.var_t1__blk325_dn10) / (locals.var_t1__blk325 * locals.var_t1__blk325)))))) / (assign12570_e14534 * assign12570_e14534)), ((((((((((locals.var_t1__blk325_dn11 * locals.var_t1__blk325) + (locals.var_t1__blk325 * locals.var_t1__blk325_dn11)) / locals.var_t0__blk324) * locals.var_cnst1bulk) - (assign12570_e14526 * locals.var_cnst1bulk_dn11)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12570_e14528) * assign12570_e14534) - (assign12570_e14529 * (-((2.0 * locals.var_t1__blk325_dn11) / (locals.var_t1__blk325 * locals.var_t1__blk325))))) / (assign12570_e14534 * assign12570_e14534)), ((((((((((locals.var_t1__blk325_dn12 * locals.var_t1__blk325) + (locals.var_t1__blk325 * locals.var_t1__blk325_dn12)) / locals.var_t0__blk324) * locals.var_cnst1bulk) - (assign12570_e14526 * locals.var_cnst1bulk_dn12)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12570_e14528) * assign12570_e14534) - (assign12570_e14529 * (-((2.0 * locals.var_t1__blk325_dn12) / (locals.var_t1__blk325 * locals.var_t1__blk325))))) / (assign12570_e14534 * assign12570_e14534)), ((((((((((locals.var_t1__blk325_dn17 * locals.var_t1__blk325) + (locals.var_t1__blk325 * locals.var_t1__blk325_dn17)) / locals.var_t0__blk324) * locals.var_cnst1bulk) - (assign12570_e14526 * locals.var_cnst1bulk_dn17)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12570_e14528) * assign12570_e14534) - (assign12570_e14529 * (-((2.0 * locals.var_t1__blk325_dn17) / (locals.var_t1__blk325 * locals.var_t1__blk325))))) / (assign12570_e14534 * assign12570_e14534)),)
    } else {
        (locals.var_psb_inib__blk329, locals.var_psb_inib__blk329_dn0, locals.var_psb_inib__blk329_dn2, locals.var_psb_inib__blk329_dn6, locals.var_psb_inib__blk329_dn7, locals.var_psb_inib__blk329_dn10, locals.var_psb_inib__blk329_dn11, locals.var_psb_inib__blk329_dn12, locals.var_psb_inib__blk329_dn17,)
    }
};
        locals.var_psb_inib__blk329 = assign12570_e14537;
        locals.var_psb_inib__blk329_dn0 = assign12570_e14537_d_n0;
        locals.var_psb_inib__blk329_dn2 = assign12570_e14537_d_n2;
        locals.var_psb_inib__blk329_dn6 = assign12570_e14537_d_n6;
        locals.var_psb_inib__blk329_dn7 = assign12570_e14537_d_n7;
        locals.var_psb_inib__blk329_dn10 = assign12570_e14537_d_n10;
        locals.var_psb_inib__blk329_dn11 = assign12570_e14537_d_n11;
        locals.var_psb_inib__blk329_dn12 = assign12570_e14537_d_n12;
        locals.var_psb_inib__blk329_dn17 = assign12570_e14537_d_n17;

        let assign12580_e14540: f64 = if locals.var_psb_inia__blk328 < locals.var_pb2_bulk { 1.0 } else { 0.0 };
        locals.var_guard332 = assign12580_e14540;

        let (assign12590_e14558, assign12590_e14558_d_n0, assign12590_e14558_d_n2, assign12590_e14558_d_n6, assign12590_e14558_d_n7, assign12590_e14558_d_n10, assign12590_e14558_d_n11, assign12590_e14558_d_n12, assign12590_e14558_d_n17,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard330 == 0.0)) && (locals.var_guard332 != 0.0)) {
        (locals.var_psb_inia__blk328, locals.var_psb_inia__blk328_dn0, locals.var_psb_inia__blk328_dn2, locals.var_psb_inia__blk328_dn6, locals.var_psb_inia__blk328_dn7, locals.var_psb_inia__blk328_dn10, locals.var_psb_inia__blk328_dn11, locals.var_psb_inia__blk328_dn12, locals.var_psb_inia__blk328_dn17,)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
        locals.var_phi_sl_bulk = assign12590_e14558;
        locals.var_phi_sl_bulk_dn0 = assign12590_e14558_d_n0;
        locals.var_phi_sl_bulk_dn2 = assign12590_e14558_d_n2;
        locals.var_phi_sl_bulk_dn6 = assign12590_e14558_d_n6;
        locals.var_phi_sl_bulk_dn7 = assign12590_e14558_d_n7;
        locals.var_phi_sl_bulk_dn10 = assign12590_e14558_d_n10;
        locals.var_phi_sl_bulk_dn11 = assign12590_e14558_d_n11;
        locals.var_phi_sl_bulk_dn12 = assign12590_e14558_d_n12;
        locals.var_phi_sl_bulk_dn17 = assign12590_e14558_d_n17;

        let (assign12600_e14581, assign12600_e14581_d_n0, assign12600_e14581_d_n2, assign12600_e14581_d_n6, assign12600_e14581_d_n7, assign12600_e14581_d_n10, assign12600_e14581_d_n11, assign12600_e14581_d_n12, assign12600_e14581_d_n17,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard330 == 0.0)) && (locals.var_guard332 == 0.0)) {
        let assign12600_e14577: f64 = (locals.var_psb_inib__blk329 - locals.var_psb_inia__blk328);
        let assign12600_e14579: f64 = (assign12600_e14577 - 0.0008);
        (assign12600_e14579, (locals.var_psb_inib__blk329_dn0 - locals.var_psb_inia__blk328_dn0), (locals.var_psb_inib__blk329_dn2 - locals.var_psb_inia__blk328_dn2), (locals.var_psb_inib__blk329_dn6 - locals.var_psb_inia__blk328_dn6), (locals.var_psb_inib__blk329_dn7 - locals.var_psb_inia__blk328_dn7), (locals.var_psb_inib__blk329_dn10 - locals.var_psb_inia__blk328_dn10), (locals.var_psb_inib__blk329_dn11 - locals.var_psb_inia__blk328_dn11), (locals.var_psb_inib__blk329_dn12 - locals.var_psb_inia__blk328_dn12), (locals.var_psb_inib__blk329_dn17 - locals.var_psb_inia__blk328_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign12600_e14581;
        locals.var_tmf1_dn0 = assign12600_e14581_d_n0;
        locals.var_tmf1_dn2 = assign12600_e14581_d_n2;
        locals.var_tmf1_dn6 = assign12600_e14581_d_n6;
        locals.var_tmf1_dn7 = assign12600_e14581_d_n7;
        locals.var_tmf1_dn10 = assign12600_e14581_d_n10;
        locals.var_tmf1_dn11 = assign12600_e14581_d_n11;
        locals.var_tmf1_dn12 = assign12600_e14581_d_n12;
        locals.var_tmf1_dn17 = assign12600_e14581_d_n17;

        let (assign12610_e14604, assign12610_e14604_d_n0, assign12610_e14604_d_n2, assign12610_e14604_d_n6, assign12610_e14604_d_n7, assign12610_e14604_d_n10, assign12610_e14604_d_n11, assign12610_e14604_d_n12, assign12610_e14604_d_n17,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard330 == 0.0)) && (locals.var_guard332 == 0.0)) {
        let assign12610_e14600: f64 = (4.0 * locals.var_psb_inib__blk329);
        let assign12610_e14602: f64 = (assign12610_e14600 * 0.0008);
        (assign12610_e14602, ((4.0 * locals.var_psb_inib__blk329_dn0) * 0.0008), ((4.0 * locals.var_psb_inib__blk329_dn2) * 0.0008), ((4.0 * locals.var_psb_inib__blk329_dn6) * 0.0008), ((4.0 * locals.var_psb_inib__blk329_dn7) * 0.0008), ((4.0 * locals.var_psb_inib__blk329_dn10) * 0.0008), ((4.0 * locals.var_psb_inib__blk329_dn11) * 0.0008), ((4.0 * locals.var_psb_inib__blk329_dn12) * 0.0008), ((4.0 * locals.var_psb_inib__blk329_dn17) * 0.0008),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign12610_e14604;
        locals.var_tmf2_dn0 = assign12610_e14604_d_n0;
        locals.var_tmf2_dn2 = assign12610_e14604_d_n2;
        locals.var_tmf2_dn6 = assign12610_e14604_d_n6;
        locals.var_tmf2_dn7 = assign12610_e14604_d_n7;
        locals.var_tmf2_dn10 = assign12610_e14604_d_n10;
        locals.var_tmf2_dn11 = assign12610_e14604_d_n11;
        locals.var_tmf2_dn12 = assign12610_e14604_d_n12;
        locals.var_tmf2_dn17 = assign12610_e14604_d_n17;

        let (assign12620_e14629, assign12620_e14629_d_n0, assign12620_e14629_d_n2, assign12620_e14629_d_n6, assign12620_e14629_d_n7, assign12620_e14629_d_n10, assign12620_e14629_d_n11, assign12620_e14629_d_n12, assign12620_e14629_d_n17,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard330 == 0.0)) && (locals.var_guard332 == 0.0)) {
        let (assign12620_e14627, assign12620_e14627_d_n0, assign12620_e14627_d_n2, assign12620_e14627_d_n6, assign12620_e14627_d_n7, assign12620_e14627_d_n10, assign12620_e14627_d_n11, assign12620_e14627_d_n12, assign12620_e14627_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign12620_e14626: f64 = (-locals.var_tmf2);
                (assign12620_e14626, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign12620_e14627, assign12620_e14627_d_n0, assign12620_e14627_d_n2, assign12620_e14627_d_n6, assign12620_e14627_d_n7, assign12620_e14627_d_n10, assign12620_e14627_d_n11, assign12620_e14627_d_n12, assign12620_e14627_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign12620_e14629;
        locals.var_tmf2_dn0 = assign12620_e14629_d_n0;
        locals.var_tmf2_dn2 = assign12620_e14629_d_n2;
        locals.var_tmf2_dn6 = assign12620_e14629_d_n6;
        locals.var_tmf2_dn7 = assign12620_e14629_d_n7;
        locals.var_tmf2_dn10 = assign12620_e14629_d_n10;
        locals.var_tmf2_dn11 = assign12620_e14629_d_n11;
        locals.var_tmf2_dn12 = assign12620_e14629_d_n12;
        locals.var_tmf2_dn17 = assign12620_e14629_d_n17;

        let (assign12630_e14653, assign12630_e14653_d_n0, assign12630_e14653_d_n2, assign12630_e14653_d_n6, assign12630_e14653_d_n7, assign12630_e14653_d_n10, assign12630_e14653_d_n11, assign12630_e14653_d_n12, assign12630_e14653_d_n17,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard330 == 0.0)) && (locals.var_guard332 == 0.0)) {
        let assign12630_e14648: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign12630_e14650: f64 = (assign12630_e14648 + locals.var_tmf2);
        let assign12630_e14651: f64 = (assign12630_e14650).sqrt();
        (assign12630_e14651, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign12630_e14651)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign12630_e14651)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign12630_e14651)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign12630_e14651)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign12630_e14651)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign12630_e14651)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign12630_e14651)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign12630_e14651)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign12630_e14653;
        locals.var_tmf2_dn0 = assign12630_e14653_d_n0;
        locals.var_tmf2_dn2 = assign12630_e14653_d_n2;
        locals.var_tmf2_dn6 = assign12630_e14653_d_n6;
        locals.var_tmf2_dn7 = assign12630_e14653_d_n7;
        locals.var_tmf2_dn10 = assign12630_e14653_d_n10;
        locals.var_tmf2_dn11 = assign12630_e14653_d_n11;
        locals.var_tmf2_dn12 = assign12630_e14653_d_n12;
        locals.var_tmf2_dn17 = assign12630_e14653_d_n17;

        let (assign12640_e14678, assign12640_e14678_d_n0, assign12640_e14678_d_n2, assign12640_e14678_d_n6, assign12640_e14678_d_n7, assign12640_e14678_d_n10, assign12640_e14678_d_n11, assign12640_e14678_d_n12, assign12640_e14678_d_n17,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard330 == 0.0)) && (locals.var_guard332 == 0.0)) {
        let assign12640_e14674: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign12640_e14675: f64 = (0.5 * assign12640_e14674);
        let assign12640_e14676: f64 = (locals.var_psb_inib__blk329 - assign12640_e14675);
        (assign12640_e14676, (locals.var_psb_inib__blk329_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psb_inib__blk329_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psb_inib__blk329_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psb_inib__blk329_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psb_inib__blk329_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psb_inib__blk329_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psb_inib__blk329_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_psb_inib__blk329_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
        locals.var_phi_sl_bulk = assign12640_e14678;
        locals.var_phi_sl_bulk_dn0 = assign12640_e14678_d_n0;
        locals.var_phi_sl_bulk_dn2 = assign12640_e14678_d_n2;
        locals.var_phi_sl_bulk_dn6 = assign12640_e14678_d_n6;
        locals.var_phi_sl_bulk_dn7 = assign12640_e14678_d_n7;
        locals.var_phi_sl_bulk_dn10 = assign12640_e14678_d_n10;
        locals.var_phi_sl_bulk_dn11 = assign12640_e14678_d_n11;
        locals.var_phi_sl_bulk_dn12 = assign12640_e14678_d_n12;
        locals.var_phi_sl_bulk_dn17 = assign12640_e14678_d_n17;

        let (assign12650_e14699, assign12650_e14699_d_n0, assign12650_e14699_d_n2, assign12650_e14699_d_n6, assign12650_e14699_d_n7, assign12650_e14699_d_n10, assign12650_e14699_d_n11, assign12650_e14699_d_n12, assign12650_e14699_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) {
        let assign12650_e14691: f64 = (2.0 * 1.034943e-10);
        let assign12650_e14693: f64 = (assign12650_e14691 / 1.6021918e-19);
        let assign12650_e14695: f64 = (assign12650_e14693 * locals.var_phi_sl_soi);
        let assign12650_e14697: f64 = (assign12650_e14695 / locals.var_uc_nsubs);
        (assign12650_e14697, ((((assign12650_e14693 * locals.var_phi_sl_soi_dn0) * locals.var_uc_nsubs) - (assign12650_e14695 * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((assign12650_e14693 * locals.var_phi_sl_soi_dn2) * locals.var_uc_nsubs) - (assign12650_e14695 * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((assign12650_e14693 * locals.var_phi_sl_soi_dn6) * locals.var_uc_nsubs) - (assign12650_e14695 * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((assign12650_e14693 * locals.var_phi_sl_soi_dn7) * locals.var_uc_nsubs) - (assign12650_e14695 * locals.var_uc_nsubs_dn7)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((assign12650_e14693 * locals.var_phi_sl_soi_dn10) * locals.var_uc_nsubs) - (assign12650_e14695 * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((assign12650_e14693 * locals.var_phi_sl_soi_dn11) * locals.var_uc_nsubs) - (assign12650_e14695 * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((assign12650_e14693 * locals.var_phi_sl_soi_dn12) * locals.var_uc_nsubs) - (assign12650_e14695 * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((assign12650_e14693 * locals.var_phi_sl_soi_dn17) * locals.var_uc_nsubs) - (assign12650_e14695 * locals.var_uc_nsubs_dn17)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)),)
    } else {
        (locals.var_t0__blk333, locals.var_t0__blk333_dn0, locals.var_t0__blk333_dn2, locals.var_t0__blk333_dn6, locals.var_t0__blk333_dn7, locals.var_t0__blk333_dn10, locals.var_t0__blk333_dn11, locals.var_t0__blk333_dn12, locals.var_t0__blk333_dn17,)
    }
};
        locals.var_t0__blk333 = assign12650_e14699;
        locals.var_t0__blk333_dn0 = assign12650_e14699_d_n0;
        locals.var_t0__blk333_dn2 = assign12650_e14699_d_n2;
        locals.var_t0__blk333_dn6 = assign12650_e14699_d_n6;
        locals.var_t0__blk333_dn7 = assign12650_e14699_d_n7;
        locals.var_t0__blk333_dn10 = assign12650_e14699_d_n10;
        locals.var_t0__blk333_dn11 = assign12650_e14699_d_n11;
        locals.var_t0__blk333_dn12 = assign12650_e14699_d_n12;
        locals.var_t0__blk333_dn17 = assign12650_e14699_d_n17;

        let assign12660_e14702: f64 = if locals.var_t0__blk333 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard341 = assign12660_e14702;

        let (assign12670_e14726, assign12670_e14726_d_n0, assign12670_e14726_d_n2, assign12670_e14726_d_n6, assign12670_e14726_d_n7, assign12670_e14726_d_n10, assign12670_e14726_d_n11, assign12670_e14726_d_n12, assign12670_e14726_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard341 != 0.0)) {
        let assign12670_e14717: f64 = (2.0 * 1.034943e-10);
        let assign12670_e14719: f64 = (assign12670_e14717 / 1.6021918e-19);
        let assign12670_e14721: f64 = (assign12670_e14719 * locals.var_phi_sl_soi);
        let assign12670_e14723: f64 = (assign12670_e14721 / locals.var_uc_nsubs);
        let assign12670_e14724: f64 = (assign12670_e14723).sqrt();
        (assign12670_e14724, (((((assign12670_e14719 * locals.var_phi_sl_soi_dn0) * locals.var_uc_nsubs) - (assign12670_e14721 * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign12670_e14724)), (((((assign12670_e14719 * locals.var_phi_sl_soi_dn2) * locals.var_uc_nsubs) - (assign12670_e14721 * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign12670_e14724)), (((((assign12670_e14719 * locals.var_phi_sl_soi_dn6) * locals.var_uc_nsubs) - (assign12670_e14721 * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign12670_e14724)), (((((assign12670_e14719 * locals.var_phi_sl_soi_dn7) * locals.var_uc_nsubs) - (assign12670_e14721 * locals.var_uc_nsubs_dn7)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign12670_e14724)), (((((assign12670_e14719 * locals.var_phi_sl_soi_dn10) * locals.var_uc_nsubs) - (assign12670_e14721 * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign12670_e14724)), (((((assign12670_e14719 * locals.var_phi_sl_soi_dn11) * locals.var_uc_nsubs) - (assign12670_e14721 * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign12670_e14724)), (((((assign12670_e14719 * locals.var_phi_sl_soi_dn12) * locals.var_uc_nsubs) - (assign12670_e14721 * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign12670_e14724)), (((((assign12670_e14719 * locals.var_phi_sl_soi_dn17) * locals.var_uc_nsubs) - (assign12670_e14721 * locals.var_uc_nsubs_dn17)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign12670_e14724)),)
    } else {
        (locals.var_wdsoi, locals.var_wdsoi_dn0, locals.var_wdsoi_dn2, locals.var_wdsoi_dn6, locals.var_wdsoi_dn7, locals.var_wdsoi_dn10, locals.var_wdsoi_dn11, locals.var_wdsoi_dn12, locals.var_wdsoi_dn17,)
    }
};
        locals.var_wdsoi = assign12670_e14726;
        locals.var_wdsoi_dn0 = assign12670_e14726_d_n0;
        locals.var_wdsoi_dn2 = assign12670_e14726_d_n2;
        locals.var_wdsoi_dn6 = assign12670_e14726_d_n6;
        locals.var_wdsoi_dn7 = assign12670_e14726_d_n7;
        locals.var_wdsoi_dn10 = assign12670_e14726_d_n10;
        locals.var_wdsoi_dn11 = assign12670_e14726_d_n11;
        locals.var_wdsoi_dn12 = assign12670_e14726_d_n12;
        locals.var_wdsoi_dn17 = assign12670_e14726_d_n17;

        let (assign12680_e14742, assign12680_e14742_d_n0, assign12680_e14742_d_n2, assign12680_e14742_d_n6, assign12680_e14742_d_n7, assign12680_e14742_d_n10, assign12680_e14742_d_n11, assign12680_e14742_d_n12, assign12680_e14742_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard341 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wdsoi, locals.var_wdsoi_dn0, locals.var_wdsoi_dn2, locals.var_wdsoi_dn6, locals.var_wdsoi_dn7, locals.var_wdsoi_dn10, locals.var_wdsoi_dn11, locals.var_wdsoi_dn12, locals.var_wdsoi_dn17,)
    }
};
        locals.var_wdsoi = assign12680_e14742;
        locals.var_wdsoi_dn0 = assign12680_e14742_d_n0;
        locals.var_wdsoi_dn2 = assign12680_e14742_d_n2;
        locals.var_wdsoi_dn6 = assign12680_e14742_d_n6;
        locals.var_wdsoi_dn7 = assign12680_e14742_d_n7;
        locals.var_wdsoi_dn10 = assign12680_e14742_d_n10;
        locals.var_wdsoi_dn11 = assign12680_e14742_d_n11;
        locals.var_wdsoi_dn12 = assign12680_e14742_d_n12;
        locals.var_wdsoi_dn17 = assign12680_e14742_d_n17;

        let assign12690_e14747: f64 = if ((locals.var_phi_sl_soi < locals.var_fd_end) && (0.0 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard342 = assign12690_e14747;

        let (assign12710_e14777,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_lp_sl,)
    }
};
        locals.var_lp_sl = assign12710_e14777;

    }

    pub(super) fn stamp_transient_block_37(
        locals: &mut StampLocals,
    ) {
        let mut assign12720_loop_guard: usize = 0;
        while {
            let assign12720_cond_e14793: f64 = if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 != 0.0)) && (locals.var_lp_sl < locals.var_lp_sl_max)) { 1.0 } else { 0.0 };
            assign12720_cond_e14793 != 0.0
        } {
            assign12720_loop_guard += 1;
            assert!(assign12720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign12720_body0_e14808, assign12720_body0_e14808_d_n10,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 != 0.0)) {
        (locals.var_cnst0bulk, locals.var_cnst0bulk_dn10,)
    } else {
        (locals.var_t1__blk334, locals.var_t1__blk334_dn10,)
    }
};
            locals.var_t1__blk334 = assign12720_body0_e14808;
            locals.var_t1__blk334_dn10 = assign12720_body0_e14808_d_n10;
            let (assign12720_body1_e14825, assign12720_body1_e14825_d_n0, assign12720_body1_e14825_d_n2, assign12720_body1_e14825_d_n6, assign12720_body1_e14825_d_n7, assign12720_body1_e14825_d_n10, assign12720_body1_e14825_d_n11, assign12720_body1_e14825_d_n12, assign12720_body1_e14825_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 != 0.0)) {
        let assign12720_body1_e14823: f64 = (locals.var_beta * locals.var_phi_sl_bulk);
        (assign12720_body1_e14823, (locals.var_beta * locals.var_phi_sl_bulk_dn0), (locals.var_beta * locals.var_phi_sl_bulk_dn2), (locals.var_beta * locals.var_phi_sl_bulk_dn6), (locals.var_beta * locals.var_phi_sl_bulk_dn7), ((locals.var_beta_dn10 * locals.var_phi_sl_bulk) + (locals.var_beta * locals.var_phi_sl_bulk_dn10)), (locals.var_beta * locals.var_phi_sl_bulk_dn11), (locals.var_beta * locals.var_phi_sl_bulk_dn12), (locals.var_beta * locals.var_phi_sl_bulk_dn17),)
    } else {
        (locals.var_t2__blk335, locals.var_t2__blk335_dn0, locals.var_t2__blk335_dn2, locals.var_t2__blk335_dn6, locals.var_t2__blk335_dn7, locals.var_t2__blk335_dn10, locals.var_t2__blk335_dn11, locals.var_t2__blk335_dn12, locals.var_t2__blk335_dn17,)
    }
};
            locals.var_t2__blk335 = assign12720_body1_e14825;
            locals.var_t2__blk335_dn0 = assign12720_body1_e14825_d_n0;
            locals.var_t2__blk335_dn2 = assign12720_body1_e14825_d_n2;
            locals.var_t2__blk335_dn6 = assign12720_body1_e14825_d_n6;
            locals.var_t2__blk335_dn7 = assign12720_body1_e14825_d_n7;
            locals.var_t2__blk335_dn10 = assign12720_body1_e14825_d_n10;
            locals.var_t2__blk335_dn11 = assign12720_body1_e14825_d_n11;
            locals.var_t2__blk335_dn12 = assign12720_body1_e14825_d_n12;
            locals.var_t2__blk335_dn17 = assign12720_body1_e14825_d_n17;
            let (assign12720_body2_e14842, assign12720_body2_e14842_d_n0, assign12720_body2_e14842_d_n2, assign12720_body2_e14842_d_n6, assign12720_body2_e14842_d_n7, assign12720_body2_e14842_d_n10, assign12720_body2_e14842_d_n11, assign12720_body2_e14842_d_n12, assign12720_body2_e14842_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 != 0.0)) {
        let assign12720_body2_e14839: f64 = (-locals.var_t2__blk335);
        let assign12720_body2_e14840: f64 = (assign12720_body2_e14839).exp();
        (assign12720_body2_e14840, (assign12720_body2_e14840 * (-locals.var_t2__blk335_dn0)), (assign12720_body2_e14840 * (-locals.var_t2__blk335_dn2)), (assign12720_body2_e14840 * (-locals.var_t2__blk335_dn6)), (assign12720_body2_e14840 * (-locals.var_t2__blk335_dn7)), (assign12720_body2_e14840 * (-locals.var_t2__blk335_dn10)), (assign12720_body2_e14840 * (-locals.var_t2__blk335_dn11)), (assign12720_body2_e14840 * (-locals.var_t2__blk335_dn12)), (assign12720_body2_e14840 * (-locals.var_t2__blk335_dn17)),)
    } else {
        (locals.var_t3__blk336, locals.var_t3__blk336_dn0, locals.var_t3__blk336_dn2, locals.var_t3__blk336_dn6, locals.var_t3__blk336_dn7, locals.var_t3__blk336_dn10, locals.var_t3__blk336_dn11, locals.var_t3__blk336_dn12, locals.var_t3__blk336_dn17,)
    }
};
            locals.var_t3__blk336 = assign12720_body2_e14842;
            locals.var_t3__blk336_dn0 = assign12720_body2_e14842_d_n0;
            locals.var_t3__blk336_dn2 = assign12720_body2_e14842_d_n2;
            locals.var_t3__blk336_dn6 = assign12720_body2_e14842_d_n6;
            locals.var_t3__blk336_dn7 = assign12720_body2_e14842_d_n7;
            locals.var_t3__blk336_dn10 = assign12720_body2_e14842_d_n10;
            locals.var_t3__blk336_dn11 = assign12720_body2_e14842_d_n11;
            locals.var_t3__blk336_dn12 = assign12720_body2_e14842_d_n12;
            locals.var_t3__blk336_dn17 = assign12720_body2_e14842_d_n17;
            let assign12720_body3_e14845: f64 = if locals.var_phi_sl_bulk > 1e-9 { 1.0 } else { 0.0 };
            locals.var_guard343 = assign12720_body3_e14845;
            let (assign12720_body4_e14865, assign12720_body4_e14865_d_n0, assign12720_body4_e14865_d_n2, assign12720_body4_e14865_d_n6, assign12720_body4_e14865_d_n7, assign12720_body4_e14865_d_n10, assign12720_body4_e14865_d_n11, assign12720_body4_e14865_d_n12, assign12720_body4_e14865_d_n17,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 != 0.0)) && (locals.var_guard343 != 0.0)) {
        let assign12720_body4_e14862: f64 = (locals.var_beta * locals.var_phi_sl_bulk);
        let assign12720_body4_e14863: f64 = (assign12720_body4_e14862).exp();
        (assign12720_body4_e14863, (assign12720_body4_e14863 * (locals.var_beta * locals.var_phi_sl_bulk_dn0)), (assign12720_body4_e14863 * (locals.var_beta * locals.var_phi_sl_bulk_dn2)), (assign12720_body4_e14863 * (locals.var_beta * locals.var_phi_sl_bulk_dn6)), (assign12720_body4_e14863 * (locals.var_beta * locals.var_phi_sl_bulk_dn7)), (assign12720_body4_e14863 * ((locals.var_beta_dn10 * locals.var_phi_sl_bulk) + (locals.var_beta * locals.var_phi_sl_bulk_dn10))), (assign12720_body4_e14863 * (locals.var_beta * locals.var_phi_sl_bulk_dn11)), (assign12720_body4_e14863 * (locals.var_beta * locals.var_phi_sl_bulk_dn12)), (assign12720_body4_e14863 * (locals.var_beta * locals.var_phi_sl_bulk_dn17)),)
    } else {
        (locals.var_t0__blk333, locals.var_t0__blk333_dn0, locals.var_t0__blk333_dn2, locals.var_t0__blk333_dn6, locals.var_t0__blk333_dn7, locals.var_t0__blk333_dn10, locals.var_t0__blk333_dn11, locals.var_t0__blk333_dn12, locals.var_t0__blk333_dn17,)
    }
};
            locals.var_t0__blk333 = assign12720_body4_e14865;
            locals.var_t0__blk333_dn0 = assign12720_body4_e14865_d_n0;
            locals.var_t0__blk333_dn2 = assign12720_body4_e14865_d_n2;
            locals.var_t0__blk333_dn6 = assign12720_body4_e14865_d_n6;
            locals.var_t0__blk333_dn7 = assign12720_body4_e14865_d_n7;
            locals.var_t0__blk333_dn10 = assign12720_body4_e14865_d_n10;
            locals.var_t0__blk333_dn11 = assign12720_body4_e14865_d_n11;
            locals.var_t0__blk333_dn12 = assign12720_body4_e14865_d_n12;
            locals.var_t0__blk333_dn17 = assign12720_body4_e14865_d_n17;
            let (assign12720_body5_e14896, assign12720_body5_e14896_d_n0, assign12720_body5_e14896_d_n2, assign12720_body5_e14896_d_n6, assign12720_body5_e14896_d_n7, assign12720_body5_e14896_d_n10, assign12720_body5_e14896_d_n11, assign12720_body5_e14896_d_n12, assign12720_body5_e14896_d_n17,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 != 0.0)) && (locals.var_guard343 != 0.0)) {
        let assign12720_body5_e14881: f64 = (-locals.var_t1__blk334);
        let assign12720_body5_e14884: f64 = (locals.var_t3__blk336 + locals.var_t2__blk335);
        let assign12720_body5_e14886: f64 = (assign12720_body5_e14884 - 1.0);
        let assign12720_body5_e14890: f64 = (locals.var_t0__blk333 - 1.0);
        let assign12720_body5_e14891: f64 = (locals.var_cnst1bulk * assign12720_body5_e14890);
        let assign12720_body5_e14892: f64 = (assign12720_body5_e14886 + assign12720_body5_e14891);
        let assign12720_body5_e14893: f64 = (assign12720_body5_e14892).sqrt();
        let assign12720_body5_e14894: f64 = (assign12720_body5_e14881 * assign12720_body5_e14893);
        (assign12720_body5_e14894, (assign12720_body5_e14881 * (((locals.var_t3__blk336_dn0 + locals.var_t2__blk335_dn0) + ((locals.var_cnst1bulk_dn0 * assign12720_body5_e14890) + (locals.var_cnst1bulk * locals.var_t0__blk333_dn0))) / (2.0 * assign12720_body5_e14893))), (assign12720_body5_e14881 * (((locals.var_t3__blk336_dn2 + locals.var_t2__blk335_dn2) + ((locals.var_cnst1bulk_dn2 * assign12720_body5_e14890) + (locals.var_cnst1bulk * locals.var_t0__blk333_dn2))) / (2.0 * assign12720_body5_e14893))), (assign12720_body5_e14881 * (((locals.var_t3__blk336_dn6 + locals.var_t2__blk335_dn6) + ((locals.var_cnst1bulk_dn6 * assign12720_body5_e14890) + (locals.var_cnst1bulk * locals.var_t0__blk333_dn6))) / (2.0 * assign12720_body5_e14893))), (assign12720_body5_e14881 * (((locals.var_t3__blk336_dn7 + locals.var_t2__blk335_dn7) + ((locals.var_cnst1bulk_dn7 * assign12720_body5_e14890) + (locals.var_cnst1bulk * locals.var_t0__blk333_dn7))) / (2.0 * assign12720_body5_e14893))), (((-locals.var_t1__blk334_dn10) * assign12720_body5_e14893) + (assign12720_body5_e14881 * (((locals.var_t3__blk336_dn10 + locals.var_t2__blk335_dn10) + ((locals.var_cnst1bulk_dn10 * assign12720_body5_e14890) + (locals.var_cnst1bulk * locals.var_t0__blk333_dn10))) / (2.0 * assign12720_body5_e14893)))), (assign12720_body5_e14881 * (((locals.var_t3__blk336_dn11 + locals.var_t2__blk335_dn11) + ((locals.var_cnst1bulk_dn11 * assign12720_body5_e14890) + (locals.var_cnst1bulk * locals.var_t0__blk333_dn11))) / (2.0 * assign12720_body5_e14893))), (assign12720_body5_e14881 * (((locals.var_t3__blk336_dn12 + locals.var_t2__blk335_dn12) + ((locals.var_cnst1bulk_dn12 * assign12720_body5_e14890) + (locals.var_cnst1bulk * locals.var_t0__blk333_dn12))) / (2.0 * assign12720_body5_e14893))), (assign12720_body5_e14881 * (((locals.var_t3__blk336_dn17 + locals.var_t2__blk335_dn17) + ((locals.var_cnst1bulk_dn17 * assign12720_body5_e14890) + (locals.var_cnst1bulk * locals.var_t0__blk333_dn17))) / (2.0 * assign12720_body5_e14893))),)
    } else {
        (locals.var_t4__blk337, locals.var_t4__blk337_dn0, locals.var_t4__blk337_dn2, locals.var_t4__blk337_dn6, locals.var_t4__blk337_dn7, locals.var_t4__blk337_dn10, locals.var_t4__blk337_dn11, locals.var_t4__blk337_dn12, locals.var_t4__blk337_dn17,)
    }
};
            locals.var_t4__blk337 = assign12720_body5_e14896;
            locals.var_t4__blk337_dn0 = assign12720_body5_e14896_d_n0;
            locals.var_t4__blk337_dn2 = assign12720_body5_e14896_d_n2;
            locals.var_t4__blk337_dn6 = assign12720_body5_e14896_d_n6;
            locals.var_t4__blk337_dn7 = assign12720_body5_e14896_d_n7;
            locals.var_t4__blk337_dn10 = assign12720_body5_e14896_d_n10;
            locals.var_t4__blk337_dn11 = assign12720_body5_e14896_d_n11;
            locals.var_t4__blk337_dn12 = assign12720_body5_e14896_d_n12;
            locals.var_t4__blk337_dn17 = assign12720_body5_e14896_d_n17;
            let (assign12720_body6_e14924, assign12720_body6_e14924_d_n0, assign12720_body6_e14924_d_n2, assign12720_body6_e14924_d_n6, assign12720_body6_e14924_d_n7, assign12720_body6_e14924_d_n10, assign12720_body6_e14924_d_n11, assign12720_body6_e14924_d_n12, assign12720_body6_e14924_d_n17,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 != 0.0)) && (locals.var_guard343 != 0.0)) {
        let assign12720_body6_e14913: f64 = (locals.var_c0bulk / locals.var_t4__blk337);
        let assign12720_body6_e14915: f64 = (-locals.var_t3__blk336);
        let assign12720_body6_e14917: f64 = (assign12720_body6_e14915 + 1.0);
        let assign12720_body6_e14920: f64 = (locals.var_cnst1bulk * locals.var_t0__blk333);
        let assign12720_body6_e14921: f64 = (assign12720_body6_e14917 + assign12720_body6_e14920);
        let assign12720_body6_e14922: f64 = (assign12720_body6_e14913 * assign12720_body6_e14921);
        (assign12720_body6_e14922, (((-((locals.var_c0bulk * locals.var_t4__blk337_dn0) / (locals.var_t4__blk337 * locals.var_t4__blk337))) * assign12720_body6_e14921) + (assign12720_body6_e14913 * ((-locals.var_t3__blk336_dn0) + ((locals.var_cnst1bulk_dn0 * locals.var_t0__blk333) + (locals.var_cnst1bulk * locals.var_t0__blk333_dn0))))), (((-((locals.var_c0bulk * locals.var_t4__blk337_dn2) / (locals.var_t4__blk337 * locals.var_t4__blk337))) * assign12720_body6_e14921) + (assign12720_body6_e14913 * ((-locals.var_t3__blk336_dn2) + ((locals.var_cnst1bulk_dn2 * locals.var_t0__blk333) + (locals.var_cnst1bulk * locals.var_t0__blk333_dn2))))), (((-((locals.var_c0bulk * locals.var_t4__blk337_dn6) / (locals.var_t4__blk337 * locals.var_t4__blk337))) * assign12720_body6_e14921) + (assign12720_body6_e14913 * ((-locals.var_t3__blk336_dn6) + ((locals.var_cnst1bulk_dn6 * locals.var_t0__blk333) + (locals.var_cnst1bulk * locals.var_t0__blk333_dn6))))), (((-((locals.var_c0bulk * locals.var_t4__blk337_dn7) / (locals.var_t4__blk337 * locals.var_t4__blk337))) * assign12720_body6_e14921) + (assign12720_body6_e14913 * ((-locals.var_t3__blk336_dn7) + ((locals.var_cnst1bulk_dn7 * locals.var_t0__blk333) + (locals.var_cnst1bulk * locals.var_t0__blk333_dn7))))), (((-((locals.var_c0bulk * locals.var_t4__blk337_dn10) / (locals.var_t4__blk337 * locals.var_t4__blk337))) * assign12720_body6_e14921) + (assign12720_body6_e14913 * ((-locals.var_t3__blk336_dn10) + ((locals.var_cnst1bulk_dn10 * locals.var_t0__blk333) + (locals.var_cnst1bulk * locals.var_t0__blk333_dn10))))), (((-((locals.var_c0bulk * locals.var_t4__blk337_dn11) / (locals.var_t4__blk337 * locals.var_t4__blk337))) * assign12720_body6_e14921) + (assign12720_body6_e14913 * ((-locals.var_t3__blk336_dn11) + ((locals.var_cnst1bulk_dn11 * locals.var_t0__blk333) + (locals.var_cnst1bulk * locals.var_t0__blk333_dn11))))), (((-((locals.var_c0bulk * locals.var_t4__blk337_dn12) / (locals.var_t4__blk337 * locals.var_t4__blk337))) * assign12720_body6_e14921) + (assign12720_body6_e14913 * ((-locals.var_t3__blk336_dn12) + ((locals.var_cnst1bulk_dn12 * locals.var_t0__blk333) + (locals.var_cnst1bulk * locals.var_t0__blk333_dn12))))), (((-((locals.var_c0bulk * locals.var_t4__blk337_dn17) / (locals.var_t4__blk337 * locals.var_t4__blk337))) * assign12720_body6_e14921) + (assign12720_body6_e14913 * ((-locals.var_t3__blk336_dn17) + ((locals.var_cnst1bulk_dn17 * locals.var_t0__blk333) + (locals.var_cnst1bulk * locals.var_t0__blk333_dn17))))),)
    } else {
        (locals.var_t5__blk338, locals.var_t5__blk338_dn0, locals.var_t5__blk338_dn2, locals.var_t5__blk338_dn6, locals.var_t5__blk338_dn7, locals.var_t5__blk338_dn10, locals.var_t5__blk338_dn11, locals.var_t5__blk338_dn12, locals.var_t5__blk338_dn17,)
    }
};
            locals.var_t5__blk338 = assign12720_body6_e14924;
            locals.var_t5__blk338_dn0 = assign12720_body6_e14924_d_n0;
            locals.var_t5__blk338_dn2 = assign12720_body6_e14924_d_n2;
            locals.var_t5__blk338_dn6 = assign12720_body6_e14924_d_n6;
            locals.var_t5__blk338_dn7 = assign12720_body6_e14924_d_n7;
            locals.var_t5__blk338_dn10 = assign12720_body6_e14924_d_n10;
            locals.var_t5__blk338_dn11 = assign12720_body6_e14924_d_n11;
            locals.var_t5__blk338_dn12 = assign12720_body6_e14924_d_n12;
            locals.var_t5__blk338_dn17 = assign12720_body6_e14924_d_n17;
            let assign12720_body7_e14927: f64 = (-1e-9);
            let assign12720_body7_e14928: f64 = if locals.var_phi_sl_bulk < assign12720_body7_e14927 { 1.0 } else { 0.0 };
            locals.var_guard344 = assign12720_body7_e14928;
            let (assign12720_body8_e14955, assign12720_body8_e14955_d_n0, assign12720_body8_e14955_d_n2, assign12720_body8_e14955_d_n6, assign12720_body8_e14955_d_n7, assign12720_body8_e14955_d_n10, assign12720_body8_e14955_d_n11, assign12720_body8_e14955_d_n12, assign12720_body8_e14955_d_n17,) = {
    if (((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard344 != 0.0)) {
        let assign12720_body8_e14949: f64 = (locals.var_t3__blk336 + locals.var_t2__blk335);
        let assign12720_body8_e14951: f64 = (assign12720_body8_e14949 - 1.0);
        let assign12720_body8_e14952: f64 = (assign12720_body8_e14951).sqrt();
        let assign12720_body8_e14953: f64 = (locals.var_t1__blk334 * assign12720_body8_e14952);
        (assign12720_body8_e14953, (locals.var_t1__blk334 * ((locals.var_t3__blk336_dn0 + locals.var_t2__blk335_dn0) / (2.0 * assign12720_body8_e14952))), (locals.var_t1__blk334 * ((locals.var_t3__blk336_dn2 + locals.var_t2__blk335_dn2) / (2.0 * assign12720_body8_e14952))), (locals.var_t1__blk334 * ((locals.var_t3__blk336_dn6 + locals.var_t2__blk335_dn6) / (2.0 * assign12720_body8_e14952))), (locals.var_t1__blk334 * ((locals.var_t3__blk336_dn7 + locals.var_t2__blk335_dn7) / (2.0 * assign12720_body8_e14952))), ((locals.var_t1__blk334_dn10 * assign12720_body8_e14952) + (locals.var_t1__blk334 * ((locals.var_t3__blk336_dn10 + locals.var_t2__blk335_dn10) / (2.0 * assign12720_body8_e14952)))), (locals.var_t1__blk334 * ((locals.var_t3__blk336_dn11 + locals.var_t2__blk335_dn11) / (2.0 * assign12720_body8_e14952))), (locals.var_t1__blk334 * ((locals.var_t3__blk336_dn12 + locals.var_t2__blk335_dn12) / (2.0 * assign12720_body8_e14952))), (locals.var_t1__blk334 * ((locals.var_t3__blk336_dn17 + locals.var_t2__blk335_dn17) / (2.0 * assign12720_body8_e14952))),)
    } else {
        (locals.var_t4__blk337, locals.var_t4__blk337_dn0, locals.var_t4__blk337_dn2, locals.var_t4__blk337_dn6, locals.var_t4__blk337_dn7, locals.var_t4__blk337_dn10, locals.var_t4__blk337_dn11, locals.var_t4__blk337_dn12, locals.var_t4__blk337_dn17,)
    }
};
            locals.var_t4__blk337 = assign12720_body8_e14955;
            locals.var_t4__blk337_dn0 = assign12720_body8_e14955_d_n0;
            locals.var_t4__blk337_dn2 = assign12720_body8_e14955_d_n2;
            locals.var_t4__blk337_dn6 = assign12720_body8_e14955_d_n6;
            locals.var_t4__blk337_dn7 = assign12720_body8_e14955_d_n7;
            locals.var_t4__blk337_dn10 = assign12720_body8_e14955_d_n10;
            locals.var_t4__blk337_dn11 = assign12720_body8_e14955_d_n11;
            locals.var_t4__blk337_dn12 = assign12720_body8_e14955_d_n12;
            locals.var_t4__blk337_dn17 = assign12720_body8_e14955_d_n17;
            let (assign12720_body9_e14982, assign12720_body9_e14982_d_n0, assign12720_body9_e14982_d_n2, assign12720_body9_e14982_d_n6, assign12720_body9_e14982_d_n7, assign12720_body9_e14982_d_n10, assign12720_body9_e14982_d_n11, assign12720_body9_e14982_d_n12, assign12720_body9_e14982_d_n17,) = {
    if (((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard344 != 0.0)) {
        let assign12720_body9_e14975: f64 = (locals.var_c0bulk / locals.var_t4__blk337);
        let assign12720_body9_e14977: f64 = (-locals.var_t3__blk336);
        let assign12720_body9_e14979: f64 = (assign12720_body9_e14977 + 1.0);
        let assign12720_body9_e14980: f64 = (assign12720_body9_e14975 * assign12720_body9_e14979);
        (assign12720_body9_e14980, (((-((locals.var_c0bulk * locals.var_t4__blk337_dn0) / (locals.var_t4__blk337 * locals.var_t4__blk337))) * assign12720_body9_e14979) + (assign12720_body9_e14975 * (-locals.var_t3__blk336_dn0))), (((-((locals.var_c0bulk * locals.var_t4__blk337_dn2) / (locals.var_t4__blk337 * locals.var_t4__blk337))) * assign12720_body9_e14979) + (assign12720_body9_e14975 * (-locals.var_t3__blk336_dn2))), (((-((locals.var_c0bulk * locals.var_t4__blk337_dn6) / (locals.var_t4__blk337 * locals.var_t4__blk337))) * assign12720_body9_e14979) + (assign12720_body9_e14975 * (-locals.var_t3__blk336_dn6))), (((-((locals.var_c0bulk * locals.var_t4__blk337_dn7) / (locals.var_t4__blk337 * locals.var_t4__blk337))) * assign12720_body9_e14979) + (assign12720_body9_e14975 * (-locals.var_t3__blk336_dn7))), (((-((locals.var_c0bulk * locals.var_t4__blk337_dn10) / (locals.var_t4__blk337 * locals.var_t4__blk337))) * assign12720_body9_e14979) + (assign12720_body9_e14975 * (-locals.var_t3__blk336_dn10))), (((-((locals.var_c0bulk * locals.var_t4__blk337_dn11) / (locals.var_t4__blk337 * locals.var_t4__blk337))) * assign12720_body9_e14979) + (assign12720_body9_e14975 * (-locals.var_t3__blk336_dn11))), (((-((locals.var_c0bulk * locals.var_t4__blk337_dn12) / (locals.var_t4__blk337 * locals.var_t4__blk337))) * assign12720_body9_e14979) + (assign12720_body9_e14975 * (-locals.var_t3__blk336_dn12))), (((-((locals.var_c0bulk * locals.var_t4__blk337_dn17) / (locals.var_t4__blk337 * locals.var_t4__blk337))) * assign12720_body9_e14979) + (assign12720_body9_e14975 * (-locals.var_t3__blk336_dn17))),)
    } else {
        (locals.var_t5__blk338, locals.var_t5__blk338_dn0, locals.var_t5__blk338_dn2, locals.var_t5__blk338_dn6, locals.var_t5__blk338_dn7, locals.var_t5__blk338_dn10, locals.var_t5__blk338_dn11, locals.var_t5__blk338_dn12, locals.var_t5__blk338_dn17,)
    }
};
            locals.var_t5__blk338 = assign12720_body9_e14982;
            locals.var_t5__blk338_dn0 = assign12720_body9_e14982_d_n0;
            locals.var_t5__blk338_dn2 = assign12720_body9_e14982_d_n2;
            locals.var_t5__blk338_dn6 = assign12720_body9_e14982_d_n6;
            locals.var_t5__blk338_dn7 = assign12720_body9_e14982_d_n7;
            locals.var_t5__blk338_dn10 = assign12720_body9_e14982_d_n10;
            locals.var_t5__blk338_dn11 = assign12720_body9_e14982_d_n11;
            locals.var_t5__blk338_dn12 = assign12720_body9_e14982_d_n12;
            locals.var_t5__blk338_dn17 = assign12720_body9_e14982_d_n17;
            let (assign12720_body10_e15011, assign12720_body10_e15011_d_n0, assign12720_body10_e15011_d_n2, assign12720_body10_e15011_d_n6, assign12720_body10_e15011_d_n7, assign12720_body10_e15011_d_n10, assign12720_body10_e15011_d_n11, assign12720_body10_e15011_d_n12, assign12720_body10_e15011_d_n17,) = {
    if (((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard344 == 0.0)) {
        let assign12720_body10_e15003: f64 = (locals.var_c0bulk / locals.var_beta);
        let assign12720_body10_e15004: f64 = (assign12720_body10_e15003).sqrt();
        let assign12720_body10_e15005: f64 = (-assign12720_body10_e15004);
        let assign12720_body10_e15007: f64 = (assign12720_body10_e15005 * locals.var_beta);
        let assign12720_body10_e15009: f64 = (assign12720_body10_e15007 * locals.var_phi_sl_bulk);
        (assign12720_body10_e15009, (assign12720_body10_e15007 * locals.var_phi_sl_bulk_dn0), (assign12720_body10_e15007 * locals.var_phi_sl_bulk_dn2), (assign12720_body10_e15007 * locals.var_phi_sl_bulk_dn6), (assign12720_body10_e15007 * locals.var_phi_sl_bulk_dn7), (((((-((-((locals.var_c0bulk * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) / (2.0 * assign12720_body10_e15004))) * locals.var_beta) + (assign12720_body10_e15005 * locals.var_beta_dn10)) * locals.var_phi_sl_bulk) + (assign12720_body10_e15007 * locals.var_phi_sl_bulk_dn10)), (assign12720_body10_e15007 * locals.var_phi_sl_bulk_dn11), (assign12720_body10_e15007 * locals.var_phi_sl_bulk_dn12), (assign12720_body10_e15007 * locals.var_phi_sl_bulk_dn17),)
    } else {
        (locals.var_t4__blk337, locals.var_t4__blk337_dn0, locals.var_t4__blk337_dn2, locals.var_t4__blk337_dn6, locals.var_t4__blk337_dn7, locals.var_t4__blk337_dn10, locals.var_t4__blk337_dn11, locals.var_t4__blk337_dn12, locals.var_t4__blk337_dn17,)
    }
};
            locals.var_t4__blk337 = assign12720_body10_e15011;
            locals.var_t4__blk337_dn0 = assign12720_body10_e15011_d_n0;
            locals.var_t4__blk337_dn2 = assign12720_body10_e15011_d_n2;
            locals.var_t4__blk337_dn6 = assign12720_body10_e15011_d_n6;
            locals.var_t4__blk337_dn7 = assign12720_body10_e15011_d_n7;
            locals.var_t4__blk337_dn10 = assign12720_body10_e15011_d_n10;
            locals.var_t4__blk337_dn11 = assign12720_body10_e15011_d_n11;
            locals.var_t4__blk337_dn12 = assign12720_body10_e15011_d_n12;
            locals.var_t4__blk337_dn17 = assign12720_body10_e15011_d_n17;
            let (assign12720_body11_e15036, assign12720_body11_e15036_d_n0, assign12720_body11_e15036_d_n2, assign12720_body11_e15036_d_n6, assign12720_body11_e15036_d_n7, assign12720_body11_e15036_d_n10, assign12720_body11_e15036_d_n11, assign12720_body11_e15036_d_n12, assign12720_body11_e15036_d_n17,) = {
    if (((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 != 0.0)) && (locals.var_guard343 == 0.0)) && (locals.var_guard344 == 0.0)) {
        let assign12720_body11_e15032: f64 = (locals.var_c0bulk * locals.var_beta);
        let assign12720_body11_e15033: f64 = (assign12720_body11_e15032).sqrt();
        let assign12720_body11_e15034: f64 = (-assign12720_body11_e15033);
        (assign12720_body11_e15034, 0.0, 0.0, 0.0, 0.0, (-((locals.var_c0bulk * locals.var_beta_dn10) / (2.0 * assign12720_body11_e15033))), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5__blk338, locals.var_t5__blk338_dn0, locals.var_t5__blk338_dn2, locals.var_t5__blk338_dn6, locals.var_t5__blk338_dn7, locals.var_t5__blk338_dn10, locals.var_t5__blk338_dn11, locals.var_t5__blk338_dn12, locals.var_t5__blk338_dn17,)
    }
};
            locals.var_t5__blk338 = assign12720_body11_e15036;
            locals.var_t5__blk338_dn0 = assign12720_body11_e15036_d_n0;
            locals.var_t5__blk338_dn2 = assign12720_body11_e15036_d_n2;
            locals.var_t5__blk338_dn6 = assign12720_body11_e15036_d_n6;
            locals.var_t5__blk338_dn7 = assign12720_body11_e15036_d_n7;
            locals.var_t5__blk338_dn10 = assign12720_body11_e15036_d_n10;
            locals.var_t5__blk338_dn11 = assign12720_body11_e15036_d_n11;
            locals.var_t5__blk338_dn12 = assign12720_body11_e15036_d_n12;
            locals.var_t5__blk338_dn17 = assign12720_body11_e15036_d_n17;
            let (assign12720_body12_e15060, assign12720_body12_e15060_d_n0, assign12720_body12_e15060_d_n2, assign12720_body12_e15060_d_n6, assign12720_body12_e15060_d_n7, assign12720_body12_e15060_d_n10, assign12720_body12_e15060_d_n11, assign12720_body12_e15060_d_n12, assign12720_body12_e15060_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 != 0.0)) {
        let assign12720_body12_e15051: f64 = (locals.var_t4__blk337 * locals.var_t4__blk337);
        let assign12720_body12_e15054: f64 = (4.0 * locals.var_q_fd_dlt1);
        let assign12720_body12_e15056: f64 = (assign12720_body12_e15054 * locals.var_q_fd_dlt1);
        let assign12720_body12_e15057: f64 = (assign12720_body12_e15051 + assign12720_body12_e15056);
        let assign12720_body12_e15058: f64 = (assign12720_body12_e15057).sqrt();
        (assign12720_body12_e15058, ((((locals.var_t4__blk337_dn0 * locals.var_t4__blk337) + (locals.var_t4__blk337 * locals.var_t4__blk337_dn0)) + (((4.0 * locals.var_q_fd_dlt1_dn0) * locals.var_q_fd_dlt1) + (assign12720_body12_e15054 * locals.var_q_fd_dlt1_dn0))) / (2.0 * assign12720_body12_e15058)), ((((locals.var_t4__blk337_dn2 * locals.var_t4__blk337) + (locals.var_t4__blk337 * locals.var_t4__blk337_dn2)) + (((4.0 * locals.var_q_fd_dlt1_dn2) * locals.var_q_fd_dlt1) + (assign12720_body12_e15054 * locals.var_q_fd_dlt1_dn2))) / (2.0 * assign12720_body12_e15058)), ((((locals.var_t4__blk337_dn6 * locals.var_t4__blk337) + (locals.var_t4__blk337 * locals.var_t4__blk337_dn6)) + (((4.0 * locals.var_q_fd_dlt1_dn6) * locals.var_q_fd_dlt1) + (assign12720_body12_e15054 * locals.var_q_fd_dlt1_dn6))) / (2.0 * assign12720_body12_e15058)), ((((locals.var_t4__blk337_dn7 * locals.var_t4__blk337) + (locals.var_t4__blk337 * locals.var_t4__blk337_dn7)) + (((4.0 * locals.var_q_fd_dlt1_dn7) * locals.var_q_fd_dlt1) + (assign12720_body12_e15054 * locals.var_q_fd_dlt1_dn7))) / (2.0 * assign12720_body12_e15058)), ((((locals.var_t4__blk337_dn10 * locals.var_t4__blk337) + (locals.var_t4__blk337 * locals.var_t4__blk337_dn10)) + (((4.0 * locals.var_q_fd_dlt1_dn10) * locals.var_q_fd_dlt1) + (assign12720_body12_e15054 * locals.var_q_fd_dlt1_dn10))) / (2.0 * assign12720_body12_e15058)), ((((locals.var_t4__blk337_dn11 * locals.var_t4__blk337) + (locals.var_t4__blk337 * locals.var_t4__blk337_dn11)) + (((4.0 * locals.var_q_fd_dlt1_dn11) * locals.var_q_fd_dlt1) + (assign12720_body12_e15054 * locals.var_q_fd_dlt1_dn11))) / (2.0 * assign12720_body12_e15058)), ((((locals.var_t4__blk337_dn12 * locals.var_t4__blk337) + (locals.var_t4__blk337 * locals.var_t4__blk337_dn12)) + (((4.0 * locals.var_q_fd_dlt1_dn12) * locals.var_q_fd_dlt1) + (assign12720_body12_e15054 * locals.var_q_fd_dlt1_dn12))) / (2.0 * assign12720_body12_e15058)), ((((locals.var_t4__blk337_dn17 * locals.var_t4__blk337) + (locals.var_t4__blk337 * locals.var_t4__blk337_dn17)) + (((4.0 * locals.var_q_fd_dlt1_dn17) * locals.var_q_fd_dlt1) + (assign12720_body12_e15054 * locals.var_q_fd_dlt1_dn17))) / (2.0 * assign12720_body12_e15058)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign12720_body12_e15060;
            locals.var_tmf2_dn0 = assign12720_body12_e15060_d_n0;
            locals.var_tmf2_dn2 = assign12720_body12_e15060_d_n2;
            locals.var_tmf2_dn6 = assign12720_body12_e15060_d_n6;
            locals.var_tmf2_dn7 = assign12720_body12_e15060_d_n7;
            locals.var_tmf2_dn10 = assign12720_body12_e15060_d_n10;
            locals.var_tmf2_dn11 = assign12720_body12_e15060_d_n11;
            locals.var_tmf2_dn12 = assign12720_body12_e15060_d_n12;
            locals.var_tmf2_dn17 = assign12720_body12_e15060_d_n17;
            let (assign12720_body13_e15081, assign12720_body13_e15081_d_n0, assign12720_body13_e15081_d_n2, assign12720_body13_e15081_d_n6, assign12720_body13_e15081_d_n7, assign12720_body13_e15081_d_n10, assign12720_body13_e15081_d_n11, assign12720_body13_e15081_d_n12, assign12720_body13_e15081_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 != 0.0)) {
        let assign12720_body13_e15077: f64 = (locals.var_t4__blk337 / locals.var_tmf2);
        let assign12720_body13_e15078: f64 = (1.0 + assign12720_body13_e15077);
        let assign12720_body13_e15079: f64 = (0.5 * assign12720_body13_e15078);
        (assign12720_body13_e15079, (0.5 * (((locals.var_t4__blk337_dn0 * locals.var_tmf2) - (locals.var_t4__blk337 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk337_dn2 * locals.var_tmf2) - (locals.var_t4__blk337 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk337_dn6 * locals.var_tmf2) - (locals.var_t4__blk337 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk337_dn7 * locals.var_tmf2) - (locals.var_t4__blk337 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk337_dn10 * locals.var_tmf2) - (locals.var_t4__blk337 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk337_dn11 * locals.var_tmf2) - (locals.var_t4__blk337 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk337_dn12 * locals.var_tmf2) - (locals.var_t4__blk337 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk337_dn17 * locals.var_tmf2) - (locals.var_t4__blk337 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t7__blk340, locals.var_t7__blk340_dn0, locals.var_t7__blk340_dn2, locals.var_t7__blk340_dn6, locals.var_t7__blk340_dn7, locals.var_t7__blk340_dn10, locals.var_t7__blk340_dn11, locals.var_t7__blk340_dn12, locals.var_t7__blk340_dn17,)
    }
};
            locals.var_t7__blk340 = assign12720_body13_e15081;
            locals.var_t7__blk340_dn0 = assign12720_body13_e15081_d_n0;
            locals.var_t7__blk340_dn2 = assign12720_body13_e15081_d_n2;
            locals.var_t7__blk340_dn6 = assign12720_body13_e15081_d_n6;
            locals.var_t7__blk340_dn7 = assign12720_body13_e15081_d_n7;
            locals.var_t7__blk340_dn10 = assign12720_body13_e15081_d_n10;
            locals.var_t7__blk340_dn11 = assign12720_body13_e15081_d_n11;
            locals.var_t7__blk340_dn12 = assign12720_body13_e15081_d_n12;
            locals.var_t7__blk340_dn17 = assign12720_body13_e15081_d_n17;
            let (assign12720_body14_e15104, assign12720_body14_e15104_d_n0, assign12720_body14_e15104_d_n2, assign12720_body14_e15104_d_n6, assign12720_body14_e15104_d_n7, assign12720_body14_e15104_d_n10, assign12720_body14_e15104_d_n11, assign12720_body14_e15104_d_n12, assign12720_body14_e15104_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 != 0.0)) {
        let assign12720_body14_e15097: f64 = (locals.var_t4__blk337 + locals.var_tmf2);
        let assign12720_body14_e15098: f64 = (0.5 * assign12720_body14_e15097);
        let assign12720_body14_e15101: f64 = (1e-10 * locals.var_q_fd_dlt1);
        let assign12720_body14_e15102: f64 = (assign12720_body14_e15098 + assign12720_body14_e15101);
        (assign12720_body14_e15102, ((0.5 * (locals.var_t4__blk337_dn0 + locals.var_tmf2_dn0)) + (1e-10 * locals.var_q_fd_dlt1_dn0)), ((0.5 * (locals.var_t4__blk337_dn2 + locals.var_tmf2_dn2)) + (1e-10 * locals.var_q_fd_dlt1_dn2)), ((0.5 * (locals.var_t4__blk337_dn6 + locals.var_tmf2_dn6)) + (1e-10 * locals.var_q_fd_dlt1_dn6)), ((0.5 * (locals.var_t4__blk337_dn7 + locals.var_tmf2_dn7)) + (1e-10 * locals.var_q_fd_dlt1_dn7)), ((0.5 * (locals.var_t4__blk337_dn10 + locals.var_tmf2_dn10)) + (1e-10 * locals.var_q_fd_dlt1_dn10)), ((0.5 * (locals.var_t4__blk337_dn11 + locals.var_tmf2_dn11)) + (1e-10 * locals.var_q_fd_dlt1_dn11)), ((0.5 * (locals.var_t4__blk337_dn12 + locals.var_tmf2_dn12)) + (1e-10 * locals.var_q_fd_dlt1_dn12)), ((0.5 * (locals.var_t4__blk337_dn17 + locals.var_tmf2_dn17)) + (1e-10 * locals.var_q_fd_dlt1_dn17)),)
    } else {
        (locals.var_t6__blk339, locals.var_t6__blk339_dn0, locals.var_t6__blk339_dn2, locals.var_t6__blk339_dn6, locals.var_t6__blk339_dn7, locals.var_t6__blk339_dn10, locals.var_t6__blk339_dn11, locals.var_t6__blk339_dn12, locals.var_t6__blk339_dn17,)
    }
};
            locals.var_t6__blk339 = assign12720_body14_e15104;
            locals.var_t6__blk339_dn0 = assign12720_body14_e15104_d_n0;
            locals.var_t6__blk339_dn2 = assign12720_body14_e15104_d_n2;
            locals.var_t6__blk339_dn6 = assign12720_body14_e15104_d_n6;
            locals.var_t6__blk339_dn7 = assign12720_body14_e15104_d_n7;
            locals.var_t6__blk339_dn10 = assign12720_body14_e15104_d_n10;
            locals.var_t6__blk339_dn11 = assign12720_body14_e15104_d_n11;
            locals.var_t6__blk339_dn12 = assign12720_body14_e15104_d_n12;
            locals.var_t6__blk339_dn17 = assign12720_body14_e15104_d_n17;
            let assign12720_body15_e15107: f64 = if locals.var_t6__blk339 < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard345 = assign12720_body15_e15107;
            let (assign12720_body16_e15124, assign12720_body16_e15124_d_n0, assign12720_body16_e15124_d_n2, assign12720_body16_e15124_d_n6, assign12720_body16_e15124_d_n7, assign12720_body16_e15124_d_n10, assign12720_body16_e15124_d_n11, assign12720_body16_e15124_d_n12, assign12720_body16_e15124_d_n17,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 != 0.0)) && (locals.var_guard345 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk339, locals.var_t6__blk339_dn0, locals.var_t6__blk339_dn2, locals.var_t6__blk339_dn6, locals.var_t6__blk339_dn7, locals.var_t6__blk339_dn10, locals.var_t6__blk339_dn11, locals.var_t6__blk339_dn12, locals.var_t6__blk339_dn17,)
    }
};
            locals.var_t6__blk339 = assign12720_body16_e15124;
            locals.var_t6__blk339_dn0 = assign12720_body16_e15124_d_n0;
            locals.var_t6__blk339_dn2 = assign12720_body16_e15124_d_n2;
            locals.var_t6__blk339_dn6 = assign12720_body16_e15124_d_n6;
            locals.var_t6__blk339_dn7 = assign12720_body16_e15124_d_n7;
            locals.var_t6__blk339_dn10 = assign12720_body16_e15124_d_n10;
            locals.var_t6__blk339_dn11 = assign12720_body16_e15124_d_n11;
            locals.var_t6__blk339_dn12 = assign12720_body16_e15124_d_n12;
            locals.var_t6__blk339_dn17 = assign12720_body16_e15124_d_n17;
            let (assign12720_body17_e15141, assign12720_body17_e15141_d_n0, assign12720_body17_e15141_d_n2, assign12720_body17_e15141_d_n6, assign12720_body17_e15141_d_n7, assign12720_body17_e15141_d_n10, assign12720_body17_e15141_d_n11, assign12720_body17_e15141_d_n12, assign12720_body17_e15141_d_n17,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 != 0.0)) && (locals.var_guard345 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7__blk340, locals.var_t7__blk340_dn0, locals.var_t7__blk340_dn2, locals.var_t7__blk340_dn6, locals.var_t7__blk340_dn7, locals.var_t7__blk340_dn10, locals.var_t7__blk340_dn11, locals.var_t7__blk340_dn12, locals.var_t7__blk340_dn17,)
    }
};
            locals.var_t7__blk340 = assign12720_body17_e15141;
            locals.var_t7__blk340_dn0 = assign12720_body17_e15141_d_n0;
            locals.var_t7__blk340_dn2 = assign12720_body17_e15141_d_n2;
            locals.var_t7__blk340_dn6 = assign12720_body17_e15141_d_n6;
            locals.var_t7__blk340_dn7 = assign12720_body17_e15141_d_n7;
            locals.var_t7__blk340_dn10 = assign12720_body17_e15141_d_n10;
            locals.var_t7__blk340_dn11 = assign12720_body17_e15141_d_n11;
            locals.var_t7__blk340_dn12 = assign12720_body17_e15141_d_n12;
            locals.var_t7__blk340_dn17 = assign12720_body17_e15141_d_n17;
            let (assign12720_body18_e15161, assign12720_body18_e15161_d_n0, assign12720_body18_e15161_d_n2, assign12720_body18_e15161_d_n6, assign12720_body18_e15161_d_n7, assign12720_body18_e15161_d_n10, assign12720_body18_e15161_d_n11, assign12720_body18_e15161_d_n12, assign12720_body18_e15161_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 != 0.0)) {
        let assign12720_body18_e15155: f64 = (-locals.var_q_fd_soi);
        let assign12720_body18_e15157: f64 = (assign12720_body18_e15155 - locals.var_t6__blk339);
        let assign12720_body18_e15159: f64 = (assign12720_body18_e15157 - locals.var_q_fd_dlt2);
        (assign12720_body18_e15159, (((-locals.var_q_fd_soi_dn0) - locals.var_t6__blk339_dn0) - locals.var_q_fd_dlt2_dn0), (((-locals.var_q_fd_soi_dn2) - locals.var_t6__blk339_dn2) - locals.var_q_fd_dlt2_dn2), (((-locals.var_q_fd_soi_dn6) - locals.var_t6__blk339_dn6) - locals.var_q_fd_dlt2_dn6), (((-locals.var_q_fd_soi_dn7) - locals.var_t6__blk339_dn7) - locals.var_q_fd_dlt2_dn7), (((-locals.var_q_fd_soi_dn10) - locals.var_t6__blk339_dn10) - locals.var_q_fd_dlt2_dn10), (((-locals.var_q_fd_soi_dn11) - locals.var_t6__blk339_dn11) - locals.var_q_fd_dlt2_dn11), (((-locals.var_q_fd_soi_dn12) - locals.var_t6__blk339_dn12) - locals.var_q_fd_dlt2_dn12), (((-locals.var_q_fd_soi_dn17) - locals.var_t6__blk339_dn17) - locals.var_q_fd_dlt2_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
            locals.var_tmf1 = assign12720_body18_e15161;
            locals.var_tmf1_dn0 = assign12720_body18_e15161_d_n0;
            locals.var_tmf1_dn2 = assign12720_body18_e15161_d_n2;
            locals.var_tmf1_dn6 = assign12720_body18_e15161_d_n6;
            locals.var_tmf1_dn7 = assign12720_body18_e15161_d_n7;
            locals.var_tmf1_dn10 = assign12720_body18_e15161_d_n10;
            locals.var_tmf1_dn11 = assign12720_body18_e15161_d_n11;
            locals.var_tmf1_dn12 = assign12720_body18_e15161_d_n12;
            locals.var_tmf1_dn17 = assign12720_body18_e15161_d_n17;
            let (assign12720_body19_e15181, assign12720_body19_e15181_d_n0, assign12720_body19_e15181_d_n2, assign12720_body19_e15181_d_n6, assign12720_body19_e15181_d_n7, assign12720_body19_e15181_d_n10, assign12720_body19_e15181_d_n11, assign12720_body19_e15181_d_n12, assign12720_body19_e15181_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 != 0.0)) {
        let assign12720_body19_e15176: f64 = (-locals.var_q_fd_soi);
        let assign12720_body19_e15177: f64 = (4.0 * assign12720_body19_e15176);
        let assign12720_body19_e15179: f64 = (assign12720_body19_e15177 * locals.var_q_fd_dlt2);
        (assign12720_body19_e15179, (((4.0 * (-locals.var_q_fd_soi_dn0)) * locals.var_q_fd_dlt2) + (assign12720_body19_e15177 * locals.var_q_fd_dlt2_dn0)), (((4.0 * (-locals.var_q_fd_soi_dn2)) * locals.var_q_fd_dlt2) + (assign12720_body19_e15177 * locals.var_q_fd_dlt2_dn2)), (((4.0 * (-locals.var_q_fd_soi_dn6)) * locals.var_q_fd_dlt2) + (assign12720_body19_e15177 * locals.var_q_fd_dlt2_dn6)), (((4.0 * (-locals.var_q_fd_soi_dn7)) * locals.var_q_fd_dlt2) + (assign12720_body19_e15177 * locals.var_q_fd_dlt2_dn7)), (((4.0 * (-locals.var_q_fd_soi_dn10)) * locals.var_q_fd_dlt2) + (assign12720_body19_e15177 * locals.var_q_fd_dlt2_dn10)), (((4.0 * (-locals.var_q_fd_soi_dn11)) * locals.var_q_fd_dlt2) + (assign12720_body19_e15177 * locals.var_q_fd_dlt2_dn11)), (((4.0 * (-locals.var_q_fd_soi_dn12)) * locals.var_q_fd_dlt2) + (assign12720_body19_e15177 * locals.var_q_fd_dlt2_dn12)), (((4.0 * (-locals.var_q_fd_soi_dn17)) * locals.var_q_fd_dlt2) + (assign12720_body19_e15177 * locals.var_q_fd_dlt2_dn17)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign12720_body19_e15181;
            locals.var_tmf2_dn0 = assign12720_body19_e15181_d_n0;
            locals.var_tmf2_dn2 = assign12720_body19_e15181_d_n2;
            locals.var_tmf2_dn6 = assign12720_body19_e15181_d_n6;
            locals.var_tmf2_dn7 = assign12720_body19_e15181_d_n7;
            locals.var_tmf2_dn10 = assign12720_body19_e15181_d_n10;
            locals.var_tmf2_dn11 = assign12720_body19_e15181_d_n11;
            locals.var_tmf2_dn12 = assign12720_body19_e15181_d_n12;
            locals.var_tmf2_dn17 = assign12720_body19_e15181_d_n17;
            let (assign12720_body20_e15202, assign12720_body20_e15202_d_n0, assign12720_body20_e15202_d_n2, assign12720_body20_e15202_d_n6, assign12720_body20_e15202_d_n7, assign12720_body20_e15202_d_n10, assign12720_body20_e15202_d_n11, assign12720_body20_e15202_d_n12, assign12720_body20_e15202_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 != 0.0)) {
        let (assign12720_body20_e15200, assign12720_body20_e15200_d_n0, assign12720_body20_e15200_d_n2, assign12720_body20_e15200_d_n6, assign12720_body20_e15200_d_n7, assign12720_body20_e15200_d_n10, assign12720_body20_e15200_d_n11, assign12720_body20_e15200_d_n12, assign12720_body20_e15200_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign12720_body20_e15199: f64 = (-locals.var_tmf2);
                (assign12720_body20_e15199, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign12720_body20_e15200, assign12720_body20_e15200_d_n0, assign12720_body20_e15200_d_n2, assign12720_body20_e15200_d_n6, assign12720_body20_e15200_d_n7, assign12720_body20_e15200_d_n10, assign12720_body20_e15200_d_n11, assign12720_body20_e15200_d_n12, assign12720_body20_e15200_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign12720_body20_e15202;
            locals.var_tmf2_dn0 = assign12720_body20_e15202_d_n0;
            locals.var_tmf2_dn2 = assign12720_body20_e15202_d_n2;
            locals.var_tmf2_dn6 = assign12720_body20_e15202_d_n6;
            locals.var_tmf2_dn7 = assign12720_body20_e15202_d_n7;
            locals.var_tmf2_dn10 = assign12720_body20_e15202_d_n10;
            locals.var_tmf2_dn11 = assign12720_body20_e15202_d_n11;
            locals.var_tmf2_dn12 = assign12720_body20_e15202_d_n12;
            locals.var_tmf2_dn17 = assign12720_body20_e15202_d_n17;
            let (assign12720_body21_e15222, assign12720_body21_e15222_d_n0, assign12720_body21_e15222_d_n2, assign12720_body21_e15222_d_n6, assign12720_body21_e15222_d_n7, assign12720_body21_e15222_d_n10, assign12720_body21_e15222_d_n11, assign12720_body21_e15222_d_n12, assign12720_body21_e15222_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 != 0.0)) {
        let assign12720_body21_e15217: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign12720_body21_e15219: f64 = (assign12720_body21_e15217 + locals.var_tmf2);
        let assign12720_body21_e15220: f64 = (assign12720_body21_e15219).sqrt();
        (assign12720_body21_e15220, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign12720_body21_e15220)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign12720_body21_e15220)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign12720_body21_e15220)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign12720_body21_e15220)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign12720_body21_e15220)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign12720_body21_e15220)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign12720_body21_e15220)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign12720_body21_e15220)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign12720_body21_e15222;
            locals.var_tmf2_dn0 = assign12720_body21_e15222_d_n0;
            locals.var_tmf2_dn2 = assign12720_body21_e15222_d_n2;
            locals.var_tmf2_dn6 = assign12720_body21_e15222_d_n6;
            locals.var_tmf2_dn7 = assign12720_body21_e15222_d_n7;
            locals.var_tmf2_dn10 = assign12720_body21_e15222_d_n10;
            locals.var_tmf2_dn11 = assign12720_body21_e15222_d_n11;
            locals.var_tmf2_dn12 = assign12720_body21_e15222_d_n12;
            locals.var_tmf2_dn17 = assign12720_body21_e15222_d_n17;
            let (assign12720_body22_e15243, assign12720_body22_e15243_d_n0, assign12720_body22_e15243_d_n2, assign12720_body22_e15243_d_n6, assign12720_body22_e15243_d_n7, assign12720_body22_e15243_d_n10, assign12720_body22_e15243_d_n11, assign12720_body22_e15243_d_n12, assign12720_body22_e15243_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 != 0.0)) {
        let assign12720_body22_e15239: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign12720_body22_e15240: f64 = (1.0 + assign12720_body22_e15239);
        let assign12720_body22_e15241: f64 = (0.5 * assign12720_body22_e15240);
        (assign12720_body22_e15241, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn17,)
    }
};
            locals.var_t8 = assign12720_body22_e15243;
            locals.var_t8_dn0 = assign12720_body22_e15243_d_n0;
            locals.var_t8_dn2 = assign12720_body22_e15243_d_n2;
            locals.var_t8_dn6 = assign12720_body22_e15243_d_n6;
            locals.var_t8_dn7 = assign12720_body22_e15243_d_n7;
            locals.var_t8_dn10 = assign12720_body22_e15243_d_n10;
            locals.var_t8_dn11 = assign12720_body22_e15243_d_n11;
            locals.var_t8_dn12 = assign12720_body22_e15243_d_n12;
            locals.var_t8_dn17 = assign12720_body22_e15243_d_n17;
            let (assign12720_body23_e15265, assign12720_body23_e15265_d_n0, assign12720_body23_e15265_d_n2, assign12720_body23_e15265_d_n6, assign12720_body23_e15265_d_n7, assign12720_body23_e15265_d_n10, assign12720_body23_e15265_d_n11, assign12720_body23_e15265_d_n12, assign12720_body23_e15265_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 != 0.0)) {
        let assign12720_body23_e15257: f64 = (-locals.var_q_fd_soi);
        let assign12720_body23_e15261: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign12720_body23_e15262: f64 = (0.5 * assign12720_body23_e15261);
        let assign12720_body23_e15263: f64 = (assign12720_body23_e15257 - assign12720_body23_e15262);
        (assign12720_body23_e15263, ((-locals.var_q_fd_soi_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((-locals.var_q_fd_soi_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((-locals.var_q_fd_soi_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((-locals.var_q_fd_soi_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((-locals.var_q_fd_soi_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((-locals.var_q_fd_soi_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((-locals.var_q_fd_soi_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((-locals.var_q_fd_soi_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_t6__blk339, locals.var_t6__blk339_dn0, locals.var_t6__blk339_dn2, locals.var_t6__blk339_dn6, locals.var_t6__blk339_dn7, locals.var_t6__blk339_dn10, locals.var_t6__blk339_dn11, locals.var_t6__blk339_dn12, locals.var_t6__blk339_dn17,)
    }
};
            locals.var_t6__blk339 = assign12720_body23_e15265;
            locals.var_t6__blk339_dn0 = assign12720_body23_e15265_d_n0;
            locals.var_t6__blk339_dn2 = assign12720_body23_e15265_d_n2;
            locals.var_t6__blk339_dn6 = assign12720_body23_e15265_d_n6;
            locals.var_t6__blk339_dn7 = assign12720_body23_e15265_d_n7;
            locals.var_t6__blk339_dn10 = assign12720_body23_e15265_d_n10;
            locals.var_t6__blk339_dn11 = assign12720_body23_e15265_d_n11;
            locals.var_t6__blk339_dn12 = assign12720_body23_e15265_d_n12;
            locals.var_t6__blk339_dn17 = assign12720_body23_e15265_d_n17;
            let (assign12720_body24_e15284, assign12720_body24_e15284_d_n0, assign12720_body24_e15284_d_n2, assign12720_body24_e15284_d_n6, assign12720_body24_e15284_d_n7, assign12720_body24_e15284_d_n10, assign12720_body24_e15284_d_n11, assign12720_body24_e15284_d_n12, assign12720_body24_e15284_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 != 0.0)) {
        let assign12720_body24_e15281: f64 = (locals.var_t5__blk338 * locals.var_t8);
        let assign12720_body24_e15282: f64 = (locals.var_t7__blk340 * assign12720_body24_e15281);
        (assign12720_body24_e15282, ((locals.var_t7__blk340_dn0 * assign12720_body24_e15281) + (locals.var_t7__blk340 * ((locals.var_t5__blk338_dn0 * locals.var_t8) + (locals.var_t5__blk338 * locals.var_t8_dn0)))), ((locals.var_t7__blk340_dn2 * assign12720_body24_e15281) + (locals.var_t7__blk340 * ((locals.var_t5__blk338_dn2 * locals.var_t8) + (locals.var_t5__blk338 * locals.var_t8_dn2)))), ((locals.var_t7__blk340_dn6 * assign12720_body24_e15281) + (locals.var_t7__blk340 * ((locals.var_t5__blk338_dn6 * locals.var_t8) + (locals.var_t5__blk338 * locals.var_t8_dn6)))), ((locals.var_t7__blk340_dn7 * assign12720_body24_e15281) + (locals.var_t7__blk340 * ((locals.var_t5__blk338_dn7 * locals.var_t8) + (locals.var_t5__blk338 * locals.var_t8_dn7)))), ((locals.var_t7__blk340_dn10 * assign12720_body24_e15281) + (locals.var_t7__blk340 * ((locals.var_t5__blk338_dn10 * locals.var_t8) + (locals.var_t5__blk338 * locals.var_t8_dn10)))), ((locals.var_t7__blk340_dn11 * assign12720_body24_e15281) + (locals.var_t7__blk340 * ((locals.var_t5__blk338_dn11 * locals.var_t8) + (locals.var_t5__blk338 * locals.var_t8_dn11)))), ((locals.var_t7__blk340_dn12 * assign12720_body24_e15281) + (locals.var_t7__blk340 * ((locals.var_t5__blk338_dn12 * locals.var_t8) + (locals.var_t5__blk338 * locals.var_t8_dn12)))), ((locals.var_t7__blk340_dn17 * assign12720_body24_e15281) + (locals.var_t7__blk340 * ((locals.var_t5__blk338_dn17 * locals.var_t8) + (locals.var_t5__blk338 * locals.var_t8_dn17)))),)
    } else {
        (locals.var_t7__blk340, locals.var_t7__blk340_dn0, locals.var_t7__blk340_dn2, locals.var_t7__blk340_dn6, locals.var_t7__blk340_dn7, locals.var_t7__blk340_dn10, locals.var_t7__blk340_dn11, locals.var_t7__blk340_dn12, locals.var_t7__blk340_dn17,)
    }
};
            locals.var_t7__blk340 = assign12720_body24_e15284;
            locals.var_t7__blk340_dn0 = assign12720_body24_e15284_d_n0;
            locals.var_t7__blk340_dn2 = assign12720_body24_e15284_d_n2;
            locals.var_t7__blk340_dn6 = assign12720_body24_e15284_d_n6;
            locals.var_t7__blk340_dn7 = assign12720_body24_e15284_d_n7;
            locals.var_t7__blk340_dn10 = assign12720_body24_e15284_d_n10;
            locals.var_t7__blk340_dn11 = assign12720_body24_e15284_d_n11;
            locals.var_t7__blk340_dn12 = assign12720_body24_e15284_d_n12;
            locals.var_t7__blk340_dn17 = assign12720_body24_e15284_d_n17;
            let (assign12720_body25_e15309, assign12720_body25_e15309_d_n0, assign12720_body25_e15309_d_n2, assign12720_body25_e15309_d_n6, assign12720_body25_e15309_d_n7, assign12720_body25_e15309_d_n10, assign12720_body25_e15309_d_n11, assign12720_body25_e15309_d_n12, assign12720_body25_e15309_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 != 0.0)) {
        let assign12720_body25_e15299: f64 = (locals.var_t6__blk339 * locals.var_t6__blk339);
        let assign12720_body25_e15301: f64 = (assign12720_body25_e15299 / 2.0);
        let assign12720_body25_e15303: f64 = (assign12720_body25_e15301 / 1.034943e-10);
        let assign12720_body25_e15305: f64 = (assign12720_body25_e15303 / 1.6021918e-19);
        let assign12720_body25_e15307: f64 = (assign12720_body25_e15305 / locals.var_uc_nsubs);
        (assign12720_body25_e15307, ((((((((locals.var_t6__blk339_dn0 * locals.var_t6__blk339) + (locals.var_t6__blk339 * locals.var_t6__blk339_dn0)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12720_body25_e15305 * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk339_dn2 * locals.var_t6__blk339) + (locals.var_t6__blk339 * locals.var_t6__blk339_dn2)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12720_body25_e15305 * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk339_dn6 * locals.var_t6__blk339) + (locals.var_t6__blk339 * locals.var_t6__blk339_dn6)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12720_body25_e15305 * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk339_dn7 * locals.var_t6__blk339) + (locals.var_t6__blk339 * locals.var_t6__blk339_dn7)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12720_body25_e15305 * locals.var_uc_nsubs_dn7)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk339_dn10 * locals.var_t6__blk339) + (locals.var_t6__blk339 * locals.var_t6__blk339_dn10)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12720_body25_e15305 * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk339_dn11 * locals.var_t6__blk339) + (locals.var_t6__blk339 * locals.var_t6__blk339_dn11)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12720_body25_e15305 * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk339_dn12 * locals.var_t6__blk339) + (locals.var_t6__blk339 * locals.var_t6__blk339_dn12)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12720_body25_e15305 * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk339_dn17 * locals.var_t6__blk339) + (locals.var_t6__blk339 * locals.var_t6__blk339_dn17)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12720_body25_e15305 * locals.var_uc_nsubs_dn17)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)),)
    } else {
        (locals.var_phi_b_dep, locals.var_phi_b_dep_dn0, locals.var_phi_b_dep_dn2, locals.var_phi_b_dep_dn6, locals.var_phi_b_dep_dn7, locals.var_phi_b_dep_dn10, locals.var_phi_b_dep_dn11, locals.var_phi_b_dep_dn12, locals.var_phi_b_dep_dn17,)
    }
};
            locals.var_phi_b_dep = assign12720_body25_e15309;
            locals.var_phi_b_dep_dn0 = assign12720_body25_e15309_d_n0;
            locals.var_phi_b_dep_dn2 = assign12720_body25_e15309_d_n2;
            locals.var_phi_b_dep_dn6 = assign12720_body25_e15309_d_n6;
            locals.var_phi_b_dep_dn7 = assign12720_body25_e15309_d_n7;
            locals.var_phi_b_dep_dn10 = assign12720_body25_e15309_d_n10;
            locals.var_phi_b_dep_dn11 = assign12720_body25_e15309_d_n11;
            locals.var_phi_b_dep_dn12 = assign12720_body25_e15309_d_n12;
            locals.var_phi_b_dep_dn17 = assign12720_body25_e15309_d_n17;
            let (assign12720_body26_e15330, assign12720_body26_e15330_d_n0, assign12720_body26_e15330_d_n2, assign12720_body26_e15330_d_n6, assign12720_body26_e15330_d_n7, assign12720_body26_e15330_d_n10, assign12720_body26_e15330_d_n11, assign12720_body26_e15330_d_n12, assign12720_body26_e15330_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 != 0.0)) {
        let assign12720_body26_e15324: f64 = (2.0 * locals.var_phi_b_dep);
        let assign12720_body26_e15326: f64 = (assign12720_body26_e15324 * locals.var_t7__blk340);
        let assign12720_body26_e15328: f64 = (assign12720_body26_e15326 / locals.var_t6__blk339);
        (assign12720_body26_e15328, ((((((2.0 * locals.var_phi_b_dep_dn0) * locals.var_t7__blk340) + (assign12720_body26_e15324 * locals.var_t7__blk340_dn0)) * locals.var_t6__blk339) - (assign12720_body26_e15326 * locals.var_t6__blk339_dn0)) / (locals.var_t6__blk339 * locals.var_t6__blk339)), ((((((2.0 * locals.var_phi_b_dep_dn2) * locals.var_t7__blk340) + (assign12720_body26_e15324 * locals.var_t7__blk340_dn2)) * locals.var_t6__blk339) - (assign12720_body26_e15326 * locals.var_t6__blk339_dn2)) / (locals.var_t6__blk339 * locals.var_t6__blk339)), ((((((2.0 * locals.var_phi_b_dep_dn6) * locals.var_t7__blk340) + (assign12720_body26_e15324 * locals.var_t7__blk340_dn6)) * locals.var_t6__blk339) - (assign12720_body26_e15326 * locals.var_t6__blk339_dn6)) / (locals.var_t6__blk339 * locals.var_t6__blk339)), ((((((2.0 * locals.var_phi_b_dep_dn7) * locals.var_t7__blk340) + (assign12720_body26_e15324 * locals.var_t7__blk340_dn7)) * locals.var_t6__blk339) - (assign12720_body26_e15326 * locals.var_t6__blk339_dn7)) / (locals.var_t6__blk339 * locals.var_t6__blk339)), ((((((2.0 * locals.var_phi_b_dep_dn10) * locals.var_t7__blk340) + (assign12720_body26_e15324 * locals.var_t7__blk340_dn10)) * locals.var_t6__blk339) - (assign12720_body26_e15326 * locals.var_t6__blk339_dn10)) / (locals.var_t6__blk339 * locals.var_t6__blk339)), ((((((2.0 * locals.var_phi_b_dep_dn11) * locals.var_t7__blk340) + (assign12720_body26_e15324 * locals.var_t7__blk340_dn11)) * locals.var_t6__blk339) - (assign12720_body26_e15326 * locals.var_t6__blk339_dn11)) / (locals.var_t6__blk339 * locals.var_t6__blk339)), ((((((2.0 * locals.var_phi_b_dep_dn12) * locals.var_t7__blk340) + (assign12720_body26_e15324 * locals.var_t7__blk340_dn12)) * locals.var_t6__blk339) - (assign12720_body26_e15326 * locals.var_t6__blk339_dn12)) / (locals.var_t6__blk339 * locals.var_t6__blk339)), ((((((2.0 * locals.var_phi_b_dep_dn17) * locals.var_t7__blk340) + (assign12720_body26_e15324 * locals.var_t7__blk340_dn17)) * locals.var_t6__blk339) - (assign12720_body26_e15326 * locals.var_t6__blk339_dn17)) / (locals.var_t6__blk339 * locals.var_t6__blk339)),)
    } else {
        (locals.var_phi_b_dep_dpsb, locals.var_phi_b_dep_dpsb_dn0, locals.var_phi_b_dep_dpsb_dn2, locals.var_phi_b_dep_dpsb_dn6, locals.var_phi_b_dep_dpsb_dn7, locals.var_phi_b_dep_dpsb_dn10, locals.var_phi_b_dep_dpsb_dn11, locals.var_phi_b_dep_dpsb_dn12, locals.var_phi_b_dep_dpsb_dn17,)
    }
};
            locals.var_phi_b_dep_dpsb = assign12720_body26_e15330;
            locals.var_phi_b_dep_dpsb_dn0 = assign12720_body26_e15330_d_n0;
            locals.var_phi_b_dep_dpsb_dn2 = assign12720_body26_e15330_d_n2;
            locals.var_phi_b_dep_dpsb_dn6 = assign12720_body26_e15330_d_n6;
            locals.var_phi_b_dep_dpsb_dn7 = assign12720_body26_e15330_d_n7;
            locals.var_phi_b_dep_dpsb_dn10 = assign12720_body26_e15330_d_n10;
            locals.var_phi_b_dep_dpsb_dn11 = assign12720_body26_e15330_d_n11;
            locals.var_phi_b_dep_dpsb_dn12 = assign12720_body26_e15330_d_n12;
            locals.var_phi_b_dep_dpsb_dn17 = assign12720_body26_e15330_d_n17;
            let (assign12720_body27_e15365, assign12720_body27_e15365_d_n0, assign12720_body27_e15365_d_n2, assign12720_body27_e15365_d_n6, assign12720_body27_e15365_d_n7, assign12720_body27_e15365_d_n10, assign12720_body27_e15365_d_n11, assign12720_body27_e15365_d_n12, assign12720_body27_e15365_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 != 0.0)) {
        let assign12720_body27_e15345: f64 = (-locals.var_phi_sl_bulk);
        let assign12720_body27_e15348: f64 = (locals.var_t4__blk337 / locals.var_c_box);
        let assign12720_body27_e15349: f64 = (assign12720_body27_e15345 + assign12720_body27_e15348);
        let assign12720_body27_e15351: f64 = (assign12720_body27_e15349 - locals.var_vbsbiz);
        let assign12720_body27_e15353: f64 = (assign12720_body27_e15351 + locals.var_phi_b_dep);
        let assign12720_body27_e15355: f64 = (-1.0);
        let assign12720_body27_e15358: f64 = (locals.var_t5__blk338 / locals.var_c_box);
        let assign12720_body27_e15359: f64 = (assign12720_body27_e15355 + assign12720_body27_e15358);
        let assign12720_body27_e15361: f64 = (assign12720_body27_e15359 + locals.var_phi_b_dep_dpsb);
        let assign12720_body27_e15362: f64 = (assign12720_body27_e15353 / assign12720_body27_e15361);
        let assign12720_body27_e15363: f64 = (locals.var_phi_sl_bulk - assign12720_body27_e15362);
        (assign12720_body27_e15363, (locals.var_phi_sl_bulk_dn0 - (((((((-locals.var_phi_sl_bulk_dn0) + (locals.var_t4__blk337_dn0 / locals.var_c_box)) - locals.var_vbsbiz_dn0) + locals.var_phi_b_dep_dn0) * assign12720_body27_e15361) - (assign12720_body27_e15353 * ((locals.var_t5__blk338_dn0 / locals.var_c_box) + locals.var_phi_b_dep_dpsb_dn0))) / (assign12720_body27_e15361 * assign12720_body27_e15361))), (locals.var_phi_sl_bulk_dn2 - (((((((-locals.var_phi_sl_bulk_dn2) + (locals.var_t4__blk337_dn2 / locals.var_c_box)) - locals.var_vbsbiz_dn2) + locals.var_phi_b_dep_dn2) * assign12720_body27_e15361) - (assign12720_body27_e15353 * ((locals.var_t5__blk338_dn2 / locals.var_c_box) + locals.var_phi_b_dep_dpsb_dn2))) / (assign12720_body27_e15361 * assign12720_body27_e15361))), (locals.var_phi_sl_bulk_dn6 - (((((((-locals.var_phi_sl_bulk_dn6) + (locals.var_t4__blk337_dn6 / locals.var_c_box)) - locals.var_vbsbiz_dn6) + locals.var_phi_b_dep_dn6) * assign12720_body27_e15361) - (assign12720_body27_e15353 * ((locals.var_t5__blk338_dn6 / locals.var_c_box) + locals.var_phi_b_dep_dpsb_dn6))) / (assign12720_body27_e15361 * assign12720_body27_e15361))), (locals.var_phi_sl_bulk_dn7 - (((((((-locals.var_phi_sl_bulk_dn7) + (locals.var_t4__blk337_dn7 / locals.var_c_box)) - locals.var_vbsbiz_dn7) + locals.var_phi_b_dep_dn7) * assign12720_body27_e15361) - (assign12720_body27_e15353 * ((locals.var_t5__blk338_dn7 / locals.var_c_box) + locals.var_phi_b_dep_dpsb_dn7))) / (assign12720_body27_e15361 * assign12720_body27_e15361))), (locals.var_phi_sl_bulk_dn10 - (((((((-locals.var_phi_sl_bulk_dn10) + (locals.var_t4__blk337_dn10 / locals.var_c_box)) - locals.var_vbsbiz_dn10) + locals.var_phi_b_dep_dn10) * assign12720_body27_e15361) - (assign12720_body27_e15353 * ((locals.var_t5__blk338_dn10 / locals.var_c_box) + locals.var_phi_b_dep_dpsb_dn10))) / (assign12720_body27_e15361 * assign12720_body27_e15361))), (locals.var_phi_sl_bulk_dn11 - (((((((-locals.var_phi_sl_bulk_dn11) + (locals.var_t4__blk337_dn11 / locals.var_c_box)) - locals.var_vbsbiz_dn11) + locals.var_phi_b_dep_dn11) * assign12720_body27_e15361) - (assign12720_body27_e15353 * ((locals.var_t5__blk338_dn11 / locals.var_c_box) + locals.var_phi_b_dep_dpsb_dn11))) / (assign12720_body27_e15361 * assign12720_body27_e15361))), (locals.var_phi_sl_bulk_dn12 - (((((((-locals.var_phi_sl_bulk_dn12) + (locals.var_t4__blk337_dn12 / locals.var_c_box)) - locals.var_vbsbiz_dn12) + locals.var_phi_b_dep_dn12) * assign12720_body27_e15361) - (assign12720_body27_e15353 * ((locals.var_t5__blk338_dn12 / locals.var_c_box) + locals.var_phi_b_dep_dpsb_dn12))) / (assign12720_body27_e15361 * assign12720_body27_e15361))), (locals.var_phi_sl_bulk_dn17 - (((((((-locals.var_phi_sl_bulk_dn17) + (locals.var_t4__blk337_dn17 / locals.var_c_box)) - locals.var_vbsbiz_dn17) + locals.var_phi_b_dep_dn17) * assign12720_body27_e15361) - (assign12720_body27_e15353 * ((locals.var_t5__blk338_dn17 / locals.var_c_box) + locals.var_phi_b_dep_dpsb_dn17))) / (assign12720_body27_e15361 * assign12720_body27_e15361))),)
    } else {
        (locals.var_t6__blk339, locals.var_t6__blk339_dn0, locals.var_t6__blk339_dn2, locals.var_t6__blk339_dn6, locals.var_t6__blk339_dn7, locals.var_t6__blk339_dn10, locals.var_t6__blk339_dn11, locals.var_t6__blk339_dn12, locals.var_t6__blk339_dn17,)
    }
};
            locals.var_t6__blk339 = assign12720_body27_e15365;
            locals.var_t6__blk339_dn0 = assign12720_body27_e15365_d_n0;
            locals.var_t6__blk339_dn2 = assign12720_body27_e15365_d_n2;
            locals.var_t6__blk339_dn6 = assign12720_body27_e15365_d_n6;
            locals.var_t6__blk339_dn7 = assign12720_body27_e15365_d_n7;
            locals.var_t6__blk339_dn10 = assign12720_body27_e15365_d_n10;
            locals.var_t6__blk339_dn11 = assign12720_body27_e15365_d_n11;
            locals.var_t6__blk339_dn12 = assign12720_body27_e15365_d_n12;
            locals.var_t6__blk339_dn17 = assign12720_body27_e15365_d_n17;
            let assign12720_body28_e15368: f64 = (locals.var_t6__blk339 - locals.var_phi_sl_bulk);
            let assign12720_body28_e15369: f64 = (assign12720_body28_e15368).abs();
            let assign12720_body28_e15371: f64 = if assign12720_body28_e15369 < 5e-12 { 1.0 } else { 0.0 };
            locals.var_guard346 = assign12720_body28_e15371;
            let (assign12720_body29_e15388,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 != 0.0)) && (locals.var_guard346 != 0.0)) {
        (locals.var_lp_sl_max,)
    } else {
        (locals.var_lp_sl,)
    }
};
            locals.var_lp_sl = assign12720_body29_e15388;
            let (assign12720_body30_e15403, assign12720_body30_e15403_d_n0, assign12720_body30_e15403_d_n2, assign12720_body30_e15403_d_n6, assign12720_body30_e15403_d_n7, assign12720_body30_e15403_d_n10, assign12720_body30_e15403_d_n11, assign12720_body30_e15403_d_n12, assign12720_body30_e15403_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 != 0.0)) {
        (locals.var_t6__blk339, locals.var_t6__blk339_dn0, locals.var_t6__blk339_dn2, locals.var_t6__blk339_dn6, locals.var_t6__blk339_dn7, locals.var_t6__blk339_dn10, locals.var_t6__blk339_dn11, locals.var_t6__blk339_dn12, locals.var_t6__blk339_dn17,)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
            locals.var_phi_sl_bulk = assign12720_body30_e15403;
            locals.var_phi_sl_bulk_dn0 = assign12720_body30_e15403_d_n0;
            locals.var_phi_sl_bulk_dn2 = assign12720_body30_e15403_d_n2;
            locals.var_phi_sl_bulk_dn6 = assign12720_body30_e15403_d_n6;
            locals.var_phi_sl_bulk_dn7 = assign12720_body30_e15403_d_n7;
            locals.var_phi_sl_bulk_dn10 = assign12720_body30_e15403_d_n10;
            locals.var_phi_sl_bulk_dn11 = assign12720_body30_e15403_d_n11;
            locals.var_phi_sl_bulk_dn12 = assign12720_body30_e15403_d_n12;
            locals.var_phi_sl_bulk_dn17 = assign12720_body30_e15403_d_n17;
            let (assign12720_body31_e15418, assign12720_body31_e15418_d_n0, assign12720_body31_e15418_d_n2, assign12720_body31_e15418_d_n6, assign12720_body31_e15418_d_n7, assign12720_body31_e15418_d_n10, assign12720_body31_e15418_d_n11, assign12720_body31_e15418_d_n12, assign12720_body31_e15418_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 != 0.0)) {
        (locals.var_t4__blk337, locals.var_t4__blk337_dn0, locals.var_t4__blk337_dn2, locals.var_t4__blk337_dn6, locals.var_t4__blk337_dn7, locals.var_t4__blk337_dn10, locals.var_t4__blk337_dn11, locals.var_t4__blk337_dn12, locals.var_t4__blk337_dn17,)
    } else {
        (locals.var_q_sl_bulk, locals.var_q_sl_bulk_dn0, locals.var_q_sl_bulk_dn2, locals.var_q_sl_bulk_dn6, locals.var_q_sl_bulk_dn7, locals.var_q_sl_bulk_dn10, locals.var_q_sl_bulk_dn11, locals.var_q_sl_bulk_dn12, locals.var_q_sl_bulk_dn17,)
    }
};
            locals.var_q_sl_bulk = assign12720_body31_e15418;
            locals.var_q_sl_bulk_dn0 = assign12720_body31_e15418_d_n0;
            locals.var_q_sl_bulk_dn2 = assign12720_body31_e15418_d_n2;
            locals.var_q_sl_bulk_dn6 = assign12720_body31_e15418_d_n6;
            locals.var_q_sl_bulk_dn7 = assign12720_body31_e15418_d_n7;
            locals.var_q_sl_bulk_dn10 = assign12720_body31_e15418_d_n10;
            locals.var_q_sl_bulk_dn11 = assign12720_body31_e15418_d_n11;
            locals.var_q_sl_bulk_dn12 = assign12720_body31_e15418_d_n12;
            locals.var_q_sl_bulk_dn17 = assign12720_body31_e15418_d_n17;
            let (assign12720_body32_e15435,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 != 0.0)) {
        let assign12720_body32_e15433: f64 = (locals.var_lp_sl + 1.0);
        (assign12720_body32_e15433,)
    } else {
        (locals.var_lp_sl,)
    }
};
            locals.var_lp_sl = assign12720_body32_e15435;
        }

    }

    pub(super) fn stamp_transient_block_38(
        locals: &mut StampLocals,
    ) {
        let (assign12730_e15452, assign12730_e15452_d_n0, assign12730_e15452_d_n2, assign12730_e15452_d_n6, assign12730_e15452_d_n7, assign12730_e15452_d_n10, assign12730_e15452_d_n11, assign12730_e15452_d_n12, assign12730_e15452_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 != 0.0)) {
        let assign12730_e15450: f64 = (locals.var_vbsbiz + locals.var_phi_sl_bulk);
        (assign12730_e15450, (locals.var_vbsbiz_dn0 + locals.var_phi_sl_bulk_dn0), (locals.var_vbsbiz_dn2 + locals.var_phi_sl_bulk_dn2), (locals.var_vbsbiz_dn6 + locals.var_phi_sl_bulk_dn6), (locals.var_vbsbiz_dn7 + locals.var_phi_sl_bulk_dn7), (locals.var_vbsbiz_dn10 + locals.var_phi_sl_bulk_dn10), (locals.var_vbsbiz_dn11 + locals.var_phi_sl_bulk_dn11), (locals.var_vbsbiz_dn12 + locals.var_phi_sl_bulk_dn12), (locals.var_vbsbiz_dn17 + locals.var_phi_sl_bulk_dn17),)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
        locals.var_phi_sl_bulk = assign12730_e15452;
        locals.var_phi_sl_bulk_dn0 = assign12730_e15452_d_n0;
        locals.var_phi_sl_bulk_dn2 = assign12730_e15452_d_n2;
        locals.var_phi_sl_bulk_dn6 = assign12730_e15452_d_n6;
        locals.var_phi_sl_bulk_dn7 = assign12730_e15452_d_n7;
        locals.var_phi_sl_bulk_dn10 = assign12730_e15452_d_n10;
        locals.var_phi_sl_bulk_dn11 = assign12730_e15452_d_n11;
        locals.var_phi_sl_bulk_dn12 = assign12730_e15452_d_n12;
        locals.var_phi_sl_bulk_dn17 = assign12730_e15452_d_n17;

        let (assign12740_e15471, assign12740_e15471_d_n0, assign12740_e15471_d_n2, assign12740_e15471_d_n6, assign12740_e15471_d_n7, assign12740_e15471_d_n10, assign12740_e15471_d_n11, assign12740_e15471_d_n12, assign12740_e15471_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 != 0.0)) {
        let assign12740_e15468: f64 = (locals.var_q_sl_bulk / locals.var_c_box);
        let assign12740_e15469: f64 = (locals.var_phi_sl_bulk - assign12740_e15468);
        (assign12740_e15469, (locals.var_phi_sl_bulk_dn0 - (locals.var_q_sl_bulk_dn0 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn2 - (locals.var_q_sl_bulk_dn2 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn6 - (locals.var_q_sl_bulk_dn6 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn7 - (locals.var_q_sl_bulk_dn7 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn10 - (locals.var_q_sl_bulk_dn10 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn11 - (locals.var_q_sl_bulk_dn11 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn12 - (locals.var_q_sl_bulk_dn12 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn17 - (locals.var_q_sl_bulk_dn17 / locals.var_c_box)),)
    } else {
        (locals.var_phi_bl_soi, locals.var_phi_bl_soi_dn0, locals.var_phi_bl_soi_dn2, locals.var_phi_bl_soi_dn6, locals.var_phi_bl_soi_dn7, locals.var_phi_bl_soi_dn10, locals.var_phi_bl_soi_dn11, locals.var_phi_bl_soi_dn12, locals.var_phi_bl_soi_dn17,)
    }
};
        locals.var_phi_bl_soi = assign12740_e15471;
        locals.var_phi_bl_soi_dn0 = assign12740_e15471_d_n0;
        locals.var_phi_bl_soi_dn2 = assign12740_e15471_d_n2;
        locals.var_phi_bl_soi_dn6 = assign12740_e15471_d_n6;
        locals.var_phi_bl_soi_dn7 = assign12740_e15471_d_n7;
        locals.var_phi_bl_soi_dn10 = assign12740_e15471_d_n10;
        locals.var_phi_bl_soi_dn11 = assign12740_e15471_d_n11;
        locals.var_phi_bl_soi_dn12 = assign12740_e15471_d_n12;
        locals.var_phi_bl_soi_dn17 = assign12740_e15471_d_n17;

        let (assign12760_e15503,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_lp_sl,)
    }
};
        locals.var_lp_sl = assign12760_e15503;

    }

    pub(super) fn stamp_transient_block_39(
        locals: &mut StampLocals,
    ) {
        let mut assign12770_loop_guard: usize = 0;
        while {
            let assign12770_cond_e15520: f64 = if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 == 0.0)) && (locals.var_lp_sl < locals.var_lp_sl_max)) { 1.0 } else { 0.0 };
            assign12770_cond_e15520 != 0.0
        } {
            assign12770_loop_guard += 1;
            assert!(assign12770_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign12770_body0_e15536, assign12770_body0_e15536_d_n10,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 == 0.0)) {
        (locals.var_cnst0bulk, locals.var_cnst0bulk_dn10,)
    } else {
        (locals.var_t1__blk334, locals.var_t1__blk334_dn10,)
    }
};
            locals.var_t1__blk334 = assign12770_body0_e15536;
            locals.var_t1__blk334_dn10 = assign12770_body0_e15536_d_n10;
            let (assign12770_body1_e15554, assign12770_body1_e15554_d_n0, assign12770_body1_e15554_d_n2, assign12770_body1_e15554_d_n6, assign12770_body1_e15554_d_n7, assign12770_body1_e15554_d_n10, assign12770_body1_e15554_d_n11, assign12770_body1_e15554_d_n12, assign12770_body1_e15554_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 == 0.0)) {
        let assign12770_body1_e15552: f64 = (locals.var_beta * locals.var_phi_sl_bulk);
        (assign12770_body1_e15552, (locals.var_beta * locals.var_phi_sl_bulk_dn0), (locals.var_beta * locals.var_phi_sl_bulk_dn2), (locals.var_beta * locals.var_phi_sl_bulk_dn6), (locals.var_beta * locals.var_phi_sl_bulk_dn7), ((locals.var_beta_dn10 * locals.var_phi_sl_bulk) + (locals.var_beta * locals.var_phi_sl_bulk_dn10)), (locals.var_beta * locals.var_phi_sl_bulk_dn11), (locals.var_beta * locals.var_phi_sl_bulk_dn12), (locals.var_beta * locals.var_phi_sl_bulk_dn17),)
    } else {
        (locals.var_t2__blk335, locals.var_t2__blk335_dn0, locals.var_t2__blk335_dn2, locals.var_t2__blk335_dn6, locals.var_t2__blk335_dn7, locals.var_t2__blk335_dn10, locals.var_t2__blk335_dn11, locals.var_t2__blk335_dn12, locals.var_t2__blk335_dn17,)
    }
};
            locals.var_t2__blk335 = assign12770_body1_e15554;
            locals.var_t2__blk335_dn0 = assign12770_body1_e15554_d_n0;
            locals.var_t2__blk335_dn2 = assign12770_body1_e15554_d_n2;
            locals.var_t2__blk335_dn6 = assign12770_body1_e15554_d_n6;
            locals.var_t2__blk335_dn7 = assign12770_body1_e15554_d_n7;
            locals.var_t2__blk335_dn10 = assign12770_body1_e15554_d_n10;
            locals.var_t2__blk335_dn11 = assign12770_body1_e15554_d_n11;
            locals.var_t2__blk335_dn12 = assign12770_body1_e15554_d_n12;
            locals.var_t2__blk335_dn17 = assign12770_body1_e15554_d_n17;
            let (assign12770_body2_e15572, assign12770_body2_e15572_d_n0, assign12770_body2_e15572_d_n2, assign12770_body2_e15572_d_n6, assign12770_body2_e15572_d_n7, assign12770_body2_e15572_d_n10, assign12770_body2_e15572_d_n11, assign12770_body2_e15572_d_n12, assign12770_body2_e15572_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 == 0.0)) {
        let assign12770_body2_e15569: f64 = (-locals.var_t2__blk335);
        let assign12770_body2_e15570: f64 = (assign12770_body2_e15569).exp();
        (assign12770_body2_e15570, (assign12770_body2_e15570 * (-locals.var_t2__blk335_dn0)), (assign12770_body2_e15570 * (-locals.var_t2__blk335_dn2)), (assign12770_body2_e15570 * (-locals.var_t2__blk335_dn6)), (assign12770_body2_e15570 * (-locals.var_t2__blk335_dn7)), (assign12770_body2_e15570 * (-locals.var_t2__blk335_dn10)), (assign12770_body2_e15570 * (-locals.var_t2__blk335_dn11)), (assign12770_body2_e15570 * (-locals.var_t2__blk335_dn12)), (assign12770_body2_e15570 * (-locals.var_t2__blk335_dn17)),)
    } else {
        (locals.var_t3__blk336, locals.var_t3__blk336_dn0, locals.var_t3__blk336_dn2, locals.var_t3__blk336_dn6, locals.var_t3__blk336_dn7, locals.var_t3__blk336_dn10, locals.var_t3__blk336_dn11, locals.var_t3__blk336_dn12, locals.var_t3__blk336_dn17,)
    }
};
            locals.var_t3__blk336 = assign12770_body2_e15572;
            locals.var_t3__blk336_dn0 = assign12770_body2_e15572_d_n0;
            locals.var_t3__blk336_dn2 = assign12770_body2_e15572_d_n2;
            locals.var_t3__blk336_dn6 = assign12770_body2_e15572_d_n6;
            locals.var_t3__blk336_dn7 = assign12770_body2_e15572_d_n7;
            locals.var_t3__blk336_dn10 = assign12770_body2_e15572_d_n10;
            locals.var_t3__blk336_dn11 = assign12770_body2_e15572_d_n11;
            locals.var_t3__blk336_dn12 = assign12770_body2_e15572_d_n12;
            locals.var_t3__blk336_dn17 = assign12770_body2_e15572_d_n17;
            let assign12770_body3_e15575: f64 = if locals.var_phi_sl_bulk > 1e-9 { 1.0 } else { 0.0 };
            locals.var_guard347 = assign12770_body3_e15575;
            let (assign12770_body4_e15596, assign12770_body4_e15596_d_n0, assign12770_body4_e15596_d_n2, assign12770_body4_e15596_d_n6, assign12770_body4_e15596_d_n7, assign12770_body4_e15596_d_n10, assign12770_body4_e15596_d_n11, assign12770_body4_e15596_d_n12, assign12770_body4_e15596_d_n17,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 == 0.0)) && (locals.var_guard347 != 0.0)) {
        let assign12770_body4_e15593: f64 = (locals.var_beta * locals.var_phi_sl_bulk);
        let assign12770_body4_e15594: f64 = (assign12770_body4_e15593).exp();
        (assign12770_body4_e15594, (assign12770_body4_e15594 * (locals.var_beta * locals.var_phi_sl_bulk_dn0)), (assign12770_body4_e15594 * (locals.var_beta * locals.var_phi_sl_bulk_dn2)), (assign12770_body4_e15594 * (locals.var_beta * locals.var_phi_sl_bulk_dn6)), (assign12770_body4_e15594 * (locals.var_beta * locals.var_phi_sl_bulk_dn7)), (assign12770_body4_e15594 * ((locals.var_beta_dn10 * locals.var_phi_sl_bulk) + (locals.var_beta * locals.var_phi_sl_bulk_dn10))), (assign12770_body4_e15594 * (locals.var_beta * locals.var_phi_sl_bulk_dn11)), (assign12770_body4_e15594 * (locals.var_beta * locals.var_phi_sl_bulk_dn12)), (assign12770_body4_e15594 * (locals.var_beta * locals.var_phi_sl_bulk_dn17)),)
    } else {
        (locals.var_t0__blk333, locals.var_t0__blk333_dn0, locals.var_t0__blk333_dn2, locals.var_t0__blk333_dn6, locals.var_t0__blk333_dn7, locals.var_t0__blk333_dn10, locals.var_t0__blk333_dn11, locals.var_t0__blk333_dn12, locals.var_t0__blk333_dn17,)
    }
};
            locals.var_t0__blk333 = assign12770_body4_e15596;
            locals.var_t0__blk333_dn0 = assign12770_body4_e15596_d_n0;
            locals.var_t0__blk333_dn2 = assign12770_body4_e15596_d_n2;
            locals.var_t0__blk333_dn6 = assign12770_body4_e15596_d_n6;
            locals.var_t0__blk333_dn7 = assign12770_body4_e15596_d_n7;
            locals.var_t0__blk333_dn10 = assign12770_body4_e15596_d_n10;
            locals.var_t0__blk333_dn11 = assign12770_body4_e15596_d_n11;
            locals.var_t0__blk333_dn12 = assign12770_body4_e15596_d_n12;
            locals.var_t0__blk333_dn17 = assign12770_body4_e15596_d_n17;
            let (assign12770_body5_e15628, assign12770_body5_e15628_d_n0, assign12770_body5_e15628_d_n2, assign12770_body5_e15628_d_n6, assign12770_body5_e15628_d_n7, assign12770_body5_e15628_d_n10, assign12770_body5_e15628_d_n11, assign12770_body5_e15628_d_n12, assign12770_body5_e15628_d_n17,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 == 0.0)) && (locals.var_guard347 != 0.0)) {
        let assign12770_body5_e15613: f64 = (-locals.var_t1__blk334);
        let assign12770_body5_e15616: f64 = (locals.var_t3__blk336 + locals.var_t2__blk335);
        let assign12770_body5_e15618: f64 = (assign12770_body5_e15616 - 1.0);
        let assign12770_body5_e15622: f64 = (locals.var_t0__blk333 - 1.0);
        let assign12770_body5_e15623: f64 = (locals.var_cnst1bulk * assign12770_body5_e15622);
        let assign12770_body5_e15624: f64 = (assign12770_body5_e15618 + assign12770_body5_e15623);
        let assign12770_body5_e15625: f64 = (assign12770_body5_e15624).sqrt();
        let assign12770_body5_e15626: f64 = (assign12770_body5_e15613 * assign12770_body5_e15625);
        (assign12770_body5_e15626, (assign12770_body5_e15613 * (((locals.var_t3__blk336_dn0 + locals.var_t2__blk335_dn0) + ((locals.var_cnst1bulk_dn0 * assign12770_body5_e15622) + (locals.var_cnst1bulk * locals.var_t0__blk333_dn0))) / (2.0 * assign12770_body5_e15625))), (assign12770_body5_e15613 * (((locals.var_t3__blk336_dn2 + locals.var_t2__blk335_dn2) + ((locals.var_cnst1bulk_dn2 * assign12770_body5_e15622) + (locals.var_cnst1bulk * locals.var_t0__blk333_dn2))) / (2.0 * assign12770_body5_e15625))), (assign12770_body5_e15613 * (((locals.var_t3__blk336_dn6 + locals.var_t2__blk335_dn6) + ((locals.var_cnst1bulk_dn6 * assign12770_body5_e15622) + (locals.var_cnst1bulk * locals.var_t0__blk333_dn6))) / (2.0 * assign12770_body5_e15625))), (assign12770_body5_e15613 * (((locals.var_t3__blk336_dn7 + locals.var_t2__blk335_dn7) + ((locals.var_cnst1bulk_dn7 * assign12770_body5_e15622) + (locals.var_cnst1bulk * locals.var_t0__blk333_dn7))) / (2.0 * assign12770_body5_e15625))), (((-locals.var_t1__blk334_dn10) * assign12770_body5_e15625) + (assign12770_body5_e15613 * (((locals.var_t3__blk336_dn10 + locals.var_t2__blk335_dn10) + ((locals.var_cnst1bulk_dn10 * assign12770_body5_e15622) + (locals.var_cnst1bulk * locals.var_t0__blk333_dn10))) / (2.0 * assign12770_body5_e15625)))), (assign12770_body5_e15613 * (((locals.var_t3__blk336_dn11 + locals.var_t2__blk335_dn11) + ((locals.var_cnst1bulk_dn11 * assign12770_body5_e15622) + (locals.var_cnst1bulk * locals.var_t0__blk333_dn11))) / (2.0 * assign12770_body5_e15625))), (assign12770_body5_e15613 * (((locals.var_t3__blk336_dn12 + locals.var_t2__blk335_dn12) + ((locals.var_cnst1bulk_dn12 * assign12770_body5_e15622) + (locals.var_cnst1bulk * locals.var_t0__blk333_dn12))) / (2.0 * assign12770_body5_e15625))), (assign12770_body5_e15613 * (((locals.var_t3__blk336_dn17 + locals.var_t2__blk335_dn17) + ((locals.var_cnst1bulk_dn17 * assign12770_body5_e15622) + (locals.var_cnst1bulk * locals.var_t0__blk333_dn17))) / (2.0 * assign12770_body5_e15625))),)
    } else {
        (locals.var_t4__blk337, locals.var_t4__blk337_dn0, locals.var_t4__blk337_dn2, locals.var_t4__blk337_dn6, locals.var_t4__blk337_dn7, locals.var_t4__blk337_dn10, locals.var_t4__blk337_dn11, locals.var_t4__blk337_dn12, locals.var_t4__blk337_dn17,)
    }
};
            locals.var_t4__blk337 = assign12770_body5_e15628;
            locals.var_t4__blk337_dn0 = assign12770_body5_e15628_d_n0;
            locals.var_t4__blk337_dn2 = assign12770_body5_e15628_d_n2;
            locals.var_t4__blk337_dn6 = assign12770_body5_e15628_d_n6;
            locals.var_t4__blk337_dn7 = assign12770_body5_e15628_d_n7;
            locals.var_t4__blk337_dn10 = assign12770_body5_e15628_d_n10;
            locals.var_t4__blk337_dn11 = assign12770_body5_e15628_d_n11;
            locals.var_t4__blk337_dn12 = assign12770_body5_e15628_d_n12;
            locals.var_t4__blk337_dn17 = assign12770_body5_e15628_d_n17;
            let (assign12770_body6_e15657, assign12770_body6_e15657_d_n0, assign12770_body6_e15657_d_n2, assign12770_body6_e15657_d_n6, assign12770_body6_e15657_d_n7, assign12770_body6_e15657_d_n10, assign12770_body6_e15657_d_n11, assign12770_body6_e15657_d_n12, assign12770_body6_e15657_d_n17,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 == 0.0)) && (locals.var_guard347 != 0.0)) {
        let assign12770_body6_e15646: f64 = (locals.var_c0bulk / locals.var_t4__blk337);
        let assign12770_body6_e15648: f64 = (-locals.var_t3__blk336);
        let assign12770_body6_e15650: f64 = (assign12770_body6_e15648 + 1.0);
        let assign12770_body6_e15653: f64 = (locals.var_cnst1bulk * locals.var_t0__blk333);
        let assign12770_body6_e15654: f64 = (assign12770_body6_e15650 + assign12770_body6_e15653);
        let assign12770_body6_e15655: f64 = (assign12770_body6_e15646 * assign12770_body6_e15654);
        (assign12770_body6_e15655, (((-((locals.var_c0bulk * locals.var_t4__blk337_dn0) / (locals.var_t4__blk337 * locals.var_t4__blk337))) * assign12770_body6_e15654) + (assign12770_body6_e15646 * ((-locals.var_t3__blk336_dn0) + ((locals.var_cnst1bulk_dn0 * locals.var_t0__blk333) + (locals.var_cnst1bulk * locals.var_t0__blk333_dn0))))), (((-((locals.var_c0bulk * locals.var_t4__blk337_dn2) / (locals.var_t4__blk337 * locals.var_t4__blk337))) * assign12770_body6_e15654) + (assign12770_body6_e15646 * ((-locals.var_t3__blk336_dn2) + ((locals.var_cnst1bulk_dn2 * locals.var_t0__blk333) + (locals.var_cnst1bulk * locals.var_t0__blk333_dn2))))), (((-((locals.var_c0bulk * locals.var_t4__blk337_dn6) / (locals.var_t4__blk337 * locals.var_t4__blk337))) * assign12770_body6_e15654) + (assign12770_body6_e15646 * ((-locals.var_t3__blk336_dn6) + ((locals.var_cnst1bulk_dn6 * locals.var_t0__blk333) + (locals.var_cnst1bulk * locals.var_t0__blk333_dn6))))), (((-((locals.var_c0bulk * locals.var_t4__blk337_dn7) / (locals.var_t4__blk337 * locals.var_t4__blk337))) * assign12770_body6_e15654) + (assign12770_body6_e15646 * ((-locals.var_t3__blk336_dn7) + ((locals.var_cnst1bulk_dn7 * locals.var_t0__blk333) + (locals.var_cnst1bulk * locals.var_t0__blk333_dn7))))), (((-((locals.var_c0bulk * locals.var_t4__blk337_dn10) / (locals.var_t4__blk337 * locals.var_t4__blk337))) * assign12770_body6_e15654) + (assign12770_body6_e15646 * ((-locals.var_t3__blk336_dn10) + ((locals.var_cnst1bulk_dn10 * locals.var_t0__blk333) + (locals.var_cnst1bulk * locals.var_t0__blk333_dn10))))), (((-((locals.var_c0bulk * locals.var_t4__blk337_dn11) / (locals.var_t4__blk337 * locals.var_t4__blk337))) * assign12770_body6_e15654) + (assign12770_body6_e15646 * ((-locals.var_t3__blk336_dn11) + ((locals.var_cnst1bulk_dn11 * locals.var_t0__blk333) + (locals.var_cnst1bulk * locals.var_t0__blk333_dn11))))), (((-((locals.var_c0bulk * locals.var_t4__blk337_dn12) / (locals.var_t4__blk337 * locals.var_t4__blk337))) * assign12770_body6_e15654) + (assign12770_body6_e15646 * ((-locals.var_t3__blk336_dn12) + ((locals.var_cnst1bulk_dn12 * locals.var_t0__blk333) + (locals.var_cnst1bulk * locals.var_t0__blk333_dn12))))), (((-((locals.var_c0bulk * locals.var_t4__blk337_dn17) / (locals.var_t4__blk337 * locals.var_t4__blk337))) * assign12770_body6_e15654) + (assign12770_body6_e15646 * ((-locals.var_t3__blk336_dn17) + ((locals.var_cnst1bulk_dn17 * locals.var_t0__blk333) + (locals.var_cnst1bulk * locals.var_t0__blk333_dn17))))),)
    } else {
        (locals.var_t5__blk338, locals.var_t5__blk338_dn0, locals.var_t5__blk338_dn2, locals.var_t5__blk338_dn6, locals.var_t5__blk338_dn7, locals.var_t5__blk338_dn10, locals.var_t5__blk338_dn11, locals.var_t5__blk338_dn12, locals.var_t5__blk338_dn17,)
    }
};
            locals.var_t5__blk338 = assign12770_body6_e15657;
            locals.var_t5__blk338_dn0 = assign12770_body6_e15657_d_n0;
            locals.var_t5__blk338_dn2 = assign12770_body6_e15657_d_n2;
            locals.var_t5__blk338_dn6 = assign12770_body6_e15657_d_n6;
            locals.var_t5__blk338_dn7 = assign12770_body6_e15657_d_n7;
            locals.var_t5__blk338_dn10 = assign12770_body6_e15657_d_n10;
            locals.var_t5__blk338_dn11 = assign12770_body6_e15657_d_n11;
            locals.var_t5__blk338_dn12 = assign12770_body6_e15657_d_n12;
            locals.var_t5__blk338_dn17 = assign12770_body6_e15657_d_n17;
            let assign12770_body7_e15660: f64 = (-1e-9);
            let assign12770_body7_e15661: f64 = if locals.var_phi_sl_bulk < assign12770_body7_e15660 { 1.0 } else { 0.0 };
            locals.var_guard348 = assign12770_body7_e15661;
            let (assign12770_body8_e15689, assign12770_body8_e15689_d_n0, assign12770_body8_e15689_d_n2, assign12770_body8_e15689_d_n6, assign12770_body8_e15689_d_n7, assign12770_body8_e15689_d_n10, assign12770_body8_e15689_d_n11, assign12770_body8_e15689_d_n12, assign12770_body8_e15689_d_n17,) = {
    if (((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 == 0.0)) && (locals.var_guard347 == 0.0)) && (locals.var_guard348 != 0.0)) {
        let assign12770_body8_e15683: f64 = (locals.var_t3__blk336 + locals.var_t2__blk335);
        let assign12770_body8_e15685: f64 = (assign12770_body8_e15683 - 1.0);
        let assign12770_body8_e15686: f64 = (assign12770_body8_e15685).sqrt();
        let assign12770_body8_e15687: f64 = (locals.var_t1__blk334 * assign12770_body8_e15686);
        (assign12770_body8_e15687, (locals.var_t1__blk334 * ((locals.var_t3__blk336_dn0 + locals.var_t2__blk335_dn0) / (2.0 * assign12770_body8_e15686))), (locals.var_t1__blk334 * ((locals.var_t3__blk336_dn2 + locals.var_t2__blk335_dn2) / (2.0 * assign12770_body8_e15686))), (locals.var_t1__blk334 * ((locals.var_t3__blk336_dn6 + locals.var_t2__blk335_dn6) / (2.0 * assign12770_body8_e15686))), (locals.var_t1__blk334 * ((locals.var_t3__blk336_dn7 + locals.var_t2__blk335_dn7) / (2.0 * assign12770_body8_e15686))), ((locals.var_t1__blk334_dn10 * assign12770_body8_e15686) + (locals.var_t1__blk334 * ((locals.var_t3__blk336_dn10 + locals.var_t2__blk335_dn10) / (2.0 * assign12770_body8_e15686)))), (locals.var_t1__blk334 * ((locals.var_t3__blk336_dn11 + locals.var_t2__blk335_dn11) / (2.0 * assign12770_body8_e15686))), (locals.var_t1__blk334 * ((locals.var_t3__blk336_dn12 + locals.var_t2__blk335_dn12) / (2.0 * assign12770_body8_e15686))), (locals.var_t1__blk334 * ((locals.var_t3__blk336_dn17 + locals.var_t2__blk335_dn17) / (2.0 * assign12770_body8_e15686))),)
    } else {
        (locals.var_t4__blk337, locals.var_t4__blk337_dn0, locals.var_t4__blk337_dn2, locals.var_t4__blk337_dn6, locals.var_t4__blk337_dn7, locals.var_t4__blk337_dn10, locals.var_t4__blk337_dn11, locals.var_t4__blk337_dn12, locals.var_t4__blk337_dn17,)
    }
};
            locals.var_t4__blk337 = assign12770_body8_e15689;
            locals.var_t4__blk337_dn0 = assign12770_body8_e15689_d_n0;
            locals.var_t4__blk337_dn2 = assign12770_body8_e15689_d_n2;
            locals.var_t4__blk337_dn6 = assign12770_body8_e15689_d_n6;
            locals.var_t4__blk337_dn7 = assign12770_body8_e15689_d_n7;
            locals.var_t4__blk337_dn10 = assign12770_body8_e15689_d_n10;
            locals.var_t4__blk337_dn11 = assign12770_body8_e15689_d_n11;
            locals.var_t4__blk337_dn12 = assign12770_body8_e15689_d_n12;
            locals.var_t4__blk337_dn17 = assign12770_body8_e15689_d_n17;
            let (assign12770_body9_e15717, assign12770_body9_e15717_d_n0, assign12770_body9_e15717_d_n2, assign12770_body9_e15717_d_n6, assign12770_body9_e15717_d_n7, assign12770_body9_e15717_d_n10, assign12770_body9_e15717_d_n11, assign12770_body9_e15717_d_n12, assign12770_body9_e15717_d_n17,) = {
    if (((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 == 0.0)) && (locals.var_guard347 == 0.0)) && (locals.var_guard348 != 0.0)) {
        let assign12770_body9_e15710: f64 = (locals.var_c0bulk / locals.var_t4__blk337);
        let assign12770_body9_e15712: f64 = (-locals.var_t3__blk336);
        let assign12770_body9_e15714: f64 = (assign12770_body9_e15712 + 1.0);
        let assign12770_body9_e15715: f64 = (assign12770_body9_e15710 * assign12770_body9_e15714);
        (assign12770_body9_e15715, (((-((locals.var_c0bulk * locals.var_t4__blk337_dn0) / (locals.var_t4__blk337 * locals.var_t4__blk337))) * assign12770_body9_e15714) + (assign12770_body9_e15710 * (-locals.var_t3__blk336_dn0))), (((-((locals.var_c0bulk * locals.var_t4__blk337_dn2) / (locals.var_t4__blk337 * locals.var_t4__blk337))) * assign12770_body9_e15714) + (assign12770_body9_e15710 * (-locals.var_t3__blk336_dn2))), (((-((locals.var_c0bulk * locals.var_t4__blk337_dn6) / (locals.var_t4__blk337 * locals.var_t4__blk337))) * assign12770_body9_e15714) + (assign12770_body9_e15710 * (-locals.var_t3__blk336_dn6))), (((-((locals.var_c0bulk * locals.var_t4__blk337_dn7) / (locals.var_t4__blk337 * locals.var_t4__blk337))) * assign12770_body9_e15714) + (assign12770_body9_e15710 * (-locals.var_t3__blk336_dn7))), (((-((locals.var_c0bulk * locals.var_t4__blk337_dn10) / (locals.var_t4__blk337 * locals.var_t4__blk337))) * assign12770_body9_e15714) + (assign12770_body9_e15710 * (-locals.var_t3__blk336_dn10))), (((-((locals.var_c0bulk * locals.var_t4__blk337_dn11) / (locals.var_t4__blk337 * locals.var_t4__blk337))) * assign12770_body9_e15714) + (assign12770_body9_e15710 * (-locals.var_t3__blk336_dn11))), (((-((locals.var_c0bulk * locals.var_t4__blk337_dn12) / (locals.var_t4__blk337 * locals.var_t4__blk337))) * assign12770_body9_e15714) + (assign12770_body9_e15710 * (-locals.var_t3__blk336_dn12))), (((-((locals.var_c0bulk * locals.var_t4__blk337_dn17) / (locals.var_t4__blk337 * locals.var_t4__blk337))) * assign12770_body9_e15714) + (assign12770_body9_e15710 * (-locals.var_t3__blk336_dn17))),)
    } else {
        (locals.var_t5__blk338, locals.var_t5__blk338_dn0, locals.var_t5__blk338_dn2, locals.var_t5__blk338_dn6, locals.var_t5__blk338_dn7, locals.var_t5__blk338_dn10, locals.var_t5__blk338_dn11, locals.var_t5__blk338_dn12, locals.var_t5__blk338_dn17,)
    }
};
            locals.var_t5__blk338 = assign12770_body9_e15717;
            locals.var_t5__blk338_dn0 = assign12770_body9_e15717_d_n0;
            locals.var_t5__blk338_dn2 = assign12770_body9_e15717_d_n2;
            locals.var_t5__blk338_dn6 = assign12770_body9_e15717_d_n6;
            locals.var_t5__blk338_dn7 = assign12770_body9_e15717_d_n7;
            locals.var_t5__blk338_dn10 = assign12770_body9_e15717_d_n10;
            locals.var_t5__blk338_dn11 = assign12770_body9_e15717_d_n11;
            locals.var_t5__blk338_dn12 = assign12770_body9_e15717_d_n12;
            locals.var_t5__blk338_dn17 = assign12770_body9_e15717_d_n17;
            let (assign12770_body10_e15747, assign12770_body10_e15747_d_n0, assign12770_body10_e15747_d_n2, assign12770_body10_e15747_d_n6, assign12770_body10_e15747_d_n7, assign12770_body10_e15747_d_n10, assign12770_body10_e15747_d_n11, assign12770_body10_e15747_d_n12, assign12770_body10_e15747_d_n17,) = {
    if (((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 == 0.0)) && (locals.var_guard347 == 0.0)) && (locals.var_guard348 == 0.0)) {
        let assign12770_body10_e15739: f64 = (locals.var_c0bulk / locals.var_beta);
        let assign12770_body10_e15740: f64 = (assign12770_body10_e15739).sqrt();
        let assign12770_body10_e15741: f64 = (-assign12770_body10_e15740);
        let assign12770_body10_e15743: f64 = (assign12770_body10_e15741 * locals.var_beta);
        let assign12770_body10_e15745: f64 = (assign12770_body10_e15743 * locals.var_phi_sl_bulk);
        (assign12770_body10_e15745, (assign12770_body10_e15743 * locals.var_phi_sl_bulk_dn0), (assign12770_body10_e15743 * locals.var_phi_sl_bulk_dn2), (assign12770_body10_e15743 * locals.var_phi_sl_bulk_dn6), (assign12770_body10_e15743 * locals.var_phi_sl_bulk_dn7), (((((-((-((locals.var_c0bulk * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) / (2.0 * assign12770_body10_e15740))) * locals.var_beta) + (assign12770_body10_e15741 * locals.var_beta_dn10)) * locals.var_phi_sl_bulk) + (assign12770_body10_e15743 * locals.var_phi_sl_bulk_dn10)), (assign12770_body10_e15743 * locals.var_phi_sl_bulk_dn11), (assign12770_body10_e15743 * locals.var_phi_sl_bulk_dn12), (assign12770_body10_e15743 * locals.var_phi_sl_bulk_dn17),)
    } else {
        (locals.var_t4__blk337, locals.var_t4__blk337_dn0, locals.var_t4__blk337_dn2, locals.var_t4__blk337_dn6, locals.var_t4__blk337_dn7, locals.var_t4__blk337_dn10, locals.var_t4__blk337_dn11, locals.var_t4__blk337_dn12, locals.var_t4__blk337_dn17,)
    }
};
            locals.var_t4__blk337 = assign12770_body10_e15747;
            locals.var_t4__blk337_dn0 = assign12770_body10_e15747_d_n0;
            locals.var_t4__blk337_dn2 = assign12770_body10_e15747_d_n2;
            locals.var_t4__blk337_dn6 = assign12770_body10_e15747_d_n6;
            locals.var_t4__blk337_dn7 = assign12770_body10_e15747_d_n7;
            locals.var_t4__blk337_dn10 = assign12770_body10_e15747_d_n10;
            locals.var_t4__blk337_dn11 = assign12770_body10_e15747_d_n11;
            locals.var_t4__blk337_dn12 = assign12770_body10_e15747_d_n12;
            locals.var_t4__blk337_dn17 = assign12770_body10_e15747_d_n17;
            let (assign12770_body11_e15773, assign12770_body11_e15773_d_n0, assign12770_body11_e15773_d_n2, assign12770_body11_e15773_d_n6, assign12770_body11_e15773_d_n7, assign12770_body11_e15773_d_n10, assign12770_body11_e15773_d_n11, assign12770_body11_e15773_d_n12, assign12770_body11_e15773_d_n17,) = {
    if (((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 == 0.0)) && (locals.var_guard347 == 0.0)) && (locals.var_guard348 == 0.0)) {
        let assign12770_body11_e15769: f64 = (locals.var_c0bulk * locals.var_beta);
        let assign12770_body11_e15770: f64 = (assign12770_body11_e15769).sqrt();
        let assign12770_body11_e15771: f64 = (-assign12770_body11_e15770);
        (assign12770_body11_e15771, 0.0, 0.0, 0.0, 0.0, (-((locals.var_c0bulk * locals.var_beta_dn10) / (2.0 * assign12770_body11_e15770))), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5__blk338, locals.var_t5__blk338_dn0, locals.var_t5__blk338_dn2, locals.var_t5__blk338_dn6, locals.var_t5__blk338_dn7, locals.var_t5__blk338_dn10, locals.var_t5__blk338_dn11, locals.var_t5__blk338_dn12, locals.var_t5__blk338_dn17,)
    }
};
            locals.var_t5__blk338 = assign12770_body11_e15773;
            locals.var_t5__blk338_dn0 = assign12770_body11_e15773_d_n0;
            locals.var_t5__blk338_dn2 = assign12770_body11_e15773_d_n2;
            locals.var_t5__blk338_dn6 = assign12770_body11_e15773_d_n6;
            locals.var_t5__blk338_dn7 = assign12770_body11_e15773_d_n7;
            locals.var_t5__blk338_dn10 = assign12770_body11_e15773_d_n10;
            locals.var_t5__blk338_dn11 = assign12770_body11_e15773_d_n11;
            locals.var_t5__blk338_dn12 = assign12770_body11_e15773_d_n12;
            locals.var_t5__blk338_dn17 = assign12770_body11_e15773_d_n17;
            let (assign12770_body12_e15798, assign12770_body12_e15798_d_n0, assign12770_body12_e15798_d_n2, assign12770_body12_e15798_d_n6, assign12770_body12_e15798_d_n7, assign12770_body12_e15798_d_n10, assign12770_body12_e15798_d_n11, assign12770_body12_e15798_d_n12, assign12770_body12_e15798_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 == 0.0)) {
        let assign12770_body12_e15789: f64 = (locals.var_t4__blk337 * locals.var_t4__blk337);
        let assign12770_body12_e15792: f64 = (4.0 * locals.var_q_fd_dlt1);
        let assign12770_body12_e15794: f64 = (assign12770_body12_e15792 * locals.var_q_fd_dlt1);
        let assign12770_body12_e15795: f64 = (assign12770_body12_e15789 + assign12770_body12_e15794);
        let assign12770_body12_e15796: f64 = (assign12770_body12_e15795).sqrt();
        (assign12770_body12_e15796, ((((locals.var_t4__blk337_dn0 * locals.var_t4__blk337) + (locals.var_t4__blk337 * locals.var_t4__blk337_dn0)) + (((4.0 * locals.var_q_fd_dlt1_dn0) * locals.var_q_fd_dlt1) + (assign12770_body12_e15792 * locals.var_q_fd_dlt1_dn0))) / (2.0 * assign12770_body12_e15796)), ((((locals.var_t4__blk337_dn2 * locals.var_t4__blk337) + (locals.var_t4__blk337 * locals.var_t4__blk337_dn2)) + (((4.0 * locals.var_q_fd_dlt1_dn2) * locals.var_q_fd_dlt1) + (assign12770_body12_e15792 * locals.var_q_fd_dlt1_dn2))) / (2.0 * assign12770_body12_e15796)), ((((locals.var_t4__blk337_dn6 * locals.var_t4__blk337) + (locals.var_t4__blk337 * locals.var_t4__blk337_dn6)) + (((4.0 * locals.var_q_fd_dlt1_dn6) * locals.var_q_fd_dlt1) + (assign12770_body12_e15792 * locals.var_q_fd_dlt1_dn6))) / (2.0 * assign12770_body12_e15796)), ((((locals.var_t4__blk337_dn7 * locals.var_t4__blk337) + (locals.var_t4__blk337 * locals.var_t4__blk337_dn7)) + (((4.0 * locals.var_q_fd_dlt1_dn7) * locals.var_q_fd_dlt1) + (assign12770_body12_e15792 * locals.var_q_fd_dlt1_dn7))) / (2.0 * assign12770_body12_e15796)), ((((locals.var_t4__blk337_dn10 * locals.var_t4__blk337) + (locals.var_t4__blk337 * locals.var_t4__blk337_dn10)) + (((4.0 * locals.var_q_fd_dlt1_dn10) * locals.var_q_fd_dlt1) + (assign12770_body12_e15792 * locals.var_q_fd_dlt1_dn10))) / (2.0 * assign12770_body12_e15796)), ((((locals.var_t4__blk337_dn11 * locals.var_t4__blk337) + (locals.var_t4__blk337 * locals.var_t4__blk337_dn11)) + (((4.0 * locals.var_q_fd_dlt1_dn11) * locals.var_q_fd_dlt1) + (assign12770_body12_e15792 * locals.var_q_fd_dlt1_dn11))) / (2.0 * assign12770_body12_e15796)), ((((locals.var_t4__blk337_dn12 * locals.var_t4__blk337) + (locals.var_t4__blk337 * locals.var_t4__blk337_dn12)) + (((4.0 * locals.var_q_fd_dlt1_dn12) * locals.var_q_fd_dlt1) + (assign12770_body12_e15792 * locals.var_q_fd_dlt1_dn12))) / (2.0 * assign12770_body12_e15796)), ((((locals.var_t4__blk337_dn17 * locals.var_t4__blk337) + (locals.var_t4__blk337 * locals.var_t4__blk337_dn17)) + (((4.0 * locals.var_q_fd_dlt1_dn17) * locals.var_q_fd_dlt1) + (assign12770_body12_e15792 * locals.var_q_fd_dlt1_dn17))) / (2.0 * assign12770_body12_e15796)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign12770_body12_e15798;
            locals.var_tmf2_dn0 = assign12770_body12_e15798_d_n0;
            locals.var_tmf2_dn2 = assign12770_body12_e15798_d_n2;
            locals.var_tmf2_dn6 = assign12770_body12_e15798_d_n6;
            locals.var_tmf2_dn7 = assign12770_body12_e15798_d_n7;
            locals.var_tmf2_dn10 = assign12770_body12_e15798_d_n10;
            locals.var_tmf2_dn11 = assign12770_body12_e15798_d_n11;
            locals.var_tmf2_dn12 = assign12770_body12_e15798_d_n12;
            locals.var_tmf2_dn17 = assign12770_body12_e15798_d_n17;
            let (assign12770_body13_e15820, assign12770_body13_e15820_d_n0, assign12770_body13_e15820_d_n2, assign12770_body13_e15820_d_n6, assign12770_body13_e15820_d_n7, assign12770_body13_e15820_d_n10, assign12770_body13_e15820_d_n11, assign12770_body13_e15820_d_n12, assign12770_body13_e15820_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 == 0.0)) {
        let assign12770_body13_e15816: f64 = (locals.var_t4__blk337 / locals.var_tmf2);
        let assign12770_body13_e15817: f64 = (1.0 + assign12770_body13_e15816);
        let assign12770_body13_e15818: f64 = (0.5 * assign12770_body13_e15817);
        (assign12770_body13_e15818, (0.5 * (((locals.var_t4__blk337_dn0 * locals.var_tmf2) - (locals.var_t4__blk337 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk337_dn2 * locals.var_tmf2) - (locals.var_t4__blk337 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk337_dn6 * locals.var_tmf2) - (locals.var_t4__blk337 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk337_dn7 * locals.var_tmf2) - (locals.var_t4__blk337 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk337_dn10 * locals.var_tmf2) - (locals.var_t4__blk337 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk337_dn11 * locals.var_tmf2) - (locals.var_t4__blk337 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk337_dn12 * locals.var_tmf2) - (locals.var_t4__blk337 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk337_dn17 * locals.var_tmf2) - (locals.var_t4__blk337 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t7__blk340, locals.var_t7__blk340_dn0, locals.var_t7__blk340_dn2, locals.var_t7__blk340_dn6, locals.var_t7__blk340_dn7, locals.var_t7__blk340_dn10, locals.var_t7__blk340_dn11, locals.var_t7__blk340_dn12, locals.var_t7__blk340_dn17,)
    }
};
            locals.var_t7__blk340 = assign12770_body13_e15820;
            locals.var_t7__blk340_dn0 = assign12770_body13_e15820_d_n0;
            locals.var_t7__blk340_dn2 = assign12770_body13_e15820_d_n2;
            locals.var_t7__blk340_dn6 = assign12770_body13_e15820_d_n6;
            locals.var_t7__blk340_dn7 = assign12770_body13_e15820_d_n7;
            locals.var_t7__blk340_dn10 = assign12770_body13_e15820_d_n10;
            locals.var_t7__blk340_dn11 = assign12770_body13_e15820_d_n11;
            locals.var_t7__blk340_dn12 = assign12770_body13_e15820_d_n12;
            locals.var_t7__blk340_dn17 = assign12770_body13_e15820_d_n17;
            let (assign12770_body14_e15844, assign12770_body14_e15844_d_n0, assign12770_body14_e15844_d_n2, assign12770_body14_e15844_d_n6, assign12770_body14_e15844_d_n7, assign12770_body14_e15844_d_n10, assign12770_body14_e15844_d_n11, assign12770_body14_e15844_d_n12, assign12770_body14_e15844_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 == 0.0)) {
        let assign12770_body14_e15837: f64 = (locals.var_t4__blk337 + locals.var_tmf2);
        let assign12770_body14_e15838: f64 = (0.5 * assign12770_body14_e15837);
        let assign12770_body14_e15841: f64 = (1e-10 * locals.var_q_fd_dlt1);
        let assign12770_body14_e15842: f64 = (assign12770_body14_e15838 + assign12770_body14_e15841);
        (assign12770_body14_e15842, ((0.5 * (locals.var_t4__blk337_dn0 + locals.var_tmf2_dn0)) + (1e-10 * locals.var_q_fd_dlt1_dn0)), ((0.5 * (locals.var_t4__blk337_dn2 + locals.var_tmf2_dn2)) + (1e-10 * locals.var_q_fd_dlt1_dn2)), ((0.5 * (locals.var_t4__blk337_dn6 + locals.var_tmf2_dn6)) + (1e-10 * locals.var_q_fd_dlt1_dn6)), ((0.5 * (locals.var_t4__blk337_dn7 + locals.var_tmf2_dn7)) + (1e-10 * locals.var_q_fd_dlt1_dn7)), ((0.5 * (locals.var_t4__blk337_dn10 + locals.var_tmf2_dn10)) + (1e-10 * locals.var_q_fd_dlt1_dn10)), ((0.5 * (locals.var_t4__blk337_dn11 + locals.var_tmf2_dn11)) + (1e-10 * locals.var_q_fd_dlt1_dn11)), ((0.5 * (locals.var_t4__blk337_dn12 + locals.var_tmf2_dn12)) + (1e-10 * locals.var_q_fd_dlt1_dn12)), ((0.5 * (locals.var_t4__blk337_dn17 + locals.var_tmf2_dn17)) + (1e-10 * locals.var_q_fd_dlt1_dn17)),)
    } else {
        (locals.var_t6__blk339, locals.var_t6__blk339_dn0, locals.var_t6__blk339_dn2, locals.var_t6__blk339_dn6, locals.var_t6__blk339_dn7, locals.var_t6__blk339_dn10, locals.var_t6__blk339_dn11, locals.var_t6__blk339_dn12, locals.var_t6__blk339_dn17,)
    }
};
            locals.var_t6__blk339 = assign12770_body14_e15844;
            locals.var_t6__blk339_dn0 = assign12770_body14_e15844_d_n0;
            locals.var_t6__blk339_dn2 = assign12770_body14_e15844_d_n2;
            locals.var_t6__blk339_dn6 = assign12770_body14_e15844_d_n6;
            locals.var_t6__blk339_dn7 = assign12770_body14_e15844_d_n7;
            locals.var_t6__blk339_dn10 = assign12770_body14_e15844_d_n10;
            locals.var_t6__blk339_dn11 = assign12770_body14_e15844_d_n11;
            locals.var_t6__blk339_dn12 = assign12770_body14_e15844_d_n12;
            locals.var_t6__blk339_dn17 = assign12770_body14_e15844_d_n17;
            let assign12770_body15_e15847: f64 = if locals.var_t6__blk339 < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard349 = assign12770_body15_e15847;
            let (assign12770_body16_e15865, assign12770_body16_e15865_d_n0, assign12770_body16_e15865_d_n2, assign12770_body16_e15865_d_n6, assign12770_body16_e15865_d_n7, assign12770_body16_e15865_d_n10, assign12770_body16_e15865_d_n11, assign12770_body16_e15865_d_n12, assign12770_body16_e15865_d_n17,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 == 0.0)) && (locals.var_guard349 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk339, locals.var_t6__blk339_dn0, locals.var_t6__blk339_dn2, locals.var_t6__blk339_dn6, locals.var_t6__blk339_dn7, locals.var_t6__blk339_dn10, locals.var_t6__blk339_dn11, locals.var_t6__blk339_dn12, locals.var_t6__blk339_dn17,)
    }
};
            locals.var_t6__blk339 = assign12770_body16_e15865;
            locals.var_t6__blk339_dn0 = assign12770_body16_e15865_d_n0;
            locals.var_t6__blk339_dn2 = assign12770_body16_e15865_d_n2;
            locals.var_t6__blk339_dn6 = assign12770_body16_e15865_d_n6;
            locals.var_t6__blk339_dn7 = assign12770_body16_e15865_d_n7;
            locals.var_t6__blk339_dn10 = assign12770_body16_e15865_d_n10;
            locals.var_t6__blk339_dn11 = assign12770_body16_e15865_d_n11;
            locals.var_t6__blk339_dn12 = assign12770_body16_e15865_d_n12;
            locals.var_t6__blk339_dn17 = assign12770_body16_e15865_d_n17;
            let (assign12770_body17_e15883, assign12770_body17_e15883_d_n0, assign12770_body17_e15883_d_n2, assign12770_body17_e15883_d_n6, assign12770_body17_e15883_d_n7, assign12770_body17_e15883_d_n10, assign12770_body17_e15883_d_n11, assign12770_body17_e15883_d_n12, assign12770_body17_e15883_d_n17,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 == 0.0)) && (locals.var_guard349 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7__blk340, locals.var_t7__blk340_dn0, locals.var_t7__blk340_dn2, locals.var_t7__blk340_dn6, locals.var_t7__blk340_dn7, locals.var_t7__blk340_dn10, locals.var_t7__blk340_dn11, locals.var_t7__blk340_dn12, locals.var_t7__blk340_dn17,)
    }
};
            locals.var_t7__blk340 = assign12770_body17_e15883;
            locals.var_t7__blk340_dn0 = assign12770_body17_e15883_d_n0;
            locals.var_t7__blk340_dn2 = assign12770_body17_e15883_d_n2;
            locals.var_t7__blk340_dn6 = assign12770_body17_e15883_d_n6;
            locals.var_t7__blk340_dn7 = assign12770_body17_e15883_d_n7;
            locals.var_t7__blk340_dn10 = assign12770_body17_e15883_d_n10;
            locals.var_t7__blk340_dn11 = assign12770_body17_e15883_d_n11;
            locals.var_t7__blk340_dn12 = assign12770_body17_e15883_d_n12;
            locals.var_t7__blk340_dn17 = assign12770_body17_e15883_d_n17;
            let (assign12770_body18_e15904, assign12770_body18_e15904_d_n0, assign12770_body18_e15904_d_n2, assign12770_body18_e15904_d_n6, assign12770_body18_e15904_d_n7, assign12770_body18_e15904_d_n10, assign12770_body18_e15904_d_n11, assign12770_body18_e15904_d_n12, assign12770_body18_e15904_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 == 0.0)) {
        let assign12770_body18_e15898: f64 = (-locals.var_q_fd_soi);
        let assign12770_body18_e15900: f64 = (assign12770_body18_e15898 - locals.var_t6__blk339);
        let assign12770_body18_e15902: f64 = (assign12770_body18_e15900 - locals.var_q_fd_dlt2);
        (assign12770_body18_e15902, (((-locals.var_q_fd_soi_dn0) - locals.var_t6__blk339_dn0) - locals.var_q_fd_dlt2_dn0), (((-locals.var_q_fd_soi_dn2) - locals.var_t6__blk339_dn2) - locals.var_q_fd_dlt2_dn2), (((-locals.var_q_fd_soi_dn6) - locals.var_t6__blk339_dn6) - locals.var_q_fd_dlt2_dn6), (((-locals.var_q_fd_soi_dn7) - locals.var_t6__blk339_dn7) - locals.var_q_fd_dlt2_dn7), (((-locals.var_q_fd_soi_dn10) - locals.var_t6__blk339_dn10) - locals.var_q_fd_dlt2_dn10), (((-locals.var_q_fd_soi_dn11) - locals.var_t6__blk339_dn11) - locals.var_q_fd_dlt2_dn11), (((-locals.var_q_fd_soi_dn12) - locals.var_t6__blk339_dn12) - locals.var_q_fd_dlt2_dn12), (((-locals.var_q_fd_soi_dn17) - locals.var_t6__blk339_dn17) - locals.var_q_fd_dlt2_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
            locals.var_tmf1 = assign12770_body18_e15904;
            locals.var_tmf1_dn0 = assign12770_body18_e15904_d_n0;
            locals.var_tmf1_dn2 = assign12770_body18_e15904_d_n2;
            locals.var_tmf1_dn6 = assign12770_body18_e15904_d_n6;
            locals.var_tmf1_dn7 = assign12770_body18_e15904_d_n7;
            locals.var_tmf1_dn10 = assign12770_body18_e15904_d_n10;
            locals.var_tmf1_dn11 = assign12770_body18_e15904_d_n11;
            locals.var_tmf1_dn12 = assign12770_body18_e15904_d_n12;
            locals.var_tmf1_dn17 = assign12770_body18_e15904_d_n17;
            let (assign12770_body19_e15925, assign12770_body19_e15925_d_n0, assign12770_body19_e15925_d_n2, assign12770_body19_e15925_d_n6, assign12770_body19_e15925_d_n7, assign12770_body19_e15925_d_n10, assign12770_body19_e15925_d_n11, assign12770_body19_e15925_d_n12, assign12770_body19_e15925_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 == 0.0)) {
        let assign12770_body19_e15920: f64 = (-locals.var_q_fd_soi);
        let assign12770_body19_e15921: f64 = (4.0 * assign12770_body19_e15920);
        let assign12770_body19_e15923: f64 = (assign12770_body19_e15921 * locals.var_q_fd_dlt2);
        (assign12770_body19_e15923, (((4.0 * (-locals.var_q_fd_soi_dn0)) * locals.var_q_fd_dlt2) + (assign12770_body19_e15921 * locals.var_q_fd_dlt2_dn0)), (((4.0 * (-locals.var_q_fd_soi_dn2)) * locals.var_q_fd_dlt2) + (assign12770_body19_e15921 * locals.var_q_fd_dlt2_dn2)), (((4.0 * (-locals.var_q_fd_soi_dn6)) * locals.var_q_fd_dlt2) + (assign12770_body19_e15921 * locals.var_q_fd_dlt2_dn6)), (((4.0 * (-locals.var_q_fd_soi_dn7)) * locals.var_q_fd_dlt2) + (assign12770_body19_e15921 * locals.var_q_fd_dlt2_dn7)), (((4.0 * (-locals.var_q_fd_soi_dn10)) * locals.var_q_fd_dlt2) + (assign12770_body19_e15921 * locals.var_q_fd_dlt2_dn10)), (((4.0 * (-locals.var_q_fd_soi_dn11)) * locals.var_q_fd_dlt2) + (assign12770_body19_e15921 * locals.var_q_fd_dlt2_dn11)), (((4.0 * (-locals.var_q_fd_soi_dn12)) * locals.var_q_fd_dlt2) + (assign12770_body19_e15921 * locals.var_q_fd_dlt2_dn12)), (((4.0 * (-locals.var_q_fd_soi_dn17)) * locals.var_q_fd_dlt2) + (assign12770_body19_e15921 * locals.var_q_fd_dlt2_dn17)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign12770_body19_e15925;
            locals.var_tmf2_dn0 = assign12770_body19_e15925_d_n0;
            locals.var_tmf2_dn2 = assign12770_body19_e15925_d_n2;
            locals.var_tmf2_dn6 = assign12770_body19_e15925_d_n6;
            locals.var_tmf2_dn7 = assign12770_body19_e15925_d_n7;
            locals.var_tmf2_dn10 = assign12770_body19_e15925_d_n10;
            locals.var_tmf2_dn11 = assign12770_body19_e15925_d_n11;
            locals.var_tmf2_dn12 = assign12770_body19_e15925_d_n12;
            locals.var_tmf2_dn17 = assign12770_body19_e15925_d_n17;
            let (assign12770_body20_e15947, assign12770_body20_e15947_d_n0, assign12770_body20_e15947_d_n2, assign12770_body20_e15947_d_n6, assign12770_body20_e15947_d_n7, assign12770_body20_e15947_d_n10, assign12770_body20_e15947_d_n11, assign12770_body20_e15947_d_n12, assign12770_body20_e15947_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 == 0.0)) {
        let (assign12770_body20_e15945, assign12770_body20_e15945_d_n0, assign12770_body20_e15945_d_n2, assign12770_body20_e15945_d_n6, assign12770_body20_e15945_d_n7, assign12770_body20_e15945_d_n10, assign12770_body20_e15945_d_n11, assign12770_body20_e15945_d_n12, assign12770_body20_e15945_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign12770_body20_e15944: f64 = (-locals.var_tmf2);
                (assign12770_body20_e15944, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign12770_body20_e15945, assign12770_body20_e15945_d_n0, assign12770_body20_e15945_d_n2, assign12770_body20_e15945_d_n6, assign12770_body20_e15945_d_n7, assign12770_body20_e15945_d_n10, assign12770_body20_e15945_d_n11, assign12770_body20_e15945_d_n12, assign12770_body20_e15945_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign12770_body20_e15947;
            locals.var_tmf2_dn0 = assign12770_body20_e15947_d_n0;
            locals.var_tmf2_dn2 = assign12770_body20_e15947_d_n2;
            locals.var_tmf2_dn6 = assign12770_body20_e15947_d_n6;
            locals.var_tmf2_dn7 = assign12770_body20_e15947_d_n7;
            locals.var_tmf2_dn10 = assign12770_body20_e15947_d_n10;
            locals.var_tmf2_dn11 = assign12770_body20_e15947_d_n11;
            locals.var_tmf2_dn12 = assign12770_body20_e15947_d_n12;
            locals.var_tmf2_dn17 = assign12770_body20_e15947_d_n17;
            let (assign12770_body21_e15968, assign12770_body21_e15968_d_n0, assign12770_body21_e15968_d_n2, assign12770_body21_e15968_d_n6, assign12770_body21_e15968_d_n7, assign12770_body21_e15968_d_n10, assign12770_body21_e15968_d_n11, assign12770_body21_e15968_d_n12, assign12770_body21_e15968_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 == 0.0)) {
        let assign12770_body21_e15963: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign12770_body21_e15965: f64 = (assign12770_body21_e15963 + locals.var_tmf2);
        let assign12770_body21_e15966: f64 = (assign12770_body21_e15965).sqrt();
        (assign12770_body21_e15966, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign12770_body21_e15966)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign12770_body21_e15966)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign12770_body21_e15966)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign12770_body21_e15966)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign12770_body21_e15966)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign12770_body21_e15966)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign12770_body21_e15966)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign12770_body21_e15966)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign12770_body21_e15968;
            locals.var_tmf2_dn0 = assign12770_body21_e15968_d_n0;
            locals.var_tmf2_dn2 = assign12770_body21_e15968_d_n2;
            locals.var_tmf2_dn6 = assign12770_body21_e15968_d_n6;
            locals.var_tmf2_dn7 = assign12770_body21_e15968_d_n7;
            locals.var_tmf2_dn10 = assign12770_body21_e15968_d_n10;
            locals.var_tmf2_dn11 = assign12770_body21_e15968_d_n11;
            locals.var_tmf2_dn12 = assign12770_body21_e15968_d_n12;
            locals.var_tmf2_dn17 = assign12770_body21_e15968_d_n17;
            let (assign12770_body22_e15990, assign12770_body22_e15990_d_n0, assign12770_body22_e15990_d_n2, assign12770_body22_e15990_d_n6, assign12770_body22_e15990_d_n7, assign12770_body22_e15990_d_n10, assign12770_body22_e15990_d_n11, assign12770_body22_e15990_d_n12, assign12770_body22_e15990_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 == 0.0)) {
        let assign12770_body22_e15986: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign12770_body22_e15987: f64 = (1.0 + assign12770_body22_e15986);
        let assign12770_body22_e15988: f64 = (0.5 * assign12770_body22_e15987);
        (assign12770_body22_e15988, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn17,)
    }
};
            locals.var_t8 = assign12770_body22_e15990;
            locals.var_t8_dn0 = assign12770_body22_e15990_d_n0;
            locals.var_t8_dn2 = assign12770_body22_e15990_d_n2;
            locals.var_t8_dn6 = assign12770_body22_e15990_d_n6;
            locals.var_t8_dn7 = assign12770_body22_e15990_d_n7;
            locals.var_t8_dn10 = assign12770_body22_e15990_d_n10;
            locals.var_t8_dn11 = assign12770_body22_e15990_d_n11;
            locals.var_t8_dn12 = assign12770_body22_e15990_d_n12;
            locals.var_t8_dn17 = assign12770_body22_e15990_d_n17;
            let (assign12770_body23_e16013, assign12770_body23_e16013_d_n0, assign12770_body23_e16013_d_n2, assign12770_body23_e16013_d_n6, assign12770_body23_e16013_d_n7, assign12770_body23_e16013_d_n10, assign12770_body23_e16013_d_n11, assign12770_body23_e16013_d_n12, assign12770_body23_e16013_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 == 0.0)) {
        let assign12770_body23_e16005: f64 = (-locals.var_q_fd_soi);
        let assign12770_body23_e16009: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign12770_body23_e16010: f64 = (0.5 * assign12770_body23_e16009);
        let assign12770_body23_e16011: f64 = (assign12770_body23_e16005 - assign12770_body23_e16010);
        (assign12770_body23_e16011, ((-locals.var_q_fd_soi_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((-locals.var_q_fd_soi_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((-locals.var_q_fd_soi_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((-locals.var_q_fd_soi_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((-locals.var_q_fd_soi_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((-locals.var_q_fd_soi_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((-locals.var_q_fd_soi_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((-locals.var_q_fd_soi_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_t6__blk339, locals.var_t6__blk339_dn0, locals.var_t6__blk339_dn2, locals.var_t6__blk339_dn6, locals.var_t6__blk339_dn7, locals.var_t6__blk339_dn10, locals.var_t6__blk339_dn11, locals.var_t6__blk339_dn12, locals.var_t6__blk339_dn17,)
    }
};
            locals.var_t6__blk339 = assign12770_body23_e16013;
            locals.var_t6__blk339_dn0 = assign12770_body23_e16013_d_n0;
            locals.var_t6__blk339_dn2 = assign12770_body23_e16013_d_n2;
            locals.var_t6__blk339_dn6 = assign12770_body23_e16013_d_n6;
            locals.var_t6__blk339_dn7 = assign12770_body23_e16013_d_n7;
            locals.var_t6__blk339_dn10 = assign12770_body23_e16013_d_n10;
            locals.var_t6__blk339_dn11 = assign12770_body23_e16013_d_n11;
            locals.var_t6__blk339_dn12 = assign12770_body23_e16013_d_n12;
            locals.var_t6__blk339_dn17 = assign12770_body23_e16013_d_n17;
            let (assign12770_body24_e16033, assign12770_body24_e16033_d_n0, assign12770_body24_e16033_d_n2, assign12770_body24_e16033_d_n6, assign12770_body24_e16033_d_n7, assign12770_body24_e16033_d_n10, assign12770_body24_e16033_d_n11, assign12770_body24_e16033_d_n12, assign12770_body24_e16033_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 == 0.0)) {
        let assign12770_body24_e16030: f64 = (locals.var_t5__blk338 * locals.var_t8);
        let assign12770_body24_e16031: f64 = (locals.var_t7__blk340 * assign12770_body24_e16030);
        (assign12770_body24_e16031, ((locals.var_t7__blk340_dn0 * assign12770_body24_e16030) + (locals.var_t7__blk340 * ((locals.var_t5__blk338_dn0 * locals.var_t8) + (locals.var_t5__blk338 * locals.var_t8_dn0)))), ((locals.var_t7__blk340_dn2 * assign12770_body24_e16030) + (locals.var_t7__blk340 * ((locals.var_t5__blk338_dn2 * locals.var_t8) + (locals.var_t5__blk338 * locals.var_t8_dn2)))), ((locals.var_t7__blk340_dn6 * assign12770_body24_e16030) + (locals.var_t7__blk340 * ((locals.var_t5__blk338_dn6 * locals.var_t8) + (locals.var_t5__blk338 * locals.var_t8_dn6)))), ((locals.var_t7__blk340_dn7 * assign12770_body24_e16030) + (locals.var_t7__blk340 * ((locals.var_t5__blk338_dn7 * locals.var_t8) + (locals.var_t5__blk338 * locals.var_t8_dn7)))), ((locals.var_t7__blk340_dn10 * assign12770_body24_e16030) + (locals.var_t7__blk340 * ((locals.var_t5__blk338_dn10 * locals.var_t8) + (locals.var_t5__blk338 * locals.var_t8_dn10)))), ((locals.var_t7__blk340_dn11 * assign12770_body24_e16030) + (locals.var_t7__blk340 * ((locals.var_t5__blk338_dn11 * locals.var_t8) + (locals.var_t5__blk338 * locals.var_t8_dn11)))), ((locals.var_t7__blk340_dn12 * assign12770_body24_e16030) + (locals.var_t7__blk340 * ((locals.var_t5__blk338_dn12 * locals.var_t8) + (locals.var_t5__blk338 * locals.var_t8_dn12)))), ((locals.var_t7__blk340_dn17 * assign12770_body24_e16030) + (locals.var_t7__blk340 * ((locals.var_t5__blk338_dn17 * locals.var_t8) + (locals.var_t5__blk338 * locals.var_t8_dn17)))),)
    } else {
        (locals.var_t7__blk340, locals.var_t7__blk340_dn0, locals.var_t7__blk340_dn2, locals.var_t7__blk340_dn6, locals.var_t7__blk340_dn7, locals.var_t7__blk340_dn10, locals.var_t7__blk340_dn11, locals.var_t7__blk340_dn12, locals.var_t7__blk340_dn17,)
    }
};
            locals.var_t7__blk340 = assign12770_body24_e16033;
            locals.var_t7__blk340_dn0 = assign12770_body24_e16033_d_n0;
            locals.var_t7__blk340_dn2 = assign12770_body24_e16033_d_n2;
            locals.var_t7__blk340_dn6 = assign12770_body24_e16033_d_n6;
            locals.var_t7__blk340_dn7 = assign12770_body24_e16033_d_n7;
            locals.var_t7__blk340_dn10 = assign12770_body24_e16033_d_n10;
            locals.var_t7__blk340_dn11 = assign12770_body24_e16033_d_n11;
            locals.var_t7__blk340_dn12 = assign12770_body24_e16033_d_n12;
            locals.var_t7__blk340_dn17 = assign12770_body24_e16033_d_n17;
            let (assign12770_body25_e16059, assign12770_body25_e16059_d_n0, assign12770_body25_e16059_d_n2, assign12770_body25_e16059_d_n6, assign12770_body25_e16059_d_n7, assign12770_body25_e16059_d_n10, assign12770_body25_e16059_d_n11, assign12770_body25_e16059_d_n12, assign12770_body25_e16059_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 == 0.0)) {
        let assign12770_body25_e16049: f64 = (locals.var_t6__blk339 * locals.var_t6__blk339);
        let assign12770_body25_e16051: f64 = (assign12770_body25_e16049 / 2.0);
        let assign12770_body25_e16053: f64 = (assign12770_body25_e16051 / 1.034943e-10);
        let assign12770_body25_e16055: f64 = (assign12770_body25_e16053 / 1.6021918e-19);
        let assign12770_body25_e16057: f64 = (assign12770_body25_e16055 / locals.var_uc_nsubs);
        (assign12770_body25_e16057, ((((((((locals.var_t6__blk339_dn0 * locals.var_t6__blk339) + (locals.var_t6__blk339 * locals.var_t6__blk339_dn0)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12770_body25_e16055 * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk339_dn2 * locals.var_t6__blk339) + (locals.var_t6__blk339 * locals.var_t6__blk339_dn2)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12770_body25_e16055 * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk339_dn6 * locals.var_t6__blk339) + (locals.var_t6__blk339 * locals.var_t6__blk339_dn6)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12770_body25_e16055 * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk339_dn7 * locals.var_t6__blk339) + (locals.var_t6__blk339 * locals.var_t6__blk339_dn7)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12770_body25_e16055 * locals.var_uc_nsubs_dn7)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk339_dn10 * locals.var_t6__blk339) + (locals.var_t6__blk339 * locals.var_t6__blk339_dn10)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12770_body25_e16055 * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk339_dn11 * locals.var_t6__blk339) + (locals.var_t6__blk339 * locals.var_t6__blk339_dn11)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12770_body25_e16055 * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk339_dn12 * locals.var_t6__blk339) + (locals.var_t6__blk339 * locals.var_t6__blk339_dn12)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12770_body25_e16055 * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk339_dn17 * locals.var_t6__blk339) + (locals.var_t6__blk339 * locals.var_t6__blk339_dn17)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12770_body25_e16055 * locals.var_uc_nsubs_dn17)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)),)
    } else {
        (locals.var_phi_b_dep, locals.var_phi_b_dep_dn0, locals.var_phi_b_dep_dn2, locals.var_phi_b_dep_dn6, locals.var_phi_b_dep_dn7, locals.var_phi_b_dep_dn10, locals.var_phi_b_dep_dn11, locals.var_phi_b_dep_dn12, locals.var_phi_b_dep_dn17,)
    }
};
            locals.var_phi_b_dep = assign12770_body25_e16059;
            locals.var_phi_b_dep_dn0 = assign12770_body25_e16059_d_n0;
            locals.var_phi_b_dep_dn2 = assign12770_body25_e16059_d_n2;
            locals.var_phi_b_dep_dn6 = assign12770_body25_e16059_d_n6;
            locals.var_phi_b_dep_dn7 = assign12770_body25_e16059_d_n7;
            locals.var_phi_b_dep_dn10 = assign12770_body25_e16059_d_n10;
            locals.var_phi_b_dep_dn11 = assign12770_body25_e16059_d_n11;
            locals.var_phi_b_dep_dn12 = assign12770_body25_e16059_d_n12;
            locals.var_phi_b_dep_dn17 = assign12770_body25_e16059_d_n17;
            let (assign12770_body26_e16081, assign12770_body26_e16081_d_n0, assign12770_body26_e16081_d_n2, assign12770_body26_e16081_d_n6, assign12770_body26_e16081_d_n7, assign12770_body26_e16081_d_n10, assign12770_body26_e16081_d_n11, assign12770_body26_e16081_d_n12, assign12770_body26_e16081_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 == 0.0)) {
        let assign12770_body26_e16075: f64 = (2.0 * locals.var_phi_b_dep);
        let assign12770_body26_e16077: f64 = (assign12770_body26_e16075 * locals.var_t7__blk340);
        let assign12770_body26_e16079: f64 = (assign12770_body26_e16077 / locals.var_t6__blk339);
        (assign12770_body26_e16079, ((((((2.0 * locals.var_phi_b_dep_dn0) * locals.var_t7__blk340) + (assign12770_body26_e16075 * locals.var_t7__blk340_dn0)) * locals.var_t6__blk339) - (assign12770_body26_e16077 * locals.var_t6__blk339_dn0)) / (locals.var_t6__blk339 * locals.var_t6__blk339)), ((((((2.0 * locals.var_phi_b_dep_dn2) * locals.var_t7__blk340) + (assign12770_body26_e16075 * locals.var_t7__blk340_dn2)) * locals.var_t6__blk339) - (assign12770_body26_e16077 * locals.var_t6__blk339_dn2)) / (locals.var_t6__blk339 * locals.var_t6__blk339)), ((((((2.0 * locals.var_phi_b_dep_dn6) * locals.var_t7__blk340) + (assign12770_body26_e16075 * locals.var_t7__blk340_dn6)) * locals.var_t6__blk339) - (assign12770_body26_e16077 * locals.var_t6__blk339_dn6)) / (locals.var_t6__blk339 * locals.var_t6__blk339)), ((((((2.0 * locals.var_phi_b_dep_dn7) * locals.var_t7__blk340) + (assign12770_body26_e16075 * locals.var_t7__blk340_dn7)) * locals.var_t6__blk339) - (assign12770_body26_e16077 * locals.var_t6__blk339_dn7)) / (locals.var_t6__blk339 * locals.var_t6__blk339)), ((((((2.0 * locals.var_phi_b_dep_dn10) * locals.var_t7__blk340) + (assign12770_body26_e16075 * locals.var_t7__blk340_dn10)) * locals.var_t6__blk339) - (assign12770_body26_e16077 * locals.var_t6__blk339_dn10)) / (locals.var_t6__blk339 * locals.var_t6__blk339)), ((((((2.0 * locals.var_phi_b_dep_dn11) * locals.var_t7__blk340) + (assign12770_body26_e16075 * locals.var_t7__blk340_dn11)) * locals.var_t6__blk339) - (assign12770_body26_e16077 * locals.var_t6__blk339_dn11)) / (locals.var_t6__blk339 * locals.var_t6__blk339)), ((((((2.0 * locals.var_phi_b_dep_dn12) * locals.var_t7__blk340) + (assign12770_body26_e16075 * locals.var_t7__blk340_dn12)) * locals.var_t6__blk339) - (assign12770_body26_e16077 * locals.var_t6__blk339_dn12)) / (locals.var_t6__blk339 * locals.var_t6__blk339)), ((((((2.0 * locals.var_phi_b_dep_dn17) * locals.var_t7__blk340) + (assign12770_body26_e16075 * locals.var_t7__blk340_dn17)) * locals.var_t6__blk339) - (assign12770_body26_e16077 * locals.var_t6__blk339_dn17)) / (locals.var_t6__blk339 * locals.var_t6__blk339)),)
    } else {
        (locals.var_phi_b_dep_dpsb, locals.var_phi_b_dep_dpsb_dn0, locals.var_phi_b_dep_dpsb_dn2, locals.var_phi_b_dep_dpsb_dn6, locals.var_phi_b_dep_dpsb_dn7, locals.var_phi_b_dep_dpsb_dn10, locals.var_phi_b_dep_dpsb_dn11, locals.var_phi_b_dep_dpsb_dn12, locals.var_phi_b_dep_dpsb_dn17,)
    }
};
            locals.var_phi_b_dep_dpsb = assign12770_body26_e16081;
            locals.var_phi_b_dep_dpsb_dn0 = assign12770_body26_e16081_d_n0;
            locals.var_phi_b_dep_dpsb_dn2 = assign12770_body26_e16081_d_n2;
            locals.var_phi_b_dep_dpsb_dn6 = assign12770_body26_e16081_d_n6;
            locals.var_phi_b_dep_dpsb_dn7 = assign12770_body26_e16081_d_n7;
            locals.var_phi_b_dep_dpsb_dn10 = assign12770_body26_e16081_d_n10;
            locals.var_phi_b_dep_dpsb_dn11 = assign12770_body26_e16081_d_n11;
            locals.var_phi_b_dep_dpsb_dn12 = assign12770_body26_e16081_d_n12;
            locals.var_phi_b_dep_dpsb_dn17 = assign12770_body26_e16081_d_n17;
            let (assign12770_body27_e16134, assign12770_body27_e16134_d_n0, assign12770_body27_e16134_d_n2, assign12770_body27_e16134_d_n6, assign12770_body27_e16134_d_n7, assign12770_body27_e16134_d_n10, assign12770_body27_e16134_d_n11, assign12770_body27_e16134_d_n12, assign12770_body27_e16134_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 == 0.0)) {
        let assign12770_body27_e16098: f64 = (locals.var_phi_sl_soi - locals.var_phi_sl_bulk);
        let assign12770_body27_e16101: f64 = (locals.var_t4__blk337 / locals.var_c_box);
        let assign12770_body27_e16102: f64 = (assign12770_body27_e16098 + assign12770_body27_e16101);
        let assign12770_body27_e16106: f64 = (locals.var_q_fd_soi / 2.0);
        let assign12770_body27_e16107: f64 = (locals.var_t4__blk337 + assign12770_body27_e16106);
        let assign12770_body27_e16109: f64 = (assign12770_body27_e16107 * locals.var_t_soi);
        let assign12770_body27_e16111: f64 = (assign12770_body27_e16109 / 1.034943e-10);
        let assign12770_body27_e16112: f64 = (assign12770_body27_e16102 + assign12770_body27_e16111);
        let assign12770_body27_e16114: f64 = (assign12770_body27_e16112 - locals.var_vbsbiz);
        let assign12770_body27_e16116: f64 = (assign12770_body27_e16114 + locals.var_phi_b_dep);
        let assign12770_body27_e16118: f64 = (-1.0);
        let assign12770_body27_e16121: f64 = (locals.var_t5__blk338 / locals.var_c_box);
        let assign12770_body27_e16122: f64 = (assign12770_body27_e16118 + assign12770_body27_e16121);
        let assign12770_body27_e16125: f64 = (locals.var_t5__blk338 * locals.var_t_soi);
        let assign12770_body27_e16127: f64 = (assign12770_body27_e16125 / 1.034943e-10);
        let assign12770_body27_e16128: f64 = (assign12770_body27_e16122 + assign12770_body27_e16127);
        let assign12770_body27_e16130: f64 = (assign12770_body27_e16128 + locals.var_phi_b_dep_dpsb);
        let assign12770_body27_e16131: f64 = (assign12770_body27_e16116 / assign12770_body27_e16130);
        let assign12770_body27_e16132: f64 = (locals.var_phi_sl_bulk - assign12770_body27_e16131);
        (assign12770_body27_e16132, (locals.var_phi_sl_bulk_dn0 - ((((((((locals.var_phi_sl_soi_dn0 - locals.var_phi_sl_bulk_dn0) + (locals.var_t4__blk337_dn0 / locals.var_c_box)) + (((locals.var_t4__blk337_dn0 + (locals.var_q_fd_soi_dn0 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn0) + locals.var_phi_b_dep_dn0) * assign12770_body27_e16130) - (assign12770_body27_e16116 * (((locals.var_t5__blk338_dn0 / locals.var_c_box) + ((locals.var_t5__blk338_dn0 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn0))) / (assign12770_body27_e16130 * assign12770_body27_e16130))), (locals.var_phi_sl_bulk_dn2 - ((((((((locals.var_phi_sl_soi_dn2 - locals.var_phi_sl_bulk_dn2) + (locals.var_t4__blk337_dn2 / locals.var_c_box)) + (((locals.var_t4__blk337_dn2 + (locals.var_q_fd_soi_dn2 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn2) + locals.var_phi_b_dep_dn2) * assign12770_body27_e16130) - (assign12770_body27_e16116 * (((locals.var_t5__blk338_dn2 / locals.var_c_box) + ((locals.var_t5__blk338_dn2 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn2))) / (assign12770_body27_e16130 * assign12770_body27_e16130))), (locals.var_phi_sl_bulk_dn6 - ((((((((locals.var_phi_sl_soi_dn6 - locals.var_phi_sl_bulk_dn6) + (locals.var_t4__blk337_dn6 / locals.var_c_box)) + (((locals.var_t4__blk337_dn6 + (locals.var_q_fd_soi_dn6 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn6) + locals.var_phi_b_dep_dn6) * assign12770_body27_e16130) - (assign12770_body27_e16116 * (((locals.var_t5__blk338_dn6 / locals.var_c_box) + ((locals.var_t5__blk338_dn6 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn6))) / (assign12770_body27_e16130 * assign12770_body27_e16130))), (locals.var_phi_sl_bulk_dn7 - ((((((((locals.var_phi_sl_soi_dn7 - locals.var_phi_sl_bulk_dn7) + (locals.var_t4__blk337_dn7 / locals.var_c_box)) + (((locals.var_t4__blk337_dn7 + (locals.var_q_fd_soi_dn7 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn7) + locals.var_phi_b_dep_dn7) * assign12770_body27_e16130) - (assign12770_body27_e16116 * (((locals.var_t5__blk338_dn7 / locals.var_c_box) + ((locals.var_t5__blk338_dn7 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn7))) / (assign12770_body27_e16130 * assign12770_body27_e16130))), (locals.var_phi_sl_bulk_dn10 - ((((((((locals.var_phi_sl_soi_dn10 - locals.var_phi_sl_bulk_dn10) + (locals.var_t4__blk337_dn10 / locals.var_c_box)) + (((locals.var_t4__blk337_dn10 + (locals.var_q_fd_soi_dn10 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn10) + locals.var_phi_b_dep_dn10) * assign12770_body27_e16130) - (assign12770_body27_e16116 * (((locals.var_t5__blk338_dn10 / locals.var_c_box) + ((locals.var_t5__blk338_dn10 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn10))) / (assign12770_body27_e16130 * assign12770_body27_e16130))), (locals.var_phi_sl_bulk_dn11 - ((((((((locals.var_phi_sl_soi_dn11 - locals.var_phi_sl_bulk_dn11) + (locals.var_t4__blk337_dn11 / locals.var_c_box)) + (((locals.var_t4__blk337_dn11 + (locals.var_q_fd_soi_dn11 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn11) + locals.var_phi_b_dep_dn11) * assign12770_body27_e16130) - (assign12770_body27_e16116 * (((locals.var_t5__blk338_dn11 / locals.var_c_box) + ((locals.var_t5__blk338_dn11 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn11))) / (assign12770_body27_e16130 * assign12770_body27_e16130))), (locals.var_phi_sl_bulk_dn12 - ((((((((locals.var_phi_sl_soi_dn12 - locals.var_phi_sl_bulk_dn12) + (locals.var_t4__blk337_dn12 / locals.var_c_box)) + (((locals.var_t4__blk337_dn12 + (locals.var_q_fd_soi_dn12 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn12) + locals.var_phi_b_dep_dn12) * assign12770_body27_e16130) - (assign12770_body27_e16116 * (((locals.var_t5__blk338_dn12 / locals.var_c_box) + ((locals.var_t5__blk338_dn12 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn12))) / (assign12770_body27_e16130 * assign12770_body27_e16130))), (locals.var_phi_sl_bulk_dn17 - ((((((((locals.var_phi_sl_soi_dn17 - locals.var_phi_sl_bulk_dn17) + (locals.var_t4__blk337_dn17 / locals.var_c_box)) + (((locals.var_t4__blk337_dn17 + (locals.var_q_fd_soi_dn17 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn17) + locals.var_phi_b_dep_dn17) * assign12770_body27_e16130) - (assign12770_body27_e16116 * (((locals.var_t5__blk338_dn17 / locals.var_c_box) + ((locals.var_t5__blk338_dn17 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn17))) / (assign12770_body27_e16130 * assign12770_body27_e16130))),)
    } else {
        (locals.var_t6__blk339, locals.var_t6__blk339_dn0, locals.var_t6__blk339_dn2, locals.var_t6__blk339_dn6, locals.var_t6__blk339_dn7, locals.var_t6__blk339_dn10, locals.var_t6__blk339_dn11, locals.var_t6__blk339_dn12, locals.var_t6__blk339_dn17,)
    }
};
            locals.var_t6__blk339 = assign12770_body27_e16134;
            locals.var_t6__blk339_dn0 = assign12770_body27_e16134_d_n0;
            locals.var_t6__blk339_dn2 = assign12770_body27_e16134_d_n2;
            locals.var_t6__blk339_dn6 = assign12770_body27_e16134_d_n6;
            locals.var_t6__blk339_dn7 = assign12770_body27_e16134_d_n7;
            locals.var_t6__blk339_dn10 = assign12770_body27_e16134_d_n10;
            locals.var_t6__blk339_dn11 = assign12770_body27_e16134_d_n11;
            locals.var_t6__blk339_dn12 = assign12770_body27_e16134_d_n12;
            locals.var_t6__blk339_dn17 = assign12770_body27_e16134_d_n17;
            let assign12770_body28_e16137: f64 = (locals.var_t6__blk339 - locals.var_phi_sl_bulk);
            let assign12770_body28_e16138: f64 = (assign12770_body28_e16137).abs();
            let assign12770_body28_e16140: f64 = if assign12770_body28_e16138 < 5e-12 { 1.0 } else { 0.0 };
            locals.var_guard350 = assign12770_body28_e16140;
            let (assign12770_body29_e16158,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 == 0.0)) && (locals.var_guard350 != 0.0)) {
        (locals.var_lp_sl_max,)
    } else {
        (locals.var_lp_sl,)
    }
};
            locals.var_lp_sl = assign12770_body29_e16158;
            let (assign12770_body30_e16174, assign12770_body30_e16174_d_n0, assign12770_body30_e16174_d_n2, assign12770_body30_e16174_d_n6, assign12770_body30_e16174_d_n7, assign12770_body30_e16174_d_n10, assign12770_body30_e16174_d_n11, assign12770_body30_e16174_d_n12, assign12770_body30_e16174_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 == 0.0)) {
        (locals.var_t6__blk339, locals.var_t6__blk339_dn0, locals.var_t6__blk339_dn2, locals.var_t6__blk339_dn6, locals.var_t6__blk339_dn7, locals.var_t6__blk339_dn10, locals.var_t6__blk339_dn11, locals.var_t6__blk339_dn12, locals.var_t6__blk339_dn17,)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
            locals.var_phi_sl_bulk = assign12770_body30_e16174;
            locals.var_phi_sl_bulk_dn0 = assign12770_body30_e16174_d_n0;
            locals.var_phi_sl_bulk_dn2 = assign12770_body30_e16174_d_n2;
            locals.var_phi_sl_bulk_dn6 = assign12770_body30_e16174_d_n6;
            locals.var_phi_sl_bulk_dn7 = assign12770_body30_e16174_d_n7;
            locals.var_phi_sl_bulk_dn10 = assign12770_body30_e16174_d_n10;
            locals.var_phi_sl_bulk_dn11 = assign12770_body30_e16174_d_n11;
            locals.var_phi_sl_bulk_dn12 = assign12770_body30_e16174_d_n12;
            locals.var_phi_sl_bulk_dn17 = assign12770_body30_e16174_d_n17;
            let (assign12770_body31_e16190, assign12770_body31_e16190_d_n0, assign12770_body31_e16190_d_n2, assign12770_body31_e16190_d_n6, assign12770_body31_e16190_d_n7, assign12770_body31_e16190_d_n10, assign12770_body31_e16190_d_n11, assign12770_body31_e16190_d_n12, assign12770_body31_e16190_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 == 0.0)) {
        (locals.var_t4__blk337, locals.var_t4__blk337_dn0, locals.var_t4__blk337_dn2, locals.var_t4__blk337_dn6, locals.var_t4__blk337_dn7, locals.var_t4__blk337_dn10, locals.var_t4__blk337_dn11, locals.var_t4__blk337_dn12, locals.var_t4__blk337_dn17,)
    } else {
        (locals.var_q_sl_bulk, locals.var_q_sl_bulk_dn0, locals.var_q_sl_bulk_dn2, locals.var_q_sl_bulk_dn6, locals.var_q_sl_bulk_dn7, locals.var_q_sl_bulk_dn10, locals.var_q_sl_bulk_dn11, locals.var_q_sl_bulk_dn12, locals.var_q_sl_bulk_dn17,)
    }
};
            locals.var_q_sl_bulk = assign12770_body31_e16190;
            locals.var_q_sl_bulk_dn0 = assign12770_body31_e16190_d_n0;
            locals.var_q_sl_bulk_dn2 = assign12770_body31_e16190_d_n2;
            locals.var_q_sl_bulk_dn6 = assign12770_body31_e16190_d_n6;
            locals.var_q_sl_bulk_dn7 = assign12770_body31_e16190_d_n7;
            locals.var_q_sl_bulk_dn10 = assign12770_body31_e16190_d_n10;
            locals.var_q_sl_bulk_dn11 = assign12770_body31_e16190_d_n11;
            locals.var_q_sl_bulk_dn12 = assign12770_body31_e16190_d_n12;
            locals.var_q_sl_bulk_dn17 = assign12770_body31_e16190_d_n17;
            let (assign12770_body32_e16208,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 == 0.0)) {
        let assign12770_body32_e16206: f64 = (locals.var_lp_sl + 1.0);
        (assign12770_body32_e16206,)
    } else {
        (locals.var_lp_sl,)
    }
};
            locals.var_lp_sl = assign12770_body32_e16208;
        }

    }

    pub(super) fn stamp_transient_block_40(
        locals: &mut StampLocals,
    ) {
        let (assign12780_e16226, assign12780_e16226_d_n0, assign12780_e16226_d_n2, assign12780_e16226_d_n6, assign12780_e16226_d_n7, assign12780_e16226_d_n10, assign12780_e16226_d_n11, assign12780_e16226_d_n12, assign12780_e16226_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 == 0.0)) {
        let assign12780_e16224: f64 = (locals.var_vbsbiz + locals.var_phi_sl_bulk);
        (assign12780_e16224, (locals.var_vbsbiz_dn0 + locals.var_phi_sl_bulk_dn0), (locals.var_vbsbiz_dn2 + locals.var_phi_sl_bulk_dn2), (locals.var_vbsbiz_dn6 + locals.var_phi_sl_bulk_dn6), (locals.var_vbsbiz_dn7 + locals.var_phi_sl_bulk_dn7), (locals.var_vbsbiz_dn10 + locals.var_phi_sl_bulk_dn10), (locals.var_vbsbiz_dn11 + locals.var_phi_sl_bulk_dn11), (locals.var_vbsbiz_dn12 + locals.var_phi_sl_bulk_dn12), (locals.var_vbsbiz_dn17 + locals.var_phi_sl_bulk_dn17),)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
        locals.var_phi_sl_bulk = assign12780_e16226;
        locals.var_phi_sl_bulk_dn0 = assign12780_e16226_d_n0;
        locals.var_phi_sl_bulk_dn2 = assign12780_e16226_d_n2;
        locals.var_phi_sl_bulk_dn6 = assign12780_e16226_d_n6;
        locals.var_phi_sl_bulk_dn7 = assign12780_e16226_d_n7;
        locals.var_phi_sl_bulk_dn10 = assign12780_e16226_d_n10;
        locals.var_phi_sl_bulk_dn11 = assign12780_e16226_d_n11;
        locals.var_phi_sl_bulk_dn12 = assign12780_e16226_d_n12;
        locals.var_phi_sl_bulk_dn17 = assign12780_e16226_d_n17;

        let (assign12790_e16246, assign12790_e16246_d_n0, assign12790_e16246_d_n2, assign12790_e16246_d_n6, assign12790_e16246_d_n7, assign12790_e16246_d_n10, assign12790_e16246_d_n11, assign12790_e16246_d_n12, assign12790_e16246_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard342 == 0.0)) {
        let assign12790_e16243: f64 = (locals.var_q_sl_bulk / locals.var_c_box);
        let assign12790_e16244: f64 = (locals.var_phi_sl_bulk - assign12790_e16243);
        (assign12790_e16244, (locals.var_phi_sl_bulk_dn0 - (locals.var_q_sl_bulk_dn0 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn2 - (locals.var_q_sl_bulk_dn2 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn6 - (locals.var_q_sl_bulk_dn6 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn7 - (locals.var_q_sl_bulk_dn7 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn10 - (locals.var_q_sl_bulk_dn10 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn11 - (locals.var_q_sl_bulk_dn11 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn12 - (locals.var_q_sl_bulk_dn12 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn17 - (locals.var_q_sl_bulk_dn17 / locals.var_c_box)),)
    } else {
        (locals.var_phi_bl_soi, locals.var_phi_bl_soi_dn0, locals.var_phi_bl_soi_dn2, locals.var_phi_bl_soi_dn6, locals.var_phi_bl_soi_dn7, locals.var_phi_bl_soi_dn10, locals.var_phi_bl_soi_dn11, locals.var_phi_bl_soi_dn12, locals.var_phi_bl_soi_dn17,)
    }
};
        locals.var_phi_bl_soi = assign12790_e16246;
        locals.var_phi_bl_soi_dn0 = assign12790_e16246_d_n0;
        locals.var_phi_bl_soi_dn2 = assign12790_e16246_d_n2;
        locals.var_phi_bl_soi_dn6 = assign12790_e16246_d_n6;
        locals.var_phi_bl_soi_dn7 = assign12790_e16246_d_n7;
        locals.var_phi_bl_soi_dn10 = assign12790_e16246_d_n10;
        locals.var_phi_bl_soi_dn11 = assign12790_e16246_d_n11;
        locals.var_phi_bl_soi_dn12 = assign12790_e16246_d_n12;
        locals.var_phi_bl_soi_dn17 = assign12790_e16246_d_n17;

        let assign12800_e16249: f64 = if locals.var_phi_bl_soi < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard351 = assign12800_e16249;

        let (assign12810_e16264, assign12810_e16264_d_n0, assign12810_e16264_d_n2, assign12810_e16264_d_n6, assign12810_e16264_d_n7, assign12810_e16264_d_n10, assign12810_e16264_d_n11, assign12810_e16264_d_n12, assign12810_e16264_d_n17,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 == 0.0)) && (locals.var_guard351 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_bl_soi, locals.var_phi_bl_soi_dn0, locals.var_phi_bl_soi_dn2, locals.var_phi_bl_soi_dn6, locals.var_phi_bl_soi_dn7, locals.var_phi_bl_soi_dn10, locals.var_phi_bl_soi_dn11, locals.var_phi_bl_soi_dn12, locals.var_phi_bl_soi_dn17,)
    }
};
        locals.var_phi_bl_soi = assign12810_e16264;
        locals.var_phi_bl_soi_dn0 = assign12810_e16264_d_n0;
        locals.var_phi_bl_soi_dn2 = assign12810_e16264_d_n2;
        locals.var_phi_bl_soi_dn6 = assign12810_e16264_d_n6;
        locals.var_phi_bl_soi_dn7 = assign12810_e16264_d_n7;
        locals.var_phi_bl_soi_dn10 = assign12810_e16264_d_n10;
        locals.var_phi_bl_soi_dn11 = assign12810_e16264_d_n11;
        locals.var_phi_bl_soi_dn12 = assign12810_e16264_d_n12;
        locals.var_phi_bl_soi_dn17 = assign12810_e16264_d_n17;

        let assign12820_e16267: f64 = if locals.var_phi_s0_soi < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard387 = assign12820_e16267;

        let (assign12830_e16276, assign12830_e16276_d_n0, assign12830_e16276_d_n2, assign12830_e16276_d_n6, assign12830_e16276_d_n7, assign12830_e16276_d_n10, assign12830_e16276_d_n11, assign12830_e16276_d_n12, assign12830_e16276_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard387 != 0.0)) {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
        locals.var_phi_sl_soi = assign12830_e16276;
        locals.var_phi_sl_soi_dn0 = assign12830_e16276_d_n0;
        locals.var_phi_sl_soi_dn2 = assign12830_e16276_d_n2;
        locals.var_phi_sl_soi_dn6 = assign12830_e16276_d_n6;
        locals.var_phi_sl_soi_dn7 = assign12830_e16276_d_n7;
        locals.var_phi_sl_soi_dn10 = assign12830_e16276_d_n10;
        locals.var_phi_sl_soi_dn11 = assign12830_e16276_d_n11;
        locals.var_phi_sl_soi_dn12 = assign12830_e16276_d_n12;
        locals.var_phi_sl_soi_dn17 = assign12830_e16276_d_n17;

        let assign12840_e16279: f64 = if locals.var_phi_bl_soi < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard388 = assign12840_e16279;

        let (assign12850_e16296, assign12850_e16296_d_n0, assign12850_e16296_d_n2, assign12850_e16296_d_n6, assign12850_e16296_d_n7, assign12850_e16296_d_n10, assign12850_e16296_d_n11, assign12850_e16296_d_n12, assign12850_e16296_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard388 != 0.0)) {
        let assign12850_e16290: f64 = (0.5 * locals.var_q_fd_soi);
        let assign12850_e16292: f64 = (assign12850_e16290 + locals.var_q_s0_bulk);
        let assign12850_e16293: f64 = (locals.var_c_soi_inv__blk115 * assign12850_e16292);
        let assign12850_e16294: f64 = (locals.var_phi_sl_soi + assign12850_e16293);
        (assign12850_e16294, (locals.var_phi_sl_soi_dn0 + (locals.var_c_soi_inv__blk115 * ((0.5 * locals.var_q_fd_soi_dn0) + locals.var_q_s0_bulk_dn0))), (locals.var_phi_sl_soi_dn2 + (locals.var_c_soi_inv__blk115 * ((0.5 * locals.var_q_fd_soi_dn2) + locals.var_q_s0_bulk_dn2))), (locals.var_phi_sl_soi_dn6 + (locals.var_c_soi_inv__blk115 * ((0.5 * locals.var_q_fd_soi_dn6) + locals.var_q_s0_bulk_dn6))), (locals.var_phi_sl_soi_dn7 + (locals.var_c_soi_inv__blk115 * ((0.5 * locals.var_q_fd_soi_dn7) + locals.var_q_s0_bulk_dn7))), (locals.var_phi_sl_soi_dn10 + (locals.var_c_soi_inv__blk115 * ((0.5 * locals.var_q_fd_soi_dn10) + locals.var_q_s0_bulk_dn10))), (locals.var_phi_sl_soi_dn11 + (locals.var_c_soi_inv__blk115 * ((0.5 * locals.var_q_fd_soi_dn11) + locals.var_q_s0_bulk_dn11))), (locals.var_phi_sl_soi_dn12 + (locals.var_c_soi_inv__blk115 * ((0.5 * locals.var_q_fd_soi_dn12) + locals.var_q_s0_bulk_dn12))), (locals.var_phi_sl_soi_dn17 + (locals.var_c_soi_inv__blk115 * ((0.5 * locals.var_q_fd_soi_dn17) + locals.var_q_s0_bulk_dn17))),)
    } else {
        (locals.var_phi_bl_soi, locals.var_phi_bl_soi_dn0, locals.var_phi_bl_soi_dn2, locals.var_phi_bl_soi_dn6, locals.var_phi_bl_soi_dn7, locals.var_phi_bl_soi_dn10, locals.var_phi_bl_soi_dn11, locals.var_phi_bl_soi_dn12, locals.var_phi_bl_soi_dn17,)
    }
};
        locals.var_phi_bl_soi = assign12850_e16296;
        locals.var_phi_bl_soi_dn0 = assign12850_e16296_d_n0;
        locals.var_phi_bl_soi_dn2 = assign12850_e16296_d_n2;
        locals.var_phi_bl_soi_dn6 = assign12850_e16296_d_n6;
        locals.var_phi_bl_soi_dn7 = assign12850_e16296_d_n7;
        locals.var_phi_bl_soi_dn10 = assign12850_e16296_d_n10;
        locals.var_phi_bl_soi_dn11 = assign12850_e16296_d_n11;
        locals.var_phi_bl_soi_dn12 = assign12850_e16296_d_n12;
        locals.var_phi_bl_soi_dn17 = assign12850_e16296_d_n17;

        let (assign12860_e16303, assign12860_e16303_d_n0, assign12860_e16303_d_n2, assign12860_e16303_d_n6, assign12860_e16303_d_n7, assign12860_e16303_d_n10, assign12860_e16303_d_n11, assign12860_e16303_d_n12, assign12860_e16303_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    } else {
        (locals.var_phi_sl_soi_ini, locals.var_phi_sl_soi_ini_dn0, locals.var_phi_sl_soi_ini_dn2, locals.var_phi_sl_soi_ini_dn6, locals.var_phi_sl_soi_ini_dn7, locals.var_phi_sl_soi_ini_dn10, locals.var_phi_sl_soi_ini_dn11, locals.var_phi_sl_soi_ini_dn12, locals.var_phi_sl_soi_ini_dn17,)
    }
};
        locals.var_phi_sl_soi_ini = assign12860_e16303;
        locals.var_phi_sl_soi_ini_dn0 = assign12860_e16303_d_n0;
        locals.var_phi_sl_soi_ini_dn2 = assign12860_e16303_d_n2;
        locals.var_phi_sl_soi_ini_dn6 = assign12860_e16303_d_n6;
        locals.var_phi_sl_soi_ini_dn7 = assign12860_e16303_d_n7;
        locals.var_phi_sl_soi_ini_dn10 = assign12860_e16303_d_n10;
        locals.var_phi_sl_soi_ini_dn11 = assign12860_e16303_d_n11;
        locals.var_phi_sl_soi_ini_dn12 = assign12860_e16303_d_n12;
        locals.var_phi_sl_soi_ini_dn17 = assign12860_e16303_d_n17;

        let (assign12870_e16310, assign12870_e16310_d_n0, assign12870_e16310_d_n2, assign12870_e16310_d_n6, assign12870_e16310_d_n7, assign12870_e16310_d_n10, assign12870_e16310_d_n11, assign12870_e16310_d_n12, assign12870_e16310_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        (locals.var_phi_bl_soi, locals.var_phi_bl_soi_dn0, locals.var_phi_bl_soi_dn2, locals.var_phi_bl_soi_dn6, locals.var_phi_bl_soi_dn7, locals.var_phi_bl_soi_dn10, locals.var_phi_bl_soi_dn11, locals.var_phi_bl_soi_dn12, locals.var_phi_bl_soi_dn17,)
    } else {
        (locals.var_phi_bl_soi_ini, locals.var_phi_bl_soi_ini_dn0, locals.var_phi_bl_soi_ini_dn2, locals.var_phi_bl_soi_ini_dn6, locals.var_phi_bl_soi_ini_dn7, locals.var_phi_bl_soi_ini_dn10, locals.var_phi_bl_soi_ini_dn11, locals.var_phi_bl_soi_ini_dn12, locals.var_phi_bl_soi_ini_dn17,)
    }
};
        locals.var_phi_bl_soi_ini = assign12870_e16310;
        locals.var_phi_bl_soi_ini_dn0 = assign12870_e16310_d_n0;
        locals.var_phi_bl_soi_ini_dn2 = assign12870_e16310_d_n2;
        locals.var_phi_bl_soi_ini_dn6 = assign12870_e16310_d_n6;
        locals.var_phi_bl_soi_ini_dn7 = assign12870_e16310_d_n7;
        locals.var_phi_bl_soi_ini_dn10 = assign12870_e16310_d_n10;
        locals.var_phi_bl_soi_ini_dn11 = assign12870_e16310_d_n11;
        locals.var_phi_bl_soi_ini_dn12 = assign12870_e16310_d_n12;
        locals.var_phi_bl_soi_ini_dn17 = assign12870_e16310_d_n17;

        let (assign12880_e16317, assign12880_e16317_d_n0, assign12880_e16317_d_n2, assign12880_e16317_d_n6, assign12880_e16317_d_n7, assign12880_e16317_d_n10, assign12880_e16317_d_n11, assign12880_e16317_d_n12, assign12880_e16317_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    } else {
        (locals.var_phi_sl_bulk_ini, locals.var_phi_sl_bulk_ini_dn0, locals.var_phi_sl_bulk_ini_dn2, locals.var_phi_sl_bulk_ini_dn6, locals.var_phi_sl_bulk_ini_dn7, locals.var_phi_sl_bulk_ini_dn10, locals.var_phi_sl_bulk_ini_dn11, locals.var_phi_sl_bulk_ini_dn12, locals.var_phi_sl_bulk_ini_dn17,)
    }
};
        locals.var_phi_sl_bulk_ini = assign12880_e16317;
        locals.var_phi_sl_bulk_ini_dn0 = assign12880_e16317_d_n0;
        locals.var_phi_sl_bulk_ini_dn2 = assign12880_e16317_d_n2;
        locals.var_phi_sl_bulk_ini_dn6 = assign12880_e16317_d_n6;
        locals.var_phi_sl_bulk_ini_dn7 = assign12880_e16317_d_n7;
        locals.var_phi_sl_bulk_ini_dn10 = assign12880_e16317_d_n10;
        locals.var_phi_sl_bulk_ini_dn11 = assign12880_e16317_d_n11;
        locals.var_phi_sl_bulk_ini_dn12 = assign12880_e16317_d_n12;
        locals.var_phi_sl_bulk_ini_dn17 = assign12880_e16317_d_n17;

        let (assign12890_e16324,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign12890_e16324;

        let (assign12900_e16331,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_brk8,)
    }
};
        locals.var_flg_brk8 = assign12900_e16331;

        let (assign12910_e16338,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_sl,)
    }
};
        locals.var_lp_sl = assign12910_e16338;

    }

    pub(super) fn stamp_transient_block_41(
        locals: &mut StampLocals,
    ) {
        let mut assign12920_loop_guard: usize = 0;
        while {
            let assign12920_cond_e16346: f64 = if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_lp_sl <= locals.var_lp_sl_max)) { 1.0 } else { 0.0 };
            assign12920_cond_e16346 != 0.0
        } {
            assign12920_loop_guard += 1;
            assert!(assign12920_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign12920_body0_e16355, assign12920_body0_e16355_d_n0, assign12920_body0_e16355_d_n2, assign12920_body0_e16355_d_n6, assign12920_body0_e16355_d_n7, assign12920_body0_e16355_d_n10, assign12920_body0_e16355_d_n11, assign12920_body0_e16355_d_n12, assign12920_body0_e16355_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign12920_body0_e16353: f64 = (locals.var_phi_sl_bulk - locals.var_vbsbiz);
        (assign12920_body0_e16353, (locals.var_phi_sl_bulk_dn0 - locals.var_vbsbiz_dn0), (locals.var_phi_sl_bulk_dn2 - locals.var_vbsbiz_dn2), (locals.var_phi_sl_bulk_dn6 - locals.var_vbsbiz_dn6), (locals.var_phi_sl_bulk_dn7 - locals.var_vbsbiz_dn7), (locals.var_phi_sl_bulk_dn10 - locals.var_vbsbiz_dn10), (locals.var_phi_sl_bulk_dn11 - locals.var_vbsbiz_dn11), (locals.var_phi_sl_bulk_dn12 - locals.var_vbsbiz_dn12), (locals.var_phi_sl_bulk_dn17 - locals.var_vbsbiz_dn17),)
    } else {
        (locals.var_t1__blk353, locals.var_t1__blk353_dn0, locals.var_t1__blk353_dn2, locals.var_t1__blk353_dn6, locals.var_t1__blk353_dn7, locals.var_t1__blk353_dn10, locals.var_t1__blk353_dn11, locals.var_t1__blk353_dn12, locals.var_t1__blk353_dn17,)
    }
};
            locals.var_t1__blk353 = assign12920_body0_e16355;
            locals.var_t1__blk353_dn0 = assign12920_body0_e16355_d_n0;
            locals.var_t1__blk353_dn2 = assign12920_body0_e16355_d_n2;
            locals.var_t1__blk353_dn6 = assign12920_body0_e16355_d_n6;
            locals.var_t1__blk353_dn7 = assign12920_body0_e16355_d_n7;
            locals.var_t1__blk353_dn10 = assign12920_body0_e16355_d_n10;
            locals.var_t1__blk353_dn11 = assign12920_body0_e16355_d_n11;
            locals.var_t1__blk353_dn12 = assign12920_body0_e16355_d_n12;
            locals.var_t1__blk353_dn17 = assign12920_body0_e16355_d_n17;
            let (assign12920_body1_e16364, assign12920_body1_e16364_d_n0, assign12920_body1_e16364_d_n2, assign12920_body1_e16364_d_n6, assign12920_body1_e16364_d_n7, assign12920_body1_e16364_d_n10, assign12920_body1_e16364_d_n11, assign12920_body1_e16364_d_n12, assign12920_body1_e16364_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign12920_body1_e16362: f64 = (locals.var_beta * locals.var_t1__blk353);
        (assign12920_body1_e16362, (locals.var_beta * locals.var_t1__blk353_dn0), (locals.var_beta * locals.var_t1__blk353_dn2), (locals.var_beta * locals.var_t1__blk353_dn6), (locals.var_beta * locals.var_t1__blk353_dn7), ((locals.var_beta_dn10 * locals.var_t1__blk353) + (locals.var_beta * locals.var_t1__blk353_dn10)), (locals.var_beta * locals.var_t1__blk353_dn11), (locals.var_beta * locals.var_t1__blk353_dn12), (locals.var_beta * locals.var_t1__blk353_dn17),)
    } else {
        (locals.var_el, locals.var_el_dn0, locals.var_el_dn2, locals.var_el_dn6, locals.var_el_dn7, locals.var_el_dn10, locals.var_el_dn11, locals.var_el_dn12, locals.var_el_dn17,)
    }
};
            locals.var_el = assign12920_body1_e16364;
            locals.var_el_dn0 = assign12920_body1_e16364_d_n0;
            locals.var_el_dn2 = assign12920_body1_e16364_d_n2;
            locals.var_el_dn6 = assign12920_body1_e16364_d_n6;
            locals.var_el_dn7 = assign12920_body1_e16364_d_n7;
            locals.var_el_dn10 = assign12920_body1_e16364_d_n10;
            locals.var_el_dn11 = assign12920_body1_e16364_d_n11;
            locals.var_el_dn12 = assign12920_body1_e16364_d_n12;
            locals.var_el_dn17 = assign12920_body1_e16364_d_n17;
            let (assign12920_body2_e16373, assign12920_body2_e16373_d_n0, assign12920_body2_e16373_d_n2, assign12920_body2_e16373_d_n6, assign12920_body2_e16373_d_n7, assign12920_body2_e16373_d_n10, assign12920_body2_e16373_d_n11, assign12920_body2_e16373_d_n12, assign12920_body2_e16373_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign12920_body2_e16370: f64 = (-locals.var_el);
        let assign12920_body2_e16371: f64 = (assign12920_body2_e16370).exp();
        (assign12920_body2_e16371, (assign12920_body2_e16371 * (-locals.var_el_dn0)), (assign12920_body2_e16371 * (-locals.var_el_dn2)), (assign12920_body2_e16371 * (-locals.var_el_dn6)), (assign12920_body2_e16371 * (-locals.var_el_dn7)), (assign12920_body2_e16371 * (-locals.var_el_dn10)), (assign12920_body2_e16371 * (-locals.var_el_dn11)), (assign12920_body2_e16371 * (-locals.var_el_dn12)), (assign12920_body2_e16371 * (-locals.var_el_dn17)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign12920_body2_e16373;
            locals.var_t0_dn0 = assign12920_body2_e16373_d_n0;
            locals.var_t0_dn2 = assign12920_body2_e16373_d_n2;
            locals.var_t0_dn6 = assign12920_body2_e16373_d_n6;
            locals.var_t0_dn7 = assign12920_body2_e16373_d_n7;
            locals.var_t0_dn10 = assign12920_body2_e16373_d_n10;
            locals.var_t0_dn11 = assign12920_body2_e16373_d_n11;
            locals.var_t0_dn12 = assign12920_body2_e16373_d_n12;
            locals.var_t0_dn17 = assign12920_body2_e16373_d_n17;
            let assign12920_body3_e16376: f64 = (-1e-9);
            let assign12920_body3_e16377: f64 = if locals.var_t1__blk353 < assign12920_body3_e16376 { 1.0 } else { 0.0 };
            locals.var_guard389 = assign12920_body3_e16377;
            let (assign12920_body4_e16393, assign12920_body4_e16393_d_n0, assign12920_body4_e16393_d_n2, assign12920_body4_e16393_d_n6, assign12920_body4_e16393_d_n7, assign12920_body4_e16393_d_n10, assign12920_body4_e16393_d_n11, assign12920_body4_e16393_d_n12, assign12920_body4_e16393_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard389 != 0.0)) {
        let assign12920_body4_e16387: f64 = (locals.var_t0 + locals.var_el);
        let assign12920_body4_e16389: f64 = (assign12920_body4_e16387 - 1.0);
        let assign12920_body4_e16390: f64 = (assign12920_body4_e16389).sqrt();
        let assign12920_body4_e16391: f64 = (locals.var_cnst0bulk * assign12920_body4_e16390);
        (assign12920_body4_e16391, (locals.var_cnst0bulk * ((locals.var_t0_dn0 + locals.var_el_dn0) / (2.0 * assign12920_body4_e16390))), (locals.var_cnst0bulk * ((locals.var_t0_dn2 + locals.var_el_dn2) / (2.0 * assign12920_body4_e16390))), (locals.var_cnst0bulk * ((locals.var_t0_dn6 + locals.var_el_dn6) / (2.0 * assign12920_body4_e16390))), (locals.var_cnst0bulk * ((locals.var_t0_dn7 + locals.var_el_dn7) / (2.0 * assign12920_body4_e16390))), ((locals.var_cnst0bulk_dn10 * assign12920_body4_e16390) + (locals.var_cnst0bulk * ((locals.var_t0_dn10 + locals.var_el_dn10) / (2.0 * assign12920_body4_e16390)))), (locals.var_cnst0bulk * ((locals.var_t0_dn11 + locals.var_el_dn11) / (2.0 * assign12920_body4_e16390))), (locals.var_cnst0bulk * ((locals.var_t0_dn12 + locals.var_el_dn12) / (2.0 * assign12920_body4_e16390))), (locals.var_cnst0bulk * ((locals.var_t0_dn17 + locals.var_el_dn17) / (2.0 * assign12920_body4_e16390))),)
    } else {
        (locals.var_q_sl_bulk, locals.var_q_sl_bulk_dn0, locals.var_q_sl_bulk_dn2, locals.var_q_sl_bulk_dn6, locals.var_q_sl_bulk_dn7, locals.var_q_sl_bulk_dn10, locals.var_q_sl_bulk_dn11, locals.var_q_sl_bulk_dn12, locals.var_q_sl_bulk_dn17,)
    }
};
            locals.var_q_sl_bulk = assign12920_body4_e16393;
            locals.var_q_sl_bulk_dn0 = assign12920_body4_e16393_d_n0;
            locals.var_q_sl_bulk_dn2 = assign12920_body4_e16393_d_n2;
            locals.var_q_sl_bulk_dn6 = assign12920_body4_e16393_d_n6;
            locals.var_q_sl_bulk_dn7 = assign12920_body4_e16393_d_n7;
            locals.var_q_sl_bulk_dn10 = assign12920_body4_e16393_d_n10;
            locals.var_q_sl_bulk_dn11 = assign12920_body4_e16393_d_n11;
            locals.var_q_sl_bulk_dn12 = assign12920_body4_e16393_d_n12;
            locals.var_q_sl_bulk_dn17 = assign12920_body4_e16393_d_n17;
            let (assign12920_body5_e16409, assign12920_body5_e16409_d_n0, assign12920_body5_e16409_d_n2, assign12920_body5_e16409_d_n6, assign12920_body5_e16409_d_n7, assign12920_body5_e16409_d_n10, assign12920_body5_e16409_d_n11, assign12920_body5_e16409_d_n12, assign12920_body5_e16409_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard389 != 0.0)) {
        let assign12920_body5_e16402: f64 = (-locals.var_t0);
        let assign12920_body5_e16404: f64 = (assign12920_body5_e16402 + 1.0);
        let assign12920_body5_e16405: f64 = (locals.var_c0bulk * assign12920_body5_e16404);
        let assign12920_body5_e16407: f64 = (assign12920_body5_e16405 / locals.var_q_sl_bulk);
        (assign12920_body5_e16407, ((((locals.var_c0bulk * (-locals.var_t0_dn0)) * locals.var_q_sl_bulk) - (assign12920_body5_e16405 * locals.var_q_sl_bulk_dn0)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * (-locals.var_t0_dn2)) * locals.var_q_sl_bulk) - (assign12920_body5_e16405 * locals.var_q_sl_bulk_dn2)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * (-locals.var_t0_dn6)) * locals.var_q_sl_bulk) - (assign12920_body5_e16405 * locals.var_q_sl_bulk_dn6)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * (-locals.var_t0_dn7)) * locals.var_q_sl_bulk) - (assign12920_body5_e16405 * locals.var_q_sl_bulk_dn7)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * (-locals.var_t0_dn10)) * locals.var_q_sl_bulk) - (assign12920_body5_e16405 * locals.var_q_sl_bulk_dn10)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * (-locals.var_t0_dn11)) * locals.var_q_sl_bulk) - (assign12920_body5_e16405 * locals.var_q_sl_bulk_dn11)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * (-locals.var_t0_dn12)) * locals.var_q_sl_bulk) - (assign12920_body5_e16405 * locals.var_q_sl_bulk_dn12)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * (-locals.var_t0_dn17)) * locals.var_q_sl_bulk) - (assign12920_body5_e16405 * locals.var_q_sl_bulk_dn17)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)),)
    } else {
        (locals.var_q_sl_bulk_dpsb, locals.var_q_sl_bulk_dpsb_dn0, locals.var_q_sl_bulk_dpsb_dn2, locals.var_q_sl_bulk_dpsb_dn6, locals.var_q_sl_bulk_dpsb_dn7, locals.var_q_sl_bulk_dpsb_dn10, locals.var_q_sl_bulk_dpsb_dn11, locals.var_q_sl_bulk_dpsb_dn12, locals.var_q_sl_bulk_dpsb_dn17,)
    }
};
            locals.var_q_sl_bulk_dpsb = assign12920_body5_e16409;
            locals.var_q_sl_bulk_dpsb_dn0 = assign12920_body5_e16409_d_n0;
            locals.var_q_sl_bulk_dpsb_dn2 = assign12920_body5_e16409_d_n2;
            locals.var_q_sl_bulk_dpsb_dn6 = assign12920_body5_e16409_d_n6;
            locals.var_q_sl_bulk_dpsb_dn7 = assign12920_body5_e16409_d_n7;
            locals.var_q_sl_bulk_dpsb_dn10 = assign12920_body5_e16409_d_n10;
            locals.var_q_sl_bulk_dpsb_dn11 = assign12920_body5_e16409_d_n11;
            locals.var_q_sl_bulk_dpsb_dn12 = assign12920_body5_e16409_d_n12;
            locals.var_q_sl_bulk_dpsb_dn17 = assign12920_body5_e16409_d_n17;
            let assign12920_body6_e16412: f64 = if locals.var_t1__blk353 > 1e-9 { 1.0 } else { 0.0 };
            locals.var_guard390 = assign12920_body6_e16412;
            let (assign12920_body7_e16425, assign12920_body7_e16425_d_n0, assign12920_body7_e16425_d_n2, assign12920_body7_e16425_d_n6, assign12920_body7_e16425_d_n7, assign12920_body7_e16425_d_n10, assign12920_body7_e16425_d_n11, assign12920_body7_e16425_d_n12, assign12920_body7_e16425_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard389 == 0.0)) && (locals.var_guard390 != 0.0)) {
        let assign12920_body7_e16423: f64 = (locals.var_el).exp();
        (assign12920_body7_e16423, (assign12920_body7_e16423 * locals.var_el_dn0), (assign12920_body7_e16423 * locals.var_el_dn2), (assign12920_body7_e16423 * locals.var_el_dn6), (assign12920_body7_e16423 * locals.var_el_dn7), (assign12920_body7_e16423 * locals.var_el_dn10), (assign12920_body7_e16423 * locals.var_el_dn11), (assign12920_body7_e16423 * locals.var_el_dn12), (assign12920_body7_e16423 * locals.var_el_dn17),)
    } else {
        (locals.var_t2__blk354, locals.var_t2__blk354_dn0, locals.var_t2__blk354_dn2, locals.var_t2__blk354_dn6, locals.var_t2__blk354_dn7, locals.var_t2__blk354_dn10, locals.var_t2__blk354_dn11, locals.var_t2__blk354_dn12, locals.var_t2__blk354_dn17,)
    }
};
            locals.var_t2__blk354 = assign12920_body7_e16425;
            locals.var_t2__blk354_dn0 = assign12920_body7_e16425_d_n0;
            locals.var_t2__blk354_dn2 = assign12920_body7_e16425_d_n2;
            locals.var_t2__blk354_dn6 = assign12920_body7_e16425_d_n6;
            locals.var_t2__blk354_dn7 = assign12920_body7_e16425_d_n7;
            locals.var_t2__blk354_dn10 = assign12920_body7_e16425_d_n10;
            locals.var_t2__blk354_dn11 = assign12920_body7_e16425_d_n11;
            locals.var_t2__blk354_dn12 = assign12920_body7_e16425_d_n12;
            locals.var_t2__blk354_dn17 = assign12920_body7_e16425_d_n17;
            let (assign12920_body8_e16453, assign12920_body8_e16453_d_n0, assign12920_body8_e16453_d_n2, assign12920_body8_e16453_d_n6, assign12920_body8_e16453_d_n7, assign12920_body8_e16453_d_n10, assign12920_body8_e16453_d_n11, assign12920_body8_e16453_d_n12, assign12920_body8_e16453_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard389 == 0.0)) && (locals.var_guard390 != 0.0)) {
        let assign12920_body8_e16436: f64 = (-locals.var_cnst0bulk);
        let assign12920_body8_e16439: f64 = (locals.var_t0 + locals.var_el);
        let assign12920_body8_e16441: f64 = (assign12920_body8_e16439 - 1.0);
        let assign12920_body8_e16445: f64 = (locals.var_t2__blk354 + locals.var_el);
        let assign12920_body8_e16447: f64 = (assign12920_body8_e16445 - 1.0);
        let assign12920_body8_e16448: f64 = (locals.var_cnst1bulk * assign12920_body8_e16447);
        let assign12920_body8_e16449: f64 = (assign12920_body8_e16441 + assign12920_body8_e16448);
        let assign12920_body8_e16450: f64 = (assign12920_body8_e16449).sqrt();
        let assign12920_body8_e16451: f64 = (assign12920_body8_e16436 * assign12920_body8_e16450);
        (assign12920_body8_e16451, (assign12920_body8_e16436 * (((locals.var_t0_dn0 + locals.var_el_dn0) + ((locals.var_cnst1bulk_dn0 * assign12920_body8_e16447) + (locals.var_cnst1bulk * (locals.var_t2__blk354_dn0 + locals.var_el_dn0)))) / (2.0 * assign12920_body8_e16450))), (assign12920_body8_e16436 * (((locals.var_t0_dn2 + locals.var_el_dn2) + ((locals.var_cnst1bulk_dn2 * assign12920_body8_e16447) + (locals.var_cnst1bulk * (locals.var_t2__blk354_dn2 + locals.var_el_dn2)))) / (2.0 * assign12920_body8_e16450))), (assign12920_body8_e16436 * (((locals.var_t0_dn6 + locals.var_el_dn6) + ((locals.var_cnst1bulk_dn6 * assign12920_body8_e16447) + (locals.var_cnst1bulk * (locals.var_t2__blk354_dn6 + locals.var_el_dn6)))) / (2.0 * assign12920_body8_e16450))), (assign12920_body8_e16436 * (((locals.var_t0_dn7 + locals.var_el_dn7) + ((locals.var_cnst1bulk_dn7 * assign12920_body8_e16447) + (locals.var_cnst1bulk * (locals.var_t2__blk354_dn7 + locals.var_el_dn7)))) / (2.0 * assign12920_body8_e16450))), (((-locals.var_cnst0bulk_dn10) * assign12920_body8_e16450) + (assign12920_body8_e16436 * (((locals.var_t0_dn10 + locals.var_el_dn10) + ((locals.var_cnst1bulk_dn10 * assign12920_body8_e16447) + (locals.var_cnst1bulk * (locals.var_t2__blk354_dn10 + locals.var_el_dn10)))) / (2.0 * assign12920_body8_e16450)))), (assign12920_body8_e16436 * (((locals.var_t0_dn11 + locals.var_el_dn11) + ((locals.var_cnst1bulk_dn11 * assign12920_body8_e16447) + (locals.var_cnst1bulk * (locals.var_t2__blk354_dn11 + locals.var_el_dn11)))) / (2.0 * assign12920_body8_e16450))), (assign12920_body8_e16436 * (((locals.var_t0_dn12 + locals.var_el_dn12) + ((locals.var_cnst1bulk_dn12 * assign12920_body8_e16447) + (locals.var_cnst1bulk * (locals.var_t2__blk354_dn12 + locals.var_el_dn12)))) / (2.0 * assign12920_body8_e16450))), (assign12920_body8_e16436 * (((locals.var_t0_dn17 + locals.var_el_dn17) + ((locals.var_cnst1bulk_dn17 * assign12920_body8_e16447) + (locals.var_cnst1bulk * (locals.var_t2__blk354_dn17 + locals.var_el_dn17)))) / (2.0 * assign12920_body8_e16450))),)
    } else {
        (locals.var_q_sl_bulk, locals.var_q_sl_bulk_dn0, locals.var_q_sl_bulk_dn2, locals.var_q_sl_bulk_dn6, locals.var_q_sl_bulk_dn7, locals.var_q_sl_bulk_dn10, locals.var_q_sl_bulk_dn11, locals.var_q_sl_bulk_dn12, locals.var_q_sl_bulk_dn17,)
    }
};
            locals.var_q_sl_bulk = assign12920_body8_e16453;
            locals.var_q_sl_bulk_dn0 = assign12920_body8_e16453_d_n0;
            locals.var_q_sl_bulk_dn2 = assign12920_body8_e16453_d_n2;
            locals.var_q_sl_bulk_dn6 = assign12920_body8_e16453_d_n6;
            locals.var_q_sl_bulk_dn7 = assign12920_body8_e16453_d_n7;
            locals.var_q_sl_bulk_dn10 = assign12920_body8_e16453_d_n10;
            locals.var_q_sl_bulk_dn11 = assign12920_body8_e16453_d_n11;
            locals.var_q_sl_bulk_dn12 = assign12920_body8_e16453_d_n12;
            locals.var_q_sl_bulk_dn17 = assign12920_body8_e16453_d_n17;
            let (assign12920_body9_e16478, assign12920_body9_e16478_d_n0, assign12920_body9_e16478_d_n2, assign12920_body9_e16478_d_n6, assign12920_body9_e16478_d_n7, assign12920_body9_e16478_d_n10, assign12920_body9_e16478_d_n11, assign12920_body9_e16478_d_n12, assign12920_body9_e16478_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard389 == 0.0)) && (locals.var_guard390 != 0.0)) {
        let assign12920_body9_e16465: f64 = (-locals.var_t0);
        let assign12920_body9_e16467: f64 = (assign12920_body9_e16465 + 1.0);
        let assign12920_body9_e16471: f64 = (locals.var_t2__blk354 + 1.0);
        let assign12920_body9_e16472: f64 = (locals.var_cnst1bulk * assign12920_body9_e16471);
        let assign12920_body9_e16473: f64 = (assign12920_body9_e16467 + assign12920_body9_e16472);
        let assign12920_body9_e16474: f64 = (locals.var_c0bulk * assign12920_body9_e16473);
        let assign12920_body9_e16476: f64 = (assign12920_body9_e16474 / locals.var_q_sl_bulk);
        (assign12920_body9_e16476, ((((locals.var_c0bulk * ((-locals.var_t0_dn0) + ((locals.var_cnst1bulk_dn0 * assign12920_body9_e16471) + (locals.var_cnst1bulk * locals.var_t2__blk354_dn0)))) * locals.var_q_sl_bulk) - (assign12920_body9_e16474 * locals.var_q_sl_bulk_dn0)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * ((-locals.var_t0_dn2) + ((locals.var_cnst1bulk_dn2 * assign12920_body9_e16471) + (locals.var_cnst1bulk * locals.var_t2__blk354_dn2)))) * locals.var_q_sl_bulk) - (assign12920_body9_e16474 * locals.var_q_sl_bulk_dn2)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * ((-locals.var_t0_dn6) + ((locals.var_cnst1bulk_dn6 * assign12920_body9_e16471) + (locals.var_cnst1bulk * locals.var_t2__blk354_dn6)))) * locals.var_q_sl_bulk) - (assign12920_body9_e16474 * locals.var_q_sl_bulk_dn6)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * ((-locals.var_t0_dn7) + ((locals.var_cnst1bulk_dn7 * assign12920_body9_e16471) + (locals.var_cnst1bulk * locals.var_t2__blk354_dn7)))) * locals.var_q_sl_bulk) - (assign12920_body9_e16474 * locals.var_q_sl_bulk_dn7)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * ((-locals.var_t0_dn10) + ((locals.var_cnst1bulk_dn10 * assign12920_body9_e16471) + (locals.var_cnst1bulk * locals.var_t2__blk354_dn10)))) * locals.var_q_sl_bulk) - (assign12920_body9_e16474 * locals.var_q_sl_bulk_dn10)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * ((-locals.var_t0_dn11) + ((locals.var_cnst1bulk_dn11 * assign12920_body9_e16471) + (locals.var_cnst1bulk * locals.var_t2__blk354_dn11)))) * locals.var_q_sl_bulk) - (assign12920_body9_e16474 * locals.var_q_sl_bulk_dn11)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * ((-locals.var_t0_dn12) + ((locals.var_cnst1bulk_dn12 * assign12920_body9_e16471) + (locals.var_cnst1bulk * locals.var_t2__blk354_dn12)))) * locals.var_q_sl_bulk) - (assign12920_body9_e16474 * locals.var_q_sl_bulk_dn12)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * ((-locals.var_t0_dn17) + ((locals.var_cnst1bulk_dn17 * assign12920_body9_e16471) + (locals.var_cnst1bulk * locals.var_t2__blk354_dn17)))) * locals.var_q_sl_bulk) - (assign12920_body9_e16474 * locals.var_q_sl_bulk_dn17)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)),)
    } else {
        (locals.var_q_sl_bulk_dpsb, locals.var_q_sl_bulk_dpsb_dn0, locals.var_q_sl_bulk_dpsb_dn2, locals.var_q_sl_bulk_dpsb_dn6, locals.var_q_sl_bulk_dpsb_dn7, locals.var_q_sl_bulk_dpsb_dn10, locals.var_q_sl_bulk_dpsb_dn11, locals.var_q_sl_bulk_dpsb_dn12, locals.var_q_sl_bulk_dpsb_dn17,)
    }
};
            locals.var_q_sl_bulk_dpsb = assign12920_body9_e16478;
            locals.var_q_sl_bulk_dpsb_dn0 = assign12920_body9_e16478_d_n0;
            locals.var_q_sl_bulk_dpsb_dn2 = assign12920_body9_e16478_d_n2;
            locals.var_q_sl_bulk_dpsb_dn6 = assign12920_body9_e16478_d_n6;
            locals.var_q_sl_bulk_dpsb_dn7 = assign12920_body9_e16478_d_n7;
            locals.var_q_sl_bulk_dpsb_dn10 = assign12920_body9_e16478_d_n10;
            locals.var_q_sl_bulk_dpsb_dn11 = assign12920_body9_e16478_d_n11;
            locals.var_q_sl_bulk_dpsb_dn12 = assign12920_body9_e16478_d_n12;
            locals.var_q_sl_bulk_dpsb_dn17 = assign12920_body9_e16478_d_n17;
            let (assign12920_body10_e16494, assign12920_body10_e16494_d_n0, assign12920_body10_e16494_d_n2, assign12920_body10_e16494_d_n6, assign12920_body10_e16494_d_n7, assign12920_body10_e16494_d_n10, assign12920_body10_e16494_d_n11, assign12920_body10_e16494_d_n12, assign12920_body10_e16494_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard389 == 0.0)) && (locals.var_guard390 == 0.0)) {
        let assign12920_body10_e16490: f64 = (-locals.var_cnst0bulk);
        let assign12920_body10_e16492: f64 = (assign12920_body10_e16490 * locals.var_el);
        (assign12920_body10_e16492, (assign12920_body10_e16490 * locals.var_el_dn0), (assign12920_body10_e16490 * locals.var_el_dn2), (assign12920_body10_e16490 * locals.var_el_dn6), (assign12920_body10_e16490 * locals.var_el_dn7), (((-locals.var_cnst0bulk_dn10) * locals.var_el) + (assign12920_body10_e16490 * locals.var_el_dn10)), (assign12920_body10_e16490 * locals.var_el_dn11), (assign12920_body10_e16490 * locals.var_el_dn12), (assign12920_body10_e16490 * locals.var_el_dn17),)
    } else {
        (locals.var_q_sl_bulk, locals.var_q_sl_bulk_dn0, locals.var_q_sl_bulk_dn2, locals.var_q_sl_bulk_dn6, locals.var_q_sl_bulk_dn7, locals.var_q_sl_bulk_dn10, locals.var_q_sl_bulk_dn11, locals.var_q_sl_bulk_dn12, locals.var_q_sl_bulk_dn17,)
    }
};
            locals.var_q_sl_bulk = assign12920_body10_e16494;
            locals.var_q_sl_bulk_dn0 = assign12920_body10_e16494_d_n0;
            locals.var_q_sl_bulk_dn2 = assign12920_body10_e16494_d_n2;
            locals.var_q_sl_bulk_dn6 = assign12920_body10_e16494_d_n6;
            locals.var_q_sl_bulk_dn7 = assign12920_body10_e16494_d_n7;
            locals.var_q_sl_bulk_dn10 = assign12920_body10_e16494_d_n10;
            locals.var_q_sl_bulk_dn11 = assign12920_body10_e16494_d_n11;
            locals.var_q_sl_bulk_dn12 = assign12920_body10_e16494_d_n12;
            locals.var_q_sl_bulk_dn17 = assign12920_body10_e16494_d_n17;
            let (assign12920_body11_e16510, assign12920_body11_e16510_d_n0, assign12920_body11_e16510_d_n2, assign12920_body11_e16510_d_n6, assign12920_body11_e16510_d_n7, assign12920_body11_e16510_d_n10, assign12920_body11_e16510_d_n11, assign12920_body11_e16510_d_n12, assign12920_body11_e16510_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard389 == 0.0)) && (locals.var_guard390 == 0.0)) {
        let assign12920_body11_e16506: f64 = (-locals.var_cnst0bulk);
        let assign12920_body11_e16508: f64 = (assign12920_body11_e16506 * locals.var_beta);
        (assign12920_body11_e16508, 0.0, 0.0, 0.0, 0.0, (((-locals.var_cnst0bulk_dn10) * locals.var_beta) + (assign12920_body11_e16506 * locals.var_beta_dn10)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_sl_bulk_dpsb, locals.var_q_sl_bulk_dpsb_dn0, locals.var_q_sl_bulk_dpsb_dn2, locals.var_q_sl_bulk_dpsb_dn6, locals.var_q_sl_bulk_dpsb_dn7, locals.var_q_sl_bulk_dpsb_dn10, locals.var_q_sl_bulk_dpsb_dn11, locals.var_q_sl_bulk_dpsb_dn12, locals.var_q_sl_bulk_dpsb_dn17,)
    }
};
            locals.var_q_sl_bulk_dpsb = assign12920_body11_e16510;
            locals.var_q_sl_bulk_dpsb_dn0 = assign12920_body11_e16510_d_n0;
            locals.var_q_sl_bulk_dpsb_dn2 = assign12920_body11_e16510_d_n2;
            locals.var_q_sl_bulk_dpsb_dn6 = assign12920_body11_e16510_d_n6;
            locals.var_q_sl_bulk_dpsb_dn7 = assign12920_body11_e16510_d_n7;
            locals.var_q_sl_bulk_dpsb_dn10 = assign12920_body11_e16510_d_n10;
            locals.var_q_sl_bulk_dpsb_dn11 = assign12920_body11_e16510_d_n11;
            locals.var_q_sl_bulk_dpsb_dn12 = assign12920_body11_e16510_d_n12;
            locals.var_q_sl_bulk_dpsb_dn17 = assign12920_body11_e16510_d_n17;
            let (assign12920_body12_e16517, assign12920_body12_e16517_d_n0, assign12920_body12_e16517_d_n2, assign12920_body12_e16517_d_n6, assign12920_body12_e16517_d_n7, assign12920_body12_e16517_d_n10, assign12920_body12_e16517_d_n11, assign12920_body12_e16517_d_n12, assign12920_body12_e16517_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        (locals.var_q_s0_dep_ini, locals.var_q_s0_dep_ini_dn0, locals.var_q_s0_dep_ini_dn2, locals.var_q_s0_dep_ini_dn6, locals.var_q_s0_dep_ini_dn7, locals.var_q_s0_dep_ini_dn10, locals.var_q_s0_dep_ini_dn11, locals.var_q_s0_dep_ini_dn12, locals.var_q_s0_dep_ini_dn17,)
    } else {
        (locals.var_q_sl_dep, locals.var_q_sl_dep_dn0, locals.var_q_sl_dep_dn2, locals.var_q_sl_dep_dn6, locals.var_q_sl_dep_dn7, locals.var_q_sl_dep_dn10, locals.var_q_sl_dep_dn11, locals.var_q_sl_dep_dn12, locals.var_q_sl_dep_dn17,)
    }
};
            locals.var_q_sl_dep = assign12920_body12_e16517;
            locals.var_q_sl_dep_dn0 = assign12920_body12_e16517_d_n0;
            locals.var_q_sl_dep_dn2 = assign12920_body12_e16517_d_n2;
            locals.var_q_sl_dep_dn6 = assign12920_body12_e16517_d_n6;
            locals.var_q_sl_dep_dn7 = assign12920_body12_e16517_d_n7;
            locals.var_q_sl_dep_dn10 = assign12920_body12_e16517_d_n10;
            locals.var_q_sl_dep_dn11 = assign12920_body12_e16517_d_n11;
            locals.var_q_sl_dep_dn12 = assign12920_body12_e16517_d_n12;
            locals.var_q_sl_dep_dn17 = assign12920_body12_e16517_d_n17;
            let (assign12920_body13_e16529, assign12920_body13_e16529_d_n0, assign12920_body13_e16529_d_n2, assign12920_body13_e16529_d_n6, assign12920_body13_e16529_d_n7, assign12920_body13_e16529_d_n10, assign12920_body13_e16529_d_n11, assign12920_body13_e16529_d_n12, assign12920_body13_e16529_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign12920_body13_e16525: f64 = (locals.var_phi_sl_soi - locals.var_vds);
        let assign12920_body13_e16526: f64 = (locals.var_beta * assign12920_body13_e16525);
        let assign12920_body13_e16527: f64 = (assign12920_body13_e16526).exp();
        (assign12920_body13_e16527, (assign12920_body13_e16527 * (locals.var_beta * (locals.var_phi_sl_soi_dn0 - locals.var_vds_dn0))), (assign12920_body13_e16527 * (locals.var_beta * (locals.var_phi_sl_soi_dn2 - locals.var_vds_dn2))), (assign12920_body13_e16527 * (locals.var_beta * (locals.var_phi_sl_soi_dn6 - locals.var_vds_dn6))), (assign12920_body13_e16527 * (locals.var_beta * (locals.var_phi_sl_soi_dn7 - locals.var_vds_dn7))), (assign12920_body13_e16527 * ((locals.var_beta_dn10 * assign12920_body13_e16525) + (locals.var_beta * (locals.var_phi_sl_soi_dn10 - locals.var_vds_dn10)))), (assign12920_body13_e16527 * (locals.var_beta * (locals.var_phi_sl_soi_dn11 - locals.var_vds_dn11))), (assign12920_body13_e16527 * (locals.var_beta * (locals.var_phi_sl_soi_dn12 - locals.var_vds_dn12))), (assign12920_body13_e16527 * (locals.var_beta * (locals.var_phi_sl_soi_dn17 - locals.var_vds_dn17))),)
    } else {
        (locals.var_t5__blk357, locals.var_t5__blk357_dn0, locals.var_t5__blk357_dn2, locals.var_t5__blk357_dn6, locals.var_t5__blk357_dn7, locals.var_t5__blk357_dn10, locals.var_t5__blk357_dn11, locals.var_t5__blk357_dn12, locals.var_t5__blk357_dn17,)
    }
};
            locals.var_t5__blk357 = assign12920_body13_e16529;
            locals.var_t5__blk357_dn0 = assign12920_body13_e16529_d_n0;
            locals.var_t5__blk357_dn2 = assign12920_body13_e16529_d_n2;
            locals.var_t5__blk357_dn6 = assign12920_body13_e16529_d_n6;
            locals.var_t5__blk357_dn7 = assign12920_body13_e16529_d_n7;
            locals.var_t5__blk357_dn10 = assign12920_body13_e16529_d_n10;
            locals.var_t5__blk357_dn11 = assign12920_body13_e16529_d_n11;
            locals.var_t5__blk357_dn12 = assign12920_body13_e16529_d_n12;
            locals.var_t5__blk357_dn17 = assign12920_body13_e16529_d_n17;
            let (assign12920_body14_e16536, assign12920_body14_e16536_d_n0, assign12920_body14_e16536_d_n2, assign12920_body14_e16536_d_n6, assign12920_body14_e16536_d_n7, assign12920_body14_e16536_d_n10, assign12920_body14_e16536_d_n11, assign12920_body14_e16536_d_n12, assign12920_body14_e16536_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3__blk355, locals.var_t3__blk355_dn0, locals.var_t3__blk355_dn2, locals.var_t3__blk355_dn6, locals.var_t3__blk355_dn7, locals.var_t3__blk355_dn10, locals.var_t3__blk355_dn11, locals.var_t3__blk355_dn12, locals.var_t3__blk355_dn17,)
    }
};
            locals.var_t3__blk355 = assign12920_body14_e16536;
            locals.var_t3__blk355_dn0 = assign12920_body14_e16536_d_n0;
            locals.var_t3__blk355_dn2 = assign12920_body14_e16536_d_n2;
            locals.var_t3__blk355_dn6 = assign12920_body14_e16536_d_n6;
            locals.var_t3__blk355_dn7 = assign12920_body14_e16536_d_n7;
            locals.var_t3__blk355_dn10 = assign12920_body14_e16536_d_n10;
            locals.var_t3__blk355_dn11 = assign12920_body14_e16536_d_n11;
            locals.var_t3__blk355_dn12 = assign12920_body14_e16536_d_n12;
            locals.var_t3__blk355_dn17 = assign12920_body14_e16536_d_n17;
            let (assign12920_body15_e16560, assign12920_body15_e16560_d_n0, assign12920_body15_e16560_d_n2, assign12920_body15_e16560_d_n6, assign12920_body15_e16560_d_n7, assign12920_body15_e16560_d_n10, assign12920_body15_e16560_d_n11, assign12920_body15_e16560_d_n12, assign12920_body15_e16560_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign12920_body15_e16543: f64 = (locals.var_q_sl_dep * locals.var_q_sl_dep);
        let assign12920_body15_e16546: f64 = (locals.var_cnst0soi * locals.var_cnst0soi);
        let assign12920_body15_e16547: f64 = (assign12920_body15_e16543 / assign12920_body15_e16546);
        let assign12920_body15_e16550: f64 = (2.0 * locals.var_cnst1soi);
        let assign12920_body15_e16553: f64 = (locals.var_t5__blk357 + locals.var_el);
        let assign12920_body15_e16555: f64 = (assign12920_body15_e16553 - locals.var_t3__blk355);
        let assign12920_body15_e16556: f64 = (assign12920_body15_e16550 * assign12920_body15_e16555);
        let assign12920_body15_e16557: f64 = (assign12920_body15_e16547 + assign12920_body15_e16556);
        let assign12920_body15_e16558: f64 = (assign12920_body15_e16557).sqrt();
        (assign12920_body15_e16558, (((((((locals.var_q_sl_dep_dn0 * locals.var_q_sl_dep) + (locals.var_q_sl_dep * locals.var_q_sl_dep_dn0)) * assign12920_body15_e16546) - (assign12920_body15_e16543 * ((locals.var_cnst0soi_dn0 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn0)))) / (assign12920_body15_e16546 * assign12920_body15_e16546)) + (((2.0 * locals.var_cnst1soi_dn0) * assign12920_body15_e16555) + (assign12920_body15_e16550 * ((locals.var_t5__blk357_dn0 + locals.var_el_dn0) - locals.var_t3__blk355_dn0)))) / (2.0 * assign12920_body15_e16558)), (((((((locals.var_q_sl_dep_dn2 * locals.var_q_sl_dep) + (locals.var_q_sl_dep * locals.var_q_sl_dep_dn2)) * assign12920_body15_e16546) - (assign12920_body15_e16543 * ((locals.var_cnst0soi_dn2 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn2)))) / (assign12920_body15_e16546 * assign12920_body15_e16546)) + (((2.0 * locals.var_cnst1soi_dn2) * assign12920_body15_e16555) + (assign12920_body15_e16550 * ((locals.var_t5__blk357_dn2 + locals.var_el_dn2) - locals.var_t3__blk355_dn2)))) / (2.0 * assign12920_body15_e16558)), (((((((locals.var_q_sl_dep_dn6 * locals.var_q_sl_dep) + (locals.var_q_sl_dep * locals.var_q_sl_dep_dn6)) * assign12920_body15_e16546) - (assign12920_body15_e16543 * ((locals.var_cnst0soi_dn6 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn6)))) / (assign12920_body15_e16546 * assign12920_body15_e16546)) + (((2.0 * locals.var_cnst1soi_dn6) * assign12920_body15_e16555) + (assign12920_body15_e16550 * ((locals.var_t5__blk357_dn6 + locals.var_el_dn6) - locals.var_t3__blk355_dn6)))) / (2.0 * assign12920_body15_e16558)), (((((((locals.var_q_sl_dep_dn7 * locals.var_q_sl_dep) + (locals.var_q_sl_dep * locals.var_q_sl_dep_dn7)) * assign12920_body15_e16546) - (assign12920_body15_e16543 * ((locals.var_cnst0soi_dn7 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn7)))) / (assign12920_body15_e16546 * assign12920_body15_e16546)) + (((2.0 * locals.var_cnst1soi_dn7) * assign12920_body15_e16555) + (assign12920_body15_e16550 * ((locals.var_t5__blk357_dn7 + locals.var_el_dn7) - locals.var_t3__blk355_dn7)))) / (2.0 * assign12920_body15_e16558)), (((((((locals.var_q_sl_dep_dn10 * locals.var_q_sl_dep) + (locals.var_q_sl_dep * locals.var_q_sl_dep_dn10)) * assign12920_body15_e16546) - (assign12920_body15_e16543 * ((locals.var_cnst0soi_dn10 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn10)))) / (assign12920_body15_e16546 * assign12920_body15_e16546)) + (((2.0 * locals.var_cnst1soi_dn10) * assign12920_body15_e16555) + (assign12920_body15_e16550 * ((locals.var_t5__blk357_dn10 + locals.var_el_dn10) - locals.var_t3__blk355_dn10)))) / (2.0 * assign12920_body15_e16558)), (((((((locals.var_q_sl_dep_dn11 * locals.var_q_sl_dep) + (locals.var_q_sl_dep * locals.var_q_sl_dep_dn11)) * assign12920_body15_e16546) - (assign12920_body15_e16543 * ((locals.var_cnst0soi_dn11 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn11)))) / (assign12920_body15_e16546 * assign12920_body15_e16546)) + (((2.0 * locals.var_cnst1soi_dn11) * assign12920_body15_e16555) + (assign12920_body15_e16550 * ((locals.var_t5__blk357_dn11 + locals.var_el_dn11) - locals.var_t3__blk355_dn11)))) / (2.0 * assign12920_body15_e16558)), (((((((locals.var_q_sl_dep_dn12 * locals.var_q_sl_dep) + (locals.var_q_sl_dep * locals.var_q_sl_dep_dn12)) * assign12920_body15_e16546) - (assign12920_body15_e16543 * ((locals.var_cnst0soi_dn12 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn12)))) / (assign12920_body15_e16546 * assign12920_body15_e16546)) + (((2.0 * locals.var_cnst1soi_dn12) * assign12920_body15_e16555) + (assign12920_body15_e16550 * ((locals.var_t5__blk357_dn12 + locals.var_el_dn12) - locals.var_t3__blk355_dn12)))) / (2.0 * assign12920_body15_e16558)), (((((((locals.var_q_sl_dep_dn17 * locals.var_q_sl_dep) + (locals.var_q_sl_dep * locals.var_q_sl_dep_dn17)) * assign12920_body15_e16546) - (assign12920_body15_e16543 * ((locals.var_cnst0soi_dn17 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn17)))) / (assign12920_body15_e16546 * assign12920_body15_e16546)) + (((2.0 * locals.var_cnst1soi_dn17) * assign12920_body15_e16555) + (assign12920_body15_e16550 * ((locals.var_t5__blk357_dn17 + locals.var_el_dn17) - locals.var_t3__blk355_dn17)))) / (2.0 * assign12920_body15_e16558)),)
    } else {
        (locals.var_t4__blk356, locals.var_t4__blk356_dn0, locals.var_t4__blk356_dn2, locals.var_t4__blk356_dn6, locals.var_t4__blk356_dn7, locals.var_t4__blk356_dn10, locals.var_t4__blk356_dn11, locals.var_t4__blk356_dn12, locals.var_t4__blk356_dn17,)
    }
};
            locals.var_t4__blk356 = assign12920_body15_e16560;
            locals.var_t4__blk356_dn0 = assign12920_body15_e16560_d_n0;
            locals.var_t4__blk356_dn2 = assign12920_body15_e16560_d_n2;
            locals.var_t4__blk356_dn6 = assign12920_body15_e16560_d_n6;
            locals.var_t4__blk356_dn7 = assign12920_body15_e16560_d_n7;
            locals.var_t4__blk356_dn10 = assign12920_body15_e16560_d_n10;
            locals.var_t4__blk356_dn11 = assign12920_body15_e16560_d_n11;
            locals.var_t4__blk356_dn12 = assign12920_body15_e16560_d_n12;
            locals.var_t4__blk356_dn17 = assign12920_body15_e16560_d_n17;
            let (assign12920_body16_e16579, assign12920_body16_e16579_d_n0, assign12920_body16_e16579_d_n2, assign12920_body16_e16579_d_n6, assign12920_body16_e16579_d_n7, assign12920_body16_e16579_d_n10, assign12920_body16_e16579_d_n11, assign12920_body16_e16579_d_n12, assign12920_body16_e16579_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign12920_body16_e16567: f64 = (2.0 * locals.var_beta);
        let assign12920_body16_e16569: f64 = (assign12920_body16_e16567 * locals.var_cnst1soi);
        let assign12920_body16_e16572: f64 = (locals.var_t5__blk357 + 1.0);
        let assign12920_body16_e16573: f64 = (assign12920_body16_e16569 * assign12920_body16_e16572);
        let assign12920_body16_e16576: f64 = (2.0 * locals.var_t4__blk356);
        let assign12920_body16_e16577: f64 = (assign12920_body16_e16573 / assign12920_body16_e16576);
        (assign12920_body16_e16577, ((((((assign12920_body16_e16567 * locals.var_cnst1soi_dn0) * assign12920_body16_e16572) + (assign12920_body16_e16569 * locals.var_t5__blk357_dn0)) * assign12920_body16_e16576) - (assign12920_body16_e16573 * (2.0 * locals.var_t4__blk356_dn0))) / (assign12920_body16_e16576 * assign12920_body16_e16576)), ((((((assign12920_body16_e16567 * locals.var_cnst1soi_dn2) * assign12920_body16_e16572) + (assign12920_body16_e16569 * locals.var_t5__blk357_dn2)) * assign12920_body16_e16576) - (assign12920_body16_e16573 * (2.0 * locals.var_t4__blk356_dn2))) / (assign12920_body16_e16576 * assign12920_body16_e16576)), ((((((assign12920_body16_e16567 * locals.var_cnst1soi_dn6) * assign12920_body16_e16572) + (assign12920_body16_e16569 * locals.var_t5__blk357_dn6)) * assign12920_body16_e16576) - (assign12920_body16_e16573 * (2.0 * locals.var_t4__blk356_dn6))) / (assign12920_body16_e16576 * assign12920_body16_e16576)), ((((((assign12920_body16_e16567 * locals.var_cnst1soi_dn7) * assign12920_body16_e16572) + (assign12920_body16_e16569 * locals.var_t5__blk357_dn7)) * assign12920_body16_e16576) - (assign12920_body16_e16573 * (2.0 * locals.var_t4__blk356_dn7))) / (assign12920_body16_e16576 * assign12920_body16_e16576)), ((((((((2.0 * locals.var_beta_dn10) * locals.var_cnst1soi) + (assign12920_body16_e16567 * locals.var_cnst1soi_dn10)) * assign12920_body16_e16572) + (assign12920_body16_e16569 * locals.var_t5__blk357_dn10)) * assign12920_body16_e16576) - (assign12920_body16_e16573 * (2.0 * locals.var_t4__blk356_dn10))) / (assign12920_body16_e16576 * assign12920_body16_e16576)), ((((((assign12920_body16_e16567 * locals.var_cnst1soi_dn11) * assign12920_body16_e16572) + (assign12920_body16_e16569 * locals.var_t5__blk357_dn11)) * assign12920_body16_e16576) - (assign12920_body16_e16573 * (2.0 * locals.var_t4__blk356_dn11))) / (assign12920_body16_e16576 * assign12920_body16_e16576)), ((((((assign12920_body16_e16567 * locals.var_cnst1soi_dn12) * assign12920_body16_e16572) + (assign12920_body16_e16569 * locals.var_t5__blk357_dn12)) * assign12920_body16_e16576) - (assign12920_body16_e16573 * (2.0 * locals.var_t4__blk356_dn12))) / (assign12920_body16_e16576 * assign12920_body16_e16576)), ((((((assign12920_body16_e16567 * locals.var_cnst1soi_dn17) * assign12920_body16_e16572) + (assign12920_body16_e16569 * locals.var_t5__blk357_dn17)) * assign12920_body16_e16576) - (assign12920_body16_e16573 * (2.0 * locals.var_t4__blk356_dn17))) / (assign12920_body16_e16576 * assign12920_body16_e16576)),)
    } else {
        (locals.var_t4_dpss__blk386, locals.var_t4_dpss__blk386_dn0, locals.var_t4_dpss__blk386_dn2, locals.var_t4_dpss__blk386_dn6, locals.var_t4_dpss__blk386_dn7, locals.var_t4_dpss__blk386_dn10, locals.var_t4_dpss__blk386_dn11, locals.var_t4_dpss__blk386_dn12, locals.var_t4_dpss__blk386_dn17,)
    }
};
            locals.var_t4_dpss__blk386 = assign12920_body16_e16579;
            locals.var_t4_dpss__blk386_dn0 = assign12920_body16_e16579_d_n0;
            locals.var_t4_dpss__blk386_dn2 = assign12920_body16_e16579_d_n2;
            locals.var_t4_dpss__blk386_dn6 = assign12920_body16_e16579_d_n6;
            locals.var_t4_dpss__blk386_dn7 = assign12920_body16_e16579_d_n7;
            locals.var_t4_dpss__blk386_dn10 = assign12920_body16_e16579_d_n10;
            locals.var_t4_dpss__blk386_dn11 = assign12920_body16_e16579_d_n11;
            locals.var_t4_dpss__blk386_dn12 = assign12920_body16_e16579_d_n12;
            locals.var_t4_dpss__blk386_dn17 = assign12920_body16_e16579_d_n17;
            let (assign12920_body17_e16591, assign12920_body17_e16591_d_n0, assign12920_body17_e16591_d_n2, assign12920_body17_e16591_d_n6, assign12920_body17_e16591_d_n7, assign12920_body17_e16591_d_n10, assign12920_body17_e16591_d_n11, assign12920_body17_e16591_d_n12, assign12920_body17_e16591_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign12920_body17_e16585: f64 = (-locals.var_cnst0soi);
        let assign12920_body17_e16587: f64 = (assign12920_body17_e16585 * locals.var_t4__blk356);
        let assign12920_body17_e16589: f64 = (assign12920_body17_e16587 - locals.var_q_sl_dep);
        (assign12920_body17_e16589, ((((-locals.var_cnst0soi_dn0) * locals.var_t4__blk356) + (assign12920_body17_e16585 * locals.var_t4__blk356_dn0)) - locals.var_q_sl_dep_dn0), ((((-locals.var_cnst0soi_dn2) * locals.var_t4__blk356) + (assign12920_body17_e16585 * locals.var_t4__blk356_dn2)) - locals.var_q_sl_dep_dn2), ((((-locals.var_cnst0soi_dn6) * locals.var_t4__blk356) + (assign12920_body17_e16585 * locals.var_t4__blk356_dn6)) - locals.var_q_sl_dep_dn6), ((((-locals.var_cnst0soi_dn7) * locals.var_t4__blk356) + (assign12920_body17_e16585 * locals.var_t4__blk356_dn7)) - locals.var_q_sl_dep_dn7), ((((-locals.var_cnst0soi_dn10) * locals.var_t4__blk356) + (assign12920_body17_e16585 * locals.var_t4__blk356_dn10)) - locals.var_q_sl_dep_dn10), ((((-locals.var_cnst0soi_dn11) * locals.var_t4__blk356) + (assign12920_body17_e16585 * locals.var_t4__blk356_dn11)) - locals.var_q_sl_dep_dn11), ((((-locals.var_cnst0soi_dn12) * locals.var_t4__blk356) + (assign12920_body17_e16585 * locals.var_t4__blk356_dn12)) - locals.var_q_sl_dep_dn12), ((((-locals.var_cnst0soi_dn17) * locals.var_t4__blk356) + (assign12920_body17_e16585 * locals.var_t4__blk356_dn17)) - locals.var_q_sl_dep_dn17),)
    } else {
        (locals.var_q_nl, locals.var_q_nl_dn0, locals.var_q_nl_dn2, locals.var_q_nl_dn6, locals.var_q_nl_dn7, locals.var_q_nl_dn10, locals.var_q_nl_dn11, locals.var_q_nl_dn12, locals.var_q_nl_dn17,)
    }
};
            locals.var_q_nl = assign12920_body17_e16591;
            locals.var_q_nl_dn0 = assign12920_body17_e16591_d_n0;
            locals.var_q_nl_dn2 = assign12920_body17_e16591_d_n2;
            locals.var_q_nl_dn6 = assign12920_body17_e16591_d_n6;
            locals.var_q_nl_dn7 = assign12920_body17_e16591_d_n7;
            locals.var_q_nl_dn10 = assign12920_body17_e16591_d_n10;
            locals.var_q_nl_dn11 = assign12920_body17_e16591_d_n11;
            locals.var_q_nl_dn12 = assign12920_body17_e16591_d_n12;
            locals.var_q_nl_dn17 = assign12920_body17_e16591_d_n17;
            let (assign12920_body18_e16601, assign12920_body18_e16601_d_n0, assign12920_body18_e16601_d_n2, assign12920_body18_e16601_d_n6, assign12920_body18_e16601_d_n7, assign12920_body18_e16601_d_n10, assign12920_body18_e16601_d_n11, assign12920_body18_e16601_d_n12, assign12920_body18_e16601_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign12920_body18_e16597: f64 = (-locals.var_cnst0soi);
        let assign12920_body18_e16599: f64 = (assign12920_body18_e16597 * locals.var_t4_dpss__blk386);
        (assign12920_body18_e16599, (((-locals.var_cnst0soi_dn0) * locals.var_t4_dpss__blk386) + (assign12920_body18_e16597 * locals.var_t4_dpss__blk386_dn0)), (((-locals.var_cnst0soi_dn2) * locals.var_t4_dpss__blk386) + (assign12920_body18_e16597 * locals.var_t4_dpss__blk386_dn2)), (((-locals.var_cnst0soi_dn6) * locals.var_t4_dpss__blk386) + (assign12920_body18_e16597 * locals.var_t4_dpss__blk386_dn6)), (((-locals.var_cnst0soi_dn7) * locals.var_t4_dpss__blk386) + (assign12920_body18_e16597 * locals.var_t4_dpss__blk386_dn7)), (((-locals.var_cnst0soi_dn10) * locals.var_t4_dpss__blk386) + (assign12920_body18_e16597 * locals.var_t4_dpss__blk386_dn10)), (((-locals.var_cnst0soi_dn11) * locals.var_t4_dpss__blk386) + (assign12920_body18_e16597 * locals.var_t4_dpss__blk386_dn11)), (((-locals.var_cnst0soi_dn12) * locals.var_t4_dpss__blk386) + (assign12920_body18_e16597 * locals.var_t4_dpss__blk386_dn12)), (((-locals.var_cnst0soi_dn17) * locals.var_t4_dpss__blk386) + (assign12920_body18_e16597 * locals.var_t4_dpss__blk386_dn17)),)
    } else {
        (locals.var_q_nl_dpss, locals.var_q_nl_dpss_dn0, locals.var_q_nl_dpss_dn2, locals.var_q_nl_dpss_dn6, locals.var_q_nl_dpss_dn7, locals.var_q_nl_dpss_dn10, locals.var_q_nl_dpss_dn11, locals.var_q_nl_dpss_dn12, locals.var_q_nl_dpss_dn17,)
    }
};
            locals.var_q_nl_dpss = assign12920_body18_e16601;
            locals.var_q_nl_dpss_dn0 = assign12920_body18_e16601_d_n0;
            locals.var_q_nl_dpss_dn2 = assign12920_body18_e16601_d_n2;
            locals.var_q_nl_dpss_dn6 = assign12920_body18_e16601_d_n6;
            locals.var_q_nl_dpss_dn7 = assign12920_body18_e16601_d_n7;
            locals.var_q_nl_dpss_dn10 = assign12920_body18_e16601_d_n10;
            locals.var_q_nl_dpss_dn11 = assign12920_body18_e16601_d_n11;
            locals.var_q_nl_dpss_dn12 = assign12920_body18_e16601_d_n12;
            locals.var_q_nl_dpss_dn17 = assign12920_body18_e16601_d_n17;
            let (assign12920_body19_e16612, assign12920_body19_e16612_d_n0, assign12920_body19_e16612_d_n2, assign12920_body19_e16612_d_n6, assign12920_body19_e16612_d_n7, assign12920_body19_e16612_d_n10, assign12920_body19_e16612_d_n11, assign12920_body19_e16612_d_n12, assign12920_body19_e16612_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign12920_body19_e16608: f64 = (locals.var_phi_bl_soi - locals.var_phi_sl_soi);
        let assign12920_body19_e16610: f64 = (assign12920_body19_e16608 / locals.var_qdepb_dlt);
        (assign12920_body19_e16610, ((locals.var_phi_bl_soi_dn0 - locals.var_phi_sl_soi_dn0) / locals.var_qdepb_dlt), ((locals.var_phi_bl_soi_dn2 - locals.var_phi_sl_soi_dn2) / locals.var_qdepb_dlt), ((locals.var_phi_bl_soi_dn6 - locals.var_phi_sl_soi_dn6) / locals.var_qdepb_dlt), ((locals.var_phi_bl_soi_dn7 - locals.var_phi_sl_soi_dn7) / locals.var_qdepb_dlt), ((locals.var_phi_bl_soi_dn10 - locals.var_phi_sl_soi_dn10) / locals.var_qdepb_dlt), ((locals.var_phi_bl_soi_dn11 - locals.var_phi_sl_soi_dn11) / locals.var_qdepb_dlt), ((locals.var_phi_bl_soi_dn12 - locals.var_phi_sl_soi_dn12) / locals.var_qdepb_dlt), ((locals.var_phi_bl_soi_dn17 - locals.var_phi_sl_soi_dn17) / locals.var_qdepb_dlt),)
    } else {
        (locals.var_t1__blk353, locals.var_t1__blk353_dn0, locals.var_t1__blk353_dn2, locals.var_t1__blk353_dn6, locals.var_t1__blk353_dn7, locals.var_t1__blk353_dn10, locals.var_t1__blk353_dn11, locals.var_t1__blk353_dn12, locals.var_t1__blk353_dn17,)
    }
};
            locals.var_t1__blk353 = assign12920_body19_e16612;
            locals.var_t1__blk353_dn0 = assign12920_body19_e16612_d_n0;
            locals.var_t1__blk353_dn2 = assign12920_body19_e16612_d_n2;
            locals.var_t1__blk353_dn6 = assign12920_body19_e16612_d_n6;
            locals.var_t1__blk353_dn7 = assign12920_body19_e16612_d_n7;
            locals.var_t1__blk353_dn10 = assign12920_body19_e16612_d_n10;
            locals.var_t1__blk353_dn11 = assign12920_body19_e16612_d_n11;
            locals.var_t1__blk353_dn12 = assign12920_body19_e16612_d_n12;
            locals.var_t1__blk353_dn17 = assign12920_body19_e16612_d_n17;
            let (assign12920_body20_e16621, assign12920_body20_e16621_d_n0, assign12920_body20_e16621_d_n2, assign12920_body20_e16621_d_n6, assign12920_body20_e16621_d_n7, assign12920_body20_e16621_d_n10, assign12920_body20_e16621_d_n11, assign12920_body20_e16621_d_n12, assign12920_body20_e16621_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign12920_body20_e16619: f64 = (locals.var_beta * locals.var_t1__blk353);
        (assign12920_body20_e16619, (locals.var_beta * locals.var_t1__blk353_dn0), (locals.var_beta * locals.var_t1__blk353_dn2), (locals.var_beta * locals.var_t1__blk353_dn6), (locals.var_beta * locals.var_t1__blk353_dn7), ((locals.var_beta_dn10 * locals.var_t1__blk353) + (locals.var_beta * locals.var_t1__blk353_dn10)), (locals.var_beta * locals.var_t1__blk353_dn11), (locals.var_beta * locals.var_t1__blk353_dn12), (locals.var_beta * locals.var_t1__blk353_dn17),)
    } else {
        (locals.var_el, locals.var_el_dn0, locals.var_el_dn2, locals.var_el_dn6, locals.var_el_dn7, locals.var_el_dn10, locals.var_el_dn11, locals.var_el_dn12, locals.var_el_dn17,)
    }
};
            locals.var_el = assign12920_body20_e16621;
            locals.var_el_dn0 = assign12920_body20_e16621_d_n0;
            locals.var_el_dn2 = assign12920_body20_e16621_d_n2;
            locals.var_el_dn6 = assign12920_body20_e16621_d_n6;
            locals.var_el_dn7 = assign12920_body20_e16621_d_n7;
            locals.var_el_dn10 = assign12920_body20_e16621_d_n10;
            locals.var_el_dn11 = assign12920_body20_e16621_d_n11;
            locals.var_el_dn12 = assign12920_body20_e16621_d_n12;
            locals.var_el_dn17 = assign12920_body20_e16621_d_n17;
            let assign12920_body21_e16623: f64 = (-locals.var_el);
            let assign12920_body21_e16625: f64 = if assign12920_body21_e16623 >= 500.0 { 1.0 } else { 0.0 };
            locals.var_guard391 = assign12920_body21_e16625;
            let (assign12920_body22_e16641, assign12920_body22_e16641_d_n0, assign12920_body22_e16641_d_n2, assign12920_body22_e16641_d_n6, assign12920_body22_e16641_d_n7, assign12920_body22_e16641_d_n10, assign12920_body22_e16641_d_n11, assign12920_body22_e16641_d_n12, assign12920_body22_e16641_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard391 != 0.0)) {
        let assign12920_body22_e16635: f64 = (-locals.var_el);
        let assign12920_body22_e16636: f64 = (1.0 + assign12920_body22_e16635);
        let assign12920_body22_e16638: f64 = (assign12920_body22_e16636 - 500.0);
        let assign12920_body22_e16639: f64 = (1.403592217853e217 * assign12920_body22_e16638);
        (assign12920_body22_e16639, (1.403592217853e217 * (-locals.var_el_dn0)), (1.403592217853e217 * (-locals.var_el_dn2)), (1.403592217853e217 * (-locals.var_el_dn6)), (1.403592217853e217 * (-locals.var_el_dn7)), (1.403592217853e217 * (-locals.var_el_dn10)), (1.403592217853e217 * (-locals.var_el_dn11)), (1.403592217853e217 * (-locals.var_el_dn12)), (1.403592217853e217 * (-locals.var_el_dn17)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign12920_body22_e16641;
            locals.var_t0_dn0 = assign12920_body22_e16641_d_n0;
            locals.var_t0_dn2 = assign12920_body22_e16641_d_n2;
            locals.var_t0_dn6 = assign12920_body22_e16641_d_n6;
            locals.var_t0_dn7 = assign12920_body22_e16641_d_n7;
            locals.var_t0_dn10 = assign12920_body22_e16641_d_n10;
            locals.var_t0_dn11 = assign12920_body22_e16641_d_n11;
            locals.var_t0_dn12 = assign12920_body22_e16641_d_n12;
            locals.var_t0_dn17 = assign12920_body22_e16641_d_n17;
            let (assign12920_body23_e16650, assign12920_body23_e16650_d_n0, assign12920_body23_e16650_d_n2, assign12920_body23_e16650_d_n6, assign12920_body23_e16650_d_n7, assign12920_body23_e16650_d_n10, assign12920_body23_e16650_d_n11, assign12920_body23_e16650_d_n12, assign12920_body23_e16650_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard391 != 0.0)) {
        (1.403592217853e217, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17,)
    }
};
            locals.var_t6 = assign12920_body23_e16650;
            locals.var_t6_dn0 = assign12920_body23_e16650_d_n0;
            locals.var_t6_dn2 = assign12920_body23_e16650_d_n2;
            locals.var_t6_dn6 = assign12920_body23_e16650_d_n6;
            locals.var_t6_dn7 = assign12920_body23_e16650_d_n7;
            locals.var_t6_dn10 = assign12920_body23_e16650_d_n10;
            locals.var_t6_dn11 = assign12920_body23_e16650_d_n11;
            locals.var_t6_dn12 = assign12920_body23_e16650_d_n12;
            locals.var_t6_dn17 = assign12920_body23_e16650_d_n17;
            let (assign12920_body24_e16661, assign12920_body24_e16661_d_n0, assign12920_body24_e16661_d_n2, assign12920_body24_e16661_d_n6, assign12920_body24_e16661_d_n7, assign12920_body24_e16661_d_n10, assign12920_body24_e16661_d_n11, assign12920_body24_e16661_d_n12, assign12920_body24_e16661_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard391 == 0.0)) {
        let assign12920_body24_e16659: f64 = (-locals.var_el);
        (assign12920_body24_e16659, (-locals.var_el_dn0), (-locals.var_el_dn2), (-locals.var_el_dn6), (-locals.var_el_dn7), (-locals.var_el_dn10), (-locals.var_el_dn11), (-locals.var_el_dn12), (-locals.var_el_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
            locals.var_tmf1 = assign12920_body24_e16661;
            locals.var_tmf1_dn0 = assign12920_body24_e16661_d_n0;
            locals.var_tmf1_dn2 = assign12920_body24_e16661_d_n2;
            locals.var_tmf1_dn6 = assign12920_body24_e16661_d_n6;
            locals.var_tmf1_dn7 = assign12920_body24_e16661_d_n7;
            locals.var_tmf1_dn10 = assign12920_body24_e16661_d_n10;
            locals.var_tmf1_dn11 = assign12920_body24_e16661_d_n11;
            locals.var_tmf1_dn12 = assign12920_body24_e16661_d_n12;
            locals.var_tmf1_dn17 = assign12920_body24_e16661_d_n17;
            let (assign12920_body25_e16671, assign12920_body25_e16671_d_n0, assign12920_body25_e16671_d_n2, assign12920_body25_e16671_d_n6, assign12920_body25_e16671_d_n7, assign12920_body25_e16671_d_n10, assign12920_body25_e16671_d_n11, assign12920_body25_e16671_d_n12, assign12920_body25_e16671_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard391 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign12920_body25_e16671;
            locals.var_t0_dn0 = assign12920_body25_e16671_d_n0;
            locals.var_t0_dn2 = assign12920_body25_e16671_d_n2;
            locals.var_t0_dn6 = assign12920_body25_e16671_d_n6;
            locals.var_t0_dn7 = assign12920_body25_e16671_d_n7;
            locals.var_t0_dn10 = assign12920_body25_e16671_d_n10;
            locals.var_t0_dn11 = assign12920_body25_e16671_d_n11;
            locals.var_t0_dn12 = assign12920_body25_e16671_d_n12;
            locals.var_t0_dn17 = assign12920_body25_e16671_d_n17;
            let mut assign12920_body26_loop_guard: usize = 0;
            while {
                let assign12920_body26_cond_e16682: f64 = if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard391 == 0.0)) && (locals.var_tmf1 >= 60.0)) { 1.0 } else { 0.0 };
                assign12920_body26_cond_e16682 != 0.0
            } {
                assign12920_body26_loop_guard += 1;
                assert!(assign12920_body26_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                let (assign12920_body26_body0_e16694, assign12920_body26_body0_e16694_d_n0, assign12920_body26_body0_e16694_d_n2, assign12920_body26_body0_e16694_d_n6, assign12920_body26_body0_e16694_d_n7, assign12920_body26_body0_e16694_d_n10, assign12920_body26_body0_e16694_d_n11, assign12920_body26_body0_e16694_d_n12, assign12920_body26_body0_e16694_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard391 == 0.0)) {
        let assign12920_body26_body0_e16692: f64 = (locals.var_t0 * 1.14200738981568e26);
        (assign12920_body26_body0_e16692, (locals.var_t0_dn0 * 1.14200738981568e26), (locals.var_t0_dn2 * 1.14200738981568e26), (locals.var_t0_dn6 * 1.14200738981568e26), (locals.var_t0_dn7 * 1.14200738981568e26), (locals.var_t0_dn10 * 1.14200738981568e26), (locals.var_t0_dn11 * 1.14200738981568e26), (locals.var_t0_dn12 * 1.14200738981568e26), (locals.var_t0_dn17 * 1.14200738981568e26),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
                locals.var_t0 = assign12920_body26_body0_e16694;
                locals.var_t0_dn0 = assign12920_body26_body0_e16694_d_n0;
                locals.var_t0_dn2 = assign12920_body26_body0_e16694_d_n2;
                locals.var_t0_dn6 = assign12920_body26_body0_e16694_d_n6;
                locals.var_t0_dn7 = assign12920_body26_body0_e16694_d_n7;
                locals.var_t0_dn10 = assign12920_body26_body0_e16694_d_n10;
                locals.var_t0_dn11 = assign12920_body26_body0_e16694_d_n11;
                locals.var_t0_dn12 = assign12920_body26_body0_e16694_d_n12;
                locals.var_t0_dn17 = assign12920_body26_body0_e16694_d_n17;
                let (assign12920_body26_body1_e16706, assign12920_body26_body1_e16706_d_n0, assign12920_body26_body1_e16706_d_n2, assign12920_body26_body1_e16706_d_n6, assign12920_body26_body1_e16706_d_n7, assign12920_body26_body1_e16706_d_n10, assign12920_body26_body1_e16706_d_n11, assign12920_body26_body1_e16706_d_n12, assign12920_body26_body1_e16706_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard391 == 0.0)) {
        let assign12920_body26_body1_e16704: f64 = (locals.var_tmf1 - 60.0);
        (assign12920_body26_body1_e16704, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
                locals.var_tmf1 = assign12920_body26_body1_e16706;
                locals.var_tmf1_dn0 = assign12920_body26_body1_e16706_d_n0;
                locals.var_tmf1_dn2 = assign12920_body26_body1_e16706_d_n2;
                locals.var_tmf1_dn6 = assign12920_body26_body1_e16706_d_n6;
                locals.var_tmf1_dn7 = assign12920_body26_body1_e16706_d_n7;
                locals.var_tmf1_dn10 = assign12920_body26_body1_e16706_d_n10;
                locals.var_tmf1_dn11 = assign12920_body26_body1_e16706_d_n11;
                locals.var_tmf1_dn12 = assign12920_body26_body1_e16706_d_n12;
                locals.var_tmf1_dn17 = assign12920_body26_body1_e16706_d_n17;
            }
            let (assign12920_body27_e16719, assign12920_body27_e16719_d_n0, assign12920_body27_e16719_d_n2, assign12920_body27_e16719_d_n6, assign12920_body27_e16719_d_n7, assign12920_body27_e16719_d_n10, assign12920_body27_e16719_d_n11, assign12920_body27_e16719_d_n12, assign12920_body27_e16719_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard391 == 0.0)) {
        let assign12920_body27_e16716: f64 = (locals.var_tmf1).exp();
        let assign12920_body27_e16717: f64 = (locals.var_t0 * assign12920_body27_e16716);
        (assign12920_body27_e16717, ((locals.var_t0_dn0 * assign12920_body27_e16716) + (locals.var_t0 * (assign12920_body27_e16716 * locals.var_tmf1_dn0))), ((locals.var_t0_dn2 * assign12920_body27_e16716) + (locals.var_t0 * (assign12920_body27_e16716 * locals.var_tmf1_dn2))), ((locals.var_t0_dn6 * assign12920_body27_e16716) + (locals.var_t0 * (assign12920_body27_e16716 * locals.var_tmf1_dn6))), ((locals.var_t0_dn7 * assign12920_body27_e16716) + (locals.var_t0 * (assign12920_body27_e16716 * locals.var_tmf1_dn7))), ((locals.var_t0_dn10 * assign12920_body27_e16716) + (locals.var_t0 * (assign12920_body27_e16716 * locals.var_tmf1_dn10))), ((locals.var_t0_dn11 * assign12920_body27_e16716) + (locals.var_t0 * (assign12920_body27_e16716 * locals.var_tmf1_dn11))), ((locals.var_t0_dn12 * assign12920_body27_e16716) + (locals.var_t0 * (assign12920_body27_e16716 * locals.var_tmf1_dn12))), ((locals.var_t0_dn17 * assign12920_body27_e16716) + (locals.var_t0 * (assign12920_body27_e16716 * locals.var_tmf1_dn17))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign12920_body27_e16719;
            locals.var_t0_dn0 = assign12920_body27_e16719_d_n0;
            locals.var_t0_dn2 = assign12920_body27_e16719_d_n2;
            locals.var_t0_dn6 = assign12920_body27_e16719_d_n6;
            locals.var_t0_dn7 = assign12920_body27_e16719_d_n7;
            locals.var_t0_dn10 = assign12920_body27_e16719_d_n10;
            locals.var_t0_dn11 = assign12920_body27_e16719_d_n11;
            locals.var_t0_dn12 = assign12920_body27_e16719_d_n12;
            locals.var_t0_dn17 = assign12920_body27_e16719_d_n17;
            let (assign12920_body28_e16729, assign12920_body28_e16729_d_n0, assign12920_body28_e16729_d_n2, assign12920_body28_e16729_d_n6, assign12920_body28_e16729_d_n7, assign12920_body28_e16729_d_n10, assign12920_body28_e16729_d_n11, assign12920_body28_e16729_d_n12, assign12920_body28_e16729_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard391 == 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17,)
    }
};
            locals.var_t6 = assign12920_body28_e16729;
            locals.var_t6_dn0 = assign12920_body28_e16729_d_n0;
            locals.var_t6_dn2 = assign12920_body28_e16729_d_n2;
            locals.var_t6_dn6 = assign12920_body28_e16729_d_n6;
            locals.var_t6_dn7 = assign12920_body28_e16729_d_n7;
            locals.var_t6_dn10 = assign12920_body28_e16729_d_n10;
            locals.var_t6_dn11 = assign12920_body28_e16729_d_n11;
            locals.var_t6_dn12 = assign12920_body28_e16729_d_n12;
            locals.var_t6_dn17 = assign12920_body28_e16729_d_n17;
            let (assign12920_body29_e16741, assign12920_body29_e16741_d_n0, assign12920_body29_e16741_d_n2, assign12920_body29_e16741_d_n6, assign12920_body29_e16741_d_n7, assign12920_body29_e16741_d_n10, assign12920_body29_e16741_d_n11, assign12920_body29_e16741_d_n12, assign12920_body29_e16741_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign12920_body29_e16736: f64 = (locals.var_t0 + locals.var_el);
        let assign12920_body29_e16738: f64 = (assign12920_body29_e16736 - 1.0);
        let assign12920_body29_e16739: f64 = (assign12920_body29_e16738).sqrt();
        (assign12920_body29_e16739, ((locals.var_t0_dn0 + locals.var_el_dn0) / (2.0 * assign12920_body29_e16739)), ((locals.var_t0_dn2 + locals.var_el_dn2) / (2.0 * assign12920_body29_e16739)), ((locals.var_t0_dn6 + locals.var_el_dn6) / (2.0 * assign12920_body29_e16739)), ((locals.var_t0_dn7 + locals.var_el_dn7) / (2.0 * assign12920_body29_e16739)), ((locals.var_t0_dn10 + locals.var_el_dn10) / (2.0 * assign12920_body29_e16739)), ((locals.var_t0_dn11 + locals.var_el_dn11) / (2.0 * assign12920_body29_e16739)), ((locals.var_t0_dn12 + locals.var_el_dn12) / (2.0 * assign12920_body29_e16739)), ((locals.var_t0_dn17 + locals.var_el_dn17) / (2.0 * assign12920_body29_e16739)),)
    } else {
        (locals.var_t2__blk354, locals.var_t2__blk354_dn0, locals.var_t2__blk354_dn2, locals.var_t2__blk354_dn6, locals.var_t2__blk354_dn7, locals.var_t2__blk354_dn10, locals.var_t2__blk354_dn11, locals.var_t2__blk354_dn12, locals.var_t2__blk354_dn17,)
    }
};
            locals.var_t2__blk354 = assign12920_body29_e16741;
            locals.var_t2__blk354_dn0 = assign12920_body29_e16741_d_n0;
            locals.var_t2__blk354_dn2 = assign12920_body29_e16741_d_n2;
            locals.var_t2__blk354_dn6 = assign12920_body29_e16741_d_n6;
            locals.var_t2__blk354_dn7 = assign12920_body29_e16741_d_n7;
            locals.var_t2__blk354_dn10 = assign12920_body29_e16741_d_n10;
            locals.var_t2__blk354_dn11 = assign12920_body29_e16741_d_n11;
            locals.var_t2__blk354_dn12 = assign12920_body29_e16741_d_n12;
            locals.var_t2__blk354_dn17 = assign12920_body29_e16741_d_n17;
            let assign12920_body30_e16744: f64 = (-1e-9);
            let assign12920_body30_e16745: f64 = if locals.var_t1__blk353 < assign12920_body30_e16744 { 1.0 } else { 0.0 };
            locals.var_guard392 = assign12920_body30_e16745;
            let (assign12920_body31_e16756, assign12920_body31_e16756_d_n0, assign12920_body31_e16756_d_n2, assign12920_body31_e16756_d_n6, assign12920_body31_e16756_d_n7, assign12920_body31_e16756_d_n10, assign12920_body31_e16756_d_n11, assign12920_body31_e16756_d_n12, assign12920_body31_e16756_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard392 != 0.0)) {
        let assign12920_body31_e16754: f64 = (locals.var_cnst0soi * locals.var_t2__blk354);
        (assign12920_body31_e16754, ((locals.var_cnst0soi_dn0 * locals.var_t2__blk354) + (locals.var_cnst0soi * locals.var_t2__blk354_dn0)), ((locals.var_cnst0soi_dn2 * locals.var_t2__blk354) + (locals.var_cnst0soi * locals.var_t2__blk354_dn2)), ((locals.var_cnst0soi_dn6 * locals.var_t2__blk354) + (locals.var_cnst0soi * locals.var_t2__blk354_dn6)), ((locals.var_cnst0soi_dn7 * locals.var_t2__blk354) + (locals.var_cnst0soi * locals.var_t2__blk354_dn7)), ((locals.var_cnst0soi_dn10 * locals.var_t2__blk354) + (locals.var_cnst0soi * locals.var_t2__blk354_dn10)), ((locals.var_cnst0soi_dn11 * locals.var_t2__blk354) + (locals.var_cnst0soi * locals.var_t2__blk354_dn11)), ((locals.var_cnst0soi_dn12 * locals.var_t2__blk354) + (locals.var_cnst0soi * locals.var_t2__blk354_dn12)), ((locals.var_cnst0soi_dn17 * locals.var_t2__blk354) + (locals.var_cnst0soi * locals.var_t2__blk354_dn17)),)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn12, locals.var_q_bl_dep_dn17,)
    }
};
            locals.var_q_bl_dep = assign12920_body31_e16756;
            locals.var_q_bl_dep_dn0 = assign12920_body31_e16756_d_n0;
            locals.var_q_bl_dep_dn2 = assign12920_body31_e16756_d_n2;
            locals.var_q_bl_dep_dn6 = assign12920_body31_e16756_d_n6;
            locals.var_q_bl_dep_dn7 = assign12920_body31_e16756_d_n7;
            locals.var_q_bl_dep_dn10 = assign12920_body31_e16756_d_n10;
            locals.var_q_bl_dep_dn11 = assign12920_body31_e16756_d_n11;
            locals.var_q_bl_dep_dn12 = assign12920_body31_e16756_d_n12;
            locals.var_q_bl_dep_dn17 = assign12920_body31_e16756_d_n17;
            let (assign12920_body32_e16778, assign12920_body32_e16778_d_n0, assign12920_body32_e16778_d_n2, assign12920_body32_e16778_d_n6, assign12920_body32_e16778_d_n7, assign12920_body32_e16778_d_n10, assign12920_body32_e16778_d_n11, assign12920_body32_e16778_d_n12, assign12920_body32_e16778_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard392 != 0.0)) {
        let assign12920_body32_e16765: f64 = (locals.var_cnst0soi * locals.var_beta);
        let assign12920_body32_e16767: f64 = (-locals.var_t6);
        let assign12920_body32_e16769: f64 = (assign12920_body32_e16767 + 1.0);
        let assign12920_body32_e16770: f64 = (assign12920_body32_e16765 * assign12920_body32_e16769);
        let assign12920_body32_e16773: f64 = (2.0 * locals.var_t2__blk354);
        let assign12920_body32_e16774: f64 = (assign12920_body32_e16770 / assign12920_body32_e16773);
        let assign12920_body32_e16776: f64 = (assign12920_body32_e16774 / locals.var_qdepb_dlt);
        (assign12920_body32_e16776, (((((((locals.var_cnst0soi_dn0 * locals.var_beta) * assign12920_body32_e16769) + (assign12920_body32_e16765 * (-locals.var_t6_dn0))) * assign12920_body32_e16773) - (assign12920_body32_e16770 * (2.0 * locals.var_t2__blk354_dn0))) / (assign12920_body32_e16773 * assign12920_body32_e16773)) / locals.var_qdepb_dlt), (((((((locals.var_cnst0soi_dn2 * locals.var_beta) * assign12920_body32_e16769) + (assign12920_body32_e16765 * (-locals.var_t6_dn2))) * assign12920_body32_e16773) - (assign12920_body32_e16770 * (2.0 * locals.var_t2__blk354_dn2))) / (assign12920_body32_e16773 * assign12920_body32_e16773)) / locals.var_qdepb_dlt), (((((((locals.var_cnst0soi_dn6 * locals.var_beta) * assign12920_body32_e16769) + (assign12920_body32_e16765 * (-locals.var_t6_dn6))) * assign12920_body32_e16773) - (assign12920_body32_e16770 * (2.0 * locals.var_t2__blk354_dn6))) / (assign12920_body32_e16773 * assign12920_body32_e16773)) / locals.var_qdepb_dlt), (((((((locals.var_cnst0soi_dn7 * locals.var_beta) * assign12920_body32_e16769) + (assign12920_body32_e16765 * (-locals.var_t6_dn7))) * assign12920_body32_e16773) - (assign12920_body32_e16770 * (2.0 * locals.var_t2__blk354_dn7))) / (assign12920_body32_e16773 * assign12920_body32_e16773)) / locals.var_qdepb_dlt), ((((((((locals.var_cnst0soi_dn10 * locals.var_beta) + (locals.var_cnst0soi * locals.var_beta_dn10)) * assign12920_body32_e16769) + (assign12920_body32_e16765 * (-locals.var_t6_dn10))) * assign12920_body32_e16773) - (assign12920_body32_e16770 * (2.0 * locals.var_t2__blk354_dn10))) / (assign12920_body32_e16773 * assign12920_body32_e16773)) / locals.var_qdepb_dlt), (((((((locals.var_cnst0soi_dn11 * locals.var_beta) * assign12920_body32_e16769) + (assign12920_body32_e16765 * (-locals.var_t6_dn11))) * assign12920_body32_e16773) - (assign12920_body32_e16770 * (2.0 * locals.var_t2__blk354_dn11))) / (assign12920_body32_e16773 * assign12920_body32_e16773)) / locals.var_qdepb_dlt), (((((((locals.var_cnst0soi_dn12 * locals.var_beta) * assign12920_body32_e16769) + (assign12920_body32_e16765 * (-locals.var_t6_dn12))) * assign12920_body32_e16773) - (assign12920_body32_e16770 * (2.0 * locals.var_t2__blk354_dn12))) / (assign12920_body32_e16773 * assign12920_body32_e16773)) / locals.var_qdepb_dlt), (((((((locals.var_cnst0soi_dn17 * locals.var_beta) * assign12920_body32_e16769) + (assign12920_body32_e16765 * (-locals.var_t6_dn17))) * assign12920_body32_e16773) - (assign12920_body32_e16770 * (2.0 * locals.var_t2__blk354_dn17))) / (assign12920_body32_e16773 * assign12920_body32_e16773)) / locals.var_qdepb_dlt),)
    } else {
        (locals.var_q_bl_dep_dpbs, locals.var_q_bl_dep_dpbs_dn0, locals.var_q_bl_dep_dpbs_dn2, locals.var_q_bl_dep_dpbs_dn6, locals.var_q_bl_dep_dpbs_dn7, locals.var_q_bl_dep_dpbs_dn10, locals.var_q_bl_dep_dpbs_dn11, locals.var_q_bl_dep_dpbs_dn12, locals.var_q_bl_dep_dpbs_dn17,)
    }
};
            locals.var_q_bl_dep_dpbs = assign12920_body32_e16778;
            locals.var_q_bl_dep_dpbs_dn0 = assign12920_body32_e16778_d_n0;
            locals.var_q_bl_dep_dpbs_dn2 = assign12920_body32_e16778_d_n2;
            locals.var_q_bl_dep_dpbs_dn6 = assign12920_body32_e16778_d_n6;
            locals.var_q_bl_dep_dpbs_dn7 = assign12920_body32_e16778_d_n7;
            locals.var_q_bl_dep_dpbs_dn10 = assign12920_body32_e16778_d_n10;
            locals.var_q_bl_dep_dpbs_dn11 = assign12920_body32_e16778_d_n11;
            locals.var_q_bl_dep_dpbs_dn12 = assign12920_body32_e16778_d_n12;
            locals.var_q_bl_dep_dpbs_dn17 = assign12920_body32_e16778_d_n17;
            let (assign12920_body33_e16788, assign12920_body33_e16788_d_n0, assign12920_body33_e16788_d_n2, assign12920_body33_e16788_d_n6, assign12920_body33_e16788_d_n7, assign12920_body33_e16788_d_n10, assign12920_body33_e16788_d_n11, assign12920_body33_e16788_d_n12, assign12920_body33_e16788_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard392 != 0.0)) {
        let assign12920_body33_e16786: f64 = (-locals.var_q_bl_dep_dpbs);
        (assign12920_body33_e16786, (-locals.var_q_bl_dep_dpbs_dn0), (-locals.var_q_bl_dep_dpbs_dn2), (-locals.var_q_bl_dep_dpbs_dn6), (-locals.var_q_bl_dep_dpbs_dn7), (-locals.var_q_bl_dep_dpbs_dn10), (-locals.var_q_bl_dep_dpbs_dn11), (-locals.var_q_bl_dep_dpbs_dn12), (-locals.var_q_bl_dep_dpbs_dn17),)
    } else {
        (locals.var_q_bl_dep_dpss, locals.var_q_bl_dep_dpss_dn0, locals.var_q_bl_dep_dpss_dn2, locals.var_q_bl_dep_dpss_dn6, locals.var_q_bl_dep_dpss_dn7, locals.var_q_bl_dep_dpss_dn10, locals.var_q_bl_dep_dpss_dn11, locals.var_q_bl_dep_dpss_dn12, locals.var_q_bl_dep_dpss_dn17,)
    }
};
            locals.var_q_bl_dep_dpss = assign12920_body33_e16788;
            locals.var_q_bl_dep_dpss_dn0 = assign12920_body33_e16788_d_n0;
            locals.var_q_bl_dep_dpss_dn2 = assign12920_body33_e16788_d_n2;
            locals.var_q_bl_dep_dpss_dn6 = assign12920_body33_e16788_d_n6;
            locals.var_q_bl_dep_dpss_dn7 = assign12920_body33_e16788_d_n7;
            locals.var_q_bl_dep_dpss_dn10 = assign12920_body33_e16788_d_n10;
            locals.var_q_bl_dep_dpss_dn11 = assign12920_body33_e16788_d_n11;
            locals.var_q_bl_dep_dpss_dn12 = assign12920_body33_e16788_d_n12;
            locals.var_q_bl_dep_dpss_dn17 = assign12920_body33_e16788_d_n17;
            let assign12920_body34_e16791: f64 = if locals.var_t1__blk353 > 1e-9 { 1.0 } else { 0.0 };
            locals.var_guard393 = assign12920_body34_e16791;
            let (assign12920_body35_e16806, assign12920_body35_e16806_d_n0, assign12920_body35_e16806_d_n2, assign12920_body35_e16806_d_n6, assign12920_body35_e16806_d_n7, assign12920_body35_e16806_d_n10, assign12920_body35_e16806_d_n11, assign12920_body35_e16806_d_n12, assign12920_body35_e16806_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard392 == 0.0)) && (locals.var_guard393 != 0.0)) {
        let assign12920_body35_e16802: f64 = (-locals.var_cnst0soi);
        let assign12920_body35_e16804: f64 = (assign12920_body35_e16802 * locals.var_t2__blk354);
        (assign12920_body35_e16804, (((-locals.var_cnst0soi_dn0) * locals.var_t2__blk354) + (assign12920_body35_e16802 * locals.var_t2__blk354_dn0)), (((-locals.var_cnst0soi_dn2) * locals.var_t2__blk354) + (assign12920_body35_e16802 * locals.var_t2__blk354_dn2)), (((-locals.var_cnst0soi_dn6) * locals.var_t2__blk354) + (assign12920_body35_e16802 * locals.var_t2__blk354_dn6)), (((-locals.var_cnst0soi_dn7) * locals.var_t2__blk354) + (assign12920_body35_e16802 * locals.var_t2__blk354_dn7)), (((-locals.var_cnst0soi_dn10) * locals.var_t2__blk354) + (assign12920_body35_e16802 * locals.var_t2__blk354_dn10)), (((-locals.var_cnst0soi_dn11) * locals.var_t2__blk354) + (assign12920_body35_e16802 * locals.var_t2__blk354_dn11)), (((-locals.var_cnst0soi_dn12) * locals.var_t2__blk354) + (assign12920_body35_e16802 * locals.var_t2__blk354_dn12)), (((-locals.var_cnst0soi_dn17) * locals.var_t2__blk354) + (assign12920_body35_e16802 * locals.var_t2__blk354_dn17)),)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn12, locals.var_q_bl_dep_dn17,)
    }
};
            locals.var_q_bl_dep = assign12920_body35_e16806;
            locals.var_q_bl_dep_dn0 = assign12920_body35_e16806_d_n0;
            locals.var_q_bl_dep_dn2 = assign12920_body35_e16806_d_n2;
            locals.var_q_bl_dep_dn6 = assign12920_body35_e16806_d_n6;
            locals.var_q_bl_dep_dn7 = assign12920_body35_e16806_d_n7;
            locals.var_q_bl_dep_dn10 = assign12920_body35_e16806_d_n10;
            locals.var_q_bl_dep_dn11 = assign12920_body35_e16806_d_n11;
            locals.var_q_bl_dep_dn12 = assign12920_body35_e16806_d_n12;
            locals.var_q_bl_dep_dn17 = assign12920_body35_e16806_d_n17;
            let (assign12920_body36_e16832, assign12920_body36_e16832_d_n0, assign12920_body36_e16832_d_n2, assign12920_body36_e16832_d_n6, assign12920_body36_e16832_d_n7, assign12920_body36_e16832_d_n10, assign12920_body36_e16832_d_n11, assign12920_body36_e16832_d_n12, assign12920_body36_e16832_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard392 == 0.0)) && (locals.var_guard393 != 0.0)) {
        let assign12920_body36_e16817: f64 = (-locals.var_cnst0soi);
        let assign12920_body36_e16819: f64 = (assign12920_body36_e16817 * locals.var_beta);
        let assign12920_body36_e16821: f64 = (-locals.var_t6);
        let assign12920_body36_e16823: f64 = (assign12920_body36_e16821 + 1.0);
        let assign12920_body36_e16824: f64 = (assign12920_body36_e16819 * assign12920_body36_e16823);
        let assign12920_body36_e16827: f64 = (2.0 * locals.var_t2__blk354);
        let assign12920_body36_e16828: f64 = (assign12920_body36_e16824 / assign12920_body36_e16827);
        let assign12920_body36_e16830: f64 = (assign12920_body36_e16828 / locals.var_qdepb_dlt);
        (assign12920_body36_e16830, ((((((((-locals.var_cnst0soi_dn0) * locals.var_beta) * assign12920_body36_e16823) + (assign12920_body36_e16819 * (-locals.var_t6_dn0))) * assign12920_body36_e16827) - (assign12920_body36_e16824 * (2.0 * locals.var_t2__blk354_dn0))) / (assign12920_body36_e16827 * assign12920_body36_e16827)) / locals.var_qdepb_dlt), ((((((((-locals.var_cnst0soi_dn2) * locals.var_beta) * assign12920_body36_e16823) + (assign12920_body36_e16819 * (-locals.var_t6_dn2))) * assign12920_body36_e16827) - (assign12920_body36_e16824 * (2.0 * locals.var_t2__blk354_dn2))) / (assign12920_body36_e16827 * assign12920_body36_e16827)) / locals.var_qdepb_dlt), ((((((((-locals.var_cnst0soi_dn6) * locals.var_beta) * assign12920_body36_e16823) + (assign12920_body36_e16819 * (-locals.var_t6_dn6))) * assign12920_body36_e16827) - (assign12920_body36_e16824 * (2.0 * locals.var_t2__blk354_dn6))) / (assign12920_body36_e16827 * assign12920_body36_e16827)) / locals.var_qdepb_dlt), ((((((((-locals.var_cnst0soi_dn7) * locals.var_beta) * assign12920_body36_e16823) + (assign12920_body36_e16819 * (-locals.var_t6_dn7))) * assign12920_body36_e16827) - (assign12920_body36_e16824 * (2.0 * locals.var_t2__blk354_dn7))) / (assign12920_body36_e16827 * assign12920_body36_e16827)) / locals.var_qdepb_dlt), (((((((((-locals.var_cnst0soi_dn10) * locals.var_beta) + (assign12920_body36_e16817 * locals.var_beta_dn10)) * assign12920_body36_e16823) + (assign12920_body36_e16819 * (-locals.var_t6_dn10))) * assign12920_body36_e16827) - (assign12920_body36_e16824 * (2.0 * locals.var_t2__blk354_dn10))) / (assign12920_body36_e16827 * assign12920_body36_e16827)) / locals.var_qdepb_dlt), ((((((((-locals.var_cnst0soi_dn11) * locals.var_beta) * assign12920_body36_e16823) + (assign12920_body36_e16819 * (-locals.var_t6_dn11))) * assign12920_body36_e16827) - (assign12920_body36_e16824 * (2.0 * locals.var_t2__blk354_dn11))) / (assign12920_body36_e16827 * assign12920_body36_e16827)) / locals.var_qdepb_dlt), ((((((((-locals.var_cnst0soi_dn12) * locals.var_beta) * assign12920_body36_e16823) + (assign12920_body36_e16819 * (-locals.var_t6_dn12))) * assign12920_body36_e16827) - (assign12920_body36_e16824 * (2.0 * locals.var_t2__blk354_dn12))) / (assign12920_body36_e16827 * assign12920_body36_e16827)) / locals.var_qdepb_dlt), ((((((((-locals.var_cnst0soi_dn17) * locals.var_beta) * assign12920_body36_e16823) + (assign12920_body36_e16819 * (-locals.var_t6_dn17))) * assign12920_body36_e16827) - (assign12920_body36_e16824 * (2.0 * locals.var_t2__blk354_dn17))) / (assign12920_body36_e16827 * assign12920_body36_e16827)) / locals.var_qdepb_dlt),)
    } else {
        (locals.var_q_bl_dep_dpbs, locals.var_q_bl_dep_dpbs_dn0, locals.var_q_bl_dep_dpbs_dn2, locals.var_q_bl_dep_dpbs_dn6, locals.var_q_bl_dep_dpbs_dn7, locals.var_q_bl_dep_dpbs_dn10, locals.var_q_bl_dep_dpbs_dn11, locals.var_q_bl_dep_dpbs_dn12, locals.var_q_bl_dep_dpbs_dn17,)
    }
};
            locals.var_q_bl_dep_dpbs = assign12920_body36_e16832;
            locals.var_q_bl_dep_dpbs_dn0 = assign12920_body36_e16832_d_n0;
            locals.var_q_bl_dep_dpbs_dn2 = assign12920_body36_e16832_d_n2;
            locals.var_q_bl_dep_dpbs_dn6 = assign12920_body36_e16832_d_n6;
            locals.var_q_bl_dep_dpbs_dn7 = assign12920_body36_e16832_d_n7;
            locals.var_q_bl_dep_dpbs_dn10 = assign12920_body36_e16832_d_n10;
            locals.var_q_bl_dep_dpbs_dn11 = assign12920_body36_e16832_d_n11;
            locals.var_q_bl_dep_dpbs_dn12 = assign12920_body36_e16832_d_n12;
            locals.var_q_bl_dep_dpbs_dn17 = assign12920_body36_e16832_d_n17;
            let (assign12920_body37_e16845, assign12920_body37_e16845_d_n0, assign12920_body37_e16845_d_n2, assign12920_body37_e16845_d_n6, assign12920_body37_e16845_d_n7, assign12920_body37_e16845_d_n10, assign12920_body37_e16845_d_n11, assign12920_body37_e16845_d_n12, assign12920_body37_e16845_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard392 == 0.0)) && (locals.var_guard393 != 0.0)) {
        let assign12920_body37_e16843: f64 = (-locals.var_q_bl_dep_dpbs);
        (assign12920_body37_e16843, (-locals.var_q_bl_dep_dpbs_dn0), (-locals.var_q_bl_dep_dpbs_dn2), (-locals.var_q_bl_dep_dpbs_dn6), (-locals.var_q_bl_dep_dpbs_dn7), (-locals.var_q_bl_dep_dpbs_dn10), (-locals.var_q_bl_dep_dpbs_dn11), (-locals.var_q_bl_dep_dpbs_dn12), (-locals.var_q_bl_dep_dpbs_dn17),)
    } else {
        (locals.var_q_bl_dep_dpss, locals.var_q_bl_dep_dpss_dn0, locals.var_q_bl_dep_dpss_dn2, locals.var_q_bl_dep_dpss_dn6, locals.var_q_bl_dep_dpss_dn7, locals.var_q_bl_dep_dpss_dn10, locals.var_q_bl_dep_dpss_dn11, locals.var_q_bl_dep_dpss_dn12, locals.var_q_bl_dep_dpss_dn17,)
    }
};
            locals.var_q_bl_dep_dpss = assign12920_body37_e16845;
            locals.var_q_bl_dep_dpss_dn0 = assign12920_body37_e16845_d_n0;
            locals.var_q_bl_dep_dpss_dn2 = assign12920_body37_e16845_d_n2;
            locals.var_q_bl_dep_dpss_dn6 = assign12920_body37_e16845_d_n6;
            locals.var_q_bl_dep_dpss_dn7 = assign12920_body37_e16845_d_n7;
            locals.var_q_bl_dep_dpss_dn10 = assign12920_body37_e16845_d_n10;
            locals.var_q_bl_dep_dpss_dn11 = assign12920_body37_e16845_d_n11;
            locals.var_q_bl_dep_dpss_dn12 = assign12920_body37_e16845_d_n12;
            locals.var_q_bl_dep_dpss_dn17 = assign12920_body37_e16845_d_n17;
            let (assign12920_body38_e16863, assign12920_body38_e16863_d_n0, assign12920_body38_e16863_d_n2, assign12920_body38_e16863_d_n6, assign12920_body38_e16863_d_n7, assign12920_body38_e16863_d_n10, assign12920_body38_e16863_d_n11, assign12920_body38_e16863_d_n12, assign12920_body38_e16863_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard392 == 0.0)) && (locals.var_guard393 == 0.0)) {
        let assign12920_body38_e16857: f64 = (-locals.var_cnst0soi);
        let assign12920_body38_e16859: f64 = (assign12920_body38_e16857 * locals.var_el);
        let assign12920_body38_e16861: f64 = (assign12920_body38_e16859 / 1.414213562373095);
        (assign12920_body38_e16861, ((((-locals.var_cnst0soi_dn0) * locals.var_el) + (assign12920_body38_e16857 * locals.var_el_dn0)) / 1.414213562373095), ((((-locals.var_cnst0soi_dn2) * locals.var_el) + (assign12920_body38_e16857 * locals.var_el_dn2)) / 1.414213562373095), ((((-locals.var_cnst0soi_dn6) * locals.var_el) + (assign12920_body38_e16857 * locals.var_el_dn6)) / 1.414213562373095), ((((-locals.var_cnst0soi_dn7) * locals.var_el) + (assign12920_body38_e16857 * locals.var_el_dn7)) / 1.414213562373095), ((((-locals.var_cnst0soi_dn10) * locals.var_el) + (assign12920_body38_e16857 * locals.var_el_dn10)) / 1.414213562373095), ((((-locals.var_cnst0soi_dn11) * locals.var_el) + (assign12920_body38_e16857 * locals.var_el_dn11)) / 1.414213562373095), ((((-locals.var_cnst0soi_dn12) * locals.var_el) + (assign12920_body38_e16857 * locals.var_el_dn12)) / 1.414213562373095), ((((-locals.var_cnst0soi_dn17) * locals.var_el) + (assign12920_body38_e16857 * locals.var_el_dn17)) / 1.414213562373095),)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn12, locals.var_q_bl_dep_dn17,)
    }
};
            locals.var_q_bl_dep = assign12920_body38_e16863;
            locals.var_q_bl_dep_dn0 = assign12920_body38_e16863_d_n0;
            locals.var_q_bl_dep_dn2 = assign12920_body38_e16863_d_n2;
            locals.var_q_bl_dep_dn6 = assign12920_body38_e16863_d_n6;
            locals.var_q_bl_dep_dn7 = assign12920_body38_e16863_d_n7;
            locals.var_q_bl_dep_dn10 = assign12920_body38_e16863_d_n10;
            locals.var_q_bl_dep_dn11 = assign12920_body38_e16863_d_n11;
            locals.var_q_bl_dep_dn12 = assign12920_body38_e16863_d_n12;
            locals.var_q_bl_dep_dn17 = assign12920_body38_e16863_d_n17;
            let (assign12920_body39_e16881, assign12920_body39_e16881_d_n0, assign12920_body39_e16881_d_n2, assign12920_body39_e16881_d_n6, assign12920_body39_e16881_d_n7, assign12920_body39_e16881_d_n10, assign12920_body39_e16881_d_n11, assign12920_body39_e16881_d_n12, assign12920_body39_e16881_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard392 == 0.0)) && (locals.var_guard393 == 0.0)) {
        let assign12920_body39_e16875: f64 = (-locals.var_cnst0soi);
        let assign12920_body39_e16877: f64 = (assign12920_body39_e16875 * locals.var_beta);
        let assign12920_body39_e16879: f64 = (assign12920_body39_e16877 / 1.414213562373095);
        (assign12920_body39_e16879, (((-locals.var_cnst0soi_dn0) * locals.var_beta) / 1.414213562373095), (((-locals.var_cnst0soi_dn2) * locals.var_beta) / 1.414213562373095), (((-locals.var_cnst0soi_dn6) * locals.var_beta) / 1.414213562373095), (((-locals.var_cnst0soi_dn7) * locals.var_beta) / 1.414213562373095), ((((-locals.var_cnst0soi_dn10) * locals.var_beta) + (assign12920_body39_e16875 * locals.var_beta_dn10)) / 1.414213562373095), (((-locals.var_cnst0soi_dn11) * locals.var_beta) / 1.414213562373095), (((-locals.var_cnst0soi_dn12) * locals.var_beta) / 1.414213562373095), (((-locals.var_cnst0soi_dn17) * locals.var_beta) / 1.414213562373095),)
    } else {
        (locals.var_q_bl_dep_dpbs, locals.var_q_bl_dep_dpbs_dn0, locals.var_q_bl_dep_dpbs_dn2, locals.var_q_bl_dep_dpbs_dn6, locals.var_q_bl_dep_dpbs_dn7, locals.var_q_bl_dep_dpbs_dn10, locals.var_q_bl_dep_dpbs_dn11, locals.var_q_bl_dep_dpbs_dn12, locals.var_q_bl_dep_dpbs_dn17,)
    }
};
            locals.var_q_bl_dep_dpbs = assign12920_body39_e16881;
            locals.var_q_bl_dep_dpbs_dn0 = assign12920_body39_e16881_d_n0;
            locals.var_q_bl_dep_dpbs_dn2 = assign12920_body39_e16881_d_n2;
            locals.var_q_bl_dep_dpbs_dn6 = assign12920_body39_e16881_d_n6;
            locals.var_q_bl_dep_dpbs_dn7 = assign12920_body39_e16881_d_n7;
            locals.var_q_bl_dep_dpbs_dn10 = assign12920_body39_e16881_d_n10;
            locals.var_q_bl_dep_dpbs_dn11 = assign12920_body39_e16881_d_n11;
            locals.var_q_bl_dep_dpbs_dn12 = assign12920_body39_e16881_d_n12;
            locals.var_q_bl_dep_dpbs_dn17 = assign12920_body39_e16881_d_n17;
            let (assign12920_body40_e16895, assign12920_body40_e16895_d_n0, assign12920_body40_e16895_d_n2, assign12920_body40_e16895_d_n6, assign12920_body40_e16895_d_n7, assign12920_body40_e16895_d_n10, assign12920_body40_e16895_d_n11, assign12920_body40_e16895_d_n12, assign12920_body40_e16895_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard392 == 0.0)) && (locals.var_guard393 == 0.0)) {
        let assign12920_body40_e16893: f64 = (-locals.var_q_bl_dep_dpbs);
        (assign12920_body40_e16893, (-locals.var_q_bl_dep_dpbs_dn0), (-locals.var_q_bl_dep_dpbs_dn2), (-locals.var_q_bl_dep_dpbs_dn6), (-locals.var_q_bl_dep_dpbs_dn7), (-locals.var_q_bl_dep_dpbs_dn10), (-locals.var_q_bl_dep_dpbs_dn11), (-locals.var_q_bl_dep_dpbs_dn12), (-locals.var_q_bl_dep_dpbs_dn17),)
    } else {
        (locals.var_q_bl_dep_dpss, locals.var_q_bl_dep_dpss_dn0, locals.var_q_bl_dep_dpss_dn2, locals.var_q_bl_dep_dpss_dn6, locals.var_q_bl_dep_dpss_dn7, locals.var_q_bl_dep_dpss_dn10, locals.var_q_bl_dep_dpss_dn11, locals.var_q_bl_dep_dpss_dn12, locals.var_q_bl_dep_dpss_dn17,)
    }
};
            locals.var_q_bl_dep_dpss = assign12920_body40_e16895;
            locals.var_q_bl_dep_dpss_dn0 = assign12920_body40_e16895_d_n0;
            locals.var_q_bl_dep_dpss_dn2 = assign12920_body40_e16895_d_n2;
            locals.var_q_bl_dep_dpss_dn6 = assign12920_body40_e16895_d_n6;
            locals.var_q_bl_dep_dpss_dn7 = assign12920_body40_e16895_d_n7;
            locals.var_q_bl_dep_dpss_dn10 = assign12920_body40_e16895_d_n10;
            locals.var_q_bl_dep_dpss_dn11 = assign12920_body40_e16895_d_n11;
            locals.var_q_bl_dep_dpss_dn12 = assign12920_body40_e16895_d_n12;
            locals.var_q_bl_dep_dpss_dn17 = assign12920_body40_e16895_d_n17;
            let assign12920_body41_e16899: f64 = (-locals.var_q_wdsoi_max);
            let assign12920_body41_e16901: f64 = assign12920_body41_e16899;
            let assign12920_body41_e16902: f64 = (-assign12920_body41_e16901);
            let assign12920_body41_e16905: f64 = (-locals.var_q_wdsoi_max);
            let assign12920_body41_e16907: f64 = assign12920_body41_e16905;
            let assign12920_body41_e16910: f64 = if ((locals.var_q_bl_dep > assign12920_body41_e16902) && (assign12920_body41_e16907 >= 0.0)) { 1.0 } else { 0.0 };
            locals.var_guard394 = assign12920_body41_e16910;
            let (assign12920_body42_e16926, assign12920_body42_e16926_d_n0, assign12920_body42_e16926_d_n2, assign12920_body42_e16926_d_n6, assign12920_body42_e16926_d_n7, assign12920_body42_e16926_d_n10, assign12920_body42_e16926_d_n11, assign12920_body42_e16926_d_n12, assign12920_body42_e16926_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard394 != 0.0)) {
        let assign12920_body42_e16919: f64 = locals.var_q_bl_dep;
        let assign12920_body42_e16921: f64 = (-locals.var_q_wdsoi_max);
        let assign12920_body42_e16923: f64 = assign12920_body42_e16921;
        let assign12920_body42_e16924: f64 = (assign12920_body42_e16919 + assign12920_body42_e16923);
        (assign12920_body42_e16924, (locals.var_q_bl_dep_dn0 + (-locals.var_q_wdsoi_max_dn0)), (locals.var_q_bl_dep_dn2 + (-locals.var_q_wdsoi_max_dn2)), (locals.var_q_bl_dep_dn6 + (-locals.var_q_wdsoi_max_dn6)), (locals.var_q_bl_dep_dn7 + (-locals.var_q_wdsoi_max_dn7)), (locals.var_q_bl_dep_dn10 + (-locals.var_q_wdsoi_max_dn10)), (locals.var_q_bl_dep_dn11 + (-locals.var_q_wdsoi_max_dn11)), (locals.var_q_bl_dep_dn12 + (-locals.var_q_wdsoi_max_dn12)), (locals.var_q_bl_dep_dn17 + (-locals.var_q_wdsoi_max_dn17)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
            locals.var_tmf1 = assign12920_body42_e16926;
            locals.var_tmf1_dn0 = assign12920_body42_e16926_d_n0;
            locals.var_tmf1_dn2 = assign12920_body42_e16926_d_n2;
            locals.var_tmf1_dn6 = assign12920_body42_e16926_d_n6;
            locals.var_tmf1_dn7 = assign12920_body42_e16926_d_n7;
            locals.var_tmf1_dn10 = assign12920_body42_e16926_d_n10;
            locals.var_tmf1_dn11 = assign12920_body42_e16926_d_n11;
            locals.var_tmf1_dn12 = assign12920_body42_e16926_d_n12;
            locals.var_tmf1_dn17 = assign12920_body42_e16926_d_n17;
            let (assign12920_body43_e16937, assign12920_body43_e16937_d_n0, assign12920_body43_e16937_d_n2, assign12920_body43_e16937_d_n6, assign12920_body43_e16937_d_n7, assign12920_body43_e16937_d_n10, assign12920_body43_e16937_d_n11, assign12920_body43_e16937_d_n12, assign12920_body43_e16937_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard394 != 0.0)) {
        let assign12920_body43_e16935: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign12920_body43_e16935, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
            locals.var_x2 = assign12920_body43_e16937;
            locals.var_x2_dn0 = assign12920_body43_e16937_d_n0;
            locals.var_x2_dn2 = assign12920_body43_e16937_d_n2;
            locals.var_x2_dn6 = assign12920_body43_e16937_d_n6;
            locals.var_x2_dn7 = assign12920_body43_e16937_d_n7;
            locals.var_x2_dn10 = assign12920_body43_e16937_d_n10;
            locals.var_x2_dn11 = assign12920_body43_e16937_d_n11;
            locals.var_x2_dn12 = assign12920_body43_e16937_d_n12;
            locals.var_x2_dn17 = assign12920_body43_e16937_d_n17;
            let (assign12920_body44_e16954, assign12920_body44_e16954_d_n0, assign12920_body44_e16954_d_n2, assign12920_body44_e16954_d_n6, assign12920_body44_e16954_d_n7, assign12920_body44_e16954_d_n10, assign12920_body44_e16954_d_n11, assign12920_body44_e16954_d_n12, assign12920_body44_e16954_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard394 != 0.0)) {
        let assign12920_body44_e16945: f64 = (-locals.var_q_wdsoi_max);
        let assign12920_body44_e16947: f64 = assign12920_body44_e16945;
        let assign12920_body44_e16949: f64 = (-locals.var_q_wdsoi_max);
        let assign12920_body44_e16951: f64 = assign12920_body44_e16949;
        let assign12920_body44_e16952: f64 = (assign12920_body44_e16947 * assign12920_body44_e16951);
        (assign12920_body44_e16952, (((-locals.var_q_wdsoi_max_dn0) * assign12920_body44_e16951) + (assign12920_body44_e16947 * (-locals.var_q_wdsoi_max_dn0))), (((-locals.var_q_wdsoi_max_dn2) * assign12920_body44_e16951) + (assign12920_body44_e16947 * (-locals.var_q_wdsoi_max_dn2))), (((-locals.var_q_wdsoi_max_dn6) * assign12920_body44_e16951) + (assign12920_body44_e16947 * (-locals.var_q_wdsoi_max_dn6))), (((-locals.var_q_wdsoi_max_dn7) * assign12920_body44_e16951) + (assign12920_body44_e16947 * (-locals.var_q_wdsoi_max_dn7))), (((-locals.var_q_wdsoi_max_dn10) * assign12920_body44_e16951) + (assign12920_body44_e16947 * (-locals.var_q_wdsoi_max_dn10))), (((-locals.var_q_wdsoi_max_dn11) * assign12920_body44_e16951) + (assign12920_body44_e16947 * (-locals.var_q_wdsoi_max_dn11))), (((-locals.var_q_wdsoi_max_dn12) * assign12920_body44_e16951) + (assign12920_body44_e16947 * (-locals.var_q_wdsoi_max_dn12))), (((-locals.var_q_wdsoi_max_dn17) * assign12920_body44_e16951) + (assign12920_body44_e16947 * (-locals.var_q_wdsoi_max_dn17))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
            locals.var_xmax2 = assign12920_body44_e16954;
            locals.var_xmax2_dn0 = assign12920_body44_e16954_d_n0;
            locals.var_xmax2_dn2 = assign12920_body44_e16954_d_n2;
            locals.var_xmax2_dn6 = assign12920_body44_e16954_d_n6;
            locals.var_xmax2_dn7 = assign12920_body44_e16954_d_n7;
            locals.var_xmax2_dn10 = assign12920_body44_e16954_d_n10;
            locals.var_xmax2_dn11 = assign12920_body44_e16954_d_n11;
            locals.var_xmax2_dn12 = assign12920_body44_e16954_d_n12;
            locals.var_xmax2_dn17 = assign12920_body44_e16954_d_n17;
            let (assign12920_body45_e16963, assign12920_body45_e16963_d_n0, assign12920_body45_e16963_d_n2, assign12920_body45_e16963_d_n6, assign12920_body45_e16963_d_n7, assign12920_body45_e16963_d_n10, assign12920_body45_e16963_d_n11, assign12920_body45_e16963_d_n12, assign12920_body45_e16963_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard394 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
            locals.var_xp = assign12920_body45_e16963;
            locals.var_xp_dn0 = assign12920_body45_e16963_d_n0;
            locals.var_xp_dn2 = assign12920_body45_e16963_d_n2;
            locals.var_xp_dn6 = assign12920_body45_e16963_d_n6;
            locals.var_xp_dn7 = assign12920_body45_e16963_d_n7;
            locals.var_xp_dn10 = assign12920_body45_e16963_d_n10;
            locals.var_xp_dn11 = assign12920_body45_e16963_d_n11;
            locals.var_xp_dn12 = assign12920_body45_e16963_d_n12;
            locals.var_xp_dn17 = assign12920_body45_e16963_d_n17;
            let (assign12920_body46_e16972, assign12920_body46_e16972_d_n0, assign12920_body46_e16972_d_n2, assign12920_body46_e16972_d_n6, assign12920_body46_e16972_d_n7, assign12920_body46_e16972_d_n10, assign12920_body46_e16972_d_n11, assign12920_body46_e16972_d_n12, assign12920_body46_e16972_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard394 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
            locals.var_xmp = assign12920_body46_e16972;
            locals.var_xmp_dn0 = assign12920_body46_e16972_d_n0;
            locals.var_xmp_dn2 = assign12920_body46_e16972_d_n2;
            locals.var_xmp_dn6 = assign12920_body46_e16972_d_n6;
            locals.var_xmp_dn7 = assign12920_body46_e16972_d_n7;
            locals.var_xmp_dn10 = assign12920_body46_e16972_d_n10;
            locals.var_xmp_dn11 = assign12920_body46_e16972_d_n11;
            locals.var_xmp_dn12 = assign12920_body46_e16972_d_n12;
            locals.var_xmp_dn17 = assign12920_body46_e16972_d_n17;
            let (assign12920_body47_e16981,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard394 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign12920_body47_e16981;
            let (assign12920_body48_e16990,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard394 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign12920_body48_e16990;
            let (assign12920_body49_e16999, assign12920_body49_e16999_d_n0, assign12920_body49_e16999_d_n2, assign12920_body49_e16999_d_n6, assign12920_body49_e16999_d_n7, assign12920_body49_e16999_d_n10, assign12920_body49_e16999_d_n11, assign12920_body49_e16999_d_n12, assign12920_body49_e16999_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard394 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
            locals.var_arg = assign12920_body49_e16999;
            locals.var_arg_dn0 = assign12920_body49_e16999_d_n0;
            locals.var_arg_dn2 = assign12920_body49_e16999_d_n2;
            locals.var_arg_dn6 = assign12920_body49_e16999_d_n6;
            locals.var_arg_dn7 = assign12920_body49_e16999_d_n7;
            locals.var_arg_dn10 = assign12920_body49_e16999_d_n10;
            locals.var_arg_dn11 = assign12920_body49_e16999_d_n11;
            locals.var_arg_dn12 = assign12920_body49_e16999_d_n12;
            locals.var_arg_dn17 = assign12920_body49_e16999_d_n17;
            let (assign12920_body50_e17008, assign12920_body50_e17008_d_n0, assign12920_body50_e17008_d_n2, assign12920_body50_e17008_d_n6, assign12920_body50_e17008_d_n7, assign12920_body50_e17008_d_n10, assign12920_body50_e17008_d_n11, assign12920_body50_e17008_d_n12, assign12920_body50_e17008_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard394 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign12920_body50_e17008;
            locals.var_dnm_dn0 = assign12920_body50_e17008_d_n0;
            locals.var_dnm_dn2 = assign12920_body50_e17008_d_n2;
            locals.var_dnm_dn6 = assign12920_body50_e17008_d_n6;
            locals.var_dnm_dn7 = assign12920_body50_e17008_d_n7;
            locals.var_dnm_dn10 = assign12920_body50_e17008_d_n10;
            locals.var_dnm_dn11 = assign12920_body50_e17008_d_n11;
            locals.var_dnm_dn12 = assign12920_body50_e17008_d_n12;
            locals.var_dnm_dn17 = assign12920_body50_e17008_d_n17;
            let (assign12920_body51_e17019, assign12920_body51_e17019_d_n0, assign12920_body51_e17019_d_n2, assign12920_body51_e17019_d_n6, assign12920_body51_e17019_d_n7, assign12920_body51_e17019_d_n10, assign12920_body51_e17019_d_n11, assign12920_body51_e17019_d_n12, assign12920_body51_e17019_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard394 != 0.0)) {
        let assign12920_body51_e17017: f64 = (locals.var_xp * locals.var_x2);
        (assign12920_body51_e17017, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
            locals.var_xp = assign12920_body51_e17019;
            locals.var_xp_dn0 = assign12920_body51_e17019_d_n0;
            locals.var_xp_dn2 = assign12920_body51_e17019_d_n2;
            locals.var_xp_dn6 = assign12920_body51_e17019_d_n6;
            locals.var_xp_dn7 = assign12920_body51_e17019_d_n7;
            locals.var_xp_dn10 = assign12920_body51_e17019_d_n10;
            locals.var_xp_dn11 = assign12920_body51_e17019_d_n11;
            locals.var_xp_dn12 = assign12920_body51_e17019_d_n12;
            locals.var_xp_dn17 = assign12920_body51_e17019_d_n17;
            let (assign12920_body52_e17030, assign12920_body52_e17030_d_n0, assign12920_body52_e17030_d_n2, assign12920_body52_e17030_d_n6, assign12920_body52_e17030_d_n7, assign12920_body52_e17030_d_n10, assign12920_body52_e17030_d_n11, assign12920_body52_e17030_d_n12, assign12920_body52_e17030_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard394 != 0.0)) {
        let assign12920_body52_e17028: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign12920_body52_e17028, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
            locals.var_xmp = assign12920_body52_e17030;
            locals.var_xmp_dn0 = assign12920_body52_e17030_d_n0;
            locals.var_xmp_dn2 = assign12920_body52_e17030_d_n2;
            locals.var_xmp_dn6 = assign12920_body52_e17030_d_n6;
            locals.var_xmp_dn7 = assign12920_body52_e17030_d_n7;
            locals.var_xmp_dn10 = assign12920_body52_e17030_d_n10;
            locals.var_xmp_dn11 = assign12920_body52_e17030_d_n11;
            locals.var_xmp_dn12 = assign12920_body52_e17030_d_n12;
            locals.var_xmp_dn17 = assign12920_body52_e17030_d_n17;
            let (assign12920_body53_e17041, assign12920_body53_e17041_d_n0, assign12920_body53_e17041_d_n2, assign12920_body53_e17041_d_n6, assign12920_body53_e17041_d_n7, assign12920_body53_e17041_d_n10, assign12920_body53_e17041_d_n11, assign12920_body53_e17041_d_n12, assign12920_body53_e17041_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard394 != 0.0)) {
        let assign12920_body53_e17039: f64 = (locals.var_xp * locals.var_x2);
        (assign12920_body53_e17039, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
            locals.var_xp = assign12920_body53_e17041;
            locals.var_xp_dn0 = assign12920_body53_e17041_d_n0;
            locals.var_xp_dn2 = assign12920_body53_e17041_d_n2;
            locals.var_xp_dn6 = assign12920_body53_e17041_d_n6;
            locals.var_xp_dn7 = assign12920_body53_e17041_d_n7;
            locals.var_xp_dn10 = assign12920_body53_e17041_d_n10;
            locals.var_xp_dn11 = assign12920_body53_e17041_d_n11;
            locals.var_xp_dn12 = assign12920_body53_e17041_d_n12;
            locals.var_xp_dn17 = assign12920_body53_e17041_d_n17;
            let (assign12920_body54_e17052, assign12920_body54_e17052_d_n0, assign12920_body54_e17052_d_n2, assign12920_body54_e17052_d_n6, assign12920_body54_e17052_d_n7, assign12920_body54_e17052_d_n10, assign12920_body54_e17052_d_n11, assign12920_body54_e17052_d_n12, assign12920_body54_e17052_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard394 != 0.0)) {
        let assign12920_body54_e17050: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign12920_body54_e17050, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
            locals.var_xmp = assign12920_body54_e17052;
            locals.var_xmp_dn0 = assign12920_body54_e17052_d_n0;
            locals.var_xmp_dn2 = assign12920_body54_e17052_d_n2;
            locals.var_xmp_dn6 = assign12920_body54_e17052_d_n6;
            locals.var_xmp_dn7 = assign12920_body54_e17052_d_n7;
            locals.var_xmp_dn10 = assign12920_body54_e17052_d_n10;
            locals.var_xmp_dn11 = assign12920_body54_e17052_d_n11;
            locals.var_xmp_dn12 = assign12920_body54_e17052_d_n12;
            locals.var_xmp_dn17 = assign12920_body54_e17052_d_n17;
            let (assign12920_body55_e17063, assign12920_body55_e17063_d_n0, assign12920_body55_e17063_d_n2, assign12920_body55_e17063_d_n6, assign12920_body55_e17063_d_n7, assign12920_body55_e17063_d_n10, assign12920_body55_e17063_d_n11, assign12920_body55_e17063_d_n12, assign12920_body55_e17063_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard394 != 0.0)) {
        let assign12920_body55_e17061: f64 = (locals.var_xp + locals.var_xmp);
        (assign12920_body55_e17061, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
            locals.var_arg = assign12920_body55_e17063;
            locals.var_arg_dn0 = assign12920_body55_e17063_d_n0;
            locals.var_arg_dn2 = assign12920_body55_e17063_d_n2;
            locals.var_arg_dn6 = assign12920_body55_e17063_d_n6;
            locals.var_arg_dn7 = assign12920_body55_e17063_d_n7;
            locals.var_arg_dn10 = assign12920_body55_e17063_d_n10;
            locals.var_arg_dn11 = assign12920_body55_e17063_d_n11;
            locals.var_arg_dn12 = assign12920_body55_e17063_d_n12;
            locals.var_arg_dn17 = assign12920_body55_e17063_d_n17;
            let (assign12920_body56_e17072, assign12920_body56_e17072_d_n0, assign12920_body56_e17072_d_n2, assign12920_body56_e17072_d_n6, assign12920_body56_e17072_d_n7, assign12920_body56_e17072_d_n10, assign12920_body56_e17072_d_n11, assign12920_body56_e17072_d_n12, assign12920_body56_e17072_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard394 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign12920_body56_e17072;
            locals.var_dnm_dn0 = assign12920_body56_e17072_d_n0;
            locals.var_dnm_dn2 = assign12920_body56_e17072_d_n2;
            locals.var_dnm_dn6 = assign12920_body56_e17072_d_n6;
            locals.var_dnm_dn7 = assign12920_body56_e17072_d_n7;
            locals.var_dnm_dn10 = assign12920_body56_e17072_d_n10;
            locals.var_dnm_dn11 = assign12920_body56_e17072_d_n11;
            locals.var_dnm_dn12 = assign12920_body56_e17072_d_n12;
            locals.var_dnm_dn17 = assign12920_body56_e17072_d_n17;
            let assign12920_body57_e17087: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
            locals.var_guard395 = assign12920_body57_e17087;
            let assign12920_body58_e17090: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard396 = assign12920_body58_e17090;
            let (assign12920_body59_e17103,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard394 != 0.0)) && (locals.var_guard395 != 0.0)) && (locals.var_guard396 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign12920_body59_e17103;
            let assign12920_body60_e17106: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
            locals.var_guard397 = assign12920_body60_e17106;
            let (assign12920_body61_e17122,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard394 != 0.0)) && (locals.var_guard395 != 0.0)) && (locals.var_guard396 == 0.0)) && (locals.var_guard397 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign12920_body61_e17122;
            let assign12920_body62_e17125: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
            locals.var_guard398 = assign12920_body62_e17125;
            let (assign12920_body63_e17144,) = {
    if (((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard394 != 0.0)) && (locals.var_guard395 != 0.0)) && (locals.var_guard396 == 0.0)) && (locals.var_guard397 == 0.0)) && (locals.var_guard398 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign12920_body63_e17144;
            let assign12920_body64_e17147: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
            locals.var_guard399 = assign12920_body64_e17147;
            let (assign12920_body65_e17169,) = {
    if ((((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard394 != 0.0)) && (locals.var_guard395 != 0.0)) && (locals.var_guard396 == 0.0)) && (locals.var_guard397 == 0.0)) && (locals.var_guard398 == 0.0)) && (locals.var_guard399 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign12920_body65_e17169;
            let (assign12920_body66_e17180,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard394 != 0.0)) && (locals.var_guard395 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign12920_body66_e17180;
            let mut assign12920_body67_loop_guard: usize = 0;
            while {
                let assign12920_body67_cond_e17192: f64 = if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard394 != 0.0)) && (locals.var_guard395 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
                assign12920_body67_cond_e17192 != 0.0
            } {
                assign12920_body67_loop_guard += 1;
                assert!(assign12920_body67_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                let (assign12920_body67_body0_e17204, assign12920_body67_body0_e17204_d_n0, assign12920_body67_body0_e17204_d_n2, assign12920_body67_body0_e17204_d_n6, assign12920_body67_body0_e17204_d_n7, assign12920_body67_body0_e17204_d_n10, assign12920_body67_body0_e17204_d_n11, assign12920_body67_body0_e17204_d_n12, assign12920_body67_body0_e17204_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard394 != 0.0)) && (locals.var_guard395 != 0.0)) {
        let assign12920_body67_body0_e17202: f64 = (locals.var_dnm).sqrt();
        (assign12920_body67_body0_e17202, (locals.var_dnm_dn0 / (2.0 * assign12920_body67_body0_e17202)), (locals.var_dnm_dn2 / (2.0 * assign12920_body67_body0_e17202)), (locals.var_dnm_dn6 / (2.0 * assign12920_body67_body0_e17202)), (locals.var_dnm_dn7 / (2.0 * assign12920_body67_body0_e17202)), (locals.var_dnm_dn10 / (2.0 * assign12920_body67_body0_e17202)), (locals.var_dnm_dn11 / (2.0 * assign12920_body67_body0_e17202)), (locals.var_dnm_dn12 / (2.0 * assign12920_body67_body0_e17202)), (locals.var_dnm_dn17 / (2.0 * assign12920_body67_body0_e17202)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
                locals.var_dnm = assign12920_body67_body0_e17204;
                locals.var_dnm_dn0 = assign12920_body67_body0_e17204_d_n0;
                locals.var_dnm_dn2 = assign12920_body67_body0_e17204_d_n2;
                locals.var_dnm_dn6 = assign12920_body67_body0_e17204_d_n6;
                locals.var_dnm_dn7 = assign12920_body67_body0_e17204_d_n7;
                locals.var_dnm_dn10 = assign12920_body67_body0_e17204_d_n10;
                locals.var_dnm_dn11 = assign12920_body67_body0_e17204_d_n11;
                locals.var_dnm_dn12 = assign12920_body67_body0_e17204_d_n12;
                locals.var_dnm_dn17 = assign12920_body67_body0_e17204_d_n17;
                let (assign12920_body67_body1_e17217,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard394 != 0.0)) && (locals.var_guard395 != 0.0)) {
        let assign12920_body67_body1_e17215: f64 = (locals.var_m0 + 1.0);
        (assign12920_body67_body1_e17215,)
    } else {
        (locals.var_m0,)
    }
};
                locals.var_m0 = assign12920_body67_body1_e17217;
            }
            let (assign12920_body68_e17235, assign12920_body68_e17235_d_n0, assign12920_body68_e17235_d_n2, assign12920_body68_e17235_d_n6, assign12920_body68_e17235_d_n7, assign12920_body68_e17235_d_n10, assign12920_body68_e17235_d_n11, assign12920_body68_e17235_d_n12, assign12920_body68_e17235_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard394 != 0.0)) && (locals.var_guard395 == 0.0)) {
        let assign12920_body68_e17231: f64 = (2.0 * 2.0);
        let assign12920_body68_e17232: f64 = (1.0 / assign12920_body68_e17231);
        let assign12920_body68_e17233: f64 = (locals.var_dnm).powf(assign12920_body68_e17232);
        (assign12920_body68_e17233, if 0.0 == 0.0 && ((assign12920_body68_e17232) as f64).is_finite() && ((assign12920_body68_e17232) as f64).fract() == 0.0 { if assign12920_body68_e17232 == 0.0 { 0.0 } else { (assign12920_body68_e17232 * ((locals.var_dnm).powf(assign12920_body68_e17232 - 1.0) * locals.var_dnm_dn0)) } } else { (assign12920_body68_e17233 * (assign12920_body68_e17232 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12920_body68_e17232) as f64).is_finite() && ((assign12920_body68_e17232) as f64).fract() == 0.0 { if assign12920_body68_e17232 == 0.0 { 0.0 } else { (assign12920_body68_e17232 * ((locals.var_dnm).powf(assign12920_body68_e17232 - 1.0) * locals.var_dnm_dn2)) } } else { (assign12920_body68_e17233 * (assign12920_body68_e17232 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12920_body68_e17232) as f64).is_finite() && ((assign12920_body68_e17232) as f64).fract() == 0.0 { if assign12920_body68_e17232 == 0.0 { 0.0 } else { (assign12920_body68_e17232 * ((locals.var_dnm).powf(assign12920_body68_e17232 - 1.0) * locals.var_dnm_dn6)) } } else { (assign12920_body68_e17233 * (assign12920_body68_e17232 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12920_body68_e17232) as f64).is_finite() && ((assign12920_body68_e17232) as f64).fract() == 0.0 { if assign12920_body68_e17232 == 0.0 { 0.0 } else { (assign12920_body68_e17232 * ((locals.var_dnm).powf(assign12920_body68_e17232 - 1.0) * locals.var_dnm_dn7)) } } else { (assign12920_body68_e17233 * (assign12920_body68_e17232 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12920_body68_e17232) as f64).is_finite() && ((assign12920_body68_e17232) as f64).fract() == 0.0 { if assign12920_body68_e17232 == 0.0 { 0.0 } else { (assign12920_body68_e17232 * ((locals.var_dnm).powf(assign12920_body68_e17232 - 1.0) * locals.var_dnm_dn10)) } } else { (assign12920_body68_e17233 * (assign12920_body68_e17232 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12920_body68_e17232) as f64).is_finite() && ((assign12920_body68_e17232) as f64).fract() == 0.0 { if assign12920_body68_e17232 == 0.0 { 0.0 } else { (assign12920_body68_e17232 * ((locals.var_dnm).powf(assign12920_body68_e17232 - 1.0) * locals.var_dnm_dn11)) } } else { (assign12920_body68_e17233 * (assign12920_body68_e17232 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12920_body68_e17232) as f64).is_finite() && ((assign12920_body68_e17232) as f64).fract() == 0.0 { if assign12920_body68_e17232 == 0.0 { 0.0 } else { (assign12920_body68_e17232 * ((locals.var_dnm).powf(assign12920_body68_e17232 - 1.0) * locals.var_dnm_dn12)) } } else { (assign12920_body68_e17233 * (assign12920_body68_e17232 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12920_body68_e17232) as f64).is_finite() && ((assign12920_body68_e17232) as f64).fract() == 0.0 { if assign12920_body68_e17232 == 0.0 { 0.0 } else { (assign12920_body68_e17232 * ((locals.var_dnm).powf(assign12920_body68_e17232 - 1.0) * locals.var_dnm_dn17)) } } else { (assign12920_body68_e17233 * (assign12920_body68_e17232 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign12920_body68_e17235;
            locals.var_dnm_dn0 = assign12920_body68_e17235_d_n0;
            locals.var_dnm_dn2 = assign12920_body68_e17235_d_n2;
            locals.var_dnm_dn6 = assign12920_body68_e17235_d_n6;
            locals.var_dnm_dn7 = assign12920_body68_e17235_d_n7;
            locals.var_dnm_dn10 = assign12920_body68_e17235_d_n10;
            locals.var_dnm_dn11 = assign12920_body68_e17235_d_n11;
            locals.var_dnm_dn12 = assign12920_body68_e17235_d_n12;
            locals.var_dnm_dn17 = assign12920_body68_e17235_d_n17;
            let (assign12920_body69_e17246, assign12920_body69_e17246_d_n0, assign12920_body69_e17246_d_n2, assign12920_body69_e17246_d_n6, assign12920_body69_e17246_d_n7, assign12920_body69_e17246_d_n10, assign12920_body69_e17246_d_n11, assign12920_body69_e17246_d_n12, assign12920_body69_e17246_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard394 != 0.0)) {
        let assign12920_body69_e17244: f64 = (1.0 / locals.var_dnm);
        (assign12920_body69_e17244, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign12920_body69_e17246;
            locals.var_dnm_dn0 = assign12920_body69_e17246_d_n0;
            locals.var_dnm_dn2 = assign12920_body69_e17246_d_n2;
            locals.var_dnm_dn6 = assign12920_body69_e17246_d_n6;
            locals.var_dnm_dn7 = assign12920_body69_e17246_d_n7;
            locals.var_dnm_dn10 = assign12920_body69_e17246_d_n10;
            locals.var_dnm_dn11 = assign12920_body69_e17246_d_n11;
            locals.var_dnm_dn12 = assign12920_body69_e17246_d_n12;
            locals.var_dnm_dn17 = assign12920_body69_e17246_d_n17;
            let (assign12920_body70_e17262, assign12920_body70_e17262_d_n0, assign12920_body70_e17262_d_n2, assign12920_body70_e17262_d_n6, assign12920_body70_e17262_d_n7, assign12920_body70_e17262_d_n10, assign12920_body70_e17262_d_n11, assign12920_body70_e17262_d_n12, assign12920_body70_e17262_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard394 != 0.0)) {
        let assign12920_body70_e17255: f64 = (-locals.var_q_wdsoi_max);
        let assign12920_body70_e17257: f64 = assign12920_body70_e17255;
        let assign12920_body70_e17258: f64 = (locals.var_tmf1 * assign12920_body70_e17257);
        let assign12920_body70_e17260: f64 = (assign12920_body70_e17258 * locals.var_dnm);
        (assign12920_body70_e17260, ((((locals.var_tmf1_dn0 * assign12920_body70_e17257) + (locals.var_tmf1 * (-locals.var_q_wdsoi_max_dn0))) * locals.var_dnm) + (assign12920_body70_e17258 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign12920_body70_e17257) + (locals.var_tmf1 * (-locals.var_q_wdsoi_max_dn2))) * locals.var_dnm) + (assign12920_body70_e17258 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn6 * assign12920_body70_e17257) + (locals.var_tmf1 * (-locals.var_q_wdsoi_max_dn6))) * locals.var_dnm) + (assign12920_body70_e17258 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign12920_body70_e17257) + (locals.var_tmf1 * (-locals.var_q_wdsoi_max_dn7))) * locals.var_dnm) + (assign12920_body70_e17258 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn10 * assign12920_body70_e17257) + (locals.var_tmf1 * (-locals.var_q_wdsoi_max_dn10))) * locals.var_dnm) + (assign12920_body70_e17258 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * assign12920_body70_e17257) + (locals.var_tmf1 * (-locals.var_q_wdsoi_max_dn11))) * locals.var_dnm) + (assign12920_body70_e17258 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn12 * assign12920_body70_e17257) + (locals.var_tmf1 * (-locals.var_q_wdsoi_max_dn12))) * locals.var_dnm) + (assign12920_body70_e17258 * locals.var_dnm_dn12)), ((((locals.var_tmf1_dn17 * assign12920_body70_e17257) + (locals.var_tmf1 * (-locals.var_q_wdsoi_max_dn17))) * locals.var_dnm) + (assign12920_body70_e17258 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0__blk385, locals.var_tmf0__blk385_dn0, locals.var_tmf0__blk385_dn2, locals.var_tmf0__blk385_dn6, locals.var_tmf0__blk385_dn7, locals.var_tmf0__blk385_dn10, locals.var_tmf0__blk385_dn11, locals.var_tmf0__blk385_dn12, locals.var_tmf0__blk385_dn17,)
    }
};
            locals.var_tmf0__blk385 = assign12920_body70_e17262;
            locals.var_tmf0__blk385_dn0 = assign12920_body70_e17262_d_n0;
            locals.var_tmf0__blk385_dn2 = assign12920_body70_e17262_d_n2;
            locals.var_tmf0__blk385_dn6 = assign12920_body70_e17262_d_n6;
            locals.var_tmf0__blk385_dn7 = assign12920_body70_e17262_d_n7;
            locals.var_tmf0__blk385_dn10 = assign12920_body70_e17262_d_n10;
            locals.var_tmf0__blk385_dn11 = assign12920_body70_e17262_d_n11;
            locals.var_tmf0__blk385_dn12 = assign12920_body70_e17262_d_n12;
            locals.var_tmf0__blk385_dn17 = assign12920_body70_e17262_d_n17;
            let (assign12920_body71_e17280, assign12920_body71_e17280_d_n0, assign12920_body71_e17280_d_n2, assign12920_body71_e17280_d_n6, assign12920_body71_e17280_d_n7, assign12920_body71_e17280_d_n10, assign12920_body71_e17280_d_n11, assign12920_body71_e17280_d_n12, assign12920_body71_e17280_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard394 != 0.0)) {
        let assign12920_body71_e17270: f64 = (-locals.var_q_wdsoi_max);
        let assign12920_body71_e17272: f64 = assign12920_body71_e17270;
        let assign12920_body71_e17274: f64 = (assign12920_body71_e17272 * locals.var_xmp);
        let assign12920_body71_e17276: f64 = (assign12920_body71_e17274 * locals.var_dnm);
        let assign12920_body71_e17278: f64 = (assign12920_body71_e17276 / locals.var_arg);
        (assign12920_body71_e17278, ((((((((-locals.var_q_wdsoi_max_dn0) * locals.var_xmp) + (assign12920_body71_e17272 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign12920_body71_e17274 * locals.var_dnm_dn0)) * locals.var_arg) - (assign12920_body71_e17276 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((-locals.var_q_wdsoi_max_dn2) * locals.var_xmp) + (assign12920_body71_e17272 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign12920_body71_e17274 * locals.var_dnm_dn2)) * locals.var_arg) - (assign12920_body71_e17276 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((-locals.var_q_wdsoi_max_dn6) * locals.var_xmp) + (assign12920_body71_e17272 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign12920_body71_e17274 * locals.var_dnm_dn6)) * locals.var_arg) - (assign12920_body71_e17276 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((-locals.var_q_wdsoi_max_dn7) * locals.var_xmp) + (assign12920_body71_e17272 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign12920_body71_e17274 * locals.var_dnm_dn7)) * locals.var_arg) - (assign12920_body71_e17276 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((-locals.var_q_wdsoi_max_dn10) * locals.var_xmp) + (assign12920_body71_e17272 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign12920_body71_e17274 * locals.var_dnm_dn10)) * locals.var_arg) - (assign12920_body71_e17276 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((-locals.var_q_wdsoi_max_dn11) * locals.var_xmp) + (assign12920_body71_e17272 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign12920_body71_e17274 * locals.var_dnm_dn11)) * locals.var_arg) - (assign12920_body71_e17276 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((((-locals.var_q_wdsoi_max_dn12) * locals.var_xmp) + (assign12920_body71_e17272 * locals.var_xmp_dn12)) * locals.var_dnm) + (assign12920_body71_e17274 * locals.var_dnm_dn12)) * locals.var_arg) - (assign12920_body71_e17276 * locals.var_arg_dn12)) / (locals.var_arg * locals.var_arg)), ((((((((-locals.var_q_wdsoi_max_dn17) * locals.var_xmp) + (assign12920_body71_e17272 * locals.var_xmp_dn17)) * locals.var_dnm) + (assign12920_body71_e17274 * locals.var_dnm_dn17)) * locals.var_arg) - (assign12920_body71_e17276 * locals.var_arg_dn17)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign12920_body71_e17280;
            locals.var_t0_dn0 = assign12920_body71_e17280_d_n0;
            locals.var_t0_dn2 = assign12920_body71_e17280_d_n2;
            locals.var_t0_dn6 = assign12920_body71_e17280_d_n6;
            locals.var_t0_dn7 = assign12920_body71_e17280_d_n7;
            locals.var_t0_dn10 = assign12920_body71_e17280_d_n10;
            locals.var_t0_dn11 = assign12920_body71_e17280_d_n11;
            locals.var_t0_dn12 = assign12920_body71_e17280_d_n12;
            locals.var_t0_dn17 = assign12920_body71_e17280_d_n17;
            let (assign12920_body72_e17296, assign12920_body72_e17296_d_n0, assign12920_body72_e17296_d_n2, assign12920_body72_e17296_d_n6, assign12920_body72_e17296_d_n7, assign12920_body72_e17296_d_n10, assign12920_body72_e17296_d_n11, assign12920_body72_e17296_d_n12, assign12920_body72_e17296_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard394 != 0.0)) {
        let assign12920_body72_e17289: f64 = (-locals.var_q_wdsoi_max);
        let assign12920_body72_e17291: f64 = assign12920_body72_e17289;
        let assign12920_body72_e17292: f64 = (-assign12920_body72_e17291);
        let assign12920_body72_e17294: f64 = (assign12920_body72_e17292 + locals.var_tmf0__blk385);
        (assign12920_body72_e17294, ((-(-locals.var_q_wdsoi_max_dn0)) + locals.var_tmf0__blk385_dn0), ((-(-locals.var_q_wdsoi_max_dn2)) + locals.var_tmf0__blk385_dn2), ((-(-locals.var_q_wdsoi_max_dn6)) + locals.var_tmf0__blk385_dn6), ((-(-locals.var_q_wdsoi_max_dn7)) + locals.var_tmf0__blk385_dn7), ((-(-locals.var_q_wdsoi_max_dn10)) + locals.var_tmf0__blk385_dn10), ((-(-locals.var_q_wdsoi_max_dn11)) + locals.var_tmf0__blk385_dn11), ((-(-locals.var_q_wdsoi_max_dn12)) + locals.var_tmf0__blk385_dn12), ((-(-locals.var_q_wdsoi_max_dn17)) + locals.var_tmf0__blk385_dn17),)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn12, locals.var_q_bl_dep_dn17,)
    }
};
            locals.var_q_bl_dep = assign12920_body72_e17296;
            locals.var_q_bl_dep_dn0 = assign12920_body72_e17296_d_n0;
            locals.var_q_bl_dep_dn2 = assign12920_body72_e17296_d_n2;
            locals.var_q_bl_dep_dn6 = assign12920_body72_e17296_d_n6;
            locals.var_q_bl_dep_dn7 = assign12920_body72_e17296_d_n7;
            locals.var_q_bl_dep_dn10 = assign12920_body72_e17296_d_n10;
            locals.var_q_bl_dep_dn11 = assign12920_body72_e17296_d_n11;
            locals.var_q_bl_dep_dn12 = assign12920_body72_e17296_d_n12;
            locals.var_q_bl_dep_dn17 = assign12920_body72_e17296_d_n17;
            let (assign12920_body73_e17305, assign12920_body73_e17305_d_n0, assign12920_body73_e17305_d_n2, assign12920_body73_e17305_d_n6, assign12920_body73_e17305_d_n7, assign12920_body73_e17305_d_n10, assign12920_body73_e17305_d_n11, assign12920_body73_e17305_d_n12, assign12920_body73_e17305_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard394 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign12920_body73_e17305;
            locals.var_t0_dn0 = assign12920_body73_e17305_d_n0;
            locals.var_t0_dn2 = assign12920_body73_e17305_d_n2;
            locals.var_t0_dn6 = assign12920_body73_e17305_d_n6;
            locals.var_t0_dn7 = assign12920_body73_e17305_d_n7;
            locals.var_t0_dn10 = assign12920_body73_e17305_d_n10;
            locals.var_t0_dn11 = assign12920_body73_e17305_d_n11;
            locals.var_t0_dn12 = assign12920_body73_e17305_d_n12;
            locals.var_t0_dn17 = assign12920_body73_e17305_d_n17;
            let (assign12920_body74_e17315, assign12920_body74_e17315_d_n0, assign12920_body74_e17315_d_n2, assign12920_body74_e17315_d_n6, assign12920_body74_e17315_d_n7, assign12920_body74_e17315_d_n10, assign12920_body74_e17315_d_n11, assign12920_body74_e17315_d_n12, assign12920_body74_e17315_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard394 == 0.0)) {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn12, locals.var_q_bl_dep_dn17,)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn12, locals.var_q_bl_dep_dn17,)
    }
};
            locals.var_q_bl_dep = assign12920_body74_e17315;
            locals.var_q_bl_dep_dn0 = assign12920_body74_e17315_d_n0;
            locals.var_q_bl_dep_dn2 = assign12920_body74_e17315_d_n2;
            locals.var_q_bl_dep_dn6 = assign12920_body74_e17315_d_n6;
            locals.var_q_bl_dep_dn7 = assign12920_body74_e17315_d_n7;
            locals.var_q_bl_dep_dn10 = assign12920_body74_e17315_d_n10;
            locals.var_q_bl_dep_dn11 = assign12920_body74_e17315_d_n11;
            locals.var_q_bl_dep_dn12 = assign12920_body74_e17315_d_n12;
            locals.var_q_bl_dep_dn17 = assign12920_body74_e17315_d_n17;
            let (assign12920_body75_e17325, assign12920_body75_e17325_d_n0, assign12920_body75_e17325_d_n2, assign12920_body75_e17325_d_n6, assign12920_body75_e17325_d_n7, assign12920_body75_e17325_d_n10, assign12920_body75_e17325_d_n11, assign12920_body75_e17325_d_n12, assign12920_body75_e17325_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard394 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign12920_body75_e17325;
            locals.var_t0_dn0 = assign12920_body75_e17325_d_n0;
            locals.var_t0_dn2 = assign12920_body75_e17325_d_n2;
            locals.var_t0_dn6 = assign12920_body75_e17325_d_n6;
            locals.var_t0_dn7 = assign12920_body75_e17325_d_n7;
            locals.var_t0_dn10 = assign12920_body75_e17325_d_n10;
            locals.var_t0_dn11 = assign12920_body75_e17325_d_n11;
            locals.var_t0_dn12 = assign12920_body75_e17325_d_n12;
            locals.var_t0_dn17 = assign12920_body75_e17325_d_n17;
            let (assign12920_body76_e17334, assign12920_body76_e17334_d_n0, assign12920_body76_e17334_d_n2, assign12920_body76_e17334_d_n6, assign12920_body76_e17334_d_n7, assign12920_body76_e17334_d_n10, assign12920_body76_e17334_d_n11, assign12920_body76_e17334_d_n12, assign12920_body76_e17334_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign12920_body76_e17332: f64 = (locals.var_q_bl_dep_dpbs * locals.var_t0);
        (assign12920_body76_e17332, ((locals.var_q_bl_dep_dpbs_dn0 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn0)), ((locals.var_q_bl_dep_dpbs_dn2 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn2)), ((locals.var_q_bl_dep_dpbs_dn6 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn6)), ((locals.var_q_bl_dep_dpbs_dn7 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn7)), ((locals.var_q_bl_dep_dpbs_dn10 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn10)), ((locals.var_q_bl_dep_dpbs_dn11 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn11)), ((locals.var_q_bl_dep_dpbs_dn12 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn12)), ((locals.var_q_bl_dep_dpbs_dn17 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn17)),)
    } else {
        (locals.var_q_bl_dep_dpbs, locals.var_q_bl_dep_dpbs_dn0, locals.var_q_bl_dep_dpbs_dn2, locals.var_q_bl_dep_dpbs_dn6, locals.var_q_bl_dep_dpbs_dn7, locals.var_q_bl_dep_dpbs_dn10, locals.var_q_bl_dep_dpbs_dn11, locals.var_q_bl_dep_dpbs_dn12, locals.var_q_bl_dep_dpbs_dn17,)
    }
};
            locals.var_q_bl_dep_dpbs = assign12920_body76_e17334;
            locals.var_q_bl_dep_dpbs_dn0 = assign12920_body76_e17334_d_n0;
            locals.var_q_bl_dep_dpbs_dn2 = assign12920_body76_e17334_d_n2;
            locals.var_q_bl_dep_dpbs_dn6 = assign12920_body76_e17334_d_n6;
            locals.var_q_bl_dep_dpbs_dn7 = assign12920_body76_e17334_d_n7;
            locals.var_q_bl_dep_dpbs_dn10 = assign12920_body76_e17334_d_n10;
            locals.var_q_bl_dep_dpbs_dn11 = assign12920_body76_e17334_d_n11;
            locals.var_q_bl_dep_dpbs_dn12 = assign12920_body76_e17334_d_n12;
            locals.var_q_bl_dep_dpbs_dn17 = assign12920_body76_e17334_d_n17;
            let (assign12920_body77_e17343, assign12920_body77_e17343_d_n0, assign12920_body77_e17343_d_n2, assign12920_body77_e17343_d_n6, assign12920_body77_e17343_d_n7, assign12920_body77_e17343_d_n10, assign12920_body77_e17343_d_n11, assign12920_body77_e17343_d_n12, assign12920_body77_e17343_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign12920_body77_e17341: f64 = (locals.var_q_bl_dep_dpss * locals.var_t0);
        (assign12920_body77_e17341, ((locals.var_q_bl_dep_dpss_dn0 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn0)), ((locals.var_q_bl_dep_dpss_dn2 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn2)), ((locals.var_q_bl_dep_dpss_dn6 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn6)), ((locals.var_q_bl_dep_dpss_dn7 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn7)), ((locals.var_q_bl_dep_dpss_dn10 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn10)), ((locals.var_q_bl_dep_dpss_dn11 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn11)), ((locals.var_q_bl_dep_dpss_dn12 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn12)), ((locals.var_q_bl_dep_dpss_dn17 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn17)),)
    } else {
        (locals.var_q_bl_dep_dpss, locals.var_q_bl_dep_dpss_dn0, locals.var_q_bl_dep_dpss_dn2, locals.var_q_bl_dep_dpss_dn6, locals.var_q_bl_dep_dpss_dn7, locals.var_q_bl_dep_dpss_dn10, locals.var_q_bl_dep_dpss_dn11, locals.var_q_bl_dep_dpss_dn12, locals.var_q_bl_dep_dpss_dn17,)
    }
};
            locals.var_q_bl_dep_dpss = assign12920_body77_e17343;
            locals.var_q_bl_dep_dpss_dn0 = assign12920_body77_e17343_d_n0;
            locals.var_q_bl_dep_dpss_dn2 = assign12920_body77_e17343_d_n2;
            locals.var_q_bl_dep_dpss_dn6 = assign12920_body77_e17343_d_n6;
            locals.var_q_bl_dep_dpss_dn7 = assign12920_body77_e17343_d_n7;
            locals.var_q_bl_dep_dpss_dn10 = assign12920_body77_e17343_d_n10;
            locals.var_q_bl_dep_dpss_dn11 = assign12920_body77_e17343_d_n11;
            locals.var_q_bl_dep_dpss_dn12 = assign12920_body77_e17343_d_n12;
            locals.var_q_bl_dep_dpss_dn17 = assign12920_body77_e17343_d_n17;
            let assign12920_body78_e17347: f64 = (locals.var_q_fd_soi - locals.var_q_sl_dep);
            let assign12920_body78_e17350: f64 = (locals.var_q_fd_soi - locals.var_q_sl_dep);
            let assign12920_body78_e17351: f64 = (-assign12920_body78_e17350);
            let assign12920_body78_e17353: f64 = assign12920_body78_e17351;
            let assign12920_body78_e17354: f64 = (assign12920_body78_e17347 + assign12920_body78_e17353);
            let assign12920_body78_e17358: f64 = (locals.var_q_fd_soi - locals.var_q_sl_dep);
            let assign12920_body78_e17359: f64 = (-assign12920_body78_e17358);
            let assign12920_body78_e17361: f64 = assign12920_body78_e17359;
            let assign12920_body78_e17364: f64 = if ((locals.var_q_bl_dep < assign12920_body78_e17354) && (assign12920_body78_e17361 >= 0.0)) { 1.0 } else { 0.0 };
            locals.var_guard400 = assign12920_body78_e17364;
            let (assign12920_body79_e17384, assign12920_body79_e17384_d_n0, assign12920_body79_e17384_d_n2, assign12920_body79_e17384_d_n6, assign12920_body79_e17384_d_n7, assign12920_body79_e17384_d_n10, assign12920_body79_e17384_d_n11, assign12920_body79_e17384_d_n12, assign12920_body79_e17384_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let assign12920_body79_e17373: f64 = (locals.var_q_fd_soi - locals.var_q_sl_dep);
        let assign12920_body79_e17376: f64 = (locals.var_q_fd_soi - locals.var_q_sl_dep);
        let assign12920_body79_e17377: f64 = (-assign12920_body79_e17376);
        let assign12920_body79_e17379: f64 = assign12920_body79_e17377;
        let assign12920_body79_e17380: f64 = (assign12920_body79_e17373 + assign12920_body79_e17379);
        let assign12920_body79_e17382: f64 = (assign12920_body79_e17380 - locals.var_q_bl_dep);
        (assign12920_body79_e17382, (((locals.var_q_fd_soi_dn0 - locals.var_q_sl_dep_dn0) + (-(locals.var_q_fd_soi_dn0 - locals.var_q_sl_dep_dn0))) - locals.var_q_bl_dep_dn0), (((locals.var_q_fd_soi_dn2 - locals.var_q_sl_dep_dn2) + (-(locals.var_q_fd_soi_dn2 - locals.var_q_sl_dep_dn2))) - locals.var_q_bl_dep_dn2), (((locals.var_q_fd_soi_dn6 - locals.var_q_sl_dep_dn6) + (-(locals.var_q_fd_soi_dn6 - locals.var_q_sl_dep_dn6))) - locals.var_q_bl_dep_dn6), (((locals.var_q_fd_soi_dn7 - locals.var_q_sl_dep_dn7) + (-(locals.var_q_fd_soi_dn7 - locals.var_q_sl_dep_dn7))) - locals.var_q_bl_dep_dn7), (((locals.var_q_fd_soi_dn10 - locals.var_q_sl_dep_dn10) + (-(locals.var_q_fd_soi_dn10 - locals.var_q_sl_dep_dn10))) - locals.var_q_bl_dep_dn10), (((locals.var_q_fd_soi_dn11 - locals.var_q_sl_dep_dn11) + (-(locals.var_q_fd_soi_dn11 - locals.var_q_sl_dep_dn11))) - locals.var_q_bl_dep_dn11), (((locals.var_q_fd_soi_dn12 - locals.var_q_sl_dep_dn12) + (-(locals.var_q_fd_soi_dn12 - locals.var_q_sl_dep_dn12))) - locals.var_q_bl_dep_dn12), (((locals.var_q_fd_soi_dn17 - locals.var_q_sl_dep_dn17) + (-(locals.var_q_fd_soi_dn17 - locals.var_q_sl_dep_dn17))) - locals.var_q_bl_dep_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
            locals.var_tmf1 = assign12920_body79_e17384;
            locals.var_tmf1_dn0 = assign12920_body79_e17384_d_n0;
            locals.var_tmf1_dn2 = assign12920_body79_e17384_d_n2;
            locals.var_tmf1_dn6 = assign12920_body79_e17384_d_n6;
            locals.var_tmf1_dn7 = assign12920_body79_e17384_d_n7;
            locals.var_tmf1_dn10 = assign12920_body79_e17384_d_n10;
            locals.var_tmf1_dn11 = assign12920_body79_e17384_d_n11;
            locals.var_tmf1_dn12 = assign12920_body79_e17384_d_n12;
            locals.var_tmf1_dn17 = assign12920_body79_e17384_d_n17;
            let (assign12920_body80_e17395, assign12920_body80_e17395_d_n0, assign12920_body80_e17395_d_n2, assign12920_body80_e17395_d_n6, assign12920_body80_e17395_d_n7, assign12920_body80_e17395_d_n10, assign12920_body80_e17395_d_n11, assign12920_body80_e17395_d_n12, assign12920_body80_e17395_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let assign12920_body80_e17393: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign12920_body80_e17393, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
            locals.var_x2 = assign12920_body80_e17395;
            locals.var_x2_dn0 = assign12920_body80_e17395_d_n0;
            locals.var_x2_dn2 = assign12920_body80_e17395_d_n2;
            locals.var_x2_dn6 = assign12920_body80_e17395_d_n6;
            locals.var_x2_dn7 = assign12920_body80_e17395_d_n7;
            locals.var_x2_dn10 = assign12920_body80_e17395_d_n10;
            locals.var_x2_dn11 = assign12920_body80_e17395_d_n11;
            locals.var_x2_dn12 = assign12920_body80_e17395_d_n12;
            locals.var_x2_dn17 = assign12920_body80_e17395_d_n17;
            let (assign12920_body81_e17416, assign12920_body81_e17416_d_n0, assign12920_body81_e17416_d_n2, assign12920_body81_e17416_d_n6, assign12920_body81_e17416_d_n7, assign12920_body81_e17416_d_n10, assign12920_body81_e17416_d_n11, assign12920_body81_e17416_d_n12, assign12920_body81_e17416_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let assign12920_body81_e17404: f64 = (locals.var_q_fd_soi - locals.var_q_sl_dep);
        let assign12920_body81_e17405: f64 = (-assign12920_body81_e17404);
        let assign12920_body81_e17407: f64 = assign12920_body81_e17405;
        let assign12920_body81_e17410: f64 = (locals.var_q_fd_soi - locals.var_q_sl_dep);
        let assign12920_body81_e17411: f64 = (-assign12920_body81_e17410);
        let assign12920_body81_e17413: f64 = assign12920_body81_e17411;
        let assign12920_body81_e17414: f64 = (assign12920_body81_e17407 * assign12920_body81_e17413);
        (assign12920_body81_e17414, (((-(locals.var_q_fd_soi_dn0 - locals.var_q_sl_dep_dn0)) * assign12920_body81_e17413) + (assign12920_body81_e17407 * (-(locals.var_q_fd_soi_dn0 - locals.var_q_sl_dep_dn0)))), (((-(locals.var_q_fd_soi_dn2 - locals.var_q_sl_dep_dn2)) * assign12920_body81_e17413) + (assign12920_body81_e17407 * (-(locals.var_q_fd_soi_dn2 - locals.var_q_sl_dep_dn2)))), (((-(locals.var_q_fd_soi_dn6 - locals.var_q_sl_dep_dn6)) * assign12920_body81_e17413) + (assign12920_body81_e17407 * (-(locals.var_q_fd_soi_dn6 - locals.var_q_sl_dep_dn6)))), (((-(locals.var_q_fd_soi_dn7 - locals.var_q_sl_dep_dn7)) * assign12920_body81_e17413) + (assign12920_body81_e17407 * (-(locals.var_q_fd_soi_dn7 - locals.var_q_sl_dep_dn7)))), (((-(locals.var_q_fd_soi_dn10 - locals.var_q_sl_dep_dn10)) * assign12920_body81_e17413) + (assign12920_body81_e17407 * (-(locals.var_q_fd_soi_dn10 - locals.var_q_sl_dep_dn10)))), (((-(locals.var_q_fd_soi_dn11 - locals.var_q_sl_dep_dn11)) * assign12920_body81_e17413) + (assign12920_body81_e17407 * (-(locals.var_q_fd_soi_dn11 - locals.var_q_sl_dep_dn11)))), (((-(locals.var_q_fd_soi_dn12 - locals.var_q_sl_dep_dn12)) * assign12920_body81_e17413) + (assign12920_body81_e17407 * (-(locals.var_q_fd_soi_dn12 - locals.var_q_sl_dep_dn12)))), (((-(locals.var_q_fd_soi_dn17 - locals.var_q_sl_dep_dn17)) * assign12920_body81_e17413) + (assign12920_body81_e17407 * (-(locals.var_q_fd_soi_dn17 - locals.var_q_sl_dep_dn17)))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
            locals.var_xmax2 = assign12920_body81_e17416;
            locals.var_xmax2_dn0 = assign12920_body81_e17416_d_n0;
            locals.var_xmax2_dn2 = assign12920_body81_e17416_d_n2;
            locals.var_xmax2_dn6 = assign12920_body81_e17416_d_n6;
            locals.var_xmax2_dn7 = assign12920_body81_e17416_d_n7;
            locals.var_xmax2_dn10 = assign12920_body81_e17416_d_n10;
            locals.var_xmax2_dn11 = assign12920_body81_e17416_d_n11;
            locals.var_xmax2_dn12 = assign12920_body81_e17416_d_n12;
            locals.var_xmax2_dn17 = assign12920_body81_e17416_d_n17;
            let (assign12920_body82_e17425, assign12920_body82_e17425_d_n0, assign12920_body82_e17425_d_n2, assign12920_body82_e17425_d_n6, assign12920_body82_e17425_d_n7, assign12920_body82_e17425_d_n10, assign12920_body82_e17425_d_n11, assign12920_body82_e17425_d_n12, assign12920_body82_e17425_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard400 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
            locals.var_xp = assign12920_body82_e17425;
            locals.var_xp_dn0 = assign12920_body82_e17425_d_n0;
            locals.var_xp_dn2 = assign12920_body82_e17425_d_n2;
            locals.var_xp_dn6 = assign12920_body82_e17425_d_n6;
            locals.var_xp_dn7 = assign12920_body82_e17425_d_n7;
            locals.var_xp_dn10 = assign12920_body82_e17425_d_n10;
            locals.var_xp_dn11 = assign12920_body82_e17425_d_n11;
            locals.var_xp_dn12 = assign12920_body82_e17425_d_n12;
            locals.var_xp_dn17 = assign12920_body82_e17425_d_n17;
            let (assign12920_body83_e17434, assign12920_body83_e17434_d_n0, assign12920_body83_e17434_d_n2, assign12920_body83_e17434_d_n6, assign12920_body83_e17434_d_n7, assign12920_body83_e17434_d_n10, assign12920_body83_e17434_d_n11, assign12920_body83_e17434_d_n12, assign12920_body83_e17434_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard400 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
            locals.var_xmp = assign12920_body83_e17434;
            locals.var_xmp_dn0 = assign12920_body83_e17434_d_n0;
            locals.var_xmp_dn2 = assign12920_body83_e17434_d_n2;
            locals.var_xmp_dn6 = assign12920_body83_e17434_d_n6;
            locals.var_xmp_dn7 = assign12920_body83_e17434_d_n7;
            locals.var_xmp_dn10 = assign12920_body83_e17434_d_n10;
            locals.var_xmp_dn11 = assign12920_body83_e17434_d_n11;
            locals.var_xmp_dn12 = assign12920_body83_e17434_d_n12;
            locals.var_xmp_dn17 = assign12920_body83_e17434_d_n17;
            let (assign12920_body84_e17443,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard400 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign12920_body84_e17443;
            let (assign12920_body85_e17452,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard400 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign12920_body85_e17452;
            let (assign12920_body86_e17461, assign12920_body86_e17461_d_n0, assign12920_body86_e17461_d_n2, assign12920_body86_e17461_d_n6, assign12920_body86_e17461_d_n7, assign12920_body86_e17461_d_n10, assign12920_body86_e17461_d_n11, assign12920_body86_e17461_d_n12, assign12920_body86_e17461_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard400 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
            locals.var_arg = assign12920_body86_e17461;
            locals.var_arg_dn0 = assign12920_body86_e17461_d_n0;
            locals.var_arg_dn2 = assign12920_body86_e17461_d_n2;
            locals.var_arg_dn6 = assign12920_body86_e17461_d_n6;
            locals.var_arg_dn7 = assign12920_body86_e17461_d_n7;
            locals.var_arg_dn10 = assign12920_body86_e17461_d_n10;
            locals.var_arg_dn11 = assign12920_body86_e17461_d_n11;
            locals.var_arg_dn12 = assign12920_body86_e17461_d_n12;
            locals.var_arg_dn17 = assign12920_body86_e17461_d_n17;
            let (assign12920_body87_e17470, assign12920_body87_e17470_d_n0, assign12920_body87_e17470_d_n2, assign12920_body87_e17470_d_n6, assign12920_body87_e17470_d_n7, assign12920_body87_e17470_d_n10, assign12920_body87_e17470_d_n11, assign12920_body87_e17470_d_n12, assign12920_body87_e17470_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard400 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign12920_body87_e17470;
            locals.var_dnm_dn0 = assign12920_body87_e17470_d_n0;
            locals.var_dnm_dn2 = assign12920_body87_e17470_d_n2;
            locals.var_dnm_dn6 = assign12920_body87_e17470_d_n6;
            locals.var_dnm_dn7 = assign12920_body87_e17470_d_n7;
            locals.var_dnm_dn10 = assign12920_body87_e17470_d_n10;
            locals.var_dnm_dn11 = assign12920_body87_e17470_d_n11;
            locals.var_dnm_dn12 = assign12920_body87_e17470_d_n12;
            locals.var_dnm_dn17 = assign12920_body87_e17470_d_n17;
            let (assign12920_body88_e17481, assign12920_body88_e17481_d_n0, assign12920_body88_e17481_d_n2, assign12920_body88_e17481_d_n6, assign12920_body88_e17481_d_n7, assign12920_body88_e17481_d_n10, assign12920_body88_e17481_d_n11, assign12920_body88_e17481_d_n12, assign12920_body88_e17481_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let assign12920_body88_e17479: f64 = (locals.var_xp * locals.var_x2);
        (assign12920_body88_e17479, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
            locals.var_xp = assign12920_body88_e17481;
            locals.var_xp_dn0 = assign12920_body88_e17481_d_n0;
            locals.var_xp_dn2 = assign12920_body88_e17481_d_n2;
            locals.var_xp_dn6 = assign12920_body88_e17481_d_n6;
            locals.var_xp_dn7 = assign12920_body88_e17481_d_n7;
            locals.var_xp_dn10 = assign12920_body88_e17481_d_n10;
            locals.var_xp_dn11 = assign12920_body88_e17481_d_n11;
            locals.var_xp_dn12 = assign12920_body88_e17481_d_n12;
            locals.var_xp_dn17 = assign12920_body88_e17481_d_n17;
            let (assign12920_body89_e17492, assign12920_body89_e17492_d_n0, assign12920_body89_e17492_d_n2, assign12920_body89_e17492_d_n6, assign12920_body89_e17492_d_n7, assign12920_body89_e17492_d_n10, assign12920_body89_e17492_d_n11, assign12920_body89_e17492_d_n12, assign12920_body89_e17492_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let assign12920_body89_e17490: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign12920_body89_e17490, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
            locals.var_xmp = assign12920_body89_e17492;
            locals.var_xmp_dn0 = assign12920_body89_e17492_d_n0;
            locals.var_xmp_dn2 = assign12920_body89_e17492_d_n2;
            locals.var_xmp_dn6 = assign12920_body89_e17492_d_n6;
            locals.var_xmp_dn7 = assign12920_body89_e17492_d_n7;
            locals.var_xmp_dn10 = assign12920_body89_e17492_d_n10;
            locals.var_xmp_dn11 = assign12920_body89_e17492_d_n11;
            locals.var_xmp_dn12 = assign12920_body89_e17492_d_n12;
            locals.var_xmp_dn17 = assign12920_body89_e17492_d_n17;
            let (assign12920_body90_e17503, assign12920_body90_e17503_d_n0, assign12920_body90_e17503_d_n2, assign12920_body90_e17503_d_n6, assign12920_body90_e17503_d_n7, assign12920_body90_e17503_d_n10, assign12920_body90_e17503_d_n11, assign12920_body90_e17503_d_n12, assign12920_body90_e17503_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let assign12920_body90_e17501: f64 = (locals.var_xp * locals.var_x2);
        (assign12920_body90_e17501, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
            locals.var_xp = assign12920_body90_e17503;
            locals.var_xp_dn0 = assign12920_body90_e17503_d_n0;
            locals.var_xp_dn2 = assign12920_body90_e17503_d_n2;
            locals.var_xp_dn6 = assign12920_body90_e17503_d_n6;
            locals.var_xp_dn7 = assign12920_body90_e17503_d_n7;
            locals.var_xp_dn10 = assign12920_body90_e17503_d_n10;
            locals.var_xp_dn11 = assign12920_body90_e17503_d_n11;
            locals.var_xp_dn12 = assign12920_body90_e17503_d_n12;
            locals.var_xp_dn17 = assign12920_body90_e17503_d_n17;
            let (assign12920_body91_e17514, assign12920_body91_e17514_d_n0, assign12920_body91_e17514_d_n2, assign12920_body91_e17514_d_n6, assign12920_body91_e17514_d_n7, assign12920_body91_e17514_d_n10, assign12920_body91_e17514_d_n11, assign12920_body91_e17514_d_n12, assign12920_body91_e17514_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let assign12920_body91_e17512: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign12920_body91_e17512, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
            locals.var_xmp = assign12920_body91_e17514;
            locals.var_xmp_dn0 = assign12920_body91_e17514_d_n0;
            locals.var_xmp_dn2 = assign12920_body91_e17514_d_n2;
            locals.var_xmp_dn6 = assign12920_body91_e17514_d_n6;
            locals.var_xmp_dn7 = assign12920_body91_e17514_d_n7;
            locals.var_xmp_dn10 = assign12920_body91_e17514_d_n10;
            locals.var_xmp_dn11 = assign12920_body91_e17514_d_n11;
            locals.var_xmp_dn12 = assign12920_body91_e17514_d_n12;
            locals.var_xmp_dn17 = assign12920_body91_e17514_d_n17;
            let (assign12920_body92_e17525, assign12920_body92_e17525_d_n0, assign12920_body92_e17525_d_n2, assign12920_body92_e17525_d_n6, assign12920_body92_e17525_d_n7, assign12920_body92_e17525_d_n10, assign12920_body92_e17525_d_n11, assign12920_body92_e17525_d_n12, assign12920_body92_e17525_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let assign12920_body92_e17523: f64 = (locals.var_xp + locals.var_xmp);
        (assign12920_body92_e17523, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
            locals.var_arg = assign12920_body92_e17525;
            locals.var_arg_dn0 = assign12920_body92_e17525_d_n0;
            locals.var_arg_dn2 = assign12920_body92_e17525_d_n2;
            locals.var_arg_dn6 = assign12920_body92_e17525_d_n6;
            locals.var_arg_dn7 = assign12920_body92_e17525_d_n7;
            locals.var_arg_dn10 = assign12920_body92_e17525_d_n10;
            locals.var_arg_dn11 = assign12920_body92_e17525_d_n11;
            locals.var_arg_dn12 = assign12920_body92_e17525_d_n12;
            locals.var_arg_dn17 = assign12920_body92_e17525_d_n17;
            let (assign12920_body93_e17534, assign12920_body93_e17534_d_n0, assign12920_body93_e17534_d_n2, assign12920_body93_e17534_d_n6, assign12920_body93_e17534_d_n7, assign12920_body93_e17534_d_n10, assign12920_body93_e17534_d_n11, assign12920_body93_e17534_d_n12, assign12920_body93_e17534_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard400 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign12920_body93_e17534;
            locals.var_dnm_dn0 = assign12920_body93_e17534_d_n0;
            locals.var_dnm_dn2 = assign12920_body93_e17534_d_n2;
            locals.var_dnm_dn6 = assign12920_body93_e17534_d_n6;
            locals.var_dnm_dn7 = assign12920_body93_e17534_d_n7;
            locals.var_dnm_dn10 = assign12920_body93_e17534_d_n10;
            locals.var_dnm_dn11 = assign12920_body93_e17534_d_n11;
            locals.var_dnm_dn12 = assign12920_body93_e17534_d_n12;
            locals.var_dnm_dn17 = assign12920_body93_e17534_d_n17;
            let assign12920_body94_e17549: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
            locals.var_guard401 = assign12920_body94_e17549;
            let assign12920_body95_e17552: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard402 = assign12920_body95_e17552;
            let (assign12920_body96_e17565,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard400 != 0.0)) && (locals.var_guard401 != 0.0)) && (locals.var_guard402 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign12920_body96_e17565;
            let assign12920_body97_e17568: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
            locals.var_guard403 = assign12920_body97_e17568;
            let (assign12920_body98_e17584,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard400 != 0.0)) && (locals.var_guard401 != 0.0)) && (locals.var_guard402 == 0.0)) && (locals.var_guard403 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign12920_body98_e17584;
            let assign12920_body99_e17587: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
            locals.var_guard404 = assign12920_body99_e17587;
            let (assign12920_body100_e17606,) = {
    if (((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard400 != 0.0)) && (locals.var_guard401 != 0.0)) && (locals.var_guard402 == 0.0)) && (locals.var_guard403 == 0.0)) && (locals.var_guard404 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign12920_body100_e17606;
            let assign12920_body101_e17609: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
            locals.var_guard405 = assign12920_body101_e17609;
            let (assign12920_body102_e17631,) = {
    if ((((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard400 != 0.0)) && (locals.var_guard401 != 0.0)) && (locals.var_guard402 == 0.0)) && (locals.var_guard403 == 0.0)) && (locals.var_guard404 == 0.0)) && (locals.var_guard405 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign12920_body102_e17631;
            let (assign12920_body103_e17642,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard400 != 0.0)) && (locals.var_guard401 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign12920_body103_e17642;
            let mut assign12920_body104_loop_guard: usize = 0;
            while {
                let assign12920_body104_cond_e17654: f64 = if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard400 != 0.0)) && (locals.var_guard401 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
                assign12920_body104_cond_e17654 != 0.0
            } {
                assign12920_body104_loop_guard += 1;
                assert!(assign12920_body104_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                let (assign12920_body104_body0_e17666, assign12920_body104_body0_e17666_d_n0, assign12920_body104_body0_e17666_d_n2, assign12920_body104_body0_e17666_d_n6, assign12920_body104_body0_e17666_d_n7, assign12920_body104_body0_e17666_d_n10, assign12920_body104_body0_e17666_d_n11, assign12920_body104_body0_e17666_d_n12, assign12920_body104_body0_e17666_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard400 != 0.0)) && (locals.var_guard401 != 0.0)) {
        let assign12920_body104_body0_e17664: f64 = (locals.var_dnm).sqrt();
        (assign12920_body104_body0_e17664, (locals.var_dnm_dn0 / (2.0 * assign12920_body104_body0_e17664)), (locals.var_dnm_dn2 / (2.0 * assign12920_body104_body0_e17664)), (locals.var_dnm_dn6 / (2.0 * assign12920_body104_body0_e17664)), (locals.var_dnm_dn7 / (2.0 * assign12920_body104_body0_e17664)), (locals.var_dnm_dn10 / (2.0 * assign12920_body104_body0_e17664)), (locals.var_dnm_dn11 / (2.0 * assign12920_body104_body0_e17664)), (locals.var_dnm_dn12 / (2.0 * assign12920_body104_body0_e17664)), (locals.var_dnm_dn17 / (2.0 * assign12920_body104_body0_e17664)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
                locals.var_dnm = assign12920_body104_body0_e17666;
                locals.var_dnm_dn0 = assign12920_body104_body0_e17666_d_n0;
                locals.var_dnm_dn2 = assign12920_body104_body0_e17666_d_n2;
                locals.var_dnm_dn6 = assign12920_body104_body0_e17666_d_n6;
                locals.var_dnm_dn7 = assign12920_body104_body0_e17666_d_n7;
                locals.var_dnm_dn10 = assign12920_body104_body0_e17666_d_n10;
                locals.var_dnm_dn11 = assign12920_body104_body0_e17666_d_n11;
                locals.var_dnm_dn12 = assign12920_body104_body0_e17666_d_n12;
                locals.var_dnm_dn17 = assign12920_body104_body0_e17666_d_n17;
                let (assign12920_body104_body1_e17679,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard400 != 0.0)) && (locals.var_guard401 != 0.0)) {
        let assign12920_body104_body1_e17677: f64 = (locals.var_m0 + 1.0);
        (assign12920_body104_body1_e17677,)
    } else {
        (locals.var_m0,)
    }
};
                locals.var_m0 = assign12920_body104_body1_e17679;
            }
            let (assign12920_body105_e17697, assign12920_body105_e17697_d_n0, assign12920_body105_e17697_d_n2, assign12920_body105_e17697_d_n6, assign12920_body105_e17697_d_n7, assign12920_body105_e17697_d_n10, assign12920_body105_e17697_d_n11, assign12920_body105_e17697_d_n12, assign12920_body105_e17697_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard400 != 0.0)) && (locals.var_guard401 == 0.0)) {
        let assign12920_body105_e17693: f64 = (2.0 * 2.0);
        let assign12920_body105_e17694: f64 = (1.0 / assign12920_body105_e17693);
        let assign12920_body105_e17695: f64 = (locals.var_dnm).powf(assign12920_body105_e17694);
        (assign12920_body105_e17695, if 0.0 == 0.0 && ((assign12920_body105_e17694) as f64).is_finite() && ((assign12920_body105_e17694) as f64).fract() == 0.0 { if assign12920_body105_e17694 == 0.0 { 0.0 } else { (assign12920_body105_e17694 * ((locals.var_dnm).powf(assign12920_body105_e17694 - 1.0) * locals.var_dnm_dn0)) } } else { (assign12920_body105_e17695 * (assign12920_body105_e17694 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12920_body105_e17694) as f64).is_finite() && ((assign12920_body105_e17694) as f64).fract() == 0.0 { if assign12920_body105_e17694 == 0.0 { 0.0 } else { (assign12920_body105_e17694 * ((locals.var_dnm).powf(assign12920_body105_e17694 - 1.0) * locals.var_dnm_dn2)) } } else { (assign12920_body105_e17695 * (assign12920_body105_e17694 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12920_body105_e17694) as f64).is_finite() && ((assign12920_body105_e17694) as f64).fract() == 0.0 { if assign12920_body105_e17694 == 0.0 { 0.0 } else { (assign12920_body105_e17694 * ((locals.var_dnm).powf(assign12920_body105_e17694 - 1.0) * locals.var_dnm_dn6)) } } else { (assign12920_body105_e17695 * (assign12920_body105_e17694 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12920_body105_e17694) as f64).is_finite() && ((assign12920_body105_e17694) as f64).fract() == 0.0 { if assign12920_body105_e17694 == 0.0 { 0.0 } else { (assign12920_body105_e17694 * ((locals.var_dnm).powf(assign12920_body105_e17694 - 1.0) * locals.var_dnm_dn7)) } } else { (assign12920_body105_e17695 * (assign12920_body105_e17694 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12920_body105_e17694) as f64).is_finite() && ((assign12920_body105_e17694) as f64).fract() == 0.0 { if assign12920_body105_e17694 == 0.0 { 0.0 } else { (assign12920_body105_e17694 * ((locals.var_dnm).powf(assign12920_body105_e17694 - 1.0) * locals.var_dnm_dn10)) } } else { (assign12920_body105_e17695 * (assign12920_body105_e17694 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12920_body105_e17694) as f64).is_finite() && ((assign12920_body105_e17694) as f64).fract() == 0.0 { if assign12920_body105_e17694 == 0.0 { 0.0 } else { (assign12920_body105_e17694 * ((locals.var_dnm).powf(assign12920_body105_e17694 - 1.0) * locals.var_dnm_dn11)) } } else { (assign12920_body105_e17695 * (assign12920_body105_e17694 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12920_body105_e17694) as f64).is_finite() && ((assign12920_body105_e17694) as f64).fract() == 0.0 { if assign12920_body105_e17694 == 0.0 { 0.0 } else { (assign12920_body105_e17694 * ((locals.var_dnm).powf(assign12920_body105_e17694 - 1.0) * locals.var_dnm_dn12)) } } else { (assign12920_body105_e17695 * (assign12920_body105_e17694 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12920_body105_e17694) as f64).is_finite() && ((assign12920_body105_e17694) as f64).fract() == 0.0 { if assign12920_body105_e17694 == 0.0 { 0.0 } else { (assign12920_body105_e17694 * ((locals.var_dnm).powf(assign12920_body105_e17694 - 1.0) * locals.var_dnm_dn17)) } } else { (assign12920_body105_e17695 * (assign12920_body105_e17694 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign12920_body105_e17697;
            locals.var_dnm_dn0 = assign12920_body105_e17697_d_n0;
            locals.var_dnm_dn2 = assign12920_body105_e17697_d_n2;
            locals.var_dnm_dn6 = assign12920_body105_e17697_d_n6;
            locals.var_dnm_dn7 = assign12920_body105_e17697_d_n7;
            locals.var_dnm_dn10 = assign12920_body105_e17697_d_n10;
            locals.var_dnm_dn11 = assign12920_body105_e17697_d_n11;
            locals.var_dnm_dn12 = assign12920_body105_e17697_d_n12;
            locals.var_dnm_dn17 = assign12920_body105_e17697_d_n17;
            let (assign12920_body106_e17708, assign12920_body106_e17708_d_n0, assign12920_body106_e17708_d_n2, assign12920_body106_e17708_d_n6, assign12920_body106_e17708_d_n7, assign12920_body106_e17708_d_n10, assign12920_body106_e17708_d_n11, assign12920_body106_e17708_d_n12, assign12920_body106_e17708_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let assign12920_body106_e17706: f64 = (1.0 / locals.var_dnm);
        (assign12920_body106_e17706, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign12920_body106_e17708;
            locals.var_dnm_dn0 = assign12920_body106_e17708_d_n0;
            locals.var_dnm_dn2 = assign12920_body106_e17708_d_n2;
            locals.var_dnm_dn6 = assign12920_body106_e17708_d_n6;
            locals.var_dnm_dn7 = assign12920_body106_e17708_d_n7;
            locals.var_dnm_dn10 = assign12920_body106_e17708_d_n10;
            locals.var_dnm_dn11 = assign12920_body106_e17708_d_n11;
            locals.var_dnm_dn12 = assign12920_body106_e17708_d_n12;
            locals.var_dnm_dn17 = assign12920_body106_e17708_d_n17;
            let (assign12920_body107_e17726, assign12920_body107_e17726_d_n0, assign12920_body107_e17726_d_n2, assign12920_body107_e17726_d_n6, assign12920_body107_e17726_d_n7, assign12920_body107_e17726_d_n10, assign12920_body107_e17726_d_n11, assign12920_body107_e17726_d_n12, assign12920_body107_e17726_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let assign12920_body107_e17718: f64 = (locals.var_q_fd_soi - locals.var_q_sl_dep);
        let assign12920_body107_e17719: f64 = (-assign12920_body107_e17718);
        let assign12920_body107_e17721: f64 = assign12920_body107_e17719;
        let assign12920_body107_e17722: f64 = (locals.var_tmf1 * assign12920_body107_e17721);
        let assign12920_body107_e17724: f64 = (assign12920_body107_e17722 * locals.var_dnm);
        (assign12920_body107_e17724, ((((locals.var_tmf1_dn0 * assign12920_body107_e17721) + (locals.var_tmf1 * (-(locals.var_q_fd_soi_dn0 - locals.var_q_sl_dep_dn0)))) * locals.var_dnm) + (assign12920_body107_e17722 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign12920_body107_e17721) + (locals.var_tmf1 * (-(locals.var_q_fd_soi_dn2 - locals.var_q_sl_dep_dn2)))) * locals.var_dnm) + (assign12920_body107_e17722 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn6 * assign12920_body107_e17721) + (locals.var_tmf1 * (-(locals.var_q_fd_soi_dn6 - locals.var_q_sl_dep_dn6)))) * locals.var_dnm) + (assign12920_body107_e17722 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign12920_body107_e17721) + (locals.var_tmf1 * (-(locals.var_q_fd_soi_dn7 - locals.var_q_sl_dep_dn7)))) * locals.var_dnm) + (assign12920_body107_e17722 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn10 * assign12920_body107_e17721) + (locals.var_tmf1 * (-(locals.var_q_fd_soi_dn10 - locals.var_q_sl_dep_dn10)))) * locals.var_dnm) + (assign12920_body107_e17722 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * assign12920_body107_e17721) + (locals.var_tmf1 * (-(locals.var_q_fd_soi_dn11 - locals.var_q_sl_dep_dn11)))) * locals.var_dnm) + (assign12920_body107_e17722 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn12 * assign12920_body107_e17721) + (locals.var_tmf1 * (-(locals.var_q_fd_soi_dn12 - locals.var_q_sl_dep_dn12)))) * locals.var_dnm) + (assign12920_body107_e17722 * locals.var_dnm_dn12)), ((((locals.var_tmf1_dn17 * assign12920_body107_e17721) + (locals.var_tmf1 * (-(locals.var_q_fd_soi_dn17 - locals.var_q_sl_dep_dn17)))) * locals.var_dnm) + (assign12920_body107_e17722 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0__blk385, locals.var_tmf0__blk385_dn0, locals.var_tmf0__blk385_dn2, locals.var_tmf0__blk385_dn6, locals.var_tmf0__blk385_dn7, locals.var_tmf0__blk385_dn10, locals.var_tmf0__blk385_dn11, locals.var_tmf0__blk385_dn12, locals.var_tmf0__blk385_dn17,)
    }
};
            locals.var_tmf0__blk385 = assign12920_body107_e17726;
            locals.var_tmf0__blk385_dn0 = assign12920_body107_e17726_d_n0;
            locals.var_tmf0__blk385_dn2 = assign12920_body107_e17726_d_n2;
            locals.var_tmf0__blk385_dn6 = assign12920_body107_e17726_d_n6;
            locals.var_tmf0__blk385_dn7 = assign12920_body107_e17726_d_n7;
            locals.var_tmf0__blk385_dn10 = assign12920_body107_e17726_d_n10;
            locals.var_tmf0__blk385_dn11 = assign12920_body107_e17726_d_n11;
            locals.var_tmf0__blk385_dn12 = assign12920_body107_e17726_d_n12;
            locals.var_tmf0__blk385_dn17 = assign12920_body107_e17726_d_n17;
            let (assign12920_body108_e17746, assign12920_body108_e17746_d_n0, assign12920_body108_e17746_d_n2, assign12920_body108_e17746_d_n6, assign12920_body108_e17746_d_n7, assign12920_body108_e17746_d_n10, assign12920_body108_e17746_d_n11, assign12920_body108_e17746_d_n12, assign12920_body108_e17746_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let assign12920_body108_e17735: f64 = (locals.var_q_fd_soi - locals.var_q_sl_dep);
        let assign12920_body108_e17736: f64 = (-assign12920_body108_e17735);
        let assign12920_body108_e17738: f64 = assign12920_body108_e17736;
        let assign12920_body108_e17740: f64 = (assign12920_body108_e17738 * locals.var_xmp);
        let assign12920_body108_e17742: f64 = (assign12920_body108_e17740 * locals.var_dnm);
        let assign12920_body108_e17744: f64 = (assign12920_body108_e17742 / locals.var_arg);
        (assign12920_body108_e17744, ((((((((-(locals.var_q_fd_soi_dn0 - locals.var_q_sl_dep_dn0)) * locals.var_xmp) + (assign12920_body108_e17738 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign12920_body108_e17740 * locals.var_dnm_dn0)) * locals.var_arg) - (assign12920_body108_e17742 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((-(locals.var_q_fd_soi_dn2 - locals.var_q_sl_dep_dn2)) * locals.var_xmp) + (assign12920_body108_e17738 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign12920_body108_e17740 * locals.var_dnm_dn2)) * locals.var_arg) - (assign12920_body108_e17742 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((-(locals.var_q_fd_soi_dn6 - locals.var_q_sl_dep_dn6)) * locals.var_xmp) + (assign12920_body108_e17738 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign12920_body108_e17740 * locals.var_dnm_dn6)) * locals.var_arg) - (assign12920_body108_e17742 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((-(locals.var_q_fd_soi_dn7 - locals.var_q_sl_dep_dn7)) * locals.var_xmp) + (assign12920_body108_e17738 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign12920_body108_e17740 * locals.var_dnm_dn7)) * locals.var_arg) - (assign12920_body108_e17742 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((-(locals.var_q_fd_soi_dn10 - locals.var_q_sl_dep_dn10)) * locals.var_xmp) + (assign12920_body108_e17738 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign12920_body108_e17740 * locals.var_dnm_dn10)) * locals.var_arg) - (assign12920_body108_e17742 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((-(locals.var_q_fd_soi_dn11 - locals.var_q_sl_dep_dn11)) * locals.var_xmp) + (assign12920_body108_e17738 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign12920_body108_e17740 * locals.var_dnm_dn11)) * locals.var_arg) - (assign12920_body108_e17742 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((((-(locals.var_q_fd_soi_dn12 - locals.var_q_sl_dep_dn12)) * locals.var_xmp) + (assign12920_body108_e17738 * locals.var_xmp_dn12)) * locals.var_dnm) + (assign12920_body108_e17740 * locals.var_dnm_dn12)) * locals.var_arg) - (assign12920_body108_e17742 * locals.var_arg_dn12)) / (locals.var_arg * locals.var_arg)), ((((((((-(locals.var_q_fd_soi_dn17 - locals.var_q_sl_dep_dn17)) * locals.var_xmp) + (assign12920_body108_e17738 * locals.var_xmp_dn17)) * locals.var_dnm) + (assign12920_body108_e17740 * locals.var_dnm_dn17)) * locals.var_arg) - (assign12920_body108_e17742 * locals.var_arg_dn17)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign12920_body108_e17746;
            locals.var_t0_dn0 = assign12920_body108_e17746_d_n0;
            locals.var_t0_dn2 = assign12920_body108_e17746_d_n2;
            locals.var_t0_dn6 = assign12920_body108_e17746_d_n6;
            locals.var_t0_dn7 = assign12920_body108_e17746_d_n7;
            locals.var_t0_dn10 = assign12920_body108_e17746_d_n10;
            locals.var_t0_dn11 = assign12920_body108_e17746_d_n11;
            locals.var_t0_dn12 = assign12920_body108_e17746_d_n12;
            locals.var_t0_dn17 = assign12920_body108_e17746_d_n17;
            let (assign12920_body109_e17766, assign12920_body109_e17766_d_n0, assign12920_body109_e17766_d_n2, assign12920_body109_e17766_d_n6, assign12920_body109_e17766_d_n7, assign12920_body109_e17766_d_n10, assign12920_body109_e17766_d_n11, assign12920_body109_e17766_d_n12, assign12920_body109_e17766_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard400 != 0.0)) {
        let assign12920_body109_e17755: f64 = (locals.var_q_fd_soi - locals.var_q_sl_dep);
        let assign12920_body109_e17758: f64 = (locals.var_q_fd_soi - locals.var_q_sl_dep);
        let assign12920_body109_e17759: f64 = (-assign12920_body109_e17758);
        let assign12920_body109_e17761: f64 = assign12920_body109_e17759;
        let assign12920_body109_e17762: f64 = (assign12920_body109_e17755 + assign12920_body109_e17761);
        let assign12920_body109_e17764: f64 = (assign12920_body109_e17762 - locals.var_tmf0__blk385);
        (assign12920_body109_e17764, (((locals.var_q_fd_soi_dn0 - locals.var_q_sl_dep_dn0) + (-(locals.var_q_fd_soi_dn0 - locals.var_q_sl_dep_dn0))) - locals.var_tmf0__blk385_dn0), (((locals.var_q_fd_soi_dn2 - locals.var_q_sl_dep_dn2) + (-(locals.var_q_fd_soi_dn2 - locals.var_q_sl_dep_dn2))) - locals.var_tmf0__blk385_dn2), (((locals.var_q_fd_soi_dn6 - locals.var_q_sl_dep_dn6) + (-(locals.var_q_fd_soi_dn6 - locals.var_q_sl_dep_dn6))) - locals.var_tmf0__blk385_dn6), (((locals.var_q_fd_soi_dn7 - locals.var_q_sl_dep_dn7) + (-(locals.var_q_fd_soi_dn7 - locals.var_q_sl_dep_dn7))) - locals.var_tmf0__blk385_dn7), (((locals.var_q_fd_soi_dn10 - locals.var_q_sl_dep_dn10) + (-(locals.var_q_fd_soi_dn10 - locals.var_q_sl_dep_dn10))) - locals.var_tmf0__blk385_dn10), (((locals.var_q_fd_soi_dn11 - locals.var_q_sl_dep_dn11) + (-(locals.var_q_fd_soi_dn11 - locals.var_q_sl_dep_dn11))) - locals.var_tmf0__blk385_dn11), (((locals.var_q_fd_soi_dn12 - locals.var_q_sl_dep_dn12) + (-(locals.var_q_fd_soi_dn12 - locals.var_q_sl_dep_dn12))) - locals.var_tmf0__blk385_dn12), (((locals.var_q_fd_soi_dn17 - locals.var_q_sl_dep_dn17) + (-(locals.var_q_fd_soi_dn17 - locals.var_q_sl_dep_dn17))) - locals.var_tmf0__blk385_dn17),)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn12, locals.var_q_bl_dep_dn17,)
    }
};
            locals.var_q_bl_dep = assign12920_body109_e17766;
            locals.var_q_bl_dep_dn0 = assign12920_body109_e17766_d_n0;
            locals.var_q_bl_dep_dn2 = assign12920_body109_e17766_d_n2;
            locals.var_q_bl_dep_dn6 = assign12920_body109_e17766_d_n6;
            locals.var_q_bl_dep_dn7 = assign12920_body109_e17766_d_n7;
            locals.var_q_bl_dep_dn10 = assign12920_body109_e17766_d_n10;
            locals.var_q_bl_dep_dn11 = assign12920_body109_e17766_d_n11;
            locals.var_q_bl_dep_dn12 = assign12920_body109_e17766_d_n12;
            locals.var_q_bl_dep_dn17 = assign12920_body109_e17766_d_n17;
            let (assign12920_body110_e17775, assign12920_body110_e17775_d_n0, assign12920_body110_e17775_d_n2, assign12920_body110_e17775_d_n6, assign12920_body110_e17775_d_n7, assign12920_body110_e17775_d_n10, assign12920_body110_e17775_d_n11, assign12920_body110_e17775_d_n12, assign12920_body110_e17775_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard400 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign12920_body110_e17775;
            locals.var_t0_dn0 = assign12920_body110_e17775_d_n0;
            locals.var_t0_dn2 = assign12920_body110_e17775_d_n2;
            locals.var_t0_dn6 = assign12920_body110_e17775_d_n6;
            locals.var_t0_dn7 = assign12920_body110_e17775_d_n7;
            locals.var_t0_dn10 = assign12920_body110_e17775_d_n10;
            locals.var_t0_dn11 = assign12920_body110_e17775_d_n11;
            locals.var_t0_dn12 = assign12920_body110_e17775_d_n12;
            locals.var_t0_dn17 = assign12920_body110_e17775_d_n17;
            let (assign12920_body111_e17785, assign12920_body111_e17785_d_n0, assign12920_body111_e17785_d_n2, assign12920_body111_e17785_d_n6, assign12920_body111_e17785_d_n7, assign12920_body111_e17785_d_n10, assign12920_body111_e17785_d_n11, assign12920_body111_e17785_d_n12, assign12920_body111_e17785_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard400 == 0.0)) {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn12, locals.var_q_bl_dep_dn17,)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn12, locals.var_q_bl_dep_dn17,)
    }
};
            locals.var_q_bl_dep = assign12920_body111_e17785;
            locals.var_q_bl_dep_dn0 = assign12920_body111_e17785_d_n0;
            locals.var_q_bl_dep_dn2 = assign12920_body111_e17785_d_n2;
            locals.var_q_bl_dep_dn6 = assign12920_body111_e17785_d_n6;
            locals.var_q_bl_dep_dn7 = assign12920_body111_e17785_d_n7;
            locals.var_q_bl_dep_dn10 = assign12920_body111_e17785_d_n10;
            locals.var_q_bl_dep_dn11 = assign12920_body111_e17785_d_n11;
            locals.var_q_bl_dep_dn12 = assign12920_body111_e17785_d_n12;
            locals.var_q_bl_dep_dn17 = assign12920_body111_e17785_d_n17;
            let (assign12920_body112_e17795, assign12920_body112_e17795_d_n0, assign12920_body112_e17795_d_n2, assign12920_body112_e17795_d_n6, assign12920_body112_e17795_d_n7, assign12920_body112_e17795_d_n10, assign12920_body112_e17795_d_n11, assign12920_body112_e17795_d_n12, assign12920_body112_e17795_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard400 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign12920_body112_e17795;
            locals.var_t0_dn0 = assign12920_body112_e17795_d_n0;
            locals.var_t0_dn2 = assign12920_body112_e17795_d_n2;
            locals.var_t0_dn6 = assign12920_body112_e17795_d_n6;
            locals.var_t0_dn7 = assign12920_body112_e17795_d_n7;
            locals.var_t0_dn10 = assign12920_body112_e17795_d_n10;
            locals.var_t0_dn11 = assign12920_body112_e17795_d_n11;
            locals.var_t0_dn12 = assign12920_body112_e17795_d_n12;
            locals.var_t0_dn17 = assign12920_body112_e17795_d_n17;
            let (assign12920_body113_e17804, assign12920_body113_e17804_d_n0, assign12920_body113_e17804_d_n2, assign12920_body113_e17804_d_n6, assign12920_body113_e17804_d_n7, assign12920_body113_e17804_d_n10, assign12920_body113_e17804_d_n11, assign12920_body113_e17804_d_n12, assign12920_body113_e17804_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign12920_body113_e17802: f64 = (locals.var_q_bl_dep_dpss * locals.var_t0);
        (assign12920_body113_e17802, ((locals.var_q_bl_dep_dpss_dn0 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn0)), ((locals.var_q_bl_dep_dpss_dn2 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn2)), ((locals.var_q_bl_dep_dpss_dn6 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn6)), ((locals.var_q_bl_dep_dpss_dn7 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn7)), ((locals.var_q_bl_dep_dpss_dn10 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn10)), ((locals.var_q_bl_dep_dpss_dn11 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn11)), ((locals.var_q_bl_dep_dpss_dn12 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn12)), ((locals.var_q_bl_dep_dpss_dn17 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn17)),)
    } else {
        (locals.var_q_bl_dep_dpss, locals.var_q_bl_dep_dpss_dn0, locals.var_q_bl_dep_dpss_dn2, locals.var_q_bl_dep_dpss_dn6, locals.var_q_bl_dep_dpss_dn7, locals.var_q_bl_dep_dpss_dn10, locals.var_q_bl_dep_dpss_dn11, locals.var_q_bl_dep_dpss_dn12, locals.var_q_bl_dep_dpss_dn17,)
    }
};
            locals.var_q_bl_dep_dpss = assign12920_body113_e17804;
            locals.var_q_bl_dep_dpss_dn0 = assign12920_body113_e17804_d_n0;
            locals.var_q_bl_dep_dpss_dn2 = assign12920_body113_e17804_d_n2;
            locals.var_q_bl_dep_dpss_dn6 = assign12920_body113_e17804_d_n6;
            locals.var_q_bl_dep_dpss_dn7 = assign12920_body113_e17804_d_n7;
            locals.var_q_bl_dep_dpss_dn10 = assign12920_body113_e17804_d_n10;
            locals.var_q_bl_dep_dpss_dn11 = assign12920_body113_e17804_d_n11;
            locals.var_q_bl_dep_dpss_dn12 = assign12920_body113_e17804_d_n12;
            locals.var_q_bl_dep_dpss_dn17 = assign12920_body113_e17804_d_n17;
            let (assign12920_body114_e17813, assign12920_body114_e17813_d_n0, assign12920_body114_e17813_d_n2, assign12920_body114_e17813_d_n6, assign12920_body114_e17813_d_n7, assign12920_body114_e17813_d_n10, assign12920_body114_e17813_d_n11, assign12920_body114_e17813_d_n12, assign12920_body114_e17813_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign12920_body114_e17811: f64 = (locals.var_q_bl_dep_dpbs * locals.var_t0);
        (assign12920_body114_e17811, ((locals.var_q_bl_dep_dpbs_dn0 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn0)), ((locals.var_q_bl_dep_dpbs_dn2 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn2)), ((locals.var_q_bl_dep_dpbs_dn6 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn6)), ((locals.var_q_bl_dep_dpbs_dn7 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn7)), ((locals.var_q_bl_dep_dpbs_dn10 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn10)), ((locals.var_q_bl_dep_dpbs_dn11 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn11)), ((locals.var_q_bl_dep_dpbs_dn12 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn12)), ((locals.var_q_bl_dep_dpbs_dn17 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn17)),)
    } else {
        (locals.var_q_bl_dep_dpbs, locals.var_q_bl_dep_dpbs_dn0, locals.var_q_bl_dep_dpbs_dn2, locals.var_q_bl_dep_dpbs_dn6, locals.var_q_bl_dep_dpbs_dn7, locals.var_q_bl_dep_dpbs_dn10, locals.var_q_bl_dep_dpbs_dn11, locals.var_q_bl_dep_dpbs_dn12, locals.var_q_bl_dep_dpbs_dn17,)
    }
};
            locals.var_q_bl_dep_dpbs = assign12920_body114_e17813;
            locals.var_q_bl_dep_dpbs_dn0 = assign12920_body114_e17813_d_n0;
            locals.var_q_bl_dep_dpbs_dn2 = assign12920_body114_e17813_d_n2;
            locals.var_q_bl_dep_dpbs_dn6 = assign12920_body114_e17813_d_n6;
            locals.var_q_bl_dep_dpbs_dn7 = assign12920_body114_e17813_d_n7;
            locals.var_q_bl_dep_dpbs_dn10 = assign12920_body114_e17813_d_n10;
            locals.var_q_bl_dep_dpbs_dn11 = assign12920_body114_e17813_d_n11;
            locals.var_q_bl_dep_dpbs_dn12 = assign12920_body114_e17813_d_n12;
            locals.var_q_bl_dep_dpbs_dn17 = assign12920_body114_e17813_d_n17;
            let (assign12920_body115_e17822, assign12920_body115_e17822_d_n0, assign12920_body115_e17822_d_n2, assign12920_body115_e17822_d_n6, assign12920_body115_e17822_d_n7, assign12920_body115_e17822_d_n10, assign12920_body115_e17822_d_n11, assign12920_body115_e17822_d_n12, assign12920_body115_e17822_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign12920_body115_e17820: f64 = (locals.var_q_sl_dep + locals.var_q_bl_dep);
        (assign12920_body115_e17820, (locals.var_q_sl_dep_dn0 + locals.var_q_bl_dep_dn0), (locals.var_q_sl_dep_dn2 + locals.var_q_bl_dep_dn2), (locals.var_q_sl_dep_dn6 + locals.var_q_bl_dep_dn6), (locals.var_q_sl_dep_dn7 + locals.var_q_bl_dep_dn7), (locals.var_q_sl_dep_dn10 + locals.var_q_bl_dep_dn10), (locals.var_q_sl_dep_dn11 + locals.var_q_bl_dep_dn11), (locals.var_q_sl_dep_dn12 + locals.var_q_bl_dep_dn12), (locals.var_q_sl_dep_dn17 + locals.var_q_bl_dep_dn17),)
    } else {
        (locals.var_q_depl, locals.var_q_depl_dn0, locals.var_q_depl_dn2, locals.var_q_depl_dn6, locals.var_q_depl_dn7, locals.var_q_depl_dn10, locals.var_q_depl_dn11, locals.var_q_depl_dn12, locals.var_q_depl_dn17,)
    }
};
            locals.var_q_depl = assign12920_body115_e17822;
            locals.var_q_depl_dn0 = assign12920_body115_e17822_d_n0;
            locals.var_q_depl_dn2 = assign12920_body115_e17822_d_n2;
            locals.var_q_depl_dn6 = assign12920_body115_e17822_d_n6;
            locals.var_q_depl_dn7 = assign12920_body115_e17822_d_n7;
            locals.var_q_depl_dn10 = assign12920_body115_e17822_d_n10;
            locals.var_q_depl_dn11 = assign12920_body115_e17822_d_n11;
            locals.var_q_depl_dn12 = assign12920_body115_e17822_d_n12;
            locals.var_q_depl_dn17 = assign12920_body115_e17822_d_n17;
            let assign12920_body116_e17829: f64 = if ((locals.var_flg_conv == 1.0) && (locals.var_lp_sl > 3.0)) { 1.0 } else { 0.0 };
            locals.var_guard406 = assign12920_body116_e17829;
            let (assign12920_body117_e17838,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 != 0.0)) {
        (locals.var_lp_sl,)
    } else {
        (locals.var_flg_brk8,)
    }
};
            locals.var_flg_brk8 = assign12920_body117_e17838;
            let (assign12920_body118_e17847,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 != 0.0)) {
        (locals.var_lp_sl_max,)
    } else {
        (locals.var_lp_sl,)
    }
};
            locals.var_lp_sl = assign12920_body118_e17847;
            let (assign12920_body119_e17871, assign12920_body119_e17871_d_n0, assign12920_body119_e17871_d_n2, assign12920_body119_e17871_d_n6, assign12920_body119_e17871_d_n7, assign12920_body119_e17871_d_n10, assign12920_body119_e17871_d_n11, assign12920_body119_e17871_d_n12, assign12920_body119_e17871_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) {
        let assign12920_body119_e17857: f64 = (locals.var_phi_sl_soi - locals.var_vgpz);
        let assign12920_body119_e17861: f64 = (locals.var_q_sl_bulk + locals.var_q_sl_dep);
        let assign12920_body119_e17863: f64 = (assign12920_body119_e17861 + locals.var_q_nl);
        let assign12920_body119_e17865: f64 = (assign12920_body119_e17863 + locals.var_q_bl_dep);
        let assign12920_body119_e17867: f64 = (assign12920_body119_e17865 + locals.var_qhs);
        let assign12920_body119_e17868: f64 = (locals.var_c_fox_inv * assign12920_body119_e17867);
        let assign12920_body119_e17869: f64 = (assign12920_body119_e17857 - assign12920_body119_e17868);
        (assign12920_body119_e17869, ((locals.var_phi_sl_soi_dn0 - locals.var_vgpz_dn0) - ((locals.var_c_fox_inv_dn0 * assign12920_body119_e17867) + (locals.var_c_fox_inv * ((((locals.var_q_sl_bulk_dn0 + locals.var_q_sl_dep_dn0) + locals.var_q_nl_dn0) + locals.var_q_bl_dep_dn0) + locals.var_qhs_dn0)))), ((locals.var_phi_sl_soi_dn2 - locals.var_vgpz_dn2) - ((locals.var_c_fox_inv_dn2 * assign12920_body119_e17867) + (locals.var_c_fox_inv * ((((locals.var_q_sl_bulk_dn2 + locals.var_q_sl_dep_dn2) + locals.var_q_nl_dn2) + locals.var_q_bl_dep_dn2) + locals.var_qhs_dn2)))), ((locals.var_phi_sl_soi_dn6 - locals.var_vgpz_dn6) - ((locals.var_c_fox_inv_dn6 * assign12920_body119_e17867) + (locals.var_c_fox_inv * ((((locals.var_q_sl_bulk_dn6 + locals.var_q_sl_dep_dn6) + locals.var_q_nl_dn6) + locals.var_q_bl_dep_dn6) + locals.var_qhs_dn6)))), ((locals.var_phi_sl_soi_dn7 - locals.var_vgpz_dn7) - ((locals.var_c_fox_inv_dn7 * assign12920_body119_e17867) + (locals.var_c_fox_inv * ((((locals.var_q_sl_bulk_dn7 + locals.var_q_sl_dep_dn7) + locals.var_q_nl_dn7) + locals.var_q_bl_dep_dn7) + locals.var_qhs_dn7)))), ((locals.var_phi_sl_soi_dn10 - locals.var_vgpz_dn10) - ((locals.var_c_fox_inv_dn10 * assign12920_body119_e17867) + (locals.var_c_fox_inv * ((((locals.var_q_sl_bulk_dn10 + locals.var_q_sl_dep_dn10) + locals.var_q_nl_dn10) + locals.var_q_bl_dep_dn10) + locals.var_qhs_dn10)))), ((locals.var_phi_sl_soi_dn11 - locals.var_vgpz_dn11) - ((locals.var_c_fox_inv_dn11 * assign12920_body119_e17867) + (locals.var_c_fox_inv * ((((locals.var_q_sl_bulk_dn11 + locals.var_q_sl_dep_dn11) + locals.var_q_nl_dn11) + locals.var_q_bl_dep_dn11) + locals.var_qhs_dn11)))), ((locals.var_phi_sl_soi_dn12 - locals.var_vgpz_dn12) - ((locals.var_c_fox_inv_dn12 * assign12920_body119_e17867) + (locals.var_c_fox_inv * ((((locals.var_q_sl_bulk_dn12 + locals.var_q_sl_dep_dn12) + locals.var_q_nl_dn12) + locals.var_q_bl_dep_dn12) + locals.var_qhs_dn12)))), ((locals.var_phi_sl_soi_dn17 - locals.var_vgpz_dn17) - ((locals.var_c_fox_inv_dn17 * assign12920_body119_e17867) + (locals.var_c_fox_inv * ((((locals.var_q_sl_bulk_dn17 + locals.var_q_sl_dep_dn17) + locals.var_q_nl_dn17) + locals.var_q_bl_dep_dn17) + locals.var_qhs_dn17)))),)
    } else {
        (locals.var_pf1__blk363, locals.var_pf1__blk363_dn0, locals.var_pf1__blk363_dn2, locals.var_pf1__blk363_dn6, locals.var_pf1__blk363_dn7, locals.var_pf1__blk363_dn10, locals.var_pf1__blk363_dn11, locals.var_pf1__blk363_dn12, locals.var_pf1__blk363_dn17,)
    }
};
            locals.var_pf1__blk363 = assign12920_body119_e17871;
            locals.var_pf1__blk363_dn0 = assign12920_body119_e17871_d_n0;
            locals.var_pf1__blk363_dn2 = assign12920_body119_e17871_d_n2;
            locals.var_pf1__blk363_dn6 = assign12920_body119_e17871_d_n6;
            locals.var_pf1__blk363_dn7 = assign12920_body119_e17871_d_n7;
            locals.var_pf1__blk363_dn10 = assign12920_body119_e17871_d_n10;
            locals.var_pf1__blk363_dn11 = assign12920_body119_e17871_d_n11;
            locals.var_pf1__blk363_dn12 = assign12920_body119_e17871_d_n12;
            locals.var_pf1__blk363_dn17 = assign12920_body119_e17871_d_n17;
            let (assign12920_body120_e17887, assign12920_body120_e17887_d_n0, assign12920_body120_e17887_d_n2, assign12920_body120_e17887_d_n6, assign12920_body120_e17887_d_n7, assign12920_body120_e17887_d_n10, assign12920_body120_e17887_d_n11, assign12920_body120_e17887_d_n12, assign12920_body120_e17887_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) {
        let assign12920_body120_e17883: f64 = (locals.var_q_nl_dpss + locals.var_q_bl_dep_dpss);
        let assign12920_body120_e17884: f64 = (locals.var_c_fox_inv * assign12920_body120_e17883);
        let assign12920_body120_e17885: f64 = (1.0 - assign12920_body120_e17884);
        (assign12920_body120_e17885, (-((locals.var_c_fox_inv_dn0 * assign12920_body120_e17883) + (locals.var_c_fox_inv * (locals.var_q_nl_dpss_dn0 + locals.var_q_bl_dep_dpss_dn0)))), (-((locals.var_c_fox_inv_dn2 * assign12920_body120_e17883) + (locals.var_c_fox_inv * (locals.var_q_nl_dpss_dn2 + locals.var_q_bl_dep_dpss_dn2)))), (-((locals.var_c_fox_inv_dn6 * assign12920_body120_e17883) + (locals.var_c_fox_inv * (locals.var_q_nl_dpss_dn6 + locals.var_q_bl_dep_dpss_dn6)))), (-((locals.var_c_fox_inv_dn7 * assign12920_body120_e17883) + (locals.var_c_fox_inv * (locals.var_q_nl_dpss_dn7 + locals.var_q_bl_dep_dpss_dn7)))), (-((locals.var_c_fox_inv_dn10 * assign12920_body120_e17883) + (locals.var_c_fox_inv * (locals.var_q_nl_dpss_dn10 + locals.var_q_bl_dep_dpss_dn10)))), (-((locals.var_c_fox_inv_dn11 * assign12920_body120_e17883) + (locals.var_c_fox_inv * (locals.var_q_nl_dpss_dn11 + locals.var_q_bl_dep_dpss_dn11)))), (-((locals.var_c_fox_inv_dn12 * assign12920_body120_e17883) + (locals.var_c_fox_inv * (locals.var_q_nl_dpss_dn12 + locals.var_q_bl_dep_dpss_dn12)))), (-((locals.var_c_fox_inv_dn17 * assign12920_body120_e17883) + (locals.var_c_fox_inv * (locals.var_q_nl_dpss_dn17 + locals.var_q_bl_dep_dpss_dn17)))),)
    } else {
        (locals.var_pf11__blk364, locals.var_pf11__blk364_dn0, locals.var_pf11__blk364_dn2, locals.var_pf11__blk364_dn6, locals.var_pf11__blk364_dn7, locals.var_pf11__blk364_dn10, locals.var_pf11__blk364_dn11, locals.var_pf11__blk364_dn12, locals.var_pf11__blk364_dn17,)
    }
};
            locals.var_pf11__blk364 = assign12920_body120_e17887;
            locals.var_pf11__blk364_dn0 = assign12920_body120_e17887_d_n0;
            locals.var_pf11__blk364_dn2 = assign12920_body120_e17887_d_n2;
            locals.var_pf11__blk364_dn6 = assign12920_body120_e17887_d_n6;
            locals.var_pf11__blk364_dn7 = assign12920_body120_e17887_d_n7;
            locals.var_pf11__blk364_dn10 = assign12920_body120_e17887_d_n10;
            locals.var_pf11__blk364_dn11 = assign12920_body120_e17887_d_n11;
            locals.var_pf11__blk364_dn12 = assign12920_body120_e17887_d_n12;
            locals.var_pf11__blk364_dn17 = assign12920_body120_e17887_d_n17;
            let (assign12920_body121_e17900, assign12920_body121_e17900_d_n0, assign12920_body121_e17900_d_n2, assign12920_body121_e17900_d_n6, assign12920_body121_e17900_d_n7, assign12920_body121_e17900_d_n10, assign12920_body121_e17900_d_n11, assign12920_body121_e17900_d_n12, assign12920_body121_e17900_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) {
        let assign12920_body121_e17896: f64 = (-locals.var_c_fox_inv);
        let assign12920_body121_e17898: f64 = (assign12920_body121_e17896 * locals.var_q_bl_dep_dpbs);
        (assign12920_body121_e17898, (((-locals.var_c_fox_inv_dn0) * locals.var_q_bl_dep_dpbs) + (assign12920_body121_e17896 * locals.var_q_bl_dep_dpbs_dn0)), (((-locals.var_c_fox_inv_dn2) * locals.var_q_bl_dep_dpbs) + (assign12920_body121_e17896 * locals.var_q_bl_dep_dpbs_dn2)), (((-locals.var_c_fox_inv_dn6) * locals.var_q_bl_dep_dpbs) + (assign12920_body121_e17896 * locals.var_q_bl_dep_dpbs_dn6)), (((-locals.var_c_fox_inv_dn7) * locals.var_q_bl_dep_dpbs) + (assign12920_body121_e17896 * locals.var_q_bl_dep_dpbs_dn7)), (((-locals.var_c_fox_inv_dn10) * locals.var_q_bl_dep_dpbs) + (assign12920_body121_e17896 * locals.var_q_bl_dep_dpbs_dn10)), (((-locals.var_c_fox_inv_dn11) * locals.var_q_bl_dep_dpbs) + (assign12920_body121_e17896 * locals.var_q_bl_dep_dpbs_dn11)), (((-locals.var_c_fox_inv_dn12) * locals.var_q_bl_dep_dpbs) + (assign12920_body121_e17896 * locals.var_q_bl_dep_dpbs_dn12)), (((-locals.var_c_fox_inv_dn17) * locals.var_q_bl_dep_dpbs) + (assign12920_body121_e17896 * locals.var_q_bl_dep_dpbs_dn17)),)
    } else {
        (locals.var_pf12__blk365, locals.var_pf12__blk365_dn0, locals.var_pf12__blk365_dn2, locals.var_pf12__blk365_dn6, locals.var_pf12__blk365_dn7, locals.var_pf12__blk365_dn10, locals.var_pf12__blk365_dn11, locals.var_pf12__blk365_dn12, locals.var_pf12__blk365_dn17,)
    }
};
            locals.var_pf12__blk365 = assign12920_body121_e17900;
            locals.var_pf12__blk365_dn0 = assign12920_body121_e17900_d_n0;
            locals.var_pf12__blk365_dn2 = assign12920_body121_e17900_d_n2;
            locals.var_pf12__blk365_dn6 = assign12920_body121_e17900_d_n6;
            locals.var_pf12__blk365_dn7 = assign12920_body121_e17900_d_n7;
            locals.var_pf12__blk365_dn10 = assign12920_body121_e17900_d_n10;
            locals.var_pf12__blk365_dn11 = assign12920_body121_e17900_d_n11;
            locals.var_pf12__blk365_dn12 = assign12920_body121_e17900_d_n12;
            locals.var_pf12__blk365_dn17 = assign12920_body121_e17900_d_n17;
            let (assign12920_body122_e17913, assign12920_body122_e17913_d_n0, assign12920_body122_e17913_d_n2, assign12920_body122_e17913_d_n6, assign12920_body122_e17913_d_n7, assign12920_body122_e17913_d_n10, assign12920_body122_e17913_d_n11, assign12920_body122_e17913_d_n12, assign12920_body122_e17913_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) {
        let assign12920_body122_e17909: f64 = (-locals.var_c_fox_inv);
        let assign12920_body122_e17911: f64 = (assign12920_body122_e17909 * locals.var_q_sl_bulk_dpsb);
        (assign12920_body122_e17911, (((-locals.var_c_fox_inv_dn0) * locals.var_q_sl_bulk_dpsb) + (assign12920_body122_e17909 * locals.var_q_sl_bulk_dpsb_dn0)), (((-locals.var_c_fox_inv_dn2) * locals.var_q_sl_bulk_dpsb) + (assign12920_body122_e17909 * locals.var_q_sl_bulk_dpsb_dn2)), (((-locals.var_c_fox_inv_dn6) * locals.var_q_sl_bulk_dpsb) + (assign12920_body122_e17909 * locals.var_q_sl_bulk_dpsb_dn6)), (((-locals.var_c_fox_inv_dn7) * locals.var_q_sl_bulk_dpsb) + (assign12920_body122_e17909 * locals.var_q_sl_bulk_dpsb_dn7)), (((-locals.var_c_fox_inv_dn10) * locals.var_q_sl_bulk_dpsb) + (assign12920_body122_e17909 * locals.var_q_sl_bulk_dpsb_dn10)), (((-locals.var_c_fox_inv_dn11) * locals.var_q_sl_bulk_dpsb) + (assign12920_body122_e17909 * locals.var_q_sl_bulk_dpsb_dn11)), (((-locals.var_c_fox_inv_dn12) * locals.var_q_sl_bulk_dpsb) + (assign12920_body122_e17909 * locals.var_q_sl_bulk_dpsb_dn12)), (((-locals.var_c_fox_inv_dn17) * locals.var_q_sl_bulk_dpsb) + (assign12920_body122_e17909 * locals.var_q_sl_bulk_dpsb_dn17)),)
    } else {
        (locals.var_pf13__blk366, locals.var_pf13__blk366_dn0, locals.var_pf13__blk366_dn2, locals.var_pf13__blk366_dn6, locals.var_pf13__blk366_dn7, locals.var_pf13__blk366_dn10, locals.var_pf13__blk366_dn11, locals.var_pf13__blk366_dn12, locals.var_pf13__blk366_dn17,)
    }
};
            locals.var_pf13__blk366 = assign12920_body122_e17913;
            locals.var_pf13__blk366_dn0 = assign12920_body122_e17913_d_n0;
            locals.var_pf13__blk366_dn2 = assign12920_body122_e17913_d_n2;
            locals.var_pf13__blk366_dn6 = assign12920_body122_e17913_d_n6;
            locals.var_pf13__blk366_dn7 = assign12920_body122_e17913_d_n7;
            locals.var_pf13__blk366_dn10 = assign12920_body122_e17913_d_n10;
            locals.var_pf13__blk366_dn11 = assign12920_body122_e17913_d_n11;
            locals.var_pf13__blk366_dn12 = assign12920_body122_e17913_d_n12;
            locals.var_pf13__blk366_dn17 = assign12920_body122_e17913_d_n17;
            let (assign12920_body123_e17931, assign12920_body123_e17931_d_n0, assign12920_body123_e17931_d_n2, assign12920_body123_e17931_d_n6, assign12920_body123_e17931_d_n7, assign12920_body123_e17931_d_n10, assign12920_body123_e17931_d_n11, assign12920_body123_e17931_d_n12, assign12920_body123_e17931_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) {
        let assign12920_body123_e17925: f64 = (0.5 * locals.var_q_fd_soi);
        let assign12920_body123_e17927: f64 = (assign12920_body123_e17925 + locals.var_q_sl_bulk);
        let assign12920_body123_e17928: f64 = (locals.var_c_soi_inv__blk115 * assign12920_body123_e17927);
        let assign12920_body123_e17929: f64 = (locals.var_phi_sl_soi + assign12920_body123_e17928);
        (assign12920_body123_e17929, (locals.var_phi_sl_soi_dn0 + (locals.var_c_soi_inv__blk115 * ((0.5 * locals.var_q_fd_soi_dn0) + locals.var_q_sl_bulk_dn0))), (locals.var_phi_sl_soi_dn2 + (locals.var_c_soi_inv__blk115 * ((0.5 * locals.var_q_fd_soi_dn2) + locals.var_q_sl_bulk_dn2))), (locals.var_phi_sl_soi_dn6 + (locals.var_c_soi_inv__blk115 * ((0.5 * locals.var_q_fd_soi_dn6) + locals.var_q_sl_bulk_dn6))), (locals.var_phi_sl_soi_dn7 + (locals.var_c_soi_inv__blk115 * ((0.5 * locals.var_q_fd_soi_dn7) + locals.var_q_sl_bulk_dn7))), (locals.var_phi_sl_soi_dn10 + (locals.var_c_soi_inv__blk115 * ((0.5 * locals.var_q_fd_soi_dn10) + locals.var_q_sl_bulk_dn10))), (locals.var_phi_sl_soi_dn11 + (locals.var_c_soi_inv__blk115 * ((0.5 * locals.var_q_fd_soi_dn11) + locals.var_q_sl_bulk_dn11))), (locals.var_phi_sl_soi_dn12 + (locals.var_c_soi_inv__blk115 * ((0.5 * locals.var_q_fd_soi_dn12) + locals.var_q_sl_bulk_dn12))), (locals.var_phi_sl_soi_dn17 + (locals.var_c_soi_inv__blk115 * ((0.5 * locals.var_q_fd_soi_dn17) + locals.var_q_sl_bulk_dn17))),)
    } else {
        (locals.var_t1__blk353, locals.var_t1__blk353_dn0, locals.var_t1__blk353_dn2, locals.var_t1__blk353_dn6, locals.var_t1__blk353_dn7, locals.var_t1__blk353_dn10, locals.var_t1__blk353_dn11, locals.var_t1__blk353_dn12, locals.var_t1__blk353_dn17,)
    }
};
            locals.var_t1__blk353 = assign12920_body123_e17931;
            locals.var_t1__blk353_dn0 = assign12920_body123_e17931_d_n0;
            locals.var_t1__blk353_dn2 = assign12920_body123_e17931_d_n2;
            locals.var_t1__blk353_dn6 = assign12920_body123_e17931_d_n6;
            locals.var_t1__blk353_dn7 = assign12920_body123_e17931_d_n7;
            locals.var_t1__blk353_dn10 = assign12920_body123_e17931_d_n10;
            locals.var_t1__blk353_dn11 = assign12920_body123_e17931_d_n11;
            locals.var_t1__blk353_dn12 = assign12920_body123_e17931_d_n12;
            locals.var_t1__blk353_dn17 = assign12920_body123_e17931_d_n17;
            let (assign12920_body124_e17943, assign12920_body124_e17943_d_n0, assign12920_body124_e17943_d_n2, assign12920_body124_e17943_d_n6, assign12920_body124_e17943_d_n7, assign12920_body124_e17943_d_n10, assign12920_body124_e17943_d_n11, assign12920_body124_e17943_d_n12, assign12920_body124_e17943_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) {
        let assign12920_body124_e17941: f64 = (locals.var_c_soi_inv__blk115 * locals.var_q_sl_bulk_dpsb);
        (assign12920_body124_e17941, (locals.var_c_soi_inv__blk115 * locals.var_q_sl_bulk_dpsb_dn0), (locals.var_c_soi_inv__blk115 * locals.var_q_sl_bulk_dpsb_dn2), (locals.var_c_soi_inv__blk115 * locals.var_q_sl_bulk_dpsb_dn6), (locals.var_c_soi_inv__blk115 * locals.var_q_sl_bulk_dpsb_dn7), (locals.var_c_soi_inv__blk115 * locals.var_q_sl_bulk_dpsb_dn10), (locals.var_c_soi_inv__blk115 * locals.var_q_sl_bulk_dpsb_dn11), (locals.var_c_soi_inv__blk115 * locals.var_q_sl_bulk_dpsb_dn12), (locals.var_c_soi_inv__blk115 * locals.var_q_sl_bulk_dpsb_dn17),)
    } else {
        (locals.var_t3__blk355, locals.var_t3__blk355_dn0, locals.var_t3__blk355_dn2, locals.var_t3__blk355_dn6, locals.var_t3__blk355_dn7, locals.var_t3__blk355_dn10, locals.var_t3__blk355_dn11, locals.var_t3__blk355_dn12, locals.var_t3__blk355_dn17,)
    }
};
            locals.var_t3__blk355 = assign12920_body124_e17943;
            locals.var_t3__blk355_dn0 = assign12920_body124_e17943_d_n0;
            locals.var_t3__blk355_dn2 = assign12920_body124_e17943_d_n2;
            locals.var_t3__blk355_dn6 = assign12920_body124_e17943_d_n6;
            locals.var_t3__blk355_dn7 = assign12920_body124_e17943_d_n7;
            locals.var_t3__blk355_dn10 = assign12920_body124_e17943_d_n10;
            locals.var_t3__blk355_dn11 = assign12920_body124_e17943_d_n11;
            locals.var_t3__blk355_dn12 = assign12920_body124_e17943_d_n12;
            locals.var_t3__blk355_dn17 = assign12920_body124_e17943_d_n17;
            let (assign12920_body125_e17955, assign12920_body125_e17955_d_n0, assign12920_body125_e17955_d_n2, assign12920_body125_e17955_d_n6, assign12920_body125_e17955_d_n7, assign12920_body125_e17955_d_n10, assign12920_body125_e17955_d_n11, assign12920_body125_e17955_d_n12, assign12920_body125_e17955_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) {
        let assign12920_body125_e17953: f64 = (locals.var_phi_bl_soi - locals.var_t1__blk353);
        (assign12920_body125_e17953, (locals.var_phi_bl_soi_dn0 - locals.var_t1__blk353_dn0), (locals.var_phi_bl_soi_dn2 - locals.var_t1__blk353_dn2), (locals.var_phi_bl_soi_dn6 - locals.var_t1__blk353_dn6), (locals.var_phi_bl_soi_dn7 - locals.var_t1__blk353_dn7), (locals.var_phi_bl_soi_dn10 - locals.var_t1__blk353_dn10), (locals.var_phi_bl_soi_dn11 - locals.var_t1__blk353_dn11), (locals.var_phi_bl_soi_dn12 - locals.var_t1__blk353_dn12), (locals.var_phi_bl_soi_dn17 - locals.var_t1__blk353_dn17),)
    } else {
        (locals.var_pf2__blk367, locals.var_pf2__blk367_dn0, locals.var_pf2__blk367_dn2, locals.var_pf2__blk367_dn6, locals.var_pf2__blk367_dn7, locals.var_pf2__blk367_dn10, locals.var_pf2__blk367_dn11, locals.var_pf2__blk367_dn12, locals.var_pf2__blk367_dn17,)
    }
};
            locals.var_pf2__blk367 = assign12920_body125_e17955;
            locals.var_pf2__blk367_dn0 = assign12920_body125_e17955_d_n0;
            locals.var_pf2__blk367_dn2 = assign12920_body125_e17955_d_n2;
            locals.var_pf2__blk367_dn6 = assign12920_body125_e17955_d_n6;
            locals.var_pf2__blk367_dn7 = assign12920_body125_e17955_d_n7;
            locals.var_pf2__blk367_dn10 = assign12920_body125_e17955_d_n10;
            locals.var_pf2__blk367_dn11 = assign12920_body125_e17955_d_n11;
            locals.var_pf2__blk367_dn12 = assign12920_body125_e17955_d_n12;
            locals.var_pf2__blk367_dn17 = assign12920_body125_e17955_d_n17;
            let (assign12920_body126_e17966,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) {
        let assign12920_body126_e17964: f64 = (-1.0);
        (assign12920_body126_e17964,)
    } else {
        (locals.var_pf21__blk368,)
    }
};
            locals.var_pf21__blk368 = assign12920_body126_e17966;
            let (assign12920_body127_e17976,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_pf22__blk369,)
    }
};
            locals.var_pf22__blk369 = assign12920_body127_e17976;
            let (assign12920_body128_e17987, assign12920_body128_e17987_d_n0, assign12920_body128_e17987_d_n2, assign12920_body128_e17987_d_n6, assign12920_body128_e17987_d_n7, assign12920_body128_e17987_d_n10, assign12920_body128_e17987_d_n11, assign12920_body128_e17987_d_n12, assign12920_body128_e17987_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) {
        let assign12920_body128_e17985: f64 = (-locals.var_t3__blk355);
        (assign12920_body128_e17985, (-locals.var_t3__blk355_dn0), (-locals.var_t3__blk355_dn2), (-locals.var_t3__blk355_dn6), (-locals.var_t3__blk355_dn7), (-locals.var_t3__blk355_dn10), (-locals.var_t3__blk355_dn11), (-locals.var_t3__blk355_dn12), (-locals.var_t3__blk355_dn17),)
    } else {
        (locals.var_pf23__blk370, locals.var_pf23__blk370_dn0, locals.var_pf23__blk370_dn2, locals.var_pf23__blk370_dn6, locals.var_pf23__blk370_dn7, locals.var_pf23__blk370_dn10, locals.var_pf23__blk370_dn11, locals.var_pf23__blk370_dn12, locals.var_pf23__blk370_dn17,)
    }
};
            locals.var_pf23__blk370 = assign12920_body128_e17987;
            locals.var_pf23__blk370_dn0 = assign12920_body128_e17987_d_n0;
            locals.var_pf23__blk370_dn2 = assign12920_body128_e17987_d_n2;
            locals.var_pf23__blk370_dn6 = assign12920_body128_e17987_d_n6;
            locals.var_pf23__blk370_dn7 = assign12920_body128_e17987_d_n7;
            locals.var_pf23__blk370_dn10 = assign12920_body128_e17987_d_n10;
            locals.var_pf23__blk370_dn11 = assign12920_body128_e17987_d_n11;
            locals.var_pf23__blk370_dn12 = assign12920_body128_e17987_d_n12;
            locals.var_pf23__blk370_dn17 = assign12920_body128_e17987_d_n17;
            let (assign12920_body129_e18003, assign12920_body129_e18003_d_n0, assign12920_body129_e18003_d_n2, assign12920_body129_e18003_d_n6, assign12920_body129_e18003_d_n7, assign12920_body129_e18003_d_n10, assign12920_body129_e18003_d_n11, assign12920_body129_e18003_d_n12, assign12920_body129_e18003_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) {
        let assign12920_body129_e17997: f64 = (locals.var_phi_sl_bulk - locals.var_phi_bl_soi);
        let assign12920_body129_e18000: f64 = (locals.var_c_box_inv * locals.var_q_sl_bulk);
        let assign12920_body129_e18001: f64 = (assign12920_body129_e17997 - assign12920_body129_e18000);
        (assign12920_body129_e18001, ((locals.var_phi_sl_bulk_dn0 - locals.var_phi_bl_soi_dn0) - (locals.var_c_box_inv * locals.var_q_sl_bulk_dn0)), ((locals.var_phi_sl_bulk_dn2 - locals.var_phi_bl_soi_dn2) - (locals.var_c_box_inv * locals.var_q_sl_bulk_dn2)), ((locals.var_phi_sl_bulk_dn6 - locals.var_phi_bl_soi_dn6) - (locals.var_c_box_inv * locals.var_q_sl_bulk_dn6)), ((locals.var_phi_sl_bulk_dn7 - locals.var_phi_bl_soi_dn7) - (locals.var_c_box_inv * locals.var_q_sl_bulk_dn7)), ((locals.var_phi_sl_bulk_dn10 - locals.var_phi_bl_soi_dn10) - (locals.var_c_box_inv * locals.var_q_sl_bulk_dn10)), ((locals.var_phi_sl_bulk_dn11 - locals.var_phi_bl_soi_dn11) - (locals.var_c_box_inv * locals.var_q_sl_bulk_dn11)), ((locals.var_phi_sl_bulk_dn12 - locals.var_phi_bl_soi_dn12) - (locals.var_c_box_inv * locals.var_q_sl_bulk_dn12)), ((locals.var_phi_sl_bulk_dn17 - locals.var_phi_bl_soi_dn17) - (locals.var_c_box_inv * locals.var_q_sl_bulk_dn17)),)
    } else {
        (locals.var_pf3__blk371, locals.var_pf3__blk371_dn0, locals.var_pf3__blk371_dn2, locals.var_pf3__blk371_dn6, locals.var_pf3__blk371_dn7, locals.var_pf3__blk371_dn10, locals.var_pf3__blk371_dn11, locals.var_pf3__blk371_dn12, locals.var_pf3__blk371_dn17,)
    }
};
            locals.var_pf3__blk371 = assign12920_body129_e18003;
            locals.var_pf3__blk371_dn0 = assign12920_body129_e18003_d_n0;
            locals.var_pf3__blk371_dn2 = assign12920_body129_e18003_d_n2;
            locals.var_pf3__blk371_dn6 = assign12920_body129_e18003_d_n6;
            locals.var_pf3__blk371_dn7 = assign12920_body129_e18003_d_n7;
            locals.var_pf3__blk371_dn10 = assign12920_body129_e18003_d_n10;
            locals.var_pf3__blk371_dn11 = assign12920_body129_e18003_d_n11;
            locals.var_pf3__blk371_dn12 = assign12920_body129_e18003_d_n12;
            locals.var_pf3__blk371_dn17 = assign12920_body129_e18003_d_n17;
            let (assign12920_body130_e18014,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) {
        let assign12920_body130_e18012: f64 = (-1.0);
        (assign12920_body130_e18012,)
    } else {
        (locals.var_pf32__blk372,)
    }
};
            locals.var_pf32__blk372 = assign12920_body130_e18014;
            let (assign12920_body131_e18028, assign12920_body131_e18028_d_n0, assign12920_body131_e18028_d_n2, assign12920_body131_e18028_d_n6, assign12920_body131_e18028_d_n7, assign12920_body131_e18028_d_n10, assign12920_body131_e18028_d_n11, assign12920_body131_e18028_d_n12, assign12920_body131_e18028_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) {
        let assign12920_body131_e18025: f64 = (locals.var_c_box_inv * locals.var_q_sl_bulk_dpsb);
        let assign12920_body131_e18026: f64 = (1.0 - assign12920_body131_e18025);
        (assign12920_body131_e18026, (-(locals.var_c_box_inv * locals.var_q_sl_bulk_dpsb_dn0)), (-(locals.var_c_box_inv * locals.var_q_sl_bulk_dpsb_dn2)), (-(locals.var_c_box_inv * locals.var_q_sl_bulk_dpsb_dn6)), (-(locals.var_c_box_inv * locals.var_q_sl_bulk_dpsb_dn7)), (-(locals.var_c_box_inv * locals.var_q_sl_bulk_dpsb_dn10)), (-(locals.var_c_box_inv * locals.var_q_sl_bulk_dpsb_dn11)), (-(locals.var_c_box_inv * locals.var_q_sl_bulk_dpsb_dn12)), (-(locals.var_c_box_inv * locals.var_q_sl_bulk_dpsb_dn17)),)
    } else {
        (locals.var_pf33__blk373, locals.var_pf33__blk373_dn0, locals.var_pf33__blk373_dn2, locals.var_pf33__blk373_dn6, locals.var_pf33__blk373_dn7, locals.var_pf33__blk373_dn10, locals.var_pf33__blk373_dn11, locals.var_pf33__blk373_dn12, locals.var_pf33__blk373_dn17,)
    }
};
            locals.var_pf33__blk373 = assign12920_body131_e18028;
            locals.var_pf33__blk373_dn0 = assign12920_body131_e18028_d_n0;
            locals.var_pf33__blk373_dn2 = assign12920_body131_e18028_d_n2;
            locals.var_pf33__blk373_dn6 = assign12920_body131_e18028_d_n6;
            locals.var_pf33__blk373_dn7 = assign12920_body131_e18028_d_n7;
            locals.var_pf33__blk373_dn10 = assign12920_body131_e18028_d_n10;
            locals.var_pf33__blk373_dn11 = assign12920_body131_e18028_d_n11;
            locals.var_pf33__blk373_dn12 = assign12920_body131_e18028_d_n12;
            locals.var_pf33__blk373_dn17 = assign12920_body131_e18028_d_n17;
            let (assign12920_body132_e18060, assign12920_body132_e18060_d_n0, assign12920_body132_e18060_d_n2, assign12920_body132_e18060_d_n6, assign12920_body132_e18060_d_n7, assign12920_body132_e18060_d_n10, assign12920_body132_e18060_d_n11, assign12920_body132_e18060_d_n12, assign12920_body132_e18060_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) {
        let assign12920_body132_e18038: f64 = (locals.var_pf11__blk364 * locals.var_pf22__blk369);
        let assign12920_body132_e18040: f64 = (assign12920_body132_e18038 * locals.var_pf33__blk373);
        let assign12920_body132_e18043: f64 = (locals.var_pf11__blk364 * locals.var_pf23__blk370);
        let assign12920_body132_e18045: f64 = (assign12920_body132_e18043 * locals.var_pf32__blk372);
        let assign12920_body132_e18046: f64 = (assign12920_body132_e18040 - assign12920_body132_e18045);
        let assign12920_body132_e18049: f64 = (locals.var_pf12__blk365 * locals.var_pf21__blk368);
        let assign12920_body132_e18051: f64 = (assign12920_body132_e18049 * locals.var_pf33__blk373);
        let assign12920_body132_e18052: f64 = (assign12920_body132_e18046 - assign12920_body132_e18051);
        let assign12920_body132_e18055: f64 = (locals.var_pf13__blk366 * locals.var_pf21__blk368);
        let assign12920_body132_e18057: f64 = (assign12920_body132_e18055 * locals.var_pf32__blk372);
        let assign12920_body132_e18058: f64 = (assign12920_body132_e18052 + assign12920_body132_e18057);
        (assign12920_body132_e18058, ((((((locals.var_pf11__blk364_dn0 * locals.var_pf22__blk369) * locals.var_pf33__blk373) + (assign12920_body132_e18038 * locals.var_pf33__blk373_dn0)) - (((locals.var_pf11__blk364_dn0 * locals.var_pf23__blk370) + (locals.var_pf11__blk364 * locals.var_pf23__blk370_dn0)) * locals.var_pf32__blk372)) - (((locals.var_pf12__blk365_dn0 * locals.var_pf21__blk368) * locals.var_pf33__blk373) + (assign12920_body132_e18049 * locals.var_pf33__blk373_dn0))) + ((locals.var_pf13__blk366_dn0 * locals.var_pf21__blk368) * locals.var_pf32__blk372)), ((((((locals.var_pf11__blk364_dn2 * locals.var_pf22__blk369) * locals.var_pf33__blk373) + (assign12920_body132_e18038 * locals.var_pf33__blk373_dn2)) - (((locals.var_pf11__blk364_dn2 * locals.var_pf23__blk370) + (locals.var_pf11__blk364 * locals.var_pf23__blk370_dn2)) * locals.var_pf32__blk372)) - (((locals.var_pf12__blk365_dn2 * locals.var_pf21__blk368) * locals.var_pf33__blk373) + (assign12920_body132_e18049 * locals.var_pf33__blk373_dn2))) + ((locals.var_pf13__blk366_dn2 * locals.var_pf21__blk368) * locals.var_pf32__blk372)), ((((((locals.var_pf11__blk364_dn6 * locals.var_pf22__blk369) * locals.var_pf33__blk373) + (assign12920_body132_e18038 * locals.var_pf33__blk373_dn6)) - (((locals.var_pf11__blk364_dn6 * locals.var_pf23__blk370) + (locals.var_pf11__blk364 * locals.var_pf23__blk370_dn6)) * locals.var_pf32__blk372)) - (((locals.var_pf12__blk365_dn6 * locals.var_pf21__blk368) * locals.var_pf33__blk373) + (assign12920_body132_e18049 * locals.var_pf33__blk373_dn6))) + ((locals.var_pf13__blk366_dn6 * locals.var_pf21__blk368) * locals.var_pf32__blk372)), ((((((locals.var_pf11__blk364_dn7 * locals.var_pf22__blk369) * locals.var_pf33__blk373) + (assign12920_body132_e18038 * locals.var_pf33__blk373_dn7)) - (((locals.var_pf11__blk364_dn7 * locals.var_pf23__blk370) + (locals.var_pf11__blk364 * locals.var_pf23__blk370_dn7)) * locals.var_pf32__blk372)) - (((locals.var_pf12__blk365_dn7 * locals.var_pf21__blk368) * locals.var_pf33__blk373) + (assign12920_body132_e18049 * locals.var_pf33__blk373_dn7))) + ((locals.var_pf13__blk366_dn7 * locals.var_pf21__blk368) * locals.var_pf32__blk372)), ((((((locals.var_pf11__blk364_dn10 * locals.var_pf22__blk369) * locals.var_pf33__blk373) + (assign12920_body132_e18038 * locals.var_pf33__blk373_dn10)) - (((locals.var_pf11__blk364_dn10 * locals.var_pf23__blk370) + (locals.var_pf11__blk364 * locals.var_pf23__blk370_dn10)) * locals.var_pf32__blk372)) - (((locals.var_pf12__blk365_dn10 * locals.var_pf21__blk368) * locals.var_pf33__blk373) + (assign12920_body132_e18049 * locals.var_pf33__blk373_dn10))) + ((locals.var_pf13__blk366_dn10 * locals.var_pf21__blk368) * locals.var_pf32__blk372)), ((((((locals.var_pf11__blk364_dn11 * locals.var_pf22__blk369) * locals.var_pf33__blk373) + (assign12920_body132_e18038 * locals.var_pf33__blk373_dn11)) - (((locals.var_pf11__blk364_dn11 * locals.var_pf23__blk370) + (locals.var_pf11__blk364 * locals.var_pf23__blk370_dn11)) * locals.var_pf32__blk372)) - (((locals.var_pf12__blk365_dn11 * locals.var_pf21__blk368) * locals.var_pf33__blk373) + (assign12920_body132_e18049 * locals.var_pf33__blk373_dn11))) + ((locals.var_pf13__blk366_dn11 * locals.var_pf21__blk368) * locals.var_pf32__blk372)), ((((((locals.var_pf11__blk364_dn12 * locals.var_pf22__blk369) * locals.var_pf33__blk373) + (assign12920_body132_e18038 * locals.var_pf33__blk373_dn12)) - (((locals.var_pf11__blk364_dn12 * locals.var_pf23__blk370) + (locals.var_pf11__blk364 * locals.var_pf23__blk370_dn12)) * locals.var_pf32__blk372)) - (((locals.var_pf12__blk365_dn12 * locals.var_pf21__blk368) * locals.var_pf33__blk373) + (assign12920_body132_e18049 * locals.var_pf33__blk373_dn12))) + ((locals.var_pf13__blk366_dn12 * locals.var_pf21__blk368) * locals.var_pf32__blk372)), ((((((locals.var_pf11__blk364_dn17 * locals.var_pf22__blk369) * locals.var_pf33__blk373) + (assign12920_body132_e18038 * locals.var_pf33__blk373_dn17)) - (((locals.var_pf11__blk364_dn17 * locals.var_pf23__blk370) + (locals.var_pf11__blk364 * locals.var_pf23__blk370_dn17)) * locals.var_pf32__blk372)) - (((locals.var_pf12__blk365_dn17 * locals.var_pf21__blk368) * locals.var_pf33__blk373) + (assign12920_body132_e18049 * locals.var_pf33__blk373_dn17))) + ((locals.var_pf13__blk366_dn17 * locals.var_pf21__blk368) * locals.var_pf32__blk372)),)
    } else {
        (locals.var_pdj__blk374, locals.var_pdj__blk374_dn0, locals.var_pdj__blk374_dn2, locals.var_pdj__blk374_dn6, locals.var_pdj__blk374_dn7, locals.var_pdj__blk374_dn10, locals.var_pdj__blk374_dn11, locals.var_pdj__blk374_dn12, locals.var_pdj__blk374_dn17,)
    }
};
            locals.var_pdj__blk374 = assign12920_body132_e18060;
            locals.var_pdj__blk374_dn0 = assign12920_body132_e18060_d_n0;
            locals.var_pdj__blk374_dn2 = assign12920_body132_e18060_d_n2;
            locals.var_pdj__blk374_dn6 = assign12920_body132_e18060_d_n6;
            locals.var_pdj__blk374_dn7 = assign12920_body132_e18060_d_n7;
            locals.var_pdj__blk374_dn10 = assign12920_body132_e18060_d_n10;
            locals.var_pdj__blk374_dn11 = assign12920_body132_e18060_d_n11;
            locals.var_pdj__blk374_dn12 = assign12920_body132_e18060_d_n12;
            locals.var_pdj__blk374_dn17 = assign12920_body132_e18060_d_n17;
            let (assign12920_body133_e18074, assign12920_body133_e18074_d_n0, assign12920_body133_e18074_d_n2, assign12920_body133_e18074_d_n6, assign12920_body133_e18074_d_n7, assign12920_body133_e18074_d_n10, assign12920_body133_e18074_d_n11, assign12920_body133_e18074_d_n12, assign12920_body133_e18074_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) {
        let assign12920_body133_e18071: f64 = (locals.var_pdj__blk374 + 1e-50);
        let assign12920_body133_e18072: f64 = (1.0 / assign12920_body133_e18071);
        (assign12920_body133_e18072, (-(locals.var_pdj__blk374_dn0 / (assign12920_body133_e18071 * assign12920_body133_e18071))), (-(locals.var_pdj__blk374_dn2 / (assign12920_body133_e18071 * assign12920_body133_e18071))), (-(locals.var_pdj__blk374_dn6 / (assign12920_body133_e18071 * assign12920_body133_e18071))), (-(locals.var_pdj__blk374_dn7 / (assign12920_body133_e18071 * assign12920_body133_e18071))), (-(locals.var_pdj__blk374_dn10 / (assign12920_body133_e18071 * assign12920_body133_e18071))), (-(locals.var_pdj__blk374_dn11 / (assign12920_body133_e18071 * assign12920_body133_e18071))), (-(locals.var_pdj__blk374_dn12 / (assign12920_body133_e18071 * assign12920_body133_e18071))), (-(locals.var_pdj__blk374_dn17 / (assign12920_body133_e18071 * assign12920_body133_e18071))),)
    } else {
        (locals.var_pdji__blk375, locals.var_pdji__blk375_dn0, locals.var_pdji__blk375_dn2, locals.var_pdji__blk375_dn6, locals.var_pdji__blk375_dn7, locals.var_pdji__blk375_dn10, locals.var_pdji__blk375_dn11, locals.var_pdji__blk375_dn12, locals.var_pdji__blk375_dn17,)
    }
};
            locals.var_pdji__blk375 = assign12920_body133_e18074;
            locals.var_pdji__blk375_dn0 = assign12920_body133_e18074_d_n0;
            locals.var_pdji__blk375_dn2 = assign12920_body133_e18074_d_n2;
            locals.var_pdji__blk375_dn6 = assign12920_body133_e18074_d_n6;
            locals.var_pdji__blk375_dn7 = assign12920_body133_e18074_d_n7;
            locals.var_pdji__blk375_dn10 = assign12920_body133_e18074_d_n10;
            locals.var_pdji__blk375_dn11 = assign12920_body133_e18074_d_n11;
            locals.var_pdji__blk375_dn12 = assign12920_body133_e18074_d_n12;
            locals.var_pdji__blk375_dn17 = assign12920_body133_e18074_d_n17;
            let (assign12920_body134_e18090, assign12920_body134_e18090_d_n0, assign12920_body134_e18090_d_n2, assign12920_body134_e18090_d_n6, assign12920_body134_e18090_d_n7, assign12920_body134_e18090_d_n10, assign12920_body134_e18090_d_n11, assign12920_body134_e18090_d_n12, assign12920_body134_e18090_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) {
        let assign12920_body134_e18084: f64 = (locals.var_pf22__blk369 * locals.var_pf33__blk373);
        let assign12920_body134_e18087: f64 = (locals.var_pf23__blk370 * locals.var_pf32__blk372);
        let assign12920_body134_e18088: f64 = (assign12920_body134_e18084 - assign12920_body134_e18087);
        (assign12920_body134_e18088, ((locals.var_pf22__blk369 * locals.var_pf33__blk373_dn0) - (locals.var_pf23__blk370_dn0 * locals.var_pf32__blk372)), ((locals.var_pf22__blk369 * locals.var_pf33__blk373_dn2) - (locals.var_pf23__blk370_dn2 * locals.var_pf32__blk372)), ((locals.var_pf22__blk369 * locals.var_pf33__blk373_dn6) - (locals.var_pf23__blk370_dn6 * locals.var_pf32__blk372)), ((locals.var_pf22__blk369 * locals.var_pf33__blk373_dn7) - (locals.var_pf23__blk370_dn7 * locals.var_pf32__blk372)), ((locals.var_pf22__blk369 * locals.var_pf33__blk373_dn10) - (locals.var_pf23__blk370_dn10 * locals.var_pf32__blk372)), ((locals.var_pf22__blk369 * locals.var_pf33__blk373_dn11) - (locals.var_pf23__blk370_dn11 * locals.var_pf32__blk372)), ((locals.var_pf22__blk369 * locals.var_pf33__blk373_dn12) - (locals.var_pf23__blk370_dn12 * locals.var_pf32__blk372)), ((locals.var_pf22__blk369 * locals.var_pf33__blk373_dn17) - (locals.var_pf23__blk370_dn17 * locals.var_pf32__blk372)),)
    } else {
        (locals.var_pji11__blk376, locals.var_pji11__blk376_dn0, locals.var_pji11__blk376_dn2, locals.var_pji11__blk376_dn6, locals.var_pji11__blk376_dn7, locals.var_pji11__blk376_dn10, locals.var_pji11__blk376_dn11, locals.var_pji11__blk376_dn12, locals.var_pji11__blk376_dn17,)
    }
};
            locals.var_pji11__blk376 = assign12920_body134_e18090;
            locals.var_pji11__blk376_dn0 = assign12920_body134_e18090_d_n0;
            locals.var_pji11__blk376_dn2 = assign12920_body134_e18090_d_n2;
            locals.var_pji11__blk376_dn6 = assign12920_body134_e18090_d_n6;
            locals.var_pji11__blk376_dn7 = assign12920_body134_e18090_d_n7;
            locals.var_pji11__blk376_dn10 = assign12920_body134_e18090_d_n10;
            locals.var_pji11__blk376_dn11 = assign12920_body134_e18090_d_n11;
            locals.var_pji11__blk376_dn12 = assign12920_body134_e18090_d_n12;
            locals.var_pji11__blk376_dn17 = assign12920_body134_e18090_d_n17;
            let (assign12920_body135_e18106, assign12920_body135_e18106_d_n0, assign12920_body135_e18106_d_n2, assign12920_body135_e18106_d_n6, assign12920_body135_e18106_d_n7, assign12920_body135_e18106_d_n10, assign12920_body135_e18106_d_n11, assign12920_body135_e18106_d_n12, assign12920_body135_e18106_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) {
        let assign12920_body135_e18100: f64 = (locals.var_pf13__blk366 * locals.var_pf32__blk372);
        let assign12920_body135_e18103: f64 = (locals.var_pf12__blk365 * locals.var_pf33__blk373);
        let assign12920_body135_e18104: f64 = (assign12920_body135_e18100 - assign12920_body135_e18103);
        (assign12920_body135_e18104, ((locals.var_pf13__blk366_dn0 * locals.var_pf32__blk372) - ((locals.var_pf12__blk365_dn0 * locals.var_pf33__blk373) + (locals.var_pf12__blk365 * locals.var_pf33__blk373_dn0))), ((locals.var_pf13__blk366_dn2 * locals.var_pf32__blk372) - ((locals.var_pf12__blk365_dn2 * locals.var_pf33__blk373) + (locals.var_pf12__blk365 * locals.var_pf33__blk373_dn2))), ((locals.var_pf13__blk366_dn6 * locals.var_pf32__blk372) - ((locals.var_pf12__blk365_dn6 * locals.var_pf33__blk373) + (locals.var_pf12__blk365 * locals.var_pf33__blk373_dn6))), ((locals.var_pf13__blk366_dn7 * locals.var_pf32__blk372) - ((locals.var_pf12__blk365_dn7 * locals.var_pf33__blk373) + (locals.var_pf12__blk365 * locals.var_pf33__blk373_dn7))), ((locals.var_pf13__blk366_dn10 * locals.var_pf32__blk372) - ((locals.var_pf12__blk365_dn10 * locals.var_pf33__blk373) + (locals.var_pf12__blk365 * locals.var_pf33__blk373_dn10))), ((locals.var_pf13__blk366_dn11 * locals.var_pf32__blk372) - ((locals.var_pf12__blk365_dn11 * locals.var_pf33__blk373) + (locals.var_pf12__blk365 * locals.var_pf33__blk373_dn11))), ((locals.var_pf13__blk366_dn12 * locals.var_pf32__blk372) - ((locals.var_pf12__blk365_dn12 * locals.var_pf33__blk373) + (locals.var_pf12__blk365 * locals.var_pf33__blk373_dn12))), ((locals.var_pf13__blk366_dn17 * locals.var_pf32__blk372) - ((locals.var_pf12__blk365_dn17 * locals.var_pf33__blk373) + (locals.var_pf12__blk365 * locals.var_pf33__blk373_dn17))),)
    } else {
        (locals.var_pji12__blk377, locals.var_pji12__blk377_dn0, locals.var_pji12__blk377_dn2, locals.var_pji12__blk377_dn6, locals.var_pji12__blk377_dn7, locals.var_pji12__blk377_dn10, locals.var_pji12__blk377_dn11, locals.var_pji12__blk377_dn12, locals.var_pji12__blk377_dn17,)
    }
};
            locals.var_pji12__blk377 = assign12920_body135_e18106;
            locals.var_pji12__blk377_dn0 = assign12920_body135_e18106_d_n0;
            locals.var_pji12__blk377_dn2 = assign12920_body135_e18106_d_n2;
            locals.var_pji12__blk377_dn6 = assign12920_body135_e18106_d_n6;
            locals.var_pji12__blk377_dn7 = assign12920_body135_e18106_d_n7;
            locals.var_pji12__blk377_dn10 = assign12920_body135_e18106_d_n10;
            locals.var_pji12__blk377_dn11 = assign12920_body135_e18106_d_n11;
            locals.var_pji12__blk377_dn12 = assign12920_body135_e18106_d_n12;
            locals.var_pji12__blk377_dn17 = assign12920_body135_e18106_d_n17;
            let (assign12920_body136_e18122, assign12920_body136_e18122_d_n0, assign12920_body136_e18122_d_n2, assign12920_body136_e18122_d_n6, assign12920_body136_e18122_d_n7, assign12920_body136_e18122_d_n10, assign12920_body136_e18122_d_n11, assign12920_body136_e18122_d_n12, assign12920_body136_e18122_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) {
        let assign12920_body136_e18116: f64 = (locals.var_pf12__blk365 * locals.var_pf23__blk370);
        let assign12920_body136_e18119: f64 = (locals.var_pf13__blk366 * locals.var_pf22__blk369);
        let assign12920_body136_e18120: f64 = (assign12920_body136_e18116 - assign12920_body136_e18119);
        (assign12920_body136_e18120, (((locals.var_pf12__blk365_dn0 * locals.var_pf23__blk370) + (locals.var_pf12__blk365 * locals.var_pf23__blk370_dn0)) - (locals.var_pf13__blk366_dn0 * locals.var_pf22__blk369)), (((locals.var_pf12__blk365_dn2 * locals.var_pf23__blk370) + (locals.var_pf12__blk365 * locals.var_pf23__blk370_dn2)) - (locals.var_pf13__blk366_dn2 * locals.var_pf22__blk369)), (((locals.var_pf12__blk365_dn6 * locals.var_pf23__blk370) + (locals.var_pf12__blk365 * locals.var_pf23__blk370_dn6)) - (locals.var_pf13__blk366_dn6 * locals.var_pf22__blk369)), (((locals.var_pf12__blk365_dn7 * locals.var_pf23__blk370) + (locals.var_pf12__blk365 * locals.var_pf23__blk370_dn7)) - (locals.var_pf13__blk366_dn7 * locals.var_pf22__blk369)), (((locals.var_pf12__blk365_dn10 * locals.var_pf23__blk370) + (locals.var_pf12__blk365 * locals.var_pf23__blk370_dn10)) - (locals.var_pf13__blk366_dn10 * locals.var_pf22__blk369)), (((locals.var_pf12__blk365_dn11 * locals.var_pf23__blk370) + (locals.var_pf12__blk365 * locals.var_pf23__blk370_dn11)) - (locals.var_pf13__blk366_dn11 * locals.var_pf22__blk369)), (((locals.var_pf12__blk365_dn12 * locals.var_pf23__blk370) + (locals.var_pf12__blk365 * locals.var_pf23__blk370_dn12)) - (locals.var_pf13__blk366_dn12 * locals.var_pf22__blk369)), (((locals.var_pf12__blk365_dn17 * locals.var_pf23__blk370) + (locals.var_pf12__blk365 * locals.var_pf23__blk370_dn17)) - (locals.var_pf13__blk366_dn17 * locals.var_pf22__blk369)),)
    } else {
        (locals.var_pji13__blk378, locals.var_pji13__blk378_dn0, locals.var_pji13__blk378_dn2, locals.var_pji13__blk378_dn6, locals.var_pji13__blk378_dn7, locals.var_pji13__blk378_dn10, locals.var_pji13__blk378_dn11, locals.var_pji13__blk378_dn12, locals.var_pji13__blk378_dn17,)
    }
};
            locals.var_pji13__blk378 = assign12920_body136_e18122;
            locals.var_pji13__blk378_dn0 = assign12920_body136_e18122_d_n0;
            locals.var_pji13__blk378_dn2 = assign12920_body136_e18122_d_n2;
            locals.var_pji13__blk378_dn6 = assign12920_body136_e18122_d_n6;
            locals.var_pji13__blk378_dn7 = assign12920_body136_e18122_d_n7;
            locals.var_pji13__blk378_dn10 = assign12920_body136_e18122_d_n10;
            locals.var_pji13__blk378_dn11 = assign12920_body136_e18122_d_n11;
            locals.var_pji13__blk378_dn12 = assign12920_body136_e18122_d_n12;
            locals.var_pji13__blk378_dn17 = assign12920_body136_e18122_d_n17;
            let (assign12920_body137_e18135, assign12920_body137_e18135_d_n0, assign12920_body137_e18135_d_n2, assign12920_body137_e18135_d_n6, assign12920_body137_e18135_d_n7, assign12920_body137_e18135_d_n10, assign12920_body137_e18135_d_n11, assign12920_body137_e18135_d_n12, assign12920_body137_e18135_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) {
        let assign12920_body137_e18131: f64 = (-locals.var_pf21__blk368);
        let assign12920_body137_e18133: f64 = (assign12920_body137_e18131 * locals.var_pf33__blk373);
        (assign12920_body137_e18133, (assign12920_body137_e18131 * locals.var_pf33__blk373_dn0), (assign12920_body137_e18131 * locals.var_pf33__blk373_dn2), (assign12920_body137_e18131 * locals.var_pf33__blk373_dn6), (assign12920_body137_e18131 * locals.var_pf33__blk373_dn7), (assign12920_body137_e18131 * locals.var_pf33__blk373_dn10), (assign12920_body137_e18131 * locals.var_pf33__blk373_dn11), (assign12920_body137_e18131 * locals.var_pf33__blk373_dn12), (assign12920_body137_e18131 * locals.var_pf33__blk373_dn17),)
    } else {
        (locals.var_pji21__blk379, locals.var_pji21__blk379_dn0, locals.var_pji21__blk379_dn2, locals.var_pji21__blk379_dn6, locals.var_pji21__blk379_dn7, locals.var_pji21__blk379_dn10, locals.var_pji21__blk379_dn11, locals.var_pji21__blk379_dn12, locals.var_pji21__blk379_dn17,)
    }
};
            locals.var_pji21__blk379 = assign12920_body137_e18135;
            locals.var_pji21__blk379_dn0 = assign12920_body137_e18135_d_n0;
            locals.var_pji21__blk379_dn2 = assign12920_body137_e18135_d_n2;
            locals.var_pji21__blk379_dn6 = assign12920_body137_e18135_d_n6;
            locals.var_pji21__blk379_dn7 = assign12920_body137_e18135_d_n7;
            locals.var_pji21__blk379_dn10 = assign12920_body137_e18135_d_n10;
            locals.var_pji21__blk379_dn11 = assign12920_body137_e18135_d_n11;
            locals.var_pji21__blk379_dn12 = assign12920_body137_e18135_d_n12;
            locals.var_pji21__blk379_dn17 = assign12920_body137_e18135_d_n17;
            let (assign12920_body138_e18147, assign12920_body138_e18147_d_n0, assign12920_body138_e18147_d_n2, assign12920_body138_e18147_d_n6, assign12920_body138_e18147_d_n7, assign12920_body138_e18147_d_n10, assign12920_body138_e18147_d_n11, assign12920_body138_e18147_d_n12, assign12920_body138_e18147_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) {
        let assign12920_body138_e18145: f64 = (locals.var_pf11__blk364 * locals.var_pf33__blk373);
        (assign12920_body138_e18145, ((locals.var_pf11__blk364_dn0 * locals.var_pf33__blk373) + (locals.var_pf11__blk364 * locals.var_pf33__blk373_dn0)), ((locals.var_pf11__blk364_dn2 * locals.var_pf33__blk373) + (locals.var_pf11__blk364 * locals.var_pf33__blk373_dn2)), ((locals.var_pf11__blk364_dn6 * locals.var_pf33__blk373) + (locals.var_pf11__blk364 * locals.var_pf33__blk373_dn6)), ((locals.var_pf11__blk364_dn7 * locals.var_pf33__blk373) + (locals.var_pf11__blk364 * locals.var_pf33__blk373_dn7)), ((locals.var_pf11__blk364_dn10 * locals.var_pf33__blk373) + (locals.var_pf11__blk364 * locals.var_pf33__blk373_dn10)), ((locals.var_pf11__blk364_dn11 * locals.var_pf33__blk373) + (locals.var_pf11__blk364 * locals.var_pf33__blk373_dn11)), ((locals.var_pf11__blk364_dn12 * locals.var_pf33__blk373) + (locals.var_pf11__blk364 * locals.var_pf33__blk373_dn12)), ((locals.var_pf11__blk364_dn17 * locals.var_pf33__blk373) + (locals.var_pf11__blk364 * locals.var_pf33__blk373_dn17)),)
    } else {
        (locals.var_pji22__blk380, locals.var_pji22__blk380_dn0, locals.var_pji22__blk380_dn2, locals.var_pji22__blk380_dn6, locals.var_pji22__blk380_dn7, locals.var_pji22__blk380_dn10, locals.var_pji22__blk380_dn11, locals.var_pji22__blk380_dn12, locals.var_pji22__blk380_dn17,)
    }
};
            locals.var_pji22__blk380 = assign12920_body138_e18147;
            locals.var_pji22__blk380_dn0 = assign12920_body138_e18147_d_n0;
            locals.var_pji22__blk380_dn2 = assign12920_body138_e18147_d_n2;
            locals.var_pji22__blk380_dn6 = assign12920_body138_e18147_d_n6;
            locals.var_pji22__blk380_dn7 = assign12920_body138_e18147_d_n7;
            locals.var_pji22__blk380_dn10 = assign12920_body138_e18147_d_n10;
            locals.var_pji22__blk380_dn11 = assign12920_body138_e18147_d_n11;
            locals.var_pji22__blk380_dn12 = assign12920_body138_e18147_d_n12;
            locals.var_pji22__blk380_dn17 = assign12920_body138_e18147_d_n17;
            let (assign12920_body139_e18163, assign12920_body139_e18163_d_n0, assign12920_body139_e18163_d_n2, assign12920_body139_e18163_d_n6, assign12920_body139_e18163_d_n7, assign12920_body139_e18163_d_n10, assign12920_body139_e18163_d_n11, assign12920_body139_e18163_d_n12, assign12920_body139_e18163_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) {
        let assign12920_body139_e18157: f64 = (locals.var_pf13__blk366 * locals.var_pf21__blk368);
        let assign12920_body139_e18160: f64 = (locals.var_pf11__blk364 * locals.var_pf23__blk370);
        let assign12920_body139_e18161: f64 = (assign12920_body139_e18157 - assign12920_body139_e18160);
        (assign12920_body139_e18161, ((locals.var_pf13__blk366_dn0 * locals.var_pf21__blk368) - ((locals.var_pf11__blk364_dn0 * locals.var_pf23__blk370) + (locals.var_pf11__blk364 * locals.var_pf23__blk370_dn0))), ((locals.var_pf13__blk366_dn2 * locals.var_pf21__blk368) - ((locals.var_pf11__blk364_dn2 * locals.var_pf23__blk370) + (locals.var_pf11__blk364 * locals.var_pf23__blk370_dn2))), ((locals.var_pf13__blk366_dn6 * locals.var_pf21__blk368) - ((locals.var_pf11__blk364_dn6 * locals.var_pf23__blk370) + (locals.var_pf11__blk364 * locals.var_pf23__blk370_dn6))), ((locals.var_pf13__blk366_dn7 * locals.var_pf21__blk368) - ((locals.var_pf11__blk364_dn7 * locals.var_pf23__blk370) + (locals.var_pf11__blk364 * locals.var_pf23__blk370_dn7))), ((locals.var_pf13__blk366_dn10 * locals.var_pf21__blk368) - ((locals.var_pf11__blk364_dn10 * locals.var_pf23__blk370) + (locals.var_pf11__blk364 * locals.var_pf23__blk370_dn10))), ((locals.var_pf13__blk366_dn11 * locals.var_pf21__blk368) - ((locals.var_pf11__blk364_dn11 * locals.var_pf23__blk370) + (locals.var_pf11__blk364 * locals.var_pf23__blk370_dn11))), ((locals.var_pf13__blk366_dn12 * locals.var_pf21__blk368) - ((locals.var_pf11__blk364_dn12 * locals.var_pf23__blk370) + (locals.var_pf11__blk364 * locals.var_pf23__blk370_dn12))), ((locals.var_pf13__blk366_dn17 * locals.var_pf21__blk368) - ((locals.var_pf11__blk364_dn17 * locals.var_pf23__blk370) + (locals.var_pf11__blk364 * locals.var_pf23__blk370_dn17))),)
    } else {
        (locals.var_pji23__blk381, locals.var_pji23__blk381_dn0, locals.var_pji23__blk381_dn2, locals.var_pji23__blk381_dn6, locals.var_pji23__blk381_dn7, locals.var_pji23__blk381_dn10, locals.var_pji23__blk381_dn11, locals.var_pji23__blk381_dn12, locals.var_pji23__blk381_dn17,)
    }
};
            locals.var_pji23__blk381 = assign12920_body139_e18163;
            locals.var_pji23__blk381_dn0 = assign12920_body139_e18163_d_n0;
            locals.var_pji23__blk381_dn2 = assign12920_body139_e18163_d_n2;
            locals.var_pji23__blk381_dn6 = assign12920_body139_e18163_d_n6;
            locals.var_pji23__blk381_dn7 = assign12920_body139_e18163_d_n7;
            locals.var_pji23__blk381_dn10 = assign12920_body139_e18163_d_n10;
            locals.var_pji23__blk381_dn11 = assign12920_body139_e18163_d_n11;
            locals.var_pji23__blk381_dn12 = assign12920_body139_e18163_d_n12;
            locals.var_pji23__blk381_dn17 = assign12920_body139_e18163_d_n17;
            let (assign12920_body140_e18175,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) {
        let assign12920_body140_e18173: f64 = (locals.var_pf21__blk368 * locals.var_pf32__blk372);
        (assign12920_body140_e18173,)
    } else {
        (locals.var_pji31__blk382,)
    }
};
            locals.var_pji31__blk382 = assign12920_body140_e18175;
            let (assign12920_body141_e18188, assign12920_body141_e18188_d_n0, assign12920_body141_e18188_d_n2, assign12920_body141_e18188_d_n6, assign12920_body141_e18188_d_n7, assign12920_body141_e18188_d_n10, assign12920_body141_e18188_d_n11, assign12920_body141_e18188_d_n12, assign12920_body141_e18188_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) {
        let assign12920_body141_e18184: f64 = (-locals.var_pf11__blk364);
        let assign12920_body141_e18186: f64 = (assign12920_body141_e18184 * locals.var_pf32__blk372);
        (assign12920_body141_e18186, ((-locals.var_pf11__blk364_dn0) * locals.var_pf32__blk372), ((-locals.var_pf11__blk364_dn2) * locals.var_pf32__blk372), ((-locals.var_pf11__blk364_dn6) * locals.var_pf32__blk372), ((-locals.var_pf11__blk364_dn7) * locals.var_pf32__blk372), ((-locals.var_pf11__blk364_dn10) * locals.var_pf32__blk372), ((-locals.var_pf11__blk364_dn11) * locals.var_pf32__blk372), ((-locals.var_pf11__blk364_dn12) * locals.var_pf32__blk372), ((-locals.var_pf11__blk364_dn17) * locals.var_pf32__blk372),)
    } else {
        (locals.var_pji32__blk383, locals.var_pji32__blk383_dn0, locals.var_pji32__blk383_dn2, locals.var_pji32__blk383_dn6, locals.var_pji32__blk383_dn7, locals.var_pji32__blk383_dn10, locals.var_pji32__blk383_dn11, locals.var_pji32__blk383_dn12, locals.var_pji32__blk383_dn17,)
    }
};
            locals.var_pji32__blk383 = assign12920_body141_e18188;
            locals.var_pji32__blk383_dn0 = assign12920_body141_e18188_d_n0;
            locals.var_pji32__blk383_dn2 = assign12920_body141_e18188_d_n2;
            locals.var_pji32__blk383_dn6 = assign12920_body141_e18188_d_n6;
            locals.var_pji32__blk383_dn7 = assign12920_body141_e18188_d_n7;
            locals.var_pji32__blk383_dn10 = assign12920_body141_e18188_d_n10;
            locals.var_pji32__blk383_dn11 = assign12920_body141_e18188_d_n11;
            locals.var_pji32__blk383_dn12 = assign12920_body141_e18188_d_n12;
            locals.var_pji32__blk383_dn17 = assign12920_body141_e18188_d_n17;
            let (assign12920_body142_e18204, assign12920_body142_e18204_d_n0, assign12920_body142_e18204_d_n2, assign12920_body142_e18204_d_n6, assign12920_body142_e18204_d_n7, assign12920_body142_e18204_d_n10, assign12920_body142_e18204_d_n11, assign12920_body142_e18204_d_n12, assign12920_body142_e18204_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) {
        let assign12920_body142_e18198: f64 = (locals.var_pf11__blk364 * locals.var_pf22__blk369);
        let assign12920_body142_e18201: f64 = (locals.var_pf12__blk365 * locals.var_pf21__blk368);
        let assign12920_body142_e18202: f64 = (assign12920_body142_e18198 - assign12920_body142_e18201);
        (assign12920_body142_e18202, ((locals.var_pf11__blk364_dn0 * locals.var_pf22__blk369) - (locals.var_pf12__blk365_dn0 * locals.var_pf21__blk368)), ((locals.var_pf11__blk364_dn2 * locals.var_pf22__blk369) - (locals.var_pf12__blk365_dn2 * locals.var_pf21__blk368)), ((locals.var_pf11__blk364_dn6 * locals.var_pf22__blk369) - (locals.var_pf12__blk365_dn6 * locals.var_pf21__blk368)), ((locals.var_pf11__blk364_dn7 * locals.var_pf22__blk369) - (locals.var_pf12__blk365_dn7 * locals.var_pf21__blk368)), ((locals.var_pf11__blk364_dn10 * locals.var_pf22__blk369) - (locals.var_pf12__blk365_dn10 * locals.var_pf21__blk368)), ((locals.var_pf11__blk364_dn11 * locals.var_pf22__blk369) - (locals.var_pf12__blk365_dn11 * locals.var_pf21__blk368)), ((locals.var_pf11__blk364_dn12 * locals.var_pf22__blk369) - (locals.var_pf12__blk365_dn12 * locals.var_pf21__blk368)), ((locals.var_pf11__blk364_dn17 * locals.var_pf22__blk369) - (locals.var_pf12__blk365_dn17 * locals.var_pf21__blk368)),)
    } else {
        (locals.var_pji33__blk384, locals.var_pji33__blk384_dn0, locals.var_pji33__blk384_dn2, locals.var_pji33__blk384_dn6, locals.var_pji33__blk384_dn7, locals.var_pji33__blk384_dn10, locals.var_pji33__blk384_dn11, locals.var_pji33__blk384_dn12, locals.var_pji33__blk384_dn17,)
    }
};
            locals.var_pji33__blk384 = assign12920_body142_e18204;
            locals.var_pji33__blk384_dn0 = assign12920_body142_e18204_d_n0;
            locals.var_pji33__blk384_dn2 = assign12920_body142_e18204_d_n2;
            locals.var_pji33__blk384_dn6 = assign12920_body142_e18204_d_n6;
            locals.var_pji33__blk384_dn7 = assign12920_body142_e18204_d_n7;
            locals.var_pji33__blk384_dn10 = assign12920_body142_e18204_d_n10;
            locals.var_pji33__blk384_dn11 = assign12920_body142_e18204_d_n11;
            locals.var_pji33__blk384_dn12 = assign12920_body142_e18204_d_n12;
            locals.var_pji33__blk384_dn17 = assign12920_body142_e18204_d_n17;
            let (assign12920_body143_e18227, assign12920_body143_e18227_d_n0, assign12920_body143_e18227_d_n2, assign12920_body143_e18227_d_n6, assign12920_body143_e18227_d_n7, assign12920_body143_e18227_d_n10, assign12920_body143_e18227_d_n11, assign12920_body143_e18227_d_n12, assign12920_body143_e18227_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) {
        let assign12920_body143_e18213: f64 = (-locals.var_pdji__blk375);
        let assign12920_body143_e18216: f64 = (locals.var_pji11__blk376 * locals.var_pf1__blk363);
        let assign12920_body143_e18219: f64 = (locals.var_pji12__blk377 * locals.var_pf2__blk367);
        let assign12920_body143_e18220: f64 = (assign12920_body143_e18216 + assign12920_body143_e18219);
        let assign12920_body143_e18223: f64 = (locals.var_pji13__blk378 * locals.var_pf3__blk371);
        let assign12920_body143_e18224: f64 = (assign12920_body143_e18220 + assign12920_body143_e18223);
        let assign12920_body143_e18225: f64 = (assign12920_body143_e18213 * assign12920_body143_e18224);
        (assign12920_body143_e18225, (((-locals.var_pdji__blk375_dn0) * assign12920_body143_e18224) + (assign12920_body143_e18213 * ((((locals.var_pji11__blk376_dn0 * locals.var_pf1__blk363) + (locals.var_pji11__blk376 * locals.var_pf1__blk363_dn0)) + ((locals.var_pji12__blk377_dn0 * locals.var_pf2__blk367) + (locals.var_pji12__blk377 * locals.var_pf2__blk367_dn0))) + ((locals.var_pji13__blk378_dn0 * locals.var_pf3__blk371) + (locals.var_pji13__blk378 * locals.var_pf3__blk371_dn0))))), (((-locals.var_pdji__blk375_dn2) * assign12920_body143_e18224) + (assign12920_body143_e18213 * ((((locals.var_pji11__blk376_dn2 * locals.var_pf1__blk363) + (locals.var_pji11__blk376 * locals.var_pf1__blk363_dn2)) + ((locals.var_pji12__blk377_dn2 * locals.var_pf2__blk367) + (locals.var_pji12__blk377 * locals.var_pf2__blk367_dn2))) + ((locals.var_pji13__blk378_dn2 * locals.var_pf3__blk371) + (locals.var_pji13__blk378 * locals.var_pf3__blk371_dn2))))), (((-locals.var_pdji__blk375_dn6) * assign12920_body143_e18224) + (assign12920_body143_e18213 * ((((locals.var_pji11__blk376_dn6 * locals.var_pf1__blk363) + (locals.var_pji11__blk376 * locals.var_pf1__blk363_dn6)) + ((locals.var_pji12__blk377_dn6 * locals.var_pf2__blk367) + (locals.var_pji12__blk377 * locals.var_pf2__blk367_dn6))) + ((locals.var_pji13__blk378_dn6 * locals.var_pf3__blk371) + (locals.var_pji13__blk378 * locals.var_pf3__blk371_dn6))))), (((-locals.var_pdji__blk375_dn7) * assign12920_body143_e18224) + (assign12920_body143_e18213 * ((((locals.var_pji11__blk376_dn7 * locals.var_pf1__blk363) + (locals.var_pji11__blk376 * locals.var_pf1__blk363_dn7)) + ((locals.var_pji12__blk377_dn7 * locals.var_pf2__blk367) + (locals.var_pji12__blk377 * locals.var_pf2__blk367_dn7))) + ((locals.var_pji13__blk378_dn7 * locals.var_pf3__blk371) + (locals.var_pji13__blk378 * locals.var_pf3__blk371_dn7))))), (((-locals.var_pdji__blk375_dn10) * assign12920_body143_e18224) + (assign12920_body143_e18213 * ((((locals.var_pji11__blk376_dn10 * locals.var_pf1__blk363) + (locals.var_pji11__blk376 * locals.var_pf1__blk363_dn10)) + ((locals.var_pji12__blk377_dn10 * locals.var_pf2__blk367) + (locals.var_pji12__blk377 * locals.var_pf2__blk367_dn10))) + ((locals.var_pji13__blk378_dn10 * locals.var_pf3__blk371) + (locals.var_pji13__blk378 * locals.var_pf3__blk371_dn10))))), (((-locals.var_pdji__blk375_dn11) * assign12920_body143_e18224) + (assign12920_body143_e18213 * ((((locals.var_pji11__blk376_dn11 * locals.var_pf1__blk363) + (locals.var_pji11__blk376 * locals.var_pf1__blk363_dn11)) + ((locals.var_pji12__blk377_dn11 * locals.var_pf2__blk367) + (locals.var_pji12__blk377 * locals.var_pf2__blk367_dn11))) + ((locals.var_pji13__blk378_dn11 * locals.var_pf3__blk371) + (locals.var_pji13__blk378 * locals.var_pf3__blk371_dn11))))), (((-locals.var_pdji__blk375_dn12) * assign12920_body143_e18224) + (assign12920_body143_e18213 * ((((locals.var_pji11__blk376_dn12 * locals.var_pf1__blk363) + (locals.var_pji11__blk376 * locals.var_pf1__blk363_dn12)) + ((locals.var_pji12__blk377_dn12 * locals.var_pf2__blk367) + (locals.var_pji12__blk377 * locals.var_pf2__blk367_dn12))) + ((locals.var_pji13__blk378_dn12 * locals.var_pf3__blk371) + (locals.var_pji13__blk378 * locals.var_pf3__blk371_dn12))))), (((-locals.var_pdji__blk375_dn17) * assign12920_body143_e18224) + (assign12920_body143_e18213 * ((((locals.var_pji11__blk376_dn17 * locals.var_pf1__blk363) + (locals.var_pji11__blk376 * locals.var_pf1__blk363_dn17)) + ((locals.var_pji12__blk377_dn17 * locals.var_pf2__blk367) + (locals.var_pji12__blk377 * locals.var_pf2__blk367_dn17))) + ((locals.var_pji13__blk378_dn17 * locals.var_pf3__blk371) + (locals.var_pji13__blk378 * locals.var_pf3__blk371_dn17))))),)
    } else {
        (locals.var_dpss__blk360, locals.var_dpss__blk360_dn0, locals.var_dpss__blk360_dn2, locals.var_dpss__blk360_dn6, locals.var_dpss__blk360_dn7, locals.var_dpss__blk360_dn10, locals.var_dpss__blk360_dn11, locals.var_dpss__blk360_dn12, locals.var_dpss__blk360_dn17,)
    }
};
            locals.var_dpss__blk360 = assign12920_body143_e18227;
            locals.var_dpss__blk360_dn0 = assign12920_body143_e18227_d_n0;
            locals.var_dpss__blk360_dn2 = assign12920_body143_e18227_d_n2;
            locals.var_dpss__blk360_dn6 = assign12920_body143_e18227_d_n6;
            locals.var_dpss__blk360_dn7 = assign12920_body143_e18227_d_n7;
            locals.var_dpss__blk360_dn10 = assign12920_body143_e18227_d_n10;
            locals.var_dpss__blk360_dn11 = assign12920_body143_e18227_d_n11;
            locals.var_dpss__blk360_dn12 = assign12920_body143_e18227_d_n12;
            locals.var_dpss__blk360_dn17 = assign12920_body143_e18227_d_n17;
            let (assign12920_body144_e18250, assign12920_body144_e18250_d_n0, assign12920_body144_e18250_d_n2, assign12920_body144_e18250_d_n6, assign12920_body144_e18250_d_n7, assign12920_body144_e18250_d_n10, assign12920_body144_e18250_d_n11, assign12920_body144_e18250_d_n12, assign12920_body144_e18250_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) {
        let assign12920_body144_e18236: f64 = (-locals.var_pdji__blk375);
        let assign12920_body144_e18239: f64 = (locals.var_pji21__blk379 * locals.var_pf1__blk363);
        let assign12920_body144_e18242: f64 = (locals.var_pji22__blk380 * locals.var_pf2__blk367);
        let assign12920_body144_e18243: f64 = (assign12920_body144_e18239 + assign12920_body144_e18242);
        let assign12920_body144_e18246: f64 = (locals.var_pji23__blk381 * locals.var_pf3__blk371);
        let assign12920_body144_e18247: f64 = (assign12920_body144_e18243 + assign12920_body144_e18246);
        let assign12920_body144_e18248: f64 = (assign12920_body144_e18236 * assign12920_body144_e18247);
        (assign12920_body144_e18248, (((-locals.var_pdji__blk375_dn0) * assign12920_body144_e18247) + (assign12920_body144_e18236 * ((((locals.var_pji21__blk379_dn0 * locals.var_pf1__blk363) + (locals.var_pji21__blk379 * locals.var_pf1__blk363_dn0)) + ((locals.var_pji22__blk380_dn0 * locals.var_pf2__blk367) + (locals.var_pji22__blk380 * locals.var_pf2__blk367_dn0))) + ((locals.var_pji23__blk381_dn0 * locals.var_pf3__blk371) + (locals.var_pji23__blk381 * locals.var_pf3__blk371_dn0))))), (((-locals.var_pdji__blk375_dn2) * assign12920_body144_e18247) + (assign12920_body144_e18236 * ((((locals.var_pji21__blk379_dn2 * locals.var_pf1__blk363) + (locals.var_pji21__blk379 * locals.var_pf1__blk363_dn2)) + ((locals.var_pji22__blk380_dn2 * locals.var_pf2__blk367) + (locals.var_pji22__blk380 * locals.var_pf2__blk367_dn2))) + ((locals.var_pji23__blk381_dn2 * locals.var_pf3__blk371) + (locals.var_pji23__blk381 * locals.var_pf3__blk371_dn2))))), (((-locals.var_pdji__blk375_dn6) * assign12920_body144_e18247) + (assign12920_body144_e18236 * ((((locals.var_pji21__blk379_dn6 * locals.var_pf1__blk363) + (locals.var_pji21__blk379 * locals.var_pf1__blk363_dn6)) + ((locals.var_pji22__blk380_dn6 * locals.var_pf2__blk367) + (locals.var_pji22__blk380 * locals.var_pf2__blk367_dn6))) + ((locals.var_pji23__blk381_dn6 * locals.var_pf3__blk371) + (locals.var_pji23__blk381 * locals.var_pf3__blk371_dn6))))), (((-locals.var_pdji__blk375_dn7) * assign12920_body144_e18247) + (assign12920_body144_e18236 * ((((locals.var_pji21__blk379_dn7 * locals.var_pf1__blk363) + (locals.var_pji21__blk379 * locals.var_pf1__blk363_dn7)) + ((locals.var_pji22__blk380_dn7 * locals.var_pf2__blk367) + (locals.var_pji22__blk380 * locals.var_pf2__blk367_dn7))) + ((locals.var_pji23__blk381_dn7 * locals.var_pf3__blk371) + (locals.var_pji23__blk381 * locals.var_pf3__blk371_dn7))))), (((-locals.var_pdji__blk375_dn10) * assign12920_body144_e18247) + (assign12920_body144_e18236 * ((((locals.var_pji21__blk379_dn10 * locals.var_pf1__blk363) + (locals.var_pji21__blk379 * locals.var_pf1__blk363_dn10)) + ((locals.var_pji22__blk380_dn10 * locals.var_pf2__blk367) + (locals.var_pji22__blk380 * locals.var_pf2__blk367_dn10))) + ((locals.var_pji23__blk381_dn10 * locals.var_pf3__blk371) + (locals.var_pji23__blk381 * locals.var_pf3__blk371_dn10))))), (((-locals.var_pdji__blk375_dn11) * assign12920_body144_e18247) + (assign12920_body144_e18236 * ((((locals.var_pji21__blk379_dn11 * locals.var_pf1__blk363) + (locals.var_pji21__blk379 * locals.var_pf1__blk363_dn11)) + ((locals.var_pji22__blk380_dn11 * locals.var_pf2__blk367) + (locals.var_pji22__blk380 * locals.var_pf2__blk367_dn11))) + ((locals.var_pji23__blk381_dn11 * locals.var_pf3__blk371) + (locals.var_pji23__blk381 * locals.var_pf3__blk371_dn11))))), (((-locals.var_pdji__blk375_dn12) * assign12920_body144_e18247) + (assign12920_body144_e18236 * ((((locals.var_pji21__blk379_dn12 * locals.var_pf1__blk363) + (locals.var_pji21__blk379 * locals.var_pf1__blk363_dn12)) + ((locals.var_pji22__blk380_dn12 * locals.var_pf2__blk367) + (locals.var_pji22__blk380 * locals.var_pf2__blk367_dn12))) + ((locals.var_pji23__blk381_dn12 * locals.var_pf3__blk371) + (locals.var_pji23__blk381 * locals.var_pf3__blk371_dn12))))), (((-locals.var_pdji__blk375_dn17) * assign12920_body144_e18247) + (assign12920_body144_e18236 * ((((locals.var_pji21__blk379_dn17 * locals.var_pf1__blk363) + (locals.var_pji21__blk379 * locals.var_pf1__blk363_dn17)) + ((locals.var_pji22__blk380_dn17 * locals.var_pf2__blk367) + (locals.var_pji22__blk380 * locals.var_pf2__blk367_dn17))) + ((locals.var_pji23__blk381_dn17 * locals.var_pf3__blk371) + (locals.var_pji23__blk381 * locals.var_pf3__blk371_dn17))))),)
    } else {
        (locals.var_dpbs__blk361, locals.var_dpbs__blk361_dn0, locals.var_dpbs__blk361_dn2, locals.var_dpbs__blk361_dn6, locals.var_dpbs__blk361_dn7, locals.var_dpbs__blk361_dn10, locals.var_dpbs__blk361_dn11, locals.var_dpbs__blk361_dn12, locals.var_dpbs__blk361_dn17,)
    }
};
            locals.var_dpbs__blk361 = assign12920_body144_e18250;
            locals.var_dpbs__blk361_dn0 = assign12920_body144_e18250_d_n0;
            locals.var_dpbs__blk361_dn2 = assign12920_body144_e18250_d_n2;
            locals.var_dpbs__blk361_dn6 = assign12920_body144_e18250_d_n6;
            locals.var_dpbs__blk361_dn7 = assign12920_body144_e18250_d_n7;
            locals.var_dpbs__blk361_dn10 = assign12920_body144_e18250_d_n10;
            locals.var_dpbs__blk361_dn11 = assign12920_body144_e18250_d_n11;
            locals.var_dpbs__blk361_dn12 = assign12920_body144_e18250_d_n12;
            locals.var_dpbs__blk361_dn17 = assign12920_body144_e18250_d_n17;
            let (assign12920_body145_e18273, assign12920_body145_e18273_d_n0, assign12920_body145_e18273_d_n2, assign12920_body145_e18273_d_n6, assign12920_body145_e18273_d_n7, assign12920_body145_e18273_d_n10, assign12920_body145_e18273_d_n11, assign12920_body145_e18273_d_n12, assign12920_body145_e18273_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) {
        let assign12920_body145_e18259: f64 = (-locals.var_pdji__blk375);
        let assign12920_body145_e18262: f64 = (locals.var_pji31__blk382 * locals.var_pf1__blk363);
        let assign12920_body145_e18265: f64 = (locals.var_pji32__blk383 * locals.var_pf2__blk367);
        let assign12920_body145_e18266: f64 = (assign12920_body145_e18262 + assign12920_body145_e18265);
        let assign12920_body145_e18269: f64 = (locals.var_pji33__blk384 * locals.var_pf3__blk371);
        let assign12920_body145_e18270: f64 = (assign12920_body145_e18266 + assign12920_body145_e18269);
        let assign12920_body145_e18271: f64 = (assign12920_body145_e18259 * assign12920_body145_e18270);
        (assign12920_body145_e18271, (((-locals.var_pdji__blk375_dn0) * assign12920_body145_e18270) + (assign12920_body145_e18259 * (((locals.var_pji31__blk382 * locals.var_pf1__blk363_dn0) + ((locals.var_pji32__blk383_dn0 * locals.var_pf2__blk367) + (locals.var_pji32__blk383 * locals.var_pf2__blk367_dn0))) + ((locals.var_pji33__blk384_dn0 * locals.var_pf3__blk371) + (locals.var_pji33__blk384 * locals.var_pf3__blk371_dn0))))), (((-locals.var_pdji__blk375_dn2) * assign12920_body145_e18270) + (assign12920_body145_e18259 * (((locals.var_pji31__blk382 * locals.var_pf1__blk363_dn2) + ((locals.var_pji32__blk383_dn2 * locals.var_pf2__blk367) + (locals.var_pji32__blk383 * locals.var_pf2__blk367_dn2))) + ((locals.var_pji33__blk384_dn2 * locals.var_pf3__blk371) + (locals.var_pji33__blk384 * locals.var_pf3__blk371_dn2))))), (((-locals.var_pdji__blk375_dn6) * assign12920_body145_e18270) + (assign12920_body145_e18259 * (((locals.var_pji31__blk382 * locals.var_pf1__blk363_dn6) + ((locals.var_pji32__blk383_dn6 * locals.var_pf2__blk367) + (locals.var_pji32__blk383 * locals.var_pf2__blk367_dn6))) + ((locals.var_pji33__blk384_dn6 * locals.var_pf3__blk371) + (locals.var_pji33__blk384 * locals.var_pf3__blk371_dn6))))), (((-locals.var_pdji__blk375_dn7) * assign12920_body145_e18270) + (assign12920_body145_e18259 * (((locals.var_pji31__blk382 * locals.var_pf1__blk363_dn7) + ((locals.var_pji32__blk383_dn7 * locals.var_pf2__blk367) + (locals.var_pji32__blk383 * locals.var_pf2__blk367_dn7))) + ((locals.var_pji33__blk384_dn7 * locals.var_pf3__blk371) + (locals.var_pji33__blk384 * locals.var_pf3__blk371_dn7))))), (((-locals.var_pdji__blk375_dn10) * assign12920_body145_e18270) + (assign12920_body145_e18259 * (((locals.var_pji31__blk382 * locals.var_pf1__blk363_dn10) + ((locals.var_pji32__blk383_dn10 * locals.var_pf2__blk367) + (locals.var_pji32__blk383 * locals.var_pf2__blk367_dn10))) + ((locals.var_pji33__blk384_dn10 * locals.var_pf3__blk371) + (locals.var_pji33__blk384 * locals.var_pf3__blk371_dn10))))), (((-locals.var_pdji__blk375_dn11) * assign12920_body145_e18270) + (assign12920_body145_e18259 * (((locals.var_pji31__blk382 * locals.var_pf1__blk363_dn11) + ((locals.var_pji32__blk383_dn11 * locals.var_pf2__blk367) + (locals.var_pji32__blk383 * locals.var_pf2__blk367_dn11))) + ((locals.var_pji33__blk384_dn11 * locals.var_pf3__blk371) + (locals.var_pji33__blk384 * locals.var_pf3__blk371_dn11))))), (((-locals.var_pdji__blk375_dn12) * assign12920_body145_e18270) + (assign12920_body145_e18259 * (((locals.var_pji31__blk382 * locals.var_pf1__blk363_dn12) + ((locals.var_pji32__blk383_dn12 * locals.var_pf2__blk367) + (locals.var_pji32__blk383 * locals.var_pf2__blk367_dn12))) + ((locals.var_pji33__blk384_dn12 * locals.var_pf3__blk371) + (locals.var_pji33__blk384 * locals.var_pf3__blk371_dn12))))), (((-locals.var_pdji__blk375_dn17) * assign12920_body145_e18270) + (assign12920_body145_e18259 * (((locals.var_pji31__blk382 * locals.var_pf1__blk363_dn17) + ((locals.var_pji32__blk383_dn17 * locals.var_pf2__blk367) + (locals.var_pji32__blk383 * locals.var_pf2__blk367_dn17))) + ((locals.var_pji33__blk384_dn17 * locals.var_pf3__blk371) + (locals.var_pji33__blk384 * locals.var_pf3__blk371_dn17))))),)
    } else {
        (locals.var_dpsb__blk362, locals.var_dpsb__blk362_dn0, locals.var_dpsb__blk362_dn2, locals.var_dpsb__blk362_dn6, locals.var_dpsb__blk362_dn7, locals.var_dpsb__blk362_dn10, locals.var_dpsb__blk362_dn11, locals.var_dpsb__blk362_dn12, locals.var_dpsb__blk362_dn17,)
    }
};
            locals.var_dpsb__blk362 = assign12920_body145_e18273;
            locals.var_dpsb__blk362_dn0 = assign12920_body145_e18273_d_n0;
            locals.var_dpsb__blk362_dn2 = assign12920_body145_e18273_d_n2;
            locals.var_dpsb__blk362_dn6 = assign12920_body145_e18273_d_n6;
            locals.var_dpsb__blk362_dn7 = assign12920_body145_e18273_d_n7;
            locals.var_dpsb__blk362_dn10 = assign12920_body145_e18273_d_n10;
            locals.var_dpsb__blk362_dn11 = assign12920_body145_e18273_d_n11;
            locals.var_dpsb__blk362_dn12 = assign12920_body145_e18273_d_n12;
            locals.var_dpsb__blk362_dn17 = assign12920_body145_e18273_d_n17;
            let (assign12920_body146_e18284, assign12920_body146_e18284_d_n0, assign12920_body146_e18284_d_n2, assign12920_body146_e18284_d_n6, assign12920_body146_e18284_d_n7, assign12920_body146_e18284_d_n10, assign12920_body146_e18284_d_n11, assign12920_body146_e18284_d_n12, assign12920_body146_e18284_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) {
        let assign12920_body146_e18282: f64 = (locals.var_dpss__blk360).abs();
        (assign12920_body146_e18282, if locals.var_dpss__blk360 >= 0.0 { locals.var_dpss__blk360_dn0 } else { (-locals.var_dpss__blk360_dn0) }, if locals.var_dpss__blk360 >= 0.0 { locals.var_dpss__blk360_dn2 } else { (-locals.var_dpss__blk360_dn2) }, if locals.var_dpss__blk360 >= 0.0 { locals.var_dpss__blk360_dn6 } else { (-locals.var_dpss__blk360_dn6) }, if locals.var_dpss__blk360 >= 0.0 { locals.var_dpss__blk360_dn7 } else { (-locals.var_dpss__blk360_dn7) }, if locals.var_dpss__blk360 >= 0.0 { locals.var_dpss__blk360_dn10 } else { (-locals.var_dpss__blk360_dn10) }, if locals.var_dpss__blk360 >= 0.0 { locals.var_dpss__blk360_dn11 } else { (-locals.var_dpss__blk360_dn11) }, if locals.var_dpss__blk360 >= 0.0 { locals.var_dpss__blk360_dn12 } else { (-locals.var_dpss__blk360_dn12) }, if locals.var_dpss__blk360 >= 0.0 { locals.var_dpss__blk360_dn17 } else { (-locals.var_dpss__blk360_dn17) },)
    } else {
        (locals.var_t1__blk353, locals.var_t1__blk353_dn0, locals.var_t1__blk353_dn2, locals.var_t1__blk353_dn6, locals.var_t1__blk353_dn7, locals.var_t1__blk353_dn10, locals.var_t1__blk353_dn11, locals.var_t1__blk353_dn12, locals.var_t1__blk353_dn17,)
    }
};
            locals.var_t1__blk353 = assign12920_body146_e18284;
            locals.var_t1__blk353_dn0 = assign12920_body146_e18284_d_n0;
            locals.var_t1__blk353_dn2 = assign12920_body146_e18284_d_n2;
            locals.var_t1__blk353_dn6 = assign12920_body146_e18284_d_n6;
            locals.var_t1__blk353_dn7 = assign12920_body146_e18284_d_n7;
            locals.var_t1__blk353_dn10 = assign12920_body146_e18284_d_n10;
            locals.var_t1__blk353_dn11 = assign12920_body146_e18284_d_n11;
            locals.var_t1__blk353_dn12 = assign12920_body146_e18284_d_n12;
            locals.var_t1__blk353_dn17 = assign12920_body146_e18284_d_n17;
            let assign12920_body147_e18287: f64 = (locals.var_dpbs__blk361).abs();
            let assign12920_body147_e18288: f64 = if locals.var_t1__blk353 < assign12920_body147_e18287 { 1.0 } else { 0.0 };
            locals.var_guard407 = assign12920_body147_e18288;
            let (assign12920_body148_e18301, assign12920_body148_e18301_d_n0, assign12920_body148_e18301_d_n2, assign12920_body148_e18301_d_n6, assign12920_body148_e18301_d_n7, assign12920_body148_e18301_d_n10, assign12920_body148_e18301_d_n11, assign12920_body148_e18301_d_n12, assign12920_body148_e18301_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) && (locals.var_guard407 != 0.0)) {
        let assign12920_body148_e18299: f64 = (locals.var_dpbs__blk361).abs();
        (assign12920_body148_e18299, if locals.var_dpbs__blk361 >= 0.0 { locals.var_dpbs__blk361_dn0 } else { (-locals.var_dpbs__blk361_dn0) }, if locals.var_dpbs__blk361 >= 0.0 { locals.var_dpbs__blk361_dn2 } else { (-locals.var_dpbs__blk361_dn2) }, if locals.var_dpbs__blk361 >= 0.0 { locals.var_dpbs__blk361_dn6 } else { (-locals.var_dpbs__blk361_dn6) }, if locals.var_dpbs__blk361 >= 0.0 { locals.var_dpbs__blk361_dn7 } else { (-locals.var_dpbs__blk361_dn7) }, if locals.var_dpbs__blk361 >= 0.0 { locals.var_dpbs__blk361_dn10 } else { (-locals.var_dpbs__blk361_dn10) }, if locals.var_dpbs__blk361 >= 0.0 { locals.var_dpbs__blk361_dn11 } else { (-locals.var_dpbs__blk361_dn11) }, if locals.var_dpbs__blk361 >= 0.0 { locals.var_dpbs__blk361_dn12 } else { (-locals.var_dpbs__blk361_dn12) }, if locals.var_dpbs__blk361 >= 0.0 { locals.var_dpbs__blk361_dn17 } else { (-locals.var_dpbs__blk361_dn17) },)
    } else {
        (locals.var_t1__blk353, locals.var_t1__blk353_dn0, locals.var_t1__blk353_dn2, locals.var_t1__blk353_dn6, locals.var_t1__blk353_dn7, locals.var_t1__blk353_dn10, locals.var_t1__blk353_dn11, locals.var_t1__blk353_dn12, locals.var_t1__blk353_dn17,)
    }
};
            locals.var_t1__blk353 = assign12920_body148_e18301;
            locals.var_t1__blk353_dn0 = assign12920_body148_e18301_d_n0;
            locals.var_t1__blk353_dn2 = assign12920_body148_e18301_d_n2;
            locals.var_t1__blk353_dn6 = assign12920_body148_e18301_d_n6;
            locals.var_t1__blk353_dn7 = assign12920_body148_e18301_d_n7;
            locals.var_t1__blk353_dn10 = assign12920_body148_e18301_d_n10;
            locals.var_t1__blk353_dn11 = assign12920_body148_e18301_d_n11;
            locals.var_t1__blk353_dn12 = assign12920_body148_e18301_d_n12;
            locals.var_t1__blk353_dn17 = assign12920_body148_e18301_d_n17;
            let assign12920_body149_e18304: f64 = (locals.var_dpsb__blk362).abs();
            let assign12920_body149_e18305: f64 = if locals.var_t1__blk353 < assign12920_body149_e18304 { 1.0 } else { 0.0 };
            locals.var_guard408 = assign12920_body149_e18305;
            let (assign12920_body150_e18318, assign12920_body150_e18318_d_n0, assign12920_body150_e18318_d_n2, assign12920_body150_e18318_d_n6, assign12920_body150_e18318_d_n7, assign12920_body150_e18318_d_n10, assign12920_body150_e18318_d_n11, assign12920_body150_e18318_d_n12, assign12920_body150_e18318_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) && (locals.var_guard408 != 0.0)) {
        let assign12920_body150_e18316: f64 = (locals.var_dpsb__blk362).abs();
        (assign12920_body150_e18316, if locals.var_dpsb__blk362 >= 0.0 { locals.var_dpsb__blk362_dn0 } else { (-locals.var_dpsb__blk362_dn0) }, if locals.var_dpsb__blk362 >= 0.0 { locals.var_dpsb__blk362_dn2 } else { (-locals.var_dpsb__blk362_dn2) }, if locals.var_dpsb__blk362 >= 0.0 { locals.var_dpsb__blk362_dn6 } else { (-locals.var_dpsb__blk362_dn6) }, if locals.var_dpsb__blk362 >= 0.0 { locals.var_dpsb__blk362_dn7 } else { (-locals.var_dpsb__blk362_dn7) }, if locals.var_dpsb__blk362 >= 0.0 { locals.var_dpsb__blk362_dn10 } else { (-locals.var_dpsb__blk362_dn10) }, if locals.var_dpsb__blk362 >= 0.0 { locals.var_dpsb__blk362_dn11 } else { (-locals.var_dpsb__blk362_dn11) }, if locals.var_dpsb__blk362 >= 0.0 { locals.var_dpsb__blk362_dn12 } else { (-locals.var_dpsb__blk362_dn12) }, if locals.var_dpsb__blk362 >= 0.0 { locals.var_dpsb__blk362_dn17 } else { (-locals.var_dpsb__blk362_dn17) },)
    } else {
        (locals.var_t1__blk353, locals.var_t1__blk353_dn0, locals.var_t1__blk353_dn2, locals.var_t1__blk353_dn6, locals.var_t1__blk353_dn7, locals.var_t1__blk353_dn10, locals.var_t1__blk353_dn11, locals.var_t1__blk353_dn12, locals.var_t1__blk353_dn17,)
    }
};
            locals.var_t1__blk353 = assign12920_body150_e18318;
            locals.var_t1__blk353_dn0 = assign12920_body150_e18318_d_n0;
            locals.var_t1__blk353_dn2 = assign12920_body150_e18318_d_n2;
            locals.var_t1__blk353_dn6 = assign12920_body150_e18318_d_n6;
            locals.var_t1__blk353_dn7 = assign12920_body150_e18318_d_n7;
            locals.var_t1__blk353_dn10 = assign12920_body150_e18318_d_n10;
            locals.var_t1__blk353_dn11 = assign12920_body150_e18318_d_n11;
            locals.var_t1__blk353_dn12 = assign12920_body150_e18318_d_n12;
            locals.var_t1__blk353_dn17 = assign12920_body150_e18318_d_n17;
            let (assign12920_body151_e18328,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_scale_fac,)
    }
};
            locals.var_scale_fac = assign12920_body151_e18328;
            let assign12920_body152_e18331: f64 = if locals.var_lp_sl > 80.0 { 1.0 } else { 0.0 };
            locals.var_guard409 = assign12920_body152_e18331;
            let (assign12920_body153_e18343,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) && (locals.var_guard409 != 0.0)) {
        (125.0,)
    } else {
        (locals.var_scale_fac,)
    }
};
            locals.var_scale_fac = assign12920_body153_e18343;
            let assign12920_body154_e18346: f64 = if locals.var_lp_sl > 40.0 { 1.0 } else { 0.0 };
            locals.var_guard410 = assign12920_body154_e18346;
            let (assign12920_body155_e18361,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) && (locals.var_guard409 == 0.0)) && (locals.var_guard410 != 0.0)) {
        (125.0,)
    } else {
        (locals.var_scale_fac,)
    }
};
            locals.var_scale_fac = assign12920_body155_e18361;
            let assign12920_body156_e18364: f64 = if locals.var_lp_sl > 20.0 { 1.0 } else { 0.0 };
            locals.var_guard411 = assign12920_body156_e18364;
            let (assign12920_body157_e18382,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) && (locals.var_guard409 == 0.0)) && (locals.var_guard410 == 0.0)) && (locals.var_guard411 != 0.0)) {
        (25.0,)
    } else {
        (locals.var_scale_fac,)
    }
};
            locals.var_scale_fac = assign12920_body157_e18382;
            let assign12920_body158_e18385: f64 = if locals.var_lp_sl > 10.0 { 1.0 } else { 0.0 };
            locals.var_guard412 = assign12920_body158_e18385;
            let (assign12920_body159_e18406,) = {
    if (((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) && (locals.var_guard409 == 0.0)) && (locals.var_guard410 == 0.0)) && (locals.var_guard411 == 0.0)) && (locals.var_guard412 != 0.0)) {
        (5.0,)
    } else {
        (locals.var_scale_fac,)
    }
};
            locals.var_scale_fac = assign12920_body159_e18406;
            let assign12920_body160_e18410: f64 = (0.1 / locals.var_scale_fac);
            let assign12920_body160_e18411: f64 = if locals.var_t1__blk353 > assign12920_body160_e18410 { 1.0 } else { 0.0 };
            locals.var_guard413 = assign12920_body160_e18411;
            let (assign12920_body161_e18429, assign12920_body161_e18429_d_n0, assign12920_body161_e18429_d_n2, assign12920_body161_e18429_d_n6, assign12920_body161_e18429_d_n7, assign12920_body161_e18429_d_n10, assign12920_body161_e18429_d_n11, assign12920_body161_e18429_d_n12, assign12920_body161_e18429_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign12920_body161_e18424: f64 = (0.1 / locals.var_scale_fac);
        let assign12920_body161_e18426: f64 = (assign12920_body161_e18424 / locals.var_t1__blk353);
        let assign12920_body161_e18427: f64 = (locals.var_dpss__blk360 * assign12920_body161_e18426);
        (assign12920_body161_e18427, ((locals.var_dpss__blk360_dn0 * assign12920_body161_e18426) + (locals.var_dpss__blk360 * (-((assign12920_body161_e18424 * locals.var_t1__blk353_dn0) / (locals.var_t1__blk353 * locals.var_t1__blk353))))), ((locals.var_dpss__blk360_dn2 * assign12920_body161_e18426) + (locals.var_dpss__blk360 * (-((assign12920_body161_e18424 * locals.var_t1__blk353_dn2) / (locals.var_t1__blk353 * locals.var_t1__blk353))))), ((locals.var_dpss__blk360_dn6 * assign12920_body161_e18426) + (locals.var_dpss__blk360 * (-((assign12920_body161_e18424 * locals.var_t1__blk353_dn6) / (locals.var_t1__blk353 * locals.var_t1__blk353))))), ((locals.var_dpss__blk360_dn7 * assign12920_body161_e18426) + (locals.var_dpss__blk360 * (-((assign12920_body161_e18424 * locals.var_t1__blk353_dn7) / (locals.var_t1__blk353 * locals.var_t1__blk353))))), ((locals.var_dpss__blk360_dn10 * assign12920_body161_e18426) + (locals.var_dpss__blk360 * (-((assign12920_body161_e18424 * locals.var_t1__blk353_dn10) / (locals.var_t1__blk353 * locals.var_t1__blk353))))), ((locals.var_dpss__blk360_dn11 * assign12920_body161_e18426) + (locals.var_dpss__blk360 * (-((assign12920_body161_e18424 * locals.var_t1__blk353_dn11) / (locals.var_t1__blk353 * locals.var_t1__blk353))))), ((locals.var_dpss__blk360_dn12 * assign12920_body161_e18426) + (locals.var_dpss__blk360 * (-((assign12920_body161_e18424 * locals.var_t1__blk353_dn12) / (locals.var_t1__blk353 * locals.var_t1__blk353))))), ((locals.var_dpss__blk360_dn17 * assign12920_body161_e18426) + (locals.var_dpss__blk360 * (-((assign12920_body161_e18424 * locals.var_t1__blk353_dn17) / (locals.var_t1__blk353 * locals.var_t1__blk353))))),)
    } else {
        (locals.var_dpss__blk360, locals.var_dpss__blk360_dn0, locals.var_dpss__blk360_dn2, locals.var_dpss__blk360_dn6, locals.var_dpss__blk360_dn7, locals.var_dpss__blk360_dn10, locals.var_dpss__blk360_dn11, locals.var_dpss__blk360_dn12, locals.var_dpss__blk360_dn17,)
    }
};
            locals.var_dpss__blk360 = assign12920_body161_e18429;
            locals.var_dpss__blk360_dn0 = assign12920_body161_e18429_d_n0;
            locals.var_dpss__blk360_dn2 = assign12920_body161_e18429_d_n2;
            locals.var_dpss__blk360_dn6 = assign12920_body161_e18429_d_n6;
            locals.var_dpss__blk360_dn7 = assign12920_body161_e18429_d_n7;
            locals.var_dpss__blk360_dn10 = assign12920_body161_e18429_d_n10;
            locals.var_dpss__blk360_dn11 = assign12920_body161_e18429_d_n11;
            locals.var_dpss__blk360_dn12 = assign12920_body161_e18429_d_n12;
            locals.var_dpss__blk360_dn17 = assign12920_body161_e18429_d_n17;
            let (assign12920_body162_e18447, assign12920_body162_e18447_d_n0, assign12920_body162_e18447_d_n2, assign12920_body162_e18447_d_n6, assign12920_body162_e18447_d_n7, assign12920_body162_e18447_d_n10, assign12920_body162_e18447_d_n11, assign12920_body162_e18447_d_n12, assign12920_body162_e18447_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign12920_body162_e18442: f64 = (0.1 / locals.var_scale_fac);
        let assign12920_body162_e18444: f64 = (assign12920_body162_e18442 / locals.var_t1__blk353);
        let assign12920_body162_e18445: f64 = (locals.var_dpbs__blk361 * assign12920_body162_e18444);
        (assign12920_body162_e18445, ((locals.var_dpbs__blk361_dn0 * assign12920_body162_e18444) + (locals.var_dpbs__blk361 * (-((assign12920_body162_e18442 * locals.var_t1__blk353_dn0) / (locals.var_t1__blk353 * locals.var_t1__blk353))))), ((locals.var_dpbs__blk361_dn2 * assign12920_body162_e18444) + (locals.var_dpbs__blk361 * (-((assign12920_body162_e18442 * locals.var_t1__blk353_dn2) / (locals.var_t1__blk353 * locals.var_t1__blk353))))), ((locals.var_dpbs__blk361_dn6 * assign12920_body162_e18444) + (locals.var_dpbs__blk361 * (-((assign12920_body162_e18442 * locals.var_t1__blk353_dn6) / (locals.var_t1__blk353 * locals.var_t1__blk353))))), ((locals.var_dpbs__blk361_dn7 * assign12920_body162_e18444) + (locals.var_dpbs__blk361 * (-((assign12920_body162_e18442 * locals.var_t1__blk353_dn7) / (locals.var_t1__blk353 * locals.var_t1__blk353))))), ((locals.var_dpbs__blk361_dn10 * assign12920_body162_e18444) + (locals.var_dpbs__blk361 * (-((assign12920_body162_e18442 * locals.var_t1__blk353_dn10) / (locals.var_t1__blk353 * locals.var_t1__blk353))))), ((locals.var_dpbs__blk361_dn11 * assign12920_body162_e18444) + (locals.var_dpbs__blk361 * (-((assign12920_body162_e18442 * locals.var_t1__blk353_dn11) / (locals.var_t1__blk353 * locals.var_t1__blk353))))), ((locals.var_dpbs__blk361_dn12 * assign12920_body162_e18444) + (locals.var_dpbs__blk361 * (-((assign12920_body162_e18442 * locals.var_t1__blk353_dn12) / (locals.var_t1__blk353 * locals.var_t1__blk353))))), ((locals.var_dpbs__blk361_dn17 * assign12920_body162_e18444) + (locals.var_dpbs__blk361 * (-((assign12920_body162_e18442 * locals.var_t1__blk353_dn17) / (locals.var_t1__blk353 * locals.var_t1__blk353))))),)
    } else {
        (locals.var_dpbs__blk361, locals.var_dpbs__blk361_dn0, locals.var_dpbs__blk361_dn2, locals.var_dpbs__blk361_dn6, locals.var_dpbs__blk361_dn7, locals.var_dpbs__blk361_dn10, locals.var_dpbs__blk361_dn11, locals.var_dpbs__blk361_dn12, locals.var_dpbs__blk361_dn17,)
    }
};
            locals.var_dpbs__blk361 = assign12920_body162_e18447;
            locals.var_dpbs__blk361_dn0 = assign12920_body162_e18447_d_n0;
            locals.var_dpbs__blk361_dn2 = assign12920_body162_e18447_d_n2;
            locals.var_dpbs__blk361_dn6 = assign12920_body162_e18447_d_n6;
            locals.var_dpbs__blk361_dn7 = assign12920_body162_e18447_d_n7;
            locals.var_dpbs__blk361_dn10 = assign12920_body162_e18447_d_n10;
            locals.var_dpbs__blk361_dn11 = assign12920_body162_e18447_d_n11;
            locals.var_dpbs__blk361_dn12 = assign12920_body162_e18447_d_n12;
            locals.var_dpbs__blk361_dn17 = assign12920_body162_e18447_d_n17;
            let (assign12920_body163_e18465, assign12920_body163_e18465_d_n0, assign12920_body163_e18465_d_n2, assign12920_body163_e18465_d_n6, assign12920_body163_e18465_d_n7, assign12920_body163_e18465_d_n10, assign12920_body163_e18465_d_n11, assign12920_body163_e18465_d_n12, assign12920_body163_e18465_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign12920_body163_e18460: f64 = (0.1 / locals.var_scale_fac);
        let assign12920_body163_e18462: f64 = (assign12920_body163_e18460 / locals.var_t1__blk353);
        let assign12920_body163_e18463: f64 = (locals.var_dpsb__blk362 * assign12920_body163_e18462);
        (assign12920_body163_e18463, ((locals.var_dpsb__blk362_dn0 * assign12920_body163_e18462) + (locals.var_dpsb__blk362 * (-((assign12920_body163_e18460 * locals.var_t1__blk353_dn0) / (locals.var_t1__blk353 * locals.var_t1__blk353))))), ((locals.var_dpsb__blk362_dn2 * assign12920_body163_e18462) + (locals.var_dpsb__blk362 * (-((assign12920_body163_e18460 * locals.var_t1__blk353_dn2) / (locals.var_t1__blk353 * locals.var_t1__blk353))))), ((locals.var_dpsb__blk362_dn6 * assign12920_body163_e18462) + (locals.var_dpsb__blk362 * (-((assign12920_body163_e18460 * locals.var_t1__blk353_dn6) / (locals.var_t1__blk353 * locals.var_t1__blk353))))), ((locals.var_dpsb__blk362_dn7 * assign12920_body163_e18462) + (locals.var_dpsb__blk362 * (-((assign12920_body163_e18460 * locals.var_t1__blk353_dn7) / (locals.var_t1__blk353 * locals.var_t1__blk353))))), ((locals.var_dpsb__blk362_dn10 * assign12920_body163_e18462) + (locals.var_dpsb__blk362 * (-((assign12920_body163_e18460 * locals.var_t1__blk353_dn10) / (locals.var_t1__blk353 * locals.var_t1__blk353))))), ((locals.var_dpsb__blk362_dn11 * assign12920_body163_e18462) + (locals.var_dpsb__blk362 * (-((assign12920_body163_e18460 * locals.var_t1__blk353_dn11) / (locals.var_t1__blk353 * locals.var_t1__blk353))))), ((locals.var_dpsb__blk362_dn12 * assign12920_body163_e18462) + (locals.var_dpsb__blk362 * (-((assign12920_body163_e18460 * locals.var_t1__blk353_dn12) / (locals.var_t1__blk353 * locals.var_t1__blk353))))), ((locals.var_dpsb__blk362_dn17 * assign12920_body163_e18462) + (locals.var_dpsb__blk362 * (-((assign12920_body163_e18460 * locals.var_t1__blk353_dn17) / (locals.var_t1__blk353 * locals.var_t1__blk353))))),)
    } else {
        (locals.var_dpsb__blk362, locals.var_dpsb__blk362_dn0, locals.var_dpsb__blk362_dn2, locals.var_dpsb__blk362_dn6, locals.var_dpsb__blk362_dn7, locals.var_dpsb__blk362_dn10, locals.var_dpsb__blk362_dn11, locals.var_dpsb__blk362_dn12, locals.var_dpsb__blk362_dn17,)
    }
};
            locals.var_dpsb__blk362 = assign12920_body163_e18465;
            locals.var_dpsb__blk362_dn0 = assign12920_body163_e18465_d_n0;
            locals.var_dpsb__blk362_dn2 = assign12920_body163_e18465_d_n2;
            locals.var_dpsb__blk362_dn6 = assign12920_body163_e18465_d_n6;
            locals.var_dpsb__blk362_dn7 = assign12920_body163_e18465_d_n7;
            locals.var_dpsb__blk362_dn10 = assign12920_body163_e18465_d_n10;
            locals.var_dpsb__blk362_dn11 = assign12920_body163_e18465_d_n11;
            locals.var_dpsb__blk362_dn12 = assign12920_body163_e18465_d_n12;
            locals.var_dpsb__blk362_dn17 = assign12920_body163_e18465_d_n17;
            let (assign12920_body164_e18477, assign12920_body164_e18477_d_n0, assign12920_body164_e18477_d_n2, assign12920_body164_e18477_d_n6, assign12920_body164_e18477_d_n7, assign12920_body164_e18477_d_n10, assign12920_body164_e18477_d_n11, assign12920_body164_e18477_d_n12, assign12920_body164_e18477_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) {
        let assign12920_body164_e18475: f64 = (locals.var_phi_sl_soi + locals.var_dpss__blk360);
        (assign12920_body164_e18475, (locals.var_phi_sl_soi_dn0 + locals.var_dpss__blk360_dn0), (locals.var_phi_sl_soi_dn2 + locals.var_dpss__blk360_dn2), (locals.var_phi_sl_soi_dn6 + locals.var_dpss__blk360_dn6), (locals.var_phi_sl_soi_dn7 + locals.var_dpss__blk360_dn7), (locals.var_phi_sl_soi_dn10 + locals.var_dpss__blk360_dn10), (locals.var_phi_sl_soi_dn11 + locals.var_dpss__blk360_dn11), (locals.var_phi_sl_soi_dn12 + locals.var_dpss__blk360_dn12), (locals.var_phi_sl_soi_dn17 + locals.var_dpss__blk360_dn17),)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
            locals.var_phi_sl_soi = assign12920_body164_e18477;
            locals.var_phi_sl_soi_dn0 = assign12920_body164_e18477_d_n0;
            locals.var_phi_sl_soi_dn2 = assign12920_body164_e18477_d_n2;
            locals.var_phi_sl_soi_dn6 = assign12920_body164_e18477_d_n6;
            locals.var_phi_sl_soi_dn7 = assign12920_body164_e18477_d_n7;
            locals.var_phi_sl_soi_dn10 = assign12920_body164_e18477_d_n10;
            locals.var_phi_sl_soi_dn11 = assign12920_body164_e18477_d_n11;
            locals.var_phi_sl_soi_dn12 = assign12920_body164_e18477_d_n12;
            locals.var_phi_sl_soi_dn17 = assign12920_body164_e18477_d_n17;
            let (assign12920_body165_e18489, assign12920_body165_e18489_d_n0, assign12920_body165_e18489_d_n2, assign12920_body165_e18489_d_n6, assign12920_body165_e18489_d_n7, assign12920_body165_e18489_d_n10, assign12920_body165_e18489_d_n11, assign12920_body165_e18489_d_n12, assign12920_body165_e18489_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) {
        let assign12920_body165_e18487: f64 = (locals.var_phi_bl_soi + locals.var_dpbs__blk361);
        (assign12920_body165_e18487, (locals.var_phi_bl_soi_dn0 + locals.var_dpbs__blk361_dn0), (locals.var_phi_bl_soi_dn2 + locals.var_dpbs__blk361_dn2), (locals.var_phi_bl_soi_dn6 + locals.var_dpbs__blk361_dn6), (locals.var_phi_bl_soi_dn7 + locals.var_dpbs__blk361_dn7), (locals.var_phi_bl_soi_dn10 + locals.var_dpbs__blk361_dn10), (locals.var_phi_bl_soi_dn11 + locals.var_dpbs__blk361_dn11), (locals.var_phi_bl_soi_dn12 + locals.var_dpbs__blk361_dn12), (locals.var_phi_bl_soi_dn17 + locals.var_dpbs__blk361_dn17),)
    } else {
        (locals.var_phi_bl_soi, locals.var_phi_bl_soi_dn0, locals.var_phi_bl_soi_dn2, locals.var_phi_bl_soi_dn6, locals.var_phi_bl_soi_dn7, locals.var_phi_bl_soi_dn10, locals.var_phi_bl_soi_dn11, locals.var_phi_bl_soi_dn12, locals.var_phi_bl_soi_dn17,)
    }
};
            locals.var_phi_bl_soi = assign12920_body165_e18489;
            locals.var_phi_bl_soi_dn0 = assign12920_body165_e18489_d_n0;
            locals.var_phi_bl_soi_dn2 = assign12920_body165_e18489_d_n2;
            locals.var_phi_bl_soi_dn6 = assign12920_body165_e18489_d_n6;
            locals.var_phi_bl_soi_dn7 = assign12920_body165_e18489_d_n7;
            locals.var_phi_bl_soi_dn10 = assign12920_body165_e18489_d_n10;
            locals.var_phi_bl_soi_dn11 = assign12920_body165_e18489_d_n11;
            locals.var_phi_bl_soi_dn12 = assign12920_body165_e18489_d_n12;
            locals.var_phi_bl_soi_dn17 = assign12920_body165_e18489_d_n17;
            let (assign12920_body166_e18501, assign12920_body166_e18501_d_n0, assign12920_body166_e18501_d_n2, assign12920_body166_e18501_d_n6, assign12920_body166_e18501_d_n7, assign12920_body166_e18501_d_n10, assign12920_body166_e18501_d_n11, assign12920_body166_e18501_d_n12, assign12920_body166_e18501_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) {
        let assign12920_body166_e18499: f64 = (locals.var_phi_sl_bulk + locals.var_dpsb__blk362);
        (assign12920_body166_e18499, (locals.var_phi_sl_bulk_dn0 + locals.var_dpsb__blk362_dn0), (locals.var_phi_sl_bulk_dn2 + locals.var_dpsb__blk362_dn2), (locals.var_phi_sl_bulk_dn6 + locals.var_dpsb__blk362_dn6), (locals.var_phi_sl_bulk_dn7 + locals.var_dpsb__blk362_dn7), (locals.var_phi_sl_bulk_dn10 + locals.var_dpsb__blk362_dn10), (locals.var_phi_sl_bulk_dn11 + locals.var_dpsb__blk362_dn11), (locals.var_phi_sl_bulk_dn12 + locals.var_dpsb__blk362_dn12), (locals.var_phi_sl_bulk_dn17 + locals.var_dpsb__blk362_dn17),)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
            locals.var_phi_sl_bulk = assign12920_body166_e18501;
            locals.var_phi_sl_bulk_dn0 = assign12920_body166_e18501_d_n0;
            locals.var_phi_sl_bulk_dn2 = assign12920_body166_e18501_d_n2;
            locals.var_phi_sl_bulk_dn6 = assign12920_body166_e18501_d_n6;
            locals.var_phi_sl_bulk_dn7 = assign12920_body166_e18501_d_n7;
            locals.var_phi_sl_bulk_dn10 = assign12920_body166_e18501_d_n10;
            locals.var_phi_sl_bulk_dn11 = assign12920_body166_e18501_d_n11;
            locals.var_phi_sl_bulk_dn12 = assign12920_body166_e18501_d_n12;
            locals.var_phi_sl_bulk_dn17 = assign12920_body166_e18501_d_n17;
            let (assign12920_body167_e18515,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) {
        let assign12920_body167_e18511: f64 = (5e-12 * locals.var_scale_fac);
        let assign12920_body167_e18513: f64 = assign12920_body167_e18511;
        (assign12920_body167_e18513,)
    } else {
        (locals.var_psconv_3d,)
    }
};
            locals.var_psconv_3d = assign12920_body167_e18515;
            let assign12920_body168_e18518: f64 = if locals.var_t1__blk353 < locals.var_psconv_3d { 1.0 } else { 0.0 };
            locals.var_guard414 = assign12920_body168_e18518;
            let (assign12920_body169_e18530,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard406 == 0.0)) && (locals.var_guard414 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign12920_body169_e18530;
            let (assign12920_body170_e18539,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign12920_body170_e18537: f64 = (locals.var_lp_sl + 1.0);
        (assign12920_body170_e18537,)
    } else {
        (locals.var_lp_sl,)
    }
};
            locals.var_lp_sl = assign12920_body170_e18539;
        }

    }

    pub(super) fn stamp_transient_block_42(
        locals: &mut StampLocals,
    ) {
        let (assign12930_e18551,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let (assign12930_e18549,) = {
            if (locals.var_flg_brk8 > 0.0) {
                (locals.var_flg_brk8,)
            } else {
                (locals.var_lp_sl,)
            }
        };
        (assign12930_e18549,)
    } else {
        (locals.var_lp_sl,)
    }
};
        locals.var_lp_sl = assign12930_e18551;

        let assign12940_e18554: f64 = if locals.var_flg_conv == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard415 = assign12940_e18554;

        let (assign12950_e18563, assign12950_e18563_d_n0, assign12950_e18563_d_n2, assign12950_e18563_d_n6, assign12950_e18563_d_n7, assign12950_e18563_d_n10, assign12950_e18563_d_n11, assign12950_e18563_d_n12, assign12950_e18563_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard415 != 0.0)) {
        (locals.var_phi_sl_soi_ini, locals.var_phi_sl_soi_ini_dn0, locals.var_phi_sl_soi_ini_dn2, locals.var_phi_sl_soi_ini_dn6, locals.var_phi_sl_soi_ini_dn7, locals.var_phi_sl_soi_ini_dn10, locals.var_phi_sl_soi_ini_dn11, locals.var_phi_sl_soi_ini_dn12, locals.var_phi_sl_soi_ini_dn17,)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
        locals.var_phi_sl_soi = assign12950_e18563;
        locals.var_phi_sl_soi_dn0 = assign12950_e18563_d_n0;
        locals.var_phi_sl_soi_dn2 = assign12950_e18563_d_n2;
        locals.var_phi_sl_soi_dn6 = assign12950_e18563_d_n6;
        locals.var_phi_sl_soi_dn7 = assign12950_e18563_d_n7;
        locals.var_phi_sl_soi_dn10 = assign12950_e18563_d_n10;
        locals.var_phi_sl_soi_dn11 = assign12950_e18563_d_n11;
        locals.var_phi_sl_soi_dn12 = assign12950_e18563_d_n12;
        locals.var_phi_sl_soi_dn17 = assign12950_e18563_d_n17;

        let (assign12960_e18572, assign12960_e18572_d_n0, assign12960_e18572_d_n2, assign12960_e18572_d_n6, assign12960_e18572_d_n7, assign12960_e18572_d_n10, assign12960_e18572_d_n11, assign12960_e18572_d_n12, assign12960_e18572_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard415 != 0.0)) {
        (locals.var_phi_bl_soi_ini, locals.var_phi_bl_soi_ini_dn0, locals.var_phi_bl_soi_ini_dn2, locals.var_phi_bl_soi_ini_dn6, locals.var_phi_bl_soi_ini_dn7, locals.var_phi_bl_soi_ini_dn10, locals.var_phi_bl_soi_ini_dn11, locals.var_phi_bl_soi_ini_dn12, locals.var_phi_bl_soi_ini_dn17,)
    } else {
        (locals.var_phi_bl_soi, locals.var_phi_bl_soi_dn0, locals.var_phi_bl_soi_dn2, locals.var_phi_bl_soi_dn6, locals.var_phi_bl_soi_dn7, locals.var_phi_bl_soi_dn10, locals.var_phi_bl_soi_dn11, locals.var_phi_bl_soi_dn12, locals.var_phi_bl_soi_dn17,)
    }
};
        locals.var_phi_bl_soi = assign12960_e18572;
        locals.var_phi_bl_soi_dn0 = assign12960_e18572_d_n0;
        locals.var_phi_bl_soi_dn2 = assign12960_e18572_d_n2;
        locals.var_phi_bl_soi_dn6 = assign12960_e18572_d_n6;
        locals.var_phi_bl_soi_dn7 = assign12960_e18572_d_n7;
        locals.var_phi_bl_soi_dn10 = assign12960_e18572_d_n10;
        locals.var_phi_bl_soi_dn11 = assign12960_e18572_d_n11;
        locals.var_phi_bl_soi_dn12 = assign12960_e18572_d_n12;
        locals.var_phi_bl_soi_dn17 = assign12960_e18572_d_n17;

        let (assign12970_e18581, assign12970_e18581_d_n0, assign12970_e18581_d_n2, assign12970_e18581_d_n6, assign12970_e18581_d_n7, assign12970_e18581_d_n10, assign12970_e18581_d_n11, assign12970_e18581_d_n12, assign12970_e18581_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard415 != 0.0)) {
        (locals.var_phi_sl_bulk_ini, locals.var_phi_sl_bulk_ini_dn0, locals.var_phi_sl_bulk_ini_dn2, locals.var_phi_sl_bulk_ini_dn6, locals.var_phi_sl_bulk_ini_dn7, locals.var_phi_sl_bulk_ini_dn10, locals.var_phi_sl_bulk_ini_dn11, locals.var_phi_sl_bulk_ini_dn12, locals.var_phi_sl_bulk_ini_dn17,)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
        locals.var_phi_sl_bulk = assign12970_e18581;
        locals.var_phi_sl_bulk_dn0 = assign12970_e18581_d_n0;
        locals.var_phi_sl_bulk_dn2 = assign12970_e18581_d_n2;
        locals.var_phi_sl_bulk_dn6 = assign12970_e18581_d_n6;
        locals.var_phi_sl_bulk_dn7 = assign12970_e18581_d_n7;
        locals.var_phi_sl_bulk_dn10 = assign12970_e18581_d_n10;
        locals.var_phi_sl_bulk_dn11 = assign12970_e18581_d_n11;
        locals.var_phi_sl_bulk_dn12 = assign12970_e18581_d_n12;
        locals.var_phi_sl_bulk_dn17 = assign12970_e18581_d_n17;

        let (assign12980_e18588, assign12980_e18588_d_n0, assign12980_e18588_d_n2, assign12980_e18588_d_n6, assign12980_e18588_d_n7, assign12980_e18588_d_n10, assign12980_e18588_d_n11, assign12980_e18588_d_n12, assign12980_e18588_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    }
};
        locals.var_psl = assign12980_e18588;
        locals.var_psl_dn0 = assign12980_e18588_d_n0;
        locals.var_psl_dn2 = assign12980_e18588_d_n2;
        locals.var_psl_dn6 = assign12980_e18588_d_n6;
        locals.var_psl_dn7 = assign12980_e18588_d_n7;
        locals.var_psl_dn10 = assign12980_e18588_d_n10;
        locals.var_psl_dn11 = assign12980_e18588_d_n11;
        locals.var_psl_dn12 = assign12980_e18588_d_n12;
        locals.var_psl_dn17 = assign12980_e18588_d_n17;

        let (assign13000_e18602, assign13000_e18602_d_n0, assign13000_e18602_d_n2, assign13000_e18602_d_n6, assign13000_e18602_d_n7, assign13000_e18602_d_n10, assign13000_e18602_d_n11, assign13000_e18602_d_n12, assign13000_e18602_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn12, locals.var_vdsorg_dn17,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    }
};
        locals.var_vds = assign13000_e18602;
        locals.var_vds_dn0 = assign13000_e18602_d_n0;
        locals.var_vds_dn2 = assign13000_e18602_d_n2;
        locals.var_vds_dn6 = assign13000_e18602_d_n6;
        locals.var_vds_dn7 = assign13000_e18602_d_n7;
        locals.var_vds_dn10 = assign13000_e18602_d_n10;
        locals.var_vds_dn11 = assign13000_e18602_d_n11;
        locals.var_vds_dn12 = assign13000_e18602_d_n12;
        locals.var_vds_dn17 = assign13000_e18602_d_n17;

        let assign13010_e18605: f64 = if locals.var_phi_s0_soi < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard416 = assign13010_e18605;

        let (assign13020_e18614,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard416 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_noqi,)
    }
};
        locals.var_flg_noqi = assign13020_e18614;

        let (assign13030_e18621, assign13030_e18621_d_n0, assign13030_e18621_d_n2, assign13030_e18621_d_n6, assign13030_e18621_d_n7, assign13030_e18621_d_n10, assign13030_e18621_d_n11, assign13030_e18621_d_n12, assign13030_e18621_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    } else {
        (locals.var_ps0s, locals.var_ps0s_dn0, locals.var_ps0s_dn2, locals.var_ps0s_dn6, locals.var_ps0s_dn7, locals.var_ps0s_dn10, locals.var_ps0s_dn11, locals.var_ps0s_dn12, locals.var_ps0s_dn17,)
    }
};
        locals.var_ps0s = assign13030_e18621;
        locals.var_ps0s_dn0 = assign13030_e18621_d_n0;
        locals.var_ps0s_dn2 = assign13030_e18621_d_n2;
        locals.var_ps0s_dn6 = assign13030_e18621_d_n6;
        locals.var_ps0s_dn7 = assign13030_e18621_d_n7;
        locals.var_ps0s_dn10 = assign13030_e18621_d_n10;
        locals.var_ps0s_dn11 = assign13030_e18621_d_n11;
        locals.var_ps0s_dn12 = assign13030_e18621_d_n12;
        locals.var_ps0s_dn17 = assign13030_e18621_d_n17;

        let (assign13040_e18628, assign13040_e18628_d_n0, assign13040_e18628_d_n2, assign13040_e18628_d_n6, assign13040_e18628_d_n7, assign13040_e18628_d_n10, assign13040_e18628_d_n11, assign13040_e18628_d_n12, assign13040_e18628_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    } else {
        (locals.var_psls, locals.var_psls_dn0, locals.var_psls_dn2, locals.var_psls_dn6, locals.var_psls_dn7, locals.var_psls_dn10, locals.var_psls_dn11, locals.var_psls_dn12, locals.var_psls_dn17,)
    }
};
        locals.var_psls = assign13040_e18628;
        locals.var_psls_dn0 = assign13040_e18628_d_n0;
        locals.var_psls_dn2 = assign13040_e18628_d_n2;
        locals.var_psls_dn6 = assign13040_e18628_d_n6;
        locals.var_psls_dn7 = assign13040_e18628_d_n7;
        locals.var_psls_dn10 = assign13040_e18628_d_n10;
        locals.var_psls_dn11 = assign13040_e18628_d_n11;
        locals.var_psls_dn12 = assign13040_e18628_d_n12;
        locals.var_psls_dn17 = assign13040_e18628_d_n17;

        let (assign13050_e18637, assign13050_e18637_d_n0, assign13050_e18637_d_n2, assign13050_e18637_d_n6, assign13050_e18637_d_n7, assign13050_e18637_d_n10, assign13050_e18637_d_n11, assign13050_e18637_d_n12, assign13050_e18637_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign13050_e18635: f64 = (locals.var_psls - locals.var_ps0s);
        (assign13050_e18635, (locals.var_psls_dn0 - locals.var_ps0s_dn0), (locals.var_psls_dn2 - locals.var_ps0s_dn2), (locals.var_psls_dn6 - locals.var_ps0s_dn6), (locals.var_psls_dn7 - locals.var_ps0s_dn7), (locals.var_psls_dn10 - locals.var_ps0s_dn10), (locals.var_psls_dn11 - locals.var_ps0s_dn11), (locals.var_psls_dn12 - locals.var_ps0s_dn12), (locals.var_psls_dn17 - locals.var_ps0s_dn17),)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn12, locals.var_pds_dn17,)
    }
};
        locals.var_pds = assign13050_e18637;
        locals.var_pds_dn0 = assign13050_e18637_d_n0;
        locals.var_pds_dn2 = assign13050_e18637_d_n2;
        locals.var_pds_dn6 = assign13050_e18637_d_n6;
        locals.var_pds_dn7 = assign13050_e18637_d_n7;
        locals.var_pds_dn10 = assign13050_e18637_d_n10;
        locals.var_pds_dn11 = assign13050_e18637_d_n11;
        locals.var_pds_dn12 = assign13050_e18637_d_n12;
        locals.var_pds_dn17 = assign13050_e18637_d_n17;

        let (assign13060_e18644, assign13060_e18644_d_n0, assign13060_e18644_d_n2, assign13060_e18644_d_n6, assign13060_e18644_d_n7, assign13060_e18644_d_n10, assign13060_e18644_d_n11, assign13060_e18644_d_n12, assign13060_e18644_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        (locals.var_phi_s0_bulk, locals.var_phi_s0_bulk_dn0, locals.var_phi_s0_bulk_dn2, locals.var_phi_s0_bulk_dn6, locals.var_phi_s0_bulk_dn7, locals.var_phi_s0_bulk_dn10, locals.var_phi_s0_bulk_dn11, locals.var_phi_s0_bulk_dn12, locals.var_phi_s0_bulk_dn17,)
    } else {
        (locals.var_ps0b, locals.var_ps0b_dn0, locals.var_ps0b_dn2, locals.var_ps0b_dn6, locals.var_ps0b_dn7, locals.var_ps0b_dn10, locals.var_ps0b_dn11, locals.var_ps0b_dn12, locals.var_ps0b_dn17,)
    }
};
        locals.var_ps0b = assign13060_e18644;
        locals.var_ps0b_dn0 = assign13060_e18644_d_n0;
        locals.var_ps0b_dn2 = assign13060_e18644_d_n2;
        locals.var_ps0b_dn6 = assign13060_e18644_d_n6;
        locals.var_ps0b_dn7 = assign13060_e18644_d_n7;
        locals.var_ps0b_dn10 = assign13060_e18644_d_n10;
        locals.var_ps0b_dn11 = assign13060_e18644_d_n11;
        locals.var_ps0b_dn12 = assign13060_e18644_d_n12;
        locals.var_ps0b_dn17 = assign13060_e18644_d_n17;

        let (assign13070_e18653, assign13070_e18653_d_n0, assign13070_e18653_d_n2, assign13070_e18653_d_n6, assign13070_e18653_d_n7, assign13070_e18653_d_n10, assign13070_e18653_d_n11, assign13070_e18653_d_n12, assign13070_e18653_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign13070_e18651: f64 = (locals.var_wdsoi / 1.034943e-10);
        (assign13070_e18651, (locals.var_wdsoi_dn0 / 1.034943e-10), (locals.var_wdsoi_dn2 / 1.034943e-10), (locals.var_wdsoi_dn6 / 1.034943e-10), (locals.var_wdsoi_dn7 / 1.034943e-10), (locals.var_wdsoi_dn10 / 1.034943e-10), (locals.var_wdsoi_dn11 / 1.034943e-10), (locals.var_wdsoi_dn12 / 1.034943e-10), (locals.var_wdsoi_dn17 / 1.034943e-10),)
    } else {
        (locals.var_c_s_inv, locals.var_c_s_inv_dn0, locals.var_c_s_inv_dn2, locals.var_c_s_inv_dn6, locals.var_c_s_inv_dn7, locals.var_c_s_inv_dn10, locals.var_c_s_inv_dn11, locals.var_c_s_inv_dn12, locals.var_c_s_inv_dn17,)
    }
};
        locals.var_c_s_inv = assign13070_e18653;
        locals.var_c_s_inv_dn0 = assign13070_e18653_d_n0;
        locals.var_c_s_inv_dn2 = assign13070_e18653_d_n2;
        locals.var_c_s_inv_dn6 = assign13070_e18653_d_n6;
        locals.var_c_s_inv_dn7 = assign13070_e18653_d_n7;
        locals.var_c_s_inv_dn10 = assign13070_e18653_d_n10;
        locals.var_c_s_inv_dn11 = assign13070_e18653_d_n11;
        locals.var_c_s_inv_dn12 = assign13070_e18653_d_n12;
        locals.var_c_s_inv_dn17 = assign13070_e18653_d_n17;

        let (assign13080_e18674, assign13080_e18674_d_n0, assign13080_e18674_d_n2, assign13080_e18674_d_n6, assign13080_e18674_d_n7, assign13080_e18674_d_n10, assign13080_e18674_d_n11, assign13080_e18674_d_n12, assign13080_e18674_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign13080_e18660: f64 = (locals.var_q_nl - locals.var_q_n0);
        let assign13080_e18664: f64 = (locals.var_q_nl + locals.var_q_n0);
        let assign13080_e18665: f64 = (locals.var_beta * assign13080_e18664);
        let assign13080_e18668: f64 = (locals.var_psls - locals.var_ps0s);
        let assign13080_e18669: f64 = (assign13080_e18665 * assign13080_e18668);
        let assign13080_e18671: f64 = (assign13080_e18669 * 0.5);
        let assign13080_e18672: f64 = (assign13080_e18660 - assign13080_e18671);
        (assign13080_e18672, ((locals.var_q_nl_dn0 - locals.var_q_n0_dn0) - ((((locals.var_beta * (locals.var_q_nl_dn0 + locals.var_q_n0_dn0)) * assign13080_e18668) + (assign13080_e18665 * (locals.var_psls_dn0 - locals.var_ps0s_dn0))) * 0.5)), ((locals.var_q_nl_dn2 - locals.var_q_n0_dn2) - ((((locals.var_beta * (locals.var_q_nl_dn2 + locals.var_q_n0_dn2)) * assign13080_e18668) + (assign13080_e18665 * (locals.var_psls_dn2 - locals.var_ps0s_dn2))) * 0.5)), ((locals.var_q_nl_dn6 - locals.var_q_n0_dn6) - ((((locals.var_beta * (locals.var_q_nl_dn6 + locals.var_q_n0_dn6)) * assign13080_e18668) + (assign13080_e18665 * (locals.var_psls_dn6 - locals.var_ps0s_dn6))) * 0.5)), ((locals.var_q_nl_dn7 - locals.var_q_n0_dn7) - ((((locals.var_beta * (locals.var_q_nl_dn7 + locals.var_q_n0_dn7)) * assign13080_e18668) + (assign13080_e18665 * (locals.var_psls_dn7 - locals.var_ps0s_dn7))) * 0.5)), ((locals.var_q_nl_dn10 - locals.var_q_n0_dn10) - (((((locals.var_beta_dn10 * assign13080_e18664) + (locals.var_beta * (locals.var_q_nl_dn10 + locals.var_q_n0_dn10))) * assign13080_e18668) + (assign13080_e18665 * (locals.var_psls_dn10 - locals.var_ps0s_dn10))) * 0.5)), ((locals.var_q_nl_dn11 - locals.var_q_n0_dn11) - ((((locals.var_beta * (locals.var_q_nl_dn11 + locals.var_q_n0_dn11)) * assign13080_e18668) + (assign13080_e18665 * (locals.var_psls_dn11 - locals.var_ps0s_dn11))) * 0.5)), ((locals.var_q_nl_dn12 - locals.var_q_n0_dn12) - ((((locals.var_beta * (locals.var_q_nl_dn12 + locals.var_q_n0_dn12)) * assign13080_e18668) + (assign13080_e18665 * (locals.var_psls_dn12 - locals.var_ps0s_dn12))) * 0.5)), ((locals.var_q_nl_dn17 - locals.var_q_n0_dn17) - ((((locals.var_beta * (locals.var_q_nl_dn17 + locals.var_q_n0_dn17)) * assign13080_e18668) + (assign13080_e18665 * (locals.var_psls_dn17 - locals.var_ps0s_dn17))) * 0.5)),)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn10, locals.var_idd_dn11, locals.var_idd_dn12, locals.var_idd_dn17,)
    }
};
        locals.var_idd = assign13080_e18674;
        locals.var_idd_dn0 = assign13080_e18674_d_n0;
        locals.var_idd_dn2 = assign13080_e18674_d_n2;
        locals.var_idd_dn6 = assign13080_e18674_d_n6;
        locals.var_idd_dn7 = assign13080_e18674_d_n7;
        locals.var_idd_dn10 = assign13080_e18674_d_n10;
        locals.var_idd_dn11 = assign13080_e18674_d_n11;
        locals.var_idd_dn12 = assign13080_e18674_d_n12;
        locals.var_idd_dn17 = assign13080_e18674_d_n17;

        let assign13090_e18681: f64 = if ((locals.var_idd < 0.0) || (locals.var_vds == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard417 = assign13090_e18681;

        let (assign13100_e18690, assign13100_e18690_d_n0, assign13100_e18690_d_n2, assign13100_e18690_d_n6, assign13100_e18690_d_n7, assign13100_e18690_d_n10, assign13100_e18690_d_n11, assign13100_e18690_d_n12, assign13100_e18690_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard417 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn10, locals.var_idd_dn11, locals.var_idd_dn12, locals.var_idd_dn17,)
    }
};
        locals.var_idd = assign13100_e18690;
        locals.var_idd_dn0 = assign13100_e18690_d_n0;
        locals.var_idd_dn2 = assign13100_e18690_d_n2;
        locals.var_idd_dn6 = assign13100_e18690_d_n6;
        locals.var_idd_dn7 = assign13100_e18690_d_n7;
        locals.var_idd_dn10 = assign13100_e18690_d_n10;
        locals.var_idd_dn11 = assign13100_e18690_d_n11;
        locals.var_idd_dn12 = assign13100_e18690_d_n12;
        locals.var_idd_dn17 = assign13100_e18690_d_n17;

        let (assign13110_e18702, assign13110_e18702_d_n0, assign13110_e18702_d_n2, assign13110_e18702_d_n6, assign13110_e18702_d_n7, assign13110_e18702_d_n10, assign13110_e18702_d_n11, assign13110_e18702_d_n12, assign13110_e18702_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign13110_e18696: f64 = (-0.5);
        let assign13110_e18699: f64 = (locals.var_q_depl + locals.var_q_dep0);
        let assign13110_e18700: f64 = (assign13110_e18696 * assign13110_e18699);
        (assign13110_e18700, (assign13110_e18696 * (locals.var_q_depl_dn0 + locals.var_q_dep0_dn0)), (assign13110_e18696 * (locals.var_q_depl_dn2 + locals.var_q_dep0_dn2)), (assign13110_e18696 * (locals.var_q_depl_dn6 + locals.var_q_dep0_dn6)), (assign13110_e18696 * (locals.var_q_depl_dn7 + locals.var_q_dep0_dn7)), (assign13110_e18696 * (locals.var_q_depl_dn10 + locals.var_q_dep0_dn10)), (assign13110_e18696 * (locals.var_q_depl_dn11 + locals.var_q_dep0_dn11)), (assign13110_e18696 * (locals.var_q_depl_dn12 + locals.var_q_dep0_dn12)), (assign13110_e18696 * (locals.var_q_depl_dn17 + locals.var_q_dep0_dn17)),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign13110_e18702;
        locals.var_qbu_dn0 = assign13110_e18702_d_n0;
        locals.var_qbu_dn2 = assign13110_e18702_d_n2;
        locals.var_qbu_dn6 = assign13110_e18702_d_n6;
        locals.var_qbu_dn7 = assign13110_e18702_d_n7;
        locals.var_qbu_dn10 = assign13110_e18702_d_n10;
        locals.var_qbu_dn11 = assign13110_e18702_d_n11;
        locals.var_qbu_dn12 = assign13110_e18702_d_n12;
        locals.var_qbu_dn17 = assign13110_e18702_d_n17;

        let (assign13120_e18711, assign13120_e18711_d_n0, assign13120_e18711_d_n2, assign13120_e18711_d_n6, assign13120_e18711_d_n7, assign13120_e18711_d_n10, assign13120_e18711_d_n11, assign13120_e18711_d_n12, assign13120_e18711_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign13120_e18709: f64 = (locals.var_phi_sl_soi - locals.var_phi_s0_soi);
        (assign13120_e18709, (locals.var_phi_sl_soi_dn0 - locals.var_phi_s0_soi_dn0), (locals.var_phi_sl_soi_dn2 - locals.var_phi_s0_soi_dn2), (locals.var_phi_sl_soi_dn6 - locals.var_phi_s0_soi_dn6), (locals.var_phi_sl_soi_dn7 - locals.var_phi_s0_soi_dn7), (locals.var_phi_sl_soi_dn10 - locals.var_phi_s0_soi_dn10), (locals.var_phi_sl_soi_dn11 - locals.var_phi_s0_soi_dn11), (locals.var_phi_sl_soi_dn12 - locals.var_phi_s0_soi_dn12), (locals.var_phi_sl_soi_dn17 - locals.var_phi_s0_soi_dn17),)
    } else {
        (locals.var_rrr_p0, locals.var_rrr_p0_dn0, locals.var_rrr_p0_dn2, locals.var_rrr_p0_dn6, locals.var_rrr_p0_dn7, locals.var_rrr_p0_dn10, locals.var_rrr_p0_dn11, locals.var_rrr_p0_dn12, locals.var_rrr_p0_dn17,)
    }
};
        locals.var_rrr_p0 = assign13120_e18711;
        locals.var_rrr_p0_dn0 = assign13120_e18711_d_n0;
        locals.var_rrr_p0_dn2 = assign13120_e18711_d_n2;
        locals.var_rrr_p0_dn6 = assign13120_e18711_d_n6;
        locals.var_rrr_p0_dn7 = assign13120_e18711_d_n7;
        locals.var_rrr_p0_dn10 = assign13120_e18711_d_n10;
        locals.var_rrr_p0_dn11 = assign13120_e18711_d_n11;
        locals.var_rrr_p0_dn12 = assign13120_e18711_d_n12;
        locals.var_rrr_p0_dn17 = assign13120_e18711_d_n17;

        let (assign13130_e18720, assign13130_e18720_d_n0, assign13130_e18720_d_n2, assign13130_e18720_d_n6, assign13130_e18720_d_n7, assign13130_e18720_d_n10, assign13130_e18720_d_n11, assign13130_e18720_d_n12, assign13130_e18720_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign13130_e18718: f64 = (locals.var_rrr_p0 + 5e-12);
        (assign13130_e18718, locals.var_rrr_p0_dn0, locals.var_rrr_p0_dn2, locals.var_rrr_p0_dn6, locals.var_rrr_p0_dn7, locals.var_rrr_p0_dn10, locals.var_rrr_p0_dn11, locals.var_rrr_p0_dn12, locals.var_rrr_p0_dn17,)
    } else {
        (locals.var_rrr_p0, locals.var_rrr_p0_dn0, locals.var_rrr_p0_dn2, locals.var_rrr_p0_dn6, locals.var_rrr_p0_dn7, locals.var_rrr_p0_dn10, locals.var_rrr_p0_dn11, locals.var_rrr_p0_dn12, locals.var_rrr_p0_dn17,)
    }
};
        locals.var_rrr_p0 = assign13130_e18720;
        locals.var_rrr_p0_dn0 = assign13130_e18720_d_n0;
        locals.var_rrr_p0_dn2 = assign13130_e18720_d_n2;
        locals.var_rrr_p0_dn6 = assign13130_e18720_d_n6;
        locals.var_rrr_p0_dn7 = assign13130_e18720_d_n7;
        locals.var_rrr_p0_dn10 = assign13130_e18720_d_n10;
        locals.var_rrr_p0_dn11 = assign13130_e18720_d_n11;
        locals.var_rrr_p0_dn12 = assign13130_e18720_d_n12;
        locals.var_rrr_p0_dn17 = assign13130_e18720_d_n17;

        let (assign13140_e18733, assign13140_e18733_d_n0, assign13140_e18733_d_n2, assign13140_e18733_d_n6, assign13140_e18733_d_n7, assign13140_e18733_d_n10, assign13140_e18733_d_n11, assign13140_e18733_d_n12, assign13140_e18733_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign13140_e18728: f64 = (locals.var_c_box * locals.var_c_s_inv);
        let assign13140_e18730: f64 = (assign13140_e18728 + 1.0);
        let assign13140_e18731: f64 = (locals.var_c_box / assign13140_e18730);
        (assign13140_e18731, (-((locals.var_c_box * (locals.var_c_box * locals.var_c_s_inv_dn0)) / (assign13140_e18730 * assign13140_e18730))), (-((locals.var_c_box * (locals.var_c_box * locals.var_c_s_inv_dn2)) / (assign13140_e18730 * assign13140_e18730))), (-((locals.var_c_box * (locals.var_c_box * locals.var_c_s_inv_dn6)) / (assign13140_e18730 * assign13140_e18730))), (-((locals.var_c_box * (locals.var_c_box * locals.var_c_s_inv_dn7)) / (assign13140_e18730 * assign13140_e18730))), (-((locals.var_c_box * (locals.var_c_box * locals.var_c_s_inv_dn10)) / (assign13140_e18730 * assign13140_e18730))), (-((locals.var_c_box * (locals.var_c_box * locals.var_c_s_inv_dn11)) / (assign13140_e18730 * assign13140_e18730))), (-((locals.var_c_box * (locals.var_c_box * locals.var_c_s_inv_dn12)) / (assign13140_e18730 * assign13140_e18730))), (-((locals.var_c_box * (locals.var_c_box * locals.var_c_s_inv_dn17)) / (assign13140_e18730 * assign13140_e18730))),)
    } else {
        (locals.var_rrr_csoi_cbox, locals.var_rrr_csoi_cbox_dn0, locals.var_rrr_csoi_cbox_dn2, locals.var_rrr_csoi_cbox_dn6, locals.var_rrr_csoi_cbox_dn7, locals.var_rrr_csoi_cbox_dn10, locals.var_rrr_csoi_cbox_dn11, locals.var_rrr_csoi_cbox_dn12, locals.var_rrr_csoi_cbox_dn17,)
    }
};
        locals.var_rrr_csoi_cbox = assign13140_e18733;
        locals.var_rrr_csoi_cbox_dn0 = assign13140_e18733_d_n0;
        locals.var_rrr_csoi_cbox_dn2 = assign13140_e18733_d_n2;
        locals.var_rrr_csoi_cbox_dn6 = assign13140_e18733_d_n6;
        locals.var_rrr_csoi_cbox_dn7 = assign13140_e18733_d_n7;
        locals.var_rrr_csoi_cbox_dn10 = assign13140_e18733_d_n10;
        locals.var_rrr_csoi_cbox_dn11 = assign13140_e18733_d_n11;
        locals.var_rrr_csoi_cbox_dn12 = assign13140_e18733_d_n12;
        locals.var_rrr_csoi_cbox_dn17 = assign13140_e18733_d_n17;

        let (assign13150_e18748, assign13150_e18748_d_n0, assign13150_e18748_d_n2, assign13150_e18748_d_n6, assign13150_e18748_d_n7, assign13150_e18748_d_n10, assign13150_e18748_d_n11, assign13150_e18748_d_n12, assign13150_e18748_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign13150_e18740: f64 = (locals.var_q_sl_bulk * locals.var_q_sl_bulk);
        let assign13150_e18743: f64 = (locals.var_q_s0_bulk * locals.var_q_s0_bulk);
        let assign13150_e18744: f64 = (assign13150_e18740 - assign13150_e18743);
        let assign13150_e18746: f64 = (assign13150_e18744 / locals.var_rrr_csoi_cbox);
        (assign13150_e18746, ((((((locals.var_q_sl_bulk_dn0 * locals.var_q_sl_bulk) + (locals.var_q_sl_bulk * locals.var_q_sl_bulk_dn0)) - ((locals.var_q_s0_bulk_dn0 * locals.var_q_s0_bulk) + (locals.var_q_s0_bulk * locals.var_q_s0_bulk_dn0))) * locals.var_rrr_csoi_cbox) - (assign13150_e18744 * locals.var_rrr_csoi_cbox_dn0)) / (locals.var_rrr_csoi_cbox * locals.var_rrr_csoi_cbox)), ((((((locals.var_q_sl_bulk_dn2 * locals.var_q_sl_bulk) + (locals.var_q_sl_bulk * locals.var_q_sl_bulk_dn2)) - ((locals.var_q_s0_bulk_dn2 * locals.var_q_s0_bulk) + (locals.var_q_s0_bulk * locals.var_q_s0_bulk_dn2))) * locals.var_rrr_csoi_cbox) - (assign13150_e18744 * locals.var_rrr_csoi_cbox_dn2)) / (locals.var_rrr_csoi_cbox * locals.var_rrr_csoi_cbox)), ((((((locals.var_q_sl_bulk_dn6 * locals.var_q_sl_bulk) + (locals.var_q_sl_bulk * locals.var_q_sl_bulk_dn6)) - ((locals.var_q_s0_bulk_dn6 * locals.var_q_s0_bulk) + (locals.var_q_s0_bulk * locals.var_q_s0_bulk_dn6))) * locals.var_rrr_csoi_cbox) - (assign13150_e18744 * locals.var_rrr_csoi_cbox_dn6)) / (locals.var_rrr_csoi_cbox * locals.var_rrr_csoi_cbox)), ((((((locals.var_q_sl_bulk_dn7 * locals.var_q_sl_bulk) + (locals.var_q_sl_bulk * locals.var_q_sl_bulk_dn7)) - ((locals.var_q_s0_bulk_dn7 * locals.var_q_s0_bulk) + (locals.var_q_s0_bulk * locals.var_q_s0_bulk_dn7))) * locals.var_rrr_csoi_cbox) - (assign13150_e18744 * locals.var_rrr_csoi_cbox_dn7)) / (locals.var_rrr_csoi_cbox * locals.var_rrr_csoi_cbox)), ((((((locals.var_q_sl_bulk_dn10 * locals.var_q_sl_bulk) + (locals.var_q_sl_bulk * locals.var_q_sl_bulk_dn10)) - ((locals.var_q_s0_bulk_dn10 * locals.var_q_s0_bulk) + (locals.var_q_s0_bulk * locals.var_q_s0_bulk_dn10))) * locals.var_rrr_csoi_cbox) - (assign13150_e18744 * locals.var_rrr_csoi_cbox_dn10)) / (locals.var_rrr_csoi_cbox * locals.var_rrr_csoi_cbox)), ((((((locals.var_q_sl_bulk_dn11 * locals.var_q_sl_bulk) + (locals.var_q_sl_bulk * locals.var_q_sl_bulk_dn11)) - ((locals.var_q_s0_bulk_dn11 * locals.var_q_s0_bulk) + (locals.var_q_s0_bulk * locals.var_q_s0_bulk_dn11))) * locals.var_rrr_csoi_cbox) - (assign13150_e18744 * locals.var_rrr_csoi_cbox_dn11)) / (locals.var_rrr_csoi_cbox * locals.var_rrr_csoi_cbox)), ((((((locals.var_q_sl_bulk_dn12 * locals.var_q_sl_bulk) + (locals.var_q_sl_bulk * locals.var_q_sl_bulk_dn12)) - ((locals.var_q_s0_bulk_dn12 * locals.var_q_s0_bulk) + (locals.var_q_s0_bulk * locals.var_q_s0_bulk_dn12))) * locals.var_rrr_csoi_cbox) - (assign13150_e18744 * locals.var_rrr_csoi_cbox_dn12)) / (locals.var_rrr_csoi_cbox * locals.var_rrr_csoi_cbox)), ((((((locals.var_q_sl_bulk_dn17 * locals.var_q_sl_bulk) + (locals.var_q_sl_bulk * locals.var_q_sl_bulk_dn17)) - ((locals.var_q_s0_bulk_dn17 * locals.var_q_s0_bulk) + (locals.var_q_s0_bulk * locals.var_q_s0_bulk_dn17))) * locals.var_rrr_csoi_cbox) - (assign13150_e18744 * locals.var_rrr_csoi_cbox_dn17)) / (locals.var_rrr_csoi_cbox * locals.var_rrr_csoi_cbox)),)
    } else {
        (locals.var_rrr_b, locals.var_rrr_b_dn0, locals.var_rrr_b_dn2, locals.var_rrr_b_dn6, locals.var_rrr_b_dn7, locals.var_rrr_b_dn10, locals.var_rrr_b_dn11, locals.var_rrr_b_dn12, locals.var_rrr_b_dn17,)
    }
};
        locals.var_rrr_b = assign13150_e18748;
        locals.var_rrr_b_dn0 = assign13150_e18748_d_n0;
        locals.var_rrr_b_dn2 = assign13150_e18748_d_n2;
        locals.var_rrr_b_dn6 = assign13150_e18748_d_n6;
        locals.var_rrr_b_dn7 = assign13150_e18748_d_n7;
        locals.var_rrr_b_dn10 = assign13150_e18748_d_n10;
        locals.var_rrr_b_dn11 = assign13150_e18748_d_n11;
        locals.var_rrr_b_dn12 = assign13150_e18748_d_n12;
        locals.var_rrr_b_dn17 = assign13150_e18748_d_n17;

        let assign13160_e18750: f64 = (-locals.var_rrr_b);
        let assign13160_e18754: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13160_e18755: f64 = assign13160_e18754;
        let assign13160_e18759: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13160_e18762: f64 = if ((assign13160_e18750 < assign13160_e18755) && (assign13160_e18759 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard418 = assign13160_e18762;

        let (assign13170_e18778, assign13170_e18778_d_n0, assign13170_e18778_d_n2, assign13170_e18778_d_n6, assign13170_e18778_d_n7, assign13170_e18778_d_n10, assign13170_e18778_d_n11, assign13170_e18778_d_n12, assign13170_e18778_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign13170_e18772: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13170_e18773: f64 = assign13170_e18772;
        let assign13170_e18775: f64 = (-locals.var_rrr_b);
        let assign13170_e18776: f64 = (assign13170_e18773 - assign13170_e18775);
        (assign13170_e18776, ((locals.var_q_fd_soi_dn0 * 1e-5) - (-locals.var_rrr_b_dn0)), ((locals.var_q_fd_soi_dn2 * 1e-5) - (-locals.var_rrr_b_dn2)), ((locals.var_q_fd_soi_dn6 * 1e-5) - (-locals.var_rrr_b_dn6)), ((locals.var_q_fd_soi_dn7 * 1e-5) - (-locals.var_rrr_b_dn7)), ((locals.var_q_fd_soi_dn10 * 1e-5) - (-locals.var_rrr_b_dn10)), ((locals.var_q_fd_soi_dn11 * 1e-5) - (-locals.var_rrr_b_dn11)), ((locals.var_q_fd_soi_dn12 * 1e-5) - (-locals.var_rrr_b_dn12)), ((locals.var_q_fd_soi_dn17 * 1e-5) - (-locals.var_rrr_b_dn17)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign13170_e18778;
        locals.var_tmf1_dn0 = assign13170_e18778_d_n0;
        locals.var_tmf1_dn2 = assign13170_e18778_d_n2;
        locals.var_tmf1_dn6 = assign13170_e18778_d_n6;
        locals.var_tmf1_dn7 = assign13170_e18778_d_n7;
        locals.var_tmf1_dn10 = assign13170_e18778_d_n10;
        locals.var_tmf1_dn11 = assign13170_e18778_d_n11;
        locals.var_tmf1_dn12 = assign13170_e18778_d_n12;
        locals.var_tmf1_dn17 = assign13170_e18778_d_n17;

        let (assign13180_e18789, assign13180_e18789_d_n0, assign13180_e18789_d_n2, assign13180_e18789_d_n6, assign13180_e18789_d_n7, assign13180_e18789_d_n10, assign13180_e18789_d_n11, assign13180_e18789_d_n12, assign13180_e18789_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign13180_e18787: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign13180_e18787, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign13180_e18789;
        locals.var_x2_dn0 = assign13180_e18789_d_n0;
        locals.var_x2_dn2 = assign13180_e18789_d_n2;
        locals.var_x2_dn6 = assign13180_e18789_d_n6;
        locals.var_x2_dn7 = assign13180_e18789_d_n7;
        locals.var_x2_dn10 = assign13180_e18789_d_n10;
        locals.var_x2_dn11 = assign13180_e18789_d_n11;
        locals.var_x2_dn12 = assign13180_e18789_d_n12;
        locals.var_x2_dn17 = assign13180_e18789_d_n17;

        let (assign13190_e18804, assign13190_e18804_d_n0, assign13190_e18804_d_n2, assign13190_e18804_d_n6, assign13190_e18804_d_n7, assign13190_e18804_d_n10, assign13190_e18804_d_n11, assign13190_e18804_d_n12, assign13190_e18804_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign13190_e18798: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13190_e18801: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13190_e18802: f64 = (assign13190_e18798 * assign13190_e18801);
        (assign13190_e18802, (((locals.var_q_fd_soi_dn0 * 1e-5) * assign13190_e18801) + (assign13190_e18798 * (locals.var_q_fd_soi_dn0 * 1e-5))), (((locals.var_q_fd_soi_dn2 * 1e-5) * assign13190_e18801) + (assign13190_e18798 * (locals.var_q_fd_soi_dn2 * 1e-5))), (((locals.var_q_fd_soi_dn6 * 1e-5) * assign13190_e18801) + (assign13190_e18798 * (locals.var_q_fd_soi_dn6 * 1e-5))), (((locals.var_q_fd_soi_dn7 * 1e-5) * assign13190_e18801) + (assign13190_e18798 * (locals.var_q_fd_soi_dn7 * 1e-5))), (((locals.var_q_fd_soi_dn10 * 1e-5) * assign13190_e18801) + (assign13190_e18798 * (locals.var_q_fd_soi_dn10 * 1e-5))), (((locals.var_q_fd_soi_dn11 * 1e-5) * assign13190_e18801) + (assign13190_e18798 * (locals.var_q_fd_soi_dn11 * 1e-5))), (((locals.var_q_fd_soi_dn12 * 1e-5) * assign13190_e18801) + (assign13190_e18798 * (locals.var_q_fd_soi_dn12 * 1e-5))), (((locals.var_q_fd_soi_dn17 * 1e-5) * assign13190_e18801) + (assign13190_e18798 * (locals.var_q_fd_soi_dn17 * 1e-5))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign13190_e18804;
        locals.var_xmax2_dn0 = assign13190_e18804_d_n0;
        locals.var_xmax2_dn2 = assign13190_e18804_d_n2;
        locals.var_xmax2_dn6 = assign13190_e18804_d_n6;
        locals.var_xmax2_dn7 = assign13190_e18804_d_n7;
        locals.var_xmax2_dn10 = assign13190_e18804_d_n10;
        locals.var_xmax2_dn11 = assign13190_e18804_d_n11;
        locals.var_xmax2_dn12 = assign13190_e18804_d_n12;
        locals.var_xmax2_dn17 = assign13190_e18804_d_n17;

        let (assign13200_e18813, assign13200_e18813_d_n0, assign13200_e18813_d_n2, assign13200_e18813_d_n6, assign13200_e18813_d_n7, assign13200_e18813_d_n10, assign13200_e18813_d_n11, assign13200_e18813_d_n12, assign13200_e18813_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard418 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13200_e18813;
        locals.var_xp_dn0 = assign13200_e18813_d_n0;
        locals.var_xp_dn2 = assign13200_e18813_d_n2;
        locals.var_xp_dn6 = assign13200_e18813_d_n6;
        locals.var_xp_dn7 = assign13200_e18813_d_n7;
        locals.var_xp_dn10 = assign13200_e18813_d_n10;
        locals.var_xp_dn11 = assign13200_e18813_d_n11;
        locals.var_xp_dn12 = assign13200_e18813_d_n12;
        locals.var_xp_dn17 = assign13200_e18813_d_n17;

        let (assign13210_e18822, assign13210_e18822_d_n0, assign13210_e18822_d_n2, assign13210_e18822_d_n6, assign13210_e18822_d_n7, assign13210_e18822_d_n10, assign13210_e18822_d_n11, assign13210_e18822_d_n12, assign13210_e18822_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard418 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13210_e18822;
        locals.var_xmp_dn0 = assign13210_e18822_d_n0;
        locals.var_xmp_dn2 = assign13210_e18822_d_n2;
        locals.var_xmp_dn6 = assign13210_e18822_d_n6;
        locals.var_xmp_dn7 = assign13210_e18822_d_n7;
        locals.var_xmp_dn10 = assign13210_e18822_d_n10;
        locals.var_xmp_dn11 = assign13210_e18822_d_n11;
        locals.var_xmp_dn12 = assign13210_e18822_d_n12;
        locals.var_xmp_dn17 = assign13210_e18822_d_n17;

        let (assign13220_e18831,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard418 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign13220_e18831;

        let (assign13230_e18840,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard418 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13230_e18840;

        let (assign13240_e18849, assign13240_e18849_d_n0, assign13240_e18849_d_n2, assign13240_e18849_d_n6, assign13240_e18849_d_n7, assign13240_e18849_d_n10, assign13240_e18849_d_n11, assign13240_e18849_d_n12, assign13240_e18849_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard418 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign13240_e18849;
        locals.var_arg_dn0 = assign13240_e18849_d_n0;
        locals.var_arg_dn2 = assign13240_e18849_d_n2;
        locals.var_arg_dn6 = assign13240_e18849_d_n6;
        locals.var_arg_dn7 = assign13240_e18849_d_n7;
        locals.var_arg_dn10 = assign13240_e18849_d_n10;
        locals.var_arg_dn11 = assign13240_e18849_d_n11;
        locals.var_arg_dn12 = assign13240_e18849_d_n12;
        locals.var_arg_dn17 = assign13240_e18849_d_n17;

        let (assign13250_e18858, assign13250_e18858_d_n0, assign13250_e18858_d_n2, assign13250_e18858_d_n6, assign13250_e18858_d_n7, assign13250_e18858_d_n10, assign13250_e18858_d_n11, assign13250_e18858_d_n12, assign13250_e18858_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard418 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13250_e18858;
        locals.var_dnm_dn0 = assign13250_e18858_d_n0;
        locals.var_dnm_dn2 = assign13250_e18858_d_n2;
        locals.var_dnm_dn6 = assign13250_e18858_d_n6;
        locals.var_dnm_dn7 = assign13250_e18858_d_n7;
        locals.var_dnm_dn10 = assign13250_e18858_d_n10;
        locals.var_dnm_dn11 = assign13250_e18858_d_n11;
        locals.var_dnm_dn12 = assign13250_e18858_d_n12;
        locals.var_dnm_dn17 = assign13250_e18858_d_n17;

    }

    pub(super) fn stamp_transient_block_43(
        locals: &mut StampLocals,
    ) {
        let (assign13260_e18869, assign13260_e18869_d_n0, assign13260_e18869_d_n2, assign13260_e18869_d_n6, assign13260_e18869_d_n7, assign13260_e18869_d_n10, assign13260_e18869_d_n11, assign13260_e18869_d_n12, assign13260_e18869_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign13260_e18867: f64 = (locals.var_xp * locals.var_x2);
        (assign13260_e18867, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13260_e18869;
        locals.var_xp_dn0 = assign13260_e18869_d_n0;
        locals.var_xp_dn2 = assign13260_e18869_d_n2;
        locals.var_xp_dn6 = assign13260_e18869_d_n6;
        locals.var_xp_dn7 = assign13260_e18869_d_n7;
        locals.var_xp_dn10 = assign13260_e18869_d_n10;
        locals.var_xp_dn11 = assign13260_e18869_d_n11;
        locals.var_xp_dn12 = assign13260_e18869_d_n12;
        locals.var_xp_dn17 = assign13260_e18869_d_n17;

        let (assign13270_e18880, assign13270_e18880_d_n0, assign13270_e18880_d_n2, assign13270_e18880_d_n6, assign13270_e18880_d_n7, assign13270_e18880_d_n10, assign13270_e18880_d_n11, assign13270_e18880_d_n12, assign13270_e18880_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign13270_e18878: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign13270_e18878, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13270_e18880;
        locals.var_xmp_dn0 = assign13270_e18880_d_n0;
        locals.var_xmp_dn2 = assign13270_e18880_d_n2;
        locals.var_xmp_dn6 = assign13270_e18880_d_n6;
        locals.var_xmp_dn7 = assign13270_e18880_d_n7;
        locals.var_xmp_dn10 = assign13270_e18880_d_n10;
        locals.var_xmp_dn11 = assign13270_e18880_d_n11;
        locals.var_xmp_dn12 = assign13270_e18880_d_n12;
        locals.var_xmp_dn17 = assign13270_e18880_d_n17;

        let (assign13280_e18891, assign13280_e18891_d_n0, assign13280_e18891_d_n2, assign13280_e18891_d_n6, assign13280_e18891_d_n7, assign13280_e18891_d_n10, assign13280_e18891_d_n11, assign13280_e18891_d_n12, assign13280_e18891_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign13280_e18889: f64 = (locals.var_xp * locals.var_x2);
        (assign13280_e18889, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13280_e18891;
        locals.var_xp_dn0 = assign13280_e18891_d_n0;
        locals.var_xp_dn2 = assign13280_e18891_d_n2;
        locals.var_xp_dn6 = assign13280_e18891_d_n6;
        locals.var_xp_dn7 = assign13280_e18891_d_n7;
        locals.var_xp_dn10 = assign13280_e18891_d_n10;
        locals.var_xp_dn11 = assign13280_e18891_d_n11;
        locals.var_xp_dn12 = assign13280_e18891_d_n12;
        locals.var_xp_dn17 = assign13280_e18891_d_n17;

        let (assign13290_e18902, assign13290_e18902_d_n0, assign13290_e18902_d_n2, assign13290_e18902_d_n6, assign13290_e18902_d_n7, assign13290_e18902_d_n10, assign13290_e18902_d_n11, assign13290_e18902_d_n12, assign13290_e18902_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign13290_e18900: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign13290_e18900, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13290_e18902;
        locals.var_xmp_dn0 = assign13290_e18902_d_n0;
        locals.var_xmp_dn2 = assign13290_e18902_d_n2;
        locals.var_xmp_dn6 = assign13290_e18902_d_n6;
        locals.var_xmp_dn7 = assign13290_e18902_d_n7;
        locals.var_xmp_dn10 = assign13290_e18902_d_n10;
        locals.var_xmp_dn11 = assign13290_e18902_d_n11;
        locals.var_xmp_dn12 = assign13290_e18902_d_n12;
        locals.var_xmp_dn17 = assign13290_e18902_d_n17;

        let (assign13300_e18913, assign13300_e18913_d_n0, assign13300_e18913_d_n2, assign13300_e18913_d_n6, assign13300_e18913_d_n7, assign13300_e18913_d_n10, assign13300_e18913_d_n11, assign13300_e18913_d_n12, assign13300_e18913_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign13300_e18911: f64 = (locals.var_xp + locals.var_xmp);
        (assign13300_e18911, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign13300_e18913;
        locals.var_arg_dn0 = assign13300_e18913_d_n0;
        locals.var_arg_dn2 = assign13300_e18913_d_n2;
        locals.var_arg_dn6 = assign13300_e18913_d_n6;
        locals.var_arg_dn7 = assign13300_e18913_d_n7;
        locals.var_arg_dn10 = assign13300_e18913_d_n10;
        locals.var_arg_dn11 = assign13300_e18913_d_n11;
        locals.var_arg_dn12 = assign13300_e18913_d_n12;
        locals.var_arg_dn17 = assign13300_e18913_d_n17;

        let (assign13310_e18922, assign13310_e18922_d_n0, assign13310_e18922_d_n2, assign13310_e18922_d_n6, assign13310_e18922_d_n7, assign13310_e18922_d_n10, assign13310_e18922_d_n11, assign13310_e18922_d_n12, assign13310_e18922_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard418 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13310_e18922;
        locals.var_dnm_dn0 = assign13310_e18922_d_n0;
        locals.var_dnm_dn2 = assign13310_e18922_d_n2;
        locals.var_dnm_dn6 = assign13310_e18922_d_n6;
        locals.var_dnm_dn7 = assign13310_e18922_d_n7;
        locals.var_dnm_dn10 = assign13310_e18922_d_n10;
        locals.var_dnm_dn11 = assign13310_e18922_d_n11;
        locals.var_dnm_dn12 = assign13310_e18922_d_n12;
        locals.var_dnm_dn17 = assign13310_e18922_d_n17;

        let assign13320_e18937: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard419 = assign13320_e18937;

        let assign13330_e18940: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard420 = assign13330_e18940;

        let (assign13340_e18953,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard418 != 0.0)) && (locals.var_guard419 != 0.0)) && (locals.var_guard420 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13340_e18953;

        let assign13350_e18956: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard421 = assign13350_e18956;

        let (assign13360_e18972,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard418 != 0.0)) && (locals.var_guard419 != 0.0)) && (locals.var_guard420 == 0.0)) && (locals.var_guard421 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13360_e18972;

        let assign13370_e18975: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard422 = assign13370_e18975;

        let (assign13380_e18994,) = {
    if (((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard418 != 0.0)) && (locals.var_guard419 != 0.0)) && (locals.var_guard420 == 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard422 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13380_e18994;

        let assign13390_e18997: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard423 = assign13390_e18997;

        let (assign13400_e19019,) = {
    if ((((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard418 != 0.0)) && (locals.var_guard419 != 0.0)) && (locals.var_guard420 == 0.0)) && (locals.var_guard421 == 0.0)) && (locals.var_guard422 == 0.0)) && (locals.var_guard423 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13400_e19019;

        let (assign13410_e19030,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard418 != 0.0)) && (locals.var_guard419 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign13410_e19030;

        let mut assign13420_loop_guard: usize = 0;
        while {
            let assign13420_cond_e19042: f64 = if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard418 != 0.0)) && (locals.var_guard419 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign13420_cond_e19042 != 0.0
        } {
            assign13420_loop_guard += 1;
            assert!(assign13420_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign13420_body0_e19054, assign13420_body0_e19054_d_n0, assign13420_body0_e19054_d_n2, assign13420_body0_e19054_d_n6, assign13420_body0_e19054_d_n7, assign13420_body0_e19054_d_n10, assign13420_body0_e19054_d_n11, assign13420_body0_e19054_d_n12, assign13420_body0_e19054_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard418 != 0.0)) && (locals.var_guard419 != 0.0)) {
        let assign13420_body0_e19052: f64 = (locals.var_dnm).sqrt();
        (assign13420_body0_e19052, (locals.var_dnm_dn0 / (2.0 * assign13420_body0_e19052)), (locals.var_dnm_dn2 / (2.0 * assign13420_body0_e19052)), (locals.var_dnm_dn6 / (2.0 * assign13420_body0_e19052)), (locals.var_dnm_dn7 / (2.0 * assign13420_body0_e19052)), (locals.var_dnm_dn10 / (2.0 * assign13420_body0_e19052)), (locals.var_dnm_dn11 / (2.0 * assign13420_body0_e19052)), (locals.var_dnm_dn12 / (2.0 * assign13420_body0_e19052)), (locals.var_dnm_dn17 / (2.0 * assign13420_body0_e19052)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign13420_body0_e19054;
            locals.var_dnm_dn0 = assign13420_body0_e19054_d_n0;
            locals.var_dnm_dn2 = assign13420_body0_e19054_d_n2;
            locals.var_dnm_dn6 = assign13420_body0_e19054_d_n6;
            locals.var_dnm_dn7 = assign13420_body0_e19054_d_n7;
            locals.var_dnm_dn10 = assign13420_body0_e19054_d_n10;
            locals.var_dnm_dn11 = assign13420_body0_e19054_d_n11;
            locals.var_dnm_dn12 = assign13420_body0_e19054_d_n12;
            locals.var_dnm_dn17 = assign13420_body0_e19054_d_n17;
            let (assign13420_body1_e19067,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard418 != 0.0)) && (locals.var_guard419 != 0.0)) {
        let assign13420_body1_e19065: f64 = (locals.var_m0 + 1.0);
        (assign13420_body1_e19065,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign13420_body1_e19067;
        }

        let (assign13430_e19085, assign13430_e19085_d_n0, assign13430_e19085_d_n2, assign13430_e19085_d_n6, assign13430_e19085_d_n7, assign13430_e19085_d_n10, assign13430_e19085_d_n11, assign13430_e19085_d_n12, assign13430_e19085_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard418 != 0.0)) && (locals.var_guard419 == 0.0)) {
        let assign13430_e19081: f64 = (2.0 * 2.0);
        let assign13430_e19082: f64 = (1.0 / assign13430_e19081);
        let assign13430_e19083: f64 = (locals.var_dnm).powf(assign13430_e19082);
        (assign13430_e19083, if 0.0 == 0.0 && ((assign13430_e19082) as f64).is_finite() && ((assign13430_e19082) as f64).fract() == 0.0 { if assign13430_e19082 == 0.0 { 0.0 } else { (assign13430_e19082 * ((locals.var_dnm).powf(assign13430_e19082 - 1.0) * locals.var_dnm_dn0)) } } else { (assign13430_e19083 * (assign13430_e19082 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13430_e19082) as f64).is_finite() && ((assign13430_e19082) as f64).fract() == 0.0 { if assign13430_e19082 == 0.0 { 0.0 } else { (assign13430_e19082 * ((locals.var_dnm).powf(assign13430_e19082 - 1.0) * locals.var_dnm_dn2)) } } else { (assign13430_e19083 * (assign13430_e19082 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13430_e19082) as f64).is_finite() && ((assign13430_e19082) as f64).fract() == 0.0 { if assign13430_e19082 == 0.0 { 0.0 } else { (assign13430_e19082 * ((locals.var_dnm).powf(assign13430_e19082 - 1.0) * locals.var_dnm_dn6)) } } else { (assign13430_e19083 * (assign13430_e19082 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13430_e19082) as f64).is_finite() && ((assign13430_e19082) as f64).fract() == 0.0 { if assign13430_e19082 == 0.0 { 0.0 } else { (assign13430_e19082 * ((locals.var_dnm).powf(assign13430_e19082 - 1.0) * locals.var_dnm_dn7)) } } else { (assign13430_e19083 * (assign13430_e19082 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13430_e19082) as f64).is_finite() && ((assign13430_e19082) as f64).fract() == 0.0 { if assign13430_e19082 == 0.0 { 0.0 } else { (assign13430_e19082 * ((locals.var_dnm).powf(assign13430_e19082 - 1.0) * locals.var_dnm_dn10)) } } else { (assign13430_e19083 * (assign13430_e19082 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13430_e19082) as f64).is_finite() && ((assign13430_e19082) as f64).fract() == 0.0 { if assign13430_e19082 == 0.0 { 0.0 } else { (assign13430_e19082 * ((locals.var_dnm).powf(assign13430_e19082 - 1.0) * locals.var_dnm_dn11)) } } else { (assign13430_e19083 * (assign13430_e19082 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13430_e19082) as f64).is_finite() && ((assign13430_e19082) as f64).fract() == 0.0 { if assign13430_e19082 == 0.0 { 0.0 } else { (assign13430_e19082 * ((locals.var_dnm).powf(assign13430_e19082 - 1.0) * locals.var_dnm_dn12)) } } else { (assign13430_e19083 * (assign13430_e19082 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13430_e19082) as f64).is_finite() && ((assign13430_e19082) as f64).fract() == 0.0 { if assign13430_e19082 == 0.0 { 0.0 } else { (assign13430_e19082 * ((locals.var_dnm).powf(assign13430_e19082 - 1.0) * locals.var_dnm_dn17)) } } else { (assign13430_e19083 * (assign13430_e19082 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13430_e19085;
        locals.var_dnm_dn0 = assign13430_e19085_d_n0;
        locals.var_dnm_dn2 = assign13430_e19085_d_n2;
        locals.var_dnm_dn6 = assign13430_e19085_d_n6;
        locals.var_dnm_dn7 = assign13430_e19085_d_n7;
        locals.var_dnm_dn10 = assign13430_e19085_d_n10;
        locals.var_dnm_dn11 = assign13430_e19085_d_n11;
        locals.var_dnm_dn12 = assign13430_e19085_d_n12;
        locals.var_dnm_dn17 = assign13430_e19085_d_n17;

        let (assign13440_e19096, assign13440_e19096_d_n0, assign13440_e19096_d_n2, assign13440_e19096_d_n6, assign13440_e19096_d_n7, assign13440_e19096_d_n10, assign13440_e19096_d_n11, assign13440_e19096_d_n12, assign13440_e19096_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign13440_e19094: f64 = (1.0 / locals.var_dnm);
        (assign13440_e19094, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13440_e19096;
        locals.var_dnm_dn0 = assign13440_e19096_d_n0;
        locals.var_dnm_dn2 = assign13440_e19096_d_n2;
        locals.var_dnm_dn6 = assign13440_e19096_d_n6;
        locals.var_dnm_dn7 = assign13440_e19096_d_n7;
        locals.var_dnm_dn10 = assign13440_e19096_d_n10;
        locals.var_dnm_dn11 = assign13440_e19096_d_n11;
        locals.var_dnm_dn12 = assign13440_e19096_d_n12;
        locals.var_dnm_dn17 = assign13440_e19096_d_n17;

        let (assign13450_e19111, assign13450_e19111_d_n0, assign13450_e19111_d_n2, assign13450_e19111_d_n6, assign13450_e19111_d_n7, assign13450_e19111_d_n10, assign13450_e19111_d_n11, assign13450_e19111_d_n12, assign13450_e19111_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign13450_e19106: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13450_e19107: f64 = (locals.var_tmf1 * assign13450_e19106);
        let assign13450_e19109: f64 = (assign13450_e19107 * locals.var_dnm);
        (assign13450_e19109, ((((locals.var_tmf1_dn0 * assign13450_e19106) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn0 * 1e-5))) * locals.var_dnm) + (assign13450_e19107 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign13450_e19106) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn2 * 1e-5))) * locals.var_dnm) + (assign13450_e19107 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn6 * assign13450_e19106) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn6 * 1e-5))) * locals.var_dnm) + (assign13450_e19107 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign13450_e19106) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn7 * 1e-5))) * locals.var_dnm) + (assign13450_e19107 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn10 * assign13450_e19106) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn10 * 1e-5))) * locals.var_dnm) + (assign13450_e19107 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * assign13450_e19106) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn11 * 1e-5))) * locals.var_dnm) + (assign13450_e19107 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn12 * assign13450_e19106) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn12 * 1e-5))) * locals.var_dnm) + (assign13450_e19107 * locals.var_dnm_dn12)), ((((locals.var_tmf1_dn17 * assign13450_e19106) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn17 * 1e-5))) * locals.var_dnm) + (assign13450_e19107 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign13450_e19111;
        locals.var_tmf0_dn0 = assign13450_e19111_d_n0;
        locals.var_tmf0_dn2 = assign13450_e19111_d_n2;
        locals.var_tmf0_dn6 = assign13450_e19111_d_n6;
        locals.var_tmf0_dn7 = assign13450_e19111_d_n7;
        locals.var_tmf0_dn10 = assign13450_e19111_d_n10;
        locals.var_tmf0_dn11 = assign13450_e19111_d_n11;
        locals.var_tmf0_dn12 = assign13450_e19111_d_n12;
        locals.var_tmf0_dn17 = assign13450_e19111_d_n17;

        let (assign13460_e19126, assign13460_e19126_d_n0, assign13460_e19126_d_n2, assign13460_e19126_d_n6, assign13460_e19126_d_n7, assign13460_e19126_d_n10, assign13460_e19126_d_n11, assign13460_e19126_d_n12, assign13460_e19126_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign13460_e19121: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13460_e19122: f64 = assign13460_e19121;
        let assign13460_e19124: f64 = (assign13460_e19122 - locals.var_tmf0);
        (assign13460_e19124, ((locals.var_q_fd_soi_dn0 * 1e-5) - locals.var_tmf0_dn0), ((locals.var_q_fd_soi_dn2 * 1e-5) - locals.var_tmf0_dn2), ((locals.var_q_fd_soi_dn6 * 1e-5) - locals.var_tmf0_dn6), ((locals.var_q_fd_soi_dn7 * 1e-5) - locals.var_tmf0_dn7), ((locals.var_q_fd_soi_dn10 * 1e-5) - locals.var_tmf0_dn10), ((locals.var_q_fd_soi_dn11 * 1e-5) - locals.var_tmf0_dn11), ((locals.var_q_fd_soi_dn12 * 1e-5) - locals.var_tmf0_dn12), ((locals.var_q_fd_soi_dn17 * 1e-5) - locals.var_tmf0_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign13460_e19126;
        locals.var_t1_dn0 = assign13460_e19126_d_n0;
        locals.var_t1_dn2 = assign13460_e19126_d_n2;
        locals.var_t1_dn6 = assign13460_e19126_d_n6;
        locals.var_t1_dn7 = assign13460_e19126_d_n7;
        locals.var_t1_dn10 = assign13460_e19126_d_n10;
        locals.var_t1_dn11 = assign13460_e19126_d_n11;
        locals.var_t1_dn12 = assign13460_e19126_d_n12;
        locals.var_t1_dn17 = assign13460_e19126_d_n17;

        let (assign13470_e19137, assign13470_e19137_d_n0, assign13470_e19137_d_n2, assign13470_e19137_d_n6, assign13470_e19137_d_n7, assign13470_e19137_d_n10, assign13470_e19137_d_n11, assign13470_e19137_d_n12, assign13470_e19137_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard418 == 0.0)) {
        let assign13470_e19135: f64 = (-locals.var_rrr_b);
        (assign13470_e19135, (-locals.var_rrr_b_dn0), (-locals.var_rrr_b_dn2), (-locals.var_rrr_b_dn6), (-locals.var_rrr_b_dn7), (-locals.var_rrr_b_dn10), (-locals.var_rrr_b_dn11), (-locals.var_rrr_b_dn12), (-locals.var_rrr_b_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign13470_e19137;
        locals.var_t1_dn0 = assign13470_e19137_d_n0;
        locals.var_t1_dn2 = assign13470_e19137_d_n2;
        locals.var_t1_dn6 = assign13470_e19137_d_n6;
        locals.var_t1_dn7 = assign13470_e19137_d_n7;
        locals.var_t1_dn10 = assign13470_e19137_d_n10;
        locals.var_t1_dn11 = assign13470_e19137_d_n11;
        locals.var_t1_dn12 = assign13470_e19137_d_n12;
        locals.var_t1_dn17 = assign13470_e19137_d_n17;

        let (assign13480_e19145, assign13480_e19145_d_n0, assign13480_e19145_d_n2, assign13480_e19145_d_n6, assign13480_e19145_d_n7, assign13480_e19145_d_n10, assign13480_e19145_d_n11, assign13480_e19145_d_n12, assign13480_e19145_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign13480_e19143: f64 = (-locals.var_t1);
        (assign13480_e19143, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn10), (-locals.var_t1_dn11), (-locals.var_t1_dn12), (-locals.var_t1_dn17),)
    } else {
        (locals.var_rrr_b, locals.var_rrr_b_dn0, locals.var_rrr_b_dn2, locals.var_rrr_b_dn6, locals.var_rrr_b_dn7, locals.var_rrr_b_dn10, locals.var_rrr_b_dn11, locals.var_rrr_b_dn12, locals.var_rrr_b_dn17,)
    }
};
        locals.var_rrr_b = assign13480_e19145;
        locals.var_rrr_b_dn0 = assign13480_e19145_d_n0;
        locals.var_rrr_b_dn2 = assign13480_e19145_d_n2;
        locals.var_rrr_b_dn6 = assign13480_e19145_d_n6;
        locals.var_rrr_b_dn7 = assign13480_e19145_d_n7;
        locals.var_rrr_b_dn10 = assign13480_e19145_d_n10;
        locals.var_rrr_b_dn11 = assign13480_e19145_d_n11;
        locals.var_rrr_b_dn12 = assign13480_e19145_d_n12;
        locals.var_rrr_b_dn17 = assign13480_e19145_d_n17;

        let assign13490_e19148: f64 = (locals.var_beta * locals.var_ps0b);
        let assign13490_e19150: f64 = (assign13490_e19148 - 1.0);
        let assign13490_e19152: f64 = if assign13490_e19150 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard424 = assign13490_e19152;

        let (assign13500_e19166, assign13500_e19166_d_n0, assign13500_e19166_d_n2, assign13500_e19166_d_n6, assign13500_e19166_d_n7, assign13500_e19166_d_n10, assign13500_e19166_d_n11, assign13500_e19166_d_n12, assign13500_e19166_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard424 != 0.0)) {
        let assign13500_e19161: f64 = (locals.var_beta * locals.var_ps0b);
        let assign13500_e19163: f64 = (assign13500_e19161 - 1.0);
        let assign13500_e19164: f64 = (assign13500_e19163).sqrt();
        (assign13500_e19164, ((locals.var_beta * locals.var_ps0b_dn0) / (2.0 * assign13500_e19164)), ((locals.var_beta * locals.var_ps0b_dn2) / (2.0 * assign13500_e19164)), ((locals.var_beta * locals.var_ps0b_dn6) / (2.0 * assign13500_e19164)), ((locals.var_beta * locals.var_ps0b_dn7) / (2.0 * assign13500_e19164)), (((locals.var_beta_dn10 * locals.var_ps0b) + (locals.var_beta * locals.var_ps0b_dn10)) / (2.0 * assign13500_e19164)), ((locals.var_beta * locals.var_ps0b_dn11) / (2.0 * assign13500_e19164)), ((locals.var_beta * locals.var_ps0b_dn12) / (2.0 * assign13500_e19164)), ((locals.var_beta * locals.var_ps0b_dn17) / (2.0 * assign13500_e19164)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign13500_e19166;
        locals.var_t1_dn0 = assign13500_e19166_d_n0;
        locals.var_t1_dn2 = assign13500_e19166_d_n2;
        locals.var_t1_dn6 = assign13500_e19166_d_n6;
        locals.var_t1_dn7 = assign13500_e19166_d_n7;
        locals.var_t1_dn10 = assign13500_e19166_d_n10;
        locals.var_t1_dn11 = assign13500_e19166_d_n11;
        locals.var_t1_dn12 = assign13500_e19166_d_n12;
        locals.var_t1_dn17 = assign13500_e19166_d_n17;

        let (assign13510_e19176, assign13510_e19176_d_n0, assign13510_e19176_d_n2, assign13510_e19176_d_n6, assign13510_e19176_d_n7, assign13510_e19176_d_n10, assign13510_e19176_d_n11, assign13510_e19176_d_n12, assign13510_e19176_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign13510_e19173: f64 = (locals.var_q_nl - locals.var_q_n0);
        let assign13510_e19174: f64 = (-assign13510_e19173);
        (assign13510_e19174, (-(locals.var_q_nl_dn0 - locals.var_q_n0_dn0)), (-(locals.var_q_nl_dn2 - locals.var_q_n0_dn2)), (-(locals.var_q_nl_dn6 - locals.var_q_n0_dn6)), (-(locals.var_q_nl_dn7 - locals.var_q_n0_dn7)), (-(locals.var_q_nl_dn10 - locals.var_q_n0_dn10)), (-(locals.var_q_nl_dn11 - locals.var_q_n0_dn11)), (-(locals.var_q_nl_dn12 - locals.var_q_n0_dn12)), (-(locals.var_q_nl_dn17 - locals.var_q_n0_dn17)),)
    } else {
        (locals.var_rrr_cc, locals.var_rrr_cc_dn0, locals.var_rrr_cc_dn2, locals.var_rrr_cc_dn6, locals.var_rrr_cc_dn7, locals.var_rrr_cc_dn10, locals.var_rrr_cc_dn11, locals.var_rrr_cc_dn12, locals.var_rrr_cc_dn17,)
    }
};
        locals.var_rrr_cc = assign13510_e19176;
        locals.var_rrr_cc_dn0 = assign13510_e19176_d_n0;
        locals.var_rrr_cc_dn2 = assign13510_e19176_d_n2;
        locals.var_rrr_cc_dn6 = assign13510_e19176_d_n6;
        locals.var_rrr_cc_dn7 = assign13510_e19176_d_n7;
        locals.var_rrr_cc_dn10 = assign13510_e19176_d_n10;
        locals.var_rrr_cc_dn11 = assign13510_e19176_d_n11;
        locals.var_rrr_cc_dn12 = assign13510_e19176_d_n12;
        locals.var_rrr_cc_dn17 = assign13510_e19176_d_n17;

        let assign13520_e19181: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13520_e19182: f64 = assign13520_e19181;
        let assign13520_e19186: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13520_e19189: f64 = if ((locals.var_rrr_cc < assign13520_e19182) && (assign13520_e19186 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard425 = assign13520_e19189;

        let (assign13530_e19204, assign13530_e19204_d_n0, assign13530_e19204_d_n2, assign13530_e19204_d_n6, assign13530_e19204_d_n7, assign13530_e19204_d_n10, assign13530_e19204_d_n11, assign13530_e19204_d_n12, assign13530_e19204_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard425 != 0.0)) {
        let assign13530_e19199: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13530_e19200: f64 = assign13530_e19199;
        let assign13530_e19202: f64 = (assign13530_e19200 - locals.var_rrr_cc);
        (assign13530_e19202, ((locals.var_q_fd_soi_dn0 * 1e-5) - locals.var_rrr_cc_dn0), ((locals.var_q_fd_soi_dn2 * 1e-5) - locals.var_rrr_cc_dn2), ((locals.var_q_fd_soi_dn6 * 1e-5) - locals.var_rrr_cc_dn6), ((locals.var_q_fd_soi_dn7 * 1e-5) - locals.var_rrr_cc_dn7), ((locals.var_q_fd_soi_dn10 * 1e-5) - locals.var_rrr_cc_dn10), ((locals.var_q_fd_soi_dn11 * 1e-5) - locals.var_rrr_cc_dn11), ((locals.var_q_fd_soi_dn12 * 1e-5) - locals.var_rrr_cc_dn12), ((locals.var_q_fd_soi_dn17 * 1e-5) - locals.var_rrr_cc_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign13530_e19204;
        locals.var_tmf1_dn0 = assign13530_e19204_d_n0;
        locals.var_tmf1_dn2 = assign13530_e19204_d_n2;
        locals.var_tmf1_dn6 = assign13530_e19204_d_n6;
        locals.var_tmf1_dn7 = assign13530_e19204_d_n7;
        locals.var_tmf1_dn10 = assign13530_e19204_d_n10;
        locals.var_tmf1_dn11 = assign13530_e19204_d_n11;
        locals.var_tmf1_dn12 = assign13530_e19204_d_n12;
        locals.var_tmf1_dn17 = assign13530_e19204_d_n17;

        let (assign13540_e19215, assign13540_e19215_d_n0, assign13540_e19215_d_n2, assign13540_e19215_d_n6, assign13540_e19215_d_n7, assign13540_e19215_d_n10, assign13540_e19215_d_n11, assign13540_e19215_d_n12, assign13540_e19215_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard425 != 0.0)) {
        let assign13540_e19213: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign13540_e19213, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign13540_e19215;
        locals.var_x2_dn0 = assign13540_e19215_d_n0;
        locals.var_x2_dn2 = assign13540_e19215_d_n2;
        locals.var_x2_dn6 = assign13540_e19215_d_n6;
        locals.var_x2_dn7 = assign13540_e19215_d_n7;
        locals.var_x2_dn10 = assign13540_e19215_d_n10;
        locals.var_x2_dn11 = assign13540_e19215_d_n11;
        locals.var_x2_dn12 = assign13540_e19215_d_n12;
        locals.var_x2_dn17 = assign13540_e19215_d_n17;

        let (assign13550_e19230, assign13550_e19230_d_n0, assign13550_e19230_d_n2, assign13550_e19230_d_n6, assign13550_e19230_d_n7, assign13550_e19230_d_n10, assign13550_e19230_d_n11, assign13550_e19230_d_n12, assign13550_e19230_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard425 != 0.0)) {
        let assign13550_e19224: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13550_e19227: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13550_e19228: f64 = (assign13550_e19224 * assign13550_e19227);
        (assign13550_e19228, (((locals.var_q_fd_soi_dn0 * 1e-5) * assign13550_e19227) + (assign13550_e19224 * (locals.var_q_fd_soi_dn0 * 1e-5))), (((locals.var_q_fd_soi_dn2 * 1e-5) * assign13550_e19227) + (assign13550_e19224 * (locals.var_q_fd_soi_dn2 * 1e-5))), (((locals.var_q_fd_soi_dn6 * 1e-5) * assign13550_e19227) + (assign13550_e19224 * (locals.var_q_fd_soi_dn6 * 1e-5))), (((locals.var_q_fd_soi_dn7 * 1e-5) * assign13550_e19227) + (assign13550_e19224 * (locals.var_q_fd_soi_dn7 * 1e-5))), (((locals.var_q_fd_soi_dn10 * 1e-5) * assign13550_e19227) + (assign13550_e19224 * (locals.var_q_fd_soi_dn10 * 1e-5))), (((locals.var_q_fd_soi_dn11 * 1e-5) * assign13550_e19227) + (assign13550_e19224 * (locals.var_q_fd_soi_dn11 * 1e-5))), (((locals.var_q_fd_soi_dn12 * 1e-5) * assign13550_e19227) + (assign13550_e19224 * (locals.var_q_fd_soi_dn12 * 1e-5))), (((locals.var_q_fd_soi_dn17 * 1e-5) * assign13550_e19227) + (assign13550_e19224 * (locals.var_q_fd_soi_dn17 * 1e-5))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign13550_e19230;
        locals.var_xmax2_dn0 = assign13550_e19230_d_n0;
        locals.var_xmax2_dn2 = assign13550_e19230_d_n2;
        locals.var_xmax2_dn6 = assign13550_e19230_d_n6;
        locals.var_xmax2_dn7 = assign13550_e19230_d_n7;
        locals.var_xmax2_dn10 = assign13550_e19230_d_n10;
        locals.var_xmax2_dn11 = assign13550_e19230_d_n11;
        locals.var_xmax2_dn12 = assign13550_e19230_d_n12;
        locals.var_xmax2_dn17 = assign13550_e19230_d_n17;

        let (assign13560_e19239, assign13560_e19239_d_n0, assign13560_e19239_d_n2, assign13560_e19239_d_n6, assign13560_e19239_d_n7, assign13560_e19239_d_n10, assign13560_e19239_d_n11, assign13560_e19239_d_n12, assign13560_e19239_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard425 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13560_e19239;
        locals.var_xp_dn0 = assign13560_e19239_d_n0;
        locals.var_xp_dn2 = assign13560_e19239_d_n2;
        locals.var_xp_dn6 = assign13560_e19239_d_n6;
        locals.var_xp_dn7 = assign13560_e19239_d_n7;
        locals.var_xp_dn10 = assign13560_e19239_d_n10;
        locals.var_xp_dn11 = assign13560_e19239_d_n11;
        locals.var_xp_dn12 = assign13560_e19239_d_n12;
        locals.var_xp_dn17 = assign13560_e19239_d_n17;

        let (assign13570_e19248, assign13570_e19248_d_n0, assign13570_e19248_d_n2, assign13570_e19248_d_n6, assign13570_e19248_d_n7, assign13570_e19248_d_n10, assign13570_e19248_d_n11, assign13570_e19248_d_n12, assign13570_e19248_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard425 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13570_e19248;
        locals.var_xmp_dn0 = assign13570_e19248_d_n0;
        locals.var_xmp_dn2 = assign13570_e19248_d_n2;
        locals.var_xmp_dn6 = assign13570_e19248_d_n6;
        locals.var_xmp_dn7 = assign13570_e19248_d_n7;
        locals.var_xmp_dn10 = assign13570_e19248_d_n10;
        locals.var_xmp_dn11 = assign13570_e19248_d_n11;
        locals.var_xmp_dn12 = assign13570_e19248_d_n12;
        locals.var_xmp_dn17 = assign13570_e19248_d_n17;

        let (assign13580_e19257,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard425 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign13580_e19257;

        let (assign13590_e19266,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard425 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13590_e19266;

        let (assign13600_e19275, assign13600_e19275_d_n0, assign13600_e19275_d_n2, assign13600_e19275_d_n6, assign13600_e19275_d_n7, assign13600_e19275_d_n10, assign13600_e19275_d_n11, assign13600_e19275_d_n12, assign13600_e19275_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard425 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign13600_e19275;
        locals.var_arg_dn0 = assign13600_e19275_d_n0;
        locals.var_arg_dn2 = assign13600_e19275_d_n2;
        locals.var_arg_dn6 = assign13600_e19275_d_n6;
        locals.var_arg_dn7 = assign13600_e19275_d_n7;
        locals.var_arg_dn10 = assign13600_e19275_d_n10;
        locals.var_arg_dn11 = assign13600_e19275_d_n11;
        locals.var_arg_dn12 = assign13600_e19275_d_n12;
        locals.var_arg_dn17 = assign13600_e19275_d_n17;

        let (assign13610_e19284, assign13610_e19284_d_n0, assign13610_e19284_d_n2, assign13610_e19284_d_n6, assign13610_e19284_d_n7, assign13610_e19284_d_n10, assign13610_e19284_d_n11, assign13610_e19284_d_n12, assign13610_e19284_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard425 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13610_e19284;
        locals.var_dnm_dn0 = assign13610_e19284_d_n0;
        locals.var_dnm_dn2 = assign13610_e19284_d_n2;
        locals.var_dnm_dn6 = assign13610_e19284_d_n6;
        locals.var_dnm_dn7 = assign13610_e19284_d_n7;
        locals.var_dnm_dn10 = assign13610_e19284_d_n10;
        locals.var_dnm_dn11 = assign13610_e19284_d_n11;
        locals.var_dnm_dn12 = assign13610_e19284_d_n12;
        locals.var_dnm_dn17 = assign13610_e19284_d_n17;

    }

    pub(super) fn stamp_transient_block_44(
        locals: &mut StampLocals,
    ) {
        let (assign13620_e19295, assign13620_e19295_d_n0, assign13620_e19295_d_n2, assign13620_e19295_d_n6, assign13620_e19295_d_n7, assign13620_e19295_d_n10, assign13620_e19295_d_n11, assign13620_e19295_d_n12, assign13620_e19295_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard425 != 0.0)) {
        let assign13620_e19293: f64 = (locals.var_xp * locals.var_x2);
        (assign13620_e19293, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13620_e19295;
        locals.var_xp_dn0 = assign13620_e19295_d_n0;
        locals.var_xp_dn2 = assign13620_e19295_d_n2;
        locals.var_xp_dn6 = assign13620_e19295_d_n6;
        locals.var_xp_dn7 = assign13620_e19295_d_n7;
        locals.var_xp_dn10 = assign13620_e19295_d_n10;
        locals.var_xp_dn11 = assign13620_e19295_d_n11;
        locals.var_xp_dn12 = assign13620_e19295_d_n12;
        locals.var_xp_dn17 = assign13620_e19295_d_n17;

        let (assign13630_e19306, assign13630_e19306_d_n0, assign13630_e19306_d_n2, assign13630_e19306_d_n6, assign13630_e19306_d_n7, assign13630_e19306_d_n10, assign13630_e19306_d_n11, assign13630_e19306_d_n12, assign13630_e19306_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard425 != 0.0)) {
        let assign13630_e19304: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign13630_e19304, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13630_e19306;
        locals.var_xmp_dn0 = assign13630_e19306_d_n0;
        locals.var_xmp_dn2 = assign13630_e19306_d_n2;
        locals.var_xmp_dn6 = assign13630_e19306_d_n6;
        locals.var_xmp_dn7 = assign13630_e19306_d_n7;
        locals.var_xmp_dn10 = assign13630_e19306_d_n10;
        locals.var_xmp_dn11 = assign13630_e19306_d_n11;
        locals.var_xmp_dn12 = assign13630_e19306_d_n12;
        locals.var_xmp_dn17 = assign13630_e19306_d_n17;

        let (assign13640_e19317, assign13640_e19317_d_n0, assign13640_e19317_d_n2, assign13640_e19317_d_n6, assign13640_e19317_d_n7, assign13640_e19317_d_n10, assign13640_e19317_d_n11, assign13640_e19317_d_n12, assign13640_e19317_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard425 != 0.0)) {
        let assign13640_e19315: f64 = (locals.var_xp * locals.var_x2);
        (assign13640_e19315, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13640_e19317;
        locals.var_xp_dn0 = assign13640_e19317_d_n0;
        locals.var_xp_dn2 = assign13640_e19317_d_n2;
        locals.var_xp_dn6 = assign13640_e19317_d_n6;
        locals.var_xp_dn7 = assign13640_e19317_d_n7;
        locals.var_xp_dn10 = assign13640_e19317_d_n10;
        locals.var_xp_dn11 = assign13640_e19317_d_n11;
        locals.var_xp_dn12 = assign13640_e19317_d_n12;
        locals.var_xp_dn17 = assign13640_e19317_d_n17;

        let (assign13650_e19328, assign13650_e19328_d_n0, assign13650_e19328_d_n2, assign13650_e19328_d_n6, assign13650_e19328_d_n7, assign13650_e19328_d_n10, assign13650_e19328_d_n11, assign13650_e19328_d_n12, assign13650_e19328_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard425 != 0.0)) {
        let assign13650_e19326: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign13650_e19326, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13650_e19328;
        locals.var_xmp_dn0 = assign13650_e19328_d_n0;
        locals.var_xmp_dn2 = assign13650_e19328_d_n2;
        locals.var_xmp_dn6 = assign13650_e19328_d_n6;
        locals.var_xmp_dn7 = assign13650_e19328_d_n7;
        locals.var_xmp_dn10 = assign13650_e19328_d_n10;
        locals.var_xmp_dn11 = assign13650_e19328_d_n11;
        locals.var_xmp_dn12 = assign13650_e19328_d_n12;
        locals.var_xmp_dn17 = assign13650_e19328_d_n17;

        let (assign13660_e19339, assign13660_e19339_d_n0, assign13660_e19339_d_n2, assign13660_e19339_d_n6, assign13660_e19339_d_n7, assign13660_e19339_d_n10, assign13660_e19339_d_n11, assign13660_e19339_d_n12, assign13660_e19339_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard425 != 0.0)) {
        let assign13660_e19337: f64 = (locals.var_xp + locals.var_xmp);
        (assign13660_e19337, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign13660_e19339;
        locals.var_arg_dn0 = assign13660_e19339_d_n0;
        locals.var_arg_dn2 = assign13660_e19339_d_n2;
        locals.var_arg_dn6 = assign13660_e19339_d_n6;
        locals.var_arg_dn7 = assign13660_e19339_d_n7;
        locals.var_arg_dn10 = assign13660_e19339_d_n10;
        locals.var_arg_dn11 = assign13660_e19339_d_n11;
        locals.var_arg_dn12 = assign13660_e19339_d_n12;
        locals.var_arg_dn17 = assign13660_e19339_d_n17;

        let (assign13670_e19348, assign13670_e19348_d_n0, assign13670_e19348_d_n2, assign13670_e19348_d_n6, assign13670_e19348_d_n7, assign13670_e19348_d_n10, assign13670_e19348_d_n11, assign13670_e19348_d_n12, assign13670_e19348_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard425 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13670_e19348;
        locals.var_dnm_dn0 = assign13670_e19348_d_n0;
        locals.var_dnm_dn2 = assign13670_e19348_d_n2;
        locals.var_dnm_dn6 = assign13670_e19348_d_n6;
        locals.var_dnm_dn7 = assign13670_e19348_d_n7;
        locals.var_dnm_dn10 = assign13670_e19348_d_n10;
        locals.var_dnm_dn11 = assign13670_e19348_d_n11;
        locals.var_dnm_dn12 = assign13670_e19348_d_n12;
        locals.var_dnm_dn17 = assign13670_e19348_d_n17;

        let assign13680_e19363: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard426 = assign13680_e19363;

        let assign13690_e19366: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard427 = assign13690_e19366;

        let (assign13700_e19379,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard425 != 0.0)) && (locals.var_guard426 != 0.0)) && (locals.var_guard427 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13700_e19379;

        let assign13710_e19382: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard428 = assign13710_e19382;

        let (assign13720_e19398,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard425 != 0.0)) && (locals.var_guard426 != 0.0)) && (locals.var_guard427 == 0.0)) && (locals.var_guard428 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13720_e19398;

        let assign13730_e19401: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard429 = assign13730_e19401;

        let (assign13740_e19420,) = {
    if (((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard425 != 0.0)) && (locals.var_guard426 != 0.0)) && (locals.var_guard427 == 0.0)) && (locals.var_guard428 == 0.0)) && (locals.var_guard429 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13740_e19420;

        let assign13750_e19423: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard430 = assign13750_e19423;

        let (assign13760_e19445,) = {
    if ((((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard425 != 0.0)) && (locals.var_guard426 != 0.0)) && (locals.var_guard427 == 0.0)) && (locals.var_guard428 == 0.0)) && (locals.var_guard429 == 0.0)) && (locals.var_guard430 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13760_e19445;

        let (assign13770_e19456,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard425 != 0.0)) && (locals.var_guard426 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign13770_e19456;

        let mut assign13780_loop_guard: usize = 0;
        while {
            let assign13780_cond_e19468: f64 = if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard425 != 0.0)) && (locals.var_guard426 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign13780_cond_e19468 != 0.0
        } {
            assign13780_loop_guard += 1;
            assert!(assign13780_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign13780_body0_e19480, assign13780_body0_e19480_d_n0, assign13780_body0_e19480_d_n2, assign13780_body0_e19480_d_n6, assign13780_body0_e19480_d_n7, assign13780_body0_e19480_d_n10, assign13780_body0_e19480_d_n11, assign13780_body0_e19480_d_n12, assign13780_body0_e19480_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard425 != 0.0)) && (locals.var_guard426 != 0.0)) {
        let assign13780_body0_e19478: f64 = (locals.var_dnm).sqrt();
        (assign13780_body0_e19478, (locals.var_dnm_dn0 / (2.0 * assign13780_body0_e19478)), (locals.var_dnm_dn2 / (2.0 * assign13780_body0_e19478)), (locals.var_dnm_dn6 / (2.0 * assign13780_body0_e19478)), (locals.var_dnm_dn7 / (2.0 * assign13780_body0_e19478)), (locals.var_dnm_dn10 / (2.0 * assign13780_body0_e19478)), (locals.var_dnm_dn11 / (2.0 * assign13780_body0_e19478)), (locals.var_dnm_dn12 / (2.0 * assign13780_body0_e19478)), (locals.var_dnm_dn17 / (2.0 * assign13780_body0_e19478)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign13780_body0_e19480;
            locals.var_dnm_dn0 = assign13780_body0_e19480_d_n0;
            locals.var_dnm_dn2 = assign13780_body0_e19480_d_n2;
            locals.var_dnm_dn6 = assign13780_body0_e19480_d_n6;
            locals.var_dnm_dn7 = assign13780_body0_e19480_d_n7;
            locals.var_dnm_dn10 = assign13780_body0_e19480_d_n10;
            locals.var_dnm_dn11 = assign13780_body0_e19480_d_n11;
            locals.var_dnm_dn12 = assign13780_body0_e19480_d_n12;
            locals.var_dnm_dn17 = assign13780_body0_e19480_d_n17;
            let (assign13780_body1_e19493,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard425 != 0.0)) && (locals.var_guard426 != 0.0)) {
        let assign13780_body1_e19491: f64 = (locals.var_m0 + 1.0);
        (assign13780_body1_e19491,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign13780_body1_e19493;
        }

        let (assign13790_e19511, assign13790_e19511_d_n0, assign13790_e19511_d_n2, assign13790_e19511_d_n6, assign13790_e19511_d_n7, assign13790_e19511_d_n10, assign13790_e19511_d_n11, assign13790_e19511_d_n12, assign13790_e19511_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard425 != 0.0)) && (locals.var_guard426 == 0.0)) {
        let assign13790_e19507: f64 = (2.0 * 2.0);
        let assign13790_e19508: f64 = (1.0 / assign13790_e19507);
        let assign13790_e19509: f64 = (locals.var_dnm).powf(assign13790_e19508);
        (assign13790_e19509, if 0.0 == 0.0 && ((assign13790_e19508) as f64).is_finite() && ((assign13790_e19508) as f64).fract() == 0.0 { if assign13790_e19508 == 0.0 { 0.0 } else { (assign13790_e19508 * ((locals.var_dnm).powf(assign13790_e19508 - 1.0) * locals.var_dnm_dn0)) } } else { (assign13790_e19509 * (assign13790_e19508 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13790_e19508) as f64).is_finite() && ((assign13790_e19508) as f64).fract() == 0.0 { if assign13790_e19508 == 0.0 { 0.0 } else { (assign13790_e19508 * ((locals.var_dnm).powf(assign13790_e19508 - 1.0) * locals.var_dnm_dn2)) } } else { (assign13790_e19509 * (assign13790_e19508 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13790_e19508) as f64).is_finite() && ((assign13790_e19508) as f64).fract() == 0.0 { if assign13790_e19508 == 0.0 { 0.0 } else { (assign13790_e19508 * ((locals.var_dnm).powf(assign13790_e19508 - 1.0) * locals.var_dnm_dn6)) } } else { (assign13790_e19509 * (assign13790_e19508 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13790_e19508) as f64).is_finite() && ((assign13790_e19508) as f64).fract() == 0.0 { if assign13790_e19508 == 0.0 { 0.0 } else { (assign13790_e19508 * ((locals.var_dnm).powf(assign13790_e19508 - 1.0) * locals.var_dnm_dn7)) } } else { (assign13790_e19509 * (assign13790_e19508 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13790_e19508) as f64).is_finite() && ((assign13790_e19508) as f64).fract() == 0.0 { if assign13790_e19508 == 0.0 { 0.0 } else { (assign13790_e19508 * ((locals.var_dnm).powf(assign13790_e19508 - 1.0) * locals.var_dnm_dn10)) } } else { (assign13790_e19509 * (assign13790_e19508 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13790_e19508) as f64).is_finite() && ((assign13790_e19508) as f64).fract() == 0.0 { if assign13790_e19508 == 0.0 { 0.0 } else { (assign13790_e19508 * ((locals.var_dnm).powf(assign13790_e19508 - 1.0) * locals.var_dnm_dn11)) } } else { (assign13790_e19509 * (assign13790_e19508 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13790_e19508) as f64).is_finite() && ((assign13790_e19508) as f64).fract() == 0.0 { if assign13790_e19508 == 0.0 { 0.0 } else { (assign13790_e19508 * ((locals.var_dnm).powf(assign13790_e19508 - 1.0) * locals.var_dnm_dn12)) } } else { (assign13790_e19509 * (assign13790_e19508 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13790_e19508) as f64).is_finite() && ((assign13790_e19508) as f64).fract() == 0.0 { if assign13790_e19508 == 0.0 { 0.0 } else { (assign13790_e19508 * ((locals.var_dnm).powf(assign13790_e19508 - 1.0) * locals.var_dnm_dn17)) } } else { (assign13790_e19509 * (assign13790_e19508 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13790_e19511;
        locals.var_dnm_dn0 = assign13790_e19511_d_n0;
        locals.var_dnm_dn2 = assign13790_e19511_d_n2;
        locals.var_dnm_dn6 = assign13790_e19511_d_n6;
        locals.var_dnm_dn7 = assign13790_e19511_d_n7;
        locals.var_dnm_dn10 = assign13790_e19511_d_n10;
        locals.var_dnm_dn11 = assign13790_e19511_d_n11;
        locals.var_dnm_dn12 = assign13790_e19511_d_n12;
        locals.var_dnm_dn17 = assign13790_e19511_d_n17;

        let (assign13800_e19522, assign13800_e19522_d_n0, assign13800_e19522_d_n2, assign13800_e19522_d_n6, assign13800_e19522_d_n7, assign13800_e19522_d_n10, assign13800_e19522_d_n11, assign13800_e19522_d_n12, assign13800_e19522_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard425 != 0.0)) {
        let assign13800_e19520: f64 = (1.0 / locals.var_dnm);
        (assign13800_e19520, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13800_e19522;
        locals.var_dnm_dn0 = assign13800_e19522_d_n0;
        locals.var_dnm_dn2 = assign13800_e19522_d_n2;
        locals.var_dnm_dn6 = assign13800_e19522_d_n6;
        locals.var_dnm_dn7 = assign13800_e19522_d_n7;
        locals.var_dnm_dn10 = assign13800_e19522_d_n10;
        locals.var_dnm_dn11 = assign13800_e19522_d_n11;
        locals.var_dnm_dn12 = assign13800_e19522_d_n12;
        locals.var_dnm_dn17 = assign13800_e19522_d_n17;

        let (assign13810_e19537, assign13810_e19537_d_n0, assign13810_e19537_d_n2, assign13810_e19537_d_n6, assign13810_e19537_d_n7, assign13810_e19537_d_n10, assign13810_e19537_d_n11, assign13810_e19537_d_n12, assign13810_e19537_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard425 != 0.0)) {
        let assign13810_e19532: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13810_e19533: f64 = (locals.var_tmf1 * assign13810_e19532);
        let assign13810_e19535: f64 = (assign13810_e19533 * locals.var_dnm);
        (assign13810_e19535, ((((locals.var_tmf1_dn0 * assign13810_e19532) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn0 * 1e-5))) * locals.var_dnm) + (assign13810_e19533 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign13810_e19532) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn2 * 1e-5))) * locals.var_dnm) + (assign13810_e19533 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn6 * assign13810_e19532) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn6 * 1e-5))) * locals.var_dnm) + (assign13810_e19533 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign13810_e19532) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn7 * 1e-5))) * locals.var_dnm) + (assign13810_e19533 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn10 * assign13810_e19532) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn10 * 1e-5))) * locals.var_dnm) + (assign13810_e19533 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * assign13810_e19532) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn11 * 1e-5))) * locals.var_dnm) + (assign13810_e19533 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn12 * assign13810_e19532) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn12 * 1e-5))) * locals.var_dnm) + (assign13810_e19533 * locals.var_dnm_dn12)), ((((locals.var_tmf1_dn17 * assign13810_e19532) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn17 * 1e-5))) * locals.var_dnm) + (assign13810_e19533 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign13810_e19537;
        locals.var_tmf0_dn0 = assign13810_e19537_d_n0;
        locals.var_tmf0_dn2 = assign13810_e19537_d_n2;
        locals.var_tmf0_dn6 = assign13810_e19537_d_n6;
        locals.var_tmf0_dn7 = assign13810_e19537_d_n7;
        locals.var_tmf0_dn10 = assign13810_e19537_d_n10;
        locals.var_tmf0_dn11 = assign13810_e19537_d_n11;
        locals.var_tmf0_dn12 = assign13810_e19537_d_n12;
        locals.var_tmf0_dn17 = assign13810_e19537_d_n17;

        let (assign13820_e19552, assign13820_e19552_d_n0, assign13820_e19552_d_n2, assign13820_e19552_d_n6, assign13820_e19552_d_n7, assign13820_e19552_d_n10, assign13820_e19552_d_n11, assign13820_e19552_d_n12, assign13820_e19552_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard425 != 0.0)) {
        let assign13820_e19547: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13820_e19548: f64 = assign13820_e19547;
        let assign13820_e19550: f64 = (assign13820_e19548 - locals.var_tmf0);
        (assign13820_e19550, ((locals.var_q_fd_soi_dn0 * 1e-5) - locals.var_tmf0_dn0), ((locals.var_q_fd_soi_dn2 * 1e-5) - locals.var_tmf0_dn2), ((locals.var_q_fd_soi_dn6 * 1e-5) - locals.var_tmf0_dn6), ((locals.var_q_fd_soi_dn7 * 1e-5) - locals.var_tmf0_dn7), ((locals.var_q_fd_soi_dn10 * 1e-5) - locals.var_tmf0_dn10), ((locals.var_q_fd_soi_dn11 * 1e-5) - locals.var_tmf0_dn11), ((locals.var_q_fd_soi_dn12 * 1e-5) - locals.var_tmf0_dn12), ((locals.var_q_fd_soi_dn17 * 1e-5) - locals.var_tmf0_dn17),)
    } else {
        (locals.var_rrr_cc, locals.var_rrr_cc_dn0, locals.var_rrr_cc_dn2, locals.var_rrr_cc_dn6, locals.var_rrr_cc_dn7, locals.var_rrr_cc_dn10, locals.var_rrr_cc_dn11, locals.var_rrr_cc_dn12, locals.var_rrr_cc_dn17,)
    }
};
        locals.var_rrr_cc = assign13820_e19552;
        locals.var_rrr_cc_dn0 = assign13820_e19552_d_n0;
        locals.var_rrr_cc_dn2 = assign13820_e19552_d_n2;
        locals.var_rrr_cc_dn6 = assign13820_e19552_d_n6;
        locals.var_rrr_cc_dn7 = assign13820_e19552_d_n7;
        locals.var_rrr_cc_dn10 = assign13820_e19552_d_n10;
        locals.var_rrr_cc_dn11 = assign13820_e19552_d_n11;
        locals.var_rrr_cc_dn12 = assign13820_e19552_d_n12;
        locals.var_rrr_cc_dn17 = assign13820_e19552_d_n17;

        let (assign13830_e19562, assign13830_e19562_d_n0, assign13830_e19562_d_n2, assign13830_e19562_d_n6, assign13830_e19562_d_n7, assign13830_e19562_d_n10, assign13830_e19562_d_n11, assign13830_e19562_d_n12, assign13830_e19562_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard425 == 0.0)) {
        (locals.var_rrr_cc, locals.var_rrr_cc_dn0, locals.var_rrr_cc_dn2, locals.var_rrr_cc_dn6, locals.var_rrr_cc_dn7, locals.var_rrr_cc_dn10, locals.var_rrr_cc_dn11, locals.var_rrr_cc_dn12, locals.var_rrr_cc_dn17,)
    } else {
        (locals.var_rrr_cc, locals.var_rrr_cc_dn0, locals.var_rrr_cc_dn2, locals.var_rrr_cc_dn6, locals.var_rrr_cc_dn7, locals.var_rrr_cc_dn10, locals.var_rrr_cc_dn11, locals.var_rrr_cc_dn12, locals.var_rrr_cc_dn17,)
    }
};
        locals.var_rrr_cc = assign13830_e19562;
        locals.var_rrr_cc_dn0 = assign13830_e19562_d_n0;
        locals.var_rrr_cc_dn2 = assign13830_e19562_d_n2;
        locals.var_rrr_cc_dn6 = assign13830_e19562_d_n6;
        locals.var_rrr_cc_dn7 = assign13830_e19562_d_n7;
        locals.var_rrr_cc_dn10 = assign13830_e19562_d_n10;
        locals.var_rrr_cc_dn11 = assign13830_e19562_d_n11;
        locals.var_rrr_cc_dn12 = assign13830_e19562_d_n12;
        locals.var_rrr_cc_dn17 = assign13830_e19562_d_n17;

        let (assign13840_e19582, assign13840_e19582_d_n0, assign13840_e19582_d_n2, assign13840_e19582_d_n6, assign13840_e19582_d_n7, assign13840_e19582_d_n10, assign13840_e19582_d_n11, assign13840_e19582_d_n12, assign13840_e19582_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign13840_e19570: f64 = (-locals.var_rrr_cc);
        let assign13840_e19571: f64 = (2.0 * assign13840_e19570);
        let assign13840_e19574: f64 = (locals.var_beta * locals.var_c_fox);
        let assign13840_e19576: f64 = (assign13840_e19574 * locals.var_rrr_p0);
        let assign13840_e19578: f64 = (assign13840_e19576 * locals.var_rrr_p0);
        let assign13840_e19579: f64 = (assign13840_e19571 / assign13840_e19578);
        let assign13840_e19580: f64 = (1.0 + assign13840_e19579);
        (assign13840_e19580, ((((2.0 * (-locals.var_rrr_cc_dn0)) * assign13840_e19578) - (assign13840_e19571 * (((((locals.var_beta * locals.var_c_fox_dn0) * locals.var_rrr_p0) + (assign13840_e19574 * locals.var_rrr_p0_dn0)) * locals.var_rrr_p0) + (assign13840_e19576 * locals.var_rrr_p0_dn0)))) / (assign13840_e19578 * assign13840_e19578)), ((((2.0 * (-locals.var_rrr_cc_dn2)) * assign13840_e19578) - (assign13840_e19571 * (((((locals.var_beta * locals.var_c_fox_dn2) * locals.var_rrr_p0) + (assign13840_e19574 * locals.var_rrr_p0_dn2)) * locals.var_rrr_p0) + (assign13840_e19576 * locals.var_rrr_p0_dn2)))) / (assign13840_e19578 * assign13840_e19578)), ((((2.0 * (-locals.var_rrr_cc_dn6)) * assign13840_e19578) - (assign13840_e19571 * (((((locals.var_beta * locals.var_c_fox_dn6) * locals.var_rrr_p0) + (assign13840_e19574 * locals.var_rrr_p0_dn6)) * locals.var_rrr_p0) + (assign13840_e19576 * locals.var_rrr_p0_dn6)))) / (assign13840_e19578 * assign13840_e19578)), ((((2.0 * (-locals.var_rrr_cc_dn7)) * assign13840_e19578) - (assign13840_e19571 * (((((locals.var_beta * locals.var_c_fox_dn7) * locals.var_rrr_p0) + (assign13840_e19574 * locals.var_rrr_p0_dn7)) * locals.var_rrr_p0) + (assign13840_e19576 * locals.var_rrr_p0_dn7)))) / (assign13840_e19578 * assign13840_e19578)), ((((2.0 * (-locals.var_rrr_cc_dn10)) * assign13840_e19578) - (assign13840_e19571 * ((((((locals.var_beta_dn10 * locals.var_c_fox) + (locals.var_beta * locals.var_c_fox_dn10)) * locals.var_rrr_p0) + (assign13840_e19574 * locals.var_rrr_p0_dn10)) * locals.var_rrr_p0) + (assign13840_e19576 * locals.var_rrr_p0_dn10)))) / (assign13840_e19578 * assign13840_e19578)), ((((2.0 * (-locals.var_rrr_cc_dn11)) * assign13840_e19578) - (assign13840_e19571 * (((((locals.var_beta * locals.var_c_fox_dn11) * locals.var_rrr_p0) + (assign13840_e19574 * locals.var_rrr_p0_dn11)) * locals.var_rrr_p0) + (assign13840_e19576 * locals.var_rrr_p0_dn11)))) / (assign13840_e19578 * assign13840_e19578)), ((((2.0 * (-locals.var_rrr_cc_dn12)) * assign13840_e19578) - (assign13840_e19571 * (((((locals.var_beta * locals.var_c_fox_dn12) * locals.var_rrr_p0) + (assign13840_e19574 * locals.var_rrr_p0_dn12)) * locals.var_rrr_p0) + (assign13840_e19576 * locals.var_rrr_p0_dn12)))) / (assign13840_e19578 * assign13840_e19578)), ((((2.0 * (-locals.var_rrr_cc_dn17)) * assign13840_e19578) - (assign13840_e19571 * (((((locals.var_beta * locals.var_c_fox_dn17) * locals.var_rrr_p0) + (assign13840_e19574 * locals.var_rrr_p0_dn17)) * locals.var_rrr_p0) + (assign13840_e19576 * locals.var_rrr_p0_dn17)))) / (assign13840_e19578 * assign13840_e19578)),)
    } else {
        (locals.var_rrr_alpha_soi, locals.var_rrr_alpha_soi_dn0, locals.var_rrr_alpha_soi_dn2, locals.var_rrr_alpha_soi_dn6, locals.var_rrr_alpha_soi_dn7, locals.var_rrr_alpha_soi_dn10, locals.var_rrr_alpha_soi_dn11, locals.var_rrr_alpha_soi_dn12, locals.var_rrr_alpha_soi_dn17,)
    }
};
        locals.var_rrr_alpha_soi = assign13840_e19582;
        locals.var_rrr_alpha_soi_dn0 = assign13840_e19582_d_n0;
        locals.var_rrr_alpha_soi_dn2 = assign13840_e19582_d_n2;
        locals.var_rrr_alpha_soi_dn6 = assign13840_e19582_d_n6;
        locals.var_rrr_alpha_soi_dn7 = assign13840_e19582_d_n7;
        locals.var_rrr_alpha_soi_dn10 = assign13840_e19582_d_n10;
        locals.var_rrr_alpha_soi_dn11 = assign13840_e19582_d_n11;
        locals.var_rrr_alpha_soi_dn12 = assign13840_e19582_d_n12;
        locals.var_rrr_alpha_soi_dn17 = assign13840_e19582_d_n17;

        let (assign13850_e19595, assign13850_e19595_d_n0, assign13850_e19595_d_n2, assign13850_e19595_d_n6, assign13850_e19595_d_n7, assign13850_e19595_d_n10, assign13850_e19595_d_n11, assign13850_e19595_d_n12, assign13850_e19595_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign13850_e19589: f64 = (locals.var_rrr_p0 * locals.var_rrr_p0);
        let assign13850_e19591: f64 = (assign13850_e19589 * locals.var_rrr_p0);
        let assign13850_e19593: f64 = (assign13850_e19591 * locals.var_rrr_p0);
        (assign13850_e19593, ((((((locals.var_rrr_p0_dn0 * locals.var_rrr_p0) + (locals.var_rrr_p0 * locals.var_rrr_p0_dn0)) * locals.var_rrr_p0) + (assign13850_e19589 * locals.var_rrr_p0_dn0)) * locals.var_rrr_p0) + (assign13850_e19591 * locals.var_rrr_p0_dn0)), ((((((locals.var_rrr_p0_dn2 * locals.var_rrr_p0) + (locals.var_rrr_p0 * locals.var_rrr_p0_dn2)) * locals.var_rrr_p0) + (assign13850_e19589 * locals.var_rrr_p0_dn2)) * locals.var_rrr_p0) + (assign13850_e19591 * locals.var_rrr_p0_dn2)), ((((((locals.var_rrr_p0_dn6 * locals.var_rrr_p0) + (locals.var_rrr_p0 * locals.var_rrr_p0_dn6)) * locals.var_rrr_p0) + (assign13850_e19589 * locals.var_rrr_p0_dn6)) * locals.var_rrr_p0) + (assign13850_e19591 * locals.var_rrr_p0_dn6)), ((((((locals.var_rrr_p0_dn7 * locals.var_rrr_p0) + (locals.var_rrr_p0 * locals.var_rrr_p0_dn7)) * locals.var_rrr_p0) + (assign13850_e19589 * locals.var_rrr_p0_dn7)) * locals.var_rrr_p0) + (assign13850_e19591 * locals.var_rrr_p0_dn7)), ((((((locals.var_rrr_p0_dn10 * locals.var_rrr_p0) + (locals.var_rrr_p0 * locals.var_rrr_p0_dn10)) * locals.var_rrr_p0) + (assign13850_e19589 * locals.var_rrr_p0_dn10)) * locals.var_rrr_p0) + (assign13850_e19591 * locals.var_rrr_p0_dn10)), ((((((locals.var_rrr_p0_dn11 * locals.var_rrr_p0) + (locals.var_rrr_p0 * locals.var_rrr_p0_dn11)) * locals.var_rrr_p0) + (assign13850_e19589 * locals.var_rrr_p0_dn11)) * locals.var_rrr_p0) + (assign13850_e19591 * locals.var_rrr_p0_dn11)), ((((((locals.var_rrr_p0_dn12 * locals.var_rrr_p0) + (locals.var_rrr_p0 * locals.var_rrr_p0_dn12)) * locals.var_rrr_p0) + (assign13850_e19589 * locals.var_rrr_p0_dn12)) * locals.var_rrr_p0) + (assign13850_e19591 * locals.var_rrr_p0_dn12)), ((((((locals.var_rrr_p0_dn17 * locals.var_rrr_p0) + (locals.var_rrr_p0 * locals.var_rrr_p0_dn17)) * locals.var_rrr_p0) + (assign13850_e19589 * locals.var_rrr_p0_dn17)) * locals.var_rrr_p0) + (assign13850_e19591 * locals.var_rrr_p0_dn17)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign13850_e19595;
        locals.var_t1_dn0 = assign13850_e19595_d_n0;
        locals.var_t1_dn2 = assign13850_e19595_d_n2;
        locals.var_t1_dn6 = assign13850_e19595_d_n6;
        locals.var_t1_dn7 = assign13850_e19595_d_n7;
        locals.var_t1_dn10 = assign13850_e19595_d_n10;
        locals.var_t1_dn11 = assign13850_e19595_d_n11;
        locals.var_t1_dn12 = assign13850_e19595_d_n12;
        locals.var_t1_dn17 = assign13850_e19595_d_n17;

        let (assign13860_e19604, assign13860_e19604_d_n0, assign13860_e19604_d_n2, assign13860_e19604_d_n6, assign13860_e19604_d_n7, assign13860_e19604_d_n10, assign13860_e19604_d_n11, assign13860_e19604_d_n12, assign13860_e19604_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign13860_e19602: f64 = (locals.var_rrr_alpha_soi * locals.var_rrr_p0);
        (assign13860_e19602, ((locals.var_rrr_alpha_soi_dn0 * locals.var_rrr_p0) + (locals.var_rrr_alpha_soi * locals.var_rrr_p0_dn0)), ((locals.var_rrr_alpha_soi_dn2 * locals.var_rrr_p0) + (locals.var_rrr_alpha_soi * locals.var_rrr_p0_dn2)), ((locals.var_rrr_alpha_soi_dn6 * locals.var_rrr_p0) + (locals.var_rrr_alpha_soi * locals.var_rrr_p0_dn6)), ((locals.var_rrr_alpha_soi_dn7 * locals.var_rrr_p0) + (locals.var_rrr_alpha_soi * locals.var_rrr_p0_dn7)), ((locals.var_rrr_alpha_soi_dn10 * locals.var_rrr_p0) + (locals.var_rrr_alpha_soi * locals.var_rrr_p0_dn10)), ((locals.var_rrr_alpha_soi_dn11 * locals.var_rrr_p0) + (locals.var_rrr_alpha_soi * locals.var_rrr_p0_dn11)), ((locals.var_rrr_alpha_soi_dn12 * locals.var_rrr_p0) + (locals.var_rrr_alpha_soi * locals.var_rrr_p0_dn12)), ((locals.var_rrr_alpha_soi_dn17 * locals.var_rrr_p0) + (locals.var_rrr_alpha_soi * locals.var_rrr_p0_dn17)),)
    } else {
        (locals.var_rrr_dd, locals.var_rrr_dd_dn0, locals.var_rrr_dd_dn2, locals.var_rrr_dd_dn6, locals.var_rrr_dd_dn7, locals.var_rrr_dd_dn10, locals.var_rrr_dd_dn11, locals.var_rrr_dd_dn12, locals.var_rrr_dd_dn17,)
    }
};
        locals.var_rrr_dd = assign13860_e19604;
        locals.var_rrr_dd_dn0 = assign13860_e19604_d_n0;
        locals.var_rrr_dd_dn2 = assign13860_e19604_d_n2;
        locals.var_rrr_dd_dn6 = assign13860_e19604_d_n6;
        locals.var_rrr_dd_dn7 = assign13860_e19604_d_n7;
        locals.var_rrr_dd_dn10 = assign13860_e19604_d_n10;
        locals.var_rrr_dd_dn11 = assign13860_e19604_d_n11;
        locals.var_rrr_dd_dn12 = assign13860_e19604_d_n12;
        locals.var_rrr_dd_dn17 = assign13860_e19604_d_n17;

        let (assign13870_e19615, assign13870_e19615_d_n0, assign13870_e19615_d_n2, assign13870_e19615_d_n6, assign13870_e19615_d_n7, assign13870_e19615_d_n10, assign13870_e19615_d_n11, assign13870_e19615_d_n12, assign13870_e19615_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign13870_e19612: f64 = (locals.var_rrr_dd / locals.var_vgvt);
        let assign13870_e19613: f64 = (1.0 - assign13870_e19612);
        (assign13870_e19613, (-(((locals.var_rrr_dd_dn0 * locals.var_vgvt) - (locals.var_rrr_dd * locals.var_vgvt_dn0)) / (locals.var_vgvt * locals.var_vgvt))), (-(((locals.var_rrr_dd_dn2 * locals.var_vgvt) - (locals.var_rrr_dd * locals.var_vgvt_dn2)) / (locals.var_vgvt * locals.var_vgvt))), (-(((locals.var_rrr_dd_dn6 * locals.var_vgvt) - (locals.var_rrr_dd * locals.var_vgvt_dn6)) / (locals.var_vgvt * locals.var_vgvt))), (-(((locals.var_rrr_dd_dn7 * locals.var_vgvt) - (locals.var_rrr_dd * locals.var_vgvt_dn7)) / (locals.var_vgvt * locals.var_vgvt))), (-(((locals.var_rrr_dd_dn10 * locals.var_vgvt) - (locals.var_rrr_dd * locals.var_vgvt_dn10)) / (locals.var_vgvt * locals.var_vgvt))), (-(((locals.var_rrr_dd_dn11 * locals.var_vgvt) - (locals.var_rrr_dd * locals.var_vgvt_dn11)) / (locals.var_vgvt * locals.var_vgvt))), (-(((locals.var_rrr_dd_dn12 * locals.var_vgvt) - (locals.var_rrr_dd * locals.var_vgvt_dn12)) / (locals.var_vgvt * locals.var_vgvt))), (-(((locals.var_rrr_dd_dn17 * locals.var_vgvt) - (locals.var_rrr_dd * locals.var_vgvt_dn17)) / (locals.var_vgvt * locals.var_vgvt))),)
    } else {
        (locals.var_rrr_eta, locals.var_rrr_eta_dn0, locals.var_rrr_eta_dn2, locals.var_rrr_eta_dn6, locals.var_rrr_eta_dn7, locals.var_rrr_eta_dn10, locals.var_rrr_eta_dn11, locals.var_rrr_eta_dn12, locals.var_rrr_eta_dn17,)
    }
};
        locals.var_rrr_eta = assign13870_e19615;
        locals.var_rrr_eta_dn0 = assign13870_e19615_d_n0;
        locals.var_rrr_eta_dn2 = assign13870_e19615_d_n2;
        locals.var_rrr_eta_dn6 = assign13870_e19615_d_n6;
        locals.var_rrr_eta_dn7 = assign13870_e19615_d_n7;
        locals.var_rrr_eta_dn10 = assign13870_e19615_d_n10;
        locals.var_rrr_eta_dn11 = assign13870_e19615_d_n11;
        locals.var_rrr_eta_dn12 = assign13870_e19615_d_n12;
        locals.var_rrr_eta_dn17 = assign13870_e19615_d_n17;

        let assign13880_e19619: f64 = 1e-5;
        let assign13880_e19624: f64 = if ((locals.var_rrr_eta < assign13880_e19619) && (1e-5 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard431 = assign13880_e19624;

        let (assign13890_e19637, assign13890_e19637_d_n0, assign13890_e19637_d_n2, assign13890_e19637_d_n6, assign13890_e19637_d_n7, assign13890_e19637_d_n10, assign13890_e19637_d_n11, assign13890_e19637_d_n12, assign13890_e19637_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        let assign13890_e19633: f64 = 1e-5;
        let assign13890_e19635: f64 = (assign13890_e19633 - locals.var_rrr_eta);
        (assign13890_e19635, (-locals.var_rrr_eta_dn0), (-locals.var_rrr_eta_dn2), (-locals.var_rrr_eta_dn6), (-locals.var_rrr_eta_dn7), (-locals.var_rrr_eta_dn10), (-locals.var_rrr_eta_dn11), (-locals.var_rrr_eta_dn12), (-locals.var_rrr_eta_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign13890_e19637;
        locals.var_tmf1_dn0 = assign13890_e19637_d_n0;
        locals.var_tmf1_dn2 = assign13890_e19637_d_n2;
        locals.var_tmf1_dn6 = assign13890_e19637_d_n6;
        locals.var_tmf1_dn7 = assign13890_e19637_d_n7;
        locals.var_tmf1_dn10 = assign13890_e19637_d_n10;
        locals.var_tmf1_dn11 = assign13890_e19637_d_n11;
        locals.var_tmf1_dn12 = assign13890_e19637_d_n12;
        locals.var_tmf1_dn17 = assign13890_e19637_d_n17;

        let (assign13900_e19648, assign13900_e19648_d_n0, assign13900_e19648_d_n2, assign13900_e19648_d_n6, assign13900_e19648_d_n7, assign13900_e19648_d_n10, assign13900_e19648_d_n11, assign13900_e19648_d_n12, assign13900_e19648_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        let assign13900_e19646: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign13900_e19646, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign13900_e19648;
        locals.var_x2_dn0 = assign13900_e19648_d_n0;
        locals.var_x2_dn2 = assign13900_e19648_d_n2;
        locals.var_x2_dn6 = assign13900_e19648_d_n6;
        locals.var_x2_dn7 = assign13900_e19648_d_n7;
        locals.var_x2_dn10 = assign13900_e19648_d_n10;
        locals.var_x2_dn11 = assign13900_e19648_d_n11;
        locals.var_x2_dn12 = assign13900_e19648_d_n12;
        locals.var_x2_dn17 = assign13900_e19648_d_n17;

        let (assign13910_e19659, assign13910_e19659_d_n0, assign13910_e19659_d_n2, assign13910_e19659_d_n6, assign13910_e19659_d_n7, assign13910_e19659_d_n10, assign13910_e19659_d_n11, assign13910_e19659_d_n12, assign13910_e19659_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        let assign13910_e19657: f64 = (1e-5 * 1e-5);
        (assign13910_e19657, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign13910_e19659;
        locals.var_xmax2_dn0 = assign13910_e19659_d_n0;
        locals.var_xmax2_dn2 = assign13910_e19659_d_n2;
        locals.var_xmax2_dn6 = assign13910_e19659_d_n6;
        locals.var_xmax2_dn7 = assign13910_e19659_d_n7;
        locals.var_xmax2_dn10 = assign13910_e19659_d_n10;
        locals.var_xmax2_dn11 = assign13910_e19659_d_n11;
        locals.var_xmax2_dn12 = assign13910_e19659_d_n12;
        locals.var_xmax2_dn17 = assign13910_e19659_d_n17;

        let (assign13920_e19668, assign13920_e19668_d_n0, assign13920_e19668_d_n2, assign13920_e19668_d_n6, assign13920_e19668_d_n7, assign13920_e19668_d_n10, assign13920_e19668_d_n11, assign13920_e19668_d_n12, assign13920_e19668_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13920_e19668;
        locals.var_xp_dn0 = assign13920_e19668_d_n0;
        locals.var_xp_dn2 = assign13920_e19668_d_n2;
        locals.var_xp_dn6 = assign13920_e19668_d_n6;
        locals.var_xp_dn7 = assign13920_e19668_d_n7;
        locals.var_xp_dn10 = assign13920_e19668_d_n10;
        locals.var_xp_dn11 = assign13920_e19668_d_n11;
        locals.var_xp_dn12 = assign13920_e19668_d_n12;
        locals.var_xp_dn17 = assign13920_e19668_d_n17;

        let (assign13930_e19677, assign13930_e19677_d_n0, assign13930_e19677_d_n2, assign13930_e19677_d_n6, assign13930_e19677_d_n7, assign13930_e19677_d_n10, assign13930_e19677_d_n11, assign13930_e19677_d_n12, assign13930_e19677_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13930_e19677;
        locals.var_xmp_dn0 = assign13930_e19677_d_n0;
        locals.var_xmp_dn2 = assign13930_e19677_d_n2;
        locals.var_xmp_dn6 = assign13930_e19677_d_n6;
        locals.var_xmp_dn7 = assign13930_e19677_d_n7;
        locals.var_xmp_dn10 = assign13930_e19677_d_n10;
        locals.var_xmp_dn11 = assign13930_e19677_d_n11;
        locals.var_xmp_dn12 = assign13930_e19677_d_n12;
        locals.var_xmp_dn17 = assign13930_e19677_d_n17;

        let (assign13940_e19686,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign13940_e19686;

        let (assign13950_e19695,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13950_e19695;

        let (assign13960_e19704, assign13960_e19704_d_n0, assign13960_e19704_d_n2, assign13960_e19704_d_n6, assign13960_e19704_d_n7, assign13960_e19704_d_n10, assign13960_e19704_d_n11, assign13960_e19704_d_n12, assign13960_e19704_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign13960_e19704;
        locals.var_arg_dn0 = assign13960_e19704_d_n0;
        locals.var_arg_dn2 = assign13960_e19704_d_n2;
        locals.var_arg_dn6 = assign13960_e19704_d_n6;
        locals.var_arg_dn7 = assign13960_e19704_d_n7;
        locals.var_arg_dn10 = assign13960_e19704_d_n10;
        locals.var_arg_dn11 = assign13960_e19704_d_n11;
        locals.var_arg_dn12 = assign13960_e19704_d_n12;
        locals.var_arg_dn17 = assign13960_e19704_d_n17;

    }

    pub(super) fn stamp_transient_block_45(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13970_e19713, assign13970_e19713_d_n0, assign13970_e19713_d_n2, assign13970_e19713_d_n6, assign13970_e19713_d_n7, assign13970_e19713_d_n10, assign13970_e19713_d_n11, assign13970_e19713_d_n12, assign13970_e19713_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13970_e19713;
        locals.var_dnm_dn0 = assign13970_e19713_d_n0;
        locals.var_dnm_dn2 = assign13970_e19713_d_n2;
        locals.var_dnm_dn6 = assign13970_e19713_d_n6;
        locals.var_dnm_dn7 = assign13970_e19713_d_n7;
        locals.var_dnm_dn10 = assign13970_e19713_d_n10;
        locals.var_dnm_dn11 = assign13970_e19713_d_n11;
        locals.var_dnm_dn12 = assign13970_e19713_d_n12;
        locals.var_dnm_dn17 = assign13970_e19713_d_n17;

        let (assign13980_e19724, assign13980_e19724_d_n0, assign13980_e19724_d_n2, assign13980_e19724_d_n6, assign13980_e19724_d_n7, assign13980_e19724_d_n10, assign13980_e19724_d_n11, assign13980_e19724_d_n12, assign13980_e19724_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        let assign13980_e19722: f64 = (locals.var_xp * locals.var_x2);
        (assign13980_e19722, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13980_e19724;
        locals.var_xp_dn0 = assign13980_e19724_d_n0;
        locals.var_xp_dn2 = assign13980_e19724_d_n2;
        locals.var_xp_dn6 = assign13980_e19724_d_n6;
        locals.var_xp_dn7 = assign13980_e19724_d_n7;
        locals.var_xp_dn10 = assign13980_e19724_d_n10;
        locals.var_xp_dn11 = assign13980_e19724_d_n11;
        locals.var_xp_dn12 = assign13980_e19724_d_n12;
        locals.var_xp_dn17 = assign13980_e19724_d_n17;

        let (assign13990_e19735, assign13990_e19735_d_n0, assign13990_e19735_d_n2, assign13990_e19735_d_n6, assign13990_e19735_d_n7, assign13990_e19735_d_n10, assign13990_e19735_d_n11, assign13990_e19735_d_n12, assign13990_e19735_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        let assign13990_e19733: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign13990_e19733, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13990_e19735;
        locals.var_xmp_dn0 = assign13990_e19735_d_n0;
        locals.var_xmp_dn2 = assign13990_e19735_d_n2;
        locals.var_xmp_dn6 = assign13990_e19735_d_n6;
        locals.var_xmp_dn7 = assign13990_e19735_d_n7;
        locals.var_xmp_dn10 = assign13990_e19735_d_n10;
        locals.var_xmp_dn11 = assign13990_e19735_d_n11;
        locals.var_xmp_dn12 = assign13990_e19735_d_n12;
        locals.var_xmp_dn17 = assign13990_e19735_d_n17;

        let (assign14000_e19746, assign14000_e19746_d_n0, assign14000_e19746_d_n2, assign14000_e19746_d_n6, assign14000_e19746_d_n7, assign14000_e19746_d_n10, assign14000_e19746_d_n11, assign14000_e19746_d_n12, assign14000_e19746_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        let assign14000_e19744: f64 = (locals.var_xp * locals.var_x2);
        (assign14000_e19744, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign14000_e19746;
        locals.var_xp_dn0 = assign14000_e19746_d_n0;
        locals.var_xp_dn2 = assign14000_e19746_d_n2;
        locals.var_xp_dn6 = assign14000_e19746_d_n6;
        locals.var_xp_dn7 = assign14000_e19746_d_n7;
        locals.var_xp_dn10 = assign14000_e19746_d_n10;
        locals.var_xp_dn11 = assign14000_e19746_d_n11;
        locals.var_xp_dn12 = assign14000_e19746_d_n12;
        locals.var_xp_dn17 = assign14000_e19746_d_n17;

        let (assign14010_e19757, assign14010_e19757_d_n0, assign14010_e19757_d_n2, assign14010_e19757_d_n6, assign14010_e19757_d_n7, assign14010_e19757_d_n10, assign14010_e19757_d_n11, assign14010_e19757_d_n12, assign14010_e19757_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        let assign14010_e19755: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign14010_e19755, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign14010_e19757;
        locals.var_xmp_dn0 = assign14010_e19757_d_n0;
        locals.var_xmp_dn2 = assign14010_e19757_d_n2;
        locals.var_xmp_dn6 = assign14010_e19757_d_n6;
        locals.var_xmp_dn7 = assign14010_e19757_d_n7;
        locals.var_xmp_dn10 = assign14010_e19757_d_n10;
        locals.var_xmp_dn11 = assign14010_e19757_d_n11;
        locals.var_xmp_dn12 = assign14010_e19757_d_n12;
        locals.var_xmp_dn17 = assign14010_e19757_d_n17;

        let (assign14020_e19768, assign14020_e19768_d_n0, assign14020_e19768_d_n2, assign14020_e19768_d_n6, assign14020_e19768_d_n7, assign14020_e19768_d_n10, assign14020_e19768_d_n11, assign14020_e19768_d_n12, assign14020_e19768_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        let assign14020_e19766: f64 = (locals.var_xp + locals.var_xmp);
        (assign14020_e19766, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign14020_e19768;
        locals.var_arg_dn0 = assign14020_e19768_d_n0;
        locals.var_arg_dn2 = assign14020_e19768_d_n2;
        locals.var_arg_dn6 = assign14020_e19768_d_n6;
        locals.var_arg_dn7 = assign14020_e19768_d_n7;
        locals.var_arg_dn10 = assign14020_e19768_d_n10;
        locals.var_arg_dn11 = assign14020_e19768_d_n11;
        locals.var_arg_dn12 = assign14020_e19768_d_n12;
        locals.var_arg_dn17 = assign14020_e19768_d_n17;

        let (assign14030_e19777, assign14030_e19777_d_n0, assign14030_e19777_d_n2, assign14030_e19777_d_n6, assign14030_e19777_d_n7, assign14030_e19777_d_n10, assign14030_e19777_d_n11, assign14030_e19777_d_n12, assign14030_e19777_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign14030_e19777;
        locals.var_dnm_dn0 = assign14030_e19777_d_n0;
        locals.var_dnm_dn2 = assign14030_e19777_d_n2;
        locals.var_dnm_dn6 = assign14030_e19777_d_n6;
        locals.var_dnm_dn7 = assign14030_e19777_d_n7;
        locals.var_dnm_dn10 = assign14030_e19777_d_n10;
        locals.var_dnm_dn11 = assign14030_e19777_d_n11;
        locals.var_dnm_dn12 = assign14030_e19777_d_n12;
        locals.var_dnm_dn17 = assign14030_e19777_d_n17;

        let assign14040_e19792: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard432 = assign14040_e19792;

        let assign14050_e19795: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard433 = assign14050_e19795;

        let (assign14060_e19808,) = {
    if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) && (locals.var_guard432 != 0.0)) && (locals.var_guard433 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign14060_e19808;

        let assign14070_e19811: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard434 = assign14070_e19811;

        let (assign14080_e19827,) = {
    if ((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) && (locals.var_guard432 != 0.0)) && (locals.var_guard433 == 0.0)) && (locals.var_guard434 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign14080_e19827;

        let assign14090_e19830: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard435 = assign14090_e19830;

        let (assign14100_e19849,) = {
    if (((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) && (locals.var_guard432 != 0.0)) && (locals.var_guard433 == 0.0)) && (locals.var_guard434 == 0.0)) && (locals.var_guard435 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign14100_e19849;

        let assign14110_e19852: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard436 = assign14110_e19852;

        let (assign14120_e19874,) = {
    if ((((((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) && (locals.var_guard432 != 0.0)) && (locals.var_guard433 == 0.0)) && (locals.var_guard434 == 0.0)) && (locals.var_guard435 == 0.0)) && (locals.var_guard436 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign14120_e19874;

        let (assign14130_e19885,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) && (locals.var_guard432 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign14130_e19885;

        let mut assign14140_loop_guard: usize = 0;
        while {
            let assign14140_cond_e19897: f64 = if (((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) && (locals.var_guard432 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign14140_cond_e19897 != 0.0
        } {
            assign14140_loop_guard += 1;
            assert!(assign14140_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign14140_body0_e19909, assign14140_body0_e19909_d_n0, assign14140_body0_e19909_d_n2, assign14140_body0_e19909_d_n6, assign14140_body0_e19909_d_n7, assign14140_body0_e19909_d_n10, assign14140_body0_e19909_d_n11, assign14140_body0_e19909_d_n12, assign14140_body0_e19909_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) && (locals.var_guard432 != 0.0)) {
        let assign14140_body0_e19907: f64 = (locals.var_dnm).sqrt();
        (assign14140_body0_e19907, (locals.var_dnm_dn0 / (2.0 * assign14140_body0_e19907)), (locals.var_dnm_dn2 / (2.0 * assign14140_body0_e19907)), (locals.var_dnm_dn6 / (2.0 * assign14140_body0_e19907)), (locals.var_dnm_dn7 / (2.0 * assign14140_body0_e19907)), (locals.var_dnm_dn10 / (2.0 * assign14140_body0_e19907)), (locals.var_dnm_dn11 / (2.0 * assign14140_body0_e19907)), (locals.var_dnm_dn12 / (2.0 * assign14140_body0_e19907)), (locals.var_dnm_dn17 / (2.0 * assign14140_body0_e19907)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign14140_body0_e19909;
            locals.var_dnm_dn0 = assign14140_body0_e19909_d_n0;
            locals.var_dnm_dn2 = assign14140_body0_e19909_d_n2;
            locals.var_dnm_dn6 = assign14140_body0_e19909_d_n6;
            locals.var_dnm_dn7 = assign14140_body0_e19909_d_n7;
            locals.var_dnm_dn10 = assign14140_body0_e19909_d_n10;
            locals.var_dnm_dn11 = assign14140_body0_e19909_d_n11;
            locals.var_dnm_dn12 = assign14140_body0_e19909_d_n12;
            locals.var_dnm_dn17 = assign14140_body0_e19909_d_n17;
            let (assign14140_body1_e19922,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) && (locals.var_guard432 != 0.0)) {
        let assign14140_body1_e19920: f64 = (locals.var_m0 + 1.0);
        (assign14140_body1_e19920,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign14140_body1_e19922;
        }

        let (assign14150_e19940, assign14150_e19940_d_n0, assign14150_e19940_d_n2, assign14150_e19940_d_n6, assign14150_e19940_d_n7, assign14150_e19940_d_n10, assign14150_e19940_d_n11, assign14150_e19940_d_n12, assign14150_e19940_d_n17,) = {
    if ((((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) && (locals.var_guard432 == 0.0)) {
        let assign14150_e19936: f64 = (2.0 * 2.0);
        let assign14150_e19937: f64 = (1.0 / assign14150_e19936);
        let assign14150_e19938: f64 = (locals.var_dnm).powf(assign14150_e19937);
        (assign14150_e19938, if 0.0 == 0.0 && ((assign14150_e19937) as f64).is_finite() && ((assign14150_e19937) as f64).fract() == 0.0 { if assign14150_e19937 == 0.0 { 0.0 } else { (assign14150_e19937 * ((locals.var_dnm).powf(assign14150_e19937 - 1.0) * locals.var_dnm_dn0)) } } else { (assign14150_e19938 * (assign14150_e19937 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14150_e19937) as f64).is_finite() && ((assign14150_e19937) as f64).fract() == 0.0 { if assign14150_e19937 == 0.0 { 0.0 } else { (assign14150_e19937 * ((locals.var_dnm).powf(assign14150_e19937 - 1.0) * locals.var_dnm_dn2)) } } else { (assign14150_e19938 * (assign14150_e19937 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14150_e19937) as f64).is_finite() && ((assign14150_e19937) as f64).fract() == 0.0 { if assign14150_e19937 == 0.0 { 0.0 } else { (assign14150_e19937 * ((locals.var_dnm).powf(assign14150_e19937 - 1.0) * locals.var_dnm_dn6)) } } else { (assign14150_e19938 * (assign14150_e19937 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14150_e19937) as f64).is_finite() && ((assign14150_e19937) as f64).fract() == 0.0 { if assign14150_e19937 == 0.0 { 0.0 } else { (assign14150_e19937 * ((locals.var_dnm).powf(assign14150_e19937 - 1.0) * locals.var_dnm_dn7)) } } else { (assign14150_e19938 * (assign14150_e19937 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14150_e19937) as f64).is_finite() && ((assign14150_e19937) as f64).fract() == 0.0 { if assign14150_e19937 == 0.0 { 0.0 } else { (assign14150_e19937 * ((locals.var_dnm).powf(assign14150_e19937 - 1.0) * locals.var_dnm_dn10)) } } else { (assign14150_e19938 * (assign14150_e19937 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14150_e19937) as f64).is_finite() && ((assign14150_e19937) as f64).fract() == 0.0 { if assign14150_e19937 == 0.0 { 0.0 } else { (assign14150_e19937 * ((locals.var_dnm).powf(assign14150_e19937 - 1.0) * locals.var_dnm_dn11)) } } else { (assign14150_e19938 * (assign14150_e19937 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14150_e19937) as f64).is_finite() && ((assign14150_e19937) as f64).fract() == 0.0 { if assign14150_e19937 == 0.0 { 0.0 } else { (assign14150_e19937 * ((locals.var_dnm).powf(assign14150_e19937 - 1.0) * locals.var_dnm_dn12)) } } else { (assign14150_e19938 * (assign14150_e19937 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14150_e19937) as f64).is_finite() && ((assign14150_e19937) as f64).fract() == 0.0 { if assign14150_e19937 == 0.0 { 0.0 } else { (assign14150_e19937 * ((locals.var_dnm).powf(assign14150_e19937 - 1.0) * locals.var_dnm_dn17)) } } else { (assign14150_e19938 * (assign14150_e19937 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign14150_e19940;
        locals.var_dnm_dn0 = assign14150_e19940_d_n0;
        locals.var_dnm_dn2 = assign14150_e19940_d_n2;
        locals.var_dnm_dn6 = assign14150_e19940_d_n6;
        locals.var_dnm_dn7 = assign14150_e19940_d_n7;
        locals.var_dnm_dn10 = assign14150_e19940_d_n10;
        locals.var_dnm_dn11 = assign14150_e19940_d_n11;
        locals.var_dnm_dn12 = assign14150_e19940_d_n12;
        locals.var_dnm_dn17 = assign14150_e19940_d_n17;

        let (assign14160_e19951, assign14160_e19951_d_n0, assign14160_e19951_d_n2, assign14160_e19951_d_n6, assign14160_e19951_d_n7, assign14160_e19951_d_n10, assign14160_e19951_d_n11, assign14160_e19951_d_n12, assign14160_e19951_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        let assign14160_e19949: f64 = (1.0 / locals.var_dnm);
        (assign14160_e19949, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign14160_e19951;
        locals.var_dnm_dn0 = assign14160_e19951_d_n0;
        locals.var_dnm_dn2 = assign14160_e19951_d_n2;
        locals.var_dnm_dn6 = assign14160_e19951_d_n6;
        locals.var_dnm_dn7 = assign14160_e19951_d_n7;
        locals.var_dnm_dn10 = assign14160_e19951_d_n10;
        locals.var_dnm_dn11 = assign14160_e19951_d_n11;
        locals.var_dnm_dn12 = assign14160_e19951_d_n12;
        locals.var_dnm_dn17 = assign14160_e19951_d_n17;

        let (assign14170_e19964, assign14170_e19964_d_n0, assign14170_e19964_d_n2, assign14170_e19964_d_n6, assign14170_e19964_d_n7, assign14170_e19964_d_n10, assign14170_e19964_d_n11, assign14170_e19964_d_n12, assign14170_e19964_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        let assign14170_e19960: f64 = (locals.var_tmf1 * 1e-5);
        let assign14170_e19962: f64 = (assign14170_e19960 * locals.var_dnm);
        (assign14170_e19962, (((locals.var_tmf1_dn0 * 1e-5) * locals.var_dnm) + (assign14170_e19960 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-5) * locals.var_dnm) + (assign14170_e19960 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn6 * 1e-5) * locals.var_dnm) + (assign14170_e19960 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-5) * locals.var_dnm) + (assign14170_e19960 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn10 * 1e-5) * locals.var_dnm) + (assign14170_e19960 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1e-5) * locals.var_dnm) + (assign14170_e19960 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn12 * 1e-5) * locals.var_dnm) + (assign14170_e19960 * locals.var_dnm_dn12)), (((locals.var_tmf1_dn17 * 1e-5) * locals.var_dnm) + (assign14170_e19960 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign14170_e19964;
        locals.var_tmf0_dn0 = assign14170_e19964_d_n0;
        locals.var_tmf0_dn2 = assign14170_e19964_d_n2;
        locals.var_tmf0_dn6 = assign14170_e19964_d_n6;
        locals.var_tmf0_dn7 = assign14170_e19964_d_n7;
        locals.var_tmf0_dn10 = assign14170_e19964_d_n10;
        locals.var_tmf0_dn11 = assign14170_e19964_d_n11;
        locals.var_tmf0_dn12 = assign14170_e19964_d_n12;
        locals.var_tmf0_dn17 = assign14170_e19964_d_n17;

        let (assign14180_e19977, assign14180_e19977_d_n0, assign14180_e19977_d_n2, assign14180_e19977_d_n6, assign14180_e19977_d_n7, assign14180_e19977_d_n10, assign14180_e19977_d_n11, assign14180_e19977_d_n12, assign14180_e19977_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 != 0.0)) {
        let assign14180_e19973: f64 = 1e-5;
        let assign14180_e19975: f64 = (assign14180_e19973 - locals.var_tmf0);
        (assign14180_e19975, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn12), (-locals.var_tmf0_dn17),)
    } else {
        (locals.var_rrr_eta, locals.var_rrr_eta_dn0, locals.var_rrr_eta_dn2, locals.var_rrr_eta_dn6, locals.var_rrr_eta_dn7, locals.var_rrr_eta_dn10, locals.var_rrr_eta_dn11, locals.var_rrr_eta_dn12, locals.var_rrr_eta_dn17,)
    }
};
        locals.var_rrr_eta = assign14180_e19977;
        locals.var_rrr_eta_dn0 = assign14180_e19977_d_n0;
        locals.var_rrr_eta_dn2 = assign14180_e19977_d_n2;
        locals.var_rrr_eta_dn6 = assign14180_e19977_d_n6;
        locals.var_rrr_eta_dn7 = assign14180_e19977_d_n7;
        locals.var_rrr_eta_dn10 = assign14180_e19977_d_n10;
        locals.var_rrr_eta_dn11 = assign14180_e19977_d_n11;
        locals.var_rrr_eta_dn12 = assign14180_e19977_d_n12;
        locals.var_rrr_eta_dn17 = assign14180_e19977_d_n17;

        let (assign14190_e19987, assign14190_e19987_d_n0, assign14190_e19987_d_n2, assign14190_e19987_d_n6, assign14190_e19987_d_n7, assign14190_e19987_d_n10, assign14190_e19987_d_n11, assign14190_e19987_d_n12, assign14190_e19987_d_n17,) = {
    if (((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) && (locals.var_guard431 == 0.0)) {
        (locals.var_rrr_eta, locals.var_rrr_eta_dn0, locals.var_rrr_eta_dn2, locals.var_rrr_eta_dn6, locals.var_rrr_eta_dn7, locals.var_rrr_eta_dn10, locals.var_rrr_eta_dn11, locals.var_rrr_eta_dn12, locals.var_rrr_eta_dn17,)
    } else {
        (locals.var_rrr_eta, locals.var_rrr_eta_dn0, locals.var_rrr_eta_dn2, locals.var_rrr_eta_dn6, locals.var_rrr_eta_dn7, locals.var_rrr_eta_dn10, locals.var_rrr_eta_dn11, locals.var_rrr_eta_dn12, locals.var_rrr_eta_dn17,)
    }
};
        locals.var_rrr_eta = assign14190_e19987;
        locals.var_rrr_eta_dn0 = assign14190_e19987_d_n0;
        locals.var_rrr_eta_dn2 = assign14190_e19987_d_n2;
        locals.var_rrr_eta_dn6 = assign14190_e19987_d_n6;
        locals.var_rrr_eta_dn7 = assign14190_e19987_d_n7;
        locals.var_rrr_eta_dn10 = assign14190_e19987_d_n10;
        locals.var_rrr_eta_dn11 = assign14190_e19987_d_n11;
        locals.var_rrr_eta_dn12 = assign14190_e19987_d_n12;
        locals.var_rrr_eta_dn17 = assign14190_e19987_d_n17;

        let (assign14200_e19994, assign14200_e19994_d_n0, assign14200_e19994_d_n2, assign14200_e19994_d_n6, assign14200_e19994_d_n7, assign14200_e19994_d_n10, assign14200_e19994_d_n11, assign14200_e19994_d_n12, assign14200_e19994_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        (locals.var_rrr_eta, locals.var_rrr_eta_dn0, locals.var_rrr_eta_dn2, locals.var_rrr_eta_dn6, locals.var_rrr_eta_dn7, locals.var_rrr_eta_dn10, locals.var_rrr_eta_dn11, locals.var_rrr_eta_dn12, locals.var_rrr_eta_dn17,)
    } else {
        (locals.var_alpha, locals.var_alpha_dn0, locals.var_alpha_dn2, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn10, locals.var_alpha_dn11, locals.var_alpha_dn12, locals.var_alpha_dn17,)
    }
};
        locals.var_alpha = assign14200_e19994;
        locals.var_alpha_dn0 = assign14200_e19994_d_n0;
        locals.var_alpha_dn2 = assign14200_e19994_d_n2;
        locals.var_alpha_dn6 = assign14200_e19994_d_n6;
        locals.var_alpha_dn7 = assign14200_e19994_d_n7;
        locals.var_alpha_dn10 = assign14200_e19994_d_n10;
        locals.var_alpha_dn11 = assign14200_e19994_d_n11;
        locals.var_alpha_dn12 = assign14200_e19994_d_n12;
        locals.var_alpha_dn17 = assign14200_e19994_d_n17;

        let (assign14210_e20007, assign14210_e20007_d_n0, assign14210_e20007_d_n2, assign14210_e20007_d_n6, assign14210_e20007_d_n7, assign14210_e20007_d_n10, assign14210_e20007_d_n11, assign14210_e20007_d_n12, assign14210_e20007_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign14210_e20003: f64 = (1.0 + locals.var_alpha);
        let assign14210_e20004: f64 = (locals.var_alpha * assign14210_e20003);
        let assign14210_e20005: f64 = (1.0 + assign14210_e20004);
        (assign14210_e20005, ((locals.var_alpha_dn0 * assign14210_e20003) + (locals.var_alpha * locals.var_alpha_dn0)), ((locals.var_alpha_dn2 * assign14210_e20003) + (locals.var_alpha * locals.var_alpha_dn2)), ((locals.var_alpha_dn6 * assign14210_e20003) + (locals.var_alpha * locals.var_alpha_dn6)), ((locals.var_alpha_dn7 * assign14210_e20003) + (locals.var_alpha * locals.var_alpha_dn7)), ((locals.var_alpha_dn10 * assign14210_e20003) + (locals.var_alpha * locals.var_alpha_dn10)), ((locals.var_alpha_dn11 * assign14210_e20003) + (locals.var_alpha * locals.var_alpha_dn11)), ((locals.var_alpha_dn12 * assign14210_e20003) + (locals.var_alpha * locals.var_alpha_dn12)), ((locals.var_alpha_dn17 * assign14210_e20003) + (locals.var_alpha * locals.var_alpha_dn17)),)
    } else {
        (locals.var_qinm, locals.var_qinm_dn0, locals.var_qinm_dn2, locals.var_qinm_dn6, locals.var_qinm_dn7, locals.var_qinm_dn10, locals.var_qinm_dn11, locals.var_qinm_dn12, locals.var_qinm_dn17,)
    }
};
        locals.var_qinm = assign14210_e20007;
        locals.var_qinm_dn0 = assign14210_e20007_d_n0;
        locals.var_qinm_dn2 = assign14210_e20007_d_n2;
        locals.var_qinm_dn6 = assign14210_e20007_d_n6;
        locals.var_qinm_dn7 = assign14210_e20007_d_n7;
        locals.var_qinm_dn10 = assign14210_e20007_d_n10;
        locals.var_qinm_dn11 = assign14210_e20007_d_n11;
        locals.var_qinm_dn12 = assign14210_e20007_d_n12;
        locals.var_qinm_dn17 = assign14210_e20007_d_n17;

        let (assign14220_e20027, assign14220_e20027_d_n0, assign14220_e20027_d_n2, assign14220_e20027_d_n6, assign14220_e20027_d_n7, assign14220_e20027_d_n10, assign14220_e20027_d_n11, assign14220_e20027_d_n12, assign14220_e20027_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign14220_e20014: f64 = (1.0 + locals.var_alpha);
        let assign14220_e20017: f64 = (10.0 * 2.220446049250313e-16);
        let (assign14220_e20025, assign14220_e20025_d_n0, assign14220_e20025_d_n2, assign14220_e20025_d_n6, assign14220_e20025_d_n7, assign14220_e20025_d_n10, assign14220_e20025_d_n11, assign14220_e20025_d_n12, assign14220_e20025_d_n17,) = {
            if (assign14220_e20014 >= assign14220_e20017) {
                let assign14220_e20021: f64 = (1.0 + locals.var_alpha);
                (assign14220_e20021, locals.var_alpha_dn0, locals.var_alpha_dn2, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn10, locals.var_alpha_dn11, locals.var_alpha_dn12, locals.var_alpha_dn17,)
            } else {
                let assign14220_e20024: f64 = (10.0 * 2.220446049250313e-16);
                (assign14220_e20024, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign14220_e20025, assign14220_e20025_d_n0, assign14220_e20025_d_n2, assign14220_e20025_d_n6, assign14220_e20025_d_n7, assign14220_e20025_d_n10, assign14220_e20025_d_n11, assign14220_e20025_d_n12, assign14220_e20025_d_n17,)
    } else {
        (locals.var_qidn, locals.var_qidn_dn0, locals.var_qidn_dn2, locals.var_qidn_dn6, locals.var_qidn_dn7, locals.var_qidn_dn10, locals.var_qidn_dn11, locals.var_qidn_dn12, locals.var_qidn_dn17,)
    }
};
        locals.var_qidn = assign14220_e20027;
        locals.var_qidn_dn0 = assign14220_e20027_d_n0;
        locals.var_qidn_dn2 = assign14220_e20027_d_n2;
        locals.var_qidn_dn6 = assign14220_e20027_d_n6;
        locals.var_qidn_dn7 = assign14220_e20027_d_n7;
        locals.var_qidn_dn10 = assign14220_e20027_d_n10;
        locals.var_qidn_dn11 = assign14220_e20027_d_n11;
        locals.var_qidn_dn12 = assign14220_e20027_d_n12;
        locals.var_qidn_dn17 = assign14220_e20027_d_n17;

        let (assign14230_e20039, assign14230_e20039_d_n0, assign14230_e20039_d_n2, assign14230_e20039_d_n6, assign14230_e20039_d_n7, assign14230_e20039_d_n10, assign14230_e20039_d_n11, assign14230_e20039_d_n12, assign14230_e20039_d_n17,) = {
    if ((locals.var_guard113 != 0.0) && (locals.var_guard305 == 0.0)) {
        let assign14230_e20033: f64 = (-0.5);
        let assign14230_e20036: f64 = (locals.var_q_n0 + locals.var_q_nl);
        let assign14230_e20037: f64 = (assign14230_e20033 * assign14230_e20036);
        (assign14230_e20037, (assign14230_e20033 * (locals.var_q_n0_dn0 + locals.var_q_nl_dn0)), (assign14230_e20033 * (locals.var_q_n0_dn2 + locals.var_q_nl_dn2)), (assign14230_e20033 * (locals.var_q_n0_dn6 + locals.var_q_nl_dn6)), (assign14230_e20033 * (locals.var_q_n0_dn7 + locals.var_q_nl_dn7)), (assign14230_e20033 * (locals.var_q_n0_dn10 + locals.var_q_nl_dn10)), (assign14230_e20033 * (locals.var_q_n0_dn11 + locals.var_q_nl_dn11)), (assign14230_e20033 * (locals.var_q_n0_dn12 + locals.var_q_nl_dn12)), (assign14230_e20033 * (locals.var_q_n0_dn17 + locals.var_q_nl_dn17)),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn12, locals.var_qiu_dn17,)
    }
};
        locals.var_qiu = assign14230_e20039;
        locals.var_qiu_dn0 = assign14230_e20039_d_n0;
        locals.var_qiu_dn2 = assign14230_e20039_d_n2;
        locals.var_qiu_dn6 = assign14230_e20039_d_n6;
        locals.var_qiu_dn7 = assign14230_e20039_d_n7;
        locals.var_qiu_dn10 = assign14230_e20039_d_n10;
        locals.var_qiu_dn11 = assign14230_e20039_d_n11;
        locals.var_qiu_dn12 = assign14230_e20039_d_n12;
        locals.var_qiu_dn17 = assign14230_e20039_d_n17;

        let (assign14300_e20072, assign14300_e20072_d_n0, assign14300_e20072_d_n2, assign14300_e20072_d_n6, assign14300_e20072_d_n7, assign14300_e20072_d_n10, assign14300_e20072_d_n11, assign14300_e20072_d_n12, assign14300_e20072_d_n17,) = {
    if (locals.var_guard113 == 0.0) {
        (locals.var_vbsc, locals.var_vbsc_dn0, locals.var_vbsc_dn2, locals.var_vbsc_dn6, locals.var_vbsc_dn7, locals.var_vbsc_dn10, locals.var_vbsc_dn11, locals.var_vbsc_dn12, locals.var_vbsc_dn17,)
    } else {
        (locals.var_vbcs_cl, locals.var_vbcs_cl_dn0, locals.var_vbcs_cl_dn2, locals.var_vbcs_cl_dn6, locals.var_vbcs_cl_dn7, locals.var_vbcs_cl_dn10, locals.var_vbcs_cl_dn11, locals.var_vbcs_cl_dn12, locals.var_vbcs_cl_dn17,)
    }
};
        locals.var_vbcs_cl = assign14300_e20072;
        locals.var_vbcs_cl_dn0 = assign14300_e20072_d_n0;
        locals.var_vbcs_cl_dn2 = assign14300_e20072_d_n2;
        locals.var_vbcs_cl_dn6 = assign14300_e20072_d_n6;
        locals.var_vbcs_cl_dn7 = assign14300_e20072_d_n7;
        locals.var_vbcs_cl_dn10 = assign14300_e20072_d_n10;
        locals.var_vbcs_cl_dn11 = assign14300_e20072_d_n11;
        locals.var_vbcs_cl_dn12 = assign14300_e20072_d_n12;
        locals.var_vbcs_cl_dn17 = assign14300_e20072_d_n17;

        let assign14310_e20075: f64 = if locals.var_wdsoi_ini < p.p237 { 1.0 } else { 0.0 };
        locals.var_guard443 = assign14310_e20075;

        let (assign14320_e20082,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard443 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_depmode,)
    }
};
        locals.var_flg_depmode = assign14320_e20082;

        let (assign14330_e20090,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard443 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_flg_depmode,)
    }
};
        locals.var_flg_depmode = assign14330_e20090;

        let (assign14340_e20101,) = {
    if (locals.var_guard113 == 0.0) {
        let assign14340_e20095: f64 = (locals.var_vfb - locals.var_dvth);
        let assign14340_e20097: f64 = (assign14340_e20095 + locals.var_dppg);
        let assign14340_e20099: f64 = (assign14340_e20097 + locals.var_vbcs_cl);
        (assign14340_e20099,)
    } else {
        (locals.var_vgs_fb,)
    }
};
        locals.var_vgs_fb = assign14340_e20101;

        let assign14350_e20104: f64 = if locals.var_vgs < locals.var_vgs_fb { 1.0 } else { 0.0 };
        locals.var_guard444 = assign14350_e20104;

        let (assign14360_e20112,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14360_e20110: f64 = (-1.0);
        (assign14360_e20110,)
    } else {
        (locals.var_flg_zone,)
    }
};
        locals.var_flg_zone = assign14360_e20112;

        let (assign14370_e20127, assign14370_e20127_d_n0, assign14370_e20127_d_n2, assign14370_e20127_d_n6, assign14370_e20127_d_n7, assign14370_e20127_d_n10, assign14370_e20127_d_n11, assign14370_e20127_d_n12, assign14370_e20127_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14370_e20119: f64 = (2.0 * locals.var_beta_inv);
        let assign14370_e20121: f64 = (-locals.var_vgs_min);
        let assign14370_e20123: f64 = (assign14370_e20121 / locals.var_fac1);
        let assign14370_e20124: f64 = (assign14370_e20123).ln();
        let assign14370_e20125: f64 = (assign14370_e20119 * assign14370_e20124);
        (assign14370_e20125, (assign14370_e20119 * ((-((assign14370_e20121 * locals.var_fac1_dn0) / (locals.var_fac1 * locals.var_fac1))) / assign14370_e20123)), (assign14370_e20119 * ((-((assign14370_e20121 * locals.var_fac1_dn2) / (locals.var_fac1 * locals.var_fac1))) / assign14370_e20123)), (assign14370_e20119 * ((-((assign14370_e20121 * locals.var_fac1_dn6) / (locals.var_fac1 * locals.var_fac1))) / assign14370_e20123)), (assign14370_e20119 * ((-((assign14370_e20121 * locals.var_fac1_dn7) / (locals.var_fac1 * locals.var_fac1))) / assign14370_e20123)), (((2.0 * locals.var_beta_inv_dn10) * assign14370_e20124) + (assign14370_e20119 * ((-((assign14370_e20121 * locals.var_fac1_dn10) / (locals.var_fac1 * locals.var_fac1))) / assign14370_e20123))), (assign14370_e20119 * ((-((assign14370_e20121 * locals.var_fac1_dn11) / (locals.var_fac1 * locals.var_fac1))) / assign14370_e20123)), (assign14370_e20119 * ((-((assign14370_e20121 * locals.var_fac1_dn12) / (locals.var_fac1 * locals.var_fac1))) / assign14370_e20123)), (assign14370_e20119 * ((-((assign14370_e20121 * locals.var_fac1_dn17) / (locals.var_fac1 * locals.var_fac1))) / assign14370_e20123)),)
    } else {
        (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn6, locals.var_ps0_min_dn7, locals.var_ps0_min_dn10, locals.var_ps0_min_dn11, locals.var_ps0_min_dn12, locals.var_ps0_min_dn17,)
    }
};
        locals.var_ps0_min = assign14370_e20127;
        locals.var_ps0_min_dn0 = assign14370_e20127_d_n0;
        locals.var_ps0_min_dn2 = assign14370_e20127_d_n2;
        locals.var_ps0_min_dn6 = assign14370_e20127_d_n6;
        locals.var_ps0_min_dn7 = assign14370_e20127_d_n7;
        locals.var_ps0_min_dn10 = assign14370_e20127_d_n10;
        locals.var_ps0_min_dn11 = assign14370_e20127_d_n11;
        locals.var_ps0_min_dn12 = assign14370_e20127_d_n12;
        locals.var_ps0_min_dn17 = assign14370_e20127_d_n17;

        let (assign14380_e20138, assign14380_e20138_d_n0, assign14380_e20138_d_n2, assign14380_e20138_d_n6, assign14380_e20138_d_n7, assign14380_e20138_d_n10, assign14380_e20138_d_n11, assign14380_e20138_d_n12, assign14380_e20138_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14380_e20135: f64 = (locals.var_vgp - locals.var_vbcs_cl);
        let assign14380_e20136: f64 = (locals.var_beta * assign14380_e20135);
        (assign14380_e20136, (locals.var_beta * (locals.var_vgp_dn0 - locals.var_vbcs_cl_dn0)), (locals.var_beta * (locals.var_vgp_dn2 - locals.var_vbcs_cl_dn2)), (locals.var_beta * (locals.var_vgp_dn6 - locals.var_vbcs_cl_dn6)), (locals.var_beta * (locals.var_vgp_dn7 - locals.var_vbcs_cl_dn7)), ((locals.var_beta_dn10 * assign14380_e20135) + (locals.var_beta * (locals.var_vgp_dn10 - locals.var_vbcs_cl_dn10))), (locals.var_beta * (locals.var_vgp_dn11 - locals.var_vbcs_cl_dn11)), (locals.var_beta * (locals.var_vgp_dn12 - locals.var_vbcs_cl_dn12)), (locals.var_beta * (locals.var_vgp_dn17 - locals.var_vbcs_cl_dn17)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign14380_e20138;
        locals.var_tx_dn0 = assign14380_e20138_d_n0;
        locals.var_tx_dn2 = assign14380_e20138_d_n2;
        locals.var_tx_dn6 = assign14380_e20138_d_n6;
        locals.var_tx_dn7 = assign14380_e20138_d_n7;
        locals.var_tx_dn10 = assign14380_e20138_d_n10;
        locals.var_tx_dn11 = assign14380_e20138_d_n11;
        locals.var_tx_dn12 = assign14380_e20138_d_n12;
        locals.var_tx_dn17 = assign14380_e20138_d_n17;

    }

    pub(super) fn stamp_transient_block_46(
        locals: &mut StampLocals,
    ) {
        let (assign14390_e20149, assign14390_e20149_d_n0, assign14390_e20149_d_n2, assign14390_e20149_d_n6, assign14390_e20149_d_n7, assign14390_e20149_d_n10, assign14390_e20149_d_n11, assign14390_e20149_d_n12, assign14390_e20149_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14390_e20146: f64 = (locals.var_beta * locals.var_cnst0soi);
        let assign14390_e20147: f64 = (1.0 / assign14390_e20146);
        (assign14390_e20147, (-((locals.var_beta * locals.var_cnst0soi_dn0) / (assign14390_e20146 * assign14390_e20146))), (-((locals.var_beta * locals.var_cnst0soi_dn2) / (assign14390_e20146 * assign14390_e20146))), (-((locals.var_beta * locals.var_cnst0soi_dn6) / (assign14390_e20146 * assign14390_e20146))), (-((locals.var_beta * locals.var_cnst0soi_dn7) / (assign14390_e20146 * assign14390_e20146))), (-(((locals.var_beta_dn10 * locals.var_cnst0soi) + (locals.var_beta * locals.var_cnst0soi_dn10)) / (assign14390_e20146 * assign14390_e20146))), (-((locals.var_beta * locals.var_cnst0soi_dn11) / (assign14390_e20146 * assign14390_e20146))), (-((locals.var_beta * locals.var_cnst0soi_dn12) / (assign14390_e20146 * assign14390_e20146))), (-((locals.var_beta * locals.var_cnst0soi_dn17) / (assign14390_e20146 * assign14390_e20146))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign14390_e20149;
        locals.var_t1_dn0 = assign14390_e20149_d_n0;
        locals.var_t1_dn2 = assign14390_e20149_d_n2;
        locals.var_t1_dn6 = assign14390_e20149_d_n6;
        locals.var_t1_dn7 = assign14390_e20149_d_n7;
        locals.var_t1_dn10 = assign14390_e20149_d_n10;
        locals.var_t1_dn11 = assign14390_e20149_d_n11;
        locals.var_t1_dn12 = assign14390_e20149_d_n12;
        locals.var_t1_dn17 = assign14390_e20149_d_n17;

        let (assign14400_e20158, assign14400_e20158_d_n0, assign14400_e20158_d_n2, assign14400_e20158_d_n6, assign14400_e20158_d_n7, assign14400_e20158_d_n10, assign14400_e20158_d_n11, assign14400_e20158_d_n12, assign14400_e20158_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14400_e20156: f64 = (locals.var_t1 * locals.var_c_fox);
        (assign14400_e20156, ((locals.var_t1_dn0 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn0)), ((locals.var_t1_dn2 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn2)), ((locals.var_t1_dn6 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn6)), ((locals.var_t1_dn7 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn7)), ((locals.var_t1_dn10 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn10)), ((locals.var_t1_dn11 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn11)), ((locals.var_t1_dn12 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn12)), ((locals.var_t1_dn17 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn17)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
        locals.var_ty = assign14400_e20158;
        locals.var_ty_dn0 = assign14400_e20158_d_n0;
        locals.var_ty_dn2 = assign14400_e20158_d_n2;
        locals.var_ty_dn6 = assign14400_e20158_d_n6;
        locals.var_ty_dn7 = assign14400_e20158_d_n7;
        locals.var_ty_dn10 = assign14400_e20158_d_n10;
        locals.var_ty_dn11 = assign14400_e20158_d_n11;
        locals.var_ty_dn12 = assign14400_e20158_d_n12;
        locals.var_ty_dn17 = assign14400_e20158_d_n17;

        let (assign14410_e20171, assign14410_e20171_d_n0, assign14410_e20171_d_n2, assign14410_e20171_d_n6, assign14410_e20171_d_n7, assign14410_e20171_d_n10, assign14410_e20171_d_n11, assign14410_e20171_d_n12, assign14410_e20171_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14410_e20166: f64 = (3.0 * 1.414213562373095);
        let assign14410_e20168: f64 = (assign14410_e20166 * locals.var_ty);
        let assign14410_e20169: f64 = (2.0 + assign14410_e20168);
        (assign14410_e20169, (assign14410_e20166 * locals.var_ty_dn0), (assign14410_e20166 * locals.var_ty_dn2), (assign14410_e20166 * locals.var_ty_dn6), (assign14410_e20166 * locals.var_ty_dn7), (assign14410_e20166 * locals.var_ty_dn10), (assign14410_e20166 * locals.var_ty_dn11), (assign14410_e20166 * locals.var_ty_dn12), (assign14410_e20166 * locals.var_ty_dn17),)
    } else {
        (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn6, locals.var_ac41_dn7, locals.var_ac41_dn10, locals.var_ac41_dn11, locals.var_ac41_dn12, locals.var_ac41_dn17,)
    }
};
        locals.var_ac41 = assign14410_e20171;
        locals.var_ac41_dn0 = assign14410_e20171_d_n0;
        locals.var_ac41_dn2 = assign14410_e20171_d_n2;
        locals.var_ac41_dn6 = assign14410_e20171_d_n6;
        locals.var_ac41_dn7 = assign14410_e20171_d_n7;
        locals.var_ac41_dn10 = assign14410_e20171_d_n10;
        locals.var_ac41_dn11 = assign14410_e20171_d_n11;
        locals.var_ac41_dn12 = assign14410_e20171_d_n12;
        locals.var_ac41_dn17 = assign14410_e20171_d_n17;

        let (assign14420_e20184, assign14420_e20184_d_n0, assign14420_e20184_d_n2, assign14420_e20184_d_n6, assign14420_e20184_d_n7, assign14420_e20184_d_n10, assign14420_e20184_d_n11, assign14420_e20184_d_n12, assign14420_e20184_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14420_e20178: f64 = (8.0 * locals.var_ac41);
        let assign14420_e20180: f64 = (assign14420_e20178 * locals.var_ac41);
        let assign14420_e20182: f64 = (assign14420_e20180 * locals.var_ac41);
        (assign14420_e20182, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign14420_e20178 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign14420_e20180 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign14420_e20178 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign14420_e20180 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign14420_e20178 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign14420_e20180 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn7) * locals.var_ac41) + (assign14420_e20178 * locals.var_ac41_dn7)) * locals.var_ac41) + (assign14420_e20180 * locals.var_ac41_dn7)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign14420_e20178 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign14420_e20180 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn11) * locals.var_ac41) + (assign14420_e20178 * locals.var_ac41_dn11)) * locals.var_ac41) + (assign14420_e20180 * locals.var_ac41_dn11)), (((((8.0 * locals.var_ac41_dn12) * locals.var_ac41) + (assign14420_e20178 * locals.var_ac41_dn12)) * locals.var_ac41) + (assign14420_e20180 * locals.var_ac41_dn12)), (((((8.0 * locals.var_ac41_dn17) * locals.var_ac41) + (assign14420_e20178 * locals.var_ac41_dn17)) * locals.var_ac41) + (assign14420_e20180 * locals.var_ac41_dn17)),)
    } else {
        (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn6, locals.var_ac4_dn7, locals.var_ac4_dn10, locals.var_ac4_dn11, locals.var_ac4_dn12, locals.var_ac4_dn17,)
    }
};
        locals.var_ac4 = assign14420_e20184;
        locals.var_ac4_dn0 = assign14420_e20184_d_n0;
        locals.var_ac4_dn2 = assign14420_e20184_d_n2;
        locals.var_ac4_dn6 = assign14420_e20184_d_n6;
        locals.var_ac4_dn7 = assign14420_e20184_d_n7;
        locals.var_ac4_dn10 = assign14420_e20184_d_n10;
        locals.var_ac4_dn11 = assign14420_e20184_d_n11;
        locals.var_ac4_dn12 = assign14420_e20184_d_n12;
        locals.var_ac4_dn17 = assign14420_e20184_d_n17;

        let (assign14430_e20193, assign14430_e20193_d_n0, assign14430_e20193_d_n2, assign14430_e20193_d_n6, assign14430_e20193_d_n7, assign14430_e20193_d_n10, assign14430_e20193_d_n11, assign14430_e20193_d_n12, assign14430_e20193_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14430_e20191: f64 = (locals.var_tx - 2.0);
        (assign14430_e20191, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign14430_e20193;
        locals.var_t4_dn0 = assign14430_e20193_d_n0;
        locals.var_t4_dn2 = assign14430_e20193_d_n2;
        locals.var_t4_dn6 = assign14430_e20193_d_n6;
        locals.var_t4_dn7 = assign14430_e20193_d_n7;
        locals.var_t4_dn10 = assign14430_e20193_d_n10;
        locals.var_t4_dn11 = assign14430_e20193_d_n11;
        locals.var_t4_dn12 = assign14430_e20193_d_n12;
        locals.var_t4_dn17 = assign14430_e20193_d_n17;

        let (assign14440_e20204, assign14440_e20204_d_n0, assign14440_e20204_d_n2, assign14440_e20204_d_n6, assign14440_e20204_d_n7, assign14440_e20204_d_n10, assign14440_e20204_d_n11, assign14440_e20204_d_n12, assign14440_e20204_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14440_e20200: f64 = (9.0 * locals.var_ty);
        let assign14440_e20202: f64 = (assign14440_e20200 * locals.var_t4);
        (assign14440_e20202, (((9.0 * locals.var_ty_dn0) * locals.var_t4) + (assign14440_e20200 * locals.var_t4_dn0)), (((9.0 * locals.var_ty_dn2) * locals.var_t4) + (assign14440_e20200 * locals.var_t4_dn2)), (((9.0 * locals.var_ty_dn6) * locals.var_t4) + (assign14440_e20200 * locals.var_t4_dn6)), (((9.0 * locals.var_ty_dn7) * locals.var_t4) + (assign14440_e20200 * locals.var_t4_dn7)), (((9.0 * locals.var_ty_dn10) * locals.var_t4) + (assign14440_e20200 * locals.var_t4_dn10)), (((9.0 * locals.var_ty_dn11) * locals.var_t4) + (assign14440_e20200 * locals.var_t4_dn11)), (((9.0 * locals.var_ty_dn12) * locals.var_t4) + (assign14440_e20200 * locals.var_t4_dn12)), (((9.0 * locals.var_ty_dn17) * locals.var_t4) + (assign14440_e20200 * locals.var_t4_dn17)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign14440_e20204;
        locals.var_t5_dn0 = assign14440_e20204_d_n0;
        locals.var_t5_dn2 = assign14440_e20204_d_n2;
        locals.var_t5_dn6 = assign14440_e20204_d_n6;
        locals.var_t5_dn7 = assign14440_e20204_d_n7;
        locals.var_t5_dn10 = assign14440_e20204_d_n10;
        locals.var_t5_dn11 = assign14440_e20204_d_n11;
        locals.var_t5_dn12 = assign14440_e20204_d_n12;
        locals.var_t5_dn17 = assign14440_e20204_d_n17;

        let (assign14450_e20215, assign14450_e20215_d_n0, assign14450_e20215_d_n2, assign14450_e20215_d_n6, assign14450_e20215_d_n7, assign14450_e20215_d_n10, assign14450_e20215_d_n11, assign14450_e20215_d_n12, assign14450_e20215_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14450_e20211: f64 = (7.0 * 1.414213562373095);
        let assign14450_e20213: f64 = (assign14450_e20211 - locals.var_t5);
        (assign14450_e20213, (-locals.var_t5_dn0), (-locals.var_t5_dn2), (-locals.var_t5_dn6), (-locals.var_t5_dn7), (-locals.var_t5_dn10), (-locals.var_t5_dn11), (-locals.var_t5_dn12), (-locals.var_t5_dn17),)
    } else {
        (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn6, locals.var_ac31_dn7, locals.var_ac31_dn10, locals.var_ac31_dn11, locals.var_ac31_dn12, locals.var_ac31_dn17,)
    }
};
        locals.var_ac31 = assign14450_e20215;
        locals.var_ac31_dn0 = assign14450_e20215_d_n0;
        locals.var_ac31_dn2 = assign14450_e20215_d_n2;
        locals.var_ac31_dn6 = assign14450_e20215_d_n6;
        locals.var_ac31_dn7 = assign14450_e20215_d_n7;
        locals.var_ac31_dn10 = assign14450_e20215_d_n10;
        locals.var_ac31_dn11 = assign14450_e20215_d_n11;
        locals.var_ac31_dn12 = assign14450_e20215_d_n12;
        locals.var_ac31_dn17 = assign14450_e20215_d_n17;

        let (assign14460_e20224, assign14460_e20224_d_n0, assign14460_e20224_d_n2, assign14460_e20224_d_n6, assign14460_e20224_d_n7, assign14460_e20224_d_n10, assign14460_e20224_d_n11, assign14460_e20224_d_n12, assign14460_e20224_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14460_e20222: f64 = (locals.var_ac31 * locals.var_ac31);
        (assign14460_e20222, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn7 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn7)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn11 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn11)), ((locals.var_ac31_dn12 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn12)), ((locals.var_ac31_dn17 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn17)),)
    } else {
        (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn6, locals.var_ac3_dn7, locals.var_ac3_dn10, locals.var_ac3_dn11, locals.var_ac3_dn12, locals.var_ac3_dn17,)
    }
};
        locals.var_ac3 = assign14460_e20224;
        locals.var_ac3_dn0 = assign14460_e20224_d_n0;
        locals.var_ac3_dn2 = assign14460_e20224_d_n2;
        locals.var_ac3_dn6 = assign14460_e20224_d_n6;
        locals.var_ac3_dn7 = assign14460_e20224_d_n7;
        locals.var_ac3_dn10 = assign14460_e20224_d_n10;
        locals.var_ac3_dn11 = assign14460_e20224_d_n11;
        locals.var_ac3_dn12 = assign14460_e20224_d_n12;
        locals.var_ac3_dn17 = assign14460_e20224_d_n17;

        let assign14470_e20228: f64 = (locals.var_ac3 * 1e-8);
        let assign14470_e20229: f64 = if locals.var_ac4 < assign14470_e20228 { 1.0 } else { 0.0 };
        locals.var_guard445 = assign14470_e20229;

        let (assign14480_e20251, assign14480_e20251_d_n0, assign14480_e20251_d_n2, assign14480_e20251_d_n6, assign14480_e20251_d_n7, assign14480_e20251_d_n10, assign14480_e20251_d_n11, assign14480_e20251_d_n12, assign14480_e20251_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard445 != 0.0)) {
        let assign14480_e20237: f64 = (-7.0);
        let assign14480_e20239: f64 = (assign14480_e20237 * 1.414213562373095);
        let assign14480_e20241: f64 = (assign14480_e20239 + locals.var_ac31);
        let assign14480_e20244: f64 = (0.5 * locals.var_ac4);
        let assign14480_e20246: f64 = (assign14480_e20244 / locals.var_ac31);
        let assign14480_e20247: f64 = (assign14480_e20241 + assign14480_e20246);
        let assign14480_e20249: f64 = (assign14480_e20247 + locals.var_t5);
        (assign14480_e20249, ((locals.var_ac31_dn0 + ((((0.5 * locals.var_ac4_dn0) * locals.var_ac31) - (assign14480_e20244 * locals.var_ac31_dn0)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn0), ((locals.var_ac31_dn2 + ((((0.5 * locals.var_ac4_dn2) * locals.var_ac31) - (assign14480_e20244 * locals.var_ac31_dn2)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn2), ((locals.var_ac31_dn6 + ((((0.5 * locals.var_ac4_dn6) * locals.var_ac31) - (assign14480_e20244 * locals.var_ac31_dn6)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn6), ((locals.var_ac31_dn7 + ((((0.5 * locals.var_ac4_dn7) * locals.var_ac31) - (assign14480_e20244 * locals.var_ac31_dn7)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn7), ((locals.var_ac31_dn10 + ((((0.5 * locals.var_ac4_dn10) * locals.var_ac31) - (assign14480_e20244 * locals.var_ac31_dn10)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn10), ((locals.var_ac31_dn11 + ((((0.5 * locals.var_ac4_dn11) * locals.var_ac31) - (assign14480_e20244 * locals.var_ac31_dn11)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn11), ((locals.var_ac31_dn12 + ((((0.5 * locals.var_ac4_dn12) * locals.var_ac31) - (assign14480_e20244 * locals.var_ac31_dn12)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn12), ((locals.var_ac31_dn17 + ((((0.5 * locals.var_ac4_dn17) * locals.var_ac31) - (assign14480_e20244 * locals.var_ac31_dn17)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn17),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn12, locals.var_ac1_dn17,)
    }
};
        locals.var_ac1 = assign14480_e20251;
        locals.var_ac1_dn0 = assign14480_e20251_d_n0;
        locals.var_ac1_dn2 = assign14480_e20251_d_n2;
        locals.var_ac1_dn6 = assign14480_e20251_d_n6;
        locals.var_ac1_dn7 = assign14480_e20251_d_n7;
        locals.var_ac1_dn10 = assign14480_e20251_d_n10;
        locals.var_ac1_dn11 = assign14480_e20251_d_n11;
        locals.var_ac1_dn12 = assign14480_e20251_d_n12;
        locals.var_ac1_dn17 = assign14480_e20251_d_n17;

        let (assign14490_e20264, assign14490_e20264_d_n0, assign14490_e20264_d_n2, assign14490_e20264_d_n6, assign14490_e20264_d_n7, assign14490_e20264_d_n10, assign14490_e20264_d_n11, assign14490_e20264_d_n12, assign14490_e20264_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard445 == 0.0)) {
        let assign14490_e20261: f64 = (locals.var_ac4 + locals.var_ac3);
        let assign14490_e20262: f64 = (assign14490_e20261).sqrt();
        (assign14490_e20262, ((locals.var_ac4_dn0 + locals.var_ac3_dn0) / (2.0 * assign14490_e20262)), ((locals.var_ac4_dn2 + locals.var_ac3_dn2) / (2.0 * assign14490_e20262)), ((locals.var_ac4_dn6 + locals.var_ac3_dn6) / (2.0 * assign14490_e20262)), ((locals.var_ac4_dn7 + locals.var_ac3_dn7) / (2.0 * assign14490_e20262)), ((locals.var_ac4_dn10 + locals.var_ac3_dn10) / (2.0 * assign14490_e20262)), ((locals.var_ac4_dn11 + locals.var_ac3_dn11) / (2.0 * assign14490_e20262)), ((locals.var_ac4_dn12 + locals.var_ac3_dn12) / (2.0 * assign14490_e20262)), ((locals.var_ac4_dn17 + locals.var_ac3_dn17) / (2.0 * assign14490_e20262)),)
    } else {
        (locals.var_ac2, locals.var_ac2_dn0, locals.var_ac2_dn2, locals.var_ac2_dn6, locals.var_ac2_dn7, locals.var_ac2_dn10, locals.var_ac2_dn11, locals.var_ac2_dn12, locals.var_ac2_dn17,)
    }
};
        locals.var_ac2 = assign14490_e20264;
        locals.var_ac2_dn0 = assign14490_e20264_d_n0;
        locals.var_ac2_dn2 = assign14490_e20264_d_n2;
        locals.var_ac2_dn6 = assign14490_e20264_d_n6;
        locals.var_ac2_dn7 = assign14490_e20264_d_n7;
        locals.var_ac2_dn10 = assign14490_e20264_d_n10;
        locals.var_ac2_dn11 = assign14490_e20264_d_n11;
        locals.var_ac2_dn12 = assign14490_e20264_d_n12;
        locals.var_ac2_dn17 = assign14490_e20264_d_n17;

        let (assign14500_e20281, assign14500_e20281_d_n0, assign14500_e20281_d_n2, assign14500_e20281_d_n6, assign14500_e20281_d_n7, assign14500_e20281_d_n10, assign14500_e20281_d_n11, assign14500_e20281_d_n12, assign14500_e20281_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard445 == 0.0)) {
        let assign14500_e20273: f64 = (-7.0);
        let assign14500_e20275: f64 = (assign14500_e20273 * 1.414213562373095);
        let assign14500_e20277: f64 = (assign14500_e20275 + locals.var_ac2);
        let assign14500_e20279: f64 = (assign14500_e20277 + locals.var_t5);
        (assign14500_e20279, (locals.var_ac2_dn0 + locals.var_t5_dn0), (locals.var_ac2_dn2 + locals.var_t5_dn2), (locals.var_ac2_dn6 + locals.var_t5_dn6), (locals.var_ac2_dn7 + locals.var_t5_dn7), (locals.var_ac2_dn10 + locals.var_t5_dn10), (locals.var_ac2_dn11 + locals.var_t5_dn11), (locals.var_ac2_dn12 + locals.var_t5_dn12), (locals.var_ac2_dn17 + locals.var_t5_dn17),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn12, locals.var_ac1_dn17,)
    }
};
        locals.var_ac1 = assign14500_e20281;
        locals.var_ac1_dn0 = assign14500_e20281_d_n0;
        locals.var_ac1_dn2 = assign14500_e20281_d_n2;
        locals.var_ac1_dn6 = assign14500_e20281_d_n6;
        locals.var_ac1_dn7 = assign14500_e20281_d_n7;
        locals.var_ac1_dn10 = assign14500_e20281_d_n10;
        locals.var_ac1_dn11 = assign14500_e20281_d_n11;
        locals.var_ac1_dn12 = assign14500_e20281_d_n12;
        locals.var_ac1_dn17 = assign14500_e20281_d_n17;

        let (assign14510_e20290, assign14510_e20290_d_n0, assign14510_e20290_d_n2, assign14510_e20290_d_n6, assign14510_e20290_d_n7, assign14510_e20290_d_n10, assign14510_e20290_d_n11, assign14510_e20290_d_n12, assign14510_e20290_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14510_e20288: f64 = (locals.var_ac1).powf(0.3333333333333333);
        (assign14510_e20288, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn0)) } } else { (assign14510_e20288 * (0.3333333333333333 * (locals.var_ac1_dn0 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn2)) } } else { (assign14510_e20288 * (0.3333333333333333 * (locals.var_ac1_dn2 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn6)) } } else { (assign14510_e20288 * (0.3333333333333333 * (locals.var_ac1_dn6 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn7)) } } else { (assign14510_e20288 * (0.3333333333333333 * (locals.var_ac1_dn7 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn10)) } } else { (assign14510_e20288 * (0.3333333333333333 * (locals.var_ac1_dn10 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn11)) } } else { (assign14510_e20288 * (0.3333333333333333 * (locals.var_ac1_dn11 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn12)) } } else { (assign14510_e20288 * (0.3333333333333333 * (locals.var_ac1_dn12 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn17)) } } else { (assign14510_e20288 * (0.3333333333333333 * (locals.var_ac1_dn17 / locals.var_ac1))) },)
    } else {
        (locals.var_acd, locals.var_acd_dn0, locals.var_acd_dn2, locals.var_acd_dn6, locals.var_acd_dn7, locals.var_acd_dn10, locals.var_acd_dn11, locals.var_acd_dn12, locals.var_acd_dn17,)
    }
};
        locals.var_acd = assign14510_e20290;
        locals.var_acd_dn0 = assign14510_e20290_d_n0;
        locals.var_acd_dn2 = assign14510_e20290_d_n2;
        locals.var_acd_dn6 = assign14510_e20290_d_n6;
        locals.var_acd_dn7 = assign14510_e20290_d_n7;
        locals.var_acd_dn10 = assign14510_e20290_d_n10;
        locals.var_acd_dn11 = assign14510_e20290_d_n11;
        locals.var_acd_dn12 = assign14510_e20290_d_n12;
        locals.var_acd_dn17 = assign14510_e20290_d_n17;

        let (assign14520_e20314, assign14520_e20314_d_n0, assign14520_e20314_d_n2, assign14520_e20314_d_n6, assign14520_e20314_d_n7, assign14520_e20314_d_n10, assign14520_e20314_d_n11, assign14520_e20314_d_n12, assign14520_e20314_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14520_e20296: f64 = (-4.0);
        let assign14520_e20298: f64 = (assign14520_e20296 * 1.414213562373095);
        let assign14520_e20301: f64 = (12.0 * locals.var_ty);
        let assign14520_e20302: f64 = (assign14520_e20298 - assign14520_e20301);
        let assign14520_e20305: f64 = (2.0 * locals.var_acd);
        let assign14520_e20306: f64 = (assign14520_e20302 + assign14520_e20305);
        let assign14520_e20309: f64 = (1.414213562373095 * locals.var_acd);
        let assign14520_e20311: f64 = (assign14520_e20309 * locals.var_acd);
        let assign14520_e20312: f64 = (assign14520_e20306 + assign14520_e20311);
        (assign14520_e20312, (((-(12.0 * locals.var_ty_dn0)) + (2.0 * locals.var_acd_dn0)) + (((1.414213562373095 * locals.var_acd_dn0) * locals.var_acd) + (assign14520_e20309 * locals.var_acd_dn0))), (((-(12.0 * locals.var_ty_dn2)) + (2.0 * locals.var_acd_dn2)) + (((1.414213562373095 * locals.var_acd_dn2) * locals.var_acd) + (assign14520_e20309 * locals.var_acd_dn2))), (((-(12.0 * locals.var_ty_dn6)) + (2.0 * locals.var_acd_dn6)) + (((1.414213562373095 * locals.var_acd_dn6) * locals.var_acd) + (assign14520_e20309 * locals.var_acd_dn6))), (((-(12.0 * locals.var_ty_dn7)) + (2.0 * locals.var_acd_dn7)) + (((1.414213562373095 * locals.var_acd_dn7) * locals.var_acd) + (assign14520_e20309 * locals.var_acd_dn7))), (((-(12.0 * locals.var_ty_dn10)) + (2.0 * locals.var_acd_dn10)) + (((1.414213562373095 * locals.var_acd_dn10) * locals.var_acd) + (assign14520_e20309 * locals.var_acd_dn10))), (((-(12.0 * locals.var_ty_dn11)) + (2.0 * locals.var_acd_dn11)) + (((1.414213562373095 * locals.var_acd_dn11) * locals.var_acd) + (assign14520_e20309 * locals.var_acd_dn11))), (((-(12.0 * locals.var_ty_dn12)) + (2.0 * locals.var_acd_dn12)) + (((1.414213562373095 * locals.var_acd_dn12) * locals.var_acd) + (assign14520_e20309 * locals.var_acd_dn12))), (((-(12.0 * locals.var_ty_dn17)) + (2.0 * locals.var_acd_dn17)) + (((1.414213562373095 * locals.var_acd_dn17) * locals.var_acd) + (assign14520_e20309 * locals.var_acd_dn17))),)
    } else {
        (locals.var_acn, locals.var_acn_dn0, locals.var_acn_dn2, locals.var_acn_dn6, locals.var_acn_dn7, locals.var_acn_dn10, locals.var_acn_dn11, locals.var_acn_dn12, locals.var_acn_dn17,)
    }
};
        locals.var_acn = assign14520_e20314;
        locals.var_acn_dn0 = assign14520_e20314_d_n0;
        locals.var_acn_dn2 = assign14520_e20314_d_n2;
        locals.var_acn_dn6 = assign14520_e20314_d_n6;
        locals.var_acn_dn7 = assign14520_e20314_d_n7;
        locals.var_acn_dn10 = assign14520_e20314_d_n10;
        locals.var_acn_dn11 = assign14520_e20314_d_n11;
        locals.var_acn_dn12 = assign14520_e20314_d_n12;
        locals.var_acn_dn17 = assign14520_e20314_d_n17;

        let (assign14530_e20323, assign14530_e20323_d_n0, assign14530_e20323_d_n2, assign14530_e20323_d_n6, assign14530_e20323_d_n7, assign14530_e20323_d_n10, assign14530_e20323_d_n11, assign14530_e20323_d_n12, assign14530_e20323_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14530_e20321: f64 = (1.0 / locals.var_acd);
        (assign14530_e20321, (-(locals.var_acd_dn0 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn2 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn6 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn7 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn10 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn11 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn12 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn17 / (locals.var_acd * locals.var_acd))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign14530_e20323;
        locals.var_t1_dn0 = assign14530_e20323_d_n0;
        locals.var_t1_dn2 = assign14530_e20323_d_n2;
        locals.var_t1_dn6 = assign14530_e20323_d_n6;
        locals.var_t1_dn7 = assign14530_e20323_d_n7;
        locals.var_t1_dn10 = assign14530_e20323_d_n10;
        locals.var_t1_dn11 = assign14530_e20323_d_n11;
        locals.var_t1_dn12 = assign14530_e20323_d_n12;
        locals.var_t1_dn17 = assign14530_e20323_d_n17;

        let (assign14540_e20332, assign14540_e20332_d_n0, assign14540_e20332_d_n2, assign14540_e20332_d_n6, assign14540_e20332_d_n7, assign14540_e20332_d_n10, assign14540_e20332_d_n11, assign14540_e20332_d_n12, assign14540_e20332_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14540_e20330: f64 = (locals.var_acn * locals.var_t1);
        (assign14540_e20330, ((locals.var_acn_dn0 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn0)), ((locals.var_acn_dn2 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn2)), ((locals.var_acn_dn6 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn6)), ((locals.var_acn_dn7 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn7)), ((locals.var_acn_dn10 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn10)), ((locals.var_acn_dn11 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn11)), ((locals.var_acn_dn12 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn12)), ((locals.var_acn_dn17 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn17)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, locals.var_chi_dn17,)
    }
};
        locals.var_chi = assign14540_e20332;
        locals.var_chi_dn0 = assign14540_e20332_d_n0;
        locals.var_chi_dn2 = assign14540_e20332_d_n2;
        locals.var_chi_dn6 = assign14540_e20332_d_n6;
        locals.var_chi_dn7 = assign14540_e20332_d_n7;
        locals.var_chi_dn10 = assign14540_e20332_d_n10;
        locals.var_chi_dn11 = assign14540_e20332_d_n11;
        locals.var_chi_dn12 = assign14540_e20332_d_n12;
        locals.var_chi_dn17 = assign14540_e20332_d_n17;

        let (assign14550_e20343, assign14550_e20343_d_n0, assign14550_e20343_d_n2, assign14550_e20343_d_n6, assign14550_e20343_d_n7, assign14550_e20343_d_n10, assign14550_e20343_d_n11, assign14550_e20343_d_n12, assign14550_e20343_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14550_e20339: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign14550_e20341: f64 = (assign14550_e20339 + locals.var_vbcs_cl);
        (assign14550_e20341, ((locals.var_chi_dn0 * locals.var_beta_inv) + locals.var_vbcs_cl_dn0), ((locals.var_chi_dn2 * locals.var_beta_inv) + locals.var_vbcs_cl_dn2), ((locals.var_chi_dn6 * locals.var_beta_inv) + locals.var_vbcs_cl_dn6), ((locals.var_chi_dn7 * locals.var_beta_inv) + locals.var_vbcs_cl_dn7), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) + locals.var_vbcs_cl_dn10), ((locals.var_chi_dn11 * locals.var_beta_inv) + locals.var_vbcs_cl_dn11), ((locals.var_chi_dn12 * locals.var_beta_inv) + locals.var_vbcs_cl_dn12), ((locals.var_chi_dn17 * locals.var_beta_inv) + locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_psa, locals.var_psa_dn0, locals.var_psa_dn2, locals.var_psa_dn6, locals.var_psa_dn7, locals.var_psa_dn10, locals.var_psa_dn11, locals.var_psa_dn12, locals.var_psa_dn17,)
    }
};
        locals.var_psa = assign14550_e20343;
        locals.var_psa_dn0 = assign14550_e20343_d_n0;
        locals.var_psa_dn2 = assign14550_e20343_d_n2;
        locals.var_psa_dn6 = assign14550_e20343_d_n6;
        locals.var_psa_dn7 = assign14550_e20343_d_n7;
        locals.var_psa_dn10 = assign14550_e20343_d_n10;
        locals.var_psa_dn11 = assign14550_e20343_d_n11;
        locals.var_psa_dn12 = assign14550_e20343_d_n12;
        locals.var_psa_dn17 = assign14550_e20343_d_n17;

        let (assign14560_e20352, assign14560_e20352_d_n0, assign14560_e20352_d_n2, assign14560_e20352_d_n6, assign14560_e20352_d_n7, assign14560_e20352_d_n10, assign14560_e20352_d_n11, assign14560_e20352_d_n12, assign14560_e20352_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14560_e20350: f64 = (locals.var_psa - locals.var_vbcs_cl);
        (assign14560_e20350, (locals.var_psa_dn0 - locals.var_vbcs_cl_dn0), (locals.var_psa_dn2 - locals.var_vbcs_cl_dn2), (locals.var_psa_dn6 - locals.var_vbcs_cl_dn6), (locals.var_psa_dn7 - locals.var_vbcs_cl_dn7), (locals.var_psa_dn10 - locals.var_vbcs_cl_dn10), (locals.var_psa_dn11 - locals.var_vbcs_cl_dn11), (locals.var_psa_dn12 - locals.var_vbcs_cl_dn12), (locals.var_psa_dn17 - locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign14560_e20352;
        locals.var_t1_dn0 = assign14560_e20352_d_n0;
        locals.var_t1_dn2 = assign14560_e20352_d_n2;
        locals.var_t1_dn6 = assign14560_e20352_d_n6;
        locals.var_t1_dn7 = assign14560_e20352_d_n7;
        locals.var_t1_dn10 = assign14560_e20352_d_n10;
        locals.var_t1_dn11 = assign14560_e20352_d_n11;
        locals.var_t1_dn12 = assign14560_e20352_d_n12;
        locals.var_t1_dn17 = assign14560_e20352_d_n17;

        let (assign14570_e20361, assign14570_e20361_d_n0, assign14570_e20361_d_n2, assign14570_e20361_d_n6, assign14570_e20361_d_n7, assign14570_e20361_d_n10, assign14570_e20361_d_n11, assign14570_e20361_d_n12, assign14570_e20361_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14570_e20359: f64 = (locals.var_t1 / locals.var_ps0_min);
        (assign14570_e20359, (((locals.var_t1_dn0 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn0)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn2 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn2)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn6 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn6)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn7 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn7)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn10 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn10)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn11 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn11)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn12 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn12)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn17 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn17)) / (locals.var_ps0_min * locals.var_ps0_min)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign14570_e20361;
        locals.var_t2_dn0 = assign14570_e20361_d_n0;
        locals.var_t2_dn2 = assign14570_e20361_d_n2;
        locals.var_t2_dn6 = assign14570_e20361_d_n6;
        locals.var_t2_dn7 = assign14570_e20361_d_n7;
        locals.var_t2_dn10 = assign14570_e20361_d_n10;
        locals.var_t2_dn11 = assign14570_e20361_d_n11;
        locals.var_t2_dn12 = assign14570_e20361_d_n12;
        locals.var_t2_dn17 = assign14570_e20361_d_n17;

        let (assign14580_e20373, assign14580_e20373_d_n0, assign14580_e20373_d_n2, assign14580_e20373_d_n6, assign14580_e20373_d_n7, assign14580_e20373_d_n10, assign14580_e20373_d_n11, assign14580_e20373_d_n12, assign14580_e20373_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14580_e20369: f64 = (locals.var_t2 * locals.var_t2);
        let assign14580_e20370: f64 = (1.0 + assign14580_e20369);
        let assign14580_e20371: f64 = (assign14580_e20370).sqrt();
        (assign14580_e20371, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign14580_e20371)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign14580_e20371)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign14580_e20371)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign14580_e20371)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign14580_e20371)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign14580_e20371)), (((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)) / (2.0 * assign14580_e20371)), (((locals.var_t2_dn17 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn17)) / (2.0 * assign14580_e20371)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign14580_e20373;
        locals.var_t3_dn0 = assign14580_e20373_d_n0;
        locals.var_t3_dn2 = assign14580_e20373_d_n2;
        locals.var_t3_dn6 = assign14580_e20373_d_n6;
        locals.var_t3_dn7 = assign14580_e20373_d_n7;
        locals.var_t3_dn10 = assign14580_e20373_d_n10;
        locals.var_t3_dn11 = assign14580_e20373_d_n11;
        locals.var_t3_dn12 = assign14580_e20373_d_n12;
        locals.var_t3_dn17 = assign14580_e20373_d_n17;

        let (assign14590_e20384, assign14590_e20384_d_n0, assign14590_e20384_d_n2, assign14590_e20384_d_n6, assign14590_e20384_d_n7, assign14590_e20384_d_n10, assign14590_e20384_d_n11, assign14590_e20384_d_n12, assign14590_e20384_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 != 0.0)) {
        let assign14590_e20380: f64 = (locals.var_t1 / locals.var_t3);
        let assign14590_e20382: f64 = (assign14590_e20380 + locals.var_vbcs_cl);
        (assign14590_e20382, ((((locals.var_t1_dn0 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn0), ((((locals.var_t1_dn2 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn2), ((((locals.var_t1_dn6 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn6), ((((locals.var_t1_dn7 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn7), ((((locals.var_t1_dn10 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn10), ((((locals.var_t1_dn11 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn11), ((((locals.var_t1_dn12 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn12)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn12), ((((locals.var_t1_dn17 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn17)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    }
};
        locals.var_ps0 = assign14590_e20384;
        locals.var_ps0_dn0 = assign14590_e20384_d_n0;
        locals.var_ps0_dn2 = assign14590_e20384_d_n2;
        locals.var_ps0_dn6 = assign14590_e20384_d_n6;
        locals.var_ps0_dn7 = assign14590_e20384_d_n7;
        locals.var_ps0_dn10 = assign14590_e20384_d_n10;
        locals.var_ps0_dn11 = assign14590_e20384_d_n11;
        locals.var_ps0_dn12 = assign14590_e20384_d_n12;
        locals.var_ps0_dn17 = assign14590_e20384_d_n17;

        let assign14600_e20387: f64 = if locals.var_flg_pprv >= 1.0 { 1.0 } else { 0.0 };
        locals.var_guard446 = assign14600_e20387;

        let (assign14610_e20397, assign14610_e20397_d_n0, assign14610_e20397_d_n2, assign14610_e20397_d_n6, assign14610_e20397_d_n7, assign14610_e20397_d_n10, assign14610_e20397_d_n11, assign14610_e20397_d_n12, assign14610_e20397_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 != 0.0)) {
        (locals.var_pss0_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    }
};
        locals.var_phi_s0_soi = assign14610_e20397;
        locals.var_phi_s0_soi_dn0 = assign14610_e20397_d_n0;
        locals.var_phi_s0_soi_dn2 = assign14610_e20397_d_n2;
        locals.var_phi_s0_soi_dn6 = assign14610_e20397_d_n6;
        locals.var_phi_s0_soi_dn7 = assign14610_e20397_d_n7;
        locals.var_phi_s0_soi_dn10 = assign14610_e20397_d_n10;
        locals.var_phi_s0_soi_dn11 = assign14610_e20397_d_n11;
        locals.var_phi_s0_soi_dn12 = assign14610_e20397_d_n12;
        locals.var_phi_s0_soi_dn17 = assign14610_e20397_d_n17;

        let (assign14620_e20407, assign14620_e20407_d_n0, assign14620_e20407_d_n2, assign14620_e20407_d_n6, assign14620_e20407_d_n7, assign14620_e20407_d_n10, assign14620_e20407_d_n11, assign14620_e20407_d_n12, assign14620_e20407_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 != 0.0)) {
        (locals.var_pss0_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign14620_e20407;
        locals.var_ps0_ini_dn0 = assign14620_e20407_d_n0;
        locals.var_ps0_ini_dn2 = assign14620_e20407_d_n2;
        locals.var_ps0_ini_dn6 = assign14620_e20407_d_n6;
        locals.var_ps0_ini_dn7 = assign14620_e20407_d_n7;
        locals.var_ps0_ini_dn10 = assign14620_e20407_d_n10;
        locals.var_ps0_ini_dn11 = assign14620_e20407_d_n11;
        locals.var_ps0_ini_dn12 = assign14620_e20407_d_n12;
        locals.var_ps0_ini_dn17 = assign14620_e20407_d_n17;

        let (assign14630_e20432, assign14630_e20432_d_n0, assign14630_e20432_d_n2, assign14630_e20432_d_n6, assign14630_e20432_d_n7, assign14630_e20432_d_n10, assign14630_e20432_d_n11, assign14630_e20432_d_n12, assign14630_e20432_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) {
        let assign14630_e20421: f64 = (locals.var_vgp - locals.var_vbcs_cl);
        let assign14630_e20422: f64 = (locals.var_beta * assign14630_e20421);
        let assign14630_e20424: f64 = (assign14630_e20422 - 1.0);
        let assign14630_e20425: f64 = (4.0 * assign14630_e20424);
        let assign14630_e20428: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign14630_e20429: f64 = (assign14630_e20425 / assign14630_e20428);
        let assign14630_e20430: f64 = (1.0 + assign14630_e20429);
        (assign14630_e20430, ((((4.0 * (locals.var_beta * (locals.var_vgp_dn0 - locals.var_vbcs_cl_dn0))) * assign14630_e20428) - (assign14630_e20425 * (locals.var_fac1p2_dn0 * locals.var_beta2))) / (assign14630_e20428 * assign14630_e20428)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn2 - locals.var_vbcs_cl_dn2))) * assign14630_e20428) - (assign14630_e20425 * (locals.var_fac1p2_dn2 * locals.var_beta2))) / (assign14630_e20428 * assign14630_e20428)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn6 - locals.var_vbcs_cl_dn6))) * assign14630_e20428) - (assign14630_e20425 * (locals.var_fac1p2_dn6 * locals.var_beta2))) / (assign14630_e20428 * assign14630_e20428)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn7 - locals.var_vbcs_cl_dn7))) * assign14630_e20428) - (assign14630_e20425 * (locals.var_fac1p2_dn7 * locals.var_beta2))) / (assign14630_e20428 * assign14630_e20428)), ((((4.0 * ((locals.var_beta_dn10 * assign14630_e20421) + (locals.var_beta * (locals.var_vgp_dn10 - locals.var_vbcs_cl_dn10)))) * assign14630_e20428) - (assign14630_e20425 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign14630_e20428 * assign14630_e20428)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn11 - locals.var_vbcs_cl_dn11))) * assign14630_e20428) - (assign14630_e20425 * (locals.var_fac1p2_dn11 * locals.var_beta2))) / (assign14630_e20428 * assign14630_e20428)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn12 - locals.var_vbcs_cl_dn12))) * assign14630_e20428) - (assign14630_e20425 * (locals.var_fac1p2_dn12 * locals.var_beta2))) / (assign14630_e20428 * assign14630_e20428)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn17 - locals.var_vbcs_cl_dn17))) * assign14630_e20428) - (assign14630_e20425 * (locals.var_fac1p2_dn17 * locals.var_beta2))) / (assign14630_e20428 * assign14630_e20428)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign14630_e20432;
        locals.var_tx_dn0 = assign14630_e20432_d_n0;
        locals.var_tx_dn2 = assign14630_e20432_d_n2;
        locals.var_tx_dn6 = assign14630_e20432_d_n6;
        locals.var_tx_dn7 = assign14630_e20432_d_n7;
        locals.var_tx_dn10 = assign14630_e20432_d_n10;
        locals.var_tx_dn11 = assign14630_e20432_d_n11;
        locals.var_tx_dn12 = assign14630_e20432_d_n12;
        locals.var_tx_dn17 = assign14630_e20432_d_n17;

        let (assign14640_e20452, assign14640_e20452_d_n0, assign14640_e20452_d_n2, assign14640_e20452_d_n6, assign14640_e20452_d_n7, assign14640_e20452_d_n10, assign14640_e20452_d_n11, assign14640_e20452_d_n12, assign14640_e20452_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) {
        let assign14640_e20444: f64 = (10.0 * 2.220446049250313e-16);
        let (assign14640_e20450, assign14640_e20450_d_n0, assign14640_e20450_d_n2, assign14640_e20450_d_n6, assign14640_e20450_d_n7, assign14640_e20450_d_n10, assign14640_e20450_d_n11, assign14640_e20450_d_n12, assign14640_e20450_d_n17,) = {
            if (locals.var_tx >= assign14640_e20444) {
                (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
            } else {
                let assign14640_e20449: f64 = (10.0 * 2.220446049250313e-16);
                (assign14640_e20449, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign14640_e20450, assign14640_e20450_d_n0, assign14640_e20450_d_n2, assign14640_e20450_d_n6, assign14640_e20450_d_n7, assign14640_e20450_d_n10, assign14640_e20450_d_n11, assign14640_e20450_d_n12, assign14640_e20450_d_n17,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign14640_e20452;
        locals.var_tx_dn0 = assign14640_e20452_d_n0;
        locals.var_tx_dn2 = assign14640_e20452_d_n2;
        locals.var_tx_dn6 = assign14640_e20452_d_n6;
        locals.var_tx_dn7 = assign14640_e20452_d_n7;
        locals.var_tx_dn10 = assign14640_e20452_d_n10;
        locals.var_tx_dn11 = assign14640_e20452_d_n11;
        locals.var_tx_dn12 = assign14640_e20452_d_n12;
        locals.var_tx_dn17 = assign14640_e20452_d_n17;

        let (assign14650_e20474, assign14650_e20474_d_n0, assign14650_e20474_d_n2, assign14650_e20474_d_n6, assign14650_e20474_d_n7, assign14650_e20474_d_n10, assign14650_e20474_d_n11, assign14650_e20474_d_n12, assign14650_e20474_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) {
        let assign14650_e20464: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign14650_e20466: f64 = (assign14650_e20464 * 0.5);
        let assign14650_e20469: f64 = (locals.var_tx).sqrt();
        let assign14650_e20470: f64 = (1.0 - assign14650_e20469);
        let assign14650_e20471: f64 = (assign14650_e20466 * assign14650_e20470);
        let assign14650_e20472: f64 = (locals.var_vgp + assign14650_e20471);
        (assign14650_e20472, (locals.var_vgp_dn0 + ((((locals.var_fac1p2_dn0 * locals.var_beta) * 0.5) * assign14650_e20470) + (assign14650_e20466 * (-(locals.var_tx_dn0 / (2.0 * assign14650_e20469)))))), (locals.var_vgp_dn2 + ((((locals.var_fac1p2_dn2 * locals.var_beta) * 0.5) * assign14650_e20470) + (assign14650_e20466 * (-(locals.var_tx_dn2 / (2.0 * assign14650_e20469)))))), (locals.var_vgp_dn6 + ((((locals.var_fac1p2_dn6 * locals.var_beta) * 0.5) * assign14650_e20470) + (assign14650_e20466 * (-(locals.var_tx_dn6 / (2.0 * assign14650_e20469)))))), (locals.var_vgp_dn7 + ((((locals.var_fac1p2_dn7 * locals.var_beta) * 0.5) * assign14650_e20470) + (assign14650_e20466 * (-(locals.var_tx_dn7 / (2.0 * assign14650_e20469)))))), (locals.var_vgp_dn10 + (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) * 0.5) * assign14650_e20470) + (assign14650_e20466 * (-(locals.var_tx_dn10 / (2.0 * assign14650_e20469)))))), (locals.var_vgp_dn11 + ((((locals.var_fac1p2_dn11 * locals.var_beta) * 0.5) * assign14650_e20470) + (assign14650_e20466 * (-(locals.var_tx_dn11 / (2.0 * assign14650_e20469)))))), (locals.var_vgp_dn12 + ((((locals.var_fac1p2_dn12 * locals.var_beta) * 0.5) * assign14650_e20470) + (assign14650_e20466 * (-(locals.var_tx_dn12 / (2.0 * assign14650_e20469)))))), (locals.var_vgp_dn17 + ((((locals.var_fac1p2_dn17 * locals.var_beta) * 0.5) * assign14650_e20470) + (assign14650_e20466 * (-(locals.var_tx_dn17 / (2.0 * assign14650_e20469)))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign14650_e20474;
        locals.var_ps0_inia_dn0 = assign14650_e20474_d_n0;
        locals.var_ps0_inia_dn2 = assign14650_e20474_d_n2;
        locals.var_ps0_inia_dn6 = assign14650_e20474_d_n6;
        locals.var_ps0_inia_dn7 = assign14650_e20474_d_n7;
        locals.var_ps0_inia_dn10 = assign14650_e20474_d_n10;
        locals.var_ps0_inia_dn11 = assign14650_e20474_d_n11;
        locals.var_ps0_inia_dn12 = assign14650_e20474_d_n12;
        locals.var_ps0_inia_dn17 = assign14650_e20474_d_n17;

    }

    pub(super) fn stamp_transient_block_47(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14660_e20489, assign14660_e20489_d_n0, assign14660_e20489_d_n2, assign14660_e20489_d_n6, assign14660_e20489_d_n7, assign14660_e20489_d_n10, assign14660_e20489_d_n11, assign14660_e20489_d_n12, assign14660_e20489_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) {
        let assign14660_e20486: f64 = (locals.var_ps0_inia - locals.var_vbcs_cl);
        let assign14660_e20487: f64 = (locals.var_beta * assign14660_e20486);
        (assign14660_e20487, (locals.var_beta * (locals.var_ps0_inia_dn0 - locals.var_vbcs_cl_dn0)), (locals.var_beta * (locals.var_ps0_inia_dn2 - locals.var_vbcs_cl_dn2)), (locals.var_beta * (locals.var_ps0_inia_dn6 - locals.var_vbcs_cl_dn6)), (locals.var_beta * (locals.var_ps0_inia_dn7 - locals.var_vbcs_cl_dn7)), ((locals.var_beta_dn10 * assign14660_e20486) + (locals.var_beta * (locals.var_ps0_inia_dn10 - locals.var_vbcs_cl_dn10))), (locals.var_beta * (locals.var_ps0_inia_dn11 - locals.var_vbcs_cl_dn11)), (locals.var_beta * (locals.var_ps0_inia_dn12 - locals.var_vbcs_cl_dn12)), (locals.var_beta * (locals.var_ps0_inia_dn17 - locals.var_vbcs_cl_dn17)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, locals.var_chi_dn17,)
    }
};
        locals.var_chi = assign14660_e20489;
        locals.var_chi_dn0 = assign14660_e20489_d_n0;
        locals.var_chi_dn2 = assign14660_e20489_d_n2;
        locals.var_chi_dn6 = assign14660_e20489_d_n6;
        locals.var_chi_dn7 = assign14660_e20489_d_n7;
        locals.var_chi_dn10 = assign14660_e20489_d_n10;
        locals.var_chi_dn11 = assign14660_e20489_d_n11;
        locals.var_chi_dn12 = assign14660_e20489_d_n12;
        locals.var_chi_dn17 = assign14660_e20489_d_n17;

        let assign14670_e20492: f64 = if locals.var_chi < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard447 = assign14670_e20492;

        let (assign14680_e20509, assign14680_e20509_d_n0, assign14680_e20509_d_n2, assign14680_e20509_d_n6, assign14680_e20509_d_n7, assign14680_e20509_d_n10, assign14680_e20509_d_n11, assign14680_e20509_d_n12, assign14680_e20509_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 != 0.0)) {
        let assign14680_e20506: f64 = (locals.var_vgp - locals.var_vbcs_cl);
        let assign14680_e20507: f64 = (locals.var_beta * assign14680_e20506);
        (assign14680_e20507, (locals.var_beta * (locals.var_vgp_dn0 - locals.var_vbcs_cl_dn0)), (locals.var_beta * (locals.var_vgp_dn2 - locals.var_vbcs_cl_dn2)), (locals.var_beta * (locals.var_vgp_dn6 - locals.var_vbcs_cl_dn6)), (locals.var_beta * (locals.var_vgp_dn7 - locals.var_vbcs_cl_dn7)), ((locals.var_beta_dn10 * assign14680_e20506) + (locals.var_beta * (locals.var_vgp_dn10 - locals.var_vbcs_cl_dn10))), (locals.var_beta * (locals.var_vgp_dn11 - locals.var_vbcs_cl_dn11)), (locals.var_beta * (locals.var_vgp_dn12 - locals.var_vbcs_cl_dn12)), (locals.var_beta * (locals.var_vgp_dn17 - locals.var_vbcs_cl_dn17)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
        locals.var_ty = assign14680_e20509;
        locals.var_ty_dn0 = assign14680_e20509_d_n0;
        locals.var_ty_dn2 = assign14680_e20509_d_n2;
        locals.var_ty_dn6 = assign14680_e20509_d_n6;
        locals.var_ty_dn7 = assign14680_e20509_d_n7;
        locals.var_ty_dn10 = assign14680_e20509_d_n10;
        locals.var_ty_dn11 = assign14680_e20509_d_n11;
        locals.var_ty_dn12 = assign14680_e20509_d_n12;
        locals.var_ty_dn17 = assign14680_e20509_d_n17;

        let (assign14690_e20530, assign14690_e20530_d_n0, assign14690_e20530_d_n2, assign14690_e20530_d_n6, assign14690_e20530_d_n7, assign14690_e20530_d_n10, assign14690_e20530_d_n11, assign14690_e20530_d_n12, assign14690_e20530_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 != 0.0)) {
        let assign14690_e20523: f64 = (1.414213562373095 / 108.0);
        let assign14690_e20525: f64 = (assign14690_e20523 * locals.var_beta);
        let assign14690_e20527: f64 = (assign14690_e20525 * locals.var_fac1);
        let assign14690_e20528: f64 = (1.0 / assign14690_e20527);
        (assign14690_e20528, (-((assign14690_e20525 * locals.var_fac1_dn0) / (assign14690_e20527 * assign14690_e20527))), (-((assign14690_e20525 * locals.var_fac1_dn2) / (assign14690_e20527 * assign14690_e20527))), (-((assign14690_e20525 * locals.var_fac1_dn6) / (assign14690_e20527 * assign14690_e20527))), (-((assign14690_e20525 * locals.var_fac1_dn7) / (assign14690_e20527 * assign14690_e20527))), (-((((assign14690_e20523 * locals.var_beta_dn10) * locals.var_fac1) + (assign14690_e20525 * locals.var_fac1_dn10)) / (assign14690_e20527 * assign14690_e20527))), (-((assign14690_e20525 * locals.var_fac1_dn11) / (assign14690_e20527 * assign14690_e20527))), (-((assign14690_e20525 * locals.var_fac1_dn12) / (assign14690_e20527 * assign14690_e20527))), (-((assign14690_e20525 * locals.var_fac1_dn17) / (assign14690_e20527 * assign14690_e20527))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign14690_e20530;
        locals.var_t1_dn0 = assign14690_e20530_d_n0;
        locals.var_t1_dn2 = assign14690_e20530_d_n2;
        locals.var_t1_dn6 = assign14690_e20530_d_n6;
        locals.var_t1_dn7 = assign14690_e20530_d_n7;
        locals.var_t1_dn10 = assign14690_e20530_d_n10;
        locals.var_t1_dn11 = assign14690_e20530_d_n11;
        locals.var_t1_dn12 = assign14690_e20530_d_n12;
        locals.var_t1_dn17 = assign14690_e20530_d_n17;

        let (assign14700_e20547, assign14700_e20547_d_n0, assign14700_e20547_d_n2, assign14700_e20547_d_n6, assign14700_e20547_d_n7, assign14700_e20547_d_n10, assign14700_e20547_d_n11, assign14700_e20547_d_n12, assign14700_e20547_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 != 0.0)) {
        let assign14700_e20544: f64 = (3.0 * locals.var_t1);
        let assign14700_e20545: f64 = (81.0 + assign14700_e20544);
        (assign14700_e20545, (3.0 * locals.var_t1_dn0), (3.0 * locals.var_t1_dn2), (3.0 * locals.var_t1_dn6), (3.0 * locals.var_t1_dn7), (3.0 * locals.var_t1_dn10), (3.0 * locals.var_t1_dn11), (3.0 * locals.var_t1_dn12), (3.0 * locals.var_t1_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign14700_e20547;
        locals.var_t2_dn0 = assign14700_e20547_d_n0;
        locals.var_t2_dn2 = assign14700_e20547_d_n2;
        locals.var_t2_dn6 = assign14700_e20547_d_n6;
        locals.var_t2_dn7 = assign14700_e20547_d_n7;
        locals.var_t2_dn10 = assign14700_e20547_d_n10;
        locals.var_t2_dn11 = assign14700_e20547_d_n11;
        locals.var_t2_dn12 = assign14700_e20547_d_n12;
        locals.var_t2_dn17 = assign14700_e20547_d_n17;

        let (assign14710_e20571, assign14710_e20571_d_n0, assign14710_e20571_d_n2, assign14710_e20571_d_n6, assign14710_e20571_d_n7, assign14710_e20571_d_n10, assign14710_e20571_d_n11, assign14710_e20571_d_n12, assign14710_e20571_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 != 0.0)) {
        let assign14710_e20559: f64 = (-2916.0);
        let assign14710_e20562: f64 = (81.0 * locals.var_t1);
        let assign14710_e20563: f64 = (assign14710_e20559 - assign14710_e20562);
        let assign14710_e20566: f64 = (27.0 * locals.var_t1);
        let assign14710_e20568: f64 = (assign14710_e20566 * locals.var_ty);
        let assign14710_e20569: f64 = (assign14710_e20563 + assign14710_e20568);
        (assign14710_e20569, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign14710_e20566 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign14710_e20566 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign14710_e20566 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign14710_e20566 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign14710_e20566 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn11)) + (((27.0 * locals.var_t1_dn11) * locals.var_ty) + (assign14710_e20566 * locals.var_ty_dn11))), ((-(81.0 * locals.var_t1_dn12)) + (((27.0 * locals.var_t1_dn12) * locals.var_ty) + (assign14710_e20566 * locals.var_ty_dn12))), ((-(81.0 * locals.var_t1_dn17)) + (((27.0 * locals.var_t1_dn17) * locals.var_ty) + (assign14710_e20566 * locals.var_ty_dn17))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign14710_e20571;
        locals.var_t3_dn0 = assign14710_e20571_d_n0;
        locals.var_t3_dn2 = assign14710_e20571_d_n2;
        locals.var_t3_dn6 = assign14710_e20571_d_n6;
        locals.var_t3_dn7 = assign14710_e20571_d_n7;
        locals.var_t3_dn10 = assign14710_e20571_d_n10;
        locals.var_t3_dn11 = assign14710_e20571_d_n11;
        locals.var_t3_dn12 = assign14710_e20571_d_n12;
        locals.var_t3_dn17 = assign14710_e20571_d_n17;

        let (assign14720_e20596, assign14720_e20596_d_n0, assign14720_e20596_d_n2, assign14720_e20596_d_n6, assign14720_e20596_d_n7, assign14720_e20596_d_n10, assign14720_e20596_d_n11, assign14720_e20596_d_n12, assign14720_e20596_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 != 0.0)) {
        let assign14720_e20586: f64 = (54.0 + locals.var_t1);
        let assign14720_e20587: f64 = (81.0 * assign14720_e20586);
        let assign14720_e20588: f64 = (1458.0 - assign14720_e20587);
        let assign14720_e20591: f64 = (27.0 * locals.var_t1);
        let assign14720_e20593: f64 = (assign14720_e20591 * locals.var_ty);
        let assign14720_e20594: f64 = (assign14720_e20588 + assign14720_e20593);
        (assign14720_e20594, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign14720_e20591 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign14720_e20591 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign14720_e20591 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign14720_e20591 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign14720_e20591 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn11)) + (((27.0 * locals.var_t1_dn11) * locals.var_ty) + (assign14720_e20591 * locals.var_ty_dn11))), ((-(81.0 * locals.var_t1_dn12)) + (((27.0 * locals.var_t1_dn12) * locals.var_ty) + (assign14720_e20591 * locals.var_ty_dn12))), ((-(81.0 * locals.var_t1_dn17)) + (((27.0 * locals.var_t1_dn17) * locals.var_ty) + (assign14720_e20591 * locals.var_ty_dn17))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign14720_e20596;
        locals.var_t4_dn0 = assign14720_e20596_d_n0;
        locals.var_t4_dn2 = assign14720_e20596_d_n2;
        locals.var_t4_dn6 = assign14720_e20596_d_n6;
        locals.var_t4_dn7 = assign14720_e20596_d_n7;
        locals.var_t4_dn10 = assign14720_e20596_d_n10;
        locals.var_t4_dn11 = assign14720_e20596_d_n11;
        locals.var_t4_dn12 = assign14720_e20596_d_n12;
        locals.var_t4_dn17 = assign14720_e20596_d_n17;

        let (assign14730_e20611, assign14730_e20611_d_n0, assign14730_e20611_d_n2, assign14730_e20611_d_n6, assign14730_e20611_d_n7, assign14730_e20611_d_n10, assign14730_e20611_d_n11, assign14730_e20611_d_n12, assign14730_e20611_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 != 0.0)) {
        let assign14730_e20609: f64 = (locals.var_t4 * locals.var_t4);
        (assign14730_e20609, ((locals.var_t4_dn0 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn0)), ((locals.var_t4_dn2 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn2)), ((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)), ((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)), ((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)), ((locals.var_t4_dn11 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn11)), ((locals.var_t4_dn12 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn12)), ((locals.var_t4_dn17 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn17)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign14730_e20611;
        locals.var_t4_dn0 = assign14730_e20611_d_n0;
        locals.var_t4_dn2 = assign14730_e20611_d_n2;
        locals.var_t4_dn6 = assign14730_e20611_d_n6;
        locals.var_t4_dn7 = assign14730_e20611_d_n7;
        locals.var_t4_dn10 = assign14730_e20611_d_n10;
        locals.var_t4_dn11 = assign14730_e20611_d_n11;
        locals.var_t4_dn12 = assign14730_e20611_d_n12;
        locals.var_t4_dn17 = assign14730_e20611_d_n17;

        let (assign14740_e20637, assign14740_e20637_d_n0, assign14740_e20637_d_n2, assign14740_e20637_d_n6, assign14740_e20637_d_n7, assign14740_e20637_d_n10, assign14740_e20637_d_n11, assign14740_e20637_d_n12, assign14740_e20637_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 != 0.0)) {
        let assign14740_e20625: f64 = (4.0 * locals.var_t2);
        let assign14740_e20627: f64 = (assign14740_e20625 * locals.var_t2);
        let assign14740_e20629: f64 = (assign14740_e20627 * locals.var_t2);
        let assign14740_e20631: f64 = (assign14740_e20629 + locals.var_t4);
        let assign14740_e20632: f64 = (assign14740_e20631).sqrt();
        let assign14740_e20633: f64 = (locals.var_t3 + assign14740_e20632);
        let assign14740_e20635: f64 = (assign14740_e20633).powf(0.3333333333333333);
        (assign14740_e20635, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14740_e20633).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign14740_e20625 * locals.var_t2_dn0)) * locals.var_t2) + (assign14740_e20627 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign14740_e20632))))) } } else { (assign14740_e20635 * (0.3333333333333333 * ((locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign14740_e20625 * locals.var_t2_dn0)) * locals.var_t2) + (assign14740_e20627 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign14740_e20632))) / assign14740_e20633))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14740_e20633).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign14740_e20625 * locals.var_t2_dn2)) * locals.var_t2) + (assign14740_e20627 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign14740_e20632))))) } } else { (assign14740_e20635 * (0.3333333333333333 * ((locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign14740_e20625 * locals.var_t2_dn2)) * locals.var_t2) + (assign14740_e20627 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign14740_e20632))) / assign14740_e20633))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14740_e20633).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign14740_e20625 * locals.var_t2_dn6)) * locals.var_t2) + (assign14740_e20627 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign14740_e20632))))) } } else { (assign14740_e20635 * (0.3333333333333333 * ((locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign14740_e20625 * locals.var_t2_dn6)) * locals.var_t2) + (assign14740_e20627 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign14740_e20632))) / assign14740_e20633))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14740_e20633).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign14740_e20625 * locals.var_t2_dn7)) * locals.var_t2) + (assign14740_e20627 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign14740_e20632))))) } } else { (assign14740_e20635 * (0.3333333333333333 * ((locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign14740_e20625 * locals.var_t2_dn7)) * locals.var_t2) + (assign14740_e20627 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign14740_e20632))) / assign14740_e20633))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14740_e20633).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign14740_e20625 * locals.var_t2_dn10)) * locals.var_t2) + (assign14740_e20627 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign14740_e20632))))) } } else { (assign14740_e20635 * (0.3333333333333333 * ((locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign14740_e20625 * locals.var_t2_dn10)) * locals.var_t2) + (assign14740_e20627 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign14740_e20632))) / assign14740_e20633))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14740_e20633).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn11 + (((((((4.0 * locals.var_t2_dn11) * locals.var_t2) + (assign14740_e20625 * locals.var_t2_dn11)) * locals.var_t2) + (assign14740_e20627 * locals.var_t2_dn11)) + locals.var_t4_dn11) / (2.0 * assign14740_e20632))))) } } else { (assign14740_e20635 * (0.3333333333333333 * ((locals.var_t3_dn11 + (((((((4.0 * locals.var_t2_dn11) * locals.var_t2) + (assign14740_e20625 * locals.var_t2_dn11)) * locals.var_t2) + (assign14740_e20627 * locals.var_t2_dn11)) + locals.var_t4_dn11) / (2.0 * assign14740_e20632))) / assign14740_e20633))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14740_e20633).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn12 + (((((((4.0 * locals.var_t2_dn12) * locals.var_t2) + (assign14740_e20625 * locals.var_t2_dn12)) * locals.var_t2) + (assign14740_e20627 * locals.var_t2_dn12)) + locals.var_t4_dn12) / (2.0 * assign14740_e20632))))) } } else { (assign14740_e20635 * (0.3333333333333333 * ((locals.var_t3_dn12 + (((((((4.0 * locals.var_t2_dn12) * locals.var_t2) + (assign14740_e20625 * locals.var_t2_dn12)) * locals.var_t2) + (assign14740_e20627 * locals.var_t2_dn12)) + locals.var_t4_dn12) / (2.0 * assign14740_e20632))) / assign14740_e20633))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14740_e20633).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn17 + (((((((4.0 * locals.var_t2_dn17) * locals.var_t2) + (assign14740_e20625 * locals.var_t2_dn17)) * locals.var_t2) + (assign14740_e20627 * locals.var_t2_dn17)) + locals.var_t4_dn17) / (2.0 * assign14740_e20632))))) } } else { (assign14740_e20635 * (0.3333333333333333 * ((locals.var_t3_dn17 + (((((((4.0 * locals.var_t2_dn17) * locals.var_t2) + (assign14740_e20625 * locals.var_t2_dn17)) * locals.var_t2) + (assign14740_e20627 * locals.var_t2_dn17)) + locals.var_t4_dn17) / (2.0 * assign14740_e20632))) / assign14740_e20633))) },)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign14740_e20637;
        locals.var_t5_dn0 = assign14740_e20637_d_n0;
        locals.var_t5_dn2 = assign14740_e20637_d_n2;
        locals.var_t5_dn6 = assign14740_e20637_d_n6;
        locals.var_t5_dn7 = assign14740_e20637_d_n7;
        locals.var_t5_dn10 = assign14740_e20637_d_n10;
        locals.var_t5_dn11 = assign14740_e20637_d_n11;
        locals.var_t5_dn12 = assign14740_e20637_d_n12;
        locals.var_t5_dn17 = assign14740_e20637_d_n17;

        let (assign14750_e20666, assign14750_e20666_d_n0, assign14750_e20666_d_n2, assign14750_e20666_d_n6, assign14750_e20666_d_n7, assign14750_e20666_d_n10, assign14750_e20666_d_n11, assign14750_e20666_d_n12, assign14750_e20666_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 != 0.0)) {
        let assign14750_e20651: f64 = (1.259921049894873 * locals.var_t2);
        let assign14750_e20654: f64 = (3.0 * locals.var_t5);
        let assign14750_e20655: f64 = (assign14750_e20651 / assign14750_e20654);
        let assign14750_e20656: f64 = (3.0 - assign14750_e20655);
        let assign14750_e20660: f64 = (3.0 * 1.259921049894873);
        let assign14750_e20661: f64 = (1.0 / assign14750_e20660);
        let assign14750_e20663: f64 = (assign14750_e20661 * locals.var_t5);
        let assign14750_e20664: f64 = (assign14750_e20656 + assign14750_e20663);
        (assign14750_e20664, ((-((((1.259921049894873 * locals.var_t2_dn0) * assign14750_e20654) - (assign14750_e20651 * (3.0 * locals.var_t5_dn0))) / (assign14750_e20654 * assign14750_e20654))) + (assign14750_e20661 * locals.var_t5_dn0)), ((-((((1.259921049894873 * locals.var_t2_dn2) * assign14750_e20654) - (assign14750_e20651 * (3.0 * locals.var_t5_dn2))) / (assign14750_e20654 * assign14750_e20654))) + (assign14750_e20661 * locals.var_t5_dn2)), ((-((((1.259921049894873 * locals.var_t2_dn6) * assign14750_e20654) - (assign14750_e20651 * (3.0 * locals.var_t5_dn6))) / (assign14750_e20654 * assign14750_e20654))) + (assign14750_e20661 * locals.var_t5_dn6)), ((-((((1.259921049894873 * locals.var_t2_dn7) * assign14750_e20654) - (assign14750_e20651 * (3.0 * locals.var_t5_dn7))) / (assign14750_e20654 * assign14750_e20654))) + (assign14750_e20661 * locals.var_t5_dn7)), ((-((((1.259921049894873 * locals.var_t2_dn10) * assign14750_e20654) - (assign14750_e20651 * (3.0 * locals.var_t5_dn10))) / (assign14750_e20654 * assign14750_e20654))) + (assign14750_e20661 * locals.var_t5_dn10)), ((-((((1.259921049894873 * locals.var_t2_dn11) * assign14750_e20654) - (assign14750_e20651 * (3.0 * locals.var_t5_dn11))) / (assign14750_e20654 * assign14750_e20654))) + (assign14750_e20661 * locals.var_t5_dn11)), ((-((((1.259921049894873 * locals.var_t2_dn12) * assign14750_e20654) - (assign14750_e20651 * (3.0 * locals.var_t5_dn12))) / (assign14750_e20654 * assign14750_e20654))) + (assign14750_e20661 * locals.var_t5_dn12)), ((-((((1.259921049894873 * locals.var_t2_dn17) * assign14750_e20654) - (assign14750_e20651 * (3.0 * locals.var_t5_dn17))) / (assign14750_e20654 * assign14750_e20654))) + (assign14750_e20661 * locals.var_t5_dn17)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign14750_e20666;
        locals.var_tx_dn0 = assign14750_e20666_d_n0;
        locals.var_tx_dn2 = assign14750_e20666_d_n2;
        locals.var_tx_dn6 = assign14750_e20666_d_n6;
        locals.var_tx_dn7 = assign14750_e20666_d_n7;
        locals.var_tx_dn10 = assign14750_e20666_d_n10;
        locals.var_tx_dn11 = assign14750_e20666_d_n11;
        locals.var_tx_dn12 = assign14750_e20666_d_n12;
        locals.var_tx_dn17 = assign14750_e20666_d_n17;

        let (assign14760_e20683, assign14760_e20683_d_n0, assign14760_e20683_d_n2, assign14760_e20683_d_n6, assign14760_e20683_d_n7, assign14760_e20683_d_n10, assign14760_e20683_d_n11, assign14760_e20683_d_n12, assign14760_e20683_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 != 0.0)) {
        let assign14760_e20679: f64 = (locals.var_tx * locals.var_beta_inv);
        let assign14760_e20681: f64 = (assign14760_e20679 + locals.var_vbcs_cl);
        (assign14760_e20681, ((locals.var_tx_dn0 * locals.var_beta_inv) + locals.var_vbcs_cl_dn0), ((locals.var_tx_dn2 * locals.var_beta_inv) + locals.var_vbcs_cl_dn2), ((locals.var_tx_dn6 * locals.var_beta_inv) + locals.var_vbcs_cl_dn6), ((locals.var_tx_dn7 * locals.var_beta_inv) + locals.var_vbcs_cl_dn7), (((locals.var_tx_dn10 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn10)) + locals.var_vbcs_cl_dn10), ((locals.var_tx_dn11 * locals.var_beta_inv) + locals.var_vbcs_cl_dn11), ((locals.var_tx_dn12 * locals.var_beta_inv) + locals.var_vbcs_cl_dn12), ((locals.var_tx_dn17 * locals.var_beta_inv) + locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign14760_e20683;
        locals.var_ps0_inia_dn0 = assign14760_e20683_d_n0;
        locals.var_ps0_inia_dn2 = assign14760_e20683_d_n2;
        locals.var_ps0_inia_dn6 = assign14760_e20683_d_n6;
        locals.var_ps0_inia_dn7 = assign14760_e20683_d_n7;
        locals.var_ps0_inia_dn10 = assign14760_e20683_d_n10;
        locals.var_ps0_inia_dn11 = assign14760_e20683_d_n11;
        locals.var_ps0_inia_dn12 = assign14760_e20683_d_n12;
        locals.var_ps0_inia_dn17 = assign14760_e20683_d_n17;

        let (assign14770_e20696, assign14770_e20696_d_n0, assign14770_e20696_d_n2, assign14770_e20696_d_n6, assign14770_e20696_d_n7, assign14770_e20696_d_n10, assign14770_e20696_d_n11, assign14770_e20696_d_n12, assign14770_e20696_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign14770_e20696;
        locals.var_ps0_ini_dn0 = assign14770_e20696_d_n0;
        locals.var_ps0_ini_dn2 = assign14770_e20696_d_n2;
        locals.var_ps0_ini_dn6 = assign14770_e20696_d_n6;
        locals.var_ps0_ini_dn7 = assign14770_e20696_d_n7;
        locals.var_ps0_ini_dn10 = assign14770_e20696_d_n10;
        locals.var_ps0_ini_dn11 = assign14770_e20696_d_n11;
        locals.var_ps0_ini_dn12 = assign14770_e20696_d_n12;
        locals.var_ps0_ini_dn17 = assign14770_e20696_d_n17;

        let assign14780_e20699: f64 = if locals.var_vgs <= locals.var_vth { 1.0 } else { 0.0 };
        locals.var_guard448 = assign14780_e20699;

        let (assign14790_e20715, assign14790_e20715_d_n0, assign14790_e20715_d_n2, assign14790_e20715_d_n6, assign14790_e20715_d_n7, assign14790_e20715_d_n10, assign14790_e20715_d_n11, assign14790_e20715_d_n12, assign14790_e20715_d_n17,) = {
    if (((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 == 0.0)) && (locals.var_guard448 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign14790_e20715;
        locals.var_ps0_ini_dn0 = assign14790_e20715_d_n0;
        locals.var_ps0_ini_dn2 = assign14790_e20715_d_n2;
        locals.var_ps0_ini_dn6 = assign14790_e20715_d_n6;
        locals.var_ps0_ini_dn7 = assign14790_e20715_d_n7;
        locals.var_ps0_ini_dn10 = assign14790_e20715_d_n10;
        locals.var_ps0_ini_dn11 = assign14790_e20715_d_n11;
        locals.var_ps0_ini_dn12 = assign14790_e20715_d_n12;
        locals.var_ps0_ini_dn17 = assign14790_e20715_d_n17;

        let (assign14800_e20736, assign14800_e20736_d_n0, assign14800_e20736_d_n2, assign14800_e20736_d_n6, assign14800_e20736_d_n7, assign14800_e20736_d_n10, assign14800_e20736_d_n11, assign14800_e20736_d_n12, assign14800_e20736_d_n17,) = {
    if (((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 == 0.0)) && (locals.var_guard448 == 0.0)) {
        let assign14800_e20732: f64 = (1.0 / locals.var_cnst1soi);
        let assign14800_e20734: f64 = (assign14800_e20732 / locals.var_cnstc_foxi);
        (assign14800_e20734, ((((-(locals.var_cnst1soi_dn0 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14800_e20732 * locals.var_cnstc_foxi_dn0)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn2 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14800_e20732 * locals.var_cnstc_foxi_dn2)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn6 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14800_e20732 * locals.var_cnstc_foxi_dn6)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn7 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14800_e20732 * locals.var_cnstc_foxi_dn7)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn10 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14800_e20732 * locals.var_cnstc_foxi_dn10)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn11 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14800_e20732 * locals.var_cnstc_foxi_dn11)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn12 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14800_e20732 * locals.var_cnstc_foxi_dn12)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn17 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14800_e20732 * locals.var_cnstc_foxi_dn17)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign14800_e20736;
        locals.var_t1_dn0 = assign14800_e20736_d_n0;
        locals.var_t1_dn2 = assign14800_e20736_d_n2;
        locals.var_t1_dn6 = assign14800_e20736_d_n6;
        locals.var_t1_dn7 = assign14800_e20736_d_n7;
        locals.var_t1_dn10 = assign14800_e20736_d_n10;
        locals.var_t1_dn11 = assign14800_e20736_d_n11;
        locals.var_t1_dn12 = assign14800_e20736_d_n12;
        locals.var_t1_dn17 = assign14800_e20736_d_n17;

        let (assign14810_e20757, assign14810_e20757_d_n0, assign14810_e20757_d_n2, assign14810_e20757_d_n6, assign14810_e20757_d_n7, assign14810_e20757_d_n10, assign14810_e20757_d_n11, assign14810_e20757_d_n12, assign14810_e20757_d_n17,) = {
    if (((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 == 0.0)) && (locals.var_guard448 == 0.0)) {
        let assign14810_e20753: f64 = (locals.var_t1 * locals.var_vgp);
        let assign14810_e20755: f64 = (assign14810_e20753 * locals.var_vgp);
        (assign14810_e20755, ((((locals.var_t1_dn0 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn0)) * locals.var_vgp) + (assign14810_e20753 * locals.var_vgp_dn0)), ((((locals.var_t1_dn2 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn2)) * locals.var_vgp) + (assign14810_e20753 * locals.var_vgp_dn2)), ((((locals.var_t1_dn6 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn6)) * locals.var_vgp) + (assign14810_e20753 * locals.var_vgp_dn6)), ((((locals.var_t1_dn7 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn7)) * locals.var_vgp) + (assign14810_e20753 * locals.var_vgp_dn7)), ((((locals.var_t1_dn10 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn10)) * locals.var_vgp) + (assign14810_e20753 * locals.var_vgp_dn10)), ((((locals.var_t1_dn11 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn11)) * locals.var_vgp) + (assign14810_e20753 * locals.var_vgp_dn11)), ((((locals.var_t1_dn12 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn12)) * locals.var_vgp) + (assign14810_e20753 * locals.var_vgp_dn12)), ((((locals.var_t1_dn17 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn17)) * locals.var_vgp) + (assign14810_e20753 * locals.var_vgp_dn17)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign14810_e20757;
        locals.var_t2_dn0 = assign14810_e20757_d_n0;
        locals.var_t2_dn2 = assign14810_e20757_d_n2;
        locals.var_t2_dn6 = assign14810_e20757_d_n6;
        locals.var_t2_dn7 = assign14810_e20757_d_n7;
        locals.var_t2_dn10 = assign14810_e20757_d_n10;
        locals.var_t2_dn11 = assign14810_e20757_d_n11;
        locals.var_t2_dn12 = assign14810_e20757_d_n12;
        locals.var_t2_dn17 = assign14810_e20757_d_n17;

        let (assign14820_e20778, assign14820_e20778_d_n0, assign14820_e20778_d_n2, assign14820_e20778_d_n6, assign14820_e20778_d_n7, assign14820_e20778_d_n10, assign14820_e20778_d_n11, assign14820_e20778_d_n12, assign14820_e20778_d_n17,) = {
    if (((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 == 0.0)) && (locals.var_guard448 == 0.0)) {
        let assign14820_e20775: f64 = (2.0 / locals.var_vgp);
        let assign14820_e20776: f64 = (locals.var_beta + assign14820_e20775);
        (assign14820_e20776, (-((2.0 * locals.var_vgp_dn0) / (locals.var_vgp * locals.var_vgp))), (-((2.0 * locals.var_vgp_dn2) / (locals.var_vgp * locals.var_vgp))), (-((2.0 * locals.var_vgp_dn6) / (locals.var_vgp * locals.var_vgp))), (-((2.0 * locals.var_vgp_dn7) / (locals.var_vgp * locals.var_vgp))), (locals.var_beta_dn10 + (-((2.0 * locals.var_vgp_dn10) / (locals.var_vgp * locals.var_vgp)))), (-((2.0 * locals.var_vgp_dn11) / (locals.var_vgp * locals.var_vgp))), (-((2.0 * locals.var_vgp_dn12) / (locals.var_vgp * locals.var_vgp))), (-((2.0 * locals.var_vgp_dn17) / (locals.var_vgp * locals.var_vgp))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign14820_e20778;
        locals.var_t3_dn0 = assign14820_e20778_d_n0;
        locals.var_t3_dn2 = assign14820_e20778_d_n2;
        locals.var_t3_dn6 = assign14820_e20778_d_n6;
        locals.var_t3_dn7 = assign14820_e20778_d_n7;
        locals.var_t3_dn10 = assign14820_e20778_d_n10;
        locals.var_t3_dn11 = assign14820_e20778_d_n11;
        locals.var_t3_dn12 = assign14820_e20778_d_n12;
        locals.var_t3_dn17 = assign14820_e20778_d_n17;

        let (assign14830_e20798, assign14830_e20798_d_n0, assign14830_e20798_d_n2, assign14830_e20798_d_n6, assign14830_e20798_d_n7, assign14830_e20798_d_n10, assign14830_e20798_d_n11, assign14830_e20798_d_n12, assign14830_e20798_d_n17,) = {
    if (((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 == 0.0)) && (locals.var_guard448 == 0.0)) {
        let assign14830_e20794: f64 = (locals.var_t2).ln();
        let assign14830_e20796: f64 = (assign14830_e20794 / locals.var_t3);
        (assign14830_e20796, ((((locals.var_t2_dn0 / locals.var_t2) * locals.var_t3) - (assign14830_e20794 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn2 / locals.var_t2) * locals.var_t3) - (assign14830_e20794 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn6 / locals.var_t2) * locals.var_t3) - (assign14830_e20794 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn7 / locals.var_t2) * locals.var_t3) - (assign14830_e20794 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn10 / locals.var_t2) * locals.var_t3) - (assign14830_e20794 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn11 / locals.var_t2) * locals.var_t3) - (assign14830_e20794 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn12 / locals.var_t2) * locals.var_t3) - (assign14830_e20794 * locals.var_t3_dn12)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn17 / locals.var_t2) * locals.var_t3) - (assign14830_e20794 * locals.var_t3_dn17)) / (locals.var_t3 * locals.var_t3)),)
    } else {
        (locals.var_ps0_inib, locals.var_ps0_inib_dn0, locals.var_ps0_inib_dn2, locals.var_ps0_inib_dn6, locals.var_ps0_inib_dn7, locals.var_ps0_inib_dn10, locals.var_ps0_inib_dn11, locals.var_ps0_inib_dn12, locals.var_ps0_inib_dn17,)
    }
};
        locals.var_ps0_inib = assign14830_e20798;
        locals.var_ps0_inib_dn0 = assign14830_e20798_d_n0;
        locals.var_ps0_inib_dn2 = assign14830_e20798_d_n2;
        locals.var_ps0_inib_dn6 = assign14830_e20798_d_n6;
        locals.var_ps0_inib_dn7 = assign14830_e20798_d_n7;
        locals.var_ps0_inib_dn10 = assign14830_e20798_d_n10;
        locals.var_ps0_inib_dn11 = assign14830_e20798_d_n11;
        locals.var_ps0_inib_dn12 = assign14830_e20798_d_n12;
        locals.var_ps0_inib_dn17 = assign14830_e20798_d_n17;

        let (assign14840_e20819, assign14840_e20819_d_n0, assign14840_e20819_d_n2, assign14840_e20819_d_n6, assign14840_e20819_d_n7, assign14840_e20819_d_n10, assign14840_e20819_d_n11, assign14840_e20819_d_n12, assign14840_e20819_d_n17,) = {
    if (((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 == 0.0)) && (locals.var_guard448 == 0.0)) {
        let assign14840_e20815: f64 = (locals.var_ps0_inib - locals.var_ps0_inia);
        let assign14840_e20817: f64 = (assign14840_e20815 - 0.0008);
        (assign14840_e20817, (locals.var_ps0_inib_dn0 - locals.var_ps0_inia_dn0), (locals.var_ps0_inib_dn2 - locals.var_ps0_inia_dn2), (locals.var_ps0_inib_dn6 - locals.var_ps0_inia_dn6), (locals.var_ps0_inib_dn7 - locals.var_ps0_inia_dn7), (locals.var_ps0_inib_dn10 - locals.var_ps0_inia_dn10), (locals.var_ps0_inib_dn11 - locals.var_ps0_inia_dn11), (locals.var_ps0_inib_dn12 - locals.var_ps0_inia_dn12), (locals.var_ps0_inib_dn17 - locals.var_ps0_inia_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign14840_e20819;
        locals.var_tmf1_dn0 = assign14840_e20819_d_n0;
        locals.var_tmf1_dn2 = assign14840_e20819_d_n2;
        locals.var_tmf1_dn6 = assign14840_e20819_d_n6;
        locals.var_tmf1_dn7 = assign14840_e20819_d_n7;
        locals.var_tmf1_dn10 = assign14840_e20819_d_n10;
        locals.var_tmf1_dn11 = assign14840_e20819_d_n11;
        locals.var_tmf1_dn12 = assign14840_e20819_d_n12;
        locals.var_tmf1_dn17 = assign14840_e20819_d_n17;

        let (assign14850_e20840, assign14850_e20840_d_n0, assign14850_e20840_d_n2, assign14850_e20840_d_n6, assign14850_e20840_d_n7, assign14850_e20840_d_n10, assign14850_e20840_d_n11, assign14850_e20840_d_n12, assign14850_e20840_d_n17,) = {
    if (((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 == 0.0)) && (locals.var_guard448 == 0.0)) {
        let assign14850_e20836: f64 = (4.0 * locals.var_ps0_inib);
        let assign14850_e20838: f64 = (assign14850_e20836 * 0.0008);
        (assign14850_e20838, ((4.0 * locals.var_ps0_inib_dn0) * 0.0008), ((4.0 * locals.var_ps0_inib_dn2) * 0.0008), ((4.0 * locals.var_ps0_inib_dn6) * 0.0008), ((4.0 * locals.var_ps0_inib_dn7) * 0.0008), ((4.0 * locals.var_ps0_inib_dn10) * 0.0008), ((4.0 * locals.var_ps0_inib_dn11) * 0.0008), ((4.0 * locals.var_ps0_inib_dn12) * 0.0008), ((4.0 * locals.var_ps0_inib_dn17) * 0.0008),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign14850_e20840;
        locals.var_tmf2_dn0 = assign14850_e20840_d_n0;
        locals.var_tmf2_dn2 = assign14850_e20840_d_n2;
        locals.var_tmf2_dn6 = assign14850_e20840_d_n6;
        locals.var_tmf2_dn7 = assign14850_e20840_d_n7;
        locals.var_tmf2_dn10 = assign14850_e20840_d_n10;
        locals.var_tmf2_dn11 = assign14850_e20840_d_n11;
        locals.var_tmf2_dn12 = assign14850_e20840_d_n12;
        locals.var_tmf2_dn17 = assign14850_e20840_d_n17;

        let (assign14860_e20863, assign14860_e20863_d_n0, assign14860_e20863_d_n2, assign14860_e20863_d_n6, assign14860_e20863_d_n7, assign14860_e20863_d_n10, assign14860_e20863_d_n11, assign14860_e20863_d_n12, assign14860_e20863_d_n17,) = {
    if (((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 == 0.0)) && (locals.var_guard448 == 0.0)) {
        let (assign14860_e20861, assign14860_e20861_d_n0, assign14860_e20861_d_n2, assign14860_e20861_d_n6, assign14860_e20861_d_n7, assign14860_e20861_d_n10, assign14860_e20861_d_n11, assign14860_e20861_d_n12, assign14860_e20861_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign14860_e20860: f64 = (-locals.var_tmf2);
                (assign14860_e20860, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign14860_e20861, assign14860_e20861_d_n0, assign14860_e20861_d_n2, assign14860_e20861_d_n6, assign14860_e20861_d_n7, assign14860_e20861_d_n10, assign14860_e20861_d_n11, assign14860_e20861_d_n12, assign14860_e20861_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign14860_e20863;
        locals.var_tmf2_dn0 = assign14860_e20863_d_n0;
        locals.var_tmf2_dn2 = assign14860_e20863_d_n2;
        locals.var_tmf2_dn6 = assign14860_e20863_d_n6;
        locals.var_tmf2_dn7 = assign14860_e20863_d_n7;
        locals.var_tmf2_dn10 = assign14860_e20863_d_n10;
        locals.var_tmf2_dn11 = assign14860_e20863_d_n11;
        locals.var_tmf2_dn12 = assign14860_e20863_d_n12;
        locals.var_tmf2_dn17 = assign14860_e20863_d_n17;

        let (assign14870_e20885, assign14870_e20885_d_n0, assign14870_e20885_d_n2, assign14870_e20885_d_n6, assign14870_e20885_d_n7, assign14870_e20885_d_n10, assign14870_e20885_d_n11, assign14870_e20885_d_n12, assign14870_e20885_d_n17,) = {
    if (((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 == 0.0)) && (locals.var_guard448 == 0.0)) {
        let assign14870_e20880: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14870_e20882: f64 = (assign14870_e20880 + locals.var_tmf2);
        let assign14870_e20883: f64 = (assign14870_e20882).sqrt();
        (assign14870_e20883, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14870_e20883)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14870_e20883)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14870_e20883)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14870_e20883)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14870_e20883)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign14870_e20883)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign14870_e20883)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign14870_e20883)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign14870_e20885;
        locals.var_tmf2_dn0 = assign14870_e20885_d_n0;
        locals.var_tmf2_dn2 = assign14870_e20885_d_n2;
        locals.var_tmf2_dn6 = assign14870_e20885_d_n6;
        locals.var_tmf2_dn7 = assign14870_e20885_d_n7;
        locals.var_tmf2_dn10 = assign14870_e20885_d_n10;
        locals.var_tmf2_dn11 = assign14870_e20885_d_n11;
        locals.var_tmf2_dn12 = assign14870_e20885_d_n12;
        locals.var_tmf2_dn17 = assign14870_e20885_d_n17;

        let (assign14880_e20908, assign14880_e20908_d_n0, assign14880_e20908_d_n2, assign14880_e20908_d_n6, assign14880_e20908_d_n7, assign14880_e20908_d_n10, assign14880_e20908_d_n11, assign14880_e20908_d_n12, assign14880_e20908_d_n17,) = {
    if (((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard447 == 0.0)) && (locals.var_guard448 == 0.0)) {
        let assign14880_e20904: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14880_e20905: f64 = (0.5 * assign14880_e20904);
        let assign14880_e20906: f64 = (locals.var_ps0_inib - assign14880_e20905);
        (assign14880_e20906, (locals.var_ps0_inib_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_ps0_inib_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_ps0_inib_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_ps0_inib_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_ps0_inib_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_ps0_inib_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_ps0_inib_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_ps0_inib_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign14880_e20908;
        locals.var_ps0_ini_dn0 = assign14880_e20908_d_n0;
        locals.var_ps0_ini_dn2 = assign14880_e20908_d_n2;
        locals.var_ps0_ini_dn6 = assign14880_e20908_d_n6;
        locals.var_ps0_ini_dn7 = assign14880_e20908_d_n7;
        locals.var_ps0_ini_dn10 = assign14880_e20908_d_n10;
        locals.var_ps0_ini_dn11 = assign14880_e20908_d_n11;
        locals.var_ps0_ini_dn12 = assign14880_e20908_d_n12;
        locals.var_ps0_ini_dn17 = assign14880_e20908_d_n17;

        let (assign14890_e20923, assign14890_e20923_d_n0, assign14890_e20923_d_n2, assign14890_e20923_d_n6, assign14890_e20923_d_n7, assign14890_e20923_d_n10, assign14890_e20923_d_n11, assign14890_e20923_d_n12, assign14890_e20923_d_n17,) = {
    if (((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) {
        let assign14890_e20920: f64 = (5e-12 / 2.0);
        let assign14890_e20921: f64 = (locals.var_vbcs_cl + assign14890_e20920);
        (assign14890_e20921, locals.var_vbcs_cl_dn0, locals.var_vbcs_cl_dn2, locals.var_vbcs_cl_dn6, locals.var_vbcs_cl_dn7, locals.var_vbcs_cl_dn10, locals.var_vbcs_cl_dn11, locals.var_vbcs_cl_dn12, locals.var_vbcs_cl_dn17,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign14890_e20923;
        locals.var_tx_dn0 = assign14890_e20923_d_n0;
        locals.var_tx_dn2 = assign14890_e20923_d_n2;
        locals.var_tx_dn6 = assign14890_e20923_d_n6;
        locals.var_tx_dn7 = assign14890_e20923_d_n7;
        locals.var_tx_dn10 = assign14890_e20923_d_n10;
        locals.var_tx_dn11 = assign14890_e20923_d_n11;
        locals.var_tx_dn12 = assign14890_e20923_d_n12;
        locals.var_tx_dn17 = assign14890_e20923_d_n17;

        let assign14900_e20926: f64 = if locals.var_ps0_ini < locals.var_tx { 1.0 } else { 0.0 };
        locals.var_guard449 = assign14900_e20926;

        let (assign14910_e20939, assign14910_e20939_d_n0, assign14910_e20939_d_n2, assign14910_e20939_d_n6, assign14910_e20939_d_n7, assign14910_e20939_d_n10, assign14910_e20939_d_n11, assign14910_e20939_d_n12, assign14910_e20939_d_n17,) = {
    if ((((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) && (locals.var_guard446 == 0.0)) && (locals.var_guard449 != 0.0)) {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign14910_e20939;
        locals.var_ps0_ini_dn0 = assign14910_e20939_d_n0;
        locals.var_ps0_ini_dn2 = assign14910_e20939_d_n2;
        locals.var_ps0_ini_dn6 = assign14910_e20939_d_n6;
        locals.var_ps0_ini_dn7 = assign14910_e20939_d_n7;
        locals.var_ps0_ini_dn10 = assign14910_e20939_d_n10;
        locals.var_ps0_ini_dn11 = assign14910_e20939_d_n11;
        locals.var_ps0_ini_dn12 = assign14910_e20939_d_n12;
        locals.var_ps0_ini_dn17 = assign14910_e20939_d_n17;

        let (assign14920_e20947, assign14920_e20947_d_n0, assign14920_e20947_d_n2, assign14920_e20947_d_n6, assign14920_e20947_d_n7, assign14920_e20947_d_n10, assign14920_e20947_d_n11, assign14920_e20947_d_n12, assign14920_e20947_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    } else {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    }
};
        locals.var_ps0 = assign14920_e20947;
        locals.var_ps0_dn0 = assign14920_e20947_d_n0;
        locals.var_ps0_dn2 = assign14920_e20947_d_n2;
        locals.var_ps0_dn6 = assign14920_e20947_d_n6;
        locals.var_ps0_dn7 = assign14920_e20947_d_n7;
        locals.var_ps0_dn10 = assign14920_e20947_d_n10;
        locals.var_ps0_dn11 = assign14920_e20947_d_n11;
        locals.var_ps0_dn12 = assign14920_e20947_d_n12;
        locals.var_ps0_dn17 = assign14920_e20947_d_n17;

        let (assign14930_e20955, assign14930_e20955_d_n0, assign14930_e20955_d_n2, assign14930_e20955_d_n6, assign14930_e20955_d_n7, assign14930_e20955_d_n10, assign14930_e20955_d_n11, assign14930_e20955_d_n12, assign14930_e20955_d_n17,) = {
    if ((locals.var_guard113 == 0.0) && (locals.var_guard444 == 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_psl_lim, locals.var_psl_lim_dn0, locals.var_psl_lim_dn2, locals.var_psl_lim_dn6, locals.var_psl_lim_dn7, locals.var_psl_lim_dn10, locals.var_psl_lim_dn11, locals.var_psl_lim_dn12, locals.var_psl_lim_dn17,)
    }
};
        locals.var_psl_lim = assign14930_e20955;
        locals.var_psl_lim_dn0 = assign14930_e20955_d_n0;
        locals.var_psl_lim_dn2 = assign14930_e20955_d_n2;
        locals.var_psl_lim_dn6 = assign14930_e20955_d_n6;
        locals.var_psl_lim_dn7 = assign14930_e20955_d_n7;
        locals.var_psl_lim_dn10 = assign14930_e20955_d_n10;
        locals.var_psl_lim_dn11 = assign14930_e20955_d_n11;
        locals.var_psl_lim_dn12 = assign14930_e20955_d_n12;
        locals.var_psl_lim_dn17 = assign14930_e20955_d_n17;

        let assign14940_e20962: f64 = if ((p.p25 == 1.0) && (p.p26 == 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard450 = assign14940_e20962;

    }
}

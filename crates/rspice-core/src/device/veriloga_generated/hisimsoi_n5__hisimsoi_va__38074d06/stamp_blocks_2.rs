#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_32(
        locals: &mut StampLocals,
    ) {
        let (assign11150_e12845,) = {
    if (locals.var_guard111 != 0.0) {
        let (assign11150_e12843,) = {
            if (locals.var_flg_brk8 > 0.0) {
                (locals.var_flg_brk8,)
            } else {
                (locals.var_lp_s0,)
            }
        };
        (assign11150_e12843,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign11150_e12845;

        let assign11160_e12848: f64 = if locals.var_flg_conv == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard301 = assign11160_e12848;

        let (assign11170_e12854, assign11170_e12854_d_n0, assign11170_e12854_d_n2, assign11170_e12854_d_n6, assign11170_e12854_d_n7, assign11170_e12854_d_n10, assign11170_e12854_d_n11, assign11170_e12854_d_n12, assign11170_e12854_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard301 != 0.0)) {
        (locals.var_phi_s0_soi_ini, locals.var_phi_s0_soi_ini_dn0, locals.var_phi_s0_soi_ini_dn2, locals.var_phi_s0_soi_ini_dn6, locals.var_phi_s0_soi_ini_dn7, locals.var_phi_s0_soi_ini_dn10, locals.var_phi_s0_soi_ini_dn11, locals.var_phi_s0_soi_ini_dn12, locals.var_phi_s0_soi_ini_dn17,)
    } else {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    }
};
        locals.var_phi_s0_soi = assign11170_e12854;
        locals.var_phi_s0_soi_dn0 = assign11170_e12854_d_n0;
        locals.var_phi_s0_soi_dn2 = assign11170_e12854_d_n2;
        locals.var_phi_s0_soi_dn6 = assign11170_e12854_d_n6;
        locals.var_phi_s0_soi_dn7 = assign11170_e12854_d_n7;
        locals.var_phi_s0_soi_dn10 = assign11170_e12854_d_n10;
        locals.var_phi_s0_soi_dn11 = assign11170_e12854_d_n11;
        locals.var_phi_s0_soi_dn12 = assign11170_e12854_d_n12;
        locals.var_phi_s0_soi_dn17 = assign11170_e12854_d_n17;

        let (assign11180_e12860, assign11180_e12860_d_n0, assign11180_e12860_d_n2, assign11180_e12860_d_n6, assign11180_e12860_d_n7, assign11180_e12860_d_n10, assign11180_e12860_d_n11, assign11180_e12860_d_n12, assign11180_e12860_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard301 != 0.0)) {
        (locals.var_phi_b0_soi_ini, locals.var_phi_b0_soi_ini_dn0, locals.var_phi_b0_soi_ini_dn2, locals.var_phi_b0_soi_ini_dn6, locals.var_phi_b0_soi_ini_dn7, locals.var_phi_b0_soi_ini_dn10, locals.var_phi_b0_soi_ini_dn11, locals.var_phi_b0_soi_ini_dn12, locals.var_phi_b0_soi_ini_dn17,)
    } else {
        (locals.var_phi_b0_soi, locals.var_phi_b0_soi_dn0, locals.var_phi_b0_soi_dn2, locals.var_phi_b0_soi_dn6, locals.var_phi_b0_soi_dn7, locals.var_phi_b0_soi_dn10, locals.var_phi_b0_soi_dn11, locals.var_phi_b0_soi_dn12, locals.var_phi_b0_soi_dn17,)
    }
};
        locals.var_phi_b0_soi = assign11180_e12860;
        locals.var_phi_b0_soi_dn0 = assign11180_e12860_d_n0;
        locals.var_phi_b0_soi_dn2 = assign11180_e12860_d_n2;
        locals.var_phi_b0_soi_dn6 = assign11180_e12860_d_n6;
        locals.var_phi_b0_soi_dn7 = assign11180_e12860_d_n7;
        locals.var_phi_b0_soi_dn10 = assign11180_e12860_d_n10;
        locals.var_phi_b0_soi_dn11 = assign11180_e12860_d_n11;
        locals.var_phi_b0_soi_dn12 = assign11180_e12860_d_n12;
        locals.var_phi_b0_soi_dn17 = assign11180_e12860_d_n17;

        let (assign11190_e12866, assign11190_e12866_d_n0, assign11190_e12866_d_n2, assign11190_e12866_d_n6, assign11190_e12866_d_n7, assign11190_e12866_d_n10, assign11190_e12866_d_n11, assign11190_e12866_d_n12, assign11190_e12866_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard301 != 0.0)) {
        (locals.var_phi_s0_bulk_ini, locals.var_phi_s0_bulk_ini_dn0, locals.var_phi_s0_bulk_ini_dn2, locals.var_phi_s0_bulk_ini_dn6, locals.var_phi_s0_bulk_ini_dn7, locals.var_phi_s0_bulk_ini_dn10, locals.var_phi_s0_bulk_ini_dn11, locals.var_phi_s0_bulk_ini_dn12, locals.var_phi_s0_bulk_ini_dn17,)
    } else {
        (locals.var_phi_s0_bulk, locals.var_phi_s0_bulk_dn0, locals.var_phi_s0_bulk_dn2, locals.var_phi_s0_bulk_dn6, locals.var_phi_s0_bulk_dn7, locals.var_phi_s0_bulk_dn10, locals.var_phi_s0_bulk_dn11, locals.var_phi_s0_bulk_dn12, locals.var_phi_s0_bulk_dn17,)
    }
};
        locals.var_phi_s0_bulk = assign11190_e12866;
        locals.var_phi_s0_bulk_dn0 = assign11190_e12866_d_n0;
        locals.var_phi_s0_bulk_dn2 = assign11190_e12866_d_n2;
        locals.var_phi_s0_bulk_dn6 = assign11190_e12866_d_n6;
        locals.var_phi_s0_bulk_dn7 = assign11190_e12866_d_n7;
        locals.var_phi_s0_bulk_dn10 = assign11190_e12866_d_n10;
        locals.var_phi_s0_bulk_dn11 = assign11190_e12866_d_n11;
        locals.var_phi_s0_bulk_dn12 = assign11190_e12866_d_n12;
        locals.var_phi_s0_bulk_dn17 = assign11190_e12866_d_n17;

        let (assign11200_e12870, assign11200_e12870_d_n0, assign11200_e12870_d_n2, assign11200_e12870_d_n6, assign11200_e12870_d_n7, assign11200_e12870_d_n10, assign11200_e12870_d_n11, assign11200_e12870_d_n12, assign11200_e12870_d_n17,) = {
    if (locals.var_guard111 != 0.0) {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    } else {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    }
};
        locals.var_ps0 = assign11200_e12870;
        locals.var_ps0_dn0 = assign11200_e12870_d_n0;
        locals.var_ps0_dn2 = assign11200_e12870_d_n2;
        locals.var_ps0_dn6 = assign11200_e12870_d_n6;
        locals.var_ps0_dn7 = assign11200_e12870_d_n7;
        locals.var_ps0_dn10 = assign11200_e12870_d_n10;
        locals.var_ps0_dn11 = assign11200_e12870_d_n11;
        locals.var_ps0_dn12 = assign11200_e12870_d_n12;
        locals.var_ps0_dn17 = assign11200_e12870_d_n17;

        let (assign11210_e12875, assign11210_e12875_d_n0, assign11210_e12875_d_n2, assign11210_e12875_d_n6, assign11210_e12875_d_n7, assign11210_e12875_d_n10, assign11210_e12875_d_n11, assign11210_e12875_d_n12, assign11210_e12875_d_n17,) = {
    if (locals.var_guard111 != 0.0) {
        let assign11210_e12873: f64 = (-locals.var_q_n0);
        (assign11210_e12873, (-locals.var_q_n0_dn0), (-locals.var_q_n0_dn2), (-locals.var_q_n0_dn6), (-locals.var_q_n0_dn7), (-locals.var_q_n0_dn10), (-locals.var_q_n0_dn11), (-locals.var_q_n0_dn12), (-locals.var_q_n0_dn17),)
    } else {
        (locals.var_qn0, locals.var_qn0_dn0, locals.var_qn0_dn2, locals.var_qn0_dn6, locals.var_qn0_dn7, locals.var_qn0_dn10, locals.var_qn0_dn11, locals.var_qn0_dn12, locals.var_qn0_dn17,)
    }
};
        locals.var_qn0 = assign11210_e12875;
        locals.var_qn0_dn0 = assign11210_e12875_d_n0;
        locals.var_qn0_dn2 = assign11210_e12875_d_n2;
        locals.var_qn0_dn6 = assign11210_e12875_d_n6;
        locals.var_qn0_dn7 = assign11210_e12875_d_n7;
        locals.var_qn0_dn10 = assign11210_e12875_d_n10;
        locals.var_qn0_dn11 = assign11210_e12875_d_n11;
        locals.var_qn0_dn12 = assign11210_e12875_d_n12;
        locals.var_qn0_dn17 = assign11210_e12875_d_n17;

        let assign11220_e12878: f64 = if locals.var_qn0 <= 1e-50 { 1.0 } else { 0.0 };
        locals.var_guard302 = assign11220_e12878;

        let (assign11230_e12884, assign11230_e12884_d_n0, assign11230_e12884_d_n2, assign11230_e12884_d_n6, assign11230_e12884_d_n7, assign11230_e12884_d_n10, assign11230_e12884_d_n11, assign11230_e12884_d_n12, assign11230_e12884_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard302 != 0.0)) {
        (1e-50, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qn0, locals.var_qn0_dn0, locals.var_qn0_dn2, locals.var_qn0_dn6, locals.var_qn0_dn7, locals.var_qn0_dn10, locals.var_qn0_dn11, locals.var_qn0_dn12, locals.var_qn0_dn17,)
    }
};
        locals.var_qn0 = assign11230_e12884;
        locals.var_qn0_dn0 = assign11230_e12884_d_n0;
        locals.var_qn0_dn2 = assign11230_e12884_d_n2;
        locals.var_qn0_dn6 = assign11230_e12884_d_n6;
        locals.var_qn0_dn7 = assign11230_e12884_d_n7;
        locals.var_qn0_dn10 = assign11230_e12884_d_n10;
        locals.var_qn0_dn11 = assign11230_e12884_d_n11;
        locals.var_qn0_dn12 = assign11230_e12884_d_n12;
        locals.var_qn0_dn17 = assign11230_e12884_d_n17;

        let (assign11250_e12894, assign11250_e12894_d_n0, assign11250_e12894_d_n2, assign11250_e12894_d_n6, assign11250_e12894_d_n7, assign11250_e12894_d_n10, assign11250_e12894_d_n11, assign11250_e12894_d_n12, assign11250_e12894_d_n17,) = {
    if (locals.var_guard111 != 0.0) {
        let assign11250_e12892: f64 = (locals.var_qn0 * locals.var_c_fox_inv);
        (assign11250_e12892, ((locals.var_qn0_dn0 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn0)), ((locals.var_qn0_dn2 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn2)), ((locals.var_qn0_dn6 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn6)), ((locals.var_qn0_dn7 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn7)), ((locals.var_qn0_dn10 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn10)), ((locals.var_qn0_dn11 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn11)), ((locals.var_qn0_dn12 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn12)), ((locals.var_qn0_dn17 * locals.var_c_fox_inv) + (locals.var_qn0 * locals.var_c_fox_inv_dn17)),)
    } else {
        (locals.var_vgvt, locals.var_vgvt_dn0, locals.var_vgvt_dn2, locals.var_vgvt_dn6, locals.var_vgvt_dn7, locals.var_vgvt_dn10, locals.var_vgvt_dn11, locals.var_vgvt_dn12, locals.var_vgvt_dn17,)
    }
};
        locals.var_vgvt = assign11250_e12894;
        locals.var_vgvt_dn0 = assign11250_e12894_d_n0;
        locals.var_vgvt_dn2 = assign11250_e12894_d_n2;
        locals.var_vgvt_dn6 = assign11250_e12894_d_n6;
        locals.var_vgvt_dn7 = assign11250_e12894_d_n7;
        locals.var_vgvt_dn10 = assign11250_e12894_d_n10;
        locals.var_vgvt_dn11 = assign11250_e12894_d_n11;
        locals.var_vgvt_dn12 = assign11250_e12894_d_n12;
        locals.var_vgvt_dn17 = assign11250_e12894_d_n17;

        let assign11260_e12899: f64 = if ((locals.var_phi_s0_soi <= 0.0) && (locals.var_flg_skipacc != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard303 = assign11260_e12899;

        let (assign11280_e12914, assign11280_e12914_d_n0, assign11280_e12914_d_n2, assign11280_e12914_d_n6, assign11280_e12914_d_n7, assign11280_e12914_d_n10, assign11280_e12914_d_n11, assign11280_e12914_d_n12, assign11280_e12914_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 != 0.0)) {
        let assign11280_e12910: f64 = (-locals.var_weffcv_nf);
        let assign11280_e12912: f64 = (assign11280_e12910 * locals.var_leff_cv);
        (assign11280_e12912, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign11280_e12914;
        locals.var_t0_dn0 = assign11280_e12914_d_n0;
        locals.var_t0_dn2 = assign11280_e12914_d_n2;
        locals.var_t0_dn6 = assign11280_e12914_d_n6;
        locals.var_t0_dn7 = assign11280_e12914_d_n7;
        locals.var_t0_dn10 = assign11280_e12914_d_n10;
        locals.var_t0_dn11 = assign11280_e12914_d_n11;
        locals.var_t0_dn12 = assign11280_e12914_d_n12;
        locals.var_t0_dn17 = assign11280_e12914_d_n17;

        let (assign11290_e12920, assign11290_e12920_d_n0, assign11290_e12920_d_n2, assign11290_e12920_d_n6, assign11290_e12920_d_n7, assign11290_e12920_d_n10, assign11290_e12920_d_n11, assign11290_e12920_d_n12, assign11290_e12920_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 != 0.0)) {
        (locals.var_q_s0_dep_ini, locals.var_q_s0_dep_ini_dn0, locals.var_q_s0_dep_ini_dn2, locals.var_q_s0_dep_ini_dn6, locals.var_q_s0_dep_ini_dn7, locals.var_q_s0_dep_ini_dn10, locals.var_q_s0_dep_ini_dn11, locals.var_q_s0_dep_ini_dn12, locals.var_q_s0_dep_ini_dn17,)
    } else {
        (locals.var_q_sl_dep, locals.var_q_sl_dep_dn0, locals.var_q_sl_dep_dn2, locals.var_q_sl_dep_dn6, locals.var_q_sl_dep_dn7, locals.var_q_sl_dep_dn10, locals.var_q_sl_dep_dn11, locals.var_q_sl_dep_dn12, locals.var_q_sl_dep_dn17,)
    }
};
        locals.var_q_sl_dep = assign11290_e12920;
        locals.var_q_sl_dep_dn0 = assign11290_e12920_d_n0;
        locals.var_q_sl_dep_dn2 = assign11290_e12920_d_n2;
        locals.var_q_sl_dep_dn6 = assign11290_e12920_d_n6;
        locals.var_q_sl_dep_dn7 = assign11290_e12920_d_n7;
        locals.var_q_sl_dep_dn10 = assign11290_e12920_d_n10;
        locals.var_q_sl_dep_dn11 = assign11290_e12920_d_n11;
        locals.var_q_sl_dep_dn12 = assign11290_e12920_d_n12;
        locals.var_q_sl_dep_dn17 = assign11290_e12920_d_n17;

        let (assign11300_e12926, assign11300_e12926_d_n0, assign11300_e12926_d_n2, assign11300_e12926_d_n6, assign11300_e12926_d_n7, assign11300_e12926_d_n10, assign11300_e12926_d_n11, assign11300_e12926_d_n12, assign11300_e12926_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 != 0.0)) {
        (locals.var_q_b0_dep, locals.var_q_b0_dep_dn0, locals.var_q_b0_dep_dn2, locals.var_q_b0_dep_dn6, locals.var_q_b0_dep_dn7, locals.var_q_b0_dep_dn10, locals.var_q_b0_dep_dn11, locals.var_q_b0_dep_dn12, locals.var_q_b0_dep_dn17,)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn12, locals.var_q_bl_dep_dn17,)
    }
};
        locals.var_q_bl_dep = assign11300_e12926;
        locals.var_q_bl_dep_dn0 = assign11300_e12926_d_n0;
        locals.var_q_bl_dep_dn2 = assign11300_e12926_d_n2;
        locals.var_q_bl_dep_dn6 = assign11300_e12926_d_n6;
        locals.var_q_bl_dep_dn7 = assign11300_e12926_d_n7;
        locals.var_q_bl_dep_dn10 = assign11300_e12926_d_n10;
        locals.var_q_bl_dep_dn11 = assign11300_e12926_d_n11;
        locals.var_q_bl_dep_dn12 = assign11300_e12926_d_n12;
        locals.var_q_bl_dep_dn17 = assign11300_e12926_d_n17;

        let (assign11310_e12934, assign11310_e12934_d_n0, assign11310_e12934_d_n2, assign11310_e12934_d_n6, assign11310_e12934_d_n7, assign11310_e12934_d_n10, assign11310_e12934_d_n11, assign11310_e12934_d_n12, assign11310_e12934_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 != 0.0)) {
        let assign11310_e12932: f64 = (locals.var_q_sl_dep + locals.var_q_bl_dep);
        (assign11310_e12932, (locals.var_q_sl_dep_dn0 + locals.var_q_bl_dep_dn0), (locals.var_q_sl_dep_dn2 + locals.var_q_bl_dep_dn2), (locals.var_q_sl_dep_dn6 + locals.var_q_bl_dep_dn6), (locals.var_q_sl_dep_dn7 + locals.var_q_bl_dep_dn7), (locals.var_q_sl_dep_dn10 + locals.var_q_bl_dep_dn10), (locals.var_q_sl_dep_dn11 + locals.var_q_bl_dep_dn11), (locals.var_q_sl_dep_dn12 + locals.var_q_bl_dep_dn12), (locals.var_q_sl_dep_dn17 + locals.var_q_bl_dep_dn17),)
    } else {
        (locals.var_q_depl, locals.var_q_depl_dn0, locals.var_q_depl_dn2, locals.var_q_depl_dn6, locals.var_q_depl_dn7, locals.var_q_depl_dn10, locals.var_q_depl_dn11, locals.var_q_depl_dn12, locals.var_q_depl_dn17,)
    }
};
        locals.var_q_depl = assign11310_e12934;
        locals.var_q_depl_dn0 = assign11310_e12934_d_n0;
        locals.var_q_depl_dn2 = assign11310_e12934_d_n2;
        locals.var_q_depl_dn6 = assign11310_e12934_d_n6;
        locals.var_q_depl_dn7 = assign11310_e12934_d_n7;
        locals.var_q_depl_dn10 = assign11310_e12934_d_n10;
        locals.var_q_depl_dn11 = assign11310_e12934_d_n11;
        locals.var_q_depl_dn12 = assign11310_e12934_d_n12;
        locals.var_q_depl_dn17 = assign11310_e12934_d_n17;

        let (assign11320_e12945, assign11320_e12945_d_n0, assign11320_e12945_d_n2, assign11320_e12945_d_n6, assign11320_e12945_d_n7, assign11320_e12945_d_n10, assign11320_e12945_d_n11, assign11320_e12945_d_n12, assign11320_e12945_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 != 0.0)) {
        let assign11320_e12939: f64 = (-0.5);
        let assign11320_e12942: f64 = (locals.var_q_depl + locals.var_q_dep0);
        let assign11320_e12943: f64 = (assign11320_e12939 * assign11320_e12942);
        (assign11320_e12943, (assign11320_e12939 * (locals.var_q_depl_dn0 + locals.var_q_dep0_dn0)), (assign11320_e12939 * (locals.var_q_depl_dn2 + locals.var_q_dep0_dn2)), (assign11320_e12939 * (locals.var_q_depl_dn6 + locals.var_q_dep0_dn6)), (assign11320_e12939 * (locals.var_q_depl_dn7 + locals.var_q_dep0_dn7)), (assign11320_e12939 * (locals.var_q_depl_dn10 + locals.var_q_dep0_dn10)), (assign11320_e12939 * (locals.var_q_depl_dn11 + locals.var_q_dep0_dn11)), (assign11320_e12939 * (locals.var_q_depl_dn12 + locals.var_q_dep0_dn12)), (assign11320_e12939 * (locals.var_q_depl_dn17 + locals.var_q_dep0_dn17)),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign11320_e12945;
        locals.var_qbu_dn0 = assign11320_e12945_d_n0;
        locals.var_qbu_dn2 = assign11320_e12945_d_n2;
        locals.var_qbu_dn6 = assign11320_e12945_d_n6;
        locals.var_qbu_dn7 = assign11320_e12945_d_n7;
        locals.var_qbu_dn10 = assign11320_e12945_d_n10;
        locals.var_qbu_dn11 = assign11320_e12945_d_n11;
        locals.var_qbu_dn12 = assign11320_e12945_d_n12;
        locals.var_qbu_dn17 = assign11320_e12945_d_n17;

        let (assign11330_e12953, assign11330_e12953_d_n0, assign11330_e12953_d_n2, assign11330_e12953_d_n6, assign11330_e12953_d_n7, assign11330_e12953_d_n10, assign11330_e12953_d_n11, assign11330_e12953_d_n12, assign11330_e12953_d_n13, assign11330_e12953_d_n15, assign11330_e12953_d_n16, assign11330_e12953_d_n17, assign11330_e12953_d_n18,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 != 0.0)) {
        let assign11330_e12951: f64 = (locals.var_t0 * locals.var_qbu);
        (assign11330_e12951, ((locals.var_t0_dn0 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn0)), ((locals.var_t0_dn2 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn2)), ((locals.var_t0_dn6 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn6)), ((locals.var_t0_dn7 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn7)), ((locals.var_t0_dn10 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn10)), ((locals.var_t0_dn11 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn11)), ((locals.var_t0_dn12 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn12)), 0.0, 0.0, 0.0, ((locals.var_t0_dn17 * locals.var_qbu) + (locals.var_t0 * locals.var_qbu_dn17)), 0.0,)
    } else {
        (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn12, locals.var_qb_dn13, locals.var_qb_dn15, locals.var_qb_dn16, locals.var_qb_dn17, locals.var_qb_dn18,)
    }
};
        locals.var_qb = assign11330_e12953;
        locals.var_qb_dn0 = assign11330_e12953_d_n0;
        locals.var_qb_dn2 = assign11330_e12953_d_n2;
        locals.var_qb_dn6 = assign11330_e12953_d_n6;
        locals.var_qb_dn7 = assign11330_e12953_d_n7;
        locals.var_qb_dn10 = assign11330_e12953_d_n10;
        locals.var_qb_dn11 = assign11330_e12953_d_n11;
        locals.var_qb_dn12 = assign11330_e12953_d_n12;
        locals.var_qb_dn13 = assign11330_e12953_d_n13;
        locals.var_qb_dn15 = assign11330_e12953_d_n15;
        locals.var_qb_dn16 = assign11330_e12953_d_n16;
        locals.var_qb_dn17 = assign11330_e12953_d_n17;
        locals.var_qb_dn18 = assign11330_e12953_d_n18;

        let (assign11340_e12961, assign11340_e12961_d_n0, assign11340_e12961_d_n2, assign11340_e12961_d_n6, assign11340_e12961_d_n7, assign11340_e12961_d_n10, assign11340_e12961_d_n11, assign11340_e12961_d_n12, assign11340_e12961_d_n13, assign11340_e12961_d_n15, assign11340_e12961_d_n16, assign11340_e12961_d_n17, assign11340_e12961_d_n18,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 != 0.0)) {
        let assign11340_e12959: f64 = (locals.var_qb * 0.5);
        (assign11340_e12959, (locals.var_qb_dn0 * 0.5), (locals.var_qb_dn2 * 0.5), (locals.var_qb_dn6 * 0.5), (locals.var_qb_dn7 * 0.5), (locals.var_qb_dn10 * 0.5), (locals.var_qb_dn11 * 0.5), (locals.var_qb_dn12 * 0.5), (locals.var_qb_dn13 * 0.5), (locals.var_qb_dn15 * 0.5), (locals.var_qb_dn16 * 0.5), (locals.var_qb_dn17 * 0.5), (locals.var_qb_dn18 * 0.5),)
    } else {
        (locals.var_qd_fb, locals.var_qd_fb_dn0, locals.var_qd_fb_dn2, locals.var_qd_fb_dn6, locals.var_qd_fb_dn7, locals.var_qd_fb_dn10, locals.var_qd_fb_dn11, locals.var_qd_fb_dn12, locals.var_qd_fb_dn13, locals.var_qd_fb_dn15, locals.var_qd_fb_dn16, locals.var_qd_fb_dn17, locals.var_qd_fb_dn18,)
    }
};
        locals.var_qd_fb = assign11340_e12961;
        locals.var_qd_fb_dn0 = assign11340_e12961_d_n0;
        locals.var_qd_fb_dn2 = assign11340_e12961_d_n2;
        locals.var_qd_fb_dn6 = assign11340_e12961_d_n6;
        locals.var_qd_fb_dn7 = assign11340_e12961_d_n7;
        locals.var_qd_fb_dn10 = assign11340_e12961_d_n10;
        locals.var_qd_fb_dn11 = assign11340_e12961_d_n11;
        locals.var_qd_fb_dn12 = assign11340_e12961_d_n12;
        locals.var_qd_fb_dn13 = assign11340_e12961_d_n13;
        locals.var_qd_fb_dn15 = assign11340_e12961_d_n15;
        locals.var_qd_fb_dn16 = assign11340_e12961_d_n16;
        locals.var_qd_fb_dn17 = assign11340_e12961_d_n17;
        locals.var_qd_fb_dn18 = assign11340_e12961_d_n18;

        let (assign11350_e12971, assign11350_e12971_d_n0, assign11350_e12971_d_n2, assign11350_e12971_d_n6, assign11350_e12971_d_n7, assign11350_e12971_d_n10, assign11350_e12971_d_n11, assign11350_e12971_d_n12, assign11350_e12971_d_n13, assign11350_e12971_d_n15, assign11350_e12971_d_n16, assign11350_e12971_d_n17, assign11350_e12971_d_n18,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 != 0.0)) {
        let assign11350_e12968: f64 = (1.0 - 0.5);
        let assign11350_e12969: f64 = (locals.var_qb * assign11350_e12968);
        (assign11350_e12969, (locals.var_qb_dn0 * assign11350_e12968), (locals.var_qb_dn2 * assign11350_e12968), (locals.var_qb_dn6 * assign11350_e12968), (locals.var_qb_dn7 * assign11350_e12968), (locals.var_qb_dn10 * assign11350_e12968), (locals.var_qb_dn11 * assign11350_e12968), (locals.var_qb_dn12 * assign11350_e12968), (locals.var_qb_dn13 * assign11350_e12968), (locals.var_qb_dn15 * assign11350_e12968), (locals.var_qb_dn16 * assign11350_e12968), (locals.var_qb_dn17 * assign11350_e12968), (locals.var_qb_dn18 * assign11350_e12968),)
    } else {
        (locals.var_qs_fb, locals.var_qs_fb_dn0, locals.var_qs_fb_dn2, locals.var_qs_fb_dn6, locals.var_qs_fb_dn7, locals.var_qs_fb_dn10, locals.var_qs_fb_dn11, locals.var_qs_fb_dn12, locals.var_qs_fb_dn13, locals.var_qs_fb_dn15, locals.var_qs_fb_dn16, locals.var_qs_fb_dn17, locals.var_qs_fb_dn18,)
    }
};
        locals.var_qs_fb = assign11350_e12971;
        locals.var_qs_fb_dn0 = assign11350_e12971_d_n0;
        locals.var_qs_fb_dn2 = assign11350_e12971_d_n2;
        locals.var_qs_fb_dn6 = assign11350_e12971_d_n6;
        locals.var_qs_fb_dn7 = assign11350_e12971_d_n7;
        locals.var_qs_fb_dn10 = assign11350_e12971_d_n10;
        locals.var_qs_fb_dn11 = assign11350_e12971_d_n11;
        locals.var_qs_fb_dn12 = assign11350_e12971_d_n12;
        locals.var_qs_fb_dn13 = assign11350_e12971_d_n13;
        locals.var_qs_fb_dn15 = assign11350_e12971_d_n15;
        locals.var_qs_fb_dn16 = assign11350_e12971_d_n16;
        locals.var_qs_fb_dn17 = assign11350_e12971_d_n17;
        locals.var_qs_fb_dn18 = assign11350_e12971_d_n18;

        let (assign11360_e12977, assign11360_e12977_d_n0, assign11360_e12977_d_n2, assign11360_e12977_d_n6, assign11360_e12977_d_n7, assign11360_e12977_d_n10, assign11360_e12977_d_n11, assign11360_e12977_d_n12, assign11360_e12977_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qi, locals.var_qi_dn0, locals.var_qi_dn2, locals.var_qi_dn6, locals.var_qi_dn7, locals.var_qi_dn10, locals.var_qi_dn11, locals.var_qi_dn12, locals.var_qi_dn17,)
    }
};
        locals.var_qi = assign11360_e12977;
        locals.var_qi_dn0 = assign11360_e12977_d_n0;
        locals.var_qi_dn2 = assign11360_e12977_d_n2;
        locals.var_qi_dn6 = assign11360_e12977_d_n6;
        locals.var_qi_dn7 = assign11360_e12977_d_n7;
        locals.var_qi_dn10 = assign11360_e12977_d_n10;
        locals.var_qi_dn11 = assign11360_e12977_d_n11;
        locals.var_qi_dn12 = assign11360_e12977_d_n12;
        locals.var_qi_dn17 = assign11360_e12977_d_n17;

        let (assign11370_e12987, assign11370_e12987_d_n0, assign11370_e12987_d_n2, assign11370_e12987_d_n6, assign11370_e12987_d_n7, assign11370_e12987_d_n10, assign11370_e12987_d_n11, assign11370_e12987_d_n12, assign11370_e12987_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 != 0.0)) {
        let assign11370_e12983: f64 = (locals.var_q_s0_bulk * locals.var_leff_cv);
        let assign11370_e12985: f64 = (assign11370_e12983 * locals.var_weffcv_nf);
        (assign11370_e12985, ((locals.var_q_s0_bulk_dn0 * locals.var_leff_cv) * locals.var_weffcv_nf), ((locals.var_q_s0_bulk_dn2 * locals.var_leff_cv) * locals.var_weffcv_nf), ((locals.var_q_s0_bulk_dn6 * locals.var_leff_cv) * locals.var_weffcv_nf), ((locals.var_q_s0_bulk_dn7 * locals.var_leff_cv) * locals.var_weffcv_nf), ((locals.var_q_s0_bulk_dn10 * locals.var_leff_cv) * locals.var_weffcv_nf), ((locals.var_q_s0_bulk_dn11 * locals.var_leff_cv) * locals.var_weffcv_nf), ((locals.var_q_s0_bulk_dn12 * locals.var_leff_cv) * locals.var_weffcv_nf), ((locals.var_q_s0_bulk_dn17 * locals.var_leff_cv) * locals.var_weffcv_nf),)
    } else {
        (locals.var_qsub, locals.var_qsub_dn0, locals.var_qsub_dn2, locals.var_qsub_dn6, locals.var_qsub_dn7, locals.var_qsub_dn10, locals.var_qsub_dn11, locals.var_qsub_dn12, locals.var_qsub_dn17,)
    }
};
        locals.var_qsub = assign11370_e12987;
        locals.var_qsub_dn0 = assign11370_e12987_d_n0;
        locals.var_qsub_dn2 = assign11370_e12987_d_n2;
        locals.var_qsub_dn6 = assign11370_e12987_d_n6;
        locals.var_qsub_dn7 = assign11370_e12987_d_n7;
        locals.var_qsub_dn10 = assign11370_e12987_d_n10;
        locals.var_qsub_dn11 = assign11370_e12987_d_n11;
        locals.var_qsub_dn12 = assign11370_e12987_d_n12;
        locals.var_qsub_dn17 = assign11370_e12987_d_n17;

        let (assign11380_e12993, assign11380_e12993_d_n0, assign11380_e12993_d_n2, assign11380_e12993_d_n6, assign11380_e12993_d_n7, assign11380_e12993_d_n10, assign11380_e12993_d_n11, assign11380_e12993_d_n12, assign11380_e12993_d_n13, assign11380_e12993_d_n15, assign11380_e12993_d_n16, assign11380_e12993_d_n17, assign11380_e12993_d_n18,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18,)
    }
};
        locals.var_qd = assign11380_e12993;
        locals.var_qd_dn0 = assign11380_e12993_d_n0;
        locals.var_qd_dn2 = assign11380_e12993_d_n2;
        locals.var_qd_dn6 = assign11380_e12993_d_n6;
        locals.var_qd_dn7 = assign11380_e12993_d_n7;
        locals.var_qd_dn10 = assign11380_e12993_d_n10;
        locals.var_qd_dn11 = assign11380_e12993_d_n11;
        locals.var_qd_dn12 = assign11380_e12993_d_n12;
        locals.var_qd_dn13 = assign11380_e12993_d_n13;
        locals.var_qd_dn15 = assign11380_e12993_d_n15;
        locals.var_qd_dn16 = assign11380_e12993_d_n16;
        locals.var_qd_dn17 = assign11380_e12993_d_n17;
        locals.var_qd_dn18 = assign11380_e12993_d_n18;

        let (assign11390_e12999, assign11390_e12999_d_n0, assign11390_e12999_d_n2, assign11390_e12999_d_n6, assign11390_e12999_d_n7, assign11390_e12999_d_n10, assign11390_e12999_d_n11, assign11390_e12999_d_n12, assign11390_e12999_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17,)
    }
};
        locals.var_ids = assign11390_e12999;
        locals.var_ids_dn0 = assign11390_e12999_d_n0;
        locals.var_ids_dn2 = assign11390_e12999_d_n2;
        locals.var_ids_dn6 = assign11390_e12999_d_n6;
        locals.var_ids_dn7 = assign11390_e12999_d_n7;
        locals.var_ids_dn10 = assign11390_e12999_d_n10;
        locals.var_ids_dn11 = assign11390_e12999_d_n11;
        locals.var_ids_dn12 = assign11390_e12999_d_n12;
        locals.var_ids_dn17 = assign11390_e12999_d_n17;

        let (assign11400_e13005, assign11400_e13005_d_n0, assign11400_e13005_d_n2, assign11400_e13005_d_n6, assign11400_e13005_d_n7, assign11400_e13005_d_n10, assign11400_e13005_d_n11, assign11400_e13005_d_n12, assign11400_e13005_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgvt, locals.var_vgvt_dn0, locals.var_vgvt_dn2, locals.var_vgvt_dn6, locals.var_vgvt_dn7, locals.var_vgvt_dn10, locals.var_vgvt_dn11, locals.var_vgvt_dn12, locals.var_vgvt_dn17,)
    }
};
        locals.var_vgvt = assign11400_e13005;
        locals.var_vgvt_dn0 = assign11400_e13005_d_n0;
        locals.var_vgvt_dn2 = assign11400_e13005_d_n2;
        locals.var_vgvt_dn6 = assign11400_e13005_d_n6;
        locals.var_vgvt_dn7 = assign11400_e13005_d_n7;
        locals.var_vgvt_dn10 = assign11400_e13005_d_n10;
        locals.var_vgvt_dn11 = assign11400_e13005_d_n11;
        locals.var_vgvt_dn12 = assign11400_e13005_d_n12;
        locals.var_vgvt_dn17 = assign11400_e13005_d_n17;

        let (assign11410_e13011,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_noqi,)
    }
};
        locals.var_flg_noqi = assign11410_e13011;

        let (assign11420_e13017, assign11420_e13017_d_n0, assign11420_e13017_d_n2, assign11420_e13017_d_n6, assign11420_e13017_d_n7, assign11420_e13017_d_n10, assign11420_e13017_d_n11, assign11420_e13017_d_n12, assign11420_e13017_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 != 0.0)) {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
        locals.var_phi_sl_soi = assign11420_e13017;
        locals.var_phi_sl_soi_dn0 = assign11420_e13017_d_n0;
        locals.var_phi_sl_soi_dn2 = assign11420_e13017_d_n2;
        locals.var_phi_sl_soi_dn6 = assign11420_e13017_d_n6;
        locals.var_phi_sl_soi_dn7 = assign11420_e13017_d_n7;
        locals.var_phi_sl_soi_dn10 = assign11420_e13017_d_n10;
        locals.var_phi_sl_soi_dn11 = assign11420_e13017_d_n11;
        locals.var_phi_sl_soi_dn12 = assign11420_e13017_d_n12;
        locals.var_phi_sl_soi_dn17 = assign11420_e13017_d_n17;

        let (assign11430_e13023, assign11430_e13023_d_n0, assign11430_e13023_d_n2, assign11430_e13023_d_n6, assign11430_e13023_d_n7, assign11430_e13023_d_n10, assign11430_e13023_d_n11, assign11430_e13023_d_n12, assign11430_e13023_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 != 0.0)) {
        (locals.var_phi_b0_soi, locals.var_phi_b0_soi_dn0, locals.var_phi_b0_soi_dn2, locals.var_phi_b0_soi_dn6, locals.var_phi_b0_soi_dn7, locals.var_phi_b0_soi_dn10, locals.var_phi_b0_soi_dn11, locals.var_phi_b0_soi_dn12, locals.var_phi_b0_soi_dn17,)
    } else {
        (locals.var_phi_bl_soi, locals.var_phi_bl_soi_dn0, locals.var_phi_bl_soi_dn2, locals.var_phi_bl_soi_dn6, locals.var_phi_bl_soi_dn7, locals.var_phi_bl_soi_dn10, locals.var_phi_bl_soi_dn11, locals.var_phi_bl_soi_dn12, locals.var_phi_bl_soi_dn17,)
    }
};
        locals.var_phi_bl_soi = assign11430_e13023;
        locals.var_phi_bl_soi_dn0 = assign11430_e13023_d_n0;
        locals.var_phi_bl_soi_dn2 = assign11430_e13023_d_n2;
        locals.var_phi_bl_soi_dn6 = assign11430_e13023_d_n6;
        locals.var_phi_bl_soi_dn7 = assign11430_e13023_d_n7;
        locals.var_phi_bl_soi_dn10 = assign11430_e13023_d_n10;
        locals.var_phi_bl_soi_dn11 = assign11430_e13023_d_n11;
        locals.var_phi_bl_soi_dn12 = assign11430_e13023_d_n12;
        locals.var_phi_bl_soi_dn17 = assign11430_e13023_d_n17;

        let (assign11440_e13029, assign11440_e13029_d_n0, assign11440_e13029_d_n2, assign11440_e13029_d_n6, assign11440_e13029_d_n7, assign11440_e13029_d_n10, assign11440_e13029_d_n11, assign11440_e13029_d_n12, assign11440_e13029_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 != 0.0)) {
        (locals.var_phi_s0_bulk, locals.var_phi_s0_bulk_dn0, locals.var_phi_s0_bulk_dn2, locals.var_phi_s0_bulk_dn6, locals.var_phi_s0_bulk_dn7, locals.var_phi_s0_bulk_dn10, locals.var_phi_s0_bulk_dn11, locals.var_phi_s0_bulk_dn12, locals.var_phi_s0_bulk_dn17,)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
        locals.var_phi_sl_bulk = assign11440_e13029;
        locals.var_phi_sl_bulk_dn0 = assign11440_e13029_d_n0;
        locals.var_phi_sl_bulk_dn2 = assign11440_e13029_d_n2;
        locals.var_phi_sl_bulk_dn6 = assign11440_e13029_d_n6;
        locals.var_phi_sl_bulk_dn7 = assign11440_e13029_d_n7;
        locals.var_phi_sl_bulk_dn10 = assign11440_e13029_d_n10;
        locals.var_phi_sl_bulk_dn11 = assign11440_e13029_d_n11;
        locals.var_phi_sl_bulk_dn12 = assign11440_e13029_d_n12;
        locals.var_phi_sl_bulk_dn17 = assign11440_e13029_d_n17;

        let (assign11450_e13035, assign11450_e13035_d_n0, assign11450_e13035_d_n2, assign11450_e13035_d_n6, assign11450_e13035_d_n7, assign11450_e13035_d_n10, assign11450_e13035_d_n11, assign11450_e13035_d_n12, assign11450_e13035_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 != 0.0)) {
        (locals.var_q_s0_bulk, locals.var_q_s0_bulk_dn0, locals.var_q_s0_bulk_dn2, locals.var_q_s0_bulk_dn6, locals.var_q_s0_bulk_dn7, locals.var_q_s0_bulk_dn10, locals.var_q_s0_bulk_dn11, locals.var_q_s0_bulk_dn12, locals.var_q_s0_bulk_dn17,)
    } else {
        (locals.var_q_sl_bulk, locals.var_q_sl_bulk_dn0, locals.var_q_sl_bulk_dn2, locals.var_q_sl_bulk_dn6, locals.var_q_sl_bulk_dn7, locals.var_q_sl_bulk_dn10, locals.var_q_sl_bulk_dn11, locals.var_q_sl_bulk_dn12, locals.var_q_sl_bulk_dn17,)
    }
};
        locals.var_q_sl_bulk = assign11450_e13035;
        locals.var_q_sl_bulk_dn0 = assign11450_e13035_d_n0;
        locals.var_q_sl_bulk_dn2 = assign11450_e13035_d_n2;
        locals.var_q_sl_bulk_dn6 = assign11450_e13035_d_n6;
        locals.var_q_sl_bulk_dn7 = assign11450_e13035_d_n7;
        locals.var_q_sl_bulk_dn10 = assign11450_e13035_d_n10;
        locals.var_q_sl_bulk_dn11 = assign11450_e13035_d_n11;
        locals.var_q_sl_bulk_dn12 = assign11450_e13035_d_n12;
        locals.var_q_sl_bulk_dn17 = assign11450_e13035_d_n17;

        let (assign11460_e13041, assign11460_e13041_d_n0, assign11460_e13041_d_n2, assign11460_e13041_d_n6, assign11460_e13041_d_n7, assign11460_e13041_d_n10, assign11460_e13041_d_n11, assign11460_e13041_d_n12, assign11460_e13041_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 != 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    }
};
        locals.var_psl = assign11460_e13041;
        locals.var_psl_dn0 = assign11460_e13041_d_n0;
        locals.var_psl_dn2 = assign11460_e13041_d_n2;
        locals.var_psl_dn6 = assign11460_e13041_d_n6;
        locals.var_psl_dn7 = assign11460_e13041_d_n7;
        locals.var_psl_dn10 = assign11460_e13041_d_n10;
        locals.var_psl_dn11 = assign11460_e13041_d_n11;
        locals.var_psl_dn12 = assign11460_e13041_d_n12;
        locals.var_psl_dn17 = assign11460_e13041_d_n17;

        let (assign11470_e13047, assign11470_e13047_d_n0, assign11470_e13047_d_n2, assign11470_e13047_d_n6, assign11470_e13047_d_n7, assign11470_e13047_d_n10, assign11470_e13047_d_n11, assign11470_e13047_d_n12, assign11470_e13047_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 != 0.0)) {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17,)
    }
};
        locals.var_psdl = assign11470_e13047;
        locals.var_psdl_dn0 = assign11470_e13047_d_n0;
        locals.var_psdl_dn2 = assign11470_e13047_d_n2;
        locals.var_psdl_dn6 = assign11470_e13047_d_n6;
        locals.var_psdl_dn7 = assign11470_e13047_d_n7;
        locals.var_psdl_dn10 = assign11470_e13047_d_n10;
        locals.var_psdl_dn11 = assign11470_e13047_d_n11;
        locals.var_psdl_dn12 = assign11470_e13047_d_n12;
        locals.var_psdl_dn17 = assign11470_e13047_d_n17;

    }

    pub(super) fn stamp_transient_block_33(
        locals: &mut StampLocals,
    ) {
        let (assign11490_e13060, assign11490_e13060_d_n0, assign11490_e13060_d_n2, assign11490_e13060_d_n6, assign11490_e13060_d_n7, assign11490_e13060_d_n10, assign11490_e13060_d_n11, assign11490_e13060_d_n12, assign11490_e13060_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    } else {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn12, locals.var_vdsorg_dn17,)
    }
};
        locals.var_vdsorg = assign11490_e13060;
        locals.var_vdsorg_dn0 = assign11490_e13060_d_n0;
        locals.var_vdsorg_dn2 = assign11490_e13060_d_n2;
        locals.var_vdsorg_dn6 = assign11490_e13060_d_n6;
        locals.var_vdsorg_dn7 = assign11490_e13060_d_n7;
        locals.var_vdsorg_dn10 = assign11490_e13060_d_n10;
        locals.var_vdsorg_dn11 = assign11490_e13060_d_n11;
        locals.var_vdsorg_dn12 = assign11490_e13060_d_n12;
        locals.var_vdsorg_dn17 = assign11490_e13060_d_n17;

        let (assign11500_e13067, assign11500_e13067_d_n0, assign11500_e13067_d_n2, assign11500_e13067_d_n6, assign11500_e13067_d_n7, assign11500_e13067_d_n10, assign11500_e13067_d_n11, assign11500_e13067_d_n12, assign11500_e13067_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        (1e-50, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12, locals.var_t10_dn17,)
    }
};
        locals.var_t10 = assign11500_e13067;
        locals.var_t10_dn0 = assign11500_e13067_d_n0;
        locals.var_t10_dn2 = assign11500_e13067_d_n2;
        locals.var_t10_dn6 = assign11500_e13067_d_n6;
        locals.var_t10_dn7 = assign11500_e13067_d_n7;
        locals.var_t10_dn10 = assign11500_e13067_d_n10;
        locals.var_t10_dn11 = assign11500_e13067_d_n11;
        locals.var_t10_dn12 = assign11500_e13067_d_n12;
        locals.var_t10_dn17 = assign11500_e13067_d_n17;

        let (assign11510_e13078, assign11510_e13078_d_n0, assign11510_e13078_d_n2, assign11510_e13078_d_n6, assign11510_e13078_d_n7, assign11510_e13078_d_n10, assign11510_e13078_d_n11, assign11510_e13078_d_n12, assign11510_e13078_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign11510_e13075: f64 = (locals.var_c_fox * locals.var_c_fox);
        let assign11510_e13076: f64 = (locals.var_qnsub_esi / assign11510_e13075);
        (assign11510_e13076, (((locals.var_qnsub_esi_dn0 * assign11510_e13075) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn0 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn0)))) / (assign11510_e13075 * assign11510_e13075)), (((locals.var_qnsub_esi_dn2 * assign11510_e13075) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn2 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn2)))) / (assign11510_e13075 * assign11510_e13075)), (((locals.var_qnsub_esi_dn6 * assign11510_e13075) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn6 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn6)))) / (assign11510_e13075 * assign11510_e13075)), (((locals.var_qnsub_esi_dn7 * assign11510_e13075) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn7 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn7)))) / (assign11510_e13075 * assign11510_e13075)), (((locals.var_qnsub_esi_dn10 * assign11510_e13075) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn10 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn10)))) / (assign11510_e13075 * assign11510_e13075)), (((locals.var_qnsub_esi_dn11 * assign11510_e13075) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn11 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn11)))) / (assign11510_e13075 * assign11510_e13075)), (((locals.var_qnsub_esi_dn12 * assign11510_e13075) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn12 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn12)))) / (assign11510_e13075 * assign11510_e13075)), (((locals.var_qnsub_esi_dn17 * assign11510_e13075) - (locals.var_qnsub_esi * ((locals.var_c_fox_dn17 * locals.var_c_fox) + (locals.var_c_fox * locals.var_c_fox_dn17)))) / (assign11510_e13075 * assign11510_e13075)),)
    } else {
        (locals.var_t2__blk305, locals.var_t2__blk305_dn0, locals.var_t2__blk305_dn2, locals.var_t2__blk305_dn6, locals.var_t2__blk305_dn7, locals.var_t2__blk305_dn10, locals.var_t2__blk305_dn11, locals.var_t2__blk305_dn12, locals.var_t2__blk305_dn17,)
    }
};
        locals.var_t2__blk305 = assign11510_e13078;
        locals.var_t2__blk305_dn0 = assign11510_e13078_d_n0;
        locals.var_t2__blk305_dn2 = assign11510_e13078_d_n2;
        locals.var_t2__blk305_dn6 = assign11510_e13078_d_n6;
        locals.var_t2__blk305_dn7 = assign11510_e13078_d_n7;
        locals.var_t2__blk305_dn10 = assign11510_e13078_d_n10;
        locals.var_t2__blk305_dn11 = assign11510_e13078_d_n11;
        locals.var_t2__blk305_dn12 = assign11510_e13078_d_n12;
        locals.var_t2__blk305_dn17 = assign11510_e13078_d_n17;

        let (assign11520_e13093, assign11520_e13093_d_n0, assign11520_e13093_d_n2, assign11520_e13093_d_n6, assign11520_e13093_d_n7, assign11520_e13093_d_n10, assign11520_e13093_d_n11, assign11520_e13093_d_n12, assign11520_e13093_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign11520_e13086: f64 = (2.0 / locals.var_t2__blk305);
        let assign11520_e13089: f64 = (locals.var_vgp - locals.var_t10);
        let assign11520_e13090: f64 = (assign11520_e13086 * assign11520_e13089);
        let assign11520_e13091: f64 = (1.0 + assign11520_e13090);
        (assign11520_e13091, (((-((2.0 * locals.var_t2__blk305_dn0) / (locals.var_t2__blk305 * locals.var_t2__blk305))) * assign11520_e13089) + (assign11520_e13086 * (locals.var_vgp_dn0 - locals.var_t10_dn0))), (((-((2.0 * locals.var_t2__blk305_dn2) / (locals.var_t2__blk305 * locals.var_t2__blk305))) * assign11520_e13089) + (assign11520_e13086 * (locals.var_vgp_dn2 - locals.var_t10_dn2))), (((-((2.0 * locals.var_t2__blk305_dn6) / (locals.var_t2__blk305 * locals.var_t2__blk305))) * assign11520_e13089) + (assign11520_e13086 * (locals.var_vgp_dn6 - locals.var_t10_dn6))), (((-((2.0 * locals.var_t2__blk305_dn7) / (locals.var_t2__blk305 * locals.var_t2__blk305))) * assign11520_e13089) + (assign11520_e13086 * (locals.var_vgp_dn7 - locals.var_t10_dn7))), (((-((2.0 * locals.var_t2__blk305_dn10) / (locals.var_t2__blk305 * locals.var_t2__blk305))) * assign11520_e13089) + (assign11520_e13086 * (locals.var_vgp_dn10 - locals.var_t10_dn10))), (((-((2.0 * locals.var_t2__blk305_dn11) / (locals.var_t2__blk305 * locals.var_t2__blk305))) * assign11520_e13089) + (assign11520_e13086 * (locals.var_vgp_dn11 - locals.var_t10_dn11))), (((-((2.0 * locals.var_t2__blk305_dn12) / (locals.var_t2__blk305 * locals.var_t2__blk305))) * assign11520_e13089) + (assign11520_e13086 * (locals.var_vgp_dn12 - locals.var_t10_dn12))), (((-((2.0 * locals.var_t2__blk305_dn17) / (locals.var_t2__blk305 * locals.var_t2__blk305))) * assign11520_e13089) + (assign11520_e13086 * (locals.var_vgp_dn17 - locals.var_t10_dn17))),)
    } else {
        (locals.var_t4__blk307, locals.var_t4__blk307_dn0, locals.var_t4__blk307_dn2, locals.var_t4__blk307_dn6, locals.var_t4__blk307_dn7, locals.var_t4__blk307_dn10, locals.var_t4__blk307_dn11, locals.var_t4__blk307_dn12, locals.var_t4__blk307_dn17,)
    }
};
        locals.var_t4__blk307 = assign11520_e13093;
        locals.var_t4__blk307_dn0 = assign11520_e13093_d_n0;
        locals.var_t4__blk307_dn2 = assign11520_e13093_d_n2;
        locals.var_t4__blk307_dn6 = assign11520_e13093_d_n6;
        locals.var_t4__blk307_dn7 = assign11520_e13093_d_n7;
        locals.var_t4__blk307_dn10 = assign11520_e13093_d_n10;
        locals.var_t4__blk307_dn11 = assign11520_e13093_d_n11;
        locals.var_t4__blk307_dn12 = assign11520_e13093_d_n12;
        locals.var_t4__blk307_dn17 = assign11520_e13093_d_n17;

        let (assign11530_e13104, assign11530_e13104_d_n0, assign11530_e13104_d_n2, assign11530_e13104_d_n6, assign11530_e13104_d_n7, assign11530_e13104_d_n10, assign11530_e13104_d_n11, assign11530_e13104_d_n12, assign11530_e13104_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign11530_e13101: f64 = (2.0 / locals.var_t2__blk305);
        let assign11530_e13102: f64 = (1.0 + assign11530_e13101);
        (assign11530_e13102, (-((2.0 * locals.var_t2__blk305_dn0) / (locals.var_t2__blk305 * locals.var_t2__blk305))), (-((2.0 * locals.var_t2__blk305_dn2) / (locals.var_t2__blk305 * locals.var_t2__blk305))), (-((2.0 * locals.var_t2__blk305_dn6) / (locals.var_t2__blk305 * locals.var_t2__blk305))), (-((2.0 * locals.var_t2__blk305_dn7) / (locals.var_t2__blk305 * locals.var_t2__blk305))), (-((2.0 * locals.var_t2__blk305_dn10) / (locals.var_t2__blk305 * locals.var_t2__blk305))), (-((2.0 * locals.var_t2__blk305_dn11) / (locals.var_t2__blk305 * locals.var_t2__blk305))), (-((2.0 * locals.var_t2__blk305_dn12) / (locals.var_t2__blk305 * locals.var_t2__blk305))), (-((2.0 * locals.var_t2__blk305_dn17) / (locals.var_t2__blk305 * locals.var_t2__blk305))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign11530_e13104;
        locals.var_t5_dn0 = assign11530_e13104_d_n0;
        locals.var_t5_dn2 = assign11530_e13104_d_n2;
        locals.var_t5_dn6 = assign11530_e13104_d_n6;
        locals.var_t5_dn7 = assign11530_e13104_d_n7;
        locals.var_t5_dn10 = assign11530_e13104_d_n10;
        locals.var_t5_dn11 = assign11530_e13104_d_n11;
        locals.var_t5_dn12 = assign11530_e13104_d_n12;
        locals.var_t5_dn17 = assign11530_e13104_d_n17;

        let assign11540_e13108: f64 = locals.var_t5;
        let assign11540_e13113: f64 = if ((locals.var_t4__blk307 < assign11540_e13108) && (locals.var_t5 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard311 = assign11540_e13113;

        let (assign11550_e13126, assign11550_e13126_d_n0, assign11550_e13126_d_n2, assign11550_e13126_d_n6, assign11550_e13126_d_n7, assign11550_e13126_d_n10, assign11550_e13126_d_n11, assign11550_e13126_d_n12, assign11550_e13126_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign11550_e13122: f64 = locals.var_t5;
        let assign11550_e13124: f64 = (assign11550_e13122 - locals.var_t4__blk307);
        (assign11550_e13124, (locals.var_t5_dn0 - locals.var_t4__blk307_dn0), (locals.var_t5_dn2 - locals.var_t4__blk307_dn2), (locals.var_t5_dn6 - locals.var_t4__blk307_dn6), (locals.var_t5_dn7 - locals.var_t4__blk307_dn7), (locals.var_t5_dn10 - locals.var_t4__blk307_dn10), (locals.var_t5_dn11 - locals.var_t4__blk307_dn11), (locals.var_t5_dn12 - locals.var_t4__blk307_dn12), (locals.var_t5_dn17 - locals.var_t4__blk307_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign11550_e13126;
        locals.var_tmf1_dn0 = assign11550_e13126_d_n0;
        locals.var_tmf1_dn2 = assign11550_e13126_d_n2;
        locals.var_tmf1_dn6 = assign11550_e13126_d_n6;
        locals.var_tmf1_dn7 = assign11550_e13126_d_n7;
        locals.var_tmf1_dn10 = assign11550_e13126_d_n10;
        locals.var_tmf1_dn11 = assign11550_e13126_d_n11;
        locals.var_tmf1_dn12 = assign11550_e13126_d_n12;
        locals.var_tmf1_dn17 = assign11550_e13126_d_n17;

        let (assign11560_e13137, assign11560_e13137_d_n0, assign11560_e13137_d_n2, assign11560_e13137_d_n6, assign11560_e13137_d_n7, assign11560_e13137_d_n10, assign11560_e13137_d_n11, assign11560_e13137_d_n12, assign11560_e13137_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign11560_e13135: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign11560_e13135, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign11560_e13137;
        locals.var_x2_dn0 = assign11560_e13137_d_n0;
        locals.var_x2_dn2 = assign11560_e13137_d_n2;
        locals.var_x2_dn6 = assign11560_e13137_d_n6;
        locals.var_x2_dn7 = assign11560_e13137_d_n7;
        locals.var_x2_dn10 = assign11560_e13137_d_n10;
        locals.var_x2_dn11 = assign11560_e13137_d_n11;
        locals.var_x2_dn12 = assign11560_e13137_d_n12;
        locals.var_x2_dn17 = assign11560_e13137_d_n17;

        let (assign11570_e13148, assign11570_e13148_d_n0, assign11570_e13148_d_n2, assign11570_e13148_d_n6, assign11570_e13148_d_n7, assign11570_e13148_d_n10, assign11570_e13148_d_n11, assign11570_e13148_d_n12, assign11570_e13148_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign11570_e13146: f64 = (locals.var_t5 * locals.var_t5);
        (assign11570_e13146, ((locals.var_t5_dn0 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn0)), ((locals.var_t5_dn2 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn2)), ((locals.var_t5_dn6 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn6)), ((locals.var_t5_dn7 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn7)), ((locals.var_t5_dn10 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn10)), ((locals.var_t5_dn11 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn11)), ((locals.var_t5_dn12 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn12)), ((locals.var_t5_dn17 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn17)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign11570_e13148;
        locals.var_xmax2_dn0 = assign11570_e13148_d_n0;
        locals.var_xmax2_dn2 = assign11570_e13148_d_n2;
        locals.var_xmax2_dn6 = assign11570_e13148_d_n6;
        locals.var_xmax2_dn7 = assign11570_e13148_d_n7;
        locals.var_xmax2_dn10 = assign11570_e13148_d_n10;
        locals.var_xmax2_dn11 = assign11570_e13148_d_n11;
        locals.var_xmax2_dn12 = assign11570_e13148_d_n12;
        locals.var_xmax2_dn17 = assign11570_e13148_d_n17;

        let (assign11580_e13157, assign11580_e13157_d_n0, assign11580_e13157_d_n2, assign11580_e13157_d_n6, assign11580_e13157_d_n7, assign11580_e13157_d_n10, assign11580_e13157_d_n11, assign11580_e13157_d_n12, assign11580_e13157_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard311 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign11580_e13157;
        locals.var_xp_dn0 = assign11580_e13157_d_n0;
        locals.var_xp_dn2 = assign11580_e13157_d_n2;
        locals.var_xp_dn6 = assign11580_e13157_d_n6;
        locals.var_xp_dn7 = assign11580_e13157_d_n7;
        locals.var_xp_dn10 = assign11580_e13157_d_n10;
        locals.var_xp_dn11 = assign11580_e13157_d_n11;
        locals.var_xp_dn12 = assign11580_e13157_d_n12;
        locals.var_xp_dn17 = assign11580_e13157_d_n17;

        let (assign11590_e13166, assign11590_e13166_d_n0, assign11590_e13166_d_n2, assign11590_e13166_d_n6, assign11590_e13166_d_n7, assign11590_e13166_d_n10, assign11590_e13166_d_n11, assign11590_e13166_d_n12, assign11590_e13166_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard311 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign11590_e13166;
        locals.var_xmp_dn0 = assign11590_e13166_d_n0;
        locals.var_xmp_dn2 = assign11590_e13166_d_n2;
        locals.var_xmp_dn6 = assign11590_e13166_d_n6;
        locals.var_xmp_dn7 = assign11590_e13166_d_n7;
        locals.var_xmp_dn10 = assign11590_e13166_d_n10;
        locals.var_xmp_dn11 = assign11590_e13166_d_n11;
        locals.var_xmp_dn12 = assign11590_e13166_d_n12;
        locals.var_xmp_dn17 = assign11590_e13166_d_n17;

        let (assign11600_e13175,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard311 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign11600_e13175;

        let (assign11610_e13184,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard311 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign11610_e13184;

        let (assign11620_e13193, assign11620_e13193_d_n0, assign11620_e13193_d_n2, assign11620_e13193_d_n6, assign11620_e13193_d_n7, assign11620_e13193_d_n10, assign11620_e13193_d_n11, assign11620_e13193_d_n12, assign11620_e13193_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard311 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign11620_e13193;
        locals.var_arg_dn0 = assign11620_e13193_d_n0;
        locals.var_arg_dn2 = assign11620_e13193_d_n2;
        locals.var_arg_dn6 = assign11620_e13193_d_n6;
        locals.var_arg_dn7 = assign11620_e13193_d_n7;
        locals.var_arg_dn10 = assign11620_e13193_d_n10;
        locals.var_arg_dn11 = assign11620_e13193_d_n11;
        locals.var_arg_dn12 = assign11620_e13193_d_n12;
        locals.var_arg_dn17 = assign11620_e13193_d_n17;

        let (assign11630_e13202, assign11630_e13202_d_n0, assign11630_e13202_d_n2, assign11630_e13202_d_n6, assign11630_e13202_d_n7, assign11630_e13202_d_n10, assign11630_e13202_d_n11, assign11630_e13202_d_n12, assign11630_e13202_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard311 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign11630_e13202;
        locals.var_dnm_dn0 = assign11630_e13202_d_n0;
        locals.var_dnm_dn2 = assign11630_e13202_d_n2;
        locals.var_dnm_dn6 = assign11630_e13202_d_n6;
        locals.var_dnm_dn7 = assign11630_e13202_d_n7;
        locals.var_dnm_dn10 = assign11630_e13202_d_n10;
        locals.var_dnm_dn11 = assign11630_e13202_d_n11;
        locals.var_dnm_dn12 = assign11630_e13202_d_n12;
        locals.var_dnm_dn17 = assign11630_e13202_d_n17;

        let (assign11640_e13213, assign11640_e13213_d_n0, assign11640_e13213_d_n2, assign11640_e13213_d_n6, assign11640_e13213_d_n7, assign11640_e13213_d_n10, assign11640_e13213_d_n11, assign11640_e13213_d_n12, assign11640_e13213_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign11640_e13211: f64 = (locals.var_xp * locals.var_x2);
        (assign11640_e13211, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign11640_e13213;
        locals.var_xp_dn0 = assign11640_e13213_d_n0;
        locals.var_xp_dn2 = assign11640_e13213_d_n2;
        locals.var_xp_dn6 = assign11640_e13213_d_n6;
        locals.var_xp_dn7 = assign11640_e13213_d_n7;
        locals.var_xp_dn10 = assign11640_e13213_d_n10;
        locals.var_xp_dn11 = assign11640_e13213_d_n11;
        locals.var_xp_dn12 = assign11640_e13213_d_n12;
        locals.var_xp_dn17 = assign11640_e13213_d_n17;

        let (assign11650_e13224, assign11650_e13224_d_n0, assign11650_e13224_d_n2, assign11650_e13224_d_n6, assign11650_e13224_d_n7, assign11650_e13224_d_n10, assign11650_e13224_d_n11, assign11650_e13224_d_n12, assign11650_e13224_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign11650_e13222: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign11650_e13222, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign11650_e13224;
        locals.var_xmp_dn0 = assign11650_e13224_d_n0;
        locals.var_xmp_dn2 = assign11650_e13224_d_n2;
        locals.var_xmp_dn6 = assign11650_e13224_d_n6;
        locals.var_xmp_dn7 = assign11650_e13224_d_n7;
        locals.var_xmp_dn10 = assign11650_e13224_d_n10;
        locals.var_xmp_dn11 = assign11650_e13224_d_n11;
        locals.var_xmp_dn12 = assign11650_e13224_d_n12;
        locals.var_xmp_dn17 = assign11650_e13224_d_n17;

        let (assign11660_e13235, assign11660_e13235_d_n0, assign11660_e13235_d_n2, assign11660_e13235_d_n6, assign11660_e13235_d_n7, assign11660_e13235_d_n10, assign11660_e13235_d_n11, assign11660_e13235_d_n12, assign11660_e13235_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign11660_e13233: f64 = (locals.var_xp * locals.var_x2);
        (assign11660_e13233, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign11660_e13235;
        locals.var_xp_dn0 = assign11660_e13235_d_n0;
        locals.var_xp_dn2 = assign11660_e13235_d_n2;
        locals.var_xp_dn6 = assign11660_e13235_d_n6;
        locals.var_xp_dn7 = assign11660_e13235_d_n7;
        locals.var_xp_dn10 = assign11660_e13235_d_n10;
        locals.var_xp_dn11 = assign11660_e13235_d_n11;
        locals.var_xp_dn12 = assign11660_e13235_d_n12;
        locals.var_xp_dn17 = assign11660_e13235_d_n17;

        let (assign11670_e13246, assign11670_e13246_d_n0, assign11670_e13246_d_n2, assign11670_e13246_d_n6, assign11670_e13246_d_n7, assign11670_e13246_d_n10, assign11670_e13246_d_n11, assign11670_e13246_d_n12, assign11670_e13246_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign11670_e13244: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign11670_e13244, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign11670_e13246;
        locals.var_xmp_dn0 = assign11670_e13246_d_n0;
        locals.var_xmp_dn2 = assign11670_e13246_d_n2;
        locals.var_xmp_dn6 = assign11670_e13246_d_n6;
        locals.var_xmp_dn7 = assign11670_e13246_d_n7;
        locals.var_xmp_dn10 = assign11670_e13246_d_n10;
        locals.var_xmp_dn11 = assign11670_e13246_d_n11;
        locals.var_xmp_dn12 = assign11670_e13246_d_n12;
        locals.var_xmp_dn17 = assign11670_e13246_d_n17;

        let (assign11680_e13257, assign11680_e13257_d_n0, assign11680_e13257_d_n2, assign11680_e13257_d_n6, assign11680_e13257_d_n7, assign11680_e13257_d_n10, assign11680_e13257_d_n11, assign11680_e13257_d_n12, assign11680_e13257_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign11680_e13255: f64 = (locals.var_xp * locals.var_x2);
        (assign11680_e13255, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign11680_e13257;
        locals.var_xp_dn0 = assign11680_e13257_d_n0;
        locals.var_xp_dn2 = assign11680_e13257_d_n2;
        locals.var_xp_dn6 = assign11680_e13257_d_n6;
        locals.var_xp_dn7 = assign11680_e13257_d_n7;
        locals.var_xp_dn10 = assign11680_e13257_d_n10;
        locals.var_xp_dn11 = assign11680_e13257_d_n11;
        locals.var_xp_dn12 = assign11680_e13257_d_n12;
        locals.var_xp_dn17 = assign11680_e13257_d_n17;

        let (assign11690_e13268, assign11690_e13268_d_n0, assign11690_e13268_d_n2, assign11690_e13268_d_n6, assign11690_e13268_d_n7, assign11690_e13268_d_n10, assign11690_e13268_d_n11, assign11690_e13268_d_n12, assign11690_e13268_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign11690_e13266: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign11690_e13266, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign11690_e13268;
        locals.var_xmp_dn0 = assign11690_e13268_d_n0;
        locals.var_xmp_dn2 = assign11690_e13268_d_n2;
        locals.var_xmp_dn6 = assign11690_e13268_d_n6;
        locals.var_xmp_dn7 = assign11690_e13268_d_n7;
        locals.var_xmp_dn10 = assign11690_e13268_d_n10;
        locals.var_xmp_dn11 = assign11690_e13268_d_n11;
        locals.var_xmp_dn12 = assign11690_e13268_d_n12;
        locals.var_xmp_dn17 = assign11690_e13268_d_n17;

        let (assign11700_e13279, assign11700_e13279_d_n0, assign11700_e13279_d_n2, assign11700_e13279_d_n6, assign11700_e13279_d_n7, assign11700_e13279_d_n10, assign11700_e13279_d_n11, assign11700_e13279_d_n12, assign11700_e13279_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign11700_e13277: f64 = (locals.var_xp * locals.var_x2);
        (assign11700_e13277, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign11700_e13279;
        locals.var_xp_dn0 = assign11700_e13279_d_n0;
        locals.var_xp_dn2 = assign11700_e13279_d_n2;
        locals.var_xp_dn6 = assign11700_e13279_d_n6;
        locals.var_xp_dn7 = assign11700_e13279_d_n7;
        locals.var_xp_dn10 = assign11700_e13279_d_n10;
        locals.var_xp_dn11 = assign11700_e13279_d_n11;
        locals.var_xp_dn12 = assign11700_e13279_d_n12;
        locals.var_xp_dn17 = assign11700_e13279_d_n17;

        let (assign11710_e13290, assign11710_e13290_d_n0, assign11710_e13290_d_n2, assign11710_e13290_d_n6, assign11710_e13290_d_n7, assign11710_e13290_d_n10, assign11710_e13290_d_n11, assign11710_e13290_d_n12, assign11710_e13290_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign11710_e13288: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign11710_e13288, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign11710_e13290;
        locals.var_xmp_dn0 = assign11710_e13290_d_n0;
        locals.var_xmp_dn2 = assign11710_e13290_d_n2;
        locals.var_xmp_dn6 = assign11710_e13290_d_n6;
        locals.var_xmp_dn7 = assign11710_e13290_d_n7;
        locals.var_xmp_dn10 = assign11710_e13290_d_n10;
        locals.var_xmp_dn11 = assign11710_e13290_d_n11;
        locals.var_xmp_dn12 = assign11710_e13290_d_n12;
        locals.var_xmp_dn17 = assign11710_e13290_d_n17;

        let (assign11720_e13301, assign11720_e13301_d_n0, assign11720_e13301_d_n2, assign11720_e13301_d_n6, assign11720_e13301_d_n7, assign11720_e13301_d_n10, assign11720_e13301_d_n11, assign11720_e13301_d_n12, assign11720_e13301_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign11720_e13299: f64 = (locals.var_xp + locals.var_xmp);
        (assign11720_e13299, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign11720_e13301;
        locals.var_arg_dn0 = assign11720_e13301_d_n0;
        locals.var_arg_dn2 = assign11720_e13301_d_n2;
        locals.var_arg_dn6 = assign11720_e13301_d_n6;
        locals.var_arg_dn7 = assign11720_e13301_d_n7;
        locals.var_arg_dn10 = assign11720_e13301_d_n10;
        locals.var_arg_dn11 = assign11720_e13301_d_n11;
        locals.var_arg_dn12 = assign11720_e13301_d_n12;
        locals.var_arg_dn17 = assign11720_e13301_d_n17;

        let (assign11730_e13310, assign11730_e13310_d_n0, assign11730_e13310_d_n2, assign11730_e13310_d_n6, assign11730_e13310_d_n7, assign11730_e13310_d_n10, assign11730_e13310_d_n11, assign11730_e13310_d_n12, assign11730_e13310_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard311 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign11730_e13310;
        locals.var_dnm_dn0 = assign11730_e13310_d_n0;
        locals.var_dnm_dn2 = assign11730_e13310_d_n2;
        locals.var_dnm_dn6 = assign11730_e13310_d_n6;
        locals.var_dnm_dn7 = assign11730_e13310_d_n7;
        locals.var_dnm_dn10 = assign11730_e13310_d_n10;
        locals.var_dnm_dn11 = assign11730_e13310_d_n11;
        locals.var_dnm_dn12 = assign11730_e13310_d_n12;
        locals.var_dnm_dn17 = assign11730_e13310_d_n17;

        let assign11740_e13325: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard312 = assign11740_e13325;

        let assign11750_e13328: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard313 = assign11750_e13328;

        let (assign11760_e13341,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 != 0.0)) && (locals.var_guard313 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign11760_e13341;

        let assign11770_e13344: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard314 = assign11770_e13344;

        let (assign11780_e13360,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 != 0.0)) && (locals.var_guard313 == 0.0)) && (locals.var_guard314 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign11780_e13360;

        let assign11790_e13363: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard315 = assign11790_e13363;

        let (assign11800_e13382,) = {
    if (((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 != 0.0)) && (locals.var_guard313 == 0.0)) && (locals.var_guard314 == 0.0)) && (locals.var_guard315 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign11800_e13382;

        let assign11810_e13385: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard316 = assign11810_e13385;

        let (assign11820_e13407,) = {
    if ((((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 != 0.0)) && (locals.var_guard313 == 0.0)) && (locals.var_guard314 == 0.0)) && (locals.var_guard315 == 0.0)) && (locals.var_guard316 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign11820_e13407;

        let (assign11830_e13418,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign11830_e13418;

        let mut assign11840_loop_guard: usize = 0;
        while {
            let assign11840_cond_e13430: f64 = if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign11840_cond_e13430 != 0.0
        } {
            assign11840_loop_guard += 1;
            assert!(assign11840_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign11840_body0_e13442, assign11840_body0_e13442_d_n0, assign11840_body0_e13442_d_n2, assign11840_body0_e13442_d_n6, assign11840_body0_e13442_d_n7, assign11840_body0_e13442_d_n10, assign11840_body0_e13442_d_n11, assign11840_body0_e13442_d_n12, assign11840_body0_e13442_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 != 0.0)) {
        let assign11840_body0_e13440: f64 = (locals.var_dnm).sqrt();
        (assign11840_body0_e13440, (locals.var_dnm_dn0 / (2.0 * assign11840_body0_e13440)), (locals.var_dnm_dn2 / (2.0 * assign11840_body0_e13440)), (locals.var_dnm_dn6 / (2.0 * assign11840_body0_e13440)), (locals.var_dnm_dn7 / (2.0 * assign11840_body0_e13440)), (locals.var_dnm_dn10 / (2.0 * assign11840_body0_e13440)), (locals.var_dnm_dn11 / (2.0 * assign11840_body0_e13440)), (locals.var_dnm_dn12 / (2.0 * assign11840_body0_e13440)), (locals.var_dnm_dn17 / (2.0 * assign11840_body0_e13440)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign11840_body0_e13442;
            locals.var_dnm_dn0 = assign11840_body0_e13442_d_n0;
            locals.var_dnm_dn2 = assign11840_body0_e13442_d_n2;
            locals.var_dnm_dn6 = assign11840_body0_e13442_d_n6;
            locals.var_dnm_dn7 = assign11840_body0_e13442_d_n7;
            locals.var_dnm_dn10 = assign11840_body0_e13442_d_n10;
            locals.var_dnm_dn11 = assign11840_body0_e13442_d_n11;
            locals.var_dnm_dn12 = assign11840_body0_e13442_d_n12;
            locals.var_dnm_dn17 = assign11840_body0_e13442_d_n17;
            let (assign11840_body1_e13455,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 != 0.0)) {
        let assign11840_body1_e13453: f64 = (locals.var_m0 + 1.0);
        (assign11840_body1_e13453,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign11840_body1_e13455;
        }

    }

    pub(super) fn stamp_transient_block_34(
        locals: &mut StampLocals,
    ) {
        let (assign11850_e13473, assign11850_e13473_d_n0, assign11850_e13473_d_n2, assign11850_e13473_d_n6, assign11850_e13473_d_n7, assign11850_e13473_d_n10, assign11850_e13473_d_n11, assign11850_e13473_d_n12, assign11850_e13473_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 == 0.0)) {
        let assign11850_e13469: f64 = (2.0 * 4.0);
        let assign11850_e13470: f64 = (1.0 / assign11850_e13469);
        let assign11850_e13471: f64 = (locals.var_dnm).powf(assign11850_e13470);
        (assign11850_e13471, if 0.0 == 0.0 && ((assign11850_e13470) as f64).is_finite() && ((assign11850_e13470) as f64).fract() == 0.0 { if assign11850_e13470 == 0.0 { 0.0 } else { (assign11850_e13470 * ((locals.var_dnm).powf(assign11850_e13470 - 1.0) * locals.var_dnm_dn0)) } } else { (assign11850_e13471 * (assign11850_e13470 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign11850_e13470) as f64).is_finite() && ((assign11850_e13470) as f64).fract() == 0.0 { if assign11850_e13470 == 0.0 { 0.0 } else { (assign11850_e13470 * ((locals.var_dnm).powf(assign11850_e13470 - 1.0) * locals.var_dnm_dn2)) } } else { (assign11850_e13471 * (assign11850_e13470 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign11850_e13470) as f64).is_finite() && ((assign11850_e13470) as f64).fract() == 0.0 { if assign11850_e13470 == 0.0 { 0.0 } else { (assign11850_e13470 * ((locals.var_dnm).powf(assign11850_e13470 - 1.0) * locals.var_dnm_dn6)) } } else { (assign11850_e13471 * (assign11850_e13470 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign11850_e13470) as f64).is_finite() && ((assign11850_e13470) as f64).fract() == 0.0 { if assign11850_e13470 == 0.0 { 0.0 } else { (assign11850_e13470 * ((locals.var_dnm).powf(assign11850_e13470 - 1.0) * locals.var_dnm_dn7)) } } else { (assign11850_e13471 * (assign11850_e13470 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign11850_e13470) as f64).is_finite() && ((assign11850_e13470) as f64).fract() == 0.0 { if assign11850_e13470 == 0.0 { 0.0 } else { (assign11850_e13470 * ((locals.var_dnm).powf(assign11850_e13470 - 1.0) * locals.var_dnm_dn10)) } } else { (assign11850_e13471 * (assign11850_e13470 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign11850_e13470) as f64).is_finite() && ((assign11850_e13470) as f64).fract() == 0.0 { if assign11850_e13470 == 0.0 { 0.0 } else { (assign11850_e13470 * ((locals.var_dnm).powf(assign11850_e13470 - 1.0) * locals.var_dnm_dn11)) } } else { (assign11850_e13471 * (assign11850_e13470 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign11850_e13470) as f64).is_finite() && ((assign11850_e13470) as f64).fract() == 0.0 { if assign11850_e13470 == 0.0 { 0.0 } else { (assign11850_e13470 * ((locals.var_dnm).powf(assign11850_e13470 - 1.0) * locals.var_dnm_dn12)) } } else { (assign11850_e13471 * (assign11850_e13470 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign11850_e13470) as f64).is_finite() && ((assign11850_e13470) as f64).fract() == 0.0 { if assign11850_e13470 == 0.0 { 0.0 } else { (assign11850_e13470 * ((locals.var_dnm).powf(assign11850_e13470 - 1.0) * locals.var_dnm_dn17)) } } else { (assign11850_e13471 * (assign11850_e13470 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign11850_e13473;
        locals.var_dnm_dn0 = assign11850_e13473_d_n0;
        locals.var_dnm_dn2 = assign11850_e13473_d_n2;
        locals.var_dnm_dn6 = assign11850_e13473_d_n6;
        locals.var_dnm_dn7 = assign11850_e13473_d_n7;
        locals.var_dnm_dn10 = assign11850_e13473_d_n10;
        locals.var_dnm_dn11 = assign11850_e13473_d_n11;
        locals.var_dnm_dn12 = assign11850_e13473_d_n12;
        locals.var_dnm_dn17 = assign11850_e13473_d_n17;

        let (assign11860_e13484, assign11860_e13484_d_n0, assign11860_e13484_d_n2, assign11860_e13484_d_n6, assign11860_e13484_d_n7, assign11860_e13484_d_n10, assign11860_e13484_d_n11, assign11860_e13484_d_n12, assign11860_e13484_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign11860_e13482: f64 = (1.0 / locals.var_dnm);
        (assign11860_e13482, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign11860_e13484;
        locals.var_dnm_dn0 = assign11860_e13484_d_n0;
        locals.var_dnm_dn2 = assign11860_e13484_d_n2;
        locals.var_dnm_dn6 = assign11860_e13484_d_n6;
        locals.var_dnm_dn7 = assign11860_e13484_d_n7;
        locals.var_dnm_dn10 = assign11860_e13484_d_n10;
        locals.var_dnm_dn11 = assign11860_e13484_d_n11;
        locals.var_dnm_dn12 = assign11860_e13484_d_n12;
        locals.var_dnm_dn17 = assign11860_e13484_d_n17;

        let (assign11870_e13497, assign11870_e13497_d_n0, assign11870_e13497_d_n2, assign11870_e13497_d_n6, assign11870_e13497_d_n7, assign11870_e13497_d_n10, assign11870_e13497_d_n11, assign11870_e13497_d_n12, assign11870_e13497_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign11870_e13493: f64 = (locals.var_tmf1 * locals.var_t5);
        let assign11870_e13495: f64 = (assign11870_e13493 * locals.var_dnm);
        (assign11870_e13495, ((((locals.var_tmf1_dn0 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn0)) * locals.var_dnm) + (assign11870_e13493 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn2)) * locals.var_dnm) + (assign11870_e13493 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn6 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn6)) * locals.var_dnm) + (assign11870_e13493 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn7)) * locals.var_dnm) + (assign11870_e13493 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn10 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn10)) * locals.var_dnm) + (assign11870_e13493 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn11)) * locals.var_dnm) + (assign11870_e13493 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn12 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn12)) * locals.var_dnm) + (assign11870_e13493 * locals.var_dnm_dn12)), ((((locals.var_tmf1_dn17 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn17)) * locals.var_dnm) + (assign11870_e13493 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign11870_e13497;
        locals.var_tmf0_dn0 = assign11870_e13497_d_n0;
        locals.var_tmf0_dn2 = assign11870_e13497_d_n2;
        locals.var_tmf0_dn6 = assign11870_e13497_d_n6;
        locals.var_tmf0_dn7 = assign11870_e13497_d_n7;
        locals.var_tmf0_dn10 = assign11870_e13497_d_n10;
        locals.var_tmf0_dn11 = assign11870_e13497_d_n11;
        locals.var_tmf0_dn12 = assign11870_e13497_d_n12;
        locals.var_tmf0_dn17 = assign11870_e13497_d_n17;

        let (assign11880_e13510, assign11880_e13510_d_n0, assign11880_e13510_d_n2, assign11880_e13510_d_n6, assign11880_e13510_d_n7, assign11880_e13510_d_n10, assign11880_e13510_d_n11, assign11880_e13510_d_n12, assign11880_e13510_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign11880_e13506: f64 = locals.var_t5;
        let assign11880_e13508: f64 = (assign11880_e13506 - locals.var_tmf0);
        (assign11880_e13508, (locals.var_t5_dn0 - locals.var_tmf0_dn0), (locals.var_t5_dn2 - locals.var_tmf0_dn2), (locals.var_t5_dn6 - locals.var_tmf0_dn6), (locals.var_t5_dn7 - locals.var_tmf0_dn7), (locals.var_t5_dn10 - locals.var_tmf0_dn10), (locals.var_t5_dn11 - locals.var_tmf0_dn11), (locals.var_t5_dn12 - locals.var_tmf0_dn12), (locals.var_t5_dn17 - locals.var_tmf0_dn17),)
    } else {
        (locals.var_t4__blk307, locals.var_t4__blk307_dn0, locals.var_t4__blk307_dn2, locals.var_t4__blk307_dn6, locals.var_t4__blk307_dn7, locals.var_t4__blk307_dn10, locals.var_t4__blk307_dn11, locals.var_t4__blk307_dn12, locals.var_t4__blk307_dn17,)
    }
};
        locals.var_t4__blk307 = assign11880_e13510;
        locals.var_t4__blk307_dn0 = assign11880_e13510_d_n0;
        locals.var_t4__blk307_dn2 = assign11880_e13510_d_n2;
        locals.var_t4__blk307_dn6 = assign11880_e13510_d_n6;
        locals.var_t4__blk307_dn7 = assign11880_e13510_d_n7;
        locals.var_t4__blk307_dn10 = assign11880_e13510_d_n10;
        locals.var_t4__blk307_dn11 = assign11880_e13510_d_n11;
        locals.var_t4__blk307_dn12 = assign11880_e13510_d_n12;
        locals.var_t4__blk307_dn17 = assign11880_e13510_d_n17;

        let (assign11890_e13520, assign11890_e13520_d_n0, assign11890_e13520_d_n2, assign11890_e13520_d_n6, assign11890_e13520_d_n7, assign11890_e13520_d_n10, assign11890_e13520_d_n11, assign11890_e13520_d_n12, assign11890_e13520_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard311 == 0.0)) {
        (locals.var_t4__blk307, locals.var_t4__blk307_dn0, locals.var_t4__blk307_dn2, locals.var_t4__blk307_dn6, locals.var_t4__blk307_dn7, locals.var_t4__blk307_dn10, locals.var_t4__blk307_dn11, locals.var_t4__blk307_dn12, locals.var_t4__blk307_dn17,)
    } else {
        (locals.var_t4__blk307, locals.var_t4__blk307_dn0, locals.var_t4__blk307_dn2, locals.var_t4__blk307_dn6, locals.var_t4__blk307_dn7, locals.var_t4__blk307_dn10, locals.var_t4__blk307_dn11, locals.var_t4__blk307_dn12, locals.var_t4__blk307_dn17,)
    }
};
        locals.var_t4__blk307 = assign11890_e13520;
        locals.var_t4__blk307_dn0 = assign11890_e13520_d_n0;
        locals.var_t4__blk307_dn2 = assign11890_e13520_d_n2;
        locals.var_t4__blk307_dn6 = assign11890_e13520_d_n6;
        locals.var_t4__blk307_dn7 = assign11890_e13520_d_n7;
        locals.var_t4__blk307_dn10 = assign11890_e13520_d_n10;
        locals.var_t4__blk307_dn11 = assign11890_e13520_d_n11;
        locals.var_t4__blk307_dn12 = assign11890_e13520_d_n12;
        locals.var_t4__blk307_dn17 = assign11890_e13520_d_n17;

        let (assign11900_e13528, assign11900_e13528_d_n0, assign11900_e13528_d_n2, assign11900_e13528_d_n6, assign11900_e13528_d_n7, assign11900_e13528_d_n10, assign11900_e13528_d_n11, assign11900_e13528_d_n12, assign11900_e13528_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign11900_e13526: f64 = (locals.var_t4__blk307).sqrt();
        (assign11900_e13526, (locals.var_t4__blk307_dn0 / (2.0 * assign11900_e13526)), (locals.var_t4__blk307_dn2 / (2.0 * assign11900_e13526)), (locals.var_t4__blk307_dn6 / (2.0 * assign11900_e13526)), (locals.var_t4__blk307_dn7 / (2.0 * assign11900_e13526)), (locals.var_t4__blk307_dn10 / (2.0 * assign11900_e13526)), (locals.var_t4__blk307_dn11 / (2.0 * assign11900_e13526)), (locals.var_t4__blk307_dn12 / (2.0 * assign11900_e13526)), (locals.var_t4__blk307_dn17 / (2.0 * assign11900_e13526)),)
    } else {
        (locals.var_t3__blk306, locals.var_t3__blk306_dn0, locals.var_t3__blk306_dn2, locals.var_t3__blk306_dn6, locals.var_t3__blk306_dn7, locals.var_t3__blk306_dn10, locals.var_t3__blk306_dn11, locals.var_t3__blk306_dn12, locals.var_t3__blk306_dn17,)
    }
};
        locals.var_t3__blk306 = assign11900_e13528;
        locals.var_t3__blk306_dn0 = assign11900_e13528_d_n0;
        locals.var_t3__blk306_dn2 = assign11900_e13528_d_n2;
        locals.var_t3__blk306_dn6 = assign11900_e13528_d_n6;
        locals.var_t3__blk306_dn7 = assign11900_e13528_d_n7;
        locals.var_t3__blk306_dn10 = assign11900_e13528_d_n10;
        locals.var_t3__blk306_dn11 = assign11900_e13528_d_n11;
        locals.var_t3__blk306_dn12 = assign11900_e13528_d_n12;
        locals.var_t3__blk306_dn17 = assign11900_e13528_d_n17;

        let (assign11910_e13541, assign11910_e13541_d_n0, assign11910_e13541_d_n2, assign11910_e13541_d_n6, assign11910_e13541_d_n7, assign11910_e13541_d_n10, assign11910_e13541_d_n11, assign11910_e13541_d_n12, assign11910_e13541_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign11910_e13537: f64 = (1.0 - locals.var_t3__blk306);
        let assign11910_e13538: f64 = (locals.var_t2__blk305 * assign11910_e13537);
        let assign11910_e13539: f64 = (locals.var_vgp + assign11910_e13538);
        (assign11910_e13539, (locals.var_vgp_dn0 + ((locals.var_t2__blk305_dn0 * assign11910_e13537) + (locals.var_t2__blk305 * (-locals.var_t3__blk306_dn0)))), (locals.var_vgp_dn2 + ((locals.var_t2__blk305_dn2 * assign11910_e13537) + (locals.var_t2__blk305 * (-locals.var_t3__blk306_dn2)))), (locals.var_vgp_dn6 + ((locals.var_t2__blk305_dn6 * assign11910_e13537) + (locals.var_t2__blk305 * (-locals.var_t3__blk306_dn6)))), (locals.var_vgp_dn7 + ((locals.var_t2__blk305_dn7 * assign11910_e13537) + (locals.var_t2__blk305 * (-locals.var_t3__blk306_dn7)))), (locals.var_vgp_dn10 + ((locals.var_t2__blk305_dn10 * assign11910_e13537) + (locals.var_t2__blk305 * (-locals.var_t3__blk306_dn10)))), (locals.var_vgp_dn11 + ((locals.var_t2__blk305_dn11 * assign11910_e13537) + (locals.var_t2__blk305 * (-locals.var_t3__blk306_dn11)))), (locals.var_vgp_dn12 + ((locals.var_t2__blk305_dn12 * assign11910_e13537) + (locals.var_t2__blk305 * (-locals.var_t3__blk306_dn12)))), (locals.var_vgp_dn17 + ((locals.var_t2__blk305_dn17 * assign11910_e13537) + (locals.var_t2__blk305 * (-locals.var_t3__blk306_dn17)))),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12, locals.var_t10_dn17,)
    }
};
        locals.var_t10 = assign11910_e13541;
        locals.var_t10_dn0 = assign11910_e13541_d_n0;
        locals.var_t10_dn2 = assign11910_e13541_d_n2;
        locals.var_t10_dn6 = assign11910_e13541_d_n6;
        locals.var_t10_dn7 = assign11910_e13541_d_n7;
        locals.var_t10_dn10 = assign11910_e13541_d_n10;
        locals.var_t10_dn11 = assign11910_e13541_d_n11;
        locals.var_t10_dn12 = assign11910_e13541_d_n12;
        locals.var_t10_dn17 = assign11910_e13541_d_n17;

        let (assign11920_e13557, assign11920_e13557_d_n0, assign11920_e13557_d_n2, assign11920_e13557_d_n6, assign11920_e13557_d_n7, assign11920_e13557_d_n10, assign11920_e13557_d_n11, assign11920_e13557_d_n12, assign11920_e13557_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign11920_e13548: f64 = (locals.var_t10 * locals.var_t10);
        let assign11920_e13551: f64 = (4.0 * 0.01);
        let assign11920_e13553: f64 = (assign11920_e13551 * 0.01);
        let assign11920_e13554: f64 = (assign11920_e13548 + assign11920_e13553);
        let assign11920_e13555: f64 = (assign11920_e13554).sqrt();
        (assign11920_e13555, (((locals.var_t10_dn0 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn0)) / (2.0 * assign11920_e13555)), (((locals.var_t10_dn2 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn2)) / (2.0 * assign11920_e13555)), (((locals.var_t10_dn6 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn6)) / (2.0 * assign11920_e13555)), (((locals.var_t10_dn7 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn7)) / (2.0 * assign11920_e13555)), (((locals.var_t10_dn10 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn10)) / (2.0 * assign11920_e13555)), (((locals.var_t10_dn11 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn11)) / (2.0 * assign11920_e13555)), (((locals.var_t10_dn12 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn12)) / (2.0 * assign11920_e13555)), (((locals.var_t10_dn17 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn17)) / (2.0 * assign11920_e13555)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign11920_e13557;
        locals.var_tmf1_dn0 = assign11920_e13557_d_n0;
        locals.var_tmf1_dn2 = assign11920_e13557_d_n2;
        locals.var_tmf1_dn6 = assign11920_e13557_d_n6;
        locals.var_tmf1_dn7 = assign11920_e13557_d_n7;
        locals.var_tmf1_dn10 = assign11920_e13557_d_n10;
        locals.var_tmf1_dn11 = assign11920_e13557_d_n11;
        locals.var_tmf1_dn12 = assign11920_e13557_d_n12;
        locals.var_tmf1_dn17 = assign11920_e13557_d_n17;

        let (assign11930_e13572, assign11930_e13572_d_n0, assign11930_e13572_d_n2, assign11930_e13572_d_n6, assign11930_e13572_d_n7, assign11930_e13572_d_n10, assign11930_e13572_d_n11, assign11930_e13572_d_n12, assign11930_e13572_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign11930_e13565: f64 = (locals.var_t10 + locals.var_tmf1);
        let assign11930_e13566: f64 = (0.5 * assign11930_e13565);
        let assign11930_e13569: f64 = (1e-10 * 0.01);
        let assign11930_e13570: f64 = (assign11930_e13566 + assign11930_e13569);
        (assign11930_e13570, (0.5 * (locals.var_t10_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t10_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t10_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t10_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t10_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t10_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t10_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t10_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12, locals.var_t10_dn17,)
    }
};
        locals.var_t10 = assign11930_e13572;
        locals.var_t10_dn0 = assign11930_e13572_d_n0;
        locals.var_t10_dn2 = assign11930_e13572_d_n2;
        locals.var_t10_dn6 = assign11930_e13572_d_n6;
        locals.var_t10_dn7 = assign11930_e13572_d_n7;
        locals.var_t10_dn10 = assign11930_e13572_d_n10;
        locals.var_t10_dn11 = assign11930_e13572_d_n11;
        locals.var_t10_dn12 = assign11930_e13572_d_n12;
        locals.var_t10_dn17 = assign11930_e13572_d_n17;

        let assign11940_e13575: f64 = if locals.var_t10 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard317 = assign11940_e13575;

        let (assign11950_e13584, assign11950_e13584_d_n0, assign11950_e13584_d_n2, assign11950_e13584_d_n6, assign11950_e13584_d_n7, assign11950_e13584_d_n10, assign11950_e13584_d_n11, assign11950_e13584_d_n12, assign11950_e13584_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard317 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12, locals.var_t10_dn17,)
    }
};
        locals.var_t10 = assign11950_e13584;
        locals.var_t10_dn0 = assign11950_e13584_d_n0;
        locals.var_t10_dn2 = assign11950_e13584_d_n2;
        locals.var_t10_dn6 = assign11950_e13584_d_n6;
        locals.var_t10_dn7 = assign11950_e13584_d_n7;
        locals.var_t10_dn10 = assign11950_e13584_d_n10;
        locals.var_t10_dn11 = assign11950_e13584_d_n11;
        locals.var_t10_dn12 = assign11950_e13584_d_n12;
        locals.var_t10_dn17 = assign11950_e13584_d_n17;

        let (assign11970_e13600, assign11970_e13600_d_n0, assign11970_e13600_d_n2, assign11970_e13600_d_n6, assign11970_e13600_d_n7, assign11970_e13600_d_n10, assign11970_e13600_d_n11, assign11970_e13600_d_n12, assign11970_e13600_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign11970_e13598: f64 = (locals.var_vds / locals.var_t10);
        (assign11970_e13598, (((locals.var_vds_dn0 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn0)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn2 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn2)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn6 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn6)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn7 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn7)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn10 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn10)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn11 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn11)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn12 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn12)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn17 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn17)) / (locals.var_t10 * locals.var_t10)),)
    } else {
        (locals.var_t1__blk304, locals.var_t1__blk304_dn0, locals.var_t1__blk304_dn2, locals.var_t1__blk304_dn6, locals.var_t1__blk304_dn7, locals.var_t1__blk304_dn10, locals.var_t1__blk304_dn11, locals.var_t1__blk304_dn12, locals.var_t1__blk304_dn17,)
    }
};
        locals.var_t1__blk304 = assign11970_e13600;
        locals.var_t1__blk304_dn0 = assign11970_e13600_d_n0;
        locals.var_t1__blk304_dn2 = assign11970_e13600_d_n2;
        locals.var_t1__blk304_dn6 = assign11970_e13600_d_n6;
        locals.var_t1__blk304_dn7 = assign11970_e13600_d_n7;
        locals.var_t1__blk304_dn10 = assign11970_e13600_d_n10;
        locals.var_t1__blk304_dn11 = assign11970_e13600_d_n11;
        locals.var_t1__blk304_dn12 = assign11970_e13600_d_n12;
        locals.var_t1__blk304_dn17 = assign11970_e13600_d_n17;

        let (assign11980_e13611, assign11980_e13611_d_n0, assign11980_e13611_d_n2, assign11980_e13611_d_n6, assign11980_e13611_d_n7, assign11980_e13611_d_n10, assign11980_e13611_d_n11, assign11980_e13611_d_n12, assign11980_e13611_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign11980_e13608: f64 = (locals.var_ddlte - 1.0);
        let assign11980_e13609: f64 = (locals.var_t1__blk304).powf(assign11980_e13608);
        (assign11980_e13609, if 0.0 == 0.0 && ((assign11980_e13608) as f64).is_finite() && ((assign11980_e13608) as f64).fract() == 0.0 { if assign11980_e13608 == 0.0 { 0.0 } else { (assign11980_e13608 * ((locals.var_t1__blk304).powf(assign11980_e13608 - 1.0) * locals.var_t1__blk304_dn0)) } } else { (assign11980_e13609 * (assign11980_e13608 * (locals.var_t1__blk304_dn0 / locals.var_t1__blk304))) }, if 0.0 == 0.0 && ((assign11980_e13608) as f64).is_finite() && ((assign11980_e13608) as f64).fract() == 0.0 { if assign11980_e13608 == 0.0 { 0.0 } else { (assign11980_e13608 * ((locals.var_t1__blk304).powf(assign11980_e13608 - 1.0) * locals.var_t1__blk304_dn2)) } } else { (assign11980_e13609 * (assign11980_e13608 * (locals.var_t1__blk304_dn2 / locals.var_t1__blk304))) }, if 0.0 == 0.0 && ((assign11980_e13608) as f64).is_finite() && ((assign11980_e13608) as f64).fract() == 0.0 { if assign11980_e13608 == 0.0 { 0.0 } else { (assign11980_e13608 * ((locals.var_t1__blk304).powf(assign11980_e13608 - 1.0) * locals.var_t1__blk304_dn6)) } } else { (assign11980_e13609 * (assign11980_e13608 * (locals.var_t1__blk304_dn6 / locals.var_t1__blk304))) }, if 0.0 == 0.0 && ((assign11980_e13608) as f64).is_finite() && ((assign11980_e13608) as f64).fract() == 0.0 { if assign11980_e13608 == 0.0 { 0.0 } else { (assign11980_e13608 * ((locals.var_t1__blk304).powf(assign11980_e13608 - 1.0) * locals.var_t1__blk304_dn7)) } } else { (assign11980_e13609 * (assign11980_e13608 * (locals.var_t1__blk304_dn7 / locals.var_t1__blk304))) }, if 0.0 == 0.0 && ((assign11980_e13608) as f64).is_finite() && ((assign11980_e13608) as f64).fract() == 0.0 { if assign11980_e13608 == 0.0 { 0.0 } else { (assign11980_e13608 * ((locals.var_t1__blk304).powf(assign11980_e13608 - 1.0) * locals.var_t1__blk304_dn10)) } } else { (assign11980_e13609 * (assign11980_e13608 * (locals.var_t1__blk304_dn10 / locals.var_t1__blk304))) }, if 0.0 == 0.0 && ((assign11980_e13608) as f64).is_finite() && ((assign11980_e13608) as f64).fract() == 0.0 { if assign11980_e13608 == 0.0 { 0.0 } else { (assign11980_e13608 * ((locals.var_t1__blk304).powf(assign11980_e13608 - 1.0) * locals.var_t1__blk304_dn11)) } } else { (assign11980_e13609 * (assign11980_e13608 * (locals.var_t1__blk304_dn11 / locals.var_t1__blk304))) }, if 0.0 == 0.0 && ((assign11980_e13608) as f64).is_finite() && ((assign11980_e13608) as f64).fract() == 0.0 { if assign11980_e13608 == 0.0 { 0.0 } else { (assign11980_e13608 * ((locals.var_t1__blk304).powf(assign11980_e13608 - 1.0) * locals.var_t1__blk304_dn12)) } } else { (assign11980_e13609 * (assign11980_e13608 * (locals.var_t1__blk304_dn12 / locals.var_t1__blk304))) }, if 0.0 == 0.0 && ((assign11980_e13608) as f64).is_finite() && ((assign11980_e13608) as f64).fract() == 0.0 { if assign11980_e13608 == 0.0 { 0.0 } else { (assign11980_e13608 * ((locals.var_t1__blk304).powf(assign11980_e13608 - 1.0) * locals.var_t1__blk304_dn17)) } } else { (assign11980_e13609 * (assign11980_e13608 * (locals.var_t1__blk304_dn17 / locals.var_t1__blk304))) },)
    } else {
        (locals.var_t2__blk305, locals.var_t2__blk305_dn0, locals.var_t2__blk305_dn2, locals.var_t2__blk305_dn6, locals.var_t2__blk305_dn7, locals.var_t2__blk305_dn10, locals.var_t2__blk305_dn11, locals.var_t2__blk305_dn12, locals.var_t2__blk305_dn17,)
    }
};
        locals.var_t2__blk305 = assign11980_e13611;
        locals.var_t2__blk305_dn0 = assign11980_e13611_d_n0;
        locals.var_t2__blk305_dn2 = assign11980_e13611_d_n2;
        locals.var_t2__blk305_dn6 = assign11980_e13611_d_n6;
        locals.var_t2__blk305_dn7 = assign11980_e13611_d_n7;
        locals.var_t2__blk305_dn10 = assign11980_e13611_d_n10;
        locals.var_t2__blk305_dn11 = assign11980_e13611_d_n11;
        locals.var_t2__blk305_dn12 = assign11980_e13611_d_n12;
        locals.var_t2__blk305_dn17 = assign11980_e13611_d_n17;

        let (assign11990_e13620, assign11990_e13620_d_n0, assign11990_e13620_d_n2, assign11990_e13620_d_n6, assign11990_e13620_d_n7, assign11990_e13620_d_n10, assign11990_e13620_d_n11, assign11990_e13620_d_n12, assign11990_e13620_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign11990_e13618: f64 = (locals.var_t2__blk305 * locals.var_t1__blk304);
        (assign11990_e13618, ((locals.var_t2__blk305_dn0 * locals.var_t1__blk304) + (locals.var_t2__blk305 * locals.var_t1__blk304_dn0)), ((locals.var_t2__blk305_dn2 * locals.var_t1__blk304) + (locals.var_t2__blk305 * locals.var_t1__blk304_dn2)), ((locals.var_t2__blk305_dn6 * locals.var_t1__blk304) + (locals.var_t2__blk305 * locals.var_t1__blk304_dn6)), ((locals.var_t2__blk305_dn7 * locals.var_t1__blk304) + (locals.var_t2__blk305 * locals.var_t1__blk304_dn7)), ((locals.var_t2__blk305_dn10 * locals.var_t1__blk304) + (locals.var_t2__blk305 * locals.var_t1__blk304_dn10)), ((locals.var_t2__blk305_dn11 * locals.var_t1__blk304) + (locals.var_t2__blk305 * locals.var_t1__blk304_dn11)), ((locals.var_t2__blk305_dn12 * locals.var_t1__blk304) + (locals.var_t2__blk305 * locals.var_t1__blk304_dn12)), ((locals.var_t2__blk305_dn17 * locals.var_t1__blk304) + (locals.var_t2__blk305 * locals.var_t1__blk304_dn17)),)
    } else {
        (locals.var_t7__blk309, locals.var_t7__blk309_dn0, locals.var_t7__blk309_dn2, locals.var_t7__blk309_dn6, locals.var_t7__blk309_dn7, locals.var_t7__blk309_dn10, locals.var_t7__blk309_dn11, locals.var_t7__blk309_dn12, locals.var_t7__blk309_dn17,)
    }
};
        locals.var_t7__blk309 = assign11990_e13620;
        locals.var_t7__blk309_dn0 = assign11990_e13620_d_n0;
        locals.var_t7__blk309_dn2 = assign11990_e13620_d_n2;
        locals.var_t7__blk309_dn6 = assign11990_e13620_d_n6;
        locals.var_t7__blk309_dn7 = assign11990_e13620_d_n7;
        locals.var_t7__blk309_dn10 = assign11990_e13620_d_n10;
        locals.var_t7__blk309_dn11 = assign11990_e13620_d_n11;
        locals.var_t7__blk309_dn12 = assign11990_e13620_d_n12;
        locals.var_t7__blk309_dn17 = assign11990_e13620_d_n17;

        let (assign12000_e13629, assign12000_e13629_d_n0, assign12000_e13629_d_n2, assign12000_e13629_d_n6, assign12000_e13629_d_n7, assign12000_e13629_d_n10, assign12000_e13629_d_n11, assign12000_e13629_d_n12, assign12000_e13629_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign12000_e13627: f64 = (1.0 + locals.var_t7__blk309);
        (assign12000_e13627, locals.var_t7__blk309_dn0, locals.var_t7__blk309_dn2, locals.var_t7__blk309_dn6, locals.var_t7__blk309_dn7, locals.var_t7__blk309_dn10, locals.var_t7__blk309_dn11, locals.var_t7__blk309_dn12, locals.var_t7__blk309_dn17,)
    } else {
        (locals.var_t3__blk306, locals.var_t3__blk306_dn0, locals.var_t3__blk306_dn2, locals.var_t3__blk306_dn6, locals.var_t3__blk306_dn7, locals.var_t3__blk306_dn10, locals.var_t3__blk306_dn11, locals.var_t3__blk306_dn12, locals.var_t3__blk306_dn17,)
    }
};
        locals.var_t3__blk306 = assign12000_e13629;
        locals.var_t3__blk306_dn0 = assign12000_e13629_d_n0;
        locals.var_t3__blk306_dn2 = assign12000_e13629_d_n2;
        locals.var_t3__blk306_dn6 = assign12000_e13629_d_n6;
        locals.var_t3__blk306_dn7 = assign12000_e13629_d_n7;
        locals.var_t3__blk306_dn10 = assign12000_e13629_d_n10;
        locals.var_t3__blk306_dn11 = assign12000_e13629_d_n11;
        locals.var_t3__blk306_dn12 = assign12000_e13629_d_n12;
        locals.var_t3__blk306_dn17 = assign12000_e13629_d_n17;

        let (assign12010_e13642, assign12010_e13642_d_n0, assign12010_e13642_d_n2, assign12010_e13642_d_n6, assign12010_e13642_d_n7, assign12010_e13642_d_n10, assign12010_e13642_d_n11, assign12010_e13642_d_n12, assign12010_e13642_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign12010_e13637: f64 = (1.0 / locals.var_ddlte);
        let assign12010_e13639: f64 = (assign12010_e13637 - 1.0);
        let assign12010_e13640: f64 = (locals.var_t3__blk306).powf(assign12010_e13639);
        (assign12010_e13640, if 0.0 == 0.0 && ((assign12010_e13639) as f64).is_finite() && ((assign12010_e13639) as f64).fract() == 0.0 { if assign12010_e13639 == 0.0 { 0.0 } else { (assign12010_e13639 * ((locals.var_t3__blk306).powf(assign12010_e13639 - 1.0) * locals.var_t3__blk306_dn0)) } } else { (assign12010_e13640 * (assign12010_e13639 * (locals.var_t3__blk306_dn0 / locals.var_t3__blk306))) }, if 0.0 == 0.0 && ((assign12010_e13639) as f64).is_finite() && ((assign12010_e13639) as f64).fract() == 0.0 { if assign12010_e13639 == 0.0 { 0.0 } else { (assign12010_e13639 * ((locals.var_t3__blk306).powf(assign12010_e13639 - 1.0) * locals.var_t3__blk306_dn2)) } } else { (assign12010_e13640 * (assign12010_e13639 * (locals.var_t3__blk306_dn2 / locals.var_t3__blk306))) }, if 0.0 == 0.0 && ((assign12010_e13639) as f64).is_finite() && ((assign12010_e13639) as f64).fract() == 0.0 { if assign12010_e13639 == 0.0 { 0.0 } else { (assign12010_e13639 * ((locals.var_t3__blk306).powf(assign12010_e13639 - 1.0) * locals.var_t3__blk306_dn6)) } } else { (assign12010_e13640 * (assign12010_e13639 * (locals.var_t3__blk306_dn6 / locals.var_t3__blk306))) }, if 0.0 == 0.0 && ((assign12010_e13639) as f64).is_finite() && ((assign12010_e13639) as f64).fract() == 0.0 { if assign12010_e13639 == 0.0 { 0.0 } else { (assign12010_e13639 * ((locals.var_t3__blk306).powf(assign12010_e13639 - 1.0) * locals.var_t3__blk306_dn7)) } } else { (assign12010_e13640 * (assign12010_e13639 * (locals.var_t3__blk306_dn7 / locals.var_t3__blk306))) }, if 0.0 == 0.0 && ((assign12010_e13639) as f64).is_finite() && ((assign12010_e13639) as f64).fract() == 0.0 { if assign12010_e13639 == 0.0 { 0.0 } else { (assign12010_e13639 * ((locals.var_t3__blk306).powf(assign12010_e13639 - 1.0) * locals.var_t3__blk306_dn10)) } } else { (assign12010_e13640 * (assign12010_e13639 * (locals.var_t3__blk306_dn10 / locals.var_t3__blk306))) }, if 0.0 == 0.0 && ((assign12010_e13639) as f64).is_finite() && ((assign12010_e13639) as f64).fract() == 0.0 { if assign12010_e13639 == 0.0 { 0.0 } else { (assign12010_e13639 * ((locals.var_t3__blk306).powf(assign12010_e13639 - 1.0) * locals.var_t3__blk306_dn11)) } } else { (assign12010_e13640 * (assign12010_e13639 * (locals.var_t3__blk306_dn11 / locals.var_t3__blk306))) }, if 0.0 == 0.0 && ((assign12010_e13639) as f64).is_finite() && ((assign12010_e13639) as f64).fract() == 0.0 { if assign12010_e13639 == 0.0 { 0.0 } else { (assign12010_e13639 * ((locals.var_t3__blk306).powf(assign12010_e13639 - 1.0) * locals.var_t3__blk306_dn12)) } } else { (assign12010_e13640 * (assign12010_e13639 * (locals.var_t3__blk306_dn12 / locals.var_t3__blk306))) }, if 0.0 == 0.0 && ((assign12010_e13639) as f64).is_finite() && ((assign12010_e13639) as f64).fract() == 0.0 { if assign12010_e13639 == 0.0 { 0.0 } else { (assign12010_e13639 * ((locals.var_t3__blk306).powf(assign12010_e13639 - 1.0) * locals.var_t3__blk306_dn17)) } } else { (assign12010_e13640 * (assign12010_e13639 * (locals.var_t3__blk306_dn17 / locals.var_t3__blk306))) },)
    } else {
        (locals.var_t4__blk307, locals.var_t4__blk307_dn0, locals.var_t4__blk307_dn2, locals.var_t4__blk307_dn6, locals.var_t4__blk307_dn7, locals.var_t4__blk307_dn10, locals.var_t4__blk307_dn11, locals.var_t4__blk307_dn12, locals.var_t4__blk307_dn17,)
    }
};
        locals.var_t4__blk307 = assign12010_e13642;
        locals.var_t4__blk307_dn0 = assign12010_e13642_d_n0;
        locals.var_t4__blk307_dn2 = assign12010_e13642_d_n2;
        locals.var_t4__blk307_dn6 = assign12010_e13642_d_n6;
        locals.var_t4__blk307_dn7 = assign12010_e13642_d_n7;
        locals.var_t4__blk307_dn10 = assign12010_e13642_d_n10;
        locals.var_t4__blk307_dn11 = assign12010_e13642_d_n11;
        locals.var_t4__blk307_dn12 = assign12010_e13642_d_n12;
        locals.var_t4__blk307_dn17 = assign12010_e13642_d_n17;

        let (assign12020_e13651, assign12020_e13651_d_n0, assign12020_e13651_d_n2, assign12020_e13651_d_n6, assign12020_e13651_d_n7, assign12020_e13651_d_n10, assign12020_e13651_d_n11, assign12020_e13651_d_n12, assign12020_e13651_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign12020_e13649: f64 = (locals.var_t4__blk307 * locals.var_t3__blk306);
        (assign12020_e13649, ((locals.var_t4__blk307_dn0 * locals.var_t3__blk306) + (locals.var_t4__blk307 * locals.var_t3__blk306_dn0)), ((locals.var_t4__blk307_dn2 * locals.var_t3__blk306) + (locals.var_t4__blk307 * locals.var_t3__blk306_dn2)), ((locals.var_t4__blk307_dn6 * locals.var_t3__blk306) + (locals.var_t4__blk307 * locals.var_t3__blk306_dn6)), ((locals.var_t4__blk307_dn7 * locals.var_t3__blk306) + (locals.var_t4__blk307 * locals.var_t3__blk306_dn7)), ((locals.var_t4__blk307_dn10 * locals.var_t3__blk306) + (locals.var_t4__blk307 * locals.var_t3__blk306_dn10)), ((locals.var_t4__blk307_dn11 * locals.var_t3__blk306) + (locals.var_t4__blk307 * locals.var_t3__blk306_dn11)), ((locals.var_t4__blk307_dn12 * locals.var_t3__blk306) + (locals.var_t4__blk307 * locals.var_t3__blk306_dn12)), ((locals.var_t4__blk307_dn17 * locals.var_t3__blk306) + (locals.var_t4__blk307 * locals.var_t3__blk306_dn17)),)
    } else {
        (locals.var_t6__blk308, locals.var_t6__blk308_dn0, locals.var_t6__blk308_dn2, locals.var_t6__blk308_dn6, locals.var_t6__blk308_dn7, locals.var_t6__blk308_dn10, locals.var_t6__blk308_dn11, locals.var_t6__blk308_dn12, locals.var_t6__blk308_dn17,)
    }
};
        locals.var_t6__blk308 = assign12020_e13651;
        locals.var_t6__blk308_dn0 = assign12020_e13651_d_n0;
        locals.var_t6__blk308_dn2 = assign12020_e13651_d_n2;
        locals.var_t6__blk308_dn6 = assign12020_e13651_d_n6;
        locals.var_t6__blk308_dn7 = assign12020_e13651_d_n7;
        locals.var_t6__blk308_dn10 = assign12020_e13651_d_n10;
        locals.var_t6__blk308_dn11 = assign12020_e13651_d_n11;
        locals.var_t6__blk308_dn12 = assign12020_e13651_d_n12;
        locals.var_t6__blk308_dn17 = assign12020_e13651_d_n17;

        let (assign12030_e13660, assign12030_e13660_d_n0, assign12030_e13660_d_n2, assign12030_e13660_d_n6, assign12030_e13660_d_n7, assign12030_e13660_d_n10, assign12030_e13660_d_n11, assign12030_e13660_d_n12, assign12030_e13660_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign12030_e13658: f64 = (locals.var_vds / locals.var_t6__blk308);
        (assign12030_e13658, (((locals.var_vds_dn0 * locals.var_t6__blk308) - (locals.var_vds * locals.var_t6__blk308_dn0)) / (locals.var_t6__blk308 * locals.var_t6__blk308)), (((locals.var_vds_dn2 * locals.var_t6__blk308) - (locals.var_vds * locals.var_t6__blk308_dn2)) / (locals.var_t6__blk308 * locals.var_t6__blk308)), (((locals.var_vds_dn6 * locals.var_t6__blk308) - (locals.var_vds * locals.var_t6__blk308_dn6)) / (locals.var_t6__blk308 * locals.var_t6__blk308)), (((locals.var_vds_dn7 * locals.var_t6__blk308) - (locals.var_vds * locals.var_t6__blk308_dn7)) / (locals.var_t6__blk308 * locals.var_t6__blk308)), (((locals.var_vds_dn10 * locals.var_t6__blk308) - (locals.var_vds * locals.var_t6__blk308_dn10)) / (locals.var_t6__blk308 * locals.var_t6__blk308)), (((locals.var_vds_dn11 * locals.var_t6__blk308) - (locals.var_vds * locals.var_t6__blk308_dn11)) / (locals.var_t6__blk308 * locals.var_t6__blk308)), (((locals.var_vds_dn12 * locals.var_t6__blk308) - (locals.var_vds * locals.var_t6__blk308_dn12)) / (locals.var_t6__blk308 * locals.var_t6__blk308)), (((locals.var_vds_dn17 * locals.var_t6__blk308) - (locals.var_vds * locals.var_t6__blk308_dn17)) / (locals.var_t6__blk308 * locals.var_t6__blk308)),)
    } else {
        (locals.var_vdseff, locals.var_vdseff_dn0, locals.var_vdseff_dn2, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn10, locals.var_vdseff_dn11, locals.var_vdseff_dn12, locals.var_vdseff_dn17,)
    }
};
        locals.var_vdseff = assign12030_e13660;
        locals.var_vdseff_dn0 = assign12030_e13660_d_n0;
        locals.var_vdseff_dn2 = assign12030_e13660_d_n2;
        locals.var_vdseff_dn6 = assign12030_e13660_d_n6;
        locals.var_vdseff_dn7 = assign12030_e13660_d_n7;
        locals.var_vdseff_dn10 = assign12030_e13660_d_n10;
        locals.var_vdseff_dn11 = assign12030_e13660_d_n11;
        locals.var_vdseff_dn12 = assign12030_e13660_d_n12;
        locals.var_vdseff_dn17 = assign12030_e13660_d_n17;

        let (assign12040_e13667, assign12040_e13667_d_n0, assign12040_e13667_d_n2, assign12040_e13667_d_n6, assign12040_e13667_d_n7, assign12040_e13667_d_n10, assign12040_e13667_d_n11, assign12040_e13667_d_n12, assign12040_e13667_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        (locals.var_vdseff, locals.var_vdseff_dn0, locals.var_vdseff_dn2, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn10, locals.var_vdseff_dn11, locals.var_vdseff_dn12, locals.var_vdseff_dn17,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    }
};
        locals.var_vds = assign12040_e13667;
        locals.var_vds_dn0 = assign12040_e13667_d_n0;
        locals.var_vds_dn2 = assign12040_e13667_d_n2;
        locals.var_vds_dn6 = assign12040_e13667_d_n6;
        locals.var_vds_dn7 = assign12040_e13667_d_n7;
        locals.var_vds_dn10 = assign12040_e13667_d_n10;
        locals.var_vds_dn11 = assign12040_e13667_d_n11;
        locals.var_vds_dn12 = assign12040_e13667_d_n12;
        locals.var_vds_dn17 = assign12040_e13667_d_n17;

        let assign12050_e13670: f64 = if locals.var_vds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard318 = assign12050_e13670;

        let (assign12060_e13679, assign12060_e13679_d_n0, assign12060_e13679_d_n2, assign12060_e13679_d_n6, assign12060_e13679_d_n7, assign12060_e13679_d_n10, assign12060_e13679_d_n11, assign12060_e13679_d_n12, assign12060_e13679_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 != 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    }
};
        locals.var_psl = assign12060_e13679;
        locals.var_psl_dn0 = assign12060_e13679_d_n0;
        locals.var_psl_dn2 = assign12060_e13679_d_n2;
        locals.var_psl_dn6 = assign12060_e13679_d_n6;
        locals.var_psl_dn7 = assign12060_e13679_d_n7;
        locals.var_psl_dn10 = assign12060_e13679_d_n10;
        locals.var_psl_dn11 = assign12060_e13679_d_n11;
        locals.var_psl_dn12 = assign12060_e13679_d_n12;
        locals.var_psl_dn17 = assign12060_e13679_d_n17;

        let (assign12070_e13690, assign12070_e13690_d_n0, assign12070_e13690_d_n2, assign12070_e13690_d_n6, assign12070_e13690_d_n7, assign12070_e13690_d_n10, assign12070_e13690_d_n11, assign12070_e13690_d_n12, assign12070_e13690_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign12070_e13688: f64 = (locals.var_psl - locals.var_ps0);
        (assign12070_e13688, (locals.var_psl_dn0 - locals.var_ps0_dn0), (locals.var_psl_dn2 - locals.var_ps0_dn2), (locals.var_psl_dn6 - locals.var_ps0_dn6), (locals.var_psl_dn7 - locals.var_ps0_dn7), (locals.var_psl_dn10 - locals.var_ps0_dn10), (locals.var_psl_dn11 - locals.var_ps0_dn11), (locals.var_psl_dn12 - locals.var_ps0_dn12), (locals.var_psl_dn17 - locals.var_ps0_dn17),)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn12, locals.var_pds_dn17,)
    }
};
        locals.var_pds = assign12070_e13690;
        locals.var_pds_dn0 = assign12070_e13690_d_n0;
        locals.var_pds_dn2 = assign12070_e13690_d_n2;
        locals.var_pds_dn6 = assign12070_e13690_d_n6;
        locals.var_pds_dn7 = assign12070_e13690_d_n7;
        locals.var_pds_dn10 = assign12070_e13690_d_n10;
        locals.var_pds_dn11 = assign12070_e13690_d_n11;
        locals.var_pds_dn12 = assign12070_e13690_d_n12;
        locals.var_pds_dn17 = assign12070_e13690_d_n17;

        let (assign12080_e13699, assign12080_e13699_d_n0, assign12080_e13699_d_n2, assign12080_e13699_d_n6, assign12080_e13699_d_n7, assign12080_e13699_d_n10, assign12080_e13699_d_n11, assign12080_e13699_d_n12, assign12080_e13699_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 != 0.0)) {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
        locals.var_phi_sl_soi = assign12080_e13699;
        locals.var_phi_sl_soi_dn0 = assign12080_e13699_d_n0;
        locals.var_phi_sl_soi_dn2 = assign12080_e13699_d_n2;
        locals.var_phi_sl_soi_dn6 = assign12080_e13699_d_n6;
        locals.var_phi_sl_soi_dn7 = assign12080_e13699_d_n7;
        locals.var_phi_sl_soi_dn10 = assign12080_e13699_d_n10;
        locals.var_phi_sl_soi_dn11 = assign12080_e13699_d_n11;
        locals.var_phi_sl_soi_dn12 = assign12080_e13699_d_n12;
        locals.var_phi_sl_soi_dn17 = assign12080_e13699_d_n17;

        let (assign12090_e13708, assign12090_e13708_d_n0, assign12090_e13708_d_n2, assign12090_e13708_d_n6, assign12090_e13708_d_n7, assign12090_e13708_d_n10, assign12090_e13708_d_n11, assign12090_e13708_d_n12, assign12090_e13708_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 != 0.0)) {
        (locals.var_phi_b0_soi, locals.var_phi_b0_soi_dn0, locals.var_phi_b0_soi_dn2, locals.var_phi_b0_soi_dn6, locals.var_phi_b0_soi_dn7, locals.var_phi_b0_soi_dn10, locals.var_phi_b0_soi_dn11, locals.var_phi_b0_soi_dn12, locals.var_phi_b0_soi_dn17,)
    } else {
        (locals.var_phi_bl_soi, locals.var_phi_bl_soi_dn0, locals.var_phi_bl_soi_dn2, locals.var_phi_bl_soi_dn6, locals.var_phi_bl_soi_dn7, locals.var_phi_bl_soi_dn10, locals.var_phi_bl_soi_dn11, locals.var_phi_bl_soi_dn12, locals.var_phi_bl_soi_dn17,)
    }
};
        locals.var_phi_bl_soi = assign12090_e13708;
        locals.var_phi_bl_soi_dn0 = assign12090_e13708_d_n0;
        locals.var_phi_bl_soi_dn2 = assign12090_e13708_d_n2;
        locals.var_phi_bl_soi_dn6 = assign12090_e13708_d_n6;
        locals.var_phi_bl_soi_dn7 = assign12090_e13708_d_n7;
        locals.var_phi_bl_soi_dn10 = assign12090_e13708_d_n10;
        locals.var_phi_bl_soi_dn11 = assign12090_e13708_d_n11;
        locals.var_phi_bl_soi_dn12 = assign12090_e13708_d_n12;
        locals.var_phi_bl_soi_dn17 = assign12090_e13708_d_n17;

        let (assign12100_e13717, assign12100_e13717_d_n0, assign12100_e13717_d_n2, assign12100_e13717_d_n6, assign12100_e13717_d_n7, assign12100_e13717_d_n10, assign12100_e13717_d_n11, assign12100_e13717_d_n12, assign12100_e13717_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 != 0.0)) {
        (locals.var_phi_s0_bulk, locals.var_phi_s0_bulk_dn0, locals.var_phi_s0_bulk_dn2, locals.var_phi_s0_bulk_dn6, locals.var_phi_s0_bulk_dn7, locals.var_phi_s0_bulk_dn10, locals.var_phi_s0_bulk_dn11, locals.var_phi_s0_bulk_dn12, locals.var_phi_s0_bulk_dn17,)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
        locals.var_phi_sl_bulk = assign12100_e13717;
        locals.var_phi_sl_bulk_dn0 = assign12100_e13717_d_n0;
        locals.var_phi_sl_bulk_dn2 = assign12100_e13717_d_n2;
        locals.var_phi_sl_bulk_dn6 = assign12100_e13717_d_n6;
        locals.var_phi_sl_bulk_dn7 = assign12100_e13717_d_n7;
        locals.var_phi_sl_bulk_dn10 = assign12100_e13717_d_n10;
        locals.var_phi_sl_bulk_dn11 = assign12100_e13717_d_n11;
        locals.var_phi_sl_bulk_dn12 = assign12100_e13717_d_n12;
        locals.var_phi_sl_bulk_dn17 = assign12100_e13717_d_n17;

        let (assign12110_e13726,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign12110_e13726;

        let assign12130_e13738: f64 = if locals.var_flg_pprv >= 1.0 { 1.0 } else { 0.0 };
        locals.var_guard319 = assign12130_e13738;

        let (assign12140_e13750, assign12140_e13750_d_n0, assign12140_e13750_d_n2, assign12140_e13750_d_n6, assign12140_e13750_d_n7, assign12140_e13750_d_n10, assign12140_e13750_d_n11, assign12140_e13750_d_n12, assign12140_e13750_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 != 0.0)) {
        (locals.var_pssl_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
        locals.var_phi_sl_soi = assign12140_e13750;
        locals.var_phi_sl_soi_dn0 = assign12140_e13750_d_n0;
        locals.var_phi_sl_soi_dn2 = assign12140_e13750_d_n2;
        locals.var_phi_sl_soi_dn6 = assign12140_e13750_d_n6;
        locals.var_phi_sl_soi_dn7 = assign12140_e13750_d_n7;
        locals.var_phi_sl_soi_dn10 = assign12140_e13750_d_n10;
        locals.var_phi_sl_soi_dn11 = assign12140_e13750_d_n11;
        locals.var_phi_sl_soi_dn12 = assign12140_e13750_d_n12;
        locals.var_phi_sl_soi_dn17 = assign12140_e13750_d_n17;

        let (assign12150_e13762, assign12150_e13762_d_n0, assign12150_e13762_d_n2, assign12150_e13762_d_n6, assign12150_e13762_d_n7, assign12150_e13762_d_n10, assign12150_e13762_d_n11, assign12150_e13762_d_n12, assign12150_e13762_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 != 0.0)) {
        (locals.var_pbsl_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_bl_soi, locals.var_phi_bl_soi_dn0, locals.var_phi_bl_soi_dn2, locals.var_phi_bl_soi_dn6, locals.var_phi_bl_soi_dn7, locals.var_phi_bl_soi_dn10, locals.var_phi_bl_soi_dn11, locals.var_phi_bl_soi_dn12, locals.var_phi_bl_soi_dn17,)
    }
};
        locals.var_phi_bl_soi = assign12150_e13762;
        locals.var_phi_bl_soi_dn0 = assign12150_e13762_d_n0;
        locals.var_phi_bl_soi_dn2 = assign12150_e13762_d_n2;
        locals.var_phi_bl_soi_dn6 = assign12150_e13762_d_n6;
        locals.var_phi_bl_soi_dn7 = assign12150_e13762_d_n7;
        locals.var_phi_bl_soi_dn10 = assign12150_e13762_d_n10;
        locals.var_phi_bl_soi_dn11 = assign12150_e13762_d_n11;
        locals.var_phi_bl_soi_dn12 = assign12150_e13762_d_n12;
        locals.var_phi_bl_soi_dn17 = assign12150_e13762_d_n17;

        let (assign12160_e13774, assign12160_e13774_d_n0, assign12160_e13774_d_n2, assign12160_e13774_d_n6, assign12160_e13774_d_n7, assign12160_e13774_d_n10, assign12160_e13774_d_n11, assign12160_e13774_d_n12, assign12160_e13774_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 != 0.0)) {
        (locals.var_psbl_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
        locals.var_phi_sl_bulk = assign12160_e13774;
        locals.var_phi_sl_bulk_dn0 = assign12160_e13774_d_n0;
        locals.var_phi_sl_bulk_dn2 = assign12160_e13774_d_n2;
        locals.var_phi_sl_bulk_dn6 = assign12160_e13774_d_n6;
        locals.var_phi_sl_bulk_dn7 = assign12160_e13774_d_n7;
        locals.var_phi_sl_bulk_dn10 = assign12160_e13774_d_n10;
        locals.var_phi_sl_bulk_dn11 = assign12160_e13774_d_n11;
        locals.var_phi_sl_bulk_dn12 = assign12160_e13774_d_n12;
        locals.var_phi_sl_bulk_dn17 = assign12160_e13774_d_n17;

    }

    pub(super) fn stamp_transient_block_35(
        locals: &mut StampLocals,
    ) {
        let (assign12180_e13813, assign12180_e13813_d_n0, assign12180_e13813_d_n2, assign12180_e13813_d_n6, assign12180_e13813_d_n7, assign12180_e13813_d_n10, assign12180_e13813_d_n11, assign12180_e13813_d_n12, assign12180_e13813_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) {
        let assign12180_e13804: f64 = (locals.var_psl_lim - locals.var_phi_s0_soi);
        let (assign12180_e13811, assign12180_e13811_d_n0, assign12180_e13811_d_n2, assign12180_e13811_d_n6, assign12180_e13811_d_n7, assign12180_e13811_d_n10, assign12180_e13811_d_n11, assign12180_e13811_d_n12, assign12180_e13811_d_n17,) = {
            if (assign12180_e13804 >= 0.0) {
                let assign12180_e13809: f64 = (locals.var_psl_lim - locals.var_phi_s0_soi);
                (assign12180_e13809, (locals.var_psl_lim_dn0 - locals.var_phi_s0_soi_dn0), (locals.var_psl_lim_dn2 - locals.var_phi_s0_soi_dn2), (locals.var_psl_lim_dn6 - locals.var_phi_s0_soi_dn6), (locals.var_psl_lim_dn7 - locals.var_phi_s0_soi_dn7), (locals.var_psl_lim_dn10 - locals.var_phi_s0_soi_dn10), (locals.var_psl_lim_dn11 - locals.var_phi_s0_soi_dn11), (locals.var_psl_lim_dn12 - locals.var_phi_s0_soi_dn12), (locals.var_psl_lim_dn17 - locals.var_phi_s0_soi_dn17),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign12180_e13811, assign12180_e13811_d_n0, assign12180_e13811_d_n2, assign12180_e13811_d_n6, assign12180_e13811_d_n7, assign12180_e13811_d_n10, assign12180_e13811_d_n11, assign12180_e13811_d_n12, assign12180_e13811_d_n17,)
    } else {
        (locals.var_pds_max, locals.var_pds_max_dn0, locals.var_pds_max_dn2, locals.var_pds_max_dn6, locals.var_pds_max_dn7, locals.var_pds_max_dn10, locals.var_pds_max_dn11, locals.var_pds_max_dn12, locals.var_pds_max_dn17,)
    }
};
        locals.var_pds_max = assign12180_e13813;
        locals.var_pds_max_dn0 = assign12180_e13813_d_n0;
        locals.var_pds_max_dn2 = assign12180_e13813_d_n2;
        locals.var_pds_max_dn6 = assign12180_e13813_d_n6;
        locals.var_pds_max_dn7 = assign12180_e13813_d_n7;
        locals.var_pds_max_dn10 = assign12180_e13813_d_n10;
        locals.var_pds_max_dn11 = assign12180_e13813_d_n11;
        locals.var_pds_max_dn12 = assign12180_e13813_d_n12;
        locals.var_pds_max_dn17 = assign12180_e13813_d_n17;

        let (assign12190_e13834, assign12190_e13834_d_n0, assign12190_e13834_d_n2, assign12190_e13834_d_n6, assign12190_e13834_d_n7, assign12190_e13834_d_n10, assign12190_e13834_d_n11, assign12190_e13834_d_n12, assign12190_e13834_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) {
        let assign12190_e13826: f64 = (1.0 + 0.3);
        let assign12190_e13828: f64 = (assign12190_e13826 * locals.var_pds_max);
        let assign12190_e13830: f64 = (assign12190_e13828 - locals.var_vds);
        let assign12190_e13832: f64 = (assign12190_e13830 - 0.03);
        (assign12190_e13832, ((assign12190_e13826 * locals.var_pds_max_dn0) - locals.var_vds_dn0), ((assign12190_e13826 * locals.var_pds_max_dn2) - locals.var_vds_dn2), ((assign12190_e13826 * locals.var_pds_max_dn6) - locals.var_vds_dn6), ((assign12190_e13826 * locals.var_pds_max_dn7) - locals.var_vds_dn7), ((assign12190_e13826 * locals.var_pds_max_dn10) - locals.var_vds_dn10), ((assign12190_e13826 * locals.var_pds_max_dn11) - locals.var_vds_dn11), ((assign12190_e13826 * locals.var_pds_max_dn12) - locals.var_vds_dn12), ((assign12190_e13826 * locals.var_pds_max_dn17) - locals.var_vds_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign12190_e13834;
        locals.var_tmf1_dn0 = assign12190_e13834_d_n0;
        locals.var_tmf1_dn2 = assign12190_e13834_d_n2;
        locals.var_tmf1_dn6 = assign12190_e13834_d_n6;
        locals.var_tmf1_dn7 = assign12190_e13834_d_n7;
        locals.var_tmf1_dn10 = assign12190_e13834_d_n10;
        locals.var_tmf1_dn11 = assign12190_e13834_d_n11;
        locals.var_tmf1_dn12 = assign12190_e13834_d_n12;
        locals.var_tmf1_dn17 = assign12190_e13834_d_n17;

        let (assign12200_e13855, assign12200_e13855_d_n0, assign12200_e13855_d_n2, assign12200_e13855_d_n6, assign12200_e13855_d_n7, assign12200_e13855_d_n10, assign12200_e13855_d_n11, assign12200_e13855_d_n12, assign12200_e13855_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) {
        let assign12200_e13848: f64 = (1.0 + 0.3);
        let assign12200_e13850: f64 = (assign12200_e13848 * locals.var_pds_max);
        let assign12200_e13851: f64 = (4.0 * assign12200_e13850);
        let assign12200_e13853: f64 = (assign12200_e13851 * 0.03);
        (assign12200_e13853, ((4.0 * (assign12200_e13848 * locals.var_pds_max_dn0)) * 0.03), ((4.0 * (assign12200_e13848 * locals.var_pds_max_dn2)) * 0.03), ((4.0 * (assign12200_e13848 * locals.var_pds_max_dn6)) * 0.03), ((4.0 * (assign12200_e13848 * locals.var_pds_max_dn7)) * 0.03), ((4.0 * (assign12200_e13848 * locals.var_pds_max_dn10)) * 0.03), ((4.0 * (assign12200_e13848 * locals.var_pds_max_dn11)) * 0.03), ((4.0 * (assign12200_e13848 * locals.var_pds_max_dn12)) * 0.03), ((4.0 * (assign12200_e13848 * locals.var_pds_max_dn17)) * 0.03),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign12200_e13855;
        locals.var_tmf2_dn0 = assign12200_e13855_d_n0;
        locals.var_tmf2_dn2 = assign12200_e13855_d_n2;
        locals.var_tmf2_dn6 = assign12200_e13855_d_n6;
        locals.var_tmf2_dn7 = assign12200_e13855_d_n7;
        locals.var_tmf2_dn10 = assign12200_e13855_d_n10;
        locals.var_tmf2_dn11 = assign12200_e13855_d_n11;
        locals.var_tmf2_dn12 = assign12200_e13855_d_n12;
        locals.var_tmf2_dn17 = assign12200_e13855_d_n17;

        let (assign12210_e13874, assign12210_e13874_d_n0, assign12210_e13874_d_n2, assign12210_e13874_d_n6, assign12210_e13874_d_n7, assign12210_e13874_d_n10, assign12210_e13874_d_n11, assign12210_e13874_d_n12, assign12210_e13874_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) {
        let (assign12210_e13872, assign12210_e13872_d_n0, assign12210_e13872_d_n2, assign12210_e13872_d_n6, assign12210_e13872_d_n7, assign12210_e13872_d_n10, assign12210_e13872_d_n11, assign12210_e13872_d_n12, assign12210_e13872_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign12210_e13871: f64 = (-locals.var_tmf2);
                (assign12210_e13871, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign12210_e13872, assign12210_e13872_d_n0, assign12210_e13872_d_n2, assign12210_e13872_d_n6, assign12210_e13872_d_n7, assign12210_e13872_d_n10, assign12210_e13872_d_n11, assign12210_e13872_d_n12, assign12210_e13872_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign12210_e13874;
        locals.var_tmf2_dn0 = assign12210_e13874_d_n0;
        locals.var_tmf2_dn2 = assign12210_e13874_d_n2;
        locals.var_tmf2_dn6 = assign12210_e13874_d_n6;
        locals.var_tmf2_dn7 = assign12210_e13874_d_n7;
        locals.var_tmf2_dn10 = assign12210_e13874_d_n10;
        locals.var_tmf2_dn11 = assign12210_e13874_d_n11;
        locals.var_tmf2_dn12 = assign12210_e13874_d_n12;
        locals.var_tmf2_dn17 = assign12210_e13874_d_n17;

        let (assign12220_e13892, assign12220_e13892_d_n0, assign12220_e13892_d_n2, assign12220_e13892_d_n6, assign12220_e13892_d_n7, assign12220_e13892_d_n10, assign12220_e13892_d_n11, assign12220_e13892_d_n12, assign12220_e13892_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) {
        let assign12220_e13887: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign12220_e13889: f64 = (assign12220_e13887 + locals.var_tmf2);
        let assign12220_e13890: f64 = (assign12220_e13889).sqrt();
        (assign12220_e13890, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign12220_e13890)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign12220_e13890)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign12220_e13890)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign12220_e13890)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign12220_e13890)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign12220_e13890)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign12220_e13890)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign12220_e13890)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign12220_e13892;
        locals.var_tmf2_dn0 = assign12220_e13892_d_n0;
        locals.var_tmf2_dn2 = assign12220_e13892_d_n2;
        locals.var_tmf2_dn6 = assign12220_e13892_d_n6;
        locals.var_tmf2_dn7 = assign12220_e13892_d_n7;
        locals.var_tmf2_dn10 = assign12220_e13892_d_n10;
        locals.var_tmf2_dn11 = assign12220_e13892_d_n11;
        locals.var_tmf2_dn12 = assign12220_e13892_d_n12;
        locals.var_tmf2_dn17 = assign12220_e13892_d_n17;

        let (assign12230_e13915, assign12230_e13915_d_n0, assign12230_e13915_d_n2, assign12230_e13915_d_n6, assign12230_e13915_d_n7, assign12230_e13915_d_n10, assign12230_e13915_d_n11, assign12230_e13915_d_n12, assign12230_e13915_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) {
        let assign12230_e13905: f64 = (1.0 + 0.3);
        let assign12230_e13907: f64 = (assign12230_e13905 * locals.var_pds_max);
        let assign12230_e13911: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign12230_e13912: f64 = (0.5 * assign12230_e13911);
        let assign12230_e13913: f64 = (assign12230_e13907 - assign12230_e13912);
        (assign12230_e13913, ((assign12230_e13905 * locals.var_pds_max_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((assign12230_e13905 * locals.var_pds_max_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((assign12230_e13905 * locals.var_pds_max_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((assign12230_e13905 * locals.var_pds_max_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((assign12230_e13905 * locals.var_pds_max_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((assign12230_e13905 * locals.var_pds_max_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((assign12230_e13905 * locals.var_pds_max_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((assign12230_e13905 * locals.var_pds_max_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign12230_e13915;
        locals.var_pds_ini_dn0 = assign12230_e13915_d_n0;
        locals.var_pds_ini_dn2 = assign12230_e13915_d_n2;
        locals.var_pds_ini_dn6 = assign12230_e13915_d_n6;
        locals.var_pds_ini_dn7 = assign12230_e13915_d_n7;
        locals.var_pds_ini_dn10 = assign12230_e13915_d_n10;
        locals.var_pds_ini_dn11 = assign12230_e13915_d_n11;
        locals.var_pds_ini_dn12 = assign12230_e13915_d_n12;
        locals.var_pds_ini_dn17 = assign12230_e13915_d_n17;

        let (assign12240_e13933, assign12240_e13933_d_n0, assign12240_e13933_d_n2, assign12240_e13933_d_n6, assign12240_e13933_d_n7, assign12240_e13933_d_n10, assign12240_e13933_d_n11, assign12240_e13933_d_n12, assign12240_e13933_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) {
        let (assign12240_e13931, assign12240_e13931_d_n0, assign12240_e13931_d_n2, assign12240_e13931_d_n6, assign12240_e13931_d_n7, assign12240_e13931_d_n10, assign12240_e13931_d_n11, assign12240_e13931_d_n12, assign12240_e13931_d_n17,) = {
            if (locals.var_pds_ini <= locals.var_pds_max) {
                (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
            } else {
                (locals.var_pds_max, locals.var_pds_max_dn0, locals.var_pds_max_dn2, locals.var_pds_max_dn6, locals.var_pds_max_dn7, locals.var_pds_max_dn10, locals.var_pds_max_dn11, locals.var_pds_max_dn12, locals.var_pds_max_dn17,)
            }
        };
        (assign12240_e13931, assign12240_e13931_d_n0, assign12240_e13931_d_n2, assign12240_e13931_d_n6, assign12240_e13931_d_n7, assign12240_e13931_d_n10, assign12240_e13931_d_n11, assign12240_e13931_d_n12, assign12240_e13931_d_n17,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign12240_e13933;
        locals.var_pds_ini_dn0 = assign12240_e13933_d_n0;
        locals.var_pds_ini_dn2 = assign12240_e13933_d_n2;
        locals.var_pds_ini_dn6 = assign12240_e13933_d_n6;
        locals.var_pds_ini_dn7 = assign12240_e13933_d_n7;
        locals.var_pds_ini_dn10 = assign12240_e13933_d_n10;
        locals.var_pds_ini_dn11 = assign12240_e13933_d_n11;
        locals.var_pds_ini_dn12 = assign12240_e13933_d_n12;
        locals.var_pds_ini_dn17 = assign12240_e13933_d_n17;

        let assign12250_e13936: f64 = if locals.var_pds_ini < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard320 = assign12250_e13936;

        let (assign12260_e13951, assign12260_e13951_d_n0, assign12260_e13951_d_n2, assign12260_e13951_d_n6, assign12260_e13951_d_n7, assign12260_e13951_d_n10, assign12260_e13951_d_n11, assign12260_e13951_d_n12, assign12260_e13951_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard320 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign12260_e13951;
        locals.var_pds_ini_dn0 = assign12260_e13951_d_n0;
        locals.var_pds_ini_dn2 = assign12260_e13951_d_n2;
        locals.var_pds_ini_dn6 = assign12260_e13951_d_n6;
        locals.var_pds_ini_dn7 = assign12260_e13951_d_n7;
        locals.var_pds_ini_dn10 = assign12260_e13951_d_n10;
        locals.var_pds_ini_dn11 = assign12260_e13951_d_n11;
        locals.var_pds_ini_dn12 = assign12260_e13951_d_n12;
        locals.var_pds_ini_dn17 = assign12260_e13951_d_n17;

        let assign12270_e13954: f64 = if locals.var_pds_ini > locals.var_vds { 1.0 } else { 0.0 };
        locals.var_guard321 = assign12270_e13954;

        let (assign12280_e13972, assign12280_e13972_d_n0, assign12280_e13972_d_n2, assign12280_e13972_d_n6, assign12280_e13972_d_n7, assign12280_e13972_d_n10, assign12280_e13972_d_n11, assign12280_e13972_d_n12, assign12280_e13972_d_n17,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard320 == 0.0)) && (locals.var_guard321 != 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    }
};
        locals.var_pds_ini = assign12280_e13972;
        locals.var_pds_ini_dn0 = assign12280_e13972_d_n0;
        locals.var_pds_ini_dn2 = assign12280_e13972_d_n2;
        locals.var_pds_ini_dn6 = assign12280_e13972_d_n6;
        locals.var_pds_ini_dn7 = assign12280_e13972_d_n7;
        locals.var_pds_ini_dn10 = assign12280_e13972_d_n10;
        locals.var_pds_ini_dn11 = assign12280_e13972_d_n11;
        locals.var_pds_ini_dn12 = assign12280_e13972_d_n12;
        locals.var_pds_ini_dn17 = assign12280_e13972_d_n17;

        let (assign12290_e13985, assign12290_e13985_d_n0, assign12290_e13985_d_n2, assign12290_e13985_d_n6, assign12290_e13985_d_n7, assign12290_e13985_d_n10, assign12290_e13985_d_n11, assign12290_e13985_d_n12, assign12290_e13985_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn12, locals.var_pds_ini_dn17,)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn12, locals.var_pds_dn17,)
    }
};
        locals.var_pds = assign12290_e13985;
        locals.var_pds_dn0 = assign12290_e13985_d_n0;
        locals.var_pds_dn2 = assign12290_e13985_d_n2;
        locals.var_pds_dn6 = assign12290_e13985_d_n6;
        locals.var_pds_dn7 = assign12290_e13985_d_n7;
        locals.var_pds_dn10 = assign12290_e13985_d_n10;
        locals.var_pds_dn11 = assign12290_e13985_d_n11;
        locals.var_pds_dn12 = assign12290_e13985_d_n12;
        locals.var_pds_dn17 = assign12290_e13985_d_n17;

        let (assign12300_e14000, assign12300_e14000_d_n0, assign12300_e14000_d_n2, assign12300_e14000_d_n6, assign12300_e14000_d_n7, assign12300_e14000_d_n10, assign12300_e14000_d_n11, assign12300_e14000_d_n12, assign12300_e14000_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) {
        let assign12300_e13998: f64 = (locals.var_phi_s0_soi + locals.var_pds);
        (assign12300_e13998, (locals.var_phi_s0_soi_dn0 + locals.var_pds_dn0), (locals.var_phi_s0_soi_dn2 + locals.var_pds_dn2), (locals.var_phi_s0_soi_dn6 + locals.var_pds_dn6), (locals.var_phi_s0_soi_dn7 + locals.var_pds_dn7), (locals.var_phi_s0_soi_dn10 + locals.var_pds_dn10), (locals.var_phi_s0_soi_dn11 + locals.var_pds_dn11), (locals.var_phi_s0_soi_dn12 + locals.var_pds_dn12), (locals.var_phi_s0_soi_dn17 + locals.var_pds_dn17),)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    }
};
        locals.var_psl = assign12300_e14000;
        locals.var_psl_dn0 = assign12300_e14000_d_n0;
        locals.var_psl_dn2 = assign12300_e14000_d_n2;
        locals.var_psl_dn6 = assign12300_e14000_d_n6;
        locals.var_psl_dn7 = assign12300_e14000_d_n7;
        locals.var_psl_dn10 = assign12300_e14000_d_n10;
        locals.var_psl_dn11 = assign12300_e14000_d_n11;
        locals.var_psl_dn12 = assign12300_e14000_d_n12;
        locals.var_psl_dn17 = assign12300_e14000_d_n17;

        let (assign12310_e14013, assign12310_e14013_d_n0, assign12310_e14013_d_n2, assign12310_e14013_d_n6, assign12310_e14013_d_n7, assign12310_e14013_d_n10, assign12310_e14013_d_n11, assign12310_e14013_d_n12, assign12310_e14013_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
        locals.var_phi_sl_soi = assign12310_e14013;
        locals.var_phi_sl_soi_dn0 = assign12310_e14013_d_n0;
        locals.var_phi_sl_soi_dn2 = assign12310_e14013_d_n2;
        locals.var_phi_sl_soi_dn6 = assign12310_e14013_d_n6;
        locals.var_phi_sl_soi_dn7 = assign12310_e14013_d_n7;
        locals.var_phi_sl_soi_dn10 = assign12310_e14013_d_n10;
        locals.var_phi_sl_soi_dn11 = assign12310_e14013_d_n11;
        locals.var_phi_sl_soi_dn12 = assign12310_e14013_d_n12;
        locals.var_phi_sl_soi_dn17 = assign12310_e14013_d_n17;

        let (assign12320_e14026, assign12320_e14026_d_n0, assign12320_e14026_d_n2, assign12320_e14026_d_n6, assign12320_e14026_d_n7, assign12320_e14026_d_n10, assign12320_e14026_d_n11, assign12320_e14026_d_n12, assign12320_e14026_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) {
        (locals.var_phi_b_dep0, locals.var_phi_b_dep0_dn0, locals.var_phi_b_dep0_dn2, locals.var_phi_b_dep0_dn6, locals.var_phi_b_dep0_dn7, locals.var_phi_b_dep0_dn10, locals.var_phi_b_dep0_dn11, locals.var_phi_b_dep0_dn12, locals.var_phi_b_dep0_dn17,)
    } else {
        (locals.var_phi_b_dep, locals.var_phi_b_dep_dn0, locals.var_phi_b_dep_dn2, locals.var_phi_b_dep_dn6, locals.var_phi_b_dep_dn7, locals.var_phi_b_dep_dn10, locals.var_phi_b_dep_dn11, locals.var_phi_b_dep_dn12, locals.var_phi_b_dep_dn17,)
    }
};
        locals.var_phi_b_dep = assign12320_e14026;
        locals.var_phi_b_dep_dn0 = assign12320_e14026_d_n0;
        locals.var_phi_b_dep_dn2 = assign12320_e14026_d_n2;
        locals.var_phi_b_dep_dn6 = assign12320_e14026_d_n6;
        locals.var_phi_b_dep_dn7 = assign12320_e14026_d_n7;
        locals.var_phi_b_dep_dn10 = assign12320_e14026_d_n10;
        locals.var_phi_b_dep_dn11 = assign12320_e14026_d_n11;
        locals.var_phi_b_dep_dn12 = assign12320_e14026_d_n12;
        locals.var_phi_b_dep_dn17 = assign12320_e14026_d_n17;

        let (assign12330_e14045, assign12330_e14045_d_n10,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) {
        let assign12330_e14039: f64 = (locals.var_cnst0bulk * locals.var_cnst0bulk);
        let assign12330_e14041: f64 = (assign12330_e14039 * locals.var_c_box_fd_inv);
        let assign12330_e14043: f64 = (assign12330_e14041 * locals.var_c_box_fd_inv);
        (assign12330_e14043, ((((locals.var_cnst0bulk_dn10 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn10)) * locals.var_c_box_fd_inv) * locals.var_c_box_fd_inv),)
    } else {
        (locals.var_t0__blk322, locals.var_t0__blk322_dn10,)
    }
};
        locals.var_t0__blk322 = assign12330_e14045;
        locals.var_t0__blk322_dn10 = assign12330_e14045_d_n10;

        let assign12340_e14048: f64 = if locals.var_phi_sl_soi < locals.var_fd_end { 1.0 } else { 0.0 };
        locals.var_guard328 = assign12340_e14048;

        let (assign12350_e14064, assign12350_e14064_d_n0, assign12350_e14064_d_n2, assign12350_e14064_d_n6, assign12350_e14064_d_n7, assign12350_e14064_d_n10, assign12350_e14064_d_n11, assign12350_e14064_d_n12, assign12350_e14064_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard328 != 0.0)) {
        let assign12350_e14062: f64 = (-locals.var_vbsbiz);
        (assign12350_e14062, (-locals.var_vbsbiz_dn0), (-locals.var_vbsbiz_dn2), (-locals.var_vbsbiz_dn6), (-locals.var_vbsbiz_dn7), (-locals.var_vbsbiz_dn10), (-locals.var_vbsbiz_dn11), (-locals.var_vbsbiz_dn12), (-locals.var_vbsbiz_dn17),)
    } else {
        (locals.var_t1__blk323, locals.var_t1__blk323_dn0, locals.var_t1__blk323_dn2, locals.var_t1__blk323_dn6, locals.var_t1__blk323_dn7, locals.var_t1__blk323_dn10, locals.var_t1__blk323_dn11, locals.var_t1__blk323_dn12, locals.var_t1__blk323_dn17,)
    }
};
        locals.var_t1__blk323 = assign12350_e14064;
        locals.var_t1__blk323_dn0 = assign12350_e14064_d_n0;
        locals.var_t1__blk323_dn2 = assign12350_e14064_d_n2;
        locals.var_t1__blk323_dn6 = assign12350_e14064_d_n6;
        locals.var_t1__blk323_dn7 = assign12350_e14064_d_n7;
        locals.var_t1__blk323_dn10 = assign12350_e14064_d_n10;
        locals.var_t1__blk323_dn11 = assign12350_e14064_d_n11;
        locals.var_t1__blk323_dn12 = assign12350_e14064_d_n12;
        locals.var_t1__blk323_dn17 = assign12350_e14064_d_n17;

        let (assign12360_e14101, assign12360_e14101_d_n0, assign12360_e14101_d_n2, assign12360_e14101_d_n6, assign12360_e14101_d_n7, assign12360_e14101_d_n10, assign12360_e14101_d_n11, assign12360_e14101_d_n12, assign12360_e14101_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard328 != 0.0)) {
        let assign12360_e14079: f64 = (2.0 * locals.var_t1__blk323);
        let assign12360_e14082: f64 = (locals.var_t0__blk322 * locals.var_beta);
        let assign12360_e14083: f64 = (assign12360_e14079 + assign12360_e14082);
        let assign12360_e14086: f64 = (2.0 * locals.var_t1__blk323);
        let assign12360_e14089: f64 = (locals.var_t0__blk322 * locals.var_beta);
        let assign12360_e14090: f64 = (assign12360_e14086 + assign12360_e14089);
        let assign12360_e14091: f64 = (assign12360_e14083 * assign12360_e14090);
        let assign12360_e14095: f64 = (locals.var_t1__blk323 * locals.var_t1__blk323);
        let assign12360_e14097: f64 = (assign12360_e14095 + locals.var_t0__blk322);
        let assign12360_e14098: f64 = (4.0 * assign12360_e14097);
        let assign12360_e14099: f64 = (assign12360_e14091 - assign12360_e14098);
        (assign12360_e14099, ((((2.0 * locals.var_t1__blk323_dn0) * assign12360_e14090) + (assign12360_e14083 * (2.0 * locals.var_t1__blk323_dn0))) - (4.0 * ((locals.var_t1__blk323_dn0 * locals.var_t1__blk323) + (locals.var_t1__blk323 * locals.var_t1__blk323_dn0)))), ((((2.0 * locals.var_t1__blk323_dn2) * assign12360_e14090) + (assign12360_e14083 * (2.0 * locals.var_t1__blk323_dn2))) - (4.0 * ((locals.var_t1__blk323_dn2 * locals.var_t1__blk323) + (locals.var_t1__blk323 * locals.var_t1__blk323_dn2)))), ((((2.0 * locals.var_t1__blk323_dn6) * assign12360_e14090) + (assign12360_e14083 * (2.0 * locals.var_t1__blk323_dn6))) - (4.0 * ((locals.var_t1__blk323_dn6 * locals.var_t1__blk323) + (locals.var_t1__blk323 * locals.var_t1__blk323_dn6)))), ((((2.0 * locals.var_t1__blk323_dn7) * assign12360_e14090) + (assign12360_e14083 * (2.0 * locals.var_t1__blk323_dn7))) - (4.0 * ((locals.var_t1__blk323_dn7 * locals.var_t1__blk323) + (locals.var_t1__blk323 * locals.var_t1__blk323_dn7)))), (((((2.0 * locals.var_t1__blk323_dn10) + ((locals.var_t0__blk322_dn10 * locals.var_beta) + (locals.var_t0__blk322 * locals.var_beta_dn10))) * assign12360_e14090) + (assign12360_e14083 * ((2.0 * locals.var_t1__blk323_dn10) + ((locals.var_t0__blk322_dn10 * locals.var_beta) + (locals.var_t0__blk322 * locals.var_beta_dn10))))) - (4.0 * (((locals.var_t1__blk323_dn10 * locals.var_t1__blk323) + (locals.var_t1__blk323 * locals.var_t1__blk323_dn10)) + locals.var_t0__blk322_dn10))), ((((2.0 * locals.var_t1__blk323_dn11) * assign12360_e14090) + (assign12360_e14083 * (2.0 * locals.var_t1__blk323_dn11))) - (4.0 * ((locals.var_t1__blk323_dn11 * locals.var_t1__blk323) + (locals.var_t1__blk323 * locals.var_t1__blk323_dn11)))), ((((2.0 * locals.var_t1__blk323_dn12) * assign12360_e14090) + (assign12360_e14083 * (2.0 * locals.var_t1__blk323_dn12))) - (4.0 * ((locals.var_t1__blk323_dn12 * locals.var_t1__blk323) + (locals.var_t1__blk323 * locals.var_t1__blk323_dn12)))), ((((2.0 * locals.var_t1__blk323_dn17) * assign12360_e14090) + (assign12360_e14083 * (2.0 * locals.var_t1__blk323_dn17))) - (4.0 * ((locals.var_t1__blk323_dn17 * locals.var_t1__blk323) + (locals.var_t1__blk323 * locals.var_t1__blk323_dn17)))),)
    } else {
        (locals.var_t2__blk324, locals.var_t2__blk324_dn0, locals.var_t2__blk324_dn2, locals.var_t2__blk324_dn6, locals.var_t2__blk324_dn7, locals.var_t2__blk324_dn10, locals.var_t2__blk324_dn11, locals.var_t2__blk324_dn12, locals.var_t2__blk324_dn17,)
    }
};
        locals.var_t2__blk324 = assign12360_e14101;
        locals.var_t2__blk324_dn0 = assign12360_e14101_d_n0;
        locals.var_t2__blk324_dn2 = assign12360_e14101_d_n2;
        locals.var_t2__blk324_dn6 = assign12360_e14101_d_n6;
        locals.var_t2__blk324_dn7 = assign12360_e14101_d_n7;
        locals.var_t2__blk324_dn10 = assign12360_e14101_d_n10;
        locals.var_t2__blk324_dn11 = assign12360_e14101_d_n11;
        locals.var_t2__blk324_dn12 = assign12360_e14101_d_n12;
        locals.var_t2__blk324_dn17 = assign12360_e14101_d_n17;

        let (assign12370_e14125, assign12370_e14125_d_n0, assign12370_e14125_d_n2, assign12370_e14125_d_n6, assign12370_e14125_d_n7, assign12370_e14125_d_n10, assign12370_e14125_d_n11, assign12370_e14125_d_n12, assign12370_e14125_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard328 != 0.0)) {
        let assign12370_e14117: f64 = (10.0 * 2.220446049250313e-16);
        let (assign12370_e14123, assign12370_e14123_d_n0, assign12370_e14123_d_n2, assign12370_e14123_d_n6, assign12370_e14123_d_n7, assign12370_e14123_d_n10, assign12370_e14123_d_n11, assign12370_e14123_d_n12, assign12370_e14123_d_n17,) = {
            if (locals.var_t2__blk324 >= assign12370_e14117) {
                (locals.var_t2__blk324, locals.var_t2__blk324_dn0, locals.var_t2__blk324_dn2, locals.var_t2__blk324_dn6, locals.var_t2__blk324_dn7, locals.var_t2__blk324_dn10, locals.var_t2__blk324_dn11, locals.var_t2__blk324_dn12, locals.var_t2__blk324_dn17,)
            } else {
                let assign12370_e14122: f64 = (10.0 * 2.220446049250313e-16);
                (assign12370_e14122, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign12370_e14123, assign12370_e14123_d_n0, assign12370_e14123_d_n2, assign12370_e14123_d_n6, assign12370_e14123_d_n7, assign12370_e14123_d_n10, assign12370_e14123_d_n11, assign12370_e14123_d_n12, assign12370_e14123_d_n17,)
    } else {
        (locals.var_t2__blk324, locals.var_t2__blk324_dn0, locals.var_t2__blk324_dn2, locals.var_t2__blk324_dn6, locals.var_t2__blk324_dn7, locals.var_t2__blk324_dn10, locals.var_t2__blk324_dn11, locals.var_t2__blk324_dn12, locals.var_t2__blk324_dn17,)
    }
};
        locals.var_t2__blk324 = assign12370_e14125;
        locals.var_t2__blk324_dn0 = assign12370_e14125_d_n0;
        locals.var_t2__blk324_dn2 = assign12370_e14125_d_n2;
        locals.var_t2__blk324_dn6 = assign12370_e14125_d_n6;
        locals.var_t2__blk324_dn7 = assign12370_e14125_d_n7;
        locals.var_t2__blk324_dn10 = assign12370_e14125_d_n10;
        locals.var_t2__blk324_dn11 = assign12370_e14125_d_n11;
        locals.var_t2__blk324_dn12 = assign12370_e14125_d_n12;
        locals.var_t2__blk324_dn17 = assign12370_e14125_d_n17;

        let (assign12380_e14141, assign12380_e14141_d_n0, assign12380_e14141_d_n2, assign12380_e14141_d_n6, assign12380_e14141_d_n7, assign12380_e14141_d_n10, assign12380_e14141_d_n11, assign12380_e14141_d_n12, assign12380_e14141_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard328 != 0.0)) {
        let assign12380_e14139: f64 = (locals.var_t2__blk324).sqrt();
        (assign12380_e14139, (locals.var_t2__blk324_dn0 / (2.0 * assign12380_e14139)), (locals.var_t2__blk324_dn2 / (2.0 * assign12380_e14139)), (locals.var_t2__blk324_dn6 / (2.0 * assign12380_e14139)), (locals.var_t2__blk324_dn7 / (2.0 * assign12380_e14139)), (locals.var_t2__blk324_dn10 / (2.0 * assign12380_e14139)), (locals.var_t2__blk324_dn11 / (2.0 * assign12380_e14139)), (locals.var_t2__blk324_dn12 / (2.0 * assign12380_e14139)), (locals.var_t2__blk324_dn17 / (2.0 * assign12380_e14139)),)
    } else {
        (locals.var_t2__blk324, locals.var_t2__blk324_dn0, locals.var_t2__blk324_dn2, locals.var_t2__blk324_dn6, locals.var_t2__blk324_dn7, locals.var_t2__blk324_dn10, locals.var_t2__blk324_dn11, locals.var_t2__blk324_dn12, locals.var_t2__blk324_dn17,)
    }
};
        locals.var_t2__blk324 = assign12380_e14141;
        locals.var_t2__blk324_dn0 = assign12380_e14141_d_n0;
        locals.var_t2__blk324_dn2 = assign12380_e14141_d_n2;
        locals.var_t2__blk324_dn6 = assign12380_e14141_d_n6;
        locals.var_t2__blk324_dn7 = assign12380_e14141_d_n7;
        locals.var_t2__blk324_dn10 = assign12380_e14141_d_n10;
        locals.var_t2__blk324_dn11 = assign12380_e14141_d_n11;
        locals.var_t2__blk324_dn12 = assign12380_e14141_d_n12;
        locals.var_t2__blk324_dn17 = assign12380_e14141_d_n17;

        let (assign12390_e14162, assign12390_e14162_d_n0, assign12390_e14162_d_n2, assign12390_e14162_d_n6, assign12390_e14162_d_n7, assign12390_e14162_d_n10, assign12390_e14162_d_n11, assign12390_e14162_d_n12, assign12390_e14162_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard328 != 0.0)) {
        let assign12390_e14156: f64 = (2.0 * locals.var_t1__blk323);
        let assign12390_e14159: f64 = (locals.var_t0__blk322 * locals.var_beta);
        let assign12390_e14160: f64 = (assign12390_e14156 + assign12390_e14159);
        (assign12390_e14160, (2.0 * locals.var_t1__blk323_dn0), (2.0 * locals.var_t1__blk323_dn2), (2.0 * locals.var_t1__blk323_dn6), (2.0 * locals.var_t1__blk323_dn7), ((2.0 * locals.var_t1__blk323_dn10) + ((locals.var_t0__blk322_dn10 * locals.var_beta) + (locals.var_t0__blk322 * locals.var_beta_dn10))), (2.0 * locals.var_t1__blk323_dn11), (2.0 * locals.var_t1__blk323_dn12), (2.0 * locals.var_t1__blk323_dn17),)
    } else {
        (locals.var_t3__blk325, locals.var_t3__blk325_dn0, locals.var_t3__blk325_dn2, locals.var_t3__blk325_dn6, locals.var_t3__blk325_dn7, locals.var_t3__blk325_dn10, locals.var_t3__blk325_dn11, locals.var_t3__blk325_dn12, locals.var_t3__blk325_dn17,)
    }
};
        locals.var_t3__blk325 = assign12390_e14162;
        locals.var_t3__blk325_dn0 = assign12390_e14162_d_n0;
        locals.var_t3__blk325_dn2 = assign12390_e14162_d_n2;
        locals.var_t3__blk325_dn6 = assign12390_e14162_d_n6;
        locals.var_t3__blk325_dn7 = assign12390_e14162_d_n7;
        locals.var_t3__blk325_dn10 = assign12390_e14162_d_n10;
        locals.var_t3__blk325_dn11 = assign12390_e14162_d_n11;
        locals.var_t3__blk325_dn12 = assign12390_e14162_d_n12;
        locals.var_t3__blk325_dn17 = assign12390_e14162_d_n17;

        let (assign12400_e14181, assign12400_e14181_d_n0, assign12400_e14181_d_n2, assign12400_e14181_d_n6, assign12400_e14181_d_n7, assign12400_e14181_d_n10, assign12400_e14181_d_n11, assign12400_e14181_d_n12, assign12400_e14181_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard328 != 0.0)) {
        let assign12400_e14177: f64 = (locals.var_t3__blk325 - locals.var_t2__blk324);
        let assign12400_e14179: f64 = (assign12400_e14177 / 2.0);
        (assign12400_e14179, ((locals.var_t3__blk325_dn0 - locals.var_t2__blk324_dn0) / 2.0), ((locals.var_t3__blk325_dn2 - locals.var_t2__blk324_dn2) / 2.0), ((locals.var_t3__blk325_dn6 - locals.var_t2__blk324_dn6) / 2.0), ((locals.var_t3__blk325_dn7 - locals.var_t2__blk324_dn7) / 2.0), ((locals.var_t3__blk325_dn10 - locals.var_t2__blk324_dn10) / 2.0), ((locals.var_t3__blk325_dn11 - locals.var_t2__blk324_dn11) / 2.0), ((locals.var_t3__blk325_dn12 - locals.var_t2__blk324_dn12) / 2.0), ((locals.var_t3__blk325_dn17 - locals.var_t2__blk324_dn17) / 2.0),)
    } else {
        (locals.var_psb_inia__blk326, locals.var_psb_inia__blk326_dn0, locals.var_psb_inia__blk326_dn2, locals.var_psb_inia__blk326_dn6, locals.var_psb_inia__blk326_dn7, locals.var_psb_inia__blk326_dn10, locals.var_psb_inia__blk326_dn11, locals.var_psb_inia__blk326_dn12, locals.var_psb_inia__blk326_dn17,)
    }
};
        locals.var_psb_inia__blk326 = assign12400_e14181;
        locals.var_psb_inia__blk326_dn0 = assign12400_e14181_d_n0;
        locals.var_psb_inia__blk326_dn2 = assign12400_e14181_d_n2;
        locals.var_psb_inia__blk326_dn6 = assign12400_e14181_d_n6;
        locals.var_psb_inia__blk326_dn7 = assign12400_e14181_d_n7;
        locals.var_psb_inia__blk326_dn10 = assign12400_e14181_d_n10;
        locals.var_psb_inia__blk326_dn11 = assign12400_e14181_d_n11;
        locals.var_psb_inia__blk326_dn12 = assign12400_e14181_d_n12;
        locals.var_psb_inia__blk326_dn17 = assign12400_e14181_d_n17;

        let (assign12410_e14209, assign12410_e14209_d_n0, assign12410_e14209_d_n2, assign12410_e14209_d_n6, assign12410_e14209_d_n7, assign12410_e14209_d_n10, assign12410_e14209_d_n11, assign12410_e14209_d_n12, assign12410_e14209_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard328 != 0.0)) {
        let assign12410_e14196: f64 = (locals.var_t1__blk323 * locals.var_t1__blk323);
        let assign12410_e14198: f64 = (assign12410_e14196 / locals.var_t0__blk322);
        let assign12410_e14200: f64 = (assign12410_e14198 / locals.var_cnst1bulk);
        let assign12410_e14201: f64 = (assign12410_e14200).ln();
        let assign12410_e14205: f64 = (2.0 / locals.var_t1__blk323);
        let assign12410_e14206: f64 = (locals.var_beta + assign12410_e14205);
        let assign12410_e14207: f64 = (assign12410_e14201 / assign12410_e14206);
        (assign12410_e14207, ((((((((((locals.var_t1__blk323_dn0 * locals.var_t1__blk323) + (locals.var_t1__blk323 * locals.var_t1__blk323_dn0)) / locals.var_t0__blk322) * locals.var_cnst1bulk) - (assign12410_e14198 * locals.var_cnst1bulk_dn0)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12410_e14200) * assign12410_e14206) - (assign12410_e14201 * (-((2.0 * locals.var_t1__blk323_dn0) / (locals.var_t1__blk323 * locals.var_t1__blk323))))) / (assign12410_e14206 * assign12410_e14206)), ((((((((((locals.var_t1__blk323_dn2 * locals.var_t1__blk323) + (locals.var_t1__blk323 * locals.var_t1__blk323_dn2)) / locals.var_t0__blk322) * locals.var_cnst1bulk) - (assign12410_e14198 * locals.var_cnst1bulk_dn2)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12410_e14200) * assign12410_e14206) - (assign12410_e14201 * (-((2.0 * locals.var_t1__blk323_dn2) / (locals.var_t1__blk323 * locals.var_t1__blk323))))) / (assign12410_e14206 * assign12410_e14206)), ((((((((((locals.var_t1__blk323_dn6 * locals.var_t1__blk323) + (locals.var_t1__blk323 * locals.var_t1__blk323_dn6)) / locals.var_t0__blk322) * locals.var_cnst1bulk) - (assign12410_e14198 * locals.var_cnst1bulk_dn6)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12410_e14200) * assign12410_e14206) - (assign12410_e14201 * (-((2.0 * locals.var_t1__blk323_dn6) / (locals.var_t1__blk323 * locals.var_t1__blk323))))) / (assign12410_e14206 * assign12410_e14206)), ((((((((((locals.var_t1__blk323_dn7 * locals.var_t1__blk323) + (locals.var_t1__blk323 * locals.var_t1__blk323_dn7)) / locals.var_t0__blk322) * locals.var_cnst1bulk) - (assign12410_e14198 * locals.var_cnst1bulk_dn7)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12410_e14200) * assign12410_e14206) - (assign12410_e14201 * (-((2.0 * locals.var_t1__blk323_dn7) / (locals.var_t1__blk323 * locals.var_t1__blk323))))) / (assign12410_e14206 * assign12410_e14206)), ((((((((((((locals.var_t1__blk323_dn10 * locals.var_t1__blk323) + (locals.var_t1__blk323 * locals.var_t1__blk323_dn10)) * locals.var_t0__blk322) - (assign12410_e14196 * locals.var_t0__blk322_dn10)) / (locals.var_t0__blk322 * locals.var_t0__blk322)) * locals.var_cnst1bulk) - (assign12410_e14198 * locals.var_cnst1bulk_dn10)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12410_e14200) * assign12410_e14206) - (assign12410_e14201 * (locals.var_beta_dn10 + (-((2.0 * locals.var_t1__blk323_dn10) / (locals.var_t1__blk323 * locals.var_t1__blk323)))))) / (assign12410_e14206 * assign12410_e14206)), ((((((((((locals.var_t1__blk323_dn11 * locals.var_t1__blk323) + (locals.var_t1__blk323 * locals.var_t1__blk323_dn11)) / locals.var_t0__blk322) * locals.var_cnst1bulk) - (assign12410_e14198 * locals.var_cnst1bulk_dn11)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12410_e14200) * assign12410_e14206) - (assign12410_e14201 * (-((2.0 * locals.var_t1__blk323_dn11) / (locals.var_t1__blk323 * locals.var_t1__blk323))))) / (assign12410_e14206 * assign12410_e14206)), ((((((((((locals.var_t1__blk323_dn12 * locals.var_t1__blk323) + (locals.var_t1__blk323 * locals.var_t1__blk323_dn12)) / locals.var_t0__blk322) * locals.var_cnst1bulk) - (assign12410_e14198 * locals.var_cnst1bulk_dn12)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12410_e14200) * assign12410_e14206) - (assign12410_e14201 * (-((2.0 * locals.var_t1__blk323_dn12) / (locals.var_t1__blk323 * locals.var_t1__blk323))))) / (assign12410_e14206 * assign12410_e14206)), ((((((((((locals.var_t1__blk323_dn17 * locals.var_t1__blk323) + (locals.var_t1__blk323 * locals.var_t1__blk323_dn17)) / locals.var_t0__blk322) * locals.var_cnst1bulk) - (assign12410_e14198 * locals.var_cnst1bulk_dn17)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12410_e14200) * assign12410_e14206) - (assign12410_e14201 * (-((2.0 * locals.var_t1__blk323_dn17) / (locals.var_t1__blk323 * locals.var_t1__blk323))))) / (assign12410_e14206 * assign12410_e14206)),)
    } else {
        (locals.var_psb_inib__blk327, locals.var_psb_inib__blk327_dn0, locals.var_psb_inib__blk327_dn2, locals.var_psb_inib__blk327_dn6, locals.var_psb_inib__blk327_dn7, locals.var_psb_inib__blk327_dn10, locals.var_psb_inib__blk327_dn11, locals.var_psb_inib__blk327_dn12, locals.var_psb_inib__blk327_dn17,)
    }
};
        locals.var_psb_inib__blk327 = assign12410_e14209;
        locals.var_psb_inib__blk327_dn0 = assign12410_e14209_d_n0;
        locals.var_psb_inib__blk327_dn2 = assign12410_e14209_d_n2;
        locals.var_psb_inib__blk327_dn6 = assign12410_e14209_d_n6;
        locals.var_psb_inib__blk327_dn7 = assign12410_e14209_d_n7;
        locals.var_psb_inib__blk327_dn10 = assign12410_e14209_d_n10;
        locals.var_psb_inib__blk327_dn11 = assign12410_e14209_d_n11;
        locals.var_psb_inib__blk327_dn12 = assign12410_e14209_d_n12;
        locals.var_psb_inib__blk327_dn17 = assign12410_e14209_d_n17;

        let assign12420_e14212: f64 = if locals.var_psb_inia__blk326 < locals.var_pb2_bulk { 1.0 } else { 0.0 };
        locals.var_guard329 = assign12420_e14212;

        let (assign12430_e14229, assign12430_e14229_d_n0, assign12430_e14229_d_n2, assign12430_e14229_d_n6, assign12430_e14229_d_n7, assign12430_e14229_d_n10, assign12430_e14229_d_n11, assign12430_e14229_d_n12, assign12430_e14229_d_n17,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard328 != 0.0)) && (locals.var_guard329 != 0.0)) {
        (locals.var_psb_inia__blk326, locals.var_psb_inia__blk326_dn0, locals.var_psb_inia__blk326_dn2, locals.var_psb_inia__blk326_dn6, locals.var_psb_inia__blk326_dn7, locals.var_psb_inia__blk326_dn10, locals.var_psb_inia__blk326_dn11, locals.var_psb_inia__blk326_dn12, locals.var_psb_inia__blk326_dn17,)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
        locals.var_phi_sl_bulk = assign12430_e14229;
        locals.var_phi_sl_bulk_dn0 = assign12430_e14229_d_n0;
        locals.var_phi_sl_bulk_dn2 = assign12430_e14229_d_n2;
        locals.var_phi_sl_bulk_dn6 = assign12430_e14229_d_n6;
        locals.var_phi_sl_bulk_dn7 = assign12430_e14229_d_n7;
        locals.var_phi_sl_bulk_dn10 = assign12430_e14229_d_n10;
        locals.var_phi_sl_bulk_dn11 = assign12430_e14229_d_n11;
        locals.var_phi_sl_bulk_dn12 = assign12430_e14229_d_n12;
        locals.var_phi_sl_bulk_dn17 = assign12430_e14229_d_n17;

        let (assign12440_e14251, assign12440_e14251_d_n0, assign12440_e14251_d_n2, assign12440_e14251_d_n6, assign12440_e14251_d_n7, assign12440_e14251_d_n10, assign12440_e14251_d_n11, assign12440_e14251_d_n12, assign12440_e14251_d_n17,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard328 != 0.0)) && (locals.var_guard329 == 0.0)) {
        let assign12440_e14247: f64 = (locals.var_psb_inib__blk327 - locals.var_psb_inia__blk326);
        let assign12440_e14249: f64 = (assign12440_e14247 - 0.0008);
        (assign12440_e14249, (locals.var_psb_inib__blk327_dn0 - locals.var_psb_inia__blk326_dn0), (locals.var_psb_inib__blk327_dn2 - locals.var_psb_inia__blk326_dn2), (locals.var_psb_inib__blk327_dn6 - locals.var_psb_inia__blk326_dn6), (locals.var_psb_inib__blk327_dn7 - locals.var_psb_inia__blk326_dn7), (locals.var_psb_inib__blk327_dn10 - locals.var_psb_inia__blk326_dn10), (locals.var_psb_inib__blk327_dn11 - locals.var_psb_inia__blk326_dn11), (locals.var_psb_inib__blk327_dn12 - locals.var_psb_inia__blk326_dn12), (locals.var_psb_inib__blk327_dn17 - locals.var_psb_inia__blk326_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign12440_e14251;
        locals.var_tmf1_dn0 = assign12440_e14251_d_n0;
        locals.var_tmf1_dn2 = assign12440_e14251_d_n2;
        locals.var_tmf1_dn6 = assign12440_e14251_d_n6;
        locals.var_tmf1_dn7 = assign12440_e14251_d_n7;
        locals.var_tmf1_dn10 = assign12440_e14251_d_n10;
        locals.var_tmf1_dn11 = assign12440_e14251_d_n11;
        locals.var_tmf1_dn12 = assign12440_e14251_d_n12;
        locals.var_tmf1_dn17 = assign12440_e14251_d_n17;

        let (assign12450_e14273, assign12450_e14273_d_n0, assign12450_e14273_d_n2, assign12450_e14273_d_n6, assign12450_e14273_d_n7, assign12450_e14273_d_n10, assign12450_e14273_d_n11, assign12450_e14273_d_n12, assign12450_e14273_d_n17,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard328 != 0.0)) && (locals.var_guard329 == 0.0)) {
        let assign12450_e14269: f64 = (4.0 * locals.var_psb_inib__blk327);
        let assign12450_e14271: f64 = (assign12450_e14269 * 0.0008);
        (assign12450_e14271, ((4.0 * locals.var_psb_inib__blk327_dn0) * 0.0008), ((4.0 * locals.var_psb_inib__blk327_dn2) * 0.0008), ((4.0 * locals.var_psb_inib__blk327_dn6) * 0.0008), ((4.0 * locals.var_psb_inib__blk327_dn7) * 0.0008), ((4.0 * locals.var_psb_inib__blk327_dn10) * 0.0008), ((4.0 * locals.var_psb_inib__blk327_dn11) * 0.0008), ((4.0 * locals.var_psb_inib__blk327_dn12) * 0.0008), ((4.0 * locals.var_psb_inib__blk327_dn17) * 0.0008),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign12450_e14273;
        locals.var_tmf2_dn0 = assign12450_e14273_d_n0;
        locals.var_tmf2_dn2 = assign12450_e14273_d_n2;
        locals.var_tmf2_dn6 = assign12450_e14273_d_n6;
        locals.var_tmf2_dn7 = assign12450_e14273_d_n7;
        locals.var_tmf2_dn10 = assign12450_e14273_d_n10;
        locals.var_tmf2_dn11 = assign12450_e14273_d_n11;
        locals.var_tmf2_dn12 = assign12450_e14273_d_n12;
        locals.var_tmf2_dn17 = assign12450_e14273_d_n17;

    }

    pub(super) fn stamp_transient_block_36(
        locals: &mut StampLocals,
    ) {
        let (assign12460_e14297, assign12460_e14297_d_n0, assign12460_e14297_d_n2, assign12460_e14297_d_n6, assign12460_e14297_d_n7, assign12460_e14297_d_n10, assign12460_e14297_d_n11, assign12460_e14297_d_n12, assign12460_e14297_d_n17,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard328 != 0.0)) && (locals.var_guard329 == 0.0)) {
        let (assign12460_e14295, assign12460_e14295_d_n0, assign12460_e14295_d_n2, assign12460_e14295_d_n6, assign12460_e14295_d_n7, assign12460_e14295_d_n10, assign12460_e14295_d_n11, assign12460_e14295_d_n12, assign12460_e14295_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign12460_e14294: f64 = (-locals.var_tmf2);
                (assign12460_e14294, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign12460_e14295, assign12460_e14295_d_n0, assign12460_e14295_d_n2, assign12460_e14295_d_n6, assign12460_e14295_d_n7, assign12460_e14295_d_n10, assign12460_e14295_d_n11, assign12460_e14295_d_n12, assign12460_e14295_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign12460_e14297;
        locals.var_tmf2_dn0 = assign12460_e14297_d_n0;
        locals.var_tmf2_dn2 = assign12460_e14297_d_n2;
        locals.var_tmf2_dn6 = assign12460_e14297_d_n6;
        locals.var_tmf2_dn7 = assign12460_e14297_d_n7;
        locals.var_tmf2_dn10 = assign12460_e14297_d_n10;
        locals.var_tmf2_dn11 = assign12460_e14297_d_n11;
        locals.var_tmf2_dn12 = assign12460_e14297_d_n12;
        locals.var_tmf2_dn17 = assign12460_e14297_d_n17;

        let (assign12470_e14320, assign12470_e14320_d_n0, assign12470_e14320_d_n2, assign12470_e14320_d_n6, assign12470_e14320_d_n7, assign12470_e14320_d_n10, assign12470_e14320_d_n11, assign12470_e14320_d_n12, assign12470_e14320_d_n17,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard328 != 0.0)) && (locals.var_guard329 == 0.0)) {
        let assign12470_e14315: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign12470_e14317: f64 = (assign12470_e14315 + locals.var_tmf2);
        let assign12470_e14318: f64 = (assign12470_e14317).sqrt();
        (assign12470_e14318, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign12470_e14318)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign12470_e14318)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign12470_e14318)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign12470_e14318)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign12470_e14318)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign12470_e14318)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign12470_e14318)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign12470_e14318)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign12470_e14320;
        locals.var_tmf2_dn0 = assign12470_e14320_d_n0;
        locals.var_tmf2_dn2 = assign12470_e14320_d_n2;
        locals.var_tmf2_dn6 = assign12470_e14320_d_n6;
        locals.var_tmf2_dn7 = assign12470_e14320_d_n7;
        locals.var_tmf2_dn10 = assign12470_e14320_d_n10;
        locals.var_tmf2_dn11 = assign12470_e14320_d_n11;
        locals.var_tmf2_dn12 = assign12470_e14320_d_n12;
        locals.var_tmf2_dn17 = assign12470_e14320_d_n17;

        let (assign12480_e14344, assign12480_e14344_d_n0, assign12480_e14344_d_n2, assign12480_e14344_d_n6, assign12480_e14344_d_n7, assign12480_e14344_d_n10, assign12480_e14344_d_n11, assign12480_e14344_d_n12, assign12480_e14344_d_n17,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard328 != 0.0)) && (locals.var_guard329 == 0.0)) {
        let assign12480_e14340: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign12480_e14341: f64 = (0.5 * assign12480_e14340);
        let assign12480_e14342: f64 = (locals.var_psb_inib__blk327 - assign12480_e14341);
        (assign12480_e14342, (locals.var_psb_inib__blk327_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psb_inib__blk327_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psb_inib__blk327_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psb_inib__blk327_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psb_inib__blk327_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psb_inib__blk327_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psb_inib__blk327_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_psb_inib__blk327_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
        locals.var_phi_sl_bulk = assign12480_e14344;
        locals.var_phi_sl_bulk_dn0 = assign12480_e14344_d_n0;
        locals.var_phi_sl_bulk_dn2 = assign12480_e14344_d_n2;
        locals.var_phi_sl_bulk_dn6 = assign12480_e14344_d_n6;
        locals.var_phi_sl_bulk_dn7 = assign12480_e14344_d_n7;
        locals.var_phi_sl_bulk_dn10 = assign12480_e14344_d_n10;
        locals.var_phi_sl_bulk_dn11 = assign12480_e14344_d_n11;
        locals.var_phi_sl_bulk_dn12 = assign12480_e14344_d_n12;
        locals.var_phi_sl_bulk_dn17 = assign12480_e14344_d_n17;

        let (assign12490_e14371, assign12490_e14371_d_n0, assign12490_e14371_d_n2, assign12490_e14371_d_n6, assign12490_e14371_d_n7, assign12490_e14371_d_n10, assign12490_e14371_d_n11, assign12490_e14371_d_n12, assign12490_e14371_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard328 == 0.0)) {
        let assign12490_e14360: f64 = (locals.var_vbsbiz - locals.var_phi_sl_soi);
        let assign12490_e14363: f64 = (locals.var_q_fd_soi / 2.0);
        let assign12490_e14365: f64 = (assign12490_e14363 * locals.var_t_soi);
        let assign12490_e14367: f64 = (assign12490_e14365 / 1.034943e-10);
        let assign12490_e14368: f64 = (assign12490_e14360 - assign12490_e14367);
        let assign12490_e14369: f64 = (-assign12490_e14368);
        (assign12490_e14369, (-((locals.var_vbsbiz_dn0 - locals.var_phi_sl_soi_dn0) - (((locals.var_q_fd_soi_dn0 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn2 - locals.var_phi_sl_soi_dn2) - (((locals.var_q_fd_soi_dn2 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn6 - locals.var_phi_sl_soi_dn6) - (((locals.var_q_fd_soi_dn6 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn7 - locals.var_phi_sl_soi_dn7) - (((locals.var_q_fd_soi_dn7 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn10 - locals.var_phi_sl_soi_dn10) - (((locals.var_q_fd_soi_dn10 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn11 - locals.var_phi_sl_soi_dn11) - (((locals.var_q_fd_soi_dn11 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn12 - locals.var_phi_sl_soi_dn12) - (((locals.var_q_fd_soi_dn12 / 2.0) * locals.var_t_soi) / 1.034943e-10))), (-((locals.var_vbsbiz_dn17 - locals.var_phi_sl_soi_dn17) - (((locals.var_q_fd_soi_dn17 / 2.0) * locals.var_t_soi) / 1.034943e-10))),)
    } else {
        (locals.var_t1__blk323, locals.var_t1__blk323_dn0, locals.var_t1__blk323_dn2, locals.var_t1__blk323_dn6, locals.var_t1__blk323_dn7, locals.var_t1__blk323_dn10, locals.var_t1__blk323_dn11, locals.var_t1__blk323_dn12, locals.var_t1__blk323_dn17,)
    }
};
        locals.var_t1__blk323 = assign12490_e14371;
        locals.var_t1__blk323_dn0 = assign12490_e14371_d_n0;
        locals.var_t1__blk323_dn2 = assign12490_e14371_d_n2;
        locals.var_t1__blk323_dn6 = assign12490_e14371_d_n6;
        locals.var_t1__blk323_dn7 = assign12490_e14371_d_n7;
        locals.var_t1__blk323_dn10 = assign12490_e14371_d_n10;
        locals.var_t1__blk323_dn11 = assign12490_e14371_d_n11;
        locals.var_t1__blk323_dn12 = assign12490_e14371_d_n12;
        locals.var_t1__blk323_dn17 = assign12490_e14371_d_n17;

        let (assign12500_e14409, assign12500_e14409_d_n0, assign12500_e14409_d_n2, assign12500_e14409_d_n6, assign12500_e14409_d_n7, assign12500_e14409_d_n10, assign12500_e14409_d_n11, assign12500_e14409_d_n12, assign12500_e14409_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard328 == 0.0)) {
        let assign12500_e14387: f64 = (2.0 * locals.var_t1__blk323);
        let assign12500_e14390: f64 = (locals.var_t0__blk322 * locals.var_beta);
        let assign12500_e14391: f64 = (assign12500_e14387 + assign12500_e14390);
        let assign12500_e14394: f64 = (2.0 * locals.var_t1__blk323);
        let assign12500_e14397: f64 = (locals.var_t0__blk322 * locals.var_beta);
        let assign12500_e14398: f64 = (assign12500_e14394 + assign12500_e14397);
        let assign12500_e14399: f64 = (assign12500_e14391 * assign12500_e14398);
        let assign12500_e14403: f64 = (locals.var_t1__blk323 * locals.var_t1__blk323);
        let assign12500_e14405: f64 = (assign12500_e14403 + locals.var_t0__blk322);
        let assign12500_e14406: f64 = (4.0 * assign12500_e14405);
        let assign12500_e14407: f64 = (assign12500_e14399 - assign12500_e14406);
        (assign12500_e14407, ((((2.0 * locals.var_t1__blk323_dn0) * assign12500_e14398) + (assign12500_e14391 * (2.0 * locals.var_t1__blk323_dn0))) - (4.0 * ((locals.var_t1__blk323_dn0 * locals.var_t1__blk323) + (locals.var_t1__blk323 * locals.var_t1__blk323_dn0)))), ((((2.0 * locals.var_t1__blk323_dn2) * assign12500_e14398) + (assign12500_e14391 * (2.0 * locals.var_t1__blk323_dn2))) - (4.0 * ((locals.var_t1__blk323_dn2 * locals.var_t1__blk323) + (locals.var_t1__blk323 * locals.var_t1__blk323_dn2)))), ((((2.0 * locals.var_t1__blk323_dn6) * assign12500_e14398) + (assign12500_e14391 * (2.0 * locals.var_t1__blk323_dn6))) - (4.0 * ((locals.var_t1__blk323_dn6 * locals.var_t1__blk323) + (locals.var_t1__blk323 * locals.var_t1__blk323_dn6)))), ((((2.0 * locals.var_t1__blk323_dn7) * assign12500_e14398) + (assign12500_e14391 * (2.0 * locals.var_t1__blk323_dn7))) - (4.0 * ((locals.var_t1__blk323_dn7 * locals.var_t1__blk323) + (locals.var_t1__blk323 * locals.var_t1__blk323_dn7)))), (((((2.0 * locals.var_t1__blk323_dn10) + ((locals.var_t0__blk322_dn10 * locals.var_beta) + (locals.var_t0__blk322 * locals.var_beta_dn10))) * assign12500_e14398) + (assign12500_e14391 * ((2.0 * locals.var_t1__blk323_dn10) + ((locals.var_t0__blk322_dn10 * locals.var_beta) + (locals.var_t0__blk322 * locals.var_beta_dn10))))) - (4.0 * (((locals.var_t1__blk323_dn10 * locals.var_t1__blk323) + (locals.var_t1__blk323 * locals.var_t1__blk323_dn10)) + locals.var_t0__blk322_dn10))), ((((2.0 * locals.var_t1__blk323_dn11) * assign12500_e14398) + (assign12500_e14391 * (2.0 * locals.var_t1__blk323_dn11))) - (4.0 * ((locals.var_t1__blk323_dn11 * locals.var_t1__blk323) + (locals.var_t1__blk323 * locals.var_t1__blk323_dn11)))), ((((2.0 * locals.var_t1__blk323_dn12) * assign12500_e14398) + (assign12500_e14391 * (2.0 * locals.var_t1__blk323_dn12))) - (4.0 * ((locals.var_t1__blk323_dn12 * locals.var_t1__blk323) + (locals.var_t1__blk323 * locals.var_t1__blk323_dn12)))), ((((2.0 * locals.var_t1__blk323_dn17) * assign12500_e14398) + (assign12500_e14391 * (2.0 * locals.var_t1__blk323_dn17))) - (4.0 * ((locals.var_t1__blk323_dn17 * locals.var_t1__blk323) + (locals.var_t1__blk323 * locals.var_t1__blk323_dn17)))),)
    } else {
        (locals.var_t2__blk324, locals.var_t2__blk324_dn0, locals.var_t2__blk324_dn2, locals.var_t2__blk324_dn6, locals.var_t2__blk324_dn7, locals.var_t2__blk324_dn10, locals.var_t2__blk324_dn11, locals.var_t2__blk324_dn12, locals.var_t2__blk324_dn17,)
    }
};
        locals.var_t2__blk324 = assign12500_e14409;
        locals.var_t2__blk324_dn0 = assign12500_e14409_d_n0;
        locals.var_t2__blk324_dn2 = assign12500_e14409_d_n2;
        locals.var_t2__blk324_dn6 = assign12500_e14409_d_n6;
        locals.var_t2__blk324_dn7 = assign12500_e14409_d_n7;
        locals.var_t2__blk324_dn10 = assign12500_e14409_d_n10;
        locals.var_t2__blk324_dn11 = assign12500_e14409_d_n11;
        locals.var_t2__blk324_dn12 = assign12500_e14409_d_n12;
        locals.var_t2__blk324_dn17 = assign12500_e14409_d_n17;

        let (assign12510_e14434, assign12510_e14434_d_n0, assign12510_e14434_d_n2, assign12510_e14434_d_n6, assign12510_e14434_d_n7, assign12510_e14434_d_n10, assign12510_e14434_d_n11, assign12510_e14434_d_n12, assign12510_e14434_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard328 == 0.0)) {
        let assign12510_e14426: f64 = (10.0 * 2.220446049250313e-16);
        let (assign12510_e14432, assign12510_e14432_d_n0, assign12510_e14432_d_n2, assign12510_e14432_d_n6, assign12510_e14432_d_n7, assign12510_e14432_d_n10, assign12510_e14432_d_n11, assign12510_e14432_d_n12, assign12510_e14432_d_n17,) = {
            if (locals.var_t2__blk324 >= assign12510_e14426) {
                (locals.var_t2__blk324, locals.var_t2__blk324_dn0, locals.var_t2__blk324_dn2, locals.var_t2__blk324_dn6, locals.var_t2__blk324_dn7, locals.var_t2__blk324_dn10, locals.var_t2__blk324_dn11, locals.var_t2__blk324_dn12, locals.var_t2__blk324_dn17,)
            } else {
                let assign12510_e14431: f64 = (10.0 * 2.220446049250313e-16);
                (assign12510_e14431, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign12510_e14432, assign12510_e14432_d_n0, assign12510_e14432_d_n2, assign12510_e14432_d_n6, assign12510_e14432_d_n7, assign12510_e14432_d_n10, assign12510_e14432_d_n11, assign12510_e14432_d_n12, assign12510_e14432_d_n17,)
    } else {
        (locals.var_t2__blk324, locals.var_t2__blk324_dn0, locals.var_t2__blk324_dn2, locals.var_t2__blk324_dn6, locals.var_t2__blk324_dn7, locals.var_t2__blk324_dn10, locals.var_t2__blk324_dn11, locals.var_t2__blk324_dn12, locals.var_t2__blk324_dn17,)
    }
};
        locals.var_t2__blk324 = assign12510_e14434;
        locals.var_t2__blk324_dn0 = assign12510_e14434_d_n0;
        locals.var_t2__blk324_dn2 = assign12510_e14434_d_n2;
        locals.var_t2__blk324_dn6 = assign12510_e14434_d_n6;
        locals.var_t2__blk324_dn7 = assign12510_e14434_d_n7;
        locals.var_t2__blk324_dn10 = assign12510_e14434_d_n10;
        locals.var_t2__blk324_dn11 = assign12510_e14434_d_n11;
        locals.var_t2__blk324_dn12 = assign12510_e14434_d_n12;
        locals.var_t2__blk324_dn17 = assign12510_e14434_d_n17;

        let (assign12520_e14451, assign12520_e14451_d_n0, assign12520_e14451_d_n2, assign12520_e14451_d_n6, assign12520_e14451_d_n7, assign12520_e14451_d_n10, assign12520_e14451_d_n11, assign12520_e14451_d_n12, assign12520_e14451_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard328 == 0.0)) {
        let assign12520_e14449: f64 = (locals.var_t2__blk324).sqrt();
        (assign12520_e14449, (locals.var_t2__blk324_dn0 / (2.0 * assign12520_e14449)), (locals.var_t2__blk324_dn2 / (2.0 * assign12520_e14449)), (locals.var_t2__blk324_dn6 / (2.0 * assign12520_e14449)), (locals.var_t2__blk324_dn7 / (2.0 * assign12520_e14449)), (locals.var_t2__blk324_dn10 / (2.0 * assign12520_e14449)), (locals.var_t2__blk324_dn11 / (2.0 * assign12520_e14449)), (locals.var_t2__blk324_dn12 / (2.0 * assign12520_e14449)), (locals.var_t2__blk324_dn17 / (2.0 * assign12520_e14449)),)
    } else {
        (locals.var_t2__blk324, locals.var_t2__blk324_dn0, locals.var_t2__blk324_dn2, locals.var_t2__blk324_dn6, locals.var_t2__blk324_dn7, locals.var_t2__blk324_dn10, locals.var_t2__blk324_dn11, locals.var_t2__blk324_dn12, locals.var_t2__blk324_dn17,)
    }
};
        locals.var_t2__blk324 = assign12520_e14451;
        locals.var_t2__blk324_dn0 = assign12520_e14451_d_n0;
        locals.var_t2__blk324_dn2 = assign12520_e14451_d_n2;
        locals.var_t2__blk324_dn6 = assign12520_e14451_d_n6;
        locals.var_t2__blk324_dn7 = assign12520_e14451_d_n7;
        locals.var_t2__blk324_dn10 = assign12520_e14451_d_n10;
        locals.var_t2__blk324_dn11 = assign12520_e14451_d_n11;
        locals.var_t2__blk324_dn12 = assign12520_e14451_d_n12;
        locals.var_t2__blk324_dn17 = assign12520_e14451_d_n17;

        let (assign12530_e14473, assign12530_e14473_d_n0, assign12530_e14473_d_n2, assign12530_e14473_d_n6, assign12530_e14473_d_n7, assign12530_e14473_d_n10, assign12530_e14473_d_n11, assign12530_e14473_d_n12, assign12530_e14473_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard328 == 0.0)) {
        let assign12530_e14467: f64 = (2.0 * locals.var_t1__blk323);
        let assign12530_e14470: f64 = (locals.var_t0__blk322 * locals.var_beta);
        let assign12530_e14471: f64 = (assign12530_e14467 + assign12530_e14470);
        (assign12530_e14471, (2.0 * locals.var_t1__blk323_dn0), (2.0 * locals.var_t1__blk323_dn2), (2.0 * locals.var_t1__blk323_dn6), (2.0 * locals.var_t1__blk323_dn7), ((2.0 * locals.var_t1__blk323_dn10) + ((locals.var_t0__blk322_dn10 * locals.var_beta) + (locals.var_t0__blk322 * locals.var_beta_dn10))), (2.0 * locals.var_t1__blk323_dn11), (2.0 * locals.var_t1__blk323_dn12), (2.0 * locals.var_t1__blk323_dn17),)
    } else {
        (locals.var_t3__blk325, locals.var_t3__blk325_dn0, locals.var_t3__blk325_dn2, locals.var_t3__blk325_dn6, locals.var_t3__blk325_dn7, locals.var_t3__blk325_dn10, locals.var_t3__blk325_dn11, locals.var_t3__blk325_dn12, locals.var_t3__blk325_dn17,)
    }
};
        locals.var_t3__blk325 = assign12530_e14473;
        locals.var_t3__blk325_dn0 = assign12530_e14473_d_n0;
        locals.var_t3__blk325_dn2 = assign12530_e14473_d_n2;
        locals.var_t3__blk325_dn6 = assign12530_e14473_d_n6;
        locals.var_t3__blk325_dn7 = assign12530_e14473_d_n7;
        locals.var_t3__blk325_dn10 = assign12530_e14473_d_n10;
        locals.var_t3__blk325_dn11 = assign12530_e14473_d_n11;
        locals.var_t3__blk325_dn12 = assign12530_e14473_d_n12;
        locals.var_t3__blk325_dn17 = assign12530_e14473_d_n17;

        let (assign12540_e14493, assign12540_e14493_d_n0, assign12540_e14493_d_n2, assign12540_e14493_d_n6, assign12540_e14493_d_n7, assign12540_e14493_d_n10, assign12540_e14493_d_n11, assign12540_e14493_d_n12, assign12540_e14493_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard328 == 0.0)) {
        let assign12540_e14489: f64 = (locals.var_t3__blk325 - locals.var_t2__blk324);
        let assign12540_e14491: f64 = (assign12540_e14489 / 2.0);
        (assign12540_e14491, ((locals.var_t3__blk325_dn0 - locals.var_t2__blk324_dn0) / 2.0), ((locals.var_t3__blk325_dn2 - locals.var_t2__blk324_dn2) / 2.0), ((locals.var_t3__blk325_dn6 - locals.var_t2__blk324_dn6) / 2.0), ((locals.var_t3__blk325_dn7 - locals.var_t2__blk324_dn7) / 2.0), ((locals.var_t3__blk325_dn10 - locals.var_t2__blk324_dn10) / 2.0), ((locals.var_t3__blk325_dn11 - locals.var_t2__blk324_dn11) / 2.0), ((locals.var_t3__blk325_dn12 - locals.var_t2__blk324_dn12) / 2.0), ((locals.var_t3__blk325_dn17 - locals.var_t2__blk324_dn17) / 2.0),)
    } else {
        (locals.var_psb_inia__blk326, locals.var_psb_inia__blk326_dn0, locals.var_psb_inia__blk326_dn2, locals.var_psb_inia__blk326_dn6, locals.var_psb_inia__blk326_dn7, locals.var_psb_inia__blk326_dn10, locals.var_psb_inia__blk326_dn11, locals.var_psb_inia__blk326_dn12, locals.var_psb_inia__blk326_dn17,)
    }
};
        locals.var_psb_inia__blk326 = assign12540_e14493;
        locals.var_psb_inia__blk326_dn0 = assign12540_e14493_d_n0;
        locals.var_psb_inia__blk326_dn2 = assign12540_e14493_d_n2;
        locals.var_psb_inia__blk326_dn6 = assign12540_e14493_d_n6;
        locals.var_psb_inia__blk326_dn7 = assign12540_e14493_d_n7;
        locals.var_psb_inia__blk326_dn10 = assign12540_e14493_d_n10;
        locals.var_psb_inia__blk326_dn11 = assign12540_e14493_d_n11;
        locals.var_psb_inia__blk326_dn12 = assign12540_e14493_d_n12;
        locals.var_psb_inia__blk326_dn17 = assign12540_e14493_d_n17;

        let (assign12550_e14522, assign12550_e14522_d_n0, assign12550_e14522_d_n2, assign12550_e14522_d_n6, assign12550_e14522_d_n7, assign12550_e14522_d_n10, assign12550_e14522_d_n11, assign12550_e14522_d_n12, assign12550_e14522_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard328 == 0.0)) {
        let assign12550_e14509: f64 = (locals.var_t1__blk323 * locals.var_t1__blk323);
        let assign12550_e14511: f64 = (assign12550_e14509 / locals.var_t0__blk322);
        let assign12550_e14513: f64 = (assign12550_e14511 / locals.var_cnst1bulk);
        let assign12550_e14514: f64 = (assign12550_e14513).ln();
        let assign12550_e14518: f64 = (2.0 / locals.var_t1__blk323);
        let assign12550_e14519: f64 = (locals.var_beta + assign12550_e14518);
        let assign12550_e14520: f64 = (assign12550_e14514 / assign12550_e14519);
        (assign12550_e14520, ((((((((((locals.var_t1__blk323_dn0 * locals.var_t1__blk323) + (locals.var_t1__blk323 * locals.var_t1__blk323_dn0)) / locals.var_t0__blk322) * locals.var_cnst1bulk) - (assign12550_e14511 * locals.var_cnst1bulk_dn0)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12550_e14513) * assign12550_e14519) - (assign12550_e14514 * (-((2.0 * locals.var_t1__blk323_dn0) / (locals.var_t1__blk323 * locals.var_t1__blk323))))) / (assign12550_e14519 * assign12550_e14519)), ((((((((((locals.var_t1__blk323_dn2 * locals.var_t1__blk323) + (locals.var_t1__blk323 * locals.var_t1__blk323_dn2)) / locals.var_t0__blk322) * locals.var_cnst1bulk) - (assign12550_e14511 * locals.var_cnst1bulk_dn2)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12550_e14513) * assign12550_e14519) - (assign12550_e14514 * (-((2.0 * locals.var_t1__blk323_dn2) / (locals.var_t1__blk323 * locals.var_t1__blk323))))) / (assign12550_e14519 * assign12550_e14519)), ((((((((((locals.var_t1__blk323_dn6 * locals.var_t1__blk323) + (locals.var_t1__blk323 * locals.var_t1__blk323_dn6)) / locals.var_t0__blk322) * locals.var_cnst1bulk) - (assign12550_e14511 * locals.var_cnst1bulk_dn6)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12550_e14513) * assign12550_e14519) - (assign12550_e14514 * (-((2.0 * locals.var_t1__blk323_dn6) / (locals.var_t1__blk323 * locals.var_t1__blk323))))) / (assign12550_e14519 * assign12550_e14519)), ((((((((((locals.var_t1__blk323_dn7 * locals.var_t1__blk323) + (locals.var_t1__blk323 * locals.var_t1__blk323_dn7)) / locals.var_t0__blk322) * locals.var_cnst1bulk) - (assign12550_e14511 * locals.var_cnst1bulk_dn7)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12550_e14513) * assign12550_e14519) - (assign12550_e14514 * (-((2.0 * locals.var_t1__blk323_dn7) / (locals.var_t1__blk323 * locals.var_t1__blk323))))) / (assign12550_e14519 * assign12550_e14519)), ((((((((((((locals.var_t1__blk323_dn10 * locals.var_t1__blk323) + (locals.var_t1__blk323 * locals.var_t1__blk323_dn10)) * locals.var_t0__blk322) - (assign12550_e14509 * locals.var_t0__blk322_dn10)) / (locals.var_t0__blk322 * locals.var_t0__blk322)) * locals.var_cnst1bulk) - (assign12550_e14511 * locals.var_cnst1bulk_dn10)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12550_e14513) * assign12550_e14519) - (assign12550_e14514 * (locals.var_beta_dn10 + (-((2.0 * locals.var_t1__blk323_dn10) / (locals.var_t1__blk323 * locals.var_t1__blk323)))))) / (assign12550_e14519 * assign12550_e14519)), ((((((((((locals.var_t1__blk323_dn11 * locals.var_t1__blk323) + (locals.var_t1__blk323 * locals.var_t1__blk323_dn11)) / locals.var_t0__blk322) * locals.var_cnst1bulk) - (assign12550_e14511 * locals.var_cnst1bulk_dn11)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12550_e14513) * assign12550_e14519) - (assign12550_e14514 * (-((2.0 * locals.var_t1__blk323_dn11) / (locals.var_t1__blk323 * locals.var_t1__blk323))))) / (assign12550_e14519 * assign12550_e14519)), ((((((((((locals.var_t1__blk323_dn12 * locals.var_t1__blk323) + (locals.var_t1__blk323 * locals.var_t1__blk323_dn12)) / locals.var_t0__blk322) * locals.var_cnst1bulk) - (assign12550_e14511 * locals.var_cnst1bulk_dn12)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12550_e14513) * assign12550_e14519) - (assign12550_e14514 * (-((2.0 * locals.var_t1__blk323_dn12) / (locals.var_t1__blk323 * locals.var_t1__blk323))))) / (assign12550_e14519 * assign12550_e14519)), ((((((((((locals.var_t1__blk323_dn17 * locals.var_t1__blk323) + (locals.var_t1__blk323 * locals.var_t1__blk323_dn17)) / locals.var_t0__blk322) * locals.var_cnst1bulk) - (assign12550_e14511 * locals.var_cnst1bulk_dn17)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign12550_e14513) * assign12550_e14519) - (assign12550_e14514 * (-((2.0 * locals.var_t1__blk323_dn17) / (locals.var_t1__blk323 * locals.var_t1__blk323))))) / (assign12550_e14519 * assign12550_e14519)),)
    } else {
        (locals.var_psb_inib__blk327, locals.var_psb_inib__blk327_dn0, locals.var_psb_inib__blk327_dn2, locals.var_psb_inib__blk327_dn6, locals.var_psb_inib__blk327_dn7, locals.var_psb_inib__blk327_dn10, locals.var_psb_inib__blk327_dn11, locals.var_psb_inib__blk327_dn12, locals.var_psb_inib__blk327_dn17,)
    }
};
        locals.var_psb_inib__blk327 = assign12550_e14522;
        locals.var_psb_inib__blk327_dn0 = assign12550_e14522_d_n0;
        locals.var_psb_inib__blk327_dn2 = assign12550_e14522_d_n2;
        locals.var_psb_inib__blk327_dn6 = assign12550_e14522_d_n6;
        locals.var_psb_inib__blk327_dn7 = assign12550_e14522_d_n7;
        locals.var_psb_inib__blk327_dn10 = assign12550_e14522_d_n10;
        locals.var_psb_inib__blk327_dn11 = assign12550_e14522_d_n11;
        locals.var_psb_inib__blk327_dn12 = assign12550_e14522_d_n12;
        locals.var_psb_inib__blk327_dn17 = assign12550_e14522_d_n17;

        let assign12560_e14525: f64 = if locals.var_psb_inia__blk326 < locals.var_pb2_bulk { 1.0 } else { 0.0 };
        locals.var_guard330 = assign12560_e14525;

        let (assign12570_e14543, assign12570_e14543_d_n0, assign12570_e14543_d_n2, assign12570_e14543_d_n6, assign12570_e14543_d_n7, assign12570_e14543_d_n10, assign12570_e14543_d_n11, assign12570_e14543_d_n12, assign12570_e14543_d_n17,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard328 == 0.0)) && (locals.var_guard330 != 0.0)) {
        (locals.var_psb_inia__blk326, locals.var_psb_inia__blk326_dn0, locals.var_psb_inia__blk326_dn2, locals.var_psb_inia__blk326_dn6, locals.var_psb_inia__blk326_dn7, locals.var_psb_inia__blk326_dn10, locals.var_psb_inia__blk326_dn11, locals.var_psb_inia__blk326_dn12, locals.var_psb_inia__blk326_dn17,)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
        locals.var_phi_sl_bulk = assign12570_e14543;
        locals.var_phi_sl_bulk_dn0 = assign12570_e14543_d_n0;
        locals.var_phi_sl_bulk_dn2 = assign12570_e14543_d_n2;
        locals.var_phi_sl_bulk_dn6 = assign12570_e14543_d_n6;
        locals.var_phi_sl_bulk_dn7 = assign12570_e14543_d_n7;
        locals.var_phi_sl_bulk_dn10 = assign12570_e14543_d_n10;
        locals.var_phi_sl_bulk_dn11 = assign12570_e14543_d_n11;
        locals.var_phi_sl_bulk_dn12 = assign12570_e14543_d_n12;
        locals.var_phi_sl_bulk_dn17 = assign12570_e14543_d_n17;

        let (assign12580_e14566, assign12580_e14566_d_n0, assign12580_e14566_d_n2, assign12580_e14566_d_n6, assign12580_e14566_d_n7, assign12580_e14566_d_n10, assign12580_e14566_d_n11, assign12580_e14566_d_n12, assign12580_e14566_d_n17,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard328 == 0.0)) && (locals.var_guard330 == 0.0)) {
        let assign12580_e14562: f64 = (locals.var_psb_inib__blk327 - locals.var_psb_inia__blk326);
        let assign12580_e14564: f64 = (assign12580_e14562 - 0.0008);
        (assign12580_e14564, (locals.var_psb_inib__blk327_dn0 - locals.var_psb_inia__blk326_dn0), (locals.var_psb_inib__blk327_dn2 - locals.var_psb_inia__blk326_dn2), (locals.var_psb_inib__blk327_dn6 - locals.var_psb_inia__blk326_dn6), (locals.var_psb_inib__blk327_dn7 - locals.var_psb_inia__blk326_dn7), (locals.var_psb_inib__blk327_dn10 - locals.var_psb_inia__blk326_dn10), (locals.var_psb_inib__blk327_dn11 - locals.var_psb_inia__blk326_dn11), (locals.var_psb_inib__blk327_dn12 - locals.var_psb_inia__blk326_dn12), (locals.var_psb_inib__blk327_dn17 - locals.var_psb_inia__blk326_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign12580_e14566;
        locals.var_tmf1_dn0 = assign12580_e14566_d_n0;
        locals.var_tmf1_dn2 = assign12580_e14566_d_n2;
        locals.var_tmf1_dn6 = assign12580_e14566_d_n6;
        locals.var_tmf1_dn7 = assign12580_e14566_d_n7;
        locals.var_tmf1_dn10 = assign12580_e14566_d_n10;
        locals.var_tmf1_dn11 = assign12580_e14566_d_n11;
        locals.var_tmf1_dn12 = assign12580_e14566_d_n12;
        locals.var_tmf1_dn17 = assign12580_e14566_d_n17;

        let (assign12590_e14589, assign12590_e14589_d_n0, assign12590_e14589_d_n2, assign12590_e14589_d_n6, assign12590_e14589_d_n7, assign12590_e14589_d_n10, assign12590_e14589_d_n11, assign12590_e14589_d_n12, assign12590_e14589_d_n17,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard328 == 0.0)) && (locals.var_guard330 == 0.0)) {
        let assign12590_e14585: f64 = (4.0 * locals.var_psb_inib__blk327);
        let assign12590_e14587: f64 = (assign12590_e14585 * 0.0008);
        (assign12590_e14587, ((4.0 * locals.var_psb_inib__blk327_dn0) * 0.0008), ((4.0 * locals.var_psb_inib__blk327_dn2) * 0.0008), ((4.0 * locals.var_psb_inib__blk327_dn6) * 0.0008), ((4.0 * locals.var_psb_inib__blk327_dn7) * 0.0008), ((4.0 * locals.var_psb_inib__blk327_dn10) * 0.0008), ((4.0 * locals.var_psb_inib__blk327_dn11) * 0.0008), ((4.0 * locals.var_psb_inib__blk327_dn12) * 0.0008), ((4.0 * locals.var_psb_inib__blk327_dn17) * 0.0008),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign12590_e14589;
        locals.var_tmf2_dn0 = assign12590_e14589_d_n0;
        locals.var_tmf2_dn2 = assign12590_e14589_d_n2;
        locals.var_tmf2_dn6 = assign12590_e14589_d_n6;
        locals.var_tmf2_dn7 = assign12590_e14589_d_n7;
        locals.var_tmf2_dn10 = assign12590_e14589_d_n10;
        locals.var_tmf2_dn11 = assign12590_e14589_d_n11;
        locals.var_tmf2_dn12 = assign12590_e14589_d_n12;
        locals.var_tmf2_dn17 = assign12590_e14589_d_n17;

        let (assign12600_e14614, assign12600_e14614_d_n0, assign12600_e14614_d_n2, assign12600_e14614_d_n6, assign12600_e14614_d_n7, assign12600_e14614_d_n10, assign12600_e14614_d_n11, assign12600_e14614_d_n12, assign12600_e14614_d_n17,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard328 == 0.0)) && (locals.var_guard330 == 0.0)) {
        let (assign12600_e14612, assign12600_e14612_d_n0, assign12600_e14612_d_n2, assign12600_e14612_d_n6, assign12600_e14612_d_n7, assign12600_e14612_d_n10, assign12600_e14612_d_n11, assign12600_e14612_d_n12, assign12600_e14612_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign12600_e14611: f64 = (-locals.var_tmf2);
                (assign12600_e14611, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign12600_e14612, assign12600_e14612_d_n0, assign12600_e14612_d_n2, assign12600_e14612_d_n6, assign12600_e14612_d_n7, assign12600_e14612_d_n10, assign12600_e14612_d_n11, assign12600_e14612_d_n12, assign12600_e14612_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign12600_e14614;
        locals.var_tmf2_dn0 = assign12600_e14614_d_n0;
        locals.var_tmf2_dn2 = assign12600_e14614_d_n2;
        locals.var_tmf2_dn6 = assign12600_e14614_d_n6;
        locals.var_tmf2_dn7 = assign12600_e14614_d_n7;
        locals.var_tmf2_dn10 = assign12600_e14614_d_n10;
        locals.var_tmf2_dn11 = assign12600_e14614_d_n11;
        locals.var_tmf2_dn12 = assign12600_e14614_d_n12;
        locals.var_tmf2_dn17 = assign12600_e14614_d_n17;

        let (assign12610_e14638, assign12610_e14638_d_n0, assign12610_e14638_d_n2, assign12610_e14638_d_n6, assign12610_e14638_d_n7, assign12610_e14638_d_n10, assign12610_e14638_d_n11, assign12610_e14638_d_n12, assign12610_e14638_d_n17,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard328 == 0.0)) && (locals.var_guard330 == 0.0)) {
        let assign12610_e14633: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign12610_e14635: f64 = (assign12610_e14633 + locals.var_tmf2);
        let assign12610_e14636: f64 = (assign12610_e14635).sqrt();
        (assign12610_e14636, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign12610_e14636)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign12610_e14636)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign12610_e14636)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign12610_e14636)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign12610_e14636)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign12610_e14636)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign12610_e14636)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign12610_e14636)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign12610_e14638;
        locals.var_tmf2_dn0 = assign12610_e14638_d_n0;
        locals.var_tmf2_dn2 = assign12610_e14638_d_n2;
        locals.var_tmf2_dn6 = assign12610_e14638_d_n6;
        locals.var_tmf2_dn7 = assign12610_e14638_d_n7;
        locals.var_tmf2_dn10 = assign12610_e14638_d_n10;
        locals.var_tmf2_dn11 = assign12610_e14638_d_n11;
        locals.var_tmf2_dn12 = assign12610_e14638_d_n12;
        locals.var_tmf2_dn17 = assign12610_e14638_d_n17;

        let (assign12620_e14663, assign12620_e14663_d_n0, assign12620_e14663_d_n2, assign12620_e14663_d_n6, assign12620_e14663_d_n7, assign12620_e14663_d_n10, assign12620_e14663_d_n11, assign12620_e14663_d_n12, assign12620_e14663_d_n17,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard328 == 0.0)) && (locals.var_guard330 == 0.0)) {
        let assign12620_e14659: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign12620_e14660: f64 = (0.5 * assign12620_e14659);
        let assign12620_e14661: f64 = (locals.var_psb_inib__blk327 - assign12620_e14660);
        (assign12620_e14661, (locals.var_psb_inib__blk327_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psb_inib__blk327_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psb_inib__blk327_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psb_inib__blk327_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psb_inib__blk327_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psb_inib__blk327_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psb_inib__blk327_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_psb_inib__blk327_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
        locals.var_phi_sl_bulk = assign12620_e14663;
        locals.var_phi_sl_bulk_dn0 = assign12620_e14663_d_n0;
        locals.var_phi_sl_bulk_dn2 = assign12620_e14663_d_n2;
        locals.var_phi_sl_bulk_dn6 = assign12620_e14663_d_n6;
        locals.var_phi_sl_bulk_dn7 = assign12620_e14663_d_n7;
        locals.var_phi_sl_bulk_dn10 = assign12620_e14663_d_n10;
        locals.var_phi_sl_bulk_dn11 = assign12620_e14663_d_n11;
        locals.var_phi_sl_bulk_dn12 = assign12620_e14663_d_n12;
        locals.var_phi_sl_bulk_dn17 = assign12620_e14663_d_n17;

        let (assign12630_e14684, assign12630_e14684_d_n0, assign12630_e14684_d_n2, assign12630_e14684_d_n6, assign12630_e14684_d_n7, assign12630_e14684_d_n10, assign12630_e14684_d_n11, assign12630_e14684_d_n12, assign12630_e14684_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) {
        let assign12630_e14676: f64 = (2.0 * 1.034943e-10);
        let assign12630_e14678: f64 = (assign12630_e14676 / 1.6021918e-19);
        let assign12630_e14680: f64 = (assign12630_e14678 * locals.var_phi_sl_soi);
        let assign12630_e14682: f64 = (assign12630_e14680 / locals.var_uc_nsubs);
        (assign12630_e14682, ((((assign12630_e14678 * locals.var_phi_sl_soi_dn0) * locals.var_uc_nsubs) - (assign12630_e14680 * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((assign12630_e14678 * locals.var_phi_sl_soi_dn2) * locals.var_uc_nsubs) - (assign12630_e14680 * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((assign12630_e14678 * locals.var_phi_sl_soi_dn6) * locals.var_uc_nsubs) - (assign12630_e14680 * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((assign12630_e14678 * locals.var_phi_sl_soi_dn7) * locals.var_uc_nsubs) - (assign12630_e14680 * locals.var_uc_nsubs_dn7)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((assign12630_e14678 * locals.var_phi_sl_soi_dn10) * locals.var_uc_nsubs) - (assign12630_e14680 * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((assign12630_e14678 * locals.var_phi_sl_soi_dn11) * locals.var_uc_nsubs) - (assign12630_e14680 * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((assign12630_e14678 * locals.var_phi_sl_soi_dn12) * locals.var_uc_nsubs) - (assign12630_e14680 * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((assign12630_e14678 * locals.var_phi_sl_soi_dn17) * locals.var_uc_nsubs) - (assign12630_e14680 * locals.var_uc_nsubs_dn17)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)),)
    } else {
        (locals.var_t0__blk331, locals.var_t0__blk331_dn0, locals.var_t0__blk331_dn2, locals.var_t0__blk331_dn6, locals.var_t0__blk331_dn7, locals.var_t0__blk331_dn10, locals.var_t0__blk331_dn11, locals.var_t0__blk331_dn12, locals.var_t0__blk331_dn17,)
    }
};
        locals.var_t0__blk331 = assign12630_e14684;
        locals.var_t0__blk331_dn0 = assign12630_e14684_d_n0;
        locals.var_t0__blk331_dn2 = assign12630_e14684_d_n2;
        locals.var_t0__blk331_dn6 = assign12630_e14684_d_n6;
        locals.var_t0__blk331_dn7 = assign12630_e14684_d_n7;
        locals.var_t0__blk331_dn10 = assign12630_e14684_d_n10;
        locals.var_t0__blk331_dn11 = assign12630_e14684_d_n11;
        locals.var_t0__blk331_dn12 = assign12630_e14684_d_n12;
        locals.var_t0__blk331_dn17 = assign12630_e14684_d_n17;

        let assign12640_e14687: f64 = if locals.var_t0__blk331 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard339 = assign12640_e14687;

        let (assign12650_e14711, assign12650_e14711_d_n0, assign12650_e14711_d_n2, assign12650_e14711_d_n6, assign12650_e14711_d_n7, assign12650_e14711_d_n10, assign12650_e14711_d_n11, assign12650_e14711_d_n12, assign12650_e14711_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard339 != 0.0)) {
        let assign12650_e14702: f64 = (2.0 * 1.034943e-10);
        let assign12650_e14704: f64 = (assign12650_e14702 / 1.6021918e-19);
        let assign12650_e14706: f64 = (assign12650_e14704 * locals.var_phi_sl_soi);
        let assign12650_e14708: f64 = (assign12650_e14706 / locals.var_uc_nsubs);
        let assign12650_e14709: f64 = (assign12650_e14708).sqrt();
        (assign12650_e14709, (((((assign12650_e14704 * locals.var_phi_sl_soi_dn0) * locals.var_uc_nsubs) - (assign12650_e14706 * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign12650_e14709)), (((((assign12650_e14704 * locals.var_phi_sl_soi_dn2) * locals.var_uc_nsubs) - (assign12650_e14706 * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign12650_e14709)), (((((assign12650_e14704 * locals.var_phi_sl_soi_dn6) * locals.var_uc_nsubs) - (assign12650_e14706 * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign12650_e14709)), (((((assign12650_e14704 * locals.var_phi_sl_soi_dn7) * locals.var_uc_nsubs) - (assign12650_e14706 * locals.var_uc_nsubs_dn7)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign12650_e14709)), (((((assign12650_e14704 * locals.var_phi_sl_soi_dn10) * locals.var_uc_nsubs) - (assign12650_e14706 * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign12650_e14709)), (((((assign12650_e14704 * locals.var_phi_sl_soi_dn11) * locals.var_uc_nsubs) - (assign12650_e14706 * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign12650_e14709)), (((((assign12650_e14704 * locals.var_phi_sl_soi_dn12) * locals.var_uc_nsubs) - (assign12650_e14706 * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign12650_e14709)), (((((assign12650_e14704 * locals.var_phi_sl_soi_dn17) * locals.var_uc_nsubs) - (assign12650_e14706 * locals.var_uc_nsubs_dn17)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign12650_e14709)),)
    } else {
        (locals.var_wdsoi, locals.var_wdsoi_dn0, locals.var_wdsoi_dn2, locals.var_wdsoi_dn6, locals.var_wdsoi_dn7, locals.var_wdsoi_dn10, locals.var_wdsoi_dn11, locals.var_wdsoi_dn12, locals.var_wdsoi_dn17,)
    }
};
        locals.var_wdsoi = assign12650_e14711;
        locals.var_wdsoi_dn0 = assign12650_e14711_d_n0;
        locals.var_wdsoi_dn2 = assign12650_e14711_d_n2;
        locals.var_wdsoi_dn6 = assign12650_e14711_d_n6;
        locals.var_wdsoi_dn7 = assign12650_e14711_d_n7;
        locals.var_wdsoi_dn10 = assign12650_e14711_d_n10;
        locals.var_wdsoi_dn11 = assign12650_e14711_d_n11;
        locals.var_wdsoi_dn12 = assign12650_e14711_d_n12;
        locals.var_wdsoi_dn17 = assign12650_e14711_d_n17;

        let (assign12660_e14727, assign12660_e14727_d_n0, assign12660_e14727_d_n2, assign12660_e14727_d_n6, assign12660_e14727_d_n7, assign12660_e14727_d_n10, assign12660_e14727_d_n11, assign12660_e14727_d_n12, assign12660_e14727_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard339 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wdsoi, locals.var_wdsoi_dn0, locals.var_wdsoi_dn2, locals.var_wdsoi_dn6, locals.var_wdsoi_dn7, locals.var_wdsoi_dn10, locals.var_wdsoi_dn11, locals.var_wdsoi_dn12, locals.var_wdsoi_dn17,)
    }
};
        locals.var_wdsoi = assign12660_e14727;
        locals.var_wdsoi_dn0 = assign12660_e14727_d_n0;
        locals.var_wdsoi_dn2 = assign12660_e14727_d_n2;
        locals.var_wdsoi_dn6 = assign12660_e14727_d_n6;
        locals.var_wdsoi_dn7 = assign12660_e14727_d_n7;
        locals.var_wdsoi_dn10 = assign12660_e14727_d_n10;
        locals.var_wdsoi_dn11 = assign12660_e14727_d_n11;
        locals.var_wdsoi_dn12 = assign12660_e14727_d_n12;
        locals.var_wdsoi_dn17 = assign12660_e14727_d_n17;

        let assign12670_e14732: f64 = if ((locals.var_phi_sl_soi < locals.var_fd_end) && (0.0 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard340 = assign12670_e14732;

        let (assign12690_e14762,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_lp_sl,)
    }
};
        locals.var_lp_sl = assign12690_e14762;

    }

    pub(super) fn stamp_transient_block_37(
        locals: &mut StampLocals,
    ) {
        let mut assign12700_loop_guard: usize = 0;
        while {
            let assign12700_cond_e14778: f64 = if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 != 0.0)) && (locals.var_lp_sl < locals.var_lp_sl_max)) { 1.0 } else { 0.0 };
            assign12700_cond_e14778 != 0.0
        } {
            assign12700_loop_guard += 1;
            assert!(assign12700_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign12700_body0_e14793, assign12700_body0_e14793_d_n10,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 != 0.0)) {
        (locals.var_cnst0bulk, locals.var_cnst0bulk_dn10,)
    } else {
        (locals.var_t1__blk332, locals.var_t1__blk332_dn10,)
    }
};
            locals.var_t1__blk332 = assign12700_body0_e14793;
            locals.var_t1__blk332_dn10 = assign12700_body0_e14793_d_n10;
            let (assign12700_body1_e14810, assign12700_body1_e14810_d_n0, assign12700_body1_e14810_d_n2, assign12700_body1_e14810_d_n6, assign12700_body1_e14810_d_n7, assign12700_body1_e14810_d_n10, assign12700_body1_e14810_d_n11, assign12700_body1_e14810_d_n12, assign12700_body1_e14810_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 != 0.0)) {
        let assign12700_body1_e14808: f64 = (locals.var_beta * locals.var_phi_sl_bulk);
        (assign12700_body1_e14808, (locals.var_beta * locals.var_phi_sl_bulk_dn0), (locals.var_beta * locals.var_phi_sl_bulk_dn2), (locals.var_beta * locals.var_phi_sl_bulk_dn6), (locals.var_beta * locals.var_phi_sl_bulk_dn7), ((locals.var_beta_dn10 * locals.var_phi_sl_bulk) + (locals.var_beta * locals.var_phi_sl_bulk_dn10)), (locals.var_beta * locals.var_phi_sl_bulk_dn11), (locals.var_beta * locals.var_phi_sl_bulk_dn12), (locals.var_beta * locals.var_phi_sl_bulk_dn17),)
    } else {
        (locals.var_t2__blk333, locals.var_t2__blk333_dn0, locals.var_t2__blk333_dn2, locals.var_t2__blk333_dn6, locals.var_t2__blk333_dn7, locals.var_t2__blk333_dn10, locals.var_t2__blk333_dn11, locals.var_t2__blk333_dn12, locals.var_t2__blk333_dn17,)
    }
};
            locals.var_t2__blk333 = assign12700_body1_e14810;
            locals.var_t2__blk333_dn0 = assign12700_body1_e14810_d_n0;
            locals.var_t2__blk333_dn2 = assign12700_body1_e14810_d_n2;
            locals.var_t2__blk333_dn6 = assign12700_body1_e14810_d_n6;
            locals.var_t2__blk333_dn7 = assign12700_body1_e14810_d_n7;
            locals.var_t2__blk333_dn10 = assign12700_body1_e14810_d_n10;
            locals.var_t2__blk333_dn11 = assign12700_body1_e14810_d_n11;
            locals.var_t2__blk333_dn12 = assign12700_body1_e14810_d_n12;
            locals.var_t2__blk333_dn17 = assign12700_body1_e14810_d_n17;
            let (assign12700_body2_e14827, assign12700_body2_e14827_d_n0, assign12700_body2_e14827_d_n2, assign12700_body2_e14827_d_n6, assign12700_body2_e14827_d_n7, assign12700_body2_e14827_d_n10, assign12700_body2_e14827_d_n11, assign12700_body2_e14827_d_n12, assign12700_body2_e14827_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 != 0.0)) {
        let assign12700_body2_e14824: f64 = (-locals.var_t2__blk333);
        let assign12700_body2_e14825: f64 = (assign12700_body2_e14824).exp();
        (assign12700_body2_e14825, (assign12700_body2_e14825 * (-locals.var_t2__blk333_dn0)), (assign12700_body2_e14825 * (-locals.var_t2__blk333_dn2)), (assign12700_body2_e14825 * (-locals.var_t2__blk333_dn6)), (assign12700_body2_e14825 * (-locals.var_t2__blk333_dn7)), (assign12700_body2_e14825 * (-locals.var_t2__blk333_dn10)), (assign12700_body2_e14825 * (-locals.var_t2__blk333_dn11)), (assign12700_body2_e14825 * (-locals.var_t2__blk333_dn12)), (assign12700_body2_e14825 * (-locals.var_t2__blk333_dn17)),)
    } else {
        (locals.var_t3__blk334, locals.var_t3__blk334_dn0, locals.var_t3__blk334_dn2, locals.var_t3__blk334_dn6, locals.var_t3__blk334_dn7, locals.var_t3__blk334_dn10, locals.var_t3__blk334_dn11, locals.var_t3__blk334_dn12, locals.var_t3__blk334_dn17,)
    }
};
            locals.var_t3__blk334 = assign12700_body2_e14827;
            locals.var_t3__blk334_dn0 = assign12700_body2_e14827_d_n0;
            locals.var_t3__blk334_dn2 = assign12700_body2_e14827_d_n2;
            locals.var_t3__blk334_dn6 = assign12700_body2_e14827_d_n6;
            locals.var_t3__blk334_dn7 = assign12700_body2_e14827_d_n7;
            locals.var_t3__blk334_dn10 = assign12700_body2_e14827_d_n10;
            locals.var_t3__blk334_dn11 = assign12700_body2_e14827_d_n11;
            locals.var_t3__blk334_dn12 = assign12700_body2_e14827_d_n12;
            locals.var_t3__blk334_dn17 = assign12700_body2_e14827_d_n17;
            let assign12700_body3_e14830: f64 = if locals.var_phi_sl_bulk > 1e-9 { 1.0 } else { 0.0 };
            locals.var_guard341 = assign12700_body3_e14830;
            let (assign12700_body4_e14850, assign12700_body4_e14850_d_n0, assign12700_body4_e14850_d_n2, assign12700_body4_e14850_d_n6, assign12700_body4_e14850_d_n7, assign12700_body4_e14850_d_n10, assign12700_body4_e14850_d_n11, assign12700_body4_e14850_d_n12, assign12700_body4_e14850_d_n17,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 != 0.0)) && (locals.var_guard341 != 0.0)) {
        let assign12700_body4_e14847: f64 = (locals.var_beta * locals.var_phi_sl_bulk);
        let assign12700_body4_e14848: f64 = (assign12700_body4_e14847).exp();
        (assign12700_body4_e14848, (assign12700_body4_e14848 * (locals.var_beta * locals.var_phi_sl_bulk_dn0)), (assign12700_body4_e14848 * (locals.var_beta * locals.var_phi_sl_bulk_dn2)), (assign12700_body4_e14848 * (locals.var_beta * locals.var_phi_sl_bulk_dn6)), (assign12700_body4_e14848 * (locals.var_beta * locals.var_phi_sl_bulk_dn7)), (assign12700_body4_e14848 * ((locals.var_beta_dn10 * locals.var_phi_sl_bulk) + (locals.var_beta * locals.var_phi_sl_bulk_dn10))), (assign12700_body4_e14848 * (locals.var_beta * locals.var_phi_sl_bulk_dn11)), (assign12700_body4_e14848 * (locals.var_beta * locals.var_phi_sl_bulk_dn12)), (assign12700_body4_e14848 * (locals.var_beta * locals.var_phi_sl_bulk_dn17)),)
    } else {
        (locals.var_t0__blk331, locals.var_t0__blk331_dn0, locals.var_t0__blk331_dn2, locals.var_t0__blk331_dn6, locals.var_t0__blk331_dn7, locals.var_t0__blk331_dn10, locals.var_t0__blk331_dn11, locals.var_t0__blk331_dn12, locals.var_t0__blk331_dn17,)
    }
};
            locals.var_t0__blk331 = assign12700_body4_e14850;
            locals.var_t0__blk331_dn0 = assign12700_body4_e14850_d_n0;
            locals.var_t0__blk331_dn2 = assign12700_body4_e14850_d_n2;
            locals.var_t0__blk331_dn6 = assign12700_body4_e14850_d_n6;
            locals.var_t0__blk331_dn7 = assign12700_body4_e14850_d_n7;
            locals.var_t0__blk331_dn10 = assign12700_body4_e14850_d_n10;
            locals.var_t0__blk331_dn11 = assign12700_body4_e14850_d_n11;
            locals.var_t0__blk331_dn12 = assign12700_body4_e14850_d_n12;
            locals.var_t0__blk331_dn17 = assign12700_body4_e14850_d_n17;
            let (assign12700_body5_e14881, assign12700_body5_e14881_d_n0, assign12700_body5_e14881_d_n2, assign12700_body5_e14881_d_n6, assign12700_body5_e14881_d_n7, assign12700_body5_e14881_d_n10, assign12700_body5_e14881_d_n11, assign12700_body5_e14881_d_n12, assign12700_body5_e14881_d_n17,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 != 0.0)) && (locals.var_guard341 != 0.0)) {
        let assign12700_body5_e14866: f64 = (-locals.var_t1__blk332);
        let assign12700_body5_e14869: f64 = (locals.var_t3__blk334 + locals.var_t2__blk333);
        let assign12700_body5_e14871: f64 = (assign12700_body5_e14869 - 1.0);
        let assign12700_body5_e14875: f64 = (locals.var_t0__blk331 - 1.0);
        let assign12700_body5_e14876: f64 = (locals.var_cnst1bulk * assign12700_body5_e14875);
        let assign12700_body5_e14877: f64 = (assign12700_body5_e14871 + assign12700_body5_e14876);
        let assign12700_body5_e14878: f64 = (assign12700_body5_e14877).sqrt();
        let assign12700_body5_e14879: f64 = (assign12700_body5_e14866 * assign12700_body5_e14878);
        (assign12700_body5_e14879, (assign12700_body5_e14866 * (((locals.var_t3__blk334_dn0 + locals.var_t2__blk333_dn0) + ((locals.var_cnst1bulk_dn0 * assign12700_body5_e14875) + (locals.var_cnst1bulk * locals.var_t0__blk331_dn0))) / (2.0 * assign12700_body5_e14878))), (assign12700_body5_e14866 * (((locals.var_t3__blk334_dn2 + locals.var_t2__blk333_dn2) + ((locals.var_cnst1bulk_dn2 * assign12700_body5_e14875) + (locals.var_cnst1bulk * locals.var_t0__blk331_dn2))) / (2.0 * assign12700_body5_e14878))), (assign12700_body5_e14866 * (((locals.var_t3__blk334_dn6 + locals.var_t2__blk333_dn6) + ((locals.var_cnst1bulk_dn6 * assign12700_body5_e14875) + (locals.var_cnst1bulk * locals.var_t0__blk331_dn6))) / (2.0 * assign12700_body5_e14878))), (assign12700_body5_e14866 * (((locals.var_t3__blk334_dn7 + locals.var_t2__blk333_dn7) + ((locals.var_cnst1bulk_dn7 * assign12700_body5_e14875) + (locals.var_cnst1bulk * locals.var_t0__blk331_dn7))) / (2.0 * assign12700_body5_e14878))), (((-locals.var_t1__blk332_dn10) * assign12700_body5_e14878) + (assign12700_body5_e14866 * (((locals.var_t3__blk334_dn10 + locals.var_t2__blk333_dn10) + ((locals.var_cnst1bulk_dn10 * assign12700_body5_e14875) + (locals.var_cnst1bulk * locals.var_t0__blk331_dn10))) / (2.0 * assign12700_body5_e14878)))), (assign12700_body5_e14866 * (((locals.var_t3__blk334_dn11 + locals.var_t2__blk333_dn11) + ((locals.var_cnst1bulk_dn11 * assign12700_body5_e14875) + (locals.var_cnst1bulk * locals.var_t0__blk331_dn11))) / (2.0 * assign12700_body5_e14878))), (assign12700_body5_e14866 * (((locals.var_t3__blk334_dn12 + locals.var_t2__blk333_dn12) + ((locals.var_cnst1bulk_dn12 * assign12700_body5_e14875) + (locals.var_cnst1bulk * locals.var_t0__blk331_dn12))) / (2.0 * assign12700_body5_e14878))), (assign12700_body5_e14866 * (((locals.var_t3__blk334_dn17 + locals.var_t2__blk333_dn17) + ((locals.var_cnst1bulk_dn17 * assign12700_body5_e14875) + (locals.var_cnst1bulk * locals.var_t0__blk331_dn17))) / (2.0 * assign12700_body5_e14878))),)
    } else {
        (locals.var_t4__blk335, locals.var_t4__blk335_dn0, locals.var_t4__blk335_dn2, locals.var_t4__blk335_dn6, locals.var_t4__blk335_dn7, locals.var_t4__blk335_dn10, locals.var_t4__blk335_dn11, locals.var_t4__blk335_dn12, locals.var_t4__blk335_dn17,)
    }
};
            locals.var_t4__blk335 = assign12700_body5_e14881;
            locals.var_t4__blk335_dn0 = assign12700_body5_e14881_d_n0;
            locals.var_t4__blk335_dn2 = assign12700_body5_e14881_d_n2;
            locals.var_t4__blk335_dn6 = assign12700_body5_e14881_d_n6;
            locals.var_t4__blk335_dn7 = assign12700_body5_e14881_d_n7;
            locals.var_t4__blk335_dn10 = assign12700_body5_e14881_d_n10;
            locals.var_t4__blk335_dn11 = assign12700_body5_e14881_d_n11;
            locals.var_t4__blk335_dn12 = assign12700_body5_e14881_d_n12;
            locals.var_t4__blk335_dn17 = assign12700_body5_e14881_d_n17;
            let (assign12700_body6_e14909, assign12700_body6_e14909_d_n0, assign12700_body6_e14909_d_n2, assign12700_body6_e14909_d_n6, assign12700_body6_e14909_d_n7, assign12700_body6_e14909_d_n10, assign12700_body6_e14909_d_n11, assign12700_body6_e14909_d_n12, assign12700_body6_e14909_d_n17,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 != 0.0)) && (locals.var_guard341 != 0.0)) {
        let assign12700_body6_e14898: f64 = (locals.var_c0bulk / locals.var_t4__blk335);
        let assign12700_body6_e14900: f64 = (-locals.var_t3__blk334);
        let assign12700_body6_e14902: f64 = (assign12700_body6_e14900 + 1.0);
        let assign12700_body6_e14905: f64 = (locals.var_cnst1bulk * locals.var_t0__blk331);
        let assign12700_body6_e14906: f64 = (assign12700_body6_e14902 + assign12700_body6_e14905);
        let assign12700_body6_e14907: f64 = (assign12700_body6_e14898 * assign12700_body6_e14906);
        (assign12700_body6_e14907, (((-((locals.var_c0bulk * locals.var_t4__blk335_dn0) / (locals.var_t4__blk335 * locals.var_t4__blk335))) * assign12700_body6_e14906) + (assign12700_body6_e14898 * ((-locals.var_t3__blk334_dn0) + ((locals.var_cnst1bulk_dn0 * locals.var_t0__blk331) + (locals.var_cnst1bulk * locals.var_t0__blk331_dn0))))), (((-((locals.var_c0bulk * locals.var_t4__blk335_dn2) / (locals.var_t4__blk335 * locals.var_t4__blk335))) * assign12700_body6_e14906) + (assign12700_body6_e14898 * ((-locals.var_t3__blk334_dn2) + ((locals.var_cnst1bulk_dn2 * locals.var_t0__blk331) + (locals.var_cnst1bulk * locals.var_t0__blk331_dn2))))), (((-((locals.var_c0bulk * locals.var_t4__blk335_dn6) / (locals.var_t4__blk335 * locals.var_t4__blk335))) * assign12700_body6_e14906) + (assign12700_body6_e14898 * ((-locals.var_t3__blk334_dn6) + ((locals.var_cnst1bulk_dn6 * locals.var_t0__blk331) + (locals.var_cnst1bulk * locals.var_t0__blk331_dn6))))), (((-((locals.var_c0bulk * locals.var_t4__blk335_dn7) / (locals.var_t4__blk335 * locals.var_t4__blk335))) * assign12700_body6_e14906) + (assign12700_body6_e14898 * ((-locals.var_t3__blk334_dn7) + ((locals.var_cnst1bulk_dn7 * locals.var_t0__blk331) + (locals.var_cnst1bulk * locals.var_t0__blk331_dn7))))), (((-((locals.var_c0bulk * locals.var_t4__blk335_dn10) / (locals.var_t4__blk335 * locals.var_t4__blk335))) * assign12700_body6_e14906) + (assign12700_body6_e14898 * ((-locals.var_t3__blk334_dn10) + ((locals.var_cnst1bulk_dn10 * locals.var_t0__blk331) + (locals.var_cnst1bulk * locals.var_t0__blk331_dn10))))), (((-((locals.var_c0bulk * locals.var_t4__blk335_dn11) / (locals.var_t4__blk335 * locals.var_t4__blk335))) * assign12700_body6_e14906) + (assign12700_body6_e14898 * ((-locals.var_t3__blk334_dn11) + ((locals.var_cnst1bulk_dn11 * locals.var_t0__blk331) + (locals.var_cnst1bulk * locals.var_t0__blk331_dn11))))), (((-((locals.var_c0bulk * locals.var_t4__blk335_dn12) / (locals.var_t4__blk335 * locals.var_t4__blk335))) * assign12700_body6_e14906) + (assign12700_body6_e14898 * ((-locals.var_t3__blk334_dn12) + ((locals.var_cnst1bulk_dn12 * locals.var_t0__blk331) + (locals.var_cnst1bulk * locals.var_t0__blk331_dn12))))), (((-((locals.var_c0bulk * locals.var_t4__blk335_dn17) / (locals.var_t4__blk335 * locals.var_t4__blk335))) * assign12700_body6_e14906) + (assign12700_body6_e14898 * ((-locals.var_t3__blk334_dn17) + ((locals.var_cnst1bulk_dn17 * locals.var_t0__blk331) + (locals.var_cnst1bulk * locals.var_t0__blk331_dn17))))),)
    } else {
        (locals.var_t5__blk336, locals.var_t5__blk336_dn0, locals.var_t5__blk336_dn2, locals.var_t5__blk336_dn6, locals.var_t5__blk336_dn7, locals.var_t5__blk336_dn10, locals.var_t5__blk336_dn11, locals.var_t5__blk336_dn12, locals.var_t5__blk336_dn17,)
    }
};
            locals.var_t5__blk336 = assign12700_body6_e14909;
            locals.var_t5__blk336_dn0 = assign12700_body6_e14909_d_n0;
            locals.var_t5__blk336_dn2 = assign12700_body6_e14909_d_n2;
            locals.var_t5__blk336_dn6 = assign12700_body6_e14909_d_n6;
            locals.var_t5__blk336_dn7 = assign12700_body6_e14909_d_n7;
            locals.var_t5__blk336_dn10 = assign12700_body6_e14909_d_n10;
            locals.var_t5__blk336_dn11 = assign12700_body6_e14909_d_n11;
            locals.var_t5__blk336_dn12 = assign12700_body6_e14909_d_n12;
            locals.var_t5__blk336_dn17 = assign12700_body6_e14909_d_n17;
            let assign12700_body7_e14912: f64 = (-1e-9);
            let assign12700_body7_e14913: f64 = if locals.var_phi_sl_bulk < assign12700_body7_e14912 { 1.0 } else { 0.0 };
            locals.var_guard342 = assign12700_body7_e14913;
            let (assign12700_body8_e14940, assign12700_body8_e14940_d_n0, assign12700_body8_e14940_d_n2, assign12700_body8_e14940_d_n6, assign12700_body8_e14940_d_n7, assign12700_body8_e14940_d_n10, assign12700_body8_e14940_d_n11, assign12700_body8_e14940_d_n12, assign12700_body8_e14940_d_n17,) = {
    if (((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 != 0.0)) && (locals.var_guard341 == 0.0)) && (locals.var_guard342 != 0.0)) {
        let assign12700_body8_e14934: f64 = (locals.var_t3__blk334 + locals.var_t2__blk333);
        let assign12700_body8_e14936: f64 = (assign12700_body8_e14934 - 1.0);
        let assign12700_body8_e14937: f64 = (assign12700_body8_e14936).sqrt();
        let assign12700_body8_e14938: f64 = (locals.var_t1__blk332 * assign12700_body8_e14937);
        (assign12700_body8_e14938, (locals.var_t1__blk332 * ((locals.var_t3__blk334_dn0 + locals.var_t2__blk333_dn0) / (2.0 * assign12700_body8_e14937))), (locals.var_t1__blk332 * ((locals.var_t3__blk334_dn2 + locals.var_t2__blk333_dn2) / (2.0 * assign12700_body8_e14937))), (locals.var_t1__blk332 * ((locals.var_t3__blk334_dn6 + locals.var_t2__blk333_dn6) / (2.0 * assign12700_body8_e14937))), (locals.var_t1__blk332 * ((locals.var_t3__blk334_dn7 + locals.var_t2__blk333_dn7) / (2.0 * assign12700_body8_e14937))), ((locals.var_t1__blk332_dn10 * assign12700_body8_e14937) + (locals.var_t1__blk332 * ((locals.var_t3__blk334_dn10 + locals.var_t2__blk333_dn10) / (2.0 * assign12700_body8_e14937)))), (locals.var_t1__blk332 * ((locals.var_t3__blk334_dn11 + locals.var_t2__blk333_dn11) / (2.0 * assign12700_body8_e14937))), (locals.var_t1__blk332 * ((locals.var_t3__blk334_dn12 + locals.var_t2__blk333_dn12) / (2.0 * assign12700_body8_e14937))), (locals.var_t1__blk332 * ((locals.var_t3__blk334_dn17 + locals.var_t2__blk333_dn17) / (2.0 * assign12700_body8_e14937))),)
    } else {
        (locals.var_t4__blk335, locals.var_t4__blk335_dn0, locals.var_t4__blk335_dn2, locals.var_t4__blk335_dn6, locals.var_t4__blk335_dn7, locals.var_t4__blk335_dn10, locals.var_t4__blk335_dn11, locals.var_t4__blk335_dn12, locals.var_t4__blk335_dn17,)
    }
};
            locals.var_t4__blk335 = assign12700_body8_e14940;
            locals.var_t4__blk335_dn0 = assign12700_body8_e14940_d_n0;
            locals.var_t4__blk335_dn2 = assign12700_body8_e14940_d_n2;
            locals.var_t4__blk335_dn6 = assign12700_body8_e14940_d_n6;
            locals.var_t4__blk335_dn7 = assign12700_body8_e14940_d_n7;
            locals.var_t4__blk335_dn10 = assign12700_body8_e14940_d_n10;
            locals.var_t4__blk335_dn11 = assign12700_body8_e14940_d_n11;
            locals.var_t4__blk335_dn12 = assign12700_body8_e14940_d_n12;
            locals.var_t4__blk335_dn17 = assign12700_body8_e14940_d_n17;
            let (assign12700_body9_e14967, assign12700_body9_e14967_d_n0, assign12700_body9_e14967_d_n2, assign12700_body9_e14967_d_n6, assign12700_body9_e14967_d_n7, assign12700_body9_e14967_d_n10, assign12700_body9_e14967_d_n11, assign12700_body9_e14967_d_n12, assign12700_body9_e14967_d_n17,) = {
    if (((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 != 0.0)) && (locals.var_guard341 == 0.0)) && (locals.var_guard342 != 0.0)) {
        let assign12700_body9_e14960: f64 = (locals.var_c0bulk / locals.var_t4__blk335);
        let assign12700_body9_e14962: f64 = (-locals.var_t3__blk334);
        let assign12700_body9_e14964: f64 = (assign12700_body9_e14962 + 1.0);
        let assign12700_body9_e14965: f64 = (assign12700_body9_e14960 * assign12700_body9_e14964);
        (assign12700_body9_e14965, (((-((locals.var_c0bulk * locals.var_t4__blk335_dn0) / (locals.var_t4__blk335 * locals.var_t4__blk335))) * assign12700_body9_e14964) + (assign12700_body9_e14960 * (-locals.var_t3__blk334_dn0))), (((-((locals.var_c0bulk * locals.var_t4__blk335_dn2) / (locals.var_t4__blk335 * locals.var_t4__blk335))) * assign12700_body9_e14964) + (assign12700_body9_e14960 * (-locals.var_t3__blk334_dn2))), (((-((locals.var_c0bulk * locals.var_t4__blk335_dn6) / (locals.var_t4__blk335 * locals.var_t4__blk335))) * assign12700_body9_e14964) + (assign12700_body9_e14960 * (-locals.var_t3__blk334_dn6))), (((-((locals.var_c0bulk * locals.var_t4__blk335_dn7) / (locals.var_t4__blk335 * locals.var_t4__blk335))) * assign12700_body9_e14964) + (assign12700_body9_e14960 * (-locals.var_t3__blk334_dn7))), (((-((locals.var_c0bulk * locals.var_t4__blk335_dn10) / (locals.var_t4__blk335 * locals.var_t4__blk335))) * assign12700_body9_e14964) + (assign12700_body9_e14960 * (-locals.var_t3__blk334_dn10))), (((-((locals.var_c0bulk * locals.var_t4__blk335_dn11) / (locals.var_t4__blk335 * locals.var_t4__blk335))) * assign12700_body9_e14964) + (assign12700_body9_e14960 * (-locals.var_t3__blk334_dn11))), (((-((locals.var_c0bulk * locals.var_t4__blk335_dn12) / (locals.var_t4__blk335 * locals.var_t4__blk335))) * assign12700_body9_e14964) + (assign12700_body9_e14960 * (-locals.var_t3__blk334_dn12))), (((-((locals.var_c0bulk * locals.var_t4__blk335_dn17) / (locals.var_t4__blk335 * locals.var_t4__blk335))) * assign12700_body9_e14964) + (assign12700_body9_e14960 * (-locals.var_t3__blk334_dn17))),)
    } else {
        (locals.var_t5__blk336, locals.var_t5__blk336_dn0, locals.var_t5__blk336_dn2, locals.var_t5__blk336_dn6, locals.var_t5__blk336_dn7, locals.var_t5__blk336_dn10, locals.var_t5__blk336_dn11, locals.var_t5__blk336_dn12, locals.var_t5__blk336_dn17,)
    }
};
            locals.var_t5__blk336 = assign12700_body9_e14967;
            locals.var_t5__blk336_dn0 = assign12700_body9_e14967_d_n0;
            locals.var_t5__blk336_dn2 = assign12700_body9_e14967_d_n2;
            locals.var_t5__blk336_dn6 = assign12700_body9_e14967_d_n6;
            locals.var_t5__blk336_dn7 = assign12700_body9_e14967_d_n7;
            locals.var_t5__blk336_dn10 = assign12700_body9_e14967_d_n10;
            locals.var_t5__blk336_dn11 = assign12700_body9_e14967_d_n11;
            locals.var_t5__blk336_dn12 = assign12700_body9_e14967_d_n12;
            locals.var_t5__blk336_dn17 = assign12700_body9_e14967_d_n17;
            let (assign12700_body10_e14996, assign12700_body10_e14996_d_n0, assign12700_body10_e14996_d_n2, assign12700_body10_e14996_d_n6, assign12700_body10_e14996_d_n7, assign12700_body10_e14996_d_n10, assign12700_body10_e14996_d_n11, assign12700_body10_e14996_d_n12, assign12700_body10_e14996_d_n17,) = {
    if (((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 != 0.0)) && (locals.var_guard341 == 0.0)) && (locals.var_guard342 == 0.0)) {
        let assign12700_body10_e14988: f64 = (locals.var_c0bulk / locals.var_beta);
        let assign12700_body10_e14989: f64 = (assign12700_body10_e14988).sqrt();
        let assign12700_body10_e14990: f64 = (-assign12700_body10_e14989);
        let assign12700_body10_e14992: f64 = (assign12700_body10_e14990 * locals.var_beta);
        let assign12700_body10_e14994: f64 = (assign12700_body10_e14992 * locals.var_phi_sl_bulk);
        (assign12700_body10_e14994, (assign12700_body10_e14992 * locals.var_phi_sl_bulk_dn0), (assign12700_body10_e14992 * locals.var_phi_sl_bulk_dn2), (assign12700_body10_e14992 * locals.var_phi_sl_bulk_dn6), (assign12700_body10_e14992 * locals.var_phi_sl_bulk_dn7), (((((-((-((locals.var_c0bulk * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) / (2.0 * assign12700_body10_e14989))) * locals.var_beta) + (assign12700_body10_e14990 * locals.var_beta_dn10)) * locals.var_phi_sl_bulk) + (assign12700_body10_e14992 * locals.var_phi_sl_bulk_dn10)), (assign12700_body10_e14992 * locals.var_phi_sl_bulk_dn11), (assign12700_body10_e14992 * locals.var_phi_sl_bulk_dn12), (assign12700_body10_e14992 * locals.var_phi_sl_bulk_dn17),)
    } else {
        (locals.var_t4__blk335, locals.var_t4__blk335_dn0, locals.var_t4__blk335_dn2, locals.var_t4__blk335_dn6, locals.var_t4__blk335_dn7, locals.var_t4__blk335_dn10, locals.var_t4__blk335_dn11, locals.var_t4__blk335_dn12, locals.var_t4__blk335_dn17,)
    }
};
            locals.var_t4__blk335 = assign12700_body10_e14996;
            locals.var_t4__blk335_dn0 = assign12700_body10_e14996_d_n0;
            locals.var_t4__blk335_dn2 = assign12700_body10_e14996_d_n2;
            locals.var_t4__blk335_dn6 = assign12700_body10_e14996_d_n6;
            locals.var_t4__blk335_dn7 = assign12700_body10_e14996_d_n7;
            locals.var_t4__blk335_dn10 = assign12700_body10_e14996_d_n10;
            locals.var_t4__blk335_dn11 = assign12700_body10_e14996_d_n11;
            locals.var_t4__blk335_dn12 = assign12700_body10_e14996_d_n12;
            locals.var_t4__blk335_dn17 = assign12700_body10_e14996_d_n17;
            let (assign12700_body11_e15021, assign12700_body11_e15021_d_n0, assign12700_body11_e15021_d_n2, assign12700_body11_e15021_d_n6, assign12700_body11_e15021_d_n7, assign12700_body11_e15021_d_n10, assign12700_body11_e15021_d_n11, assign12700_body11_e15021_d_n12, assign12700_body11_e15021_d_n17,) = {
    if (((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 != 0.0)) && (locals.var_guard341 == 0.0)) && (locals.var_guard342 == 0.0)) {
        let assign12700_body11_e15017: f64 = (locals.var_c0bulk * locals.var_beta);
        let assign12700_body11_e15018: f64 = (assign12700_body11_e15017).sqrt();
        let assign12700_body11_e15019: f64 = (-assign12700_body11_e15018);
        (assign12700_body11_e15019, 0.0, 0.0, 0.0, 0.0, (-((locals.var_c0bulk * locals.var_beta_dn10) / (2.0 * assign12700_body11_e15018))), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5__blk336, locals.var_t5__blk336_dn0, locals.var_t5__blk336_dn2, locals.var_t5__blk336_dn6, locals.var_t5__blk336_dn7, locals.var_t5__blk336_dn10, locals.var_t5__blk336_dn11, locals.var_t5__blk336_dn12, locals.var_t5__blk336_dn17,)
    }
};
            locals.var_t5__blk336 = assign12700_body11_e15021;
            locals.var_t5__blk336_dn0 = assign12700_body11_e15021_d_n0;
            locals.var_t5__blk336_dn2 = assign12700_body11_e15021_d_n2;
            locals.var_t5__blk336_dn6 = assign12700_body11_e15021_d_n6;
            locals.var_t5__blk336_dn7 = assign12700_body11_e15021_d_n7;
            locals.var_t5__blk336_dn10 = assign12700_body11_e15021_d_n10;
            locals.var_t5__blk336_dn11 = assign12700_body11_e15021_d_n11;
            locals.var_t5__blk336_dn12 = assign12700_body11_e15021_d_n12;
            locals.var_t5__blk336_dn17 = assign12700_body11_e15021_d_n17;
            let (assign12700_body12_e15045, assign12700_body12_e15045_d_n0, assign12700_body12_e15045_d_n2, assign12700_body12_e15045_d_n6, assign12700_body12_e15045_d_n7, assign12700_body12_e15045_d_n10, assign12700_body12_e15045_d_n11, assign12700_body12_e15045_d_n12, assign12700_body12_e15045_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 != 0.0)) {
        let assign12700_body12_e15036: f64 = (locals.var_t4__blk335 * locals.var_t4__blk335);
        let assign12700_body12_e15039: f64 = (4.0 * locals.var_q_fd_dlt1);
        let assign12700_body12_e15041: f64 = (assign12700_body12_e15039 * locals.var_q_fd_dlt1);
        let assign12700_body12_e15042: f64 = (assign12700_body12_e15036 + assign12700_body12_e15041);
        let assign12700_body12_e15043: f64 = (assign12700_body12_e15042).sqrt();
        (assign12700_body12_e15043, ((((locals.var_t4__blk335_dn0 * locals.var_t4__blk335) + (locals.var_t4__blk335 * locals.var_t4__blk335_dn0)) + (((4.0 * locals.var_q_fd_dlt1_dn0) * locals.var_q_fd_dlt1) + (assign12700_body12_e15039 * locals.var_q_fd_dlt1_dn0))) / (2.0 * assign12700_body12_e15043)), ((((locals.var_t4__blk335_dn2 * locals.var_t4__blk335) + (locals.var_t4__blk335 * locals.var_t4__blk335_dn2)) + (((4.0 * locals.var_q_fd_dlt1_dn2) * locals.var_q_fd_dlt1) + (assign12700_body12_e15039 * locals.var_q_fd_dlt1_dn2))) / (2.0 * assign12700_body12_e15043)), ((((locals.var_t4__blk335_dn6 * locals.var_t4__blk335) + (locals.var_t4__blk335 * locals.var_t4__blk335_dn6)) + (((4.0 * locals.var_q_fd_dlt1_dn6) * locals.var_q_fd_dlt1) + (assign12700_body12_e15039 * locals.var_q_fd_dlt1_dn6))) / (2.0 * assign12700_body12_e15043)), ((((locals.var_t4__blk335_dn7 * locals.var_t4__blk335) + (locals.var_t4__blk335 * locals.var_t4__blk335_dn7)) + (((4.0 * locals.var_q_fd_dlt1_dn7) * locals.var_q_fd_dlt1) + (assign12700_body12_e15039 * locals.var_q_fd_dlt1_dn7))) / (2.0 * assign12700_body12_e15043)), ((((locals.var_t4__blk335_dn10 * locals.var_t4__blk335) + (locals.var_t4__blk335 * locals.var_t4__blk335_dn10)) + (((4.0 * locals.var_q_fd_dlt1_dn10) * locals.var_q_fd_dlt1) + (assign12700_body12_e15039 * locals.var_q_fd_dlt1_dn10))) / (2.0 * assign12700_body12_e15043)), ((((locals.var_t4__blk335_dn11 * locals.var_t4__blk335) + (locals.var_t4__blk335 * locals.var_t4__blk335_dn11)) + (((4.0 * locals.var_q_fd_dlt1_dn11) * locals.var_q_fd_dlt1) + (assign12700_body12_e15039 * locals.var_q_fd_dlt1_dn11))) / (2.0 * assign12700_body12_e15043)), ((((locals.var_t4__blk335_dn12 * locals.var_t4__blk335) + (locals.var_t4__blk335 * locals.var_t4__blk335_dn12)) + (((4.0 * locals.var_q_fd_dlt1_dn12) * locals.var_q_fd_dlt1) + (assign12700_body12_e15039 * locals.var_q_fd_dlt1_dn12))) / (2.0 * assign12700_body12_e15043)), ((((locals.var_t4__blk335_dn17 * locals.var_t4__blk335) + (locals.var_t4__blk335 * locals.var_t4__blk335_dn17)) + (((4.0 * locals.var_q_fd_dlt1_dn17) * locals.var_q_fd_dlt1) + (assign12700_body12_e15039 * locals.var_q_fd_dlt1_dn17))) / (2.0 * assign12700_body12_e15043)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign12700_body12_e15045;
            locals.var_tmf2_dn0 = assign12700_body12_e15045_d_n0;
            locals.var_tmf2_dn2 = assign12700_body12_e15045_d_n2;
            locals.var_tmf2_dn6 = assign12700_body12_e15045_d_n6;
            locals.var_tmf2_dn7 = assign12700_body12_e15045_d_n7;
            locals.var_tmf2_dn10 = assign12700_body12_e15045_d_n10;
            locals.var_tmf2_dn11 = assign12700_body12_e15045_d_n11;
            locals.var_tmf2_dn12 = assign12700_body12_e15045_d_n12;
            locals.var_tmf2_dn17 = assign12700_body12_e15045_d_n17;
            let (assign12700_body13_e15066, assign12700_body13_e15066_d_n0, assign12700_body13_e15066_d_n2, assign12700_body13_e15066_d_n6, assign12700_body13_e15066_d_n7, assign12700_body13_e15066_d_n10, assign12700_body13_e15066_d_n11, assign12700_body13_e15066_d_n12, assign12700_body13_e15066_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 != 0.0)) {
        let assign12700_body13_e15062: f64 = (locals.var_t4__blk335 / locals.var_tmf2);
        let assign12700_body13_e15063: f64 = (1.0 + assign12700_body13_e15062);
        let assign12700_body13_e15064: f64 = (0.5 * assign12700_body13_e15063);
        (assign12700_body13_e15064, (0.5 * (((locals.var_t4__blk335_dn0 * locals.var_tmf2) - (locals.var_t4__blk335 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk335_dn2 * locals.var_tmf2) - (locals.var_t4__blk335 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk335_dn6 * locals.var_tmf2) - (locals.var_t4__blk335 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk335_dn7 * locals.var_tmf2) - (locals.var_t4__blk335 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk335_dn10 * locals.var_tmf2) - (locals.var_t4__blk335 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk335_dn11 * locals.var_tmf2) - (locals.var_t4__blk335 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk335_dn12 * locals.var_tmf2) - (locals.var_t4__blk335 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk335_dn17 * locals.var_tmf2) - (locals.var_t4__blk335 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t7__blk338, locals.var_t7__blk338_dn0, locals.var_t7__blk338_dn2, locals.var_t7__blk338_dn6, locals.var_t7__blk338_dn7, locals.var_t7__blk338_dn10, locals.var_t7__blk338_dn11, locals.var_t7__blk338_dn12, locals.var_t7__blk338_dn17,)
    }
};
            locals.var_t7__blk338 = assign12700_body13_e15066;
            locals.var_t7__blk338_dn0 = assign12700_body13_e15066_d_n0;
            locals.var_t7__blk338_dn2 = assign12700_body13_e15066_d_n2;
            locals.var_t7__blk338_dn6 = assign12700_body13_e15066_d_n6;
            locals.var_t7__blk338_dn7 = assign12700_body13_e15066_d_n7;
            locals.var_t7__blk338_dn10 = assign12700_body13_e15066_d_n10;
            locals.var_t7__blk338_dn11 = assign12700_body13_e15066_d_n11;
            locals.var_t7__blk338_dn12 = assign12700_body13_e15066_d_n12;
            locals.var_t7__blk338_dn17 = assign12700_body13_e15066_d_n17;
            let (assign12700_body14_e15089, assign12700_body14_e15089_d_n0, assign12700_body14_e15089_d_n2, assign12700_body14_e15089_d_n6, assign12700_body14_e15089_d_n7, assign12700_body14_e15089_d_n10, assign12700_body14_e15089_d_n11, assign12700_body14_e15089_d_n12, assign12700_body14_e15089_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 != 0.0)) {
        let assign12700_body14_e15082: f64 = (locals.var_t4__blk335 + locals.var_tmf2);
        let assign12700_body14_e15083: f64 = (0.5 * assign12700_body14_e15082);
        let assign12700_body14_e15086: f64 = (1e-10 * locals.var_q_fd_dlt1);
        let assign12700_body14_e15087: f64 = (assign12700_body14_e15083 + assign12700_body14_e15086);
        (assign12700_body14_e15087, ((0.5 * (locals.var_t4__blk335_dn0 + locals.var_tmf2_dn0)) + (1e-10 * locals.var_q_fd_dlt1_dn0)), ((0.5 * (locals.var_t4__blk335_dn2 + locals.var_tmf2_dn2)) + (1e-10 * locals.var_q_fd_dlt1_dn2)), ((0.5 * (locals.var_t4__blk335_dn6 + locals.var_tmf2_dn6)) + (1e-10 * locals.var_q_fd_dlt1_dn6)), ((0.5 * (locals.var_t4__blk335_dn7 + locals.var_tmf2_dn7)) + (1e-10 * locals.var_q_fd_dlt1_dn7)), ((0.5 * (locals.var_t4__blk335_dn10 + locals.var_tmf2_dn10)) + (1e-10 * locals.var_q_fd_dlt1_dn10)), ((0.5 * (locals.var_t4__blk335_dn11 + locals.var_tmf2_dn11)) + (1e-10 * locals.var_q_fd_dlt1_dn11)), ((0.5 * (locals.var_t4__blk335_dn12 + locals.var_tmf2_dn12)) + (1e-10 * locals.var_q_fd_dlt1_dn12)), ((0.5 * (locals.var_t4__blk335_dn17 + locals.var_tmf2_dn17)) + (1e-10 * locals.var_q_fd_dlt1_dn17)),)
    } else {
        (locals.var_t6__blk337, locals.var_t6__blk337_dn0, locals.var_t6__blk337_dn2, locals.var_t6__blk337_dn6, locals.var_t6__blk337_dn7, locals.var_t6__blk337_dn10, locals.var_t6__blk337_dn11, locals.var_t6__blk337_dn12, locals.var_t6__blk337_dn17,)
    }
};
            locals.var_t6__blk337 = assign12700_body14_e15089;
            locals.var_t6__blk337_dn0 = assign12700_body14_e15089_d_n0;
            locals.var_t6__blk337_dn2 = assign12700_body14_e15089_d_n2;
            locals.var_t6__blk337_dn6 = assign12700_body14_e15089_d_n6;
            locals.var_t6__blk337_dn7 = assign12700_body14_e15089_d_n7;
            locals.var_t6__blk337_dn10 = assign12700_body14_e15089_d_n10;
            locals.var_t6__blk337_dn11 = assign12700_body14_e15089_d_n11;
            locals.var_t6__blk337_dn12 = assign12700_body14_e15089_d_n12;
            locals.var_t6__blk337_dn17 = assign12700_body14_e15089_d_n17;
            let assign12700_body15_e15092: f64 = if locals.var_t6__blk337 < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard343 = assign12700_body15_e15092;
            let (assign12700_body16_e15109, assign12700_body16_e15109_d_n0, assign12700_body16_e15109_d_n2, assign12700_body16_e15109_d_n6, assign12700_body16_e15109_d_n7, assign12700_body16_e15109_d_n10, assign12700_body16_e15109_d_n11, assign12700_body16_e15109_d_n12, assign12700_body16_e15109_d_n17,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 != 0.0)) && (locals.var_guard343 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk337, locals.var_t6__blk337_dn0, locals.var_t6__blk337_dn2, locals.var_t6__blk337_dn6, locals.var_t6__blk337_dn7, locals.var_t6__blk337_dn10, locals.var_t6__blk337_dn11, locals.var_t6__blk337_dn12, locals.var_t6__blk337_dn17,)
    }
};
            locals.var_t6__blk337 = assign12700_body16_e15109;
            locals.var_t6__blk337_dn0 = assign12700_body16_e15109_d_n0;
            locals.var_t6__blk337_dn2 = assign12700_body16_e15109_d_n2;
            locals.var_t6__blk337_dn6 = assign12700_body16_e15109_d_n6;
            locals.var_t6__blk337_dn7 = assign12700_body16_e15109_d_n7;
            locals.var_t6__blk337_dn10 = assign12700_body16_e15109_d_n10;
            locals.var_t6__blk337_dn11 = assign12700_body16_e15109_d_n11;
            locals.var_t6__blk337_dn12 = assign12700_body16_e15109_d_n12;
            locals.var_t6__blk337_dn17 = assign12700_body16_e15109_d_n17;
            let (assign12700_body17_e15126, assign12700_body17_e15126_d_n0, assign12700_body17_e15126_d_n2, assign12700_body17_e15126_d_n6, assign12700_body17_e15126_d_n7, assign12700_body17_e15126_d_n10, assign12700_body17_e15126_d_n11, assign12700_body17_e15126_d_n12, assign12700_body17_e15126_d_n17,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 != 0.0)) && (locals.var_guard343 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7__blk338, locals.var_t7__blk338_dn0, locals.var_t7__blk338_dn2, locals.var_t7__blk338_dn6, locals.var_t7__blk338_dn7, locals.var_t7__blk338_dn10, locals.var_t7__blk338_dn11, locals.var_t7__blk338_dn12, locals.var_t7__blk338_dn17,)
    }
};
            locals.var_t7__blk338 = assign12700_body17_e15126;
            locals.var_t7__blk338_dn0 = assign12700_body17_e15126_d_n0;
            locals.var_t7__blk338_dn2 = assign12700_body17_e15126_d_n2;
            locals.var_t7__blk338_dn6 = assign12700_body17_e15126_d_n6;
            locals.var_t7__blk338_dn7 = assign12700_body17_e15126_d_n7;
            locals.var_t7__blk338_dn10 = assign12700_body17_e15126_d_n10;
            locals.var_t7__blk338_dn11 = assign12700_body17_e15126_d_n11;
            locals.var_t7__blk338_dn12 = assign12700_body17_e15126_d_n12;
            locals.var_t7__blk338_dn17 = assign12700_body17_e15126_d_n17;
            let (assign12700_body18_e15146, assign12700_body18_e15146_d_n0, assign12700_body18_e15146_d_n2, assign12700_body18_e15146_d_n6, assign12700_body18_e15146_d_n7, assign12700_body18_e15146_d_n10, assign12700_body18_e15146_d_n11, assign12700_body18_e15146_d_n12, assign12700_body18_e15146_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 != 0.0)) {
        let assign12700_body18_e15140: f64 = (-locals.var_q_fd_soi);
        let assign12700_body18_e15142: f64 = (assign12700_body18_e15140 - locals.var_t6__blk337);
        let assign12700_body18_e15144: f64 = (assign12700_body18_e15142 - locals.var_q_fd_dlt2);
        (assign12700_body18_e15144, (((-locals.var_q_fd_soi_dn0) - locals.var_t6__blk337_dn0) - locals.var_q_fd_dlt2_dn0), (((-locals.var_q_fd_soi_dn2) - locals.var_t6__blk337_dn2) - locals.var_q_fd_dlt2_dn2), (((-locals.var_q_fd_soi_dn6) - locals.var_t6__blk337_dn6) - locals.var_q_fd_dlt2_dn6), (((-locals.var_q_fd_soi_dn7) - locals.var_t6__blk337_dn7) - locals.var_q_fd_dlt2_dn7), (((-locals.var_q_fd_soi_dn10) - locals.var_t6__blk337_dn10) - locals.var_q_fd_dlt2_dn10), (((-locals.var_q_fd_soi_dn11) - locals.var_t6__blk337_dn11) - locals.var_q_fd_dlt2_dn11), (((-locals.var_q_fd_soi_dn12) - locals.var_t6__blk337_dn12) - locals.var_q_fd_dlt2_dn12), (((-locals.var_q_fd_soi_dn17) - locals.var_t6__blk337_dn17) - locals.var_q_fd_dlt2_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
            locals.var_tmf1 = assign12700_body18_e15146;
            locals.var_tmf1_dn0 = assign12700_body18_e15146_d_n0;
            locals.var_tmf1_dn2 = assign12700_body18_e15146_d_n2;
            locals.var_tmf1_dn6 = assign12700_body18_e15146_d_n6;
            locals.var_tmf1_dn7 = assign12700_body18_e15146_d_n7;
            locals.var_tmf1_dn10 = assign12700_body18_e15146_d_n10;
            locals.var_tmf1_dn11 = assign12700_body18_e15146_d_n11;
            locals.var_tmf1_dn12 = assign12700_body18_e15146_d_n12;
            locals.var_tmf1_dn17 = assign12700_body18_e15146_d_n17;
            let (assign12700_body19_e15166, assign12700_body19_e15166_d_n0, assign12700_body19_e15166_d_n2, assign12700_body19_e15166_d_n6, assign12700_body19_e15166_d_n7, assign12700_body19_e15166_d_n10, assign12700_body19_e15166_d_n11, assign12700_body19_e15166_d_n12, assign12700_body19_e15166_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 != 0.0)) {
        let assign12700_body19_e15161: f64 = (-locals.var_q_fd_soi);
        let assign12700_body19_e15162: f64 = (4.0 * assign12700_body19_e15161);
        let assign12700_body19_e15164: f64 = (assign12700_body19_e15162 * locals.var_q_fd_dlt2);
        (assign12700_body19_e15164, (((4.0 * (-locals.var_q_fd_soi_dn0)) * locals.var_q_fd_dlt2) + (assign12700_body19_e15162 * locals.var_q_fd_dlt2_dn0)), (((4.0 * (-locals.var_q_fd_soi_dn2)) * locals.var_q_fd_dlt2) + (assign12700_body19_e15162 * locals.var_q_fd_dlt2_dn2)), (((4.0 * (-locals.var_q_fd_soi_dn6)) * locals.var_q_fd_dlt2) + (assign12700_body19_e15162 * locals.var_q_fd_dlt2_dn6)), (((4.0 * (-locals.var_q_fd_soi_dn7)) * locals.var_q_fd_dlt2) + (assign12700_body19_e15162 * locals.var_q_fd_dlt2_dn7)), (((4.0 * (-locals.var_q_fd_soi_dn10)) * locals.var_q_fd_dlt2) + (assign12700_body19_e15162 * locals.var_q_fd_dlt2_dn10)), (((4.0 * (-locals.var_q_fd_soi_dn11)) * locals.var_q_fd_dlt2) + (assign12700_body19_e15162 * locals.var_q_fd_dlt2_dn11)), (((4.0 * (-locals.var_q_fd_soi_dn12)) * locals.var_q_fd_dlt2) + (assign12700_body19_e15162 * locals.var_q_fd_dlt2_dn12)), (((4.0 * (-locals.var_q_fd_soi_dn17)) * locals.var_q_fd_dlt2) + (assign12700_body19_e15162 * locals.var_q_fd_dlt2_dn17)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign12700_body19_e15166;
            locals.var_tmf2_dn0 = assign12700_body19_e15166_d_n0;
            locals.var_tmf2_dn2 = assign12700_body19_e15166_d_n2;
            locals.var_tmf2_dn6 = assign12700_body19_e15166_d_n6;
            locals.var_tmf2_dn7 = assign12700_body19_e15166_d_n7;
            locals.var_tmf2_dn10 = assign12700_body19_e15166_d_n10;
            locals.var_tmf2_dn11 = assign12700_body19_e15166_d_n11;
            locals.var_tmf2_dn12 = assign12700_body19_e15166_d_n12;
            locals.var_tmf2_dn17 = assign12700_body19_e15166_d_n17;
            let (assign12700_body20_e15187, assign12700_body20_e15187_d_n0, assign12700_body20_e15187_d_n2, assign12700_body20_e15187_d_n6, assign12700_body20_e15187_d_n7, assign12700_body20_e15187_d_n10, assign12700_body20_e15187_d_n11, assign12700_body20_e15187_d_n12, assign12700_body20_e15187_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 != 0.0)) {
        let (assign12700_body20_e15185, assign12700_body20_e15185_d_n0, assign12700_body20_e15185_d_n2, assign12700_body20_e15185_d_n6, assign12700_body20_e15185_d_n7, assign12700_body20_e15185_d_n10, assign12700_body20_e15185_d_n11, assign12700_body20_e15185_d_n12, assign12700_body20_e15185_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign12700_body20_e15184: f64 = (-locals.var_tmf2);
                (assign12700_body20_e15184, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign12700_body20_e15185, assign12700_body20_e15185_d_n0, assign12700_body20_e15185_d_n2, assign12700_body20_e15185_d_n6, assign12700_body20_e15185_d_n7, assign12700_body20_e15185_d_n10, assign12700_body20_e15185_d_n11, assign12700_body20_e15185_d_n12, assign12700_body20_e15185_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign12700_body20_e15187;
            locals.var_tmf2_dn0 = assign12700_body20_e15187_d_n0;
            locals.var_tmf2_dn2 = assign12700_body20_e15187_d_n2;
            locals.var_tmf2_dn6 = assign12700_body20_e15187_d_n6;
            locals.var_tmf2_dn7 = assign12700_body20_e15187_d_n7;
            locals.var_tmf2_dn10 = assign12700_body20_e15187_d_n10;
            locals.var_tmf2_dn11 = assign12700_body20_e15187_d_n11;
            locals.var_tmf2_dn12 = assign12700_body20_e15187_d_n12;
            locals.var_tmf2_dn17 = assign12700_body20_e15187_d_n17;
            let (assign12700_body21_e15207, assign12700_body21_e15207_d_n0, assign12700_body21_e15207_d_n2, assign12700_body21_e15207_d_n6, assign12700_body21_e15207_d_n7, assign12700_body21_e15207_d_n10, assign12700_body21_e15207_d_n11, assign12700_body21_e15207_d_n12, assign12700_body21_e15207_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 != 0.0)) {
        let assign12700_body21_e15202: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign12700_body21_e15204: f64 = (assign12700_body21_e15202 + locals.var_tmf2);
        let assign12700_body21_e15205: f64 = (assign12700_body21_e15204).sqrt();
        (assign12700_body21_e15205, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign12700_body21_e15205)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign12700_body21_e15205)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign12700_body21_e15205)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign12700_body21_e15205)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign12700_body21_e15205)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign12700_body21_e15205)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign12700_body21_e15205)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign12700_body21_e15205)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign12700_body21_e15207;
            locals.var_tmf2_dn0 = assign12700_body21_e15207_d_n0;
            locals.var_tmf2_dn2 = assign12700_body21_e15207_d_n2;
            locals.var_tmf2_dn6 = assign12700_body21_e15207_d_n6;
            locals.var_tmf2_dn7 = assign12700_body21_e15207_d_n7;
            locals.var_tmf2_dn10 = assign12700_body21_e15207_d_n10;
            locals.var_tmf2_dn11 = assign12700_body21_e15207_d_n11;
            locals.var_tmf2_dn12 = assign12700_body21_e15207_d_n12;
            locals.var_tmf2_dn17 = assign12700_body21_e15207_d_n17;
            let (assign12700_body22_e15228, assign12700_body22_e15228_d_n0, assign12700_body22_e15228_d_n2, assign12700_body22_e15228_d_n6, assign12700_body22_e15228_d_n7, assign12700_body22_e15228_d_n10, assign12700_body22_e15228_d_n11, assign12700_body22_e15228_d_n12, assign12700_body22_e15228_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 != 0.0)) {
        let assign12700_body22_e15224: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign12700_body22_e15225: f64 = (1.0 + assign12700_body22_e15224);
        let assign12700_body22_e15226: f64 = (0.5 * assign12700_body22_e15225);
        (assign12700_body22_e15226, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn17,)
    }
};
            locals.var_t8 = assign12700_body22_e15228;
            locals.var_t8_dn0 = assign12700_body22_e15228_d_n0;
            locals.var_t8_dn2 = assign12700_body22_e15228_d_n2;
            locals.var_t8_dn6 = assign12700_body22_e15228_d_n6;
            locals.var_t8_dn7 = assign12700_body22_e15228_d_n7;
            locals.var_t8_dn10 = assign12700_body22_e15228_d_n10;
            locals.var_t8_dn11 = assign12700_body22_e15228_d_n11;
            locals.var_t8_dn12 = assign12700_body22_e15228_d_n12;
            locals.var_t8_dn17 = assign12700_body22_e15228_d_n17;
            let (assign12700_body23_e15250, assign12700_body23_e15250_d_n0, assign12700_body23_e15250_d_n2, assign12700_body23_e15250_d_n6, assign12700_body23_e15250_d_n7, assign12700_body23_e15250_d_n10, assign12700_body23_e15250_d_n11, assign12700_body23_e15250_d_n12, assign12700_body23_e15250_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 != 0.0)) {
        let assign12700_body23_e15242: f64 = (-locals.var_q_fd_soi);
        let assign12700_body23_e15246: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign12700_body23_e15247: f64 = (0.5 * assign12700_body23_e15246);
        let assign12700_body23_e15248: f64 = (assign12700_body23_e15242 - assign12700_body23_e15247);
        (assign12700_body23_e15248, ((-locals.var_q_fd_soi_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((-locals.var_q_fd_soi_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((-locals.var_q_fd_soi_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((-locals.var_q_fd_soi_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((-locals.var_q_fd_soi_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((-locals.var_q_fd_soi_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((-locals.var_q_fd_soi_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((-locals.var_q_fd_soi_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_t6__blk337, locals.var_t6__blk337_dn0, locals.var_t6__blk337_dn2, locals.var_t6__blk337_dn6, locals.var_t6__blk337_dn7, locals.var_t6__blk337_dn10, locals.var_t6__blk337_dn11, locals.var_t6__blk337_dn12, locals.var_t6__blk337_dn17,)
    }
};
            locals.var_t6__blk337 = assign12700_body23_e15250;
            locals.var_t6__blk337_dn0 = assign12700_body23_e15250_d_n0;
            locals.var_t6__blk337_dn2 = assign12700_body23_e15250_d_n2;
            locals.var_t6__blk337_dn6 = assign12700_body23_e15250_d_n6;
            locals.var_t6__blk337_dn7 = assign12700_body23_e15250_d_n7;
            locals.var_t6__blk337_dn10 = assign12700_body23_e15250_d_n10;
            locals.var_t6__blk337_dn11 = assign12700_body23_e15250_d_n11;
            locals.var_t6__blk337_dn12 = assign12700_body23_e15250_d_n12;
            locals.var_t6__blk337_dn17 = assign12700_body23_e15250_d_n17;
            let (assign12700_body24_e15269, assign12700_body24_e15269_d_n0, assign12700_body24_e15269_d_n2, assign12700_body24_e15269_d_n6, assign12700_body24_e15269_d_n7, assign12700_body24_e15269_d_n10, assign12700_body24_e15269_d_n11, assign12700_body24_e15269_d_n12, assign12700_body24_e15269_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 != 0.0)) {
        let assign12700_body24_e15266: f64 = (locals.var_t5__blk336 * locals.var_t8);
        let assign12700_body24_e15267: f64 = (locals.var_t7__blk338 * assign12700_body24_e15266);
        (assign12700_body24_e15267, ((locals.var_t7__blk338_dn0 * assign12700_body24_e15266) + (locals.var_t7__blk338 * ((locals.var_t5__blk336_dn0 * locals.var_t8) + (locals.var_t5__blk336 * locals.var_t8_dn0)))), ((locals.var_t7__blk338_dn2 * assign12700_body24_e15266) + (locals.var_t7__blk338 * ((locals.var_t5__blk336_dn2 * locals.var_t8) + (locals.var_t5__blk336 * locals.var_t8_dn2)))), ((locals.var_t7__blk338_dn6 * assign12700_body24_e15266) + (locals.var_t7__blk338 * ((locals.var_t5__blk336_dn6 * locals.var_t8) + (locals.var_t5__blk336 * locals.var_t8_dn6)))), ((locals.var_t7__blk338_dn7 * assign12700_body24_e15266) + (locals.var_t7__blk338 * ((locals.var_t5__blk336_dn7 * locals.var_t8) + (locals.var_t5__blk336 * locals.var_t8_dn7)))), ((locals.var_t7__blk338_dn10 * assign12700_body24_e15266) + (locals.var_t7__blk338 * ((locals.var_t5__blk336_dn10 * locals.var_t8) + (locals.var_t5__blk336 * locals.var_t8_dn10)))), ((locals.var_t7__blk338_dn11 * assign12700_body24_e15266) + (locals.var_t7__blk338 * ((locals.var_t5__blk336_dn11 * locals.var_t8) + (locals.var_t5__blk336 * locals.var_t8_dn11)))), ((locals.var_t7__blk338_dn12 * assign12700_body24_e15266) + (locals.var_t7__blk338 * ((locals.var_t5__blk336_dn12 * locals.var_t8) + (locals.var_t5__blk336 * locals.var_t8_dn12)))), ((locals.var_t7__blk338_dn17 * assign12700_body24_e15266) + (locals.var_t7__blk338 * ((locals.var_t5__blk336_dn17 * locals.var_t8) + (locals.var_t5__blk336 * locals.var_t8_dn17)))),)
    } else {
        (locals.var_t7__blk338, locals.var_t7__blk338_dn0, locals.var_t7__blk338_dn2, locals.var_t7__blk338_dn6, locals.var_t7__blk338_dn7, locals.var_t7__blk338_dn10, locals.var_t7__blk338_dn11, locals.var_t7__blk338_dn12, locals.var_t7__blk338_dn17,)
    }
};
            locals.var_t7__blk338 = assign12700_body24_e15269;
            locals.var_t7__blk338_dn0 = assign12700_body24_e15269_d_n0;
            locals.var_t7__blk338_dn2 = assign12700_body24_e15269_d_n2;
            locals.var_t7__blk338_dn6 = assign12700_body24_e15269_d_n6;
            locals.var_t7__blk338_dn7 = assign12700_body24_e15269_d_n7;
            locals.var_t7__blk338_dn10 = assign12700_body24_e15269_d_n10;
            locals.var_t7__blk338_dn11 = assign12700_body24_e15269_d_n11;
            locals.var_t7__blk338_dn12 = assign12700_body24_e15269_d_n12;
            locals.var_t7__blk338_dn17 = assign12700_body24_e15269_d_n17;
            let (assign12700_body25_e15294, assign12700_body25_e15294_d_n0, assign12700_body25_e15294_d_n2, assign12700_body25_e15294_d_n6, assign12700_body25_e15294_d_n7, assign12700_body25_e15294_d_n10, assign12700_body25_e15294_d_n11, assign12700_body25_e15294_d_n12, assign12700_body25_e15294_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 != 0.0)) {
        let assign12700_body25_e15284: f64 = (locals.var_t6__blk337 * locals.var_t6__blk337);
        let assign12700_body25_e15286: f64 = (assign12700_body25_e15284 / 2.0);
        let assign12700_body25_e15288: f64 = (assign12700_body25_e15286 / 1.034943e-10);
        let assign12700_body25_e15290: f64 = (assign12700_body25_e15288 / 1.6021918e-19);
        let assign12700_body25_e15292: f64 = (assign12700_body25_e15290 / locals.var_uc_nsubs);
        (assign12700_body25_e15292, ((((((((locals.var_t6__blk337_dn0 * locals.var_t6__blk337) + (locals.var_t6__blk337 * locals.var_t6__blk337_dn0)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12700_body25_e15290 * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk337_dn2 * locals.var_t6__blk337) + (locals.var_t6__blk337 * locals.var_t6__blk337_dn2)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12700_body25_e15290 * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk337_dn6 * locals.var_t6__blk337) + (locals.var_t6__blk337 * locals.var_t6__blk337_dn6)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12700_body25_e15290 * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk337_dn7 * locals.var_t6__blk337) + (locals.var_t6__blk337 * locals.var_t6__blk337_dn7)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12700_body25_e15290 * locals.var_uc_nsubs_dn7)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk337_dn10 * locals.var_t6__blk337) + (locals.var_t6__blk337 * locals.var_t6__blk337_dn10)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12700_body25_e15290 * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk337_dn11 * locals.var_t6__blk337) + (locals.var_t6__blk337 * locals.var_t6__blk337_dn11)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12700_body25_e15290 * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk337_dn12 * locals.var_t6__blk337) + (locals.var_t6__blk337 * locals.var_t6__blk337_dn12)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12700_body25_e15290 * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk337_dn17 * locals.var_t6__blk337) + (locals.var_t6__blk337 * locals.var_t6__blk337_dn17)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12700_body25_e15290 * locals.var_uc_nsubs_dn17)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)),)
    } else {
        (locals.var_phi_b_dep, locals.var_phi_b_dep_dn0, locals.var_phi_b_dep_dn2, locals.var_phi_b_dep_dn6, locals.var_phi_b_dep_dn7, locals.var_phi_b_dep_dn10, locals.var_phi_b_dep_dn11, locals.var_phi_b_dep_dn12, locals.var_phi_b_dep_dn17,)
    }
};
            locals.var_phi_b_dep = assign12700_body25_e15294;
            locals.var_phi_b_dep_dn0 = assign12700_body25_e15294_d_n0;
            locals.var_phi_b_dep_dn2 = assign12700_body25_e15294_d_n2;
            locals.var_phi_b_dep_dn6 = assign12700_body25_e15294_d_n6;
            locals.var_phi_b_dep_dn7 = assign12700_body25_e15294_d_n7;
            locals.var_phi_b_dep_dn10 = assign12700_body25_e15294_d_n10;
            locals.var_phi_b_dep_dn11 = assign12700_body25_e15294_d_n11;
            locals.var_phi_b_dep_dn12 = assign12700_body25_e15294_d_n12;
            locals.var_phi_b_dep_dn17 = assign12700_body25_e15294_d_n17;
            let (assign12700_body26_e15315, assign12700_body26_e15315_d_n0, assign12700_body26_e15315_d_n2, assign12700_body26_e15315_d_n6, assign12700_body26_e15315_d_n7, assign12700_body26_e15315_d_n10, assign12700_body26_e15315_d_n11, assign12700_body26_e15315_d_n12, assign12700_body26_e15315_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 != 0.0)) {
        let assign12700_body26_e15309: f64 = (2.0 * locals.var_phi_b_dep);
        let assign12700_body26_e15311: f64 = (assign12700_body26_e15309 * locals.var_t7__blk338);
        let assign12700_body26_e15313: f64 = (assign12700_body26_e15311 / locals.var_t6__blk337);
        (assign12700_body26_e15313, ((((((2.0 * locals.var_phi_b_dep_dn0) * locals.var_t7__blk338) + (assign12700_body26_e15309 * locals.var_t7__blk338_dn0)) * locals.var_t6__blk337) - (assign12700_body26_e15311 * locals.var_t6__blk337_dn0)) / (locals.var_t6__blk337 * locals.var_t6__blk337)), ((((((2.0 * locals.var_phi_b_dep_dn2) * locals.var_t7__blk338) + (assign12700_body26_e15309 * locals.var_t7__blk338_dn2)) * locals.var_t6__blk337) - (assign12700_body26_e15311 * locals.var_t6__blk337_dn2)) / (locals.var_t6__blk337 * locals.var_t6__blk337)), ((((((2.0 * locals.var_phi_b_dep_dn6) * locals.var_t7__blk338) + (assign12700_body26_e15309 * locals.var_t7__blk338_dn6)) * locals.var_t6__blk337) - (assign12700_body26_e15311 * locals.var_t6__blk337_dn6)) / (locals.var_t6__blk337 * locals.var_t6__blk337)), ((((((2.0 * locals.var_phi_b_dep_dn7) * locals.var_t7__blk338) + (assign12700_body26_e15309 * locals.var_t7__blk338_dn7)) * locals.var_t6__blk337) - (assign12700_body26_e15311 * locals.var_t6__blk337_dn7)) / (locals.var_t6__blk337 * locals.var_t6__blk337)), ((((((2.0 * locals.var_phi_b_dep_dn10) * locals.var_t7__blk338) + (assign12700_body26_e15309 * locals.var_t7__blk338_dn10)) * locals.var_t6__blk337) - (assign12700_body26_e15311 * locals.var_t6__blk337_dn10)) / (locals.var_t6__blk337 * locals.var_t6__blk337)), ((((((2.0 * locals.var_phi_b_dep_dn11) * locals.var_t7__blk338) + (assign12700_body26_e15309 * locals.var_t7__blk338_dn11)) * locals.var_t6__blk337) - (assign12700_body26_e15311 * locals.var_t6__blk337_dn11)) / (locals.var_t6__blk337 * locals.var_t6__blk337)), ((((((2.0 * locals.var_phi_b_dep_dn12) * locals.var_t7__blk338) + (assign12700_body26_e15309 * locals.var_t7__blk338_dn12)) * locals.var_t6__blk337) - (assign12700_body26_e15311 * locals.var_t6__blk337_dn12)) / (locals.var_t6__blk337 * locals.var_t6__blk337)), ((((((2.0 * locals.var_phi_b_dep_dn17) * locals.var_t7__blk338) + (assign12700_body26_e15309 * locals.var_t7__blk338_dn17)) * locals.var_t6__blk337) - (assign12700_body26_e15311 * locals.var_t6__blk337_dn17)) / (locals.var_t6__blk337 * locals.var_t6__blk337)),)
    } else {
        (locals.var_phi_b_dep_dpsb, locals.var_phi_b_dep_dpsb_dn0, locals.var_phi_b_dep_dpsb_dn2, locals.var_phi_b_dep_dpsb_dn6, locals.var_phi_b_dep_dpsb_dn7, locals.var_phi_b_dep_dpsb_dn10, locals.var_phi_b_dep_dpsb_dn11, locals.var_phi_b_dep_dpsb_dn12, locals.var_phi_b_dep_dpsb_dn17,)
    }
};
            locals.var_phi_b_dep_dpsb = assign12700_body26_e15315;
            locals.var_phi_b_dep_dpsb_dn0 = assign12700_body26_e15315_d_n0;
            locals.var_phi_b_dep_dpsb_dn2 = assign12700_body26_e15315_d_n2;
            locals.var_phi_b_dep_dpsb_dn6 = assign12700_body26_e15315_d_n6;
            locals.var_phi_b_dep_dpsb_dn7 = assign12700_body26_e15315_d_n7;
            locals.var_phi_b_dep_dpsb_dn10 = assign12700_body26_e15315_d_n10;
            locals.var_phi_b_dep_dpsb_dn11 = assign12700_body26_e15315_d_n11;
            locals.var_phi_b_dep_dpsb_dn12 = assign12700_body26_e15315_d_n12;
            locals.var_phi_b_dep_dpsb_dn17 = assign12700_body26_e15315_d_n17;
            let (assign12700_body27_e15350, assign12700_body27_e15350_d_n0, assign12700_body27_e15350_d_n2, assign12700_body27_e15350_d_n6, assign12700_body27_e15350_d_n7, assign12700_body27_e15350_d_n10, assign12700_body27_e15350_d_n11, assign12700_body27_e15350_d_n12, assign12700_body27_e15350_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 != 0.0)) {
        let assign12700_body27_e15330: f64 = (-locals.var_phi_sl_bulk);
        let assign12700_body27_e15333: f64 = (locals.var_t4__blk335 / locals.var_c_box);
        let assign12700_body27_e15334: f64 = (assign12700_body27_e15330 + assign12700_body27_e15333);
        let assign12700_body27_e15336: f64 = (assign12700_body27_e15334 - locals.var_vbsbiz);
        let assign12700_body27_e15338: f64 = (assign12700_body27_e15336 + locals.var_phi_b_dep);
        let assign12700_body27_e15340: f64 = (-1.0);
        let assign12700_body27_e15343: f64 = (locals.var_t5__blk336 / locals.var_c_box);
        let assign12700_body27_e15344: f64 = (assign12700_body27_e15340 + assign12700_body27_e15343);
        let assign12700_body27_e15346: f64 = (assign12700_body27_e15344 + locals.var_phi_b_dep_dpsb);
        let assign12700_body27_e15347: f64 = (assign12700_body27_e15338 / assign12700_body27_e15346);
        let assign12700_body27_e15348: f64 = (locals.var_phi_sl_bulk - assign12700_body27_e15347);
        (assign12700_body27_e15348, (locals.var_phi_sl_bulk_dn0 - (((((((-locals.var_phi_sl_bulk_dn0) + (locals.var_t4__blk335_dn0 / locals.var_c_box)) - locals.var_vbsbiz_dn0) + locals.var_phi_b_dep_dn0) * assign12700_body27_e15346) - (assign12700_body27_e15338 * ((locals.var_t5__blk336_dn0 / locals.var_c_box) + locals.var_phi_b_dep_dpsb_dn0))) / (assign12700_body27_e15346 * assign12700_body27_e15346))), (locals.var_phi_sl_bulk_dn2 - (((((((-locals.var_phi_sl_bulk_dn2) + (locals.var_t4__blk335_dn2 / locals.var_c_box)) - locals.var_vbsbiz_dn2) + locals.var_phi_b_dep_dn2) * assign12700_body27_e15346) - (assign12700_body27_e15338 * ((locals.var_t5__blk336_dn2 / locals.var_c_box) + locals.var_phi_b_dep_dpsb_dn2))) / (assign12700_body27_e15346 * assign12700_body27_e15346))), (locals.var_phi_sl_bulk_dn6 - (((((((-locals.var_phi_sl_bulk_dn6) + (locals.var_t4__blk335_dn6 / locals.var_c_box)) - locals.var_vbsbiz_dn6) + locals.var_phi_b_dep_dn6) * assign12700_body27_e15346) - (assign12700_body27_e15338 * ((locals.var_t5__blk336_dn6 / locals.var_c_box) + locals.var_phi_b_dep_dpsb_dn6))) / (assign12700_body27_e15346 * assign12700_body27_e15346))), (locals.var_phi_sl_bulk_dn7 - (((((((-locals.var_phi_sl_bulk_dn7) + (locals.var_t4__blk335_dn7 / locals.var_c_box)) - locals.var_vbsbiz_dn7) + locals.var_phi_b_dep_dn7) * assign12700_body27_e15346) - (assign12700_body27_e15338 * ((locals.var_t5__blk336_dn7 / locals.var_c_box) + locals.var_phi_b_dep_dpsb_dn7))) / (assign12700_body27_e15346 * assign12700_body27_e15346))), (locals.var_phi_sl_bulk_dn10 - (((((((-locals.var_phi_sl_bulk_dn10) + (locals.var_t4__blk335_dn10 / locals.var_c_box)) - locals.var_vbsbiz_dn10) + locals.var_phi_b_dep_dn10) * assign12700_body27_e15346) - (assign12700_body27_e15338 * ((locals.var_t5__blk336_dn10 / locals.var_c_box) + locals.var_phi_b_dep_dpsb_dn10))) / (assign12700_body27_e15346 * assign12700_body27_e15346))), (locals.var_phi_sl_bulk_dn11 - (((((((-locals.var_phi_sl_bulk_dn11) + (locals.var_t4__blk335_dn11 / locals.var_c_box)) - locals.var_vbsbiz_dn11) + locals.var_phi_b_dep_dn11) * assign12700_body27_e15346) - (assign12700_body27_e15338 * ((locals.var_t5__blk336_dn11 / locals.var_c_box) + locals.var_phi_b_dep_dpsb_dn11))) / (assign12700_body27_e15346 * assign12700_body27_e15346))), (locals.var_phi_sl_bulk_dn12 - (((((((-locals.var_phi_sl_bulk_dn12) + (locals.var_t4__blk335_dn12 / locals.var_c_box)) - locals.var_vbsbiz_dn12) + locals.var_phi_b_dep_dn12) * assign12700_body27_e15346) - (assign12700_body27_e15338 * ((locals.var_t5__blk336_dn12 / locals.var_c_box) + locals.var_phi_b_dep_dpsb_dn12))) / (assign12700_body27_e15346 * assign12700_body27_e15346))), (locals.var_phi_sl_bulk_dn17 - (((((((-locals.var_phi_sl_bulk_dn17) + (locals.var_t4__blk335_dn17 / locals.var_c_box)) - locals.var_vbsbiz_dn17) + locals.var_phi_b_dep_dn17) * assign12700_body27_e15346) - (assign12700_body27_e15338 * ((locals.var_t5__blk336_dn17 / locals.var_c_box) + locals.var_phi_b_dep_dpsb_dn17))) / (assign12700_body27_e15346 * assign12700_body27_e15346))),)
    } else {
        (locals.var_t6__blk337, locals.var_t6__blk337_dn0, locals.var_t6__blk337_dn2, locals.var_t6__blk337_dn6, locals.var_t6__blk337_dn7, locals.var_t6__blk337_dn10, locals.var_t6__blk337_dn11, locals.var_t6__blk337_dn12, locals.var_t6__blk337_dn17,)
    }
};
            locals.var_t6__blk337 = assign12700_body27_e15350;
            locals.var_t6__blk337_dn0 = assign12700_body27_e15350_d_n0;
            locals.var_t6__blk337_dn2 = assign12700_body27_e15350_d_n2;
            locals.var_t6__blk337_dn6 = assign12700_body27_e15350_d_n6;
            locals.var_t6__blk337_dn7 = assign12700_body27_e15350_d_n7;
            locals.var_t6__blk337_dn10 = assign12700_body27_e15350_d_n10;
            locals.var_t6__blk337_dn11 = assign12700_body27_e15350_d_n11;
            locals.var_t6__blk337_dn12 = assign12700_body27_e15350_d_n12;
            locals.var_t6__blk337_dn17 = assign12700_body27_e15350_d_n17;
            let assign12700_body28_e15353: f64 = (locals.var_t6__blk337 - locals.var_phi_sl_bulk);
            let assign12700_body28_e15354: f64 = (assign12700_body28_e15353).abs();
            let assign12700_body28_e15356: f64 = if assign12700_body28_e15354 < 5e-12 { 1.0 } else { 0.0 };
            locals.var_guard344 = assign12700_body28_e15356;
            let (assign12700_body29_e15373,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 != 0.0)) && (locals.var_guard344 != 0.0)) {
        (locals.var_lp_sl_max,)
    } else {
        (locals.var_lp_sl,)
    }
};
            locals.var_lp_sl = assign12700_body29_e15373;
            let (assign12700_body30_e15388, assign12700_body30_e15388_d_n0, assign12700_body30_e15388_d_n2, assign12700_body30_e15388_d_n6, assign12700_body30_e15388_d_n7, assign12700_body30_e15388_d_n10, assign12700_body30_e15388_d_n11, assign12700_body30_e15388_d_n12, assign12700_body30_e15388_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 != 0.0)) {
        (locals.var_t6__blk337, locals.var_t6__blk337_dn0, locals.var_t6__blk337_dn2, locals.var_t6__blk337_dn6, locals.var_t6__blk337_dn7, locals.var_t6__blk337_dn10, locals.var_t6__blk337_dn11, locals.var_t6__blk337_dn12, locals.var_t6__blk337_dn17,)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
            locals.var_phi_sl_bulk = assign12700_body30_e15388;
            locals.var_phi_sl_bulk_dn0 = assign12700_body30_e15388_d_n0;
            locals.var_phi_sl_bulk_dn2 = assign12700_body30_e15388_d_n2;
            locals.var_phi_sl_bulk_dn6 = assign12700_body30_e15388_d_n6;
            locals.var_phi_sl_bulk_dn7 = assign12700_body30_e15388_d_n7;
            locals.var_phi_sl_bulk_dn10 = assign12700_body30_e15388_d_n10;
            locals.var_phi_sl_bulk_dn11 = assign12700_body30_e15388_d_n11;
            locals.var_phi_sl_bulk_dn12 = assign12700_body30_e15388_d_n12;
            locals.var_phi_sl_bulk_dn17 = assign12700_body30_e15388_d_n17;
            let (assign12700_body31_e15403, assign12700_body31_e15403_d_n0, assign12700_body31_e15403_d_n2, assign12700_body31_e15403_d_n6, assign12700_body31_e15403_d_n7, assign12700_body31_e15403_d_n10, assign12700_body31_e15403_d_n11, assign12700_body31_e15403_d_n12, assign12700_body31_e15403_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 != 0.0)) {
        (locals.var_t4__blk335, locals.var_t4__blk335_dn0, locals.var_t4__blk335_dn2, locals.var_t4__blk335_dn6, locals.var_t4__blk335_dn7, locals.var_t4__blk335_dn10, locals.var_t4__blk335_dn11, locals.var_t4__blk335_dn12, locals.var_t4__blk335_dn17,)
    } else {
        (locals.var_q_sl_bulk, locals.var_q_sl_bulk_dn0, locals.var_q_sl_bulk_dn2, locals.var_q_sl_bulk_dn6, locals.var_q_sl_bulk_dn7, locals.var_q_sl_bulk_dn10, locals.var_q_sl_bulk_dn11, locals.var_q_sl_bulk_dn12, locals.var_q_sl_bulk_dn17,)
    }
};
            locals.var_q_sl_bulk = assign12700_body31_e15403;
            locals.var_q_sl_bulk_dn0 = assign12700_body31_e15403_d_n0;
            locals.var_q_sl_bulk_dn2 = assign12700_body31_e15403_d_n2;
            locals.var_q_sl_bulk_dn6 = assign12700_body31_e15403_d_n6;
            locals.var_q_sl_bulk_dn7 = assign12700_body31_e15403_d_n7;
            locals.var_q_sl_bulk_dn10 = assign12700_body31_e15403_d_n10;
            locals.var_q_sl_bulk_dn11 = assign12700_body31_e15403_d_n11;
            locals.var_q_sl_bulk_dn12 = assign12700_body31_e15403_d_n12;
            locals.var_q_sl_bulk_dn17 = assign12700_body31_e15403_d_n17;
            let (assign12700_body32_e15420,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 != 0.0)) {
        let assign12700_body32_e15418: f64 = (locals.var_lp_sl + 1.0);
        (assign12700_body32_e15418,)
    } else {
        (locals.var_lp_sl,)
    }
};
            locals.var_lp_sl = assign12700_body32_e15420;
        }

    }

    pub(super) fn stamp_transient_block_38(
        locals: &mut StampLocals,
    ) {
        let (assign12710_e15437, assign12710_e15437_d_n0, assign12710_e15437_d_n2, assign12710_e15437_d_n6, assign12710_e15437_d_n7, assign12710_e15437_d_n10, assign12710_e15437_d_n11, assign12710_e15437_d_n12, assign12710_e15437_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 != 0.0)) {
        let assign12710_e15435: f64 = (locals.var_vbsbiz + locals.var_phi_sl_bulk);
        (assign12710_e15435, (locals.var_vbsbiz_dn0 + locals.var_phi_sl_bulk_dn0), (locals.var_vbsbiz_dn2 + locals.var_phi_sl_bulk_dn2), (locals.var_vbsbiz_dn6 + locals.var_phi_sl_bulk_dn6), (locals.var_vbsbiz_dn7 + locals.var_phi_sl_bulk_dn7), (locals.var_vbsbiz_dn10 + locals.var_phi_sl_bulk_dn10), (locals.var_vbsbiz_dn11 + locals.var_phi_sl_bulk_dn11), (locals.var_vbsbiz_dn12 + locals.var_phi_sl_bulk_dn12), (locals.var_vbsbiz_dn17 + locals.var_phi_sl_bulk_dn17),)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
        locals.var_phi_sl_bulk = assign12710_e15437;
        locals.var_phi_sl_bulk_dn0 = assign12710_e15437_d_n0;
        locals.var_phi_sl_bulk_dn2 = assign12710_e15437_d_n2;
        locals.var_phi_sl_bulk_dn6 = assign12710_e15437_d_n6;
        locals.var_phi_sl_bulk_dn7 = assign12710_e15437_d_n7;
        locals.var_phi_sl_bulk_dn10 = assign12710_e15437_d_n10;
        locals.var_phi_sl_bulk_dn11 = assign12710_e15437_d_n11;
        locals.var_phi_sl_bulk_dn12 = assign12710_e15437_d_n12;
        locals.var_phi_sl_bulk_dn17 = assign12710_e15437_d_n17;

        let (assign12720_e15456, assign12720_e15456_d_n0, assign12720_e15456_d_n2, assign12720_e15456_d_n6, assign12720_e15456_d_n7, assign12720_e15456_d_n10, assign12720_e15456_d_n11, assign12720_e15456_d_n12, assign12720_e15456_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 != 0.0)) {
        let assign12720_e15453: f64 = (locals.var_q_sl_bulk / locals.var_c_box);
        let assign12720_e15454: f64 = (locals.var_phi_sl_bulk - assign12720_e15453);
        (assign12720_e15454, (locals.var_phi_sl_bulk_dn0 - (locals.var_q_sl_bulk_dn0 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn2 - (locals.var_q_sl_bulk_dn2 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn6 - (locals.var_q_sl_bulk_dn6 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn7 - (locals.var_q_sl_bulk_dn7 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn10 - (locals.var_q_sl_bulk_dn10 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn11 - (locals.var_q_sl_bulk_dn11 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn12 - (locals.var_q_sl_bulk_dn12 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn17 - (locals.var_q_sl_bulk_dn17 / locals.var_c_box)),)
    } else {
        (locals.var_phi_bl_soi, locals.var_phi_bl_soi_dn0, locals.var_phi_bl_soi_dn2, locals.var_phi_bl_soi_dn6, locals.var_phi_bl_soi_dn7, locals.var_phi_bl_soi_dn10, locals.var_phi_bl_soi_dn11, locals.var_phi_bl_soi_dn12, locals.var_phi_bl_soi_dn17,)
    }
};
        locals.var_phi_bl_soi = assign12720_e15456;
        locals.var_phi_bl_soi_dn0 = assign12720_e15456_d_n0;
        locals.var_phi_bl_soi_dn2 = assign12720_e15456_d_n2;
        locals.var_phi_bl_soi_dn6 = assign12720_e15456_d_n6;
        locals.var_phi_bl_soi_dn7 = assign12720_e15456_d_n7;
        locals.var_phi_bl_soi_dn10 = assign12720_e15456_d_n10;
        locals.var_phi_bl_soi_dn11 = assign12720_e15456_d_n11;
        locals.var_phi_bl_soi_dn12 = assign12720_e15456_d_n12;
        locals.var_phi_bl_soi_dn17 = assign12720_e15456_d_n17;

        let (assign12740_e15488,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_lp_sl,)
    }
};
        locals.var_lp_sl = assign12740_e15488;

    }

    pub(super) fn stamp_transient_block_39(
        locals: &mut StampLocals,
    ) {
        let mut assign12750_loop_guard: usize = 0;
        while {
            let assign12750_cond_e15505: f64 = if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 == 0.0)) && (locals.var_lp_sl < locals.var_lp_sl_max)) { 1.0 } else { 0.0 };
            assign12750_cond_e15505 != 0.0
        } {
            assign12750_loop_guard += 1;
            assert!(assign12750_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign12750_body0_e15521, assign12750_body0_e15521_d_n10,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 == 0.0)) {
        (locals.var_cnst0bulk, locals.var_cnst0bulk_dn10,)
    } else {
        (locals.var_t1__blk332, locals.var_t1__blk332_dn10,)
    }
};
            locals.var_t1__blk332 = assign12750_body0_e15521;
            locals.var_t1__blk332_dn10 = assign12750_body0_e15521_d_n10;
            let (assign12750_body1_e15539, assign12750_body1_e15539_d_n0, assign12750_body1_e15539_d_n2, assign12750_body1_e15539_d_n6, assign12750_body1_e15539_d_n7, assign12750_body1_e15539_d_n10, assign12750_body1_e15539_d_n11, assign12750_body1_e15539_d_n12, assign12750_body1_e15539_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 == 0.0)) {
        let assign12750_body1_e15537: f64 = (locals.var_beta * locals.var_phi_sl_bulk);
        (assign12750_body1_e15537, (locals.var_beta * locals.var_phi_sl_bulk_dn0), (locals.var_beta * locals.var_phi_sl_bulk_dn2), (locals.var_beta * locals.var_phi_sl_bulk_dn6), (locals.var_beta * locals.var_phi_sl_bulk_dn7), ((locals.var_beta_dn10 * locals.var_phi_sl_bulk) + (locals.var_beta * locals.var_phi_sl_bulk_dn10)), (locals.var_beta * locals.var_phi_sl_bulk_dn11), (locals.var_beta * locals.var_phi_sl_bulk_dn12), (locals.var_beta * locals.var_phi_sl_bulk_dn17),)
    } else {
        (locals.var_t2__blk333, locals.var_t2__blk333_dn0, locals.var_t2__blk333_dn2, locals.var_t2__blk333_dn6, locals.var_t2__blk333_dn7, locals.var_t2__blk333_dn10, locals.var_t2__blk333_dn11, locals.var_t2__blk333_dn12, locals.var_t2__blk333_dn17,)
    }
};
            locals.var_t2__blk333 = assign12750_body1_e15539;
            locals.var_t2__blk333_dn0 = assign12750_body1_e15539_d_n0;
            locals.var_t2__blk333_dn2 = assign12750_body1_e15539_d_n2;
            locals.var_t2__blk333_dn6 = assign12750_body1_e15539_d_n6;
            locals.var_t2__blk333_dn7 = assign12750_body1_e15539_d_n7;
            locals.var_t2__blk333_dn10 = assign12750_body1_e15539_d_n10;
            locals.var_t2__blk333_dn11 = assign12750_body1_e15539_d_n11;
            locals.var_t2__blk333_dn12 = assign12750_body1_e15539_d_n12;
            locals.var_t2__blk333_dn17 = assign12750_body1_e15539_d_n17;
            let (assign12750_body2_e15557, assign12750_body2_e15557_d_n0, assign12750_body2_e15557_d_n2, assign12750_body2_e15557_d_n6, assign12750_body2_e15557_d_n7, assign12750_body2_e15557_d_n10, assign12750_body2_e15557_d_n11, assign12750_body2_e15557_d_n12, assign12750_body2_e15557_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 == 0.0)) {
        let assign12750_body2_e15554: f64 = (-locals.var_t2__blk333);
        let assign12750_body2_e15555: f64 = (assign12750_body2_e15554).exp();
        (assign12750_body2_e15555, (assign12750_body2_e15555 * (-locals.var_t2__blk333_dn0)), (assign12750_body2_e15555 * (-locals.var_t2__blk333_dn2)), (assign12750_body2_e15555 * (-locals.var_t2__blk333_dn6)), (assign12750_body2_e15555 * (-locals.var_t2__blk333_dn7)), (assign12750_body2_e15555 * (-locals.var_t2__blk333_dn10)), (assign12750_body2_e15555 * (-locals.var_t2__blk333_dn11)), (assign12750_body2_e15555 * (-locals.var_t2__blk333_dn12)), (assign12750_body2_e15555 * (-locals.var_t2__blk333_dn17)),)
    } else {
        (locals.var_t3__blk334, locals.var_t3__blk334_dn0, locals.var_t3__blk334_dn2, locals.var_t3__blk334_dn6, locals.var_t3__blk334_dn7, locals.var_t3__blk334_dn10, locals.var_t3__blk334_dn11, locals.var_t3__blk334_dn12, locals.var_t3__blk334_dn17,)
    }
};
            locals.var_t3__blk334 = assign12750_body2_e15557;
            locals.var_t3__blk334_dn0 = assign12750_body2_e15557_d_n0;
            locals.var_t3__blk334_dn2 = assign12750_body2_e15557_d_n2;
            locals.var_t3__blk334_dn6 = assign12750_body2_e15557_d_n6;
            locals.var_t3__blk334_dn7 = assign12750_body2_e15557_d_n7;
            locals.var_t3__blk334_dn10 = assign12750_body2_e15557_d_n10;
            locals.var_t3__blk334_dn11 = assign12750_body2_e15557_d_n11;
            locals.var_t3__blk334_dn12 = assign12750_body2_e15557_d_n12;
            locals.var_t3__blk334_dn17 = assign12750_body2_e15557_d_n17;
            let assign12750_body3_e15560: f64 = if locals.var_phi_sl_bulk > 1e-9 { 1.0 } else { 0.0 };
            locals.var_guard345 = assign12750_body3_e15560;
            let (assign12750_body4_e15581, assign12750_body4_e15581_d_n0, assign12750_body4_e15581_d_n2, assign12750_body4_e15581_d_n6, assign12750_body4_e15581_d_n7, assign12750_body4_e15581_d_n10, assign12750_body4_e15581_d_n11, assign12750_body4_e15581_d_n12, assign12750_body4_e15581_d_n17,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 == 0.0)) && (locals.var_guard345 != 0.0)) {
        let assign12750_body4_e15578: f64 = (locals.var_beta * locals.var_phi_sl_bulk);
        let assign12750_body4_e15579: f64 = (assign12750_body4_e15578).exp();
        (assign12750_body4_e15579, (assign12750_body4_e15579 * (locals.var_beta * locals.var_phi_sl_bulk_dn0)), (assign12750_body4_e15579 * (locals.var_beta * locals.var_phi_sl_bulk_dn2)), (assign12750_body4_e15579 * (locals.var_beta * locals.var_phi_sl_bulk_dn6)), (assign12750_body4_e15579 * (locals.var_beta * locals.var_phi_sl_bulk_dn7)), (assign12750_body4_e15579 * ((locals.var_beta_dn10 * locals.var_phi_sl_bulk) + (locals.var_beta * locals.var_phi_sl_bulk_dn10))), (assign12750_body4_e15579 * (locals.var_beta * locals.var_phi_sl_bulk_dn11)), (assign12750_body4_e15579 * (locals.var_beta * locals.var_phi_sl_bulk_dn12)), (assign12750_body4_e15579 * (locals.var_beta * locals.var_phi_sl_bulk_dn17)),)
    } else {
        (locals.var_t0__blk331, locals.var_t0__blk331_dn0, locals.var_t0__blk331_dn2, locals.var_t0__blk331_dn6, locals.var_t0__blk331_dn7, locals.var_t0__blk331_dn10, locals.var_t0__blk331_dn11, locals.var_t0__blk331_dn12, locals.var_t0__blk331_dn17,)
    }
};
            locals.var_t0__blk331 = assign12750_body4_e15581;
            locals.var_t0__blk331_dn0 = assign12750_body4_e15581_d_n0;
            locals.var_t0__blk331_dn2 = assign12750_body4_e15581_d_n2;
            locals.var_t0__blk331_dn6 = assign12750_body4_e15581_d_n6;
            locals.var_t0__blk331_dn7 = assign12750_body4_e15581_d_n7;
            locals.var_t0__blk331_dn10 = assign12750_body4_e15581_d_n10;
            locals.var_t0__blk331_dn11 = assign12750_body4_e15581_d_n11;
            locals.var_t0__blk331_dn12 = assign12750_body4_e15581_d_n12;
            locals.var_t0__blk331_dn17 = assign12750_body4_e15581_d_n17;
            let (assign12750_body5_e15613, assign12750_body5_e15613_d_n0, assign12750_body5_e15613_d_n2, assign12750_body5_e15613_d_n6, assign12750_body5_e15613_d_n7, assign12750_body5_e15613_d_n10, assign12750_body5_e15613_d_n11, assign12750_body5_e15613_d_n12, assign12750_body5_e15613_d_n17,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 == 0.0)) && (locals.var_guard345 != 0.0)) {
        let assign12750_body5_e15598: f64 = (-locals.var_t1__blk332);
        let assign12750_body5_e15601: f64 = (locals.var_t3__blk334 + locals.var_t2__blk333);
        let assign12750_body5_e15603: f64 = (assign12750_body5_e15601 - 1.0);
        let assign12750_body5_e15607: f64 = (locals.var_t0__blk331 - 1.0);
        let assign12750_body5_e15608: f64 = (locals.var_cnst1bulk * assign12750_body5_e15607);
        let assign12750_body5_e15609: f64 = (assign12750_body5_e15603 + assign12750_body5_e15608);
        let assign12750_body5_e15610: f64 = (assign12750_body5_e15609).sqrt();
        let assign12750_body5_e15611: f64 = (assign12750_body5_e15598 * assign12750_body5_e15610);
        (assign12750_body5_e15611, (assign12750_body5_e15598 * (((locals.var_t3__blk334_dn0 + locals.var_t2__blk333_dn0) + ((locals.var_cnst1bulk_dn0 * assign12750_body5_e15607) + (locals.var_cnst1bulk * locals.var_t0__blk331_dn0))) / (2.0 * assign12750_body5_e15610))), (assign12750_body5_e15598 * (((locals.var_t3__blk334_dn2 + locals.var_t2__blk333_dn2) + ((locals.var_cnst1bulk_dn2 * assign12750_body5_e15607) + (locals.var_cnst1bulk * locals.var_t0__blk331_dn2))) / (2.0 * assign12750_body5_e15610))), (assign12750_body5_e15598 * (((locals.var_t3__blk334_dn6 + locals.var_t2__blk333_dn6) + ((locals.var_cnst1bulk_dn6 * assign12750_body5_e15607) + (locals.var_cnst1bulk * locals.var_t0__blk331_dn6))) / (2.0 * assign12750_body5_e15610))), (assign12750_body5_e15598 * (((locals.var_t3__blk334_dn7 + locals.var_t2__blk333_dn7) + ((locals.var_cnst1bulk_dn7 * assign12750_body5_e15607) + (locals.var_cnst1bulk * locals.var_t0__blk331_dn7))) / (2.0 * assign12750_body5_e15610))), (((-locals.var_t1__blk332_dn10) * assign12750_body5_e15610) + (assign12750_body5_e15598 * (((locals.var_t3__blk334_dn10 + locals.var_t2__blk333_dn10) + ((locals.var_cnst1bulk_dn10 * assign12750_body5_e15607) + (locals.var_cnst1bulk * locals.var_t0__blk331_dn10))) / (2.0 * assign12750_body5_e15610)))), (assign12750_body5_e15598 * (((locals.var_t3__blk334_dn11 + locals.var_t2__blk333_dn11) + ((locals.var_cnst1bulk_dn11 * assign12750_body5_e15607) + (locals.var_cnst1bulk * locals.var_t0__blk331_dn11))) / (2.0 * assign12750_body5_e15610))), (assign12750_body5_e15598 * (((locals.var_t3__blk334_dn12 + locals.var_t2__blk333_dn12) + ((locals.var_cnst1bulk_dn12 * assign12750_body5_e15607) + (locals.var_cnst1bulk * locals.var_t0__blk331_dn12))) / (2.0 * assign12750_body5_e15610))), (assign12750_body5_e15598 * (((locals.var_t3__blk334_dn17 + locals.var_t2__blk333_dn17) + ((locals.var_cnst1bulk_dn17 * assign12750_body5_e15607) + (locals.var_cnst1bulk * locals.var_t0__blk331_dn17))) / (2.0 * assign12750_body5_e15610))),)
    } else {
        (locals.var_t4__blk335, locals.var_t4__blk335_dn0, locals.var_t4__blk335_dn2, locals.var_t4__blk335_dn6, locals.var_t4__blk335_dn7, locals.var_t4__blk335_dn10, locals.var_t4__blk335_dn11, locals.var_t4__blk335_dn12, locals.var_t4__blk335_dn17,)
    }
};
            locals.var_t4__blk335 = assign12750_body5_e15613;
            locals.var_t4__blk335_dn0 = assign12750_body5_e15613_d_n0;
            locals.var_t4__blk335_dn2 = assign12750_body5_e15613_d_n2;
            locals.var_t4__blk335_dn6 = assign12750_body5_e15613_d_n6;
            locals.var_t4__blk335_dn7 = assign12750_body5_e15613_d_n7;
            locals.var_t4__blk335_dn10 = assign12750_body5_e15613_d_n10;
            locals.var_t4__blk335_dn11 = assign12750_body5_e15613_d_n11;
            locals.var_t4__blk335_dn12 = assign12750_body5_e15613_d_n12;
            locals.var_t4__blk335_dn17 = assign12750_body5_e15613_d_n17;
            let (assign12750_body6_e15642, assign12750_body6_e15642_d_n0, assign12750_body6_e15642_d_n2, assign12750_body6_e15642_d_n6, assign12750_body6_e15642_d_n7, assign12750_body6_e15642_d_n10, assign12750_body6_e15642_d_n11, assign12750_body6_e15642_d_n12, assign12750_body6_e15642_d_n17,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 == 0.0)) && (locals.var_guard345 != 0.0)) {
        let assign12750_body6_e15631: f64 = (locals.var_c0bulk / locals.var_t4__blk335);
        let assign12750_body6_e15633: f64 = (-locals.var_t3__blk334);
        let assign12750_body6_e15635: f64 = (assign12750_body6_e15633 + 1.0);
        let assign12750_body6_e15638: f64 = (locals.var_cnst1bulk * locals.var_t0__blk331);
        let assign12750_body6_e15639: f64 = (assign12750_body6_e15635 + assign12750_body6_e15638);
        let assign12750_body6_e15640: f64 = (assign12750_body6_e15631 * assign12750_body6_e15639);
        (assign12750_body6_e15640, (((-((locals.var_c0bulk * locals.var_t4__blk335_dn0) / (locals.var_t4__blk335 * locals.var_t4__blk335))) * assign12750_body6_e15639) + (assign12750_body6_e15631 * ((-locals.var_t3__blk334_dn0) + ((locals.var_cnst1bulk_dn0 * locals.var_t0__blk331) + (locals.var_cnst1bulk * locals.var_t0__blk331_dn0))))), (((-((locals.var_c0bulk * locals.var_t4__blk335_dn2) / (locals.var_t4__blk335 * locals.var_t4__blk335))) * assign12750_body6_e15639) + (assign12750_body6_e15631 * ((-locals.var_t3__blk334_dn2) + ((locals.var_cnst1bulk_dn2 * locals.var_t0__blk331) + (locals.var_cnst1bulk * locals.var_t0__blk331_dn2))))), (((-((locals.var_c0bulk * locals.var_t4__blk335_dn6) / (locals.var_t4__blk335 * locals.var_t4__blk335))) * assign12750_body6_e15639) + (assign12750_body6_e15631 * ((-locals.var_t3__blk334_dn6) + ((locals.var_cnst1bulk_dn6 * locals.var_t0__blk331) + (locals.var_cnst1bulk * locals.var_t0__blk331_dn6))))), (((-((locals.var_c0bulk * locals.var_t4__blk335_dn7) / (locals.var_t4__blk335 * locals.var_t4__blk335))) * assign12750_body6_e15639) + (assign12750_body6_e15631 * ((-locals.var_t3__blk334_dn7) + ((locals.var_cnst1bulk_dn7 * locals.var_t0__blk331) + (locals.var_cnst1bulk * locals.var_t0__blk331_dn7))))), (((-((locals.var_c0bulk * locals.var_t4__blk335_dn10) / (locals.var_t4__blk335 * locals.var_t4__blk335))) * assign12750_body6_e15639) + (assign12750_body6_e15631 * ((-locals.var_t3__blk334_dn10) + ((locals.var_cnst1bulk_dn10 * locals.var_t0__blk331) + (locals.var_cnst1bulk * locals.var_t0__blk331_dn10))))), (((-((locals.var_c0bulk * locals.var_t4__blk335_dn11) / (locals.var_t4__blk335 * locals.var_t4__blk335))) * assign12750_body6_e15639) + (assign12750_body6_e15631 * ((-locals.var_t3__blk334_dn11) + ((locals.var_cnst1bulk_dn11 * locals.var_t0__blk331) + (locals.var_cnst1bulk * locals.var_t0__blk331_dn11))))), (((-((locals.var_c0bulk * locals.var_t4__blk335_dn12) / (locals.var_t4__blk335 * locals.var_t4__blk335))) * assign12750_body6_e15639) + (assign12750_body6_e15631 * ((-locals.var_t3__blk334_dn12) + ((locals.var_cnst1bulk_dn12 * locals.var_t0__blk331) + (locals.var_cnst1bulk * locals.var_t0__blk331_dn12))))), (((-((locals.var_c0bulk * locals.var_t4__blk335_dn17) / (locals.var_t4__blk335 * locals.var_t4__blk335))) * assign12750_body6_e15639) + (assign12750_body6_e15631 * ((-locals.var_t3__blk334_dn17) + ((locals.var_cnst1bulk_dn17 * locals.var_t0__blk331) + (locals.var_cnst1bulk * locals.var_t0__blk331_dn17))))),)
    } else {
        (locals.var_t5__blk336, locals.var_t5__blk336_dn0, locals.var_t5__blk336_dn2, locals.var_t5__blk336_dn6, locals.var_t5__blk336_dn7, locals.var_t5__blk336_dn10, locals.var_t5__blk336_dn11, locals.var_t5__blk336_dn12, locals.var_t5__blk336_dn17,)
    }
};
            locals.var_t5__blk336 = assign12750_body6_e15642;
            locals.var_t5__blk336_dn0 = assign12750_body6_e15642_d_n0;
            locals.var_t5__blk336_dn2 = assign12750_body6_e15642_d_n2;
            locals.var_t5__blk336_dn6 = assign12750_body6_e15642_d_n6;
            locals.var_t5__blk336_dn7 = assign12750_body6_e15642_d_n7;
            locals.var_t5__blk336_dn10 = assign12750_body6_e15642_d_n10;
            locals.var_t5__blk336_dn11 = assign12750_body6_e15642_d_n11;
            locals.var_t5__blk336_dn12 = assign12750_body6_e15642_d_n12;
            locals.var_t5__blk336_dn17 = assign12750_body6_e15642_d_n17;
            let assign12750_body7_e15645: f64 = (-1e-9);
            let assign12750_body7_e15646: f64 = if locals.var_phi_sl_bulk < assign12750_body7_e15645 { 1.0 } else { 0.0 };
            locals.var_guard346 = assign12750_body7_e15646;
            let (assign12750_body8_e15674, assign12750_body8_e15674_d_n0, assign12750_body8_e15674_d_n2, assign12750_body8_e15674_d_n6, assign12750_body8_e15674_d_n7, assign12750_body8_e15674_d_n10, assign12750_body8_e15674_d_n11, assign12750_body8_e15674_d_n12, assign12750_body8_e15674_d_n17,) = {
    if (((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 == 0.0)) && (locals.var_guard345 == 0.0)) && (locals.var_guard346 != 0.0)) {
        let assign12750_body8_e15668: f64 = (locals.var_t3__blk334 + locals.var_t2__blk333);
        let assign12750_body8_e15670: f64 = (assign12750_body8_e15668 - 1.0);
        let assign12750_body8_e15671: f64 = (assign12750_body8_e15670).sqrt();
        let assign12750_body8_e15672: f64 = (locals.var_t1__blk332 * assign12750_body8_e15671);
        (assign12750_body8_e15672, (locals.var_t1__blk332 * ((locals.var_t3__blk334_dn0 + locals.var_t2__blk333_dn0) / (2.0 * assign12750_body8_e15671))), (locals.var_t1__blk332 * ((locals.var_t3__blk334_dn2 + locals.var_t2__blk333_dn2) / (2.0 * assign12750_body8_e15671))), (locals.var_t1__blk332 * ((locals.var_t3__blk334_dn6 + locals.var_t2__blk333_dn6) / (2.0 * assign12750_body8_e15671))), (locals.var_t1__blk332 * ((locals.var_t3__blk334_dn7 + locals.var_t2__blk333_dn7) / (2.0 * assign12750_body8_e15671))), ((locals.var_t1__blk332_dn10 * assign12750_body8_e15671) + (locals.var_t1__blk332 * ((locals.var_t3__blk334_dn10 + locals.var_t2__blk333_dn10) / (2.0 * assign12750_body8_e15671)))), (locals.var_t1__blk332 * ((locals.var_t3__blk334_dn11 + locals.var_t2__blk333_dn11) / (2.0 * assign12750_body8_e15671))), (locals.var_t1__blk332 * ((locals.var_t3__blk334_dn12 + locals.var_t2__blk333_dn12) / (2.0 * assign12750_body8_e15671))), (locals.var_t1__blk332 * ((locals.var_t3__blk334_dn17 + locals.var_t2__blk333_dn17) / (2.0 * assign12750_body8_e15671))),)
    } else {
        (locals.var_t4__blk335, locals.var_t4__blk335_dn0, locals.var_t4__blk335_dn2, locals.var_t4__blk335_dn6, locals.var_t4__blk335_dn7, locals.var_t4__blk335_dn10, locals.var_t4__blk335_dn11, locals.var_t4__blk335_dn12, locals.var_t4__blk335_dn17,)
    }
};
            locals.var_t4__blk335 = assign12750_body8_e15674;
            locals.var_t4__blk335_dn0 = assign12750_body8_e15674_d_n0;
            locals.var_t4__blk335_dn2 = assign12750_body8_e15674_d_n2;
            locals.var_t4__blk335_dn6 = assign12750_body8_e15674_d_n6;
            locals.var_t4__blk335_dn7 = assign12750_body8_e15674_d_n7;
            locals.var_t4__blk335_dn10 = assign12750_body8_e15674_d_n10;
            locals.var_t4__blk335_dn11 = assign12750_body8_e15674_d_n11;
            locals.var_t4__blk335_dn12 = assign12750_body8_e15674_d_n12;
            locals.var_t4__blk335_dn17 = assign12750_body8_e15674_d_n17;
            let (assign12750_body9_e15702, assign12750_body9_e15702_d_n0, assign12750_body9_e15702_d_n2, assign12750_body9_e15702_d_n6, assign12750_body9_e15702_d_n7, assign12750_body9_e15702_d_n10, assign12750_body9_e15702_d_n11, assign12750_body9_e15702_d_n12, assign12750_body9_e15702_d_n17,) = {
    if (((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 == 0.0)) && (locals.var_guard345 == 0.0)) && (locals.var_guard346 != 0.0)) {
        let assign12750_body9_e15695: f64 = (locals.var_c0bulk / locals.var_t4__blk335);
        let assign12750_body9_e15697: f64 = (-locals.var_t3__blk334);
        let assign12750_body9_e15699: f64 = (assign12750_body9_e15697 + 1.0);
        let assign12750_body9_e15700: f64 = (assign12750_body9_e15695 * assign12750_body9_e15699);
        (assign12750_body9_e15700, (((-((locals.var_c0bulk * locals.var_t4__blk335_dn0) / (locals.var_t4__blk335 * locals.var_t4__blk335))) * assign12750_body9_e15699) + (assign12750_body9_e15695 * (-locals.var_t3__blk334_dn0))), (((-((locals.var_c0bulk * locals.var_t4__blk335_dn2) / (locals.var_t4__blk335 * locals.var_t4__blk335))) * assign12750_body9_e15699) + (assign12750_body9_e15695 * (-locals.var_t3__blk334_dn2))), (((-((locals.var_c0bulk * locals.var_t4__blk335_dn6) / (locals.var_t4__blk335 * locals.var_t4__blk335))) * assign12750_body9_e15699) + (assign12750_body9_e15695 * (-locals.var_t3__blk334_dn6))), (((-((locals.var_c0bulk * locals.var_t4__blk335_dn7) / (locals.var_t4__blk335 * locals.var_t4__blk335))) * assign12750_body9_e15699) + (assign12750_body9_e15695 * (-locals.var_t3__blk334_dn7))), (((-((locals.var_c0bulk * locals.var_t4__blk335_dn10) / (locals.var_t4__blk335 * locals.var_t4__blk335))) * assign12750_body9_e15699) + (assign12750_body9_e15695 * (-locals.var_t3__blk334_dn10))), (((-((locals.var_c0bulk * locals.var_t4__blk335_dn11) / (locals.var_t4__blk335 * locals.var_t4__blk335))) * assign12750_body9_e15699) + (assign12750_body9_e15695 * (-locals.var_t3__blk334_dn11))), (((-((locals.var_c0bulk * locals.var_t4__blk335_dn12) / (locals.var_t4__blk335 * locals.var_t4__blk335))) * assign12750_body9_e15699) + (assign12750_body9_e15695 * (-locals.var_t3__blk334_dn12))), (((-((locals.var_c0bulk * locals.var_t4__blk335_dn17) / (locals.var_t4__blk335 * locals.var_t4__blk335))) * assign12750_body9_e15699) + (assign12750_body9_e15695 * (-locals.var_t3__blk334_dn17))),)
    } else {
        (locals.var_t5__blk336, locals.var_t5__blk336_dn0, locals.var_t5__blk336_dn2, locals.var_t5__blk336_dn6, locals.var_t5__blk336_dn7, locals.var_t5__blk336_dn10, locals.var_t5__blk336_dn11, locals.var_t5__blk336_dn12, locals.var_t5__blk336_dn17,)
    }
};
            locals.var_t5__blk336 = assign12750_body9_e15702;
            locals.var_t5__blk336_dn0 = assign12750_body9_e15702_d_n0;
            locals.var_t5__blk336_dn2 = assign12750_body9_e15702_d_n2;
            locals.var_t5__blk336_dn6 = assign12750_body9_e15702_d_n6;
            locals.var_t5__blk336_dn7 = assign12750_body9_e15702_d_n7;
            locals.var_t5__blk336_dn10 = assign12750_body9_e15702_d_n10;
            locals.var_t5__blk336_dn11 = assign12750_body9_e15702_d_n11;
            locals.var_t5__blk336_dn12 = assign12750_body9_e15702_d_n12;
            locals.var_t5__blk336_dn17 = assign12750_body9_e15702_d_n17;
            let (assign12750_body10_e15732, assign12750_body10_e15732_d_n0, assign12750_body10_e15732_d_n2, assign12750_body10_e15732_d_n6, assign12750_body10_e15732_d_n7, assign12750_body10_e15732_d_n10, assign12750_body10_e15732_d_n11, assign12750_body10_e15732_d_n12, assign12750_body10_e15732_d_n17,) = {
    if (((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 == 0.0)) && (locals.var_guard345 == 0.0)) && (locals.var_guard346 == 0.0)) {
        let assign12750_body10_e15724: f64 = (locals.var_c0bulk / locals.var_beta);
        let assign12750_body10_e15725: f64 = (assign12750_body10_e15724).sqrt();
        let assign12750_body10_e15726: f64 = (-assign12750_body10_e15725);
        let assign12750_body10_e15728: f64 = (assign12750_body10_e15726 * locals.var_beta);
        let assign12750_body10_e15730: f64 = (assign12750_body10_e15728 * locals.var_phi_sl_bulk);
        (assign12750_body10_e15730, (assign12750_body10_e15728 * locals.var_phi_sl_bulk_dn0), (assign12750_body10_e15728 * locals.var_phi_sl_bulk_dn2), (assign12750_body10_e15728 * locals.var_phi_sl_bulk_dn6), (assign12750_body10_e15728 * locals.var_phi_sl_bulk_dn7), (((((-((-((locals.var_c0bulk * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) / (2.0 * assign12750_body10_e15725))) * locals.var_beta) + (assign12750_body10_e15726 * locals.var_beta_dn10)) * locals.var_phi_sl_bulk) + (assign12750_body10_e15728 * locals.var_phi_sl_bulk_dn10)), (assign12750_body10_e15728 * locals.var_phi_sl_bulk_dn11), (assign12750_body10_e15728 * locals.var_phi_sl_bulk_dn12), (assign12750_body10_e15728 * locals.var_phi_sl_bulk_dn17),)
    } else {
        (locals.var_t4__blk335, locals.var_t4__blk335_dn0, locals.var_t4__blk335_dn2, locals.var_t4__blk335_dn6, locals.var_t4__blk335_dn7, locals.var_t4__blk335_dn10, locals.var_t4__blk335_dn11, locals.var_t4__blk335_dn12, locals.var_t4__blk335_dn17,)
    }
};
            locals.var_t4__blk335 = assign12750_body10_e15732;
            locals.var_t4__blk335_dn0 = assign12750_body10_e15732_d_n0;
            locals.var_t4__blk335_dn2 = assign12750_body10_e15732_d_n2;
            locals.var_t4__blk335_dn6 = assign12750_body10_e15732_d_n6;
            locals.var_t4__blk335_dn7 = assign12750_body10_e15732_d_n7;
            locals.var_t4__blk335_dn10 = assign12750_body10_e15732_d_n10;
            locals.var_t4__blk335_dn11 = assign12750_body10_e15732_d_n11;
            locals.var_t4__blk335_dn12 = assign12750_body10_e15732_d_n12;
            locals.var_t4__blk335_dn17 = assign12750_body10_e15732_d_n17;
            let (assign12750_body11_e15758, assign12750_body11_e15758_d_n0, assign12750_body11_e15758_d_n2, assign12750_body11_e15758_d_n6, assign12750_body11_e15758_d_n7, assign12750_body11_e15758_d_n10, assign12750_body11_e15758_d_n11, assign12750_body11_e15758_d_n12, assign12750_body11_e15758_d_n17,) = {
    if (((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 == 0.0)) && (locals.var_guard345 == 0.0)) && (locals.var_guard346 == 0.0)) {
        let assign12750_body11_e15754: f64 = (locals.var_c0bulk * locals.var_beta);
        let assign12750_body11_e15755: f64 = (assign12750_body11_e15754).sqrt();
        let assign12750_body11_e15756: f64 = (-assign12750_body11_e15755);
        (assign12750_body11_e15756, 0.0, 0.0, 0.0, 0.0, (-((locals.var_c0bulk * locals.var_beta_dn10) / (2.0 * assign12750_body11_e15755))), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5__blk336, locals.var_t5__blk336_dn0, locals.var_t5__blk336_dn2, locals.var_t5__blk336_dn6, locals.var_t5__blk336_dn7, locals.var_t5__blk336_dn10, locals.var_t5__blk336_dn11, locals.var_t5__blk336_dn12, locals.var_t5__blk336_dn17,)
    }
};
            locals.var_t5__blk336 = assign12750_body11_e15758;
            locals.var_t5__blk336_dn0 = assign12750_body11_e15758_d_n0;
            locals.var_t5__blk336_dn2 = assign12750_body11_e15758_d_n2;
            locals.var_t5__blk336_dn6 = assign12750_body11_e15758_d_n6;
            locals.var_t5__blk336_dn7 = assign12750_body11_e15758_d_n7;
            locals.var_t5__blk336_dn10 = assign12750_body11_e15758_d_n10;
            locals.var_t5__blk336_dn11 = assign12750_body11_e15758_d_n11;
            locals.var_t5__blk336_dn12 = assign12750_body11_e15758_d_n12;
            locals.var_t5__blk336_dn17 = assign12750_body11_e15758_d_n17;
            let (assign12750_body12_e15783, assign12750_body12_e15783_d_n0, assign12750_body12_e15783_d_n2, assign12750_body12_e15783_d_n6, assign12750_body12_e15783_d_n7, assign12750_body12_e15783_d_n10, assign12750_body12_e15783_d_n11, assign12750_body12_e15783_d_n12, assign12750_body12_e15783_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 == 0.0)) {
        let assign12750_body12_e15774: f64 = (locals.var_t4__blk335 * locals.var_t4__blk335);
        let assign12750_body12_e15777: f64 = (4.0 * locals.var_q_fd_dlt1);
        let assign12750_body12_e15779: f64 = (assign12750_body12_e15777 * locals.var_q_fd_dlt1);
        let assign12750_body12_e15780: f64 = (assign12750_body12_e15774 + assign12750_body12_e15779);
        let assign12750_body12_e15781: f64 = (assign12750_body12_e15780).sqrt();
        (assign12750_body12_e15781, ((((locals.var_t4__blk335_dn0 * locals.var_t4__blk335) + (locals.var_t4__blk335 * locals.var_t4__blk335_dn0)) + (((4.0 * locals.var_q_fd_dlt1_dn0) * locals.var_q_fd_dlt1) + (assign12750_body12_e15777 * locals.var_q_fd_dlt1_dn0))) / (2.0 * assign12750_body12_e15781)), ((((locals.var_t4__blk335_dn2 * locals.var_t4__blk335) + (locals.var_t4__blk335 * locals.var_t4__blk335_dn2)) + (((4.0 * locals.var_q_fd_dlt1_dn2) * locals.var_q_fd_dlt1) + (assign12750_body12_e15777 * locals.var_q_fd_dlt1_dn2))) / (2.0 * assign12750_body12_e15781)), ((((locals.var_t4__blk335_dn6 * locals.var_t4__blk335) + (locals.var_t4__blk335 * locals.var_t4__blk335_dn6)) + (((4.0 * locals.var_q_fd_dlt1_dn6) * locals.var_q_fd_dlt1) + (assign12750_body12_e15777 * locals.var_q_fd_dlt1_dn6))) / (2.0 * assign12750_body12_e15781)), ((((locals.var_t4__blk335_dn7 * locals.var_t4__blk335) + (locals.var_t4__blk335 * locals.var_t4__blk335_dn7)) + (((4.0 * locals.var_q_fd_dlt1_dn7) * locals.var_q_fd_dlt1) + (assign12750_body12_e15777 * locals.var_q_fd_dlt1_dn7))) / (2.0 * assign12750_body12_e15781)), ((((locals.var_t4__blk335_dn10 * locals.var_t4__blk335) + (locals.var_t4__blk335 * locals.var_t4__blk335_dn10)) + (((4.0 * locals.var_q_fd_dlt1_dn10) * locals.var_q_fd_dlt1) + (assign12750_body12_e15777 * locals.var_q_fd_dlt1_dn10))) / (2.0 * assign12750_body12_e15781)), ((((locals.var_t4__blk335_dn11 * locals.var_t4__blk335) + (locals.var_t4__blk335 * locals.var_t4__blk335_dn11)) + (((4.0 * locals.var_q_fd_dlt1_dn11) * locals.var_q_fd_dlt1) + (assign12750_body12_e15777 * locals.var_q_fd_dlt1_dn11))) / (2.0 * assign12750_body12_e15781)), ((((locals.var_t4__blk335_dn12 * locals.var_t4__blk335) + (locals.var_t4__blk335 * locals.var_t4__blk335_dn12)) + (((4.0 * locals.var_q_fd_dlt1_dn12) * locals.var_q_fd_dlt1) + (assign12750_body12_e15777 * locals.var_q_fd_dlt1_dn12))) / (2.0 * assign12750_body12_e15781)), ((((locals.var_t4__blk335_dn17 * locals.var_t4__blk335) + (locals.var_t4__blk335 * locals.var_t4__blk335_dn17)) + (((4.0 * locals.var_q_fd_dlt1_dn17) * locals.var_q_fd_dlt1) + (assign12750_body12_e15777 * locals.var_q_fd_dlt1_dn17))) / (2.0 * assign12750_body12_e15781)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign12750_body12_e15783;
            locals.var_tmf2_dn0 = assign12750_body12_e15783_d_n0;
            locals.var_tmf2_dn2 = assign12750_body12_e15783_d_n2;
            locals.var_tmf2_dn6 = assign12750_body12_e15783_d_n6;
            locals.var_tmf2_dn7 = assign12750_body12_e15783_d_n7;
            locals.var_tmf2_dn10 = assign12750_body12_e15783_d_n10;
            locals.var_tmf2_dn11 = assign12750_body12_e15783_d_n11;
            locals.var_tmf2_dn12 = assign12750_body12_e15783_d_n12;
            locals.var_tmf2_dn17 = assign12750_body12_e15783_d_n17;
            let (assign12750_body13_e15805, assign12750_body13_e15805_d_n0, assign12750_body13_e15805_d_n2, assign12750_body13_e15805_d_n6, assign12750_body13_e15805_d_n7, assign12750_body13_e15805_d_n10, assign12750_body13_e15805_d_n11, assign12750_body13_e15805_d_n12, assign12750_body13_e15805_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 == 0.0)) {
        let assign12750_body13_e15801: f64 = (locals.var_t4__blk335 / locals.var_tmf2);
        let assign12750_body13_e15802: f64 = (1.0 + assign12750_body13_e15801);
        let assign12750_body13_e15803: f64 = (0.5 * assign12750_body13_e15802);
        (assign12750_body13_e15803, (0.5 * (((locals.var_t4__blk335_dn0 * locals.var_tmf2) - (locals.var_t4__blk335 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk335_dn2 * locals.var_tmf2) - (locals.var_t4__blk335 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk335_dn6 * locals.var_tmf2) - (locals.var_t4__blk335 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk335_dn7 * locals.var_tmf2) - (locals.var_t4__blk335 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk335_dn10 * locals.var_tmf2) - (locals.var_t4__blk335 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk335_dn11 * locals.var_tmf2) - (locals.var_t4__blk335 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk335_dn12 * locals.var_tmf2) - (locals.var_t4__blk335 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t4__blk335_dn17 * locals.var_tmf2) - (locals.var_t4__blk335 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t7__blk338, locals.var_t7__blk338_dn0, locals.var_t7__blk338_dn2, locals.var_t7__blk338_dn6, locals.var_t7__blk338_dn7, locals.var_t7__blk338_dn10, locals.var_t7__blk338_dn11, locals.var_t7__blk338_dn12, locals.var_t7__blk338_dn17,)
    }
};
            locals.var_t7__blk338 = assign12750_body13_e15805;
            locals.var_t7__blk338_dn0 = assign12750_body13_e15805_d_n0;
            locals.var_t7__blk338_dn2 = assign12750_body13_e15805_d_n2;
            locals.var_t7__blk338_dn6 = assign12750_body13_e15805_d_n6;
            locals.var_t7__blk338_dn7 = assign12750_body13_e15805_d_n7;
            locals.var_t7__blk338_dn10 = assign12750_body13_e15805_d_n10;
            locals.var_t7__blk338_dn11 = assign12750_body13_e15805_d_n11;
            locals.var_t7__blk338_dn12 = assign12750_body13_e15805_d_n12;
            locals.var_t7__blk338_dn17 = assign12750_body13_e15805_d_n17;
            let (assign12750_body14_e15829, assign12750_body14_e15829_d_n0, assign12750_body14_e15829_d_n2, assign12750_body14_e15829_d_n6, assign12750_body14_e15829_d_n7, assign12750_body14_e15829_d_n10, assign12750_body14_e15829_d_n11, assign12750_body14_e15829_d_n12, assign12750_body14_e15829_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 == 0.0)) {
        let assign12750_body14_e15822: f64 = (locals.var_t4__blk335 + locals.var_tmf2);
        let assign12750_body14_e15823: f64 = (0.5 * assign12750_body14_e15822);
        let assign12750_body14_e15826: f64 = (1e-10 * locals.var_q_fd_dlt1);
        let assign12750_body14_e15827: f64 = (assign12750_body14_e15823 + assign12750_body14_e15826);
        (assign12750_body14_e15827, ((0.5 * (locals.var_t4__blk335_dn0 + locals.var_tmf2_dn0)) + (1e-10 * locals.var_q_fd_dlt1_dn0)), ((0.5 * (locals.var_t4__blk335_dn2 + locals.var_tmf2_dn2)) + (1e-10 * locals.var_q_fd_dlt1_dn2)), ((0.5 * (locals.var_t4__blk335_dn6 + locals.var_tmf2_dn6)) + (1e-10 * locals.var_q_fd_dlt1_dn6)), ((0.5 * (locals.var_t4__blk335_dn7 + locals.var_tmf2_dn7)) + (1e-10 * locals.var_q_fd_dlt1_dn7)), ((0.5 * (locals.var_t4__blk335_dn10 + locals.var_tmf2_dn10)) + (1e-10 * locals.var_q_fd_dlt1_dn10)), ((0.5 * (locals.var_t4__blk335_dn11 + locals.var_tmf2_dn11)) + (1e-10 * locals.var_q_fd_dlt1_dn11)), ((0.5 * (locals.var_t4__blk335_dn12 + locals.var_tmf2_dn12)) + (1e-10 * locals.var_q_fd_dlt1_dn12)), ((0.5 * (locals.var_t4__blk335_dn17 + locals.var_tmf2_dn17)) + (1e-10 * locals.var_q_fd_dlt1_dn17)),)
    } else {
        (locals.var_t6__blk337, locals.var_t6__blk337_dn0, locals.var_t6__blk337_dn2, locals.var_t6__blk337_dn6, locals.var_t6__blk337_dn7, locals.var_t6__blk337_dn10, locals.var_t6__blk337_dn11, locals.var_t6__blk337_dn12, locals.var_t6__blk337_dn17,)
    }
};
            locals.var_t6__blk337 = assign12750_body14_e15829;
            locals.var_t6__blk337_dn0 = assign12750_body14_e15829_d_n0;
            locals.var_t6__blk337_dn2 = assign12750_body14_e15829_d_n2;
            locals.var_t6__blk337_dn6 = assign12750_body14_e15829_d_n6;
            locals.var_t6__blk337_dn7 = assign12750_body14_e15829_d_n7;
            locals.var_t6__blk337_dn10 = assign12750_body14_e15829_d_n10;
            locals.var_t6__blk337_dn11 = assign12750_body14_e15829_d_n11;
            locals.var_t6__blk337_dn12 = assign12750_body14_e15829_d_n12;
            locals.var_t6__blk337_dn17 = assign12750_body14_e15829_d_n17;
            let assign12750_body15_e15832: f64 = if locals.var_t6__blk337 < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard347 = assign12750_body15_e15832;
            let (assign12750_body16_e15850, assign12750_body16_e15850_d_n0, assign12750_body16_e15850_d_n2, assign12750_body16_e15850_d_n6, assign12750_body16_e15850_d_n7, assign12750_body16_e15850_d_n10, assign12750_body16_e15850_d_n11, assign12750_body16_e15850_d_n12, assign12750_body16_e15850_d_n17,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 == 0.0)) && (locals.var_guard347 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk337, locals.var_t6__blk337_dn0, locals.var_t6__blk337_dn2, locals.var_t6__blk337_dn6, locals.var_t6__blk337_dn7, locals.var_t6__blk337_dn10, locals.var_t6__blk337_dn11, locals.var_t6__blk337_dn12, locals.var_t6__blk337_dn17,)
    }
};
            locals.var_t6__blk337 = assign12750_body16_e15850;
            locals.var_t6__blk337_dn0 = assign12750_body16_e15850_d_n0;
            locals.var_t6__blk337_dn2 = assign12750_body16_e15850_d_n2;
            locals.var_t6__blk337_dn6 = assign12750_body16_e15850_d_n6;
            locals.var_t6__blk337_dn7 = assign12750_body16_e15850_d_n7;
            locals.var_t6__blk337_dn10 = assign12750_body16_e15850_d_n10;
            locals.var_t6__blk337_dn11 = assign12750_body16_e15850_d_n11;
            locals.var_t6__blk337_dn12 = assign12750_body16_e15850_d_n12;
            locals.var_t6__blk337_dn17 = assign12750_body16_e15850_d_n17;
            let (assign12750_body17_e15868, assign12750_body17_e15868_d_n0, assign12750_body17_e15868_d_n2, assign12750_body17_e15868_d_n6, assign12750_body17_e15868_d_n7, assign12750_body17_e15868_d_n10, assign12750_body17_e15868_d_n11, assign12750_body17_e15868_d_n12, assign12750_body17_e15868_d_n17,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 == 0.0)) && (locals.var_guard347 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7__blk338, locals.var_t7__blk338_dn0, locals.var_t7__blk338_dn2, locals.var_t7__blk338_dn6, locals.var_t7__blk338_dn7, locals.var_t7__blk338_dn10, locals.var_t7__blk338_dn11, locals.var_t7__blk338_dn12, locals.var_t7__blk338_dn17,)
    }
};
            locals.var_t7__blk338 = assign12750_body17_e15868;
            locals.var_t7__blk338_dn0 = assign12750_body17_e15868_d_n0;
            locals.var_t7__blk338_dn2 = assign12750_body17_e15868_d_n2;
            locals.var_t7__blk338_dn6 = assign12750_body17_e15868_d_n6;
            locals.var_t7__blk338_dn7 = assign12750_body17_e15868_d_n7;
            locals.var_t7__blk338_dn10 = assign12750_body17_e15868_d_n10;
            locals.var_t7__blk338_dn11 = assign12750_body17_e15868_d_n11;
            locals.var_t7__blk338_dn12 = assign12750_body17_e15868_d_n12;
            locals.var_t7__blk338_dn17 = assign12750_body17_e15868_d_n17;
            let (assign12750_body18_e15889, assign12750_body18_e15889_d_n0, assign12750_body18_e15889_d_n2, assign12750_body18_e15889_d_n6, assign12750_body18_e15889_d_n7, assign12750_body18_e15889_d_n10, assign12750_body18_e15889_d_n11, assign12750_body18_e15889_d_n12, assign12750_body18_e15889_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 == 0.0)) {
        let assign12750_body18_e15883: f64 = (-locals.var_q_fd_soi);
        let assign12750_body18_e15885: f64 = (assign12750_body18_e15883 - locals.var_t6__blk337);
        let assign12750_body18_e15887: f64 = (assign12750_body18_e15885 - locals.var_q_fd_dlt2);
        (assign12750_body18_e15887, (((-locals.var_q_fd_soi_dn0) - locals.var_t6__blk337_dn0) - locals.var_q_fd_dlt2_dn0), (((-locals.var_q_fd_soi_dn2) - locals.var_t6__blk337_dn2) - locals.var_q_fd_dlt2_dn2), (((-locals.var_q_fd_soi_dn6) - locals.var_t6__blk337_dn6) - locals.var_q_fd_dlt2_dn6), (((-locals.var_q_fd_soi_dn7) - locals.var_t6__blk337_dn7) - locals.var_q_fd_dlt2_dn7), (((-locals.var_q_fd_soi_dn10) - locals.var_t6__blk337_dn10) - locals.var_q_fd_dlt2_dn10), (((-locals.var_q_fd_soi_dn11) - locals.var_t6__blk337_dn11) - locals.var_q_fd_dlt2_dn11), (((-locals.var_q_fd_soi_dn12) - locals.var_t6__blk337_dn12) - locals.var_q_fd_dlt2_dn12), (((-locals.var_q_fd_soi_dn17) - locals.var_t6__blk337_dn17) - locals.var_q_fd_dlt2_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
            locals.var_tmf1 = assign12750_body18_e15889;
            locals.var_tmf1_dn0 = assign12750_body18_e15889_d_n0;
            locals.var_tmf1_dn2 = assign12750_body18_e15889_d_n2;
            locals.var_tmf1_dn6 = assign12750_body18_e15889_d_n6;
            locals.var_tmf1_dn7 = assign12750_body18_e15889_d_n7;
            locals.var_tmf1_dn10 = assign12750_body18_e15889_d_n10;
            locals.var_tmf1_dn11 = assign12750_body18_e15889_d_n11;
            locals.var_tmf1_dn12 = assign12750_body18_e15889_d_n12;
            locals.var_tmf1_dn17 = assign12750_body18_e15889_d_n17;
            let (assign12750_body19_e15910, assign12750_body19_e15910_d_n0, assign12750_body19_e15910_d_n2, assign12750_body19_e15910_d_n6, assign12750_body19_e15910_d_n7, assign12750_body19_e15910_d_n10, assign12750_body19_e15910_d_n11, assign12750_body19_e15910_d_n12, assign12750_body19_e15910_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 == 0.0)) {
        let assign12750_body19_e15905: f64 = (-locals.var_q_fd_soi);
        let assign12750_body19_e15906: f64 = (4.0 * assign12750_body19_e15905);
        let assign12750_body19_e15908: f64 = (assign12750_body19_e15906 * locals.var_q_fd_dlt2);
        (assign12750_body19_e15908, (((4.0 * (-locals.var_q_fd_soi_dn0)) * locals.var_q_fd_dlt2) + (assign12750_body19_e15906 * locals.var_q_fd_dlt2_dn0)), (((4.0 * (-locals.var_q_fd_soi_dn2)) * locals.var_q_fd_dlt2) + (assign12750_body19_e15906 * locals.var_q_fd_dlt2_dn2)), (((4.0 * (-locals.var_q_fd_soi_dn6)) * locals.var_q_fd_dlt2) + (assign12750_body19_e15906 * locals.var_q_fd_dlt2_dn6)), (((4.0 * (-locals.var_q_fd_soi_dn7)) * locals.var_q_fd_dlt2) + (assign12750_body19_e15906 * locals.var_q_fd_dlt2_dn7)), (((4.0 * (-locals.var_q_fd_soi_dn10)) * locals.var_q_fd_dlt2) + (assign12750_body19_e15906 * locals.var_q_fd_dlt2_dn10)), (((4.0 * (-locals.var_q_fd_soi_dn11)) * locals.var_q_fd_dlt2) + (assign12750_body19_e15906 * locals.var_q_fd_dlt2_dn11)), (((4.0 * (-locals.var_q_fd_soi_dn12)) * locals.var_q_fd_dlt2) + (assign12750_body19_e15906 * locals.var_q_fd_dlt2_dn12)), (((4.0 * (-locals.var_q_fd_soi_dn17)) * locals.var_q_fd_dlt2) + (assign12750_body19_e15906 * locals.var_q_fd_dlt2_dn17)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign12750_body19_e15910;
            locals.var_tmf2_dn0 = assign12750_body19_e15910_d_n0;
            locals.var_tmf2_dn2 = assign12750_body19_e15910_d_n2;
            locals.var_tmf2_dn6 = assign12750_body19_e15910_d_n6;
            locals.var_tmf2_dn7 = assign12750_body19_e15910_d_n7;
            locals.var_tmf2_dn10 = assign12750_body19_e15910_d_n10;
            locals.var_tmf2_dn11 = assign12750_body19_e15910_d_n11;
            locals.var_tmf2_dn12 = assign12750_body19_e15910_d_n12;
            locals.var_tmf2_dn17 = assign12750_body19_e15910_d_n17;
            let (assign12750_body20_e15932, assign12750_body20_e15932_d_n0, assign12750_body20_e15932_d_n2, assign12750_body20_e15932_d_n6, assign12750_body20_e15932_d_n7, assign12750_body20_e15932_d_n10, assign12750_body20_e15932_d_n11, assign12750_body20_e15932_d_n12, assign12750_body20_e15932_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 == 0.0)) {
        let (assign12750_body20_e15930, assign12750_body20_e15930_d_n0, assign12750_body20_e15930_d_n2, assign12750_body20_e15930_d_n6, assign12750_body20_e15930_d_n7, assign12750_body20_e15930_d_n10, assign12750_body20_e15930_d_n11, assign12750_body20_e15930_d_n12, assign12750_body20_e15930_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign12750_body20_e15929: f64 = (-locals.var_tmf2);
                (assign12750_body20_e15929, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign12750_body20_e15930, assign12750_body20_e15930_d_n0, assign12750_body20_e15930_d_n2, assign12750_body20_e15930_d_n6, assign12750_body20_e15930_d_n7, assign12750_body20_e15930_d_n10, assign12750_body20_e15930_d_n11, assign12750_body20_e15930_d_n12, assign12750_body20_e15930_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign12750_body20_e15932;
            locals.var_tmf2_dn0 = assign12750_body20_e15932_d_n0;
            locals.var_tmf2_dn2 = assign12750_body20_e15932_d_n2;
            locals.var_tmf2_dn6 = assign12750_body20_e15932_d_n6;
            locals.var_tmf2_dn7 = assign12750_body20_e15932_d_n7;
            locals.var_tmf2_dn10 = assign12750_body20_e15932_d_n10;
            locals.var_tmf2_dn11 = assign12750_body20_e15932_d_n11;
            locals.var_tmf2_dn12 = assign12750_body20_e15932_d_n12;
            locals.var_tmf2_dn17 = assign12750_body20_e15932_d_n17;
            let (assign12750_body21_e15953, assign12750_body21_e15953_d_n0, assign12750_body21_e15953_d_n2, assign12750_body21_e15953_d_n6, assign12750_body21_e15953_d_n7, assign12750_body21_e15953_d_n10, assign12750_body21_e15953_d_n11, assign12750_body21_e15953_d_n12, assign12750_body21_e15953_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 == 0.0)) {
        let assign12750_body21_e15948: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign12750_body21_e15950: f64 = (assign12750_body21_e15948 + locals.var_tmf2);
        let assign12750_body21_e15951: f64 = (assign12750_body21_e15950).sqrt();
        (assign12750_body21_e15951, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign12750_body21_e15951)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign12750_body21_e15951)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign12750_body21_e15951)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign12750_body21_e15951)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign12750_body21_e15951)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign12750_body21_e15951)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign12750_body21_e15951)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign12750_body21_e15951)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
            locals.var_tmf2 = assign12750_body21_e15953;
            locals.var_tmf2_dn0 = assign12750_body21_e15953_d_n0;
            locals.var_tmf2_dn2 = assign12750_body21_e15953_d_n2;
            locals.var_tmf2_dn6 = assign12750_body21_e15953_d_n6;
            locals.var_tmf2_dn7 = assign12750_body21_e15953_d_n7;
            locals.var_tmf2_dn10 = assign12750_body21_e15953_d_n10;
            locals.var_tmf2_dn11 = assign12750_body21_e15953_d_n11;
            locals.var_tmf2_dn12 = assign12750_body21_e15953_d_n12;
            locals.var_tmf2_dn17 = assign12750_body21_e15953_d_n17;
            let (assign12750_body22_e15975, assign12750_body22_e15975_d_n0, assign12750_body22_e15975_d_n2, assign12750_body22_e15975_d_n6, assign12750_body22_e15975_d_n7, assign12750_body22_e15975_d_n10, assign12750_body22_e15975_d_n11, assign12750_body22_e15975_d_n12, assign12750_body22_e15975_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 == 0.0)) {
        let assign12750_body22_e15971: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign12750_body22_e15972: f64 = (1.0 + assign12750_body22_e15971);
        let assign12750_body22_e15973: f64 = (0.5 * assign12750_body22_e15972);
        (assign12750_body22_e15973, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn17,)
    }
};
            locals.var_t8 = assign12750_body22_e15975;
            locals.var_t8_dn0 = assign12750_body22_e15975_d_n0;
            locals.var_t8_dn2 = assign12750_body22_e15975_d_n2;
            locals.var_t8_dn6 = assign12750_body22_e15975_d_n6;
            locals.var_t8_dn7 = assign12750_body22_e15975_d_n7;
            locals.var_t8_dn10 = assign12750_body22_e15975_d_n10;
            locals.var_t8_dn11 = assign12750_body22_e15975_d_n11;
            locals.var_t8_dn12 = assign12750_body22_e15975_d_n12;
            locals.var_t8_dn17 = assign12750_body22_e15975_d_n17;
            let (assign12750_body23_e15998, assign12750_body23_e15998_d_n0, assign12750_body23_e15998_d_n2, assign12750_body23_e15998_d_n6, assign12750_body23_e15998_d_n7, assign12750_body23_e15998_d_n10, assign12750_body23_e15998_d_n11, assign12750_body23_e15998_d_n12, assign12750_body23_e15998_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 == 0.0)) {
        let assign12750_body23_e15990: f64 = (-locals.var_q_fd_soi);
        let assign12750_body23_e15994: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign12750_body23_e15995: f64 = (0.5 * assign12750_body23_e15994);
        let assign12750_body23_e15996: f64 = (assign12750_body23_e15990 - assign12750_body23_e15995);
        (assign12750_body23_e15996, ((-locals.var_q_fd_soi_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((-locals.var_q_fd_soi_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((-locals.var_q_fd_soi_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((-locals.var_q_fd_soi_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((-locals.var_q_fd_soi_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((-locals.var_q_fd_soi_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((-locals.var_q_fd_soi_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((-locals.var_q_fd_soi_dn17) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_t6__blk337, locals.var_t6__blk337_dn0, locals.var_t6__blk337_dn2, locals.var_t6__blk337_dn6, locals.var_t6__blk337_dn7, locals.var_t6__blk337_dn10, locals.var_t6__blk337_dn11, locals.var_t6__blk337_dn12, locals.var_t6__blk337_dn17,)
    }
};
            locals.var_t6__blk337 = assign12750_body23_e15998;
            locals.var_t6__blk337_dn0 = assign12750_body23_e15998_d_n0;
            locals.var_t6__blk337_dn2 = assign12750_body23_e15998_d_n2;
            locals.var_t6__blk337_dn6 = assign12750_body23_e15998_d_n6;
            locals.var_t6__blk337_dn7 = assign12750_body23_e15998_d_n7;
            locals.var_t6__blk337_dn10 = assign12750_body23_e15998_d_n10;
            locals.var_t6__blk337_dn11 = assign12750_body23_e15998_d_n11;
            locals.var_t6__blk337_dn12 = assign12750_body23_e15998_d_n12;
            locals.var_t6__blk337_dn17 = assign12750_body23_e15998_d_n17;
            let (assign12750_body24_e16018, assign12750_body24_e16018_d_n0, assign12750_body24_e16018_d_n2, assign12750_body24_e16018_d_n6, assign12750_body24_e16018_d_n7, assign12750_body24_e16018_d_n10, assign12750_body24_e16018_d_n11, assign12750_body24_e16018_d_n12, assign12750_body24_e16018_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 == 0.0)) {
        let assign12750_body24_e16015: f64 = (locals.var_t5__blk336 * locals.var_t8);
        let assign12750_body24_e16016: f64 = (locals.var_t7__blk338 * assign12750_body24_e16015);
        (assign12750_body24_e16016, ((locals.var_t7__blk338_dn0 * assign12750_body24_e16015) + (locals.var_t7__blk338 * ((locals.var_t5__blk336_dn0 * locals.var_t8) + (locals.var_t5__blk336 * locals.var_t8_dn0)))), ((locals.var_t7__blk338_dn2 * assign12750_body24_e16015) + (locals.var_t7__blk338 * ((locals.var_t5__blk336_dn2 * locals.var_t8) + (locals.var_t5__blk336 * locals.var_t8_dn2)))), ((locals.var_t7__blk338_dn6 * assign12750_body24_e16015) + (locals.var_t7__blk338 * ((locals.var_t5__blk336_dn6 * locals.var_t8) + (locals.var_t5__blk336 * locals.var_t8_dn6)))), ((locals.var_t7__blk338_dn7 * assign12750_body24_e16015) + (locals.var_t7__blk338 * ((locals.var_t5__blk336_dn7 * locals.var_t8) + (locals.var_t5__blk336 * locals.var_t8_dn7)))), ((locals.var_t7__blk338_dn10 * assign12750_body24_e16015) + (locals.var_t7__blk338 * ((locals.var_t5__blk336_dn10 * locals.var_t8) + (locals.var_t5__blk336 * locals.var_t8_dn10)))), ((locals.var_t7__blk338_dn11 * assign12750_body24_e16015) + (locals.var_t7__blk338 * ((locals.var_t5__blk336_dn11 * locals.var_t8) + (locals.var_t5__blk336 * locals.var_t8_dn11)))), ((locals.var_t7__blk338_dn12 * assign12750_body24_e16015) + (locals.var_t7__blk338 * ((locals.var_t5__blk336_dn12 * locals.var_t8) + (locals.var_t5__blk336 * locals.var_t8_dn12)))), ((locals.var_t7__blk338_dn17 * assign12750_body24_e16015) + (locals.var_t7__blk338 * ((locals.var_t5__blk336_dn17 * locals.var_t8) + (locals.var_t5__blk336 * locals.var_t8_dn17)))),)
    } else {
        (locals.var_t7__blk338, locals.var_t7__blk338_dn0, locals.var_t7__blk338_dn2, locals.var_t7__blk338_dn6, locals.var_t7__blk338_dn7, locals.var_t7__blk338_dn10, locals.var_t7__blk338_dn11, locals.var_t7__blk338_dn12, locals.var_t7__blk338_dn17,)
    }
};
            locals.var_t7__blk338 = assign12750_body24_e16018;
            locals.var_t7__blk338_dn0 = assign12750_body24_e16018_d_n0;
            locals.var_t7__blk338_dn2 = assign12750_body24_e16018_d_n2;
            locals.var_t7__blk338_dn6 = assign12750_body24_e16018_d_n6;
            locals.var_t7__blk338_dn7 = assign12750_body24_e16018_d_n7;
            locals.var_t7__blk338_dn10 = assign12750_body24_e16018_d_n10;
            locals.var_t7__blk338_dn11 = assign12750_body24_e16018_d_n11;
            locals.var_t7__blk338_dn12 = assign12750_body24_e16018_d_n12;
            locals.var_t7__blk338_dn17 = assign12750_body24_e16018_d_n17;
            let (assign12750_body25_e16044, assign12750_body25_e16044_d_n0, assign12750_body25_e16044_d_n2, assign12750_body25_e16044_d_n6, assign12750_body25_e16044_d_n7, assign12750_body25_e16044_d_n10, assign12750_body25_e16044_d_n11, assign12750_body25_e16044_d_n12, assign12750_body25_e16044_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 == 0.0)) {
        let assign12750_body25_e16034: f64 = (locals.var_t6__blk337 * locals.var_t6__blk337);
        let assign12750_body25_e16036: f64 = (assign12750_body25_e16034 / 2.0);
        let assign12750_body25_e16038: f64 = (assign12750_body25_e16036 / 1.034943e-10);
        let assign12750_body25_e16040: f64 = (assign12750_body25_e16038 / 1.6021918e-19);
        let assign12750_body25_e16042: f64 = (assign12750_body25_e16040 / locals.var_uc_nsubs);
        (assign12750_body25_e16042, ((((((((locals.var_t6__blk337_dn0 * locals.var_t6__blk337) + (locals.var_t6__blk337 * locals.var_t6__blk337_dn0)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12750_body25_e16040 * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk337_dn2 * locals.var_t6__blk337) + (locals.var_t6__blk337 * locals.var_t6__blk337_dn2)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12750_body25_e16040 * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk337_dn6 * locals.var_t6__blk337) + (locals.var_t6__blk337 * locals.var_t6__blk337_dn6)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12750_body25_e16040 * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk337_dn7 * locals.var_t6__blk337) + (locals.var_t6__blk337 * locals.var_t6__blk337_dn7)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12750_body25_e16040 * locals.var_uc_nsubs_dn7)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk337_dn10 * locals.var_t6__blk337) + (locals.var_t6__blk337 * locals.var_t6__blk337_dn10)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12750_body25_e16040 * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk337_dn11 * locals.var_t6__blk337) + (locals.var_t6__blk337 * locals.var_t6__blk337_dn11)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12750_body25_e16040 * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk337_dn12 * locals.var_t6__blk337) + (locals.var_t6__blk337 * locals.var_t6__blk337_dn12)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12750_body25_e16040 * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), ((((((((locals.var_t6__blk337_dn17 * locals.var_t6__blk337) + (locals.var_t6__blk337 * locals.var_t6__blk337_dn17)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign12750_body25_e16040 * locals.var_uc_nsubs_dn17)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)),)
    } else {
        (locals.var_phi_b_dep, locals.var_phi_b_dep_dn0, locals.var_phi_b_dep_dn2, locals.var_phi_b_dep_dn6, locals.var_phi_b_dep_dn7, locals.var_phi_b_dep_dn10, locals.var_phi_b_dep_dn11, locals.var_phi_b_dep_dn12, locals.var_phi_b_dep_dn17,)
    }
};
            locals.var_phi_b_dep = assign12750_body25_e16044;
            locals.var_phi_b_dep_dn0 = assign12750_body25_e16044_d_n0;
            locals.var_phi_b_dep_dn2 = assign12750_body25_e16044_d_n2;
            locals.var_phi_b_dep_dn6 = assign12750_body25_e16044_d_n6;
            locals.var_phi_b_dep_dn7 = assign12750_body25_e16044_d_n7;
            locals.var_phi_b_dep_dn10 = assign12750_body25_e16044_d_n10;
            locals.var_phi_b_dep_dn11 = assign12750_body25_e16044_d_n11;
            locals.var_phi_b_dep_dn12 = assign12750_body25_e16044_d_n12;
            locals.var_phi_b_dep_dn17 = assign12750_body25_e16044_d_n17;
            let (assign12750_body26_e16066, assign12750_body26_e16066_d_n0, assign12750_body26_e16066_d_n2, assign12750_body26_e16066_d_n6, assign12750_body26_e16066_d_n7, assign12750_body26_e16066_d_n10, assign12750_body26_e16066_d_n11, assign12750_body26_e16066_d_n12, assign12750_body26_e16066_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 == 0.0)) {
        let assign12750_body26_e16060: f64 = (2.0 * locals.var_phi_b_dep);
        let assign12750_body26_e16062: f64 = (assign12750_body26_e16060 * locals.var_t7__blk338);
        let assign12750_body26_e16064: f64 = (assign12750_body26_e16062 / locals.var_t6__blk337);
        (assign12750_body26_e16064, ((((((2.0 * locals.var_phi_b_dep_dn0) * locals.var_t7__blk338) + (assign12750_body26_e16060 * locals.var_t7__blk338_dn0)) * locals.var_t6__blk337) - (assign12750_body26_e16062 * locals.var_t6__blk337_dn0)) / (locals.var_t6__blk337 * locals.var_t6__blk337)), ((((((2.0 * locals.var_phi_b_dep_dn2) * locals.var_t7__blk338) + (assign12750_body26_e16060 * locals.var_t7__blk338_dn2)) * locals.var_t6__blk337) - (assign12750_body26_e16062 * locals.var_t6__blk337_dn2)) / (locals.var_t6__blk337 * locals.var_t6__blk337)), ((((((2.0 * locals.var_phi_b_dep_dn6) * locals.var_t7__blk338) + (assign12750_body26_e16060 * locals.var_t7__blk338_dn6)) * locals.var_t6__blk337) - (assign12750_body26_e16062 * locals.var_t6__blk337_dn6)) / (locals.var_t6__blk337 * locals.var_t6__blk337)), ((((((2.0 * locals.var_phi_b_dep_dn7) * locals.var_t7__blk338) + (assign12750_body26_e16060 * locals.var_t7__blk338_dn7)) * locals.var_t6__blk337) - (assign12750_body26_e16062 * locals.var_t6__blk337_dn7)) / (locals.var_t6__blk337 * locals.var_t6__blk337)), ((((((2.0 * locals.var_phi_b_dep_dn10) * locals.var_t7__blk338) + (assign12750_body26_e16060 * locals.var_t7__blk338_dn10)) * locals.var_t6__blk337) - (assign12750_body26_e16062 * locals.var_t6__blk337_dn10)) / (locals.var_t6__blk337 * locals.var_t6__blk337)), ((((((2.0 * locals.var_phi_b_dep_dn11) * locals.var_t7__blk338) + (assign12750_body26_e16060 * locals.var_t7__blk338_dn11)) * locals.var_t6__blk337) - (assign12750_body26_e16062 * locals.var_t6__blk337_dn11)) / (locals.var_t6__blk337 * locals.var_t6__blk337)), ((((((2.0 * locals.var_phi_b_dep_dn12) * locals.var_t7__blk338) + (assign12750_body26_e16060 * locals.var_t7__blk338_dn12)) * locals.var_t6__blk337) - (assign12750_body26_e16062 * locals.var_t6__blk337_dn12)) / (locals.var_t6__blk337 * locals.var_t6__blk337)), ((((((2.0 * locals.var_phi_b_dep_dn17) * locals.var_t7__blk338) + (assign12750_body26_e16060 * locals.var_t7__blk338_dn17)) * locals.var_t6__blk337) - (assign12750_body26_e16062 * locals.var_t6__blk337_dn17)) / (locals.var_t6__blk337 * locals.var_t6__blk337)),)
    } else {
        (locals.var_phi_b_dep_dpsb, locals.var_phi_b_dep_dpsb_dn0, locals.var_phi_b_dep_dpsb_dn2, locals.var_phi_b_dep_dpsb_dn6, locals.var_phi_b_dep_dpsb_dn7, locals.var_phi_b_dep_dpsb_dn10, locals.var_phi_b_dep_dpsb_dn11, locals.var_phi_b_dep_dpsb_dn12, locals.var_phi_b_dep_dpsb_dn17,)
    }
};
            locals.var_phi_b_dep_dpsb = assign12750_body26_e16066;
            locals.var_phi_b_dep_dpsb_dn0 = assign12750_body26_e16066_d_n0;
            locals.var_phi_b_dep_dpsb_dn2 = assign12750_body26_e16066_d_n2;
            locals.var_phi_b_dep_dpsb_dn6 = assign12750_body26_e16066_d_n6;
            locals.var_phi_b_dep_dpsb_dn7 = assign12750_body26_e16066_d_n7;
            locals.var_phi_b_dep_dpsb_dn10 = assign12750_body26_e16066_d_n10;
            locals.var_phi_b_dep_dpsb_dn11 = assign12750_body26_e16066_d_n11;
            locals.var_phi_b_dep_dpsb_dn12 = assign12750_body26_e16066_d_n12;
            locals.var_phi_b_dep_dpsb_dn17 = assign12750_body26_e16066_d_n17;
            let (assign12750_body27_e16119, assign12750_body27_e16119_d_n0, assign12750_body27_e16119_d_n2, assign12750_body27_e16119_d_n6, assign12750_body27_e16119_d_n7, assign12750_body27_e16119_d_n10, assign12750_body27_e16119_d_n11, assign12750_body27_e16119_d_n12, assign12750_body27_e16119_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 == 0.0)) {
        let assign12750_body27_e16083: f64 = (locals.var_phi_sl_soi - locals.var_phi_sl_bulk);
        let assign12750_body27_e16086: f64 = (locals.var_t4__blk335 / locals.var_c_box);
        let assign12750_body27_e16087: f64 = (assign12750_body27_e16083 + assign12750_body27_e16086);
        let assign12750_body27_e16091: f64 = (locals.var_q_fd_soi / 2.0);
        let assign12750_body27_e16092: f64 = (locals.var_t4__blk335 + assign12750_body27_e16091);
        let assign12750_body27_e16094: f64 = (assign12750_body27_e16092 * locals.var_t_soi);
        let assign12750_body27_e16096: f64 = (assign12750_body27_e16094 / 1.034943e-10);
        let assign12750_body27_e16097: f64 = (assign12750_body27_e16087 + assign12750_body27_e16096);
        let assign12750_body27_e16099: f64 = (assign12750_body27_e16097 - locals.var_vbsbiz);
        let assign12750_body27_e16101: f64 = (assign12750_body27_e16099 + locals.var_phi_b_dep);
        let assign12750_body27_e16103: f64 = (-1.0);
        let assign12750_body27_e16106: f64 = (locals.var_t5__blk336 / locals.var_c_box);
        let assign12750_body27_e16107: f64 = (assign12750_body27_e16103 + assign12750_body27_e16106);
        let assign12750_body27_e16110: f64 = (locals.var_t5__blk336 * locals.var_t_soi);
        let assign12750_body27_e16112: f64 = (assign12750_body27_e16110 / 1.034943e-10);
        let assign12750_body27_e16113: f64 = (assign12750_body27_e16107 + assign12750_body27_e16112);
        let assign12750_body27_e16115: f64 = (assign12750_body27_e16113 + locals.var_phi_b_dep_dpsb);
        let assign12750_body27_e16116: f64 = (assign12750_body27_e16101 / assign12750_body27_e16115);
        let assign12750_body27_e16117: f64 = (locals.var_phi_sl_bulk - assign12750_body27_e16116);
        (assign12750_body27_e16117, (locals.var_phi_sl_bulk_dn0 - ((((((((locals.var_phi_sl_soi_dn0 - locals.var_phi_sl_bulk_dn0) + (locals.var_t4__blk335_dn0 / locals.var_c_box)) + (((locals.var_t4__blk335_dn0 + (locals.var_q_fd_soi_dn0 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn0) + locals.var_phi_b_dep_dn0) * assign12750_body27_e16115) - (assign12750_body27_e16101 * (((locals.var_t5__blk336_dn0 / locals.var_c_box) + ((locals.var_t5__blk336_dn0 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn0))) / (assign12750_body27_e16115 * assign12750_body27_e16115))), (locals.var_phi_sl_bulk_dn2 - ((((((((locals.var_phi_sl_soi_dn2 - locals.var_phi_sl_bulk_dn2) + (locals.var_t4__blk335_dn2 / locals.var_c_box)) + (((locals.var_t4__blk335_dn2 + (locals.var_q_fd_soi_dn2 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn2) + locals.var_phi_b_dep_dn2) * assign12750_body27_e16115) - (assign12750_body27_e16101 * (((locals.var_t5__blk336_dn2 / locals.var_c_box) + ((locals.var_t5__blk336_dn2 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn2))) / (assign12750_body27_e16115 * assign12750_body27_e16115))), (locals.var_phi_sl_bulk_dn6 - ((((((((locals.var_phi_sl_soi_dn6 - locals.var_phi_sl_bulk_dn6) + (locals.var_t4__blk335_dn6 / locals.var_c_box)) + (((locals.var_t4__blk335_dn6 + (locals.var_q_fd_soi_dn6 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn6) + locals.var_phi_b_dep_dn6) * assign12750_body27_e16115) - (assign12750_body27_e16101 * (((locals.var_t5__blk336_dn6 / locals.var_c_box) + ((locals.var_t5__blk336_dn6 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn6))) / (assign12750_body27_e16115 * assign12750_body27_e16115))), (locals.var_phi_sl_bulk_dn7 - ((((((((locals.var_phi_sl_soi_dn7 - locals.var_phi_sl_bulk_dn7) + (locals.var_t4__blk335_dn7 / locals.var_c_box)) + (((locals.var_t4__blk335_dn7 + (locals.var_q_fd_soi_dn7 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn7) + locals.var_phi_b_dep_dn7) * assign12750_body27_e16115) - (assign12750_body27_e16101 * (((locals.var_t5__blk336_dn7 / locals.var_c_box) + ((locals.var_t5__blk336_dn7 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn7))) / (assign12750_body27_e16115 * assign12750_body27_e16115))), (locals.var_phi_sl_bulk_dn10 - ((((((((locals.var_phi_sl_soi_dn10 - locals.var_phi_sl_bulk_dn10) + (locals.var_t4__blk335_dn10 / locals.var_c_box)) + (((locals.var_t4__blk335_dn10 + (locals.var_q_fd_soi_dn10 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn10) + locals.var_phi_b_dep_dn10) * assign12750_body27_e16115) - (assign12750_body27_e16101 * (((locals.var_t5__blk336_dn10 / locals.var_c_box) + ((locals.var_t5__blk336_dn10 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn10))) / (assign12750_body27_e16115 * assign12750_body27_e16115))), (locals.var_phi_sl_bulk_dn11 - ((((((((locals.var_phi_sl_soi_dn11 - locals.var_phi_sl_bulk_dn11) + (locals.var_t4__blk335_dn11 / locals.var_c_box)) + (((locals.var_t4__blk335_dn11 + (locals.var_q_fd_soi_dn11 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn11) + locals.var_phi_b_dep_dn11) * assign12750_body27_e16115) - (assign12750_body27_e16101 * (((locals.var_t5__blk336_dn11 / locals.var_c_box) + ((locals.var_t5__blk336_dn11 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn11))) / (assign12750_body27_e16115 * assign12750_body27_e16115))), (locals.var_phi_sl_bulk_dn12 - ((((((((locals.var_phi_sl_soi_dn12 - locals.var_phi_sl_bulk_dn12) + (locals.var_t4__blk335_dn12 / locals.var_c_box)) + (((locals.var_t4__blk335_dn12 + (locals.var_q_fd_soi_dn12 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn12) + locals.var_phi_b_dep_dn12) * assign12750_body27_e16115) - (assign12750_body27_e16101 * (((locals.var_t5__blk336_dn12 / locals.var_c_box) + ((locals.var_t5__blk336_dn12 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn12))) / (assign12750_body27_e16115 * assign12750_body27_e16115))), (locals.var_phi_sl_bulk_dn17 - ((((((((locals.var_phi_sl_soi_dn17 - locals.var_phi_sl_bulk_dn17) + (locals.var_t4__blk335_dn17 / locals.var_c_box)) + (((locals.var_t4__blk335_dn17 + (locals.var_q_fd_soi_dn17 / 2.0)) * locals.var_t_soi) / 1.034943e-10)) - locals.var_vbsbiz_dn17) + locals.var_phi_b_dep_dn17) * assign12750_body27_e16115) - (assign12750_body27_e16101 * (((locals.var_t5__blk336_dn17 / locals.var_c_box) + ((locals.var_t5__blk336_dn17 * locals.var_t_soi) / 1.034943e-10)) + locals.var_phi_b_dep_dpsb_dn17))) / (assign12750_body27_e16115 * assign12750_body27_e16115))),)
    } else {
        (locals.var_t6__blk337, locals.var_t6__blk337_dn0, locals.var_t6__blk337_dn2, locals.var_t6__blk337_dn6, locals.var_t6__blk337_dn7, locals.var_t6__blk337_dn10, locals.var_t6__blk337_dn11, locals.var_t6__blk337_dn12, locals.var_t6__blk337_dn17,)
    }
};
            locals.var_t6__blk337 = assign12750_body27_e16119;
            locals.var_t6__blk337_dn0 = assign12750_body27_e16119_d_n0;
            locals.var_t6__blk337_dn2 = assign12750_body27_e16119_d_n2;
            locals.var_t6__blk337_dn6 = assign12750_body27_e16119_d_n6;
            locals.var_t6__blk337_dn7 = assign12750_body27_e16119_d_n7;
            locals.var_t6__blk337_dn10 = assign12750_body27_e16119_d_n10;
            locals.var_t6__blk337_dn11 = assign12750_body27_e16119_d_n11;
            locals.var_t6__blk337_dn12 = assign12750_body27_e16119_d_n12;
            locals.var_t6__blk337_dn17 = assign12750_body27_e16119_d_n17;
            let assign12750_body28_e16122: f64 = (locals.var_t6__blk337 - locals.var_phi_sl_bulk);
            let assign12750_body28_e16123: f64 = (assign12750_body28_e16122).abs();
            let assign12750_body28_e16125: f64 = if assign12750_body28_e16123 < 5e-12 { 1.0 } else { 0.0 };
            locals.var_guard348 = assign12750_body28_e16125;
            let (assign12750_body29_e16143,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 == 0.0)) && (locals.var_guard348 != 0.0)) {
        (locals.var_lp_sl_max,)
    } else {
        (locals.var_lp_sl,)
    }
};
            locals.var_lp_sl = assign12750_body29_e16143;
            let (assign12750_body30_e16159, assign12750_body30_e16159_d_n0, assign12750_body30_e16159_d_n2, assign12750_body30_e16159_d_n6, assign12750_body30_e16159_d_n7, assign12750_body30_e16159_d_n10, assign12750_body30_e16159_d_n11, assign12750_body30_e16159_d_n12, assign12750_body30_e16159_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 == 0.0)) {
        (locals.var_t6__blk337, locals.var_t6__blk337_dn0, locals.var_t6__blk337_dn2, locals.var_t6__blk337_dn6, locals.var_t6__blk337_dn7, locals.var_t6__blk337_dn10, locals.var_t6__blk337_dn11, locals.var_t6__blk337_dn12, locals.var_t6__blk337_dn17,)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
            locals.var_phi_sl_bulk = assign12750_body30_e16159;
            locals.var_phi_sl_bulk_dn0 = assign12750_body30_e16159_d_n0;
            locals.var_phi_sl_bulk_dn2 = assign12750_body30_e16159_d_n2;
            locals.var_phi_sl_bulk_dn6 = assign12750_body30_e16159_d_n6;
            locals.var_phi_sl_bulk_dn7 = assign12750_body30_e16159_d_n7;
            locals.var_phi_sl_bulk_dn10 = assign12750_body30_e16159_d_n10;
            locals.var_phi_sl_bulk_dn11 = assign12750_body30_e16159_d_n11;
            locals.var_phi_sl_bulk_dn12 = assign12750_body30_e16159_d_n12;
            locals.var_phi_sl_bulk_dn17 = assign12750_body30_e16159_d_n17;
            let (assign12750_body31_e16175, assign12750_body31_e16175_d_n0, assign12750_body31_e16175_d_n2, assign12750_body31_e16175_d_n6, assign12750_body31_e16175_d_n7, assign12750_body31_e16175_d_n10, assign12750_body31_e16175_d_n11, assign12750_body31_e16175_d_n12, assign12750_body31_e16175_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 == 0.0)) {
        (locals.var_t4__blk335, locals.var_t4__blk335_dn0, locals.var_t4__blk335_dn2, locals.var_t4__blk335_dn6, locals.var_t4__blk335_dn7, locals.var_t4__blk335_dn10, locals.var_t4__blk335_dn11, locals.var_t4__blk335_dn12, locals.var_t4__blk335_dn17,)
    } else {
        (locals.var_q_sl_bulk, locals.var_q_sl_bulk_dn0, locals.var_q_sl_bulk_dn2, locals.var_q_sl_bulk_dn6, locals.var_q_sl_bulk_dn7, locals.var_q_sl_bulk_dn10, locals.var_q_sl_bulk_dn11, locals.var_q_sl_bulk_dn12, locals.var_q_sl_bulk_dn17,)
    }
};
            locals.var_q_sl_bulk = assign12750_body31_e16175;
            locals.var_q_sl_bulk_dn0 = assign12750_body31_e16175_d_n0;
            locals.var_q_sl_bulk_dn2 = assign12750_body31_e16175_d_n2;
            locals.var_q_sl_bulk_dn6 = assign12750_body31_e16175_d_n6;
            locals.var_q_sl_bulk_dn7 = assign12750_body31_e16175_d_n7;
            locals.var_q_sl_bulk_dn10 = assign12750_body31_e16175_d_n10;
            locals.var_q_sl_bulk_dn11 = assign12750_body31_e16175_d_n11;
            locals.var_q_sl_bulk_dn12 = assign12750_body31_e16175_d_n12;
            locals.var_q_sl_bulk_dn17 = assign12750_body31_e16175_d_n17;
            let (assign12750_body32_e16193,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 == 0.0)) {
        let assign12750_body32_e16191: f64 = (locals.var_lp_sl + 1.0);
        (assign12750_body32_e16191,)
    } else {
        (locals.var_lp_sl,)
    }
};
            locals.var_lp_sl = assign12750_body32_e16193;
        }

    }

    pub(super) fn stamp_transient_block_40(
        locals: &mut StampLocals,
    ) {
        let (assign12760_e16211, assign12760_e16211_d_n0, assign12760_e16211_d_n2, assign12760_e16211_d_n6, assign12760_e16211_d_n7, assign12760_e16211_d_n10, assign12760_e16211_d_n11, assign12760_e16211_d_n12, assign12760_e16211_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 == 0.0)) {
        let assign12760_e16209: f64 = (locals.var_vbsbiz + locals.var_phi_sl_bulk);
        (assign12760_e16209, (locals.var_vbsbiz_dn0 + locals.var_phi_sl_bulk_dn0), (locals.var_vbsbiz_dn2 + locals.var_phi_sl_bulk_dn2), (locals.var_vbsbiz_dn6 + locals.var_phi_sl_bulk_dn6), (locals.var_vbsbiz_dn7 + locals.var_phi_sl_bulk_dn7), (locals.var_vbsbiz_dn10 + locals.var_phi_sl_bulk_dn10), (locals.var_vbsbiz_dn11 + locals.var_phi_sl_bulk_dn11), (locals.var_vbsbiz_dn12 + locals.var_phi_sl_bulk_dn12), (locals.var_vbsbiz_dn17 + locals.var_phi_sl_bulk_dn17),)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
        locals.var_phi_sl_bulk = assign12760_e16211;
        locals.var_phi_sl_bulk_dn0 = assign12760_e16211_d_n0;
        locals.var_phi_sl_bulk_dn2 = assign12760_e16211_d_n2;
        locals.var_phi_sl_bulk_dn6 = assign12760_e16211_d_n6;
        locals.var_phi_sl_bulk_dn7 = assign12760_e16211_d_n7;
        locals.var_phi_sl_bulk_dn10 = assign12760_e16211_d_n10;
        locals.var_phi_sl_bulk_dn11 = assign12760_e16211_d_n11;
        locals.var_phi_sl_bulk_dn12 = assign12760_e16211_d_n12;
        locals.var_phi_sl_bulk_dn17 = assign12760_e16211_d_n17;

        let (assign12770_e16231, assign12770_e16231_d_n0, assign12770_e16231_d_n2, assign12770_e16231_d_n6, assign12770_e16231_d_n7, assign12770_e16231_d_n10, assign12770_e16231_d_n11, assign12770_e16231_d_n12, assign12770_e16231_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard340 == 0.0)) {
        let assign12770_e16228: f64 = (locals.var_q_sl_bulk / locals.var_c_box);
        let assign12770_e16229: f64 = (locals.var_phi_sl_bulk - assign12770_e16228);
        (assign12770_e16229, (locals.var_phi_sl_bulk_dn0 - (locals.var_q_sl_bulk_dn0 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn2 - (locals.var_q_sl_bulk_dn2 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn6 - (locals.var_q_sl_bulk_dn6 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn7 - (locals.var_q_sl_bulk_dn7 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn10 - (locals.var_q_sl_bulk_dn10 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn11 - (locals.var_q_sl_bulk_dn11 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn12 - (locals.var_q_sl_bulk_dn12 / locals.var_c_box)), (locals.var_phi_sl_bulk_dn17 - (locals.var_q_sl_bulk_dn17 / locals.var_c_box)),)
    } else {
        (locals.var_phi_bl_soi, locals.var_phi_bl_soi_dn0, locals.var_phi_bl_soi_dn2, locals.var_phi_bl_soi_dn6, locals.var_phi_bl_soi_dn7, locals.var_phi_bl_soi_dn10, locals.var_phi_bl_soi_dn11, locals.var_phi_bl_soi_dn12, locals.var_phi_bl_soi_dn17,)
    }
};
        locals.var_phi_bl_soi = assign12770_e16231;
        locals.var_phi_bl_soi_dn0 = assign12770_e16231_d_n0;
        locals.var_phi_bl_soi_dn2 = assign12770_e16231_d_n2;
        locals.var_phi_bl_soi_dn6 = assign12770_e16231_d_n6;
        locals.var_phi_bl_soi_dn7 = assign12770_e16231_d_n7;
        locals.var_phi_bl_soi_dn10 = assign12770_e16231_d_n10;
        locals.var_phi_bl_soi_dn11 = assign12770_e16231_d_n11;
        locals.var_phi_bl_soi_dn12 = assign12770_e16231_d_n12;
        locals.var_phi_bl_soi_dn17 = assign12770_e16231_d_n17;

        let assign12780_e16234: f64 = if locals.var_phi_bl_soi < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard349 = assign12780_e16234;

        let (assign12790_e16249, assign12790_e16249_d_n0, assign12790_e16249_d_n2, assign12790_e16249_d_n6, assign12790_e16249_d_n7, assign12790_e16249_d_n10, assign12790_e16249_d_n11, assign12790_e16249_d_n12, assign12790_e16249_d_n17,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard318 == 0.0)) && (locals.var_guard319 == 0.0)) && (locals.var_guard349 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_bl_soi, locals.var_phi_bl_soi_dn0, locals.var_phi_bl_soi_dn2, locals.var_phi_bl_soi_dn6, locals.var_phi_bl_soi_dn7, locals.var_phi_bl_soi_dn10, locals.var_phi_bl_soi_dn11, locals.var_phi_bl_soi_dn12, locals.var_phi_bl_soi_dn17,)
    }
};
        locals.var_phi_bl_soi = assign12790_e16249;
        locals.var_phi_bl_soi_dn0 = assign12790_e16249_d_n0;
        locals.var_phi_bl_soi_dn2 = assign12790_e16249_d_n2;
        locals.var_phi_bl_soi_dn6 = assign12790_e16249_d_n6;
        locals.var_phi_bl_soi_dn7 = assign12790_e16249_d_n7;
        locals.var_phi_bl_soi_dn10 = assign12790_e16249_d_n10;
        locals.var_phi_bl_soi_dn11 = assign12790_e16249_d_n11;
        locals.var_phi_bl_soi_dn12 = assign12790_e16249_d_n12;
        locals.var_phi_bl_soi_dn17 = assign12790_e16249_d_n17;

        let assign12800_e16252: f64 = if locals.var_phi_s0_soi < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard385 = assign12800_e16252;

        let (assign12810_e16261, assign12810_e16261_d_n0, assign12810_e16261_d_n2, assign12810_e16261_d_n6, assign12810_e16261_d_n7, assign12810_e16261_d_n10, assign12810_e16261_d_n11, assign12810_e16261_d_n12, assign12810_e16261_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard385 != 0.0)) {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
        locals.var_phi_sl_soi = assign12810_e16261;
        locals.var_phi_sl_soi_dn0 = assign12810_e16261_d_n0;
        locals.var_phi_sl_soi_dn2 = assign12810_e16261_d_n2;
        locals.var_phi_sl_soi_dn6 = assign12810_e16261_d_n6;
        locals.var_phi_sl_soi_dn7 = assign12810_e16261_d_n7;
        locals.var_phi_sl_soi_dn10 = assign12810_e16261_d_n10;
        locals.var_phi_sl_soi_dn11 = assign12810_e16261_d_n11;
        locals.var_phi_sl_soi_dn12 = assign12810_e16261_d_n12;
        locals.var_phi_sl_soi_dn17 = assign12810_e16261_d_n17;

        let assign12820_e16264: f64 = if locals.var_phi_bl_soi < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard386 = assign12820_e16264;

        let (assign12830_e16281, assign12830_e16281_d_n0, assign12830_e16281_d_n2, assign12830_e16281_d_n6, assign12830_e16281_d_n7, assign12830_e16281_d_n10, assign12830_e16281_d_n11, assign12830_e16281_d_n12, assign12830_e16281_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard386 != 0.0)) {
        let assign12830_e16275: f64 = (0.5 * locals.var_q_fd_soi);
        let assign12830_e16277: f64 = (assign12830_e16275 + locals.var_q_s0_bulk);
        let assign12830_e16278: f64 = (locals.var_c_soi_inv__blk113 * assign12830_e16277);
        let assign12830_e16279: f64 = (locals.var_phi_sl_soi + assign12830_e16278);
        (assign12830_e16279, (locals.var_phi_sl_soi_dn0 + (locals.var_c_soi_inv__blk113 * ((0.5 * locals.var_q_fd_soi_dn0) + locals.var_q_s0_bulk_dn0))), (locals.var_phi_sl_soi_dn2 + (locals.var_c_soi_inv__blk113 * ((0.5 * locals.var_q_fd_soi_dn2) + locals.var_q_s0_bulk_dn2))), (locals.var_phi_sl_soi_dn6 + (locals.var_c_soi_inv__blk113 * ((0.5 * locals.var_q_fd_soi_dn6) + locals.var_q_s0_bulk_dn6))), (locals.var_phi_sl_soi_dn7 + (locals.var_c_soi_inv__blk113 * ((0.5 * locals.var_q_fd_soi_dn7) + locals.var_q_s0_bulk_dn7))), (locals.var_phi_sl_soi_dn10 + (locals.var_c_soi_inv__blk113 * ((0.5 * locals.var_q_fd_soi_dn10) + locals.var_q_s0_bulk_dn10))), (locals.var_phi_sl_soi_dn11 + (locals.var_c_soi_inv__blk113 * ((0.5 * locals.var_q_fd_soi_dn11) + locals.var_q_s0_bulk_dn11))), (locals.var_phi_sl_soi_dn12 + (locals.var_c_soi_inv__blk113 * ((0.5 * locals.var_q_fd_soi_dn12) + locals.var_q_s0_bulk_dn12))), (locals.var_phi_sl_soi_dn17 + (locals.var_c_soi_inv__blk113 * ((0.5 * locals.var_q_fd_soi_dn17) + locals.var_q_s0_bulk_dn17))),)
    } else {
        (locals.var_phi_bl_soi, locals.var_phi_bl_soi_dn0, locals.var_phi_bl_soi_dn2, locals.var_phi_bl_soi_dn6, locals.var_phi_bl_soi_dn7, locals.var_phi_bl_soi_dn10, locals.var_phi_bl_soi_dn11, locals.var_phi_bl_soi_dn12, locals.var_phi_bl_soi_dn17,)
    }
};
        locals.var_phi_bl_soi = assign12830_e16281;
        locals.var_phi_bl_soi_dn0 = assign12830_e16281_d_n0;
        locals.var_phi_bl_soi_dn2 = assign12830_e16281_d_n2;
        locals.var_phi_bl_soi_dn6 = assign12830_e16281_d_n6;
        locals.var_phi_bl_soi_dn7 = assign12830_e16281_d_n7;
        locals.var_phi_bl_soi_dn10 = assign12830_e16281_d_n10;
        locals.var_phi_bl_soi_dn11 = assign12830_e16281_d_n11;
        locals.var_phi_bl_soi_dn12 = assign12830_e16281_d_n12;
        locals.var_phi_bl_soi_dn17 = assign12830_e16281_d_n17;

        let (assign12840_e16288, assign12840_e16288_d_n0, assign12840_e16288_d_n2, assign12840_e16288_d_n6, assign12840_e16288_d_n7, assign12840_e16288_d_n10, assign12840_e16288_d_n11, assign12840_e16288_d_n12, assign12840_e16288_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    } else {
        (locals.var_phi_sl_soi_ini, locals.var_phi_sl_soi_ini_dn0, locals.var_phi_sl_soi_ini_dn2, locals.var_phi_sl_soi_ini_dn6, locals.var_phi_sl_soi_ini_dn7, locals.var_phi_sl_soi_ini_dn10, locals.var_phi_sl_soi_ini_dn11, locals.var_phi_sl_soi_ini_dn12, locals.var_phi_sl_soi_ini_dn17,)
    }
};
        locals.var_phi_sl_soi_ini = assign12840_e16288;
        locals.var_phi_sl_soi_ini_dn0 = assign12840_e16288_d_n0;
        locals.var_phi_sl_soi_ini_dn2 = assign12840_e16288_d_n2;
        locals.var_phi_sl_soi_ini_dn6 = assign12840_e16288_d_n6;
        locals.var_phi_sl_soi_ini_dn7 = assign12840_e16288_d_n7;
        locals.var_phi_sl_soi_ini_dn10 = assign12840_e16288_d_n10;
        locals.var_phi_sl_soi_ini_dn11 = assign12840_e16288_d_n11;
        locals.var_phi_sl_soi_ini_dn12 = assign12840_e16288_d_n12;
        locals.var_phi_sl_soi_ini_dn17 = assign12840_e16288_d_n17;

        let (assign12850_e16295, assign12850_e16295_d_n0, assign12850_e16295_d_n2, assign12850_e16295_d_n6, assign12850_e16295_d_n7, assign12850_e16295_d_n10, assign12850_e16295_d_n11, assign12850_e16295_d_n12, assign12850_e16295_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        (locals.var_phi_bl_soi, locals.var_phi_bl_soi_dn0, locals.var_phi_bl_soi_dn2, locals.var_phi_bl_soi_dn6, locals.var_phi_bl_soi_dn7, locals.var_phi_bl_soi_dn10, locals.var_phi_bl_soi_dn11, locals.var_phi_bl_soi_dn12, locals.var_phi_bl_soi_dn17,)
    } else {
        (locals.var_phi_bl_soi_ini, locals.var_phi_bl_soi_ini_dn0, locals.var_phi_bl_soi_ini_dn2, locals.var_phi_bl_soi_ini_dn6, locals.var_phi_bl_soi_ini_dn7, locals.var_phi_bl_soi_ini_dn10, locals.var_phi_bl_soi_ini_dn11, locals.var_phi_bl_soi_ini_dn12, locals.var_phi_bl_soi_ini_dn17,)
    }
};
        locals.var_phi_bl_soi_ini = assign12850_e16295;
        locals.var_phi_bl_soi_ini_dn0 = assign12850_e16295_d_n0;
        locals.var_phi_bl_soi_ini_dn2 = assign12850_e16295_d_n2;
        locals.var_phi_bl_soi_ini_dn6 = assign12850_e16295_d_n6;
        locals.var_phi_bl_soi_ini_dn7 = assign12850_e16295_d_n7;
        locals.var_phi_bl_soi_ini_dn10 = assign12850_e16295_d_n10;
        locals.var_phi_bl_soi_ini_dn11 = assign12850_e16295_d_n11;
        locals.var_phi_bl_soi_ini_dn12 = assign12850_e16295_d_n12;
        locals.var_phi_bl_soi_ini_dn17 = assign12850_e16295_d_n17;

        let (assign12860_e16302, assign12860_e16302_d_n0, assign12860_e16302_d_n2, assign12860_e16302_d_n6, assign12860_e16302_d_n7, assign12860_e16302_d_n10, assign12860_e16302_d_n11, assign12860_e16302_d_n12, assign12860_e16302_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    } else {
        (locals.var_phi_sl_bulk_ini, locals.var_phi_sl_bulk_ini_dn0, locals.var_phi_sl_bulk_ini_dn2, locals.var_phi_sl_bulk_ini_dn6, locals.var_phi_sl_bulk_ini_dn7, locals.var_phi_sl_bulk_ini_dn10, locals.var_phi_sl_bulk_ini_dn11, locals.var_phi_sl_bulk_ini_dn12, locals.var_phi_sl_bulk_ini_dn17,)
    }
};
        locals.var_phi_sl_bulk_ini = assign12860_e16302;
        locals.var_phi_sl_bulk_ini_dn0 = assign12860_e16302_d_n0;
        locals.var_phi_sl_bulk_ini_dn2 = assign12860_e16302_d_n2;
        locals.var_phi_sl_bulk_ini_dn6 = assign12860_e16302_d_n6;
        locals.var_phi_sl_bulk_ini_dn7 = assign12860_e16302_d_n7;
        locals.var_phi_sl_bulk_ini_dn10 = assign12860_e16302_d_n10;
        locals.var_phi_sl_bulk_ini_dn11 = assign12860_e16302_d_n11;
        locals.var_phi_sl_bulk_ini_dn12 = assign12860_e16302_d_n12;
        locals.var_phi_sl_bulk_ini_dn17 = assign12860_e16302_d_n17;

        let (assign12870_e16309,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign12870_e16309;

        let (assign12880_e16316,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_brk8,)
    }
};
        locals.var_flg_brk8 = assign12880_e16316;

        let (assign12890_e16323,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_sl,)
    }
};
        locals.var_lp_sl = assign12890_e16323;

    }

    pub(super) fn stamp_transient_block_41(
        locals: &mut StampLocals,
    ) {
        let mut assign12900_loop_guard: usize = 0;
        while {
            let assign12900_cond_e16331: f64 = if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_lp_sl <= locals.var_lp_sl_max)) { 1.0 } else { 0.0 };
            assign12900_cond_e16331 != 0.0
        } {
            assign12900_loop_guard += 1;
            assert!(assign12900_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign12900_body0_e16340, assign12900_body0_e16340_d_n0, assign12900_body0_e16340_d_n2, assign12900_body0_e16340_d_n6, assign12900_body0_e16340_d_n7, assign12900_body0_e16340_d_n10, assign12900_body0_e16340_d_n11, assign12900_body0_e16340_d_n12, assign12900_body0_e16340_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign12900_body0_e16338: f64 = (locals.var_phi_sl_bulk - locals.var_vbsbiz);
        (assign12900_body0_e16338, (locals.var_phi_sl_bulk_dn0 - locals.var_vbsbiz_dn0), (locals.var_phi_sl_bulk_dn2 - locals.var_vbsbiz_dn2), (locals.var_phi_sl_bulk_dn6 - locals.var_vbsbiz_dn6), (locals.var_phi_sl_bulk_dn7 - locals.var_vbsbiz_dn7), (locals.var_phi_sl_bulk_dn10 - locals.var_vbsbiz_dn10), (locals.var_phi_sl_bulk_dn11 - locals.var_vbsbiz_dn11), (locals.var_phi_sl_bulk_dn12 - locals.var_vbsbiz_dn12), (locals.var_phi_sl_bulk_dn17 - locals.var_vbsbiz_dn17),)
    } else {
        (locals.var_t1__blk351, locals.var_t1__blk351_dn0, locals.var_t1__blk351_dn2, locals.var_t1__blk351_dn6, locals.var_t1__blk351_dn7, locals.var_t1__blk351_dn10, locals.var_t1__blk351_dn11, locals.var_t1__blk351_dn12, locals.var_t1__blk351_dn17,)
    }
};
            locals.var_t1__blk351 = assign12900_body0_e16340;
            locals.var_t1__blk351_dn0 = assign12900_body0_e16340_d_n0;
            locals.var_t1__blk351_dn2 = assign12900_body0_e16340_d_n2;
            locals.var_t1__blk351_dn6 = assign12900_body0_e16340_d_n6;
            locals.var_t1__blk351_dn7 = assign12900_body0_e16340_d_n7;
            locals.var_t1__blk351_dn10 = assign12900_body0_e16340_d_n10;
            locals.var_t1__blk351_dn11 = assign12900_body0_e16340_d_n11;
            locals.var_t1__blk351_dn12 = assign12900_body0_e16340_d_n12;
            locals.var_t1__blk351_dn17 = assign12900_body0_e16340_d_n17;
            let (assign12900_body1_e16349, assign12900_body1_e16349_d_n0, assign12900_body1_e16349_d_n2, assign12900_body1_e16349_d_n6, assign12900_body1_e16349_d_n7, assign12900_body1_e16349_d_n10, assign12900_body1_e16349_d_n11, assign12900_body1_e16349_d_n12, assign12900_body1_e16349_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign12900_body1_e16347: f64 = (locals.var_beta * locals.var_t1__blk351);
        (assign12900_body1_e16347, (locals.var_beta * locals.var_t1__blk351_dn0), (locals.var_beta * locals.var_t1__blk351_dn2), (locals.var_beta * locals.var_t1__blk351_dn6), (locals.var_beta * locals.var_t1__blk351_dn7), ((locals.var_beta_dn10 * locals.var_t1__blk351) + (locals.var_beta * locals.var_t1__blk351_dn10)), (locals.var_beta * locals.var_t1__blk351_dn11), (locals.var_beta * locals.var_t1__blk351_dn12), (locals.var_beta * locals.var_t1__blk351_dn17),)
    } else {
        (locals.var_el, locals.var_el_dn0, locals.var_el_dn2, locals.var_el_dn6, locals.var_el_dn7, locals.var_el_dn10, locals.var_el_dn11, locals.var_el_dn12, locals.var_el_dn17,)
    }
};
            locals.var_el = assign12900_body1_e16349;
            locals.var_el_dn0 = assign12900_body1_e16349_d_n0;
            locals.var_el_dn2 = assign12900_body1_e16349_d_n2;
            locals.var_el_dn6 = assign12900_body1_e16349_d_n6;
            locals.var_el_dn7 = assign12900_body1_e16349_d_n7;
            locals.var_el_dn10 = assign12900_body1_e16349_d_n10;
            locals.var_el_dn11 = assign12900_body1_e16349_d_n11;
            locals.var_el_dn12 = assign12900_body1_e16349_d_n12;
            locals.var_el_dn17 = assign12900_body1_e16349_d_n17;
            let (assign12900_body2_e16358, assign12900_body2_e16358_d_n0, assign12900_body2_e16358_d_n2, assign12900_body2_e16358_d_n6, assign12900_body2_e16358_d_n7, assign12900_body2_e16358_d_n10, assign12900_body2_e16358_d_n11, assign12900_body2_e16358_d_n12, assign12900_body2_e16358_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign12900_body2_e16355: f64 = (-locals.var_el);
        let assign12900_body2_e16356: f64 = (assign12900_body2_e16355).exp();
        (assign12900_body2_e16356, (assign12900_body2_e16356 * (-locals.var_el_dn0)), (assign12900_body2_e16356 * (-locals.var_el_dn2)), (assign12900_body2_e16356 * (-locals.var_el_dn6)), (assign12900_body2_e16356 * (-locals.var_el_dn7)), (assign12900_body2_e16356 * (-locals.var_el_dn10)), (assign12900_body2_e16356 * (-locals.var_el_dn11)), (assign12900_body2_e16356 * (-locals.var_el_dn12)), (assign12900_body2_e16356 * (-locals.var_el_dn17)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign12900_body2_e16358;
            locals.var_t0_dn0 = assign12900_body2_e16358_d_n0;
            locals.var_t0_dn2 = assign12900_body2_e16358_d_n2;
            locals.var_t0_dn6 = assign12900_body2_e16358_d_n6;
            locals.var_t0_dn7 = assign12900_body2_e16358_d_n7;
            locals.var_t0_dn10 = assign12900_body2_e16358_d_n10;
            locals.var_t0_dn11 = assign12900_body2_e16358_d_n11;
            locals.var_t0_dn12 = assign12900_body2_e16358_d_n12;
            locals.var_t0_dn17 = assign12900_body2_e16358_d_n17;
            let assign12900_body3_e16361: f64 = (-1e-9);
            let assign12900_body3_e16362: f64 = if locals.var_t1__blk351 < assign12900_body3_e16361 { 1.0 } else { 0.0 };
            locals.var_guard387 = assign12900_body3_e16362;
            let (assign12900_body4_e16378, assign12900_body4_e16378_d_n0, assign12900_body4_e16378_d_n2, assign12900_body4_e16378_d_n6, assign12900_body4_e16378_d_n7, assign12900_body4_e16378_d_n10, assign12900_body4_e16378_d_n11, assign12900_body4_e16378_d_n12, assign12900_body4_e16378_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign12900_body4_e16372: f64 = (locals.var_t0 + locals.var_el);
        let assign12900_body4_e16374: f64 = (assign12900_body4_e16372 - 1.0);
        let assign12900_body4_e16375: f64 = (assign12900_body4_e16374).sqrt();
        let assign12900_body4_e16376: f64 = (locals.var_cnst0bulk * assign12900_body4_e16375);
        (assign12900_body4_e16376, (locals.var_cnst0bulk * ((locals.var_t0_dn0 + locals.var_el_dn0) / (2.0 * assign12900_body4_e16375))), (locals.var_cnst0bulk * ((locals.var_t0_dn2 + locals.var_el_dn2) / (2.0 * assign12900_body4_e16375))), (locals.var_cnst0bulk * ((locals.var_t0_dn6 + locals.var_el_dn6) / (2.0 * assign12900_body4_e16375))), (locals.var_cnst0bulk * ((locals.var_t0_dn7 + locals.var_el_dn7) / (2.0 * assign12900_body4_e16375))), ((locals.var_cnst0bulk_dn10 * assign12900_body4_e16375) + (locals.var_cnst0bulk * ((locals.var_t0_dn10 + locals.var_el_dn10) / (2.0 * assign12900_body4_e16375)))), (locals.var_cnst0bulk * ((locals.var_t0_dn11 + locals.var_el_dn11) / (2.0 * assign12900_body4_e16375))), (locals.var_cnst0bulk * ((locals.var_t0_dn12 + locals.var_el_dn12) / (2.0 * assign12900_body4_e16375))), (locals.var_cnst0bulk * ((locals.var_t0_dn17 + locals.var_el_dn17) / (2.0 * assign12900_body4_e16375))),)
    } else {
        (locals.var_q_sl_bulk, locals.var_q_sl_bulk_dn0, locals.var_q_sl_bulk_dn2, locals.var_q_sl_bulk_dn6, locals.var_q_sl_bulk_dn7, locals.var_q_sl_bulk_dn10, locals.var_q_sl_bulk_dn11, locals.var_q_sl_bulk_dn12, locals.var_q_sl_bulk_dn17,)
    }
};
            locals.var_q_sl_bulk = assign12900_body4_e16378;
            locals.var_q_sl_bulk_dn0 = assign12900_body4_e16378_d_n0;
            locals.var_q_sl_bulk_dn2 = assign12900_body4_e16378_d_n2;
            locals.var_q_sl_bulk_dn6 = assign12900_body4_e16378_d_n6;
            locals.var_q_sl_bulk_dn7 = assign12900_body4_e16378_d_n7;
            locals.var_q_sl_bulk_dn10 = assign12900_body4_e16378_d_n10;
            locals.var_q_sl_bulk_dn11 = assign12900_body4_e16378_d_n11;
            locals.var_q_sl_bulk_dn12 = assign12900_body4_e16378_d_n12;
            locals.var_q_sl_bulk_dn17 = assign12900_body4_e16378_d_n17;
            let (assign12900_body5_e16394, assign12900_body5_e16394_d_n0, assign12900_body5_e16394_d_n2, assign12900_body5_e16394_d_n6, assign12900_body5_e16394_d_n7, assign12900_body5_e16394_d_n10, assign12900_body5_e16394_d_n11, assign12900_body5_e16394_d_n12, assign12900_body5_e16394_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign12900_body5_e16387: f64 = (-locals.var_t0);
        let assign12900_body5_e16389: f64 = (assign12900_body5_e16387 + 1.0);
        let assign12900_body5_e16390: f64 = (locals.var_c0bulk * assign12900_body5_e16389);
        let assign12900_body5_e16392: f64 = (assign12900_body5_e16390 / locals.var_q_sl_bulk);
        (assign12900_body5_e16392, ((((locals.var_c0bulk * (-locals.var_t0_dn0)) * locals.var_q_sl_bulk) - (assign12900_body5_e16390 * locals.var_q_sl_bulk_dn0)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * (-locals.var_t0_dn2)) * locals.var_q_sl_bulk) - (assign12900_body5_e16390 * locals.var_q_sl_bulk_dn2)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * (-locals.var_t0_dn6)) * locals.var_q_sl_bulk) - (assign12900_body5_e16390 * locals.var_q_sl_bulk_dn6)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * (-locals.var_t0_dn7)) * locals.var_q_sl_bulk) - (assign12900_body5_e16390 * locals.var_q_sl_bulk_dn7)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * (-locals.var_t0_dn10)) * locals.var_q_sl_bulk) - (assign12900_body5_e16390 * locals.var_q_sl_bulk_dn10)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * (-locals.var_t0_dn11)) * locals.var_q_sl_bulk) - (assign12900_body5_e16390 * locals.var_q_sl_bulk_dn11)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * (-locals.var_t0_dn12)) * locals.var_q_sl_bulk) - (assign12900_body5_e16390 * locals.var_q_sl_bulk_dn12)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * (-locals.var_t0_dn17)) * locals.var_q_sl_bulk) - (assign12900_body5_e16390 * locals.var_q_sl_bulk_dn17)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)),)
    } else {
        (locals.var_q_sl_bulk_dpsb, locals.var_q_sl_bulk_dpsb_dn0, locals.var_q_sl_bulk_dpsb_dn2, locals.var_q_sl_bulk_dpsb_dn6, locals.var_q_sl_bulk_dpsb_dn7, locals.var_q_sl_bulk_dpsb_dn10, locals.var_q_sl_bulk_dpsb_dn11, locals.var_q_sl_bulk_dpsb_dn12, locals.var_q_sl_bulk_dpsb_dn17,)
    }
};
            locals.var_q_sl_bulk_dpsb = assign12900_body5_e16394;
            locals.var_q_sl_bulk_dpsb_dn0 = assign12900_body5_e16394_d_n0;
            locals.var_q_sl_bulk_dpsb_dn2 = assign12900_body5_e16394_d_n2;
            locals.var_q_sl_bulk_dpsb_dn6 = assign12900_body5_e16394_d_n6;
            locals.var_q_sl_bulk_dpsb_dn7 = assign12900_body5_e16394_d_n7;
            locals.var_q_sl_bulk_dpsb_dn10 = assign12900_body5_e16394_d_n10;
            locals.var_q_sl_bulk_dpsb_dn11 = assign12900_body5_e16394_d_n11;
            locals.var_q_sl_bulk_dpsb_dn12 = assign12900_body5_e16394_d_n12;
            locals.var_q_sl_bulk_dpsb_dn17 = assign12900_body5_e16394_d_n17;
            let assign12900_body6_e16397: f64 = if locals.var_t1__blk351 > 1e-9 { 1.0 } else { 0.0 };
            locals.var_guard388 = assign12900_body6_e16397;
            let (assign12900_body7_e16410, assign12900_body7_e16410_d_n0, assign12900_body7_e16410_d_n2, assign12900_body7_e16410_d_n6, assign12900_body7_e16410_d_n7, assign12900_body7_e16410_d_n10, assign12900_body7_e16410_d_n11, assign12900_body7_e16410_d_n12, assign12900_body7_e16410_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard387 == 0.0)) && (locals.var_guard388 != 0.0)) {
        let assign12900_body7_e16408: f64 = (locals.var_el).exp();
        (assign12900_body7_e16408, (assign12900_body7_e16408 * locals.var_el_dn0), (assign12900_body7_e16408 * locals.var_el_dn2), (assign12900_body7_e16408 * locals.var_el_dn6), (assign12900_body7_e16408 * locals.var_el_dn7), (assign12900_body7_e16408 * locals.var_el_dn10), (assign12900_body7_e16408 * locals.var_el_dn11), (assign12900_body7_e16408 * locals.var_el_dn12), (assign12900_body7_e16408 * locals.var_el_dn17),)
    } else {
        (locals.var_t2__blk352, locals.var_t2__blk352_dn0, locals.var_t2__blk352_dn2, locals.var_t2__blk352_dn6, locals.var_t2__blk352_dn7, locals.var_t2__blk352_dn10, locals.var_t2__blk352_dn11, locals.var_t2__blk352_dn12, locals.var_t2__blk352_dn17,)
    }
};
            locals.var_t2__blk352 = assign12900_body7_e16410;
            locals.var_t2__blk352_dn0 = assign12900_body7_e16410_d_n0;
            locals.var_t2__blk352_dn2 = assign12900_body7_e16410_d_n2;
            locals.var_t2__blk352_dn6 = assign12900_body7_e16410_d_n6;
            locals.var_t2__blk352_dn7 = assign12900_body7_e16410_d_n7;
            locals.var_t2__blk352_dn10 = assign12900_body7_e16410_d_n10;
            locals.var_t2__blk352_dn11 = assign12900_body7_e16410_d_n11;
            locals.var_t2__blk352_dn12 = assign12900_body7_e16410_d_n12;
            locals.var_t2__blk352_dn17 = assign12900_body7_e16410_d_n17;
            let (assign12900_body8_e16438, assign12900_body8_e16438_d_n0, assign12900_body8_e16438_d_n2, assign12900_body8_e16438_d_n6, assign12900_body8_e16438_d_n7, assign12900_body8_e16438_d_n10, assign12900_body8_e16438_d_n11, assign12900_body8_e16438_d_n12, assign12900_body8_e16438_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard387 == 0.0)) && (locals.var_guard388 != 0.0)) {
        let assign12900_body8_e16421: f64 = (-locals.var_cnst0bulk);
        let assign12900_body8_e16424: f64 = (locals.var_t0 + locals.var_el);
        let assign12900_body8_e16426: f64 = (assign12900_body8_e16424 - 1.0);
        let assign12900_body8_e16430: f64 = (locals.var_t2__blk352 + locals.var_el);
        let assign12900_body8_e16432: f64 = (assign12900_body8_e16430 - 1.0);
        let assign12900_body8_e16433: f64 = (locals.var_cnst1bulk * assign12900_body8_e16432);
        let assign12900_body8_e16434: f64 = (assign12900_body8_e16426 + assign12900_body8_e16433);
        let assign12900_body8_e16435: f64 = (assign12900_body8_e16434).sqrt();
        let assign12900_body8_e16436: f64 = (assign12900_body8_e16421 * assign12900_body8_e16435);
        (assign12900_body8_e16436, (assign12900_body8_e16421 * (((locals.var_t0_dn0 + locals.var_el_dn0) + ((locals.var_cnst1bulk_dn0 * assign12900_body8_e16432) + (locals.var_cnst1bulk * (locals.var_t2__blk352_dn0 + locals.var_el_dn0)))) / (2.0 * assign12900_body8_e16435))), (assign12900_body8_e16421 * (((locals.var_t0_dn2 + locals.var_el_dn2) + ((locals.var_cnst1bulk_dn2 * assign12900_body8_e16432) + (locals.var_cnst1bulk * (locals.var_t2__blk352_dn2 + locals.var_el_dn2)))) / (2.0 * assign12900_body8_e16435))), (assign12900_body8_e16421 * (((locals.var_t0_dn6 + locals.var_el_dn6) + ((locals.var_cnst1bulk_dn6 * assign12900_body8_e16432) + (locals.var_cnst1bulk * (locals.var_t2__blk352_dn6 + locals.var_el_dn6)))) / (2.0 * assign12900_body8_e16435))), (assign12900_body8_e16421 * (((locals.var_t0_dn7 + locals.var_el_dn7) + ((locals.var_cnst1bulk_dn7 * assign12900_body8_e16432) + (locals.var_cnst1bulk * (locals.var_t2__blk352_dn7 + locals.var_el_dn7)))) / (2.0 * assign12900_body8_e16435))), (((-locals.var_cnst0bulk_dn10) * assign12900_body8_e16435) + (assign12900_body8_e16421 * (((locals.var_t0_dn10 + locals.var_el_dn10) + ((locals.var_cnst1bulk_dn10 * assign12900_body8_e16432) + (locals.var_cnst1bulk * (locals.var_t2__blk352_dn10 + locals.var_el_dn10)))) / (2.0 * assign12900_body8_e16435)))), (assign12900_body8_e16421 * (((locals.var_t0_dn11 + locals.var_el_dn11) + ((locals.var_cnst1bulk_dn11 * assign12900_body8_e16432) + (locals.var_cnst1bulk * (locals.var_t2__blk352_dn11 + locals.var_el_dn11)))) / (2.0 * assign12900_body8_e16435))), (assign12900_body8_e16421 * (((locals.var_t0_dn12 + locals.var_el_dn12) + ((locals.var_cnst1bulk_dn12 * assign12900_body8_e16432) + (locals.var_cnst1bulk * (locals.var_t2__blk352_dn12 + locals.var_el_dn12)))) / (2.0 * assign12900_body8_e16435))), (assign12900_body8_e16421 * (((locals.var_t0_dn17 + locals.var_el_dn17) + ((locals.var_cnst1bulk_dn17 * assign12900_body8_e16432) + (locals.var_cnst1bulk * (locals.var_t2__blk352_dn17 + locals.var_el_dn17)))) / (2.0 * assign12900_body8_e16435))),)
    } else {
        (locals.var_q_sl_bulk, locals.var_q_sl_bulk_dn0, locals.var_q_sl_bulk_dn2, locals.var_q_sl_bulk_dn6, locals.var_q_sl_bulk_dn7, locals.var_q_sl_bulk_dn10, locals.var_q_sl_bulk_dn11, locals.var_q_sl_bulk_dn12, locals.var_q_sl_bulk_dn17,)
    }
};
            locals.var_q_sl_bulk = assign12900_body8_e16438;
            locals.var_q_sl_bulk_dn0 = assign12900_body8_e16438_d_n0;
            locals.var_q_sl_bulk_dn2 = assign12900_body8_e16438_d_n2;
            locals.var_q_sl_bulk_dn6 = assign12900_body8_e16438_d_n6;
            locals.var_q_sl_bulk_dn7 = assign12900_body8_e16438_d_n7;
            locals.var_q_sl_bulk_dn10 = assign12900_body8_e16438_d_n10;
            locals.var_q_sl_bulk_dn11 = assign12900_body8_e16438_d_n11;
            locals.var_q_sl_bulk_dn12 = assign12900_body8_e16438_d_n12;
            locals.var_q_sl_bulk_dn17 = assign12900_body8_e16438_d_n17;
            let (assign12900_body9_e16463, assign12900_body9_e16463_d_n0, assign12900_body9_e16463_d_n2, assign12900_body9_e16463_d_n6, assign12900_body9_e16463_d_n7, assign12900_body9_e16463_d_n10, assign12900_body9_e16463_d_n11, assign12900_body9_e16463_d_n12, assign12900_body9_e16463_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard387 == 0.0)) && (locals.var_guard388 != 0.0)) {
        let assign12900_body9_e16450: f64 = (-locals.var_t0);
        let assign12900_body9_e16452: f64 = (assign12900_body9_e16450 + 1.0);
        let assign12900_body9_e16456: f64 = (locals.var_t2__blk352 + 1.0);
        let assign12900_body9_e16457: f64 = (locals.var_cnst1bulk * assign12900_body9_e16456);
        let assign12900_body9_e16458: f64 = (assign12900_body9_e16452 + assign12900_body9_e16457);
        let assign12900_body9_e16459: f64 = (locals.var_c0bulk * assign12900_body9_e16458);
        let assign12900_body9_e16461: f64 = (assign12900_body9_e16459 / locals.var_q_sl_bulk);
        (assign12900_body9_e16461, ((((locals.var_c0bulk * ((-locals.var_t0_dn0) + ((locals.var_cnst1bulk_dn0 * assign12900_body9_e16456) + (locals.var_cnst1bulk * locals.var_t2__blk352_dn0)))) * locals.var_q_sl_bulk) - (assign12900_body9_e16459 * locals.var_q_sl_bulk_dn0)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * ((-locals.var_t0_dn2) + ((locals.var_cnst1bulk_dn2 * assign12900_body9_e16456) + (locals.var_cnst1bulk * locals.var_t2__blk352_dn2)))) * locals.var_q_sl_bulk) - (assign12900_body9_e16459 * locals.var_q_sl_bulk_dn2)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * ((-locals.var_t0_dn6) + ((locals.var_cnst1bulk_dn6 * assign12900_body9_e16456) + (locals.var_cnst1bulk * locals.var_t2__blk352_dn6)))) * locals.var_q_sl_bulk) - (assign12900_body9_e16459 * locals.var_q_sl_bulk_dn6)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * ((-locals.var_t0_dn7) + ((locals.var_cnst1bulk_dn7 * assign12900_body9_e16456) + (locals.var_cnst1bulk * locals.var_t2__blk352_dn7)))) * locals.var_q_sl_bulk) - (assign12900_body9_e16459 * locals.var_q_sl_bulk_dn7)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * ((-locals.var_t0_dn10) + ((locals.var_cnst1bulk_dn10 * assign12900_body9_e16456) + (locals.var_cnst1bulk * locals.var_t2__blk352_dn10)))) * locals.var_q_sl_bulk) - (assign12900_body9_e16459 * locals.var_q_sl_bulk_dn10)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * ((-locals.var_t0_dn11) + ((locals.var_cnst1bulk_dn11 * assign12900_body9_e16456) + (locals.var_cnst1bulk * locals.var_t2__blk352_dn11)))) * locals.var_q_sl_bulk) - (assign12900_body9_e16459 * locals.var_q_sl_bulk_dn11)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * ((-locals.var_t0_dn12) + ((locals.var_cnst1bulk_dn12 * assign12900_body9_e16456) + (locals.var_cnst1bulk * locals.var_t2__blk352_dn12)))) * locals.var_q_sl_bulk) - (assign12900_body9_e16459 * locals.var_q_sl_bulk_dn12)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)), ((((locals.var_c0bulk * ((-locals.var_t0_dn17) + ((locals.var_cnst1bulk_dn17 * assign12900_body9_e16456) + (locals.var_cnst1bulk * locals.var_t2__blk352_dn17)))) * locals.var_q_sl_bulk) - (assign12900_body9_e16459 * locals.var_q_sl_bulk_dn17)) / (locals.var_q_sl_bulk * locals.var_q_sl_bulk)),)
    } else {
        (locals.var_q_sl_bulk_dpsb, locals.var_q_sl_bulk_dpsb_dn0, locals.var_q_sl_bulk_dpsb_dn2, locals.var_q_sl_bulk_dpsb_dn6, locals.var_q_sl_bulk_dpsb_dn7, locals.var_q_sl_bulk_dpsb_dn10, locals.var_q_sl_bulk_dpsb_dn11, locals.var_q_sl_bulk_dpsb_dn12, locals.var_q_sl_bulk_dpsb_dn17,)
    }
};
            locals.var_q_sl_bulk_dpsb = assign12900_body9_e16463;
            locals.var_q_sl_bulk_dpsb_dn0 = assign12900_body9_e16463_d_n0;
            locals.var_q_sl_bulk_dpsb_dn2 = assign12900_body9_e16463_d_n2;
            locals.var_q_sl_bulk_dpsb_dn6 = assign12900_body9_e16463_d_n6;
            locals.var_q_sl_bulk_dpsb_dn7 = assign12900_body9_e16463_d_n7;
            locals.var_q_sl_bulk_dpsb_dn10 = assign12900_body9_e16463_d_n10;
            locals.var_q_sl_bulk_dpsb_dn11 = assign12900_body9_e16463_d_n11;
            locals.var_q_sl_bulk_dpsb_dn12 = assign12900_body9_e16463_d_n12;
            locals.var_q_sl_bulk_dpsb_dn17 = assign12900_body9_e16463_d_n17;
            let (assign12900_body10_e16479, assign12900_body10_e16479_d_n0, assign12900_body10_e16479_d_n2, assign12900_body10_e16479_d_n6, assign12900_body10_e16479_d_n7, assign12900_body10_e16479_d_n10, assign12900_body10_e16479_d_n11, assign12900_body10_e16479_d_n12, assign12900_body10_e16479_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard387 == 0.0)) && (locals.var_guard388 == 0.0)) {
        let assign12900_body10_e16475: f64 = (-locals.var_cnst0bulk);
        let assign12900_body10_e16477: f64 = (assign12900_body10_e16475 * locals.var_el);
        (assign12900_body10_e16477, (assign12900_body10_e16475 * locals.var_el_dn0), (assign12900_body10_e16475 * locals.var_el_dn2), (assign12900_body10_e16475 * locals.var_el_dn6), (assign12900_body10_e16475 * locals.var_el_dn7), (((-locals.var_cnst0bulk_dn10) * locals.var_el) + (assign12900_body10_e16475 * locals.var_el_dn10)), (assign12900_body10_e16475 * locals.var_el_dn11), (assign12900_body10_e16475 * locals.var_el_dn12), (assign12900_body10_e16475 * locals.var_el_dn17),)
    } else {
        (locals.var_q_sl_bulk, locals.var_q_sl_bulk_dn0, locals.var_q_sl_bulk_dn2, locals.var_q_sl_bulk_dn6, locals.var_q_sl_bulk_dn7, locals.var_q_sl_bulk_dn10, locals.var_q_sl_bulk_dn11, locals.var_q_sl_bulk_dn12, locals.var_q_sl_bulk_dn17,)
    }
};
            locals.var_q_sl_bulk = assign12900_body10_e16479;
            locals.var_q_sl_bulk_dn0 = assign12900_body10_e16479_d_n0;
            locals.var_q_sl_bulk_dn2 = assign12900_body10_e16479_d_n2;
            locals.var_q_sl_bulk_dn6 = assign12900_body10_e16479_d_n6;
            locals.var_q_sl_bulk_dn7 = assign12900_body10_e16479_d_n7;
            locals.var_q_sl_bulk_dn10 = assign12900_body10_e16479_d_n10;
            locals.var_q_sl_bulk_dn11 = assign12900_body10_e16479_d_n11;
            locals.var_q_sl_bulk_dn12 = assign12900_body10_e16479_d_n12;
            locals.var_q_sl_bulk_dn17 = assign12900_body10_e16479_d_n17;
            let (assign12900_body11_e16495, assign12900_body11_e16495_d_n0, assign12900_body11_e16495_d_n2, assign12900_body11_e16495_d_n6, assign12900_body11_e16495_d_n7, assign12900_body11_e16495_d_n10, assign12900_body11_e16495_d_n11, assign12900_body11_e16495_d_n12, assign12900_body11_e16495_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard387 == 0.0)) && (locals.var_guard388 == 0.0)) {
        let assign12900_body11_e16491: f64 = (-locals.var_cnst0bulk);
        let assign12900_body11_e16493: f64 = (assign12900_body11_e16491 * locals.var_beta);
        (assign12900_body11_e16493, 0.0, 0.0, 0.0, 0.0, (((-locals.var_cnst0bulk_dn10) * locals.var_beta) + (assign12900_body11_e16491 * locals.var_beta_dn10)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_sl_bulk_dpsb, locals.var_q_sl_bulk_dpsb_dn0, locals.var_q_sl_bulk_dpsb_dn2, locals.var_q_sl_bulk_dpsb_dn6, locals.var_q_sl_bulk_dpsb_dn7, locals.var_q_sl_bulk_dpsb_dn10, locals.var_q_sl_bulk_dpsb_dn11, locals.var_q_sl_bulk_dpsb_dn12, locals.var_q_sl_bulk_dpsb_dn17,)
    }
};
            locals.var_q_sl_bulk_dpsb = assign12900_body11_e16495;
            locals.var_q_sl_bulk_dpsb_dn0 = assign12900_body11_e16495_d_n0;
            locals.var_q_sl_bulk_dpsb_dn2 = assign12900_body11_e16495_d_n2;
            locals.var_q_sl_bulk_dpsb_dn6 = assign12900_body11_e16495_d_n6;
            locals.var_q_sl_bulk_dpsb_dn7 = assign12900_body11_e16495_d_n7;
            locals.var_q_sl_bulk_dpsb_dn10 = assign12900_body11_e16495_d_n10;
            locals.var_q_sl_bulk_dpsb_dn11 = assign12900_body11_e16495_d_n11;
            locals.var_q_sl_bulk_dpsb_dn12 = assign12900_body11_e16495_d_n12;
            locals.var_q_sl_bulk_dpsb_dn17 = assign12900_body11_e16495_d_n17;
            let (assign12900_body12_e16502, assign12900_body12_e16502_d_n0, assign12900_body12_e16502_d_n2, assign12900_body12_e16502_d_n6, assign12900_body12_e16502_d_n7, assign12900_body12_e16502_d_n10, assign12900_body12_e16502_d_n11, assign12900_body12_e16502_d_n12, assign12900_body12_e16502_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        (locals.var_q_s0_dep_ini, locals.var_q_s0_dep_ini_dn0, locals.var_q_s0_dep_ini_dn2, locals.var_q_s0_dep_ini_dn6, locals.var_q_s0_dep_ini_dn7, locals.var_q_s0_dep_ini_dn10, locals.var_q_s0_dep_ini_dn11, locals.var_q_s0_dep_ini_dn12, locals.var_q_s0_dep_ini_dn17,)
    } else {
        (locals.var_q_sl_dep, locals.var_q_sl_dep_dn0, locals.var_q_sl_dep_dn2, locals.var_q_sl_dep_dn6, locals.var_q_sl_dep_dn7, locals.var_q_sl_dep_dn10, locals.var_q_sl_dep_dn11, locals.var_q_sl_dep_dn12, locals.var_q_sl_dep_dn17,)
    }
};
            locals.var_q_sl_dep = assign12900_body12_e16502;
            locals.var_q_sl_dep_dn0 = assign12900_body12_e16502_d_n0;
            locals.var_q_sl_dep_dn2 = assign12900_body12_e16502_d_n2;
            locals.var_q_sl_dep_dn6 = assign12900_body12_e16502_d_n6;
            locals.var_q_sl_dep_dn7 = assign12900_body12_e16502_d_n7;
            locals.var_q_sl_dep_dn10 = assign12900_body12_e16502_d_n10;
            locals.var_q_sl_dep_dn11 = assign12900_body12_e16502_d_n11;
            locals.var_q_sl_dep_dn12 = assign12900_body12_e16502_d_n12;
            locals.var_q_sl_dep_dn17 = assign12900_body12_e16502_d_n17;
            let (assign12900_body13_e16514, assign12900_body13_e16514_d_n0, assign12900_body13_e16514_d_n2, assign12900_body13_e16514_d_n6, assign12900_body13_e16514_d_n7, assign12900_body13_e16514_d_n10, assign12900_body13_e16514_d_n11, assign12900_body13_e16514_d_n12, assign12900_body13_e16514_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign12900_body13_e16510: f64 = (locals.var_phi_sl_soi - locals.var_vds);
        let assign12900_body13_e16511: f64 = (locals.var_beta * assign12900_body13_e16510);
        let assign12900_body13_e16512: f64 = (assign12900_body13_e16511).exp();
        (assign12900_body13_e16512, (assign12900_body13_e16512 * (locals.var_beta * (locals.var_phi_sl_soi_dn0 - locals.var_vds_dn0))), (assign12900_body13_e16512 * (locals.var_beta * (locals.var_phi_sl_soi_dn2 - locals.var_vds_dn2))), (assign12900_body13_e16512 * (locals.var_beta * (locals.var_phi_sl_soi_dn6 - locals.var_vds_dn6))), (assign12900_body13_e16512 * (locals.var_beta * (locals.var_phi_sl_soi_dn7 - locals.var_vds_dn7))), (assign12900_body13_e16512 * ((locals.var_beta_dn10 * assign12900_body13_e16510) + (locals.var_beta * (locals.var_phi_sl_soi_dn10 - locals.var_vds_dn10)))), (assign12900_body13_e16512 * (locals.var_beta * (locals.var_phi_sl_soi_dn11 - locals.var_vds_dn11))), (assign12900_body13_e16512 * (locals.var_beta * (locals.var_phi_sl_soi_dn12 - locals.var_vds_dn12))), (assign12900_body13_e16512 * (locals.var_beta * (locals.var_phi_sl_soi_dn17 - locals.var_vds_dn17))),)
    } else {
        (locals.var_t5__blk355, locals.var_t5__blk355_dn0, locals.var_t5__blk355_dn2, locals.var_t5__blk355_dn6, locals.var_t5__blk355_dn7, locals.var_t5__blk355_dn10, locals.var_t5__blk355_dn11, locals.var_t5__blk355_dn12, locals.var_t5__blk355_dn17,)
    }
};
            locals.var_t5__blk355 = assign12900_body13_e16514;
            locals.var_t5__blk355_dn0 = assign12900_body13_e16514_d_n0;
            locals.var_t5__blk355_dn2 = assign12900_body13_e16514_d_n2;
            locals.var_t5__blk355_dn6 = assign12900_body13_e16514_d_n6;
            locals.var_t5__blk355_dn7 = assign12900_body13_e16514_d_n7;
            locals.var_t5__blk355_dn10 = assign12900_body13_e16514_d_n10;
            locals.var_t5__blk355_dn11 = assign12900_body13_e16514_d_n11;
            locals.var_t5__blk355_dn12 = assign12900_body13_e16514_d_n12;
            locals.var_t5__blk355_dn17 = assign12900_body13_e16514_d_n17;
            let (assign12900_body14_e16521, assign12900_body14_e16521_d_n0, assign12900_body14_e16521_d_n2, assign12900_body14_e16521_d_n6, assign12900_body14_e16521_d_n7, assign12900_body14_e16521_d_n10, assign12900_body14_e16521_d_n11, assign12900_body14_e16521_d_n12, assign12900_body14_e16521_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3__blk353, locals.var_t3__blk353_dn0, locals.var_t3__blk353_dn2, locals.var_t3__blk353_dn6, locals.var_t3__blk353_dn7, locals.var_t3__blk353_dn10, locals.var_t3__blk353_dn11, locals.var_t3__blk353_dn12, locals.var_t3__blk353_dn17,)
    }
};
            locals.var_t3__blk353 = assign12900_body14_e16521;
            locals.var_t3__blk353_dn0 = assign12900_body14_e16521_d_n0;
            locals.var_t3__blk353_dn2 = assign12900_body14_e16521_d_n2;
            locals.var_t3__blk353_dn6 = assign12900_body14_e16521_d_n6;
            locals.var_t3__blk353_dn7 = assign12900_body14_e16521_d_n7;
            locals.var_t3__blk353_dn10 = assign12900_body14_e16521_d_n10;
            locals.var_t3__blk353_dn11 = assign12900_body14_e16521_d_n11;
            locals.var_t3__blk353_dn12 = assign12900_body14_e16521_d_n12;
            locals.var_t3__blk353_dn17 = assign12900_body14_e16521_d_n17;
            let (assign12900_body15_e16545, assign12900_body15_e16545_d_n0, assign12900_body15_e16545_d_n2, assign12900_body15_e16545_d_n6, assign12900_body15_e16545_d_n7, assign12900_body15_e16545_d_n10, assign12900_body15_e16545_d_n11, assign12900_body15_e16545_d_n12, assign12900_body15_e16545_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign12900_body15_e16528: f64 = (locals.var_q_sl_dep * locals.var_q_sl_dep);
        let assign12900_body15_e16531: f64 = (locals.var_cnst0soi * locals.var_cnst0soi);
        let assign12900_body15_e16532: f64 = (assign12900_body15_e16528 / assign12900_body15_e16531);
        let assign12900_body15_e16535: f64 = (2.0 * locals.var_cnst1soi);
        let assign12900_body15_e16538: f64 = (locals.var_t5__blk355 + locals.var_el);
        let assign12900_body15_e16540: f64 = (assign12900_body15_e16538 - locals.var_t3__blk353);
        let assign12900_body15_e16541: f64 = (assign12900_body15_e16535 * assign12900_body15_e16540);
        let assign12900_body15_e16542: f64 = (assign12900_body15_e16532 + assign12900_body15_e16541);
        let assign12900_body15_e16543: f64 = (assign12900_body15_e16542).sqrt();
        (assign12900_body15_e16543, (((((((locals.var_q_sl_dep_dn0 * locals.var_q_sl_dep) + (locals.var_q_sl_dep * locals.var_q_sl_dep_dn0)) * assign12900_body15_e16531) - (assign12900_body15_e16528 * ((locals.var_cnst0soi_dn0 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn0)))) / (assign12900_body15_e16531 * assign12900_body15_e16531)) + (((2.0 * locals.var_cnst1soi_dn0) * assign12900_body15_e16540) + (assign12900_body15_e16535 * ((locals.var_t5__blk355_dn0 + locals.var_el_dn0) - locals.var_t3__blk353_dn0)))) / (2.0 * assign12900_body15_e16543)), (((((((locals.var_q_sl_dep_dn2 * locals.var_q_sl_dep) + (locals.var_q_sl_dep * locals.var_q_sl_dep_dn2)) * assign12900_body15_e16531) - (assign12900_body15_e16528 * ((locals.var_cnst0soi_dn2 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn2)))) / (assign12900_body15_e16531 * assign12900_body15_e16531)) + (((2.0 * locals.var_cnst1soi_dn2) * assign12900_body15_e16540) + (assign12900_body15_e16535 * ((locals.var_t5__blk355_dn2 + locals.var_el_dn2) - locals.var_t3__blk353_dn2)))) / (2.0 * assign12900_body15_e16543)), (((((((locals.var_q_sl_dep_dn6 * locals.var_q_sl_dep) + (locals.var_q_sl_dep * locals.var_q_sl_dep_dn6)) * assign12900_body15_e16531) - (assign12900_body15_e16528 * ((locals.var_cnst0soi_dn6 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn6)))) / (assign12900_body15_e16531 * assign12900_body15_e16531)) + (((2.0 * locals.var_cnst1soi_dn6) * assign12900_body15_e16540) + (assign12900_body15_e16535 * ((locals.var_t5__blk355_dn6 + locals.var_el_dn6) - locals.var_t3__blk353_dn6)))) / (2.0 * assign12900_body15_e16543)), (((((((locals.var_q_sl_dep_dn7 * locals.var_q_sl_dep) + (locals.var_q_sl_dep * locals.var_q_sl_dep_dn7)) * assign12900_body15_e16531) - (assign12900_body15_e16528 * ((locals.var_cnst0soi_dn7 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn7)))) / (assign12900_body15_e16531 * assign12900_body15_e16531)) + (((2.0 * locals.var_cnst1soi_dn7) * assign12900_body15_e16540) + (assign12900_body15_e16535 * ((locals.var_t5__blk355_dn7 + locals.var_el_dn7) - locals.var_t3__blk353_dn7)))) / (2.0 * assign12900_body15_e16543)), (((((((locals.var_q_sl_dep_dn10 * locals.var_q_sl_dep) + (locals.var_q_sl_dep * locals.var_q_sl_dep_dn10)) * assign12900_body15_e16531) - (assign12900_body15_e16528 * ((locals.var_cnst0soi_dn10 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn10)))) / (assign12900_body15_e16531 * assign12900_body15_e16531)) + (((2.0 * locals.var_cnst1soi_dn10) * assign12900_body15_e16540) + (assign12900_body15_e16535 * ((locals.var_t5__blk355_dn10 + locals.var_el_dn10) - locals.var_t3__blk353_dn10)))) / (2.0 * assign12900_body15_e16543)), (((((((locals.var_q_sl_dep_dn11 * locals.var_q_sl_dep) + (locals.var_q_sl_dep * locals.var_q_sl_dep_dn11)) * assign12900_body15_e16531) - (assign12900_body15_e16528 * ((locals.var_cnst0soi_dn11 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn11)))) / (assign12900_body15_e16531 * assign12900_body15_e16531)) + (((2.0 * locals.var_cnst1soi_dn11) * assign12900_body15_e16540) + (assign12900_body15_e16535 * ((locals.var_t5__blk355_dn11 + locals.var_el_dn11) - locals.var_t3__blk353_dn11)))) / (2.0 * assign12900_body15_e16543)), (((((((locals.var_q_sl_dep_dn12 * locals.var_q_sl_dep) + (locals.var_q_sl_dep * locals.var_q_sl_dep_dn12)) * assign12900_body15_e16531) - (assign12900_body15_e16528 * ((locals.var_cnst0soi_dn12 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn12)))) / (assign12900_body15_e16531 * assign12900_body15_e16531)) + (((2.0 * locals.var_cnst1soi_dn12) * assign12900_body15_e16540) + (assign12900_body15_e16535 * ((locals.var_t5__blk355_dn12 + locals.var_el_dn12) - locals.var_t3__blk353_dn12)))) / (2.0 * assign12900_body15_e16543)), (((((((locals.var_q_sl_dep_dn17 * locals.var_q_sl_dep) + (locals.var_q_sl_dep * locals.var_q_sl_dep_dn17)) * assign12900_body15_e16531) - (assign12900_body15_e16528 * ((locals.var_cnst0soi_dn17 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn17)))) / (assign12900_body15_e16531 * assign12900_body15_e16531)) + (((2.0 * locals.var_cnst1soi_dn17) * assign12900_body15_e16540) + (assign12900_body15_e16535 * ((locals.var_t5__blk355_dn17 + locals.var_el_dn17) - locals.var_t3__blk353_dn17)))) / (2.0 * assign12900_body15_e16543)),)
    } else {
        (locals.var_t4__blk354, locals.var_t4__blk354_dn0, locals.var_t4__blk354_dn2, locals.var_t4__blk354_dn6, locals.var_t4__blk354_dn7, locals.var_t4__blk354_dn10, locals.var_t4__blk354_dn11, locals.var_t4__blk354_dn12, locals.var_t4__blk354_dn17,)
    }
};
            locals.var_t4__blk354 = assign12900_body15_e16545;
            locals.var_t4__blk354_dn0 = assign12900_body15_e16545_d_n0;
            locals.var_t4__blk354_dn2 = assign12900_body15_e16545_d_n2;
            locals.var_t4__blk354_dn6 = assign12900_body15_e16545_d_n6;
            locals.var_t4__blk354_dn7 = assign12900_body15_e16545_d_n7;
            locals.var_t4__blk354_dn10 = assign12900_body15_e16545_d_n10;
            locals.var_t4__blk354_dn11 = assign12900_body15_e16545_d_n11;
            locals.var_t4__blk354_dn12 = assign12900_body15_e16545_d_n12;
            locals.var_t4__blk354_dn17 = assign12900_body15_e16545_d_n17;
            let (assign12900_body16_e16564, assign12900_body16_e16564_d_n0, assign12900_body16_e16564_d_n2, assign12900_body16_e16564_d_n6, assign12900_body16_e16564_d_n7, assign12900_body16_e16564_d_n10, assign12900_body16_e16564_d_n11, assign12900_body16_e16564_d_n12, assign12900_body16_e16564_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign12900_body16_e16552: f64 = (2.0 * locals.var_beta);
        let assign12900_body16_e16554: f64 = (assign12900_body16_e16552 * locals.var_cnst1soi);
        let assign12900_body16_e16557: f64 = (locals.var_t5__blk355 + 1.0);
        let assign12900_body16_e16558: f64 = (assign12900_body16_e16554 * assign12900_body16_e16557);
        let assign12900_body16_e16561: f64 = (2.0 * locals.var_t4__blk354);
        let assign12900_body16_e16562: f64 = (assign12900_body16_e16558 / assign12900_body16_e16561);
        (assign12900_body16_e16562, ((((((assign12900_body16_e16552 * locals.var_cnst1soi_dn0) * assign12900_body16_e16557) + (assign12900_body16_e16554 * locals.var_t5__blk355_dn0)) * assign12900_body16_e16561) - (assign12900_body16_e16558 * (2.0 * locals.var_t4__blk354_dn0))) / (assign12900_body16_e16561 * assign12900_body16_e16561)), ((((((assign12900_body16_e16552 * locals.var_cnst1soi_dn2) * assign12900_body16_e16557) + (assign12900_body16_e16554 * locals.var_t5__blk355_dn2)) * assign12900_body16_e16561) - (assign12900_body16_e16558 * (2.0 * locals.var_t4__blk354_dn2))) / (assign12900_body16_e16561 * assign12900_body16_e16561)), ((((((assign12900_body16_e16552 * locals.var_cnst1soi_dn6) * assign12900_body16_e16557) + (assign12900_body16_e16554 * locals.var_t5__blk355_dn6)) * assign12900_body16_e16561) - (assign12900_body16_e16558 * (2.0 * locals.var_t4__blk354_dn6))) / (assign12900_body16_e16561 * assign12900_body16_e16561)), ((((((assign12900_body16_e16552 * locals.var_cnst1soi_dn7) * assign12900_body16_e16557) + (assign12900_body16_e16554 * locals.var_t5__blk355_dn7)) * assign12900_body16_e16561) - (assign12900_body16_e16558 * (2.0 * locals.var_t4__blk354_dn7))) / (assign12900_body16_e16561 * assign12900_body16_e16561)), ((((((((2.0 * locals.var_beta_dn10) * locals.var_cnst1soi) + (assign12900_body16_e16552 * locals.var_cnst1soi_dn10)) * assign12900_body16_e16557) + (assign12900_body16_e16554 * locals.var_t5__blk355_dn10)) * assign12900_body16_e16561) - (assign12900_body16_e16558 * (2.0 * locals.var_t4__blk354_dn10))) / (assign12900_body16_e16561 * assign12900_body16_e16561)), ((((((assign12900_body16_e16552 * locals.var_cnst1soi_dn11) * assign12900_body16_e16557) + (assign12900_body16_e16554 * locals.var_t5__blk355_dn11)) * assign12900_body16_e16561) - (assign12900_body16_e16558 * (2.0 * locals.var_t4__blk354_dn11))) / (assign12900_body16_e16561 * assign12900_body16_e16561)), ((((((assign12900_body16_e16552 * locals.var_cnst1soi_dn12) * assign12900_body16_e16557) + (assign12900_body16_e16554 * locals.var_t5__blk355_dn12)) * assign12900_body16_e16561) - (assign12900_body16_e16558 * (2.0 * locals.var_t4__blk354_dn12))) / (assign12900_body16_e16561 * assign12900_body16_e16561)), ((((((assign12900_body16_e16552 * locals.var_cnst1soi_dn17) * assign12900_body16_e16557) + (assign12900_body16_e16554 * locals.var_t5__blk355_dn17)) * assign12900_body16_e16561) - (assign12900_body16_e16558 * (2.0 * locals.var_t4__blk354_dn17))) / (assign12900_body16_e16561 * assign12900_body16_e16561)),)
    } else {
        (locals.var_t4_dpss__blk384, locals.var_t4_dpss__blk384_dn0, locals.var_t4_dpss__blk384_dn2, locals.var_t4_dpss__blk384_dn6, locals.var_t4_dpss__blk384_dn7, locals.var_t4_dpss__blk384_dn10, locals.var_t4_dpss__blk384_dn11, locals.var_t4_dpss__blk384_dn12, locals.var_t4_dpss__blk384_dn17,)
    }
};
            locals.var_t4_dpss__blk384 = assign12900_body16_e16564;
            locals.var_t4_dpss__blk384_dn0 = assign12900_body16_e16564_d_n0;
            locals.var_t4_dpss__blk384_dn2 = assign12900_body16_e16564_d_n2;
            locals.var_t4_dpss__blk384_dn6 = assign12900_body16_e16564_d_n6;
            locals.var_t4_dpss__blk384_dn7 = assign12900_body16_e16564_d_n7;
            locals.var_t4_dpss__blk384_dn10 = assign12900_body16_e16564_d_n10;
            locals.var_t4_dpss__blk384_dn11 = assign12900_body16_e16564_d_n11;
            locals.var_t4_dpss__blk384_dn12 = assign12900_body16_e16564_d_n12;
            locals.var_t4_dpss__blk384_dn17 = assign12900_body16_e16564_d_n17;
            let (assign12900_body17_e16576, assign12900_body17_e16576_d_n0, assign12900_body17_e16576_d_n2, assign12900_body17_e16576_d_n6, assign12900_body17_e16576_d_n7, assign12900_body17_e16576_d_n10, assign12900_body17_e16576_d_n11, assign12900_body17_e16576_d_n12, assign12900_body17_e16576_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign12900_body17_e16570: f64 = (-locals.var_cnst0soi);
        let assign12900_body17_e16572: f64 = (assign12900_body17_e16570 * locals.var_t4__blk354);
        let assign12900_body17_e16574: f64 = (assign12900_body17_e16572 - locals.var_q_sl_dep);
        (assign12900_body17_e16574, ((((-locals.var_cnst0soi_dn0) * locals.var_t4__blk354) + (assign12900_body17_e16570 * locals.var_t4__blk354_dn0)) - locals.var_q_sl_dep_dn0), ((((-locals.var_cnst0soi_dn2) * locals.var_t4__blk354) + (assign12900_body17_e16570 * locals.var_t4__blk354_dn2)) - locals.var_q_sl_dep_dn2), ((((-locals.var_cnst0soi_dn6) * locals.var_t4__blk354) + (assign12900_body17_e16570 * locals.var_t4__blk354_dn6)) - locals.var_q_sl_dep_dn6), ((((-locals.var_cnst0soi_dn7) * locals.var_t4__blk354) + (assign12900_body17_e16570 * locals.var_t4__blk354_dn7)) - locals.var_q_sl_dep_dn7), ((((-locals.var_cnst0soi_dn10) * locals.var_t4__blk354) + (assign12900_body17_e16570 * locals.var_t4__blk354_dn10)) - locals.var_q_sl_dep_dn10), ((((-locals.var_cnst0soi_dn11) * locals.var_t4__blk354) + (assign12900_body17_e16570 * locals.var_t4__blk354_dn11)) - locals.var_q_sl_dep_dn11), ((((-locals.var_cnst0soi_dn12) * locals.var_t4__blk354) + (assign12900_body17_e16570 * locals.var_t4__blk354_dn12)) - locals.var_q_sl_dep_dn12), ((((-locals.var_cnst0soi_dn17) * locals.var_t4__blk354) + (assign12900_body17_e16570 * locals.var_t4__blk354_dn17)) - locals.var_q_sl_dep_dn17),)
    } else {
        (locals.var_q_nl, locals.var_q_nl_dn0, locals.var_q_nl_dn2, locals.var_q_nl_dn6, locals.var_q_nl_dn7, locals.var_q_nl_dn10, locals.var_q_nl_dn11, locals.var_q_nl_dn12, locals.var_q_nl_dn17,)
    }
};
            locals.var_q_nl = assign12900_body17_e16576;
            locals.var_q_nl_dn0 = assign12900_body17_e16576_d_n0;
            locals.var_q_nl_dn2 = assign12900_body17_e16576_d_n2;
            locals.var_q_nl_dn6 = assign12900_body17_e16576_d_n6;
            locals.var_q_nl_dn7 = assign12900_body17_e16576_d_n7;
            locals.var_q_nl_dn10 = assign12900_body17_e16576_d_n10;
            locals.var_q_nl_dn11 = assign12900_body17_e16576_d_n11;
            locals.var_q_nl_dn12 = assign12900_body17_e16576_d_n12;
            locals.var_q_nl_dn17 = assign12900_body17_e16576_d_n17;
            let (assign12900_body18_e16586, assign12900_body18_e16586_d_n0, assign12900_body18_e16586_d_n2, assign12900_body18_e16586_d_n6, assign12900_body18_e16586_d_n7, assign12900_body18_e16586_d_n10, assign12900_body18_e16586_d_n11, assign12900_body18_e16586_d_n12, assign12900_body18_e16586_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign12900_body18_e16582: f64 = (-locals.var_cnst0soi);
        let assign12900_body18_e16584: f64 = (assign12900_body18_e16582 * locals.var_t4_dpss__blk384);
        (assign12900_body18_e16584, (((-locals.var_cnst0soi_dn0) * locals.var_t4_dpss__blk384) + (assign12900_body18_e16582 * locals.var_t4_dpss__blk384_dn0)), (((-locals.var_cnst0soi_dn2) * locals.var_t4_dpss__blk384) + (assign12900_body18_e16582 * locals.var_t4_dpss__blk384_dn2)), (((-locals.var_cnst0soi_dn6) * locals.var_t4_dpss__blk384) + (assign12900_body18_e16582 * locals.var_t4_dpss__blk384_dn6)), (((-locals.var_cnst0soi_dn7) * locals.var_t4_dpss__blk384) + (assign12900_body18_e16582 * locals.var_t4_dpss__blk384_dn7)), (((-locals.var_cnst0soi_dn10) * locals.var_t4_dpss__blk384) + (assign12900_body18_e16582 * locals.var_t4_dpss__blk384_dn10)), (((-locals.var_cnst0soi_dn11) * locals.var_t4_dpss__blk384) + (assign12900_body18_e16582 * locals.var_t4_dpss__blk384_dn11)), (((-locals.var_cnst0soi_dn12) * locals.var_t4_dpss__blk384) + (assign12900_body18_e16582 * locals.var_t4_dpss__blk384_dn12)), (((-locals.var_cnst0soi_dn17) * locals.var_t4_dpss__blk384) + (assign12900_body18_e16582 * locals.var_t4_dpss__blk384_dn17)),)
    } else {
        (locals.var_q_nl_dpss, locals.var_q_nl_dpss_dn0, locals.var_q_nl_dpss_dn2, locals.var_q_nl_dpss_dn6, locals.var_q_nl_dpss_dn7, locals.var_q_nl_dpss_dn10, locals.var_q_nl_dpss_dn11, locals.var_q_nl_dpss_dn12, locals.var_q_nl_dpss_dn17,)
    }
};
            locals.var_q_nl_dpss = assign12900_body18_e16586;
            locals.var_q_nl_dpss_dn0 = assign12900_body18_e16586_d_n0;
            locals.var_q_nl_dpss_dn2 = assign12900_body18_e16586_d_n2;
            locals.var_q_nl_dpss_dn6 = assign12900_body18_e16586_d_n6;
            locals.var_q_nl_dpss_dn7 = assign12900_body18_e16586_d_n7;
            locals.var_q_nl_dpss_dn10 = assign12900_body18_e16586_d_n10;
            locals.var_q_nl_dpss_dn11 = assign12900_body18_e16586_d_n11;
            locals.var_q_nl_dpss_dn12 = assign12900_body18_e16586_d_n12;
            locals.var_q_nl_dpss_dn17 = assign12900_body18_e16586_d_n17;
            let (assign12900_body19_e16597, assign12900_body19_e16597_d_n0, assign12900_body19_e16597_d_n2, assign12900_body19_e16597_d_n6, assign12900_body19_e16597_d_n7, assign12900_body19_e16597_d_n10, assign12900_body19_e16597_d_n11, assign12900_body19_e16597_d_n12, assign12900_body19_e16597_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign12900_body19_e16593: f64 = (locals.var_phi_bl_soi - locals.var_phi_sl_soi);
        let assign12900_body19_e16595: f64 = (assign12900_body19_e16593 / locals.var_qdepb_dlt);
        (assign12900_body19_e16595, ((locals.var_phi_bl_soi_dn0 - locals.var_phi_sl_soi_dn0) / locals.var_qdepb_dlt), ((locals.var_phi_bl_soi_dn2 - locals.var_phi_sl_soi_dn2) / locals.var_qdepb_dlt), ((locals.var_phi_bl_soi_dn6 - locals.var_phi_sl_soi_dn6) / locals.var_qdepb_dlt), ((locals.var_phi_bl_soi_dn7 - locals.var_phi_sl_soi_dn7) / locals.var_qdepb_dlt), ((locals.var_phi_bl_soi_dn10 - locals.var_phi_sl_soi_dn10) / locals.var_qdepb_dlt), ((locals.var_phi_bl_soi_dn11 - locals.var_phi_sl_soi_dn11) / locals.var_qdepb_dlt), ((locals.var_phi_bl_soi_dn12 - locals.var_phi_sl_soi_dn12) / locals.var_qdepb_dlt), ((locals.var_phi_bl_soi_dn17 - locals.var_phi_sl_soi_dn17) / locals.var_qdepb_dlt),)
    } else {
        (locals.var_t1__blk351, locals.var_t1__blk351_dn0, locals.var_t1__blk351_dn2, locals.var_t1__blk351_dn6, locals.var_t1__blk351_dn7, locals.var_t1__blk351_dn10, locals.var_t1__blk351_dn11, locals.var_t1__blk351_dn12, locals.var_t1__blk351_dn17,)
    }
};
            locals.var_t1__blk351 = assign12900_body19_e16597;
            locals.var_t1__blk351_dn0 = assign12900_body19_e16597_d_n0;
            locals.var_t1__blk351_dn2 = assign12900_body19_e16597_d_n2;
            locals.var_t1__blk351_dn6 = assign12900_body19_e16597_d_n6;
            locals.var_t1__blk351_dn7 = assign12900_body19_e16597_d_n7;
            locals.var_t1__blk351_dn10 = assign12900_body19_e16597_d_n10;
            locals.var_t1__blk351_dn11 = assign12900_body19_e16597_d_n11;
            locals.var_t1__blk351_dn12 = assign12900_body19_e16597_d_n12;
            locals.var_t1__blk351_dn17 = assign12900_body19_e16597_d_n17;
            let (assign12900_body20_e16606, assign12900_body20_e16606_d_n0, assign12900_body20_e16606_d_n2, assign12900_body20_e16606_d_n6, assign12900_body20_e16606_d_n7, assign12900_body20_e16606_d_n10, assign12900_body20_e16606_d_n11, assign12900_body20_e16606_d_n12, assign12900_body20_e16606_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign12900_body20_e16604: f64 = (locals.var_beta * locals.var_t1__blk351);
        (assign12900_body20_e16604, (locals.var_beta * locals.var_t1__blk351_dn0), (locals.var_beta * locals.var_t1__blk351_dn2), (locals.var_beta * locals.var_t1__blk351_dn6), (locals.var_beta * locals.var_t1__blk351_dn7), ((locals.var_beta_dn10 * locals.var_t1__blk351) + (locals.var_beta * locals.var_t1__blk351_dn10)), (locals.var_beta * locals.var_t1__blk351_dn11), (locals.var_beta * locals.var_t1__blk351_dn12), (locals.var_beta * locals.var_t1__blk351_dn17),)
    } else {
        (locals.var_el, locals.var_el_dn0, locals.var_el_dn2, locals.var_el_dn6, locals.var_el_dn7, locals.var_el_dn10, locals.var_el_dn11, locals.var_el_dn12, locals.var_el_dn17,)
    }
};
            locals.var_el = assign12900_body20_e16606;
            locals.var_el_dn0 = assign12900_body20_e16606_d_n0;
            locals.var_el_dn2 = assign12900_body20_e16606_d_n2;
            locals.var_el_dn6 = assign12900_body20_e16606_d_n6;
            locals.var_el_dn7 = assign12900_body20_e16606_d_n7;
            locals.var_el_dn10 = assign12900_body20_e16606_d_n10;
            locals.var_el_dn11 = assign12900_body20_e16606_d_n11;
            locals.var_el_dn12 = assign12900_body20_e16606_d_n12;
            locals.var_el_dn17 = assign12900_body20_e16606_d_n17;
            let assign12900_body21_e16608: f64 = (-locals.var_el);
            let assign12900_body21_e16610: f64 = if assign12900_body21_e16608 >= 500.0 { 1.0 } else { 0.0 };
            locals.var_guard389 = assign12900_body21_e16610;
            let (assign12900_body22_e16626, assign12900_body22_e16626_d_n0, assign12900_body22_e16626_d_n2, assign12900_body22_e16626_d_n6, assign12900_body22_e16626_d_n7, assign12900_body22_e16626_d_n10, assign12900_body22_e16626_d_n11, assign12900_body22_e16626_d_n12, assign12900_body22_e16626_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard389 != 0.0)) {
        let assign12900_body22_e16620: f64 = (-locals.var_el);
        let assign12900_body22_e16621: f64 = (1.0 + assign12900_body22_e16620);
        let assign12900_body22_e16623: f64 = (assign12900_body22_e16621 - 500.0);
        let assign12900_body22_e16624: f64 = (1.403592217853e217 * assign12900_body22_e16623);
        (assign12900_body22_e16624, (1.403592217853e217 * (-locals.var_el_dn0)), (1.403592217853e217 * (-locals.var_el_dn2)), (1.403592217853e217 * (-locals.var_el_dn6)), (1.403592217853e217 * (-locals.var_el_dn7)), (1.403592217853e217 * (-locals.var_el_dn10)), (1.403592217853e217 * (-locals.var_el_dn11)), (1.403592217853e217 * (-locals.var_el_dn12)), (1.403592217853e217 * (-locals.var_el_dn17)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign12900_body22_e16626;
            locals.var_t0_dn0 = assign12900_body22_e16626_d_n0;
            locals.var_t0_dn2 = assign12900_body22_e16626_d_n2;
            locals.var_t0_dn6 = assign12900_body22_e16626_d_n6;
            locals.var_t0_dn7 = assign12900_body22_e16626_d_n7;
            locals.var_t0_dn10 = assign12900_body22_e16626_d_n10;
            locals.var_t0_dn11 = assign12900_body22_e16626_d_n11;
            locals.var_t0_dn12 = assign12900_body22_e16626_d_n12;
            locals.var_t0_dn17 = assign12900_body22_e16626_d_n17;
            let (assign12900_body23_e16635, assign12900_body23_e16635_d_n0, assign12900_body23_e16635_d_n2, assign12900_body23_e16635_d_n6, assign12900_body23_e16635_d_n7, assign12900_body23_e16635_d_n10, assign12900_body23_e16635_d_n11, assign12900_body23_e16635_d_n12, assign12900_body23_e16635_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard389 != 0.0)) {
        (1.403592217853e217, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17,)
    }
};
            locals.var_t6 = assign12900_body23_e16635;
            locals.var_t6_dn0 = assign12900_body23_e16635_d_n0;
            locals.var_t6_dn2 = assign12900_body23_e16635_d_n2;
            locals.var_t6_dn6 = assign12900_body23_e16635_d_n6;
            locals.var_t6_dn7 = assign12900_body23_e16635_d_n7;
            locals.var_t6_dn10 = assign12900_body23_e16635_d_n10;
            locals.var_t6_dn11 = assign12900_body23_e16635_d_n11;
            locals.var_t6_dn12 = assign12900_body23_e16635_d_n12;
            locals.var_t6_dn17 = assign12900_body23_e16635_d_n17;
            let (assign12900_body24_e16646, assign12900_body24_e16646_d_n0, assign12900_body24_e16646_d_n2, assign12900_body24_e16646_d_n6, assign12900_body24_e16646_d_n7, assign12900_body24_e16646_d_n10, assign12900_body24_e16646_d_n11, assign12900_body24_e16646_d_n12, assign12900_body24_e16646_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard389 == 0.0)) {
        let assign12900_body24_e16644: f64 = (-locals.var_el);
        (assign12900_body24_e16644, (-locals.var_el_dn0), (-locals.var_el_dn2), (-locals.var_el_dn6), (-locals.var_el_dn7), (-locals.var_el_dn10), (-locals.var_el_dn11), (-locals.var_el_dn12), (-locals.var_el_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
            locals.var_tmf1 = assign12900_body24_e16646;
            locals.var_tmf1_dn0 = assign12900_body24_e16646_d_n0;
            locals.var_tmf1_dn2 = assign12900_body24_e16646_d_n2;
            locals.var_tmf1_dn6 = assign12900_body24_e16646_d_n6;
            locals.var_tmf1_dn7 = assign12900_body24_e16646_d_n7;
            locals.var_tmf1_dn10 = assign12900_body24_e16646_d_n10;
            locals.var_tmf1_dn11 = assign12900_body24_e16646_d_n11;
            locals.var_tmf1_dn12 = assign12900_body24_e16646_d_n12;
            locals.var_tmf1_dn17 = assign12900_body24_e16646_d_n17;
            let (assign12900_body25_e16656, assign12900_body25_e16656_d_n0, assign12900_body25_e16656_d_n2, assign12900_body25_e16656_d_n6, assign12900_body25_e16656_d_n7, assign12900_body25_e16656_d_n10, assign12900_body25_e16656_d_n11, assign12900_body25_e16656_d_n12, assign12900_body25_e16656_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard389 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign12900_body25_e16656;
            locals.var_t0_dn0 = assign12900_body25_e16656_d_n0;
            locals.var_t0_dn2 = assign12900_body25_e16656_d_n2;
            locals.var_t0_dn6 = assign12900_body25_e16656_d_n6;
            locals.var_t0_dn7 = assign12900_body25_e16656_d_n7;
            locals.var_t0_dn10 = assign12900_body25_e16656_d_n10;
            locals.var_t0_dn11 = assign12900_body25_e16656_d_n11;
            locals.var_t0_dn12 = assign12900_body25_e16656_d_n12;
            locals.var_t0_dn17 = assign12900_body25_e16656_d_n17;
            let mut assign12900_body26_loop_guard: usize = 0;
            while {
                let assign12900_body26_cond_e16667: f64 = if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard389 == 0.0)) && (locals.var_tmf1 >= 60.0)) { 1.0 } else { 0.0 };
                assign12900_body26_cond_e16667 != 0.0
            } {
                assign12900_body26_loop_guard += 1;
                assert!(assign12900_body26_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                let (assign12900_body26_body0_e16679, assign12900_body26_body0_e16679_d_n0, assign12900_body26_body0_e16679_d_n2, assign12900_body26_body0_e16679_d_n6, assign12900_body26_body0_e16679_d_n7, assign12900_body26_body0_e16679_d_n10, assign12900_body26_body0_e16679_d_n11, assign12900_body26_body0_e16679_d_n12, assign12900_body26_body0_e16679_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard389 == 0.0)) {
        let assign12900_body26_body0_e16677: f64 = (locals.var_t0 * 1.14200738981568e26);
        (assign12900_body26_body0_e16677, (locals.var_t0_dn0 * 1.14200738981568e26), (locals.var_t0_dn2 * 1.14200738981568e26), (locals.var_t0_dn6 * 1.14200738981568e26), (locals.var_t0_dn7 * 1.14200738981568e26), (locals.var_t0_dn10 * 1.14200738981568e26), (locals.var_t0_dn11 * 1.14200738981568e26), (locals.var_t0_dn12 * 1.14200738981568e26), (locals.var_t0_dn17 * 1.14200738981568e26),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
                locals.var_t0 = assign12900_body26_body0_e16679;
                locals.var_t0_dn0 = assign12900_body26_body0_e16679_d_n0;
                locals.var_t0_dn2 = assign12900_body26_body0_e16679_d_n2;
                locals.var_t0_dn6 = assign12900_body26_body0_e16679_d_n6;
                locals.var_t0_dn7 = assign12900_body26_body0_e16679_d_n7;
                locals.var_t0_dn10 = assign12900_body26_body0_e16679_d_n10;
                locals.var_t0_dn11 = assign12900_body26_body0_e16679_d_n11;
                locals.var_t0_dn12 = assign12900_body26_body0_e16679_d_n12;
                locals.var_t0_dn17 = assign12900_body26_body0_e16679_d_n17;
                let (assign12900_body26_body1_e16691, assign12900_body26_body1_e16691_d_n0, assign12900_body26_body1_e16691_d_n2, assign12900_body26_body1_e16691_d_n6, assign12900_body26_body1_e16691_d_n7, assign12900_body26_body1_e16691_d_n10, assign12900_body26_body1_e16691_d_n11, assign12900_body26_body1_e16691_d_n12, assign12900_body26_body1_e16691_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard389 == 0.0)) {
        let assign12900_body26_body1_e16689: f64 = (locals.var_tmf1 - 60.0);
        (assign12900_body26_body1_e16689, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
                locals.var_tmf1 = assign12900_body26_body1_e16691;
                locals.var_tmf1_dn0 = assign12900_body26_body1_e16691_d_n0;
                locals.var_tmf1_dn2 = assign12900_body26_body1_e16691_d_n2;
                locals.var_tmf1_dn6 = assign12900_body26_body1_e16691_d_n6;
                locals.var_tmf1_dn7 = assign12900_body26_body1_e16691_d_n7;
                locals.var_tmf1_dn10 = assign12900_body26_body1_e16691_d_n10;
                locals.var_tmf1_dn11 = assign12900_body26_body1_e16691_d_n11;
                locals.var_tmf1_dn12 = assign12900_body26_body1_e16691_d_n12;
                locals.var_tmf1_dn17 = assign12900_body26_body1_e16691_d_n17;
            }
            let (assign12900_body27_e16704, assign12900_body27_e16704_d_n0, assign12900_body27_e16704_d_n2, assign12900_body27_e16704_d_n6, assign12900_body27_e16704_d_n7, assign12900_body27_e16704_d_n10, assign12900_body27_e16704_d_n11, assign12900_body27_e16704_d_n12, assign12900_body27_e16704_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard389 == 0.0)) {
        let assign12900_body27_e16701: f64 = (locals.var_tmf1).exp();
        let assign12900_body27_e16702: f64 = (locals.var_t0 * assign12900_body27_e16701);
        (assign12900_body27_e16702, ((locals.var_t0_dn0 * assign12900_body27_e16701) + (locals.var_t0 * (assign12900_body27_e16701 * locals.var_tmf1_dn0))), ((locals.var_t0_dn2 * assign12900_body27_e16701) + (locals.var_t0 * (assign12900_body27_e16701 * locals.var_tmf1_dn2))), ((locals.var_t0_dn6 * assign12900_body27_e16701) + (locals.var_t0 * (assign12900_body27_e16701 * locals.var_tmf1_dn6))), ((locals.var_t0_dn7 * assign12900_body27_e16701) + (locals.var_t0 * (assign12900_body27_e16701 * locals.var_tmf1_dn7))), ((locals.var_t0_dn10 * assign12900_body27_e16701) + (locals.var_t0 * (assign12900_body27_e16701 * locals.var_tmf1_dn10))), ((locals.var_t0_dn11 * assign12900_body27_e16701) + (locals.var_t0 * (assign12900_body27_e16701 * locals.var_tmf1_dn11))), ((locals.var_t0_dn12 * assign12900_body27_e16701) + (locals.var_t0 * (assign12900_body27_e16701 * locals.var_tmf1_dn12))), ((locals.var_t0_dn17 * assign12900_body27_e16701) + (locals.var_t0 * (assign12900_body27_e16701 * locals.var_tmf1_dn17))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign12900_body27_e16704;
            locals.var_t0_dn0 = assign12900_body27_e16704_d_n0;
            locals.var_t0_dn2 = assign12900_body27_e16704_d_n2;
            locals.var_t0_dn6 = assign12900_body27_e16704_d_n6;
            locals.var_t0_dn7 = assign12900_body27_e16704_d_n7;
            locals.var_t0_dn10 = assign12900_body27_e16704_d_n10;
            locals.var_t0_dn11 = assign12900_body27_e16704_d_n11;
            locals.var_t0_dn12 = assign12900_body27_e16704_d_n12;
            locals.var_t0_dn17 = assign12900_body27_e16704_d_n17;
            let (assign12900_body28_e16714, assign12900_body28_e16714_d_n0, assign12900_body28_e16714_d_n2, assign12900_body28_e16714_d_n6, assign12900_body28_e16714_d_n7, assign12900_body28_e16714_d_n10, assign12900_body28_e16714_d_n11, assign12900_body28_e16714_d_n12, assign12900_body28_e16714_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard389 == 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn17,)
    }
};
            locals.var_t6 = assign12900_body28_e16714;
            locals.var_t6_dn0 = assign12900_body28_e16714_d_n0;
            locals.var_t6_dn2 = assign12900_body28_e16714_d_n2;
            locals.var_t6_dn6 = assign12900_body28_e16714_d_n6;
            locals.var_t6_dn7 = assign12900_body28_e16714_d_n7;
            locals.var_t6_dn10 = assign12900_body28_e16714_d_n10;
            locals.var_t6_dn11 = assign12900_body28_e16714_d_n11;
            locals.var_t6_dn12 = assign12900_body28_e16714_d_n12;
            locals.var_t6_dn17 = assign12900_body28_e16714_d_n17;
            let (assign12900_body29_e16726, assign12900_body29_e16726_d_n0, assign12900_body29_e16726_d_n2, assign12900_body29_e16726_d_n6, assign12900_body29_e16726_d_n7, assign12900_body29_e16726_d_n10, assign12900_body29_e16726_d_n11, assign12900_body29_e16726_d_n12, assign12900_body29_e16726_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign12900_body29_e16721: f64 = (locals.var_t0 + locals.var_el);
        let assign12900_body29_e16723: f64 = (assign12900_body29_e16721 - 1.0);
        let assign12900_body29_e16724: f64 = (assign12900_body29_e16723).sqrt();
        (assign12900_body29_e16724, ((locals.var_t0_dn0 + locals.var_el_dn0) / (2.0 * assign12900_body29_e16724)), ((locals.var_t0_dn2 + locals.var_el_dn2) / (2.0 * assign12900_body29_e16724)), ((locals.var_t0_dn6 + locals.var_el_dn6) / (2.0 * assign12900_body29_e16724)), ((locals.var_t0_dn7 + locals.var_el_dn7) / (2.0 * assign12900_body29_e16724)), ((locals.var_t0_dn10 + locals.var_el_dn10) / (2.0 * assign12900_body29_e16724)), ((locals.var_t0_dn11 + locals.var_el_dn11) / (2.0 * assign12900_body29_e16724)), ((locals.var_t0_dn12 + locals.var_el_dn12) / (2.0 * assign12900_body29_e16724)), ((locals.var_t0_dn17 + locals.var_el_dn17) / (2.0 * assign12900_body29_e16724)),)
    } else {
        (locals.var_t2__blk352, locals.var_t2__blk352_dn0, locals.var_t2__blk352_dn2, locals.var_t2__blk352_dn6, locals.var_t2__blk352_dn7, locals.var_t2__blk352_dn10, locals.var_t2__blk352_dn11, locals.var_t2__blk352_dn12, locals.var_t2__blk352_dn17,)
    }
};
            locals.var_t2__blk352 = assign12900_body29_e16726;
            locals.var_t2__blk352_dn0 = assign12900_body29_e16726_d_n0;
            locals.var_t2__blk352_dn2 = assign12900_body29_e16726_d_n2;
            locals.var_t2__blk352_dn6 = assign12900_body29_e16726_d_n6;
            locals.var_t2__blk352_dn7 = assign12900_body29_e16726_d_n7;
            locals.var_t2__blk352_dn10 = assign12900_body29_e16726_d_n10;
            locals.var_t2__blk352_dn11 = assign12900_body29_e16726_d_n11;
            locals.var_t2__blk352_dn12 = assign12900_body29_e16726_d_n12;
            locals.var_t2__blk352_dn17 = assign12900_body29_e16726_d_n17;
            let assign12900_body30_e16729: f64 = (-1e-9);
            let assign12900_body30_e16730: f64 = if locals.var_t1__blk351 < assign12900_body30_e16729 { 1.0 } else { 0.0 };
            locals.var_guard390 = assign12900_body30_e16730;
            let (assign12900_body31_e16741, assign12900_body31_e16741_d_n0, assign12900_body31_e16741_d_n2, assign12900_body31_e16741_d_n6, assign12900_body31_e16741_d_n7, assign12900_body31_e16741_d_n10, assign12900_body31_e16741_d_n11, assign12900_body31_e16741_d_n12, assign12900_body31_e16741_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard390 != 0.0)) {
        let assign12900_body31_e16739: f64 = (locals.var_cnst0soi * locals.var_t2__blk352);
        (assign12900_body31_e16739, ((locals.var_cnst0soi_dn0 * locals.var_t2__blk352) + (locals.var_cnst0soi * locals.var_t2__blk352_dn0)), ((locals.var_cnst0soi_dn2 * locals.var_t2__blk352) + (locals.var_cnst0soi * locals.var_t2__blk352_dn2)), ((locals.var_cnst0soi_dn6 * locals.var_t2__blk352) + (locals.var_cnst0soi * locals.var_t2__blk352_dn6)), ((locals.var_cnst0soi_dn7 * locals.var_t2__blk352) + (locals.var_cnst0soi * locals.var_t2__blk352_dn7)), ((locals.var_cnst0soi_dn10 * locals.var_t2__blk352) + (locals.var_cnst0soi * locals.var_t2__blk352_dn10)), ((locals.var_cnst0soi_dn11 * locals.var_t2__blk352) + (locals.var_cnst0soi * locals.var_t2__blk352_dn11)), ((locals.var_cnst0soi_dn12 * locals.var_t2__blk352) + (locals.var_cnst0soi * locals.var_t2__blk352_dn12)), ((locals.var_cnst0soi_dn17 * locals.var_t2__blk352) + (locals.var_cnst0soi * locals.var_t2__blk352_dn17)),)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn12, locals.var_q_bl_dep_dn17,)
    }
};
            locals.var_q_bl_dep = assign12900_body31_e16741;
            locals.var_q_bl_dep_dn0 = assign12900_body31_e16741_d_n0;
            locals.var_q_bl_dep_dn2 = assign12900_body31_e16741_d_n2;
            locals.var_q_bl_dep_dn6 = assign12900_body31_e16741_d_n6;
            locals.var_q_bl_dep_dn7 = assign12900_body31_e16741_d_n7;
            locals.var_q_bl_dep_dn10 = assign12900_body31_e16741_d_n10;
            locals.var_q_bl_dep_dn11 = assign12900_body31_e16741_d_n11;
            locals.var_q_bl_dep_dn12 = assign12900_body31_e16741_d_n12;
            locals.var_q_bl_dep_dn17 = assign12900_body31_e16741_d_n17;
            let (assign12900_body32_e16763, assign12900_body32_e16763_d_n0, assign12900_body32_e16763_d_n2, assign12900_body32_e16763_d_n6, assign12900_body32_e16763_d_n7, assign12900_body32_e16763_d_n10, assign12900_body32_e16763_d_n11, assign12900_body32_e16763_d_n12, assign12900_body32_e16763_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard390 != 0.0)) {
        let assign12900_body32_e16750: f64 = (locals.var_cnst0soi * locals.var_beta);
        let assign12900_body32_e16752: f64 = (-locals.var_t6);
        let assign12900_body32_e16754: f64 = (assign12900_body32_e16752 + 1.0);
        let assign12900_body32_e16755: f64 = (assign12900_body32_e16750 * assign12900_body32_e16754);
        let assign12900_body32_e16758: f64 = (2.0 * locals.var_t2__blk352);
        let assign12900_body32_e16759: f64 = (assign12900_body32_e16755 / assign12900_body32_e16758);
        let assign12900_body32_e16761: f64 = (assign12900_body32_e16759 / locals.var_qdepb_dlt);
        (assign12900_body32_e16761, (((((((locals.var_cnst0soi_dn0 * locals.var_beta) * assign12900_body32_e16754) + (assign12900_body32_e16750 * (-locals.var_t6_dn0))) * assign12900_body32_e16758) - (assign12900_body32_e16755 * (2.0 * locals.var_t2__blk352_dn0))) / (assign12900_body32_e16758 * assign12900_body32_e16758)) / locals.var_qdepb_dlt), (((((((locals.var_cnst0soi_dn2 * locals.var_beta) * assign12900_body32_e16754) + (assign12900_body32_e16750 * (-locals.var_t6_dn2))) * assign12900_body32_e16758) - (assign12900_body32_e16755 * (2.0 * locals.var_t2__blk352_dn2))) / (assign12900_body32_e16758 * assign12900_body32_e16758)) / locals.var_qdepb_dlt), (((((((locals.var_cnst0soi_dn6 * locals.var_beta) * assign12900_body32_e16754) + (assign12900_body32_e16750 * (-locals.var_t6_dn6))) * assign12900_body32_e16758) - (assign12900_body32_e16755 * (2.0 * locals.var_t2__blk352_dn6))) / (assign12900_body32_e16758 * assign12900_body32_e16758)) / locals.var_qdepb_dlt), (((((((locals.var_cnst0soi_dn7 * locals.var_beta) * assign12900_body32_e16754) + (assign12900_body32_e16750 * (-locals.var_t6_dn7))) * assign12900_body32_e16758) - (assign12900_body32_e16755 * (2.0 * locals.var_t2__blk352_dn7))) / (assign12900_body32_e16758 * assign12900_body32_e16758)) / locals.var_qdepb_dlt), ((((((((locals.var_cnst0soi_dn10 * locals.var_beta) + (locals.var_cnst0soi * locals.var_beta_dn10)) * assign12900_body32_e16754) + (assign12900_body32_e16750 * (-locals.var_t6_dn10))) * assign12900_body32_e16758) - (assign12900_body32_e16755 * (2.0 * locals.var_t2__blk352_dn10))) / (assign12900_body32_e16758 * assign12900_body32_e16758)) / locals.var_qdepb_dlt), (((((((locals.var_cnst0soi_dn11 * locals.var_beta) * assign12900_body32_e16754) + (assign12900_body32_e16750 * (-locals.var_t6_dn11))) * assign12900_body32_e16758) - (assign12900_body32_e16755 * (2.0 * locals.var_t2__blk352_dn11))) / (assign12900_body32_e16758 * assign12900_body32_e16758)) / locals.var_qdepb_dlt), (((((((locals.var_cnst0soi_dn12 * locals.var_beta) * assign12900_body32_e16754) + (assign12900_body32_e16750 * (-locals.var_t6_dn12))) * assign12900_body32_e16758) - (assign12900_body32_e16755 * (2.0 * locals.var_t2__blk352_dn12))) / (assign12900_body32_e16758 * assign12900_body32_e16758)) / locals.var_qdepb_dlt), (((((((locals.var_cnst0soi_dn17 * locals.var_beta) * assign12900_body32_e16754) + (assign12900_body32_e16750 * (-locals.var_t6_dn17))) * assign12900_body32_e16758) - (assign12900_body32_e16755 * (2.0 * locals.var_t2__blk352_dn17))) / (assign12900_body32_e16758 * assign12900_body32_e16758)) / locals.var_qdepb_dlt),)
    } else {
        (locals.var_q_bl_dep_dpbs, locals.var_q_bl_dep_dpbs_dn0, locals.var_q_bl_dep_dpbs_dn2, locals.var_q_bl_dep_dpbs_dn6, locals.var_q_bl_dep_dpbs_dn7, locals.var_q_bl_dep_dpbs_dn10, locals.var_q_bl_dep_dpbs_dn11, locals.var_q_bl_dep_dpbs_dn12, locals.var_q_bl_dep_dpbs_dn17,)
    }
};
            locals.var_q_bl_dep_dpbs = assign12900_body32_e16763;
            locals.var_q_bl_dep_dpbs_dn0 = assign12900_body32_e16763_d_n0;
            locals.var_q_bl_dep_dpbs_dn2 = assign12900_body32_e16763_d_n2;
            locals.var_q_bl_dep_dpbs_dn6 = assign12900_body32_e16763_d_n6;
            locals.var_q_bl_dep_dpbs_dn7 = assign12900_body32_e16763_d_n7;
            locals.var_q_bl_dep_dpbs_dn10 = assign12900_body32_e16763_d_n10;
            locals.var_q_bl_dep_dpbs_dn11 = assign12900_body32_e16763_d_n11;
            locals.var_q_bl_dep_dpbs_dn12 = assign12900_body32_e16763_d_n12;
            locals.var_q_bl_dep_dpbs_dn17 = assign12900_body32_e16763_d_n17;
            let (assign12900_body33_e16773, assign12900_body33_e16773_d_n0, assign12900_body33_e16773_d_n2, assign12900_body33_e16773_d_n6, assign12900_body33_e16773_d_n7, assign12900_body33_e16773_d_n10, assign12900_body33_e16773_d_n11, assign12900_body33_e16773_d_n12, assign12900_body33_e16773_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard390 != 0.0)) {
        let assign12900_body33_e16771: f64 = (-locals.var_q_bl_dep_dpbs);
        (assign12900_body33_e16771, (-locals.var_q_bl_dep_dpbs_dn0), (-locals.var_q_bl_dep_dpbs_dn2), (-locals.var_q_bl_dep_dpbs_dn6), (-locals.var_q_bl_dep_dpbs_dn7), (-locals.var_q_bl_dep_dpbs_dn10), (-locals.var_q_bl_dep_dpbs_dn11), (-locals.var_q_bl_dep_dpbs_dn12), (-locals.var_q_bl_dep_dpbs_dn17),)
    } else {
        (locals.var_q_bl_dep_dpss, locals.var_q_bl_dep_dpss_dn0, locals.var_q_bl_dep_dpss_dn2, locals.var_q_bl_dep_dpss_dn6, locals.var_q_bl_dep_dpss_dn7, locals.var_q_bl_dep_dpss_dn10, locals.var_q_bl_dep_dpss_dn11, locals.var_q_bl_dep_dpss_dn12, locals.var_q_bl_dep_dpss_dn17,)
    }
};
            locals.var_q_bl_dep_dpss = assign12900_body33_e16773;
            locals.var_q_bl_dep_dpss_dn0 = assign12900_body33_e16773_d_n0;
            locals.var_q_bl_dep_dpss_dn2 = assign12900_body33_e16773_d_n2;
            locals.var_q_bl_dep_dpss_dn6 = assign12900_body33_e16773_d_n6;
            locals.var_q_bl_dep_dpss_dn7 = assign12900_body33_e16773_d_n7;
            locals.var_q_bl_dep_dpss_dn10 = assign12900_body33_e16773_d_n10;
            locals.var_q_bl_dep_dpss_dn11 = assign12900_body33_e16773_d_n11;
            locals.var_q_bl_dep_dpss_dn12 = assign12900_body33_e16773_d_n12;
            locals.var_q_bl_dep_dpss_dn17 = assign12900_body33_e16773_d_n17;
            let assign12900_body34_e16776: f64 = if locals.var_t1__blk351 > 1e-9 { 1.0 } else { 0.0 };
            locals.var_guard391 = assign12900_body34_e16776;
            let (assign12900_body35_e16791, assign12900_body35_e16791_d_n0, assign12900_body35_e16791_d_n2, assign12900_body35_e16791_d_n6, assign12900_body35_e16791_d_n7, assign12900_body35_e16791_d_n10, assign12900_body35_e16791_d_n11, assign12900_body35_e16791_d_n12, assign12900_body35_e16791_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard390 == 0.0)) && (locals.var_guard391 != 0.0)) {
        let assign12900_body35_e16787: f64 = (-locals.var_cnst0soi);
        let assign12900_body35_e16789: f64 = (assign12900_body35_e16787 * locals.var_t2__blk352);
        (assign12900_body35_e16789, (((-locals.var_cnst0soi_dn0) * locals.var_t2__blk352) + (assign12900_body35_e16787 * locals.var_t2__blk352_dn0)), (((-locals.var_cnst0soi_dn2) * locals.var_t2__blk352) + (assign12900_body35_e16787 * locals.var_t2__blk352_dn2)), (((-locals.var_cnst0soi_dn6) * locals.var_t2__blk352) + (assign12900_body35_e16787 * locals.var_t2__blk352_dn6)), (((-locals.var_cnst0soi_dn7) * locals.var_t2__blk352) + (assign12900_body35_e16787 * locals.var_t2__blk352_dn7)), (((-locals.var_cnst0soi_dn10) * locals.var_t2__blk352) + (assign12900_body35_e16787 * locals.var_t2__blk352_dn10)), (((-locals.var_cnst0soi_dn11) * locals.var_t2__blk352) + (assign12900_body35_e16787 * locals.var_t2__blk352_dn11)), (((-locals.var_cnst0soi_dn12) * locals.var_t2__blk352) + (assign12900_body35_e16787 * locals.var_t2__blk352_dn12)), (((-locals.var_cnst0soi_dn17) * locals.var_t2__blk352) + (assign12900_body35_e16787 * locals.var_t2__blk352_dn17)),)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn12, locals.var_q_bl_dep_dn17,)
    }
};
            locals.var_q_bl_dep = assign12900_body35_e16791;
            locals.var_q_bl_dep_dn0 = assign12900_body35_e16791_d_n0;
            locals.var_q_bl_dep_dn2 = assign12900_body35_e16791_d_n2;
            locals.var_q_bl_dep_dn6 = assign12900_body35_e16791_d_n6;
            locals.var_q_bl_dep_dn7 = assign12900_body35_e16791_d_n7;
            locals.var_q_bl_dep_dn10 = assign12900_body35_e16791_d_n10;
            locals.var_q_bl_dep_dn11 = assign12900_body35_e16791_d_n11;
            locals.var_q_bl_dep_dn12 = assign12900_body35_e16791_d_n12;
            locals.var_q_bl_dep_dn17 = assign12900_body35_e16791_d_n17;
            let (assign12900_body36_e16817, assign12900_body36_e16817_d_n0, assign12900_body36_e16817_d_n2, assign12900_body36_e16817_d_n6, assign12900_body36_e16817_d_n7, assign12900_body36_e16817_d_n10, assign12900_body36_e16817_d_n11, assign12900_body36_e16817_d_n12, assign12900_body36_e16817_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard390 == 0.0)) && (locals.var_guard391 != 0.0)) {
        let assign12900_body36_e16802: f64 = (-locals.var_cnst0soi);
        let assign12900_body36_e16804: f64 = (assign12900_body36_e16802 * locals.var_beta);
        let assign12900_body36_e16806: f64 = (-locals.var_t6);
        let assign12900_body36_e16808: f64 = (assign12900_body36_e16806 + 1.0);
        let assign12900_body36_e16809: f64 = (assign12900_body36_e16804 * assign12900_body36_e16808);
        let assign12900_body36_e16812: f64 = (2.0 * locals.var_t2__blk352);
        let assign12900_body36_e16813: f64 = (assign12900_body36_e16809 / assign12900_body36_e16812);
        let assign12900_body36_e16815: f64 = (assign12900_body36_e16813 / locals.var_qdepb_dlt);
        (assign12900_body36_e16815, ((((((((-locals.var_cnst0soi_dn0) * locals.var_beta) * assign12900_body36_e16808) + (assign12900_body36_e16804 * (-locals.var_t6_dn0))) * assign12900_body36_e16812) - (assign12900_body36_e16809 * (2.0 * locals.var_t2__blk352_dn0))) / (assign12900_body36_e16812 * assign12900_body36_e16812)) / locals.var_qdepb_dlt), ((((((((-locals.var_cnst0soi_dn2) * locals.var_beta) * assign12900_body36_e16808) + (assign12900_body36_e16804 * (-locals.var_t6_dn2))) * assign12900_body36_e16812) - (assign12900_body36_e16809 * (2.0 * locals.var_t2__blk352_dn2))) / (assign12900_body36_e16812 * assign12900_body36_e16812)) / locals.var_qdepb_dlt), ((((((((-locals.var_cnst0soi_dn6) * locals.var_beta) * assign12900_body36_e16808) + (assign12900_body36_e16804 * (-locals.var_t6_dn6))) * assign12900_body36_e16812) - (assign12900_body36_e16809 * (2.0 * locals.var_t2__blk352_dn6))) / (assign12900_body36_e16812 * assign12900_body36_e16812)) / locals.var_qdepb_dlt), ((((((((-locals.var_cnst0soi_dn7) * locals.var_beta) * assign12900_body36_e16808) + (assign12900_body36_e16804 * (-locals.var_t6_dn7))) * assign12900_body36_e16812) - (assign12900_body36_e16809 * (2.0 * locals.var_t2__blk352_dn7))) / (assign12900_body36_e16812 * assign12900_body36_e16812)) / locals.var_qdepb_dlt), (((((((((-locals.var_cnst0soi_dn10) * locals.var_beta) + (assign12900_body36_e16802 * locals.var_beta_dn10)) * assign12900_body36_e16808) + (assign12900_body36_e16804 * (-locals.var_t6_dn10))) * assign12900_body36_e16812) - (assign12900_body36_e16809 * (2.0 * locals.var_t2__blk352_dn10))) / (assign12900_body36_e16812 * assign12900_body36_e16812)) / locals.var_qdepb_dlt), ((((((((-locals.var_cnst0soi_dn11) * locals.var_beta) * assign12900_body36_e16808) + (assign12900_body36_e16804 * (-locals.var_t6_dn11))) * assign12900_body36_e16812) - (assign12900_body36_e16809 * (2.0 * locals.var_t2__blk352_dn11))) / (assign12900_body36_e16812 * assign12900_body36_e16812)) / locals.var_qdepb_dlt), ((((((((-locals.var_cnst0soi_dn12) * locals.var_beta) * assign12900_body36_e16808) + (assign12900_body36_e16804 * (-locals.var_t6_dn12))) * assign12900_body36_e16812) - (assign12900_body36_e16809 * (2.0 * locals.var_t2__blk352_dn12))) / (assign12900_body36_e16812 * assign12900_body36_e16812)) / locals.var_qdepb_dlt), ((((((((-locals.var_cnst0soi_dn17) * locals.var_beta) * assign12900_body36_e16808) + (assign12900_body36_e16804 * (-locals.var_t6_dn17))) * assign12900_body36_e16812) - (assign12900_body36_e16809 * (2.0 * locals.var_t2__blk352_dn17))) / (assign12900_body36_e16812 * assign12900_body36_e16812)) / locals.var_qdepb_dlt),)
    } else {
        (locals.var_q_bl_dep_dpbs, locals.var_q_bl_dep_dpbs_dn0, locals.var_q_bl_dep_dpbs_dn2, locals.var_q_bl_dep_dpbs_dn6, locals.var_q_bl_dep_dpbs_dn7, locals.var_q_bl_dep_dpbs_dn10, locals.var_q_bl_dep_dpbs_dn11, locals.var_q_bl_dep_dpbs_dn12, locals.var_q_bl_dep_dpbs_dn17,)
    }
};
            locals.var_q_bl_dep_dpbs = assign12900_body36_e16817;
            locals.var_q_bl_dep_dpbs_dn0 = assign12900_body36_e16817_d_n0;
            locals.var_q_bl_dep_dpbs_dn2 = assign12900_body36_e16817_d_n2;
            locals.var_q_bl_dep_dpbs_dn6 = assign12900_body36_e16817_d_n6;
            locals.var_q_bl_dep_dpbs_dn7 = assign12900_body36_e16817_d_n7;
            locals.var_q_bl_dep_dpbs_dn10 = assign12900_body36_e16817_d_n10;
            locals.var_q_bl_dep_dpbs_dn11 = assign12900_body36_e16817_d_n11;
            locals.var_q_bl_dep_dpbs_dn12 = assign12900_body36_e16817_d_n12;
            locals.var_q_bl_dep_dpbs_dn17 = assign12900_body36_e16817_d_n17;
            let (assign12900_body37_e16830, assign12900_body37_e16830_d_n0, assign12900_body37_e16830_d_n2, assign12900_body37_e16830_d_n6, assign12900_body37_e16830_d_n7, assign12900_body37_e16830_d_n10, assign12900_body37_e16830_d_n11, assign12900_body37_e16830_d_n12, assign12900_body37_e16830_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard390 == 0.0)) && (locals.var_guard391 != 0.0)) {
        let assign12900_body37_e16828: f64 = (-locals.var_q_bl_dep_dpbs);
        (assign12900_body37_e16828, (-locals.var_q_bl_dep_dpbs_dn0), (-locals.var_q_bl_dep_dpbs_dn2), (-locals.var_q_bl_dep_dpbs_dn6), (-locals.var_q_bl_dep_dpbs_dn7), (-locals.var_q_bl_dep_dpbs_dn10), (-locals.var_q_bl_dep_dpbs_dn11), (-locals.var_q_bl_dep_dpbs_dn12), (-locals.var_q_bl_dep_dpbs_dn17),)
    } else {
        (locals.var_q_bl_dep_dpss, locals.var_q_bl_dep_dpss_dn0, locals.var_q_bl_dep_dpss_dn2, locals.var_q_bl_dep_dpss_dn6, locals.var_q_bl_dep_dpss_dn7, locals.var_q_bl_dep_dpss_dn10, locals.var_q_bl_dep_dpss_dn11, locals.var_q_bl_dep_dpss_dn12, locals.var_q_bl_dep_dpss_dn17,)
    }
};
            locals.var_q_bl_dep_dpss = assign12900_body37_e16830;
            locals.var_q_bl_dep_dpss_dn0 = assign12900_body37_e16830_d_n0;
            locals.var_q_bl_dep_dpss_dn2 = assign12900_body37_e16830_d_n2;
            locals.var_q_bl_dep_dpss_dn6 = assign12900_body37_e16830_d_n6;
            locals.var_q_bl_dep_dpss_dn7 = assign12900_body37_e16830_d_n7;
            locals.var_q_bl_dep_dpss_dn10 = assign12900_body37_e16830_d_n10;
            locals.var_q_bl_dep_dpss_dn11 = assign12900_body37_e16830_d_n11;
            locals.var_q_bl_dep_dpss_dn12 = assign12900_body37_e16830_d_n12;
            locals.var_q_bl_dep_dpss_dn17 = assign12900_body37_e16830_d_n17;
            let (assign12900_body38_e16848, assign12900_body38_e16848_d_n0, assign12900_body38_e16848_d_n2, assign12900_body38_e16848_d_n6, assign12900_body38_e16848_d_n7, assign12900_body38_e16848_d_n10, assign12900_body38_e16848_d_n11, assign12900_body38_e16848_d_n12, assign12900_body38_e16848_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard390 == 0.0)) && (locals.var_guard391 == 0.0)) {
        let assign12900_body38_e16842: f64 = (-locals.var_cnst0soi);
        let assign12900_body38_e16844: f64 = (assign12900_body38_e16842 * locals.var_el);
        let assign12900_body38_e16846: f64 = (assign12900_body38_e16844 / 1.414213562373095);
        (assign12900_body38_e16846, ((((-locals.var_cnst0soi_dn0) * locals.var_el) + (assign12900_body38_e16842 * locals.var_el_dn0)) / 1.414213562373095), ((((-locals.var_cnst0soi_dn2) * locals.var_el) + (assign12900_body38_e16842 * locals.var_el_dn2)) / 1.414213562373095), ((((-locals.var_cnst0soi_dn6) * locals.var_el) + (assign12900_body38_e16842 * locals.var_el_dn6)) / 1.414213562373095), ((((-locals.var_cnst0soi_dn7) * locals.var_el) + (assign12900_body38_e16842 * locals.var_el_dn7)) / 1.414213562373095), ((((-locals.var_cnst0soi_dn10) * locals.var_el) + (assign12900_body38_e16842 * locals.var_el_dn10)) / 1.414213562373095), ((((-locals.var_cnst0soi_dn11) * locals.var_el) + (assign12900_body38_e16842 * locals.var_el_dn11)) / 1.414213562373095), ((((-locals.var_cnst0soi_dn12) * locals.var_el) + (assign12900_body38_e16842 * locals.var_el_dn12)) / 1.414213562373095), ((((-locals.var_cnst0soi_dn17) * locals.var_el) + (assign12900_body38_e16842 * locals.var_el_dn17)) / 1.414213562373095),)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn12, locals.var_q_bl_dep_dn17,)
    }
};
            locals.var_q_bl_dep = assign12900_body38_e16848;
            locals.var_q_bl_dep_dn0 = assign12900_body38_e16848_d_n0;
            locals.var_q_bl_dep_dn2 = assign12900_body38_e16848_d_n2;
            locals.var_q_bl_dep_dn6 = assign12900_body38_e16848_d_n6;
            locals.var_q_bl_dep_dn7 = assign12900_body38_e16848_d_n7;
            locals.var_q_bl_dep_dn10 = assign12900_body38_e16848_d_n10;
            locals.var_q_bl_dep_dn11 = assign12900_body38_e16848_d_n11;
            locals.var_q_bl_dep_dn12 = assign12900_body38_e16848_d_n12;
            locals.var_q_bl_dep_dn17 = assign12900_body38_e16848_d_n17;
            let (assign12900_body39_e16866, assign12900_body39_e16866_d_n0, assign12900_body39_e16866_d_n2, assign12900_body39_e16866_d_n6, assign12900_body39_e16866_d_n7, assign12900_body39_e16866_d_n10, assign12900_body39_e16866_d_n11, assign12900_body39_e16866_d_n12, assign12900_body39_e16866_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard390 == 0.0)) && (locals.var_guard391 == 0.0)) {
        let assign12900_body39_e16860: f64 = (-locals.var_cnst0soi);
        let assign12900_body39_e16862: f64 = (assign12900_body39_e16860 * locals.var_beta);
        let assign12900_body39_e16864: f64 = (assign12900_body39_e16862 / 1.414213562373095);
        (assign12900_body39_e16864, (((-locals.var_cnst0soi_dn0) * locals.var_beta) / 1.414213562373095), (((-locals.var_cnst0soi_dn2) * locals.var_beta) / 1.414213562373095), (((-locals.var_cnst0soi_dn6) * locals.var_beta) / 1.414213562373095), (((-locals.var_cnst0soi_dn7) * locals.var_beta) / 1.414213562373095), ((((-locals.var_cnst0soi_dn10) * locals.var_beta) + (assign12900_body39_e16860 * locals.var_beta_dn10)) / 1.414213562373095), (((-locals.var_cnst0soi_dn11) * locals.var_beta) / 1.414213562373095), (((-locals.var_cnst0soi_dn12) * locals.var_beta) / 1.414213562373095), (((-locals.var_cnst0soi_dn17) * locals.var_beta) / 1.414213562373095),)
    } else {
        (locals.var_q_bl_dep_dpbs, locals.var_q_bl_dep_dpbs_dn0, locals.var_q_bl_dep_dpbs_dn2, locals.var_q_bl_dep_dpbs_dn6, locals.var_q_bl_dep_dpbs_dn7, locals.var_q_bl_dep_dpbs_dn10, locals.var_q_bl_dep_dpbs_dn11, locals.var_q_bl_dep_dpbs_dn12, locals.var_q_bl_dep_dpbs_dn17,)
    }
};
            locals.var_q_bl_dep_dpbs = assign12900_body39_e16866;
            locals.var_q_bl_dep_dpbs_dn0 = assign12900_body39_e16866_d_n0;
            locals.var_q_bl_dep_dpbs_dn2 = assign12900_body39_e16866_d_n2;
            locals.var_q_bl_dep_dpbs_dn6 = assign12900_body39_e16866_d_n6;
            locals.var_q_bl_dep_dpbs_dn7 = assign12900_body39_e16866_d_n7;
            locals.var_q_bl_dep_dpbs_dn10 = assign12900_body39_e16866_d_n10;
            locals.var_q_bl_dep_dpbs_dn11 = assign12900_body39_e16866_d_n11;
            locals.var_q_bl_dep_dpbs_dn12 = assign12900_body39_e16866_d_n12;
            locals.var_q_bl_dep_dpbs_dn17 = assign12900_body39_e16866_d_n17;
            let (assign12900_body40_e16880, assign12900_body40_e16880_d_n0, assign12900_body40_e16880_d_n2, assign12900_body40_e16880_d_n6, assign12900_body40_e16880_d_n7, assign12900_body40_e16880_d_n10, assign12900_body40_e16880_d_n11, assign12900_body40_e16880_d_n12, assign12900_body40_e16880_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard390 == 0.0)) && (locals.var_guard391 == 0.0)) {
        let assign12900_body40_e16878: f64 = (-locals.var_q_bl_dep_dpbs);
        (assign12900_body40_e16878, (-locals.var_q_bl_dep_dpbs_dn0), (-locals.var_q_bl_dep_dpbs_dn2), (-locals.var_q_bl_dep_dpbs_dn6), (-locals.var_q_bl_dep_dpbs_dn7), (-locals.var_q_bl_dep_dpbs_dn10), (-locals.var_q_bl_dep_dpbs_dn11), (-locals.var_q_bl_dep_dpbs_dn12), (-locals.var_q_bl_dep_dpbs_dn17),)
    } else {
        (locals.var_q_bl_dep_dpss, locals.var_q_bl_dep_dpss_dn0, locals.var_q_bl_dep_dpss_dn2, locals.var_q_bl_dep_dpss_dn6, locals.var_q_bl_dep_dpss_dn7, locals.var_q_bl_dep_dpss_dn10, locals.var_q_bl_dep_dpss_dn11, locals.var_q_bl_dep_dpss_dn12, locals.var_q_bl_dep_dpss_dn17,)
    }
};
            locals.var_q_bl_dep_dpss = assign12900_body40_e16880;
            locals.var_q_bl_dep_dpss_dn0 = assign12900_body40_e16880_d_n0;
            locals.var_q_bl_dep_dpss_dn2 = assign12900_body40_e16880_d_n2;
            locals.var_q_bl_dep_dpss_dn6 = assign12900_body40_e16880_d_n6;
            locals.var_q_bl_dep_dpss_dn7 = assign12900_body40_e16880_d_n7;
            locals.var_q_bl_dep_dpss_dn10 = assign12900_body40_e16880_d_n10;
            locals.var_q_bl_dep_dpss_dn11 = assign12900_body40_e16880_d_n11;
            locals.var_q_bl_dep_dpss_dn12 = assign12900_body40_e16880_d_n12;
            locals.var_q_bl_dep_dpss_dn17 = assign12900_body40_e16880_d_n17;
            let assign12900_body41_e16884: f64 = (-locals.var_q_wdsoi_max);
            let assign12900_body41_e16886: f64 = assign12900_body41_e16884;
            let assign12900_body41_e16887: f64 = (-assign12900_body41_e16886);
            let assign12900_body41_e16890: f64 = (-locals.var_q_wdsoi_max);
            let assign12900_body41_e16892: f64 = assign12900_body41_e16890;
            let assign12900_body41_e16895: f64 = if ((locals.var_q_bl_dep > assign12900_body41_e16887) && (assign12900_body41_e16892 >= 0.0)) { 1.0 } else { 0.0 };
            locals.var_guard392 = assign12900_body41_e16895;
            let (assign12900_body42_e16911, assign12900_body42_e16911_d_n0, assign12900_body42_e16911_d_n2, assign12900_body42_e16911_d_n6, assign12900_body42_e16911_d_n7, assign12900_body42_e16911_d_n10, assign12900_body42_e16911_d_n11, assign12900_body42_e16911_d_n12, assign12900_body42_e16911_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard392 != 0.0)) {
        let assign12900_body42_e16904: f64 = locals.var_q_bl_dep;
        let assign12900_body42_e16906: f64 = (-locals.var_q_wdsoi_max);
        let assign12900_body42_e16908: f64 = assign12900_body42_e16906;
        let assign12900_body42_e16909: f64 = (assign12900_body42_e16904 + assign12900_body42_e16908);
        (assign12900_body42_e16909, (locals.var_q_bl_dep_dn0 + (-locals.var_q_wdsoi_max_dn0)), (locals.var_q_bl_dep_dn2 + (-locals.var_q_wdsoi_max_dn2)), (locals.var_q_bl_dep_dn6 + (-locals.var_q_wdsoi_max_dn6)), (locals.var_q_bl_dep_dn7 + (-locals.var_q_wdsoi_max_dn7)), (locals.var_q_bl_dep_dn10 + (-locals.var_q_wdsoi_max_dn10)), (locals.var_q_bl_dep_dn11 + (-locals.var_q_wdsoi_max_dn11)), (locals.var_q_bl_dep_dn12 + (-locals.var_q_wdsoi_max_dn12)), (locals.var_q_bl_dep_dn17 + (-locals.var_q_wdsoi_max_dn17)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
            locals.var_tmf1 = assign12900_body42_e16911;
            locals.var_tmf1_dn0 = assign12900_body42_e16911_d_n0;
            locals.var_tmf1_dn2 = assign12900_body42_e16911_d_n2;
            locals.var_tmf1_dn6 = assign12900_body42_e16911_d_n6;
            locals.var_tmf1_dn7 = assign12900_body42_e16911_d_n7;
            locals.var_tmf1_dn10 = assign12900_body42_e16911_d_n10;
            locals.var_tmf1_dn11 = assign12900_body42_e16911_d_n11;
            locals.var_tmf1_dn12 = assign12900_body42_e16911_d_n12;
            locals.var_tmf1_dn17 = assign12900_body42_e16911_d_n17;
            let (assign12900_body43_e16922, assign12900_body43_e16922_d_n0, assign12900_body43_e16922_d_n2, assign12900_body43_e16922_d_n6, assign12900_body43_e16922_d_n7, assign12900_body43_e16922_d_n10, assign12900_body43_e16922_d_n11, assign12900_body43_e16922_d_n12, assign12900_body43_e16922_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard392 != 0.0)) {
        let assign12900_body43_e16920: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign12900_body43_e16920, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
            locals.var_x2 = assign12900_body43_e16922;
            locals.var_x2_dn0 = assign12900_body43_e16922_d_n0;
            locals.var_x2_dn2 = assign12900_body43_e16922_d_n2;
            locals.var_x2_dn6 = assign12900_body43_e16922_d_n6;
            locals.var_x2_dn7 = assign12900_body43_e16922_d_n7;
            locals.var_x2_dn10 = assign12900_body43_e16922_d_n10;
            locals.var_x2_dn11 = assign12900_body43_e16922_d_n11;
            locals.var_x2_dn12 = assign12900_body43_e16922_d_n12;
            locals.var_x2_dn17 = assign12900_body43_e16922_d_n17;
            let (assign12900_body44_e16939, assign12900_body44_e16939_d_n0, assign12900_body44_e16939_d_n2, assign12900_body44_e16939_d_n6, assign12900_body44_e16939_d_n7, assign12900_body44_e16939_d_n10, assign12900_body44_e16939_d_n11, assign12900_body44_e16939_d_n12, assign12900_body44_e16939_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard392 != 0.0)) {
        let assign12900_body44_e16930: f64 = (-locals.var_q_wdsoi_max);
        let assign12900_body44_e16932: f64 = assign12900_body44_e16930;
        let assign12900_body44_e16934: f64 = (-locals.var_q_wdsoi_max);
        let assign12900_body44_e16936: f64 = assign12900_body44_e16934;
        let assign12900_body44_e16937: f64 = (assign12900_body44_e16932 * assign12900_body44_e16936);
        (assign12900_body44_e16937, (((-locals.var_q_wdsoi_max_dn0) * assign12900_body44_e16936) + (assign12900_body44_e16932 * (-locals.var_q_wdsoi_max_dn0))), (((-locals.var_q_wdsoi_max_dn2) * assign12900_body44_e16936) + (assign12900_body44_e16932 * (-locals.var_q_wdsoi_max_dn2))), (((-locals.var_q_wdsoi_max_dn6) * assign12900_body44_e16936) + (assign12900_body44_e16932 * (-locals.var_q_wdsoi_max_dn6))), (((-locals.var_q_wdsoi_max_dn7) * assign12900_body44_e16936) + (assign12900_body44_e16932 * (-locals.var_q_wdsoi_max_dn7))), (((-locals.var_q_wdsoi_max_dn10) * assign12900_body44_e16936) + (assign12900_body44_e16932 * (-locals.var_q_wdsoi_max_dn10))), (((-locals.var_q_wdsoi_max_dn11) * assign12900_body44_e16936) + (assign12900_body44_e16932 * (-locals.var_q_wdsoi_max_dn11))), (((-locals.var_q_wdsoi_max_dn12) * assign12900_body44_e16936) + (assign12900_body44_e16932 * (-locals.var_q_wdsoi_max_dn12))), (((-locals.var_q_wdsoi_max_dn17) * assign12900_body44_e16936) + (assign12900_body44_e16932 * (-locals.var_q_wdsoi_max_dn17))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
            locals.var_xmax2 = assign12900_body44_e16939;
            locals.var_xmax2_dn0 = assign12900_body44_e16939_d_n0;
            locals.var_xmax2_dn2 = assign12900_body44_e16939_d_n2;
            locals.var_xmax2_dn6 = assign12900_body44_e16939_d_n6;
            locals.var_xmax2_dn7 = assign12900_body44_e16939_d_n7;
            locals.var_xmax2_dn10 = assign12900_body44_e16939_d_n10;
            locals.var_xmax2_dn11 = assign12900_body44_e16939_d_n11;
            locals.var_xmax2_dn12 = assign12900_body44_e16939_d_n12;
            locals.var_xmax2_dn17 = assign12900_body44_e16939_d_n17;
            let (assign12900_body45_e16948, assign12900_body45_e16948_d_n0, assign12900_body45_e16948_d_n2, assign12900_body45_e16948_d_n6, assign12900_body45_e16948_d_n7, assign12900_body45_e16948_d_n10, assign12900_body45_e16948_d_n11, assign12900_body45_e16948_d_n12, assign12900_body45_e16948_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard392 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
            locals.var_xp = assign12900_body45_e16948;
            locals.var_xp_dn0 = assign12900_body45_e16948_d_n0;
            locals.var_xp_dn2 = assign12900_body45_e16948_d_n2;
            locals.var_xp_dn6 = assign12900_body45_e16948_d_n6;
            locals.var_xp_dn7 = assign12900_body45_e16948_d_n7;
            locals.var_xp_dn10 = assign12900_body45_e16948_d_n10;
            locals.var_xp_dn11 = assign12900_body45_e16948_d_n11;
            locals.var_xp_dn12 = assign12900_body45_e16948_d_n12;
            locals.var_xp_dn17 = assign12900_body45_e16948_d_n17;
            let (assign12900_body46_e16957, assign12900_body46_e16957_d_n0, assign12900_body46_e16957_d_n2, assign12900_body46_e16957_d_n6, assign12900_body46_e16957_d_n7, assign12900_body46_e16957_d_n10, assign12900_body46_e16957_d_n11, assign12900_body46_e16957_d_n12, assign12900_body46_e16957_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard392 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
            locals.var_xmp = assign12900_body46_e16957;
            locals.var_xmp_dn0 = assign12900_body46_e16957_d_n0;
            locals.var_xmp_dn2 = assign12900_body46_e16957_d_n2;
            locals.var_xmp_dn6 = assign12900_body46_e16957_d_n6;
            locals.var_xmp_dn7 = assign12900_body46_e16957_d_n7;
            locals.var_xmp_dn10 = assign12900_body46_e16957_d_n10;
            locals.var_xmp_dn11 = assign12900_body46_e16957_d_n11;
            locals.var_xmp_dn12 = assign12900_body46_e16957_d_n12;
            locals.var_xmp_dn17 = assign12900_body46_e16957_d_n17;
            let (assign12900_body47_e16966,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard392 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign12900_body47_e16966;
            let (assign12900_body48_e16975,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard392 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign12900_body48_e16975;
            let (assign12900_body49_e16984, assign12900_body49_e16984_d_n0, assign12900_body49_e16984_d_n2, assign12900_body49_e16984_d_n6, assign12900_body49_e16984_d_n7, assign12900_body49_e16984_d_n10, assign12900_body49_e16984_d_n11, assign12900_body49_e16984_d_n12, assign12900_body49_e16984_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard392 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
            locals.var_arg = assign12900_body49_e16984;
            locals.var_arg_dn0 = assign12900_body49_e16984_d_n0;
            locals.var_arg_dn2 = assign12900_body49_e16984_d_n2;
            locals.var_arg_dn6 = assign12900_body49_e16984_d_n6;
            locals.var_arg_dn7 = assign12900_body49_e16984_d_n7;
            locals.var_arg_dn10 = assign12900_body49_e16984_d_n10;
            locals.var_arg_dn11 = assign12900_body49_e16984_d_n11;
            locals.var_arg_dn12 = assign12900_body49_e16984_d_n12;
            locals.var_arg_dn17 = assign12900_body49_e16984_d_n17;
            let (assign12900_body50_e16993, assign12900_body50_e16993_d_n0, assign12900_body50_e16993_d_n2, assign12900_body50_e16993_d_n6, assign12900_body50_e16993_d_n7, assign12900_body50_e16993_d_n10, assign12900_body50_e16993_d_n11, assign12900_body50_e16993_d_n12, assign12900_body50_e16993_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard392 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign12900_body50_e16993;
            locals.var_dnm_dn0 = assign12900_body50_e16993_d_n0;
            locals.var_dnm_dn2 = assign12900_body50_e16993_d_n2;
            locals.var_dnm_dn6 = assign12900_body50_e16993_d_n6;
            locals.var_dnm_dn7 = assign12900_body50_e16993_d_n7;
            locals.var_dnm_dn10 = assign12900_body50_e16993_d_n10;
            locals.var_dnm_dn11 = assign12900_body50_e16993_d_n11;
            locals.var_dnm_dn12 = assign12900_body50_e16993_d_n12;
            locals.var_dnm_dn17 = assign12900_body50_e16993_d_n17;
            let (assign12900_body51_e17004, assign12900_body51_e17004_d_n0, assign12900_body51_e17004_d_n2, assign12900_body51_e17004_d_n6, assign12900_body51_e17004_d_n7, assign12900_body51_e17004_d_n10, assign12900_body51_e17004_d_n11, assign12900_body51_e17004_d_n12, assign12900_body51_e17004_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard392 != 0.0)) {
        let assign12900_body51_e17002: f64 = (locals.var_xp * locals.var_x2);
        (assign12900_body51_e17002, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
            locals.var_xp = assign12900_body51_e17004;
            locals.var_xp_dn0 = assign12900_body51_e17004_d_n0;
            locals.var_xp_dn2 = assign12900_body51_e17004_d_n2;
            locals.var_xp_dn6 = assign12900_body51_e17004_d_n6;
            locals.var_xp_dn7 = assign12900_body51_e17004_d_n7;
            locals.var_xp_dn10 = assign12900_body51_e17004_d_n10;
            locals.var_xp_dn11 = assign12900_body51_e17004_d_n11;
            locals.var_xp_dn12 = assign12900_body51_e17004_d_n12;
            locals.var_xp_dn17 = assign12900_body51_e17004_d_n17;
            let (assign12900_body52_e17015, assign12900_body52_e17015_d_n0, assign12900_body52_e17015_d_n2, assign12900_body52_e17015_d_n6, assign12900_body52_e17015_d_n7, assign12900_body52_e17015_d_n10, assign12900_body52_e17015_d_n11, assign12900_body52_e17015_d_n12, assign12900_body52_e17015_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard392 != 0.0)) {
        let assign12900_body52_e17013: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign12900_body52_e17013, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
            locals.var_xmp = assign12900_body52_e17015;
            locals.var_xmp_dn0 = assign12900_body52_e17015_d_n0;
            locals.var_xmp_dn2 = assign12900_body52_e17015_d_n2;
            locals.var_xmp_dn6 = assign12900_body52_e17015_d_n6;
            locals.var_xmp_dn7 = assign12900_body52_e17015_d_n7;
            locals.var_xmp_dn10 = assign12900_body52_e17015_d_n10;
            locals.var_xmp_dn11 = assign12900_body52_e17015_d_n11;
            locals.var_xmp_dn12 = assign12900_body52_e17015_d_n12;
            locals.var_xmp_dn17 = assign12900_body52_e17015_d_n17;
            let (assign12900_body53_e17026, assign12900_body53_e17026_d_n0, assign12900_body53_e17026_d_n2, assign12900_body53_e17026_d_n6, assign12900_body53_e17026_d_n7, assign12900_body53_e17026_d_n10, assign12900_body53_e17026_d_n11, assign12900_body53_e17026_d_n12, assign12900_body53_e17026_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard392 != 0.0)) {
        let assign12900_body53_e17024: f64 = (locals.var_xp * locals.var_x2);
        (assign12900_body53_e17024, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
            locals.var_xp = assign12900_body53_e17026;
            locals.var_xp_dn0 = assign12900_body53_e17026_d_n0;
            locals.var_xp_dn2 = assign12900_body53_e17026_d_n2;
            locals.var_xp_dn6 = assign12900_body53_e17026_d_n6;
            locals.var_xp_dn7 = assign12900_body53_e17026_d_n7;
            locals.var_xp_dn10 = assign12900_body53_e17026_d_n10;
            locals.var_xp_dn11 = assign12900_body53_e17026_d_n11;
            locals.var_xp_dn12 = assign12900_body53_e17026_d_n12;
            locals.var_xp_dn17 = assign12900_body53_e17026_d_n17;
            let (assign12900_body54_e17037, assign12900_body54_e17037_d_n0, assign12900_body54_e17037_d_n2, assign12900_body54_e17037_d_n6, assign12900_body54_e17037_d_n7, assign12900_body54_e17037_d_n10, assign12900_body54_e17037_d_n11, assign12900_body54_e17037_d_n12, assign12900_body54_e17037_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard392 != 0.0)) {
        let assign12900_body54_e17035: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign12900_body54_e17035, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
            locals.var_xmp = assign12900_body54_e17037;
            locals.var_xmp_dn0 = assign12900_body54_e17037_d_n0;
            locals.var_xmp_dn2 = assign12900_body54_e17037_d_n2;
            locals.var_xmp_dn6 = assign12900_body54_e17037_d_n6;
            locals.var_xmp_dn7 = assign12900_body54_e17037_d_n7;
            locals.var_xmp_dn10 = assign12900_body54_e17037_d_n10;
            locals.var_xmp_dn11 = assign12900_body54_e17037_d_n11;
            locals.var_xmp_dn12 = assign12900_body54_e17037_d_n12;
            locals.var_xmp_dn17 = assign12900_body54_e17037_d_n17;
            let (assign12900_body55_e17048, assign12900_body55_e17048_d_n0, assign12900_body55_e17048_d_n2, assign12900_body55_e17048_d_n6, assign12900_body55_e17048_d_n7, assign12900_body55_e17048_d_n10, assign12900_body55_e17048_d_n11, assign12900_body55_e17048_d_n12, assign12900_body55_e17048_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard392 != 0.0)) {
        let assign12900_body55_e17046: f64 = (locals.var_xp + locals.var_xmp);
        (assign12900_body55_e17046, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
            locals.var_arg = assign12900_body55_e17048;
            locals.var_arg_dn0 = assign12900_body55_e17048_d_n0;
            locals.var_arg_dn2 = assign12900_body55_e17048_d_n2;
            locals.var_arg_dn6 = assign12900_body55_e17048_d_n6;
            locals.var_arg_dn7 = assign12900_body55_e17048_d_n7;
            locals.var_arg_dn10 = assign12900_body55_e17048_d_n10;
            locals.var_arg_dn11 = assign12900_body55_e17048_d_n11;
            locals.var_arg_dn12 = assign12900_body55_e17048_d_n12;
            locals.var_arg_dn17 = assign12900_body55_e17048_d_n17;
            let (assign12900_body56_e17057, assign12900_body56_e17057_d_n0, assign12900_body56_e17057_d_n2, assign12900_body56_e17057_d_n6, assign12900_body56_e17057_d_n7, assign12900_body56_e17057_d_n10, assign12900_body56_e17057_d_n11, assign12900_body56_e17057_d_n12, assign12900_body56_e17057_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard392 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign12900_body56_e17057;
            locals.var_dnm_dn0 = assign12900_body56_e17057_d_n0;
            locals.var_dnm_dn2 = assign12900_body56_e17057_d_n2;
            locals.var_dnm_dn6 = assign12900_body56_e17057_d_n6;
            locals.var_dnm_dn7 = assign12900_body56_e17057_d_n7;
            locals.var_dnm_dn10 = assign12900_body56_e17057_d_n10;
            locals.var_dnm_dn11 = assign12900_body56_e17057_d_n11;
            locals.var_dnm_dn12 = assign12900_body56_e17057_d_n12;
            locals.var_dnm_dn17 = assign12900_body56_e17057_d_n17;
            let assign12900_body57_e17072: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
            locals.var_guard393 = assign12900_body57_e17072;
            let assign12900_body58_e17075: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard394 = assign12900_body58_e17075;
            let (assign12900_body59_e17088,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard392 != 0.0)) && (locals.var_guard393 != 0.0)) && (locals.var_guard394 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign12900_body59_e17088;
            let assign12900_body60_e17091: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
            locals.var_guard395 = assign12900_body60_e17091;
            let (assign12900_body61_e17107,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard392 != 0.0)) && (locals.var_guard393 != 0.0)) && (locals.var_guard394 == 0.0)) && (locals.var_guard395 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign12900_body61_e17107;
            let assign12900_body62_e17110: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
            locals.var_guard396 = assign12900_body62_e17110;
            let (assign12900_body63_e17129,) = {
    if (((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard392 != 0.0)) && (locals.var_guard393 != 0.0)) && (locals.var_guard394 == 0.0)) && (locals.var_guard395 == 0.0)) && (locals.var_guard396 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign12900_body63_e17129;
            let assign12900_body64_e17132: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
            locals.var_guard397 = assign12900_body64_e17132;
            let (assign12900_body65_e17154,) = {
    if ((((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard392 != 0.0)) && (locals.var_guard393 != 0.0)) && (locals.var_guard394 == 0.0)) && (locals.var_guard395 == 0.0)) && (locals.var_guard396 == 0.0)) && (locals.var_guard397 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign12900_body65_e17154;
            let (assign12900_body66_e17165,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard392 != 0.0)) && (locals.var_guard393 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign12900_body66_e17165;
            let mut assign12900_body67_loop_guard: usize = 0;
            while {
                let assign12900_body67_cond_e17177: f64 = if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard392 != 0.0)) && (locals.var_guard393 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
                assign12900_body67_cond_e17177 != 0.0
            } {
                assign12900_body67_loop_guard += 1;
                assert!(assign12900_body67_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                let (assign12900_body67_body0_e17189, assign12900_body67_body0_e17189_d_n0, assign12900_body67_body0_e17189_d_n2, assign12900_body67_body0_e17189_d_n6, assign12900_body67_body0_e17189_d_n7, assign12900_body67_body0_e17189_d_n10, assign12900_body67_body0_e17189_d_n11, assign12900_body67_body0_e17189_d_n12, assign12900_body67_body0_e17189_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard392 != 0.0)) && (locals.var_guard393 != 0.0)) {
        let assign12900_body67_body0_e17187: f64 = (locals.var_dnm).sqrt();
        (assign12900_body67_body0_e17187, (locals.var_dnm_dn0 / (2.0 * assign12900_body67_body0_e17187)), (locals.var_dnm_dn2 / (2.0 * assign12900_body67_body0_e17187)), (locals.var_dnm_dn6 / (2.0 * assign12900_body67_body0_e17187)), (locals.var_dnm_dn7 / (2.0 * assign12900_body67_body0_e17187)), (locals.var_dnm_dn10 / (2.0 * assign12900_body67_body0_e17187)), (locals.var_dnm_dn11 / (2.0 * assign12900_body67_body0_e17187)), (locals.var_dnm_dn12 / (2.0 * assign12900_body67_body0_e17187)), (locals.var_dnm_dn17 / (2.0 * assign12900_body67_body0_e17187)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
                locals.var_dnm = assign12900_body67_body0_e17189;
                locals.var_dnm_dn0 = assign12900_body67_body0_e17189_d_n0;
                locals.var_dnm_dn2 = assign12900_body67_body0_e17189_d_n2;
                locals.var_dnm_dn6 = assign12900_body67_body0_e17189_d_n6;
                locals.var_dnm_dn7 = assign12900_body67_body0_e17189_d_n7;
                locals.var_dnm_dn10 = assign12900_body67_body0_e17189_d_n10;
                locals.var_dnm_dn11 = assign12900_body67_body0_e17189_d_n11;
                locals.var_dnm_dn12 = assign12900_body67_body0_e17189_d_n12;
                locals.var_dnm_dn17 = assign12900_body67_body0_e17189_d_n17;
                let (assign12900_body67_body1_e17202,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard392 != 0.0)) && (locals.var_guard393 != 0.0)) {
        let assign12900_body67_body1_e17200: f64 = (locals.var_m0 + 1.0);
        (assign12900_body67_body1_e17200,)
    } else {
        (locals.var_m0,)
    }
};
                locals.var_m0 = assign12900_body67_body1_e17202;
            }
            let (assign12900_body68_e17220, assign12900_body68_e17220_d_n0, assign12900_body68_e17220_d_n2, assign12900_body68_e17220_d_n6, assign12900_body68_e17220_d_n7, assign12900_body68_e17220_d_n10, assign12900_body68_e17220_d_n11, assign12900_body68_e17220_d_n12, assign12900_body68_e17220_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard392 != 0.0)) && (locals.var_guard393 == 0.0)) {
        let assign12900_body68_e17216: f64 = (2.0 * 2.0);
        let assign12900_body68_e17217: f64 = (1.0 / assign12900_body68_e17216);
        let assign12900_body68_e17218: f64 = (locals.var_dnm).powf(assign12900_body68_e17217);
        (assign12900_body68_e17218, if 0.0 == 0.0 && ((assign12900_body68_e17217) as f64).is_finite() && ((assign12900_body68_e17217) as f64).fract() == 0.0 { if assign12900_body68_e17217 == 0.0 { 0.0 } else { (assign12900_body68_e17217 * ((locals.var_dnm).powf(assign12900_body68_e17217 - 1.0) * locals.var_dnm_dn0)) } } else { (assign12900_body68_e17218 * (assign12900_body68_e17217 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12900_body68_e17217) as f64).is_finite() && ((assign12900_body68_e17217) as f64).fract() == 0.0 { if assign12900_body68_e17217 == 0.0 { 0.0 } else { (assign12900_body68_e17217 * ((locals.var_dnm).powf(assign12900_body68_e17217 - 1.0) * locals.var_dnm_dn2)) } } else { (assign12900_body68_e17218 * (assign12900_body68_e17217 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12900_body68_e17217) as f64).is_finite() && ((assign12900_body68_e17217) as f64).fract() == 0.0 { if assign12900_body68_e17217 == 0.0 { 0.0 } else { (assign12900_body68_e17217 * ((locals.var_dnm).powf(assign12900_body68_e17217 - 1.0) * locals.var_dnm_dn6)) } } else { (assign12900_body68_e17218 * (assign12900_body68_e17217 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12900_body68_e17217) as f64).is_finite() && ((assign12900_body68_e17217) as f64).fract() == 0.0 { if assign12900_body68_e17217 == 0.0 { 0.0 } else { (assign12900_body68_e17217 * ((locals.var_dnm).powf(assign12900_body68_e17217 - 1.0) * locals.var_dnm_dn7)) } } else { (assign12900_body68_e17218 * (assign12900_body68_e17217 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12900_body68_e17217) as f64).is_finite() && ((assign12900_body68_e17217) as f64).fract() == 0.0 { if assign12900_body68_e17217 == 0.0 { 0.0 } else { (assign12900_body68_e17217 * ((locals.var_dnm).powf(assign12900_body68_e17217 - 1.0) * locals.var_dnm_dn10)) } } else { (assign12900_body68_e17218 * (assign12900_body68_e17217 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12900_body68_e17217) as f64).is_finite() && ((assign12900_body68_e17217) as f64).fract() == 0.0 { if assign12900_body68_e17217 == 0.0 { 0.0 } else { (assign12900_body68_e17217 * ((locals.var_dnm).powf(assign12900_body68_e17217 - 1.0) * locals.var_dnm_dn11)) } } else { (assign12900_body68_e17218 * (assign12900_body68_e17217 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12900_body68_e17217) as f64).is_finite() && ((assign12900_body68_e17217) as f64).fract() == 0.0 { if assign12900_body68_e17217 == 0.0 { 0.0 } else { (assign12900_body68_e17217 * ((locals.var_dnm).powf(assign12900_body68_e17217 - 1.0) * locals.var_dnm_dn12)) } } else { (assign12900_body68_e17218 * (assign12900_body68_e17217 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12900_body68_e17217) as f64).is_finite() && ((assign12900_body68_e17217) as f64).fract() == 0.0 { if assign12900_body68_e17217 == 0.0 { 0.0 } else { (assign12900_body68_e17217 * ((locals.var_dnm).powf(assign12900_body68_e17217 - 1.0) * locals.var_dnm_dn17)) } } else { (assign12900_body68_e17218 * (assign12900_body68_e17217 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign12900_body68_e17220;
            locals.var_dnm_dn0 = assign12900_body68_e17220_d_n0;
            locals.var_dnm_dn2 = assign12900_body68_e17220_d_n2;
            locals.var_dnm_dn6 = assign12900_body68_e17220_d_n6;
            locals.var_dnm_dn7 = assign12900_body68_e17220_d_n7;
            locals.var_dnm_dn10 = assign12900_body68_e17220_d_n10;
            locals.var_dnm_dn11 = assign12900_body68_e17220_d_n11;
            locals.var_dnm_dn12 = assign12900_body68_e17220_d_n12;
            locals.var_dnm_dn17 = assign12900_body68_e17220_d_n17;
            let (assign12900_body69_e17231, assign12900_body69_e17231_d_n0, assign12900_body69_e17231_d_n2, assign12900_body69_e17231_d_n6, assign12900_body69_e17231_d_n7, assign12900_body69_e17231_d_n10, assign12900_body69_e17231_d_n11, assign12900_body69_e17231_d_n12, assign12900_body69_e17231_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard392 != 0.0)) {
        let assign12900_body69_e17229: f64 = (1.0 / locals.var_dnm);
        (assign12900_body69_e17229, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign12900_body69_e17231;
            locals.var_dnm_dn0 = assign12900_body69_e17231_d_n0;
            locals.var_dnm_dn2 = assign12900_body69_e17231_d_n2;
            locals.var_dnm_dn6 = assign12900_body69_e17231_d_n6;
            locals.var_dnm_dn7 = assign12900_body69_e17231_d_n7;
            locals.var_dnm_dn10 = assign12900_body69_e17231_d_n10;
            locals.var_dnm_dn11 = assign12900_body69_e17231_d_n11;
            locals.var_dnm_dn12 = assign12900_body69_e17231_d_n12;
            locals.var_dnm_dn17 = assign12900_body69_e17231_d_n17;
            let (assign12900_body70_e17247, assign12900_body70_e17247_d_n0, assign12900_body70_e17247_d_n2, assign12900_body70_e17247_d_n6, assign12900_body70_e17247_d_n7, assign12900_body70_e17247_d_n10, assign12900_body70_e17247_d_n11, assign12900_body70_e17247_d_n12, assign12900_body70_e17247_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard392 != 0.0)) {
        let assign12900_body70_e17240: f64 = (-locals.var_q_wdsoi_max);
        let assign12900_body70_e17242: f64 = assign12900_body70_e17240;
        let assign12900_body70_e17243: f64 = (locals.var_tmf1 * assign12900_body70_e17242);
        let assign12900_body70_e17245: f64 = (assign12900_body70_e17243 * locals.var_dnm);
        (assign12900_body70_e17245, ((((locals.var_tmf1_dn0 * assign12900_body70_e17242) + (locals.var_tmf1 * (-locals.var_q_wdsoi_max_dn0))) * locals.var_dnm) + (assign12900_body70_e17243 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign12900_body70_e17242) + (locals.var_tmf1 * (-locals.var_q_wdsoi_max_dn2))) * locals.var_dnm) + (assign12900_body70_e17243 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn6 * assign12900_body70_e17242) + (locals.var_tmf1 * (-locals.var_q_wdsoi_max_dn6))) * locals.var_dnm) + (assign12900_body70_e17243 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign12900_body70_e17242) + (locals.var_tmf1 * (-locals.var_q_wdsoi_max_dn7))) * locals.var_dnm) + (assign12900_body70_e17243 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn10 * assign12900_body70_e17242) + (locals.var_tmf1 * (-locals.var_q_wdsoi_max_dn10))) * locals.var_dnm) + (assign12900_body70_e17243 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * assign12900_body70_e17242) + (locals.var_tmf1 * (-locals.var_q_wdsoi_max_dn11))) * locals.var_dnm) + (assign12900_body70_e17243 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn12 * assign12900_body70_e17242) + (locals.var_tmf1 * (-locals.var_q_wdsoi_max_dn12))) * locals.var_dnm) + (assign12900_body70_e17243 * locals.var_dnm_dn12)), ((((locals.var_tmf1_dn17 * assign12900_body70_e17242) + (locals.var_tmf1 * (-locals.var_q_wdsoi_max_dn17))) * locals.var_dnm) + (assign12900_body70_e17243 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0__blk383, locals.var_tmf0__blk383_dn0, locals.var_tmf0__blk383_dn2, locals.var_tmf0__blk383_dn6, locals.var_tmf0__blk383_dn7, locals.var_tmf0__blk383_dn10, locals.var_tmf0__blk383_dn11, locals.var_tmf0__blk383_dn12, locals.var_tmf0__blk383_dn17,)
    }
};
            locals.var_tmf0__blk383 = assign12900_body70_e17247;
            locals.var_tmf0__blk383_dn0 = assign12900_body70_e17247_d_n0;
            locals.var_tmf0__blk383_dn2 = assign12900_body70_e17247_d_n2;
            locals.var_tmf0__blk383_dn6 = assign12900_body70_e17247_d_n6;
            locals.var_tmf0__blk383_dn7 = assign12900_body70_e17247_d_n7;
            locals.var_tmf0__blk383_dn10 = assign12900_body70_e17247_d_n10;
            locals.var_tmf0__blk383_dn11 = assign12900_body70_e17247_d_n11;
            locals.var_tmf0__blk383_dn12 = assign12900_body70_e17247_d_n12;
            locals.var_tmf0__blk383_dn17 = assign12900_body70_e17247_d_n17;
            let (assign12900_body71_e17265, assign12900_body71_e17265_d_n0, assign12900_body71_e17265_d_n2, assign12900_body71_e17265_d_n6, assign12900_body71_e17265_d_n7, assign12900_body71_e17265_d_n10, assign12900_body71_e17265_d_n11, assign12900_body71_e17265_d_n12, assign12900_body71_e17265_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard392 != 0.0)) {
        let assign12900_body71_e17255: f64 = (-locals.var_q_wdsoi_max);
        let assign12900_body71_e17257: f64 = assign12900_body71_e17255;
        let assign12900_body71_e17259: f64 = (assign12900_body71_e17257 * locals.var_xmp);
        let assign12900_body71_e17261: f64 = (assign12900_body71_e17259 * locals.var_dnm);
        let assign12900_body71_e17263: f64 = (assign12900_body71_e17261 / locals.var_arg);
        (assign12900_body71_e17263, ((((((((-locals.var_q_wdsoi_max_dn0) * locals.var_xmp) + (assign12900_body71_e17257 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign12900_body71_e17259 * locals.var_dnm_dn0)) * locals.var_arg) - (assign12900_body71_e17261 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((-locals.var_q_wdsoi_max_dn2) * locals.var_xmp) + (assign12900_body71_e17257 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign12900_body71_e17259 * locals.var_dnm_dn2)) * locals.var_arg) - (assign12900_body71_e17261 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((-locals.var_q_wdsoi_max_dn6) * locals.var_xmp) + (assign12900_body71_e17257 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign12900_body71_e17259 * locals.var_dnm_dn6)) * locals.var_arg) - (assign12900_body71_e17261 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((-locals.var_q_wdsoi_max_dn7) * locals.var_xmp) + (assign12900_body71_e17257 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign12900_body71_e17259 * locals.var_dnm_dn7)) * locals.var_arg) - (assign12900_body71_e17261 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((-locals.var_q_wdsoi_max_dn10) * locals.var_xmp) + (assign12900_body71_e17257 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign12900_body71_e17259 * locals.var_dnm_dn10)) * locals.var_arg) - (assign12900_body71_e17261 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((-locals.var_q_wdsoi_max_dn11) * locals.var_xmp) + (assign12900_body71_e17257 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign12900_body71_e17259 * locals.var_dnm_dn11)) * locals.var_arg) - (assign12900_body71_e17261 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((((-locals.var_q_wdsoi_max_dn12) * locals.var_xmp) + (assign12900_body71_e17257 * locals.var_xmp_dn12)) * locals.var_dnm) + (assign12900_body71_e17259 * locals.var_dnm_dn12)) * locals.var_arg) - (assign12900_body71_e17261 * locals.var_arg_dn12)) / (locals.var_arg * locals.var_arg)), ((((((((-locals.var_q_wdsoi_max_dn17) * locals.var_xmp) + (assign12900_body71_e17257 * locals.var_xmp_dn17)) * locals.var_dnm) + (assign12900_body71_e17259 * locals.var_dnm_dn17)) * locals.var_arg) - (assign12900_body71_e17261 * locals.var_arg_dn17)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign12900_body71_e17265;
            locals.var_t0_dn0 = assign12900_body71_e17265_d_n0;
            locals.var_t0_dn2 = assign12900_body71_e17265_d_n2;
            locals.var_t0_dn6 = assign12900_body71_e17265_d_n6;
            locals.var_t0_dn7 = assign12900_body71_e17265_d_n7;
            locals.var_t0_dn10 = assign12900_body71_e17265_d_n10;
            locals.var_t0_dn11 = assign12900_body71_e17265_d_n11;
            locals.var_t0_dn12 = assign12900_body71_e17265_d_n12;
            locals.var_t0_dn17 = assign12900_body71_e17265_d_n17;
            let (assign12900_body72_e17281, assign12900_body72_e17281_d_n0, assign12900_body72_e17281_d_n2, assign12900_body72_e17281_d_n6, assign12900_body72_e17281_d_n7, assign12900_body72_e17281_d_n10, assign12900_body72_e17281_d_n11, assign12900_body72_e17281_d_n12, assign12900_body72_e17281_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard392 != 0.0)) {
        let assign12900_body72_e17274: f64 = (-locals.var_q_wdsoi_max);
        let assign12900_body72_e17276: f64 = assign12900_body72_e17274;
        let assign12900_body72_e17277: f64 = (-assign12900_body72_e17276);
        let assign12900_body72_e17279: f64 = (assign12900_body72_e17277 + locals.var_tmf0__blk383);
        (assign12900_body72_e17279, ((-(-locals.var_q_wdsoi_max_dn0)) + locals.var_tmf0__blk383_dn0), ((-(-locals.var_q_wdsoi_max_dn2)) + locals.var_tmf0__blk383_dn2), ((-(-locals.var_q_wdsoi_max_dn6)) + locals.var_tmf0__blk383_dn6), ((-(-locals.var_q_wdsoi_max_dn7)) + locals.var_tmf0__blk383_dn7), ((-(-locals.var_q_wdsoi_max_dn10)) + locals.var_tmf0__blk383_dn10), ((-(-locals.var_q_wdsoi_max_dn11)) + locals.var_tmf0__blk383_dn11), ((-(-locals.var_q_wdsoi_max_dn12)) + locals.var_tmf0__blk383_dn12), ((-(-locals.var_q_wdsoi_max_dn17)) + locals.var_tmf0__blk383_dn17),)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn12, locals.var_q_bl_dep_dn17,)
    }
};
            locals.var_q_bl_dep = assign12900_body72_e17281;
            locals.var_q_bl_dep_dn0 = assign12900_body72_e17281_d_n0;
            locals.var_q_bl_dep_dn2 = assign12900_body72_e17281_d_n2;
            locals.var_q_bl_dep_dn6 = assign12900_body72_e17281_d_n6;
            locals.var_q_bl_dep_dn7 = assign12900_body72_e17281_d_n7;
            locals.var_q_bl_dep_dn10 = assign12900_body72_e17281_d_n10;
            locals.var_q_bl_dep_dn11 = assign12900_body72_e17281_d_n11;
            locals.var_q_bl_dep_dn12 = assign12900_body72_e17281_d_n12;
            locals.var_q_bl_dep_dn17 = assign12900_body72_e17281_d_n17;
            let (assign12900_body73_e17290, assign12900_body73_e17290_d_n0, assign12900_body73_e17290_d_n2, assign12900_body73_e17290_d_n6, assign12900_body73_e17290_d_n7, assign12900_body73_e17290_d_n10, assign12900_body73_e17290_d_n11, assign12900_body73_e17290_d_n12, assign12900_body73_e17290_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard392 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign12900_body73_e17290;
            locals.var_t0_dn0 = assign12900_body73_e17290_d_n0;
            locals.var_t0_dn2 = assign12900_body73_e17290_d_n2;
            locals.var_t0_dn6 = assign12900_body73_e17290_d_n6;
            locals.var_t0_dn7 = assign12900_body73_e17290_d_n7;
            locals.var_t0_dn10 = assign12900_body73_e17290_d_n10;
            locals.var_t0_dn11 = assign12900_body73_e17290_d_n11;
            locals.var_t0_dn12 = assign12900_body73_e17290_d_n12;
            locals.var_t0_dn17 = assign12900_body73_e17290_d_n17;
            let (assign12900_body74_e17300, assign12900_body74_e17300_d_n0, assign12900_body74_e17300_d_n2, assign12900_body74_e17300_d_n6, assign12900_body74_e17300_d_n7, assign12900_body74_e17300_d_n10, assign12900_body74_e17300_d_n11, assign12900_body74_e17300_d_n12, assign12900_body74_e17300_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard392 == 0.0)) {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn12, locals.var_q_bl_dep_dn17,)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn12, locals.var_q_bl_dep_dn17,)
    }
};
            locals.var_q_bl_dep = assign12900_body74_e17300;
            locals.var_q_bl_dep_dn0 = assign12900_body74_e17300_d_n0;
            locals.var_q_bl_dep_dn2 = assign12900_body74_e17300_d_n2;
            locals.var_q_bl_dep_dn6 = assign12900_body74_e17300_d_n6;
            locals.var_q_bl_dep_dn7 = assign12900_body74_e17300_d_n7;
            locals.var_q_bl_dep_dn10 = assign12900_body74_e17300_d_n10;
            locals.var_q_bl_dep_dn11 = assign12900_body74_e17300_d_n11;
            locals.var_q_bl_dep_dn12 = assign12900_body74_e17300_d_n12;
            locals.var_q_bl_dep_dn17 = assign12900_body74_e17300_d_n17;
            let (assign12900_body75_e17310, assign12900_body75_e17310_d_n0, assign12900_body75_e17310_d_n2, assign12900_body75_e17310_d_n6, assign12900_body75_e17310_d_n7, assign12900_body75_e17310_d_n10, assign12900_body75_e17310_d_n11, assign12900_body75_e17310_d_n12, assign12900_body75_e17310_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard392 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign12900_body75_e17310;
            locals.var_t0_dn0 = assign12900_body75_e17310_d_n0;
            locals.var_t0_dn2 = assign12900_body75_e17310_d_n2;
            locals.var_t0_dn6 = assign12900_body75_e17310_d_n6;
            locals.var_t0_dn7 = assign12900_body75_e17310_d_n7;
            locals.var_t0_dn10 = assign12900_body75_e17310_d_n10;
            locals.var_t0_dn11 = assign12900_body75_e17310_d_n11;
            locals.var_t0_dn12 = assign12900_body75_e17310_d_n12;
            locals.var_t0_dn17 = assign12900_body75_e17310_d_n17;
            let (assign12900_body76_e17319, assign12900_body76_e17319_d_n0, assign12900_body76_e17319_d_n2, assign12900_body76_e17319_d_n6, assign12900_body76_e17319_d_n7, assign12900_body76_e17319_d_n10, assign12900_body76_e17319_d_n11, assign12900_body76_e17319_d_n12, assign12900_body76_e17319_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign12900_body76_e17317: f64 = (locals.var_q_bl_dep_dpbs * locals.var_t0);
        (assign12900_body76_e17317, ((locals.var_q_bl_dep_dpbs_dn0 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn0)), ((locals.var_q_bl_dep_dpbs_dn2 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn2)), ((locals.var_q_bl_dep_dpbs_dn6 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn6)), ((locals.var_q_bl_dep_dpbs_dn7 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn7)), ((locals.var_q_bl_dep_dpbs_dn10 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn10)), ((locals.var_q_bl_dep_dpbs_dn11 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn11)), ((locals.var_q_bl_dep_dpbs_dn12 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn12)), ((locals.var_q_bl_dep_dpbs_dn17 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn17)),)
    } else {
        (locals.var_q_bl_dep_dpbs, locals.var_q_bl_dep_dpbs_dn0, locals.var_q_bl_dep_dpbs_dn2, locals.var_q_bl_dep_dpbs_dn6, locals.var_q_bl_dep_dpbs_dn7, locals.var_q_bl_dep_dpbs_dn10, locals.var_q_bl_dep_dpbs_dn11, locals.var_q_bl_dep_dpbs_dn12, locals.var_q_bl_dep_dpbs_dn17,)
    }
};
            locals.var_q_bl_dep_dpbs = assign12900_body76_e17319;
            locals.var_q_bl_dep_dpbs_dn0 = assign12900_body76_e17319_d_n0;
            locals.var_q_bl_dep_dpbs_dn2 = assign12900_body76_e17319_d_n2;
            locals.var_q_bl_dep_dpbs_dn6 = assign12900_body76_e17319_d_n6;
            locals.var_q_bl_dep_dpbs_dn7 = assign12900_body76_e17319_d_n7;
            locals.var_q_bl_dep_dpbs_dn10 = assign12900_body76_e17319_d_n10;
            locals.var_q_bl_dep_dpbs_dn11 = assign12900_body76_e17319_d_n11;
            locals.var_q_bl_dep_dpbs_dn12 = assign12900_body76_e17319_d_n12;
            locals.var_q_bl_dep_dpbs_dn17 = assign12900_body76_e17319_d_n17;
            let (assign12900_body77_e17328, assign12900_body77_e17328_d_n0, assign12900_body77_e17328_d_n2, assign12900_body77_e17328_d_n6, assign12900_body77_e17328_d_n7, assign12900_body77_e17328_d_n10, assign12900_body77_e17328_d_n11, assign12900_body77_e17328_d_n12, assign12900_body77_e17328_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign12900_body77_e17326: f64 = (locals.var_q_bl_dep_dpss * locals.var_t0);
        (assign12900_body77_e17326, ((locals.var_q_bl_dep_dpss_dn0 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn0)), ((locals.var_q_bl_dep_dpss_dn2 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn2)), ((locals.var_q_bl_dep_dpss_dn6 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn6)), ((locals.var_q_bl_dep_dpss_dn7 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn7)), ((locals.var_q_bl_dep_dpss_dn10 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn10)), ((locals.var_q_bl_dep_dpss_dn11 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn11)), ((locals.var_q_bl_dep_dpss_dn12 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn12)), ((locals.var_q_bl_dep_dpss_dn17 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn17)),)
    } else {
        (locals.var_q_bl_dep_dpss, locals.var_q_bl_dep_dpss_dn0, locals.var_q_bl_dep_dpss_dn2, locals.var_q_bl_dep_dpss_dn6, locals.var_q_bl_dep_dpss_dn7, locals.var_q_bl_dep_dpss_dn10, locals.var_q_bl_dep_dpss_dn11, locals.var_q_bl_dep_dpss_dn12, locals.var_q_bl_dep_dpss_dn17,)
    }
};
            locals.var_q_bl_dep_dpss = assign12900_body77_e17328;
            locals.var_q_bl_dep_dpss_dn0 = assign12900_body77_e17328_d_n0;
            locals.var_q_bl_dep_dpss_dn2 = assign12900_body77_e17328_d_n2;
            locals.var_q_bl_dep_dpss_dn6 = assign12900_body77_e17328_d_n6;
            locals.var_q_bl_dep_dpss_dn7 = assign12900_body77_e17328_d_n7;
            locals.var_q_bl_dep_dpss_dn10 = assign12900_body77_e17328_d_n10;
            locals.var_q_bl_dep_dpss_dn11 = assign12900_body77_e17328_d_n11;
            locals.var_q_bl_dep_dpss_dn12 = assign12900_body77_e17328_d_n12;
            locals.var_q_bl_dep_dpss_dn17 = assign12900_body77_e17328_d_n17;
            let assign12900_body78_e17332: f64 = (locals.var_q_fd_soi - locals.var_q_sl_dep);
            let assign12900_body78_e17335: f64 = (locals.var_q_fd_soi - locals.var_q_sl_dep);
            let assign12900_body78_e17336: f64 = (-assign12900_body78_e17335);
            let assign12900_body78_e17338: f64 = assign12900_body78_e17336;
            let assign12900_body78_e17339: f64 = (assign12900_body78_e17332 + assign12900_body78_e17338);
            let assign12900_body78_e17343: f64 = (locals.var_q_fd_soi - locals.var_q_sl_dep);
            let assign12900_body78_e17344: f64 = (-assign12900_body78_e17343);
            let assign12900_body78_e17346: f64 = assign12900_body78_e17344;
            let assign12900_body78_e17349: f64 = if ((locals.var_q_bl_dep < assign12900_body78_e17339) && (assign12900_body78_e17346 >= 0.0)) { 1.0 } else { 0.0 };
            locals.var_guard398 = assign12900_body78_e17349;
            let (assign12900_body79_e17369, assign12900_body79_e17369_d_n0, assign12900_body79_e17369_d_n2, assign12900_body79_e17369_d_n6, assign12900_body79_e17369_d_n7, assign12900_body79_e17369_d_n10, assign12900_body79_e17369_d_n11, assign12900_body79_e17369_d_n12, assign12900_body79_e17369_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard398 != 0.0)) {
        let assign12900_body79_e17358: f64 = (locals.var_q_fd_soi - locals.var_q_sl_dep);
        let assign12900_body79_e17361: f64 = (locals.var_q_fd_soi - locals.var_q_sl_dep);
        let assign12900_body79_e17362: f64 = (-assign12900_body79_e17361);
        let assign12900_body79_e17364: f64 = assign12900_body79_e17362;
        let assign12900_body79_e17365: f64 = (assign12900_body79_e17358 + assign12900_body79_e17364);
        let assign12900_body79_e17367: f64 = (assign12900_body79_e17365 - locals.var_q_bl_dep);
        (assign12900_body79_e17367, (((locals.var_q_fd_soi_dn0 - locals.var_q_sl_dep_dn0) + (-(locals.var_q_fd_soi_dn0 - locals.var_q_sl_dep_dn0))) - locals.var_q_bl_dep_dn0), (((locals.var_q_fd_soi_dn2 - locals.var_q_sl_dep_dn2) + (-(locals.var_q_fd_soi_dn2 - locals.var_q_sl_dep_dn2))) - locals.var_q_bl_dep_dn2), (((locals.var_q_fd_soi_dn6 - locals.var_q_sl_dep_dn6) + (-(locals.var_q_fd_soi_dn6 - locals.var_q_sl_dep_dn6))) - locals.var_q_bl_dep_dn6), (((locals.var_q_fd_soi_dn7 - locals.var_q_sl_dep_dn7) + (-(locals.var_q_fd_soi_dn7 - locals.var_q_sl_dep_dn7))) - locals.var_q_bl_dep_dn7), (((locals.var_q_fd_soi_dn10 - locals.var_q_sl_dep_dn10) + (-(locals.var_q_fd_soi_dn10 - locals.var_q_sl_dep_dn10))) - locals.var_q_bl_dep_dn10), (((locals.var_q_fd_soi_dn11 - locals.var_q_sl_dep_dn11) + (-(locals.var_q_fd_soi_dn11 - locals.var_q_sl_dep_dn11))) - locals.var_q_bl_dep_dn11), (((locals.var_q_fd_soi_dn12 - locals.var_q_sl_dep_dn12) + (-(locals.var_q_fd_soi_dn12 - locals.var_q_sl_dep_dn12))) - locals.var_q_bl_dep_dn12), (((locals.var_q_fd_soi_dn17 - locals.var_q_sl_dep_dn17) + (-(locals.var_q_fd_soi_dn17 - locals.var_q_sl_dep_dn17))) - locals.var_q_bl_dep_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
            locals.var_tmf1 = assign12900_body79_e17369;
            locals.var_tmf1_dn0 = assign12900_body79_e17369_d_n0;
            locals.var_tmf1_dn2 = assign12900_body79_e17369_d_n2;
            locals.var_tmf1_dn6 = assign12900_body79_e17369_d_n6;
            locals.var_tmf1_dn7 = assign12900_body79_e17369_d_n7;
            locals.var_tmf1_dn10 = assign12900_body79_e17369_d_n10;
            locals.var_tmf1_dn11 = assign12900_body79_e17369_d_n11;
            locals.var_tmf1_dn12 = assign12900_body79_e17369_d_n12;
            locals.var_tmf1_dn17 = assign12900_body79_e17369_d_n17;
            let (assign12900_body80_e17380, assign12900_body80_e17380_d_n0, assign12900_body80_e17380_d_n2, assign12900_body80_e17380_d_n6, assign12900_body80_e17380_d_n7, assign12900_body80_e17380_d_n10, assign12900_body80_e17380_d_n11, assign12900_body80_e17380_d_n12, assign12900_body80_e17380_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard398 != 0.0)) {
        let assign12900_body80_e17378: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign12900_body80_e17378, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
            locals.var_x2 = assign12900_body80_e17380;
            locals.var_x2_dn0 = assign12900_body80_e17380_d_n0;
            locals.var_x2_dn2 = assign12900_body80_e17380_d_n2;
            locals.var_x2_dn6 = assign12900_body80_e17380_d_n6;
            locals.var_x2_dn7 = assign12900_body80_e17380_d_n7;
            locals.var_x2_dn10 = assign12900_body80_e17380_d_n10;
            locals.var_x2_dn11 = assign12900_body80_e17380_d_n11;
            locals.var_x2_dn12 = assign12900_body80_e17380_d_n12;
            locals.var_x2_dn17 = assign12900_body80_e17380_d_n17;
            let (assign12900_body81_e17401, assign12900_body81_e17401_d_n0, assign12900_body81_e17401_d_n2, assign12900_body81_e17401_d_n6, assign12900_body81_e17401_d_n7, assign12900_body81_e17401_d_n10, assign12900_body81_e17401_d_n11, assign12900_body81_e17401_d_n12, assign12900_body81_e17401_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard398 != 0.0)) {
        let assign12900_body81_e17389: f64 = (locals.var_q_fd_soi - locals.var_q_sl_dep);
        let assign12900_body81_e17390: f64 = (-assign12900_body81_e17389);
        let assign12900_body81_e17392: f64 = assign12900_body81_e17390;
        let assign12900_body81_e17395: f64 = (locals.var_q_fd_soi - locals.var_q_sl_dep);
        let assign12900_body81_e17396: f64 = (-assign12900_body81_e17395);
        let assign12900_body81_e17398: f64 = assign12900_body81_e17396;
        let assign12900_body81_e17399: f64 = (assign12900_body81_e17392 * assign12900_body81_e17398);
        (assign12900_body81_e17399, (((-(locals.var_q_fd_soi_dn0 - locals.var_q_sl_dep_dn0)) * assign12900_body81_e17398) + (assign12900_body81_e17392 * (-(locals.var_q_fd_soi_dn0 - locals.var_q_sl_dep_dn0)))), (((-(locals.var_q_fd_soi_dn2 - locals.var_q_sl_dep_dn2)) * assign12900_body81_e17398) + (assign12900_body81_e17392 * (-(locals.var_q_fd_soi_dn2 - locals.var_q_sl_dep_dn2)))), (((-(locals.var_q_fd_soi_dn6 - locals.var_q_sl_dep_dn6)) * assign12900_body81_e17398) + (assign12900_body81_e17392 * (-(locals.var_q_fd_soi_dn6 - locals.var_q_sl_dep_dn6)))), (((-(locals.var_q_fd_soi_dn7 - locals.var_q_sl_dep_dn7)) * assign12900_body81_e17398) + (assign12900_body81_e17392 * (-(locals.var_q_fd_soi_dn7 - locals.var_q_sl_dep_dn7)))), (((-(locals.var_q_fd_soi_dn10 - locals.var_q_sl_dep_dn10)) * assign12900_body81_e17398) + (assign12900_body81_e17392 * (-(locals.var_q_fd_soi_dn10 - locals.var_q_sl_dep_dn10)))), (((-(locals.var_q_fd_soi_dn11 - locals.var_q_sl_dep_dn11)) * assign12900_body81_e17398) + (assign12900_body81_e17392 * (-(locals.var_q_fd_soi_dn11 - locals.var_q_sl_dep_dn11)))), (((-(locals.var_q_fd_soi_dn12 - locals.var_q_sl_dep_dn12)) * assign12900_body81_e17398) + (assign12900_body81_e17392 * (-(locals.var_q_fd_soi_dn12 - locals.var_q_sl_dep_dn12)))), (((-(locals.var_q_fd_soi_dn17 - locals.var_q_sl_dep_dn17)) * assign12900_body81_e17398) + (assign12900_body81_e17392 * (-(locals.var_q_fd_soi_dn17 - locals.var_q_sl_dep_dn17)))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
            locals.var_xmax2 = assign12900_body81_e17401;
            locals.var_xmax2_dn0 = assign12900_body81_e17401_d_n0;
            locals.var_xmax2_dn2 = assign12900_body81_e17401_d_n2;
            locals.var_xmax2_dn6 = assign12900_body81_e17401_d_n6;
            locals.var_xmax2_dn7 = assign12900_body81_e17401_d_n7;
            locals.var_xmax2_dn10 = assign12900_body81_e17401_d_n10;
            locals.var_xmax2_dn11 = assign12900_body81_e17401_d_n11;
            locals.var_xmax2_dn12 = assign12900_body81_e17401_d_n12;
            locals.var_xmax2_dn17 = assign12900_body81_e17401_d_n17;
            let (assign12900_body82_e17410, assign12900_body82_e17410_d_n0, assign12900_body82_e17410_d_n2, assign12900_body82_e17410_d_n6, assign12900_body82_e17410_d_n7, assign12900_body82_e17410_d_n10, assign12900_body82_e17410_d_n11, assign12900_body82_e17410_d_n12, assign12900_body82_e17410_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard398 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
            locals.var_xp = assign12900_body82_e17410;
            locals.var_xp_dn0 = assign12900_body82_e17410_d_n0;
            locals.var_xp_dn2 = assign12900_body82_e17410_d_n2;
            locals.var_xp_dn6 = assign12900_body82_e17410_d_n6;
            locals.var_xp_dn7 = assign12900_body82_e17410_d_n7;
            locals.var_xp_dn10 = assign12900_body82_e17410_d_n10;
            locals.var_xp_dn11 = assign12900_body82_e17410_d_n11;
            locals.var_xp_dn12 = assign12900_body82_e17410_d_n12;
            locals.var_xp_dn17 = assign12900_body82_e17410_d_n17;
            let (assign12900_body83_e17419, assign12900_body83_e17419_d_n0, assign12900_body83_e17419_d_n2, assign12900_body83_e17419_d_n6, assign12900_body83_e17419_d_n7, assign12900_body83_e17419_d_n10, assign12900_body83_e17419_d_n11, assign12900_body83_e17419_d_n12, assign12900_body83_e17419_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard398 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
            locals.var_xmp = assign12900_body83_e17419;
            locals.var_xmp_dn0 = assign12900_body83_e17419_d_n0;
            locals.var_xmp_dn2 = assign12900_body83_e17419_d_n2;
            locals.var_xmp_dn6 = assign12900_body83_e17419_d_n6;
            locals.var_xmp_dn7 = assign12900_body83_e17419_d_n7;
            locals.var_xmp_dn10 = assign12900_body83_e17419_d_n10;
            locals.var_xmp_dn11 = assign12900_body83_e17419_d_n11;
            locals.var_xmp_dn12 = assign12900_body83_e17419_d_n12;
            locals.var_xmp_dn17 = assign12900_body83_e17419_d_n17;
            let (assign12900_body84_e17428,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard398 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign12900_body84_e17428;
            let (assign12900_body85_e17437,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard398 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign12900_body85_e17437;
            let (assign12900_body86_e17446, assign12900_body86_e17446_d_n0, assign12900_body86_e17446_d_n2, assign12900_body86_e17446_d_n6, assign12900_body86_e17446_d_n7, assign12900_body86_e17446_d_n10, assign12900_body86_e17446_d_n11, assign12900_body86_e17446_d_n12, assign12900_body86_e17446_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard398 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
            locals.var_arg = assign12900_body86_e17446;
            locals.var_arg_dn0 = assign12900_body86_e17446_d_n0;
            locals.var_arg_dn2 = assign12900_body86_e17446_d_n2;
            locals.var_arg_dn6 = assign12900_body86_e17446_d_n6;
            locals.var_arg_dn7 = assign12900_body86_e17446_d_n7;
            locals.var_arg_dn10 = assign12900_body86_e17446_d_n10;
            locals.var_arg_dn11 = assign12900_body86_e17446_d_n11;
            locals.var_arg_dn12 = assign12900_body86_e17446_d_n12;
            locals.var_arg_dn17 = assign12900_body86_e17446_d_n17;
            let (assign12900_body87_e17455, assign12900_body87_e17455_d_n0, assign12900_body87_e17455_d_n2, assign12900_body87_e17455_d_n6, assign12900_body87_e17455_d_n7, assign12900_body87_e17455_d_n10, assign12900_body87_e17455_d_n11, assign12900_body87_e17455_d_n12, assign12900_body87_e17455_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard398 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign12900_body87_e17455;
            locals.var_dnm_dn0 = assign12900_body87_e17455_d_n0;
            locals.var_dnm_dn2 = assign12900_body87_e17455_d_n2;
            locals.var_dnm_dn6 = assign12900_body87_e17455_d_n6;
            locals.var_dnm_dn7 = assign12900_body87_e17455_d_n7;
            locals.var_dnm_dn10 = assign12900_body87_e17455_d_n10;
            locals.var_dnm_dn11 = assign12900_body87_e17455_d_n11;
            locals.var_dnm_dn12 = assign12900_body87_e17455_d_n12;
            locals.var_dnm_dn17 = assign12900_body87_e17455_d_n17;
            let (assign12900_body88_e17466, assign12900_body88_e17466_d_n0, assign12900_body88_e17466_d_n2, assign12900_body88_e17466_d_n6, assign12900_body88_e17466_d_n7, assign12900_body88_e17466_d_n10, assign12900_body88_e17466_d_n11, assign12900_body88_e17466_d_n12, assign12900_body88_e17466_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard398 != 0.0)) {
        let assign12900_body88_e17464: f64 = (locals.var_xp * locals.var_x2);
        (assign12900_body88_e17464, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
            locals.var_xp = assign12900_body88_e17466;
            locals.var_xp_dn0 = assign12900_body88_e17466_d_n0;
            locals.var_xp_dn2 = assign12900_body88_e17466_d_n2;
            locals.var_xp_dn6 = assign12900_body88_e17466_d_n6;
            locals.var_xp_dn7 = assign12900_body88_e17466_d_n7;
            locals.var_xp_dn10 = assign12900_body88_e17466_d_n10;
            locals.var_xp_dn11 = assign12900_body88_e17466_d_n11;
            locals.var_xp_dn12 = assign12900_body88_e17466_d_n12;
            locals.var_xp_dn17 = assign12900_body88_e17466_d_n17;
            let (assign12900_body89_e17477, assign12900_body89_e17477_d_n0, assign12900_body89_e17477_d_n2, assign12900_body89_e17477_d_n6, assign12900_body89_e17477_d_n7, assign12900_body89_e17477_d_n10, assign12900_body89_e17477_d_n11, assign12900_body89_e17477_d_n12, assign12900_body89_e17477_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard398 != 0.0)) {
        let assign12900_body89_e17475: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign12900_body89_e17475, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
            locals.var_xmp = assign12900_body89_e17477;
            locals.var_xmp_dn0 = assign12900_body89_e17477_d_n0;
            locals.var_xmp_dn2 = assign12900_body89_e17477_d_n2;
            locals.var_xmp_dn6 = assign12900_body89_e17477_d_n6;
            locals.var_xmp_dn7 = assign12900_body89_e17477_d_n7;
            locals.var_xmp_dn10 = assign12900_body89_e17477_d_n10;
            locals.var_xmp_dn11 = assign12900_body89_e17477_d_n11;
            locals.var_xmp_dn12 = assign12900_body89_e17477_d_n12;
            locals.var_xmp_dn17 = assign12900_body89_e17477_d_n17;
            let (assign12900_body90_e17488, assign12900_body90_e17488_d_n0, assign12900_body90_e17488_d_n2, assign12900_body90_e17488_d_n6, assign12900_body90_e17488_d_n7, assign12900_body90_e17488_d_n10, assign12900_body90_e17488_d_n11, assign12900_body90_e17488_d_n12, assign12900_body90_e17488_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard398 != 0.0)) {
        let assign12900_body90_e17486: f64 = (locals.var_xp * locals.var_x2);
        (assign12900_body90_e17486, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
            locals.var_xp = assign12900_body90_e17488;
            locals.var_xp_dn0 = assign12900_body90_e17488_d_n0;
            locals.var_xp_dn2 = assign12900_body90_e17488_d_n2;
            locals.var_xp_dn6 = assign12900_body90_e17488_d_n6;
            locals.var_xp_dn7 = assign12900_body90_e17488_d_n7;
            locals.var_xp_dn10 = assign12900_body90_e17488_d_n10;
            locals.var_xp_dn11 = assign12900_body90_e17488_d_n11;
            locals.var_xp_dn12 = assign12900_body90_e17488_d_n12;
            locals.var_xp_dn17 = assign12900_body90_e17488_d_n17;
            let (assign12900_body91_e17499, assign12900_body91_e17499_d_n0, assign12900_body91_e17499_d_n2, assign12900_body91_e17499_d_n6, assign12900_body91_e17499_d_n7, assign12900_body91_e17499_d_n10, assign12900_body91_e17499_d_n11, assign12900_body91_e17499_d_n12, assign12900_body91_e17499_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard398 != 0.0)) {
        let assign12900_body91_e17497: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign12900_body91_e17497, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
            locals.var_xmp = assign12900_body91_e17499;
            locals.var_xmp_dn0 = assign12900_body91_e17499_d_n0;
            locals.var_xmp_dn2 = assign12900_body91_e17499_d_n2;
            locals.var_xmp_dn6 = assign12900_body91_e17499_d_n6;
            locals.var_xmp_dn7 = assign12900_body91_e17499_d_n7;
            locals.var_xmp_dn10 = assign12900_body91_e17499_d_n10;
            locals.var_xmp_dn11 = assign12900_body91_e17499_d_n11;
            locals.var_xmp_dn12 = assign12900_body91_e17499_d_n12;
            locals.var_xmp_dn17 = assign12900_body91_e17499_d_n17;
            let (assign12900_body92_e17510, assign12900_body92_e17510_d_n0, assign12900_body92_e17510_d_n2, assign12900_body92_e17510_d_n6, assign12900_body92_e17510_d_n7, assign12900_body92_e17510_d_n10, assign12900_body92_e17510_d_n11, assign12900_body92_e17510_d_n12, assign12900_body92_e17510_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard398 != 0.0)) {
        let assign12900_body92_e17508: f64 = (locals.var_xp + locals.var_xmp);
        (assign12900_body92_e17508, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
            locals.var_arg = assign12900_body92_e17510;
            locals.var_arg_dn0 = assign12900_body92_e17510_d_n0;
            locals.var_arg_dn2 = assign12900_body92_e17510_d_n2;
            locals.var_arg_dn6 = assign12900_body92_e17510_d_n6;
            locals.var_arg_dn7 = assign12900_body92_e17510_d_n7;
            locals.var_arg_dn10 = assign12900_body92_e17510_d_n10;
            locals.var_arg_dn11 = assign12900_body92_e17510_d_n11;
            locals.var_arg_dn12 = assign12900_body92_e17510_d_n12;
            locals.var_arg_dn17 = assign12900_body92_e17510_d_n17;
            let (assign12900_body93_e17519, assign12900_body93_e17519_d_n0, assign12900_body93_e17519_d_n2, assign12900_body93_e17519_d_n6, assign12900_body93_e17519_d_n7, assign12900_body93_e17519_d_n10, assign12900_body93_e17519_d_n11, assign12900_body93_e17519_d_n12, assign12900_body93_e17519_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard398 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign12900_body93_e17519;
            locals.var_dnm_dn0 = assign12900_body93_e17519_d_n0;
            locals.var_dnm_dn2 = assign12900_body93_e17519_d_n2;
            locals.var_dnm_dn6 = assign12900_body93_e17519_d_n6;
            locals.var_dnm_dn7 = assign12900_body93_e17519_d_n7;
            locals.var_dnm_dn10 = assign12900_body93_e17519_d_n10;
            locals.var_dnm_dn11 = assign12900_body93_e17519_d_n11;
            locals.var_dnm_dn12 = assign12900_body93_e17519_d_n12;
            locals.var_dnm_dn17 = assign12900_body93_e17519_d_n17;
            let assign12900_body94_e17534: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
            locals.var_guard399 = assign12900_body94_e17534;
            let assign12900_body95_e17537: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard400 = assign12900_body95_e17537;
            let (assign12900_body96_e17550,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard398 != 0.0)) && (locals.var_guard399 != 0.0)) && (locals.var_guard400 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign12900_body96_e17550;
            let assign12900_body97_e17553: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
            locals.var_guard401 = assign12900_body97_e17553;
            let (assign12900_body98_e17569,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard398 != 0.0)) && (locals.var_guard399 != 0.0)) && (locals.var_guard400 == 0.0)) && (locals.var_guard401 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign12900_body98_e17569;
            let assign12900_body99_e17572: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
            locals.var_guard402 = assign12900_body99_e17572;
            let (assign12900_body100_e17591,) = {
    if (((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard398 != 0.0)) && (locals.var_guard399 != 0.0)) && (locals.var_guard400 == 0.0)) && (locals.var_guard401 == 0.0)) && (locals.var_guard402 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign12900_body100_e17591;
            let assign12900_body101_e17594: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
            locals.var_guard403 = assign12900_body101_e17594;
            let (assign12900_body102_e17616,) = {
    if ((((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard398 != 0.0)) && (locals.var_guard399 != 0.0)) && (locals.var_guard400 == 0.0)) && (locals.var_guard401 == 0.0)) && (locals.var_guard402 == 0.0)) && (locals.var_guard403 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign12900_body102_e17616;
            let (assign12900_body103_e17627,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard398 != 0.0)) && (locals.var_guard399 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign12900_body103_e17627;
            let mut assign12900_body104_loop_guard: usize = 0;
            while {
                let assign12900_body104_cond_e17639: f64 = if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard398 != 0.0)) && (locals.var_guard399 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
                assign12900_body104_cond_e17639 != 0.0
            } {
                assign12900_body104_loop_guard += 1;
                assert!(assign12900_body104_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                let (assign12900_body104_body0_e17651, assign12900_body104_body0_e17651_d_n0, assign12900_body104_body0_e17651_d_n2, assign12900_body104_body0_e17651_d_n6, assign12900_body104_body0_e17651_d_n7, assign12900_body104_body0_e17651_d_n10, assign12900_body104_body0_e17651_d_n11, assign12900_body104_body0_e17651_d_n12, assign12900_body104_body0_e17651_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard398 != 0.0)) && (locals.var_guard399 != 0.0)) {
        let assign12900_body104_body0_e17649: f64 = (locals.var_dnm).sqrt();
        (assign12900_body104_body0_e17649, (locals.var_dnm_dn0 / (2.0 * assign12900_body104_body0_e17649)), (locals.var_dnm_dn2 / (2.0 * assign12900_body104_body0_e17649)), (locals.var_dnm_dn6 / (2.0 * assign12900_body104_body0_e17649)), (locals.var_dnm_dn7 / (2.0 * assign12900_body104_body0_e17649)), (locals.var_dnm_dn10 / (2.0 * assign12900_body104_body0_e17649)), (locals.var_dnm_dn11 / (2.0 * assign12900_body104_body0_e17649)), (locals.var_dnm_dn12 / (2.0 * assign12900_body104_body0_e17649)), (locals.var_dnm_dn17 / (2.0 * assign12900_body104_body0_e17649)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
                locals.var_dnm = assign12900_body104_body0_e17651;
                locals.var_dnm_dn0 = assign12900_body104_body0_e17651_d_n0;
                locals.var_dnm_dn2 = assign12900_body104_body0_e17651_d_n2;
                locals.var_dnm_dn6 = assign12900_body104_body0_e17651_d_n6;
                locals.var_dnm_dn7 = assign12900_body104_body0_e17651_d_n7;
                locals.var_dnm_dn10 = assign12900_body104_body0_e17651_d_n10;
                locals.var_dnm_dn11 = assign12900_body104_body0_e17651_d_n11;
                locals.var_dnm_dn12 = assign12900_body104_body0_e17651_d_n12;
                locals.var_dnm_dn17 = assign12900_body104_body0_e17651_d_n17;
                let (assign12900_body104_body1_e17664,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard398 != 0.0)) && (locals.var_guard399 != 0.0)) {
        let assign12900_body104_body1_e17662: f64 = (locals.var_m0 + 1.0);
        (assign12900_body104_body1_e17662,)
    } else {
        (locals.var_m0,)
    }
};
                locals.var_m0 = assign12900_body104_body1_e17664;
            }
            let (assign12900_body105_e17682, assign12900_body105_e17682_d_n0, assign12900_body105_e17682_d_n2, assign12900_body105_e17682_d_n6, assign12900_body105_e17682_d_n7, assign12900_body105_e17682_d_n10, assign12900_body105_e17682_d_n11, assign12900_body105_e17682_d_n12, assign12900_body105_e17682_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard398 != 0.0)) && (locals.var_guard399 == 0.0)) {
        let assign12900_body105_e17678: f64 = (2.0 * 2.0);
        let assign12900_body105_e17679: f64 = (1.0 / assign12900_body105_e17678);
        let assign12900_body105_e17680: f64 = (locals.var_dnm).powf(assign12900_body105_e17679);
        (assign12900_body105_e17680, if 0.0 == 0.0 && ((assign12900_body105_e17679) as f64).is_finite() && ((assign12900_body105_e17679) as f64).fract() == 0.0 { if assign12900_body105_e17679 == 0.0 { 0.0 } else { (assign12900_body105_e17679 * ((locals.var_dnm).powf(assign12900_body105_e17679 - 1.0) * locals.var_dnm_dn0)) } } else { (assign12900_body105_e17680 * (assign12900_body105_e17679 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12900_body105_e17679) as f64).is_finite() && ((assign12900_body105_e17679) as f64).fract() == 0.0 { if assign12900_body105_e17679 == 0.0 { 0.0 } else { (assign12900_body105_e17679 * ((locals.var_dnm).powf(assign12900_body105_e17679 - 1.0) * locals.var_dnm_dn2)) } } else { (assign12900_body105_e17680 * (assign12900_body105_e17679 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12900_body105_e17679) as f64).is_finite() && ((assign12900_body105_e17679) as f64).fract() == 0.0 { if assign12900_body105_e17679 == 0.0 { 0.0 } else { (assign12900_body105_e17679 * ((locals.var_dnm).powf(assign12900_body105_e17679 - 1.0) * locals.var_dnm_dn6)) } } else { (assign12900_body105_e17680 * (assign12900_body105_e17679 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12900_body105_e17679) as f64).is_finite() && ((assign12900_body105_e17679) as f64).fract() == 0.0 { if assign12900_body105_e17679 == 0.0 { 0.0 } else { (assign12900_body105_e17679 * ((locals.var_dnm).powf(assign12900_body105_e17679 - 1.0) * locals.var_dnm_dn7)) } } else { (assign12900_body105_e17680 * (assign12900_body105_e17679 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12900_body105_e17679) as f64).is_finite() && ((assign12900_body105_e17679) as f64).fract() == 0.0 { if assign12900_body105_e17679 == 0.0 { 0.0 } else { (assign12900_body105_e17679 * ((locals.var_dnm).powf(assign12900_body105_e17679 - 1.0) * locals.var_dnm_dn10)) } } else { (assign12900_body105_e17680 * (assign12900_body105_e17679 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12900_body105_e17679) as f64).is_finite() && ((assign12900_body105_e17679) as f64).fract() == 0.0 { if assign12900_body105_e17679 == 0.0 { 0.0 } else { (assign12900_body105_e17679 * ((locals.var_dnm).powf(assign12900_body105_e17679 - 1.0) * locals.var_dnm_dn11)) } } else { (assign12900_body105_e17680 * (assign12900_body105_e17679 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12900_body105_e17679) as f64).is_finite() && ((assign12900_body105_e17679) as f64).fract() == 0.0 { if assign12900_body105_e17679 == 0.0 { 0.0 } else { (assign12900_body105_e17679 * ((locals.var_dnm).powf(assign12900_body105_e17679 - 1.0) * locals.var_dnm_dn12)) } } else { (assign12900_body105_e17680 * (assign12900_body105_e17679 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign12900_body105_e17679) as f64).is_finite() && ((assign12900_body105_e17679) as f64).fract() == 0.0 { if assign12900_body105_e17679 == 0.0 { 0.0 } else { (assign12900_body105_e17679 * ((locals.var_dnm).powf(assign12900_body105_e17679 - 1.0) * locals.var_dnm_dn17)) } } else { (assign12900_body105_e17680 * (assign12900_body105_e17679 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign12900_body105_e17682;
            locals.var_dnm_dn0 = assign12900_body105_e17682_d_n0;
            locals.var_dnm_dn2 = assign12900_body105_e17682_d_n2;
            locals.var_dnm_dn6 = assign12900_body105_e17682_d_n6;
            locals.var_dnm_dn7 = assign12900_body105_e17682_d_n7;
            locals.var_dnm_dn10 = assign12900_body105_e17682_d_n10;
            locals.var_dnm_dn11 = assign12900_body105_e17682_d_n11;
            locals.var_dnm_dn12 = assign12900_body105_e17682_d_n12;
            locals.var_dnm_dn17 = assign12900_body105_e17682_d_n17;
            let (assign12900_body106_e17693, assign12900_body106_e17693_d_n0, assign12900_body106_e17693_d_n2, assign12900_body106_e17693_d_n6, assign12900_body106_e17693_d_n7, assign12900_body106_e17693_d_n10, assign12900_body106_e17693_d_n11, assign12900_body106_e17693_d_n12, assign12900_body106_e17693_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard398 != 0.0)) {
        let assign12900_body106_e17691: f64 = (1.0 / locals.var_dnm);
        (assign12900_body106_e17691, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign12900_body106_e17693;
            locals.var_dnm_dn0 = assign12900_body106_e17693_d_n0;
            locals.var_dnm_dn2 = assign12900_body106_e17693_d_n2;
            locals.var_dnm_dn6 = assign12900_body106_e17693_d_n6;
            locals.var_dnm_dn7 = assign12900_body106_e17693_d_n7;
            locals.var_dnm_dn10 = assign12900_body106_e17693_d_n10;
            locals.var_dnm_dn11 = assign12900_body106_e17693_d_n11;
            locals.var_dnm_dn12 = assign12900_body106_e17693_d_n12;
            locals.var_dnm_dn17 = assign12900_body106_e17693_d_n17;
            let (assign12900_body107_e17711, assign12900_body107_e17711_d_n0, assign12900_body107_e17711_d_n2, assign12900_body107_e17711_d_n6, assign12900_body107_e17711_d_n7, assign12900_body107_e17711_d_n10, assign12900_body107_e17711_d_n11, assign12900_body107_e17711_d_n12, assign12900_body107_e17711_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard398 != 0.0)) {
        let assign12900_body107_e17703: f64 = (locals.var_q_fd_soi - locals.var_q_sl_dep);
        let assign12900_body107_e17704: f64 = (-assign12900_body107_e17703);
        let assign12900_body107_e17706: f64 = assign12900_body107_e17704;
        let assign12900_body107_e17707: f64 = (locals.var_tmf1 * assign12900_body107_e17706);
        let assign12900_body107_e17709: f64 = (assign12900_body107_e17707 * locals.var_dnm);
        (assign12900_body107_e17709, ((((locals.var_tmf1_dn0 * assign12900_body107_e17706) + (locals.var_tmf1 * (-(locals.var_q_fd_soi_dn0 - locals.var_q_sl_dep_dn0)))) * locals.var_dnm) + (assign12900_body107_e17707 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign12900_body107_e17706) + (locals.var_tmf1 * (-(locals.var_q_fd_soi_dn2 - locals.var_q_sl_dep_dn2)))) * locals.var_dnm) + (assign12900_body107_e17707 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn6 * assign12900_body107_e17706) + (locals.var_tmf1 * (-(locals.var_q_fd_soi_dn6 - locals.var_q_sl_dep_dn6)))) * locals.var_dnm) + (assign12900_body107_e17707 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign12900_body107_e17706) + (locals.var_tmf1 * (-(locals.var_q_fd_soi_dn7 - locals.var_q_sl_dep_dn7)))) * locals.var_dnm) + (assign12900_body107_e17707 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn10 * assign12900_body107_e17706) + (locals.var_tmf1 * (-(locals.var_q_fd_soi_dn10 - locals.var_q_sl_dep_dn10)))) * locals.var_dnm) + (assign12900_body107_e17707 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * assign12900_body107_e17706) + (locals.var_tmf1 * (-(locals.var_q_fd_soi_dn11 - locals.var_q_sl_dep_dn11)))) * locals.var_dnm) + (assign12900_body107_e17707 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn12 * assign12900_body107_e17706) + (locals.var_tmf1 * (-(locals.var_q_fd_soi_dn12 - locals.var_q_sl_dep_dn12)))) * locals.var_dnm) + (assign12900_body107_e17707 * locals.var_dnm_dn12)), ((((locals.var_tmf1_dn17 * assign12900_body107_e17706) + (locals.var_tmf1 * (-(locals.var_q_fd_soi_dn17 - locals.var_q_sl_dep_dn17)))) * locals.var_dnm) + (assign12900_body107_e17707 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0__blk383, locals.var_tmf0__blk383_dn0, locals.var_tmf0__blk383_dn2, locals.var_tmf0__blk383_dn6, locals.var_tmf0__blk383_dn7, locals.var_tmf0__blk383_dn10, locals.var_tmf0__blk383_dn11, locals.var_tmf0__blk383_dn12, locals.var_tmf0__blk383_dn17,)
    }
};
            locals.var_tmf0__blk383 = assign12900_body107_e17711;
            locals.var_tmf0__blk383_dn0 = assign12900_body107_e17711_d_n0;
            locals.var_tmf0__blk383_dn2 = assign12900_body107_e17711_d_n2;
            locals.var_tmf0__blk383_dn6 = assign12900_body107_e17711_d_n6;
            locals.var_tmf0__blk383_dn7 = assign12900_body107_e17711_d_n7;
            locals.var_tmf0__blk383_dn10 = assign12900_body107_e17711_d_n10;
            locals.var_tmf0__blk383_dn11 = assign12900_body107_e17711_d_n11;
            locals.var_tmf0__blk383_dn12 = assign12900_body107_e17711_d_n12;
            locals.var_tmf0__blk383_dn17 = assign12900_body107_e17711_d_n17;
            let (assign12900_body108_e17731, assign12900_body108_e17731_d_n0, assign12900_body108_e17731_d_n2, assign12900_body108_e17731_d_n6, assign12900_body108_e17731_d_n7, assign12900_body108_e17731_d_n10, assign12900_body108_e17731_d_n11, assign12900_body108_e17731_d_n12, assign12900_body108_e17731_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard398 != 0.0)) {
        let assign12900_body108_e17720: f64 = (locals.var_q_fd_soi - locals.var_q_sl_dep);
        let assign12900_body108_e17721: f64 = (-assign12900_body108_e17720);
        let assign12900_body108_e17723: f64 = assign12900_body108_e17721;
        let assign12900_body108_e17725: f64 = (assign12900_body108_e17723 * locals.var_xmp);
        let assign12900_body108_e17727: f64 = (assign12900_body108_e17725 * locals.var_dnm);
        let assign12900_body108_e17729: f64 = (assign12900_body108_e17727 / locals.var_arg);
        (assign12900_body108_e17729, ((((((((-(locals.var_q_fd_soi_dn0 - locals.var_q_sl_dep_dn0)) * locals.var_xmp) + (assign12900_body108_e17723 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign12900_body108_e17725 * locals.var_dnm_dn0)) * locals.var_arg) - (assign12900_body108_e17727 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((-(locals.var_q_fd_soi_dn2 - locals.var_q_sl_dep_dn2)) * locals.var_xmp) + (assign12900_body108_e17723 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign12900_body108_e17725 * locals.var_dnm_dn2)) * locals.var_arg) - (assign12900_body108_e17727 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((-(locals.var_q_fd_soi_dn6 - locals.var_q_sl_dep_dn6)) * locals.var_xmp) + (assign12900_body108_e17723 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign12900_body108_e17725 * locals.var_dnm_dn6)) * locals.var_arg) - (assign12900_body108_e17727 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((-(locals.var_q_fd_soi_dn7 - locals.var_q_sl_dep_dn7)) * locals.var_xmp) + (assign12900_body108_e17723 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign12900_body108_e17725 * locals.var_dnm_dn7)) * locals.var_arg) - (assign12900_body108_e17727 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((-(locals.var_q_fd_soi_dn10 - locals.var_q_sl_dep_dn10)) * locals.var_xmp) + (assign12900_body108_e17723 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign12900_body108_e17725 * locals.var_dnm_dn10)) * locals.var_arg) - (assign12900_body108_e17727 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((-(locals.var_q_fd_soi_dn11 - locals.var_q_sl_dep_dn11)) * locals.var_xmp) + (assign12900_body108_e17723 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign12900_body108_e17725 * locals.var_dnm_dn11)) * locals.var_arg) - (assign12900_body108_e17727 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((((-(locals.var_q_fd_soi_dn12 - locals.var_q_sl_dep_dn12)) * locals.var_xmp) + (assign12900_body108_e17723 * locals.var_xmp_dn12)) * locals.var_dnm) + (assign12900_body108_e17725 * locals.var_dnm_dn12)) * locals.var_arg) - (assign12900_body108_e17727 * locals.var_arg_dn12)) / (locals.var_arg * locals.var_arg)), ((((((((-(locals.var_q_fd_soi_dn17 - locals.var_q_sl_dep_dn17)) * locals.var_xmp) + (assign12900_body108_e17723 * locals.var_xmp_dn17)) * locals.var_dnm) + (assign12900_body108_e17725 * locals.var_dnm_dn17)) * locals.var_arg) - (assign12900_body108_e17727 * locals.var_arg_dn17)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign12900_body108_e17731;
            locals.var_t0_dn0 = assign12900_body108_e17731_d_n0;
            locals.var_t0_dn2 = assign12900_body108_e17731_d_n2;
            locals.var_t0_dn6 = assign12900_body108_e17731_d_n6;
            locals.var_t0_dn7 = assign12900_body108_e17731_d_n7;
            locals.var_t0_dn10 = assign12900_body108_e17731_d_n10;
            locals.var_t0_dn11 = assign12900_body108_e17731_d_n11;
            locals.var_t0_dn12 = assign12900_body108_e17731_d_n12;
            locals.var_t0_dn17 = assign12900_body108_e17731_d_n17;
            let (assign12900_body109_e17751, assign12900_body109_e17751_d_n0, assign12900_body109_e17751_d_n2, assign12900_body109_e17751_d_n6, assign12900_body109_e17751_d_n7, assign12900_body109_e17751_d_n10, assign12900_body109_e17751_d_n11, assign12900_body109_e17751_d_n12, assign12900_body109_e17751_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard398 != 0.0)) {
        let assign12900_body109_e17740: f64 = (locals.var_q_fd_soi - locals.var_q_sl_dep);
        let assign12900_body109_e17743: f64 = (locals.var_q_fd_soi - locals.var_q_sl_dep);
        let assign12900_body109_e17744: f64 = (-assign12900_body109_e17743);
        let assign12900_body109_e17746: f64 = assign12900_body109_e17744;
        let assign12900_body109_e17747: f64 = (assign12900_body109_e17740 + assign12900_body109_e17746);
        let assign12900_body109_e17749: f64 = (assign12900_body109_e17747 - locals.var_tmf0__blk383);
        (assign12900_body109_e17749, (((locals.var_q_fd_soi_dn0 - locals.var_q_sl_dep_dn0) + (-(locals.var_q_fd_soi_dn0 - locals.var_q_sl_dep_dn0))) - locals.var_tmf0__blk383_dn0), (((locals.var_q_fd_soi_dn2 - locals.var_q_sl_dep_dn2) + (-(locals.var_q_fd_soi_dn2 - locals.var_q_sl_dep_dn2))) - locals.var_tmf0__blk383_dn2), (((locals.var_q_fd_soi_dn6 - locals.var_q_sl_dep_dn6) + (-(locals.var_q_fd_soi_dn6 - locals.var_q_sl_dep_dn6))) - locals.var_tmf0__blk383_dn6), (((locals.var_q_fd_soi_dn7 - locals.var_q_sl_dep_dn7) + (-(locals.var_q_fd_soi_dn7 - locals.var_q_sl_dep_dn7))) - locals.var_tmf0__blk383_dn7), (((locals.var_q_fd_soi_dn10 - locals.var_q_sl_dep_dn10) + (-(locals.var_q_fd_soi_dn10 - locals.var_q_sl_dep_dn10))) - locals.var_tmf0__blk383_dn10), (((locals.var_q_fd_soi_dn11 - locals.var_q_sl_dep_dn11) + (-(locals.var_q_fd_soi_dn11 - locals.var_q_sl_dep_dn11))) - locals.var_tmf0__blk383_dn11), (((locals.var_q_fd_soi_dn12 - locals.var_q_sl_dep_dn12) + (-(locals.var_q_fd_soi_dn12 - locals.var_q_sl_dep_dn12))) - locals.var_tmf0__blk383_dn12), (((locals.var_q_fd_soi_dn17 - locals.var_q_sl_dep_dn17) + (-(locals.var_q_fd_soi_dn17 - locals.var_q_sl_dep_dn17))) - locals.var_tmf0__blk383_dn17),)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn12, locals.var_q_bl_dep_dn17,)
    }
};
            locals.var_q_bl_dep = assign12900_body109_e17751;
            locals.var_q_bl_dep_dn0 = assign12900_body109_e17751_d_n0;
            locals.var_q_bl_dep_dn2 = assign12900_body109_e17751_d_n2;
            locals.var_q_bl_dep_dn6 = assign12900_body109_e17751_d_n6;
            locals.var_q_bl_dep_dn7 = assign12900_body109_e17751_d_n7;
            locals.var_q_bl_dep_dn10 = assign12900_body109_e17751_d_n10;
            locals.var_q_bl_dep_dn11 = assign12900_body109_e17751_d_n11;
            locals.var_q_bl_dep_dn12 = assign12900_body109_e17751_d_n12;
            locals.var_q_bl_dep_dn17 = assign12900_body109_e17751_d_n17;
            let (assign12900_body110_e17760, assign12900_body110_e17760_d_n0, assign12900_body110_e17760_d_n2, assign12900_body110_e17760_d_n6, assign12900_body110_e17760_d_n7, assign12900_body110_e17760_d_n10, assign12900_body110_e17760_d_n11, assign12900_body110_e17760_d_n12, assign12900_body110_e17760_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard398 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign12900_body110_e17760;
            locals.var_t0_dn0 = assign12900_body110_e17760_d_n0;
            locals.var_t0_dn2 = assign12900_body110_e17760_d_n2;
            locals.var_t0_dn6 = assign12900_body110_e17760_d_n6;
            locals.var_t0_dn7 = assign12900_body110_e17760_d_n7;
            locals.var_t0_dn10 = assign12900_body110_e17760_d_n10;
            locals.var_t0_dn11 = assign12900_body110_e17760_d_n11;
            locals.var_t0_dn12 = assign12900_body110_e17760_d_n12;
            locals.var_t0_dn17 = assign12900_body110_e17760_d_n17;
            let (assign12900_body111_e17770, assign12900_body111_e17770_d_n0, assign12900_body111_e17770_d_n2, assign12900_body111_e17770_d_n6, assign12900_body111_e17770_d_n7, assign12900_body111_e17770_d_n10, assign12900_body111_e17770_d_n11, assign12900_body111_e17770_d_n12, assign12900_body111_e17770_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard398 == 0.0)) {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn12, locals.var_q_bl_dep_dn17,)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn11, locals.var_q_bl_dep_dn12, locals.var_q_bl_dep_dn17,)
    }
};
            locals.var_q_bl_dep = assign12900_body111_e17770;
            locals.var_q_bl_dep_dn0 = assign12900_body111_e17770_d_n0;
            locals.var_q_bl_dep_dn2 = assign12900_body111_e17770_d_n2;
            locals.var_q_bl_dep_dn6 = assign12900_body111_e17770_d_n6;
            locals.var_q_bl_dep_dn7 = assign12900_body111_e17770_d_n7;
            locals.var_q_bl_dep_dn10 = assign12900_body111_e17770_d_n10;
            locals.var_q_bl_dep_dn11 = assign12900_body111_e17770_d_n11;
            locals.var_q_bl_dep_dn12 = assign12900_body111_e17770_d_n12;
            locals.var_q_bl_dep_dn17 = assign12900_body111_e17770_d_n17;
            let (assign12900_body112_e17780, assign12900_body112_e17780_d_n0, assign12900_body112_e17780_d_n2, assign12900_body112_e17780_d_n6, assign12900_body112_e17780_d_n7, assign12900_body112_e17780_d_n10, assign12900_body112_e17780_d_n11, assign12900_body112_e17780_d_n12, assign12900_body112_e17780_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard398 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
            locals.var_t0 = assign12900_body112_e17780;
            locals.var_t0_dn0 = assign12900_body112_e17780_d_n0;
            locals.var_t0_dn2 = assign12900_body112_e17780_d_n2;
            locals.var_t0_dn6 = assign12900_body112_e17780_d_n6;
            locals.var_t0_dn7 = assign12900_body112_e17780_d_n7;
            locals.var_t0_dn10 = assign12900_body112_e17780_d_n10;
            locals.var_t0_dn11 = assign12900_body112_e17780_d_n11;
            locals.var_t0_dn12 = assign12900_body112_e17780_d_n12;
            locals.var_t0_dn17 = assign12900_body112_e17780_d_n17;
            let (assign12900_body113_e17789, assign12900_body113_e17789_d_n0, assign12900_body113_e17789_d_n2, assign12900_body113_e17789_d_n6, assign12900_body113_e17789_d_n7, assign12900_body113_e17789_d_n10, assign12900_body113_e17789_d_n11, assign12900_body113_e17789_d_n12, assign12900_body113_e17789_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign12900_body113_e17787: f64 = (locals.var_q_bl_dep_dpss * locals.var_t0);
        (assign12900_body113_e17787, ((locals.var_q_bl_dep_dpss_dn0 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn0)), ((locals.var_q_bl_dep_dpss_dn2 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn2)), ((locals.var_q_bl_dep_dpss_dn6 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn6)), ((locals.var_q_bl_dep_dpss_dn7 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn7)), ((locals.var_q_bl_dep_dpss_dn10 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn10)), ((locals.var_q_bl_dep_dpss_dn11 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn11)), ((locals.var_q_bl_dep_dpss_dn12 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn12)), ((locals.var_q_bl_dep_dpss_dn17 * locals.var_t0) + (locals.var_q_bl_dep_dpss * locals.var_t0_dn17)),)
    } else {
        (locals.var_q_bl_dep_dpss, locals.var_q_bl_dep_dpss_dn0, locals.var_q_bl_dep_dpss_dn2, locals.var_q_bl_dep_dpss_dn6, locals.var_q_bl_dep_dpss_dn7, locals.var_q_bl_dep_dpss_dn10, locals.var_q_bl_dep_dpss_dn11, locals.var_q_bl_dep_dpss_dn12, locals.var_q_bl_dep_dpss_dn17,)
    }
};
            locals.var_q_bl_dep_dpss = assign12900_body113_e17789;
            locals.var_q_bl_dep_dpss_dn0 = assign12900_body113_e17789_d_n0;
            locals.var_q_bl_dep_dpss_dn2 = assign12900_body113_e17789_d_n2;
            locals.var_q_bl_dep_dpss_dn6 = assign12900_body113_e17789_d_n6;
            locals.var_q_bl_dep_dpss_dn7 = assign12900_body113_e17789_d_n7;
            locals.var_q_bl_dep_dpss_dn10 = assign12900_body113_e17789_d_n10;
            locals.var_q_bl_dep_dpss_dn11 = assign12900_body113_e17789_d_n11;
            locals.var_q_bl_dep_dpss_dn12 = assign12900_body113_e17789_d_n12;
            locals.var_q_bl_dep_dpss_dn17 = assign12900_body113_e17789_d_n17;
            let (assign12900_body114_e17798, assign12900_body114_e17798_d_n0, assign12900_body114_e17798_d_n2, assign12900_body114_e17798_d_n6, assign12900_body114_e17798_d_n7, assign12900_body114_e17798_d_n10, assign12900_body114_e17798_d_n11, assign12900_body114_e17798_d_n12, assign12900_body114_e17798_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign12900_body114_e17796: f64 = (locals.var_q_bl_dep_dpbs * locals.var_t0);
        (assign12900_body114_e17796, ((locals.var_q_bl_dep_dpbs_dn0 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn0)), ((locals.var_q_bl_dep_dpbs_dn2 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn2)), ((locals.var_q_bl_dep_dpbs_dn6 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn6)), ((locals.var_q_bl_dep_dpbs_dn7 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn7)), ((locals.var_q_bl_dep_dpbs_dn10 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn10)), ((locals.var_q_bl_dep_dpbs_dn11 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn11)), ((locals.var_q_bl_dep_dpbs_dn12 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn12)), ((locals.var_q_bl_dep_dpbs_dn17 * locals.var_t0) + (locals.var_q_bl_dep_dpbs * locals.var_t0_dn17)),)
    } else {
        (locals.var_q_bl_dep_dpbs, locals.var_q_bl_dep_dpbs_dn0, locals.var_q_bl_dep_dpbs_dn2, locals.var_q_bl_dep_dpbs_dn6, locals.var_q_bl_dep_dpbs_dn7, locals.var_q_bl_dep_dpbs_dn10, locals.var_q_bl_dep_dpbs_dn11, locals.var_q_bl_dep_dpbs_dn12, locals.var_q_bl_dep_dpbs_dn17,)
    }
};
            locals.var_q_bl_dep_dpbs = assign12900_body114_e17798;
            locals.var_q_bl_dep_dpbs_dn0 = assign12900_body114_e17798_d_n0;
            locals.var_q_bl_dep_dpbs_dn2 = assign12900_body114_e17798_d_n2;
            locals.var_q_bl_dep_dpbs_dn6 = assign12900_body114_e17798_d_n6;
            locals.var_q_bl_dep_dpbs_dn7 = assign12900_body114_e17798_d_n7;
            locals.var_q_bl_dep_dpbs_dn10 = assign12900_body114_e17798_d_n10;
            locals.var_q_bl_dep_dpbs_dn11 = assign12900_body114_e17798_d_n11;
            locals.var_q_bl_dep_dpbs_dn12 = assign12900_body114_e17798_d_n12;
            locals.var_q_bl_dep_dpbs_dn17 = assign12900_body114_e17798_d_n17;
            let (assign12900_body115_e17807, assign12900_body115_e17807_d_n0, assign12900_body115_e17807_d_n2, assign12900_body115_e17807_d_n6, assign12900_body115_e17807_d_n7, assign12900_body115_e17807_d_n10, assign12900_body115_e17807_d_n11, assign12900_body115_e17807_d_n12, assign12900_body115_e17807_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign12900_body115_e17805: f64 = (locals.var_q_sl_dep + locals.var_q_bl_dep);
        (assign12900_body115_e17805, (locals.var_q_sl_dep_dn0 + locals.var_q_bl_dep_dn0), (locals.var_q_sl_dep_dn2 + locals.var_q_bl_dep_dn2), (locals.var_q_sl_dep_dn6 + locals.var_q_bl_dep_dn6), (locals.var_q_sl_dep_dn7 + locals.var_q_bl_dep_dn7), (locals.var_q_sl_dep_dn10 + locals.var_q_bl_dep_dn10), (locals.var_q_sl_dep_dn11 + locals.var_q_bl_dep_dn11), (locals.var_q_sl_dep_dn12 + locals.var_q_bl_dep_dn12), (locals.var_q_sl_dep_dn17 + locals.var_q_bl_dep_dn17),)
    } else {
        (locals.var_q_depl, locals.var_q_depl_dn0, locals.var_q_depl_dn2, locals.var_q_depl_dn6, locals.var_q_depl_dn7, locals.var_q_depl_dn10, locals.var_q_depl_dn11, locals.var_q_depl_dn12, locals.var_q_depl_dn17,)
    }
};
            locals.var_q_depl = assign12900_body115_e17807;
            locals.var_q_depl_dn0 = assign12900_body115_e17807_d_n0;
            locals.var_q_depl_dn2 = assign12900_body115_e17807_d_n2;
            locals.var_q_depl_dn6 = assign12900_body115_e17807_d_n6;
            locals.var_q_depl_dn7 = assign12900_body115_e17807_d_n7;
            locals.var_q_depl_dn10 = assign12900_body115_e17807_d_n10;
            locals.var_q_depl_dn11 = assign12900_body115_e17807_d_n11;
            locals.var_q_depl_dn12 = assign12900_body115_e17807_d_n12;
            locals.var_q_depl_dn17 = assign12900_body115_e17807_d_n17;
            let assign12900_body116_e17814: f64 = if ((locals.var_flg_conv == 1.0) && (locals.var_lp_sl > 3.0)) { 1.0 } else { 0.0 };
            locals.var_guard404 = assign12900_body116_e17814;
            let (assign12900_body117_e17823,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 != 0.0)) {
        (locals.var_lp_sl,)
    } else {
        (locals.var_flg_brk8,)
    }
};
            locals.var_flg_brk8 = assign12900_body117_e17823;
            let (assign12900_body118_e17832,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 != 0.0)) {
        (locals.var_lp_sl_max,)
    } else {
        (locals.var_lp_sl,)
    }
};
            locals.var_lp_sl = assign12900_body118_e17832;
            let (assign12900_body119_e17856, assign12900_body119_e17856_d_n0, assign12900_body119_e17856_d_n2, assign12900_body119_e17856_d_n6, assign12900_body119_e17856_d_n7, assign12900_body119_e17856_d_n10, assign12900_body119_e17856_d_n11, assign12900_body119_e17856_d_n12, assign12900_body119_e17856_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12900_body119_e17842: f64 = (locals.var_phi_sl_soi - locals.var_vgpz);
        let assign12900_body119_e17846: f64 = (locals.var_q_sl_bulk + locals.var_q_sl_dep);
        let assign12900_body119_e17848: f64 = (assign12900_body119_e17846 + locals.var_q_nl);
        let assign12900_body119_e17850: f64 = (assign12900_body119_e17848 + locals.var_q_bl_dep);
        let assign12900_body119_e17852: f64 = (assign12900_body119_e17850 + locals.var_qhs);
        let assign12900_body119_e17853: f64 = (locals.var_c_fox_inv * assign12900_body119_e17852);
        let assign12900_body119_e17854: f64 = (assign12900_body119_e17842 - assign12900_body119_e17853);
        (assign12900_body119_e17854, ((locals.var_phi_sl_soi_dn0 - locals.var_vgpz_dn0) - ((locals.var_c_fox_inv_dn0 * assign12900_body119_e17852) + (locals.var_c_fox_inv * ((((locals.var_q_sl_bulk_dn0 + locals.var_q_sl_dep_dn0) + locals.var_q_nl_dn0) + locals.var_q_bl_dep_dn0) + locals.var_qhs_dn0)))), ((locals.var_phi_sl_soi_dn2 - locals.var_vgpz_dn2) - ((locals.var_c_fox_inv_dn2 * assign12900_body119_e17852) + (locals.var_c_fox_inv * ((((locals.var_q_sl_bulk_dn2 + locals.var_q_sl_dep_dn2) + locals.var_q_nl_dn2) + locals.var_q_bl_dep_dn2) + locals.var_qhs_dn2)))), ((locals.var_phi_sl_soi_dn6 - locals.var_vgpz_dn6) - ((locals.var_c_fox_inv_dn6 * assign12900_body119_e17852) + (locals.var_c_fox_inv * ((((locals.var_q_sl_bulk_dn6 + locals.var_q_sl_dep_dn6) + locals.var_q_nl_dn6) + locals.var_q_bl_dep_dn6) + locals.var_qhs_dn6)))), ((locals.var_phi_sl_soi_dn7 - locals.var_vgpz_dn7) - ((locals.var_c_fox_inv_dn7 * assign12900_body119_e17852) + (locals.var_c_fox_inv * ((((locals.var_q_sl_bulk_dn7 + locals.var_q_sl_dep_dn7) + locals.var_q_nl_dn7) + locals.var_q_bl_dep_dn7) + locals.var_qhs_dn7)))), ((locals.var_phi_sl_soi_dn10 - locals.var_vgpz_dn10) - ((locals.var_c_fox_inv_dn10 * assign12900_body119_e17852) + (locals.var_c_fox_inv * ((((locals.var_q_sl_bulk_dn10 + locals.var_q_sl_dep_dn10) + locals.var_q_nl_dn10) + locals.var_q_bl_dep_dn10) + locals.var_qhs_dn10)))), ((locals.var_phi_sl_soi_dn11 - locals.var_vgpz_dn11) - ((locals.var_c_fox_inv_dn11 * assign12900_body119_e17852) + (locals.var_c_fox_inv * ((((locals.var_q_sl_bulk_dn11 + locals.var_q_sl_dep_dn11) + locals.var_q_nl_dn11) + locals.var_q_bl_dep_dn11) + locals.var_qhs_dn11)))), ((locals.var_phi_sl_soi_dn12 - locals.var_vgpz_dn12) - ((locals.var_c_fox_inv_dn12 * assign12900_body119_e17852) + (locals.var_c_fox_inv * ((((locals.var_q_sl_bulk_dn12 + locals.var_q_sl_dep_dn12) + locals.var_q_nl_dn12) + locals.var_q_bl_dep_dn12) + locals.var_qhs_dn12)))), ((locals.var_phi_sl_soi_dn17 - locals.var_vgpz_dn17) - ((locals.var_c_fox_inv_dn17 * assign12900_body119_e17852) + (locals.var_c_fox_inv * ((((locals.var_q_sl_bulk_dn17 + locals.var_q_sl_dep_dn17) + locals.var_q_nl_dn17) + locals.var_q_bl_dep_dn17) + locals.var_qhs_dn17)))),)
    } else {
        (locals.var_pf1__blk361, locals.var_pf1__blk361_dn0, locals.var_pf1__blk361_dn2, locals.var_pf1__blk361_dn6, locals.var_pf1__blk361_dn7, locals.var_pf1__blk361_dn10, locals.var_pf1__blk361_dn11, locals.var_pf1__blk361_dn12, locals.var_pf1__blk361_dn17,)
    }
};
            locals.var_pf1__blk361 = assign12900_body119_e17856;
            locals.var_pf1__blk361_dn0 = assign12900_body119_e17856_d_n0;
            locals.var_pf1__blk361_dn2 = assign12900_body119_e17856_d_n2;
            locals.var_pf1__blk361_dn6 = assign12900_body119_e17856_d_n6;
            locals.var_pf1__blk361_dn7 = assign12900_body119_e17856_d_n7;
            locals.var_pf1__blk361_dn10 = assign12900_body119_e17856_d_n10;
            locals.var_pf1__blk361_dn11 = assign12900_body119_e17856_d_n11;
            locals.var_pf1__blk361_dn12 = assign12900_body119_e17856_d_n12;
            locals.var_pf1__blk361_dn17 = assign12900_body119_e17856_d_n17;
            let (assign12900_body120_e17872, assign12900_body120_e17872_d_n0, assign12900_body120_e17872_d_n2, assign12900_body120_e17872_d_n6, assign12900_body120_e17872_d_n7, assign12900_body120_e17872_d_n10, assign12900_body120_e17872_d_n11, assign12900_body120_e17872_d_n12, assign12900_body120_e17872_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12900_body120_e17868: f64 = (locals.var_q_nl_dpss + locals.var_q_bl_dep_dpss);
        let assign12900_body120_e17869: f64 = (locals.var_c_fox_inv * assign12900_body120_e17868);
        let assign12900_body120_e17870: f64 = (1.0 - assign12900_body120_e17869);
        (assign12900_body120_e17870, (-((locals.var_c_fox_inv_dn0 * assign12900_body120_e17868) + (locals.var_c_fox_inv * (locals.var_q_nl_dpss_dn0 + locals.var_q_bl_dep_dpss_dn0)))), (-((locals.var_c_fox_inv_dn2 * assign12900_body120_e17868) + (locals.var_c_fox_inv * (locals.var_q_nl_dpss_dn2 + locals.var_q_bl_dep_dpss_dn2)))), (-((locals.var_c_fox_inv_dn6 * assign12900_body120_e17868) + (locals.var_c_fox_inv * (locals.var_q_nl_dpss_dn6 + locals.var_q_bl_dep_dpss_dn6)))), (-((locals.var_c_fox_inv_dn7 * assign12900_body120_e17868) + (locals.var_c_fox_inv * (locals.var_q_nl_dpss_dn7 + locals.var_q_bl_dep_dpss_dn7)))), (-((locals.var_c_fox_inv_dn10 * assign12900_body120_e17868) + (locals.var_c_fox_inv * (locals.var_q_nl_dpss_dn10 + locals.var_q_bl_dep_dpss_dn10)))), (-((locals.var_c_fox_inv_dn11 * assign12900_body120_e17868) + (locals.var_c_fox_inv * (locals.var_q_nl_dpss_dn11 + locals.var_q_bl_dep_dpss_dn11)))), (-((locals.var_c_fox_inv_dn12 * assign12900_body120_e17868) + (locals.var_c_fox_inv * (locals.var_q_nl_dpss_dn12 + locals.var_q_bl_dep_dpss_dn12)))), (-((locals.var_c_fox_inv_dn17 * assign12900_body120_e17868) + (locals.var_c_fox_inv * (locals.var_q_nl_dpss_dn17 + locals.var_q_bl_dep_dpss_dn17)))),)
    } else {
        (locals.var_pf11__blk362, locals.var_pf11__blk362_dn0, locals.var_pf11__blk362_dn2, locals.var_pf11__blk362_dn6, locals.var_pf11__blk362_dn7, locals.var_pf11__blk362_dn10, locals.var_pf11__blk362_dn11, locals.var_pf11__blk362_dn12, locals.var_pf11__blk362_dn17,)
    }
};
            locals.var_pf11__blk362 = assign12900_body120_e17872;
            locals.var_pf11__blk362_dn0 = assign12900_body120_e17872_d_n0;
            locals.var_pf11__blk362_dn2 = assign12900_body120_e17872_d_n2;
            locals.var_pf11__blk362_dn6 = assign12900_body120_e17872_d_n6;
            locals.var_pf11__blk362_dn7 = assign12900_body120_e17872_d_n7;
            locals.var_pf11__blk362_dn10 = assign12900_body120_e17872_d_n10;
            locals.var_pf11__blk362_dn11 = assign12900_body120_e17872_d_n11;
            locals.var_pf11__blk362_dn12 = assign12900_body120_e17872_d_n12;
            locals.var_pf11__blk362_dn17 = assign12900_body120_e17872_d_n17;
            let (assign12900_body121_e17885, assign12900_body121_e17885_d_n0, assign12900_body121_e17885_d_n2, assign12900_body121_e17885_d_n6, assign12900_body121_e17885_d_n7, assign12900_body121_e17885_d_n10, assign12900_body121_e17885_d_n11, assign12900_body121_e17885_d_n12, assign12900_body121_e17885_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12900_body121_e17881: f64 = (-locals.var_c_fox_inv);
        let assign12900_body121_e17883: f64 = (assign12900_body121_e17881 * locals.var_q_bl_dep_dpbs);
        (assign12900_body121_e17883, (((-locals.var_c_fox_inv_dn0) * locals.var_q_bl_dep_dpbs) + (assign12900_body121_e17881 * locals.var_q_bl_dep_dpbs_dn0)), (((-locals.var_c_fox_inv_dn2) * locals.var_q_bl_dep_dpbs) + (assign12900_body121_e17881 * locals.var_q_bl_dep_dpbs_dn2)), (((-locals.var_c_fox_inv_dn6) * locals.var_q_bl_dep_dpbs) + (assign12900_body121_e17881 * locals.var_q_bl_dep_dpbs_dn6)), (((-locals.var_c_fox_inv_dn7) * locals.var_q_bl_dep_dpbs) + (assign12900_body121_e17881 * locals.var_q_bl_dep_dpbs_dn7)), (((-locals.var_c_fox_inv_dn10) * locals.var_q_bl_dep_dpbs) + (assign12900_body121_e17881 * locals.var_q_bl_dep_dpbs_dn10)), (((-locals.var_c_fox_inv_dn11) * locals.var_q_bl_dep_dpbs) + (assign12900_body121_e17881 * locals.var_q_bl_dep_dpbs_dn11)), (((-locals.var_c_fox_inv_dn12) * locals.var_q_bl_dep_dpbs) + (assign12900_body121_e17881 * locals.var_q_bl_dep_dpbs_dn12)), (((-locals.var_c_fox_inv_dn17) * locals.var_q_bl_dep_dpbs) + (assign12900_body121_e17881 * locals.var_q_bl_dep_dpbs_dn17)),)
    } else {
        (locals.var_pf12__blk363, locals.var_pf12__blk363_dn0, locals.var_pf12__blk363_dn2, locals.var_pf12__blk363_dn6, locals.var_pf12__blk363_dn7, locals.var_pf12__blk363_dn10, locals.var_pf12__blk363_dn11, locals.var_pf12__blk363_dn12, locals.var_pf12__blk363_dn17,)
    }
};
            locals.var_pf12__blk363 = assign12900_body121_e17885;
            locals.var_pf12__blk363_dn0 = assign12900_body121_e17885_d_n0;
            locals.var_pf12__blk363_dn2 = assign12900_body121_e17885_d_n2;
            locals.var_pf12__blk363_dn6 = assign12900_body121_e17885_d_n6;
            locals.var_pf12__blk363_dn7 = assign12900_body121_e17885_d_n7;
            locals.var_pf12__blk363_dn10 = assign12900_body121_e17885_d_n10;
            locals.var_pf12__blk363_dn11 = assign12900_body121_e17885_d_n11;
            locals.var_pf12__blk363_dn12 = assign12900_body121_e17885_d_n12;
            locals.var_pf12__blk363_dn17 = assign12900_body121_e17885_d_n17;
            let (assign12900_body122_e17898, assign12900_body122_e17898_d_n0, assign12900_body122_e17898_d_n2, assign12900_body122_e17898_d_n6, assign12900_body122_e17898_d_n7, assign12900_body122_e17898_d_n10, assign12900_body122_e17898_d_n11, assign12900_body122_e17898_d_n12, assign12900_body122_e17898_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12900_body122_e17894: f64 = (-locals.var_c_fox_inv);
        let assign12900_body122_e17896: f64 = (assign12900_body122_e17894 * locals.var_q_sl_bulk_dpsb);
        (assign12900_body122_e17896, (((-locals.var_c_fox_inv_dn0) * locals.var_q_sl_bulk_dpsb) + (assign12900_body122_e17894 * locals.var_q_sl_bulk_dpsb_dn0)), (((-locals.var_c_fox_inv_dn2) * locals.var_q_sl_bulk_dpsb) + (assign12900_body122_e17894 * locals.var_q_sl_bulk_dpsb_dn2)), (((-locals.var_c_fox_inv_dn6) * locals.var_q_sl_bulk_dpsb) + (assign12900_body122_e17894 * locals.var_q_sl_bulk_dpsb_dn6)), (((-locals.var_c_fox_inv_dn7) * locals.var_q_sl_bulk_dpsb) + (assign12900_body122_e17894 * locals.var_q_sl_bulk_dpsb_dn7)), (((-locals.var_c_fox_inv_dn10) * locals.var_q_sl_bulk_dpsb) + (assign12900_body122_e17894 * locals.var_q_sl_bulk_dpsb_dn10)), (((-locals.var_c_fox_inv_dn11) * locals.var_q_sl_bulk_dpsb) + (assign12900_body122_e17894 * locals.var_q_sl_bulk_dpsb_dn11)), (((-locals.var_c_fox_inv_dn12) * locals.var_q_sl_bulk_dpsb) + (assign12900_body122_e17894 * locals.var_q_sl_bulk_dpsb_dn12)), (((-locals.var_c_fox_inv_dn17) * locals.var_q_sl_bulk_dpsb) + (assign12900_body122_e17894 * locals.var_q_sl_bulk_dpsb_dn17)),)
    } else {
        (locals.var_pf13__blk364, locals.var_pf13__blk364_dn0, locals.var_pf13__blk364_dn2, locals.var_pf13__blk364_dn6, locals.var_pf13__blk364_dn7, locals.var_pf13__blk364_dn10, locals.var_pf13__blk364_dn11, locals.var_pf13__blk364_dn12, locals.var_pf13__blk364_dn17,)
    }
};
            locals.var_pf13__blk364 = assign12900_body122_e17898;
            locals.var_pf13__blk364_dn0 = assign12900_body122_e17898_d_n0;
            locals.var_pf13__blk364_dn2 = assign12900_body122_e17898_d_n2;
            locals.var_pf13__blk364_dn6 = assign12900_body122_e17898_d_n6;
            locals.var_pf13__blk364_dn7 = assign12900_body122_e17898_d_n7;
            locals.var_pf13__blk364_dn10 = assign12900_body122_e17898_d_n10;
            locals.var_pf13__blk364_dn11 = assign12900_body122_e17898_d_n11;
            locals.var_pf13__blk364_dn12 = assign12900_body122_e17898_d_n12;
            locals.var_pf13__blk364_dn17 = assign12900_body122_e17898_d_n17;
            let (assign12900_body123_e17916, assign12900_body123_e17916_d_n0, assign12900_body123_e17916_d_n2, assign12900_body123_e17916_d_n6, assign12900_body123_e17916_d_n7, assign12900_body123_e17916_d_n10, assign12900_body123_e17916_d_n11, assign12900_body123_e17916_d_n12, assign12900_body123_e17916_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12900_body123_e17910: f64 = (0.5 * locals.var_q_fd_soi);
        let assign12900_body123_e17912: f64 = (assign12900_body123_e17910 + locals.var_q_sl_bulk);
        let assign12900_body123_e17913: f64 = (locals.var_c_soi_inv__blk113 * assign12900_body123_e17912);
        let assign12900_body123_e17914: f64 = (locals.var_phi_sl_soi + assign12900_body123_e17913);
        (assign12900_body123_e17914, (locals.var_phi_sl_soi_dn0 + (locals.var_c_soi_inv__blk113 * ((0.5 * locals.var_q_fd_soi_dn0) + locals.var_q_sl_bulk_dn0))), (locals.var_phi_sl_soi_dn2 + (locals.var_c_soi_inv__blk113 * ((0.5 * locals.var_q_fd_soi_dn2) + locals.var_q_sl_bulk_dn2))), (locals.var_phi_sl_soi_dn6 + (locals.var_c_soi_inv__blk113 * ((0.5 * locals.var_q_fd_soi_dn6) + locals.var_q_sl_bulk_dn6))), (locals.var_phi_sl_soi_dn7 + (locals.var_c_soi_inv__blk113 * ((0.5 * locals.var_q_fd_soi_dn7) + locals.var_q_sl_bulk_dn7))), (locals.var_phi_sl_soi_dn10 + (locals.var_c_soi_inv__blk113 * ((0.5 * locals.var_q_fd_soi_dn10) + locals.var_q_sl_bulk_dn10))), (locals.var_phi_sl_soi_dn11 + (locals.var_c_soi_inv__blk113 * ((0.5 * locals.var_q_fd_soi_dn11) + locals.var_q_sl_bulk_dn11))), (locals.var_phi_sl_soi_dn12 + (locals.var_c_soi_inv__blk113 * ((0.5 * locals.var_q_fd_soi_dn12) + locals.var_q_sl_bulk_dn12))), (locals.var_phi_sl_soi_dn17 + (locals.var_c_soi_inv__blk113 * ((0.5 * locals.var_q_fd_soi_dn17) + locals.var_q_sl_bulk_dn17))),)
    } else {
        (locals.var_t1__blk351, locals.var_t1__blk351_dn0, locals.var_t1__blk351_dn2, locals.var_t1__blk351_dn6, locals.var_t1__blk351_dn7, locals.var_t1__blk351_dn10, locals.var_t1__blk351_dn11, locals.var_t1__blk351_dn12, locals.var_t1__blk351_dn17,)
    }
};
            locals.var_t1__blk351 = assign12900_body123_e17916;
            locals.var_t1__blk351_dn0 = assign12900_body123_e17916_d_n0;
            locals.var_t1__blk351_dn2 = assign12900_body123_e17916_d_n2;
            locals.var_t1__blk351_dn6 = assign12900_body123_e17916_d_n6;
            locals.var_t1__blk351_dn7 = assign12900_body123_e17916_d_n7;
            locals.var_t1__blk351_dn10 = assign12900_body123_e17916_d_n10;
            locals.var_t1__blk351_dn11 = assign12900_body123_e17916_d_n11;
            locals.var_t1__blk351_dn12 = assign12900_body123_e17916_d_n12;
            locals.var_t1__blk351_dn17 = assign12900_body123_e17916_d_n17;
            let (assign12900_body124_e17928, assign12900_body124_e17928_d_n0, assign12900_body124_e17928_d_n2, assign12900_body124_e17928_d_n6, assign12900_body124_e17928_d_n7, assign12900_body124_e17928_d_n10, assign12900_body124_e17928_d_n11, assign12900_body124_e17928_d_n12, assign12900_body124_e17928_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12900_body124_e17926: f64 = (locals.var_c_soi_inv__blk113 * locals.var_q_sl_bulk_dpsb);
        (assign12900_body124_e17926, (locals.var_c_soi_inv__blk113 * locals.var_q_sl_bulk_dpsb_dn0), (locals.var_c_soi_inv__blk113 * locals.var_q_sl_bulk_dpsb_dn2), (locals.var_c_soi_inv__blk113 * locals.var_q_sl_bulk_dpsb_dn6), (locals.var_c_soi_inv__blk113 * locals.var_q_sl_bulk_dpsb_dn7), (locals.var_c_soi_inv__blk113 * locals.var_q_sl_bulk_dpsb_dn10), (locals.var_c_soi_inv__blk113 * locals.var_q_sl_bulk_dpsb_dn11), (locals.var_c_soi_inv__blk113 * locals.var_q_sl_bulk_dpsb_dn12), (locals.var_c_soi_inv__blk113 * locals.var_q_sl_bulk_dpsb_dn17),)
    } else {
        (locals.var_t3__blk353, locals.var_t3__blk353_dn0, locals.var_t3__blk353_dn2, locals.var_t3__blk353_dn6, locals.var_t3__blk353_dn7, locals.var_t3__blk353_dn10, locals.var_t3__blk353_dn11, locals.var_t3__blk353_dn12, locals.var_t3__blk353_dn17,)
    }
};
            locals.var_t3__blk353 = assign12900_body124_e17928;
            locals.var_t3__blk353_dn0 = assign12900_body124_e17928_d_n0;
            locals.var_t3__blk353_dn2 = assign12900_body124_e17928_d_n2;
            locals.var_t3__blk353_dn6 = assign12900_body124_e17928_d_n6;
            locals.var_t3__blk353_dn7 = assign12900_body124_e17928_d_n7;
            locals.var_t3__blk353_dn10 = assign12900_body124_e17928_d_n10;
            locals.var_t3__blk353_dn11 = assign12900_body124_e17928_d_n11;
            locals.var_t3__blk353_dn12 = assign12900_body124_e17928_d_n12;
            locals.var_t3__blk353_dn17 = assign12900_body124_e17928_d_n17;
            let (assign12900_body125_e17940, assign12900_body125_e17940_d_n0, assign12900_body125_e17940_d_n2, assign12900_body125_e17940_d_n6, assign12900_body125_e17940_d_n7, assign12900_body125_e17940_d_n10, assign12900_body125_e17940_d_n11, assign12900_body125_e17940_d_n12, assign12900_body125_e17940_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12900_body125_e17938: f64 = (locals.var_phi_bl_soi - locals.var_t1__blk351);
        (assign12900_body125_e17938, (locals.var_phi_bl_soi_dn0 - locals.var_t1__blk351_dn0), (locals.var_phi_bl_soi_dn2 - locals.var_t1__blk351_dn2), (locals.var_phi_bl_soi_dn6 - locals.var_t1__blk351_dn6), (locals.var_phi_bl_soi_dn7 - locals.var_t1__blk351_dn7), (locals.var_phi_bl_soi_dn10 - locals.var_t1__blk351_dn10), (locals.var_phi_bl_soi_dn11 - locals.var_t1__blk351_dn11), (locals.var_phi_bl_soi_dn12 - locals.var_t1__blk351_dn12), (locals.var_phi_bl_soi_dn17 - locals.var_t1__blk351_dn17),)
    } else {
        (locals.var_pf2__blk365, locals.var_pf2__blk365_dn0, locals.var_pf2__blk365_dn2, locals.var_pf2__blk365_dn6, locals.var_pf2__blk365_dn7, locals.var_pf2__blk365_dn10, locals.var_pf2__blk365_dn11, locals.var_pf2__blk365_dn12, locals.var_pf2__blk365_dn17,)
    }
};
            locals.var_pf2__blk365 = assign12900_body125_e17940;
            locals.var_pf2__blk365_dn0 = assign12900_body125_e17940_d_n0;
            locals.var_pf2__blk365_dn2 = assign12900_body125_e17940_d_n2;
            locals.var_pf2__blk365_dn6 = assign12900_body125_e17940_d_n6;
            locals.var_pf2__blk365_dn7 = assign12900_body125_e17940_d_n7;
            locals.var_pf2__blk365_dn10 = assign12900_body125_e17940_d_n10;
            locals.var_pf2__blk365_dn11 = assign12900_body125_e17940_d_n11;
            locals.var_pf2__blk365_dn12 = assign12900_body125_e17940_d_n12;
            locals.var_pf2__blk365_dn17 = assign12900_body125_e17940_d_n17;
            let (assign12900_body126_e17951,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12900_body126_e17949: f64 = (-1.0);
        (assign12900_body126_e17949,)
    } else {
        (locals.var_pf21__blk366,)
    }
};
            locals.var_pf21__blk366 = assign12900_body126_e17951;
            let (assign12900_body127_e17961,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_pf22__blk367,)
    }
};
            locals.var_pf22__blk367 = assign12900_body127_e17961;
            let (assign12900_body128_e17972, assign12900_body128_e17972_d_n0, assign12900_body128_e17972_d_n2, assign12900_body128_e17972_d_n6, assign12900_body128_e17972_d_n7, assign12900_body128_e17972_d_n10, assign12900_body128_e17972_d_n11, assign12900_body128_e17972_d_n12, assign12900_body128_e17972_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12900_body128_e17970: f64 = (-locals.var_t3__blk353);
        (assign12900_body128_e17970, (-locals.var_t3__blk353_dn0), (-locals.var_t3__blk353_dn2), (-locals.var_t3__blk353_dn6), (-locals.var_t3__blk353_dn7), (-locals.var_t3__blk353_dn10), (-locals.var_t3__blk353_dn11), (-locals.var_t3__blk353_dn12), (-locals.var_t3__blk353_dn17),)
    } else {
        (locals.var_pf23__blk368, locals.var_pf23__blk368_dn0, locals.var_pf23__blk368_dn2, locals.var_pf23__blk368_dn6, locals.var_pf23__blk368_dn7, locals.var_pf23__blk368_dn10, locals.var_pf23__blk368_dn11, locals.var_pf23__blk368_dn12, locals.var_pf23__blk368_dn17,)
    }
};
            locals.var_pf23__blk368 = assign12900_body128_e17972;
            locals.var_pf23__blk368_dn0 = assign12900_body128_e17972_d_n0;
            locals.var_pf23__blk368_dn2 = assign12900_body128_e17972_d_n2;
            locals.var_pf23__blk368_dn6 = assign12900_body128_e17972_d_n6;
            locals.var_pf23__blk368_dn7 = assign12900_body128_e17972_d_n7;
            locals.var_pf23__blk368_dn10 = assign12900_body128_e17972_d_n10;
            locals.var_pf23__blk368_dn11 = assign12900_body128_e17972_d_n11;
            locals.var_pf23__blk368_dn12 = assign12900_body128_e17972_d_n12;
            locals.var_pf23__blk368_dn17 = assign12900_body128_e17972_d_n17;
            let (assign12900_body129_e17988, assign12900_body129_e17988_d_n0, assign12900_body129_e17988_d_n2, assign12900_body129_e17988_d_n6, assign12900_body129_e17988_d_n7, assign12900_body129_e17988_d_n10, assign12900_body129_e17988_d_n11, assign12900_body129_e17988_d_n12, assign12900_body129_e17988_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12900_body129_e17982: f64 = (locals.var_phi_sl_bulk - locals.var_phi_bl_soi);
        let assign12900_body129_e17985: f64 = (locals.var_c_box_inv * locals.var_q_sl_bulk);
        let assign12900_body129_e17986: f64 = (assign12900_body129_e17982 - assign12900_body129_e17985);
        (assign12900_body129_e17986, ((locals.var_phi_sl_bulk_dn0 - locals.var_phi_bl_soi_dn0) - (locals.var_c_box_inv * locals.var_q_sl_bulk_dn0)), ((locals.var_phi_sl_bulk_dn2 - locals.var_phi_bl_soi_dn2) - (locals.var_c_box_inv * locals.var_q_sl_bulk_dn2)), ((locals.var_phi_sl_bulk_dn6 - locals.var_phi_bl_soi_dn6) - (locals.var_c_box_inv * locals.var_q_sl_bulk_dn6)), ((locals.var_phi_sl_bulk_dn7 - locals.var_phi_bl_soi_dn7) - (locals.var_c_box_inv * locals.var_q_sl_bulk_dn7)), ((locals.var_phi_sl_bulk_dn10 - locals.var_phi_bl_soi_dn10) - (locals.var_c_box_inv * locals.var_q_sl_bulk_dn10)), ((locals.var_phi_sl_bulk_dn11 - locals.var_phi_bl_soi_dn11) - (locals.var_c_box_inv * locals.var_q_sl_bulk_dn11)), ((locals.var_phi_sl_bulk_dn12 - locals.var_phi_bl_soi_dn12) - (locals.var_c_box_inv * locals.var_q_sl_bulk_dn12)), ((locals.var_phi_sl_bulk_dn17 - locals.var_phi_bl_soi_dn17) - (locals.var_c_box_inv * locals.var_q_sl_bulk_dn17)),)
    } else {
        (locals.var_pf3__blk369, locals.var_pf3__blk369_dn0, locals.var_pf3__blk369_dn2, locals.var_pf3__blk369_dn6, locals.var_pf3__blk369_dn7, locals.var_pf3__blk369_dn10, locals.var_pf3__blk369_dn11, locals.var_pf3__blk369_dn12, locals.var_pf3__blk369_dn17,)
    }
};
            locals.var_pf3__blk369 = assign12900_body129_e17988;
            locals.var_pf3__blk369_dn0 = assign12900_body129_e17988_d_n0;
            locals.var_pf3__blk369_dn2 = assign12900_body129_e17988_d_n2;
            locals.var_pf3__blk369_dn6 = assign12900_body129_e17988_d_n6;
            locals.var_pf3__blk369_dn7 = assign12900_body129_e17988_d_n7;
            locals.var_pf3__blk369_dn10 = assign12900_body129_e17988_d_n10;
            locals.var_pf3__blk369_dn11 = assign12900_body129_e17988_d_n11;
            locals.var_pf3__blk369_dn12 = assign12900_body129_e17988_d_n12;
            locals.var_pf3__blk369_dn17 = assign12900_body129_e17988_d_n17;
            let (assign12900_body130_e17999,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12900_body130_e17997: f64 = (-1.0);
        (assign12900_body130_e17997,)
    } else {
        (locals.var_pf32__blk370,)
    }
};
            locals.var_pf32__blk370 = assign12900_body130_e17999;
            let (assign12900_body131_e18013, assign12900_body131_e18013_d_n0, assign12900_body131_e18013_d_n2, assign12900_body131_e18013_d_n6, assign12900_body131_e18013_d_n7, assign12900_body131_e18013_d_n10, assign12900_body131_e18013_d_n11, assign12900_body131_e18013_d_n12, assign12900_body131_e18013_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12900_body131_e18010: f64 = (locals.var_c_box_inv * locals.var_q_sl_bulk_dpsb);
        let assign12900_body131_e18011: f64 = (1.0 - assign12900_body131_e18010);
        (assign12900_body131_e18011, (-(locals.var_c_box_inv * locals.var_q_sl_bulk_dpsb_dn0)), (-(locals.var_c_box_inv * locals.var_q_sl_bulk_dpsb_dn2)), (-(locals.var_c_box_inv * locals.var_q_sl_bulk_dpsb_dn6)), (-(locals.var_c_box_inv * locals.var_q_sl_bulk_dpsb_dn7)), (-(locals.var_c_box_inv * locals.var_q_sl_bulk_dpsb_dn10)), (-(locals.var_c_box_inv * locals.var_q_sl_bulk_dpsb_dn11)), (-(locals.var_c_box_inv * locals.var_q_sl_bulk_dpsb_dn12)), (-(locals.var_c_box_inv * locals.var_q_sl_bulk_dpsb_dn17)),)
    } else {
        (locals.var_pf33__blk371, locals.var_pf33__blk371_dn0, locals.var_pf33__blk371_dn2, locals.var_pf33__blk371_dn6, locals.var_pf33__blk371_dn7, locals.var_pf33__blk371_dn10, locals.var_pf33__blk371_dn11, locals.var_pf33__blk371_dn12, locals.var_pf33__blk371_dn17,)
    }
};
            locals.var_pf33__blk371 = assign12900_body131_e18013;
            locals.var_pf33__blk371_dn0 = assign12900_body131_e18013_d_n0;
            locals.var_pf33__blk371_dn2 = assign12900_body131_e18013_d_n2;
            locals.var_pf33__blk371_dn6 = assign12900_body131_e18013_d_n6;
            locals.var_pf33__blk371_dn7 = assign12900_body131_e18013_d_n7;
            locals.var_pf33__blk371_dn10 = assign12900_body131_e18013_d_n10;
            locals.var_pf33__blk371_dn11 = assign12900_body131_e18013_d_n11;
            locals.var_pf33__blk371_dn12 = assign12900_body131_e18013_d_n12;
            locals.var_pf33__blk371_dn17 = assign12900_body131_e18013_d_n17;
            let (assign12900_body132_e18045, assign12900_body132_e18045_d_n0, assign12900_body132_e18045_d_n2, assign12900_body132_e18045_d_n6, assign12900_body132_e18045_d_n7, assign12900_body132_e18045_d_n10, assign12900_body132_e18045_d_n11, assign12900_body132_e18045_d_n12, assign12900_body132_e18045_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12900_body132_e18023: f64 = (locals.var_pf11__blk362 * locals.var_pf22__blk367);
        let assign12900_body132_e18025: f64 = (assign12900_body132_e18023 * locals.var_pf33__blk371);
        let assign12900_body132_e18028: f64 = (locals.var_pf11__blk362 * locals.var_pf23__blk368);
        let assign12900_body132_e18030: f64 = (assign12900_body132_e18028 * locals.var_pf32__blk370);
        let assign12900_body132_e18031: f64 = (assign12900_body132_e18025 - assign12900_body132_e18030);
        let assign12900_body132_e18034: f64 = (locals.var_pf12__blk363 * locals.var_pf21__blk366);
        let assign12900_body132_e18036: f64 = (assign12900_body132_e18034 * locals.var_pf33__blk371);
        let assign12900_body132_e18037: f64 = (assign12900_body132_e18031 - assign12900_body132_e18036);
        let assign12900_body132_e18040: f64 = (locals.var_pf13__blk364 * locals.var_pf21__blk366);
        let assign12900_body132_e18042: f64 = (assign12900_body132_e18040 * locals.var_pf32__blk370);
        let assign12900_body132_e18043: f64 = (assign12900_body132_e18037 + assign12900_body132_e18042);
        (assign12900_body132_e18043, ((((((locals.var_pf11__blk362_dn0 * locals.var_pf22__blk367) * locals.var_pf33__blk371) + (assign12900_body132_e18023 * locals.var_pf33__blk371_dn0)) - (((locals.var_pf11__blk362_dn0 * locals.var_pf23__blk368) + (locals.var_pf11__blk362 * locals.var_pf23__blk368_dn0)) * locals.var_pf32__blk370)) - (((locals.var_pf12__blk363_dn0 * locals.var_pf21__blk366) * locals.var_pf33__blk371) + (assign12900_body132_e18034 * locals.var_pf33__blk371_dn0))) + ((locals.var_pf13__blk364_dn0 * locals.var_pf21__blk366) * locals.var_pf32__blk370)), ((((((locals.var_pf11__blk362_dn2 * locals.var_pf22__blk367) * locals.var_pf33__blk371) + (assign12900_body132_e18023 * locals.var_pf33__blk371_dn2)) - (((locals.var_pf11__blk362_dn2 * locals.var_pf23__blk368) + (locals.var_pf11__blk362 * locals.var_pf23__blk368_dn2)) * locals.var_pf32__blk370)) - (((locals.var_pf12__blk363_dn2 * locals.var_pf21__blk366) * locals.var_pf33__blk371) + (assign12900_body132_e18034 * locals.var_pf33__blk371_dn2))) + ((locals.var_pf13__blk364_dn2 * locals.var_pf21__blk366) * locals.var_pf32__blk370)), ((((((locals.var_pf11__blk362_dn6 * locals.var_pf22__blk367) * locals.var_pf33__blk371) + (assign12900_body132_e18023 * locals.var_pf33__blk371_dn6)) - (((locals.var_pf11__blk362_dn6 * locals.var_pf23__blk368) + (locals.var_pf11__blk362 * locals.var_pf23__blk368_dn6)) * locals.var_pf32__blk370)) - (((locals.var_pf12__blk363_dn6 * locals.var_pf21__blk366) * locals.var_pf33__blk371) + (assign12900_body132_e18034 * locals.var_pf33__blk371_dn6))) + ((locals.var_pf13__blk364_dn6 * locals.var_pf21__blk366) * locals.var_pf32__blk370)), ((((((locals.var_pf11__blk362_dn7 * locals.var_pf22__blk367) * locals.var_pf33__blk371) + (assign12900_body132_e18023 * locals.var_pf33__blk371_dn7)) - (((locals.var_pf11__blk362_dn7 * locals.var_pf23__blk368) + (locals.var_pf11__blk362 * locals.var_pf23__blk368_dn7)) * locals.var_pf32__blk370)) - (((locals.var_pf12__blk363_dn7 * locals.var_pf21__blk366) * locals.var_pf33__blk371) + (assign12900_body132_e18034 * locals.var_pf33__blk371_dn7))) + ((locals.var_pf13__blk364_dn7 * locals.var_pf21__blk366) * locals.var_pf32__blk370)), ((((((locals.var_pf11__blk362_dn10 * locals.var_pf22__blk367) * locals.var_pf33__blk371) + (assign12900_body132_e18023 * locals.var_pf33__blk371_dn10)) - (((locals.var_pf11__blk362_dn10 * locals.var_pf23__blk368) + (locals.var_pf11__blk362 * locals.var_pf23__blk368_dn10)) * locals.var_pf32__blk370)) - (((locals.var_pf12__blk363_dn10 * locals.var_pf21__blk366) * locals.var_pf33__blk371) + (assign12900_body132_e18034 * locals.var_pf33__blk371_dn10))) + ((locals.var_pf13__blk364_dn10 * locals.var_pf21__blk366) * locals.var_pf32__blk370)), ((((((locals.var_pf11__blk362_dn11 * locals.var_pf22__blk367) * locals.var_pf33__blk371) + (assign12900_body132_e18023 * locals.var_pf33__blk371_dn11)) - (((locals.var_pf11__blk362_dn11 * locals.var_pf23__blk368) + (locals.var_pf11__blk362 * locals.var_pf23__blk368_dn11)) * locals.var_pf32__blk370)) - (((locals.var_pf12__blk363_dn11 * locals.var_pf21__blk366) * locals.var_pf33__blk371) + (assign12900_body132_e18034 * locals.var_pf33__blk371_dn11))) + ((locals.var_pf13__blk364_dn11 * locals.var_pf21__blk366) * locals.var_pf32__blk370)), ((((((locals.var_pf11__blk362_dn12 * locals.var_pf22__blk367) * locals.var_pf33__blk371) + (assign12900_body132_e18023 * locals.var_pf33__blk371_dn12)) - (((locals.var_pf11__blk362_dn12 * locals.var_pf23__blk368) + (locals.var_pf11__blk362 * locals.var_pf23__blk368_dn12)) * locals.var_pf32__blk370)) - (((locals.var_pf12__blk363_dn12 * locals.var_pf21__blk366) * locals.var_pf33__blk371) + (assign12900_body132_e18034 * locals.var_pf33__blk371_dn12))) + ((locals.var_pf13__blk364_dn12 * locals.var_pf21__blk366) * locals.var_pf32__blk370)), ((((((locals.var_pf11__blk362_dn17 * locals.var_pf22__blk367) * locals.var_pf33__blk371) + (assign12900_body132_e18023 * locals.var_pf33__blk371_dn17)) - (((locals.var_pf11__blk362_dn17 * locals.var_pf23__blk368) + (locals.var_pf11__blk362 * locals.var_pf23__blk368_dn17)) * locals.var_pf32__blk370)) - (((locals.var_pf12__blk363_dn17 * locals.var_pf21__blk366) * locals.var_pf33__blk371) + (assign12900_body132_e18034 * locals.var_pf33__blk371_dn17))) + ((locals.var_pf13__blk364_dn17 * locals.var_pf21__blk366) * locals.var_pf32__blk370)),)
    } else {
        (locals.var_pdj__blk372, locals.var_pdj__blk372_dn0, locals.var_pdj__blk372_dn2, locals.var_pdj__blk372_dn6, locals.var_pdj__blk372_dn7, locals.var_pdj__blk372_dn10, locals.var_pdj__blk372_dn11, locals.var_pdj__blk372_dn12, locals.var_pdj__blk372_dn17,)
    }
};
            locals.var_pdj__blk372 = assign12900_body132_e18045;
            locals.var_pdj__blk372_dn0 = assign12900_body132_e18045_d_n0;
            locals.var_pdj__blk372_dn2 = assign12900_body132_e18045_d_n2;
            locals.var_pdj__blk372_dn6 = assign12900_body132_e18045_d_n6;
            locals.var_pdj__blk372_dn7 = assign12900_body132_e18045_d_n7;
            locals.var_pdj__blk372_dn10 = assign12900_body132_e18045_d_n10;
            locals.var_pdj__blk372_dn11 = assign12900_body132_e18045_d_n11;
            locals.var_pdj__blk372_dn12 = assign12900_body132_e18045_d_n12;
            locals.var_pdj__blk372_dn17 = assign12900_body132_e18045_d_n17;
            let (assign12900_body133_e18059, assign12900_body133_e18059_d_n0, assign12900_body133_e18059_d_n2, assign12900_body133_e18059_d_n6, assign12900_body133_e18059_d_n7, assign12900_body133_e18059_d_n10, assign12900_body133_e18059_d_n11, assign12900_body133_e18059_d_n12, assign12900_body133_e18059_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12900_body133_e18056: f64 = (locals.var_pdj__blk372 + 1e-50);
        let assign12900_body133_e18057: f64 = (1.0 / assign12900_body133_e18056);
        (assign12900_body133_e18057, (-(locals.var_pdj__blk372_dn0 / (assign12900_body133_e18056 * assign12900_body133_e18056))), (-(locals.var_pdj__blk372_dn2 / (assign12900_body133_e18056 * assign12900_body133_e18056))), (-(locals.var_pdj__blk372_dn6 / (assign12900_body133_e18056 * assign12900_body133_e18056))), (-(locals.var_pdj__blk372_dn7 / (assign12900_body133_e18056 * assign12900_body133_e18056))), (-(locals.var_pdj__blk372_dn10 / (assign12900_body133_e18056 * assign12900_body133_e18056))), (-(locals.var_pdj__blk372_dn11 / (assign12900_body133_e18056 * assign12900_body133_e18056))), (-(locals.var_pdj__blk372_dn12 / (assign12900_body133_e18056 * assign12900_body133_e18056))), (-(locals.var_pdj__blk372_dn17 / (assign12900_body133_e18056 * assign12900_body133_e18056))),)
    } else {
        (locals.var_pdji__blk373, locals.var_pdji__blk373_dn0, locals.var_pdji__blk373_dn2, locals.var_pdji__blk373_dn6, locals.var_pdji__blk373_dn7, locals.var_pdji__blk373_dn10, locals.var_pdji__blk373_dn11, locals.var_pdji__blk373_dn12, locals.var_pdji__blk373_dn17,)
    }
};
            locals.var_pdji__blk373 = assign12900_body133_e18059;
            locals.var_pdji__blk373_dn0 = assign12900_body133_e18059_d_n0;
            locals.var_pdji__blk373_dn2 = assign12900_body133_e18059_d_n2;
            locals.var_pdji__blk373_dn6 = assign12900_body133_e18059_d_n6;
            locals.var_pdji__blk373_dn7 = assign12900_body133_e18059_d_n7;
            locals.var_pdji__blk373_dn10 = assign12900_body133_e18059_d_n10;
            locals.var_pdji__blk373_dn11 = assign12900_body133_e18059_d_n11;
            locals.var_pdji__blk373_dn12 = assign12900_body133_e18059_d_n12;
            locals.var_pdji__blk373_dn17 = assign12900_body133_e18059_d_n17;
            let (assign12900_body134_e18075, assign12900_body134_e18075_d_n0, assign12900_body134_e18075_d_n2, assign12900_body134_e18075_d_n6, assign12900_body134_e18075_d_n7, assign12900_body134_e18075_d_n10, assign12900_body134_e18075_d_n11, assign12900_body134_e18075_d_n12, assign12900_body134_e18075_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12900_body134_e18069: f64 = (locals.var_pf22__blk367 * locals.var_pf33__blk371);
        let assign12900_body134_e18072: f64 = (locals.var_pf23__blk368 * locals.var_pf32__blk370);
        let assign12900_body134_e18073: f64 = (assign12900_body134_e18069 - assign12900_body134_e18072);
        (assign12900_body134_e18073, ((locals.var_pf22__blk367 * locals.var_pf33__blk371_dn0) - (locals.var_pf23__blk368_dn0 * locals.var_pf32__blk370)), ((locals.var_pf22__blk367 * locals.var_pf33__blk371_dn2) - (locals.var_pf23__blk368_dn2 * locals.var_pf32__blk370)), ((locals.var_pf22__blk367 * locals.var_pf33__blk371_dn6) - (locals.var_pf23__blk368_dn6 * locals.var_pf32__blk370)), ((locals.var_pf22__blk367 * locals.var_pf33__blk371_dn7) - (locals.var_pf23__blk368_dn7 * locals.var_pf32__blk370)), ((locals.var_pf22__blk367 * locals.var_pf33__blk371_dn10) - (locals.var_pf23__blk368_dn10 * locals.var_pf32__blk370)), ((locals.var_pf22__blk367 * locals.var_pf33__blk371_dn11) - (locals.var_pf23__blk368_dn11 * locals.var_pf32__blk370)), ((locals.var_pf22__blk367 * locals.var_pf33__blk371_dn12) - (locals.var_pf23__blk368_dn12 * locals.var_pf32__blk370)), ((locals.var_pf22__blk367 * locals.var_pf33__blk371_dn17) - (locals.var_pf23__blk368_dn17 * locals.var_pf32__blk370)),)
    } else {
        (locals.var_pji11__blk374, locals.var_pji11__blk374_dn0, locals.var_pji11__blk374_dn2, locals.var_pji11__blk374_dn6, locals.var_pji11__blk374_dn7, locals.var_pji11__blk374_dn10, locals.var_pji11__blk374_dn11, locals.var_pji11__blk374_dn12, locals.var_pji11__blk374_dn17,)
    }
};
            locals.var_pji11__blk374 = assign12900_body134_e18075;
            locals.var_pji11__blk374_dn0 = assign12900_body134_e18075_d_n0;
            locals.var_pji11__blk374_dn2 = assign12900_body134_e18075_d_n2;
            locals.var_pji11__blk374_dn6 = assign12900_body134_e18075_d_n6;
            locals.var_pji11__blk374_dn7 = assign12900_body134_e18075_d_n7;
            locals.var_pji11__blk374_dn10 = assign12900_body134_e18075_d_n10;
            locals.var_pji11__blk374_dn11 = assign12900_body134_e18075_d_n11;
            locals.var_pji11__blk374_dn12 = assign12900_body134_e18075_d_n12;
            locals.var_pji11__blk374_dn17 = assign12900_body134_e18075_d_n17;
            let (assign12900_body135_e18091, assign12900_body135_e18091_d_n0, assign12900_body135_e18091_d_n2, assign12900_body135_e18091_d_n6, assign12900_body135_e18091_d_n7, assign12900_body135_e18091_d_n10, assign12900_body135_e18091_d_n11, assign12900_body135_e18091_d_n12, assign12900_body135_e18091_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12900_body135_e18085: f64 = (locals.var_pf13__blk364 * locals.var_pf32__blk370);
        let assign12900_body135_e18088: f64 = (locals.var_pf12__blk363 * locals.var_pf33__blk371);
        let assign12900_body135_e18089: f64 = (assign12900_body135_e18085 - assign12900_body135_e18088);
        (assign12900_body135_e18089, ((locals.var_pf13__blk364_dn0 * locals.var_pf32__blk370) - ((locals.var_pf12__blk363_dn0 * locals.var_pf33__blk371) + (locals.var_pf12__blk363 * locals.var_pf33__blk371_dn0))), ((locals.var_pf13__blk364_dn2 * locals.var_pf32__blk370) - ((locals.var_pf12__blk363_dn2 * locals.var_pf33__blk371) + (locals.var_pf12__blk363 * locals.var_pf33__blk371_dn2))), ((locals.var_pf13__blk364_dn6 * locals.var_pf32__blk370) - ((locals.var_pf12__blk363_dn6 * locals.var_pf33__blk371) + (locals.var_pf12__blk363 * locals.var_pf33__blk371_dn6))), ((locals.var_pf13__blk364_dn7 * locals.var_pf32__blk370) - ((locals.var_pf12__blk363_dn7 * locals.var_pf33__blk371) + (locals.var_pf12__blk363 * locals.var_pf33__blk371_dn7))), ((locals.var_pf13__blk364_dn10 * locals.var_pf32__blk370) - ((locals.var_pf12__blk363_dn10 * locals.var_pf33__blk371) + (locals.var_pf12__blk363 * locals.var_pf33__blk371_dn10))), ((locals.var_pf13__blk364_dn11 * locals.var_pf32__blk370) - ((locals.var_pf12__blk363_dn11 * locals.var_pf33__blk371) + (locals.var_pf12__blk363 * locals.var_pf33__blk371_dn11))), ((locals.var_pf13__blk364_dn12 * locals.var_pf32__blk370) - ((locals.var_pf12__blk363_dn12 * locals.var_pf33__blk371) + (locals.var_pf12__blk363 * locals.var_pf33__blk371_dn12))), ((locals.var_pf13__blk364_dn17 * locals.var_pf32__blk370) - ((locals.var_pf12__blk363_dn17 * locals.var_pf33__blk371) + (locals.var_pf12__blk363 * locals.var_pf33__blk371_dn17))),)
    } else {
        (locals.var_pji12__blk375, locals.var_pji12__blk375_dn0, locals.var_pji12__blk375_dn2, locals.var_pji12__blk375_dn6, locals.var_pji12__blk375_dn7, locals.var_pji12__blk375_dn10, locals.var_pji12__blk375_dn11, locals.var_pji12__blk375_dn12, locals.var_pji12__blk375_dn17,)
    }
};
            locals.var_pji12__blk375 = assign12900_body135_e18091;
            locals.var_pji12__blk375_dn0 = assign12900_body135_e18091_d_n0;
            locals.var_pji12__blk375_dn2 = assign12900_body135_e18091_d_n2;
            locals.var_pji12__blk375_dn6 = assign12900_body135_e18091_d_n6;
            locals.var_pji12__blk375_dn7 = assign12900_body135_e18091_d_n7;
            locals.var_pji12__blk375_dn10 = assign12900_body135_e18091_d_n10;
            locals.var_pji12__blk375_dn11 = assign12900_body135_e18091_d_n11;
            locals.var_pji12__blk375_dn12 = assign12900_body135_e18091_d_n12;
            locals.var_pji12__blk375_dn17 = assign12900_body135_e18091_d_n17;
            let (assign12900_body136_e18107, assign12900_body136_e18107_d_n0, assign12900_body136_e18107_d_n2, assign12900_body136_e18107_d_n6, assign12900_body136_e18107_d_n7, assign12900_body136_e18107_d_n10, assign12900_body136_e18107_d_n11, assign12900_body136_e18107_d_n12, assign12900_body136_e18107_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12900_body136_e18101: f64 = (locals.var_pf12__blk363 * locals.var_pf23__blk368);
        let assign12900_body136_e18104: f64 = (locals.var_pf13__blk364 * locals.var_pf22__blk367);
        let assign12900_body136_e18105: f64 = (assign12900_body136_e18101 - assign12900_body136_e18104);
        (assign12900_body136_e18105, (((locals.var_pf12__blk363_dn0 * locals.var_pf23__blk368) + (locals.var_pf12__blk363 * locals.var_pf23__blk368_dn0)) - (locals.var_pf13__blk364_dn0 * locals.var_pf22__blk367)), (((locals.var_pf12__blk363_dn2 * locals.var_pf23__blk368) + (locals.var_pf12__blk363 * locals.var_pf23__blk368_dn2)) - (locals.var_pf13__blk364_dn2 * locals.var_pf22__blk367)), (((locals.var_pf12__blk363_dn6 * locals.var_pf23__blk368) + (locals.var_pf12__blk363 * locals.var_pf23__blk368_dn6)) - (locals.var_pf13__blk364_dn6 * locals.var_pf22__blk367)), (((locals.var_pf12__blk363_dn7 * locals.var_pf23__blk368) + (locals.var_pf12__blk363 * locals.var_pf23__blk368_dn7)) - (locals.var_pf13__blk364_dn7 * locals.var_pf22__blk367)), (((locals.var_pf12__blk363_dn10 * locals.var_pf23__blk368) + (locals.var_pf12__blk363 * locals.var_pf23__blk368_dn10)) - (locals.var_pf13__blk364_dn10 * locals.var_pf22__blk367)), (((locals.var_pf12__blk363_dn11 * locals.var_pf23__blk368) + (locals.var_pf12__blk363 * locals.var_pf23__blk368_dn11)) - (locals.var_pf13__blk364_dn11 * locals.var_pf22__blk367)), (((locals.var_pf12__blk363_dn12 * locals.var_pf23__blk368) + (locals.var_pf12__blk363 * locals.var_pf23__blk368_dn12)) - (locals.var_pf13__blk364_dn12 * locals.var_pf22__blk367)), (((locals.var_pf12__blk363_dn17 * locals.var_pf23__blk368) + (locals.var_pf12__blk363 * locals.var_pf23__blk368_dn17)) - (locals.var_pf13__blk364_dn17 * locals.var_pf22__blk367)),)
    } else {
        (locals.var_pji13__blk376, locals.var_pji13__blk376_dn0, locals.var_pji13__blk376_dn2, locals.var_pji13__blk376_dn6, locals.var_pji13__blk376_dn7, locals.var_pji13__blk376_dn10, locals.var_pji13__blk376_dn11, locals.var_pji13__blk376_dn12, locals.var_pji13__blk376_dn17,)
    }
};
            locals.var_pji13__blk376 = assign12900_body136_e18107;
            locals.var_pji13__blk376_dn0 = assign12900_body136_e18107_d_n0;
            locals.var_pji13__blk376_dn2 = assign12900_body136_e18107_d_n2;
            locals.var_pji13__blk376_dn6 = assign12900_body136_e18107_d_n6;
            locals.var_pji13__blk376_dn7 = assign12900_body136_e18107_d_n7;
            locals.var_pji13__blk376_dn10 = assign12900_body136_e18107_d_n10;
            locals.var_pji13__blk376_dn11 = assign12900_body136_e18107_d_n11;
            locals.var_pji13__blk376_dn12 = assign12900_body136_e18107_d_n12;
            locals.var_pji13__blk376_dn17 = assign12900_body136_e18107_d_n17;
            let (assign12900_body137_e18120, assign12900_body137_e18120_d_n0, assign12900_body137_e18120_d_n2, assign12900_body137_e18120_d_n6, assign12900_body137_e18120_d_n7, assign12900_body137_e18120_d_n10, assign12900_body137_e18120_d_n11, assign12900_body137_e18120_d_n12, assign12900_body137_e18120_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12900_body137_e18116: f64 = (-locals.var_pf21__blk366);
        let assign12900_body137_e18118: f64 = (assign12900_body137_e18116 * locals.var_pf33__blk371);
        (assign12900_body137_e18118, (assign12900_body137_e18116 * locals.var_pf33__blk371_dn0), (assign12900_body137_e18116 * locals.var_pf33__blk371_dn2), (assign12900_body137_e18116 * locals.var_pf33__blk371_dn6), (assign12900_body137_e18116 * locals.var_pf33__blk371_dn7), (assign12900_body137_e18116 * locals.var_pf33__blk371_dn10), (assign12900_body137_e18116 * locals.var_pf33__blk371_dn11), (assign12900_body137_e18116 * locals.var_pf33__blk371_dn12), (assign12900_body137_e18116 * locals.var_pf33__blk371_dn17),)
    } else {
        (locals.var_pji21__blk377, locals.var_pji21__blk377_dn0, locals.var_pji21__blk377_dn2, locals.var_pji21__blk377_dn6, locals.var_pji21__blk377_dn7, locals.var_pji21__blk377_dn10, locals.var_pji21__blk377_dn11, locals.var_pji21__blk377_dn12, locals.var_pji21__blk377_dn17,)
    }
};
            locals.var_pji21__blk377 = assign12900_body137_e18120;
            locals.var_pji21__blk377_dn0 = assign12900_body137_e18120_d_n0;
            locals.var_pji21__blk377_dn2 = assign12900_body137_e18120_d_n2;
            locals.var_pji21__blk377_dn6 = assign12900_body137_e18120_d_n6;
            locals.var_pji21__blk377_dn7 = assign12900_body137_e18120_d_n7;
            locals.var_pji21__blk377_dn10 = assign12900_body137_e18120_d_n10;
            locals.var_pji21__blk377_dn11 = assign12900_body137_e18120_d_n11;
            locals.var_pji21__blk377_dn12 = assign12900_body137_e18120_d_n12;
            locals.var_pji21__blk377_dn17 = assign12900_body137_e18120_d_n17;
            let (assign12900_body138_e18132, assign12900_body138_e18132_d_n0, assign12900_body138_e18132_d_n2, assign12900_body138_e18132_d_n6, assign12900_body138_e18132_d_n7, assign12900_body138_e18132_d_n10, assign12900_body138_e18132_d_n11, assign12900_body138_e18132_d_n12, assign12900_body138_e18132_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12900_body138_e18130: f64 = (locals.var_pf11__blk362 * locals.var_pf33__blk371);
        (assign12900_body138_e18130, ((locals.var_pf11__blk362_dn0 * locals.var_pf33__blk371) + (locals.var_pf11__blk362 * locals.var_pf33__blk371_dn0)), ((locals.var_pf11__blk362_dn2 * locals.var_pf33__blk371) + (locals.var_pf11__blk362 * locals.var_pf33__blk371_dn2)), ((locals.var_pf11__blk362_dn6 * locals.var_pf33__blk371) + (locals.var_pf11__blk362 * locals.var_pf33__blk371_dn6)), ((locals.var_pf11__blk362_dn7 * locals.var_pf33__blk371) + (locals.var_pf11__blk362 * locals.var_pf33__blk371_dn7)), ((locals.var_pf11__blk362_dn10 * locals.var_pf33__blk371) + (locals.var_pf11__blk362 * locals.var_pf33__blk371_dn10)), ((locals.var_pf11__blk362_dn11 * locals.var_pf33__blk371) + (locals.var_pf11__blk362 * locals.var_pf33__blk371_dn11)), ((locals.var_pf11__blk362_dn12 * locals.var_pf33__blk371) + (locals.var_pf11__blk362 * locals.var_pf33__blk371_dn12)), ((locals.var_pf11__blk362_dn17 * locals.var_pf33__blk371) + (locals.var_pf11__blk362 * locals.var_pf33__blk371_dn17)),)
    } else {
        (locals.var_pji22__blk378, locals.var_pji22__blk378_dn0, locals.var_pji22__blk378_dn2, locals.var_pji22__blk378_dn6, locals.var_pji22__blk378_dn7, locals.var_pji22__blk378_dn10, locals.var_pji22__blk378_dn11, locals.var_pji22__blk378_dn12, locals.var_pji22__blk378_dn17,)
    }
};
            locals.var_pji22__blk378 = assign12900_body138_e18132;
            locals.var_pji22__blk378_dn0 = assign12900_body138_e18132_d_n0;
            locals.var_pji22__blk378_dn2 = assign12900_body138_e18132_d_n2;
            locals.var_pji22__blk378_dn6 = assign12900_body138_e18132_d_n6;
            locals.var_pji22__blk378_dn7 = assign12900_body138_e18132_d_n7;
            locals.var_pji22__blk378_dn10 = assign12900_body138_e18132_d_n10;
            locals.var_pji22__blk378_dn11 = assign12900_body138_e18132_d_n11;
            locals.var_pji22__blk378_dn12 = assign12900_body138_e18132_d_n12;
            locals.var_pji22__blk378_dn17 = assign12900_body138_e18132_d_n17;
            let (assign12900_body139_e18148, assign12900_body139_e18148_d_n0, assign12900_body139_e18148_d_n2, assign12900_body139_e18148_d_n6, assign12900_body139_e18148_d_n7, assign12900_body139_e18148_d_n10, assign12900_body139_e18148_d_n11, assign12900_body139_e18148_d_n12, assign12900_body139_e18148_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12900_body139_e18142: f64 = (locals.var_pf13__blk364 * locals.var_pf21__blk366);
        let assign12900_body139_e18145: f64 = (locals.var_pf11__blk362 * locals.var_pf23__blk368);
        let assign12900_body139_e18146: f64 = (assign12900_body139_e18142 - assign12900_body139_e18145);
        (assign12900_body139_e18146, ((locals.var_pf13__blk364_dn0 * locals.var_pf21__blk366) - ((locals.var_pf11__blk362_dn0 * locals.var_pf23__blk368) + (locals.var_pf11__blk362 * locals.var_pf23__blk368_dn0))), ((locals.var_pf13__blk364_dn2 * locals.var_pf21__blk366) - ((locals.var_pf11__blk362_dn2 * locals.var_pf23__blk368) + (locals.var_pf11__blk362 * locals.var_pf23__blk368_dn2))), ((locals.var_pf13__blk364_dn6 * locals.var_pf21__blk366) - ((locals.var_pf11__blk362_dn6 * locals.var_pf23__blk368) + (locals.var_pf11__blk362 * locals.var_pf23__blk368_dn6))), ((locals.var_pf13__blk364_dn7 * locals.var_pf21__blk366) - ((locals.var_pf11__blk362_dn7 * locals.var_pf23__blk368) + (locals.var_pf11__blk362 * locals.var_pf23__blk368_dn7))), ((locals.var_pf13__blk364_dn10 * locals.var_pf21__blk366) - ((locals.var_pf11__blk362_dn10 * locals.var_pf23__blk368) + (locals.var_pf11__blk362 * locals.var_pf23__blk368_dn10))), ((locals.var_pf13__blk364_dn11 * locals.var_pf21__blk366) - ((locals.var_pf11__blk362_dn11 * locals.var_pf23__blk368) + (locals.var_pf11__blk362 * locals.var_pf23__blk368_dn11))), ((locals.var_pf13__blk364_dn12 * locals.var_pf21__blk366) - ((locals.var_pf11__blk362_dn12 * locals.var_pf23__blk368) + (locals.var_pf11__blk362 * locals.var_pf23__blk368_dn12))), ((locals.var_pf13__blk364_dn17 * locals.var_pf21__blk366) - ((locals.var_pf11__blk362_dn17 * locals.var_pf23__blk368) + (locals.var_pf11__blk362 * locals.var_pf23__blk368_dn17))),)
    } else {
        (locals.var_pji23__blk379, locals.var_pji23__blk379_dn0, locals.var_pji23__blk379_dn2, locals.var_pji23__blk379_dn6, locals.var_pji23__blk379_dn7, locals.var_pji23__blk379_dn10, locals.var_pji23__blk379_dn11, locals.var_pji23__blk379_dn12, locals.var_pji23__blk379_dn17,)
    }
};
            locals.var_pji23__blk379 = assign12900_body139_e18148;
            locals.var_pji23__blk379_dn0 = assign12900_body139_e18148_d_n0;
            locals.var_pji23__blk379_dn2 = assign12900_body139_e18148_d_n2;
            locals.var_pji23__blk379_dn6 = assign12900_body139_e18148_d_n6;
            locals.var_pji23__blk379_dn7 = assign12900_body139_e18148_d_n7;
            locals.var_pji23__blk379_dn10 = assign12900_body139_e18148_d_n10;
            locals.var_pji23__blk379_dn11 = assign12900_body139_e18148_d_n11;
            locals.var_pji23__blk379_dn12 = assign12900_body139_e18148_d_n12;
            locals.var_pji23__blk379_dn17 = assign12900_body139_e18148_d_n17;
            let (assign12900_body140_e18160,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12900_body140_e18158: f64 = (locals.var_pf21__blk366 * locals.var_pf32__blk370);
        (assign12900_body140_e18158,)
    } else {
        (locals.var_pji31__blk380,)
    }
};
            locals.var_pji31__blk380 = assign12900_body140_e18160;
            let (assign12900_body141_e18173, assign12900_body141_e18173_d_n0, assign12900_body141_e18173_d_n2, assign12900_body141_e18173_d_n6, assign12900_body141_e18173_d_n7, assign12900_body141_e18173_d_n10, assign12900_body141_e18173_d_n11, assign12900_body141_e18173_d_n12, assign12900_body141_e18173_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12900_body141_e18169: f64 = (-locals.var_pf11__blk362);
        let assign12900_body141_e18171: f64 = (assign12900_body141_e18169 * locals.var_pf32__blk370);
        (assign12900_body141_e18171, ((-locals.var_pf11__blk362_dn0) * locals.var_pf32__blk370), ((-locals.var_pf11__blk362_dn2) * locals.var_pf32__blk370), ((-locals.var_pf11__blk362_dn6) * locals.var_pf32__blk370), ((-locals.var_pf11__blk362_dn7) * locals.var_pf32__blk370), ((-locals.var_pf11__blk362_dn10) * locals.var_pf32__blk370), ((-locals.var_pf11__blk362_dn11) * locals.var_pf32__blk370), ((-locals.var_pf11__blk362_dn12) * locals.var_pf32__blk370), ((-locals.var_pf11__blk362_dn17) * locals.var_pf32__blk370),)
    } else {
        (locals.var_pji32__blk381, locals.var_pji32__blk381_dn0, locals.var_pji32__blk381_dn2, locals.var_pji32__blk381_dn6, locals.var_pji32__blk381_dn7, locals.var_pji32__blk381_dn10, locals.var_pji32__blk381_dn11, locals.var_pji32__blk381_dn12, locals.var_pji32__blk381_dn17,)
    }
};
            locals.var_pji32__blk381 = assign12900_body141_e18173;
            locals.var_pji32__blk381_dn0 = assign12900_body141_e18173_d_n0;
            locals.var_pji32__blk381_dn2 = assign12900_body141_e18173_d_n2;
            locals.var_pji32__blk381_dn6 = assign12900_body141_e18173_d_n6;
            locals.var_pji32__blk381_dn7 = assign12900_body141_e18173_d_n7;
            locals.var_pji32__blk381_dn10 = assign12900_body141_e18173_d_n10;
            locals.var_pji32__blk381_dn11 = assign12900_body141_e18173_d_n11;
            locals.var_pji32__blk381_dn12 = assign12900_body141_e18173_d_n12;
            locals.var_pji32__blk381_dn17 = assign12900_body141_e18173_d_n17;
            let (assign12900_body142_e18189, assign12900_body142_e18189_d_n0, assign12900_body142_e18189_d_n2, assign12900_body142_e18189_d_n6, assign12900_body142_e18189_d_n7, assign12900_body142_e18189_d_n10, assign12900_body142_e18189_d_n11, assign12900_body142_e18189_d_n12, assign12900_body142_e18189_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12900_body142_e18183: f64 = (locals.var_pf11__blk362 * locals.var_pf22__blk367);
        let assign12900_body142_e18186: f64 = (locals.var_pf12__blk363 * locals.var_pf21__blk366);
        let assign12900_body142_e18187: f64 = (assign12900_body142_e18183 - assign12900_body142_e18186);
        (assign12900_body142_e18187, ((locals.var_pf11__blk362_dn0 * locals.var_pf22__blk367) - (locals.var_pf12__blk363_dn0 * locals.var_pf21__blk366)), ((locals.var_pf11__blk362_dn2 * locals.var_pf22__blk367) - (locals.var_pf12__blk363_dn2 * locals.var_pf21__blk366)), ((locals.var_pf11__blk362_dn6 * locals.var_pf22__blk367) - (locals.var_pf12__blk363_dn6 * locals.var_pf21__blk366)), ((locals.var_pf11__blk362_dn7 * locals.var_pf22__blk367) - (locals.var_pf12__blk363_dn7 * locals.var_pf21__blk366)), ((locals.var_pf11__blk362_dn10 * locals.var_pf22__blk367) - (locals.var_pf12__blk363_dn10 * locals.var_pf21__blk366)), ((locals.var_pf11__blk362_dn11 * locals.var_pf22__blk367) - (locals.var_pf12__blk363_dn11 * locals.var_pf21__blk366)), ((locals.var_pf11__blk362_dn12 * locals.var_pf22__blk367) - (locals.var_pf12__blk363_dn12 * locals.var_pf21__blk366)), ((locals.var_pf11__blk362_dn17 * locals.var_pf22__blk367) - (locals.var_pf12__blk363_dn17 * locals.var_pf21__blk366)),)
    } else {
        (locals.var_pji33__blk382, locals.var_pji33__blk382_dn0, locals.var_pji33__blk382_dn2, locals.var_pji33__blk382_dn6, locals.var_pji33__blk382_dn7, locals.var_pji33__blk382_dn10, locals.var_pji33__blk382_dn11, locals.var_pji33__blk382_dn12, locals.var_pji33__blk382_dn17,)
    }
};
            locals.var_pji33__blk382 = assign12900_body142_e18189;
            locals.var_pji33__blk382_dn0 = assign12900_body142_e18189_d_n0;
            locals.var_pji33__blk382_dn2 = assign12900_body142_e18189_d_n2;
            locals.var_pji33__blk382_dn6 = assign12900_body142_e18189_d_n6;
            locals.var_pji33__blk382_dn7 = assign12900_body142_e18189_d_n7;
            locals.var_pji33__blk382_dn10 = assign12900_body142_e18189_d_n10;
            locals.var_pji33__blk382_dn11 = assign12900_body142_e18189_d_n11;
            locals.var_pji33__blk382_dn12 = assign12900_body142_e18189_d_n12;
            locals.var_pji33__blk382_dn17 = assign12900_body142_e18189_d_n17;
            let (assign12900_body143_e18212, assign12900_body143_e18212_d_n0, assign12900_body143_e18212_d_n2, assign12900_body143_e18212_d_n6, assign12900_body143_e18212_d_n7, assign12900_body143_e18212_d_n10, assign12900_body143_e18212_d_n11, assign12900_body143_e18212_d_n12, assign12900_body143_e18212_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12900_body143_e18198: f64 = (-locals.var_pdji__blk373);
        let assign12900_body143_e18201: f64 = (locals.var_pji11__blk374 * locals.var_pf1__blk361);
        let assign12900_body143_e18204: f64 = (locals.var_pji12__blk375 * locals.var_pf2__blk365);
        let assign12900_body143_e18205: f64 = (assign12900_body143_e18201 + assign12900_body143_e18204);
        let assign12900_body143_e18208: f64 = (locals.var_pji13__blk376 * locals.var_pf3__blk369);
        let assign12900_body143_e18209: f64 = (assign12900_body143_e18205 + assign12900_body143_e18208);
        let assign12900_body143_e18210: f64 = (assign12900_body143_e18198 * assign12900_body143_e18209);
        (assign12900_body143_e18210, (((-locals.var_pdji__blk373_dn0) * assign12900_body143_e18209) + (assign12900_body143_e18198 * ((((locals.var_pji11__blk374_dn0 * locals.var_pf1__blk361) + (locals.var_pji11__blk374 * locals.var_pf1__blk361_dn0)) + ((locals.var_pji12__blk375_dn0 * locals.var_pf2__blk365) + (locals.var_pji12__blk375 * locals.var_pf2__blk365_dn0))) + ((locals.var_pji13__blk376_dn0 * locals.var_pf3__blk369) + (locals.var_pji13__blk376 * locals.var_pf3__blk369_dn0))))), (((-locals.var_pdji__blk373_dn2) * assign12900_body143_e18209) + (assign12900_body143_e18198 * ((((locals.var_pji11__blk374_dn2 * locals.var_pf1__blk361) + (locals.var_pji11__blk374 * locals.var_pf1__blk361_dn2)) + ((locals.var_pji12__blk375_dn2 * locals.var_pf2__blk365) + (locals.var_pji12__blk375 * locals.var_pf2__blk365_dn2))) + ((locals.var_pji13__blk376_dn2 * locals.var_pf3__blk369) + (locals.var_pji13__blk376 * locals.var_pf3__blk369_dn2))))), (((-locals.var_pdji__blk373_dn6) * assign12900_body143_e18209) + (assign12900_body143_e18198 * ((((locals.var_pji11__blk374_dn6 * locals.var_pf1__blk361) + (locals.var_pji11__blk374 * locals.var_pf1__blk361_dn6)) + ((locals.var_pji12__blk375_dn6 * locals.var_pf2__blk365) + (locals.var_pji12__blk375 * locals.var_pf2__blk365_dn6))) + ((locals.var_pji13__blk376_dn6 * locals.var_pf3__blk369) + (locals.var_pji13__blk376 * locals.var_pf3__blk369_dn6))))), (((-locals.var_pdji__blk373_dn7) * assign12900_body143_e18209) + (assign12900_body143_e18198 * ((((locals.var_pji11__blk374_dn7 * locals.var_pf1__blk361) + (locals.var_pji11__blk374 * locals.var_pf1__blk361_dn7)) + ((locals.var_pji12__blk375_dn7 * locals.var_pf2__blk365) + (locals.var_pji12__blk375 * locals.var_pf2__blk365_dn7))) + ((locals.var_pji13__blk376_dn7 * locals.var_pf3__blk369) + (locals.var_pji13__blk376 * locals.var_pf3__blk369_dn7))))), (((-locals.var_pdji__blk373_dn10) * assign12900_body143_e18209) + (assign12900_body143_e18198 * ((((locals.var_pji11__blk374_dn10 * locals.var_pf1__blk361) + (locals.var_pji11__blk374 * locals.var_pf1__blk361_dn10)) + ((locals.var_pji12__blk375_dn10 * locals.var_pf2__blk365) + (locals.var_pji12__blk375 * locals.var_pf2__blk365_dn10))) + ((locals.var_pji13__blk376_dn10 * locals.var_pf3__blk369) + (locals.var_pji13__blk376 * locals.var_pf3__blk369_dn10))))), (((-locals.var_pdji__blk373_dn11) * assign12900_body143_e18209) + (assign12900_body143_e18198 * ((((locals.var_pji11__blk374_dn11 * locals.var_pf1__blk361) + (locals.var_pji11__blk374 * locals.var_pf1__blk361_dn11)) + ((locals.var_pji12__blk375_dn11 * locals.var_pf2__blk365) + (locals.var_pji12__blk375 * locals.var_pf2__blk365_dn11))) + ((locals.var_pji13__blk376_dn11 * locals.var_pf3__blk369) + (locals.var_pji13__blk376 * locals.var_pf3__blk369_dn11))))), (((-locals.var_pdji__blk373_dn12) * assign12900_body143_e18209) + (assign12900_body143_e18198 * ((((locals.var_pji11__blk374_dn12 * locals.var_pf1__blk361) + (locals.var_pji11__blk374 * locals.var_pf1__blk361_dn12)) + ((locals.var_pji12__blk375_dn12 * locals.var_pf2__blk365) + (locals.var_pji12__blk375 * locals.var_pf2__blk365_dn12))) + ((locals.var_pji13__blk376_dn12 * locals.var_pf3__blk369) + (locals.var_pji13__blk376 * locals.var_pf3__blk369_dn12))))), (((-locals.var_pdji__blk373_dn17) * assign12900_body143_e18209) + (assign12900_body143_e18198 * ((((locals.var_pji11__blk374_dn17 * locals.var_pf1__blk361) + (locals.var_pji11__blk374 * locals.var_pf1__blk361_dn17)) + ((locals.var_pji12__blk375_dn17 * locals.var_pf2__blk365) + (locals.var_pji12__blk375 * locals.var_pf2__blk365_dn17))) + ((locals.var_pji13__blk376_dn17 * locals.var_pf3__blk369) + (locals.var_pji13__blk376 * locals.var_pf3__blk369_dn17))))),)
    } else {
        (locals.var_dpss__blk358, locals.var_dpss__blk358_dn0, locals.var_dpss__blk358_dn2, locals.var_dpss__blk358_dn6, locals.var_dpss__blk358_dn7, locals.var_dpss__blk358_dn10, locals.var_dpss__blk358_dn11, locals.var_dpss__blk358_dn12, locals.var_dpss__blk358_dn17,)
    }
};
            locals.var_dpss__blk358 = assign12900_body143_e18212;
            locals.var_dpss__blk358_dn0 = assign12900_body143_e18212_d_n0;
            locals.var_dpss__blk358_dn2 = assign12900_body143_e18212_d_n2;
            locals.var_dpss__blk358_dn6 = assign12900_body143_e18212_d_n6;
            locals.var_dpss__blk358_dn7 = assign12900_body143_e18212_d_n7;
            locals.var_dpss__blk358_dn10 = assign12900_body143_e18212_d_n10;
            locals.var_dpss__blk358_dn11 = assign12900_body143_e18212_d_n11;
            locals.var_dpss__blk358_dn12 = assign12900_body143_e18212_d_n12;
            locals.var_dpss__blk358_dn17 = assign12900_body143_e18212_d_n17;
            let (assign12900_body144_e18235, assign12900_body144_e18235_d_n0, assign12900_body144_e18235_d_n2, assign12900_body144_e18235_d_n6, assign12900_body144_e18235_d_n7, assign12900_body144_e18235_d_n10, assign12900_body144_e18235_d_n11, assign12900_body144_e18235_d_n12, assign12900_body144_e18235_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12900_body144_e18221: f64 = (-locals.var_pdji__blk373);
        let assign12900_body144_e18224: f64 = (locals.var_pji21__blk377 * locals.var_pf1__blk361);
        let assign12900_body144_e18227: f64 = (locals.var_pji22__blk378 * locals.var_pf2__blk365);
        let assign12900_body144_e18228: f64 = (assign12900_body144_e18224 + assign12900_body144_e18227);
        let assign12900_body144_e18231: f64 = (locals.var_pji23__blk379 * locals.var_pf3__blk369);
        let assign12900_body144_e18232: f64 = (assign12900_body144_e18228 + assign12900_body144_e18231);
        let assign12900_body144_e18233: f64 = (assign12900_body144_e18221 * assign12900_body144_e18232);
        (assign12900_body144_e18233, (((-locals.var_pdji__blk373_dn0) * assign12900_body144_e18232) + (assign12900_body144_e18221 * ((((locals.var_pji21__blk377_dn0 * locals.var_pf1__blk361) + (locals.var_pji21__blk377 * locals.var_pf1__blk361_dn0)) + ((locals.var_pji22__blk378_dn0 * locals.var_pf2__blk365) + (locals.var_pji22__blk378 * locals.var_pf2__blk365_dn0))) + ((locals.var_pji23__blk379_dn0 * locals.var_pf3__blk369) + (locals.var_pji23__blk379 * locals.var_pf3__blk369_dn0))))), (((-locals.var_pdji__blk373_dn2) * assign12900_body144_e18232) + (assign12900_body144_e18221 * ((((locals.var_pji21__blk377_dn2 * locals.var_pf1__blk361) + (locals.var_pji21__blk377 * locals.var_pf1__blk361_dn2)) + ((locals.var_pji22__blk378_dn2 * locals.var_pf2__blk365) + (locals.var_pji22__blk378 * locals.var_pf2__blk365_dn2))) + ((locals.var_pji23__blk379_dn2 * locals.var_pf3__blk369) + (locals.var_pji23__blk379 * locals.var_pf3__blk369_dn2))))), (((-locals.var_pdji__blk373_dn6) * assign12900_body144_e18232) + (assign12900_body144_e18221 * ((((locals.var_pji21__blk377_dn6 * locals.var_pf1__blk361) + (locals.var_pji21__blk377 * locals.var_pf1__blk361_dn6)) + ((locals.var_pji22__blk378_dn6 * locals.var_pf2__blk365) + (locals.var_pji22__blk378 * locals.var_pf2__blk365_dn6))) + ((locals.var_pji23__blk379_dn6 * locals.var_pf3__blk369) + (locals.var_pji23__blk379 * locals.var_pf3__blk369_dn6))))), (((-locals.var_pdji__blk373_dn7) * assign12900_body144_e18232) + (assign12900_body144_e18221 * ((((locals.var_pji21__blk377_dn7 * locals.var_pf1__blk361) + (locals.var_pji21__blk377 * locals.var_pf1__blk361_dn7)) + ((locals.var_pji22__blk378_dn7 * locals.var_pf2__blk365) + (locals.var_pji22__blk378 * locals.var_pf2__blk365_dn7))) + ((locals.var_pji23__blk379_dn7 * locals.var_pf3__blk369) + (locals.var_pji23__blk379 * locals.var_pf3__blk369_dn7))))), (((-locals.var_pdji__blk373_dn10) * assign12900_body144_e18232) + (assign12900_body144_e18221 * ((((locals.var_pji21__blk377_dn10 * locals.var_pf1__blk361) + (locals.var_pji21__blk377 * locals.var_pf1__blk361_dn10)) + ((locals.var_pji22__blk378_dn10 * locals.var_pf2__blk365) + (locals.var_pji22__blk378 * locals.var_pf2__blk365_dn10))) + ((locals.var_pji23__blk379_dn10 * locals.var_pf3__blk369) + (locals.var_pji23__blk379 * locals.var_pf3__blk369_dn10))))), (((-locals.var_pdji__blk373_dn11) * assign12900_body144_e18232) + (assign12900_body144_e18221 * ((((locals.var_pji21__blk377_dn11 * locals.var_pf1__blk361) + (locals.var_pji21__blk377 * locals.var_pf1__blk361_dn11)) + ((locals.var_pji22__blk378_dn11 * locals.var_pf2__blk365) + (locals.var_pji22__blk378 * locals.var_pf2__blk365_dn11))) + ((locals.var_pji23__blk379_dn11 * locals.var_pf3__blk369) + (locals.var_pji23__blk379 * locals.var_pf3__blk369_dn11))))), (((-locals.var_pdji__blk373_dn12) * assign12900_body144_e18232) + (assign12900_body144_e18221 * ((((locals.var_pji21__blk377_dn12 * locals.var_pf1__blk361) + (locals.var_pji21__blk377 * locals.var_pf1__blk361_dn12)) + ((locals.var_pji22__blk378_dn12 * locals.var_pf2__blk365) + (locals.var_pji22__blk378 * locals.var_pf2__blk365_dn12))) + ((locals.var_pji23__blk379_dn12 * locals.var_pf3__blk369) + (locals.var_pji23__blk379 * locals.var_pf3__blk369_dn12))))), (((-locals.var_pdji__blk373_dn17) * assign12900_body144_e18232) + (assign12900_body144_e18221 * ((((locals.var_pji21__blk377_dn17 * locals.var_pf1__blk361) + (locals.var_pji21__blk377 * locals.var_pf1__blk361_dn17)) + ((locals.var_pji22__blk378_dn17 * locals.var_pf2__blk365) + (locals.var_pji22__blk378 * locals.var_pf2__blk365_dn17))) + ((locals.var_pji23__blk379_dn17 * locals.var_pf3__blk369) + (locals.var_pji23__blk379 * locals.var_pf3__blk369_dn17))))),)
    } else {
        (locals.var_dpbs__blk359, locals.var_dpbs__blk359_dn0, locals.var_dpbs__blk359_dn2, locals.var_dpbs__blk359_dn6, locals.var_dpbs__blk359_dn7, locals.var_dpbs__blk359_dn10, locals.var_dpbs__blk359_dn11, locals.var_dpbs__blk359_dn12, locals.var_dpbs__blk359_dn17,)
    }
};
            locals.var_dpbs__blk359 = assign12900_body144_e18235;
            locals.var_dpbs__blk359_dn0 = assign12900_body144_e18235_d_n0;
            locals.var_dpbs__blk359_dn2 = assign12900_body144_e18235_d_n2;
            locals.var_dpbs__blk359_dn6 = assign12900_body144_e18235_d_n6;
            locals.var_dpbs__blk359_dn7 = assign12900_body144_e18235_d_n7;
            locals.var_dpbs__blk359_dn10 = assign12900_body144_e18235_d_n10;
            locals.var_dpbs__blk359_dn11 = assign12900_body144_e18235_d_n11;
            locals.var_dpbs__blk359_dn12 = assign12900_body144_e18235_d_n12;
            locals.var_dpbs__blk359_dn17 = assign12900_body144_e18235_d_n17;
            let (assign12900_body145_e18258, assign12900_body145_e18258_d_n0, assign12900_body145_e18258_d_n2, assign12900_body145_e18258_d_n6, assign12900_body145_e18258_d_n7, assign12900_body145_e18258_d_n10, assign12900_body145_e18258_d_n11, assign12900_body145_e18258_d_n12, assign12900_body145_e18258_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12900_body145_e18244: f64 = (-locals.var_pdji__blk373);
        let assign12900_body145_e18247: f64 = (locals.var_pji31__blk380 * locals.var_pf1__blk361);
        let assign12900_body145_e18250: f64 = (locals.var_pji32__blk381 * locals.var_pf2__blk365);
        let assign12900_body145_e18251: f64 = (assign12900_body145_e18247 + assign12900_body145_e18250);
        let assign12900_body145_e18254: f64 = (locals.var_pji33__blk382 * locals.var_pf3__blk369);
        let assign12900_body145_e18255: f64 = (assign12900_body145_e18251 + assign12900_body145_e18254);
        let assign12900_body145_e18256: f64 = (assign12900_body145_e18244 * assign12900_body145_e18255);
        (assign12900_body145_e18256, (((-locals.var_pdji__blk373_dn0) * assign12900_body145_e18255) + (assign12900_body145_e18244 * (((locals.var_pji31__blk380 * locals.var_pf1__blk361_dn0) + ((locals.var_pji32__blk381_dn0 * locals.var_pf2__blk365) + (locals.var_pji32__blk381 * locals.var_pf2__blk365_dn0))) + ((locals.var_pji33__blk382_dn0 * locals.var_pf3__blk369) + (locals.var_pji33__blk382 * locals.var_pf3__blk369_dn0))))), (((-locals.var_pdji__blk373_dn2) * assign12900_body145_e18255) + (assign12900_body145_e18244 * (((locals.var_pji31__blk380 * locals.var_pf1__blk361_dn2) + ((locals.var_pji32__blk381_dn2 * locals.var_pf2__blk365) + (locals.var_pji32__blk381 * locals.var_pf2__blk365_dn2))) + ((locals.var_pji33__blk382_dn2 * locals.var_pf3__blk369) + (locals.var_pji33__blk382 * locals.var_pf3__blk369_dn2))))), (((-locals.var_pdji__blk373_dn6) * assign12900_body145_e18255) + (assign12900_body145_e18244 * (((locals.var_pji31__blk380 * locals.var_pf1__blk361_dn6) + ((locals.var_pji32__blk381_dn6 * locals.var_pf2__blk365) + (locals.var_pji32__blk381 * locals.var_pf2__blk365_dn6))) + ((locals.var_pji33__blk382_dn6 * locals.var_pf3__blk369) + (locals.var_pji33__blk382 * locals.var_pf3__blk369_dn6))))), (((-locals.var_pdji__blk373_dn7) * assign12900_body145_e18255) + (assign12900_body145_e18244 * (((locals.var_pji31__blk380 * locals.var_pf1__blk361_dn7) + ((locals.var_pji32__blk381_dn7 * locals.var_pf2__blk365) + (locals.var_pji32__blk381 * locals.var_pf2__blk365_dn7))) + ((locals.var_pji33__blk382_dn7 * locals.var_pf3__blk369) + (locals.var_pji33__blk382 * locals.var_pf3__blk369_dn7))))), (((-locals.var_pdji__blk373_dn10) * assign12900_body145_e18255) + (assign12900_body145_e18244 * (((locals.var_pji31__blk380 * locals.var_pf1__blk361_dn10) + ((locals.var_pji32__blk381_dn10 * locals.var_pf2__blk365) + (locals.var_pji32__blk381 * locals.var_pf2__blk365_dn10))) + ((locals.var_pji33__blk382_dn10 * locals.var_pf3__blk369) + (locals.var_pji33__blk382 * locals.var_pf3__blk369_dn10))))), (((-locals.var_pdji__blk373_dn11) * assign12900_body145_e18255) + (assign12900_body145_e18244 * (((locals.var_pji31__blk380 * locals.var_pf1__blk361_dn11) + ((locals.var_pji32__blk381_dn11 * locals.var_pf2__blk365) + (locals.var_pji32__blk381 * locals.var_pf2__blk365_dn11))) + ((locals.var_pji33__blk382_dn11 * locals.var_pf3__blk369) + (locals.var_pji33__blk382 * locals.var_pf3__blk369_dn11))))), (((-locals.var_pdji__blk373_dn12) * assign12900_body145_e18255) + (assign12900_body145_e18244 * (((locals.var_pji31__blk380 * locals.var_pf1__blk361_dn12) + ((locals.var_pji32__blk381_dn12 * locals.var_pf2__blk365) + (locals.var_pji32__blk381 * locals.var_pf2__blk365_dn12))) + ((locals.var_pji33__blk382_dn12 * locals.var_pf3__blk369) + (locals.var_pji33__blk382 * locals.var_pf3__blk369_dn12))))), (((-locals.var_pdji__blk373_dn17) * assign12900_body145_e18255) + (assign12900_body145_e18244 * (((locals.var_pji31__blk380 * locals.var_pf1__blk361_dn17) + ((locals.var_pji32__blk381_dn17 * locals.var_pf2__blk365) + (locals.var_pji32__blk381 * locals.var_pf2__blk365_dn17))) + ((locals.var_pji33__blk382_dn17 * locals.var_pf3__blk369) + (locals.var_pji33__blk382 * locals.var_pf3__blk369_dn17))))),)
    } else {
        (locals.var_dpsb__blk360, locals.var_dpsb__blk360_dn0, locals.var_dpsb__blk360_dn2, locals.var_dpsb__blk360_dn6, locals.var_dpsb__blk360_dn7, locals.var_dpsb__blk360_dn10, locals.var_dpsb__blk360_dn11, locals.var_dpsb__blk360_dn12, locals.var_dpsb__blk360_dn17,)
    }
};
            locals.var_dpsb__blk360 = assign12900_body145_e18258;
            locals.var_dpsb__blk360_dn0 = assign12900_body145_e18258_d_n0;
            locals.var_dpsb__blk360_dn2 = assign12900_body145_e18258_d_n2;
            locals.var_dpsb__blk360_dn6 = assign12900_body145_e18258_d_n6;
            locals.var_dpsb__blk360_dn7 = assign12900_body145_e18258_d_n7;
            locals.var_dpsb__blk360_dn10 = assign12900_body145_e18258_d_n10;
            locals.var_dpsb__blk360_dn11 = assign12900_body145_e18258_d_n11;
            locals.var_dpsb__blk360_dn12 = assign12900_body145_e18258_d_n12;
            locals.var_dpsb__blk360_dn17 = assign12900_body145_e18258_d_n17;
            let (assign12900_body146_e18269, assign12900_body146_e18269_d_n0, assign12900_body146_e18269_d_n2, assign12900_body146_e18269_d_n6, assign12900_body146_e18269_d_n7, assign12900_body146_e18269_d_n10, assign12900_body146_e18269_d_n11, assign12900_body146_e18269_d_n12, assign12900_body146_e18269_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12900_body146_e18267: f64 = (locals.var_dpss__blk358).abs();
        (assign12900_body146_e18267, if locals.var_dpss__blk358 >= 0.0 { locals.var_dpss__blk358_dn0 } else { (-locals.var_dpss__blk358_dn0) }, if locals.var_dpss__blk358 >= 0.0 { locals.var_dpss__blk358_dn2 } else { (-locals.var_dpss__blk358_dn2) }, if locals.var_dpss__blk358 >= 0.0 { locals.var_dpss__blk358_dn6 } else { (-locals.var_dpss__blk358_dn6) }, if locals.var_dpss__blk358 >= 0.0 { locals.var_dpss__blk358_dn7 } else { (-locals.var_dpss__blk358_dn7) }, if locals.var_dpss__blk358 >= 0.0 { locals.var_dpss__blk358_dn10 } else { (-locals.var_dpss__blk358_dn10) }, if locals.var_dpss__blk358 >= 0.0 { locals.var_dpss__blk358_dn11 } else { (-locals.var_dpss__blk358_dn11) }, if locals.var_dpss__blk358 >= 0.0 { locals.var_dpss__blk358_dn12 } else { (-locals.var_dpss__blk358_dn12) }, if locals.var_dpss__blk358 >= 0.0 { locals.var_dpss__blk358_dn17 } else { (-locals.var_dpss__blk358_dn17) },)
    } else {
        (locals.var_t1__blk351, locals.var_t1__blk351_dn0, locals.var_t1__blk351_dn2, locals.var_t1__blk351_dn6, locals.var_t1__blk351_dn7, locals.var_t1__blk351_dn10, locals.var_t1__blk351_dn11, locals.var_t1__blk351_dn12, locals.var_t1__blk351_dn17,)
    }
};
            locals.var_t1__blk351 = assign12900_body146_e18269;
            locals.var_t1__blk351_dn0 = assign12900_body146_e18269_d_n0;
            locals.var_t1__blk351_dn2 = assign12900_body146_e18269_d_n2;
            locals.var_t1__blk351_dn6 = assign12900_body146_e18269_d_n6;
            locals.var_t1__blk351_dn7 = assign12900_body146_e18269_d_n7;
            locals.var_t1__blk351_dn10 = assign12900_body146_e18269_d_n10;
            locals.var_t1__blk351_dn11 = assign12900_body146_e18269_d_n11;
            locals.var_t1__blk351_dn12 = assign12900_body146_e18269_d_n12;
            locals.var_t1__blk351_dn17 = assign12900_body146_e18269_d_n17;
            let assign12900_body147_e18272: f64 = (locals.var_dpbs__blk359).abs();
            let assign12900_body147_e18273: f64 = if locals.var_t1__blk351 < assign12900_body147_e18272 { 1.0 } else { 0.0 };
            locals.var_guard405 = assign12900_body147_e18273;
            let (assign12900_body148_e18286, assign12900_body148_e18286_d_n0, assign12900_body148_e18286_d_n2, assign12900_body148_e18286_d_n6, assign12900_body148_e18286_d_n7, assign12900_body148_e18286_d_n10, assign12900_body148_e18286_d_n11, assign12900_body148_e18286_d_n12, assign12900_body148_e18286_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) && (locals.var_guard405 != 0.0)) {
        let assign12900_body148_e18284: f64 = (locals.var_dpbs__blk359).abs();
        (assign12900_body148_e18284, if locals.var_dpbs__blk359 >= 0.0 { locals.var_dpbs__blk359_dn0 } else { (-locals.var_dpbs__blk359_dn0) }, if locals.var_dpbs__blk359 >= 0.0 { locals.var_dpbs__blk359_dn2 } else { (-locals.var_dpbs__blk359_dn2) }, if locals.var_dpbs__blk359 >= 0.0 { locals.var_dpbs__blk359_dn6 } else { (-locals.var_dpbs__blk359_dn6) }, if locals.var_dpbs__blk359 >= 0.0 { locals.var_dpbs__blk359_dn7 } else { (-locals.var_dpbs__blk359_dn7) }, if locals.var_dpbs__blk359 >= 0.0 { locals.var_dpbs__blk359_dn10 } else { (-locals.var_dpbs__blk359_dn10) }, if locals.var_dpbs__blk359 >= 0.0 { locals.var_dpbs__blk359_dn11 } else { (-locals.var_dpbs__blk359_dn11) }, if locals.var_dpbs__blk359 >= 0.0 { locals.var_dpbs__blk359_dn12 } else { (-locals.var_dpbs__blk359_dn12) }, if locals.var_dpbs__blk359 >= 0.0 { locals.var_dpbs__blk359_dn17 } else { (-locals.var_dpbs__blk359_dn17) },)
    } else {
        (locals.var_t1__blk351, locals.var_t1__blk351_dn0, locals.var_t1__blk351_dn2, locals.var_t1__blk351_dn6, locals.var_t1__blk351_dn7, locals.var_t1__blk351_dn10, locals.var_t1__blk351_dn11, locals.var_t1__blk351_dn12, locals.var_t1__blk351_dn17,)
    }
};
            locals.var_t1__blk351 = assign12900_body148_e18286;
            locals.var_t1__blk351_dn0 = assign12900_body148_e18286_d_n0;
            locals.var_t1__blk351_dn2 = assign12900_body148_e18286_d_n2;
            locals.var_t1__blk351_dn6 = assign12900_body148_e18286_d_n6;
            locals.var_t1__blk351_dn7 = assign12900_body148_e18286_d_n7;
            locals.var_t1__blk351_dn10 = assign12900_body148_e18286_d_n10;
            locals.var_t1__blk351_dn11 = assign12900_body148_e18286_d_n11;
            locals.var_t1__blk351_dn12 = assign12900_body148_e18286_d_n12;
            locals.var_t1__blk351_dn17 = assign12900_body148_e18286_d_n17;
            let assign12900_body149_e18289: f64 = (locals.var_dpsb__blk360).abs();
            let assign12900_body149_e18290: f64 = if locals.var_t1__blk351 < assign12900_body149_e18289 { 1.0 } else { 0.0 };
            locals.var_guard406 = assign12900_body149_e18290;
            let (assign12900_body150_e18303, assign12900_body150_e18303_d_n0, assign12900_body150_e18303_d_n2, assign12900_body150_e18303_d_n6, assign12900_body150_e18303_d_n7, assign12900_body150_e18303_d_n10, assign12900_body150_e18303_d_n11, assign12900_body150_e18303_d_n12, assign12900_body150_e18303_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) && (locals.var_guard406 != 0.0)) {
        let assign12900_body150_e18301: f64 = (locals.var_dpsb__blk360).abs();
        (assign12900_body150_e18301, if locals.var_dpsb__blk360 >= 0.0 { locals.var_dpsb__blk360_dn0 } else { (-locals.var_dpsb__blk360_dn0) }, if locals.var_dpsb__blk360 >= 0.0 { locals.var_dpsb__blk360_dn2 } else { (-locals.var_dpsb__blk360_dn2) }, if locals.var_dpsb__blk360 >= 0.0 { locals.var_dpsb__blk360_dn6 } else { (-locals.var_dpsb__blk360_dn6) }, if locals.var_dpsb__blk360 >= 0.0 { locals.var_dpsb__blk360_dn7 } else { (-locals.var_dpsb__blk360_dn7) }, if locals.var_dpsb__blk360 >= 0.0 { locals.var_dpsb__blk360_dn10 } else { (-locals.var_dpsb__blk360_dn10) }, if locals.var_dpsb__blk360 >= 0.0 { locals.var_dpsb__blk360_dn11 } else { (-locals.var_dpsb__blk360_dn11) }, if locals.var_dpsb__blk360 >= 0.0 { locals.var_dpsb__blk360_dn12 } else { (-locals.var_dpsb__blk360_dn12) }, if locals.var_dpsb__blk360 >= 0.0 { locals.var_dpsb__blk360_dn17 } else { (-locals.var_dpsb__blk360_dn17) },)
    } else {
        (locals.var_t1__blk351, locals.var_t1__blk351_dn0, locals.var_t1__blk351_dn2, locals.var_t1__blk351_dn6, locals.var_t1__blk351_dn7, locals.var_t1__blk351_dn10, locals.var_t1__blk351_dn11, locals.var_t1__blk351_dn12, locals.var_t1__blk351_dn17,)
    }
};
            locals.var_t1__blk351 = assign12900_body150_e18303;
            locals.var_t1__blk351_dn0 = assign12900_body150_e18303_d_n0;
            locals.var_t1__blk351_dn2 = assign12900_body150_e18303_d_n2;
            locals.var_t1__blk351_dn6 = assign12900_body150_e18303_d_n6;
            locals.var_t1__blk351_dn7 = assign12900_body150_e18303_d_n7;
            locals.var_t1__blk351_dn10 = assign12900_body150_e18303_d_n10;
            locals.var_t1__blk351_dn11 = assign12900_body150_e18303_d_n11;
            locals.var_t1__blk351_dn12 = assign12900_body150_e18303_d_n12;
            locals.var_t1__blk351_dn17 = assign12900_body150_e18303_d_n17;
            let (assign12900_body151_e18313,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_scale_fac,)
    }
};
            locals.var_scale_fac = assign12900_body151_e18313;
            let assign12900_body152_e18316: f64 = if locals.var_lp_sl > 80.0 { 1.0 } else { 0.0 };
            locals.var_guard407 = assign12900_body152_e18316;
            let (assign12900_body153_e18328,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) && (locals.var_guard407 != 0.0)) {
        (125.0,)
    } else {
        (locals.var_scale_fac,)
    }
};
            locals.var_scale_fac = assign12900_body153_e18328;
            let assign12900_body154_e18331: f64 = if locals.var_lp_sl > 40.0 { 1.0 } else { 0.0 };
            locals.var_guard408 = assign12900_body154_e18331;
            let (assign12900_body155_e18346,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) && (locals.var_guard407 == 0.0)) && (locals.var_guard408 != 0.0)) {
        (125.0,)
    } else {
        (locals.var_scale_fac,)
    }
};
            locals.var_scale_fac = assign12900_body155_e18346;
            let assign12900_body156_e18349: f64 = if locals.var_lp_sl > 20.0 { 1.0 } else { 0.0 };
            locals.var_guard409 = assign12900_body156_e18349;
            let (assign12900_body157_e18367,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) && (locals.var_guard407 == 0.0)) && (locals.var_guard408 == 0.0)) && (locals.var_guard409 != 0.0)) {
        (25.0,)
    } else {
        (locals.var_scale_fac,)
    }
};
            locals.var_scale_fac = assign12900_body157_e18367;
            let assign12900_body158_e18370: f64 = if locals.var_lp_sl > 10.0 { 1.0 } else { 0.0 };
            locals.var_guard410 = assign12900_body158_e18370;
            let (assign12900_body159_e18391,) = {
    if (((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) && (locals.var_guard407 == 0.0)) && (locals.var_guard408 == 0.0)) && (locals.var_guard409 == 0.0)) && (locals.var_guard410 != 0.0)) {
        (5.0,)
    } else {
        (locals.var_scale_fac,)
    }
};
            locals.var_scale_fac = assign12900_body159_e18391;
            let assign12900_body160_e18395: f64 = (0.1 / locals.var_scale_fac);
            let assign12900_body160_e18396: f64 = if locals.var_t1__blk351 > assign12900_body160_e18395 { 1.0 } else { 0.0 };
            locals.var_guard411 = assign12900_body160_e18396;
            let (assign12900_body161_e18414, assign12900_body161_e18414_d_n0, assign12900_body161_e18414_d_n2, assign12900_body161_e18414_d_n6, assign12900_body161_e18414_d_n7, assign12900_body161_e18414_d_n10, assign12900_body161_e18414_d_n11, assign12900_body161_e18414_d_n12, assign12900_body161_e18414_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) && (locals.var_guard411 != 0.0)) {
        let assign12900_body161_e18409: f64 = (0.1 / locals.var_scale_fac);
        let assign12900_body161_e18411: f64 = (assign12900_body161_e18409 / locals.var_t1__blk351);
        let assign12900_body161_e18412: f64 = (locals.var_dpss__blk358 * assign12900_body161_e18411);
        (assign12900_body161_e18412, ((locals.var_dpss__blk358_dn0 * assign12900_body161_e18411) + (locals.var_dpss__blk358 * (-((assign12900_body161_e18409 * locals.var_t1__blk351_dn0) / (locals.var_t1__blk351 * locals.var_t1__blk351))))), ((locals.var_dpss__blk358_dn2 * assign12900_body161_e18411) + (locals.var_dpss__blk358 * (-((assign12900_body161_e18409 * locals.var_t1__blk351_dn2) / (locals.var_t1__blk351 * locals.var_t1__blk351))))), ((locals.var_dpss__blk358_dn6 * assign12900_body161_e18411) + (locals.var_dpss__blk358 * (-((assign12900_body161_e18409 * locals.var_t1__blk351_dn6) / (locals.var_t1__blk351 * locals.var_t1__blk351))))), ((locals.var_dpss__blk358_dn7 * assign12900_body161_e18411) + (locals.var_dpss__blk358 * (-((assign12900_body161_e18409 * locals.var_t1__blk351_dn7) / (locals.var_t1__blk351 * locals.var_t1__blk351))))), ((locals.var_dpss__blk358_dn10 * assign12900_body161_e18411) + (locals.var_dpss__blk358 * (-((assign12900_body161_e18409 * locals.var_t1__blk351_dn10) / (locals.var_t1__blk351 * locals.var_t1__blk351))))), ((locals.var_dpss__blk358_dn11 * assign12900_body161_e18411) + (locals.var_dpss__blk358 * (-((assign12900_body161_e18409 * locals.var_t1__blk351_dn11) / (locals.var_t1__blk351 * locals.var_t1__blk351))))), ((locals.var_dpss__blk358_dn12 * assign12900_body161_e18411) + (locals.var_dpss__blk358 * (-((assign12900_body161_e18409 * locals.var_t1__blk351_dn12) / (locals.var_t1__blk351 * locals.var_t1__blk351))))), ((locals.var_dpss__blk358_dn17 * assign12900_body161_e18411) + (locals.var_dpss__blk358 * (-((assign12900_body161_e18409 * locals.var_t1__blk351_dn17) / (locals.var_t1__blk351 * locals.var_t1__blk351))))),)
    } else {
        (locals.var_dpss__blk358, locals.var_dpss__blk358_dn0, locals.var_dpss__blk358_dn2, locals.var_dpss__blk358_dn6, locals.var_dpss__blk358_dn7, locals.var_dpss__blk358_dn10, locals.var_dpss__blk358_dn11, locals.var_dpss__blk358_dn12, locals.var_dpss__blk358_dn17,)
    }
};
            locals.var_dpss__blk358 = assign12900_body161_e18414;
            locals.var_dpss__blk358_dn0 = assign12900_body161_e18414_d_n0;
            locals.var_dpss__blk358_dn2 = assign12900_body161_e18414_d_n2;
            locals.var_dpss__blk358_dn6 = assign12900_body161_e18414_d_n6;
            locals.var_dpss__blk358_dn7 = assign12900_body161_e18414_d_n7;
            locals.var_dpss__blk358_dn10 = assign12900_body161_e18414_d_n10;
            locals.var_dpss__blk358_dn11 = assign12900_body161_e18414_d_n11;
            locals.var_dpss__blk358_dn12 = assign12900_body161_e18414_d_n12;
            locals.var_dpss__blk358_dn17 = assign12900_body161_e18414_d_n17;
            let (assign12900_body162_e18432, assign12900_body162_e18432_d_n0, assign12900_body162_e18432_d_n2, assign12900_body162_e18432_d_n6, assign12900_body162_e18432_d_n7, assign12900_body162_e18432_d_n10, assign12900_body162_e18432_d_n11, assign12900_body162_e18432_d_n12, assign12900_body162_e18432_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) && (locals.var_guard411 != 0.0)) {
        let assign12900_body162_e18427: f64 = (0.1 / locals.var_scale_fac);
        let assign12900_body162_e18429: f64 = (assign12900_body162_e18427 / locals.var_t1__blk351);
        let assign12900_body162_e18430: f64 = (locals.var_dpbs__blk359 * assign12900_body162_e18429);
        (assign12900_body162_e18430, ((locals.var_dpbs__blk359_dn0 * assign12900_body162_e18429) + (locals.var_dpbs__blk359 * (-((assign12900_body162_e18427 * locals.var_t1__blk351_dn0) / (locals.var_t1__blk351 * locals.var_t1__blk351))))), ((locals.var_dpbs__blk359_dn2 * assign12900_body162_e18429) + (locals.var_dpbs__blk359 * (-((assign12900_body162_e18427 * locals.var_t1__blk351_dn2) / (locals.var_t1__blk351 * locals.var_t1__blk351))))), ((locals.var_dpbs__blk359_dn6 * assign12900_body162_e18429) + (locals.var_dpbs__blk359 * (-((assign12900_body162_e18427 * locals.var_t1__blk351_dn6) / (locals.var_t1__blk351 * locals.var_t1__blk351))))), ((locals.var_dpbs__blk359_dn7 * assign12900_body162_e18429) + (locals.var_dpbs__blk359 * (-((assign12900_body162_e18427 * locals.var_t1__blk351_dn7) / (locals.var_t1__blk351 * locals.var_t1__blk351))))), ((locals.var_dpbs__blk359_dn10 * assign12900_body162_e18429) + (locals.var_dpbs__blk359 * (-((assign12900_body162_e18427 * locals.var_t1__blk351_dn10) / (locals.var_t1__blk351 * locals.var_t1__blk351))))), ((locals.var_dpbs__blk359_dn11 * assign12900_body162_e18429) + (locals.var_dpbs__blk359 * (-((assign12900_body162_e18427 * locals.var_t1__blk351_dn11) / (locals.var_t1__blk351 * locals.var_t1__blk351))))), ((locals.var_dpbs__blk359_dn12 * assign12900_body162_e18429) + (locals.var_dpbs__blk359 * (-((assign12900_body162_e18427 * locals.var_t1__blk351_dn12) / (locals.var_t1__blk351 * locals.var_t1__blk351))))), ((locals.var_dpbs__blk359_dn17 * assign12900_body162_e18429) + (locals.var_dpbs__blk359 * (-((assign12900_body162_e18427 * locals.var_t1__blk351_dn17) / (locals.var_t1__blk351 * locals.var_t1__blk351))))),)
    } else {
        (locals.var_dpbs__blk359, locals.var_dpbs__blk359_dn0, locals.var_dpbs__blk359_dn2, locals.var_dpbs__blk359_dn6, locals.var_dpbs__blk359_dn7, locals.var_dpbs__blk359_dn10, locals.var_dpbs__blk359_dn11, locals.var_dpbs__blk359_dn12, locals.var_dpbs__blk359_dn17,)
    }
};
            locals.var_dpbs__blk359 = assign12900_body162_e18432;
            locals.var_dpbs__blk359_dn0 = assign12900_body162_e18432_d_n0;
            locals.var_dpbs__blk359_dn2 = assign12900_body162_e18432_d_n2;
            locals.var_dpbs__blk359_dn6 = assign12900_body162_e18432_d_n6;
            locals.var_dpbs__blk359_dn7 = assign12900_body162_e18432_d_n7;
            locals.var_dpbs__blk359_dn10 = assign12900_body162_e18432_d_n10;
            locals.var_dpbs__blk359_dn11 = assign12900_body162_e18432_d_n11;
            locals.var_dpbs__blk359_dn12 = assign12900_body162_e18432_d_n12;
            locals.var_dpbs__blk359_dn17 = assign12900_body162_e18432_d_n17;
            let (assign12900_body163_e18450, assign12900_body163_e18450_d_n0, assign12900_body163_e18450_d_n2, assign12900_body163_e18450_d_n6, assign12900_body163_e18450_d_n7, assign12900_body163_e18450_d_n10, assign12900_body163_e18450_d_n11, assign12900_body163_e18450_d_n12, assign12900_body163_e18450_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) && (locals.var_guard411 != 0.0)) {
        let assign12900_body163_e18445: f64 = (0.1 / locals.var_scale_fac);
        let assign12900_body163_e18447: f64 = (assign12900_body163_e18445 / locals.var_t1__blk351);
        let assign12900_body163_e18448: f64 = (locals.var_dpsb__blk360 * assign12900_body163_e18447);
        (assign12900_body163_e18448, ((locals.var_dpsb__blk360_dn0 * assign12900_body163_e18447) + (locals.var_dpsb__blk360 * (-((assign12900_body163_e18445 * locals.var_t1__blk351_dn0) / (locals.var_t1__blk351 * locals.var_t1__blk351))))), ((locals.var_dpsb__blk360_dn2 * assign12900_body163_e18447) + (locals.var_dpsb__blk360 * (-((assign12900_body163_e18445 * locals.var_t1__blk351_dn2) / (locals.var_t1__blk351 * locals.var_t1__blk351))))), ((locals.var_dpsb__blk360_dn6 * assign12900_body163_e18447) + (locals.var_dpsb__blk360 * (-((assign12900_body163_e18445 * locals.var_t1__blk351_dn6) / (locals.var_t1__blk351 * locals.var_t1__blk351))))), ((locals.var_dpsb__blk360_dn7 * assign12900_body163_e18447) + (locals.var_dpsb__blk360 * (-((assign12900_body163_e18445 * locals.var_t1__blk351_dn7) / (locals.var_t1__blk351 * locals.var_t1__blk351))))), ((locals.var_dpsb__blk360_dn10 * assign12900_body163_e18447) + (locals.var_dpsb__blk360 * (-((assign12900_body163_e18445 * locals.var_t1__blk351_dn10) / (locals.var_t1__blk351 * locals.var_t1__blk351))))), ((locals.var_dpsb__blk360_dn11 * assign12900_body163_e18447) + (locals.var_dpsb__blk360 * (-((assign12900_body163_e18445 * locals.var_t1__blk351_dn11) / (locals.var_t1__blk351 * locals.var_t1__blk351))))), ((locals.var_dpsb__blk360_dn12 * assign12900_body163_e18447) + (locals.var_dpsb__blk360 * (-((assign12900_body163_e18445 * locals.var_t1__blk351_dn12) / (locals.var_t1__blk351 * locals.var_t1__blk351))))), ((locals.var_dpsb__blk360_dn17 * assign12900_body163_e18447) + (locals.var_dpsb__blk360 * (-((assign12900_body163_e18445 * locals.var_t1__blk351_dn17) / (locals.var_t1__blk351 * locals.var_t1__blk351))))),)
    } else {
        (locals.var_dpsb__blk360, locals.var_dpsb__blk360_dn0, locals.var_dpsb__blk360_dn2, locals.var_dpsb__blk360_dn6, locals.var_dpsb__blk360_dn7, locals.var_dpsb__blk360_dn10, locals.var_dpsb__blk360_dn11, locals.var_dpsb__blk360_dn12, locals.var_dpsb__blk360_dn17,)
    }
};
            locals.var_dpsb__blk360 = assign12900_body163_e18450;
            locals.var_dpsb__blk360_dn0 = assign12900_body163_e18450_d_n0;
            locals.var_dpsb__blk360_dn2 = assign12900_body163_e18450_d_n2;
            locals.var_dpsb__blk360_dn6 = assign12900_body163_e18450_d_n6;
            locals.var_dpsb__blk360_dn7 = assign12900_body163_e18450_d_n7;
            locals.var_dpsb__blk360_dn10 = assign12900_body163_e18450_d_n10;
            locals.var_dpsb__blk360_dn11 = assign12900_body163_e18450_d_n11;
            locals.var_dpsb__blk360_dn12 = assign12900_body163_e18450_d_n12;
            locals.var_dpsb__blk360_dn17 = assign12900_body163_e18450_d_n17;
            let (assign12900_body164_e18462, assign12900_body164_e18462_d_n0, assign12900_body164_e18462_d_n2, assign12900_body164_e18462_d_n6, assign12900_body164_e18462_d_n7, assign12900_body164_e18462_d_n10, assign12900_body164_e18462_d_n11, assign12900_body164_e18462_d_n12, assign12900_body164_e18462_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12900_body164_e18460: f64 = (locals.var_phi_sl_soi + locals.var_dpss__blk358);
        (assign12900_body164_e18460, (locals.var_phi_sl_soi_dn0 + locals.var_dpss__blk358_dn0), (locals.var_phi_sl_soi_dn2 + locals.var_dpss__blk358_dn2), (locals.var_phi_sl_soi_dn6 + locals.var_dpss__blk358_dn6), (locals.var_phi_sl_soi_dn7 + locals.var_dpss__blk358_dn7), (locals.var_phi_sl_soi_dn10 + locals.var_dpss__blk358_dn10), (locals.var_phi_sl_soi_dn11 + locals.var_dpss__blk358_dn11), (locals.var_phi_sl_soi_dn12 + locals.var_dpss__blk358_dn12), (locals.var_phi_sl_soi_dn17 + locals.var_dpss__blk358_dn17),)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
            locals.var_phi_sl_soi = assign12900_body164_e18462;
            locals.var_phi_sl_soi_dn0 = assign12900_body164_e18462_d_n0;
            locals.var_phi_sl_soi_dn2 = assign12900_body164_e18462_d_n2;
            locals.var_phi_sl_soi_dn6 = assign12900_body164_e18462_d_n6;
            locals.var_phi_sl_soi_dn7 = assign12900_body164_e18462_d_n7;
            locals.var_phi_sl_soi_dn10 = assign12900_body164_e18462_d_n10;
            locals.var_phi_sl_soi_dn11 = assign12900_body164_e18462_d_n11;
            locals.var_phi_sl_soi_dn12 = assign12900_body164_e18462_d_n12;
            locals.var_phi_sl_soi_dn17 = assign12900_body164_e18462_d_n17;
            let (assign12900_body165_e18474, assign12900_body165_e18474_d_n0, assign12900_body165_e18474_d_n2, assign12900_body165_e18474_d_n6, assign12900_body165_e18474_d_n7, assign12900_body165_e18474_d_n10, assign12900_body165_e18474_d_n11, assign12900_body165_e18474_d_n12, assign12900_body165_e18474_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12900_body165_e18472: f64 = (locals.var_phi_bl_soi + locals.var_dpbs__blk359);
        (assign12900_body165_e18472, (locals.var_phi_bl_soi_dn0 + locals.var_dpbs__blk359_dn0), (locals.var_phi_bl_soi_dn2 + locals.var_dpbs__blk359_dn2), (locals.var_phi_bl_soi_dn6 + locals.var_dpbs__blk359_dn6), (locals.var_phi_bl_soi_dn7 + locals.var_dpbs__blk359_dn7), (locals.var_phi_bl_soi_dn10 + locals.var_dpbs__blk359_dn10), (locals.var_phi_bl_soi_dn11 + locals.var_dpbs__blk359_dn11), (locals.var_phi_bl_soi_dn12 + locals.var_dpbs__blk359_dn12), (locals.var_phi_bl_soi_dn17 + locals.var_dpbs__blk359_dn17),)
    } else {
        (locals.var_phi_bl_soi, locals.var_phi_bl_soi_dn0, locals.var_phi_bl_soi_dn2, locals.var_phi_bl_soi_dn6, locals.var_phi_bl_soi_dn7, locals.var_phi_bl_soi_dn10, locals.var_phi_bl_soi_dn11, locals.var_phi_bl_soi_dn12, locals.var_phi_bl_soi_dn17,)
    }
};
            locals.var_phi_bl_soi = assign12900_body165_e18474;
            locals.var_phi_bl_soi_dn0 = assign12900_body165_e18474_d_n0;
            locals.var_phi_bl_soi_dn2 = assign12900_body165_e18474_d_n2;
            locals.var_phi_bl_soi_dn6 = assign12900_body165_e18474_d_n6;
            locals.var_phi_bl_soi_dn7 = assign12900_body165_e18474_d_n7;
            locals.var_phi_bl_soi_dn10 = assign12900_body165_e18474_d_n10;
            locals.var_phi_bl_soi_dn11 = assign12900_body165_e18474_d_n11;
            locals.var_phi_bl_soi_dn12 = assign12900_body165_e18474_d_n12;
            locals.var_phi_bl_soi_dn17 = assign12900_body165_e18474_d_n17;
            let (assign12900_body166_e18486, assign12900_body166_e18486_d_n0, assign12900_body166_e18486_d_n2, assign12900_body166_e18486_d_n6, assign12900_body166_e18486_d_n7, assign12900_body166_e18486_d_n10, assign12900_body166_e18486_d_n11, assign12900_body166_e18486_d_n12, assign12900_body166_e18486_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12900_body166_e18484: f64 = (locals.var_phi_sl_bulk + locals.var_dpsb__blk360);
        (assign12900_body166_e18484, (locals.var_phi_sl_bulk_dn0 + locals.var_dpsb__blk360_dn0), (locals.var_phi_sl_bulk_dn2 + locals.var_dpsb__blk360_dn2), (locals.var_phi_sl_bulk_dn6 + locals.var_dpsb__blk360_dn6), (locals.var_phi_sl_bulk_dn7 + locals.var_dpsb__blk360_dn7), (locals.var_phi_sl_bulk_dn10 + locals.var_dpsb__blk360_dn10), (locals.var_phi_sl_bulk_dn11 + locals.var_dpsb__blk360_dn11), (locals.var_phi_sl_bulk_dn12 + locals.var_dpsb__blk360_dn12), (locals.var_phi_sl_bulk_dn17 + locals.var_dpsb__blk360_dn17),)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
            locals.var_phi_sl_bulk = assign12900_body166_e18486;
            locals.var_phi_sl_bulk_dn0 = assign12900_body166_e18486_d_n0;
            locals.var_phi_sl_bulk_dn2 = assign12900_body166_e18486_d_n2;
            locals.var_phi_sl_bulk_dn6 = assign12900_body166_e18486_d_n6;
            locals.var_phi_sl_bulk_dn7 = assign12900_body166_e18486_d_n7;
            locals.var_phi_sl_bulk_dn10 = assign12900_body166_e18486_d_n10;
            locals.var_phi_sl_bulk_dn11 = assign12900_body166_e18486_d_n11;
            locals.var_phi_sl_bulk_dn12 = assign12900_body166_e18486_d_n12;
            locals.var_phi_sl_bulk_dn17 = assign12900_body166_e18486_d_n17;
            let (assign12900_body167_e18500,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) {
        let assign12900_body167_e18496: f64 = (5e-12 * locals.var_scale_fac);
        let assign12900_body167_e18498: f64 = assign12900_body167_e18496;
        (assign12900_body167_e18498,)
    } else {
        (locals.var_psconv_3d,)
    }
};
            locals.var_psconv_3d = assign12900_body167_e18500;
            let assign12900_body168_e18503: f64 = if locals.var_t1__blk351 < locals.var_psconv_3d { 1.0 } else { 0.0 };
            locals.var_guard412 = assign12900_body168_e18503;
            let (assign12900_body169_e18515,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard404 == 0.0)) && (locals.var_guard412 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign12900_body169_e18515;
            let (assign12900_body170_e18524,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign12900_body170_e18522: f64 = (locals.var_lp_sl + 1.0);
        (assign12900_body170_e18522,)
    } else {
        (locals.var_lp_sl,)
    }
};
            locals.var_lp_sl = assign12900_body170_e18524;
        }

    }

    pub(super) fn stamp_transient_block_42(
        locals: &mut StampLocals,
    ) {
        let (assign12910_e18536,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let (assign12910_e18534,) = {
            if (locals.var_flg_brk8 > 0.0) {
                (locals.var_flg_brk8,)
            } else {
                (locals.var_lp_sl,)
            }
        };
        (assign12910_e18534,)
    } else {
        (locals.var_lp_sl,)
    }
};
        locals.var_lp_sl = assign12910_e18536;

        let assign12920_e18539: f64 = if locals.var_flg_conv == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard413 = assign12920_e18539;

        let (assign12930_e18548, assign12930_e18548_d_n0, assign12930_e18548_d_n2, assign12930_e18548_d_n6, assign12930_e18548_d_n7, assign12930_e18548_d_n10, assign12930_e18548_d_n11, assign12930_e18548_d_n12, assign12930_e18548_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard413 != 0.0)) {
        (locals.var_phi_sl_soi_ini, locals.var_phi_sl_soi_ini_dn0, locals.var_phi_sl_soi_ini_dn2, locals.var_phi_sl_soi_ini_dn6, locals.var_phi_sl_soi_ini_dn7, locals.var_phi_sl_soi_ini_dn10, locals.var_phi_sl_soi_ini_dn11, locals.var_phi_sl_soi_ini_dn12, locals.var_phi_sl_soi_ini_dn17,)
    } else {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    }
};
        locals.var_phi_sl_soi = assign12930_e18548;
        locals.var_phi_sl_soi_dn0 = assign12930_e18548_d_n0;
        locals.var_phi_sl_soi_dn2 = assign12930_e18548_d_n2;
        locals.var_phi_sl_soi_dn6 = assign12930_e18548_d_n6;
        locals.var_phi_sl_soi_dn7 = assign12930_e18548_d_n7;
        locals.var_phi_sl_soi_dn10 = assign12930_e18548_d_n10;
        locals.var_phi_sl_soi_dn11 = assign12930_e18548_d_n11;
        locals.var_phi_sl_soi_dn12 = assign12930_e18548_d_n12;
        locals.var_phi_sl_soi_dn17 = assign12930_e18548_d_n17;

        let (assign12940_e18557, assign12940_e18557_d_n0, assign12940_e18557_d_n2, assign12940_e18557_d_n6, assign12940_e18557_d_n7, assign12940_e18557_d_n10, assign12940_e18557_d_n11, assign12940_e18557_d_n12, assign12940_e18557_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard413 != 0.0)) {
        (locals.var_phi_bl_soi_ini, locals.var_phi_bl_soi_ini_dn0, locals.var_phi_bl_soi_ini_dn2, locals.var_phi_bl_soi_ini_dn6, locals.var_phi_bl_soi_ini_dn7, locals.var_phi_bl_soi_ini_dn10, locals.var_phi_bl_soi_ini_dn11, locals.var_phi_bl_soi_ini_dn12, locals.var_phi_bl_soi_ini_dn17,)
    } else {
        (locals.var_phi_bl_soi, locals.var_phi_bl_soi_dn0, locals.var_phi_bl_soi_dn2, locals.var_phi_bl_soi_dn6, locals.var_phi_bl_soi_dn7, locals.var_phi_bl_soi_dn10, locals.var_phi_bl_soi_dn11, locals.var_phi_bl_soi_dn12, locals.var_phi_bl_soi_dn17,)
    }
};
        locals.var_phi_bl_soi = assign12940_e18557;
        locals.var_phi_bl_soi_dn0 = assign12940_e18557_d_n0;
        locals.var_phi_bl_soi_dn2 = assign12940_e18557_d_n2;
        locals.var_phi_bl_soi_dn6 = assign12940_e18557_d_n6;
        locals.var_phi_bl_soi_dn7 = assign12940_e18557_d_n7;
        locals.var_phi_bl_soi_dn10 = assign12940_e18557_d_n10;
        locals.var_phi_bl_soi_dn11 = assign12940_e18557_d_n11;
        locals.var_phi_bl_soi_dn12 = assign12940_e18557_d_n12;
        locals.var_phi_bl_soi_dn17 = assign12940_e18557_d_n17;

        let (assign12950_e18566, assign12950_e18566_d_n0, assign12950_e18566_d_n2, assign12950_e18566_d_n6, assign12950_e18566_d_n7, assign12950_e18566_d_n10, assign12950_e18566_d_n11, assign12950_e18566_d_n12, assign12950_e18566_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard413 != 0.0)) {
        (locals.var_phi_sl_bulk_ini, locals.var_phi_sl_bulk_ini_dn0, locals.var_phi_sl_bulk_ini_dn2, locals.var_phi_sl_bulk_ini_dn6, locals.var_phi_sl_bulk_ini_dn7, locals.var_phi_sl_bulk_ini_dn10, locals.var_phi_sl_bulk_ini_dn11, locals.var_phi_sl_bulk_ini_dn12, locals.var_phi_sl_bulk_ini_dn17,)
    } else {
        (locals.var_phi_sl_bulk, locals.var_phi_sl_bulk_dn0, locals.var_phi_sl_bulk_dn2, locals.var_phi_sl_bulk_dn6, locals.var_phi_sl_bulk_dn7, locals.var_phi_sl_bulk_dn10, locals.var_phi_sl_bulk_dn11, locals.var_phi_sl_bulk_dn12, locals.var_phi_sl_bulk_dn17,)
    }
};
        locals.var_phi_sl_bulk = assign12950_e18566;
        locals.var_phi_sl_bulk_dn0 = assign12950_e18566_d_n0;
        locals.var_phi_sl_bulk_dn2 = assign12950_e18566_d_n2;
        locals.var_phi_sl_bulk_dn6 = assign12950_e18566_d_n6;
        locals.var_phi_sl_bulk_dn7 = assign12950_e18566_d_n7;
        locals.var_phi_sl_bulk_dn10 = assign12950_e18566_d_n10;
        locals.var_phi_sl_bulk_dn11 = assign12950_e18566_d_n11;
        locals.var_phi_sl_bulk_dn12 = assign12950_e18566_d_n12;
        locals.var_phi_sl_bulk_dn17 = assign12950_e18566_d_n17;

        let (assign12960_e18573, assign12960_e18573_d_n0, assign12960_e18573_d_n2, assign12960_e18573_d_n6, assign12960_e18573_d_n7, assign12960_e18573_d_n10, assign12960_e18573_d_n11, assign12960_e18573_d_n12, assign12960_e18573_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn12, locals.var_psl_dn17,)
    }
};
        locals.var_psl = assign12960_e18573;
        locals.var_psl_dn0 = assign12960_e18573_d_n0;
        locals.var_psl_dn2 = assign12960_e18573_d_n2;
        locals.var_psl_dn6 = assign12960_e18573_d_n6;
        locals.var_psl_dn7 = assign12960_e18573_d_n7;
        locals.var_psl_dn10 = assign12960_e18573_d_n10;
        locals.var_psl_dn11 = assign12960_e18573_d_n11;
        locals.var_psl_dn12 = assign12960_e18573_d_n12;
        locals.var_psl_dn17 = assign12960_e18573_d_n17;

        let (assign12980_e18587, assign12980_e18587_d_n0, assign12980_e18587_d_n2, assign12980_e18587_d_n6, assign12980_e18587_d_n7, assign12980_e18587_d_n10, assign12980_e18587_d_n11, assign12980_e18587_d_n12, assign12980_e18587_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn12, locals.var_vdsorg_dn17,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    }
};
        locals.var_vds = assign12980_e18587;
        locals.var_vds_dn0 = assign12980_e18587_d_n0;
        locals.var_vds_dn2 = assign12980_e18587_d_n2;
        locals.var_vds_dn6 = assign12980_e18587_d_n6;
        locals.var_vds_dn7 = assign12980_e18587_d_n7;
        locals.var_vds_dn10 = assign12980_e18587_d_n10;
        locals.var_vds_dn11 = assign12980_e18587_d_n11;
        locals.var_vds_dn12 = assign12980_e18587_d_n12;
        locals.var_vds_dn17 = assign12980_e18587_d_n17;

        let assign12990_e18590: f64 = if locals.var_phi_s0_soi < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard414 = assign12990_e18590;

        let (assign13000_e18599,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard414 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_noqi,)
    }
};
        locals.var_flg_noqi = assign13000_e18599;

        let (assign13010_e18606, assign13010_e18606_d_n0, assign13010_e18606_d_n2, assign13010_e18606_d_n6, assign13010_e18606_d_n7, assign13010_e18606_d_n10, assign13010_e18606_d_n11, assign13010_e18606_d_n12, assign13010_e18606_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    } else {
        (locals.var_ps0s, locals.var_ps0s_dn0, locals.var_ps0s_dn2, locals.var_ps0s_dn6, locals.var_ps0s_dn7, locals.var_ps0s_dn10, locals.var_ps0s_dn11, locals.var_ps0s_dn12, locals.var_ps0s_dn17,)
    }
};
        locals.var_ps0s = assign13010_e18606;
        locals.var_ps0s_dn0 = assign13010_e18606_d_n0;
        locals.var_ps0s_dn2 = assign13010_e18606_d_n2;
        locals.var_ps0s_dn6 = assign13010_e18606_d_n6;
        locals.var_ps0s_dn7 = assign13010_e18606_d_n7;
        locals.var_ps0s_dn10 = assign13010_e18606_d_n10;
        locals.var_ps0s_dn11 = assign13010_e18606_d_n11;
        locals.var_ps0s_dn12 = assign13010_e18606_d_n12;
        locals.var_ps0s_dn17 = assign13010_e18606_d_n17;

        let (assign13020_e18613, assign13020_e18613_d_n0, assign13020_e18613_d_n2, assign13020_e18613_d_n6, assign13020_e18613_d_n7, assign13020_e18613_d_n10, assign13020_e18613_d_n11, assign13020_e18613_d_n12, assign13020_e18613_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        (locals.var_phi_sl_soi, locals.var_phi_sl_soi_dn0, locals.var_phi_sl_soi_dn2, locals.var_phi_sl_soi_dn6, locals.var_phi_sl_soi_dn7, locals.var_phi_sl_soi_dn10, locals.var_phi_sl_soi_dn11, locals.var_phi_sl_soi_dn12, locals.var_phi_sl_soi_dn17,)
    } else {
        (locals.var_psls, locals.var_psls_dn0, locals.var_psls_dn2, locals.var_psls_dn6, locals.var_psls_dn7, locals.var_psls_dn10, locals.var_psls_dn11, locals.var_psls_dn12, locals.var_psls_dn17,)
    }
};
        locals.var_psls = assign13020_e18613;
        locals.var_psls_dn0 = assign13020_e18613_d_n0;
        locals.var_psls_dn2 = assign13020_e18613_d_n2;
        locals.var_psls_dn6 = assign13020_e18613_d_n6;
        locals.var_psls_dn7 = assign13020_e18613_d_n7;
        locals.var_psls_dn10 = assign13020_e18613_d_n10;
        locals.var_psls_dn11 = assign13020_e18613_d_n11;
        locals.var_psls_dn12 = assign13020_e18613_d_n12;
        locals.var_psls_dn17 = assign13020_e18613_d_n17;

        let (assign13030_e18622, assign13030_e18622_d_n0, assign13030_e18622_d_n2, assign13030_e18622_d_n6, assign13030_e18622_d_n7, assign13030_e18622_d_n10, assign13030_e18622_d_n11, assign13030_e18622_d_n12, assign13030_e18622_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign13030_e18620: f64 = (locals.var_psls - locals.var_ps0s);
        (assign13030_e18620, (locals.var_psls_dn0 - locals.var_ps0s_dn0), (locals.var_psls_dn2 - locals.var_ps0s_dn2), (locals.var_psls_dn6 - locals.var_ps0s_dn6), (locals.var_psls_dn7 - locals.var_ps0s_dn7), (locals.var_psls_dn10 - locals.var_ps0s_dn10), (locals.var_psls_dn11 - locals.var_ps0s_dn11), (locals.var_psls_dn12 - locals.var_ps0s_dn12), (locals.var_psls_dn17 - locals.var_ps0s_dn17),)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn12, locals.var_pds_dn17,)
    }
};
        locals.var_pds = assign13030_e18622;
        locals.var_pds_dn0 = assign13030_e18622_d_n0;
        locals.var_pds_dn2 = assign13030_e18622_d_n2;
        locals.var_pds_dn6 = assign13030_e18622_d_n6;
        locals.var_pds_dn7 = assign13030_e18622_d_n7;
        locals.var_pds_dn10 = assign13030_e18622_d_n10;
        locals.var_pds_dn11 = assign13030_e18622_d_n11;
        locals.var_pds_dn12 = assign13030_e18622_d_n12;
        locals.var_pds_dn17 = assign13030_e18622_d_n17;

        let (assign13040_e18629, assign13040_e18629_d_n0, assign13040_e18629_d_n2, assign13040_e18629_d_n6, assign13040_e18629_d_n7, assign13040_e18629_d_n10, assign13040_e18629_d_n11, assign13040_e18629_d_n12, assign13040_e18629_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        (locals.var_phi_s0_bulk, locals.var_phi_s0_bulk_dn0, locals.var_phi_s0_bulk_dn2, locals.var_phi_s0_bulk_dn6, locals.var_phi_s0_bulk_dn7, locals.var_phi_s0_bulk_dn10, locals.var_phi_s0_bulk_dn11, locals.var_phi_s0_bulk_dn12, locals.var_phi_s0_bulk_dn17,)
    } else {
        (locals.var_ps0b, locals.var_ps0b_dn0, locals.var_ps0b_dn2, locals.var_ps0b_dn6, locals.var_ps0b_dn7, locals.var_ps0b_dn10, locals.var_ps0b_dn11, locals.var_ps0b_dn12, locals.var_ps0b_dn17,)
    }
};
        locals.var_ps0b = assign13040_e18629;
        locals.var_ps0b_dn0 = assign13040_e18629_d_n0;
        locals.var_ps0b_dn2 = assign13040_e18629_d_n2;
        locals.var_ps0b_dn6 = assign13040_e18629_d_n6;
        locals.var_ps0b_dn7 = assign13040_e18629_d_n7;
        locals.var_ps0b_dn10 = assign13040_e18629_d_n10;
        locals.var_ps0b_dn11 = assign13040_e18629_d_n11;
        locals.var_ps0b_dn12 = assign13040_e18629_d_n12;
        locals.var_ps0b_dn17 = assign13040_e18629_d_n17;

        let (assign13050_e18638, assign13050_e18638_d_n0, assign13050_e18638_d_n2, assign13050_e18638_d_n6, assign13050_e18638_d_n7, assign13050_e18638_d_n10, assign13050_e18638_d_n11, assign13050_e18638_d_n12, assign13050_e18638_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign13050_e18636: f64 = (locals.var_wdsoi / 1.034943e-10);
        (assign13050_e18636, (locals.var_wdsoi_dn0 / 1.034943e-10), (locals.var_wdsoi_dn2 / 1.034943e-10), (locals.var_wdsoi_dn6 / 1.034943e-10), (locals.var_wdsoi_dn7 / 1.034943e-10), (locals.var_wdsoi_dn10 / 1.034943e-10), (locals.var_wdsoi_dn11 / 1.034943e-10), (locals.var_wdsoi_dn12 / 1.034943e-10), (locals.var_wdsoi_dn17 / 1.034943e-10),)
    } else {
        (locals.var_c_s_inv, locals.var_c_s_inv_dn0, locals.var_c_s_inv_dn2, locals.var_c_s_inv_dn6, locals.var_c_s_inv_dn7, locals.var_c_s_inv_dn10, locals.var_c_s_inv_dn11, locals.var_c_s_inv_dn12, locals.var_c_s_inv_dn17,)
    }
};
        locals.var_c_s_inv = assign13050_e18638;
        locals.var_c_s_inv_dn0 = assign13050_e18638_d_n0;
        locals.var_c_s_inv_dn2 = assign13050_e18638_d_n2;
        locals.var_c_s_inv_dn6 = assign13050_e18638_d_n6;
        locals.var_c_s_inv_dn7 = assign13050_e18638_d_n7;
        locals.var_c_s_inv_dn10 = assign13050_e18638_d_n10;
        locals.var_c_s_inv_dn11 = assign13050_e18638_d_n11;
        locals.var_c_s_inv_dn12 = assign13050_e18638_d_n12;
        locals.var_c_s_inv_dn17 = assign13050_e18638_d_n17;

        let (assign13060_e18659, assign13060_e18659_d_n0, assign13060_e18659_d_n2, assign13060_e18659_d_n6, assign13060_e18659_d_n7, assign13060_e18659_d_n10, assign13060_e18659_d_n11, assign13060_e18659_d_n12, assign13060_e18659_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign13060_e18645: f64 = (locals.var_q_nl - locals.var_q_n0);
        let assign13060_e18649: f64 = (locals.var_q_nl + locals.var_q_n0);
        let assign13060_e18650: f64 = (locals.var_beta * assign13060_e18649);
        let assign13060_e18653: f64 = (locals.var_psls - locals.var_ps0s);
        let assign13060_e18654: f64 = (assign13060_e18650 * assign13060_e18653);
        let assign13060_e18656: f64 = (assign13060_e18654 * 0.5);
        let assign13060_e18657: f64 = (assign13060_e18645 - assign13060_e18656);
        (assign13060_e18657, ((locals.var_q_nl_dn0 - locals.var_q_n0_dn0) - ((((locals.var_beta * (locals.var_q_nl_dn0 + locals.var_q_n0_dn0)) * assign13060_e18653) + (assign13060_e18650 * (locals.var_psls_dn0 - locals.var_ps0s_dn0))) * 0.5)), ((locals.var_q_nl_dn2 - locals.var_q_n0_dn2) - ((((locals.var_beta * (locals.var_q_nl_dn2 + locals.var_q_n0_dn2)) * assign13060_e18653) + (assign13060_e18650 * (locals.var_psls_dn2 - locals.var_ps0s_dn2))) * 0.5)), ((locals.var_q_nl_dn6 - locals.var_q_n0_dn6) - ((((locals.var_beta * (locals.var_q_nl_dn6 + locals.var_q_n0_dn6)) * assign13060_e18653) + (assign13060_e18650 * (locals.var_psls_dn6 - locals.var_ps0s_dn6))) * 0.5)), ((locals.var_q_nl_dn7 - locals.var_q_n0_dn7) - ((((locals.var_beta * (locals.var_q_nl_dn7 + locals.var_q_n0_dn7)) * assign13060_e18653) + (assign13060_e18650 * (locals.var_psls_dn7 - locals.var_ps0s_dn7))) * 0.5)), ((locals.var_q_nl_dn10 - locals.var_q_n0_dn10) - (((((locals.var_beta_dn10 * assign13060_e18649) + (locals.var_beta * (locals.var_q_nl_dn10 + locals.var_q_n0_dn10))) * assign13060_e18653) + (assign13060_e18650 * (locals.var_psls_dn10 - locals.var_ps0s_dn10))) * 0.5)), ((locals.var_q_nl_dn11 - locals.var_q_n0_dn11) - ((((locals.var_beta * (locals.var_q_nl_dn11 + locals.var_q_n0_dn11)) * assign13060_e18653) + (assign13060_e18650 * (locals.var_psls_dn11 - locals.var_ps0s_dn11))) * 0.5)), ((locals.var_q_nl_dn12 - locals.var_q_n0_dn12) - ((((locals.var_beta * (locals.var_q_nl_dn12 + locals.var_q_n0_dn12)) * assign13060_e18653) + (assign13060_e18650 * (locals.var_psls_dn12 - locals.var_ps0s_dn12))) * 0.5)), ((locals.var_q_nl_dn17 - locals.var_q_n0_dn17) - ((((locals.var_beta * (locals.var_q_nl_dn17 + locals.var_q_n0_dn17)) * assign13060_e18653) + (assign13060_e18650 * (locals.var_psls_dn17 - locals.var_ps0s_dn17))) * 0.5)),)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn10, locals.var_idd_dn11, locals.var_idd_dn12, locals.var_idd_dn17,)
    }
};
        locals.var_idd = assign13060_e18659;
        locals.var_idd_dn0 = assign13060_e18659_d_n0;
        locals.var_idd_dn2 = assign13060_e18659_d_n2;
        locals.var_idd_dn6 = assign13060_e18659_d_n6;
        locals.var_idd_dn7 = assign13060_e18659_d_n7;
        locals.var_idd_dn10 = assign13060_e18659_d_n10;
        locals.var_idd_dn11 = assign13060_e18659_d_n11;
        locals.var_idd_dn12 = assign13060_e18659_d_n12;
        locals.var_idd_dn17 = assign13060_e18659_d_n17;

        let assign13070_e18666: f64 = if ((locals.var_idd < 0.0) || (locals.var_vds == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard415 = assign13070_e18666;

        let (assign13080_e18675, assign13080_e18675_d_n0, assign13080_e18675_d_n2, assign13080_e18675_d_n6, assign13080_e18675_d_n7, assign13080_e18675_d_n10, assign13080_e18675_d_n11, assign13080_e18675_d_n12, assign13080_e18675_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard415 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn10, locals.var_idd_dn11, locals.var_idd_dn12, locals.var_idd_dn17,)
    }
};
        locals.var_idd = assign13080_e18675;
        locals.var_idd_dn0 = assign13080_e18675_d_n0;
        locals.var_idd_dn2 = assign13080_e18675_d_n2;
        locals.var_idd_dn6 = assign13080_e18675_d_n6;
        locals.var_idd_dn7 = assign13080_e18675_d_n7;
        locals.var_idd_dn10 = assign13080_e18675_d_n10;
        locals.var_idd_dn11 = assign13080_e18675_d_n11;
        locals.var_idd_dn12 = assign13080_e18675_d_n12;
        locals.var_idd_dn17 = assign13080_e18675_d_n17;

        let (assign13090_e18687, assign13090_e18687_d_n0, assign13090_e18687_d_n2, assign13090_e18687_d_n6, assign13090_e18687_d_n7, assign13090_e18687_d_n10, assign13090_e18687_d_n11, assign13090_e18687_d_n12, assign13090_e18687_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign13090_e18681: f64 = (-0.5);
        let assign13090_e18684: f64 = (locals.var_q_depl + locals.var_q_dep0);
        let assign13090_e18685: f64 = (assign13090_e18681 * assign13090_e18684);
        (assign13090_e18685, (assign13090_e18681 * (locals.var_q_depl_dn0 + locals.var_q_dep0_dn0)), (assign13090_e18681 * (locals.var_q_depl_dn2 + locals.var_q_dep0_dn2)), (assign13090_e18681 * (locals.var_q_depl_dn6 + locals.var_q_dep0_dn6)), (assign13090_e18681 * (locals.var_q_depl_dn7 + locals.var_q_dep0_dn7)), (assign13090_e18681 * (locals.var_q_depl_dn10 + locals.var_q_dep0_dn10)), (assign13090_e18681 * (locals.var_q_depl_dn11 + locals.var_q_dep0_dn11)), (assign13090_e18681 * (locals.var_q_depl_dn12 + locals.var_q_dep0_dn12)), (assign13090_e18681 * (locals.var_q_depl_dn17 + locals.var_q_dep0_dn17)),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn12, locals.var_qbu_dn17,)
    }
};
        locals.var_qbu = assign13090_e18687;
        locals.var_qbu_dn0 = assign13090_e18687_d_n0;
        locals.var_qbu_dn2 = assign13090_e18687_d_n2;
        locals.var_qbu_dn6 = assign13090_e18687_d_n6;
        locals.var_qbu_dn7 = assign13090_e18687_d_n7;
        locals.var_qbu_dn10 = assign13090_e18687_d_n10;
        locals.var_qbu_dn11 = assign13090_e18687_d_n11;
        locals.var_qbu_dn12 = assign13090_e18687_d_n12;
        locals.var_qbu_dn17 = assign13090_e18687_d_n17;

        let (assign13100_e18696, assign13100_e18696_d_n0, assign13100_e18696_d_n2, assign13100_e18696_d_n6, assign13100_e18696_d_n7, assign13100_e18696_d_n10, assign13100_e18696_d_n11, assign13100_e18696_d_n12, assign13100_e18696_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign13100_e18694: f64 = (locals.var_phi_sl_soi - locals.var_phi_s0_soi);
        (assign13100_e18694, (locals.var_phi_sl_soi_dn0 - locals.var_phi_s0_soi_dn0), (locals.var_phi_sl_soi_dn2 - locals.var_phi_s0_soi_dn2), (locals.var_phi_sl_soi_dn6 - locals.var_phi_s0_soi_dn6), (locals.var_phi_sl_soi_dn7 - locals.var_phi_s0_soi_dn7), (locals.var_phi_sl_soi_dn10 - locals.var_phi_s0_soi_dn10), (locals.var_phi_sl_soi_dn11 - locals.var_phi_s0_soi_dn11), (locals.var_phi_sl_soi_dn12 - locals.var_phi_s0_soi_dn12), (locals.var_phi_sl_soi_dn17 - locals.var_phi_s0_soi_dn17),)
    } else {
        (locals.var_rrr_p0, locals.var_rrr_p0_dn0, locals.var_rrr_p0_dn2, locals.var_rrr_p0_dn6, locals.var_rrr_p0_dn7, locals.var_rrr_p0_dn10, locals.var_rrr_p0_dn11, locals.var_rrr_p0_dn12, locals.var_rrr_p0_dn17,)
    }
};
        locals.var_rrr_p0 = assign13100_e18696;
        locals.var_rrr_p0_dn0 = assign13100_e18696_d_n0;
        locals.var_rrr_p0_dn2 = assign13100_e18696_d_n2;
        locals.var_rrr_p0_dn6 = assign13100_e18696_d_n6;
        locals.var_rrr_p0_dn7 = assign13100_e18696_d_n7;
        locals.var_rrr_p0_dn10 = assign13100_e18696_d_n10;
        locals.var_rrr_p0_dn11 = assign13100_e18696_d_n11;
        locals.var_rrr_p0_dn12 = assign13100_e18696_d_n12;
        locals.var_rrr_p0_dn17 = assign13100_e18696_d_n17;

        let (assign13110_e18705, assign13110_e18705_d_n0, assign13110_e18705_d_n2, assign13110_e18705_d_n6, assign13110_e18705_d_n7, assign13110_e18705_d_n10, assign13110_e18705_d_n11, assign13110_e18705_d_n12, assign13110_e18705_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign13110_e18703: f64 = (locals.var_rrr_p0 + 5e-12);
        (assign13110_e18703, locals.var_rrr_p0_dn0, locals.var_rrr_p0_dn2, locals.var_rrr_p0_dn6, locals.var_rrr_p0_dn7, locals.var_rrr_p0_dn10, locals.var_rrr_p0_dn11, locals.var_rrr_p0_dn12, locals.var_rrr_p0_dn17,)
    } else {
        (locals.var_rrr_p0, locals.var_rrr_p0_dn0, locals.var_rrr_p0_dn2, locals.var_rrr_p0_dn6, locals.var_rrr_p0_dn7, locals.var_rrr_p0_dn10, locals.var_rrr_p0_dn11, locals.var_rrr_p0_dn12, locals.var_rrr_p0_dn17,)
    }
};
        locals.var_rrr_p0 = assign13110_e18705;
        locals.var_rrr_p0_dn0 = assign13110_e18705_d_n0;
        locals.var_rrr_p0_dn2 = assign13110_e18705_d_n2;
        locals.var_rrr_p0_dn6 = assign13110_e18705_d_n6;
        locals.var_rrr_p0_dn7 = assign13110_e18705_d_n7;
        locals.var_rrr_p0_dn10 = assign13110_e18705_d_n10;
        locals.var_rrr_p0_dn11 = assign13110_e18705_d_n11;
        locals.var_rrr_p0_dn12 = assign13110_e18705_d_n12;
        locals.var_rrr_p0_dn17 = assign13110_e18705_d_n17;

        let (assign13120_e18718, assign13120_e18718_d_n0, assign13120_e18718_d_n2, assign13120_e18718_d_n6, assign13120_e18718_d_n7, assign13120_e18718_d_n10, assign13120_e18718_d_n11, assign13120_e18718_d_n12, assign13120_e18718_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign13120_e18713: f64 = (locals.var_c_box * locals.var_c_s_inv);
        let assign13120_e18715: f64 = (assign13120_e18713 + 1.0);
        let assign13120_e18716: f64 = (locals.var_c_box / assign13120_e18715);
        (assign13120_e18716, (-((locals.var_c_box * (locals.var_c_box * locals.var_c_s_inv_dn0)) / (assign13120_e18715 * assign13120_e18715))), (-((locals.var_c_box * (locals.var_c_box * locals.var_c_s_inv_dn2)) / (assign13120_e18715 * assign13120_e18715))), (-((locals.var_c_box * (locals.var_c_box * locals.var_c_s_inv_dn6)) / (assign13120_e18715 * assign13120_e18715))), (-((locals.var_c_box * (locals.var_c_box * locals.var_c_s_inv_dn7)) / (assign13120_e18715 * assign13120_e18715))), (-((locals.var_c_box * (locals.var_c_box * locals.var_c_s_inv_dn10)) / (assign13120_e18715 * assign13120_e18715))), (-((locals.var_c_box * (locals.var_c_box * locals.var_c_s_inv_dn11)) / (assign13120_e18715 * assign13120_e18715))), (-((locals.var_c_box * (locals.var_c_box * locals.var_c_s_inv_dn12)) / (assign13120_e18715 * assign13120_e18715))), (-((locals.var_c_box * (locals.var_c_box * locals.var_c_s_inv_dn17)) / (assign13120_e18715 * assign13120_e18715))),)
    } else {
        (locals.var_rrr_csoi_cbox, locals.var_rrr_csoi_cbox_dn0, locals.var_rrr_csoi_cbox_dn2, locals.var_rrr_csoi_cbox_dn6, locals.var_rrr_csoi_cbox_dn7, locals.var_rrr_csoi_cbox_dn10, locals.var_rrr_csoi_cbox_dn11, locals.var_rrr_csoi_cbox_dn12, locals.var_rrr_csoi_cbox_dn17,)
    }
};
        locals.var_rrr_csoi_cbox = assign13120_e18718;
        locals.var_rrr_csoi_cbox_dn0 = assign13120_e18718_d_n0;
        locals.var_rrr_csoi_cbox_dn2 = assign13120_e18718_d_n2;
        locals.var_rrr_csoi_cbox_dn6 = assign13120_e18718_d_n6;
        locals.var_rrr_csoi_cbox_dn7 = assign13120_e18718_d_n7;
        locals.var_rrr_csoi_cbox_dn10 = assign13120_e18718_d_n10;
        locals.var_rrr_csoi_cbox_dn11 = assign13120_e18718_d_n11;
        locals.var_rrr_csoi_cbox_dn12 = assign13120_e18718_d_n12;
        locals.var_rrr_csoi_cbox_dn17 = assign13120_e18718_d_n17;

        let (assign13130_e18733, assign13130_e18733_d_n0, assign13130_e18733_d_n2, assign13130_e18733_d_n6, assign13130_e18733_d_n7, assign13130_e18733_d_n10, assign13130_e18733_d_n11, assign13130_e18733_d_n12, assign13130_e18733_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign13130_e18725: f64 = (locals.var_q_sl_bulk * locals.var_q_sl_bulk);
        let assign13130_e18728: f64 = (locals.var_q_s0_bulk * locals.var_q_s0_bulk);
        let assign13130_e18729: f64 = (assign13130_e18725 - assign13130_e18728);
        let assign13130_e18731: f64 = (assign13130_e18729 / locals.var_rrr_csoi_cbox);
        (assign13130_e18731, ((((((locals.var_q_sl_bulk_dn0 * locals.var_q_sl_bulk) + (locals.var_q_sl_bulk * locals.var_q_sl_bulk_dn0)) - ((locals.var_q_s0_bulk_dn0 * locals.var_q_s0_bulk) + (locals.var_q_s0_bulk * locals.var_q_s0_bulk_dn0))) * locals.var_rrr_csoi_cbox) - (assign13130_e18729 * locals.var_rrr_csoi_cbox_dn0)) / (locals.var_rrr_csoi_cbox * locals.var_rrr_csoi_cbox)), ((((((locals.var_q_sl_bulk_dn2 * locals.var_q_sl_bulk) + (locals.var_q_sl_bulk * locals.var_q_sl_bulk_dn2)) - ((locals.var_q_s0_bulk_dn2 * locals.var_q_s0_bulk) + (locals.var_q_s0_bulk * locals.var_q_s0_bulk_dn2))) * locals.var_rrr_csoi_cbox) - (assign13130_e18729 * locals.var_rrr_csoi_cbox_dn2)) / (locals.var_rrr_csoi_cbox * locals.var_rrr_csoi_cbox)), ((((((locals.var_q_sl_bulk_dn6 * locals.var_q_sl_bulk) + (locals.var_q_sl_bulk * locals.var_q_sl_bulk_dn6)) - ((locals.var_q_s0_bulk_dn6 * locals.var_q_s0_bulk) + (locals.var_q_s0_bulk * locals.var_q_s0_bulk_dn6))) * locals.var_rrr_csoi_cbox) - (assign13130_e18729 * locals.var_rrr_csoi_cbox_dn6)) / (locals.var_rrr_csoi_cbox * locals.var_rrr_csoi_cbox)), ((((((locals.var_q_sl_bulk_dn7 * locals.var_q_sl_bulk) + (locals.var_q_sl_bulk * locals.var_q_sl_bulk_dn7)) - ((locals.var_q_s0_bulk_dn7 * locals.var_q_s0_bulk) + (locals.var_q_s0_bulk * locals.var_q_s0_bulk_dn7))) * locals.var_rrr_csoi_cbox) - (assign13130_e18729 * locals.var_rrr_csoi_cbox_dn7)) / (locals.var_rrr_csoi_cbox * locals.var_rrr_csoi_cbox)), ((((((locals.var_q_sl_bulk_dn10 * locals.var_q_sl_bulk) + (locals.var_q_sl_bulk * locals.var_q_sl_bulk_dn10)) - ((locals.var_q_s0_bulk_dn10 * locals.var_q_s0_bulk) + (locals.var_q_s0_bulk * locals.var_q_s0_bulk_dn10))) * locals.var_rrr_csoi_cbox) - (assign13130_e18729 * locals.var_rrr_csoi_cbox_dn10)) / (locals.var_rrr_csoi_cbox * locals.var_rrr_csoi_cbox)), ((((((locals.var_q_sl_bulk_dn11 * locals.var_q_sl_bulk) + (locals.var_q_sl_bulk * locals.var_q_sl_bulk_dn11)) - ((locals.var_q_s0_bulk_dn11 * locals.var_q_s0_bulk) + (locals.var_q_s0_bulk * locals.var_q_s0_bulk_dn11))) * locals.var_rrr_csoi_cbox) - (assign13130_e18729 * locals.var_rrr_csoi_cbox_dn11)) / (locals.var_rrr_csoi_cbox * locals.var_rrr_csoi_cbox)), ((((((locals.var_q_sl_bulk_dn12 * locals.var_q_sl_bulk) + (locals.var_q_sl_bulk * locals.var_q_sl_bulk_dn12)) - ((locals.var_q_s0_bulk_dn12 * locals.var_q_s0_bulk) + (locals.var_q_s0_bulk * locals.var_q_s0_bulk_dn12))) * locals.var_rrr_csoi_cbox) - (assign13130_e18729 * locals.var_rrr_csoi_cbox_dn12)) / (locals.var_rrr_csoi_cbox * locals.var_rrr_csoi_cbox)), ((((((locals.var_q_sl_bulk_dn17 * locals.var_q_sl_bulk) + (locals.var_q_sl_bulk * locals.var_q_sl_bulk_dn17)) - ((locals.var_q_s0_bulk_dn17 * locals.var_q_s0_bulk) + (locals.var_q_s0_bulk * locals.var_q_s0_bulk_dn17))) * locals.var_rrr_csoi_cbox) - (assign13130_e18729 * locals.var_rrr_csoi_cbox_dn17)) / (locals.var_rrr_csoi_cbox * locals.var_rrr_csoi_cbox)),)
    } else {
        (locals.var_rrr_b, locals.var_rrr_b_dn0, locals.var_rrr_b_dn2, locals.var_rrr_b_dn6, locals.var_rrr_b_dn7, locals.var_rrr_b_dn10, locals.var_rrr_b_dn11, locals.var_rrr_b_dn12, locals.var_rrr_b_dn17,)
    }
};
        locals.var_rrr_b = assign13130_e18733;
        locals.var_rrr_b_dn0 = assign13130_e18733_d_n0;
        locals.var_rrr_b_dn2 = assign13130_e18733_d_n2;
        locals.var_rrr_b_dn6 = assign13130_e18733_d_n6;
        locals.var_rrr_b_dn7 = assign13130_e18733_d_n7;
        locals.var_rrr_b_dn10 = assign13130_e18733_d_n10;
        locals.var_rrr_b_dn11 = assign13130_e18733_d_n11;
        locals.var_rrr_b_dn12 = assign13130_e18733_d_n12;
        locals.var_rrr_b_dn17 = assign13130_e18733_d_n17;

        let assign13140_e18735: f64 = (-locals.var_rrr_b);
        let assign13140_e18739: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13140_e18740: f64 = assign13140_e18739;
        let assign13140_e18744: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13140_e18747: f64 = if ((assign13140_e18735 < assign13140_e18740) && (assign13140_e18744 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard416 = assign13140_e18747;

        let (assign13150_e18763, assign13150_e18763_d_n0, assign13150_e18763_d_n2, assign13150_e18763_d_n6, assign13150_e18763_d_n7, assign13150_e18763_d_n10, assign13150_e18763_d_n11, assign13150_e18763_d_n12, assign13150_e18763_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign13150_e18757: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13150_e18758: f64 = assign13150_e18757;
        let assign13150_e18760: f64 = (-locals.var_rrr_b);
        let assign13150_e18761: f64 = (assign13150_e18758 - assign13150_e18760);
        (assign13150_e18761, ((locals.var_q_fd_soi_dn0 * 1e-5) - (-locals.var_rrr_b_dn0)), ((locals.var_q_fd_soi_dn2 * 1e-5) - (-locals.var_rrr_b_dn2)), ((locals.var_q_fd_soi_dn6 * 1e-5) - (-locals.var_rrr_b_dn6)), ((locals.var_q_fd_soi_dn7 * 1e-5) - (-locals.var_rrr_b_dn7)), ((locals.var_q_fd_soi_dn10 * 1e-5) - (-locals.var_rrr_b_dn10)), ((locals.var_q_fd_soi_dn11 * 1e-5) - (-locals.var_rrr_b_dn11)), ((locals.var_q_fd_soi_dn12 * 1e-5) - (-locals.var_rrr_b_dn12)), ((locals.var_q_fd_soi_dn17 * 1e-5) - (-locals.var_rrr_b_dn17)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign13150_e18763;
        locals.var_tmf1_dn0 = assign13150_e18763_d_n0;
        locals.var_tmf1_dn2 = assign13150_e18763_d_n2;
        locals.var_tmf1_dn6 = assign13150_e18763_d_n6;
        locals.var_tmf1_dn7 = assign13150_e18763_d_n7;
        locals.var_tmf1_dn10 = assign13150_e18763_d_n10;
        locals.var_tmf1_dn11 = assign13150_e18763_d_n11;
        locals.var_tmf1_dn12 = assign13150_e18763_d_n12;
        locals.var_tmf1_dn17 = assign13150_e18763_d_n17;

        let (assign13160_e18774, assign13160_e18774_d_n0, assign13160_e18774_d_n2, assign13160_e18774_d_n6, assign13160_e18774_d_n7, assign13160_e18774_d_n10, assign13160_e18774_d_n11, assign13160_e18774_d_n12, assign13160_e18774_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign13160_e18772: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign13160_e18772, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign13160_e18774;
        locals.var_x2_dn0 = assign13160_e18774_d_n0;
        locals.var_x2_dn2 = assign13160_e18774_d_n2;
        locals.var_x2_dn6 = assign13160_e18774_d_n6;
        locals.var_x2_dn7 = assign13160_e18774_d_n7;
        locals.var_x2_dn10 = assign13160_e18774_d_n10;
        locals.var_x2_dn11 = assign13160_e18774_d_n11;
        locals.var_x2_dn12 = assign13160_e18774_d_n12;
        locals.var_x2_dn17 = assign13160_e18774_d_n17;

        let (assign13170_e18789, assign13170_e18789_d_n0, assign13170_e18789_d_n2, assign13170_e18789_d_n6, assign13170_e18789_d_n7, assign13170_e18789_d_n10, assign13170_e18789_d_n11, assign13170_e18789_d_n12, assign13170_e18789_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign13170_e18783: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13170_e18786: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13170_e18787: f64 = (assign13170_e18783 * assign13170_e18786);
        (assign13170_e18787, (((locals.var_q_fd_soi_dn0 * 1e-5) * assign13170_e18786) + (assign13170_e18783 * (locals.var_q_fd_soi_dn0 * 1e-5))), (((locals.var_q_fd_soi_dn2 * 1e-5) * assign13170_e18786) + (assign13170_e18783 * (locals.var_q_fd_soi_dn2 * 1e-5))), (((locals.var_q_fd_soi_dn6 * 1e-5) * assign13170_e18786) + (assign13170_e18783 * (locals.var_q_fd_soi_dn6 * 1e-5))), (((locals.var_q_fd_soi_dn7 * 1e-5) * assign13170_e18786) + (assign13170_e18783 * (locals.var_q_fd_soi_dn7 * 1e-5))), (((locals.var_q_fd_soi_dn10 * 1e-5) * assign13170_e18786) + (assign13170_e18783 * (locals.var_q_fd_soi_dn10 * 1e-5))), (((locals.var_q_fd_soi_dn11 * 1e-5) * assign13170_e18786) + (assign13170_e18783 * (locals.var_q_fd_soi_dn11 * 1e-5))), (((locals.var_q_fd_soi_dn12 * 1e-5) * assign13170_e18786) + (assign13170_e18783 * (locals.var_q_fd_soi_dn12 * 1e-5))), (((locals.var_q_fd_soi_dn17 * 1e-5) * assign13170_e18786) + (assign13170_e18783 * (locals.var_q_fd_soi_dn17 * 1e-5))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign13170_e18789;
        locals.var_xmax2_dn0 = assign13170_e18789_d_n0;
        locals.var_xmax2_dn2 = assign13170_e18789_d_n2;
        locals.var_xmax2_dn6 = assign13170_e18789_d_n6;
        locals.var_xmax2_dn7 = assign13170_e18789_d_n7;
        locals.var_xmax2_dn10 = assign13170_e18789_d_n10;
        locals.var_xmax2_dn11 = assign13170_e18789_d_n11;
        locals.var_xmax2_dn12 = assign13170_e18789_d_n12;
        locals.var_xmax2_dn17 = assign13170_e18789_d_n17;

        let (assign13180_e18798, assign13180_e18798_d_n0, assign13180_e18798_d_n2, assign13180_e18798_d_n6, assign13180_e18798_d_n7, assign13180_e18798_d_n10, assign13180_e18798_d_n11, assign13180_e18798_d_n12, assign13180_e18798_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard416 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13180_e18798;
        locals.var_xp_dn0 = assign13180_e18798_d_n0;
        locals.var_xp_dn2 = assign13180_e18798_d_n2;
        locals.var_xp_dn6 = assign13180_e18798_d_n6;
        locals.var_xp_dn7 = assign13180_e18798_d_n7;
        locals.var_xp_dn10 = assign13180_e18798_d_n10;
        locals.var_xp_dn11 = assign13180_e18798_d_n11;
        locals.var_xp_dn12 = assign13180_e18798_d_n12;
        locals.var_xp_dn17 = assign13180_e18798_d_n17;

        let (assign13190_e18807, assign13190_e18807_d_n0, assign13190_e18807_d_n2, assign13190_e18807_d_n6, assign13190_e18807_d_n7, assign13190_e18807_d_n10, assign13190_e18807_d_n11, assign13190_e18807_d_n12, assign13190_e18807_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard416 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13190_e18807;
        locals.var_xmp_dn0 = assign13190_e18807_d_n0;
        locals.var_xmp_dn2 = assign13190_e18807_d_n2;
        locals.var_xmp_dn6 = assign13190_e18807_d_n6;
        locals.var_xmp_dn7 = assign13190_e18807_d_n7;
        locals.var_xmp_dn10 = assign13190_e18807_d_n10;
        locals.var_xmp_dn11 = assign13190_e18807_d_n11;
        locals.var_xmp_dn12 = assign13190_e18807_d_n12;
        locals.var_xmp_dn17 = assign13190_e18807_d_n17;

        let (assign13200_e18816,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard416 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign13200_e18816;

        let (assign13210_e18825,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard416 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13210_e18825;

        let (assign13220_e18834, assign13220_e18834_d_n0, assign13220_e18834_d_n2, assign13220_e18834_d_n6, assign13220_e18834_d_n7, assign13220_e18834_d_n10, assign13220_e18834_d_n11, assign13220_e18834_d_n12, assign13220_e18834_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard416 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign13220_e18834;
        locals.var_arg_dn0 = assign13220_e18834_d_n0;
        locals.var_arg_dn2 = assign13220_e18834_d_n2;
        locals.var_arg_dn6 = assign13220_e18834_d_n6;
        locals.var_arg_dn7 = assign13220_e18834_d_n7;
        locals.var_arg_dn10 = assign13220_e18834_d_n10;
        locals.var_arg_dn11 = assign13220_e18834_d_n11;
        locals.var_arg_dn12 = assign13220_e18834_d_n12;
        locals.var_arg_dn17 = assign13220_e18834_d_n17;

        let (assign13230_e18843, assign13230_e18843_d_n0, assign13230_e18843_d_n2, assign13230_e18843_d_n6, assign13230_e18843_d_n7, assign13230_e18843_d_n10, assign13230_e18843_d_n11, assign13230_e18843_d_n12, assign13230_e18843_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard416 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13230_e18843;
        locals.var_dnm_dn0 = assign13230_e18843_d_n0;
        locals.var_dnm_dn2 = assign13230_e18843_d_n2;
        locals.var_dnm_dn6 = assign13230_e18843_d_n6;
        locals.var_dnm_dn7 = assign13230_e18843_d_n7;
        locals.var_dnm_dn10 = assign13230_e18843_d_n10;
        locals.var_dnm_dn11 = assign13230_e18843_d_n11;
        locals.var_dnm_dn12 = assign13230_e18843_d_n12;
        locals.var_dnm_dn17 = assign13230_e18843_d_n17;

    }

    pub(super) fn stamp_transient_block_43(
        locals: &mut StampLocals,
    ) {
        let (assign13240_e18854, assign13240_e18854_d_n0, assign13240_e18854_d_n2, assign13240_e18854_d_n6, assign13240_e18854_d_n7, assign13240_e18854_d_n10, assign13240_e18854_d_n11, assign13240_e18854_d_n12, assign13240_e18854_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign13240_e18852: f64 = (locals.var_xp * locals.var_x2);
        (assign13240_e18852, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13240_e18854;
        locals.var_xp_dn0 = assign13240_e18854_d_n0;
        locals.var_xp_dn2 = assign13240_e18854_d_n2;
        locals.var_xp_dn6 = assign13240_e18854_d_n6;
        locals.var_xp_dn7 = assign13240_e18854_d_n7;
        locals.var_xp_dn10 = assign13240_e18854_d_n10;
        locals.var_xp_dn11 = assign13240_e18854_d_n11;
        locals.var_xp_dn12 = assign13240_e18854_d_n12;
        locals.var_xp_dn17 = assign13240_e18854_d_n17;

        let (assign13250_e18865, assign13250_e18865_d_n0, assign13250_e18865_d_n2, assign13250_e18865_d_n6, assign13250_e18865_d_n7, assign13250_e18865_d_n10, assign13250_e18865_d_n11, assign13250_e18865_d_n12, assign13250_e18865_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign13250_e18863: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign13250_e18863, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13250_e18865;
        locals.var_xmp_dn0 = assign13250_e18865_d_n0;
        locals.var_xmp_dn2 = assign13250_e18865_d_n2;
        locals.var_xmp_dn6 = assign13250_e18865_d_n6;
        locals.var_xmp_dn7 = assign13250_e18865_d_n7;
        locals.var_xmp_dn10 = assign13250_e18865_d_n10;
        locals.var_xmp_dn11 = assign13250_e18865_d_n11;
        locals.var_xmp_dn12 = assign13250_e18865_d_n12;
        locals.var_xmp_dn17 = assign13250_e18865_d_n17;

        let (assign13260_e18876, assign13260_e18876_d_n0, assign13260_e18876_d_n2, assign13260_e18876_d_n6, assign13260_e18876_d_n7, assign13260_e18876_d_n10, assign13260_e18876_d_n11, assign13260_e18876_d_n12, assign13260_e18876_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign13260_e18874: f64 = (locals.var_xp * locals.var_x2);
        (assign13260_e18874, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13260_e18876;
        locals.var_xp_dn0 = assign13260_e18876_d_n0;
        locals.var_xp_dn2 = assign13260_e18876_d_n2;
        locals.var_xp_dn6 = assign13260_e18876_d_n6;
        locals.var_xp_dn7 = assign13260_e18876_d_n7;
        locals.var_xp_dn10 = assign13260_e18876_d_n10;
        locals.var_xp_dn11 = assign13260_e18876_d_n11;
        locals.var_xp_dn12 = assign13260_e18876_d_n12;
        locals.var_xp_dn17 = assign13260_e18876_d_n17;

        let (assign13270_e18887, assign13270_e18887_d_n0, assign13270_e18887_d_n2, assign13270_e18887_d_n6, assign13270_e18887_d_n7, assign13270_e18887_d_n10, assign13270_e18887_d_n11, assign13270_e18887_d_n12, assign13270_e18887_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign13270_e18885: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign13270_e18885, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13270_e18887;
        locals.var_xmp_dn0 = assign13270_e18887_d_n0;
        locals.var_xmp_dn2 = assign13270_e18887_d_n2;
        locals.var_xmp_dn6 = assign13270_e18887_d_n6;
        locals.var_xmp_dn7 = assign13270_e18887_d_n7;
        locals.var_xmp_dn10 = assign13270_e18887_d_n10;
        locals.var_xmp_dn11 = assign13270_e18887_d_n11;
        locals.var_xmp_dn12 = assign13270_e18887_d_n12;
        locals.var_xmp_dn17 = assign13270_e18887_d_n17;

        let (assign13280_e18898, assign13280_e18898_d_n0, assign13280_e18898_d_n2, assign13280_e18898_d_n6, assign13280_e18898_d_n7, assign13280_e18898_d_n10, assign13280_e18898_d_n11, assign13280_e18898_d_n12, assign13280_e18898_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign13280_e18896: f64 = (locals.var_xp + locals.var_xmp);
        (assign13280_e18896, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign13280_e18898;
        locals.var_arg_dn0 = assign13280_e18898_d_n0;
        locals.var_arg_dn2 = assign13280_e18898_d_n2;
        locals.var_arg_dn6 = assign13280_e18898_d_n6;
        locals.var_arg_dn7 = assign13280_e18898_d_n7;
        locals.var_arg_dn10 = assign13280_e18898_d_n10;
        locals.var_arg_dn11 = assign13280_e18898_d_n11;
        locals.var_arg_dn12 = assign13280_e18898_d_n12;
        locals.var_arg_dn17 = assign13280_e18898_d_n17;

        let (assign13290_e18907, assign13290_e18907_d_n0, assign13290_e18907_d_n2, assign13290_e18907_d_n6, assign13290_e18907_d_n7, assign13290_e18907_d_n10, assign13290_e18907_d_n11, assign13290_e18907_d_n12, assign13290_e18907_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard416 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13290_e18907;
        locals.var_dnm_dn0 = assign13290_e18907_d_n0;
        locals.var_dnm_dn2 = assign13290_e18907_d_n2;
        locals.var_dnm_dn6 = assign13290_e18907_d_n6;
        locals.var_dnm_dn7 = assign13290_e18907_d_n7;
        locals.var_dnm_dn10 = assign13290_e18907_d_n10;
        locals.var_dnm_dn11 = assign13290_e18907_d_n11;
        locals.var_dnm_dn12 = assign13290_e18907_d_n12;
        locals.var_dnm_dn17 = assign13290_e18907_d_n17;

        let assign13300_e18922: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard417 = assign13300_e18922;

        let assign13310_e18925: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard418 = assign13310_e18925;

        let (assign13320_e18938,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard416 != 0.0)) && (locals.var_guard417 != 0.0)) && (locals.var_guard418 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13320_e18938;

        let assign13330_e18941: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard419 = assign13330_e18941;

        let (assign13340_e18957,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard416 != 0.0)) && (locals.var_guard417 != 0.0)) && (locals.var_guard418 == 0.0)) && (locals.var_guard419 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13340_e18957;

        let assign13350_e18960: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard420 = assign13350_e18960;

        let (assign13360_e18979,) = {
    if (((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard416 != 0.0)) && (locals.var_guard417 != 0.0)) && (locals.var_guard418 == 0.0)) && (locals.var_guard419 == 0.0)) && (locals.var_guard420 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13360_e18979;

        let assign13370_e18982: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard421 = assign13370_e18982;

        let (assign13380_e19004,) = {
    if ((((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard416 != 0.0)) && (locals.var_guard417 != 0.0)) && (locals.var_guard418 == 0.0)) && (locals.var_guard419 == 0.0)) && (locals.var_guard420 == 0.0)) && (locals.var_guard421 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13380_e19004;

        let (assign13390_e19015,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard416 != 0.0)) && (locals.var_guard417 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign13390_e19015;

        let mut assign13400_loop_guard: usize = 0;
        while {
            let assign13400_cond_e19027: f64 = if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard416 != 0.0)) && (locals.var_guard417 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign13400_cond_e19027 != 0.0
        } {
            assign13400_loop_guard += 1;
            assert!(assign13400_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign13400_body0_e19039, assign13400_body0_e19039_d_n0, assign13400_body0_e19039_d_n2, assign13400_body0_e19039_d_n6, assign13400_body0_e19039_d_n7, assign13400_body0_e19039_d_n10, assign13400_body0_e19039_d_n11, assign13400_body0_e19039_d_n12, assign13400_body0_e19039_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard416 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign13400_body0_e19037: f64 = (locals.var_dnm).sqrt();
        (assign13400_body0_e19037, (locals.var_dnm_dn0 / (2.0 * assign13400_body0_e19037)), (locals.var_dnm_dn2 / (2.0 * assign13400_body0_e19037)), (locals.var_dnm_dn6 / (2.0 * assign13400_body0_e19037)), (locals.var_dnm_dn7 / (2.0 * assign13400_body0_e19037)), (locals.var_dnm_dn10 / (2.0 * assign13400_body0_e19037)), (locals.var_dnm_dn11 / (2.0 * assign13400_body0_e19037)), (locals.var_dnm_dn12 / (2.0 * assign13400_body0_e19037)), (locals.var_dnm_dn17 / (2.0 * assign13400_body0_e19037)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign13400_body0_e19039;
            locals.var_dnm_dn0 = assign13400_body0_e19039_d_n0;
            locals.var_dnm_dn2 = assign13400_body0_e19039_d_n2;
            locals.var_dnm_dn6 = assign13400_body0_e19039_d_n6;
            locals.var_dnm_dn7 = assign13400_body0_e19039_d_n7;
            locals.var_dnm_dn10 = assign13400_body0_e19039_d_n10;
            locals.var_dnm_dn11 = assign13400_body0_e19039_d_n11;
            locals.var_dnm_dn12 = assign13400_body0_e19039_d_n12;
            locals.var_dnm_dn17 = assign13400_body0_e19039_d_n17;
            let (assign13400_body1_e19052,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard416 != 0.0)) && (locals.var_guard417 != 0.0)) {
        let assign13400_body1_e19050: f64 = (locals.var_m0 + 1.0);
        (assign13400_body1_e19050,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign13400_body1_e19052;
        }

        let (assign13410_e19070, assign13410_e19070_d_n0, assign13410_e19070_d_n2, assign13410_e19070_d_n6, assign13410_e19070_d_n7, assign13410_e19070_d_n10, assign13410_e19070_d_n11, assign13410_e19070_d_n12, assign13410_e19070_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard416 != 0.0)) && (locals.var_guard417 == 0.0)) {
        let assign13410_e19066: f64 = (2.0 * 2.0);
        let assign13410_e19067: f64 = (1.0 / assign13410_e19066);
        let assign13410_e19068: f64 = (locals.var_dnm).powf(assign13410_e19067);
        (assign13410_e19068, if 0.0 == 0.0 && ((assign13410_e19067) as f64).is_finite() && ((assign13410_e19067) as f64).fract() == 0.0 { if assign13410_e19067 == 0.0 { 0.0 } else { (assign13410_e19067 * ((locals.var_dnm).powf(assign13410_e19067 - 1.0) * locals.var_dnm_dn0)) } } else { (assign13410_e19068 * (assign13410_e19067 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13410_e19067) as f64).is_finite() && ((assign13410_e19067) as f64).fract() == 0.0 { if assign13410_e19067 == 0.0 { 0.0 } else { (assign13410_e19067 * ((locals.var_dnm).powf(assign13410_e19067 - 1.0) * locals.var_dnm_dn2)) } } else { (assign13410_e19068 * (assign13410_e19067 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13410_e19067) as f64).is_finite() && ((assign13410_e19067) as f64).fract() == 0.0 { if assign13410_e19067 == 0.0 { 0.0 } else { (assign13410_e19067 * ((locals.var_dnm).powf(assign13410_e19067 - 1.0) * locals.var_dnm_dn6)) } } else { (assign13410_e19068 * (assign13410_e19067 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13410_e19067) as f64).is_finite() && ((assign13410_e19067) as f64).fract() == 0.0 { if assign13410_e19067 == 0.0 { 0.0 } else { (assign13410_e19067 * ((locals.var_dnm).powf(assign13410_e19067 - 1.0) * locals.var_dnm_dn7)) } } else { (assign13410_e19068 * (assign13410_e19067 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13410_e19067) as f64).is_finite() && ((assign13410_e19067) as f64).fract() == 0.0 { if assign13410_e19067 == 0.0 { 0.0 } else { (assign13410_e19067 * ((locals.var_dnm).powf(assign13410_e19067 - 1.0) * locals.var_dnm_dn10)) } } else { (assign13410_e19068 * (assign13410_e19067 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13410_e19067) as f64).is_finite() && ((assign13410_e19067) as f64).fract() == 0.0 { if assign13410_e19067 == 0.0 { 0.0 } else { (assign13410_e19067 * ((locals.var_dnm).powf(assign13410_e19067 - 1.0) * locals.var_dnm_dn11)) } } else { (assign13410_e19068 * (assign13410_e19067 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13410_e19067) as f64).is_finite() && ((assign13410_e19067) as f64).fract() == 0.0 { if assign13410_e19067 == 0.0 { 0.0 } else { (assign13410_e19067 * ((locals.var_dnm).powf(assign13410_e19067 - 1.0) * locals.var_dnm_dn12)) } } else { (assign13410_e19068 * (assign13410_e19067 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13410_e19067) as f64).is_finite() && ((assign13410_e19067) as f64).fract() == 0.0 { if assign13410_e19067 == 0.0 { 0.0 } else { (assign13410_e19067 * ((locals.var_dnm).powf(assign13410_e19067 - 1.0) * locals.var_dnm_dn17)) } } else { (assign13410_e19068 * (assign13410_e19067 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13410_e19070;
        locals.var_dnm_dn0 = assign13410_e19070_d_n0;
        locals.var_dnm_dn2 = assign13410_e19070_d_n2;
        locals.var_dnm_dn6 = assign13410_e19070_d_n6;
        locals.var_dnm_dn7 = assign13410_e19070_d_n7;
        locals.var_dnm_dn10 = assign13410_e19070_d_n10;
        locals.var_dnm_dn11 = assign13410_e19070_d_n11;
        locals.var_dnm_dn12 = assign13410_e19070_d_n12;
        locals.var_dnm_dn17 = assign13410_e19070_d_n17;

        let (assign13420_e19081, assign13420_e19081_d_n0, assign13420_e19081_d_n2, assign13420_e19081_d_n6, assign13420_e19081_d_n7, assign13420_e19081_d_n10, assign13420_e19081_d_n11, assign13420_e19081_d_n12, assign13420_e19081_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign13420_e19079: f64 = (1.0 / locals.var_dnm);
        (assign13420_e19079, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13420_e19081;
        locals.var_dnm_dn0 = assign13420_e19081_d_n0;
        locals.var_dnm_dn2 = assign13420_e19081_d_n2;
        locals.var_dnm_dn6 = assign13420_e19081_d_n6;
        locals.var_dnm_dn7 = assign13420_e19081_d_n7;
        locals.var_dnm_dn10 = assign13420_e19081_d_n10;
        locals.var_dnm_dn11 = assign13420_e19081_d_n11;
        locals.var_dnm_dn12 = assign13420_e19081_d_n12;
        locals.var_dnm_dn17 = assign13420_e19081_d_n17;

        let (assign13430_e19096, assign13430_e19096_d_n0, assign13430_e19096_d_n2, assign13430_e19096_d_n6, assign13430_e19096_d_n7, assign13430_e19096_d_n10, assign13430_e19096_d_n11, assign13430_e19096_d_n12, assign13430_e19096_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign13430_e19091: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13430_e19092: f64 = (locals.var_tmf1 * assign13430_e19091);
        let assign13430_e19094: f64 = (assign13430_e19092 * locals.var_dnm);
        (assign13430_e19094, ((((locals.var_tmf1_dn0 * assign13430_e19091) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn0 * 1e-5))) * locals.var_dnm) + (assign13430_e19092 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign13430_e19091) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn2 * 1e-5))) * locals.var_dnm) + (assign13430_e19092 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn6 * assign13430_e19091) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn6 * 1e-5))) * locals.var_dnm) + (assign13430_e19092 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign13430_e19091) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn7 * 1e-5))) * locals.var_dnm) + (assign13430_e19092 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn10 * assign13430_e19091) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn10 * 1e-5))) * locals.var_dnm) + (assign13430_e19092 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * assign13430_e19091) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn11 * 1e-5))) * locals.var_dnm) + (assign13430_e19092 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn12 * assign13430_e19091) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn12 * 1e-5))) * locals.var_dnm) + (assign13430_e19092 * locals.var_dnm_dn12)), ((((locals.var_tmf1_dn17 * assign13430_e19091) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn17 * 1e-5))) * locals.var_dnm) + (assign13430_e19092 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign13430_e19096;
        locals.var_tmf0_dn0 = assign13430_e19096_d_n0;
        locals.var_tmf0_dn2 = assign13430_e19096_d_n2;
        locals.var_tmf0_dn6 = assign13430_e19096_d_n6;
        locals.var_tmf0_dn7 = assign13430_e19096_d_n7;
        locals.var_tmf0_dn10 = assign13430_e19096_d_n10;
        locals.var_tmf0_dn11 = assign13430_e19096_d_n11;
        locals.var_tmf0_dn12 = assign13430_e19096_d_n12;
        locals.var_tmf0_dn17 = assign13430_e19096_d_n17;

        let (assign13440_e19111, assign13440_e19111_d_n0, assign13440_e19111_d_n2, assign13440_e19111_d_n6, assign13440_e19111_d_n7, assign13440_e19111_d_n10, assign13440_e19111_d_n11, assign13440_e19111_d_n12, assign13440_e19111_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign13440_e19106: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13440_e19107: f64 = assign13440_e19106;
        let assign13440_e19109: f64 = (assign13440_e19107 - locals.var_tmf0);
        (assign13440_e19109, ((locals.var_q_fd_soi_dn0 * 1e-5) - locals.var_tmf0_dn0), ((locals.var_q_fd_soi_dn2 * 1e-5) - locals.var_tmf0_dn2), ((locals.var_q_fd_soi_dn6 * 1e-5) - locals.var_tmf0_dn6), ((locals.var_q_fd_soi_dn7 * 1e-5) - locals.var_tmf0_dn7), ((locals.var_q_fd_soi_dn10 * 1e-5) - locals.var_tmf0_dn10), ((locals.var_q_fd_soi_dn11 * 1e-5) - locals.var_tmf0_dn11), ((locals.var_q_fd_soi_dn12 * 1e-5) - locals.var_tmf0_dn12), ((locals.var_q_fd_soi_dn17 * 1e-5) - locals.var_tmf0_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign13440_e19111;
        locals.var_t1_dn0 = assign13440_e19111_d_n0;
        locals.var_t1_dn2 = assign13440_e19111_d_n2;
        locals.var_t1_dn6 = assign13440_e19111_d_n6;
        locals.var_t1_dn7 = assign13440_e19111_d_n7;
        locals.var_t1_dn10 = assign13440_e19111_d_n10;
        locals.var_t1_dn11 = assign13440_e19111_d_n11;
        locals.var_t1_dn12 = assign13440_e19111_d_n12;
        locals.var_t1_dn17 = assign13440_e19111_d_n17;

        let (assign13450_e19122, assign13450_e19122_d_n0, assign13450_e19122_d_n2, assign13450_e19122_d_n6, assign13450_e19122_d_n7, assign13450_e19122_d_n10, assign13450_e19122_d_n11, assign13450_e19122_d_n12, assign13450_e19122_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard416 == 0.0)) {
        let assign13450_e19120: f64 = (-locals.var_rrr_b);
        (assign13450_e19120, (-locals.var_rrr_b_dn0), (-locals.var_rrr_b_dn2), (-locals.var_rrr_b_dn6), (-locals.var_rrr_b_dn7), (-locals.var_rrr_b_dn10), (-locals.var_rrr_b_dn11), (-locals.var_rrr_b_dn12), (-locals.var_rrr_b_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign13450_e19122;
        locals.var_t1_dn0 = assign13450_e19122_d_n0;
        locals.var_t1_dn2 = assign13450_e19122_d_n2;
        locals.var_t1_dn6 = assign13450_e19122_d_n6;
        locals.var_t1_dn7 = assign13450_e19122_d_n7;
        locals.var_t1_dn10 = assign13450_e19122_d_n10;
        locals.var_t1_dn11 = assign13450_e19122_d_n11;
        locals.var_t1_dn12 = assign13450_e19122_d_n12;
        locals.var_t1_dn17 = assign13450_e19122_d_n17;

        let (assign13460_e19130, assign13460_e19130_d_n0, assign13460_e19130_d_n2, assign13460_e19130_d_n6, assign13460_e19130_d_n7, assign13460_e19130_d_n10, assign13460_e19130_d_n11, assign13460_e19130_d_n12, assign13460_e19130_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign13460_e19128: f64 = (-locals.var_t1);
        (assign13460_e19128, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn10), (-locals.var_t1_dn11), (-locals.var_t1_dn12), (-locals.var_t1_dn17),)
    } else {
        (locals.var_rrr_b, locals.var_rrr_b_dn0, locals.var_rrr_b_dn2, locals.var_rrr_b_dn6, locals.var_rrr_b_dn7, locals.var_rrr_b_dn10, locals.var_rrr_b_dn11, locals.var_rrr_b_dn12, locals.var_rrr_b_dn17,)
    }
};
        locals.var_rrr_b = assign13460_e19130;
        locals.var_rrr_b_dn0 = assign13460_e19130_d_n0;
        locals.var_rrr_b_dn2 = assign13460_e19130_d_n2;
        locals.var_rrr_b_dn6 = assign13460_e19130_d_n6;
        locals.var_rrr_b_dn7 = assign13460_e19130_d_n7;
        locals.var_rrr_b_dn10 = assign13460_e19130_d_n10;
        locals.var_rrr_b_dn11 = assign13460_e19130_d_n11;
        locals.var_rrr_b_dn12 = assign13460_e19130_d_n12;
        locals.var_rrr_b_dn17 = assign13460_e19130_d_n17;

        let assign13470_e19133: f64 = (locals.var_beta * locals.var_ps0b);
        let assign13470_e19135: f64 = (assign13470_e19133 - 1.0);
        let assign13470_e19137: f64 = if assign13470_e19135 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard422 = assign13470_e19137;

        let (assign13480_e19151, assign13480_e19151_d_n0, assign13480_e19151_d_n2, assign13480_e19151_d_n6, assign13480_e19151_d_n7, assign13480_e19151_d_n10, assign13480_e19151_d_n11, assign13480_e19151_d_n12, assign13480_e19151_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard422 != 0.0)) {
        let assign13480_e19146: f64 = (locals.var_beta * locals.var_ps0b);
        let assign13480_e19148: f64 = (assign13480_e19146 - 1.0);
        let assign13480_e19149: f64 = (assign13480_e19148).sqrt();
        (assign13480_e19149, ((locals.var_beta * locals.var_ps0b_dn0) / (2.0 * assign13480_e19149)), ((locals.var_beta * locals.var_ps0b_dn2) / (2.0 * assign13480_e19149)), ((locals.var_beta * locals.var_ps0b_dn6) / (2.0 * assign13480_e19149)), ((locals.var_beta * locals.var_ps0b_dn7) / (2.0 * assign13480_e19149)), (((locals.var_beta_dn10 * locals.var_ps0b) + (locals.var_beta * locals.var_ps0b_dn10)) / (2.0 * assign13480_e19149)), ((locals.var_beta * locals.var_ps0b_dn11) / (2.0 * assign13480_e19149)), ((locals.var_beta * locals.var_ps0b_dn12) / (2.0 * assign13480_e19149)), ((locals.var_beta * locals.var_ps0b_dn17) / (2.0 * assign13480_e19149)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign13480_e19151;
        locals.var_t1_dn0 = assign13480_e19151_d_n0;
        locals.var_t1_dn2 = assign13480_e19151_d_n2;
        locals.var_t1_dn6 = assign13480_e19151_d_n6;
        locals.var_t1_dn7 = assign13480_e19151_d_n7;
        locals.var_t1_dn10 = assign13480_e19151_d_n10;
        locals.var_t1_dn11 = assign13480_e19151_d_n11;
        locals.var_t1_dn12 = assign13480_e19151_d_n12;
        locals.var_t1_dn17 = assign13480_e19151_d_n17;

        let (assign13490_e19161, assign13490_e19161_d_n0, assign13490_e19161_d_n2, assign13490_e19161_d_n6, assign13490_e19161_d_n7, assign13490_e19161_d_n10, assign13490_e19161_d_n11, assign13490_e19161_d_n12, assign13490_e19161_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign13490_e19158: f64 = (locals.var_q_nl - locals.var_q_n0);
        let assign13490_e19159: f64 = (-assign13490_e19158);
        (assign13490_e19159, (-(locals.var_q_nl_dn0 - locals.var_q_n0_dn0)), (-(locals.var_q_nl_dn2 - locals.var_q_n0_dn2)), (-(locals.var_q_nl_dn6 - locals.var_q_n0_dn6)), (-(locals.var_q_nl_dn7 - locals.var_q_n0_dn7)), (-(locals.var_q_nl_dn10 - locals.var_q_n0_dn10)), (-(locals.var_q_nl_dn11 - locals.var_q_n0_dn11)), (-(locals.var_q_nl_dn12 - locals.var_q_n0_dn12)), (-(locals.var_q_nl_dn17 - locals.var_q_n0_dn17)),)
    } else {
        (locals.var_rrr_cc, locals.var_rrr_cc_dn0, locals.var_rrr_cc_dn2, locals.var_rrr_cc_dn6, locals.var_rrr_cc_dn7, locals.var_rrr_cc_dn10, locals.var_rrr_cc_dn11, locals.var_rrr_cc_dn12, locals.var_rrr_cc_dn17,)
    }
};
        locals.var_rrr_cc = assign13490_e19161;
        locals.var_rrr_cc_dn0 = assign13490_e19161_d_n0;
        locals.var_rrr_cc_dn2 = assign13490_e19161_d_n2;
        locals.var_rrr_cc_dn6 = assign13490_e19161_d_n6;
        locals.var_rrr_cc_dn7 = assign13490_e19161_d_n7;
        locals.var_rrr_cc_dn10 = assign13490_e19161_d_n10;
        locals.var_rrr_cc_dn11 = assign13490_e19161_d_n11;
        locals.var_rrr_cc_dn12 = assign13490_e19161_d_n12;
        locals.var_rrr_cc_dn17 = assign13490_e19161_d_n17;

        let assign13500_e19166: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13500_e19167: f64 = assign13500_e19166;
        let assign13500_e19171: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13500_e19174: f64 = if ((locals.var_rrr_cc < assign13500_e19167) && (assign13500_e19171 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard423 = assign13500_e19174;

        let (assign13510_e19189, assign13510_e19189_d_n0, assign13510_e19189_d_n2, assign13510_e19189_d_n6, assign13510_e19189_d_n7, assign13510_e19189_d_n10, assign13510_e19189_d_n11, assign13510_e19189_d_n12, assign13510_e19189_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) {
        let assign13510_e19184: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13510_e19185: f64 = assign13510_e19184;
        let assign13510_e19187: f64 = (assign13510_e19185 - locals.var_rrr_cc);
        (assign13510_e19187, ((locals.var_q_fd_soi_dn0 * 1e-5) - locals.var_rrr_cc_dn0), ((locals.var_q_fd_soi_dn2 * 1e-5) - locals.var_rrr_cc_dn2), ((locals.var_q_fd_soi_dn6 * 1e-5) - locals.var_rrr_cc_dn6), ((locals.var_q_fd_soi_dn7 * 1e-5) - locals.var_rrr_cc_dn7), ((locals.var_q_fd_soi_dn10 * 1e-5) - locals.var_rrr_cc_dn10), ((locals.var_q_fd_soi_dn11 * 1e-5) - locals.var_rrr_cc_dn11), ((locals.var_q_fd_soi_dn12 * 1e-5) - locals.var_rrr_cc_dn12), ((locals.var_q_fd_soi_dn17 * 1e-5) - locals.var_rrr_cc_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign13510_e19189;
        locals.var_tmf1_dn0 = assign13510_e19189_d_n0;
        locals.var_tmf1_dn2 = assign13510_e19189_d_n2;
        locals.var_tmf1_dn6 = assign13510_e19189_d_n6;
        locals.var_tmf1_dn7 = assign13510_e19189_d_n7;
        locals.var_tmf1_dn10 = assign13510_e19189_d_n10;
        locals.var_tmf1_dn11 = assign13510_e19189_d_n11;
        locals.var_tmf1_dn12 = assign13510_e19189_d_n12;
        locals.var_tmf1_dn17 = assign13510_e19189_d_n17;

        let (assign13520_e19200, assign13520_e19200_d_n0, assign13520_e19200_d_n2, assign13520_e19200_d_n6, assign13520_e19200_d_n7, assign13520_e19200_d_n10, assign13520_e19200_d_n11, assign13520_e19200_d_n12, assign13520_e19200_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) {
        let assign13520_e19198: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign13520_e19198, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign13520_e19200;
        locals.var_x2_dn0 = assign13520_e19200_d_n0;
        locals.var_x2_dn2 = assign13520_e19200_d_n2;
        locals.var_x2_dn6 = assign13520_e19200_d_n6;
        locals.var_x2_dn7 = assign13520_e19200_d_n7;
        locals.var_x2_dn10 = assign13520_e19200_d_n10;
        locals.var_x2_dn11 = assign13520_e19200_d_n11;
        locals.var_x2_dn12 = assign13520_e19200_d_n12;
        locals.var_x2_dn17 = assign13520_e19200_d_n17;

        let (assign13530_e19215, assign13530_e19215_d_n0, assign13530_e19215_d_n2, assign13530_e19215_d_n6, assign13530_e19215_d_n7, assign13530_e19215_d_n10, assign13530_e19215_d_n11, assign13530_e19215_d_n12, assign13530_e19215_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) {
        let assign13530_e19209: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13530_e19212: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13530_e19213: f64 = (assign13530_e19209 * assign13530_e19212);
        (assign13530_e19213, (((locals.var_q_fd_soi_dn0 * 1e-5) * assign13530_e19212) + (assign13530_e19209 * (locals.var_q_fd_soi_dn0 * 1e-5))), (((locals.var_q_fd_soi_dn2 * 1e-5) * assign13530_e19212) + (assign13530_e19209 * (locals.var_q_fd_soi_dn2 * 1e-5))), (((locals.var_q_fd_soi_dn6 * 1e-5) * assign13530_e19212) + (assign13530_e19209 * (locals.var_q_fd_soi_dn6 * 1e-5))), (((locals.var_q_fd_soi_dn7 * 1e-5) * assign13530_e19212) + (assign13530_e19209 * (locals.var_q_fd_soi_dn7 * 1e-5))), (((locals.var_q_fd_soi_dn10 * 1e-5) * assign13530_e19212) + (assign13530_e19209 * (locals.var_q_fd_soi_dn10 * 1e-5))), (((locals.var_q_fd_soi_dn11 * 1e-5) * assign13530_e19212) + (assign13530_e19209 * (locals.var_q_fd_soi_dn11 * 1e-5))), (((locals.var_q_fd_soi_dn12 * 1e-5) * assign13530_e19212) + (assign13530_e19209 * (locals.var_q_fd_soi_dn12 * 1e-5))), (((locals.var_q_fd_soi_dn17 * 1e-5) * assign13530_e19212) + (assign13530_e19209 * (locals.var_q_fd_soi_dn17 * 1e-5))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign13530_e19215;
        locals.var_xmax2_dn0 = assign13530_e19215_d_n0;
        locals.var_xmax2_dn2 = assign13530_e19215_d_n2;
        locals.var_xmax2_dn6 = assign13530_e19215_d_n6;
        locals.var_xmax2_dn7 = assign13530_e19215_d_n7;
        locals.var_xmax2_dn10 = assign13530_e19215_d_n10;
        locals.var_xmax2_dn11 = assign13530_e19215_d_n11;
        locals.var_xmax2_dn12 = assign13530_e19215_d_n12;
        locals.var_xmax2_dn17 = assign13530_e19215_d_n17;

        let (assign13540_e19224, assign13540_e19224_d_n0, assign13540_e19224_d_n2, assign13540_e19224_d_n6, assign13540_e19224_d_n7, assign13540_e19224_d_n10, assign13540_e19224_d_n11, assign13540_e19224_d_n12, assign13540_e19224_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13540_e19224;
        locals.var_xp_dn0 = assign13540_e19224_d_n0;
        locals.var_xp_dn2 = assign13540_e19224_d_n2;
        locals.var_xp_dn6 = assign13540_e19224_d_n6;
        locals.var_xp_dn7 = assign13540_e19224_d_n7;
        locals.var_xp_dn10 = assign13540_e19224_d_n10;
        locals.var_xp_dn11 = assign13540_e19224_d_n11;
        locals.var_xp_dn12 = assign13540_e19224_d_n12;
        locals.var_xp_dn17 = assign13540_e19224_d_n17;

        let (assign13550_e19233, assign13550_e19233_d_n0, assign13550_e19233_d_n2, assign13550_e19233_d_n6, assign13550_e19233_d_n7, assign13550_e19233_d_n10, assign13550_e19233_d_n11, assign13550_e19233_d_n12, assign13550_e19233_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13550_e19233;
        locals.var_xmp_dn0 = assign13550_e19233_d_n0;
        locals.var_xmp_dn2 = assign13550_e19233_d_n2;
        locals.var_xmp_dn6 = assign13550_e19233_d_n6;
        locals.var_xmp_dn7 = assign13550_e19233_d_n7;
        locals.var_xmp_dn10 = assign13550_e19233_d_n10;
        locals.var_xmp_dn11 = assign13550_e19233_d_n11;
        locals.var_xmp_dn12 = assign13550_e19233_d_n12;
        locals.var_xmp_dn17 = assign13550_e19233_d_n17;

        let (assign13560_e19242,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign13560_e19242;

        let (assign13570_e19251,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13570_e19251;

        let (assign13580_e19260, assign13580_e19260_d_n0, assign13580_e19260_d_n2, assign13580_e19260_d_n6, assign13580_e19260_d_n7, assign13580_e19260_d_n10, assign13580_e19260_d_n11, assign13580_e19260_d_n12, assign13580_e19260_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign13580_e19260;
        locals.var_arg_dn0 = assign13580_e19260_d_n0;
        locals.var_arg_dn2 = assign13580_e19260_d_n2;
        locals.var_arg_dn6 = assign13580_e19260_d_n6;
        locals.var_arg_dn7 = assign13580_e19260_d_n7;
        locals.var_arg_dn10 = assign13580_e19260_d_n10;
        locals.var_arg_dn11 = assign13580_e19260_d_n11;
        locals.var_arg_dn12 = assign13580_e19260_d_n12;
        locals.var_arg_dn17 = assign13580_e19260_d_n17;

        let (assign13590_e19269, assign13590_e19269_d_n0, assign13590_e19269_d_n2, assign13590_e19269_d_n6, assign13590_e19269_d_n7, assign13590_e19269_d_n10, assign13590_e19269_d_n11, assign13590_e19269_d_n12, assign13590_e19269_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13590_e19269;
        locals.var_dnm_dn0 = assign13590_e19269_d_n0;
        locals.var_dnm_dn2 = assign13590_e19269_d_n2;
        locals.var_dnm_dn6 = assign13590_e19269_d_n6;
        locals.var_dnm_dn7 = assign13590_e19269_d_n7;
        locals.var_dnm_dn10 = assign13590_e19269_d_n10;
        locals.var_dnm_dn11 = assign13590_e19269_d_n11;
        locals.var_dnm_dn12 = assign13590_e19269_d_n12;
        locals.var_dnm_dn17 = assign13590_e19269_d_n17;

    }

    pub(super) fn stamp_transient_block_44(
        locals: &mut StampLocals,
    ) {
        let (assign13600_e19280, assign13600_e19280_d_n0, assign13600_e19280_d_n2, assign13600_e19280_d_n6, assign13600_e19280_d_n7, assign13600_e19280_d_n10, assign13600_e19280_d_n11, assign13600_e19280_d_n12, assign13600_e19280_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) {
        let assign13600_e19278: f64 = (locals.var_xp * locals.var_x2);
        (assign13600_e19278, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13600_e19280;
        locals.var_xp_dn0 = assign13600_e19280_d_n0;
        locals.var_xp_dn2 = assign13600_e19280_d_n2;
        locals.var_xp_dn6 = assign13600_e19280_d_n6;
        locals.var_xp_dn7 = assign13600_e19280_d_n7;
        locals.var_xp_dn10 = assign13600_e19280_d_n10;
        locals.var_xp_dn11 = assign13600_e19280_d_n11;
        locals.var_xp_dn12 = assign13600_e19280_d_n12;
        locals.var_xp_dn17 = assign13600_e19280_d_n17;

        let (assign13610_e19291, assign13610_e19291_d_n0, assign13610_e19291_d_n2, assign13610_e19291_d_n6, assign13610_e19291_d_n7, assign13610_e19291_d_n10, assign13610_e19291_d_n11, assign13610_e19291_d_n12, assign13610_e19291_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) {
        let assign13610_e19289: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign13610_e19289, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13610_e19291;
        locals.var_xmp_dn0 = assign13610_e19291_d_n0;
        locals.var_xmp_dn2 = assign13610_e19291_d_n2;
        locals.var_xmp_dn6 = assign13610_e19291_d_n6;
        locals.var_xmp_dn7 = assign13610_e19291_d_n7;
        locals.var_xmp_dn10 = assign13610_e19291_d_n10;
        locals.var_xmp_dn11 = assign13610_e19291_d_n11;
        locals.var_xmp_dn12 = assign13610_e19291_d_n12;
        locals.var_xmp_dn17 = assign13610_e19291_d_n17;

        let (assign13620_e19302, assign13620_e19302_d_n0, assign13620_e19302_d_n2, assign13620_e19302_d_n6, assign13620_e19302_d_n7, assign13620_e19302_d_n10, assign13620_e19302_d_n11, assign13620_e19302_d_n12, assign13620_e19302_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) {
        let assign13620_e19300: f64 = (locals.var_xp * locals.var_x2);
        (assign13620_e19300, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13620_e19302;
        locals.var_xp_dn0 = assign13620_e19302_d_n0;
        locals.var_xp_dn2 = assign13620_e19302_d_n2;
        locals.var_xp_dn6 = assign13620_e19302_d_n6;
        locals.var_xp_dn7 = assign13620_e19302_d_n7;
        locals.var_xp_dn10 = assign13620_e19302_d_n10;
        locals.var_xp_dn11 = assign13620_e19302_d_n11;
        locals.var_xp_dn12 = assign13620_e19302_d_n12;
        locals.var_xp_dn17 = assign13620_e19302_d_n17;

        let (assign13630_e19313, assign13630_e19313_d_n0, assign13630_e19313_d_n2, assign13630_e19313_d_n6, assign13630_e19313_d_n7, assign13630_e19313_d_n10, assign13630_e19313_d_n11, assign13630_e19313_d_n12, assign13630_e19313_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) {
        let assign13630_e19311: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign13630_e19311, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13630_e19313;
        locals.var_xmp_dn0 = assign13630_e19313_d_n0;
        locals.var_xmp_dn2 = assign13630_e19313_d_n2;
        locals.var_xmp_dn6 = assign13630_e19313_d_n6;
        locals.var_xmp_dn7 = assign13630_e19313_d_n7;
        locals.var_xmp_dn10 = assign13630_e19313_d_n10;
        locals.var_xmp_dn11 = assign13630_e19313_d_n11;
        locals.var_xmp_dn12 = assign13630_e19313_d_n12;
        locals.var_xmp_dn17 = assign13630_e19313_d_n17;

        let (assign13640_e19324, assign13640_e19324_d_n0, assign13640_e19324_d_n2, assign13640_e19324_d_n6, assign13640_e19324_d_n7, assign13640_e19324_d_n10, assign13640_e19324_d_n11, assign13640_e19324_d_n12, assign13640_e19324_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) {
        let assign13640_e19322: f64 = (locals.var_xp + locals.var_xmp);
        (assign13640_e19322, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign13640_e19324;
        locals.var_arg_dn0 = assign13640_e19324_d_n0;
        locals.var_arg_dn2 = assign13640_e19324_d_n2;
        locals.var_arg_dn6 = assign13640_e19324_d_n6;
        locals.var_arg_dn7 = assign13640_e19324_d_n7;
        locals.var_arg_dn10 = assign13640_e19324_d_n10;
        locals.var_arg_dn11 = assign13640_e19324_d_n11;
        locals.var_arg_dn12 = assign13640_e19324_d_n12;
        locals.var_arg_dn17 = assign13640_e19324_d_n17;

        let (assign13650_e19333, assign13650_e19333_d_n0, assign13650_e19333_d_n2, assign13650_e19333_d_n6, assign13650_e19333_d_n7, assign13650_e19333_d_n10, assign13650_e19333_d_n11, assign13650_e19333_d_n12, assign13650_e19333_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13650_e19333;
        locals.var_dnm_dn0 = assign13650_e19333_d_n0;
        locals.var_dnm_dn2 = assign13650_e19333_d_n2;
        locals.var_dnm_dn6 = assign13650_e19333_d_n6;
        locals.var_dnm_dn7 = assign13650_e19333_d_n7;
        locals.var_dnm_dn10 = assign13650_e19333_d_n10;
        locals.var_dnm_dn11 = assign13650_e19333_d_n11;
        locals.var_dnm_dn12 = assign13650_e19333_d_n12;
        locals.var_dnm_dn17 = assign13650_e19333_d_n17;

        let assign13660_e19348: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard424 = assign13660_e19348;

        let assign13670_e19351: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard425 = assign13670_e19351;

        let (assign13680_e19364,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) && (locals.var_guard424 != 0.0)) && (locals.var_guard425 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13680_e19364;

        let assign13690_e19367: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard426 = assign13690_e19367;

        let (assign13700_e19383,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) && (locals.var_guard424 != 0.0)) && (locals.var_guard425 == 0.0)) && (locals.var_guard426 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13700_e19383;

        let assign13710_e19386: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard427 = assign13710_e19386;

        let (assign13720_e19405,) = {
    if (((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) && (locals.var_guard424 != 0.0)) && (locals.var_guard425 == 0.0)) && (locals.var_guard426 == 0.0)) && (locals.var_guard427 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13720_e19405;

        let assign13730_e19408: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard428 = assign13730_e19408;

        let (assign13740_e19430,) = {
    if ((((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) && (locals.var_guard424 != 0.0)) && (locals.var_guard425 == 0.0)) && (locals.var_guard426 == 0.0)) && (locals.var_guard427 == 0.0)) && (locals.var_guard428 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13740_e19430;

        let (assign13750_e19441,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) && (locals.var_guard424 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign13750_e19441;

        let mut assign13760_loop_guard: usize = 0;
        while {
            let assign13760_cond_e19453: f64 = if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) && (locals.var_guard424 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign13760_cond_e19453 != 0.0
        } {
            assign13760_loop_guard += 1;
            assert!(assign13760_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign13760_body0_e19465, assign13760_body0_e19465_d_n0, assign13760_body0_e19465_d_n2, assign13760_body0_e19465_d_n6, assign13760_body0_e19465_d_n7, assign13760_body0_e19465_d_n10, assign13760_body0_e19465_d_n11, assign13760_body0_e19465_d_n12, assign13760_body0_e19465_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) && (locals.var_guard424 != 0.0)) {
        let assign13760_body0_e19463: f64 = (locals.var_dnm).sqrt();
        (assign13760_body0_e19463, (locals.var_dnm_dn0 / (2.0 * assign13760_body0_e19463)), (locals.var_dnm_dn2 / (2.0 * assign13760_body0_e19463)), (locals.var_dnm_dn6 / (2.0 * assign13760_body0_e19463)), (locals.var_dnm_dn7 / (2.0 * assign13760_body0_e19463)), (locals.var_dnm_dn10 / (2.0 * assign13760_body0_e19463)), (locals.var_dnm_dn11 / (2.0 * assign13760_body0_e19463)), (locals.var_dnm_dn12 / (2.0 * assign13760_body0_e19463)), (locals.var_dnm_dn17 / (2.0 * assign13760_body0_e19463)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign13760_body0_e19465;
            locals.var_dnm_dn0 = assign13760_body0_e19465_d_n0;
            locals.var_dnm_dn2 = assign13760_body0_e19465_d_n2;
            locals.var_dnm_dn6 = assign13760_body0_e19465_d_n6;
            locals.var_dnm_dn7 = assign13760_body0_e19465_d_n7;
            locals.var_dnm_dn10 = assign13760_body0_e19465_d_n10;
            locals.var_dnm_dn11 = assign13760_body0_e19465_d_n11;
            locals.var_dnm_dn12 = assign13760_body0_e19465_d_n12;
            locals.var_dnm_dn17 = assign13760_body0_e19465_d_n17;
            let (assign13760_body1_e19478,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) && (locals.var_guard424 != 0.0)) {
        let assign13760_body1_e19476: f64 = (locals.var_m0 + 1.0);
        (assign13760_body1_e19476,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign13760_body1_e19478;
        }

        let (assign13770_e19496, assign13770_e19496_d_n0, assign13770_e19496_d_n2, assign13770_e19496_d_n6, assign13770_e19496_d_n7, assign13770_e19496_d_n10, assign13770_e19496_d_n11, assign13770_e19496_d_n12, assign13770_e19496_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) && (locals.var_guard424 == 0.0)) {
        let assign13770_e19492: f64 = (2.0 * 2.0);
        let assign13770_e19493: f64 = (1.0 / assign13770_e19492);
        let assign13770_e19494: f64 = (locals.var_dnm).powf(assign13770_e19493);
        (assign13770_e19494, if 0.0 == 0.0 && ((assign13770_e19493) as f64).is_finite() && ((assign13770_e19493) as f64).fract() == 0.0 { if assign13770_e19493 == 0.0 { 0.0 } else { (assign13770_e19493 * ((locals.var_dnm).powf(assign13770_e19493 - 1.0) * locals.var_dnm_dn0)) } } else { (assign13770_e19494 * (assign13770_e19493 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13770_e19493) as f64).is_finite() && ((assign13770_e19493) as f64).fract() == 0.0 { if assign13770_e19493 == 0.0 { 0.0 } else { (assign13770_e19493 * ((locals.var_dnm).powf(assign13770_e19493 - 1.0) * locals.var_dnm_dn2)) } } else { (assign13770_e19494 * (assign13770_e19493 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13770_e19493) as f64).is_finite() && ((assign13770_e19493) as f64).fract() == 0.0 { if assign13770_e19493 == 0.0 { 0.0 } else { (assign13770_e19493 * ((locals.var_dnm).powf(assign13770_e19493 - 1.0) * locals.var_dnm_dn6)) } } else { (assign13770_e19494 * (assign13770_e19493 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13770_e19493) as f64).is_finite() && ((assign13770_e19493) as f64).fract() == 0.0 { if assign13770_e19493 == 0.0 { 0.0 } else { (assign13770_e19493 * ((locals.var_dnm).powf(assign13770_e19493 - 1.0) * locals.var_dnm_dn7)) } } else { (assign13770_e19494 * (assign13770_e19493 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13770_e19493) as f64).is_finite() && ((assign13770_e19493) as f64).fract() == 0.0 { if assign13770_e19493 == 0.0 { 0.0 } else { (assign13770_e19493 * ((locals.var_dnm).powf(assign13770_e19493 - 1.0) * locals.var_dnm_dn10)) } } else { (assign13770_e19494 * (assign13770_e19493 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13770_e19493) as f64).is_finite() && ((assign13770_e19493) as f64).fract() == 0.0 { if assign13770_e19493 == 0.0 { 0.0 } else { (assign13770_e19493 * ((locals.var_dnm).powf(assign13770_e19493 - 1.0) * locals.var_dnm_dn11)) } } else { (assign13770_e19494 * (assign13770_e19493 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13770_e19493) as f64).is_finite() && ((assign13770_e19493) as f64).fract() == 0.0 { if assign13770_e19493 == 0.0 { 0.0 } else { (assign13770_e19493 * ((locals.var_dnm).powf(assign13770_e19493 - 1.0) * locals.var_dnm_dn12)) } } else { (assign13770_e19494 * (assign13770_e19493 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign13770_e19493) as f64).is_finite() && ((assign13770_e19493) as f64).fract() == 0.0 { if assign13770_e19493 == 0.0 { 0.0 } else { (assign13770_e19493 * ((locals.var_dnm).powf(assign13770_e19493 - 1.0) * locals.var_dnm_dn17)) } } else { (assign13770_e19494 * (assign13770_e19493 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13770_e19496;
        locals.var_dnm_dn0 = assign13770_e19496_d_n0;
        locals.var_dnm_dn2 = assign13770_e19496_d_n2;
        locals.var_dnm_dn6 = assign13770_e19496_d_n6;
        locals.var_dnm_dn7 = assign13770_e19496_d_n7;
        locals.var_dnm_dn10 = assign13770_e19496_d_n10;
        locals.var_dnm_dn11 = assign13770_e19496_d_n11;
        locals.var_dnm_dn12 = assign13770_e19496_d_n12;
        locals.var_dnm_dn17 = assign13770_e19496_d_n17;

        let (assign13780_e19507, assign13780_e19507_d_n0, assign13780_e19507_d_n2, assign13780_e19507_d_n6, assign13780_e19507_d_n7, assign13780_e19507_d_n10, assign13780_e19507_d_n11, assign13780_e19507_d_n12, assign13780_e19507_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) {
        let assign13780_e19505: f64 = (1.0 / locals.var_dnm);
        (assign13780_e19505, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13780_e19507;
        locals.var_dnm_dn0 = assign13780_e19507_d_n0;
        locals.var_dnm_dn2 = assign13780_e19507_d_n2;
        locals.var_dnm_dn6 = assign13780_e19507_d_n6;
        locals.var_dnm_dn7 = assign13780_e19507_d_n7;
        locals.var_dnm_dn10 = assign13780_e19507_d_n10;
        locals.var_dnm_dn11 = assign13780_e19507_d_n11;
        locals.var_dnm_dn12 = assign13780_e19507_d_n12;
        locals.var_dnm_dn17 = assign13780_e19507_d_n17;

        let (assign13790_e19522, assign13790_e19522_d_n0, assign13790_e19522_d_n2, assign13790_e19522_d_n6, assign13790_e19522_d_n7, assign13790_e19522_d_n10, assign13790_e19522_d_n11, assign13790_e19522_d_n12, assign13790_e19522_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) {
        let assign13790_e19517: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13790_e19518: f64 = (locals.var_tmf1 * assign13790_e19517);
        let assign13790_e19520: f64 = (assign13790_e19518 * locals.var_dnm);
        (assign13790_e19520, ((((locals.var_tmf1_dn0 * assign13790_e19517) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn0 * 1e-5))) * locals.var_dnm) + (assign13790_e19518 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign13790_e19517) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn2 * 1e-5))) * locals.var_dnm) + (assign13790_e19518 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn6 * assign13790_e19517) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn6 * 1e-5))) * locals.var_dnm) + (assign13790_e19518 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign13790_e19517) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn7 * 1e-5))) * locals.var_dnm) + (assign13790_e19518 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn10 * assign13790_e19517) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn10 * 1e-5))) * locals.var_dnm) + (assign13790_e19518 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * assign13790_e19517) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn11 * 1e-5))) * locals.var_dnm) + (assign13790_e19518 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn12 * assign13790_e19517) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn12 * 1e-5))) * locals.var_dnm) + (assign13790_e19518 * locals.var_dnm_dn12)), ((((locals.var_tmf1_dn17 * assign13790_e19517) + (locals.var_tmf1 * (locals.var_q_fd_soi_dn17 * 1e-5))) * locals.var_dnm) + (assign13790_e19518 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign13790_e19522;
        locals.var_tmf0_dn0 = assign13790_e19522_d_n0;
        locals.var_tmf0_dn2 = assign13790_e19522_d_n2;
        locals.var_tmf0_dn6 = assign13790_e19522_d_n6;
        locals.var_tmf0_dn7 = assign13790_e19522_d_n7;
        locals.var_tmf0_dn10 = assign13790_e19522_d_n10;
        locals.var_tmf0_dn11 = assign13790_e19522_d_n11;
        locals.var_tmf0_dn12 = assign13790_e19522_d_n12;
        locals.var_tmf0_dn17 = assign13790_e19522_d_n17;

        let (assign13800_e19537, assign13800_e19537_d_n0, assign13800_e19537_d_n2, assign13800_e19537_d_n6, assign13800_e19537_d_n7, assign13800_e19537_d_n10, assign13800_e19537_d_n11, assign13800_e19537_d_n12, assign13800_e19537_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 != 0.0)) {
        let assign13800_e19532: f64 = (locals.var_q_fd_soi * 1e-5);
        let assign13800_e19533: f64 = assign13800_e19532;
        let assign13800_e19535: f64 = (assign13800_e19533 - locals.var_tmf0);
        (assign13800_e19535, ((locals.var_q_fd_soi_dn0 * 1e-5) - locals.var_tmf0_dn0), ((locals.var_q_fd_soi_dn2 * 1e-5) - locals.var_tmf0_dn2), ((locals.var_q_fd_soi_dn6 * 1e-5) - locals.var_tmf0_dn6), ((locals.var_q_fd_soi_dn7 * 1e-5) - locals.var_tmf0_dn7), ((locals.var_q_fd_soi_dn10 * 1e-5) - locals.var_tmf0_dn10), ((locals.var_q_fd_soi_dn11 * 1e-5) - locals.var_tmf0_dn11), ((locals.var_q_fd_soi_dn12 * 1e-5) - locals.var_tmf0_dn12), ((locals.var_q_fd_soi_dn17 * 1e-5) - locals.var_tmf0_dn17),)
    } else {
        (locals.var_rrr_cc, locals.var_rrr_cc_dn0, locals.var_rrr_cc_dn2, locals.var_rrr_cc_dn6, locals.var_rrr_cc_dn7, locals.var_rrr_cc_dn10, locals.var_rrr_cc_dn11, locals.var_rrr_cc_dn12, locals.var_rrr_cc_dn17,)
    }
};
        locals.var_rrr_cc = assign13800_e19537;
        locals.var_rrr_cc_dn0 = assign13800_e19537_d_n0;
        locals.var_rrr_cc_dn2 = assign13800_e19537_d_n2;
        locals.var_rrr_cc_dn6 = assign13800_e19537_d_n6;
        locals.var_rrr_cc_dn7 = assign13800_e19537_d_n7;
        locals.var_rrr_cc_dn10 = assign13800_e19537_d_n10;
        locals.var_rrr_cc_dn11 = assign13800_e19537_d_n11;
        locals.var_rrr_cc_dn12 = assign13800_e19537_d_n12;
        locals.var_rrr_cc_dn17 = assign13800_e19537_d_n17;

        let (assign13810_e19547, assign13810_e19547_d_n0, assign13810_e19547_d_n2, assign13810_e19547_d_n6, assign13810_e19547_d_n7, assign13810_e19547_d_n10, assign13810_e19547_d_n11, assign13810_e19547_d_n12, assign13810_e19547_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard423 == 0.0)) {
        (locals.var_rrr_cc, locals.var_rrr_cc_dn0, locals.var_rrr_cc_dn2, locals.var_rrr_cc_dn6, locals.var_rrr_cc_dn7, locals.var_rrr_cc_dn10, locals.var_rrr_cc_dn11, locals.var_rrr_cc_dn12, locals.var_rrr_cc_dn17,)
    } else {
        (locals.var_rrr_cc, locals.var_rrr_cc_dn0, locals.var_rrr_cc_dn2, locals.var_rrr_cc_dn6, locals.var_rrr_cc_dn7, locals.var_rrr_cc_dn10, locals.var_rrr_cc_dn11, locals.var_rrr_cc_dn12, locals.var_rrr_cc_dn17,)
    }
};
        locals.var_rrr_cc = assign13810_e19547;
        locals.var_rrr_cc_dn0 = assign13810_e19547_d_n0;
        locals.var_rrr_cc_dn2 = assign13810_e19547_d_n2;
        locals.var_rrr_cc_dn6 = assign13810_e19547_d_n6;
        locals.var_rrr_cc_dn7 = assign13810_e19547_d_n7;
        locals.var_rrr_cc_dn10 = assign13810_e19547_d_n10;
        locals.var_rrr_cc_dn11 = assign13810_e19547_d_n11;
        locals.var_rrr_cc_dn12 = assign13810_e19547_d_n12;
        locals.var_rrr_cc_dn17 = assign13810_e19547_d_n17;

        let (assign13820_e19567, assign13820_e19567_d_n0, assign13820_e19567_d_n2, assign13820_e19567_d_n6, assign13820_e19567_d_n7, assign13820_e19567_d_n10, assign13820_e19567_d_n11, assign13820_e19567_d_n12, assign13820_e19567_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign13820_e19555: f64 = (-locals.var_rrr_cc);
        let assign13820_e19556: f64 = (2.0 * assign13820_e19555);
        let assign13820_e19559: f64 = (locals.var_beta * locals.var_c_fox);
        let assign13820_e19561: f64 = (assign13820_e19559 * locals.var_rrr_p0);
        let assign13820_e19563: f64 = (assign13820_e19561 * locals.var_rrr_p0);
        let assign13820_e19564: f64 = (assign13820_e19556 / assign13820_e19563);
        let assign13820_e19565: f64 = (1.0 + assign13820_e19564);
        (assign13820_e19565, ((((2.0 * (-locals.var_rrr_cc_dn0)) * assign13820_e19563) - (assign13820_e19556 * (((((locals.var_beta * locals.var_c_fox_dn0) * locals.var_rrr_p0) + (assign13820_e19559 * locals.var_rrr_p0_dn0)) * locals.var_rrr_p0) + (assign13820_e19561 * locals.var_rrr_p0_dn0)))) / (assign13820_e19563 * assign13820_e19563)), ((((2.0 * (-locals.var_rrr_cc_dn2)) * assign13820_e19563) - (assign13820_e19556 * (((((locals.var_beta * locals.var_c_fox_dn2) * locals.var_rrr_p0) + (assign13820_e19559 * locals.var_rrr_p0_dn2)) * locals.var_rrr_p0) + (assign13820_e19561 * locals.var_rrr_p0_dn2)))) / (assign13820_e19563 * assign13820_e19563)), ((((2.0 * (-locals.var_rrr_cc_dn6)) * assign13820_e19563) - (assign13820_e19556 * (((((locals.var_beta * locals.var_c_fox_dn6) * locals.var_rrr_p0) + (assign13820_e19559 * locals.var_rrr_p0_dn6)) * locals.var_rrr_p0) + (assign13820_e19561 * locals.var_rrr_p0_dn6)))) / (assign13820_e19563 * assign13820_e19563)), ((((2.0 * (-locals.var_rrr_cc_dn7)) * assign13820_e19563) - (assign13820_e19556 * (((((locals.var_beta * locals.var_c_fox_dn7) * locals.var_rrr_p0) + (assign13820_e19559 * locals.var_rrr_p0_dn7)) * locals.var_rrr_p0) + (assign13820_e19561 * locals.var_rrr_p0_dn7)))) / (assign13820_e19563 * assign13820_e19563)), ((((2.0 * (-locals.var_rrr_cc_dn10)) * assign13820_e19563) - (assign13820_e19556 * ((((((locals.var_beta_dn10 * locals.var_c_fox) + (locals.var_beta * locals.var_c_fox_dn10)) * locals.var_rrr_p0) + (assign13820_e19559 * locals.var_rrr_p0_dn10)) * locals.var_rrr_p0) + (assign13820_e19561 * locals.var_rrr_p0_dn10)))) / (assign13820_e19563 * assign13820_e19563)), ((((2.0 * (-locals.var_rrr_cc_dn11)) * assign13820_e19563) - (assign13820_e19556 * (((((locals.var_beta * locals.var_c_fox_dn11) * locals.var_rrr_p0) + (assign13820_e19559 * locals.var_rrr_p0_dn11)) * locals.var_rrr_p0) + (assign13820_e19561 * locals.var_rrr_p0_dn11)))) / (assign13820_e19563 * assign13820_e19563)), ((((2.0 * (-locals.var_rrr_cc_dn12)) * assign13820_e19563) - (assign13820_e19556 * (((((locals.var_beta * locals.var_c_fox_dn12) * locals.var_rrr_p0) + (assign13820_e19559 * locals.var_rrr_p0_dn12)) * locals.var_rrr_p0) + (assign13820_e19561 * locals.var_rrr_p0_dn12)))) / (assign13820_e19563 * assign13820_e19563)), ((((2.0 * (-locals.var_rrr_cc_dn17)) * assign13820_e19563) - (assign13820_e19556 * (((((locals.var_beta * locals.var_c_fox_dn17) * locals.var_rrr_p0) + (assign13820_e19559 * locals.var_rrr_p0_dn17)) * locals.var_rrr_p0) + (assign13820_e19561 * locals.var_rrr_p0_dn17)))) / (assign13820_e19563 * assign13820_e19563)),)
    } else {
        (locals.var_rrr_alpha_soi, locals.var_rrr_alpha_soi_dn0, locals.var_rrr_alpha_soi_dn2, locals.var_rrr_alpha_soi_dn6, locals.var_rrr_alpha_soi_dn7, locals.var_rrr_alpha_soi_dn10, locals.var_rrr_alpha_soi_dn11, locals.var_rrr_alpha_soi_dn12, locals.var_rrr_alpha_soi_dn17,)
    }
};
        locals.var_rrr_alpha_soi = assign13820_e19567;
        locals.var_rrr_alpha_soi_dn0 = assign13820_e19567_d_n0;
        locals.var_rrr_alpha_soi_dn2 = assign13820_e19567_d_n2;
        locals.var_rrr_alpha_soi_dn6 = assign13820_e19567_d_n6;
        locals.var_rrr_alpha_soi_dn7 = assign13820_e19567_d_n7;
        locals.var_rrr_alpha_soi_dn10 = assign13820_e19567_d_n10;
        locals.var_rrr_alpha_soi_dn11 = assign13820_e19567_d_n11;
        locals.var_rrr_alpha_soi_dn12 = assign13820_e19567_d_n12;
        locals.var_rrr_alpha_soi_dn17 = assign13820_e19567_d_n17;

        let (assign13830_e19580, assign13830_e19580_d_n0, assign13830_e19580_d_n2, assign13830_e19580_d_n6, assign13830_e19580_d_n7, assign13830_e19580_d_n10, assign13830_e19580_d_n11, assign13830_e19580_d_n12, assign13830_e19580_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign13830_e19574: f64 = (locals.var_rrr_p0 * locals.var_rrr_p0);
        let assign13830_e19576: f64 = (assign13830_e19574 * locals.var_rrr_p0);
        let assign13830_e19578: f64 = (assign13830_e19576 * locals.var_rrr_p0);
        (assign13830_e19578, ((((((locals.var_rrr_p0_dn0 * locals.var_rrr_p0) + (locals.var_rrr_p0 * locals.var_rrr_p0_dn0)) * locals.var_rrr_p0) + (assign13830_e19574 * locals.var_rrr_p0_dn0)) * locals.var_rrr_p0) + (assign13830_e19576 * locals.var_rrr_p0_dn0)), ((((((locals.var_rrr_p0_dn2 * locals.var_rrr_p0) + (locals.var_rrr_p0 * locals.var_rrr_p0_dn2)) * locals.var_rrr_p0) + (assign13830_e19574 * locals.var_rrr_p0_dn2)) * locals.var_rrr_p0) + (assign13830_e19576 * locals.var_rrr_p0_dn2)), ((((((locals.var_rrr_p0_dn6 * locals.var_rrr_p0) + (locals.var_rrr_p0 * locals.var_rrr_p0_dn6)) * locals.var_rrr_p0) + (assign13830_e19574 * locals.var_rrr_p0_dn6)) * locals.var_rrr_p0) + (assign13830_e19576 * locals.var_rrr_p0_dn6)), ((((((locals.var_rrr_p0_dn7 * locals.var_rrr_p0) + (locals.var_rrr_p0 * locals.var_rrr_p0_dn7)) * locals.var_rrr_p0) + (assign13830_e19574 * locals.var_rrr_p0_dn7)) * locals.var_rrr_p0) + (assign13830_e19576 * locals.var_rrr_p0_dn7)), ((((((locals.var_rrr_p0_dn10 * locals.var_rrr_p0) + (locals.var_rrr_p0 * locals.var_rrr_p0_dn10)) * locals.var_rrr_p0) + (assign13830_e19574 * locals.var_rrr_p0_dn10)) * locals.var_rrr_p0) + (assign13830_e19576 * locals.var_rrr_p0_dn10)), ((((((locals.var_rrr_p0_dn11 * locals.var_rrr_p0) + (locals.var_rrr_p0 * locals.var_rrr_p0_dn11)) * locals.var_rrr_p0) + (assign13830_e19574 * locals.var_rrr_p0_dn11)) * locals.var_rrr_p0) + (assign13830_e19576 * locals.var_rrr_p0_dn11)), ((((((locals.var_rrr_p0_dn12 * locals.var_rrr_p0) + (locals.var_rrr_p0 * locals.var_rrr_p0_dn12)) * locals.var_rrr_p0) + (assign13830_e19574 * locals.var_rrr_p0_dn12)) * locals.var_rrr_p0) + (assign13830_e19576 * locals.var_rrr_p0_dn12)), ((((((locals.var_rrr_p0_dn17 * locals.var_rrr_p0) + (locals.var_rrr_p0 * locals.var_rrr_p0_dn17)) * locals.var_rrr_p0) + (assign13830_e19574 * locals.var_rrr_p0_dn17)) * locals.var_rrr_p0) + (assign13830_e19576 * locals.var_rrr_p0_dn17)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign13830_e19580;
        locals.var_t1_dn0 = assign13830_e19580_d_n0;
        locals.var_t1_dn2 = assign13830_e19580_d_n2;
        locals.var_t1_dn6 = assign13830_e19580_d_n6;
        locals.var_t1_dn7 = assign13830_e19580_d_n7;
        locals.var_t1_dn10 = assign13830_e19580_d_n10;
        locals.var_t1_dn11 = assign13830_e19580_d_n11;
        locals.var_t1_dn12 = assign13830_e19580_d_n12;
        locals.var_t1_dn17 = assign13830_e19580_d_n17;

        let (assign13840_e19589, assign13840_e19589_d_n0, assign13840_e19589_d_n2, assign13840_e19589_d_n6, assign13840_e19589_d_n7, assign13840_e19589_d_n10, assign13840_e19589_d_n11, assign13840_e19589_d_n12, assign13840_e19589_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign13840_e19587: f64 = (locals.var_rrr_alpha_soi * locals.var_rrr_p0);
        (assign13840_e19587, ((locals.var_rrr_alpha_soi_dn0 * locals.var_rrr_p0) + (locals.var_rrr_alpha_soi * locals.var_rrr_p0_dn0)), ((locals.var_rrr_alpha_soi_dn2 * locals.var_rrr_p0) + (locals.var_rrr_alpha_soi * locals.var_rrr_p0_dn2)), ((locals.var_rrr_alpha_soi_dn6 * locals.var_rrr_p0) + (locals.var_rrr_alpha_soi * locals.var_rrr_p0_dn6)), ((locals.var_rrr_alpha_soi_dn7 * locals.var_rrr_p0) + (locals.var_rrr_alpha_soi * locals.var_rrr_p0_dn7)), ((locals.var_rrr_alpha_soi_dn10 * locals.var_rrr_p0) + (locals.var_rrr_alpha_soi * locals.var_rrr_p0_dn10)), ((locals.var_rrr_alpha_soi_dn11 * locals.var_rrr_p0) + (locals.var_rrr_alpha_soi * locals.var_rrr_p0_dn11)), ((locals.var_rrr_alpha_soi_dn12 * locals.var_rrr_p0) + (locals.var_rrr_alpha_soi * locals.var_rrr_p0_dn12)), ((locals.var_rrr_alpha_soi_dn17 * locals.var_rrr_p0) + (locals.var_rrr_alpha_soi * locals.var_rrr_p0_dn17)),)
    } else {
        (locals.var_rrr_dd, locals.var_rrr_dd_dn0, locals.var_rrr_dd_dn2, locals.var_rrr_dd_dn6, locals.var_rrr_dd_dn7, locals.var_rrr_dd_dn10, locals.var_rrr_dd_dn11, locals.var_rrr_dd_dn12, locals.var_rrr_dd_dn17,)
    }
};
        locals.var_rrr_dd = assign13840_e19589;
        locals.var_rrr_dd_dn0 = assign13840_e19589_d_n0;
        locals.var_rrr_dd_dn2 = assign13840_e19589_d_n2;
        locals.var_rrr_dd_dn6 = assign13840_e19589_d_n6;
        locals.var_rrr_dd_dn7 = assign13840_e19589_d_n7;
        locals.var_rrr_dd_dn10 = assign13840_e19589_d_n10;
        locals.var_rrr_dd_dn11 = assign13840_e19589_d_n11;
        locals.var_rrr_dd_dn12 = assign13840_e19589_d_n12;
        locals.var_rrr_dd_dn17 = assign13840_e19589_d_n17;

        let (assign13850_e19600, assign13850_e19600_d_n0, assign13850_e19600_d_n2, assign13850_e19600_d_n6, assign13850_e19600_d_n7, assign13850_e19600_d_n10, assign13850_e19600_d_n11, assign13850_e19600_d_n12, assign13850_e19600_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign13850_e19597: f64 = (locals.var_rrr_dd / locals.var_vgvt);
        let assign13850_e19598: f64 = (1.0 - assign13850_e19597);
        (assign13850_e19598, (-(((locals.var_rrr_dd_dn0 * locals.var_vgvt) - (locals.var_rrr_dd * locals.var_vgvt_dn0)) / (locals.var_vgvt * locals.var_vgvt))), (-(((locals.var_rrr_dd_dn2 * locals.var_vgvt) - (locals.var_rrr_dd * locals.var_vgvt_dn2)) / (locals.var_vgvt * locals.var_vgvt))), (-(((locals.var_rrr_dd_dn6 * locals.var_vgvt) - (locals.var_rrr_dd * locals.var_vgvt_dn6)) / (locals.var_vgvt * locals.var_vgvt))), (-(((locals.var_rrr_dd_dn7 * locals.var_vgvt) - (locals.var_rrr_dd * locals.var_vgvt_dn7)) / (locals.var_vgvt * locals.var_vgvt))), (-(((locals.var_rrr_dd_dn10 * locals.var_vgvt) - (locals.var_rrr_dd * locals.var_vgvt_dn10)) / (locals.var_vgvt * locals.var_vgvt))), (-(((locals.var_rrr_dd_dn11 * locals.var_vgvt) - (locals.var_rrr_dd * locals.var_vgvt_dn11)) / (locals.var_vgvt * locals.var_vgvt))), (-(((locals.var_rrr_dd_dn12 * locals.var_vgvt) - (locals.var_rrr_dd * locals.var_vgvt_dn12)) / (locals.var_vgvt * locals.var_vgvt))), (-(((locals.var_rrr_dd_dn17 * locals.var_vgvt) - (locals.var_rrr_dd * locals.var_vgvt_dn17)) / (locals.var_vgvt * locals.var_vgvt))),)
    } else {
        (locals.var_rrr_eta, locals.var_rrr_eta_dn0, locals.var_rrr_eta_dn2, locals.var_rrr_eta_dn6, locals.var_rrr_eta_dn7, locals.var_rrr_eta_dn10, locals.var_rrr_eta_dn11, locals.var_rrr_eta_dn12, locals.var_rrr_eta_dn17,)
    }
};
        locals.var_rrr_eta = assign13850_e19600;
        locals.var_rrr_eta_dn0 = assign13850_e19600_d_n0;
        locals.var_rrr_eta_dn2 = assign13850_e19600_d_n2;
        locals.var_rrr_eta_dn6 = assign13850_e19600_d_n6;
        locals.var_rrr_eta_dn7 = assign13850_e19600_d_n7;
        locals.var_rrr_eta_dn10 = assign13850_e19600_d_n10;
        locals.var_rrr_eta_dn11 = assign13850_e19600_d_n11;
        locals.var_rrr_eta_dn12 = assign13850_e19600_d_n12;
        locals.var_rrr_eta_dn17 = assign13850_e19600_d_n17;

        let assign13860_e19604: f64 = 1e-5;
        let assign13860_e19609: f64 = if ((locals.var_rrr_eta < assign13860_e19604) && (1e-5 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard429 = assign13860_e19609;

        let (assign13870_e19622, assign13870_e19622_d_n0, assign13870_e19622_d_n2, assign13870_e19622_d_n6, assign13870_e19622_d_n7, assign13870_e19622_d_n10, assign13870_e19622_d_n11, assign13870_e19622_d_n12, assign13870_e19622_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        let assign13870_e19618: f64 = 1e-5;
        let assign13870_e19620: f64 = (assign13870_e19618 - locals.var_rrr_eta);
        (assign13870_e19620, (-locals.var_rrr_eta_dn0), (-locals.var_rrr_eta_dn2), (-locals.var_rrr_eta_dn6), (-locals.var_rrr_eta_dn7), (-locals.var_rrr_eta_dn10), (-locals.var_rrr_eta_dn11), (-locals.var_rrr_eta_dn12), (-locals.var_rrr_eta_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign13870_e19622;
        locals.var_tmf1_dn0 = assign13870_e19622_d_n0;
        locals.var_tmf1_dn2 = assign13870_e19622_d_n2;
        locals.var_tmf1_dn6 = assign13870_e19622_d_n6;
        locals.var_tmf1_dn7 = assign13870_e19622_d_n7;
        locals.var_tmf1_dn10 = assign13870_e19622_d_n10;
        locals.var_tmf1_dn11 = assign13870_e19622_d_n11;
        locals.var_tmf1_dn12 = assign13870_e19622_d_n12;
        locals.var_tmf1_dn17 = assign13870_e19622_d_n17;

        let (assign13880_e19633, assign13880_e19633_d_n0, assign13880_e19633_d_n2, assign13880_e19633_d_n6, assign13880_e19633_d_n7, assign13880_e19633_d_n10, assign13880_e19633_d_n11, assign13880_e19633_d_n12, assign13880_e19633_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        let assign13880_e19631: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign13880_e19631, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign13880_e19633;
        locals.var_x2_dn0 = assign13880_e19633_d_n0;
        locals.var_x2_dn2 = assign13880_e19633_d_n2;
        locals.var_x2_dn6 = assign13880_e19633_d_n6;
        locals.var_x2_dn7 = assign13880_e19633_d_n7;
        locals.var_x2_dn10 = assign13880_e19633_d_n10;
        locals.var_x2_dn11 = assign13880_e19633_d_n11;
        locals.var_x2_dn12 = assign13880_e19633_d_n12;
        locals.var_x2_dn17 = assign13880_e19633_d_n17;

        let (assign13890_e19644, assign13890_e19644_d_n0, assign13890_e19644_d_n2, assign13890_e19644_d_n6, assign13890_e19644_d_n7, assign13890_e19644_d_n10, assign13890_e19644_d_n11, assign13890_e19644_d_n12, assign13890_e19644_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        let assign13890_e19642: f64 = (1e-5 * 1e-5);
        (assign13890_e19642, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign13890_e19644;
        locals.var_xmax2_dn0 = assign13890_e19644_d_n0;
        locals.var_xmax2_dn2 = assign13890_e19644_d_n2;
        locals.var_xmax2_dn6 = assign13890_e19644_d_n6;
        locals.var_xmax2_dn7 = assign13890_e19644_d_n7;
        locals.var_xmax2_dn10 = assign13890_e19644_d_n10;
        locals.var_xmax2_dn11 = assign13890_e19644_d_n11;
        locals.var_xmax2_dn12 = assign13890_e19644_d_n12;
        locals.var_xmax2_dn17 = assign13890_e19644_d_n17;

        let (assign13900_e19653, assign13900_e19653_d_n0, assign13900_e19653_d_n2, assign13900_e19653_d_n6, assign13900_e19653_d_n7, assign13900_e19653_d_n10, assign13900_e19653_d_n11, assign13900_e19653_d_n12, assign13900_e19653_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13900_e19653;
        locals.var_xp_dn0 = assign13900_e19653_d_n0;
        locals.var_xp_dn2 = assign13900_e19653_d_n2;
        locals.var_xp_dn6 = assign13900_e19653_d_n6;
        locals.var_xp_dn7 = assign13900_e19653_d_n7;
        locals.var_xp_dn10 = assign13900_e19653_d_n10;
        locals.var_xp_dn11 = assign13900_e19653_d_n11;
        locals.var_xp_dn12 = assign13900_e19653_d_n12;
        locals.var_xp_dn17 = assign13900_e19653_d_n17;

        let (assign13910_e19662, assign13910_e19662_d_n0, assign13910_e19662_d_n2, assign13910_e19662_d_n6, assign13910_e19662_d_n7, assign13910_e19662_d_n10, assign13910_e19662_d_n11, assign13910_e19662_d_n12, assign13910_e19662_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13910_e19662;
        locals.var_xmp_dn0 = assign13910_e19662_d_n0;
        locals.var_xmp_dn2 = assign13910_e19662_d_n2;
        locals.var_xmp_dn6 = assign13910_e19662_d_n6;
        locals.var_xmp_dn7 = assign13910_e19662_d_n7;
        locals.var_xmp_dn10 = assign13910_e19662_d_n10;
        locals.var_xmp_dn11 = assign13910_e19662_d_n11;
        locals.var_xmp_dn12 = assign13910_e19662_d_n12;
        locals.var_xmp_dn17 = assign13910_e19662_d_n17;

        let (assign13920_e19671,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign13920_e19671;

        let (assign13930_e19680,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign13930_e19680;

        let (assign13940_e19689, assign13940_e19689_d_n0, assign13940_e19689_d_n2, assign13940_e19689_d_n6, assign13940_e19689_d_n7, assign13940_e19689_d_n10, assign13940_e19689_d_n11, assign13940_e19689_d_n12, assign13940_e19689_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign13940_e19689;
        locals.var_arg_dn0 = assign13940_e19689_d_n0;
        locals.var_arg_dn2 = assign13940_e19689_d_n2;
        locals.var_arg_dn6 = assign13940_e19689_d_n6;
        locals.var_arg_dn7 = assign13940_e19689_d_n7;
        locals.var_arg_dn10 = assign13940_e19689_d_n10;
        locals.var_arg_dn11 = assign13940_e19689_d_n11;
        locals.var_arg_dn12 = assign13940_e19689_d_n12;
        locals.var_arg_dn17 = assign13940_e19689_d_n17;

    }

    pub(super) fn stamp_transient_block_45(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13950_e19698, assign13950_e19698_d_n0, assign13950_e19698_d_n2, assign13950_e19698_d_n6, assign13950_e19698_d_n7, assign13950_e19698_d_n10, assign13950_e19698_d_n11, assign13950_e19698_d_n12, assign13950_e19698_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign13950_e19698;
        locals.var_dnm_dn0 = assign13950_e19698_d_n0;
        locals.var_dnm_dn2 = assign13950_e19698_d_n2;
        locals.var_dnm_dn6 = assign13950_e19698_d_n6;
        locals.var_dnm_dn7 = assign13950_e19698_d_n7;
        locals.var_dnm_dn10 = assign13950_e19698_d_n10;
        locals.var_dnm_dn11 = assign13950_e19698_d_n11;
        locals.var_dnm_dn12 = assign13950_e19698_d_n12;
        locals.var_dnm_dn17 = assign13950_e19698_d_n17;

        let (assign13960_e19709, assign13960_e19709_d_n0, assign13960_e19709_d_n2, assign13960_e19709_d_n6, assign13960_e19709_d_n7, assign13960_e19709_d_n10, assign13960_e19709_d_n11, assign13960_e19709_d_n12, assign13960_e19709_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        let assign13960_e19707: f64 = (locals.var_xp * locals.var_x2);
        (assign13960_e19707, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13960_e19709;
        locals.var_xp_dn0 = assign13960_e19709_d_n0;
        locals.var_xp_dn2 = assign13960_e19709_d_n2;
        locals.var_xp_dn6 = assign13960_e19709_d_n6;
        locals.var_xp_dn7 = assign13960_e19709_d_n7;
        locals.var_xp_dn10 = assign13960_e19709_d_n10;
        locals.var_xp_dn11 = assign13960_e19709_d_n11;
        locals.var_xp_dn12 = assign13960_e19709_d_n12;
        locals.var_xp_dn17 = assign13960_e19709_d_n17;

        let (assign13970_e19720, assign13970_e19720_d_n0, assign13970_e19720_d_n2, assign13970_e19720_d_n6, assign13970_e19720_d_n7, assign13970_e19720_d_n10, assign13970_e19720_d_n11, assign13970_e19720_d_n12, assign13970_e19720_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        let assign13970_e19718: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign13970_e19718, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13970_e19720;
        locals.var_xmp_dn0 = assign13970_e19720_d_n0;
        locals.var_xmp_dn2 = assign13970_e19720_d_n2;
        locals.var_xmp_dn6 = assign13970_e19720_d_n6;
        locals.var_xmp_dn7 = assign13970_e19720_d_n7;
        locals.var_xmp_dn10 = assign13970_e19720_d_n10;
        locals.var_xmp_dn11 = assign13970_e19720_d_n11;
        locals.var_xmp_dn12 = assign13970_e19720_d_n12;
        locals.var_xmp_dn17 = assign13970_e19720_d_n17;

        let (assign13980_e19731, assign13980_e19731_d_n0, assign13980_e19731_d_n2, assign13980_e19731_d_n6, assign13980_e19731_d_n7, assign13980_e19731_d_n10, assign13980_e19731_d_n11, assign13980_e19731_d_n12, assign13980_e19731_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        let assign13980_e19729: f64 = (locals.var_xp * locals.var_x2);
        (assign13980_e19729, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign13980_e19731;
        locals.var_xp_dn0 = assign13980_e19731_d_n0;
        locals.var_xp_dn2 = assign13980_e19731_d_n2;
        locals.var_xp_dn6 = assign13980_e19731_d_n6;
        locals.var_xp_dn7 = assign13980_e19731_d_n7;
        locals.var_xp_dn10 = assign13980_e19731_d_n10;
        locals.var_xp_dn11 = assign13980_e19731_d_n11;
        locals.var_xp_dn12 = assign13980_e19731_d_n12;
        locals.var_xp_dn17 = assign13980_e19731_d_n17;

        let (assign13990_e19742, assign13990_e19742_d_n0, assign13990_e19742_d_n2, assign13990_e19742_d_n6, assign13990_e19742_d_n7, assign13990_e19742_d_n10, assign13990_e19742_d_n11, assign13990_e19742_d_n12, assign13990_e19742_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        let assign13990_e19740: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign13990_e19740, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign13990_e19742;
        locals.var_xmp_dn0 = assign13990_e19742_d_n0;
        locals.var_xmp_dn2 = assign13990_e19742_d_n2;
        locals.var_xmp_dn6 = assign13990_e19742_d_n6;
        locals.var_xmp_dn7 = assign13990_e19742_d_n7;
        locals.var_xmp_dn10 = assign13990_e19742_d_n10;
        locals.var_xmp_dn11 = assign13990_e19742_d_n11;
        locals.var_xmp_dn12 = assign13990_e19742_d_n12;
        locals.var_xmp_dn17 = assign13990_e19742_d_n17;

        let (assign14000_e19753, assign14000_e19753_d_n0, assign14000_e19753_d_n2, assign14000_e19753_d_n6, assign14000_e19753_d_n7, assign14000_e19753_d_n10, assign14000_e19753_d_n11, assign14000_e19753_d_n12, assign14000_e19753_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        let assign14000_e19751: f64 = (locals.var_xp + locals.var_xmp);
        (assign14000_e19751, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign14000_e19753;
        locals.var_arg_dn0 = assign14000_e19753_d_n0;
        locals.var_arg_dn2 = assign14000_e19753_d_n2;
        locals.var_arg_dn6 = assign14000_e19753_d_n6;
        locals.var_arg_dn7 = assign14000_e19753_d_n7;
        locals.var_arg_dn10 = assign14000_e19753_d_n10;
        locals.var_arg_dn11 = assign14000_e19753_d_n11;
        locals.var_arg_dn12 = assign14000_e19753_d_n12;
        locals.var_arg_dn17 = assign14000_e19753_d_n17;

        let (assign14010_e19762, assign14010_e19762_d_n0, assign14010_e19762_d_n2, assign14010_e19762_d_n6, assign14010_e19762_d_n7, assign14010_e19762_d_n10, assign14010_e19762_d_n11, assign14010_e19762_d_n12, assign14010_e19762_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign14010_e19762;
        locals.var_dnm_dn0 = assign14010_e19762_d_n0;
        locals.var_dnm_dn2 = assign14010_e19762_d_n2;
        locals.var_dnm_dn6 = assign14010_e19762_d_n6;
        locals.var_dnm_dn7 = assign14010_e19762_d_n7;
        locals.var_dnm_dn10 = assign14010_e19762_d_n10;
        locals.var_dnm_dn11 = assign14010_e19762_d_n11;
        locals.var_dnm_dn12 = assign14010_e19762_d_n12;
        locals.var_dnm_dn17 = assign14010_e19762_d_n17;

        let assign14020_e19777: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard430 = assign14020_e19777;

        let assign14030_e19780: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard431 = assign14030_e19780;

        let (assign14040_e19793,) = {
    if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) && (locals.var_guard430 != 0.0)) && (locals.var_guard431 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign14040_e19793;

        let assign14050_e19796: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard432 = assign14050_e19796;

        let (assign14060_e19812,) = {
    if ((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) && (locals.var_guard430 != 0.0)) && (locals.var_guard431 == 0.0)) && (locals.var_guard432 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign14060_e19812;

        let assign14070_e19815: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard433 = assign14070_e19815;

        let (assign14080_e19834,) = {
    if (((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) && (locals.var_guard430 != 0.0)) && (locals.var_guard431 == 0.0)) && (locals.var_guard432 == 0.0)) && (locals.var_guard433 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign14080_e19834;

        let assign14090_e19837: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard434 = assign14090_e19837;

        let (assign14100_e19859,) = {
    if ((((((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) && (locals.var_guard430 != 0.0)) && (locals.var_guard431 == 0.0)) && (locals.var_guard432 == 0.0)) && (locals.var_guard433 == 0.0)) && (locals.var_guard434 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign14100_e19859;

        let (assign14110_e19870,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) && (locals.var_guard430 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign14110_e19870;

        let mut assign14120_loop_guard: usize = 0;
        while {
            let assign14120_cond_e19882: f64 = if (((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) && (locals.var_guard430 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign14120_cond_e19882 != 0.0
        } {
            assign14120_loop_guard += 1;
            assert!(assign14120_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign14120_body0_e19894, assign14120_body0_e19894_d_n0, assign14120_body0_e19894_d_n2, assign14120_body0_e19894_d_n6, assign14120_body0_e19894_d_n7, assign14120_body0_e19894_d_n10, assign14120_body0_e19894_d_n11, assign14120_body0_e19894_d_n12, assign14120_body0_e19894_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) && (locals.var_guard430 != 0.0)) {
        let assign14120_body0_e19892: f64 = (locals.var_dnm).sqrt();
        (assign14120_body0_e19892, (locals.var_dnm_dn0 / (2.0 * assign14120_body0_e19892)), (locals.var_dnm_dn2 / (2.0 * assign14120_body0_e19892)), (locals.var_dnm_dn6 / (2.0 * assign14120_body0_e19892)), (locals.var_dnm_dn7 / (2.0 * assign14120_body0_e19892)), (locals.var_dnm_dn10 / (2.0 * assign14120_body0_e19892)), (locals.var_dnm_dn11 / (2.0 * assign14120_body0_e19892)), (locals.var_dnm_dn12 / (2.0 * assign14120_body0_e19892)), (locals.var_dnm_dn17 / (2.0 * assign14120_body0_e19892)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign14120_body0_e19894;
            locals.var_dnm_dn0 = assign14120_body0_e19894_d_n0;
            locals.var_dnm_dn2 = assign14120_body0_e19894_d_n2;
            locals.var_dnm_dn6 = assign14120_body0_e19894_d_n6;
            locals.var_dnm_dn7 = assign14120_body0_e19894_d_n7;
            locals.var_dnm_dn10 = assign14120_body0_e19894_d_n10;
            locals.var_dnm_dn11 = assign14120_body0_e19894_d_n11;
            locals.var_dnm_dn12 = assign14120_body0_e19894_d_n12;
            locals.var_dnm_dn17 = assign14120_body0_e19894_d_n17;
            let (assign14120_body1_e19907,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) && (locals.var_guard430 != 0.0)) {
        let assign14120_body1_e19905: f64 = (locals.var_m0 + 1.0);
        (assign14120_body1_e19905,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign14120_body1_e19907;
        }

        let (assign14130_e19925, assign14130_e19925_d_n0, assign14130_e19925_d_n2, assign14130_e19925_d_n6, assign14130_e19925_d_n7, assign14130_e19925_d_n10, assign14130_e19925_d_n11, assign14130_e19925_d_n12, assign14130_e19925_d_n17,) = {
    if ((((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) && (locals.var_guard430 == 0.0)) {
        let assign14130_e19921: f64 = (2.0 * 2.0);
        let assign14130_e19922: f64 = (1.0 / assign14130_e19921);
        let assign14130_e19923: f64 = (locals.var_dnm).powf(assign14130_e19922);
        (assign14130_e19923, if 0.0 == 0.0 && ((assign14130_e19922) as f64).is_finite() && ((assign14130_e19922) as f64).fract() == 0.0 { if assign14130_e19922 == 0.0 { 0.0 } else { (assign14130_e19922 * ((locals.var_dnm).powf(assign14130_e19922 - 1.0) * locals.var_dnm_dn0)) } } else { (assign14130_e19923 * (assign14130_e19922 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14130_e19922) as f64).is_finite() && ((assign14130_e19922) as f64).fract() == 0.0 { if assign14130_e19922 == 0.0 { 0.0 } else { (assign14130_e19922 * ((locals.var_dnm).powf(assign14130_e19922 - 1.0) * locals.var_dnm_dn2)) } } else { (assign14130_e19923 * (assign14130_e19922 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14130_e19922) as f64).is_finite() && ((assign14130_e19922) as f64).fract() == 0.0 { if assign14130_e19922 == 0.0 { 0.0 } else { (assign14130_e19922 * ((locals.var_dnm).powf(assign14130_e19922 - 1.0) * locals.var_dnm_dn6)) } } else { (assign14130_e19923 * (assign14130_e19922 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14130_e19922) as f64).is_finite() && ((assign14130_e19922) as f64).fract() == 0.0 { if assign14130_e19922 == 0.0 { 0.0 } else { (assign14130_e19922 * ((locals.var_dnm).powf(assign14130_e19922 - 1.0) * locals.var_dnm_dn7)) } } else { (assign14130_e19923 * (assign14130_e19922 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14130_e19922) as f64).is_finite() && ((assign14130_e19922) as f64).fract() == 0.0 { if assign14130_e19922 == 0.0 { 0.0 } else { (assign14130_e19922 * ((locals.var_dnm).powf(assign14130_e19922 - 1.0) * locals.var_dnm_dn10)) } } else { (assign14130_e19923 * (assign14130_e19922 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14130_e19922) as f64).is_finite() && ((assign14130_e19922) as f64).fract() == 0.0 { if assign14130_e19922 == 0.0 { 0.0 } else { (assign14130_e19922 * ((locals.var_dnm).powf(assign14130_e19922 - 1.0) * locals.var_dnm_dn11)) } } else { (assign14130_e19923 * (assign14130_e19922 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14130_e19922) as f64).is_finite() && ((assign14130_e19922) as f64).fract() == 0.0 { if assign14130_e19922 == 0.0 { 0.0 } else { (assign14130_e19922 * ((locals.var_dnm).powf(assign14130_e19922 - 1.0) * locals.var_dnm_dn12)) } } else { (assign14130_e19923 * (assign14130_e19922 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign14130_e19922) as f64).is_finite() && ((assign14130_e19922) as f64).fract() == 0.0 { if assign14130_e19922 == 0.0 { 0.0 } else { (assign14130_e19922 * ((locals.var_dnm).powf(assign14130_e19922 - 1.0) * locals.var_dnm_dn17)) } } else { (assign14130_e19923 * (assign14130_e19922 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign14130_e19925;
        locals.var_dnm_dn0 = assign14130_e19925_d_n0;
        locals.var_dnm_dn2 = assign14130_e19925_d_n2;
        locals.var_dnm_dn6 = assign14130_e19925_d_n6;
        locals.var_dnm_dn7 = assign14130_e19925_d_n7;
        locals.var_dnm_dn10 = assign14130_e19925_d_n10;
        locals.var_dnm_dn11 = assign14130_e19925_d_n11;
        locals.var_dnm_dn12 = assign14130_e19925_d_n12;
        locals.var_dnm_dn17 = assign14130_e19925_d_n17;

        let (assign14140_e19936, assign14140_e19936_d_n0, assign14140_e19936_d_n2, assign14140_e19936_d_n6, assign14140_e19936_d_n7, assign14140_e19936_d_n10, assign14140_e19936_d_n11, assign14140_e19936_d_n12, assign14140_e19936_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        let assign14140_e19934: f64 = (1.0 / locals.var_dnm);
        (assign14140_e19934, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign14140_e19936;
        locals.var_dnm_dn0 = assign14140_e19936_d_n0;
        locals.var_dnm_dn2 = assign14140_e19936_d_n2;
        locals.var_dnm_dn6 = assign14140_e19936_d_n6;
        locals.var_dnm_dn7 = assign14140_e19936_d_n7;
        locals.var_dnm_dn10 = assign14140_e19936_d_n10;
        locals.var_dnm_dn11 = assign14140_e19936_d_n11;
        locals.var_dnm_dn12 = assign14140_e19936_d_n12;
        locals.var_dnm_dn17 = assign14140_e19936_d_n17;

        let (assign14150_e19949, assign14150_e19949_d_n0, assign14150_e19949_d_n2, assign14150_e19949_d_n6, assign14150_e19949_d_n7, assign14150_e19949_d_n10, assign14150_e19949_d_n11, assign14150_e19949_d_n12, assign14150_e19949_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        let assign14150_e19945: f64 = (locals.var_tmf1 * 1e-5);
        let assign14150_e19947: f64 = (assign14150_e19945 * locals.var_dnm);
        (assign14150_e19947, (((locals.var_tmf1_dn0 * 1e-5) * locals.var_dnm) + (assign14150_e19945 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-5) * locals.var_dnm) + (assign14150_e19945 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn6 * 1e-5) * locals.var_dnm) + (assign14150_e19945 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-5) * locals.var_dnm) + (assign14150_e19945 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn10 * 1e-5) * locals.var_dnm) + (assign14150_e19945 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1e-5) * locals.var_dnm) + (assign14150_e19945 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn12 * 1e-5) * locals.var_dnm) + (assign14150_e19945 * locals.var_dnm_dn12)), (((locals.var_tmf1_dn17 * 1e-5) * locals.var_dnm) + (assign14150_e19945 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign14150_e19949;
        locals.var_tmf0_dn0 = assign14150_e19949_d_n0;
        locals.var_tmf0_dn2 = assign14150_e19949_d_n2;
        locals.var_tmf0_dn6 = assign14150_e19949_d_n6;
        locals.var_tmf0_dn7 = assign14150_e19949_d_n7;
        locals.var_tmf0_dn10 = assign14150_e19949_d_n10;
        locals.var_tmf0_dn11 = assign14150_e19949_d_n11;
        locals.var_tmf0_dn12 = assign14150_e19949_d_n12;
        locals.var_tmf0_dn17 = assign14150_e19949_d_n17;

        let (assign14160_e19962, assign14160_e19962_d_n0, assign14160_e19962_d_n2, assign14160_e19962_d_n6, assign14160_e19962_d_n7, assign14160_e19962_d_n10, assign14160_e19962_d_n11, assign14160_e19962_d_n12, assign14160_e19962_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 != 0.0)) {
        let assign14160_e19958: f64 = 1e-5;
        let assign14160_e19960: f64 = (assign14160_e19958 - locals.var_tmf0);
        (assign14160_e19960, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn12), (-locals.var_tmf0_dn17),)
    } else {
        (locals.var_rrr_eta, locals.var_rrr_eta_dn0, locals.var_rrr_eta_dn2, locals.var_rrr_eta_dn6, locals.var_rrr_eta_dn7, locals.var_rrr_eta_dn10, locals.var_rrr_eta_dn11, locals.var_rrr_eta_dn12, locals.var_rrr_eta_dn17,)
    }
};
        locals.var_rrr_eta = assign14160_e19962;
        locals.var_rrr_eta_dn0 = assign14160_e19962_d_n0;
        locals.var_rrr_eta_dn2 = assign14160_e19962_d_n2;
        locals.var_rrr_eta_dn6 = assign14160_e19962_d_n6;
        locals.var_rrr_eta_dn7 = assign14160_e19962_d_n7;
        locals.var_rrr_eta_dn10 = assign14160_e19962_d_n10;
        locals.var_rrr_eta_dn11 = assign14160_e19962_d_n11;
        locals.var_rrr_eta_dn12 = assign14160_e19962_d_n12;
        locals.var_rrr_eta_dn17 = assign14160_e19962_d_n17;

        let (assign14170_e19972, assign14170_e19972_d_n0, assign14170_e19972_d_n2, assign14170_e19972_d_n6, assign14170_e19972_d_n7, assign14170_e19972_d_n10, assign14170_e19972_d_n11, assign14170_e19972_d_n12, assign14170_e19972_d_n17,) = {
    if (((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) && (locals.var_guard429 == 0.0)) {
        (locals.var_rrr_eta, locals.var_rrr_eta_dn0, locals.var_rrr_eta_dn2, locals.var_rrr_eta_dn6, locals.var_rrr_eta_dn7, locals.var_rrr_eta_dn10, locals.var_rrr_eta_dn11, locals.var_rrr_eta_dn12, locals.var_rrr_eta_dn17,)
    } else {
        (locals.var_rrr_eta, locals.var_rrr_eta_dn0, locals.var_rrr_eta_dn2, locals.var_rrr_eta_dn6, locals.var_rrr_eta_dn7, locals.var_rrr_eta_dn10, locals.var_rrr_eta_dn11, locals.var_rrr_eta_dn12, locals.var_rrr_eta_dn17,)
    }
};
        locals.var_rrr_eta = assign14170_e19972;
        locals.var_rrr_eta_dn0 = assign14170_e19972_d_n0;
        locals.var_rrr_eta_dn2 = assign14170_e19972_d_n2;
        locals.var_rrr_eta_dn6 = assign14170_e19972_d_n6;
        locals.var_rrr_eta_dn7 = assign14170_e19972_d_n7;
        locals.var_rrr_eta_dn10 = assign14170_e19972_d_n10;
        locals.var_rrr_eta_dn11 = assign14170_e19972_d_n11;
        locals.var_rrr_eta_dn12 = assign14170_e19972_d_n12;
        locals.var_rrr_eta_dn17 = assign14170_e19972_d_n17;

        let (assign14180_e19979, assign14180_e19979_d_n0, assign14180_e19979_d_n2, assign14180_e19979_d_n6, assign14180_e19979_d_n7, assign14180_e19979_d_n10, assign14180_e19979_d_n11, assign14180_e19979_d_n12, assign14180_e19979_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        (locals.var_rrr_eta, locals.var_rrr_eta_dn0, locals.var_rrr_eta_dn2, locals.var_rrr_eta_dn6, locals.var_rrr_eta_dn7, locals.var_rrr_eta_dn10, locals.var_rrr_eta_dn11, locals.var_rrr_eta_dn12, locals.var_rrr_eta_dn17,)
    } else {
        (locals.var_alpha, locals.var_alpha_dn0, locals.var_alpha_dn2, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn10, locals.var_alpha_dn11, locals.var_alpha_dn12, locals.var_alpha_dn17,)
    }
};
        locals.var_alpha = assign14180_e19979;
        locals.var_alpha_dn0 = assign14180_e19979_d_n0;
        locals.var_alpha_dn2 = assign14180_e19979_d_n2;
        locals.var_alpha_dn6 = assign14180_e19979_d_n6;
        locals.var_alpha_dn7 = assign14180_e19979_d_n7;
        locals.var_alpha_dn10 = assign14180_e19979_d_n10;
        locals.var_alpha_dn11 = assign14180_e19979_d_n11;
        locals.var_alpha_dn12 = assign14180_e19979_d_n12;
        locals.var_alpha_dn17 = assign14180_e19979_d_n17;

        let (assign14190_e19992, assign14190_e19992_d_n0, assign14190_e19992_d_n2, assign14190_e19992_d_n6, assign14190_e19992_d_n7, assign14190_e19992_d_n10, assign14190_e19992_d_n11, assign14190_e19992_d_n12, assign14190_e19992_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign14190_e19988: f64 = (1.0 + locals.var_alpha);
        let assign14190_e19989: f64 = (locals.var_alpha * assign14190_e19988);
        let assign14190_e19990: f64 = (1.0 + assign14190_e19989);
        (assign14190_e19990, ((locals.var_alpha_dn0 * assign14190_e19988) + (locals.var_alpha * locals.var_alpha_dn0)), ((locals.var_alpha_dn2 * assign14190_e19988) + (locals.var_alpha * locals.var_alpha_dn2)), ((locals.var_alpha_dn6 * assign14190_e19988) + (locals.var_alpha * locals.var_alpha_dn6)), ((locals.var_alpha_dn7 * assign14190_e19988) + (locals.var_alpha * locals.var_alpha_dn7)), ((locals.var_alpha_dn10 * assign14190_e19988) + (locals.var_alpha * locals.var_alpha_dn10)), ((locals.var_alpha_dn11 * assign14190_e19988) + (locals.var_alpha * locals.var_alpha_dn11)), ((locals.var_alpha_dn12 * assign14190_e19988) + (locals.var_alpha * locals.var_alpha_dn12)), ((locals.var_alpha_dn17 * assign14190_e19988) + (locals.var_alpha * locals.var_alpha_dn17)),)
    } else {
        (locals.var_qinm, locals.var_qinm_dn0, locals.var_qinm_dn2, locals.var_qinm_dn6, locals.var_qinm_dn7, locals.var_qinm_dn10, locals.var_qinm_dn11, locals.var_qinm_dn12, locals.var_qinm_dn17,)
    }
};
        locals.var_qinm = assign14190_e19992;
        locals.var_qinm_dn0 = assign14190_e19992_d_n0;
        locals.var_qinm_dn2 = assign14190_e19992_d_n2;
        locals.var_qinm_dn6 = assign14190_e19992_d_n6;
        locals.var_qinm_dn7 = assign14190_e19992_d_n7;
        locals.var_qinm_dn10 = assign14190_e19992_d_n10;
        locals.var_qinm_dn11 = assign14190_e19992_d_n11;
        locals.var_qinm_dn12 = assign14190_e19992_d_n12;
        locals.var_qinm_dn17 = assign14190_e19992_d_n17;

        let (assign14200_e20012, assign14200_e20012_d_n0, assign14200_e20012_d_n2, assign14200_e20012_d_n6, assign14200_e20012_d_n7, assign14200_e20012_d_n10, assign14200_e20012_d_n11, assign14200_e20012_d_n12, assign14200_e20012_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign14200_e19999: f64 = (1.0 + locals.var_alpha);
        let assign14200_e20002: f64 = (10.0 * 2.220446049250313e-16);
        let (assign14200_e20010, assign14200_e20010_d_n0, assign14200_e20010_d_n2, assign14200_e20010_d_n6, assign14200_e20010_d_n7, assign14200_e20010_d_n10, assign14200_e20010_d_n11, assign14200_e20010_d_n12, assign14200_e20010_d_n17,) = {
            if (assign14200_e19999 >= assign14200_e20002) {
                let assign14200_e20006: f64 = (1.0 + locals.var_alpha);
                (assign14200_e20006, locals.var_alpha_dn0, locals.var_alpha_dn2, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn10, locals.var_alpha_dn11, locals.var_alpha_dn12, locals.var_alpha_dn17,)
            } else {
                let assign14200_e20009: f64 = (10.0 * 2.220446049250313e-16);
                (assign14200_e20009, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign14200_e20010, assign14200_e20010_d_n0, assign14200_e20010_d_n2, assign14200_e20010_d_n6, assign14200_e20010_d_n7, assign14200_e20010_d_n10, assign14200_e20010_d_n11, assign14200_e20010_d_n12, assign14200_e20010_d_n17,)
    } else {
        (locals.var_qidn, locals.var_qidn_dn0, locals.var_qidn_dn2, locals.var_qidn_dn6, locals.var_qidn_dn7, locals.var_qidn_dn10, locals.var_qidn_dn11, locals.var_qidn_dn12, locals.var_qidn_dn17,)
    }
};
        locals.var_qidn = assign14200_e20012;
        locals.var_qidn_dn0 = assign14200_e20012_d_n0;
        locals.var_qidn_dn2 = assign14200_e20012_d_n2;
        locals.var_qidn_dn6 = assign14200_e20012_d_n6;
        locals.var_qidn_dn7 = assign14200_e20012_d_n7;
        locals.var_qidn_dn10 = assign14200_e20012_d_n10;
        locals.var_qidn_dn11 = assign14200_e20012_d_n11;
        locals.var_qidn_dn12 = assign14200_e20012_d_n12;
        locals.var_qidn_dn17 = assign14200_e20012_d_n17;

        let (assign14210_e20024, assign14210_e20024_d_n0, assign14210_e20024_d_n2, assign14210_e20024_d_n6, assign14210_e20024_d_n7, assign14210_e20024_d_n10, assign14210_e20024_d_n11, assign14210_e20024_d_n12, assign14210_e20024_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard303 == 0.0)) {
        let assign14210_e20018: f64 = (-0.5);
        let assign14210_e20021: f64 = (locals.var_q_n0 + locals.var_q_nl);
        let assign14210_e20022: f64 = (assign14210_e20018 * assign14210_e20021);
        (assign14210_e20022, (assign14210_e20018 * (locals.var_q_n0_dn0 + locals.var_q_nl_dn0)), (assign14210_e20018 * (locals.var_q_n0_dn2 + locals.var_q_nl_dn2)), (assign14210_e20018 * (locals.var_q_n0_dn6 + locals.var_q_nl_dn6)), (assign14210_e20018 * (locals.var_q_n0_dn7 + locals.var_q_nl_dn7)), (assign14210_e20018 * (locals.var_q_n0_dn10 + locals.var_q_nl_dn10)), (assign14210_e20018 * (locals.var_q_n0_dn11 + locals.var_q_nl_dn11)), (assign14210_e20018 * (locals.var_q_n0_dn12 + locals.var_q_nl_dn12)), (assign14210_e20018 * (locals.var_q_n0_dn17 + locals.var_q_nl_dn17)),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn12, locals.var_qiu_dn17,)
    }
};
        locals.var_qiu = assign14210_e20024;
        locals.var_qiu_dn0 = assign14210_e20024_d_n0;
        locals.var_qiu_dn2 = assign14210_e20024_d_n2;
        locals.var_qiu_dn6 = assign14210_e20024_d_n6;
        locals.var_qiu_dn7 = assign14210_e20024_d_n7;
        locals.var_qiu_dn10 = assign14210_e20024_d_n10;
        locals.var_qiu_dn11 = assign14210_e20024_d_n11;
        locals.var_qiu_dn12 = assign14210_e20024_d_n12;
        locals.var_qiu_dn17 = assign14210_e20024_d_n17;

        let (assign14280_e20057, assign14280_e20057_d_n0, assign14280_e20057_d_n2, assign14280_e20057_d_n6, assign14280_e20057_d_n7, assign14280_e20057_d_n10, assign14280_e20057_d_n11, assign14280_e20057_d_n12, assign14280_e20057_d_n17,) = {
    if (locals.var_guard111 == 0.0) {
        (locals.var_vbsc, locals.var_vbsc_dn0, locals.var_vbsc_dn2, locals.var_vbsc_dn6, locals.var_vbsc_dn7, locals.var_vbsc_dn10, locals.var_vbsc_dn11, locals.var_vbsc_dn12, locals.var_vbsc_dn17,)
    } else {
        (locals.var_vbcs_cl, locals.var_vbcs_cl_dn0, locals.var_vbcs_cl_dn2, locals.var_vbcs_cl_dn6, locals.var_vbcs_cl_dn7, locals.var_vbcs_cl_dn10, locals.var_vbcs_cl_dn11, locals.var_vbcs_cl_dn12, locals.var_vbcs_cl_dn17,)
    }
};
        locals.var_vbcs_cl = assign14280_e20057;
        locals.var_vbcs_cl_dn0 = assign14280_e20057_d_n0;
        locals.var_vbcs_cl_dn2 = assign14280_e20057_d_n2;
        locals.var_vbcs_cl_dn6 = assign14280_e20057_d_n6;
        locals.var_vbcs_cl_dn7 = assign14280_e20057_d_n7;
        locals.var_vbcs_cl_dn10 = assign14280_e20057_d_n10;
        locals.var_vbcs_cl_dn11 = assign14280_e20057_d_n11;
        locals.var_vbcs_cl_dn12 = assign14280_e20057_d_n12;
        locals.var_vbcs_cl_dn17 = assign14280_e20057_d_n17;

        let assign14290_e20060: f64 = if locals.var_wdsoi_ini < p.p237 { 1.0 } else { 0.0 };
        locals.var_guard441 = assign14290_e20060;

        let (assign14300_e20067,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard441 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_depmode,)
    }
};
        locals.var_flg_depmode = assign14300_e20067;

        let (assign14310_e20075,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard441 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_flg_depmode,)
    }
};
        locals.var_flg_depmode = assign14310_e20075;

        let (assign14320_e20086,) = {
    if (locals.var_guard111 == 0.0) {
        let assign14320_e20080: f64 = (locals.var_vfb - locals.var_dvth);
        let assign14320_e20082: f64 = (assign14320_e20080 + locals.var_dppg);
        let assign14320_e20084: f64 = (assign14320_e20082 + locals.var_vbcs_cl);
        (assign14320_e20084,)
    } else {
        (locals.var_vgs_fb,)
    }
};
        locals.var_vgs_fb = assign14320_e20086;

        let assign14330_e20089: f64 = if locals.var_vgs < locals.var_vgs_fb { 1.0 } else { 0.0 };
        locals.var_guard442 = assign14330_e20089;

        let (assign14340_e20097,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14340_e20095: f64 = (-1.0);
        (assign14340_e20095,)
    } else {
        (locals.var_flg_zone,)
    }
};
        locals.var_flg_zone = assign14340_e20097;

        let (assign14350_e20112, assign14350_e20112_d_n0, assign14350_e20112_d_n2, assign14350_e20112_d_n6, assign14350_e20112_d_n7, assign14350_e20112_d_n10, assign14350_e20112_d_n11, assign14350_e20112_d_n12, assign14350_e20112_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14350_e20104: f64 = (2.0 * locals.var_beta_inv);
        let assign14350_e20106: f64 = (-locals.var_vgs_min);
        let assign14350_e20108: f64 = (assign14350_e20106 / locals.var_fac1);
        let assign14350_e20109: f64 = (assign14350_e20108).ln();
        let assign14350_e20110: f64 = (assign14350_e20104 * assign14350_e20109);
        (assign14350_e20110, (assign14350_e20104 * ((-((assign14350_e20106 * locals.var_fac1_dn0) / (locals.var_fac1 * locals.var_fac1))) / assign14350_e20108)), (assign14350_e20104 * ((-((assign14350_e20106 * locals.var_fac1_dn2) / (locals.var_fac1 * locals.var_fac1))) / assign14350_e20108)), (assign14350_e20104 * ((-((assign14350_e20106 * locals.var_fac1_dn6) / (locals.var_fac1 * locals.var_fac1))) / assign14350_e20108)), (assign14350_e20104 * ((-((assign14350_e20106 * locals.var_fac1_dn7) / (locals.var_fac1 * locals.var_fac1))) / assign14350_e20108)), (((2.0 * locals.var_beta_inv_dn10) * assign14350_e20109) + (assign14350_e20104 * ((-((assign14350_e20106 * locals.var_fac1_dn10) / (locals.var_fac1 * locals.var_fac1))) / assign14350_e20108))), (assign14350_e20104 * ((-((assign14350_e20106 * locals.var_fac1_dn11) / (locals.var_fac1 * locals.var_fac1))) / assign14350_e20108)), (assign14350_e20104 * ((-((assign14350_e20106 * locals.var_fac1_dn12) / (locals.var_fac1 * locals.var_fac1))) / assign14350_e20108)), (assign14350_e20104 * ((-((assign14350_e20106 * locals.var_fac1_dn17) / (locals.var_fac1 * locals.var_fac1))) / assign14350_e20108)),)
    } else {
        (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn6, locals.var_ps0_min_dn7, locals.var_ps0_min_dn10, locals.var_ps0_min_dn11, locals.var_ps0_min_dn12, locals.var_ps0_min_dn17,)
    }
};
        locals.var_ps0_min = assign14350_e20112;
        locals.var_ps0_min_dn0 = assign14350_e20112_d_n0;
        locals.var_ps0_min_dn2 = assign14350_e20112_d_n2;
        locals.var_ps0_min_dn6 = assign14350_e20112_d_n6;
        locals.var_ps0_min_dn7 = assign14350_e20112_d_n7;
        locals.var_ps0_min_dn10 = assign14350_e20112_d_n10;
        locals.var_ps0_min_dn11 = assign14350_e20112_d_n11;
        locals.var_ps0_min_dn12 = assign14350_e20112_d_n12;
        locals.var_ps0_min_dn17 = assign14350_e20112_d_n17;

        let (assign14360_e20123, assign14360_e20123_d_n0, assign14360_e20123_d_n2, assign14360_e20123_d_n6, assign14360_e20123_d_n7, assign14360_e20123_d_n10, assign14360_e20123_d_n11, assign14360_e20123_d_n12, assign14360_e20123_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14360_e20120: f64 = (locals.var_vgp - locals.var_vbcs_cl);
        let assign14360_e20121: f64 = (locals.var_beta * assign14360_e20120);
        (assign14360_e20121, (locals.var_beta * (locals.var_vgp_dn0 - locals.var_vbcs_cl_dn0)), (locals.var_beta * (locals.var_vgp_dn2 - locals.var_vbcs_cl_dn2)), (locals.var_beta * (locals.var_vgp_dn6 - locals.var_vbcs_cl_dn6)), (locals.var_beta * (locals.var_vgp_dn7 - locals.var_vbcs_cl_dn7)), ((locals.var_beta_dn10 * assign14360_e20120) + (locals.var_beta * (locals.var_vgp_dn10 - locals.var_vbcs_cl_dn10))), (locals.var_beta * (locals.var_vgp_dn11 - locals.var_vbcs_cl_dn11)), (locals.var_beta * (locals.var_vgp_dn12 - locals.var_vbcs_cl_dn12)), (locals.var_beta * (locals.var_vgp_dn17 - locals.var_vbcs_cl_dn17)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign14360_e20123;
        locals.var_tx_dn0 = assign14360_e20123_d_n0;
        locals.var_tx_dn2 = assign14360_e20123_d_n2;
        locals.var_tx_dn6 = assign14360_e20123_d_n6;
        locals.var_tx_dn7 = assign14360_e20123_d_n7;
        locals.var_tx_dn10 = assign14360_e20123_d_n10;
        locals.var_tx_dn11 = assign14360_e20123_d_n11;
        locals.var_tx_dn12 = assign14360_e20123_d_n12;
        locals.var_tx_dn17 = assign14360_e20123_d_n17;

    }

    pub(super) fn stamp_transient_block_46(
        locals: &mut StampLocals,
    ) {
        let (assign14370_e20134, assign14370_e20134_d_n0, assign14370_e20134_d_n2, assign14370_e20134_d_n6, assign14370_e20134_d_n7, assign14370_e20134_d_n10, assign14370_e20134_d_n11, assign14370_e20134_d_n12, assign14370_e20134_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14370_e20131: f64 = (locals.var_beta * locals.var_cnst0soi);
        let assign14370_e20132: f64 = (1.0 / assign14370_e20131);
        (assign14370_e20132, (-((locals.var_beta * locals.var_cnst0soi_dn0) / (assign14370_e20131 * assign14370_e20131))), (-((locals.var_beta * locals.var_cnst0soi_dn2) / (assign14370_e20131 * assign14370_e20131))), (-((locals.var_beta * locals.var_cnst0soi_dn6) / (assign14370_e20131 * assign14370_e20131))), (-((locals.var_beta * locals.var_cnst0soi_dn7) / (assign14370_e20131 * assign14370_e20131))), (-(((locals.var_beta_dn10 * locals.var_cnst0soi) + (locals.var_beta * locals.var_cnst0soi_dn10)) / (assign14370_e20131 * assign14370_e20131))), (-((locals.var_beta * locals.var_cnst0soi_dn11) / (assign14370_e20131 * assign14370_e20131))), (-((locals.var_beta * locals.var_cnst0soi_dn12) / (assign14370_e20131 * assign14370_e20131))), (-((locals.var_beta * locals.var_cnst0soi_dn17) / (assign14370_e20131 * assign14370_e20131))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign14370_e20134;
        locals.var_t1_dn0 = assign14370_e20134_d_n0;
        locals.var_t1_dn2 = assign14370_e20134_d_n2;
        locals.var_t1_dn6 = assign14370_e20134_d_n6;
        locals.var_t1_dn7 = assign14370_e20134_d_n7;
        locals.var_t1_dn10 = assign14370_e20134_d_n10;
        locals.var_t1_dn11 = assign14370_e20134_d_n11;
        locals.var_t1_dn12 = assign14370_e20134_d_n12;
        locals.var_t1_dn17 = assign14370_e20134_d_n17;

        let (assign14380_e20143, assign14380_e20143_d_n0, assign14380_e20143_d_n2, assign14380_e20143_d_n6, assign14380_e20143_d_n7, assign14380_e20143_d_n10, assign14380_e20143_d_n11, assign14380_e20143_d_n12, assign14380_e20143_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14380_e20141: f64 = (locals.var_t1 * locals.var_c_fox);
        (assign14380_e20141, ((locals.var_t1_dn0 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn0)), ((locals.var_t1_dn2 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn2)), ((locals.var_t1_dn6 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn6)), ((locals.var_t1_dn7 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn7)), ((locals.var_t1_dn10 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn10)), ((locals.var_t1_dn11 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn11)), ((locals.var_t1_dn12 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn12)), ((locals.var_t1_dn17 * locals.var_c_fox) + (locals.var_t1 * locals.var_c_fox_dn17)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
        locals.var_ty = assign14380_e20143;
        locals.var_ty_dn0 = assign14380_e20143_d_n0;
        locals.var_ty_dn2 = assign14380_e20143_d_n2;
        locals.var_ty_dn6 = assign14380_e20143_d_n6;
        locals.var_ty_dn7 = assign14380_e20143_d_n7;
        locals.var_ty_dn10 = assign14380_e20143_d_n10;
        locals.var_ty_dn11 = assign14380_e20143_d_n11;
        locals.var_ty_dn12 = assign14380_e20143_d_n12;
        locals.var_ty_dn17 = assign14380_e20143_d_n17;

        let (assign14390_e20156, assign14390_e20156_d_n0, assign14390_e20156_d_n2, assign14390_e20156_d_n6, assign14390_e20156_d_n7, assign14390_e20156_d_n10, assign14390_e20156_d_n11, assign14390_e20156_d_n12, assign14390_e20156_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14390_e20151: f64 = (3.0 * 1.414213562373095);
        let assign14390_e20153: f64 = (assign14390_e20151 * locals.var_ty);
        let assign14390_e20154: f64 = (2.0 + assign14390_e20153);
        (assign14390_e20154, (assign14390_e20151 * locals.var_ty_dn0), (assign14390_e20151 * locals.var_ty_dn2), (assign14390_e20151 * locals.var_ty_dn6), (assign14390_e20151 * locals.var_ty_dn7), (assign14390_e20151 * locals.var_ty_dn10), (assign14390_e20151 * locals.var_ty_dn11), (assign14390_e20151 * locals.var_ty_dn12), (assign14390_e20151 * locals.var_ty_dn17),)
    } else {
        (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn6, locals.var_ac41_dn7, locals.var_ac41_dn10, locals.var_ac41_dn11, locals.var_ac41_dn12, locals.var_ac41_dn17,)
    }
};
        locals.var_ac41 = assign14390_e20156;
        locals.var_ac41_dn0 = assign14390_e20156_d_n0;
        locals.var_ac41_dn2 = assign14390_e20156_d_n2;
        locals.var_ac41_dn6 = assign14390_e20156_d_n6;
        locals.var_ac41_dn7 = assign14390_e20156_d_n7;
        locals.var_ac41_dn10 = assign14390_e20156_d_n10;
        locals.var_ac41_dn11 = assign14390_e20156_d_n11;
        locals.var_ac41_dn12 = assign14390_e20156_d_n12;
        locals.var_ac41_dn17 = assign14390_e20156_d_n17;

        let (assign14400_e20169, assign14400_e20169_d_n0, assign14400_e20169_d_n2, assign14400_e20169_d_n6, assign14400_e20169_d_n7, assign14400_e20169_d_n10, assign14400_e20169_d_n11, assign14400_e20169_d_n12, assign14400_e20169_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14400_e20163: f64 = (8.0 * locals.var_ac41);
        let assign14400_e20165: f64 = (assign14400_e20163 * locals.var_ac41);
        let assign14400_e20167: f64 = (assign14400_e20165 * locals.var_ac41);
        (assign14400_e20167, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign14400_e20163 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign14400_e20165 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign14400_e20163 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign14400_e20165 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign14400_e20163 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign14400_e20165 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn7) * locals.var_ac41) + (assign14400_e20163 * locals.var_ac41_dn7)) * locals.var_ac41) + (assign14400_e20165 * locals.var_ac41_dn7)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign14400_e20163 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign14400_e20165 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn11) * locals.var_ac41) + (assign14400_e20163 * locals.var_ac41_dn11)) * locals.var_ac41) + (assign14400_e20165 * locals.var_ac41_dn11)), (((((8.0 * locals.var_ac41_dn12) * locals.var_ac41) + (assign14400_e20163 * locals.var_ac41_dn12)) * locals.var_ac41) + (assign14400_e20165 * locals.var_ac41_dn12)), (((((8.0 * locals.var_ac41_dn17) * locals.var_ac41) + (assign14400_e20163 * locals.var_ac41_dn17)) * locals.var_ac41) + (assign14400_e20165 * locals.var_ac41_dn17)),)
    } else {
        (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn6, locals.var_ac4_dn7, locals.var_ac4_dn10, locals.var_ac4_dn11, locals.var_ac4_dn12, locals.var_ac4_dn17,)
    }
};
        locals.var_ac4 = assign14400_e20169;
        locals.var_ac4_dn0 = assign14400_e20169_d_n0;
        locals.var_ac4_dn2 = assign14400_e20169_d_n2;
        locals.var_ac4_dn6 = assign14400_e20169_d_n6;
        locals.var_ac4_dn7 = assign14400_e20169_d_n7;
        locals.var_ac4_dn10 = assign14400_e20169_d_n10;
        locals.var_ac4_dn11 = assign14400_e20169_d_n11;
        locals.var_ac4_dn12 = assign14400_e20169_d_n12;
        locals.var_ac4_dn17 = assign14400_e20169_d_n17;

        let (assign14410_e20178, assign14410_e20178_d_n0, assign14410_e20178_d_n2, assign14410_e20178_d_n6, assign14410_e20178_d_n7, assign14410_e20178_d_n10, assign14410_e20178_d_n11, assign14410_e20178_d_n12, assign14410_e20178_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14410_e20176: f64 = (locals.var_tx - 2.0);
        (assign14410_e20176, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign14410_e20178;
        locals.var_t4_dn0 = assign14410_e20178_d_n0;
        locals.var_t4_dn2 = assign14410_e20178_d_n2;
        locals.var_t4_dn6 = assign14410_e20178_d_n6;
        locals.var_t4_dn7 = assign14410_e20178_d_n7;
        locals.var_t4_dn10 = assign14410_e20178_d_n10;
        locals.var_t4_dn11 = assign14410_e20178_d_n11;
        locals.var_t4_dn12 = assign14410_e20178_d_n12;
        locals.var_t4_dn17 = assign14410_e20178_d_n17;

        let (assign14420_e20189, assign14420_e20189_d_n0, assign14420_e20189_d_n2, assign14420_e20189_d_n6, assign14420_e20189_d_n7, assign14420_e20189_d_n10, assign14420_e20189_d_n11, assign14420_e20189_d_n12, assign14420_e20189_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14420_e20185: f64 = (9.0 * locals.var_ty);
        let assign14420_e20187: f64 = (assign14420_e20185 * locals.var_t4);
        (assign14420_e20187, (((9.0 * locals.var_ty_dn0) * locals.var_t4) + (assign14420_e20185 * locals.var_t4_dn0)), (((9.0 * locals.var_ty_dn2) * locals.var_t4) + (assign14420_e20185 * locals.var_t4_dn2)), (((9.0 * locals.var_ty_dn6) * locals.var_t4) + (assign14420_e20185 * locals.var_t4_dn6)), (((9.0 * locals.var_ty_dn7) * locals.var_t4) + (assign14420_e20185 * locals.var_t4_dn7)), (((9.0 * locals.var_ty_dn10) * locals.var_t4) + (assign14420_e20185 * locals.var_t4_dn10)), (((9.0 * locals.var_ty_dn11) * locals.var_t4) + (assign14420_e20185 * locals.var_t4_dn11)), (((9.0 * locals.var_ty_dn12) * locals.var_t4) + (assign14420_e20185 * locals.var_t4_dn12)), (((9.0 * locals.var_ty_dn17) * locals.var_t4) + (assign14420_e20185 * locals.var_t4_dn17)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign14420_e20189;
        locals.var_t5_dn0 = assign14420_e20189_d_n0;
        locals.var_t5_dn2 = assign14420_e20189_d_n2;
        locals.var_t5_dn6 = assign14420_e20189_d_n6;
        locals.var_t5_dn7 = assign14420_e20189_d_n7;
        locals.var_t5_dn10 = assign14420_e20189_d_n10;
        locals.var_t5_dn11 = assign14420_e20189_d_n11;
        locals.var_t5_dn12 = assign14420_e20189_d_n12;
        locals.var_t5_dn17 = assign14420_e20189_d_n17;

        let (assign14430_e20200, assign14430_e20200_d_n0, assign14430_e20200_d_n2, assign14430_e20200_d_n6, assign14430_e20200_d_n7, assign14430_e20200_d_n10, assign14430_e20200_d_n11, assign14430_e20200_d_n12, assign14430_e20200_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14430_e20196: f64 = (7.0 * 1.414213562373095);
        let assign14430_e20198: f64 = (assign14430_e20196 - locals.var_t5);
        (assign14430_e20198, (-locals.var_t5_dn0), (-locals.var_t5_dn2), (-locals.var_t5_dn6), (-locals.var_t5_dn7), (-locals.var_t5_dn10), (-locals.var_t5_dn11), (-locals.var_t5_dn12), (-locals.var_t5_dn17),)
    } else {
        (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn6, locals.var_ac31_dn7, locals.var_ac31_dn10, locals.var_ac31_dn11, locals.var_ac31_dn12, locals.var_ac31_dn17,)
    }
};
        locals.var_ac31 = assign14430_e20200;
        locals.var_ac31_dn0 = assign14430_e20200_d_n0;
        locals.var_ac31_dn2 = assign14430_e20200_d_n2;
        locals.var_ac31_dn6 = assign14430_e20200_d_n6;
        locals.var_ac31_dn7 = assign14430_e20200_d_n7;
        locals.var_ac31_dn10 = assign14430_e20200_d_n10;
        locals.var_ac31_dn11 = assign14430_e20200_d_n11;
        locals.var_ac31_dn12 = assign14430_e20200_d_n12;
        locals.var_ac31_dn17 = assign14430_e20200_d_n17;

        let (assign14440_e20209, assign14440_e20209_d_n0, assign14440_e20209_d_n2, assign14440_e20209_d_n6, assign14440_e20209_d_n7, assign14440_e20209_d_n10, assign14440_e20209_d_n11, assign14440_e20209_d_n12, assign14440_e20209_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14440_e20207: f64 = (locals.var_ac31 * locals.var_ac31);
        (assign14440_e20207, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn7 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn7)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn11 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn11)), ((locals.var_ac31_dn12 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn12)), ((locals.var_ac31_dn17 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn17)),)
    } else {
        (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn6, locals.var_ac3_dn7, locals.var_ac3_dn10, locals.var_ac3_dn11, locals.var_ac3_dn12, locals.var_ac3_dn17,)
    }
};
        locals.var_ac3 = assign14440_e20209;
        locals.var_ac3_dn0 = assign14440_e20209_d_n0;
        locals.var_ac3_dn2 = assign14440_e20209_d_n2;
        locals.var_ac3_dn6 = assign14440_e20209_d_n6;
        locals.var_ac3_dn7 = assign14440_e20209_d_n7;
        locals.var_ac3_dn10 = assign14440_e20209_d_n10;
        locals.var_ac3_dn11 = assign14440_e20209_d_n11;
        locals.var_ac3_dn12 = assign14440_e20209_d_n12;
        locals.var_ac3_dn17 = assign14440_e20209_d_n17;

        let assign14450_e20213: f64 = (locals.var_ac3 * 1e-8);
        let assign14450_e20214: f64 = if locals.var_ac4 < assign14450_e20213 { 1.0 } else { 0.0 };
        locals.var_guard443 = assign14450_e20214;

        let (assign14460_e20236, assign14460_e20236_d_n0, assign14460_e20236_d_n2, assign14460_e20236_d_n6, assign14460_e20236_d_n7, assign14460_e20236_d_n10, assign14460_e20236_d_n11, assign14460_e20236_d_n12, assign14460_e20236_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) && (locals.var_guard443 != 0.0)) {
        let assign14460_e20222: f64 = (-7.0);
        let assign14460_e20224: f64 = (assign14460_e20222 * 1.414213562373095);
        let assign14460_e20226: f64 = (assign14460_e20224 + locals.var_ac31);
        let assign14460_e20229: f64 = (0.5 * locals.var_ac4);
        let assign14460_e20231: f64 = (assign14460_e20229 / locals.var_ac31);
        let assign14460_e20232: f64 = (assign14460_e20226 + assign14460_e20231);
        let assign14460_e20234: f64 = (assign14460_e20232 + locals.var_t5);
        (assign14460_e20234, ((locals.var_ac31_dn0 + ((((0.5 * locals.var_ac4_dn0) * locals.var_ac31) - (assign14460_e20229 * locals.var_ac31_dn0)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn0), ((locals.var_ac31_dn2 + ((((0.5 * locals.var_ac4_dn2) * locals.var_ac31) - (assign14460_e20229 * locals.var_ac31_dn2)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn2), ((locals.var_ac31_dn6 + ((((0.5 * locals.var_ac4_dn6) * locals.var_ac31) - (assign14460_e20229 * locals.var_ac31_dn6)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn6), ((locals.var_ac31_dn7 + ((((0.5 * locals.var_ac4_dn7) * locals.var_ac31) - (assign14460_e20229 * locals.var_ac31_dn7)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn7), ((locals.var_ac31_dn10 + ((((0.5 * locals.var_ac4_dn10) * locals.var_ac31) - (assign14460_e20229 * locals.var_ac31_dn10)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn10), ((locals.var_ac31_dn11 + ((((0.5 * locals.var_ac4_dn11) * locals.var_ac31) - (assign14460_e20229 * locals.var_ac31_dn11)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn11), ((locals.var_ac31_dn12 + ((((0.5 * locals.var_ac4_dn12) * locals.var_ac31) - (assign14460_e20229 * locals.var_ac31_dn12)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn12), ((locals.var_ac31_dn17 + ((((0.5 * locals.var_ac4_dn17) * locals.var_ac31) - (assign14460_e20229 * locals.var_ac31_dn17)) / (locals.var_ac31 * locals.var_ac31))) + locals.var_t5_dn17),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn12, locals.var_ac1_dn17,)
    }
};
        locals.var_ac1 = assign14460_e20236;
        locals.var_ac1_dn0 = assign14460_e20236_d_n0;
        locals.var_ac1_dn2 = assign14460_e20236_d_n2;
        locals.var_ac1_dn6 = assign14460_e20236_d_n6;
        locals.var_ac1_dn7 = assign14460_e20236_d_n7;
        locals.var_ac1_dn10 = assign14460_e20236_d_n10;
        locals.var_ac1_dn11 = assign14460_e20236_d_n11;
        locals.var_ac1_dn12 = assign14460_e20236_d_n12;
        locals.var_ac1_dn17 = assign14460_e20236_d_n17;

        let (assign14470_e20249, assign14470_e20249_d_n0, assign14470_e20249_d_n2, assign14470_e20249_d_n6, assign14470_e20249_d_n7, assign14470_e20249_d_n10, assign14470_e20249_d_n11, assign14470_e20249_d_n12, assign14470_e20249_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) && (locals.var_guard443 == 0.0)) {
        let assign14470_e20246: f64 = (locals.var_ac4 + locals.var_ac3);
        let assign14470_e20247: f64 = (assign14470_e20246).sqrt();
        (assign14470_e20247, ((locals.var_ac4_dn0 + locals.var_ac3_dn0) / (2.0 * assign14470_e20247)), ((locals.var_ac4_dn2 + locals.var_ac3_dn2) / (2.0 * assign14470_e20247)), ((locals.var_ac4_dn6 + locals.var_ac3_dn6) / (2.0 * assign14470_e20247)), ((locals.var_ac4_dn7 + locals.var_ac3_dn7) / (2.0 * assign14470_e20247)), ((locals.var_ac4_dn10 + locals.var_ac3_dn10) / (2.0 * assign14470_e20247)), ((locals.var_ac4_dn11 + locals.var_ac3_dn11) / (2.0 * assign14470_e20247)), ((locals.var_ac4_dn12 + locals.var_ac3_dn12) / (2.0 * assign14470_e20247)), ((locals.var_ac4_dn17 + locals.var_ac3_dn17) / (2.0 * assign14470_e20247)),)
    } else {
        (locals.var_ac2, locals.var_ac2_dn0, locals.var_ac2_dn2, locals.var_ac2_dn6, locals.var_ac2_dn7, locals.var_ac2_dn10, locals.var_ac2_dn11, locals.var_ac2_dn12, locals.var_ac2_dn17,)
    }
};
        locals.var_ac2 = assign14470_e20249;
        locals.var_ac2_dn0 = assign14470_e20249_d_n0;
        locals.var_ac2_dn2 = assign14470_e20249_d_n2;
        locals.var_ac2_dn6 = assign14470_e20249_d_n6;
        locals.var_ac2_dn7 = assign14470_e20249_d_n7;
        locals.var_ac2_dn10 = assign14470_e20249_d_n10;
        locals.var_ac2_dn11 = assign14470_e20249_d_n11;
        locals.var_ac2_dn12 = assign14470_e20249_d_n12;
        locals.var_ac2_dn17 = assign14470_e20249_d_n17;

        let (assign14480_e20266, assign14480_e20266_d_n0, assign14480_e20266_d_n2, assign14480_e20266_d_n6, assign14480_e20266_d_n7, assign14480_e20266_d_n10, assign14480_e20266_d_n11, assign14480_e20266_d_n12, assign14480_e20266_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) && (locals.var_guard443 == 0.0)) {
        let assign14480_e20258: f64 = (-7.0);
        let assign14480_e20260: f64 = (assign14480_e20258 * 1.414213562373095);
        let assign14480_e20262: f64 = (assign14480_e20260 + locals.var_ac2);
        let assign14480_e20264: f64 = (assign14480_e20262 + locals.var_t5);
        (assign14480_e20264, (locals.var_ac2_dn0 + locals.var_t5_dn0), (locals.var_ac2_dn2 + locals.var_t5_dn2), (locals.var_ac2_dn6 + locals.var_t5_dn6), (locals.var_ac2_dn7 + locals.var_t5_dn7), (locals.var_ac2_dn10 + locals.var_t5_dn10), (locals.var_ac2_dn11 + locals.var_t5_dn11), (locals.var_ac2_dn12 + locals.var_t5_dn12), (locals.var_ac2_dn17 + locals.var_t5_dn17),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn12, locals.var_ac1_dn17,)
    }
};
        locals.var_ac1 = assign14480_e20266;
        locals.var_ac1_dn0 = assign14480_e20266_d_n0;
        locals.var_ac1_dn2 = assign14480_e20266_d_n2;
        locals.var_ac1_dn6 = assign14480_e20266_d_n6;
        locals.var_ac1_dn7 = assign14480_e20266_d_n7;
        locals.var_ac1_dn10 = assign14480_e20266_d_n10;
        locals.var_ac1_dn11 = assign14480_e20266_d_n11;
        locals.var_ac1_dn12 = assign14480_e20266_d_n12;
        locals.var_ac1_dn17 = assign14480_e20266_d_n17;

        let (assign14490_e20275, assign14490_e20275_d_n0, assign14490_e20275_d_n2, assign14490_e20275_d_n6, assign14490_e20275_d_n7, assign14490_e20275_d_n10, assign14490_e20275_d_n11, assign14490_e20275_d_n12, assign14490_e20275_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14490_e20273: f64 = (locals.var_ac1).powf(0.3333333333333333);
        (assign14490_e20273, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn0)) } } else { (assign14490_e20273 * (0.3333333333333333 * (locals.var_ac1_dn0 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn2)) } } else { (assign14490_e20273 * (0.3333333333333333 * (locals.var_ac1_dn2 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn6)) } } else { (assign14490_e20273 * (0.3333333333333333 * (locals.var_ac1_dn6 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn7)) } } else { (assign14490_e20273 * (0.3333333333333333 * (locals.var_ac1_dn7 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn10)) } } else { (assign14490_e20273 * (0.3333333333333333 * (locals.var_ac1_dn10 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn11)) } } else { (assign14490_e20273 * (0.3333333333333333 * (locals.var_ac1_dn11 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn12)) } } else { (assign14490_e20273 * (0.3333333333333333 * (locals.var_ac1_dn12 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn17)) } } else { (assign14490_e20273 * (0.3333333333333333 * (locals.var_ac1_dn17 / locals.var_ac1))) },)
    } else {
        (locals.var_acd, locals.var_acd_dn0, locals.var_acd_dn2, locals.var_acd_dn6, locals.var_acd_dn7, locals.var_acd_dn10, locals.var_acd_dn11, locals.var_acd_dn12, locals.var_acd_dn17,)
    }
};
        locals.var_acd = assign14490_e20275;
        locals.var_acd_dn0 = assign14490_e20275_d_n0;
        locals.var_acd_dn2 = assign14490_e20275_d_n2;
        locals.var_acd_dn6 = assign14490_e20275_d_n6;
        locals.var_acd_dn7 = assign14490_e20275_d_n7;
        locals.var_acd_dn10 = assign14490_e20275_d_n10;
        locals.var_acd_dn11 = assign14490_e20275_d_n11;
        locals.var_acd_dn12 = assign14490_e20275_d_n12;
        locals.var_acd_dn17 = assign14490_e20275_d_n17;

        let (assign14500_e20299, assign14500_e20299_d_n0, assign14500_e20299_d_n2, assign14500_e20299_d_n6, assign14500_e20299_d_n7, assign14500_e20299_d_n10, assign14500_e20299_d_n11, assign14500_e20299_d_n12, assign14500_e20299_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14500_e20281: f64 = (-4.0);
        let assign14500_e20283: f64 = (assign14500_e20281 * 1.414213562373095);
        let assign14500_e20286: f64 = (12.0 * locals.var_ty);
        let assign14500_e20287: f64 = (assign14500_e20283 - assign14500_e20286);
        let assign14500_e20290: f64 = (2.0 * locals.var_acd);
        let assign14500_e20291: f64 = (assign14500_e20287 + assign14500_e20290);
        let assign14500_e20294: f64 = (1.414213562373095 * locals.var_acd);
        let assign14500_e20296: f64 = (assign14500_e20294 * locals.var_acd);
        let assign14500_e20297: f64 = (assign14500_e20291 + assign14500_e20296);
        (assign14500_e20297, (((-(12.0 * locals.var_ty_dn0)) + (2.0 * locals.var_acd_dn0)) + (((1.414213562373095 * locals.var_acd_dn0) * locals.var_acd) + (assign14500_e20294 * locals.var_acd_dn0))), (((-(12.0 * locals.var_ty_dn2)) + (2.0 * locals.var_acd_dn2)) + (((1.414213562373095 * locals.var_acd_dn2) * locals.var_acd) + (assign14500_e20294 * locals.var_acd_dn2))), (((-(12.0 * locals.var_ty_dn6)) + (2.0 * locals.var_acd_dn6)) + (((1.414213562373095 * locals.var_acd_dn6) * locals.var_acd) + (assign14500_e20294 * locals.var_acd_dn6))), (((-(12.0 * locals.var_ty_dn7)) + (2.0 * locals.var_acd_dn7)) + (((1.414213562373095 * locals.var_acd_dn7) * locals.var_acd) + (assign14500_e20294 * locals.var_acd_dn7))), (((-(12.0 * locals.var_ty_dn10)) + (2.0 * locals.var_acd_dn10)) + (((1.414213562373095 * locals.var_acd_dn10) * locals.var_acd) + (assign14500_e20294 * locals.var_acd_dn10))), (((-(12.0 * locals.var_ty_dn11)) + (2.0 * locals.var_acd_dn11)) + (((1.414213562373095 * locals.var_acd_dn11) * locals.var_acd) + (assign14500_e20294 * locals.var_acd_dn11))), (((-(12.0 * locals.var_ty_dn12)) + (2.0 * locals.var_acd_dn12)) + (((1.414213562373095 * locals.var_acd_dn12) * locals.var_acd) + (assign14500_e20294 * locals.var_acd_dn12))), (((-(12.0 * locals.var_ty_dn17)) + (2.0 * locals.var_acd_dn17)) + (((1.414213562373095 * locals.var_acd_dn17) * locals.var_acd) + (assign14500_e20294 * locals.var_acd_dn17))),)
    } else {
        (locals.var_acn, locals.var_acn_dn0, locals.var_acn_dn2, locals.var_acn_dn6, locals.var_acn_dn7, locals.var_acn_dn10, locals.var_acn_dn11, locals.var_acn_dn12, locals.var_acn_dn17,)
    }
};
        locals.var_acn = assign14500_e20299;
        locals.var_acn_dn0 = assign14500_e20299_d_n0;
        locals.var_acn_dn2 = assign14500_e20299_d_n2;
        locals.var_acn_dn6 = assign14500_e20299_d_n6;
        locals.var_acn_dn7 = assign14500_e20299_d_n7;
        locals.var_acn_dn10 = assign14500_e20299_d_n10;
        locals.var_acn_dn11 = assign14500_e20299_d_n11;
        locals.var_acn_dn12 = assign14500_e20299_d_n12;
        locals.var_acn_dn17 = assign14500_e20299_d_n17;

        let (assign14510_e20308, assign14510_e20308_d_n0, assign14510_e20308_d_n2, assign14510_e20308_d_n6, assign14510_e20308_d_n7, assign14510_e20308_d_n10, assign14510_e20308_d_n11, assign14510_e20308_d_n12, assign14510_e20308_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14510_e20306: f64 = (1.0 / locals.var_acd);
        (assign14510_e20306, (-(locals.var_acd_dn0 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn2 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn6 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn7 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn10 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn11 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn12 / (locals.var_acd * locals.var_acd))), (-(locals.var_acd_dn17 / (locals.var_acd * locals.var_acd))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign14510_e20308;
        locals.var_t1_dn0 = assign14510_e20308_d_n0;
        locals.var_t1_dn2 = assign14510_e20308_d_n2;
        locals.var_t1_dn6 = assign14510_e20308_d_n6;
        locals.var_t1_dn7 = assign14510_e20308_d_n7;
        locals.var_t1_dn10 = assign14510_e20308_d_n10;
        locals.var_t1_dn11 = assign14510_e20308_d_n11;
        locals.var_t1_dn12 = assign14510_e20308_d_n12;
        locals.var_t1_dn17 = assign14510_e20308_d_n17;

        let (assign14520_e20317, assign14520_e20317_d_n0, assign14520_e20317_d_n2, assign14520_e20317_d_n6, assign14520_e20317_d_n7, assign14520_e20317_d_n10, assign14520_e20317_d_n11, assign14520_e20317_d_n12, assign14520_e20317_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14520_e20315: f64 = (locals.var_acn * locals.var_t1);
        (assign14520_e20315, ((locals.var_acn_dn0 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn0)), ((locals.var_acn_dn2 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn2)), ((locals.var_acn_dn6 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn6)), ((locals.var_acn_dn7 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn7)), ((locals.var_acn_dn10 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn10)), ((locals.var_acn_dn11 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn11)), ((locals.var_acn_dn12 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn12)), ((locals.var_acn_dn17 * locals.var_t1) + (locals.var_acn * locals.var_t1_dn17)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, locals.var_chi_dn17,)
    }
};
        locals.var_chi = assign14520_e20317;
        locals.var_chi_dn0 = assign14520_e20317_d_n0;
        locals.var_chi_dn2 = assign14520_e20317_d_n2;
        locals.var_chi_dn6 = assign14520_e20317_d_n6;
        locals.var_chi_dn7 = assign14520_e20317_d_n7;
        locals.var_chi_dn10 = assign14520_e20317_d_n10;
        locals.var_chi_dn11 = assign14520_e20317_d_n11;
        locals.var_chi_dn12 = assign14520_e20317_d_n12;
        locals.var_chi_dn17 = assign14520_e20317_d_n17;

        let (assign14530_e20328, assign14530_e20328_d_n0, assign14530_e20328_d_n2, assign14530_e20328_d_n6, assign14530_e20328_d_n7, assign14530_e20328_d_n10, assign14530_e20328_d_n11, assign14530_e20328_d_n12, assign14530_e20328_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14530_e20324: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign14530_e20326: f64 = (assign14530_e20324 + locals.var_vbcs_cl);
        (assign14530_e20326, ((locals.var_chi_dn0 * locals.var_beta_inv) + locals.var_vbcs_cl_dn0), ((locals.var_chi_dn2 * locals.var_beta_inv) + locals.var_vbcs_cl_dn2), ((locals.var_chi_dn6 * locals.var_beta_inv) + locals.var_vbcs_cl_dn6), ((locals.var_chi_dn7 * locals.var_beta_inv) + locals.var_vbcs_cl_dn7), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) + locals.var_vbcs_cl_dn10), ((locals.var_chi_dn11 * locals.var_beta_inv) + locals.var_vbcs_cl_dn11), ((locals.var_chi_dn12 * locals.var_beta_inv) + locals.var_vbcs_cl_dn12), ((locals.var_chi_dn17 * locals.var_beta_inv) + locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_psa, locals.var_psa_dn0, locals.var_psa_dn2, locals.var_psa_dn6, locals.var_psa_dn7, locals.var_psa_dn10, locals.var_psa_dn11, locals.var_psa_dn12, locals.var_psa_dn17,)
    }
};
        locals.var_psa = assign14530_e20328;
        locals.var_psa_dn0 = assign14530_e20328_d_n0;
        locals.var_psa_dn2 = assign14530_e20328_d_n2;
        locals.var_psa_dn6 = assign14530_e20328_d_n6;
        locals.var_psa_dn7 = assign14530_e20328_d_n7;
        locals.var_psa_dn10 = assign14530_e20328_d_n10;
        locals.var_psa_dn11 = assign14530_e20328_d_n11;
        locals.var_psa_dn12 = assign14530_e20328_d_n12;
        locals.var_psa_dn17 = assign14530_e20328_d_n17;

        let (assign14540_e20337, assign14540_e20337_d_n0, assign14540_e20337_d_n2, assign14540_e20337_d_n6, assign14540_e20337_d_n7, assign14540_e20337_d_n10, assign14540_e20337_d_n11, assign14540_e20337_d_n12, assign14540_e20337_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14540_e20335: f64 = (locals.var_psa - locals.var_vbcs_cl);
        (assign14540_e20335, (locals.var_psa_dn0 - locals.var_vbcs_cl_dn0), (locals.var_psa_dn2 - locals.var_vbcs_cl_dn2), (locals.var_psa_dn6 - locals.var_vbcs_cl_dn6), (locals.var_psa_dn7 - locals.var_vbcs_cl_dn7), (locals.var_psa_dn10 - locals.var_vbcs_cl_dn10), (locals.var_psa_dn11 - locals.var_vbcs_cl_dn11), (locals.var_psa_dn12 - locals.var_vbcs_cl_dn12), (locals.var_psa_dn17 - locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign14540_e20337;
        locals.var_t1_dn0 = assign14540_e20337_d_n0;
        locals.var_t1_dn2 = assign14540_e20337_d_n2;
        locals.var_t1_dn6 = assign14540_e20337_d_n6;
        locals.var_t1_dn7 = assign14540_e20337_d_n7;
        locals.var_t1_dn10 = assign14540_e20337_d_n10;
        locals.var_t1_dn11 = assign14540_e20337_d_n11;
        locals.var_t1_dn12 = assign14540_e20337_d_n12;
        locals.var_t1_dn17 = assign14540_e20337_d_n17;

        let (assign14550_e20346, assign14550_e20346_d_n0, assign14550_e20346_d_n2, assign14550_e20346_d_n6, assign14550_e20346_d_n7, assign14550_e20346_d_n10, assign14550_e20346_d_n11, assign14550_e20346_d_n12, assign14550_e20346_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14550_e20344: f64 = (locals.var_t1 / locals.var_ps0_min);
        (assign14550_e20344, (((locals.var_t1_dn0 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn0)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn2 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn2)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn6 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn6)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn7 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn7)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn10 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn10)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn11 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn11)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn12 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn12)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn17 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn17)) / (locals.var_ps0_min * locals.var_ps0_min)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign14550_e20346;
        locals.var_t2_dn0 = assign14550_e20346_d_n0;
        locals.var_t2_dn2 = assign14550_e20346_d_n2;
        locals.var_t2_dn6 = assign14550_e20346_d_n6;
        locals.var_t2_dn7 = assign14550_e20346_d_n7;
        locals.var_t2_dn10 = assign14550_e20346_d_n10;
        locals.var_t2_dn11 = assign14550_e20346_d_n11;
        locals.var_t2_dn12 = assign14550_e20346_d_n12;
        locals.var_t2_dn17 = assign14550_e20346_d_n17;

        let (assign14560_e20358, assign14560_e20358_d_n0, assign14560_e20358_d_n2, assign14560_e20358_d_n6, assign14560_e20358_d_n7, assign14560_e20358_d_n10, assign14560_e20358_d_n11, assign14560_e20358_d_n12, assign14560_e20358_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14560_e20354: f64 = (locals.var_t2 * locals.var_t2);
        let assign14560_e20355: f64 = (1.0 + assign14560_e20354);
        let assign14560_e20356: f64 = (assign14560_e20355).sqrt();
        (assign14560_e20356, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign14560_e20356)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign14560_e20356)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign14560_e20356)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign14560_e20356)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign14560_e20356)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign14560_e20356)), (((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)) / (2.0 * assign14560_e20356)), (((locals.var_t2_dn17 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn17)) / (2.0 * assign14560_e20356)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign14560_e20358;
        locals.var_t3_dn0 = assign14560_e20358_d_n0;
        locals.var_t3_dn2 = assign14560_e20358_d_n2;
        locals.var_t3_dn6 = assign14560_e20358_d_n6;
        locals.var_t3_dn7 = assign14560_e20358_d_n7;
        locals.var_t3_dn10 = assign14560_e20358_d_n10;
        locals.var_t3_dn11 = assign14560_e20358_d_n11;
        locals.var_t3_dn12 = assign14560_e20358_d_n12;
        locals.var_t3_dn17 = assign14560_e20358_d_n17;

        let (assign14570_e20369, assign14570_e20369_d_n0, assign14570_e20369_d_n2, assign14570_e20369_d_n6, assign14570_e20369_d_n7, assign14570_e20369_d_n10, assign14570_e20369_d_n11, assign14570_e20369_d_n12, assign14570_e20369_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 != 0.0)) {
        let assign14570_e20365: f64 = (locals.var_t1 / locals.var_t3);
        let assign14570_e20367: f64 = (assign14570_e20365 + locals.var_vbcs_cl);
        (assign14570_e20367, ((((locals.var_t1_dn0 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn0), ((((locals.var_t1_dn2 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn2), ((((locals.var_t1_dn6 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn6), ((((locals.var_t1_dn7 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn7), ((((locals.var_t1_dn10 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn10), ((((locals.var_t1_dn11 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn11), ((((locals.var_t1_dn12 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn12)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn12), ((((locals.var_t1_dn17 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn17)) / (locals.var_t3 * locals.var_t3)) + locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    }
};
        locals.var_ps0 = assign14570_e20369;
        locals.var_ps0_dn0 = assign14570_e20369_d_n0;
        locals.var_ps0_dn2 = assign14570_e20369_d_n2;
        locals.var_ps0_dn6 = assign14570_e20369_d_n6;
        locals.var_ps0_dn7 = assign14570_e20369_d_n7;
        locals.var_ps0_dn10 = assign14570_e20369_d_n10;
        locals.var_ps0_dn11 = assign14570_e20369_d_n11;
        locals.var_ps0_dn12 = assign14570_e20369_d_n12;
        locals.var_ps0_dn17 = assign14570_e20369_d_n17;

        let assign14580_e20372: f64 = if locals.var_flg_pprv >= 1.0 { 1.0 } else { 0.0 };
        locals.var_guard444 = assign14580_e20372;

        let (assign14590_e20382, assign14590_e20382_d_n0, assign14590_e20382_d_n2, assign14590_e20382_d_n6, assign14590_e20382_d_n7, assign14590_e20382_d_n10, assign14590_e20382_d_n11, assign14590_e20382_d_n12, assign14590_e20382_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 != 0.0)) {
        (locals.var_pss0_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn7, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12, locals.var_phi_s0_soi_dn17,)
    }
};
        locals.var_phi_s0_soi = assign14590_e20382;
        locals.var_phi_s0_soi_dn0 = assign14590_e20382_d_n0;
        locals.var_phi_s0_soi_dn2 = assign14590_e20382_d_n2;
        locals.var_phi_s0_soi_dn6 = assign14590_e20382_d_n6;
        locals.var_phi_s0_soi_dn7 = assign14590_e20382_d_n7;
        locals.var_phi_s0_soi_dn10 = assign14590_e20382_d_n10;
        locals.var_phi_s0_soi_dn11 = assign14590_e20382_d_n11;
        locals.var_phi_s0_soi_dn12 = assign14590_e20382_d_n12;
        locals.var_phi_s0_soi_dn17 = assign14590_e20382_d_n17;

        let (assign14600_e20392, assign14600_e20392_d_n0, assign14600_e20392_d_n2, assign14600_e20392_d_n6, assign14600_e20392_d_n7, assign14600_e20392_d_n10, assign14600_e20392_d_n11, assign14600_e20392_d_n12, assign14600_e20392_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 != 0.0)) {
        (locals.var_pss0_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign14600_e20392;
        locals.var_ps0_ini_dn0 = assign14600_e20392_d_n0;
        locals.var_ps0_ini_dn2 = assign14600_e20392_d_n2;
        locals.var_ps0_ini_dn6 = assign14600_e20392_d_n6;
        locals.var_ps0_ini_dn7 = assign14600_e20392_d_n7;
        locals.var_ps0_ini_dn10 = assign14600_e20392_d_n10;
        locals.var_ps0_ini_dn11 = assign14600_e20392_d_n11;
        locals.var_ps0_ini_dn12 = assign14600_e20392_d_n12;
        locals.var_ps0_ini_dn17 = assign14600_e20392_d_n17;

        let (assign14610_e20417, assign14610_e20417_d_n0, assign14610_e20417_d_n2, assign14610_e20417_d_n6, assign14610_e20417_d_n7, assign14610_e20417_d_n10, assign14610_e20417_d_n11, assign14610_e20417_d_n12, assign14610_e20417_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) {
        let assign14610_e20406: f64 = (locals.var_vgp - locals.var_vbcs_cl);
        let assign14610_e20407: f64 = (locals.var_beta * assign14610_e20406);
        let assign14610_e20409: f64 = (assign14610_e20407 - 1.0);
        let assign14610_e20410: f64 = (4.0 * assign14610_e20409);
        let assign14610_e20413: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign14610_e20414: f64 = (assign14610_e20410 / assign14610_e20413);
        let assign14610_e20415: f64 = (1.0 + assign14610_e20414);
        (assign14610_e20415, ((((4.0 * (locals.var_beta * (locals.var_vgp_dn0 - locals.var_vbcs_cl_dn0))) * assign14610_e20413) - (assign14610_e20410 * (locals.var_fac1p2_dn0 * locals.var_beta2))) / (assign14610_e20413 * assign14610_e20413)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn2 - locals.var_vbcs_cl_dn2))) * assign14610_e20413) - (assign14610_e20410 * (locals.var_fac1p2_dn2 * locals.var_beta2))) / (assign14610_e20413 * assign14610_e20413)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn6 - locals.var_vbcs_cl_dn6))) * assign14610_e20413) - (assign14610_e20410 * (locals.var_fac1p2_dn6 * locals.var_beta2))) / (assign14610_e20413 * assign14610_e20413)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn7 - locals.var_vbcs_cl_dn7))) * assign14610_e20413) - (assign14610_e20410 * (locals.var_fac1p2_dn7 * locals.var_beta2))) / (assign14610_e20413 * assign14610_e20413)), ((((4.0 * ((locals.var_beta_dn10 * assign14610_e20406) + (locals.var_beta * (locals.var_vgp_dn10 - locals.var_vbcs_cl_dn10)))) * assign14610_e20413) - (assign14610_e20410 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign14610_e20413 * assign14610_e20413)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn11 - locals.var_vbcs_cl_dn11))) * assign14610_e20413) - (assign14610_e20410 * (locals.var_fac1p2_dn11 * locals.var_beta2))) / (assign14610_e20413 * assign14610_e20413)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn12 - locals.var_vbcs_cl_dn12))) * assign14610_e20413) - (assign14610_e20410 * (locals.var_fac1p2_dn12 * locals.var_beta2))) / (assign14610_e20413 * assign14610_e20413)), ((((4.0 * (locals.var_beta * (locals.var_vgp_dn17 - locals.var_vbcs_cl_dn17))) * assign14610_e20413) - (assign14610_e20410 * (locals.var_fac1p2_dn17 * locals.var_beta2))) / (assign14610_e20413 * assign14610_e20413)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign14610_e20417;
        locals.var_tx_dn0 = assign14610_e20417_d_n0;
        locals.var_tx_dn2 = assign14610_e20417_d_n2;
        locals.var_tx_dn6 = assign14610_e20417_d_n6;
        locals.var_tx_dn7 = assign14610_e20417_d_n7;
        locals.var_tx_dn10 = assign14610_e20417_d_n10;
        locals.var_tx_dn11 = assign14610_e20417_d_n11;
        locals.var_tx_dn12 = assign14610_e20417_d_n12;
        locals.var_tx_dn17 = assign14610_e20417_d_n17;

        let (assign14620_e20437, assign14620_e20437_d_n0, assign14620_e20437_d_n2, assign14620_e20437_d_n6, assign14620_e20437_d_n7, assign14620_e20437_d_n10, assign14620_e20437_d_n11, assign14620_e20437_d_n12, assign14620_e20437_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) {
        let assign14620_e20429: f64 = (10.0 * 2.220446049250313e-16);
        let (assign14620_e20435, assign14620_e20435_d_n0, assign14620_e20435_d_n2, assign14620_e20435_d_n6, assign14620_e20435_d_n7, assign14620_e20435_d_n10, assign14620_e20435_d_n11, assign14620_e20435_d_n12, assign14620_e20435_d_n17,) = {
            if (locals.var_tx >= assign14620_e20429) {
                (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
            } else {
                let assign14620_e20434: f64 = (10.0 * 2.220446049250313e-16);
                (assign14620_e20434, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign14620_e20435, assign14620_e20435_d_n0, assign14620_e20435_d_n2, assign14620_e20435_d_n6, assign14620_e20435_d_n7, assign14620_e20435_d_n10, assign14620_e20435_d_n11, assign14620_e20435_d_n12, assign14620_e20435_d_n17,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign14620_e20437;
        locals.var_tx_dn0 = assign14620_e20437_d_n0;
        locals.var_tx_dn2 = assign14620_e20437_d_n2;
        locals.var_tx_dn6 = assign14620_e20437_d_n6;
        locals.var_tx_dn7 = assign14620_e20437_d_n7;
        locals.var_tx_dn10 = assign14620_e20437_d_n10;
        locals.var_tx_dn11 = assign14620_e20437_d_n11;
        locals.var_tx_dn12 = assign14620_e20437_d_n12;
        locals.var_tx_dn17 = assign14620_e20437_d_n17;

        let (assign14630_e20459, assign14630_e20459_d_n0, assign14630_e20459_d_n2, assign14630_e20459_d_n6, assign14630_e20459_d_n7, assign14630_e20459_d_n10, assign14630_e20459_d_n11, assign14630_e20459_d_n12, assign14630_e20459_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) {
        let assign14630_e20449: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign14630_e20451: f64 = (assign14630_e20449 * 0.5);
        let assign14630_e20454: f64 = (locals.var_tx).sqrt();
        let assign14630_e20455: f64 = (1.0 - assign14630_e20454);
        let assign14630_e20456: f64 = (assign14630_e20451 * assign14630_e20455);
        let assign14630_e20457: f64 = (locals.var_vgp + assign14630_e20456);
        (assign14630_e20457, (locals.var_vgp_dn0 + ((((locals.var_fac1p2_dn0 * locals.var_beta) * 0.5) * assign14630_e20455) + (assign14630_e20451 * (-(locals.var_tx_dn0 / (2.0 * assign14630_e20454)))))), (locals.var_vgp_dn2 + ((((locals.var_fac1p2_dn2 * locals.var_beta) * 0.5) * assign14630_e20455) + (assign14630_e20451 * (-(locals.var_tx_dn2 / (2.0 * assign14630_e20454)))))), (locals.var_vgp_dn6 + ((((locals.var_fac1p2_dn6 * locals.var_beta) * 0.5) * assign14630_e20455) + (assign14630_e20451 * (-(locals.var_tx_dn6 / (2.0 * assign14630_e20454)))))), (locals.var_vgp_dn7 + ((((locals.var_fac1p2_dn7 * locals.var_beta) * 0.5) * assign14630_e20455) + (assign14630_e20451 * (-(locals.var_tx_dn7 / (2.0 * assign14630_e20454)))))), (locals.var_vgp_dn10 + (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) * 0.5) * assign14630_e20455) + (assign14630_e20451 * (-(locals.var_tx_dn10 / (2.0 * assign14630_e20454)))))), (locals.var_vgp_dn11 + ((((locals.var_fac1p2_dn11 * locals.var_beta) * 0.5) * assign14630_e20455) + (assign14630_e20451 * (-(locals.var_tx_dn11 / (2.0 * assign14630_e20454)))))), (locals.var_vgp_dn12 + ((((locals.var_fac1p2_dn12 * locals.var_beta) * 0.5) * assign14630_e20455) + (assign14630_e20451 * (-(locals.var_tx_dn12 / (2.0 * assign14630_e20454)))))), (locals.var_vgp_dn17 + ((((locals.var_fac1p2_dn17 * locals.var_beta) * 0.5) * assign14630_e20455) + (assign14630_e20451 * (-(locals.var_tx_dn17 / (2.0 * assign14630_e20454)))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign14630_e20459;
        locals.var_ps0_inia_dn0 = assign14630_e20459_d_n0;
        locals.var_ps0_inia_dn2 = assign14630_e20459_d_n2;
        locals.var_ps0_inia_dn6 = assign14630_e20459_d_n6;
        locals.var_ps0_inia_dn7 = assign14630_e20459_d_n7;
        locals.var_ps0_inia_dn10 = assign14630_e20459_d_n10;
        locals.var_ps0_inia_dn11 = assign14630_e20459_d_n11;
        locals.var_ps0_inia_dn12 = assign14630_e20459_d_n12;
        locals.var_ps0_inia_dn17 = assign14630_e20459_d_n17;

    }

    pub(super) fn stamp_transient_block_47(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14640_e20474, assign14640_e20474_d_n0, assign14640_e20474_d_n2, assign14640_e20474_d_n6, assign14640_e20474_d_n7, assign14640_e20474_d_n10, assign14640_e20474_d_n11, assign14640_e20474_d_n12, assign14640_e20474_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) {
        let assign14640_e20471: f64 = (locals.var_ps0_inia - locals.var_vbcs_cl);
        let assign14640_e20472: f64 = (locals.var_beta * assign14640_e20471);
        (assign14640_e20472, (locals.var_beta * (locals.var_ps0_inia_dn0 - locals.var_vbcs_cl_dn0)), (locals.var_beta * (locals.var_ps0_inia_dn2 - locals.var_vbcs_cl_dn2)), (locals.var_beta * (locals.var_ps0_inia_dn6 - locals.var_vbcs_cl_dn6)), (locals.var_beta * (locals.var_ps0_inia_dn7 - locals.var_vbcs_cl_dn7)), ((locals.var_beta_dn10 * assign14640_e20471) + (locals.var_beta * (locals.var_ps0_inia_dn10 - locals.var_vbcs_cl_dn10))), (locals.var_beta * (locals.var_ps0_inia_dn11 - locals.var_vbcs_cl_dn11)), (locals.var_beta * (locals.var_ps0_inia_dn12 - locals.var_vbcs_cl_dn12)), (locals.var_beta * (locals.var_ps0_inia_dn17 - locals.var_vbcs_cl_dn17)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn12, locals.var_chi_dn17,)
    }
};
        locals.var_chi = assign14640_e20474;
        locals.var_chi_dn0 = assign14640_e20474_d_n0;
        locals.var_chi_dn2 = assign14640_e20474_d_n2;
        locals.var_chi_dn6 = assign14640_e20474_d_n6;
        locals.var_chi_dn7 = assign14640_e20474_d_n7;
        locals.var_chi_dn10 = assign14640_e20474_d_n10;
        locals.var_chi_dn11 = assign14640_e20474_d_n11;
        locals.var_chi_dn12 = assign14640_e20474_d_n12;
        locals.var_chi_dn17 = assign14640_e20474_d_n17;

        let assign14650_e20477: f64 = if locals.var_chi < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard445 = assign14650_e20477;

        let (assign14660_e20494, assign14660_e20494_d_n0, assign14660_e20494_d_n2, assign14660_e20494_d_n6, assign14660_e20494_d_n7, assign14660_e20494_d_n10, assign14660_e20494_d_n11, assign14660_e20494_d_n12, assign14660_e20494_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 != 0.0)) {
        let assign14660_e20491: f64 = (locals.var_vgp - locals.var_vbcs_cl);
        let assign14660_e20492: f64 = (locals.var_beta * assign14660_e20491);
        (assign14660_e20492, (locals.var_beta * (locals.var_vgp_dn0 - locals.var_vbcs_cl_dn0)), (locals.var_beta * (locals.var_vgp_dn2 - locals.var_vbcs_cl_dn2)), (locals.var_beta * (locals.var_vgp_dn6 - locals.var_vbcs_cl_dn6)), (locals.var_beta * (locals.var_vgp_dn7 - locals.var_vbcs_cl_dn7)), ((locals.var_beta_dn10 * assign14660_e20491) + (locals.var_beta * (locals.var_vgp_dn10 - locals.var_vbcs_cl_dn10))), (locals.var_beta * (locals.var_vgp_dn11 - locals.var_vbcs_cl_dn11)), (locals.var_beta * (locals.var_vgp_dn12 - locals.var_vbcs_cl_dn12)), (locals.var_beta * (locals.var_vgp_dn17 - locals.var_vbcs_cl_dn17)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn12, locals.var_ty_dn17,)
    }
};
        locals.var_ty = assign14660_e20494;
        locals.var_ty_dn0 = assign14660_e20494_d_n0;
        locals.var_ty_dn2 = assign14660_e20494_d_n2;
        locals.var_ty_dn6 = assign14660_e20494_d_n6;
        locals.var_ty_dn7 = assign14660_e20494_d_n7;
        locals.var_ty_dn10 = assign14660_e20494_d_n10;
        locals.var_ty_dn11 = assign14660_e20494_d_n11;
        locals.var_ty_dn12 = assign14660_e20494_d_n12;
        locals.var_ty_dn17 = assign14660_e20494_d_n17;

        let (assign14670_e20515, assign14670_e20515_d_n0, assign14670_e20515_d_n2, assign14670_e20515_d_n6, assign14670_e20515_d_n7, assign14670_e20515_d_n10, assign14670_e20515_d_n11, assign14670_e20515_d_n12, assign14670_e20515_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 != 0.0)) {
        let assign14670_e20508: f64 = (1.414213562373095 / 108.0);
        let assign14670_e20510: f64 = (assign14670_e20508 * locals.var_beta);
        let assign14670_e20512: f64 = (assign14670_e20510 * locals.var_fac1);
        let assign14670_e20513: f64 = (1.0 / assign14670_e20512);
        (assign14670_e20513, (-((assign14670_e20510 * locals.var_fac1_dn0) / (assign14670_e20512 * assign14670_e20512))), (-((assign14670_e20510 * locals.var_fac1_dn2) / (assign14670_e20512 * assign14670_e20512))), (-((assign14670_e20510 * locals.var_fac1_dn6) / (assign14670_e20512 * assign14670_e20512))), (-((assign14670_e20510 * locals.var_fac1_dn7) / (assign14670_e20512 * assign14670_e20512))), (-((((assign14670_e20508 * locals.var_beta_dn10) * locals.var_fac1) + (assign14670_e20510 * locals.var_fac1_dn10)) / (assign14670_e20512 * assign14670_e20512))), (-((assign14670_e20510 * locals.var_fac1_dn11) / (assign14670_e20512 * assign14670_e20512))), (-((assign14670_e20510 * locals.var_fac1_dn12) / (assign14670_e20512 * assign14670_e20512))), (-((assign14670_e20510 * locals.var_fac1_dn17) / (assign14670_e20512 * assign14670_e20512))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign14670_e20515;
        locals.var_t1_dn0 = assign14670_e20515_d_n0;
        locals.var_t1_dn2 = assign14670_e20515_d_n2;
        locals.var_t1_dn6 = assign14670_e20515_d_n6;
        locals.var_t1_dn7 = assign14670_e20515_d_n7;
        locals.var_t1_dn10 = assign14670_e20515_d_n10;
        locals.var_t1_dn11 = assign14670_e20515_d_n11;
        locals.var_t1_dn12 = assign14670_e20515_d_n12;
        locals.var_t1_dn17 = assign14670_e20515_d_n17;

        let (assign14680_e20532, assign14680_e20532_d_n0, assign14680_e20532_d_n2, assign14680_e20532_d_n6, assign14680_e20532_d_n7, assign14680_e20532_d_n10, assign14680_e20532_d_n11, assign14680_e20532_d_n12, assign14680_e20532_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 != 0.0)) {
        let assign14680_e20529: f64 = (3.0 * locals.var_t1);
        let assign14680_e20530: f64 = (81.0 + assign14680_e20529);
        (assign14680_e20530, (3.0 * locals.var_t1_dn0), (3.0 * locals.var_t1_dn2), (3.0 * locals.var_t1_dn6), (3.0 * locals.var_t1_dn7), (3.0 * locals.var_t1_dn10), (3.0 * locals.var_t1_dn11), (3.0 * locals.var_t1_dn12), (3.0 * locals.var_t1_dn17),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign14680_e20532;
        locals.var_t2_dn0 = assign14680_e20532_d_n0;
        locals.var_t2_dn2 = assign14680_e20532_d_n2;
        locals.var_t2_dn6 = assign14680_e20532_d_n6;
        locals.var_t2_dn7 = assign14680_e20532_d_n7;
        locals.var_t2_dn10 = assign14680_e20532_d_n10;
        locals.var_t2_dn11 = assign14680_e20532_d_n11;
        locals.var_t2_dn12 = assign14680_e20532_d_n12;
        locals.var_t2_dn17 = assign14680_e20532_d_n17;

        let (assign14690_e20556, assign14690_e20556_d_n0, assign14690_e20556_d_n2, assign14690_e20556_d_n6, assign14690_e20556_d_n7, assign14690_e20556_d_n10, assign14690_e20556_d_n11, assign14690_e20556_d_n12, assign14690_e20556_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 != 0.0)) {
        let assign14690_e20544: f64 = (-2916.0);
        let assign14690_e20547: f64 = (81.0 * locals.var_t1);
        let assign14690_e20548: f64 = (assign14690_e20544 - assign14690_e20547);
        let assign14690_e20551: f64 = (27.0 * locals.var_t1);
        let assign14690_e20553: f64 = (assign14690_e20551 * locals.var_ty);
        let assign14690_e20554: f64 = (assign14690_e20548 + assign14690_e20553);
        (assign14690_e20554, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign14690_e20551 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign14690_e20551 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign14690_e20551 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign14690_e20551 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign14690_e20551 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn11)) + (((27.0 * locals.var_t1_dn11) * locals.var_ty) + (assign14690_e20551 * locals.var_ty_dn11))), ((-(81.0 * locals.var_t1_dn12)) + (((27.0 * locals.var_t1_dn12) * locals.var_ty) + (assign14690_e20551 * locals.var_ty_dn12))), ((-(81.0 * locals.var_t1_dn17)) + (((27.0 * locals.var_t1_dn17) * locals.var_ty) + (assign14690_e20551 * locals.var_ty_dn17))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign14690_e20556;
        locals.var_t3_dn0 = assign14690_e20556_d_n0;
        locals.var_t3_dn2 = assign14690_e20556_d_n2;
        locals.var_t3_dn6 = assign14690_e20556_d_n6;
        locals.var_t3_dn7 = assign14690_e20556_d_n7;
        locals.var_t3_dn10 = assign14690_e20556_d_n10;
        locals.var_t3_dn11 = assign14690_e20556_d_n11;
        locals.var_t3_dn12 = assign14690_e20556_d_n12;
        locals.var_t3_dn17 = assign14690_e20556_d_n17;

        let (assign14700_e20581, assign14700_e20581_d_n0, assign14700_e20581_d_n2, assign14700_e20581_d_n6, assign14700_e20581_d_n7, assign14700_e20581_d_n10, assign14700_e20581_d_n11, assign14700_e20581_d_n12, assign14700_e20581_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 != 0.0)) {
        let assign14700_e20571: f64 = (54.0 + locals.var_t1);
        let assign14700_e20572: f64 = (81.0 * assign14700_e20571);
        let assign14700_e20573: f64 = (1458.0 - assign14700_e20572);
        let assign14700_e20576: f64 = (27.0 * locals.var_t1);
        let assign14700_e20578: f64 = (assign14700_e20576 * locals.var_ty);
        let assign14700_e20579: f64 = (assign14700_e20573 + assign14700_e20578);
        (assign14700_e20579, ((-(81.0 * locals.var_t1_dn0)) + (((27.0 * locals.var_t1_dn0) * locals.var_ty) + (assign14700_e20576 * locals.var_ty_dn0))), ((-(81.0 * locals.var_t1_dn2)) + (((27.0 * locals.var_t1_dn2) * locals.var_ty) + (assign14700_e20576 * locals.var_ty_dn2))), ((-(81.0 * locals.var_t1_dn6)) + (((27.0 * locals.var_t1_dn6) * locals.var_ty) + (assign14700_e20576 * locals.var_ty_dn6))), ((-(81.0 * locals.var_t1_dn7)) + (((27.0 * locals.var_t1_dn7) * locals.var_ty) + (assign14700_e20576 * locals.var_ty_dn7))), ((-(81.0 * locals.var_t1_dn10)) + (((27.0 * locals.var_t1_dn10) * locals.var_ty) + (assign14700_e20576 * locals.var_ty_dn10))), ((-(81.0 * locals.var_t1_dn11)) + (((27.0 * locals.var_t1_dn11) * locals.var_ty) + (assign14700_e20576 * locals.var_ty_dn11))), ((-(81.0 * locals.var_t1_dn12)) + (((27.0 * locals.var_t1_dn12) * locals.var_ty) + (assign14700_e20576 * locals.var_ty_dn12))), ((-(81.0 * locals.var_t1_dn17)) + (((27.0 * locals.var_t1_dn17) * locals.var_ty) + (assign14700_e20576 * locals.var_ty_dn17))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign14700_e20581;
        locals.var_t4_dn0 = assign14700_e20581_d_n0;
        locals.var_t4_dn2 = assign14700_e20581_d_n2;
        locals.var_t4_dn6 = assign14700_e20581_d_n6;
        locals.var_t4_dn7 = assign14700_e20581_d_n7;
        locals.var_t4_dn10 = assign14700_e20581_d_n10;
        locals.var_t4_dn11 = assign14700_e20581_d_n11;
        locals.var_t4_dn12 = assign14700_e20581_d_n12;
        locals.var_t4_dn17 = assign14700_e20581_d_n17;

        let (assign14710_e20596, assign14710_e20596_d_n0, assign14710_e20596_d_n2, assign14710_e20596_d_n6, assign14710_e20596_d_n7, assign14710_e20596_d_n10, assign14710_e20596_d_n11, assign14710_e20596_d_n12, assign14710_e20596_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 != 0.0)) {
        let assign14710_e20594: f64 = (locals.var_t4 * locals.var_t4);
        (assign14710_e20594, ((locals.var_t4_dn0 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn0)), ((locals.var_t4_dn2 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn2)), ((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)), ((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)), ((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)), ((locals.var_t4_dn11 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn11)), ((locals.var_t4_dn12 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn12)), ((locals.var_t4_dn17 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn17)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign14710_e20596;
        locals.var_t4_dn0 = assign14710_e20596_d_n0;
        locals.var_t4_dn2 = assign14710_e20596_d_n2;
        locals.var_t4_dn6 = assign14710_e20596_d_n6;
        locals.var_t4_dn7 = assign14710_e20596_d_n7;
        locals.var_t4_dn10 = assign14710_e20596_d_n10;
        locals.var_t4_dn11 = assign14710_e20596_d_n11;
        locals.var_t4_dn12 = assign14710_e20596_d_n12;
        locals.var_t4_dn17 = assign14710_e20596_d_n17;

        let (assign14720_e20622, assign14720_e20622_d_n0, assign14720_e20622_d_n2, assign14720_e20622_d_n6, assign14720_e20622_d_n7, assign14720_e20622_d_n10, assign14720_e20622_d_n11, assign14720_e20622_d_n12, assign14720_e20622_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 != 0.0)) {
        let assign14720_e20610: f64 = (4.0 * locals.var_t2);
        let assign14720_e20612: f64 = (assign14720_e20610 * locals.var_t2);
        let assign14720_e20614: f64 = (assign14720_e20612 * locals.var_t2);
        let assign14720_e20616: f64 = (assign14720_e20614 + locals.var_t4);
        let assign14720_e20617: f64 = (assign14720_e20616).sqrt();
        let assign14720_e20618: f64 = (locals.var_t3 + assign14720_e20617);
        let assign14720_e20620: f64 = (assign14720_e20618).powf(0.3333333333333333);
        (assign14720_e20620, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14720_e20618).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign14720_e20610 * locals.var_t2_dn0)) * locals.var_t2) + (assign14720_e20612 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign14720_e20617))))) } } else { (assign14720_e20620 * (0.3333333333333333 * ((locals.var_t3_dn0 + (((((((4.0 * locals.var_t2_dn0) * locals.var_t2) + (assign14720_e20610 * locals.var_t2_dn0)) * locals.var_t2) + (assign14720_e20612 * locals.var_t2_dn0)) + locals.var_t4_dn0) / (2.0 * assign14720_e20617))) / assign14720_e20618))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14720_e20618).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign14720_e20610 * locals.var_t2_dn2)) * locals.var_t2) + (assign14720_e20612 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign14720_e20617))))) } } else { (assign14720_e20620 * (0.3333333333333333 * ((locals.var_t3_dn2 + (((((((4.0 * locals.var_t2_dn2) * locals.var_t2) + (assign14720_e20610 * locals.var_t2_dn2)) * locals.var_t2) + (assign14720_e20612 * locals.var_t2_dn2)) + locals.var_t4_dn2) / (2.0 * assign14720_e20617))) / assign14720_e20618))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14720_e20618).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign14720_e20610 * locals.var_t2_dn6)) * locals.var_t2) + (assign14720_e20612 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign14720_e20617))))) } } else { (assign14720_e20620 * (0.3333333333333333 * ((locals.var_t3_dn6 + (((((((4.0 * locals.var_t2_dn6) * locals.var_t2) + (assign14720_e20610 * locals.var_t2_dn6)) * locals.var_t2) + (assign14720_e20612 * locals.var_t2_dn6)) + locals.var_t4_dn6) / (2.0 * assign14720_e20617))) / assign14720_e20618))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14720_e20618).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign14720_e20610 * locals.var_t2_dn7)) * locals.var_t2) + (assign14720_e20612 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign14720_e20617))))) } } else { (assign14720_e20620 * (0.3333333333333333 * ((locals.var_t3_dn7 + (((((((4.0 * locals.var_t2_dn7) * locals.var_t2) + (assign14720_e20610 * locals.var_t2_dn7)) * locals.var_t2) + (assign14720_e20612 * locals.var_t2_dn7)) + locals.var_t4_dn7) / (2.0 * assign14720_e20617))) / assign14720_e20618))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14720_e20618).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign14720_e20610 * locals.var_t2_dn10)) * locals.var_t2) + (assign14720_e20612 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign14720_e20617))))) } } else { (assign14720_e20620 * (0.3333333333333333 * ((locals.var_t3_dn10 + (((((((4.0 * locals.var_t2_dn10) * locals.var_t2) + (assign14720_e20610 * locals.var_t2_dn10)) * locals.var_t2) + (assign14720_e20612 * locals.var_t2_dn10)) + locals.var_t4_dn10) / (2.0 * assign14720_e20617))) / assign14720_e20618))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14720_e20618).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn11 + (((((((4.0 * locals.var_t2_dn11) * locals.var_t2) + (assign14720_e20610 * locals.var_t2_dn11)) * locals.var_t2) + (assign14720_e20612 * locals.var_t2_dn11)) + locals.var_t4_dn11) / (2.0 * assign14720_e20617))))) } } else { (assign14720_e20620 * (0.3333333333333333 * ((locals.var_t3_dn11 + (((((((4.0 * locals.var_t2_dn11) * locals.var_t2) + (assign14720_e20610 * locals.var_t2_dn11)) * locals.var_t2) + (assign14720_e20612 * locals.var_t2_dn11)) + locals.var_t4_dn11) / (2.0 * assign14720_e20617))) / assign14720_e20618))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14720_e20618).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn12 + (((((((4.0 * locals.var_t2_dn12) * locals.var_t2) + (assign14720_e20610 * locals.var_t2_dn12)) * locals.var_t2) + (assign14720_e20612 * locals.var_t2_dn12)) + locals.var_t4_dn12) / (2.0 * assign14720_e20617))))) } } else { (assign14720_e20620 * (0.3333333333333333 * ((locals.var_t3_dn12 + (((((((4.0 * locals.var_t2_dn12) * locals.var_t2) + (assign14720_e20610 * locals.var_t2_dn12)) * locals.var_t2) + (assign14720_e20612 * locals.var_t2_dn12)) + locals.var_t4_dn12) / (2.0 * assign14720_e20617))) / assign14720_e20618))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign14720_e20618).powf(0.3333333333333333 - 1.0) * (locals.var_t3_dn17 + (((((((4.0 * locals.var_t2_dn17) * locals.var_t2) + (assign14720_e20610 * locals.var_t2_dn17)) * locals.var_t2) + (assign14720_e20612 * locals.var_t2_dn17)) + locals.var_t4_dn17) / (2.0 * assign14720_e20617))))) } } else { (assign14720_e20620 * (0.3333333333333333 * ((locals.var_t3_dn17 + (((((((4.0 * locals.var_t2_dn17) * locals.var_t2) + (assign14720_e20610 * locals.var_t2_dn17)) * locals.var_t2) + (assign14720_e20612 * locals.var_t2_dn17)) + locals.var_t4_dn17) / (2.0 * assign14720_e20617))) / assign14720_e20618))) },)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn17,)
    }
};
        locals.var_t5 = assign14720_e20622;
        locals.var_t5_dn0 = assign14720_e20622_d_n0;
        locals.var_t5_dn2 = assign14720_e20622_d_n2;
        locals.var_t5_dn6 = assign14720_e20622_d_n6;
        locals.var_t5_dn7 = assign14720_e20622_d_n7;
        locals.var_t5_dn10 = assign14720_e20622_d_n10;
        locals.var_t5_dn11 = assign14720_e20622_d_n11;
        locals.var_t5_dn12 = assign14720_e20622_d_n12;
        locals.var_t5_dn17 = assign14720_e20622_d_n17;

        let (assign14730_e20651, assign14730_e20651_d_n0, assign14730_e20651_d_n2, assign14730_e20651_d_n6, assign14730_e20651_d_n7, assign14730_e20651_d_n10, assign14730_e20651_d_n11, assign14730_e20651_d_n12, assign14730_e20651_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 != 0.0)) {
        let assign14730_e20636: f64 = (1.259921049894873 * locals.var_t2);
        let assign14730_e20639: f64 = (3.0 * locals.var_t5);
        let assign14730_e20640: f64 = (assign14730_e20636 / assign14730_e20639);
        let assign14730_e20641: f64 = (3.0 - assign14730_e20640);
        let assign14730_e20645: f64 = (3.0 * 1.259921049894873);
        let assign14730_e20646: f64 = (1.0 / assign14730_e20645);
        let assign14730_e20648: f64 = (assign14730_e20646 * locals.var_t5);
        let assign14730_e20649: f64 = (assign14730_e20641 + assign14730_e20648);
        (assign14730_e20649, ((-((((1.259921049894873 * locals.var_t2_dn0) * assign14730_e20639) - (assign14730_e20636 * (3.0 * locals.var_t5_dn0))) / (assign14730_e20639 * assign14730_e20639))) + (assign14730_e20646 * locals.var_t5_dn0)), ((-((((1.259921049894873 * locals.var_t2_dn2) * assign14730_e20639) - (assign14730_e20636 * (3.0 * locals.var_t5_dn2))) / (assign14730_e20639 * assign14730_e20639))) + (assign14730_e20646 * locals.var_t5_dn2)), ((-((((1.259921049894873 * locals.var_t2_dn6) * assign14730_e20639) - (assign14730_e20636 * (3.0 * locals.var_t5_dn6))) / (assign14730_e20639 * assign14730_e20639))) + (assign14730_e20646 * locals.var_t5_dn6)), ((-((((1.259921049894873 * locals.var_t2_dn7) * assign14730_e20639) - (assign14730_e20636 * (3.0 * locals.var_t5_dn7))) / (assign14730_e20639 * assign14730_e20639))) + (assign14730_e20646 * locals.var_t5_dn7)), ((-((((1.259921049894873 * locals.var_t2_dn10) * assign14730_e20639) - (assign14730_e20636 * (3.0 * locals.var_t5_dn10))) / (assign14730_e20639 * assign14730_e20639))) + (assign14730_e20646 * locals.var_t5_dn10)), ((-((((1.259921049894873 * locals.var_t2_dn11) * assign14730_e20639) - (assign14730_e20636 * (3.0 * locals.var_t5_dn11))) / (assign14730_e20639 * assign14730_e20639))) + (assign14730_e20646 * locals.var_t5_dn11)), ((-((((1.259921049894873 * locals.var_t2_dn12) * assign14730_e20639) - (assign14730_e20636 * (3.0 * locals.var_t5_dn12))) / (assign14730_e20639 * assign14730_e20639))) + (assign14730_e20646 * locals.var_t5_dn12)), ((-((((1.259921049894873 * locals.var_t2_dn17) * assign14730_e20639) - (assign14730_e20636 * (3.0 * locals.var_t5_dn17))) / (assign14730_e20639 * assign14730_e20639))) + (assign14730_e20646 * locals.var_t5_dn17)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign14730_e20651;
        locals.var_tx_dn0 = assign14730_e20651_d_n0;
        locals.var_tx_dn2 = assign14730_e20651_d_n2;
        locals.var_tx_dn6 = assign14730_e20651_d_n6;
        locals.var_tx_dn7 = assign14730_e20651_d_n7;
        locals.var_tx_dn10 = assign14730_e20651_d_n10;
        locals.var_tx_dn11 = assign14730_e20651_d_n11;
        locals.var_tx_dn12 = assign14730_e20651_d_n12;
        locals.var_tx_dn17 = assign14730_e20651_d_n17;

        let (assign14740_e20668, assign14740_e20668_d_n0, assign14740_e20668_d_n2, assign14740_e20668_d_n6, assign14740_e20668_d_n7, assign14740_e20668_d_n10, assign14740_e20668_d_n11, assign14740_e20668_d_n12, assign14740_e20668_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 != 0.0)) {
        let assign14740_e20664: f64 = (locals.var_tx * locals.var_beta_inv);
        let assign14740_e20666: f64 = (assign14740_e20664 + locals.var_vbcs_cl);
        (assign14740_e20666, ((locals.var_tx_dn0 * locals.var_beta_inv) + locals.var_vbcs_cl_dn0), ((locals.var_tx_dn2 * locals.var_beta_inv) + locals.var_vbcs_cl_dn2), ((locals.var_tx_dn6 * locals.var_beta_inv) + locals.var_vbcs_cl_dn6), ((locals.var_tx_dn7 * locals.var_beta_inv) + locals.var_vbcs_cl_dn7), (((locals.var_tx_dn10 * locals.var_beta_inv) + (locals.var_tx * locals.var_beta_inv_dn10)) + locals.var_vbcs_cl_dn10), ((locals.var_tx_dn11 * locals.var_beta_inv) + locals.var_vbcs_cl_dn11), ((locals.var_tx_dn12 * locals.var_beta_inv) + locals.var_vbcs_cl_dn12), ((locals.var_tx_dn17 * locals.var_beta_inv) + locals.var_vbcs_cl_dn17),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    }
};
        locals.var_ps0_inia = assign14740_e20668;
        locals.var_ps0_inia_dn0 = assign14740_e20668_d_n0;
        locals.var_ps0_inia_dn2 = assign14740_e20668_d_n2;
        locals.var_ps0_inia_dn6 = assign14740_e20668_d_n6;
        locals.var_ps0_inia_dn7 = assign14740_e20668_d_n7;
        locals.var_ps0_inia_dn10 = assign14740_e20668_d_n10;
        locals.var_ps0_inia_dn11 = assign14740_e20668_d_n11;
        locals.var_ps0_inia_dn12 = assign14740_e20668_d_n12;
        locals.var_ps0_inia_dn17 = assign14740_e20668_d_n17;

        let (assign14750_e20681, assign14750_e20681_d_n0, assign14750_e20681_d_n2, assign14750_e20681_d_n6, assign14750_e20681_d_n7, assign14750_e20681_d_n10, assign14750_e20681_d_n11, assign14750_e20681_d_n12, assign14750_e20681_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign14750_e20681;
        locals.var_ps0_ini_dn0 = assign14750_e20681_d_n0;
        locals.var_ps0_ini_dn2 = assign14750_e20681_d_n2;
        locals.var_ps0_ini_dn6 = assign14750_e20681_d_n6;
        locals.var_ps0_ini_dn7 = assign14750_e20681_d_n7;
        locals.var_ps0_ini_dn10 = assign14750_e20681_d_n10;
        locals.var_ps0_ini_dn11 = assign14750_e20681_d_n11;
        locals.var_ps0_ini_dn12 = assign14750_e20681_d_n12;
        locals.var_ps0_ini_dn17 = assign14750_e20681_d_n17;

        let assign14760_e20684: f64 = if locals.var_vgs <= locals.var_vth { 1.0 } else { 0.0 };
        locals.var_guard446 = assign14760_e20684;

        let (assign14770_e20700, assign14770_e20700_d_n0, assign14770_e20700_d_n2, assign14770_e20700_d_n6, assign14770_e20700_d_n7, assign14770_e20700_d_n10, assign14770_e20700_d_n11, assign14770_e20700_d_n12, assign14770_e20700_d_n17,) = {
    if (((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 == 0.0)) && (locals.var_guard446 != 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign14770_e20700;
        locals.var_ps0_ini_dn0 = assign14770_e20700_d_n0;
        locals.var_ps0_ini_dn2 = assign14770_e20700_d_n2;
        locals.var_ps0_ini_dn6 = assign14770_e20700_d_n6;
        locals.var_ps0_ini_dn7 = assign14770_e20700_d_n7;
        locals.var_ps0_ini_dn10 = assign14770_e20700_d_n10;
        locals.var_ps0_ini_dn11 = assign14770_e20700_d_n11;
        locals.var_ps0_ini_dn12 = assign14770_e20700_d_n12;
        locals.var_ps0_ini_dn17 = assign14770_e20700_d_n17;

        let (assign14780_e20721, assign14780_e20721_d_n0, assign14780_e20721_d_n2, assign14780_e20721_d_n6, assign14780_e20721_d_n7, assign14780_e20721_d_n10, assign14780_e20721_d_n11, assign14780_e20721_d_n12, assign14780_e20721_d_n17,) = {
    if (((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 == 0.0)) && (locals.var_guard446 == 0.0)) {
        let assign14780_e20717: f64 = (1.0 / locals.var_cnst1soi);
        let assign14780_e20719: f64 = (assign14780_e20717 / locals.var_cnstc_foxi);
        (assign14780_e20719, ((((-(locals.var_cnst1soi_dn0 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14780_e20717 * locals.var_cnstc_foxi_dn0)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn2 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14780_e20717 * locals.var_cnstc_foxi_dn2)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn6 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14780_e20717 * locals.var_cnstc_foxi_dn6)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn7 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14780_e20717 * locals.var_cnstc_foxi_dn7)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn10 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14780_e20717 * locals.var_cnstc_foxi_dn10)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn11 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14780_e20717 * locals.var_cnstc_foxi_dn11)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn12 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14780_e20717 * locals.var_cnstc_foxi_dn12)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn17 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign14780_e20717 * locals.var_cnstc_foxi_dn17)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign14780_e20721;
        locals.var_t1_dn0 = assign14780_e20721_d_n0;
        locals.var_t1_dn2 = assign14780_e20721_d_n2;
        locals.var_t1_dn6 = assign14780_e20721_d_n6;
        locals.var_t1_dn7 = assign14780_e20721_d_n7;
        locals.var_t1_dn10 = assign14780_e20721_d_n10;
        locals.var_t1_dn11 = assign14780_e20721_d_n11;
        locals.var_t1_dn12 = assign14780_e20721_d_n12;
        locals.var_t1_dn17 = assign14780_e20721_d_n17;

        let (assign14790_e20742, assign14790_e20742_d_n0, assign14790_e20742_d_n2, assign14790_e20742_d_n6, assign14790_e20742_d_n7, assign14790_e20742_d_n10, assign14790_e20742_d_n11, assign14790_e20742_d_n12, assign14790_e20742_d_n17,) = {
    if (((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 == 0.0)) && (locals.var_guard446 == 0.0)) {
        let assign14790_e20738: f64 = (locals.var_t1 * locals.var_vgp);
        let assign14790_e20740: f64 = (assign14790_e20738 * locals.var_vgp);
        (assign14790_e20740, ((((locals.var_t1_dn0 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn0)) * locals.var_vgp) + (assign14790_e20738 * locals.var_vgp_dn0)), ((((locals.var_t1_dn2 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn2)) * locals.var_vgp) + (assign14790_e20738 * locals.var_vgp_dn2)), ((((locals.var_t1_dn6 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn6)) * locals.var_vgp) + (assign14790_e20738 * locals.var_vgp_dn6)), ((((locals.var_t1_dn7 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn7)) * locals.var_vgp) + (assign14790_e20738 * locals.var_vgp_dn7)), ((((locals.var_t1_dn10 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn10)) * locals.var_vgp) + (assign14790_e20738 * locals.var_vgp_dn10)), ((((locals.var_t1_dn11 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn11)) * locals.var_vgp) + (assign14790_e20738 * locals.var_vgp_dn11)), ((((locals.var_t1_dn12 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn12)) * locals.var_vgp) + (assign14790_e20738 * locals.var_vgp_dn12)), ((((locals.var_t1_dn17 * locals.var_vgp) + (locals.var_t1 * locals.var_vgp_dn17)) * locals.var_vgp) + (assign14790_e20738 * locals.var_vgp_dn17)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign14790_e20742;
        locals.var_t2_dn0 = assign14790_e20742_d_n0;
        locals.var_t2_dn2 = assign14790_e20742_d_n2;
        locals.var_t2_dn6 = assign14790_e20742_d_n6;
        locals.var_t2_dn7 = assign14790_e20742_d_n7;
        locals.var_t2_dn10 = assign14790_e20742_d_n10;
        locals.var_t2_dn11 = assign14790_e20742_d_n11;
        locals.var_t2_dn12 = assign14790_e20742_d_n12;
        locals.var_t2_dn17 = assign14790_e20742_d_n17;

        let (assign14800_e20763, assign14800_e20763_d_n0, assign14800_e20763_d_n2, assign14800_e20763_d_n6, assign14800_e20763_d_n7, assign14800_e20763_d_n10, assign14800_e20763_d_n11, assign14800_e20763_d_n12, assign14800_e20763_d_n17,) = {
    if (((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 == 0.0)) && (locals.var_guard446 == 0.0)) {
        let assign14800_e20760: f64 = (2.0 / locals.var_vgp);
        let assign14800_e20761: f64 = (locals.var_beta + assign14800_e20760);
        (assign14800_e20761, (-((2.0 * locals.var_vgp_dn0) / (locals.var_vgp * locals.var_vgp))), (-((2.0 * locals.var_vgp_dn2) / (locals.var_vgp * locals.var_vgp))), (-((2.0 * locals.var_vgp_dn6) / (locals.var_vgp * locals.var_vgp))), (-((2.0 * locals.var_vgp_dn7) / (locals.var_vgp * locals.var_vgp))), (locals.var_beta_dn10 + (-((2.0 * locals.var_vgp_dn10) / (locals.var_vgp * locals.var_vgp)))), (-((2.0 * locals.var_vgp_dn11) / (locals.var_vgp * locals.var_vgp))), (-((2.0 * locals.var_vgp_dn12) / (locals.var_vgp * locals.var_vgp))), (-((2.0 * locals.var_vgp_dn17) / (locals.var_vgp * locals.var_vgp))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign14800_e20763;
        locals.var_t3_dn0 = assign14800_e20763_d_n0;
        locals.var_t3_dn2 = assign14800_e20763_d_n2;
        locals.var_t3_dn6 = assign14800_e20763_d_n6;
        locals.var_t3_dn7 = assign14800_e20763_d_n7;
        locals.var_t3_dn10 = assign14800_e20763_d_n10;
        locals.var_t3_dn11 = assign14800_e20763_d_n11;
        locals.var_t3_dn12 = assign14800_e20763_d_n12;
        locals.var_t3_dn17 = assign14800_e20763_d_n17;

        let (assign14810_e20783, assign14810_e20783_d_n0, assign14810_e20783_d_n2, assign14810_e20783_d_n6, assign14810_e20783_d_n7, assign14810_e20783_d_n10, assign14810_e20783_d_n11, assign14810_e20783_d_n12, assign14810_e20783_d_n17,) = {
    if (((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 == 0.0)) && (locals.var_guard446 == 0.0)) {
        let assign14810_e20779: f64 = (locals.var_t2).ln();
        let assign14810_e20781: f64 = (assign14810_e20779 / locals.var_t3);
        (assign14810_e20781, ((((locals.var_t2_dn0 / locals.var_t2) * locals.var_t3) - (assign14810_e20779 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn2 / locals.var_t2) * locals.var_t3) - (assign14810_e20779 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn6 / locals.var_t2) * locals.var_t3) - (assign14810_e20779 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn7 / locals.var_t2) * locals.var_t3) - (assign14810_e20779 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn10 / locals.var_t2) * locals.var_t3) - (assign14810_e20779 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn11 / locals.var_t2) * locals.var_t3) - (assign14810_e20779 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn12 / locals.var_t2) * locals.var_t3) - (assign14810_e20779 * locals.var_t3_dn12)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn17 / locals.var_t2) * locals.var_t3) - (assign14810_e20779 * locals.var_t3_dn17)) / (locals.var_t3 * locals.var_t3)),)
    } else {
        (locals.var_ps0_inib, locals.var_ps0_inib_dn0, locals.var_ps0_inib_dn2, locals.var_ps0_inib_dn6, locals.var_ps0_inib_dn7, locals.var_ps0_inib_dn10, locals.var_ps0_inib_dn11, locals.var_ps0_inib_dn12, locals.var_ps0_inib_dn17,)
    }
};
        locals.var_ps0_inib = assign14810_e20783;
        locals.var_ps0_inib_dn0 = assign14810_e20783_d_n0;
        locals.var_ps0_inib_dn2 = assign14810_e20783_d_n2;
        locals.var_ps0_inib_dn6 = assign14810_e20783_d_n6;
        locals.var_ps0_inib_dn7 = assign14810_e20783_d_n7;
        locals.var_ps0_inib_dn10 = assign14810_e20783_d_n10;
        locals.var_ps0_inib_dn11 = assign14810_e20783_d_n11;
        locals.var_ps0_inib_dn12 = assign14810_e20783_d_n12;
        locals.var_ps0_inib_dn17 = assign14810_e20783_d_n17;

        let (assign14820_e20804, assign14820_e20804_d_n0, assign14820_e20804_d_n2, assign14820_e20804_d_n6, assign14820_e20804_d_n7, assign14820_e20804_d_n10, assign14820_e20804_d_n11, assign14820_e20804_d_n12, assign14820_e20804_d_n17,) = {
    if (((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 == 0.0)) && (locals.var_guard446 == 0.0)) {
        let assign14820_e20800: f64 = (locals.var_ps0_inib - locals.var_ps0_inia);
        let assign14820_e20802: f64 = (assign14820_e20800 - 0.0008);
        (assign14820_e20802, (locals.var_ps0_inib_dn0 - locals.var_ps0_inia_dn0), (locals.var_ps0_inib_dn2 - locals.var_ps0_inia_dn2), (locals.var_ps0_inib_dn6 - locals.var_ps0_inia_dn6), (locals.var_ps0_inib_dn7 - locals.var_ps0_inia_dn7), (locals.var_ps0_inib_dn10 - locals.var_ps0_inia_dn10), (locals.var_ps0_inib_dn11 - locals.var_ps0_inia_dn11), (locals.var_ps0_inib_dn12 - locals.var_ps0_inia_dn12), (locals.var_ps0_inib_dn17 - locals.var_ps0_inia_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign14820_e20804;
        locals.var_tmf1_dn0 = assign14820_e20804_d_n0;
        locals.var_tmf1_dn2 = assign14820_e20804_d_n2;
        locals.var_tmf1_dn6 = assign14820_e20804_d_n6;
        locals.var_tmf1_dn7 = assign14820_e20804_d_n7;
        locals.var_tmf1_dn10 = assign14820_e20804_d_n10;
        locals.var_tmf1_dn11 = assign14820_e20804_d_n11;
        locals.var_tmf1_dn12 = assign14820_e20804_d_n12;
        locals.var_tmf1_dn17 = assign14820_e20804_d_n17;

        let (assign14830_e20825, assign14830_e20825_d_n0, assign14830_e20825_d_n2, assign14830_e20825_d_n6, assign14830_e20825_d_n7, assign14830_e20825_d_n10, assign14830_e20825_d_n11, assign14830_e20825_d_n12, assign14830_e20825_d_n17,) = {
    if (((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 == 0.0)) && (locals.var_guard446 == 0.0)) {
        let assign14830_e20821: f64 = (4.0 * locals.var_ps0_inib);
        let assign14830_e20823: f64 = (assign14830_e20821 * 0.0008);
        (assign14830_e20823, ((4.0 * locals.var_ps0_inib_dn0) * 0.0008), ((4.0 * locals.var_ps0_inib_dn2) * 0.0008), ((4.0 * locals.var_ps0_inib_dn6) * 0.0008), ((4.0 * locals.var_ps0_inib_dn7) * 0.0008), ((4.0 * locals.var_ps0_inib_dn10) * 0.0008), ((4.0 * locals.var_ps0_inib_dn11) * 0.0008), ((4.0 * locals.var_ps0_inib_dn12) * 0.0008), ((4.0 * locals.var_ps0_inib_dn17) * 0.0008),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign14830_e20825;
        locals.var_tmf2_dn0 = assign14830_e20825_d_n0;
        locals.var_tmf2_dn2 = assign14830_e20825_d_n2;
        locals.var_tmf2_dn6 = assign14830_e20825_d_n6;
        locals.var_tmf2_dn7 = assign14830_e20825_d_n7;
        locals.var_tmf2_dn10 = assign14830_e20825_d_n10;
        locals.var_tmf2_dn11 = assign14830_e20825_d_n11;
        locals.var_tmf2_dn12 = assign14830_e20825_d_n12;
        locals.var_tmf2_dn17 = assign14830_e20825_d_n17;

        let (assign14840_e20848, assign14840_e20848_d_n0, assign14840_e20848_d_n2, assign14840_e20848_d_n6, assign14840_e20848_d_n7, assign14840_e20848_d_n10, assign14840_e20848_d_n11, assign14840_e20848_d_n12, assign14840_e20848_d_n17,) = {
    if (((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 == 0.0)) && (locals.var_guard446 == 0.0)) {
        let (assign14840_e20846, assign14840_e20846_d_n0, assign14840_e20846_d_n2, assign14840_e20846_d_n6, assign14840_e20846_d_n7, assign14840_e20846_d_n10, assign14840_e20846_d_n11, assign14840_e20846_d_n12, assign14840_e20846_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign14840_e20845: f64 = (-locals.var_tmf2);
                (assign14840_e20845, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign14840_e20846, assign14840_e20846_d_n0, assign14840_e20846_d_n2, assign14840_e20846_d_n6, assign14840_e20846_d_n7, assign14840_e20846_d_n10, assign14840_e20846_d_n11, assign14840_e20846_d_n12, assign14840_e20846_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign14840_e20848;
        locals.var_tmf2_dn0 = assign14840_e20848_d_n0;
        locals.var_tmf2_dn2 = assign14840_e20848_d_n2;
        locals.var_tmf2_dn6 = assign14840_e20848_d_n6;
        locals.var_tmf2_dn7 = assign14840_e20848_d_n7;
        locals.var_tmf2_dn10 = assign14840_e20848_d_n10;
        locals.var_tmf2_dn11 = assign14840_e20848_d_n11;
        locals.var_tmf2_dn12 = assign14840_e20848_d_n12;
        locals.var_tmf2_dn17 = assign14840_e20848_d_n17;

        let (assign14850_e20870, assign14850_e20870_d_n0, assign14850_e20870_d_n2, assign14850_e20870_d_n6, assign14850_e20870_d_n7, assign14850_e20870_d_n10, assign14850_e20870_d_n11, assign14850_e20870_d_n12, assign14850_e20870_d_n17,) = {
    if (((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 == 0.0)) && (locals.var_guard446 == 0.0)) {
        let assign14850_e20865: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14850_e20867: f64 = (assign14850_e20865 + locals.var_tmf2);
        let assign14850_e20868: f64 = (assign14850_e20867).sqrt();
        (assign14850_e20868, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14850_e20868)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14850_e20868)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14850_e20868)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14850_e20868)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14850_e20868)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign14850_e20868)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign14850_e20868)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign14850_e20868)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign14850_e20870;
        locals.var_tmf2_dn0 = assign14850_e20870_d_n0;
        locals.var_tmf2_dn2 = assign14850_e20870_d_n2;
        locals.var_tmf2_dn6 = assign14850_e20870_d_n6;
        locals.var_tmf2_dn7 = assign14850_e20870_d_n7;
        locals.var_tmf2_dn10 = assign14850_e20870_d_n10;
        locals.var_tmf2_dn11 = assign14850_e20870_d_n11;
        locals.var_tmf2_dn12 = assign14850_e20870_d_n12;
        locals.var_tmf2_dn17 = assign14850_e20870_d_n17;

        let (assign14860_e20893, assign14860_e20893_d_n0, assign14860_e20893_d_n2, assign14860_e20893_d_n6, assign14860_e20893_d_n7, assign14860_e20893_d_n10, assign14860_e20893_d_n11, assign14860_e20893_d_n12, assign14860_e20893_d_n17,) = {
    if (((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard445 == 0.0)) && (locals.var_guard446 == 0.0)) {
        let assign14860_e20889: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14860_e20890: f64 = (0.5 * assign14860_e20889);
        let assign14860_e20891: f64 = (locals.var_ps0_inib - assign14860_e20890);
        (assign14860_e20891, (locals.var_ps0_inib_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_ps0_inib_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_ps0_inib_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_ps0_inib_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_ps0_inib_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_ps0_inib_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_ps0_inib_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_ps0_inib_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign14860_e20893;
        locals.var_ps0_ini_dn0 = assign14860_e20893_d_n0;
        locals.var_ps0_ini_dn2 = assign14860_e20893_d_n2;
        locals.var_ps0_ini_dn6 = assign14860_e20893_d_n6;
        locals.var_ps0_ini_dn7 = assign14860_e20893_d_n7;
        locals.var_ps0_ini_dn10 = assign14860_e20893_d_n10;
        locals.var_ps0_ini_dn11 = assign14860_e20893_d_n11;
        locals.var_ps0_ini_dn12 = assign14860_e20893_d_n12;
        locals.var_ps0_ini_dn17 = assign14860_e20893_d_n17;

        let (assign14870_e20908, assign14870_e20908_d_n0, assign14870_e20908_d_n2, assign14870_e20908_d_n6, assign14870_e20908_d_n7, assign14870_e20908_d_n10, assign14870_e20908_d_n11, assign14870_e20908_d_n12, assign14870_e20908_d_n17,) = {
    if (((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) {
        let assign14870_e20905: f64 = (5e-12 / 2.0);
        let assign14870_e20906: f64 = (locals.var_vbcs_cl + assign14870_e20905);
        (assign14870_e20906, locals.var_vbcs_cl_dn0, locals.var_vbcs_cl_dn2, locals.var_vbcs_cl_dn6, locals.var_vbcs_cl_dn7, locals.var_vbcs_cl_dn10, locals.var_vbcs_cl_dn11, locals.var_vbcs_cl_dn12, locals.var_vbcs_cl_dn17,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    }
};
        locals.var_tx = assign14870_e20908;
        locals.var_tx_dn0 = assign14870_e20908_d_n0;
        locals.var_tx_dn2 = assign14870_e20908_d_n2;
        locals.var_tx_dn6 = assign14870_e20908_d_n6;
        locals.var_tx_dn7 = assign14870_e20908_d_n7;
        locals.var_tx_dn10 = assign14870_e20908_d_n10;
        locals.var_tx_dn11 = assign14870_e20908_d_n11;
        locals.var_tx_dn12 = assign14870_e20908_d_n12;
        locals.var_tx_dn17 = assign14870_e20908_d_n17;

        let assign14880_e20911: f64 = if locals.var_ps0_ini < locals.var_tx { 1.0 } else { 0.0 };
        locals.var_guard447 = assign14880_e20911;

        let (assign14890_e20924, assign14890_e20924_d_n0, assign14890_e20924_d_n2, assign14890_e20924_d_n6, assign14890_e20924_d_n7, assign14890_e20924_d_n10, assign14890_e20924_d_n11, assign14890_e20924_d_n12, assign14890_e20924_d_n17,) = {
    if ((((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) && (locals.var_guard444 == 0.0)) && (locals.var_guard447 != 0.0)) {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn12, locals.var_tx_dn17,)
    } else {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    }
};
        locals.var_ps0_ini = assign14890_e20924;
        locals.var_ps0_ini_dn0 = assign14890_e20924_d_n0;
        locals.var_ps0_ini_dn2 = assign14890_e20924_d_n2;
        locals.var_ps0_ini_dn6 = assign14890_e20924_d_n6;
        locals.var_ps0_ini_dn7 = assign14890_e20924_d_n7;
        locals.var_ps0_ini_dn10 = assign14890_e20924_d_n10;
        locals.var_ps0_ini_dn11 = assign14890_e20924_d_n11;
        locals.var_ps0_ini_dn12 = assign14890_e20924_d_n12;
        locals.var_ps0_ini_dn17 = assign14890_e20924_d_n17;

        let (assign14900_e20932, assign14900_e20932_d_n0, assign14900_e20932_d_n2, assign14900_e20932_d_n6, assign14900_e20932_d_n7, assign14900_e20932_d_n10, assign14900_e20932_d_n11, assign14900_e20932_d_n12, assign14900_e20932_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) {
        (locals.var_ps0_ini, locals.var_ps0_ini_dn0, locals.var_ps0_ini_dn2, locals.var_ps0_ini_dn6, locals.var_ps0_ini_dn7, locals.var_ps0_ini_dn10, locals.var_ps0_ini_dn11, locals.var_ps0_ini_dn12, locals.var_ps0_ini_dn17,)
    } else {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    }
};
        locals.var_ps0 = assign14900_e20932;
        locals.var_ps0_dn0 = assign14900_e20932_d_n0;
        locals.var_ps0_dn2 = assign14900_e20932_d_n2;
        locals.var_ps0_dn6 = assign14900_e20932_d_n6;
        locals.var_ps0_dn7 = assign14900_e20932_d_n7;
        locals.var_ps0_dn10 = assign14900_e20932_d_n10;
        locals.var_ps0_dn11 = assign14900_e20932_d_n11;
        locals.var_ps0_dn12 = assign14900_e20932_d_n12;
        locals.var_ps0_dn17 = assign14900_e20932_d_n17;

        let (assign14910_e20940, assign14910_e20940_d_n0, assign14910_e20940_d_n2, assign14910_e20940_d_n6, assign14910_e20940_d_n7, assign14910_e20940_d_n10, assign14910_e20940_d_n11, assign14910_e20940_d_n12, assign14910_e20940_d_n17,) = {
    if ((locals.var_guard111 == 0.0) && (locals.var_guard442 == 0.0)) {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn12, locals.var_ps0_inia_dn17,)
    } else {
        (locals.var_psl_lim, locals.var_psl_lim_dn0, locals.var_psl_lim_dn2, locals.var_psl_lim_dn6, locals.var_psl_lim_dn7, locals.var_psl_lim_dn10, locals.var_psl_lim_dn11, locals.var_psl_lim_dn12, locals.var_psl_lim_dn17,)
    }
};
        locals.var_psl_lim = assign14910_e20940;
        locals.var_psl_lim_dn0 = assign14910_e20940_d_n0;
        locals.var_psl_lim_dn2 = assign14910_e20940_d_n2;
        locals.var_psl_lim_dn6 = assign14910_e20940_d_n6;
        locals.var_psl_lim_dn7 = assign14910_e20940_d_n7;
        locals.var_psl_lim_dn10 = assign14910_e20940_d_n10;
        locals.var_psl_lim_dn11 = assign14910_e20940_d_n11;
        locals.var_psl_lim_dn12 = assign14910_e20940_d_n12;
        locals.var_psl_lim_dn17 = assign14910_e20940_d_n17;

        let assign14920_e20947: f64 = if ((p.p25 == 1.0) && (p.p26 == 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard448 = assign14920_e20947;

    }
}
